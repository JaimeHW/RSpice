//! HDF5 Output Support for Large Waveform Storage
//!
//! Provides efficient binary storage for simulation results using HDF5 format.
//! This is ideal for large transient simulations, Monte Carlo sweeps, and
//! post-processing with external tools like Python/MATLAB.
//!
//! # File Structure
//! ```text
//! simulation.h5
//! ├── /metadata
//! │   ├── title (attribute)
//! │   ├── date (attribute)
//! │   └── simulator_version (attribute)
//! ├── /transient
//! │   ├── time (dataset: [N])
//! │   └── signals (group)
//! │       ├── V(out) (dataset: [N])
//! │       └── I(r1) (dataset: [N])
//! ├── /ac
//! │   ├── frequency (dataset: [M])
//! │   └── signals (group)
//! │       └── V(out) (dataset: [M, 2] - magnitude, phase)
//! └── /measurements
//!     └── (name, value) pairs
//! ```

#![allow(dead_code)] // Reserved public APIs for HDF5 feature

#[cfg(feature = "hdf5")]
use hdf5::{File as Hdf5File, Result as Hdf5Result};

use std::collections::HashMap;
use std::path::Path;

/// Result from simulation suitable for HDF5 export
pub struct Hdf5SimulationData {
    /// Simulation title/name
    pub title: String,
    /// Time vector for transient results
    pub time: Vec<f64>,
    /// Signal data: name -> values
    pub signals: HashMap<String, Vec<f64>>,
    /// AC frequency vector (if applicable)
    pub frequency: Option<Vec<f64>>,
    /// AC magnitude/phase data: name -> (magnitude, phase)
    pub ac_signals: HashMap<String, (Vec<f64>, Vec<f64>)>,
    /// Measurement results: name -> value
    pub measurements: HashMap<String, f64>,
}

impl Default for Hdf5SimulationData {
    fn default() -> Self {
        Self::new()
    }
}

impl Hdf5SimulationData {
    /// Create empty simulation data container
    pub fn new() -> Self {
        Self {
            title: String::new(),
            time: Vec::new(),
            signals: HashMap::new(),
            frequency: None,
            ac_signals: HashMap::new(),
            measurements: HashMap::new(),
        }
    }

    /// Add transient signal data
    pub fn add_transient_signal(&mut self, name: impl Into<String>, values: Vec<f64>) {
        self.signals.insert(name.into(), values);
    }

    /// Add AC signal data
    pub fn add_ac_signal(&mut self, name: impl Into<String>, magnitude: Vec<f64>, phase: Vec<f64>) {
        self.ac_signals.insert(name.into(), (magnitude, phase));
    }

    /// Add measurement result
    pub fn add_measurement(&mut self, name: impl Into<String>, value: f64) {
        self.measurements.insert(name.into(), value);
    }
}

/// Write simulation data to HDF5 file
#[cfg(feature = "hdf5")]
pub fn write_hdf5(path: &Path, data: &Hdf5SimulationData) -> Hdf5Result<()> {
    let file = Hdf5File::create(path)?;

    // Write metadata
    let root = file.group("/")?;
    root.new_attr::<hdf5::types::VarLenUnicode>()
        .create("title")?
        .write_scalar(&hdf5::types::VarLenUnicode::from_str(&data.title))?;
    root.new_attr::<hdf5::types::VarLenUnicode>()
        .create("simulator")?
        .write_scalar(&hdf5::types::VarLenUnicode::from_str("RSpice"))?;

    // Write transient data
    if !data.time.is_empty() {
        let tran = file.create_group("transient")?;
        tran.new_dataset::<f64>()
            .shape([data.time.len()])
            .create("time")?
            .write(&data.time)?;

        let signals = tran.create_group("signals")?;
        for (name, values) in &data.signals {
            signals
                .new_dataset::<f64>()
                .shape([values.len()])
                .create(name)?
                .write(values)?;
        }
    }

    // Write AC data
    if let Some(ref freq) = data.frequency {
        let ac = file.create_group("ac")?;
        ac.new_dataset::<f64>()
            .shape([freq.len()])
            .create("frequency")?
            .write(freq)?;

        let signals = ac.create_group("signals")?;
        for (name, (mag, phase)) in &data.ac_signals {
            let group = signals.create_group(name)?;
            group
                .new_dataset::<f64>()
                .shape([mag.len()])
                .create("magnitude")?
                .write(mag)?;
            group
                .new_dataset::<f64>()
                .shape([phase.len()])
                .create("phase")?
                .write(phase)?;
        }
    }

    // Write measurements
    if !data.measurements.is_empty() {
        let meas = file.create_group("measurements")?;
        for (name, value) in &data.measurements {
            meas.new_attr::<f64>().create(name)?.write_scalar(value)?;
        }
    }

    Ok(())
}

/// Stub for when HDF5 feature is not enabled
#[cfg(not(feature = "hdf5"))]
pub fn write_hdf5(_path: &Path, _data: &Hdf5SimulationData) -> Result<(), String> {
    Err("HDF5 support not enabled. Rebuild with --features hdf5".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_data_builder() {
        let mut data = Hdf5SimulationData::new();
        data.title = "Test Simulation".to_string();
        data.time = vec![0.0, 1.0, 2.0];
        data.add_transient_signal("V(out)", vec![0.0, 1.0, 0.5]);
        data.add_measurement("vmax", 1.0);

        assert_eq!(data.signals.len(), 1);
        assert_eq!(data.measurements.len(), 1);
        assert_eq!(data.time.len(), 3);
    }
}
