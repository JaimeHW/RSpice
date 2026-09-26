//! Safe-operating-area evidence exchanged with the simulation worker.

use serde::{Deserialize, Serialize};

use rspice_results::safety::{
    SoAEvaluation, SoAParameter, SoARuleVerdict, SoAViolation, ViolationSeverity,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerSoAEvaluation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope: Option<rspice_results::safety::SoaCurrentEnvelopeEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<rspice_results::safety::SoaDurationEvidence>,
    #[serde(
        default,
        skip_serializing_if = "rspice_results::safety::SoaThresholds::is_default"
    )]
    pub thresholds: rspice_results::safety::SoaThresholds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derating: Option<rspice_results::safety::SoaPowerDeratingEvidence>,
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: WorkerSoARuleVerdict,
}

impl From<SoAEvaluation> for WorkerSoAEvaluation {
    fn from(value: SoAEvaluation) -> Self {
        Self {
            duration: value.duration,
            thresholds: value.thresholds,
            envelope: value.envelope,
            derating: value.derating,
            device_id: value.device_id,
            parameter: WorkerSoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            worst_actual_value: value.worst_actual_value,
            worst_time: value.worst_time,
            sample_count: value.sample_count,
            unit: value.unit,
            description: value.description,
            verdict: WorkerSoARuleVerdict::from(value.verdict),
        }
    }
}

