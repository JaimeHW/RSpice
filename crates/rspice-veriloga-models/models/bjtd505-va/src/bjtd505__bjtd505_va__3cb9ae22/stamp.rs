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
        let mut key = Vec::with_capacity(282);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[143] = values[0];
        self.canonical_staged[91] = values[1];
        self.canonical_staged[1] = values[2];
        self.canonical_staged[144] = values[3];
        self.canonical_staged[0] = values[4];
        self.canonical_staged[77] = values[5];
        self.canonical_staged[81] = values[6];
        self.canonical_staged[145] = values[7];
        self.canonical_staged[25] = values[8];
        self.canonical_staged[26] = values[9];
        self.canonical_staged[84] = values[10];
        self.canonical_staged[88] = values[11];
        self.canonical_staged[146] = values[12];
        self.canonical_staged[27] = values[13];
        self.canonical_staged[28] = values[14];
        self.canonical_staged[2] = values[15];
        self.canonical_staged[3] = values[16];
        self.canonical_staged[4] = values[17];
        self.canonical_staged[5] = values[18];
        self.canonical_staged[7] = values[19];
        self.canonical_staged[157] = values[20];
        self.canonical_staged[159] = values[21];
        self.canonical_staged[8] = values[22];
        self.canonical_staged[9] = values[23];
        self.canonical_staged[10] = values[24];
        self.canonical_staged[11] = values[25];
        self.canonical_staged[12] = values[26];
        self.canonical_staged[13] = values[27];
        self.canonical_staged[14] = values[28];
        self.canonical_staged[15] = values[29];
        self.canonical_staged[16] = values[30];
        self.canonical_staged[17] = values[31];
        self.canonical_staged[162] = values[32];
        self.canonical_staged[18] = values[33];
        self.canonical_staged[19] = values[34];
        self.canonical_staged[20] = values[35];
        self.canonical_staged[21] = values[36];
        self.canonical_staged[22] = values[37];
        self.canonical_staged[23] = values[38];
        self.canonical_staged[24] = values[39];
        self.canonical_staged[29] = values[40];
        self.canonical_staged[30] = values[41];
        self.canonical_staged[31] = values[42];
        self.canonical_staged[32] = values[43];
        self.canonical_staged[33] = values[44];
        self.canonical_staged[34] = values[45];
        self.canonical_staged[35] = values[46];
        self.canonical_staged[164] = values[47];
        self.canonical_staged[166] = values[48];
        self.canonical_staged[168] = values[49];
        self.canonical_staged[43] = values[50];
        self.canonical_staged[47] = values[51];
        self.canonical_staged[170] = values[52];
        self.canonical_staged[171] = values[53];
        self.canonical_staged[49] = values[54];
        self.canonical_staged[51] = values[55];
        self.canonical_staged[172] = values[56];
        self.canonical_staged[173] = values[57];
        self.canonical_staged[69] = values[58];
        self.canonical_staged[76] = values[59];
        self.canonical_staged[82] = values[60];
        self.canonical_staged[174] = values[61];
        self.canonical_staged[92] = values[62];
        self.canonical_staged[175] = values[63];
        self.canonical_staged[176] = values[64];
        self.canonical_staged[99] = values[65];
        self.canonical_staged[97] = values[66];
        self.canonical_staged[98] = values[67];
        self.canonical_staged[103] = values[68];
        self.canonical_staged[107] = values[69];
        self.canonical_staged[114] = values[70];
        self.canonical_staged[115] = values[71];
        self.canonical_staged[177] = values[72];
        self.canonical_staged[178] = values[73];
        self.canonical_staged[125] = values[74];
        self.canonical_staged[127] = values[75];
        self.canonical_staged[179] = values[76];
        self.canonical_staged[129] = values[77];
        self.canonical_staged[130] = values[78];
        self.canonical_staged[131] = values[79];
        self.canonical_staged[132] = values[80];
        self.canonical_staged[133] = values[81];
        self.canonical_staged[180] = values[82];
        self.canonical_staged[181] = values[83];
        self.canonical_staged[182] = values[84];
        self.canonical_staged[183] = values[85];
        self.canonical_staged[189] = values[86];
        self.canonical_staged[192] = values[87];
        self.canonical_staged[194] = values[88];
        self.canonical_staged[196] = values[89];
        self.canonical_staged[185] = values[90];
        self.canonical_staged[186] = values[91];
        self.canonical_staged[187] = values[92];
        self.canonical_staged[188] = values[93];
        self.canonical_staged[190] = values[94];
        self.canonical_staged[191] = values[95];
        self.canonical_staged[193] = values[96];
        self.canonical_staged[195] = values[97];
        self.canonical_staged[138] = values[98];
        self.canonical_staged[139] = values[99];
        self.canonical_staged[140] = values[100];
        self.canonical_staged[142] = values[101];
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
                let A = parameters[3];
                let B = 1e0f64;
                let D = 1.23e8f64;
                let E = 7.03e7f64;
                let F = 2.04e8f64;
                let G = 1.58e8f64;
                let J = parameters[32];
                let M = parameters[137];
                let N = 0e0f64;
                let P = 1e-12f64;
                let R = 2e0f64;
                let S = parameters[66];
                let V = parameters[113];
                let X = 5e-2f64;
                let Y = 1e-1f64;
                let AG = parameters[71];
                let AJ = parameters[116];
                let AV = parameters[97];
                let AW = parameters[95];
                let BA = 4e0f64;
                let BB = parameters[120];
                let BF = parameters[102];
                let BH = 6e0f64;
                let CF = 3e0f64;
                let CI = 1e0f64;
                let CK = parameters[73];
                let CS = parameters[92];
                let CX = parameters[5];
                let DD = parameters[81];
                let DF = parameters[80];
                let DW = 0e0f64;
                let DY = 0e0f64;
                let EB = parameters[130];
                let EF = 0e0f64;
                let EG = 0e0f64;
                let ER = 0e0f64;
                let ES = 0e0f64;
                let ET = 0e0f64;
                let EU = 0e0f64;
                let EV = 0e0f64;
                let FB = 0e0f64;
                let FC = 0e0f64;
                let FD = 0e0f64;
                let mut oBP = 0.0;
                let mut oBQ = 0.0;
                let mut oBR = 0.0;
                let mut oCM = 0.0;
                let mut oCT = 0.0;
                let mut oCU = 0.0;
                let mut oDA = 0.0;
                let mut oDB = 0.0;
                let mut oDE = 0.0;
                let mut oDG = 0.0;
                let mut oDH = 0.0;
                let mut oDO = 0.0;
                let mut oDP = 0.0;
                let mut oDQ = 0.0;
                let mut oDR = 0.0;
                let mut oDS = 0.0;
                let mut oDT = 0.0;
                let mut oED = 0.0;
                let C = if A == B { 1.0 } else { 0.0 };
                let H;
                let I;
                if C != 0.0 {
                    H = D;
                    I = E;
                } else {
                    H = F;
                    I = G;
                }
                let K = B - J;
                let L = parameters[4] + 2.7315e2f64;
                let O = if M == N { 1.0 } else { 0.0 };
                let Q = if O != 0.0 {
                    P
                } else {
                    M
                };
                let T = R.powf((R - S));
                let U = B / T;
                let W = V + (((parameters[114] * L) * L) / (L + parameters[115]));
                let Z = (W - X) / Y;
                let AA = if W < X { 1.0 } else { 0.0 };
                let AD = if AA != 0.0 {
                    let AB = X + (Y * ((B + (Z.exp())).ln()));
                    AB
                } else {
                    let AC = W + (Y * ((B + ((-Z).exp())).ln()));
                    AC
                };
                let AE = B / V;
                let AF = B / parameters[65];
                let AH = R.powf((R - AG));
                let AI = B / AH;
                let AK = AJ + (((parameters[117] * L) * L) / (L + parameters[118]));
                let AL = (AK - X) / Y;
                let AM = if AK < X { 1.0 } else { 0.0 };
                let AP = if AM != 0.0 {
                    let AN = X + (Y * ((B + (AL.exp())).ln()));
                    AN
                } else {
                    let AO = AK + (Y * ((B + ((-AL).exp())).ln()));
                    AO
                };
                let AQ = B / AJ;
                let AR = B / parameters[70];
                let AS = B - (B / parameters[82]);
                let AT = B / (8.617086918058125e-5f64 * L);
                let AU = B - parameters[74];
                let AX = AV - AW;
                let AY = if parameters[121] != N { 1.0 } else { 0.0 };
                let AZ = if parameters[122] != N { 1.0 } else { 0.0 };
                let BC = ((BA - AV) - AW) + BB;
                let BD = -parameters[104];
                let BE = B - AV;
                let BG = B - BF;
                let BI = BH - (R * parameters[20]);
                let BJ = -parameters[112];
                let BK = BH - (R * parameters[31]);
                let BL = -parameters[109];
                let BM = (BA - parameters[96]) + BB;
                let BN = -parameters[110];
                let BO = if parameters[23] == B { 1.0 } else { 0.0 };
                if BO != 0.0 {
                    let BP = -parameters[106];
                    oBP = BP;
                    let BQ = -parameters[105];
                    oBQ = BQ;
                    let BR = -parameters[107];
                    oBR = BR;
                } else {
                }
                let BS = (BA - BF) + BB;
                let BT = -parameters[111];
                let BU = BH - (R * parameters[22]);
                let BV = BA / parameters[133];
                let BW = AV - R;
                let BX = -parameters[119];
                let BY = (AW + AV) - B;
                let BZ = parameters[98] - B;
                let CA = parameters[86] + parameters[87];
                let CB = parameters[99] - B;
                let CC = if parameters[56] > N { 1.0 } else { 0.0 };
                let CD = if parameters[57] > N { 1.0 } else { 0.0 };
                let CE = if parameters[58] > N { 1.0 } else { 0.0 };
                let CG = B - (CF.powf((-1e0f64 / S)));
                let CH = B - S;
                let CJ = CH - CI;
                let CL = if CK == B { 1.0 } else { 0.0 };
                if CL != 0.0 {
                } else {
                    let CM = if CK == R { 1.0 } else { 0.0 };
                    oCM = CM;
                }
                let CN = -1e0f64 / AG;
                let CO = parameters[75] - CI;
                let CP = B - AG;
                let CQ = CP - CI;
                let CR = if parameters[91] == N { 1.0 } else { 0.0 };
                if BO != 0.0 {
                } else {
                    let CT = if CS == N { 1.0 } else { 0.0 };
                    oCT = CT;
                    if CT != 0.0 {
                    } else {
                        let CU = B - CS;
                        oCU = CU;
                    }
                }
                let CV = if (if parameters[33] > N { 1.0 } else { 0.0 }) != 0.0 && (if parameters[34] > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CW = if (if parameters[35] > N { 1.0 } else { 0.0 }) != 0.0 && (if parameters[36] > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CY = if J > N { 1.0 } else { 0.0 };
                let CZ = if (if CX > N { 1.0 } else { 0.0 }) != 0.0 && CY != 0.0 { 1.0 } else { 0.0 };
                if CZ != 0.0 {
                    let DA = J * R;
                    oDA = DA;
                    let DB = if CX == B { 1.0 } else { 0.0 };
                    oDB = DB;
                } else {
                }
                let DC = if parameters[83] == B { 1.0 } else { 0.0 };
                if DC != 0.0 {
                    let DE = B / (B - (AS.powf(DD)));
                    oDE = DE;
                    let DG = AS * DF;
                    oDG = DG;
                    let DH = (((DE * DE) * (AS.powf((DD - B)))) * DD) / DF;
                    oDH = DH;
                } else {
                }
                let DI = B - parameters[67];
                let DJ = B - parameters[76];
                let DK = B / parameters[84];
                let DL = if parameters[78] == N { 1.0 } else { 0.0 };
                let DM = if (if (if CX == B { 1.0 } else { 0.0 }) != 0.0 || (if CX == CF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && CY != 0.0 { 1.0 } else { 0.0 };
                if DM != 0.0 {
                    if DL != 0.0 {
                        let DO = 5e-1f64 * J;
                        oDO = DO;
                    } else {
                        let DP = R * J;
                        oDP = DP;
                    }
                } else {
                }
                let DN = if parameters[6] == B { 1.0 } else { 0.0 };
                if DN != 0.0 {
                    let DQ = -S;
                    oDQ = DQ;
                    let DR = DQ - CI;
                    oDR = DR;
                    let DS = B - parameters[94];
                    oDS = DS;
                    let DT = B - parameters[93];
                    oDT = DT;
                } else {
                }
                let DU = A * parameters[68];
                let DV = A * parameters[77];
                let DX = if CD != 0.0 {
                    N
                } else {
                    DW
                };
                let DZ = if CE != 0.0 {
                    N
                } else {
                    DY
                };
                let EA = if parameters[129] > N { 1.0 } else { 0.0 };
                let EC = if EB == B { 1.0 } else { 0.0 };
                if EC != 0.0 {
                } else {
                    let ED = if EB == R { 1.0 } else { 0.0 };
                    oED = ED;
                }
                let EE = if J == N { 1.0 } else { 0.0 };
                let EH;
                let EI;
                if BO != 0.0 {
                    EH = EF;
                    EI = N;
                } else {
                    EH = N;
                    EI = EG;
                }
                let EJ;
                let EK;
                let EL;
                let EM;
                let EN;
                let EO;
                let EP;
                let EQ;
                if CD != 0.0 {
                    let EW;
                    let EX;
                    let EY;
                    let EZ;
                    let FA;
                    if CE != 0.0 {
                        EW = ER;
                        EX = N;
                        EY = ES;
                        EZ = ET;
                        FA = N;
                    } else {
                        EW = N;
                        EX = EU;
                        EY = N;
                        EZ = N;
                        FA = EV;
                    }
                    EJ = EW;
                    EK = EX;
                    EL = N;
                    EM = N;
                    EN = EY;
                    EO = EZ;
                    EP = FA;
                    EQ = N;
                } else {
                    let FE;
                    let FF;
                    let FG;
                    if CE != 0.0 {
                        FE = FB;
                        FF = N;
                        FG = FC;
                    } else {
                        FE = N;
                        FF = FD;
                        FG = N;
                    }
                    EJ = N;
                    EK = N;
                    EL = FE;
                    EM = FF;
                    EN = N;
                    EO = N;
                    EP = N;
                    EQ = FG;
                }
            [C, K, L, O, Q, T, U, AA, AE, AF, AH, AI, AM, AQ, AR, AT, AD, AP, AU, AX, AY, AZ, BC, BD, BE, BG, BI, BJ, BK, BL, BM, BN, BO, oBP, oBQ, oBR, BS, BT, BU, BV, BW, BX, BY, BZ, CA, CB, H, CC, CD, CE, CG, CH, CL, oCM, CN, CP, CR, oCT, oCU, CV, CW, CZ, oDA, oDB, DC, oDE, oDG, oDH, I, DI, DJ, DK, DL, DM, oDO, oDP, DN, oDQ, oDS, oDT, DU, DV, EA, EC, oED, EE, EJ, EK, EL, EM, DX, DZ, EH, EI, EN, EO, EP, EQ, CJ, CO, CQ, oDR]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 3] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[1];
                let C = 1e0f64;
                let B = staged[0] * A;
                let D = C / B;
                let E = if A != C { 1.0 } else { 0.0 };
            [B, D, E]
        };
        self.canonical_staged[6] = produced[0];
        self.canonical_staged[36] = produced[1];
        self.canonical_staged[184] = produced[2];
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
        let produced: [f64; 92] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let B = staged[1];
                let E = 1e0f64;
                let K = 5e-2f64;
                let L = 1e-1f64;
                let X = parameters[65];
                let AS = parameters[70];
                let BP = parameters[71];
                let BS = parameters[74];
                let BY = staged[6];
                let CK = staged[157];
                let CL = parameters[9];
                let CN = 1e-3f64;
                let CR = staged[159];
                let CW = parameters[10];
                let DD = 0e0f64;
                let DJ = 1e-6f64;
                let DL = 5e-1f64;
                let DV = parameters[16];
                let DY = parameters[18];
                let EA = staged[162];
                let EL = staged[25];
                let EO = parameters[34];
                let EQ = staged[26];
                let ES = staged[27];
                let EU = parameters[36];
                let EW = staged[28];
                let FJ = staged[35];
                let FO = staged[164];
                let FQ = staged[36];
                let FT = staged[166];
                let FY = staged[168];
                let GH = 2e0f64;
                let GM = 4e0f64;
                let GQ = staged[172];
                let GX = staged[174];
                let GZ = staged[175];
                let HN = staged[177];
                let HP = staged[178];
                let mut oCP = 0.0;
                let mut oCZ = 0.0;
                let mut oFR = 0.0;
                let mut oFW = 0.0;
                let mut oGB = 0.0;
                let mut oGR = 0.0;
                let mut oGT = 0.0;
                let mut oGU = 0.0;
                let mut oGY = 0.0;
                let mut oHA = 0.0;
                let mut oHB = 0.0;
                let mut oHO = 0.0;
                let mut oHQ = 0.0;
                let mut oHR = 0.0;
                let mut oHS = 0.0;
                let A = temperature + parameters[0];
                let C = A / B;
                let D = 8.617086918058125e-5f64 * A;
                let F = E / D;
                let G = F - staged[2];
                let H = A - B;
                let I = C.ln();
                let J = staged[3] - (((parameters[114] * A) * A) / (A + parameters[115]));
                let M = (J - K) / L;
                let N = if J < K { 1.0 } else { 0.0 };
                let Q = if N != 0.0 {
                    let O = K + (L * ((E + (M.exp())).ln()));
                    O
                } else {
                    let P = J + (L * ((E + ((-M).exp())).ln()));
                    P
                };
                let R = staged[4] - (((parameters[117] * A) * A) / (A + parameters[118]));
                let S = (R - K) / L;
                let T = if R < K { 1.0 } else { 0.0 };
                let W = if T != 0.0 {
                    let U = K + (L * ((E + (S.exp())).ln()));
                    U
                } else {
                    let V = R + (L * ((E + ((-S).exp())).ln()));
                    V
                };
                let Y = E - C;
                let Z = (((-3e0f64 * D) * I) + (X * C)) + (Y * parameters[104]);
                let AA = (K - Z) / D;
                let AB = if K < Z { 1.0 } else { 0.0 };
                let AE = if AB != 0.0 {
                    let AC = Z + (D * ((E + (AA.exp())).ln()));
                    AC
                } else {
                    let AD = K + (D * ((E + ((-AA).exp())).ln()));
                    AD
                };
                let AF = Y * parameters[109];
                let AG = (((-3e0f64 * D) * I) + (parameters[63] * C)) + AF;
                let AH = (K - AG) / D;
                let AI = if K < AG { 1.0 } else { 0.0 };
                let AL = if AI != 0.0 {
                    let AJ = AG + (D * ((E + (AH.exp())).ln()));
                    AJ
                } else {
                    let AK = K + (D * ((E + ((-AH).exp())).ln()));
                    AK
                };
                let AM = (((-3e0f64 * D) * I) + (parameters[79] * C)) + AF;
                let AN = (K - AM) / D;
                let AO = if K < AM { 1.0 } else { 0.0 };
                let AR = if AO != 0.0 {
                    let AP = AM + (D * ((E + (AN.exp())).ln()));
                    AP
                } else {
                    let AQ = K + (D * ((E + ((-AN).exp())).ln()));
                    AQ
                };
                let AT = AS * C;
                let AU = (((-3e0f64 * D) * I) + AT) + AF;
                let AV = (K - AU) / D;
                let AW = if K < AU { 1.0 } else { 0.0 };
                let AZ = if AW != 0.0 {
                    let AX = AU + (D * ((E + (AV.exp())).ln()));
                    AX
                } else {
                    let AY = K + (D * ((E + ((-AV).exp())).ln()));
                    AY
                };
                let BA = (((-3e0f64 * D) * I) + AT) + AF;
                let BB = (K - BA) / D;
                let BC = if K < BA { 1.0 } else { 0.0 };
                let BF = if BC != 0.0 {
                    let BD = BA + (D * ((E + (BB.exp())).ln()));
                    BD
                } else {
                    let BE = K + (D * ((E + ((-BB).exp())).ln()));
                    BE
                };
                let BG = (((-3e0f64 * D) * I) + (parameters[26] * C)) + (Y * parameters[108]);
                let BH = (K - BG) / D;
                let BI = if K < BG { 1.0 } else { 0.0 };
                let BL = if BI != 0.0 {
                    let BJ = BG + (D * ((E + (BH.exp())).ln()));
                    BJ
                } else {
                    let BK = K + (D * ((E + ((-BH).exp())).ln()));
                    BK
                };
                let BM = E / AE;
                let BN = E / BF;
                let BO = (X * BM).powf(parameters[66]);
                let BQ = (AS * BN).powf(BP);
                let BR = parameters[64] * BO;
                let BT = (staged[5] * ((AS / AZ).powf(BP))) + BS;
                let BU = E / BT;
                let BV = parameters[69] * BT;
                let BW = BS * BU;
                let BX = parameters[53] * ((I * parameters[96]).exp());
                let BZ = if BX < BY { 1.0 } else { 0.0 };
                let CA = if BZ != 0.0 {
                    BY
                } else {
                    BX
                };
                let CB = parameters[55] * ((I * staged[7]).exp());
                let CC = parameters[54] * ((I * parameters[100]).exp());
                let CD = if CC < BY { 1.0 } else { 0.0 };
                let CE = if CD != 0.0 {
                    BY
                } else {
                    CC
                };
                let CF = parameters[56] * ((I * parameters[101]).exp());
                let CG = (I * parameters[103]).exp();
                let CH = parameters[57] * CG;
                let CI = parameters[58] * CG;
                let CJ = parameters[59] * ((I * parameters[98]).exp());
                let CQ;
                if CK != 0.0 {
                    let CM = CL * (E + (H * parameters[121]));
                    let CO = (CM - E) / CN;
                    let CP = if CM < E { 1.0 } else { 0.0 };
                    oCP = CP;
                    let CU = if CP != 0.0 {
                        let CS = E + (CN * ((E + (CO.exp())).ln()));
                        CS
                    } else {
                        let CT = CM + (CN * ((E + ((-CO).exp())).ln()));
                        CT
                    };
                    let CV = CU - 6.931471805599453e-4f64;
                    CQ = CV;
                } else {
                    CQ = CL;
                }
                let DA;
                if CR != 0.0 {
                    let CX = CW * (E + (H * parameters[122]));
                    let CY = (CX - E) / CN;
                    let CZ = if CX < E { 1.0 } else { 0.0 };
                    oCZ = CZ;
                    let DH = if CZ != 0.0 {
                        let DF = E + (CN * ((E + (CY.exp())).ln()));
                        DF
                    } else {
                        let DG = CX + (CN * ((E + ((-CY).exp())).ln()));
                        DG
                    };
                    let DI = DH - 6.931471805599453e-4f64;
                    DA = DI;
                } else {
                    DA = CW;
                }
                let DB = parameters[42] * (E + (parameters[123] * H));
                let DC = DB * DB;
                let DE = if DB < DD { 1.0 } else { 0.0 };
                let DN = if DE != 0.0 {
                    let DK = 5e-7f64 / (((DC + DJ).sqrt()) - DB);
                    DK
                } else {
                    let DM = DL * (((DC + DJ).sqrt()) + DB);
                    DM
                };
                let DO = (parameters[8] * (((I * staged[8]) / CQ).exp())) * (((staged[9] * G) / CQ).exp());
                let DP = parameters[11] * ((I * staged[10]).exp());
                let DQ = parameters[29] * ((I * staged[11]).exp());
                let DR = staged[13] * G;
                let DS = (parameters[19] * ((I * staged[12]).exp())) * ((DR / parameters[20]).exp());
                let DT = (parameters[30] * ((I * staged[14]).exp())) * (((staged[15] * G) / parameters[31]).exp());
                let DU = I * staged[16];
                let DW = staged[17] * G;
                let DX = (parameters[15] * ((DU / DV).exp())) * ((DW / DV).exp());
                let DZ = (parameters[17] * ((DU / DY).exp())) * ((DW / DY).exp());
                let EE;
                let EF;
                let EG;
                if EA != 0.0 {
                    let EB = parameters[24] * (((staged[18] * G) / DV).exp());
                    let EC = parameters[27] * ((staged[19] * G).exp());
                    let ED = parameters[25] * (((staged[20] * G) / DY).exp());
                    EE = EB;
                    EF = EC;
                    EG = ED;
                } else {
                    EE = DD;
                    EF = DD;
                    EG = DD;
                }
                let EH = (parameters[28] * ((I * staged[21]).exp())) * ((staged[22] * G).exp());
                let EI = (parameters[21] * ((I * staged[23]).exp())) * ((DR / parameters[22]).exp());
                let EJ = (parameters[132] * ((I * staged[24]).exp())) * ((DR / parameters[133]).exp());
                let EK = (parameters[138] * (C.sqrt())) * ((parameters[140] * H).exp());
                let EM = (Q * EL).powf(-5e-1f64);
                let EN = E / BO;
                let EP = (((((((EO * Q) * Q) * EM) * EN) * X) * BM) * EL) * EL;
                let ER = ((((((parameters[33] * EM) * AE) * AE) * EQ) * EQ) * BO) * ((EO - EP).exp());
                let ET = (W * ES).powf(-5e-1f64);
                let EV = (((((((EU * W) * W) * ET) * (E / BQ)) * AS) * BN) * ES) * ES;
                let EX = ((((((parameters[35] * ET) * BF) * BF) * EW) * EW) * BQ) * ((EU - EV).exp());
                let EY = (I * parameters[95]).exp();
                let EZ = (parameters[13] * EY) * BU;
                let FA = (parameters[12] * EY) * EN;
                let FB = (parameters[85] * ((I * staged[29]).exp())) * ((staged[30] * G).exp());
                let FC = parameters[86] * ((I * staged[31]).exp());
                let FD = parameters[87] * ((I * staged[32]).exp());
                let FE = FC + FD;
                let FF = (parameters[88] * FE) / staged[33];
                let FG = parameters[89] * ((I * staged[34]).exp());
                let FH = A - 3e2f64;
                let FI = if A < 5.25e2f64 { 1.0 } else { 0.0 };
                let FM = if FI != 0.0 {
                    let FK = FJ * ((E + (7.2e-4f64 * FH)) - ((1.6e-6f64 * FH) * FH));
                    FK
                } else {
                    let FL = FJ * 1.081e0f64;
                    FL
                };
                let FN = parameters[91] * EY;
                let FS;
                if FO != 0.0 {
                    let FP = E / CF;
                    let FR = if FP > FQ { 1.0 } else { 0.0 };
                    oFR = FR;
                    let FU = if FR != 0.0 {
                        FQ
                    } else {
                        FP
                    };
                    FS = FU;
                } else {
                    FS = DD;
                }
                let FX;
                if FT != 0.0 {
                    let FV = E / CH;
                    let FW = if FV > FQ { 1.0 } else { 0.0 };
                    oFW = FW;
                    let FZ = if FW != 0.0 {
                        FQ
                    } else {
                        FV
                    };
                    FX = FZ;
                } else {
                    FX = DD;
                }
                let GC;
                if FY != 0.0 {
                    let GA = E / CI;
                    let GB = if GA > FQ { 1.0 } else { 0.0 };
                    oGB = GB;
                    let GD = if GB != 0.0 {
                        FQ
                    } else {
                        GA
                    };
                    GC = GD;
                } else {
                    GC = DD;
                }
                let GE = AE * staged[43];
                let GF = L * AE;
                let GG = AE / staged[47];
                let GI = E - BW;
                let GJ = (GH - BW) / GI;
                let GK = AZ * (E - (GJ.powf(staged[49])));
                let GL = AZ / staged[51];
                let GN = (GM * DO) / DP;
                let GO = E / DA;
                let GP = GO - 1e0f64;
                if GQ != 0.0 {
                } else {
                    let GR = ((FN * F).exp()) - E;
                    oGR = GR;
                }
                let GS = parameters[14] * DO;
                if EA != 0.0 {
                    let GT = EE * GH;
                    oGT = GT;
                } else {
                }
                if EA != 0.0 {
                    let GU = EG * GH;
                    oGU = GU;
                } else {
                }
                let GV = GH * EH;
                let GW = (GM * EH) / DQ;
                if GX != 0.0 {
                    let GY = staged[92] * EH;
                    oGY = GY;
                    if GZ != 0.0 {
                        let HA = (parameters[32] * EH) * CF;
                        oHA = HA;
                        let HB = D * (GH - ((HA * F).ln()));
                        oHB = HB;
                    } else {
                    }
                } else {
                }
                let HC = GH * D;
                let HD = staged[107] * BR;
                let HE = parameters[67] * BR;
                let HF = parameters[76] * BV;
                let HG = FC * DP;
                let HH = DL * HG;
                let HI = L * AZ;
                let HJ = (FB * DP) * ((DO / DP).powf(staged[115]));
                let HK = parameters[84] * D;
                let HL = ((GM * FD) * D) / CJ;
                let HM = DL * HL;
                if HN != 0.0 {
                    let HO = FF * DL;
                    oHO = HO;
                } else {
                    let HQ = GV * FG;
                    oHQ = HQ;
                }
                if HP != 0.0 {
                    if HN != 0.0 {
                        let HR = staged[125] * FF;
                        oHR = HR;
                    } else {
                        let HS = (staged[127] * EH) * FG;
                        oHS = HS;
                    }
                } else {
                }
            [D, F, N, T, AB, AI, AO, AW, BC, BI, BM, BN, AZ, BV, BW, BZ, CB, CD, CF, CJ, oCP, oCZ, DE, CQ, DO, DS, DT, DX, DZ, EI, EJ, EK, Q, EP, ER, W, EV, EX, EZ, FA, FC, FE, FI, FN, oFR, oFW, oGB, AL, GE, GF, GG, GI, GJ, GK, GL, GN, GO, oGR, GS, BL, oGT, EF, oGU, GV, GW, oGY, oHA, oHB, HC, DN, FM, CE, CA, HD, HE, HF, HG, HH, HI, HJ, HK, HL, HM, oHO, AR, oHQ, oHR, oHS, FS, FX, GC, GP]
        };
        self.canonical_staged[40] = produced[0];
        self.canonical_staged[37] = produced[1];
        self.canonical_staged[147] = produced[2];
        self.canonical_staged[148] = produced[3];
        self.canonical_staged[149] = produced[4];
        self.canonical_staged[150] = produced[5];
        self.canonical_staged[151] = produced[6];
        self.canonical_staged[152] = produced[7];
        self.canonical_staged[153] = produced[8];
        self.canonical_staged[154] = produced[9];
        self.canonical_staged[46] = produced[10];
        self.canonical_staged[83] = produced[11];
        self.canonical_staged[42] = produced[12];
        self.canonical_staged[113] = produced[13];
        self.canonical_staged[55] = produced[14];
        self.canonical_staged[155] = produced[15];
        self.canonical_staged[100] = produced[16];
        self.canonical_staged[156] = produced[17];
        self.canonical_staged[95] = produced[18];
        self.canonical_staged[41] = produced[19];
        self.canonical_staged[158] = produced[20];
        self.canonical_staged[160] = produced[21];
        self.canonical_staged[161] = produced[22];
        self.canonical_staged[38] = produced[23];
        self.canonical_staged[63] = produced[24];
        self.canonical_staged[72] = produced[25];
        self.canonical_staged[74] = produced[26];
        self.canonical_staged[66] = produced[27];
        self.canonical_staged[70] = produced[28];
        self.canonical_staged[73] = produced[29];
        self.canonical_staged[75] = produced[30];
        self.canonical_staged[64] = produced[31];
        self.canonical_staged[79] = produced[32];
        self.canonical_staged[78] = produced[33];
        self.canonical_staged[80] = produced[34];
        self.canonical_staged[86] = produced[35];
        self.canonical_staged[85] = produced[36];
        self.canonical_staged[87] = produced[37];
        self.canonical_staged[59] = produced[38];
        self.canonical_staged[58] = produced[39];
        self.canonical_staged[137] = produced[40];
        self.canonical_staged[122] = produced[41];
        self.canonical_staged[163] = produced[42];
        self.canonical_staged[60] = produced[43];
        self.canonical_staged[165] = produced[44];
        self.canonical_staged[167] = produced[45];
        self.canonical_staged[169] = produced[46];
        self.canonical_staged[39] = produced[47];
        self.canonical_staged[44] = produced[48];
        self.canonical_staged[45] = produced[49];
        self.canonical_staged[48] = produced[50];
        self.canonical_staged[54] = produced[51];
        self.canonical_staged[53] = produced[52];
        self.canonical_staged[50] = produced[53];
        self.canonical_staged[52] = produced[54];
        self.canonical_staged[56] = produced[55];
        self.canonical_staged[57] = produced[56];
        self.canonical_staged[61] = produced[57];
        self.canonical_staged[62] = produced[58];
        self.canonical_staged[65] = produced[59];
        self.canonical_staged[67] = produced[60];
        self.canonical_staged[68] = produced[61];
        self.canonical_staged[71] = produced[62];
        self.canonical_staged[89] = produced[63];
        self.canonical_staged[90] = produced[64];
        self.canonical_staged[93] = produced[65];
        self.canonical_staged[96] = produced[66];
        self.canonical_staged[94] = produced[67];
        self.canonical_staged[101] = produced[68];
        self.canonical_staged[102] = produced[69];
        self.canonical_staged[104] = produced[70];
        self.canonical_staged[105] = produced[71];
        self.canonical_staged[106] = produced[72];
        self.canonical_staged[108] = produced[73];
        self.canonical_staged[109] = produced[74];
        self.canonical_staged[110] = produced[75];
        self.canonical_staged[119] = produced[76];
        self.canonical_staged[111] = produced[77];
        self.canonical_staged[112] = produced[78];
        self.canonical_staged[117] = produced[79];
        self.canonical_staged[116] = produced[80];
        self.canonical_staged[120] = produced[81];
        self.canonical_staged[118] = produced[82];
        self.canonical_staged[121] = produced[83];
        self.canonical_staged[123] = produced[84];
        self.canonical_staged[124] = produced[85];
        self.canonical_staged[126] = produced[86];
        self.canonical_staged[128] = produced[87];
        self.canonical_staged[134] = produced[88];
        self.canonical_staged[135] = produced[89];
        self.canonical_staged[136] = produced[90];
        self.canonical_staged[141] = produced[91];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 12033 => 0usize, 12039 => 1usize, 12049 => 2usize, 12055 => 3usize, 12063 => 4usize, 12071 => 5usize, 12091 => 6usize, 12110 => 7usize, 12363 => 8usize, _ => usize::MAX };
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
            let C = staged[162];
            let D = staged[166];
            let E = staged[168];
            let F = node_potentials[5];
            let G = node_potentials[6];
            let H = 1e0f64;
            let I = 1e0f64;
            let J = parameters[3];
            let M = node_potentials[7];
            let N = 1e0f64;
            let Q = node_potentials[3];
            let R = 1e0f64;
            let U = node_potentials[4];
            let V = 1e0f64;
            let AC = node_potentials[2];
            let AD = 1e0f64;
            let AG = node_potentials[1];
            let AH = 1e0f64;
            let AM = 1e0f64;
            let AP = node_potentials[9];
            let AQ = 1e0f64;
            let AT = 1e0f64;
            let BA = -1e0f64;
            let BI = staged[37];
            let BL = parameters[134];
            let BQ = 1e0f64;
            let BX = staged[38];
            let DM = staged[39];
            let EY = 4e0f64;
            let FA = 2e0f64;
            let FB = 1e0f64;
            let FF = 2e0f64;
            let FJ = parameters[136];
            let FL = Lanes([0e0f64; 2]);
            let FS = staged[40];
            let FW = staged[41];
            let GA = 1e2f64;
            let GT = staged[44];
            let GU = staged[45];
            let HE = 5e-1f64;
            let HK = 2e-1f64;
            let IA = parameters[61];
            let IB = parameters[60];
            let IJ = parameters[62];
            let KL = parameters[135];
            let KN = Lanes([0e0f64; 3]);
            let LD = staged[42];
            let LE = 1e-1f64;
            let MQ = staged[46];
            let MT = staged[47];
            let MV = staged[138];
            let MX = staged[48];
            let MY = 3e0f64;
            let NB = staged[170];
            let ND = staged[171];
            let NG = staged[50];
            let OB = parameters[75];
            let OF = staged[51];
            let OH = staged[140];
            let OI = staged[52];
            let OJ = staged[53];
            let OM = staged[54];
            let ON = staged[55];
            let OR = staged[56];
            let OZ = staged[57];
            let PI = staged[172];
            let PJ = staged[58];
            let PL = staged[59];
            let PP = staged[60];
            let PU = staged[61];
            let QD = 1.0000000000000002e-2f64;
            let QR = staged[62];
            let QU = staged[63];
            let RB = 1e-4f64;
            let RP = parameters[139];
            let SA = staged[64];
            let SD = parameters[141];
            let SE = 1e-3f64;
            let SS = parameters[142];
            let SY = parameters[16];
            let TJ = staged[65];
            let TM = staged[173];
            let TR = parameters[18];
            let UE = 4e1f64;
            let UI = 2.3538526683702e17f64;
            let UO = staged[66];
            let UQ = staged[67];
            let UX = staged[68];
            let VH = staged[69];
            let VJ = parameters[92];
            let WA = staged[70];
            let WF = parameters[20];
            let WR = staged[71];
            let XE = staged[72];
            let XH = parameters[22];
            let XS = staged[73];
            let XV = parameters[31];
            let YG = staged[74];
            let YJ = parameters[133];
            let YU = staged[75];
            let YZ = staged[77];
            let ZB = staged[78];
            let ZF = Lanes([0e0f64; 2]);
            let ZT = 1e-30f64;
            let ZV = parameters[66];
            let ZZ = 6e0f64;
            let AAE = 1.6666666666666666e-1f64;
            let AAF = staged[79];
            let AAN = 3.333333333333333e-1f64;
            let AAP = 2.5e-1f64;
            let AAZ = staged[81];
            let ABO = staged[83];
            let ABV = staged[84];
            let ABX = staged[85];
            let ACP = staged[89];
            let ACQ = staged[90];
            let ACV = staged[174];
            let ADF = parameters[71];
            let ADN = staged[86];
            let AEF = staged[88];
            let AEU = staged[91];
            let AEX = staged[93];
            let AFC = staged[175];
            let AFD = Lanes([0e0f64; 8]);
            let AFK = staged[176];
            let AFU = 1.21e-2f64;
            let AGE = staged[95];
            let AGK = -1e0f64;
            let AGL = -1e0f64;
            let AGQ = Lanes([0e0f64; 3]);
            let AHN = 1e-12f64;
            let AHP = -1e0f64;
            let AHU = -1e0f64;
            let AHZ = staged[97];
            let AIB = parameters[80];
            let AID = parameters[81];
            let AIH = staged[98];
            let AIM = 1.0000000000000002e-2f64;
            let AIZ = staged[6];
            let AJB = Lanes([0e0f64; 4]);
            let AJG = staged[101];
            let AJM = parameters[38];
            let AJR = parameters[43];
            let AJX = parameters[41];
            let AKO = staged[102];
            let AKQ = parameters[40];
            let ALB = parameters[39];
            let ALK = parameters[44];
            let ALS = parameters[7];
            let AMR = parameters[46];
            let ANQ = staged[103];
            let ANR = staged[104];
            let AOS = parameters[47];
            let AOW = parameters[48];
            let APG = parameters[50];
            let AQA = parameters[49];
            let AQT = staged[105];
            let AQX = staged[106];
            let ARG = 1e-6f64;
            let ASC = staged[108];
            let AST = staged[109];
            let ASW = staged[110];
            let ASZ = staged[111];
            let ATI = staged[112];
            let ATX = staged[113];
            let ATY = staged[114];
            let AUP = parameters[32];
            let AUS = staged[116];
            let AVD = staged[117];
            let AVG = staged[118];
            let AVL = staged[177];
            let AVM = staged[119];
            let AVN = staged[120];
            let AVO = staged[121];
            let AVP = staged[122];
            let AVS = staged[123];
            let AVT = parameters[90];
            let AVZ = staged[178];
            let AWH = staged[124];
            let AWS = staged[179];
            let AXD = staged[126];
            let AXT = staged[128];
            let AXY = staged[129];
            let AYC = Lanes([0e0f64; 5]);
            let AYL = parameters[1];
            let AZT = staged[130];
            let AZW = parameters[94];
            let BAA = parameters[93];
            let BAD = staged[131];
            let BAQ = -1e0f64;
            let BAZ = ddt_scale();
            let BBW = staged[132];
            let BCD = staged[133];
            let BCM = staged[134];
            let BDD = staged[135];
            let BDG = Lanes([0e0f64; 2]);
            let BDJ = staged[136];
            let BDM = Lanes([0e0f64; 2]);
            let BDR = staged[180];
            let BEA = staged[137];
            let BEG = staged[181];
            let BEJ = staged[182];
            let BEN = parameters[131];
            let BEW = node_potentials[10];
            let BEY = 1e0f64;
            let BKQ = 0e0f64;
            let BKR = 0e0f64;
            let BKS = 0e0f64;
            let BKT = 0e0f64;
            let BKU = 0e0f64;
            let BKV = 0e0f64;
            let BKW = 0e0f64;
            let BKX = 0e0f64;
            let BKY = 0e0f64;
            let BKZ = 0e0f64;
            let BLA = 0e0f64;
            let BLB = 0e0f64;
            let BLC = 0e0f64;
            let BLD = 0e0f64;
            let BLE = 0e0f64;
            let B = ctx.simparam_or("gmin", A);
            let K = J * (F - G);
            let L = (Lanes([H, 0.0]) - Lanes([0.0, I])) * J;
            let O = J * (F - M);
            let P = (Lanes([H, 0.0]) - Lanes([0.0, N])) * J;
            let S = J * (F - Q);
            let T = (Lanes([0.0, H]) - Lanes([R, 0.0])) * J;
            let W = J * (U - Q);
            let X = (Lanes([0.0, V]) - Lanes([R, 0.0])) * J;
            let Y = J * (U - F);
            let Z = (Lanes([V, 0.0]) - Lanes([0.0, H])) * J;
            let AA = J * (G - M);
            let AB = (Lanes([I, 0.0]) - Lanes([0.0, N])) * J;
            let AE = J * (AC - Q);
            let AF = (Lanes([AD, 0.0]) - Lanes([0.0, R])) * J;
            let AI = J * (AG - U);
            let AJ = (Lanes([AH, 0.0]) - Lanes([0.0, V])) * J;
            let AK = J * (AG - AC);
            let AL = (Lanes([AH, 0.0]) - Lanes([0.0, AD])) * J;
            let AN = J * (AG - node_potentials[0]);
            let AO = (Lanes([0.0, AH]) - Lanes([AM, 0.0])) * J;
            let AR = J * (AP - G);
            let AS = (Lanes([0.0, AQ]) - Lanes([I, 0.0])) * J;
            let AU = J * (node_potentials[8] - AP);
            let AV = (Lanes([AT, 0.0]) - Lanes([0.0, AQ])) * J;
            let AW = Lanes([Z[0], Z[1], 0.0]) + Lanes([0.0, P[0], P[1]]);
            let AX = Lanes([AW[0], AW[1], 0.0, AW[2]]) - Lanes([0.0, 0.0, AB[0], AB[1]]);
            let AY = ((Y + O) - AA) - AR;
            let AZ = Lanes([AX[0], AX[1], AX[2], AX[3], 0.0]) - Lanes([0.0, 0.0, AS[0], 0.0, AS[1]]);
            let BB = AO * BA;
            let BC = Lanes([BB[0], BB[1], 0.0]) + Lanes([0.0, AJ[0], AJ[1]]);
            let BD = Lanes([BC[0], BC[1], BC[2], 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, AZ[0], AZ[1], AZ[2], AZ[3], AZ[4]]);
            let BE = (((-AN) + AI) + AY) - AU;
            let BF = Lanes([BD[0], BD[1], BD[2], BD[3], BD[4], BD[5], 0.0, BD[6]]) - Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, AV[0], AV[1]]);
            let BG = AN + BE;
            let BH = Lanes([AO[0], AO[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + BF;
            let BJ = O * BI;
            let BK = P * BI;
            let BM = if BJ < BL { 1.0 } else { 0.0 };
            let BT;
            let BU;
            if BM != 0.0 {
                let BN = BJ.exp();
                let BO = BK * BN;
                BT = BN;
                BU = BO;
            } else {
                let BP = BL.exp();
                let BR = BP * (BQ + (BJ - BL));
                let BS = BK * BP;
                BT = BR;
                BU = BS;
            }
            let BV = S * BI;
            let BW = T * BI;
            let BY = BV / BX;
            let BZ = BW / BX;
            let CA = if BY < BL { 1.0 } else { 0.0 };
            let CG;
            let CH;
            if CA != 0.0 {
                let CB = BY.exp();
                let CC = BZ * CB;
                CG = CB;
                CH = CC;
            } else {
                let CD = BL.exp();
                let CE = CD * (BQ + (BY - BL));
                let CF = BZ * CD;
                CG = CE;
                CH = CF;
            }
            let CI = AY * BI;
            let CJ = AZ * BI;
            let CK = if CI < BL { 1.0 } else { 0.0 };
            let CQ;
            let CR;
            if CK != 0.0 {
                let CL = CI.exp();
                let CM = CJ * CL;
                CQ = CL;
                CR = CM;
            } else {
                let CN = BL.exp();
                let CO = CN * (BQ + (CI - BL));
                let CP = CJ * CN;
                CQ = CO;
                CR = CP;
            }
            let CS = Y * BI;
            let CT = Z * BI;
            let CU = if CS < BL { 1.0 } else { 0.0 };
            let DA;
            let DB;
            if CU != 0.0 {
                let CV = CS.exp();
                let CW = CT * CV;
                DA = CV;
                DB = CW;
            } else {
                let CX = BL.exp();
                let CY = CX * (BQ + (CS - BL));
                let CZ = CT * CX;
                DA = CY;
                DB = CZ;
            }
            let DC = BG * BI;
            let DD = BH * BI;
            let DE = if DC < BL { 1.0 } else { 0.0 };
            let DK;
            let DL;
            if DE != 0.0 {
                let DF = DC.exp();
                let DG = DD * DF;
                DK = DF;
                DL = DG;
            } else {
                let DH = BL.exp();
                let DI = DH * (BQ + (DC - BL));
                let DJ = DD * DH;
                DK = DI;
                DL = DJ;
            }
            let DN = (BG - DM) * BI;
            let DO = if DN < BL { 1.0 } else { 0.0 };
            let DU;
            let DV;
            if DO != 0.0 {
                let DP = DN.exp();
                let DQ = DD * DP;
                DU = DP;
                DV = DQ;
            } else {
                let DR = BL.exp();
                let DS = DR * (BQ + (DN - BL));
                let DT = DD * DR;
                DU = DS;
                DV = DT;
            }
            let DW = (AY - DM) * BI;
            let DX = if DW < BL { 1.0 } else { 0.0 };
            let ED;
            let EE;
            if DX != 0.0 {
                let DY = DW.exp();
                let DZ = CJ * DY;
                ED = DY;
                EE = DZ;
            } else {
                let EA = BL.exp();
                let EB = EA * (BQ + (DW - BL));
                let EC = CJ * EA;
                ED = EB;
                EE = EC;
            }
            let EF = (O - DM) * BI;
            let EG = if EF < BL { 1.0 } else { 0.0 };
            let EM;
            let EN;
            if EG != 0.0 {
                let EH = EF.exp();
                let EI = BK * EH;
                EM = EH;
                EN = EI;
            } else {
                let EJ = BL.exp();
                let EK = EJ * (BQ + (EF - BL));
                let EL = BK * EJ;
                EM = EK;
                EN = EL;
            }
            let EO = (K - DM) * BI;
            let EP = L * BI;
            let EQ = if EO < BL { 1.0 } else { 0.0 };
            let EW;
            let EX;
            if EQ != 0.0 {
                let ER = EO.exp();
                let ES = EP * ER;
                EW = ER;
                EX = ES;
            } else {
                let ET = BL.exp();
                let EU = ET * (BQ + (EO - BL));
                let EV = EP * ET;
                EW = EU;
                EX = EV;
            }
            let EZ = (BQ + (EY * EM)).sqrt();
            let FC = (EN * EY) * (FB / (FA * EZ));
            let FD = (BQ + (EY * EW)).sqrt();
            let FE = (EX * EY) * (FB / (FA * FD));
            let FG = BQ + FD;
            let FH = (FF * EW) / FG;
            let FI = ((EX * FF) - (FE * FH)) / FG;
            let FK = if FH < FJ { 1.0 } else { 0.0 };
            let FM;
            let FN;
            if FK != 0.0 {
                FM = FJ;
                FN = FL;
            } else {
                FM = FH;
                FN = FI;
            }
            let FO = Lanes([FC[0], 0.0, FC[1]]);
            let FP = EZ + BQ;
            let FQ = FP / FG;
            let FR = FE * FQ;
            let FT = FS * ((EZ - FD) - (FQ.ln()));
            let FU = ((FO - Lanes([FE[0], FE[1], 0.0])) - (((FO - Lanes([FR[0], FR[1], 0.0])) / FG) * (FB / FQ))) * FS;
            let FV = Lanes([0.0, AB[0], AB[1]]);
            let FX = (FT + AA) / FW;
            let FY = (FU + FV) / FW;
            let FZ = if FX > A { 1.0 } else { 0.0 };
            let GF;
            let GG;
            let GH;
            let GI;
            let GJ;
            let GK;
            let GL;
            let GM;
            let GN;
            let GO;
            let GP;
            let GQ;
            let GR;
            let GS;
            if FZ != 0.0 {
                let GB = if K < GA { 1.0 } else { 0.0 };
                let HB;
                let HC;
                if GB != 0.0 {
                    HB = K;
                    HC = L;
                } else {
                    let GY = BQ + (K - GA);
                    let GZ = L * (FB / GY);
                    let HA = GA + (GY.ln());
                    HB = HA;
                    HC = GZ;
                }
                let HD = FF * FS;
                let HF = (HE * FX) * FW;
                let HG = (FY * HE) * FW;
                let HH = (HF * BI) + BQ;
                let HI = (DM + (HD * (HH.ln()))) - HB;
                let HJ = (((HG * BI) * (FB / HH)) * HD) - Lanes([HC[0], HC[1], 0.0]);
                let HL = HK * DM;
                let HM = HL * HL;
                let HN = HI * HI;
                let HO = HJ * HI;
                let HP = HO + HO;
                let HQ = if HI < A { 1.0 } else { 0.0 };
                let HY;
                let HZ;
                if HQ != 0.0 {
                    let HR = (HN + HM).sqrt();
                    let HS = HR - HI;
                    let HT = (HE * HM) / HS;
                    let HU = ((((HP * (FB / (FA * HR))) - HJ) * HT) * BA) / HS;
                    HY = HT;
                    HZ = HU;
                } else {
                    let HV = (HN + HM).sqrt();
                    let HW = HE * (HV + HI);
                    let HX = ((HP * (FB / (FA * HV))) + HJ) * HE;
                    HY = HW;
                    HZ = HX;
                }
                let IC = IA * IB;
                let ID = HY + IC;
                let IE = IB * (HY + (IA * FW));
                let IF = (HY * ID) / IE;
                let IG = (((HZ * ID) + (HZ * HY)) - ((HZ * IB) * IF)) / IE;
                let IH = FX / IF;
                let II = (FY - (IG * IH)) / IF;
                let IK = (IH - BQ) / IJ;
                let IL = II / IJ;
                let IM = if IH < BQ { 1.0 } else { 0.0 };
                let IV;
                let IW;
                if IM != 0.0 {
                    let IN = IK.exp();
                    let IO = BQ + IN;
                    let IP = ((IL * IN) * (FB / IO)) * IJ;
                    let IQ = BQ + (IJ * (IO.ln()));
                    IV = IQ;
                    IW = IP;
                } else {
                    let IR = (-IK).exp();
                    let IS = BQ + IR;
                    let IT = IH + (IJ * (IS.ln()));
                    let IU = II + ((((IL * BA) * IR) * (FB / IS)) * IJ);
                    IV = IT;
                    IW = IU;
                }
                let IX = BQ + (IJ * ((BQ + ((-1e0f64 / IJ).exp())).ln()));
                let IY = IV / IX;
                let IZ = IW / IX;
                let JA = HY / IC;
                let JB = HZ / IC;
                let JC = EY * IY;
                let JD = JC * JA;
                let JE = BQ + JA;
                let JF = (BQ + (JD * JE)).sqrt();
                let JG = FF * IY;
                let JH = JG * JE;
                let JI = (BQ + JF) / JH;
                let JJ = (((((((IZ * EY) * JA) + (JB * JC)) * JE) + (JB * JD)) * (FB / (FA * JF))) - ((((IZ * FF) * JE) + (JB * JG)) * JI)) / JH;
                let JK = FM * JI;
                let JL = FN * JI;
                let JM = Lanes([JL[0], JL[1], 0.0]) + (JJ * FM);
                let JN = BQ + JK;
                let JO = ((BQ - JI) + JK) / JN;
                let JP = (((JJ * BA) + JM) - (JM * JO)) / JN;
                let JQ = (HF * JO) * BI;
                let JR = ((HG * JO) + (JP * HF)) * BI;
                let JS = (FM + JQ) + BQ;
                let JT = FN * JS;
                let JU = (FF * JQ) + (FM * JS);
                let JV = (JR * FF) + (Lanes([JT[0], JT[1], 0.0]) + ((Lanes([FN[0], FN[1], 0.0]) + JR) * FM));
                let JW = HE * (JQ - BQ);
                let JX = JR * HE;
                let JY = JX * JW;
                let JZ = (JW * JW) + JU;
                let KA = (JY + JY) + JV;
                let KB = if JQ >= BQ { 1.0 } else { 0.0 };
                let KJ;
                let KK;
                if KB != 0.0 {
                    let KC = JZ.sqrt();
                    let KD = JW + KC;
                    let KE = JX + (KA * (FB / (FA * KC)));
                    KJ = KD;
                    KK = KE;
                } else {
                    let KF = JZ.sqrt();
                    let KG = KF - JW;
                    let KH = JU / KG;
                    let KI = (JV - (((KA * (FB / (FA * KF))) - JX) * KH)) / KG;
                    KJ = KH;
                    KK = KI;
                }
                let KM = if KJ < KL { 1.0 } else { 0.0 };
                let KO;
                let KP;
                if KM != 0.0 {
                    KO = KL;
                    KP = KN;
                } else {
                    KO = KJ;
                    KP = KK;
                }
                let KQ = KO + BQ;
                let KR = (DM * BI).exp();
                let KS = (KO * KQ) * KR;
                let KT = ((KP * KQ) + (KP * KO)) * KR;
                let KU = HE * IB;
                let KV = KU * (FX - IA);
                let KW = FY * KU;
                let KX = (IB * FW) * IA;
                let KY = KW * KV;
                let KZ = ((KV * KV) + (KX * FX)).sqrt();
                let LA = KV + KZ;
                let LB = KW + (((KY + KY) + (FY * KX)) * (FB / (FA * KZ)));
                let LC = if parameters[72] == A { 1.0 } else { 0.0 };
                let LK;
                let LL;
                if LC != 0.0 {
                    let LF = LD * LE;
                    LK = LF;
                    LL = KN;
                } else {
                    let LG = FX + IF;
                    let LH = (FF * FX) / LG;
                    let LI = LD * (LE + LH);
                    let LJ = (((FY * FF) - ((FY + IG) * LH)) / LG) * LD;
                    LK = LI;
                    LL = LJ;
                }
                let LM = IA + FX;
                let LN = (IA * FX) / LM;
                let LO = ((FY * IA) - (FY * LN)) / LM;
                let LP = IA / LM;
                let LQ = ((FY * LP) * BA) / LM;
                GF = LA;
                GG = LK;
                GH = LP;
                GI = KS;
                GJ = JO;
                GK = LN;
                GL = KO;
                GM = LB;
                GN = LL;
                GO = LQ;
                GP = KT;
                GQ = JP;
                GR = LO;
                GS = KP;
            } else {
                let GC = (FF * EM) / FP;
                let GD = ((EN * FF) - (FC * GC)) / FP;
                let GE = if (if (AA.abs()) < (1e-5f64 * FS) { 1.0 } else { 0.0 }) != 0.0 || (if (FT.abs()) < ((1e-40f64 * FS) * (EZ + FD)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let LZ;
                let MA;
                if GE != 0.0 {
                    let LR = HE * (GC + FM);
                    let LS = (Lanes([GD[0], 0.0, GD[1]]) + Lanes([FN[0], FN[1], 0.0])) * HE;
                    let LT = LR + BQ;
                    let LU = LR / LT;
                    let LV = (LS - (LS * LU)) / LT;
                    LZ = LU;
                    MA = LV;
                } else {
                    let LW = (FT + O) - K;
                    let LX = FT / LW;
                    let LY = (FU - (((FU + Lanes([P[0], 0.0, P[1]])) - Lanes([L[0], L[1], 0.0])) * LX)) / LW;
                    LZ = LX;
                    MA = LY;
                }
                let MB = LE * LD;
                let MC = BQ - (FX / IA);
                let MD = (FY / IA) * BA;
                let ME = Lanes([BU[0], 0.0, BU[1]]);
                let MF = Lanes([GD[0], 0.0, GD[1]]);
                GF = AA;
                GG = MB;
                GH = MC;
                GI = BT;
                GJ = LZ;
                GK = FX;
                GL = GC;
                GM = FV;
                GN = KN;
                GO = MD;
                GP = ME;
                GQ = MA;
                GR = FY;
                GS = MF;
            }
            let GV = (S - GT) / GU;
            let GW = T / GU;
            let GX = if S < GT { 1.0 } else { 0.0 };
            let MO;
            let MP;
            if GX != 0.0 {
                let MG = GV.exp();
                let MH = BQ + MG;
                let MI = S - (GU * (MH.ln()));
                let MJ = T - (((GW * MG) * (FB / MH)) * GU);
                MO = MI;
                MP = MJ;
            } else {
                let MK = (-GV).exp();
                let ML = BQ + MK;
                let MM = GT - (GU * (ML.ln()));
                let MN = ((((GW * BA) * MK) * (FB / ML)) * GU) * BA;
                MO = MM;
                MP = MN;
            }
            let MR = BQ - (MO * MQ);
            let MS = (MP * MQ) * BA;
            let MU = MR.powf(MT);
            let MW = MS * (MT * (MR.powf(MV)));
            let MZ = (MX * (BQ - MU)) + (MY * (S - MO));
            let NA = ((MW * BA) * MX) + ((T - MP) * MY);
            let NE;
            let NF;
            if NB != 0.0 {
                let NC = Lanes([L[0], L[1], 0.0]);
                NE = K;
                NF = NC;
            } else {
                let NN;
                let NO;
                if ND != 0.0 {
                    let NK = K + GF;
                    let NL = Lanes([L[0], L[1], 0.0]) + GM;
                    NN = NK;
                    NO = NL;
                } else {
                    let NM = Lanes([P[0], 0.0, P[1]]);
                    NN = O;
                    NO = NM;
                }
                NE = NN;
                NF = NO;
            }
            let NH = (NE - NG) / GG;
            let NI = (NF - (GN * NH)) / GG;
            let NJ = if NE < NG { 1.0 } else { 0.0 };
            let NZ;
            let OA;
            if NJ != 0.0 {
                let NP = NH.exp();
                let NQ = BQ + NP;
                let NR = NQ.ln();
                let NS = NE - (GG * NR);
                let NT = NF - ((GN * NR) + (((NI * NP) * (FB / NQ)) * GG));
                NZ = NS;
                OA = NT;
            } else {
                let NU = (-NH).exp();
                let NV = BQ + NU;
                let NW = NV.ln();
                let NX = NG - (GG * NW);
                let NY = ((GN * NW) + ((((NI * BA) * NU) * (FB / NV)) * GG)) * BA;
                NZ = NX;
                OA = NY;
            }
            let OC = GH.powf(OB);
            let OD = GO * (OB * (GH.powf(staged[139])));
            let OE = BQ - (NZ / LD);
            let OG = OE.powf(OF);
            let OK = OC * OJ;
            let OL = NE - NZ;
            let OO = L * ON;
            let OP = (OM * ((OI * (BQ - (OC * OG))) + (OK * OL))) + (ON * K);
            let OQ = ((((((OD * OG) + ((((OA / LD) * BA) * (OF * (OE.powf(OH)))) * OC)) * BA) * OI) + (((OD * OJ) * OL) + ((NF - OA) * OK))) * OM) + Lanes([OO[0], OO[1], 0.0]);
            let OS = OR * CG;
            let OT = CH * OR;
            let OU = (BQ + OS).sqrt();
            let OV = OT * (FB / (FA * OU));
            let OW = BQ + OU;
            let OX = OS / OW;
            let OY = (OT - (OV * OX)) / OW;
            let PA = GI.powf(OZ);
            let PB = GP * (OZ * (GI.powf(staged[141])));
            let PC = OR * PA;
            let PD = PB * OR;
            let PE = (BQ + PC).sqrt();
            let PF = BQ + PE;
            let PG = PC / PF;
            let PH = (PD - ((PD * (FB / (FA * PE))) * PG)) / PF;
            let PX;
            let PY;
            if PI != 0.0 {
                let PK = NA / PJ;
                let PM = OQ / PL;
                let PN = (BQ + (MZ / PJ)) + (OP / PL);
                let PO = Lanes([PK[0], PK[1], 0.0, 0.0]) + Lanes([0.0, PM[0], PM[1], PM[2]]);
                PX = PN;
                PY = PO;
            } else {
                let PQ = ((((MZ / PJ) + BQ) * PP) * BI).exp();
                let PR = (((NA / PJ) * PP) * BI) * PQ;
                let PS = ((((-OP) / PL) * PP) * BI).exp();
                let PT = ((((OQ * BA) / PL) * PP) * BI) * PS;
                let PV = (PQ - PS) / PU;
                let PW = (Lanes([PR[0], PR[1], 0.0, 0.0]) - Lanes([0.0, PT[0], PT[1], PT[2]])) / PU;
                PX = PV;
                PY = PW;
            }
            let PZ = PX * PX;
            let QA = PY * PX;
            let QB = QA + QA;
            let QC = if PX < A { 1.0 } else { 0.0 };
            let QL;
            let QM;
            if QC != 0.0 {
                let QE = (PZ + QD).sqrt();
                let QF = QE - PX;
                let QG = 5.000000000000001e-3f64 / QF;
                let QH = ((((QB * (FB / (FA * QE))) - PY) * QG) * BA) / QF;
                QL = QG;
                QM = QH;
            } else {
                let QI = (PZ + QD).sqrt();
                let QJ = HE * (QI + PX);
                let QK = ((QB * (FB / (FA * QI))) + PY) * HE;
                QL = QJ;
                QM = QK;
            }
            let QN = (Lanes([OY[0], OY[1], 0.0, 0.0]) + Lanes([0.0, PH[0], PH[1], PH[2]])) * HE;
            let QO = BQ + (HE * (OX + PG));
            let QP = QL * QO;
            let QQ = (QM * QO) + (QN * QL);
            let QS = QR * PA;
            let QT = PB * QR;
            let QV = QU * CG;
            let QW = CH * QU;
            let QX = Lanes([QW[0], QW[1], 0.0, 0.0]);
            let QY = Lanes([0.0, QT[0], QT[1], QT[2]]);
            let QZ = (QV - QS) / QP;
            let RA = ((QX - QY) - (QQ * QZ)) / QP;
            let RC = S / RB;
            let RD = T / RB;
            let RE = if S < A { 1.0 } else { 0.0 };
            let RN;
            let RO;
            if RE != 0.0 {
                let RF = RC.exp();
                let RG = BQ + RF;
                let RH = RB * (RG.ln());
                let RI = ((RD * RF) * (FB / RG)) * RB;
                RN = RH;
                RO = RI;
            } else {
                let RJ = (-RC).exp();
                let RK = BQ + RJ;
                let RL = S + (RB * (RK.ln()));
                let RM = T + ((((RD * BA) * RJ) * (FB / RK)) * RB);
                RN = RL;
                RO = RM;
            }
            let RQ = RN / RP;
            let RR = RO / RP;
            let RS = if RQ < BL { 1.0 } else { 0.0 };
            let RY;
            let RZ;
            if RS != 0.0 {
                let RT = RQ.exp();
                let RU = RR * RT;
                RY = RT;
                RZ = RU;
            } else {
                let RV = BL.exp();
                let RW = RV * (BQ + (RQ - BL));
                let RX = RR * RV;
                RY = RW;
                RZ = RX;
            }
            let SB = SA * (RY - BQ);
            let SC = RZ * SA;
            let SF = (S - SD) / SE;
            let SG = T / SE;
            let SH = if S < SD { 1.0 } else { 0.0 };
            let SQ;
            let SR;
            if SH != 0.0 {
                let SI = SF.exp();
                let SJ = BQ + SI;
                let SK = S - (SE * (SJ.ln()));
                let SL = T - (((SG * SI) * (FB / SJ)) * SE);
                SQ = SK;
                SR = SL;
            } else {
                let SM = (-SF).exp();
                let SN = BQ + SM;
                let SO = SD - (SE * (SN.ln()));
                let SP = ((((SG * BA) * SM) * (FB / SN)) * SE) * BA;
                SQ = SO;
                SR = SP;
            }
            let ST = SS * SQ;
            let SU = SD - SQ;
            let SV = SU * SU;
            let SW = ST * SV;
            let SX = ((SR * SS) * SV) + (((SR * BA) * (FF * SU)) * ST);
            let SZ = BV / SY;
            let TA = BW / SY;
            let TB = if SZ < BL { 1.0 } else { 0.0 };
            let TH;
            let TI;
            if TB != 0.0 {
                let TC = SZ.exp();
                let TD = TA * TC;
                TH = TC;
                TI = TD;
            } else {
                let TE = BL.exp();
                let TF = TE * (BQ + (SZ - BL));
                let TG = TA * TE;
                TH = TF;
                TI = TG;
            }
            let TN;
            let TO;
            if C != 0.0 {
                let TK = (S - TJ) * BI;
                let TL = if TK < BL { 1.0 } else { 0.0 };
                let UA;
                let UB;
                if TL != 0.0 {
                    let TV = TK.exp();
                    let TW = BW * TV;
                    UA = TV;
                    UB = TW;
                } else {
                    let TX = BL.exp();
                    let TY = TX * (BQ + (TK - BL));
                    let TZ = BW * TX;
                    UA = TY;
                    UB = TZ;
                }
                let UC = RA / QU;
                let UD = (QZ / QU) - 1e3f64;
                let UF = if UD < UE { 1.0 } else { 0.0 };
                let UL;
                let UM;
                if UF != 0.0 {
                    let UG = UD.exp();
                    let UH = UC * UG;
                    UL = UG;
                    UM = UH;
                } else {
                    let UJ = UI * (BQ + (UD - UE));
                    let UK = UC * UI;
                    UL = UJ;
                    UM = UK;
                }
                let UN = TH - BQ;
                let UP = TI * UO;
                let UR = (BQ + (EY * UA)).sqrt();
                let US = BQ + UR;
                let UT = (UQ * UN) / US;
                let UU = BQ + (OP / PL);
                let UV = (((TI * UQ) - (((UB * EY) * (FB / (FA * UR))) * UT)) / US) * UU;
                let UW = (OQ / PL) * UT;
                let UY = UX * (GI - BQ);
                let UZ = (GP * UX) * UL;
                let VA = BQ + UL;
                let VB = (UY * UL) / VA;
                let VC = ((UO * UN) + (UT * UU)) + VB;
                let VD = (Lanes([UP[0], UP[1], 0.0, 0.0]) + (Lanes([UV[0], UV[1], 0.0, 0.0]) + Lanes([0.0, UW[0], UW[1], UW[2]]))) + (((Lanes([0.0, UZ[0], UZ[1], UZ[2]]) + (UM * UY)) - (UM * VB)) / VA);
                TN = VC;
                TO = VD;
            } else {
                let VP;
                let VQ;
                if TM != 0.0 {
                    let VE = UO * (TH - BQ);
                    let VF = TI * UO;
                    let VG = Lanes([VF[0], VF[1], 0.0, 0.0]);
                    VP = VE;
                    VQ = VG;
                } else {
                    let VI = TI * VH;
                    let VK = VJ * ((TH + GI) - FF);
                    let VL = BQ + (OP / PL);
                    let VM = (OQ / PL) * VK;
                    let VN = UO * ((VH * (TH - BQ)) + (VK * VL));
                    let VO = (Lanes([VI[0], VI[1], 0.0, 0.0]) + ((((Lanes([TI[0], TI[1], 0.0, 0.0]) + Lanes([0.0, GP[0], GP[1], GP[2]])) * VJ) * VL) + Lanes([0.0, VM[0], VM[1], VM[2]]))) * UO;
                    VP = VN;
                    VQ = VO;
                }
                TN = VP;
                TO = VQ;
            }
            let TP = W * BI;
            let TQ = X * BI;
            let TS = TP / TR;
            let TT = TQ / TR;
            let TU = if TS < BL { 1.0 } else { 0.0 };
            let VW;
            let VX;
            if TU != 0.0 {
                let VR = TS.exp();
                let VS = TT * VR;
                VW = VR;
                VX = VS;
            } else {
                let VT = BL.exp();
                let VU = VT * (BQ + (TS - BL));
                let VV = TT * VT;
                VW = VU;
                VX = VV;
            }
            let WD;
            let WE;
            if C != 0.0 {
                let VY = (W - TJ) * BI;
                let VZ = if VY < BL { 1.0 } else { 0.0 };
                let WO;
                let WP;
                if VZ != 0.0 {
                    let WJ = VY.exp();
                    let WK = TQ * WJ;
                    WO = WJ;
                    WP = WK;
                } else {
                    let WL = BL.exp();
                    let WM = WL * (BQ + (VY - BL));
                    let WN = TQ * WL;
                    WO = WM;
                    WP = WN;
                }
                let WQ = VW - BQ;
                let WS = (BQ + (EY * WO)).sqrt();
                let WT = BQ + WS;
                let WU = (WR * WQ) / WT;
                let WV = (WA * WQ) + WU;
                let WW = (VX * WA) + (((VX * WR) - (((WP * EY) * (FB / (FA * WS))) * WU)) / WT);
                WD = WV;
                WE = WW;
            } else {
                let WB = WA * (VW - BQ);
                let WC = VX * WA;
                WD = WB;
                WE = WC;
            }
            let WG = BV / WF;
            let WH = BW / WF;
            let WI = if WG < BL { 1.0 } else { 0.0 };
            let XC;
            let XD;
            if WI != 0.0 {
                let WX = WG.exp();
                let WY = WH * WX;
                XC = WX;
                XD = WY;
            } else {
                let WZ = BL.exp();
                let XA = WZ * (BQ + (WG - BL));
                let XB = WH * WZ;
                XC = XA;
                XD = XB;
            }
            let XF = XE * (XC - BQ);
            let XG = XD * XE;
            let XI = TP / XH;
            let XJ = TQ / XH;
            let XK = if XI < BL { 1.0 } else { 0.0 };
            let XQ;
            let XR;
            if XK != 0.0 {
                let XL = XI.exp();
                let XM = XJ * XL;
                XQ = XL;
                XR = XM;
            } else {
                let XN = BL.exp();
                let XO = XN * (BQ + (XI - BL));
                let XP = XJ * XN;
                XQ = XO;
                XR = XP;
            }
            let XT = XS * (XQ - BQ);
            let XU = XR * XS;
            let XW = CI / XV;
            let XX = CJ / XV;
            let XY = if XW < BL { 1.0 } else { 0.0 };
            let YE;
            let YF;
            if XY != 0.0 {
                let XZ = XW.exp();
                let YA = XX * XZ;
                YE = XZ;
                YF = YA;
            } else {
                let YB = BL.exp();
                let YC = YB * (BQ + (XW - BL));
                let YD = XX * YB;
                YE = YC;
                YF = YD;
            }
            let YH = YG * (YE - BQ);
            let YI = YF * YG;
            let YK = TP / YJ;
            let YL = TQ / YJ;
            let YM = if YK < BL { 1.0 } else { 0.0 };
            let YS;
            let YT;
            if YM != 0.0 {
                let YN = YK.exp();
                let YO = YL * YN;
                YS = YN;
                YT = YO;
            } else {
                let YP = BL.exp();
                let YQ = YP * (BQ + (YK - BL));
                let YR = YL * YP;
                YS = YQ;
                YT = YR;
            }
            let YV = YU * (YS - BQ);
            let YW = YT * YU;
            let YX = if staged[76] != 0.0 && RE != 0.0 { 1.0 } else { 0.0 };
            let ZG;
            let ZH;
            if YX != 0.0 {
                let YY = FF * MU;
                let ZA = YZ / YY;
                let ZC = ZB * (BQ - ZA);
                let ZD = (((((MW * FF) * ZA) * BA) / YY) * BA) * ZB;
                let ZE = if ZC < BL { 1.0 } else { 0.0 };
                let ZO;
                let ZP;
                if ZE != 0.0 {
                    let ZJ = ZC.exp();
                    let ZK = ZD * ZJ;
                    ZO = ZJ;
                    ZP = ZK;
                } else {
                    let ZL = BL.exp();
                    let ZM = ZL * (BQ + (ZC - BL));
                    let ZN = ZD * ZL;
                    ZO = ZM;
                    ZP = ZN;
                }
                let ZQ = S * MQ;
                let ZR = T * MQ;
                let ZS = ZR * ZQ;
                let ZU = ((ZQ * ZQ) + ZT).sqrt();
                let ZW = -2e0f64 - ZV;
                let ZX = ZU.powf(ZW);
                let ZY = ZV - BQ;
                let AAA = ZZ * ZQ;
                let AAB = AAA * ZQ;
                let AAC = ZY + ZQ;
                let AAD = (ZV * ((BQ - (ZV * ZV)) - ((MY * ZQ) * ZY))) - (AAB * AAC);
                let AAG = AAF * ((ZX * AAD) * AAE);
                let AAH = ((S * YZ) * ZB) / AAG;
                let AAI = (((T * YZ) * ZB) - ((((((((ZS + ZS) * (FB / (FA * ZU))) * (ZW * (ZU.powf((ZW - FB))))) * AAD) + ((((((ZR * MY) * ZY) * BA) * ZV) - (((((ZR * ZZ) * ZQ) + (ZR * AAA)) * AAC) + (ZR * AAB))) * ZX)) * AAE) * AAF) * AAH)) / AAG;
                let AAJ = if AAH < -1e-3f64 { 1.0 } else { 0.0 };
                let AAU;
                let AAV;
                if AAJ != 0.0 {
                    let AAK = if AAH < BL { 1.0 } else { 0.0 };
                    let ABH;
                    let ABI;
                    if AAK != 0.0 {
                        let ABC = AAH.exp();
                        let ABD = AAI * ABC;
                        ABH = ABC;
                        ABI = ABD;
                    } else {
                        let ABE = BL.exp();
                        let ABF = ABE * (BQ + (AAH - BL));
                        let ABG = AAI * ABE;
                        ABH = ABF;
                        ABI = ABG;
                    }
                    let ABJ = -S;
                    let ABK = (BQ - ABH) / AAH;
                    let ABL = BQ + ABK;
                    let ABM = ABJ * ABL;
                    let ABN = ((T * BA) * ABL) + ((((ABI * BA) - (AAI * ABK)) / AAH) * ABJ);
                    AAU = ABM;
                    AAV = ABN;
                } else {
                    let AAL = S * HE;
                    let AAM = AAL * AAH;
                    let AAO = AAH * AAN;
                    let AAQ = BQ + (AAP * AAH);
                    let AAR = BQ + (AAO * AAQ);
                    let AAS = AAM * AAR;
                    let AAT = ((((T * HE) * AAH) + (AAI * AAL)) * AAR) + ((((AAI * AAN) * AAQ) + ((AAI * AAP) * AAO)) * AAM);
                    AAU = AAS;
                    AAV = AAT;
                }
                let AAW = FF * staged[80];
                let AAX = AAW * AAU;
                let AAY = AAX * MU;
                let ABA = ((AAY * ZO) * MQ) * AAZ;
                let ABB = ((((((AAV * AAW) * MU) + (MW * AAX)) * ZO) + (ZP * AAY)) * MQ) * AAZ;
                ZG = ABA;
                ZH = ABB;
            } else {
                ZG = A;
                ZH = ZF;
            }
            let ZI = if staged[82] != 0.0 && (if K < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ACB;
            let ACC;
            if ZI != 0.0 {
                let ABP = K * ABO;
                let ABQ = L * ABO;
                let ABR = BQ - ABP;
                let ABS = ABR.powf(OF);
                let ABT = (ABQ * BA) * (OF * (ABR.powf(OH)));
                let ABU = FF * ABS;
                let ABW = ABV / ABU;
                let ABY = ABX * (BQ - ABW);
                let ABZ = (((((ABT * FF) * ABW) * BA) / ABU) * BA) * ABX;
                let ACA = if ABY < BL { 1.0 } else { 0.0 };
                let ADB;
                let ADC;
                if ACA != 0.0 {
                    let ACW = ABY.exp();
                    let ACX = ABZ * ACW;
                    ADB = ACW;
                    ADC = ACX;
                } else {
                    let ACY = BL.exp();
                    let ACZ = ACY * (BQ + (ABY - BL));
                    let ADA = ABZ * ACY;
                    ADB = ACZ;
                    ADC = ADA;
                }
                let ADD = ABQ * ABP;
                let ADE = ((ABP * ABP) + ZT).sqrt();
                let ADG = -2e0f64 - ADF;
                let ADH = ADE.powf(ADG);
                let ADI = ADF - BQ;
                let ADJ = ZZ * ABP;
                let ADK = ADJ * ABP;
                let ADL = ADI + ABP;
                let ADM = (ADF * ((BQ - (ADF * ADF)) - ((MY * ABP) * ADI))) - (ADK * ADL);
                let ADO = ADN * ((ADH * ADM) * AAE);
                let ADP = ((K * ABV) * ABX) / ADO;
                let ADQ = (((L * ABV) * ABX) - ((((((((ADD + ADD) * (FB / (FA * ADE))) * (ADG * (ADE.powf((ADG - FB))))) * ADM) + ((((((ABQ * MY) * ADI) * BA) * ADF) - (((((ABQ * ZZ) * ABP) + (ABQ * ADJ)) * ADL) + (ABQ * ADK))) * ADH)) * AAE) * ADN) * ADP)) / ADO;
                let ADR = if ADP < -1e-3f64 { 1.0 } else { 0.0 };
                let AEA;
                let AEB;
                if ADR != 0.0 {
                    let ADS = if ADP < BL { 1.0 } else { 0.0 };
                    let AEN;
                    let AEO;
                    if ADS != 0.0 {
                        let AEI = ADP.exp();
                        let AEJ = ADQ * AEI;
                        AEN = AEI;
                        AEO = AEJ;
                    } else {
                        let AEK = BL.exp();
                        let AEL = AEK * (BQ + (ADP - BL));
                        let AEM = ADQ * AEK;
                        AEN = AEL;
                        AEO = AEM;
                    }
                    let AEP = -K;
                    let AEQ = (BQ - AEN) / ADP;
                    let AER = BQ + AEQ;
                    let AES = AEP * AER;
                    let AET = ((L * BA) * AER) + ((((AEO * BA) - (ADQ * AEQ)) / ADP) * AEP);
                    AEA = AES;
                    AEB = AET;
                } else {
                    let ADT = K * HE;
                    let ADU = ADT * ADP;
                    let ADV = ADP * AAN;
                    let ADW = BQ + (AAP * ADP);
                    let ADX = BQ + (ADV * ADW);
                    let ADY = ADU * ADX;
                    let ADZ = ((((L * HE) * ADP) + (ADQ * ADT)) * ADX) + ((((ADQ * AAN) * ADW) + ((ADQ * AAP) * ADV)) * ADU);
                    AEA = ADY;
                    AEB = ADZ;
                }
                let AEC = FF * staged[87];
                let AED = AEC * AEA;
                let AEE = AED * ABS;
                let AEG = ((AEE * ADB) * ABO) * AEF;
                let AEH = ((((((AEB * AEC) * ABS) + (ABT * AED)) * ADB) + (ADC * AEE)) * ABO) * AEF;
                ACB = AEG;
                ACC = AEH;
            } else {
                ACB = A;
                ACC = FL;
            }
            let ACD = OR * CQ;
            let ACE = CR * OR;
            let ACF = EY * ED;
            let ACG = EE * EY;
            let ACH = (BQ + ACD).sqrt();
            let ACI = BQ + ACH;
            let ACJ = (ACD - OR) / ACI;
            let ACK = (ACE - ((ACE * (FB / (FA * ACH))) * ACJ)) / ACI;
            let ACL = (BQ + ACF).sqrt();
            let ACM = BQ + ACL;
            let ACN = ACF / ACM;
            let ACO = (ACG - ((ACG * (FB / (FA * ACL))) * ACN)) / ACM;
            let ACR = (BQ + (ACQ * CQ)).sqrt();
            let ACS = BQ + ACR;
            let ACT = (ACP * (CQ - BQ)) / ACS;
            let ACU = ((CR * ACP) - (((CR * ACQ) * (FB / (FA * ACR))) * ACT)) / ACS;
            let AFE;
            let AFF;
            let AFG;
            let AFH;
            let AFI;
            let AFJ;
            if ACV != 0.0 {
                let AEV = ACT * AEU;
                let AEW = ACU * AEU;
                let AEY = (BQ + (ACQ * DK)).sqrt();
                let AEZ = BQ + AEY;
                let AFA = (AEX * (DK - BQ)) / AEZ;
                let AFB = ((DL * AEX) - (((DL * ACQ) * (FB / (FA * AEY))) * AFA)) / AEZ;
                let AFQ;
                let AFR;
                if AFC != 0.0 {
                    let AFL = BG - staged[94];
                    let AFM = AFL * AFL;
                    let AFN = BH * AFL;
                    let AFO = AFN + AFN;
                    let AFP = if AFL < A { 1.0 } else { 0.0 };
                    let AGC;
                    let AGD;
                    if AFP != 0.0 {
                        let AFV = (AFM + AFU).sqrt();
                        let AFW = AFV - AFL;
                        let AFX = 6.05e-3f64 / AFW;
                        let AFY = ((((AFO * (FB / (FA * AFV))) - BH) * AFX) * BA) / AFW;
                        AGC = AFX;
                        AGD = AFY;
                    } else {
                        let AFZ = (AFM + AFU).sqrt();
                        let AGA = HE * (AFZ + AFL);
                        let AGB = ((AFO * (FB / (FA * AFZ))) + BH) * HE;
                        AGC = AGA;
                        AGD = AGB;
                    }
                    let AGF = (staged[96] + (AFA * AGE)) + AGC;
                    let AGG = AGC / AGF;
                    let AGH = (AGD - (((AFB * AGE) + AGD) * AGG)) / AGF;
                    AFQ = AGG;
                    AFR = AGH;
                } else {
                    AFQ = BQ;
                    AFR = AFD;
                }
                let AFS = AFQ * AFA;
                let AFT = (AFR * AFA) + (AFB * AFQ);
                AFE = AEV;
                AFF = AFS;
                AFG = AFQ;
                AFH = AEW;
                AFI = AFT;
                AFJ = AFR;
            } else {
                AFE = ACT;
                AFF = A;
                AFG = BQ;
                AFH = ACU;
                AFI = AFD;
                AFJ = AFD;
            }
            let AGR;
            let AGS;
            if AFK != 0.0 {
                let AGI = Y + K;
                let AGJ = Lanes([Z[0], Z[1], 0.0]) + Lanes([0.0, L[0], L[1]]);
                let AGM = (AGK * AGI) * AGL;
                let AGN = AGM * AGI;
                let AGO = (((AGJ * AGK) * AGL) * AGI) + (AGJ * AGM);
                let AGP = if (-1e0f64 * AGI) < A { 1.0 } else { 0.0 };
                let AHX;
                let AHY;
                if AGP != 0.0 {
                    let AHO = (AGN + AHN).sqrt();
                    let AHQ = AHO - (AHP * AGI);
                    let AHR = 5e-13f64 / AHQ;
                    let AHS = ((((AGO * (FB / (FA * AHO))) - (AGJ * AHP)) * AHR) * BA) / AHQ;
                    AHX = AHR;
                    AHY = AHS;
                } else {
                    let AHT = (AGN + AHN).sqrt();
                    let AHV = HE * (AHT + (AHU * AGI));
                    let AHW = ((AGO * (FB / (FA * AHT))) + (AGJ * AHU)) * HE;
                    AHX = AHV;
                    AHY = AHW;
                }
                let AIA = if AHX < AHZ { 1.0 } else { 0.0 };
                let AIK;
                let AIL;
                if AIA != 0.0 {
                    let AIC = AHX / AIB;
                    let AIE = BQ - (AIC.powf(AID));
                    let AIF = BQ / AIE;
                    let AIG = (((((AHY / AIB) * (AID * (AIC.powf((AID - FB))))) * BA) * AIF) * BA) / AIE;
                    AIK = AIF;
                    AIL = AIG;
                } else {
                    let AII = AHY * AIH;
                    let AIJ = staged[99] + ((AHX - AHZ) * AIH);
                    AIK = AIJ;
                    AIL = AII;
                }
                AGR = AIK;
                AGS = AIL;
            } else {
                AGR = BQ;
                AGS = AGQ;
            }
            let AGT = ACB * AGR;
            let AGU = ACC * AGR;
            let AGV = Lanes([0.0, AGU[0], AGU[1]]) + (AGS * ACB);
            let AGW = AFE * AGR;
            let AGX = AGS * AFE;
            let AGY = (AFH * AGR) + Lanes([AGX[0], AGX[1], AGX[2], 0.0, 0.0]);
            let AGZ = YH * AGR;
            let AHA = AGS * YH;
            let AHB = (YI * AGR) + Lanes([AHA[0], AHA[1], AHA[2], 0.0, 0.0]);
            let AHC = AFF * AGR;
            let AHD = AGS * AFF;
            let AHE = (AFI * AGR) + Lanes([0.0, 0.0, AHD[0], AHD[1], AHD[2], 0.0, 0.0, 0.0]);
            let AHF = NA / PJ;
            let AHG = OQ / PL;
            let AHH = (BQ + (MZ / PJ)) + (OP / PL);
            let AHI = Lanes([AHF[0], AHF[1], 0.0, 0.0]) + Lanes([0.0, AHG[0], AHG[1], AHG[2]]);
            let AHJ = AHH * AHH;
            let AHK = AHI * AHH;
            let AHL = AHK + AHK;
            let AHM = if AHH < A { 1.0 } else { 0.0 };
            let AIU;
            let AIV;
            if AHM != 0.0 {
                let AIN = (AHJ + AIM).sqrt();
                let AIO = AIN - AHH;
                let AIP = 5.000000000000001e-3f64 / AIO;
                let AIQ = ((((AHL * (FB / (FA * AIN))) - AHI) * AIP) * BA) / AIO;
                AIU = AIP;
                AIV = AIQ;
            } else {
                let AIR = (AHJ + AIM).sqrt();
                let AIS = HE * (AIR + AHH);
                let AIT = ((AHL * (FB / (FA * AIR))) + AHI) * HE;
                AIU = AIS;
                AIV = AIT;
            }
            let AIW = AIU * QO;
            let AIX = staged[100] / AIW;
            let AIY = ((((AIV * QO) + (QN * AIU)) * AIX) * BA) / AIW;
            let AJA = if AIX < AIZ { 1.0 } else { 0.0 };
            let AJC;
            let AJD;
            if AJA != 0.0 {
                AJC = AIZ;
                AJD = AJB;
            } else {
                AJC = AIX;
                AJD = AIY;
            }
            let AJE = MY * AJC;
            let AJF = AJD * MY;
            let AJH = (DB * AJG) + Z;
            let AJI = ((AJG * (DA - BQ)) + Y) / AJE;
            let AJJ = AJF * AJI;
            let AJK = (Lanes([0.0, AJH[0], AJH[1], 0.0, 0.0]) - Lanes([AJJ[0], 0.0, AJJ[1], AJJ[2], AJJ[3]])) / AJE;
            let AJL = if QZ > A { 1.0 } else { 0.0 };
            let AJO;
            let AJP;
            if AJL != 0.0 {
                let AJN = if AJM == BQ { 1.0 } else { 0.0 };
                let AJU;
                let AJV;
                if AJN != 0.0 {
                    let AJS = if K < AJR { 1.0 } else { 0.0 };
                    let AKB;
                    let AKC;
                    if AJS != 0.0 {
                        let AJY = (-QZ) / AJX;
                        let AJZ = (RA * BA) / AJX;
                        let AKA = if AJY < BL { 1.0 } else { 0.0 };
                        let AKI;
                        let AKJ;
                        if AKA != 0.0 {
                            let AKD = AJY.exp();
                            let AKE = AJZ * AKD;
                            AKI = AKD;
                            AKJ = AKE;
                        } else {
                            let AKF = BL.exp();
                            let AKG = AKF * (BQ + (AJY - BL));
                            let AKH = AJZ * AKF;
                            AKI = AKG;
                            AKJ = AKH;
                        }
                        let AKK = AJR - K;
                        let AKL = AKK * AKI;
                        let AKM = (L * BA) * AKI;
                        let AKN = Lanes([0.0, AKM[0], AKM[1], 0.0]) + (AKJ * AKK);
                        let AKP = -AKO;
                        let AKR = AKP * (AKL.powf(AKQ));
                        let AKS = (AKN * (AKQ * (AKL.powf((AKQ - FB))))) * AKP;
                        let AKT = if AKR < BL { 1.0 } else { 0.0 };
                        let AKZ;
                        let ALA;
                        if AKT != 0.0 {
                            let AKU = AKR.exp();
                            let AKV = AKS * AKU;
                            AKZ = AKU;
                            ALA = AKV;
                        } else {
                            let AKW = BL.exp();
                            let AKX = AKW * (BQ + (AKR - BL));
                            let AKY = AKS * AKW;
                            AKZ = AKX;
                            ALA = AKY;
                        }
                        let ALC = ALB / AKO;
                        let ALD = ALC * AKL;
                        let ALE = ALD * AKZ;
                        let ALF = ((AKN * ALC) * AKZ) + (ALA * ALD);
                        AKB = ALE;
                        AKC = ALF;
                    } else {
                        AKB = A;
                        AKC = AJB;
                    }
                    AJU = AKB;
                    AJV = AKC;
                } else {
                    let AJT = if AJM == FF { 1.0 } else { 0.0 };
                    let ALI;
                    let ALJ;
                    if AJT != 0.0 {
                        let ALG = if K < DM { 1.0 } else { 0.0 };
                        let ALU;
                        let ALV;
                        if ALG != 0.0 {
                            let ALL = (FF * parameters[45]) / (ALK * ALK);
                            let ALM = DM - K;
                            let ALN = L * BA;
                            let ALO = ALM / GH;
                            let ALP = Lanes([ALN[0], ALN[1], 0.0]);
                            let ALQ = ((FF * ALO) / ALL).sqrt();
                            let ALR = ((((ALP - (GO * ALO)) / GH) * FF) / ALL) * (FB / (FA * ALQ));
                            let ALT = if ALS == A { 1.0 } else { 0.0 };
                            let AMB;
                            let AMC;
                            if ALT != 0.0 {
                                AMB = ALK;
                                AMC = KN;
                            } else {
                                let ALW = BQ - (HE * GJ);
                                let ALX = (GQ * HE) * BA;
                                let ALY = ALK * ALW;
                                let ALZ = ALY * ALW;
                                let AMA = ((ALX * ALK) * ALW) + (ALX * ALY);
                                AMB = ALZ;
                                AMC = AMA;
                            }
                            let AMD = ALR * ALQ;
                            let AME = AMC * AMB;
                            let AMF = ((ALQ * ALQ) + (AMB * AMB)).sqrt();
                            let AMG = (ALQ * AMB) / AMF;
                            let AMH = (((ALR * AMB) + (AMC * ALQ)) - ((((AMD + AMD) + (AME + AME)) * (FB / (FA * AMF))) * AMG)) / AMF;
                            let AMI = ALM / AMG;
                            let AMJ = (ALP - (AMH * AMI)) / AMG;
                            let AMK = HE * AMG;
                            let AML = AMH * HE;
                            let AMM = AMK * ALL;
                            let AMN = AML * ALL;
                            let AMO = AMI + (AMM * GH);
                            let AMP = AMJ + ((AMN * GH) + (GO * AMM));
                            let ANJ;
                            let ANK;
                            if ALT != 0.0 {
                                let AMQ = Lanes([0.0, AMP[0], AMP[1], AMP[2]]);
                                ANJ = AMO;
                                ANK = AMQ;
                            } else {
                                let AMS = FF * AMR;
                                let AMT = IA * (BQ + (AMS * (BQ + (FF * GJ))));
                                let AMU = QZ / AMT;
                                let AMV = (((GQ * FF) * AMS) * IA) * AMU;
                                let AMW = ((BQ + AMR) / (BQ + AMS)) - AMU;
                                let AMX = AMN * AMW;
                                let AMY = AMI - (AMM * AMW);
                                let AMZ = Lanes([0.0, AMJ[0], AMJ[1], AMJ[2]]) - (Lanes([0.0, AMX[0], AMX[1], AMX[2]]) + ((((RA - Lanes([0.0, AMV[0], AMV[1], AMV[2]])) / AMT) * BA) * AMM));
                                let ANA = AMY - AMO;
                                let ANB = Lanes([0.0, AMP[0], AMP[1], AMP[2]]);
                                let ANC = (AMZ - ANB) * ANA;
                                let AND = LE * AMI;
                                let ANE = AND * AMI;
                                let ANF = (((((AMJ * LE) * AMI) + (AMJ * AND)) * GK) + (GR * ANE)) / IA;
                                let ANG = ((ANA * ANA) + ((ANE * GK) / IA)).sqrt();
                                let ANH = HE * ((AMY + AMO) + ANG);
                                let ANI = ((AMZ + ANB) + (((ANC + ANC) + Lanes([0.0, ANF[0], ANF[1], ANF[2]])) * (FB / (FA * ANG)))) * HE;
                                ANJ = ANH;
                                ANK = ANI;
                            }
                            let ANL = (ANJ - AMI) / ANJ;
                            let ANM = ((ANK - Lanes([0.0, AMJ[0], AMJ[1], AMJ[2]])) - (ANK * ANL)) / ANJ;
                            let ANN = if (ANL.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let AOK;
                            let AOL;
                            if ANN != 0.0 {
                                let ANO = AMK / ANL;
                                let ANP = (Lanes([0.0, AML[0], AML[1], AML[2]]) - (ANM * ANO)) / ANL;
                                let ANS = ANQ / ANR;
                                let ANT = ANS * ANJ;
                                let ANU = ANT * ANO;
                                let ANV = (-ANR) / ANJ;
                                let ANW = ((ANK * ANV) * BA) / ANJ;
                                let ANX = ANV.exp();
                                let ANY = AMB / ANO;
                                let ANZ = BQ + ANY;
                                let AOA = (ANV * ANZ).exp();
                                let AOB = ANX - AOA;
                                let AOC = ANU * AOB;
                                let AOD = ((((ANK * ANS) * ANO) + (ANP * ANT)) * AOB) + (((ANW * ANX) - (((ANW * ANZ) + (((Lanes([0.0, AMC[0], AMC[1], AMC[2]]) - (ANP * ANY)) / ANO) * ANV)) * AOA)) * ANU);
                                AOK = AOC;
                                AOL = AOD;
                            } else {
                                let AOE = ANQ * AMB;
                                let AOF = (-ANR) / ANJ;
                                let AOG = AOF.exp();
                                let AOH = AOE * AOG;
                                let AOI = (AMC * ANQ) * AOG;
                                let AOJ = Lanes([0.0, AOI[0], AOI[1], AOI[2]]) + (((((ANK * AOF) * BA) / ANJ) * AOG) * AOE);
                                AOK = AOH;
                                AOL = AOJ;
                            }
                            ALU = AOK;
                            ALV = AOL;
                        } else {
                            ALU = A;
                            ALV = AJB;
                        }
                        ALI = ALU;
                        ALJ = ALV;
                    } else {
                        let ALH = if AJM == MY { 1.0 } else { 0.0 };
                        let AON;
                        let AOO;
                        if ALH != 0.0 {
                            let AOM = if K < AJR { 1.0 } else { 0.0 };
                            let APC;
                            let APD;
                            if AOM != 0.0 {
                                let AOP = AJR - K;
                                let AOQ = L * BA;
                                let AOR = AOP.powf(AKQ);
                                let AOT = AOS + QZ;
                                let AOU = QZ / AOT;
                                let AOV = BQ - AOU;
                                let AOX = AOV.powf(AOW);
                                let AOY = AOR * AOX;
                                let AOZ = (AOQ * (AKQ * (AOP.powf((AKQ - FB))))) * AOX;
                                let APA = Lanes([0.0, AOZ[0], AOZ[1], 0.0]) + (((((RA - (RA * AOU)) / AOT) * BA) * (AOW * (AOV.powf((AOW - FB))))) * AOR);
                                let APB = if ALS == A { 1.0 } else { 0.0 };
                                let APK;
                                let APL;
                                if APB != 0.0 {
                                    APK = AOY;
                                    APL = APA;
                                } else {
                                    let APE = (QZ - parameters[51]) / AOS;
                                    let APF = RA / AOS;
                                    let APH = (APE - BQ) / APG;
                                    let API = APF / APG;
                                    let APJ = if APE < BQ { 1.0 } else { 0.0 };
                                    let APY;
                                    let APZ;
                                    if APJ != 0.0 {
                                        let APQ = APH.exp();
                                        let APR = BQ + APQ;
                                        let APS = ((API * APQ) * (FB / APR)) * APG;
                                        let APT = BQ + (APG * (APR.ln()));
                                        APY = APT;
                                        APZ = APS;
                                    } else {
                                        let APU = (-APH).exp();
                                        let APV = BQ + APU;
                                        let APW = APE + (APG * (APV.ln()));
                                        let APX = APF + ((((API * BA) * APU) * (FB / APV)) * APG);
                                        APY = APW;
                                        APZ = APX;
                                    }
                                    let AQB = APY.powf(AQA);
                                    let AQC = AOY * AQB;
                                    let AQD = (APA * AQB) + ((APZ * (AQA * (APY.powf((AQA - FB))))) * AOY);
                                    APK = AQC;
                                    APL = AQD;
                                }
                                let APM = -AKO;
                                let APN = APM * APK;
                                let APO = APL * APM;
                                let APP = if APN < BL { 1.0 } else { 0.0 };
                                let AQJ;
                                let AQK;
                                if APP != 0.0 {
                                    let AQE = APN.exp();
                                    let AQF = APO * AQE;
                                    AQJ = AQE;
                                    AQK = AQF;
                                } else {
                                    let AQG = BL.exp();
                                    let AQH = AQG * (BQ + (APN - BL));
                                    let AQI = APO * AQG;
                                    AQJ = AQH;
                                    AQK = AQI;
                                }
                                let AQL = ALB / AKO;
                                let AQM = AQL * AOP;
                                let AQN = AQM * AQJ;
                                let AQO = (AOQ * AQL) * AQJ;
                                let AQP = Lanes([0.0, AQO[0], AQO[1], 0.0]) + (AQK * AQM);
                                APC = AQN;
                                APD = AQP;
                            } else {
                                APC = A;
                                APD = AJB;
                            }
                            AON = APC;
                            AOO = APD;
                        } else {
                            AON = A;
                            AOO = AJB;
                        }
                        ALI = AON;
                        ALJ = AOO;
                    }
                    AJU = ALI;
                    AJV = ALJ;
                }
                let AJW = if AJU > A { 1.0 } else { 0.0 };
                let AQR;
                let AQS;
                if AJW != 0.0 {
                    let AQQ = if parameters[52] == BQ { 1.0 } else { 0.0 };
                    let ARE;
                    let ARF;
                    if AQQ != 0.0 {
                        let AQU = AQT + AJE;
                        let AQV = QZ * AQU;
                        let AQW = FS / AQV;
                        let AQY = AQX / AQU;
                        let AQZ = (AQW + ((QP / QU) * UO)) + AQY;
                        let ARA = ((((((RA * AQU) + (AJF * QZ)) * AQW) * BA) / AQV) + ((QQ / QU) * UO)) + (((AJF * AQY) * BA) / AQU);
                        let ARB = if AJM == MY { 1.0 } else { 0.0 };
                        let ARO;
                        let ARP;
                        if ARB != 0.0 {
                            let ARH = (AJU - AQZ) / ARG;
                            let ARI = (AJV - ARA) / ARG;
                            let ARJ = if AJU < AQZ { 1.0 } else { 0.0 };
                            let ARY;
                            let ARZ;
                            if ARJ != 0.0 {
                                let ARQ = ARH.exp();
                                let ARR = BQ + ARQ;
                                let ARS = AJU - (ARG * (ARR.ln()));
                                let ART = AJV - (((ARI * ARQ) * (FB / ARR)) * ARG);
                                ARY = ARS;
                                ARZ = ART;
                            } else {
                                let ARU = (-ARH).exp();
                                let ARV = BQ + ARU;
                                let ARW = AQZ - (ARG * (ARV.ln()));
                                let ARX = ARA - ((((ARI * BA) * ARU) * (FB / ARV)) * ARG);
                                ARY = ARW;
                                ARZ = ARX;
                            }
                            let ASA = QZ * ARY;
                            let ASB = (RA * ARY) + (ARZ * QZ);
                            ARO = ASA;
                            ARP = ASB;
                        } else {
                            let ARK = QZ * AJU;
                            let ARL = AJU + AQZ;
                            let ARM = (ARK * AQZ) / ARL;
                            let ARN = (((((RA * AJU) + (AJV * QZ)) * AQZ) + (ARA * ARK)) - ((AJV + ARA) * ARM)) / ARL;
                            ARO = ARM;
                            ARP = ARN;
                        }
                        ARE = ARO;
                        ARF = ARP;
                    } else {
                        let ARC = QZ * AJU;
                        let ARD = (RA * AJU) + (AJV * QZ);
                        ARE = ARC;
                        ARF = ARD;
                    }
                    AQR = ARE;
                    AQS = ARF;
                } else {
                    AQR = A;
                    AQS = AJB;
                }
                AJO = AQR;
                AJP = AQS;
            } else {
                AJO = A;
                AJP = AJB;
            }
            let AJQ = if GI > A { 1.0 } else { 0.0 };
            let ASD = ASC * MZ;
            let ASE = NA * ASC;
            let ASF = (W - GT) / GU;
            let ASG = X / GU;
            let ASH = if W < GT { 1.0 } else { 0.0 };
            let ASQ;
            let ASR;
            if ASH != 0.0 {
                let ASI = ASF.exp();
                let ASJ = BQ + ASI;
                let ASK = W - (GU * (ASJ.ln()));
                let ASL = X - (((ASG * ASI) * (FB / ASJ)) * GU);
                ASQ = ASK;
                ASR = ASL;
            } else {
                let ASM = (-ASF).exp();
                let ASN = BQ + ASM;
                let ASO = GT - (GU * (ASN.ln()));
                let ASP = ((((ASG * BA) * ASM) * (FB / ASN)) * GU) * BA;
                ASQ = ASO;
                ASR = ASP;
            }
            let ASS = BQ - (ASQ * MQ);
            let ASU = AST * ((MX * (BQ - (ASS.powf(MT)))) + (MY * (W - ASQ)));
            let ASV = ((((((ASR * MQ) * BA) * (MT * (ASS.powf(MV)))) * BA) * MX) + ((X - ASR) * MY)) * AST;
            let ASX = ASW * OP;
            let ASY = OQ * ASW;
            let ATA = ASZ * OX;
            let ATB = ATA * AIU;
            let ATC = (OY * ASZ) * AIU;
            let ATD = Lanes([ATC[0], ATC[1], 0.0, 0.0]) + (AIV * ATA);
            let ATE = ASZ * PG;
            let ATF = ATE * AIU;
            let ATG = (PH * ASZ) * AIU;
            let ATH = Lanes([0.0, ATG[0], ATG[1], ATG[2]]) + (AIV * ATE);
            let ATJ = (AY - NG) / ATI;
            let ATK = AZ / ATI;
            let ATL = if AY < NG { 1.0 } else { 0.0 };
            let ATU;
            let ATV;
            if ATL != 0.0 {
                let ATM = ATJ.exp();
                let ATN = BQ + ATM;
                let ATO = AY - (ATI * (ATN.ln()));
                let ATP = AZ - (((ATK * ATM) * (FB / ATN)) * ATI);
                ATU = ATO;
                ATV = ATP;
            } else {
                let ATQ = (-ATJ).exp();
                let ATR = BQ + ATQ;
                let ATS = NG - (ATI * (ATR.ln()));
                let ATT = ((((ATK * BA) * ATQ) * (FB / ATR)) * ATI) * BA;
                ATU = ATS;
                ATV = ATT;
            }
            let ATW = BQ - (ATU / LD);
            let ATZ = ((ATX * ((OM * ((OI * (BQ - (ATW.powf(OF)))) + (OJ * (AY - ATU)))) + (ON * AY))) * ATY) * AEU;
            let AUA = ((((((((((ATV / LD) * BA) * (OF * (ATW.powf(OH)))) * BA) * OI) + ((AZ - ATV) * OJ)) * OM) + (AZ * ON)) * ATX) * ATY) * AEU;
            let AUB = (BG - NG) / ATI;
            let AUC = BH / ATI;
            let AUD = if BG < NG { 1.0 } else { 0.0 };
            let AUM;
            let AUN;
            if AUD != 0.0 {
                let AUE = AUB.exp();
                let AUF = BQ + AUE;
                let AUG = BG - (ATI * (AUF.ln()));
                let AUH = BH - (((AUC * AUE) * (FB / AUF)) * ATI);
                AUM = AUG;
                AUN = AUH;
            } else {
                let AUI = (-AUB).exp();
                let AUJ = BQ + AUI;
                let AUK = NG - (ATI * (AUJ.ln()));
                let AUL = ((((AUC * BA) * AUI) * (FB / AUJ)) * ATI) * BA;
                AUM = AUK;
                AUN = AUL;
            }
            let AUO = BQ - (AUM / LD);
            let AUQ = ((ATX * ((OM * ((OI * (BQ - (AUO.powf(OF)))) + (OJ * (BG - AUM)))) + (ON * BG))) * ATY) * AUP;
            let AUR = ((((((((((AUN / LD) * BA) * (OF * (AUO.powf(OH)))) * BA) * OI) + ((BH - AUN) * OJ)) * OM) + (BH * ON)) * ATX) * ATY) * AUP;
            let AUT = S / AUS;
            let AUU = T / AUS;
            let AUV = if AUT < BL { 1.0 } else { 0.0 };
            let AVB;
            let AVC;
            if AUV != 0.0 {
                let AUW = AUT.exp();
                let AUX = AUU * AUW;
                AVB = AUW;
                AVC = AUX;
            } else {
                let AUY = BL.exp();
                let AUZ = AUY * (BQ + (AUT - BL));
                let AVA = AUU * AUY;
                AVB = AUZ;
                AVC = AVA;
            }
            let AVE = AVD * AVB;
            let AVF = AVC * AVD;
            let AVH = AVG * GJ;
            let AVI = (GL + FM) + FF;
            let AVJ = AVH * AVI;
            let AVK = ((GQ * AVG) * AVI) + ((GS + Lanes([FN[0], FN[1], 0.0])) * AVH);
            let AVX;
            let AVY;
            if AVL != 0.0 {
                let AVQ = (AVO * ((AVM * ACJ) + (AVN * ACN))) / AVP;
                let AVR = (((ACK * AVM) + (ACO * AVN)) * AVO) / AVP;
                AVX = AVQ;
                AVY = AVR;
            } else {
                let AVU = ((AY - AVS) / AVT) * BI;
                let AVV = (AZ / AVT) * BI;
                let AVW = if AVU < BL { 1.0 } else { 0.0 };
                let AWF;
                let AWG;
                if AVW != 0.0 {
                    let AWA = AVU.exp();
                    let AWB = AVV * AWA;
                    AWF = AWA;
                    AWG = AWB;
                } else {
                    let AWC = BL.exp();
                    let AWD = AWC * (BQ + (AVU - BL));
                    let AWE = AVV * AWC;
                    AWF = AWD;
                    AWG = AWE;
                }
                let AWI = (BQ + (EY * AWF)).sqrt();
                let AWJ = BQ + AWI;
                let AWK = (AWH * CQ) / AWJ;
                let AWL = ((CR * AWH) - (((AWG * EY) * (FB / (FA * AWI))) * AWK)) / AWJ;
                AVX = AWK;
                AVY = AWL;
            }
            let AWO;
            let AWP;
            let AWQ;
            let AWR;
            if AVZ != 0.0 {
                let AWM = AVX * AEU;
                let AWN = AVY * AEU;
                let AXI;
                let AXJ;
                if AVL != 0.0 {
                    let AWT = OR * DK;
                    let AWU = DL * OR;
                    let AWV = (BQ + AWT).sqrt();
                    let AWW = BQ + AWV;
                    let AWX = (AWT - OR) / AWW;
                    let AWY = EY * DU;
                    let AWZ = DV * EY;
                    let AXA = (BQ + AWY).sqrt();
                    let AXB = BQ + AXA;
                    let AXC = AWY / AXB;
                    let AXE = (AXD * ((AVM * AWX) + (AVN * AXC))) / AVP;
                    let AXF = (((((AWU - ((AWU * (FB / (FA * AWV))) * AWX)) / AWW) * AVM) + (((AWZ - ((AWZ * (FB / (FA * AXA))) * AXC)) / AXB) * AVN)) * AXD) / AVP;
                    AXI = AXE;
                    AXJ = AXF;
                } else {
                    let AXG = (BG - AVS) * BI;
                    let AXH = if AXG < BL { 1.0 } else { 0.0 };
                    let AXR;
                    let AXS;
                    if AXH != 0.0 {
                        let AXM = AXG.exp();
                        let AXN = DD * AXM;
                        AXR = AXM;
                        AXS = AXN;
                    } else {
                        let AXO = BL.exp();
                        let AXP = AXO * (BQ + (AXG - BL));
                        let AXQ = DD * AXO;
                        AXR = AXP;
                        AXS = AXQ;
                    }
                    let AXU = (BQ + (EY * AXR)).sqrt();
                    let AXV = BQ + AXU;
                    let AXW = (AXT * DK) / AXV;
                    let AXX = ((DL * AXT) - (((AXS * EY) * (FB / (FA * AXU))) * AXW)) / AXV;
                    AXI = AXW;
                    AXJ = AXX;
                }
                let AXK = AFG * AXI;
                let AXL = (AFJ * AXI) + (AXJ * AFG);
                AWO = AXK;
                AWP = AWM;
                AWQ = AXL;
                AWR = AWN;
            } else {
                AWO = A;
                AWP = AVX;
                AWQ = AFD;
                AWR = AVY;
            }
            let AYD;
            let AYE;
            let AYF;
            let AYG;
            let AYH;
            let AYI;
            let AYJ;
            let AYK;
            if AWS != 0.0 {
                let AXZ = MS * (AXY * (MR.powf(staged[142])));
                let AYA = (MR.powf(AXY)) - MY;
                let AYB = if GV < A { 1.0 } else { 0.0 };
                let AZE;
                let AZF;
                if AYB != 0.0 {
                    let AYV = GV.exp();
                    let AYW = BQ + AYV;
                    let AYX = BQ / AYW;
                    let AYY = (((GW * AYV) * AYX) * BA) / AYW;
                    AZE = AYX;
                    AZF = AYY;
                } else {
                    let AYZ = (-GV).exp();
                    let AZA = (GW * BA) * AYZ;
                    let AZB = BQ + AYZ;
                    let AZC = AYZ / AZB;
                    let AZD = (AZA - (AZA * AZC)) / AZB;
                    AZE = AZC;
                    AZF = AZD;
                }
                let AZG = ((AXZ * AZE) + (AZF * AYA)) * ASC;
                let AZH = (OS * BI) / BX;
                let AZI = HE / OU;
                let AZJ = AZH * AZI;
                let AZK = ASZ * AIU;
                let AZL = ((((OT * BI) / BX) * AZI) + ((((OV * AZI) * BA) / OU) * AZH)) * AZK;
                let AZM = AVF / AUS;
                let AZN = HK * Y;
                let AZO = ((ASC * ((AYA * AZE) + MY)) + (AZK * AZJ)) + (AVE / AUS);
                let AZP = AZN * AZO;
                let AZQ = (Z * HK) * AZO;
                let AZR = ((Lanes([AZG[0], AZG[1], 0.0, 0.0]) + (((AIV * ASZ) * AZJ) + Lanes([AZL[0], AZL[1], 0.0, 0.0]))) + Lanes([AZM[0], AZM[1], 0.0, 0.0])) * AZN;
                let AZS = Lanes([0.0, AZQ[0], AZQ[1], 0.0, 0.0]) + Lanes([AZR[0], 0.0, AZR[1], AZR[2], AZR[3]]);
                let AZU = AZT * AVE;
                let AZV = AVF * AZT;
                let AZX = AVF * AZW;
                let AZY = ATB + (AZW * AVE);
                let AZZ = ATD + Lanes([AZX[0], AZX[1], 0.0, 0.0]);
                let BAB = (BAA * AZY) + ATF;
                let BAC = (AZZ * BAA) + ATH;
                let BAE = BAD * AZY;
                let BAF = AZZ * BAD;
                AYD = BAE;
                AYE = AZU;
                AYF = BAB;
                AYG = AZP;
                AYH = BAF;
                AYI = AZV;
                AYJ = BAC;
                AYK = AZS;
            } else {
                AYD = ATB;
                AYE = AVE;
                AYF = ATF;
                AYG = A;
                AYH = ATD;
                AYI = AVF;
                AYJ = ATH;
                AYK = AYC;
            }
            let AYM = (J * FX) * AYL;
            let AYN = (FY * J) * AYL;
            let AYO = (J * QZ) * AYL;
            let AYP = (RA * J) * AYL;
            let AYQ = (J * ((WD + XT) + YV)) * AYL;
            let AYR = (((WE + XU) + YW) * J) * AYL;
            let AYS = T * B;
            let AYT = (J * (((((TN + XF) + (B * S)) - ZG) + SW) + SB)) * AYL;
            let AYU = ((((((TO + Lanes([XG[0], XG[1], 0.0, 0.0])) + Lanes([AYS[0], AYS[1], 0.0, 0.0])) - Lanes([ZH[0], ZH[1], 0.0, 0.0])) + Lanes([SX[0], SX[1], 0.0, 0.0])) + Lanes([SC[0], SC[1], 0.0, 0.0])) * J) * AYL;
            let BAK;
            let BAL;
            let BAM;
            let BAN;
            if C != 0.0 {
                let BAG = (J * (-AGT)) * AYL;
                let BAH = ((AGV * BA) * J) * AYL;
                BAK = BAG;
                BAL = A;
                BAM = BAH;
                BAN = AGQ;
            } else {
                let BAI = (J * (-AGT)) * AYL;
                let BAJ = ((AGV * BA) * J) * AYL;
                BAK = A;
                BAL = BAI;
                BAM = AGQ;
                BAN = BAJ;
            }
            let BAO = (J * AJI) * AYL;
            let BAP = (AJK * J) * AYL;
            let BAR = (J * (BAQ * AJO)) * AYL;
            let BAS = ((AJP * BAQ) * J) * AYL;
            let BAT = ((J * AE) / AQX) * AYL;
            let BAU = ((AF * J) / AQX) * AYL;
            let BAV = ((J * AI) / AQT) * AYL;
            let BAW = ((AJ * J) / AQT) * AYL;
            let BAX = J * ((ASD + AYD) + AYE);
            let BAY = ((Lanes([ASE[0], ASE[1], 0.0, 0.0]) + AYH) + Lanes([AYI[0], AYI[1], 0.0, 0.0])) * J;
            let BBA = ddt(12033, BAX) * AYL;
            let BBB = (BAY * BAZ) * AYL;
            let BBC = BAX * AYL;
            let BBD = BAY * AYL;
            let BBE = J * ASU;
            let BBF = ASV * J;
            let BBG = ddt(12039, BBE) * AYL;
            let BBH = (BBF * BAZ) * AYL;
            let BBI = BBE * AYL;
            let BBJ = BBF * AYL;
            let BBK = J * ((ASX + AYF) + AVJ);
            let BBL = ((Lanes([0.0, ASY[0], ASY[1], ASY[2]]) + AYJ) + Lanes([0.0, AVK[0], AVK[1], AVK[2]])) * J;
            let BBM = ddt(12049, BBK) * AYL;
            let BBN = (BBL * BAZ) * AYL;
            let BBO = BBK * AYL;
            let BBP = BBL * AYL;
            let BBQ = J * AYG;
            let BBR = AYK * J;
            let BBS = ddt(12055, BBQ) * AYL;
            let BBT = (BBR * BAZ) * AYL;
            let BBU = BBQ * AYL;
            let BBV = BBR * AYL;
            let BBX = BBW * AK;
            let BBY = AL * BBW;
            let BBZ = ddt(12063, BBX) * AYL;
            let BCA = (BBY * BAZ) * AYL;
            let BCB = BBX * AYL;
            let BCC = BBY * AYL;
            let BCE = BCD * AN;
            let BCF = AO * BCD;
            let BCG = ddt(12071, BCE) * AYL;
            let BCH = (BCF * BAZ) * AYL;
            let BCI = BCE * AYL;
            let BCJ = BCF * AYL;
            let BCK = (J * AHC) * AYL;
            let BCL = (AHE * J) * AYL;
            let BCN = ((J * BE) * BCM) * AYL;
            let BCO = ((BF * J) * BCM) * AYL;
            let BCP = J * (AUQ + AWO);
            let BCQ = (AUR + AWQ) * J;
            let BCR = ddt(12091, BCP) * AYL;
            let BCS = (BCQ * BAZ) * AYL;
            let BCT = BCP * AYL;
            let BCU = BCQ * AYL;
            let BCV = (J * ((AGZ + (B * AY)) + AGW)) * AYL;
            let BCW = (((AHB + (AZ * B)) + AGY) * J) * AYL;
            let BCX = J * (ATZ + AWP);
            let BCY = (AUA + AWR) * J;
            let BCZ = ddt(12110, BCX) * AYL;
            let BDA = (BCY * BAZ) * AYL;
            let BDB = BCX * AYL;
            let BDC = BCY * AYL;
            let BDH;
            let BDI;
            if D != 0.0 {
                let BDE = ((J * AU) * BDD) * AYL;
                let BDF = ((AV * J) * BDD) * AYL;
                BDH = BDE;
                BDI = BDF;
            } else {
                BDH = A;
                BDI = BDG;
            }
            let BDN;
            let BDO;
            if E != 0.0 {
                let BDK = ((J * AR) * BDJ) * AYL;
                let BDL = ((AS * J) * BDJ) * AYL;
                BDN = BDK;
                BDO = BDL;
            } else {
                BDN = A;
                BDO = BDM;
            }
            let BDP = (QV + QS) / QP;
            let BDQ = ((QX + QY) - (QQ * BDP)) / QP;
            let BDV;
            let BDW;
            if BDR != 0.0 {
                let BDS = AJO / BDP;
                let BDT = BDS.abs();
                let BDU = ((AJP - (BDQ * BDS)) / BDP) * ((FA * (if BDS >= 0e0f64 { 1.0 } else { 0.0 })) - FB);
                BDV = BDT;
                BDW = BDU;
            } else {
                BDV = A;
                BDW = AJB;
            }
            let BDX = if BDP > A { 1.0 } else { 0.0 };
            let BEE;
            let BEF;
            if BDX != 0.0 {
                let BDY = (AYD + AYF) / BDP;
                let BDZ = ((AYH + AYJ) - (BDQ * BDY)) / BDP;
                BEE = BDY;
                BEF = BDZ;
            } else {
                let BEB = BEA * AIU;
                let BEC = BEB * QP;
                let BED = ((AIV * BEA) * QP) + (QQ * BEB);
                BEE = BEC;
                BEF = BED;
            }
            let BEK;
            let BEL;
            if BEG != 0.0 {
                let BEH = BAA * BEE;
                let BEI = BEF * BAA;
                BEK = BEH;
                BEL = BEI;
            } else {
                let BEQ;
                let BER;
                if BEJ != 0.0 {
                    let BEO = BEN * BEE;
                    let BEP = BEF * BEN;
                    BEQ = BEO;
                    BER = BEP;
                } else {
                    BEQ = A;
                    BER = AJB;
                }
                BEK = BEQ;
                BEL = BER;
            }
            let BEM = if (TN + WD) < A { 1.0 } else { 0.0 };
            let BES = if ((XF + XT) + YV) < A { 1.0 } else { 0.0 };
            let BET = if AGZ < A { 1.0 } else { 0.0 };
            let BEU = if AGW < A { 1.0 } else { 0.0 };
            let BEV = if AHC < A { 1.0 } else { 0.0 };
            let BEX = ddt(12363, BEW);
            let BEZ = BEK * BEX;
            let BFA = BEL * BEX;
            let BFB = Lanes([BFA[0], BFA[1], BFA[2], BFA[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, ((BEY * BAZ) * BEK)]);
            let BFC = BEK * BEW;
            let BFD = BEL * BEW;
            let BFE = Lanes([BFD[0], BFD[1], BFD[2], BFD[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, (BEY * BEK)]);
            let BFF = BDV * BEW;
            let BFG = BDW * BEW;
            let BFH = Lanes([BFG[0], BFG[1], BFG[2], BFG[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, (BEY * BDV)]);
            let BFI = if ((((BAV + BBZ) + BCG) + BCK) + BCR) == A { 1.0 } else { 0.0 };
            let BFJ = AYN[0];
            let BFK = AYN[1];
            let BFL = AYN[2];
            let BFM = AYP[0];
            let BFN = AYP[1];
            let BFO = AYP[2];
            let BFP = AYP[3];
            let BFQ = AYR[0];
            let BFR = AYR[1];
            let BFS = AYU[0];
            let BFT = AYU[1];
            let BFU = AYU[2];
            let BFV = AYU[3];
            let BFW = BAM[0];
            let BFX = BAM[1];
            let BFY = BAM[2];
            let BFZ = BAN[0];
            let BGA = BAN[1];
            let BGB = BAN[2];
            let BGC = BAP[0];
            let BGD = BAP[1];
            let BGE = BAP[2];
            let BGF = BAP[3];
            let BGG = BAP[4];
            let BGH = BAS[0];
            let BGI = BAS[1];
            let BGJ = BAS[2];
            let BGK = BAS[3];
            let BGL = BAU[0];
            let BGM = BAU[1];
            let BGN = BAW[0];
            let BGO = BAW[1];
            let BGP = BBB[0];
            let BGQ = BBB[1];
            let BGR = BBB[2];
            let BGS = BBB[3];
            let BGT = BBH[0];
            let BGU = BBH[1];
            let BGV = BBN[0];
            let BGW = BBN[1];
            let BGX = BBN[2];
            let BGY = BBN[3];
            let BGZ = BBT[0];
            let BHA = BBT[1];
            let BHB = BBT[2];
            let BHC = BBT[3];
            let BHD = BBT[4];
            let BHE = BCA[0];
            let BHF = BCA[1];
            let BHG = BCH[0];
            let BHH = BCH[1];
            let BHI = BCL[0];
            let BHJ = BCL[1];
            let BHK = BCL[2];
            let BHL = BCL[3];
            let BHM = BCL[4];
            let BHN = BCL[5];
            let BHO = BCL[6];
            let BHP = BCL[7];
            let BHQ = BCO[0];
            let BHR = BCO[1];
            let BHS = BCO[2];
            let BHT = BCO[3];
            let BHU = BCO[4];
            let BHV = BCO[5];
            let BHW = BCO[6];
            let BHX = BCO[7];
            let BHY = BCS[0];
            let BHZ = BCS[1];
            let BIA = BCS[2];
            let BIB = BCS[3];
            let BIC = BCS[4];
            let BID = BCS[5];
            let BIE = BCS[6];
            let BIF = BCS[7];
            let BIG = BCW[0];
            let BIH = BCW[1];
            let BII = BCW[2];
            let BIJ = BCW[3];
            let BIK = BCW[4];
            let BIL = BDA[0];
            let BIM = BDA[1];
            let BIN = BDA[2];
            let BIO = BDA[3];
            let BIP = BDA[4];
            let BIQ = BDI[0];
            let BIR = BDI[1];
            let BIS = BDO[0];
            let BIT = BDO[1];
            let BIU = BEY;
            let BIV = BFB[0];
            let BIW = BFB[1];
            let BIX = BFB[2];
            let BIY = BFB[3];
            let BIZ = BFB[4];
            let BJA = BFH[0];
            let BJB = BFH[1];
            let BJC = BFH[2];
            let BJD = BFH[3];
            let BJE = BFH[4];
            let BJF = BBD[0];
            let BJG = BBD[1];
            let BJH = BBD[2];
            let BJI = BBD[3];
            let BJJ = BBJ[0];
            let BJK = BBJ[1];
            let BJL = BBP[0];
            let BJM = BBP[1];
            let BJN = BBP[2];
            let BJO = BBP[3];
            let BJP = BBV[0];
            let BJQ = BBV[1];
            let BJR = BBV[2];
            let BJS = BBV[3];
            let BJT = BBV[4];
            let BJU = BCC[0];
            let BJV = BCC[1];
            let BJW = BCJ[0];
            let BJX = BCJ[1];
            let BJY = BCU[0];
            let BJZ = BCU[1];
            let BKA = BCU[2];
            let BKB = BCU[3];
            let BKC = BCU[4];
            let BKD = BCU[5];
            let BKE = BCU[6];
            let BKF = BCU[7];
            let BKG = BDC[0];
            let BKH = BDC[1];
            let BKI = BDC[2];
            let BKJ = BDC[3];
            let BKK = BDC[4];
            let BKL = BFE[0];
            let BKM = BFE[1];
            let BKN = BFE[2];
            let BKO = BFE[3];
            let BKP = BFE[4];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(7),
            multiplicity * (AYM),
            [5, 6, 7],
            [BFJ, BFK, BFL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (AYO),
            [3, 5, 6, 7],
            [BFM, BFN, BFO, BFP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (AYQ),
            [3, 4],
            [BFQ, BFR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(3),
            multiplicity * (AYT),
            [3, 5, 6, 7],
            [BFS, BFT, BFU, BFV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(6),
            multiplicity * (BAK),
            [4, 5, 6],
            [BFW, BFX, BFY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(7),
            multiplicity * (BAL),
            [4, 5, 6],
            [BFZ, BGA, BGB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * (BAO),
            [3, 4, 5, 6, 7],
            [BGC, BGD, BGE, BGF, BGG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (BAR),
            [3, 5, 6, 7],
            [BGH, BGI, BGJ, BGK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(3),
            multiplicity * (BAT),
            [2, 3],
            [BGL, BGM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(4),
            multiplicity * (BAV),
            [1, 4],
            [BGN, BGO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(3),
            multiplicity * (BBA),
            [3, 5, 6, 7],
            [BGP, BGQ, BGR, BGS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(3),
            multiplicity * (BBG),
            [3, 4],
            [BGT, BGU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (BBM),
            [3, 5, 6, 7],
            [BGV, BGW, BGX, BGY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * (BBS),
            [3, 4, 5, 6, 7],
            [BGZ, BHA, BHB, BHC, BHD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (BBZ),
            [1, 2],
            [BHE, BHF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (BCG),
            [0, 1],
            [BHG, BHH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(1),
            Some(8),
            multiplicity * (BCK),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [BHI, BHJ, BHK, BHL, BHM, BHN, BHO, BHP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * (BCN),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [BHQ, BHR, BHS, BHT, BHU, BHV, BHW, BHX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(1),
            Some(8),
            multiplicity * (BCR),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [BHY, BHZ, BIA, BIB, BIC, BID, BIE, BIF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * (BCV),
            [4, 5, 6, 7, 9],
            [BIG, BIH, BII, BIJ, BIK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * (BCZ),
            [4, 5, 6, 7, 9],
            [BIL, BIM, BIN, BIO, BIP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(8),
            Some(9),
            multiplicity * (BDH),
            [8, 9],
            [BIQ, BIR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(8), Some(9), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[185],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(6),
            multiplicity * (BDN),
            [6, 9],
            [BIS, BIT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(6), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[186],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            None,
            multiplicity * (BKQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (BEW),
            [10],
            [BIU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * (BEZ),
            [3, 5, 6, 7, 10],
            [BIV, BIW, BIX, BIY, BIZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (BFF),
            [3, 5, 6, 7, 10],
            [BJA, BJB, BJC, BJD, BJE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            Some(3),
            multiplicity * (BEW),
            [10],
            [BIU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (BKR),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(3),
            multiplicity * (BKS),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(3),
            multiplicity * (BKT),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (BKU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (BKV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(3),
            multiplicity * (BKW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(3),
            multiplicity * (BKX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(3),
            multiplicity * (BKY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (BKZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (BLA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (BLB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(9),
            multiplicity * (BLC),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(8),
            multiplicity * (BLD),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(8),
            multiplicity * (BLE),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (staged[187]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (staged[188]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(8),
            multiplicity * (staged[189]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(9),
            multiplicity * (staged[190]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(6),
            multiplicity * (staged[191]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(8),
            multiplicity * (staged[192]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[193]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (staged[194]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(6),
            multiplicity * (staged[195]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(6),
            multiplicity * (staged[196]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = AYM;
        self.canonical_reactive[1] = AYO;
        self.canonical_reactive[2] = AYQ;
        self.canonical_reactive[3] = AYT;
        self.canonical_reactive[4] = BAK;
        self.canonical_reactive[5] = BAL;
        self.canonical_reactive[6] = BAO;
        self.canonical_reactive[7] = BAR;
        self.canonical_reactive[8] = BAT;
        self.canonical_reactive[9] = BAV;
        self.canonical_reactive[10] = BBC;
        self.canonical_reactive[11] = BJF;
        self.canonical_reactive[12] = BJG;
        self.canonical_reactive[13] = BJH;
        self.canonical_reactive[14] = BJI;
        self.canonical_reactive[15] = BBI;
        self.canonical_reactive[16] = BJJ;
        self.canonical_reactive[17] = BJK;
        self.canonical_reactive[18] = BBO;
        self.canonical_reactive[19] = BJL;
        self.canonical_reactive[20] = BJM;
        self.canonical_reactive[21] = BJN;
        self.canonical_reactive[22] = BJO;
        self.canonical_reactive[23] = BBU;
        self.canonical_reactive[24] = BJP;
        self.canonical_reactive[25] = BJQ;
        self.canonical_reactive[26] = BJR;
        self.canonical_reactive[27] = BJS;
        self.canonical_reactive[28] = BJT;
        self.canonical_reactive[29] = BCB;
        self.canonical_reactive[30] = BJU;
        self.canonical_reactive[31] = BJV;
        self.canonical_reactive[32] = BCI;
        self.canonical_reactive[33] = BJW;
        self.canonical_reactive[34] = BJX;
        self.canonical_reactive[35] = BCK;
        self.canonical_reactive[36] = BCN;
        self.canonical_reactive[37] = BCT;
        self.canonical_reactive[38] = BJY;
        self.canonical_reactive[39] = BJZ;
        self.canonical_reactive[40] = BKA;
        self.canonical_reactive[41] = BKB;
        self.canonical_reactive[42] = BKC;
        self.canonical_reactive[43] = BKD;
        self.canonical_reactive[44] = BKE;
        self.canonical_reactive[45] = BKF;
        self.canonical_reactive[46] = BCV;
        self.canonical_reactive[47] = BDB;
        self.canonical_reactive[48] = BKG;
        self.canonical_reactive[49] = BKH;
        self.canonical_reactive[50] = BKI;
        self.canonical_reactive[51] = BKJ;
        self.canonical_reactive[52] = BKK;
        self.canonical_reactive[53] = BDH;
        self.canonical_reactive[54] = staged[185];
        self.canonical_reactive[55] = BDN;
        self.canonical_reactive[56] = staged[186];
        self.canonical_reactive[57] = BKQ;
        self.canonical_reactive[58] = BEW;
        self.canonical_reactive[59] = BFC;
        self.canonical_reactive[60] = BKL;
        self.canonical_reactive[61] = BKM;
        self.canonical_reactive[62] = BKN;
        self.canonical_reactive[63] = BKO;
        self.canonical_reactive[64] = BKP;
        self.canonical_reactive[65] = BFF;
        self.canonical_reactive[66] = BEW;
        self.canonical_reactive[67] = BKR;
        self.canonical_reactive[68] = BKS;
        self.canonical_reactive[69] = BKT;
        self.canonical_reactive[70] = BKU;
        self.canonical_reactive[71] = BKV;
        self.canonical_reactive[72] = BKW;
        self.canonical_reactive[73] = BKX;
        self.canonical_reactive[74] = BKY;
        self.canonical_reactive[75] = BKZ;
        self.canonical_reactive[76] = BLA;
        self.canonical_reactive[77] = BLB;
        self.canonical_reactive[78] = BLC;
        self.canonical_reactive[79] = BLD;
        self.canonical_reactive[80] = BLE;
        self.canonical_reactive[81] = staged[187];
        self.canonical_reactive[82] = staged[188];
        self.canonical_reactive[83] = staged[189];
        self.canonical_reactive[84] = staged[190];
        self.canonical_reactive[85] = staged[191];
        self.canonical_reactive[86] = staged[192];
        self.canonical_reactive[87] = staged[193];
        self.canonical_reactive[88] = staged[194];
        self.canonical_reactive[89] = staged[195];
        self.canonical_reactive[90] = staged[196];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 5, 6, 7],
            &[cached[11], cached[12], cached[13], cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(3),
            &[3, 4],
            &[cached[16], cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(7),
            &[3, 5, 6, 7],
            &[cached[19], cached[20], cached[21], cached[22]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(5),
            &[3, 4, 5, 6, 7],
            &[cached[24], cached[25], cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[30], cached[31]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(8),
            &[0, 1, 4, 5, 6, 7, 8, 9],
            &[cached[38], cached[39], cached[40], cached[41], cached[42], cached[43], cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            Some(9),
            &[4, 5, 6, 7, 9],
            &[cached[48], cached[49], cached[50], cached[51], cached[52]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 5, 6, 7, 10],
            &[cached[60], cached[61], cached[62], cached[63], cached[64]],
            &[],
            &[],
            multiplicity,
        );
    }

}
