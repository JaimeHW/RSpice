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
        let mut key = Vec::with_capacity(222);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[0] = values[0];
        self.canonical_staged[2] = values[1];
        self.canonical_staged[8] = values[2];
        self.canonical_staged[16] = values[3];
        self.canonical_staged[3] = values[4];
        self.canonical_staged[9] = values[5];
        self.canonical_staged[17] = values[6];
        self.canonical_staged[11] = values[7];
        self.canonical_staged[12] = values[8];
        self.canonical_staged[23] = values[9];
        self.canonical_staged[1] = values[10];
        self.canonical_staged[4] = values[11];
        self.canonical_staged[5] = values[12];
        self.canonical_staged[6] = values[13];
        self.canonical_staged[7] = values[14];
        self.canonical_staged[10] = values[15];
        self.canonical_staged[26] = values[16];
        self.canonical_staged[27] = values[17];
        self.canonical_staged[13] = values[18];
        self.canonical_staged[28] = values[19];
        self.canonical_staged[14] = values[20];
        self.canonical_staged[15] = values[21];
        self.canonical_staged[29] = values[22];
        self.canonical_staged[30] = values[23];
        self.canonical_staged[70] = values[24];
        self.canonical_staged[71] = values[25];
        self.canonical_staged[19] = values[26];
        self.canonical_staged[72] = values[27];
        self.canonical_staged[73] = values[28];
        self.canonical_staged[74] = values[29];
        self.canonical_staged[75] = values[30];
        self.canonical_staged[20] = values[31];
        self.canonical_staged[21] = values[32];
        self.canonical_staged[76] = values[33];
        self.canonical_staged[77] = values[34];
        self.canonical_staged[78] = values[35];
        self.canonical_staged[79] = values[36];
        self.canonical_staged[80] = values[37];
        self.canonical_staged[81] = values[38];
        self.canonical_staged[82] = values[39];
        self.canonical_staged[83] = values[40];
        self.canonical_staged[84] = values[41];
        self.canonical_staged[85] = values[42];
        self.canonical_staged[86] = values[43];
        self.canonical_staged[87] = values[44];
        self.canonical_staged[88] = values[45];
        self.canonical_staged[22] = values[46];
        self.canonical_staged[93] = values[47];
        self.canonical_staged[89] = values[48];
        self.canonical_staged[90] = values[49];
        self.canonical_staged[91] = values[50];
        self.canonical_staged[92] = values[51];
        self.canonical_staged[94] = values[52];
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
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let B = 1.3806226e-23f64;
                let C = 1.602176462e-19f64;
                let F = parameters[76];
                let G = parameters[77];
                let H = 5e-1f64;
                let J = parameters[78];
                let N = 1e0f64;
                let O = parameters[87];
                let T = 0e0f64;
                let AP = parameters[103];
                let AQ = parameters[104];
                let AR = parameters[111];
                let AU = 1e2f64;
                let AW = parameters[36];
                let AZ = parameters[39];
                let BD = 1e6f64;
                let BQ = 0e0f64;
                let BT = 0e0f64;
                let BW = 0e0f64;
                let CB = 0e0f64;
                let CD = 0e0f64;
                let CF = 0e0f64;
                let CH = 0e0f64;
                let mut oBE = 0.0;
                let A = parameters[108] + 2.7315e2f64;
                let D = (B * A) / C;
                let E = parameters[88] * A;
                let I = H * (F + G);
                let K = H * (F + J);
                let L = H * (parameters[79] + J);
                let M = 3e0f64 - ((C * parameters[80]) / B);
                let P = (M + N) - O;
                let Q = M - 1.5e0f64;
                let R = (parameters[82] - parameters[81]) - H;
                let S = F - G;
                let U = if (if parameters[21] > T { 1.0 } else { 0.0 }) != 0.0 && (if parameters[41] > T { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let V = if U != 0.0 {
                    N
                } else {
                    T
                };
                let W = (H * parameters[35]) / D;
                let X = 2e0f64 * D;
                let Y = X * (((W.exp()) - ((-W).exp())).ln());
                let Z = (H * parameters[38]) / D;
                let AA = X * (((Z.exp()) - ((-Z).exp())).ln());
                let AB = H * M;
                let AC = H * I;
                let AD = (H * parameters[42]) / D;
                let AE = X * (((AD.exp()) - ((-AD).exp())).ln());
                let AF = O - E;
                let AG = if parameters[65] > T { 1.0 } else { 0.0 };
                let AH = if parameters[96] == N { 1.0 } else { 0.0 };
                let AI = O - N;
                let AJ = if V == N { 1.0 } else { 0.0 };
                let AK = (H * parameters[46]) / D;
                let AL = X * (((AK.exp()) - ((-AK).exp())).ln());
                let AM = (H * parameters[51]) / D;
                let AN = X * (((AM.exp()) - ((-AM).exp())).ln());
                let AO = if parameters[0] <= 2e2f64 { 1.0 } else { 0.0 };
                let AS = if AQ >= AR { 1.0 } else { 0.0 };
                let AT = if (if AP != T { 1.0 } else { 0.0 }) != 0.0 && AS != 0.0 { 1.0 } else { 0.0 };
                let AV = if parameters[44] < AU { 1.0 } else { 0.0 };
                let AX = if AO != 0.0 {
                    AZ
                } else {
                    AW
                };
                let AY = if parameters[7] == T { 1.0 } else { 0.0 };
                let BA = if parameters[10] == N { 1.0 } else { 0.0 };
                let BB = if parameters[13] != T { 1.0 } else { 0.0 };
                let BC = if parameters[2] == T { 1.0 } else { 0.0 };
                if BC != 0.0 {
                } else {
                    let BE = if (if parameters[9] == BD { 1.0 } else { 0.0 }) != 0.0 && (if parameters[12] == BD { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBE = BE;
                }
                let BF = N + ((N + parameters[60]).sqrt());
                let BG = parameters[58] + N;
                let BH = if parameters[15] > T { 1.0 } else { 0.0 };
                let BI = if parameters[17] > T { 1.0 } else { 0.0 };
                let BJ = if parameters[19] > T { 1.0 } else { 0.0 };
                let BK = if parameters[30] > T { 1.0 } else { 0.0 };
                let BL = if parameters[32] > T { 1.0 } else { 0.0 };
                let BM = if parameters[53] < AU { 1.0 } else { 0.0 };
                let BN = if (if AP == N { 1.0 } else { 0.0 }) != 0.0 && AS != 0.0 { 1.0 } else { 0.0 };
                let BO = if (if parameters[73] != T { 1.0 } else { 0.0 }) != 0.0 && (if parameters[54] != T { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BP = if parameters[28] >= AR { 1.0 } else { 0.0 };
                let BR = if BP != 0.0 {
                    T
                } else {
                    BQ
                };
                let BS = if parameters[29] >= AR { 1.0 } else { 0.0 };
                let BU = if BS != 0.0 {
                    T
                } else {
                    BT
                };
                let BV = if (if parameters[23] >= AR { 1.0 } else { 0.0 }) != 0.0 || (if parameters[26] >= AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BX = if BV != 0.0 {
                    T
                } else {
                    BW
                };
                let BY = if AP == T { 1.0 } else { 0.0 };
                let BZ = if BY != 0.0 || (if parameters[107] == T { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CA = if BY != 0.0 || (if AQ < AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CC = if CA != 0.0 {
                    CB
                } else {
                    T
                };
                let CE = if BV != 0.0 {
                    CD
                } else {
                    T
                };
                let CG = if BS != 0.0 {
                    CF
                } else {
                    T
                };
                let CI = if BP != 0.0 {
                    CH
                } else {
                    T
                };
            [A, I, K, L, M, P, Q, R, S, U, Y, AA, AB, AC, AE, AF, AG, AH, AI, AJ, AL, AN, AO, AT, AV, AY, AX, BA, BB, BC, oBE, BF, BG, BH, BI, BJ, BK, BL, BM, BN, BO, BP, BS, BV, BZ, CA, CE, CG, BR, BU, BX, CC, CI]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
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
        let produced: [f64; 42] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let C = 1.7314999999999998e2f64;
                let G = 1e0f64;
                let I = staged[0];
                let S = 4e0f64;
                let T = 5e-1f64;
                let V = parameters[35];
                let W = parameters[34];
                let AB = parameters[38];
                let AN = parameters[83];
                let AR = parameters[64];
                let AT = staged[26];
                let AU = 6e2f64;
                let AW = parameters[65];
                let BB = staged[27];
                let BC = staged[12];
                let BD = parameters[57];
                let BH = staged[28];
                let BI = parameters[21];
                let BK = parameters[22];
                let CA = staged[29];
                let mut oD = 0.0;
                let A = temperature + parameters[109];
                let B = if A < 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let E;
                if B != 0.0 {
                    E = C;
                } else {
                    let D = if A > 6e2f64 { 1.0 } else { 0.0 };
                    oD = D;
                    let AV = if D != 0.0 {
                        AU
                    } else {
                        A
                    };
                    E = AV;
                }
                let F = (1.3806226e-23f64 * E) / 1.602176462e-19f64;
                let H = G / F;
                let J = E - I;
                let K = E / I;
                let L = K.ln();
                let M = H * (K - G);
                let N = G - K;
                let O = staged[2] * N;
                let P = (staged[3] * F) * L;
                let Q = ((staged[1] * K) + O) - P;
                let R = 2e0f64 * F;
                let U = Q + (R * ((T * (G + ((G + (S * (((-Q) * H).exp()))).sqrt()))).ln()));
                let X = W * ((parameters[36] * ((V / U).ln())).exp());
                let Y = (parameters[37] * U) / V;
                let Z = ((staged[4] * K) + O) - P;
                let AA = Z + (R * ((T * (G + ((G + (S * (((-Z) * H).exp()))).sqrt()))).ln()));
                let AC = W * ((parameters[39] * ((AB / AA).ln())).exp());
                let AD = (parameters[40] * AA) / AB;
                let AE = parameters[15] * (((parameters[82] * L) + (parameters[77] * M)).exp());
                let AF = parameters[17] * (((staged[5] * L) + (staged[6] * M)).exp());
                let AG = staged[8] * N;
                let AH = ((staged[7] * K) + AG) - P;
                let AI = AH + (R * ((T * (G + ((G + (S * (((-AH) * H).exp()))).sqrt()))).ln()));
                let AJ = parameters[41] * ((parameters[43] * ((parameters[42] / AI).ln())).exp());
                let AK = parameters[78] * M;
                let AL = parameters[19] * (((staged[9] * L) + AK).exp());
                let AM = parameters[1] * (((parameters[81] * L) + (parameters[76] * M)).exp());
                let AO = parameters[9] * (((parameters[95] * L) - (AN * M)).exp());
                let AP = parameters[62] * ((staged[10] * L).exp());
                let AQ = G / (parameters[61] * ((parameters[87] * L).exp()));
                let AS = AR * (G + (parameters[89] * J));
                let AY;
                let AZ;
                if AT != 0.0 {
                    let AX = AW * (G - (parameters[90] * J));
                    AY = AX;
                    AZ = AR;
                } else {
                    AY = AW;
                    AZ = AS;
                }
                let BA = parameters[54] * ((G + (parameters[85] * J)) + ((parameters[86] * J) * J));
                let BF = if BB != 0.0 {
                    let BE = BD * (((staged[11] * L) - (BC * M)).exp());
                    BE
                } else {
                    BD
                };
                let BG = parameters[59] * ((staged[13] * L).exp());
                let BM;
                let BN;
                if BH != 0.0 {
                    let BJ = BI * ((parameters[99] * J).exp());
                    let BL = BK * ((parameters[100] * J).exp());
                    BM = BL;
                    BN = BJ;
                } else {
                    BM = BK;
                    BN = BI;
                }
                let BO = parameters[23] * ((parameters[91] * L).exp());
                let BP = ((staged[14] * K) + AG) - P;
                let BQ = BP + (R * ((T * (G + ((G + (S * (((-BP) * H).exp()))).sqrt()))).ln()));
                let BR = parameters[45] * ((parameters[47] * ((parameters[46] / BQ).ln())).exp());
                let BS = ((staged[15] * K) + (staged[16] * N)) - P;
                let BT = BS + (R * ((T * (G + ((G + (S * (((-BS) * H).exp()))).sqrt()))).ln()));
                let BU = parameters[50] * ((parameters[52] * ((parameters[51] / BT).ln())).exp());
                let BV = staged[17] * L;
                let BW = parameters[32] * ((BV + (parameters[79] * M)).exp());
                let BX = parameters[30] * ((BV + AK).exp());
                let BY = parameters[7] * ((parameters[97] * L).exp());
                let BZ = parameters[6] / (((AN * H) * (((parameters[84] * L).exp()) - G)).exp());
                let CD = if CA != 0.0 {
                    let CB = G + (J * (parameters[101] + (parameters[102] * J)));
                    CB
                } else {
                    let CC = (parameters[98] * L).exp();
                    CC
                };
                let CE = parameters[12] * CD;
                let CF = (parameters[13] * CD) * ((BC * M).exp());
                let CG = parameters[29] * ((parameters[93] * L).exp());
                let CH = parameters[26] * ((parameters[92] * L).exp());
                let CI = parameters[28] * ((parameters[94] * L).exp());
                let CJ = (parameters[104] * ((parameters[105] * L).exp())) * (G + (parameters[106] * J));
            [A, B, oD, F, H, U, X, Y, AA, AC, AD, AE, AF, AI, AJ, AL, AM, AO, AP, AQ, BA, BG, BO, BQ, BR, BT, BU, BW, BX, BY, BZ, CE, CF, CG, CH, CI, CJ, AY, AZ, BF, BM, BN]
        };
        self.canonical_staged[18] = produced[0];
        self.canonical_staged[24] = produced[1];
        self.canonical_staged[25] = produced[2];
        self.canonical_staged[35] = produced[3];
        self.canonical_staged[34] = produced[4];
        self.canonical_staged[42] = produced[5];
        self.canonical_staged[41] = produced[6];
        self.canonical_staged[43] = produced[7];
        self.canonical_staged[45] = produced[8];
        self.canonical_staged[44] = produced[9];
        self.canonical_staged[46] = produced[10];
        self.canonical_staged[56] = produced[11];
        self.canonical_staged[57] = produced[12];
        self.canonical_staged[33] = produced[13];
        self.canonical_staged[32] = produced[14];
        self.canonical_staged[58] = produced[15];
        self.canonical_staged[51] = produced[16];
        self.canonical_staged[50] = produced[17];
        self.canonical_staged[39] = produced[18];
        self.canonical_staged[40] = produced[19];
        self.canonical_staged[49] = produced[20];
        self.canonical_staged[54] = produced[21];
        self.canonical_staged[61] = produced[22];
        self.canonical_staged[36] = produced[23];
        self.canonical_staged[31] = produced[24];
        self.canonical_staged[66] = produced[25];
        self.canonical_staged[65] = produced[26];
        self.canonical_staged[64] = produced[27];
        self.canonical_staged[63] = produced[28];
        self.canonical_staged[47] = produced[29];
        self.canonical_staged[48] = produced[30];
        self.canonical_staged[53] = produced[31];
        self.canonical_staged[52] = produced[32];
        self.canonical_staged[68] = produced[33];
        self.canonical_staged[62] = produced[34];
        self.canonical_staged[67] = produced[35];
        self.canonical_staged[69] = produced[36];
        self.canonical_staged[37] = produced[37];
        self.canonical_staged[38] = produced[38];
        self.canonical_staged[55] = produced[39];
        self.canonical_staged[59] = produced[40];
        self.canonical_staged[60] = produced[41];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 12300 => 0usize, 12302 => 1usize, 12304 => 2usize, 12306 => 3usize, 12334 => 4usize, 12337 => 5usize, 12350 => 6usize, 12367 => 7usize, 12370 => 8usize, _ => usize::MAX };
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
            let A = node_potentials[1];
            let B = node_potentials[5];
            let C = 1e0f64;
            let D = 1e0f64;
            let E = parameters[110];
            let H = node_potentials[6];
            let J = 1e0f64;
            let N = node_potentials[7];
            let P = 1e0f64;
            let V = 1e0f64;
            let Y = node_potentials[2];
            let Z = 1e0f64;
            let AF = 1e0f64;
            let AJ = staged[26];
            let AK = staged[27];
            let AL = staged[28];
            let AM = staged[29];
            let AN = staged[30];
            let AO = node_potentials[4];
            let AR = staged[31];
            let AS = staged[32];
            let AT = staged[33];
            let AU = staged[34];
            let AV = staged[35];
            let AW = staged[36];
            let AX = staged[37];
            let AY = staged[38];
            let AZ = staged[39];
            let BA = staged[40];
            let BB = staged[41];
            let BC = staged[42];
            let BD = staged[43];
            let BE = staged[44];
            let BF = staged[45];
            let BG = staged[46];
            let BH = staged[47];
            let BI = staged[48];
            let BJ = staged[49];
            let BK = staged[50];
            let BL = staged[51];
            let BM = staged[52];
            let BN = staged[53];
            let BO = staged[54];
            let BP = staged[55];
            let BQ = staged[56];
            let BR = staged[57];
            let BS = staged[58];
            let BT = staged[59];
            let BU = staged[60];
            let BV = staged[61];
            let BW = staged[62];
            let BX = staged[63];
            let BY = staged[64];
            let BZ = staged[65];
            let CA = staged[66];
            let CB = staged[67];
            let CC = staged[68];
            let CD = staged[69];
            let CE = 0e0f64;
            let FG = 1.7314999999999998e2f64;
            let FK = 1.3806226e-23f64;
            let FL = 1.602176462e-19f64;
            let FO = 1e0f64;
            let FQ = -1e0f64;
            let FS = staged[0];
            let FX = 1e0f64;
            let GC = staged[1];
            let GF = staged[2];
            let GI = staged[3];
            let GO = 2e0f64;
            let GT = 4e0f64;
            let GV = 2e0f64;
            let GW = 5e-1f64;
            let HB = parameters[35];
            let HD = parameters[36];
            let HF = parameters[34];
            let HI = parameters[37];
            let HL = staged[4];
            let HV = parameters[38];
            let HX = parameters[39];
            let IB = parameters[40];
            let IE = parameters[82];
            let IF = parameters[77];
            let IH = parameters[15];
            let IK = staged[5];
            let IL = staged[6];
            let IN = parameters[17];
            let IQ = staged[7];
            let IR = staged[8];
            let JE = parameters[43];
            let JG = parameters[41];
            let JJ = staged[9];
            let JK = parameters[78];
            let JO = parameters[19];
            let JR = parameters[81];
            let JS = parameters[76];
            let JU = parameters[1];
            let JX = parameters[95];
            let JY = parameters[83];
            let KA = parameters[9];
            let KD = staged[10];
            let KF = parameters[62];
            let KI = parameters[87];
            let KK = parameters[61];
            let KO = parameters[89];
            let KP = parameters[64];
            let KS = 6e2f64;
            let KT = 1e0f64;
            let KW = parameters[90];
            let KX = parameters[65];
            let LE = parameters[85];
            let LF = parameters[86];
            let LH = parameters[54];
            let LK = staged[11];
            let LL = staged[12];
            let LN = parameters[57];
            let LS = staged[13];
            let LU = parameters[59];
            let LX = parameters[99];
            let LZ = parameters[21];
            let MC = parameters[100];
            let ME = parameters[22];
            let ML = parameters[91];
            let MN = parameters[23];
            let MQ = staged[14];
            let NB = parameters[47];
            let ND = parameters[45];
            let NG = staged[15];
            let NH = staged[16];
            let NS = parameters[52];
            let NU = parameters[50];
            let NX = staged[17];
            let OA = parameters[79];
            let OC = parameters[32];
            let OG = parameters[30];
            let OJ = parameters[97];
            let OL = parameters[7];
            let OP = parameters[84];
            let OV = parameters[102];
            let OZ = parameters[98];
            let PE = parameters[12];
            let PH = parameters[13];
            let PM = parameters[93];
            let PO = parameters[29];
            let PR = parameters[92];
            let PT = parameters[26];
            let PW = parameters[94];
            let PY = parameters[28];
            let QB = parameters[105];
            let QD = parameters[104];
            let QF = parameters[106];
            let QJ = parameters[49];
            let QP = parameters[44];
            let QQ = 1e2f64;
            let QU = parameters[48];
            let RC = staged[70];
            let RD = 0e0f64;
            let RI = Lanes([0e0f64; 3]);
            let RP = 2.4e0f64;
            let SC = 8e1f64;
            let SE = Lanes([0e0f64; 3]);
            let SO = 1e-1f64;
            let UD = 1.921812e0f64;
            let AGJ = parameters[67];
            let AGO = parameters[63];
            let AIC = Lanes([0e0f64; 3]);
            let AIM = staged[71];
            let AJN = parameters[8];
            let AJY = staged[19];
            let AKI = parameters[5];
            let AKK = 2e1f64;
            let AKP = 2.5e-2f64;
            let AKS = parameters[55];
            let AKU = parameters[56];
            let AKX = staged[72];
            let ALM = parameters[3];
            let AMB = parameters[4];
            let AMQ = staged[73];
            let AMU = parameters[11];
            let ANF = 6.666e-1f64;
            let AOT = 1e-2f64;
            let AOV = 2.004987562112089e0f64;
            let AOY = Lanes([0e0f64; 4]);
            let APB = staged[74];
            let APC = -2e0f64;
            let APF = staged[75];
            let API = 1e-20f64;
            let ARD = 3.333333333333333e-1f64;
            let ARH = 2.7e1f64;
            let ARL = 2.5e-1f64;
            let ART = 3e0f64;
            let ATY = -4e0f64;
            let AUV = staged[20];
            let AVB = parameters[58];
            let AVE = staged[21];
            let AVH = parameters[68];
            let AVK = staged[76];
            let AVL = parameters[16];
            let AVS = staged[77];
            let AWC = parameters[18];
            let AWL = staged[78];
            let AWV = parameters[20];
            let BCJ = parameters[24];
            let BCL = parameters[25];
            let BDB = staged[79];
            let BDC = parameters[27];
            let BDT = parameters[31];
            let BEF = Lanes([0e0f64; 4]);
            let BEI = staged[80];
            let BEJ = parameters[33];
            let BEO = Lanes([0e0f64; 3]);
            let BER = staged[81];
            let BFF = staged[82];
            let BFH = parameters[53];
            let BIS = staged[83];
            let BIT = node_potentials[8];
            let BIV = 1e0f64;
            let BIY = parameters[71];
            let BJB = node_potentials[9];
            let BJD = 1e0f64;
            let BJG = parameters[72];
            let BJN = 0e0f64;
            let BJO = 0e0f64;
            let BKB = parameters[70];
            let BKC = parameters[69];
            let BLI = ddt_scale();
            let BLQ = staged[84];
            let BLT = Lanes([0e0f64; 3]);
            let BLW = staged[85];
            let BLZ = Lanes([0e0f64; 3]);
            let BMC = staged[86];
            let BMG = Lanes([0e0f64; 5]);
            let BMN = staged[87];
            let BMO = parameters[107];
            let BMX = staged[88];
            let BRJ = 0e0f64;
            let BRK = 0e0f64;
            let BRL = 0e0f64;
            let F = E * (A - B);
            let G = (Lanes([C, 0.0]) - Lanes([0.0, D])) * E;
            let I = H - B;
            let K = Lanes([0.0, J]) - Lanes([D, 0.0]);
            let L = E * I;
            let M = K * E;
            let O = H - N;
            let Q = Lanes([J, 0.0]) - Lanes([0.0, P]);
            let R = E * O;
            let S = Q * E;
            let T = R - L;
            let U = Lanes([0.0, S[0], S[1]]) - Lanes([M[0], M[1], 0.0]);
            let W = E * (node_potentials[3] - B);
            let X = (Lanes([V, 0.0]) - Lanes([0.0, D])) * E;
            let AA = E * (A - Y);
            let AB = (Lanes([C, 0.0]) - Lanes([0.0, Z])) * E;
            let AC = N - Y;
            let AD = Lanes([0.0, P]) - Lanes([Z, 0.0]);
            let AE = B - node_potentials[0];
            let AG = Lanes([0.0, D]) - Lanes([AF, 0.0]);
            let AH = A - H;
            let AI = Lanes([C, 0.0]) - Lanes([0.0, J]);
            let CF;
            let CG;
            let CH;
            let CI;
            let CJ;
            let CK;
            let CL;
            let CM;
            let CN;
            let CO;
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
            let DN;
            let DO;
            let DP;
            let DQ;
            let DR;
            let DS;
            let DT;
            let DU;
            let DV;
            let DW;
            let DX;
            let DY;
            let DZ;
            let EA;
            let EB;
            let EC;
            let ED;
            let EE;
            let EF;
            let EG;
            let EH;
            let EI;
            let EJ;
            let EK;
            let EL;
            let EM;
            let EN;
            let EO;
            let EP;
            let EQ;
            let ER;
            let ES;
            let ET;
            let EU;
            let EV;
            let EW;
            let EX;
            let EY;
            let EZ;
            let FA;
            let FB;
            let FC;
            let FD;
            let FE;
            if AN != 0.0 {
                let AP = staged[18] + AO;
                let AQ = if AP < 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let FI;
                let FJ;
                if AQ != 0.0 {
                    FI = FG;
                    FJ = CE;
                } else {
                    let FH = if AP > 6e2f64 { 1.0 } else { 0.0 };
                    let KU;
                    let KV;
                    if FH != 0.0 {
                        KU = KS;
                        KV = CE;
                    } else {
                        KU = AP;
                        KV = KT;
                    }
                    FI = KU;
                    FJ = KV;
                }
                let FM = (FK * FI) / FL;
                let FN = (FJ * FK) / FL;
                let FP = FO / FM;
                let FR = ((FN * FP) * FQ) / FM;
                let FT = FI - FS;
                let FU = FI / FS;
                let FV = FJ / FS;
                let FW = FU.ln();
                let FY = FV * (FX / FU);
                let FZ = FU - FO;
                let GA = FP * FZ;
                let GB = (FR * FZ) + (FV * FP);
                let GD = FO - FU;
                let GE = FV * FQ;
                let GG = GF * GD;
                let GH = GE * GF;
                let GJ = GI * FM;
                let GK = GJ * FW;
                let GL = ((FN * GI) * FW) + (FY * GJ);
                let GM = ((GC * FU) + GG) - GK;
                let GN = ((FV * GC) + GH) - GL;
                let GP = GO * FM;
                let GQ = FN * GO;
                let GR = -GM;
                let GS = (GR * FP).exp();
                let GU = (FO + (GT * GS)).sqrt();
                let GX = GW * (FO + GU);
                let GY = GX.ln();
                let GZ = GM + (GP * GY);
                let HA = GN + ((GQ * GY) + (((((((((GN * FQ) * FP) + (FR * GR)) * GS) * GT) * (FX / (GV * GU))) * GW) * (FX / GX)) * GP));
                let HC = HB / GZ;
                let HE = (HD * (HC.ln())).exp();
                let HG = HF * HE;
                let HH = ((((((HA * HC) * FQ) / GZ) * (FX / HC)) * HD) * HE) * HF;
                let HJ = (HI * GZ) / HB;
                let HK = (HA * HI) / HB;
                let HM = ((HL * FU) + GG) - GK;
                let HN = ((FV * HL) + GH) - GL;
                let HO = -HM;
                let HP = (HO * FP).exp();
                let HQ = (FO + (GT * HP)).sqrt();
                let HR = GW * (FO + HQ);
                let HS = HR.ln();
                let HT = HM + (GP * HS);
                let HU = HN + ((GQ * HS) + (((((((((HN * FQ) * FP) + (FR * HO)) * HP) * GT) * (FX / (GV * HQ))) * GW) * (FX / HR)) * GP));
                let HW = HV / HT;
                let HY = (HX * (HW.ln())).exp();
                let HZ = HF * HY;
                let IA = ((((((HU * HW) * FQ) / HT) * (FX / HW)) * HX) * HY) * HF;
                let IC = (IB * HT) / HV;
                let ID = (HU * IB) / HV;
                let IG = ((IE * FW) + (IF * GA)).exp();
                let II = IH * IG;
                let IJ = (((FY * IE) + (GB * IF)) * IG) * IH;
                let IM = ((IK * FW) + (IL * GA)).exp();
                let IO = IN * IM;
                let IP = (((FY * IK) + (GB * IL)) * IM) * IN;
                let IS = IR * GD;
                let IT = GE * IR;
                let IU = ((IQ * FU) + IS) - GK;
                let IV = ((FV * IQ) + IT) - GL;
                let IW = -IU;
                let IX = (IW * FP).exp();
                let IY = (FO + (GT * IX)).sqrt();
                let IZ = GW * (FO + IY);
                let JA = IZ.ln();
                let JB = IU + (GP * JA);
                let JC = IV + ((GQ * JA) + (((((((((IV * FQ) * FP) + (FR * IW)) * IX) * GT) * (FX / (GV * IY))) * GW) * (FX / IZ)) * GP));
                let JD = parameters[42] / JB;
                let JF = (JE * (JD.ln())).exp();
                let JH = JG * JF;
                let JI = ((((((JC * JD) * FQ) / JB) * (FX / JD)) * JE) * JF) * JG;
                let JL = JK * GA;
                let JM = GB * JK;
                let JN = ((JJ * FW) + JL).exp();
                let JP = JO * JN;
                let JQ = (((FY * JJ) + JM) * JN) * JO;
                let JT = ((JR * FW) + (JS * GA)).exp();
                let JV = JU * JT;
                let JW = (((FY * JR) + (GB * JS)) * JT) * JU;
                let JZ = ((JX * FW) - (JY * GA)).exp();
                let KB = KA * JZ;
                let KC = (((FY * JX) - (GB * JY)) * JZ) * KA;
                let KE = (KD * FW).exp();
                let KG = KF * KE;
                let KH = ((FY * KD) * KE) * KF;
                let KJ = (KI * FW).exp();
                let KL = KK * KJ;
                let KM = FO / KL;
                let KN = (((((FY * KI) * KJ) * KK) * KM) * FQ) / KL;
                let KQ = KP * (FO + (KO * FT));
                let KR = (FJ * KO) * KP;
                let LA;
                let LB;
                let LC;
                let LD;
                if AJ != 0.0 {
                    let KY = KX * (FO - (KW * FT));
                    let KZ = ((FJ * KW) * FQ) * KX;
                    LA = KY;
                    LB = KP;
                    LC = KZ;
                    LD = CE;
                } else {
                    LA = KX;
                    LB = KQ;
                    LC = CE;
                    LD = KR;
                }
                let LG = LF * FT;
                let LI = LH * ((FO + (LE * FT)) + (LG * FT));
                let LJ = ((FJ * LE) + (((FJ * LF) * FT) + (FJ * LG))) * LH;
                let LQ;
                let LR;
                if AK != 0.0 {
                    let LM = ((LK * FW) - (LL * GA)).exp();
                    let LO = LN * LM;
                    let LP = (((FY * LK) - (GB * LL)) * LM) * LN;
                    LQ = LO;
                    LR = LP;
                } else {
                    LQ = LN;
                    LR = CE;
                }
                let LT = (LS * FW).exp();
                let LV = LU * LT;
                let LW = ((FY * LS) * LT) * LU;
                let MH;
                let MI;
                let MJ;
                let MK;
                if AL != 0.0 {
                    let LY = (LX * FT).exp();
                    let MA = LZ * LY;
                    let MB = ((FJ * LX) * LY) * LZ;
                    let MD = (MC * FT).exp();
                    let MF = ME * MD;
                    let MG = ((FJ * MC) * MD) * ME;
                    MH = MF;
                    MI = MA;
                    MJ = MG;
                    MK = MB;
                } else {
                    MH = ME;
                    MI = LZ;
                    MJ = CE;
                    MK = CE;
                }
                let MM = (ML * FW).exp();
                let MO = MN * MM;
                let MP = ((FY * ML) * MM) * MN;
                let MR = ((MQ * FU) + IS) - GK;
                let MS = ((FV * MQ) + IT) - GL;
                let MT = -MR;
                let MU = (MT * FP).exp();
                let MV = (FO + (GT * MU)).sqrt();
                let MW = GW * (FO + MV);
                let MX = MW.ln();
                let MY = MR + (GP * MX);
                let MZ = MS + ((GQ * MX) + (((((((((MS * FQ) * FP) + (FR * MT)) * MU) * GT) * (FX / (GV * MV))) * GW) * (FX / MW)) * GP));
                let NA = parameters[46] / MY;
                let NC = (NB * (NA.ln())).exp();
                let NE = ND * NC;
                let NF = ((((((MZ * NA) * FQ) / MY) * (FX / NA)) * NB) * NC) * ND;
                let NI = ((NG * FU) + (NH * GD)) - GK;
                let NJ = ((FV * NG) + (GE * NH)) - GL;
                let NK = -NI;
                let NL = (NK * FP).exp();
                let NM = (FO + (GT * NL)).sqrt();
                let NN = GW * (FO + NM);
                let NO = NN.ln();
                let NP = NI + (GP * NO);
                let NQ = NJ + ((GQ * NO) + (((((((((NJ * FQ) * FP) + (FR * NK)) * NL) * GT) * (FX / (GV * NM))) * GW) * (FX / NN)) * GP));
                let NR = parameters[51] / NP;
                let NT = (NS * (NR.ln())).exp();
                let NV = NU * NT;
                let NW = ((((((NQ * NR) * FQ) / NP) * (FX / NR)) * NS) * NT) * NU;
                let NY = NX * FW;
                let NZ = FY * NX;
                let OB = (NY + (OA * GA)).exp();
                let OD = OC * OB;
                let OE = ((NZ + (GB * OA)) * OB) * OC;
                let OF = (NY + JL).exp();
                let OH = OG * OF;
                let OI = ((NZ + JM) * OF) * OG;
                let OK = (OJ * FW).exp();
                let OM = OL * OK;
                let ON = ((FY * OJ) * OK) * OL;
                let OO = JY * FP;
                let OQ = (OP * FW).exp();
                let OR = OQ - FO;
                let OS = (OO * OR).exp();
                let OT = parameters[6] / OS;
                let OU = ((((((FR * JY) * OR) + (((FY * OP) * OQ) * OO)) * OS) * OT) * FQ) / OS;
                let PC;
                let PD;
                if AM != 0.0 {
                    let OW = parameters[101] + (OV * FT);
                    let OX = (FJ * OW) + ((FJ * OV) * FT);
                    let OY = FO + (FT * OW);
                    PC = OY;
                    PD = OX;
                } else {
                    let PA = (OZ * FW).exp();
                    let PB = (FY * OZ) * PA;
                    PC = PA;
                    PD = PB;
                }
                let PF = PE * PC;
                let PG = PD * PE;
                let PI = PH * PC;
                let PJ = (LL * GA).exp();
                let PK = PI * PJ;
                let PL = ((PD * PH) * PJ) + (((GB * LL) * PJ) * PI);
                let PN = (PM * FW).exp();
                let PP = PO * PN;
                let PQ = ((FY * PM) * PN) * PO;
                let PS = (PR * FW).exp();
                let PU = PT * PS;
                let PV = ((FY * PR) * PS) * PT;
                let PX = (PW * FW).exp();
                let PZ = PY * PX;
                let QA = ((FY * PW) * PX) * PY;
                let QC = (QB * FW).exp();
                let QE = QD * QC;
                let QG = FO + (QF * FT);
                let QH = QE * QG;
                let QI = ((((FY * QB) * QC) * QD) * QG) + ((FJ * QF) * QE);
                CF = NE;
                CG = JH;
                CH = JB;
                CI = FP;
                CJ = FM;
                CK = MY;
                CL = LA;
                CM = LB;
                CN = KG;
                CO = KM;
                CP = HG;
                CQ = GZ;
                CR = HJ;
                CS = HZ;
                CT = HT;
                CU = IC;
                CV = OM;
                CW = OT;
                CX = LI;
                CY = KB;
                CZ = JV;
                DA = PK;
                DB = PF;
                DC = LV;
                DD = LQ;
                DE = II;
                DF = IO;
                DG = JP;
                DH = MH;
                DI = MI;
                DJ = MO;
                DK = PU;
                DL = OH;
                DM = OD;
                DN = NV;
                DO = NP;
                DP = PZ;
                DQ = PP;
                DR = QH;
                DS = NF;
                DT = JI;
                DU = JC;
                DV = FR;
                DW = FN;
                DX = MZ;
                DY = LC;
                DZ = LD;
                EA = KH;
                EB = KN;
                EC = HH;
                ED = HA;
                EE = HK;
                EF = IA;
                EG = HU;
                EH = ID;
                EI = ON;
                EJ = OU;
                EK = LJ;
                EL = KC;
                EM = JW;
                EN = PL;
                EO = PG;
                EP = LW;
                EQ = LR;
                ER = IJ;
                ES = IP;
                ET = JQ;
                EU = MJ;
                EV = MK;
                EW = MP;
                EX = PV;
                EY = OI;
                EZ = OE;
                FA = NW;
                FB = NQ;
                FC = QA;
                FD = PQ;
                FE = QI;
            } else {
                CF = AR;
                CG = AS;
                CH = AT;
                CI = AU;
                CJ = AV;
                CK = AW;
                CL = AX;
                CM = AY;
                CN = AZ;
                CO = BA;
                CP = BB;
                CQ = BC;
                CR = BD;
                CS = BE;
                CT = BF;
                CU = BG;
                CV = BH;
                CW = BI;
                CX = BJ;
                CY = BK;
                CZ = BL;
                DA = BM;
                DB = BN;
                DC = BO;
                DD = BP;
                DE = BQ;
                DF = BR;
                DG = BS;
                DH = BT;
                DI = BU;
                DJ = BV;
                DK = BW;
                DL = BX;
                DM = BY;
                DN = BZ;
                DO = CA;
                DP = CB;
                DQ = CC;
                DR = CD;
                DS = CE;
                DT = CE;
                DU = CE;
                DV = CE;
                DW = CE;
                DX = CE;
                DY = CE;
                DZ = CE;
                EA = CE;
                EB = CE;
                EC = CE;
                ED = CE;
                EE = CE;
                EF = CE;
                EG = CE;
                EH = CE;
                EI = CE;
                EJ = CE;
                EK = CE;
                EL = CE;
                EM = CE;
                EN = CE;
                EO = CE;
                EP = CE;
                EQ = CE;
                ER = CE;
                ES = CE;
                ET = CE;
                EU = CE;
                EV = CE;
                EW = CE;
                EX = CE;
                EY = CE;
                EZ = CE;
                FA = CE;
                FB = CE;
                FC = CE;
                FD = CE;
                FE = CE;
            }
            let FF = if CF <= 1e-30f64 { 1.0 } else { 0.0 };
            let QW;
            let QX;
            let QY;
            let QZ;
            let RA;
            let RB;
            if FF != 0.0 {
                let QK = CG * QJ;
                let QL = DT * QJ;
                let QM = FO - QJ;
                let QN = CG * QM;
                let QO = DT * QM;
                let QR = if QP < QQ { 1.0 } else { 0.0 };
                let RG;
                let RH;
                if QR != 0.0 {
                    let RE = if QN > RD { 1.0 } else { 0.0 };
                    let SF;
                    let SG;
                    if RE != 0.0 {
                        let RJ = JE / GT;
                        let RK = QP - CH;
                        let RL = DU * FQ;
                        let RM = FO - ((-8.754687373538999e-1f64 / JE).exp());
                        let RN = CH * RM;
                        let RO = DU * RM;
                        let RQ = RP * QN;
                        let RR = QO * RP;
                        let RS = RJ - JE;
                        let RT = QP / CH;
                        let RU = (RS * (RT.ln())).exp();
                        let RV = QN * RU;
                        let RW = (QO * RU) + (((((((DU * RT) * FQ) / CH) * (FX / RT)) * RS) * RU) * QN);
                        let RX = RN - F;
                        let RY = Lanes([0.0, RO, 0.0]);
                        let RZ = Lanes([G[0], 0.0, G[1]]);
                        let SA = RX * CI;
                        let SB = ((RY - RZ) * CI) + Lanes([0.0, (DV * RX), 0.0]);
                        let SD = if SA < SC { 1.0 } else { 0.0 };
                        let SM;
                        let SN;
                        if SD != 0.0 {
                            let SH = SA.exp();
                            let SI = FO + SH;
                            let SJ = SI.ln();
                            let SK = RN - (CJ * SJ);
                            let SL = RY - (Lanes([0.0, (DW * SJ), 0.0]) + (((SB * SH) * (FX / SI)) * CJ));
                            SM = SK;
                            SN = SL;
                        } else {
                            SM = F;
                            SN = RZ;
                        }
                        let SP = (SO * RK) + (GT * CJ);
                        let SQ = (RL * SO) + (DW * GT);
                        let SR = (RK + SM) / SP;
                        let SS = ((Lanes([0.0, RL, 0.0]) + SN) - Lanes([0.0, (SQ * SR), 0.0])) / SP;
                        let ST = if SR < SC { 1.0 } else { 0.0 };
                        let TB;
                        let TC;
                        if ST != 0.0 {
                            let SU = SR.exp();
                            let SV = FO + SU;
                            let SW = (-(RK + RN)) / SP;
                            let SX = SW.exp();
                            let SY = (SV.ln()) - SX;
                            let SZ = (-RK) + (SP * SY);
                            let TA = Lanes([0.0, (RL * FQ), 0.0]) + (Lanes([0.0, (SQ * SY), 0.0]) + ((((SS * SU) * (FX / SV)) - Lanes([0.0, (((((RL + RO) * FQ) - (SQ * SW)) / SP) * SX), 0.0])) * SP));
                            TB = SZ;
                            TC = TA;
                        } else {
                            TB = SM;
                            TC = SN;
                        }
                        let TD = F - SM;
                        let TE = SM / CH;
                        let TF = FO - TE;
                        let TG = TB / CH;
                        let TH = FO - TG;
                        let TI = TH.ln();
                        let TJ = (((TC - Lanes([0.0, (DU * TG), 0.0])) / CH) * FQ) * (FX / TH);
                        let TK = FO - JE;
                        let TL = FO - RJ;
                        let TM = (TI * TK).exp();
                        let TN = FO - TM;
                        let TO = ((TF.ln()) * TL).exp();
                        let TP = FO - TO;
                        let TQ = (TI * TL).exp();
                        let TR = FO - TQ;
                        let TS = (((QN * TN) / TK) + ((RV * TP) / TL)) - ((RV * TR) / TL);
                        let TT = (TS * CH) + (RQ * TD);
                        let TU = ((((((Lanes([0.0, (QO * TN), 0.0]) + ((((TJ * TK) * TM) * FQ) * QN)) / TK) + ((Lanes([0.0, (RW * TP), 0.0]) + ((((((((SN - Lanes([0.0, (DU * TE), 0.0])) / CH) * FQ) * (FX / TF)) * TL) * TO) * FQ) * RV)) / TL)) - ((Lanes([0.0, (RW * TR), 0.0]) + ((((TJ * TL) * TQ) * FQ) * RV)) / TL)) * CH) + Lanes([0.0, (DU * TS), 0.0])) + (Lanes([0.0, (RR * TD), 0.0]) + ((RZ - SN) * RQ));
                        SF = TT;
                        SG = TU;
                    } else {
                        SF = RD;
                        SG = SE;
                    }
                    RG = SF;
                    RH = SG;
                } else {
                    let RF = if QN > RD { 1.0 } else { 0.0 };
                    let UQ;
                    let UR;
                    if RF != 0.0 {
                        let TV = FO - ((-8.754687373538999e-1f64 / JE).exp());
                        let TW = CH * TV;
                        let TX = TW - F;
                        let TY = Lanes([0.0, (DU * TV), 0.0]);
                        let TZ = Lanes([G[0], 0.0, G[1]]);
                        let UA = TX * CI;
                        let UB = ((TY - TZ) * CI) + Lanes([0.0, (DV * TX), 0.0]);
                        let UC = UB * UA;
                        let UE = ((UA * UA) + UD).sqrt();
                        let UF = (UA + UE) * GW;
                        let UG = TW - (CJ * UF);
                        let UH = TY - (Lanes([0.0, (DW * UF), 0.0]) + (((UB + ((UC + UC) * (FX / (GV * UE)))) * GW) * CJ));
                        let UI = UG / CH;
                        let UJ = FO - UI;
                        let UK = FO - JE;
                        let UL = ((UJ.ln()) * UK).exp();
                        let UM = FO - UL;
                        let UN = ((CH * UM) / UK) + (RP * (F - UG));
                        let UO = QN * UN;
                        let UP = Lanes([0.0, (QO * UN), 0.0]) + ((((Lanes([0.0, (DU * UM), 0.0]) + ((((((((UH - Lanes([0.0, (DU * UI), 0.0])) / CH) * FQ) * (FX / UJ)) * UK) * UL) * FQ) * CH)) / UK) + ((TZ - UH) * RP)) * QN);
                        UQ = UO;
                        UR = UP;
                    } else {
                        UQ = RD;
                        UR = SE;
                    }
                    RG = UQ;
                    RH = UR;
                }
                QW = QK;
                QX = RD;
                QY = RG;
                QZ = QL;
                RA = RI;
                RB = RH;
            } else {
                let QS = CF * QJ;
                let QT = DS * QJ;
                let QV = if QU < QQ { 1.0 } else { 0.0 };
                let UU;
                let UV;
                if QV != 0.0 {
                    let US = if QS > RD { 1.0 } else { 0.0 };
                    let VS;
                    let VT;
                    if US != 0.0 {
                        let UZ = NB / GT;
                        let VA = QU - CK;
                        let VB = DX * FQ;
                        let VC = FO - ((-8.754687373538999e-1f64 / NB).exp());
                        let VD = CK * VC;
                        let VE = DX * VC;
                        let VF = RP * QS;
                        let VG = QT * RP;
                        let VH = UZ - NB;
                        let VI = QU / CK;
                        let VJ = (VH * (VI.ln())).exp();
                        let VK = QS * VJ;
                        let VL = (QT * VJ) + (((((((DX * VI) * FQ) / CK) * (FX / VI)) * VH) * VJ) * QS);
                        let VM = VD - L;
                        let VN = Lanes([VE, 0.0, 0.0]);
                        let VO = Lanes([0.0, M[0], M[1]]);
                        let VP = VM * CI;
                        let VQ = ((VN - VO) * CI) + Lanes([(DV * VM), 0.0, 0.0]);
                        let VR = if VP < SC { 1.0 } else { 0.0 };
                        let VZ;
                        let WA;
                        if VR != 0.0 {
                            let VU = VP.exp();
                            let VV = FO + VU;
                            let VW = VV.ln();
                            let VX = VD - (CJ * VW);
                            let VY = VN - (Lanes([(DW * VW), 0.0, 0.0]) + (((VQ * VU) * (FX / VV)) * CJ));
                            VZ = VX;
                            WA = VY;
                        } else {
                            VZ = L;
                            WA = VO;
                        }
                        let WB = (SO * VA) + (GT * CJ);
                        let WC = (VB * SO) + (DW * GT);
                        let WD = (VA + VZ) / WB;
                        let WE = ((Lanes([VB, 0.0, 0.0]) + WA) - Lanes([(WC * WD), 0.0, 0.0])) / WB;
                        let WF = if WD < SC { 1.0 } else { 0.0 };
                        let WN;
                        let WO;
                        if WF != 0.0 {
                            let WG = WD.exp();
                            let WH = FO + WG;
                            let WI = (-(VA + VD)) / WB;
                            let WJ = WI.exp();
                            let WK = (WH.ln()) - WJ;
                            let WL = (-VA) + (WB * WK);
                            let WM = Lanes([(VB * FQ), 0.0, 0.0]) + (Lanes([(WC * WK), 0.0, 0.0]) + ((((WE * WG) * (FX / WH)) - Lanes([(((((VB + VE) * FQ) - (WC * WI)) / WB) * WJ), 0.0, 0.0])) * WB));
                            WN = WL;
                            WO = WM;
                        } else {
                            WN = VZ;
                            WO = WA;
                        }
                        let WP = L - VZ;
                        let WQ = VZ / CK;
                        let WR = FO - WQ;
                        let WS = WN / CK;
                        let WT = FO - WS;
                        let WU = WT.ln();
                        let WV = (((WO - Lanes([(DX * WS), 0.0, 0.0])) / CK) * FQ) * (FX / WT);
                        let WW = FO - NB;
                        let WX = FO - UZ;
                        let WY = (WU * WW).exp();
                        let WZ = FO - WY;
                        let XA = ((WR.ln()) * WX).exp();
                        let XB = FO - XA;
                        let XC = (WU * WX).exp();
                        let XD = FO - XC;
                        let XE = (((QS * WZ) / WW) + ((VK * XB) / WX)) - ((VK * XD) / WX);
                        let XF = (XE * CK) + (VF * WP);
                        let XG = ((((((Lanes([(QT * WZ), 0.0, 0.0]) + ((((WV * WW) * WY) * FQ) * QS)) / WW) + ((Lanes([(VL * XB), 0.0, 0.0]) + ((((((((WA - Lanes([(DX * WQ), 0.0, 0.0])) / CK) * FQ) * (FX / WR)) * WX) * XA) * FQ) * VK)) / WX)) - ((Lanes([(VL * XD), 0.0, 0.0]) + ((((WV * WX) * XC) * FQ) * VK)) / WX)) * CK) + Lanes([(DX * XE), 0.0, 0.0])) + (Lanes([(VG * WP), 0.0, 0.0]) + ((VO - WA) * VF));
                        VS = XF;
                        VT = XG;
                    } else {
                        VS = RD;
                        VT = RI;
                    }
                    UU = VS;
                    UV = VT;
                } else {
                    let UT = if QS > RD { 1.0 } else { 0.0 };
                    let YB;
                    let YC;
                    if UT != 0.0 {
                        let XH = FO - ((-8.754687373538999e-1f64 / NB).exp());
                        let XI = CK * XH;
                        let XJ = XI - L;
                        let XK = Lanes([(DX * XH), 0.0, 0.0]);
                        let XL = Lanes([0.0, M[0], M[1]]);
                        let XM = XJ * CI;
                        let XN = ((XK - XL) * CI) + Lanes([(DV * XJ), 0.0, 0.0]);
                        let XO = XN * XM;
                        let XP = ((XM * XM) + UD).sqrt();
                        let XQ = (XM + XP) * GW;
                        let XR = XI - (CJ * XQ);
                        let XS = XK - (Lanes([(DW * XQ), 0.0, 0.0]) + (((XN + ((XO + XO) * (FX / (GV * XP)))) * GW) * CJ));
                        let XT = XR / CK;
                        let XU = FO - XT;
                        let XV = FO - NB;
                        let XW = ((XU.ln()) * XV).exp();
                        let XX = FO - XW;
                        let XY = ((CK * XX) / XV) + (RP * (L - XR));
                        let XZ = QS * XY;
                        let YA = Lanes([(QT * XY), 0.0, 0.0]) + ((((Lanes([(DX * XX), 0.0, 0.0]) + ((((((((XS - Lanes([(DX * XT), 0.0, 0.0])) / CK) * FQ) * (FX / XU)) * XV) * XW) * FQ) * CK)) / XV) + ((XL - XS) * RP)) * QS);
                        YB = XZ;
                        YC = YA;
                    } else {
                        YB = RD;
                        YC = RI;
                    }
                    UU = YB;
                    UV = YC;
                }
                let UW = FO - QJ;
                let UX = CF * UW;
                let UY = DS * UW;
                let YF;
                let YG;
                if QV != 0.0 {
                    let YD = if UX > RD { 1.0 } else { 0.0 };
                    let ZA;
                    let ZB;
                    if YD != 0.0 {
                        let YH = NB / GT;
                        let YI = QU - CK;
                        let YJ = DX * FQ;
                        let YK = FO - ((-8.754687373538999e-1f64 / NB).exp());
                        let YL = CK * YK;
                        let YM = DX * YK;
                        let YN = RP * UX;
                        let YO = UY * RP;
                        let YP = YH - NB;
                        let YQ = QU / CK;
                        let YR = (YP * (YQ.ln())).exp();
                        let YS = UX * YR;
                        let YT = (UY * YR) + (((((((DX * YQ) * FQ) / CK) * (FX / YQ)) * YP) * YR) * UX);
                        let YU = YL - F;
                        let YV = Lanes([0.0, YM, 0.0]);
                        let YW = Lanes([G[0], 0.0, G[1]]);
                        let YX = YU * CI;
                        let YY = ((YV - YW) * CI) + Lanes([0.0, (DV * YU), 0.0]);
                        let YZ = if YX < SC { 1.0 } else { 0.0 };
                        let ZH;
                        let ZI;
                        if YZ != 0.0 {
                            let ZC = YX.exp();
                            let ZD = FO + ZC;
                            let ZE = ZD.ln();
                            let ZF = YL - (CJ * ZE);
                            let ZG = YV - (Lanes([0.0, (DW * ZE), 0.0]) + (((YY * ZC) * (FX / ZD)) * CJ));
                            ZH = ZF;
                            ZI = ZG;
                        } else {
                            ZH = F;
                            ZI = YW;
                        }
                        let ZJ = (SO * YI) + (GT * CJ);
                        let ZK = (YJ * SO) + (DW * GT);
                        let ZL = (YI + ZH) / ZJ;
                        let ZM = ((Lanes([0.0, YJ, 0.0]) + ZI) - Lanes([0.0, (ZK * ZL), 0.0])) / ZJ;
                        let ZN = if ZL < SC { 1.0 } else { 0.0 };
                        let ZV;
                        let ZW;
                        if ZN != 0.0 {
                            let ZO = ZL.exp();
                            let ZP = FO + ZO;
                            let ZQ = (-(YI + YL)) / ZJ;
                            let ZR = ZQ.exp();
                            let ZS = (ZP.ln()) - ZR;
                            let ZT = (-YI) + (ZJ * ZS);
                            let ZU = Lanes([0.0, (YJ * FQ), 0.0]) + (Lanes([0.0, (ZK * ZS), 0.0]) + ((((ZM * ZO) * (FX / ZP)) - Lanes([0.0, (((((YJ + YM) * FQ) - (ZK * ZQ)) / ZJ) * ZR), 0.0])) * ZJ));
                            ZV = ZT;
                            ZW = ZU;
                        } else {
                            ZV = ZH;
                            ZW = ZI;
                        }
                        let ZX = F - ZH;
                        let ZY = ZH / CK;
                        let ZZ = FO - ZY;
                        let AAA = ZV / CK;
                        let AAB = FO - AAA;
                        let AAC = AAB.ln();
                        let AAD = (((ZW - Lanes([0.0, (DX * AAA), 0.0])) / CK) * FQ) * (FX / AAB);
                        let AAE = FO - NB;
                        let AAF = FO - YH;
                        let AAG = (AAC * AAE).exp();
                        let AAH = FO - AAG;
                        let AAI = ((ZZ.ln()) * AAF).exp();
                        let AAJ = FO - AAI;
                        let AAK = (AAC * AAF).exp();
                        let AAL = FO - AAK;
                        let AAM = (((UX * AAH) / AAE) + ((YS * AAJ) / AAF)) - ((YS * AAL) / AAF);
                        let AAN = (AAM * CK) + (YN * ZX);
                        let AAO = ((((((Lanes([0.0, (UY * AAH), 0.0]) + ((((AAD * AAE) * AAG) * FQ) * UX)) / AAE) + ((Lanes([0.0, (YT * AAJ), 0.0]) + ((((((((ZI - Lanes([0.0, (DX * ZY), 0.0])) / CK) * FQ) * (FX / ZZ)) * AAF) * AAI) * FQ) * YS)) / AAF)) - ((Lanes([0.0, (YT * AAL), 0.0]) + ((((AAD * AAF) * AAK) * FQ) * YS)) / AAF)) * CK) + Lanes([0.0, (DX * AAM), 0.0])) + (Lanes([0.0, (YO * ZX), 0.0]) + ((YW - ZI) * YN));
                        ZA = AAN;
                        ZB = AAO;
                    } else {
                        ZA = RD;
                        ZB = SE;
                    }
                    YF = ZA;
                    YG = ZB;
                } else {
                    let YE = if UX > RD { 1.0 } else { 0.0 };
                    let ABJ;
                    let ABK;
                    if YE != 0.0 {
                        let AAP = FO - ((-8.754687373538999e-1f64 / NB).exp());
                        let AAQ = CK * AAP;
                        let AAR = AAQ - F;
                        let AAS = Lanes([0.0, (DX * AAP), 0.0]);
                        let AAT = Lanes([G[0], 0.0, G[1]]);
                        let AAU = AAR * CI;
                        let AAV = ((AAS - AAT) * CI) + Lanes([0.0, (DV * AAR), 0.0]);
                        let AAW = AAV * AAU;
                        let AAX = ((AAU * AAU) + UD).sqrt();
                        let AAY = (AAU + AAX) * GW;
                        let AAZ = AAQ - (CJ * AAY);
                        let ABA = AAS - (Lanes([0.0, (DW * AAY), 0.0]) + (((AAV + ((AAW + AAW) * (FX / (GV * AAX)))) * GW) * CJ));
                        let ABB = AAZ / CK;
                        let ABC = FO - ABB;
                        let ABD = FO - NB;
                        let ABE = ((ABC.ln()) * ABD).exp();
                        let ABF = FO - ABE;
                        let ABG = ((CK * ABF) / ABD) + (RP * (F - AAZ));
                        let ABH = UX * ABG;
                        let ABI = Lanes([0.0, (UY * ABG), 0.0]) + ((((Lanes([0.0, (DX * ABF), 0.0]) + ((((((((ABA - Lanes([0.0, (DX * ABB), 0.0])) / CK) * FQ) * (FX / ABC)) * ABD) * ABE) * FQ) * CK)) / ABD) + ((AAT - ABA) * RP)) * UX);
                        ABJ = ABH;
                        ABK = ABI;
                    } else {
                        ABJ = RD;
                        ABK = SE;
                    }
                    YF = ABJ;
                    YG = ABK;
                }
                QW = CG;
                QX = UU;
                QY = YF;
                QZ = DT;
                RA = UV;
                RB = YG;
            }
            let ABN;
            let ABO;
            if RC != 0.0 {
                let ABL = if QW > RD { 1.0 } else { 0.0 };
                let ACL;
                let ACM;
                if ABL != 0.0 {
                    let ABS = JE / GT;
                    let ABT = QP - CH;
                    let ABU = DU * FQ;
                    let ABV = FO - ((-8.754687373538999e-1f64 / JE).exp());
                    let ABW = CH * ABV;
                    let ABX = DU * ABV;
                    let ABY = RP * QW;
                    let ABZ = QZ * RP;
                    let ACA = ABS - JE;
                    let ACB = QP / CH;
                    let ACC = (ACA * (ACB.ln())).exp();
                    let ACD = QW * ACC;
                    let ACE = (QZ * ACC) + (((((((DU * ACB) * FQ) / CH) * (FX / ACB)) * ACA) * ACC) * QW);
                    let ACF = ABW - L;
                    let ACG = Lanes([ABX, 0.0, 0.0]);
                    let ACH = Lanes([0.0, M[0], M[1]]);
                    let ACI = ACF * CI;
                    let ACJ = ((ACG - ACH) * CI) + Lanes([(DV * ACF), 0.0, 0.0]);
                    let ACK = if ACI < SC { 1.0 } else { 0.0 };
                    let ACS;
                    let ACT;
                    if ACK != 0.0 {
                        let ACN = ACI.exp();
                        let ACO = FO + ACN;
                        let ACP = ACO.ln();
                        let ACQ = ABW - (CJ * ACP);
                        let ACR = ACG - (Lanes([(DW * ACP), 0.0, 0.0]) + (((ACJ * ACN) * (FX / ACO)) * CJ));
                        ACS = ACQ;
                        ACT = ACR;
                    } else {
                        ACS = L;
                        ACT = ACH;
                    }
                    let ACU = (SO * ABT) + (GT * CJ);
                    let ACV = (ABU * SO) + (DW * GT);
                    let ACW = (ABT + ACS) / ACU;
                    let ACX = ((Lanes([ABU, 0.0, 0.0]) + ACT) - Lanes([(ACV * ACW), 0.0, 0.0])) / ACU;
                    let ACY = if ACW < SC { 1.0 } else { 0.0 };
                    let ADG;
                    let ADH;
                    if ACY != 0.0 {
                        let ACZ = ACW.exp();
                        let ADA = FO + ACZ;
                        let ADB = (-(ABT + ABW)) / ACU;
                        let ADC = ADB.exp();
                        let ADD = (ADA.ln()) - ADC;
                        let ADE = (-ABT) + (ACU * ADD);
                        let ADF = Lanes([(ABU * FQ), 0.0, 0.0]) + (Lanes([(ACV * ADD), 0.0, 0.0]) + ((((ACX * ACZ) * (FX / ADA)) - Lanes([(((((ABU + ABX) * FQ) - (ACV * ADB)) / ACU) * ADC), 0.0, 0.0])) * ACU));
                        ADG = ADE;
                        ADH = ADF;
                    } else {
                        ADG = ACS;
                        ADH = ACT;
                    }
                    let ADI = L - ACS;
                    let ADJ = ACS / CH;
                    let ADK = FO - ADJ;
                    let ADL = ADG / CH;
                    let ADM = FO - ADL;
                    let ADN = ADM.ln();
                    let ADO = (((ADH - Lanes([(DU * ADL), 0.0, 0.0])) / CH) * FQ) * (FX / ADM);
                    let ADP = FO - JE;
                    let ADQ = FO - ABS;
                    let ADR = (ADN * ADP).exp();
                    let ADS = FO - ADR;
                    let ADT = ((ADK.ln()) * ADQ).exp();
                    let ADU = FO - ADT;
                    let ADV = (ADN * ADQ).exp();
                    let ADW = FO - ADV;
                    let ADX = (((QW * ADS) / ADP) + ((ACD * ADU) / ADQ)) - ((ACD * ADW) / ADQ);
                    let ADY = (ADX * CH) + (ABY * ADI);
                    let ADZ = ((((((Lanes([(QZ * ADS), 0.0, 0.0]) + ((((ADO * ADP) * ADR) * FQ) * QW)) / ADP) + ((Lanes([(ACE * ADU), 0.0, 0.0]) + ((((((((ACT - Lanes([(DU * ADJ), 0.0, 0.0])) / CH) * FQ) * (FX / ADK)) * ADQ) * ADT) * FQ) * ACD)) / ADQ)) - ((Lanes([(ACE * ADW), 0.0, 0.0]) + ((((ADO * ADQ) * ADV) * FQ) * ACD)) / ADQ)) * CH) + Lanes([(DU * ADX), 0.0, 0.0])) + (Lanes([(ABZ * ADI), 0.0, 0.0]) + ((ACH - ACT) * ABY));
                    ACL = ADY;
                    ACM = ADZ;
                } else {
                    ACL = RD;
                    ACM = RI;
                }
                ABN = ACL;
                ABO = ACM;
            } else {
                let ABM = if QW > RD { 1.0 } else { 0.0 };
                let AEU;
                let AEV;
                if ABM != 0.0 {
                    let AEA = FO - ((-8.754687373538999e-1f64 / JE).exp());
                    let AEB = CH * AEA;
                    let AEC = AEB - L;
                    let AED = Lanes([(DU * AEA), 0.0, 0.0]);
                    let AEE = Lanes([0.0, M[0], M[1]]);
                    let AEF = AEC * CI;
                    let AEG = ((AED - AEE) * CI) + Lanes([(DV * AEC), 0.0, 0.0]);
                    let AEH = AEG * AEF;
                    let AEI = ((AEF * AEF) + UD).sqrt();
                    let AEJ = (AEF + AEI) * GW;
                    let AEK = AEB - (CJ * AEJ);
                    let AEL = AED - (Lanes([(DW * AEJ), 0.0, 0.0]) + (((AEG + ((AEH + AEH) * (FX / (GV * AEI)))) * GW) * CJ));
                    let AEM = AEK / CH;
                    let AEN = FO - AEM;
                    let AEO = FO - JE;
                    let AEP = ((AEN.ln()) * AEO).exp();
                    let AEQ = FO - AEP;
                    let AER = ((CH * AEQ) / AEO) + (RP * (L - AEK));
                    let AES = QW * AER;
                    let AET = Lanes([(QZ * AER), 0.0, 0.0]) + ((((Lanes([(DU * AEQ), 0.0, 0.0]) + ((((((((AEL - Lanes([(DU * AEM), 0.0, 0.0])) / CH) * FQ) * (FX / AEN)) * AEO) * AEP) * FQ) * CH)) / AEO) + ((AEE - AEL) * RP)) * QW);
                    AEU = AES;
                    AEV = AET;
                } else {
                    AEU = RD;
                    AEV = RI;
                }
                ABN = AEU;
                ABO = AEV;
            }
            let ABP = ABN + QX;
            let ABQ = ABO + RA;
            let ABR = if QW > RD { 1.0 } else { 0.0 };
            let AFS;
            let AFT;
            if ABR != 0.0 {
                let AEW = FO - ((-8.754687373538999e-1f64 / JE).exp());
                let AEX = CH * AEW;
                let AEY = AEX - L;
                let AEZ = Lanes([(DU * AEW), 0.0, 0.0]);
                let AFA = AEY * CI;
                let AFB = ((AEZ - Lanes([0.0, M[0], M[1]])) * CI) + Lanes([(DV * AEY), 0.0, 0.0]);
                let AFC = AFB * AFA;
                let AFD = ((AFA * AFA) + UD).sqrt();
                let AFE = (AFC + AFC) * (FX / (GV * AFD));
                let AFF = (AFA + AFD) * GW;
                let AFG = (AFB + AFE) * GW;
                let AFH = AFF / AFD;
                let AFI = (AFG - (AFE * AFH)) / AFD;
                let AFJ = -JE;
                let AFK = (AEX - (CJ * AFF)) / CH;
                let AFL = FO - AFK;
                let AFM = (AFJ * (AFL.ln())).exp();
                let AFN = QW * AFM;
                let AFO = RP * QW;
                let AFP = FO - AFH;
                let AFQ = (AFN * AFH) + (AFO * AFP);
                let AFR = (((Lanes([(QZ * AFM), 0.0, 0.0]) + ((((((((AEZ - (Lanes([(DW * AFF), 0.0, 0.0]) + (AFG * CJ))) - Lanes([(DU * AFK), 0.0, 0.0])) / CH) * FQ) * (FX / AFL)) * AFJ) * AFM) * QW)) * AFH) + (AFI * AFN)) + (Lanes([((QZ * RP) * AFP), 0.0, 0.0]) + ((AFI * FQ) * AFO));
                AFS = AFQ;
                AFT = AFR;
            } else {
                AFS = RD;
                AFT = RI;
            }
            let AFZ;
            let AGA;
            if AJ != 0.0 {
                let AFU = CL - L;
                let AFV = Lanes([DY, 0.0, 0.0]) - Lanes([0.0, M[0], M[1]]);
                let AFW = Lanes([AFV[0], AFV[1], AFV[2], 0.0]);
                AFZ = AFU;
                AGA = AFW;
            } else {
                let AFX = T - CM;
                let AFY = Lanes([0.0, U[0], U[1], U[2]]) - Lanes([DZ, 0.0, 0.0, 0.0]);
                AFZ = AFX;
                AGA = AFY;
            }
            let AGB = (AGA * CI) + Lanes([(DV * AFZ), 0.0, 0.0, 0.0]);
            let AGC = (AFZ * CI) - FO;
            let AGD = AGB * AGC;
            let AGE = ((AGC * AGC) + UD).sqrt();
            let AGF = FO + ((AGC + AGE) / GO);
            let AGG = AGF * CJ;
            let AGH = (((AGB + ((AGD + AGD) * (FX / (GV * AGE)))) / GO) * CJ) + Lanes([(DW * AGF), 0.0, 0.0, 0.0]);
            let AGI = AGG / CN;
            let AGK = (AGJ * (AGI.ln())).exp();
            let AGL = FO + AGK;
            let AGM = ((AGL.ln()) / AGJ).exp();
            let AGN = (AGG * CO) / AGM;
            let AGP = (AGG - CN) / AGO;
            let AGQ = (AGH - Lanes([EA, 0.0, 0.0, 0.0])) / AGO;
            let AGR = AGQ * AGP;
            let AGS = ((AGP * AGP) + parameters[66]).sqrt();
            let AGT = FO + (GW * (AGP + AGS));
            let AGU = AGN * AGT;
            let AGV = (((((AGH * CO) + Lanes([(EB * AGG), 0.0, 0.0, 0.0])) - (((((((((AGH - Lanes([(EA * AGI), 0.0, 0.0, 0.0])) / CN) * (FX / AGI)) * AGJ) * AGK) * (FX / AGL)) / AGJ) * AGM) * AGN)) / AGM) * AGT) + (((AGQ + ((AGR + AGR) * (FX / (GV * AGS)))) * GW) * AGN);
            let AGW = if (if AFS > RD { 1.0 } else { 0.0 }) != 0.0 && ABR != 0.0 { 1.0 } else { 0.0 };
            let AHB;
            let AHC;
            let AHD;
            let AHE;
            if AGW != 0.0 {
                let AGX = QW / AFS;
                let AGY = (Lanes([QZ, 0.0, 0.0]) - (AFT * AGX)) / AFS;
                let AGZ = ABN / QW;
                let AHA = (ABO - Lanes([(QZ * AGZ), 0.0, 0.0])) / QW;
                AHB = AGZ;
                AHC = AGX;
                AHD = AHA;
                AHE = AGY;
            } else {
                AHB = RD;
                AHC = FO;
                AHD = RI;
                AHE = RI;
            }
            let AHF = if CP > RD { 1.0 } else { 0.0 };
            let AID;
            let AIE;
            if AHF != 0.0 {
                let AHG = ((-(CR.ln())) / HD).exp();
                let AHH = FO - AHG;
                let AHI = CQ * AHH;
                let AHJ = AHI - R;
                let AHK = Lanes([((ED * AHH) + ((((((EE * (FX / CR)) * FQ) / HD) * AHG) * FQ) * CQ)), 0.0, 0.0]);
                let AHL = Lanes([0.0, S[0], S[1]]);
                let AHM = AHJ * CI;
                let AHN = ((AHK - AHL) * CI) + Lanes([(DV * AHJ), 0.0, 0.0]);
                let AHO = AHN * AHM;
                let AHP = ((AHM * AHM) + UD).sqrt();
                let AHQ = (AHM + AHP) * GW;
                let AHR = AHI - (CJ * AHQ);
                let AHS = AHK - (Lanes([(DW * AHQ), 0.0, 0.0]) + (((AHN + ((AHO + AHO) * (FX / (GV * AHP)))) * GW) * CJ));
                let AHT = AHR / CQ;
                let AHU = FO - AHT;
                let AHV = FO - HD;
                let AHW = ((AHU.ln()) * AHV).exp();
                let AHX = FO - AHW;
                let AHY = R - AHR;
                let AHZ = ((CQ * AHX) / AHV) + (CR * AHY);
                let AIA = CP * AHZ;
                let AIB = Lanes([(EC * AHZ), 0.0, 0.0]) + ((((Lanes([(ED * AHX), 0.0, 0.0]) + ((((((((AHS - Lanes([(ED * AHT), 0.0, 0.0])) / CQ) * FQ) * (FX / AHU)) * AHV) * AHW) * FQ) * CQ)) / AHV) + (Lanes([(EE * AHY), 0.0, 0.0]) + ((AHL - AHS) * CR))) * CP);
                AID = AIA;
                AIE = AIB;
            } else {
                AID = RD;
                AIE = AIC;
            }
            let AIF = AID / CP;
            let AIG = (AIE - Lanes([(EC * AIF), 0.0, 0.0])) / CP;
            let AII;
            let AIJ;
            let AIK;
            let AIL;
            if AM != 0.0 {
                let AIH = if CS > RD { 1.0 } else { 0.0 };
                let AJJ;
                let AJK;
                if AIH != 0.0 {
                    let AIN = ((-(CU.ln())) / HX).exp();
                    let AIO = FO - AIN;
                    let AIP = CT * AIO;
                    let AIQ = AIP - R;
                    let AIR = Lanes([((EG * AIO) + ((((((EH * (FX / CU)) * FQ) / HX) * AIN) * FQ) * CT)), 0.0, 0.0]);
                    let AIS = Lanes([0.0, S[0], S[1]]);
                    let AIT = AIQ * CI;
                    let AIU = ((AIR - AIS) * CI) + Lanes([(DV * AIQ), 0.0, 0.0]);
                    let AIV = AIU * AIT;
                    let AIW = ((AIT * AIT) + UD).sqrt();
                    let AIX = (AIT + AIW) * GW;
                    let AIY = AIP - (CJ * AIX);
                    let AIZ = AIR - (Lanes([(DW * AIX), 0.0, 0.0]) + (((AIU + ((AIV + AIV) * (FX / (GV * AIW)))) * GW) * CJ));
                    let AJA = AIY / CT;
                    let AJB = FO - AJA;
                    let AJC = FO - HX;
                    let AJD = ((AJB.ln()) * AJC).exp();
                    let AJE = FO - AJD;
                    let AJF = R - AIY;
                    let AJG = ((CT * AJE) / AJC) + (CU * AJF);
                    let AJH = CS * AJG;
                    let AJI = Lanes([(EF * AJG), 0.0, 0.0]) + ((((Lanes([(EG * AJE), 0.0, 0.0]) + ((((((((AIZ - Lanes([(EG * AJA), 0.0, 0.0])) / CT) * FQ) * (FX / AJB)) * AJC) * AJD) * FQ) * CT)) / AJC) + (Lanes([(EH * AJF), 0.0, 0.0]) + ((AIS - AIZ) * CU))) * CS);
                    AJJ = AJH;
                    AJK = AJI;
                } else {
                    AJJ = RD;
                    AJK = AIC;
                }
                let AJL = AJJ / CS;
                let AJM = (AJK - Lanes([(EF * AJL), 0.0, 0.0])) / CS;
                AII = CT;
                AIJ = AJL;
                AIK = EG;
                AIL = AJM;
            } else {
                AII = CQ;
                AIJ = AIF;
                AIK = ED;
                AIL = AIG;
            }
            let AKE;
            let AKF;
            if AIM != 0.0 {
                AKE = FO;
                AKF = AIC;
            } else {
                let AJO = AJN * CJ;
                let AJP = DW * AJN;
                let AJQ = Lanes([AIK, 0.0, 0.0]);
                let AJR = (AII - R) / AJO;
                let AJS = ((AJQ - Lanes([0.0, S[0], S[1]])) - Lanes([(AJP * AJR), 0.0, 0.0])) / AJO;
                let AJT = AJS * AJR;
                let AJU = ((AJR * AJR) + UD).sqrt();
                let AJV = AJR + AJU;
                let AJW = (AII - ((AJO * AJV) * GW)) / AII;
                let AJX = FO - AJW;
                let AJZ = (AJY * (AJX.ln())).exp();
                let AKA = FO - AJZ;
                let AKB = CV * AKA;
                let AKC = Lanes([(EI * AKA), 0.0, 0.0]) + (((((((((AJQ - ((Lanes([(AJP * AJV), 0.0, 0.0]) + ((AJS + ((AJT + AJT) * (FX / (GV * AJU)))) * AJO)) * GW)) - Lanes([(AIK * AJW), 0.0, 0.0])) / AII) * FQ) * (FX / AJX)) * AJY) * AJZ) * FQ) * CV);
                let AKD = if (AKB.abs()) >= 1e-3f64 { 1.0 } else { 0.0 };
                let ALD;
                let ALE;
                if AKD != 0.0 {
                    let AKY = AKB.exp();
                    let AKZ = (AKY - FO) / AKB;
                    let ALA = ((AKC * AKY) - (AKC * AKZ)) / AKB;
                    ALD = AKZ;
                    ALE = ALA;
                } else {
                    let ALB = AKC * GW;
                    let ALC = FO + (AKB * GW);
                    ALD = ALC;
                    ALE = ALB;
                }
                AKE = ALD;
                AKF = ALE;
            }
            let AKG = (AKE * AIJ) / CW;
            let AKH = (((AKF * AIJ) + (AIL * AKE)) - Lanes([(EJ * AKG), 0.0, 0.0])) / CW;
            let AKJ = AHD / AKI;
            let AKL = (Lanes([AKH[0], 0.0, AKH[1], AKH[2]]) + Lanes([AKJ[0], AKJ[1], AKJ[2], 0.0])) * AKK;
            let AKM = (AKK * ((FO + AKG) + (AHB / AKI))) - FO;
            let AKN = AKL * AKM;
            let AKO = ((AKM * AKM) + UD).sqrt();
            let AKQ = AKP * (FO + ((AKM + AKO) / GO));
            let AKR = ((AKL + ((AKN + AKN) * (FX / (GV * AKO)))) / GO) * AKP;
            let AKT = FO / AHC;
            let AKV = (CX + (AKS * (AHC - FO))) + (AKU * (AKT - FO));
            let AKW = (Lanes([EK, 0.0, 0.0]) + (AHE * AKS)) + ((((AHE * AKT) * FQ) / AHC) * AKU);
            let ALK;
            let ALL;
            if AKX != 0.0 {
                let ALF = AKV / CX;
                let ALG = FO + (ALF - FO);
                let ALH = CY / ALG;
                let ALI = (Lanes([EL, 0.0, 0.0]) - (((AKW - Lanes([(EK * ALF), 0.0, 0.0])) / CX) * ALH)) / ALG;
                ALK = ALH;
                ALL = ALI;
            } else {
                let ALJ = Lanes([EL, 0.0, 0.0]);
                ALK = CY;
                ALL = ALJ;
            }
            let ALN = ALM * CJ;
            let ALO = R / ALN;
            let ALP = Lanes([0.0, S[0], S[1]]);
            let ALQ = (ALP - Lanes([((DW * ALM) * ALO), 0.0, 0.0])) / ALN;
            let ALR = if ALO > SC { 1.0 } else { 0.0 };
            let ALT;
            let ALU;
            let ALV;
            let ALW;
            if ALR != 0.0 {
                let ALS = FO + (ALO - SC);
                ALT = ALS;
                ALU = SC;
                ALV = ALQ;
                ALW = AIC;
            } else {
                ALT = FO;
                ALU = ALO;
                ALV = AIC;
                ALW = ALQ;
            }
            let ALX = rspice_limexp(ALU);
            let ALY = ALT * ALX;
            let ALZ = CZ * ALY;
            let AMA = Lanes([(EM * ALY), 0.0, 0.0]) + (((ALV * ALX) + ((ALW * ALX) * ALT)) * CZ);
            let AMC = AMB * CJ;
            let AMD = L / AMC;
            let AME = Lanes([0.0, M[0], M[1]]);
            let AMF = (AME - Lanes([((DW * AMB) * AMD), 0.0, 0.0])) / AMC;
            let AMG = if AMD > SC { 1.0 } else { 0.0 };
            let AMI;
            let AMJ;
            let AMK;
            let AML;
            if AMG != 0.0 {
                let AMH = FO + (AMD - SC);
                AMI = AMH;
                AMJ = SC;
                AMK = AMF;
                AML = RI;
            } else {
                AMI = FO;
                AMJ = AMD;
                AMK = RI;
                AML = AMF;
            }
            let AMM = rspice_limexp(AMJ);
            let AMN = AMI * AMM;
            let AMO = CZ * AMN;
            let AMP = Lanes([(EM * AMN), 0.0, 0.0]) + (((AMK * AMM) + ((AML * AMM) * AMI)) * CZ);
            let ANW;
            let ANX;
            let ANY;
            let ANZ;
            if AMQ != 0.0 {
                let AMR = ALZ / ALK;
                let AMS = ALL * AMR;
                let AMT = Lanes([AMA[0], 0.0, AMA[1], AMA[2]]);
                let AMV = AMP / AMU;
                let AMW = AMR + (AMO / AMU);
                let AMX = ((AMT - Lanes([AMS[0], AMS[1], AMS[2], 0.0])) / ALK) + Lanes([AMV[0], AMV[1], AMV[2], 0.0]);
                let AMY = ALZ / AGU;
                let AMZ = ALZ * AMY;
                let ANA = AMA * AMY;
                let ANB = DA / DB;
                let ANC = AMZ * ANB;
                let AND = ((Lanes([ANA[0], 0.0, ANA[1], ANA[2]]) + (((AMT - (AGV * AMY)) / AGU) * ALZ)) * ANB) + Lanes([(((EN - (EO * ANB)) / DB) * AMZ), 0.0, 0.0, 0.0]);
                let ANE = FX / ANC;
                let ANG = (ANF * (ANC.ln())).exp();
                let ANH = AMW + ANG;
                let ANI = AMX + (((AND * ANE) * ANF) * ANG);
                let ANJ = ALZ / DB;
                let ANK = (AMA - Lanes([(EO * ANJ), 0.0, 0.0])) / DB;
                let ANL = (AMW + ANJ) + ANG;
                let ANM = (AMX + Lanes([ANK[0], 0.0, ANK[1], ANK[2]])) + (((AND * ANE) * ANF) * ANG);
                ANW = ANH;
                ANX = ANL;
                ANY = ANI;
                ANZ = ANM;
            } else {
                let ANN = ALZ / ALK;
                let ANO = ALL * ANN;
                let ANP = AMP / AMU;
                let ANQ = ANN + (AMO / AMU);
                let ANR = ((Lanes([AMA[0], 0.0, AMA[1], AMA[2]]) - Lanes([ANO[0], ANO[1], ANO[2], 0.0])) / ALK) + Lanes([ANP[0], ANP[1], ANP[2], 0.0]);
                let ANS = ALZ / DB;
                let ANT = (AMA - Lanes([(EO * ANS), 0.0, 0.0])) / DB;
                let ANU = ANQ + ANS;
                let ANV = ANR + Lanes([ANT[0], 0.0, ANT[1], ANT[2]]);
                ANW = ANQ;
                ANX = ANU;
                ANY = ANR;
                ANZ = ANV;
            }
            let AOA = AKQ * AKQ;
            let AOB = AKR * AKQ;
            let AOC = AOB + AOB;
            let AOD = (AOA + ANW).sqrt();
            let AOE = AKQ + AOD;
            let AOF = AKR + ((AOC + ANY) * (FX / (GV * AOD)));
            let AOG = (AOA + ANX).sqrt();
            let AOH = AKQ + AOG;
            let AOI = AKR + ((AOC + ANZ) * (FX / (GV * AOG)));
            let AOJ = if ((ANX - ANW).abs()) > 1e-8f64 { 1.0 } else { 0.0 };
            let AOZ;
            let APA;
            if AOJ != 0.0 {
                let AOK = FO + parameters[14];
                let AOL = (AGU / AOK) / ALZ;
                let AOM = AMA * AOL;
                let AON = ((AGV / AOK) - Lanes([AOM[0], 0.0, AOM[1], AOM[2]])) / ALZ;
                let AOO = AOH - AOE;
                let AOP = FO + (AOL * AOO);
                let AOQ = (FO - (AOL * AOE)) / AOP;
                let AOR = ((((AON * AOE) + (AOF * AOL)) * FQ) - (((AON * AOO) + ((AOI - AOF) * AOL)) * AOQ)) / AOP;
                let AOS = AOR * AOQ;
                let AOU = ((AOQ * AOQ) + AOT).sqrt();
                let AOW = (AOU + AOQ) / AOV;
                let AOX = (((AOS + AOS) * (FX / (GV * AOU))) + AOR) / AOV;
                AOZ = AOW;
                APA = AOX;
            } else {
                AOZ = RD;
                APA = AOY;
            }
            let APG;
            let APH;
            if APB != 0.0 {
                let AQH;
                let AQI;
                if AMQ != 0.0 {
                    let APK = ALZ / ALK;
                    let APL = ALL * APK;
                    let APM = Lanes([AMA[0], 0.0, AMA[1], AMA[2]]);
                    let APN = AMP / AMU;
                    let APO = ALZ / DB;
                    let APP = APO * AOZ;
                    let APQ = ((AMA - Lanes([(EO * APO), 0.0, 0.0])) / DB) * AOZ;
                    let APR = ALZ / AGU;
                    let APS = ALZ * APR;
                    let APT = AMA * APR;
                    let APU = DA / DB;
                    let APV = APS * APU;
                    let APW = (ANF * (APV.ln())).exp();
                    let APX = ((APK + (AMO / AMU)) + (APP * AOZ)) + APW;
                    let APY = ((((APM - Lanes([APL[0], APL[1], APL[2], 0.0])) / ALK) + Lanes([APN[0], APN[1], APN[2], 0.0])) + (((Lanes([APQ[0], 0.0, APQ[1], APQ[2]]) + (APA * APO)) * AOZ) + (APA * APP))) + ((((((Lanes([APT[0], 0.0, APT[1], APT[2]]) + (((APM - (AGV * APR)) / AGU) * ALZ)) * APU) + Lanes([(((EN - (EO * APU)) / DB) * APS), 0.0, 0.0, 0.0])) * (FX / APV)) * ANF) * APW);
                    AQH = APX;
                    AQI = APY;
                } else {
                    let APZ = ALZ / ALK;
                    let AQA = ALL * APZ;
                    let AQB = AMP / AMU;
                    let AQC = ALZ / DB;
                    let AQD = AQC * AOZ;
                    let AQE = ((AMA - Lanes([(EO * AQC), 0.0, 0.0])) / DB) * AOZ;
                    let AQF = (APZ + (AMO / AMU)) + (AQD * AOZ);
                    let AQG = (((Lanes([AMA[0], 0.0, AMA[1], AMA[2]]) - Lanes([AQA[0], AQA[1], AQA[2], 0.0])) / ALK) + Lanes([AQB[0], AQB[1], AQB[2], 0.0])) + (((Lanes([AQE[0], 0.0, AQE[1], AQE[2]]) + (APA * AQC)) * AOZ) + (APA * AQD));
                    AQH = AQF;
                    AQI = AQG;
                }
                let AQJ = (AOA + AQH).sqrt();
                let AQK = AKQ + AQJ;
                let AQL = AKR + ((AOC + AQI) * (FX / (GV * AQJ)));
                APG = AQK;
                APH = AQL;
            } else {
                let APD = APC * AKQ;
                let APE = AKR * APC;
                let AQU;
                let AQV;
                if APF != 0.0 {
                    AQU = RD;
                    AQV = AOY;
                } else {
                    let AQM = ALZ / ALK;
                    let AQN = ALL * AQM;
                    let AQO = AMP / AMU;
                    let AQP = ALZ / DB;
                    let AQQ = AQP * AOZ;
                    let AQR = ((AMA - Lanes([(EO * AQP), 0.0, 0.0])) / DB) * AOZ;
                    let AQS = -((AQM + (AMO / AMU)) + (AQQ * AOZ));
                    let AQT = ((((Lanes([AMA[0], 0.0, AMA[1], AMA[2]]) - Lanes([AQN[0], AQN[1], AQN[2], 0.0])) / ALK) + Lanes([AQO[0], AQO[1], AQO[2], 0.0])) + (((Lanes([AQR[0], 0.0, AQR[1], AQR[2]]) + (APA * AQP)) * AOZ) + (APA * AQQ))) * FQ;
                    AQU = AQS;
                    AQV = AQT;
                }
                let AQW = -ALZ;
                let AQX = ((AMA * FQ) * ALZ) + (AMA * AQW);
                let AQY = (AQW * ALZ) / AGU;
                let AQZ = (AQY * DA) / DB;
                let ARA = APD * APD;
                let ARB = APE * APD;
                let ARC = ARB + ARB;
                let ARE = AQU - (ARA * ARD);
                let ARF = AQV - (ARC * ARD);
                let ARG = GO * APD;
                let ARI = (((ARG * ARA) / ARH) - ((APD * AQU) * ARD)) + AQZ;
                let ARJ = (((((APE * GO) * ARA) + (ARC * ARG)) / ARH) - (((APE * AQU) + (AQV * APD)) * ARD)) + ((((((Lanes([AQX[0], 0.0, AQX[1], AQX[2]]) - (AGV * AQY)) / AGU) * DA) + Lanes([(EN * AQY), 0.0, 0.0, 0.0])) - Lanes([(EO * AQZ), 0.0, 0.0, 0.0])) / DB);
                let ARK = ARJ * ARI;
                let ARM = ARE * ARE;
                let ARN = ARF * ARE;
                let ARO = ARM * ARE;
                let ARP = ((ARN + ARN) * ARE) + (ARF * ARM);
                let ARQ = ((ARI * ARI) * ARL) + (ARO / ARH);
                let ARR = ((ARK + ARK) * ARL) + (ARP / ARH);
                let ARS = if (ARQ.abs()) < 1e-10f64 { 1.0 } else { 0.0 };
                let ARY;
                let ARZ;
                if ARS != 0.0 {
                    let ARU = (ART * ARI) / ARE;
                    let ARV = ARU - (APD * ARD);
                    let ARW = (((ARJ * ART) - (ARF * ARU)) / ARE) - (APE * ARD);
                    ARY = ARV;
                    ARZ = ARW;
                } else {
                    let ARX = if ARQ > RD { 1.0 } else { 0.0 };
                    let ASP;
                    let ASQ;
                    if ARX != 0.0 {
                        let ASA = (-ARI) * GW;
                        let ASB = (ARJ * FQ) * GW;
                        let ASC = ARQ.sqrt();
                        let ASD = ARR * (FX / (GV * ASC));
                        let ASE = ASA + ASC;
                        let ASF = ASB + ASD;
                        let ASG = if ASE > RD { 1.0 } else { 0.0 };
                        let ASX;
                        let ASY;
                        if ASG != 0.0 {
                            let ASR = (ARD * (ASE.ln())).exp();
                            let ASS = ((ASF * (FX / ASE)) * ARD) * ASR;
                            ASX = ASR;
                            ASY = ASS;
                        } else {
                            let AST = -ASE;
                            let ASU = (ARD * (AST.ln())).exp();
                            let ASV = -ASU;
                            let ASW = ((((ASF * FQ) * (FX / AST)) * ARD) * ASU) * FQ;
                            ASX = ASV;
                            ASY = ASW;
                        }
                        let ASZ = ASA - ASC;
                        let ATA = ASB - ASD;
                        let ATB = if ASZ > RD { 1.0 } else { 0.0 };
                        let ATI;
                        let ATJ;
                        if ATB != 0.0 {
                            let ATC = (ARD * (ASZ.ln())).exp();
                            let ATD = ((ATA * (FX / ASZ)) * ARD) * ATC;
                            ATI = ATC;
                            ATJ = ATD;
                        } else {
                            let ATE = -ASZ;
                            let ATF = (ARD * (ATE.ln())).exp();
                            let ATG = -ATF;
                            let ATH = ((((ATA * FQ) * (FX / ATE)) * ARD) * ATF) * FQ;
                            ATI = ATG;
                            ATJ = ATH;
                        }
                        let ATK = (ASX + ATI) - (APD * ARD);
                        let ATL = (ASY + ATJ) - (APE * ARD);
                        ASP = ATK;
                        ASQ = ATL;
                    } else {
                        let ASH = (-ARI) * GW;
                        let ASI = -2.7e1f64 / ARO;
                        let ASJ = ASI.sqrt();
                        let ASK = ASH * ASJ;
                        let ASL = ASK * ASK;
                        let ASM = ((((ARJ * FQ) * GW) * ASJ) + (((((ARP * ASI) * FQ) / ARO) * (FX / (GV * ASJ))) * ASH)) * ASK;
                        let ASN = ASM + ASM;
                        let ASO = if ASK >= RD { 1.0 } else { 0.0 };
                        let ATW;
                        let ATX;
                        if ASO != 0.0 {
                            let ATM = FO - ASL;
                            let ATN = ASL / ATM;
                            let ATO = ATN.sqrt();
                            let ATP = 1.5707963267948966e0f64 - (ATO.atan());
                            let ATQ = ((((ASN - ((ASN * FQ) * ATN)) / ATM) * (FX / (GV * ATO))) * (FX / (FX + (ATO * ATO)))) * FQ;
                            ATW = ATP;
                            ATX = ATQ;
                        } else {
                            let ATR = FO - ASL;
                            let ATS = ASL / ATR;
                            let ATT = ATS.sqrt();
                            let ATU = (((ASN - ((ASN * FQ) * ATS)) / ATR) * (FX / (GV * ATT))) * (FX / (FX + (ATT * ATT)));
                            let ATV = 1.5707963267948966e0f64 + (ATT.atan());
                            ATW = ATV;
                            ATX = ATU;
                        }
                        let ATZ = ((ATY * ARE) * ARD).sqrt();
                        let AUA = ARD * ATW;
                        let AUB = AUA.cos();
                        let AUC = (ATZ * AUB) - (APD * ARD);
                        let AUD = (((((ARF * ATY) * ARD) * (FX / (GV * ATZ))) * AUB) + (((ATX * ARD) * (FQ * (AUA.sin()))) * ATZ)) - (APE * ARD);
                        ASP = AUC;
                        ASQ = AUD;
                    }
                    ARY = ASP;
                    ARZ = ASQ;
                }
                APG = ARY;
                APH = ARZ;
            }
            let APJ = if APG < API { 1.0 } else { 0.0 };
            let AUE;
            let AUF;
            if APJ != 0.0 {
                AUE = API;
                AUF = AOY;
            } else {
                AUE = APG;
                AUF = APH;
            }
            let AUG = ALZ / AUE;
            let AUH = (Lanes([AMA[0], 0.0, AMA[1], AMA[2]]) - (AUF * AUG)) / AUE;
            let AUI = AMO / AUE;
            let AUJ = (Lanes([AMP[0], AMP[1], AMP[2], 0.0]) - (AUF * AUI)) / AUE;
            let AUK = if AUG < API { 1.0 } else { 0.0 };
            let AUL;
            let AUM;
            if AUK != 0.0 {
                AUL = API;
                AUM = AOY;
            } else {
                AUL = AUG;
                AUM = AUH;
            }
            let AUN = AUL - AUI;
            let AUO = AUM - AUJ;
            let AUP = AKW * AUL;
            let AUQ = AGU / AUL;
            let AUR = FO - AUQ;
            let AUS = ((AGV - (AUM * AUQ)) / AUL) * FQ;
            let AUT = AUS * AUR;
            let AUU = ((AUR * AUR) + parameters[60]).sqrt();
            let AUW = (AUR + AUU) / AUV;
            let AUX = (AUS + ((AUT + AUT) * (FX / (GV * AUU)))) / AUV;
            let AUY = DC * AUW;
            let AUZ = AUY * AUW;
            let AVA = AUL / AGU;
            let AVC = (AVB * (AVA.ln())).exp();
            let AVD = DD * AVC;
            let AVF = ((AKV * AUL) + ((AVD * AUL) / AVE)) + (AUZ * AUL);
            let AVG = ((Lanes([AUP[0], AUP[1], AUP[2], 0.0]) + (AUM * AKV)) + ((((Lanes([(EQ * AVC), 0.0, 0.0, 0.0]) + ((((((AUM - (AGV * AVA)) / AGU) * (FX / AVA)) * AVB) * AVC) * DD)) * AUL) + (AUM * AVD)) / AVE)) + (((((Lanes([(EP * AUW), 0.0, 0.0, 0.0]) + (AUX * DC)) * AUW) + (AUX * AUY)) * AUL) + (AUM * AUZ));
            let AVI = AVH * AUI;
            let AVJ = AUJ * AVH;
            let AVQ;
            let AVR;
            if AVK != 0.0 {
                let AVM = AVL * CJ;
                let AVN = R / AVM;
                let AVO = (ALP - Lanes([((DW * AVL) * AVN), 0.0, 0.0])) / AVM;
                let AVP = if AVN > SC { 1.0 } else { 0.0 };
                let AVU;
                let AVV;
                let AVW;
                let AVX;
                if AVP != 0.0 {
                    let AVT = FO + (AVN - SC);
                    AVU = AVT;
                    AVV = SC;
                    AVW = AVO;
                    AVX = AIC;
                } else {
                    AVU = FO;
                    AVV = AVN;
                    AVW = AIC;
                    AVX = AVO;
                }
                let AVY = rspice_limexp(AVV);
                let AVZ = (AVU * AVY) - FO;
                let AWA = DE * AVZ;
                let AWB = Lanes([(ER * AVZ), 0.0, 0.0]) + (((AVW * AVY) + ((AVX * AVY) * AVU)) * DE);
                AVQ = AWA;
                AVR = AWB;
            } else {
                AVQ = RD;
                AVR = AIC;
            }
            let AWH;
            let AWI;
            if AVS != 0.0 {
                let AWD = AWC * CJ;
                let AWE = R / AWD;
                let AWF = (ALP - Lanes([((DW * AWC) * AWE), 0.0, 0.0])) / AWD;
                let AWG = if AWE > SC { 1.0 } else { 0.0 };
                let AWN;
                let AWO;
                let AWP;
                let AWQ;
                if AWG != 0.0 {
                    let AWM = FO + (AWE - SC);
                    AWN = AWM;
                    AWO = SC;
                    AWP = AWF;
                    AWQ = AIC;
                } else {
                    AWN = FO;
                    AWO = AWE;
                    AWP = AIC;
                    AWQ = AWF;
                }
                let AWR = rspice_limexp(AWO);
                let AWS = (AWN * AWR) - FO;
                let AWT = DF * AWS;
                let AWU = Lanes([(ES * AWS), 0.0, 0.0]) + (((AWP * AWR) + ((AWQ * AWR) * AWN)) * DF);
                AWH = AWT;
                AWI = AWU;
            } else {
                AWH = RD;
                AWI = AIC;
            }
            let AWJ = AVQ + AWH;
            let AWK = AVR + AWI;
            let AXA;
            let AXB;
            if AWL != 0.0 {
                let AWW = AWV * CJ;
                let AWX = L / AWW;
                let AWY = (AME - Lanes([((DW * AWV) * AWX), 0.0, 0.0])) / AWW;
                let AWZ = if AWX > SC { 1.0 } else { 0.0 };
                let AXG;
                let AXH;
                let AXI;
                let AXJ;
                if AWZ != 0.0 {
                    let AXF = FO + (AWX - SC);
                    AXG = AXF;
                    AXH = SC;
                    AXI = AWY;
                    AXJ = RI;
                } else {
                    AXG = FO;
                    AXH = AWX;
                    AXI = RI;
                    AXJ = AWY;
                }
                let AXK = rspice_limexp(AXH);
                let AXL = (AXG * AXK) - FO;
                let AXM = DG * AXL;
                let AXN = Lanes([(ET * AXL), 0.0, 0.0]) + (((AXI * AXK) + ((AXJ * AXK) * AXG)) * DG);
                AXA = AXM;
                AXB = AXN;
            } else {
                AXA = RD;
                AXB = RI;
            }
            let AXC = AWJ + AXA;
            let AXD = Lanes([AXB[0], AXB[1], AXB[2], 0.0]);
            let AXE = Lanes([AWK[0], 0.0, AWK[1], AWK[2]]) + AXD;
            let AXQ;
            let AXR;
            if RC != 0.0 {
                let AXO = if CG > RD { 1.0 } else { 0.0 };
                let AYK;
                let AYL;
                if AXO != 0.0 {
                    let AXS = JE / GT;
                    let AXT = QP - CH;
                    let AXU = DU * FQ;
                    let AXV = FO - ((-8.754687373538999e-1f64 / JE).exp());
                    let AXW = CH * AXV;
                    let AXX = DU * AXV;
                    let AXY = RP * CG;
                    let AXZ = DT * RP;
                    let AYA = AXS - JE;
                    let AYB = QP / CH;
                    let AYC = (AYA * (AYB.ln())).exp();
                    let AYD = CG * AYC;
                    let AYE = (DT * AYC) + (((((((DU * AYB) * FQ) / CH) * (FX / AYB)) * AYA) * AYC) * CG);
                    let AYF = AXW - L;
                    let AYG = Lanes([AXX, 0.0, 0.0]);
                    let AYH = AYF * CI;
                    let AYI = ((AYG - AME) * CI) + Lanes([(DV * AYF), 0.0, 0.0]);
                    let AYJ = if AYH < SC { 1.0 } else { 0.0 };
                    let AYU;
                    let AYV;
                    let AYW;
                    let AYX;
                    if AYJ != 0.0 {
                        let AYM = AYH.exp();
                        let AYN = AYI * AYM;
                        let AYO = FO + AYM;
                        let AYP = AYM / AYO;
                        let AYQ = (AYN - (AYN * AYP)) / AYO;
                        let AYR = AYO.ln();
                        let AYS = AXW - (CJ * AYR);
                        let AYT = AYG - (Lanes([(DW * AYR), 0.0, 0.0]) + ((AYN * (FX / AYO)) * CJ));
                        AYU = AYS;
                        AYV = AYP;
                        AYW = AYT;
                        AYX = AYQ;
                    } else {
                        AYU = L;
                        AYV = FO;
                        AYW = AME;
                        AYX = RI;
                    }
                    let AYY = (SO * AXT) + (GT * CJ);
                    let AYZ = (AXU * SO) + (DW * GT);
                    let AZA = (AXT + AYU) / AYY;
                    let AZB = ((Lanes([AXU, 0.0, 0.0]) + AYW) - Lanes([(AYZ * AZA), 0.0, 0.0])) / AYY;
                    let AZC = if AZA < SC { 1.0 } else { 0.0 };
                    let AZN;
                    let AZO;
                    let AZP;
                    let AZQ;
                    if AZC != 0.0 {
                        let AZD = AZA.exp();
                        let AZE = AZB * AZD;
                        let AZF = FO + AZD;
                        let AZG = AZD / AZF;
                        let AZH = (AZE - (AZE * AZG)) / AZF;
                        let AZI = (-(AXT + AXW)) / AYY;
                        let AZJ = AZI.exp();
                        let AZK = (AZF.ln()) - AZJ;
                        let AZL = (-AXT) + (AYY * AZK);
                        let AZM = Lanes([(AXU * FQ), 0.0, 0.0]) + (Lanes([(AYZ * AZK), 0.0, 0.0]) + (((AZE * (FX / AZF)) - Lanes([(((((AXU + AXX) * FQ) - (AYZ * AZI)) / AYY) * AZJ), 0.0, 0.0])) * AYY));
                        AZN = AZL;
                        AZO = AZG;
                        AZP = AZM;
                        AZQ = AZH;
                    } else {
                        AZN = AYU;
                        AZO = FO;
                        AZP = AYW;
                        AZQ = RI;
                    }
                    let AZR = AYU / CH;
                    let AZS = FO - AZR;
                    let AZT = AZN / CH;
                    let AZU = FO - AZT;
                    let AZV = -JE;
                    let AZW = ((AZU.ln()) * AZV).exp();
                    let AZX = CG * AZW;
                    let AZY = AZX * AYV;
                    let AZZ = -AXS;
                    let BAA = ((AZS.ln()) * AZZ).exp();
                    let BAB = AYD * BAA;
                    let BAC = FO - AZO;
                    let BAD = FO - AYV;
                    let BAE = ((AZY * AZO) + (BAB * BAC)) + (AXY * BAD);
                    let BAF = ((((((Lanes([(DT * AZW), 0.0, 0.0]) + (((((((AZP - Lanes([(DU * AZT), 0.0, 0.0])) / CH) * FQ) * (FX / AZU)) * AZV) * AZW) * CG)) * AYV) + (AYX * AZX)) * AZO) + (AZQ * AZY)) + (((Lanes([(AYE * BAA), 0.0, 0.0]) + (((((((AYW - Lanes([(DU * AZR), 0.0, 0.0])) / CH) * FQ) * (FX / AZS)) * AZZ) * BAA) * AYD)) * BAC) + ((AZQ * FQ) * BAB))) + (Lanes([(AXZ * BAD), 0.0, 0.0]) + ((AYX * FQ) * AXY));
                    AYK = BAE;
                    AYL = BAF;
                } else {
                    AYK = RD;
                    AYL = RI;
                }
                AXQ = AYK;
                AXR = AYL;
            } else {
                let AXP = if CG > RD { 1.0 } else { 0.0 };
                let BBA;
                let BBB;
                if AXP != 0.0 {
                    let BAG = FO - ((-8.754687373538999e-1f64 / JE).exp());
                    let BAH = CH * BAG;
                    let BAI = BAH - L;
                    let BAJ = Lanes([(DU * BAG), 0.0, 0.0]);
                    let BAK = BAI * CI;
                    let BAL = ((BAJ - AME) * CI) + Lanes([(DV * BAI), 0.0, 0.0]);
                    let BAM = BAL * BAK;
                    let BAN = ((BAK * BAK) + UD).sqrt();
                    let BAO = (BAM + BAM) * (FX / (GV * BAN));
                    let BAP = (BAK + BAN) * GW;
                    let BAQ = (BAL + BAO) * GW;
                    let BAR = BAP / BAN;
                    let BAS = (BAQ - (BAO * BAR)) / BAN;
                    let BAT = (BAH - (CJ * BAP)) / CH;
                    let BAU = FO - BAT;
                    let BAV = -JE;
                    let BAW = (BAV * (BAU.ln())).exp();
                    let BAX = (BAW * BAR) + (RP * (FO - BAR));
                    let BAY = CG * BAX;
                    let BAZ = Lanes([(DT * BAX), 0.0, 0.0]) + (((((((((((BAJ - (Lanes([(DW * BAP), 0.0, 0.0]) + (BAQ * CJ))) - Lanes([(DU * BAT), 0.0, 0.0])) / CH) * FQ) * (FX / BAU)) * BAV) * BAW) * BAR) + (BAS * BAW)) + ((BAS * FQ) * RP)) * CG);
                    BBA = BAY;
                    BBB = BAZ;
                } else {
                    BBA = RD;
                    BBB = RI;
                }
                AXQ = BBA;
                AXR = BBB;
            }
            let BBF;
            let BBG;
            if AL != 0.0 {
                let BBC = CH - L;
                let BBD = Lanes([DU, 0.0, 0.0]) - AME;
                let BBE = if BBC > RD { 1.0 } else { 0.0 };
                let BBN;
                let BBO;
                if BBE != 0.0 {
                    let BBI = DH / AXQ;
                    let BBJ = (Lanes([EU, 0.0, 0.0]) - (AXR * BBI)) / AXQ;
                    let BBK = DH / CG;
                    let BBL = (EU - (DT * BBK)) / CG;
                    let BBM = if BBC > BBK { 1.0 } else { 0.0 };
                    let BCE;
                    let BCF;
                    if BBM != 0.0 {
                        let BBP = (-BBI) / BBK;
                        let BBQ = BBP.exp();
                        let BBR = DI * BBQ;
                        let BBS = BBI / BBK;
                        let BBT = FO + BBS;
                        let BBU = BBC - BBK;
                        let BBV = Lanes([BBL, 0.0, 0.0]);
                        let BBW = BBK + (BBT * BBU);
                        let BBX = BBR * BBW;
                        let BBY = ((Lanes([(EV * BBQ), 0.0, 0.0]) + (((((BBJ * FQ) - Lanes([(BBL * BBP), 0.0, 0.0])) / BBK) * BBQ) * DI)) * BBW) + ((BBV + ((((BBJ - Lanes([(BBL * BBS), 0.0, 0.0])) / BBK) * BBU) + ((BBD - BBV) * BBT))) * BBR);
                        BCE = BBX;
                        BCF = BBY;
                    } else {
                        let BBZ = DI * BBC;
                        let BCA = (-BBI) / BBC;
                        let BCB = BCA.exp();
                        let BCC = BBZ * BCB;
                        let BCD = ((Lanes([(EV * BBC), 0.0, 0.0]) + (BBD * DI)) * BCB) + (((((BBJ * FQ) - (BBD * BCA)) / BBC) * BCB) * BBZ);
                        BCE = BCC;
                        BCF = BCD;
                    }
                    let BCG = AUL * BCE;
                    let BCH = BCF * AUL;
                    let BCI = (AUM * BCE) + Lanes([BCH[0], BCH[1], BCH[2], 0.0]);
                    BBN = BCG;
                    BBO = BCI;
                } else {
                    BBN = RD;
                    BBO = AOY;
                }
                BBF = BBN;
                BBG = BBO;
            } else {
                BBF = RD;
                BBG = AOY;
            }
            let BBH = if DJ > RD { 1.0 } else { 0.0 };
            let BCX;
            let BCY;
            if BBH != 0.0 {
                let BCK = AIG / BCJ;
                let BCM = AHD / BCL;
                let BCN = AUL / ALK;
                let BCO = ALL * BCN;
                let BCP = (((FO + (AIF / BCJ)) + (AHB / BCL)) + BCN) + (AUI / AMU);
                let BCQ = ((Lanes([BCK[0], 0.0, BCK[1], BCK[2]]) + Lanes([BCM[0], BCM[1], BCM[2], 0.0])) + ((AUM - Lanes([BCO[0], BCO[1], BCO[2], 0.0])) / ALK)) + (AUJ / AMU);
                let BCR = BCQ * BCP;
                let BCS = ((BCP * BCP) + AOT).sqrt();
                let BCT = GW * (BCP + BCS);
                let BCU = DJ / BCT;
                let BCV = (Lanes([EW, 0.0, 0.0, 0.0]) - (((BCQ + ((BCR + BCR) * (FX / (GV * BCS)))) * GW) * BCU)) / BCT;
                let BCW = if AXC > RD { 1.0 } else { 0.0 };
                let BDI;
                let BDJ;
                if BCW != 0.0 {
                    let BDD = BDC * BCU;
                    let BDE = BDD * AXC;
                    let BDF = BDE * CI;
                    let BDG = ((((BCV * BDC) * AXC) + (AXE * BDD)) * CI) + Lanes([(DV * BDE), 0.0, 0.0, 0.0]);
                    let BDH = if BDF < 1e-6f64 { 1.0 } else { 0.0 };
                    let BDR;
                    let BDS;
                    if BDH != 0.0 {
                        let BDK = FO - (GW * BDF);
                        let BDL = BCU * BDK;
                        let BDM = (BCV * BDK) + (((BDG * GW) * FQ) * BCU);
                        BDR = BDL;
                        BDS = BDM;
                    } else {
                        let BDN = BDF + FO;
                        let BDO = BDN.ln();
                        let BDP = (BCU * BDO) / BDF;
                        let BDQ = (((BCV * BDO) + ((BDG * (FX / BDN)) * BCU)) - (BDG * BDP)) / BDF;
                        BDR = BDP;
                        BDS = BDQ;
                    }
                    BDI = BDR;
                    BDJ = BDS;
                } else {
                    BDI = BCU;
                    BDJ = BCV;
                }
                BCX = BDI;
                BCY = BDJ;
            } else {
                BCX = RD;
                BCY = AOY;
            }
            let BCZ = BCX + DK;
            let BDA = BCY + Lanes([EX, 0.0, 0.0, 0.0]);
            let BEG;
            let BEH;
            if BDB != 0.0 {
                let BDU = BDT * CJ;
                let BDV = DW * BDT;
                let BDW = F / BDU;
                let BDX = rspice_limexp(BDW);
                let BDY = ((Lanes([G[0], 0.0, G[1]]) - Lanes([0.0, (BDV * BDW), 0.0])) / BDU) * BDX;
                let BDZ = W / BDU;
                let BEA = rspice_limexp(BDZ);
                let BEB = ((Lanes([X[0], 0.0, X[1]]) - Lanes([0.0, (BDV * BDZ), 0.0])) / BDU) * BEA;
                let BEC = BDX - BEA;
                let BED = DL * BEC;
                let BEE = Lanes([0.0, 0.0, (EY * BEC), 0.0]) + ((Lanes([BDY[0], 0.0, BDY[1], BDY[2]]) - Lanes([0.0, BEB[0], BEB[1], BEB[2]])) * DL);
                BEG = BED;
                BEH = BEE;
            } else {
                BEG = RD;
                BEH = BEF;
            }
            let BEP;
            let BEQ;
            if BEI != 0.0 {
                let BEK = BEJ * CJ;
                let BEL = W / BEK;
                let BEM = (Lanes([X[0], 0.0, X[1]]) - Lanes([0.0, ((DW * BEJ) * BEL), 0.0])) / BEK;
                let BEN = if BEL > SC { 1.0 } else { 0.0 };
                let BET;
                let BEU;
                let BEV;
                let BEW;
                if BEN != 0.0 {
                    let BES = FO + (BEL - SC);
                    BET = BES;
                    BEU = SC;
                    BEV = BEM;
                    BEW = BEO;
                } else {
                    BET = FO;
                    BEU = BEL;
                    BEV = BEO;
                    BEW = BEM;
                }
                let BEX = rspice_limexp(BEU);
                let BEY = (BET * BEX) - FO;
                let BEZ = DM * BEY;
                let BFA = Lanes([0.0, (EZ * BEY), 0.0]) + (((BEV * BEX) + ((BEW * BEX) * BET)) * DM);
                BEP = BEZ;
                BEQ = BFA;
            } else {
                BEP = RD;
                BEQ = BEO;
            }
            let BFD;
            let BFE;
            if BER != 0.0 {
                let BFB = if DN > RD { 1.0 } else { 0.0 };
                let BGA;
                let BGB;
                if BFB != 0.0 {
                    let BFG = NS / GT;
                    let BFI = BFH - DO;
                    let BFJ = FB * FQ;
                    let BFK = FO - ((-8.754687373538999e-1f64 / NS).exp());
                    let BFL = DO * BFK;
                    let BFM = FB * BFK;
                    let BFN = RP * DN;
                    let BFO = FA * RP;
                    let BFP = BFG - NS;
                    let BFQ = BFH / DO;
                    let BFR = (BFP * (BFQ.ln())).exp();
                    let BFS = DN * BFR;
                    let BFT = (FA * BFR) + (((((((FB * BFQ) * FQ) / DO) * (FX / BFQ)) * BFP) * BFR) * DN);
                    let BFU = BFL - W;
                    let BFV = Lanes([0.0, BFM, 0.0]);
                    let BFW = Lanes([X[0], 0.0, X[1]]);
                    let BFX = BFU * CI;
                    let BFY = ((BFV - BFW) * CI) + Lanes([0.0, (DV * BFU), 0.0]);
                    let BFZ = if BFX < SC { 1.0 } else { 0.0 };
                    let BGH;
                    let BGI;
                    if BFZ != 0.0 {
                        let BGC = BFX.exp();
                        let BGD = FO + BGC;
                        let BGE = BGD.ln();
                        let BGF = BFL - (CJ * BGE);
                        let BGG = BFV - (Lanes([0.0, (DW * BGE), 0.0]) + (((BFY * BGC) * (FX / BGD)) * CJ));
                        BGH = BGF;
                        BGI = BGG;
                    } else {
                        BGH = W;
                        BGI = BFW;
                    }
                    let BGJ = (SO * BFI) + (GT * CJ);
                    let BGK = (BFJ * SO) + (DW * GT);
                    let BGL = (BFI + BGH) / BGJ;
                    let BGM = ((Lanes([0.0, BFJ, 0.0]) + BGI) - Lanes([0.0, (BGK * BGL), 0.0])) / BGJ;
                    let BGN = if BGL < SC { 1.0 } else { 0.0 };
                    let BGV;
                    let BGW;
                    if BGN != 0.0 {
                        let BGO = BGL.exp();
                        let BGP = FO + BGO;
                        let BGQ = (-(BFI + BFL)) / BGJ;
                        let BGR = BGQ.exp();
                        let BGS = (BGP.ln()) - BGR;
                        let BGT = (-BFI) + (BGJ * BGS);
                        let BGU = Lanes([0.0, (BFJ * FQ), 0.0]) + (Lanes([0.0, (BGK * BGS), 0.0]) + ((((BGM * BGO) * (FX / BGP)) - Lanes([0.0, (((((BFJ + BFM) * FQ) - (BGK * BGQ)) / BGJ) * BGR), 0.0])) * BGJ));
                        BGV = BGT;
                        BGW = BGU;
                    } else {
                        BGV = BGH;
                        BGW = BGI;
                    }
                    let BGX = W - BGH;
                    let BGY = BGH / DO;
                    let BGZ = FO - BGY;
                    let BHA = BGV / DO;
                    let BHB = FO - BHA;
                    let BHC = BHB.ln();
                    let BHD = (((BGW - Lanes([0.0, (FB * BHA), 0.0])) / DO) * FQ) * (FX / BHB);
                    let BHE = FO - NS;
                    let BHF = FO - BFG;
                    let BHG = (BHC * BHE).exp();
                    let BHH = FO - BHG;
                    let BHI = ((BGZ.ln()) * BHF).exp();
                    let BHJ = FO - BHI;
                    let BHK = (BHC * BHF).exp();
                    let BHL = FO - BHK;
                    let BHM = (((DN * BHH) / BHE) + ((BFS * BHJ) / BHF)) - ((BFS * BHL) / BHF);
                    let BHN = (BHM * DO) + (BFN * BGX);
                    let BHO = ((((((Lanes([0.0, (FA * BHH), 0.0]) + ((((BHD * BHE) * BHG) * FQ) * DN)) / BHE) + ((Lanes([0.0, (BFT * BHJ), 0.0]) + ((((((((BGI - Lanes([0.0, (FB * BGY), 0.0])) / DO) * FQ) * (FX / BGZ)) * BHF) * BHI) * FQ) * BFS)) / BHF)) - ((Lanes([0.0, (BFT * BHL), 0.0]) + ((((BHD * BHF) * BHK) * FQ) * BFS)) / BHF)) * DO) + Lanes([0.0, (FB * BHM), 0.0])) + (Lanes([0.0, (BFO * BGX), 0.0]) + ((BFW - BGI) * BFN));
                    BGA = BHN;
                    BGB = BHO;
                } else {
                    BGA = RD;
                    BGB = BEO;
                }
                BFD = BGA;
                BFE = BGB;
            } else {
                let BFC = if DN > RD { 1.0 } else { 0.0 };
                let BIJ;
                let BIK;
                if BFC != 0.0 {
                    let BHP = FO - ((-8.754687373538999e-1f64 / NS).exp());
                    let BHQ = DO * BHP;
                    let BHR = BHQ - W;
                    let BHS = Lanes([0.0, (FB * BHP), 0.0]);
                    let BHT = Lanes([X[0], 0.0, X[1]]);
                    let BHU = BHR * CI;
                    let BHV = ((BHS - BHT) * CI) + Lanes([0.0, (DV * BHR), 0.0]);
                    let BHW = BHV * BHU;
                    let BHX = ((BHU * BHU) + UD).sqrt();
                    let BHY = (BHU + BHX) * GW;
                    let BHZ = BHQ - (CJ * BHY);
                    let BIA = BHS - (Lanes([0.0, (DW * BHY), 0.0]) + (((BHV + ((BHW + BHW) * (FX / (GV * BHX)))) * GW) * CJ));
                    let BIB = BHZ / DO;
                    let BIC = FO - BIB;
                    let BID = FO - NS;
                    let BIE = ((BIC.ln()) * BID).exp();
                    let BIF = FO - BIE;
                    let BIG = ((DO * BIF) / BID) + (RP * (W - BHZ));
                    let BIH = DN * BIG;
                    let BII = Lanes([0.0, (FA * BIG), 0.0]) + ((((Lanes([0.0, (FB * BIF), 0.0]) + ((((((((BIA - Lanes([0.0, (FB * BIB), 0.0])) / DO) * FQ) * (FX / BIC)) * BID) * BIE) * FQ) * DO)) / BID) + ((BHT - BIA) * RP)) * DN);
                    BIJ = BIH;
                    BIK = BII;
                } else {
                    BIJ = RD;
                    BIK = BEO;
                }
                BFD = BIJ;
                BFE = BIK;
            }
            let BIQ;
            let BIR;
            if BFF != 0.0 {
                let BIL = U * AUN;
                let BIM = CH - L;
                let BIN = (Lanes([DU, 0.0, 0.0]) - AME) * BBF;
                let BIO = (T * AUN) + (BIM * BBF);
                let BIP = (Lanes([0.0, BIL[0], BIL[1], BIL[2]]) + (AUO * T)) + (Lanes([BIN[0], BIN[1], BIN[2], 0.0]) + (BBG * BIM));
                BIQ = BIO;
                BIR = BIP;
            } else {
                BIQ = RD;
                BIR = AOY;
            }
            let BJP;
            let BJQ;
            let BJR;
            let BJS;
            let BJT;
            let BJU;
            let BJV;
            let BJW;
            let BJX;
            let BJY;
            let BJZ;
            let BKA;
            if BIS != 0.0 {
                let BIU = BIT - AVF;
                let BIW = Lanes([0.0, 0.0, 0.0, 0.0, BIV]);
                let BIX = BIW - Lanes([AVG[0], AVG[1], AVG[2], AVG[3], 0.0]);
                let BIZ = (BIY * BIT) * LH;
                let BJA = (BIV * BIY) * LH;
                let BJC = BJB - AUL;
                let BJE = Lanes([0.0, 0.0, 0.0, 0.0, BJD]);
                let BJF = BJE - Lanes([AUM[0], AUM[1], AUM[2], AUM[3], 0.0]);
                let BJH = (BJG * BJB) * LH;
                let BJI = (BJD * BJG) * LH;
                BJP = BIT;
                BJQ = BJB;
                BJR = BIU;
                BJS = BIZ;
                BJT = BJC;
                BJU = BJH;
                BJV = BIW;
                BJW = BJE;
                BJX = BIX;
                BJY = BJA;
                BJZ = BJF;
                BKA = BJI;
            } else {
                let BJJ = Lanes([AVG[0], AVG[1], AVG[2], AVG[3], 0.0]);
                let BJK = Lanes([AUM[0], AUM[1], AUM[2], AUM[3], 0.0]);
                let BJL = Lanes([0.0, 0.0, 0.0, 0.0, BIV]);
                let BJM = Lanes([0.0, 0.0, 0.0, 0.0, BJD]);
                BJP = AVF;
                BJQ = AUL;
                BJR = BIT;
                BJS = RD;
                BJT = BJB;
                BJU = RD;
                BJV = BJJ;
                BJW = BJK;
                BJX = BJL;
                BJY = BJN;
                BJZ = BJM;
                BKA = BJO;
            }
            let BKD = E * BEP;
            let BKE = BEQ * E;
            let BKF = E * BFD;
            let BKG = BFE * E;
            let BKH = E * QY;
            let BKI = RB * E;
            let BKJ = E * (BKB * F);
            let BKK = (G * BKB) * E;
            let BKL = E * (BKC * AA);
            let BKM = (AB * BKC) * E;
            let BKN = E * (AXA - BBF);
            let BKO = (AXD - BBG) * E;
            let BKP = E * (ABP + AVI);
            let BKQ = (Lanes([ABQ[0], ABQ[1], ABQ[2], 0.0]) + AVJ) * E;
            let BKR = E * AWJ;
            let BKS = AWK * E;
            let BKT = E * (AID + BJP);
            let BKU = (Lanes([AIE[0], 0.0, AIE[1], AIE[2], 0.0]) + BJV) * E;
            let BKV = AUO * E;
            let BKW = E * (BJQ - AUI);
            let BKX = (BJW - Lanes([AUJ[0], AUJ[1], AUJ[2], AUJ[3], 0.0])) * E;
            let BKY = BBG * E;
            let BKZ = ctx.simparam_or("gmin", RD);
            let BLA = BKZ * O;
            let BLB = Q * BKZ;
            let BLC = ctx.simparam_or("gmin", RD);
            let BLD = BLC * I;
            let BLE = K * BLC;
            let BLF = E * BEG;
            let BLG = BEH * E;
            let BLH = ddt(12300, BKF);
            let BLJ = BKG * BLI;
            let BLK = ddt(12302, BKH);
            let BLL = BKI * BLI;
            let BLM = ddt(12304, BKJ);
            let BLN = BKK * BLI;
            let BLO = ddt(12306, BKL);
            let BLP = BKM * BLI;
            let BLU;
            let BLV;
            if BLQ != 0.0 {
                let BLR = AC / DP;
                let BLS = (Lanes([AD[0], 0.0, AD[1]]) - Lanes([0.0, (FC * BLR), 0.0])) / DP;
                BLU = BLR;
                BLV = BLS;
            } else {
                BLU = RD;
                BLV = BLT;
            }
            let BMA;
            let BMB;
            if BLW != 0.0 {
                let BLX = AE / DQ;
                let BLY = (Lanes([AG[0], 0.0, AG[1]]) - Lanes([0.0, (FD * BLX), 0.0])) / DQ;
                BMA = BLX;
                BMB = BLY;
            } else {
                BMA = RD;
                BMB = BLZ;
            }
            let BMH;
            let BMI;
            if BMC != 0.0 {
                let BMD = AH / BCZ;
                let BME = BDA * BMD;
                let BMF = (Lanes([AI[0], 0.0, 0.0, AI[1], 0.0]) - Lanes([0.0, BME[0], BME[1], BME[2], BME[3]])) / BCZ;
                BMH = BMD;
                BMI = BMF;
            } else {
                BMH = RD;
                BMI = BMG;
            }
            let BMJ = ddt(12334, BKP);
            let BMK = BKQ * BLI;
            let BML = ddt(12337, BKT);
            let BMM = BKU * BLI;
            let BMT;
            let BMU;
            let BMV;
            let BMW;
            if BMN != 0.0 {
                BMT = RD;
                BMU = RD;
                BMV = CE;
                BMW = CE;
            } else {
                let BMP = BMO * AO;
                let BMQ = KT * BMO;
                let BMR = ddt(12350, BMP);
                let BMS = BMQ * BLI;
                BMT = BMR;
                BMU = BMP;
                BMV = BMS;
                BMW = BMQ;
            }
            let BNB;
            let BNC;
            let BND;
            let BNE;
            let BNF;
            let BNG;
            if BMX != 0.0 {
                BNB = RD;
                BNC = RD;
                BND = RD;
                BNE = AOY;
                BNF = CE;
                BNG = CE;
            } else {
                let BMY = AO / DR;
                let BMZ = BMY - BIQ;
                let BNA = Lanes([((KT - (FE * BMY)) / DR), 0.0, 0.0, 0.0]) - BIR;
                BNB = BMZ;
                BNC = BMT;
                BND = BMU;
                BNE = BNA;
                BNF = BMV;
                BNG = BMW;
            }
            let BNH = ddt(12367, BJS);
            let BNI = BJY * BLI;
            let BNJ = ddt(12370, BJU);
            let BNK = BKA * BLI;
            let BNL = if ((((((BLF + BLK) + BLM) + BLO) + BMH) + staged[22]) + branch_unknown_flows[2]) != RD { 1.0 } else { 0.0 };
            let BNM = BKS[2];
            let BNN = (-BNM) - (-BKY[3]);
            let BNO = if (BNN.abs()) > (ctx.simparam_or("gmin", RD)) { 1.0 } else { 0.0 };
            if BNO != 0.0 {
            } else {
                let BNP = if BNN >= RD { 1.0 } else { 0.0 };
            }
            let BNQ = (-AXB[1]) - (-BKY[1]);
            let BNR = if (BNQ.abs()) > (ctx.simparam_or("gmin", RD)) { 1.0 } else { 0.0 };
            if BNR != 0.0 {
            } else {
                let BNS = if BNQ >= RD { 1.0 } else { 0.0 };
            }
            let BNT = BKV[1];
            let BNU = if (BNT.abs()) > (ctx.simparam_or("gmin", RD)) { 1.0 } else { 0.0 };
            if BNU != 0.0 {
            } else {
                let BNV = if BNT >= RD { 1.0 } else { 0.0 };
            }
            let BNW = BLB[0];
            let BNX = BLB[1];
            let BNY = BLE[0];
            let BNZ = BLE[1];
            let BOA = BLG[0];
            let BOB = BLG[1];
            let BOC = BLG[2];
            let BOD = BLG[3];
            let BOE = BKE[0];
            let BOF = BKE[1];
            let BOG = BKE[2];
            let BOH = BLJ[0];
            let BOI = BLJ[1];
            let BOJ = BLJ[2];
            let BOK = BLL[0];
            let BOL = BLL[1];
            let BOM = BLL[2];
            let BON = BLN[0];
            let BOO = BLN[1];
            let BOP = BLP[0];
            let BOQ = BLP[1];
            let BOR = BLV[0];
            let BOS = BLV[1];
            let BOT = BLV[2];
            let BOU = BMB[0];
            let BOV = BMB[1];
            let BOW = BMB[2];
            let BOX = BMI[0];
            let BOY = BMI[1];
            let BOZ = BMI[2];
            let BPA = BMI[3];
            let BPB = BMI[4];
            let BPC = BKO[0];
            let BPD = BKO[1];
            let BPE = BKO[2];
            let BPF = BKO[3];
            let BPG = BMK[0];
            let BPH = BMK[1];
            let BPI = BMK[2];
            let BPJ = BMK[3];
            let BPK = BKS[0];
            let BPL = BKS[1];
            let BPM = BMM[0];
            let BPN = BMM[1];
            let BPO = BMM[2];
            let BPP = BMM[3];
            let BPQ = BMM[4];
            let BPR = BKX[0];
            let BPS = BKX[1];
            let BPT = BKX[2];
            let BPU = BKX[3];
            let BPV = BKX[4];
            let BPW = BNE[0];
            let BPX = BNE[1];
            let BPY = BNE[2];
            let BPZ = BNE[3];
            let BQA = BNF;
            let BQB = BJX[0];
            let BQC = BJX[1];
            let BQD = BJX[2];
            let BQE = BJX[3];
            let BQF = BJX[4];
            let BQG = BNI;
            let BQH = BJZ[0];
            let BQI = BJZ[1];
            let BQJ = BJZ[2];
            let BQK = BJZ[3];
            let BQL = BJZ[4];
            let BQM = BNK;
            let BQN = BKG[0];
            let BQO = BKG[1];
            let BQP = BKG[2];
            let BQQ = BKI[0];
            let BQR = BKI[1];
            let BQS = BKI[2];
            let BQT = BKK[0];
            let BQU = BKK[1];
            let BQV = BKM[0];
            let BQW = BKM[1];
            let BQX = BKQ[0];
            let BQY = BKQ[1];
            let BQZ = BKQ[2];
            let BRA = BKQ[3];
            let BRB = BKU[0];
            let BRC = BKU[1];
            let BRD = BKU[2];
            let BRE = BKU[3];
            let BRF = BKU[4];
            let BRG = BNG;
            let BRH = BJY;
            let BRI = BKA;
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(7),
            multiplicity * (BLA),
            [6, 7],
            [BNW, BNX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(5),
            multiplicity * (BLD),
            [5, 6],
            [BNY, BNZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * (BLF),
            [1, 3, 4, 5],
            [BOA, BOB, BOC, BOD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(5),
            multiplicity * (BKD),
            [3, 4, 5],
            [BOE, BOF, BOG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(5),
            multiplicity * (BLH),
            [3, 4, 5],
            [BOH, BOI, BOJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (BLK),
            [1, 4, 5],
            [BOK, BOL, BOM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (BLM),
            [1, 5],
            [BON, BOO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (BLO),
            [1, 2],
            [BOP, BOQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(2),
            multiplicity * (BLU),
            [2, 4, 7],
            [BOR, BOS, BOT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[89],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(0),
            multiplicity * (BMA),
            [0, 4, 5],
            [BOU, BOV, BOW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(0), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[90],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(6),
            multiplicity * (BMH),
            [1, 4, 5, 6, 7],
            [BOX, BOY, BOZ, BPA, BPB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(6), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[91],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(5),
            multiplicity * (BKN),
            [4, 5, 6, 7],
            [BPC, BPD, BPE, BPF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(5),
            multiplicity * (BMJ),
            [4, 5, 6, 7],
            [BPG, BPH, BPI, BPJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(7),
            multiplicity * (BKR),
            [4, 6, 7],
            [BPK, BPL, BNM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (BML),
            [4, 5, 6, 7, 8],
            [BPM, BPN, BPO, BPP, BPQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(7),
            multiplicity * (BKW),
            [4, 5, 6, 7, 9],
            [BPR, BPS, BPT, BPU, BPV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[92],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * (BNB),
            [4, 5, 6, 7],
            [BPW, BPX, BPY, BPZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (BNC),
            [4],
            [BQA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            None,
            multiplicity * (BJR),
            [4, 5, 6, 7, 8],
            [BQB, BQC, BQD, BQE, BQF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            None,
            multiplicity * (BNH),
            [8],
            [BQG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            None,
            multiplicity * (BJT),
            [4, 5, 6, 7, 9],
            [BQH, BQI, BQJ, BQK, BQL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (BNJ),
            [9],
            [BQM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(6),
            multiplicity * (staged[22]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(0),
            multiplicity * (staged[93]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(2),
            multiplicity * (staged[94]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (BRJ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (BRK),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (BRL),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = BLA;
        self.canonical_reactive[1] = BLD;
        self.canonical_reactive[2] = BLF;
        self.canonical_reactive[3] = BKD;
        self.canonical_reactive[4] = BKF;
        self.canonical_reactive[5] = BQN;
        self.canonical_reactive[6] = BQO;
        self.canonical_reactive[7] = BQP;
        self.canonical_reactive[8] = BKH;
        self.canonical_reactive[9] = BQQ;
        self.canonical_reactive[10] = BQR;
        self.canonical_reactive[11] = BQS;
        self.canonical_reactive[12] = BKJ;
        self.canonical_reactive[13] = BQT;
        self.canonical_reactive[14] = BQU;
        self.canonical_reactive[15] = BKL;
        self.canonical_reactive[16] = BQV;
        self.canonical_reactive[17] = BQW;
        self.canonical_reactive[18] = BLU;
        self.canonical_reactive[19] = staged[89];
        self.canonical_reactive[20] = BMA;
        self.canonical_reactive[21] = staged[90];
        self.canonical_reactive[22] = BMH;
        self.canonical_reactive[23] = staged[91];
        self.canonical_reactive[24] = BKN;
        self.canonical_reactive[25] = BKP;
        self.canonical_reactive[26] = BQX;
        self.canonical_reactive[27] = BQY;
        self.canonical_reactive[28] = BQZ;
        self.canonical_reactive[29] = BRA;
        self.canonical_reactive[30] = BKR;
        self.canonical_reactive[31] = BKT;
        self.canonical_reactive[32] = BRB;
        self.canonical_reactive[33] = BRC;
        self.canonical_reactive[34] = BRD;
        self.canonical_reactive[35] = BRE;
        self.canonical_reactive[36] = BRF;
        self.canonical_reactive[37] = BKW;
        self.canonical_reactive[38] = staged[92];
        self.canonical_reactive[39] = BNB;
        self.canonical_reactive[40] = BND;
        self.canonical_reactive[41] = BRG;
        self.canonical_reactive[42] = BJR;
        self.canonical_reactive[43] = BJS;
        self.canonical_reactive[44] = BRH;
        self.canonical_reactive[45] = BJT;
        self.canonical_reactive[46] = BJU;
        self.canonical_reactive[47] = BRI;
        self.canonical_reactive[48] = staged[22];
        self.canonical_reactive[49] = staged[93];
        self.canonical_reactive[50] = staged[94];
        self.canonical_reactive[51] = BRJ;
        self.canonical_reactive[52] = BRK;
        self.canonical_reactive[53] = BRL;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(5),
            &[3, 4, 5],
            &[cached[5], cached[6], cached[7]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 4, 5],
            &[cached[9], cached[10], cached[11]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 5],
            &[cached[13], cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[16], cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(5),
            &[4, 5, 6, 7],
            &[cached[26], cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(7),
            &[4, 5, 6, 7, 8],
            &[cached[32], cached[33], cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[41]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            None,
            &[8],
            &[cached[44]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            None,
            &[9],
            &[cached[47]],
            &[],
            &[],
            multiplicity,
        );
    }

}
