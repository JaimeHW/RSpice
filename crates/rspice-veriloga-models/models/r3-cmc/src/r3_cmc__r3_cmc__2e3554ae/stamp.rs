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
        self.canonical_staged[65] = values[0];
        self.canonical_staged[66] = values[1];
        self.canonical_staged[67] = values[2];
        self.canonical_staged[2] = values[3];
        self.canonical_staged[1] = values[4];
        self.canonical_staged[0] = values[5];
        self.canonical_staged[24] = values[6];
        self.canonical_staged[26] = values[7];
        self.canonical_staged[44] = values[8];
        self.canonical_staged[5] = values[9];
        self.canonical_staged[85] = values[10];
        self.canonical_staged[6] = values[11];
        self.canonical_staged[86] = values[12];
        self.canonical_staged[87] = values[13];
        self.canonical_staged[8] = values[14];
        self.canonical_staged[10] = values[15];
        self.canonical_staged[12] = values[16];
        self.canonical_staged[13] = values[17];
        self.canonical_staged[14] = values[18];
        self.canonical_staged[15] = values[19];
        self.canonical_staged[16] = values[20];
        self.canonical_staged[92] = values[21];
        self.canonical_staged[22] = values[22];
        self.canonical_staged[93] = values[23];
        self.canonical_staged[23] = values[24];
        self.canonical_staged[94] = values[25];
        self.canonical_staged[28] = values[26];
        self.canonical_staged[29] = values[27];
        self.canonical_staged[95] = values[28];
        self.canonical_staged[30] = values[29];
        self.canonical_staged[31] = values[30];
        self.canonical_staged[96] = values[31];
        self.canonical_staged[32] = values[32];
        self.canonical_staged[33] = values[33];
        self.canonical_staged[61] = values[34];
        self.canonical_staged[62] = values[35];
        self.canonical_staged[63] = values[36];
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
                let B = 1e0f64;
                let K = parameters[63];
                let M = 2e0f64;
                let N = parameters[64];
                let P = 0e0f64;
                let W = parameters[71];
                let Z = parameters[72];
                let AD = 1e0f64;
                let AG = parameters[90];
                let AL = 5e-1f64;
                let AM = parameters[73];
                let AR = parameters[80];
                let mut oO = 0.0;
                let mut oQ = 0.0;
                let mut oR = 0.0;
                let mut oS = 0.0;
                let mut oT = 0.0;
                let mut oU = 0.0;
                let mut oAH = 0.0;
                let mut oAJ = 0.0;
                let mut oAN = 0.0;
                let mut oAO = 0.0;
                let mut oAP = 0.0;
                let mut oAS = 0.0;
                let mut oAT = 0.0;
                let mut oAU = 0.0;
                let mut oAX = 0.0;
                let A = if 1.003e3f64 != parameters[20] { 1.0 } else { 0.0 };
                let C = if B != parameters[17] { 1.0 } else { 0.0 };
                let D = if B < parameters[18] { 1.0 } else { 0.0 };
                let E = ((B - (1e-2f64 * parameters[23])) * parameters[22]) * 1e6f64;
                let F = E * E;
                let G = 2.7315e2f64 + parameters[28];
                let H = parameters[35] + B;
                let I = 0e0f64 * F;
                let J = 0e0f64 * F;
                let L = if K > B { 1.0 } else { 0.0 };
                if L != 0.0 {
                    let O = M * N;
                    oO = O;
                } else {
                    let Q = if K > P { 1.0 } else { 0.0 };
                    oQ = Q;
                    if Q != 0.0 {
                        let R = M * N;
                        oR = R;
                    } else {
                    }
                }
                if L != 0.0 {
                    let S = if K > M { 1.0 } else { 0.0 };
                    oS = S;
                } else {
                    let T = if K > P { 1.0 } else { 0.0 };
                    oT = T;
                    if T != 0.0 {
                        let U = M * parameters[46];
                        oU = U;
                    } else {
                    }
                }
                let V = if parameters[66] > P { 1.0 } else { 0.0 };
                let X = W * I;
                let Y = W * J;
                let AA = Z * I;
                let AB = Z * J;
                let AC = -parameters[21];
                let AE = parameters[92] - AD;
                let AF = if parameters[69] > P { 1.0 } else { 0.0 };
                if AF != 0.0 {
                    let AH = -AG;
                    oAH = AH;
                } else {
                }
                let AI = if parameters[76] > P { 1.0 } else { 0.0 };
                if AI != 0.0 {
                    let AJ = -AG;
                    oAJ = AJ;
                } else {
                }
                let AK = if Z > P { 1.0 } else { 0.0 };
                if AK != 0.0 {
                    let AN = AL * AM;
                    oAN = AN;
                    let AO = -5e-1f64 * AM;
                    oAO = AO;
                    let AP = parameters[74] - AD;
                    oAP = AP;
                } else {
                }
                let AQ = if parameters[79] > P { 1.0 } else { 0.0 };
                if AQ != 0.0 {
                    let AS = AL * AR;
                    oAS = AS;
                    let AT = -5e-1f64 * AR;
                    oAT = AT;
                    let AU = parameters[81] - AD;
                    oAU = AU;
                } else {
                }
                let AV = if parameters[83] > P { 1.0 } else { 0.0 };
                if AV != 0.0 {
                    let AX = parameters[27] / parameters[84];
                    oAX = AX;
                } else {
                }
                let AW = if parameters[60] > P { 1.0 } else { 0.0 };
            [A, C, D, E, G, H, I, J, L, oO, oQ, oR, oS, oT, oU, V, X, Y, AA, AB, AC, AF, oAH, AI, oAJ, AK, oAN, oAO, AQ, oAS, oAT, AV, oAX, AW, AE, oAP, oAU]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 43] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = 1e0f64;
                let B = staged[2];
                let L = 2e0f64;
                let M = parameters[5];
                let N = 0e0f64;
                let P = parameters[8];
                let W = parameters[127];
                let Z = parameters[16];
                let AA = parameters[119];
                let AB = parameters[122];
                let AC = parameters[125];
                let AD = multiplicity;
                let AG = parameters[120];
                let AH = parameters[123];
                let AI = parameters[126];
                let AK = parameters[118];
                let AL = parameters[121];
                let AM = parameters[124];
                let AN = 1e-2f64;
                let BI = parameters[53];
                let BO = parameters[15];
                let BR = staged[44];
                let BT = staged[86];
                let BU = staged[87];
                let BX = 4e0f64;
                let CB = staged[10];
                let CD = parameters[67];
                let CE = parameters[66];
                let CQ = parameters[78];
                let CT = parameters[79];
                let CY = parameters[65];
                let DD = parameters[26];
                let DG = parameters[13];
                let DN = 0e0f64;
                let DO = 0e0f64;
                let DP = 0e0f64;
                let DQ = 0e0f64;
                let mut oAP = 0.0;
                let mut oAX = 0.0;
                let mut oBB = 0.0;
                let mut oBW = 0.0;
                let mut oCA = 0.0;
                let mut oCL = 0.0;
                let mut oCZ = 0.0;
                let mut oDA = 0.0;
                let mut oDL = 0.0;
                let mut oDM = 0.0;
                let C = parameters[0] * B;
                let D = parameters[1] * B;
                let E = if C < parameters[31] { 1.0 } else { 0.0 };
                let F = if C > parameters[32] { 1.0 } else { 0.0 };
                let G = if D < parameters[29] { 1.0 } else { 0.0 };
                let H = if D > parameters[30] { 1.0 } else { 0.0 };
                let I = parameters[4] * B;
                let J = parameters[7] * B;
                let K = D * C;
                let O = if M > N { 1.0 } else { 0.0 };
                let Q = if P > N { 1.0 } else { 0.0 };
                let R = O + Q;
                let S = (L * D) + (R * C);
                let T = 5e-1f64 * R;
                let U = (((C + parameters[38]) + (parameters[39] / C)) + (parameters[42] * (A - (((-C) / parameters[41]).exp())))) / (A - ((parameters[40] * (parameters[2] * B)) / K));
                let V = D + (T * (parameters[43] + (parameters[44] / C)));
                let X;
                let Y;
                if W != 0.0 {
                    X = V;
                    Y = U;
                } else {
                    X = D;
                    Y = C;
                }
                let AQ;
                let AR;
                let AS;
                if Z != 0.0 {
                    let AE = AD * X;
                    let AF = (U + (AA * AB)) + ((parameters[11] * AC) / (AE.sqrt()));
                    let AJ = (V + (AG * AH)) + ((parameters[12] * AI) / ((AD * Y).sqrt()));
                    let AO = (AN * ((AK * AL) + ((parameters[10] * AM) / ((AE * Y).sqrt())))).exp();
                    AQ = AF;
                    AR = AJ;
                    AS = AO;
                } else {
                    let AP = if (if AA != N { 1.0 } else { 0.0 }) != 0.0 && (if (if AC > N { 1.0 } else { 0.0 }) != 0.0 || (if AB > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAP = AP;
                    let AW = if AP != 0.0 {
                        let AU = AC / ((AD * X).sqrt());
                        let AV = U + (AA * (((AB * AB) + (AU * AU)).sqrt()));
                        AV
                    } else {
                        U
                    };
                    let AX = if (if AG != N { 1.0 } else { 0.0 }) != 0.0 && (if (if AI > N { 1.0 } else { 0.0 }) != 0.0 || (if AH > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAX = AX;
                    let BA = if AX != 0.0 {
                        let AY = AI / ((AD * Y).sqrt());
                        let AZ = V + (AG * (((AH * AH) + (AY * AY)).sqrt()));
                        AZ
                    } else {
                        V
                    };
                    let BB = if (if AK != N { 1.0 } else { 0.0 }) != 0.0 && (if (if AM > N { 1.0 } else { 0.0 }) != 0.0 || (if AL > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBB = BB;
                    let BE = if BB != 0.0 {
                        let BC = AM / (((AD * X) * Y).sqrt());
                        let BD = ((AN * AK) * (((AL * AL) + (BC * BC)).sqrt())).exp();
                        BD
                    } else {
                        A
                    };
                    AQ = AW;
                    AR = BA;
                    AS = BE;
                }
                let AT = if AQ <= N { 1.0 } else { 0.0 };
                let BF = if AR <= N { 1.0 } else { 0.0 };
                let BG = AR + parameters[45];
                let BH = if BG <= N { 1.0 } else { 0.0 };
                let BJ;
                let BK;
                if BI != 0.0 {
                    BJ = AQ;
                    BK = AR;
                } else {
                    BJ = C;
                    BK = D;
                }
                let BL = A / (BJ.powf(parameters[56]));
                let BM = A / (BK.powf(parameters[58]));
                let BN = ((parameters[54] * (A + (parameters[55] * BL))) * (A + (parameters[57] * BM))) * (A + ((parameters[59] * BL) * BM));
                let BQ = if BO != 0.0 {
                    N
                } else {
                    let BP = parameters[49] + ((((parameters[50] * BK) + (parameters[51] * BJ)) + parameters[52]) / (BK * BJ));
                    BP
                };
                let BS = parameters[47] / (A + (parameters[48] / AR));
                if BR != 0.0 {
                    if BT != 0.0 {
                        let BW = -BS;
                        oBW = BW;
                    } else {
                    }
                } else {
                    let CA = if BU != 0.0 {
                        let BY = (BX * BS) * BS;
                        BY
                    } else {
                        let BZ = (BX * BS) * BS;
                        BZ
                    };
                    oCA = CA;
                }
                let BV = (parameters[37] * AS) * (AR / AQ);
                let CC = if CB != 0.0 && O != 0.0 { 1.0 } else { 0.0 };
                let CG = if CC != 0.0 {
                    let CF = (CE + (CD / C)) / M;
                    CF
                } else {
                    N
                };
                let CH = if CB != 0.0 && Q != 0.0 { 1.0 } else { 0.0 };
                let CJ = if CH != 0.0 {
                    let CI = (CE + (CD / C)) / P;
                    CI
                } else {
                    N
                };
                let CN = if BO != 0.0 {
                    N
                } else {
                    let CK = M + P;
                    let CL = ((parameters[110] + (parameters[111] * S)) + (parameters[112] * K)) + (parameters[113] * CK);
                    oCL = CL;
                    let CM = ((parameters[114] + (parameters[115] * S)) + (parameters[116] * K)) + (parameters[117] * CK);
                    CM
                };
                let CO = (parameters[93] + (parameters[97] / AQ)) + ((T * (parameters[95] + (parameters[99] / AQ))) / AR);
                let CP = (parameters[94] + (parameters[98] / AQ)) + ((T * (parameters[96] + (parameters[100] / AQ))) / AR);
                let CR = staged[12] + (CQ * I);
                let CS = staged[13] + (CQ * J);
                let CU = staged[14] + (CT * I);
                let CV = staged[15] + (CT * J);
                let CW = if BO == 0.0 { 1.0 } else { 0.0 };
                let CX = if staged[33] != 0.0 && CW != 0.0 { 1.0 } else { 0.0 };
                if CX != 0.0 {
                    let CZ = (BX * CY) * CY;
                    oCZ = CZ;
                    let DA = L * CY;
                    oDA = DA;
                } else {
                }
                let DB = if CU > N { 1.0 } else { 0.0 };
                let DC = if CV > N { 1.0 } else { 0.0 };
                let DE = if (CG / AD) <= DD { 1.0 } else { 0.0 };
                let DF = if (CJ / AD) <= DD { 1.0 } else { 0.0 };
                let DH;
                let DI;
                let DJ;
                let DK;
                if DG != 0.0 {
                    let DL = if CG > N { 1.0 } else { 0.0 };
                    oDL = DL;
                    let DM = if CJ > N { 1.0 } else { 0.0 };
                    oDM = DM;
                    DH = DN;
                    DI = DO;
                    DJ = DP;
                    DK = DQ;
                } else {
                    DH = N;
                    DI = N;
                    DJ = N;
                    DK = N;
                }
            [E, F, G, H, I, J, oAP, oAX, oBB, AQ, AT, BF, BG, BH, BN, BQ, oBW, BV, CC, CH, oCL, CO, CP, CR, CS, CW, CX, oCZ, oDA, oCA, DB, DC, CN, CG, DE, CJ, DF, oDL, oDM, DH, DI, DJ, DK]
        };
        self.canonical_staged[72] = produced[0];
        self.canonical_staged[73] = produced[1];
        self.canonical_staged[74] = produced[2];
        self.canonical_staged[75] = produced[3];
        self.canonical_staged[25] = produced[4];
        self.canonical_staged[27] = produced[5];
        self.canonical_staged[76] = produced[6];
        self.canonical_staged[78] = produced[7];
        self.canonical_staged[79] = produced[8];
        self.canonical_staged[54] = produced[9];
        self.canonical_staged[77] = produced[10];
        self.canonical_staged[80] = produced[11];
        self.canonical_staged[36] = produced[12];
        self.canonical_staged[81] = produced[13];
        self.canonical_staged[3] = produced[14];
        self.canonical_staged[4] = produced[15];
        self.canonical_staged[7] = produced[16];
        self.canonical_staged[9] = produced[17];
        self.canonical_staged[90] = produced[18];
        self.canonical_staged[91] = produced[19];
        self.canonical_staged[11] = produced[20];
        self.canonical_staged[19] = produced[21];
        self.canonical_staged[18] = produced[22];
        self.canonical_staged[55] = produced[23];
        self.canonical_staged[56] = produced[24];
        self.canonical_staged[48] = produced[25];
        self.canonical_staged[97] = produced[26];
        self.canonical_staged[34] = produced[27];
        self.canonical_staged[35] = produced[28];
        self.canonical_staged[89] = produced[29];
        self.canonical_staged[104] = produced[30];
        self.canonical_staged[105] = produced[31];
        self.canonical_staged[57] = produced[32];
        self.canonical_staged[58] = produced[33];
        self.canonical_staged[106] = produced[34];
        self.canonical_staged[59] = produced[35];
        self.canonical_staged[107] = produced[36];
        self.canonical_staged[109] = produced[37];
        self.canonical_staged[110] = produced[38];
        self.canonical_staged[111] = produced[39];
        self.canonical_staged[112] = produced[40];
        self.canonical_staged[113] = produced[41];
        self.canonical_staged[114] = produced[42];
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
        let produced: [f64; 35] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = 1e0f64;
                let C = 2.7315e2f64;
                let H = parameters[35];
                let J = parameters[36];
                let O = staged[1];
                let U = 1e-1f64;
                let Z = parameters[15];
                let AA = staged[4];
                let AC = 0e0f64;
                let AH = 5e-1f64;
                let AK = staged[44];
                let AP = staged[85];
                let AU = parameters[46];
                let AW = staged[86];
                let AX = staged[87];
                let BJ = staged[89];
                let BK = parameters[109];
                let BN = parameters[63];
                let mut oK = 0.0;
                let mut oAD = 0.0;
                let mut oBO = 0.0;
                let mut oBP = 0.0;
                let mut oBR = 0.0;
                let mut oBU = 0.0;
                let mut oBW = 0.0;
                let mut oBY = 0.0;
                let mut oBZ = 0.0;
                let mut oCA = 0.0;
                let mut oCD = 0.0;
                let mut oCE = 0.0;
                let mut oCF = 0.0;
                let B = temperature + parameters[9];
                let D = B - C;
                let E = if D < parameters[24] { 1.0 } else { 0.0 };
                let F = if D > parameters[25] { 1.0 } else { 0.0 };
                let G = if D < staged[0] { 1.0 } else { 0.0 };
                let L;
                if G != 0.0 {
                    let I = H + (((D - H) - A).exp());
                    L = I;
                } else {
                    let K = if D > (J - A) { 1.0 } else { 0.0 };
                    oK = K;
                    let S = if K != 0.0 {
                        let R = J - (((J - D) - A).exp());
                        R
                    } else {
                        D
                    };
                    L = S;
                }
                let M = L + C;
                let N = (1.3806505e-23f64 * M) / 1.60217653e-19f64;
                let P = M / O;
                let Q = M - O;
                let T = staged[3] * (A + (Q * (parameters[103] + (Q * parameters[104]))));
                let V = if T > U { 1.0 } else { 0.0 };
                let W = if V != 0.0 {
                    T
                } else {
                    U
                };
                let X = W.sqrt();
                let Y = X / (W + 1e4f64);
                let AB = if AA < Y { 1.0 } else { 0.0 };
                let AF;
                let AG;
                if AB != 0.0 {
                    let AD = if AA > AC { 1.0 } else { 0.0 };
                    oAD = AD;
                    let AL = if AD != 0.0 {
                        AA
                    } else {
                        AC
                    };
                    let AM = Y * Y;
                    AF = AM;
                    AG = AL;
                } else {
                    let AE = AA * AA;
                    AF = AE;
                    AG = AA;
                }
                let AI = W * AH;
                let AJ = (AH / AF) - AI;
                let AQ;
                let AR;
                if AK != 0.0 {
                    let AN = AJ - (staged[5] / AF);
                    let AO = (1.666666666666667e-1f64 / AF) - AI;
                    AQ = AN;
                    AR = AO;
                } else {
                    let AT = if AP != 0.0 {
                        let AS = AJ - ((staged[6] / AF).sqrt());
                        AS
                    } else {
                        AJ
                    };
                    AQ = AT;
                    AR = AC;
                }
                let AY;
                let AZ;
                if AK != 0.0 {
                    let AV = AU * N;
                    let BF = if AW != 0.0 {
                        let BD = (5.5e-1f64 * N) * (A + ((staged[7] / N).exp()));
                        BD
                    } else {
                        let BE = 1.1e0f64 * N;
                        BE
                    };
                    AY = AV;
                    AZ = BF;
                } else {
                    let BI = if AX != 0.0 {
                        let BG = staged[8] * N;
                        BG
                    } else {
                        let BH = AU * N;
                        BH
                    };
                    AY = BI;
                    AZ = BJ;
                }
                let BA = A - (AG * X);
                let BB = staged[9] * BA;
                let BC = if BB <= 1e-99f64 { 1.0 } else { 0.0 };
                let BM = if Z != 0.0 {
                    AC
                } else {
                    let BL = staged[11] * (P.powf(BK));
                    BL
                };
                if BN != 0.0 {
                    let BO = BB * BA;
                    oBO = BO;
                } else {
                }
                if BN != 0.0 {
                } else {
                    let BP = -4e-1f64 * W;
                    oBP = BP;
                }
                let BQ = if AK != 0.0 && (if AG > 1e-9f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if BQ != 0.0 {
                    let BR = if BN > 2e0f64 { 1.0 } else { 0.0 };
                    oBR = BR;
                } else {
                }
                let BS = if BM > AC { 1.0 } else { 0.0 };
                let BT = if (if BS != 0.0 && parameters[14] != 0.0 { 1.0 } else { 0.0 }) != 0.0 && staged[48] != 0.0 { 1.0 } else { 0.0 };
                if BT != 0.0 {
                    let BU = if BK == AC { 1.0 } else { 0.0 };
                    oBU = BU;
                    if BU != 0.0 {
                    } else {
                        let BX;
                        if G != 0.0 {
                            let BV = H + (((D - H) - A).exp());
                            BX = BV;
                        } else {
                            let BW = if D > (J - A) { 1.0 } else { 0.0 };
                            oBW = BW;
                            let CC = if BW != 0.0 {
                                let CB = J - (((J - D) - A).exp());
                                CB
                            } else {
                                D
                            };
                            BX = CC;
                        }
                        let BY = BX + C;
                        oBY = BY;
                        let BZ = BK + A;
                        oBZ = BZ;
                        let CA = if (BZ.abs()) > U { 1.0 } else { 0.0 };
                        oCA = CA;
                        if CA != 0.0 {
                            let CD = BM * BY;
                            oCD = CD;
                            let CE = BZ - 1e0f64;
                            oCE = CE;
                        } else {
                            let CF = AH * BK;
                            oCF = CF;
                        }
                    }
                } else {
                }
                let CH = if BS != 0.0 {
                    let CG = A / BM;
                    CG
                } else {
                    AC
                };
            [B, E, F, G, oK, N, V, W, AB, oAD, AF, AJ, AG, BB, BC, oBO, AQ, AY, oBP, AR, BQ, AZ, oBR, BM, BS, BT, oBU, oBW, oBY, oBZ, oCA, oCD, oCF, CH, oCE]
        };
        self.canonical_staged[17] = produced[0];
        self.canonical_staged[68] = produced[1];
        self.canonical_staged[69] = produced[2];
        self.canonical_staged[70] = produced[3];
        self.canonical_staged[71] = produced[4];
        self.canonical_staged[45] = produced[5];
        self.canonical_staged[82] = produced[6];
        self.canonical_staged[39] = produced[7];
        self.canonical_staged[83] = produced[8];
        self.canonical_staged[84] = produced[9];
        self.canonical_staged[41] = produced[10];
        self.canonical_staged[43] = produced[11];
        self.canonical_staged[46] = produced[12];
        self.canonical_staged[21] = produced[13];
        self.canonical_staged[88] = produced[14];
        self.canonical_staged[20] = produced[15];
        self.canonical_staged[37] = produced[16];
        self.canonical_staged[38] = produced[17];
        self.canonical_staged[40] = produced[18];
        self.canonical_staged[42] = produced[19];
        self.canonical_staged[98] = produced[20];
        self.canonical_staged[47] = produced[21];
        self.canonical_staged[99] = produced[22];
        self.canonical_staged[49] = produced[23];
        self.canonical_staged[108] = produced[24];
        self.canonical_staged[100] = produced[25];
        self.canonical_staged[101] = produced[26];
        self.canonical_staged[102] = produced[27];
        self.canonical_staged[50] = produced[28];
        self.canonical_staged[51] = produced[29];
        self.canonical_staged[103] = produced[30];
        self.canonical_staged[52] = produced[31];
        self.canonical_staged[53] = produced[32];
        self.canonical_staged[60] = produced[33];
        self.canonical_staged[64] = produced[34];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 8156 => 0usize, 8158 => 1usize, 8160 => 2usize, _ => usize::MAX };
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
            let A = 0e0f64;
            let C = 1e0f64;
            let D = node_potentials[5];
            let E = node_potentials[4];
            let F = 1e0f64;
            let G = 1e0f64;
            let H = staged[16];
            let K = node_potentials[1];
            let L = 1e0f64;
            let Q = node_potentials[3];
            let R = 2.7315e2f64;
            let U = parameters[35];
            let W = 1e0f64;
            let Z = parameters[36];
            let AE = 1.3806505e-23f64;
            let AF = 1.60217653e-19f64;
            let AI = staged[1];
            let AM = staged[18];
            let AR = -1e0f64;
            let AX = 1e-2f64;
            let AY = 1e1f64;
            let BA = 1e-1f64;
            let BF = parameters[63];
            let BG = staged[20];
            let BK = staged[21];
            let BQ = parameters[102];
            let CA = parameters[92];
            let CD = staged[92];
            let CE = staged[22];
            let CG = 1e0f64;
            let CH = parameters[91];
            let CI = parameters[70];
            let CK = parameters[69];
            let CO = parameters[27];
            let CU = 0e0f64;
            let CZ = staged[93];
            let DA = staged[23];
            let DC = parameters[77];
            let DE = parameters[76];
            let DR = staged[24];
            let DU = staged[25];
            let DY = staged[26];
            let EB = staged[27];
            let EF = staged[94];
            let EH = 2e0f64;
            let EJ = staged[28];
            let EM = staged[29];
            let ES = 3e0f64;
            let EV = parameters[90];
            let FB = 4e0f64;
            let FD = 2e0f64;
            let FE = 5e-1f64;
            let FJ = parameters[73];
            let FL = parameters[74];
            let FM = parameters[72];
            let FT = staged[95];
            let FW = staged[30];
            let FZ = staged[31];
            let GR = parameters[80];
            let GT = parameters[81];
            let GU = parameters[79];
            let HC = staged[96];
            let HD = parameters[106];
            let HF = parameters[83];
            let HJ = parameters[85];
            let HQ = staged[97];
            let HT = parameters[107];
            let IE = parameters[62];
            let IF = 1e3f64;
            let IQ = staged[36];
            let IT = 1e5f64;
            let IV = parameters[61];
            let IZ = parameters[60];
            let JI = staged[34];
            let JL = staged[35];
            let JO = parameters[65];
            let KJ = -1e0f64;
            let KS = staged[37];
            let KU = staged[38];
            let LM = staged[39];
            let LW = -4e-1f64;
            let MA = Lanes([0e0f64; 3]);
            let MD = staged[41];
            let MK = 1.5e0f64;
            let NJ = 3.333333333333333e-1f64;
            let NN = 9e0f64;
            let NS = 2.7e1f64;
            let NV = 2.5e-1f64;
            let OD = staged[98];
            let OE = -5e-1f64;
            let OJ = -5e-1f64;
            let OS = 1e-6f64;
            let PD = 1e4f64;
            let PR = 7.5e-1f64;
            let QI = -2.5e-1f64;
            let QP = -2.5e-1f64;
            let QU = staged[43];
            let RS = staged[46];
            let SA = staged[47];
            let SO = parameters[64];
            let UF = parameters[47];
            let VF = staged[99];
            let WK = Lanes([0e0f64; 4]);
            let XP = Lanes([0e0f64; 3]);
            let AAF = parameters[84];
            let AAL = Lanes([0e0f64; 3]);
            let AAV = 1e0f64;
            let AAX = branch_unknown_flows[0];
            let AAY = 1e0f64;
            let ABD = 1e0f64;
            let ABF = branch_unknown_flows[1];
            let ABG = 1e0f64;
            let ABL = staged[100];
            let AEE = staged[101];
            let AEF = 1e6f64;
            let AEI = Lanes([0e0f64; 8]);
            let AET = staged[54];
            let AEU = parameters[33];
            let AEW = staged[49];
            let AFB = staged[103];
            let AFC = staged[50];
            let AFE = staged[51];
            let AFF = staged[52];
            let AFJ = staged[53];
            let AFQ = parameters[34];
            let AFU = staged[104];
            let AFX = staged[105];
            let AGA = 4e-2f64;
            let AGN = parameters[68];
            let AGQ = parameters[75];
            let AHC = -5e-1f64;
            let AJE = parameters[82];
            let AJR = -5e-1f64;
            let ALR = staged[55];
            let ALT = staged[56];
            let ALZ = staged[57];
            let AMC = staged[106];
            let ANE = -5e-1f64;
            let APS = -5e-1f64;
            let ARQ = staged[58];
            let ARU = Lanes([0e0f64; 3]);
            let ARY = Lanes([0e0f64; 2]);
            let ASD = staged[107];
            let ASE = staged[59];
            let ASI = Lanes([0e0f64; 3]);
            let ASM = Lanes([0e0f64; 2]);
            let ASS = ddt_scale();
            let ASY = parameters[13];
            let ATC = 0e0f64;
            let ATE = 0e0f64;
            let B = ctx.simparam_or("gmin", A);
            let I = H * (D - E);
            let J = (Lanes([0.0, F]) - Lanes([G, 0.0])) * H;
            let M = H * (K - E);
            let N = (Lanes([L, 0.0]) - Lanes([0.0, G])) * H;
            let O = H * (K - D);
            let P = (Lanes([L, 0.0]) - Lanes([0.0, F])) * H;
            let S = (staged[17] + Q) - R;
            let T = if S < staged[0] { 1.0 } else { 0.0 };
            let AB;
            let AC;
            if T != 0.0 {
                let V = ((S - U) - C).exp();
                let X = W * V;
                let Y = U + V;
                AB = Y;
                AC = X;
            } else {
                let AA = if S > (Z - C) { 1.0 } else { 0.0 };
                let AV;
                let AW;
                if AA != 0.0 {
                    let AS = ((Z - S) - C).exp();
                    let AT = Z - AS;
                    let AU = ((W * AR) * AS) * AR;
                    AV = AT;
                    AW = AU;
                } else {
                    AV = S;
                    AW = W;
                }
                AB = AV;
                AC = AW;
            }
            let AD = AB + R;
            let AG = (AE * AD) / AF;
            let AH = (AC * AE) / AF;
            let AJ = AD / AI;
            let AK = AC / AI;
            let AL = AD - AI;
            let AN = staged[19] + (AL * AM);
            let AO = (AC * AN) + ((AC * AM) * AL);
            let AP = C + (AL * AN);
            let AQ = if AP < 1.1e-1f64 { 1.0 } else { 0.0 };
            let BD;
            let BE;
            if AQ != 0.0 {
                let AZ = ((AY * (AP - AX)) - C).exp();
                let BB = ((AO * AY) * AZ) * BA;
                let BC = AX + (BA * AZ);
                BD = BC;
                BE = BB;
            } else {
                BD = AP;
                BE = AO;
            }
            let BO;
            let BP;
            if BF != 0.0 {
                let BH = BG * BD;
                let BI = C / BH;
                let BJ = (((BE * BG) * BI) * AR) / BH;
                BO = BI;
                BP = BJ;
            } else {
                let BL = BK * BD;
                let BM = C / BL;
                let BN = (((BE * BK) * BM) * AR) / BL;
                BO = BM;
                BP = BN;
            }
            let BR = parameters[101] + (AL * BQ);
            let BS = (AC * BR) + ((AC * BQ) * AL);
            let BT = C + (AL * BR);
            let BU = if BT < 1.1e-1f64 { 1.0 } else { 0.0 };
            let BY;
            let BZ;
            if BU != 0.0 {
                let BV = ((AY * (BT - AX)) - C).exp();
                let BW = ((BS * AY) * BV) * BA;
                let BX = AX + (BA * BV);
                BY = BX;
                BZ = BW;
            } else {
                BY = BT;
                BZ = BS;
            }
            let CB = AJ.powf(CA);
            let CC = AK * (CA * (AJ.powf(staged[61])));
            let CV;
            let CW;
            let CX;
            let CY;
            if CD != 0.0 {
                let CF = (CE * (C - AJ)) / AG;
                let CJ = ((CF + (CH * (AJ.ln()))) / CI).exp();
                let CL = CK * CJ;
                let CM = (((((((AK * AR) * CE) - (AH * CF)) / AG) + ((AK * (CG / AJ)) * CH)) / CI) * CJ) * CK;
                let CN = CI * AG;
                let CP = CO / CL;
                let CQ = C + CP;
                let CR = CQ.ln();
                let CS = CN * CR;
                let CT = ((AH * CI) * CR) + (((((CM * CP) * AR) / CL) * (CG / CQ)) * CN);
                CV = CL;
                CW = CS;
                CX = CM;
                CY = CT;
            } else {
                CV = A;
                CW = A;
                CX = CU;
                CY = CU;
            }
            let DN;
            let DO;
            let DP;
            let DQ;
            if CZ != 0.0 {
                let DB = (DA * (C - AJ)) / AG;
                let DD = ((DB + (CH * (AJ.ln()))) / DC).exp();
                let DF = DE * DD;
                let DG = (((((((AK * AR) * DA) - (AH * DB)) / AG) + ((AK * (CG / AJ)) * CH)) / DC) * DD) * DE;
                let DH = DC * AG;
                let DI = CO / DF;
                let DJ = C + DI;
                let DK = DJ.ln();
                let DL = DH * DK;
                let DM = ((AH * DC) * DK) + (((((DG * DI) * AR) / DF) * (CG / DJ)) * DH);
                DN = DF;
                DO = DL;
                DP = DG;
                DQ = DM;
            } else {
                DN = A;
                DO = A;
                DP = CU;
                DQ = CU;
            }
            let DS = DR * CV;
            let DT = CX * DR;
            let DV = DU * DN;
            let DW = DP * DU;
            let DX = DS + DV;
            let DZ = DY * CV;
            let EA = CX * DY;
            let EC = EB * DN;
            let ED = DP * EB;
            let EE = DZ + EC;
            let FP;
            let FQ;
            let FR;
            let FS;
            if EF != 0.0 {
                let EG = AG / AJ;
                let EI = EH * EG;
                let EK = (EJ * AJ) / AG;
                let EL = EK.exp();
                let EN = (EM * AJ) / AG;
                let EO = EN.exp();
                let EP = EL - EO;
                let EQ = EP.ln();
                let ER = EI * EQ;
                let ET = ES * AG;
                let EU = AJ.ln();
                let EW = ((ER * AJ) - (ET * EU)) - (EV * (AJ - C));
                let EX = ((((((((AH - (AK * EG)) / AJ) * EH) * EQ) + (((((((AK * EJ) - (AH * EK)) / AG) * EL) - ((((AK * EM) - (AH * EN)) / AG) * EO)) * (CG / EP)) * EI)) * AJ) + (AK * ER)) - (((AH * ES) * EU) + ((AK * (CG / AJ)) * ET))) - (AK * EV);
                let EY = EH * AG;
                let EZ = (-EW) / AG;
                let FA = EZ.exp();
                let FC = (C + (FB * FA)).sqrt();
                let FF = FE * (C + FC);
                let FG = FF.ln();
                let FH = EW + (EY * FG);
                let FI = EX + (((AH * EH) * FG) + (((((((((EX * AR) - (AH * EZ)) / AG) * FA) * FB) * (CG / (FD * FC))) * FE) * (CG / FF)) * EY));
                let FK = FJ / FH;
                let FN = FM * (FK.powf(FL));
                let FO = ((((FI * FK) * AR) / FH) * (FL * (FK.powf(staged[62])))) * FM;
                FP = FN;
                FQ = FH;
                FR = FO;
                FS = FI;
            } else {
                FP = A;
                FQ = FJ;
                FR = CU;
                FS = CU;
            }
            let GX;
            let GY;
            let GZ;
            let HA;
            if FT != 0.0 {
                let FU = AG / AJ;
                let FV = EH * FU;
                let FX = (FW * AJ) / AG;
                let FY = FX.exp();
                let GA = (FZ * AJ) / AG;
                let GB = GA.exp();
                let GC = FY - GB;
                let GD = GC.ln();
                let GE = FV * GD;
                let GF = ES * AG;
                let GG = AJ.ln();
                let GH = ((GE * AJ) - (GF * GG)) - (EV * (AJ - C));
                let GI = ((((((((AH - (AK * FU)) / AJ) * EH) * GD) + (((((((AK * FW) - (AH * FX)) / AG) * FY) - ((((AK * FZ) - (AH * GA)) / AG) * GB)) * (CG / GC)) * FV)) * AJ) + (AK * GE)) - (((AH * ES) * GG) + ((AK * (CG / AJ)) * GF))) - (AK * EV);
                let GJ = EH * AG;
                let GK = (-GH) / AG;
                let GL = GK.exp();
                let GM = (C + (FB * GL)).sqrt();
                let GN = FE * (C + GM);
                let GO = GN.ln();
                let GP = GH + (GJ * GO);
                let GQ = GI + (((AH * EH) * GO) + (((((((((GI * AR) - (AH * GK)) / AG) * GL) * FB) * (CG / (FD * GM))) * FE) * (CG / GN)) * GJ));
                let GS = GR / GP;
                let GV = GU * (GS.powf(GT));
                let GW = ((((GQ * GS) * AR) / GP) * (GT * (GS.powf(staged[63])))) * GU;
                GX = GV;
                GY = GP;
                GZ = GW;
                HA = GQ;
            } else {
                GX = A;
                GY = GR;
                GZ = CU;
                HA = CU;
            }
            let HB = if ((C + (AL * parameters[108])) * parameters[86]) > A { 1.0 } else { 0.0 };
            let HK;
            let HL;
            let HM;
            let HN;
            let HO;
            let HP;
            if HC != 0.0 {
                let HE = parameters[105] + (AL * HD);
                let HG = HF * (C + (AL * HE));
                let HH = ((AC * HE) + ((AC * HD) * AL)) * HF;
                let HI = if HG > A { 1.0 } else { 0.0 };
                let HR;
                let HS;
                if HI != 0.0 {
                    HR = HG;
                    HS = HH;
                } else {
                    HR = A;
                    HS = CU;
                }
                let HU = HJ * (C + (HT * AL));
                let HV = (AC * HT) * HJ;
                let HW = HU * AG;
                let HX = (HV * AG) + (AH * HU);
                let HY = (-HR) / HW;
                let HZ = HY.exp();
                let IA = HZ + staged[32];
                let IB = IA.ln();
                let IC = HW * IB;
                let ID = (HX * IB) + ((((((HS * AR) - (HX * HY)) / HW) * HZ) * (CG / IA)) * HW);
                HK = HR;
                HL = HU;
                HM = IC;
                HN = HS;
                HO = HV;
                HP = ID;
            } else {
                HK = HF;
                HL = HJ;
                HM = C;
                HN = CU;
                HO = CU;
                HP = CU;
            }
            let IG;
            let IH;
            let II;
            let IJ;
            let IK;
            let IL;
            let IM;
            let IN;
            let IO;
            let IP;
            if HQ != 0.0 {
                let JD;
                let JE;
                let JF;
                let JG;
                if IE != 0.0 {
                    let IW = IV * CB;
                    let IX = IW * BD;
                    let IY = ((CC * IV) * BD) + (BE * IW);
                    let JA = IZ * CB;
                    let JB = JA * BD;
                    let JC = ((CC * IZ) * BD) + (BE * JA);
                    JD = IX;
                    JE = JB;
                    JF = IY;
                    JG = JC;
                } else {
                    JD = IV;
                    JE = IZ;
                    JF = CU;
                    JG = CU;
                }
                let JH = JF * JD;
                let JJ = JI * JE;
                let JK = ((JD * JD) + (JJ * JE)).sqrt();
                let JM = JK - (JL * JE);
                let JN = (((JH + JH) + (((JG * JI) * JE) + (JG * JJ))) * (CG / (FD * JK))) - (JG * JL);
                let JP = (JO * JM) / JE;
                let JQ = ((JN * JO) - (JG * JP)) / JE;
                let JR = JN * JM;
                let JS = JE * JE;
                let JT = JG * JE;
                let JU = (JM * JM) / JS;
                let JV = (JU + (FB * JP)).sqrt();
                let JW = ((((JR + JR) - ((JT + JT) * JU)) / JS) + (JQ * FB)) * (CG / (FD * JV));
                let JX = JE - JD;
                let JY = JG - JF;
                let JZ = C / JE;
                let KA = ((JG * JZ) * AR) / JE;
                IG = JX;
                IH = JZ;
                II = JM;
                IJ = JP;
                IK = JV;
                IL = JY;
                IM = KA;
                IN = JN;
                IO = JQ;
                IP = JW;
            } else {
                IG = IF;
                IH = A;
                II = A;
                IJ = A;
                IK = A;
                IL = CU;
                IM = CU;
                IN = CU;
                IO = CU;
                IP = CU;
            }
            let IR = IQ * IG;
            let IS = IL * IQ;
            let IU = if IR > IT { 1.0 } else { 0.0 };
            let KB;
            let KC;
            if IU != 0.0 {
                KB = IT;
                KC = CU;
            } else {
                KB = IR;
                KC = IS;
            }
            let KD = if I < A { 1.0 } else { 0.0 };
            let KN;
            let KO;
            let KP;
            let KQ;
            let KR;
            if KD != 0.0 {
                let KE = -O;
                let KF = P * AR;
                let KG = -I;
                let KH = J * AR;
                let KI = Lanes([KF[0], 0.0, KF[1]]);
                KN = KE;
                KO = KG;
                KP = KJ;
                KQ = KI;
                KR = KH;
            } else {
                let KK = -M;
                let KL = N * AR;
                let KM = Lanes([KL[0], KL[1], 0.0]);
                KN = KK;
                KO = I;
                KP = C;
                KQ = KM;
                KR = J;
            }
            let KT = if KN > KS { 1.0 } else { 0.0 };
            let LD;
            let LE;
            if KT != 0.0 {
                let KV = ((KS - KN) / KU).exp();
                let KW = C + KV;
                let KX = KS - (KU * (KW.ln()));
                let KY = (((((KQ * AR) / KU) * KV) * (CG / KW)) * KU) * AR;
                LD = KX;
                LE = KY;
            } else {
                let KZ = ((KN - KS) / KU).exp();
                let LA = C + KZ;
                let LB = KN - (KU * (LA.ln()));
                let LC = KQ - ((((KQ / KU) * KZ) * (CG / LA)) * KU);
                LD = LB;
                LE = LC;
            }
            let LJ;
            let LK;
            if BF != 0.0 {
                let LF = KS - LD;
                let LG = LE * AR;
                let LH = if KO < LF { 1.0 } else { 0.0 };
                let LP = if LH != 0.0 {
                    KO
                } else {
                    LF
                };
                let LQ = if LD < (-4e-1f64 * (LM + LP)) { 1.0 } else { 0.0 };
                let LR;
                let LS;
                if LQ != 0.0 {
                    let LU;
                    let LV;
                    if LH != 0.0 {
                        let LT = Lanes([0.0, KR[0], KR[1]]);
                        LU = KO;
                        LV = LT;
                    } else {
                        LU = LF;
                        LV = LG;
                    }
                    let LX = LW * (LM + LU);
                    let LY = LV * LW;
                    LR = LX;
                    LS = LY;
                } else {
                    LR = LD;
                    LS = LE;
                }
                LJ = LR;
                LK = LS;
            } else {
                let LI = if LD < staged[40] { 1.0 } else { 0.0 };
                let MB;
                let MC;
                if LI != 0.0 {
                    let LZ = -4e-1f64 * LM;
                    MB = LZ;
                    MC = MA;
                } else {
                    MB = LD;
                    MC = LE;
                }
                LJ = MB;
                LK = MC;
            }
            let LL = LK * EH;
            let LN = LM + (EH * LJ);
            let LO = if IH > A { 1.0 } else { 0.0 };
            let OB;
            let OC;
            if LO != 0.0 {
                let ME = MD * LN;
                let MF = (ME * LN) - LN;
                let MG = ES * MD;
                let MH = -1e0f64 + (MG * LN);
                let MI = LN / KB;
                let MJ = MD * (2.25e0f64 + MI);
                let ML = (MK * MD) / KB;
                let MM = FB * KB;
                let MN = (MM * KB) / MD;
                let MO = (((KC * FB) * KB) + (KC * MM)) / MD;
                let MP = MF * MN;
                let MQ = ((((LL * MD) * LN) + (LL * ME)) - LL) * MN;
                let MR = Lanes([MQ[0], 0.0, MQ[1], MQ[2]]) + Lanes([0.0, (MO * MF), 0.0, 0.0]);
                let MS = MH * MN;
                let MT = (LL * MG) * MN;
                let MU = Lanes([MT[0], 0.0, MT[1], MT[2]]) + Lanes([0.0, (MO * MH), 0.0, 0.0]);
                let MV = MJ * MN;
                let MW = ((((Lanes([LL[0], 0.0, LL[1], LL[2]]) - Lanes([0.0, (KC * MI), 0.0, 0.0])) / KB) * MD) * MN) + Lanes([0.0, (MO * MJ), 0.0, 0.0]);
                let MX = ML * MN;
                let MY = ((((KC * ML) * AR) / KB) * MN) + (MO * ML);
                let MZ = MX * MX;
                let NA = MY * MX;
                let NB = NA + NA;
                let NC = -MV;
                let ND = MW * AR;
                let NE = (MX * MS) - (FB * MP);
                let NF = (Lanes([0.0, (MY * MS), 0.0, 0.0]) + (MU * MX)) - (MR * FB);
                let NG = FB * MV;
                let NH = MU * MS;
                let NI = ND * NC;
                let NK = NE - ((NC * NC) * NJ);
                let NL = NF - ((NI + NI) * NJ);
                let NM = NE + (EH * NK);
                let NO = (((NG * MP) - (MS * MS)) - (MP * MZ)) - ((NC * NM) / NN);
                let NP = (((((MW * FB) * MP) + (MR * NG)) - (NH + NH)) - ((MR * MZ) + Lanes([0.0, (NB * MP), 0.0, 0.0]))) - (((ND * NM) + ((NF + (NL * EH)) * NC)) / NN);
                let NQ = NK * NK;
                let NR = NL * NK;
                let NT = (NQ * NK) / NS;
                let NU = (((NR + NR) * NK) + (NL * NQ)) / NS;
                let NW = NV * NO;
                let NX = ((NW * NO) + NT).sqrt();
                let NY = ((((NP * NV) * NO) + (NP * NW)) + NU) * (CG / (FD * NX));
                let NZ = if NO < A { 1.0 } else { 0.0 };
                let OO;
                let OP;
                let OQ;
                let OR;
                if NZ != 0.0 {
                    let OF = (OE * NO) + NX;
                    let OG = (NP * OE) + NY;
                    let OH = (-NT) / OF;
                    let OI = ((NU * AR) - (OG * OH)) / OF;
                    OO = OF;
                    OP = OH;
                    OQ = OG;
                    OR = OI;
                } else {
                    let OK = (OJ * NO) - NX;
                    let OL = (NP * OJ) - NY;
                    let OM = (-NT) / OK;
                    let ON = ((NU * AR) - (OL * OM)) / OK;
                    OO = OM;
                    OP = OK;
                    OQ = ON;
                    OR = OL;
                }
                let OT = if OO > OS { 1.0 } else { 0.0 };
                let OX;
                let OY;
                if OT != 0.0 {
                    let OU = OO.powf(NJ);
                    let OV = OQ * (NJ * (OO.powf(-6.666666666666667e-1f64)));
                    OX = OU;
                    OY = OV;
                } else {
                    let OW = if OO < -1e-6f64 { 1.0 } else { 0.0 };
                    let PG;
                    let PH;
                    if OW != 0.0 {
                        let PA = -OO;
                        let PB = -(PA.powf(NJ));
                        let PC = ((OQ * AR) * (NJ * (PA.powf(-6.666666666666667e-1f64)))) * AR;
                        PG = PB;
                        PH = PC;
                    } else {
                        let PE = PD * OO;
                        let PF = OQ * PD;
                        PG = PE;
                        PH = PF;
                    }
                    OX = PG;
                    OY = PH;
                }
                let OZ = if OP > OS { 1.0 } else { 0.0 };
                let PL;
                let PM;
                if OZ != 0.0 {
                    let PI = OP.powf(NJ);
                    let PJ = OR * (NJ * (OP.powf(-6.666666666666667e-1f64)));
                    PL = PI;
                    PM = PJ;
                } else {
                    let PK = if OP < -1e-6f64 { 1.0 } else { 0.0 };
                    let QF;
                    let QG;
                    if PK != 0.0 {
                        let QA = -OP;
                        let QB = -(QA.powf(NJ));
                        let QC = ((OR * AR) * (NJ * (QA.powf(-6.666666666666667e-1f64)))) * AR;
                        QF = QB;
                        QG = QC;
                    } else {
                        let QD = PD * OP;
                        let QE = OR * PD;
                        QF = QD;
                        QG = QE;
                    }
                    PL = QF;
                    PM = QG;
                }
                let PN = NV * MZ;
                let PO = NB * NV;
                let PP = ((PN - MV) + ((OX + PL) - (NC * NJ))).sqrt();
                let PQ = ((Lanes([0.0, PO, 0.0, 0.0]) - MW) + ((OY + PM) - (ND * NJ))) * (CG / (FD * PP));
                let PS = PQ * PP;
                let PT = ((PR * MZ) - (PP * PP)) - (EH * MV);
                let PU = (Lanes([0.0, (NB * PR), 0.0, 0.0]) - (PS + PS)) - (MW * EH);
                let PV = (((MX * MV) - (EH * MS)) - (PN * MX)) / PP;
                let PW = ((((Lanes([0.0, (MY * MV), 0.0, 0.0]) + (MW * MX)) - (MU * EH)) - Lanes([0.0, ((PO * MX) + (MY * PN)), 0.0, 0.0])) - (PQ * PV)) / PP;
                let PX = PT + PV;
                let PY = PU + PW;
                let PZ = if PX > A { 1.0 } else { 0.0 };
                let QS;
                let QT;
                if PZ != 0.0 {
                    let QH = PX.sqrt();
                    let QJ = (QI * MX) + (FE * (QH + PP));
                    let QK = Lanes([0.0, (MY * QI), 0.0, 0.0]) + (((PY * (CG / (FD * QH))) + PQ) * FE);
                    QS = QJ;
                    QT = QK;
                } else {
                    let QL = PT - PV;
                    let QM = (PU - PW) * QL;
                    let QN = ((QL * QL) + 1e-4f64).sqrt();
                    let QO = QN.sqrt();
                    let QQ = (QP * MX) + (FE * (QO - PP));
                    let QR = Lanes([0.0, (MY * QP), 0.0, 0.0]) + (((((QM + QM) * (CG / (FD * QN))) * (CG / (FD * QO))) - PQ) * FE);
                    QS = QQ;
                    QT = QR;
                }
                OB = QS;
                OC = QT;
            } else {
                let OA = if LJ > staged[42] { 1.0 } else { 0.0 };
                let RL;
                let RM;
                if OA != 0.0 {
                    let QV = QU - LJ;
                    let QW = LK * AR;
                    let QX = MD * QV;
                    let QY = QW * MD;
                    let QZ = EH * (C - (EH * QX));
                    let RA = (C - (MK * QX)).sqrt();
                    let RB = (C - (ES * QX)) + RA;
                    let RC = (QZ * QV) / RB;
                    let RD = ((((((QY * EH) * AR) * EH) * QV) + (QW * QZ)) - ((((QY * ES) * AR) + (((QY * MK) * AR) * (CG / (FD * RA)))) * RC)) / RB;
                    RL = RC;
                    RM = RD;
                } else {
                    let RE = ES * MD;
                    let RF = RE * LN;
                    let RG = LL * RE;
                    let RH = (C + RF).sqrt();
                    let RI = 4.5e0f64 * MD;
                    let RJ = ((C - RF) + RH) / RI;
                    let RK = ((RG * AR) + (RG * (CG / (FD * RH)))) / RI;
                    RL = RJ;
                    RM = RK;
                }
                let RN = Lanes([RM[0], 0.0, RM[1], RM[2]]);
                OB = RL;
                OC = RN;
            }
            let SI;
            let SJ;
            let SK;
            let SL;
            let SM;
            let SN;
            if OD != 0.0 {
                let RO = OB + staged[45];
                let RP = LN + OB;
                let RQ = Lanes([LL[0], 0.0, LL[1], LL[2]]) + OC;
                let RR = RP.sqrt();
                let RT = RS * RR;
                let RU = (RQ * (CG / (FD * RR))) * RS;
                let TY;
                let TZ;
                if LO != 0.0 {
                    let SQ = RO / IQ;
                    let SR = OC / IQ;
                    let SS = Lanes([0.0, IN, 0.0, 0.0]);
                    let ST = FE * (SQ - II);
                    let SU = ST * IH;
                    let SV = (((SR - SS) * FE) * IH) + Lanes([0.0, (IM * ST), 0.0, 0.0]);
                    let SW = FE * (SQ + II);
                    let SX = SW * IH;
                    let SY = (((SR + SS) * FE) * IH) + Lanes([0.0, (IM * SW), 0.0, 0.0]);
                    let SZ = SV * SU;
                    let TA = Lanes([0.0, IO, 0.0, 0.0]);
                    let TB = ((SU * SU) + IJ).sqrt();
                    let TC = ((SZ + SZ) + TA) * (CG / (FD * TB));
                    let TD = SY * SX;
                    let TE = ((SX * SX) + IJ).sqrt();
                    let TF = ((TD + TD) + TA) * (CG / (FD * TE));
                    let TG = SU / TB;
                    let TH = SX / TE;
                    let TI = FE * (TG + TH);
                    let TJ = (TI * IH) / IQ;
                    let TK = EH * RT;
                    let TL = C - RT;
                    let TM = TK * TL;
                    let TN = C + ((TB + TE) - IK);
                    let TO = (TJ * RO) / TN;
                    let TP = C - TO;
                    let TQ = (TM * TP) / RO;
                    let TR = TQ.sqrt();
                    let TS = (((((((RU * EH) * TL) + ((RU * AR) * TK)) * TP) + (((((((((((((SV - (TC * TG)) / TB) + ((SY - (TF * TH)) / TE)) * FE) * IH) + Lanes([0.0, (IM * TI), 0.0, 0.0])) / IQ) * RO) + (OC * TJ)) - (((TC + TF) - Lanes([0.0, IP, 0.0, 0.0])) * TO)) / TN) * AR) * TM)) - (OC * TQ)) / RO) * (CG / (FD * TR));
                    TY = TR;
                    TZ = TS;
                } else {
                    let TT = EH * RT;
                    let TU = C - RT;
                    let TV = (TT * TU) / RO;
                    let TW = TV.sqrt();
                    let TX = (((((RU * EH) * TU) + ((RU * AR) * TT)) - (OC * TV)) / RO) * (CG / (FD * TW));
                    TY = TW;
                    TZ = TX;
                }
                let UA = TY * TY;
                let UB = TZ * TY;
                let UC = (MD * RP) / UA;
                let UD = UC - RO;
                let UE = (((RQ * MD) - ((UB + UB) * UC)) / UA) - OC;
                let UG = UF + RO;
                let UH = (UF * OB) / UG;
                let UI = ((OC * UF) - (OC * UH)) / UG;
                let UJ = SA + UH;
                let UK = FB * UJ;
                let UL = UK * UJ;
                let UM = ((UI * FB) * UJ) + (UI * UK);
                let UN = EH * KO;
                let UO = UN * RO;
                let UP = (KR * EH) * RO;
                let UQ = Lanes([0.0, 0.0, UP[0], UP[1]]) + (OC * UN);
                let UR = KO - RO;
                let US = Lanes([0.0, 0.0, KR[0], KR[1]]);
                let UT = UR * UR;
                let UU = (US - OC) * UR;
                let UV = UU + UU;
                let UW = (UT + UL).sqrt();
                let UX = KO + RO;
                let UY = UX * UX;
                let UZ = (US + OC) * UX;
                let VA = UZ + UZ;
                let VB = (UY + UL).sqrt();
                let VC = UW + VB;
                let VD = UO / VC;
                let VE = (UQ - ((((UV + UM) * (CG / (FD * UW))) + ((VA + UM) * (CG / (FD * VB)))) * VD)) / VC;
                let VR;
                let VS;
                if VF != 0.0 {
                    let VG = (UF * VD) / UG;
                    let VH = ((VE * UF) - (OC * VG)) / UG;
                    let VI = SA + VG;
                    let VJ = FB * VI;
                    let VK = VJ * VI;
                    let VL = ((VH * FB) * VI) + (VH * VJ);
                    let VM = (UT + VK).sqrt();
                    let VN = (UY + VK).sqrt();
                    let VO = VM + VN;
                    let VP = UO / VO;
                    let VQ = (UQ - ((((UV + VL) * (CG / (FD * VM))) + ((VA + VL) * (CG / (FD * VN)))) * VP)) / VO;
                    VR = VP;
                    VS = VQ;
                } else {
                    VR = VD;
                    VS = VE;
                }
                let VT = (UD + VR).sqrt();
                let VU = C - (TY * VT);
                let VV = ((TZ * VT) + (((UE + VS) * (CG / (FD * VT))) * TY)) * AR;
                let WL;
                let WM;
                if LO != 0.0 {
                    let VW = VR / IQ;
                    let VX = VS / IQ;
                    let VY = Lanes([0.0, IN, 0.0, 0.0]);
                    let VZ = FE * (VW - II);
                    let WA = VZ * IH;
                    let WB = FE * (VW + II);
                    let WC = WB * IH;
                    let WD = ((((VX - VY) * FE) * IH) + Lanes([0.0, (IM * VZ), 0.0, 0.0])) * WA;
                    let WE = Lanes([0.0, IO, 0.0, 0.0]);
                    let WF = ((WA * WA) + IJ).sqrt();
                    let WG = ((((VX + VY) * FE) * IH) + Lanes([0.0, (IM * WB), 0.0, 0.0])) * WC;
                    let WH = ((WC * WC) + IJ).sqrt();
                    let WI = (WF + WH) - IK;
                    let WJ = ((((WD + WD) + WE) * (CG / (FD * WF))) + (((WG + WG) + WE) * (CG / (FD * WH)))) - Lanes([0.0, IP, 0.0, 0.0]);
                    WL = WI;
                    WM = WJ;
                } else {
                    WL = A;
                    WM = WK;
                }
                SI = VU;
                SJ = WL;
                SK = VR;
                SL = VV;
                SM = WM;
                SN = VS;
            } else {
                let RV = EH * KO;
                let RW = (KR * EH) * OB;
                let RX = KO - OB;
                let RY = Lanes([0.0, 0.0, KR[0], KR[1]]);
                let RZ = (RY - OC) * RX;
                let SB = ((RX * RX) + SA).sqrt();
                let SC = KO + OB;
                let SD = (RY + OC) * SC;
                let SE = ((SC * SC) + SA).sqrt();
                let SF = SB + SE;
                let SG = (RV * OB) / SF;
                let SH = ((Lanes([0.0, 0.0, RW[0], RW[1]]) + (OC * RV)) - ((((RZ + RZ) * (CG / (FD * SB))) + ((SD + SD) * (CG / (FD * SE)))) * SG)) / SF;
                let XB;
                let XC;
                if LO != 0.0 {
                    let WN = SG / IQ;
                    let WO = SH / IQ;
                    let WP = Lanes([0.0, IN, 0.0, 0.0]);
                    let WQ = FE * (WN - II);
                    let WR = WQ * IH;
                    let WS = FE * (WN + II);
                    let WT = WS * IH;
                    let WU = ((((WO - WP) * FE) * IH) + Lanes([0.0, (IM * WQ), 0.0, 0.0])) * WR;
                    let WV = Lanes([0.0, IO, 0.0, 0.0]);
                    let WW = ((WR * WR) + IJ).sqrt();
                    let WX = ((((WO + WP) * FE) * IH) + Lanes([0.0, (IM * WS), 0.0, 0.0])) * WT;
                    let WY = ((WT * WT) + IJ).sqrt();
                    let WZ = (WW + WY) - IK;
                    let XA = ((((WU + WU) + WV) * (CG / (FD * WW))) + (((WX + WX) + WV) * (CG / (FD * WY)))) - Lanes([0.0, IP, 0.0, 0.0]);
                    XB = WZ;
                    XC = XA;
                } else {
                    XB = A;
                    XC = WK;
                }
                let XD = (LN + SG).sqrt();
                let XE = C - (RS * XD);
                let XF = (((Lanes([LL[0], 0.0, LL[1], LL[2]]) + SH) * (CG / (FD * XD))) * RS) * AR;
                SI = XE;
                SJ = XB;
                SK = SG;
                SL = XF;
                SM = XC;
                SN = SH;
            }
            let SP = if SI < SO { 1.0 } else { 0.0 };
            let XG;
            let XH;
            if SP != 0.0 {
                XG = SO;
                XH = WK;
            } else {
                XG = SI;
                XH = SL;
            }
            let XI = C + SJ;
            let XJ = (BO * XG) / XI;
            let XK = KP * XJ;
            let XL = XK * SK;
            let XM = (((((Lanes([0.0, (BP * XG), 0.0, 0.0]) + (XH * BO)) - (SM * XJ)) / XI) * KP) * SK) + (SN * XK);
            let XN = if DX > A { 1.0 } else { 0.0 };
            let XQ;
            let XR;
            if XN != 0.0 {
                let XO = if DS > A { 1.0 } else { 0.0 };
                let XX;
                let XY;
                if XO != 0.0 {
                    let XT = CI * AG;
                    let XU = C / XT;
                    let XV = (((AH * CI) * XU) * AR) / XT;
                    let XW = if M < CW { 1.0 } else { 0.0 };
                    let YI;
                    let YJ;
                    if XW != 0.0 {
                        let YA = N * XU;
                        let YB = (M * XU).exp();
                        let YC = (Lanes([YA[0], 0.0, YA[1]]) + Lanes([0.0, (XV * M), 0.0])) * YB;
                        YI = YB;
                        YJ = YC;
                    } else {
                        let YD = (CW * XU).exp();
                        let YE = M - CW;
                        let YF = C + (YE * XU);
                        let YG = YD * YF;
                        let YH = Lanes([0.0, ((((CY * XU) + (XV * CW)) * YD) * YF), 0.0]) + ((((Lanes([N[0], 0.0, N[1]]) - Lanes([0.0, CY, 0.0])) * XU) + Lanes([0.0, (XV * YE), 0.0])) * YD);
                        YI = YG;
                        YJ = YH;
                    }
                    let YK = YI - C;
                    let YL = DS * YK;
                    let YM = Lanes([0.0, (DT * YK), 0.0]) + (YJ * DS);
                    XX = YL;
                    XY = YM;
                } else {
                    XX = A;
                    XY = XP;
                }
                let XZ = if DV > A { 1.0 } else { 0.0 };
                let YR;
                let YS;
                if XZ != 0.0 {
                    let YN = DC * AG;
                    let YO = C / YN;
                    let YP = (((AH * DC) * YO) * AR) / YN;
                    let YQ = if M < DO { 1.0 } else { 0.0 };
                    let ZE;
                    let ZF;
                    if YQ != 0.0 {
                        let YW = N * YO;
                        let YX = (M * YO).exp();
                        let YY = (Lanes([YW[0], 0.0, YW[1]]) + Lanes([0.0, (YP * M), 0.0])) * YX;
                        ZE = YX;
                        ZF = YY;
                    } else {
                        let YZ = (DO * YO).exp();
                        let ZA = M - DO;
                        let ZB = C + (ZA * YO);
                        let ZC = YZ * ZB;
                        let ZD = Lanes([0.0, ((((DQ * YO) + (YP * DO)) * YZ) * ZB), 0.0]) + ((((Lanes([N[0], 0.0, N[1]]) - Lanes([0.0, DQ, 0.0])) * YO) + Lanes([0.0, (YP * ZA), 0.0])) * YZ);
                        ZE = ZC;
                        ZF = ZD;
                    }
                    let ZG = ZE - C;
                    let ZH = DV * ZG;
                    let ZI = Lanes([0.0, (DW * ZG), 0.0]) + (ZF * DV);
                    YR = ZH;
                    YS = ZI;
                } else {
                    YR = A;
                    YS = XP;
                }
                let YT = XX + YR;
                let YU = XY + YS;
                let YV = if HK > A { 1.0 } else { 0.0 };
                let ZR;
                let ZS;
                if YV != 0.0 {
                    let ZJ = -HK;
                    let ZK = HN * AR;
                    let ZL = ZJ - M;
                    let ZM = Lanes([0.0, ZK, 0.0]) - Lanes([N[0], 0.0, N[1]]);
                    let ZN = HL * AG;
                    let ZO = C / ZN;
                    let ZP = ((((HO * AG) + (AH * HL)) * ZO) * AR) / ZN;
                    let ZQ = if ZL < HM { 1.0 } else { 0.0 };
                    let AAD;
                    let AAE;
                    if ZQ != 0.0 {
                        let ZW = (ZL * ZO).exp();
                        let ZX = ((ZM * ZO) + Lanes([0.0, (ZP * ZL), 0.0])) * ZW;
                        AAD = ZW;
                        AAE = ZX;
                    } else {
                        let ZY = (HM * ZO).exp();
                        let ZZ = ZL - HM;
                        let AAA = C + (ZZ * ZO);
                        let AAB = ZY * AAA;
                        let AAC = Lanes([0.0, ((((HP * ZO) + (ZP * HM)) * ZY) * AAA), 0.0]) + ((((ZM - Lanes([0.0, HP, 0.0])) * ZO) + Lanes([0.0, (ZP * ZZ), 0.0])) * ZY);
                        AAD = AAB;
                        AAE = AAC;
                    }
                    let AAG = -AAF;
                    let AAH = (ZJ * ZO).exp();
                    let AAI = AAG * (AAD - AAH);
                    let AAJ = (AAE - Lanes([0.0, (((ZK * ZO) + (ZP * ZJ)) * AAH), 0.0])) * AAG;
                    ZR = AAI;
                    ZS = AAJ;
                } else {
                    ZR = A;
                    ZS = XP;
                }
                let ZT = N * B;
                let ZU = (YT + ZR) + (B * M);
                let ZV = (YU + ZS) + Lanes([ZT[0], 0.0, ZT[1]]);
                XQ = ZU;
                XR = ZV;
            } else {
                XQ = A;
                XR = XP;
            }
            let XS = if EE > A { 1.0 } else { 0.0 };
            let AAM;
            let AAN;
            if XS != 0.0 {
                let AAK = if DZ > A { 1.0 } else { 0.0 };
                let ABQ;
                let ABR;
                if AAK != 0.0 {
                    let ABM = CI * AG;
                    let ABN = C / ABM;
                    let ABO = (((AH * CI) * ABN) * AR) / ABM;
                    let ABP = if O < CW { 1.0 } else { 0.0 };
                    let ACB;
                    let ACC;
                    if ABP != 0.0 {
                        let ABT = P * ABN;
                        let ABU = (O * ABN).exp();
                        let ABV = (Lanes([ABT[0], 0.0, ABT[1]]) + Lanes([0.0, (ABO * O), 0.0])) * ABU;
                        ACB = ABU;
                        ACC = ABV;
                    } else {
                        let ABW = (CW * ABN).exp();
                        let ABX = O - CW;
                        let ABY = C + (ABX * ABN);
                        let ABZ = ABW * ABY;
                        let ACA = Lanes([0.0, ((((CY * ABN) + (ABO * CW)) * ABW) * ABY), 0.0]) + ((((Lanes([P[0], 0.0, P[1]]) - Lanes([0.0, CY, 0.0])) * ABN) + Lanes([0.0, (ABO * ABX), 0.0])) * ABW);
                        ACB = ABZ;
                        ACC = ACA;
                    }
                    let ACD = ACB - C;
                    let ACE = DZ * ACD;
                    let ACF = Lanes([0.0, (EA * ACD), 0.0]) + (ACC * DZ);
                    ABQ = ACE;
                    ABR = ACF;
                } else {
                    ABQ = A;
                    ABR = AAL;
                }
                let ABS = if EC > A { 1.0 } else { 0.0 };
                let ACK;
                let ACL;
                if ABS != 0.0 {
                    let ACG = DC * AG;
                    let ACH = C / ACG;
                    let ACI = (((AH * DC) * ACH) * AR) / ACG;
                    let ACJ = if O < DO { 1.0 } else { 0.0 };
                    let ACX;
                    let ACY;
                    if ACJ != 0.0 {
                        let ACP = P * ACH;
                        let ACQ = (O * ACH).exp();
                        let ACR = (Lanes([ACP[0], 0.0, ACP[1]]) + Lanes([0.0, (ACI * O), 0.0])) * ACQ;
                        ACX = ACQ;
                        ACY = ACR;
                    } else {
                        let ACS = (DO * ACH).exp();
                        let ACT = O - DO;
                        let ACU = C + (ACT * ACH);
                        let ACV = ACS * ACU;
                        let ACW = Lanes([0.0, ((((DQ * ACH) + (ACI * DO)) * ACS) * ACU), 0.0]) + ((((Lanes([P[0], 0.0, P[1]]) - Lanes([0.0, DQ, 0.0])) * ACH) + Lanes([0.0, (ACI * ACT), 0.0])) * ACS);
                        ACX = ACV;
                        ACY = ACW;
                    }
                    let ACZ = ACX - C;
                    let ADA = EC * ACZ;
                    let ADB = Lanes([0.0, (ED * ACZ), 0.0]) + (ACY * EC);
                    ACK = ADA;
                    ACL = ADB;
                } else {
                    ACK = A;
                    ACL = AAL;
                }
                let ACM = ABQ + ACK;
                let ACN = ABR + ACL;
                let ACO = if HK > A { 1.0 } else { 0.0 };
                let ADK;
                let ADL;
                if ACO != 0.0 {
                    let ADC = -HK;
                    let ADD = HN * AR;
                    let ADE = ADC - O;
                    let ADF = Lanes([0.0, ADD, 0.0]) - Lanes([P[0], 0.0, P[1]]);
                    let ADG = HL * AG;
                    let ADH = C / ADG;
                    let ADI = ((((HO * AG) + (AH * HL)) * ADH) * AR) / ADG;
                    let ADJ = if ADE < HM { 1.0 } else { 0.0 };
                    let ADW;
                    let ADX;
                    if ADJ != 0.0 {
                        let ADP = (ADE * ADH).exp();
                        let ADQ = ((ADF * ADH) + Lanes([0.0, (ADI * ADE), 0.0])) * ADP;
                        ADW = ADP;
                        ADX = ADQ;
                    } else {
                        let ADR = (HM * ADH).exp();
                        let ADS = ADE - HM;
                        let ADT = C + (ADS * ADH);
                        let ADU = ADR * ADT;
                        let ADV = Lanes([0.0, ((((HP * ADH) + (ADI * HM)) * ADR) * ADT), 0.0]) + ((((ADF - Lanes([0.0, HP, 0.0])) * ADH) + Lanes([0.0, (ADI * ADS), 0.0])) * ADR);
                        ADW = ADU;
                        ADX = ADV;
                    }
                    let ADY = -AAF;
                    let ADZ = (ADC * ADH).exp();
                    let AEA = ADY * (ADW - ADZ);
                    let AEB = (ADX - Lanes([0.0, (((ADD * ADH) + (ADI * ADC)) * ADZ), 0.0])) * ADY;
                    ADK = AEA;
                    ADL = AEB;
                } else {
                    ADK = A;
                    ADL = AAL;
                }
                let ADM = P * B;
                let ADN = (ACM + ADK) + (B * O);
                let ADO = (ACN + ADL) + Lanes([ADM[0], 0.0, ADM[1]]);
                AAM = ADN;
                AAN = ADO;
            } else {
                AAM = A;
                AAN = AAL;
            }
            let AAO = J * XL;
            let AAP = N * XQ;
            let AAQ = (XR * M) + Lanes([AAP[0], 0.0, AAP[1]]);
            let AAR = P * AAM;
            let AAS = (AAN * O) + Lanes([AAR[0], 0.0, AAR[1]]);
            let AAT = (((XM * I) + Lanes([0.0, 0.0, AAO[0], AAO[1]])) + Lanes([AAQ[0], AAQ[1], AAQ[2], 0.0])) + Lanes([AAS[0], AAS[1], 0.0, AAS[2]]);
            let AAU = node_potentials[0] - E;
            let AAW = Lanes([AAV, 0.0]) - Lanes([0.0, G]);
            let AAZ = AAW * AAX;
            let ABA = Lanes([0.0, 0.0, (AAY * AAU)]) + Lanes([AAZ[0], AAZ[1], 0.0]);
            let ABB = Lanes([0.0, AAT[0], AAT[1], AAT[2], AAT[3], 0.0]) + Lanes([ABA[0], 0.0, 0.0, ABA[1], 0.0, ABA[2]]);
            let ABC = node_potentials[2] - D;
            let ABE = Lanes([ABD, 0.0]) - Lanes([0.0, F]);
            let ABH = ABE * ABF;
            let ABI = Lanes([0.0, 0.0, (ABG * ABC)]) + Lanes([ABH[0], ABH[1], 0.0]);
            let ABJ = ((((XL * I) + (XQ * M)) + (AAM * O)) + (AAX * AAU)) + (ABF * ABC);
            let ABK = Lanes([ABB[0], ABB[1], 0.0, ABB[2], ABB[3], ABB[4], ABB[5], 0.0]) + Lanes([0.0, 0.0, ABI[0], 0.0, 0.0, ABI[1], 0.0, ABI[2]]);
            let AEJ;
            let AEK;
            let AEL;
            let AEM;
            if ABL != 0.0 {
                let AEC = -ABJ;
                let AED = ABK * AR;
                let AEZ;
                let AFA;
                if AEE != 0.0 {
                    let AEX = AEW * Q;
                    let AEY = W * AEW;
                    AEZ = AEX;
                    AFA = AEY;
                } else {
                    let AFN;
                    let AFO;
                    if AFB != 0.0 {
                        let AFD = C + (Q / AFC);
                        let AFG = (AFF * ((AFD.powf(AFE)) - C)) / AFE;
                        let AFH = (((W / AFC) * (AFE * (AFD.powf(staged[64])))) * AFF) / AFE;
                        AFN = AFG;
                        AFO = AFH;
                    } else {
                        let AFI = AEW * Q;
                        let AFK = C + ((AFJ * Q) / AFC);
                        let AFL = AFI * AFK;
                        let AFM = ((W * AEW) * AFK) + (((W * AFJ) / AFC) * AFI);
                        AFN = AFL;
                        AFO = AFM;
                    }
                    AEZ = AFN;
                    AFA = AFO;
                }
                AEJ = AEZ;
                AEK = AEC;
                AEL = AFA;
                AEM = AED;
            } else {
                let AEG = AEF * Q;
                let AEH = W * AEF;
                AEJ = AEG;
                AEK = A;
                AEL = AEH;
                AEM = AEI;
            }
            let AEN = H * XL;
            let AEO = XM * H;
            let AEP = H * XQ;
            let AEQ = XR * H;
            let AER = H * AAM;
            let AES = AAN * H;
            let AEV = if ((AEN / AET).abs()) > AEU { 1.0 } else { 0.0 };
            let AFP = if ((AEP / AET).abs()) > AEU { 1.0 } else { 0.0 };
            let AFR = if (M.abs()) > AFQ { 1.0 } else { 0.0 };
            let AFS = if ((AER / AET).abs()) > AEU { 1.0 } else { 0.0 };
            let AFT = if (O.abs()) > AFQ { 1.0 } else { 0.0 };
            let AFV;
            let AFW;
            if AFU != 0.0 {
                let AGE;
                let AGF;
                if BF != 0.0 {
                    let AFY = M + QU;
                    let AFZ = N * AFY;
                    let AGB = ((AFY * AFY) + AGA).sqrt();
                    let AGC = FE * ((M - QU) + AGB);
                    let AGD = (N + ((AFZ + AFZ) * (CG / (FD * AGB)))) * FE;
                    AGE = AGC;
                    AGF = AGD;
                } else {
                    AGE = M;
                    AGF = N;
                }
                let AGG = DR * FP;
                let AGH = FR * DR;
                let AGI = DU * GX;
                let AGJ = GZ * DU;
                let AGK = if AGG > A { 1.0 } else { 0.0 };
                let AGS;
                let AGT;
                if AGK != 0.0 {
                    let AGL = -FQ;
                    let AGM = FS * AR;
                    let AGO = AGL * AGN;
                    let AGP = AGM * AGN;
                    let AGR = if AGQ <= A { 1.0 } else { 0.0 };
                    let AIA;
                    let AIB;
                    if AGR != 0.0 {
                        let AGV = AGE + AGO;
                        let AGW = Lanes([AGF[0], 0.0, AGF[1]]);
                        let AGX = AGW + Lanes([0.0, AGP, 0.0]);
                        let AGY = if AGV > A { 1.0 } else { 0.0 };
                        let AIU;
                        let AIV;
                        let AIW;
                        let AIX;
                        if AGY != 0.0 {
                            let AIC = C - AGN;
                            let AID = AIC.powf((-FL));
                            let AIE = C - (AID * AIC);
                            let AIF = C - FL;
                            let AIG = (FQ * AIE) / AIF;
                            let AIH = FE * FL;
                            let AII = FQ * AIC;
                            let AIJ = (AIH * AGV) / AII;
                            let AIK = C + AIJ;
                            let AIL = (AGV * AIK) * AID;
                            let AIM = ((AGX * AIK) + ((((AGX * AIH) - Lanes([0.0, ((FS * AIC) * AIJ), 0.0])) / AII) * AGV)) * AID;
                            let AIN = Lanes([0.0, ((FS * AIE) / AIF), 0.0]);
                            AIU = AIG;
                            AIV = AIL;
                            AIW = AIN;
                            AIX = AIM;
                        } else {
                            let AIO = AGE / FQ;
                            let AIP = C - AIO;
                            let AIQ = C - FL;
                            let AIR = C - (AIP.powf(AIQ));
                            let AIS = (FQ * AIR) / AIQ;
                            let AIT = (Lanes([0.0, (FS * AIR), 0.0]) + ((((((AGW - Lanes([0.0, (FS * AIO), 0.0])) / FQ) * AR) * (AIQ * (AIP.powf((AIQ - CG))))) * AR) * FQ)) / AIQ;
                            AIU = AIS;
                            AIV = A;
                            AIW = AIT;
                            AIX = XP;
                        }
                        let AIY = AIU + AIV;
                        let AIZ = AIW + AIX;
                        AIA = AIY;
                        AIB = AIZ;
                    } else {
                        let AGZ = AGP * AGO;
                        let AHA = (FB * AGQ) * AGQ;
                        let AHB = ((AGO * AGO) + AHA).sqrt();
                        let AHD = AGE + AGO;
                        let AHE = Lanes([AGF[0], 0.0, AGF[1]]);
                        let AHF = Lanes([0.0, AGP, 0.0]);
                        let AHG = AHE + AHF;
                        let AHH = AHG * AHD;
                        let AHI = ((AHD * AHD) + AHA).sqrt();
                        let AHJ = (FE * (AHD - AHI)) - AGO;
                        let AHK = ((AHG - ((AHH + AHH) * (CG / (FD * AHI)))) * FE) - AHF;
                        let AHL = AHJ / FQ;
                        let AHM = C - AHL;
                        let AHN = C - FL;
                        let AHO = AHM.powf(AHN);
                        let AHP = C - AGN;
                        let AHQ = AHP.powf((-FL));
                        let AHR = (AGE - AHJ) + (AHC * (AGO + AHB));
                        let AHS = (AHE - AHK) + Lanes([0.0, ((AGP + ((AGZ + AGZ) * (CG / (FD * AHB)))) * AHC), 0.0]);
                        let AHT = AHQ * AHR;
                        let AHU = FE * FL;
                        let AHV = FQ * AHP;
                        let AHW = (AHU * AHR) / AHV;
                        let AHX = C + AHW;
                        let AHY = ((AGL * AHO) / AHN) + (AHT * AHX);
                        let AHZ = ((Lanes([0.0, (AGM * AHO), 0.0]) + (((((AHK - Lanes([0.0, (FS * AHL), 0.0])) / FQ) * AR) * (AHN * (AHM.powf((AHN - CG))))) * AGL)) / AHN) + (((AHS * AHQ) * AHX) + ((((AHS * AHU) - Lanes([0.0, ((FS * AHP) * AHW), 0.0])) / AHV) * AHT));
                        AIA = AHY;
                        AIB = AHZ;
                    }
                    AGS = AIA;
                    AGT = AIB;
                } else {
                    AGS = A;
                    AGT = XP;
                }
                let AGU = if AGI > A { 1.0 } else { 0.0 };
                let AJG;
                let AJH;
                if AGU != 0.0 {
                    let AJA = -GY;
                    let AJB = HA * AR;
                    let AJC = AJA * AGN;
                    let AJD = AJB * AGN;
                    let AJF = if AJE <= A { 1.0 } else { 0.0 };
                    let AKP;
                    let AKQ;
                    if AJF != 0.0 {
                        let AJK = AGE + AJC;
                        let AJL = Lanes([AGF[0], 0.0, AGF[1]]);
                        let AJM = AJL + Lanes([0.0, AJD, 0.0]);
                        let AJN = if AJK > A { 1.0 } else { 0.0 };
                        let ALJ;
                        let ALK;
                        let ALL;
                        let ALM;
                        if AJN != 0.0 {
                            let AKR = C - AGN;
                            let AKS = AKR.powf((-GT));
                            let AKT = C - (AKS * AKR);
                            let AKU = C - GT;
                            let AKV = (GY * AKT) / AKU;
                            let AKW = FE * GT;
                            let AKX = GY * AKR;
                            let AKY = (AKW * AJK) / AKX;
                            let AKZ = C + AKY;
                            let ALA = (AJK * AKZ) * AKS;
                            let ALB = ((AJM * AKZ) + ((((AJM * AKW) - Lanes([0.0, ((HA * AKR) * AKY), 0.0])) / AKX) * AJK)) * AKS;
                            let ALC = Lanes([0.0, ((HA * AKT) / AKU), 0.0]);
                            ALJ = AKV;
                            ALK = ALA;
                            ALL = ALC;
                            ALM = ALB;
                        } else {
                            let ALD = AGE / GY;
                            let ALE = C - ALD;
                            let ALF = C - GT;
                            let ALG = C - (ALE.powf(ALF));
                            let ALH = (GY * ALG) / ALF;
                            let ALI = (Lanes([0.0, (HA * ALG), 0.0]) + ((((((AJL - Lanes([0.0, (HA * ALD), 0.0])) / GY) * AR) * (ALF * (ALE.powf((ALF - CG))))) * AR) * GY)) / ALF;
                            ALJ = ALH;
                            ALK = A;
                            ALL = ALI;
                            ALM = XP;
                        }
                        let ALN = ALJ + ALK;
                        let ALO = ALL + ALM;
                        AKP = ALN;
                        AKQ = ALO;
                    } else {
                        let AJO = AJD * AJC;
                        let AJP = (FB * AJE) * AJE;
                        let AJQ = ((AJC * AJC) + AJP).sqrt();
                        let AJS = AGE + AJC;
                        let AJT = Lanes([AGF[0], 0.0, AGF[1]]);
                        let AJU = Lanes([0.0, AJD, 0.0]);
                        let AJV = AJT + AJU;
                        let AJW = AJV * AJS;
                        let AJX = ((AJS * AJS) + AJP).sqrt();
                        let AJY = (FE * (AJS - AJX)) - AJC;
                        let AJZ = ((AJV - ((AJW + AJW) * (CG / (FD * AJX)))) * FE) - AJU;
                        let AKA = AJY / GY;
                        let AKB = C - AKA;
                        let AKC = C - GT;
                        let AKD = AKB.powf(AKC);
                        let AKE = C - AGN;
                        let AKF = AKE.powf((-GT));
                        let AKG = (AGE - AJY) + (AJR * (AJC + AJQ));
                        let AKH = (AJT - AJZ) + Lanes([0.0, ((AJD + ((AJO + AJO) * (CG / (FD * AJQ)))) * AJR), 0.0]);
                        let AKI = AKF * AKG;
                        let AKJ = FE * GT;
                        let AKK = GY * AKE;
                        let AKL = (AKJ * AKG) / AKK;
                        let AKM = C + AKL;
                        let AKN = ((AJA * AKD) / AKC) + (AKI * AKM);
                        let AKO = ((Lanes([0.0, (AJB * AKD), 0.0]) + (((((AJZ - Lanes([0.0, (HA * AKA), 0.0])) / GY) * AR) * (AKC * (AKB.powf((AKC - CG))))) * AJA)) / AKC) + (((AKH * AKF) * AKM) + ((((AKH * AKJ) - Lanes([0.0, ((HA * AKE) * AKL), 0.0])) / AKK) * AKI));
                        AKP = AKN;
                        AKQ = AKO;
                    }
                    AJG = AKP;
                    AJH = AKQ;
                } else {
                    AJG = A;
                    AJH = XP;
                }
                let AJI = (AGG * AGS) + (AGI * AJG);
                let AJJ = (Lanes([0.0, (AGH * AGS), 0.0]) + (AGT * AGG)) + (Lanes([0.0, (AGJ * AJG), 0.0]) + (AJH * AGI));
                AFV = AJI;
                AFW = AJJ;
            } else {
                AFV = A;
                AFW = XP;
            }
            let ALP;
            let ALQ;
            if AFX != 0.0 {
                let AMI;
                let AMJ;
                if BF != 0.0 {
                    let AMD = O + QU;
                    let AME = P * AMD;
                    let AMF = ((AMD * AMD) + AGA).sqrt();
                    let AMG = FE * ((O - QU) + AMF);
                    let AMH = (P + ((AME + AME) * (CG / (FD * AMF)))) * FE;
                    AMI = AMG;
                    AMJ = AMH;
                } else {
                    AMI = O;
                    AMJ = P;
                }
                let AMK = DY * FP;
                let AML = FR * DY;
                let AMM = EB * GX;
                let AMN = GZ * EB;
                let AMO = if AMK > A { 1.0 } else { 0.0 };
                let AMU;
                let AMV;
                if AMO != 0.0 {
                    let AMP = -FQ;
                    let AMQ = FS * AR;
                    let AMR = AMP * AGN;
                    let AMS = AMQ * AGN;
                    let AMT = if AGQ <= A { 1.0 } else { 0.0 };
                    let AOC;
                    let AOD;
                    if AMT != 0.0 {
                        let AMX = AMI + AMR;
                        let AMY = Lanes([AMJ[0], 0.0, AMJ[1]]);
                        let AMZ = AMY + Lanes([0.0, AMS, 0.0]);
                        let ANA = if AMX > A { 1.0 } else { 0.0 };
                        let AOW;
                        let AOX;
                        let AOY;
                        let AOZ;
                        if ANA != 0.0 {
                            let AOE = C - AGN;
                            let AOF = AOE.powf((-FL));
                            let AOG = C - (AOF * AOE);
                            let AOH = C - FL;
                            let AOI = (FQ * AOG) / AOH;
                            let AOJ = FE * FL;
                            let AOK = FQ * AOE;
                            let AOL = (AOJ * AMX) / AOK;
                            let AOM = C + AOL;
                            let AON = (AMX * AOM) * AOF;
                            let AOO = ((AMZ * AOM) + ((((AMZ * AOJ) - Lanes([0.0, ((FS * AOE) * AOL), 0.0])) / AOK) * AMX)) * AOF;
                            let AOP = Lanes([0.0, ((FS * AOG) / AOH), 0.0]);
                            AOW = AOI;
                            AOX = AON;
                            AOY = AOP;
                            AOZ = AOO;
                        } else {
                            let AOQ = AMI / FQ;
                            let AOR = C - AOQ;
                            let AOS = C - FL;
                            let AOT = C - (AOR.powf(AOS));
                            let AOU = (FQ * AOT) / AOS;
                            let AOV = (Lanes([0.0, (FS * AOT), 0.0]) + ((((((AMY - Lanes([0.0, (FS * AOQ), 0.0])) / FQ) * AR) * (AOS * (AOR.powf((AOS - CG))))) * AR) * FQ)) / AOS;
                            AOW = AOU;
                            AOX = A;
                            AOY = AOV;
                            AOZ = AAL;
                        }
                        let APA = AOW + AOX;
                        let APB = AOY + AOZ;
                        AOC = APA;
                        AOD = APB;
                    } else {
                        let ANB = AMS * AMR;
                        let ANC = (FB * AGQ) * AGQ;
                        let AND = ((AMR * AMR) + ANC).sqrt();
                        let ANF = AMI + AMR;
                        let ANG = Lanes([AMJ[0], 0.0, AMJ[1]]);
                        let ANH = Lanes([0.0, AMS, 0.0]);
                        let ANI = ANG + ANH;
                        let ANJ = ANI * ANF;
                        let ANK = ((ANF * ANF) + ANC).sqrt();
                        let ANL = (FE * (ANF - ANK)) - AMR;
                        let ANM = ((ANI - ((ANJ + ANJ) * (CG / (FD * ANK)))) * FE) - ANH;
                        let ANN = ANL / FQ;
                        let ANO = C - ANN;
                        let ANP = C - FL;
                        let ANQ = ANO.powf(ANP);
                        let ANR = C - AGN;
                        let ANS = ANR.powf((-FL));
                        let ANT = (AMI - ANL) + (ANE * (AMR + AND));
                        let ANU = (ANG - ANM) + Lanes([0.0, ((AMS + ((ANB + ANB) * (CG / (FD * AND)))) * ANE), 0.0]);
                        let ANV = ANS * ANT;
                        let ANW = FE * FL;
                        let ANX = FQ * ANR;
                        let ANY = (ANW * ANT) / ANX;
                        let ANZ = C + ANY;
                        let AOA = ((AMP * ANQ) / ANP) + (ANV * ANZ);
                        let AOB = ((Lanes([0.0, (AMQ * ANQ), 0.0]) + (((((ANM - Lanes([0.0, (FS * ANN), 0.0])) / FQ) * AR) * (ANP * (ANO.powf((ANP - CG))))) * AMP)) / ANP) + (((ANU * ANS) * ANZ) + ((((ANU * ANW) - Lanes([0.0, ((FS * ANR) * ANY), 0.0])) / ANX) * ANV));
                        AOC = AOA;
                        AOD = AOB;
                    }
                    AMU = AOC;
                    AMV = AOD;
                } else {
                    AMU = A;
                    AMV = AAL;
                }
                let AMW = if AMM > A { 1.0 } else { 0.0 };
                let APH;
                let API;
                if AMW != 0.0 {
                    let APC = -GY;
                    let APD = HA * AR;
                    let APE = APC * AGN;
                    let APF = APD * AGN;
                    let APG = if AJE <= A { 1.0 } else { 0.0 };
                    let AQQ;
                    let AQR;
                    if APG != 0.0 {
                        let APL = AMI + APE;
                        let APM = Lanes([AMJ[0], 0.0, AMJ[1]]);
                        let APN = APM + Lanes([0.0, APF, 0.0]);
                        let APO = if APL > A { 1.0 } else { 0.0 };
                        let ARK;
                        let ARL;
                        let ARM;
                        let ARN;
                        if APO != 0.0 {
                            let AQS = C - AGN;
                            let AQT = AQS.powf((-GT));
                            let AQU = C - (AQT * AQS);
                            let AQV = C - GT;
                            let AQW = (GY * AQU) / AQV;
                            let AQX = FE * GT;
                            let AQY = GY * AQS;
                            let AQZ = (AQX * APL) / AQY;
                            let ARA = C + AQZ;
                            let ARB = (APL * ARA) * AQT;
                            let ARC = ((APN * ARA) + ((((APN * AQX) - Lanes([0.0, ((HA * AQS) * AQZ), 0.0])) / AQY) * APL)) * AQT;
                            let ARD = Lanes([0.0, ((HA * AQU) / AQV), 0.0]);
                            ARK = AQW;
                            ARL = ARB;
                            ARM = ARD;
                            ARN = ARC;
                        } else {
                            let ARE = AMI / GY;
                            let ARF = C - ARE;
                            let ARG = C - GT;
                            let ARH = C - (ARF.powf(ARG));
                            let ARI = (GY * ARH) / ARG;
                            let ARJ = (Lanes([0.0, (HA * ARH), 0.0]) + ((((((APM - Lanes([0.0, (HA * ARE), 0.0])) / GY) * AR) * (ARG * (ARF.powf((ARG - CG))))) * AR) * GY)) / ARG;
                            ARK = ARI;
                            ARL = A;
                            ARM = ARJ;
                            ARN = AAL;
                        }
                        let ARO = ARK + ARL;
                        let ARP = ARM + ARN;
                        AQQ = ARO;
                        AQR = ARP;
                    } else {
                        let APP = APF * APE;
                        let APQ = (FB * AJE) * AJE;
                        let APR = ((APE * APE) + APQ).sqrt();
                        let APT = AMI + APE;
                        let APU = Lanes([AMJ[0], 0.0, AMJ[1]]);
                        let APV = Lanes([0.0, APF, 0.0]);
                        let APW = APU + APV;
                        let APX = APW * APT;
                        let APY = ((APT * APT) + APQ).sqrt();
                        let APZ = (FE * (APT - APY)) - APE;
                        let AQA = ((APW - ((APX + APX) * (CG / (FD * APY)))) * FE) - APV;
                        let AQB = APZ / GY;
                        let AQC = C - AQB;
                        let AQD = C - GT;
                        let AQE = AQC.powf(AQD);
                        let AQF = C - AGN;
                        let AQG = AQF.powf((-GT));
                        let AQH = (AMI - APZ) + (APS * (APE + APR));
                        let AQI = (APU - AQA) + Lanes([0.0, ((APF + ((APP + APP) * (CG / (FD * APR)))) * APS), 0.0]);
                        let AQJ = AQG * AQH;
                        let AQK = FE * GT;
                        let AQL = GY * AQF;
                        let AQM = (AQK * AQH) / AQL;
                        let AQN = C + AQM;
                        let AQO = ((APC * AQE) / AQD) + (AQJ * AQN);
                        let AQP = ((Lanes([0.0, (APD * AQE), 0.0]) + (((((AQA - Lanes([0.0, (HA * AQB), 0.0])) / GY) * AR) * (AQD * (AQC.powf((AQD - CG))))) * APC)) / AQD) + (((AQI * AQG) * AQN) + ((((AQI * AQK) - Lanes([0.0, ((HA * AQF) * AQM), 0.0])) / AQL) * AQJ));
                        AQQ = AQO;
                        AQR = AQP;
                    }
                    APH = AQQ;
                    API = AQR;
                } else {
                    APH = A;
                    API = AAL;
                }
                let APJ = (AMK * AMU) + (AMM * APH);
                let APK = (Lanes([0.0, (AML * AMU), 0.0]) + (AMV * AMK)) + (Lanes([0.0, (AMN * APH), 0.0]) + (API * AMM));
                ALP = APJ;
                ALQ = APK;
            } else {
                ALP = A;
                ALQ = AAL;
            }
            let ALS = N * ALR;
            let ALU = P * ALT;
            let ALV = H * (AFV + (ALR * M));
            let ALW = (AFW + Lanes([ALS[0], 0.0, ALS[1]])) * H;
            let ALX = H * (ALP + (ALT * O));
            let ALY = (ALQ + Lanes([ALU[0], 0.0, ALU[1]])) * H;
            let AMA = Q * ALZ;
            let AMB = W * ALZ;
            let ARZ;
            let ASA;
            let ASB;
            let ASC;
            if AMC != 0.0 {
                let ARR = AAX * ARQ;
                let ARS = ARR * BY;
                let ART = Lanes([0.0, ((AAY * ARQ) * BY)]) + Lanes([(BZ * ARR), 0.0]);
                ARZ = ARS;
                ASA = A;
                ASB = ART;
                ASC = ARU;
            } else {
                let ARV = ARQ * BY;
                let ARW = AAU / ARV;
                let ARX = (Lanes([AAW[0], 0.0, AAW[1]]) - Lanes([0.0, ((BZ * ARQ) * ARW), 0.0])) / ARV;
                ARZ = A;
                ASA = ARW;
                ASB = ARY;
                ASC = ARX;
            }
            let ASN;
            let ASO;
            let ASP;
            let ASQ;
            if ASD != 0.0 {
                let ASF = ABF * ASE;
                let ASG = ASF * BY;
                let ASH = Lanes([0.0, ((ABG * ASE) * BY)]) + Lanes([(BZ * ASF), 0.0]);
                ASN = ASG;
                ASO = A;
                ASP = ASH;
                ASQ = ASI;
            } else {
                let ASJ = ASE * BY;
                let ASK = ABC / ASJ;
                let ASL = (Lanes([ABE[0], 0.0, ABE[1]]) - Lanes([0.0, ((BZ * ASE) * ASK), 0.0])) / ASJ;
                ASN = A;
                ASO = ASK;
                ASP = ASM;
                ASQ = ASL;
            }
            let ASR = ddt(8156, ALV);
            let AST = ALW * ASS;
            let ASU = ddt(8158, ALX);
            let ASV = ALY * ASS;
            let ASW = ddt(8160, AMA);
            let ASX = AMB * ASS;
            let ASZ;
            let ATA;
            if ASY != 0.0 {
                let ATB = if AEN < A { 1.0 } else { 0.0 };
                let ATD = if XN != 0.0 {
                    ATC
                } else {
                    A
                };
                let ATF = if XS != 0.0 {
                    ATE
                } else {
                    A
                };
                ASZ = ATD;
                ATA = ATF;
            } else {
                ASZ = A;
                ATA = A;
            }
            let ATG = AEO[3];
            let ATH = AEO[1];
            let ATI = if ((ATG + ((ATH * staged[60]) * (AEN + (I * ATG)))).abs()) > 1e-99f64 { 1.0 } else { 0.0 };
            let ATJ = AEO[0];
            let ATK = AEO[2];
            let ATL = AEQ[0];
            let ATM = AEQ[1];
            let ATN = AEQ[2];
            let ATO = AES[0];
            let ATP = AES[1];
            let ATQ = AES[2];
            let ATR = AEL;
            let ATS = AEM[0];
            let ATT = AEM[1];
            let ATU = AEM[2];
            let ATV = AEM[3];
            let ATW = AEM[4];
            let ATX = AEM[5];
            let ATY = AEM[6];
            let ATZ = AEM[7];
            let AUA = ASB[0];
            let AUB = ASB[1];
            let AUC = ASC[0];
            let AUD = ASC[1];
            let AUE = ASC[2];
            let AUF = ASP[0];
            let AUG = ASP[1];
            let AUH = ASQ[0];
            let AUI = ASQ[1];
            let AUJ = ASQ[2];
            let AUK = AST[0];
            let AUL = AST[1];
            let AUM = AST[2];
            let AUN = ASV[0];
            let AUO = ASV[1];
            let AUP = ASV[2];
            let AUQ = ASX;
            let AUR = ALW[0];
            let AUS = ALW[1];
            let AUT = ALW[2];
            let AUU = ALY[0];
            let AUV = ALY[1];
            let AUW = ALY[2];
            let AUX = AMB;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (AEN),
            [1, 3, 4, 5],
            [ATJ, ATH, ATK, ATG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (AEP),
            [1, 3, 4],
            [ATL, ATM, ATN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (AER),
            [1, 3, 5],
            [ATO, ATP, ATQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (AEJ),
            [3],
            [ATR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 2>(
            Some(3),
            None,
            multiplicity * (AEK),
            [0, 1, 2, 3, 4, 5],
            [ATS, ATT, ATU, ATV, ATW, ATX],
            [0, 1],
            [ATY, ATZ],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(4), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<1, 1>(
            0,
            ARZ,
            [3],
            [AUA],
            [0],
            [AUB],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(4),
            multiplicity * (ASA),
            [0, 3, 4],
            [AUC, AUD, AUE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(2), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<1, 1>(
            1,
            ASN,
            [3],
            [AUF],
            [1],
            [AUG],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(5),
            multiplicity * (ASO),
            [2, 3, 5],
            [AUH, AUI, AUJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (ASR),
            [1, 3, 4],
            [AUK, AUL, AUM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (ASU),
            [1, 3, 5],
            [AUN, AUO, AUP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (ASW),
            [3],
            [AUQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (staged[111]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (staged[112]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(4),
            multiplicity * (staged[113]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(5),
            multiplicity * (staged[114]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (ASZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (ATA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = AEN;
        self.canonical_reactive[1] = AEP;
        self.canonical_reactive[2] = AER;
        self.canonical_reactive[3] = AEJ;
        self.canonical_reactive[4] = AEK;
        self.canonical_reactive[5] = ARZ;
        self.canonical_reactive[6] = ASA;
        self.canonical_reactive[7] = ASN;
        self.canonical_reactive[8] = ASO;
        self.canonical_reactive[9] = ALV;
        self.canonical_reactive[10] = AUR;
        self.canonical_reactive[11] = AUS;
        self.canonical_reactive[12] = AUT;
        self.canonical_reactive[13] = ALX;
        self.canonical_reactive[14] = AUU;
        self.canonical_reactive[15] = AUV;
        self.canonical_reactive[16] = AUW;
        self.canonical_reactive[17] = AMA;
        self.canonical_reactive[18] = AUX;
        self.canonical_reactive[19] = staged[111];
        self.canonical_reactive[20] = staged[112];
        self.canonical_reactive[21] = staged[113];
        self.canonical_reactive[22] = staged[114];
        self.canonical_reactive[23] = ASZ;
        self.canonical_reactive[24] = ATA;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4],
            &[cached[10], cached[11], cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 3, 5],
            &[cached[14], cached[15], cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[18]],
            &[],
            &[],
            multiplicity,
        );
    }

}
