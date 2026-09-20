//! Safe Operating Area (SOA) Manager
//!
//! Provides commercial-grade safety checking for circuit devices.
//! Monitors configured terminal-voltage and current stress against device limits.
//!
//! # Features
//!
//! - Multi-parameter limit checking (Vgs, Vds, Vbe, Vce)
//! - Warning, violation, and critical threshold classification
//! - Complete sampled-rule coverage and exact worst-point retention
//! - Integration with Schematic and Waveform viewers for visual alerts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// SOA Limits
// =============================================================================

/// Type of SOA limit parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SoAParameter {
    Vgs,
    Vds,
    Vgd,
    Vbe,
    Vce,
    Vbc,
    Id,
    Ic,
    Pdiss,
    Temp,
    VgsPositive,
    VgsNegative,
    VdsPositive,
    VdsNegative,
    VgdPositive,
    VgdNegative,
    VbePositive,
    VbeNegative,
    VcePositive,
    VceNegative,
    VbcPositive,
    VbcNegative,
    IdPositive,
    IdNegative,
    IcPositive,
    IcNegative,
    Ig,
    IgPositive,
    IgNegative,
    Is,
    IsPositive,
    IsNegative,
    Ib,
    IbPositive,
    IbNegative,
    Ie,
    IePositive,
    IeNegative,
    Vbs,
    VbsPositive,
    VbsNegative,
    Vbd,
    VbdPositive,
    VbdNegative,
    Vgb,
    VgbPositive,
    VgbNegative,
    Ibulk,
    IbulkPositive,
    IbulkNegative,
    Ves,
    VesPositive,
    VesNegative,
    Ved,
    VedPositive,
    VedNegative,
    Vge,
    VgePositive,
    VgeNegative,
    Ibackgate,
    IbackgatePositive,
    IbackgateNegative,
    VbodyBackgate,
    VbodyBackgatePositive,
    VbodyBackgateNegative,
    Vcsub,
    VcsubPositive,
    VcsubNegative,
    Vbsub,
    VbsubPositive,
    VbsubNegative,
    Vesub,
    VesubPositive,
    VesubNegative,
    Isub,
    IsubPositive,
    IsubNegative,
    Vak,
    VakPositive,
    VakNegative,
    Ia,
    IaPositive,
    IaNegative,
}

