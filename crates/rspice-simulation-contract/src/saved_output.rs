//! Saved-output records, and the field validators the project model shares.
//!
//! A saved output names a signal, how precisely to keep it, whether it may
//! stream, and what makes a retained result compatible with the request.
//! Shared lexical validation comes from `rspice-vocabulary`, independently
//! of a live project workspace.

use rspice_vocabulary::text_validation::{validate_bounded_text, validate_parameter_name};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use rspice_results::calculator::parser::Parser;
use rspice_vocabulary::product::{AnalysisInstanceId, ObjectRevision, SavedOutputId};

pub use crate::output_policy::OutputSelectionMode;

use rspice_results::saved_output::parse_probe_target;
pub use rspice_results::saved_output::{
    ComplexExpressionPolicy, SavedOutputDisplayIntent, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, device_current_probe, raw_probe_unit,
    saved_output_references, validate_raw_probe,
};

/// Authored authority for a saved output. Probe-owned rows remain in the plan
/// so undo can restore their exact identity, but execution includes them only
/// while an enabled schematic marker still references that identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputOrigin {
    #[default]
    Plan,
    SchematicProbe,
    /// A bounded node selection synthesized for the prepared run.
    Automatic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SavedOutputCompatibility {
    OpTranAc,
    AllCompatibleAnalyses,
    SelectedAnalysis { analysis_id: AnalysisInstanceId },
}

impl SavedOutputCompatibility {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::OpTranAc => "OP + TRAN + AC",
            Self::AllCompatibleAnalyses => "All compatible analyses",
            Self::SelectedAnalysis { .. } => "Selected analysis only",
        }
    }
}

/// Persisted waveform/data contract owned by the simulation plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutput {
    pub id: SavedOutputId,
    #[serde(default)]
    pub revision: ObjectRevision,
    #[serde(default)]
    pub origin: SavedOutputOrigin,
    #[serde(default)]
    pub display_intent: SavedOutputDisplayIntent,
    pub kind: SavedOutputKind,
    pub name: String,
    pub source_expression: String,
    #[serde(default, skip_serializing_if = "ComplexExpressionPolicy::is_legacy")]
    pub complex_policy: ComplexExpressionPolicy,
    pub compatible_analyses: SavedOutputCompatibility,
    pub save_policy: SavedOutputPolicy,
    pub stored_precision: SavedOutputPrecision,
    pub streaming: SavedOutputStreaming,
}

impl SavedOutput {
    pub fn new(
        kind: SavedOutputKind,
        name: impl Into<String>,
        source_expression: impl Into<String>,
        compatible_analyses: SavedOutputCompatibility,
        save_policy: SavedOutputPolicy,
        stored_precision: SavedOutputPrecision,
        streaming: SavedOutputStreaming,
    ) -> Result<Self, String> {
        let output = Self {
            id: SavedOutputId::new(),
            revision: ObjectRevision::INITIAL,
            origin: SavedOutputOrigin::Plan,
            display_intent: SavedOutputDisplayIntent::Plot,
            kind,
            name: name.into(),
            source_expression: source_expression.into(),
            complex_policy: if kind == SavedOutputKind::DerivedExpression {
                ComplexExpressionPolicy::Rectangular
            } else {
                ComplexExpressionPolicy::LegacyMagnitude
            },
            compatible_analyses,
            save_policy,
            stored_precision,
            streaming,
        };
        output.validate()?;
        Ok(output)
    }

    #[must_use]
    pub fn with_origin(mut self, origin: SavedOutputOrigin) -> Self {
        self.origin = origin;
        self
    }

    #[must_use]
    pub fn with_display_intent(mut self, display_intent: SavedOutputDisplayIntent) -> Self {
        self.display_intent = display_intent;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.kind != SavedOutputKind::DerivedExpression && !self.complex_policy.is_legacy() {
            return Err("complex expression policy applies only to derived expressions".to_owned());
        }
        validate_bounded_text("name", &self.name, 256, false)?;
        validate_bounded_text(
            "source or expression",
            &self.source_expression,
            8_192,
            false,
        )?;
        validate_saved_output_expression(self.kind, &self.source_expression)
    }

