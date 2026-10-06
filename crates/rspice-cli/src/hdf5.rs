//! HDF5 storage for simulation results.
//!
//! The layout — path-safe dataset names, so an exported file stays robust when
//! a signal name carries SPICE syntax — belongs to [`rspice_core::io::hdf5`],
//! which is also what the GUI writes and reads. This module maps the CLI's
//! result types onto that document and adds the two section families only a
//! command-line run produces: a `.DISTO` series and an `.FFT` result with its
//! metrics. Reading stays here too: the CLI reads back everything it writes,
//! including those two.

use crate::commands::publish;
use rspice_core::io::{
    Hdf5Attribute, Hdf5Column, Hdf5Coordinate, Hdf5Document, Hdf5Group, Hdf5Table,
};
use rspice_output::AtomicArtifactError;
use rustyhdf5::{AttrValue, File as Hdf5File};
use thiserror::Error;

use std::collections::HashMap;
use std::path::Path;

mod admission;
mod fft_schema;
pub(crate) use fft_schema::*;

/// The document version this build reads. The writer's half of the same
/// number lives with the layout, so a reader and a writer cannot drift.
const SCHEMA_VERSION: &str = rspice_core::io::HDF5_SCHEMA_VERSION;
const FFT_SECTION_SCHEMA_VERSION: &str = "3";

