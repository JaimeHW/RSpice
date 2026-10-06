//! Primary plotting series beside complete, authenticated quasiperiodic evidence.
use super::*;
use crate::abort_signal::AbortSignal;
use crate::engine::{
    QpacAnalysisResult, QpacInputQuantity, QpnoiseAnalysisResult, QpnoiseValue, QpssOperatingPoint,
    QpxfAnalysisResult, QpxfGroupDelay,
};
use crate::execution::result_document::quasi_periodic::evidence_error;
use crate::execution::result_document::{QpacPayload, QpnoisePayload, QpssPayload, QpxfPayload};
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};

const LOCATION: &str = "quasiperiodic result";

fn at<'a, T>(values: &'a [T], index: usize, name: &str) -> Result<&'a T, ResultDocumentError> {
    values
        .get(index)
        .ok_or_else(|| source_error(LOCATION, format!("{name} index {index} is absent")))
}

fn admit(
    source: &impl serde::Serialize,
    projected: usize,
    limits: &ResourceLimits,
) -> Result<(), ResultDocumentError> {
    let count = super::super::numeric_count::count(source).saturating_add(projected);
    ResourceLimitError::ensure(ResourceKind::ResultValues, count, limits.max_result_values)
        .map_err(ResultDocumentError::ResourceLimit)
}

fn axis(
    name: &str,
    kind: ResultAxisKind,
    values: &[f64],
) -> Result<ResultAxis, ResultDocumentError> {
    ResultAxis::new(
        name,
        name,
        kind,
        SignalUnit::Hertz,
        AxisValues::Real {
            values: values.to_vec(),
        },
    )
}

fn complex(
    name: &str,
    unit: SignalUnit,
    values: &[Complex64],
) -> Result<ResultSignal, ResultDocumentError> {
    ResultSignal::new(
        analysis_descriptor(
            LOCATION,
            name,
            name,
            unit,
            SignalValueType::Complex,
            values.len(),
        )?,
        None,
        SeriesAvailability::Available,
        SeriesValues::Complex {
            samples: finite_complex_samples(LOCATION, name, values)?,
        },
    )
}

fn real(
    name: &str,
    unit: SignalUnit,
    samples: Vec<Option<f64>>,
) -> Result<ResultSignal, ResultDocumentError> {
    ResultSignal::new(
        analysis_descriptor(
            LOCATION,
            name,
            name,
            unit,
            SignalValueType::Real,
            samples.len(),
        )?,
        None,
        SeriesAvailability::Available,
        SeriesValues::Real { samples },
    )
}

fn transfer_unit(output: QpacInputQuantity, input: QpacInputQuantity) -> SignalUnit {
    match (output, input) {
        (QpacInputQuantity::Voltage, QpacInputQuantity::Current) => SignalUnit::Ohm,
        (QpacInputQuantity::Current, QpacInputQuantity::Voltage) => SignalUnit::Siemens,
        _ => SignalUnit::Dimensionless,
    }
}

fn tuple_name(tuple: &[i32]) -> String {
    tuple
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}
fn noise_value(value: &QpnoiseValue) -> Option<f64> {
    match value {
        QpnoiseValue::Finite(value) => Some(*value),
        QpnoiseValue::Unavailable(_) => None,
    }
}

pub(in crate::execution::result_document) struct PrimaryProjection {
    points: usize,
    axes: Vec<ResultAxis>,
    signals: Vec<ResultSignal>,
}
impl PrimaryProjection {
    fn new(points: usize) -> Self {
        Self {
            points,
            axes: Vec::new(),
            signals: Vec::new(),
        }
    }
    fn axis(mut self, axis: ResultAxis) -> Self {
        self.axes.push(axis);
        self
    }
    fn signal(mut self, signal: ResultSignal) -> Self {
        self.signals.push(signal);
        self
    }
    fn finish(
        self,
        analysis: AnalysisInstanceId,
        payload: ResultPayload,
        parent: Option<AnalysisInstanceId>,
    ) -> AnalysisResultDocumentBuilder {
        let mut builder =
            AnalysisResultDocument::builder(analysis, payload, self.points).signals(self.signals);
        for axis in self.axes {
            builder = builder.axis(axis);
        }
        if let Some(parent) = parent {
            builder = builder.parent_analysis(parent);
        }
        builder
    }
    pub(in crate::execution::result_document) fn validate(
        &self,
        document: &AnalysisResultDocument,
    ) -> Result<(), ResultDocumentError> {
        if document.point_count() != self.points
            || document.axes() != self.axes
            || document.signals() != self.signals
        {
            return Err(source_error(
                LOCATION,
                "primary axes or signals differ from the retained quasiperiodic evidence",
            ));
        }
        Ok(())
    }
}

