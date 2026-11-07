// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The changes that the demonstration server makes by itself: the values that a simulation gives at each period. The
//! values are computed here, without a clock; the binary calls `next` once per period.

use std::f64::consts::PI;

use igs_iec_tase2_model::generated::iso802::clause_6_2_1::DataReal;
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms_codec::generated::types::Data;

use crate::config::{PointKind, Simulation, SimulationKind};

/// The number of periods of one sine wave.
const SINE_PERIODS: u64 = 60;

/// The state of one simulation.
#[derive(Debug, Clone)]
pub struct Stepper {
    simulation: Simulation,
    /// The number of periods given so far.
    count: u64,
    /// The current value of a ramp.
    ramp: f64,
    /// The current state of a toggle.
    flag: bool,
}

impl Stepper {
    /// A simulation at its start: a ramp at its lowest value, a toggle off.
    pub fn new(simulation: Simulation) -> Self {
        let ramp = simulation.min;
        Self {
            simulation,
            count: 0,
            ramp,
            flag: false,
        }
    }

    /// The value of the point at the next period.
    pub fn next(&mut self) -> Data {
        self.count = self.count.saturating_add(1);
        match self.simulation.kind {
            SimulationKind::Toggle => {
                self.flag = !self.flag;
                Data::Boolean(self.flag)
            }
            SimulationKind::Ramp => {
                let next = self.ramp + self.simulation.step;
                self.ramp = if next > self.simulation.max {
                    self.simulation.min
                } else {
                    next
                };
                self.value_of(self.ramp)
            }
            SimulationKind::Sine => {
                let phase = 2.0 * PI * (self.count % SINE_PERIODS) as f64 / SINE_PERIODS as f64;
                let level = self.simulation.min
                    + (self.simulation.max - self.simulation.min) * (1.0 + phase.sin()) / 2.0;
                self.value_of(level)
            }
        }
    }

    /// The value of a number, in the MMS type of the point.
    fn value_of(&self, level: f64) -> Data {
        match self.simulation.point {
            PointKind::Integer => Data::Integer(level.round() as i128),
            PointKind::Unsigned => Data::Unsigned(level.round().max(0.0) as u128),
            PointKind::Float => DataReal(level).to_mms(),
            PointKind::Boolean => Data::Boolean(level > 0.0),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use std::time::Duration;

    use igs_iec_tase2_link::bilateral::{ObjectName, Scope};

    use super::*;

    fn simulation(
        kind: SimulationKind,
        point: PointKind,
        min: f64,
        max: f64,
        step: f64,
    ) -> Simulation {
        Simulation {
            name: ObjectName {
                scope: Scope::Domain("ICC1".to_string()),
                name: "Point".to_string(),
            },
            kind,
            period: Duration::from_secs(1),
            min,
            max,
            step,
            point,
        }
    }

    #[test]
    fn a_ramp_climbs_by_its_step_and_starts_again_after_its_maximum() {
        let mut stepper = Stepper::new(simulation(
            SimulationKind::Ramp,
            PointKind::Integer,
            0.0,
            3.0,
            1.0,
        ));
        let values: Vec<Data> = (0..5).map(|_| stepper.next()).collect();
        assert_eq!(
            values,
            vec![
                Data::Integer(1),
                Data::Integer(2),
                Data::Integer(3),
                Data::Integer(0),
                Data::Integer(1),
            ]
        );
    }

    #[test]
    fn a_toggle_changes_at_each_period() {
        let mut stepper = Stepper::new(simulation(
            SimulationKind::Toggle,
            PointKind::Boolean,
            0.0,
            0.0,
            0.0,
        ));
        assert_eq!(stepper.next(), Data::Boolean(true));
        assert_eq!(stepper.next(), Data::Boolean(false));
    }

    #[test]
    fn a_sine_stays_between_its_bounds_and_is_a_real() {
        let mut stepper = Stepper::new(simulation(
            SimulationKind::Sine,
            PointKind::Float,
            -1.0,
            1.0,
            0.0,
        ));
        for _ in 0..SINE_PERIODS {
            let Data::FloatingPoint(float) = stepper.next() else {
                panic!("a real is expected");
            };
            let value = DataReal::from_mms(&Data::FloatingPoint(float)).unwrap().0;
            assert!((-1.0..=1.0).contains(&value), "{value}");
        }
    }

    #[test]
    fn an_unsigned_point_is_never_negative() {
        let mut stepper = Stepper::new(simulation(
            SimulationKind::Sine,
            PointKind::Unsigned,
            -5.0,
            5.0,
            0.0,
        ));
        for _ in 0..SINE_PERIODS {
            let Data::Unsigned(_) = stepper.next() else {
                panic!("an unsigned is expected");
            };
        }
    }
}
