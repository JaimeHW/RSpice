//! Retained safe-operating-area rule and violation evidence.

/// Electrical quantity governed by a retained safe-operating-area rule.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaParameterEvidence {
    GateSourceVoltage,
    DrainSourceVoltage,
    GateDrainVoltage,
    BaseEmitterVoltage,
    CollectorEmitterVoltage,
    BaseCollectorVoltage,
    DrainCurrent,
    CollectorCurrent,
    PowerDissipation,
    Temperature,
    GateSourceVoltagePositive,
    GateSourceVoltageNegative,
    DrainSourceVoltagePositive,
    DrainSourceVoltageNegative,
    GateDrainVoltagePositive,
    GateDrainVoltageNegative,
    BaseEmitterVoltagePositive,
    BaseEmitterVoltageNegative,
    CollectorEmitterVoltagePositive,
    CollectorEmitterVoltageNegative,
    BaseCollectorVoltagePositive,
    BaseCollectorVoltageNegative,
    DrainCurrentPositive,
    DrainCurrentNegative,
    CollectorCurrentPositive,
    CollectorCurrentNegative,
    GateCurrent,
    GateCurrentPositive,
    GateCurrentNegative,
    SourceCurrent,
    SourceCurrentPositive,
    SourceCurrentNegative,
    BaseCurrent,
    BaseCurrentPositive,
    BaseCurrentNegative,
    EmitterCurrent,
    EmitterCurrentPositive,
    EmitterCurrentNegative,
    BodySourceVoltage,
    BodySourceVoltagePositive,
    BodySourceVoltageNegative,
    BodyDrainVoltage,
    BodyDrainVoltagePositive,
    BodyDrainVoltageNegative,
    GateBodyVoltage,
    GateBodyVoltagePositive,
    GateBodyVoltageNegative,
    BulkCurrent,
    BulkCurrentPositive,
    BulkCurrentNegative,
    BackgateSourceVoltage,
    BackgateSourceVoltagePositive,
    BackgateSourceVoltageNegative,
    BackgateDrainVoltage,
    BackgateDrainVoltagePositive,
    BackgateDrainVoltageNegative,
    GateBackgateVoltage,
    GateBackgateVoltagePositive,
    GateBackgateVoltageNegative,
    BackgateCurrent,
    BackgateCurrentPositive,
    BackgateCurrentNegative,
    BodyBackgateVoltage,
    BodyBackgateVoltagePositive,
    BodyBackgateVoltageNegative,
    CollectorSubstrateVoltage,
    CollectorSubstrateVoltagePositive,
    CollectorSubstrateVoltageNegative,
    BaseSubstrateVoltage,
    BaseSubstrateVoltagePositive,
    BaseSubstrateVoltageNegative,
    EmitterSubstrateVoltage,
    EmitterSubstrateVoltagePositive,
    EmitterSubstrateVoltageNegative,
    SubstrateCurrent,
    SubstrateCurrentPositive,
    SubstrateCurrentNegative,
    AnodeCathodeVoltage,
    AnodeCathodeVoltagePositive,
    AnodeCathodeVoltageNegative,
    AnodeCurrent,
    AnodeCurrentPositive,
    AnodeCurrentNegative,
}

impl SoaParameterEvidence {
    pub const fn is_directional(self) -> bool {
        matches!(
            self,
            Self::GateSourceVoltagePositive
                | Self::GateSourceVoltageNegative
                | Self::DrainSourceVoltagePositive
                | Self::DrainSourceVoltageNegative
                | Self::GateDrainVoltagePositive
                | Self::GateDrainVoltageNegative
                | Self::BaseEmitterVoltagePositive
                | Self::BaseEmitterVoltageNegative
                | Self::CollectorEmitterVoltagePositive
                | Self::CollectorEmitterVoltageNegative
                | Self::BaseCollectorVoltagePositive
                | Self::BaseCollectorVoltageNegative
                | Self::DrainCurrentPositive
                | Self::DrainCurrentNegative
                | Self::CollectorCurrentPositive
                | Self::CollectorCurrentNegative
                | Self::GateCurrentPositive
                | Self::GateCurrentNegative
                | Self::SourceCurrentPositive
                | Self::SourceCurrentNegative
                | Self::BaseCurrentPositive
                | Self::BaseCurrentNegative
                | Self::EmitterCurrentPositive
                | Self::EmitterCurrentNegative
                | Self::BodySourceVoltagePositive
                | Self::BodySourceVoltageNegative
                | Self::BodyDrainVoltagePositive
                | Self::BodyDrainVoltageNegative
                | Self::GateBodyVoltagePositive
                | Self::GateBodyVoltageNegative
                | Self::BulkCurrentPositive
                | Self::BulkCurrentNegative
                | Self::BackgateSourceVoltagePositive
                | Self::BackgateSourceVoltageNegative
                | Self::BackgateDrainVoltagePositive
                | Self::BackgateDrainVoltageNegative
                | Self::GateBackgateVoltagePositive
                | Self::GateBackgateVoltageNegative
                | Self::BackgateCurrentPositive
                | Self::BackgateCurrentNegative
                | Self::BodyBackgateVoltagePositive
                | Self::BodyBackgateVoltageNegative
                | Self::CollectorSubstrateVoltagePositive
                | Self::CollectorSubstrateVoltageNegative
                | Self::BaseSubstrateVoltagePositive
                | Self::BaseSubstrateVoltageNegative
                | Self::EmitterSubstrateVoltagePositive
                | Self::EmitterSubstrateVoltageNegative
                | Self::SubstrateCurrentPositive
                | Self::SubstrateCurrentNegative
                | Self::AnodeCathodeVoltagePositive
                | Self::AnodeCathodeVoltageNegative
                | Self::AnodeCurrentPositive
                | Self::AnodeCurrentNegative
        )
    }
}

