//! Authored SOA rules and the evidence retained with a completed result.

use super::{SoaPowerDerating, SoaPowerDeratingEvidence};
use serde::{Deserialize, Serialize};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_envelope: Option<super::SoaCurrentEnvelope>,
    #[serde(default, skip_serializing_if = "super::SoaDurationMode::is_default")]
    pub duration_mode: super::SoaDurationMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_duration_s: Option<f64>,
    #[serde(default)]
    pub power_derating: Option<SoaPowerDerating>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope: Option<super::SoaCurrentEnvelopeEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<super::SoaDurationEvidence>,
    #[serde(default, skip_serializing_if = "super::SoaThresholds::is_default")]
    pub thresholds: super::SoaThresholds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derating: Option<SoaPowerDeratingEvidence>,
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
