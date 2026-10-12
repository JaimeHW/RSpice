//! The one bias preparation every small-signal analysis linearizes around.
//!
//! AC, S-parameters, the transfer function, AC sensitivity, noise, SP noise,
//! STB, distortion and the adjoint DC sensitivity all need the same thing
//! before their first frequency: a circuit whose every participant holds one
//! accepted operating point. For an analog circuit that is the converged DC
//! solution and the device caches aligned with it. For a circuit with a mixed
//! Verilog-AMS module it is more — the coupled time-zero fixed point has to be
//! *committed* as the held equilibrium (design S1), because every operating-point
//! trial is speculative and rolls back: without the commit the discrete half
//! would be linearized at the state its initial blocks left rather than at the
//! state the bias settled.
//!
//! Each analysis used to carry its own copy of this sequence, and the copies
//! had drifted: some delivered the Verilog-A initialization tasks and some did
//! not, some refused coupled lines and some did not. This is the one copy, and
//! the differences that are analysis semantics rather than drift are keyed by
//! the analysis it is asked for.

use super::{CapabilityAnalysis, Engine, SimulationError, UnsupportedCapabilityError};
use crate::abort_signal::{AbortSignal, ModelRunSignal};
use crate::solver::StaticMatrix;
use crate::{CircuitData, Netlist, Value};

/// Where the bias a small-signal analysis linearizes around comes from.
///
/// The operating point is the only source an analysis card selects today. The
/// accepted transient state at a time T (design S10) joins this enum with the
/// `ACTIMES=` and `CHECKPOINT=` spellings, as two further arms that skip the
/// solve and the commit because that state is already accepted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::engine) enum BiasSource {
    /// The coupled time-zero operating point, committed as the held
    /// equilibrium (design S1).
    OperatingPoint,
}

/// A circuit prepared for linearization, with its matrix structure and the
/// bias every participant holds.
pub(in crate::engine) struct SmallSignalBias {
    pub circuit: CircuitData,
    pub matrix: StaticMatrix,
    pub bias: Vec<Value>,
}

/// The native charge-model checks a small-signal analysis runs, by the name
/// its refusals carry. The adjoint DC sensitivity (physical analysis 0)
/// linearizes static equations only and runs none.
fn dynamic_charge_label(analysis: CapabilityAnalysis, physical: u8) -> Option<&'static str> {
    use CapabilityAnalysis as A;
    match (analysis, physical) {
        (_, 0) => None,
        (A::Sp, 3) => Some("SP noise"),
        (A::Ac | A::Sp | A::Tf | A::Sens, _) => Some("AC"),
        (A::Noise, _) => Some("Noise"),
        (A::Stb, _) => Some("STB"),
        (A::Disto, _) => Some("Distortion"),
        (A::Pz, _) => Some("Pole-zero"),
        _ => None,
    }
}

/// The coupled-line refusal of the analyses that have one: a CPL line has no
/// small-signal load, so the AC family and distortion refuse it instead of
/// returning silently dead ports.
fn coupled_line_token(analysis: CapabilityAnalysis, physical: u8) -> Option<&'static str> {
    use CapabilityAnalysis as A;
    match (analysis, physical) {
        (A::Ac | A::Sp | A::Tf | A::Sens, 1) => {
            Some("analysis.ac.device.coupled_transmission_line")
        }
        (A::Disto, _) => Some("analysis.disto.device.coupled_transmission_line"),
        _ => None,
    }
}

