//! Authored-card/configuration boundary for driven and autonomous QPSS.
use super::{
    QpssConfig, QpssInitialState, QpssOscillator, QpssOscillatorSeed, QpssSourceTone, invalid,
    numerical_error,
};
use crate::analysis::quasi_periodic::{QuasiPeriodicLinearMethod, QuasiPeriodicSampling};
use crate::engine::SimulationError;
use crate::netlist::QpssCard;

impl QpssConfig {
    /// Cheap authoring checks; phase-grid allocation and circuit-dependent
    /// resource/degeneracy checks remain at execution.
    pub fn validate_configuration(&self) -> Result<(), SimulationError> {
        self.grid.validate().map_err(numerical_error)?;
        self.solver.validate().map_err(numerical_error)?;
        if let Some(oscillator) = &self.oscillator {
            oscillator.validate(&self.grid)?;
        }
        if self
            .grid
            .frequencies_hz
            .iter()
            .enumerate()
            .any(|(i, f)| self.grid.frequencies_hz[..i].contains(f))
        {
            return Err(invalid("tone frequencies must be distinct"));
        }
        let mut seen = std::collections::HashSet::new();
        for source in &self.source_tones {
            let mut chars = source.source.chars();
            let start = chars
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic() || matches!(c, '_' | '%'));
            if !start
                || !chars.all(|c| {
                    c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '#' | ':' | '%' | '!')
                })
                || source.tone >= self.grid.frequencies_hz.len()
                || self
                    .oscillator
                    .as_ref()
                    .is_some_and(|oscillator| oscillator.tone == source.tone)
                || !seen.insert((source.source.to_ascii_lowercase(), source.tone))
            {
                return Err(invalid(
                    "source assignments need bare SPICE source names, valid tone indices, and unique source/tone pairs",
                ));
            }
        }
        Ok(())
    }

    /// Resolve every optional card value against the core's defaults. QPSS
    /// has independent controls; `.OPTIONS HBINT` does not configure this solver.
    pub fn from_qpss_card(card: &QpssCard) -> Result<Self, SimulationError> {
        let tones = card.frequencies.len();
        if card.oversample.as_ref().is_some_and(Vec::is_empty)
            || card.points.as_ref().is_some_and(Vec::is_empty)
        {
            return Err(invalid(
                "an authored sampling mode requires positive counts",
            ));
        }
        let harmonics = counts(&card.harmonics, tones, 7, "HARMS")?;
        let mut config = Self::new(card.frequencies.clone(), harmonics);
        let oscillator_fields = card.oscillator_tone.is_some()
            || card.oscillator_node.is_some()
            || card.oscillator_tuple.is_some()
            || card.oscillator_amplitude.is_some()
            || card.oscillator_minimum_amplitude.is_some()
            || card.oscillator_frequency_step.is_some()
            || !card.oscillator_seeds.is_empty();
        if oscillator_fields {
            let tone = card
                .oscillator_tone
                .ok_or_else(|| invalid("oscillator settings require OSCTONE"))?;
            let node = card
                .oscillator_node
                .clone()
                .ok_or_else(|| invalid("oscillator settings require OSCNODE"))?;
            let mut oscillator = QpssOscillator::new(tone, node);
            if let Some(tuple) = &card.oscillator_tuple {
                oscillator.phase_tuple = tuple.clone();
            }
            if let Some(v) = card.oscillator_amplitude {
                oscillator.initial_amplitude = v;
            }
            if let Some(v) = card.oscillator_minimum_amplitude {
                oscillator.minimum_amplitude = v;
            }
            if let Some(v) = card.oscillator_frequency_step {
                oscillator.max_relative_frequency_step = v;
            }
            oscillator.additional_seeds = card
                .oscillator_seeds
                .iter()
                .map(|(node, amplitude, phase_degrees)| QpssOscillatorSeed {
                    node: node.clone(),
                    amplitude: *amplitude,
                    phase_degrees: *phase_degrees,
                })
                .collect();
            config.oscillator = Some(oscillator);
        }
        match (&card.oversample, &card.points) {
            (Some(_), Some(_)) => {
                return Err(invalid("POINTS and OVERSAMPLE are mutually exclusive"));
            }
            (Some(values), None) => {
                config.grid.sampling =
                    QuasiPeriodicSampling::Oversample(counts(values, tones, 2, "OVERSAMPLE")?)
            }
            (None, Some(values)) => {
                config.grid.sampling =
                    QuasiPeriodicSampling::Exact(counts(values, tones, 0, "POINTS")?)
            }
            (None, None) => (),
        }
        config.grid.max_mixing_order = card.max_mixing_order;
        if let Some(method) = &card.linear_solver {
            config.solver.linear.method = match method.to_ascii_uppercase().as_str() {
                "AUTO" => QuasiPeriodicLinearMethod::Auto,
                "DIRECT" => QuasiPeriodicLinearMethod::Direct,
                "KRYLOV" => QuasiPeriodicLinearMethod::Krylov,
                _ => return Err(invalid("SOLVER must be AUTO, DIRECT or KRYLOV")),
            };
        }
        if let Some(v) = card.krylov_restart {
            config.solver.linear.restart = v;
        }
        if let Some(v) = card.krylov_cycles {
            config.solver.linear.max_cycles = v;
        }
        if let Some(v) = card.linear_tolerance {
            config.solver.linear.relative_tolerance = v;
        }
        if let Some(v) = card.relative_tolerance {
            config.solver.relative_tolerance = v;
        }
        if let Some(v) = card.current_absolute_tolerance {
            config.solver.current_absolute_tolerance = v;
        }
        if let Some(v) = card.voltage_absolute_tolerance {
            config.solver.voltage_absolute_tolerance = v;
        }
        if let Some(v) = card.max_iterations {
            config.solver.max_iterations = v;
        }
        if let Some(v) = card.max_backtracks {
            config.solver.max_backtracks = v;
        }
        if card.dc_initial_state == Some(true) {
            config.initial_state = QpssInitialState::DcOperatingPoint;
        }
        config.source_tones = card
            .sources
            .iter()
            .map(|(source, tone)| QpssSourceTone {
                source: source.clone(),
                tone: *tone,
            })
            .collect();
        config.validate_configuration()?;
        Ok(config)
    }

    /// Lossless decimal authoring of every active setting. Source assignment
    /// order is retained because it participates in the retained state identity.
    pub fn to_spice(&self) -> Result<String, SimulationError> {
        self.validate_configuration()?;
        let mut fields = vec![".QPSS".to_owned()];
        fields.extend(self.grid.frequencies_hz.iter().map(ToString::to_string));
        fields.push(format!("HARMS=({})", list(&self.grid.harmonics)));
        fields.push(match &self.grid.sampling {
            QuasiPeriodicSampling::Oversample(v) => format!("OVERSAMPLE=({})", list(v)),
            QuasiPeriodicSampling::Exact(v) => format!("POINTS=({})", list(v)),
        });
        if let Some(v) = self.grid.max_mixing_order {
            fields.push(format!("MAXMIXING={v}"));
        }
        fields.extend([
            format!(
                "SOLVER={}",
                match self.solver.linear.method {
                    QuasiPeriodicLinearMethod::Auto => "AUTO",
                    QuasiPeriodicLinearMethod::Direct => "DIRECT",
                    QuasiPeriodicLinearMethod::Krylov => "KRYLOV",
                }
            ),
            format!("KRYLOVRESTART={}", self.solver.linear.restart),
            format!("KRYLOVCYCLES={}", self.solver.linear.max_cycles),
            format!("LINEARTOL={}", self.solver.linear.relative_tolerance),
            format!("RELTOL={}", self.solver.relative_tolerance),
            format!("IABSTOL={}", self.solver.current_absolute_tolerance),
            format!("VABSTOL={}", self.solver.voltage_absolute_tolerance),
            format!("MAXITER={}", self.solver.max_iterations),
            format!("MAXBACKTRACKS={}", self.solver.max_backtracks),
            format!(
                "INIT={}",
                if self.initial_state == QpssInitialState::Zero {
                    "ZERO"
                } else {
                    "DC"
                }
            ),
        ]);
        fields.extend(
            self.source_tones
                .iter()
                .map(|s| format!("SOURCE{}={}", s.tone + 1, s.source)),
        );
        if let Some(oscillator) = &self.oscillator {
            fields.extend([
                format!("OSCTONE={}", oscillator.tone + 1),
                format!("OSCNODE={}", oscillator.node),
                format!("OSCAMPLITUDE={}", oscillator.initial_amplitude),
                format!("OSCMINAMPLITUDE={}", oscillator.minimum_amplitude),
                format!("OSCFREQSTEP={}", oscillator.max_relative_frequency_step),
            ]);
            if !oscillator.phase_tuple.is_empty() {
                fields.push(format!(
                    "OSCTUPLE=({})",
                    oscillator
                        .phase_tuple
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                ));
            }
            fields.extend(oscillator.additional_seeds.iter().map(|seed| {
                format!(
                    "OSCSEED=({},{},{})",
                    seed.node, seed.amplitude, seed.phase_degrees
                )
            }));
        }
        Ok(fields.join(" "))
    }
}

