#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::{CanonicalModelValues, Instance, PARAMETER_MODEL_FLAGS};
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Lanes, rspice_eval_ddt, rspice_eval_idt, rspice_limexp, rspice_limited_exp, rspice_limited_exp_derivative};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, Weak};

static CANONICAL_MODEL_CACHE: OnceLock<Mutex<HashMap<Box<[u64]>, Weak<CanonicalModelValues>>>> = OnceLock::new();

fn canonical_model_cache() -> &'static Mutex<HashMap<Box<[u64]>, Weak<CanonicalModelValues>>> {
    CANONICAL_MODEL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn canonical_model_cache_lookup(key: &[u64]) -> Option<Arc<CanonicalModelValues>> {
    let mut cache = canonical_model_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let found = cache.get(key).and_then(Weak::upgrade);
    if found.is_none() {
        cache.remove(key);
    }
    found
}

fn canonical_model_cache_intern(
    key: Box<[u64]>,
    candidate: Arc<CanonicalModelValues>,
) -> Arc<CanonicalModelValues> {
    let mut cache = canonical_model_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = cache.get(key.as_ref()).and_then(Weak::upgrade) {
        return existing;
    }
    cache.retain(|_, values| values.strong_count() > 0);
    cache.insert(key, Arc::downgrade(&candidate));
    candidate
}

impl Instance {
    fn canonical_model_key(&self) -> Box<[u64]> {
        let mut key = Vec::with_capacity(194);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[1] = values[0];
        self.canonical_staged[2] = values[1];
        self.canonical_staged[17] = values[2];
        self.canonical_staged[3] = values[3];
        self.canonical_staged[18] = values[4];
        self.canonical_staged[19] = values[5];
        self.canonical_staged[21] = values[6];
        self.canonical_staged[22] = values[7];
        self.canonical_staged[23] = values[8];
        self.canonical_staged[20] = values[9];
        self.canonical_staged[24] = values[10];
        self.canonical_staged[25] = values[11];
        self.canonical_staged[4] = values[12];
        self.canonical_staged[5] = values[13];
        self.canonical_staged[6] = values[14];
        self.canonical_staged[26] = values[15];
        self.canonical_staged[27] = values[16];
        self.canonical_staged[7] = values[17];
        self.canonical_staged[29] = values[18];
        self.canonical_staged[8] = values[19];
        self.canonical_staged[30] = values[20];
        self.canonical_staged[9] = values[21];
        self.canonical_staged[10] = values[22];
        self.canonical_staged[31] = values[23];
        self.canonical_staged[11] = values[24];
        self.canonical_staged[12] = values[25];
        self.canonical_staged[13] = values[26];
        self.canonical_staged[14] = values[27];
        self.canonical_staged[28] = values[28];
        self.canonical_staged[32] = values[29];
        self.canonical_staged[33] = values[30];
        self.canonical_staged[34] = values[31];
        self.canonical_staged[35] = values[32];
        self.canonical_staged[36] = values[33];
        self.canonical_staged[37] = values[34];
        self.canonical_staged[38] = values[35];
        self.canonical_staged[39] = values[36];
        self.canonical_staged[40] = values[37];
        self.canonical_staged[41] = values[38];
        self.canonical_staged[42] = values[39];
        self.canonical_staged[46] = values[40];
        self.canonical_staged[47] = values[41];
        self.canonical_staged[48] = values[42];
        self.canonical_staged[49] = values[43];
        self.canonical_staged[51] = values[44];
        self.canonical_staged[53] = values[45];
        self.canonical_staged[55] = values[46];
        self.canonical_staged[57] = values[47];
        self.canonical_model_values = Some(values);
    }