#[derive(Debug, Error)]
pub enum Hdf5Error {
    #[error(transparent)]
    ResourceLimit(#[from] rspice_core::ResourceLimitError),
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error("invalid HDF5 schema: {0}")]
    InvalidSchema(String),
    #[error("failed while writing staged HDF5 artifact: {0}")]
    ArtifactWrite(#[source] std::io::Error),
    #[error(transparent)]
    Publication(#[from] rspice_core::OutputCommitError),
}

#[derive(Debug, Error)]
enum Hdf5StagingError {
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error(transparent)]
    Core(#[from] rspice_core::io::Hdf5Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// The core writer's refusals, in this module's vocabulary.
///
/// A backend failure and a failed write keep the variants they already had, so
/// a caller that matched on them before the layout moved still matches. Every
/// other refusal is the document contradicting the layout, which is what
/// `InvalidSchema` says.
impl From<rspice_core::io::Hdf5Error> for Hdf5Error {
    fn from(error: rspice_core::io::Hdf5Error) -> Self {
        match error {
            rspice_core::io::Hdf5Error::Backend(error) => Self::Backend(error),
            rspice_core::io::Hdf5Error::Write(error) => Self::ArtifactWrite(error),
            other => Self::InvalidSchema(other.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Hdf5Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Signal {
    pub name: String,
    pub var_type: String,
    /// Unit symbol, or `None` when nothing stated one.
    ///
    /// `None` is not dimensionless: the layout writes `signal_NNNN_unit` only
    /// for a signal whose producer named a quantity, so a reader can tell a
    /// pure ratio from a column nobody declared a unit for.
    pub unit: Option<String>,
    pub values: Vec<f64>,
}

impl Hdf5Signal {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self::new_typed(name, "value", None, values)
    }

    pub fn new_typed(
        name: impl Into<String>,
        var_type: impl Into<String>,
        unit: Option<String>,
        values: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            var_type: var_type.into(),
            unit,
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5WaveformSection {
    pub independent_name: String,
    pub independent_values: Vec<f64>,
    pub signals: Vec<Hdf5Signal>,
}

impl Hdf5WaveformSection {
    pub fn new(independent_name: impl Into<String>, independent_values: Vec<f64>) -> Self {
        Self {
            independent_name: independent_name.into(),
            independent_values,
            signals: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, name: impl Into<String>, values: Vec<f64>) {
        self.signals.push(Hdf5Signal::new(name, values));
    }

    pub fn add_typed_signal(
        &mut self,
        name: impl Into<String>,
        var_type: impl Into<String>,
        unit: Option<String>,
        values: Vec<f64>,
    ) {
        self.signals
            .push(Hdf5Signal::new_typed(name, var_type, unit, values));
    }

    fn validate(&self, section_name: &str) -> Result<()> {
        finite_samples(&self.independent_name, &self.independent_values)?;
        for (index, signal) in self.signals.iter().enumerate() {
            finite_samples(&signal.name, &signal.values)?;
            let paired = if let Some(quantity) = signal.var_type.strip_prefix("complex_real:") {
                signal
                    .name
                    .strip_prefix("Re(")
                    .and_then(|name| name.strip_suffix(')'))
                    .is_some_and(|name| {
                        self.signals.get(index + 1).is_some_and(|imag| {
                            imag.name == format!("Im({name})")
                                && imag.var_type == format!("complex_imag:{quantity}")
                        })
                    })
            } else if let Some(quantity) = signal.var_type.strip_prefix("complex_imag:") {
                signal
                    .name
                    .strip_prefix("Im(")
                    .and_then(|name| name.strip_suffix(')'))
                    .is_some_and(|name| {
                        index
                            .checked_sub(1)
                            .and_then(|index| self.signals.get(index))
                            .is_some_and(|real| {
                                real.name == format!("Re({name})")
                                    && real.var_type == format!("complex_real:{quantity}")
                            })
                    })
            } else {
                true
            };
            if !paired {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "unpaired complex column '{}' in {section_name}",
                    signal.name
                )));
            }
            if signal.values.len() != self.independent_values.len() {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "{section_name} signal '{}' has {} points, expected {}",
                    signal.name,
                    signal.values.len(),
                    self.independent_values.len()
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5ComplexSignal {
    pub name: String,
    /// Unit symbol, or `None` when nothing stated one. See [`Hdf5Signal::unit`].
    pub unit: Option<String>,
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl Hdf5ComplexSignal {
    pub fn new(
        name: impl Into<String>,
        unit: Option<String>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            unit,
            real,
            imag,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5AcSection {
    pub frequency: Vec<f64>,
    pub signals: Vec<Hdf5ComplexSignal>,
}

impl Hdf5AcSection {
    pub fn new(frequency: Vec<f64>) -> Self {
        Self {
            frequency,
            signals: Vec::new(),
        }
    }

    pub fn add_signal(
        &mut self,
        name: impl Into<String>,
        unit: Option<String>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) {
        self.signals
            .push(Hdf5ComplexSignal::new(name, unit, real, imag));
    }

    fn validate(&self) -> Result<()> {
        finite_samples("frequency", &self.frequency)?;
        for signal in &self.signals {
            finite_samples(&signal.name, &signal.real)?;
            finite_samples(&signal.name, &signal.imag)?;
            if signal.real.len() != self.frequency.len() {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "AC signal '{}' real part has {} points, expected {}",
                    signal.name,
                    signal.real.len(),
                    self.frequency.len()
                )));
            }
            if signal.imag.len() != self.frequency.len() {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "AC signal '{}' imaginary part has {} points, expected {}",
                    signal.name,
                    signal.imag.len(),
                    self.frequency.len()
                )));
            }
        }
        Ok(())
    }
}

/// One complex signal within an F1 fundamental, F2 fundamental, or nonlinear
/// product series in a `.DISTO` result.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5DistortionSignal {
    pub name: String,
    pub var_type: String,
    /// Actual sinusoidal peak phasor components, never an internal Volterra
    /// kernel.
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
    pub magnitude: Vec<f64>,
    pub phase_degrees: Vec<f64>,
    /// Magnitude divided by the F1 magnitude for the same signal. F1 itself
    /// has no ratio because it is the normalization reference.
    pub magnitude_ratio_to_f1: Option<Vec<f64>>,
}

/// A stable spectral identity and its response across the swept F1 values.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5DistortionSeries {
    /// `f1`, `f2`, `2f1`, `3f1`, `f1+f2`, `f1-f2`, or `2f1-f2`.
    pub label: String,
    pub is_product: bool,
    /// Physical output frequency for every swept F1 row.
    pub physical_frequency: Vec<f64>,
    pub signals: Vec<Hdf5DistortionSignal>,
}

/// Typed third-order Volterra distortion results.
///
/// This is an additive schema-v1 section. Older readers that ignore unknown
/// root groups remain compatible; updated readers retain product identity,
/// phasor convention, and normalization provenance without pretending the
/// result is an ordinary AC sweep.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5DistortionSection {
    pub mode: String,
    pub f2_over_f1: Option<f64>,
    pub phasor_convention: String,
    pub ratio_normalization: String,
    pub f1_frequency: Vec<f64>,
    pub series: Vec<Hdf5DistortionSeries>,
}

fn finite_samples(name: &str, values: &[f64]) -> Result<()> {
    if let Some(index) = values.iter().position(|value| !value.is_finite()) {
        return Err(Hdf5Error::InvalidSchema(format!(
            "non-finite value in '{name}' at sample {index}"
        )));
    }
    Ok(())
}

impl Hdf5DistortionSection {
    fn validate(&self) -> Result<()> {
        finite_samples("f1_frequency", &self.f1_frequency)?;
        if self.mode != "harmonic" && self.mode != "two_tone" {
            return Err(Hdf5Error::InvalidSchema(format!(
                "distortion mode must be 'harmonic' or 'two_tone', got '{}'",
                self.mode
            )));
        }
        if self.phasor_convention != "actual_sinusoidal_peak" {
            return Err(Hdf5Error::InvalidSchema(format!(
                "unsupported distortion phasor convention '{}'",
                self.phasor_convention
            )));
        }
        if self.ratio_normalization != "magnitude_over_same_signal_f1_magnitude" {
            return Err(Hdf5Error::InvalidSchema(format!(
                "unsupported distortion ratio normalization '{}'",
                self.ratio_normalization
            )));
        }
        match (self.mode.as_str(), self.f2_over_f1) {
            ("harmonic", None) => {}
            ("two_tone", Some(ratio)) if ratio.is_finite() && ratio > 0.0 && ratio < 1.0 => {}
            ("harmonic", Some(ratio)) => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "harmonic distortion section unexpectedly has f2_over_f1={ratio}"
                )));
            }
            ("two_tone", ratio) => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "two-tone distortion section requires 0 < f2_over_f1 < 1, got {ratio:?}"
                )));
            }
            _ => unreachable!("mode was validated above"),
        }
        if self.f1_frequency.is_empty() {
            return Err(Hdf5Error::InvalidSchema(
                "distortion section has no F1 frequencies".to_string(),
            ));
        }

        let count = self.f1_frequency.len();
        let expected_series: &[(&str, bool)] = if self.mode == "two_tone" {
            &[
                ("f1", false),
                ("f2", false),
                ("f1+f2", true),
                ("f1-f2", true),
                ("2f1-f2", true),
            ]
        } else {
            &[("f1", false), ("2f1", true), ("3f1", true)]
        };
        if self.series.len() != expected_series.len() {
            return Err(Hdf5Error::InvalidSchema(format!(
                "{} distortion section has {} spectral series, expected {}",
                self.mode,
                self.series.len(),
                expected_series.len()
            )));
        }

        for (series, &(expected_label, expected_product)) in self.series.iter().zip(expected_series)
        {
            finite_samples(&series.label, &series.physical_frequency)?;
            if series.label != expected_label || series.is_product != expected_product {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "distortion series expected label '{expected_label}' with is_product={expected_product}, got '{}' with is_product={}",
                    series.label, series.is_product
                )));
            }
            if series.physical_frequency.len() != count {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "distortion series '{}' has {} frequencies, expected {count}",
                    series.label,
                    series.physical_frequency.len()
                )));
            }
            if series.label == "f1" && series.physical_frequency != self.f1_frequency {
                return Err(Hdf5Error::InvalidSchema(
                    "distortion F1 series frequency does not match the independent F1 scale"
                        .to_string(),
                ));
            }
            for signal in &series.signals {
                for values in [
                    &signal.real,
                    &signal.imag,
                    &signal.magnitude,
                    &signal.phase_degrees,
                ] {
                    finite_samples(&signal.name, values)?;
                }
                if let Some(ratio) = &signal.magnitude_ratio_to_f1 {
                    finite_samples(&signal.name, ratio)?;
                }
                for (quantity, actual) in [
                    ("real", signal.real.len()),
                    ("imaginary", signal.imag.len()),
                    ("magnitude", signal.magnitude.len()),
                    ("phase", signal.phase_degrees.len()),
                ] {
                    if actual != count {
                        return Err(Hdf5Error::InvalidSchema(format!(
                            "distortion series '{}' signal '{}' {quantity} data has {actual} points, expected {count}",
                            series.label, signal.name
                        )));
                    }
                }
                if let Some(ratio) = &signal.magnitude_ratio_to_f1
                    && ratio.len() != count
                {
                    return Err(Hdf5Error::InvalidSchema(format!(
                        "distortion series '{}' signal '{}' ratio data has {} points, expected {count}",
                        series.label,
                        signal.name,
                        ratio.len()
                    )));
                }
                if (series.label == "f1") != signal.magnitude_ratio_to_f1.is_none() {
                    return Err(Hdf5Error::InvalidSchema(format!(
                        "distortion series '{}' signal '{}' has inconsistent F1 normalization provenance",
                        series.label, signal.name
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Measurement {
    pub name: String,
    pub value: f64,
}

impl Hdf5Measurement {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// Identity one HDF5 document publishes under.
///
/// A `run` artifact names the canonical analysis instance that produced it,
/// and, for an axis coordinate, the coordinate and topology it belongs to. A
/// document `convert` produced from a file that declared none carries none.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5ResultIdentity {
    /// Canonical `AnalysisInstanceId::tag()`, which is also the group name.
    pub analysis_id: String,
    pub coordinate_id: Option<String>,
    pub coordinate_tag: Option<String>,
    pub coordinate_assignment: Option<String>,
    pub topology_fingerprint: Option<String>,
}

/// A general result projection with explicit coordinate and quantity types.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5TableSection {
    pub coordinate_unit: Option<String>,
    pub analysis: String,
    pub coordinate_type: String,
    pub waveform: Hdf5WaveformSection,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5SimulationData {
    pub title: String,
    /// The analysis instance this document publishes. When present, the
    /// section group is named by it instead of by the result family, so two
    /// `.AC` cards cannot collide in one file and a reader can tell which card
    /// a group came from.
    pub identity: Option<Hdf5ResultIdentity>,
    pub operating_point: Option<Hdf5WaveformSection>,
    pub transient: Option<Hdf5WaveformSection>,
    pub dc_sweep: Option<Hdf5WaveformSection>,
    pub noise: Option<Hdf5WaveformSection>,
    pub ac: Option<Hdf5AcSection>,
    pub distortion: Option<Hdf5DistortionSection>,
    pub fft: Option<Hdf5FftSection>,
    pub measurements: Vec<Hdf5Measurement>,
    pub table: Option<Hdf5TableSection>,
}

impl Hdf5SimulationData {
    pub fn new() -> Self {
        Self::default()
    }

    fn validate(&self) -> Result<()> {
        if let Some(table) = &self.table {
            table.waveform.validate("table")?;
        }
        if let Some(operating_point) = &self.operating_point {
            operating_point.validate("operating_point")?;
        }
        if let Some(transient) = &self.transient {
            transient.validate("transient")?;
        }
        if let Some(dc_sweep) = &self.dc_sweep {
            dc_sweep.validate("dc_sweep")?;
        }
        if let Some(noise) = &self.noise {
            noise.validate("noise")?;
        }
        if let Some(ac) = &self.ac {
            ac.validate()?;
        }
        if let Some(distortion) = &self.distortion {
            distortion.validate()?;
        }
        if let Some(fft) = &self.fft {
            fft.validate()?;
        }
        Ok(())
    }
}

pub fn write_hdf5(path: &Path, data: &Hdf5SimulationData) -> Result<()> {
    let document = build_hdf5(data)?;
    write_hdf5_staged(path, |file| {
        rspice_core::io::write_hdf5(file, &document).map_err(Hdf5StagingError::Core)
    })
}

/// Publish a projected table, retaining coordinate/quantity types for every family.
pub(crate) fn write_table(
    path: &Path,
    table: &crate::commands::export_table::ExportTable,
    identity: Option<Hdf5ResultIdentity>,
) -> std::result::Result<(), crate::cli::CliError> {
    let data = table_data(table, identity);
    write_hdf5(path, &data).map_err(|error| map_output_error(path, error))
}

pub(crate) fn table_data(
    table: &crate::commands::export_table::ExportTable,
    identity: Option<Hdf5ResultIdentity>,
) -> Hdf5SimulationData {
    use crate::commands::export_table::ColumnData;
    let mut waveform = Hdf5WaveformSection::new(table.scale_name.clone(), table.scale.clone());
    for column in &table.columns {
        match &column.data {
            ColumnData::Real(values) => waveform.add_typed_signal(
                column.name.clone(),
                column.var_type.clone(),
                column.unit.clone(),
                values.clone(),
            ),
            ColumnData::Complex { real, imag } => {
                waveform.add_typed_signal(
                    format!("Re({})", column.name),
                    format!("complex_real:{}", column.var_type),
                    column.unit.clone(),
                    real.clone(),
                );
                waveform.add_typed_signal(
                    format!("Im({})", column.name),
                    format!("complex_imag:{}", column.var_type),
                    column.unit.clone(),
                    imag.clone(),
                );
            }
        }
    }
    Hdf5SimulationData {
        title: table.plot_name.clone(),
        identity,
        table: Some(Hdf5TableSection {
            coordinate_unit: table.scale_unit.clone(),
            analysis: table.analysis.clone(),
            coordinate_type: table.scale_type.clone(),
            waveform,
        }),
        ..Hdf5SimulationData::default()
    }
}

/// Serialize an HDF5 document into an already prepared artifact. Callers that
/// publish a logical multi-file result use this to finish every sibling before
/// committing any destination.
pub(crate) fn write_hdf5_to_writer(
    writer: &mut dyn std::io::Write,
    data: &Hdf5SimulationData,
) -> Result<()> {
    let document = build_hdf5(data)?;
    rspice_core::io::write_hdf5(writer, &document).map_err(Hdf5Error::from)
}

fn build_hdf5(data: &Hdf5SimulationData) -> Result<Hdf5Document> {
    data.validate()?;

    let mut document = Hdf5Document::new(data.title.clone());
    // The identity travels on the root as well as in the group name, so a
    // reader that walks groups by their `section_type` still learns which
    // analysis instance and coordinate produced them.
    if let Some(identity) = &data.identity {
        document.set_attr(
            "analysis_id",
            Hdf5Attribute::Text(identity.analysis_id.clone()),
        );
        for (name, value) in [
            ("coordinate_id", identity.coordinate_id.as_ref()),
            ("coordinate_tag", identity.coordinate_tag.as_ref()),
            (
                "coordinate_assignment",
                identity.coordinate_assignment.as_ref(),
            ),
            (
                "topology_fingerprint",
                identity.topology_fingerprint.as_ref(),
            ),
        ] {
            if let Some(value) = value {
                document.set_attr(name, Hdf5Attribute::Text(value.clone()));
            }
        }
    }
    // One document carries one analysis, so the identity names its single
    // section group. A converted document with no identity keeps the family
    // name it was decoded under.
    let section_name = |family: &'static str| {
        data.identity.as_ref().map_or_else(
            || family.to_string(),
            |identity| identity.analysis_id.clone(),
        )
    };

    if let Some(operating_point) = &data.operating_point {
        add_waveform_section(
            &mut document,
            &section_name("operating_point"),
            "operating_point",
            operating_point,
        )?;
    }
    if let Some(transient) = &data.transient {
        add_waveform_section(
            &mut document,
            &section_name("transient"),
            "transient",
            transient,
        )?;
    }
    if let Some(dc_sweep) = &data.dc_sweep {
        add_waveform_section(
            &mut document,
            &section_name("dc_sweep"),
            "dc_sweep",
            dc_sweep,
        )?;
    }
    if let Some(noise) = &data.noise {
        add_waveform_section(&mut document, &section_name("noise"), "noise", noise)?;
    }
    if let Some(ac) = &data.ac {
        add_ac_section(&mut document, &section_name("ac"), ac)?;
    }
    if let Some(distortion) = &data.distortion {
        add_distortion_section(&mut document, &section_name("distortion"), distortion)?;
    }
    if let Some(fft) = &data.fft {
        add_fft_section(&mut document, &section_name("fft"), fft)?;
    }
    if let Some(table) = &data.table {
        let name = section_name("table");
        add_waveform_section(&mut document, &name, "table", &table.waveform)?;
        if let Some(group) = document.groups.last_mut() {
            if let Some(unit) = &table.coordinate_unit {
                group.set_attr("coordinate_unit", Hdf5Attribute::Text(unit.clone()));
            }
            group.set_attr("analysis", Hdf5Attribute::Text(table.analysis.clone()));
            group.set_attr(
                "coordinate_type",
                Hdf5Attribute::Text(table.coordinate_type.clone()),
            );
        }
    }
    if !data.measurements.is_empty() {
        add_measurements(&mut document, &data.measurements)?;
    }

    Ok(document)
}

/// Publish HDF5 bytes atomically: the destination this replaces survives
/// intact when serialization fails, because nothing is committed until the
/// whole document has been encoded.
fn write_hdf5_staged(
    path: &Path,
    write: impl FnOnce(&mut dyn std::io::Write) -> std::result::Result<(), Hdf5StagingError>,
) -> Result<()> {
    publish::artifact(path, write).map_err(|error| match error {
        AtomicArtifactError::Write(Hdf5StagingError::Backend(error)) => Hdf5Error::Backend(error),
        AtomicArtifactError::Write(Hdf5StagingError::Core(error)) => Hdf5Error::from(error),
        AtomicArtifactError::Write(Hdf5StagingError::Io(error)) => Hdf5Error::ArtifactWrite(error),
        error => Hdf5Error::Publication(rspice_core::OutputCommitError::from_atomic(path, &error)),
    })
}

#[cfg(test)]
pub(crate) fn read_hdf5(path: &Path) -> Result<Hdf5SimulationData> {
    read_hdf5_with_limits(path, rspice_core::ResourceLimits::default())
}

/// Read the legacy family view, rejecting repeated families instead of replacing them.
#[cfg(test)]
pub fn read_hdf5_with_limits(
    path: &Path,
    limits: rspice_core::ResourceLimits,
) -> Result<Hdf5SimulationData> {
    let Hdf5Readback {
        metadata: mut data,
        sections,
    } = read_hdf5_sections_with_limits(path, limits)?;
    for (name, section) in sections {
        macro_rules! merge {
            ($field:ident) => {
                if let Some(value) = section.$field {
                    if data.$field.replace(value).is_some() {
                        return Err(Hdf5Error::InvalidSchema(format!(
                            "repeated {} section '{name}'; select an explicit result section",
                            stringify!($field)
                        )));
                    }
                }
            };
        }
        merge!(operating_point);
        merge!(transient);
        merge!(dc_sweep);
        merge!(noise);
        merge!(ac);
        merge!(distortion);
        merge!(fft);
        merge!(table);
    }
    Ok(data)
}

pub struct Hdf5Readback {
    pub metadata: Hdf5SimulationData,
    pub sections: Vec<(String, Hdf5SimulationData)>,
}

pub fn read_hdf5_sections_with_limits(
    path: &Path,
    limits: rspice_core::ResourceLimits,
) -> Result<Hdf5Readback> {
    let bytes = std::fs::metadata(path).map_err(rustyhdf5::Error::Io)?.len();
    admission::admit(
        rspice_core::ResourceKind::ExternalDataBytes,
        usize::try_from(bytes).unwrap_or(usize::MAX),
        limits.max_external_data_bytes,
    )?;
    let file = Hdf5File::open(path)?;
    let root = file.root();
    let root_attrs = root.attrs()?;
    let schema_version = read_required_string_attr(&root_attrs, "schema_version")?;
    if schema_version != SCHEMA_VERSION {
        return Err(Hdf5Error::InvalidSchema(format!(
            "unsupported HDF5 schema version '{schema_version}', expected '{SCHEMA_VERSION}'"
        )));
    }
    admission::validate(&file, limits)?;

    let title = read_string_attr(&root_attrs, "title")?.unwrap_or_default();
    let identity = read_string_attr(&root_attrs, "analysis_id")?.map(|analysis_id| {
        Ok::<_, Hdf5Error>(Hdf5ResultIdentity {
            analysis_id,
            coordinate_id: read_string_attr(&root_attrs, "coordinate_id")?,
            coordinate_tag: read_string_attr(&root_attrs, "coordinate_tag")?,
            coordinate_assignment: read_string_attr(&root_attrs, "coordinate_assignment")?,
            topology_fingerprint: read_string_attr(&root_attrs, "topology_fingerprint")?,
        })
    });
    let identity = identity.transpose()?;
    let root_groups = root.groups()?;

    // A section group is named by the analysis instance that produced it, so
    // the family it belongs to is read from its own `section_type` attribute
    // rather than guessed from the group name.
    let mut data = Hdf5SimulationData {
        title,
        identity,
        ..Hdf5SimulationData::default()
    };
    let mut sections = Vec::new();
    for group_name in &root_groups {
        if group_name == "measurements" {
            data.measurements = read_measurements(&file)?;
            continue;
        }
        let family = read_required_string_attr(&file.group(group_name)?.attrs()?, "section_type")
            .map_err(|_| {
            Hdf5Error::InvalidSchema(format!(
                "group '{group_name}' declares no section_type, so its result family is unknown"
            ))
        })?;
        let mut section = Hdf5SimulationData::default();
        match family.as_str() {
            "operating_point" => {
                section.operating_point = Some(read_waveform_section(&file, group_name)?);
            }
            "transient" => section.transient = Some(read_waveform_section(&file, group_name)?),
            "dc_sweep" => section.dc_sweep = Some(read_waveform_section(&file, group_name)?),
            "noise" => section.noise = Some(read_waveform_section(&file, group_name)?),
            "ac" => section.ac = Some(read_ac_section(&file, group_name)?),
            "distortion" => section.distortion = Some(read_distortion_section(&file, group_name)?),
            "fft" => section.fft = Some(read_fft_section(&file, group_name)?),
            "table" => {
                section.table = Some(Hdf5TableSection {
                    coordinate_unit: read_string_attr(
                        &file.group(group_name)?.attrs()?,
                        "coordinate_unit",
                    )?,
                    analysis: read_required_string_attr(
                        &file.group(group_name)?.attrs()?,
                        "analysis",
                    )?,
                    coordinate_type: read_required_string_attr(
                        &file.group(group_name)?.attrs()?,
                        "coordinate_type",
                    )?,
                    waveform: read_waveform_section(&file, group_name)?,
                })
            }
            other => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "group '{group_name}' declares unknown section_type '{other}'"
                )));
            }
        }
        sections.push((group_name.clone(), section));
    }

    Ok(Hdf5Readback {
        metadata: data,
        sections,
    })
}

