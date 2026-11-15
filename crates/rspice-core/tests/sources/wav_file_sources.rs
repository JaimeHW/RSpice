//! A PCM wavefile played as a PWL source keeps every sample, however long.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// A 16-bit mono recording of alternating full-scale samples: a tone at the
/// Nyquist rate. Keeping one sample in two turns it into DC.
fn nyquist_recording(samples: usize, rate: u32) -> Vec<u8> {
    let data_size = u32::try_from(samples * 2).expect("fixture fits a RIFF chunk");
    let mut bytes = Vec::with_capacity(44 + samples * 2);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&rate.to_le_bytes());
    bytes.extend_from_slice(&(rate * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    for index in 0..samples {
        let sample = if index % 2 == 0 { i16::MAX } else { -i16::MAX };
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    bytes
}

/// LTspice plays a whole wavefile. 1.2 million samples used to be thinned to
/// every second one to fit a one-million-point count, which aliased this
/// Nyquist tone to a constant; every sample now reaches the circuit.
#[test]
fn a_wavefile_past_a_million_samples_plays_every_sample() {
    let rate = 48_000u32;
    let samples = 1_200_000;
    let directory = std::env::temp_dir().join(format!("rspice-wav-source-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("temporary directory");
    let path = directory.join("nyquist.wav");
    std::fs::write(&path, nyquist_recording(samples, rate)).expect("write the recording");
    let deck = format!(
        "wavefile past a million samples\nV1 in 0 PWL(FILE='{}')\nR1 in 0 1k\n.end\n",
        path.display().to_string().replace('\\', "/")
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let period = 1.0 / f64::from(rate);
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 8.0 * period, period / 4.0)
        .expect("the recording plays");
    let _ = std::fs::remove_dir_all(&directory);
    let node = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in"))
        .expect("V(in) is retained");
    let full_scale = f64::from(i16::MAX) / 32768.0;
    for sample in 0..=8 {
        let time = f64::from(sample) * period;
        let index = result
            .time
            .iter()
            .position(|&t| (t - time).abs() <= 1.0e-12 * period)
            .unwrap_or_else(|| panic!("the run lands on sample {sample}"));
        let expected = if sample % 2 == 0 {
            full_scale
        } else {
            -full_scale
        };
        let actual = result.voltages[node][index];
        assert!(
            (actual - expected).abs() < 1.0e-12,
            "sample {sample}: V(in) = {actual}, recorded {expected}"
        );
    }
}
