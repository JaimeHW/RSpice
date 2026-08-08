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
        let mut key = Vec::with_capacity(308);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[175] = values[0];
        self.canonical_staged[109] = values[1];
        self.canonical_staged[1] = values[2];
        self.canonical_staged[176] = values[3];
        self.canonical_staged[0] = values[4];
        self.canonical_staged[177] = values[5];
        self.canonical_staged[82] = values[6];
        self.canonical_staged[86] = values[7];
        self.canonical_staged[178] = values[8];
        self.canonical_staged[25] = values[9];
        self.canonical_staged[26] = values[10];
        self.canonical_staged[89] = values[11];
        self.canonical_staged[93] = values[12];
        self.canonical_staged[179] = values[13];
        self.canonical_staged[27] = values[14];
        self.canonical_staged[28] = values[15];
        self.canonical_staged[2] = values[16];
        self.canonical_staged[3] = values[17];
        self.canonical_staged[4] = values[18];
        self.canonical_staged[5] = values[19];
        self.canonical_staged[7] = values[20];
        self.canonical_staged[191] = values[21];
        self.canonical_staged[193] = values[22];
        self.canonical_staged[8] = values[23];
        self.canonical_staged[9] = values[24];
        self.canonical_staged[10] = values[25];
        self.canonical_staged[11] = values[26];
        self.canonical_staged[12] = values[27];
        self.canonical_staged[13] = values[28];
        self.canonical_staged[14] = values[29];
        self.canonical_staged[15] = values[30];
        self.canonical_staged[16] = values[31];
        self.canonical_staged[17] = values[32];
        self.canonical_staged[196] = values[33];
        self.canonical_staged[18] = values[34];
        self.canonical_staged[19] = values[35];
        self.canonical_staged[20] = values[36];
        self.canonical_staged[21] = values[37];
        self.canonical_staged[22] = values[38];
        self.canonical_staged[23] = values[39];
        self.canonical_staged[24] = values[40];
        self.canonical_staged[29] = values[41];
        self.canonical_staged[30] = values[42];
        self.canonical_staged[31] = values[43];
        self.canonical_staged[32] = values[44];
        self.canonical_staged[33] = values[45];
        self.canonical_staged[34] = values[46];
        self.canonical_staged[35] = values[47];
        self.canonical_staged[36] = values[48];
        self.canonical_staged[37] = values[49];
        self.canonical_staged[38] = values[50];
        self.canonical_staged[39] = values[51];
        self.canonical_staged[40] = values[52];
        self.canonical_staged[198] = values[53];
        self.canonical_staged[200] = values[54];
        self.canonical_staged[202] = values[55];
        self.canonical_staged[48] = values[56];
        self.canonical_staged[52] = values[57];
        self.canonical_staged[204] = values[58];
        self.canonical_staged[205] = values[59];
        self.canonical_staged[54] = values[60];
        self.canonical_staged[56] = values[61];
        self.canonical_staged[206] = values[62];
        self.canonical_staged[207] = values[63];
        self.canonical_staged[74] = values[64];
        self.canonical_staged[81] = values[65];
        self.canonical_staged[87] = values[66];
        self.canonical_staged[208] = values[67];
        self.canonical_staged[96] = values[68];
        self.canonical_staged[99] = values[69];
        self.canonical_staged[101] = values[70];
        self.canonical_staged[104] = values[71];
        self.canonical_staged[107] = values[72];
        self.canonical_staged[209] = values[73];
        self.canonical_staged[110] = values[74];
        self.canonical_staged[112] = values[75];
        self.canonical_staged[115] = values[76];
        self.canonical_staged[211] = values[77];
        self.canonical_staged[210] = values[78];
        self.canonical_staged[123] = values[79];
        self.canonical_staged[121] = values[80];
        self.canonical_staged[122] = values[81];
        self.canonical_staged[127] = values[82];
        self.canonical_staged[131] = values[83];
        self.canonical_staged[138] = values[84];
        self.canonical_staged[139] = values[85];
        self.canonical_staged[142] = values[86];
        self.canonical_staged[146] = values[87];
        self.canonical_staged[212] = values[88];
        self.canonical_staged[213] = values[89];
        self.canonical_staged[156] = values[90];
        self.canonical_staged[158] = values[91];
        self.canonical_staged[214] = values[92];
        self.canonical_staged[160] = values[93];
        self.canonical_staged[161] = values[94];
        self.canonical_staged[162] = values[95];
        self.canonical_staged[163] = values[96];
        self.canonical_staged[164] = values[97];
        self.canonical_staged[215] = values[98];
        self.canonical_staged[216] = values[99];
        self.canonical_staged[217] = values[100];
        self.canonical_staged[218] = values[101];
        self.canonical_staged[224] = values[102];
        self.canonical_staged[227] = values[103];
        self.canonical_staged[229] = values[104];
        self.canonical_staged[231] = values[105];
        self.canonical_staged[220] = values[106];
        self.canonical_staged[221] = values[107];
        self.canonical_staged[222] = values[108];
        self.canonical_staged[223] = values[109];
        self.canonical_staged[225] = values[110];
        self.canonical_staged[226] = values[111];
        self.canonical_staged[228] = values[112];
        self.canonical_staged[230] = values[113];
        self.canonical_staged[169] = values[114];
        self.canonical_staged[170] = values[115];
        self.canonical_staged[171] = values[116];
        self.canonical_staged[173] = values[117];
        self.canonical_staged[174] = values[118];
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
                let J = parameters[33];
                let M = parameters[150];
                let N = 0e0f64;
                let P = 1e-12f64;
                let S = 2e0f64;
                let T = parameters[67];
                let W = parameters[114];
                let Y = 5e-2f64;
                let Z = 1e-1f64;
                let AH = parameters[72];
                let AK = parameters[117];
                let AW = parameters[98];
                let AX = parameters[96];
                let BB = 4e0f64;
                let BC = parameters[121];
                let BG = parameters[103];
                let BI = 6e0f64;
                let BX = parameters[141];
                let CA = 5e-1f64;
                let CB = parameters[142];
                let CO = 3e0f64;
                let CR = 1e0f64;
                let CT = parameters[74];
                let DB = parameters[93];
                let DH = parameters[143];
                let DN = parameters[5];
                let DV = parameters[82];
                let DX = parameters[81];
                let EC = parameters[139];
                let ES = 0e0f64;
                let EU = 0e0f64;
                let EX = parameters[131];
                let FB = 0e0f64;
                let FC = 0e0f64;
                let FN = 0e0f64;
                let FO = 0e0f64;
                let FP = 0e0f64;
                let FQ = 0e0f64;
                let FR = 0e0f64;
                let FX = 0e0f64;
                let FY = 0e0f64;
                let FZ = 0e0f64;
                let mut oBQ = 0.0;
                let mut oBR = 0.0;
                let mut oBS = 0.0;
                let mut oCV = 0.0;
                let mut oDC = 0.0;
                let mut oDD = 0.0;
                let mut oDI = 0.0;
                let mut oDJ = 0.0;
                let mut oDK = 0.0;
                let mut oDL = 0.0;
                let mut oDQ = 0.0;
                let mut oDS = 0.0;
                let mut oDT = 0.0;
                let mut oDU = 0.0;
                let mut oDW = 0.0;
                let mut oDY = 0.0;
                let mut oDZ = 0.0;
                let mut oEK = 0.0;
                let mut oEL = 0.0;
                let mut oEM = 0.0;
                let mut oEN = 0.0;
                let mut oEO = 0.0;
                let mut oEP = 0.0;
                let mut oEZ = 0.0;
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
                let R = if parameters[134] > N { 1.0 } else { 0.0 };
                let U = S.powf((S - T));
                let V = B / U;
                let X = W + (((parameters[115] * L) * L) / (L + parameters[116]));
                let AA = (X - Y) / Z;
                let AB = if X < Y { 1.0 } else { 0.0 };
                let AE = if AB != 0.0 {
                    let AC = Y + (Z * ((B + (AA.exp())).ln()));
                    AC
                } else {
                    let AD = X + (Z * ((B + ((-AA).exp())).ln()));
                    AD
                };
                let AF = B / W;
                let AG = B / parameters[66];
                let AI = S.powf((S - AH));
                let AJ = B / AI;
                let AL = AK + (((parameters[118] * L) * L) / (L + parameters[119]));
                let AM = (AL - Y) / Z;
                let AN = if AL < Y { 1.0 } else { 0.0 };
                let AQ = if AN != 0.0 {
                    let AO = Y + (Z * ((B + (AM.exp())).ln()));
                    AO
                } else {
                    let AP = AL + (Z * ((B + ((-AM).exp())).ln()));
                    AP
                };
                let AR = B / AK;
                let AS = B / parameters[71];
                let AT = B - (B / parameters[83]);
                let AU = B / (8.617086918058125e-5f64 * L);
                let AV = B - parameters[75];
                let AY = AW - AX;
                let AZ = if parameters[122] != N { 1.0 } else { 0.0 };
                let BA = if parameters[123] != N { 1.0 } else { 0.0 };
                let BD = ((BB - AW) - AX) + BC;
                let BE = -parameters[105];
                let BF = B - AW;
                let BH = B - BG;
                let BJ = BI - (S * parameters[21]);
                let BK = -parameters[113];
                let BL = BI - (S * parameters[32]);
                let BM = -parameters[110];
                let BN = (BB - parameters[97]) + BC;
                let BO = -parameters[111];
                let BP = if parameters[24] == B { 1.0 } else { 0.0 };
                if BP != 0.0 {
                    let BQ = -parameters[107];
                    oBQ = BQ;
                    let BR = -parameters[106];
                    oBR = BR;
                    let BS = -parameters[108];
                    oBS = BS;
                } else {
                }
                let BT = (BB - BG) + BC;
                let BU = -parameters[112];
                let BV = BI - (S * parameters[23]);
                let BW = BB / parameters[146];
                let BY = BB - BX;
                let BZ = -parameters[140];
                let CC = 3.5e0f64 - (CA * CB);
                let CD = B - BX;
                let CE = B - CB;
                let CF = AW - S;
                let CG = -parameters[120];
                let CH = (AX + AW) - B;
                let CI = parameters[99] - B;
                let CJ = parameters[87] + parameters[88];
                let CK = parameters[100] - B;
                let CL = if parameters[57] > N { 1.0 } else { 0.0 };
                let CM = if parameters[58] > N { 1.0 } else { 0.0 };
                let CN = if parameters[59] > N { 1.0 } else { 0.0 };
                let CP = B - (CO.powf((-1e0f64 / T)));
                let CQ = B - T;
                let CS = CQ - CR;
                let CU = if CT == B { 1.0 } else { 0.0 };
                if CU != 0.0 {
                } else {
                    let CV = if CT == S { 1.0 } else { 0.0 };
                    oCV = CV;
                }
                let CW = -1e0f64 / AH;
                let CX = parameters[76] - CR;
                let CY = B - AH;
                let CZ = CY - CR;
                let DA = if parameters[92] == N { 1.0 } else { 0.0 };
                if BP != 0.0 {
                } else {
                    let DC = if DB == N { 1.0 } else { 0.0 };
                    oDC = DC;
                    if DC != 0.0 {
                    } else {
                        let DD = B - DB;
                        oDD = DD;
                    }
                }
                let DE = if (if parameters[34] > N { 1.0 } else { 0.0 }) != 0.0 && (if parameters[35] > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DF = if (if parameters[36] > N { 1.0 } else { 0.0 }) != 0.0 && (if parameters[37] > N { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DG = if parameters[8] == B { 1.0 } else { 0.0 };
                if DG != 0.0 {
                    let DI = DH * S;
                    oDI = DI;
                    let DJ = (B - DH) * S;
                    oDJ = DJ;
                } else {
                    let DK = DH * S;
                    oDK = DK;
                    let DL = (B - DH) * S;
                    oDL = DL;
                }
                let DM = parameters[144] * BB;
                let DO = if J > N { 1.0 } else { 0.0 };
                let DP = if (if DN > N { 1.0 } else { 0.0 }) != 0.0 && DO != 0.0 { 1.0 } else { 0.0 };
                if DP != 0.0 {
                    let DQ = J * S;
                    oDQ = DQ;
                    if DG != 0.0 {
                        let DS = ((B - DH) * J) * S;
                        oDS = DS;
                    } else {
                        let DT = ((B - DH) * J) * S;
                        oDT = DT;
                    }
                    let DU = if DN == B { 1.0 } else { 0.0 };
                    oDU = DU;
                } else {
                }
                let DR = if parameters[84] == B { 1.0 } else { 0.0 };
                if DR != 0.0 {
                    let DW = B / (B - (AT.powf(DV)));
                    oDW = DW;
                    let DY = AT * DX;
                    oDY = DY;
                    let DZ = (((DW * DW) * (AT.powf((DV - B)))) * DV) / DX;
                    oDZ = DZ;
                } else {
                }
                let EA = B - parameters[68];
                let EB = B - parameters[77];
                let ED = B - (S.powf((-1e0f64 / EC)));
                let EE = B - EC;
                let EF = EE - CR;
                let EG = B / parameters[85];
                let EH = if parameters[79] == N { 1.0 } else { 0.0 };
                let EI = if (if (if DN == B { 1.0 } else { 0.0 }) != 0.0 || (if DN == CO { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && DO != 0.0 { 1.0 } else { 0.0 };
                if EI != 0.0 {
                    if EH != 0.0 {
                        let EK = CA * J;
                        oEK = EK;
                    } else {
                        let EL = S * J;
                        oEL = EL;
                    }
                } else {
                }
                let EJ = if parameters[6] == B { 1.0 } else { 0.0 };
                if EJ != 0.0 {
                    let EM = -T;
                    oEM = EM;
                    let EN = EM - CR;
                    oEN = EN;
                    let EO = B - parameters[95];
                    oEO = EO;
                    let EP = B - parameters[94];
                    oEP = EP;
                } else {
                }
                let EQ = A * parameters[69];
                let ER = A * parameters[78];
                let ET = if CM != 0.0 {
                    N
                } else {
                    ES
                };
                let EV = if CN != 0.0 {
                    N
                } else {
                    EU
                };
                let EW = if parameters[130] > N { 1.0 } else { 0.0 };
                let EY = if EX == B { 1.0 } else { 0.0 };
                if EY != 0.0 {
                } else {
                    let EZ = if EX == S { 1.0 } else { 0.0 };
                    oEZ = EZ;
                }
                let FA = if J == N { 1.0 } else { 0.0 };
                let FD;
                let FE;
                if BP != 0.0 {
                    FD = FB;
                    FE = N;
                } else {
                    FD = N;
                    FE = FC;
                }
                let FF;
                let FG;
                let FH;
                let FI;
                let FJ;
                let FK;
                let FL;
                let FM;
                if CM != 0.0 {
                    let FS;
                    let FT;
                    let FU;
                    let FV;
                    let FW;
                    if CN != 0.0 {
                        FS = FN;
                        FT = N;
                        FU = FO;
                        FV = FP;
                        FW = N;
                    } else {
                        FS = N;
                        FT = FQ;
                        FU = N;
                        FV = N;
                        FW = FR;
                    }
                    FF = FS;
                    FG = FT;
                    FH = N;
                    FI = N;
                    FJ = FU;
                    FK = FV;
                    FL = FW;
                    FM = N;
                } else {
                    let GA;
                    let GB;
                    let GC;
                    if CN != 0.0 {
                        GA = FX;
                        GB = N;
                        GC = FY;
                    } else {
                        GA = N;
                        GB = FZ;
                        GC = N;
                    }
                    FF = N;
                    FG = N;
                    FH = GA;
                    FI = GB;
                    FJ = N;
                    FK = N;
                    FL = N;
                    FM = GC;
                }
            [C, K, L, O, Q, R, U, V, AB, AF, AG, AI, AJ, AN, AR, AS, AU, AE, AQ, AV, AY, AZ, BA, BD, BE, BF, BH, BJ, BK, BL, BM, BN, BO, BP, oBQ, oBR, oBS, BT, BU, BV, BW, BY, BZ, CC, CD, CE, CF, CG, CH, CI, CJ, CK, H, CL, CM, CN, CP, CQ, CU, oCV, CW, CY, DA, oDC, oDD, DE, DF, DG, oDI, oDJ, oDK, oDL, DM, DP, oDQ, oDS, oDT, oDU, DR, oDW, oDY, oDZ, I, EA, EB, ED, EE, EG, EH, EI, oEK, oEL, EJ, oEM, oEO, oEP, EQ, ER, EW, EY, oEZ, FA, FF, FG, FH, FI, ET, EV, FD, FE, FJ, FK, FL, FM, CS, CX, CZ, EF, oEN]
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
        self.canonical_staged[41] = produced[1];
        self.canonical_staged[219] = produced[2];
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
        let produced: [f64; 110] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let B = staged[1];
                let E = 1e0f64;
                let K = 5e-2f64;
                let L = 1e-1f64;
                let X = parameters[66];
                let AS = parameters[71];
                let BM = parameters[138];
                let BW = parameters[72];
                let CA = parameters[75];
                let CG = staged[6];
                let CS = staged[191];
                let CT = parameters[10];
                let CV = 1e-3f64;
                let CZ = staged[193];
                let DE = parameters[11];
                let DL = 0e0f64;
                let DR = 1e-6f64;
                let DT = 5e-1f64;
                let ED = parameters[17];
                let EG = parameters[19];
                let EI = staged[196];
                let ET = staged[25];
                let EW = parameters[35];
                let EY = staged[26];
                let FA = staged[27];
                let FC = parameters[37];
                let FE = staged[28];
                let FW = staged[40];
                let GB = staged[198];
                let GD = staged[41];
                let GG = staged[200];
                let GL = staged[202];
                let GU = 2e0f64;
                let GZ = 4e0f64;
                let HD = staged[206];
                let HK = staged[208];
                let HT = staged[209];
                let HZ = staged[211];
                let IQ = staged[212];
                let IS = staged[213];
                let mut oCX = 0.0;
                let mut oDH = 0.0;
                let mut oGE = 0.0;
                let mut oGJ = 0.0;
                let mut oGO = 0.0;
                let mut oHE = 0.0;
                let mut oHG = 0.0;
                let mut oHH = 0.0;
                let mut oHL = 0.0;
                let mut oHM = 0.0;
                let mut oHN = 0.0;
                let mut oHO = 0.0;
                let mut oHP = 0.0;
                let mut oHQ = 0.0;
                let mut oHU = 0.0;
                let mut oHV = 0.0;
                let mut oHW = 0.0;
                let mut oHX = 0.0;
                let mut oHY = 0.0;
                let mut oIA = 0.0;
                let mut oIB = 0.0;
                let mut oIR = 0.0;
                let mut oIT = 0.0;
                let mut oIU = 0.0;
                let mut oIV = 0.0;
                let A = temperature + parameters[0];
                let C = A / B;
                let D = 8.617086918058125e-5f64 * A;
                let F = E / D;
                let G = F - staged[2];
                let H = A - B;
                let I = C.ln();
                let J = staged[3] - (((parameters[115] * A) * A) / (A + parameters[116]));
                let M = (J - K) / L;
                let N = if J < K { 1.0 } else { 0.0 };
                let Q = if N != 0.0 {
                    let O = K + (L * ((E + (M.exp())).ln()));
                    O
                } else {
                    let P = J + (L * ((E + ((-M).exp())).ln()));
                    P
                };
                let R = staged[4] - (((parameters[118] * A) * A) / (A + parameters[119]));
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
                let Z = (((-3e0f64 * D) * I) + (X * C)) + (Y * parameters[105]);
                let AA = (K - Z) / D;
                let AB = if K < Z { 1.0 } else { 0.0 };
                let AE = if AB != 0.0 {
                    let AC = Z + (D * ((E + (AA.exp())).ln()));
                    AC
                } else {
                    let AD = K + (D * ((E + ((-AA).exp())).ln()));
                    AD
                };
                let AF = Y * parameters[110];
                let AG = (((-3e0f64 * D) * I) + (parameters[64] * C)) + AF;
                let AH = (K - AG) / D;
                let AI = if K < AG { 1.0 } else { 0.0 };
                let AL = if AI != 0.0 {
                    let AJ = AG + (D * ((E + (AH.exp())).ln()));
                    AJ
                } else {
                    let AK = K + (D * ((E + ((-AH).exp())).ln()));
                    AK
                };
                let AM = (((-3e0f64 * D) * I) + (parameters[80] * C)) + AF;
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
                let BG = (((-3e0f64 * D) * I) + (parameters[27] * C)) + (Y * parameters[109]);
                let BH = (K - BG) / D;
                let BI = if K < BG { 1.0 } else { 0.0 };
                let BL = if BI != 0.0 {
                    let BJ = BG + (D * ((E + (BH.exp())).ln()));
                    BJ
                } else {
                    let BK = K + (D * ((E + ((-BH).exp())).ln()));
                    BK
                };
                let BN = (((-3e0f64 * D) * I) + (BM * C)) + (Y * parameters[140]);
                let BO = (K - BN) / D;
                let BP = if K < BN { 1.0 } else { 0.0 };
                let BS = if BP != 0.0 {
                    let BQ = BN + (D * ((E + (BO.exp())).ln()));
                    BQ
                } else {
                    let BR = K + (D * ((E + ((-BO).exp())).ln()));
                    BR
                };
                let BT = E / AE;
                let BU = E / BF;
                let BV = (X * BT).powf(parameters[67]);
                let BX = (AS * BU).powf(BW);
                let BY = parameters[65] * BV;
                let BZ = parameters[137] * ((BM / BS).powf(parameters[139]));
                let CB = (staged[5] * ((AS / AZ).powf(BW))) + CA;
                let CC = E / CB;
                let CD = parameters[70] * CB;
                let CE = CA * CC;
                let CF = parameters[54] * ((I * parameters[97]).exp());
                let CH = if CF < CG { 1.0 } else { 0.0 };
                let CI = if CH != 0.0 {
                    CG
                } else {
                    CF
                };
                let CJ = parameters[56] * ((I * staged[7]).exp());
                let CK = parameters[55] * ((I * parameters[101]).exp());
                let CL = if CK < CG { 1.0 } else { 0.0 };
                let CM = if CL != 0.0 {
                    CG
                } else {
                    CK
                };
                let CN = parameters[57] * ((I * parameters[102]).exp());
                let CO = (I * parameters[104]).exp();
                let CP = parameters[58] * CO;
                let CQ = parameters[59] * CO;
                let CR = parameters[60] * ((I * parameters[99]).exp());
                let CY;
                if CS != 0.0 {
                    let CU = CT * (E + (H * parameters[122]));
                    let CW = (CU - E) / CV;
                    let CX = if CU < E { 1.0 } else { 0.0 };
                    oCX = CX;
                    let DC = if CX != 0.0 {
                        let DA = E + (CV * ((E + (CW.exp())).ln()));
                        DA
                    } else {
                        let DB = CU + (CV * ((E + ((-CW).exp())).ln()));
                        DB
                    };
                    let DD = DC - 6.931471805599453e-4f64;
                    CY = DD;
                } else {
                    CY = CT;
                }
                let DI;
                if CZ != 0.0 {
                    let DF = DE * (E + (H * parameters[123]));
                    let DG = (DF - E) / CV;
                    let DH = if DF < E { 1.0 } else { 0.0 };
                    oDH = DH;
                    let DP = if DH != 0.0 {
                        let DN = E + (CV * ((E + (DG.exp())).ln()));
                        DN
                    } else {
                        let DO = DF + (CV * ((E + ((-DG).exp())).ln()));
                        DO
                    };
                    let DQ = DP - 6.931471805599453e-4f64;
                    DI = DQ;
                } else {
                    DI = DE;
                }
                let DJ = parameters[43] * (E + (parameters[124] * H));
                let DK = DJ * DJ;
                let DM = if DJ < DL { 1.0 } else { 0.0 };
                let DV = if DM != 0.0 {
                    let DS = 5e-7f64 / (((DK + DR).sqrt()) - DJ);
                    DS
                } else {
                    let DU = DT * (((DK + DR).sqrt()) + DJ);
                    DU
                };
                let DW = (parameters[9] * (((I * staged[8]) / CY).exp())) * (((staged[9] * G) / CY).exp());
                let DX = parameters[12] * ((I * staged[10]).exp());
                let DY = parameters[30] * ((I * staged[11]).exp());
                let DZ = staged[13] * G;
                let EA = (parameters[20] * ((I * staged[12]).exp())) * ((DZ / parameters[21]).exp());
                let EB = (parameters[31] * ((I * staged[14]).exp())) * (((staged[15] * G) / parameters[32]).exp());
                let EC = I * staged[16];
                let EE = staged[17] * G;
                let EF = (parameters[16] * ((EC / ED).exp())) * ((EE / ED).exp());
                let EH = (parameters[18] * ((EC / EG).exp())) * ((EE / EG).exp());
                let EM;
                let EN;
                let EO;
                if EI != 0.0 {
                    let EJ = parameters[25] * (((staged[18] * G) / ED).exp());
                    let EK = parameters[28] * ((staged[19] * G).exp());
                    let EL = parameters[26] * (((staged[20] * G) / EG).exp());
                    EM = EJ;
                    EN = EK;
                    EO = EL;
                } else {
                    EM = DL;
                    EN = DL;
                    EO = DL;
                }
                let EP = (parameters[29] * ((I * staged[21]).exp())) * ((staged[22] * G).exp());
                let EQ = (parameters[22] * ((I * staged[23]).exp())) * ((DZ / parameters[23]).exp());
                let ER = (parameters[145] * ((I * staged[24]).exp())) * ((DZ / parameters[146]).exp());
                let ES = (parameters[151] * (C.sqrt())) * ((parameters[153] * H).exp());
                let EU = (Q * ET).powf(-5e-1f64);
                let EV = E / BV;
                let EX = (((((((EW * Q) * Q) * EU) * EV) * X) * BT) * ET) * ET;
                let EZ = ((((((parameters[34] * EU) * AE) * AE) * EY) * EY) * BV) * ((EW - EX).exp());
                let FB = (W * FA).powf(-5e-1f64);
                let FD = (((((((FC * W) * W) * FB) * (E / BX)) * AS) * BU) * FA) * FA;
                let FF = ((((((parameters[36] * FB) * BF) * BF) * FE) * FE) * BX) * ((FC - FD).exp());
                let FG = (I * parameters[96]).exp();
                let FH = (parameters[14] * FG) * CC;
                let FI = (parameters[13] * FG) * EV;
                let FJ = (staged[30] * G).exp();
                let FK = (parameters[133] * ((I * staged[29]).exp())) * FJ;
                let FL = (parameters[134] * ((I * staged[31]).exp())) * FJ;
                let FM = parameters[135] * ((I * staged[32]).exp());
                let FN = parameters[136] * ((I * staged[33]).exp());
                let FO = (parameters[86] * ((I * staged[34]).exp())) * ((staged[35] * G).exp());
                let FP = parameters[87] * ((I * staged[36]).exp());
                let FQ = parameters[88] * ((I * staged[37]).exp());
                let FR = FP + FQ;
                let FS = (parameters[89] * FR) / staged[38];
                let FT = parameters[90] * ((I * staged[39]).exp());
                let FU = A - 3e2f64;
                let FV = if A < 5.25e2f64 { 1.0 } else { 0.0 };
                let FZ = if FV != 0.0 {
                    let FX = FW * ((E + (7.2e-4f64 * FU)) - ((1.6e-6f64 * FU) * FU));
                    FX
                } else {
                    let FY = FW * 1.081e0f64;
                    FY
                };
                let GA = parameters[92] * FG;
                let GF;
                if GB != 0.0 {
                    let GC = E / CN;
                    let GE = if GC > GD { 1.0 } else { 0.0 };
                    oGE = GE;
                    let GH = if GE != 0.0 {
                        GD
                    } else {
                        GC
                    };
                    GF = GH;
                } else {
                    GF = DL;
                }
                let GK;
                if GG != 0.0 {
                    let GI = E / CP;
                    let GJ = if GI > GD { 1.0 } else { 0.0 };
                    oGJ = GJ;
                    let GM = if GJ != 0.0 {
                        GD
                    } else {
                        GI
                    };
                    GK = GM;
                } else {
                    GK = DL;
                }
                let GP;
                if GL != 0.0 {
                    let GN = E / CQ;
                    let GO = if GN > GD { 1.0 } else { 0.0 };
                    oGO = GO;
                    let GQ = if GO != 0.0 {
                        GD
                    } else {
                        GN
                    };
                    GP = GQ;
                } else {
                    GP = DL;
                }
                let GR = AE * staged[48];
                let GS = L * AE;
                let GT = AE / staged[52];
                let GV = E - CE;
                let GW = (GU - CE) / GV;
                let GX = AZ * (E - (GW.powf(staged[54])));
                let GY = AZ / staged[56];
                let HA = (GZ * DW) / DX;
                let HB = E / DI;
                let HC = HB - 1e0f64;
                if HD != 0.0 {
                } else {
                    let HE = ((GA * F).exp()) - E;
                    oHE = HE;
                }
                let HF = parameters[15] * DW;
                if EI != 0.0 {
                    let HG = EM * GU;
                    oHG = HG;
                } else {
                }
                if EI != 0.0 {
                    let HH = EO * GU;
                    oHH = HH;
                } else {
                }
                let HI = GU * EP;
                let HJ = (GZ * EP) / DY;
                if HK != 0.0 {
                    let HL = staged[96] * FK;
                    oHL = HL;
                    let HM = GZ * (FK / FM);
                    oHM = HM;
                    let HN = staged[99] * FK;
                    oHN = HN;
                } else {
                    let HO = staged[101] * FK;
                    oHO = HO;
                    let HP = GZ * (FK / FM);
                    oHP = HP;
                    let HQ = staged[104] * FK;
                    oHQ = HQ;
                }
                let HR = GU * FL;
                let HS = staged[107] * (FL / FN);
                if HT != 0.0 {
                    let HU = staged[110] * EP;
                    oHU = HU;
                    if HK != 0.0 {
                        let HV = staged[112] * FK;
                        oHV = HV;
                        let HW = (GZ * FK) / FM;
                        oHW = HW;
                    } else {
                        let HX = staged[115] * FK;
                        oHX = HX;
                        let HY = (GZ * FK) / FM;
                        oHY = HY;
                    }
                    if HZ != 0.0 {
                        let IA = (parameters[33] * (EP + FK)) * CN;
                        oIA = IA;
                        let IB = D * (GU - ((IA * F).ln()));
                        oIB = IB;
                    } else {
                    }
                } else {
                }
                let IC = GU * D;
                let ID = staged[131] * BY;
                let IE = parameters[68] * BY;
                let IF = parameters[77] * CD;
                let IG = FP * DX;
                let IH = DT * IG;
                let II = L * AZ;
                let IJ = L * BS;
                let IK = BS * staged[139];
                let IL = BS / staged[142];
                let IM = (FO * DX) * ((DW / DX).powf(staged[146]));
                let IN = parameters[85] * D;
                let IO = ((GZ * FQ) * D) / CR;
                let IP = DT * IO;
                if IQ != 0.0 {
                    let IR = FS * DT;
                    oIR = IR;
                } else {
                    let IT = HI * FT;
                    oIT = IT;
                }
                if IS != 0.0 {
                    if IQ != 0.0 {
                        let IU = staged[156] * FS;
                        oIU = IU;
                    } else {
                        let IV = (staged[158] * EP) * FT;
                        oIV = IV;
                    }
                } else {
                }
            [D, F, N, T, AB, AI, AO, AW, BC, BI, BP, BT, BU, BS, BZ, AZ, CD, CE, CH, CJ, CL, CN, CR, oCX, oDH, DM, CY, DW, EA, EB, EF, EH, EQ, ER, ES, Q, EX, EZ, W, FD, FF, FH, FI, FP, FR, FV, GA, oGE, oGJ, oGO, AL, GR, GS, GT, GV, GW, GX, GY, HA, HB, oHE, HF, BL, oHG, EN, oHH, HI, HJ, oHL, oHM, oHN, oHO, oHP, oHQ, HR, HS, oHU, oHV, oHW, oHX, oHY, oIA, oIB, IC, DV, FZ, CM, CI, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO, IP, oIR, AR, oIT, oIU, oIV, GF, GK, GP, HC]
        };
        self.canonical_staged[45] = produced[0];
        self.canonical_staged[42] = produced[1];
        self.canonical_staged[180] = produced[2];
        self.canonical_staged[181] = produced[3];
        self.canonical_staged[182] = produced[4];
        self.canonical_staged[183] = produced[5];
        self.canonical_staged[184] = produced[6];
        self.canonical_staged[185] = produced[7];
        self.canonical_staged[186] = produced[8];
        self.canonical_staged[187] = produced[9];
        self.canonical_staged[188] = produced[10];
        self.canonical_staged[51] = produced[11];
        self.canonical_staged[88] = produced[12];
        self.canonical_staged[143] = produced[13];
        self.canonical_staged[145] = produced[14];
        self.canonical_staged[47] = produced[15];
        self.canonical_staged[137] = produced[16];
        self.canonical_staged[60] = produced[17];
        self.canonical_staged[189] = produced[18];
        self.canonical_staged[124] = produced[19];
        self.canonical_staged[190] = produced[20];
        self.canonical_staged[119] = produced[21];
        self.canonical_staged[46] = produced[22];
        self.canonical_staged[192] = produced[23];
        self.canonical_staged[194] = produced[24];
        self.canonical_staged[195] = produced[25];
        self.canonical_staged[43] = produced[26];
        self.canonical_staged[68] = produced[27];
        self.canonical_staged[77] = produced[28];
        self.canonical_staged[79] = produced[29];
        self.canonical_staged[71] = produced[30];
        self.canonical_staged[75] = produced[31];
        self.canonical_staged[78] = produced[32];
        self.canonical_staged[80] = produced[33];
        self.canonical_staged[69] = produced[34];
        self.canonical_staged[84] = produced[35];
        self.canonical_staged[83] = produced[36];
        self.canonical_staged[85] = produced[37];
        self.canonical_staged[91] = produced[38];
        self.canonical_staged[90] = produced[39];
        self.canonical_staged[92] = produced[40];
        self.canonical_staged[64] = produced[41];
        self.canonical_staged[63] = produced[42];
        self.canonical_staged[168] = produced[43];
        self.canonical_staged[153] = produced[44];
        self.canonical_staged[197] = produced[45];
        self.canonical_staged[65] = produced[46];
        self.canonical_staged[199] = produced[47];
        self.canonical_staged[201] = produced[48];
        self.canonical_staged[203] = produced[49];
        self.canonical_staged[44] = produced[50];
        self.canonical_staged[49] = produced[51];
        self.canonical_staged[50] = produced[52];
        self.canonical_staged[53] = produced[53];
        self.canonical_staged[59] = produced[54];
        self.canonical_staged[58] = produced[55];
        self.canonical_staged[55] = produced[56];
        self.canonical_staged[57] = produced[57];
        self.canonical_staged[61] = produced[58];
        self.canonical_staged[62] = produced[59];
        self.canonical_staged[66] = produced[60];
        self.canonical_staged[67] = produced[61];
        self.canonical_staged[70] = produced[62];
        self.canonical_staged[72] = produced[63];
        self.canonical_staged[73] = produced[64];
        self.canonical_staged[76] = produced[65];
        self.canonical_staged[94] = produced[66];
        self.canonical_staged[95] = produced[67];
        self.canonical_staged[97] = produced[68];
        self.canonical_staged[98] = produced[69];
        self.canonical_staged[100] = produced[70];
        self.canonical_staged[102] = produced[71];
        self.canonical_staged[103] = produced[72];
        self.canonical_staged[105] = produced[73];
        self.canonical_staged[106] = produced[74];
        self.canonical_staged[108] = produced[75];
        self.canonical_staged[111] = produced[76];
        self.canonical_staged[113] = produced[77];
        self.canonical_staged[114] = produced[78];
        self.canonical_staged[116] = produced[79];
        self.canonical_staged[117] = produced[80];
        self.canonical_staged[120] = produced[81];
        self.canonical_staged[118] = produced[82];
        self.canonical_staged[125] = produced[83];
        self.canonical_staged[126] = produced[84];
        self.canonical_staged[128] = produced[85];
        self.canonical_staged[129] = produced[86];
        self.canonical_staged[130] = produced[87];
        self.canonical_staged[132] = produced[88];
        self.canonical_staged[133] = produced[89];
        self.canonical_staged[134] = produced[90];
        self.canonical_staged[150] = produced[91];
        self.canonical_staged[135] = produced[92];
        self.canonical_staged[136] = produced[93];
        self.canonical_staged[141] = produced[94];
        self.canonical_staged[140] = produced[95];
        self.canonical_staged[144] = produced[96];
        self.canonical_staged[148] = produced[97];
        self.canonical_staged[147] = produced[98];
        self.canonical_staged[151] = produced[99];
        self.canonical_staged[149] = produced[100];
        self.canonical_staged[152] = produced[101];
        self.canonical_staged[154] = produced[102];
        self.canonical_staged[155] = produced[103];
        self.canonical_staged[157] = produced[104];
        self.canonical_staged[159] = produced[105];
        self.canonical_staged[165] = produced[106];
        self.canonical_staged[166] = produced[107];
        self.canonical_staged[167] = produced[108];
        self.canonical_staged[172] = produced[109];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 13137 => 0usize, 13143 => 1usize, 13153 => 2usize, 13159 => 3usize, 13165 => 4usize, 13173 => 5usize, 13181 => 6usize, 13201 => 7usize, 13220 => 8usize, 13491 => 9usize, _ => usize::MAX };
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
            let C = staged[177];
            let E = staged[196];
            let F = staged[200];
            let G = staged[202];
            let H = node_potentials[6];
            let I = node_potentials[7];
            let J = 1e0f64;
            let K = 1e0f64;
            let L = parameters[3];
            let O = node_potentials[8];
            let P = 1e0f64;
            let S = node_potentials[4];
            let T = 1e0f64;
            let W = node_potentials[5];
            let X = 1e0f64;
            let AC = 1e0f64;
            let AH = node_potentials[2];
            let AI = 1e0f64;
            let AL = node_potentials[1];
            let AM = 1e0f64;
            let AR = 1e0f64;
            let AU = node_potentials[10];
            let AV = 1e0f64;
            let AY = 1e0f64;
            let BF = -1e0f64;
            let BR = staged[42];
            let BU = parameters[147];
            let BZ = 1e0f64;
            let CG = staged[43];
            let EZ = staged[44];
            let GL = 4e0f64;
            let GN = 2e0f64;
            let GO = 1e0f64;
            let GS = 2e0f64;
            let GW = parameters[149];
            let GY = Lanes([0e0f64; 2]);
            let HF = staged[45];
            let HJ = staged[46];
            let HN = 1e2f64;
            let IG = staged[49];
            let IH = staged[50];
            let IR = 5e-1f64;
            let IX = 2e-1f64;
            let JN = parameters[62];
            let JO = parameters[61];
            let JW = parameters[63];
            let LY = parameters[148];
            let MA = Lanes([0e0f64; 3]);
            let MQ = staged[47];
            let MR = 1e-1f64;
            let OD = staged[51];
            let OG = staged[52];
            let OI = staged[169];
            let OK = staged[53];
            let OL = 3e0f64;
            let OO = staged[204];
            let OQ = staged[205];
            let OT = staged[55];
            let PO = parameters[76];
            let PS = staged[56];
            let PU = staged[171];
            let PV = staged[57];
            let PW = staged[58];
            let PZ = staged[59];
            let QA = staged[60];
            let QE = staged[61];
            let QM = staged[62];
            let QV = staged[206];
            let QW = staged[63];
            let QY = staged[64];
            let RC = staged[65];
            let RH = staged[66];
            let RQ = 1.0000000000000002e-2f64;
            let SE = staged[67];
            let SH = staged[68];
            let SO = 1e-4f64;
            let TC = parameters[152];
            let TN = staged[69];
            let TQ = parameters[154];
            let TR = 1e-3f64;
            let UF = parameters[155];
            let UL = parameters[17];
            let UW = staged[70];
            let UZ = staged[207];
            let VE = parameters[19];
            let VR = 4e1f64;
            let VV = 2.3538526683702e17f64;
            let WB = staged[71];
            let WD = staged[72];
            let WK = staged[73];
            let WU = staged[74];
            let WW = parameters[93];
            let XN = staged[75];
            let XS = parameters[21];
            let YE = staged[76];
            let YR = staged[77];
            let YU = parameters[23];
            let ZF = staged[78];
            let ZI = parameters[32];
            let ZT = staged[79];
            let ZW = parameters[146];
            let AAH = staged[80];
            let AAM = staged[82];
            let AAO = staged[83];
            let AAS = Lanes([0e0f64; 2]);
            let ABG = 1e-30f64;
            let ABI = parameters[67];
            let ABM = 6e0f64;
            let ABR = 1.6666666666666666e-1f64;
            let ABS = staged[84];
            let ACA = 3.333333333333333e-1f64;
            let ACC = 2.5e-1f64;
            let ACM = staged[86];
            let ADB = staged[88];
            let ADI = staged[89];
            let ADK = staged[90];
            let AED = staged[94];
            let AEE = staged[95];
            let AEJ = staged[208];
            let AET = parameters[72];
            let AFB = staged[91];
            let AFT = staged[93];
            let AGJ = staged[97];
            let AGK = parameters[144];
            let AGM = staged[98];
            let AGS = staged[100];
            let AGY = staged[102];
            let AGZ = staged[103];
            let AHE = staged[105];
            let AHP = staged[106];
            let AHQ = staged[108];
            let AHW = staged[209];
            let AHX = staged[109];
            let AID = staged[111];
            let AII = Lanes([0e0f64; 9]);
            let AIT = staged[210];
            let AIV = staged[113];
            let AIX = staged[114];
            let AJC = staged[116];
            let AJD = staged[117];
            let AJL = staged[211];
            let AJY = 1.21e-2f64;
            let AKI = staged[119];
            let AKP = -1e0f64;
            let AKQ = -1e0f64;
            let AKV = Lanes([0e0f64; 3]);
            let ALS = 1e-12f64;
            let ALU = -1e0f64;
            let ALZ = -1e0f64;
            let AME = staged[121];
            let AMG = parameters[81];
            let AMI = parameters[82];
            let AMM = staged[122];
            let AMR = 1.0000000000000002e-2f64;
            let ANE = staged[6];
            let ANG = Lanes([0e0f64; 4]);
            let ANL = staged[125];
            let ANR = parameters[39];
            let ANW = parameters[44];
            let AOC = parameters[42];
            let AOT = staged[126];
            let AOV = parameters[41];
            let APG = parameters[40];
            let APP = parameters[45];
            let APX = parameters[7];
            let AQW = parameters[47];
            let ARV = staged[127];
            let ARW = staged[128];
            let ASX = parameters[48];
            let ATB = parameters[49];
            let ATL = parameters[51];
            let AUF = parameters[50];
            let AUY = staged[129];
            let AVC = staged[130];
            let AVL = 1e-6f64;
            let AWH = staged[132];
            let AWY = staged[133];
            let AXB = staged[134];
            let AXE = staged[135];
            let AXN = staged[136];
            let AYC = staged[137];
            let AYD = staged[138];
            let AYU = parameters[33];
            let AYX = staged[140];
            let AYY = staged[141];
            let AZM = staged[143];
            let AZO = staged[142];
            let AZP = staged[144];
            let AZQ = staged[145];
            let AZT = staged[147];
            let BAE = staged[148];
            let BAH = staged[149];
            let BAM = staged[212];
            let BAN = staged[150];
            let BAO = staged[151];
            let BAP = staged[152];
            let BAQ = staged[153];
            let BAT = staged[154];
            let BAU = parameters[91];
            let BBA = staged[213];
            let BBI = staged[155];
            let BBT = staged[214];
            let BCE = staged[157];
            let BCV = staged[159];
            let BDA = staged[160];
            let BDE = Lanes([0e0f64; 5]);
            let BDN = parameters[1];
            let BEV = staged[161];
            let BEY = parameters[95];
            let BFC = parameters[94];
            let BFF = staged[162];
            let BGA = -1e0f64;
            let BGJ = ddt_scale();
            let BHM = staged[163];
            let BHT = staged[164];
            let BIC = staged[165];
            let BIT = staged[166];
            let BIW = Lanes([0e0f64; 2]);
            let BIZ = staged[167];
            let BJC = Lanes([0e0f64; 2]);
            let BJH = staged[215];
            let BJQ = staged[168];
            let BJW = staged[216];
            let BJZ = staged[217];
            let BKD = parameters[132];
            let BKM = node_potentials[11];
            let BKO = 1e0f64;
            let BRI = 0e0f64;
            let BRJ = 0e0f64;
            let BRK = 0e0f64;
            let BRL = 0e0f64;
            let BRM = 0e0f64;
            let BRN = 0e0f64;
            let BRO = 0e0f64;
            let BRP = 0e0f64;
            let BRQ = 0e0f64;
            let BRR = 0e0f64;
            let BRS = 0e0f64;
            let BRT = 0e0f64;
            let BRU = 0e0f64;
            let BRV = 0e0f64;
            let BRW = 0e0f64;
            let BRX = 0e0f64;
            let BRY = 0e0f64;
            let BRZ = 0e0f64;
            let B = ctx.simparam_or("gmin", A);
            let D = if C != 0.0 {
                B
            } else {
                A
            };
            let M = L * (H - I);
            let N = (Lanes([J, 0.0]) - Lanes([0.0, K])) * L;
            let Q = L * (H - O);
            let R = (Lanes([J, 0.0]) - Lanes([0.0, P])) * L;
            let U = L * (H - S);
            let V = (Lanes([0.0, J]) - Lanes([T, 0.0])) * L;
            let Y = L * (W - S);
            let Z = (Lanes([0.0, X]) - Lanes([T, 0.0])) * L;
            let AA = L * (W - H);
            let AB = (Lanes([X, 0.0]) - Lanes([0.0, J])) * L;
            let AD = L * (node_potentials[3] - I);
            let AE = (Lanes([AC, 0.0]) - Lanes([0.0, K])) * L;
            let AF = L * (I - O);
            let AG = (Lanes([K, 0.0]) - Lanes([0.0, P])) * L;
            let AJ = L * (AH - S);
            let AK = (Lanes([AI, 0.0]) - Lanes([0.0, T])) * L;
            let AN = L * (AL - W);
            let AO = (Lanes([AM, 0.0]) - Lanes([0.0, X])) * L;
            let AP = L * (AL - AH);
            let AQ = (Lanes([AM, 0.0]) - Lanes([0.0, AI])) * L;
            let AS = L * (AL - node_potentials[0]);
            let AT = (Lanes([0.0, AM]) - Lanes([AR, 0.0])) * L;
            let AW = L * (AU - I);
            let AX = (Lanes([0.0, AV]) - Lanes([K, 0.0])) * L;
            let AZ = L * (node_potentials[9] - AU);
            let BA = (Lanes([AY, 0.0]) - Lanes([0.0, AV])) * L;
            let BB = Lanes([AB[0], AB[1], 0.0]) + Lanes([0.0, R[0], R[1]]);
            let BC = Lanes([BB[0], BB[1], 0.0, BB[2]]) - Lanes([0.0, 0.0, AG[0], AG[1]]);
            let BD = ((AA + Q) - AF) - AW;
            let BE = Lanes([BC[0], BC[1], BC[2], BC[3], 0.0]) - Lanes([0.0, 0.0, AX[0], 0.0, AX[1]]);
            let BG = AT * BF;
            let BH = Lanes([BG[0], BG[1], 0.0]) + Lanes([0.0, AO[0], AO[1]]);
            let BI = Lanes([BH[0], BH[1], BH[2], 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, BE[0], BE[1], BE[2], BE[3], BE[4]]);
            let BJ = (((-AS) + AN) + BD) - AZ;
            let BK = Lanes([BI[0], BI[1], BI[2], BI[3], BI[4], BI[5], 0.0, BI[6]]) - Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, BA[0], BA[1]]);
            let BL = AS + BJ;
            let BM = Lanes([AT[0], AT[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + BK;
            let BN = AD - AW;
            let BO = Lanes([AE[0], AE[1], 0.0]) - Lanes([0.0, AX[0], AX[1]]);
            let BP = BN - AZ;
            let BQ = Lanes([BO[0], BO[1], 0.0, BO[2]]) - Lanes([0.0, 0.0, BA[0], BA[1]]);
            let BS = Q * BR;
            let BT = R * BR;
            let BV = if BS < BU { 1.0 } else { 0.0 };
            let CC;
            let CD;
            if BV != 0.0 {
                let BW = BS.exp();
                let BX = BT * BW;
                CC = BW;
                CD = BX;
            } else {
                let BY = BU.exp();
                let CA = BY * (BZ + (BS - BU));
                let CB = BT * BY;
                CC = CA;
                CD = CB;
            }
            let CE = U * BR;
            let CF = V * BR;
            let CH = CE / CG;
            let CI = CF / CG;
            let CJ = if CH < BU { 1.0 } else { 0.0 };
            let CP;
            let CQ;
            if CJ != 0.0 {
                let CK = CH.exp();
                let CL = CI * CK;
                CP = CK;
                CQ = CL;
            } else {
                let CM = BU.exp();
                let CN = CM * (BZ + (CH - BU));
                let CO = CI * CM;
                CP = CN;
                CQ = CO;
            }
            let CR = BD * BR;
            let CS = BE * BR;
            let CT = if CR < BU { 1.0 } else { 0.0 };
            let CZ;
            let DA;
            if CT != 0.0 {
                let CU = CR.exp();
                let CV = CS * CU;
                CZ = CU;
                DA = CV;
            } else {
                let CW = BU.exp();
                let CX = CW * (BZ + (CR - BU));
                let CY = CS * CW;
                CZ = CX;
                DA = CY;
            }
            let DB = AA * BR;
            let DC = AB * BR;
            let DD = if DB < BU { 1.0 } else { 0.0 };
            let DJ;
            let DK;
            if DD != 0.0 {
                let DE = DB.exp();
                let DF = DC * DE;
                DJ = DE;
                DK = DF;
            } else {
                let DG = BU.exp();
                let DH = DG * (BZ + (DB - BU));
                let DI = DC * DG;
                DJ = DH;
                DK = DI;
            }
            let DL = BL * BR;
            let DM = BM * BR;
            let DN = if DL < BU { 1.0 } else { 0.0 };
            let DT;
            let DU;
            if DN != 0.0 {
                let DO = DL.exp();
                let DP = DM * DO;
                DT = DO;
                DU = DP;
            } else {
                let DQ = BU.exp();
                let DR = DQ * (BZ + (DL - BU));
                let DS = DM * DQ;
                DT = DR;
                DU = DS;
            }
            let DV = AD * BR;
            let DW = AE * BR;
            let DX = if DV < BU { 1.0 } else { 0.0 };
            let ED;
            let EE;
            if DX != 0.0 {
                let DY = DV.exp();
                let DZ = DW * DY;
                ED = DY;
                EE = DZ;
            } else {
                let EA = BU.exp();
                let EB = EA * (BZ + (DV - BU));
                let EC = DW * EA;
                ED = EB;
                EE = EC;
            }
            let EF = BP * BR;
            let EG = BQ * BR;
            let EH = if EF < BU { 1.0 } else { 0.0 };
            let EN;
            let EO;
            if EH != 0.0 {
                let EI = EF.exp();
                let EJ = EG * EI;
                EN = EI;
                EO = EJ;
            } else {
                let EK = BU.exp();
                let EL = EK * (BZ + (EF - BU));
                let EM = EG * EK;
                EN = EL;
                EO = EM;
            }
            let EP = BN * BR;
            let EQ = BO * BR;
            let ER = if EP < BU { 1.0 } else { 0.0 };
            let EX;
            let EY;
            if ER != 0.0 {
                let ES = EP.exp();
                let ET = EQ * ES;
                EX = ES;
                EY = ET;
            } else {
                let EU = BU.exp();
                let EV = EU * (BZ + (EP - BU));
                let EW = EQ * EU;
                EX = EV;
                EY = EW;
            }
            let FA = (BL - EZ) * BR;
            let FB = if FA < BU { 1.0 } else { 0.0 };
            let FH;
            let FI;
            if FB != 0.0 {
                let FC = FA.exp();
                let FD = DM * FC;
                FH = FC;
                FI = FD;
            } else {
                let FE = BU.exp();
                let FF = FE * (BZ + (FA - BU));
                let FG = DM * FE;
                FH = FF;
                FI = FG;
            }
            let FJ = (BD - EZ) * BR;
            let FK = if FJ < BU { 1.0 } else { 0.0 };
            let FQ;
            let FR;
            if FK != 0.0 {
                let FL = FJ.exp();
                let FM = CS * FL;
                FQ = FL;
                FR = FM;
            } else {
                let FN = BU.exp();
                let FO = FN * (BZ + (FJ - BU));
                let FP = CS * FN;
                FQ = FO;
                FR = FP;
            }
            let FS = (Q - EZ) * BR;
            let FT = if FS < BU { 1.0 } else { 0.0 };
            let FZ;
            let GA;
            if FT != 0.0 {
                let FU = FS.exp();
                let FV = BT * FU;
                FZ = FU;
                GA = FV;
            } else {
                let FW = BU.exp();
                let FX = FW * (BZ + (FS - BU));
                let FY = BT * FW;
                FZ = FX;
                GA = FY;
            }
            let GB = (M - EZ) * BR;
            let GC = N * BR;
            let GD = if GB < BU { 1.0 } else { 0.0 };
            let GJ;
            let GK;
            if GD != 0.0 {
                let GE = GB.exp();
                let GF = GC * GE;
                GJ = GE;
                GK = GF;
            } else {
                let GG = BU.exp();
                let GH = GG * (BZ + (GB - BU));
                let GI = GC * GG;
                GJ = GH;
                GK = GI;
            }
            let GM = (BZ + (GL * FZ)).sqrt();
            let GP = (GA * GL) * (GO / (GN * GM));
            let GQ = (BZ + (GL * GJ)).sqrt();
            let GR = (GK * GL) * (GO / (GN * GQ));
            let GT = BZ + GQ;
            let GU = (GS * GJ) / GT;
            let GV = ((GK * GS) - (GR * GU)) / GT;
            let GX = if GU < GW { 1.0 } else { 0.0 };
            let GZ;
            let HA;
            if GX != 0.0 {
                GZ = GW;
                HA = GY;
            } else {
                GZ = GU;
                HA = GV;
            }
            let HB = Lanes([GP[0], 0.0, GP[1]]);
            let HC = GM + BZ;
            let HD = HC / GT;
            let HE = GR * HD;
            let HG = HF * ((GM - GQ) - (HD.ln()));
            let HH = ((HB - Lanes([GR[0], GR[1], 0.0])) - (((HB - Lanes([HE[0], HE[1], 0.0])) / GT) * (GO / HD))) * HF;
            let HI = Lanes([0.0, AG[0], AG[1]]);
            let HK = (HG + AF) / HJ;
            let HL = (HH + HI) / HJ;
            let HM = if HK > A { 1.0 } else { 0.0 };
            let HS;
            let HT;
            let HU;
            let HV;
            let HW;
            let HX;
            let HY;
            let HZ;
            let IA;
            let IB;
            let IC;
            let ID;
            let IE;
            let IF;
            if HM != 0.0 {
                let HO = if M < HN { 1.0 } else { 0.0 };
                let IO;
                let IP;
                if HO != 0.0 {
                    IO = M;
                    IP = N;
                } else {
                    let IL = BZ + (M - HN);
                    let IM = N * (GO / IL);
                    let IN = HN + (IL.ln());
                    IO = IN;
                    IP = IM;
                }
                let IQ = GS * HF;
                let IS = (IR * HK) * HJ;
                let IT = (HL * IR) * HJ;
                let IU = (IS * BR) + BZ;
                let IV = (EZ + (IQ * (IU.ln()))) - IO;
                let IW = (((IT * BR) * (GO / IU)) * IQ) - Lanes([IP[0], IP[1], 0.0]);
                let IY = IX * EZ;
                let IZ = IY * IY;
                let JA = IV * IV;
                let JB = IW * IV;
                let JC = JB + JB;
                let JD = if IV < A { 1.0 } else { 0.0 };
                let JL;
                let JM;
                if JD != 0.0 {
                    let JE = (JA + IZ).sqrt();
                    let JF = JE - IV;
                    let JG = (IR * IZ) / JF;
                    let JH = ((((JC * (GO / (GN * JE))) - IW) * JG) * BF) / JF;
                    JL = JG;
                    JM = JH;
                } else {
                    let JI = (JA + IZ).sqrt();
                    let JJ = IR * (JI + IV);
                    let JK = ((JC * (GO / (GN * JI))) + IW) * IR;
                    JL = JJ;
                    JM = JK;
                }
                let JP = JN * JO;
                let JQ = JL + JP;
                let JR = JO * (JL + (JN * HJ));
                let JS = (JL * JQ) / JR;
                let JT = (((JM * JQ) + (JM * JL)) - ((JM * JO) * JS)) / JR;
                let JU = HK / JS;
                let JV = (HL - (JT * JU)) / JS;
                let JX = (JU - BZ) / JW;
                let JY = JV / JW;
                let JZ = if JU < BZ { 1.0 } else { 0.0 };
                let KI;
                let KJ;
                if JZ != 0.0 {
                    let KA = JX.exp();
                    let KB = BZ + KA;
                    let KC = ((JY * KA) * (GO / KB)) * JW;
                    let KD = BZ + (JW * (KB.ln()));
                    KI = KD;
                    KJ = KC;
                } else {
                    let KE = (-JX).exp();
                    let KF = BZ + KE;
                    let KG = JU + (JW * (KF.ln()));
                    let KH = JV + ((((JY * BF) * KE) * (GO / KF)) * JW);
                    KI = KG;
                    KJ = KH;
                }
                let KK = BZ + (JW * ((BZ + ((-1e0f64 / JW).exp())).ln()));
                let KL = KI / KK;
                let KM = KJ / KK;
                let KN = JL / JP;
                let KO = JM / JP;
                let KP = GL * KL;
                let KQ = KP * KN;
                let KR = BZ + KN;
                let KS = (BZ + (KQ * KR)).sqrt();
                let KT = GS * KL;
                let KU = KT * KR;
                let KV = (BZ + KS) / KU;
                let KW = (((((((KM * GL) * KN) + (KO * KP)) * KR) + (KO * KQ)) * (GO / (GN * KS))) - ((((KM * GS) * KR) + (KO * KT)) * KV)) / KU;
                let KX = GZ * KV;
                let KY = HA * KV;
                let KZ = Lanes([KY[0], KY[1], 0.0]) + (KW * GZ);
                let LA = BZ + KX;
                let LB = ((BZ - KV) + KX) / LA;
                let LC = (((KW * BF) + KZ) - (KZ * LB)) / LA;
                let LD = (IS * LB) * BR;
                let LE = ((IT * LB) + (LC * IS)) * BR;
                let LF = (GZ + LD) + BZ;
                let LG = HA * LF;
                let LH = (GS * LD) + (GZ * LF);
                let LI = (LE * GS) + (Lanes([LG[0], LG[1], 0.0]) + ((Lanes([HA[0], HA[1], 0.0]) + LE) * GZ));
                let LJ = IR * (LD - BZ);
                let LK = LE * IR;
                let LL = LK * LJ;
                let LM = (LJ * LJ) + LH;
                let LN = (LL + LL) + LI;
                let LO = if LD >= BZ { 1.0 } else { 0.0 };
                let LW;
                let LX;
                if LO != 0.0 {
                    let LP = LM.sqrt();
                    let LQ = LJ + LP;
                    let LR = LK + (LN * (GO / (GN * LP)));
                    LW = LQ;
                    LX = LR;
                } else {
                    let LS = LM.sqrt();
                    let LT = LS - LJ;
                    let LU = LH / LT;
                    let LV = (LI - (((LN * (GO / (GN * LS))) - LK) * LU)) / LT;
                    LW = LU;
                    LX = LV;
                }
                let LZ = if LW < LY { 1.0 } else { 0.0 };
                let MB;
                let MC;
                if LZ != 0.0 {
                    MB = LY;
                    MC = MA;
                } else {
                    MB = LW;
                    MC = LX;
                }
                let MD = MB + BZ;
                let ME = (EZ * BR).exp();
                let MF = (MB * MD) * ME;
                let MG = ((MC * MD) + (MC * MB)) * ME;
                let MH = IR * JO;
                let MI = MH * (HK - JN);
                let MJ = HL * MH;
                let MK = (JO * HJ) * JN;
                let ML = MJ * MI;
                let MM = ((MI * MI) + (MK * HK)).sqrt();
                let MN = MI + MM;
                let MO = MJ + (((ML + ML) + (HL * MK)) * (GO / (GN * MM)));
                let MP = if parameters[73] == A { 1.0 } else { 0.0 };
                let MX;
                let MY;
                if MP != 0.0 {
                    let MS = MQ * MR;
                    MX = MS;
                    MY = MA;
                } else {
                    let MT = HK + JS;
                    let MU = (GS * HK) / MT;
                    let MV = MQ * (MR + MU);
                    let MW = (((HL * GS) - ((HL + JT) * MU)) / MT) * MQ;
                    MX = MV;
                    MY = MW;
                }
                let MZ = JN + HK;
                let NA = (JN * HK) / MZ;
                let NB = ((HL * JN) - (HL * NA)) / MZ;
                let NC = JN / MZ;
                let ND = ((HL * NC) * BF) / MZ;
                HS = MN;
                HT = MX;
                HU = NC;
                HV = MF;
                HW = LB;
                HX = NA;
                HY = MB;
                HZ = MO;
                IA = MY;
                IB = ND;
                IC = MG;
                ID = LC;
                IE = NB;
                IF = MC;
            } else {
                let HP = (GS * FZ) / HC;
                let HQ = ((GA * GS) - (GP * HP)) / HC;
                let HR = if (if (AF.abs()) < (1e-5f64 * HF) { 1.0 } else { 0.0 }) != 0.0 || (if (HG.abs()) < ((1e-40f64 * HF) * (GM + GQ)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let NM;
                let NN;
                if HR != 0.0 {
                    let NE = IR * (HP + GZ);
                    let NF = (Lanes([HQ[0], 0.0, HQ[1]]) + Lanes([HA[0], HA[1], 0.0])) * IR;
                    let NG = NE + BZ;
                    let NH = NE / NG;
                    let NI = (NF - (NF * NH)) / NG;
                    NM = NH;
                    NN = NI;
                } else {
                    let NJ = (HG + Q) - M;
                    let NK = HG / NJ;
                    let NL = (HH - (((HH + Lanes([R[0], 0.0, R[1]])) - Lanes([N[0], N[1], 0.0])) * NK)) / NJ;
                    NM = NK;
                    NN = NL;
                }
                let NO = MR * MQ;
                let NP = BZ - (HK / JN);
                let NQ = (HL / JN) * BF;
                let NR = Lanes([CD[0], 0.0, CD[1]]);
                let NS = Lanes([HQ[0], 0.0, HQ[1]]);
                HS = AF;
                HT = NO;
                HU = NP;
                HV = CC;
                HW = NM;
                HX = HK;
                HY = HP;
                HZ = HI;
                IA = MA;
                IB = NQ;
                IC = NR;
                ID = NN;
                IE = HL;
                IF = NS;
            }
            let II = (U - IG) / IH;
            let IJ = V / IH;
            let IK = if U < IG { 1.0 } else { 0.0 };
            let OB;
            let OC;
            if IK != 0.0 {
                let NT = II.exp();
                let NU = BZ + NT;
                let NV = U - (IH * (NU.ln()));
                let NW = V - (((IJ * NT) * (GO / NU)) * IH);
                OB = NV;
                OC = NW;
            } else {
                let NX = (-II).exp();
                let NY = BZ + NX;
                let NZ = IG - (IH * (NY.ln()));
                let OA = ((((IJ * BF) * NX) * (GO / NY)) * IH) * BF;
                OB = NZ;
                OC = OA;
            }
            let OE = BZ - (OB * OD);
            let OF = (OC * OD) * BF;
            let OH = OE.powf(OG);
            let OJ = OF * (OG * (OE.powf(OI)));
            let OM = (OK * (BZ - OH)) + (OL * (U - OB));
            let ON = ((OJ * BF) * OK) + ((V - OC) * OL);
            let OR;
            let OS;
            if OO != 0.0 {
                let OP = Lanes([N[0], N[1], 0.0]);
                OR = M;
                OS = OP;
            } else {
                let PA;
                let PB;
                if OQ != 0.0 {
                    let OX = M + HS;
                    let OY = Lanes([N[0], N[1], 0.0]) + HZ;
                    PA = OX;
                    PB = OY;
                } else {
                    let OZ = Lanes([R[0], 0.0, R[1]]);
                    PA = Q;
                    PB = OZ;
                }
                OR = PA;
                OS = PB;
            }
            let OU = (OR - OT) / HT;
            let OV = (OS - (IA * OU)) / HT;
            let OW = if OR < OT { 1.0 } else { 0.0 };
            let PM;
            let PN;
            if OW != 0.0 {
                let PC = OU.exp();
                let PD = BZ + PC;
                let PE = PD.ln();
                let PF = OR - (HT * PE);
                let PG = OS - ((IA * PE) + (((OV * PC) * (GO / PD)) * HT));
                PM = PF;
                PN = PG;
            } else {
                let PH = (-OU).exp();
                let PI = BZ + PH;
                let PJ = PI.ln();
                let PK = OT - (HT * PJ);
                let PL = ((IA * PJ) + ((((OV * BF) * PH) * (GO / PI)) * HT)) * BF;
                PM = PK;
                PN = PL;
            }
            let PP = HU.powf(PO);
            let PQ = IB * (PO * (HU.powf(staged[170])));
            let PR = BZ - (PM / MQ);
            let PT = PR.powf(PS);
            let PX = PP * PW;
            let PY = OR - PM;
            let QB = N * QA;
            let QC = (PZ * ((PV * (BZ - (PP * PT))) + (PX * PY))) + (QA * M);
            let QD = ((((((PQ * PT) + ((((PN / MQ) * BF) * (PS * (PR.powf(PU)))) * PP)) * BF) * PV) + (((PQ * PW) * PY) + ((OS - PN) * PX))) * PZ) + Lanes([QB[0], QB[1], 0.0]);
            let QF = QE * CP;
            let QG = CQ * QE;
            let QH = (BZ + QF).sqrt();
            let QI = QG * (GO / (GN * QH));
            let QJ = BZ + QH;
            let QK = QF / QJ;
            let QL = (QG - (QI * QK)) / QJ;
            let QN = HV.powf(QM);
            let QO = IC * (QM * (HV.powf(staged[172])));
            let QP = QE * QN;
            let QQ = QO * QE;
            let QR = (BZ + QP).sqrt();
            let QS = BZ + QR;
            let QT = QP / QS;
            let QU = (QQ - ((QQ * (GO / (GN * QR))) * QT)) / QS;
            let RK;
            let RL;
            if QV != 0.0 {
                let QX = ON / QW;
                let QZ = QD / QY;
                let RA = (BZ + (OM / QW)) + (QC / QY);
                let RB = Lanes([QX[0], QX[1], 0.0, 0.0]) + Lanes([0.0, QZ[0], QZ[1], QZ[2]]);
                RK = RA;
                RL = RB;
            } else {
                let RD = ((((OM / QW) + BZ) * RC) * BR).exp();
                let RE = (((ON / QW) * RC) * BR) * RD;
                let RF = ((((-QC) / QY) * RC) * BR).exp();
                let RG = ((((QD * BF) / QY) * RC) * BR) * RF;
                let RI = (RD - RF) / RH;
                let RJ = (Lanes([RE[0], RE[1], 0.0, 0.0]) - Lanes([0.0, RG[0], RG[1], RG[2]])) / RH;
                RK = RI;
                RL = RJ;
            }
            let RM = RK * RK;
            let RN = RL * RK;
            let RO = RN + RN;
            let RP = if RK < A { 1.0 } else { 0.0 };
            let RY;
            let RZ;
            if RP != 0.0 {
                let RR = (RM + RQ).sqrt();
                let RS = RR - RK;
                let RT = 5.000000000000001e-3f64 / RS;
                let RU = ((((RO * (GO / (GN * RR))) - RL) * RT) * BF) / RS;
                RY = RT;
                RZ = RU;
            } else {
                let RV = (RM + RQ).sqrt();
                let RW = IR * (RV + RK);
                let RX = ((RO * (GO / (GN * RV))) + RL) * IR;
                RY = RW;
                RZ = RX;
            }
            let SA = (Lanes([QL[0], QL[1], 0.0, 0.0]) + Lanes([0.0, QU[0], QU[1], QU[2]])) * IR;
            let SB = BZ + (IR * (QK + QT));
            let SC = RY * SB;
            let SD = (RZ * SB) + (SA * RY);
            let SF = SE * QN;
            let SG = QO * SE;
            let SI = SH * CP;
            let SJ = CQ * SH;
            let SK = Lanes([SJ[0], SJ[1], 0.0, 0.0]);
            let SL = Lanes([0.0, SG[0], SG[1], SG[2]]);
            let SM = (SI - SF) / SC;
            let SN = ((SK - SL) - (SD * SM)) / SC;
            let SP = U / SO;
            let SQ = V / SO;
            let SR = if U < A { 1.0 } else { 0.0 };
            let TA;
            let TB;
            if SR != 0.0 {
                let SS = SP.exp();
                let ST = BZ + SS;
                let SU = SO * (ST.ln());
                let SV = ((SQ * SS) * (GO / ST)) * SO;
                TA = SU;
                TB = SV;
            } else {
                let SW = (-SP).exp();
                let SX = BZ + SW;
                let SY = U + (SO * (SX.ln()));
                let SZ = V + ((((SQ * BF) * SW) * (GO / SX)) * SO);
                TA = SY;
                TB = SZ;
            }
            let TD = TA / TC;
            let TE = TB / TC;
            let TF = if TD < BU { 1.0 } else { 0.0 };
            let TL;
            let TM;
            if TF != 0.0 {
                let TG = TD.exp();
                let TH = TE * TG;
                TL = TG;
                TM = TH;
            } else {
                let TI = BU.exp();
                let TJ = TI * (BZ + (TD - BU));
                let TK = TE * TI;
                TL = TJ;
                TM = TK;
            }
            let TO = TN * (TL - BZ);
            let TP = TM * TN;
            let TS = (U - TQ) / TR;
            let TT = V / TR;
            let TU = if U < TQ { 1.0 } else { 0.0 };
            let UD;
            let UE;
            if TU != 0.0 {
                let TV = TS.exp();
                let TW = BZ + TV;
                let TX = U - (TR * (TW.ln()));
                let TY = V - (((TT * TV) * (GO / TW)) * TR);
                UD = TX;
                UE = TY;
            } else {
                let TZ = (-TS).exp();
                let UA = BZ + TZ;
                let UB = TQ - (TR * (UA.ln()));
                let UC = ((((TT * BF) * TZ) * (GO / UA)) * TR) * BF;
                UD = UB;
                UE = UC;
            }
            let UG = UF * UD;
            let UH = TQ - UD;
            let UI = UH * UH;
            let UJ = UG * UI;
            let UK = ((UE * UF) * UI) + (((UE * BF) * (GS * UH)) * UG);
            let UM = CE / UL;
            let UN = CF / UL;
            let UO = if UM < BU { 1.0 } else { 0.0 };
            let UU;
            let UV;
            if UO != 0.0 {
                let UP = UM.exp();
                let UQ = UN * UP;
                UU = UP;
                UV = UQ;
            } else {
                let UR = BU.exp();
                let US = UR * (BZ + (UM - BU));
                let UT = UN * UR;
                UU = US;
                UV = UT;
            }
            let VA;
            let VB;
            if E != 0.0 {
                let UX = (U - UW) * BR;
                let UY = if UX < BU { 1.0 } else { 0.0 };
                let VN;
                let VO;
                if UY != 0.0 {
                    let VI = UX.exp();
                    let VJ = CF * VI;
                    VN = VI;
                    VO = VJ;
                } else {
                    let VK = BU.exp();
                    let VL = VK * (BZ + (UX - BU));
                    let VM = CF * VK;
                    VN = VL;
                    VO = VM;
                }
                let VP = SN / SH;
                let VQ = (SM / SH) - 1e3f64;
                let VS = if VQ < VR { 1.0 } else { 0.0 };
                let VY;
                let VZ;
                if VS != 0.0 {
                    let VT = VQ.exp();
                    let VU = VP * VT;
                    VY = VT;
                    VZ = VU;
                } else {
                    let VW = VV * (BZ + (VQ - VR));
                    let VX = VP * VV;
                    VY = VW;
                    VZ = VX;
                }
                let WA = UU - BZ;
                let WC = UV * WB;
                let WE = (BZ + (GL * VN)).sqrt();
                let WF = BZ + WE;
                let WG = (WD * WA) / WF;
                let WH = BZ + (QC / QY);
                let WI = (((UV * WD) - (((VO * GL) * (GO / (GN * WE))) * WG)) / WF) * WH;
                let WJ = (QD / QY) * WG;
                let WL = WK * (HV - BZ);
                let WM = (IC * WK) * VY;
                let WN = BZ + VY;
                let WO = (WL * VY) / WN;
                let WP = ((WB * WA) + (WG * WH)) + WO;
                let WQ = (Lanes([WC[0], WC[1], 0.0, 0.0]) + (Lanes([WI[0], WI[1], 0.0, 0.0]) + Lanes([0.0, WJ[0], WJ[1], WJ[2]]))) + (((Lanes([0.0, WM[0], WM[1], WM[2]]) + (VZ * WL)) - (VZ * WO)) / WN);
                VA = WP;
                VB = WQ;
            } else {
                let XC;
                let XD;
                if UZ != 0.0 {
                    let WR = WB * (UU - BZ);
                    let WS = UV * WB;
                    let WT = Lanes([WS[0], WS[1], 0.0, 0.0]);
                    XC = WR;
                    XD = WT;
                } else {
                    let WV = UV * WU;
                    let WX = WW * ((UU + HV) - GS);
                    let WY = BZ + (QC / QY);
                    let WZ = (QD / QY) * WX;
                    let XA = WB * ((WU * (UU - BZ)) + (WX * WY));
                    let XB = (Lanes([WV[0], WV[1], 0.0, 0.0]) + ((((Lanes([UV[0], UV[1], 0.0, 0.0]) + Lanes([0.0, IC[0], IC[1], IC[2]])) * WW) * WY) + Lanes([0.0, WZ[0], WZ[1], WZ[2]]))) * WB;
                    XC = XA;
                    XD = XB;
                }
                VA = XC;
                VB = XD;
            }
            let VC = Y * BR;
            let VD = Z * BR;
            let VF = VC / VE;
            let VG = VD / VE;
            let VH = if VF < BU { 1.0 } else { 0.0 };
            let XJ;
            let XK;
            if VH != 0.0 {
                let XE = VF.exp();
                let XF = VG * XE;
                XJ = XE;
                XK = XF;
            } else {
                let XG = BU.exp();
                let XH = XG * (BZ + (VF - BU));
                let XI = VG * XG;
                XJ = XH;
                XK = XI;
            }
            let XQ;
            let XR;
            if E != 0.0 {
                let XL = (Y - UW) * BR;
                let XM = if XL < BU { 1.0 } else { 0.0 };
                let YB;
                let YC;
                if XM != 0.0 {
                    let XW = XL.exp();
                    let XX = VD * XW;
                    YB = XW;
                    YC = XX;
                } else {
                    let XY = BU.exp();
                    let XZ = XY * (BZ + (XL - BU));
                    let YA = VD * XY;
                    YB = XZ;
                    YC = YA;
                }
                let YD = XJ - BZ;
                let YF = (BZ + (GL * YB)).sqrt();
                let YG = BZ + YF;
                let YH = (YE * YD) / YG;
                let YI = (XN * YD) + YH;
                let YJ = (XK * XN) + (((XK * YE) - (((YC * GL) * (GO / (GN * YF))) * YH)) / YG);
                XQ = YI;
                XR = YJ;
            } else {
                let XO = XN * (XJ - BZ);
                let XP = XK * XN;
                XQ = XO;
                XR = XP;
            }
            let XT = CE / XS;
            let XU = CF / XS;
            let XV = if XT < BU { 1.0 } else { 0.0 };
            let YP;
            let YQ;
            if XV != 0.0 {
                let YK = XT.exp();
                let YL = XU * YK;
                YP = YK;
                YQ = YL;
            } else {
                let YM = BU.exp();
                let YN = YM * (BZ + (XT - BU));
                let YO = XU * YM;
                YP = YN;
                YQ = YO;
            }
            let YS = YR * (YP - BZ);
            let YT = YQ * YR;
            let YV = VC / YU;
            let YW = VD / YU;
            let YX = if YV < BU { 1.0 } else { 0.0 };
            let ZD;
            let ZE;
            if YX != 0.0 {
                let YY = YV.exp();
                let YZ = YW * YY;
                ZD = YY;
                ZE = YZ;
            } else {
                let ZA = BU.exp();
                let ZB = ZA * (BZ + (YV - BU));
                let ZC = YW * ZA;
                ZD = ZB;
                ZE = ZC;
            }
            let ZG = ZF * (ZD - BZ);
            let ZH = ZE * ZF;
            let ZJ = CR / ZI;
            let ZK = CS / ZI;
            let ZL = if ZJ < BU { 1.0 } else { 0.0 };
            let ZR;
            let ZS;
            if ZL != 0.0 {
                let ZM = ZJ.exp();
                let ZN = ZK * ZM;
                ZR = ZM;
                ZS = ZN;
            } else {
                let ZO = BU.exp();
                let ZP = ZO * (BZ + (ZJ - BU));
                let ZQ = ZK * ZO;
                ZR = ZP;
                ZS = ZQ;
            }
            let ZU = ZT * (ZR - BZ);
            let ZV = ZS * ZT;
            let ZX = VC / ZW;
            let ZY = VD / ZW;
            let ZZ = if ZX < BU { 1.0 } else { 0.0 };
            let AAF;
            let AAG;
            if ZZ != 0.0 {
                let AAA = ZX.exp();
                let AAB = ZY * AAA;
                AAF = AAA;
                AAG = AAB;
            } else {
                let AAC = BU.exp();
                let AAD = AAC * (BZ + (ZX - BU));
                let AAE = ZY * AAC;
                AAF = AAD;
                AAG = AAE;
            }
            let AAI = AAH * (AAF - BZ);
            let AAJ = AAG * AAH;
            let AAK = if staged[81] != 0.0 && SR != 0.0 { 1.0 } else { 0.0 };
            let AAT;
            let AAU;
            if AAK != 0.0 {
                let AAL = GS * OH;
                let AAN = AAM / AAL;
                let AAP = AAO * (BZ - AAN);
                let AAQ = (((((OJ * GS) * AAN) * BF) / AAL) * BF) * AAO;
                let AAR = if AAP < BU { 1.0 } else { 0.0 };
                let ABB;
                let ABC;
                if AAR != 0.0 {
                    let AAW = AAP.exp();
                    let AAX = AAQ * AAW;
                    ABB = AAW;
                    ABC = AAX;
                } else {
                    let AAY = BU.exp();
                    let AAZ = AAY * (BZ + (AAP - BU));
                    let ABA = AAQ * AAY;
                    ABB = AAZ;
                    ABC = ABA;
                }
                let ABD = U * OD;
                let ABE = V * OD;
                let ABF = ABE * ABD;
                let ABH = ((ABD * ABD) + ABG).sqrt();
                let ABJ = -2e0f64 - ABI;
                let ABK = ABH.powf(ABJ);
                let ABL = ABI - BZ;
                let ABN = ABM * ABD;
                let ABO = ABN * ABD;
                let ABP = ABL + ABD;
                let ABQ = (ABI * ((BZ - (ABI * ABI)) - ((OL * ABD) * ABL))) - (ABO * ABP);
                let ABT = ABS * ((ABK * ABQ) * ABR);
                let ABU = ((U * AAM) * AAO) / ABT;
                let ABV = (((V * AAM) * AAO) - ((((((((ABF + ABF) * (GO / (GN * ABH))) * (ABJ * (ABH.powf((ABJ - GO))))) * ABQ) + ((((((ABE * OL) * ABL) * BF) * ABI) - (((((ABE * ABM) * ABD) + (ABE * ABN)) * ABP) + (ABE * ABO))) * ABK)) * ABR) * ABS) * ABU)) / ABT;
                let ABW = if ABU < -1e-3f64 { 1.0 } else { 0.0 };
                let ACH;
                let ACI;
                if ABW != 0.0 {
                    let ABX = if ABU < BU { 1.0 } else { 0.0 };
                    let ACU;
                    let ACV;
                    if ABX != 0.0 {
                        let ACP = ABU.exp();
                        let ACQ = ABV * ACP;
                        ACU = ACP;
                        ACV = ACQ;
                    } else {
                        let ACR = BU.exp();
                        let ACS = ACR * (BZ + (ABU - BU));
                        let ACT = ABV * ACR;
                        ACU = ACS;
                        ACV = ACT;
                    }
                    let ACW = -U;
                    let ACX = (BZ - ACU) / ABU;
                    let ACY = BZ + ACX;
                    let ACZ = ACW * ACY;
                    let ADA = ((V * BF) * ACY) + ((((ACV * BF) - (ABV * ACX)) / ABU) * ACW);
                    ACH = ACZ;
                    ACI = ADA;
                } else {
                    let ABY = U * IR;
                    let ABZ = ABY * ABU;
                    let ACB = ABU * ACA;
                    let ACD = BZ + (ACC * ABU);
                    let ACE = BZ + (ACB * ACD);
                    let ACF = ABZ * ACE;
                    let ACG = ((((V * IR) * ABU) + (ABV * ABY)) * ACE) + ((((ABV * ACA) * ACD) + ((ABV * ACC) * ACB)) * ABZ);
                    ACH = ACF;
                    ACI = ACG;
                }
                let ACJ = GS * staged[85];
                let ACK = ACJ * ACH;
                let ACL = ACK * OH;
                let ACN = ((ACL * ABB) * OD) * ACM;
                let ACO = ((((((ACI * ACJ) * OH) + (OJ * ACK)) * ABB) + (ABC * ACL)) * OD) * ACM;
                AAT = ACN;
                AAU = ACO;
            } else {
                AAT = A;
                AAU = AAS;
            }
            let AAV = if staged[87] != 0.0 && (if M < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ADO;
            let ADP;
            if AAV != 0.0 {
                let ADC = M * ADB;
                let ADD = N * ADB;
                let ADE = BZ - ADC;
                let ADF = ADE.powf(PS);
                let ADG = (ADD * BF) * (PS * (ADE.powf(PU)));
                let ADH = GS * ADF;
                let ADJ = ADI / ADH;
                let ADL = ADK * (BZ - ADJ);
                let ADM = (((((ADG * GS) * ADJ) * BF) / ADH) * BF) * ADK;
                let ADN = if ADL < BU { 1.0 } else { 0.0 };
                let AEP;
                let AEQ;
                if ADN != 0.0 {
                    let AEK = ADL.exp();
                    let AEL = ADM * AEK;
                    AEP = AEK;
                    AEQ = AEL;
                } else {
                    let AEM = BU.exp();
                    let AEN = AEM * (BZ + (ADL - BU));
                    let AEO = ADM * AEM;
                    AEP = AEN;
                    AEQ = AEO;
                }
                let AER = ADD * ADC;
                let AES = ((ADC * ADC) + ABG).sqrt();
                let AEU = -2e0f64 - AET;
                let AEV = AES.powf(AEU);
                let AEW = AET - BZ;
                let AEX = ABM * ADC;
                let AEY = AEX * ADC;
                let AEZ = AEW + ADC;
                let AFA = (AET * ((BZ - (AET * AET)) - ((OL * ADC) * AEW))) - (AEY * AEZ);
                let AFC = AFB * ((AEV * AFA) * ABR);
                let AFD = ((M * ADI) * ADK) / AFC;
                let AFE = (((N * ADI) * ADK) - ((((((((AER + AER) * (GO / (GN * AES))) * (AEU * (AES.powf((AEU - GO))))) * AFA) + ((((((ADD * OL) * AEW) * BF) * AET) - (((((ADD * ABM) * ADC) + (ADD * AEX)) * AEZ) + (ADD * AEY))) * AEV)) * ABR) * AFB) * AFD)) / AFC;
                let AFF = if AFD < -1e-3f64 { 1.0 } else { 0.0 };
                let AFO;
                let AFP;
                if AFF != 0.0 {
                    let AFG = if AFD < BU { 1.0 } else { 0.0 };
                    let AGB;
                    let AGC;
                    if AFG != 0.0 {
                        let AFW = AFD.exp();
                        let AFX = AFE * AFW;
                        AGB = AFW;
                        AGC = AFX;
                    } else {
                        let AFY = BU.exp();
                        let AFZ = AFY * (BZ + (AFD - BU));
                        let AGA = AFE * AFY;
                        AGB = AFZ;
                        AGC = AGA;
                    }
                    let AGD = -M;
                    let AGE = (BZ - AGB) / AFD;
                    let AGF = BZ + AGE;
                    let AGG = AGD * AGF;
                    let AGH = ((N * BF) * AGF) + ((((AGC * BF) - (AFE * AGE)) / AFD) * AGD);
                    AFO = AGG;
                    AFP = AGH;
                } else {
                    let AFH = M * IR;
                    let AFI = AFH * AFD;
                    let AFJ = AFD * ACA;
                    let AFK = BZ + (ACC * AFD);
                    let AFL = BZ + (AFJ * AFK);
                    let AFM = AFI * AFL;
                    let AFN = ((((N * IR) * AFD) + (AFE * AFH)) * AFL) + ((((AFE * ACA) * AFK) + ((AFE * ACC) * AFJ)) * AFI);
                    AFO = AFM;
                    AFP = AFN;
                }
                let AFQ = GS * staged[92];
                let AFR = AFQ * AFO;
                let AFS = AFR * ADF;
                let AFU = ((AFS * AEP) * ADB) * AFT;
                let AFV = ((((((AFP * AFQ) * ADF) + (ADG * AFR)) * AEP) + (AEQ * AFS)) * ADB) * AFT;
                ADO = AFU;
                ADP = AFV;
            } else {
                ADO = A;
                ADP = GY;
            }
            let ADQ = QE * CZ;
            let ADR = DA * QE;
            let ADS = GL * FQ;
            let ADT = FR * GL;
            let ADU = (BZ + ADQ).sqrt();
            let ADV = BZ + ADU;
            let ADW = (ADQ - QE) / ADV;
            let ADX = (ADR - ((ADR * (GO / (GN * ADU))) * ADW)) / ADV;
            let ADY = (BZ + ADS).sqrt();
            let ADZ = BZ + ADY;
            let AEA = ADS / ADZ;
            let AEB = (ADT - ((ADT * (GO / (GN * ADY))) * AEA)) / ADZ;
            let AEC = CZ - BZ;
            let AEF = (BZ + (AEE * CZ)).sqrt();
            let AEG = BZ + AEF;
            let AEH = (AED * AEC) / AEG;
            let AEI = ((DA * AED) - (((DA * AEE) * (GO / (GN * AEF))) * AEH)) / AEG;
            let AHL;
            let AHM;
            let AHN;
            let AHO;
            if AEJ != 0.0 {
                let AGI = Lanes([0.0, CD[0], 0.0, CD[1]]);
                let AGL = EE * AGK;
                let AGN = (BZ + (AGM * (CC + (AGK * ED)))).sqrt();
                let AGO = BZ + AGN;
                let AGP = (AGJ * (CC - ED)) / AGO;
                let AGQ = (((AGI - Lanes([EE[0], 0.0, EE[1], 0.0])) * AGJ) - ((((AGI + Lanes([AGL[0], 0.0, AGL[1], 0.0])) * AGM) * (GO / (GN * AGN))) * AGP)) / AGO;
                let AGR = Lanes([0.0, DA[0], DA[1], DA[2], DA[3], DA[4]]);
                let AGT = EY * AGK;
                let AGU = (BZ + (AGM * (CZ + (AGK * EX)))).sqrt();
                let AGV = BZ + AGU;
                let AGW = (AGS * (CZ - EX)) / AGV;
                let AGX = (((AGR - Lanes([EY[0], 0.0, 0.0, EY[1], 0.0, EY[2]])) * AGS) - ((((AGR + Lanes([AGT[0], 0.0, 0.0, AGT[1], 0.0, AGT[2]])) * AGM) * (GO / (GN * AGU))) * AGW)) / AGV;
                AHL = AGW;
                AHM = AGP;
                AHN = AGX;
                AHO = AGQ;
            } else {
                let AHA = (BZ + (AGZ * CC)).sqrt();
                let AHB = BZ + AHA;
                let AHC = (AGY * (CC - BZ)) / AHB;
                let AHD = ((CD * AGY) - (((CD * AGZ) * (GO / (GN * AHA))) * AHC)) / AHB;
                let AHF = (BZ + (AGZ * CZ)).sqrt();
                let AHG = BZ + AHF;
                let AHH = (AHE * AEC) / AHG;
                let AHI = ((DA * AHE) - (((DA * AGZ) * (GO / (GN * AHF))) * AHH)) / AHG;
                let AHJ = Lanes([0.0, AHI[0], AHI[1], AHI[2], AHI[3], AHI[4]]);
                let AHK = Lanes([0.0, AHD[0], 0.0, AHD[1]]);
                AHL = AHH;
                AHM = AHC;
                AHN = AHJ;
                AHO = AHK;
            }
            let AHR = (BZ + (AHQ * ED)).sqrt();
            let AHS = BZ + AHR;
            let AHT = (AHP * (ED - BZ)) / AHS;
            let AHU = AHT + (AD * D);
            let AHV = (((EE * AHP) - (((EE * AHQ) * (GO / (GN * AHR))) * AHT)) / AHS) + (AE * D);
            let AIJ;
            let AIK;
            let AIL;
            let AIM;
            let AIN;
            let AIO;
            let AIP;
            let AIQ;
            let AIR;
            let AIS;
            if AHW != 0.0 {
                let AHY = AEH * AHX;
                let AHZ = AEI * AHX;
                let AIA = AHL * AHX;
                let AIB = AHN * AHX;
                let AIC = DT - BZ;
                let AIE = (BZ + (AEE * DT)).sqrt();
                let AIF = BZ + AIE;
                let AIG = (AID * AIC) / AIF;
                let AIH = ((DU * AID) - (((DU * AEE) * (GO / (GN * AIE))) * AIG)) / AIF;
                let AJJ;
                let AJK;
                if AEJ != 0.0 {
                    let AIU = Lanes([DU[0], DU[1], 0.0, DU[2], DU[3], DU[4], DU[5], DU[6], DU[7]]);
                    let AIW = EO * AGK;
                    let AIY = (BZ + (AIX * (DT + (AGK * EN)))).sqrt();
                    let AIZ = BZ + AIY;
                    let AJA = (AIV * (DT - EN)) / AIZ;
                    let AJB = (((AIU - Lanes([0.0, 0.0, EO[0], 0.0, 0.0, EO[1], 0.0, EO[2], EO[3]])) * AIV) - ((((AIU + Lanes([0.0, 0.0, AIW[0], 0.0, 0.0, AIW[1], 0.0, AIW[2], AIW[3]])) * AIX) * (GO / (GN * AIY))) * AJA)) / AIZ;
                    AJJ = AJA;
                    AJK = AJB;
                } else {
                    let AJE = (BZ + (AJD * DT)).sqrt();
                    let AJF = BZ + AJE;
                    let AJG = (AJC * AIC) / AJF;
                    let AJH = ((DU * AJC) - (((DU * AJD) * (GO / (GN * AJE))) * AJG)) / AJF;
                    let AJI = Lanes([AJH[0], AJH[1], 0.0, AJH[2], AJH[3], AJH[4], AJH[5], AJH[6], AJH[7]]);
                    AJJ = AJG;
                    AJK = AJI;
                }
                let AJR;
                let AJS;
                if AJL != 0.0 {
                    let AJM = BL - staged[118];
                    let AJN = AJM * AJM;
                    let AJO = BM * AJM;
                    let AJP = AJO + AJO;
                    let AJQ = if AJM < A { 1.0 } else { 0.0 };
                    let AKG;
                    let AKH;
                    if AJQ != 0.0 {
                        let AJZ = (AJN + AJY).sqrt();
                        let AKA = AJZ - AJM;
                        let AKB = 6.05e-3f64 / AKA;
                        let AKC = ((((AJP * (GO / (GN * AJZ))) - BM) * AKB) * BF) / AKA;
                        AKG = AKB;
                        AKH = AKC;
                    } else {
                        let AKD = (AJN + AJY).sqrt();
                        let AKE = IR * (AKD + AJM);
                        let AKF = ((AJP * (GO / (GN * AKD))) + BM) * IR;
                        AKG = AKE;
                        AKH = AKF;
                    }
                    let AKJ = (staged[120] + ((AIG + AJJ) * AKI)) + AKG;
                    let AKK = Lanes([AKH[0], AKH[1], 0.0, AKH[2], AKH[3], AKH[4], AKH[5], AKH[6], AKH[7]]);
                    let AKL = AKG / AKJ;
                    let AKM = (AKK - ((((Lanes([AIH[0], AIH[1], 0.0, AIH[2], AIH[3], AIH[4], AIH[5], AIH[6], AIH[7]]) + AJK) * AKI) + AKK) * AKL)) / AKJ;
                    AJR = AKL;
                    AJS = AKM;
                } else {
                    AJR = BZ;
                    AJS = AII;
                }
                let AJT = AJR * AIG;
                let AJU = AIH * AJR;
                let AJV = (AJS * AIG) + Lanes([AJU[0], AJU[1], 0.0, AJU[2], AJU[3], AJU[4], AJU[5], AJU[6], AJU[7]]);
                let AJW = AJR * AJJ;
                let AJX = (AJS * AJJ) + (AJK * AJR);
                AIJ = AHY;
                AIK = AJT;
                AIL = AJR;
                AIM = AIA;
                AIN = AJW;
                AIO = AHZ;
                AIP = AJV;
                AIQ = AJS;
                AIR = AIB;
                AIS = AJX;
            } else {
                AIJ = AEH;
                AIK = A;
                AIL = BZ;
                AIM = AHL;
                AIN = A;
                AIO = AEI;
                AIP = AII;
                AIQ = AII;
                AIR = AHN;
                AIS = AII;
            }
            let AKW;
            let AKX;
            if AIT != 0.0 {
                let AKN = AA + M;
                let AKO = Lanes([AB[0], AB[1], 0.0]) + Lanes([0.0, N[0], N[1]]);
                let AKR = (AKP * AKN) * AKQ;
                let AKS = AKR * AKN;
                let AKT = (((AKO * AKP) * AKQ) * AKN) + (AKO * AKR);
                let AKU = if (-1e0f64 * AKN) < A { 1.0 } else { 0.0 };
                let AMC;
                let AMD;
                if AKU != 0.0 {
                    let ALT = (AKS + ALS).sqrt();
                    let ALV = ALT - (ALU * AKN);
                    let ALW = 5e-13f64 / ALV;
                    let ALX = ((((AKT * (GO / (GN * ALT))) - (AKO * ALU)) * ALW) * BF) / ALV;
                    AMC = ALW;
                    AMD = ALX;
                } else {
                    let ALY = (AKS + ALS).sqrt();
                    let AMA = IR * (ALY + (ALZ * AKN));
                    let AMB = ((AKT * (GO / (GN * ALY))) + (AKO * ALZ)) * IR;
                    AMC = AMA;
                    AMD = AMB;
                }
                let AMF = if AMC < AME { 1.0 } else { 0.0 };
                let AMP;
                let AMQ;
                if AMF != 0.0 {
                    let AMH = AMC / AMG;
                    let AMJ = BZ - (AMH.powf(AMI));
                    let AMK = BZ / AMJ;
                    let AML = (((((AMD / AMG) * (AMI * (AMH.powf((AMI - GO))))) * BF) * AMK) * BF) / AMJ;
                    AMP = AMK;
                    AMQ = AML;
                } else {
                    let AMN = AMD * AMM;
                    let AMO = staged[123] + ((AMC - AME) * AMM);
                    AMP = AMO;
                    AMQ = AMN;
                }
                AKW = AMP;
                AKX = AMQ;
            } else {
                AKW = BZ;
                AKX = AKV;
            }
            let AKY = ADO * AKW;
            let AKZ = ADP * AKW;
            let ALA = Lanes([0.0, AKZ[0], AKZ[1]]) + (AKX * ADO);
            let ALB = AIJ * AKW;
            let ALC = AKX * AIJ;
            let ALD = (AIO * AKW) + Lanes([ALC[0], ALC[1], ALC[2], 0.0, 0.0]);
            let ALE = ZU * AKW;
            let ALF = AKX * ZU;
            let ALG = (ZV * AKW) + Lanes([ALF[0], ALF[1], ALF[2], 0.0, 0.0]);
            let ALH = AIK * AKW;
            let ALI = AKX * AIK;
            let ALJ = (AIP * AKW) + Lanes([0.0, 0.0, 0.0, ALI[0], ALI[1], ALI[2], 0.0, 0.0, 0.0]);
            let ALK = ON / QW;
            let ALL = QD / QY;
            let ALM = (BZ + (OM / QW)) + (QC / QY);
            let ALN = Lanes([ALK[0], ALK[1], 0.0, 0.0]) + Lanes([0.0, ALL[0], ALL[1], ALL[2]]);
            let ALO = ALM * ALM;
            let ALP = ALN * ALM;
            let ALQ = ALP + ALP;
            let ALR = if ALM < A { 1.0 } else { 0.0 };
            let AMZ;
            let ANA;
            if ALR != 0.0 {
                let AMS = (ALO + AMR).sqrt();
                let AMT = AMS - ALM;
                let AMU = 5.000000000000001e-3f64 / AMT;
                let AMV = ((((ALQ * (GO / (GN * AMS))) - ALN) * AMU) * BF) / AMT;
                AMZ = AMU;
                ANA = AMV;
            } else {
                let AMW = (ALO + AMR).sqrt();
                let AMX = IR * (AMW + ALM);
                let AMY = ((ALQ * (GO / (GN * AMW))) + ALN) * IR;
                AMZ = AMX;
                ANA = AMY;
            }
            let ANB = AMZ * SB;
            let ANC = staged[124] / ANB;
            let AND = ((((ANA * SB) + (SA * AMZ)) * ANC) * BF) / ANB;
            let ANF = if ANC < ANE { 1.0 } else { 0.0 };
            let ANH;
            let ANI;
            if ANF != 0.0 {
                ANH = ANE;
                ANI = ANG;
            } else {
                ANH = ANC;
                ANI = AND;
            }
            let ANJ = OL * ANH;
            let ANK = ANI * OL;
            let ANM = (DK * ANL) + AB;
            let ANN = ((ANL * (DJ - BZ)) + AA) / ANJ;
            let ANO = ANK * ANN;
            let ANP = (Lanes([0.0, ANM[0], ANM[1], 0.0, 0.0]) - Lanes([ANO[0], 0.0, ANO[1], ANO[2], ANO[3]])) / ANJ;
            let ANQ = if SM > A { 1.0 } else { 0.0 };
            let ANT;
            let ANU;
            if ANQ != 0.0 {
                let ANS = if ANR == BZ { 1.0 } else { 0.0 };
                let ANZ;
                let AOA;
                if ANS != 0.0 {
                    let ANX = if M < ANW { 1.0 } else { 0.0 };
                    let AOG;
                    let AOH;
                    if ANX != 0.0 {
                        let AOD = (-SM) / AOC;
                        let AOE = (SN * BF) / AOC;
                        let AOF = if AOD < BU { 1.0 } else { 0.0 };
                        let AON;
                        let AOO;
                        if AOF != 0.0 {
                            let AOI = AOD.exp();
                            let AOJ = AOE * AOI;
                            AON = AOI;
                            AOO = AOJ;
                        } else {
                            let AOK = BU.exp();
                            let AOL = AOK * (BZ + (AOD - BU));
                            let AOM = AOE * AOK;
                            AON = AOL;
                            AOO = AOM;
                        }
                        let AOP = ANW - M;
                        let AOQ = AOP * AON;
                        let AOR = (N * BF) * AON;
                        let AOS = Lanes([0.0, AOR[0], AOR[1], 0.0]) + (AOO * AOP);
                        let AOU = -AOT;
                        let AOW = AOU * (AOQ.powf(AOV));
                        let AOX = (AOS * (AOV * (AOQ.powf((AOV - GO))))) * AOU;
                        let AOY = if AOW < BU { 1.0 } else { 0.0 };
                        let APE;
                        let APF;
                        if AOY != 0.0 {
                            let AOZ = AOW.exp();
                            let APA = AOX * AOZ;
                            APE = AOZ;
                            APF = APA;
                        } else {
                            let APB = BU.exp();
                            let APC = APB * (BZ + (AOW - BU));
                            let APD = AOX * APB;
                            APE = APC;
                            APF = APD;
                        }
                        let APH = APG / AOT;
                        let API = APH * AOQ;
                        let APJ = API * APE;
                        let APK = ((AOS * APH) * APE) + (APF * API);
                        AOG = APJ;
                        AOH = APK;
                    } else {
                        AOG = A;
                        AOH = ANG;
                    }
                    ANZ = AOG;
                    AOA = AOH;
                } else {
                    let ANY = if ANR == GS { 1.0 } else { 0.0 };
                    let APN;
                    let APO;
                    if ANY != 0.0 {
                        let APL = if M < EZ { 1.0 } else { 0.0 };
                        let APZ;
                        let AQA;
                        if APL != 0.0 {
                            let APQ = (GS * parameters[46]) / (APP * APP);
                            let APR = EZ - M;
                            let APS = N * BF;
                            let APT = APR / HU;
                            let APU = Lanes([APS[0], APS[1], 0.0]);
                            let APV = ((GS * APT) / APQ).sqrt();
                            let APW = ((((APU - (IB * APT)) / HU) * GS) / APQ) * (GO / (GN * APV));
                            let APY = if APX == A { 1.0 } else { 0.0 };
                            let AQG;
                            let AQH;
                            if APY != 0.0 {
                                AQG = APP;
                                AQH = MA;
                            } else {
                                let AQB = BZ - (IR * HW);
                                let AQC = (ID * IR) * BF;
                                let AQD = APP * AQB;
                                let AQE = AQD * AQB;
                                let AQF = ((AQC * APP) * AQB) + (AQC * AQD);
                                AQG = AQE;
                                AQH = AQF;
                            }
                            let AQI = APW * APV;
                            let AQJ = AQH * AQG;
                            let AQK = ((APV * APV) + (AQG * AQG)).sqrt();
                            let AQL = (APV * AQG) / AQK;
                            let AQM = (((APW * AQG) + (AQH * APV)) - ((((AQI + AQI) + (AQJ + AQJ)) * (GO / (GN * AQK))) * AQL)) / AQK;
                            let AQN = APR / AQL;
                            let AQO = (APU - (AQM * AQN)) / AQL;
                            let AQP = IR * AQL;
                            let AQQ = AQM * IR;
                            let AQR = AQP * APQ;
                            let AQS = AQQ * APQ;
                            let AQT = AQN + (AQR * HU);
                            let AQU = AQO + ((AQS * HU) + (IB * AQR));
                            let ARO;
                            let ARP;
                            if APY != 0.0 {
                                let AQV = Lanes([0.0, AQU[0], AQU[1], AQU[2]]);
                                ARO = AQT;
                                ARP = AQV;
                            } else {
                                let AQX = GS * AQW;
                                let AQY = JN * (BZ + (AQX * (BZ + (GS * HW))));
                                let AQZ = SM / AQY;
                                let ARA = (((ID * GS) * AQX) * JN) * AQZ;
                                let ARB = ((BZ + AQW) / (BZ + AQX)) - AQZ;
                                let ARC = AQS * ARB;
                                let ARD = AQN - (AQR * ARB);
                                let ARE = Lanes([0.0, AQO[0], AQO[1], AQO[2]]) - (Lanes([0.0, ARC[0], ARC[1], ARC[2]]) + ((((SN - Lanes([0.0, ARA[0], ARA[1], ARA[2]])) / AQY) * BF) * AQR));
                                let ARF = ARD - AQT;
                                let ARG = Lanes([0.0, AQU[0], AQU[1], AQU[2]]);
                                let ARH = (ARE - ARG) * ARF;
                                let ARI = MR * AQN;
                                let ARJ = ARI * AQN;
                                let ARK = (((((AQO * MR) * AQN) + (AQO * ARI)) * HX) + (IE * ARJ)) / JN;
                                let ARL = ((ARF * ARF) + ((ARJ * HX) / JN)).sqrt();
                                let ARM = IR * ((ARD + AQT) + ARL);
                                let ARN = ((ARE + ARG) + (((ARH + ARH) + Lanes([0.0, ARK[0], ARK[1], ARK[2]])) * (GO / (GN * ARL)))) * IR;
                                ARO = ARM;
                                ARP = ARN;
                            }
                            let ARQ = (ARO - AQN) / ARO;
                            let ARR = ((ARP - Lanes([0.0, AQO[0], AQO[1], AQO[2]])) - (ARP * ARQ)) / ARO;
                            let ARS = if (ARQ.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let ASP;
                            let ASQ;
                            if ARS != 0.0 {
                                let ART = AQP / ARQ;
                                let ARU = (Lanes([0.0, AQQ[0], AQQ[1], AQQ[2]]) - (ARR * ART)) / ARQ;
                                let ARX = ARV / ARW;
                                let ARY = ARX * ARO;
                                let ARZ = ARY * ART;
                                let ASA = (-ARW) / ARO;
                                let ASB = ((ARP * ASA) * BF) / ARO;
                                let ASC = ASA.exp();
                                let ASD = AQG / ART;
                                let ASE = BZ + ASD;
                                let ASF = (ASA * ASE).exp();
                                let ASG = ASC - ASF;
                                let ASH = ARZ * ASG;
                                let ASI = ((((ARP * ARX) * ART) + (ARU * ARY)) * ASG) + (((ASB * ASC) - (((ASB * ASE) + (((Lanes([0.0, AQH[0], AQH[1], AQH[2]]) - (ARU * ASD)) / ART) * ASA)) * ASF)) * ARZ);
                                ASP = ASH;
                                ASQ = ASI;
                            } else {
                                let ASJ = ARV * AQG;
                                let ASK = (-ARW) / ARO;
                                let ASL = ASK.exp();
                                let ASM = ASJ * ASL;
                                let ASN = (AQH * ARV) * ASL;
                                let ASO = Lanes([0.0, ASN[0], ASN[1], ASN[2]]) + (((((ARP * ASK) * BF) / ARO) * ASL) * ASJ);
                                ASP = ASM;
                                ASQ = ASO;
                            }
                            APZ = ASP;
                            AQA = ASQ;
                        } else {
                            APZ = A;
                            AQA = ANG;
                        }
                        APN = APZ;
                        APO = AQA;
                    } else {
                        let APM = if ANR == OL { 1.0 } else { 0.0 };
                        let ASS;
                        let AST;
                        if APM != 0.0 {
                            let ASR = if M < ANW { 1.0 } else { 0.0 };
                            let ATH;
                            let ATI;
                            if ASR != 0.0 {
                                let ASU = ANW - M;
                                let ASV = N * BF;
                                let ASW = ASU.powf(AOV);
                                let ASY = ASX + SM;
                                let ASZ = SM / ASY;
                                let ATA = BZ - ASZ;
                                let ATC = ATA.powf(ATB);
                                let ATD = ASW * ATC;
                                let ATE = (ASV * (AOV * (ASU.powf((AOV - GO))))) * ATC;
                                let ATF = Lanes([0.0, ATE[0], ATE[1], 0.0]) + (((((SN - (SN * ASZ)) / ASY) * BF) * (ATB * (ATA.powf((ATB - GO))))) * ASW);
                                let ATG = if APX == A { 1.0 } else { 0.0 };
                                let ATP;
                                let ATQ;
                                if ATG != 0.0 {
                                    ATP = ATD;
                                    ATQ = ATF;
                                } else {
                                    let ATJ = (SM - parameters[52]) / ASX;
                                    let ATK = SN / ASX;
                                    let ATM = (ATJ - BZ) / ATL;
                                    let ATN = ATK / ATL;
                                    let ATO = if ATJ < BZ { 1.0 } else { 0.0 };
                                    let AUD;
                                    let AUE;
                                    if ATO != 0.0 {
                                        let ATV = ATM.exp();
                                        let ATW = BZ + ATV;
                                        let ATX = ((ATN * ATV) * (GO / ATW)) * ATL;
                                        let ATY = BZ + (ATL * (ATW.ln()));
                                        AUD = ATY;
                                        AUE = ATX;
                                    } else {
                                        let ATZ = (-ATM).exp();
                                        let AUA = BZ + ATZ;
                                        let AUB = ATJ + (ATL * (AUA.ln()));
                                        let AUC = ATK + ((((ATN * BF) * ATZ) * (GO / AUA)) * ATL);
                                        AUD = AUB;
                                        AUE = AUC;
                                    }
                                    let AUG = AUD.powf(AUF);
                                    let AUH = ATD * AUG;
                                    let AUI = (ATF * AUG) + ((AUE * (AUF * (AUD.powf((AUF - GO))))) * ATD);
                                    ATP = AUH;
                                    ATQ = AUI;
                                }
                                let ATR = -AOT;
                                let ATS = ATR * ATP;
                                let ATT = ATQ * ATR;
                                let ATU = if ATS < BU { 1.0 } else { 0.0 };
                                let AUO;
                                let AUP;
                                if ATU != 0.0 {
                                    let AUJ = ATS.exp();
                                    let AUK = ATT * AUJ;
                                    AUO = AUJ;
                                    AUP = AUK;
                                } else {
                                    let AUL = BU.exp();
                                    let AUM = AUL * (BZ + (ATS - BU));
                                    let AUN = ATT * AUL;
                                    AUO = AUM;
                                    AUP = AUN;
                                }
                                let AUQ = APG / AOT;
                                let AUR = AUQ * ASU;
                                let AUS = AUR * AUO;
                                let AUT = (ASV * AUQ) * AUO;
                                let AUU = Lanes([0.0, AUT[0], AUT[1], 0.0]) + (AUP * AUR);
                                ATH = AUS;
                                ATI = AUU;
                            } else {
                                ATH = A;
                                ATI = ANG;
                            }
                            ASS = ATH;
                            AST = ATI;
                        } else {
                            ASS = A;
                            AST = ANG;
                        }
                        APN = ASS;
                        APO = AST;
                    }
                    ANZ = APN;
                    AOA = APO;
                }
                let AOB = if ANZ > A { 1.0 } else { 0.0 };
                let AUW;
                let AUX;
                if AOB != 0.0 {
                    let AUV = if parameters[53] == BZ { 1.0 } else { 0.0 };
                    let AVJ;
                    let AVK;
                    if AUV != 0.0 {
                        let AUZ = AUY + ANJ;
                        let AVA = SM * AUZ;
                        let AVB = HF / AVA;
                        let AVD = AVC / AUZ;
                        let AVE = (AVB + ((SC / SH) * WB)) + AVD;
                        let AVF = ((((((SN * AUZ) + (ANK * SM)) * AVB) * BF) / AVA) + ((SD / SH) * WB)) + (((ANK * AVD) * BF) / AUZ);
                        let AVG = if ANR == OL { 1.0 } else { 0.0 };
                        let AVT;
                        let AVU;
                        if AVG != 0.0 {
                            let AVM = (ANZ - AVE) / AVL;
                            let AVN = (AOA - AVF) / AVL;
                            let AVO = if ANZ < AVE { 1.0 } else { 0.0 };
                            let AWD;
                            let AWE;
                            if AVO != 0.0 {
                                let AVV = AVM.exp();
                                let AVW = BZ + AVV;
                                let AVX = ANZ - (AVL * (AVW.ln()));
                                let AVY = AOA - (((AVN * AVV) * (GO / AVW)) * AVL);
                                AWD = AVX;
                                AWE = AVY;
                            } else {
                                let AVZ = (-AVM).exp();
                                let AWA = BZ + AVZ;
                                let AWB = AVE - (AVL * (AWA.ln()));
                                let AWC = AVF - ((((AVN * BF) * AVZ) * (GO / AWA)) * AVL);
                                AWD = AWB;
                                AWE = AWC;
                            }
                            let AWF = SM * AWD;
                            let AWG = (SN * AWD) + (AWE * SM);
                            AVT = AWF;
                            AVU = AWG;
                        } else {
                            let AVP = SM * ANZ;
                            let AVQ = ANZ + AVE;
                            let AVR = (AVP * AVE) / AVQ;
                            let AVS = (((((SN * ANZ) + (AOA * SM)) * AVE) + (AVF * AVP)) - ((AOA + AVF) * AVR)) / AVQ;
                            AVT = AVR;
                            AVU = AVS;
                        }
                        AVJ = AVT;
                        AVK = AVU;
                    } else {
                        let AVH = SM * ANZ;
                        let AVI = (SN * ANZ) + (AOA * SM);
                        AVJ = AVH;
                        AVK = AVI;
                    }
                    AUW = AVJ;
                    AUX = AVK;
                } else {
                    AUW = A;
                    AUX = ANG;
                }
                ANT = AUW;
                ANU = AUX;
            } else {
                ANT = A;
                ANU = ANG;
            }
            let ANV = if HV > A { 1.0 } else { 0.0 };
            let AWI = AWH * OM;
            let AWJ = ON * AWH;
            let AWK = (Y - IG) / IH;
            let AWL = Z / IH;
            let AWM = if Y < IG { 1.0 } else { 0.0 };
            let AWV;
            let AWW;
            if AWM != 0.0 {
                let AWN = AWK.exp();
                let AWO = BZ + AWN;
                let AWP = Y - (IH * (AWO.ln()));
                let AWQ = Z - (((AWL * AWN) * (GO / AWO)) * IH);
                AWV = AWP;
                AWW = AWQ;
            } else {
                let AWR = (-AWK).exp();
                let AWS = BZ + AWR;
                let AWT = IG - (IH * (AWS.ln()));
                let AWU = ((((AWL * BF) * AWR) * (GO / AWS)) * IH) * BF;
                AWV = AWT;
                AWW = AWU;
            }
            let AWX = BZ - (AWV * OD);
            let AWZ = AWY * ((OK * (BZ - (AWX.powf(OG)))) + (OL * (Y - AWV)));
            let AXA = ((((((AWW * OD) * BF) * (OG * (AWX.powf(OI)))) * BF) * OK) + ((Z - AWW) * OL)) * AWY;
            let AXC = AXB * QC;
            let AXD = QD * AXB;
            let AXF = AXE * QK;
            let AXG = AXF * AMZ;
            let AXH = (QL * AXE) * AMZ;
            let AXI = Lanes([AXH[0], AXH[1], 0.0, 0.0]) + (ANA * AXF);
            let AXJ = AXE * QT;
            let AXK = AXJ * AMZ;
            let AXL = (QU * AXE) * AMZ;
            let AXM = Lanes([0.0, AXL[0], AXL[1], AXL[2]]) + (ANA * AXJ);
            let AXO = (BD - OT) / AXN;
            let AXP = BE / AXN;
            let AXQ = if BD < OT { 1.0 } else { 0.0 };
            let AXZ;
            let AYA;
            if AXQ != 0.0 {
                let AXR = AXO.exp();
                let AXS = BZ + AXR;
                let AXT = BD - (AXN * (AXS.ln()));
                let AXU = BE - (((AXP * AXR) * (GO / AXS)) * AXN);
                AXZ = AXT;
                AYA = AXU;
            } else {
                let AXV = (-AXO).exp();
                let AXW = BZ + AXV;
                let AXX = OT - (AXN * (AXW.ln()));
                let AXY = ((((AXP * BF) * AXV) * (GO / AXW)) * AXN) * BF;
                AXZ = AXX;
                AYA = AXY;
            }
            let AYB = BZ - (AXZ / MQ);
            let AYE = ((AYC * ((PZ * ((PV * (BZ - (AYB.powf(PS)))) + (PW * (BD - AXZ)))) + (QA * BD))) * AYD) * AHX;
            let AYF = ((((((((((AYA / MQ) * BF) * (PS * (AYB.powf(PU)))) * BF) * PV) + ((BE - AYA) * PW)) * PZ) + (BE * QA)) * AYC) * AYD) * AHX;
            let AYG = (BL - OT) / AXN;
            let AYH = BM / AXN;
            let AYI = if BL < OT { 1.0 } else { 0.0 };
            let AYR;
            let AYS;
            if AYI != 0.0 {
                let AYJ = AYG.exp();
                let AYK = BZ + AYJ;
                let AYL = BL - (AXN * (AYK.ln()));
                let AYM = BM - (((AYH * AYJ) * (GO / AYK)) * AXN);
                AYR = AYL;
                AYS = AYM;
            } else {
                let AYN = (-AYG).exp();
                let AYO = BZ + AYN;
                let AYP = OT - (AXN * (AYO.ln()));
                let AYQ = ((((AYH * BF) * AYN) * (GO / AYO)) * AXN) * BF;
                AYR = AYP;
                AYS = AYQ;
            }
            let AYT = BZ - (AYR / MQ);
            let AYV = ((AYC * ((PZ * ((PV * (BZ - (AYT.powf(PS)))) + (PW * (BL - AYR)))) + (QA * BL))) * AYD) * AYU;
            let AYW = ((((((((((AYS / MQ) * BF) * (PS * (AYT.powf(PU)))) * BF) * PV) + ((BM - AYS) * PW)) * PZ) + (BM * QA)) * AYC) * AYD) * AYU;
            let AYZ = (AD - AYX) / AYY;
            let AZA = AE / AYY;
            let AZB = if AD < AYX { 1.0 } else { 0.0 };
            let AZK;
            let AZL;
            if AZB != 0.0 {
                let AZC = AYZ.exp();
                let AZD = BZ + AZC;
                let AZE = AD - (AYY * (AZD.ln()));
                let AZF = AE - (((AZA * AZC) * (GO / AZD)) * AYY);
                AZK = AZE;
                AZL = AZF;
            } else {
                let AZG = (-AYZ).exp();
                let AZH = BZ + AZG;
                let AZI = AYX - (AYY * (AZH.ln()));
                let AZJ = ((((AZA * BF) * AZG) * (GO / AZH)) * AYY) * BF;
                AZK = AZI;
                AZL = AZJ;
            }
            let AZN = BZ - (AZK / AZM);
            let AZR = AZQ * ((AZP * (BZ - (AZN.powf(AZO)))) + (GS * (AD - AZK)));
            let AZS = ((((((AZL / AZM) * BF) * (AZO * (AZN.powf(staged[173])))) * BF) * AZP) + ((AE - AZL) * GS)) * AZQ;
            let AZU = U / AZT;
            let AZV = V / AZT;
            let AZW = if AZU < BU { 1.0 } else { 0.0 };
            let BAC;
            let BAD;
            if AZW != 0.0 {
                let AZX = AZU.exp();
                let AZY = AZV * AZX;
                BAC = AZX;
                BAD = AZY;
            } else {
                let AZZ = BU.exp();
                let BAA = AZZ * (BZ + (AZU - BU));
                let BAB = AZV * AZZ;
                BAC = BAA;
                BAD = BAB;
            }
            let BAF = BAE * BAC;
            let BAG = BAD * BAE;
            let BAI = BAH * HW;
            let BAJ = (HY + GZ) + GS;
            let BAK = BAI * BAJ;
            let BAL = ((ID * BAH) * BAJ) + ((IF + Lanes([HA[0], HA[1], 0.0])) * BAI);
            let BAY;
            let BAZ;
            if BAM != 0.0 {
                let BAR = (BAP * ((BAN * ADW) + (BAO * AEA))) / BAQ;
                let BAS = (((ADX * BAN) + (AEB * BAO)) * BAP) / BAQ;
                BAY = BAR;
                BAZ = BAS;
            } else {
                let BAV = ((BD - BAT) / BAU) * BR;
                let BAW = (BE / BAU) * BR;
                let BAX = if BAV < BU { 1.0 } else { 0.0 };
                let BBG;
                let BBH;
                if BAX != 0.0 {
                    let BBB = BAV.exp();
                    let BBC = BAW * BBB;
                    BBG = BBB;
                    BBH = BBC;
                } else {
                    let BBD = BU.exp();
                    let BBE = BBD * (BZ + (BAV - BU));
                    let BBF = BAW * BBD;
                    BBG = BBE;
                    BBH = BBF;
                }
                let BBJ = (BZ + (GL * BBG)).sqrt();
                let BBK = BZ + BBJ;
                let BBL = (BBI * CZ) / BBK;
                let BBM = ((DA * BBI) - (((BBH * GL) * (GO / (GN * BBJ))) * BBL)) / BBK;
                BAY = BBL;
                BAZ = BBM;
            }
            let BBP;
            let BBQ;
            let BBR;
            let BBS;
            if BBA != 0.0 {
                let BBN = BAY * AHX;
                let BBO = BAZ * AHX;
                let BCJ;
                let BCK;
                if BAM != 0.0 {
                    let BBU = QE * DT;
                    let BBV = DU * QE;
                    let BBW = (BZ + BBU).sqrt();
                    let BBX = BZ + BBW;
                    let BBY = (BBU - QE) / BBX;
                    let BBZ = GL * FH;
                    let BCA = FI * GL;
                    let BCB = (BZ + BBZ).sqrt();
                    let BCC = BZ + BCB;
                    let BCD = BBZ / BCC;
                    let BCF = (BCE * ((BAN * BBY) + (BAO * BCD))) / BAQ;
                    let BCG = (((((BBV - ((BBV * (GO / (GN * BBW))) * BBY)) / BBX) * BAN) + (((BCA - ((BCA * (GO / (GN * BCB))) * BCD)) / BCC) * BAO)) * BCE) / BAQ;
                    BCJ = BCF;
                    BCK = BCG;
                } else {
                    let BCH = (BL - BAT) * BR;
                    let BCI = if BCH < BU { 1.0 } else { 0.0 };
                    let BCT;
                    let BCU;
                    if BCI != 0.0 {
                        let BCO = BCH.exp();
                        let BCP = DM * BCO;
                        BCT = BCO;
                        BCU = BCP;
                    } else {
                        let BCQ = BU.exp();
                        let BCR = BCQ * (BZ + (BCH - BU));
                        let BCS = DM * BCQ;
                        BCT = BCR;
                        BCU = BCS;
                    }
                    let BCW = (BZ + (GL * BCT)).sqrt();
                    let BCX = BZ + BCW;
                    let BCY = (BCV * DT) / BCX;
                    let BCZ = ((DU * BCV) - (((BCU * GL) * (GO / (GN * BCW))) * BCY)) / BCX;
                    BCJ = BCY;
                    BCK = BCZ;
                }
                let BCL = AIL * BCJ;
                let BCM = BCK * AIL;
                let BCN = (AIQ * BCJ) + Lanes([BCM[0], BCM[1], 0.0, BCM[2], BCM[3], BCM[4], BCM[5], BCM[6], BCM[7]]);
                BBP = BCL;
                BBQ = BBN;
                BBR = BCN;
                BBS = BBO;
            } else {
                BBP = A;
                BBQ = BAY;
                BBR = AII;
                BBS = BAZ;
            }
            let BDF;
            let BDG;
            let BDH;
            let BDI;
            let BDJ;
            let BDK;
            let BDL;
            let BDM;
            if BBT != 0.0 {
                let BDB = OF * (BDA * (OE.powf(staged[174])));
                let BDC = (OE.powf(BDA)) - OL;
                let BDD = if II < A { 1.0 } else { 0.0 };
                let BEG;
                let BEH;
                if BDD != 0.0 {
                    let BDX = II.exp();
                    let BDY = BZ + BDX;
                    let BDZ = BZ / BDY;
                    let BEA = (((IJ * BDX) * BDZ) * BF) / BDY;
                    BEG = BDZ;
                    BEH = BEA;
                } else {
                    let BEB = (-II).exp();
                    let BEC = (IJ * BF) * BEB;
                    let BED = BZ + BEB;
                    let BEE = BEB / BED;
                    let BEF = (BEC - (BEC * BEE)) / BED;
                    BEG = BEE;
                    BEH = BEF;
                }
                let BEI = ((BDB * BEG) + (BEH * BDC)) * AWH;
                let BEJ = (QF * BR) / CG;
                let BEK = IR / QH;
                let BEL = BEJ * BEK;
                let BEM = AXE * AMZ;
                let BEN = ((((QG * BR) / CG) * BEK) + ((((QI * BEK) * BF) / QH) * BEJ)) * BEM;
                let BEO = BAG / AZT;
                let BEP = IX * AA;
                let BEQ = ((AWH * ((BDC * BEG) + OL)) + (BEM * BEL)) + (BAF / AZT);
                let BER = BEP * BEQ;
                let BES = (AB * IX) * BEQ;
                let BET = ((Lanes([BEI[0], BEI[1], 0.0, 0.0]) + (((ANA * AXE) * BEL) + Lanes([BEN[0], BEN[1], 0.0, 0.0]))) + Lanes([BEO[0], BEO[1], 0.0, 0.0])) * BEP;
                let BEU = Lanes([0.0, BES[0], BES[1], 0.0, 0.0]) + Lanes([BET[0], 0.0, BET[1], BET[2], BET[3]]);
                let BEW = BEV * BAF;
                let BEX = BAG * BEV;
                let BEZ = BAG * BEY;
                let BFA = AXG + (BEY * BAF);
                let BFB = AXI + Lanes([BEZ[0], BEZ[1], 0.0, 0.0]);
                let BFD = (BFC * BFA) + AXK;
                let BFE = (BFB * BFC) + AXM;
                let BFG = BFF * BFA;
                let BFH = BFB * BFF;
                BDF = BFG;
                BDG = BEW;
                BDH = BFD;
                BDI = BER;
                BDJ = BFH;
                BDK = BEX;
                BDL = BFE;
                BDM = BEU;
            } else {
                BDF = AXG;
                BDG = BAF;
                BDH = AXK;
                BDI = A;
                BDJ = AXI;
                BDK = BAG;
                BDL = AXM;
                BDM = BDE;
            }
            let BDO = (L * HK) * BDN;
            let BDP = (HL * L) * BDN;
            let BDQ = (L * SM) * BDN;
            let BDR = (SN * L) * BDN;
            let BDS = (L * ((XQ + ZG) + AAI)) * BDN;
            let BDT = (((XR + ZH) + AAJ) * L) * BDN;
            let BDU = V * B;
            let BDV = (L * (((((VA + YS) + (B * U)) - AAT) + UJ) + TO)) * BDN;
            let BDW = ((((((VB + Lanes([YT[0], YT[1], 0.0, 0.0])) + Lanes([BDU[0], BDU[1], 0.0, 0.0])) - Lanes([AAU[0], AAU[1], 0.0, 0.0])) + Lanes([UK[0], UK[1], 0.0, 0.0])) + Lanes([TP[0], TP[1], 0.0, 0.0])) * L) * BDN;
            let BFM;
            let BFN;
            let BFO;
            let BFP;
            if E != 0.0 {
                let BFI = (L * (-AKY)) * BDN;
                let BFJ = ((ALA * BF) * L) * BDN;
                BFM = BFI;
                BFN = A;
                BFO = BFJ;
                BFP = AKV;
            } else {
                let BFK = (L * (-AKY)) * BDN;
                let BFL = ((ALA * BF) * L) * BDN;
                BFM = A;
                BFN = BFK;
                BFO = AKV;
                BFP = BFL;
            }
            let BFQ = (L * AIM) * BDN;
            let BFR = (AIR * L) * BDN;
            let BFS = (L * AHM) * BDN;
            let BFT = (AHO * L) * BDN;
            let BFU = (L * AIN) * BDN;
            let BFV = (AIS * L) * BDN;
            let BFW = (L * AHU) * BDN;
            let BFX = (AHV * L) * BDN;
            let BFY = (L * ANN) * BDN;
            let BFZ = (ANP * L) * BDN;
            let BGB = (L * (BGA * ANT)) * BDN;
            let BGC = ((ANU * BGA) * L) * BDN;
            let BGD = ((L * AJ) / AVC) * BDN;
            let BGE = ((AK * L) / AVC) * BDN;
            let BGF = ((L * AN) / AUY) * BDN;
            let BGG = ((AO * L) / AUY) * BDN;
            let BGH = L * ((AWI + BDF) + BDG);
            let BGI = ((Lanes([AWJ[0], AWJ[1], 0.0, 0.0]) + BDJ) + Lanes([BDK[0], BDK[1], 0.0, 0.0])) * L;
            let BGK = ddt(13137, BGH) * BDN;
            let BGL = (BGI * BGJ) * BDN;
            let BGM = BGH * BDN;
            let BGN = BGI * BDN;
            let BGO = L * AWZ;
            let BGP = AXA * L;
            let BGQ = ddt(13143, BGO) * BDN;
            let BGR = (BGP * BGJ) * BDN;
            let BGS = BGO * BDN;
            let BGT = BGP * BDN;
            let BGU = L * ((AXC + BDH) + BAK);
            let BGV = ((Lanes([0.0, AXD[0], AXD[1], AXD[2]]) + BDL) + Lanes([0.0, BAL[0], BAL[1], BAL[2]])) * L;
            let BGW = ddt(13153, BGU) * BDN;
            let BGX = (BGV * BGJ) * BDN;
            let BGY = BGU * BDN;
            let BGZ = BGV * BDN;
            let BHA = L * AZR;
            let BHB = AZS * L;
            let BHC = ddt(13159, BHA) * BDN;
            let BHD = (BHB * BGJ) * BDN;
            let BHE = BHA * BDN;
            let BHF = BHB * BDN;
            let BHG = L * BDI;
            let BHH = BDM * L;
            let BHI = ddt(13165, BHG) * BDN;
            let BHJ = (BHH * BGJ) * BDN;
            let BHK = BHG * BDN;
            let BHL = BHH * BDN;
            let BHN = BHM * AP;
            let BHO = AQ * BHM;
            let BHP = ddt(13173, BHN) * BDN;
            let BHQ = (BHO * BGJ) * BDN;
            let BHR = BHN * BDN;
            let BHS = BHO * BDN;
            let BHU = BHT * AS;
            let BHV = AT * BHT;
            let BHW = ddt(13181, BHU) * BDN;
            let BHX = (BHV * BGJ) * BDN;
            let BHY = BHU * BDN;
            let BHZ = BHV * BDN;
            let BIA = (L * ALH) * BDN;
            let BIB = (ALJ * L) * BDN;
            let BID = ((L * BJ) * BIC) * BDN;
            let BIE = ((BK * L) * BIC) * BDN;
            let BIF = L * (AYV + BBP);
            let BIG = (Lanes([AYW[0], AYW[1], 0.0, AYW[2], AYW[3], AYW[4], AYW[5], AYW[6], AYW[7]]) + BBR) * L;
            let BIH = ddt(13201, BIF) * BDN;
            let BII = (BIG * BGJ) * BDN;
            let BIJ = BIF * BDN;
            let BIK = BIG * BDN;
            let BIL = (L * ((ALE + (B * BD)) + ALB)) * BDN;
            let BIM = (((ALG + (BE * B)) + ALD) * L) * BDN;
            let BIN = L * (AYE + BBQ);
            let BIO = (AYF + BBS) * L;
            let BIP = ddt(13220, BIN) * BDN;
            let BIQ = (BIO * BGJ) * BDN;
            let BIR = BIN * BDN;
            let BIS = BIO * BDN;
            let BIX;
            let BIY;
            if F != 0.0 {
                let BIU = ((L * AZ) * BIT) * BDN;
                let BIV = ((BA * L) * BIT) * BDN;
                BIX = BIU;
                BIY = BIV;
            } else {
                BIX = A;
                BIY = BIW;
            }
            let BJD;
            let BJE;
            if G != 0.0 {
                let BJA = ((L * AW) * BIZ) * BDN;
                let BJB = ((AX * L) * BIZ) * BDN;
                BJD = BJA;
                BJE = BJB;
            } else {
                BJD = A;
                BJE = BJC;
            }
            let BJF = (SI + SF) / SC;
            let BJG = ((SK + SL) - (SD * BJF)) / SC;
            let BJL;
            let BJM;
            if BJH != 0.0 {
                let BJI = ANT / BJF;
                let BJJ = BJI.abs();
                let BJK = ((ANU - (BJG * BJI)) / BJF) * ((GN * (if BJI >= 0e0f64 { 1.0 } else { 0.0 })) - GO);
                BJL = BJJ;
                BJM = BJK;
            } else {
                BJL = A;
                BJM = ANG;
            }
            let BJN = if BJF > A { 1.0 } else { 0.0 };
            let BJU;
            let BJV;
            if BJN != 0.0 {
                let BJO = (BDF + BDH) / BJF;
                let BJP = ((BDJ + BDL) - (BJG * BJO)) / BJF;
                BJU = BJO;
                BJV = BJP;
            } else {
                let BJR = BJQ * AMZ;
                let BJS = BJR * SC;
                let BJT = ((ANA * BJQ) * SC) + (SD * BJR);
                BJU = BJS;
                BJV = BJT;
            }
            let BKA;
            let BKB;
            if BJW != 0.0 {
                let BJX = BFC * BJU;
                let BJY = BJV * BFC;
                BKA = BJX;
                BKB = BJY;
            } else {
                let BKG;
                let BKH;
                if BJZ != 0.0 {
                    let BKE = BKD * BJU;
                    let BKF = BJV * BKD;
                    BKG = BKE;
                    BKH = BKF;
                } else {
                    BKG = A;
                    BKH = ANG;
                }
                BKA = BKG;
                BKB = BKH;
            }
            let BKC = if (VA + XQ) < A { 1.0 } else { 0.0 };
            let BKI = if ((YS + ZG) + AAI) < A { 1.0 } else { 0.0 };
            let BKJ = if ALE < A { 1.0 } else { 0.0 };
            let BKK = if ALB < A { 1.0 } else { 0.0 };
            let BKL = if ALH < A { 1.0 } else { 0.0 };
            let BKN = ddt(13491, BKM);
            let BKP = BKA * BKN;
            let BKQ = BKB * BKN;
            let BKR = Lanes([BKQ[0], BKQ[1], BKQ[2], BKQ[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, ((BKO * BGJ) * BKA)]);
            let BKS = BKA * BKM;
            let BKT = BKB * BKM;
            let BKU = Lanes([BKT[0], BKT[1], BKT[2], BKT[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, (BKO * BKA)]);
            let BKV = BJL * BKM;
            let BKW = BJM * BKM;
            let BKX = Lanes([BKW[0], BKW[1], BKW[2], BKW[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, (BKO * BJL)]);
            let BKY = if (((((BFU + BGF) + BHP) + BHW) + BIA) + BIH) == A { 1.0 } else { 0.0 };
            let BKZ = BDP[0];
            let BLA = BDP[1];
            let BLB = BDP[2];
            let BLC = BDR[0];
            let BLD = BDR[1];
            let BLE = BDR[2];
            let BLF = BDR[3];
            let BLG = BDT[0];
            let BLH = BDT[1];
            let BLI = BDW[0];
            let BLJ = BDW[1];
            let BLK = BDW[2];
            let BLL = BDW[3];
            let BLM = BFO[0];
            let BLN = BFO[1];
            let BLO = BFO[2];
            let BLP = BFP[0];
            let BLQ = BFP[1];
            let BLR = BFP[2];
            let BLS = BFR[0];
            let BLT = BFR[1];
            let BLU = BFR[2];
            let BLV = BFR[3];
            let BLW = BFR[4];
            let BLX = BFR[5];
            let BLY = BFT[0];
            let BLZ = BFT[1];
            let BMA = BFT[2];
            let BMB = BFT[3];
            let BMC = BFV[0];
            let BMD = BFV[1];
            let BME = BFV[2];
            let BMF = BFV[3];
            let BMG = BFV[4];
            let BMH = BFV[5];
            let BMI = BFV[6];
            let BMJ = BFV[7];
            let BMK = BFV[8];
            let BML = BFX[0];
            let BMM = BFX[1];
            let BMN = BFZ[0];
            let BMO = BFZ[1];
            let BMP = BFZ[2];
            let BMQ = BFZ[3];
            let BMR = BFZ[4];
            let BMS = BGC[0];
            let BMT = BGC[1];
            let BMU = BGC[2];
            let BMV = BGC[3];
            let BMW = BGE[0];
            let BMX = BGE[1];
            let BMY = BGG[0];
            let BMZ = BGG[1];
            let BNA = BGL[0];
            let BNB = BGL[1];
            let BNC = BGL[2];
            let BND = BGL[3];
            let BNE = BGR[0];
            let BNF = BGR[1];
            let BNG = BGX[0];
            let BNH = BGX[1];
            let BNI = BGX[2];
            let BNJ = BGX[3];
            let BNK = BHD[0];
            let BNL = BHD[1];
            let BNM = BHJ[0];
            let BNN = BHJ[1];
            let BNO = BHJ[2];
            let BNP = BHJ[3];
            let BNQ = BHJ[4];
            let BNR = BHQ[0];
            let BNS = BHQ[1];
            let BNT = BHX[0];
            let BNU = BHX[1];
            let BNV = BIB[0];
            let BNW = BIB[1];
            let BNX = BIB[2];
            let BNY = BIB[3];
            let BNZ = BIB[4];
            let BOA = BIB[5];
            let BOB = BIB[6];
            let BOC = BIB[7];
            let BOD = BIB[8];
            let BOE = BIE[0];
            let BOF = BIE[1];
            let BOG = BIE[2];
            let BOH = BIE[3];
            let BOI = BIE[4];
            let BOJ = BIE[5];
            let BOK = BIE[6];
            let BOL = BIE[7];
            let BOM = BII[0];
            let BON = BII[1];
            let BOO = BII[2];
            let BOP = BII[3];
            let BOQ = BII[4];
            let BOR = BII[5];
            let BOS = BII[6];
            let BOT = BII[7];
            let BOU = BII[8];
            let BOV = BIM[0];
            let BOW = BIM[1];
            let BOX = BIM[2];
            let BOY = BIM[3];
            let BOZ = BIM[4];
            let BPA = BIQ[0];
            let BPB = BIQ[1];
            let BPC = BIQ[2];
            let BPD = BIQ[3];
            let BPE = BIQ[4];
            let BPF = BIY[0];
            let BPG = BIY[1];
            let BPH = BJE[0];
            let BPI = BJE[1];
            let BPJ = BKO;
            let BPK = BKR[0];
            let BPL = BKR[1];
            let BPM = BKR[2];
            let BPN = BKR[3];
            let BPO = BKR[4];
            let BPP = BKX[0];
            let BPQ = BKX[1];
            let BPR = BKX[2];
            let BPS = BKX[3];
            let BPT = BKX[4];
            let BPU = BGN[0];
            let BPV = BGN[1];
            let BPW = BGN[2];
            let BPX = BGN[3];
            let BPY = BGT[0];
            let BPZ = BGT[1];
            let BQA = BGZ[0];
            let BQB = BGZ[1];
            let BQC = BGZ[2];
            let BQD = BGZ[3];
            let BQE = BHF[0];
            let BQF = BHF[1];
            let BQG = BHL[0];
            let BQH = BHL[1];
            let BQI = BHL[2];
            let BQJ = BHL[3];
            let BQK = BHL[4];
            let BQL = BHS[0];
            let BQM = BHS[1];
            let BQN = BHZ[0];
            let BQO = BHZ[1];
            let BQP = BIK[0];
            let BQQ = BIK[1];
            let BQR = BIK[2];
            let BQS = BIK[3];
            let BQT = BIK[4];
            let BQU = BIK[5];
            let BQV = BIK[6];
            let BQW = BIK[7];
            let BQX = BIK[8];
            let BQY = BIS[0];
            let BQZ = BIS[1];
            let BRA = BIS[2];
            let BRB = BIS[3];
            let BRC = BIS[4];
            let BRD = BKU[0];
            let BRE = BKU[1];
            let BRF = BKU[2];
            let BRG = BKU[3];
            let BRH = BKU[4];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(8),
            multiplicity * (BDO),
            [6, 7, 8],
            [BKZ, BLA, BLB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * (BDQ),
            [4, 6, 7, 8],
            [BLC, BLD, BLE, BLF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (BDS),
            [4, 5],
            [BLG, BLH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(4),
            multiplicity * (BDV),
            [4, 6, 7, 8],
            [BLI, BLJ, BLK, BLL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(7),
            multiplicity * (BFM),
            [5, 6, 7],
            [BLM, BLN, BLO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(8),
            multiplicity * (BFN),
            [5, 6, 7],
            [BLP, BLQ, BLR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (BFQ),
            [3, 5, 6, 7, 8, 10],
            [BLS, BLT, BLU, BLV, BLW, BLX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * (BFS),
            [3, 6, 7, 8],
            [BLY, BLZ, BMA, BMB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(3),
            multiplicity * (BFU),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [BMC, BMD, BME, BMF, BMG, BMH, BMI, BMJ, BMK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (BFW),
            [3, 7],
            [BML, BMM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (BFY),
            [4, 5, 6, 7, 8],
            [BMN, BMO, BMP, BMQ, BMR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (BGB),
            [4, 6, 7, 8],
            [BMS, BMT, BMU, BMV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(4),
            multiplicity * (BGD),
            [2, 4],
            [BMW, BMX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (BGF),
            [1, 5],
            [BMY, BMZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(4),
            multiplicity * (BGK),
            [4, 6, 7, 8],
            [BNA, BNB, BNC, BND],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (BGQ),
            [4, 5],
            [BNE, BNF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (BGW),
            [4, 6, 7, 8],
            [BNG, BNH, BNI, BNJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (BHC),
            [3, 7],
            [BNK, BNL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (BHI),
            [4, 5, 6, 7, 8],
            [BNM, BNN, BNO, BNP, BNQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (BHP),
            [1, 2],
            [BNR, BNS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (BHW),
            [0, 1],
            [BNT, BNU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (BIA),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [BNV, BNW, BNX, BNY, BNZ, BOA, BOB, BOC, BOD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * (BID),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [BOE, BOF, BOG, BOH, BOI, BOJ, BOK, BOL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (BIH),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [BOM, BON, BOO, BOP, BOQ, BOR, BOS, BOT, BOU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * (BIL),
            [5, 6, 7, 8, 10],
            [BOV, BOW, BOX, BOY, BOZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * (BIP),
            [5, 6, 7, 8, 10],
            [BPA, BPB, BPC, BPD, BPE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(10),
            multiplicity * (BIX),
            [9, 10],
            [BPF, BPG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(10), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[220],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(10),
            Some(7),
            multiplicity * (BJD),
            [7, 10],
            [BPH, BPI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(7), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[221],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            None,
            multiplicity * (BRI),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (BKM),
            [11],
            [BPJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (BKP),
            [4, 6, 7, 8, 11],
            [BPK, BPL, BPM, BPN, BPO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (BKV),
            [4, 6, 7, 8, 11],
            [BPP, BPQ, BPR, BPS, BPT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(4),
            multiplicity * (BKM),
            [11],
            [BPJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (BRJ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (BRK),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(4),
            multiplicity * (BRL),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (BRM),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (BRN),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (BRO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (BRP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (BRQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BRR),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BRS),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BRT),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BRU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (BRV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (BRW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (staged[222]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[223]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(3),
            multiplicity * (BRX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(3),
            multiplicity * (BRY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(3),
            multiplicity * (BRZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (staged[224]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(10),
            multiplicity * (staged[225]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (staged[226]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (staged[227]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (staged[228]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (staged[229]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (staged[230]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(7),
            multiplicity * (staged[231]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = BDO;
        self.canonical_reactive[1] = BDQ;
        self.canonical_reactive[2] = BDS;
        self.canonical_reactive[3] = BDV;
        self.canonical_reactive[4] = BFM;
        self.canonical_reactive[5] = BFN;
        self.canonical_reactive[6] = BFQ;
        self.canonical_reactive[7] = BFS;
        self.canonical_reactive[8] = BFU;
        self.canonical_reactive[9] = BFW;
        self.canonical_reactive[10] = BFY;
        self.canonical_reactive[11] = BGB;
        self.canonical_reactive[12] = BGD;
        self.canonical_reactive[13] = BGF;
        self.canonical_reactive[14] = BGM;
        self.canonical_reactive[15] = BPU;
        self.canonical_reactive[16] = BPV;
        self.canonical_reactive[17] = BPW;
        self.canonical_reactive[18] = BPX;
        self.canonical_reactive[19] = BGS;
        self.canonical_reactive[20] = BPY;
        self.canonical_reactive[21] = BPZ;
        self.canonical_reactive[22] = BGY;
        self.canonical_reactive[23] = BQA;
        self.canonical_reactive[24] = BQB;
        self.canonical_reactive[25] = BQC;
        self.canonical_reactive[26] = BQD;
        self.canonical_reactive[27] = BHE;
        self.canonical_reactive[28] = BQE;
        self.canonical_reactive[29] = BQF;
        self.canonical_reactive[30] = BHK;
        self.canonical_reactive[31] = BQG;
        self.canonical_reactive[32] = BQH;
        self.canonical_reactive[33] = BQI;
        self.canonical_reactive[34] = BQJ;
        self.canonical_reactive[35] = BQK;
        self.canonical_reactive[36] = BHR;
        self.canonical_reactive[37] = BQL;
        self.canonical_reactive[38] = BQM;
        self.canonical_reactive[39] = BHY;
        self.canonical_reactive[40] = BQN;
        self.canonical_reactive[41] = BQO;
        self.canonical_reactive[42] = BIA;
        self.canonical_reactive[43] = BID;
        self.canonical_reactive[44] = BIJ;
        self.canonical_reactive[45] = BQP;
        self.canonical_reactive[46] = BQQ;
        self.canonical_reactive[47] = BQR;
        self.canonical_reactive[48] = BQS;
        self.canonical_reactive[49] = BQT;
        self.canonical_reactive[50] = BQU;
        self.canonical_reactive[51] = BQV;
        self.canonical_reactive[52] = BQW;
        self.canonical_reactive[53] = BQX;
        self.canonical_reactive[54] = BIL;
        self.canonical_reactive[55] = BIR;
        self.canonical_reactive[56] = BQY;
        self.canonical_reactive[57] = BQZ;
        self.canonical_reactive[58] = BRA;
        self.canonical_reactive[59] = BRB;
        self.canonical_reactive[60] = BRC;
        self.canonical_reactive[61] = BIX;
        self.canonical_reactive[62] = staged[220];
        self.canonical_reactive[63] = BJD;
        self.canonical_reactive[64] = staged[221];
        self.canonical_reactive[65] = BRI;
        self.canonical_reactive[66] = BKM;
        self.canonical_reactive[67] = BKS;
        self.canonical_reactive[68] = BRD;
        self.canonical_reactive[69] = BRE;
        self.canonical_reactive[70] = BRF;
        self.canonical_reactive[71] = BRG;
        self.canonical_reactive[72] = BRH;
        self.canonical_reactive[73] = BKV;
        self.canonical_reactive[74] = BKM;
        self.canonical_reactive[75] = BRJ;
        self.canonical_reactive[76] = BRK;
        self.canonical_reactive[77] = BRL;
        self.canonical_reactive[78] = BRM;
        self.canonical_reactive[79] = BRN;
        self.canonical_reactive[80] = BRO;
        self.canonical_reactive[81] = BRP;
        self.canonical_reactive[82] = BRQ;
        self.canonical_reactive[83] = BRR;
        self.canonical_reactive[84] = BRS;
        self.canonical_reactive[85] = BRT;
        self.canonical_reactive[86] = BRU;
        self.canonical_reactive[87] = BRV;
        self.canonical_reactive[88] = BRW;
        self.canonical_reactive[89] = staged[222];
        self.canonical_reactive[90] = staged[223];
        self.canonical_reactive[91] = BRX;
        self.canonical_reactive[92] = BRY;
        self.canonical_reactive[93] = BRZ;
        self.canonical_reactive[94] = staged[224];
        self.canonical_reactive[95] = staged[225];
        self.canonical_reactive[96] = staged[226];
        self.canonical_reactive[97] = staged[227];
        self.canonical_reactive[98] = staged[228];
        self.canonical_reactive[99] = staged[229];
        self.canonical_reactive[100] = staged[230];
        self.canonical_reactive[101] = staged[231];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[4, 6, 7, 8],
            &[cached[15], cached[16], cached[17], cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[4, 5],
            &[cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[4, 6, 7, 8],
            &[cached[23], cached[24], cached[25], cached[26]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(7),
            &[3, 7],
            &[cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[4, 5, 6, 7, 8],
            &[cached[31], cached[32], cached[33], cached[34], cached[35]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[40], cached[41]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(9),
            &[0, 1, 3, 5, 6, 7, 8, 9, 10],
            &[cached[45], cached[46], cached[47], cached[48], cached[49], cached[50], cached[51], cached[52], cached[53]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(10),
            &[5, 6, 7, 8, 10],
            &[cached[56], cached[57], cached[58], cached[59], cached[60]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[4, 6, 7, 8, 11],
            &[cached[68], cached[69], cached[70], cached[71], cached[72]],
            &[],
            &[],
            multiplicity,
        );
    }

}
