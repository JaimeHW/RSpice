//! Versioned, lossless STB result documents for browser consumers.
//!
//! Core deliberately retains the primary loop-gain sweep and its derived
//! Bode/Nyquist records. This document preserves those records separately so
//! browser projection has exactly the same resource-accounting shape instead
//! of silently coalescing duplicate frequency and complex-gain columns.

use std::fmt;

use rspice_core::{AbortSignal, NoAbort, engine::StbAnalysisResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeSeq};

pub const STB_RESULT_SCHEMA: &str = "rspice-stb-result";
pub const STB_RESULT_VERSION: u32 = 1;
const STB_PROJECTION_ABORT_POLL_STRIDE: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StbDocumentError {
    Aborted,
    Invalid(String),
    Allocation(String),
}

impl From<String> for StbDocumentError {
    fn from(message: String) -> Self {
        Self::Invalid(message)
    }
}

impl fmt::Display for StbDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aborted => formatter.write_str("STB result projection was aborted"),
            Self::Invalid(message) => formatter.write_str(message),
            Self::Allocation(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for StbDocumentError {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbResultDocument {
    pub schema: String,
    pub schema_version: u32,
    pub analysis: StbAnalysisIdentity,
    /// Scalar execution has no authored STEP/TEMP coordinate.
    pub coordinate_id: Option<String>,
    pub point_count: usize,
    pub probe_name: String,
    pub primary: StbPrimarySeries,
    pub bode: StbBodeSeries,
    /// `None` means Nyquist projection was disabled, while `Some` retains a
    /// complete contour aligned with the primary sweep.
    pub nyquist: Option<StbNyquistSeries>,
    pub margins: StbMargins,
    pub success: bool,
    pub warnings: Vec<String>,
    pub assessment: String,
}

impl StbResultDocument {
    pub fn validate(&self) -> Result<(), String> {
        match self.validate_with_abort(&NoAbort) {
            Ok(()) => Ok(()),
            Err(StbDocumentError::Invalid(message)) => Err(message),
            Err(StbDocumentError::Allocation(message)) => Err(message),
            Err(StbDocumentError::Aborted) => {
                unreachable!("NoAbort cannot cancel STB document validation")
            }
        }
    }

    pub(crate) fn validate_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<(), StbDocumentError> {
        ensure_projection_not_aborted(abort)?;
        if self.schema != STB_RESULT_SCHEMA {
            return Err(format!("unexpected STB result schema {:?}", self.schema).into());
        }
        if self.schema_version != STB_RESULT_VERSION {
            return Err(format!(
                "STB result version {} is unsupported (current version is {})",
                self.schema_version, STB_RESULT_VERSION
            )
            .into());
        }
        if self.analysis.ordinal == 0
            || self.analysis.id != format!("stb-{:03}", self.analysis.ordinal)
            || self.analysis.kind != StbAnalysisKind::Stability
            || self.analysis.request_kind != "stb"
        {
            return Err("STB analysis identity does not match its kind and ordinal"
                .to_owned()
                .into());
        }
        if self.coordinate_id.is_some() {
            return Err("scalar STB results must not carry a STEP/TEMP coordinate"
                .to_owned()
                .into());
        }
        if self.point_count == 0 || self.probe_name.trim().is_empty() {
            return Err("STB requires a nonzero point count and probe identity"
                .to_owned()
                .into());
        }
        if self.primary.frequency_unit != StbUnit::Hertz
            || self.primary.loop_gain_unit != StbUnit::Dimensionless
            || self.bode.frequency_unit != StbUnit::Hertz
            || self.bode.magnitude_unit != StbUnit::Dimensionless
            || self.bode.magnitude_db_unit != StbUnit::Decibel
            || self.bode.phase_unit != StbUnit::Degree
            || self.bode.loop_gain_unit != StbUnit::Dimensionless
        {
            return Err("STB primary or Bode units do not match the schema"
                .to_owned()
                .into());
        }
        self.validate_series_shapes()?;
        for (index, frequency) in self.primary.frequencies.iter().enumerate() {
            poll_projection_abort(abort, index)?;
            if !frequency.is_finite() || *frequency <= 0.0 {
                return Err("STB primary frequencies must be finite and positive"
                    .to_owned()
                    .into());
            }
        }
        if !same_float_slice_with_abort(&self.primary.frequencies, &self.bode.frequencies, abort)?
            || !same_complex_slice_with_abort(
                &self.primary.loop_gains,
                &self.bode.loop_gains,
                abort,
            )?
        {
            return Err("STB Bode records are not aligned with the primary sweep"
                .to_owned()
                .into());
        }
        for (index, ((frequency, gain), ((magnitude, magnitude_db), phase_degrees))) in self
            .primary
            .frequencies
            .iter()
            .zip(&self.primary.loop_gains)
            .zip(
                self.bode
                    .magnitudes
                    .iter()
                    .zip(&self.bode.magnitudes_db)
                    .zip(&self.bode.phase_degrees),
            )
            .enumerate()
        {
            poll_projection_abort(abort, index)?;
            let expected = rspice_core::analysis::BodePoint::from_loop_gain(
                *frequency,
                rspice_core::Complex64::new(gain.real, gain.imaginary),
            );
            if !same_derived_float(*magnitude, expected.magnitude)
                || !same_derived_float(*magnitude_db, expected.magnitude_db)
                || !same_derived_float(*phase_degrees, expected.phase_deg)
            {
                return Err(format!(
                    "STB Bode magnitude, decibel, or phase data contradict the loop gain at point {index}: retained [{magnitude:?}, {magnitude_db:?}, {phase_degrees:?}], derived [{:?}, {:?}, {:?}]",
                    expected.magnitude, expected.magnitude_db, expected.phase_deg
                )
                .into());
            }
        }

        if let Some(nyquist) = &self.nyquist {
            if nyquist.real_unit != StbUnit::Dimensionless
                || nyquist.imaginary_unit != StbUnit::Dimensionless
                || nyquist.frequency_unit != StbUnit::Hertz
            {
                return Err("STB Nyquist units do not match the schema"
                    .to_owned()
                    .into());
            }
            if !same_float_slice_with_abort(&self.primary.frequencies, &nyquist.frequencies, abort)?
                || !complex_component_matches_with_abort(
                    &self.primary.loop_gains,
                    &nyquist.real,
                    abort,
                    |gain| gain.real,
                )?
                || !complex_component_matches_with_abort(
                    &self.primary.loop_gains,
                    &nyquist.imaginary,
                    abort,
                    |gain| gain.imaginary,
                )?
            {
                return Err("STB Nyquist records are not aligned with the primary sweep"
                    .to_owned()
                    .into());
            }
        }
        self.margins
            .validate_units()
            .map_err(StbDocumentError::Invalid)?;
        let expected_stable =
            self.margins.gain_margin_db > 0.0 && self.margins.phase_margin_degrees > 0.0;
        let expected_conditionally_stable = self.margins.num_crossovers > 1;
        let expected_assessment = if !expected_stable {
            "UNSTABLE"
        } else if expected_conditionally_stable {
            "CONDITIONALLY STABLE"
        } else if self.margins.phase_margin_degrees < 30.0 {
            "MARGINALLY STABLE"
        } else if self.margins.phase_margin_degrees >= 60.0 && self.margins.gain_margin_db >= 12.0 {
            "WELL DAMPED"
        } else {
            "STABLE"
        };
        if self.margins.is_stable != expected_stable
            || self.margins.conditionally_stable != expected_conditionally_stable
            || self.assessment != expected_assessment
        {
            return Err(
                "STB margin flags or assessment contradict the retained margin values"
                    .to_owned()
                    .into(),
            );
        }
        for (index, warning) in self.warnings.iter().enumerate() {
            poll_projection_abort(abort, index)?;
            if warning.is_empty() {
                return Err("STB assessment and warning strings must not be empty"
                    .to_owned()
                    .into());
            }
        }
        ensure_projection_not_aborted(abort)?;
        Ok(())
    }

    /// Numeric values retained by core and by this document. Boolean flags,
    /// counters, strings, and unit descriptors are intentionally not charged.
    pub fn retained_numeric_value_count(&self) -> Result<usize, String> {
        checked_retained_numeric_value_count(self.point_count, self.nyquist.is_some())
    }

    pub fn metadata(
        &self,
        maximum_window_values: usize,
    ) -> Result<StbResultMetadata, StbDocumentError> {
        let mut series = vec![
            descriptor("primary", "frequency", StbUnit::Hertz, StbValueType::Real),
            descriptor(
                "primary",
                "loop_gain",
                StbUnit::Dimensionless,
                StbValueType::Complex,
            ),
            descriptor("bode", "frequency", StbUnit::Hertz, StbValueType::Real),
            descriptor(
                "bode",
                "magnitude",
                StbUnit::Dimensionless,
                StbValueType::Real,
            ),
            descriptor("bode", "magnitude_db", StbUnit::Decibel, StbValueType::Real),
            descriptor("bode", "phase_degrees", StbUnit::Degree, StbValueType::Real),
            descriptor(
                "bode",
                "loop_gain",
                StbUnit::Dimensionless,
                StbValueType::Complex,
            ),
        ];
        if self.nyquist.is_some() {
            series.extend([
                descriptor(
                    "nyquist",
                    "real",
                    StbUnit::Dimensionless,
                    StbValueType::Real,
                ),
                descriptor(
                    "nyquist",
                    "imaginary",
                    StbUnit::Dimensionless,
                    StbValueType::Real,
                ),
                descriptor("nyquist", "frequency", StbUnit::Hertz, StbValueType::Real),
            ]);
        }
        Ok(StbResultMetadata {
            schema: try_clone_string(&self.schema, "STB metadata schema")?,
            schema_version: self.schema_version,
            analysis: StbAnalysisIdentity {
                id: try_clone_string(&self.analysis.id, "STB metadata analysis ID")?,
                kind: self.analysis.kind,
                request_kind: try_clone_string(
                    &self.analysis.request_kind,
                    "STB metadata request kind",
                )?,
                ordinal: self.analysis.ordinal,
            },
            coordinate_id: self
                .coordinate_id
                .as_deref()
                .map(|value| try_clone_string(value, "STB metadata coordinate ID"))
                .transpose()?,
            point_count: self.point_count,
            probe_name: try_clone_string(&self.probe_name, "STB metadata probe name")?,
            has_nyquist: self.nyquist.is_some(),
            series,
            margin_descriptors: StbMarginDescriptors::default(),
            margins: self.margins.clone(),
            success: self.success,
            warnings: try_clone_strings(&self.warnings, "STB metadata warnings")?,
            assessment: try_clone_string(&self.assessment, "STB metadata assessment")?,
            maximum_window_values,
        })
    }

    pub fn window(
        &self,
        start: usize,
        count: usize,
        maximum_values: usize,
    ) -> Result<StbResultWindow, StbDocumentError> {
        self.validate_series_shapes()?;
        let end = start
            .checked_add(count)
            .ok_or_else(|| "STB result window range overflows usize".to_owned())?;
        if count == 0 || start >= self.point_count || end > self.point_count {
            return Err(format!(
                "STB result window [{start}, {end}) is outside 0..{}",
                self.point_count
            )
            .into());
        }
        let requested_values = checked_window_value_count(count, self.nyquist.is_some())?;
        if requested_values > maximum_values {
            return Err(format!(
                "STB result window requires {requested_values} numeric values but the transfer limit is {maximum_values}"
            )
            .into());
        }
        let primary_frequencies = try_copy_f64_slice(
            &self.primary.frequencies[start..end],
            "STB primary frequency window",
        )?;
        let primary_loop_gain = complex_window(
            &self.primary.loop_gains[start..end],
            "STB primary loop-gain window",
        )?;
        let bode_frequencies = try_copy_f64_slice(
            &self.bode.frequencies[start..end],
            "STB Bode frequency window",
        )?;
        let bode_magnitudes = try_copy_f64_slice(
            &self.bode.magnitudes[start..end],
            "STB Bode magnitude window",
        )?;
        let bode_magnitudes_db = try_copy_f64_slice(
            &self.bode.magnitudes_db[start..end],
            "STB Bode decibel window",
        )?;
        let bode_phase_degrees = try_copy_f64_slice(
            &self.bode.phase_degrees[start..end],
            "STB Bode phase window",
        )?;
        let bode_loop_gain = complex_window(
            &self.bode.loop_gains[start..end],
            "STB Bode loop-gain window",
        )?;
        let nyquist = self
            .nyquist
            .as_ref()
            .map(|nyquist| {
                Ok::<StbNyquistWindow, StbDocumentError>(StbNyquistWindow {
                    real: try_copy_f64_slice(&nyquist.real[start..end], "STB Nyquist real window")?,
                    imaginary: try_copy_f64_slice(
                        &nyquist.imaginary[start..end],
                        "STB Nyquist imaginary window",
                    )?,
                    frequencies: try_copy_f64_slice(
                        &nyquist.frequencies[start..end],
                        "STB Nyquist frequency window",
                    )?,
                })
            })
            .transpose()?;
        Ok(StbResultWindow {
            schema_version: self.schema_version,
            analysis_id: try_clone_string(&self.analysis.id, "STB window analysis ID")?,
            coordinate_id: self
                .coordinate_id
                .as_deref()
                .map(|value| try_clone_string(value, "STB window coordinate ID"))
                .transpose()?,
            start,
            end,
            point_count: self.point_count,
            primary: StbPrimaryWindow {
                frequencies: primary_frequencies,
                loop_gain: primary_loop_gain,
            },
            bode: StbBodeWindow {
                frequencies: bode_frequencies,
                magnitudes: bode_magnitudes,
                magnitudes_db: bode_magnitudes_db,
                phase_degrees: bode_phase_degrees,
                loop_gain: bode_loop_gain,
            },
            nyquist,
        })
    }

    fn validate_series_shapes(&self) -> Result<(), StbDocumentError> {
        if self.primary.frequencies.len() != self.point_count
            || self.primary.loop_gains.len() != self.point_count
            || self.bode.frequencies.len() != self.point_count
            || self.bode.magnitudes.len() != self.point_count
            || self.bode.magnitudes_db.len() != self.point_count
            || self.bode.phase_degrees.len() != self.point_count
            || self.bode.loop_gains.len() != self.point_count
            || self.nyquist.as_ref().is_some_and(|nyquist| {
                nyquist.real.len() != self.point_count
                    || nyquist.imaginary.len() != self.point_count
                    || nyquist.frequencies.len() != self.point_count
            })
        {
            return Err(StbDocumentError::Invalid(
                "STB result series do not match the point count".to_owned(),
            ));
        }
        Ok(())
    }
}

pub fn stb_document_with_abort(
    result: StbAnalysisResult,
    ordinal: usize,
    abort: &dyn AbortSignal,
) -> Result<StbResultDocument, StbDocumentError> {
    ensure_projection_not_aborted(abort)?;
    let point_count = result.frequencies.len();
    checked_retained_numeric_value_count(point_count, !result.result.nyquist_points.is_empty())?;
    let primary_loop_gains = try_map_slice_with_abort(
        &result.loop_gains,
        "STB primary loop-gain projection",
        abort,
        |gain| StbComplexSample::from(*gain),
    )?;
    let has_nyquist = !result.result.nyquist_points.is_empty();
    let margins = &result.result.margins;
    let margins = StbMargins {
        gain_margin_db: margins.gain_margin_db,
        gain_margin_frequency: margins.gain_margin_freq,
        phase_margin_degrees: margins.phase_margin_deg,
        phase_margin_frequency: margins.phase_margin_freq,
        dc_gain_db: margins.dc_gain_db,
        unity_gain_bandwidth: margins.unity_gain_bandwidth,
        conditionally_stable: margins.conditionally_stable,
        num_crossovers: margins.num_crossovers,
        is_stable: margins.is_stable(),
        units: StbMarginUnits::default(),
    };
    let assessment = result.result.assessment();
    let bode_frequencies = try_map_slice_with_abort(
        &result.result.bode_points,
        "STB Bode frequency projection",
        abort,
        |point| point.frequency,
    )?;
    let bode_magnitudes = try_map_slice_with_abort(
        &result.result.bode_points,
        "STB Bode magnitude projection",
        abort,
        |point| point.magnitude,
    )?;
    let bode_magnitudes_db = try_map_slice_with_abort(
        &result.result.bode_points,
        "STB Bode decibel projection",
        abort,
        |point| point.magnitude_db,
    )?;
    let bode_phase_degrees = try_map_slice_with_abort(
        &result.result.bode_points,
        "STB Bode phase projection",
        abort,
        |point| point.phase_deg,
    )?;
    let bode_loop_gains = try_map_slice_with_abort(
        &result.result.bode_points,
        "STB Bode loop-gain projection",
        abort,
        |point| StbComplexSample::from(point.loop_gain),
    )?;
    let nyquist = if has_nyquist {
        Some(StbNyquistSeries {
            real_unit: StbUnit::Dimensionless,
            imaginary_unit: StbUnit::Dimensionless,
            frequency_unit: StbUnit::Hertz,
            real: try_map_slice_with_abort(
                &result.result.nyquist_points,
                "STB Nyquist real projection",
                abort,
                |point| point.real,
            )?,
            imaginary: try_map_slice_with_abort(
                &result.result.nyquist_points,
                "STB Nyquist imaginary projection",
                abort,
                |point| point.imag,
            )?,
            frequencies: try_map_slice_with_abort(
                &result.result.nyquist_points,
                "STB Nyquist frequency projection",
                abort,
                |point| point.frequency,
            )?,
        })
    } else {
        None
    };
    let document = StbResultDocument {
        schema: STB_RESULT_SCHEMA.to_owned(),
        schema_version: STB_RESULT_VERSION,
        analysis: StbAnalysisIdentity {
            id: format!("stb-{ordinal:03}"),
            kind: StbAnalysisKind::Stability,
            request_kind: "stb".to_owned(),
            ordinal,
        },
        coordinate_id: None,
        point_count,
        probe_name: result.probe_name,
        primary: StbPrimarySeries {
            frequency_unit: StbUnit::Hertz,
            loop_gain_unit: StbUnit::Dimensionless,
            frequencies: result.frequencies,
            loop_gains: primary_loop_gains,
        },
        bode: StbBodeSeries {
            frequency_unit: StbUnit::Hertz,
            magnitude_unit: StbUnit::Dimensionless,
            magnitude_db_unit: StbUnit::Decibel,
            phase_unit: StbUnit::Degree,
            loop_gain_unit: StbUnit::Dimensionless,
            frequencies: bode_frequencies,
            magnitudes: bode_magnitudes,
            magnitudes_db: bode_magnitudes_db,
            phase_degrees: bode_phase_degrees,
            loop_gains: bode_loop_gains,
        },
        nyquist,
        margins,
        success: result.result.success,
        assessment,
        warnings: result.result.warnings,
    };
    document.validate_with_abort(abort)?;
    Ok(document)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbAnalysisIdentity {
    pub id: String,
    pub kind: StbAnalysisKind,
    pub request_kind: String,
    pub ordinal: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StbAnalysisKind {
    Stability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StbUnit {
    Hertz,
    Dimensionless,
    Decibel,
    Degree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StbValueType {
    Real,
    Complex,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbComplexSample {
    #[serde(with = "float_wire")]
    pub real: f64,
    #[serde(with = "float_wire")]
    pub imaginary: f64,
}

impl From<rspice_core::Complex64> for StbComplexSample {
    fn from(value: rspice_core::Complex64) -> Self {
        Self {
            real: value.re,
            imaginary: value.im,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbPrimarySeries {
    pub frequency_unit: StbUnit,
    pub loop_gain_unit: StbUnit,
    #[serde(with = "float_vec_wire")]
    pub frequencies: Vec<f64>,
    pub loop_gains: Vec<StbComplexSample>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbBodeSeries {
    pub frequency_unit: StbUnit,
    pub magnitude_unit: StbUnit,
    pub magnitude_db_unit: StbUnit,
    pub phase_unit: StbUnit,
    pub loop_gain_unit: StbUnit,
    #[serde(with = "float_vec_wire")]
    pub frequencies: Vec<f64>,
    #[serde(with = "float_vec_wire")]
    pub magnitudes: Vec<f64>,
    #[serde(with = "float_vec_wire")]
    pub magnitudes_db: Vec<f64>,
    #[serde(with = "float_vec_wire")]
    pub phase_degrees: Vec<f64>,
    pub loop_gains: Vec<StbComplexSample>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbNyquistSeries {
    pub real_unit: StbUnit,
    pub imaginary_unit: StbUnit,
    pub frequency_unit: StbUnit,
    #[serde(with = "float_vec_wire")]
    pub real: Vec<f64>,
    #[serde(with = "float_vec_wire")]
    pub imaginary: Vec<f64>,
    #[serde(with = "float_vec_wire")]
    pub frequencies: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbMargins {
    #[serde(with = "float_wire")]
    pub gain_margin_db: f64,
    #[serde(with = "float_wire")]
    pub gain_margin_frequency: f64,
    #[serde(with = "float_wire")]
    pub phase_margin_degrees: f64,
    #[serde(with = "float_wire")]
    pub phase_margin_frequency: f64,
    #[serde(with = "float_wire")]
    pub dc_gain_db: f64,
    #[serde(with = "float_wire")]
    pub unity_gain_bandwidth: f64,
    pub conditionally_stable: bool,
    pub num_crossovers: usize,
    pub is_stable: bool,
    pub units: StbMarginUnits,
}

impl StbMargins {
    fn validate_units(&self) -> Result<(), String> {
        if self.units != StbMarginUnits::default() {
            return Err("STB margin units do not match the schema".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StbMarginUnits {
    pub gain_margin_db: StbUnit,
    pub gain_margin_frequency: StbUnit,
    pub phase_margin_degrees: StbUnit,
    pub phase_margin_frequency: StbUnit,
    pub dc_gain_db: StbUnit,
    pub unity_gain_bandwidth: StbUnit,
}

impl Default for StbMarginUnits {
    fn default() -> Self {
        Self {
            gain_margin_db: StbUnit::Decibel,
            gain_margin_frequency: StbUnit::Hertz,
            phase_margin_degrees: StbUnit::Degree,
            phase_margin_frequency: StbUnit::Hertz,
            dc_gain_db: StbUnit::Decibel,
            unity_gain_bandwidth: StbUnit::Hertz,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbResultMetadata {
    pub schema: String,
    pub schema_version: u32,
    pub analysis: StbAnalysisIdentity,
    pub coordinate_id: Option<String>,
    pub point_count: usize,
    pub probe_name: String,
    pub has_nyquist: bool,
    pub series: Vec<StbSeriesDescriptor>,
    pub margin_descriptors: StbMarginDescriptors,
    pub margins: StbMargins,
    pub success: bool,
    pub warnings: Vec<String>,
    pub assessment: String,
    pub maximum_window_values: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbSeriesDescriptor {
    pub group: String,
    pub name: String,
    pub unit: StbUnit,
    pub value_type: StbValueType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbMarginDescriptors {
    pub gain_margin_db: StbUnit,
    pub gain_margin_frequency: StbUnit,
    pub phase_margin_degrees: StbUnit,
    pub phase_margin_frequency: StbUnit,
    pub dc_gain_db: StbUnit,
    pub unity_gain_bandwidth: StbUnit,
}

impl Default for StbMarginDescriptors {
    fn default() -> Self {
        let units = StbMarginUnits::default();
        Self {
            gain_margin_db: units.gain_margin_db,
            gain_margin_frequency: units.gain_margin_frequency,
            phase_margin_degrees: units.phase_margin_degrees,
            phase_margin_frequency: units.phase_margin_frequency,
            dc_gain_db: units.dc_gain_db,
            unity_gain_bandwidth: units.unity_gain_bandwidth,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbResultWindow {
    pub schema_version: u32,
    pub analysis_id: String,
    pub coordinate_id: Option<String>,
    pub start: usize,
    pub end: usize,
    pub point_count: usize,
    pub primary: StbPrimaryWindow,
    pub bode: StbBodeWindow,
    pub nyquist: Option<StbNyquistWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbPrimaryWindow {
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub frequencies: Vec<f64>,
    pub loop_gain: StbComplexWindow,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbBodeWindow {
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub frequencies: Vec<f64>,
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub magnitudes: Vec<f64>,
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub magnitudes_db: Vec<f64>,
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub phase_degrees: Vec<f64>,
    pub loop_gain: StbComplexWindow,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbNyquistWindow {
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub real: Vec<f64>,
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub imaginary: Vec<f64>,
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub frequencies: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StbComplexWindow {
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub real: Vec<f64>,
    #[serde(serialize_with = "float_vec_wire::serialize")]
    pub imaginary: Vec<f64>,
}

fn descriptor(
    group: &str,
    name: &str,
    unit: StbUnit,
    value_type: StbValueType,
) -> StbSeriesDescriptor {
    StbSeriesDescriptor {
        group: group.to_owned(),
        name: name.to_owned(),
        unit,
        value_type,
    }
}

fn checked_retained_numeric_value_count(
    point_count: usize,
    has_nyquist: bool,
) -> Result<usize, String> {
    let values_per_point = if has_nyquist { 12usize } else { 9usize };
    point_count
        .checked_mul(values_per_point)
        .and_then(|values| values.checked_add(6))
        .ok_or_else(|| {
            format!(
                "STB retained-value count overflows usize for {point_count} points and {values_per_point} values per point"
            )
        })
}

fn checked_window_value_count(count: usize, has_nyquist: bool) -> Result<usize, String> {
    let values_per_point = if has_nyquist { 12usize } else { 9usize };
    count.checked_mul(values_per_point).ok_or_else(|| {
        format!(
            "STB result window value count overflows usize for {count} points and {values_per_point} values per point"
        )
    })
}

fn allocation_error(context: &str, error: std::collections::TryReserveError) -> String {
    format!("{context} allocation failed: {error}")
}

fn try_copy_f64_slice(values: &[f64], context: &str) -> Result<Vec<f64>, StbDocumentError> {
    let mut copied = Vec::new();
    copied
        .try_reserve_exact(values.len())
        .map_err(|error| StbDocumentError::Allocation(allocation_error(context, error)))?;
    copied.extend_from_slice(values);
    Ok(copied)
}

fn ensure_projection_not_aborted(abort: &dyn AbortSignal) -> Result<(), StbDocumentError> {
    if abort.is_aborted() {
        Err(StbDocumentError::Aborted)
    } else {
        Ok(())
    }
}

fn poll_projection_abort(abort: &dyn AbortSignal, index: usize) -> Result<(), StbDocumentError> {
    if index.is_multiple_of(STB_PROJECTION_ABORT_POLL_STRIDE) {
        ensure_projection_not_aborted(abort)?;
    }
    Ok(())
}

fn try_map_slice_with_abort<T, U>(
    values: &[T],
    context: &str,
    abort: &dyn AbortSignal,
    mut map: impl FnMut(&T) -> U,
) -> Result<Vec<U>, StbDocumentError> {
    let mut mapped = Vec::new();
    mapped
        .try_reserve_exact(values.len())
        .map_err(|error| StbDocumentError::Allocation(allocation_error(context, error)))?;
    for (index, value) in values.iter().enumerate() {
        poll_projection_abort(abort, index)?;
        mapped.push(map(value));
    }
    ensure_projection_not_aborted(abort)?;
    Ok(mapped)
}

fn try_clone_string(value: &str, context: &str) -> Result<String, StbDocumentError> {
    let mut cloned = String::new();
    cloned
        .try_reserve_exact(value.len())
        .map_err(|error| StbDocumentError::Allocation(allocation_error(context, error)))?;
    cloned.push_str(value);
    Ok(cloned)
}

fn try_clone_strings(values: &[String], context: &str) -> Result<Vec<String>, StbDocumentError> {
    let mut cloned = Vec::new();
    cloned
        .try_reserve_exact(values.len())
        .map_err(|error| StbDocumentError::Allocation(allocation_error(context, error)))?;
    for value in values {
        cloned.push(try_clone_string(value, context)?);
    }
    Ok(cloned)
}

fn complex_window(
    samples: &[StbComplexSample],
    context: &str,
) -> Result<StbComplexWindow, StbDocumentError> {
    let mut real = Vec::new();
    let mut imaginary = Vec::new();
    real.try_reserve_exact(samples.len())
        .map_err(|error| StbDocumentError::Allocation(allocation_error(context, error)))?;
    imaginary
        .try_reserve_exact(samples.len())
        .map_err(|error| StbDocumentError::Allocation(allocation_error(context, error)))?;
    for sample in samples {
        real.push(sample.real);
        imaginary.push(sample.imaginary);
    }
    Ok(StbComplexWindow { real, imaginary })
}

fn same_float(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits()
}

fn same_derived_float(left: f64, right: f64) -> bool {
    same_float(left, right)
        || (left.is_finite()
            && right.is_finite()
            && (left - right).abs() <= 32.0 * f64::EPSILON * left.abs().max(right.abs()).max(1.0))
}

fn same_float_slice_with_abort(
    left: &[f64],
    right: &[f64],
    abort: &dyn AbortSignal,
) -> Result<bool, StbDocumentError> {
    if left.len() != right.len() {
        return Ok(false);
    }
    for (index, (left, right)) in left.iter().zip(right).enumerate() {
        poll_projection_abort(abort, index)?;
        if !same_float(*left, *right) {
            return Ok(false);
        }
    }
    ensure_projection_not_aborted(abort)?;
    Ok(true)
}

fn same_complex_slice_with_abort(
    left: &[StbComplexSample],
    right: &[StbComplexSample],
    abort: &dyn AbortSignal,
) -> Result<bool, StbDocumentError> {
    if left.len() != right.len() {
        return Ok(false);
    }
    for (index, (left, right)) in left.iter().zip(right).enumerate() {
        poll_projection_abort(abort, index)?;
        if !same_float(left.real, right.real) || !same_float(left.imaginary, right.imaginary) {
            return Ok(false);
        }
    }
    ensure_projection_not_aborted(abort)?;
    Ok(true)
}

fn complex_component_matches_with_abort(
    gains: &[StbComplexSample],
    values: &[f64],
    abort: &dyn AbortSignal,
    component: impl Fn(&StbComplexSample) -> f64,
) -> Result<bool, StbDocumentError> {
    if gains.len() != values.len() {
        return Ok(false);
    }
    for (index, (gain, value)) in gains.iter().zip(values).enumerate() {
        poll_projection_abort(abort, index)?;
        if !same_float(component(gain), *value) {
            return Ok(false);
        }
    }
    ensure_projection_not_aborted(abort)?;
    Ok(true)
}

mod float_wire {
    use super::*;

    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if value.is_finite() {
            serializer.serialize_f64(*value)
        } else if value.is_nan() {
            serializer.serialize_str("NaN")
        } else if value.is_sign_positive() {
            serializer.serialize_str("Infinity")
        } else {
            serializer.serialize_str("-Infinity")
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FloatVisitor)
    }

    pub(super) struct FloatVisitor;

    impl Visitor<'_> for FloatVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a number or NaN/Infinity/-Infinity")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value as f64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                "NaN" => Ok(f64::NAN),
                "Infinity" => Ok(f64::INFINITY),
                "-Infinity" => Ok(f64::NEG_INFINITY),
                _ => Err(E::unknown_variant(value, &["NaN", "Infinity", "-Infinity"])),
            }
        }
    }
}

mod float_vec_wire {
    use super::*;

    pub fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            sequence.serialize_element(&FloatRef(*value))?;
        }
        sequence.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<FloatValue>::deserialize(deserializer)
            .map(|values| values.into_iter().map(|value| value.0).collect())
    }

    struct FloatRef(f64);

    impl Serialize for FloatRef {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            float_wire::serialize(&self.0, serializer)
        }
    }

    struct FloatValue(f64);

    impl<'de> Deserialize<'de> for FloatValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            float_wire::deserialize(deserializer).map(Self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_core_result(point_count: usize) -> StbAnalysisResult {
        use rspice_core::analysis::{BodePoint, NyquistPoint, StabilityMargins, StbResult};

        let frequencies = (0..point_count)
            .map(|index| index as f64 + 1.0)
            .collect::<Vec<_>>();
        let loop_gains = (0..point_count)
            .map(|index| rspice_core::Complex64::new(2.0, -(index as f64) * 0.001))
            .collect::<Vec<_>>();
        let bode_points = frequencies
            .iter()
            .copied()
            .zip(loop_gains.iter().copied())
            .map(|(frequency, gain)| BodePoint::from_loop_gain(frequency, gain))
            .collect();
        let nyquist_points = frequencies
            .iter()
            .copied()
            .zip(loop_gains.iter().copied())
            .map(|(frequency, gain)| NyquistPoint::from_loop_gain(gain, frequency))
            .collect();
        StbAnalysisResult {
            frequencies,
            loop_gains,
            result: StbResult {
                bode_points,
                nyquist_points,
                margins: StabilityMargins::default(),
                success: true,
                warnings: Vec::new(),
            },
            probe_name: "VPROBE".to_owned(),
        }
    }

    fn synthetic_document(nyquist: bool) -> StbResultDocument {
        let gains = vec![
            StbComplexSample {
                real: 10.0,
                imaginary: -1.0,
            },
            StbComplexSample {
                real: 0.5,
                imaginary: -0.5,
            },
        ];
        let first_bode = rspice_core::analysis::BodePoint::from_loop_gain(
            1.0,
            rspice_core::Complex64::new(gains[0].real, gains[0].imaginary),
        );
        let second_bode = rspice_core::analysis::BodePoint::from_loop_gain(
            10.0,
            rspice_core::Complex64::new(gains[1].real, gains[1].imaginary),
        );
        StbResultDocument {
            schema: STB_RESULT_SCHEMA.to_owned(),
            schema_version: STB_RESULT_VERSION,
            analysis: StbAnalysisIdentity {
                id: "stb-002".to_owned(),
                kind: StbAnalysisKind::Stability,
                request_kind: "stb".to_owned(),
                ordinal: 2,
            },
            coordinate_id: None,
            point_count: 2,
            probe_name: "VPROBE".to_owned(),
            primary: StbPrimarySeries {
                frequency_unit: StbUnit::Hertz,
                loop_gain_unit: StbUnit::Dimensionless,
                frequencies: vec![1.0, 10.0],
                loop_gains: gains.clone(),
            },
            bode: StbBodeSeries {
                frequency_unit: StbUnit::Hertz,
                magnitude_unit: StbUnit::Dimensionless,
                magnitude_db_unit: StbUnit::Decibel,
                phase_unit: StbUnit::Degree,
                loop_gain_unit: StbUnit::Dimensionless,
                frequencies: vec![1.0, 10.0],
                magnitudes: vec![first_bode.magnitude, second_bode.magnitude],
                magnitudes_db: vec![first_bode.magnitude_db, second_bode.magnitude_db],
                phase_degrees: vec![first_bode.phase_deg, second_bode.phase_deg],
                loop_gains: gains,
            },
            nyquist: nyquist.then(|| StbNyquistSeries {
                real_unit: StbUnit::Dimensionless,
                imaginary_unit: StbUnit::Dimensionless,
                frequency_unit: StbUnit::Hertz,
                real: vec![10.0, 0.5],
                imaginary: vec![-1.0, -0.5],
                frequencies: vec![1.0, 10.0],
            }),
            margins: StbMargins {
                gain_margin_db: f64::INFINITY,
                gain_margin_frequency: 0.0,
                phase_margin_degrees: f64::NEG_INFINITY,
                phase_margin_frequency: 10.0,
                dc_gain_db: f64::NAN,
                unity_gain_bandwidth: 10.0,
                conditionally_stable: false,
                num_crossovers: 1,
                is_stable: false,
                units: StbMarginUnits::default(),
            },
            success: true,
            warnings: vec!["synthetic warning".to_owned()],
            assessment: "UNSTABLE".to_owned(),
        }
    }

    #[test]
    fn json_round_trip_preserves_full_stb_records_and_extended_values() {
        let document = synthetic_document(true);
        document
            .validate()
            .expect("synthetic STB document validates");
        assert_eq!(document.retained_numeric_value_count().unwrap(), 30);
        let encoded = serde_json::to_string(&document).expect("STB document serializes");
        assert!(encoded.contains("\"Infinity\""));
        assert!(encoded.contains("\"NaN\""));
        let decoded: StbResultDocument =
            serde_json::from_str(&encoded).expect("STB document deserializes");
        decoded
            .validate()
            .expect("round-tripped STB document validates");
        assert!(decoded.margins.gain_margin_db.is_infinite());
        assert!(decoded.margins.dc_gain_db.is_nan());
        assert_eq!(decoded.primary, document.primary);
        assert_eq!(decoded.nyquist, document.nyquist);
    }

    #[test]
    fn bounded_windows_charge_exact_primary_bode_and_optional_nyquist_columns() {
        let without_nyquist = synthetic_document(false);
        assert_eq!(without_nyquist.retained_numeric_value_count().unwrap(), 24);
        assert!(without_nyquist.window(0, 2, 18).is_ok());
        assert!(without_nyquist.window(0, 2, 17).is_err());

        let with_nyquist = synthetic_document(true);
        assert!(with_nyquist.window(0, 2, 24).is_ok());
        assert!(with_nyquist.window(0, 2, 23).is_err());
        assert!(with_nyquist.window(2, 1, 12).is_err());
    }

    #[test]
    fn forward_versions_and_misaligned_derived_records_fail_closed() {
        let mut document = synthetic_document(true);
        document.schema_version += 1;
        assert!(document.validate().unwrap_err().contains("unsupported"));

        let mut document = synthetic_document(true);
        document.bode.frequencies[1] = 11.0;
        assert!(document.validate().unwrap_err().contains("not aligned"));

        let mut document = synthetic_document(true);
        document.bode.magnitudes[0] += 1.0;
        assert!(document.validate().unwrap_err().contains("contradict"));

        let mut document = synthetic_document(true);
        document.margins.is_stable = true;
        assert!(document.validate().unwrap_err().contains("contradict"));
    }

    #[test]
    fn malformed_public_documents_reject_windows_without_slicing_panics() {
        let mut document = synthetic_document(true);
        document.bode.phase_degrees.pop();
        let error = document
            .window(0, 1, 12)
            .expect_err("a malformed public document must fail before slicing");
        assert!(matches!(error, StbDocumentError::Invalid(_)));
        assert!(error.to_string().contains("point count"));
    }

    #[test]
    fn retained_and_window_counts_accept_the_exact_usize_boundary_and_reject_overflow() {
        let largest_retained_point_count = (usize::MAX - 6) / 12;
        let retained =
            checked_retained_numeric_value_count(largest_retained_point_count, true).unwrap();
        assert_eq!(retained, largest_retained_point_count * 12 + 6);
        assert!(
            checked_retained_numeric_value_count(largest_retained_point_count + 1, true).is_err()
        );

        let largest_window_count = usize::MAX / 12;
        assert_eq!(
            checked_window_value_count(largest_window_count, true).unwrap(),
            largest_window_count * 12
        );
        assert!(checked_window_value_count(largest_window_count + 1, true).is_err());

        let document = synthetic_document(true);
        assert!(document.window(usize::MAX, 1, usize::MAX).is_err());
    }

    #[test]
    fn result_projection_observes_abort_deterministically_mid_copy() {
        let abort = rspice_core::abort_signal::CountingAbort::new(3);
        let error = stb_document_with_abort(synthetic_core_result(1_024), 1, &abort)
            .expect_err("counted abort must stop STB SoA projection");
        assert_eq!(error, StbDocumentError::Aborted);
        assert_eq!(
            abort.count(),
            4,
            "projection must stop on the first true stride poll"
        );
    }
}
