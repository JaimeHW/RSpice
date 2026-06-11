use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::advanced) fn noise_node_voltage(
        voltages: &[Value],
        node: usize,
    ) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub(in crate::engine::advanced) fn differential_noise_output(
        solution: &[Complex64],
        output_pos: usize,
        output_neg: Option<usize>,
        num_nodes: usize,
    ) -> Value {
        let v_pos = if output_pos > 0 && output_pos <= num_nodes {
            solution[output_pos - 1]
        } else {
            Complex64::new(0.0, 0.0)
        };
        let v_neg = match output_neg {
            Some(node) if node > 0 && node <= num_nodes => solution[node - 1],
            _ => Complex64::new(0.0, 0.0),
        };
        (v_pos - v_neg).norm()
    }

    pub(in crate::engine::advanced) fn collect_noise_sources(
        circuit: &CircuitData,
        dc_solution: &[Value],
    ) -> Vec<NoiseSource> {
        let mut noise_sources = Vec::new();

        // Thermal noise from resistors (4kT/R).
        for (i, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let conductance = circuit.resistors.small_signal_conductance(i);
            let resistance = if conductance.abs() > 0.0 {
                1.0 / conductance
            } else {
                f64::INFINITY
            };
            if resistance <= 0.0 || !resistance.is_finite() || resistance >= 1e12 {
                continue;
            }

            let name = circuit
                .resistors
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("R{}", i + 1));
            noise_sources.push(NoiseSource::thermal(
                name,
                stamp.pp.row,
                stamp.nn.row,
                resistance,
            ));
        }

        // Shot noise from diodes (2qI).
        for diode in &circuit.diodes.devices {
            let vd = Self::noise_node_voltage(dc_solution, diode.node_anode)
                - Self::noise_node_voltage(dc_solution, diode.node_cathode);
            let id = diode.current(vd);
            if id.abs() > 1e-15 {
                noise_sources.push(NoiseSource::shot(
                    diode.name.clone(),
                    diode.node_anode,
                    diode.node_cathode,
                    id,
                ));
            }
        }

        // BJT collector/base shot noise and model-card flicker noise.
        for bjt in &circuit.bjts.devices {
            let (ic, ibe, ibc) = bjt.noise_branch_currents();
            if ic > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IC", bjt.name),
                    bjt.node_collector,
                    bjt.node_emitter,
                    ic,
                ));
            }
            if ibe > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IBE", bjt.name),
                    bjt.node_base,
                    bjt.node_emitter,
                    ibe,
                ));
            }
            if ibc > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IBC", bjt.name),
                    bjt.node_base,
                    bjt.node_collector,
                    ibc,
                ));
            }

            if let Some((kf, af, ef)) = bjt.flicker_noise_coefficients() {
                let (_, ib, _) = bjt.operating_point_currents();
                if ib.abs() > 1e-18 {
                    noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", bjt.name),
                        bjt.node_base,
                        bjt.node_emitter,
                        kf,
                        af,
                        ef,
                        ib,
                    ));
                }
            }

            // VBIC base-emitter flicker noise (vbicnoise.c FLBENOIZ):
            // S = m·KFN·|Ibe/m|^AFN / f^BFN across the intrinsic B-E
            // junction, where Ibe is the m-folded junction current. The
            // multiplicity folds into the coefficient as KFN·m^(1−AFN);
            // injection lands on the promoted internal nodes so the base
            // and emitter resistances shape the transfer like the oracle.
            if let Some((kfn, afn, bfn)) = bjt.vbic_flicker_noise_coefficients() {
                let (_, ibe, _) = bjt.noise_branch_currents();
                if ibe > 1e-18 {
                    let m = bjt.m.max(1.0);
                    let (node_pos, node_neg) = if bjt.vbic_mna_promoted() {
                        (bjt.node_bi, bjt.node_ei)
                    } else {
                        (bjt.node_base, bjt.node_emitter)
                    };
                    noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", bjt.name),
                        node_pos,
                        node_neg,
                        kfn * m.powf(1.0 - afn),
                        afn,
                        bfn,
                        ibe,
                    ));
                }
            }
        }

        // MOS channel thermal noise and 1/f noise.
        for mos in &circuit.mosfets.devices {
            let gm = mos.transconductance();
            let gamma = mos.channel_thermal_noise_gamma();
            if gm > 1e-18 && gamma > 0.0 {
                let resistance = 1.0 / (gamma * gm).max(1e-30);
                noise_sources.push(NoiseSource::thermal(
                    format!("{}:thermal", mos.name),
                    mos.node_drain,
                    mos.node_source,
                    resistance,
                ));
            }

            if let Some((kf, af, ef)) = mos.flicker_noise_coefficients() {
                let id = mos.drain_current();
                if id.abs() > 1e-18 {
                    noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", mos.name),
                        mos.node_drain,
                        mos.node_source,
                        kf,
                        af,
                        ef,
                        id,
                    ));
                }
            }
        }

        // JFET channel thermal noise, gate shot noise, and flicker noise.
        for jfet in &circuit.jfets {
            let vd = Self::noise_node_voltage(dc_solution, jfet.drain);
            let vg = Self::noise_node_voltage(dc_solution, jfet.gate);
            let vs = Self::noise_node_voltage(dc_solution, jfet.source);
            let vgs = vg - vs;
            let vds = vd - vs;
            let vgd = vg - vd;
            let temp = jfet.params.tnom;
            let (ids, gm, _) = jfet.calculate(vgs, vds, temp);
            if gm.abs() > 1e-18 {
                let resistance = 1.0 / ((2.0 / 3.0) * gm.abs()).max(1e-30);
                noise_sources.push(NoiseSource::thermal(
                    format!("{}:thermal", jfet.name),
                    jfet.drain,
                    jfet.source,
                    resistance,
                ));
            }

            let (igs, igd) = jfet.gate_current(vgs, vgd, temp);
            if igs.abs() > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IGS", jfet.name),
                    jfet.gate,
                    jfet.source,
                    igs,
                ));
            }
            if igd.abs() > 1e-18 {
                noise_sources.push(NoiseSource::shot(
                    format!("{}:IGD", jfet.name),
                    jfet.gate,
                    jfet.drain,
                    igd,
                ));
            }

            if let Some((kf, af, ef)) = jfet.flicker_noise_coefficients()
                && ids.abs() > 1e-18
            {
                noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                    format!("{}:flicker", jfet.name),
                    jfet.drain,
                    jfet.source,
                    kf,
                    af,
                    ef,
                    ids,
                ));
            }
        }

        noise_sources
    }

    /// Run noise analysis
    ///
    /// Computes thermal, shot, and flicker noise at each frequency point.
    /// Returns integrated noise results.
    pub fn run_noise(
        &self,
        netlist: &Netlist,
        output_node: usize,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_ports(netlist, output_node, None, frequencies, temperature)
    }

    /// Run noise analysis with optional differential output reference and
    /// explicit input source for input-referred normalization.
    pub fn run_noise_with_input_source(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        input_source: &str,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            output_pos,
            output_neg,
            Some(input_source),
            frequencies,
            temperature,
        )
    }

    /// Run noise analysis with optional differential output reference.
    ///
    /// The measured output noise is based on:
    /// - `V(output_pos)` when `output_neg` is `None`
    /// - `V(output_pos) - V(output_neg)` when `output_neg` is provided
    pub fn run_noise_ports(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            output_pos,
            output_neg,
            None,
            frequencies,
            temperature,
        )
    }

    pub(in crate::engine::advanced) fn run_noise_internal(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        input_source: Option<&str>,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        #[derive(Clone, Copy)]
        enum InputExcitation {
            VoltageSource { branch_matrix_index: usize },
            CurrentSource { node_pos: usize, node_neg: usize },
        }

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point for bias-dependent noise.
        let dc_solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit.prepare_behavioral_small_signal(&dc_solution);
        let noise_sources = Self::collect_noise_sources(&circuit, &dc_solution);

        // Compute noise at each frequency
        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        if output_pos > num_nodes {
            return Err(SimulationError::Circuit(format!(
                "Invalid node for noise analysis: output_pos={} (max={})",
                output_pos, num_nodes
            )));
        }
        if let Some(node) = output_neg {
            if node > num_nodes {
                return Err(SimulationError::Circuit(format!(
                    "Invalid node for noise analysis: output_neg={} (max={})",
                    node, num_nodes
                )));
            }
            if node == output_pos {
                return Err(SimulationError::Circuit(
                    "Invalid noise output port: output_pos and output_neg cannot be the same"
                        .to_string(),
                ));
            }
        }

        let input_excitation = match input_source {
            None => None,
            Some(source_name) => {
                if let Some(voltage_idx) = circuit
                    .voltage_sources
                    .names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(source_name))
                {
                    let branch_ordinal = circuit.voltage_sources.branch_indices[voltage_idx];
                    let branch_matrix_index = circuit.get_branch_matrix_index(branch_ordinal) - 1;
                    Some(InputExcitation::VoltageSource {
                        branch_matrix_index,
                    })
                } else if let Some(current_idx) = circuit
                    .current_sources
                    .names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(source_name))
                {
                    Some(InputExcitation::CurrentSource {
                        node_pos: circuit.current_sources.node_pos[current_idx],
                        node_neg: circuit.current_sources.node_neg[current_idx],
                    })
                } else {
                    return Err(SimulationError::Circuit(format!(
                        "Noise input source '{}' not found (expected independent V/I source)",
                        source_name
                    )));
                }
            }
        };

        // One excitation vector reused across every source and frequency:
        // each use zeroes it and sets one or two entries.
        let mut rhs = vec![Complex64::new(0.0, 0.0); size];
        let results: Result<Vec<NoiseResult>, SimulationError> = frequencies
            .iter()
            .map(|&freq| {
                let omega = 2.0 * PI * freq;
                let mut ac_matrix =
                    Self::build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, omega);

                let input_gain_sq = if let Some(excitation) = input_excitation {
                    rhs.fill(Complex64::new(0.0, 0.0));
                    match excitation {
                        InputExcitation::VoltageSource {
                            branch_matrix_index,
                        } => {
                            if branch_matrix_index < rhs.len() {
                                rhs[branch_matrix_index] = Complex64::new(1.0, 0.0);
                            }
                        }
                        InputExcitation::CurrentSource { node_pos, node_neg } => {
                            if node_pos > 0 && node_pos <= num_nodes {
                                rhs[node_pos - 1] -= Complex64::new(1.0, 0.0);
                            }
                            if node_neg > 0 && node_neg <= num_nodes {
                                rhs[node_neg - 1] += Complex64::new(1.0, 0.0);
                            }
                        }
                    }

                    let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                    let gain = Self::differential_noise_output(
                        &solution, output_pos, output_neg, num_nodes,
                    );
                    gain * gain
                } else {
                    1.0
                };

                if input_excitation.is_some() && (!input_gain_sq.is_finite() || input_gain_sq <= 1e-30)
                {
                    return Err(SimulationError::Circuit(format!(
                        "Input-referred noise is undefined for source '{}' at {} Hz because the small-signal transfer to the selected output is zero or non-finite",
                        input_source.unwrap_or("<unknown>"),
                        freq
                    )));
                }

                let mut total_noise_v2_hz = 0.0;
                let mut contributions = Vec::new();

                for source in &noise_sources {
                    let si = source.spectral_density(freq, temperature);
                    if !si.is_finite() || si <= 0.0 {
                        continue;
                    }

                    rhs.fill(Complex64::new(0.0, 0.0));
                    if source.node_pos > 0 && source.node_pos <= num_nodes {
                        rhs[source.node_pos - 1] += Complex64::new(1.0, 0.0);
                    }
                    if source.node_neg > 0 && source.node_neg <= num_nodes {
                        rhs[source.node_neg - 1] -= Complex64::new(1.0, 0.0);
                    }

                    let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                    let v_out = Self::differential_noise_output(
                        &solution, output_pos, output_neg, num_nodes,
                    );
                    let output_v2 = si * v_out * v_out;
                    if output_v2.is_finite() && output_v2 > 0.0 {
                        total_noise_v2_hz += output_v2;
                        contributions.push(NoiseContribution {
                            device_name: source.device_name.clone(),
                            noise_type: source.noise_type,
                            output_contribution: output_v2,
                            percentage: 0.0,
                        });
                    }
                }

                for contrib in &mut contributions {
                    contrib.percentage = if total_noise_v2_hz > 0.0 {
                        100.0 * contrib.output_contribution / total_noise_v2_hz
                    } else {
                        0.0
                    };
                }

                Ok(NoiseResult {
                    frequency: freq,
                    output_noise_density: total_noise_v2_hz,
                    input_referred_density: if input_excitation.is_some() {
                        total_noise_v2_hz / input_gain_sq
                    } else {
                        total_noise_v2_hz
                    },
                    contributions,
                })
            })
            .collect();

        results
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Engine;
    use crate::Netlist;

    /// VBIC KFN/AFN/BFN flicker noise must ride the intrinsic B-E junction
    /// with vbicnoise.c's multiplicity folding: `m·KFN·|Ibe/m|^AFN / f^BFN`,
    /// i.e. an effective coefficient of `KFN·m^(1−AFN)` on the m-folded
    /// junction current. The regression deck only exercises AFN=1 (where m
    /// cancels), so the folding is pinned here at AFN≠1.
    #[test]
    fn vbic_flicker_source_follows_vbicnoise_multiplicity_folding() {
        let deck = "\
VBIC flicker source construction

V1 VCC 0 3.3
VIN B 0 DC 0.8
RC VCC C 1k
RE E 0 100
Q1 C B E 0 N1 M=3

.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3
+ CJE=1e-13 CJC=2e-14 TF=10e-12 TR=100e-12
+ KFN=2e-14 AFN=1.5 BFN=0.8

.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");
        circuit.update_nonlinear(&solution);

        let sources = Engine::collect_noise_sources(&circuit, &solution);
        let flicker = sources
            .iter()
            .find(|source| source.device_name.ends_with(":flicker"))
            .expect("VBIC KFN card must produce a flicker source");

        let m: f64 = 3.0;
        let kfn = 2e-14;
        let afn = 1.5;
        let bfn = 0.8;
        let expected_coefficient = kfn * m.powf(1.0 - afn);
        assert!(
            (flicker.parameter - expected_coefficient).abs() <= 1e-9 * expected_coefficient,
            "coefficient must fold multiplicity as KFN*m^(1-AFN): got {:e}, want {:e}",
            flicker.parameter,
            expected_coefficient,
        );
        assert_eq!(flicker.af, afn);
        assert_eq!(flicker.ef, bfn);
        assert!(
            flicker.current > 1e-12,
            "flicker rides the m-folded forward B-E junction current, got {:e}",
            flicker.current,
        );

        // Injection lands on the promoted internal base/emitter nodes, which
        // exist apart from the externals because RBI and RE are nonzero.
        let bjt = &circuit.bjts.devices[0];
        assert_eq!(flicker.node_pos, bjt.node_bi);
        assert_eq!(flicker.node_neg, bjt.node_ei);
        assert_ne!(flicker.node_pos, bjt.node_base);
        assert_ne!(flicker.node_neg, bjt.node_emitter);
    }
}