impl SoAParameter {
    /// Stable stress-trace code for this parameter.
    #[must_use]
    pub const fn stress_code(self) -> &'static str {
        match self {
            Self::Vgs => "VGS",
            Self::Vds => "VDS",
            Self::Vgd => "VGD",
            Self::Vbe => "VBE",
            Self::Vce => "VCE",
            Self::Vbc => "VBC",
            Self::Id => "ID",
            Self::Ic => "IC",
            Self::Pdiss => "PDISS",
            Self::Temp => "TEMP",
            Self::VgsPositive => "VGS_POS",
            Self::VgsNegative => "VGS_NEG",
            Self::VdsPositive => "VDS_POS",
            Self::VdsNegative => "VDS_NEG",
            Self::VgdPositive => "VGD_POS",
            Self::VgdNegative => "VGD_NEG",
            Self::VbePositive => "VBE_POS",
            Self::VbeNegative => "VBE_NEG",
            Self::VcePositive => "VCE_POS",
            Self::VceNegative => "VCE_NEG",
            Self::VbcPositive => "VBC_POS",
            Self::VbcNegative => "VBC_NEG",
            Self::IdPositive => "ID_POS",
            Self::IdNegative => "ID_NEG",
            Self::IcPositive => "IC_POS",
            Self::IcNegative => "IC_NEG",
            Self::Ig => "IG",
            Self::IgPositive => "IG_POS",
            Self::IgNegative => "IG_NEG",
            Self::Is => "IS",
            Self::IsPositive => "IS_POS",
            Self::IsNegative => "IS_NEG",
            Self::Ib => "IB",
            Self::IbPositive => "IB_POS",
            Self::IbNegative => "IB_NEG",
            Self::Ie => "IE",
            Self::IePositive => "IE_POS",
            Self::IeNegative => "IE_NEG",
            Self::Vbs => "VBS",
            Self::VbsPositive => "VBS_POS",
            Self::VbsNegative => "VBS_NEG",
            Self::Vbd => "VBD",
            Self::VbdPositive => "VBD_POS",
            Self::VbdNegative => "VBD_NEG",
            Self::Vgb => "VGB",
            Self::VgbPositive => "VGB_POS",
            Self::VgbNegative => "VGB_NEG",
            Self::Ibulk => "IBULK",
            Self::IbulkPositive => "IBULK_POS",
            Self::IbulkNegative => "IBULK_NEG",
            Self::Ves => "VES",
            Self::VesPositive => "VES_POS",
            Self::VesNegative => "VES_NEG",
            Self::Ved => "VED",
            Self::VedPositive => "VED_POS",
            Self::VedNegative => "VED_NEG",
            Self::Vge => "VGE",
            Self::VgePositive => "VGE_POS",
            Self::VgeNegative => "VGE_NEG",
            Self::Ibackgate => "IBACKGATE",
            Self::IbackgatePositive => "IBACKGATE_POS",
            Self::IbackgateNegative => "IBACKGATE_NEG",
            Self::VbodyBackgate => "VBODY_BACKGATE",
            Self::VbodyBackgatePositive => "VBODY_BACKGATE_POS",
            Self::VbodyBackgateNegative => "VBODY_BACKGATE_NEG",
            Self::Vcsub => "VCSUB",
            Self::VcsubPositive => "VCSUB_POS",
            Self::VcsubNegative => "VCSUB_NEG",
            Self::Vbsub => "VBSUB",
            Self::VbsubPositive => "VBSUB_POS",
            Self::VbsubNegative => "VBSUB_NEG",
            Self::Vesub => "VESUB",
            Self::VesubPositive => "VESUB_POS",
            Self::VesubNegative => "VESUB_NEG",
            Self::Isub => "ISUB",
            Self::IsubPositive => "ISUB_POS",
            Self::IsubNegative => "ISUB_NEG",
            Self::Vak => "VAK",
            Self::VakPositive => "VAK_POS",
            Self::VakNegative => "VAK_NEG",
            Self::Ia => "IA",
            Self::IaPositive => "IA_POS",
            Self::IaNegative => "IA_NEG",
        }
    }
    /// Signed physical observation at intrinsic electrical model nodes.
    pub const fn intrinsic_voltage_parameter(self) -> Option<&'static str> {
        match self.base_parameter() {
            Self::Vgs => Some("vgs_intrinsic"),
            Self::Vds => Some("vds_intrinsic"),
            Self::Vgd => Some("vgd_intrinsic"),
            Self::Vbs => Some("vbs_intrinsic"),
            Self::Vbd => Some("vbd_intrinsic"),
            Self::Vgb => Some("vgb_intrinsic"),
            Self::Ves => Some("ves_intrinsic"),
            Self::Ved => Some("ved_intrinsic"),
            Self::Vge => Some("vge_intrinsic"),
            Self::VbodyBackgate => Some("vbody_backgate_intrinsic"),
            Self::Vbe => Some("vbe_intrinsic"),
            Self::Vce => Some("vce_intrinsic"),
            Self::Vbc => Some("vbc_intrinsic"),
            Self::Vcsub => Some("vcsub_intrinsic"),
            Self::Vbsub => Some("vbsub_intrinsic"),
            Self::Vesub => Some("vesub_intrinsic"),
            Self::Vak => Some("vak_intrinsic"),
            _ => None,
        }
    }

    /// The unsigned terminal quantity underlying a directional constraint.
    pub const fn base_parameter(self) -> Self {
        match self {
            Self::VgsPositive | Self::VgsNegative => Self::Vgs,
            Self::VdsPositive | Self::VdsNegative => Self::Vds,
            Self::VgdPositive | Self::VgdNegative => Self::Vgd,
            Self::VbePositive | Self::VbeNegative => Self::Vbe,
            Self::VcePositive | Self::VceNegative => Self::Vce,
            Self::VbcPositive | Self::VbcNegative => Self::Vbc,
            Self::IdPositive | Self::IdNegative => Self::Id,
            Self::IcPositive | Self::IcNegative => Self::Ic,
            Self::IgPositive | Self::IgNegative => Self::Ig,
            Self::IsPositive | Self::IsNegative => Self::Is,
            Self::IbPositive | Self::IbNegative => Self::Ib,
            Self::IePositive | Self::IeNegative => Self::Ie,
            Self::VbsPositive | Self::VbsNegative => Self::Vbs,
            Self::VbdPositive | Self::VbdNegative => Self::Vbd,
            Self::VgbPositive | Self::VgbNegative => Self::Vgb,
            Self::IbulkPositive | Self::IbulkNegative => Self::Ibulk,
            Self::VesPositive | Self::VesNegative => Self::Ves,
            Self::VedPositive | Self::VedNegative => Self::Ved,
            Self::VgePositive | Self::VgeNegative => Self::Vge,
            Self::IbackgatePositive | Self::IbackgateNegative => Self::Ibackgate,
            Self::VbodyBackgatePositive | Self::VbodyBackgateNegative => Self::VbodyBackgate,
            Self::VcsubPositive | Self::VcsubNegative => Self::Vcsub,
            Self::VbsubPositive | Self::VbsubNegative => Self::Vbsub,
            Self::VesubPositive | Self::VesubNegative => Self::Vesub,
            Self::IsubPositive | Self::IsubNegative => Self::Isub,
            Self::VakPositive | Self::VakNegative => Self::Vak,
            Self::IaPositive | Self::IaNegative => Self::Ia,
            other => other,
        }
    }

    /// True for the positive part, false for the negative part; None is magnitude.
    pub const fn polarity(self) -> Option<bool> {
        match self {
            Self::VgsPositive
            | Self::VdsPositive
            | Self::VgdPositive
            | Self::VbePositive
            | Self::VcePositive
            | Self::VbcPositive
            | Self::IdPositive
            | Self::IcPositive
            | Self::IgPositive
            | Self::IsPositive
            | Self::IbPositive
            | Self::IePositive
            | Self::VbsPositive
            | Self::VbdPositive
            | Self::VgbPositive
            | Self::IbulkPositive
            | Self::VesPositive
            | Self::VedPositive
            | Self::VgePositive
            | Self::IbackgatePositive
            | Self::VbodyBackgatePositive
            | Self::VcsubPositive
            | Self::VbsubPositive
            | Self::VesubPositive
            | Self::IsubPositive
            | Self::VakPositive
            | Self::IaPositive => Some(true),
            Self::VgsNegative
            | Self::VdsNegative
            | Self::VgdNegative
            | Self::VbeNegative
            | Self::VceNegative
            | Self::VbcNegative
            | Self::IdNegative
            | Self::IcNegative
            | Self::IgNegative
            | Self::IsNegative
            | Self::IbNegative
            | Self::IeNegative
            | Self::VbsNegative
            | Self::VbdNegative
            | Self::VgbNegative
            | Self::IbulkNegative
            | Self::VesNegative
            | Self::VedNegative
            | Self::VgeNegative
            | Self::IbackgateNegative
            | Self::VbodyBackgateNegative
            | Self::VcsubNegative
            | Self::VbsubNegative
            | Self::VesubNegative
            | Self::IsubNegative
            | Self::VakNegative
            | Self::IaNegative => Some(false),
            _ => None,
        }
    }

    pub const fn directional_pair(self) -> Option<(Self, Self)> {
        match self.base_parameter() {
            Self::Vgs => Some((Self::VgsPositive, Self::VgsNegative)),
            Self::Vds => Some((Self::VdsPositive, Self::VdsNegative)),
            Self::Vgd => Some((Self::VgdPositive, Self::VgdNegative)),
            Self::Vbe => Some((Self::VbePositive, Self::VbeNegative)),
            Self::Vce => Some((Self::VcePositive, Self::VceNegative)),
            Self::Vbc => Some((Self::VbcPositive, Self::VbcNegative)),
            Self::Id => Some((Self::IdPositive, Self::IdNegative)),
            Self::Ic => Some((Self::IcPositive, Self::IcNegative)),
            Self::Vcsub => Some((Self::VcsubPositive, Self::VcsubNegative)),
            Self::Vbsub => Some((Self::VbsubPositive, Self::VbsubNegative)),
            Self::Vesub => Some((Self::VesubPositive, Self::VesubNegative)),
            Self::Isub => Some((Self::IsubPositive, Self::IsubNegative)),
            Self::Vak => Some((Self::VakPositive, Self::VakNegative)),
            Self::Ia => Some((Self::IaPositive, Self::IaNegative)),

            Self::Ig => Some((Self::IgPositive, Self::IgNegative)),
            Self::Is => Some((Self::IsPositive, Self::IsNegative)),
            Self::Ib => Some((Self::IbPositive, Self::IbNegative)),
            Self::Ie => Some((Self::IePositive, Self::IeNegative)),
            Self::Vbs => Some((Self::VbsPositive, Self::VbsNegative)),
            Self::Vbd => Some((Self::VbdPositive, Self::VbdNegative)),
            Self::Vgb => Some((Self::VgbPositive, Self::VgbNegative)),
            Self::Ibulk => Some((Self::IbulkPositive, Self::IbulkNegative)),
            Self::Ves => Some((Self::VesPositive, Self::VesNegative)),
            Self::Ved => Some((Self::VedPositive, Self::VedNegative)),
            Self::Vge => Some((Self::VgePositive, Self::VgeNegative)),
            Self::Ibackgate => Some((Self::IbackgatePositive, Self::IbackgateNegative)),
            Self::VbodyBackgate => Some((Self::VbodyBackgatePositive, Self::VbodyBackgateNegative)),

            _ => None,
        }
    }

    /// True for an authored electrical terminal current, in amperes.
    pub const fn is_current(self) -> bool {
        matches!(
            self.base_parameter(),
            Self::Id
                | Self::Ic
                | Self::Ig
                | Self::Is
                | Self::Ib
                | Self::Ie
                | Self::Ibulk
                | Self::Ibackgate
                | Self::Isub
                | Self::Ia
        )
    }

    /// Body/back-gate rules need the selected model's external pin roles.
    pub const fn requires_mos_layout(self) -> bool {
        matches!(
            self.base_parameter(),
            Self::Vbs
                | Self::Vbd
                | Self::Vgb
                | Self::Ibulk
                | Self::Ves
                | Self::Ved
                | Self::Vge
                | Self::Ibackgate
                | Self::VbodyBackgate
        )
    }

    /// Substrate rules need an explicit electrical pin, not a thermal port.
    pub const fn requires_bjt_layout(self) -> bool {
        matches!(
            self.base_parameter(),
            Self::Vcsub | Self::Vbsub | Self::Vesub | Self::Isub
        )
    }

    pub const fn requires_terminal_layout(self) -> bool {
        self.requires_mos_layout() || self.requires_bjt_layout()
    }

    /// Input must be finite; polarity follows authored terminal order, not model type.
    pub fn measured_stress(self, signed: f64) -> f64 {
        // Only consumed conductive power contributes to a dissipation limit;
        // a negative signed model contribution is not positive heat.
        if self == Self::Pdiss {
            return signed.max(0.0);
        }

        match self.polarity() {
            Some(true) => signed.max(0.0),
            Some(false) => (-signed).max(0.0),
            None => signed.abs(),
        }
    }
}

