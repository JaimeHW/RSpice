//! Recover physical noise ports eliminated by the native BJT's private solve.

use super::*;
use crate::device::semiconductor::{
    BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeSnapshot,
};

struct PrivateBjtNoiseSystem {
    internal: [[Complex64; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    external: [[Complex64; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    nodes: [usize; BJT_EXTERNAL_STATE_DIM],
}

impl PrivateBjtNoiseSystem {
    fn new(bjt: &crate::device::Bjt, solution: &[Value], s: Complex64) -> Self {
        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let [vc, vb, ve, vs] = nodes.map(|node| Engine::noise_node_voltage(solution, node));
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        Self::from_snapshot(bjt, &snapshot, s)
    }

    fn from_snapshot(bjt: &crate::device::Bjt, snapshot: &BjtChargeSnapshot, s: Complex64) -> Self {
        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let charges = Engine::bjt_ac_charge_blocks(snapshot, true);
        // GP private equations use incoming balance, opposite to terminal KCL.
        // The exact same signs and charge derivatives form the AC Schur operator.
        Self {
            internal: std::array::from_fn(|row| {
                std::array::from_fn(|col| {
                    Complex64::new(snapshot.reduction.g_ii[row][col], 0.0)
                        - s * charges.ii[row][col]
                })
            }),
            external: std::array::from_fn(|row| {
                std::array::from_fn(|col| {
                    Complex64::new(snapshot.reduction.g_ei[row][col], 0.0)
                        + s * charges.ei[row][col]
                })
            }),
            nodes,
        }
    }
}

impl Engine {
    /// Append the native model's physical noise mechanisms at one accepted or
    /// unlimited periodic bias. Returns whether a private Schur block is needed.
    pub(in crate::engine) fn append_bjt_noise_sources(
        bjt: &crate::device::Bjt,
        dc_solution: &[Value],
        snapshot: Option<&BjtChargeSnapshot>,
        offset: usize,
        noise_sources: &mut Vec<NoiseSource>,
        absolute_temperatures: &mut HashMap<crate::analysis::NoiseSourceIdentity, Value>,
    ) -> Result<bool, SimulationError> {
        if !bjt.noise_enabled() {
            return Ok(false);
        }
        if let Some(model) = bjt.vbic_noise_operating_model() {
            // Each mechanism retains Q1 as its owner for DNO(Q1) and
            // DNO(Q1,mechanism), regardless of the model's source count.
            for (mechanism, node_pos, node_neg, conductance) in model.thermal {
                let label = format!("{}:{mechanism}", bjt.name);
                if let Some(resistance) =
                    Self::noise_resistance_from_conductance(&label, conductance)?
                {
                    let mut source =
                        NoiseSource::thermal(bjt.name.clone(), node_pos, node_neg, resistance)
                            .with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                                &bjt.name, mechanism,
                            ));
                    if let Some(constants) = model.physical_constants {
                        source = source.with_physical_constants(constants);
                    }
                    if let Some(temperature) = model.absolute_temperature {
                        absolute_temperatures.insert(source.identity.clone(), temperature);
                    } else {
                        // VBIC 1.3's absolute temperature already includes
                        // TRISE and its thermal port. Only older models use
                        // the ngspice analysis-temperature offset here.
                        source.temperature_offset = bjt.noise_temperature_offset;
                    }
                    noise_sources.push(source);
                }
            }
            for (mechanism, node_pos, node_neg, current) in model.shot {
                if current != 0.0 {
                    let mut source =
                        NoiseSource::shot(bjt.name.clone(), node_pos, node_neg, current)
                            .with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                                &bjt.name, mechanism,
                            ));
                    if let Some(constants) = model.physical_constants {
                        source = source.with_physical_constants(constants);
                    }
                    noise_sources.push(source);
                }
            }
            if let Some((kfn, afn, bfn)) = bjt.vbic_flicker_noise_coefficients() {
                let m = bjt.m;
                if !m.is_finite() || m <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "Noise source '{}:FN' has invalid multiplicity {m:e}",
                        bjt.name
                    )));
                }
                for (mechanism, node_pos, node_neg, current, scale) in model.flicker {
                    if (current != 0.0 || model.flicker_current_floor > 0.0) && scale > 0.0 {
                        Self::checked_positive_noise_parameter(
                            &format!("{}:{mechanism}", bjt.name),
                            scale,
                        )?;
                        let total_current = current.abs();
                        let per_copy = total_current / m;
                        let per_copy = if per_copy.is_nan() {
                            per_copy
                        } else {
                            per_copy.max(model.flicker_current_floor)
                        };
                        let (current, multiplicity_factor) =
                            if per_copy.is_normal() || total_current == 0.0 {
                                (per_copy, m)
                            } else {
                                // Retain the total current if division by M
                                // loses range/precision. This alternative can
                                // still represent a finite spectrum (e.g. AFN=0.5).
                                let factor = Self::checked_positive_noise_parameter(
                                    &format!("{}:{mechanism} current normalization", bjt.name),
                                    m.powf(1.0 - afn),
                                )?;
                                (total_current, factor)
                            };
                        let mut source = NoiseSource::flicker_with_frequency_exponent(
                            bjt.name.clone(),
                            node_pos,
                            node_neg,
                            kfn,
                            afn,
                            bfn,
                            current,
                        )
                        .with_identity(
                            crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, mechanism),
                        );
                        // Retain coefficient range through the current
                        // power and frequency law in either representation.
                        (source.parameter, source.parameter_exponent) =
                            crate::numerics::product_binary_normalization(
                                &[kfn, multiplicity_factor, scale],
                                &[],
                            );
                        noise_sources.push(source);
                    }
                }
            }
            return Ok(false);
        }

        let terminals = bjt.legacy_noise_terminals();
        // Substrate-only resistance also owns a private noise block,
        // even when C/B/E all collapse directly onto circuit nodes.
        let has_private_noise = bjt.has_intrinsic_state_unknowns();
        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let node = |terminal: (Option<usize>, Option<usize>)| {
            terminal.0.map_or_else(
                || nodes[terminal.1.expect("physical noise terminal")],
                |index| {
                    if bjt.mna_promoted() {
                        bjt.mna_internal_node(index)
                    } else {
                        offset + index + 1
                    }
                },
            )
        };
        let [collector, base, emitter] = terminals.map(node);
        let noise_internal = if bjt.mna_promoted() {
            Some(bjt.mna_internal_state_at_solution(dc_solution))
        } else {
            snapshot.map(|s| s.reduction.internal_voltages)
        };
        if has_private_noise {
            let voltage = |id| Self::noise_node_voltage(dc_solution, id);
            let internal = noise_internal.unwrap_or_else(|| {
                bjt.charge_snapshot(
                    voltage(nodes[0]),
                    voltage(nodes[1]),
                    voltage(nodes[2]),
                    voltage(nodes[3]),
                )
                .reduction
                .internal_voltages
            });
            for branch in bjt.legacy_private_resistance_noise(internal) {
                let Some(resistance) = Self::noise_resistance_from_conductance(
                    &format!("{}:{}", bjt.name, branch.mechanism),
                    branch.conductance,
                )?
                else {
                    continue;
                };
                let mut source = NoiseSource::thermal(
                    bjt.name.clone(),
                    node(branch.terminals[0]),
                    node(branch.terminals[1]),
                    resistance,
                )
                .with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                    &bjt.name,
                    branch.mechanism,
                ));
                source.temperature_offset = bjt.noise_temperature_offset;
                noise_sources.push(source);
            }
        }
        let (ic, ib, _) = noise_internal.map_or_else(
            || bjt.noise_branch_currents(),
            |snapshot| bjt.legacy_noise_branch_currents_at_state(snapshot),
        );
        if ic != 0.0 {
            noise_sources.push(
                NoiseSource::shot(format!("{}:IC", bjt.name), collector, emitter, ic)
                    .with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                        &bjt.name, "IC",
                    )),
            );
        }
        if ib != 0.0 {
            noise_sources.push(
                NoiseSource::shot(format!("{}:IB", bjt.name), base, emitter, ib).with_identity(
                    crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "IB"),
                ),
            );
        }
        if let Some((kf, af, ef)) = bjt.flicker_noise_coefficients() {
            noise_sources.push(Self::semiconductor_flicker_source(
                NoiseSource::flicker_with_frequency_exponent(
                    bjt.name.clone(),
                    base,
                    emitter,
                    kf,
                    af,
                    ef,
                    ib,
                )
                .with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                    &bjt.name, "FN",
                )),
                bjt.m,
            )?);
        }
        Ok(has_private_noise && !bjt.mna_promoted())
    }

    /// A physical unit current at a private node has reduced RHS Yei*Yii^-1.
    /// Recover its adjoint response without adding unknowns to the circuit.
    /// Ordinary MNA entries remain the prefix; noise-only node blocks follow it.
    pub(in crate::engine) fn bjt_noise_adjoint(
        bjt: &crate::device::Bjt,
        solution: &[Value],
        s: Complex64,
        adjoint: &[Complex64],
    ) -> Result<[Complex64; BJT_INTERNAL_STATE_DIM], SimulationError> {
        let system = PrivateBjtNoiseSystem::new(bjt, solution, s);
        let transpose =
            std::array::from_fn(|row| std::array::from_fn(|col| system.internal[col][row]));
        let rhs = std::array::from_fn(|row| {
            system
                .nodes
                .iter()
                .enumerate()
                .map(|(col, &node)| {
                    Self::noise_transfer_from_adjoint(adjoint, node, 0) * system.external[col][row]
                })
                .sum()
        });
        crate::numerics::solve_small_dense(&transpose, &rhs, BJT_INTERNAL_STATE_DIM)
            .ok_or_else(|| SimulationError::Circuit(format!(
                "BJT '{}' noise private-state adjoint failed at s={s}: singular, nonfinite or unresolved internal system",
                bjt.name,
            )))
    }
}

