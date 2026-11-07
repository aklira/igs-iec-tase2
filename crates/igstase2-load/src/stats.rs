// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The latencies of the operations, and their summary: the count, the errors, and the percentiles.

use std::collections::BTreeMap;
use std::time::Duration;

/// The samples of each operation.
#[derive(Debug, Default, Clone)]
pub struct Recorder {
    samples: BTreeMap<&'static str, Vec<Duration>>,
    /// The operations that failed or returned an unexpected answer.
    pub errors: u64,
    /// Reports received where none was expected.
    pub stray_reports: u64,
}

impl Recorder {
    /// Records one sample of an operation.
    pub fn record(&mut self, operation: &'static str, latency: Duration) {
        self.samples.entry(operation).or_default().push(latency);
    }

    /// Merges the samples of another recorder.
    pub fn merge(&mut self, other: Recorder) {
        for (operation, samples) in other.samples {
            self.samples.entry(operation).or_default().extend(samples);
        }
        self.errors = self.errors.saturating_add(other.errors);
        self.stray_reports = self.stray_reports.saturating_add(other.stray_reports);
    }

    /// The number of samples of an operation.
    pub fn count(&self, operation: &str) -> usize {
        self.samples.get(operation).map_or(0, Vec::len)
    }

    /// The summary of every operation, one line each, in milliseconds.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        for (operation, samples) in &self.samples {
            let mut sorted = samples.clone();
            sorted.sort();
            let percentile = |p: f64| -> f64 {
                if sorted.is_empty() {
                    return 0.0;
                }
                let rank = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
                let index = rank.clamp(1, sorted.len()).saturating_sub(1);
                sorted.get(index).map_or(0.0, |d| d.as_secs_f64() * 1000.0)
            };
            let max = sorted.last().map_or(0.0, |d| d.as_secs_f64() * 1000.0);
            lines.push(format!(
                "{operation:<16} n={} p50={:.3}ms p95={:.3}ms p99={:.3}ms max={:.3}ms",
                sorted.len(),
                percentile(50.0),
                percentile(95.0),
                percentile(99.0),
                max
            ));
        }
        lines.push(format!(
            "errors={} stray_reports={}",
            self.errors, self.stray_reports
        ));
        lines.join("\n")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_percentiles_are_the_ranked_samples() {
        let mut recorder = Recorder::default();
        for ms in 1..=100u64 {
            recorder.record("read", Duration::from_millis(ms));
        }
        let summary = recorder.summary();
        assert!(summary.contains("n=100"), "{summary}");
        assert!(summary.contains("p50=50.000ms"), "{summary}");
        assert!(summary.contains("p95=95.000ms"), "{summary}");
        assert!(summary.contains("p99=99.000ms"), "{summary}");
        assert!(summary.contains("max=100.000ms"), "{summary}");
    }

    #[test]
    fn merging_adds_the_samples_and_the_errors() {
        let mut a = Recorder::default();
        a.record("write", Duration::from_millis(1));
        a.errors = 1;
        let mut b = Recorder::default();
        b.record("write", Duration::from_millis(2));
        b.stray_reports = 2;
        a.merge(b);
        assert_eq!(a.count("write"), 2);
        assert_eq!(a.errors, 1);
        assert_eq!(a.stray_reports, 2);
    }
}
