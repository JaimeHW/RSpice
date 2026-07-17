use super::*;

#[derive(Clone, Copy)]
pub(in crate::engine::advanced) enum NoiseOutputPort<'a> {
    NodeIds {
        positive: usize,
        negative: Option<usize>,
    },
    NodeNames {
        positive: &'a str,
        negative: Option<&'a str>,
    },
}

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
    pub(in crate::engine::advanced) fn differential_noise_output_complex(
        solution: &[Complex64],
        output_pos: usize,
        output_neg: Option<usize>,
        num_nodes: usize,
    ) -> Complex64 {
        let v_pos = if output_pos > 0 && output_pos <= num_nodes {
            solution[output_pos - 1]
        } else {
            Complex64::new(0.0, 0.0)
        };
        let v_neg = match output_neg {
            Some(node) if node > 0 && node <= num_nodes => solution[node - 1],
            _ => Complex64::new(0.0, 0.0),
        };
        v_pos - v_neg
    }

    #[inline]
    pub(in crate::engine::advanced) fn differential_noise_output(
        solution: &[Complex64],
        output_pos: usize,
        output_neg: Option<usize>,
        num_nodes: usize,
    ) -> Value {
        Self::differential_noise_output_complex(solution, output_pos, output_neg, num_nodes).norm()
    }

    #[inline]
    fn stamp_unit_noise_current_rhs(rhs: &mut [Complex64], node_pos: usize, node_neg: usize) {
        // Noise current sources use one-based unified MNA unknown IDs. Ordinary
        // current noise references node-voltage IDs; generated potential noise
        // references its concrete branch-equation ID in the same namespace.
        if node_pos > 0 && node_pos <= rhs.len() {
            rhs[node_pos - 1] += Complex64::new(1.0, 0.0);
        }
        if node_neg > 0 && node_neg <= rhs.len() {
            rhs[node_neg - 1] -= Complex64::new(1.0, 0.0);
        }
    }

    #[cfg(feature = "veriloga-builtins")]
    fn generated_noise_source(
        circuit: &CircuitData,
        instance_name: &str,
        source: crate::device::veriloga_generated::BuiltinEvaluatedNoiseSource,
    ) -> Result<NoiseSource, SimulationError> {
        use crate::device::veriloga_generated::{GeneratedNoiseInjection, GeneratedNoiseKind};

        let descriptor = source.mapped.descriptor;
        let evaluation = source.evaluation;
        let invalid = |detail: String| {
            SimulationError::Circuit(format!(
                "Generated Verilog-A device '{instance_name}' noise mechanism '{}' is invalid: {detail}",
                descriptor.mechanism
            ))
        };
        let (node_pos, node_neg) = match source.mapped.injection {
            GeneratedNoiseInjection::Current { node_pos, node_neg } => {
                if node_pos > circuit.num_nodes() || node_neg > circuit.num_nodes() {
                    return Err(invalid(format!(
                        "mapped current endpoints ({node_pos}, {node_neg}) exceed the {} circuit nodes",
                        circuit.num_nodes()
                    )));
                }
                (node_pos, node_neg)
            }
            GeneratedNoiseInjection::Potential { branch } => {
                if branch == 0 || branch > circuit.num_branches() {
                    return Err(invalid(format!(
                        "mapped potential branch {branch} is outside the {} circuit branches",
                        circuit.num_branches()
                    )));
                }
                (circuit.get_branch_matrix_index(branch), 0)
            }
        };
        let display_name = format!(
            "{instance_name}:{}",
            descriptor.label.unwrap_or(descriptor.mechanism)
        );

        let noise = match descriptor.kind {
            GeneratedNoiseKind::White => {
                if descriptor.table_len != 0
                    || evaluation.exponent.is_some()
                    || !evaluation.table_operands.is_empty()
                {
                    return Err(invalid(
                        "white noise carried flicker or table metadata".to_string(),
                    ));
                }
                NoiseSource::white(display_name, node_pos, node_neg, evaluation.psd)
            }
            GeneratedNoiseKind::Flicker => {
                if descriptor.table_len != 0 || !evaluation.table_operands.is_empty() {
                    return Err(invalid("flicker noise carried table metadata".to_string()));
                }
                let exponent = if evaluation.active {
                    evaluation.exponent.ok_or_else(|| {
                        invalid("active flicker noise has no frequency exponent".to_string())
                    })?
                } else {
                    evaluation.exponent.unwrap_or(1.0)
                };
                NoiseSource::flicker_psd(display_name, node_pos, node_neg, evaluation.psd, exponent)
            }
            GeneratedNoiseKind::Table => {
                if evaluation.exponent.is_some() {
                    return Err(invalid(
                        "table noise carried a flicker exponent".to_string(),
                    ));
                }
                let points = if evaluation.active {
                    if descriptor.table_len == 0
                        || !descriptor.table_len.is_multiple_of(2)
                        || evaluation.table_operands.len() != descriptor.table_len
                    {
                        return Err(invalid(format!(
                            "active table metadata declares {} operands and evaluated {}",
                            descriptor.table_len,
                            evaluation.table_operands.len()
                        )));
                    }
                    let mut points = Vec::with_capacity(descriptor.table_len / 2);
                    for pair in evaluation.table_operands.chunks_exact(2) {
                        let frequency = pair[0];
                        let power = pair[1];
                        let valid = if descriptor.table_log_interp {
                            frequency > 0.0 && power > 0.0
                        } else {
                            frequency >= 0.0 && power >= 0.0
                        };
                        if !valid {
                            return Err(invalid(format!(
                                "table point ({frequency}, {power}) violates {} interpolation requirements",
                                if descriptor.table_log_interp {
                                    "positive log-log"
                                } else {
                                    "nonnegative linear"
                                }
                            )));
                        }
                        points.push((frequency, power));
                    }
                    points.sort_by(|left, right| left.0.total_cmp(&right.0));
                    points
                } else {
                    Vec::new()
                };
                NoiseSource::tabulated(
                    display_name,
                    node_pos,
                    node_neg,
                    evaluation.psd,
                    points,
                    descriptor.table_log_interp,
                )
            }
        };
        Ok(
            noise.with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                instance_name,
                descriptor.mechanism,
            )),
        )
    }

    #[inline]
    fn add_port_noise_outer_product(
        covariance: &mut [Vec<Complex64>],
        compensation: &mut [Vec<Complex64>],
        amplitude: &[Complex64],
    ) -> Result<(), SimulationError> {
        for row in 0..amplitude.len() {
            for column in 0..amplitude.len() {
                let contribution = amplitude[row] * amplitude[column].conj();
                if !contribution.re.is_finite() || !contribution.im.is_finite() {
                    return Err(SimulationError::Circuit(
                        "Port-noise transfer produced a non-finite covariance contribution"
                            .to_string(),
                    ));
                }

                // Compensated complex summation preserves small mechanisms in
                // the presence of much larger contributors and reduces loss of
                // precision in off-diagonal cancellation.
                let corrected = contribution - compensation[row][column];
                let updated = covariance[row][column] + corrected;
                compensation[row][column] = (updated - covariance[row][column]) - corrected;
                covariance[row][column] = updated;
            }
        }
        Ok(())
    }

    fn collect_bsim3v3_noise_sources(
        device: &crate::device::mosfet::bsim3v3::Bsim3v3Device,
    ) -> Vec<NoiseSource> {
        let mut sources = Vec::new();
        let (op, bias) = device.noise_operating_point();
        let core = &device.core;
        let model = &core.model;
        let size = &core.size;
        let mult = device.multiplier.max(0.0);
        if mult <= 0.0 {
            return sources;
        }
        let charged_op = if matches!(model.noi_mod, 2 | 4) {
            Some(device.noise_operating_point_with_charge().0)
        } else {
            None
        };
        let op = charged_op.as_ref().unwrap_or(op);

        let gm_sum = op.gm + op.gds + op.gmbs;
        let channel_thermal_conductance = match model.noi_mod {
            1 | 3 => Some((2.0 / 3.0) * gm_sum.abs() * mult),
            2 | 4 => {
                let qinv = op.qinv.abs();
                let denom = size.leff * size.leff + op.ueff * qinv * op.rds;
                if op.ueff > 0.0 && qinv > 0.0 && denom > 0.0 {
                    Some(mult * op.ueff * qinv / denom)
                } else {
                    None
                }
            }
            5 | 6 => {
                if op.vdsat != 0.0 {
                    let vds = bias.vds.min(op.vdsat);
                    Some(((3.0 - vds / op.vdsat) / 3.0) * gm_sum.abs() * mult)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(conductance) = channel_thermal_conductance
            && conductance.is_finite()
            && conductance > 1e-30
        {
            sources.push(
                NoiseSource::thermal(
                    format!("{}:id", device.name),
                    device.node_drain,
                    device.node_source,
                    1.0 / conductance,
                )
                .with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                    &device.name,
                    "ID",
                )),
            );
        }

        match model.noi_mod {
            1 | 4 | 5 => {
                let denom = size.leff * size.leff * model.cox;
                if model.kf > 0.0 && denom > 0.0 && op.cd.abs() > 1e-18 {
                    sources.push(
                        NoiseSource::flicker_with_frequency_exponent(
                            format!("{}:flicker", device.name),
                            device.node_drain,
                            device.node_source,
                            mult * model.kf / denom,
                            model.af,
                            model.ef,
                            op.cd,
                        )
                        .with_identity(
                            crate::analysis::NoiseSourceIdentity::mechanism(&device.name, "FN"),
                        ),
                    );
                }
            }
            2 | 3 | 6 => {
                let leff_noise = size.leff - 2.0 * model.lintnoi;
                if leff_noise > 0.0 && op.cd.abs() > 1e-18 {
                    sources.push(
                        NoiseSource::bsim3_flicker(
                            format!("{}:flicker", device.name),
                            device.node_drain,
                            device.node_source,
                            Bsim3FlickerNoise {
                                multiplier: mult,
                                cd: op.cd,
                                vds: bias.vds,
                                vdseff: op.vdseff,
                                vsattemp: size.vsattemp,
                                ueff: op.ueff,
                                abulk: op.abulk,
                                ab_ov_vgst2vtm: op.ab_ov_vgst2vtm,
                                vgsteff: op.vgsteff,
                                leff: size.leff,
                                leff_noise,
                                litl: size.litl,
                                weff: size.weff,
                                cox: model.cox,
                                oxide_trap_density_a: model.oxide_trap_density_a,
                                oxide_trap_density_b: model.oxide_trap_density_b,
                                oxide_trap_density_c: model.oxide_trap_density_c,
                                em: model.em,
                                ef: model.ef,
                            },
                        )
                        .with_identity(
                            crate::analysis::NoiseSourceIdentity::mechanism(&device.name, "FN"),
                        ),
                    );
                }
            }
            _ => {}
        }

        sources
    }

    fn collect_bsim4v8_noise_sources(
        device: &crate::device::mosfet::bsim4v8::Bsim4v8Device,
    ) -> (Vec<NoiseSource>, Vec<CorrelatedNoisePair>) {
        let mut sources = Vec::new();
        let mut correlated_sources = Vec::new();
        let (op, bias) = device.noise_operating_point();
        let core = &device.core;
        let model = &core.model;
        let size = &core.size;
        let inst = &core.inst;
        let mult = device.multiplier.max(0.0);
        if mult <= 0.0 {
            return (sources, correlated_sources);
        }

        if model.rbody_mod != 0 {
            let mut push_rbody =
                |suffix: &str, node_pos: usize, node_neg: usize, conductance: Value| {
                    let effective_g = conductance * mult;
                    if effective_g.is_finite() && effective_g > 1.0e-30 {
                        sources.push(NoiseSource::thermal(
                            format!("{}.{}", device.name, suffix),
                            node_pos,
                            node_neg,
                            1.0 / effective_g,
                        ));
                    }
                };

            if inst.body_resistance_mode == 3 || inst.body_resistance_mode == 5 {
                push_rbody(
                    "rbps",
                    device.node_bulk,
                    device.node_source_body,
                    inst.body_prime_source_conductance,
                );
                push_rbody(
                    "rbpd",
                    device.node_bulk,
                    device.node_drain_body,
                    inst.body_prime_drain_conductance,
                );
            }
            push_rbody(
                "rbpb",
                device.node_bulk,
                device.node_bulk_external,
                inst.body_prime_bulk_conductance,
            );
            if inst.body_resistance_mode == 5 {
                push_rbody(
                    "rbsb",
                    device.node_bulk_external,
                    device.node_source_body,
                    inst.body_source_bulk_conductance,
                );
                push_rbody(
                    "rbdb",
                    device.node_bulk_external,
                    device.node_drain_body,
                    inst.body_drain_bulk_conductance,
                );
            }
        }

        if model.rgate_mod == 2 && op.gcrg.is_finite() && op.gcrg > 1.0e-30 {
            // b4noi.c: for RGATEMOD=2 the electrode gate resistance noise is
            // attenuated by the bias-dependent channel gate-resistance branch.
            let t0 = 1.0 + inst.gate_conductance / op.gcrg;
            let effective_g = inst.gate_conductance * mult / (t0 * t0);
            if effective_g.is_finite() && effective_g > 1.0e-30 {
                sources.push(NoiseSource::thermal(
                    format!("{}.rg", device.name),
                    device.node_gate,
                    device.node_gate_external,
                    1.0 / effective_g,
                ));
            }
        }

        let channel_thermal_conductance = match model.tnoi_mod {
            0 => {
                let rds_noise = if op.grdsw > 0.0 { 1.0 / op.grdsw } else { 0.0 };
                let t0 = op.ueff * op.qinv.abs();
                let denom = t0 * rds_noise + size.leff * size.leff;
                if t0 > 0.0 && denom > 0.0 {
                    Some((t0 / denom) * model.ntnoi * mult)
                } else {
                    None
                }
            }
            1 => {
                if op.idovvds > 0.0 && op.esat_l != 0.0 {
                    let vgsteff_over_esat_l = op.vgsteff / op.esat_l;
                    let shape = vgsteff_over_esat_l * vgsteff_over_esat_l;
                    let npart_beta = model.rnoia * (1.0 + shape * model.tnoia * size.leff);
                    let mut npart_theta = model.rnoib * (1.0 + shape * model.tnoib * size.leff);
                    if npart_theta > 0.9 {
                        npart_theta = 0.9;
                    }
                    if npart_theta > 0.9 * npart_beta {
                        npart_theta = 0.9 * npart_beta;
                    }

                    let gm_sum = op.gm + op.gmbs + op.gds;
                    let igsquare = npart_theta * npart_theta * gm_sum * gm_sum / op.idovvds;
                    let weighted = npart_beta * (op.gm + op.gmbs) + op.gds;
                    let conductance = (weighted * weighted / op.idovvds) - igsquare;
                    Some(conductance * mult)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(conductance) = channel_thermal_conductance
            && conductance.is_finite()
            && conductance > 1e-30
        {
            sources.push(NoiseSource::thermal(
                format!("{}:id", device.name),
                device.node_drain,
                device.node_source,
                1.0 / conductance,
            ));
        }

        if model.tnoi_mod == 2
            && op.noi_gd0 > 0.0
            && op.vgsteff > 0.0
            && op.esat_l != 0.0
            && size.leff > 0.0
        {
            let eta = 1.0 - op.vdseff * op.ab_ov_vgst2vtm;
            let t0 = 1.0 - eta;
            let t1 = 1.0 + eta;
            let t2 = t1 + 2.0 * op.abulk * core.model_temp.vtm / op.vgsteff;
            let lvsat = size.leff * (1.0 + op.vdseff / op.esat_l);
            if t2 != 0.0 && lvsat != 0.0 {
                let t6 = size.leff / lvsat;
                if t6 != 0.0 {
                    let mut gamma = t6 * (0.5 * t1 + t0 * t0 / (6.0 * t2));
                    let t3 = t2 * t2;
                    let t4 = t0 * t0;
                    let t5 = t3 * t3;
                    if t3 != 0.0 && t5 != 0.0 {
                        let mut delta = (t1 / t3 - (5.0 * t1 + t2) * t4 / (15.0 * t5)
                            + t4 * t4 / (9.0 * t5 * t2))
                            / (6.0 * t6 * t6 * t6);
                        let t7 = t0 / t2;
                        let epsilon = (t7 - t7 * t7 * t7 / 3.0) / (6.0 * t6);
                        let t8 = {
                            let ratio = op.vgsteff / op.esat_l;
                            ratio * ratio
                        };

                        let npart_c = model.rnoic * (1.0 + t8 * model.tnoic * size.leff);
                        let mut ctnoi = if gamma * delta > 0.0 {
                            epsilon / (gamma * delta).sqrt() * (2.5316 * npart_c)
                        } else {
                            1.0
                        };
                        ctnoi = ctnoi.clamp(0.0, 1.0);

                        let npart_beta = model.rnoia * (1.0 + t8 * model.tnoia * size.leff);
                        let npart_theta = model.rnoib * (1.0 + t8 * model.tnoib * size.leff);
                        gamma *= 3.0 * npart_beta * npart_beta;
                        delta *= 3.75 * npart_theta * npart_theta;

                        let gamma_gd0 = gamma * op.noi_gd0;
                        let c0 = op.coxeff * size.weff_cv * inst.nf * size.leff_cv;
                        let sigrat = if gamma > 0.0 && delta > 0.0 && op.noi_gd0 > 0.0 {
                            c0 / op.noi_gd0 * (delta / gamma).sqrt()
                        } else {
                            0.0
                        };

                        if gamma_gd0.is_finite() && gamma_gd0 > 0.0 {
                            let ctnoi_sq = ctnoi * ctnoi;
                            let uncorrelated_g = gamma_gd0 * (1.0 - ctnoi_sq) * mult;
                            if uncorrelated_g.is_finite() && uncorrelated_g > 1.0e-30 {
                                sources.push(NoiseSource::thermal(
                                    format!("{}:id", device.name),
                                    device.node_drain,
                                    device.node_source,
                                    1.0 / uncorrelated_g,
                                ));
                            }

                            let (first, second) = if op.mode >= 0 {
                                (
                                    NoisePort {
                                        node_pos: device.node_drain,
                                        node_neg: device.node_source,
                                    },
                                    NoisePort {
                                        node_pos: device.node_gate,
                                        node_neg: device.node_source,
                                    },
                                )
                            } else {
                                (
                                    NoisePort {
                                        node_pos: device.node_source,
                                        node_neg: device.node_drain,
                                    },
                                    NoisePort {
                                        node_pos: device.node_gate,
                                        node_neg: device.node_drain,
                                    },
                                )
                            };
                            correlated_sources.push(CorrelatedNoisePair::bsim4_tnoi2(
                                format!("{}:corl", device.name),
                                first,
                                second,
                                gamma_gd0,
                                ctnoi,
                                sigrat,
                                mult,
                            ));
                        }
                    }
                }
            }
        }

        match model.fnoi_mod {
            0 => {
                let coxe = model.coxe();
                let denom = size.leff * size.leff * coxe;
                if model.kf > 0.0 && denom > 0.0 && op.cd.abs() > 1e-18 {
                    sources.push(NoiseSource::flicker_with_frequency_exponent(
                        format!("{}:flicker", device.name),
                        device.node_drain,
                        device.node_source,
                        mult * model.kf / denom,
                        model.af,
                        model.ef,
                        op.cd,
                    ));
                }
            }
            1 => {
                let leff_noise = size.leff - 2.0 * model.lintnoi;
                if leff_noise > 0.0 && op.cd.abs() > 1e-18 {
                    sources.push(NoiseSource::bsim4_flicker(
                        format!("{}:flicker", device.name),
                        device.node_drain,
                        device.node_source,
                        Bsim4FlickerNoise {
                            multiplier: mult,
                            cd: op.cd,
                            vds: bias.vds,
                            vdseff: op.vdseff,
                            vsattemp: inst.vsattemp,
                            ueff: op.ueff,
                            abulk: op.abulk,
                            ab_ov_vgst2vtm: op.ab_ov_vgst2vtm,
                            vgsteff: op.vgsteff,
                            nstar: op.nstar,
                            leff: size.leff,
                            leff_noise,
                            litl: size.litl,
                            weff: size.weff,
                            nf: inst.nf,
                            coxe: model.coxe(),
                            oxide_trap_density_a: model.oxide_trap_density_a,
                            oxide_trap_density_b: model.oxide_trap_density_b,
                            oxide_trap_density_c: model.oxide_trap_density_c,
                            em: model.em,
                            ef: model.ef,
                        },
                    ));
                }
            }
            _ => {}
        }

        let (igs_current, igd_current) = if op.mode >= 0 {
            (op.igs + op.igcs, op.igd + op.igcd)
        } else {
            (op.igs + op.igcd, op.igd + op.igcs)
        };
        if igs_current.abs() > 1e-18 {
            sources.push(NoiseSource::shot(
                format!("{}:igs", device.name),
                device.node_gate,
                device.node_source,
                mult * igs_current,
            ));
        }
        if igd_current.abs() > 1e-18 {
            sources.push(NoiseSource::shot(
                format!("{}:igd", device.name),
                device.node_gate,
                device.node_drain,
                mult * igd_current,
            ));
        }
        if op.igb.abs() > 1e-18 {
            sources.push(NoiseSource::shot(
                format!("{}:igb", device.name),
                device.node_gate,
                device.node_bulk,
                mult * op.igb,
            ));
        }

        (sources, correlated_sources)
    }

    #[cfg(test)]
    pub(in crate::engine::advanced) fn collect_noise_sources(
        circuit: &CircuitData,
        dc_solution: &[Value],
    ) -> (Vec<NoiseSource>, Vec<CorrelatedNoisePair>) {
        Self::try_collect_noise_sources(circuit, dc_solution).unwrap_or_else(|err| panic!("{err}"))
    }

    pub(in crate::engine) fn try_collect_noise_sources(
        circuit: &CircuitData,
        dc_solution: &[Value],
    ) -> Result<(Vec<NoiseSource>, Vec<CorrelatedNoisePair>), SimulationError> {
        let mut noise_sources = Vec::new();
        let mut correlated_noise_sources = Vec::new();
        let mut bsim4_series_noise_conductances: HashMap<String, Value> = HashMap::new();

        for bsim3 in &circuit.bsim3v3.devices {
            noise_sources.extend(Self::collect_bsim3v3_noise_sources(bsim3));
        }

        for bsim4 in &circuit.bsim4v8.devices {
            let (bsim4_sources, bsim4_correlated) = Self::collect_bsim4v8_noise_sources(bsim4);
            noise_sources.extend(bsim4_sources);
            correlated_noise_sources.extend(bsim4_correlated);

            if bsim4.core.model.rds_mod == 1 {
                let (op, bias) = bsim4.noise_operating_point();
                if let Some((mut drain_g, mut source_g)) =
                    bsim4.external_rds_conductances(dc_solution)
                {
                    if bsim4.core.model.tnoi_mod == 1 && op.idovvds > 0.0 && op.esat_l != 0.0 {
                        let model = &bsim4.core.model;
                        let size = &bsim4.core.size;
                        let shape = (op.vgsteff / op.esat_l).powi(2);
                        let npart_beta = model.rnoia * (1.0 + shape * model.tnoia * size.leff);
                        let mut npart_theta = model.rnoib * (1.0 + shape * model.tnoib * size.leff);
                        if npart_theta > 0.9 {
                            npart_theta = 0.9;
                        }
                        if npart_theta > 0.9 * npart_beta {
                            npart_theta = 0.9 * npart_beta;
                        }

                        let adjusted = |g: Value| {
                            if g > 0.0 && g.is_finite() {
                                g * (1.0 + npart_theta * npart_theta * g / op.idovvds)
                            } else {
                                g
                            }
                        };
                        if bias.vds >= 0.0 {
                            source_g = adjusted(source_g);
                        } else {
                            drain_g = adjusted(drain_g);
                        }
                    }

                    let mult = bsim4.multiplier.max(0.0);
                    if drain_g > 0.0 && drain_g.is_finite() && mult > 0.0 {
                        noise_sources.push(NoiseSource::thermal(
                            format!("{}.__rd", bsim4.name),
                            bsim4.node_drain,
                            bsim4.node_drain_external,
                            1.0 / (drain_g * mult),
                        ));
                    }
                    if source_g > 0.0 && source_g.is_finite() && mult > 0.0 {
                        noise_sources.push(NoiseSource::thermal(
                            format!("{}.__rs", bsim4.name),
                            bsim4.node_source,
                            bsim4.node_source_external,
                            1.0 / (source_g * mult),
                        ));
                    }
                }
            } else if bsim4.core.model.tnoi_mod == 1 {
                let (op, bias) = bsim4.noise_operating_point();
                if op.idovvds > 0.0 && op.esat_l != 0.0 {
                    let model = &bsim4.core.model;
                    let size = &bsim4.core.size;
                    let inst = &bsim4.core.inst;
                    let shape = (op.vgsteff / op.esat_l).powi(2);
                    let npart_beta = model.rnoia * (1.0 + shape * model.tnoia * size.leff);
                    let mut npart_theta = model.rnoib * (1.0 + shape * model.tnoib * size.leff);
                    if npart_theta > 0.9 {
                        npart_theta = 0.9;
                    }
                    if npart_theta > 0.9 * npart_beta {
                        npart_theta = 0.9 * npart_beta;
                    }

                    let adjusted = |g: Value| {
                        if g > 0.0 && g.is_finite() {
                            g * (1.0 + npart_theta * npart_theta * g / op.idovvds)
                        } else {
                            g
                        }
                    };
                    let drain_g = if bias.vds < 0.0 {
                        adjusted(inst.drain_conductance)
                    } else {
                        inst.drain_conductance
                    };
                    let source_g = if bias.vds >= 0.0 {
                        adjusted(inst.source_conductance)
                    } else {
                        inst.source_conductance
                    };
                    let mult = bsim4.multiplier.max(0.0);
                    if drain_g > 0.0 && mult > 0.0 {
                        bsim4_series_noise_conductances
                            .insert(format!("{}.__rd", bsim4.name), drain_g * mult);
                    }
                    if source_g > 0.0 && mult > 0.0 {
                        bsim4_series_noise_conductances
                            .insert(format!("{}.__rs", bsim4.name), source_g * mult);
                    }
                }
            }
        }

        for device in &circuit.ekv26s.devices {
            if let Some((thermal_psd, flicker)) = device.noise_psds_at_solution(dc_solution) {
                if thermal_psd.is_finite() && thermal_psd > 0.0 {
                    noise_sources.push(NoiseSource::white(
                        format!("{}:thermal", device.name),
                        device.node_drain,
                        device.node_source,
                        thermal_psd,
                    ));
                }
                if let Some((flicker_psd, frequency_exponent)) = flicker
                    && flicker_psd.is_finite()
                    && flicker_psd > 0.0
                {
                    noise_sources.push(NoiseSource::flicker_psd(
                        format!("{}:flicker", device.name),
                        device.node_drain,
                        device.node_source,
                        flicker_psd,
                        frequency_exponent,
                    ));
                }
            }
        }

        for device in &circuit.ekv3s.devices {
            noise_sources.push(NoiseSource::tabulated(
                format!("{}:ekv3-vanoise", device.name),
                device.node_drain,
                device.node_source,
                1.0,
                device.noise_current_psd_points(),
                true,
            ));
        }

        // Verilog-A white_noise()/flicker_noise() sources, with PSDs
        // evaluated at the operating point. Potential-contribution noise
        // arrives as a series EMF on the branch-equation row, which is an
        // ordinary system unknown here, so both kinds inject the same way.
        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            let mut probe = device.clone();
            let instance = probe.name.clone();
            probe.set_analysis_type(3);
            let sources = probe.try_noise_sources(dc_solution).map_err(|err| {
                SimulationError::Circuit(format!(
                    "Verilog-A device '{instance}' noise evaluation failed: {err}"
                ))
            })?;
            for source in sources {
                let name = format!("{instance}:{}", source.name);
                noise_sources.push(match (source.table, source.exponent) {
                    (Some((points, log_interp)), _) => NoiseSource::tabulated(
                        name,
                        source.node_pos,
                        source.node_neg,
                        source.psd,
                        points,
                        log_interp,
                    ),
                    (None, None) => {
                        NoiseSource::white(name, source.node_pos, source.node_neg, source.psd)
                    }
                    (None, Some(ef)) => NoiseSource::flicker_psd(
                        name,
                        source.node_pos,
                        source.node_neg,
                        source.psd,
                        ef,
                    ),
                });
            }
        }

        #[cfg(feature = "veriloga-builtins")]
        for device in circuit.generated_veriloga_devices().iter() {
            let evaluated = device
                .evaluate_noise_sources(
                    dc_solution,
                    circuit.num_nodes(),
                    circuit.generated_simulation_parameters,
                )
                .map_err(|err| {
                    SimulationError::Circuit(format!(
                        "Generated Verilog-A device '{}' noise evaluation failed: {err}",
                        device.instance_name
                    ))
                })?;
            for source in evaluated {
                noise_sources.push(Self::generated_noise_source(
                    circuit,
                    &device.instance_name,
                    source,
                )?);
            }
        }

        // Builder-owned device series resistors are physical resistor
        // stamps, but Xyce exposes their noise under the parent MOS device as
        // their parent device mechanisms.  Build the ownership table from device
        // topology rather than inferring arbitrary user resistor names.
        let mut device_series_noise_owners = circuit
            .mosfets
            .devices
            .iter()
            .filter(|mos| matches!(mos.level, 1..=3))
            .flat_map(|mos| {
                [
                    (
                        format!("{}.__rd", mos.name).to_ascii_lowercase(),
                        crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, "RD"),
                    ),
                    (
                        format!("{}.__rs", mos.name).to_ascii_lowercase(),
                        crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, "RS"),
                    ),
                ]
            })
            .collect::<HashMap<_, _>>();
        for mos in &circuit.bsim3v3.devices {
            device_series_noise_owners.extend([
                (
                    format!("{}.__rd", mos.name).to_ascii_lowercase(),
                    crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, "RD"),
                ),
                (
                    format!("{}.__rs", mos.name).to_ascii_lowercase(),
                    crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, "RS"),
                ),
            ]);
        }
        for bjt in circuit
            .bjts
            .devices
            .iter()
            .filter(|bjt| bjt.uses_legacy_gummel_poon())
        {
            device_series_noise_owners.extend([
                (
                    format!("{}.__rc", bjt.name).to_ascii_lowercase(),
                    crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "RC"),
                ),
                (
                    format!("{}.__rb", bjt.name).to_ascii_lowercase(),
                    crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "RB"),
                ),
                (
                    format!("{}.__re", bjt.name).to_ascii_lowercase(),
                    crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "RE"),
                ),
            ]);
        }

        // Resistor thermal noise (4kT/R) and model-card flicker noise
        // (resnoise.c), both gated by the per-instance `noisy` switch.
        for (i, stamp) in circuit.resistors.stamps.iter().enumerate() {
            if !circuit.resistors.noisy.get(i).copied().unwrap_or(true) {
                continue;
            }
            let name = circuit
                .resistors
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("R{}", i + 1));
            let conductance = bsim4_series_noise_conductances
                .get(&name)
                .copied()
                .unwrap_or_else(|| circuit.resistors.small_signal_conductance(i));
            let resistance = if conductance.abs() > 0.0 {
                1.0 / conductance
            } else {
                f64::INFINITY
            };
            if resistance <= 0.0 || !resistance.is_finite() || resistance >= 1e12 {
                continue;
            }

            let mut source =
                NoiseSource::thermal(name.clone(), stamp.pp.row, stamp.nn.row, resistance);
            if let Some(identity) = device_series_noise_owners.get(&name.to_ascii_lowercase()) {
                source.identity = identity.clone();
            }
            source.temperature_offset = circuit.resistors.noise_temperature_offset(i);
            noise_sources.push(source);

            if let Some(&Some((coefficient, af, ef))) = circuit.resistors.flicker.get(i) {
                let v_pos = Self::noise_node_voltage(dc_solution, stamp.pp.row);
                let v_neg = Self::noise_node_voltage(dc_solution, stamp.nn.row);
                let current = circuit
                    .resistors
                    .conductances
                    .get(i)
                    .copied()
                    .unwrap_or(0.0)
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
                noise_sources.push(
                    NoiseSource::shot(
                        format!("{}:IC", bjt.name),
                        bjt.node_collector,
                        bjt.node_emitter,
                        ic,
                    )
                    .with_identity(
                        crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "IC"),
                    ),
                );
            }
            if ibe > 1e-18 {
                noise_sources.push(
                    NoiseSource::shot(
                        format!("{}:IBE", bjt.name),
                        bjt.node_base,
                        bjt.node_emitter,
                        ibe,
                    )
                    .with_identity(
                        crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "IB"),
                    ),
                );
            }
            if ibc > 1e-18 {
                noise_sources.push(
                    NoiseSource::shot(
                        format!("{}:IBC", bjt.name),
                        bjt.node_base,
                        bjt.node_collector,
                        ibc,
                    )
                    .with_identity(
                        crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "IB"),
                    ),
                );
            }

            if let Some((kf, af, ef)) = bjt.flicker_noise_coefficients() {
                let (_, ib, _) = bjt.operating_point_currents();
                if ib.abs() > 1e-18 {
                    noise_sources.push(
                        NoiseSource::flicker_with_frequency_exponent(
                            format!("{}:flicker", bjt.name),
                            bjt.node_base,
                            bjt.node_emitter,
                            kf,
                            af,
                            ef,
                            ib,
                        )
                        .with_identity(
                            crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, "FN"),
                        ),
                    );
                }
            }
        }

        // MOS channel thermal noise and 1/f noise.
        for mos in &circuit.mosfets.devices {
            let is_classic_noise_model = matches!(mos.level, 1..=3);
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
                if is_classic_noise_model {
                    source.identity =
                        crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, "ID");
                }
                noise_sources.push(source);
            }

            // SPICE NLEV flicker laws (mos1noi.c; NLEV defaults to 2, whose
            // gm²-based density is bias-dependent through the coefficient
            // rather than the current term).
            if let Some((coefficient, current, af, ef)) = mos.flicker_noise_source_terms()
                && coefficient > 0.0
                && current.abs() > 1e-18
            {
                let source = NoiseSource::flicker_with_frequency_exponent(
                    format!("{}:flicker", mos.name),
                    mos.node_drain,
                    mos.node_source,
                    coefficient,
                    af,
                    ef,
                    current,
                );
                noise_sources.push(if is_classic_noise_model {
                    source.with_identity(crate::analysis::NoiseSourceIdentity::mechanism(
                        &mos.name, "FN",
                    ))
                } else {
                    source
                });
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

        Ok((noise_sources, correlated_noise_sources))
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
        self.run_noise_with_abort(netlist, output_node, frequencies, temperature, &NoAbort)
    }

    /// Run single-ended noise analysis with cooperative cancellation.
    pub fn run_noise_with_abort(
        &self,
        netlist: &Netlist,
        output_node: usize,
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_ports_with_abort(netlist, output_node, None, frequencies, temperature, abort)
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
        self.run_noise_with_input_source_and_abort(
            netlist,
            output_pos,
            output_neg,
            input_source,
            frequencies,
            temperature,
            &NoAbort,
        )
    }

    /// Run input-referred noise analysis with cooperative cancellation.
    #[allow(clippy::too_many_arguments)]
    pub fn run_noise_with_input_source_and_abort(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        input_source: &str,
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            NoiseOutputPort::NodeIds {
                positive: output_pos,
                negative: output_neg,
            },
            Some(input_source),
            frequencies,
            temperature,
            abort,
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
        self.run_noise_ports_with_abort(
            netlist,
            output_pos,
            output_neg,
            frequencies,
            temperature,
            &NoAbort,
        )
    }

    /// Run differential noise analysis with cooperative cancellation.
    pub fn run_noise_ports_with_abort(
        &self,
        netlist: &Netlist,
        output_pos: usize,
        output_neg: Option<usize>,
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            NoiseOutputPort::NodeIds {
                positive: output_pos,
                negative: output_neg,
            },
            None,
            frequencies,
            temperature,
            abort,
        )
    }

    /// Run input-referred noise analysis using case-insensitive SPICE node
    /// names. Ground aliases resolve to node zero. The circuit is built only
    /// once; name resolution and analysis use the same canonical topology.
    #[allow(clippy::too_many_arguments)]
    pub fn run_noise_named_with_input_source_and_abort(
        &self,
        netlist: &Netlist,
        output_pos: &str,
        output_neg: Option<&str>,
        input_source: &str,
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_internal(
            netlist,
            NoiseOutputPort::NodeNames {
                positive: output_pos,
                negative: output_neg,
            },
            Some(input_source),
            frequencies,
            temperature,
            abort,
        )
    }

    /// Non-cancellable convenience wrapper for named-node input-referred
    /// noise analysis.
    #[allow(clippy::too_many_arguments)]
    pub fn run_noise_named_with_input_source(
        &self,
        netlist: &Netlist,
        output_pos: &str,
        output_neg: Option<&str>,
        input_source: &str,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        self.run_noise_named_with_input_source_and_abort(
            netlist,
            output_pos,
            output_neg,
            input_source,
            frequencies,
            temperature,
            &NoAbort,
        )
    }

    /// Compute the short-circuit port-current noise correlation matrix.
    ///
    /// Every entry is a complex cross-power spectral density in A²/Hz using
    /// `E[I_i * conj(I_j)]`. Each named port must be an independent voltage
    /// source: its branch enforces zero small-signal port voltage while its
    /// branch current observes the equivalent Norton noise current. This is
    /// the `Cy` matrix used by SPICE `.SP ... donoise` analysis.
    pub fn run_port_noise_correlation(
        &self,
        netlist: &Netlist,
        port_sources: &[String],
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<Vec<PortNoiseCorrelationResult>, SimulationError> {
        self.run_port_noise_correlation_with_abort(
            netlist,
            port_sources,
            frequencies,
            temperature,
            &NoAbort,
        )
    }

    /// Compute `Cy` with cooperative cancellation.
    pub fn run_port_noise_correlation_with_abort(
        &self,
        netlist: &Netlist,
        port_sources: &[String],
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PortNoiseCorrelationResult>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if port_sources.is_empty() {
            return Err(SimulationError::Circuit(
                "Port-noise analysis requires at least one voltage-source port".to_string(),
            ));
        }
        if frequencies.is_empty() {
            return Err(SimulationError::Circuit(
                "Port-noise analysis requires at least one frequency".to_string(),
            ));
        }
        if let Some(frequency) = frequencies
            .iter()
            .find(|frequency| !frequency.is_finite() || **frequency <= 0.0)
        {
            return Err(SimulationError::Circuit(format!(
                "Port-noise frequencies must be finite and strictly positive, got {frequency}"
            )));
        }
        if !temperature.is_finite() || temperature <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Port-noise temperature must be finite and strictly positive, got {temperature}"
            )));
        }

        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(frequencies.len())?;

        let mut unique_names = HashSet::with_capacity(port_sources.len());
        for source in port_sources {
            if source.trim().is_empty() {
                return Err(SimulationError::Circuit(
                    "Port-noise voltage-source names must not be empty".to_string(),
                ));
            }
            if !unique_names.insert(source.to_ascii_lowercase()) {
                return Err(SimulationError::Circuit(format!(
                    "Port-noise voltage source '{source}' is listed more than once"
                )));
            }
        }

        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::warn_xspice_mif_analysis_boundary(
            &circuit,
            "SP noise",
            "intrinsic XSPICE device-noise sources are not collected because ngspice MIF code models expose DEVnoise = NULL",
        );
        Self::ensure_supported_dynamic_charges(&circuit, "SP noise")?;
        if !circuit.ekv3s.is_empty() {
            return Err(SimulationError::Circuit(
                "SP noise does not support the restricted EKV3 LEVEL=301 VANOISE oracle slice"
                    .to_string(),
            ));
        }

        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit.prepare_behavioral_small_signal(&dc_solution);
        let (noise_sources, correlated_noise_sources) =
            Self::try_collect_noise_sources(&circuit, &dc_solution)?;
        let mut branch_matrix_indices = Vec::with_capacity(port_sources.len());
        for source_name in port_sources {
            let source_index = circuit
                .voltage_sources
                .names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(source_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Port-noise voltage source '{source_name}' was not found"
                    ))
                })?;
            let branch_ordinal = circuit.voltage_sources.branch_indices[source_index];
            let matrix_index = circuit.get_branch_matrix_index(branch_ordinal);
            if matrix_index == 0 || matrix_index > circuit.matrix_size() {
                return Err(SimulationError::Circuit(format!(
                    "Port-noise voltage source '{source_name}' has an invalid branch index"
                )));
            }
            branch_matrix_indices.push(matrix_index - 1);
        }

        let size = circuit.matrix_size();
        let num_ports = branch_matrix_indices.len();
        engine.ensure_result_shape(
            frequencies.len(),
            num_ports
                .saturating_mul(num_ports)
                .saturating_mul(2)
                .saturating_add(1),
        )?;
        let zero = Complex64::new(0.0, 0.0);
        let mut rhs = vec![zero; size];
        let mut results = Vec::with_capacity(frequencies.len());

        for &frequency in frequencies {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let omega = 2.0 * PI * frequency;
            circuit.prepare_behavioral_small_signal_at_frequency(&dc_solution, frequency);
            let mut ac_matrix =
                Self::try_build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, omega)?;
            let mut covariance = vec![vec![zero; num_ports]; num_ports];
            let mut compensation = vec![vec![zero; num_ports]; num_ports];

            let mut solve_transfer = |node_pos: usize,
                                      node_neg: usize|
             -> Result<Vec<Complex64>, SimulationError> {
                rhs.fill(zero);
                Self::stamp_unit_noise_current_rhs(&mut rhs, node_pos, node_neg);
                let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                branch_matrix_indices
                    .iter()
                    .map(|&index| {
                        solution
                            .get(index)
                            .copied()
                            .map(|current| -current)
                            .ok_or_else(|| {
                                SimulationError::Circuit(
                                    "Port-noise branch-current solution is malformed".to_string(),
                                )
                            })
                    })
                    .collect()
            };

            for source in &noise_sources {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let density = source.spectral_density(frequency, temperature);
                if !density.is_finite() || density <= 0.0 {
                    continue;
                }
                let scale = density.sqrt();
                let amplitude = solve_transfer(source.node_pos, source.node_neg)?
                    .into_iter()
                    .map(|gain| gain * scale)
                    .collect::<Vec<_>>();
                Self::add_port_noise_outer_product(&mut covariance, &mut compensation, &amplitude)?;
            }

            for source in &correlated_noise_sources {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let Some(densities) = source.spectral_densities(frequency, temperature) else {
                    continue;
                };
                if !densities.first_psd.is_finite()
                    || !densities.second_psd.is_finite()
                    || densities.first_psd < 0.0
                    || densities.second_psd < 0.0
                {
                    continue;
                }
                let first = solve_transfer(source.first.node_pos, source.first.node_neg)?;
                let second = solve_transfer(source.second.node_pos, source.second.node_neg)?;
                let first_scale = densities.first_psd.sqrt();
                let second_scale =
                    Complex64::from_polar(densities.second_psd.sqrt(), densities.phase_rad);
                let amplitude = first
                    .into_iter()
                    .zip(second)
                    .map(|(first_gain, second_gain)| {
                        first_gain * first_scale + second_gain * second_scale
                    })
                    .collect::<Vec<_>>();
                Self::add_port_noise_outer_product(&mut covariance, &mut compensation, &amplitude)?;
            }

            // Make the mathematical Hermitian invariant exact in the public
            // result and remove only impossible signed zero on its diagonal.
            for row in 0..num_ports {
                covariance[row][row] = Complex64::new(covariance[row][row].re.max(0.0), 0.0);
                for column in (row + 1)..num_ports {
                    let value = (covariance[row][column] + covariance[column][row].conj()) * 0.5;
                    covariance[row][column] = value;
                    covariance[column][row] = value.conj();
                }
            }

            results.push(PortNoiseCorrelationResult {
                frequency,
                current_correlation: covariance,
            });
        }

        Ok(results)
    }

    fn ensure_ekv3_vanoise_noise_fixture(
        circuit: &CircuitData,
        output_pos: usize,
        output_neg: Option<usize>,
        input_source: Option<&str>,
        frequencies: &[Value],
        temperature: Value,
    ) -> Result<(), SimulationError> {
        if circuit.ekv3s.is_empty() {
            return Ok(());
        }

        fn reject(detail: impl std::fmt::Display) -> SimulationError {
            SimulationError::Circuit(format!(
                "unsupported EKV3 LEVEL=301 NOISE contract: the native VANOISE slice is validated only for the Xyce NMOS150 VANOISE fixture; {detail}"
            ))
        }

        fn close(actual: Value, expected: Value) -> bool {
            let tol = 1.0e-12_f64.max(expected.abs() * 1.0e-9);
            actual.is_finite() && (actual - expected).abs() <= tol
        }

        fn node(circuit: &CircuitData, name: &str) -> Result<usize, SimulationError> {
            circuit
                .get_node_by_name(name)
                .ok_or_else(|| reject(format!("fixture node '{name}' is missing")))
        }

        fn vsource_matches(
            circuit: &CircuitData,
            name: &str,
            pos: usize,
            neg: usize,
            dc: Value,
            ac: Value,
        ) -> bool {
            circuit
                .voltage_sources
                .names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .is_some_and(|idx| {
                    circuit.voltage_sources.node_pos[idx] == pos
                        && circuit.voltage_sources.node_neg[idx] == neg
                        && close(circuit.voltage_sources.dc_values[idx], dc)
                        && close(circuit.voltage_sources.ac_magnitudes[idx], ac)
                        && close(circuit.voltage_sources.ac_phases[idx], 0.0)
                })
        }

        const ORACLE_FREQUENCIES: &[Value] = &[1.0e3, 1.0e6, 1.0e8, 1.0e9, 1.0e11];

        if circuit.ekv3s.len() != 1 {
            return Err(reject(format!(
                "expected exactly one EKV3 device, found {}",
                circuit.ekv3s.len()
            )));
        }
        if circuit.device_count() != 11 {
            return Err(reject(format!(
                "expected only M1 plus the VANOISE drain LC fixture and bias/probe sources, found {} devices",
                circuit.device_count()
            )));
        }
        if output_neg.is_some() {
            return Err(reject("fixture uses single-ended output V(D)"));
        }
        if !input_source.is_some_and(|source| source.eq_ignore_ascii_case("vg")) {
            return Err(reject("fixture input source must be 'vg'"));
        }
        if !close(temperature, 298.15) {
            return Err(reject(format!(
                "fixture noise temperature must be 298.15 K, got {temperature}"
            )));
        }
        if frequencies.is_empty()
            || frequencies.iter().any(|freq| {
                !ORACLE_FREQUENCIES
                    .iter()
                    .any(|oracle| close(*freq, *oracle))
            })
        {
            return Err(reject(
                "fixture frequencies must be selected rows from the Xyce VANOISE oracle",
            ));
        }

        let d = node(circuit, "d")?;
        let g = node(circuit, "g")?;
        let s = node(circuit, "s")?;
        let b = node(circuit, "b")?;
        let ga = node(circuit, "ga")?;
        let da = node(circuit, "da")?;
        let sa = node(circuit, "sa")?;
        let ba = node(circuit, "ba")?;
        let one = node(circuit, "1")?;

        let device = &circuit.ekv3s.devices[0];
        if !device.is_validated_nmos150() {
            return Err(reject("fixture requires the validated NMOS150 EKV3 card"));
        }
        if device.node_drain != d
            || device.node_gate != g
            || device.node_source != s
            || device.node_bulk != b
        {
            return Err(reject("EKV3 instance must connect as M1 D G S B"));
        }
        if output_pos != d {
            return Err(reject("fixture output must be node D"));
        }
        if circuit.voltage_sources.len() != 8
            || !vsource_matches(circuit, "vg", g, ga, 0.5, 1.0)
            || !vsource_matches(circuit, "vgprobe", 0, ga, 0.0, 0.0)
            || !vsource_matches(circuit, "vd", one, da, 1.0, 0.0)
            || !vsource_matches(circuit, "vdprobe", 0, da, 0.0, 0.0)
            || !vsource_matches(circuit, "vs", s, sa, 0.0, 0.0)
            || !vsource_matches(circuit, "vsprobe", 0, sa, 0.0, 0.0)
            || !vsource_matches(circuit, "vb", b, ba, 0.0, 0.0)
            || !vsource_matches(circuit, "vbprobe", 0, ba, 0.0, 0.0)
        {
            return Err(reject(
                "fixture voltage-source bias/probe network does not match the Xyce VANOISE deck",
            ));
        }
        if circuit.inductors.len() != 1
            || !circuit.inductors.names[0].eq_ignore_ascii_case("ldrain")
            || circuit.inductors.node_pos[0] != one
            || circuit.inductors.node_neg[0] != d
            || !close(circuit.inductors.inductances[0], 1.0e-3)
        {
            return Err(reject("fixture requires Ldrain 1 D 1m"));
        }
        if circuit.capacitors.len() != 1
            || !circuit.capacitors.names[0].eq_ignore_ascii_case("cdrain")
            || circuit.capacitors.stamps[0].pp.row != d
            || circuit.capacitors.stamps[0].nn.row != 0
            || !close(circuit.capacitors.capacitances[0], 1.0e-3)
        {
            return Err(reject("fixture requires Cdrain D 0 1m"));
        }

        Ok(())
    }

    pub(in crate::engine::advanced) fn run_noise_internal(
        &self,
        netlist: &Netlist,
        output_port: NoiseOutputPort<'_>,
        input_source: Option<&str>,
        frequencies: &[Value],
        temperature: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<NoiseResult>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if frequencies.is_empty() {
            return Err(SimulationError::Circuit(
                "Noise analysis requires at least one frequency".to_string(),
            ));
        }
        if let Some(frequency) = frequencies
            .iter()
            .find(|frequency| !frequency.is_finite() || **frequency <= 0.0)
        {
            return Err(SimulationError::Circuit(format!(
                "Noise frequencies must be finite and strictly positive, got {frequency}"
            )));
        }
        if !temperature.is_finite() || temperature <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Noise temperature must be finite and strictly positive, got {temperature}"
            )));
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(frequencies.len())?;
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let resolve_node = |name: &str| {
            circuit.get_node_by_name(name.trim()).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "Noise output node '{}' was not found",
                    name.trim()
                ))
            })
        };
        let (output_pos, output_neg) = match output_port {
            NoiseOutputPort::NodeIds { positive, negative } => (positive, negative),
            NoiseOutputPort::NodeNames { positive, negative } => (
                resolve_node(positive)?,
                negative.map(resolve_node).transpose()?,
            ),
        };
        Self::warn_xspice_mif_analysis_boundary(
            &circuit,
            "Noise",
            "intrinsic XSPICE device-noise sources are not collected because ngspice MIF code models expose DEVnoise = NULL",
        );
        Self::ensure_supported_dynamic_charges(&circuit, "Noise")?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point for bias-dependent noise.
        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit.prepare_behavioral_small_signal(&dc_solution);
        let (noise_sources, correlated_noise_sources) =
            Self::try_collect_noise_sources(&circuit, &dc_solution)?;
        engine.ensure_result_shape(
            frequencies.len(),
            circuit
                .matrix_size()
                .saturating_mul(2)
                .saturating_add(4)
                .saturating_add(
                    noise_sources
                        .len()
                        .saturating_add(correlated_noise_sources.len())
                        .saturating_mul(3),
                ),
        )?;

        // Compute noise at each frequency
        let mut contribution_catalog = noise_sources
            .iter()
            .map(|source| source.identity.clone())
            .collect::<Vec<_>>();
        contribution_catalog.extend(
            correlated_noise_sources
                .iter()
                .map(|source| crate::analysis::NoiseSourceIdentity::device(&source.device_name)),
        );
        for mos in circuit
            .mosfets
            .devices
            .iter()
            .filter(|mos| matches!(mos.level, 1..=3))
        {
            contribution_catalog.extend(["RD", "RS", "ID", "FN"].map(|mechanism| {
                crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, mechanism)
            }));
        }
        for mos in &circuit.bsim3v3.devices {
            contribution_catalog.extend(["RD", "RS", "ID", "FN"].map(|mechanism| {
                crate::analysis::NoiseSourceIdentity::mechanism(&mos.name, mechanism)
            }));
        }
        for bjt in circuit
            .bjts
            .devices
            .iter()
            .filter(|bjt| bjt.uses_legacy_gummel_poon())
        {
            contribution_catalog.extend(["RC", "RB", "RE", "IC", "IB", "FN"].map(|mechanism| {
                crate::analysis::NoiseSourceIdentity::mechanism(&bjt.name, mechanism)
            }));
        }
        let mut unique_catalog = Vec::with_capacity(contribution_catalog.len());
        for identity in contribution_catalog {
            if !unique_catalog
                .iter()
                .any(|existing: &crate::analysis::NoiseSourceIdentity| {
                    existing.device.eq_ignore_ascii_case(&identity.device)
                        && match (&existing.mechanism, &identity.mechanism) {
                            (None, None) => true,
                            (Some(left), Some(right)) => left.eq_ignore_ascii_case(right),
                            _ => false,
                        }
                })
            {
                unique_catalog.push(identity);
            }
        }

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

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
        Self::ensure_ekv3_vanoise_noise_fixture(
            &circuit,
            output_pos,
            output_neg,
            input_source,
            frequencies,
            temperature,
        )?;

        let has_input_source = match input_source {
            None => false,
            Some(source_name) => {
                if circuit
                    .voltage_sources
                    .names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(source_name))
                {
                    true
                } else if circuit
                    .current_sources
                    .names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(source_name))
                {
                    true
                } else {
                    return Err(SimulationError::Circuit(format!(
                        "Noise input source '{}' not found (expected independent V/I source)",
                        source_name
                    )));
                }
            }
        };

        // Xyce NOISE retains the ordinary AC solution and uses its selected
        // output phasor for input-referred gain. This is the full deck AC
        // excitation: all independent sources, magnitudes, and phases.
        let ac_excitation_rhs = Self::build_ac_excitation_rhs(&circuit);

        // One workspace reused across every noise-source transfer solve.
        let mut rhs = vec![Complex64::new(0.0, 0.0); size];
        let results: Result<Vec<NoiseResult>, SimulationError> = frequencies
            .iter()
            .map(|&freq| {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let omega = 2.0 * PI * freq;
                circuit.prepare_behavioral_small_signal_at_frequency(&dc_solution, freq);
                let mut ac_matrix =
                    Self::try_build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, omega)?;

                let ac_solution = ac_matrix
                    .solve(&ac_excitation_rhs)
                    .map_err(SimulationError::Solver)?;
                let input_gain_sq = if has_input_source {
                    let gain = Self::differential_noise_output(
                        &ac_solution,
                        output_pos,
                        output_neg,
                        num_nodes,
                    );
                    let gain_sq = gain * gain;
                    if !gain_sq.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "Input-referred noise gain is non-finite for source '{}' at {} Hz",
                            input_source.unwrap_or("<unknown>"),
                            freq
                        )));
                    }
                    // Xyce N_ANP_NOISE.C uses N_MINGAIN=1e-20 to retain a
                    // finite input-referred spectrum at transfer nulls.
                    gain_sq.max(1.0e-20)
                } else {
                    1.0
                };

                let mut total_noise_v2_hz = 0.0;
                let mut contributions = Vec::new();

                for source in &noise_sources {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    let si = source.spectral_density(freq, temperature);
                    let output_v2 = if si.is_finite() && si > 0.0 {
                        rhs.fill(Complex64::new(0.0, 0.0));
                        Self::stamp_unit_noise_current_rhs(
                            &mut rhs,
                            source.node_pos,
                            source.node_neg,
                        );

                        let solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                        let v_out = Self::differential_noise_output(
                            &solution, output_pos, output_neg, num_nodes,
                        );
                        si * v_out * v_out
                    } else {
                        0.0
                    };
                    if output_v2.is_finite() && output_v2 > 0.0 {
                        total_noise_v2_hz += output_v2;
                    }
                    contributions.push(NoiseContribution {
                        identity: source.identity.clone(),
                        noise_type: source.noise_type,
                        output_contribution: output_v2.max(0.0),
                        input_contribution: output_v2.max(0.0) / input_gain_sq,
                        percentage: 0.0,
                    });
                }

                for source in &correlated_noise_sources {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    let Some(densities) = source.spectral_densities(freq, temperature) else {
                        continue;
                    };
                    if !densities.first_psd.is_finite()
                        || !densities.second_psd.is_finite()
                        || densities.first_psd < 0.0
                        || densities.second_psd < 0.0
                    {
                        continue;
                    }

                    rhs.fill(Complex64::new(0.0, 0.0));
                    Self::stamp_unit_noise_current_rhs(
                        &mut rhs,
                        source.first.node_pos,
                        source.first.node_neg,
                    );
                    let first_solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                    let first_gain = Self::differential_noise_output_complex(
                        &first_solution,
                        output_pos,
                        output_neg,
                        num_nodes,
                    );

                    rhs.fill(Complex64::new(0.0, 0.0));
                    Self::stamp_unit_noise_current_rhs(
                        &mut rhs,
                        source.second.node_pos,
                        source.second.node_neg,
                    );
                    let second_solution = ac_matrix.solve(&rhs).map_err(SimulationError::Solver)?;
                    let second_gain = Self::differential_noise_output_complex(
                        &second_solution,
                        output_pos,
                        output_neg,
                        num_nodes,
                    );

                    let first_amp = first_gain * densities.first_psd.sqrt();
                    let second_amp = second_gain
                        * Complex64::from_polar(densities.second_psd.sqrt(), densities.phase_rad);
                    let output_v2 = (first_amp + second_amp).norm_sqr();
                    if output_v2.is_finite() && output_v2 > 0.0 {
                        total_noise_v2_hz += output_v2;
                        contributions.push(NoiseContribution {
                            identity: crate::analysis::NoiseSourceIdentity::device(
                                source.device_name.clone(),
                            ),
                            noise_type: source.noise_type,
                            output_contribution: output_v2,
                            input_contribution: output_v2 / input_gain_sq,
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

                let mut branch_currents = ac_solution[num_nodes..].to_vec();
                circuit.capacitors.project_complex_ic_branch_currents(
                    &ac_solution,
                    &mut branch_currents,
                    omega,
                );

                Ok(NoiseResult {
                    frequency: freq,
                    node_names: node_names.clone(),
                    branch_names: branch_names.clone(),
                    voltages: ac_solution[..num_nodes].to_vec(),
                    currents: branch_currents,
                    output_noise_density: total_noise_v2_hz,
                    input_referred_density: if has_input_source {
                        total_noise_v2_hz / input_gain_sq
                    } else {
                        total_noise_v2_hz
                    },
                    input_gain_squared: input_gain_sq,
                    contribution_catalog: unique_catalog.clone(),
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

    fn xyce_frequency_resistor_netlist(body: &str) -> Netlist {
        Netlist::parse_with_options(
            body,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::netlist::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect("frequency-dependent noise deck parses")
    }

    fn xyce_engine() -> Engine {
        Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        )
    }

    #[test]
    fn noise_refreshes_frequency_dependent_behavioral_conductance() {
        let netlist = xyce_frequency_resistor_netlist(
            "live FREQ noise transfer\n\
             .PARAM RUNTIME_R={FREQ}\n\
             RNOISE out 0 1k\n\
             RF out 0 {RUNTIME_R}\n\
             .END\n",
        );
        let results = xyce_engine()
            .run_noise(&netlist, 1, &[10.0, 100.0], 300.15)
            .expect("frequency-dependent noise transfer solves");
        assert_eq!(results.len(), 2);
        assert!(
            results[1].output_noise_density > 80.0 * results[0].output_noise_density,
            "noise operator retained a stale FREQ conductance: {results:?}"
        );
    }

    #[test]
    fn port_noise_refreshes_frequency_dependent_behavioral_conductance() {
        let netlist = xyce_frequency_resistor_netlist(
            "live FREQ port-noise transfer\n\
             .PARAM RUNTIME_R={FREQ}\n\
             VPORT p 0 0\n\
             RNOISE n 0 1k\n\
             RF n p {RUNTIME_R}\n\
             .END\n",
        );
        let results = xyce_engine()
            .run_port_noise_correlation(&netlist, &["VPORT".to_string()], &[10.0, 10_000.0], 300.15)
            .expect("frequency-dependent port-noise transfer solves");
        let low = results[0].current_correlation[0][0].re;
        let high = results[1].current_correlation[0][0].re;
        assert!(
            low > 100.0 * high,
            "port-noise operator retained a stale FREQ conductance: low={low:e}, high={high:e}"
        );
    }

    #[test]
    fn unit_noise_rhs_accepts_unified_branch_equation_ids() {
        let mut rhs = vec![num_complex::Complex64::new(0.0, 0.0); 7];
        Engine::stamp_unit_noise_current_rhs(&mut rhs, 7, 0);

        assert_eq!(rhs[6], num_complex::Complex64::new(1.0, 0.0));
        assert!(rhs[..6].iter().all(|value| *value == 0.0.into()));
    }

    #[cfg(feature = "veriloga-builtins")]
    #[test]
    fn generated_noise_translation_preserves_identity_table_and_potential_axis() {
        use crate::analysis::{NoiseSourceIdentity, NoiseSourceType};
        use crate::device::veriloga_generated::{
            BuiltinEvaluatedNoiseSource, GeneratedMappedNoiseDescriptor, GeneratedNoiseDescriptor,
            GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseInjection,
            GeneratedNoiseKind,
        };

        let mut circuit = crate::CircuitData::new();
        let positive = circuit.get_or_create_node("p");
        let branch = circuit.allocate_branch();
        let endpoint = GeneratedNoiseEndpoint {
            local_node: Some(0),
            name: "p",
            is_internal: false,
        };
        let ground = GeneratedNoiseEndpoint {
            local_node: None,
            name: "GND",
            is_internal: false,
        };
        let table_descriptor = GeneratedNoiseDescriptor {
            mechanism: "TABLE_P_GND_CANONICAL",
            label: Some("display label"),
            kind: GeneratedNoiseKind::Table,
            equation: 0,
            is_current: true,
            branch_ordinal: None,
            pos: endpoint,
            neg: ground,
            table_len: 4,
            table_log_interp: false,
        };
        let table = Engine::generated_noise_source(
            &circuit,
            "R1",
            BuiltinEvaluatedNoiseSource {
                mapped: GeneratedMappedNoiseDescriptor {
                    descriptor: table_descriptor,
                    injection: GeneratedNoiseInjection::Current {
                        node_pos: positive,
                        node_neg: 0,
                    },
                },
                evaluation: GeneratedNoiseEvaluation {
                    active: true,
                    psd: 2.0,
                    exponent: None,
                    table_operands: vec![10.0, 4.0, 1.0, 2.0],
                },
            },
        )
        .expect("translate generated table noise");
        assert_eq!(
            table.identity,
            NoiseSourceIdentity::mechanism("R1", "TABLE_P_GND_CANONICAL")
        );
        assert_eq!(table.noise_type, NoiseSourceType::Table);
        assert_eq!(table.node_pos, positive);
        assert_eq!(table.node_neg, 0);
        assert_eq!(table.spectral_density(1.0, 300.15), 4.0);
        assert_eq!(table.spectral_density(10.0, 300.15), 8.0);
        let malformed = Engine::generated_noise_source(
            &circuit,
            "R1",
            BuiltinEvaluatedNoiseSource {
                mapped: GeneratedMappedNoiseDescriptor {
                    descriptor: table_descriptor,
                    injection: GeneratedNoiseInjection::Current {
                        node_pos: positive,
                        node_neg: 0,
                    },
                },
                evaluation: GeneratedNoiseEvaluation {
                    active: true,
                    psd: 1.0,
                    exponent: None,
                    table_operands: vec![1.0, 2.0],
                },
            },
        )
        .expect_err("active generated tables must match their canonical operand count");
        assert!(malformed.to_string().contains("declares 4 operands"));

        let potential_descriptor = GeneratedNoiseDescriptor {
            mechanism: "WHITE_P_GND_POTENTIAL",
            label: None,
            kind: GeneratedNoiseKind::White,
            equation: 1,
            is_current: false,
            branch_ordinal: Some(0),
            pos: endpoint,
            neg: ground,
            table_len: 0,
            table_log_interp: false,
        };
        let potential = Engine::generated_noise_source(
            &circuit,
            "VNOISE",
            BuiltinEvaluatedNoiseSource {
                mapped: GeneratedMappedNoiseDescriptor {
                    descriptor: potential_descriptor,
                    injection: GeneratedNoiseInjection::Potential { branch },
                },
                evaluation: GeneratedNoiseEvaluation {
                    active: true,
                    psd: 3.0,
                    exponent: None,
                    table_operands: Vec::new(),
                },
            },
        )
        .expect("translate generated potential noise");
        assert_eq!(
            potential.node_pos,
            circuit.num_nodes() + branch,
            "potential noise uses the unified one-based branch equation ID"
        );
        let mut rhs = vec![num_complex::Complex64::new(0.0, 0.0); circuit.matrix_size()];
        Engine::stamp_unit_noise_current_rhs(&mut rhs, potential.node_pos, potential.node_neg);
        assert_eq!(
            rhs[circuit.num_nodes() + branch - 1],
            num_complex::Complex64::new(1.0, 0.0)
        );
    }

    #[cfg(feature = "veriloga-builtins")]
    #[test]
    fn generated_r2_noise_catalog_retains_canonical_mechanisms() {
        let netlist = Netlist::parse(
            r#"
v1 a 0 dc 1
r1 a 0 rmod
.model rmod r2_cmc r=1000 isnoisy=1
.op
.end
"#,
        )
        .expect("R2 noise fixture parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("R2 circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("R2 matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("R2 operating point converges");
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&solution);
        }

        let (sources, correlated) = Engine::collect_noise_sources(&circuit, &solution);
        assert!(correlated.is_empty());
        let r2_sources = sources
            .iter()
            .filter(|source| source.identity.device.eq_ignore_ascii_case("r1"))
            .collect::<Vec<_>>();
        assert_eq!(r2_sources.len(), 2, "R2 exports two canonical mechanisms");
        let mechanisms = r2_sources
            .iter()
            .filter_map(|source| source.identity.mechanism.as_deref())
            .collect::<std::collections::HashSet<_>>();
        assert!(mechanisms.contains("WHITE_N1_N2_THERMAL"));
        assert!(mechanisms.contains("FLICKER_N1_N2_FLICKER"));
        assert!(r2_sources.iter().all(|source| source.node_neg == 0));
        assert!(r2_sources.iter().all(|source| source.node_pos > 0));
        assert!(
            r2_sources
                .iter()
                .find(|source| {
                    source.identity.mechanism.as_deref() == Some("WHITE_N1_N2_THERMAL")
                })
                .expect("thermal mechanism")
                .parameter
                > 0.0
        );
    }

    #[cfg(feature = "veriloga-builtins")]
    fn assert_generated_vbic13_noise_initializes(deck: &str, expected_mechanisms: usize) {
        let netlist = Netlist::parse(deck).expect("VBIC13 oracle deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("VBIC13 circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("VBIC13 matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("VBIC13 operating point converges");
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&solution);
        }

        let (sources, correlated) = Engine::try_collect_noise_sources(&circuit, &solution)
            .expect("VBIC13 generated noise state initializes");
        assert!(correlated.is_empty());
        let vbic = sources
            .iter()
            .filter(|source| source.identity.device.eq_ignore_ascii_case("q1"))
            .collect::<Vec<_>>();
        assert_eq!(vbic.len(), expected_mechanisms);
        assert!(vbic.iter().all(|source| source.parameter.is_finite()));
        assert!(vbic.iter().all(|source| source.af.is_finite()));
        assert!(vbic.iter().all(|source| source.ef.is_finite()));
    }

    #[cfg(feature = "veriloga-builtins")]
    #[test]
    fn generated_vbic13_3t_noise_initializes_for_the_new_analysis() {
        assert_generated_vbic13_noise_initializes(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../tests/xyce/Netlists/VANOISE/commonEmitterBjt_vbic13_3T.cir"
            )),
            13,
        );
    }

    #[cfg(feature = "veriloga-builtins")]
    #[test]
    fn generated_vbic13_4t_noise_initializes_for_the_new_analysis() {
        assert_generated_vbic13_noise_initializes(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../tests/xyce/Netlists/VANOISE/commonEmitterBjt_vbic13.cir"
            )),
            15,
        );
    }

    #[test]
    fn noise_retains_the_canonical_multi_source_ac_solution() {
        let netlist = Netlist::parse(
            "Noise AC-observable parity\n\
             VIN in 0 0 AC 2 30\n\
             ITRIM out 0 0 AC 750u -40\n\
             R1 in out 1k\n\
             R2 out 0 2k\n\
             .end\n",
        )
        .expect("deck parses");
        let frequencies = [17.0, 2.5e6];
        let engine = Engine::default();
        let ac = engine
            .run_ac(&netlist, &frequencies)
            .expect("AC analysis runs");
        let noise = engine
            .run_noise_named_with_input_source(
                &netlist,
                "OUT",
                Some("0"),
                "vin",
                &frequencies,
                300.15,
            )
            .expect("named noise analysis runs");

        assert_eq!(noise.len(), ac.len());
        for (noise_point, ac_point) in noise.iter().zip(&ac) {
            assert_eq!(noise_point.frequency, ac_point.frequency);
            assert_eq!(noise_point.node_names, ac_point.node_names);
            assert_eq!(noise_point.branch_names, ac_point.branch_names);
            assert_eq!(noise_point.voltages.len(), ac_point.voltages.len());
            assert_eq!(noise_point.currents.len(), ac_point.currents.len());
            for (actual, expected) in noise_point.voltages.iter().zip(&ac_point.voltages) {
                assert!(
                    (*actual - *expected).norm() <= 1e-13 * expected.norm().max(1.0),
                    "retained voltage {actual:?} differs from AC {expected:?}"
                );
            }
            for (actual, expected) in noise_point.currents.iter().zip(&ac_point.currents) {
                assert!(
                    (*actual - *expected).norm() <= 1e-13 * expected.norm().max(1.0),
                    "retained current {actual:?} differs from AC {expected:?}"
                );
            }

            let output_index = noise_point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .expect("output name retained");
            let gain_sq = noise_point.voltages[output_index].norm_sqr();
            assert!(gain_sq > 0.0);
            let expected_input_density = noise_point.output_noise_density / gain_sq;
            assert!(
                (noise_point.input_referred_density - expected_input_density).abs()
                    <= 1e-13 * expected_input_density.max(f64::MIN_POSITIVE),
                "input-referred density must use the full ordinary AC output phasor"
            );
        }
    }

    #[test]
    fn named_noise_api_reports_an_unknown_output_node() {
        let netlist = Netlist::parse(
            "Unknown noise output\n\
             V1 in 0 0 AC 1\n\
             R1 in 0 1k\n\
             .end\n",
        )
        .expect("deck parses");
        let error = Engine::default()
            .run_noise_named_with_input_source(&netlist, "missing", None, "V1", &[1.0e3], 300.15)
            .expect_err("unknown output must fail");
        assert!(error.to_string().contains("missing"));
    }

    #[test]
    fn port_noise_correlation_is_hermitian_and_matches_a_series_resistor() {
        let netlist = Netlist::parse(
            "Series resistor port-noise fixture\n\
             V1 p1 0 0 portnum=1 z0=50\n\
             R1 p1 p2 50\n\
             V2 p2 0 0 portnum=2 z0=50\n\
             .end\n",
        )
        .expect("deck parses");
        let temperature = 300.15;
        let frequencies = [1.0e3, 1.0e9];
        let ports = vec!["V1".to_string(), "v2".to_string()];
        let results = Engine::default()
            .run_port_noise_correlation(&netlist, &ports, &frequencies, temperature)
            .expect("port-noise analysis runs");
        assert_eq!(results.len(), frequencies.len());

        let expected = 4.0 * crate::analysis::noise::K_BOLTZMANN * temperature / 50.0;
        for (point, frequency) in results.iter().zip(frequencies) {
            assert_eq!(point.frequency.to_bits(), frequency.to_bits());
            assert_eq!(point.current_correlation.len(), 2);
            let cy = &point.current_correlation;
            let tolerance = expected * 1.0e-11;
            assert!((cy[0][0].re - expected).abs() <= tolerance);
            assert!((cy[1][1].re - expected).abs() <= tolerance);
            assert!((cy[0][1].re + expected).abs() <= tolerance);
            assert!((cy[1][0].re + expected).abs() <= tolerance);
            assert_eq!(cy[0][0].im, 0.0);
            assert_eq!(cy[1][1].im, 0.0);
            assert_eq!(cy[1][0], cy[0][1].conj());
        }
    }

    #[test]
    fn port_noise_correlation_rejects_duplicate_and_unknown_ports() {
        let netlist = Netlist::parse("ports\nV1 p1 0 0\nR1 p1 0 50\n.end\n").expect("deck parses");
        let engine = Engine::default();
        let duplicate = vec!["V1".to_string(), "v1".to_string()];
        let error = engine
            .run_port_noise_correlation(&netlist, &duplicate, &[1.0e3], 300.15)
            .expect_err("duplicate port is rejected")
            .to_string();
        assert!(error.contains("listed more than once"), "{error}");

        let missing = vec!["Vmissing".to_string()];
        let error = engine
            .run_port_noise_correlation(&netlist, &missing, &[1.0e3], 300.15)
            .expect_err("unknown port is rejected")
            .to_string();
        assert!(error.contains("was not found"), "{error}");
    }

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

    #[test]
    fn classic_mos_noise_catalog_owns_series_sources_and_retains_inactive_flicker() {
        let netlist = Netlist::parse(MOS_RDRS_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("d").expect("output node");
        let result = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e3], 300.15)
            .expect("noise analysis runs")
            .into_iter()
            .next()
            .expect("one noise point");

        let contribution = |probe: &str| {
            let probe = crate::analysis::NoiseContributionProbe::parse(probe)
                .expect("contribution probe parses");
            result.contribution(&probe).expect("mechanism is valid")
        };
        let rd = contribution("DNO(M1,RD)");
        let rs = contribution("DNO(m1,rs)");
        let id = contribution("DNO(M1,id)");
        let fn_noise = contribution("DNO(m1,FN)");
        assert!(rd > 0.0, "externalized RD must contribute under M1");
        assert!(rs > 0.0, "externalized RS must contribute under M1");
        assert!(id > 0.0, "channel thermal noise must contribute under M1");
        assert_eq!(fn_noise, 0.0, "KF=0 keeps valid FN inactive");
        let whole = contribution("DNO(M1)");
        let parts = rd + rs + id + fn_noise;
        assert!((whole - parts).abs() <= 1.0e-14 * whole.max(parts));
    }

    /// Instance DTEMP must heat the channel thermal source and both
    /// externalized drain/source resistances exactly as mos1noi.c does
    /// (shot-free deck: every noise source is temperature-bearing).
    #[test]
    fn mos_dtemp_noise_matches_the_ngspice46_oracle() {
        let deck = MOS_RDRS_NOISE_DECK.replace(
            "M1 D G 0 0 NM W=20u L=2u",
            "M1 D G 0 0 NM W=20u L=2u DTEMP=150",
        );
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

    /// The acceptance gate for the GP base-resistance increment: closing
    /// the factor-five deficit required the base-prime node in the matrix.
    /// The promotion lands as one piece with its limiting discipline: the
    /// constant base part externalizes onto a real resistor, the device
    /// update replaces its junction voltages per Newton iterate via pnjlim
    /// against the previous iterate (bjtload.c), the companion anchors at
    /// the limited point, a limited iterate reports nonconvergence
    /// (CKTnoncon), and Newton takes full node steps for GP circuits. A
    /// first topology-only attempt passed this gate but broke the general
    /// suite 8/8 -> 5/8 through the engine-side clamp acting on the bare
    /// junction.
    #[test]
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

    #[test]
    fn gp_noise_catalog_owns_parasitics_and_retains_inactive_flicker() {
        let netlist = Netlist::parse(GP_RB_NOISE_DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("c").expect("output node");
        let result = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e5], 300.15)
            .expect("noise analysis runs")
            .into_iter()
            .next()
            .expect("one noise point");

        let contribution = |probe: &str| {
            let probe = crate::analysis::NoiseContributionProbe::parse(probe)
                .expect("contribution probe parses");
            result.contribution(&probe).expect("mechanism is valid")
        };
        let mechanisms = ["rc", "RB", "re", "IC", "ib"]
            .map(|mechanism| contribution(&format!("DNO(q1,{mechanism})")));
        assert!(mechanisms.iter().all(|value| *value > 0.0));
        let fn_noise = contribution("DNO(Q1,FN)");
        assert_eq!(fn_noise, 0.0, "KF=0 keeps valid GP FN inactive");
        let parts = mechanisms.into_iter().sum::<f64>() + fn_noise;
        let whole = contribution("DNO(Q1)");
        assert!((whole - parts).abs() <= 1.0e-14 * whole.max(parts));
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
            let tolerance = 1e-6 * freq_ref.abs().max(1.0);
            assert!(
                (result.frequency - freq_ref).abs() <= tolerance,
                "{label}: frequency grid diverged from the oracle at {:e} Hz: ours {:.8e} vs oracle {:.8e}",
                freq_ref,
                result.frequency,
                freq_ref,
            );
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

    const BSIM4_MODELS45: &str =
        include_str!("../../../src/device/mosfet/bsim4v8/testdata/models45.lib");
    const BSIM4_FNOI1_TNOI0_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim4_fnoi1_tnoi0_noise_ngspice46.dat");
    const BSIM4_FNOI0_TNOI0_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim4_fnoi0_tnoi0_noise_ngspice46.dat");
    const BSIM4_TNOI2_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim4_tnoi2_noise_ngspice46.dat");
    const BSIM4_TNOI1_SERIES_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim4_tnoi1_series_noise_ngspice46.dat");
    const BSIM4_RDSMOD1_TNOI1_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim4_rdsmod1_tnoi1_noise_ngspice46.dat");
    const BSIM4_RBODYMOD1_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim4_rbodymod1_noise_ngspice46.dat");
    const BSIM3_MODELS018: &str =
        include_str!("../../../src/device/mosfet/bsim3v3/testdata/models018.lib");
    const BSIM3_NOIMOD1_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim3_noimod1_noise_ngspice46.dat");
    const BSIM3_NOIMOD2_ORACLE: &str =
        include_str!("../../../tests/testdata/bsim3_noimod2_noise_ngspice46.dat");

    fn bsim3_noise_deck(model_header_suffix: &str) -> String {
        let models = if model_header_suffix.is_empty() {
            BSIM3_MODELS018.to_string()
        } else {
            BSIM3_MODELS018.replace(
                ".model n018 nmos level=49",
                &format!(".model n018 nmos level=49 {model_header_suffix}"),
            )
        };
        format!(
            "BSIM3 noise oracle deck\n\n\
             VDD VDD 0 1.8\n\
             VIN IN 0 DC 0.9 AC 1\n\
             RD VDD OUT 3k\n\
             M1 OUT IN 0 0 n018 W=1u L=0.18u AD=0.2p AS=0.2p PD=2.4u PS=2.4u NRD=0 NRS=0\n\n\
             .OPTIONS NOACCT RELTOL=1e-6\n\
             .NOISE v(out) VIN DEC 5 10 100Meg\n\n\
             {models}\n\n\
             .END\n"
        )
    }

    fn bsim4_noise_deck(model_header_suffix: &str) -> String {
        let models = if model_header_suffix.is_empty() {
            BSIM4_MODELS45.to_string()
        } else {
            BSIM4_MODELS45.replace(
                ".model n45 nmos level=54 version=4.8",
                &format!(".model n45 nmos level=54 version=4.8 {model_header_suffix}"),
            )
        };
        format!(
            "BSIM4 noise oracle deck\n\n\
             VDD VDD 0 1.1\n\
             VIN IN 0 DC 0.75 AC 1\n\
             RD VDD OUT 3k\n\
             M1 OUT IN 0 0 n45 W=1u L=45n AD=0.1p AS=0.1p PD=2.2u PS=2.2u NRD=0 NRS=0\n\n\
             .OPTIONS NOACCT RELTOL=1e-6\n\
             .NOISE v(out) VIN DEC 5 10 100Meg\n\n\
             {models}\n\n\
             .END\n"
        )
    }

    fn bsim4_noise_deck_with_load_and_instance(
        model_header_suffix: &str,
        load_resistance: f64,
        instance_suffix: &str,
    ) -> String {
        let models = if model_header_suffix.is_empty() {
            BSIM4_MODELS45.to_string()
        } else {
            BSIM4_MODELS45.replace(
                ".model n45 nmos level=54 version=4.8",
                &format!(".model n45 nmos level=54 version=4.8 {model_header_suffix}"),
            )
        };
        format!(
            "BSIM4 noise oracle deck\n\n\
             VDD VDD 0 1.1\n\
             VIN IN 0 DC 0.75 AC 1\n\
             RD VDD OUT {load_resistance}\n\
             M1 OUT IN 0 0 n45 W=1u L=45n AD=0.1p AS=0.1p PD=2.2u PS=2.2u {instance_suffix}\n\n\
             .OPTIONS NOACCT RELTOL=1e-6\n\
             .NOISE v(out) VIN DEC 5 10 100Meg\n\n\
             {models}\n\n\
             .END\n"
        )
    }

    #[test]
    fn bsim4_default_flicker_noise_matches_the_ngspice46_oracle() {
        let deck = bsim4_noise_deck("");
        assert_onoise_matches(
            &deck,
            BSIM4_FNOI1_TNOI0_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-2,
            "bsim4-fnoi1-tnoi0",
        );
    }

    #[test]
    fn bsim4_legacy_flicker_noise_matches_the_ngspice46_oracle() {
        let deck = bsim4_noise_deck("FNOIMOD=0 TNOIMOD=0 KF=2e-24 AF=1.3 EF=0.8");
        assert_onoise_matches(
            &deck,
            BSIM4_FNOI0_TNOI0_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-2,
            "bsim4-fnoi0-tnoi0",
        );
    }

    #[test]
    fn bsim4_tnoi2_correlated_thermal_noise_matches_the_ngspice46_oracle() {
        let deck = bsim4_noise_deck("FNOIMOD=0 TNOIMOD=2 KF=0");
        assert_onoise_matches(
            &deck,
            BSIM4_TNOI2_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-2,
            "bsim4-tnoi2",
        );
    }

    #[test]
    fn bsim4_tnoi1_series_noise_adjustment_matches_the_ngspice46_oracle() {
        let deck = bsim4_noise_deck_with_load_and_instance(
            "FNOIMOD=0 TNOIMOD=1 KF=0",
            150.0,
            "NRD=80 NRS=80",
        );
        assert_onoise_matches(
            &deck,
            BSIM4_TNOI1_SERIES_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-4,
            "bsim4-tnoi1-series",
        );
    }

    #[test]
    fn bsim4_rdsmod1_tnoi1_external_series_noise_matches_the_ngspice46_oracle() {
        let deck = bsim4_noise_deck_with_load_and_instance(
            "FNOIMOD=0 TNOIMOD=1 KF=0 RDSMOD=1 RDW=300 RSW=280 RDWMIN=20 RSWMIN=18",
            150.0,
            "NRD=80 NRS=80",
        );
        assert_onoise_matches(
            &deck,
            BSIM4_RDSMOD1_TNOI1_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-4,
            "bsim4-rdsmod1-tnoi1",
        );
    }

    #[test]
    fn bsim4_rbodymod1_substrate_resistor_noise_matches_the_ngspice46_oracle() {
        let deck = bsim4_noise_deck(
            "RBODYMOD=1 RBPB=5 RBPD=15 RBPS=15 RBDB=15 RBSB=15 GBMIN=1e-10 \
             FNOIMOD=0 TNOIMOD=0 KF=0",
        );
        assert_onoise_matches(
            &deck,
            BSIM4_RBODYMOD1_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-4,
            "bsim4-rbodymod1",
        );
    }

    #[test]
    fn bsim4_rbodymod1_collects_all_substrate_resistor_noise_sources() {
        let deck = bsim4_noise_deck(
            "RBODYMOD=1 RBPB=5 RBPD=15 RBPS=15 RBDB=15 RBSB=15 GBMIN=1e-10 \
             FNOIMOD=0 TNOIMOD=0 KF=0",
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");
        circuit.update_nonlinear(&solution);

        let device = &circuit.bsim4v8.devices[0];
        let expected = [
            (
                "m1.rbps",
                device.node_bulk,
                device.node_source_body,
                device.core.inst.body_prime_source_conductance,
            ),
            (
                "m1.rbpd",
                device.node_bulk,
                device.node_drain_body,
                device.core.inst.body_prime_drain_conductance,
            ),
            (
                "m1.rbpb",
                device.node_bulk,
                device.node_bulk_external,
                device.core.inst.body_prime_bulk_conductance,
            ),
            (
                "m1.rbsb",
                device.node_bulk_external,
                device.node_source_body,
                device.core.inst.body_source_bulk_conductance,
            ),
            (
                "m1.rbdb",
                device.node_bulk_external,
                device.node_drain_body,
                device.core.inst.body_drain_bulk_conductance,
            ),
        ];

        let (sources, _) = Engine::collect_noise_sources(&circuit, &solution);
        for (name, node_pos, node_neg, conductance) in expected {
            let source = sources
                .iter()
                .find(|source| source.identity.device.eq_ignore_ascii_case(name))
                .unwrap_or_else(|| {
                    panic!("{name} thermal noise missing; sources={sources:#?}");
                });
            assert_eq!(source.noise_type.label(), "thermal");
            assert_eq!(source.node_pos, node_pos, "{name} positive node");
            assert_eq!(source.node_neg, node_neg, "{name} negative node");
            let expected_resistance = 1.0 / (conductance * device.multiplier.max(0.0));
            let rel = (source.parameter - expected_resistance).abs() / expected_resistance;
            assert!(
                rel <= 1e-12,
                "{name} resistance: got {:.12e}, expected {:.12e}, rel {:.3e}",
                source.parameter,
                expected_resistance,
                rel,
            );
        }
    }

    #[test]
    fn bsim4_rbodymod2_bodymode1_collects_only_body_prime_bulk_noise_source() {
        let deck = bsim4_noise_deck(
            "RBODYMOD=2 RBPB=5 RBPD=15 RBPS=15 RBDB=15 RBSB=15 GBMIN=1e-10 \
             FNOIMOD=0 TNOIMOD=0 KF=0",
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");
        circuit.update_nonlinear(&solution);

        let device = &circuit.bsim4v8.devices[0];
        assert_eq!(device.core.inst.body_resistance_mode, 1);

        let (sources, _) = Engine::collect_noise_sources(&circuit, &solution);
        let mut rbody_names = sources
            .iter()
            .map(|source| source.identity.device.to_ascii_lowercase())
            .filter(|name| {
                matches!(
                    name.as_str(),
                    "m1.rbps" | "m1.rbpd" | "m1.rbpb" | "m1.rbsb" | "m1.rbdb"
                )
            })
            .collect::<Vec<_>>();
        rbody_names.sort();
        assert_eq!(
            rbody_names,
            vec!["m1.rbpb".to_string()],
            "RBODYMOD=2 bodymode=1 should only expose rbpb thermal noise; sources={sources:#?}",
        );

        let rbpb = sources
            .iter()
            .find(|source| source.identity.device.eq_ignore_ascii_case("m1.rbpb"))
            .expect("m1.rbpb thermal noise source");
        assert_eq!(rbpb.node_pos, device.node_bulk);
        assert_eq!(rbpb.node_neg, device.node_bulk_external);
        let expected_resistance =
            1.0 / (device.core.inst.body_prime_bulk_conductance * device.multiplier.max(0.0));
        let rel = (rbpb.parameter - expected_resistance).abs() / expected_resistance;
        assert!(
            rel <= 1e-12,
            "m1.rbpb resistance: got {:.12e}, expected {:.12e}, rel {:.3e}",
            rbpb.parameter,
            expected_resistance,
            rel,
        );
    }

    #[test]
    fn bsim3_noimod1_noise_matches_the_ngspice46_oracle() {
        let deck = bsim3_noise_deck("NOIMOD=1 KF=2e-24 AF=1.2 EF=0.9");
        assert_onoise_matches(
            &deck,
            BSIM3_NOIMOD1_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-2,
            "bsim3-noimod1",
        );
    }

    #[test]
    fn bsim3_noise_catalog_owns_series_sources_and_retains_inactive_flicker() {
        let deck = bsim3_noise_deck("")
            .replace("rsh=0", "rsh=100")
            .replace("NRD=0 NRS=0", "NRD=1 NRS=1");
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let output = circuit.get_node_by_name("out").expect("output node");
        let result = engine
            .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e3], 300.15)
            .expect("noise analysis runs")
            .into_iter()
            .next()
            .expect("one noise point");

        let contribution = |probe: &str| {
            let probe = crate::analysis::NoiseContributionProbe::parse(probe)
                .expect("contribution probe parses");
            result.contribution(&probe).expect("mechanism is valid")
        };
        let rd = contribution("DNO(M1,RD)");
        let rs = contribution("DNO(m1,rs)");
        let id = contribution("DNO(M1,id)");
        let fn_noise = contribution("DNO(m1,FN)");
        assert!(rd > 0.0, "lowered BSIM3 RD must contribute under M1");
        assert!(rs > 0.0, "lowered BSIM3 RS must contribute under M1");
        assert!(
            id > 0.0,
            "BSIM3 channel thermal noise must contribute under M1"
        );
        assert_eq!(fn_noise, 0.0, "KF=0 keeps valid BSIM3 FN inactive");
        let whole = contribution("DNO(M1)");
        let parts = rd + rs + id + fn_noise;
        assert!((whole - parts).abs() <= 1.0e-14 * whole.max(parts));
    }

    #[test]
    #[should_panic(expected = "frequency grid diverged")]
    fn bsim_noise_oracle_helper_rejects_shifted_frequency_grid() {
        let shifted_oracle = BSIM3_NOIMOD1_ORACLE
            .lines()
            .map(|line| {
                if line.trim_start().starts_with('#') || line.trim().is_empty() {
                    return line.to_string();
                }

                let mut fields = line.split_whitespace();
                let freq: f64 = fields.next().unwrap().parse().unwrap();
                let onoise: f64 = fields.next().unwrap().parse().unwrap();
                format!("{:.8e} {:.8e}", freq * 1.01, onoise)
            })
            .collect::<Vec<_>>()
            .join("\n");
        let deck = bsim3_noise_deck("NOIMOD=1 KF=2e-24 AF=1.2 EF=0.9");

        assert_onoise_matches(
            &deck,
            &shifted_oracle,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-2,
            "bsim3-shifted-grid",
        );
    }

    #[test]
    fn bsim3_noimod2_noise_matches_the_ngspice46_oracle() {
        let deck = bsim3_noise_deck("NOIMOD=2");
        assert_onoise_matches(
            &deck,
            BSIM3_NOIMOD2_ORACLE,
            "out",
            "VIN",
            5,
            10.0,
            1e8,
            5e-2,
            "bsim3-noimod2",
        );
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
            .replace(
                ".NOISE v(c) VIN DEC 5 10k 100Meg",
                ".NOISE v(c) VIN DEC 5 10k 10Meg",
            );
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
    const RES_QUIET_ORACLE: &str = include_str!("../../../tests/testdata/res_quiet_ngspice46.dat");

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

    #[test]
    fn ekv26_noise_sources_match_xyce26_equation_oracle() {
        let deck = "\
EKV26 noise source testbench

VD D 0 DC 1
VG G 0 DC 0.8
VS S 0 DC 0
VB B 0 DC 0
M1 D G S B N W=10u L=1u AS=0 AD=0 PS=0 PD=0

.OPTIONS TEMP=27 GMIN=0

.MODEL N NMOS (LEVEL=260 TNOM=27 COX=2e-3 XJ=300n VTO=0.5 TCV=0
+ GAMMA=0 PHI=0.5 KP=150u BEX=0 THETA=0 E0=0 UCRIT=2e6 UCEX=0
+ LAMBDA=0 DL=0 DW=0 WETA=0 LETA=0 Q0=0 LK=0.4u IBA=0
+ IBB=400Meg IBBT=0 IBN=1 RSH=0 HDIF=0 AVTO=1u AKP=1u AGAMMA=1u
+ KF=2e-22 AF=1.3)

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

        let (sources, _) = Engine::collect_noise_sources(&circuit, &solution);
        let source = |name: &str| {
            sources
                .iter()
                .find(|source| source.identity.device.eq_ignore_ascii_case(name))
                .unwrap_or_else(|| {
                    panic!(
                        "missing EKV26 noise source {name}; got {:?}",
                        sources
                            .iter()
                            .map(|source| source.identity.device.as_str())
                            .collect::<Vec<_>>()
                    )
                })
        };
        let thermal = source("m1:thermal");
        let flicker = source("m1:flicker");

        assert_eq!(thermal.noise_type, crate::analysis::NoiseSourceType::White);
        assert_eq!(
            flicker.noise_type,
            crate::analysis::NoiseSourceType::Flicker
        );
        assert_eq!(thermal.node_pos, circuit.get_node_by_name("d").unwrap());
        assert_eq!(thermal.node_neg, circuit.get_node_by_name("s").unwrap());
        assert_eq!(flicker.node_pos, thermal.node_pos);
        assert_eq!(flicker.node_neg, thermal.node_neg);
        assert_eq!(flicker.af, 1.0);
        assert_eq!(flicker.ef, 1.3);
        assert_eq!(flicker.current, 1.0);

        let expected_thermal = 3.426_240_019_709_936_4e-24;
        let expected_flicker_at_1hz = 8.503_778_042_345_18e-16;
        let thermal_rel = (thermal.parameter - expected_thermal).abs() / expected_thermal;
        let flicker_rel =
            (flicker.parameter - expected_flicker_at_1hz).abs() / expected_flicker_at_1hz;
        assert!(
            thermal_rel <= 2.0e-6,
            "EKV26 thermal PSD should follow Xyce Gn: got {:e}, want {:e}, rel {:.3e}",
            thermal.parameter,
            expected_thermal,
            thermal_rel
        );
        assert!(
            flicker_rel <= 2.0e-4,
            "EKV26 flicker 1Hz PSD should follow Xyce gm^2 formula: got {:e}, want {:e}, rel {:.3e}",
            flicker.parameter,
            expected_flicker_at_1hz,
            flicker_rel
        );
        let expected_10hz = 4.261_984_992_423_322e-17;
        let flicker_10hz = flicker.spectral_density(10.0, 300.15);
        let flicker_10hz_rel = (flicker_10hz - expected_10hz).abs() / expected_10hz;
        assert!(
            flicker_10hz_rel <= 2.0e-4,
            "EKV26 AF exponent should be the frequency exponent: got {:e}, want {:e}, rel {:.3e}",
            flicker_10hz,
            expected_10hz,
            flicker_10hz_rel
        );
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

        let (sources, _) = Engine::collect_noise_sources(&circuit, &solution);
        let flicker = sources
            .iter()
            .find(|source| source.identity.device.ends_with(":flicker"))
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
