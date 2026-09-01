//! HDF5 storage for simulation results.
//!
//! The CLI uses a stable, self-describing layout with path-safe dataset names
//! so exported files remain robust even when signal names contain SPICE syntax.

use crate::atomic_artifact::{AtomicArtifactError, write_atomic};
use rustyhdf5::{AttrValue, File as Hdf5File, FileBuilder};
use thiserror::Error;

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

const SCHEMA_VERSION: &str = "1";

#[derive(Debug, Error)]
pub enum Hdf5Error {
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error("invalid HDF5 schema: {0}")]
    InvalidSchema(String),
    #[error("failed to prepare staged HDF5 artifact: {0}")]
    ArtifactPreparation(#[source] std::io::Error),
    #[error("failed while writing staged HDF5 artifact: {0}")]
    ArtifactWrite(#[source] std::io::Error),
    #[error("failed to flush staged HDF5 artifact: {0}")]
    ArtifactFlush(#[source] std::io::Error),
    #[error("failed to atomically commit staged HDF5 artifact: {0}")]
    ArtifactCommit(#[source] std::io::Error),
}

#[derive(Debug, Error)]
enum Hdf5StagingError {
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Hdf5Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Signal {
    pub name: String,
    pub var_type: String,
    pub values: Vec<f64>,
}

impl Hdf5Signal {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self::new_typed(name, "value", values)
    }

    pub fn new_typed(
        name: impl Into<String>,
        var_type: impl Into<String>,
        values: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            var_type: var_type.into(),
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
        values: Vec<f64>,
    ) {
        self.signals
            .push(Hdf5Signal::new_typed(name, var_type, values));
    }

    fn validate(&self, section_name: &str) -> Result<()> {
        for signal in &self.signals {
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
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl Hdf5ComplexSignal {
    pub fn new(name: impl Into<String>, real: Vec<f64>, imag: Vec<f64>) -> Self {
        Self {
            name: name.into(),
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

    pub fn add_signal(&mut self, name: impl Into<String>, real: Vec<f64>, imag: Vec<f64>) {
        self.signals.push(Hdf5ComplexSignal::new(name, real, imag));
    }

    fn validate(&self) -> Result<()> {
        for signal in &self.signals {
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

impl Hdf5DistortionSection {
    fn validate(&self) -> Result<()> {
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5SimulationData {
    pub title: String,
    pub operating_point: Option<Hdf5WaveformSection>,
    pub transient: Option<Hdf5WaveformSection>,
    pub dc_sweep: Option<Hdf5WaveformSection>,
    pub noise: Option<Hdf5WaveformSection>,
    pub ac: Option<Hdf5AcSection>,
    pub distortion: Option<Hdf5DistortionSection>,
    pub measurements: Vec<Hdf5Measurement>,
}

impl Hdf5SimulationData {
    pub fn new() -> Self {
        Self::default()
    }

    fn validate(&self) -> Result<()> {
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
        Ok(())
    }
}

pub fn write_hdf5(path: &Path, data: &Hdf5SimulationData) -> Result<()> {
    data.validate()?;

    let mut builder = FileBuilder::new();
    builder.set_attr(
        "schema_version",
        AttrValue::String(SCHEMA_VERSION.to_string()),
    );
    builder.set_attr("simulator", AttrValue::String("RSpice".to_string()));
    builder.set_attr("title", AttrValue::String(data.title.clone()));

    if let Some(operating_point) = &data.operating_point {
        add_waveform_section(&mut builder, "operating_point", operating_point)?;
    }
    if let Some(transient) = &data.transient {
        add_waveform_section(&mut builder, "transient", transient)?;
    }
    if let Some(dc_sweep) = &data.dc_sweep {
        add_waveform_section(&mut builder, "dc_sweep", dc_sweep)?;
    }
    if let Some(noise) = &data.noise {
        add_waveform_section(&mut builder, "noise", noise)?;
    }
    if let Some(ac) = &data.ac {
        add_ac_section(&mut builder, ac)?;
    }
    if let Some(distortion) = &data.distortion {
        add_distortion_section(&mut builder, distortion)?;
    }
    if !data.measurements.is_empty() {
        add_measurements(&mut builder, &data.measurements)?;
    }

    write_hdf5_builder(path, builder)
}

fn write_hdf5_builder(path: &Path, builder: FileBuilder) -> Result<()> {
    write_atomic(path, |file| {
        let bytes = builder.finish()?;
        file.write_all(&bytes)?;
        Ok(())
    })
    .map_err(|error| match error {
        AtomicArtifactError::Preparation(error) => Hdf5Error::ArtifactPreparation(error),
        AtomicArtifactError::Write(Hdf5StagingError::Backend(error)) => Hdf5Error::Backend(error),
        AtomicArtifactError::Write(Hdf5StagingError::Io(error)) => Hdf5Error::ArtifactWrite(error),
        AtomicArtifactError::Flush(error) => Hdf5Error::ArtifactFlush(error),
        AtomicArtifactError::Commit(error) => Hdf5Error::ArtifactCommit(error),
    })
}

pub fn read_hdf5(path: &Path) -> Result<Hdf5SimulationData> {
    let file = Hdf5File::open(path)?;
    let root = file.root();
    let root_attrs = root.attrs()?;

    let title = read_string_attr(&root_attrs, "title")?.unwrap_or_default();
    let root_groups = root.groups()?;

    let operating_point = if root_groups.iter().any(|group| group == "operating_point") {
        Some(read_waveform_section(&file, "operating_point")?)
    } else {
        None
    };
    let transient = if root_groups.iter().any(|group| group == "transient") {
        Some(read_waveform_section(&file, "transient")?)
    } else {
        None
    };
    let dc_sweep = if root_groups.iter().any(|group| group == "dc_sweep") {
        Some(read_waveform_section(&file, "dc_sweep")?)
    } else {
        None
    };
    let noise = if root_groups.iter().any(|group| group == "noise") {
        Some(read_waveform_section(&file, "noise")?)
    } else {
        None
    };
    let ac = if root_groups.iter().any(|group| group == "ac") {
        Some(read_ac_section(&file)?)
    } else {
        None
    };
    let distortion = if root_groups.iter().any(|group| group == "distortion") {
        Some(read_distortion_section(&file)?)
    } else {
        None
    };
    let measurements = if root_groups.iter().any(|group| group == "measurements") {
        read_measurements(&file)?
    } else {
        Vec::new()
    };

    Ok(Hdf5SimulationData {
        title,
        operating_point,
        transient,
        dc_sweep,
        noise,
        ac,
        distortion,
        measurements,
    })
}

fn add_waveform_section(
    builder: &mut FileBuilder,
    name: &str,
    section: &Hdf5WaveformSection,
) -> Result<()> {
    let mut group = builder.create_group(name);
    group.set_attr("section_type", AttrValue::String(name.to_string()));
    group.set_attr(
        "independent_name",
        AttrValue::String(section.independent_name.clone()),
    );
    group.set_attr("signal_count", AttrValue::I64(section.signals.len() as i64));
    group
        .create_dataset("independent")
        .with_f64_data(&section.independent_values);

    for (index, signal) in section.signals.iter().enumerate() {
        let dataset_name = format!("signal_{index:04}");
        group.set_attr(
            &format!("{dataset_name}_name"),
            AttrValue::String(signal.name.clone()),
        );
        group.set_attr(
            &format!("{dataset_name}_type"),
            AttrValue::String(signal.var_type.clone()),
        );
        group
            .create_dataset(&dataset_name)
            .with_f64_data(&signal.values);
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_waveform_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5WaveformSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;

    let independent_name = read_required_string_attr(&attrs, "independent_name")?;
    let signal_count = read_required_i64_attr(&attrs, "signal_count")? as usize;
    let independent_values = group.dataset("independent")?.read_f64()?;

    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let dataset_name = format!("signal_{index:04}");
        let signal_name = read_required_string_attr(&attrs, &format!("{dataset_name}_name"))?;
        let signal_type = read_string_attr(&attrs, &format!("{dataset_name}_type"))?
            .unwrap_or_else(|| "value".to_string());
        let values = group.dataset(&dataset_name)?.read_f64()?;
        signals.push(Hdf5Signal::new_typed(signal_name, signal_type, values));
    }

    let section = Hdf5WaveformSection {
        independent_name,
        independent_values,
        signals,
    };
    section.validate(group_name)?;
    Ok(section)
}

fn add_ac_section(builder: &mut FileBuilder, section: &Hdf5AcSection) -> Result<()> {
    let mut group = builder.create_group("ac");
    group.set_attr("section_type", AttrValue::String("ac".to_string()));
    group.set_attr("signal_count", AttrValue::I64(section.signals.len() as i64));
    group
        .create_dataset("frequency")
        .with_f64_data(&section.frequency);

    for (index, signal) in section.signals.iter().enumerate() {
        let prefix = format!("signal_{index:04}");
        group.set_attr(
            &format!("{prefix}_name"),
            AttrValue::String(signal.name.clone()),
        );
        group
            .create_dataset(&format!("{prefix}_real"))
            .with_f64_data(&signal.real);
        group
            .create_dataset(&format!("{prefix}_imag"))
            .with_f64_data(&signal.imag);
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_ac_section(file: &Hdf5File) -> Result<Hdf5AcSection> {
    let group = file.group("ac")?;
    let attrs = group.attrs()?;
    let signal_count = read_required_i64_attr(&attrs, "signal_count")? as usize;
    let frequency = group.dataset("frequency")?.read_f64()?;

    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let prefix = format!("signal_{index:04}");
        let name = read_required_string_attr(&attrs, &format!("{prefix}_name"))?;
        let real = group.dataset(&format!("{prefix}_real"))?.read_f64()?;
        let imag = group.dataset(&format!("{prefix}_imag"))?.read_f64()?;
        signals.push(Hdf5ComplexSignal::new(name, real, imag));
    }

    let section = Hdf5AcSection { frequency, signals };
    section.validate()?;
    Ok(section)
}

fn add_distortion_section(
    builder: &mut FileBuilder,
    section: &Hdf5DistortionSection,
) -> Result<()> {
    let mut group = builder.create_group("distortion");
    group.set_attr("section_type", AttrValue::String("distortion".to_string()));
    group.set_attr("mode", AttrValue::String(section.mode.clone()));
    group.set_attr(
        "phasor_convention",
        AttrValue::String(section.phasor_convention.clone()),
    );
    group.set_attr(
        "ratio_normalization",
        AttrValue::String(section.ratio_normalization.clone()),
    );
    if let Some(ratio) = section.f2_over_f1 {
        group.set_attr("f2_over_f1", AttrValue::F64(ratio));
    }
    group.set_attr("series_count", AttrValue::I64(section.series.len() as i64));
    group
        .create_dataset("f1_frequency")
        .with_f64_data(&section.f1_frequency);

    for (series_index, series) in section.series.iter().enumerate() {
        let series_prefix = format!("series_{series_index:04}");
        group.set_attr(
            &format!("{series_prefix}_label"),
            AttrValue::String(series.label.clone()),
        );
        group.set_attr(
            &format!("{series_prefix}_is_product"),
            AttrValue::I64(i64::from(series.is_product)),
        );
        group.set_attr(
            &format!("{series_prefix}_signal_count"),
            AttrValue::I64(series.signals.len() as i64),
        );
        group
            .create_dataset(&format!("{series_prefix}_frequency"))
            .with_f64_data(&series.physical_frequency);

        for (signal_index, signal) in series.signals.iter().enumerate() {
            let signal_prefix = format!("{series_prefix}_signal_{signal_index:04}");
            group.set_attr(
                &format!("{signal_prefix}_name"),
                AttrValue::String(signal.name.clone()),
            );
            group.set_attr(
                &format!("{signal_prefix}_type"),
                AttrValue::String(signal.var_type.clone()),
            );
            group.set_attr(
                &format!("{signal_prefix}_has_ratio"),
                AttrValue::I64(i64::from(signal.magnitude_ratio_to_f1.is_some())),
            );
            group
                .create_dataset(&format!("{signal_prefix}_real"))
                .with_f64_data(&signal.real);
            group
                .create_dataset(&format!("{signal_prefix}_imag"))
                .with_f64_data(&signal.imag);
            group
                .create_dataset(&format!("{signal_prefix}_magnitude"))
                .with_f64_data(&signal.magnitude);
            group
                .create_dataset(&format!("{signal_prefix}_phase_degrees"))
                .with_f64_data(&signal.phase_degrees);
            if let Some(ratio) = &signal.magnitude_ratio_to_f1 {
                group
                    .create_dataset(&format!("{signal_prefix}_magnitude_ratio_to_f1"))
                    .with_f64_data(ratio);
            }
        }
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_distortion_section(file: &Hdf5File) -> Result<Hdf5DistortionSection> {
    let group = file.group("distortion")?;
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

fn add_measurements(builder: &mut FileBuilder, measurements: &[Hdf5Measurement]) -> Result<()> {
    let mut group = builder.create_group("measurements");
    group.set_attr(
        "measurement_count",
        AttrValue::I64(measurements.len() as i64),
    );
    for (index, measurement) in measurements.iter().enumerate() {
        let prefix = format!("measurement_{index:04}");
        group.set_attr(
            &format!("{prefix}_name"),
            AttrValue::String(measurement.name.clone()),
        );
        group.set_attr(
            &format!("{prefix}_value"),
            AttrValue::F64(measurement.value),
        );
    }
    builder.add_group(group.finish());
    Ok(())
}

fn read_measurements(file: &Hdf5File) -> Result<Vec<Hdf5Measurement>> {
    let group = file.group("measurements")?;
    let attrs = group.attrs()?;
    let measurement_count = read_required_i64_attr(&attrs, "measurement_count")? as usize;

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
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected string, found {other:?}"
        ))),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(std::path::PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-hdf5-atomic-{}-{id}-{tag}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create unique HDF5 test directory");
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn assert_only_destination_remains(directory: &Path, destination: &Path, exists: bool) {
        let entries: Vec<std::path::PathBuf> = std::fs::read_dir(directory)
            .expect("read HDF5 test directory")
            .map(|entry| entry.expect("read HDF5 directory entry").path())
            .collect();
        if exists {
            assert_eq!(entries, vec![destination.to_path_buf()]);
        } else {
            assert!(
                entries.is_empty(),
                "unexpected staged artifacts: {entries:?}"
            );
        }
    }

    #[test]
    fn backend_serialization_failure_preserves_old_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("backend-failure");
            let destination = directory.0.join("result.h5");
            if preexisting {
                std::fs::write(&destination, b"old complete HDF5 artifact")
                    .expect("seed existing HDF5 destination");
            }

            let mut incomplete_builder = FileBuilder::new();
            incomplete_builder.create_dataset("missing_data");
            let error = write_hdf5_builder(&destination, incomplete_builder)
                .expect_err("incomplete dataset must fail serialization");
            assert!(matches!(error, Hdf5Error::Backend(_)));

            if preexisting {
                assert_eq!(
                    std::fs::read(&destination).expect("read preserved HDF5 destination"),
                    b"old complete HDF5 artifact"
                );
            } else {
                assert!(!destination.exists());
            }
            assert_only_destination_remains(&directory.0, &destination, preexisting);
        }
    }

    #[test]
    fn successful_hdf5_write_atomically_replaces_existing_bytes() {
        let directory = TestDirectory::new("success");
        let destination = directory.0.join("result.h5");
        std::fs::write(&destination, b"old complete HDF5 artifact")
            .expect("seed existing HDF5 destination");

        let mut data = Hdf5SimulationData::new();
        data.title = "atomic result".to_string();
        let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
        transient.add_signal("V(out)", vec![0.0, 2.0]);
        data.transient = Some(transient);
        write_hdf5(&destination, &data).expect("write complete HDF5 destination");

        assert_eq!(
            read_hdf5(&destination).expect("read committed HDF5 destination"),
            data
        );
        assert_only_destination_remains(&directory.0, &destination, true);
    }
}
