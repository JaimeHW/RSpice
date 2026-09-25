//! Authored simulation-plan design variables and exact validation.

use std::collections::HashMap;

use rspice_app_types::product::{AnalysisInstanceId, DesignVariableId, ObjectRevision};
use rspice_design_model::cell_view::CellViewRef;
use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use uuid::Uuid;

use crate::design_variable_quantity::{DesignVariableQuantity, parse_design_quantity};
use crate::saved_output::{
    deserialize_or_migrate_identity, missing_identity_sentinel, validate_bounded_text,
    validate_parameter_name, validate_single_line_expression,
};

/// Exact ownership boundary for a design variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DesignVariableScope {
    Testbench,
    Project,
    SelectedCell { cell: CellViewRef },
    SelectedAnalysis { analysis_id: AnalysisInstanceId },
}

impl DesignVariableScope {
    /// The ownership boundary, named as a boundary.
    ///
    /// `Testbench` read "Lab characterization · testbench", which baked the
    /// *default plan name* into a scope label: rename the plan and the label
    /// went on naming the old one, in a column whose other three entries name
    /// nothing but boundaries. The spec-sheet importer accepts these spellings
    /// as cell values, so the rename travels there too.
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Testbench => "Testbench",
            Self::Project => "Project",
            Self::SelectedCell { .. } => "Selected cell",
            Self::SelectedAnalysis { .. } => "Selected analysis only",
        }
    }
}

/// Inclusive engineering bounds for a variable. Bounds remain expressions so
/// suffixes and owner variables survive a lossless project round trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariableRange {
    pub minimum: String,
    pub maximum: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableSweepEligibility {
    NestedSweepAndOptimization,
    OptimizationOnly,
    FixedParameter,
}

impl DesignVariableSweepEligibility {
    pub const ALL: [Self; 3] = [
        Self::NestedSweepAndOptimization,
        Self::OptimizationOnly,
        Self::FixedParameter,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::NestedSweepAndOptimization => "Nested sweep + optimization",
            Self::OptimizationOnly => "Optimization only",
            Self::FixedParameter => "Fixed parameter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableOverridePolicy {
    ExplicitTestLocalOverride,
    InheritOwnerOnly,
}

impl DesignVariableOverridePolicy {
    pub const ALL: [Self; 2] = [Self::ExplicitTestLocalOverride, Self::InheritOwnerOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ExplicitTestLocalOverride => "Explicit test-local override",
            Self::InheritOwnerOnly => "Inherit owner only",
        }
    }
}

/// Persisted, typed simulation-plan parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariable {
    pub id: DesignVariableId,
    pub revision: ObjectRevision,
    pub name: String,
    pub expression: String,
    pub quantity: DesignVariableQuantity,
    pub scope: DesignVariableScope,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub allowed_range: Option<DesignVariableRange>,
    pub sweep_eligibility: DesignVariableSweepEligibility,
    pub override_policy: DesignVariableOverridePolicy,
}

/// Which contract a design variable failed.
///
/// Deliberately coarse. These are the four kinds of thing an author can get
/// wrong about a variable, not one variant per message: a finer split would
/// have to be kept in step with the wording, which is the coupling this exists
/// to remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignVariableDefect {
    /// The name is not a usable parameter identifier.
    Identifier,
    /// A value does not read as the quantity it is declared in — the
    /// expression itself, or one end of the allowed range.
    Dimension,
    /// The allowed range is inconsistent, or the resolved value falls outside
    /// it.
    Bounds,
    /// The rest of the record contract: description length, ownership scope.
    Record,
}

#[expect(
    clippy::too_many_arguments,
    reason = "preserve the existing authored-variable constructor API while moving its owner"
)]
impl DesignVariable {
    pub fn new(
        name: impl Into<String>,
        expression: impl Into<String>,
        quantity: DesignVariableQuantity,
        scope: DesignVariableScope,
        description: impl Into<String>,
        allowed_range: Option<DesignVariableRange>,
        sweep_eligibility: DesignVariableSweepEligibility,
        override_policy: DesignVariableOverridePolicy,
    ) -> Result<Self, String> {
        Self::new_defect(
            name,
            expression,
            quantity,
            scope,
            description,
            allowed_range,
            sweep_eligibility,
            override_policy,
        )
        .map_err(|(_, message)| message)
    }