/// Severity assigned by the SOA rule evaluator.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaViolationSeverityEvidence {
    Warning,
    Violation,
    Critical,
}

/// Verdict for one fully evaluated safe-operating-area rule.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaRuleVerdictEvidence {
    Pass,
    Warning,
    Violation,
    Critical,
}

impl SoaRuleVerdictEvidence {
    /// How this verdict is named wherever it is reported — the SOA sheet, the
    /// component inspector, and the printed evidence table all read it here so
    /// a rule cannot be called one thing on screen and another on paper.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warning => "WARNING",
            Self::Violation => "VIOLATION",
            Self::Critical => "CRITICAL",
        }
    }
}

/// Complete worst-point and sampling evidence for one SOA rule.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaEvaluationEvidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope: Option<crate::safety::SoaCurrentEnvelopeEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<crate::safety::SoaDurationEvidence>,
    #[serde(
        default,
        skip_serializing_if = "crate::safety::SoaThresholds::is_default"
    )]
    pub thresholds: crate::safety::SoaThresholds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derating: Option<crate::safety::SoaPowerDeratingEvidence>,
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time_s: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: SoaRuleVerdictEvidence,
}

/// One exact, source-attributed safe-operating-area violation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaViolationEvidence {
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time_s: f64,
    pub severity: SoaViolationSeverityEvidence,
}