impl From<WorkerSoAEvaluation> for SoAEvaluation {
    fn from(value: WorkerSoAEvaluation) -> Self {
        Self {
            duration: value.duration,
            thresholds: value.thresholds,
            envelope: value.envelope,
            derating: value.derating,
            device_id: value.device_id,
            parameter: SoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            worst_actual_value: value.worst_actual_value,
            worst_time: value.worst_time,
            sample_count: value.sample_count,
            unit: value.unit,
            description: value.description,
            verdict: SoARuleVerdict::from(value.verdict),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerSoARuleVerdict {
    Pass,
    Warning,
    Violation,
    Critical,
}

impl From<SoARuleVerdict> for WorkerSoARuleVerdict {
    fn from(value: SoARuleVerdict) -> Self {
        match value {
            SoARuleVerdict::Pass => Self::Pass,
            SoARuleVerdict::Warning => Self::Warning,
            SoARuleVerdict::Violation => Self::Violation,
            SoARuleVerdict::Critical => Self::Critical,
        }
    }
}

impl From<WorkerSoARuleVerdict> for SoARuleVerdict {
    fn from(value: WorkerSoARuleVerdict) -> Self {
        match value {
            WorkerSoARuleVerdict::Pass => Self::Pass,
            WorkerSoARuleVerdict::Warning => Self::Warning,
            WorkerSoARuleVerdict::Violation => Self::Violation,
            WorkerSoARuleVerdict::Critical => Self::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerSoAViolation {
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time: f64,
    pub severity: WorkerViolationSeverity,
}

impl From<SoAViolation> for WorkerSoAViolation {
    fn from(value: SoAViolation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: WorkerSoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            actual_value: value.actual_value,
            time: value.time,
            severity: WorkerViolationSeverity::from(value.severity),
        }
    }
}

impl From<WorkerSoAViolation> for SoAViolation {
    fn from(value: WorkerSoAViolation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: SoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            actual_value: value.actual_value,
            time: value.time,
            severity: ViolationSeverity::from(value.severity),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerSoAParameter {
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

impl From<SoAParameter> for WorkerSoAParameter {
    fn from(value: SoAParameter) -> Self {
        match value {
            SoAParameter::Vgs => Self::Vgs,
            SoAParameter::Vds => Self::Vds,
            SoAParameter::Vgd => Self::Vgd,
            SoAParameter::Vbe => Self::Vbe,
            SoAParameter::Vce => Self::Vce,
            SoAParameter::Vbc => Self::Vbc,
            SoAParameter::Id => Self::Id,
            SoAParameter::Ic => Self::Ic,
            SoAParameter::Vcsub => Self::Vcsub,
            SoAParameter::VcsubPositive => Self::VcsubPositive,
            SoAParameter::VcsubNegative => Self::VcsubNegative,
            SoAParameter::Vbsub => Self::Vbsub,
            SoAParameter::VbsubPositive => Self::VbsubPositive,
            SoAParameter::VbsubNegative => Self::VbsubNegative,
            SoAParameter::Vesub => Self::Vesub,
            SoAParameter::VesubPositive => Self::VesubPositive,
            SoAParameter::VesubNegative => Self::VesubNegative,
            SoAParameter::Isub => Self::Isub,
            SoAParameter::IsubPositive => Self::IsubPositive,
            SoAParameter::IsubNegative => Self::IsubNegative,
            SoAParameter::Vak => Self::Vak,
            SoAParameter::VakPositive => Self::VakPositive,
            SoAParameter::VakNegative => Self::VakNegative,
            SoAParameter::Ia => Self::Ia,
            SoAParameter::IaPositive => Self::IaPositive,
            SoAParameter::IaNegative => Self::IaNegative,

            SoAParameter::Pdiss => Self::Pdiss,
            SoAParameter::Temp => Self::Temp,
            SoAParameter::VgsPositive => Self::VgsPositive,
            SoAParameter::VgsNegative => Self::VgsNegative,
            SoAParameter::VdsPositive => Self::VdsPositive,
            SoAParameter::VdsNegative => Self::VdsNegative,
            SoAParameter::VgdPositive => Self::VgdPositive,
            SoAParameter::VgdNegative => Self::VgdNegative,
            SoAParameter::VbePositive => Self::VbePositive,
            SoAParameter::VbeNegative => Self::VbeNegative,
            SoAParameter::VcePositive => Self::VcePositive,
            SoAParameter::VceNegative => Self::VceNegative,
            SoAParameter::VbcPositive => Self::VbcPositive,
            SoAParameter::VbcNegative => Self::VbcNegative,
            SoAParameter::IdPositive => Self::IdPositive,
            SoAParameter::IdNegative => Self::IdNegative,
            SoAParameter::IcPositive => Self::IcPositive,
            SoAParameter::IcNegative => Self::IcNegative,
            SoAParameter::Ig => Self::Ig,
            SoAParameter::IgPositive => Self::IgPositive,
            SoAParameter::IgNegative => Self::IgNegative,
            SoAParameter::Is => Self::Is,
            SoAParameter::IsPositive => Self::IsPositive,
            SoAParameter::IsNegative => Self::IsNegative,
            SoAParameter::Ib => Self::Ib,
            SoAParameter::IbPositive => Self::IbPositive,
            SoAParameter::IbNegative => Self::IbNegative,
            SoAParameter::Ie => Self::Ie,
            SoAParameter::IePositive => Self::IePositive,
            SoAParameter::IeNegative => Self::IeNegative,
            SoAParameter::Vbs => Self::Vbs,
            SoAParameter::VbsPositive => Self::VbsPositive,
            SoAParameter::VbsNegative => Self::VbsNegative,
            SoAParameter::Vbd => Self::Vbd,
            SoAParameter::VbdPositive => Self::VbdPositive,
            SoAParameter::VbdNegative => Self::VbdNegative,
            SoAParameter::Vgb => Self::Vgb,
            SoAParameter::VgbPositive => Self::VgbPositive,
            SoAParameter::VgbNegative => Self::VgbNegative,
            SoAParameter::Ibulk => Self::Ibulk,
            SoAParameter::IbulkPositive => Self::IbulkPositive,
            SoAParameter::IbulkNegative => Self::IbulkNegative,
            SoAParameter::Ves => Self::Ves,
            SoAParameter::VesPositive => Self::VesPositive,
            SoAParameter::VesNegative => Self::VesNegative,
            SoAParameter::Ved => Self::Ved,
            SoAParameter::VedPositive => Self::VedPositive,
            SoAParameter::VedNegative => Self::VedNegative,
            SoAParameter::Vge => Self::Vge,
            SoAParameter::VgePositive => Self::VgePositive,
            SoAParameter::VgeNegative => Self::VgeNegative,
            SoAParameter::Ibackgate => Self::Ibackgate,
            SoAParameter::IbackgatePositive => Self::IbackgatePositive,
            SoAParameter::IbackgateNegative => Self::IbackgateNegative,
            SoAParameter::VbodyBackgate => Self::VbodyBackgate,
            SoAParameter::VbodyBackgatePositive => Self::VbodyBackgatePositive,
            SoAParameter::VbodyBackgateNegative => Self::VbodyBackgateNegative,
        }
    }
}

impl From<WorkerSoAParameter> for SoAParameter {
    fn from(value: WorkerSoAParameter) -> Self {
        match value {
            WorkerSoAParameter::Vgs => Self::Vgs,
            WorkerSoAParameter::Vds => Self::Vds,
            WorkerSoAParameter::Vgd => Self::Vgd,
            WorkerSoAParameter::Vbe => Self::Vbe,
            WorkerSoAParameter::Vce => Self::Vce,
            WorkerSoAParameter::Vbc => Self::Vbc,
            WorkerSoAParameter::Id => Self::Id,
            WorkerSoAParameter::Ic => Self::Ic,
            WorkerSoAParameter::Vcsub => Self::Vcsub,
            WorkerSoAParameter::VcsubPositive => Self::VcsubPositive,
            WorkerSoAParameter::VcsubNegative => Self::VcsubNegative,
            WorkerSoAParameter::Vbsub => Self::Vbsub,
            WorkerSoAParameter::VbsubPositive => Self::VbsubPositive,
            WorkerSoAParameter::VbsubNegative => Self::VbsubNegative,
            WorkerSoAParameter::Vesub => Self::Vesub,
            WorkerSoAParameter::VesubPositive => Self::VesubPositive,
            WorkerSoAParameter::VesubNegative => Self::VesubNegative,
            WorkerSoAParameter::Isub => Self::Isub,
            WorkerSoAParameter::IsubPositive => Self::IsubPositive,
            WorkerSoAParameter::IsubNegative => Self::IsubNegative,
            WorkerSoAParameter::Vak => Self::Vak,
            WorkerSoAParameter::VakPositive => Self::VakPositive,
            WorkerSoAParameter::VakNegative => Self::VakNegative,
            WorkerSoAParameter::Ia => Self::Ia,
            WorkerSoAParameter::IaPositive => Self::IaPositive,
            WorkerSoAParameter::IaNegative => Self::IaNegative,

            WorkerSoAParameter::Pdiss => Self::Pdiss,
            WorkerSoAParameter::Temp => Self::Temp,
            WorkerSoAParameter::VgsPositive => Self::VgsPositive,
            WorkerSoAParameter::VgsNegative => Self::VgsNegative,
            WorkerSoAParameter::VdsPositive => Self::VdsPositive,
            WorkerSoAParameter::VdsNegative => Self::VdsNegative,
            WorkerSoAParameter::VgdPositive => Self::VgdPositive,
            WorkerSoAParameter::VgdNegative => Self::VgdNegative,
            WorkerSoAParameter::VbePositive => Self::VbePositive,
            WorkerSoAParameter::VbeNegative => Self::VbeNegative,
            WorkerSoAParameter::VcePositive => Self::VcePositive,
            WorkerSoAParameter::VceNegative => Self::VceNegative,
            WorkerSoAParameter::VbcPositive => Self::VbcPositive,
            WorkerSoAParameter::VbcNegative => Self::VbcNegative,
            WorkerSoAParameter::IdPositive => Self::IdPositive,
            WorkerSoAParameter::IdNegative => Self::IdNegative,
            WorkerSoAParameter::IcPositive => Self::IcPositive,
            WorkerSoAParameter::IcNegative => Self::IcNegative,
            WorkerSoAParameter::Ig => Self::Ig,
            WorkerSoAParameter::IgPositive => Self::IgPositive,
            WorkerSoAParameter::IgNegative => Self::IgNegative,
            WorkerSoAParameter::Is => Self::Is,
            WorkerSoAParameter::IsPositive => Self::IsPositive,
            WorkerSoAParameter::IsNegative => Self::IsNegative,
            WorkerSoAParameter::Ib => Self::Ib,
            WorkerSoAParameter::IbPositive => Self::IbPositive,
            WorkerSoAParameter::IbNegative => Self::IbNegative,
            WorkerSoAParameter::Ie => Self::Ie,
            WorkerSoAParameter::IePositive => Self::IePositive,
            WorkerSoAParameter::IeNegative => Self::IeNegative,
            WorkerSoAParameter::Vbs => Self::Vbs,
            WorkerSoAParameter::VbsPositive => Self::VbsPositive,
            WorkerSoAParameter::VbsNegative => Self::VbsNegative,
            WorkerSoAParameter::Vbd => Self::Vbd,
            WorkerSoAParameter::VbdPositive => Self::VbdPositive,
            WorkerSoAParameter::VbdNegative => Self::VbdNegative,
            WorkerSoAParameter::Vgb => Self::Vgb,
            WorkerSoAParameter::VgbPositive => Self::VgbPositive,
            WorkerSoAParameter::VgbNegative => Self::VgbNegative,
            WorkerSoAParameter::Ibulk => Self::Ibulk,
            WorkerSoAParameter::IbulkPositive => Self::IbulkPositive,
            WorkerSoAParameter::IbulkNegative => Self::IbulkNegative,
            WorkerSoAParameter::Ves => Self::Ves,
            WorkerSoAParameter::VesPositive => Self::VesPositive,
            WorkerSoAParameter::VesNegative => Self::VesNegative,
            WorkerSoAParameter::Ved => Self::Ved,
            WorkerSoAParameter::VedPositive => Self::VedPositive,
            WorkerSoAParameter::VedNegative => Self::VedNegative,
            WorkerSoAParameter::Vge => Self::Vge,
            WorkerSoAParameter::VgePositive => Self::VgePositive,
            WorkerSoAParameter::VgeNegative => Self::VgeNegative,
            WorkerSoAParameter::Ibackgate => Self::Ibackgate,
            WorkerSoAParameter::IbackgatePositive => Self::IbackgatePositive,
            WorkerSoAParameter::IbackgateNegative => Self::IbackgateNegative,
            WorkerSoAParameter::VbodyBackgate => Self::VbodyBackgate,
            WorkerSoAParameter::VbodyBackgatePositive => Self::VbodyBackgatePositive,
            WorkerSoAParameter::VbodyBackgateNegative => Self::VbodyBackgateNegative,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerViolationSeverity {
    Warning,
    Violation,
    Critical,
}

impl From<ViolationSeverity> for WorkerViolationSeverity {
    fn from(value: ViolationSeverity) -> Self {
        match value {
            ViolationSeverity::Warning => Self::Warning,
            ViolationSeverity::Violation => Self::Violation,
            ViolationSeverity::Critical => Self::Critical,
        }
    }
}

impl From<WorkerViolationSeverity> for ViolationSeverity {
    fn from(value: WorkerViolationSeverity) -> Self {
        match value {
            WorkerViolationSeverity::Warning => Self::Warning,
            WorkerViolationSeverity::Violation => Self::Violation,
            WorkerViolationSeverity::Critical => Self::Critical,
        }
    }
}
