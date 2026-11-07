// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The conversion of the names of the link (`ObjectName`) to and from the MMS object names of the codec.

use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iso9506_mms_codec::generated::types::{
    Identifier, ObjectName as MmsObjectName, ObjectNameDomainSpecific, VariableAccessSpecification,
    VariableAccessSpecificationListOfVariableItem, VariableSpecification,
};

/// The MMS object name of a name of the link: VMD-specific, or domain-specific.
pub fn to_mms(name: &ObjectName) -> MmsObjectName {
    match &name.scope {
        Scope::Vmd => MmsObjectName::VmdSpecific(Identifier(name.name.clone())),
        Scope::Domain(domain) => MmsObjectName::DomainSpecific(ObjectNameDomainSpecific {
            domain_id: Identifier(domain.clone()),
            item_id: Identifier(name.name.clone()),
        }),
    }
}

/// The name of the link of an MMS object name. An AA-specific name is not a TASE.2 name: `None`.
pub fn from_mms(name: &MmsObjectName) -> Option<ObjectName> {
    match name {
        MmsObjectName::VmdSpecific(identifier) => Some(ObjectName {
            scope: Scope::Vmd,
            name: identifier.0.clone(),
        }),
        MmsObjectName::DomainSpecific(domain) => Some(ObjectName {
            scope: Scope::Domain(domain.domain_id.0.clone()),
            name: domain.item_id.0.clone(),
        }),
        MmsObjectName::AaSpecific(_) => None,
    }
}

/// The variable specification of a name, for the requests that name variables.
pub fn variable(name: &ObjectName) -> VariableSpecification {
    VariableSpecification::Name(to_mms(name))
}

/// The list of variables named, for a Read or a Write.
pub fn list_of_variables(names: &[ObjectName]) -> VariableAccessSpecification {
    VariableAccessSpecification::ListOfVariable(
        names
            .iter()
            .map(|name| VariableAccessSpecificationListOfVariableItem {
                variable_specification: variable(name),
                alternate_access: None,
            })
            .collect(),
    )
}