impl Engine {
    /// Prepare the bias `analysis` linearizes around.
    ///
    /// `physical` is the Verilog-A analysis identity the models see while the
    /// bias is solved: 1 for the AC family, 3 for noise, 0 for a static
    /// linearization. `circuit` is the caller's elaborated circuit when it had
    /// to resolve ports or probes first; otherwise one is built.
    ///
    /// The sequence: the capability gate; the Verilog-A analysis begin and its
    /// initialization tasks; the start of digital execution; the device
    /// checks; the operating point; the held-equilibrium commit and its
    /// threshold findings; the accepted operating point of every analog model;
    /// and the device caches aligned with the bias.
    pub(in crate::engine) fn prepare_small_signal_bias(
        &self,
        netlist: &Netlist,
        circuit: Option<CircuitData>,
        analysis: CapabilityAnalysis,
        physical: u8,
        source: BiasSource,
        abort: &dyn AbortSignal,
    ) -> Result<SmallSignalBias, SimulationError> {
        let BiasSource::OperatingPoint = source;
        let run_scope = ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        let mut circuit = match circuit {
            Some(circuit) => circuit,
            None => self.build_circuit_with_abort(netlist, abort)?,
        };
        Self::ensure_analysis_capability(&circuit, analysis)?;
        circuit
            .begin_veriloga_equilibrium_analysis(physical)
            .map_err(SimulationError::Circuit)?;
        Self::deliver_initial_analog_tasks(&mut circuit, abort)?;
        #[cfg(feature = "veriloga")]
        circuit.start_mixed_digital_execution()?;
        if let Some(token) = coupled_line_token(analysis, physical)
            && !circuit.coupled_tlines.is_empty()
        {
            return Err(UnsupportedCapabilityError::new(
                token,
                format!(
                    "{} does not support coupled multiconductor (CPL) transmission lines",
                    if analysis == CapabilityAnalysis::Disto {
                        "Distortion analysis"
                    } else {
                        analysis.label()
                    }
                ),
            )
            .for_analysis_kind(analysis.kind())
            .into());
        }
        match dynamic_charge_label(analysis, physical) {
            Some("AC") => Self::ensure_supported_ac_dynamic_charges(&circuit)?,
            Some(label) => Self::ensure_supported_dynamic_charges(&circuit, label)?,
            None => {}
        }
        circuit
            .prepare_veriloga_equilibrium_analysis_point(physical, true, false)
            .map_err(SimulationError::Circuit)?;
        let mut matrix = if circuit.matrix_size() == 0 {
            Self::model_observation_matrix()?
        } else {
            self.build_matrix(&circuit)?
        };
        circuit.link_indices(&matrix);

        let has_nonlinear = circuit.has_nonlinear_devices();
        let zero_bias_explicit_lines = physical == 1
            && matches!(
                analysis,
                CapabilityAnalysis::Ac
                    | CapabilityAnalysis::Sp
                    | CapabilityAnalysis::Tf
                    | CapabilityAnalysis::Sens
            )
            && circuit.can_use_zero_bias_for_explicit_xspice_ac();
        let bias = if circuit.matrix_size() == 0 {
            Vec::new()
        } else if zero_bias_explicit_lines {
            log::debug!(
                "using zero-bias small-signal state for explicit XSPICE transmission-line AC"
            );
            vec![0.0; circuit.matrix_size()]
        } else {
            self.solve_dc_operating_point_with_abort(netlist, &mut circuit, &mut matrix, abort)?
        };
        if has_nonlinear && !bias.is_empty() {
            self.try_observe_dc_operating_point(&mut circuit, &mut matrix, &bias)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }
        for (_, finding) in self.hold_equilibrium(&mut circuit, &bias)? {
            log::warn!("[mixed.threshold] {}: {finding}", analysis.label());
        }
        self.accept_frequency_operating_point(
            netlist,
            &mut circuit,
            &mut matrix,
            &bias,
            physical,
            abort,
        )?;
        circuit
            .finish_veriloga_equilibrium_operating_point(physical)
            .map_err(SimulationError::Circuit)?;
        circuit.refresh_jiles_atherton_inductances(&bias);
        Self::prepare_small_signal_state(&mut circuit, &bias)?;
        Ok(SmallSignalBias {
            circuit,
            matrix,
            bias,
        })
    }

    /// Commit the converged operating point as the held equilibrium and return
    /// every converter input it holds at a decision boundary (S1, S5(c)), as
    /// `(converter, finding)`.
    ///
    /// The findings are warnings rather than refusals — the held decision is a
    /// consistent answer, and the warning says that the other one was too —
    /// so the caller decides how to report them: once per analysis, or
    /// aggregated per converter over a sweep.
    ///
    /// An XSPICE event model is not committed here: SPICE evaluates every DC
    /// point at time zero, where a sequential code model drives its initial
    /// state (`d_dff` drives `ic` unless set or reset), so it has no state
    /// to carry from one point to the next.
    pub(in crate::engine) fn hold_equilibrium(
        &self,
        circuit: &mut CircuitData,
        bias: &[Value],
    ) -> Result<Vec<(String, String)>, SimulationError> {
        #[cfg(feature = "veriloga")]
        {
            if !circuit.has_mixed_signal_hosts() && !circuit.has_xspice_devices() {
                return Ok(Vec::new());
            }
            if circuit.matrix_size() > 0 {
                circuit.accept_mixed_equilibrium(bias)?;
            }
            Ok(self
                .held_threshold_findings(circuit, bias)
                .into_iter()
                .map(|finding| (finding.converter.clone(), finding.to_string()))
                .collect())
        }
        #[cfg(not(feature = "veriloga"))]
        {
            let _ = (circuit, bias);
            Ok(Vec::new())
        }
    }

