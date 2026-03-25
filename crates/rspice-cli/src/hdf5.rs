//! HDF5 storage for simulation results.
//!
//! The CLI uses a stable, self-describing layout with path-safe dataset names
//! so exported files remain robust even when signal names contain SPICE syntax.

use rustyhdf5::{AttrValue, File as Hdf5File, FileBuilder};
use thiserror::Error;

use std::collections::HashMap;
use std::path::Path;

const SCHEMA_VERSION: &str = "1";

#[derive(Debug, Error)]
pub enum Hdf5Error {
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error("invalid HDF5 schema: {0}")]
    InvalidSchema(String),
}

pub type Result<T> = std::result::Result<T, Hdf5Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Signal {
    pub name: String,
    pub values: Vec<f64>,
}

impl Hdf5Signal {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
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
    pub ac: Option<Hdf5AcSection>,
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
        if let Some(ac) = &self.ac {
            ac.validate()?;
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
    if let Some(ac) = &data.ac {
        add_ac_section(&mut builder, ac)?;
    }
    if !data.measurements.is_empty() {
        add_measurements(&mut builder, &data.measurements)?;
    }

    builder.write(path)?;
    Ok(())
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
    let ac = if root_groups.iter().any(|group| group == "ac") {
        Some(read_ac_section(&file)?)
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
        ac,
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
        let values = group.dataset(&dataset_name)?.read_f64()?;
        signals.push(Hdf5Signal::new(signal_name, values));
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

    fn sample_data() -> Hdf5SimulationData {
        let mut data = Hdf5SimulationData::new();
        data.title = "Example".to_string();

        let mut operating_point = Hdf5WaveformSection::new("point", vec![0.0]);
        operating_point.add_signal("V(out)", vec![1.25]);
        data.operating_point = Some(operating_point);

        let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0e-9, 2.0e-9]);
        transient.add_signal("V(out)", vec![0.0, 0.5, 1.0]);
        transient.add_signal("I(V1)", vec![0.0, -0.2, -0.4]);
        data.transient = Some(transient);

        let mut dc_sweep = Hdf5WaveformSection::new("V1", vec![0.0, 1.0, 2.0]);
        dc_sweep.add_signal("V(out)", vec![0.0, 0.5, 1.0]);
        data.dc_sweep = Some(dc_sweep);

        let mut ac = Hdf5AcSection::new(vec![1.0, 10.0, 100.0]);
        ac.add_signal("V(out)", vec![1.0, 0.5, 0.1], vec![0.0, -0.1, -0.2]);
        data.ac = Some(ac);

        data.measurements
            .push(Hdf5Measurement::new("gain_db", 20.0));
        data
    }

    #[test]
    fn test_waveform_validation_rejects_length_mismatch() {
        let mut section = Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
        section.add_signal("V(out)", vec![1.0]);

        let err = section
            .validate("transient")
            .expect_err("length mismatch should fail");
        assert!(err.to_string().contains("expected 2"));
    }

    #[test]
    fn test_ac_validation_rejects_imag_length_mismatch() {
        let mut section = Hdf5AcSection::new(vec![1.0, 2.0]);
        section.add_signal("V(out)", vec![1.0, 0.5], vec![0.0]);

        let err = section
            .validate()
            .expect_err("imaginary length mismatch should fail");
        assert!(err.to_string().contains("imaginary part"));
    }

    #[test]
    fn test_hdf5_round_trip_preserves_all_sections() {
        let temp = tempfile::tempdir().expect("temp dir");
        let path = temp.path().join("waveforms.h5");
        let expected = sample_data();

        write_hdf5(&path, &expected).expect("write should succeed");
        let actual = read_hdf5(&path).expect("read should succeed");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hdf5_uses_safe_dataset_names() {
        let temp = tempfile::tempdir().expect("temp dir");
        let path = temp.path().join("waveforms.h5");
        let data = sample_data();

        write_hdf5(&path, &data).expect("write should succeed");

        let file = Hdf5File::open(&path).expect("file should open");
        let transient = file.group("transient").expect("transient group");
        let mut datasets = transient.datasets().expect("datasets should be listed");
        datasets.sort();

        assert!(datasets.contains(&"independent".to_string()));
        assert!(datasets.contains(&"signal_0000".to_string()));
        assert!(datasets.contains(&"signal_0001".to_string()));
        assert!(!datasets.contains(&"V(out)".to_string()));
    }
}