    /// Unit preview derived from the output schema without inspecting a
    /// mutable result dataset. Derived expressions remain explicit until the
    /// calculator's dimensional resolver evaluates their dependencies.
    pub fn inferred_unit(&self) -> &'static str {
        match self.kind {
            SavedOutputKind::RawVoltageOrCurrent => match raw_probe_unit(&self.source_expression) {
                Some("V") => "volts",
                Some("A") => "amperes",
                _ => "invalid probe",
            },
            SavedOutputKind::DerivedExpression => "resolved from expression",
            SavedOutputKind::DeviceOperatingPointQuantity => "from device quantity",
            SavedOutputKind::NoiseContributor => "V²/Hz or A²/Hz",
            SavedOutputKind::RfPortQuantity => "dimensionless",
        }
    }

    pub const fn status_label(&self) -> &'static str {
        match self.save_policy {
            SavedOutputPolicy::EveryAcceptedPoint => "full capture",
            SavedOutputPolicy::SelectedAndFinalPoints => "selected + final",
            SavedOutputPolicy::OnDemandFromRetainedState => "retained-state derivation",
            SavedOutputPolicy::FailureDiagnosticsOnly => "failure diagnostics",
        }
    }

    pub fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = SavedOutputId::new();
        cloned.revision = ObjectRevision::INITIAL;
        if let SavedOutputCompatibility::SelectedAnalysis { analysis_id } =
            &mut cloned.compatible_analyses
        {
            *analysis_id = analysis_identity_map
                .get(analysis_id)
                .copied()
                .ok_or(*analysis_id)?;
        }
        Ok(cloned)
    }
}

fn validate_saved_output_expression(kind: SavedOutputKind, expression: &str) -> Result<(), String> {
    let expression = expression.trim();
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => validate_raw_probe(expression),
        SavedOutputKind::DerivedExpression => parse_calculator_expression(expression),
        SavedOutputKind::DeviceOperatingPointQuantity => validate_device_op_probe(expression),
        SavedOutputKind::NoiseContributor => {
            if parse_probe_target(expression).is_ok() {
                Ok(())
            } else {
                parse_calculator_expression(expression)
            }
        }
        SavedOutputKind::RfPortQuantity => {
            if !expression
                .get(..2)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("S("))
            {
                return Err("RF port quantity must use S(port, port) syntax".to_owned());
            }
            parse_calculator_expression(expression)
        }
    }
}

fn parse_calculator_expression(expression: &str) -> Result<(), String> {
    Parser::new(expression, crate::spice_value::parse_spice_value_checked)
        .try_parse()
        .map(|_| ())
        .map_err(|error| format!("expression is invalid: {error}"))
}