fn counts(
    values: &[usize],
    tones: usize,
    default: usize,
    field: &str,
) -> Result<Vec<usize>, SimulationError> {
    if values.is_empty() {
        return Ok(vec![default; tones]);
    }
    if values.len() == 1 {
        return Ok(vec![values[0]; tones]);
    }
    if values.len() != tones {
        return Err(invalid(format!(
            "{field} requires one count or one count per tone"
        )));
    }
    Ok(values.to_vec())
}
fn list(values: &[usize]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::{AnalysisCommand, Netlist};

    fn parse(line: &str) -> Result<QpssConfig, String> {
        let deck =
            Netlist::parse(&format!("QPSS card\n{line}\n.end\n")).map_err(|e| e.to_string())?;
        let AnalysisCommand::Qpss(card) = &deck.analyses[0] else {
            panic!("wrong card")
        };
        QpssConfig::from_qpss_card(card).map_err(|e| e.to_string())
    }

    #[test]
    fn qpss_card_round_trip_preserves_all_controls_and_ordered_assignments() {
        for sampling in ["POINTS=(8,12,9)", "OVERSAMPLE=(3,4,2)"] {
            let line = format!(
                ".QPSS 1k 1.4142135623730951k 1.7320508075688772k HARMS=(2,3,1) {sampling} MAXMIXING=2 RELTOL=0.000001234567891234567 IABSTOL=2e-13 VABSTOL=4e-10 MAXITER=75 MAXBACKTRACKS=9 SOLVER=KRYLOV KRYLOVRESTART=24 KRYLOVCYCLES=7 LINEARTOL=2e-11 INIT=DC SOURCE2=vMix SOURCE1=vMix SOURCE2=iDrive"
            );
            let config = parse(&line).unwrap();
            assert_eq!(config, parse(&config.to_spice().unwrap()).unwrap());
            assert_eq!(
                config
                    .source_tones
                    .iter()
                    .map(|s| s.tone)
                    .collect::<Vec<_>>(),
                [1, 0, 1]
            );
        }
        let defaults = parse(".QPSS 1k 1.4142135623730951k").unwrap();
        assert_eq!(
            defaults,
            QpssConfig::new(vec![1e3, 1.414_213_562_373_095e3], vec![7, 7])
        );
        let broadcast = parse(".QPSS 1k 1.4142135623730951k HARMS=2 OVERSAMPLE=3").unwrap();
        assert_eq!(broadcast.grid.harmonics, [2, 2]);
        assert_eq!(
            broadcast.grid.sampling,
            QuasiPeriodicSampling::Oversample(vec![3, 3])
        );
    }

    #[test]
    fn qpss_card_rejects_ambiguous_or_unrepresentable_controls() {
        for suffix in [
            "HARMS=(2,3,4)",
            "HARMS=0",
            "POINTS=2",
            "POINTS=31 OVERSAMPLE=2",
            "RELTOL=1",
            "MAXBACKTRACKS=61",
            "INIT=BAD",
            "SOURCE0=V1",
            "SOURCE1=V-one",
            "SOURCE3=V1",
            "SOURCE1=V1 SOURCE1=v1",
            "ABSTOL=1e-12 IABSTOL=1e-12",
            "DAMPING=1",
            "SOLVER=BAD",
            "KRYLOVRESTART=65",
            "KRYLOVCYCLES=0",
            "LINEARTOL=1",
            "HARMS=1 HARMS=2",
        ] {
            assert!(
                parse(&format!(".QPSS 1k 1.4142135623730951k {suffix}")).is_err(),
                "{suffix}"
            );
        }
        assert!(parse(".QPSS 1k").is_err());
        assert!(parse(".QPSS 1k 1k").is_err());
        let mut config = QpssConfig::new(vec![1e3, 1.414_213_562_373_095e3], vec![1, 1]);
        config.source_tones.push(QpssSourceTone {
            source: "V1\n.end".into(),
            tone: 0,
        });
        assert!(config.to_spice().is_err());
    }
    #[test]
    fn qpss_card_default_linear_settings_preserve_legacy_config_bytes() {
        let legacy = r#"{"relative_tolerance":1e-6,"current_absolute_tolerance":1e-12,"voltage_absolute_tolerance":1e-9,"max_iterations":100,"max_backtracks":20}"#;
        let config: crate::analysis::quasi_periodic::QuasiPeriodicSolveConfig =
            serde_json::from_str(legacy).unwrap();
        assert!(config.linear.is_default());
        assert_eq!(serde_json::to_string(&config).unwrap(), legacy);
    }
}
