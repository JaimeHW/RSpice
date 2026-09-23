//! Authored oscillator phase reference and spectral startup seed.
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicAutonomousConfig;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssOscillatorSeed {
    pub node: String,
    /// Peak voltage of the selected phase tuple.
    pub amplitude: Value,
    pub phase_degrees: Value,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssOscillator {
    /// Zero-based free tone; its configured frequency is the initial guess.
    pub tone: usize,
    pub node: String,
    /// Empty selects the fundamental of the free tone.
    pub phase_tuple: Vec<i32>,
    pub initial_amplitude: Value,
    pub minimum_amplitude: Value,
    pub max_relative_frequency_step: Value,
    pub additional_seeds: Vec<QpssOscillatorSeed>,
}

impl QpssOscillator {
    pub fn new(tone: usize, node: String) -> Self {
        Self {
            tone,
            node,
            phase_tuple: Vec::new(),
            initial_amplitude: 0.1,
            minimum_amplitude: 1e-6,
            max_relative_frequency_step: 0.2,
            additional_seeds: Vec::new(),
        }
    }

    pub fn resolved_phase_tuple(&self, tones: usize) -> Result<Vec<i32>, SimulationError> {
        if self.tone >= tones {
            return Err(invalid(
                "oscillator tone is outside the configured frequencies",
            ));
        }
        let mut tuple = self.phase_tuple.clone();
        if tuple.is_empty() {
            tuple = vec![0; tones];
            tuple[self.tone] = 1;
        }
        if tuple.len() != tones || tuple[self.tone] == 0 {
            return Err(invalid(
                "oscillator phase tuple must include its free tone and have one index per tone",
            ));
        }
        Ok(tuple)
    }

    pub(super) fn validate(&self, grid: &QuasiPeriodicGridConfig) -> Result<(), SimulationError> {
        let tuple = self.resolved_phase_tuple(grid.frequencies_hz.len())?;
        let order = tuple.iter().fold(0usize, |sum, v| {
            sum.saturating_add(v.unsigned_abs() as usize)
        });
        let mixed = tuple.iter().filter(|v| **v != 0).count() > 1;
        if tuple
            .iter()
            .zip(&grid.harmonics)
            .any(|(v, h)| v.unsigned_abs() as usize > *h)
            || (mixed && grid.max_mixing_order.is_some_and(|max| order > max))
        {
            return Err(invalid(
                "oscillator phase tuple is outside the retained harmonic lattice",
            ));
        }
        if !self.initial_amplitude.is_finite()
            || !self.minimum_amplitude.is_finite()
            || self.minimum_amplitude <= 0.0
            || self.initial_amplitude < self.minimum_amplitude
            || self.initial_amplitude * 0.5 == 0.0
            || !self.max_relative_frequency_step.is_finite()
            || !(0.0 < self.max_relative_frequency_step && self.max_relative_frequency_step <= 1.0)
        {
            return Err(invalid(
                "oscillator amplitudes must be positive, initial amplitude at least the minimum, and frequency step in (0,1]",
            ));
        }
        let mut names = std::collections::HashSet::new();
        for name in
            std::iter::once(&self.node).chain(self.additional_seeds.iter().map(|seed| &seed.node))
        {
            if name.is_empty()
                || !name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "_.$:#%![]".contains(c))
                || !names.insert(name.to_ascii_lowercase())
            {
                return Err(invalid("oscillator seeds require distinct bare node names"));
            }
        }
        if self.additional_seeds.iter().any(|seed| {
            !seed.amplitude.is_finite()
                || seed.amplitude <= 0.0
                || seed.amplitude * 0.5 == 0.0
                || !seed.phase_degrees.is_finite()
        }) {
            return Err(invalid(
                "oscillator seed amplitudes must be positive and phases finite",
            ));
        }
        Ok(())
    }

    pub(super) fn prepare_seed(
        &self,
        grid: &QuasiPeriodicGrid,
        nodes: &[String],
        seed: &mut [Vec<Complex64>],
    ) -> Result<QuasiPeriodicAutonomousConfig, SimulationError> {
        let phase_tuple = self.resolved_phase_tuple(grid.config().frequencies_hz.len())?;
        let index = grid
            .index_of(&phase_tuple)
            .ok_or_else(|| invalid("oscillator phase tuple is not retained"))?;
        let node_index = |name: &str| {
            nodes
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .ok_or_else(|| {
                    invalid(format!(
                        "oscillator seed node '{name}' is absent from the circuit"
                    ))
                })
        };
        let phase_coordinate = node_index(&self.node)?;
        let mut set =
            |name: &str, amplitude: Value, degrees: Value| -> Result<(), SimulationError> {
                let row = node_index(name)?;
                seed[row][index] =
                    Complex64::from_polar(amplitude * 0.5, degrees.rem_euclid(360.0).to_radians());
                seed[row][grid.len() - 1 - index] = seed[row][index].conj();
                Ok(())
            };
        set(&self.node, self.initial_amplitude, 0.0)?;
        for extra in &self.additional_seeds {
            set(&extra.node, extra.amplitude, extra.phase_degrees)?;
        }
        Ok(QuasiPeriodicAutonomousConfig {
            tone: self.tone,
            phase_coordinate,
            phase_tuple,
            minimum_amplitude: self.minimum_amplitude,
            max_relative_frequency_step: self.max_relative_frequency_step,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::AnalysisCommand;
    use std::f64::consts::{SQRT_2, TAU};

    #[test]
    fn autonomous_qpss_engine_retains_solved_frequency_and_reproducible_startup() {
        let text = format!(
            "Modulated oscillator\nCx x 0 1\nCy y 0 1\nBx 0 x I={{(1-v(x)^2-v(y)^2)*v(x)-(1+.1*sqrt(2)*cos(sqrt(2)*time))*v(y)}}\nBy 0 y I={{(1-v(x)^2-v(y)^2)*v(y)+(1+.1*sqrt(2)*cos(sqrt(2)*time))*v(x)}}\n.QPSS {} {} HARMS=(2,3) POINTS=(13,25) RELTOL=1e-8 OSCTONE=1 OSCNODE=x OSCAMPLITUDE=.8 OSCMINAMPLITUDE=.01 OSCFREQSTEP=.15 OSCTUPLE=(1,0) OSCSEED=(y,.8,-90)\n.end\n",
            1.2 / TAU,
            SQRT_2 / TAU
        );
        let netlist = Netlist::parse(&text).unwrap();
        let AnalysisCommand::Qpss(card) = &netlist.analyses[0] else {
            panic!("QPSS card expected")
        };
        let config = QpssConfig::from_qpss_card(card).unwrap();
        let roundtrip = Netlist::parse(&format!(
            "roundtrip\n{}\n.end\n",
            config.to_spice().unwrap()
        ))
        .unwrap();
        let AnalysisCommand::Qpss(card) = &roundtrip.analyses[0] else {
            panic!("QPSS card expected")
        };
        assert_eq!(QpssConfig::from_qpss_card(card).unwrap(), config);
        let engine = Engine::new(Default::default());
        let point = engine.run_qpss(&netlist, config.clone()).unwrap();
        assert_eq!(point.config(), &config);
        let frequency = point.oscillator_frequency_hz().unwrap();
        assert!((frequency - 1.0 / TAU).abs() < 1e-8);
        let grid = engine
            .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert_eq!(grid.config().frequencies_hz, [frequency, SQRT_2 / TAU]);
        let mut transform =
            crate::analysis::quasi_periodic::QuasiPeriodicTransform::new_with_abort(
                grid.clone(),
                &NoAbort,
            )
            .unwrap();
        let x = point
            .node_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("x"))
            .unwrap();
        for (i, value) in transform
            .to_real_samples_with_abort(&point.spectra()[x], &NoAbort)
            .unwrap()
            .iter()
            .enumerate()
        {
            let phases = grid.phases(i).unwrap();
            assert!((value - (phases[0] + 0.1 * phases[1].sin()).cos()).abs() < 1e-5);
        }
        let limits = crate::ResourceLimits::default();
        let (metadata, rows) = point.clone().into_transfer_parts();
        metadata
            .validate_transfer_layout_with_abort(
                &rows.iter().map(Vec::len).collect::<Vec<_>>(),
                &limits,
                &NoAbort,
            )
            .unwrap();
        assert_eq!(
            QpssOperatingPoint::from_transfer_parts_with_abort(metadata, rows, &limits, &NoAbort)
                .unwrap(),
            point
        );
        let saved = serde_json::to_value(&point).unwrap();
        assert_eq!(
            serde_json::from_value::<QpssOperatingPoint>(saved.clone()).unwrap(),
            point
        );
        let mut altered = saved.clone();
        altered["oscillator_frequency_hz"] = serde_json::json!(frequency * 1.01);
        assert!(
            serde_json::from_value::<QpssOperatingPoint>(altered)
                .unwrap()
                .validate_retained_payload_with_abort(&limits, &NoAbort)
                .is_err()
        );
        let mut missing = saved;
        missing
            .as_object_mut()
            .unwrap()
            .remove("oscillator_frequency_hz");
        assert!(
            serde_json::from_value::<QpssOperatingPoint>(missing)
                .unwrap()
                .validate_retained_payload_with_abort(&limits, &NoAbort)
                .is_err()
        );
        for fields in [
            "OSCNODE=x",
            "OSCTONE=0 OSCNODE=x",
            "OSCTONE=3 OSCNODE=x",
            "OSCTONE=1 OSCNODE=x OSCFREQSTEP=2",
            "OSCTONE=1 OSCNODE=x OSCTUPLE=(0,1)",
            "OSCTONE=1 OSCNODE=x OSCSEED=(x,.1,90)",
            "OSCTONE=1 OSCNODE=x SOURCE1=V1",
        ] {
            let parsed = Netlist::parse(&format!("invalid\n.QPSS 1 1.414 {fields}\n.end\n"));
            if let Ok(netlist) = parsed {
                let AnalysisCommand::Qpss(card) = &netlist.analyses[0] else {
                    panic!("QPSS card expected")
                };
                assert!(QpssConfig::from_qpss_card(card).is_err(), "{fields}");
            }
        }
    }
}
