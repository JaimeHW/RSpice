//! Validated immutable requirements sealed into a run receipt.

use super::{SpecEntry, SpecificationDefinition, SpecificationPolicy};

/// Immutable specification definition sealed into one prepared run.
///
/// The wrapper keeps the exact authored row while enforcing the invariants
/// that make bitwise equality sound. Historical result readers can therefore
/// use the run's own requirement instead of the currently edited plan.
#[derive(Debug, Clone)]
pub struct PreparedSpecification {
    entry: SpecEntry,
    definition: Option<SpecificationDefinition>,
}

impl PartialEq for PreparedSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.entry.measurement == other.entry.measurement
            && self.entry.expression == other.entry.expression
            && self.entry.min.map(f64::to_bits) == other.entry.min.map(f64::to_bits)
            && self.entry.max.map(f64::to_bits) == other.entry.max.map(f64::to_bits)
            && self.entry.unit == other.entry.unit
            && self.entry.scope == other.entry.scope
            && match (&self.definition, &other.definition) {
                (None, None) => true,
                (Some(left), Some(right)) => left.bitwise_eq(right),
                (None, Some(_)) | (Some(_), None) => false,
            }
    }
}

impl Eq for PreparedSpecification {}

impl PreparedSpecification {
    pub fn new(entry: SpecEntry) -> Result<Self, String> {
        entry.validate()?;
        Ok(Self {
            entry,
            definition: None,
        })
    }

    pub fn from_definition(definition: SpecificationDefinition) -> Result<Self, String> {
        definition.validate()?;
        let entry = definition.projected_entry();
        entry.validate()?;
        Ok(Self {
            entry,
            definition: Some(definition),
        })
    }

    #[must_use]
    pub fn entry(&self) -> &SpecEntry {
        &self.entry
    }

    #[must_use]
    pub fn definition(&self) -> Option<&SpecificationDefinition> {
        self.definition.as_ref()
    }
}

/// Validated plan-wide requirement-evaluation policy sealed into a run.
///
/// The authored policy contains a floating-point yield threshold, so this
/// wrapper supplies bitwise equality only after validation has rejected
/// non-finite values. That keeps durable receipt equality reflexive and exact.
#[derive(Debug, Clone, Default)]
pub struct PreparedSpecificationPolicy {
    policy: SpecificationPolicy,
}

impl PartialEq for PreparedSpecificationPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.policy.bitwise_eq(&other.policy)
    }
}

impl Eq for PreparedSpecificationPolicy {}

impl PreparedSpecificationPolicy {
    pub fn new(policy: SpecificationPolicy) -> Result<Self, String> {
        policy.validate()?;
        Ok(Self { policy })
    }

    #[must_use]
    pub fn policy(&self) -> &SpecificationPolicy {
        &self.policy
    }
}
