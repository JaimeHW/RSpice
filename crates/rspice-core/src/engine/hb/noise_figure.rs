//! Source-referenced single-sideband noise figure around a retained HB state.
//!
//! The source is an ideal voltage generator followed by an explicitly named
//! linear resistor. Its complete folded thermal contribution is normalized to
//! the reference temperature; the circuit's other noise stays at its actual
//! operating temperature. Signal gain uses the selected input and output conversion channels.
//! Thus image-band source noise remains in the numerator (SSB convention).

use super::*;
use crate::netlist::{ElementKind, FlattenerConfig, ParseWithAbortError};

/// Noise measurement and its physical Thevenin source reference.
#[derive(Debug, Clone)]
pub struct HbNoiseFigureRequest {
    pub frequencies: Vec<Value>,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub max_sideband: i32,
    /// Series resistor already present in the authenticated producer circuit.
    pub source_resistor: String,
    /// Reference noise temperature in kelvin, conventionally 290 K.
    pub reference_temperature: Value,
}

/// Noise figure and the resolved physical reference used to calculate it.
#[derive(Debug, Clone, PartialEq)]
pub struct PeriodicNoiseFigureSpectrum {
    pub source_resistor: String,
    pub source_resistance: Value,
    pub source_temperature: Value,
    pub reference_temperature: Value,
    /// 10 log10 of noise factor; aligned with the noise result's frequencies.
    pub decibels: Vec<Value>,
}

/// Ordinary circuit noise plus the reference-temperature SSB noise figure.
#[derive(Debug, Clone)]
pub struct PeriodicNoiseFigureResult {
    /// Actual circuit noise, without altering the retained operating point.
    pub noise: PnoiseAnalysisResult,
    pub figure: PeriodicNoiseFigureSpectrum,
}

fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("HBNOISE source reference: {}", message.into()))
}

