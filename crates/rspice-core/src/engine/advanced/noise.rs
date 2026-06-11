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

        // Verilog-A white_noise()/flicker_noise() sources, with PSDs
        // evaluated at the operating point. Potential-contribution noise
        // arrives as a series EMF on the branch-equation row, which is an
        // ordinary system unknown here, so both kinds inject the same way.
        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            let mut probe = device.clone();
            let instance = probe.name.clone();
            for source in probe.noise_sources(dc_solution) {
                let name = format!("{instance}:{}", source.name);
                noise_sources.push(match source.exponent {
                    None => {
                        NoiseSource::white(name, source.node_pos, source.node_neg, source.psd)
                    }
                    Some(ef) => NoiseSource::flicker_psd(
                        name,
                        source.node_pos,
                        source.node_neg,
                        source.psd,
                        ef,
                    ),
                });
            }
        }

        // Resistor thermal noise (4kT/R) and model-card flicker noise
        // (resnoise.c), both gated by the per-instance `noisy` switch.
        for (i, stamp) in circuit.resistors.stamps.iter().enumerate() {
            if !circuit.resistors.noisy.get(i).copied().unwrap_or(true) {
                continue;
            }
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
            let mut source =
                NoiseSource::thermal(name.clone(), stamp.pp.row, stamp.nn.row, resistance);
            source.temperature_offset = circuit.resistors.noise_temperature_offset(i);
            noise_sources.push(source);

            if let Some(&Some((coefficient, af, ef))) = circuit.resistors.flicker.get(i) {
                let v_pos = Self::noise_node_voltage(dc_solution, stamp.pp.row);
                let v_neg = Self::noise_node_voltage(dc_solution, stamp.nn.row);
                let current = circuit.resistors.conductances.get(i).copied().unwrap_or(0.0)
                    * (v_pos - v_neg);
                if current.abs() > 1e-18 {
                    noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", name),
                        stamp.pp.row,
                        stamp.nn.row,
                        coefficient,
                        af,
                        ef,
                        current,
                    ));
                }
            }
        }

        // Diode shot noise (2qI) and KF flicker, both across the junction
        // (the builder externalizes RS onto an internal anode node, so
        // node_anode is the junction side and the series resistance already
        // contributes thermal noise through the resistor walk above —
        // dionoise.c's source set exactly). Flicker follows dionoise.c:
        // m·KF·|Id/m|^AF / f, with the multiplicity folded into the
        // coefficient as KF·m^(1−AF) on the folded junction current.
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
            if let Some((kf, af)) = diode.flicker_noise_coefficients()
                && id.abs() > 1e-15
            {
                let m = diode.multiplicity.max(1.0);
                noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                    format!("{}:flicker", diode.name),
                    diode.node_anode,
                    diode.node_cathode,
                    kf * m.powf(1.0 - af),
                    af,
                    1.0,
                    id.abs(),
                ));
            }
        }

        // BJT noise. Promoted VBIC instances follow vbicnoise.c on the
        // internal topology: thermal noise from the operating-point
        // conductance of every parasitic resistance, shot noise on the
        // transport and junction branch currents, and KFN flicker on the
        // intrinsic and parasitic B-E junctions with the multiplicity
        // folded as m·KFN·|I/m|^AFN / f^BFN (an effective coefficient of
        // KFN·m^(1−AFN) on the m-folded branch current). Legacy GP keeps
        // the external-node shot and KF flicker sources.
        for bjt in &circuit.bjts.devices {
            if let Some(model) = bjt.vbic_noise_operating_model() {
                for (suffix, node_pos, node_neg, conductance) in model.thermal {
                    if conductance.is_finite() && conductance > 1e-30 {
                        let mut source = NoiseSource::thermal(
                            format!("{}:{}", bjt.name, suffix),
                            node_pos,
                            node_neg,
                            1.0 / conductance,
                        );
                        source.temperature_offset = bjt.noise_temperature_offset;
                        noise_sources.push(source);
                    }
                }
                for (suffix, node_pos, node_neg, current) in model.shot {
                    if current.abs() > 1e-18 {
                        noise_sources.push(NoiseSource::shot(
                            format!("{}:{}", bjt.name, suffix),
                            node_pos,
                            node_neg,
                            current,
                        ));
                    }
                }
                if let Some((kfn, afn, bfn)) = bjt.vbic_flicker_noise_coefficients() {
                    let m = bjt.m.max(1.0);
                    let coefficient = kfn * m.powf(1.0 - afn);
                    let (bi, ei, ibe) = model.flicker_ibe;
                    if ibe.abs() > 1e-18 {
                        noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                            format!("{}:flicker", bjt.name),
                            bi,
                            ei,
                            coefficient,
                            afn,
                            bfn,
                            ibe.abs(),
                        ));
                    }
                    let (bx, bp, ibep) = model.flicker_ibep;
                    if ibep.abs() > 1e-18 {
                        noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                            format!("{}:flicker_bep", bjt.name),
                            bx,
                            bp,
                            coefficient,
                            afn,
                            bfn,
                            ibep.abs(),
                        ));
                    }
                }
                continue;
            }

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
        }

        // MOS channel thermal noise and 1/f noise.
        for mos in &circuit.mosfets.devices {
            let gm = mos.transconductance();
            let gamma = mos.channel_thermal_noise_gamma();
            if gm > 1e-18 && gamma > 0.0 {
                let resistance = 1.0 / (gamma * gm).max(1e-30);
                let mut source = NoiseSource::thermal(
                    format!("{}:thermal", mos.name),
                    mos.node_drain,
                    mos.node_source,
                    resistance,
                );
                source.temperature_offset = mos.noise_temperature_offset;
                noise_sources.push(source);
            }

            // SPICE NLEV flicker laws (mos1noi.c; NLEV defaults to 2, whose
            // gm²-based density is bias-dependent through the coefficient
            // rather than the current term).
            if let Some((coefficient, current, af, ef)) = mos.flicker_noise_source_terms()
                && coefficient > 0.0
                && current.abs() > 1e-18
            {
                noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                    format!("{}:flicker", mos.name),
                    mos.node_drain,
                    mos.node_source,
                    coefficient,
                    af,
                    ef,
                    current,
                ));
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
                let mut source = NoiseSource::thermal(
                    format!("{}:thermal", jfet.name),
                    jfet.drain,
                    jfet.source,
                    resistance,
                );
                source.temperature_offset = jfet.noise_dtemp;
                noise_sources.push(source);
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

            // jfetnoi.c rides flicker on the per-finger channel current with
            // an explicit multiplicity factor: m·KF·|cd|^AF / f. This model
            // folds m into beta, so the per-finger current and the factor
            // recombine into KF·m^(1−AF) on the folded current — exact at
            // AF=1, which is why the bare coefficient never showed.
            if let Some((kf, af, ef)) = jfet.flicker_noise_coefficients()
                && ids.abs() > 1e-18
            {
                let m = jfet.m.max(1e-12);
                noise_sources.push(NoiseSource::flicker_with_frequency_exponent(
                    format!("{}:flicker", jfet.name),
                    jfet.drain,
                    jfet.source,
                    kf * m.powf(1.0 - af),
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

    /// onoise_spectrum table of [`RB_NOISE_DECK`] from the official
    /// ngspice-46 binary, in its default root-spectral-density units.
    const RB_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/vbic_noise_rb_ngspice46.dat");

    /// A low-impedance-driven CE stage with large RBX/RBI: the parasitic
    /// base-resistance thermal sources dominate the output noise, so this
    /// deck is blind to nothing the vbicnoise.c port added — unlike the
    /// shipped regression deck, whose 100k network swamps them.
    const RB_NOISE_DECK: &str = "\
VBIC base resistance noise testbench

V1 VCC 0 5
VIN B 0 DC 0.78 AC 1
RC VCC C 1k
Q1 C B 0 0 N1

.OPTIONS NOACCT

.NOISE v(c) VIN DEC 5 100k 10Meg

.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 RCX=10
+ RCI=60 RBX=100 RBI=400 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3
+ CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2 GAMM=2e-11 HRCF=2
+ QCO=1e-12 TF=10e-12 TR=100e-12

.END
";

    /// onoise_spectrum table of [`DIODE_FLICKER_DECK`] from the official
    /// ngspice-46 binary, in its default root-spectral-density units.
    const DIODE_FLICKER_ORACLE: &str =
        include_str!("../../../tests/testdata/diode_flicker_ngspice46.dat");

    /// Forward-biased diode with KF flicker at AF=1.3 and instance M=3:
    /// the low-frequency rows are flicker-lifted above the white floor,
    /// and the non-unity AF makes the multiplicity folding observable.
    const DIODE_FLICKER_DECK: &str = "\
Diode flicker noise testbench

V1 IN 0 DC 0.7 AC 1
R1 IN A 100
D1 A 0 DM M=3

.OPTIONS NOACCT

.NOISE v(a) V1 DEC 5 10 100k

.MODEL DM D IS=1e-14 N=1.8 RS=5 CJO=2e-12 KF=1e-12 AF=1.3

.END
";

    /// Diode KF flicker and the externalized-RS source set must reproduce
    /// the official binary, including the m-folding at AF != 1.
    #[test]
    fn diode_flicker_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(DIODE_FLICKER_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("a").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            5,
            10.0,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "V1", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = DIODE_FLICKER_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "frequency grids must match");

        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            assert!(
                (result.frequency - freq_ref).abs() <= 1e-6 * freq_ref,
                "sweep grid diverged from the oracle at {:e}",
                freq_ref,
            );
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise tables of [`MOS_FLICKER_DECK`] (default NLEV) and its NLEV=0
    /// variant from the official ngspice-46 binary.
    const MOS_FLICKER_NLEV2_ORACLE: &str =
        include_str!("../../../tests/testdata/mos_flicker_nlev2_ngspice46.dat");
    const MOS_FLICKER_NLEV0_ORACLE: &str =
        include_str!("../../../tests/testdata/mos_flicker_nlev0_ngspice46.dat");

    /// A common-source stage whose low-frequency output noise is flicker
    /// dominated, with AF=1.2 and instance M=2 so both the geometry
    /// normalization and the multiplicity recombination under the folded
    /// width are oracle-observable. The default card exercises the NLEV=2
    /// gm²-law mos1set.c selects when NLEV is not given.
    const MOS_FLICKER_DECK: &str = "\
MOS flicker noise testbench

V1 VDD 0 5
VIN G 0 DC 1.5 AC 1
RD VDD D 10k
M1 D G 0 0 NM W=20u L=2u M=2

.OPTIONS NOACCT

.NOISE v(d) VIN DEC 5 10 100k

.MODEL NM NMOS LEVEL=1 VTO=1.0 KP=60u TOX=20n LD=0.2u
+ CGSO=2e-10 CGDO=2e-10 KF=2e-26 AF=1.2

.END
";

    fn assert_noise_matches_oracle(deck: &str, oracle_table: &str, label: &str) {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("d").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            5,
            10.0,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = oracle_table
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "{label}: grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "{label}: onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise table of [`GP_RCRE_NOISE_DECK`] from the official ngspice-46
    /// binary.
    const GP_RCRE_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/gp_rcre_noise_ngspice46.dat");

    /// A Gummel-Poon CE stage with RB=0 and dominant collector/emitter
    /// resistances: the builder externalizes RC and RE onto real internal
    /// nodes, so their thermal noise and the prime-node shot injection are
    /// the whole story this deck tells. The base-prime promotion (RB > 0)
    /// is the remaining GP noise increment and is deliberately absent here.
    const GP_RCRE_NOISE_DECK: &str = "\
GP collector emitter resistance noise testbench

V1 VCC 0 5
VIN B 0 DC 0.78 AC 1
RC VCC C 1k
Q1 C B 0 QN

.OPTIONS NOACCT

.NOISE v(c) VIN DEC 5 10k 100Meg

.MODEL QN NPN IS=1e-16 BF=100 BR=2 RB=0 RC=200 RE=50
+ CJE=2e-12 CJC=1e-12 TF=3e-10 TR=5e-9

.END
";

    /// onoise table of [`MOS_RDRS_NOISE_DECK`] from the official ngspice-46
    /// binary.
    const MOS_RDRS_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/mos_rdrs_noise_ngspice46.dat");

    /// A common-source stage whose card carries RD=2k and RS=1k: the source
    /// degeneration reshapes the operating point and gain while both
    /// resistances add thermal noise at the internal nodes, so the rows pin
    /// the previously unparsed drain/source ohmic resistances end to end.
    const MOS_RDRS_NOISE_DECK: &str = "\
MOS drain source resistance noise testbench

V1 VDD 0 5
VIN G 0 DC 1.5 AC 1
RD VDD D 10k
M1 D G 0 0 NM W=20u L=2u

.OPTIONS NOACCT

.NOISE v(d) VIN DEC 2 1k 100k

.MODEL NM NMOS LEVEL=1 VTO=1.0 KP=60u TOX=20n LD=0.2u RD=2k RS=1k
+ CGSO=2e-10 CGDO=2e-10

.END
";

    /// onoise table of the DTEMP=150 variant of [`MOS_RDRS_NOISE_DECK`]
    /// from the official ngspice-46 binary.
    const MOS_DTEMP_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/mos_dtemp_noise_ngspice46.dat");

    /// Instance DTEMP must heat the channel thermal source and both
    /// externalized drain/source resistances exactly as mos1noi.c does
    /// (shot-free deck: every noise source is temperature-bearing).
    #[test]
    fn mos_dtemp_noise_matches_the_ngspice46_oracle() {
        let deck = MOS_RDRS_NOISE_DECK.replace("M1 D G 0 0 NM W=20u L=2u", "M1 D G 0 0 NM W=20u L=2u DTEMP=150");
        assert_ne!(deck, MOS_RDRS_NOISE_DECK);
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("d").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            2,
            1e3,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = MOS_DTEMP_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// The externalized MOS drain/source resistances must reproduce the
    /// official binary's operating point and noise.
    #[test]
    fn mos_rdrs_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(MOS_RDRS_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("d").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            2,
            1e3,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = MOS_RDRS_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise table of [`DIODE_DTEMP_NOISE_DECK`] from the official
    /// ngspice-46 binary.
    const DIODE_DTEMP_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/diode_dtemp_noise_ngspice46.dat");

    /// A forward-biased diode at DTEMP=150 whose series resistance carries a
    /// comparable share of the output noise: dionoise.c heats the RS thermal
    /// source by the instance offset while shot noise stays
    /// temperature-free, so the row values pin the device-resistor
    /// instance-temperature plumbing.
    const DIODE_DTEMP_NOISE_DECK: &str = "\
Diode instance temperature noise testbench

V1 IN 0 DC 0.75 AC 1
R1 IN A 50
D1 A 0 DM DTEMP=150

.OPTIONS NOACCT

.NOISE v(a) V1 DEC 2 1k 100k

.MODEL DM D IS=1e-14 N=1.5 RS=200 CJO=1p

.END
";

    /// The externalized diode series resistance must heat its thermal noise
    /// by the instance DTEMP exactly as the official binary does.
    #[test]
    fn diode_dtemp_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(DIODE_DTEMP_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("a").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            2,
            1e3,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "V1", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = DIODE_DTEMP_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise table of [`RES_DTEMP_NOISE_DECK`] from the official ngspice-46
    /// binary.
    const RES_DTEMP_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/res_dtemp_noise_ngspice46.dat");

    /// Two equal resistors where one carries DTEMP=150: its thermal noise
    /// runs 150 K hotter (nevalsrc.c THERMNOISE 4k·(CKTtemp+dtemp)·g) while
    /// its resistance is unchanged, so the divider's output noise cleanly
    /// separates the instance-temperature semantics from everything else.
    const RES_DTEMP_NOISE_DECK: &str = "\
Resistor instance temperature noise testbench

V1 IN 0 DC 0 AC 1
R1 IN OUT 10k
R2 OUT 0 10k DTEMP=150

.OPTIONS NOACCT

.NOISE v(out) V1 DEC 2 1k 100k

.END
";

    /// Per-instance DTEMP must heat a resistor's thermal noise exactly as
    /// the official binary does.
    #[test]
    fn resistor_dtemp_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(RES_DTEMP_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("out").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            2,
            1e3,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "V1", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = RES_DTEMP_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 2e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise table of [`GP_RB_NOISE_DECK`] from the official ngspice-46
    /// binary.
    const GP_RB_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/gp_rb_noise_ngspice46.dat");

    /// The quantifying deck for the GP noise gap: RB=500 driven from a
    /// low-impedance source, where the base-resistance thermal noise
    /// referred through the stage gain dominates the output (the binary
    /// reports five times the amplitude shot noise alone produces). Without
    /// RBM the base resistance is constant — ngspice defaults RBM to RB,
    /// zeroing the qb-modulated part — so the externalized base node
    /// carries the whole story.
    const GP_RB_NOISE_DECK: &str = "\
GP base resistance noise check

V1 VCC 0 5
VIN B 0 DC 0.78 AC 1
RC VCC C 1k
Q1 C B 0 QN

.OPTIONS NOACCT

.NOISE v(c) VIN DEC 2 100k 10Meg

.MODEL QN NPN IS=1e-16 BF=100 BR=2 RB=500 RC=10 RE=2
+ CJE=2e-12 CJC=1e-12 TF=3e-10 TR=5e-9

.END
";

    /// The standing acceptance gate for the GP base-resistance increment:
    /// closing the factor-five deficit requires the base node in the matrix,
    /// and a first externalization attempt passed this gate but shifted the
    /// junction-limiting Newton trajectories on reference-sensitive decks
    /// (general suite 8/8 -> 5/8), so it was reverted. The increment lands
    /// when this passes with the full suite green.
    #[test]
    #[ignore = "pending the GP base-node promotion; see the test doc"]
    fn gp_rb_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(GP_RB_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("c").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            2,
            1e5,
            1e7,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = GP_RB_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 1e-2,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise tables of the DTEMP=150 variants of the VBIC rb deck and the
    /// GP rc/re deck from the official ngspice-46 binary.
    const VBIC_DTEMP_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/vbic_dtemp_noise_ngspice46.dat");
    const GP_DTEMP_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/gp_dtemp_noise_ngspice46.dat");

    fn assert_onoise_matches(
        deck: &str,
        oracle_table: &str,
        output_node: &str,
        input_source: &str,
        points_per_decade: usize,
        fstart: f64,
        fstop: f64,
        gate: f64,
        label: &str,
    ) {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name(output_node).expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            points_per_decade,
            fstart,
            fstop,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, input_source, &frequencies, 300.15)
            .expect("noise analysis runs");
        let oracle: Vec<(f64, f64)> = oracle_table
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "{label}: grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= gate,
                "{label}: onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// Instance DTEMP must heat the VBIC internal-resistance thermal model
    /// sources and reshape the operating point through the device
    /// temperature scaling exactly as the official binary does.
    #[test]
    fn vbic_dtemp_noise_matches_the_ngspice46_oracle() {
        let deck = RB_NOISE_DECK.replace("Q1 C B 0 0 N1", "Q1 C B 0 0 N1 DTEMP=150");
        assert_ne!(deck, RB_NOISE_DECK);
        assert_onoise_matches(
            &deck,
            VBIC_DTEMP_NOISE_ORACLE,
            "c",
            "VIN",
            5,
            1e5,
            1e7,
            1e-2,
            "vbic-dtemp",
        );
    }

    /// Instance DTEMP must heat the externalized GP collector/emitter
    /// resistances exactly as bjtnoise.c does. The band stops at 10 MHz:
    /// above it the +150 K junction-capacitance temperature scaling
    /// diverges from the binary by ~1.7 percent at 100 MHz, a GP
    /// cap-temperature fidelity boundary separate from the noise heating
    /// under test here.
    #[test]
    fn gp_dtemp_noise_matches_the_ngspice46_oracle() {
        let deck = GP_RCRE_NOISE_DECK
            .replace("Q1 C B 0 QN", "Q1 C B 0 QN DTEMP=150")
            .replace(".NOISE v(c) VIN DEC 5 10k 100Meg", ".NOISE v(c) VIN DEC 5 10k 10Meg");
        assert_ne!(deck, GP_RCRE_NOISE_DECK);
        assert_onoise_matches(
            &deck,
            GP_DTEMP_NOISE_ORACLE,
            "c",
            "VIN",
            5,
            1e4,
            1e7,
            1e-2,
            "gp-dtemp",
        );
    }

    /// The externalized GP collector/emitter resistances must reproduce the
    /// official binary's noise on a deck they dominate.
    #[test]
    fn gp_rcre_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(GP_RCRE_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("c").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            5,
            1e4,
            1e8,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = GP_RCRE_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 1e-2,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// onoise table of [`JFET_FLICKER_DECK`] from the official ngspice-46
    /// binary.
    const JFET_FLICKER_ORACLE: &str =
        include_str!("../../../tests/testdata/jfet_flicker_ngspice46.dat");

    /// A common-source JFET stage with flicker-dominated low-frequency
    /// noise; AF=1.4 with instance M=2 makes the per-finger multiplicity
    /// recombination observable against the binary.
    const JFET_FLICKER_DECK: &str = "\
JFET flicker noise testbench

V1 VDD 0 12
VIN G 0 DC -0.5 AC 1
RD VDD D 2k
J1 D G 0 JN M=2

.OPTIONS NOACCT

.NOISE v(d) VIN DEC 5 10 100k

.MODEL JN NJF VTO=-2 BETA=1m LAMBDA=0 RD=10 RS=10 CGS=2p CGD=2p
+ KF=1e-16 AF=1.4

.END
";

    /// onoise tables of [`RES_FLICKER_DECK`] and its quiet-R2 variant from
    /// the official ngspice-46 binary.
    const RES_FLICKER_ORACLE: &str =
        include_str!("../../../tests/testdata/res_flicker_ngspice46.dat");
    const RES_QUIET_ORACLE: &str =
        include_str!("../../../tests/testdata/res_quiet_ngspice46.dat");

    /// A current-carrying semiconductor resistor with model-card flicker:
    /// KF at AF=1.5 over the effective noise area
    /// `(L−2·SHORT)^LF·(W−2·NARROW)^WF`, lifting the low-frequency rows
    /// well above the thermal floor.
    const RES_FLICKER_DECK: &str = "\
Resistor flicker noise testbench

V1 IN 0 DC 2 AC 1
R1 IN OUT RMOD L=20u W=2u
R2 OUT 0 1k

.OPTIONS NOACCT

.NOISE v(out) V1 DEC 5 10 100k

.MODEL RMOD R RSH=100 KF=1e-22 AF=1.5 SHORT=0.5u NARROW=0.2u

.END
";

    /// resnoise.c flicker (with the effective-noise-area folding) must
    /// reproduce the official binary.
    #[test]
    fn resistor_flicker_noise_matches_the_ngspice46_oracle() {
        assert_onoise_matches(
            RES_FLICKER_DECK,
            RES_FLICKER_ORACLE,
            "out",
            "V1",
            5,
            10.0,
            1e5,
            5e-3,
            "res-flicker",
        );
    }

    /// The `noisy=0` instance switch must silence a resistor's thermal and
    /// flicker noise exactly as resnoise.c skips quiet instances.
    #[test]
    fn quiet_resistor_noise_matches_the_ngspice46_oracle() {
        let deck = RES_FLICKER_DECK.replace("R2 OUT 0 1k", "R2 OUT 0 1k NOISY=0");
        assert_ne!(deck, RES_FLICKER_DECK);
        assert_onoise_matches(
            &deck,
            RES_QUIET_ORACLE,
            "out",
            "V1",
            5,
            10.0,
            1e5,
            5e-3,
            "res-quiet",
        );
    }

    /// onoise table of the DTEMP=150 variant of [`JFET_FLICKER_DECK`] from
    /// the official ngspice-46 binary.
    const JFET_DTEMP_NOISE_ORACLE: &str =
        include_str!("../../../tests/testdata/jfet_dtemp_noise_ngspice46.dat");

    /// Instance DTEMP must heat the JFET channel thermal source and the
    /// externalized drain/source resistances exactly as jfetnoi.c does
    /// (the flicker component is temperature-free, isolating the heating
    /// to the white floor).
    #[test]
    fn jfet_dtemp_noise_matches_the_ngspice46_oracle() {
        let deck = JFET_FLICKER_DECK.replace("J1 D G 0 JN M=2", "J1 D G 0 JN M=2 DTEMP=150");
        assert_ne!(deck, JFET_FLICKER_DECK);
        assert_onoise_matches(
            &deck,
            JFET_DTEMP_NOISE_ORACLE,
            "d",
            "VIN",
            5,
            10.0,
            1e5,
            1e-2,
            "jfet-dtemp",
        );
    }

    /// The jfetnoi.c flicker law (per-finger current with an explicit
    /// multiplicity factor) must reproduce the official binary under this
    /// model's folded-beta representation.
    #[test]
    fn jfet_flicker_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(JFET_FLICKER_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("d").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            5,
            10.0,
            1e5,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = JFET_FLICKER_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "grids must match");
        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

    /// The default (NLEV=2) gm²-based flicker law and the NLEV=0 legacy law
    /// must both reproduce the official binary.
    #[test]
    fn mos_flicker_noise_matches_the_ngspice46_oracle() {
        assert_noise_matches_oracle(MOS_FLICKER_DECK, MOS_FLICKER_NLEV2_ORACLE, "nlev2");
        let nlev0_deck = MOS_FLICKER_DECK.replace("AF=1.2", "AF=1.2 NLEV=0");
        assert_ne!(nlev0_deck, MOS_FLICKER_DECK);
        assert_noise_matches_oracle(&nlev0_deck, MOS_FLICKER_NLEV0_ORACLE, "nlev0");
    }

    /// The VBIC parasitic-resistance thermal sources and internal-node shot
    /// sources must reproduce the official binary on a deck designed to
    /// expose them.
    #[test]
    fn vbic_parasitic_resistance_noise_matches_the_ngspice46_oracle() {
        let netlist = Netlist::parse(RB_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("c").expect("output node");
        let frequencies = crate::analysis::ac::ac_sweep_frequencies(
            crate::netlist::FreqVariation::Dec,
            5,
            1e5,
            1e7,
        );
        let results = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &frequencies, 300.15)
            .expect("noise analysis runs");

        let oracle: Vec<(f64, f64)> = RB_NOISE_ORACLE
            .lines()
            .filter(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .map(|line| {
                let mut fields = line.split_whitespace();
                (
                    fields.next().unwrap().parse().unwrap(),
                    fields.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        assert_eq!(results.len(), oracle.len(), "frequency grids must match");

        for (result, (freq_ref, onoise_ref)) in results.iter().zip(&oracle) {
            assert!(
                (result.frequency - freq_ref).abs() <= 1e-6 * freq_ref,
                "sweep grid diverged from the oracle at {:e}",
                freq_ref,
            );
            let onoise = result.output_noise_rms();
            let relative = (onoise - onoise_ref).abs() / onoise_ref;
            assert!(
                relative <= 5e-3,
                "onoise at {:e} Hz: ours {:.6e} vs ngspice-46 {:.6e} (rel {:.3e})",
                freq_ref,
                onoise,
                onoise_ref,
                relative,
            );
        }
    }

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
