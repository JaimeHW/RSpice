//! Native Meyer capacitances and their physical voltage sensitivities.
//!
//! Meyer storage obeys C(v) dv/dt and generally has no terminal charge
//! potential. Periodic rate coordinates therefore need dC/dV as well as C.
use super::*;
use crate::device::mosfet::dual::Dual3;

impl Mosfet {
    /// Full intrinsic capacitances (not the half values retained by the
    /// transient integrator). Columns differentiate physical Vgs, Vds, Vbs.
    pub(crate) fn meyer_capacitances_with_derivatives(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> ([Value; 3], [[Value; 3]; 3]) {
        if self.uses_legacy_bsim() {
            return ([0.0; 3], [[0.0; 3]; 3]);
        }
        let p = self.polarity();
        let gate = p * Dual3::variable(vgs, 0);
        let drain = p * Dual3::variable(vds, 1);
        let bulk = p * Dual3::variable(vbs, 2);
        let forward = drain.value >= 0.0;
        let (gate, drain, bulk) = if forward {
            (gate, drain, bulk)
        } else {
            (gate - drain, -drain, bulk - drain)
        };
        let phi = if self.level == 1 {
            self.phi
        } else {
            self.phi.max(1e-12)
        };
        let (von, vdsat) = match self.level {
            1 => {
                let (value, minus_slope) = crate::device::semiconductor::mos1_threshold(
                    p * self.vto,
                    self.gamma,
                    phi,
                    phi.sqrt(),
                    bulk.value,
                );
                let von = bulk.map_unary(value, -minus_slope);
                (von, (gate - von).max_const(0.0))
            }
            2 => self.level2_meyer_state_dual(gate, drain, bulk),
            3 | 9 => self.mos3_meyer_state_dual(gate, drain, bulk),
            6 => {
                let sqrt_phi = phi.sqrt();
                let sarg = if bulk.value <= 0.0 {
                    (phi - bulk).max_const(0.0).sqrt()
                } else {
                    (sqrt_phi - bulk / (2.0 * sqrt_phi.max(1e-12))).max_const(0.0)
                };
                let von = p * self.vto + self.gamma * (sarg - sqrt_phi)
                    - self.gamma1 * bulk
                    - self.sigma * drain;
                let overdrive = (gate - von).max_const(0.0);
                let saturation = if overdrive.value > 0.0 {
                    self.kv * overdrive.powf(self.nv)
                } else {
                    Dual3::constant(0.0)
                };
                (von, saturation)
            }
            // Simplified fallback models have no qualified native periodic law.
            _ => return ([Value::NAN; 3], [[Value::NAN; 3]; 3]),
        };
        let xyce = self.body_junction_model == MosBodyJunctionModel::XyceClassicLinearizedReverse;
        let mut caps = partition(
            gate - von,
            drain,
            vdsat,
            phi,
            self.oxide_capacitance_total(),
            xyce,
        );
        if !forward {
            caps.swap(0, 1);
        }
        (
            caps.map(|c| 2.0 * c.value),
            caps.map(|c| c.derivative.map(|d| 2.0 * d)),
        )
    }
}

fn partition(
    overdrive: Dual3,
    drain: Dual3,
    saturation: Dual3,
    phi: Value,
    oxide: Value,
    xyce: bool,
) -> [Dual3; 3] {
    let zero = Dual3::constant(0.0);
    if overdrive.value <= -phi {
        return [zero, zero, Dual3::constant(oxide / 2.0)];
    }
    let bulk = -overdrive * oxide / (2.0 * phi);
    if overdrive.value <= -phi / 2.0 {
        return [zero, zero, bulk];
    }
    let weak = overdrive.value <= 0.0;
    let base = if weak {
        overdrive * oxide / (1.5 * phi) + oxide / 3.0
    } else {
        Dual3::constant(oxide / 3.0)
    };
    if xyce && weak {
        return [base, zero, bulk];
    }
    let saturation = if xyce {
        saturation
    } else {
        saturation.max_const(0.025)
    };
    let bulk = if weak { bulk.max_const(0.0) } else { zero };
    if drain.value >= saturation.value {
        return [base.max_const(0.0), zero, bulk];
    }
    let difference = saturation - drain;
    let denominator = 2.0 * saturation - drain;
    let denominator2 = denominator * denominator;
    [
        (base * (1.0 - difference * difference / denominator2)).max_const(0.0),
        (base * (1.0 - saturation * saturation / denominator2)).max_const(0.0),
        bulk,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn native_meyer_capacitance_gradients_follow_all_classic_models() {
        for level in [1, 2, 3, 6, 9] {
            for advanced in [false, true] {
                for xyce in [false, true] {
                    for polarity in [1.0, -1.0] {
                        let params: HashMap<_, _> = [
                            ("LEVEL", level as Value),
                            ("VTO", polarity * 0.6),
                            ("KP", 50e-6),
                            ("GAMMA", 0.5),
                            ("PHI", 0.7),
                            ("TOX", 20e-9),
                            ("NSUB", 1e16),
                            ("XJ", 0.2e-6),
                            ("LD", 0.05e-6),
                            ("UO", 600.0),
                            ("UCRIT", 1e4),
                            ("UEXP", 0.2),
                            ("DELTA", 0.4),
                            ("ETA", 0.05),
                            ("THETA", 0.1),
                            ("KV", 0.8),
                            ("NV", 1.2),
                            ("GAMMA1", 0.03),
                            ("SIGMA", 0.04),
                            ("VMAX", if advanced { 1e5 } else { 0.0 }),
                            ("NFS", if advanced { 1e11 } else { 0.0 }),
                        ]
                        .into_iter()
                        .map(|(k, v)| (k.to_owned(), v))
                        .collect();
                        let mut mos = if polarity > 0.0 {
                            Mosfet::new_nmos("m".into(), 1, 2, 3, 4)
                        } else {
                            Mosfet::new_pmos("m".into(), 1, 2, 3, 4)
                        }
                        .with_params(&params)
                        .with_geometry(10e-6, 1e-6);
                        mos.multiplicity = 2.3;
                        if xyce {
                            mos.body_junction_model =
                                MosBodyJunctionModel::XyceClassicLinearizedReverse;
                        }
                        for reverse in [false, true] {
                            for gate in [-0.4, 0.2, 0.55, 1.2, 2.0] {
                                for drain in [0.011, 0.15, 1.1] {
                                    for body in [-0.3, 0.12] {
                                        let bias = if reverse {
                                            [
                                                polarity * (gate - drain),
                                                -polarity * drain,
                                                polarity * (body - drain),
                                            ]
                                        } else {
                                            [polarity * gate, polarity * drain, polarity * body]
                                        };
                                        let (caps, gradients) = mos
                                            .meyer_capacitances_with_derivatives(
                                                bias[0], bias[1], bias[2],
                                            );
                                        let native = |v: [Value; 3]| {
                                            let (gs, gd, gb) = mos
                                                .transient_capacitance_halves_at(v[0], v[1], v[2]);
                                            [2.0 * gs, 2.0 * gd, 2.0 * gb]
                                        };
                                        let expected = native(bias);
                                        let scale = mos.oxide_capacitance_total();
                                        for row in 0..3 {
                                            assert!(
                                                (caps[row] - expected[row]).abs() < scale * 1e-10,
                                                "level {level} advanced={advanced} xyce={xyce} p={polarity} bias={bias:?} cap {row}: {} vs {}",
                                                caps[row],
                                                expected[row]
                                            );
                                        }
                                        for col in 0..3 {
                                            let h = 1e-5;
                                            let mut plus = bias;
                                            plus[col] += h;
                                            let mut minus = bias;
                                            minus[col] -= h;
                                            let plus = native(plus);
                                            let minus = native(minus);
                                            for row in 0..3 {
                                                let fd = (plus[row] - minus[row]) / (2.0 * h);
                                                assert!(
                                                    (gradients[row][col] - fd).abs()
                                                        < scale * 2e-5 + fd.abs() * 2e-4,
                                                    "level {level} advanced={advanced} xyce={xyce} p={polarity} bias={bias:?} gradient {row},{col}: {} vs {fd}",
                                                    gradients[row][col]
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