impl Engine {
    /// Evaluate an authenticated HB noise spectrum and a resistor-referenced
    /// SSB noise figure. Changing the reference temperature only rescales the
    /// source's thermal noise; it does not change device bias or resistance.
    pub fn run_hb_noise_figure_with_abort(
        &self,
        netlist: &Netlist,
        request: &HbNoiseFigureRequest,
        operating_point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PeriodicNoiseFigureResult, SimulationError> {
        self.run_hb_noise_figure_at_sidebands_with_abort(
            netlist,
            request,
            PeriodicNoiseSidebands::default(),
            operating_point,
            abort,
        )
    }

    /// Calculate SSB noise figure for explicitly selected signal channels.
    /// All source image noise remains in the numerator regardless of which
    /// input channel supplies the signal-gain reference.
    pub fn run_hb_noise_figure_at_sidebands_with_abort(
        &self,
        netlist: &Netlist,
        request: &HbNoiseFigureRequest,
        sidebands: PeriodicNoiseSidebands,
        operating_point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PeriodicNoiseFigureResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !request.reference_temperature.is_finite() || request.reference_temperature <= 0.0 {
            return Err(invalid("reference temperature must be finite and positive"));
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_valid_configuration()?;
        let source_name = engine.validate_noise_figure_source(
            netlist,
            &request.input_source,
            &request.source_resistor,
            abort,
        )?;
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let (resistance, temperature) =
            resolve_noise_figure_resistor(&circuit, &source_name, engine.config.temperature)?;
        let noise = engine.run_pnoise_from_hb_request_with_abort(
            netlist,
            &PeriodicNoiseRequest {
                sampling: None,
                offsets: &request.frequencies,
                output_node: &request.output_node,
                output_ref: request.output_ref.as_deref(),
                input_source: Some(&request.input_source),
                max_sideband: request.max_sideband,
                sidebands,
            },
            operating_point,
            abort,
        )?;
        let input = noise
            .input_noise
            .as_ref()
            .ok_or_else(|| invalid("input-referred spectrum is missing"))?;
        let thermal_label = format!("{source_name} thermal");
        let thermal_index = noise
            .contributors
            .iter()
            .position(|(name, _)| name.eq_ignore_ascii_case(&thermal_label))
            .ok_or_else(|| invalid("source resistor thermal contribution is missing"))?;
        let boltzmann =
            super::pnoise::pnoise_physical_constants(engine.config.spice_dialect).boltzmann;
        engine.ensure_result_shape(
            noise.frequencies.len(),
            noise.contributors.len().saturating_add(4),
        )?;
        let mut decibels = Vec::new();
        decibels
            .try_reserve_exact(noise.frequencies.len())
            .map_err(|error| invalid(format!("noise-figure allocation failed: {error}")))?;
        for (index, (&output, &input)) in noise.output_noise.iter().zip(input).enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Sum contributors directly instead of subtracting the source
            // from a rounded total, which could erase much smaller DUT noise.
            // Log-space summation also avoids overflowing 4*k*T*R or Tref/T.
            let mut log_total = f64::NEG_INFINITY;
            for (source_index, (_, values)) in noise.contributors.iter().enumerate() {
                if source_index % 256 == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let value = values[index];
                if value > 0.0 {
                    let log_value = value.ln()
                        + if source_index == thermal_index {
                            request.reference_temperature.ln() - temperature.ln()
                        } else {
                            0.0
                        };
                    log_total = log_add(log_total, log_value);
                }
            }
            let log_factor = log_total - output.ln() + input.ln()
                - 4.0_f64.ln()
                - boltzmann.ln()
                - request.reference_temperature.ln()
                - resistance.ln();
            let nf = (10.0 / std::f64::consts::LN_10) * log_factor;
            if !nf.is_finite() {
                return Err(invalid(format!(
                    "noise figure is not finite at frequency index {index}"
                )));
            }
            decibels.push(nf);
        }
        Ok(PeriodicNoiseFigureResult {
            noise,
            figure: PeriodicNoiseFigureSpectrum {
                source_resistor: source_name,
                source_resistance: resistance,
                source_temperature: temperature,
                reference_temperature: request.reference_temperature,
                decibels,
            },
        })
    }

    pub(super) fn validate_noise_figure_source(
        &self,
        netlist: &Netlist,
        input_source: &str,
        source_resistor: &str,
        abort: &dyn AbortSignal,
    ) -> Result<String, SimulationError> {
        let flattened = crate::netlist::flatten_netlist_with_models_config_with_abort(
            netlist,
            FlattenerConfig {
                max_depth: self.config.resource_limits.max_hierarchy_depth,
                max_elements: self.config.resource_limits.max_flattened_elements,
                ..Default::default()
            },
            abort,
        )
        .map_err(|error| match error {
            ParseWithAbortError::Aborted => SimulationError::Aborted,
            ParseWithAbortError::Parse(crate::netlist::ParseError::ResourceLimit(error)) => {
                SimulationError::ResourceLimit(error)
            }
            ParseWithAbortError::Parse(error) => invalid(error.to_string()),
        })?;
        let find = |name: &str| {
            flattened
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name.trim()))
        };
        let source =
            find(input_source).ok_or_else(|| invalid("input voltage source was not found"))?;
        let resistor =
            find(source_resistor).ok_or_else(|| invalid("named source resistor was not found"))?;
        if !matches!(&source.kind, ElementKind::VoltageSource(spec) if spec.rf_port().is_none()) {
            return Err(invalid(
                "input must be an ideal voltage source with an explicit series resistor",
            ));
        }
        if !matches!(resistor.kind, ElementKind::Resistor { .. })
            || source.nodes.len() != 2
            || resistor.nodes.len() != 2
        {
            return Err(invalid(
                "source reference must name a two-terminal resistor",
            ));
        }
        // A private generator terminal makes the resistor the actual series
        // source impedance, rather than an arbitrary noisy DUT element.
        let is_private_junction = source.nodes.iter().any(|node| {
            !crate::naming::is_spice_ground_name(node)
                && resistor
                    .nodes
                    .iter()
                    .filter(|other| other.eq_ignore_ascii_case(node))
                    .count()
                    == 1
                && flattened
                    .elements
                    .iter()
                    .filter(|element| {
                        element
                            .nodes
                            .iter()
                            .any(|other| other.eq_ignore_ascii_case(node))
                    })
                    .count()
                    == 2
        });
        if !is_private_junction {
            return Err(invalid(
                "source and resistor must share a private series junction with no other circuit terminals",
            ));
        }
        Ok(resistor.name.clone())
    }
}

pub(super) fn resolve_noise_figure_resistor(
    circuit: &CircuitData,
    source_name: &str,
    ambient: Value,
) -> Result<(Value, Value), SimulationError> {
    let (resistance, temperature, noisy, flicker) = if let Some(index) = circuit
        .resistors
        .names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(source_name))
    {
        if circuit.resistors.thermal[index].is_some() {
            return Err(invalid(
                "a self-heating resistor cannot be the fixed noise reference",
            ));
        }
        (
            1.0 / circuit.resistors.small_signal_conductance(index),
            circuit.resistor_noise_temperature(index, ambient),
            circuit.resistors.noisy[index],
            circuit.resistors.flicker[index],
        )
    } else if let Some(index) = circuit
        .resistor_branches
        .names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(source_name))
    {
        (
            circuit.resistor_branches.small_signal_resistances[index],
            circuit.resistor_branches.noise_temperature(index, ambient),
            circuit.resistor_branches.noisy[index],
            circuit.resistor_branches.flicker[index],
        )
    } else {
        return Err(invalid(
            "source resistor must elaborate to a fixed linear resistance",
        ));
    };
    if !resistance.is_finite()
        || resistance <= 0.0
        || !temperature.is_finite()
        || temperature <= 0.0
    {
        return Err(invalid(
            "source resistance and temperature must be finite and positive",
        ));
    }
    if !noisy || flicker.is_some_and(|noise| noise.coefficient != 0.0) {
        return Err(invalid(
            "source resistor must have thermal noise enabled and no excess flicker noise",
        ));
    }
    Ok((resistance, temperature))
}