fn add_waveform_section(
    document: &mut Hdf5Document,
    name: &str,
    family: &str,
    section: &Hdf5WaveformSection,
) -> Result<()> {
    document.add_table(&Hdf5Table {
        group: name.to_string(),
        section_type: family.to_string(),
        coordinate: Hdf5Coordinate::Independent {
            name: section.independent_name.clone(),
            values: section.independent_values.clone(),
        },
        columns: section
            .signals
            .iter()
            .map(|signal| Hdf5Column::Real {
                name: signal.name.clone(),
                quantity: signal.var_type.clone(),
                unit: signal.unit.clone(),
                values: signal.values.clone(),
            })
            .collect(),
    })?;
    Ok(())
}

fn read_waveform_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5WaveformSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;

    let independent_name = read_required_string_attr(&attrs, "independent_name")?;
    let signal_count = non_negative_count(
        read_required_i64_attr(&attrs, "signal_count")?,
        "signal_count",
    )?;
    let independent_values = group.dataset("independent")?.read_f64()?;

    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let dataset_name = format!("signal_{index:04}");
        let signal_name = read_required_string_attr(&attrs, &format!("{dataset_name}_name"))?;
        let signal_type = read_string_attr(&attrs, &format!("{dataset_name}_type"))?
            .unwrap_or_else(|| "value".to_string());
        let unit = stated_unit(&attrs, &dataset_name)?;
        let values = group.dataset(&dataset_name)?.read_f64()?;
        signals.push(Hdf5Signal::new_typed(
            signal_name,
            signal_type,
            unit,
            values,
        ));
    }

    let section = Hdf5WaveformSection {
        independent_name,
        independent_values,
        signals,
    };
    section.validate(group_name)?;
    Ok(section)
}