impl SoaParameterEvidence {
    pub const fn runtime_parameter(self) -> crate::safety::SoAParameter {
        use crate::safety::SoAParameter;
        match self {
            SoaParameterEvidence::GateSourceVoltage => SoAParameter::Vgs,
            SoaParameterEvidence::DrainSourceVoltage => SoAParameter::Vds,
            SoaParameterEvidence::GateDrainVoltage => SoAParameter::Vgd,
            SoaParameterEvidence::BaseEmitterVoltage => SoAParameter::Vbe,
            SoaParameterEvidence::CollectorEmitterVoltage => SoAParameter::Vce,
            SoaParameterEvidence::BaseCollectorVoltage => SoAParameter::Vbc,
            SoaParameterEvidence::DrainCurrent => SoAParameter::Id,
            SoaParameterEvidence::CollectorCurrent => SoAParameter::Ic,
            SoaParameterEvidence::CollectorSubstrateVoltage => SoAParameter::Vcsub,
            SoaParameterEvidence::CollectorSubstrateVoltagePositive => SoAParameter::VcsubPositive,
            SoaParameterEvidence::CollectorSubstrateVoltageNegative => SoAParameter::VcsubNegative,
            SoaParameterEvidence::BaseSubstrateVoltage => SoAParameter::Vbsub,
            SoaParameterEvidence::BaseSubstrateVoltagePositive => SoAParameter::VbsubPositive,
            SoaParameterEvidence::BaseSubstrateVoltageNegative => SoAParameter::VbsubNegative,
            SoaParameterEvidence::EmitterSubstrateVoltage => SoAParameter::Vesub,
            SoaParameterEvidence::EmitterSubstrateVoltagePositive => SoAParameter::VesubPositive,
            SoaParameterEvidence::EmitterSubstrateVoltageNegative => SoAParameter::VesubNegative,
            SoaParameterEvidence::SubstrateCurrent => SoAParameter::Isub,
            SoaParameterEvidence::SubstrateCurrentPositive => SoAParameter::IsubPositive,
            SoaParameterEvidence::SubstrateCurrentNegative => SoAParameter::IsubNegative,
            SoaParameterEvidence::AnodeCathodeVoltage => SoAParameter::Vak,
            SoaParameterEvidence::AnodeCathodeVoltagePositive => SoAParameter::VakPositive,
            SoaParameterEvidence::AnodeCathodeVoltageNegative => SoAParameter::VakNegative,
            SoaParameterEvidence::AnodeCurrent => SoAParameter::Ia,
            SoaParameterEvidence::AnodeCurrentPositive => SoAParameter::IaPositive,
            SoaParameterEvidence::AnodeCurrentNegative => SoAParameter::IaNegative,

            SoaParameterEvidence::BodySourceVoltage => SoAParameter::Vbs,
            SoaParameterEvidence::BodySourceVoltagePositive => SoAParameter::VbsPositive,
            SoaParameterEvidence::BodySourceVoltageNegative => SoAParameter::VbsNegative,
            SoaParameterEvidence::BodyDrainVoltage => SoAParameter::Vbd,
            SoaParameterEvidence::BodyDrainVoltagePositive => SoAParameter::VbdPositive,
            SoaParameterEvidence::BodyDrainVoltageNegative => SoAParameter::VbdNegative,
            SoaParameterEvidence::GateBodyVoltage => SoAParameter::Vgb,
            SoaParameterEvidence::GateBodyVoltagePositive => SoAParameter::VgbPositive,
            SoaParameterEvidence::GateBodyVoltageNegative => SoAParameter::VgbNegative,
            SoaParameterEvidence::BulkCurrent => SoAParameter::Ibulk,
            SoaParameterEvidence::BulkCurrentPositive => SoAParameter::IbulkPositive,
            SoaParameterEvidence::BulkCurrentNegative => SoAParameter::IbulkNegative,
            SoaParameterEvidence::BackgateSourceVoltage => SoAParameter::Ves,
            SoaParameterEvidence::BackgateSourceVoltagePositive => SoAParameter::VesPositive,
            SoaParameterEvidence::BackgateSourceVoltageNegative => SoAParameter::VesNegative,
            SoaParameterEvidence::BackgateDrainVoltage => SoAParameter::Ved,
            SoaParameterEvidence::BackgateDrainVoltagePositive => SoAParameter::VedPositive,
            SoaParameterEvidence::BackgateDrainVoltageNegative => SoAParameter::VedNegative,
            SoaParameterEvidence::GateBackgateVoltage => SoAParameter::Vge,
            SoaParameterEvidence::GateBackgateVoltagePositive => SoAParameter::VgePositive,
            SoaParameterEvidence::GateBackgateVoltageNegative => SoAParameter::VgeNegative,
            SoaParameterEvidence::BackgateCurrent => SoAParameter::Ibackgate,
            SoaParameterEvidence::BackgateCurrentPositive => SoAParameter::IbackgatePositive,
            SoaParameterEvidence::BackgateCurrentNegative => SoAParameter::IbackgateNegative,
            SoaParameterEvidence::BodyBackgateVoltage => SoAParameter::VbodyBackgate,
            SoaParameterEvidence::BodyBackgateVoltagePositive => {
                SoAParameter::VbodyBackgatePositive
            }
            SoaParameterEvidence::BodyBackgateVoltageNegative => {
                SoAParameter::VbodyBackgateNegative
            }

            SoaParameterEvidence::GateCurrent => SoAParameter::Ig,
            SoaParameterEvidence::GateCurrentPositive => SoAParameter::IgPositive,
            SoaParameterEvidence::GateCurrentNegative => SoAParameter::IgNegative,
            SoaParameterEvidence::SourceCurrent => SoAParameter::Is,
            SoaParameterEvidence::SourceCurrentPositive => SoAParameter::IsPositive,
            SoaParameterEvidence::SourceCurrentNegative => SoAParameter::IsNegative,
            SoaParameterEvidence::BaseCurrent => SoAParameter::Ib,
            SoaParameterEvidence::BaseCurrentPositive => SoAParameter::IbPositive,
            SoaParameterEvidence::BaseCurrentNegative => SoAParameter::IbNegative,
            SoaParameterEvidence::EmitterCurrent => SoAParameter::Ie,
            SoaParameterEvidence::EmitterCurrentPositive => SoAParameter::IePositive,
            SoaParameterEvidence::EmitterCurrentNegative => SoAParameter::IeNegative,

            SoaParameterEvidence::PowerDissipation => SoAParameter::Pdiss,
            SoaParameterEvidence::Temperature => SoAParameter::Temp,
            SoaParameterEvidence::GateSourceVoltagePositive => SoAParameter::VgsPositive,
            SoaParameterEvidence::GateSourceVoltageNegative => SoAParameter::VgsNegative,
            SoaParameterEvidence::DrainSourceVoltagePositive => SoAParameter::VdsPositive,
            SoaParameterEvidence::DrainSourceVoltageNegative => SoAParameter::VdsNegative,
            SoaParameterEvidence::GateDrainVoltagePositive => SoAParameter::VgdPositive,
            SoaParameterEvidence::GateDrainVoltageNegative => SoAParameter::VgdNegative,
            SoaParameterEvidence::BaseEmitterVoltagePositive => SoAParameter::VbePositive,
            SoaParameterEvidence::BaseEmitterVoltageNegative => SoAParameter::VbeNegative,
            SoaParameterEvidence::CollectorEmitterVoltagePositive => SoAParameter::VcePositive,
            SoaParameterEvidence::CollectorEmitterVoltageNegative => SoAParameter::VceNegative,
            SoaParameterEvidence::BaseCollectorVoltagePositive => SoAParameter::VbcPositive,
            SoaParameterEvidence::BaseCollectorVoltageNegative => SoAParameter::VbcNegative,
            SoaParameterEvidence::DrainCurrentPositive => SoAParameter::IdPositive,
            SoaParameterEvidence::DrainCurrentNegative => SoAParameter::IdNegative,
            SoaParameterEvidence::CollectorCurrentPositive => SoAParameter::IcPositive,
            SoaParameterEvidence::CollectorCurrentNegative => SoAParameter::IcNegative,
        }
    }
}
