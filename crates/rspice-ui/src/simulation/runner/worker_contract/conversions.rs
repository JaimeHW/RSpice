//! Converting between the worker's mirror types and the core result model.
//!
//! Every conversion here is total in one direction and checked in the other:
//! going out to the worker cannot fail, but coming back must reject a payload
//! whose enum tags or lengths do not correspond to anything the core model
//! admits.  That is why these live together — the pair for each type has to be
//! read as one round trip.

use super::*;

impl From<TransferFunctionQuantity> for WorkerTransferFunctionQuantity {
    fn from(value: TransferFunctionQuantity) -> Self {
        match value {
            TransferFunctionQuantity::Voltage => Self::Voltage,
            TransferFunctionQuantity::Current => Self::Current,
        }
    }
}

impl From<WorkerTransferFunctionQuantity> for TransferFunctionQuantity {
    fn from(value: WorkerTransferFunctionQuantity) -> Self {
        match value {
            WorkerTransferFunctionQuantity::Voltage => Self::Voltage,
            WorkerTransferFunctionQuantity::Current => Self::Current,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerTransferFunctionScalar {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl From<TransferFunctionScalar> for WorkerTransferFunctionScalar {
    fn from(value: TransferFunctionScalar) -> Self {
        match value {
            TransferFunctionScalar::Finite(value) => Self::Finite(value),
            TransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            TransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

impl From<WorkerTransferFunctionScalar> for TransferFunctionScalar {
    fn from(value: WorkerTransferFunctionScalar) -> Self {
        match value {
            WorkerTransferFunctionScalar::Finite(value) => Self::Finite(value),
            WorkerTransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            WorkerTransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseSummary {
    #[serde(default)]
    pub input_quantity: Option<rspice_core::analysis::noise::NoiseInputQuantity>,
    #[serde(default)]
    pub conversion: Option<crate::state::PeriodicNoiseConversionEvidence>,
    #[serde(default)]
    pub noise_figure: Option<std::sync::Arc<crate::state::NoiseFigureEvidence>>,
    pub rows: Vec<WorkerNoiseContributorRow>,
    #[serde(default)]
    pub total_rms: Option<f64>,
    #[serde(default)]
    pub input_rms: Option<f64>,
    pub band: (f64, f64),
}

#[cfg(test)]
impl WorkerNoiseSummary {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            self.rows
                .iter()
                .map(WorkerNoiseContributorRow::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
            f64_payload_bytes(3),
            self.conversion
                .as_ref()
                .map_or(0, |_| f64_payload_bytes(1).saturating_add(12)),
            self.noise_figure.as_ref().map_or(0, |figure| {
                f64_payload_bytes(
                    3usize
                        .saturating_add(figure.frequencies.len())
                        .saturating_add(figure.decibels.len()),
                )
            }),
        ])
    }
}

impl From<NoiseSummary> for WorkerNoiseSummary {
    fn from(value: NoiseSummary) -> Self {
        Self {
            input_quantity: value.input_quantity,
            conversion: value.conversion,
            noise_figure: value.noise_figure,
            rows: value
                .rows
                .into_iter()
                .map(WorkerNoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

impl From<WorkerNoiseSummary> for NoiseSummary {
    fn from(value: WorkerNoiseSummary) -> Self {
        Self {
            input_quantity: value.input_quantity,
            conversion: value.conversion,
            noise_figure: value.noise_figure,
            rows: value
                .rows
                .into_iter()
                .map(NoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

#[cfg(test)]
impl WorkerNoiseContributorRow {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(2)
    }
}

impl From<NoiseContributorRow> for WorkerNoiseContributorRow {
    fn from(value: NoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism.to_string(),
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

impl From<WorkerNoiseContributorRow> for NoiseContributorRow {
    fn from(value: WorkerNoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism,
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMonteCarloVariable {
    /// Confidence in the mean, with estimator and successful-trial population.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_confidence: Option<crate::state::MonteCarloMeanConfidence>,
    pub name: String,
    pub samples: Vec<f64>,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<f64>,
}

#[cfg(test)]
impl WorkerMonteCarloVariable {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(4usize.saturating_add(self.samples.len())),
            if self.mean_confidence.is_some() {
                48
            } else {
                0
            },
            usize_payload_bytes(self.histogram.len()),
            f64_payload_bytes(self.bin_edges.len()),
        ])
    }
}

impl From<MonteCarloVariableResult> for WorkerMonteCarloVariable {
    fn from(value: MonteCarloVariableResult) -> Self {
        Self {
            mean_confidence: value.mean_confidence,
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

impl From<WorkerMonteCarloVariable> for MonteCarloVariableResult {
    fn from(value: WorkerMonteCarloVariable) -> Self {
        Self {
            mean_confidence: value.mean_confidence,
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSoAEvaluation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope: Option<crate::services::safety::SoaCurrentEnvelopeEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<crate::services::safety::SoaDurationEvidence>,
    #[serde(
        default,
        skip_serializing_if = "crate::services::safety::SoaThresholds::is_default"
    )]
    pub thresholds: crate::services::safety::SoaThresholds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derating: Option<crate::services::safety::SoaPowerDeratingEvidence>,
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

#[cfg(test)]
impl WorkerSoAEvaluation {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(if self.derating.is_some() { 6 } else { 3 })
            .saturating_add(f64_payload_bytes(self.envelope.as_ref().map_or(0, |e| {
                1 + usize::from(e.curve.pulse_width_s.is_some())
                    + e.curve.voltages_v.len()
                    + e.curve.dc_currents_a.as_ref().map_or(0, Vec::len)
                    + e.curve
                        .pulses
                        .iter()
                        .map(|p| 1 + p.currents_a.len())
                        .sum::<usize>()
            })))
            .saturating_add(f64_payload_bytes(if self.duration.is_some() {
                6
            } else {
                0
            }))
            .saturating_add(f64_payload_bytes(if self.thresholds.is_default() {
                0
            } else {
                usize::from(self.thresholds.warning_fraction.is_some())
                    + usize::from(self.thresholds.critical_fraction.is_some())
            }))
            .saturating_add(std::mem::size_of::<u64>())
    }
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
pub(crate) enum WorkerSoARuleVerdict {
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
pub(crate) struct WorkerSoAViolation {
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time: f64,
    pub severity: WorkerViolationSeverity,
}

#[cfg(test)]
impl WorkerSoAViolation {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3)
    }
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
pub(crate) enum WorkerSoAParameter {
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
pub(crate) enum WorkerViolationSeverity {
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

/// One committed digital event, as it crosses the worker edge.
///
/// Points are transported whole rather than as parallel time/value arrays:
/// an event history is short enough that the buffer split buys nothing, and
/// paired arrays can arrive with different lengths.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDigitalEventPoint {
    pub time_s: f64,
    pub value_code: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRealEventPoint {
    pub time_s: f64,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDigitalEventTrace {
    pub node_name: String,
    pub points: Vec<WorkerDigitalEventPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRealEventTrace {
    pub node_name: String,
    pub points: Vec<WorkerRealEventPoint>,
}

/// One digital bus declared over the digital traces beside it, on the wire.
///
/// The declaration crosses; the word does not. Reassembling a bus is
/// `rspice_core::execution::bus_events` reading the member histories, so a
/// worker and its host can never disagree about what a bus held.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDigitalBus {
    pub name: String,
    pub msb: i64,
    pub lsb: i64,
    pub members: Vec<String>,
    pub source: crate::state::DigitalBusSourceEvidence,
}

/// Every event node a transient run committed, on the wire.
///
/// Missing legacy fields retain unavailable history. The response protocol
/// version prevents a stale worker from silently omitting new observations.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerEventHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_impulses: Option<crate::state::CurrentImpulseHistoryEvidence>,
    #[serde(default)]
    pub digital: Vec<WorkerDigitalEventTrace>,
    #[serde(default)]
    pub real: Vec<WorkerRealEventTrace>,
    /// Buses declared over `digital`. Defaulted so a protocol-14 worker's
    /// response, which could not have declared one, reads as declaring none.
    #[serde(default)]
    pub buses: Vec<WorkerDigitalBus>,
}

impl From<TransientEventHistory> for WorkerEventHistory {
    fn from(value: TransientEventHistory) -> Self {
        Self {
            current_impulses: value.current_impulses,
            digital: value
                .digital
                .into_iter()
                .map(|trace| WorkerDigitalEventTrace {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| WorkerDigitalEventPoint {
                            time_s: point.time_s,
                            value_code: point.value_code,
                        })
                        .collect(),
                })
                .collect(),
            real: value
                .real
                .into_iter()
                .map(|trace| WorkerRealEventTrace {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| WorkerRealEventPoint {
                            time_s: point.time_s,
                            value: point.value,
                        })
                        .collect(),
                })
                .collect(),
            buses: value
                .digital_buses
                .into_iter()
                .map(|bus| WorkerDigitalBus {
                    name: bus.name,
                    msb: bus.msb,
                    lsb: bus.lsb,
                    members: bus.members,
                    source: bus.source,
                })
                .collect(),
        }
    }
}

impl From<WorkerEventHistory> for TransientEventHistory {
    fn from(value: WorkerEventHistory) -> Self {
        Self {
            current_impulses: value.current_impulses,
            digital: value
                .digital
                .into_iter()
                .map(|trace| EventNodeHistory {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| DigitalEventPoint {
                            time_s: point.time_s,
                            value_code: point.value_code,
                        })
                        .collect(),
                })
                .collect(),
            real: value
                .real
                .into_iter()
                .map(|trace| EventNodeHistory {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| RealEventPoint {
                            time_s: point.time_s,
                            value: point.value,
                        })
                        .collect(),
                })
                .collect(),
            digital_buses: value
                .buses
                .into_iter()
                .map(|bus| crate::state::DigitalBusEvidence {
                    name: bus.name,
                    msb: bus.msb,
                    lsb: bus.lsb,
                    members: bus.members,
                    source: bus.source,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerWaveform {
    pub name: String,
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    pub y_unit: String,
    pub is_complex: bool,
    pub y_imag: Option<Vec<f64>>,
}

#[cfg(test)]
impl WorkerWaveform {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(self.x_values.len()),
            f64_payload_bytes(self.y_values.len()),
            self.y_imag
                .as_ref()
                .map_or(0, |values| f64_payload_bytes(values.len())),
        ])
    }
}

impl From<WaveformData> for WorkerWaveform {
    fn from(value: WaveformData) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

impl From<WorkerWaveform> for WaveformData {
    fn from(value: WorkerWaveform) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMeasurement {
    pub name: String,
    pub value: Option<f64>,
    #[serde(default)]
    pub raw_value: Option<f64>,
    pub error: Option<String>,
    pub passed: bool,
    pub expected: Option<f64>,
    pub tolerance: Option<f64>,
    #[serde(default)]
    pub failure_limit: Option<f64>,
    #[serde(default)]
    pub failure_limit_exceeded: bool,
    pub event_axis: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<rspice_core::analysis::MeasurementUnits>,
}

#[cfg(test)]
impl WorkerMeasurement {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(
            usize::from(self.value.is_some())
                + usize::from(self.raw_value.is_some())
                + usize::from(self.expected.is_some())
                + usize::from(self.tolerance.is_some())
                + usize::from(self.failure_limit.is_some())
                + usize::from(self.event_axis.is_some()),
        )
    }
}

impl WorkerMeasurement {
    pub(super) fn validate_current_evidence(&self, prefix: &str) -> Result<(), String> {
        if let Some(units) = &self.units {
            units
                .validate()
                .map_err(|error| format!("{prefix}.units: {error}"))?;
        }
        if self.name.trim().is_empty() || self.name.chars().any(char::is_control) {
            return Err(format!("{prefix} has an invalid measurement name"));
        }
        if [
            self.value,
            self.raw_value,
            self.expected,
            self.tolerance,
            self.failure_limit,
            self.event_axis,
        ]
        .into_iter()
        .flatten()
        .any(|value| !value.is_finite())
            || self.tolerance.is_some_and(|tolerance| tolerance < 0.0)
        {
            return Err(format!(
                "{prefix} contains non-finite measurement evidence or a negative tolerance"
            ));
        }
        if self.value.is_some() != self.raw_value.is_some() {
            return Err(format!(
                "{prefix}.raw_value must be present exactly when value is present"
            ));
        }
        let expected_exceeded = match (self.raw_value, self.failure_limit) {
            (Some(raw_value), Some(limit)) => raw_value.abs() >= limit,
            _ => false,
        };
        if self.failure_limit_exceeded != expected_exceeded {
            return Err(format!(
                "{prefix}.failure_limit_exceeded does not match abs(raw_value) >= failure_limit"
            ));
        }
        if self.failure_limit_exceeded && self.passed {
            return Err(format!(
                "{prefix} cannot pass after its FAILVALUE limit was reached"
            ));
        }
        if self.passed && (self.value.is_none() || self.error.is_some()) {
            return Err(format!(
                "{prefix} has contradictory passing measurement evidence"
            ));
        }
        Ok(())
    }
}

impl From<rspice_core::MeasureResult> for WorkerMeasurement {
    fn from(value: rspice_core::MeasureResult) -> Self {
        Self {
            name: value.name,
            value: value.value,
            raw_value: value.raw_value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            failure_limit: value.failure_limit,
            failure_limit_exceeded: value.failure_limit_exceeded,
            event_axis: value.event_axis,
            units: value.units,
        }
    }
}

impl From<WorkerMeasurement> for rspice_core::MeasureResult {
    fn from(value: WorkerMeasurement) -> Self {
        Self {
            name: value.name,
            value: value.value,
            raw_value: value.raw_value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            failure_limit: value.failure_limit,
            failure_limit_exceeded: value.failure_limit_exceeded,
            event_axis: value.event_axis,
            units: value.units,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpReport {
    pub entries: Vec<WorkerDeviceOpEntry>,
}

#[cfg(test)]
impl WorkerDeviceOpReport {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        self.entries
            .iter()
            .map(WorkerDeviceOpEntry::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpReport> for WorkerDeviceOpReport {
    fn from(value: rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(WorkerDeviceOpEntry::from)
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpReport> for rspice_core::circuit::DeviceOpReport {
    fn from(value: WorkerDeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(rspice_core::circuit::DeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    pub region: Option<String>,
    pub params: Vec<WorkerNamedValue>,
}

#[cfg(test)]
impl WorkerDeviceOpEntry {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        self.params
            .iter()
            .map(WorkerNamedValue::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpEntry> for WorkerDeviceOpEntry {
    fn from(value: rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: value.device_kind.to_string(),
            region: value.region.map(str::to_string),
            params: value
                .params
                .into_iter()
                .map(|(name, value)| WorkerNamedValue {
                    name: name.to_string(),
                    value,
                })
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpEntry> for rspice_core::circuit::DeviceOpEntry {
    fn from(value: WorkerDeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: intern_static_label(value.device_kind),
            region: value.region.map(intern_static_label),
            params: value
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNamedValue {
    pub name: String,
    pub value: f64,
}

#[cfg(test)]
impl WorkerNamedValue {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(1)
    }
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn worker_response_from_request(request: WorkerRequest) -> WorkerResponse {
    worker_response_from_request_with_progress(request, None)
}

pub(super) fn worker_response_from_request_with_progress(
    request: WorkerRequest,
    progress_observer: Option<super::super::ProgressObserver>,
) -> WorkerResponse {
    let id = request.id;
    let (request, input) = request.into_runner_parts();
    let progress = Arc::new(Mutex::new(SimulationProgress::default()));
    let abort_flag = Arc::new(AtomicBool::new(false));

    WorkerResponse::from_result_for_transfer(
        id,
        super::super::run_simulation_thread_with_progress_observer(
            request,
            input,
            progress,
            abort_flag,
            super::super::RunStreams {
                progress_observer,
                ..Default::default()
            },
        ),
    )
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
thread_local! {
    static ACTIVE_WORKER_PROGRESS_ID: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
    static PENDING_WASM_JIT_REQUEST: std::cell::RefCell<Option<(u32, WorkerRequest)>> = const { std::cell::RefCell::new(None) };
    static NEXT_WASM_JIT_DISPATCH_TOKEN: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(super) fn emit_worker_progress_snapshot(progress: &SimulationProgress) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };

    let snapshot = WorkerProgressSnapshot::from_progress(id, progress);
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("progress"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(snapshot) = serde_wasm_bindgen::to_value(&snapshot) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("progress"), &snapshot);
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(super) fn emit_worker_transient_sample(sample: &super::super::TransientSampleDelta) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("transientSample"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(sample) = serde_wasm_bindgen::to_value(sample) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("sample"), &sample);
    } else {
        return;
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

/// Deliver the portable checkpoint as transferred bytes before terminal success.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn emit_worker_monte_carlo_checkpoint(bytes: &[u8]) -> Result<(), SimulationError> {
    use wasm_bindgen::{JsCast as _, JsValue};
    let fail = |message: String| {
        SimulationError::InvalidConfig(format!(
            "Could not deliver Monte Carlo checkpoint: {message}"
        ))
    };
    super::super::monte_carlo_checkpoint::validate_checkpoint_bytes_size(bytes.len())
        .map_err(&fail)?;
    let id = ACTIVE_WORKER_PROGRESS_ID
        .with(|active| active.get())
        .ok_or_else(|| fail("no active worker request".into()))?;
    let message = js_sys::Object::new();
    let view = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    view.copy_from(bytes);
    for (key, value) in [
        ("type", JsValue::from_str("monteCarloCheckpoint")),
        ("id", JsValue::from_f64(id as f64)),
        ("checkpoint", JsValue::from(view.clone())),
    ] {
        if !js_sys::Reflect::set(&message, &JsValue::from_str(key), &value)
            .map_err(|error| fail(format!("{error:?}")))?
        {
            return Err(fail(format!("could not set {key}")));
        }
    }
    let transfer = js_sys::Array::new();
    transfer.push(&view.buffer());
    let global = js_sys::global();
    let post = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
        .map_err(|error| fail(format!("{error:?}")))?;
    post.call2(&global, &message, &transfer)
        .map_err(|error| fail(format!("{error:?}")))?;
    Ok(())
}

/// Post one line the engine logged back to the UI instance.
///
/// The worker is its own wasm instance, so there is no queue on this side of
/// the boundary for a run to write: the same shape as
/// [`emit_worker_transient_sample`], one message per line, keyed by the active
/// request so a superseded run's lines cannot land in a newer run's Console.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(super) fn emit_worker_engine_log(line: &crate::diagnostics::engine_log::EngineLogLine) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("engineLog"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(line) = serde_wasm_bindgen::to_value(line) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("line"), &line);
    } else {
        return;
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

/// The worker image's whole logger: it feeds the run's sink and nothing else.
///
/// This image has no terminal and paints nothing, so there is no second reader
/// to filter for — where the desktop's `StudioLogger` wraps `env_logger`, this
/// is the sink half alone.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
struct WorkerEngineLogger;

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
impl log::Log for WorkerEngineLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        crate::diagnostics::engine_log::admits(metadata)
    }

    fn log(&self, record: &log::Record<'_>) {
        crate::diagnostics::engine_log::offer(record);
    }

    fn flush(&self) {}
}

/// Install [`WorkerEngineLogger`] on this worker, once.
///
/// Called at the head of a request rather than from a bootstrap export: the
/// worker script's startup sequence is a published contract with the page, and
/// a logger the first run installs needs no place in it. `set_boxed_logger`
/// succeeds once per instance, and the worker instance outlives many requests,
/// so the guard is a `OnceLock` rather than a per-run install.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn install_worker_engine_logger() {
    static INSTALLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    INSTALLED.get_or_init(|| {
        if log::set_boxed_logger(Box::new(WorkerEngineLogger)).is_ok() {
            // No stderr half at all on this image, so the level the process
            // admits is exactly what the run's sink asks for.
            crate::diagnostics::engine_log::note_stderr_level(log::LevelFilter::Off);
        }
    });
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = worker_request_from_value(value)?;
    run_decoded_worker_request(request)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn run_decoded_worker_request(
    request: WorkerRequest,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let id = request.id;
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(Some(id)));
    let stream_transient_samples = request.stream_transient_samples;
    let (request, input) = request.into_runner_parts();
    install_worker_engine_logger();
    let verbose = super::super::request_asked_for_verbose(&request);
    let progress = Arc::new(Mutex::new(SimulationProgress::default()));
    let abort_flag = Arc::new(AtomicBool::new(false));
    let response = WorkerResponse::from_result_for_transfer(
        id,
        super::super::run_simulation_thread_with_progress_observer(
            request,
            input,
            progress,
            abort_flag,
            super::super::RunStreams {
                progress_observer: Some(emit_worker_progress_snapshot),
                checkpoint_observer: Some(Arc::new(emit_worker_monte_carlo_checkpoint)),
                transient_sample_observer: stream_transient_samples
                    .then_some(emit_worker_transient_sample),
                engine_log: Some(crate::diagnostics::engine_log::RunLogSink::observed(
                    emit_worker_engine_log,
                    verbose,
                )),
                ..Default::default()
            },
        ),
    );
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(None));
    worker_response_transport_value(response)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct WasmJitRequestPreparation {
    dispatch_token: u32,
    artifacts: Vec<crate::simulation::veriloga::WasmJitWorkerArtifact>,
    errors: Vec<String>,
}

/// Compile every sealed Verilog-A runtime required by a simulation request
/// before the synchronous solver begins. The JavaScript worker installs these
/// modules into its persistent, capability-limited instance cache.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn prepare_wasm_jit_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    if PENDING_WASM_JIT_REQUEST.with(|pending| pending.borrow().is_some()) {
        return Err(wasm_bindgen::JsValue::from_str(
            "a prepared browser simulation request is already pending",
        ));
    }
    let request = worker_request_from_value(value)?;
    let dispatch_token = NEXT_WASM_JIT_DISPATCH_TOKEN.with(|next| {
        let token = next.get().wrapping_add(1).max(1);
        next.set(token);
        token
    });
    let mut preparation = WasmJitRequestPreparation {
        dispatch_token,
        artifacts: Vec::with_capacity(request.project_veriloga_runtimes.device_runtimes().len()),
        errors: Vec::new(),
    };
    for runtime in request.project_veriloga_runtimes.device_runtimes() {
        match runtime.compile_wasm_jit_artifact() {
            Ok(artifact) => preparation.artifacts.push(artifact),
            Err(error) => preparation.errors.push(format!(
                "Verilog-A runtime '{}' could not qualify for the browser JIT: {error}",
                runtime.netlist_alias()
            )),
        }
    }
    let value = serde_wasm_bindgen::to_value(&preparation)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    PENDING_WASM_JIT_REQUEST.with(|pending| {
        *pending.borrow_mut() = Some((dispatch_token, request));
    });
    Ok(value)
}

/// Consume exactly the request decoded by `prepare_wasm_jit_request_value`.
/// This avoids a second copy of every transferred numerical dependency.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn run_prepared_wasm_jit_request_value(
    dispatch_token: u32,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = PENDING_WASM_JIT_REQUEST.with(|pending| {
        let mut pending = pending.borrow_mut();
        let Some((expected_token, _)) = pending.as_ref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "no prepared browser simulation request is pending",
            ));
        };
        if dispatch_token == 0 || dispatch_token != *expected_token {
            return Err(wasm_bindgen::JsValue::from_str(
                "stale browser simulation dispatch token",
            ));
        }
        Ok(pending
            .take()
            .expect("validated prepared request must remain present")
            .1)
    })?;
    run_decoded_worker_request(request)
}

/// Discard a prepared request only when the caller presents its exact token.
///
/// This closes the one failure path between request decoding and synchronous
/// dispatch without allowing a stale JavaScript caller to cancel newer work.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn cancel_prepared_wasm_jit_request_value(
    dispatch_token: u32,
) -> Result<(), wasm_bindgen::JsValue> {
    PENDING_WASM_JIT_REQUEST.with(|pending| {
        let mut pending = pending.borrow_mut();
        let Some((expected_token, _)) = pending.as_ref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "no prepared browser simulation request is pending",
            ));
        };
        if dispatch_token == 0 || dispatch_token != *expected_token {
            return Err(wasm_bindgen::JsValue::from_str(
                "stale browser simulation dispatch token",
            ));
        }
        pending.take();
        Ok(())
    })
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_request_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerRequest, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .map_err(worker_request_js_error)?
        .as_f64()
        .and_then(|value| {
            (value.fract() == 0.0 && (0.0..=f64::from(u8::MAX)).contains(&value))
                .then_some(value as u8)
        })
        .ok_or_else(|| {
            JsValue::from_str("worker request transport protocolVersion must be an unsigned byte")
        })?;

    let request = js_sys::Reflect::get(&value, &JsValue::from_str("request"))
        .map_err(worker_request_js_error)?;
    let request = serde_wasm_bindgen::from_value::<WorkerRequestTransportMetadata>(request)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_request_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| JsValue::from_str("worker request transport buffers must be an array"))?;
    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(JsValue::from_str(&format!(
            "worker request contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = checked_worker_request_numeric_total(
            numeric_values,
            index as usize,
            view.length() as usize,
        )
        .map_err(|error| JsValue::from_str(&error))?;
    }

    let byte_buffers = js_sys::Reflect::get(&value, &JsValue::from_str("byteBuffers"))
        .map_err(worker_request_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| JsValue::from_str("worker request byteBuffers must be an array"))?;
    if byte_buffers.length() > 1 {
        return Err(JsValue::from_str(
            "worker request carries too many checkpoint buffers",
        ));
    }
    let mut byte_lengths = Vec::new();
    for index in 0..byte_buffers.length() {
        let view = byte_buffers
            .get(index)
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from_str("worker request checkpoint must be a Uint8Array"))?;
        byte_lengths.push(view.length() as usize);
    }
    validate_worker_request_checkpoint_lengths(numeric_values, &byte_lengths)
        .map_err(|error| JsValue::from_str(&error))?;
    let mut decoded_bytes = Vec::new();
    for index in 0..byte_buffers.length() {
        let view = byte_buffers
            .get(index)
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from_str("worker request checkpoint must be a Uint8Array"))?;
        let mut bytes = vec![0; view.length() as usize];
        view.copy_to(&mut bytes);
        decoded_bytes.push(bytes);
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerRequestTransport {
        protocol,
        request,
        buffers: decoded_buffers,
        byte_buffers: decoded_bytes,
    }
    .into_request()
    .map_err(|error| JsValue::from_str(&error))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_request_js_error(error: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str(&worker_js_error(error).to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_transport_value(
    response: WorkerResponse,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsValue;

    let transport = WorkerResponseTransport::from_response(response)
        .map_err(|error| JsValue::from_str(&error))?;
    let message = js_sys::Object::new();
    js_sys::Reflect::set(
        &message,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(transport.protocol)),
    )?;
    // Result identities and Monte Carlo seeds use the complete u64 range.
    // Structured clone carries BigInt exactly; the default JS-number encoder
    // rejects otherwise valid results above 2^53 - 1.
    let response = transport
        .response
        .serialize(
            &serde_wasm_bindgen::Serializer::new().serialize_large_number_types_as_bigints(true),
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    js_sys::Reflect::set(&message, &JsValue::from_str("response"), &response)?;

    let buffers = js_sys::Array::new();
    for values in transport.buffers {
        let view = js_sys::Float64Array::new_with_length(values.len() as u32);
        view.copy_from(&values);
        buffers.push(&view);
    }
    js_sys::Reflect::set(&message, &JsValue::from_str("buffers"), &buffers)?;

    Ok(JsValue::from(message))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerResponse, SimulationError> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value as u8);

    if protocol != Some(WORKER_RESPONSE_TRANSPORT_PROTOCOL) {
        let response = serde_wasm_bindgen::from_value::<WorkerResponse>(value)
            .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
        super::transport::validate_worker_response_before_transport(&response)
            .map_err(SimulationError::InvalidConfig)?;
        return Ok(response);
    }

    let response =
        js_sys::Reflect::get(&value, &JsValue::from_str("response")).map_err(worker_js_error)?;
    let response = serde_wasm_bindgen::from_value::<WorkerResponseTransportMetadata>(response)
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| {
            SimulationError::InvalidConfig(
                "worker response transport buffers must be an array".to_string(),
            )
        })?;

    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(SimulationError::InvalidConfig(format!(
            "worker response contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = numeric_values
            .checked_add(view.length() as usize)
            .ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "worker response numeric size overflows this platform".to_owned(),
                )
            })?;
        if numeric_values > MAX_WORKER_F64_VALUES {
            return Err(SimulationError::InvalidConfig(format!(
                "worker response contains more than {MAX_WORKER_F64_VALUES} numerical values"
            )));
        }
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response,
        buffers: decoded_buffers,
    }
    .into_response()
    .map_err(SimulationError::InvalidConfig)
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_js_error(error: wasm_bindgen::JsValue) -> SimulationError {
    use wasm_bindgen::JsValue;

    let message = error
        .as_string()
        .or_else(|| {
            js_sys::Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown JavaScript error".to_string());
    SimulationError::InvalidConfig(message)
}

#[cfg(test)]
pub(super) fn sum_payload_bytes(bytes: impl IntoIterator<Item = usize>) -> usize {
    bytes
        .into_iter()
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn f64_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
pub(super) fn pss_operating_point_payload_bytes(
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> usize {
    let analysis = operating_point.analysis();
    let values = analysis
        .result
        .time
        .len()
        .saturating_add(
            analysis
                .result
                .waveforms
                .iter()
                .chain(&analysis.result.branch_waveforms)
                .map(|waveform| waveform.values.len())
                .sum::<usize>(),
        )
        .saturating_add(analysis.monodromy.iter().map(Vec::len).sum::<usize>())
        .saturating_add(analysis.result.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(analysis.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(operating_point.shooting_state().len());
    f64_payload_bytes(values)
}

#[cfg(test)]
pub(super) fn usize_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<usize>())
}

#[cfg(test)]
pub(super) fn complex_pair_payload_bytes(len: usize) -> usize {
    len.saturating_mul(2)
        .saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
pub(super) fn event_history_payload_bytes(events: &WorkerEventHistory) -> usize {
    let digital = events
        .digital
        .iter()
        .map(|trace| f64_payload_bytes(trace.points.len()).saturating_add(trace.points.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes));
    let real = events
        .real
        .iter()
        .map(|trace| f64_payload_bytes(trace.points.len().saturating_mul(2)))
        .fold(digital, |total, bytes| total.saturating_add(bytes));
    // A bus contributes its two declared indices and nothing else: the
    // members are names, which this budget does not count for a trace either,
    // and there is no value in a declaration to count.
    let impulses = events.current_impulses.as_ref().map_or(0, |history| {
        history
            .traces
            .iter()
            .fold(2 * size_of::<f64>() + 1, |total, trace| {
                total
                    .saturating_add(1)
                    .saturating_add(f64_payload_bytes(trace.points.len().saturating_mul(2)))
            })
    });
    real.saturating_add(events.buses.len().saturating_mul(2 * size_of::<i64>()))
        .saturating_add(impulses)
}

#[cfg(test)]
pub(super) fn waveforms_payload_bytes(waveforms: &[WorkerWaveform]) -> usize {
    waveforms
        .iter()
        .map(WorkerWaveform::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn measurements_payload_bytes(measurements: &[WorkerMeasurement]) -> usize {
    measurements
        .iter()
        .map(WorkerMeasurement::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn vec_map_payload_bytes(values_by_name: &HashMap<String, Vec<f64>>) -> usize {
    values_by_name
        .values()
        .map(|values| f64_payload_bytes(values.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn soa_violations_payload_bytes(violations: &[WorkerSoAViolation]) -> usize {
    violations
        .iter()
        .map(WorkerSoAViolation::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn soa_evaluations_payload_bytes(evaluations: &[WorkerSoAEvaluation]) -> usize {
    evaluations
        .iter()
        .map(WorkerSoAEvaluation::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

pub(super) fn worker_waveforms(waveforms: HashMap<String, WaveformData>) -> Vec<WorkerWaveform> {
    let mut waveforms: Vec<_> = waveforms.into_values().map(WorkerWaveform::from).collect();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));
    waveforms
}

pub(super) fn pss_display_projection(
    operating_point: &rspice_core::engine::PssOperatingPoint,
    reporting_times: &[f64],
) -> Result<rspice_core::analysis::transient::TransientOutputProjection, String> {
    let result = &operating_point.analysis().result;
    let times = if reporting_times.is_empty() {
        &result.time
    } else {
        reporting_times
    };
    let channels = result
        .node_names
        .iter()
        .filter(|name| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .count()
        .saturating_add(result.branch_names.len());
    // Reconstruction allocates an x and y series for each displayed channel.
    let values = times
        .len()
        .saturating_mul(channels.saturating_mul(2).saturating_add(1));
    if values > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "PSS display requires {values} numerical values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
        ));
    }
    rspice_core::analysis::transient::TransientOutputProjection::interpolate_times(
        &result.time,
        times,
        MAX_WORKER_F64_VALUES,
    )
}

pub(super) fn validate_pss_display_contract(
    time: &[f64],
    waveforms: &HashMap<String, WaveformData>,
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> Result<(), SimulationError> {
    let result = &operating_point.analysis().result;
    if time.is_empty() {
        return Err(SimulationError::InvalidConfig(
            "PSS display time axis is empty".into(),
        ));
    }
    let projection =
        pss_display_projection(operating_point, time).map_err(SimulationError::InvalidConfig)?;
    let expected_count = result
        .node_names
        .iter()
        .filter(|name| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .count()
        + result.branch_names.len();
    if waveforms.len() != expected_count {
        return Err(SimulationError::InvalidConfig(format!(
            "PSS display contains {} waveforms, but its retained orbit requires {expected_count}",
            waveforms.len()
        )));
    }
    for (name, periodic, prefix, unit) in result
        .node_names
        .iter()
        .zip(&result.waveforms)
        .filter(|(name, _)| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .map(|(name, waveform)| (name, waveform, "V", "V"))
        .chain(
            result
                .branch_names
                .iter()
                .zip(&result.branch_waveforms)
                .map(|(name, waveform)| (name, waveform, "I", "A")),
        )
    {
        let display_name = format!("{prefix}({name})");
        let display = waveforms.get(&display_name).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "PSS display is missing retained-orbit waveform '{display_name}'"
            ))
        })?;
        let expected = projection
            .project(&periodic.values)
            .map_err(SimulationError::InvalidConfig)?;
        if display.name != display_name
            || display.x_values.as_slice() != time
            || display.y_values != expected
            || display.y_unit != unit
            || display.is_complex
            || display.y_imag.is_some()
        {
            return Err(SimulationError::InvalidConfig(format!(
                "PSS display waveform '{display_name}' does not match the reporting projection of its retained numerical orbit"
            )));
        }
    }
    Ok(())
}

pub(super) fn simulation_result_from_worker_pss(
    measurements: Vec<WorkerMeasurement>,
    operating_point: rspice_core::engine::PssOperatingPoint,
    reporting_times: Vec<f64>,
) -> SimulationResult {
    let projection = pss_display_projection(&operating_point, &reporting_times)
        .expect("PSS reporting grid is validated at worker ingress");
    let result = &operating_point.analysis().result;
    let time = projection.times().to_vec();
    let mut waveforms =
        HashMap::with_capacity(result.waveforms.len() + result.branch_waveforms.len());
    for (name, periodic, prefix, unit) in result
        .node_names
        .iter()
        .zip(&result.waveforms)
        .filter(|(name, _)| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .map(|(name, waveform)| (name, waveform, "V", "V"))
        .chain(
            result
                .branch_names
                .iter()
                .zip(&result.branch_waveforms)
                .map(|(name, waveform)| (name, waveform, "I", "A")),
        )
    {
        let display_name = format!("{prefix}({name})");
        waveforms.insert(
            display_name.clone(),
            WaveformData::new_time_domain_in_unit(
                display_name,
                time.clone(),
                projection
                    .project(&periodic.values)
                    .expect("retained PSS waveform lengths are authenticated"),
                unit,
            ),
        );
    }
    SimulationResult::Transient {
        spectra: Vec::new(),
        time,
        waveforms,
        measurements: measure_results(measurements),
        periodic_state: Some(std::sync::Arc::new(operating_point)),
        convergence: Default::default(),
        events: TransientEventHistory::default(),
    }
}

pub(super) fn waveform_map(waveforms: Vec<WorkerWaveform>) -> HashMap<String, WaveformData> {
    waveforms
        .into_iter()
        .map(|waveform| {
            let name = waveform.name.clone();
            (name, WaveformData::from(waveform))
        })
        .collect()
}

pub(super) fn worker_measurements(
    measurements: Vec<rspice_core::MeasureResult>,
) -> Vec<WorkerMeasurement> {
    measurements
        .into_iter()
        .map(WorkerMeasurement::from)
        .collect()
}

pub(super) fn measure_results(
    measurements: Vec<WorkerMeasurement>,
) -> Vec<rspice_core::MeasureResult> {
    measurements
        .into_iter()
        .map(rspice_core::MeasureResult::from)
        .collect()
}

pub(super) fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

/// Labels a worker response may carry, interned back to the `&'static str`
/// the host build uses.
///
/// The engine owns the vocabulary, so it is asked rather than restated.
/// `unknown` is accepted because [`intern_static_label`] produces it, and a
/// report that already crossed the boundary once has to survive crossing it
/// again unchanged. A noise mechanism does not come through here: the summary
/// carries it as owned text the whole way, so nothing interns it.
pub(super) fn known_static_label(value: &str) -> Option<&'static str> {
    rspice_core::circuit::resolve_op_label(value)
        .or_else(|| (value == "unknown").then_some("unknown"))
}