fn add_ac_section(document: &mut Hdf5Document, name: &str, section: &Hdf5AcSection) -> Result<()> {
    document.add_table(&Hdf5Table {
        group: name.to_string(),
        section_type: "ac".to_string(),
        coordinate: Hdf5Coordinate::Frequency(section.frequency.clone()),
        columns: section
            .signals
            .iter()
            .map(|signal| Hdf5Column::Complex {
                name: signal.name.clone(),
                unit: signal.unit.clone(),
                real: signal.real.clone(),
                imag: signal.imag.clone(),
            })
            .collect(),
    })?;
    Ok(())
}

fn read_ac_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5AcSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;
    let signal_count = non_negative_count(
        read_required_i64_attr(&attrs, "signal_count")?,
        "signal_count",
    )?;
    let frequency = group.dataset("frequency")?.read_f64()?;

    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let prefix = format!("signal_{index:04}");
        let name = read_required_string_attr(&attrs, &format!("{prefix}_name"))?;
        let unit = stated_unit(&attrs, &prefix)?;
        let real = group.dataset(&format!("{prefix}_real"))?.read_f64()?;
        let imag = group.dataset(&format!("{prefix}_imag"))?.read_f64()?;
        signals.push(Hdf5ComplexSignal::new(name, unit, real, imag));
    }

    let section = Hdf5AcSection { frequency, signals };
    section.validate()?;
    Ok(section)
}

