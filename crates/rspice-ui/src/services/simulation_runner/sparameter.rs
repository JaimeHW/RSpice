//! S-parameter analysis.
//!
//! Sweeps frequency and extracts the scattering matrix between the declared
//! ports, with the port impedances the run configuration sets.

#![allow(clippy::needless_range_loop)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::s_param;
use rspice_core::engine::Engine;
use rspice_core::netlist::{Element, ElementKind, SourceSpec};
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

/// N-port S-parameter analysis output.
#[derive(Debug, Clone)]
pub struct SParameterData {
    pub frequencies: Vec<Value>,
    /// Number of ports in the solved network.
    pub num_ports: usize,
    /// S-parameter matrix traces indexed as [row][col][frequency_index], 0-based.
    pub s: Vec<Vec<Vec<Complex64>>>,
}

/// Test-only convenience wrapper without a source path.
#[cfg(test)]
pub fn run_sparameter_analysis_with_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SParameterData> {
    run_sparameter_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run N-port S-parameter analysis with source-path resolution and
/// cooperative cancellation through parsing, solving, and matrix conversion.
pub fn run_sparameter_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SParameterData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let parsed_netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    let mut netlist = parsed_netlist;
    let ports = resolve_ports(&mut netlist, config, abort)?;
    let num_ports = ports.len();
    if num_ports < 2 {
        return Err(ServiceRunError::Failure(
            "S-parameter analysis requires at least 2 ports: place RF Port components or name \
             the port nodes in the analysis setup"
                .to_string(),
        ));
    }

    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let run = engine
        .run_sp_over_grid_with_abort(&netlist, &frequencies, false, abort)
        .map_err(|error| ServiceRunError::from_core("S-parameter analysis error", error))?;
    let frequencies = run
        .scattering
        .data
        .iter()
        .map(|point| point.frequency)
        .collect::<Vec<_>>();
    let mut s = vec![vec![Vec::with_capacity(frequencies.len()); num_ports]; num_ports];
    for (index, point) in run.scattering.data.iter().enumerate() {
        poll_periodically(abort, index)?;
        for (row, traces) in s.iter_mut().enumerate() {
            ensure_not_aborted(abort)?;
            for (column, trace) in traces.iter_mut().enumerate() {
                poll_periodically(abort, column)?;
                trace.push(point.get(row + 1, column + 1));
            }
        }
    }

    ensure_not_aborted(abort)?;
    if frequencies.is_empty()
        || frequencies
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency <= 0.0)
        || frequencies.windows(2).any(|pair| pair[1] <= pair[0])
        || s.len() != num_ports
        || s.iter().any(|row| {
            row.len() != num_ports
                || row.iter().any(|trace| {
                    trace.len() != frequencies.len()
                        || trace
                            .iter()
                            .any(|value| !value.re.is_finite() || !value.im.is_finite())
                })
        })
    {
        return Err(ServiceRunError::Failure(
            "S-parameter solver returned an invalid frequency grid or matrix payload".to_owned(),
        ));
    }
    Ok(SParameterData {
        frequencies,
        num_ports,
        s,
    })
}