    fn canonical_model_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_model_values.is_some() {
            return;
        }
        let key = self.canonical_model_key();
        if let Some(values) = canonical_model_cache_lookup(key.as_ref()) {
            self.canonical_install_model_values(values);
            return;
        }
        let produced: CanonicalModelValues = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = if parameter_given[100] { 1.0 } else { 0.0 };
                let C = 3.0015e2f64;
                let E = 0e0f64;
                let I = parameters[4];
                let K = 1e0f64;
                let M = 4e0f64;
                let O = 2e0f64;
                let Q = 3e0f64;
                let T = parameters[5];
                let AC = parameters[6];
                let AH = parameters[38];
                let AT = 0e0f64;
                let AW = 0e0f64;
                let AZ = 0e0f64;
                let BC = 0e0f64;
                let BF = 0e0f64;
                let BI = 0e0f64;
                let BL = 0e0f64;
                let BO = 0e0f64;
                let BQ = parameters[7];
                let mut oH = 0.0;
                let mut oL = 0.0;
                let mut oP = 0.0;
                let mut oR = 0.0;
                let mut oS = 0.0;
                let mut oY = 0.0;
                let mut oAE = 0.0;
                let mut oAI = 0.0;
                let mut oAJ = 0.0;
                let mut oAK = 0.0;
                let mut oAL = 0.0;
                let mut oAM = 0.0;
                let mut oAN = 0.0;
                let mut oAO = 0.0;
                let mut oAP = 0.0;
                let mut oAQ = 0.0;
                let mut oAR = 0.0;
                let mut oBS = 0.0;
                let mut oBT = 0.0;
                let D = if A != 0.0 {
                    let B = parameters[100] + 2.7315e2f64;
                    B
                } else {
                    C
                };
                let F = if parameters[66] > E { 1.0 } else { 0.0 };
                let G = if (if (if parameter_given[43] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[44] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if G != 0.0 {
                    let H = 5e-1f64 / parameters[44];
                    oH = H;
                } else {
                }
                let J = if I == E { 1.0 } else { 0.0 };
                if J != 0.0 {
                } else {
                    let L = if I == K { 1.0 } else { 0.0 };
                    oL = L;
                    if L != 0.0 {
                    } else {
                        let P = if I == O { 1.0 } else { 0.0 };
                        oP = P;
                        if P != 0.0 {
                        } else {
                            let R = if I == Q { 1.0 } else { 0.0 };
                            oR = R;
                            if R != 0.0 {
                            } else {
                                let S = if I == M { 1.0 } else { 0.0 };
                                oS = S;
                            }
                        }
                    }
                }
                let N = if (if J != 0.0 || (if I == K { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if I == M { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let U = if T == E { 1.0 } else { 0.0 };
                let Z;
                let AA;
                if U != 0.0 {
                    Z = E;
                    AA = E;
                } else {
                    let V = -parameters[85];
                    let W = rspice_limexp((V * parameters[83]));
                    let X = rspice_limexp((V * parameters[84]));
                    let Y = if T == K { 1.0 } else { 0.0 };
                    oY = Y;
                    Z = W;
                    AA = X;
                }
                let AB = 1e-3f64 * parameters[82];
                let AD = if AC == E { 1.0 } else { 0.0 };
                if AD != 0.0 {
                } else {
                    let AE = if AC == K { 1.0 } else { 0.0 };
                    oAE = AE;
                    if AE != 0.0 {
                        let AI = O * AH;
                        oAI = AI;
                    } else {
                        let AJ = if AC == O { 1.0 } else { 0.0 };
                        oAJ = AJ;
                        if AJ != 0.0 {
                            let AK = O * AH;
                            oAK = AK;
                        } else {
                            let AL = if AC == Q { 1.0 } else { 0.0 };
                            oAL = AL;
                            if AL != 0.0 {
                                let AM = K - AH;
                                oAM = AM;
                                let AN = O * AH;
                                oAN = AN;
                            } else {
                                let AO = if AC == M { 1.0 } else { 0.0 };
                                oAO = AO;
                                if AO != 0.0 {
                                    let AP = (parameters[39] * parameters[40]) * ((parameters[41] + K).powf(-5e-1f64));
                                    oAP = AP;
                                    let AQ = K - AH;
                                    oAQ = AQ;
                                    let AR = O * AH;
                                    oAR = AR;
                                } else {
                                }
                            }
                        }
                    }
                }
                let AF = parameters[56] / Q;
                let AG = if (if AC == O { 1.0 } else { 0.0 }) != 0.0 || (if AC == M { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AS = if parameters[58] > E { 1.0 } else { 0.0 };
                let AU = if AS != 0.0 {
                    E
                } else {
                    AT
                };
                let AV = if (if parameters[63] > E { 1.0 } else { 0.0 }) != 0.0 || (if parameters[62] > E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AX = if AV != 0.0 {
                    E
                } else {
                    AW
                };
                let AY = if parameters[60] > E { 1.0 } else { 0.0 };
                let BA = if AY != 0.0 {
                    E
                } else {
                    AZ
                };
                let BB = if parameters[51] > E { 1.0 } else { 0.0 };
                let BD = if BB != 0.0 {
                    E
                } else {
                    BC
                };
                let BE = if parameters[49] > E { 1.0 } else { 0.0 };
                let BG = if BE != 0.0 {
                    E
                } else {
                    BF
                };
                let BH = if parameters[46] > E { 1.0 } else { 0.0 };
                let BJ = if BH != 0.0 {
                    E
                } else {
                    BI
                };
                let BK = if parameters[50] > E { 1.0 } else { 0.0 };
                let BM = if BK != 0.0 {
                    E
                } else {
                    BL
                };
                let BN = if (if parameters[47] > E { 1.0 } else { 0.0 }) != 0.0 || (if parameters[48] > E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BP = if BN != 0.0 {
                    E
                } else {
                    BO
                };
                let BR = if BQ == E { 1.0 } else { 0.0 };
                if BR != 0.0 {
                } else {
                    let BS = if BQ == K { 1.0 } else { 0.0 };
                    oBS = BS;
                    if BS != 0.0 {
                    } else {
                        let BT = if BQ == O { 1.0 } else { 0.0 };
                        oBT = BT;
                    }
                }
            [D, F, G, oH, J, oL, oP, oR, oS, N, U, oY, Z, AB, AA, AD, oAE, oAI, oAJ, oAK, oAL, oAM, oAN, oAO, oAP, oAQ, oAR, AF, AG, AS, AV, AY, BB, BE, BH, BK, BN, BR, oBS, oBT, AU, AX, BA, BD, BG, BJ, BM, BP]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 21] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = if parameter_given[3] { 1.0 } else { 0.0 };
                let C = parameters[0];
                let D = 0e0f64;
                let E = 0e0f64;
                let G = staged[37];
                let I = staged[38];
                let J = 0e0f64;
                let M = staged[39];
                let N = 0e0f64;
                let Q = staged[40];
                let R = 0e0f64;
                let T = staged[41];
                let AC = 0e0f64;
                let AD = 0e0f64;
                let AT = parameters[90];
                let AV = 0e0f64;
                let AX = 0e0f64;
                let AY = 0e0f64;
                let AZ = 0e0f64;
                let BA = 0e0f64;
                let BB = 0e0f64;
                let BI = 0e0f64;
                let BJ = 0e0f64;
                let BM = 0e0f64;
                let BN = 0e0f64;
                let mut oB = 0.0;
                let mut oAS = 0.0;
                let mut oAU = 0.0;
                let mut oBC = 0.0;
                if A != 0.0 {
                    let B = parameters[3] + 2.7315e2f64;
                    oB = B;
                } else {
                }
                let F = if C != 0.0 {
                    D
                } else {
                    E
                };
                let H;
                if G != 0.0 {
                    let K = if C != 0.0 {
                        J
                    } else {
                        E
                    };
                    H = K;
                } else {
                    H = E;
                }
                let L;
                if I != 0.0 {
                    let O = if C != 0.0 {
                        N
                    } else {
                        E
                    };
                    L = O;
                } else {
                    L = E;
                }
                let P;
                if M != 0.0 {
                    let S = if C != 0.0 {
                        R
                    } else {
                        E
                    };
                    P = S;
                } else {
                    P = E;
                }
                let U;
                let V;
                let W;
                let X;
                let Y;
                let Z;
                let AA;
                let AB;
                if Q != 0.0 {
                    let AE;
                    let AF;
                    if C != 0.0 {
                        AE = AC;
                        AF = AD;
                    } else {
                        AE = E;
                        AF = E;
                    }
                    U = AE;
                    V = AF;
                    W = E;
                    X = E;
                    Y = E;
                    Z = E;
                    AA = E;
                    AB = E;
                } else {
                    let AG;
                    let AH;
                    let AI;
                    let AJ;
                    let AK;
                    let AL;
                    if T != 0.0 {
                        let AM;
                        let AN;
                        let AO;
                        let AP;
                        let AQ;
                        let AR;
                        if C != 0.0 {
                            let AS = (parameters[87] * parameters[86]).sqrt();
                            oAS = AS;
                            let AU = if AT > E { 1.0 } else { 0.0 };
                            oAU = AU;
                            let AW = if AU != 0.0 {
                                AV
                            } else {
                                E
                            };
                            AM = AX;
                            AN = AY;
                            AO = AZ;
                            AP = BA;
                            AQ = BB;
                            AR = AW;
                        } else {
                            AM = E;
                            AN = E;
                            AO = E;
                            AP = E;
                            AQ = E;
                            AR = E;
                        }
                        AG = AM;
                        AH = AN;
                        AI = AO;
                        AJ = AP;
                        AK = AQ;
                        AL = AR;
                    } else {
                        AG = E;
                        AH = E;
                        AI = E;
                        AJ = E;
                        AK = E;
                        AL = E;
                    }
                    U = E;
                    V = E;
                    W = AG;
                    X = AH;
                    Y = AI;
                    Z = AJ;
                    AA = AK;
                    AB = AL;
                }
                let BD;
                let BE;
                let BF;
                let BG;
                if C != 0.0 {
                    let BC = if AT > E { 1.0 } else { 0.0 };
                    oBC = BC;
                    let BK;
                    let BL;
                    if BC != 0.0 {
                        BK = BI;
                        BL = BJ;
                    } else {
                        BK = E;
                        BL = E;
                    }
                    BD = BM;
                    BE = BN;
                    BF = BK;
                    BG = BL;
                } else {
                    BD = E;
                    BE = E;
                    BF = E;
                    BG = E;
                }
                let BH = if parameters[1] == 1e0f64 { 1.0 } else { 0.0 };
            [oB, oAS, oAU, oBC, BH, F, H, L, P, U, V, W, X, Y, Z, AA, AB, BD, BE, BF, BG]
        };
        self.canonical_staged[16] = produced[0];
        self.canonical_staged[15] = produced[1];
        self.canonical_staged[43] = produced[2];
        self.canonical_staged[44] = produced[3];
        self.canonical_staged[45] = produced[4];
        self.canonical_staged[50] = produced[5];
        self.canonical_staged[52] = produced[6];
        self.canonical_staged[54] = produced[7];
        self.canonical_staged[56] = produced[8];
        self.canonical_staged[58] = produced[9];
        self.canonical_staged[59] = produced[10];
        self.canonical_staged[60] = produced[11];
        self.canonical_staged[61] = produced[12];
        self.canonical_staged[62] = produced[13];
        self.canonical_staged[63] = produced[14];
        self.canonical_staged[64] = produced[15];
        self.canonical_staged[65] = produced[16];
        self.canonical_staged[66] = produced[17];
        self.canonical_staged[67] = produced[18];
        self.canonical_staged[68] = produced[19];
        self.canonical_staged[69] = produced[20];
        self.canonical_instance_valid = true;
    }

    fn canonical_temperature_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let temperature = ctx.temperature();
        let thermal_voltage = ctx.thermal_voltage();
        if self.canonical_temperature_valid
            && self.canonical_temperature == temperature
            && self.canonical_thermal_voltage == thermal_voltage
        {
            return;
        }
        let produced: [f64; 1] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = if parameter_given[3] { 1.0 } else { 0.0 };
                let B = staged[16];
                let D = if A != 0.0 {
                    B
                } else {
                    let C = temperature + parameters[2];
                    C
                };
            [D]
        };
        self.canonical_staged[0] = produced[0];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_model_stage(ctx);
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5]), ctx.branch_current(self.branches[6]), ctx.branch_current(self.branches[7]), ctx.branch_current(self.branches[8]), ctx.branch_current(self.branches[9]), ctx.branch_current(self.branches[10]), ctx.branch_current(self.branches[11]), ctx.branch_current(self.branches[12]), ctx.branch_current(self.branches[13]), ctx.branch_current(self.branches[14]), ctx.branch_current(self.branches[15]), ctx.branch_current(self.branches[16]), ctx.branch_current(self.branches[17]), ctx.branch_current(self.branches[18])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 4327 => 0usize, 4334 => 1usize, 4346 => 2usize, 4348 => 3usize, 4352 => 4usize, 4356 => 5usize, 4360 => 6usize, 4364 => 7usize, 4368 => 8usize, 4375 => 9usize, 4398 => 10usize, 4403 => 11usize, 4455 => 12usize, 4476 => 13usize, 4501 => 14usize, 4654 => 15usize, 4752 => 16usize, _ => usize::MAX };
            rspice_eval_ddt(
                &mut ddt_state.ddt_current,
                &mut ddt_state.ddt_previous,
                &mut ddt_state.ddt_older,
                &mut ddt_state.ddt_initialized,
                &mut ddt_state.ddt_derivative_current,
                &mut ddt_state.ddt_derivative_previous,
                ddt_active,
                ddt_coefficients.derivative_scale,
                ddt_coefficients.previous_value_scale,
                ddt_coefficients.older_value_scale,
                ddt_coefficients.previous_derivative_scale,
                slot,
                value,
            )
        };
            let A = node_potentials[12];
            let B = node_potentials[8];
            let D = 1e0f64;
            let E = 1e0f64;
            let G = node_potentials[10];
            let H = node_potentials[5];
            let J = 1e0f64;
            let K = 1e0f64;
            let N = -1e0f64;
            let R = node_potentials[11];
            let T = 1e0f64;
            let V = node_potentials[4];
            let X = 1e0f64;
            let Z = parameters[1];
            let AA = node_potentials[3];
            let AB = 0e0f64;
            let AC = 2e0f64;
            let AD = 1e0f64;
            let AE = 1e0f64;
            let AG = staged[0];
            let AI = 0e0f64;
            let AL = 8.617333262e-5f64;
            let AR = 0e0f64;
            let AV = parameters[68];
            let AW = 1e0f64;
            let AX = parameters[8];
            let BA = parameters[80];
            let BB = parameters[20];
            let BE = parameters[72];
            let BF = parameters[26];
            let BI = parameters[73];
            let BJ = parameters[29];
            let BM = parameters[74];
            let BN = parameters[58];
            let BQ = parameters[75];
            let BT = parameters[59];
            let BW = parameters[78];
            let BY = parameters[9];
            let CA = parameters[71];
            let CD = parameters[30];
            let CG = parameters[36];
            let CJ = parameters[79];
            let CL = parameters[45];
            let CN = parameters[81];
            let CP = parameters[21];
            let CR = parameters[4];
            let CS = 4e0f64;
            let CU = parameters[62];
            let CV = parameters[63];
            let DW = staged[17];
            let EO = parameters[43];
            let ER = parameters[19];
            let EU = parameters[64];
            let EX = 1e-12f64;
            let FA = parameters[11];
            let FE = parameters[69];
            let FJ = parameters[70];
            let FK = parameters[13];
            let FN = parameters[10];
            let FO = parameters[15];
            let FV = parameters[22];
            let GI = parameters[12];
            let GT = 5e-1f64;
            let GX = parameters[14];
            let HC = staged[18];
            let HF = parameters[16];
            let HN = staged[19];
            let HS = staged[20];
            let IE = parameters[17];
            let IK = parameters[23];
            let IZ = staged[21];
            let KA = staged[22];
            let MB = staged[23];
            let MO = parameters[65];
            let MZ = Lanes([0e0f64; 6]);
            let NF = parameters[57];
            let NH = parameters[48];
            let NK = parameters[47];
            let NM = parameters[50];
            let OC = parameters[76];
            let OJ = parameters[77];
            let OK = parameters[66];
            let ON = staged[24];
            let OO = -1e0f64;
            let OV = parameters[83];
            let OZ = parameters[84];
            let PE = staged[25];
            let PP = parameters[85];
            let PS = staged[5];
            let PU = parameters[42];
            let QC = parameters[31];
            let QF = parameters[38];
            let QN = parameters[33];
            let QR = parameters[35];
            let QV = parameters[37];
            let RD = staged[26];
            let RT = parameters[27];
            let RU = parameters[25];
            let RV = Lanes([0e0f64; 4]);
            let RW = Lanes([0e0f64; 4]);
            let RX = staged[27];
            let SI = parameters[56];
            let SK = 1e0f64;
            let SN = ddt_scale();
            let SP = staged[14];
            let SR = 1e0f64;
            let SV = staged[28];
            let TE = staged[29];
            let TW = staged[8];
            let UQ = staged[30];
            let UZ = parameters[40];
            let VE = parameters[41];
            let VG = -1.5e0f64;
            let VI = 0e0f64;
            let VM = staged[9];
            let VS = parameters[39];
            let WD = staged[31];
            let WU = -5e-1f64;
            let WZ = staged[12];
            let XC = staged[13];
            let ZH = 1e0f64;
            let ZI = parameters[28];
            let ZN = parameters[24];
            let ZT = 1e0f64;
            let AAC = parameters[55];
            let AAD = branch_unknown_flows[1];
            let AAF = 1e0f64;
            let AAJ = staged[32];
            let AAN = Lanes([0e0f64; 7]);
            let AAO = 0e0f64;
            let AAT = staged[33];
            let ABC = Lanes([0e0f64; 3]);
            let ABD = Lanes([0e0f64; 3]);
            let ABK = node_potentials[14];
            let ABL = 1e0f64;
            let ABM = parameters[61];
            let ABR = staged[34];
            let ABS = parameters[60];
            let ABV = Lanes([0e0f64; 2]);
            let ABY = staged[35];
            let ABZ = node_potentials[13];
            let ACA = 1e0f64;
            let ACB = parameters[51];
            let ACE = Lanes([0e0f64; 2]);
            let ACH = parameters[0];
            let ACI = staged[36];
            let ACJ = parameters[49];
            let ACM = Lanes([0e0f64; 2]);
            let ACP = staged[37];
            let ACQ = parameters[46];
            let ACS = 1e0f64;
            let ACU = 0e0f64;
            let ACX = parameters[54];
            let ACZ = 1e0f64;
            let ADD = staged[38];
            let ADE = branch_unknown_flows[11];
            let ADG = 1e0f64;
            let ADJ = Lanes([0e0f64; 7]);
            let ADM = parameters[53];
            let ADO = 1e0f64;
            let ADS = staged[39];
            let ADT = branch_unknown_flows[15];
            let ADV = 1e0f64;
            let ADY = Lanes([0e0f64; 7]);
            let AEB = parameters[52];
            let AED = 1e0f64;
            let AEH = 1e0f64;
            let AEM = staged[40];
            let AEN = staged[41];
            let AFA = 0e0f64;
            let AFB = 0e0f64;
            let AFC = Lanes([0e0f64; 3]);
            let AFD = Lanes([0e0f64; 2]);
            let AGD = 5.5226012e-23f64;
            let AGE = parameters[88];
            let AGG = staged[15];
            let AGL = 3.141592653589793e0f64;
            let AGN = node_potentials[17];
            let AGO = 1e0f64;
            let AGQ = node_potentials[18];
            let AGR = 1e0f64;
            let AHA = staged[45];
            let AHH = -1e0f64;
            let AHM = parameters[67];
            let AHT = Lanes([0e0f64; 7]);
            let AIL = 1e0f64;
            let ANN = node_potentials[16];
            let ANO = 1e-15f64;
            let ANP = 1e-12f64;
            let C = A - B;
            let F = Lanes([0.0, D]) - Lanes([E, 0.0]);
            let I = G - H;
            let L = Lanes([0.0, J]) - Lanes([K, 0.0]);
            let M = -I;
            let O = L * N;
            let P = H - B;
            let Q = Lanes([K, 0.0]) - Lanes([0.0, E]);
            let S = R - B;
            let U = Lanes([0.0, T]) - Lanes([E, 0.0]);
            let W = V - B;
            let Y = Lanes([X, 0.0]) - Lanes([0.0, E]);
            let AJ;
            let AK;
            if Z != 0.0 {
                let AF = AE * ((AC * (if AA >= AB { 1.0 } else { 0.0 })) - AD);
                let AH = AG + (AA.abs());
                AJ = AH;
                AK = AF;
            } else {
                AJ = AG;
                AK = AI;
            }
            let AM = AJ * AL;
            let AN = AK * AL;
            let AO = AJ - staged[1];
            let AP = AO.abs();
            let AQ = AK * ((AC * (if AO >= AB { 1.0 } else { 0.0 })) - AD);
            let AS = if (if AP > AR { 1.0 } else { 0.0 }) != 0.0 || staged[2] != 0.0 { 1.0 } else { 0.0 };
            let CW;
            let CX;
            let CY;
            let CZ;
            let DA;
            let DB;
            let DC;
            let DD;
            let DE;
            let DF;
            let DG;
            let DH;
            let DI;
            let DJ;
            let DK;
            let DL;
            let DM;
            let DN;
            let DO;
            let DP;
            let DQ;
            let DR;
            let DS;
            let DT;
            let DU;
            let DV;
            if AS != 0.0 {
                let AT = AP.abs();
                let AU = AQ * ((AC * (if AP >= AB { 1.0 } else { 0.0 })) - AD);
                let AY = AX * (AW + (AV * AT));
                let AZ = (AU * AV) * AX;
                let BC = BB * (AW + (BA * AT));
                let BD = (AU * BA) * BB;
                let BG = BF * (AW + (BE * AT));
                let BH = (AU * BE) * BF;
                let BK = BJ * (AW + (BI * AT));
                let BL = (AU * BI) * BJ;
                let BO = BN * (AW + (BM * AT));
                let BP = (AU * BM) * BN;
                let BR = AU * BQ;
                let BS = AW + (BQ * AT);
                let BU = BT * BS;
                let BV = BR * BT;
                let BX = AQ * BW;
                let BZ = BY + (BW * AP);
                let CB = AQ * CA;
                let CC = AW + (CA * AP);
                let CE = CD * CC;
                let CF = CB * CD;
                let CH = CG * CC;
                let CI = CB * CG;
                let CK = AQ * CJ;
                let CM = CL + (CJ * AP);
                let CO = AQ * CN;
                let CQ = CP + (CN * AP);
                let CT = if (if (if CR == AW { 1.0 } else { 0.0 }) != 0.0 || (if CR == CS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[6] == CS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EI;
                let EJ;
                let EK;
                let EL;
                if CT != 0.0 {
                    let DX = AQ * AP;
                    let DY = (DX + DX) * BQ;
                    let DZ = AW + (BQ * (AP * AP));
                    let EA = CU * DZ;
                    let EB = DY * CU;
                    let EC = CV * DZ;
                    let ED = DY * CV;
                    EI = EA;
                    EJ = EC;
                    EK = EB;
                    EL = ED;
                } else {
                    let EE = CU * BS;
                    let EF = BR * CU;
                    let EG = CV * BS;
                    let EH = BR * CV;
                    EI = EE;
                    EJ = EG;
                    EK = EF;
                    EL = EH;
                }
                CW = BZ;
                CX = CQ;
                CY = AY;
                CZ = BC;
                DA = BO;
                DB = CM;
                DC = CE;
                DD = CH;
                DE = BG;
                DF = BK;
                DG = BU;
                DH = EI;
                DI = EJ;
                DJ = BX;
                DK = CO;
                DL = AZ;
                DM = BD;
                DN = BP;
                DO = CK;
                DP = CF;
                DQ = CI;
                DR = BH;
                DS = BL;
                DT = BV;
                DU = EK;
                DV = EL;
            } else {
                CW = BY;
                CX = CP;
                CY = AX;
                CZ = BB;
                DA = BN;
                DB = CL;
                DC = CD;
                DD = CG;
                DE = BF;
                DF = BJ;
                DG = BT;
                DH = CU;
                DI = CV;
                DJ = AI;
                DK = AI;
                DL = AI;
                DM = AI;
                DN = AI;
                DO = AI;
                DP = AI;
                DQ = AI;
                DR = AI;
                DS = AI;
                DT = AI;
                DU = AI;
                DV = AI;
            }
            let EP;
            let EQ;
            if DW != 0.0 {
                let EM = staged[3] / AM;
                let EN = ((AN * EM) * N) / AM;
                EP = EM;
                EQ = EN;
            } else {
                EP = EO;
                EQ = AI;
            }
            let ES = ER * P;
            let ET = ES.cosh();
            let EV = Y * EU;
            let EW = ((Q * ER) * (ES.sinh())) * ET;
            let EY = EX + (ET * ET);
            let EZ = parameters[18] / EY;
            let FB = FA * (AW + EZ);
            let FC = AP.abs();
            let FD = AQ * ((AC * (if AP >= AB { 1.0 } else { 0.0 })) - AD);
            let FF = AW + (FE * FC);
            let FG = FB * FF;
            let FH = (((((EW + EW) * EZ) * N) / EY) * FA) * FF;
            let FI = Lanes([0.0, FH[0], FH[1]]) + Lanes([((FD * FE) * FB), 0.0, 0.0]);
            let FL = FK * (AW + (FJ * FC));
            let FM = (FD * FJ) * FK;
            let FP = (FO * P).tanh();
            let FQ = ((Q * FO) * (AD - (FP * FP))) * FN;
            let FR = Lanes([DJ, 0.0, 0.0]) + Lanes([0.0, FQ[0], FQ[1]]);
            let FS = Lanes([FR[0], 0.0, FR[1], FR[2]]) - Lanes([0.0, EV[0], 0.0, EV[1]]);
            let FT = M - CX;
            let FU = Lanes([0.0, O[0], O[1]]) - Lanes([DK, 0.0, 0.0]);
            let FW = FV * FT;
            let FX = ((FU * FV) * FT) + (FU * FW);
            let FY = (((CW - FN) + (FN * FP)) - (EU * W)) - (FW * FT);
            let FZ = AW + (BW * FC);
            let GA = FY * FZ;
            let GB = ((Lanes([FS[0], FS[1], FS[2], FS[3], 0.0]) - Lanes([FX[0], 0.0, FX[1], 0.0, FX[2]])) * FZ) + Lanes([((FD * BW) * FY), 0.0, 0.0, 0.0, 0.0]);
            let GC = C - GA;
            let GD = Lanes([0.0, 0.0, 0.0, F[0], 0.0, F[1]]) - Lanes([GB[0], GB[1], GB[2], GB[3], GB[4], 0.0]);
            let GE = GC * GC;
            let GF = GD * GC;
            let GG = GF + GF;
            let GH = FI * GC;
            let GJ = GI * GE;
            let GK = GG * GI;
            let GL = FL * GC;
            let GM = ((FG * GC) + GJ) + (GL * GE);
            let GN = ((Lanes([GH[0], 0.0, GH[1], GH[2], 0.0, 0.0]) + (GD * FG)) + GK) + (((Lanes([(FM * GC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GD * FL)) * GE) + (GG * GL));
            let GO = GM.tanh();
            let GP = GN * (AD - (GO * GO));
            let GQ = AW + GO;
            let GR = rspice_limexp(GM);
            let GS = rspice_limexp((-GM));
            let GU = (GT * (GR - GS)).tanh();
            let GV = (((GN * GR) - ((GN * N) * GS)) * GT) * (AD - (GU * GU));
            let GW = AW + GU;
            let GY = GX + (FO * GQ);
            let GZ = Q * GY;
            let HA = (GY * P).tanh();
            let HB = (((GP * FO) * P) + Lanes([0.0, 0.0, GZ[0], GZ[1], 0.0, 0.0])) * (AD - (HA * HA));
            let HO;
            let HP;
            let HQ;
            let HR;
            if HC != 0.0 {
                let HD = CY * GQ;
                let HE = HD * HA;
                let HG = Q * HF;
                let HH = rspice_limexp(FT);
                let HI = Lanes([(DM * HH), 0.0, 0.0]) + ((FU * HH) * CZ);
                let HJ = (AW + (HF * P)) + (CZ * HH);
                let HK = HE * HJ;
                let HL = (Lanes([0.0, HG[0], HG[1], 0.0]) + Lanes([HI[0], HI[1], 0.0, HI[2]])) * HE;
                let HM = ((((Lanes([(DL * GQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GP * CY)) * HA) + (HB * HD)) * HJ) + Lanes([HL[0], 0.0, HL[1], HL[2], HL[3], 0.0]);
                HO = GW;
                HP = HK;
                HQ = GV;
                HR = HM;
            } else {
                let JA;
                let JB;
                let JC;
                let JD;
                if HN != 0.0 {
                    let HT = I - GA;
                    let HU = Lanes([0.0, 0.0, L[0], 0.0, L[1]]) - GB;
                    let HV = HT * HT;
                    let HW = HU * HT;
                    let HX = HW + HW;
                    let HY = HV * HT;
                    let HZ = FI * HT;
                    let IA = (((FG * HT) + (GI * HV)) + (FL * HY)).tanh();
                    let IB = (((Lanes([HZ[0], 0.0, HZ[1], HZ[2], 0.0]) + (HU * FG)) + (HX * GI)) + (Lanes([(FM * HY), 0.0, 0.0, 0.0, 0.0]) + (((HX * HT) + (HU * HV)) * FL))) * (AD - (IA * IA));
                    let IC = AW + IA;
                    let ID = GX + (FO * IC);
                    let IF = HF + (IE * GQ);
                    let IG = CY * GQ;
                    let IH = AW + HA;
                    let II = IG * IH;
                    let IJ = Q * IF;
                    let IL = rspice_limexp((IK * (P - CX)));
                    let IM = Lanes([(DM * IL), 0.0, 0.0]) + ((((Lanes([0.0, Q[0], Q[1]]) - Lanes([DK, 0.0, 0.0])) * IK) * IL) * CZ);
                    let IN = (AW + (IF * P)) + (CZ * IL);
                    let IO = HF + (IE * IC);
                    let IP = Q * ID;
                    let IQ = (ID * P).tanh();
                    let IR = CY * IC;
                    let IS = AW - IQ;
                    let IT = IR * IS;
                    let IU = Q * IO;
                    let IV = AW - (IO * P);
                    let IW = ((((Lanes([(DL * IC), 0.0, 0.0, 0.0, 0.0]) + (IB * CY)) * IS) + ((((((IB * FO) * P) + Lanes([0.0, 0.0, IP[0], IP[1], 0.0])) * (AD - (IQ * IQ))) * N) * IR)) * IV) + (((((IB * IE) * P) + Lanes([0.0, 0.0, IU[0], IU[1], 0.0])) * N) * IT);
                    let IX = GT * ((II * IN) - (IT * IV));
                    let IY = ((((((Lanes([(DL * GQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GP * CY)) * IH) + (HB * IG)) * IN) + (((((GP * IE) * P) + Lanes([0.0, 0.0, IJ[0], IJ[1], 0.0, 0.0])) + Lanes([IM[0], 0.0, IM[1], IM[2], 0.0, 0.0])) * II)) - Lanes([IW[0], IW[1], IW[2], IW[3], IW[4], 0.0])) * GT;
                    JA = GW;
                    JB = IX;
                    JC = GV;
                    JD = IY;
                } else {
                    let KB;
                    let KC;
                    let KD;
                    let KE;
                    if IZ != 0.0 {
                        let JE = FL * GE;
                        let JF = (GC + GJ) + (JE * GC);
                        let JG = FG * JF;
                        let JH = FI * JF;
                        let JI = Lanes([JH[0], 0.0, JH[1], JH[2], 0.0, 0.0]) + (((GD + GK) + (((Lanes([(FM * GE), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GG * FL)) * GC) + (GD * JE))) * FG);
                        let JJ = rspice_limexp(JG);
                        let JK = rspice_limexp((-JG));
                        let JL = (GT * (JJ - JK)).tanh();
                        let JM = (((JI * JJ) - ((JI * N) * JK)) * GT) * (AD - (JL * JL));
                        let JN = AW + JL;
                        let JO = GX + (FO * JN);
                        let JP = Q * JO;
                        let JQ = (JO * P).tanh();
                        let JR = HF + (IE * JN);
                        let JS = CY * JN;
                        let JT = JS * JQ;
                        let JU = Q * JR;
                        let JV = rspice_limexp((IK * FT));
                        let JW = Lanes([(DM * JV), 0.0, 0.0]) + (((FU * IK) * JV) * CZ);
                        let JX = (AW + (JR * P)) + (CZ * JV);
                        let JY = JT * JX;
                        let JZ = ((((Lanes([(DL * JN), 0.0, 0.0, 0.0, 0.0, 0.0]) + (JM * CY)) * JQ) + (((((JM * FO) * P) + Lanes([0.0, 0.0, JP[0], JP[1], 0.0, 0.0])) * (AD - (JQ * JQ))) * JS)) * JX) + (((((JM * IE) * P) + Lanes([0.0, 0.0, JU[0], JU[1], 0.0, 0.0])) + Lanes([JW[0], 0.0, JW[1], 0.0, JW[2], 0.0])) * JT);
                        KB = JN;
                        KC = JY;
                        KD = JM;
                        KE = JZ;
                    } else {
                        let MC;
                        let MD;
                        let ME;
                        let MF;
                        if KA != 0.0 {
                            let KF = FL * GE;
                            let KG = (GC + GJ) + (KF * GC);
                            let KH = FG * KG;
                            let KI = FI * KG;
                            let KJ = Lanes([KI[0], 0.0, KI[1], KI[2], 0.0, 0.0]) + (((GD + GK) + (((Lanes([(FM * GE), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GG * FL)) * GC) + (GD * KF))) * FG);
                            let KK = I - GA;
                            let KL = Lanes([0.0, 0.0, L[0], 0.0, L[1]]) - GB;
                            let KM = KK * KK;
                            let KN = KL * KK;
                            let KO = KN + KN;
                            let KP = FL * KK;
                            let KQ = (KK + (GI * KM)) + (KP * KM);
                            let KR = FG * KQ;
                            let KS = FI * KQ;
                            let KT = Lanes([KS[0], 0.0, KS[1], KS[2], 0.0]) + (((KL + (KO * GI)) + (((Lanes([(FM * KK), 0.0, 0.0, 0.0, 0.0]) + (KL * FL)) * KM) + (KO * KP))) * FG);
                            let KU = rspice_limexp(KH);
                            let KV = rspice_limexp((-KH));
                            let KW = (GT * (KU - KV)).tanh();
                            let KX = (((KJ * KU) - ((KJ * N) * KV)) * GT) * (AD - (KW * KW));
                            let KY = AW + KW;
                            let KZ = rspice_limexp(KR);
                            let LA = rspice_limexp((-KR));
                            let LB = (GT * (KZ - LA)).tanh();
                            let LC = (((KT * KZ) - ((KT * N) * LA)) * GT) * (AD - (LB * LB));
                            let LD = AW + LB;
                            let LE = GX + (FO * KY);
                            let LF = GX + (FO * LD);
                            let LG = Q * LE;
                            let LH = (LE * P).tanh();
                            let LI = Q * LF;
                            let LJ = (LF * P).tanh();
                            let LK = HF + (IE * LD);
                            let LL = HF + (IE * KY);
                            let LM = CY * KY;
                            let LN = AW + LH;
                            let LO = LM * LN;
                            let LP = Q * LL;
                            let LQ = rspice_limexp((IK * (P - CX)));
                            let LR = Lanes([(DM * LQ), 0.0, 0.0]) + ((((Lanes([0.0, Q[0], Q[1]]) - Lanes([DK, 0.0, 0.0])) * IK) * LQ) * CZ);
                            let LS = (AW + (LL * P)) + (CZ * LQ);
                            let LT = CY * LD;
                            let LU = AW - LJ;
                            let LV = LT * LU;
                            let LW = Q * LK;
                            let LX = AW - (LK * P);
                            let LY = ((((Lanes([(DL * LD), 0.0, 0.0, 0.0, 0.0]) + (LC * CY)) * LU) + ((((((LC * FO) * P) + Lanes([0.0, 0.0, LI[0], LI[1], 0.0])) * (AD - (LJ * LJ))) * N) * LT)) * LX) + (((((LC * IE) * P) + Lanes([0.0, 0.0, LW[0], LW[1], 0.0])) * N) * LV);
                            let LZ = GT * ((LO * LS) - (LV * LX));
                            let MA = ((((((Lanes([(DL * KY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (KX * CY)) * LN) + (((((KX * FO) * P) + Lanes([0.0, 0.0, LG[0], LG[1], 0.0, 0.0])) * (AD - (LH * LH))) * LM)) * LS) + (((((KX * IE) * P) + Lanes([0.0, 0.0, LP[0], LP[1], 0.0, 0.0])) + Lanes([LR[0], 0.0, LR[1], LR[2], 0.0, 0.0])) * LO)) - Lanes([LY[0], LY[1], LY[2], LY[3], LY[4], 0.0])) * GT;
                            MC = KY;
                            MD = LZ;
                            ME = KX;
                            MF = MA;
                        } else {
                            let NA;
                            let NB;
                            if MB != 0.0 {
                                let MG = HF + (IE * GQ);
                                let MH = GV * FO;
                                let MI = GX + (FO * GW);
                                let MJ = Q * MI;
                                let MK = (MI * P).tanh();
                                let ML = Y * MI;
                                let MM = (MI * W).tanh();
                                let MN = CY * GQ;
                                let MP = MK + (MO * MM);
                                let MQ = MN * MP;
                                let MR = Y * MO;
                                let MS = P + (MO * W);
                                let MT = (Lanes([0.0, Q[0], Q[1]]) + Lanes([MR[0], 0.0, MR[1]])) * MG;
                                let MU = rspice_limexp((IK * (P - CX)));
                                let MV = Lanes([(DM * MU), 0.0, 0.0]) + ((((Lanes([0.0, Q[0], Q[1]]) - Lanes([DK, 0.0, 0.0])) * IK) * MU) * CZ);
                                let MW = (AW + (MG * MS)) + (CZ * MU);
                                let MX = MQ * MW;
                                let MY = ((((Lanes([(DL * GQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GP * CY)) * MP) + (((((MH * P) + Lanes([0.0, 0.0, MJ[0], MJ[1], 0.0, 0.0])) * (AD - (MK * MK))) + ((((MH * W) + Lanes([0.0, ML[0], 0.0, ML[1], 0.0, 0.0])) * (AD - (MM * MM))) * MO)) * MN)) * MW) + (((((GP * IE) * MS) + Lanes([0.0, MT[0], MT[1], MT[2], 0.0, 0.0])) + Lanes([MV[0], 0.0, MV[1], MV[2], 0.0, 0.0])) * MQ);
                                NA = MX;
                                NB = MY;
                            } else {
                                NA = AR;
                                NB = MZ;
                            }
                            MC = GW;
                            MD = NA;
                            ME = GV;
                            MF = NB;
                        }
                        KB = MC;
                        KC = MD;
                        KD = ME;
                        KE = MF;
                    }
                    JA = KB;
                    JB = KC;
                    JC = KD;
                    JD = KE;
                }
                HO = JA;
                HP = JB;
                HQ = JC;
                HR = JD;
            }
            let NW;
            let NX;
            let NY;
            let NZ;
            let OA;
            let OB;
            if HS != 0.0 {
                let NC = AW + GQ;
                let ND = DA / NC;
                let NE = (Lanes([DN, 0.0, 0.0, 0.0, 0.0, 0.0]) - (GP * ND)) / NC;
                let NG = NF + ND;
                let NI = NH * GQ;
                let NJ = GP * NH;
                let NL = NK + NI;
                let NN = NM + NI;
                NW = NN;
                NX = NL;
                NY = NG;
                NZ = NJ;
                OA = NJ;
                OB = NE;
            } else {
                let NO = AW + HO;
                let NP = DA / NO;
                let NQ = (Lanes([DN, 0.0, 0.0, 0.0, 0.0, 0.0]) - (HQ * NP)) / NO;
                let NR = NF + NP;
                let NS = NH * HO;
                let NT = HQ * NH;
                let NU = NK + NS;
                let NV = NM + NS;
                NW = NV;
                NX = NU;
                NY = NR;
                NZ = NT;
                OA = NT;
                OB = NQ;
            }
            let OD = FD * OC;
            let OE = AW + (OC * FC);
            let OF = NW * OE;
            let OG = (NZ * OE) + Lanes([(OD * NW), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let OH = NX * OE;
            let OI = (OA * OE) + Lanes([(OD * NX), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let OL = OK * (AW + (OJ * FC));
            let OM = (FD * OJ) * OK;
            let PF;
            let PG;
            let PH;
            let PI;
            let PJ;
            let PK;
            let PL;
            let PM;
            let PN;
            let PO;
            if ON != 0.0 {
                let OP = (OO * DB).tanh();
                let OQ = rspice_limexp((EP * OP));
                let OR = ((EQ * OP) + (((DO * OO) * (AD - (OP * OP))) * EP)) * OQ;
                let OS = S - DB;
                let OT = Lanes([0.0, U[0], U[1]]) - Lanes([DO, 0.0, 0.0]);
                let OU = U * N;
                let OW = (-S) - OV;
                let OX = I - DB;
                let OY = Lanes([0.0, L[0], L[1]]) - Lanes([DO, 0.0, 0.0]);
                let PA = M - OZ;
                PF = OW;
                PG = OS;
                PH = OQ;
                PI = PA;
                PJ = OX;
                PK = OU;
                PL = OT;
                PM = OR;
                PN = O;
                PO = OY;
            } else {
                let PB = -EP;
                let PC = rspice_limexp((PB * DB));
                let PD = (((EQ * N) * DB) + (DO * PB)) * PC;
                let RM;
                let RN;
                let RO;
                let RP;
                if PE != 0.0 {
                    let RE = (S - DB).tanh();
                    let RF = (Lanes([0.0, U[0], U[1]]) - Lanes([DO, 0.0, 0.0])) * (AD - (RE * RE));
                    let RG = (I - DB).tanh();
                    let RH = (Lanes([0.0, L[0], L[1]]) - Lanes([DO, 0.0, 0.0])) * (AD - (RG * RG));
                    RM = RE;
                    RN = RG;
                    RO = RF;
                    RP = RH;
                } else {
                    let RI = S - DB;
                    let RJ = Lanes([0.0, U[0], U[1]]) - Lanes([DO, 0.0, 0.0]);
                    let RK = I - DB;
                    let RL = Lanes([0.0, L[0], L[1]]) - Lanes([DO, 0.0, 0.0]);
                    RM = RI;
                    RN = RK;
                    RO = RJ;
                    RP = RL;
                }
                let RQ = U * N;
                let RR = (-S) - OV;
                let RS = M - OZ;
                PF = RR;
                PG = RM;
                PH = PC;
                PI = RS;
                PJ = RN;
                PK = RQ;
                PL = RO;
                PM = PD;
                PN = O;
                PO = RP;
            }
            let PQ = rspice_limexp((PP * PF));
            let PR = rspice_limexp((EP * PG));
            let PT = ((PK * PP) * PQ) * PS;
            let PV = PU * ((PR - (PS * (PQ - staged[4]))) - PH);
            let PW = ((((Lanes([(EQ * PG), 0.0, 0.0]) + (PL * EP)) * PR) - Lanes([0.0, PT[0], PT[1]])) - Lanes([PM, 0.0, 0.0])) * PU;
            let PX = rspice_limexp((PP * PI));
            let PY = rspice_limexp((EP * PJ));
            let PZ = ((PN * PP) * PX) * PS;
            let QA = PU * ((PY - (PS * (PX - staged[6]))) - PH);
            let QB = ((((Lanes([(EQ * PJ), 0.0, 0.0]) + (PO * EP)) * PY) - Lanes([0.0, PZ[0], PZ[1]])) - Lanes([PM, 0.0, 0.0])) * PU;
            let QD = U * QC;
            let QE = Lanes([DP, 0.0, 0.0]) + Lanes([0.0, QD[0], QD[1]]);
            let QG = QF * P;
            let QH = Q * QF;
            let QI = (DC + (QC * S)) + QG;
            let QJ = Lanes([QE[0], 0.0, QE[1], QE[2]]) + Lanes([0.0, QH[0], QH[1], 0.0]);
            let QK = QI.tanh();
            let QL = QJ * (AD - (QK * QK));
            let QM = AW + QK;
            let QO = (parameters[32] + (QN * P)).tanh();
            let QP = (Q * QN) * (AD - (QO * QO));
            let QQ = AW + QO;
            let QS = (parameters[34] - (QR * P)).tanh();
            let QT = ((Q * QR) * N) * (AD - (QS * QS));
            let QU = (AW + QS) - QF;
            let QW = L * QV;
            let QX = Lanes([DQ, 0.0, 0.0]) + Lanes([0.0, QW[0], QW[1]]);
            let QY = (DD + (QV * I)) - QG;
            let QZ = Lanes([QX[0], QX[1], 0.0, QX[2]]) - Lanes([0.0, QH[0], QH[1], 0.0]);
            let RA = QY.tanh();
            let RB = QZ * (AD - (RA * RA));
            let RC = AW + RA;
            let RY;
            let RZ;
            let SA;
            let SB;
            let SC;
            let SD;
            let SE;
            let SF;
            if RD != 0.0 {
                RY = AR;
                RZ = AR;
                SA = RT;
                SB = RU;
                SC = RV;
                SD = RW;
                SE = RV;
                SF = RW;
            } else {
                let TF;
                let TG;
                let TH;
                let TI;
                let TJ;
                let TK;
                let TL;
                let TM;
                if RX != 0.0 {
                    let SW = DE * QM;
                    let SX = QP * SW;
                    let SY = ((Lanes([(DR * QM), 0.0, 0.0, 0.0]) + (QL * DE)) * QQ) + Lanes([0.0, SX[0], SX[1], 0.0]);
                    let SZ = RU + (SW * QQ);
                    let TA = QT * RC;
                    let TB = (QU * RC) + staged[7];
                    let TC = Lanes([(DS * TB), 0.0, 0.0, 0.0]) + ((Lanes([0.0, TA[0], TA[1], 0.0]) + (RB * QU)) * DF);
                    let TD = RT + (DF * TB);
                    TF = AR;
                    TG = AR;
                    TH = TD;
                    TI = SZ;
                    TJ = RV;
                    TK = RW;
                    TL = TC;
                    TM = SY;
                } else {
                    let UR;
                    let US;
                    let UT;
                    let UU;
                    let UV;
                    let UW;
                    let UX;
                    let UY;
                    if TE != 0.0 {
                        let TN = QQ - QF;
                        let TO = DC + QG;
                        let TP = Lanes([0.0, QH[0], QH[1]]);
                        let TQ = Lanes([DP, 0.0, 0.0]) + TP;
                        let TR = TO.cosh();
                        let TS = QI.cosh();
                        let TT = TQ + ((TQ * (TO.sinh())) * (AD / TR));
                        let TU = (QI + (TS.ln())) - (TO + (TR.ln()));
                        let TV = QP * TU;
                        let TX = U * TW;
                        let TY = ((TU * TN) / QC) + (TW * S);
                        let TZ = U * RU;
                        let UA = (DE * TY) + (RU * S);
                        let UB = (Lanes([(DR * TY), 0.0, 0.0, 0.0]) + (((((((QJ + ((QJ * (QI.sinh())) * (AD / TS))) - Lanes([TT[0], TT[1], TT[2], 0.0])) * TN) + Lanes([0.0, TV[0], TV[1], 0.0])) / QC) + Lanes([0.0, 0.0, TX[0], TX[1]])) * DE)) + Lanes([0.0, 0.0, TZ[0], TZ[1]]);
                        let UC = DD - QG;
                        let UD = Lanes([DQ, 0.0, 0.0]) - TP;
                        let UE = UC.cosh();
                        let UF = QY.cosh();
                        let UG = UD + ((UD * (UC.sinh())) * (AD / UE));
                        let UH = (QY + (UF.ln())) - (UC + (UE.ln()));
                        let UI = QT * UH;
                        let UJ = L * TW;
                        let UK = ((UH * QU) / QV) + (TW * I);
                        let UL = L * RT;
                        let UM = (DF * UK) + (RT * I);
                        let UN = (Lanes([(DS * UK), 0.0, 0.0, 0.0]) + (((((((QZ + ((QZ * (QY.sinh())) * (AD / UF))) - Lanes([UG[0], UG[1], UG[2], 0.0])) * QU) + Lanes([0.0, UI[0], UI[1], 0.0])) / QV) + Lanes([0.0, UJ[0], 0.0, UJ[1]])) * DF)) + Lanes([0.0, UL[0], 0.0, UL[1]]);
                        let UO = UB[3];
                        let UP = UN[3];
                        UR = UM;
                        US = UA;
                        UT = UP;
                        UU = UO;
                        UV = UN;
                        UW = UB;
                        UX = RV;
                        UY = RW;
                    } else {
                        let WE;
                        let WF;
                        let WG;
                        let WH;
                        let WI;
                        let WJ;
                        let WK;
                        let WL;
                        if UQ != 0.0 {
                            let VA = (S / UZ) - AW;
                            let VB = VA * VA;
                            let VC = (U / UZ) * VA;
                            let VD = VC + VC;
                            let VF = VE + VB;
                            let VH = VF.powf(VG);
                            let VJ = VE + (VI * VB);
                            let VK = (Lanes([0.0, U[0], U[1]]) + Lanes([QH[0], QH[1], 0.0])) * QC;
                            let VL = (DC + (QC * (S + QG))).tanh();
                            let VN = VM + QS;
                            let VO = Q * VM;
                            let VP = (Lanes([L[0], 0.0, L[1]]) + Lanes([VO[0], VO[1], 0.0])) * QV;
                            let VQ = (DD + (QV * (I + (P * VM)))).tanh();
                            let VR = AW + VQ;
                            let VT = (((VD * (VG * (VF.powf(-2.5e0f64)))) * VJ) + ((VD * VI) * VH)) * VS;
                            let VU = (AW + VL) + (VS * (VH * VJ));
                            let VV = DE * VU;
                            let VW = QP * VV;
                            let VX = ((Lanes([(DR * VU), 0.0, 0.0, 0.0]) + ((((Lanes([DP, 0.0, 0.0, 0.0]) + Lanes([0.0, VK[0], VK[1], VK[2]])) * (AD - (VL * VL))) + Lanes([0.0, 0.0, VT[0], VT[1]])) * DE)) * QQ) + Lanes([0.0, VW[0], VW[1], 0.0]);
                            let VY = (VV * QQ) + RU;
                            let VZ = QT * VR;
                            let WA = (VN * VR) + staged[10];
                            let WB = Lanes([(DS * WA), 0.0, 0.0, 0.0]) + ((Lanes([0.0, VZ[0], VZ[1], 0.0]) + (((Lanes([DQ, 0.0, 0.0, 0.0]) + Lanes([0.0, VP[0], VP[1], VP[2]])) * (AD - (VQ * VQ))) * VN)) * DF);
                            let WC = (DF * WA) + RT;
                            WE = AR;
                            WF = AR;
                            WG = WC;
                            WH = VY;
                            WI = RV;
                            WJ = RW;
                            WK = WB;
                            WL = VX;
                        } else {
                            let XX;
                            let XY;
                            let XZ;
                            let YA;
                            let YB;
                            let YC;
                            if WD != 0.0 {
                                let WM = DC + QG;
                                let WN = Lanes([0.0, QH[0], QH[1]]);
                                let WO = Lanes([DP, 0.0, 0.0]) + WN;
                                let WP = WM.cosh();
                                let WQ = QI.cosh();
                                let WR = VS * (UZ + S);
                                let WS = -1e0f64 + (S / UZ);
                                let WT = VE + (WS * WS);
                                let WV = WT.powf(WU);
                                let WW = ((U * VS) * WV) + ((((U / UZ) * (2e0f64 * WS)) * (WU * (WT.powf(-1.5e0f64)))) * WR);
                                let WX = WO + ((WO * (WM.sinh())) * (AD / WP));
                                let WY = (((QI + (WQ.ln())) - (WM + (WP.ln()))) + (WR * WV)) - staged[11];
                                let XA = WZ + QO;
                                let XB = QP * WY;
                                let XD = U * XC;
                                let XE = ((WY * XA) / QC) + (XC * S);
                                let XF = U * RU;
                                let XG = (DE * XE) + (RU * S);
                                let XH = (Lanes([(DR * XE), 0.0, 0.0, 0.0]) + ((((((((QJ + ((QJ * (QI.sinh())) * (AD / WQ))) - Lanes([WX[0], WX[1], WX[2], 0.0])) + Lanes([0.0, 0.0, WW[0], WW[1]])) * XA) + Lanes([0.0, XB[0], XB[1], 0.0])) / QC) + Lanes([0.0, 0.0, XD[0], XD[1]])) * DE)) + Lanes([0.0, 0.0, XF[0], XF[1]]);
                                let XI = DD - QG;
                                let XJ = Lanes([DQ, 0.0, 0.0]) - WN;
                                let XK = XI.cosh();
                                let XL = QY.cosh();
                                let XM = XJ + ((XJ * (XI.sinh())) * (AD / XK));
                                let XN = (QY + (XL.ln())) - (XI + (XK.ln()));
                                let XO = WZ + QS;
                                let XP = QT * XN;
                                let XQ = L * XC;
                                let XR = ((XN * XO) / QV) + (XC * I);
                                let XS = L * RT;
                                let XT = (DF * XR) + (RT * I);
                                let XU = (Lanes([(DS * XR), 0.0, 0.0, 0.0]) + (((((((QZ + ((QZ * (QY.sinh())) * (AD / XL))) - Lanes([XM[0], XM[1], XM[2], 0.0])) * XO) + Lanes([0.0, XP[0], XP[1], 0.0])) / QV) + Lanes([0.0, XQ[0], 0.0, XQ[1]])) * DF)) + Lanes([0.0, XS[0], 0.0, XS[1]]);
                                let XV = XH[3];
                                let XW = XU[3];
                                XX = XT;
                                XY = XG;
                                XZ = XW;
                                YA = XV;
                                YB = XU;
                                YC = XH;
                            } else {
                                XX = AR;
                                XY = AR;
                                XZ = AR;
                                YA = AR;
                                YB = RV;
                                YC = RW;
                            }
                            WE = XX;
                            WF = XY;
                            WG = XZ;
                            WH = YA;
                            WI = YB;
                            WJ = YC;
                            WK = RV;
                            WL = RW;
                        }
                        UR = WE;
                        US = WF;
                        UT = WG;
                        UU = WH;
                        UV = WI;
                        UW = WJ;
                        UX = WK;
                        UY = WL;
                    }
                    TF = UR;
                    TG = US;
                    TH = UT;
                    TI = UU;
                    TJ = UV;
                    TK = UW;
                    TL = UX;
                    TM = UY;
                }
                RY = TF;
                RZ = TG;
                SA = TH;
                SB = TI;
                SC = TJ;
                SD = TK;
                SE = TL;
                SF = TM;
            }
            let SG = -HP;
            let SH = HR * N;
            let SJ = SI * node_potentials[15];
            let SL = SK * SI;
            let SM = ddt(4327, SJ);
            let SO = SL * SN;
            let SQ = SP * branch_unknown_flows[0];
            let SS = SR * SP;
            let ST = ddt(4334, SQ);
            let SU = SS * SN;
            let YR;
            let YS;
            let YT;
            let YU;
            let YV;
            let YW;
            let YX;
            let YY;
            let YZ;
            let ZA;
            let ZB;
            let ZC;
            let ZD;
            let ZE;
            let ZF;
            let ZG;
            if SV != 0.0 {
                let YD = ddt(4346, RY);
                let YE = SC * SN;
                let YF = ddt(4348, RZ);
                let YG = SD * SN;
                YR = YD;
                YS = YF;
                YT = AR;
                YU = AR;
                YV = RY;
                YW = RZ;
                YX = AR;
                YY = AR;
                YZ = YE;
                ZA = YG;
                ZB = RV;
                ZC = RW;
                ZD = SC;
                ZE = SD;
                ZF = RV;
                ZG = RW;
            } else {
                let YH = SA * I;
                let YI = L * SA;
                let YJ = (SE * I) + Lanes([0.0, YI[0], 0.0, YI[1]]);
                let YK = ddt(4352, YH);
                let YL = YJ * SN;
                let YM = SB * S;
                let YN = U * SB;
                let YO = (SF * S) + Lanes([0.0, 0.0, YN[0], YN[1]]);
                let YP = ddt(4356, YM);
                let YQ = YO * SN;
                YR = AR;
                YS = AR;
                YT = YK;
                YU = YP;
                YV = AR;
                YW = AR;
                YX = YH;
                YY = YM;
                YZ = RV;
                ZA = RW;
                ZB = YL;
                ZC = YQ;
                ZD = RV;
                ZE = RW;
                ZF = YJ;
                ZG = YO;
            }
            let ZJ = ZI * (node_potentials[7] - H);
            let ZK = (Lanes([0.0, ZH]) - Lanes([K, 0.0])) * ZI;
            let ZL = ddt(4360, ZJ);
            let ZM = ZK * SN;
            let ZO = ZN * P;
            let ZP = Q * ZN;
            let ZQ = ddt(4364, ZO);
            let ZR = ZP * SN;
            let ZS = node_potentials[6] - V;
            let ZU = Lanes([0.0, ZT]) - Lanes([X, 0.0]);
            let ZV = DG * ZS;
            let ZW = ZU * DG;
            let ZX = Lanes([(DT * ZS), 0.0, 0.0]) + Lanes([0.0, ZW[0], ZW[1]]);
            let ZY = ddt(4368, ZV);
            let ZZ = ZX * SN;
            let AAA = EX * ZS;
            let AAB = ZU * EX;
            let AAE = AAC * AAD;
            let AAG = AAF * AAC;
            let AAH = ddt(4375, AAE);
            let AAI = AAG * SN;
            let AAP;
            let AAQ;
            let AAR;
            let AAS;
            if AAJ != 0.0 {
                let AAK = OB * AAD;
                let AAL = (AAD * NY) + AAH;
                let AAM = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (AAF * NY)]) + Lanes([AAK[0], AAK[1], AAK[2], AAK[3], AAK[4], AAK[5], 0.0])) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, AAI]);
                AAP = AAL;
                AAQ = AAE;
                AAR = AAM;
                AAS = AAG;
            } else {
                AAP = AR;
                AAQ = AR;
                AAR = AAN;
                AAS = AAO;
            }
            let ABE;
            let ABF;
            let ABG;
            let ABH;
            let ABI;
            let ABJ;
            if AAT != 0.0 {
                let AAU = Lanes([T, 0.0]) - Lanes([0.0, D]);
                let AAV = (R - A) / DH;
                let AAW = (Lanes([0.0, AAU[0], AAU[1]]) - Lanes([(DU * AAV), 0.0, 0.0])) / DH;
                let AAX = DI * C;
                let AAY = F * DI;
                let AAZ = Lanes([(DV * C), 0.0, 0.0]) + Lanes([0.0, AAY[0], AAY[1]]);
                let ABA = ddt(4398, AAX);
                let ABB = AAZ * SN;
                ABE = AAV;
                ABF = ABA;
                ABG = AAX;
                ABH = AAW;
                ABI = ABB;
                ABJ = AAZ;
            } else {
                ABE = AR;
                ABF = AR;
                ABG = AR;
                ABH = ABC;
                ABI = ABD;
                ABJ = ABD;
            }
            let ABN = ABM * (R - ABK);
            let ABO = (Lanes([T, 0.0]) - Lanes([0.0, ABL])) * ABM;
            let ABP = ddt(4403, ABN);
            let ABQ = ABO * SN;
            let ABW;
            let ABX;
            if ABR != 0.0 {
                let ABT = (ABK - B) / ABS;
                let ABU = (Lanes([0.0, ABL]) - Lanes([E, 0.0])) / ABS;
                ABW = ABT;
                ABX = ABU;
            } else {
                ABW = AR;
                ABX = ABV;
            }
            let ACF;
            let ACG;
            if ABY != 0.0 {
                let ACC = (ABZ - G) / ACB;
                let ACD = (Lanes([0.0, ACA]) - Lanes([J, 0.0])) / ACB;
                ACF = ACC;
                ACG = ACD;
            } else {
                ACF = AR;
                ACG = ACE;
            }
            let ACN;
            let ACO;
            if ACI != 0.0 {
                let ACK = (ABZ - R) / ACJ;
                let ACL = (Lanes([0.0, ACA]) - Lanes([T, 0.0])) / ACJ;
                ACN = ACK;
                ACO = ACL;
            } else {
                ACN = AR;
                ACO = ACM;
            }
            let ACV;
            let ACW;
            if ACP != 0.0 {
                let ACR = branch_unknown_flows[7] * ACQ;
                let ACT = ACS * ACQ;
                ACV = ACR;
                ACW = ACT;
            } else {
                ACV = AR;
                ACW = ACU;
            }
            let ACY = ACX * branch_unknown_flows[10];
            let ADA = ACZ * ACX;
            let ADB = ddt(4455, ACY);
            let ADC = ADA * SN;
            let ADK;
            let ADL;
            if ADD != 0.0 {
                let ADF = ADE * OF;
                let ADH = OG * ADE;
                let ADI = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (ADG * OF)]) + Lanes([ADH[0], ADH[1], ADH[2], ADH[3], ADH[4], ADH[5], 0.0]);
                ADK = ADF;
                ADL = ADI;
            } else {
                ADK = AR;
                ADL = ADJ;
            }
            let ADN = ADM * branch_unknown_flows[14];
            let ADP = ADO * ADM;
            let ADQ = ddt(4476, ADN);
            let ADR = ADP * SN;
            let ADZ;
            let AEA;
            if ADS != 0.0 {
                let ADU = ADT * OH;
                let ADW = OI * ADT;
                let ADX = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (ADV * OH)]) + Lanes([ADW[0], ADW[1], ADW[2], ADW[3], ADW[4], ADW[5], 0.0]);
                ADZ = ADU;
                AEA = ADX;
            } else {
                ADZ = AR;
                AEA = ADY;
            }
            let AEC = AEB * branch_unknown_flows[18];
            let AEE = AED * AEB;
            let AEF = ddt(4501, AEC);
            let AEG = AEE * SN;
            let AEI = (A - node_potentials[2]) * EX;
            let AEJ = (Lanes([0.0, D]) - Lanes([AEH, 0.0])) * EX;
            let AEK = HR[5];
            let AEL = AEK / (AW + (AEK * NM));
            let AEO;
            let AEP;
            let AEQ;
            let AER;
            let AES;
            let AET;
            let AEU;
            let AEV;
            let AEW;
            let AEX;
            let AEY;
            let AEZ;
            if AEM != 0.0 {
                AEO = AR;
                AEP = AR;
                AEQ = AR;
                AER = AR;
                AES = AR;
                AET = AR;
                AEU = AFA;
                AEV = AFB;
                AEW = AFA;
                AEX = AFC;
                AEY = AFD;
                AEZ = AFD;
            } else {
                let AFE;
                let AFF;
                let AFG;
                let AFH;
                let AFI;
                let AFJ;
                let AFK;
                let AFL;
                let AFM;
                let AFN;
                let AFO;
                let AFP;
                if AEN != 0.0 {
                    let AFR;
                    let AFS;
                    let AFT;
                    let AFU;
                    let AFV;
                    let AFW;
                    let AFX;
                    let AFY;
                    let AFZ;
                    let AGA;
                    let AGB;
                    let AGC;
                    if ACH != 0.0 {
                        let AFQ = if AEL > AR { 1.0 } else { 0.0 };
                        let AGF = (AGD * AJ) * AGE;
                        let AGH = (AGF * DE) * AGG;
                        let AGI = ((((AK * AGD) * AGE) * DE) + (DR * AGF)) * AGG;
                        let AGJ = AGI * AGH;
                        let AGK = (AW - (AGH * AGH)).sqrt();
                        let AGM = (-AGH) * AGL;
                        let AGP = Lanes([(((AGI * N) * AGL) * AGN), 0.0]) + Lanes([0.0, (AGO * AGM)]);
                        let AGS = Lanes([((((AGJ + AGJ) * N) * (AD / (AC * AGK))) * AGQ), 0.0]) + Lanes([0.0, (AGR * AGK)]);
                        let AGT = (AGM * AGN) + (AGK * AGQ);
                        let AGU = Lanes([AGP[0], AGP[1], 0.0]) + Lanes([AGS[0], 0.0, AGS[1]]);
                        let AGV = -(AGH * AGL);
                        let AGW = AGV * AGN;
                        let AGX = Lanes([(((AGI * AGL) * N) * AGN), 0.0]) + Lanes([0.0, (AGO * AGV)]);
                        let AGY = ddt(4654, AGW);
                        let AGZ = AGX * SN;
                        AFR = AGN;
                        AFS = AGQ;
                        AFT = AGN;
                        AFU = AGT;
                        AFV = AGY;
                        AFW = AGW;
                        AFX = AGO;
                        AFY = AGR;
                        AFZ = AGO;
                        AGA = AGU;
                        AGB = AGZ;
                        AGC = AGX;
                    } else {
                        AFR = AR;
                        AFS = AR;
                        AFT = AR;
                        AFU = AR;
                        AFV = AR;
                        AFW = AR;
                        AFX = AFA;
                        AFY = AFB;
                        AFZ = AFA;
                        AGA = AFC;
                        AGB = AFD;
                        AGC = AFD;
                    }
                    AFE = AFR;
                    AFF = AFS;
                    AFG = AFT;
                    AFH = AFU;
                    AFI = AFV;
                    AFJ = AFW;
                    AFK = AFX;
                    AFL = AFY;
                    AFM = AFZ;
                    AFN = AGA;
                    AFO = AGB;
                    AFP = AGC;
                } else {
                    AFE = AR;
                    AFF = AR;
                    AFG = AR;
                    AFH = AR;
                    AFI = AR;
                    AFJ = AR;
                    AFK = AFA;
                    AFL = AFB;
                    AFM = AFA;
                    AFN = AFC;
                    AFO = AFD;
                    AFP = AFD;
                }
                AEO = AFE;
                AEP = AFF;
                AEQ = AFG;
                AER = AFH;
                AES = AFI;
                AET = AFJ;
                AEU = AFK;
                AEV = AFL;
                AEW = AFM;
                AEX = AFN;
                AEY = AFO;
                AEZ = AFP;
            }
            let AHU;
            let AHV;
            let AHW;
            let AHX;
            let AHY;
            let AHZ;
            let AIA;
            let AIB;
            let AIC;
            let AID;
            if AHA != 0.0 {
                let AHB = HP * P;
                let AHC = Q * HP;
                let AHD = ((HR * P) + Lanes([0.0, 0.0, AHC[0], AHC[1], 0.0, 0.0])) * ((AC * (if AHB >= AB { 1.0 } else { 0.0 })) - AD);
                let AHE = PV * S;
                let AHF = U * PV;
                let AHG = ((PW * S) + Lanes([0.0, AHF[0], AHF[1]])) * ((AC * (if AHE >= AB { 1.0 } else { 0.0 })) - AD);
                let AHI = AHH * ((AHB.abs()) + (AHE.abs()));
                let AHJ = (Lanes([AHD[0], AHD[1], AHD[2], AHD[3], AHD[4], 0.0, AHD[5]]) + Lanes([AHG[0], 0.0, 0.0, AHG[1], 0.0, AHG[2], 0.0])) * AHH;
                let AHK = AA / OL;
                let AHL = (AE - (OM * AHK)) / OL;
                let AHN = AHM * AA;
                let AHO = AE * AHM;
                let AHP = ddt(4752, AHN);
                let AHQ = AHO * SN;
                AHU = AHI;
                AHV = AHK;
                AHW = AHP;
                AHX = AR;
                AHY = AHN;
                AHZ = AHJ;
                AIA = AHL;
                AIB = AHQ;
                AIC = AI;
                AID = AHO;
            } else {
                let AHR = AA * EX;
                let AHS = AE * EX;
                AHU = AR;
                AHV = AR;
                AHW = AR;
                AHX = AHR;
                AHY = AR;
                AHZ = AHT;
                AIA = AI;
                AIB = AI;
                AIC = AHS;
                AID = AI;
            }
            let AIE = SH[0];
            let AIF = SH[1];
            let AIG = SH[2];
            let AIH = SH[3];
            let AII = SH[4];
            let AIJ = SH[5];
            let AIK = SO;
            let AIM = AIL;
            let AIN = SU;
            let AIO = PW[0];
            let AIP = PW[1];
            let AIQ = PW[2];
            let AIR = QB[0];
            let AIS = QB[1];
            let AIT = QB[2];
            let AIU = YZ[0];
            let AIV = YZ[1];
            let AIW = YZ[2];
            let AIX = YZ[3];
            let AIY = ZA[0];
            let AIZ = ZA[1];
            let AJA = ZA[2];
            let AJB = ZA[3];
            let AJC = ZB[0];
            let AJD = ZB[1];
            let AJE = ZB[2];
            let AJF = ZB[3];
            let AJG = ZC[0];
            let AJH = ZC[1];
            let AJI = ZC[2];
            let AJJ = ZC[3];
            let AJK = ZM[0];
            let AJL = ZM[1];
            let AJM = ZR[0];
            let AJN = ZR[1];
            let AJO = ZZ[0];
            let AJP = ZZ[1];
            let AJQ = ZZ[2];
            let AJR = AAB[0];
            let AJS = AAB[1];
            let AJT = AAR[0];
            let AJU = AAR[1];
            let AJV = AAR[2];
            let AJW = AAR[3];
            let AJX = AAR[4];
            let AJY = AAR[5];
            let AJZ = AAR[6];
            let AKA = ABH[0];
            let AKB = ABH[1];
            let AKC = ABH[2];
            let AKD = ABI[0];
            let AKE = ABI[1];
            let AKF = ABI[2];
            let AKG = ABQ[0];
            let AKH = ABQ[1];
            let AKI = ABX[0];
            let AKJ = ABX[1];
            let AKK = ACG[0];
            let AKL = ACG[1];
            let AKM = ACO[0];
            let AKN = ACO[1];
            let AKO = ACW;
            let AKP = ADC;
            let AKQ = ADL[0];
            let AKR = ADL[1];
            let AKS = ADL[2];
            let AKT = ADL[3];
            let AKU = ADL[4];
            let AKV = ADL[5];
            let AKW = ADL[6];
            let AKX = ADR;
            let AKY = AEA[0];
            let AKZ = AEA[1];
            let ALA = AEA[2];
            let ALB = AEA[3];
            let ALC = AEA[4];
            let ALD = AEA[5];
            let ALE = AEA[6];
            let ALF = AEG;
            let ALG = AEJ[0];
            let ALH = AEJ[1];
            let ALI = AEU;
            let ALJ = AEV;
            let ALK = AEW;
            let ALL = AEX[0];
            let ALM = AEX[1];
            let ALN = AEX[2];
            let ALO = AEY[0];
            let ALP = AEY[1];
            let ALQ = AGO;
            let ALR = AGR;
            let ALS = AHZ[0];
            let ALT = AHZ[1];
            let ALU = AHZ[2];
            let ALV = AHZ[3];
            let ALW = AHZ[4];
            let ALX = AHZ[5];
            let ALY = AHZ[6];
            let ALZ = AIA;
            let AMA = AIB;
            let AMB = AIC;
            let AMC = SL;
            let AMD = SS;
            let AME = ZD[0];
            let AMF = ZD[1];
            let AMG = ZD[2];
            let AMH = ZD[3];
            let AMI = ZE[0];
            let AMJ = ZE[1];
            let AMK = ZE[2];
            let AML = ZE[3];
            let AMM = ZF[0];
            let AMN = ZF[1];
            let AMO = ZF[2];
            let AMP = ZF[3];
            let AMQ = ZG[0];
            let AMR = ZG[1];
            let AMS = ZG[2];
            let AMT = ZG[3];
            let AMU = ZK[0];
            let AMV = ZK[1];
            let AMW = ZP[0];
            let AMX = ZP[1];
            let AMY = ZX[0];
            let AMZ = ZX[1];
            let ANA = ZX[2];
            let ANB = AAS;
            let ANC = ABJ[0];
            let AND = ABJ[1];
            let ANE = ABJ[2];
            let ANF = ABO[0];
            let ANG = ABO[1];
            let ANH = ADA;
            let ANI = ADP;
            let ANJ = AEE;
            let ANK = AEZ[0];
            let ANL = AEZ[1];
            let ANM = AID;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(15),
            None,
            multiplicity * (SG),
            [3, 4, 5, 8, 10, 12],
            [AIE, AIF, AIG, AIH, AII, AIJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (SM),
            [15],
            [AIK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (ANN),
            [16],
            [AIM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(15), Some(16), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            0,
            ST,
            [],
            [],
            [0],
            [AIN],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            Some(8),
            multiplicity * (ANN),
            [16],
            [AIM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(8),
            multiplicity * (PV),
            [3, 8, 11],
            [AIO, AIP, AIQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(5),
            multiplicity * (QA),
            [3, 5, 10],
            [AIR, AIS, AIT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * (YR),
            [3, 5, 8, 10],
            [AIU, AIV, AIW, AIX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * (YS),
            [3, 5, 8, 11],
            [AIY, AIZ, AJA, AJB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * (YT),
            [3, 5, 8, 10],
            [AJC, AJD, AJE, AJF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * (YU),
            [3, 5, 8, 11],
            [AJG, AJH, AJI, AJJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(5),
            multiplicity * (ZL),
            [5, 7],
            [AJK, AJL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(8),
            multiplicity * (ZQ),
            [5, 8],
            [AJM, AJN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(4),
            multiplicity * (ZY),
            [3, 4, 6],
            [AJO, AJP, AJQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(4),
            multiplicity * (AAA),
            [4, 6],
            [AJR, AJS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(8), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<6, 1>(
            1,
            AAP,
            [3, 4, 5, 8, 10, 12],
            [AJT, AJU, AJV, AJW, AJX, AJY],
            [1],
            [AJZ],
        );
        stamper.stamp_potential_branch_local(Some(4), Some(8), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[46],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(12),
            multiplicity * (ABE),
            [3, 11, 12],
            [AKA, AKB, AKC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(8),
            multiplicity * (ABF),
            [3, 8, 12],
            [AKD, AKE, AKF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(11), Some(8), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[47],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(11),
            Some(14),
            multiplicity * (ABP),
            [11, 14],
            [AKG, AKH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(14),
            Some(8),
            multiplicity * (ABW),
            [8, 14],
            [AKI, AKJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(14), Some(8), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[48],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(13),
            Some(10),
            multiplicity * (ACF),
            [10, 13],
            [AKK, AKL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(13), Some(10), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[49],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(13),
            Some(10),
            multiplicity * (staged[50]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(13),
            Some(11),
            multiplicity * (ACN),
            [11, 13],
            [AKM, AKN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(13), Some(11), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[51],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(7), Some(13), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            7,
            ACV,
            [],
            [],
            [7],
            [AKO],
        );
        stamper.stamp_potential_branch_local(Some(7), Some(13), 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            8,
            staged[52],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(7), Some(13), 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            staged[53],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(7), 10, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            10,
            ADB,
            [],
            [],
            [10],
            [AKP],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 11, multiplicity);
        stamper.stamp_potential_sparse_local::<6, 1>(
            11,
            ADK,
            [3, 4, 5, 8, 10, 12],
            [AKQ, AKR, AKS, AKT, AKU, AKV],
            [11],
            [AKW],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            staged[54],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            13,
            staged[55],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(9), Some(2), 14, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            14,
            ADQ,
            [],
            [],
            [14],
            [AKX],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 15, multiplicity);
        stamper.stamp_potential_sparse_local::<6, 1>(
            15,
            ADZ,
            [3, 4, 5, 8, 10, 12],
            [AKY, AKZ, ALA, ALB, ALC, ALD],
            [15],
            [ALE],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 16, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            16,
            staged[56],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 17, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            17,
            staged[57],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(6), Some(0), 18, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            18,
            AEF,
            [],
            [],
            [18],
            [ALF],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(2),
            multiplicity * (ANO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            Some(2),
            multiplicity * (ANP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(12),
            Some(2),
            multiplicity * (AEI),
            [2, 12],
            [ALG, ALH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[58]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[59]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(17),
            None,
            multiplicity * (staged[60]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (AEO),
            [17],
            [ALI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(18),
            None,
            multiplicity * (staged[61]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (AEP),
            [18],
            [ALJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            Some(8),
            multiplicity * (AEQ),
            [17],
            [ALK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (AER),
            [3, 17, 18],
            [ALL, ALM, ALN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(5),
            multiplicity * (AES),
            [3, 17],
            [ALO, ALP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[62]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[63]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[64]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (staged[65]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (AGN),
            [17],
            [ALQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (AGQ),
            [18],
            [ALR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (staged[66]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(5),
            multiplicity * (staged[67]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (staged[68]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(5),
            multiplicity * (staged[69]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (AHU),
            [3, 4, 5, 8, 10, 11, 12],
            [ALS, ALT, ALU, ALV, ALW, ALX, ALY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (AHV),
            [3],
            [ALZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (AHW),
            [3],
            [AMA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (AHX),
            [3],
            [AMB],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = SG;
        self.canonical_reactive[1] = SJ;
        self.canonical_reactive[2] = AMC;
        self.canonical_reactive[3] = ANN;
        self.canonical_reactive[4] = SQ;
        self.canonical_reactive[5] = AMD;
        self.canonical_reactive[6] = ANN;
        self.canonical_reactive[7] = PV;
        self.canonical_reactive[8] = QA;
        self.canonical_reactive[9] = YV;
        self.canonical_reactive[10] = AME;
        self.canonical_reactive[11] = AMF;
        self.canonical_reactive[12] = AMG;
        self.canonical_reactive[13] = AMH;
        self.canonical_reactive[14] = YW;
        self.canonical_reactive[15] = AMI;
        self.canonical_reactive[16] = AMJ;
        self.canonical_reactive[17] = AMK;
        self.canonical_reactive[18] = AML;
        self.canonical_reactive[19] = YX;
        self.canonical_reactive[20] = AMM;
        self.canonical_reactive[21] = AMN;
        self.canonical_reactive[22] = AMO;
        self.canonical_reactive[23] = AMP;
        self.canonical_reactive[24] = YY;
        self.canonical_reactive[25] = AMQ;
        self.canonical_reactive[26] = AMR;
        self.canonical_reactive[27] = AMS;
        self.canonical_reactive[28] = AMT;
        self.canonical_reactive[29] = ZJ;
        self.canonical_reactive[30] = AMU;
        self.canonical_reactive[31] = AMV;
        self.canonical_reactive[32] = ZO;
        self.canonical_reactive[33] = AMW;
        self.canonical_reactive[34] = AMX;
        self.canonical_reactive[35] = ZV;
        self.canonical_reactive[36] = AMY;
        self.canonical_reactive[37] = AMZ;
        self.canonical_reactive[38] = ANA;
        self.canonical_reactive[39] = AAA;
        self.canonical_reactive[40] = AAQ;
        self.canonical_reactive[41] = ANB;
        self.canonical_reactive[42] = staged[46];
        self.canonical_reactive[43] = ABE;
        self.canonical_reactive[44] = ABG;
        self.canonical_reactive[45] = ANC;
        self.canonical_reactive[46] = AND;
        self.canonical_reactive[47] = ANE;
        self.canonical_reactive[48] = staged[47];
        self.canonical_reactive[49] = ABN;
        self.canonical_reactive[50] = ANF;
        self.canonical_reactive[51] = ANG;
        self.canonical_reactive[52] = ABW;
        self.canonical_reactive[53] = staged[48];
        self.canonical_reactive[54] = ACF;
        self.canonical_reactive[55] = staged[49];
        self.canonical_reactive[56] = staged[50];
        self.canonical_reactive[57] = ACN;
        self.canonical_reactive[58] = staged[51];
        self.canonical_reactive[59] = ACV;
        self.canonical_reactive[60] = staged[52];
        self.canonical_reactive[61] = staged[53];
        self.canonical_reactive[62] = ACY;
        self.canonical_reactive[63] = ANH;
        self.canonical_reactive[64] = ADK;
        self.canonical_reactive[65] = staged[54];
        self.canonical_reactive[66] = staged[55];
        self.canonical_reactive[67] = ADN;
        self.canonical_reactive[68] = ANI;
        self.canonical_reactive[69] = ADZ;
        self.canonical_reactive[70] = staged[56];
        self.canonical_reactive[71] = staged[57];
        self.canonical_reactive[72] = AEC;
        self.canonical_reactive[73] = ANJ;
        self.canonical_reactive[74] = ANO;
        self.canonical_reactive[75] = ANP;
        self.canonical_reactive[76] = AEI;
        self.canonical_reactive[77] = staged[58];
        self.canonical_reactive[78] = staged[59];
        self.canonical_reactive[79] = staged[60];
        self.canonical_reactive[80] = AEO;
        self.canonical_reactive[81] = staged[61];
        self.canonical_reactive[82] = AEP;
        self.canonical_reactive[83] = AEQ;
        self.canonical_reactive[84] = AER;
        self.canonical_reactive[85] = AET;
        self.canonical_reactive[86] = ANK;
        self.canonical_reactive[87] = ANL;
        self.canonical_reactive[88] = staged[62];
        self.canonical_reactive[89] = staged[63];
        self.canonical_reactive[90] = staged[64];
        self.canonical_reactive[91] = staged[65];
        self.canonical_reactive[92] = AGN;
        self.canonical_reactive[93] = AGQ;
        self.canonical_reactive[94] = staged[66];
        self.canonical_reactive[95] = staged[67];
        self.canonical_reactive[96] = staged[68];
        self.canonical_reactive[97] = staged[69];
        self.canonical_reactive[98] = AHU;
        self.canonical_reactive[99] = AHV;
        self.canonical_reactive[100] = AHY;
        self.canonical_reactive[101] = ANM;
        self.canonical_reactive[102] = AHX;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(15),
            None,
            &[15],
            &[cached[2]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            0,
            &[],
            &[],
            &[0],
            &[cached[5]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(5),
            &[3, 5, 8, 10],
            &[cached[10], cached[11], cached[12], cached[13]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(8),
            &[3, 5, 8, 11],
            &[cached[15], cached[16], cached[17], cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(5),
            &[3, 5, 8, 10],
            &[cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(8),
            &[3, 5, 8, 11],
            &[cached[25], cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[5, 7],
            &[cached[30], cached[31]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(8),
            &[5, 8],
            &[cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[3, 4, 6],
            &[cached[36], cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            1,
            &[],
            &[],
            &[1],
            &[cached[41]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            Some(8),
            &[3, 8, 12],
            &[cached[45], cached[46], cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(14),
            &[11, 14],
            &[cached[50], cached[51]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            10,
            &[],
            &[],
            &[10],
            &[cached[63]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            14,
            &[],
            &[],
            &[14],
            &[cached[68]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            18,
            &[],
            &[],
            &[18],
            &[cached[73]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 17],
            &[cached[86], cached[87]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[101]],
            &[],
            &[],
            multiplicity,
        );
    }

}