/// The exact private operators used by a frozen oscillator-noise matrix.
pub(in crate::engine) struct BjtNoiseProjection {
    systems: Vec<(String, PrivateBjtNoiseSystem)>,
}

impl BjtNoiseProjection {
    pub(in crate::engine) fn new(
        circuit: &CircuitData,
        solution: &[Value],
        private_bjts: &[usize],
        s: Value,
        snapshots: &[Option<BjtChargeSnapshot>],
    ) -> Self {
        let s = Complex64::new(s, 0.0);
        Self {
            systems: private_bjts
                .iter()
                .map(|&index| {
                    let bjt = &circuit.bjts.devices[index];
                    let system = match snapshots.get(index).and_then(Option::as_ref) {
                        Some(snapshot) => PrivateBjtNoiseSystem::from_snapshot(bjt, snapshot, s),
                        None => PrivateBjtNoiseSystem::new(bjt, solution, s),
                    };
                    (bjt.name.clone(), system)
                })
                .collect(),
        }
    }

    /// Reduce a real, frozen-time source through the same private equations.
    /// This is the forward counterpart of the stationary noise adjoint and
    /// supplies oscillator-noise probes using their backward-Euler freeze step.
    pub(in crate::engine) fn stamp(
        &self,
        node: usize,
        sign: Value,
        injection: &mut [Value],
    ) -> Result<(), SimulationError> {
        if node == 0 {
            return Ok(());
        }
        if node <= injection.len() {
            injection[node - 1] += sign;
            return Ok(());
        }
        let private = node - injection.len() - 1;
        let (name, system) = self
            .systems
            .get(private / BJT_INTERNAL_STATE_DIM)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "noise source references unknown private BJT node {node}"
                ))
            })?;
        let mut rhs = [Complex64::default(); BJT_INTERNAL_STATE_DIM];
        rhs[private % BJT_INTERNAL_STATE_DIM] = Complex64::new(sign, 0.0);
        let internal =
            crate::numerics::solve_small_dense(&system.internal, &rhs, BJT_INTERNAL_STATE_DIM)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "BJT '{}' noise private-state projection failed",
                        name
                    ))
                })?;
        for (row, &node) in system.nodes.iter().enumerate() {
            if node > 0 {
                let value: Complex64 = system.external[row]
                    .iter()
                    .zip(&internal)
                    .map(|(coefficient, value)| coefficient * value)
                    .sum();
                if !value.re.is_finite() || !value.im.is_finite() || value.im != 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "BJT '{}' has a nonfinite real noise projection",
                        name
                    )));
                }
                injection[node - 1] += value.re;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_base_noise_forward_and_adjoint_match_explicit_frozen_network() {
        let engine = Engine::new(
            crate::SimulationConfig::default()
                .with_spice_dialect(crate::config::SpiceDialect::Ngspice),
        );
        let deck = Netlist::parse("Frozen private base\nVB b 0 0\nVC c 0 0\nQ1 c b 0 qm\n.model qm NPN(IS=0 CJE=1n MJE=0 CJC=2n MJC=0 XCJC=1)\n.end").unwrap();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        // Exercise the direct/programmatic private-state API. Netlist-built
        // variable base resistances now use real matrix nodes instead.
        circuit.bjts.devices[0].rbi = 100.0;
        let solution = vec![0.0; circuit.matrix_size()];
        circuit.set_semiconductor_junction_gmin(0.0);
        circuit.update_nonlinear(&solution);
        let collected = Engine::try_collect_noise_sources(
            &circuit,
            &solution,
            crate::config::SpiceDialect::Ngspice,
        )
        .unwrap();
        let source = collected
            .elementary
            .iter()
            .find(|source| source.identity.mechanism.as_deref() == Some("RB"))
            .unwrap();
        let base = circuit.get_node_by_name("b").unwrap();
        let collector = circuit.get_node_by_name("c").unwrap();
        let mut rhs = vec![0.0; solution.len()];
        let projection =
            BjtNoiseProjection::new(&circuit, &solution, &collected.private_bjts, 1e6, &[]);
        for (node, sign) in [(source.node_pos, 1.0), (source.node_neg, -1.0)] {
            projection.stamp(node, sign, &mut rhs).unwrap();
        }
        // Eliminate a 100-ohm resistor's internal node, loaded by 1n to
        // ground and 2n to collector at s=1e6: Yii=0.01+0.001+0.002.
        assert!((rhs[base - 1] - 3.0 / 13.0).abs() < 1e-12);
        assert!((rhs[collector - 1] + 2.0 / 13.0).abs() < 1e-12);
        for observed in [base, collector] {
            let mut adjoint = vec![Complex64::default(); solution.len()];
            adjoint[observed - 1] = Complex64::new(1.0, 0.0);
            let recovered = Engine::bjt_noise_adjoint(
                &circuit.bjts.devices[0],
                &solution,
                Complex64::new(1e6, 0.0),
                &adjoint,
            )
            .unwrap();
            adjoint.extend_from_slice(&recovered);
            let response =
                Engine::noise_transfer_from_adjoint(&adjoint, source.node_pos, source.node_neg);
            assert!((response - Complex64::new(rhs[observed - 1], 0.0)).norm() < 1e-12);
        }

        // A frozen transient can retain charge at a private voltage different
        // from the DC root. Its depletion derivative must own the projection.
        let bjt = &mut circuit.bjts.devices[0];
        bjt.mje = 0.5;
        bjt.vje = 1.0;
        let mut internal = bjt
            .charge_snapshot(0.0, 0.0, 0.0, 0.0)
            .reduction
            .internal_voltages;
        internal[source.node_neg - solution.len() - 1] = -1.0;
        let frozen = bjt.charge_snapshot_for_dynamic_state(0.0, 0.0, 0.0, 0.0, internal);
        let projection = BjtNoiseProjection::new(
            &circuit,
            &solution,
            &collected.private_bjts,
            1e6,
            &[Some(frozen)],
        );
        rhs.fill(0.0);
        projection.stamp(source.node_pos, 1.0, &mut rhs).unwrap();
        projection.stamp(source.node_neg, -1.0, &mut rhs).unwrap();
        let cbe = 1e-3 / 2.0_f64.sqrt();
        let total = 0.01 + cbe + 0.002;
        assert!((rhs[base - 1] - (cbe + 0.002) / total).abs() < 1e-12);
        assert!((rhs[collector - 1] + 0.002 / total).abs() < 1e-12);
    }
}
