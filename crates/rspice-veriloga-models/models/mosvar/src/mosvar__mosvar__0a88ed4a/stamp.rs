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
        let mut key = Vec::with_capacity(126);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[74] = values[0];
        self.canonical_staged[16] = values[1];
        self.canonical_staged[1] = values[2];
        self.canonical_staged[2] = values[3];
        self.canonical_staged[75] = values[4];
        self.canonical_staged[76] = values[5];
        self.canonical_staged[77] = values[6];
        self.canonical_staged[78] = values[7];
        self.canonical_staged[79] = values[8];
        self.canonical_staged[80] = values[9];
        self.canonical_staged[0] = values[10];
        self.canonical_staged[91] = values[11];
        self.canonical_staged[7] = values[12];
        self.canonical_staged[8] = values[13];
        self.canonical_staged[101] = values[14];
        self.canonical_staged[102] = values[15];
        self.canonical_staged[38] = values[16];
        self.canonical_staged[17] = values[17];
        self.canonical_staged[103] = values[18];
        self.canonical_staged[22] = values[19];
        self.canonical_staged[31] = values[20];
        self.canonical_staged[32] = values[21];
        self.canonical_staged[33] = values[22];
        self.canonical_staged[36] = values[23];
        self.canonical_staged[104] = values[24];
        self.canonical_staged[105] = values[25];
        self.canonical_staged[106] = values[26];
        self.canonical_staged[41] = values[27];
        self.canonical_staged[48] = values[28];
        self.canonical_staged[49] = values[29];
        self.canonical_staged[52] = values[30];
        self.canonical_staged[55] = values[31];
        self.canonical_staged[56] = values[32];
        self.canonical_staged[59] = values[33];
        self.canonical_staged[63] = values[34];
        self.canonical_staged[65] = values[35];
        self.canonical_staged[118] = values[36];
        self.canonical_staged[119] = values[37];
        self.canonical_staged[120] = values[38];
        self.canonical_staged[121] = values[39];
        self.canonical_staged[122] = values[40];
        self.canonical_staged[123] = values[41];
        self.canonical_staged[124] = values[42];
        self.canonical_staged[125] = values[43];
        self.canonical_staged[126] = values[44];
        self.canonical_staged[127] = values[45];
        self.canonical_staged[128] = values[46];
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
                let B = parameters[19];
                let D = parameters[29];
                let G = parameters[30];
                let H = 0e0f64;
                let K = parameters[17];
                let Q = parameters[48];
                let V = parameters[11];
                let X = -2.73e2f64;
                let AC = parameters[16];
                let AD = parameters[66];
                let AF = parameters[49];
                let AG = 2e0f64;
                let AH = parameters[53];
                let AK = 1e0f64;
                let AL = parameters[50];
                let AN = parameters[51];
                let AP = 1.05457168e-34f64;
                let AS = parameters[59];
                let AU = 1e-1f64;
                let BF = parameters[64];
                let BL = parameters[18];
                let BU = 0e0f64;
                let BV = 0e0f64;
                let BW = 0e0f64;
                let BX = 0e0f64;
                let CC = 0e0f64;
                let CD = 0e0f64;
                let CG = 0e0f64;
                let CH = 0e0f64;
                let CI = 0e0f64;
                let CJ = 0e0f64;
                let CK = 0e0f64;
                let mut oL = 0.0;
                let mut oAI = 0.0;
                let mut oAJ = 0.0;
                let mut oAT = 0.0;
                let mut oBG = 0.0;
                let mut oBJ = 0.0;
                let mut oBM = 0.0;
                let mut oBO = 0.0;
                let A = if parameters[7] != 1e3f64 { 1.0 } else { 0.0 };
                let C = (3.453e-11f64 * (parameters[20] / 3.9e0f64)) / B;
                let E = ((3.348580862e-29f64 * D).sqrt()) / C;
                let F = ((3.348580862e-29f64 * parameters[54]).sqrt()) / C;
                let I = if G > H { 1.0 } else { 0.0 };
                let M;
                if I != 0.0 {
                    let J = (2.3807972e0f64 * G) * (C.powf(6.666666666666666e-1f64));
                    let L = if K < H { 1.0 } else { 0.0 };
                    oL = L;
                    let P = if L != 0.0 {
                        let O = 1.2514650134837189e0f64 * J;
                        O
                    } else {
                        J
                    };
                    M = P;
                } else {
                    M = H;
                }
                let N = if K < H { 1.0 } else { 0.0 };
                let T = if N != 0.0 {
                    let R = 3.333333333333333e-1f64 * Q;
                    R
                } else {
                    let S = 5e-1f64 * Q;
                    S
                };
                let U = B / 1e-9f64;
                let W = if V > -2.73e2f64 { 1.0 } else { 0.0 };
                let Y = if W != 0.0 {
                    V
                } else {
                    X
                };
                let Z = if Y < parameters[8] { 1.0 } else { 0.0 };
                let AA = if Y > parameters[9] { 1.0 } else { 0.0 };
                let AB = 2.7315e2f64 + Y;
                let AE = if AD == H { 1.0 } else { 0.0 };
                let AV;
                let AW;
                let AX;
                let AY;
                let AZ;
                let BA;
                let BB;
                let BC;
                if AF != 0.0 {
                    let AI = (AG * parameters[56]) * AH;
                    oAI = AI;
                    let AJ = (AG * parameters[61]) * AH;
                    oAJ = AJ;
                    let AM = AK / AL;
                    let AO = AK / AN;
                    let AQ = ((1.3333333333333333e0f64 * ((2.918995620956536e-49f64 * AL).sqrt())) / AP) * B;
                    let AR = ((1.3333333333333333e0f64 * ((2.918995620956536e-49f64 * AN).sqrt())) / AP) * B;
                    let AT = if AS < H { 1.0 } else { 0.0 };
                    oAT = AT;
                    let BE = if AT != 0.0 {
                        let BD = (-4.95e-1f64 * parameters[58]) / AS;
                        BD
                    } else {
                        H
                    };
                    let BG = if BF < H { 1.0 } else { 0.0 };
                    oBG = BG;
                    let BI = if BG != 0.0 {
                        let BH = (-4.95e-1f64 * parameters[63]) / BF;
                        BH
                    } else {
                        H
                    };
                    AV = AO;
                    AW = BI;
                    AX = AR;
                    AY = AM;
                    AZ = BE;
                    BA = AQ;
                    BB = AR;
                    BC = AQ;
                } else {
                    AV = AU;
                    AW = H;
                    AX = H;
                    AY = AU;
                    AZ = H;
                    BA = H;
                    BB = H;
                    BC = H;
                }
                if I != 0.0 {
                    let BJ = 7.5e-1f64 * M;
                    oBJ = BJ;
                } else {
                }
                let BK = if D < 1e27f64 { 1.0 } else { 0.0 };
                if BK != 0.0 {
                    let BM = (-K) * BL;
                    oBM = BM;
                } else {
                }
                let BN = if parameters[21] < AK { 1.0 } else { 0.0 };
                if BK != 0.0 {
                    let BO = (-K) * BL;
                    oBO = BO;
                } else {
                }
                let BP = AK + (3.7e-1f64 * U);
                let BQ = if M > H { 1.0 } else { 0.0 };
                let BR = if AD == AG { 1.0 } else { 0.0 };
                let BS = if (BL * K) == -1e0f64 { 1.0 } else { 0.0 };
                let BT = if AF != H { 1.0 } else { 0.0 };
                let BY;
                let BZ;
                let CA;
                let CB;
                if AC != 0.0 {
                    BY = H;
                    BZ = H;
                    CA = H;
                    CB = H;
                } else {
                    BY = BU;
                    BZ = BV;
                    CA = BW;
                    CB = BX;
                }
                let CE;
                let CF;
                if AF != 0.0 {
                    CE = CC;
                    CF = CD;
                } else {
                    CE = H;
                    CF = H;
                }
                let CL;
                let CM;
                let CN;
                let CO;
                let CP;
                if AC != 0.0 {
                    CL = CG;
                    CM = CH;
                    CN = CI;
                    CO = CJ;
                    CP = CK;
                } else {
                    CL = H;
                    CM = H;
                    CN = H;
                    CO = H;
                    CP = H;
                }
            [A, C, E, F, I, oL, N, W, Z, AA, AB, AE, oAI, oAJ, oAT, oBG, M, oBJ, BK, oBM, BN, oBO, BP, T, BQ, BR, BS, BT, AV, AW, AX, AY, AZ, BA, BB, BC, BY, BZ, CA, CB, CE, CF, CL, CM, CN, CO, CP]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 17] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[1];
                let D = parameters[0];
                let I = 0e0f64;
                let L = 2e0f64;
                let N = parameters[16];
                let T = parameters[49];
                let U = 1e12f64;
                let mut oO = 0.0;
                let mut oP = 0.0;
                let mut oR = 0.0;
                let mut oS = 0.0;
                let mut oV = 0.0;
                let mut oW = 0.0;
                let mut oX = 0.0;
                let mut oY = 0.0;
                let B = if A < parameters[12] { 1.0 } else { 0.0 };
                let C = if A > parameters[13] { 1.0 } else { 0.0 };
                let E = if D < parameters[14] { 1.0 } else { 0.0 };
                let F = if D > parameters[15] { 1.0 } else { 0.0 };
                let G = A + parameters[31];
                let H = D + parameters[32];
                let J = if G <= I { 1.0 } else { 0.0 };
                let K = if H <= I { 1.0 } else { 0.0 };
                let M = L * ((parameters[35] * D) + (parameters[34] * A));
                if N != 0.0 {
                    let O = (3e0f64 + ((parameters[2] - 1e0f64) * 9e0f64)) * A;
                    oO = O;
                    let P = D * A;
                    oP = P;
                    let Q = D + parameters[33];
                    let R = L * Q;
                    oR = R;
                    let S = 1.2e1f64 * Q;
                    oS = S;
                } else {
                }
                if T != 0.0 {
                    let V = ((parameters[55] * H) * G) * U;
                    oV = V;
                    let W = (staged[7] * H) * U;
                    oW = W;
                    let X = ((parameters[60] * H) * G) * U;
                    oX = X;
                    let Y = (staged[8] * H) * U;
                    oY = Y;
                } else {
                }
            [B, C, E, F, G, H, J, K, M, oO, oP, oR, oS, oV, oW, oX, oY]
        };
        self.canonical_staged[83] = produced[0];
        self.canonical_staged[84] = produced[1];
        self.canonical_staged[85] = produced[2];
        self.canonical_staged[86] = produced[3];
        self.canonical_staged[67] = produced[4];
        self.canonical_staged[68] = produced[5];
        self.canonical_staged[87] = produced[6];
        self.canonical_staged[88] = produced[7];
        self.canonical_staged[69] = produced[8];
        self.canonical_staged[3] = produced[9];
        self.canonical_staged[4] = produced[10];
        self.canonical_staged[5] = produced[11];
        self.canonical_staged[6] = produced[12];
        self.canonical_staged[9] = produced[13];
        self.canonical_staged[10] = produced[14];
        self.canonical_staged[11] = produced[15];
        self.canonical_staged[12] = produced[16];
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
        let produced: [f64; 65] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = 2.7315e2f64;
                let G = staged[0];
                let K = 1e2f64;
                let M = 1e0f64;
                let V = 1e-3f64;
                let AF = 7.071067811865475e-1f64;
                let AI = 1e-5f64;
                let AP = 4.6051701859880916e2f64;
                let AT = 5e-1f64;
                let AW = parameters[16];
                let AX = parameters[0];
                let BB = parameters[1];
                let BE = 0e0f64;
                let BK = 1e3f64;
                let CB = 2e1f64;
                let CK = parameters[49];
                let CY = parameters[17];
                let DF = staged[106];
                let DG = parameters[18];
                let DR = parameters[64];
                let DT = 1.0f64;
                let DW = parameters[59];
                let EA = 0.0f64;
                let mut oBD = 0.0;
                let mut oBL = 0.0;
                let mut oBN = 0.0;
                let mut oBP = 0.0;
                let mut oBR = 0.0;
                let mut oBT = 0.0;
                let mut oBV = 0.0;
                let mut oBX = 0.0;
                let mut oBZ = 0.0;
                let mut oCC = 0.0;
                let mut oDN = 0.0;
                let mut oDO = 0.0;
                let mut oDQ = 0.0;
                let mut oDS = 0.0;
                let mut oDU = 0.0;
                let mut oDV = 0.0;
                let mut oDX = 0.0;
                let mut oDY = 0.0;
                let mut oDZ = 0.0;
                let mut oEB = 0.0;
                let mut oEC = 0.0;
                let mut oED = 0.0;
                let B = (temperature + parameters[3]) - A;
                let C = if B < parameters[8] { 1.0 } else { 0.0 };
                let D = if B > parameters[9] { 1.0 } else { 0.0 };
                let E = B + A;
                let F = E * E;
                let H = E / G;
                let I = G / E;
                let J = (E * 1.3806505e-23f64) / 1.6021918e-19f64;
                let L = (K * J) * J;
                let N = M / J;
                let O = parameters[23] + ((E - G) * parameters[42]);
                let P = parameters[36] * (I.powf(parameters[43]));
                let Q = parameters[37] * (I.powf(parameters[44]));
                let R = parameters[38] * (I.powf(parameters[45]));
                let S = parameters[39] * (I.powf(parameters[46]));
                let T = parameters[40] * (H.powf(parameters[47]));
                let U = 1.179e0f64 - (E * (9.025e-5f64 + (E * 3.05e-7f64)));
                let W = (if ((((1.045e0f64 + (4.5e-4f64 * E)) * ((5.23e-1f64 + (1.4e-3f64 * E)) - (1.48e-6f64 * F))) * F) / 9e4f64) >= V { ((((1.045e0f64 + (4.5e-4f64 * E)) * ((5.23e-1f64 + (1.4e-3f64 * E)) - (1.48e-6f64 * F))) * F) / 9e4f64) } else { V }).sqrt();
                let X = M / ((2.5e25f64 * W) * (W.sqrt()));
                let Y = 2e0f64 * J;
                let Z = U + (Y * ((parameters[24] * X).ln()));
                let AA = U + (6e0f64 * J);
                let AB = N.sqrt();
                let AC = staged[1] * AB;
                let AD = AC * AC;
                let AE = M / AD;
                let AG = M + (AC * AF);
                let AH = M / AG;
                let AJ = AI * AG;
                let AK = (U + (Y * ((parameters[29] * X).ln()))) * N;
                let AL = staged[2] * AB;
                let AM = AL * AL;
                let AN = M + (AL * AF);
                let AO = AI * AN;
                let AQ = if AK < AP { 1.0 } else { 0.0 };
                let AV = if AQ != 0.0 {
                    let AR = (-AK).exp();
                    AR
                } else {
                    let AS = AK - AP;
                    let AU = 1e-200f64 / (M + (AS * (M + ((AT * AS) * (M + (AS * 3.333333333333333e-1f64))))));
                    AU
                };
                let BF;
                let BG;
                let BH;
                let BI;
                let BJ;
                if AW != 0.0 {
                    let AY = (P * AX) / staged[3];
                    let AZ = Q / staged[4];
                    let BA = R / staged[5];
                    let BC = (S * BB) / staged[6];
                    let BD = if AY > V { 1.0 } else { 0.0 };
                    oBD = BD;
                    let BM;
                    if BD != 0.0 {
                        let BL = if AY < BK { 1.0 } else { 0.0 };
                        oBL = BL;
                        let BO = if BL != 0.0 {
                            AY
                        } else {
                            BK
                        };
                        BM = BO;
                    } else {
                        BM = V;
                    }
                    let BN = if AZ > V { 1.0 } else { 0.0 };
                    oBN = BN;
                    let BQ;
                    if BN != 0.0 {
                        let BP = if AZ < K { 1.0 } else { 0.0 };
                        oBP = BP;
                        let BS = if BP != 0.0 {
                            AZ
                        } else {
                            K
                        };
                        BQ = BS;
                    } else {
                        BQ = V;
                    }
                    let BR = if BA > V { 1.0 } else { 0.0 };
                    oBR = BR;
                    let BU;
                    if BR != 0.0 {
                        let BT = if BA < BK { 1.0 } else { 0.0 };
                        oBT = BT;
                        let BW = if BT != 0.0 {
                            BA
                        } else {
                            BK
                        };
                        BU = BW;
                    } else {
                        BU = V;
                    }
                    let BV = if BC > V { 1.0 } else { 0.0 };
                    oBV = BV;
                    let BY;
                    if BV != 0.0 {
                        let BX = if BC < BK { 1.0 } else { 0.0 };
                        oBX = BX;
                        let CA = if BX != 0.0 {
                            BC
                        } else {
                            BK
                        };
                        BY = CA;
                    } else {
                        BY = V;
                    }
                    let BZ = if T > V { 1.0 } else { 0.0 };
                    oBZ = BZ;
                    let CD;
                    if BZ != 0.0 {
                        let CC = if T < CB { 1.0 } else { 0.0 };
                        oCC = CC;
                        let CJ = if CC != 0.0 {
                            T
                        } else {
                            CB
                        };
                        CD = CJ;
                    } else {
                        CD = V;
                    }
                    let CE = M / BM;
                    let CF = M / BQ;
                    let CG = M / BU;
                    let CH = M / BY;
                    let CI = ((1.2e1f64 * CD) * AX) / BB;
                    BF = CE;
                    BG = CF;
                    BH = CG;
                    BI = CH;
                    BJ = CI;
                } else {
                    BF = BE;
                    BG = BE;
                    BH = BE;
                    BI = BE;
                    BJ = BE;
                }
                let CQ;
                let CR;
                let CS;
                let CT;
                let CU;
                let CV;
                let CW;
                let CX;
                if CK != 0.0 {
                    let CL = H.powf(parameters[52]);
                    let CM = staged[9] * CL;
                    let CN = staged[10] * CL;
                    let CO = staged[11] * CL;
                    let CP = staged[12] * CL;
                    let CZ = AT * ((CY * Z) + U);
                    let DA = AT * ((CY * AA) + U);
                    let DB = parameters[57] * J;
                    let DC = parameters[62] * J;
                    CQ = CN;
                    CR = CP;
                    CS = DC;
                    CT = DA;
                    CU = CZ;
                    CV = DB;
                    CW = CM;
                    CX = CO;
                } else {
                    CQ = BE;
                    CR = BE;
                    CS = BE;
                    CT = BE;
                    CU = BE;
                    CV = BE;
                    CW = BE;
                    CX = BE;
                }
                let DD = 1.25e0f64 + (AL * 7.324648775608221e-1f64);
                let DE = I.sqrt();
                let DI = if DF != 0.0 {
                    let DH = DG * U;
                    DH
                } else {
                    BE
                };
                let DJ = if CQ > BE { 1.0 } else { 0.0 };
                let DK = if CR > BE { 1.0 } else { 0.0 };
                let DL = if DJ != 0.0 || DK != 0.0 { 1.0 } else { 0.0 };
                let DM = if staged[41] != 0.0 && DL != 0.0 { 1.0 } else { 0.0 };
                if CK != 0.0 {
                    if DL != 0.0 {
                        let DN = if (if DG == M { 1.0 } else { 0.0 }) != 0.0 && DK != 0.0 { 1.0 } else { 0.0 };
                        oDN = DN;
                        if DN != 0.0 {
                            let DS = if DR < BE { 1.0 } else { 0.0 };
                            oDS = DS;
                            if DT != 0.0 {
                                let DU = U - CT;
                                oDU = DU;
                            } else {
                                let DV = U - CU;
                                oDV = DV;
                            }
                        } else {
                        }
                        if DJ != 0.0 {
                            let DX = if DW < BE { 1.0 } else { 0.0 };
                            oDX = DX;
                        } else {
                        }
                    } else {
                    }
                    let DO = if CW > BE { 1.0 } else { 0.0 };
                    oDO = DO;
                    let DP = if CX > BE { 1.0 } else { 0.0 };
                    let DQ = if DO != 0.0 || DP != 0.0 { 1.0 } else { 0.0 };
                    oDQ = DQ;
                    if DQ != 0.0 {
                        let DY = if (if DG == M { 1.0 } else { 0.0 }) != 0.0 && DP != 0.0 { 1.0 } else { 0.0 };
                        oDY = DY;
                        if DY != 0.0 {
                            let DZ = if DR < BE { 1.0 } else { 0.0 };
                            oDZ = DZ;
                            if EA != 0.0 {
                                let EB = U - CT;
                                oEB = EB;
                            } else {
                                let EC = U - CU;
                                oEC = EC;
                            }
                        } else {
                        }
                        if DO != 0.0 {
                            let ED = if DW < BE { 1.0 } else { 0.0 };
                            oED = ED;
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            [C, D, I, J, L, N, O, U, X, Y, AB, AC, AD, AE, AG, AH, AJ, AK, AL, AM, AN, AO, AQ, oBD, oBL, oBN, oBP, oBR, oBT, oBV, oBX, oBZ, oCC, BF, BG, BH, BI, BJ, DD, AV, DE, DI, CQ, DJ, CR, DL, DM, oDN, CS, oDS, CT, oDU, CU, oDV, CV, oDX, CW, oDO, CX, oDQ, oDY, oDZ, oEB, oEC, oED]
        };
        self.canonical_staged[81] = produced[0];
        self.canonical_staged[82] = produced[1];
        self.canonical_staged[34] = produced[2];
        self.canonical_staged[21] = produced[3];
        self.canonical_staged[37] = produced[4];
        self.canonical_staged[19] = produced[5];
        self.canonical_staged[20] = produced[6];
        self.canonical_staged[15] = produced[7];
        self.canonical_staged[13] = produced[8];
        self.canonical_staged[14] = produced[9];
        self.canonical_staged[18] = produced[10];
        self.canonical_staged[26] = produced[11];
        self.canonical_staged[27] = produced[12];
        self.canonical_staged[28] = produced[13];
        self.canonical_staged[29] = produced[14];
        self.canonical_staged[24] = produced[15];
        self.canonical_staged[23] = produced[16];
        self.canonical_staged[30] = produced[17];
        self.canonical_staged[46] = produced[18];
        self.canonical_staged[45] = produced[19];
        self.canonical_staged[43] = produced[20];
        self.canonical_staged[42] = produced[21];
        self.canonical_staged[89] = produced[22];
        self.canonical_staged[90] = produced[23];
        self.canonical_staged[92] = produced[24];
        self.canonical_staged[93] = produced[25];
        self.canonical_staged[94] = produced[26];
        self.canonical_staged[95] = produced[27];
        self.canonical_staged[96] = produced[28];
        self.canonical_staged[97] = produced[29];
        self.canonical_staged[98] = produced[30];
        self.canonical_staged[99] = produced[31];
        self.canonical_staged[100] = produced[32];
        self.canonical_staged[70] = produced[33];
        self.canonical_staged[71] = produced[34];
        self.canonical_staged[73] = produced[35];
        self.canonical_staged[72] = produced[36];
        self.canonical_staged[39] = produced[37];
        self.canonical_staged[44] = produced[38];
        self.canonical_staged[25] = produced[39];
        self.canonical_staged[35] = produced[40];
        self.canonical_staged[40] = produced[41];
        self.canonical_staged[60] = produced[42];
        self.canonical_staged[111] = produced[43];
        self.canonical_staged[53] = produced[44];
        self.canonical_staged[108] = produced[45];
        self.canonical_staged[107] = produced[46];
        self.canonical_staged[109] = produced[47];
        self.canonical_staged[47] = produced[48];
        self.canonical_staged[112] = produced[49];
        self.canonical_staged[57] = produced[50];
        self.canonical_staged[50] = produced[51];
        self.canonical_staged[58] = produced[52];
        self.canonical_staged[51] = produced[53];
        self.canonical_staged[54] = produced[54];
        self.canonical_staged[113] = produced[55];
        self.canonical_staged[66] = produced[56];
        self.canonical_staged[115] = produced[57];
        self.canonical_staged[64] = produced[58];
        self.canonical_staged[110] = produced[59];
        self.canonical_staged[114] = produced[60];
        self.canonical_staged[116] = produced[61];
        self.canonical_staged[61] = produced[62];
        self.canonical_staged[62] = produced[63];
        self.canonical_staged[117] = produced[64];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 16846 => 0usize, 16850 => 1usize, 16852 => 2usize, _ => usize::MAX };
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
            let A = staged[75];
            let B = parameters[16];
            let C = parameters[49];
            let D = node_potentials[4];
            let E = node_potentials[5];
            let G = 1e0f64;
            let H = 1e0f64;
            let I = parameters[17];
            let L = 1e-16f64;
            let O = parameters[28];
            let Q = 2e0f64;
            let R = 1e0f64;
            let S = 5e-1f64;
            let V = 0e0f64;
            let X = -1e0f64;
            let AC = parameters[26];
            let AE = 1e0f64;
            let AG = parameters[25];
            let AP = 1e-32f64;
            let AV = 1e-6f64;
            let BD = parameters[24];
            let BG = 1e23f64;
            let BJ = staged[13];
            let BL = staged[14];
            let BO = 3.348580862e-29f64;
            let BQ = staged[16];
            let CH = 6.666666666666666e-1f64;
            let CI = staged[17];
            let CN = 1.3333333333333333e0f64;
            let CW = staged[18];
            let DE = 7.071067811865475e-1f64;
            let DJ = 1e-5f64;
            let DL = staged[19];
            let DO = 4.6051701859880916e2f64;
            let DU = 3.333333333333333e-1f64;
            let DY = 1e-200f64;
            let ED = 7.324648775608221e-1f64;
            let EF = 1.25e0f64;
            let EM = 1.666666666666667e-1f64;
            let EY = staged[103];
            let FE = 1e1f64;
            let FF = 6e0f64;
            let FI = 6.4e1f64;
            let FS = 2e0f64;
            let GP = 2.3025850929940458e2f64;
            let GR = 7.324648775608221e-1f64;
            let HL = 1e100f64;
            let IT = 1e-100f64;
            let JA = 2.5e-1f64;
            let JE = 3e0f64;
            let JK = 5e0f64;
            let KF = 1e-40f64;
            let KS = 1e-120f64;
            let NY = staged[21];
            let NZ = staged[22];
            let OC = staged[23];
            let OJ = staged[24];
            let OM = staged[25];
            let OO = staged[26];
            let PK = staged[27];
            let PP = staged[28];
            let QM = staged[29];
            let SM = staged[30];
            let AFR = Lanes([0e0f64; 2]);
            let AFX = node_potentials[6];
            let AGA = 1e0f64;
            let AIH = 1.75e0f64;
            let AND = 4e0f64;
            let AOZ = staged[32];
            let APD = Lanes([0e0f64; 3]);
            let BEP = 1.62e0f64;
            let BER = staged[33];
            let BES = staged[34];
            let BET = staged[35];
            let BGV = staged[36];
            let BGY = staged[104];
            let BHO = -1.666666666666667e-1f64;
            let BHP = staged[38];
            let BHZ = 1e-2f64;
            let BIH = -1e0f64;
            let BIP = staged[39];
            let BIQ = parameters[41];
            let BJF = node_potentials[1];
            let BJH = 1e0f64;
            let BJL = staged[107];
            let BJM = staged[42];
            let BJO = Lanes([0e0f64; 2]);
            let BJT = staged[43];
            let BKB = staged[44];
            let BKU = staged[45];
            let BMJ = staged[46];
            let BOW = staged[108];
            let BPB = staged[67];
            let BPC = staged[68];
            let BPG = parameters[22];
            let BPJ = node_potentials[3];
            let BPK = 1e0f64;
            let BPL = staged[69];
            let BPP = staged[109];
            let BPS = staged[110];
            let BPU = staged[47];
            let BQB = staged[111];
            let BQL = staged[48];
            let BQO = staged[112];
            let BQZ = staged[49];
            let BRF = 1.0f64;
            let BSU = parameters[64];
            let BSV = parameters[63];
            let BSX = staged[52];
            let BTJ = staged[53];
            let BUB = staged[54];
            let BUR = staged[55];
            let BUU = staged[113];
            let BVF = staged[56];
            let BVL = 1.0f64;
            let BWE = staged[57];
            let BWH = staged[58];
            let BXC = parameters[59];
            let BXD = parameters[58];
            let BXF = staged[59];
            let BXO = staged[60];
            let BYM = staged[114];
            let BYW = staged[115];
            let BZI = staged[116];
            let BZY = 0.0f64;
            let CBP = staged[63];
            let CCB = staged[64];
            let CDK = staged[117];
            let CEA = 0.0f64;
            let CFR = staged[65];
            let CGA = staged[66];
            let CGV = node_potentials[0];
            let CGW = 1e0f64;
            let CGX = staged[70];
            let CHA = staged[71];
            let CHJ = node_potentials[2];
            let CHK = 1e0f64;
            let CHL = staged[73];
            let CHO = Lanes([0e0f64; 2]);
            let CHP = Lanes([0e0f64; 2]);
            let CHQ = Lanes([0e0f64; 4]);
            let CHR = Lanes([0e0f64; 2]);
            let CIG = ddt_scale();
            let F = D - E;
            let J = I * (F - parameters[27]);
            let K = (Lanes([G, 0.0]) - Lanes([0.0, H])) * I;
            let M = if J > L { 1.0 } else { 0.0 };
            let AA;
            let AB;
            if M != 0.0 {
                let N = K * J;
                let P = ((J * J) + O).sqrt();
                let T = S * (J + P);
                let U = (K + ((N + N) * (R / (Q * P)))) * S;
                AA = T;
                AB = U;
            } else {
                let W = V - J;
                let Y = K * X;
                let Z = if W > L { 1.0 } else { 0.0 };
                let AS;
                let AT;
                if Z != 0.0 {
                    let AK = Y * W;
                    let AL = ((W * W) + O).sqrt();
                    let AM = W + AL;
                    let AN = (S * O) / AM;
                    let AO = (((Y + ((AK + AK) * (R / (Q * AL)))) * AN) * X) / AM;
                    AS = AN;
                    AT = AO;
                } else {
                    let AQ = S * (J + ((AP + O).sqrt()));
                    let AR = K * S;
                    AS = AQ;
                    AT = AR;
                }
                AA = AS;
                AB = AT;
            }
            let AD = AB * AC;
            let AF = AE + (AC * AA);
            let AH = AG - AF;
            let AI = AD * X;
            let AJ = if AH > L { 1.0 } else { 0.0 };
            let BB;
            let BC;
            if AJ != 0.0 {
                let AU = AI * AH;
                let AW = ((AH * AH) + AV).sqrt();
                let AX = AG - (S * (AH + AW));
                let AY = ((AI + ((AU + AU) * (R / (Q * AW)))) * S) * X;
                BB = AX;
                BC = AY;
            } else {
                let AZ = AF - AG;
                let BA = if AZ > L { 1.0 } else { 0.0 };
                let CB;
                let CC;
                if BA != 0.0 {
                    let BT = AD * AZ;
                    let BU = ((AZ * AZ) + AV).sqrt();
                    let BV = AZ + BU;
                    let BW = 5e-7f64 / BV;
                    let BX = AG - BW;
                    let BY = ((((AD + ((BT + BT) * (R / (Q * BU)))) * BW) * X) / BV) * X;
                    CB = BX;
                    CC = BY;
                } else {
                    let BZ = AG - (S * (AH + 1e-3f64));
                    let CA = (AI * S) * X;
                    CB = BZ;
                    CC = CA;
                }
                BB = CB;
                BC = CC;
            }
            let BE = BD * BB;
            let BF = BC * BD;
            let BH = BE / BG;
            let BI = BF / BG;
            let BK = BE * BJ;
            let BM = ((BF * BJ) * (R / BK)) * BL;
            let BN = staged[15] + (BL * (BK.ln()));
            let BP = (BO * BE).sqrt();
            let BR = BP / BQ;
            let BS = ((BF * BO) * (R / (Q * BP))) / BQ;
            let CS;
            let CT;
            let CU;
            let CV;
            if A != 0.0 {
                let CD = BR * BR;
                let CE = BS * BR;
                let CF = (CD * BN).sqrt();
                let CG = (((CE + CE) * BN) + (BM * CD)) * (R / (Q * CF));
                let CJ = CI * (CF.powf(CH));
                let CK = (CG * (CH * (CF.powf(-3.3333333333333337e-1f64)))) * CI;
                let CL = BN + CJ;
                let CM = BM + CK;
                let CO = (CN * CJ) / CF;
                let CP = AE + CO;
                let CQ = BR * CP;
                let CR = (BS * CP) + ((((CK * CN) - (CG * CO)) / CF) * BR);
                CS = CQ;
                CT = CL;
                CU = CR;
                CV = CM;
            } else {
                CS = BR;
                CT = BN;
                CU = BS;
                CV = BM;
            }
            let CX = CS * CW;
            let CY = CU * CW;
            let CZ = CX * CX;
            let DA = CY * CX;
            let DB = DA + DA;
            let DC = AE / CZ;
            let DD = ((DB * DC) * X) / CZ;
            let DF = CY * DE;
            let DG = AE + (CX * DE);
            let DH = AE / DG;
            let DI = ((DF * DH) * X) / DG;
            let DK = DJ * DG;
            let DM = CT * DL;
            let DN = CV * DL;
            let DP = if DM < DO { 1.0 } else { 0.0 };
            let EB;
            let EC;
            if DP != 0.0 {
                let DQ = (-DM).exp();
                let DR = (DN * X) * DQ;
                EB = DQ;
                EC = DR;
            } else {
                let DS = DM - DO;
                let DT = S * DS;
                let DV = AE + (DS * DU);
                let DW = AE + (DT * DV);
                let DX = AE + (DS * DW);
                let DZ = DY / DX;
                let EA = ((((DN * DW) + ((((DN * S) * DV) + ((DN * DU) * DT)) * DS)) * DZ) * X) / DX;
                EB = DZ;
                EC = EA;
            }
            let EE = CY * ED;
            let EG = EF + (CX * ED);
            let EH = I * (F - staged[20]);
            let EI = EH * DL;
            let EJ = K * DL;
            let EK = if (EI.abs()) <= DK { 1.0 } else { 0.0 };
            let EW;
            let EX;
            if EK != 0.0 {
                let EL = DI * DH;
                let EN = ((DH * DH) * EM) * DE;
                let EO = EI * DH;
                let EP = AE - EB;
                let EQ = EI * EP;
                let ER = EQ * CX;
                let ES = AE + (ER * EN);
                let ET = EO * ES;
                let EU = (((EJ * DH) + (DI * EI)) * ES) + (((((((EJ * EP) + ((EC * X) * EI)) * CX) + (CY * EQ)) * EN) + ((((EL + EL) * EM) * DE) * ER)) * EO);
                EW = ET;
                EX = EU;
            } else {
                let EV = if EI < (-DK) { 1.0 } else { 0.0 };
                let HD;
                let HE;
                if EV != 0.0 {
                    let EZ = -EI;
                    let FA = EJ * X;
                    let FB = EF * EZ;
                    let FC = FB * DH;
                    let FD = ((FA * EF) * DH) + (DI * FB);
                    let FG = FC - FF;
                    let FH = FD * FG;
                    let FJ = ((FG * FG) + FI).sqrt();
                    let FK = S * ((FC + FE) - FJ);
                    let FL = (FD - ((FH + FH) * (R / (Q * FJ)))) * S;
                    let FM = EZ - FK;
                    let FN = FA - FL;
                    let FO = FN * FM;
                    let FP = FK + AE;
                    let FQ = (FM * FM) + (CZ * FP);
                    let FR = (FO + FO) + ((DB * FP) + (FL * CZ));
                    let FT = (FS * FM) - CZ;
                    let FU = (FN * FS) - DB;
                    let FV = FQ * DC;
                    let FW = (-FK) + (FV.ln());
                    let FX = (FL * X) + (((FR * DC) + (DD * FQ)) * (R / FV));
                    let FY = FQ + FT;
                    let FZ = FR + FU;
                    let GA = FZ * FY;
                    let GB = S * FT;
                    let GC = (GB * FT) - FQ;
                    let GD = (FY * FY) + (GC * FW);
                    let GE = (GA + GA) + ((((((FU * S) * FT) + (FU * GB)) - FR) * FW) + (FX * GC));
                    let GF = FQ * FY;
                    let GG = FY * FW;
                    let GH = (GG * FW) / GD;
                    let GI = GH * FT;
                    let GJ = FU * FT;
                    let GK = ((FT * FT) * DU) - FQ;
                    let GL = GD + (GI * GK);
                    let GM = (GF * FW) / GL;
                    let GN = FK + GM;
                    let GO = FL + ((((((FR * FY) + (FZ * FQ)) * FW) + (FX * GF)) - ((GE + ((((((((((FZ * FW) + (FX * FY)) * FW) + (FX * GG)) - (GE * GH)) / GD) * FT) + (FU * GH)) * GK) + ((((GJ + GJ) * DU) - FR) * GI))) * GM)) / GL);
                    let GQ = if GN < GP { 1.0 } else { 0.0 };
                    let HO;
                    let HP;
                    if GQ != 0.0 {
                        let HF = GN.exp();
                        let HG = GO * HF;
                        HO = HF;
                        HP = HG;
                    } else {
                        let HH = GN - GP;
                        let HI = S * HH;
                        let HJ = AE + (HH * DU);
                        let HK = AE + (HI * HJ);
                        let HM = HL * (AE + (HH * HK));
                        let HN = ((GO * HK) + ((((GO * S) * HJ) + ((GO * DU) * HI)) * HH)) * HL;
                        HO = HM;
                        HP = HN;
                    }
                    let HQ = AE / HO;
                    let HR = EZ - GN;
                    let HS = FA - GO;
                    let HT = EB * HQ;
                    let HU = (EC * HQ) + ((((HP * HQ) * X) / HO) * EB);
                    let HV = ((HO - AE) - HT) + EB;
                    let HW = (FS * HR) + (CZ * HV);
                    let HX = (HS * FS) + ((DB * HV) + (((HP - HU) + EC) * CZ));
                    let HY = HS * HR;
                    let HZ = GN - AE;
                    let IA = (((HO - GN) - AE) + HT) + (EB * HZ);
                    let IB = HO + HT;
                    let IC = FS - (CZ * IB);
                    let ID = HX * HW;
                    let IE = FS * ((HR * HR) - (CZ * IA));
                    let IF = ((HY + HY) - ((DB * IA) + ((((HP - GO) + HU) + ((EC * HZ) + (GO * EB))) * CZ))) * FS;
                    let IG = ((HW * HW) - (IE * IC)).sqrt();
                    let IH = HW + IG;
                    let II = IE / IH;
                    let IJ = (-GN) - II;
                    let IK = (GO * X) - ((IF - ((HX + (((ID + ID) - ((IF * IC) + ((((DB * IB) + ((HP + HU) * CZ)) * X) * IE))) * (R / (Q * IG)))) * II)) / IH);
                    HD = IJ;
                    HE = IK;
                } else {
                    let GS = EF + (CX * GR);
                    let GT = AE / GS;
                    let GU = (((CY * GR) * GT) * X) / GS;
                    let GV = DG * EF;
                    let GW = (GV * GT) - AE;
                    let GX = GW * GT;
                    let GY = EI * DH;
                    let GZ = AE + (GX * EI);
                    let HA = -(GY * GZ);
                    let HB = ((((EJ * DH) + (DI * EI)) * GZ) + ((((((((DF * EF) * GT) + (GU * GV)) * GT) + (GU * GW)) * EI) + (EJ * GX)) * GY)) * X;
                    let HC = if HA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    let IW;
                    let IX;
                    if HC != 0.0 {
                        let IL = HA.exp();
                        let IM = HB * IL;
                        IW = IL;
                        IX = IM;
                    } else {
                        let IN = -2.3025850929940458e2f64 - HA;
                        let IO = HB * X;
                        let IP = S * (-2.3025850929940458e2f64 - HA);
                        let IQ = AE + ((-2.3025850929940458e2f64 - HA) * DU);
                        let IR = AE + (IP * IQ);
                        let IS = AE + (IN * IR);
                        let IU = IT / IS;
                        let IV = ((((IO * IR) + ((((IO * S) * IQ) + ((IO * DU) * IP)) * IN)) * IU) * X) / IS;
                        IW = IU;
                        IX = IV;
                    }
                    let IY = CZ * S;
                    let IZ = DB * S;
                    let JB = ((EI + (CZ * JA)) - (AE - IW)).sqrt();
                    let JC = (EI + IY) - (CX * JB);
                    let JD = (EJ + IZ) - ((CY * JB) + ((((EJ + (DB * JA)) - (IX * X)) * (R / (Q * JB))) * CX));
                    let JF = DM + JE;
                    let JG = JF - JC;
                    let JH = DN - JD;
                    let JI = if JG > L { 1.0 } else { 0.0 };
                    let JR;
                    let JS;
                    if JI != 0.0 {
                        let JJ = JH * JG;
                        let JL = ((JG * JG) + JK).sqrt();
                        let JM = JF - (S * (JG + JL));
                        let JN = DN - ((JH + ((JJ + JJ) * (R / (Q * JL)))) * S);
                        JR = JM;
                        JS = JN;
                    } else {
                        let JO = JC - JF;
                        let JP = JD - DN;
                        let JQ = if JO > L { 1.0 } else { 0.0 };
                        let LC;
                        let LD;
                        if JQ != 0.0 {
                            let KU = JP * JO;
                            let KV = ((JO * JO) + JK).sqrt();
                            let KW = JO + KV;
                            let KX = 2.5e0f64 / KW;
                            let KY = JF - KX;
                            let KZ = DN - ((((JP + ((KU + KU) * (R / (Q * KV)))) * KX) * X) / KW);
                            LC = KY;
                            LD = KZ;
                        } else {
                            let LA = JF - (S * (JG + 2.23606797749979e0f64));
                            let LB = DN - (JH * S);
                            LC = LA;
                            LD = LB;
                        }
                        JR = LC;
                        JS = LD;
                    }
                    let JT = DN * JF;
                    let JU = ((JF * JF) + JK).sqrt();
                    let JV = JR - (S * (JF - JU));
                    let JW = JS - ((DN - ((JT + JT) * (R / (Q * JU)))) * S);
                    let JX = EI - JV;
                    let JY = EJ - JW;
                    let JZ = (-JV).exp();
                    let KA = (JW * X) * JZ;
                    let KB = JY * JX;
                    let KC = JV + AE;
                    let KD = ((JZ + JV) - AE) - (EB * KC);
                    let KE = (JX * JX) - (CZ * KD);
                    let KG = if KF >= KE { KF } else { KE };
                    let KH = ((KB + KB) - ((DB * KD) + (((KA + JW) - ((EC * KC) + (JW * EB))) * CZ))) * (R - (if KF >= KE { 1.0 } else { 0.0 }));
                    let KI = AE - (IY * JZ);
                    let KJ = ((IZ * JZ) + (KA * IY)) * X;
                    let KK = (AE - JZ) - EB;
                    let KL = (FS * JX) + (CZ * KK);
                    let KM = (JY * FS) + ((DB * KK) + (((KA * X) - EC) * CZ));
                    let KN = KG / CZ;
                    let KO = (DM - JV) + (KN.ln());
                    let KP = (DN - JW) + (((KH - (DB * KN)) / CZ) * (R / KN));
                    let KQ = KG + KL;
                    let KR = KH + KM;
                    let KT = if (KO.abs()) < KS { 1.0 } else { 0.0 };
                    let LV;
                    let LW;
                    if KT != 0.0 {
                        LV = JV;
                        LW = JW;
                    } else {
                        let LE = KR * KQ;
                        let LF = S * KL;
                        let LG = KG * KI;
                        let LH = (KH * KI) + (KJ * KG);
                        let LI = (LF * KL) - LG;
                        let LJ = (KQ * KQ) + (LI * KO);
                        let LK = (LE + LE) + ((((((KM * S) * KL) + (KM * LF)) - LH) * KO) + (KP * LI));
                        let LL = KG * KQ;
                        let LM = KQ * KO;
                        let LN = (LM * KO) / LJ;
                        let LO = LN * KL;
                        let LP = KM * KL;
                        let LQ = ((KL * KL) * DU) - LG;
                        let LR = LJ + (LO * LQ);
                        let LS = (LL * KO) / LR;
                        let LT = JV + LS;
                        let LU = JW + ((((((KH * KQ) + (KR * KG)) * KO) + (KP * LL)) - ((LK + ((((((((((KR * KO) + (KP * KQ)) * KO) + (KP * LM)) - (LK * LN)) / LJ) * KL) + (KM * LN)) * LQ) + ((((LP + LP) * DU) - LH) * LO))) * LS)) / LR);
                        LV = LT;
                        LW = LU;
                    }
                    let LX = if LV < GP { 1.0 } else { 0.0 };
                    let MF;
                    let MG;
                    let MH;
                    let MI;
                    if LX != 0.0 {
                        let LY = LV.exp();
                        let LZ = LW * LY;
                        let MA = AE / LY;
                        let MB = ((LZ * MA) * X) / LY;
                        let MC = EB * LY;
                        let MD = (EC * LY) + (LZ * EB);
                        MF = MA;
                        MG = MC;
                        MH = MB;
                        MI = MD;
                    } else {
                        let ME = if LV > (DM - GP) { 1.0 } else { 0.0 };
                        let NU;
                        let NV;
                        let NW;
                        let NX;
                        if ME != 0.0 {
                            let NB = (LV - DM).exp();
                            let NC = (LW - DN) * NB;
                            let ND = EB / NB;
                            let NE = (EC - (NC * ND)) / NB;
                            NU = ND;
                            NV = NB;
                            NW = NE;
                            NX = NC;
                        } else {
                            let NF = DN - LW;
                            let NG = (DM - LV) - GP;
                            let NH = S * NG;
                            let NI = AE + (NG * DU);
                            let NJ = AE + (NH * NI);
                            let NK = AE + (NG * NJ);
                            let NL = IT / NK;
                            let NM = ((((NF * NJ) + ((((NF * S) * NI) + ((NF * DU) * NH)) * NG)) * NL) * X) / NK;
                            let NN = LV - GP;
                            let NO = S * NN;
                            let NP = AE + (NN * DU);
                            let NQ = AE + (NO * NP);
                            let NR = AE + (NN * NQ);
                            let NS = IT / NR;
                            let NT = ((((LW * NQ) + ((((LW * S) * NP) + ((LW * DU) * NO)) * NN)) * NS) * X) / NR;
                            NU = NS;
                            NV = NL;
                            NW = NT;
                            NX = NM;
                        }
                        MF = NU;
                        MG = NV;
                        MH = NW;
                        MI = NX;
                    }
                    let MJ = EI - LV;
                    let MK = EJ - LW;
                    let ML = ((AE - MF) + MG) - EB;
                    let MM = (FS * MJ) + (CZ * ML);
                    let MN = (MK * FS) + ((DB * ML) + ((((MH * X) + MI) - EC) * CZ));
                    let MO = MK * MJ;
                    let MP = LV + AE;
                    let MQ = (((MF + LV) - AE) + MG) - (EB * MP);
                    let MR = MF + MG;
                    let MS = FS - (CZ * MR);
                    let MT = MN * MM;
                    let MU = FS * ((MJ * MJ) - (CZ * MQ));
                    let MV = ((MO + MO) - ((DB * MQ) + ((((MH + LW) + MI) - ((EC * MP) + (LW * EB))) * CZ))) * FS;
                    let MW = ((MM * MM) - (MU * MS)).sqrt();
                    let MX = MM + MW;
                    let MY = MU / MX;
                    let MZ = LV + MY;
                    let NA = LW + ((MV - ((MN + (((MT + MT) - ((MV * MS) + ((((DB * MR) + ((MH + MI) * CZ)) * X) * MU))) * (R / (Q * MW)))) * MY)) / MX);
                    HD = MZ;
                    HE = NA;
                }
                EW = HD;
                EX = HE;
            }
            let OE;
            let OF;
            let OG;
            let OH;
            if EY != 0.0 {
                let OA = (NZ * (EH - (EW * NY))) * DL;
                let OB = ((K - (EX * NY)) * NZ) * DL;
                let OD = if (OA.abs()) <= OC { 1.0 } else { 0.0 };
                let OT;
                let OU;
                if OD != 0.0 {
                    let OK = ((OJ * OJ) * EM) * DE;
                    let OL = OA * OJ;
                    let ON = AE - OM;
                    let OP = AE + (((OA * ON) * OO) * OK);
                    let OQ = OL * OP;
                    let OR = ((OB * OJ) * OP) + ((((OB * ON) * OO) * OK) * OL);
                    OT = OQ;
                    OU = OR;
                } else {
                    let OS = if OA < (-OC) { 1.0 } else { 0.0 };
                    let QT;
                    let QU;
                    if OS != 0.0 {
                        let OY = -OA;
                        let OZ = OB * X;
                        let PA = (EF * OY) * OJ;
                        let PB = (OZ * EF) * OJ;
                        let PC = PA - FF;
                        let PD = PB * PC;
                        let PE = ((PC * PC) + FI).sqrt();
                        let PF = S * ((PA + FE) - PE);
                        let PG = (PB - ((PD + PD) * (R / (Q * PE)))) * S;
                        let PH = OY - PF;
                        let PI = OZ - PG;
                        let PJ = PI * PH;
                        let PL = (PH * PH) + (PK * (PF + AE));
                        let PM = (PJ + PJ) + (PG * PK);
                        let PN = PI * FS;
                        let PO = (FS * PH) - PK;
                        let PQ = PL * PP;
                        let PR = (-PF) + (PQ.ln());
                        let PS = (PG * X) + ((PM * PP) * (R / PQ));
                        let PT = PL + PO;
                        let PU = PM + PN;
                        let PV = PU * PT;
                        let PW = S * PO;
                        let PX = (PW * PO) - PL;
                        let PY = (PT * PT) + (PX * PR);
                        let PZ = (PV + PV) + ((((((PN * S) * PO) + (PN * PW)) - PM) * PR) + (PS * PX));
                        let QA = PL * PT;
                        let QB = PT * PR;
                        let QC = (QB * PR) / PY;
                        let QD = QC * PO;
                        let QE = PN * PO;
                        let QF = ((PO * PO) * DU) - PL;
                        let QG = PY + (QD * QF);
                        let QH = (QA * PR) / QG;
                        let QI = PF + QH;
                        let QJ = PG + ((((((PM * PT) + (PU * PL)) * PR) + (PS * QA)) - ((PZ + ((((((((((PU * PR) + (PS * PT)) * PR) + (PS * QB)) - (PZ * QC)) / PY) * PO) + (PN * QC)) * QF) + ((((QE + QE) * DU) - PM) * QD))) * QH)) / QG);
                        let QK = if QI < GP { 1.0 } else { 0.0 };
                        let RD;
                        let RE;
                        if QK != 0.0 {
                            let QV = QI.exp();
                            let QW = QJ * QV;
                            RD = QV;
                            RE = QW;
                        } else {
                            let QX = QI - GP;
                            let QY = S * QX;
                            let QZ = AE + (QX * DU);
                            let RA = AE + (QY * QZ);
                            let RB = HL * (AE + (QX * RA));
                            let RC = ((QJ * RA) + ((((QJ * S) * QZ) + ((QJ * DU) * QY)) * QX)) * HL;
                            RD = RB;
                            RE = RC;
                        }
                        let RF = AE / RD;
                        let RG = OY - QI;
                        let RH = OZ - QJ;
                        let RI = OM * RF;
                        let RJ = (((RE * RF) * X) / RD) * OM;
                        let RK = (FS * RG) + (PK * (((RD - AE) - RI) + OM));
                        let RL = (RH * FS) + ((RE - RJ) * PK);
                        let RM = RH * RG;
                        let RN = FS - (PK * (RD + RI));
                        let RO = RL * RK;
                        let RP = FS * ((RG * RG) - (PK * ((((RD - QI) - AE) + RI) + (OM * (QI - AE)))));
                        let RQ = ((RM + RM) - ((((RE - QJ) + RJ) + (QJ * OM)) * PK)) * FS;
                        let RR = ((RK * RK) - (RP * RN)).sqrt();
                        let RS = RK + RR;
                        let RT = RP / RS;
                        let RU = (-QI) - RT;
                        let RV = (QJ * X) - ((RQ - ((RL + (((RO + RO) - ((RQ * RN) + ((((RE + RJ) * PK) * X) * RP))) * (R / (Q * RR)))) * RT)) / RS);
                        QT = RU;
                        QU = RV;
                    } else {
                        let QL = AE / (EF + (OO * GR));
                        let QN = (((QM * EF) * QL) - AE) * QL;
                        let QO = OA * OJ;
                        let QP = AE + (QN * OA);
                        let QQ = -(QO * QP);
                        let QR = (((OB * OJ) * QP) + ((OB * QN) * QO)) * X;
                        let QS = if QQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let SG;
                        let SH;
                        if QS != 0.0 {
                            let RW = QQ.exp();
                            let RX = QR * RW;
                            SG = RW;
                            SH = RX;
                        } else {
                            let RY = -2.3025850929940458e2f64 - QQ;
                            let RZ = QR * X;
                            let SA = S * (-2.3025850929940458e2f64 - QQ);
                            let SB = AE + ((-2.3025850929940458e2f64 - QQ) * DU);
                            let SC = AE + (SA * SB);
                            let SD = AE + (RY * SC);
                            let SE = IT / SD;
                            let SF = ((((RZ * SC) + ((((RZ * S) * SB) + ((RZ * DU) * SA)) * RY)) * SE) * X) / SD;
                            SG = SE;
                            SH = SF;
                        }
                        let SI = PK * S;
                        let SJ = ((OA + (PK * JA)) - (AE - SG)).sqrt();
                        let SK = (OA + SI) - (OO * SJ);
                        let SL = OB - (((OB - (SH * X)) * (R / (Q * SJ))) * OO);
                        let SN = SM + JE;
                        let SO = SN - SK;
                        let SP = SL * X;
                        let SQ = if SO > L { 1.0 } else { 0.0 };
                        let SX;
                        let SY;
                        if SQ != 0.0 {
                            let SR = SP * SO;
                            let SS = ((SO * SO) + JK).sqrt();
                            let ST = SN - (S * (SO + SS));
                            let SU = ((SP + ((SR + SR) * (R / (Q * SS)))) * S) * X;
                            SX = ST;
                            SY = SU;
                        } else {
                            let SV = SK - SN;
                            let SW = if SV > L { 1.0 } else { 0.0 };
                            let UB;
                            let UC;
                            if SW != 0.0 {
                                let TT = SL * SV;
                                let TU = ((SV * SV) + JK).sqrt();
                                let TV = SV + TU;
                                let TW = 2.5e0f64 / TV;
                                let TX = SN - TW;
                                let TY = ((((SL + ((TT + TT) * (R / (Q * TU)))) * TW) * X) / TV) * X;
                                UB = TX;
                                UC = TY;
                            } else {
                                let TZ = SN - (S * (SO + 2.23606797749979e0f64));
                                let UA = (SP * S) * X;
                                UB = TZ;
                                UC = UA;
                            }
                            SX = UB;
                            SY = UC;
                        }
                        let SZ = SX - (S * (SN - (((SN * SN) + JK).sqrt())));
                        let TA = OA - SZ;
                        let TB = OB - SY;
                        let TC = SY * X;
                        let TD = (-SZ).exp();
                        let TE = TC * TD;
                        let TF = TB * TA;
                        let TG = (TA * TA) - (PK * (((TD + SZ) - AE) - (OM * (SZ + AE))));
                        let TH = if KF >= TG { KF } else { TG };
                        let TI = ((TF + TF) - (((TE + SY) - (SY * OM)) * PK)) * (R - (if KF >= TG { 1.0 } else { 0.0 }));
                        let TJ = AE - (SI * TD);
                        let TK = (TE * SI) * X;
                        let TL = (FS * TA) + (PK * ((AE - TD) - OM));
                        let TM = (TB * FS) + ((TE * X) * PK);
                        let TN = TH / PK;
                        let TO = (SM - SZ) + (TN.ln());
                        let TP = TC + ((TI / PK) * (R / TN));
                        let TQ = TH + TL;
                        let TR = TI + TM;
                        let TS = if (TO.abs()) < KS { 1.0 } else { 0.0 };
                        let UU;
                        let UV;
                        if TS != 0.0 {
                            UU = SZ;
                            UV = SY;
                        } else {
                            let UD = TR * TQ;
                            let UE = S * TL;
                            let UF = TH * TJ;
                            let UG = (TI * TJ) + (TK * TH);
                            let UH = (UE * TL) - UF;
                            let UI = (TQ * TQ) + (UH * TO);
                            let UJ = (UD + UD) + ((((((TM * S) * TL) + (TM * UE)) - UG) * TO) + (TP * UH));
                            let UK = TH * TQ;
                            let UL = TQ * TO;
                            let UM = (UL * TO) / UI;
                            let UN = UM * TL;
                            let UO = TM * TL;
                            let UP = ((TL * TL) * DU) - UF;
                            let UQ = UI + (UN * UP);
                            let UR = (UK * TO) / UQ;
                            let US = SZ + UR;
                            let UT = SY + ((((((TI * TQ) + (TR * TH)) * TO) + (TP * UK)) - ((UJ + ((((((((((TR * TO) + (TP * TQ)) * TO) + (TP * UL)) - (UJ * UM)) / UI) * TL) + (TM * UM)) * UP) + ((((UO + UO) * DU) - UG) * UN))) * UR)) / UQ);
                            UU = US;
                            UV = UT;
                        }
                        let UW = if UU < GP { 1.0 } else { 0.0 };
                        let VE;
                        let VF;
                        let VG;
                        let VH;
                        if UW != 0.0 {
                            let UX = UU.exp();
                            let UY = UV * UX;
                            let UZ = AE / UX;
                            let VA = ((UY * UZ) * X) / UX;
                            let VB = OM * UX;
                            let VC = UY * OM;
                            VE = UZ;
                            VF = VB;
                            VG = VA;
                            VH = VC;
                        } else {
                            let VD = if UU > (SM - GP) { 1.0 } else { 0.0 };
                            let WP;
                            let WQ;
                            let WR;
                            let WS;
                            if VD != 0.0 {
                                let VW = (UU - SM).exp();
                                let VX = UV * VW;
                                let VY = OM / VW;
                                let VZ = ((VX * VY) * X) / VW;
                                WP = VY;
                                WQ = VW;
                                WR = VZ;
                                WS = VX;
                            } else {
                                let WA = UV * X;
                                let WB = (SM - UU) - GP;
                                let WC = S * WB;
                                let WD = AE + (WB * DU);
                                let WE = AE + (WC * WD);
                                let WF = AE + (WB * WE);
                                let WG = IT / WF;
                                let WH = ((((WA * WE) + ((((WA * S) * WD) + ((WA * DU) * WC)) * WB)) * WG) * X) / WF;
                                let WI = UU - GP;
                                let WJ = S * WI;
                                let WK = AE + (WI * DU);
                                let WL = AE + (WJ * WK);
                                let WM = AE + (WI * WL);
                                let WN = IT / WM;
                                let WO = ((((UV * WL) + ((((UV * S) * WK) + ((UV * DU) * WJ)) * WI)) * WN) * X) / WM;
                                WP = WN;
                                WQ = WG;
                                WR = WO;
                                WS = WH;
                            }
                            VE = WP;
                            VF = WQ;
                            VG = WR;
                            VH = WS;
                        }
                        let VI = OA - UU;
                        let VJ = OB - UV;
                        let VK = (FS * VI) + (PK * (((AE - VE) + VF) - OM));
                        let VL = (VJ * FS) + (((VG * X) + VH) * PK);
                        let VM = VJ * VI;
                        let VN = FS - (PK * (VE + VF));
                        let VO = VL * VK;
                        let VP = FS * ((VI * VI) - (PK * ((((VE + UU) - AE) + VF) - (OM * (UU + AE)))));
                        let VQ = ((VM + VM) - ((((VG + UV) + VH) - (UV * OM)) * PK)) * FS;
                        let VR = ((VK * VK) - (VP * VN)).sqrt();
                        let VS = VK + VR;
                        let VT = VP / VS;
                        let VU = UU + VT;
                        let VV = UV + ((VQ - ((VL + (((VO + VO) - ((VQ * VN) + ((((VG + VH) * PK) * X) * VP))) * (R / (Q * VR)))) * VT)) / VS);
                        QT = VU;
                        QU = VV;
                    }
                    OT = QT;
                    OU = QU;
                }
                let OV = (EH - ((NZ * OT) * NY)) / NY;
                let OW = (K - ((OU * NZ) * NY)) / NY;
                let OX = if (OV.abs()) <= DK { 1.0 } else { 0.0 };
                let XD;
                let XE;
                if OX != 0.0 {
                    let WT = DI * DH;
                    let WU = ((DH * DH) * EM) * DE;
                    let WV = OV * DH;
                    let WW = AE - EB;
                    let WX = OV * WW;
                    let WY = WX * CX;
                    let WZ = AE + (WY * WU);
                    let XA = WV * WZ;
                    let XB = (((OW * DH) + (DI * OV)) * WZ) + (((((((OW * WW) + ((EC * X) * OV)) * CX) + (CY * WX)) * WU) + ((((WT + WT) * EM) * DE) * WY)) * WV);
                    XD = XA;
                    XE = XB;
                } else {
                    let XC = if OV < (-DK) { 1.0 } else { 0.0 };
                    let ZD;
                    let ZE;
                    if XC != 0.0 {
                        let XF = -OV;
                        let XG = OW * X;
                        let XH = EF * XF;
                        let XI = XH * DH;
                        let XJ = ((XG * EF) * DH) + (DI * XH);
                        let XK = XI - FF;
                        let XL = XJ * XK;
                        let XM = ((XK * XK) + FI).sqrt();
                        let XN = S * ((XI + FE) - XM);
                        let XO = (XJ - ((XL + XL) * (R / (Q * XM)))) * S;
                        let XP = XF - XN;
                        let XQ = XG - XO;
                        let XR = XQ * XP;
                        let XS = XN + AE;
                        let XT = (XP * XP) + (CZ * XS);
                        let XU = (XR + XR) + ((DB * XS) + (XO * CZ));
                        let XV = (FS * XP) - CZ;
                        let XW = (XQ * FS) - DB;
                        let XX = XT * DC;
                        let XY = (-XN) + (XX.ln());
                        let XZ = (XO * X) + (((XU * DC) + (DD * XT)) * (R / XX));
                        let YA = XT + XV;
                        let YB = XU + XW;
                        let YC = YB * YA;
                        let YD = S * XV;
                        let YE = (YD * XV) - XT;
                        let YF = (YA * YA) + (YE * XY);
                        let YG = (YC + YC) + ((((((XW * S) * XV) + (XW * YD)) - XU) * XY) + (XZ * YE));
                        let YH = XT * YA;
                        let YI = YA * XY;
                        let YJ = (YI * XY) / YF;
                        let YK = YJ * XV;
                        let YL = XW * XV;
                        let YM = ((XV * XV) * DU) - XT;
                        let YN = YF + (YK * YM);
                        let YO = (YH * XY) / YN;
                        let YP = XN + YO;
                        let YQ = XO + ((((((XU * YA) + (YB * XT)) * XY) + (XZ * YH)) - ((YG + ((((((((((YB * XY) + (XZ * YA)) * XY) + (XZ * YI)) - (YG * YJ)) / YF) * XV) + (XW * YJ)) * YM) + ((((YL + YL) * DU) - XU) * YK))) * YO)) / YN);
                        let YR = if YP < GP { 1.0 } else { 0.0 };
                        let ZN;
                        let ZO;
                        if YR != 0.0 {
                            let ZF = YP.exp();
                            let ZG = YQ * ZF;
                            ZN = ZF;
                            ZO = ZG;
                        } else {
                            let ZH = YP - GP;
                            let ZI = S * ZH;
                            let ZJ = AE + (ZH * DU);
                            let ZK = AE + (ZI * ZJ);
                            let ZL = HL * (AE + (ZH * ZK));
                            let ZM = ((YQ * ZK) + ((((YQ * S) * ZJ) + ((YQ * DU) * ZI)) * ZH)) * HL;
                            ZN = ZL;
                            ZO = ZM;
                        }
                        let ZP = AE / ZN;
                        let ZQ = XF - YP;
                        let ZR = XG - YQ;
                        let ZS = EB * ZP;
                        let ZT = (EC * ZP) + ((((ZO * ZP) * X) / ZN) * EB);
                        let ZU = ((ZN - AE) - ZS) + EB;
                        let ZV = (FS * ZQ) + (CZ * ZU);
                        let ZW = (ZR * FS) + ((DB * ZU) + (((ZO - ZT) + EC) * CZ));
                        let ZX = ZR * ZQ;
                        let ZY = YP - AE;
                        let ZZ = (((ZN - YP) - AE) + ZS) + (EB * ZY);
                        let AAA = ZN + ZS;
                        let AAB = FS - (CZ * AAA);
                        let AAC = ZW * ZV;
                        let AAD = FS * ((ZQ * ZQ) - (CZ * ZZ));
                        let AAE = ((ZX + ZX) - ((DB * ZZ) + ((((ZO - YQ) + ZT) + ((EC * ZY) + (YQ * EB))) * CZ))) * FS;
                        let AAF = ((ZV * ZV) - (AAD * AAB)).sqrt();
                        let AAG = ZV + AAF;
                        let AAH = AAD / AAG;
                        let AAI = (-YP) - AAH;
                        let AAJ = (YQ * X) - ((AAE - ((ZW + (((AAC + AAC) - ((AAE * AAB) + ((((DB * AAA) + ((ZO + ZT) * CZ)) * X) * AAD))) * (R / (Q * AAF)))) * AAH)) / AAG);
                        ZD = AAI;
                        ZE = AAJ;
                    } else {
                        let YS = EF + (CX * GR);
                        let YT = AE / YS;
                        let YU = (((CY * GR) * YT) * X) / YS;
                        let YV = DG * EF;
                        let YW = (YV * YT) - AE;
                        let YX = YW * YT;
                        let YY = OV * DH;
                        let YZ = AE + (YX * OV);
                        let ZA = -(YY * YZ);
                        let ZB = ((((OW * DH) + (DI * OV)) * YZ) + ((((((((DF * EF) * YT) + (YU * YV)) * YT) + (YU * YW)) * OV) + (OW * YX)) * YY)) * X;
                        let ZC = if ZA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AAU;
                        let AAV;
                        if ZC != 0.0 {
                            let AAK = ZA.exp();
                            let AAL = ZB * AAK;
                            AAU = AAK;
                            AAV = AAL;
                        } else {
                            let AAM = -2.3025850929940458e2f64 - ZA;
                            let AAN = ZB * X;
                            let AAO = S * (-2.3025850929940458e2f64 - ZA);
                            let AAP = AE + ((-2.3025850929940458e2f64 - ZA) * DU);
                            let AAQ = AE + (AAO * AAP);
                            let AAR = AE + (AAM * AAQ);
                            let AAS = IT / AAR;
                            let AAT = ((((AAN * AAQ) + ((((AAN * S) * AAP) + ((AAN * DU) * AAO)) * AAM)) * AAS) * X) / AAR;
                            AAU = AAS;
                            AAV = AAT;
                        }
                        let AAW = CZ * S;
                        let AAX = DB * S;
                        let AAY = ((OV + (CZ * JA)) - (AE - AAU)).sqrt();
                        let AAZ = (OV + AAW) - (CX * AAY);
                        let ABA = (OW + AAX) - ((CY * AAY) + ((((OW + (DB * JA)) - (AAV * X)) * (R / (Q * AAY))) * CX));
                        let ABB = DM + JE;
                        let ABC = ABB - AAZ;
                        let ABD = DN - ABA;
                        let ABE = if ABC > L { 1.0 } else { 0.0 };
                        let ABM;
                        let ABN;
                        if ABE != 0.0 {
                            let ABF = ABD * ABC;
                            let ABG = ((ABC * ABC) + JK).sqrt();
                            let ABH = ABB - (S * (ABC + ABG));
                            let ABI = DN - ((ABD + ((ABF + ABF) * (R / (Q * ABG)))) * S);
                            ABM = ABH;
                            ABN = ABI;
                        } else {
                            let ABJ = AAZ - ABB;
                            let ABK = ABA - DN;
                            let ABL = if ABJ > L { 1.0 } else { 0.0 };
                            let ACV;
                            let ACW;
                            if ABL != 0.0 {
                                let ACN = ABK * ABJ;
                                let ACO = ((ABJ * ABJ) + JK).sqrt();
                                let ACP = ABJ + ACO;
                                let ACQ = 2.5e0f64 / ACP;
                                let ACR = ABB - ACQ;
                                let ACS = DN - ((((ABK + ((ACN + ACN) * (R / (Q * ACO)))) * ACQ) * X) / ACP);
                                ACV = ACR;
                                ACW = ACS;
                            } else {
                                let ACT = ABB - (S * (ABC + 2.23606797749979e0f64));
                                let ACU = DN - (ABD * S);
                                ACV = ACT;
                                ACW = ACU;
                            }
                            ABM = ACV;
                            ABN = ACW;
                        }
                        let ABO = DN * ABB;
                        let ABP = ((ABB * ABB) + JK).sqrt();
                        let ABQ = ABM - (S * (ABB - ABP));
                        let ABR = ABN - ((DN - ((ABO + ABO) * (R / (Q * ABP)))) * S);
                        let ABS = OV - ABQ;
                        let ABT = OW - ABR;
                        let ABU = (-ABQ).exp();
                        let ABV = (ABR * X) * ABU;
                        let ABW = ABT * ABS;
                        let ABX = ABQ + AE;
                        let ABY = ((ABU + ABQ) - AE) - (EB * ABX);
                        let ABZ = (ABS * ABS) - (CZ * ABY);
                        let ACA = if KF >= ABZ { KF } else { ABZ };
                        let ACB = ((ABW + ABW) - ((DB * ABY) + (((ABV + ABR) - ((EC * ABX) + (ABR * EB))) * CZ))) * (R - (if KF >= ABZ { 1.0 } else { 0.0 }));
                        let ACC = AE - (AAW * ABU);
                        let ACD = ((AAX * ABU) + (ABV * AAW)) * X;
                        let ACE = (AE - ABU) - EB;
                        let ACF = (FS * ABS) + (CZ * ACE);
                        let ACG = (ABT * FS) + ((DB * ACE) + (((ABV * X) - EC) * CZ));
                        let ACH = ACA / CZ;
                        let ACI = (DM - ABQ) + (ACH.ln());
                        let ACJ = (DN - ABR) + (((ACB - (DB * ACH)) / CZ) * (R / ACH));
                        let ACK = ACA + ACF;
                        let ACL = ACB + ACG;
                        let ACM = if (ACI.abs()) < KS { 1.0 } else { 0.0 };
                        let ADO;
                        let ADP;
                        if ACM != 0.0 {
                            ADO = ABQ;
                            ADP = ABR;
                        } else {
                            let ACX = ACL * ACK;
                            let ACY = S * ACF;
                            let ACZ = ACA * ACC;
                            let ADA = (ACB * ACC) + (ACD * ACA);
                            let ADB = (ACY * ACF) - ACZ;
                            let ADC = (ACK * ACK) + (ADB * ACI);
                            let ADD = (ACX + ACX) + ((((((ACG * S) * ACF) + (ACG * ACY)) - ADA) * ACI) + (ACJ * ADB));
                            let ADE = ACA * ACK;
                            let ADF = ACK * ACI;
                            let ADG = (ADF * ACI) / ADC;
                            let ADH = ADG * ACF;
                            let ADI = ACG * ACF;
                            let ADJ = ((ACF * ACF) * DU) - ACZ;
                            let ADK = ADC + (ADH * ADJ);
                            let ADL = (ADE * ACI) / ADK;
                            let ADM = ABQ + ADL;
                            let ADN = ABR + ((((((ACB * ACK) + (ACL * ACA)) * ACI) + (ACJ * ADE)) - ((ADD + ((((((((((ACL * ACI) + (ACJ * ACK)) * ACI) + (ACJ * ADF)) - (ADD * ADG)) / ADC) * ACF) + (ACG * ADG)) * ADJ) + ((((ADI + ADI) * DU) - ADA) * ADH))) * ADL)) / ADK);
                            ADO = ADM;
                            ADP = ADN;
                        }
                        let ADQ = if ADO < GP { 1.0 } else { 0.0 };
                        let ADY;
                        let ADZ;
                        let AEA;
                        let AEB;
                        if ADQ != 0.0 {
                            let ADR = ADO.exp();
                            let ADS = ADP * ADR;
                            let ADT = AE / ADR;
                            let ADU = ((ADS * ADT) * X) / ADR;
                            let ADV = EB * ADR;
                            let ADW = (EC * ADR) + (ADS * EB);
                            ADY = ADT;
                            ADZ = ADV;
                            AEA = ADU;
                            AEB = ADW;
                        } else {
                            let ADX = if ADO > (DM - GP) { 1.0 } else { 0.0 };
                            let AFN;
                            let AFO;
                            let AFP;
                            let AFQ;
                            if ADX != 0.0 {
                                let AEU = (ADO - DM).exp();
                                let AEV = (ADP - DN) * AEU;
                                let AEW = EB / AEU;
                                let AEX = (EC - (AEV * AEW)) / AEU;
                                AFN = AEW;
                                AFO = AEU;
                                AFP = AEX;
                                AFQ = AEV;
                            } else {
                                let AEY = DN - ADP;
                                let AEZ = (DM - ADO) - GP;
                                let AFA = S * AEZ;
                                let AFB = AE + (AEZ * DU);
                                let AFC = AE + (AFA * AFB);
                                let AFD = AE + (AEZ * AFC);
                                let AFE = IT / AFD;
                                let AFF = ((((AEY * AFC) + ((((AEY * S) * AFB) + ((AEY * DU) * AFA)) * AEZ)) * AFE) * X) / AFD;
                                let AFG = ADO - GP;
                                let AFH = S * AFG;
                                let AFI = AE + (AFG * DU);
                                let AFJ = AE + (AFH * AFI);
                                let AFK = AE + (AFG * AFJ);
                                let AFL = IT / AFK;
                                let AFM = ((((ADP * AFJ) + ((((ADP * S) * AFI) + ((ADP * DU) * AFH)) * AFG)) * AFL) * X) / AFK;
                                AFN = AFL;
                                AFO = AFE;
                                AFP = AFM;
                                AFQ = AFF;
                            }
                            ADY = AFN;
                            ADZ = AFO;
                            AEA = AFP;
                            AEB = AFQ;
                        }
                        let AEC = OV - ADO;
                        let AED = OW - ADP;
                        let AEE = ((AE - ADY) + ADZ) - EB;
                        let AEF = (FS * AEC) + (CZ * AEE);
                        let AEG = (AED * FS) + ((DB * AEE) + ((((AEA * X) + AEB) - EC) * CZ));
                        let AEH = AED * AEC;
                        let AEI = ADO + AE;
                        let AEJ = (((ADY + ADO) - AE) + ADZ) - (EB * AEI);
                        let AEK = ADY + ADZ;
                        let AEL = FS - (CZ * AEK);
                        let AEM = AEG * AEF;
                        let AEN = FS * ((AEC * AEC) - (CZ * AEJ));
                        let AEO = ((AEH + AEH) - ((DB * AEJ) + ((((AEA + ADP) + AEB) - ((EC * AEI) + (ADP * EB))) * CZ))) * FS;
                        let AEP = ((AEF * AEF) - (AEN * AEL)).sqrt();
                        let AEQ = AEF + AEP;
                        let AER = AEN / AEQ;
                        let AES = ADO + AER;
                        let AET = ADP + ((AEO - ((AEG + (((AEM + AEM) - ((AEO * AEL) + ((((DB * AEK) + ((AEA + AEB) * CZ)) * X) * AEN))) * (R / (Q * AEP)))) * AER)) / AEQ);
                        ZD = AES;
                        ZE = AET;
                    }
                    XD = ZD;
                    XE = ZE;
                }
                OE = OV;
                OF = XD;
                OG = OW;
                OH = XE;
            } else {
                OE = EI;
                OF = EW;
                OG = EJ;
                OH = EX;
            }
            let OI = if (if OE <= V { 1.0 } else { 0.0 }) != 0.0 || staged[31] != 0.0 { 1.0 } else { 0.0 };
            let AFT;
            let AFU;
            if OI != 0.0 {
                AFT = V;
                AFU = AFR;
            } else {
                let AFS = if OF < GP { 1.0 } else { 0.0 };
                let AGO;
                let AGP;
                let AGQ;
                let AGR;
                if AFS != 0.0 {
                    let AGG = OF.exp();
                    let AGH = AE / AGG;
                    let AGI = (((OH * AGG) * AGH) * X) / AGG;
                    let AGJ = AE / AGH;
                    let AGK = (AGJ - OF) - AE;
                    let AGL = EB * AGK;
                    let AGM = (EC * AGK) + (((((AGI * AGJ) * X) / AGH) - OH) * EB);
                    AGO = AGH;
                    AGP = AGL;
                    AGQ = AGI;
                    AGR = AGM;
                } else {
                    let AGN = if OF > (DM - GP) { 1.0 } else { 0.0 };
                    let AHR;
                    let AHS;
                    let AHT;
                    let AHU;
                    if AGN != 0.0 {
                        let AGT = (OF - DM).exp();
                        let AGU = (OH - DN) * AGT;
                        let AGV = EB / AGT;
                        let AGW = (EC - (AGU * AGV)) / AGT;
                        let AGX = OF + AE;
                        let AGY = AGT - (EB * AGX);
                        let AGZ = AGU - ((EC * AGX) + (OH * EB));
                        AHR = AGV;
                        AHS = AGY;
                        AHT = AGW;
                        AHU = AGZ;
                    } else {
                        let AHA = DN - OH;
                        let AHB = (DM - OF) - GP;
                        let AHC = S * AHB;
                        let AHD = AE + (AHB * DU);
                        let AHE = AE + (AHC * AHD);
                        let AHF = AE + (AHB * AHE);
                        let AHG = IT / AHF;
                        let AHH = OF - GP;
                        let AHI = S * AHH;
                        let AHJ = AE + (AHH * DU);
                        let AHK = AE + (AHI * AHJ);
                        let AHL = AE + (AHH * AHK);
                        let AHM = IT / AHL;
                        let AHN = ((((OH * AHK) + ((((OH * S) * AHJ) + ((OH * DU) * AHI)) * AHH)) * AHM) * X) / AHL;
                        let AHO = OF + AE;
                        let AHP = AHG - (EB * AHO);
                        let AHQ = (((((AHA * AHE) + ((((AHA * S) * AHD) + ((AHA * DU) * AHC)) * AHB)) * AHG) * X) / AHF) - ((EC * AHO) + (OH * EB));
                        AHR = AHM;
                        AHS = AHP;
                        AHT = AHN;
                        AHU = AHQ;
                    }
                    AGO = AHR;
                    AGP = AHS;
                    AGQ = AHT;
                    AGR = AHU;
                }
                let AGS = if OF < DJ { 1.0 } else { 0.0 };
                let AIT;
                let AIU;
                let AIV;
                let AIW;
                let AIX;
                let AIY;
                if AGS != 0.0 {
                    let AHV = S * OF;
                    let AHW = AHV * OF;
                    let AHX = DU * OF;
                    let AHY = AE - (JA * OF);
                    let AHZ = AE - (AHX * AHY);
                    let AIA = (((OH * DU) * AHY) + (((OH * JA) * X) * AHX)) * X;
                    let AIB = AHW * AHZ;
                    let AIC = ((((OH * S) * OF) + (OH * AHV)) * AHZ) + (AIA * AHW);
                    let AID = EM * EB;
                    let AIE = AID * OF;
                    let AIF = AIE * OF;
                    let AIG = AIF * OF;
                    let AII = AE + (AIH * OF);
                    let AIJ = AIG * AII;
                    let AIK = ((((((((EC * EM) * OF) + (OH * AID)) * OF) + (OH * AIE)) * OF) + (OH * AIF)) * AII) + ((OH * AIH) * AIG);
                    let AIL = AHZ.sqrt();
                    let AIM = DE * OF;
                    let AIN = AIM * AIL;
                    let AIO = ((OH * DE) * AIL) + ((AIA * (R / (Q * AIL))) * AIM);
                    AIT = AIB;
                    AIU = AIJ;
                    AIV = AIN;
                    AIW = AIC;
                    AIX = AIK;
                    AIY = AIO;
                } else {
                    let AIP = (OF - AE) + AGO;
                    let AIQ = OH + AGQ;
                    let AIR = AIP.sqrt();
                    let AIS = AIQ * (R / (Q * AIR));
                    AIT = AIP;
                    AIU = AGP;
                    AIV = AIR;
                    AIW = AIQ;
                    AIX = AGR;
                    AIY = AIS;
                }
                let AIZ = (AIT + AIU).sqrt();
                let AJA = NY * CZ;
                let AJB = (CX * AIZ) + (CX * AIV);
                let AJC = (AJA * AIU) / AJB;
                let AJD = ((((DB * NY) * AIU) + (AIX * AJA)) - ((((CY * AIZ) + (((AIW + AIX) * (R / (Q * AIZ))) * CX)) + ((CY * AIV) + (AIY * CX))) * AJC)) / AJB;
                AFT = AJC;
                AFU = AJD;
            }
            let AFV = -AFT;
            let AFW = AFU * X;
            let AFY = EH + AFX;
            let AFZ = Lanes([K[0], K[1], 0.0]);
            let AGB = Lanes([0.0, 0.0, AGA]);
            let AGC = AFZ + AGB;
            let AGD = AFY * DL;
            let AGE = AGC * DL;
            let AGF = if (AGD.abs()) <= DK { 1.0 } else { 0.0 };
            let AJI;
            let AJJ;
            if AGF != 0.0 {
                let AJE = AGD / DG;
                let AJF = DF * AJE;
                let AJG = (AGE - Lanes([AJF[0], AJF[1], 0.0])) / DG;
                AJI = AJE;
                AJJ = AJG;
            } else {
                let AJH = if AGD > DK { 1.0 } else { 0.0 };
                let ALK;
                let ALL;
                if AJH != 0.0 {
                    let AJM = (DG * EF) / EG;
                    let AJN = (AJM - AE) / EG;
                    let AJO = AGD / DG;
                    let AJP = DF * AJO;
                    let AJQ = (((((DF * EF) - (EE * AJM)) / EG) - (EE * AJN)) / EG) * AGD;
                    let AJR = AE + (AJN * AGD);
                    let AJS = AJO * AJR;
                    let AJT = (((AGE - Lanes([AJP[0], AJP[1], 0.0])) / DG) * AJR) + ((Lanes([AJQ[0], AJQ[1], 0.0]) + (AGE * AJN)) * AJO);
                    let AJU = if AJS < DO { 1.0 } else { 0.0 };
                    let ALV;
                    let ALW;
                    if AJU != 0.0 {
                        let ALM = (-AJS).exp();
                        let ALN = (AJT * X) * ALM;
                        ALV = ALM;
                        ALW = ALN;
                    } else {
                        let ALO = AJS - DO;
                        let ALP = S * ALO;
                        let ALQ = AE + (ALO * DU);
                        let ALR = AE + (ALP * ALQ);
                        let ALS = AE + (ALO * ALR);
                        let ALT = DY / ALS;
                        let ALU = ((((AJT * ALR) + ((((AJT * S) * ALQ) + ((AJT * DU) * ALP)) * ALO)) * ALT) * X) / ALS;
                        ALV = ALT;
                        ALW = ALU;
                    }
                    let ALX = S * CZ;
                    let ALY = DB * S;
                    let ALZ = DB * JA;
                    let AMA = ((AGD + (JA * CZ)) - (AE - ALV)).sqrt();
                    let AMB = CY * AMA;
                    let AMC = (AGD + ALX) - (CX * AMA);
                    let AMD = (AGE + Lanes([ALY[0], ALY[1], 0.0])) - (Lanes([AMB[0], AMB[1], 0.0]) + ((((AGE + Lanes([ALZ[0], ALZ[1], 0.0])) - (ALW * X)) * (R / (Q * AMA))) * CX));
                    let AME = if AMC < DO { 1.0 } else { 0.0 };
                    let AMO;
                    let AMP;
                    if AME != 0.0 {
                        let AMF = (-AMC).exp();
                        let AMG = (AMD * X) * AMF;
                        AMO = AMF;
                        AMP = AMG;
                    } else {
                        let AMH = AMC - DO;
                        let AMI = S * AMH;
                        let AMJ = AE + (AMH * DU);
                        let AMK = AE + (AMI * AMJ);
                        let AML = AE + (AMH * AMK);
                        let AMM = DY / AML;
                        let AMN = ((((AMD * AMK) + ((((AMD * S) * AMJ) + ((AMD * DU) * AMI)) * AMH)) * AMM) * X) / AML;
                        AMO = AMM;
                        AMP = AMN;
                    }
                    let AMQ = ALY * AMO;
                    let AMR = AGD - AMC;
                    let AMS = AGE - AMD;
                    let AMT = AE - AMO;
                    let AMU = DB * AMT;
                    let AMV = (FS * AMR) + (CZ * AMT);
                    let AMW = (AMS * FS) + (Lanes([AMU[0], AMU[1], 0.0]) + ((AMP * X) * CZ));
                    let AMX = AMS * AMR;
                    let AMY = (AMC - AE) + AMO;
                    let AMZ = DB * AMY;
                    let ANA = (AMR * AMR) - (CZ * AMY);
                    let ANB = (AMX + AMX) - (Lanes([AMZ[0], AMZ[1], 0.0]) + ((AMD + AMP) * CZ));
                    let ANC = AMW * AMV;
                    let ANE = AND * (AE - (ALX * AMO));
                    let ANF = ((AMV * AMV) - (ANE * ANA)).sqrt();
                    let ANG = AMV + ANF;
                    let ANH = (FS * ANA) / ANG;
                    let ANI = AMC + ANH;
                    let ANJ = AMD + (((ANB * FS) - ((AMW + (((ANC + ANC) - (((((Lanes([AMQ[0], AMQ[1], 0.0]) + (AMP * ALX)) * X) * AND) * ANA) + (ANB * ANE))) * (R / (Q * ANF)))) * ANH)) / ANG);
                    ALK = ANI;
                    ALL = ANJ;
                } else {
                    let AJV = -AGD;
                    let AJW = AGE * X;
                    let AJX = (EF * AJV) / DG;
                    let AJY = DF * AJX;
                    let AJZ = ((AJW * EF) - Lanes([AJY[0], AJY[1], 0.0])) / DG;
                    let AKA = AJX - FF;
                    let AKB = AJZ * AKA;
                    let AKC = ((AKA * AKA) + FI).sqrt();
                    let AKD = S * ((AJX + FE) - AKC);
                    let AKE = (AJZ - ((AKB + AKB) * (R / (Q * AKC)))) * S;
                    let AKF = AJV - AKD;
                    let AKG = AJW - AKE;
                    let AKH = AKG * AKF;
                    let AKI = AKD + AE;
                    let AKJ = DB * AKI;
                    let AKK = (AKF * AKF) + (CZ * AKI);
                    let AKL = (AKH + AKH) + (Lanes([AKJ[0], AKJ[1], 0.0]) + (AKE * CZ));
                    let AKM = (FS * AKF) - CZ;
                    let AKN = (AKG * FS) - Lanes([DB[0], DB[1], 0.0]);
                    let AKO = AKK / CZ;
                    let AKP = DB * AKO;
                    let AKQ = (AKO.ln()) - AKD;
                    let AKR = (((AKL - Lanes([AKP[0], AKP[1], 0.0])) / CZ) * (R / AKO)) - AKE;
                    let AKS = AKK + AKM;
                    let AKT = AKL + AKN;
                    let AKU = AKT * AKS;
                    let AKV = S * AKM;
                    let AKW = (AKV * AKM) - AKK;
                    let AKX = (AKS * AKS) + (AKW * AKQ);
                    let AKY = (AKU + AKU) + ((((((AKN * S) * AKM) + (AKN * AKV)) - AKL) * AKQ) + (AKR * AKW));
                    let AKZ = AKK * AKS;
                    let ALA = AKS * AKQ;
                    let ALB = (ALA * AKQ) / AKX;
                    let ALC = ALB * AKM;
                    let ALD = AKN * AKM;
                    let ALE = ((AKM * AKM) * DU) - AKK;
                    let ALF = AKX + (ALC * ALE);
                    let ALG = (AKZ * AKQ) / ALF;
                    let ALH = AKD + ALG;
                    let ALI = AKE + ((((((AKL * AKS) + (AKT * AKK)) * AKQ) + (AKR * AKZ)) - ((AKY + ((((((((((AKT * AKQ) + (AKR * AKS)) * AKQ) + (AKR * ALA)) - (AKY * ALB)) / AKX) * AKM) + (AKN * ALB)) * ALE) + ((((ALD + ALD) * DU) - AKL) * ALC))) * ALG)) / ALF);
                    let ALJ = if (ALH.abs()) < GP { 1.0 } else { 0.0 };
                    let ANN;
                    let ANO;
                    if ALJ != 0.0 {
                        let ANK = ALH.exp();
                        let ANL = ALI * ANK;
                        ANN = ANK;
                        ANO = ANL;
                    } else {
                        let ANM = if ALH < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AOX;
                        let AOY;
                        if ANM != 0.0 {
                            let AOJ = -2.3025850929940458e2f64 - ALH;
                            let AOK = ALI * X;
                            let AOL = S * (-2.3025850929940458e2f64 - ALH);
                            let AOM = AE + ((-2.3025850929940458e2f64 - ALH) * DU);
                            let AON = AE + (AOL * AOM);
                            let AOO = AE + (AOJ * AON);
                            let AOP = IT / AOO;
                            let AOQ = ((((AOK * AON) + ((((AOK * S) * AOM) + ((AOK * DU) * AOL)) * AOJ)) * AOP) * X) / AOO;
                            AOX = AOP;
                            AOY = AOQ;
                        } else {
                            let AOR = ALH - GP;
                            let AOS = S * AOR;
                            let AOT = AE + (AOR * DU);
                            let AOU = AE + (AOS * AOT);
                            let AOV = HL * (AE + (AOR * AOU));
                            let AOW = ((ALI * AOU) + ((((ALI * S) * AOT) + ((ALI * DU) * AOS)) * AOR)) * HL;
                            AOX = AOV;
                            AOY = AOW;
                        }
                        ANN = AOX;
                        ANO = AOY;
                    }
                    let ANP = S * CZ;
                    let ANQ = (DB * S) * ANN;
                    let ANR = AJV - ALH;
                    let ANS = AJW - ALI;
                    let ANT = ANN - AE;
                    let ANU = DB * ANT;
                    let ANV = (FS * ANR) + (CZ * ANT);
                    let ANW = (ANS * FS) + (Lanes([ANU[0], ANU[1], 0.0]) + (ANO * CZ));
                    let ANX = ANS * ANR;
                    let ANY = (ALH + AE) - ANN;
                    let ANZ = DB * ANY;
                    let AOA = (ANR * ANR) + (CZ * ANY);
                    let AOB = (ANX + ANX) + (Lanes([ANZ[0], ANZ[1], 0.0]) + ((ALI - ANO) * CZ));
                    let AOC = ANW * ANV;
                    let AOD = AND * (AE - (ANP * ANN));
                    let AOE = ((ANV * ANV) - (AOD * AOA)).sqrt();
                    let AOF = ANV + AOE;
                    let AOG = (FS * AOA) / AOF;
                    let AOH = -(ALH + AOG);
                    let AOI = (ALI + (((AOB * FS) - ((ANW + (((AOC + AOC) - (((((Lanes([ANQ[0], ANQ[1], 0.0]) + (ANO * ANP)) * X) * AND) * AOA) + (AOB * AOD))) * (R / (Q * AOE)))) * AOG)) / AOF)) * X;
                    ALK = AOH;
                    ALL = AOI;
                }
                AJI = ALK;
                AJJ = ALL;
            }
            let AJK = AJI * NY;
            let AJL = AJJ * NY;
            let APE;
            let APF;
            let APG;
            let APH;
            let API;
            let APJ;
            if EY != 0.0 {
                let APA = (AOZ * (EH - AJK)) * DL;
                let APB = ((AFZ - AJL) * AOZ) * DL;
                let APC = if (APA.abs()) <= OC { 1.0 } else { 0.0 };
                let APS;
                let APT;
                if APC != 0.0 {
                    let APL = ((OJ * OJ) * EM) * DE;
                    let APM = APA * OJ;
                    let APN = AE - OM;
                    let APO = AE + (((APA * APN) * OO) * APL);
                    let APP = APM * APO;
                    let APQ = ((APB * OJ) * APO) + ((((APB * APN) * OO) * APL) * APM);
                    APS = APP;
                    APT = APQ;
                } else {
                    let APR = if APA < (-OC) { 1.0 } else { 0.0 };
                    let ARR;
                    let ARS;
                    if APR != 0.0 {
                        let APZ = -APA;
                        let AQA = APB * X;
                        let AQB = (EF * APZ) * OJ;
                        let AQC = (AQA * EF) * OJ;
                        let AQD = AQB - FF;
                        let AQE = AQC * AQD;
                        let AQF = ((AQD * AQD) + FI).sqrt();
                        let AQG = S * ((AQB + FE) - AQF);
                        let AQH = (AQC - ((AQE + AQE) * (R / (Q * AQF)))) * S;
                        let AQI = APZ - AQG;
                        let AQJ = AQA - AQH;
                        let AQK = AQJ * AQI;
                        let AQL = (AQI * AQI) + (PK * (AQG + AE));
                        let AQM = (AQK + AQK) + (AQH * PK);
                        let AQN = AQJ * FS;
                        let AQO = (FS * AQI) - PK;
                        let AQP = AQL * PP;
                        let AQQ = (-AQG) + (AQP.ln());
                        let AQR = (AQH * X) + ((AQM * PP) * (R / AQP));
                        let AQS = AQL + AQO;
                        let AQT = AQM + AQN;
                        let AQU = AQT * AQS;
                        let AQV = S * AQO;
                        let AQW = (AQV * AQO) - AQL;
                        let AQX = (AQS * AQS) + (AQW * AQQ);
                        let AQY = (AQU + AQU) + ((((((AQN * S) * AQO) + (AQN * AQV)) - AQM) * AQQ) + (AQR * AQW));
                        let AQZ = AQL * AQS;
                        let ARA = AQS * AQQ;
                        let ARB = (ARA * AQQ) / AQX;
                        let ARC = ARB * AQO;
                        let ARD = AQN * AQO;
                        let ARE = ((AQO * AQO) * DU) - AQL;
                        let ARF = AQX + (ARC * ARE);
                        let ARG = (AQZ * AQQ) / ARF;
                        let ARH = AQG + ARG;
                        let ARI = AQH + ((((((AQM * AQS) + (AQT * AQL)) * AQQ) + (AQR * AQZ)) - ((AQY + ((((((((((AQT * AQQ) + (AQR * AQS)) * AQQ) + (AQR * ARA)) - (AQY * ARB)) / AQX) * AQO) + (AQN * ARB)) * ARE) + ((((ARD + ARD) * DU) - AQM) * ARC))) * ARG)) / ARF);
                        let ARJ = if ARH < GP { 1.0 } else { 0.0 };
                        let ASB;
                        let ASC;
                        if ARJ != 0.0 {
                            let ART = ARH.exp();
                            let ARU = ARI * ART;
                            ASB = ART;
                            ASC = ARU;
                        } else {
                            let ARV = ARH - GP;
                            let ARW = S * ARV;
                            let ARX = AE + (ARV * DU);
                            let ARY = AE + (ARW * ARX);
                            let ARZ = HL * (AE + (ARV * ARY));
                            let ASA = ((ARI * ARY) + ((((ARI * S) * ARX) + ((ARI * DU) * ARW)) * ARV)) * HL;
                            ASB = ARZ;
                            ASC = ASA;
                        }
                        let ASD = AE / ASB;
                        let ASE = APZ - ARH;
                        let ASF = AQA - ARI;
                        let ASG = OM * ASD;
                        let ASH = (((ASC * ASD) * X) / ASB) * OM;
                        let ASI = (FS * ASE) + (PK * (((ASB - AE) - ASG) + OM));
                        let ASJ = (ASF * FS) + ((ASC - ASH) * PK);
                        let ASK = ASF * ASE;
                        let ASL = FS - (PK * (ASB + ASG));
                        let ASM = ASJ * ASI;
                        let ASN = FS * ((ASE * ASE) - (PK * ((((ASB - ARH) - AE) + ASG) + (OM * (ARH - AE)))));
                        let ASO = ((ASK + ASK) - ((((ASC - ARI) + ASH) + (ARI * OM)) * PK)) * FS;
                        let ASP = ((ASI * ASI) - (ASN * ASL)).sqrt();
                        let ASQ = ASI + ASP;
                        let ASR = ASN / ASQ;
                        let ASS = (-ARH) - ASR;
                        let AST = (ARI * X) - ((ASO - ((ASJ + (((ASM + ASM) - ((ASO * ASL) + ((((ASC + ASH) * PK) * X) * ASN))) * (R / (Q * ASP)))) * ASR)) / ASQ);
                        ARR = ASS;
                        ARS = AST;
                    } else {
                        let ARK = AE / (EF + (OO * GR));
                        let ARL = (((QM * EF) * ARK) - AE) * ARK;
                        let ARM = APA * OJ;
                        let ARN = AE + (ARL * APA);
                        let ARO = -(ARM * ARN);
                        let ARP = (((APB * OJ) * ARN) + ((APB * ARL) * ARM)) * X;
                        let ARQ = if ARO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ATE;
                        let ATF;
                        if ARQ != 0.0 {
                            let ASU = ARO.exp();
                            let ASV = ARP * ASU;
                            ATE = ASU;
                            ATF = ASV;
                        } else {
                            let ASW = -2.3025850929940458e2f64 - ARO;
                            let ASX = ARP * X;
                            let ASY = S * (-2.3025850929940458e2f64 - ARO);
                            let ASZ = AE + ((-2.3025850929940458e2f64 - ARO) * DU);
                            let ATA = AE + (ASY * ASZ);
                            let ATB = AE + (ASW * ATA);
                            let ATC = IT / ATB;
                            let ATD = ((((ASX * ATA) + ((((ASX * S) * ASZ) + ((ASX * DU) * ASY)) * ASW)) * ATC) * X) / ATB;
                            ATE = ATC;
                            ATF = ATD;
                        }
                        let ATG = PK * S;
                        let ATH = ((APA + (PK * JA)) - (AE - ATE)).sqrt();
                        let ATI = (APA + ATG) - (OO * ATH);
                        let ATJ = APB - (((APB - (ATF * X)) * (R / (Q * ATH))) * OO);
                        let ATK = SM + JE;
                        let ATL = ATK - ATI;
                        let ATM = ATJ * X;
                        let ATN = if ATL > L { 1.0 } else { 0.0 };
                        let ATU;
                        let ATV;
                        if ATN != 0.0 {
                            let ATO = ATM * ATL;
                            let ATP = ((ATL * ATL) + JK).sqrt();
                            let ATQ = ATK - (S * (ATL + ATP));
                            let ATR = ((ATM + ((ATO + ATO) * (R / (Q * ATP)))) * S) * X;
                            ATU = ATQ;
                            ATV = ATR;
                        } else {
                            let ATS = ATI - ATK;
                            let ATT = if ATS > L { 1.0 } else { 0.0 };
                            let AUY;
                            let AUZ;
                            if ATT != 0.0 {
                                let AUQ = ATJ * ATS;
                                let AUR = ((ATS * ATS) + JK).sqrt();
                                let AUS = ATS + AUR;
                                let AUT = 2.5e0f64 / AUS;
                                let AUU = ATK - AUT;
                                let AUV = ((((ATJ + ((AUQ + AUQ) * (R / (Q * AUR)))) * AUT) * X) / AUS) * X;
                                AUY = AUU;
                                AUZ = AUV;
                            } else {
                                let AUW = ATK - (S * (ATL + 2.23606797749979e0f64));
                                let AUX = (ATM * S) * X;
                                AUY = AUW;
                                AUZ = AUX;
                            }
                            ATU = AUY;
                            ATV = AUZ;
                        }
                        let ATW = ATU - (S * (ATK - (((ATK * ATK) + JK).sqrt())));
                        let ATX = APA - ATW;
                        let ATY = APB - ATV;
                        let ATZ = ATV * X;
                        let AUA = (-ATW).exp();
                        let AUB = ATZ * AUA;
                        let AUC = ATY * ATX;
                        let AUD = (ATX * ATX) - (PK * (((AUA + ATW) - AE) - (OM * (ATW + AE))));
                        let AUE = if KF >= AUD { KF } else { AUD };
                        let AUF = ((AUC + AUC) - (((AUB + ATV) - (ATV * OM)) * PK)) * (R - (if KF >= AUD { 1.0 } else { 0.0 }));
                        let AUG = AE - (ATG * AUA);
                        let AUH = (AUB * ATG) * X;
                        let AUI = (FS * ATX) + (PK * ((AE - AUA) - OM));
                        let AUJ = (ATY * FS) + ((AUB * X) * PK);
                        let AUK = AUE / PK;
                        let AUL = (SM - ATW) + (AUK.ln());
                        let AUM = ATZ + ((AUF / PK) * (R / AUK));
                        let AUN = AUE + AUI;
                        let AUO = AUF + AUJ;
                        let AUP = if (AUL.abs()) < KS { 1.0 } else { 0.0 };
                        let AVR;
                        let AVS;
                        if AUP != 0.0 {
                            AVR = ATW;
                            AVS = ATV;
                        } else {
                            let AVA = AUO * AUN;
                            let AVB = S * AUI;
                            let AVC = AUE * AUG;
                            let AVD = (AUF * AUG) + (AUH * AUE);
                            let AVE = (AVB * AUI) - AVC;
                            let AVF = (AUN * AUN) + (AVE * AUL);
                            let AVG = (AVA + AVA) + ((((((AUJ * S) * AUI) + (AUJ * AVB)) - AVD) * AUL) + (AUM * AVE));
                            let AVH = AUE * AUN;
                            let AVI = AUN * AUL;
                            let AVJ = (AVI * AUL) / AVF;
                            let AVK = AVJ * AUI;
                            let AVL = AUJ * AUI;
                            let AVM = ((AUI * AUI) * DU) - AVC;
                            let AVN = AVF + (AVK * AVM);
                            let AVO = (AVH * AUL) / AVN;
                            let AVP = ATW + AVO;
                            let AVQ = ATV + ((((((AUF * AUN) + (AUO * AUE)) * AUL) + (AUM * AVH)) - ((AVG + ((((((((((AUO * AUL) + (AUM * AUN)) * AUL) + (AUM * AVI)) - (AVG * AVJ)) / AVF) * AUI) + (AUJ * AVJ)) * AVM) + ((((AVL + AVL) * DU) - AVD) * AVK))) * AVO)) / AVN);
                            AVR = AVP;
                            AVS = AVQ;
                        }
                        let AVT = if AVR < GP { 1.0 } else { 0.0 };
                        let AWB;
                        let AWC;
                        let AWD;
                        let AWE;
                        if AVT != 0.0 {
                            let AVU = AVR.exp();
                            let AVV = AVS * AVU;
                            let AVW = AE / AVU;
                            let AVX = ((AVV * AVW) * X) / AVU;
                            let AVY = OM * AVU;
                            let AVZ = AVV * OM;
                            AWB = AVW;
                            AWC = AVY;
                            AWD = AVX;
                            AWE = AVZ;
                        } else {
                            let AWA = if AVR > (SM - GP) { 1.0 } else { 0.0 };
                            let AXM;
                            let AXN;
                            let AXO;
                            let AXP;
                            if AWA != 0.0 {
                                let AWT = (AVR - SM).exp();
                                let AWU = AVS * AWT;
                                let AWV = OM / AWT;
                                let AWW = ((AWU * AWV) * X) / AWT;
                                AXM = AWV;
                                AXN = AWT;
                                AXO = AWW;
                                AXP = AWU;
                            } else {
                                let AWX = AVS * X;
                                let AWY = (SM - AVR) - GP;
                                let AWZ = S * AWY;
                                let AXA = AE + (AWY * DU);
                                let AXB = AE + (AWZ * AXA);
                                let AXC = AE + (AWY * AXB);
                                let AXD = IT / AXC;
                                let AXE = ((((AWX * AXB) + ((((AWX * S) * AXA) + ((AWX * DU) * AWZ)) * AWY)) * AXD) * X) / AXC;
                                let AXF = AVR - GP;
                                let AXG = S * AXF;
                                let AXH = AE + (AXF * DU);
                                let AXI = AE + (AXG * AXH);
                                let AXJ = AE + (AXF * AXI);
                                let AXK = IT / AXJ;
                                let AXL = ((((AVS * AXI) + ((((AVS * S) * AXH) + ((AVS * DU) * AXG)) * AXF)) * AXK) * X) / AXJ;
                                AXM = AXK;
                                AXN = AXD;
                                AXO = AXL;
                                AXP = AXE;
                            }
                            AWB = AXM;
                            AWC = AXN;
                            AWD = AXO;
                            AWE = AXP;
                        }
                        let AWF = APA - AVR;
                        let AWG = APB - AVS;
                        let AWH = (FS * AWF) + (PK * (((AE - AWB) + AWC) - OM));
                        let AWI = (AWG * FS) + (((AWD * X) + AWE) * PK);
                        let AWJ = AWG * AWF;
                        let AWK = FS - (PK * (AWB + AWC));
                        let AWL = AWI * AWH;
                        let AWM = FS * ((AWF * AWF) - (PK * ((((AWB + AVR) - AE) + AWC) - (OM * (AVR + AE)))));
                        let AWN = ((AWJ + AWJ) - ((((AWD + AVS) + AWE) - (AVS * OM)) * PK)) * FS;
                        let AWO = ((AWH * AWH) - (AWM * AWK)).sqrt();
                        let AWP = AWH + AWO;
                        let AWQ = AWM / AWP;
                        let AWR = AVR + AWQ;
                        let AWS = AVS + ((AWN - ((AWI + (((AWL + AWL) - ((AWN * AWK) + ((((AWD + AWE) * PK) * X) * AWM))) * (R / (Q * AWO)))) * AWQ)) / AWP);
                        ARR = AWR;
                        ARS = AWS;
                    }
                    APS = ARR;
                    APT = ARS;
                }
                let APU = (AOZ * APS) * NY;
                let APV = (APT * AOZ) * NY;
                let APW = (AFY - APU) / NY;
                let APX = (AGC - APV) / NY;
                let APY = if (APW.abs()) <= DK { 1.0 } else { 0.0 };
                let AXU;
                let AXV;
                if APY != 0.0 {
                    let AXQ = APW / DG;
                    let AXR = DF * AXQ;
                    let AXS = (APX - Lanes([AXR[0], AXR[1], 0.0])) / DG;
                    AXU = AXQ;
                    AXV = AXS;
                } else {
                    let AXT = if APW > DK { 1.0 } else { 0.0 };
                    let AZW;
                    let AZX;
                    if AXT != 0.0 {
                        let AXY = (DG * EF) / EG;
                        let AXZ = (AXY - AE) / EG;
                        let AYA = APW / DG;
                        let AYB = DF * AYA;
                        let AYC = (((((DF * EF) - (EE * AXY)) / EG) - (EE * AXZ)) / EG) * APW;
                        let AYD = AE + (AXZ * APW);
                        let AYE = AYA * AYD;
                        let AYF = (((APX - Lanes([AYB[0], AYB[1], 0.0])) / DG) * AYD) + ((Lanes([AYC[0], AYC[1], 0.0]) + (APX * AXZ)) * AYA);
                        let AYG = if AYE < DO { 1.0 } else { 0.0 };
                        let BAH;
                        let BAI;
                        if AYG != 0.0 {
                            let AZY = (-AYE).exp();
                            let AZZ = (AYF * X) * AZY;
                            BAH = AZY;
                            BAI = AZZ;
                        } else {
                            let BAA = AYE - DO;
                            let BAB = S * BAA;
                            let BAC = AE + (BAA * DU);
                            let BAD = AE + (BAB * BAC);
                            let BAE = AE + (BAA * BAD);
                            let BAF = DY / BAE;
                            let BAG = ((((AYF * BAD) + ((((AYF * S) * BAC) + ((AYF * DU) * BAB)) * BAA)) * BAF) * X) / BAE;
                            BAH = BAF;
                            BAI = BAG;
                        }
                        let BAJ = S * CZ;
                        let BAK = DB * S;
                        let BAL = DB * JA;
                        let BAM = ((APW + (JA * CZ)) - (AE - BAH)).sqrt();
                        let BAN = CY * BAM;
                        let BAO = (APW + BAJ) - (CX * BAM);
                        let BAP = (APX + Lanes([BAK[0], BAK[1], 0.0])) - (Lanes([BAN[0], BAN[1], 0.0]) + ((((APX + Lanes([BAL[0], BAL[1], 0.0])) - (BAI * X)) * (R / (Q * BAM))) * CX));
                        let BAQ = if BAO < DO { 1.0 } else { 0.0 };
                        let BBA;
                        let BBB;
                        if BAQ != 0.0 {
                            let BAR = (-BAO).exp();
                            let BAS = (BAP * X) * BAR;
                            BBA = BAR;
                            BBB = BAS;
                        } else {
                            let BAT = BAO - DO;
                            let BAU = S * BAT;
                            let BAV = AE + (BAT * DU);
                            let BAW = AE + (BAU * BAV);
                            let BAX = AE + (BAT * BAW);
                            let BAY = DY / BAX;
                            let BAZ = ((((BAP * BAW) + ((((BAP * S) * BAV) + ((BAP * DU) * BAU)) * BAT)) * BAY) * X) / BAX;
                            BBA = BAY;
                            BBB = BAZ;
                        }
                        let BBC = BAK * BBA;
                        let BBD = APW - BAO;
                        let BBE = APX - BAP;
                        let BBF = AE - BBA;
                        let BBG = DB * BBF;
                        let BBH = (FS * BBD) + (CZ * BBF);
                        let BBI = (BBE * FS) + (Lanes([BBG[0], BBG[1], 0.0]) + ((BBB * X) * CZ));
                        let BBJ = BBE * BBD;
                        let BBK = (BAO - AE) + BBA;
                        let BBL = DB * BBK;
                        let BBM = (BBD * BBD) - (CZ * BBK);
                        let BBN = (BBJ + BBJ) - (Lanes([BBL[0], BBL[1], 0.0]) + ((BAP + BBB) * CZ));
                        let BBO = BBI * BBH;
                        let BBP = AND * (AE - (BAJ * BBA));
                        let BBQ = ((BBH * BBH) - (BBP * BBM)).sqrt();
                        let BBR = BBH + BBQ;
                        let BBS = (FS * BBM) / BBR;
                        let BBT = BAO + BBS;
                        let BBU = BAP + (((BBN * FS) - ((BBI + (((BBO + BBO) - (((((Lanes([BBC[0], BBC[1], 0.0]) + (BBB * BAJ)) * X) * AND) * BBM) + (BBN * BBP))) * (R / (Q * BBQ)))) * BBS)) / BBR);
                        AZW = BBT;
                        AZX = BBU;
                    } else {
                        let AYH = -APW;
                        let AYI = APX * X;
                        let AYJ = (EF * AYH) / DG;
                        let AYK = DF * AYJ;
                        let AYL = ((AYI * EF) - Lanes([AYK[0], AYK[1], 0.0])) / DG;
                        let AYM = AYJ - FF;
                        let AYN = AYL * AYM;
                        let AYO = ((AYM * AYM) + FI).sqrt();
                        let AYP = S * ((AYJ + FE) - AYO);
                        let AYQ = (AYL - ((AYN + AYN) * (R / (Q * AYO)))) * S;
                        let AYR = AYH - AYP;
                        let AYS = AYI - AYQ;
                        let AYT = AYS * AYR;
                        let AYU = AYP + AE;
                        let AYV = DB * AYU;
                        let AYW = (AYR * AYR) + (CZ * AYU);
                        let AYX = (AYT + AYT) + (Lanes([AYV[0], AYV[1], 0.0]) + (AYQ * CZ));
                        let AYY = (FS * AYR) - CZ;
                        let AYZ = (AYS * FS) - Lanes([DB[0], DB[1], 0.0]);
                        let AZA = AYW / CZ;
                        let AZB = DB * AZA;
                        let AZC = (AZA.ln()) - AYP;
                        let AZD = (((AYX - Lanes([AZB[0], AZB[1], 0.0])) / CZ) * (R / AZA)) - AYQ;
                        let AZE = AYW + AYY;
                        let AZF = AYX + AYZ;
                        let AZG = AZF * AZE;
                        let AZH = S * AYY;
                        let AZI = (AZH * AYY) - AYW;
                        let AZJ = (AZE * AZE) + (AZI * AZC);
                        let AZK = (AZG + AZG) + ((((((AYZ * S) * AYY) + (AYZ * AZH)) - AYX) * AZC) + (AZD * AZI));
                        let AZL = AYW * AZE;
                        let AZM = AZE * AZC;
                        let AZN = (AZM * AZC) / AZJ;
                        let AZO = AZN * AYY;
                        let AZP = AYZ * AYY;
                        let AZQ = ((AYY * AYY) * DU) - AYW;
                        let AZR = AZJ + (AZO * AZQ);
                        let AZS = (AZL * AZC) / AZR;
                        let AZT = AYP + AZS;
                        let AZU = AYQ + ((((((AYX * AZE) + (AZF * AYW)) * AZC) + (AZD * AZL)) - ((AZK + ((((((((((AZF * AZC) + (AZD * AZE)) * AZC) + (AZD * AZM)) - (AZK * AZN)) / AZJ) * AYY) + (AYZ * AZN)) * AZQ) + ((((AZP + AZP) * DU) - AYX) * AZO))) * AZS)) / AZR);
                        let AZV = if (AZT.abs()) < GP { 1.0 } else { 0.0 };
                        let BBY;
                        let BBZ;
                        if AZV != 0.0 {
                            let BBV = AZT.exp();
                            let BBW = AZU * BBV;
                            BBY = BBV;
                            BBZ = BBW;
                        } else {
                            let BBX = if AZT < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BDI;
                            let BDJ;
                            if BBX != 0.0 {
                                let BCU = -2.3025850929940458e2f64 - AZT;
                                let BCV = AZU * X;
                                let BCW = S * (-2.3025850929940458e2f64 - AZT);
                                let BCX = AE + ((-2.3025850929940458e2f64 - AZT) * DU);
                                let BCY = AE + (BCW * BCX);
                                let BCZ = AE + (BCU * BCY);
                                let BDA = IT / BCZ;
                                let BDB = ((((BCV * BCY) + ((((BCV * S) * BCX) + ((BCV * DU) * BCW)) * BCU)) * BDA) * X) / BCZ;
                                BDI = BDA;
                                BDJ = BDB;
                            } else {
                                let BDC = AZT - GP;
                                let BDD = S * BDC;
                                let BDE = AE + (BDC * DU);
                                let BDF = AE + (BDD * BDE);
                                let BDG = HL * (AE + (BDC * BDF));
                                let BDH = ((AZU * BDF) + ((((AZU * S) * BDE) + ((AZU * DU) * BDD)) * BDC)) * HL;
                                BDI = BDG;
                                BDJ = BDH;
                            }
                            BBY = BDI;
                            BBZ = BDJ;
                        }
                        let BCA = S * CZ;
                        let BCB = (DB * S) * BBY;
                        let BCC = AYH - AZT;
                        let BCD = AYI - AZU;
                        let BCE = BBY - AE;
                        let BCF = DB * BCE;
                        let BCG = (FS * BCC) + (CZ * BCE);
                        let BCH = (BCD * FS) + (Lanes([BCF[0], BCF[1], 0.0]) + (BBZ * CZ));
                        let BCI = BCD * BCC;
                        let BCJ = (AZT + AE) - BBY;
                        let BCK = DB * BCJ;
                        let BCL = (BCC * BCC) + (CZ * BCJ);
                        let BCM = (BCI + BCI) + (Lanes([BCK[0], BCK[1], 0.0]) + ((AZU - BBZ) * CZ));
                        let BCN = BCH * BCG;
                        let BCO = AND * (AE - (BCA * BBY));
                        let BCP = ((BCG * BCG) - (BCO * BCL)).sqrt();
                        let BCQ = BCG + BCP;
                        let BCR = (FS * BCL) / BCQ;
                        let BCS = -(AZT + BCR);
                        let BCT = (AZU + (((BCM * FS) - ((BCH + (((BCN + BCN) - (((((Lanes([BCB[0], BCB[1], 0.0]) + (BBZ * BCA)) * X) * AND) * BCL) + (BCM * BCO))) * (R / (Q * BCP)))) * BCR)) / BCQ)) * X;
                        AZW = BCS;
                        AZX = BCT;
                    }
                    AXU = AZW;
                    AXV = AZX;
                }
                let AXW = AXU * NY;
                let AXX = AXV * NY;
                APE = AXU;
                APF = AXW;
                APG = APU;
                APH = AXV;
                API = AXX;
                APJ = APV;
            } else {
                APE = AJI;
                APF = AJK;
                APG = V;
                APH = AJJ;
                API = AJL;
                APJ = APD;
            }
            let APK = if APE < GP { 1.0 } else { 0.0 };
            let BDO;
            let BDP;
            if APK != 0.0 {
                let BDK = APE.exp();
                let BDL = AE / BDK;
                let BDM = (((APH * BDK) * BDL) * X) / BDK;
                BDO = BDL;
                BDP = BDM;
            } else {
                let BDN = if APE > (DM - GP) { 1.0 } else { 0.0 };
                let BEC;
                let BED;
                if BDN != 0.0 {
                    let BDR = (DM - APE).exp();
                    let BDS = EB * BDR;
                    let BDT = EC * BDR;
                    let BDU = Lanes([BDT[0], BDT[1], 0.0]) + (((Lanes([DN[0], DN[1], 0.0]) - APH) * BDR) * EB);
                    BEC = BDS;
                    BED = BDU;
                } else {
                    let BDV = APE - GP;
                    let BDW = S * BDV;
                    let BDX = AE + (BDV * DU);
                    let BDY = AE + (BDW * BDX);
                    let BDZ = AE + (BDV * BDY);
                    let BEA = IT / BDZ;
                    let BEB = ((((APH * BDY) + ((((APH * S) * BDX) + ((APH * DU) * BDW)) * BDV)) * BEA) * X) / BDZ;
                    BEC = BEA;
                    BED = BEB;
                }
                BDO = BEC;
                BDP = BED;
            }
            let BDQ = if APE < (-DK) { 1.0 } else { 0.0 };
            let BEI;
            let BEJ;
            if BDQ != 0.0 {
                let BEE = ((BDO + APE) - AE).sqrt();
                let BEF = -BEE;
                let BEG = ((BDP + APH) * (R / (Q * BEE))) * X;
                BEI = BEF;
                BEJ = BEG;
            } else {
                let BEH = if (APE.abs()) <= DK { 1.0 } else { 0.0 };
                let BFJ;
                let BFK;
                if BEH != 0.0 {
                    let BFB = DU * APE;
                    let BFC = AE - (JA * APE);
                    let BFD = DE * APE;
                    let BFE = (AE - (BFB * BFC)).sqrt();
                    let BFF = BFD * BFE;
                    let BFG = ((APH * DE) * BFE) + ((((((APH * DU) * BFC) + (((APH * JA) * X) * BFB)) * X) * (R / (Q * BFE))) * BFD);
                    BFJ = BFF;
                    BFK = BFG;
                } else {
                    let BFH = ((APE - AE) + BDO).sqrt();
                    let BFI = (APH + BDP) * (R / (Q * BFH));
                    BFJ = BFH;
                    BFK = BFI;
                }
                BEI = BFJ;
                BEJ = BFK;
            }
            let BEK = NY * BEI;
            let BEL = BEK * CX;
            let BEM = CY * BEK;
            let BEN = ((BEJ * NY) * CX) + Lanes([BEM[0], BEM[1], 0.0]);
            let BEO = AE + BH;
            let BEQ = BEP * BEO;
            let BEU = ((((((BEQ * BEO) * BER) * BER) * BES) * BET) * NY) * NY;
            let BEV = ((((((((BI * BEP) * BEO) + (BI * BEQ)) * BER) * BER) * BES) * BET) * NY) * NY;
            let BEW = -BEL;
            let BEX = BEN * X;
            let BEY = BEL - BEW;
            let BEZ = BEN - BEX;
            let BFA = if BEY > L { 1.0 } else { 0.0 };
            let BFS;
            let BFT;
            if BFA != 0.0 {
                let BFL = BEZ * BEY;
                let BFM = ((BEY * BEY) + BEU).sqrt();
                let BFN = BEW + (S * (BEY + BFM));
                let BFO = BEX + ((BEZ + (((BFL + BFL) + Lanes([BEV[0], BEV[1], 0.0])) * (R / (Q * BFM)))) * S);
                BFS = BFN;
                BFT = BFO;
            } else {
                let BFP = BEW - BEL;
                let BFQ = BEX - BEN;
                let BFR = if BFP > L { 1.0 } else { 0.0 };
                let BGK;
                let BGL;
                if BFR != 0.0 {
                    let BFZ = BEV * S;
                    let BGA = BFQ * BFP;
                    let BGB = ((BFP * BFP) + BEU).sqrt();
                    let BGC = BFP + BGB;
                    let BGD = (S * BEU) / BGC;
                    let BGE = BEW + BGD;
                    let BGF = BEX + ((Lanes([BFZ[0], BFZ[1], 0.0]) - ((BFQ + (((BGA + BGA) + Lanes([BEV[0], BEV[1], 0.0])) * (R / (Q * BGB)))) * BGD)) / BGC);
                    BGK = BGE;
                    BGL = BGF;
                } else {
                    let BGG = (AP + BEU).sqrt();
                    let BGH = BEV * (R / (Q * BGG));
                    let BGI = BEW + (S * (BEY + BGG));
                    let BGJ = BEX + ((BEZ + Lanes([BGH[0], BGH[1], 0.0])) * S);
                    BGK = BGI;
                    BGL = BGJ;
                }
                BFS = BGK;
                BFT = BGL;
            }
            let BFU = -AFX;
            let BFV = AGA * X;
            let BFW = BFU - AFX;
            let BFX = BFV - AGA;
            let BFY = if BFW > L { 1.0 } else { 0.0 };
            let BGT;
            let BGU;
            if BFY != 0.0 {
                let BGM = BFX * BFW;
                let BGN = ((BFW * BFW) + BEU).sqrt();
                let BGO = AFX + (S * (BFW + BGN));
                let BGP = AGB + ((Lanes([0.0, 0.0, BFX]) + ((Lanes([0.0, 0.0, (BGM + BGM)]) + Lanes([BEV[0], BEV[1], 0.0])) * (R / (Q * BGN)))) * S);
                BGT = BGO;
                BGU = BGP;
            } else {
                let BGQ = AFX - BFU;
                let BGR = AGA - BFV;
                let BGS = if BGQ > L { 1.0 } else { 0.0 };
                let BHK;
                let BHL;
                if BGS != 0.0 {
                    let BGZ = BEV * S;
                    let BHA = BGR * BGQ;
                    let BHB = ((BGQ * BGQ) + BEU).sqrt();
                    let BHC = BGQ + BHB;
                    let BHD = (S * BEU) / BHC;
                    let BHE = AFX + BHD;
                    let BHF = AGB + ((Lanes([BGZ[0], BGZ[1], 0.0]) - ((Lanes([0.0, 0.0, BGR]) + ((Lanes([0.0, 0.0, (BHA + BHA)]) + Lanes([BEV[0], BEV[1], 0.0])) * (R / (Q * BHB)))) * BHD)) / BHC);
                    BHK = BHE;
                    BHL = BHF;
                } else {
                    let BHG = (AP + BEU).sqrt();
                    let BHH = BEV * (R / (Q * BHG));
                    let BHI = AFX + (S * (BFW + BHG));
                    let BHJ = AGB + ((Lanes([0.0, 0.0, BFX]) + Lanes([BHH[0], BHH[1], 0.0])) * S);
                    BHK = BHI;
                    BHL = BHJ;
                }
                BGT = BHK;
                BGU = BHL;
            }
            let BGW = BFS + (BGV * BGT);
            let BGX = BFT + (BGU * BGV);
            let BHT;
            let BHU;
            if BGY != 0.0 {
                let BHM = BGX * BGW;
                let BHN = (BGW * BGW) + staged[37];
                let BHQ = AE + (BHP * (BHN.powf(BHO)));
                let BHR = BQ / BHQ;
                let BHS = (((((BHM + BHM) * (BHO * (BHN.powf(-1.1666666666666667e0f64)))) * BHP) * BHR) * X) / BHQ;
                BHT = BHR;
                BHU = BHS;
            } else {
                BHT = BQ;
                BHU = APD;
            }
            let BHV = FE - OF;
            let BHW = OH * X;
            let BHX = if BHV > L { 1.0 } else { 0.0 };
            let BIF;
            let BIG;
            if BHX != 0.0 {
                let BHY = BHW * BHV;
                let BIA = ((BHV * BHV) + BHZ).sqrt();
                let BIB = FE - (S * (BHV + BIA));
                let BIC = ((BHW + ((BHY + BHY) * (R / (Q * BIA)))) * S) * X;
                BIF = BIB;
                BIG = BIC;
            } else {
                let BID = OF - FE;
                let BIE = if BID > L { 1.0 } else { 0.0 };
                let BJD;
                let BJE;
                if BIE != 0.0 {
                    let BIV = OH * BID;
                    let BIW = ((BID * BID) + BHZ).sqrt();
                    let BIX = BID + BIW;
                    let BIY = 5e-3f64 / BIX;
                    let BIZ = FE - BIY;
                    let BJA = ((((OH + ((BIV + BIV) * (R / (Q * BIW)))) * BIY) * X) / BIX) * X;
                    BJD = BIZ;
                    BJE = BJA;
                } else {
                    let BJB = FE - (S * (BHV + 1e-1f64));
                    let BJC = (BHW * S) * X;
                    BJD = BJB;
                    BJE = BJC;
                }
                BIF = BJD;
                BIG = BJE;
            }
            let BII = (BIH * BIF).exp();
            let BIJ = (NY * BII).sqrt();
            let BIK = CS * BHT;
            let BIL = CU * BHT;
            let BIM = ((((BIG * BIH) * BII) * NY) * (R / (Q * BIJ))) * BIK;
            let BIN = K * EH;
            let BIO = ((EH * EH) + 4e-2f64).sqrt();
            let BIR = AE + (BIQ * (S * ((-EH) + BIO)));
            let BIS = (BIP * (BIK * BIJ)) / BIR;
            let BIT = ((((K * X) + ((BIN + BIN) * (R / (Q * BIO)))) * S) * BIQ) * BIS;
            let BIU = (((((Lanes([BIL[0], BIL[1], 0.0]) + (BHU * CS)) * BIJ) + Lanes([BIM[0], BIM[1], 0.0])) * BIP) - Lanes([BIT[0], BIT[1], 0.0])) / BIR;
            let BJG = D - BJF;
            let BJI = (Lanes([0.0, G]) - Lanes([BJH, 0.0])) * I;
            let BJJ = (I * (BJG - staged[40])) * DL;
            let BJK = BJI * DL;
            let BJP;
            let BJQ;
            let BJR;
            let BJS;
            if BJL != 0.0 {
                let BJN = if (BJJ.abs()) <= BJM { 1.0 } else { 0.0 };
                let BJX;
                let BJY;
                if BJN != 0.0 {
                    let BJU = BJJ / BJT;
                    let BJV = BJK / BJT;
                    BJX = BJU;
                    BJY = BJV;
                } else {
                    let BJW = if BJJ > BJM { 1.0 } else { 0.0 };
                    let BLU;
                    let BLV;
                    if BJW != 0.0 {
                        let BKC = (((BJT * EF) / BKB) - AE) / BKB;
                        let BKD = BJJ / BJT;
                        let BKE = AE + (BKC * BJJ);
                        let BKF = BKD * BKE;
                        let BKG = ((BJK / BJT) * BKE) + ((BJK * BKC) * BKD);
                        let BKH = if BKF < DO { 1.0 } else { 0.0 };
                        let BMF;
                        let BMG;
                        if BKH != 0.0 {
                            let BLW = (-BKF).exp();
                            let BLX = (BKG * X) * BLW;
                            BMF = BLW;
                            BMG = BLX;
                        } else {
                            let BLY = BKF - DO;
                            let BLZ = S * BLY;
                            let BMA = AE + (BLY * DU);
                            let BMB = AE + (BLZ * BMA);
                            let BMC = AE + (BLY * BMB);
                            let BMD = DY / BMC;
                            let BME = ((((BKG * BMB) + ((((BKG * S) * BMA) + ((BKG * DU) * BLZ)) * BLY)) * BMD) * X) / BMC;
                            BMF = BMD;
                            BMG = BME;
                        }
                        let BMH = S * BKU;
                        let BMI = ((BJJ + (JA * BKU)) - (AE - BMF)).sqrt();
                        let BMK = (BJJ + BMH) - (BMJ * BMI);
                        let BML = BJK - (((BJK - (BMG * X)) * (R / (Q * BMI))) * BMJ);
                        let BMM = if BMK < DO { 1.0 } else { 0.0 };
                        let BMW;
                        let BMX;
                        if BMM != 0.0 {
                            let BMN = (-BMK).exp();
                            let BMO = (BML * X) * BMN;
                            BMW = BMN;
                            BMX = BMO;
                        } else {
                            let BMP = BMK - DO;
                            let BMQ = S * BMP;
                            let BMR = AE + (BMP * DU);
                            let BMS = AE + (BMQ * BMR);
                            let BMT = AE + (BMP * BMS);
                            let BMU = DY / BMT;
                            let BMV = ((((BML * BMS) + ((((BML * S) * BMR) + ((BML * DU) * BMQ)) * BMP)) * BMU) * X) / BMT;
                            BMW = BMU;
                            BMX = BMV;
                        }
                        let BMY = BJJ - BMK;
                        let BMZ = BJK - BML;
                        let BNA = (FS * BMY) + (BKU * (AE - BMW));
                        let BNB = (BMZ * FS) + ((BMX * X) * BKU);
                        let BNC = BMZ * BMY;
                        let BND = (BMY * BMY) - (BKU * ((BMK - AE) + BMW));
                        let BNE = (BNC + BNC) - ((BML + BMX) * BKU);
                        let BNF = BNB * BNA;
                        let BNG = AND * (AE - (BMH * BMW));
                        let BNH = ((BNA * BNA) - (BNG * BND)).sqrt();
                        let BNI = BNA + BNH;
                        let BNJ = (FS * BND) / BNI;
                        let BNK = BMK + BNJ;
                        let BNL = BML + (((BNE * FS) - ((BNB + (((BNF + BNF) - (((((BMX * BMH) * X) * AND) * BND) + (BNE * BNG))) * (R / (Q * BNH)))) * BNJ)) / BNI);
                        BLU = BNK;
                        BLV = BNL;
                    } else {
                        let BKI = -BJJ;
                        let BKJ = BJK * X;
                        let BKK = (EF * BKI) / BJT;
                        let BKL = (BKJ * EF) / BJT;
                        let BKM = BKK - FF;
                        let BKN = BKL * BKM;
                        let BKO = ((BKM * BKM) + FI).sqrt();
                        let BKP = S * ((BKK + FE) - BKO);
                        let BKQ = (BKL - ((BKN + BKN) * (R / (Q * BKO)))) * S;
                        let BKR = BKI - BKP;
                        let BKS = BKJ - BKQ;
                        let BKT = BKS * BKR;
                        let BKV = (BKR * BKR) + (BKU * (BKP + AE));
                        let BKW = (BKT + BKT) + (BKQ * BKU);
                        let BKX = BKS * FS;
                        let BKY = (FS * BKR) - BKU;
                        let BKZ = BKV / BKU;
                        let BLA = (BKZ.ln()) - BKP;
                        let BLB = ((BKW / BKU) * (R / BKZ)) - BKQ;
                        let BLC = BKV + BKY;
                        let BLD = BKW + BKX;
                        let BLE = BLD * BLC;
                        let BLF = S * BKY;
                        let BLG = (BLF * BKY) - BKV;
                        let BLH = (BLC * BLC) + (BLG * BLA);
                        let BLI = (BLE + BLE) + ((((((BKX * S) * BKY) + (BKX * BLF)) - BKW) * BLA) + (BLB * BLG));
                        let BLJ = BKV * BLC;
                        let BLK = BLC * BLA;
                        let BLL = (BLK * BLA) / BLH;
                        let BLM = BLL * BKY;
                        let BLN = BKX * BKY;
                        let BLO = ((BKY * BKY) * DU) - BKV;
                        let BLP = BLH + (BLM * BLO);
                        let BLQ = (BLJ * BLA) / BLP;
                        let BLR = BKP + BLQ;
                        let BLS = BKQ + ((((((BKW * BLC) + (BLD * BKV)) * BLA) + (BLB * BLJ)) - ((BLI + ((((((((((BLD * BLA) + (BLB * BLC)) * BLA) + (BLB * BLK)) - (BLI * BLL)) / BLH) * BKY) + (BKX * BLL)) * BLO) + ((((BLN + BLN) * DU) - BKW) * BLM))) * BLQ)) / BLP);
                        let BLT = if (BLR.abs()) < GP { 1.0 } else { 0.0 };
                        let BNP;
                        let BNQ;
                        if BLT != 0.0 {
                            let BNM = BLR.exp();
                            let BNN = BLS * BNM;
                            BNP = BNM;
                            BNQ = BNN;
                        } else {
                            let BNO = if BLR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BOU;
                            let BOV;
                            if BNO != 0.0 {
                                let BOG = -2.3025850929940458e2f64 - BLR;
                                let BOH = BLS * X;
                                let BOI = S * (-2.3025850929940458e2f64 - BLR);
                                let BOJ = AE + ((-2.3025850929940458e2f64 - BLR) * DU);
                                let BOK = AE + (BOI * BOJ);
                                let BOL = AE + (BOG * BOK);
                                let BOM = IT / BOL;
                                let BON = ((((BOH * BOK) + ((((BOH * S) * BOJ) + ((BOH * DU) * BOI)) * BOG)) * BOM) * X) / BOL;
                                BOU = BOM;
                                BOV = BON;
                            } else {
                                let BOO = BLR - GP;
                                let BOP = S * BOO;
                                let BOQ = AE + (BOO * DU);
                                let BOR = AE + (BOP * BOQ);
                                let BOS = HL * (AE + (BOO * BOR));
                                let BOT = ((BLS * BOR) + ((((BLS * S) * BOQ) + ((BLS * DU) * BOP)) * BOO)) * HL;
                                BOU = BOS;
                                BOV = BOT;
                            }
                            BNP = BOU;
                            BNQ = BOV;
                        }
                        let BNR = S * BKU;
                        let BNS = BKI - BLR;
                        let BNT = BKJ - BLS;
                        let BNU = (FS * BNS) + (BKU * (BNP - AE));
                        let BNV = (BNT * FS) + (BNQ * BKU);
                        let BNW = BNT * BNS;
                        let BNX = (BNS * BNS) + (BKU * ((BLR + AE) - BNP));
                        let BNY = (BNW + BNW) + ((BLS - BNQ) * BKU);
                        let BNZ = BNV * BNU;
                        let BOA = AND * (AE - (BNR * BNP));
                        let BOB = ((BNU * BNU) - (BOA * BNX)).sqrt();
                        let BOC = BNU + BOB;
                        let BOD = (FS * BNX) / BOC;
                        let BOE = -(BLR + BOD);
                        let BOF = (BLS + (((BNY * FS) - ((BNV + (((BNZ + BNZ) - (((((BNQ * BNR) * X) * AND) * BNX) + (BNY * BOA))) * (R / (Q * BOB)))) * BOD)) / BOC)) * X;
                        BLU = BOE;
                        BLV = BOF;
                    }
                    BJX = BLU;
                    BJY = BLV;
                }
                let BJZ = NY * (BJJ - BJX);
                let BKA = (BJK - BJY) * NY;
                BJP = BJZ;
                BJQ = BJX;
                BJR = BKA;
                BJS = BJY;
            } else {
                BJP = V;
                BJQ = V;
                BJR = BJO;
                BJS = BJO;
            }
            let BOX;
            let BOY;
            let BOZ;
            let BPA;
            if C != 0.0 {
                let BPQ;
                let BPR;
                if BOW != 0.0 {
                    let BPO = I * BJG;
                    let BPZ;
                    let BQA;
                    if BPP != 0.0 {
                        let BPT = BJR * I;
                        let BPV = (I * BJP) + BPU;
                        let BPW = V - BPV;
                        let BPX = BPT * X;
                        let BPY = if BPW > L { 1.0 } else { 0.0 };
                        let BQH;
                        let BQI;
                        if BPY != 0.0 {
                            let BQC = BPX * BPW;
                            let BQD = ((BPW * BPW) + BHZ).sqrt();
                            let BQE = BPV + (S * (BPW + BQD));
                            let BQF = BPT + ((BPX + ((BQC + BQC) * (R / (Q * BQD)))) * S);
                            BQH = BQE;
                            BQI = BQF;
                        } else {
                            let BQG = if BPV > L { 1.0 } else { 0.0 };
                            let BQX;
                            let BQY;
                            if BQG != 0.0 {
                                let BQP = BPT * BPV;
                                let BQQ = ((BPV * BPV) + BHZ).sqrt();
                                let BQR = BPV + BQQ;
                                let BQS = 5e-3f64 / BQR;
                                let BQT = BPV + BQS;
                                let BQU = BPT + ((((BPT + ((BQP + BQP) * (R / (Q * BQQ)))) * BQS) * X) / BQR);
                                BQX = BQT;
                                BQY = BQU;
                            } else {
                                let BQV = BPV + (S * (BPW + 1e-1f64));
                                let BQW = BPT + (BPX * S);
                                BQX = BQV;
                                BQY = BQW;
                            }
                            BQH = BQX;
                            BQI = BQY;
                        }
                        let BQJ = BJR * BJP;
                        let BQK = ((BJP * BJP) + AV).sqrt();
                        let BQM = BQK * BQL;
                        let BQN = ((BQJ + BQJ) * (R / (Q * BQK))) * BQL;
                        let BRD;
                        let BRE;
                        if BQO != 0.0 {
                            let BRA = BQZ - BQM;
                            let BRB = BQN * X;
                            let BRC = if BRA > L { 1.0 } else { 0.0 };
                            let BRM;
                            let BRN;
                            if BRC != 0.0 {
                                let BRG = BRB * BRA;
                                let BRH = ((BRA * BRA) + AV).sqrt();
                                let BRI = BQZ - (S * (BRA + BRH));
                                let BRJ = ((BRB + ((BRG + BRG) * (R / (Q * BRH)))) * S) * X;
                                BRM = BRI;
                                BRN = BRJ;
                            } else {
                                let BRK = BQM - BQZ;
                                let BRL = if BRK > L { 1.0 } else { 0.0 };
                                let BRW;
                                let BRX;
                                if BRL != 0.0 {
                                    let BRO = BQN * BRK;
                                    let BRP = ((BRK * BRK) + AV).sqrt();
                                    let BRQ = BRK + BRP;
                                    let BRR = 5e-7f64 / BRQ;
                                    let BRS = BQZ - BRR;
                                    let BRT = ((((BQN + ((BRO + BRO) * (R / (Q * BRP)))) * BRR) * X) / BRQ) * X;
                                    BRW = BRS;
                                    BRX = BRT;
                                } else {
                                    let BRU = BQZ - (S * (BRA + 1e-3f64));
                                    let BRV = (BRB * S) * X;
                                    BRW = BRU;
                                    BRX = BRV;
                                }
                                BRM = BRW;
                                BRN = BRX;
                            }
                            BRD = BRM;
                            BRE = BRN;
                        } else {
                            BRD = BQM;
                            BRE = BQN;
                        }
                        let BSC;
                        let BSD;
                        if BRF != 0.0 {
                            let BRY = -((I * BJQ) + ((staged[50] + BQH) * DL));
                            let BRZ = ((BJS * I) + (BQI * DL)) * X;
                            BSC = BRY;
                            BSD = BRZ;
                        } else {
                            let BSA = -((I * BJQ) + ((staged[51] + BQH) * DL));
                            let BSB = ((BJS * I) + (BQI * DL)) * X;
                            BSC = BSA;
                            BSD = BSB;
                        }
                        let BSE = if BSC < GP { 1.0 } else { 0.0 };
                        let BSJ;
                        let BSK;
                        if BSE != 0.0 {
                            let BSF = BSC.exp();
                            let BSG = AE + BSF;
                            let BSH = BSG.ln();
                            let BSI = (BSD * BSF) * (R / BSG);
                            BSJ = BSH;
                            BSK = BSI;
                        } else {
                            BSJ = BSC;
                            BSK = BSD;
                        }
                        let BSL = BSC + ((I * BPO) * DL);
                        let BSM = BSD + ((BJI * I) * DL);
                        let BSN = if BSL < GP { 1.0 } else { 0.0 };
                        let BSS;
                        let BST;
                        if BSN != 0.0 {
                            let BSO = BSL.exp();
                            let BSP = AE + BSO;
                            let BSQ = BSP.ln();
                            let BSR = (BSM * BSO) * (R / BSP);
                            BSS = BSQ;
                            BST = BSR;
                        } else {
                            BSS = BSL;
                            BST = BSM;
                        }
                        let BSW = BSV + (BSU * BRD);
                        let BSY = BSX * (-1.5e0f64 + (BRD * BSW));
                        let BSZ = ((BRE * BSW) + ((BRE * BSU) * BRD)) * BSX;
                        let BTA = if BSY > V { 1.0 } else { 0.0 };
                        let BTH;
                        let BTI;
                        if BTA != 0.0 {
                            let BTB = S * BSY;
                            let BTC = AE + (BSY * DU);
                            let BTD = AE + (BTB * BTC);
                            let BTE = (BSZ * BTD) + ((((BSZ * S) * BTC) + ((BSZ * DU) * BTB)) * BSY);
                            let BTF = AE + (BSY * BTD);
                            BTH = BTF;
                            BTI = BTE;
                        } else {
                            let BTG = if BSY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BTY;
                            let BTZ;
                            if BTG != 0.0 {
                                let BTO = BSY.exp();
                                let BTP = BSZ * BTO;
                                BTY = BTO;
                                BTZ = BTP;
                            } else {
                                let BTQ = -2.3025850929940458e2f64 - BSY;
                                let BTR = BSZ * X;
                                let BTS = S * (-2.3025850929940458e2f64 - BSY);
                                let BTT = AE + ((-2.3025850929940458e2f64 - BSY) * DU);
                                let BTU = AE + (BTS * BTT);
                                let BTV = AE + (BTQ * BTU);
                                let BTW = IT / BTV;
                                let BTX = ((((BTR * BTU) + ((((BTR * S) * BTT) + ((BTR * DU) * BTS)) * BTQ)) * BTW) * X) / BTV;
                                BTY = BTW;
                                BTZ = BTX;
                            }
                            BTH = BTY;
                            BTI = BTZ;
                        }
                        let BTK = (BTJ * BTH) * I;
                        let BTL = BSS - BSJ;
                        let BTM = BTK * BTL;
                        let BTN = (((BTI * BTJ) * I) * BTL) + ((BST - BSK) * BTK);
                        BPZ = BTM;
                        BQA = BTN;
                    } else {
                        BPZ = V;
                        BQA = BJO;
                    }
                    let BUE;
                    let BUF;
                    if BQB != 0.0 {
                        let BUA = BJR * I;
                        let BUC = (I * BJP) + BUB;
                        let BUD = if BUC > L { 1.0 } else { 0.0 };
                        let BUN;
                        let BUO;
                        if BUD != 0.0 {
                            let BUG = BUA * BUC;
                            let BUH = ((BUC * BUC) + BHZ).sqrt();
                            let BUI = BUC - (S * (BUC + BUH));
                            let BUJ = BUA - ((BUA + ((BUG + BUG) * (R / (Q * BUH)))) * S);
                            BUN = BUI;
                            BUO = BUJ;
                        } else {
                            let BUK = V - BUC;
                            let BUL = BUA * X;
                            let BUM = if BUK > L { 1.0 } else { 0.0 };
                            let BVD;
                            let BVE;
                            if BUM != 0.0 {
                                let BUV = BUL * BUK;
                                let BUW = ((BUK * BUK) + BHZ).sqrt();
                                let BUX = BUK + BUW;
                                let BUY = 5e-3f64 / BUX;
                                let BUZ = BUC - BUY;
                                let BVA = BUA - ((((BUL + ((BUV + BUV) * (R / (Q * BUW)))) * BUY) * X) / BUX);
                                BVD = BUZ;
                                BVE = BVA;
                            } else {
                                let BVB = BUC - (S * (BUC + 1e-1f64));
                                let BVC = BUA - (BUA * S);
                                BVD = BVB;
                                BVE = BVC;
                            }
                            BUN = BVD;
                            BUO = BVE;
                        }
                        let BUP = BJR * BJP;
                        let BUQ = ((BJP * BJP) + AV).sqrt();
                        let BUS = BUQ * BUR;
                        let BUT = ((BUP + BUP) * (R / (Q * BUQ))) * BUR;
                        let BVJ;
                        let BVK;
                        if BUU != 0.0 {
                            let BVG = BVF - BUS;
                            let BVH = BUT * X;
                            let BVI = if BVG > L { 1.0 } else { 0.0 };
                            let BVS;
                            let BVT;
                            if BVI != 0.0 {
                                let BVM = BVH * BVG;
                                let BVN = ((BVG * BVG) + AV).sqrt();
                                let BVO = BVF - (S * (BVG + BVN));
                                let BVP = ((BVH + ((BVM + BVM) * (R / (Q * BVN)))) * S) * X;
                                BVS = BVO;
                                BVT = BVP;
                            } else {
                                let BVQ = BUS - BVF;
                                let BVR = if BVQ > L { 1.0 } else { 0.0 };
                                let BWC;
                                let BWD;
                                if BVR != 0.0 {
                                    let BVU = BUT * BVQ;
                                    let BVV = ((BVQ * BVQ) + AV).sqrt();
                                    let BVW = BVQ + BVV;
                                    let BVX = 5e-7f64 / BVW;
                                    let BVY = BVF - BVX;
                                    let BVZ = ((((BUT + ((BVU + BVU) * (R / (Q * BVV)))) * BVX) * X) / BVW) * X;
                                    BWC = BVY;
                                    BWD = BVZ;
                                } else {
                                    let BWA = BVF - (S * (BVG + 1e-3f64));
                                    let BWB = (BVH * S) * X;
                                    BWC = BWA;
                                    BWD = BWB;
                                }
                                BVS = BWC;
                                BVT = BWD;
                            }
                            BVJ = BVS;
                            BVK = BVT;
                        } else {
                            BVJ = BUS;
                            BVK = BUT;
                        }
                        let BWK;
                        let BWL;
                        if BVL != 0.0 {
                            let BWF = (I * BJQ) + ((BUN - BWE) * DL);
                            let BWG = (BJS * I) + (BUO * DL);
                            BWK = BWF;
                            BWL = BWG;
                        } else {
                            let BWI = (I * BJQ) + ((BUN - BWH) * DL);
                            let BWJ = (BJS * I) + (BUO * DL);
                            BWK = BWI;
                            BWL = BWJ;
                        }
                        let BWM = if BWK < GP { 1.0 } else { 0.0 };
                        let BWR;
                        let BWS;
                        if BWM != 0.0 {
                            let BWN = BWK.exp();
                            let BWO = AE + BWN;
                            let BWP = BWO.ln();
                            let BWQ = (BWL * BWN) * (R / BWO);
                            BWR = BWP;
                            BWS = BWQ;
                        } else {
                            BWR = BWK;
                            BWS = BWL;
                        }
                        let BWT = BWK - ((I * BPO) * DL);
                        let BWU = BWL - ((BJI * I) * DL);
                        let BWV = if BWT < GP { 1.0 } else { 0.0 };
                        let BXA;
                        let BXB;
                        if BWV != 0.0 {
                            let BWW = BWT.exp();
                            let BWX = AE + BWW;
                            let BWY = BWX.ln();
                            let BWZ = (BWU * BWW) * (R / BWX);
                            BXA = BWY;
                            BXB = BWZ;
                        } else {
                            BXA = BWT;
                            BXB = BWU;
                        }
                        let BXE = BXD + (BXC * BVJ);
                        let BXG = BXF * (-1.5e0f64 + (BVJ * BXE));
                        let BXH = ((BVK * BXE) + ((BVK * BXC) * BVJ)) * BXF;
                        let BXI = if (BXG.abs()) < GP { 1.0 } else { 0.0 };
                        let BXM;
                        let BXN;
                        if BXI != 0.0 {
                            let BXJ = BXG.exp();
                            let BXK = BXH * BXJ;
                            BXM = BXJ;
                            BXN = BXK;
                        } else {
                            let BXL = if BXG < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BYH;
                            let BYI;
                            if BXL != 0.0 {
                                let BXT = -2.3025850929940458e2f64 - BXG;
                                let BXU = BXH * X;
                                let BXV = S * (-2.3025850929940458e2f64 - BXG);
                                let BXW = AE + ((-2.3025850929940458e2f64 - BXG) * DU);
                                let BXX = AE + (BXV * BXW);
                                let BXY = AE + (BXT * BXX);
                                let BXZ = IT / BXY;
                                let BYA = ((((BXU * BXX) + ((((BXU * S) * BXW) + ((BXU * DU) * BXV)) * BXT)) * BXZ) * X) / BXY;
                                BYH = BXZ;
                                BYI = BYA;
                            } else {
                                let BYB = BXG - GP;
                                let BYC = S * BYB;
                                let BYD = AE + (BYB * DU);
                                let BYE = AE + (BYC * BYD);
                                let BYF = HL * (AE + (BYB * BYE));
                                let BYG = ((BXH * BYE) + ((((BXH * S) * BYD) + ((BXH * DU) * BYC)) * BYB)) * HL;
                                BYH = BYF;
                                BYI = BYG;
                            }
                            BXM = BYH;
                            BXN = BYI;
                        }
                        let BXP = (BXO * BXM) * I;
                        let BXQ = BWR - BXA;
                        let BXR = BPZ + (BXP * BXQ);
                        let BXS = BQA + ((((BXN * BXO) * I) * BXQ) + ((BWS - BXB) * BXP));
                        BUE = BXR;
                        BUF = BXS;
                    } else {
                        BUE = BPZ;
                        BUF = BQA;
                    }
                    BPQ = BUE;
                    BPR = BUF;
                } else {
                    BPQ = V;
                    BPR = BJO;
                }
                let BYN;
                let BYO;
                if BPS != 0.0 {
                    let BYJ = I * F;
                    let BYK = (OE - APE) * NY;
                    let BYL = (Lanes([OG[0], OG[1], 0.0]) - APH) * NY;
                    let BYU;
                    let BYV;
                    if BYM != 0.0 {
                        let BYP = BYL * I;
                        let BYQ = (I * BYK) + BPU;
                        let BYR = V - BYQ;
                        let BYS = BYP * X;
                        let BYT = if BYR > L { 1.0 } else { 0.0 };
                        let BZC;
                        let BZD;
                        if BYT != 0.0 {
                            let BYX = BYS * BYR;
                            let BYY = ((BYR * BYR) + BHZ).sqrt();
                            let BYZ = BYQ + (S * (BYR + BYY));
                            let BZA = BYP + ((BYS + ((BYX + BYX) * (R / (Q * BYY)))) * S);
                            BZC = BYZ;
                            BZD = BZA;
                        } else {
                            let BZB = if BYQ > L { 1.0 } else { 0.0 };
                            let BZR;
                            let BZS;
                            if BZB != 0.0 {
                                let BZJ = BYP * BYQ;
                                let BZK = ((BYQ * BYQ) + BHZ).sqrt();
                                let BZL = BYQ + BZK;
                                let BZM = 5e-3f64 / BZL;
                                let BZN = BYQ + BZM;
                                let BZO = BYP + ((((BYP + ((BZJ + BZJ) * (R / (Q * BZK)))) * BZM) * X) / BZL);
                                BZR = BZN;
                                BZS = BZO;
                            } else {
                                let BZP = BYQ + (S * (BYR + 1e-1f64));
                                let BZQ = BYP + (BYS * S);
                                BZR = BZP;
                                BZS = BZQ;
                            }
                            BZC = BZR;
                            BZD = BZS;
                        }
                        let BZE = BYL * BYK;
                        let BZF = ((BYK * BYK) + AV).sqrt();
                        let BZG = BZF * BQL;
                        let BZH = ((BZE + BZE) * (R / (Q * BZF))) * BQL;
                        let BZW;
                        let BZX;
                        if BZI != 0.0 {
                            let BZT = BQZ - BZG;
                            let BZU = BZH * X;
                            let BZV = if BZT > L { 1.0 } else { 0.0 };
                            let CAF;
                            let CAG;
                            if BZV != 0.0 {
                                let BZZ = BZU * BZT;
                                let CAA = ((BZT * BZT) + AV).sqrt();
                                let CAB = BQZ - (S * (BZT + CAA));
                                let CAC = ((BZU + ((BZZ + BZZ) * (R / (Q * CAA)))) * S) * X;
                                CAF = CAB;
                                CAG = CAC;
                            } else {
                                let CAD = BZG - BQZ;
                                let CAE = if CAD > L { 1.0 } else { 0.0 };
                                let CAP;
                                let CAQ;
                                if CAE != 0.0 {
                                    let CAH = BZH * CAD;
                                    let CAI = ((CAD * CAD) + AV).sqrt();
                                    let CAJ = CAD + CAI;
                                    let CAK = 5e-7f64 / CAJ;
                                    let CAL = BQZ - CAK;
                                    let CAM = ((((BZH + ((CAH + CAH) * (R / (Q * CAI)))) * CAK) * X) / CAJ) * X;
                                    CAP = CAL;
                                    CAQ = CAM;
                                } else {
                                    let CAN = BQZ - (S * (BZT + 1e-3f64));
                                    let CAO = (BZU * S) * X;
                                    CAP = CAN;
                                    CAQ = CAO;
                                }
                                CAF = CAP;
                                CAG = CAQ;
                            }
                            BZW = CAF;
                            BZX = CAG;
                        } else {
                            BZW = BZG;
                            BZX = BZH;
                        }
                        let CAV;
                        let CAW;
                        if BZY != 0.0 {
                            let CAR = -((I * APE) + ((staged[61] + BZC) * DL));
                            let CAS = ((APH * I) + (BZD * DL)) * X;
                            CAV = CAR;
                            CAW = CAS;
                        } else {
                            let CAT = -((I * APE) + ((staged[62] + BZC) * DL));
                            let CAU = ((APH * I) + (BZD * DL)) * X;
                            CAV = CAT;
                            CAW = CAU;
                        }
                        let CAX = if CAV < GP { 1.0 } else { 0.0 };
                        let CBC;
                        let CBD;
                        if CAX != 0.0 {
                            let CAY = CAV.exp();
                            let CAZ = AE + CAY;
                            let CBA = CAZ.ln();
                            let CBB = (CAW * CAY) * (R / CAZ);
                            CBC = CBA;
                            CBD = CBB;
                        } else {
                            CBC = CAV;
                            CBD = CAW;
                        }
                        let CBE = (K * I) * DL;
                        let CBF = CAV + ((I * BYJ) * DL);
                        let CBG = CAW + Lanes([CBE[0], CBE[1], 0.0]);
                        let CBH = if CBF < GP { 1.0 } else { 0.0 };
                        let CBM;
                        let CBN;
                        if CBH != 0.0 {
                            let CBI = CBF.exp();
                            let CBJ = AE + CBI;
                            let CBK = CBJ.ln();
                            let CBL = (CBG * CBI) * (R / CBJ);
                            CBM = CBK;
                            CBN = CBL;
                        } else {
                            CBM = CBF;
                            CBN = CBG;
                        }
                        let CBO = BSV + (BSU * BZW);
                        let CBQ = CBP * (-1.5e0f64 + (BZW * CBO));
                        let CBR = ((BZX * CBO) + ((BZX * BSU) * BZW)) * CBP;
                        let CBS = if CBQ > V { 1.0 } else { 0.0 };
                        let CBZ;
                        let CCA;
                        if CBS != 0.0 {
                            let CBT = S * CBQ;
                            let CBU = AE + (CBQ * DU);
                            let CBV = AE + (CBT * CBU);
                            let CBW = (CBR * CBV) + ((((CBR * S) * CBU) + ((CBR * DU) * CBT)) * CBQ);
                            let CBX = AE + (CBQ * CBV);
                            CBZ = CBX;
                            CCA = CBW;
                        } else {
                            let CBY = if CBQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CCQ;
                            let CCR;
                            if CBY != 0.0 {
                                let CCG = CBQ.exp();
                                let CCH = CBR * CCG;
                                CCQ = CCG;
                                CCR = CCH;
                            } else {
                                let CCI = -2.3025850929940458e2f64 - CBQ;
                                let CCJ = CBR * X;
                                let CCK = S * (-2.3025850929940458e2f64 - CBQ);
                                let CCL = AE + ((-2.3025850929940458e2f64 - CBQ) * DU);
                                let CCM = AE + (CCK * CCL);
                                let CCN = AE + (CCI * CCM);
                                let CCO = IT / CCN;
                                let CCP = ((((CCJ * CCM) + ((((CCJ * S) * CCL) + ((CCJ * DU) * CCK)) * CCI)) * CCO) * X) / CCN;
                                CCQ = CCO;
                                CCR = CCP;
                            }
                            CBZ = CCQ;
                            CCA = CCR;
                        }
                        let CCC = (CCB * CBZ) * I;
                        let CCD = CBM - CBC;
                        let CCE = CCC * CCD;
                        let CCF = (((CCA * CCB) * I) * CCD) + ((CBN - CBD) * CCC);
                        BYU = CCE;
                        BYV = CCF;
                    } else {
                        BYU = V;
                        BYV = APD;
                    }
                    let CCV;
                    let CCW;
                    if BYW != 0.0 {
                        let CCS = BYL * I;
                        let CCT = (I * BYK) + BUB;
                        let CCU = if CCT > L { 1.0 } else { 0.0 };
                        let CDE;
                        let CDF;
                        if CCU != 0.0 {
                            let CCX = CCS * CCT;
                            let CCY = ((CCT * CCT) + BHZ).sqrt();
                            let CCZ = CCT - (S * (CCT + CCY));
                            let CDA = CCS - ((CCS + ((CCX + CCX) * (R / (Q * CCY)))) * S);
                            CDE = CCZ;
                            CDF = CDA;
                        } else {
                            let CDB = V - CCT;
                            let CDC = CCS * X;
                            let CDD = if CDB > L { 1.0 } else { 0.0 };
                            let CDT;
                            let CDU;
                            if CDD != 0.0 {
                                let CDL = CDC * CDB;
                                let CDM = ((CDB * CDB) + BHZ).sqrt();
                                let CDN = CDB + CDM;
                                let CDO = 5e-3f64 / CDN;
                                let CDP = CCT - CDO;
                                let CDQ = CCS - ((((CDC + ((CDL + CDL) * (R / (Q * CDM)))) * CDO) * X) / CDN);
                                CDT = CDP;
                                CDU = CDQ;
                            } else {
                                let CDR = CCT - (S * (CCT + 1e-1f64));
                                let CDS = CCS - (CCS * S);
                                CDT = CDR;
                                CDU = CDS;
                            }
                            CDE = CDT;
                            CDF = CDU;
                        }
                        let CDG = BYL * BYK;
                        let CDH = ((BYK * BYK) + AV).sqrt();
                        let CDI = CDH * BUR;
                        let CDJ = ((CDG + CDG) * (R / (Q * CDH))) * BUR;
                        let CDY;
                        let CDZ;
                        if CDK != 0.0 {
                            let CDV = BVF - CDI;
                            let CDW = CDJ * X;
                            let CDX = if CDV > L { 1.0 } else { 0.0 };
                            let CEH;
                            let CEI;
                            if CDX != 0.0 {
                                let CEB = CDW * CDV;
                                let CEC = ((CDV * CDV) + AV).sqrt();
                                let CED = BVF - (S * (CDV + CEC));
                                let CEE = ((CDW + ((CEB + CEB) * (R / (Q * CEC)))) * S) * X;
                                CEH = CED;
                                CEI = CEE;
                            } else {
                                let CEF = CDI - BVF;
                                let CEG = if CEF > L { 1.0 } else { 0.0 };
                                let CER;
                                let CES;
                                if CEG != 0.0 {
                                    let CEJ = CDJ * CEF;
                                    let CEK = ((CEF * CEF) + AV).sqrt();
                                    let CEL = CEF + CEK;
                                    let CEM = 5e-7f64 / CEL;
                                    let CEN = BVF - CEM;
                                    let CEO = ((((CDJ + ((CEJ + CEJ) * (R / (Q * CEK)))) * CEM) * X) / CEL) * X;
                                    CER = CEN;
                                    CES = CEO;
                                } else {
                                    let CEP = BVF - (S * (CDV + 1e-3f64));
                                    let CEQ = (CDW * S) * X;
                                    CER = CEP;
                                    CES = CEQ;
                                }
                                CEH = CER;
                                CEI = CES;
                            }
                            CDY = CEH;
                            CDZ = CEI;
                        } else {
                            CDY = CDI;
                            CDZ = CDJ;
                        }
                        let CEX;
                        let CEY;
                        if CEA != 0.0 {
                            let CET = (I * APE) + ((CDE - BWE) * DL);
                            let CEU = (APH * I) + (CDF * DL);
                            CEX = CET;
                            CEY = CEU;
                        } else {
                            let CEV = (I * APE) + ((CDE - BWH) * DL);
                            let CEW = (APH * I) + (CDF * DL);
                            CEX = CEV;
                            CEY = CEW;
                        }
                        let CEZ = if CEX < GP { 1.0 } else { 0.0 };
                        let CFE;
                        let CFF;
                        if CEZ != 0.0 {
                            let CFA = CEX.exp();
                            let CFB = AE + CFA;
                            let CFC = CFB.ln();
                            let CFD = (CEY * CFA) * (R / CFB);
                            CFE = CFC;
                            CFF = CFD;
                        } else {
                            CFE = CEX;
                            CFF = CEY;
                        }
                        let CFG = (K * I) * DL;
                        let CFH = CEX - ((I * BYJ) * DL);
                        let CFI = CEY - Lanes([CFG[0], CFG[1], 0.0]);
                        let CFJ = if CFH < GP { 1.0 } else { 0.0 };
                        let CFO;
                        let CFP;
                        if CFJ != 0.0 {
                            let CFK = CFH.exp();
                            let CFL = AE + CFK;
                            let CFM = CFL.ln();
                            let CFN = (CFI * CFK) * (R / CFL);
                            CFO = CFM;
                            CFP = CFN;
                        } else {
                            CFO = CFH;
                            CFP = CFI;
                        }
                        let CFQ = BXD + (BXC * CDY);
                        let CFS = CFR * (-1.5e0f64 + (CDY * CFQ));
                        let CFT = ((CDZ * CFQ) + ((CDZ * BXC) * CDY)) * CFR;
                        let CFU = if (CFS.abs()) < GP { 1.0 } else { 0.0 };
                        let CFY;
                        let CFZ;
                        if CFU != 0.0 {
                            let CFV = CFS.exp();
                            let CFW = CFT * CFV;
                            CFY = CFV;
                            CFZ = CFW;
                        } else {
                            let CFX = if CFS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CGT;
                            let CGU;
                            if CFX != 0.0 {
                                let CGF = -2.3025850929940458e2f64 - CFS;
                                let CGG = CFT * X;
                                let CGH = S * (-2.3025850929940458e2f64 - CFS);
                                let CGI = AE + ((-2.3025850929940458e2f64 - CFS) * DU);
                                let CGJ = AE + (CGH * CGI);
                                let CGK = AE + (CGF * CGJ);
                                let CGL = IT / CGK;
                                let CGM = ((((CGG * CGJ) + ((((CGG * S) * CGI) + ((CGG * DU) * CGH)) * CGF)) * CGL) * X) / CGK;
                                CGT = CGL;
                                CGU = CGM;
                            } else {
                                let CGN = CFS - GP;
                                let CGO = S * CGN;
                                let CGP = AE + (CGN * DU);
                                let CGQ = AE + (CGO * CGP);
                                let CGR = HL * (AE + (CGN * CGQ));
                                let CGS = ((CFT * CGQ) + ((((CFT * S) * CGP) + ((CFT * DU) * CGO)) * CGN)) * HL;
                                CGT = CGR;
                                CGU = CGS;
                            }
                            CFY = CGT;
                            CFZ = CGU;
                        }
                        let CGB = (CGA * CFY) * I;
                        let CGC = CFE - CFO;
                        let CGD = BYU + (CGB * CGC);
                        let CGE = BYV + ((((CFZ * CGA) * I) * CGC) + ((CFF - CFP) * CGB));
                        CCV = CGD;
                        CCW = CGE;
                    } else {
                        CCV = BYU;
                        CCW = BYV;
                    }
                    BYN = CCV;
                    BYO = CCW;
                } else {
                    BYN = V;
                    BYO = APD;
                }
                BOX = BYN;
                BOY = BPQ;
                BOZ = BYO;
                BPA = BPR;
            } else {
                BOX = V;
                BOY = V;
                BOZ = APD;
                BPA = BJO;
            }
            let BPD = (((EH - APF) - APG) * BPB) * BPC;
            let BPE = (BPD * BHT) * I;
            let BPF = ((((((AFZ - API) - APJ) * BPB) * BPC) * BHT) + (BHU * BPD)) * I;
            let BPH = BPG * AFX;
            let BPI = AGA * BPG;
            let BPM = BPL * (BPJ - BJF);
            let BPN = (Lanes([0.0, BPK]) - Lanes([BJH, 0.0])) * BPL;
            let CHS;
            let CHT;
            let CHU;
            let CHV;
            let CHW;
            let CHX;
            let CHY;
            let CHZ;
            if B != 0.0 {
                let CGY = (CGV - BPJ) * CGX;
                let CGZ = (Lanes([CGW, 0.0]) - Lanes([0.0, BPK])) * CGX;
                let CHB = (BPJ - D) * CHA;
                let CHC = (Lanes([BPK, 0.0]) - Lanes([0.0, G])) * CHA;
                let CHD = E - BJF;
                let CHE = staged[72] + BIS;
                let CHF = CHD * CHE;
                let CHG = (Lanes([0.0, H]) - Lanes([BJH, 0.0])) * CHE;
                let CHH = BIU * CHD;
                let CHI = Lanes([CHG[0], 0.0, CHG[1], 0.0]) + Lanes([0.0, CHH[0], CHH[1], CHH[2]]);
                let CHM = (BJF - CHJ) * CHL;
                let CHN = (Lanes([BJH, 0.0]) - Lanes([0.0, CHK])) * CHL;
                CHS = CGY;
                CHT = CHB;
                CHU = CHF;
                CHV = CHM;
                CHW = CGZ;
                CHX = CHC;
                CHY = CHI;
                CHZ = CHN;
            } else {
                CHS = V;
                CHT = V;
                CHU = V;
                CHV = V;
                CHW = CHO;
                CHX = CHP;
                CHY = CHQ;
                CHZ = CHR;
            }
            let CIA = I * BOX;
            let CIB = BOZ * I;
            let CIC = I * BOY;
            let CID = BPA * I;
            let CIE = if ((BOX + BOY).abs()) > parameters[65] { 1.0 } else { 0.0 };
            let CIF = ddt(16846, BPE);
            let CIH = BPF * CIG;
            let CII = -AFV;
            let CIJ = AFW * X;
            let CIK = ddt(16850, BPH);
            let CIL = BPI * CIG;
            let CIM = ddt(16852, BPM);
            let CIN = BPN * CIG;
            let CIO = if ((CGV - CHJ).abs()) > parameters[10] { 1.0 } else { 0.0 };
            let CIP = BPF[0];
            let CIQ = AGA;
            let CIR = CHW[0];
            let CIS = CHW[1];
            let CIT = CHX[0];
            let CIU = CHX[1];
            let CIV = CHY[0];
            let CIW = CHY[1];
            let CIX = CHY[2];
            let CIY = CHY[3];
            let CIZ = CHZ[0];
            let CJA = CHZ[1];
            let CJB = CIB[0];
            let CJC = CIB[1];
            let CJD = CIB[2];
            let CJE = CID[0];
            let CJF = CID[1];
            let CJG = CIH[0];
            let CJH = CIH[1];
            let CJI = CIH[2];
            let CJJ = CIJ[0];
            let CJK = CIJ[1];
            let CJL = CIL;
            let CJM = CIN[0];
            let CJN = CIN[1];
            let CJO = BPF[1];
            let CJP = BPF[2];
            let CJQ = BPI;
            let CJR = BPN[0];
            let CJS = BPN[1];
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (AFX),
            [6],
            [CIQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(3),
            multiplicity * (CHS),
            [0, 3],
            [CIR, CIS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(4),
            multiplicity * (CHT),
            [3, 4],
            [CIT, CIU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(1),
            multiplicity * (CHU),
            [1, 4, 5, 6],
            [CIV, CIW, CIX, CIY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (CHV),
            [1, 2],
            [CIZ, CJA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(3), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[118],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(4), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[119],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(1), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[120],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(2), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[121],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            Some(5),
            multiplicity * (CIA),
            [4, 5, 6],
            [CJB, CJC, CJD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(1),
            multiplicity * (CIC),
            [1, 4],
            [CJE, CJF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            Some(5),
            multiplicity * (CIF),
            [4, 5, 6],
            [CJG, CJH, CJI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            None,
            multiplicity * (CII),
            [4, 5],
            [CJJ, CJK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (CIK),
            [6],
            [CJL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(1),
            multiplicity * (CIM),
            [1, 3],
            [CJM, CJN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (staged[122]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(1),
            multiplicity * (staged[123]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(3),
            multiplicity * (staged[124]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (staged[125]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(2),
            multiplicity * (staged[126]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(1),
            multiplicity * (staged[127]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(1),
            multiplicity * (staged[128]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = AFX;
        self.canonical_reactive[1] = CHS;
        self.canonical_reactive[2] = CHT;
        self.canonical_reactive[3] = CHU;
        self.canonical_reactive[4] = CHV;
        self.canonical_reactive[5] = staged[118];
        self.canonical_reactive[6] = staged[119];
        self.canonical_reactive[7] = staged[120];
        self.canonical_reactive[8] = staged[121];
        self.canonical_reactive[9] = CIA;
        self.canonical_reactive[10] = CIC;
        self.canonical_reactive[11] = BPE;
        self.canonical_reactive[12] = CIP;
        self.canonical_reactive[13] = CJO;
        self.canonical_reactive[14] = CJP;
        self.canonical_reactive[15] = CII;
        self.canonical_reactive[16] = BPH;
        self.canonical_reactive[17] = CJQ;
        self.canonical_reactive[18] = BPM;
        self.canonical_reactive[19] = CJR;
        self.canonical_reactive[20] = CJS;
        self.canonical_reactive[21] = staged[122];
        self.canonical_reactive[22] = staged[123];
        self.canonical_reactive[23] = staged[124];
        self.canonical_reactive[24] = staged[125];
        self.canonical_reactive[25] = staged[126];
        self.canonical_reactive[26] = staged[127];
        self.canonical_reactive[27] = staged[128];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(5),
            &[4, 5, 6],
            &[cached[12], cached[13], cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            None,
            &[6],
            &[cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(1),
            &[1, 3],
            &[cached[19], cached[20]],
            &[],
            &[],
            multiplicity,
        );
    }

}