/// Canonical retained-waveform name for one rule's stress history.
///
/// The producer and the Results viewer both address the history by this name,
/// so it has exactly one owner: a second spelling anywhere would silently
/// disconnect a rule from its own trace.
#[must_use]
pub fn soa_stress_waveform_name(device_id: &str, parameter: SoAParameter) -> String {
    format!("SOA_{}({device_id})", parameter.stress_code())
}

/// Where a voltage rule observes the device. Currents remain authored pin totals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoaVoltageBasis {
    #[default]
    ExternalTerminals,
    IntrinsicNodes,
}

/// A specific limit definition for a device type or model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoALimit {
    #[serde(default)]
    pub voltage_basis: SoaVoltageBasis,
    pub parameter: SoAParameter,
    pub max_value: f64,
    pub unit: String,
    pub description: String,
}

/// Collection of limits for a specific device instance or model
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SoADefinition {
    pub limits: Vec<SoALimit>,
}

impl SoADefinition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_limit(&mut self, limit: SoALimit) {
        self.limits.push(limit);
    }
}

// =============================================================================
// SOA Violations
// =============================================================================

/// Details of a safety limit violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoAViolation {
    pub device_id: String,
    pub parameter: SoAParameter,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time: f64, // Simulation time of violation
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Near limit (e.g. > 90%)
    Warning,
    /// Limit exceeded
    Violation,
    /// Significant exceedence / Fatal path
    Critical,
}

