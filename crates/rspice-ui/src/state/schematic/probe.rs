//! Durable schematic probe flags.
//!
//! A probe flag is authored canvas intent: it retains where the user placed
//! the crosshair and, when resolved, the exact raw output expression it owns.
//! Unbound flags deliberately carry no fabricated electrical expression.

use serde::{Deserialize, Serialize};

use super::Point;

pub const MAX_SCHEMATIC_PROBE_REFERENCE_LEN: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicProbe {
    pub id: u64,
    pub position: Point,
    pub reference: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_expression: Option<String>,
}

impl SchematicProbe {
    pub fn new(
        id: u64,
        position: Point,
        reference: impl Into<String>,
        source_expression: Option<String>,
    ) -> Result<Self, String> {
        let probe = Self {
            id,
            position,
            reference: reference.into(),
            source_expression,
        };
        probe.validate()?;
        Ok(probe)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id == 0 {
            return Err("probe identity must not be zero".to_owned());
        }
        let reference = self.reference.trim();
        if reference.is_empty() {
            return Err("probe reference is required".to_owned());
        }
        if reference.chars().count() > MAX_SCHEMATIC_PROBE_REFERENCE_LEN {
            return Err(format!(
                "probe reference exceeds {MAX_SCHEMATIC_PROBE_REFERENCE_LEN} characters"
            ));
        }
        if reference.chars().any(char::is_control) {
            return Err("probe reference contains a control character".to_owned());
        }
        if let Some(expression) = &self.source_expression {
            let expression = expression.trim();
            crate::state::workspace::validate_raw_probe(expression)
                .map_err(|error| format!("probe source expression is invalid: {error}"))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_identity_and_optional_binding_fail_closed() {
        assert!(SchematicProbe::new(0, Point::origin(), "P1", None).is_err());
        assert!(SchematicProbe::new(1, Point::origin(), " ", None).is_err());
        assert!(SchematicProbe::new(1, Point::origin(), "V(out)", Some(" ".to_owned())).is_err());
        assert_eq!(
            SchematicProbe::new(1, Point::new(20, 30), "V(out)", Some("V(out)".to_owned()))
                .unwrap()
                .source_expression
                .as_deref(),
            Some("V(out)")
        );
    }
}