impl AnalysisResultDocument {
    /// Retain signed independent-tone spectra and the exact reusable QPSS point.
    pub fn from_qpss(
        analysis: AnalysisInstanceId,
        result: &QpssOperatingPoint,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        let grid = result
            .validate_retained_payload_with_abort(limits, abort)
            .map_err(evidence_error)?;
        let points = grid.len();
        admit(
            result,
            points.saturating_mul(2 + grid.dimensions().len() + 2 * result.spectra().len()),
            limits,
        )?;
        Ok(qpss_projection(result, &grid)?.finish(
            analysis,
            ResultPayload::Qpss(Box::new(QpssPayload {
                operating_point: result.clone(),
            })),
            None,
        ))
    }
    /// Project a QPAC output while retaining all unit-drive MNA spectra.
    pub fn from_qpac(
        analysis: AnalysisInstanceId,
        parent: AnalysisInstanceId,
        result: &QpacAnalysisResult,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        result
            .validate_retained_payload_with_abort(limits, abort)
            .map_err(evidence_error)?;
        let points = result.metadata.request.offsets_hz.len();
        let rows = result
            .metadata
            .node_names
            .len()
            .saturating_add(result.metadata.branch_names.len());
        admit(
            result,
            points.saturating_mul(7usize.saturating_add(rows.saturating_mul(2))),
            limits,
        )?;
        Ok(qpac_projection(result)?.finish(
            analysis,
            ResultPayload::Qpac(Box::new(QpacPayload {
                result: result.clone(),
            })),
            Some(parent),
        ))
    }
    /// Keep every selected source/tuple transfer with its physical input frequency.
    pub fn from_qpxf(
        analysis: AnalysisInstanceId,
        parent: AnalysisInstanceId,
        result: &QpxfAnalysisResult,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        result
            .validate_retained_payload_with_abort(limits, abort)
            .map_err(evidence_error)?;
        let points = result.metadata.request.frequencies_hz.len();
        admit(
            result,
            points.saturating_mul(1usize.saturating_add(result.transfers.len().saturating_mul(4))),
            limits,
        )?;
        Ok(qpxf_projection(result)?.finish(
            analysis,
            ResultPayload::Qpxf(Box::new(QpxfPayload {
                result: result.clone(),
            })),
            Some(parent),
        ))
    }
    /// Retain full noise covariance, source evidence, referral and integration status.
    pub fn from_qpnoise(
        analysis: AnalysisInstanceId,
        parent: AnalysisInstanceId,
        result: &QpnoiseAnalysisResult,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError> {
        result
            .validate_retained_payload_with_abort(limits, abort)
            .map_err(evidence_error)?;
        let points = result.points.len();
        admit(
            result,
            points.saturating_mul(1usize.saturating_add(result.outputs.len().saturating_mul(4))),
            limits,
        )?;
        Ok(qpnoise_projection(result)?.finish(
            analysis,
            ResultPayload::Qpnoise(Box::new(QpnoisePayload {
                result: result.clone(),
            })),
            Some(parent),
        ))
    }
}

pub(in crate::execution::result_document) fn qpss_projection(
    result: &QpssOperatingPoint,
    grid: &crate::analysis::quasi_periodic::QuasiPeriodicGrid,
) -> Result<PrimaryProjection, ResultDocumentError> {
    let points = grid.len();
    let mut builder = PrimaryProjection::new(points)
        .axis(ResultAxis::new(
            "tuple_index",
            "Signed tone tuple index",
            ResultAxisKind::Index,
            SignalUnit::Dimensionless,
            AxisValues::Integer {
                values: (0..points)
                    .map(|index| {
                        i64::try_from(index)
                            .map_err(|_| source_error(LOCATION, "tuple index exceeds i64"))
                    })
                    .collect::<Result<_, _>>()?,
            },
        )?)
        .signal(real(
            "frequency",
            SignalUnit::Hertz,
            grid.frequencies_hz().iter().copied().map(Some).collect(),
        )?);
    for tone in 0..grid.dimensions().len() {
        builder = builder.signal(real(
            &format!("tone_index({})", tone + 1),
            SignalUnit::Dimensionless,
            grid.indices()
                .iter()
                .map(|tuple| at(tuple, tone, "tone").map(|value| Some(f64::from(*value))))
                .collect::<Result<_, _>>()?,
        )?);
    }
    for (index, values) in result.spectra().iter().enumerate() {
        let descriptor = if index < result.node_names().len() {
            voltage_descriptor(
                LOCATION,
                at(result.node_names(), index, "node")?,
                SignalValueType::Complex,
                points,
            )?
        } else {
            current_descriptor(
                LOCATION,
                at(
                    result.branch_names(),
                    index - result.node_names().len(),
                    "branch",
                )?,
                SignalValueType::Complex,
                points,
            )?
        };
        builder = builder.signal(ResultSignal::new(
            descriptor,
            None,
            SeriesAvailability::Available,
            SeriesValues::Complex {
                samples: finite_complex_samples(LOCATION, "QPSS spectrum", values)?,
            },
        )?);
    }
    Ok(builder)
}

pub(in crate::execution::result_document) fn qpac_projection(
    result: &QpacAnalysisResult,
) -> Result<PrimaryProjection, ResultDocumentError> {
    let points = result.metadata.request.offsets_hz.len();
    let mut projection = PrimaryProjection::new(points)
        .axis(axis(
            "offset_frequency",
            ResultAxisKind::OffsetFrequency,
            &result.metadata.request.offsets_hz,
        )?)
        .signal(complex(
            "output_transfer",
            transfer_unit(QpacInputQuantity::Voltage, result.metadata.input_quantity),
            &result.output_transfer,
        )?)
        .signal(complex(
            "output_response",
            SignalUnit::Volt,
            &result.output_response,
        )?)
        .signal(real(
            "input_frequency",
            SignalUnit::Hertz,
            result
                .metadata
                .input_frequencies_hz
                .iter()
                .copied()
                .map(Some)
                .collect(),
        )?)
        .signal(real(
            "output_frequency",
            SignalUnit::Hertz,
            result
                .metadata
                .output_frequencies_hz
                .iter()
                .copied()
                .map(Some)
                .collect(),
        )?);
    let tuple = result
        .metadata
        .tuples
        .iter()
        .position(|tuple| tuple == &result.metadata.request.output_lattice)
        .ok_or_else(|| source_error(LOCATION, "QPAC output tuple is absent"))?;
    let drive = result.metadata.drive();
    for row in 0..result.metadata.node_names.len() + result.metadata.branch_names.len() {
        let descriptor = if row < result.metadata.node_names.len() {
            voltage_descriptor(
                LOCATION,
                at(&result.metadata.node_names, row, "node")?,
                SignalValueType::Complex,
                points,
            )?
        } else {
            current_descriptor(
                LOCATION,
                at(
                    &result.metadata.branch_names,
                    row - result.metadata.node_names.len(),
                    "branch",
                )?,
                SignalValueType::Complex,
                points,
            )?
        };
        let samples = result
            .unit_solutions
            .iter()
            .map(|solution| {
                let value = *at(
                    at(&solution.spectra, row, "MNA row")?,
                    tuple,
                    "output tuple",
                )? * drive;
                Ok(Some(ComplexSample::new(value.re, value.im)))
            })
            .collect::<Result<_, ResultDocumentError>>()?;
        projection = projection.signal(ResultSignal::new(
            descriptor,
            None,
            SeriesAvailability::Available,
            SeriesValues::Complex { samples },
        )?);
    }
    Ok(projection)
}

pub(in crate::execution::result_document) fn qpxf_projection(
    result: &QpxfAnalysisResult,
) -> Result<PrimaryProjection, ResultDocumentError> {
    let points = result.metadata.request.frequencies_hz.len();
    let mut builder = PrimaryProjection::new(points).axis(axis(
        "output_frequency",
        ResultAxisKind::Frequency,
        &result.metadata.output_frequencies_hz,
    )?);
    for transfer in &result.transfers {
        let source = at(
            &result.metadata.input_sources,
            transfer.input_source,
            "input source",
        )?;
        let path = format!("{};{}", source.name, tuple_name(&transfer.input_lattice));
        builder = builder
            .signal(complex(
                &format!("transfer({path})"),
                transfer_unit(result.metadata.request.output.quantity(), source.quantity),
                &transfer.values,
            )?)
            .signal(real(
                &format!("input_frequency({path})"),
                SignalUnit::Hertz,
                transfer
                    .input_frequencies_hz
                    .iter()
                    .copied()
                    .map(Some)
                    .collect(),
            )?);
        if let Some(delay) = &transfer.group_delay {
            builder = builder.signal(real(
                &format!("group_delay({path})"),
                SignalUnit::Second,
                delay
                    .iter()
                    .map(|delay| match delay {
                        QpxfGroupDelay::Finite(value) => Some(*value),
                        _ => None,
                    })
                    .collect(),
            )?);
        }
    }
    Ok(builder)
}

pub(in crate::execution::result_document) fn qpnoise_projection(
    result: &QpnoiseAnalysisResult,
) -> Result<PrimaryProjection, ResultDocumentError> {
    let points = result.points.len();
    let mut builder = PrimaryProjection::new(points).axis(axis(
        "authored_frequency",
        ResultAxisKind::Frequency,
        &result.metadata.request.frequencies_hz,
    )?);
    for (index, (output, spectrum)) in result
        .metadata
        .request
        .outputs
        .iter()
        .zip(&result.outputs)
        .enumerate()
    {
        let prefix = format!("output({})", index + 1);
        let unit = match output.observation.quantity() {
            QpacInputQuantity::Voltage => volt_squared_per_hertz(),
            QpacInputQuantity::Current => ampere_squared_per_hertz(),
        };
        builder = builder
            .signal(real(
                &format!("{prefix}.frequency"),
                SignalUnit::Hertz,
                spectrum.frequencies_hz.iter().copied().map(Some).collect(),
            )?)
            .signal(real(
                &format!("{prefix}.noise_psd"),
                unit,
                result
                    .total_covariances
                    .iter()
                    .map(|covariance| {
                        at(
                            &covariance.values,
                            index
                                .saturating_mul(covariance.outputs)
                                .saturating_add(index),
                            "covariance diagonal",
                        )
                        .map(|value| Some(value.re))
                    })
                    .collect::<Result<_, _>>()?,
            )?);
        if let Some(values) = &spectrum.input_noise {
            let unit = match result
                .metadata
                .input_source
                .as_ref()
                .map(|source| source.quantity)
            {
                Some(QpacInputQuantity::Current) => ampere_squared_per_hertz(),
                _ => volt_squared_per_hertz(),
            };
            builder = builder.signal(real(
                &format!("{prefix}.input_noise_psd"),
                unit,
                values.iter().map(noise_value).collect(),
            )?);
        }
        if let Some(values) = &spectrum.noise_figure_db {
            builder = builder.signal(real(
                &format!("{prefix}.noise_figure_db"),
                SignalUnit::Custom("dB".into()),
                values.iter().map(noise_value).collect(),
            )?);
        }
    }
    Ok(builder)
}