/// Verdict for one completely evaluated SOA rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SoARuleVerdict {
    Pass,
    Warning,
    Violation,
    Critical,
}

/// Worst observed point and coverage for one device/parameter rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoAEvaluation {
    pub device_id: String,
    pub parameter: SoAParameter,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: SoARuleVerdict,
}

// =============================================================================
// SOA Manager
// =============================================================================

/// Manager for tracking and checking Safe Operating Area limits
pub struct SoAManager {
    /// Device instance ID -> SOA Definition
    device_defs: HashMap<String, SoADefinition>,
    /// Accumulated violations from most recent check
    violations: Vec<SoAViolation>,
    /// Complete evaluated-rule coverage, keyed by stable device/parameter identity.
    evaluations: HashMap<(String, SoAParameter), SoAEvaluation>,
    /// Every sampled stress magnitude per rule, in `check_point` order.
    ///
    /// Kept beside `evaluations` and written in the same step, so a rule's
    /// history can never disagree with the worst point derived from it.
    stress_history: HashMap<(String, SoAParameter), Vec<f64>>,
}

impl Default for SoAManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoAManager {
    pub fn new() -> Self {
        Self {
            device_defs: HashMap::new(),
            violations: Vec::new(),
            evaluations: HashMap::new(),
            stress_history: HashMap::new(),
        }
    }