fn log_add(a: f64, b: f64) -> f64 {
    let high = a.max(b);
    let low = a.min(b);
    high + (low - high).exp().ln_1p()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> HbNoiseFigureRequest {
        HbNoiseFigureRequest {
            frequencies: vec![1e3, 1e4],
            output_node: "out".into(),
            output_ref: None,
            input_source: "V1".into(),
            max_sideband: 1,
            source_resistor: "Rs".into(),
            reference_temperature: 290.0,
        }
    }

    fn run(
        deck: &str,
        request: &HbNoiseFigureRequest,
    ) -> Result<PeriodicNoiseFigureResult, SimulationError> {
        let netlist = Netlist::parse(deck).unwrap();
        let engine = Engine::default();
        let hb = engine
            .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(8))
            .unwrap();
        engine.run_hb_noise_figure_with_abort(&netlist, request, &hb.operating_point, &NoAbort)
    }

    #[test]
    fn hb_noise_figure_normalizes_source_temperature_without_double_counting() {
        let deck =
            "Noise figure\nV1 in 0 0\nRs in out 1k\nRl out 0 2k\n.options temp=126.85\n.end\n";
        let mut req = request();
        let result = run(deck, &req).unwrap();
        let expected = 10.0 * (1.0_f64 + 400.0 / 290.0 * 1000.0 / 2000.0).log10();
        assert!((result.figure.source_resistance - 1000.0).abs() < 1e-10);
        assert_eq!(result.figure.source_temperature, 400.0);
        assert!(
            result
                .figure
                .decibels
                .iter()
                .all(|nf| (nf - expected).abs() < 1e-10)
        );
        req.reference_temperature = 400.0;
        let same_temperature = run(deck, &req).unwrap();
        assert!(
            same_temperature
                .figure
                .decibels
                .iter()
                .all(|nf| (nf - 10.0 * 1.5_f64.log10()).abs() < 1e-10)
        );
        assert_eq!(
            same_temperature.noise.output_noise,
            result.noise.output_noise
        );
        assert_eq!(same_temperature.noise.input_noise, result.noise.input_noise);
    }

    #[test]
    fn hb_noise_figure_resolves_authored_temperature_and_rejects_wrong_source() {
        let deck = "Noise figure\nV1 in 0 0\nRs in out 1k temp=126.85\nRl out 0 2k\n.options temp=16.85\n.end\n";
        let mut req = request();
        let result = run(deck, &req).unwrap();
        // The resistor primitive's ngspice-compatible explicit TEMP noise
        // convention adds nominal Celsius temperature (27 C here). Use that
        // actual primitive temperature, not an independent frontend guess.
        assert_eq!(result.figure.source_temperature, 427.0);
        assert!(
            result
                .figure
                .decibels
                .iter()
                .all(|nf| (nf - 10.0 * 1.5_f64.log10()).abs() < 1e-10)
        );
        req.source_resistor = "Rl".into();
        assert!(
            run(deck, &req)
                .unwrap_err()
                .to_string()
                .contains("private series junction")
        );
        req.source_resistor = "missing".into();
        assert!(
            run(deck, &req)
                .unwrap_err()
                .to_string()
                .contains("not found")
        );
        req = request();
        req.reference_temperature = 0.0;
        assert!(run(deck, &req).is_err());
    }

    #[test]
    fn hb_noise_figure_noiseless_dut_is_zero_db_and_source_must_be_thermal() {
        let deck = "Noise figure\nV1 in 0 0\nRs in out 1k\nRl out 0 2k noisy=0\n.end\n";
        let result = run(deck, &request()).unwrap();
        assert!(result.figure.decibels.iter().all(|nf| nf.abs() < 1e-10));
        let quiet_source = deck.replace("Rs in out 1k", "Rs in out 1k noisy=0");
        assert!(
            run(&quiet_source, &request())
                .unwrap_err()
                .to_string()
                .contains("thermal noise enabled")
        );
    }

    #[test]
    fn hb_noise_figure_selected_sidebands_follow_absolute_rc_frequency() {
        let netlist = Netlist::parse(
            "RC sidebands\nV1 in 0 0\nRs in out 1k\nRl out 0 1k\nC1 out 0 1n\n.end\n",
        )
        .unwrap();
        let engine = Engine::default();
        let hb = engine
            .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(8))
            .unwrap();
        let req = HbNoiseFigureRequest {
            reference_temperature: 300.15,
            ..request()
        };
        let boltzmann =
            super::super::pnoise::pnoise_physical_constants(engine.config.spice_dialect).boltzmann;
        for sideband in [-1, 0, 1] {
            let result = engine
                .run_hb_noise_figure_at_sidebands_with_abort(
                    &netlist,
                    &req,
                    PeriodicNoiseSidebands {
                        input: sideband,
                        output: sideband,
                    },
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap();
            for (index, offset) in req.frequencies.iter().enumerate() {
                let frequency = offset + f64::from(sideband) * 1e6;
                let expected = 4.0 * boltzmann * 300.15 * 500.0
                    / (1.0 + (std::f64::consts::TAU * frequency * 500.0 * 1e-9).powi(2));
                assert!((result.noise.output_noise[index] / expected - 1.0).abs() < 1e-9);
                assert!((result.figure.decibels[index] - 10.0 * 2.0_f64.log10()).abs() < 1e-9);
            }
        }
        for sidebands in [
            PeriodicNoiseSidebands {
                input: 2,
                output: 0,
            },
            PeriodicNoiseSidebands {
                input: 0,
                output: -2,
            },
            PeriodicNoiseSidebands {
                input: i32::MIN,
                output: 0,
            },
        ] {
            let error = engine
                .run_hb_noise_figure_at_sidebands_with_abort(
                    &netlist,
                    &req,
                    sidebands,
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap_err();
            assert!(error.to_string().contains("folding window"), "{error}");
        }
        let error = engine
            .run_hb_noise_figure_at_sidebands_with_abort(
                &netlist,
                &req,
                PeriodicNoiseSidebands {
                    input: 0,
                    output: 1,
                },
                &hb.operating_point,
                &NoAbort,
            )
            .unwrap_err();
        assert!(error.to_string().contains("zero input-transfer"), "{error}");
    }

    #[test]
    fn hb_noise_figure_keeps_folded_image_noise_in_the_ssb_numerator() {
        // For a memoryless periodically modulated divider, white source
        // noise uses mean(H(t)^2), while the signal uses mean(H(t))^2.
        // Independently integrate that circuit law (including switch thermal
        // noise) to qualify folding without a discontinuous switching limit.
        let netlist = Netlist::parse("SSB chopper\nV1 gen 0 0\nRs gen src 1k\nVlo ctl 0 SIN(0 1 1Meg)\nS1 src out ctl 0 SWMOD\nRl out 0 1k noisy=0\n.model SWMOD SW VT=0 RON=100 ROFF=10k SMOOTH=1\n.end\n").unwrap();
        let engine = Engine::default();
        let hb = engine
            .run_hb(
                &netlist,
                HbConfig::new(1e6).with_harmonics(24).with_oversample(4),
            )
            .unwrap();
        let req = HbNoiseFigureRequest {
            max_sideband: 12,
            ..request()
        };
        for (input, output) in [(0, 0), (1, 0), (-1, 0), (0, 1), (1, 1), (0, -1)] {
            let result = engine
                .run_hb_noise_figure_at_sidebands_with_abort(
                    &netlist,
                    &req,
                    PeriodicNoiseSidebands { input, output },
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap();
            let samples = 100_000;
            let (mut signal_re, mut signal_im, mut noise) = (0.0, 0.0, 0.0);
            for index in 0..samples {
                let phase = std::f64::consts::TAU * (index as f64 + 0.5) / samples as f64;
                let off_fraction = 0.5 * (1.0 - phase.sin().tanh());
                let resistance = (100.0_f64.ln() * (1.0 - off_fraction)
                    + 10_000.0_f64.ln() * off_fraction)
                    .exp();
                let transfer = 1000.0 / (2000.0 + resistance);
                signal_re += transfer * (f64::from(input - output) * phase).cos();
                signal_im += transfer * (f64::from(input - output) * phase).sin();
                noise += (1.0 + resistance / 1000.0 * 300.15 / 290.0) * transfer.powi(2);
            }
            let expected = noise * samples as f64 / (signal_re.powi(2) + signal_im.powi(2));
            for db in result.figure.decibels {
                assert!(
                    (10.0_f64.powf(db / 10.0) - expected).abs() < 0.001 * expected,
                    "SSB noise figure for {input} -> {output}: {db} dB, expected {} dB",
                    10.0 * expected.log10()
                );
            }
        }
    }
}