/// The ports this run measures, materializing them into the netlist if the deck
/// does not declare its own.
///
/// A design drawn with RF Port components carries its ports in the deck, and
/// those win: they hold the authored port numbers and reference impedances, and
/// each is already a generator behind a real Z0. Only a deck that declares
/// nothing falls back to the configuration's node list, which is how a plain
/// schematic gets S-parameters without placing anything.
fn resolve_ports(
    netlist: &mut rspice_core::Netlist,
    config: &SParameterRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<s_param::SParameterPort>> {
    ensure_not_aborted(abort)?;
    match s_param::collect_ports(netlist) {
        Ok(declared) => Ok(declared),
        Err(s_param::PortError::NoPortsDeclared) => inject_configured_ports(netlist, config, abort),
        Err(error) => Err(ServiceRunError::Failure(error.to_string())),
    }
}

/// Add a generator behind a reference impedance at each configured node pair.
///
/// The pair is what a port is. An earlier version put a bare ideal source across
/// the node pair instead, which pins the node to the source value and leaves no
/// reflection to measure — and would have shorted out the reference impedance of
/// any port the deck had declared for itself.
fn inject_configured_ports(
    netlist: &mut rspice_core::Netlist,
    config: &SParameterRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<s_param::SParameterPort>> {
    let mut ports = Vec::with_capacity(config.ports.len());
    for (index, port) in config.ports.iter().enumerate() {
        poll_periodically(abort, index)?;
        let source_name =
            unique_aux_element_name(netlist, &format!("__RSPICE_SP_PORT{}", index + 1), abort)?;
        let resistor_name =
            unique_aux_element_name(netlist, &format!("__RSPICE_SP_Z0_{}", index + 1), abort)?;
        let internal_node = format!("__RSPICE_SP_PORT{}_INT", index + 1);
        let z0 = port.z0.unwrap_or(config.z0);

        netlist.elements.push(Element {
            name: source_name.clone(),
            nodes: vec![internal_node.clone(), port.node_neg.clone()],
            kind: ElementKind::VoltageSource(SourceSpec::RfPort {
                inner: Box::new(SourceSpec::Dc(0.0)),
                port: rspice_core::netlist::SourceRfPort {
                    portnum: index + 1,
                    z0,
                    power: None,
                    frequency: None,
                    phase: None,
                    reference_plane: Some(port.node_pos.clone()),
                },
            }),
            provenance: rspice_core::netlist::ElementProvenance::Authored,
        });
        netlist.elements.push(Element {
            name: resistor_name,
            nodes: vec![port.node_pos.clone(), internal_node],
            kind: ElementKind::Resistor {
                value: z0,
                value_expr: None,
                model: None,
                instance_params: Vec::new(),
                deferred_params: Vec::new(),
            },
            provenance: rspice_core::netlist::ElementProvenance::Authored,
        });

        ports.push(s_param::SParameterPort {
            number: index + 1,
            source_name,
            node_pos: port.node_pos.clone(),
            node_neg: port.node_neg.clone(),
            z0,
            realization: s_param::PortRealization::Thevenin,
        });
    }
    Ok(ports)
}

fn unique_aux_element_name(
    netlist: &rspice_core::Netlist,
    base: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    let name_exists = |candidate: &str| -> ServiceRunResult<bool> {
        for (index, element) in netlist.elements.iter().enumerate() {
            poll_periodically(abort, index)?;
            if element.name.eq_ignore_ascii_case(candidate) {
                return Ok(true);
            }
        }
        Ok(false)
    };

    if !name_exists(base)? {
        return Ok(base.to_string());
    }

    for idx in 1.. {
        ensure_not_aborted(abort)?;
        let candidate = format!("{}_{}", base, idx);
        if !name_exists(&candidate)? {
            return Ok(candidate);
        }
    }
    unreachable!("unbounded iterator should always find a unique name");
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
            assert_eq!(result.frequencies, [config.start_freq]);
            assert_eq!(result.num_ports, 2);
            for row in 0..2 {
                for column in 0..2 {
                    let trace = &result.s[row][column];
                    assert_eq!(trace.len(), 1);
                    let expected = if row == column { 0.2 } else { 0.8 };
                    assert!((trace[0].re - expected).abs() < 1e-10);
                    assert!(trace[0].im.abs() < 1e-10);
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
        let mut netlist =
            rspice_core::Netlist::parse("* divider\nR1 IN OUT 50\nR2 OUT 0 50\n.end\n")
                .expect("deck parses");
        let ports = inject_configured_ports(&mut netlist, &two_port_config(), &NoAbort)
            .expect("ports inject");

        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].z0, 50.0, "an unset port takes the run's default");
        assert_eq!(ports[1].z0, 75.0, "a port's own z0 wins");
        assert_eq!(s_param::collect_ports(&netlist).unwrap(), ports);

        for (index, port) in ports.iter().enumerate() {
            assert_eq!(port.realization, s_param::PortRealization::Thevenin);
            let internal = format!("__RSPICE_SP_PORT{}_INT", index + 1);
            let source = netlist
                .elements
                .iter()
                .find(|element| element.name == port.source_name)
                .expect("generator present");
            assert_eq!(
                source.nodes,
                vec![internal.clone(), port.node_neg.clone()],
                "the generator must sit behind the reference impedance"
            );
            assert!(
                netlist.elements.iter().any(|element| matches!(
                    &element.kind,
                    ElementKind::Resistor { value, .. } if *value == port.z0
                ) && element.nodes
                    == vec![port.node_pos.clone(), internal.clone()]),
                "the reference impedance must reach the port node"
            );
        }
    }

    /// A deck that declares its own ports keeps them. Injecting a second
    /// generator onto a port node would shunt the one already there.
    #[test]
    fn a_deck_with_its_own_ports_is_not_given_injected_ones() {
        let mut netlist = rspice_core::Netlist::parse(
            "* p elements\nP1 IN 0 PORT=1 Z0=50 AC 1\nR1 IN OUT 50\nP2 OUT 0 PORT=2 Z0=75\n.end\n",
        )
        .expect("deck parses");
        let before = netlist.elements.len();

        let ports =
            resolve_ports(&mut netlist, &two_port_config(), &NoAbort).expect("ports resolve");

        assert_eq!(netlist.elements.len(), before, "nothing was injected");
        assert_eq!(
            ports
                .iter()
                .map(|port| port.source_name.as_str())
                .collect::<Vec<_>>(),
            ["P1", "P2"]
        );
        assert_eq!(ports[1].z0, 75.0, "the deck's z0 wins over the dialog's");
    }
}