    /// Register SOA limits for a device
    pub fn register_device(
        &mut self,
        device_id: impl Into<String>,
        def: SoADefinition,
    ) -> Result<(), String> {
        let device_id = device_id.into();
        if device_id.trim().is_empty() {
            return Err("SOA device identity is empty".to_owned());
        }
        if def.limits.is_empty() {
            return Err(format!("SOA device '{device_id}' has no enabled rules"));
        }
        let mut parameters = std::collections::HashSet::new();
        for limit in &def.limits {
            if !parameters.insert(limit.parameter) {
                return Err(format!(
                    "SOA device '{device_id}' has duplicate rules for {:?}",
                    limit.parameter
                ));
            }
            if !limit.max_value.is_finite()
                || limit.max_value < 0.0
                || (limit.max_value == 0.0 && limit.parameter.polarity().is_none())
            {
                return Err(format!(
                    "SOA device '{device_id}' has an invalid {:?} limit",
                    limit.parameter
                ));
            }
            if limit.unit.trim().is_empty() || limit.description.trim().is_empty() {
                return Err(format!(
                    "SOA device '{device_id}' has incomplete {:?} rule metadata",
                    limit.parameter
                ));
            }
        }
        if self.device_defs.contains_key(&device_id) {
            return Err(format!("SOA device '{device_id}' was registered twice"));
        }
        self.device_defs.insert(device_id, def);
        Ok(())
    }

    /// Clear all violations
    #[cfg(test)]
    pub fn clear_violations(&mut self) {
        self.violations.clear();
        self.evaluations.clear();
    }

