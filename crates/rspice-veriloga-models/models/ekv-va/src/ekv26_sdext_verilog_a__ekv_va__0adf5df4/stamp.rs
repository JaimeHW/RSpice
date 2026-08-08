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
        let mut key = Vec::with_capacity(150);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[2] = values[0];
        self.canonical_staged[25] = values[1];
        self.canonical_staged[40] = values[2];
        self.canonical_staged[33] = values[3];
        self.canonical_staged[69] = values[4];
        self.canonical_staged[32] = values[5];
        self.canonical_staged[70] = values[6];
        self.canonical_staged[71] = values[7];
        self.canonical_staged[72] = values[8];
        self.canonical_staged[0] = values[9];
        self.canonical_staged[1] = values[10];
        self.canonical_staged[3] = values[11];
        self.canonical_staged[7] = values[12];
        self.canonical_staged[73] = values[13];
        self.canonical_staged[4] = values[14];
        self.canonical_staged[74] = values[15];
        self.canonical_staged[5] = values[16];
        self.canonical_staged[75] = values[17];
        self.canonical_staged[6] = values[18];
        self.canonical_staged[76] = values[19];
        self.canonical_staged[8] = values[20];
        self.canonical_staged[77] = values[21];
        self.canonical_staged[10] = values[22];
        self.canonical_staged[19] = values[23];
        self.canonical_staged[18] = values[24];
        self.canonical_staged[15] = values[25];
        self.canonical_staged[16] = values[26];
        self.canonical_staged[27] = values[27];
        self.canonical_staged[29] = values[28];
        self.canonical_staged[78] = values[29];
        self.canonical_staged[36] = values[30];
        self.canonical_staged[38] = values[31];
        self.canonical_staged[39] = values[32];
        self.canonical_staged[44] = values[33];
        self.canonical_staged[79] = values[34];
        self.canonical_staged[80] = values[35];
        self.canonical_staged[81] = values[36];
        self.canonical_staged[82] = values[37];
        self.canonical_staged[45] = values[38];
        self.canonical_staged[46] = values[39];
        self.canonical_staged[47] = values[40];
        self.canonical_staged[50] = values[41];
        self.canonical_staged[58] = values[42];
        self.canonical_staged[59] = values[43];
        self.canonical_staged[83] = values[44];
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
                let A = 1.0359399871014713e-10f64;
                let B = parameters[13];
                let H = parameters[35];
                let J = parameters[22];
                let L = parameters[30];
                let N = 0e0f64;
                let P = 5e-1f64;
                let Q = 3.333333333333e-1f64;
                let S = parameters[3];
                let U = 2.7315e2f64;
                let W = parameters[4];
                let Y = 2.9815e2f64;
                let AD = parameters[27];
                let AG = 1e0f64;
                let AI = parameters[38];
                let AJ = 1e-6f64;
                let AM = parameters[39];
                let AR = parameters[40];
                let AT = parameters[17];
                let AX = parameters[8];
                let AY = 1e-1f64;
                let BN = parameters[37];
                let BQ = parameters[1];
                let BR = 0e0f64;
                let BT = parameters[9];
                let BW = 2e0f64;
                let BZ = parameters[11];
                let CB = 4e0f64;
                let CE = parameters[10];
                let CI = parameters[12];
                let mut oV = 0.0;
                let mut oAK = 0.0;
                let mut oAL = 0.0;
                let mut oAO = 0.0;
                let mut oAP = 0.0;
                let mut oAQ = 0.0;
                let C = A / B;
                let D = (C * parameters[14]).sqrt();
                let E = D * parameters[25];
                let F = (3e0f64 * C) * parameters[28];
                let G = C * parameters[29];
                let I = H + H;
                let K = B / (A * J);
                let M = (L + L) / B;
                let O = if parameters[0] > N { 1.0 } else { 0.0 };
                let R = if O != 0.0 {
                    P
                } else {
                    Q
                };
                let T = if S == 1e21f64 { 1.0 } else { 0.0 };
                if T != 0.0 {
                } else {
                    let V = S + U;
                    oV = V;
                }
                let X = if W == 1e21f64 { 1.0 } else { 0.0 };
                let AA = if X != 0.0 {
                    Y
                } else {
                    let Z = W + U;
                    Z
                };
                let AB = 1.16e0f64 - (((7.02e-4f64 * AA) * AA) / (AA + 1.108e3f64));
                let AC = parameters[5] + parameters[26];
                let AE = parameters[6] + AD;
                let AF = AE * AC;
                let AH = AG / (AF.sqrt());
                if O != 0.0 {
                    let AK = if AI != AJ { 1.0 } else { 0.0 };
                    oAK = AK;
                    if AK != 0.0 {
                        let AO = AH * (AI - AJ);
                        oAO = AO;
                    } else {
                    }
                } else {
                    let AL = if AI != AJ { 1.0 } else { 0.0 };
                    oAL = AL;
                    if AL != 0.0 {
                        let AP = AH * (AJ - AI);
                        oAP = AP;
                    } else {
                    }
                }
                let AN = if AM != AJ { 1.0 } else { 0.0 };
                if AN != 0.0 {
                    let AQ = AG + ((AM - AJ) * AH);
                    oAQ = AQ;
                } else {
                }
                let AS = if AR != AJ { 1.0 } else { 0.0 };
                let AV = if AS != 0.0 {
                    let AU = AT + ((AR - AJ) * AH);
                    AU
                } else {
                    AT
                };
                let AW = if M == N { 1.0 } else { 0.0 };
                let BC = if AW != 0.0 {
                    N
                } else {
                    let AZ = 2.8e-1f64 * ((AC / (parameters[31] * AX)) - AY);
                    let BA = AG / (AG + (P * (AZ + (((AZ * AZ) + 1.936e-3f64).sqrt()))));
                    let BB = (M * BA) * BA;
                    BB
                };
                let BD = (F * parameters[7]) / AE;
                let BE = (G * AX) / AC;
                let BF = (2.5e-1f64 * AV) * AV;
                let BG = P * AV;
                let BH = AY * AC;
                let BI = BH * BH;
                let BJ = -5e-1f64 * AV;
                let BK = if J == N { 1.0 } else { 0.0 };
                let BL = -BE;
                let BM = -AV;
                let BO = (parameters[36] * BN) / (AE - AD);
                let BP = AF * B;
                let BS = if BQ != 0.0 {
                    BR
                } else {
                    N
                };
                let BU = if BN > N { 1.0 } else { 0.0 };
                let BV = if (if BT == N { 1.0 } else { 0.0 }) != 0.0 && BU != 0.0 { 1.0 } else { 0.0 };
                let BY = if BV != 0.0 {
                    let BX = (BW * BN) * AE;
                    BX
                } else {
                    BT
                };
                let CA = if (if BZ == N { 1.0 } else { 0.0 }) != 0.0 && BU != 0.0 { 1.0 } else { 0.0 };
                let CD = if CA != 0.0 {
                    let CC = (CB * BN) + AE;
                    CC
                } else {
                    BZ
                };
                let CF = if (if CE == N { 1.0 } else { 0.0 }) != 0.0 && BU != 0.0 { 1.0 } else { 0.0 };
                let CH = if CF != 0.0 {
                    let CG = (BW * BN) * AE;
                    CG
                } else {
                    CE
                };
                let CJ = if (if CI == N { 1.0 } else { 0.0 }) != 0.0 && BU != 0.0 { 1.0 } else { 0.0 };
                let CL = if CJ != 0.0 {
                    let CK = (CB * BN) + AE;
                    CK
                } else {
                    CI
                };
                let CM = AB / (AA * 8.617333262e-5f64);
                let CN = -AE;
            [D, E, I, K, O, R, T, oV, X, AA, AB, AC, AE, oAK, oAO, oAL, oAP, AN, oAQ, AS, AV, AW, BC, BD, BE, BF, BG, BI, BJ, BK, BL, BM, BO, BP, BV, CA, CF, CJ, CM, CH, CL, CN, BY, CD, BS]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
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
        let produced: [f64; 39] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = staged[69];
                let B = staged[70];
                let D = staged[71];
                let H = 1e0f64;
                let P = staged[0];
                let X = 2e-1f64;
                let Z = 5e-1f64;
                let AD = staged[2];
                let AJ = staged[73];
                let AK = staged[74];
                let AM = staged[75];
                let AU = staged[7];
                let AZ = staged[78];
                let BD = parameters[43];
                let mut oBB = 0.0;
                let E = if B != 0.0 {
                    let C = temperature + parameters[2];
                    C
                } else {
                    D
                };
                let F = E * 8.617333262e-5f64;
                let G = 1e-1f64 * F;
                let I = H / F;
                let J = F + F;
                let K = J + J;
                let L = F * F;
                let M = L + L;
                let N = 1.6e1f64 * L;
                let O = 1.16e0f64 - (((7.02e-4f64 * E) * E) / (E + 1.108e3f64));
                let Q = E - P;
                let R = E / P;
                let S = parameters[15] - (parameters[16] * Q);
                let T = parameters[19] * (R.powf(parameters[20]));
                let U = parameters[23] * (R.powf(parameters[24]));
                let V = parameters[33] * (H + (parameters[34] * Q));
                let W = R.ln();
                let Y = ((((parameters[18] * R) - ((3e0f64 * F) * W)) - (staged[1] * R)) + O) - X;
                let AA = (Z * (Y + (((Y * Y) + L).sqrt()))) + X;
                let AB = AA.sqrt();
                let AC = H / U;
                let AE = AD * U;
                let AF = AD * V;
                let AG = parameters[32] / V;
                let AH = U * staged[3];
                let AI = F * ((((Z * AH) * I).ln()) - 6e-1f64);
                let AL;
                if A != 0.0 {
                    let AO = if AJ != 0.0 {
                        let AN = staged[4] + S;
                        AN
                    } else {
                        S
                    };
                    AL = AO;
                } else {
                    let AR = if AK != 0.0 {
                        let AP = staged[5] - S;
                        AP
                    } else {
                        let AQ = -S;
                        AQ
                    };
                    AL = AR;
                }
                let AT = if AM != 0.0 {
                    let AS = T * staged[6];
                    AS
                } else {
                    T
                };
                let AV = AU * AT;
                let AW = staged[8] * AB;
                let AX = 2e0f64 * N;
                let AY = F / AH;
                if AZ != 0.0 {
                } else {
                    let BB = AV * (H + (staged[33] * AW));
                    oBB = BB;
                }
                let BA = (K + K) * parameters[25];
                let BC = if AG > 0e0f64 { 1.0 } else { 0.0 };
                let BE = (((staged[45] - (O / F)) + (parameters[65] * W)) / BD).exp();
                let BF = parameters[44] * BE;
                let BG = parameters[45] * BE;
                let BH = parameters[46] * BE;
                let BI = parameters[50] - (parameters[69] * Q);
                let BJ = parameters[51] - (parameters[70] * Q);
                let BK = parameters[52] - (parameters[71] * Q);
                let BL = parameters[53] * (H + (parameters[66] * Q));
                let BM = parameters[54] * (H + (parameters[67] * Q));
                let BN = parameters[55] * (H + (parameters[68] * Q));
                let BO = R - H;
                let BP = parameters[59] * (H + (BO * parameters[72]));
                let BQ = parameters[60] * (H + (BO * parameters[73]));
                let BR = parameters[61] * (H + (BO * parameters[74]));
                let BS = BF * staged[46];
                let BT = BG * staged[47];
                let BU = BH * AU;
                let BV = (BS + BT) + BU;
                let BW = F * BD;
                let BX = staged[50] * BH;
                let BY = F * BR;
                let BZ = F * BQ;
                let CA = F * BP;
                let CB = BF * staged[58];
                let CC = BG * staged[59];
                let CD = (CB + CC) + BU;
            [F, G, I, K, M, N, R, AA, AC, AE, AF, AG, AH, AI, AV, AW, AL, AX, AY, oBB, BA, BC, BI, BJ, BK, BL, BM, BN, BS, BT, BV, BW, BX, BY, BZ, CA, CB, CC, CD]
        };
        self.canonical_staged[28] = produced[0];
        self.canonical_staged[17] = produced[1];
        self.canonical_staged[20] = produced[2];
        self.canonical_staged[35] = produced[3];
        self.canonical_staged[30] = produced[4];
        self.canonical_staged[14] = produced[5];
        self.canonical_staged[48] = produced[6];
        self.canonical_staged[11] = produced[7];
        self.canonical_staged[26] = produced[8];
        self.canonical_staged[24] = produced[9];
        self.canonical_staged[42] = produced[10];
        self.canonical_staged[43] = produced[11];
        self.canonical_staged[22] = produced[12];
        self.canonical_staged[23] = produced[13];
        self.canonical_staged[31] = produced[14];
        self.canonical_staged[12] = produced[15];
        self.canonical_staged[9] = produced[16];
        self.canonical_staged[13] = produced[17];
        self.canonical_staged[21] = produced[18];
        self.canonical_staged[34] = produced[19];
        self.canonical_staged[37] = produced[20];
        self.canonical_staged[41] = produced[21];
        self.canonical_staged[64] = produced[22];
        self.canonical_staged[66] = produced[23];
        self.canonical_staged[68] = produced[24];
        self.canonical_staged[63] = produced[25];
        self.canonical_staged[65] = produced[26];
        self.canonical_staged[67] = produced[27];
        self.canonical_staged[56] = produced[28];
        self.canonical_staged[54] = produced[29];
        self.canonical_staged[57] = produced[30];
        self.canonical_staged[49] = produced[31];
        self.canonical_staged[52] = produced[32];
        self.canonical_staged[51] = produced[33];
        self.canonical_staged[53] = produced[34];
        self.canonical_staged[55] = produced[35];
        self.canonical_staged[61] = produced[36];
        self.canonical_staged[60] = produced[37];
        self.canonical_staged[62] = produced[38];
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
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 4749 => 0usize, 4751 => 1usize, 4775 => 2usize, 5312 => 3usize, 5403 => 4usize, _ => usize::MAX };
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
            let A = node_potentials[3];
            let B = 1e0f64;
            let C = 1e0f64;
            let D = parameters[0];
            let G = 1e0f64;
            let J = 1e0f64;
            let O = 0e0f64;
            let Q = -1e0f64;
            let R = 1e0f64;
            let X = staged[11];
            let AB = 2e0f64;
            let AC = 1e0f64;
            let AD = 5e-1f64;
            let AI = staged[14];
            let AV = staged[8];
            let AW = staged[17];
            let AY = staged[18];
            let AZ = -1e0f64;
            let BB = staged[19];
            let BK = 2.5e-1f64;
            let BT = staged[20];
            let BX = 1.3e0f64;
            let BY = 1.6e0f64;
            let CB = 2e0f64;
            let CT = staged[21];
            let CW = staged[22];
            let DD = parameters[25];
            let DR = 7.5e-1f64;
            let EJ = 1.55e0f64;
            let FB = 1e-64f64;
            let FX = staged[24];
            let FZ = staged[25];
            let GA = staged[26];
            let IU = staged[28];
            let IX = 1.33333332e0f64;
            let JC = staged[29];
            let JF = staged[78];
            let KD = staged[30];
            let KI = parameters[21];
            let KO = 0e0f64;
            let KP = Lanes([0e0f64; 4]);
            let KQ = staged[32];
            let LU = staged[36];
            let MR = 4e0f64;
            let ND = staged[37];
            let PO = 6.6666666e-1f64;
            let QT = staged[33];
            let SG = staged[39];
            let SL = staged[40];
            let TF = staged[44];
            let TI = 3e0f64;
            let TJ = 6e0f64;
            let TM = 2.66666666e-1f64;
            let TZ = -5e-1f64;
            let UJ = ddt_scale();
            let UO = -3.5e1f64;
            let US = staged[43];
            let WQ = staged[48];
            let WR = staged[49];
            let WV = -4e1f64;
            let WW = Lanes([0e0f64; 2]);
            let WZ = parameters[58];
            let XB = 7e1f64;
            let XE = parameters[57];
            let XL = staged[51];
            let XM = parameters[64];
            let XO = 1e-3f64;
            let XS = staged[52];
            let XT = staged[53];
            let XU = parameters[63];
            let XZ = staged[54];
            let YA = staged[55];
            let YB = parameters[62];
            let YG = staged[56];
            let YI = staged[57];
            let YK = parameters[56];
            let YL = parameters[7];
            let YS = -4e1f64;
            let YT = Lanes([0e0f64; 2]);
            let ZN = staged[60];
            let ZS = staged[61];
            let ZU = staged[62];
            let ZZ = staged[63];
            let AAA = staged[46];
            let AAC = parameters[47];
            let AAE = staged[64];
            let AAJ = staged[65];
            let AAK = staged[47];
            let AAM = parameters[48];
            let AAO = staged[66];
            let AAT = staged[67];
            let AAU = staged[7];
            let AAW = parameters[49];
            let AAY = staged[68];
            let ACA = staged[58];
            let ACH = staged[59];
            let E = D * (node_potentials[1] - A);
            let F = (Lanes([B, 0.0]) - Lanes([0.0, C])) * D;
            let H = D * (node_potentials[2] - A);
            let I = (Lanes([G, 0.0]) - Lanes([0.0, C])) * D;
            let K = D * (node_potentials[0] - A);
            let L = (Lanes([J, 0.0]) - Lanes([0.0, C])) * D;
            let M = Lanes([L[0], 0.0, L[1]]);
            let N = Lanes([0.0, I[0], I[1]]);
            let P = if (K - H) < O { 1.0 } else { 0.0 };
            let S;
            let T;
            let U;
            let V;
            let W;
            if P != 0.0 {
                S = K;
                T = H;
                U = Q;
                V = M;
                W = N;
            } else {
                S = H;
                T = K;
                U = R;
                V = N;
                W = M;
            }
            let Y = (((E - staged[9]) - staged[10]) + X) + staged[12];
            let Z = F * Y;
            let AA = ((Y * Y) + staged[13]).sqrt();
            let AE = AD * (Y + AA);
            let AF = (F + ((Z + Z) * (AC / (AB * AA)))) * AD;
            let AG = X + S;
            let AH = V * AG;
            let AJ = ((AG * AG) + AI).sqrt();
            let AK = (AH + AH) * (AC / (AB * AJ));
            let AL = (AD * (AG + AJ)).sqrt();
            let AM = ((V + AK) * AD) * (AC / (AB * AL));
            let AN = X + T;
            let AO = W * AN;
            let AP = ((AN * AN) + AI).sqrt();
            let AQ = (AO + AO) * (AC / (AB * AP));
            let AR = (AD * (AN + AP)).sqrt();
            let AS = ((W + AQ) * AD) * (AC / (AB * AR));
            let AT = (AE + staged[15]).sqrt();
            let AU = AE - X;
            let AX = (((AU - (AV * (AT - staged[16]))) + X) + AW).sqrt();
            let BA = ((AM + AS) * AY) * AZ;
            let BC = ((AF - ((AF * (AC / (AB * AT))) * AV)) * (AC / (AB * AX))) * BB;
            let BD = (AV - (AY * (AL + AR))) + (BB * AX);
            let BE = Lanes([BA[0], 0.0, BA[1], BA[2]]) + Lanes([0.0, BC[0], 0.0, BC[1]]);
            let BF = BE * BD;
            let BG = ((BD * BD) + AW).sqrt();
            let BH = (BF + BF) * (AC / (AB * BG));
            let BI = AD * (BD + BG);
            let BJ = (BE + BH) * AD;
            let BL = BK * BI;
            let BM = Lanes([0.0, AF[0], 0.0, AF[1]]);
            let BN = (AE + (BL * BI)).sqrt();
            let BO = (BM + (((BJ * BK) * BI) + (BJ * BL))) * (AC / (AB * BN));
            let BP = BN - (AD * BI);
            let BQ = AU - (BI * BP);
            let BR = BM - ((BJ * BP) + ((BO - (BJ * AD)) * BI));
            let BS = Lanes([V[0], 0.0, V[1], V[2]]);
            let BU = (BQ - S) * BT;
            let BV = (BR - BS) * BT;
            let BW = if BU > -3.5e-1f64 { 1.0 } else { 0.0 };
            let CM;
            let CN;
            if BW != 0.0 {
                let BZ = BU + BY;
                let CA = (BX + BU) - (BZ.ln());
                let CC = CB / CA;
                let CD = (((BV - (BV * (AC / BZ))) * CC) * AZ) / CA;
                let CE = R + BU;
                let CF = CE + (CC.ln());
                let CG = (CB + CC) / CF;
                let CH = (CD - ((BV + (CD * (AC / CC))) * CG)) / CF;
                let CI = CB + CG;
                let CJ = (CE + (CG.ln())) / CI;
                let CK = ((BV + (CH * (AC / CG))) - (CH * CJ)) / CI;
                CM = CJ;
                CN = CK;
            } else {
                let CL = if BU > -1.5e1f64 { 1.0 } else { 0.0 };
                let ET;
                let EU;
                if CL != 0.0 {
                    let EH = (-BU).exp();
                    let EI = (BV * AZ) * EH;
                    let EK = EJ + EH;
                    let EL = R + BU;
                    let EM = EL + (EK.ln());
                    let EN = (CB + EK) / EM;
                    let EO = (EI - ((BV + (EI * (AC / EK))) * EN)) / EM;
                    let EP = CB + EN;
                    let EQ = (EL + (EN.ln())) / EP;
                    let ER = ((BV + (EO * (AC / EN))) - (EO * EQ)) / EP;
                    ET = EQ;
                    EU = ER;
                } else {
                    let ES = if BU > -2.3e1f64 { 1.0 } else { 0.0 };
                    let FD;
                    let FE;
                    if ES != 0.0 {
                        let EV = (-BU).exp();
                        let EW = CB + EV;
                        let EX = R / EW;
                        let EY = ((((BV * AZ) * EV) * EX) * AZ) / EW;
                        FD = EX;
                        FE = EY;
                    } else {
                        let EZ = BU.exp();
                        let FA = BV * EZ;
                        let FC = EZ + FB;
                        FD = FC;
                        FE = FA;
                    }
                    ET = FD;
                    EU = FE;
                }
                CM = ET;
                CN = EU;
            }
            let CO = R + CM;
            let CP = CM * CO;
            let CQ = (CN * CO) + (CN * CM);
            let CR = CP.sqrt();
            let CS = CQ * (AC / (AB * CR));
            let CU = (BK + (CR * CT)).sqrt();
            let CV = (CS * CT) * (AC / (AB * CU));
            let CX = CW * (CU - AD);
            let CY = CV * CW;
            let CZ = T - S;
            let DA = W - V;
            let DB = AD * CZ;
            let DC = DA * AD;
            let DE = AI * ((DD * (CR - (CX * BT))) + 1.5625e-2f64);
            let DF = ((CS - (CY * BT)) * DD) * AI;
            let DG = CY * CX;
            let DH = ((CX * CX) + DE).sqrt();
            let DI = ((DG + DG) + DF) * (AC / (AB * DH));
            let DJ = DB - CX;
            let DK = Lanes([DC[0], 0.0, DC[1], DC[2]]);
            let DL = DK - CY;
            let DM = DL * DJ;
            let DN = ((DJ * DJ) + DE).sqrt();
            let DO = ((DM + DM) + DF) * (AC / (AB * DN));
            let DP = DH - DN;
            let DQ = DI - DO;
            let DS = (BK + ((CR - (DR * (CP.ln()))) * CT)).sqrt();
            let DT = ((CS - ((CQ * (AC / CP)) * DR)) * CT) * (AC / (AB * DS));
            let DU = DT * CW;
            let DV = (CW * (DS - AD)) + staged[23];
            let DW = DB - DV;
            let DX = DK - DU;
            let DY = DU * DV;
            let DZ = ((DV * DV) + DE).sqrt();
            let EA = ((DY + DY) + DF) * (AC / (AB * DZ));
            let EB = DX * DW;
            let EC = ((DW * DW) + DE).sqrt();
            let ED = ((EB + EB) + DF) * (AC / (AB * EC));
            let EE = ((((BQ - DB) - S) - DZ) + EC) * BT;
            let EF = ((((BR - DK) - BS) - EA) + ED) * BT;
            let EG = if EE > -3.5e-1f64 { 1.0 } else { 0.0 };
            let FR;
            let FS;
            if EG != 0.0 {
                let FF = EE + BY;
                let FG = (BX + EE) - (FF.ln());
                let FH = CB / FG;
                let FI = (((EF - (EF * (AC / FF))) * FH) * AZ) / FG;
                let FJ = R + EE;
                let FK = FJ + (FH.ln());
                let FL = (CB + FH) / FK;
                let FM = (FI - ((EF + (FI * (AC / FH))) * FL)) / FK;
                let FN = CB + FL;
                let FO = (FJ + (FL.ln())) / FN;
                let FP = ((EF + (FM * (AC / FL))) - (FM * FO)) / FN;
                FR = FO;
                FS = FP;
            } else {
                let FQ = if EE > -1.5e1f64 { 1.0 } else { 0.0 };
                let GW;
                let GX;
                if FQ != 0.0 {
                    let GL = (-EE).exp();
                    let GM = (EF * AZ) * GL;
                    let GN = EJ + GL;
                    let GO = R + EE;
                    let GP = GO + (GN.ln());
                    let GQ = (CB + GN) / GP;
                    let GR = (GM - ((EF + (GM * (AC / GN))) * GQ)) / GP;
                    let GS = CB + GQ;
                    let GT = (GO + (GQ.ln())) / GS;
                    let GU = ((EF + (GR * (AC / GQ))) - (GR * GT)) / GS;
                    GW = GT;
                    GX = GU;
                } else {
                    let GV = if EE > -2.3e1f64 { 1.0 } else { 0.0 };
                    let HF;
                    let HG;
                    if GV != 0.0 {
                        let GY = (-EE).exp();
                        let GZ = CB + GY;
                        let HA = R / GZ;
                        let HB = ((((EF * AZ) * GY) * HA) * AZ) / GZ;
                        HF = HA;
                        HG = HB;
                    } else {
                        let HC = EE.exp();
                        let HD = EF * HC;
                        let HE = HC + FB;
                        HF = HE;
                        HG = HD;
                    }
                    GW = HF;
                    GX = HG;
                }
                FR = GW;
                FS = GX;
            }
            let FT = R + FR;
            let FU = FR * FT;
            let FV = (FS * FT) + (FS * FR);
            let FW = DK - DQ;
            let FY = R + ((DB - DP) / FX);
            let GB = (staged[3] - (FZ * (FY.ln()))) + ((DB + DP) * GA);
            let GC = ((((FW / FX) * (AC / FY)) * FZ) * AZ) + ((DK + DQ) * GA);
            let GD = GC * GB;
            let GE = ((GB * GB) + staged[27]).sqrt();
            let GF = (GD + GD) * (AC / (AB * GE));
            let GG = AD * (GB + GE);
            let GH = (GC + GF) * AD;
            let GI = (BQ - T) * BT;
            let GJ = (BR - Lanes([W[0], 0.0, W[1], W[2]])) * BT;
            let GK = if GI > -3.5e-1f64 { 1.0 } else { 0.0 };
            let HT;
            let HU;
            if GK != 0.0 {
                let HH = GI + BY;
                let HI = (BX + GI) - (HH.ln());
                let HJ = CB / HI;
                let HK = (((GJ - (GJ * (AC / HH))) * HJ) * AZ) / HI;
                let HL = R + GI;
                let HM = HL + (HJ.ln());
                let HN = (CB + HJ) / HM;
                let HO = (HK - ((GJ + (HK * (AC / HJ))) * HN)) / HM;
                let HP = CB + HN;
                let HQ = (HL + (HN.ln())) / HP;
                let HR = ((GJ + (HO * (AC / HN))) - (HO * HQ)) / HP;
                HT = HQ;
                HU = HR;
            } else {
                let HS = if GI > -1.5e1f64 { 1.0 } else { 0.0 };
                let JR;
                let JS;
                if HS != 0.0 {
                    let JG = (-GI).exp();
                    let JH = (GJ * AZ) * JG;
                    let JI = EJ + JG;
                    let JJ = R + GI;
                    let JK = JJ + (JI.ln());
                    let JL = (CB + JI) / JK;
                    let JM = (JH - ((GJ + (JH * (AC / JI))) * JL)) / JK;
                    let JN = CB + JL;
                    let JO = (JJ + (JL.ln())) / JN;
                    let JP = ((GJ + (JM * (AC / JL))) - (JM * JO)) / JN;
                    JR = JO;
                    JS = JP;
                } else {
                    let JQ = if GI > -2.3e1f64 { 1.0 } else { 0.0 };
                    let KA;
                    let KB;
                    if JQ != 0.0 {
                        let JT = (-GI).exp();
                        let JU = CB + JT;
                        let JV = R / JU;
                        let JW = ((((GJ * AZ) * JT) * JV) * AZ) / JU;
                        KA = JV;
                        KB = JW;
                    } else {
                        let JX = GI.exp();
                        let JY = GJ * JX;
                        let JZ = JX + FB;
                        KA = JZ;
                        KB = JY;
                    }
                    JR = KA;
                    JS = KB;
                }
                HT = JR;
                HU = JS;
            }
            let HV = R + HT;
            let HW = (HU * HV) + (HU * HT);
            let HX = BK + CP;
            let HY = BK + (HT * HV);
            let HZ = HX.sqrt();
            let IA = CQ * (AC / (AB * HZ));
            let IB = HY.sqrt();
            let IC = HW * (AC / (AB * IB));
            let ID = HZ + IB;
            let IE = IA + IC;
            let IF = ID * ID;
            let IG = IE * ID;
            let IH = IG + IG;
            let II = BQ + X;
            let IJ = II + 1e-6f64;
            let IK = IJ.sqrt();
            let IL = CB * IK;
            let IM = (BR * (AC / (AB * IK))) * CB;
            let IN = AV / IL;
            let IO = ((IM * IN) * AZ) / IL;
            let IP = IL + AV;
            let IQ = AV / IP;
            let IR = ((IM * IQ) * AZ) / IP;
            let IS = R + IN;
            let IT = IO * AZ;
            let IV = (-IS) * IU;
            let IW = IT * IU;
            let IY = (IX * ((HY + (IB * HZ)) + HX)) / ID;
            let IZ = IY - R;
            let JA = IV * IZ;
            let JB = (IW * IZ) + ((((((HW + ((IC * HZ) + (IA * IB))) + CQ) * IX) - (IE * IY)) / ID) * IV);
            let JD = (JC * IL) - (IQ * JA);
            let JE = (IM * JC) - ((IR * JA) + (JB * IQ));
            let KU;
            let KV;
            let KW;
            let KX;
            let KY;
            let KZ;
            let LA;
            let LB;
            let LC;
            let LD;
            if JF != 0.0 {
                let KC = BR * BQ;
                let KE = ((BQ * BQ) + KD).sqrt();
                let KF = (KC + KC) * (AC / (AB * KE));
                let KG = AD * (BQ + KE);
                let KH = (BR + KF) * AD;
                let KJ = KH * KI;
                let KK = R + (KI * KG);
                let KL = GG * KK;
                let KM = staged[31] / KL;
                let KN = ((((GH * KK) + (KJ * GG)) * KM) * AZ) / KL;
                KU = KM;
                KV = KG;
                KW = KK;
                KX = KE;
                KY = KO;
                KZ = KN;
                LA = KH;
                LB = KJ;
                LC = KF;
                LD = KP;
            } else {
                let KR = JD + (KQ * JA);
                let KS = JE + (JB * KQ);
                let KT = if KR > O { 1.0 } else { 0.0 };
                let QY;
                let QZ;
                if KT != 0.0 {
                    let QU = KS * QT;
                    let QV = R + (QT * KR);
                    QY = QV;
                    QZ = QU;
                } else {
                    let QW = R - (QT * KR);
                    let QX = (KS * QT) * AZ;
                    QY = QW;
                    QZ = QX;
                }
                let RA = GG * QY;
                let RB = staged[34] / RA;
                let RC = ((((GH * QY) + (QZ * GG)) * RB) * AZ) / RA;
                KU = RB;
                KV = O;
                KW = O;
                KX = O;
                KY = QY;
                KZ = RC;
                LA = KP;
                LB = KP;
                LC = KP;
                LD = QZ;
            }
            let LE = II + staged[35];
            let LF = LE.sqrt();
            let LG = BR * (AC / (AB * LF));
            let LH = CB * LF;
            let LI = AV / LH;
            let LJ = (((LG * CB) * LI) * AZ) / LH;
            let LK = R + LI;
            let LL = CP - FU;
            let LM = CQ - FV;
            let LN = KD * LK;
            let LO = LN * KU;
            let LP = ((LJ * KD) * KU) + (KZ * LN);
            let LQ = LO * LL;
            let LR = (LP * LL) + (LM * LO);
            let LS = BG + BG;
            let LT = BI / LS;
            let LV = LU * LT;
            let LW = ((BJ - ((BH + BH) * LT)) / LS) * LU;
            let LX = AS * LV;
            let LY = (LV * AR) / AP;
            let LZ = AQ * LY;
            let MA = AM * LV;
            let MB = (LV * AL) / AJ;
            let MC = AK * MB;
            let MD = II / BN;
            let ME = -MD;
            let MF = ((BR - (BO * MD)) / BN) * AZ;
            let MG = ME * LY;
            let MH = (MF * LY) + (((((LW * AR) + Lanes([LX[0], 0.0, LX[1], LX[2]])) - Lanes([LZ[0], 0.0, LZ[1], LZ[2]])) / AP) * ME);
            let MI = ME * MB;
            let MJ = (MF * MB) + (((((LW * AL) + Lanes([MA[0], 0.0, MA[1], MA[2]])) - Lanes([MC[0], 0.0, MC[1], MC[2]])) / AJ) * ME);
            let MK = CM * BT;
            let ML = CN * BT;
            let MM = MK * MG;
            let MN = (ML * MG) + (MH * MK);
            let MO = MI - R;
            let MP = MK * MO;
            let MQ = (ML * MO) + (MJ * MK);
            let MS = MR * CU;
            let MT = MS * CR;
            let MU = IU / MT;
            let MV = (((((CV * MR) * CR) + (CS * MS)) * MU) * AZ) / MT;
            let MW = MU * MM;
            let MX = (MV * MM) + (MN * MU);
            let MY = MU * MP;
            let MZ = (MV * MP) + (MQ * MU);
            let NA = CR + CR;
            let NB = IU / NA;
            let NC = (((CS + CS) * NB) * AZ) / NA;
            let NE = ND * ((MM * NB) - MW);
            let NF = (((MN * NB) + (NC * MM)) - MX) * ND;
            let NG = ND * ((MP * NB) - MY);
            let NH = (((MQ * NB) + (NC * MP)) - MZ) * ND;
            let NI = R / DH;
            let NJ = ((DI * NI) * AZ) / DH;
            let NK = R / DN;
            let NL = ((DO * NK) * AZ) / DN;
            let NM = (CX * MW) + NE;
            let NN = AD - MW;
            let NO = (DJ * NN) + NE;
            let NP = (NM * NI) - (NO * NK);
            let NQ = (((((CY * MW) + (MX * CX)) + NF) * NI) + (NJ * NM)) - (((((DL * NN) + ((MX * AZ) * DJ)) + NF) * NK) + (NL * NO));
            let NR = (CX * MY) + NG;
            let NS = -5e-1f64 - MY;
            let NT = (DJ * NS) + NG;
            let NU = (NR * NI) - (NT * NK);
            let NV = (((((CY * MY) + (MZ * CX)) + NH) * NI) + (NJ * NR)) - (((((DL * NS) + ((MZ * AZ) * DJ)) + NH) * NK) + (NL * NT));
            let NW = MR * DS;
            let NX = NW * CP;
            let NY = (IU * (CR - 1.5e0f64)) / NX;
            let NZ = ((CS * IU) - ((((DT * MR) * CP) + (CQ * NW)) * NY)) / NX;
            let OA = NY * MM;
            let OB = (NZ * MM) + (MN * NY);
            let OC = NY * MP;
            let OD = (NZ * MP) + (MQ * NY);
            let OE = FR * BT;
            let OF = FS * BT;
            let OG = R / DZ;
            let OH = ((EA * OG) * AZ) / DZ;
            let OI = R / EC;
            let OJ = ((ED * OI) * AZ) / EC;
            let OK = (DV * OA) + NE;
            let OL = AD - OA;
            let OM = (DW * OL) + NE;
            let ON = ((MG - AD) - (OK * OG)) + (OM * OI);
            let OO = OE * ON;
            let OP = (OF * ON) + (((MH - (((((DU * OA) + (OB * DV)) + NF) * OG) + (OH * OK))) + (((((DX * OL) + ((OB * AZ) * DW)) + NF) * OI) + (OJ * OM))) * OE);
            let OQ = (DV * OC) + NG;
            let OR = -5e-1f64 - OC;
            let OS = (DW * OR) + NG;
            let OT = ((MI - AD) - (OQ * OG)) + (OS * OI);
            let OU = OE * OT;
            let OV = (OF * OT) + (((MJ - (((((DU * OC) + (OD * DV)) + NH) * OG) + (OH * OQ))) + (((((DX * OR) + ((OD * AZ) * DW)) + NH) * OI) + (OJ * OS))) * OE);
            let OW = (FX + DB) - DP;
            let OX = FZ / OW;
            let OY = ((FW * OX) * AZ) / OW;
            let OZ = AD - NP;
            let PA = -5e-1f64 - NU;
            let PB = R / GE;
            let PC = ((GF * PB) * AZ) / GE;
            let PD = (-(OX * OZ)) + ((AD + NP) * GA);
            let PE = PB * PD;
            let PF = (PC * PD) + (((((OY * OZ) + ((NQ * AZ) * OX)) * AZ) + (NQ * GA)) * PB);
            let PG = (-(OX * PA)) + ((-5e-1f64 + NU) * GA);
            let PH = PB * PG;
            let PI = (PC * PG) + (((((OY * PA) + ((NV * AZ) * OX)) * AZ) + (NV * GA)) * PB);
            let PJ = HT * BT;
            let PK = HU * BT;
            let PL = MG - R;
            let PM = PJ * PL;
            let PN = PJ * MI;
            let PP = (IV * PO) / IF;
            let PQ = ((IW * PO) - (IH * PP)) / IF;
            let PR = HZ + (CB * IB);
            let PS = PP * PR;
            let PT = (PQ * PR) + ((IA + (IC * CB)) * PP);
            let PU = IB + (CB * HZ);
            let PV = PP * PU;
            let PW = (PQ * PU) + ((IC + (IA * CB)) * PP);
            let PX = -IN;
            let PY = (CB + IN) + IN;
            let PZ = PY * IJ;
            let QA = (PX * JA) / PZ;
            let QB = (((IT * JA) + (JB * PX)) - ((((IO + IO) * IJ) + (BR * PY)) * QA)) / PZ;
            let QC = ((QA * MG) + (PS * MM)) + (PV * PM);
            let QD = (((QB * MG) + (MH * QA)) + ((PT * MM) + (MN * PS))) + ((PW * PM) + (((PK * PL) + (MH * PJ)) * PV));
            let QE = ((QA * MI) + (PS * MP)) + (PV * PN);
            let QF = (((QB * MI) + (MJ * QA)) + ((PT * MP) + (MQ * PS))) + ((PW * PN) + (((PK * MI) + (MJ * PJ)) * PV));
            let QG = CB * IS;
            let QH = QG * IJ;
            let QI = JA / QH;
            let QJ = IS - QI;
            let QK = IO - ((JB - ((((IO * CB) * IJ) + (BR * QG)) * QI)) / QH);
            let QL = -IQ;
            let QM = IR * AZ;
            let QN = (QJ * MG) + QC;
            let QO = QL * QN;
            let QP = (QM * QN) + ((((QK * MG) + (MH * QJ)) + QD) * QL);
            let QQ = (QJ * MI) + QE;
            let QR = QL * QQ;
            let QS = (QM * QQ) + ((((QK * MI) + (MJ * QJ)) + QF) * QL);
            let RS;
            let RT;
            let RU;
            let RV;
            if JF != 0.0 {
                let RD = KW * KX;
                let RE = (KI * KV) / RD;
                let RF = ((LA * KI) - (((LB * KX) + (LC * KW)) * RE)) / RD;
                let RG = (-PE) - (RE * MG);
                let RH = (PF * AZ) - ((RF * MG) + (MH * RE));
                let RI = (-PH) - (RE * MI);
                let RJ = (PI * AZ) - ((RF * MI) + (MJ * RE));
                RS = RG;
                RT = RI;
                RU = RH;
                RV = RJ;
            } else {
                let RK = QT / KY;
                let RL = ((LD * RK) * AZ) / KY;
                let RM = QO + (KQ * QC);
                let RN = (-PE) + (RK * RM);
                let RO = (PF * AZ) + ((RL * RM) + ((QP + (QD * KQ)) * RK));
                let RP = QR + (KQ * QE);
                let RQ = (-PH) + (RK * RP);
                let RR = (PI * AZ) + ((RL * RP) + ((QS + (QF * KQ)) * RK));
                RS = RN;
                RT = RQ;
                RU = RO;
                RV = RR;
            }
            let RW = MR * LK;
            let RX = RW * LF;
            let RY = RX * LE;
            let RZ = staged[38] / RY;
            let SA = (((((((LJ * MR) * LF) + (LG * RW)) * LE) + (BR * RX)) * RZ) * AZ) / RY;
            let SB = (RZ * MG) + RS;
            let SC = ((SB * LL) + MM) - OO;
            let SD = -LO;
            let SE = (RZ * MI) + RT;
            let SF = ((SE * LL) + MP) - OU;
            let SH = (R + ((SD * SF) * SG)) + ((LO * SC) * SG);
            let SI = R / SH;
            let SJ = LQ * SI;
            let SK = (LR * SI) + (((((((((LP * AZ) * SF) + ((((((((SA * MI) + (MJ * RZ)) + RV) * LL) + (LM * SE)) + MQ) - OV) * SD)) * SG) + (((LP * SC) + ((((((((SA * MG) + (MH * RZ)) + RU) * LL) + (LM * SB)) + MN) - OP) * LO)) * SG)) * SI) * AZ) / SH) * LQ);
            let SM = CZ - (SL * CX);
            let SN = Lanes([DA[0], 0.0, DA[1], DA[2]]) - (CY * SL);
            let SO = if (if SM > O { 1.0 } else { 0.0 }) != 0.0 && staged[41] != 0.0 { 1.0 } else { 0.0 };
            let SU;
            let SV;
            if SO != 0.0 {
                let SP = R / SM;
                let SQ = -staged[42];
                let SR = SQ * SP;
                let SS = (((SN * SP) * AZ) / SM) * SQ;
                let ST = if SR < -3.5e1f64 { 1.0 } else { 0.0 };
                let UP;
                let UQ;
                if ST != 0.0 {
                    UP = UO;
                    UQ = KP;
                } else {
                    UP = SR;
                    UQ = SS;
                }
                let UR = UP.exp();
                let UT = US * SM;
                let UU = UT * UR;
                let UV = UU * SJ;
                let UW = ((((SN * US) * UR) + ((UQ * UR) * UT)) * SJ) + (SK * UU);
                SU = UV;
                SV = UW;
            } else {
                SU = O;
                SV = KP;
            }
            let SW = HZ * HX;
            let SX = (IA * HX) + (CQ * HZ);
            let SY = IB * HY;
            let SZ = (IC * HY) + (HW * IB);
            let TA = (X + (AD * BQ)).sqrt();
            let TB = (BR * AD) * (AC / (AB * TA));
            let TC = TA + TA;
            let TD = TB + TB;
            let TE = BI / TC;
            let TG = -(((R + TE) * IU) * TF);
            let TH = ((((BJ - (TD * TE)) / TC) * IU) * TF) * AZ;
            let TK = TJ * HY;
            let TL = MR * IB;
            let TN = (TM * ((((TI * SY) + (TK * HZ)) + (TL * HX)) + (CB * SW))) / IF;
            let TO = TN - AD;
            let TP = TG * TO;
            let TQ = (TH * TO) + ((((((((SZ * TI) + (((HW * TJ) * HZ) + (IA * TK))) + (((IC * MR) * HX) + (CQ * TL))) + (SX * CB)) * TM) - (IH * TN)) / IF) * TG);
            let TR = TJ * HX;
            let TS = MR * HZ;
            let TT = (TM * ((((TI * SW) + (TR * IB)) + (TS * HY)) + (CB * SY))) / IF;
            let TU = TT - AD;
            let TV = TG * TU;
            let TW = (TH * TU) + ((((((((SX * TI) + (((CQ * TJ) * IB) + (IC * TR))) + (((IA * MR) * HY) + (HW * TS))) + (SZ * CB)) * TM) - (IH * TT)) / IF) * TG);
            let TX = TV + TP;
            let TY = TW + TQ;
            let UA = TZ * BI;
            let UB = BI + TC;
            let UC = (TX * BI) / UB;
            let UD = (-TX) - ((TF * (((UA * IL) + AE) - Y)) - UC);
            let UE = (TY * AZ) - (((((((BJ * TZ) * IL) + (IM * UA)) + BM) - Lanes([0.0, F[0], 0.0, F[1]])) * TF) - ((((TY * BI) + (BJ * TX)) - ((BJ + TD) * UC)) / UB));
            let UF = D * U;
            let UG = UF * SJ;
            let UH = SK * UF;
            let UI = ddt(4749, TP);
            let UK = TQ * UJ;
            let UL = ddt(4751, TV);
            let UM = TW * UJ;
            let UN = if U == R { 1.0 } else { 0.0 };
            let VR;
            let VS;
            let VT;
            let VU;
            let VV;
            let VW;
            let VX;
            let VY;
            let VZ;
            let WA;
            let WB;
            let WC;
            let WD;
            let WE;
            let WF;
            let WG;
            let WH;
            let WI;
            let WJ;
            let WK;
            if UN != 0.0 {
                let UX = D * UI;
                let UY = UK * D;
                let UZ = D * TP;
                let VA = TQ * D;
                let VB = D * UL;
                let VC = UM * D;
                let VD = D * TV;
                let VE = TW * D;
                let VF = D * SU;
                let VG = SV * D;
                VR = UX;
                VS = VB;
                VT = VF;
                VU = O;
                VV = O;
                VW = O;
                VX = UZ;
                VY = VD;
                VZ = O;
                WA = O;
                WB = UY;
                WC = VC;
                WD = VG;
                WE = KP;
                WF = KP;
                WG = KP;
                WH = VA;
                WI = VE;
                WJ = KP;
                WK = KP;
            } else {
                let VH = D * UI;
                let VI = UK * D;
                let VJ = D * TP;
                let VK = TQ * D;
                let VL = D * UL;
                let VM = UM * D;
                let VN = D * TV;
                let VO = TW * D;
                let VP = D * SU;
                let VQ = SV * D;
                VR = O;
                VS = O;
                VT = O;
                VU = VH;
                VV = VL;
                VW = VP;
                VX = O;
                VY = O;
                VZ = VJ;
                WA = VN;
                WB = KP;
                WC = KP;
                WD = KP;
                WE = VI;
                WF = VM;
                WG = VQ;
                WH = KP;
                WI = KP;
                WJ = VK;
                WK = VO;
            }
            let WL = D * ddt(4775, UD);
            let WM = (UE * UJ) * D;
            let WN = D * UD;
            let WO = UE * D;
            let WP = -K;
            let WS = (WP * WQ) / WR;
            let WT = ((L * AZ) * WQ) / WR;
            let WU = if WS < -4e1f64 { 1.0 } else { 0.0 };
            let WX;
            let WY;
            if WU != 0.0 {
                WX = WV;
                WY = WW;
            } else {
                WX = WS;
                WY = WT;
            }
            let XA = ((WP + WZ) * WQ) / WR;
            let XC = if XA > XB { 1.0 } else { 0.0 };
            let XH;
            let XI;
            if XC != 0.0 {
                XH = R;
                XI = WW;
            } else {
                let XD = (-XA).exp();
                let XF = ((WT * AZ) * XD) * XE;
                let XG = R + (XE * XD);
                XH = XG;
                XI = XF;
            }
            let XJ = K * WQ;
            let XK = L * WQ;
            let XN = XM + K;
            let XP = if XN >= XO { XN } else { XO };
            let XQ = ((XJ / XL) * XM) / XP;
            let XR = XQ.exp();
            let XV = XU + K;
            let XW = if XV >= XO { XV } else { XO };
            let XX = ((XJ / XT) * XU) / XW;
            let XY = XX.exp();
            let YC = YB + K;
            let YD = if YC >= XO { YC } else { XO };
            let YE = ((XJ / YA) * YB) / YD;
            let YF = YE.exp();
            let YH = WX.exp();
            let YJ = YI * (R - YH);
            let YM = ((((YJ * XH) + (K * YK)) + (((XS * (XR - R)) - (XZ * (XY - R))) - (YG * (YF - R)))) * D) * YL;
            let YN = ((((((((WY * YH) * AZ) * YI) * XH) + (XI * YJ)) + (L * YK)) + ((((((((XK / XL) * XM) - ((L * (if XN >= XO { 1.0 } else { 0.0 })) * XQ)) / XP) * XR) * XS) - ((((((XK / XT) * XU) - ((L * (if XV >= XO { 1.0 } else { 0.0 })) * XX)) / XW) * XY) * XZ)) - ((((((XK / YA) * YB) - ((L * (if YC >= XO { 1.0 } else { 0.0 })) * YE)) / YD) * YF) * YG))) * D) * YL;
            let YO = -H;
            let YP = (YO * WQ) / WR;
            let YQ = ((I * AZ) * WQ) / WR;
            let YR = if YP < -4e1f64 { 1.0 } else { 0.0 };
            let YU;
            let YV;
            if YR != 0.0 {
                YU = YS;
                YV = YT;
            } else {
                YU = YP;
                YV = YQ;
            }
            let YW = ((YO + WZ) * WQ) / WR;
            let YX = if YW > XB { 1.0 } else { 0.0 };
            let ZB;
            let ZC;
            if YX != 0.0 {
                ZB = R;
                ZC = YT;
            } else {
                let YY = (-YW).exp();
                let YZ = ((YQ * AZ) * YY) * XE;
                let ZA = R + (XE * YY);
                ZB = ZA;
                ZC = YZ;
            }
            let ZD = H * WQ;
            let ZE = I * WQ;
            let ZF = XM + H;
            let ZG = if ZF >= XO { ZF } else { XO };
            let ZH = ((ZD / XL) * XM) / ZG;
            let ZI = ZH.exp();
            let ZJ = XU + H;
            let ZK = if ZJ >= XO { ZJ } else { XO };
            let ZL = ((ZD / XT) * XU) / ZK;
            let ZM = ZL.exp();
            let ZO = YB + H;
            let ZP = if ZO >= XO { ZO } else { XO };
            let ZQ = ((ZD / YA) * YB) / ZP;
            let ZR = ZQ.exp();
            let ZT = YU.exp();
            let ZV = ZU * (R - ZT);
            let ZW = ((((ZV * ZB) + (H * YK)) + (((XS * (ZI - R)) - (ZN * (ZM - R))) - (ZS * (ZR - R)))) * D) * YL;
            let ZX = ((((((((YV * ZT) * AZ) * ZU) * ZB) + (ZC * ZV)) + (I * YK)) + ((((((((ZE / XL) * XM) - ((I * (if ZF >= XO { 1.0 } else { 0.0 })) * ZH)) / ZG) * ZI) * XS) - ((((((ZE / XT) * XU) - ((I * (if ZJ >= XO { 1.0 } else { 0.0 })) * ZL)) / ZK) * ZM) * ZN)) - ((((((ZE / YA) * YB) - ((I * (if ZO >= XO { 1.0 } else { 0.0 })) * ZQ)) / ZP) * ZR) * ZS))) * D) * YL;
            let ZY = if K > O { 1.0 } else { 0.0 };
            let ABM;
            let ABN;
            let ABO;
            let ABP;
            let ABQ;
            let ABR;
            if ZY != 0.0 {
                let AAB = ZZ * AAA;
                let AAD = -AAC;
                let AAF = R + (K / AAE);
                let AAG = (AAD * (AAF.ln())).exp();
                let AAH = AAB * AAG;
                let AAI = ((((L / AAE) * (AC / AAF)) * AAD) * AAG) * AAB;
                let AAL = AAJ * AAK;
                let AAN = -AAM;
                let AAP = R + (K / AAO);
                let AAQ = (AAN * (AAP.ln())).exp();
                let AAR = AAL * AAQ;
                let AAS = ((((L / AAO) * (AC / AAP)) * AAN) * AAQ) * AAL;
                let AAV = AAT * AAU;
                let AAX = -AAW;
                let AAZ = R + (K / AAY);
                let ABA = (AAX * (AAZ.ln())).exp();
                let ABB = AAV * ABA;
                let ABC = ((((L / AAY) * (AC / AAZ)) * AAX) * ABA) * AAV;
                ABM = AAH;
                ABN = AAR;
                ABO = ABB;
                ABP = AAI;
                ABQ = AAS;
                ABR = ABC;
            } else {
                let ABD = ZZ * AAA;
                let ABE = ABD * (R - ((AAC * K) / AAE));
                let ABF = (((L * AAC) / AAE) * AZ) * ABD;
                let ABG = AAJ * AAK;
                let ABH = ABG * (R - ((AAM * K) / AAO));
                let ABI = (((L * AAM) / AAO) * AZ) * ABG;
                let ABJ = AAT * AAU;
                let ABK = ABJ * (R - ((AAW * K) / AAY));
                let ABL = (((L * AAW) / AAY) * AZ) * ABJ;
                ABM = ABE;
                ABN = ABH;
                ABO = ABK;
                ABP = ABF;
                ABQ = ABI;
                ABR = ABL;
            }
            let ABS = (ABM + ABN) + ABO;
            let ABT = ABS * K;
            let ABU = (((ABP + ABQ) + ABR) * K) + (L * ABS);
            let ABV = (ddt(5312, ABT) * D) * YL;
            let ABW = ((ABU * UJ) * D) * YL;
            let ABX = (ABT * D) * YL;
            let ABY = (ABU * D) * YL;
            let ABZ = if H > O { 1.0 } else { 0.0 };
            let ADD;
            let ADE;
            let ADF;
            let ADG;
            let ADH;
            let ADI;
            if ABZ != 0.0 {
                let ACB = ZZ * ACA;
                let ACC = -AAC;
                let ACD = R + (H / AAE);
                let ACE = (ACC * (ACD.ln())).exp();
                let ACF = ACB * ACE;
                let ACG = ((((I / AAE) * (AC / ACD)) * ACC) * ACE) * ACB;
                let ACI = AAJ * ACH;
                let ACJ = -AAM;
                let ACK = R + (H / AAO);
                let ACL = (ACJ * (ACK.ln())).exp();
                let ACM = ACI * ACL;
                let ACN = ((((I / AAO) * (AC / ACK)) * ACJ) * ACL) * ACI;
                let ACO = AAT * AAU;
                let ACP = -AAW;
                let ACQ = R + (H / AAY);
                let ACR = (ACP * (ACQ.ln())).exp();
                let ACS = ACO * ACR;
                let ACT = ((((I / AAY) * (AC / ACQ)) * ACP) * ACR) * ACO;
                ADD = ACF;
                ADE = ACM;
                ADF = ACS;
                ADG = ACG;
                ADH = ACN;
                ADI = ACT;
            } else {
                let ACU = ZZ * ACA;
                let ACV = ACU * (R - ((AAC * H) / AAE));
                let ACW = (((I * AAC) / AAE) * AZ) * ACU;
                let ACX = AAJ * ACH;
                let ACY = ACX * (R - ((AAM * H) / AAO));
                let ACZ = (((I * AAM) / AAO) * AZ) * ACX;
                let ADA = AAT * AAU;
                let ADB = ADA * (R - ((AAW * H) / AAY));
                let ADC = (((I * AAW) / AAY) * AZ) * ADA;
                ADD = ACV;
                ADE = ACY;
                ADF = ADB;
                ADG = ACW;
                ADH = ACZ;
                ADI = ADC;
            }
            let ADJ = (ADD + ADE) + ADF;
            let ADK = ADJ * H;
            let ADL = (((ADG + ADH) + ADI) * H) + (I * ADJ);
            let ADM = (ddt(5403, ADK) * D) * YL;
            let ADN = ((ADL * UJ) * D) * YL;
            let ADO = (ADK * D) * YL;
            let ADP = (ADL * D) * YL;
            let ADQ = UH[0];
            let ADR = UH[1];
            let ADS = UH[2];
            let ADT = UH[3];
            let ADU = WB[0];
            let ADV = WB[1];
            let ADW = WB[2];
            let ADX = WB[3];
            let ADY = WC[0];
            let ADZ = WC[1];
            let AEA = WC[2];
            let AEB = WC[3];
            let AEC = WD[0];
            let AED = WD[1];
            let AEE = WD[2];
            let AEF = WD[3];
            let AEG = WE[0];
            let AEH = WE[1];
            let AEI = WE[2];
            let AEJ = WE[3];
            let AEK = WF[0];
            let AEL = WF[1];
            let AEM = WF[2];
            let AEN = WF[3];
            let AEO = WG[0];
            let AEP = WG[1];
            let AEQ = WG[2];
            let AER = WG[3];
            let AES = WM[0];
            let AET = WM[1];
            let AEU = WM[2];
            let AEV = WM[3];
            let AEW = YN[0];
            let AEX = YN[1];
            let AEY = ZX[0];
            let AEZ = ZX[1];
            let AFA = ABW[0];
            let AFB = ABW[1];
            let AFC = ADN[0];
            let AFD = ADN[1];
            let AFE = WH[0];
            let AFF = WH[1];
            let AFG = WH[2];
            let AFH = WH[3];
            let AFI = WI[0];
            let AFJ = WI[1];
            let AFK = WI[2];
            let AFL = WI[3];
            let AFM = WJ[0];
            let AFN = WJ[1];
            let AFO = WJ[2];
            let AFP = WJ[3];
            let AFQ = WK[0];
            let AFR = WK[1];
            let AFS = WK[2];
            let AFT = WK[3];
            let AFU = WO[0];
            let AFV = WO[1];
            let AFW = WO[2];
            let AFX = WO[3];
            let AFY = ABY[0];
            let AFZ = ABY[1];
            let AGA = ADP[0];
            let AGB = ADP[1];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(2),
            multiplicity * (UG),
            [0, 1, 2, 3],
            [ADQ, ADR, ADS, ADT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (VR),
            [0, 1, 2, 3],
            [ADU, ADV, ADW, ADX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (VS),
            [0, 1, 2, 3],
            [ADY, ADZ, AEA, AEB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (VT),
            [0, 1, 2, 3],
            [AEC, AED, AEE, AEF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (VU),
            [0, 1, 2, 3],
            [AEG, AEH, AEI, AEJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (VV),
            [0, 1, 2, 3],
            [AEK, AEL, AEM, AEN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(2),
            Some(3),
            multiplicity * (VW),
            [0, 1, 2, 3],
            [AEO, AEP, AEQ, AER],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * (WL),
            [0, 1, 2, 3],
            [AES, AET, AEU, AEV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(2),
            multiplicity * (staged[83]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(3),
            multiplicity * (YM),
            [0, 3],
            [AEW, AEX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(3),
            multiplicity * (ZW),
            [2, 3],
            [AEY, AEZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(3),
            multiplicity * (ABV),
            [0, 3],
            [AFA, AFB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(3),
            multiplicity * (ADM),
            [2, 3],
            [AFC, AFD],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = UG;
        self.canonical_reactive[1] = VX;
        self.canonical_reactive[2] = AFE;
        self.canonical_reactive[3] = AFF;
        self.canonical_reactive[4] = AFG;
        self.canonical_reactive[5] = AFH;
        self.canonical_reactive[6] = VY;
        self.canonical_reactive[7] = AFI;
        self.canonical_reactive[8] = AFJ;
        self.canonical_reactive[9] = AFK;
        self.canonical_reactive[10] = AFL;
        self.canonical_reactive[11] = VT;
        self.canonical_reactive[12] = VZ;
        self.canonical_reactive[13] = AFM;
        self.canonical_reactive[14] = AFN;
        self.canonical_reactive[15] = AFO;
        self.canonical_reactive[16] = AFP;
        self.canonical_reactive[17] = WA;
        self.canonical_reactive[18] = AFQ;
        self.canonical_reactive[19] = AFR;
        self.canonical_reactive[20] = AFS;
        self.canonical_reactive[21] = AFT;
        self.canonical_reactive[22] = VW;
        self.canonical_reactive[23] = WN;
        self.canonical_reactive[24] = AFU;
        self.canonical_reactive[25] = AFV;
        self.canonical_reactive[26] = AFW;
        self.canonical_reactive[27] = AFX;
        self.canonical_reactive[28] = staged[83];
        self.canonical_reactive[29] = YM;
        self.canonical_reactive[30] = ZW;
        self.canonical_reactive[31] = ABX;
        self.canonical_reactive[32] = AFY;
        self.canonical_reactive[33] = AFZ;
        self.canonical_reactive[34] = ADO;
        self.canonical_reactive[35] = AGA;
        self.canonical_reactive[36] = AGB;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[2], cached[3], cached[4], cached[5]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[7], cached[8], cached[9], cached[10]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[13], cached[14], cached[15], cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(3),
            &[0, 1, 2, 3],
            &[cached[24], cached[25], cached[26], cached[27]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(3),
            &[0, 3],
            &[cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(3),
            &[2, 3],
            &[cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
    }

}