    /// [`Self::new`], with the failed contract named beside the sentence. See
    /// [`Self::validate_defect`] for why anything needs that.
    pub fn new_defect(
        name: impl Into<String>,
        expression: impl Into<String>,
        quantity: DesignVariableQuantity,
        scope: DesignVariableScope,
        description: impl Into<String>,
        allowed_range: Option<DesignVariableRange>,
        sweep_eligibility: DesignVariableSweepEligibility,
        override_policy: DesignVariableOverridePolicy,
    ) -> Result<Self, (DesignVariableDefect, String)> {
        let variable = Self {
            id: DesignVariableId::new(),
            revision: ObjectRevision::INITIAL,
            name: name.into(),
            expression: expression.into(),
            quantity,
            scope,
            description: description.into(),
            allowed_range,
            sweep_eligibility,
            override_policy,
        };
        variable.validate_defect()?;
        Ok(variable)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_defect().map_err(|(_, message)| message)
    }

    /// [`Self::validate`], with the failed contract named beside the sentence.
    ///
    /// The sentence says *how* a variable is wrong, which is what a reader
    /// needs. A caller that has to route, group or identify refusals needs to
    /// know *which* contract failed, and reading that back out of English is
    /// how a message reword becomes a silent behaviour change. The spec-sheet
    /// import is the caller that needs it: its refusals carry a stable
    /// identity, and this is where the rule that failed already is.
    ///
    /// Every arm of [`Self::validate`] is tagged here rather than a second set
    /// of checks being written beside them, so there remains exactly one
    /// definition of a valid design variable.
    pub fn validate_defect(&self) -> Result<(), (DesignVariableDefect, String)> {
        use DesignVariableDefect as Defect;
        let tag = |defect: Defect| move |message: String| (defect, message);

        validate_parameter_name(&self.name).map_err(tag(Defect::Identifier))?;
        validate_single_line_expression("expression", &self.expression)
            .map_err(tag(Defect::Dimension))?;
        let value = self.resolved_value_si().map_err(tag(Defect::Dimension))?;
        if let Some(range) = &self.allowed_range {
            validate_single_line_expression("allowed-range minimum", &range.minimum)
                .map_err(tag(Defect::Bounds))?;
            validate_single_line_expression("allowed-range maximum", &range.maximum)
                .map_err(tag(Defect::Bounds))?;
            let minimum = parse_design_quantity(&range.minimum, self.quantity)
                .map_err(|error| {
                    format!(
                        "allowed-range minimum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })
                .map_err(tag(Defect::Dimension))?;
            let maximum = parse_design_quantity(&range.maximum, self.quantity)
                .map_err(|error| {
                    format!(
                        "allowed-range maximum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })
                .map_err(tag(Defect::Dimension))?;
            if minimum > maximum {
                return Err((
                    Defect::Bounds,
                    "allowed-range minimum exceeds its maximum".to_owned(),
                ));
            }
            if value < minimum || value > maximum {
                return Err((
                    Defect::Bounds,
                    format!(
                        "resolved value {value} is outside the inclusive allowed range {minimum}..={maximum}"
                    ),
                ));
            }
        }
        validate_bounded_text("description", &self.description, 1_024, true)
            .map_err(tag(Defect::Record))?;
        if let DesignVariableScope::SelectedCell { cell } = &self.scope {
            cell.validate_name_segments()
                .map_err(|error| format!("selected cell is invalid: {error}"))
                .map_err(tag(Defect::Record))?;
        }
        Ok(())
    }

    pub fn resolved_value_si(&self) -> Result<f64, String> {
        parse_design_quantity(&self.expression, self.quantity).map_err(|error| {
            format!(
                "expression is invalid for {}: {error}",
                self.quantity.label()
            )
        })
    }

    /// Canonical top-level SPICE statement. Validation is intentionally kept
    /// separate so callers can aggregate every project diagnostic at once.
    pub fn netlist_statement(&self) -> String {
        let value = self
            .resolved_value_si()
            .expect("validated design variables always resolve to finite SI values");
        format!(".param {}={value:.17e}", self.name)
    }

    pub fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = DesignVariableId::new();
        cloned.revision = ObjectRevision::INITIAL;
        if let DesignVariableScope::SelectedAnalysis { analysis_id } = &mut cloned.scope {
            *analysis_id = analysis_identity_map
                .get(analysis_id)
                .copied()
                .ok_or(*analysis_id)?;
        }
        Ok(cloned)
    }
}

const LEGACY_DESIGN_VARIABLE_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x3c56_6c65_03dc_5e65_b66c_a4d4_86dc_8d53);

impl<'de> Deserialize<'de> for DesignVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            #[serde(default = "missing_identity_sentinel")]
            id: serde_json::Value,
            #[serde(default)]
            revision: ObjectRevision,
            name: String,
            expression: String,
            quantity: DesignVariableQuantity,
            scope: DesignVariableScope,
            #[serde(default)]
            description: String,
            #[serde(default)]
            allowed_range: Option<DesignVariableRange>,
            sweep_eligibility: DesignVariableSweepEligibility,
            override_policy: DesignVariableOverridePolicy,
        }

        let wire = Wire::deserialize(deserializer)?;
        let identity = serde_json::to_vec(&(
            &wire.name,
            &wire.expression,
            wire.quantity,
            &wire.scope,
            &wire.description,
            &wire.allowed_range,
            wire.sweep_eligibility,
            wire.override_policy,
        ))
        .map_err(D::Error::custom)?;
        let id = deserialize_or_migrate_identity::<DesignVariableId, D::Error>(
            wire.id,
            LEGACY_DESIGN_VARIABLE_ID_NAMESPACE,
            &identity,
            DesignVariableId::from_namespace,
        )?;
        Ok(Self {
            id,
            revision: wire.revision,
            name: wire.name,
            expression: wire.expression,
            quantity: wire.quantity,
            scope: wire.scope,
            description: wire.description,
            allowed_range: wire.allowed_range,
            sweep_eligibility: wire.sweep_eligibility,
            override_policy: wire.override_policy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resistance_variable(
        name: &str,
        expression: &str,
        scope: DesignVariableScope,
    ) -> DesignVariable {
        DesignVariable::new(
            name,
            expression,
            DesignVariableQuantity::Resistance,
            scope,
            "fixture",
            Some(DesignVariableRange {
                minimum: "1 kohm".to_owned(),
                maximum: "1 Mohm".to_owned(),
            }),
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .expect("fixture variable is valid")
    }

    #[test]
    fn typed_design_variable_enforces_units_range_and_canonical_netlist_value() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        assert_eq!(variable.resolved_value_si().unwrap(), 10_000.0);
        assert_eq!(
            variable.netlist_statement(),
            ".param RLOAD=1.00000000000000000e4"
        );

        let mut wrong_unit = variable.clone();
        wrong_unit.expression = "10 V".to_owned();
        assert!(wrong_unit.validate().unwrap_err().contains("resistance"));

        let mut outside = variable;
        outside.expression = "2 Mohm".to_owned();
        assert!(outside.validate().unwrap_err().contains("outside"));
    }

    #[test]
    fn missing_row_identity_migrates_deterministically_and_null_is_rejected() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let mut value = serde_json::to_value(variable).unwrap();
        value.as_object_mut().unwrap().remove("id");
        let first: DesignVariable = serde_json::from_value(value.clone()).unwrap();
        let second: DesignVariable = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(first.id, second.id);

        value
            .as_object_mut()
            .unwrap()
            .insert("id".to_owned(), serde_json::Value::Null);
        assert!(
            serde_json::from_value::<DesignVariable>(value)
                .unwrap_err()
                .to_string()
                .contains("must not be null")
        );
    }
}