fn add_distortion_section(
    document: &mut Hdf5Document,
    name: &str,
    section: &Hdf5DistortionSection,
) -> Result<()> {
    let mut group = Hdf5Group::new(name);
    group.set_attr(
        "section_type",
        Hdf5Attribute::Text("distortion".to_string()),
    );
    group.set_attr("mode", Hdf5Attribute::Text(section.mode.clone()));
    group.set_attr(
        "phasor_convention",
        Hdf5Attribute::Text(section.phasor_convention.clone()),
    );
    group.set_attr(
        "ratio_normalization",
        Hdf5Attribute::Text(section.ratio_normalization.clone()),
    );
    if let Some(ratio) = section.f2_over_f1 {
        group.set_attr("f2_over_f1", Hdf5Attribute::Real(ratio));
    }
    group.set_attr(
        "series_count",
        Hdf5Attribute::Integer(section.series.len() as i64),
    );
    group.set_real_dataset("f1_frequency", &section.f1_frequency);

    for (series_index, series) in section.series.iter().enumerate() {
        let series_prefix = format!("series_{series_index:04}");
        group.set_attr(
            &format!("{series_prefix}_label"),
            Hdf5Attribute::Text(series.label.clone()),
        );
        group.set_attr(
            &format!("{series_prefix}_is_product"),
            Hdf5Attribute::Integer(i64::from(series.is_product)),
        );
        group.set_attr(
            &format!("{series_prefix}_signal_count"),
            Hdf5Attribute::Integer(series.signals.len() as i64),
        );
        group.set_real_dataset(
            &format!("{series_prefix}_frequency"),
            &series.physical_frequency,
        );

        for (signal_index, signal) in series.signals.iter().enumerate() {
            let signal_prefix = format!("{series_prefix}_signal_{signal_index:04}");
            group.set_attr(
                &format!("{signal_prefix}_name"),
                Hdf5Attribute::Text(signal.name.clone()),
            );
            group.set_attr(
                &format!("{signal_prefix}_type"),
                Hdf5Attribute::Text(signal.var_type.clone()),
            );
            group.set_attr(
                &format!("{signal_prefix}_has_ratio"),
                Hdf5Attribute::Integer(i64::from(signal.magnitude_ratio_to_f1.is_some())),
            );
            group.set_real_dataset(&format!("{signal_prefix}_real"), &signal.real);
            group.set_real_dataset(&format!("{signal_prefix}_imag"), &signal.imag);
            group.set_real_dataset(&format!("{signal_prefix}_magnitude"), &signal.magnitude);
            group.set_real_dataset(
                &format!("{signal_prefix}_phase_degrees"),
                &signal.phase_degrees,
            );
            if let Some(ratio) = &signal.magnitude_ratio_to_f1 {
                group.set_real_dataset(&format!("{signal_prefix}_magnitude_ratio_to_f1"), ratio);
            }
        }
    }

    document.groups.push(group);
    Ok(())
}

fn read_distortion_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5DistortionSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;
    let mode = read_required_string_attr(&attrs, "mode")?;
    let f2_over_f1 = read_f64_attr(&attrs, "f2_over_f1")?;
    let phasor_convention = read_required_string_attr(&attrs, "phasor_convention")?;
    let ratio_normalization = read_required_string_attr(&attrs, "ratio_normalization")?;
    let series_count = non_negative_count(
        read_required_i64_attr(&attrs, "series_count")?,
        "distortion series_count",
    )?;
    let f1_frequency = group.dataset("f1_frequency")?.read_f64()?;
    let mut series = Vec::with_capacity(series_count);

    for series_index in 0..series_count {
        let series_prefix = format!("series_{series_index:04}");
        let label = read_required_string_attr(&attrs, &format!("{series_prefix}_label"))?;
        let is_product = read_binary_flag(&attrs, &format!("{series_prefix}_is_product"))?;
        let signal_count = non_negative_count(
            read_required_i64_attr(&attrs, &format!("{series_prefix}_signal_count"))?,
            &format!("{series_prefix}_signal_count"),
        )?;
        let physical_frequency = group
            .dataset(&format!("{series_prefix}_frequency"))?
            .read_f64()?;
        let mut signals = Vec::with_capacity(signal_count);

        for signal_index in 0..signal_count {
            let signal_prefix = format!("{series_prefix}_signal_{signal_index:04}");
            let name = read_required_string_attr(&attrs, &format!("{signal_prefix}_name"))?;
            let var_type = read_required_string_attr(&attrs, &format!("{signal_prefix}_type"))?;
            let has_ratio = read_binary_flag(&attrs, &format!("{signal_prefix}_has_ratio"))?;
            let real = group
                .dataset(&format!("{signal_prefix}_real"))?
                .read_f64()?;
            let imag = group
                .dataset(&format!("{signal_prefix}_imag"))?
                .read_f64()?;
            let magnitude = group
                .dataset(&format!("{signal_prefix}_magnitude"))?
                .read_f64()?;
            let phase_degrees = group
                .dataset(&format!("{signal_prefix}_phase_degrees"))?
                .read_f64()?;
            let magnitude_ratio_to_f1 = if has_ratio {
                Some(
                    group
                        .dataset(&format!("{signal_prefix}_magnitude_ratio_to_f1"))?
                        .read_f64()?,
                )
            } else {
                None
            };
            signals.push(Hdf5DistortionSignal {
                name,
                var_type,
                real,
                imag,
                magnitude,
                phase_degrees,
                magnitude_ratio_to_f1,
            });
        }

        series.push(Hdf5DistortionSeries {
            label,
            is_product,
            physical_frequency,
            signals,
        });
    }

    let section = Hdf5DistortionSection {
        mode,
        f2_over_f1,
        phasor_convention,
        ratio_normalization,
        f1_frequency,
        series,
    };
    section.validate()?;
    Ok(section)
}