    /// Check a single measurement point for all registered devices
    pub fn check_point(
        &mut self,
        time: f64,
        values: &HashMap<String, HashMap<SoAParameter, f64>>,
    ) -> Result<(), String> {
        if !time.is_finite() || time < 0.0 {
            return Err("SOA sample time must be finite and nonnegative".to_owned());
        }
        for (device_id, device_values) in values {
            if let Some(def) = self.device_defs.get(device_id) {
                for limit in &def.limits {
                    if let Some(&actual) = device_values.get(&limit.parameter) {
                        if !actual.is_finite() || actual < 0.0 {
                            return Err(format!(
                                "SOA device '{device_id}' has an invalid {:?} sample",
                                limit.parameter
                            ));
                        }
                        let verdict = rule_verdict(actual, limit.max_value);
                        let key = (device_id.clone(), limit.parameter);
                        self.stress_history
                            .entry(key.clone())
                            .or_default()
                            .push(actual);
                        let evaluation =
                            self.evaluations
                                .entry(key)
                                .or_insert_with(|| SoAEvaluation {
                                    device_id: device_id.clone(),
                                    parameter: limit.parameter,
                                    limit_value: limit.max_value,
                                    worst_actual_value: actual,
                                    worst_time: time,
                                    sample_count: 0,
                                    unit: limit.unit.clone(),
                                    description: limit.description.clone(),
                                    verdict,
                                });
                        evaluation.sample_count =
                            evaluation.sample_count.checked_add(1).ok_or_else(|| {
                                format!("SOA sample count overflow for device '{device_id}'")
                            })?;
                        if actual > evaluation.worst_actual_value {
                            evaluation.worst_actual_value = actual;
                            evaluation.worst_time = time;
                            evaluation.verdict = verdict;
                        }
                        let severity = match verdict {
                            SoARuleVerdict::Pass => None,
                            SoARuleVerdict::Warning => Some(ViolationSeverity::Warning),
                            SoARuleVerdict::Violation => Some(ViolationSeverity::Violation),
                            SoARuleVerdict::Critical => Some(ViolationSeverity::Critical),
                        };
                        if let Some(severity) = severity {
                            self.violations.push(SoAViolation {
                                device_id: device_id.clone(),
                                parameter: limit.parameter,
                                limit_value: limit.max_value,
                                actual_value: actual,
                                time,
                                severity,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Get all detected violations
    pub fn violations(&self) -> &[SoAViolation] {
        &self.violations
    }

    /// Iterate the complete evaluated-rule set. Callers that persist or
    /// transport these records must impose canonical ordering.
    pub fn evaluations(&self) -> impl Iterator<Item = &SoAEvaluation> {
        self.evaluations.values()
    }

    /// The sampled stress history for one evaluated rule, in sample order.
    ///
    /// Its length always equals that rule's `sample_count`; a rule the run
    /// never sampled has no entry at all.
    pub fn stress_history(&self, device_id: &str, parameter: SoAParameter) -> Option<&[f64]> {
        self.stress_history
            .get(&(device_id.to_owned(), parameter))
            .map(Vec::as_slice)
    }
}

fn rule_verdict(actual: f64, maximum: f64) -> SoARuleVerdict {
    if actual > maximum * 1.2 {
        SoARuleVerdict::Critical
    } else if actual > maximum {
        SoARuleVerdict::Violation
    } else if actual > maximum * 0.9 {
        SoARuleVerdict::Warning
    } else {
        SoARuleVerdict::Pass
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_retains_complete_rule_coverage_and_exact_worst_point() {
        let mut manager = SoAManager::new();
        manager
            .register_device(
                "M1",
                SoADefinition {
                    limits: vec![SoALimit {
                        voltage_basis: Default::default(),
                        parameter: SoAParameter::Vds,
                        max_value: 10.0,
                        unit: "V".to_owned(),
                        description: "Maximum drain-source voltage".to_owned(),
                    }],
                },
            )
            .expect("valid SOA rule registers");
        assert!(
            manager
                .register_device(
                    "M1",
                    SoADefinition {
                        limits: vec![SoALimit {
                            voltage_basis: Default::default(),
                            parameter: SoAParameter::Vds,
                            max_value: 1.0,
                            unit: "V".to_owned(),
                            description: "Conflicting duplicate".to_owned(),
                        }],
                    },
                )
                .is_err()
        );

        for (time, actual) in [(0.0, 5.0), (1.0, 9.5), (2.0, 12.5)] {
            manager
                .check_point(
                    time,
                    &HashMap::from([(
                        "M1".to_owned(),
                        HashMap::from([(SoAParameter::Vds, actual)]),
                    )]),
                )
                .expect("finite SOA sample evaluates");
        }

        let evaluations = manager.evaluations().collect::<Vec<_>>();
        assert_eq!(evaluations.len(), 1);
        assert_eq!(evaluations[0].sample_count, 3);
        assert_eq!(evaluations[0].limit_value, 10.0);
        assert_eq!(evaluations[0].worst_actual_value, 12.5);
        assert_eq!(evaluations[0].worst_time, 2.0);
        assert_eq!(evaluations[0].verdict, SoARuleVerdict::Critical);
        assert_eq!(manager.violations().len(), 2);
        assert_eq!(manager.violations()[0].severity, ViolationSeverity::Warning);
        assert_eq!(
            manager.violations()[1].severity,
            ViolationSeverity::Critical
        );

        manager.clear_violations();
        assert!(manager.violations().is_empty());
        assert_eq!(manager.evaluations().count(), 0);
    }
}
