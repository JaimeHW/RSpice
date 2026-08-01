//! Design intent: the specs a result is judged against and the variables that
//! parameterise it.
//!
//! A spec pins a `.MEAS` result as a tracked row whether or not it carries
//! bounds, so removing a limit never silently drops the measurement.  A design
//! variable declares its quantity, scope, and override policy explicitly,
//! because the same name can be legitimately overridden at a narrower scope
//! and the policy is what decides whether that is allowed.

use super::*;

/// One specification bound for a `.MEAS` result — a row of the specs
/// matrix. At least one of `min`/`max` is normally set; a spec with
/// neither still pins the measurement as a tracked row (value-only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecEntry {
    /// `.MEAS` result name this spec bounds (case-insensitive match).
    pub measurement: String,
    /// Lower bound (pass when value ≥ min).
    pub min: Option<f64>,
    /// Upper bound (pass when value ≤ max).
    pub max: Option<f64>,
    /// Display unit, purely cosmetic (e.g. "V", "s", "dB").
    pub unit: String,
}

impl SpecEntry {
    pub fn validate(&self) -> Result<(), String> {
        validate_parameter_name(&self.measurement)
            .map_err(|error| format!("measurement name is invalid: {error}"))?;
        if self.min.is_some_and(|value| !value.is_finite())
            || self.max.is_some_and(|value| !value.is_finite())
        {
            return Err("specification bounds must be finite".to_owned());
        }
        if self
            .min
            .zip(self.max)
            .is_some_and(|(minimum, maximum)| minimum > maximum)
        {
            return Err("specification minimum exceeds its maximum".to_owned());
        }
        validate_bounded_text("unit", &self.unit, 64, true)
    }

    /// Spec verdict for one measured value.
    pub fn passes(&self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }

    /// Violation magnitude (how far outside the bounds), 0 when passing.
    pub fn violation(&self, value: f64) -> f64 {
        let below = self.min.map_or(0.0, |min| (min - value).max(0.0));
        let above = self.max.map_or(0.0, |max| (value - max).max(0.0));
        below.max(above)
    }
}

/// Physical quantity carried by a design variable. The quantity is retained
/// independently from the expression so editors can validate units without
/// coercing the user's exact engineering input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableQuantity {
    Resistance,
    Capacitance,
    Voltage,
    Current,
    Temperature,
    Dimensionless,
}

impl DesignVariableQuantity {
    pub const ALL: [Self; 6] = [
        Self::Resistance,
        Self::Capacitance,
        Self::Voltage,
        Self::Current,
        Self::Temperature,
        Self::Dimensionless,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Resistance => "Resistance",
            Self::Capacitance => "Capacitance",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Temperature => "Temperature",
            Self::Dimensionless => "Dimensionless",
        }
    }
}

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
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Testbench => "Lab characterization · testbench",
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
        variable.validate()?;
        Ok(variable)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_parameter_name(&self.name)?;
        validate_single_line_expression("expression", &self.expression)?;
        let value = self.resolved_value_si()?;
        if let Some(range) = &self.allowed_range {
            validate_single_line_expression("allowed-range minimum", &range.minimum)?;
            validate_single_line_expression("allowed-range maximum", &range.maximum)?;
            let minimum =
                parse_design_quantity(&range.minimum, self.quantity).map_err(|error| {
                    format!(
                        "allowed-range minimum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })?;
            let maximum =
                parse_design_quantity(&range.maximum, self.quantity).map_err(|error| {
                    format!(
                        "allowed-range maximum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })?;
            if minimum > maximum {
                return Err("allowed-range minimum exceeds its maximum".to_owned());
            }
            if value < minimum || value > maximum {
                return Err(format!(
                    "resolved value {value} is outside the inclusive allowed range {minimum}..={maximum}"
                ));
            }
        }
        validate_bounded_text("description", &self.description, 1_024, true)?;
        if let DesignVariableScope::SelectedCell { cell } = &self.scope {
            cell.validate_name_segments()
                .map_err(|error| format!("selected cell is invalid: {error}"))?;
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

    pub(super) fn cloned_for_new_plan(
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

pub(super) const LEGACY_DESIGN_VARIABLE_ID_NAMESPACE: Uuid =
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