fn checked_i64(value: usize, name: &str) -> Result<i64> {
    i64::try_from(value).map_err(|_| {
        Hdf5Error::InvalidSchema(format!("{name} exceeds the HDF5 signed-integer range"))
    })
}

fn add_fft_section(
    document: &mut Hdf5Document,
    name: &str,
    section: &Hdf5FftSection,
) -> Result<()> {
    let mut group = Hdf5Group::new(name);
    group.set_attr("section_type", Hdf5Attribute::Text("fft".to_string()));
    group.set_attr(
        "schema_version",
        Hdf5Attribute::Text(FFT_SECTION_SCHEMA_VERSION.to_string()),
    );
    group.set_attr(
        "parent_analysis_id",
        Hdf5Attribute::Text(section.parent_analysis_id.clone()),
    );
    group.set_attr(
        "has_coordinate",
        Hdf5Attribute::Integer(i64::from(section.coordinate.is_some())),
    );
    if let Some(coordinate) = &section.coordinate {
        group.set_attr(
            "coordinate_id",
            Hdf5Attribute::Text(coordinate.coordinate_id.clone()),
        );
        group.set_attr(
            "coordinate_ordinal",
            Hdf5Attribute::Integer(checked_i64(coordinate.ordinal, "FFT coordinate ordinal")?),
        );
        group.set_attr(
            "coordinate_tag",
            Hdf5Attribute::Text(coordinate.tag.clone()),
        );
        group.set_attr(
            "coordinate_assignment",
            Hdf5Attribute::Text(coordinate.assignment.clone()),
        );
    }
    group.set_attr(
        "result_count",
        Hdf5Attribute::Integer(checked_i64(section.results.len(), "FFT result count")?),
    );

    for (index, result) in section.results.iter().enumerate() {
        let prefix = format!("result_{index:04}");
        group.set_attr(
            &format!("{prefix}_status"),
            Hdf5Attribute::Text(
                serde_json::to_string(&result.status)
                    .map_err(|error| Hdf5Error::InvalidSchema(error.to_string()))?,
            ),
        );
        for (suffix, value) in [
            ("analysis_id", result.analysis_id.as_str()),
            ("source_kind", result.source_kind.as_str()),
            ("source_text", result.source_text.as_str()),
            ("authored_output", result.authored_output.as_str()),
            ("output_name", result.output_name.as_str()),
            ("physical_type", result.physical_type.as_str()),
            ("format", result.format.as_str()),
            ("mode", result.mode.as_str()),
            ("window", result.window.as_str()),
            ("window_name", result.window_name.as_str()),
        ] {
            group.set_attr(
                &format!("{prefix}_{suffix}"),
                Hdf5Attribute::Text(value.to_string()),
            );
        }
        group.set_attr(
            &format!("{prefix}_has_value_unit"),
            Hdf5Attribute::Integer(i64::from(result.value_unit.is_some())),
        );
        if let Some(unit) = &result.value_unit {
            group.set_attr(
                &format!("{prefix}_value_unit"),
                Hdf5Attribute::Text(unit.clone()),
            );
        }
        for (suffix, value) in [
            ("start_time_s", result.start_time_s),
            ("stop_time_s", result.stop_time_s),
            ("sample_interval_s", result.sample_interval_s),
            ("alpha", result.alpha),
            ("coherent_gain", result.coherent_gain),
            ("frequency_resolution_hz", result.frequency_resolution_hz),
        ] {
            group.set_attr(&format!("{prefix}_{suffix}"), Hdf5Attribute::Real(value));
        }
        for (suffix, value) in [
            ("ordinal", result.ordinal),
            ("point_count", result.point_count),
            ("fundamental_bin", result.fundamental_bin),
            ("minimum_metric_bin", result.minimum_metric_bin),
            ("maximum_metric_bin", result.maximum_metric_bin),
            ("sfdr_search_minimum_bin", result.sfdr_search_minimum_bin),
        ] {
            group.set_attr(
                &format!("{prefix}_{suffix}"),
                Hdf5Attribute::Integer(checked_i64(value, &format!("{prefix}_{suffix}"))?),
            );
        }
        group.set_attr(
            &format!("{prefix}_accurate_sampling"),
            Hdf5Attribute::Integer(i64::from(result.accurate_sampling)),
        );
        group.set_attr(
            &format!("{prefix}_has_metrics"),
            Hdf5Attribute::Integer(i64::from(result.metrics.is_some())),
        );
        let bin_indices = result
            .bin_indices
            .iter()
            .map(|value| {
                i64::try_from(*value).map_err(|_| {
                    Hdf5Error::InvalidSchema(format!(
                        "{prefix} FFT bin index exceeds the HDF5 signed-integer range"
                    ))
                })
            })
            .collect::<Result<Vec<_>>>()?;
        group.set_integer_dataset(&format!("{prefix}_bin_index"), &bin_indices);
        for (suffix, values) in [
            ("frequency_hz", &result.frequency_hz),
            ("real", &result.real),
            ("imaginary", &result.imaginary),
            ("magnitude", &result.magnitude),
            ("phase_degrees", &result.phase_degrees),
        ] {
            group.set_real_dataset(&format!("{prefix}_{suffix}"), values);
        }

        if let Some(metrics) = &result.metrics {
            for (suffix, value) in [
                ("fundamental_magnitude", metrics.fundamental_magnitude),
                ("thd_ratio", metrics.thd_ratio),
                ("thd_db", metrics.thd_db),
                ("sndr_db", metrics.sndr_db),
                ("enob_bits", metrics.enob_bits),
                ("snr_db", metrics.snr_db),
                ("sfdr_db", metrics.sfdr_db),
            ] {
                group.set_attr(
                    &format!("{prefix}_metrics_{suffix}"),
                    Hdf5Attribute::Real(value),
                );
            }
            group.set_attr(
                &format!("{prefix}_metrics_has_spur"),
                Hdf5Attribute::Integer(i64::from(metrics.sfdr_spur_bin.is_some())),
            );
            if let (Some(bin), Some(frequency)) =
                (metrics.sfdr_spur_bin, metrics.sfdr_spur_frequency_hz)
            {
                group.set_attr(
                    &format!("{prefix}_metrics_sfdr_spur_bin"),
                    Hdf5Attribute::Integer(checked_i64(bin, "FFT SFDR spur bin")?),
                );
                group.set_attr(
                    &format!("{prefix}_metrics_sfdr_spur_frequency_hz"),
                    Hdf5Attribute::Real(frequency),
                );
            }
            let ranks = metrics
                .largest_harmonics
                .iter()
                .map(|harmonic| checked_i64(harmonic.rank, "FFT harmonic rank"))
                .collect::<Result<Vec<_>>>()?;
            let bins = metrics
                .largest_harmonics
                .iter()
                .map(|harmonic| checked_i64(harmonic.bin, "FFT harmonic bin"))
                .collect::<Result<Vec<_>>>()?;
            group.set_integer_dataset(&format!("{prefix}_metrics_harmonic_rank"), &ranks);
            group.set_integer_dataset(&format!("{prefix}_metrics_harmonic_bin"), &bins);
            for (suffix, values) in [
                (
                    "frequency_hz",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.frequency_hz)
                        .collect::<Vec<_>>(),
                ),
                (
                    "magnitude",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.magnitude)
                        .collect::<Vec<_>>(),
                ),
                (
                    "magnitude_db",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.magnitude_db)
                        .collect::<Vec<_>>(),
                ),
                (
                    "phase_degrees",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.phase_degrees)
                        .collect::<Vec<_>>(),
                ),
            ] {
                group.set_real_dataset(&format!("{prefix}_metrics_harmonic_{suffix}"), &values);
            }
        }
    }

    document.groups.push(group);
    Ok(())
}

