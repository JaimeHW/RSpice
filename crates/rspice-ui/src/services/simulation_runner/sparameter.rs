//! S-parameter analysis.
//!
//! Sweeps frequency and extracts the scattering matrix between the declared
//! ports, with the port impedances the run configuration sets.

use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::s_param;
use rspice_core::engine::Engine;
use std::path::Path;

/// Sweep type for S-parameter analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SParameterSweep {
    Decade,
    Octave,
    Linear,
}

impl SParameterSweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Port definition for S-parameter analysis.
#[derive(Debug, Clone)]
pub struct SParameterPort {
    pub node_pos: String,
    pub node_neg: String,
    pub z0: Option<Value>,
}

/// Explicit configuration for S-parameter execution.
#[derive(Debug, Clone)]
pub struct SParameterRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: SParameterSweep,
    pub z0: Value,
    pub ports: Vec<SParameterPort>,
}

impl SParameterRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("S-parameter start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err(
                "S-parameter stop frequency must be greater than start frequency".to_string(),
            );
        }
        if self.points_per_unit == 0 {
            return Err("S-parameter points per unit must be greater than zero".to_string());
        }
        if !self.z0.is_finite() || self.z0 <= 0.0 {
            return Err("S-parameter reference impedance must be positive".to_string());
        }
        // How many ports there are is a property of the resolved ports, not of
        // this list: a design drawn with RF Port components declares them in the
        // deck and needs nothing typed here. The count is checked once the two
        // sources have been reconciled.
        for (idx, port) in self.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() {
                return Err(format!(
                    "S-parameter port{} positive node is required",
                    idx + 1
                ));
            }
            if port.node_neg.trim().is_empty() {
                return Err(format!(
                    "S-parameter port{} negative node is required",
                    idx + 1
                ));
            }
            if let Some(port_z0) = port.z0
                && (!port_z0.is_finite() || port_z0 <= 0.0)
            {
                return Err(format!("S-parameter port{} z0 must be positive", idx + 1));
            }
        }
        Ok(())
    }
}

/// Test-only convenience wrapper without a source path.
#[cfg(test)]
pub fn run_sparameter_analysis_with_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<s_param::SParameterResult> {
    run_sparameter_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run N-port S-parameter analysis with source-path resolution and
/// cooperative cancellation through parsing, solving, and matrix conversion.
pub fn run_sparameter_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<s_param::SParameterResult> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    let ports = config
        .ports
        .iter()
        .enumerate()
        .map(|(index, port)| s_param::Port {
            number: index + 1,
            node_pos: port.node_pos.clone(),
            node_neg: port.node_neg.clone(),
            z0: port.z0.unwrap_or(config.z0),
        })
        .collect::<Vec<_>>();
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let run = engine
        .run_sp_over_grid_with_default_ports_and_abort(&netlist, &frequencies, false, &ports, abort)
        .map_err(|error| ServiceRunError::from_core("S-parameter analysis error", error))?;
    ensure_not_aborted(abort)?;
    Ok(run.scattering)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    fn invalid_config() -> SParameterRunConfig {
        SParameterRunConfig {
            start_freq: 0.0,
            stop_freq: 1.0,
            points_per_unit: 0,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: Vec::new(),
        }
    }

    #[test]
    fn sparameter_service_preserves_typed_entry_abort() {
        let result =
            run_sparameter_analysis_with_abort("not a netlist", &invalid_config(), &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    fn two_port_config() -> SParameterRunConfig {
        SParameterRunConfig {
            start_freq: 1e6,
            stop_freq: 1e9,
            points_per_unit: 1,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![
                SParameterPort {
                    node_pos: "IN".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
                SParameterPort {
                    node_pos: "OUT".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: Some(75.0),
                },
            ],
        }
    }

    #[test]
    fn accepted_model_finish_keeps_every_matrix_trace_and_the_actual_grid() {
        let model = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-sp-finish-{}.va", uuid::Uuid::new_v4()));
        std::fs::write(
            &model,
            r#"module sp_finish(p,n);
inout p,n; electrical p,n;
real conductance;
analog begin
    @(initial_step) conductance=0.02;
    @(final_step) conductance=0.04;
    if (analysis("ac") && !analysis("static")) $finish(1);
    I(p,n)<+conductance*V(p,n);
end
endmodule"#,
        )
        .unwrap();
        let mut config = two_port_config();
        config.ports[0].node_pos = " in ".into();
        config.ports[1].node_pos = "out".into();
        config.ports[1].z0 = Some(50.0);
        let deck = format!(
            "* Finished SP\nX1 IN OUT sp_finish\n.va \"{}\" sp_finish\n.end\n",
            model.display().to_string().replace('\\', "/"),
        );
        for declared in [false, true] {
            let source = if declared {
                deck.replace(
                    ".end",
                    "V1 IN 0 AC 1 portnum=1 z0=50\nV2 OUT 0 portnum=2 z0=50\n.end",
                )
            } else {
                deck.clone()
            };
            let result = run_sparameter_analysis_with_abort(&source, &config, &NoAbort).unwrap();
            assert_eq!(result.data.len(), 1);
            assert_eq!(result.data[0].frequency, config.start_freq);
            assert_eq!(result.num_ports, 2);
            for row in 0..2 {
                for column in 0..2 {
                    let value = result.data[0].get(row + 1, column + 1);
                    let expected = if row == column { 0.2 } else { 0.8 };
                    assert!((value.re - expected).abs() < 1e-10);
                    assert!(value.im.abs() < 1e-10);
                }
            }
        }
        std::fs::remove_file(model).unwrap();
    }

    /// A configured port becomes a generator *behind* its reference impedance.
    ///
    /// The earlier form put a bare ideal source across the node pair, which
    /// holds the node at the source value however the network responds — so
    /// there is no reflected wave to measure.
    #[test]
    fn configured_ports_are_injected_behind_their_reference_impedance() {
        let result = run_sparameter_analysis_with_abort(
            "* divider\nR1 IN OUT 50\n.end\n",
            &two_port_config(),
            &NoAbort,
        )
        .unwrap();
        assert_eq!(result.ports.len(), 2);
        assert_eq!(result.ports[0].z0, 50.0);
        assert_eq!(result.ports[1].z0, 75.0);
        for point in result.data {
            assert!((point.s11().re - 75.0 / 175.0).abs() < 1e-12);
            assert!((point.s22().re - 25.0 / 175.0).abs() < 1e-12);
            assert!((point.s21().re - 2.0 * (50.0_f64 * 75.0).sqrt() / 175.0).abs() < 1e-12);
        }
    }

    /// A deck that declares its own ports keeps them. Injecting a second
    /// generator onto a port node would shunt the one already there.
    #[test]
    fn a_deck_with_its_own_ports_is_not_given_injected_ones() {
        for declaration in [
            "P1 p 0 PORT=1 Z0=75",
            "X1 p 0 generator\n.subckt generator a b params: reference=75\nP1 a b portnum=1 z0={reference}\n.ends generator",
        ] {
            let deck = format!("* declared single port\n{declaration}\nR1 p 0 100\n.end\n");
            // The configured planes do not even occur in this circuit.
            let result =
                run_sparameter_analysis_with_abort(&deck, &two_port_config(), &NoAbort).unwrap();
            assert_eq!(result.num_ports, 1);
            assert_eq!(result.ports[0].z0, 75.0);
            for point in result.data {
                assert!((point.s11().re - 1.0 / 7.0).abs() < 1e-12);
            }
        }
    }
}