    /// The converter inputs the held equilibrium at `bias` holds at a
    /// decision boundary, under this engine's Newton tolerances.
    #[cfg(feature = "veriloga")]
    pub(in crate::engine) fn held_threshold_findings(
        &self,
        circuit: &CircuitData,
        bias: &[Value],
    ) -> Vec<crate::circuit::ThresholdFinding> {
        circuit.mixed_threshold_findings(bias, self.voltage_reltol(), self.voltage_abstol())
    }
}

#[cfg(all(test, feature = "veriloga"))]
mod tests {
    use super::{BiasSource, CapabilityAnalysis};
    use crate::circuit::ThresholdFinding;
    use crate::engine::Engine;

    struct SourceFile(std::path::PathBuf);

    impl SourceFile {
        fn new(name: &str, source: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("rspice_bias_{name}_{}.v", std::process::id()));
            std::fs::write(&path, source).expect("write module");
            Self(path)
        }
    }

    impl Drop for SourceFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn findings(deck: &str) -> Vec<ThresholdFinding> {
        let netlist = crate::Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let bias = engine
            .prepare_small_signal_bias(
                &netlist,
                None,
                CapabilityAnalysis::Ac,
                1,
                BiasSource::OperatingPoint,
                &crate::abort_signal::NoAbort,
            )
            .expect("the held equilibrium is prepared");
        engine.held_threshold_findings(&bias.circuit, &bias.bias)
    }

    fn sampler_deck(module: &SourceFile, level: f64) -> String {
        format!(
            "* converter input at {level} V\n\
             .param vcc=1.8\n\
             vd din 0 {level}\n\
             vac a 0 dc 0 ac 1\n\
             ra a 0 1k\n\
             x1 din q sampler\n\
             rq q 0 1k\n\
             .hdl \"{}\"\n\
             .end\n",
            module.0.display().to_string().replace('\\', "/")
        )
    }

    /// S5(c): an A/D input exactly at its vcc/2 threshold is reported once,
    /// naming the converter and the module port it feeds; 10 mV away it is
    /// not.
    #[test]
    fn a_converter_input_on_its_threshold_is_reported_and_one_off_it_is_not() {
        let module = SourceFile::new(
            "sampler",
            "`timescale 1ns/1ps\n\
             module sampler(din, q);\n\
               input din; output q; reg q;\n\
               initial q = 1'b0;\n\
               always @(din) q = din;\n\
             endmodule\n",
        );
        let on = findings(&sampler_deck(&module, 0.9));
        assert_eq!(on.len(), 1, "{on:?}");
        let finding = &on[0];
        assert!(!finding.converter.is_empty());
        assert!(finding.net.eq_ignore_ascii_case("din"), "{finding}");
        assert!(
            finding
                .host_port
                .as_deref()
                .is_some_and(|port| port.eq_ignore_ascii_case("x1.din")),
            "{finding}"
        );
        assert!(
            matches!(
                finding.kind,
                crate::circuit::ThresholdFindingKind::OnThreshold { threshold }
                    if (threshold - 0.9).abs() < 1.0e-12
            ),
            "{finding}"
        );
        assert!(findings(&sampler_deck(&module, 0.91)).is_empty());
    }

    /// S5(c), the band: an XSPICE converter with an unknown band and no
    /// earlier decision publishes X, and the finding says so.
    #[test]
    fn a_converter_inside_its_unknown_band_is_reported() {
        let band = findings(
            "* converter input inside its unknown band\n\
             vd din 0 0.5\n\
             aadc [din] [dd] adc\n\
             .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
             adac [dd] [q] dac\n\
             .model dac dac_bridge (out_low=0 out_high=1)\n\
             rq q 0 1k\n\
             .end\n",
        );
        assert_eq!(band.len(), 1, "{band:?}");
        assert!(
            matches!(
                band[0].kind,
                crate::circuit::ThresholdFindingKind::UndefinedBand { low, high }
                    if low == 0.4 && high == 0.6
            ),
            "{}",
            band[0]
        );
        assert!(
            band[0].converter.eq_ignore_ascii_case("aadc"),
            "{}",
            band[0]
        );
    }
}