fn read_fft_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5FftSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;
    let schema_version = read_required_string_attr(&attrs, "schema_version")?;
    if schema_version != FFT_SECTION_SCHEMA_VERSION {
        return Err(Hdf5Error::InvalidSchema(format!(
            "unsupported FFT HDF5 schema version '{schema_version}', expected '{FFT_SECTION_SCHEMA_VERSION}'"
        )));
    }
    let parent_analysis_id = read_required_string_attr(&attrs, "parent_analysis_id")?;
    let coordinate = if read_binary_flag(&attrs, "has_coordinate")? {
        Some(Hdf5FftCoordinate {
            coordinate_id: read_required_string_attr(&attrs, "coordinate_id")?,
            ordinal: non_negative_count(
                read_required_i64_attr(&attrs, "coordinate_ordinal")?,
                "FFT coordinate ordinal",
            )?,
            tag: read_required_string_attr(&attrs, "coordinate_tag")?,
            assignment: read_required_string_attr(&attrs, "coordinate_assignment")?,
        })
    } else {
        None
    };
    let result_count = non_negative_count(
        read_required_i64_attr(&attrs, "result_count")?,
        "FFT result_count",
    )?;
    let mut results = Vec::with_capacity(result_count);

    for index in 0..result_count {
        let prefix = format!("result_{index:04}");
        let metrics = if read_binary_flag(&attrs, &format!("{prefix}_has_metrics"))? {
            let has_spur = read_binary_flag(&attrs, &format!("{prefix}_metrics_has_spur"))?;
            let ranks = group
                .dataset(&format!("{prefix}_metrics_harmonic_rank"))?
                .read_i64()?;
            let bins = group
                .dataset(&format!("{prefix}_metrics_harmonic_bin"))?
                .read_i64()?;
            let frequencies = group
                .dataset(&format!("{prefix}_metrics_harmonic_frequency_hz"))?
                .read_f64()?;
            let magnitudes = group
                .dataset(&format!("{prefix}_metrics_harmonic_magnitude"))?
                .read_f64()?;
            let magnitudes_db = group
                .dataset(&format!("{prefix}_metrics_harmonic_magnitude_db"))?
                .read_f64()?;
            let phases = group
                .dataset(&format!("{prefix}_metrics_harmonic_phase_degrees"))?
                .read_f64()?;
            let harmonic_count = ranks.len();
            if [
                bins.len(),
                frequencies.len(),
                magnitudes.len(),
                magnitudes_db.len(),
                phases.len(),
            ]
            .iter()
            .any(|count| *count != harmonic_count)
            {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "{prefix} FFT harmonic datasets have inconsistent lengths"
                )));
            }
            let mut largest_harmonics = Vec::with_capacity(harmonic_count);
            for harmonic_index in 0..harmonic_count {
                largest_harmonics.push(Hdf5FftHarmonic {
                    rank: non_negative_count(
                        ranks[harmonic_index],
                        &format!("{prefix} FFT harmonic rank"),
                    )?,
                    bin: non_negative_count(
                        bins[harmonic_index],
                        &format!("{prefix} FFT harmonic bin"),
                    )?,
                    frequency_hz: frequencies[harmonic_index],
                    magnitude: magnitudes[harmonic_index],
                    magnitude_db: magnitudes_db[harmonic_index],
                    phase_degrees: phases[harmonic_index],
                });
            }
            Some(Hdf5FftMetrics {
                fundamental_magnitude: read_required_f64_attr(
                    &attrs,
                    &format!("{prefix}_metrics_fundamental_magnitude"),
                )?,
                thd_ratio: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_thd_ratio"))?,
                thd_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_thd_db"))?,
                sndr_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_sndr_db"))?,
                enob_bits: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_enob_bits"))?,
                snr_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_snr_db"))?,
                sfdr_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_sfdr_db"))?,
                sfdr_spur_bin: has_spur
                    .then(|| {
                        non_negative_count(
                            read_required_i64_attr(
                                &attrs,
                                &format!("{prefix}_metrics_sfdr_spur_bin"),
                            )?,
                            "FFT SFDR spur bin",
                        )
                    })
                    .transpose()?,
                sfdr_spur_frequency_hz: has_spur
                    .then(|| {
                        read_required_f64_attr(
                            &attrs,
                            &format!("{prefix}_metrics_sfdr_spur_frequency_hz"),
                        )
                    })
                    .transpose()?,
                largest_harmonics,
            })
        } else {
            None
        };
        let has_value_unit = read_binary_flag(&attrs, &format!("{prefix}_has_value_unit"))?;
        results.push(Hdf5FftResult {
            status: serde_json::from_str(&read_required_string_attr(
                &attrs,
                &format!("{prefix}_status"),
            )?)
            .map_err(|error| Hdf5Error::InvalidSchema(format!("invalid FFT status: {error}")))?,
            analysis_id: read_required_string_attr(&attrs, &format!("{prefix}_analysis_id"))?,
            ordinal: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_ordinal"))?,
                "FFT ordinal",
            )?,
            source_kind: read_required_string_attr(&attrs, &format!("{prefix}_source_kind"))?,
            source_text: read_required_string_attr(&attrs, &format!("{prefix}_source_text"))?,
            authored_output: read_required_string_attr(
                &attrs,
                &format!("{prefix}_authored_output"),
            )?,
            output_name: read_required_string_attr(&attrs, &format!("{prefix}_output_name"))?,
            physical_type: read_required_string_attr(&attrs, &format!("{prefix}_physical_type"))?,
            value_unit: has_value_unit
                .then(|| read_required_string_attr(&attrs, &format!("{prefix}_value_unit")))
                .transpose()?,
            start_time_s: read_required_f64_attr(&attrs, &format!("{prefix}_start_time_s"))?,
            stop_time_s: read_required_f64_attr(&attrs, &format!("{prefix}_stop_time_s"))?,
            sample_interval_s: read_required_f64_attr(
                &attrs,
                &format!("{prefix}_sample_interval_s"),
            )?,
            point_count: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_point_count"))?,
                "FFT point_count",
            )?,
            accurate_sampling: read_binary_flag(&attrs, &format!("{prefix}_accurate_sampling"))?,
            format: read_required_string_attr(&attrs, &format!("{prefix}_format"))?,
            mode: read_required_string_attr(&attrs, &format!("{prefix}_mode"))?,
            window: read_required_string_attr(&attrs, &format!("{prefix}_window"))?,
            window_name: read_required_string_attr(&attrs, &format!("{prefix}_window_name"))?,
            alpha: read_required_f64_attr(&attrs, &format!("{prefix}_alpha"))?,
            coherent_gain: read_required_f64_attr(&attrs, &format!("{prefix}_coherent_gain"))?,
            frequency_resolution_hz: read_required_f64_attr(
                &attrs,
                &format!("{prefix}_frequency_resolution_hz"),
            )?,
            fundamental_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_fundamental_bin"))?,
                "FFT fundamental_bin",
            )?,
            minimum_metric_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_minimum_metric_bin"))?,
                "FFT minimum_metric_bin",
            )?,
            maximum_metric_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_maximum_metric_bin"))?,
                "FFT maximum_metric_bin",
            )?,
            sfdr_search_minimum_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_sfdr_search_minimum_bin"))?,
                "FFT sfdr_search_minimum_bin",
            )?,
            bin_indices: group
                .dataset(&format!("{prefix}_bin_index"))?
                .read_i64()?
                .into_iter()
                .map(|value| {
                    u64::try_from(value).map_err(|_| {
                        Hdf5Error::InvalidSchema(format!(
                            "{prefix} FFT bin index must be non-negative, got {value}"
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?,
            frequency_hz: group
                .dataset(&format!("{prefix}_frequency_hz"))?
                .read_f64()?,
            real: group.dataset(&format!("{prefix}_real"))?.read_f64()?,
            imaginary: group.dataset(&format!("{prefix}_imaginary"))?.read_f64()?,
            magnitude: group.dataset(&format!("{prefix}_magnitude"))?.read_f64()?,
            phase_degrees: group
                .dataset(&format!("{prefix}_phase_degrees"))?
                .read_f64()?,
            metrics,
        });
    }

    let section = Hdf5FftSection {
        parent_analysis_id,
        coordinate,
        results,
    };
    section.validate()?;
    Ok(section)
}

fn add_measurements(document: &mut Hdf5Document, measurements: &[Hdf5Measurement]) -> Result<()> {
    let mut group = Hdf5Group::new("measurements");
    group.set_attr(
        "measurement_count",
        Hdf5Attribute::Integer(measurements.len() as i64),
    );
    for (index, measurement) in measurements.iter().enumerate() {
        let prefix = format!("measurement_{index:04}");
        group.set_attr(
            &format!("{prefix}_name"),
            Hdf5Attribute::Text(measurement.name.clone()),
        );
        group.set_attr(
            &format!("{prefix}_value"),
            Hdf5Attribute::Real(measurement.value),
        );
    }
    document.groups.push(group);
    Ok(())
}

fn read_measurements(file: &Hdf5File) -> Result<Vec<Hdf5Measurement>> {
    let group = file.group("measurements")?;
    let attrs = group.attrs()?;
    let measurement_count = non_negative_count(
        read_required_i64_attr(&attrs, "measurement_count")?,
        "measurement_count",
    )?;

    let mut measurements = Vec::with_capacity(measurement_count);
    for index in 0..measurement_count {
        let prefix = format!("measurement_{index:04}");
        let name = read_required_string_attr(&attrs, &format!("{prefix}_name"))?;
        let value = read_required_f64_attr(&attrs, &format!("{prefix}_value"))?;
        measurements.push(Hdf5Measurement::new(name, value));
    }
    Ok(measurements)
}

fn read_string_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<Option<String>> {
    match attrs.get(name) {
        None => Ok(None),
        Some(AttrValue::String(value)) => Ok(Some(value.clone())),
        // Older RSpice writers encoded empty scalar strings with width zero;
        // the backend exposes those as an empty string array on readback.
        Some(AttrValue::StringArray(values)) if values.is_empty() => Ok(Some(String::new())),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected string, found {other:?}"
        ))),
    }
}

