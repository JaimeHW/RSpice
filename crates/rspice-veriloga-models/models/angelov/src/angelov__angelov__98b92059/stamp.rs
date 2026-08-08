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
        let mut key = Vec::with_capacity(164);
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
        self.canonical_staged[9] = values[2];
        self.canonical_staged[3] = values[3];
        self.canonical_staged[10] = values[4];
        self.canonical_staged[11] = values[5];
        self.canonical_staged[13] = values[6];
        self.canonical_staged[14] = values[7];
        self.canonical_staged[12] = values[8];
        self.canonical_staged[15] = values[9];
        self.canonical_staged[16] = values[10];
        self.canonical_staged[17] = values[11];
        self.canonical_staged[18] = values[12];
        self.canonical_staged[4] = values[13];
        self.canonical_staged[20] = values[14];
        self.canonical_staged[5] = values[15];
        self.canonical_staged[6] = values[16];
        self.canonical_staged[19] = values[17];
        self.canonical_staged[21] = values[18];
        self.canonical_staged[22] = values[19];
        self.canonical_staged[23] = values[20];
        self.canonical_staged[24] = values[21];
        self.canonical_staged[25] = values[22];
        self.canonical_staged[26] = values[23];
        self.canonical_staged[27] = values[24];
        self.canonical_staged[28] = values[25];
        self.canonical_staged[29] = values[26];
        self.canonical_staged[30] = values[27];
        self.canonical_staged[31] = values[28];
        self.canonical_staged[32] = values[29];
        self.canonical_staged[36] = values[30];
        self.canonical_staged[37] = values[31];
        self.canonical_staged[39] = values[32];
        self.canonical_staged[40] = values[33];
        self.canonical_staged[42] = values[34];
        self.canonical_staged[44] = values[35];
        self.canonical_staged[46] = values[36];
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
                let A = if parameter_given[85] { 1.0 } else { 0.0 };
                let C = 3.0015e2f64;
                let E = 0e0f64;
                let I = parameters[4];
                let K = 1e0f64;
                let N = 2e0f64;
                let P = 3e0f64;
                let R = parameters[5];
                let U = parameters[6];
                let Z = parameters[37];
                let AE = 0e0f64;
                let AH = 0e0f64;
                let AK = 0e0f64;
                let AN = 0e0f64;
                let AT = 0e0f64;
                let AV = 0e0f64;
                let BA = parameters[7];
                let BC = 0e0f64;
                let mut oH = 0.0;
                let mut oL = 0.0;
                let mut oO = 0.0;
                let mut oQ = 0.0;
                let mut oT = 0.0;
                let mut oW = 0.0;
                let mut oAA = 0.0;
                let mut oAB = 0.0;
                let mut oAC = 0.0;
                let mut oAQ = 0.0;
                let mut oAY = 0.0;
                let mut oBE = 0.0;
                let mut oBF = 0.0;
                let D = if A != 0.0 {
                    let B = parameters[85] + 2.7315e2f64;
                    B
                } else {
                    C
                };
                let F = if parameters[57] > E { 1.0 } else { 0.0 };
                let G = if (if (if parameter_given[39] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[40] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if G != 0.0 {
                    let H = 5e-1f64 / parameters[40];
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
                        let O = if I == N { 1.0 } else { 0.0 };
                        oO = O;
                        if O != 0.0 {
                        } else {
                            let Q = if I == P { 1.0 } else { 0.0 };
                            oQ = Q;
                        }
                    }
                }
                let M = if J != 0.0 || (if I == K { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let S = if R == E { 1.0 } else { 0.0 };
                if S != 0.0 {
                } else {
                    let T = if R == K { 1.0 } else { 0.0 };
                    oT = T;
                }
                let V = if U == E { 1.0 } else { 0.0 };
                if V != 0.0 {
                } else {
                    let W = if U == K { 1.0 } else { 0.0 };
                    oW = W;
                    if W != 0.0 {
                        let AA = N * Z;
                        oAA = AA;
                    } else {
                        let AB = if U == N { 1.0 } else { 0.0 };
                        oAB = AB;
                        if AB != 0.0 {
                            let AC = N * Z;
                            oAC = AC;
                        } else {
                        }
                    }
                }
                let X = parameters[51] / P;
                let Y = if U == N { 1.0 } else { 0.0 };
                let AD = if parameters[53] > E { 1.0 } else { 0.0 };
                let AF = if AD != 0.0 {
                    E
                } else {
                    AE
                };
                let AG = if parameters[55] > E { 1.0 } else { 0.0 };
                let AI = if AG != 0.0 {
                    E
                } else {
                    AH
                };
                let AJ = if parameters[47] > E { 1.0 } else { 0.0 };
                let AL = if AJ != 0.0 {
                    E
                } else {
                    AK
                };
                let AM = if parameters[45] > E { 1.0 } else { 0.0 };
                let AO = if AM != 0.0 {
                    E
                } else {
                    AN
                };
                let AP = if parameters[42] > E { 1.0 } else { 0.0 };
                let AR;
                if AP != 0.0 {
                    AR = E;
                } else {
                    let AQ = if parameters[50] > E { 1.0 } else { 0.0 };
                    oAQ = AQ;
                    let AU = if AQ != 0.0 {
                        E
                    } else {
                        AT
                    };
                    AR = AU;
                }
                let AS = if parameters[46] > E { 1.0 } else { 0.0 };
                let AW = if AS != 0.0 {
                    E
                } else {
                    AV
                };
                let AX = if (if parameters[43] > E { 1.0 } else { 0.0 }) != 0.0 || (if parameters[44] > E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AZ;
                if AX != 0.0 {
                    AZ = E;
                } else {
                    let AY = if parameters[48] > E { 1.0 } else { 0.0 };
                    oAY = AY;
                    let BD = if AY != 0.0 {
                        E
                    } else {
                        BC
                    };
                    AZ = BD;
                }
                let BB = if BA == E { 1.0 } else { 0.0 };
                if BB != 0.0 {
                } else {
                    let BE = if BA == K { 1.0 } else { 0.0 };
                    oBE = BE;
                    if BE != 0.0 {
                    } else {
                        let BF = if BA == N { 1.0 } else { 0.0 };
                        oBF = BF;
                    }
                }
            [D, F, G, oH, J, oL, oO, oQ, M, S, oT, V, oW, oAA, oAB, oAC, X, Y, AD, AG, AJ, AM, AP, oAQ, AS, AX, oAY, BB, oBE, oBF, AF, AI, AL, AO, AR, AW, AZ]
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
                let C = staged[23];
                let D = parameters[0];
                let E = 0e0f64;
                let G = 0e0f64;
                let I = staged[25];
                let K = staged[27];
                let L = 0e0f64;
                let O = staged[28];
                let P = 0e0f64;
                let S = staged[30];
                let T = 0e0f64;
                let V = staged[31];
                let AE = 0e0f64;
                let AF = 0e0f64;
                let AV = parameters[75];
                let AX = 0e0f64;
                let AZ = 0e0f64;
                let BA = 0e0f64;
                let BB = 0e0f64;
                let BC = 0e0f64;
                let BD = 0e0f64;
                let BK = 0e0f64;
                let BL = 0e0f64;
                let BO = 0e0f64;
                let BP = 0e0f64;
                let mut oB = 0.0;
                let mut oAU = 0.0;
                let mut oAW = 0.0;
                let mut oBE = 0.0;
                if A != 0.0 {
                    let B = parameters[3] + 2.7315e2f64;
                    oB = B;
                } else {
                }
                let F;
                if C != 0.0 {
                    let H = if D != 0.0 {
                        G
                    } else {
                        E
                    };
                    F = H;
                } else {
                    F = E;
                }
                let J;
                if I != 0.0 {
                    let M = if D != 0.0 {
                        L
                    } else {
                        E
                    };
                    J = M;
                } else {
                    J = E;
                }
                let N;
                if K != 0.0 {
                    let Q = if D != 0.0 {
                        P
                    } else {
                        E
                    };
                    N = Q;
                } else {
                    N = E;
                }
                let R;
                if O != 0.0 {
                    let U = if D != 0.0 {
                        T
                    } else {
                        E
                    };
                    R = U;
                } else {
                    R = E;
                }
                let W;
                let X;
                let Y;
                let Z;
                let AA;
                let AB;
                let AC;
                let AD;
                if S != 0.0 {
                    let AG;
                    let AH;
                    if D != 0.0 {
                        AG = AE;
                        AH = AF;
                    } else {
                        AG = E;
                        AH = E;
                    }
                    W = AG;
                    X = AH;
                    Y = E;
                    Z = E;
                    AA = E;
                    AB = E;
                    AC = E;
                    AD = E;
                } else {
                    let AI;
                    let AJ;
                    let AK;
                    let AL;
                    let AM;
                    let AN;
                    if V != 0.0 {
                        let AO;
                        let AP;
                        let AQ;
                        let AR;
                        let AS;
                        let AT;
                        if D != 0.0 {
                            let AU = (parameters[72] * parameters[71]).sqrt();
                            oAU = AU;
                            let AW = if AV > E { 1.0 } else { 0.0 };
                            oAW = AW;
                            let AY = if AW != 0.0 {
                                AX
                            } else {
                                E
                            };
                            AO = AZ;
                            AP = BA;
                            AQ = BB;
                            AR = BC;
                            AS = BD;
                            AT = AY;
                        } else {
                            AO = E;
                            AP = E;
                            AQ = E;
                            AR = E;
                            AS = E;
                            AT = E;
                        }
                        AI = AO;
                        AJ = AP;
                        AK = AQ;
                        AL = AR;
                        AM = AS;
                        AN = AT;
                    } else {
                        AI = E;
                        AJ = E;
                        AK = E;
                        AL = E;
                        AM = E;
                        AN = E;
                    }
                    W = E;
                    X = E;
                    Y = AI;
                    Z = AJ;
                    AA = AK;
                    AB = AL;
                    AC = AM;
                    AD = AN;
                }
                let BF;
                let BG;
                let BH;
                let BI;
                if D != 0.0 {
                    let BE = if AV > E { 1.0 } else { 0.0 };
                    oBE = BE;
                    let BM;
                    let BN;
                    if BE != 0.0 {
                        BM = BK;
                        BN = BL;
                    } else {
                        BM = E;
                        BN = E;
                    }
                    BF = BO;
                    BG = BP;
                    BH = BM;
                    BI = BN;
                } else {
                    BF = E;
                    BG = E;
                    BH = E;
                    BI = E;
                }
                let BJ = if parameters[1] != 0.0 && parameters[57] != 0.0 { 1.0 } else { 0.0 };
            [oB, oAU, oAW, oBE, BJ, F, J, N, R, W, X, Y, Z, AA, AB, AC, AD, BF, BG, BH, BI]
        };
        self.canonical_staged[8] = produced[0];
        self.canonical_staged[7] = produced[1];
        self.canonical_staged[33] = produced[2];
        self.canonical_staged[34] = produced[3];
        self.canonical_staged[35] = produced[4];
        self.canonical_staged[38] = produced[5];
        self.canonical_staged[41] = produced[6];
        self.canonical_staged[43] = produced[7];
        self.canonical_staged[45] = produced[8];
        self.canonical_staged[47] = produced[9];
        self.canonical_staged[48] = produced[10];
        self.canonical_staged[49] = produced[11];
        self.canonical_staged[50] = produced[12];
        self.canonical_staged[51] = produced[13];
        self.canonical_staged[52] = produced[14];
        self.canonical_staged[53] = produced[15];
        self.canonical_staged[54] = produced[16];
        self.canonical_staged[55] = produced[17];
        self.canonical_staged[56] = produced[18];
        self.canonical_staged[57] = produced[19];
        self.canonical_staged[58] = produced[20];
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
                let B = staged[8];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5]), ctx.branch_current(self.branches[6]), ctx.branch_current(self.branches[7]), ctx.branch_current(self.branches[8]), ctx.branch_current(self.branches[9]), ctx.branch_current(self.branches[10]), ctx.branch_current(self.branches[11]), ctx.branch_current(self.branches[12]), ctx.branch_current(self.branches[13]), ctx.branch_current(self.branches[14]), ctx.branch_current(self.branches[15]), ctx.branch_current(self.branches[16]), ctx.branch_current(self.branches[17]), ctx.branch_current(self.branches[18])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 3101 => 0usize, 3108 => 1usize, 3116 => 2usize, 3118 => 3usize, 3122 => 4usize, 3126 => 5usize, 3130 => 6usize, 3134 => 7usize, 3138 => 8usize, 3149 => 9usize, 3190 => 10usize, 3207 => 11usize, 3229 => 12usize, 3243 => 13usize, 3260 => 14usize, 3399 => 15usize, 3482 => 16usize, _ => usize::MAX };
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
            let A = node_potentials[8];
            let B = node_potentials[5];
            let D = 1e0f64;
            let E = 1e0f64;
            let G = node_potentials[4];
            let H = node_potentials[3];
            let J = 1e0f64;
            let K = 1e0f64;
            let N = -1e0f64;
            let R = node_potentials[7];
            let T = 1e0f64;
            let V = parameters[1];
            let W = node_potentials[11];
            let X = 0e0f64;
            let Y = 2e0f64;
            let Z = 1e0f64;
            let AA = 1e0f64;
            let AC = staged[0];
            let AE = 0e0f64;
            let AH = 8.617333262e-5f64;
            let AN = 0e0f64;
            let AO = staged[2];
            let AQ = parameters[59];
            let AR = 1e0f64;
            let AS = parameters[8];
            let AV = parameters[60];
            let AW = parameters[11];
            let AZ = parameters[63];
            let BA = parameters[20];
            let BD = parameters[61];
            let BE = parameters[25];
            let BH = parameters[62];
            let BI = parameters[28];
            let BL = parameters[64];
            let BM = parameters[53];
            let BP = parameters[65];
            let BQ = parameters[54];
            let BT = parameters[68];
            let BV = parameters[9];
            let BX = parameters[30];
            let CA = parameters[29];
            let CC = parameters[36];
            let CF = parameters[35];
            let CH = parameters[69];
            let CJ = parameters[41];
            let CL = parameters[70];
            let CN = parameters[21];
            let DN = staged[9];
            let DQ = parameters[39];
            let DT = parameters[19];
            let ED = parameters[10];
            let EE = parameters[15];
            let EI = parameters[22];
            let EW = parameters[12];
            let EZ = parameters[13];
            let FI = 5e-1f64;
            let FM = parameters[14];
            let FR = staged[10];
            let FU = parameters[16];
            let GC = staged[11];
            let GH = staged[12];
            let GS = parameters[17];
            let HM = staged[13];
            let IN = staged[14];
            let KO = Lanes([0e0f64; 5]);
            let KW = parameters[52];
            let KY = parameters[44];
            let LB = parameters[43];
            let LD = parameters[46];
            let LU = parameters[66];
            let MF = staged[15];
            let MG = -1e0f64;
            let MR = staged[16];
            let MZ = parameters[38];
            let NH = parameters[37];
            let NP = parameters[32];
            let NT = parameters[34];
            let OE = staged[17];
            let OR = parameters[26];
            let OS = parameters[24];
            let OT = Lanes([0e0f64; 4]);
            let OU = Lanes([0e0f64; 4]);
            let OV = staged[18];
            let PG = parameters[51];
            let PI = 1e0f64;
            let PL = ddt_scale();
            let PN = staged[6];
            let PP = 1e0f64;
            let PT = staged[19];
            let QC = staged[20];
            let QU = staged[5];
            let SY = 1e0f64;
            let SZ = parameters[27];
            let TE = parameters[23];
            let TJ = node_potentials[10];
            let TL = 1e0f64;
            let TR = staged[21];
            let TW = Lanes([0e0f64; 6]);
            let TZ = node_potentials[9];
            let UA = 1e0f64;
            let UB = parameters[56];
            let UG = staged[22];
            let UH = parameters[55];
            let UK = Lanes([0e0f64; 2]);
            let UN = staged[23];
            let UO = parameters[47];
            let UR = parameters[0];
            let US = Lanes([0e0f64; 2]);
            let UV = staged[24];
            let UW = parameters[45];
            let UZ = Lanes([0e0f64; 2]);
            let VC = staged[25];
            let VD = branch_unknown_flows[5];
            let VE = parameters[42];
            let VG = 1e0f64;
            let VI = parameters[50];
            let VN = staged[26];
            let VY = staged[27];
            let VZ = 0e0f64;
            let WI = branch_unknown_flows[10];
            let WK = 1e0f64;
            let WN = Lanes([0e0f64; 6]);
            let WQ = parameters[49];
            let WS = 1e0f64;
            let WW = staged[28];
            let WX = branch_unknown_flows[14];
            let WZ = 1e0f64;
            let XC = parameters[48];
            let XH = staged[29];
            let XS = staged[30];
            let XT = 0e0f64;
            let YC = Lanes([0e0f64; 6]);
            let YD = staged[31];
            let YQ = 0e0f64;
            let YR = 0e0f64;
            let YS = Lanes([0e0f64; 3]);
            let YT = Lanes([0e0f64; 2]);
            let ZT = 5.5226012e-23f64;
            let ZU = parameters[73];
            let ZW = staged[7];
            let AAB = 3.141592653589793e0f64;
            let AAD = node_potentials[14];
            let AAE = 1e0f64;
            let AAG = node_potentials[15];
            let AAH = 1e0f64;
            let AAQ = staged[35];
            let AAR = parameters[58];
            let AAW = node_potentials[13];
            let AAY = 1e0f64;
            let ABE = -1e0f64;
            let ABH = parameters[57];
            let ABK = 1e-12f64;
            let ABN = Lanes([0e0f64; 5]);
            let C = A - B;
            let F = Lanes([0.0, D]) - Lanes([E, 0.0]);
            let I = G - H;
            let L = Lanes([0.0, J]) - Lanes([K, 0.0]);
            let M = -I;
            let O = L * N;
            let P = H - B;
            let Q = Lanes([K, 0.0]) - Lanes([0.0, E]);
            let S = R - H;
            let U = Lanes([0.0, T]) - Lanes([K, 0.0]);
            let AF;
            let AG;
            if V != 0.0 {
                let AB = AA * ((Y * (if W >= X { 1.0 } else { 0.0 })) - Z);
                let AD = AC + (W.abs());
                AF = AD;
                AG = AB;
            } else {
                AF = AC;
                AG = AE;
            }
            let AI = AF * AH;
            let AJ = AG * AH;
            let AK = AF - staged[1];
            let AL = AK.abs();
            let AM = AG * ((Y * (if AK >= X { 1.0 } else { 0.0 })) - Z);
            let AP = if (if AL > AN { 1.0 } else { 0.0 }) != 0.0 || AO != 0.0 { 1.0 } else { 0.0 };
            let CP;
            let CQ;
            let CR;
            let CS;
            let CT;
            let CU;
            let CV;
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
            if AP != 0.0 {
                let AT = AS * (AR + (AQ * AL));
                let AU = (AM * AQ) * AS;
                let AX = AW * (AR + (AV * AL));
                let AY = (AM * AV) * AW;
                let BB = BA * (AR + (AZ * AL));
                let BC = (AM * AZ) * BA;
                let BF = BE * (AR + (BD * AL));
                let BG = (AM * BD) * BE;
                let BJ = BI * (AR + (BH * AL));
                let BK = (AM * BH) * BI;
                let BN = BM * (AR + (BL * AL));
                let BO = (AM * BL) * BM;
                let BR = BQ * (AR + (BP * AL));
                let BS = (AM * BP) * BQ;
                let BU = AM * BT;
                let BW = BV + (BT * AL);
                let BY = BX * BT;
                let BZ = AM * BY;
                let CB = CA + (BY * AL);
                let CD = CC * BT;
                let CE = AM * CD;
                let CG = CF + (CD * AL);
                let CI = AM * CH;
                let CK = CJ + (CH * AL);
                let CM = AM * CL;
                let CO = CN + (CL * AL);
                CP = AX;
                CQ = BW;
                CR = CO;
                CS = AT;
                CT = BB;
                CU = BN;
                CV = CK;
                CW = CB;
                CX = CG;
                CY = BF;
                CZ = BJ;
                DA = BR;
                DB = AY;
                DC = BU;
                DD = CM;
                DE = AU;
                DF = BC;
                DG = BO;
                DH = CI;
                DI = BZ;
                DJ = CE;
                DK = BG;
                DL = BK;
                DM = BS;
            } else {
                CP = AW;
                CQ = BV;
                CR = CN;
                CS = AS;
                CT = BA;
                CU = BM;
                CV = CJ;
                CW = CA;
                CX = CF;
                CY = BE;
                CZ = BI;
                DA = BQ;
                DB = AE;
                DC = AE;
                DD = AE;
                DE = AE;
                DF = AE;
                DG = AE;
                DH = AE;
                DI = AE;
                DJ = AE;
                DK = AE;
                DL = AE;
                DM = AE;
            }
            let DR;
            let DS;
            if DN != 0.0 {
                let DO = staged[3] / AI;
                let DP = ((AJ * DO) * N) / AI;
                DR = DO;
                DS = DP;
            } else {
                DR = DQ;
                DS = AE;
            }
            let DU = DT * P;
            let DV = DU.cosh();
            let DW = DV * DV;
            let DX = ((Q * DT) * (DU.sinh())) * DV;
            let DY = parameters[18] / DW;
            let DZ = AR + DY;
            let EA = CP * DZ;
            let EB = ((((DX + DX) * DY) * N) / DW) * CP;
            let EC = Lanes([0.0, 0.0, (DB * DZ)]) + Lanes([EB[0], EB[1], 0.0]);
            let EF = (EE * P).tanh();
            let EG = ((Q * EE) * (Z - (EF * EF))) * ED;
            let EH = Lanes([0.0, 0.0, DC]) + Lanes([EG[0], EG[1], 0.0]);
            let EJ = EI * (M - CN);
            let EK = M - CR;
            let EL = Lanes([O[0], O[1], 0.0]) - Lanes([0.0, 0.0, DD]);
            let EM = (O * EI) * EK;
            let EN = Lanes([EM[0], EM[1], 0.0]) + (EL * EJ);
            let EO = ((CQ - ED) + (ED * EF)) - (EJ * EK);
            let EP = Lanes([EH[0], 0.0, EH[1], EH[2]]) - Lanes([EN[0], EN[1], 0.0, EN[2]]);
            let EQ = C - EO;
            let ER = Lanes([0.0, 0.0, F[0], F[1], 0.0]) - Lanes([EP[0], EP[1], EP[2], 0.0, EP[3]]);
            let ES = EQ * EQ;
            let ET = ER * EQ;
            let EU = ET + ET;
            let EV = EC * EQ;
            let EX = EW * ES;
            let EY = EU * EW;
            let FA = EZ * EQ;
            let FB = ((EA * EQ) + EX) + (FA * ES);
            let FC = ((Lanes([EV[0], 0.0, EV[1], 0.0, EV[2]]) + (ER * EA)) + EY) + (((ER * EZ) * ES) + (EU * FA));
            let FD = FB.tanh();
            let FE = FC * (Z - (FD * FD));
            let FF = AR + FD;
            let FG = rspice_limexp(FB);
            let FH = rspice_limexp((-FB));
            let FJ = (FI * (FG - FH)).tanh();
            let FK = (((FC * FG) - ((FC * N) * FH)) * FI) * (Z - (FJ * FJ));
            let FL = AR + FJ;
            let FN = FM + (EE * FF);
            let FO = Q * FN;
            let FP = (FN * P).tanh();
            let FQ = (((FE * EE) * P) + Lanes([FO[0], 0.0, FO[1], 0.0, 0.0])) * (Z - (FP * FP));
            let GD;
            let GE;
            let GF;
            let GG;
            if FR != 0.0 {
                let FS = CS * FF;
                let FT = FS * FP;
                let FV = Q * FU;
                let FW = rspice_limexp(EK);
                let FX = Lanes([0.0, 0.0, (DF * FW)]) + ((EL * FW) * CT);
                let FY = (AR + (FU * P)) + (CT * FW);
                let FZ = FT * FY;
                let GA = (Lanes([FV[0], 0.0, FV[1], 0.0]) + Lanes([FX[0], FX[1], 0.0, FX[2]])) * FT;
                let GB = ((((Lanes([0.0, 0.0, 0.0, 0.0, (DE * FF)]) + (FE * CS)) * FP) + (FQ * FS)) * FY) + Lanes([GA[0], GA[1], GA[2], 0.0, GA[3]]);
                GD = FL;
                GE = FZ;
                GF = FK;
                GG = GB;
            } else {
                let HN;
                let HO;
                let HP;
                let HQ;
                if GC != 0.0 {
                    let GI = I - EO;
                    let GJ = Lanes([L[0], L[1], 0.0, 0.0]) - EP;
                    let GK = GI * GI;
                    let GL = GJ * GI;
                    let GM = GL + GL;
                    let GN = EC * GI;
                    let GO = (((EA * GI) + (EW * GK)) + (EZ * (GK * GI))).tanh();
                    let GP = (((Lanes([GN[0], 0.0, GN[1], GN[2]]) + (GJ * EA)) + (GM * EW)) + (((GM * GI) + (GJ * GK)) * EZ)) * (Z - (GO * GO));
                    let GQ = AR + GO;
                    let GR = FM + (EE * GQ);
                    let GT = FU + (GS * FF);
                    let GU = CS * FF;
                    let GV = AR + FP;
                    let GW = GU * GV;
                    let GX = Q * GT;
                    let GY = rspice_limexp((P - CR));
                    let GZ = Lanes([0.0, 0.0, (DF * GY)]) + (((Lanes([Q[0], Q[1], 0.0]) - Lanes([0.0, 0.0, DD])) * GY) * CT);
                    let HA = (AR + (GT * P)) + (CT * GY);
                    let HB = FU + (GS * GQ);
                    let HC = Q * GR;
                    let HD = (GR * P).tanh();
                    let HE = CS * GQ;
                    let HF = AR - HD;
                    let HG = HE * HF;
                    let HH = Q * HB;
                    let HI = AR - (HB * P);
                    let HJ = ((((Lanes([0.0, 0.0, 0.0, (DE * GQ)]) + (GP * CS)) * HF) + ((((((GP * EE) * P) + Lanes([HC[0], 0.0, HC[1], 0.0])) * (Z - (HD * HD))) * N) * HE)) * HI) + (((((GP * GS) * P) + Lanes([HH[0], 0.0, HH[1], 0.0])) * N) * HG);
                    let HK = FI * ((GW * HA) - (HG * HI));
                    let HL = ((((((Lanes([0.0, 0.0, 0.0, 0.0, (DE * FF)]) + (FE * CS)) * GV) + (FQ * GU)) * HA) + (((((FE * GS) * P) + Lanes([GX[0], 0.0, GX[1], 0.0, 0.0])) + Lanes([GZ[0], 0.0, GZ[1], 0.0, GZ[2]])) * GW)) - Lanes([HJ[0], HJ[1], HJ[2], 0.0, HJ[3]])) * FI;
                    HN = FL;
                    HO = HK;
                    HP = FK;
                    HQ = HL;
                } else {
                    let IO;
                    let IP;
                    let IQ;
                    let IR;
                    if HM != 0.0 {
                        let HR = EZ * ES;
                        let HS = (EQ + EX) + (HR * EQ);
                        let HT = EA * HS;
                        let HU = EC * HS;
                        let HV = Lanes([HU[0], 0.0, HU[1], 0.0, HU[2]]) + (((ER + EY) + (((EU * EZ) * EQ) + (ER * HR))) * EA);
                        let HW = rspice_limexp(HT);
                        let HX = rspice_limexp((-HT));
                        let HY = (FI * (HW - HX)).tanh();
                        let HZ = (((HV * HW) - ((HV * N) * HX)) * FI) * (Z - (HY * HY));
                        let IA = AR + HY;
                        let IB = FM + (EE * IA);
                        let IC = Q * IB;
                        let ID = (IB * P).tanh();
                        let IE = FU + (GS * IA);
                        let IF = CS * IA;
                        let IG = IF * ID;
                        let IH = Q * IE;
                        let II = rspice_limexp(EK);
                        let IJ = Lanes([0.0, 0.0, (DF * II)]) + ((EL * II) * CT);
                        let IK = (AR + (IE * P)) + (CT * II);
                        let IL = IG * IK;
                        let IM = ((((Lanes([0.0, 0.0, 0.0, 0.0, (DE * IA)]) + (HZ * CS)) * ID) + (((((HZ * EE) * P) + Lanes([IC[0], 0.0, IC[1], 0.0, 0.0])) * (Z - (ID * ID))) * IF)) * IK) + (((((HZ * GS) * P) + Lanes([IH[0], 0.0, IH[1], 0.0, 0.0])) + Lanes([IJ[0], IJ[1], 0.0, 0.0, IJ[2]])) * IG);
                        IO = IA;
                        IP = IL;
                        IQ = HZ;
                        IR = IM;
                    } else {
                        let KP;
                        let KQ;
                        let KR;
                        let KS;
                        if IN != 0.0 {
                            let IS = EZ * ES;
                            let IT = (EQ + EX) + (IS * EQ);
                            let IU = EA * IT;
                            let IV = EC * IT;
                            let IW = Lanes([IV[0], 0.0, IV[1], 0.0, IV[2]]) + (((ER + EY) + (((EU * EZ) * EQ) + (ER * IS))) * EA);
                            let IX = I - EO;
                            let IY = Lanes([L[0], L[1], 0.0, 0.0]) - EP;
                            let IZ = IX * IX;
                            let JA = IY * IX;
                            let JB = JA + JA;
                            let JC = EZ * IX;
                            let JD = (IX + (EW * IZ)) + (JC * IZ);
                            let JE = EA * JD;
                            let JF = EC * JD;
                            let JG = Lanes([JF[0], 0.0, JF[1], JF[2]]) + (((IY + (JB * EW)) + (((IY * EZ) * IZ) + (JB * JC))) * EA);
                            let JH = rspice_limexp(IU);
                            let JI = rspice_limexp((-IU));
                            let JJ = (FI * (JH - JI)).tanh();
                            let JK = (((IW * JH) - ((IW * N) * JI)) * FI) * (Z - (JJ * JJ));
                            let JL = AR + JJ;
                            let JM = rspice_limexp(JE);
                            let JN = rspice_limexp((-JE));
                            let JO = (FI * (JM - JN)).tanh();
                            let JP = (((JG * JM) - ((JG * N) * JN)) * FI) * (Z - (JO * JO));
                            let JQ = AR + JO;
                            let JR = FM + (EE * JL);
                            let JS = FM + (EE * JQ);
                            let JT = Q * JR;
                            let JU = (JR * P).tanh();
                            let JV = Q * JS;
                            let JW = (JS * P).tanh();
                            let JX = FU + (GS * JQ);
                            let JY = FU + (GS * JL);
                            let JZ = CS * JL;
                            let KA = AR + JU;
                            let KB = JZ * KA;
                            let KC = Q * JY;
                            let KD = rspice_limexp((P - CR));
                            let KE = Lanes([0.0, 0.0, (DF * KD)]) + (((Lanes([Q[0], Q[1], 0.0]) - Lanes([0.0, 0.0, DD])) * KD) * CT);
                            let KF = (AR + (JY * P)) + (CT * KD);
                            let KG = CS * JQ;
                            let KH = AR - JW;
                            let KI = KG * KH;
                            let KJ = Q * JX;
                            let KK = AR - (JX * P);
                            let KL = ((((Lanes([0.0, 0.0, 0.0, (DE * JQ)]) + (JP * CS)) * KH) + ((((((JP * EE) * P) + Lanes([JV[0], 0.0, JV[1], 0.0])) * (Z - (JW * JW))) * N) * KG)) * KK) + (((((JP * GS) * P) + Lanes([KJ[0], 0.0, KJ[1], 0.0])) * N) * KI);
                            let KM = FI * ((KB * KF) - (KI * KK));
                            let KN = ((((((Lanes([0.0, 0.0, 0.0, 0.0, (DE * JL)]) + (JK * CS)) * KA) + (((((JK * EE) * P) + Lanes([JT[0], 0.0, JT[1], 0.0, 0.0])) * (Z - (JU * JU))) * JZ)) * KF) + (((((JK * GS) * P) + Lanes([KC[0], 0.0, KC[1], 0.0, 0.0])) + Lanes([KE[0], 0.0, KE[1], 0.0, KE[2]])) * KB)) - Lanes([KL[0], KL[1], KL[2], 0.0, KL[3]])) * FI;
                            KP = JL;
                            KQ = KM;
                            KR = JK;
                            KS = KN;
                        } else {
                            KP = FL;
                            KQ = AN;
                            KR = FK;
                            KS = KO;
                        }
                        IO = KP;
                        IP = KQ;
                        IQ = KR;
                        IR = KS;
                    }
                    HN = IO;
                    HO = IP;
                    HP = IQ;
                    HQ = IR;
                }
                GD = HN;
                GE = HO;
                GF = HP;
                GG = HQ;
            }
            let LN;
            let LO;
            let LP;
            let LQ;
            let LR;
            let LS;
            if GH != 0.0 {
                let KT = AR + FF;
                let KU = CU / KT;
                let KV = (Lanes([0.0, 0.0, 0.0, 0.0, DG]) - (FE * KU)) / KT;
                let KX = KW + KU;
                let KZ = KY * FF;
                let LA = FE * KY;
                let LC = LB + KZ;
                let LE = LD + KZ;
                LN = LE;
                LO = LC;
                LP = KX;
                LQ = LA;
                LR = LA;
                LS = KV;
            } else {
                let LF = AR + GD;
                let LG = CU / LF;
                let LH = (Lanes([0.0, 0.0, 0.0, 0.0, DG]) - (GF * LG)) / LF;
                let LI = KW + LG;
                let LJ = KY * GD;
                let LK = GF * KY;
                let LL = LB + LJ;
                let LM = LD + LJ;
                LN = LM;
                LO = LL;
                LP = LI;
                LQ = LK;
                LR = LK;
                LS = LH;
            }
            let LT = if AL != 0.0 || AO != 0.0 { 1.0 } else { 0.0 };
            let MB;
            let MC;
            let MD;
            let ME;
            if LT != 0.0 {
                let LV = AM * LU;
                let LW = AR + (LU * AL);
                let LX = LN * LW;
                let LY = (LQ * LW) + Lanes([0.0, 0.0, 0.0, 0.0, (LV * LN)]);
                let LZ = LO * LW;
                let MA = (LR * LW) + Lanes([0.0, 0.0, 0.0, 0.0, (LV * LO)]);
                MB = LX;
                MC = LZ;
                MD = LY;
                ME = MA;
            } else {
                MB = LN;
                MC = LO;
                MD = LQ;
                ME = LR;
            }
            let MS;
            let MT;
            let MU;
            let MV;
            let MW;
            let MX;
            if MF != 0.0 {
                let MH = (MG * CV).tanh();
                let MI = rspice_limexp((DR * MH));
                let MJ = ((DS * MH) + (((DH * MG) * (Z - (MH * MH))) * DR)) * MI;
                let MK = C - CV;
                let ML = Lanes([F[0], F[1], 0.0]) - Lanes([0.0, 0.0, DH]);
                let MM = S - CV;
                let MN = Lanes([U[0], U[1], 0.0]) - Lanes([0.0, 0.0, DH]);
                MS = MK;
                MT = MI;
                MU = MM;
                MV = ML;
                MW = MJ;
                MX = MN;
            } else {
                let MO = -DR;
                let MP = rspice_limexp((MO * CV));
                let MQ = (((DS * N) * CV) + (DH * MO)) * MP;
                let ON;
                let OO;
                let OP;
                let OQ;
                if MR != 0.0 {
                    let OF = (C - CV).tanh();
                    let OG = (Lanes([F[0], F[1], 0.0]) - Lanes([0.0, 0.0, DH])) * (Z - (OF * OF));
                    let OH = (S - CV).tanh();
                    let OI = (Lanes([U[0], U[1], 0.0]) - Lanes([0.0, 0.0, DH])) * (Z - (OH * OH));
                    ON = OF;
                    OO = OH;
                    OP = OG;
                    OQ = OI;
                } else {
                    let OJ = C - CV;
                    let OK = Lanes([F[0], F[1], 0.0]) - Lanes([0.0, 0.0, DH]);
                    let OL = S - CV;
                    let OM = Lanes([U[0], U[1], 0.0]) - Lanes([0.0, 0.0, DH]);
                    ON = OJ;
                    OO = OL;
                    OP = OK;
                    OQ = OM;
                }
                MS = ON;
                MT = MP;
                MU = OO;
                MV = OP;
                MW = MQ;
                MX = OQ;
            }
            let MY = rspice_limexp((DR * MS));
            let NA = MZ * (MY - MT);
            let NB = (((Lanes([0.0, 0.0, (DS * MS)]) + (MV * DR)) * MY) - Lanes([0.0, 0.0, MW])) * MZ;
            let NC = rspice_limexp((DR * MU));
            let ND = MZ * (NC - MT);
            let NE = (((Lanes([0.0, 0.0, (DS * MU)]) + (MX * DR)) * NC) - Lanes([0.0, 0.0, MW])) * MZ;
            let NF = F * BX;
            let NG = Lanes([0.0, 0.0, DI]) + Lanes([NF[0], NF[1], 0.0]);
            let NI = NH * P;
            let NJ = Q * NH;
            let NK = (CW + (BX * C)) + NI;
            let NL = Lanes([0.0, NG[0], NG[1], NG[2]]) + Lanes([NJ[0], NJ[1], 0.0, 0.0]);
            let NM = NK.tanh();
            let NN = NL * (Z - (NM * NM));
            let NO = AR + NM;
            let NQ = (parameters[31] + (NP * P)).tanh();
            let NR = (Q * NP) * (Z - (NQ * NQ));
            let NS = AR + NQ;
            let NU = (parameters[33] - (NT * P)).tanh();
            let NV = ((Q * NT) * N) * (Z - (NU * NU));
            let NW = (AR + NU) - NH;
            let NX = U * CC;
            let NY = Lanes([0.0, 0.0, DJ]) + Lanes([NX[0], NX[1], 0.0]);
            let NZ = (CX + (CC * S)) - NI;
            let OA = Lanes([NY[0], 0.0, NY[1], NY[2]]) - Lanes([NJ[0], NJ[1], 0.0, 0.0]);
            let OB = NZ.tanh();
            let OC = OA * (Z - (OB * OB));
            let OD = AR + OB;
            let OW;
            let OX;
            let OY;
            let OZ;
            let PA;
            let PB;
            let PC;
            let PD;
            if OE != 0.0 {
                OW = AN;
                OX = AN;
                OY = OR;
                OZ = OS;
                PA = OT;
                PB = OU;
                PC = OT;
                PD = OU;
            } else {
                let QD;
                let QE;
                let QF;
                let QG;
                let QH;
                let QI;
                let QJ;
                let QK;
                if OV != 0.0 {
                    let PU = CY * NO;
                    let PV = NR * PU;
                    let PW = ((Lanes([0.0, 0.0, 0.0, (DK * NO)]) + (NN * CY)) * NS) + Lanes([PV[0], PV[1], 0.0, 0.0]);
                    let PX = OS + (PU * NS);
                    let PY = NV * OD;
                    let PZ = (NW * OD) + staged[4];
                    let QA = Lanes([0.0, 0.0, 0.0, (DL * PZ)]) + ((Lanes([PY[0], PY[1], 0.0, 0.0]) + (OC * NW)) * CZ);
                    let QB = OR + (CZ * PZ);
                    QD = AN;
                    QE = AN;
                    QF = QB;
                    QG = PX;
                    QH = OT;
                    QI = OU;
                    QJ = QA;
                    QK = PW;
                } else {
                    let RO;
                    let RP;
                    let RQ;
                    let RR;
                    let RS;
                    let RT;
                    if QC != 0.0 {
                        let QL = NS - NH;
                        let QM = CW + NI;
                        let QN = Lanes([NJ[0], NJ[1], 0.0]);
                        let QO = Lanes([0.0, 0.0, DI]) + QN;
                        let QP = QM.cosh();
                        let QQ = NK.cosh();
                        let QR = QO + ((QO * (QM.sinh())) * (Z / QP));
                        let QS = (NK + (QQ.ln())) - (QM + (QP.ln()));
                        let QT = NR * QS;
                        let QV = F * QU;
                        let QW = ((QS * QL) / BX) + (QU * C);
                        let QX = F * OS;
                        let QY = (CY * QW) + (OS * C);
                        let QZ = (Lanes([0.0, 0.0, 0.0, (DK * QW)]) + (((((((NL + ((NL * (NK.sinh())) * (Z / QQ))) - Lanes([QR[0], QR[1], 0.0, QR[2]])) * QL) + Lanes([QT[0], QT[1], 0.0, 0.0])) / BX) + Lanes([0.0, QV[0], QV[1], 0.0])) * CY)) + Lanes([0.0, QX[0], QX[1], 0.0]);
                        let RA = CX - NI;
                        let RB = Lanes([0.0, 0.0, DJ]) - QN;
                        let RC = RA.cosh();
                        let RD = NZ.cosh();
                        let RE = RB + ((RB * (RA.sinh())) * (Z / RC));
                        let RF = (NZ + (RD.ln())) - (RA + (RC.ln()));
                        let RG = NV * RF;
                        let RH = U * QU;
                        let RI = ((RF * NW) / CC) + (QU * S);
                        let RJ = U * OR;
                        let RK = (CZ * RI) + (OR * S);
                        let RL = (Lanes([0.0, 0.0, 0.0, (DL * RI)]) + (((((((OA + ((OA * (NZ.sinh())) * (Z / RD))) - Lanes([RE[0], RE[1], 0.0, RE[2]])) * NW) + Lanes([RG[0], RG[1], 0.0, 0.0])) / CC) + Lanes([RH[0], 0.0, RH[1], 0.0])) * CZ)) + Lanes([RJ[0], 0.0, RJ[1], 0.0]);
                        let RM = QZ[2];
                        let RN = RL[2];
                        RO = RK;
                        RP = QY;
                        RQ = RN;
                        RR = RM;
                        RS = RL;
                        RT = QZ;
                    } else {
                        RO = AN;
                        RP = AN;
                        RQ = AN;
                        RR = AN;
                        RS = OT;
                        RT = OU;
                    }
                    QD = RO;
                    QE = RP;
                    QF = RQ;
                    QG = RR;
                    QH = RS;
                    QI = RT;
                    QJ = OT;
                    QK = OU;
                }
                OW = QD;
                OX = QE;
                OY = QF;
                OZ = QG;
                PA = QH;
                PB = QI;
                PC = QJ;
                PD = QK;
            }
            let PE = -GE;
            let PF = GG * N;
            let PH = PG * node_potentials[12];
            let PJ = PI * PG;
            let PK = ddt(3101, PH);
            let PM = PJ * PL;
            let PO = PN * branch_unknown_flows[0];
            let PQ = PP * PN;
            let PR = ddt(3108, PO);
            let PS = PQ * PL;
            let SI;
            let SJ;
            let SK;
            let SL;
            let SM;
            let SN;
            let SO;
            let SP;
            let SQ;
            let SR;
            let SS;
            let ST;
            let SU;
            let SV;
            let SW;
            let SX;
            if PT != 0.0 {
                let RU = ddt(3116, OW);
                let RV = PA * PL;
                let RW = ddt(3118, OX);
                let RX = PB * PL;
                SI = RU;
                SJ = RW;
                SK = AN;
                SL = AN;
                SM = OW;
                SN = OX;
                SO = AN;
                SP = AN;
                SQ = RV;
                SR = RX;
                SS = OT;
                ST = OU;
                SU = PA;
                SV = PB;
                SW = OT;
                SX = OU;
            } else {
                let RY = OY * S;
                let RZ = U * OY;
                let SA = (PC * S) + Lanes([RZ[0], 0.0, RZ[1], 0.0]);
                let SB = ddt(3122, RY);
                let SC = SA * PL;
                let SD = OZ * C;
                let SE = F * OZ;
                let SF = (PD * C) + Lanes([0.0, SE[0], SE[1], 0.0]);
                let SG = ddt(3126, SD);
                let SH = SF * PL;
                SI = AN;
                SJ = AN;
                SK = SB;
                SL = SG;
                SM = AN;
                SN = AN;
                SO = RY;
                SP = SD;
                SQ = OT;
                SR = OU;
                SS = SC;
                ST = SH;
                SU = OT;
                SV = OU;
                SW = SA;
                SX = SF;
            }
            let TA = SZ * (node_potentials[1] - H);
            let TB = (Lanes([SY, 0.0]) - Lanes([0.0, K])) * SZ;
            let TC = ddt(3130, TA);
            let TD = TB * PL;
            let TF = TE * P;
            let TG = Q * TE;
            let TH = ddt(3134, TF);
            let TI = TG * PL;
            let TK = H - TJ;
            let TM = DA * TK;
            let TN = (Lanes([K, 0.0]) - Lanes([0.0, TL])) * DA;
            let TO = Lanes([0.0, 0.0, (DM * TK)]) + Lanes([TN[0], TN[1], 0.0]);
            let TP = ddt(3138, TM);
            let TQ = TO * PL;
            let TX;
            let TY;
            if TR != 0.0 {
                let TS = Lanes([0.0, TL]) - Lanes([E, 0.0]);
                let TT = (TJ - B) / LP;
                let TU = LS * TT;
                let TV = (Lanes([0.0, 0.0, TS[0], 0.0, TS[1], 0.0]) - Lanes([TU[0], TU[1], TU[2], TU[3], 0.0, TU[4]])) / LP;
                TX = TT;
                TY = TV;
            } else {
                TX = AN;
                TY = TW;
            }
            let UC = UB * (TZ - A);
            let UD = (Lanes([0.0, UA]) - Lanes([D, 0.0])) * UB;
            let UE = ddt(3149, UC);
            let UF = UD * PL;
            let UL;
            let UM;
            if UG != 0.0 {
                let UI = (TZ - B) / UH;
                let UJ = (Lanes([0.0, UA]) - Lanes([E, 0.0])) / UH;
                UL = UI;
                UM = UJ;
            } else {
                UL = AN;
                UM = UK;
            }
            let UT;
            let UU;
            if UN != 0.0 {
                let UP = (G - R) / UO;
                let UQ = (Lanes([J, 0.0]) - Lanes([0.0, T])) / UO;
                UT = UP;
                UU = UQ;
            } else {
                UT = AN;
                UU = US;
            }
            let VA;
            let VB;
            if UV != 0.0 {
                let UX = (G - A) / UW;
                let UY = (Lanes([J, 0.0]) - Lanes([0.0, D])) / UW;
                VA = UX;
                VB = UY;
            } else {
                VA = AN;
                VB = UZ;
            }
            let VO;
            let VP;
            let VQ;
            let VR;
            let VS;
            let VT;
            let VU;
            let VV;
            let VW;
            let VX;
            if VC != 0.0 {
                let VF = VD * VE;
                let VH = VG * VE;
                let VJ = VI * VD;
                let VK = VG * VI;
                let VL = ddt(3190, VJ);
                let VM = VK * PL;
                VO = VF;
                VP = VL;
                VQ = AN;
                VR = VJ;
                VS = AN;
                VT = VH;
                VU = VM;
                VV = VZ;
                VW = VK;
                VX = VZ;
            } else {
                let WE;
                let WF;
                let WG;
                let WH;
                if VN != 0.0 {
                    let WA = VI * VD;
                    let WB = VG * VI;
                    let WC = ddt(3207, WA);
                    let WD = WB * PL;
                    WE = WC;
                    WF = WA;
                    WG = WD;
                    WH = WB;
                } else {
                    WE = AN;
                    WF = AN;
                    WG = VZ;
                    WH = VZ;
                }
                VO = AN;
                VP = AN;
                VQ = WE;
                VR = AN;
                VS = WF;
                VT = VZ;
                VU = VZ;
                VV = WG;
                VW = VZ;
                VX = WH;
            }
            let WO;
            let WP;
            if VY != 0.0 {
                let WJ = WI * MB;
                let WL = MD * WI;
                let WM = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (WK * MB)]) + Lanes([WL[0], WL[1], WL[2], WL[3], WL[4], 0.0]);
                WO = WJ;
                WP = WM;
            } else {
                WO = AN;
                WP = WN;
            }
            let WR = WQ * branch_unknown_flows[13];
            let WT = WS * WQ;
            let WU = ddt(3229, WR);
            let WV = WT * PL;
            let XI;
            let XJ;
            let XK;
            let XL;
            let XM;
            let XN;
            let XO;
            let XP;
            let XQ;
            let XR;
            if WW != 0.0 {
                let WY = WX * MC;
                let XA = ME * WX;
                let XB = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (WZ * MC)]) + Lanes([XA[0], XA[1], XA[2], XA[3], XA[4], 0.0]);
                let XD = XC * WX;
                let XE = WZ * XC;
                let XF = ddt(3243, XD);
                let XG = XE * PL;
                XI = WY;
                XJ = XF;
                XK = AN;
                XL = XD;
                XM = AN;
                XN = XB;
                XO = XG;
                XP = XT;
                XQ = XE;
                XR = XT;
            } else {
                let XY;
                let XZ;
                let YA;
                let YB;
                if XH != 0.0 {
                    let XU = XC * WX;
                    let XV = WZ * XC;
                    let XW = ddt(3260, XU);
                    let XX = XV * PL;
                    XY = XW;
                    XZ = XU;
                    YA = XX;
                    YB = XV;
                } else {
                    XY = AN;
                    XZ = AN;
                    YA = XT;
                    YB = XT;
                }
                XI = AN;
                XJ = AN;
                XK = XY;
                XL = AN;
                XM = XZ;
                XN = YC;
                XO = XT;
                XP = YA;
                XQ = XT;
                XR = YB;
            }
            let YE;
            let YF;
            let YG;
            let YH;
            let YI;
            let YJ;
            let YK;
            let YL;
            let YM;
            let YN;
            let YO;
            let YP;
            if XS != 0.0 {
                YE = AN;
                YF = AN;
                YG = AN;
                YH = AN;
                YI = AN;
                YJ = AN;
                YK = YQ;
                YL = YR;
                YM = YQ;
                YN = YS;
                YO = YT;
                YP = YT;
            } else {
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
                if YD != 0.0 {
                    let ZH;
                    let ZI;
                    let ZJ;
                    let ZK;
                    let ZL;
                    let ZM;
                    let ZN;
                    let ZO;
                    let ZP;
                    let ZQ;
                    let ZR;
                    let ZS;
                    if UR != 0.0 {
                        let ZG = if GG[1] > AN { 1.0 } else { 0.0 };
                        let ZV = (ZT * AF) * ZU;
                        let ZX = (ZV * CY) * ZW;
                        let ZY = ((((AG * ZT) * ZU) * CY) + (DK * ZV)) * ZW;
                        let ZZ = ZY * ZX;
                        let AAA = (AR - (ZX * ZX)).sqrt();
                        let AAC = (-ZX) * AAB;
                        let AAF = Lanes([(((ZY * N) * AAB) * AAD), 0.0]) + Lanes([0.0, (AAE * AAC)]);
                        let AAI = Lanes([((((ZZ + ZZ) * N) * (Z / (Y * AAA))) * AAG), 0.0]) + Lanes([0.0, (AAH * AAA)]);
                        let AAJ = (AAC * AAD) + (AAA * AAG);
                        let AAK = Lanes([AAF[0], AAF[1], 0.0]) + Lanes([AAI[0], 0.0, AAI[1]]);
                        let AAL = -(ZX * AAB);
                        let AAM = AAL * AAD;
                        let AAN = Lanes([(((ZY * AAB) * N) * AAD), 0.0]) + Lanes([0.0, (AAE * AAL)]);
                        let AAO = ddt(3399, AAM);
                        let AAP = AAN * PL;
                        ZH = AAD;
                        ZI = AAG;
                        ZJ = AAD;
                        ZK = AAJ;
                        ZL = AAO;
                        ZM = AAM;
                        ZN = AAE;
                        ZO = AAH;
                        ZP = AAE;
                        ZQ = AAK;
                        ZR = AAP;
                        ZS = AAN;
                    } else {
                        ZH = AN;
                        ZI = AN;
                        ZJ = AN;
                        ZK = AN;
                        ZL = AN;
                        ZM = AN;
                        ZN = YQ;
                        ZO = YR;
                        ZP = YQ;
                        ZQ = YS;
                        ZR = YT;
                        ZS = YT;
                    }
                    YU = ZH;
                    YV = ZI;
                    YW = ZJ;
                    YX = ZK;
                    YY = ZL;
                    YZ = ZM;
                    ZA = ZN;
                    ZB = ZO;
                    ZC = ZP;
                    ZD = ZQ;
                    ZE = ZR;
                    ZF = ZS;
                } else {
                    YU = AN;
                    YV = AN;
                    YW = AN;
                    YX = AN;
                    YY = AN;
                    YZ = AN;
                    ZA = YQ;
                    ZB = YR;
                    ZC = YQ;
                    ZD = YS;
                    ZE = YT;
                    ZF = YT;
                }
                YE = YU;
                YF = YV;
                YG = YW;
                YH = YX;
                YI = YY;
                YJ = YZ;
                YK = ZA;
                YL = ZB;
                YM = ZC;
                YN = ZD;
                YO = ZE;
                YP = ZF;
            }
            let ABO;
            let ABP;
            let ABQ;
            let ABR;
            let ABS;
            let ABT;
            let ABU;
            let ABV;
            let ABW;
            let ABX;
            if AAQ != 0.0 {
                let AAS = AAR * W;
                let AAT = AA * AAR;
                let AAU = ddt(3482, AAS);
                let AAV = AAT * PL;
                let AAX = -AAW;
                let AAZ = Q * AAX;
                let ABA = Lanes([0.0, 0.0, ((AAY * N) * P)]) + Lanes([AAZ[0], AAZ[1], 0.0]);
                let ABB = F * NA;
                let ABC = (NB * C) + Lanes([ABB[0], ABB[1], 0.0]);
                let ABD = (AAX * P) + (NA * C);
                let ABF = ABE * (ABD.abs());
                let ABG = ((Lanes([ABA[0], ABA[1], 0.0, 0.0, ABA[2]]) + Lanes([0.0, ABC[0], ABC[1], ABC[2], 0.0])) * ((Y * (if ABD >= X { 1.0 } else { 0.0 })) - Z)) * ABE;
                let ABI = W / ABH;
                let ABJ = AA / ABH;
                ABO = AAU;
                ABP = ABF;
                ABQ = ABI;
                ABR = AN;
                ABS = AAS;
                ABT = AAV;
                ABU = ABG;
                ABV = ABJ;
                ABW = AE;
                ABX = AAT;
            } else {
                let ABL = W * ABK;
                let ABM = AA * ABK;
                ABO = AN;
                ABP = AN;
                ABQ = AN;
                ABR = ABL;
                ABS = AN;
                ABT = AE;
                ABU = ABN;
                ABV = AE;
                ABW = ABM;
                ABX = AE;
            }
            let ABY = PF[0];
            let ABZ = PF[1];
            let ACA = PF[2];
            let ACB = PF[3];
            let ACC = PF[4];
            let ACD = PM;
            let ACE = AAY;
            let ACF = PS;
            let ACG = NB[0];
            let ACH = NB[1];
            let ACI = NB[2];
            let ACJ = NE[0];
            let ACK = NE[1];
            let ACL = NE[2];
            let ACM = SQ[0];
            let ACN = SQ[1];
            let ACO = SQ[2];
            let ACP = SQ[3];
            let ACQ = SR[0];
            let ACR = SR[1];
            let ACS = SR[2];
            let ACT = SR[3];
            let ACU = SS[0];
            let ACV = SS[1];
            let ACW = SS[2];
            let ACX = SS[3];
            let ACY = ST[0];
            let ACZ = ST[1];
            let ADA = ST[2];
            let ADB = ST[3];
            let ADC = TD[0];
            let ADD = TD[1];
            let ADE = TI[0];
            let ADF = TI[1];
            let ADG = TQ[0];
            let ADH = TQ[1];
            let ADI = TQ[2];
            let ADJ = TY[0];
            let ADK = TY[1];
            let ADL = TY[2];
            let ADM = TY[3];
            let ADN = TY[4];
            let ADO = TY[5];
            let ADP = UF[0];
            let ADQ = UF[1];
            let ADR = UM[0];
            let ADS = UM[1];
            let ADT = UU[0];
            let ADU = UU[1];
            let ADV = VB[0];
            let ADW = VB[1];
            let ADX = VT;
            let ADY = VU;
            let ADZ = VV;
            let AEA = WP[0];
            let AEB = WP[1];
            let AEC = WP[2];
            let AED = WP[3];
            let AEE = WP[4];
            let AEF = WP[5];
            let AEG = WV;
            let AEH = XN[0];
            let AEI = XN[1];
            let AEJ = XN[2];
            let AEK = XN[3];
            let AEL = XN[4];
            let AEM = XN[5];
            let AEN = XO;
            let AEO = XP;
            let AEP = YK;
            let AEQ = YL;
            let AER = YM;
            let AES = YN[0];
            let AET = YN[1];
            let AEU = YN[2];
            let AEV = YO[0];
            let AEW = YO[1];
            let AEX = AAE;
            let AEY = AAH;
            let AEZ = ABT;
            let AFA = ABU[0];
            let AFB = ABU[1];
            let AFC = ABU[2];
            let AFD = ABU[3];
            let AFE = ABU[4];
            let AFF = ABV;
            let AFG = ABW;
            let AFH = PJ;
            let AFI = PQ;
            let AFJ = SU[0];
            let AFK = SU[1];
            let AFL = SU[2];
            let AFM = SU[3];
            let AFN = SV[0];
            let AFO = SV[1];
            let AFP = SV[2];
            let AFQ = SV[3];
            let AFR = SW[0];
            let AFS = SW[1];
            let AFT = SW[2];
            let AFU = SW[3];
            let AFV = SX[0];
            let AFW = SX[1];
            let AFX = SX[2];
            let AFY = SX[3];
            let AFZ = TB[0];
            let AGA = TB[1];
            let AGB = TG[0];
            let AGC = TG[1];
            let AGD = TO[0];
            let AGE = TO[1];
            let AGF = TO[2];
            let AGG = UD[0];
            let AGH = UD[1];
            let AGI = VW;
            let AGJ = VX;
            let AGK = WT;
            let AGL = XQ;
            let AGM = XR;
            let AGN = YP[0];
            let AGO = YP[1];
            let AGP = ABX;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (PE),
            [3, 4, 5, 8, 11],
            [ABY, ABZ, ACA, ACB, ACC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (PK),
            [12],
            [ACD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (AAW),
            [13],
            [ACE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(12), Some(13), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            0,
            PR,
            [],
            [],
            [0],
            [ACF],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            Some(5),
            multiplicity * (AAW),
            [13],
            [ACE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (NA),
            [5, 8, 11],
            [ACG, ACH, ACI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(3),
            multiplicity * (ND),
            [3, 7, 11],
            [ACJ, ACK, ACL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (SI),
            [3, 5, 7, 11],
            [ACM, ACN, ACO, ACP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (SJ),
            [3, 5, 8, 11],
            [ACQ, ACR, ACS, ACT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (SK),
            [3, 5, 7, 11],
            [ACU, ACV, ACW, ACX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (SL),
            [3, 5, 8, 11],
            [ACY, ACZ, ADA, ADB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (TC),
            [1, 3],
            [ADC, ADD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(5),
            multiplicity * (TH),
            [3, 5],
            [ADE, ADF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(10),
            multiplicity * (TP),
            [3, 10, 11],
            [ADG, ADH, ADI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (TX),
            [3, 4, 5, 8, 10, 11],
            [ADJ, ADK, ADL, ADM, ADN, ADO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[36],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(8),
            multiplicity * (UE),
            [8, 9],
            [ADP, ADQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(5),
            multiplicity * (UL),
            [5, 9],
            [ADR, ADS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(5), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[37],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(7),
            multiplicity * (UT),
            [4, 7],
            [ADT, ADU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(7),
            multiplicity * (staged[38]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(7), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[39],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(8),
            multiplicity * (VA),
            [4, 8],
            [ADV, ADW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(8), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[40],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            5,
            VO,
            [],
            [],
            [5],
            [ADX],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            6,
            VP,
            [],
            [],
            [5],
            [ADY],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            staged[41],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            8,
            VQ,
            [],
            [],
            [5],
            [ADZ],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            staged[42],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 10, multiplicity);
        stamper.stamp_potential_sparse_local::<5, 1>(
            10,
            WO,
            [3, 4, 5, 8, 11],
            [AEA, AEB, AEC, AED, AEE],
            [10],
            [AEF],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 11, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            11,
            staged[43],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            staged[44],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(6), Some(2), 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            13,
            WU,
            [],
            [],
            [13],
            [AEG],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 14, multiplicity);
        stamper.stamp_potential_sparse_local::<5, 1>(
            14,
            XI,
            [3, 4, 5, 8, 11],
            [AEH, AEI, AEJ, AEK, AEL],
            [14],
            [AEM],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 15, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            15,
            XJ,
            [],
            [],
            [14],
            [AEN],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 16, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            16,
            staged[45],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 17, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 1>(
            17,
            XK,
            [],
            [],
            [14],
            [AEO],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(0), 18, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            18,
            staged[46],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[47]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[48]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            None,
            multiplicity * (staged[49]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (YE),
            [14],
            [AEP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(15),
            None,
            multiplicity * (staged[50]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (YF),
            [15],
            [AEQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            Some(5),
            multiplicity * (YG),
            [14],
            [AER],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            Some(3),
            multiplicity * (YH),
            [11, 14, 15],
            [AES, AET, AEU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (YI),
            [11, 14],
            [AEV, AEW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[51]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (staged[52]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[53]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(5),
            multiplicity * (staged[54]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (AAD),
            [14],
            [AEX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (AAG),
            [15],
            [AEY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (staged[55]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(3),
            multiplicity * (staged[56]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (staged[57]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(3),
            multiplicity * (staged[58]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (ABO),
            [11],
            [AEZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            None,
            multiplicity * (ABP),
            [3, 5, 8, 11, 13],
            [AFA, AFB, AFC, AFD, AFE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (ABQ),
            [11],
            [AFF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (ABR),
            [11],
            [AFG],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = PE;
        self.canonical_reactive[1] = PH;
        self.canonical_reactive[2] = AFH;
        self.canonical_reactive[3] = AAW;
        self.canonical_reactive[4] = PO;
        self.canonical_reactive[5] = AFI;
        self.canonical_reactive[6] = AAW;
        self.canonical_reactive[7] = NA;
        self.canonical_reactive[8] = ND;
        self.canonical_reactive[9] = SM;
        self.canonical_reactive[10] = AFJ;
        self.canonical_reactive[11] = AFK;
        self.canonical_reactive[12] = AFL;
        self.canonical_reactive[13] = AFM;
        self.canonical_reactive[14] = SN;
        self.canonical_reactive[15] = AFN;
        self.canonical_reactive[16] = AFO;
        self.canonical_reactive[17] = AFP;
        self.canonical_reactive[18] = AFQ;
        self.canonical_reactive[19] = SO;
        self.canonical_reactive[20] = AFR;
        self.canonical_reactive[21] = AFS;
        self.canonical_reactive[22] = AFT;
        self.canonical_reactive[23] = AFU;
        self.canonical_reactive[24] = SP;
        self.canonical_reactive[25] = AFV;
        self.canonical_reactive[26] = AFW;
        self.canonical_reactive[27] = AFX;
        self.canonical_reactive[28] = AFY;
        self.canonical_reactive[29] = TA;
        self.canonical_reactive[30] = AFZ;
        self.canonical_reactive[31] = AGA;
        self.canonical_reactive[32] = TF;
        self.canonical_reactive[33] = AGB;
        self.canonical_reactive[34] = AGC;
        self.canonical_reactive[35] = TM;
        self.canonical_reactive[36] = AGD;
        self.canonical_reactive[37] = AGE;
        self.canonical_reactive[38] = AGF;
        self.canonical_reactive[39] = TX;
        self.canonical_reactive[40] = staged[36];
        self.canonical_reactive[41] = UC;
        self.canonical_reactive[42] = AGG;
        self.canonical_reactive[43] = AGH;
        self.canonical_reactive[44] = UL;
        self.canonical_reactive[45] = staged[37];
        self.canonical_reactive[46] = UT;
        self.canonical_reactive[47] = staged[38];
        self.canonical_reactive[48] = staged[39];
        self.canonical_reactive[49] = VA;
        self.canonical_reactive[50] = staged[40];
        self.canonical_reactive[51] = VO;
        self.canonical_reactive[52] = VR;
        self.canonical_reactive[53] = AGI;
        self.canonical_reactive[54] = staged[41];
        self.canonical_reactive[55] = VS;
        self.canonical_reactive[56] = AGJ;
        self.canonical_reactive[57] = staged[42];
        self.canonical_reactive[58] = WO;
        self.canonical_reactive[59] = staged[43];
        self.canonical_reactive[60] = staged[44];
        self.canonical_reactive[61] = WR;
        self.canonical_reactive[62] = AGK;
        self.canonical_reactive[63] = XI;
        self.canonical_reactive[64] = XL;
        self.canonical_reactive[65] = AGL;
        self.canonical_reactive[66] = staged[45];
        self.canonical_reactive[67] = XM;
        self.canonical_reactive[68] = AGM;
        self.canonical_reactive[69] = staged[46];
        self.canonical_reactive[70] = staged[47];
        self.canonical_reactive[71] = staged[48];
        self.canonical_reactive[72] = staged[49];
        self.canonical_reactive[73] = YE;
        self.canonical_reactive[74] = staged[50];
        self.canonical_reactive[75] = YF;
        self.canonical_reactive[76] = YG;
        self.canonical_reactive[77] = YH;
        self.canonical_reactive[78] = YJ;
        self.canonical_reactive[79] = AGN;
        self.canonical_reactive[80] = AGO;
        self.canonical_reactive[81] = staged[51];
        self.canonical_reactive[82] = staged[52];
        self.canonical_reactive[83] = staged[53];
        self.canonical_reactive[84] = staged[54];
        self.canonical_reactive[85] = AAD;
        self.canonical_reactive[86] = AAG;
        self.canonical_reactive[87] = staged[55];
        self.canonical_reactive[88] = staged[56];
        self.canonical_reactive[89] = staged[57];
        self.canonical_reactive[90] = staged[58];
        self.canonical_reactive[91] = ABS;
        self.canonical_reactive[92] = AGP;
        self.canonical_reactive[93] = ABP;
        self.canonical_reactive[94] = ABQ;
        self.canonical_reactive[95] = ABR;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            None,
            &[12],
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
            Some(7),
            Some(3),
            &[3, 5, 7, 11],
            &[cached[10], cached[11], cached[12], cached[13]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[3, 5, 8, 11],
            &[cached[15], cached[16], cached[17], cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(3),
            &[3, 5, 7, 11],
            &[cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[3, 5, 8, 11],
            &[cached[25], cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(3),
            &[1, 3],
            &[cached[30], cached[31]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(5),
            &[3, 5],
            &[cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(10),
            &[3, 10, 11],
            &[cached[36], cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(8),
            &[8, 9],
            &[cached[42], cached[43]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            6,
            &[],
            &[],
            &[5],
            &[cached[53]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            8,
            &[],
            &[],
            &[5],
            &[cached[56]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            13,
            &[],
            &[],
            &[13],
            &[cached[62]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            15,
            &[],
            &[],
            &[14],
            &[cached[65]],
        );
        stamper.stamp_potential_reactive_indexed_dense_local(
            17,
            &[],
            &[],
            &[14],
            &[cached[68]],
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(3),
            &[11, 14],
            &[cached[79], cached[80]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            None,
            &[11],
            &[cached[92]],
            &[],
            &[],
            multiplicity,
        );
    }

}
