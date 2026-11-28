//! Authored simulation-plan design variables and exact validation.

use std::collections::HashMap;

use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::product::{AnalysisInstanceId, DesignVariableId, ObjectRevision};
use serde::{Deserialize, Serialize};

use crate::design_variable_quantity::{DesignVariableQuantity, parse_design_quantity};
use rspice_vocabulary::text_validation::{
    validate_bounded_text, validate_parameter_name, validate_single_line_expression,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariable {
    pub id: DesignVariableId,
    #[serde(default)]
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
    /// A value the engine cannot read: the expression itself, or one end of
    /// the allowed range.
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
        let value = standalone_value(&self.expression, self.quantity)
            .map_err(|error| format!("the value {} does not read: {error}", self.expression))
            .map_err(tag(Defect::Dimension))?;
        if let Some(range) = &self.allowed_range {
            validate_single_line_expression("allowed-range minimum", &range.minimum)
                .map_err(tag(Defect::Bounds))?;
            validate_single_line_expression("allowed-range maximum", &range.maximum)
                .map_err(tag(Defect::Bounds))?;
            let minimum = standalone_value(&range.minimum, self.quantity)
                .map_err(|error| format!("the minimum {} does not read: {error}", range.minimum))
                .map_err(tag(Defect::Dimension))?;
            let maximum = standalone_value(&range.maximum, self.quantity)
                .map_err(|error| format!("the maximum {} does not read: {error}", range.maximum))
                .map_err(tag(Defect::Dimension))?;
            // A bound or a value that names another variable is checked
            // where the plan resolves them all; one that stands alone is
            // checked here, so a typed number outside its range is refused
            // as it is typed.
            if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
                if minimum > maximum {
                    return Err((
                        Defect::Bounds,
                        format!(
                            "the minimum {} is above the maximum {}",
                            range.minimum, range.maximum
                        ),
                    ));
                }
                if value.is_some_and(|value| value < minimum || value > maximum) {
                    return Err((
                        Defect::Bounds,
                        format!(
                            "the value {} is outside its range {} \u{2026} {}",
                            self.expression, range.minimum, range.maximum
                        ),
                    ));
                }
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

    /// The expression the deck's `.param` card carries, which the engine
    /// resolves: the value as typed, with a number written with its unit read
    /// as that number.
    pub fn deck_expression(&self) -> String {
        deck_text(&self.expression, self.quantity)
    }

    /// The deck's card for this variable. The engine resolves the expression
    /// against the plan's other variables, so a variable that names another
    /// moves with it. Validation is kept separate so callers can aggregate
    /// every project diagnostic at once.
    pub fn netlist_statement(&self) -> String {
        format!(".param {}={{{}}}", self.name, self.deck_expression())
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

/// `text` as the engine reads it for a variable labelled `quantity`: a number
/// written with the label's unit is that number, and anything else is an
/// expression, without the braces a deck would wrap it in.
pub(crate) fn deck_text(text: &str, quantity: DesignVariableQuantity) -> String {
    match parse_design_quantity(text, quantity) {
        Ok(value) => format!("{value:.17e}"),
        Err(_) => expression_text(text),
    }
}

/// `text` without surrounding blanks or the one pair of braces a deck wraps
/// an expression in.
fn expression_text(text: &str) -> String {
    let text = text.trim();
    text.strip_prefix('{')
        .and_then(|inner| inner.strip_suffix('}'))
        .unwrap_or(text)
        .trim()
        .to_owned()
}

/// What `text` resolves to with no other variable known: the number, `None`
/// when it names another variable (the plan resolves those), or why the
/// engine cannot read it.
fn standalone_value(text: &str, quantity: DesignVariableQuantity) -> Result<Option<f64>, String> {
    use rspice_core::netlist::expr::{ExprError, ParamContext, evaluate, parse_expression};

    let expression =
        parse_expression(&deck_text(text, quantity)).map_err(|error| error.to_string())?;
    match evaluate(&expression, &ParamContext::new()) {
        Ok(value) if value.is_finite() => Ok(Some(value)),
        Ok(_) => Err("it is not a finite number".to_owned()),
        Err(ExprError::UndefinedParam(_)) => Ok(None),
        Err(error) => Err(error.to_string()),
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

    /// The deck's card is the one the engine resolves to the typed value,
    /// and a typed value outside its own range is refused as it is typed.
    #[test]
    fn a_typed_value_reaches_the_deck_and_one_outside_its_range_is_refused() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let statement = variable.netlist_statement();
        assert!(statement.starts_with(".param RLOAD={"), "{statement}");
        let deck = format!("card\n{statement}\n.end\n");
        let parsed = rspice_core::Netlist::parse(&deck).expect("the card parses");
        assert_eq!(parsed.params.get("RLOAD"), Some(10_000.0));

        let mut outside = variable;
        outside.expression = "2 Mohm".to_owned();
        assert!(outside.validate().unwrap_err().contains("outside"));
    }

    /// A value that names another variable cannot be resolved alone, so the
    /// record is accepted and the plan resolves it; one the engine cannot
    /// read at all is refused.
    #[test]
    fn an_expression_naming_another_variable_is_accepted_and_a_garbled_one_refused() {
        let mut variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        variable.expression = "2*RBASE".to_owned();
        variable.validate().expect("the plan resolves RBASE");
        assert_eq!(variable.netlist_statement(), ".param RLOAD={2*RBASE}");

        variable.expression = "2*(RBASE".to_owned();
        assert!(variable.validate().is_err());
    }

    #[test]
    fn a_missing_or_null_row_identity_is_rejected() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let mut value = serde_json::to_value(variable).unwrap();
        value.as_object_mut().unwrap().remove("id");
        assert!(serde_json::from_value::<DesignVariable>(value.clone()).is_err());
        value
            .as_object_mut()
            .unwrap()
            .insert("id".to_owned(), serde_json::Value::Null);
        assert!(serde_json::from_value::<DesignVariable>(value).is_err());
    }
}