/// The unit one signal column states, if it states one.
///
/// An absent or empty `signal_NNNN_unit` both mean *unstated*, and neither is
/// a schema error: the layout writes the attribute only for a producer that
/// named a quantity, and a file written before this build states none at all.
/// Reading an empty string back as `Some("")` would turn "nobody said" into a
/// unit whose symbol is nothing.
fn stated_unit(attrs: &HashMap<String, AttrValue>, prefix: &str) -> Result<Option<String>> {
    Ok(read_string_attr(attrs, &format!("{prefix}_unit"))?.filter(|unit| !unit.is_empty()))
}

fn read_required_string_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<String> {
    read_string_attr(attrs, name)?.ok_or_else(|| {
        Hdf5Error::InvalidSchema(format!("missing required string attribute '{name}'"))
    })
}

fn read_required_i64_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<i64> {
    match attrs.get(name) {
        Some(AttrValue::I64(value)) => Ok(*value),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected i64, found {other:?}"
        ))),
        None => Err(Hdf5Error::InvalidSchema(format!(
            "missing required integer attribute '{name}'"
        ))),
    }
}

fn read_f64_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<Option<f64>> {
    match attrs.get(name) {
        None => Ok(None),
        Some(AttrValue::F64(value)) => Ok(Some(*value)),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected f64, found {other:?}"
        ))),
    }
}

fn non_negative_count(value: i64, name: &str) -> Result<usize> {
    usize::try_from(value)
        .map_err(|_| Hdf5Error::InvalidSchema(format!("{name} must be non-negative, got {value}")))
}

fn read_binary_flag(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<bool> {
    match read_required_i64_attr(attrs, name)? {
        0 => Ok(false),
        1 => Ok(true),
        value => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' must be 0 or 1, got {value}"
        ))),
    }
}

fn read_required_f64_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<f64> {
    match attrs.get(name) {
        Some(AttrValue::F64(value)) => Ok(*value),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected f64, found {other:?}"
        ))),
        None => Err(Hdf5Error::InvalidSchema(format!(
            "missing required float attribute '{name}'"
        ))),
    }
}

/// Preserve resource admission and publication failures across every HDF5 producer.
pub(crate) fn map_output_error(path: &Path, error: Hdf5Error) -> crate::cli::CliError {
    match error {
        Hdf5Error::ResourceLimit(source) => crate::cli::CliError::ResourceLimit {
            path: path.to_path_buf(),
            source,
        },
        Hdf5Error::Publication(error) => rspice_core::SimulationError::from(error).into(),
        error => crate::cli::CliError::output_error(path, std::io::Error::other(error)),
    }
}

#[cfg(test)]
mod tests;