fn validate_device_op_probe(expression: &str) -> Result<(), String> {
    let Some(body) = expression.strip_prefix('@') else {
        return Err("device operating-point quantity must use @device[quantity] syntax".to_owned());
    };
    let Some(open) = body.find('[') else {
        return Err("device operating-point quantity must use @device[quantity] syntax".to_owned());
    };
    if !body.ends_with(']') {
        return Err("device operating-point quantity has an unterminated quantity".to_owned());
    }
    parse_probe_target(&body[..open])?;
    validate_parameter_name(&body[open + 1..body.len() - 1])
        .map_err(|error| format!("device quantity is invalid: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_policy_keeps_its_wire_format() {
        let raw = raw_output("V(out)").unwrap();
        let mut json = serde_json::to_value(&raw).unwrap();
        json["kind"] = "derived_expression".into();
        assert!(json.get("complex_policy").is_none());
        let first: SavedOutput = serde_json::from_value(json.clone()).unwrap();
        assert!(first.complex_policy.is_legacy());
        assert!(
            serde_json::to_value(&first)
                .unwrap()
                .get("complex_policy")
                .is_none()
        );
        json["complex_policy"] = "rectangular".into();
        let current: SavedOutput = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(current.complex_policy, ComplexExpressionPolicy::Rectangular);
        for invalid in [serde_json::Value::Null, "future_interpretation".into()] {
            json["complex_policy"] = invalid;
            assert!(serde_json::from_value::<SavedOutput>(json.clone()).is_err());
        }
    }

    fn raw_output(expression: &str) -> Result<SavedOutput, String> {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "probe",
            expression,
            SavedOutputCompatibility::OpTranAc,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
    }

    #[test]
    fn raw_probes_take_every_scope_spelling_and_no_empty_segment() {
        for expression in [
            "V(out)",
            "V(vdd$)",
            "V(/X1/n)",
            "V(x1.n)",
            "V(x1:n)",
            "V(top.x1.n)",
            "V(/X1/a, /X1/b)",
            "I(x1.r1)",
        ] {
            assert!(
                raw_output(expression).is_ok(),
                "{expression} must be accepted"
            );
        }
        for expression in ["V(/X1/)", "V(//n)", "V()", "V(a b)", "V(/X1/x2.n)"] {
            assert!(
                raw_output(expression).is_err(),
                "{expression} must be rejected"
            );
        }
    }

    #[test]
    fn every_spelling_of_one_target_resolves_to_the_same_canonical_path() {
        for spelling in ["/x1/n", "x1.n", "x1:n", "top.x1.n", "/top/x1/n"] {
            let target = parse_probe_target(spelling).expect("probe spelling resolves");
            assert_eq!(target.to_string(), "/x1/n", "{spelling}");
            assert_eq!(
                target.engine_name().expect("engine name"),
                "x1.n",
                "{spelling}"
            );
        }
        assert_eq!(
            parse_probe_target("out").expect("root probe").to_string(),
            "out",
            "a probe at the design root is named by its leaf alone"
        );
    }

    #[test]
    fn device_and_noise_probes_read_the_same_scopes_as_a_raw_probe() {
        for expression in ["@x1.m1[gm]", "@/X1/M1[gm]", "@m1[id]"] {
            assert_eq!(
                validate_saved_output_expression(
                    SavedOutputKind::DeviceOperatingPointQuantity,
                    expression
                ),
                Ok(()),
                "{expression}"
            );
        }
        for expression in ["@[gm]", "@x1.m1[]", "@//m1[gm]", "@x1.m1"] {
            assert!(
                validate_saved_output_expression(
                    SavedOutputKind::DeviceOperatingPointQuantity,
                    expression
                )
                .is_err(),
                "{expression} must be rejected"
            );
        }
        for expression in ["onoise", "x1.rload", "/X1/Rload", "V(x1.n) * 2"] {
            assert_eq!(
                validate_saved_output_expression(SavedOutputKind::NoiseContributor, expression),
                Ok(()),
                "{expression}"
            );
        }
    }

    #[test]
    fn loading_a_project_keeps_the_expression_the_project_stored() {
        let stored = r#"{
            "id": "6f1c2a4e-3b7d-4c59-9e21-8a0f5d3b7c11",
            "kind": "raw_voltage_or_current",
            "name": "probe",
            "source_expression": "V(top.x1.n)",
            "compatible_analyses": { "kind": "op_tran_ac" },
            "save_policy": "every_accepted_point",
            "stored_precision": "full_source_precision",
            "streaming": "store_only"
        }"#;
        let output: SavedOutput = serde_json::from_str(stored).expect("a stored output loads");
        assert_eq!(
            output.source_expression, "V(top.x1.n)",
            "a read resolves the target; it does not rewrite what the project persisted"
        );
        assert_eq!(output.validate(), Ok(()));
    }
}
