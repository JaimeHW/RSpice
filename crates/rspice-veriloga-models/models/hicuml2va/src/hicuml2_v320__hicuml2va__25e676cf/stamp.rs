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
        let mut key = Vec::with_capacity(298);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[84] = values[0];
        self.canonical_staged[1] = values[1];
        self.canonical_staged[0] = values[2];
        self.canonical_staged[61] = values[3];
        self.canonical_staged[28] = values[4];
        self.canonical_staged[20] = values[5];
        self.canonical_staged[3] = values[6];
        self.canonical_staged[9] = values[7];
        self.canonical_staged[41] = values[8];
        self.canonical_staged[4] = values[9];
        self.canonical_staged[10] = values[10];
        self.canonical_staged[39] = values[11];
        self.canonical_staged[44] = values[12];
        self.canonical_staged[85] = values[13];
        self.canonical_staged[80] = values[14];
        self.canonical_staged[81] = values[15];
        self.canonical_staged[86] = values[16];
        self.canonical_staged[73] = values[17];
        self.canonical_staged[19] = values[18];
        self.canonical_staged[87] = values[19];
        self.canonical_staged[88] = values[20];
        self.canonical_staged[89] = values[21];
        self.canonical_staged[90] = values[22];
        self.canonical_staged[91] = values[23];
        self.canonical_staged[93] = values[24];
        self.canonical_staged[33] = values[25];
        self.canonical_staged[2] = values[26];
        self.canonical_staged[96] = values[27];
        self.canonical_staged[95] = values[28];
        self.canonical_staged[5] = values[29];
        self.canonical_staged[6] = values[30];
        self.canonical_staged[7] = values[31];
        self.canonical_staged[8] = values[32];
        self.canonical_staged[98] = values[33];
        self.canonical_staged[97] = values[34];
        self.canonical_staged[11] = values[35];
        self.canonical_staged[12] = values[36];
        self.canonical_staged[99] = values[37];
        self.canonical_staged[13] = values[38];
        self.canonical_staged[14] = values[39];
        self.canonical_staged[15] = values[40];
        self.canonical_staged[16] = values[41];
        self.canonical_staged[100] = values[42];
        self.canonical_staged[17] = values[43];
        self.canonical_staged[101] = values[44];
        self.canonical_staged[18] = values[45];
        self.canonical_staged[30] = values[46];
        self.canonical_staged[24] = values[47];
        self.canonical_staged[103] = values[48];
        self.canonical_staged[102] = values[49];
        self.canonical_staged[25] = values[50];
        self.canonical_staged[27] = values[51];
        self.canonical_staged[26] = values[52];
        self.canonical_staged[36] = values[53];
        self.canonical_staged[105] = values[54];
        self.canonical_staged[104] = values[55];
        self.canonical_staged[37] = values[56];
        self.canonical_staged[38] = values[57];
        self.canonical_staged[106] = values[58];
        self.canonical_staged[40] = values[59];
        self.canonical_staged[107] = values[60];
        self.canonical_staged[42] = values[61];
        self.canonical_staged[43] = values[62];
        self.canonical_staged[111] = values[63];
        self.canonical_staged[109] = values[64];
        self.canonical_staged[110] = values[65];
        self.canonical_staged[45] = values[66];
        self.canonical_staged[46] = values[67];
        self.canonical_staged[108] = values[68];
        self.canonical_staged[112] = values[69];
        self.canonical_staged[47] = values[70];
        self.canonical_staged[113] = values[71];
        self.canonical_staged[48] = values[72];
        self.canonical_staged[117] = values[73];
        self.canonical_staged[115] = values[74];
        self.canonical_staged[116] = values[75];
        self.canonical_staged[114] = values[76];
        self.canonical_staged[50] = values[77];
        self.canonical_staged[166] = values[78];
        self.canonical_staged[165] = values[79];
        self.canonical_staged[51] = values[80];
        self.canonical_staged[168] = values[81];
        self.canonical_staged[167] = values[82];
        self.canonical_staged[52] = values[83];
        self.canonical_staged[170] = values[84];
        self.canonical_staged[169] = values[85];
        self.canonical_staged[53] = values[86];
        self.canonical_staged[172] = values[87];
        self.canonical_staged[171] = values[88];
        self.canonical_staged[173] = values[89];
        self.canonical_staged[54] = values[90];
        self.canonical_staged[174] = values[91];
        self.canonical_staged[55] = values[92];
        self.canonical_staged[56] = values[93];
        self.canonical_staged[177] = values[94];
        self.canonical_staged[175] = values[95];
        self.canonical_staged[176] = values[96];
        self.canonical_staged[178] = values[97];
        self.canonical_staged[57] = values[98];
        self.canonical_staged[179] = values[99];
        self.canonical_staged[58] = values[100];
        self.canonical_staged[182] = values[101];
        self.canonical_staged[180] = values[102];
        self.canonical_staged[181] = values[103];
        self.canonical_staged[164] = values[104];
        self.canonical_staged[183] = values[105];
        self.canonical_staged[184] = values[106];
        self.canonical_staged[185] = values[107];
        self.canonical_staged[59] = values[108];
        self.canonical_staged[60] = values[109];
        self.canonical_staged[62] = values[110];
        self.canonical_staged[72] = values[111];
        self.canonical_staged[63] = values[112];
        self.canonical_staged[64] = values[113];
        self.canonical_staged[65] = values[114];
        self.canonical_staged[66] = values[115];
        self.canonical_staged[67] = values[116];
        self.canonical_staged[68] = values[117];
        self.canonical_staged[69] = values[118];
        self.canonical_staged[70] = values[119];
        self.canonical_staged[71] = values[120];
        self.canonical_staged[74] = values[121];
        self.canonical_staged[186] = values[122];
        self.canonical_staged[187] = values[123];
        self.canonical_staged[188] = values[124];
        self.canonical_staged[189] = values[125];
        self.canonical_staged[190] = values[126];
        self.canonical_staged[191] = values[127];
        self.canonical_staged[192] = values[128];
        self.canonical_staged[193] = values[129];
        self.canonical_staged[194] = values[130];
        self.canonical_staged[195] = values[131];
        self.canonical_staged[196] = values[132];
        self.canonical_staged[198] = values[133];
        self.canonical_staged[197] = values[134];
        self.canonical_staged[199] = values[135];
        self.canonical_staged[200] = values[136];
        self.canonical_staged[201] = values[137];
        self.canonical_staged[75] = values[138];
        self.canonical_staged[76] = values[139];
        self.canonical_staged[77] = values[140];
        self.canonical_staged[78] = values[141];
        self.canonical_staged[79] = values[142];
        self.canonical_staged[202] = values[143];
        self.canonical_staged[203] = values[144];
        self.canonical_staged[204] = values[145];
        self.canonical_staged[205] = values[146];
        self.canonical_staged[206] = values[147];
        self.canonical_staged[207] = values[148];
        self.canonical_staged[208] = values[149];
        self.canonical_staged[209] = values[150];
        self.canonical_staged[210] = values[151];
        self.canonical_staged[82] = values[152];
        self.canonical_staged[83] = values[153];
        self.canonical_staged[218] = values[154];
        self.canonical_staged[220] = values[155];
        self.canonical_staged[211] = values[156];
        self.canonical_staged[212] = values[157];
        self.canonical_staged[213] = values[158];
        self.canonical_staged[214] = values[159];
        self.canonical_staged[215] = values[160];
        self.canonical_staged[216] = values[161];
        self.canonical_staged[217] = values[162];
        self.canonical_staged[219] = values[163];
        self.canonical_staged[221] = values[164];
        self.canonical_staged[222] = values[165];
        self.canonical_staged[223] = values[166];
        self.canonical_staged[224] = values[167];
        self.canonical_staged[225] = values[168];
        self.canonical_staged[226] = values[169];
        self.canonical_staged[227] = values[170];
        self.canonical_staged[228] = values[171];
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
                let A = parameters[0];
                let B = 3.1e2f64;
                let D = 1.3806226e-23f64;
                let E = 1.6021918e-19f64;
                let F = 1.380649e-23f64;
                let G = 1.602176634e-19f64;
                let L = 3e2f64;
                let O = 1e0f64;
                let Q = parameters[121];
                let U = parameters[117];
                let W = parameters[118];
                let X = parameters[119];
                let Y = 5e-1f64;
                let AD = parameters[120];
                let AH = parameters[130];
                let AJ = parameters[138];
                let AM = parameters[52];
                let AN = parameters[106];
                let AS = 0e0f64;
                let AY = parameters[104];
                let BB = parameters[22];
                let BG = 7e-1f64;
                let BL = parameters[86];
                let BN = parameters[88];
                let BO = parameters[87];
                let BR = parameters[115];
                let BS = 1e-2f64;
                let BT = parameters[116];
                let BY = 1.7e8f64;
                let BZ = 1e9f64;
                let CO = 6e0f64;
                let CY = 2e0f64;
                let CZ = parameters[40];
                let DB = parameters[42];
                let DH = parameters[48];
                let DJ = parameters[50];
                let DY = parameters[44];
                let EA = parameters[46];
                let EF = 1.0f64;
                let EG = parameters[53];
                let EI = parameters[55];
                let EL = parameters[57];
                let ER = parameters[63];
                let ET = parameters[58];
                let EV = 2.4e0f64;
                let EX = parameters[60];
                let FC = parameters[62];
                let FE = parameters[141];
                let FF = parameters[142];
                let FG = parameters[149];
                let FZ = 1.0f64;
                let GT = 1e2f64;
                let GW = parameters[49];
                let HP = parameters[89];
                let HS = 0e0f64;
                let HV = parameters[148];
                let HZ = parameters[90];
                let IB = 0e0f64;
                let ID = parameters[95];
                let IF = 0e0f64;
                let IH = parameters[96];
                let IJ = 0e0f64;
                let IL = parameters[102];
                let IO = 0e0f64;
                let IS = 0e0f64;
                let IU = 0e0f64;
                let IW = 0e0f64;
                let IY = 0e0f64;
                let JA = 0e0f64;
                let JC = 0e0f64;
                let JF = 0e0f64;
                let JG = 0e0f64;
                let JJ = 0e0f64;
                let JL = 0e0f64;
                let JO = 0e0f64;
                let JP = 0e0f64;
                let JV = 0e0f64;
                let JW = 0e0f64;
                let mut oBP = 0.0;
                let mut oBX = 0.0;
                let mut oCK = 0.0;
                let mut oDA = 0.0;
                let mut oDC = 0.0;
                let mut oDD = 0.0;
                let mut oDI = 0.0;
                let mut oDK = 0.0;
                let mut oDL = 0.0;
                let mut oDZ = 0.0;
                let mut oEB = 0.0;
                let mut oEC = 0.0;
                let mut oEH = 0.0;
                let mut oEJ = 0.0;
                let mut oEK = 0.0;
                let mut oEM = 0.0;
                let mut oEN = 0.0;
                let mut oEU = 0.0;
                let mut oEW = 0.0;
                let mut oEY = 0.0;
                let mut oEZ = 0.0;
                let mut oFA = 0.0;
                let mut oFB = 0.0;
                let mut oFD = 0.0;
                let mut oFK = 0.0;
                let mut oFL = 0.0;
                let mut oFM = 0.0;
                let mut oFN = 0.0;
                let mut oFO = 0.0;
                let mut oFQ = 0.0;
                let mut oFR = 0.0;
                let mut oFS = 0.0;
                let mut oFT = 0.0;
                let mut oFU = 0.0;
                let mut oFV = 0.0;
                let mut oFW = 0.0;
                let mut oFX = 0.0;
                let mut oFY = 0.0;
                let mut oGA = 0.0;
                let mut oGB = 0.0;
                let mut oGC = 0.0;
                let mut oGD = 0.0;
                let mut oGE = 0.0;
                let mut oGF = 0.0;
                let mut oGG = 0.0;
                let mut oGH = 0.0;
                let mut oGI = 0.0;
                let mut oGJ = 0.0;
                let mut oGK = 0.0;
                let mut oGL = 0.0;
                let mut oGM = 0.0;
                let mut oGN = 0.0;
                let mut oGO = 0.0;
                let mut oGP = 0.0;
                let mut oGQ = 0.0;
                let mut oGR = 0.0;
                let mut oHH = 0.0;
                let mut oHJ = 0.0;
                let mut oHM = 0.0;
                let mut oHO = 0.0;
                let mut oHR = 0.0;
                let mut oHW = 0.0;
                let mut oHX = 0.0;
                let mut oIN = 0.0;
                let mut oIR = 0.0;
                let mut oJU = 0.0;
                let C = if A <= B { 1.0 } else { 0.0 };
                let H;
                let I;
                if C != 0.0 {
                    H = D;
                    I = E;
                } else {
                    H = F;
                    I = G;
                }
                let J = parameters[146] + 2.7315e2f64;
                let K = H / I;
                let M = K * L;
                let N = K * J;
                let P = O / N;
                let R = (Q * J) * (J.ln());
                let S = parameters[122] * J;
                let T = parameters[131] * J;
                let V = (U + R) + S;
                let Z = (V + ((W + R) + S)) * Y;
                let AA = (V + ((X + R) + S)) * Y;
                let AB = (U + W) * Y;
                let AC = (U + X) * Y;
                let AE = (AD + X) * Y;
                let AF = 3e0f64 - (Q / K);
                let AG = AF + O;
                let AI = AG - AH;
                let AK = AG - AJ;
                let AL = AF - 1.5e0f64;
                let AO = (O - parameters[107]) * (AM + AN);
                let AP = if AO >= AN { 1.0 } else { 0.0 };
                let AU;
                let AV;
                let AW;
                let AX;
                if AP != 0.0 {
                    let AQ = AO - AN;
                    let AR = AM - AQ;
                    AU = AQ;
                    AV = AR;
                    AW = AS;
                    AX = AN;
                } else {
                    let AT = AN - AO;
                    AU = AS;
                    AV = AM;
                    AW = AT;
                    AX = AO;
                }
                let AZ = parameters[105] * AY;
                let BA = AY - AZ;
                let BC = if BB != AS { 1.0 } else { 0.0 };
                let BE = if BC != 0.0 {
                    let BD = O / BB;
                    BD
                } else {
                    AS
                };
                let BF = if A <= L { 1.0 } else { 0.0 };
                let BH = if BF != 0.0 {
                    AS
                } else {
                    BG
                };
                let BI = if parameters[47] > AS { 1.0 } else { 0.0 };
                let BJ = if (if parameters[32] > AS { 1.0 } else { 0.0 }) != 0.0 && BI != 0.0 { 1.0 } else { 0.0 };
                let BK = if BJ != 0.0 {
                    O
                } else {
                    AS
                };
                let BM = if BL != AS { 1.0 } else { 0.0 };
                let BQ;
                if BM != 0.0 {
                    let BP = if (if (if BN == AS { 1.0 } else { 0.0 }) != 0.0 && (if BO == AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if parameters[66] == AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBP = BP;
                    let BV = if BP != 0.0 {
                        AS
                    } else {
                        BL
                    };
                    BQ = BV;
                } else {
                    BQ = BL;
                }
                let BU = if (if BR >= BS { 1.0 } else { 0.0 }) != 0.0 || (if BT >= BS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CA;
                let CB;
                let CC;
                let CD;
                let CE;
                let CF;
                let CG;
                let CH;
                if BU != 0.0 {
                    let BW = Y * (BR - BT);
                    let BX = if BT < BR { 1.0 } else { 0.0 };
                    oBX = BX;
                    let CI;
                    let CJ;
                    if BX != 0.0 {
                        CI = BT;
                        CJ = BR;
                    } else {
                        CI = BR;
                        CJ = BT;
                    }
                    let CK = if CI < BS { 1.0 } else { 0.0 };
                    oCK = CK;
                    let CS;
                    let CT;
                    let CU;
                    let CV;
                    let CW;
                    if CK != 0.0 {
                        let CL = (O + CJ).ln();
                        CS = CL;
                        CT = BY;
                        CU = BZ;
                        CV = BY;
                        CW = BZ;
                    } else {
                        let CM = O / BR;
                        let CN = O / BT;
                        let CP = BR / CO;
                        let CQ = BT / CO;
                        let CR = ((O + BR) / (O + BT)).ln();
                        CS = CR;
                        CT = CP;
                        CU = CN;
                        CV = CQ;
                        CW = CM;
                    }
                    CA = BW;
                    CB = CS;
                    CC = CI;
                    CD = CJ;
                    CE = CT;
                    CF = CU;
                    CG = CV;
                    CH = CW;
                } else {
                    CA = AS;
                    CB = AS;
                    CC = BT;
                    CD = BR;
                    CE = BY;
                    CF = BZ;
                    CG = BY;
                    CH = BZ;
                }
                let CX = if parameters[39] > AS { 1.0 } else { 0.0 };
                if CX != 0.0 {
                    let DA = (CY * N) * (((((CZ * Y) * P).exp()) - (((-5e-1f64 * CZ) * P).exp())).ln());
                    oDA = DA;
                    let DC = DB.abs();
                    oDC = DC;
                    let DD = if DB > AS { 1.0 } else { 0.0 };
                    oDD = DD;
                } else {
                }
                let DE = W * P;
                let DF = AF / parameters[17];
                let DG = AB * P;
                if BI != 0.0 {
                    let DI = (CY * N) * (((((DH * Y) * P).exp()) - (((-5e-1f64 * DH) * P).exp())).ln());
                    oDI = DI;
                    let DK = DJ.abs();
                    oDK = DK;
                    let DL = if DJ > AS { 1.0 } else { 0.0 };
                    oDL = DL;
                } else {
                }
                let DM = X * P;
                let DN = U * P;
                let DO = if BF != 0.0 && (if ((parameters[8] - O).abs()) < 1e-5f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DP = parameters[125] * P;
                let DQ = (U - W) * P;
                let DR = (U - X) * P;
                let DS = AH - T;
                let DT = if parameters[79] > AS { 1.0 } else { 0.0 };
                let DU = AH - O;
                let DV = if BK == O { 1.0 } else { 0.0 };
                let DW = if parameters[37] > AS { 1.0 } else { 0.0 };
                let DX = if parameters[43] > AS { 1.0 } else { 0.0 };
                if DX != 0.0 {
                    let DZ = (CY * N) * (((((DY * Y) * P).exp()) - (((-5e-1f64 * DY) * P).exp())).ln());
                    oDZ = DZ;
                    let EB = EA.abs();
                    oEB = EB;
                    let EC = if EA > AS { 1.0 } else { 0.0 };
                    oEC = EC;
                } else {
                }
                let ED = AF / parameters[21];
                let EE = if parameters[27] > AS { 1.0 } else { 0.0 };
                if EF != 0.0 {
                    let EH = (CY * N) * (((((EG * Y) * P).exp()) - (((-5e-1f64 * EG) * P).exp())).ln());
                    oEH = EH;
                    let EJ = EI.abs();
                    oEJ = EJ;
                    let EK = if EI > AS { 1.0 } else { 0.0 };
                    oEK = EK;
                } else {
                }
                let EO;
                if BF != 0.0 {
                    let EM = if EL > AS { 1.0 } else { 0.0 };
                    oEM = EM;
                    if EM != 0.0 {
                        let EU = (CY * N) * (((((ET * Y) * P).exp()) - (((-5e-1f64 * ET) * P).exp())).ln());
                        oEU = EU;
                    } else {
                    }
                    EO = EV;
                } else {
                    let EN = if EL > AS { 1.0 } else { 0.0 };
                    oEN = EN;
                    if EN != 0.0 {
                        let EW = (CY * N) * (((((ET * Y) * P).exp()) - (((-5e-1f64 * ET) * P).exp())).ln());
                        oEW = EW;
                        let EY = -EX;
                        oEY = EY;
                        let EZ = EY.abs();
                        oEZ = EZ;
                        let FA = if EY > AS { 1.0 } else { 0.0 };
                        oFA = FA;
                    } else {
                        let FB = -EX;
                        oFB = FB;
                    }
                    EO = EX;
                }
                let EP = AD * P;
                let EQ = AJ - O;
                let ES = if ER > AS { 1.0 } else { 0.0 };
                if ES != 0.0 {
                    let FD = if FC > AS { 1.0 } else { 0.0 };
                    oFD = FD;
                    if FD != 0.0 {
                        let FK = (CY * N) * (((((ER * Y) * P).exp()) - (((-5e-1f64 * ER) * P).exp())).ln());
                        oFK = FK;
                        let FL = -EO;
                        oFL = FL;
                        let FM = FL.abs();
                        oFM = FM;
                        let FN = if FL > AS { 1.0 } else { 0.0 };
                        oFN = FN;
                    } else {
                        let FO = -EO;
                        oFO = FO;
                    }
                } else {
                }
                let FH = if FF >= FG { 1.0 } else { 0.0 };
                let FI = if FF > AS { 1.0 } else { 0.0 };
                let FJ = if (if (if FE != AS { 1.0 } else { 0.0 }) != 0.0 && FH != 0.0 { 1.0 } else { 0.0 }) != 0.0 && FI != 0.0 { 1.0 } else { 0.0 };
                if FJ != 0.0 {
                    if CX != 0.0 {
                        let FQ = (CY * N) * (((((CZ * Y) * P).exp()) - (((-5e-1f64 * CZ) * P).exp())).ln());
                        oFQ = FQ;
                        let FR = DB.abs();
                        oFR = FR;
                        let FS = if DB > AS { 1.0 } else { 0.0 };
                        oFS = FS;
                    } else {
                    }
                    if BI != 0.0 {
                        let FT = (CY * N) * (((((DH * Y) * P).exp()) - (((-5e-1f64 * DH) * P).exp())).ln());
                        oFT = FT;
                        let FU = DJ.abs();
                        oFU = FU;
                        let FV = if DJ > AS { 1.0 } else { 0.0 };
                        oFV = FV;
                    } else {
                    }
                    if DX != 0.0 {
                        let FW = (CY * N) * (((((DY * Y) * P).exp()) - (((-5e-1f64 * DY) * P).exp())).ln());
                        oFW = FW;
                        let FX = EA.abs();
                        oFX = FX;
                        let FY = if EA > AS { 1.0 } else { 0.0 };
                        oFY = FY;
                    } else {
                    }
                    if FZ != 0.0 {
                        let GA = (CY * N) * (((((EG * Y) * P).exp()) - (((-5e-1f64 * EG) * P).exp())).ln());
                        oGA = GA;
                        let GB = EI.abs();
                        oGB = GB;
                        let GC = if EI > AS { 1.0 } else { 0.0 };
                        oGC = GC;
                    } else {
                    }
                    let GF;
                    if BF != 0.0 {
                        let GD = if EL > AS { 1.0 } else { 0.0 };
                        oGD = GD;
                        if GD != 0.0 {
                            let GG = (CY * N) * (((((ET * Y) * P).exp()) - (((-5e-1f64 * ET) * P).exp())).ln());
                            oGG = GG;
                        } else {
                        }
                        GF = EV;
                    } else {
                        let GE = if EL > AS { 1.0 } else { 0.0 };
                        oGE = GE;
                        if GE != 0.0 {
                            let GH = (CY * N) * (((((ET * Y) * P).exp()) - (((-5e-1f64 * ET) * P).exp())).ln());
                            oGH = GH;
                            let GI = -EX;
                            oGI = GI;
                            let GJ = GI.abs();
                            oGJ = GJ;
                            let GK = if GI > AS { 1.0 } else { 0.0 };
                            oGK = GK;
                        } else {
                            let GL = -EX;
                            oGL = GL;
                        }
                        GF = EX;
                    }
                    oGF = GF;
                    if ES != 0.0 {
                        let GM = if FC > AS { 1.0 } else { 0.0 };
                        oGM = GM;
                        if GM != 0.0 {
                            let GN = (CY * N) * (((((ER * Y) * P).exp()) - (((-5e-1f64 * ER) * P).exp())).ln());
                            oGN = GN;
                            let GO = -GF;
                            oGO = GO;
                            let GP = GO.abs();
                            oGP = GP;
                            let GQ = if GO > AS { 1.0 } else { 0.0 };
                            oGQ = GQ;
                        } else {
                            let GR = -GF;
                            oGR = GR;
                        }
                    } else {
                    }
                } else {
                }
                let FP = if parameters[14] > AS { 1.0 } else { 0.0 };
                let GS = if parameters[16] > AS { 1.0 } else { 0.0 };
                let GU = if parameters[51] < GT { 1.0 } else { 0.0 };
                let GV = if parameters[10] > AS { 1.0 } else { 0.0 };
                let GX = O - ((-8.754687373538999e-1f64 / GW).exp());
                let GY = -GW;
                let GZ = if parameters[85] > AS { 1.0 } else { 0.0 };
                let HA = if A >= B { 1.0 } else { 0.0 };
                let HB = if A >= 3.2e2f64 { 1.0 } else { 0.0 };
                let HC = if parameters[18] > AS { 1.0 } else { 0.0 };
                let HD = if parameters[20] > AS { 1.0 } else { 0.0 };
                let HE = if parameters[56] < GT { 1.0 } else { 0.0 };
                let HF = if parameters[25] > AS { 1.0 } else { 0.0 };
                let HG = if parameters[61] < GT { 1.0 } else { 0.0 };
                if ES != 0.0 {
                    let HH = if parameters[65] < GT { 1.0 } else { 0.0 };
                    oHH = HH;
                } else {
                }
                let HI = if parameters[97] > AS { 1.0 } else { 0.0 };
                if HI != 0.0 {
                    let HJ = if parameters[101] > AS { 1.0 } else { 0.0 };
                    oHJ = HJ;
                } else {
                }
                let HK = if parameters[99] > AS { 1.0 } else { 0.0 };
                let HL = if FH != 0.0 && FI != 0.0 { 1.0 } else { 0.0 };
                if HL != 0.0 {
                    let HM = if FE == O { 1.0 } else { 0.0 };
                    oHM = HM;
                    if HM != 0.0 {
                    } else {
                        let HO = if FE == CY { 1.0 } else { 0.0 };
                        oHO = HO;
                    }
                } else {
                }
                let HN = if BQ != AS { 1.0 } else { 0.0 };
                let HQ = if (if HP >= FG { 1.0 } else { 0.0 }) != 0.0 && (if HP > AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let HT;
                if HQ != 0.0 {
                    let HR = if parameters[93] > AS { 1.0 } else { 0.0 };
                    oHR = HR;
                    HT = AS;
                } else {
                    HT = HS;
                }
                let HU = if parameters[29] == O { 1.0 } else { 0.0 };
                if HU != 0.0 {
                    let HW = -HV;
                    oHW = HW;
                } else {
                    let HX = -HV;
                    oHX = HX;
                }
                let HY = -HV;
                let IA = if (if HZ >= FG { 1.0 } else { 0.0 }) != 0.0 && (if HZ > AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let IC = if IA != 0.0 {
                    AS
                } else {
                    IB
                };
                let IE = if (if ID >= FG { 1.0 } else { 0.0 }) != 0.0 && (if ID > AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let IG = if IE != 0.0 {
                    AS
                } else {
                    IF
                };
                let II = if (if IH >= FG { 1.0 } else { 0.0 }) != 0.0 && (if IH > AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let IK = if II != 0.0 {
                    AS
                } else {
                    IJ
                };
                let IM = if (if IL >= FG { 1.0 } else { 0.0 }) != 0.0 && (if IL > AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let IP;
                if IM != 0.0 {
                    let IN = if parameters[103] > AS { 1.0 } else { 0.0 };
                    oIN = IN;
                    IP = AS;
                } else {
                    IP = IO;
                }
                let IQ = if (if (if FE >= O { 1.0 } else { 0.0 }) != 0.0 && FH != 0.0 { 1.0 } else { 0.0 }) != 0.0 && FI != 0.0 { 1.0 } else { 0.0 };
                let IT;
                if IQ != 0.0 {
                    let IR = if parameters[145] > AS { 1.0 } else { 0.0 };
                    oIR = IR;
                    IT = AS;
                } else {
                    IT = IS;
                }
                let IV = if IA != 0.0 {
                    IU
                } else {
                    AS
                };
                let IX = if HQ != 0.0 {
                    IW
                } else {
                    AS
                };
                let IZ = if II != 0.0 {
                    IY
                } else {
                    AS
                };
                let JB = if IE != 0.0 {
                    JA
                } else {
                    AS
                };
                let JD = if IM != 0.0 {
                    JC
                } else {
                    AS
                };
                let JE = if parameters[112] == -1e0f64 { 1.0 } else { 0.0 };
                let JH;
                let JI;
                if JE != 0.0 {
                    JH = JF;
                    JI = AS;
                } else {
                    JH = AS;
                    JI = JG;
                }
                let JK = if IE != 0.0 {
                    JJ
                } else {
                    AS
                };
                let JM = if HB != 0.0 {
                    JL
                } else {
                    AS
                };
                let JN = if (if parameters[109] == O { 1.0 } else { 0.0 }) != 0.0 && (if (if BN > AS { 1.0 } else { 0.0 }) != 0.0 && (if BO > AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let JQ;
                let JR;
                let JS;
                let JT;
                if JN != 0.0 {
                    let JU = (CY * BO) - (BN * BN);
                    oJU = JU;
                    JQ = JV;
                    JR = JW;
                    JS = AS;
                    JT = AS;
                } else {
                    JQ = AS;
                    JR = AS;
                    JS = JO;
                    JT = JP;
                }
            [C, J, K, M, Z, AA, AB, AC, AE, AF, AI, AK, AL, AP, AZ, BA, BC, BF, BI, BJ, BM, oBP, BU, oBX, oCK, CX, oDA, oDC, oDD, DE, DF, DG, oDI, oDK, oDL, DM, DN, DO, DP, DQ, DR, DS, DT, DU, DV, DW, DX, oDZ, oEB, oEC, ED, EE, BH, oEH, oEJ, oEK, AU, AV, oEM, oEU, oEN, oEW, oEY, oEZ, oFA, oFB, EP, EQ, ES, oFD, oFK, EO, oFL, oFM, oFN, oFO, FJ, oFQ, oFR, oFS, oFT, oFU, oFV, oFW, oFX, oFY, oGA, oGB, oGC, oGD, oGG, oGE, oGH, oGI, oGJ, oGK, oGL, oGM, oGN, oGF, oGO, oGP, oGQ, oGR, FP, GS, GU, GV, GX, GY, GZ, HA, HB, CA, CB, CC, CD, CE, CF, CG, CH, BE, HC, HD, HE, HF, HG, oHH, HI, oHJ, HK, HL, oHM, oHO, HN, HQ, oHR, HU, oHW, oHX, HY, AW, AX, IA, IE, II, IM, oIN, IQ, oIR, JE, JN, oJU, IV, IZ, JD, HT, IC, IG, IK, IP, IT, IX, JB, JH, JI, JK, JM, JQ, JR, JS, JT]
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
        let produced: [f64; 57] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = staged[73];
                let D = 7.314999999999998e1f64;
                let H = 1e0f64;
                let J = staged[1];
                let R = 5e-1f64;
                let U = staged[33];
                let V = 6e2f64;
                let X = staged[3];
                let Y = staged[4];
                let AA = 2e0f64;
                let AB = 4e0f64;
                let AD = parameters[40];
                let AE = parameters[41];
                let AF = parameters[39];
                let AH = staged[95];
                let AI = parameters[42];
                let AR = staged[19];
                let AT = staged[96];
                let AV = staged[9];
                let AY = parameters[48];
                let AZ = parameters[47];
                let BB = staged[97];
                let BC = parameters[50];
                let BH = staged[98];
                let BJ = 2.4e0f64;
                let BR = staged[99];
                let BS = parameters[125];
                let BT = parameters[127];
                let CC = staged[100];
                let CD = parameters[79];
                let CF = parameters[78];
                let CL = staged[101];
                let CM = parameters[32];
                let CO = parameters[33];
                let CT = staged[30];
                let CW = parameters[44];
                let CX = parameters[43];
                let CZ = staged[102];
                let DA = parameters[46];
                let DH = staged[103];
                let DK = 1.0f64;
                let DN = parameters[53];
                let DP = staged[104];
                let DQ = parameters[55];
                let DV = staged[105];
                let EB = staged[106];
                let EC = staged[107];
                let EK = staged[108];
                let EL = staged[41];
                let EO = parameters[58];
                let EP = parameters[59];
                let EQ = parameters[57];
                let ES = 0.0f64;
                let ET = -2.4e0f64;
                let EY = 2.4e0f64;
                let FD = staged[109];
                let FE = staged[110];
                let FJ = staged[111];
                let FL = staged[112];
                let FM = parameters[62];
                let FN = parameters[63];
                let FO = staged[113];
                let FZ = staged[115];
                let GA = staged[116];
                let GF = staged[117];
                let mut oE = 0.0;
                let B = temperature + parameters[147];
                let C = if B < 7.314999999999998e1f64 { 1.0 } else { 0.0 };
                let F;
                if C != 0.0 {
                    F = D;
                } else {
                    let E = if B > 6e2f64 { 1.0 } else { 0.0 };
                    oE = E;
                    let W = if E != 0.0 {
                        V
                    } else {
                        B
                    };
                    F = W;
                }
                let G = staged[0] * F;
                let I = H / G;
                let K = F - J;
                let L = J / F;
                let M = F / J;
                let N = M.ln();
                let O = (parameters[121] * F) * (F.ln());
                let P = parameters[122] * F;
                let Q = (parameters[117] + O) + P;
                let S = (Q + ((parameters[118] + O) + P)) * R;
                let T = (Q + ((parameters[119] + O) + P)) * R;
                let AJ;
                let AK;
                let AL;
                if U != 0.0 {
                    let Z = ((staged[2] * M) + (X * (H - M))) - ((Y * G) * N);
                    let AC = Z + ((AA * G) * ((R * (H + ((H + (AB * (((-Z) * I).exp()))).sqrt()))).ln()));
                    let AG = AF * ((AE * ((AD / AC).ln())).exp());
                    let AU = if AH != 0.0 {
                        let AS = (AI * AC) / AD;
                        AS
                    } else {
                        AT
                    };
                    AJ = AC;
                    AK = AG;
                    AL = AU;
                } else {
                    AJ = AD;
                    AK = AF;
                    AL = AI;
                }
                let AM = H - L;
                let AN = ((parameters[124] * N) + (staged[5] * AM)).exp();
                let AO = parameters[14] * AN;
                let AP = staged[7] * AM;
                let AQ = parameters[16] * (((staged[6] * N) + (AP / parameters[17])).exp());
                let BD;
                let BE;
                let BF;
                if AR != 0.0 {
                    let AW = ((staged[8] * M) + (AV * (H - M))) - ((Y * G) * N);
                    let AX = AW + ((AA * G) * ((R * (H + ((H + (AB * (((-AW) * I).exp()))).sqrt()))).ln()));
                    let BA = AZ * ((parameters[49] * ((AY / AX).ln())).exp());
                    let BI = if BB != 0.0 {
                        let BG = (BC * AX) / AY;
                        BG
                    } else {
                        BH
                    };
                    BD = AX;
                    BE = BA;
                    BF = BI;
                } else {
                    BD = AY;
                    BE = AZ;
                    BF = BC;
                }
                let BK = if A != 0.0 {
                    BJ
                } else {
                    BF
                };
                let BL = staged[11] * AM;
                let BM = parameters[23] * (((staged[10] * N) + BL).exp());
                let BN = AJ / AD;
                let BO = parameters[2] * (AA - ((AE * (BN.ln())).exp()));
                let BP = parameters[1] * (((parameters[123] * N) + (staged[12] * AM)).exp());
                let BQ = parameters[10] * ((parameters[126] * N).exp());
                let BW = if BR != 0.0 {
                    let BU = parameters[9] * (((BS * I) * (((BT * N).exp()) - H)).exp());
                    BU
                } else {
                    let BV = parameters[8] * (((BS * I) * (((BT * N).exp()) - H)).exp());
                    BV
                };
                let BX = parameters[3] * ((staged[13] * AM).exp());
                let BY = parameters[4] * ((staged[14] * AM).exp());
                let BZ = parameters[6] * ((staged[15] * AM).exp());
                let CA = parameters[75] * ((staged[16] * N).exp());
                let CB = H / (parameters[74] * ((parameters[130] * N).exp()));
                let CH;
                let CI;
                if CC != 0.0 {
                    let CE = CD * (H - (parameters[133] * K));
                    CH = CE;
                    CI = CF;
                } else {
                    let CG = CF * (H + (parameters[132] * K));
                    CH = CD;
                    CI = CG;
                }
                let CJ = parameters[66] * ((H + (parameters[128] * K)) + ((parameters[129] * K) * K));
                let CK = parameters[71] * ((staged[17] * N).exp());
                let CQ;
                let CR;
                if CL != 0.0 {
                    let CN = CM * ((parameters[139] * K).exp());
                    let CP = CO * ((parameters[140] * K).exp());
                    CQ = CP;
                    CR = CN;
                } else {
                    CQ = CO;
                    CR = CM;
                }
                let CS = parameters[89] * ((parameters[134] * N).exp());
                let DB;
                let DC;
                let DD;
                if CT != 0.0 {
                    let CU = ((staged[24] * M) + (X * (H - M))) - ((Y * G) * N);
                    let CV = CU + ((AA * G) * ((R * (H + ((H + (AB * (((-CU) * I).exp()))).sqrt()))).ln()));
                    let CY = CX * ((parameters[45] * ((CW / CV).ln())).exp());
                    let DI = if CZ != 0.0 {
                        let DG = (DA * CV) / CW;
                        DG
                    } else {
                        DH
                    };
                    DB = CV;
                    DC = CY;
                    DD = DI;
                } else {
                    DB = CW;
                    DC = CX;
                    DD = DA;
                }
                let DE = parameters[18] * AN;
                let DF = parameters[20] * (((staged[25] * N) + (AP / parameters[21])).exp());
                let DJ = parameters[30] * (((-(AJ - AD)) / parameters[31]).exp());
                let DR;
                let DS;
                let DT;
                if DK != 0.0 {
                    let DL = ((staged[36] * M) + (AV * (H - M))) - ((Y * G) * N);
                    let DM = DL + ((AA * G) * ((R * (H + ((H + (AB * (((-DL) * I).exp()))).sqrt()))).ln()));
                    let DO = (parameters[54] * ((DN / DM).ln())).exp();
                    let DW = if DP != 0.0 {
                        let DU = (DQ * DM) / DN;
                        DU
                    } else {
                        DV
                    };
                    DR = DO;
                    DS = DM;
                    DT = DW;
                } else {
                    DR = H;
                    DS = DN;
                    DT = DQ;
                }
                let DX = if A != 0.0 {
                    BJ
                } else {
                    DT
                };
                let DY = DR * staged[37];
                let DZ = DR * staged[38];
                let EA = parameters[25] * (((staged[39] * N) + BL).exp());
                let ED;
                let EE;
                let EF;
                if A != 0.0 {
                    let EU;
                    let EV;
                    let EW;
                    if EB != 0.0 {
                        let EM = ((staged[40] * M) + (EL * (H - M))) - ((Y * G) * N);
                        let EN = EM + ((AA * G) * ((R * (H + ((H + (AB * (((-EM) * I).exp()))).sqrt()))).ln()));
                        let ER = EQ * ((EP * ((EO / EN).ln())).exp());
                        let EZ = if ES != 0.0 {
                            let EX = (-2.4e0f64 * EN) / EO;
                            EX
                        } else {
                            EY
                        };
                        EU = ER;
                        EV = EN;
                        EW = EZ;
                    } else {
                        EU = EQ;
                        EV = EO;
                        EW = ET;
                    }
                    ED = EU;
                    EE = EV;
                    EF = EW;
                } else {
                    let FF;
                    let FG;
                    let FH;
                    if EC != 0.0 {
                        let FA = ((staged[42] * M) + (EL * (H - M))) - ((Y * G) * N);
                        let FB = FA + ((AA * G) * ((R * (H + ((H + (AB * (((-FA) * I).exp()))).sqrt()))).ln()));
                        let FC = EQ * ((EP * ((EO / FB).ln())).exp());
                        let FK = if FD != 0.0 {
                            let FI = (staged[43] * FB) / EO;
                            FI
                        } else {
                            FJ
                        };
                        FF = FC;
                        FG = FB;
                        FH = FK;
                    } else {
                        FF = EQ;
                        FG = EO;
                        FH = FE;
                    }
                    ED = FF;
                    EE = FG;
                    EF = FH;
                }
                let EG = staged[44] * N;
                let EH = parameters[99] * ((EG + (staged[45] * AM)).exp());
                let EI = parameters[97] * ((EG + BL).exp());
                let EJ = parameters[101] * ((staged[46] * N).exp());
                let FP;
                let FQ;
                let FR;
                if EK != 0.0 {
                    let GB;
                    let GC;
                    let GD;
                    if FL != 0.0 {
                        let FW = ((staged[47] * M) + (EL * (H - M))) - ((Y * G) * N);
                        let FX = FW + ((AA * G) * ((R * (H + ((H + (AB * (((-FW) * I).exp()))).sqrt()))).ln()));
                        let FY = FM * ((parameters[64] * ((FN / FX).ln())).exp());
                        let GG = if FZ != 0.0 {
                            let GE = (staged[48] * FX) / FN;
                            GE
                        } else {
                            GF
                        };
                        GB = FY;
                        GC = FX;
                        GD = GG;
                    } else {
                        GB = FM;
                        GC = FN;
                        GD = GA;
                    }
                    FP = GB;
                    FQ = GC;
                    FR = GD;
                } else {
                    FP = FM;
                    FQ = FN;
                    FR = FO;
                }
                let FS = parameters[96] * ((parameters[136] * N).exp());
                let FT = parameters[90] * ((parameters[135] * N).exp());
                let FU = parameters[95] * ((parameters[137] * N).exp());
                let FV = (parameters[142] * ((parameters[143] * N).exp())) * (H + (parameters[144] * K));
            [B, C, oE, G, I, S, T, AO, AQ, BM, AJ, BN, BO, BP, BQ, BX, BY, BZ, CA, CB, CJ, CK, BD, BE, CS, DE, DF, DB, DC, AK, DJ, DY, DZ, EA, EH, EI, EJ, FS, FT, FU, FV, AL, BK, BW, CH, CI, CQ, CR, DD, DS, DX, ED, EE, EF, FP, FQ, FR]
        };
        self.canonical_staged[49] = produced[0];
        self.canonical_staged[92] = produced[1];
        self.canonical_staged[94] = produced[2];
        self.canonical_staged[118] = produced[3];
        self.canonical_staged[122] = produced[4];
        self.canonical_staged[29] = produced[5];
        self.canonical_staged[21] = produced[6];
        self.canonical_staged[119] = produced[7];
        self.canonical_staged[120] = produced[8];
        self.canonical_staged[138] = produced[9];
        self.canonical_staged[123] = produced[10];
        self.canonical_staged[35] = produced[11];
        self.canonical_staged[128] = produced[12];
        self.canonical_staged[121] = produced[13];
        self.canonical_staged[126] = produced[14];
        self.canonical_staged[134] = produced[15];
        self.canonical_staged[136] = produced[16];
        self.canonical_staged[137] = produced[17];
        self.canonical_staged[132] = produced[18];
        self.canonical_staged[133] = produced[19];
        self.canonical_staged[129] = produced[20];
        self.canonical_staged[135] = produced[21];
        self.canonical_staged[22] = produced[22];
        self.canonical_staged[23] = produced[23];
        self.canonical_staged[141] = produced[24];
        self.canonical_staged[142] = produced[25];
        self.canonical_staged[143] = produced[26];
        self.canonical_staged[31] = produced[27];
        self.canonical_staged[32] = produced[28];
        self.canonical_staged[34] = produced[29];
        self.canonical_staged[145] = produced[30];
        self.canonical_staged[150] = produced[31];
        self.canonical_staged[146] = produced[32];
        self.canonical_staged[149] = produced[33];
        self.canonical_staged[159] = produced[34];
        self.canonical_staged[157] = produced[35];
        self.canonical_staged[158] = produced[36];
        self.canonical_staged[161] = produced[37];
        self.canonical_staged[162] = produced[38];
        self.canonical_staged[160] = produced[39];
        self.canonical_staged[163] = produced[40];
        self.canonical_staged[124] = produced[41];
        self.canonical_staged[125] = produced[42];
        self.canonical_staged[127] = produced[43];
        self.canonical_staged[130] = produced[44];
        self.canonical_staged[131] = produced[45];
        self.canonical_staged[139] = produced[46];
        self.canonical_staged[140] = produced[47];
        self.canonical_staged[144] = produced[48];
        self.canonical_staged[147] = produced[49];
        self.canonical_staged[148] = produced[50];
        self.canonical_staged[151] = produced[51];
        self.canonical_staged[152] = produced[52];
        self.canonical_staged[153] = produced[53];
        self.canonical_staged[154] = produced[54];
        self.canonical_staged[155] = produced[55];
        self.canonical_staged[156] = produced[56];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 20499 => 0usize, 20514 => 1usize, 20535 => 2usize, 20560 => 3usize, 20569 => 4usize, 20573 => 5usize, 20577 => 6usize, 20581 => 7usize, 20618 => 8usize, 20622 => 9usize, 20626 => 10usize, 20654 => 11usize, 20658 => 12usize, 20675 => 13usize, 20703 => 15usize, 20706 => 16usize, 20709 => 17usize, 20699 => 14usize, 20916 => 18usize, 20924 => 19usize, _ => usize::MAX };
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
            let B = node_potentials[6];
            let D = 1e0f64;
            let E = 1e0f64;
            let G = parameters[148];
            let J = node_potentials[5];
            let L = 1e0f64;
            let R = node_potentials[7];
            let S = 1e0f64;
            let Z = node_potentials[1];
            let AB = 1e0f64;
            let AF = node_potentials[9];
            let AH = 1e0f64;
            let AL = node_potentials[3];
            let AM = node_potentials[0];
            let AN = 1e0f64;
            let AO = 1e0f64;
            let AR = 0e0f64;
            let AT = staged[73];
            let AU = staged[33];
            let AV = staged[19];
            let AW = staged[99];
            let AX = staged[100];
            let AY = staged[101];
            let BA = parameters[48];
            let BC = 1e0f64;
            let BF = staged[30];
            let BG = staged[20];
            let BI = staged[22];
            let BK = staged[23];
            let BL = parameters[47];
            let BN = parameters[37];
            let BP = parameters[38];
            let BT = staged[26];
            let BV = staged[28];
            let BX = parameters[29];
            let BY = parameters[44];
            let CC = staged[31];
            let CE = staged[32];
            let CF = parameters[43];
            let CI = parameters[40];
            let CM = parameters[27];
            let CO = parameters[28];
            let CQ = staged[34];
            let CR = parameters[39];
            let CS = staged[35];
            let CX = staged[108];
            let CY = staged[114];
            let CZ = node_potentials[4];
            let DC = staged[118];
            let DD = staged[119];
            let DE = staged[120];
            let DF = staged[121];
            let DG = staged[122];
            let DH = staged[123];
            let DI = staged[124];
            let DJ = staged[125];
            let DK = staged[126];
            let DL = staged[127];
            let DM = staged[128];
            let DN = staged[129];
            let DO = staged[130];
            let DP = staged[131];
            let DQ = staged[132];
            let DR = staged[133];
            let DS = staged[134];
            let DT = staged[135];
            let DU = staged[136];
            let DV = staged[137];
            let DW = staged[138];
            let DX = staged[139];
            let DY = staged[140];
            let DZ = staged[141];
            let EA = staged[142];
            let EB = staged[143];
            let EC = staged[144];
            let ED = staged[145];
            let EE = staged[146];
            let EF = staged[147];
            let EG = staged[148];
            let EH = staged[149];
            let EI = staged[150];
            let EJ = staged[151];
            let EK = staged[152];
            let EL = staged[153];
            let EM = staged[154];
            let EN = staged[155];
            let EO = staged[156];
            let EP = staged[157];
            let EQ = staged[158];
            let ER = staged[159];
            let ES = staged[160];
            let ET = staged[161];
            let EU = staged[162];
            let EV = staged[163];
            let EW = 0e0f64;
            let JD = staged[164];
            let JE = 7.314999999999998e1f64;
            let JI = staged[0];
            let JM = -1e0f64;
            let JO = staged[1];
            let JV = 1e0f64;
            let JX = parameters[121];
            let KB = parameters[122];
            let KF = 5e-1f64;
            let KJ = 6e2f64;
            let KK = 1e0f64;
            let KN = staged[50];
            let KO = staged[3];
            let KP = staged[4];
            let KT = 2e0f64;
            let KX = 4e0f64;
            let KZ = 2e0f64;
            let LF = parameters[41];
            let LJ = staged[165];
            let LK = parameters[42];
            let LR = parameters[124];
            let LU = staged[5];
            let LX = parameters[14];
            let MA = staged[6];
            let MB = staged[7];
            let ME = parameters[17];
            let MG = parameters[16];
            let ML = staged[166];
            let MO = staged[51];
            let MP = staged[9];
            let NC = parameters[49];
            let NG = staged[167];
            let NH = parameters[50];
            let NQ = staged[168];
            let NT = 2.4e0f64;
            let NW = staged[10];
            let NX = staged[11];
            let OB = parameters[23];
            let OH = parameters[2];
            let OK = parameters[123];
            let OL = staged[12];
            let ON = parameters[1];
            let OQ = parameters[126];
            let OS = parameters[10];
            let OV = parameters[125];
            let OX = parameters[127];
            let PB = parameters[9];
            let PI = parameters[8];
            let PN = staged[13];
            let PP = parameters[3];
            let PS = staged[14];
            let PU = parameters[4];
            let PX = staged[15];
            let PZ = parameters[6];
            let QC = staged[16];
            let QE = parameters[75];
            let QH = parameters[130];
            let QJ = parameters[74];
            let QN = parameters[133];
            let QO = parameters[79];
            let QR = parameters[78];
            let QS = parameters[132];
            let QZ = parameters[128];
            let RA = parameters[129];
            let RC = parameters[66];
            let RF = staged[17];
            let RH = parameters[71];
            let RK = parameters[139];
            let RM = parameters[32];
            let RP = parameters[140];
            let RR = parameters[33];
            let SD = parameters[134];
            let SF = parameters[89];
            let TA = staged[52];
            let TN = parameters[45];
            let TR = staged[169];
            let TS = parameters[46];
            let TZ = parameters[18];
            let UC = staged[25];
            let UD = parameters[21];
            let UF = parameters[20];
            let UK = staged[170];
            let UU = parameters[31];
            let UW = parameters[30];
            let UZ = 1.0f64;
            let VJ = -1.5e0f64;
            let WD = -1.5e0f64;
            let WL = staged[53];
            let WX = parameters[53];
            let WZ = parameters[54];
            let XC = staged[171];
            let XD = parameters[55];
            let XM = staged[172];
            let XR = staged[37];
            let XU = staged[38];
            let XX = staged[39];
            let XZ = parameters[25];
            let YC = staged[173];
            let YD = staged[174];
            let YK = staged[44];
            let YN = staged[45];
            let YP = parameters[99];
            let YT = parameters[97];
            let YW = staged[46];
            let YY = parameters[101];
            let ZB = staged[54];
            let ZC = staged[41];
            let ZO = parameters[58];
            let ZQ = parameters[59];
            let ZS = parameters[57];
            let ZV = 0.0f64;
            let ZW = -2.4e0f64;
            let AAD = -2.4e0f64;
            let AAG = 2.4e0f64;
            let AAJ = staged[55];
            let AAZ = staged[175];
            let ABA = staged[176];
            let ABH = staged[56];
            let ABK = staged[177];
            let ABN = staged[178];
            let ABO = parameters[62];
            let ABP = parameters[63];
            let ABQ = staged[179];
            let ABX = parameters[136];
            let ABZ = parameters[96];
            let ACC = parameters[135];
            let ACE = parameters[90];
            let ACH = parameters[137];
            let ACJ = parameters[95];
            let ACM = parameters[143];
            let ACO = parameters[142];
            let ACQ = parameters[144];
            let ACU = staged[57];
            let ADH = parameters[64];
            let ADL = staged[180];
            let ADM = staged[181];
            let ADT = staged[58];
            let ADW = staged[182];
            let ADZ = parameters[15];
            let AED = 8e1f64;
            let AEF = Lanes([0e0f64; 3]);
            let AEI = staged[183];
            let AEZ = parameters[13];
            let AGA = 1.921812e0f64;
            let AHE = staged[184];
            let AHL = staged[185];
            let AHN = parameters[51];
            let AIH = Lanes([0e0f64; 3]);
            let AIY = 1e-1f64;
            let AMJ = parameters[11];
            let AMY = 1e-3f64;
            let ANE = parameters[12];
            let ANG = 5e-2f64;
            let ANR = staged[59];
            let AOH = staged[60];
            let AOM = parameters[67];
            let AON = parameters[68];
            let APP = staged[61];
            let APZ = parameters[77];
            let AQE = parameters[76];
            let ARA = staged[72];
            let ARE = parameters[85];
            let ARZ = 1e-6f64;
            let ASB = staged[63];
            let ASF = parameters[70];
            let ASH = parameters[69];
            let ASN = parameters[83];
            let ASQ = Lanes([0e0f64; 4]);
            let ATB = 1e-5f64;
            let ATK = parameters[73];
            let AUD = parameters[72];
            let AUJ = parameters[82];
            let AUX = parameters[115];
            let AUY = 1e-2f64;
            let AUZ = parameters[116];
            let AVA = 5e-3f64;
            let AVC = -1e10f64;
            let AVG = parameters[84];
            let AWF = staged[64];
            let AWQ = staged[65];
            let AWT = staged[66];
            let AXD = staged[68];
            let AYG = staged[67];
            let AYO = 2.5e-1f64;
            let AZF = staged[69];
            let AZO = staged[70];
            let AZP = staged[71];
            let BAO = parameters[5];
            let BBE = parameters[7];
            let BBY = parameters[93];
            let BEJ = 3e-1f64;
            let BGG = -1e10f64;
            let BOM = -1e10f64;
            let BTW = parameters[24];
            let BVI = staged[74];
            let BVM = parameters[35];
            let BVS = parameters[36];
            let BXG = parameters[34];
            let BYH = staged[186];
            let BYJ = parameters[91];
            let BZA = parameters[94];
            let BZH = parameters[19];
            let BZM = Lanes([0e0f64; 3]);
            let BZP = staged[187];
            let CCC = Lanes([0e0f64; 4]);
            let CCK = staged[188];
            let CDU = staged[189];
            let CDW = parameters[56];
            let CEQ = Lanes([0e0f64; 3]);
            let CHE = parameters[26];
            let CHY = staged[190];
            let CIT = Lanes([0e0f64; 3]);
            let CLM = parameters[61];
            let CMG = Lanes([0e0f64; 3]);
            let COU = staged[191];
            let CPA = staged[192];
            let CPG = parameters[65];
            let CQA = Lanes([0e0f64; 3]);
            let CSO = parameters[98];
            let CTA = staged[193];
            let CTB = Lanes([0e0f64; 4]);
            let CTG = staged[194];
            let CTM = parameters[100];
            let CTT = staged[195];
            let CUD = staged[196];
            let CUE = 0e0f64;
            let CUF = Lanes([0e0f64; 9]);
            let CUI = staged[197];
            let CUP = staged[198];
            let CVJ = parameters[149];
            let CVX = node_potentials[2];
            let CVZ = 1e0f64;
            let CXF = node_potentials[11];
            let CXG = 1e0f64;
            let CXM = node_potentials[10];
            let CXN = 1e0f64;
            let CXT = parameters[88];
            let CXW = 3e0f64;
            let CYA = node_potentials[12];
            let CYC = 1e0f64;
            let CYH = parameters[87];
            let CYQ = 0e0f64;
            let CYR = 0e0f64;
            let CYS = 0e0f64;
            let CZQ = ddt_scale();
            let DAD = staged[199];
            let DAH = staged[200];
            let DAI = Lanes([0e0f64; 5]);
            let DAP = staged[201];
            let DAW = staged[75];
            let DAZ = staged[76];
            let DBG = staged[77];
            let DBV = staged[78];
            let DCE = staged[79];
            let DCJ = staged[202];
            let DCN = Lanes([0e0f64; 3]);
            let DCQ = staged[203];
            let DCU = Lanes([0e0f64; 3]);
            let DCX = staged[204];
            let DDB = Lanes([0e0f64; 3]);
            let DDE = staged[80];
            let DDJ = staged[81];
            let DDO = parameters[108];
            let DEN = staged[205];
            let DES = Lanes([0e0f64; 2]);
            let DFD = parameters[102];
            let DFG = staged[206];
            let DFH = Lanes([0e0f64; 2]);
            let DFO = staged[207];
            let DFP = parameters[103];
            let DGB = staged[208];
            let DGO = parameters[145];
            let DGX = staged[210];
            let DGZ = node_potentials[13];
            let DHA = node_potentials[14];
            let DHB = 0e0f64;
            let DHC = Lanes([0e0f64; 5]);
            let DHD = Lanes([0e0f64; 5]);
            let DHE = 0e0f64;
            let DHF = 1e0f64;
            let DHG = 1e0f64;
            let DIF = 1e9f64;
            let DIK = staged[82];
            let DRT = 0e0f64;
            let DRU = 0e0f64;
            let DRV = 0e0f64;
            let DRW = 0e0f64;
            let DRX = 0e0f64;
            let DRY = 0e0f64;
            let C = A - B;
            let F = Lanes([0.0, D]) - Lanes([E, 0.0]);
            let H = G * C;
            let I = F * G;
            let K = A - J;
            let M = Lanes([0.0, D]) - Lanes([L, 0.0]);
            let N = G * K;
            let O = M * G;
            let P = H - N;
            let Q = Lanes([0.0, I[0], I[1]]) - Lanes([O[0], 0.0, O[1]]);
            let T = G * (R - B);
            let U = (Lanes([0.0, S]) - Lanes([E, 0.0])) * G;
            let V = R - J;
            let W = Lanes([0.0, S]) - Lanes([L, 0.0]);
            let X = G * V;
            let Y = W * G;
            let AA = Z - J;
            let AC = Lanes([AB, 0.0]) - Lanes([0.0, L]);
            let AD = G * AA;
            let AE = AC * G;
            let AG = AF - J;
            let AI = Lanes([0.0, AH]) - Lanes([L, 0.0]);
            let AJ = G * AG;
            let AK = AI * G;
            let AP = G * (AL - AM);
            let AQ = (Lanes([0.0, AN]) - Lanes([AO, 0.0])) * G;
            let AS = ctx.simparam_or("gmin", AR);
            let AZ = if staged[18] != 0.0 && (if N < AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BD;
            let BE;
            if AZ != 0.0 {
                let BB = if AV != 0.0 && (if BA > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BR;
                let BS;
                if BB != 0.0 {
                    let BH = BG / staged[21];
                    let BJ = BI / BA;
                    let BM = (((BH.sqrt()) * BJ) * BK) / BL;
                    let BO = (BN * BM) * BJ;
                    let BQ = BP / (BM * BH);
                    BR = BO;
                    BS = BQ;
                } else {
                    BR = BN;
                    BS = BP;
                }
                BD = BR;
                BE = BS;
            } else {
                BD = AR;
                BE = BC;
            }
            let BU = if staged[27] != 0.0 && (if (if T < BT { 1.0 } else { 0.0 }) != 0.0 || (if H < BT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CA;
            let CB;
            if BU != 0.0 {
                let BW = BV / staged[29];
                let BZ = if (if (if BX == BC { 1.0 } else { 0.0 }) != 0.0 && BF != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BY > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CK;
                let CL;
                if BZ != 0.0 {
                    let CD = CC / BY;
                    let CG = (((CE / CF) * (BW.sqrt())) * CD) * CD;
                    let CH = ((CF / CE) * (BW.powf(-1.5e0f64))) / CD;
                    CK = CG;
                    CL = CH;
                } else {
                    let CJ = if (if (if BX == AR { 1.0 } else { 0.0 }) != 0.0 && AU != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CI > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CV;
                    let CW;
                    if CJ != 0.0 {
                        let CT = (((CQ / CR) * (BW.sqrt())) * CS) * CS;
                        let CU = ((CR / CQ) * (BW.powf(-1.5e0f64))) / CS;
                        CV = CT;
                        CW = CU;
                    } else {
                        CV = BC;
                        CW = BC;
                    }
                    CK = CV;
                    CL = CW;
                }
                let CN = CM * CK;
                let CP = CO * CL;
                CA = CN;
                CB = CP;
            } else {
                CA = AR;
                CB = BC;
            }
            let EX;
            let EY;
            let EZ;
            let FA;
            let FB;
            let FC;
            let FD;
            let FE;
            let FF;
            let FG;
            let FH;
            let FI;
            let FJ;
            let FK;
            let FL;
            let FM;
            let FN;
            let FO;
            let FP;
            let FQ;
            let FR;
            let FS;
            let FT;
            let FU;
            let FV;
            let FW;
            let FX;
            let FY;
            let FZ;
            let GA;
            let GB;
            let GC;
            let GD;
            let GE;
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
            let GT;
            let GU;
            let GV;
            let GW;
            let GX;
            let GY;
            let GZ;
            let HA;
            let HB;
            let HC;
            let HD;
            let HE;
            let HF;
            let HG;
            let HH;
            let HI;
            let HJ;
            let HK;
            let HL;
            let HM;
            let HN;
            let HO;
            let HP;
            let HQ;
            let HR;
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
            let IQ;
            let IR;
            let IS;
            let IT;
            let IU;
            let IV;
            let IW;
            let IX;
            let IY;
            let IZ;
            let JA;
            let JB;
            let JC;
            if CY != 0.0 {
                let DA = staged[49] + CZ;
                let DB = if DA < 7.314999999999998e1f64 { 1.0 } else { 0.0 };
                let JG;
                let JH;
                if DB != 0.0 {
                    JG = JE;
                    JH = EW;
                } else {
                    let JF = if DA > 6e2f64 { 1.0 } else { 0.0 };
                    let KL;
                    let KM;
                    if JF != 0.0 {
                        KL = KJ;
                        KM = EW;
                    } else {
                        KL = DA;
                        KM = KK;
                    }
                    JG = KL;
                    JH = KM;
                }
                let JJ = JI * JG;
                let JK = JH * JI;
                let JL = BC / JJ;
                let JN = ((JK * JL) * JM) / JJ;
                let JP = JG - JO;
                let JQ = JO / JG;
                let JR = ((JH * JQ) * JM) / JG;
                let JS = JG / JO;
                let JT = JH / JO;
                let JU = JS.ln();
                let JW = JT * (JV / JS);
                let JY = JX * JG;
                let JZ = JG.ln();
                let KA = JY * JZ;
                let KC = KB * JG;
                let KD = (parameters[117] + KA) + KC;
                let KE = (((JH * JX) * JZ) + ((JH * (JV / JG)) * JY)) + (JH * KB);
                let KG = (KD + ((parameters[118] + KA) + KC)) * KF;
                let KH = (KE + KE) * KF;
                let KI = (KD + ((parameters[119] + KA) + KC)) * KF;
                let LL;
                let LM;
                let LN;
                let LO;
                let LP;
                let LQ;
                if AU != 0.0 {
                    let KQ = KP * JJ;
                    let KR = ((KN * JS) + (KO * (BC - JS))) - (KQ * JU);
                    let KS = ((JT * KN) + ((JT * JM) * KO)) - (((JK * KP) * JU) + (JW * KQ));
                    let KU = KT * JJ;
                    let KV = -KR;
                    let KW = (KV * JL).exp();
                    let KY = (BC + (KX * KW)).sqrt();
                    let LA = KF * (BC + KY);
                    let LB = LA.ln();
                    let LC = KR + (KU * LB);
                    let LD = KS + (((JK * KT) * LB) + (((((((((KS * JM) * JL) + (JN * KV)) * KW) * KX) * (JV / (KZ * KY))) * KF) * (JV / LA)) * KU));
                    let LE = CI / LC;
                    let LG = (LF * (LE.ln())).exp();
                    let LH = CR * LG;
                    let LI = ((((((LD * LE) * JM) / LC) * (JV / LE)) * LF) * LG) * CR;
                    let MM;
                    let MN;
                    if LJ != 0.0 {
                        let MJ = (LK * LC) / CI;
                        let MK = (LD * LK) / CI;
                        MM = MJ;
                        MN = MK;
                    } else {
                        MM = ML;
                        MN = EW;
                    }
                    LL = LC;
                    LM = LH;
                    LN = MM;
                    LO = LD;
                    LP = LI;
                    LQ = MN;
                } else {
                    LL = CI;
                    LM = CR;
                    LN = LK;
                    LO = EW;
                    LP = EW;
                    LQ = EW;
                }
                let LS = BC - JQ;
                let LT = JR * JM;
                let LV = ((LR * JU) + (LU * LS)).exp();
                let LW = ((JW * LR) + (LT * LU)) * LV;
                let LY = LX * LV;
                let LZ = LW * LX;
                let MC = MB * LS;
                let MD = LT * MB;
                let MF = ((MA * JU) + (MC / ME)).exp();
                let MH = MG * MF;
                let MI = (((JW * MA) + (MD / ME)) * MF) * MG;
                let NI;
                let NJ;
                let NK;
                let NL;
                let NM;
                let NN;
                if AV != 0.0 {
                    let MQ = KP * JJ;
                    let MR = ((MO * JS) + (MP * (BC - JS))) - (MQ * JU);
                    let MS = ((JT * MO) + ((JT * JM) * MP)) - (((JK * KP) * JU) + (JW * MQ));
                    let MT = KT * JJ;
                    let MU = -MR;
                    let MV = (MU * JL).exp();
                    let MW = (BC + (KX * MV)).sqrt();
                    let MX = KF * (BC + MW);
                    let MY = MX.ln();
                    let MZ = MR + (MT * MY);
                    let NA = MS + (((JK * KT) * MY) + (((((((((MS * JM) * JL) + (JN * MU)) * MV) * KX) * (JV / (KZ * MW))) * KF) * (JV / MX)) * MT));
                    let NB = BA / MZ;
                    let ND = (NC * (NB.ln())).exp();
                    let NE = BL * ND;
                    let NF = ((((((NA * NB) * JM) / MZ) * (JV / NB)) * NC) * ND) * BL;
                    let NR;
                    let NS;
                    if NG != 0.0 {
                        let NO = (NH * MZ) / BA;
                        let NP = (NA * NH) / BA;
                        NR = NO;
                        NS = NP;
                    } else {
                        NR = NQ;
                        NS = EW;
                    }
                    NI = MZ;
                    NJ = NE;
                    NK = NR;
                    NL = NA;
                    NM = NF;
                    NN = NS;
                } else {
                    NI = BA;
                    NJ = BL;
                    NK = NH;
                    NL = EW;
                    NM = EW;
                    NN = EW;
                }
                let NU;
                let NV;
                if AT != 0.0 {
                    NU = NT;
                    NV = EW;
                } else {
                    NU = NK;
                    NV = NN;
                }
                let NY = NX * LS;
                let NZ = LT * NX;
                let OA = ((NW * JU) + NY).exp();
                let OC = OB * OA;
                let OD = (((JW * NW) + NZ) * OA) * OB;
                let OE = LL / CI;
                let OF = LO / CI;
                let OG = (LF * (OE.ln())).exp();
                let OI = OH * (KT - OG);
                let OJ = ((((OF * (JV / OE)) * LF) * OG) * JM) * OH;
                let OM = ((OK * JU) + (OL * LS)).exp();
                let OO = ON * OM;
                let OP = (((JW * OK) + (LT * OL)) * OM) * ON;
                let OR = (OQ * JU).exp();
                let OT = OS * OR;
                let OU = ((JW * OQ) * OR) * OS;
                let PL;
                let PM;
                if AW != 0.0 {
                    let OW = OV * JL;
                    let OY = (OX * JU).exp();
                    let OZ = OY - BC;
                    let PA = (OW * OZ).exp();
                    let PC = PB * PA;
                    let PD = ((((JN * OV) * OZ) + (((JW * OX) * OY) * OW)) * PA) * PB;
                    PL = PC;
                    PM = PD;
                } else {
                    let PE = OV * JL;
                    let PF = (OX * JU).exp();
                    let PG = PF - BC;
                    let PH = (PE * PG).exp();
                    let PJ = PI * PH;
                    let PK = ((((JN * OV) * PG) + (((JW * OX) * PF) * PE)) * PH) * PI;
                    PL = PJ;
                    PM = PK;
                }
                let PO = (PN * LS).exp();
                let PQ = PP * PO;
                let PR = ((LT * PN) * PO) * PP;
                let PT = (PS * LS).exp();
                let PV = PU * PT;
                let PW = ((LT * PS) * PT) * PU;
                let PY = (PX * LS).exp();
                let QA = PZ * PY;
                let QB = ((LT * PX) * PY) * PZ;
                let QD = (QC * JU).exp();
                let QF = QE * QD;
                let QG = ((JW * QC) * QD) * QE;
                let QI = (QH * JU).exp();
                let QK = QJ * QI;
                let QL = BC / QK;
                let QM = (((((JW * QH) * QI) * QJ) * QL) * JM) / QK;
                let QV;
                let QW;
                let QX;
                let QY;
                if AX != 0.0 {
                    let QP = QO * (BC - (QN * JP));
                    let QQ = ((JH * QN) * JM) * QO;
                    QV = QP;
                    QW = QR;
                    QX = QQ;
                    QY = EW;
                } else {
                    let QT = QR * (BC + (QS * JP));
                    let QU = (JH * QS) * QR;
                    QV = QO;
                    QW = QT;
                    QX = EW;
                    QY = QU;
                }
                let RB = RA * JP;
                let RD = RC * ((BC + (QZ * JP)) + (RB * JP));
                let RE = ((JH * QZ) + (((JH * RA) * JP) + (JH * RB))) * RC;
                let RG = (RF * JU).exp();
                let RI = RH * RG;
                let RJ = ((JW * RF) * RG) * RH;
                let RU;
                let RV;
                let RW;
                let RX;
                if AY != 0.0 {
                    let RL = (RK * JP).exp();
                    let RN = RM * RL;
                    let RO = ((JH * RK) * RL) * RM;
                    let RQ = (RP * JP).exp();
                    let RS = RR * RQ;
                    let RT = ((JH * RP) * RQ) * RR;
                    RU = RS;
                    RV = RN;
                    RW = RT;
                    RX = RO;
                } else {
                    RU = RR;
                    RV = RM;
                    RW = EW;
                    RX = EW;
                }
                let RZ;
                let SA;
                let SB;
                let SC;
                if AZ != 0.0 {
                    let RY = if AV != 0.0 && (if BA > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let SW;
                    let SX;
                    let SY;
                    let SZ;
                    if RY != 0.0 {
                        let SI = BG / KI;
                        let SJ = ((KH * SI) * JM) / KI;
                        let SK = NI / BA;
                        let SL = NL / BA;
                        let SM = SI.sqrt();
                        let SN = SM * SK;
                        let SO = (SN * NJ) / BL;
                        let SP = (((((SJ * (JV / (KZ * SM))) * SK) + (SL * SM)) * NJ) + (NM * SN)) / BL;
                        let SQ = BN * SO;
                        let SR = SQ * SK;
                        let SS = ((SP * BN) * SK) + (SL * SQ);
                        let ST = SO * SI;
                        let SU = BP / ST;
                        let SV = ((((SP * SI) + (SJ * SO)) * SU) * JM) / ST;
                        SW = SR;
                        SX = SU;
                        SY = SS;
                        SZ = SV;
                    } else {
                        SW = BN;
                        SX = BP;
                        SY = EW;
                        SZ = EW;
                    }
                    RZ = SW;
                    SA = SX;
                    SB = SY;
                    SC = SZ;
                } else {
                    RZ = AR;
                    SA = BC;
                    SB = EW;
                    SC = EW;
                }
                let SE = (SD * JU).exp();
                let SG = SF * SE;
                let SH = ((JW * SD) * SE) * SF;
                let TT;
                let TU;
                let TV;
                let TW;
                let TX;
                let TY;
                if BF != 0.0 {
                    let TB = KP * JJ;
                    let TC = ((TA * JS) + (KO * (BC - JS))) - (TB * JU);
                    let TD = ((JT * TA) + ((JT * JM) * KO)) - (((JK * KP) * JU) + (JW * TB));
                    let TE = KT * JJ;
                    let TF = -TC;
                    let TG = (TF * JL).exp();
                    let TH = (BC + (KX * TG)).sqrt();
                    let TI = KF * (BC + TH);
                    let TJ = TI.ln();
                    let TK = TC + (TE * TJ);
                    let TL = TD + (((JK * KT) * TJ) + (((((((((TD * JM) * JL) + (JN * TF)) * TG) * KX) * (JV / (KZ * TH))) * KF) * (JV / TI)) * TE));
                    let TM = BY / TK;
                    let TO = (TN * (TM.ln())).exp();
                    let TP = CF * TO;
                    let TQ = ((((((TL * TM) * JM) / TK) * (JV / TM)) * TN) * TO) * CF;
                    let UL;
                    let UM;
                    if TR != 0.0 {
                        let UI = (TS * TK) / BY;
                        let UJ = (TL * TS) / BY;
                        UL = UI;
                        UM = UJ;
                    } else {
                        UL = UK;
                        UM = EW;
                    }
                    TT = TK;
                    TU = TP;
                    TV = UL;
                    TW = TL;
                    TX = TQ;
                    TY = UM;
                } else {
                    TT = BY;
                    TU = CF;
                    TV = TS;
                    TW = EW;
                    TX = EW;
                    TY = EW;
                }
                let UA = TZ * LV;
                let UB = LW * TZ;
                let UE = ((UC * JU) + (MC / UD)).exp();
                let UG = UF * UE;
                let UH = (((JW * UC) + (MD / UD)) * UE) * UF;
                let UQ;
                let UR;
                let US;
                let UT;
                if BU != 0.0 {
                    let UN = BV / KG;
                    let UO = ((KH * UN) * JM) / KG;
                    let UP = if (if (if BX == BC { 1.0 } else { 0.0 }) != 0.0 && BF != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BY > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let VO;
                    let VP;
                    let VQ;
                    let VR;
                    if UP != 0.0 {
                        let VA = TT / BY;
                        let VB = TW / BY;
                        let VC = TU / CF;
                        let VD = UN.sqrt();
                        let VE = VC * VD;
                        let VF = VE * VA;
                        let VG = VF * VA;
                        let VH = ((((((TX / CF) * VD) + ((UO * (JV / (KZ * VD))) * VC)) * VA) + (VB * VE)) * VA) + (VB * VF);
                        let VI = CF / TU;
                        let VK = UN.powf(VJ);
                        let VL = (VI * VK) / VA;
                        let VM = ((((((TX * VI) * JM) / TU) * VK) + ((UO * (VJ * (UN.powf(-2.5e0f64)))) * VI)) - (VB * VL)) / VA;
                        VO = VG;
                        VP = VL;
                        VQ = VH;
                        VR = VM;
                    } else {
                        let VN = if (if (if BX == AR { 1.0 } else { 0.0 }) != 0.0 && AU != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CI > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let WH;
                        let WI;
                        let WJ;
                        let WK;
                        if VN != 0.0 {
                            let VW = LM / CR;
                            let VX = UN.sqrt();
                            let VY = VW * VX;
                            let VZ = VY * OE;
                            let WA = VZ * OE;
                            let WB = ((((((LP / CR) * VX) + ((UO * (JV / (KZ * VX))) * VW)) * OE) + (OF * VY)) * OE) + (OF * VZ);
                            let WC = CR / LM;
                            let WE = UN.powf(WD);
                            let WF = (WC * WE) / OE;
                            let WG = ((((((LP * WC) * JM) / LM) * WE) + ((UO * (WD * (UN.powf(-2.5e0f64)))) * WC)) - (OF * WF)) / OE;
                            WH = WA;
                            WI = WF;
                            WJ = WB;
                            WK = WG;
                        } else {
                            WH = BC;
                            WI = BC;
                            WJ = EW;
                            WK = EW;
                        }
                        VO = WH;
                        VP = WI;
                        VQ = WJ;
                        VR = WK;
                    }
                    let VS = CM * VO;
                    let VT = VQ * CM;
                    let VU = CO * VP;
                    let VV = VR * CO;
                    UQ = VS;
                    UR = VU;
                    US = VT;
                    UT = VV;
                } else {
                    UQ = AR;
                    UR = BC;
                    US = EW;
                    UT = EW;
                }
                let UV = ((-(LL - CI)) / UU).exp();
                let UX = UW * UV;
                let UY = (((LO * JM) / UU) * UV) * UW;
                let XE;
                let XF;
                let XG;
                let XH;
                let XI;
                let XJ;
                if UZ != 0.0 {
                    let WM = KP * JJ;
                    let WN = ((WL * JS) + (MP * (BC - JS))) - (WM * JU);
                    let WO = ((JT * WL) + ((JT * JM) * MP)) - (((JK * KP) * JU) + (JW * WM));
                    let WP = KT * JJ;
                    let WQ = -WN;
                    let WR = (WQ * JL).exp();
                    let WS = (BC + (KX * WR)).sqrt();
                    let WT = KF * (BC + WS);
                    let WU = WT.ln();
                    let WV = WN + (WP * WU);
                    let WW = WO + (((JK * KT) * WU) + (((((((((WO * JM) * JL) + (JN * WQ)) * WR) * KX) * (JV / (KZ * WS))) * KF) * (JV / WT)) * WP));
                    let WY = WX / WV;
                    let XA = (WZ * (WY.ln())).exp();
                    let XB = (((((WW * WY) * JM) / WV) * (JV / WY)) * WZ) * XA;
                    let XN;
                    let XO;
                    if XC != 0.0 {
                        let XK = (XD * WV) / WX;
                        let XL = (WW * XD) / WX;
                        XN = XK;
                        XO = XL;
                    } else {
                        XN = XM;
                        XO = EW;
                    }
                    XE = XA;
                    XF = WV;
                    XG = XN;
                    XH = XB;
                    XI = WW;
                    XJ = XO;
                } else {
                    XE = BC;
                    XF = WX;
                    XG = XD;
                    XH = EW;
                    XI = EW;
                    XJ = EW;
                }
                let XP;
                let XQ;
                if AT != 0.0 {
                    XP = NT;
                    XQ = EW;
                } else {
                    XP = XG;
                    XQ = XJ;
                }
                let XS = XE * XR;
                let XT = XH * XR;
                let XV = XE * XU;
                let XW = XH * XU;
                let XY = ((XX * JU) + NY).exp();
                let YA = XZ * XY;
                let YB = (((JW * XX) + NZ) * XY) * XZ;
                let YE;
                let YF;
                let YG;
                let YH;
                let YI;
                let YJ;
                if AT != 0.0 {
                    let ZX;
                    let ZY;
                    let ZZ;
                    let AAA;
                    let AAB;
                    let AAC;
                    if YC != 0.0 {
                        let ZD = KP * JJ;
                        let ZE = ((ZB * JS) + (ZC * (BC - JS))) - (ZD * JU);
                        let ZF = ((JT * ZB) + ((JT * JM) * ZC)) - (((JK * KP) * JU) + (JW * ZD));
                        let ZG = KT * JJ;
                        let ZH = -ZE;
                        let ZI = (ZH * JL).exp();
                        let ZJ = (BC + (KX * ZI)).sqrt();
                        let ZK = KF * (BC + ZJ);
                        let ZL = ZK.ln();
                        let ZM = ZE + (ZG * ZL);
                        let ZN = ZF + (((JK * KT) * ZL) + (((((((((ZF * JM) * JL) + (JN * ZH)) * ZI) * KX) * (JV / (KZ * ZJ))) * KF) * (JV / ZK)) * ZG));
                        let ZP = ZO / ZM;
                        let ZR = (ZQ * (ZP.ln())).exp();
                        let ZT = ZS * ZR;
                        let ZU = ((((((ZN * ZP) * JM) / ZM) * (JV / ZP)) * ZQ) * ZR) * ZS;
                        let AAH;
                        let AAI;
                        if ZV != 0.0 {
                            let AAE = (AAD * ZM) / ZO;
                            let AAF = (ZN * AAD) / ZO;
                            AAH = AAE;
                            AAI = AAF;
                        } else {
                            AAH = AAG;
                            AAI = EW;
                        }
                        ZX = ZT;
                        ZY = ZM;
                        ZZ = AAH;
                        AAA = ZU;
                        AAB = ZN;
                        AAC = AAI;
                    } else {
                        ZX = ZS;
                        ZY = ZO;
                        ZZ = ZW;
                        AAA = EW;
                        AAB = EW;
                        AAC = EW;
                    }
                    YE = ZX;
                    YF = ZY;
                    YG = ZZ;
                    YH = AAA;
                    YI = AAB;
                    YJ = AAC;
                } else {
                    let ABB;
                    let ABC;
                    let ABD;
                    let ABE;
                    let ABF;
                    let ABG;
                    if YD != 0.0 {
                        let AAK = KP * JJ;
                        let AAL = ((AAJ * JS) + (ZC * (BC - JS))) - (AAK * JU);
                        let AAM = ((JT * AAJ) + ((JT * JM) * ZC)) - (((JK * KP) * JU) + (JW * AAK));
                        let AAN = KT * JJ;
                        let AAO = -AAL;
                        let AAP = (AAO * JL).exp();
                        let AAQ = (BC + (KX * AAP)).sqrt();
                        let AAR = KF * (BC + AAQ);
                        let AAS = AAR.ln();
                        let AAT = AAL + (AAN * AAS);
                        let AAU = AAM + (((JK * KT) * AAS) + (((((((((AAM * JM) * JL) + (JN * AAO)) * AAP) * KX) * (JV / (KZ * AAQ))) * KF) * (JV / AAR)) * AAN));
                        let AAV = ZO / AAT;
                        let AAW = (ZQ * (AAV.ln())).exp();
                        let AAX = ZS * AAW;
                        let AAY = ((((((AAU * AAV) * JM) / AAT) * (JV / AAV)) * ZQ) * AAW) * ZS;
                        let ABL;
                        let ABM;
                        if AAZ != 0.0 {
                            let ABI = (ABH * AAT) / ZO;
                            let ABJ = (AAU * ABH) / ZO;
                            ABL = ABI;
                            ABM = ABJ;
                        } else {
                            ABL = ABK;
                            ABM = EW;
                        }
                        ABB = AAX;
                        ABC = AAT;
                        ABD = ABL;
                        ABE = AAY;
                        ABF = AAU;
                        ABG = ABM;
                    } else {
                        ABB = ZS;
                        ABC = ZO;
                        ABD = ABA;
                        ABE = EW;
                        ABF = EW;
                        ABG = EW;
                    }
                    YE = ABB;
                    YF = ABC;
                    YG = ABD;
                    YH = ABE;
                    YI = ABF;
                    YJ = ABG;
                }
                let YL = YK * JU;
                let YM = JW * YK;
                let YO = (YL + (YN * LS)).exp();
                let YQ = YP * YO;
                let YR = ((YM + (LT * YN)) * YO) * YP;
                let YS = (YL + NY).exp();
                let YU = YT * YS;
                let YV = ((YM + NZ) * YS) * YT;
                let YX = (YW * JU).exp();
                let YZ = YY * YX;
                let ZA = ((JW * YW) * YX) * YY;
                let ABR;
                let ABS;
                let ABT;
                let ABU;
                let ABV;
                let ABW;
                if CX != 0.0 {
                    let ADN;
                    let ADO;
                    let ADP;
                    let ADQ;
                    let ADR;
                    let ADS;
                    if ABN != 0.0 {
                        let ACV = KP * JJ;
                        let ACW = ((ACU * JS) + (ZC * (BC - JS))) - (ACV * JU);
                        let ACX = ((JT * ACU) + ((JT * JM) * ZC)) - (((JK * KP) * JU) + (JW * ACV));
                        let ACY = KT * JJ;
                        let ACZ = -ACW;
                        let ADA = (ACZ * JL).exp();
                        let ADB = (BC + (KX * ADA)).sqrt();
                        let ADC = KF * (BC + ADB);
                        let ADD = ADC.ln();
                        let ADE = ACW + (ACY * ADD);
                        let ADF = ACX + (((JK * KT) * ADD) + (((((((((ACX * JM) * JL) + (JN * ACZ)) * ADA) * KX) * (JV / (KZ * ADB))) * KF) * (JV / ADC)) * ACY));
                        let ADG = ABP / ADE;
                        let ADI = (ADH * (ADG.ln())).exp();
                        let ADJ = ABO * ADI;
                        let ADK = ((((((ADF * ADG) * JM) / ADE) * (JV / ADG)) * ADH) * ADI) * ABO;
                        let ADX;
                        let ADY;
                        if ADL != 0.0 {
                            let ADU = (ADT * ADE) / ABP;
                            let ADV = (ADF * ADT) / ABP;
                            ADX = ADU;
                            ADY = ADV;
                        } else {
                            ADX = ADW;
                            ADY = EW;
                        }
                        ADN = ADJ;
                        ADO = ADE;
                        ADP = ADX;
                        ADQ = ADK;
                        ADR = ADF;
                        ADS = ADY;
                    } else {
                        ADN = ABO;
                        ADO = ABP;
                        ADP = ADM;
                        ADQ = EW;
                        ADR = EW;
                        ADS = EW;
                    }
                    ABR = ADN;
                    ABS = ADO;
                    ABT = ADP;
                    ABU = ADQ;
                    ABV = ADR;
                    ABW = ADS;
                } else {
                    ABR = ABO;
                    ABS = ABP;
                    ABT = ABQ;
                    ABU = EW;
                    ABV = EW;
                    ABW = EW;
                }
                let ABY = (ABX * JU).exp();
                let ACA = ABZ * ABY;
                let ACB = ((JW * ABX) * ABY) * ABZ;
                let ACD = (ACC * JU).exp();
                let ACF = ACE * ACD;
                let ACG = ((JW * ACC) * ACD) * ACE;
                let ACI = (ACH * JU).exp();
                let ACK = ACJ * ACI;
                let ACL = ((JW * ACH) * ACI) * ACJ;
                let ACN = (ACM * JU).exp();
                let ACP = ACO * ACN;
                let ACR = BC + (ACQ * JP);
                let ACS = ACP * ACR;
                let ACT = ((((JW * ACM) * ACN) * ACO) * ACR) + ((JH * ACQ) * ACP);
                EX = JJ;
                EY = LY;
                EZ = MH;
                FA = OO;
                FB = JL;
                FC = LM;
                FD = LL;
                FE = LN;
                FF = NJ;
                FG = NI;
                FH = NU;
                FI = OT;
                FJ = PL;
                FK = OI;
                FL = RD;
                FM = QV;
                FN = QW;
                FO = QF;
                FP = QL;
                FQ = PQ;
                FR = RI;
                FS = PV;
                FT = QA;
                FU = OC;
                FV = RZ;
                FW = SA;
                FX = RU;
                FY = RV;
                FZ = SG;
                GA = UA;
                GB = UG;
                GC = TU;
                GD = TT;
                GE = TV;
                GF = UQ;
                GG = UR;
                GH = UX;
                GI = XV;
                GJ = XF;
                GK = XP;
                GL = YA;
                GM = XS;
                GN = YE;
                GO = YF;
                GP = YG;
                GQ = ABR;
                GR = ABS;
                GS = ABT;
                GT = YU;
                GU = YZ;
                GV = YQ;
                GW = ACK;
                GX = ACA;
                GY = ACF;
                GZ = ACS;
                HA = JK;
                HB = LZ;
                HC = MI;
                HD = OP;
                HE = JN;
                HF = LP;
                HG = LO;
                HH = LQ;
                HI = NM;
                HJ = NL;
                HK = NV;
                HL = OU;
                HM = PM;
                HN = OJ;
                HO = RE;
                HP = QX;
                HQ = QY;
                HR = QG;
                HS = QM;
                HT = PR;
                HU = RJ;
                HV = PW;
                HW = QB;
                HX = OD;
                HY = SB;
                HZ = SC;
                IA = RW;
                IB = RX;
                IC = SH;
                ID = UB;
                IE = UH;
                IF = TX;
                IG = TW;
                IH = TY;
                II = US;
                IJ = UT;
                IK = UY;
                IL = XW;
                IM = XI;
                IN = XQ;
                IO = YB;
                IP = XT;
                IQ = YH;
                IR = YI;
                IS = YJ;
                IT = ABU;
                IU = ABV;
                IV = ABW;
                IW = YV;
                IX = ZA;
                IY = YR;
                IZ = ACL;
                JA = ACB;
                JB = ACG;
                JC = ACT;
            } else {
                EX = DC;
                EY = DD;
                EZ = DE;
                FA = DF;
                FB = DG;
                FC = CQ;
                FD = DH;
                FE = DI;
                FF = BK;
                FG = BI;
                FH = DJ;
                FI = DK;
                FJ = DL;
                FK = DM;
                FL = DN;
                FM = DO;
                FN = DP;
                FO = DQ;
                FP = DR;
                FQ = DS;
                FR = DT;
                FS = DU;
                FT = DV;
                FU = DW;
                FV = BD;
                FW = BE;
                FX = DX;
                FY = DY;
                FZ = DZ;
                GA = EA;
                GB = EB;
                GC = CE;
                GD = CC;
                GE = EC;
                GF = CA;
                GG = CB;
                GH = ED;
                GI = EE;
                GJ = EF;
                GK = EG;
                GL = EH;
                GM = EI;
                GN = EJ;
                GO = EK;
                GP = EL;
                GQ = EM;
                GR = EN;
                GS = EO;
                GT = EP;
                GU = EQ;
                GV = ER;
                GW = ES;
                GX = ET;
                GY = EU;
                GZ = EV;
                HA = EW;
                HB = EW;
                HC = EW;
                HD = EW;
                HE = EW;
                HF = EW;
                HG = EW;
                HH = EW;
                HI = EW;
                HJ = EW;
                HK = EW;
                HL = EW;
                HM = EW;
                HN = EW;
                HO = EW;
                HP = EW;
                HQ = EW;
                HR = EW;
                HS = EW;
                HT = EW;
                HU = EW;
                HV = EW;
                HW = EW;
                HX = EW;
                HY = EW;
                HZ = EW;
                IA = EW;
                IB = EW;
                IC = EW;
                ID = EW;
                IE = EW;
                IF = EW;
                IG = EW;
                IH = EW;
                II = EW;
                IJ = EW;
                IK = EW;
                IL = EW;
                IM = EW;
                IN = EW;
                IO = EW;
                IP = EW;
                IQ = EW;
                IR = EW;
                IS = EW;
                IT = EW;
                IU = EW;
                IV = EW;
                IW = EW;
                IX = EW;
                IY = EW;
                IZ = EW;
                JA = EW;
                JB = EW;
                JC = EW;
            }
            let AEG;
            let AEH;
            if JD != 0.0 {
                let AEA = ADZ * EX;
                let AEB = H / AEA;
                let AEC = (Lanes([0.0, I[0], I[1]]) - Lanes([((HA * ADZ) * AEB), 0.0, 0.0])) / AEA;
                let AEE = if AEB > AED { 1.0 } else { 0.0 };
                let AEK;
                let AEL;
                let AEM;
                let AEN;
                if AEE != 0.0 {
                    let AEJ = BC + (AEB - AED);
                    AEK = AEJ;
                    AEL = AED;
                    AEM = AEC;
                    AEN = AEF;
                } else {
                    AEK = BC;
                    AEL = AEB;
                    AEM = AEF;
                    AEN = AEC;
                }
                let AEO = rspice_limexp(AEL);
                let AEP = (AEK * AEO) - BC;
                let AEQ = EY * AEP;
                let AER = Lanes([(HB * AEP), 0.0, 0.0]) + (((AEM * AEO) + ((AEN * AEO) * AEK)) * EY);
                AEG = AEQ;
                AEH = AER;
            } else {
                AEG = AR;
                AEH = AEF;
            }
            let AEW;
            let AEX;
            if AEI != 0.0 {
                let AES = ME * EX;
                let AET = H / AES;
                let AEU = (Lanes([0.0, I[0], I[1]]) - Lanes([((HA * ME) * AET), 0.0, 0.0])) / AES;
                let AEV = if AET > AED { 1.0 } else { 0.0 };
                let AFJ;
                let AFK;
                let AFL;
                let AFM;
                if AEV != 0.0 {
                    let AFI = BC + (AET - AED);
                    AFJ = AFI;
                    AFK = AED;
                    AFL = AEU;
                    AFM = AEF;
                } else {
                    AFJ = BC;
                    AFK = AET;
                    AFL = AEF;
                    AFM = AEU;
                }
                let AFN = rspice_limexp(AFK);
                let AFO = (AFJ * AFN) - BC;
                let AFP = EZ * AFO;
                let AFQ = Lanes([(HC * AFO), 0.0, 0.0]) + (((AFL * AFN) + ((AFM * AFN) * AFJ)) * EZ);
                AEW = AFP;
                AEX = AFQ;
            } else {
                AEW = AR;
                AEX = AEF;
            }
            let AEY = I * FB;
            let AFA = rspice_limexp(((H * FB) / AEZ));
            let AFB = FA * AFA;
            let AFC = Lanes([(HD * AFA), 0.0, 0.0]) + ((((Lanes([0.0, AEY[0], AEY[1]]) + Lanes([(HE * H), 0.0, 0.0])) / AEZ) * AFA) * FA);
            let AFD = O * FB;
            let AFE = rspice_limexp((N * FB));
            let AFF = FA * AFE;
            let AFG = Lanes([(HD * AFE), 0.0, 0.0]) + (((Lanes([0.0, AFD[0], AFD[1]]) + Lanes([(HE * N), 0.0, 0.0])) * AFE) * FA);
            let AFH = if FC > AR { 1.0 } else { 0.0 };
            let AHA;
            let AHB;
            let AHC;
            let AHD;
            if AFH != 0.0 {
                let AFR = ((-(FE.ln())) / LF).exp();
                let AFS = BC - AFR;
                let AFT = FD * AFS;
                let AFU = AFT - H;
                let AFV = Lanes([((HG * AFS) + ((((((HH * (JV / FE)) * JM) / LF) * AFR) * JM) * FD)), 0.0, 0.0]);
                let AFW = Lanes([0.0, I[0], I[1]]);
                let AFX = AFU * FB;
                let AFY = ((AFV - AFW) * FB) + Lanes([(HE * AFU), 0.0, 0.0]);
                let AFZ = AFY * AFX;
                let AGB = ((AFX * AFX) + AGA).sqrt();
                let AGC = (AFZ + AFZ) * (JV / (KZ * AGB));
                let AGD = (AFX + AGB) * KF;
                let AGE = (AFY + AGC) * KF;
                let AGF = AFT - (EX * AGD);
                let AGG = AFV - (Lanes([(HA * AGD), 0.0, 0.0]) + (AGE * EX));
                let AGH = AGD / AGB;
                let AGI = (AGE - (AGC * AGH)) / AGB;
                let AGJ = AGF / FD;
                let AGK = BC - AGJ;
                let AGL = AGK.ln();
                let AGM = (((AGG - Lanes([(HG * AGJ), 0.0, 0.0])) / FD) * JM) * (JV / AGK);
                let AGN = -LF;
                let AGO = (AGN * AGL).exp();
                let AGP = BC - AGH;
                let AGQ = (AGO * AGH) + (FE * AGP);
                let AGR = FC * AGQ;
                let AGS = Lanes([(HF * AGQ), 0.0, 0.0]) + ((((((AGM * AGN) * AGO) * AGH) + (AGI * AGO)) + (Lanes([(HH * AGP), 0.0, 0.0]) + ((AGI * JM) * FE))) * FC);
                let AGT = BC - LF;
                let AGU = (AGL * AGT).exp();
                let AGV = BC - AGU;
                let AGW = H - AGF;
                let AGX = ((FD * AGV) / AGT) + (FE * AGW);
                let AGY = FC * AGX;
                let AGZ = Lanes([(HF * AGX), 0.0, 0.0]) + ((((Lanes([(HG * AGV), 0.0, 0.0]) + ((((AGM * AGT) * AGU) * JM) * FD)) / AGT) + (Lanes([(HH * AGW), 0.0, 0.0]) + ((AFW - AGG) * FE))) * FC);
                AHA = AGY;
                AHB = AGR;
                AHC = AGZ;
                AHD = AGS;
            } else {
                AHA = AR;
                AHB = AR;
                AHC = AEF;
                AHD = AEF;
            }
            let AHH;
            let AHI;
            let AHJ;
            let AHK;
            if AHE != 0.0 {
                let AHF = if FF > AR { 1.0 } else { 0.0 };
                let AII;
                let AIJ;
                let AIK;
                let AIL;
                if AHF != 0.0 {
                    let AHM = NC / KX;
                    let AHO = AHN - FG;
                    let AHP = HJ * JM;
                    let AHQ = ((-(FH.ln())) / NC).exp();
                    let AHR = BC - AHQ;
                    let AHS = FG * AHR;
                    let AHT = (HJ * AHR) + ((((((HK * (JV / FH)) * JM) / NC) * AHQ) * JM) * FG);
                    let AHU = FH * FF;
                    let AHV = (HK * FF) + (HI * FH);
                    let AHW = AHM - NC;
                    let AHX = AHN / FG;
                    let AHY = (AHW * (AHX.ln())).exp();
                    let AHZ = FF * AHY;
                    let AIA = (HI * AHY) + (((((((HJ * AHX) * JM) / FG) * (JV / AHX)) * AHW) * AHY) * FF);
                    let AIB = AHS - N;
                    let AIC = Lanes([AHT, 0.0, 0.0]);
                    let AID = Lanes([0.0, O[0], O[1]]);
                    let AIE = AIB * FB;
                    let AIF = ((AIC - AID) * FB) + Lanes([(HE * AIB), 0.0, 0.0]);
                    let AIG = if AIE < AED { 1.0 } else { 0.0 };
                    let AIU;
                    let AIV;
                    let AIW;
                    let AIX;
                    if AIG != 0.0 {
                        let AIM = AIE.exp();
                        let AIN = AIF * AIM;
                        let AIO = BC + AIM;
                        let AIP = AIM / AIO;
                        let AIQ = (AIN - (AIN * AIP)) / AIO;
                        let AIR = AIO.ln();
                        let AIS = AHS - (EX * AIR);
                        let AIT = AIC - (Lanes([(HA * AIR), 0.0, 0.0]) + ((AIN * (JV / AIO)) * EX));
                        AIU = AIS;
                        AIV = AIP;
                        AIW = AIT;
                        AIX = AIQ;
                    } else {
                        AIU = N;
                        AIV = BC;
                        AIW = AID;
                        AIX = AIH;
                    }
                    let AIZ = (AIY * AHO) + (KX * EX);
                    let AJA = (AHP * AIY) + (HA * KX);
                    let AJB = (AHO + AIU) / AIZ;
                    let AJC = ((Lanes([AHP, 0.0, 0.0]) + AIW) - Lanes([(AJA * AJB), 0.0, 0.0])) / AIZ;
                    let AJD = if AJB < AED { 1.0 } else { 0.0 };
                    let AJO;
                    let AJP;
                    let AJQ;
                    let AJR;
                    if AJD != 0.0 {
                        let AJE = AJB.exp();
                        let AJF = AJC * AJE;
                        let AJG = BC + AJE;
                        let AJH = AJE / AJG;
                        let AJI = (AJF - (AJF * AJH)) / AJG;
                        let AJJ = (-(AHO + AHS)) / AIZ;
                        let AJK = AJJ.exp();
                        let AJL = (AJG.ln()) - AJK;
                        let AJM = (-AHO) + (AIZ * AJL);
                        let AJN = Lanes([(AHP * JM), 0.0, 0.0]) + (Lanes([(AJA * AJL), 0.0, 0.0]) + (((AJF * (JV / AJG)) - Lanes([(((((AHP + AHT) * JM) - (AJA * AJJ)) / AIZ) * AJK), 0.0, 0.0])) * AIZ));
                        AJO = AJM;
                        AJP = AJH;
                        AJQ = AJN;
                        AJR = AJI;
                    } else {
                        AJO = AIU;
                        AJP = BC;
                        AJQ = AIW;
                        AJR = AIH;
                    }
                    let AJS = N - AIU;
                    let AJT = AIU / FG;
                    let AJU = BC - AJT;
                    let AJV = AJU.ln();
                    let AJW = (((AIW - Lanes([(HJ * AJT), 0.0, 0.0])) / FG) * JM) * (JV / AJU);
                    let AJX = AJO / FG;
                    let AJY = BC - AJX;
                    let AJZ = AJY.ln();
                    let AKA = (((AJQ - Lanes([(HJ * AJX), 0.0, 0.0])) / FG) * JM) * (JV / AJY);
                    let AKB = BC - NC;
                    let AKC = BC - AHM;
                    let AKD = -NC;
                    let AKE = (AJZ * AKD).exp();
                    let AKF = FF * AKE;
                    let AKG = AKF * AIV;
                    let AKH = -AHM;
                    let AKI = (AJV * AKH).exp();
                    let AKJ = AHZ * AKI;
                    let AKK = BC - AJP;
                    let AKL = BC - AIV;
                    let AKM = ((AKG * AJP) + (AKJ * AKK)) + (AHU * AKL);
                    let AKN = ((((((Lanes([(HI * AKE), 0.0, 0.0]) + (((AKA * AKD) * AKE) * FF)) * AIV) + (AIX * AKF)) * AJP) + (AJR * AKG)) + (((Lanes([(AIA * AKI), 0.0, 0.0]) + (((AJW * AKH) * AKI) * AHZ)) * AKK) + ((AJR * JM) * AKJ))) + (Lanes([(AHV * AKL), 0.0, 0.0]) + ((AIX * JM) * AHU));
                    let AKO = (AJZ * AKB).exp();
                    let AKP = BC - AKO;
                    let AKQ = (AJV * AKC).exp();
                    let AKR = BC - AKQ;
                    let AKS = (AJZ * AKC).exp();
                    let AKT = BC - AKS;
                    let AKU = (((FF * AKP) / AKB) + ((AHZ * AKR) / AKC)) - ((AHZ * AKT) / AKC);
                    let AKV = (AKU * FG) + (AHU * AJS);
                    let AKW = ((((((Lanes([(HI * AKP), 0.0, 0.0]) + ((((AKA * AKB) * AKO) * JM) * FF)) / AKB) + ((Lanes([(AIA * AKR), 0.0, 0.0]) + ((((AJW * AKC) * AKQ) * JM) * AHZ)) / AKC)) - ((Lanes([(AIA * AKT), 0.0, 0.0]) + ((((AKA * AKC) * AKS) * JM) * AHZ)) / AKC)) * FG) + Lanes([(HJ * AKU), 0.0, 0.0])) + (Lanes([(AHV * AJS), 0.0, 0.0]) + ((AID - AIW) * AHU));
                    AII = AKV;
                    AIJ = AKM;
                    AIK = AKW;
                    AIL = AKN;
                } else {
                    AII = AR;
                    AIJ = AR;
                    AIK = AIH;
                    AIL = AIH;
                }
                AHH = AII;
                AHI = AIJ;
                AHJ = AIK;
                AHK = AIL;
            } else {
                let AHG = if FF > AR { 1.0 } else { 0.0 };
                let AMF;
                let AMG;
                let AMH;
                let AMI;
                if AHG != 0.0 {
                    let AKX = ((-(FH.ln())) / NC).exp();
                    let AKY = BC - AKX;
                    let AKZ = FG * AKY;
                    let ALA = AKZ - N;
                    let ALB = Lanes([((HJ * AKY) + ((((((HK * (JV / FH)) * JM) / NC) * AKX) * JM) * FG)), 0.0, 0.0]);
                    let ALC = Lanes([0.0, O[0], O[1]]);
                    let ALD = ALA * FB;
                    let ALE = ((ALB - ALC) * FB) + Lanes([(HE * ALA), 0.0, 0.0]);
                    let ALF = ALE * ALD;
                    let ALG = ((ALD * ALD) + AGA).sqrt();
                    let ALH = (ALF + ALF) * (JV / (KZ * ALG));
                    let ALI = (ALD + ALG) * KF;
                    let ALJ = (ALE + ALH) * KF;
                    let ALK = AKZ - (EX * ALI);
                    let ALL = ALB - (Lanes([(HA * ALI), 0.0, 0.0]) + (ALJ * EX));
                    let ALM = ALI / ALG;
                    let ALN = (ALJ - (ALH * ALM)) / ALG;
                    let ALO = ALK / FG;
                    let ALP = BC - ALO;
                    let ALQ = ALP.ln();
                    let ALR = (((ALL - Lanes([(HJ * ALO), 0.0, 0.0])) / FG) * JM) * (JV / ALP);
                    let ALS = -NC;
                    let ALT = (ALS * ALQ).exp();
                    let ALU = BC - ALM;
                    let ALV = (ALT * ALM) + (FH * ALU);
                    let ALW = FF * ALV;
                    let ALX = Lanes([(HI * ALV), 0.0, 0.0]) + ((((((ALR * ALS) * ALT) * ALM) + (ALN * ALT)) + (Lanes([(HK * ALU), 0.0, 0.0]) + ((ALN * JM) * FH))) * FF);
                    let ALY = BC - NC;
                    let ALZ = (ALQ * ALY).exp();
                    let AMA = BC - ALZ;
                    let AMB = N - ALK;
                    let AMC = ((FG * AMA) / ALY) + (FH * AMB);
                    let AMD = FF * AMC;
                    let AME = Lanes([(HI * AMC), 0.0, 0.0]) + ((((Lanes([(HJ * AMA), 0.0, 0.0]) + ((((ALR * ALY) * ALZ) * JM) * FG)) / ALY) + (Lanes([(HK * AMB), 0.0, 0.0]) + ((ALC - ALL) * FH))) * FF);
                    AMF = AMD;
                    AMG = ALW;
                    AMH = AME;
                    AMI = ALX;
                } else {
                    AMF = AR;
                    AMG = AR;
                    AMH = AIH;
                    AMI = AIH;
                }
                AHH = AMF;
                AHI = AMG;
                AHJ = AMH;
                AHK = AMI;
            }
            let ANB;
            let ANC;
            if AHL != 0.0 {
                let AMK = AMJ * EX;
                let AML = HA * AMJ;
                let AMM = Lanes([HG, 0.0, 0.0]);
                let AMN = (FD - H) / AMK;
                let AMO = ((AMM - Lanes([0.0, I[0], I[1]])) - Lanes([(AML * AMN), 0.0, 0.0])) / AMK;
                let AMP = AMO * AMN;
                let AMQ = ((AMN * AMN) + AGA).sqrt();
                let AMR = AMN + AMQ;
                let AMS = (FD - ((AMK * AMR) * KF)) / FD;
                let AMT = BC - AMS;
                let AMU = (LF * (AMT.ln())).exp();
                let AMV = BC - AMU;
                let AMW = FI * AMV;
                let AMX = Lanes([(HL * AMV), 0.0, 0.0]) + (((((((((AMM - ((Lanes([(AML * AMR), 0.0, 0.0]) + ((AMO + ((AMP + AMP) * (JV / (KZ * AMQ)))) * AMK)) * KF)) - Lanes([(HG * AMS), 0.0, 0.0])) / FD) * JM) * (JV / AMT)) * LF) * AMU) * JM) * FI);
                let AMZ = if (AMW.abs()) > AMY { 1.0 } else { 0.0 };
                let AOX;
                let AOY;
                if AMZ != 0.0 {
                    let AOQ = AMW.exp();
                    let AOR = AOQ - BC;
                    let AOS = (FJ * AOR) / AMW;
                    let AOT = ((Lanes([(HM * AOR), 0.0, 0.0]) + ((AMX * AOQ) * FJ)) - (AMX * AOS)) / AMW;
                    AOX = AOS;
                    AOY = AOT;
                } else {
                    let AOU = BC + (AMW * KF);
                    let AOV = FJ * AOU;
                    let AOW = Lanes([(HM * AOU), 0.0, 0.0]) + ((AMX * KF) * FJ);
                    AOX = AOV;
                    AOY = AOW;
                }
                ANB = AOX;
                ANC = AOY;
            } else {
                let ANA = Lanes([HM, 0.0, 0.0]);
                ANB = FJ;
                ANC = ANA;
            }
            let AND = Lanes([HN, 0.0, 0.0]) + ((ANC * AHA) + (AHC * ANB));
            let ANF = AHJ * ANE;
            let ANH = ANG * FK;
            let ANI = HN * ANG;
            let ANJ = ((FK + (ANB * AHA)) + (ANE * AHH)) / ANH;
            let ANK = ((Lanes([AND[0], 0.0, AND[1], AND[2]]) + Lanes([ANF[0], ANF[1], 0.0, ANF[2]])) - Lanes([(ANI * ANJ), 0.0, 0.0, 0.0])) / ANH;
            let ANL = ANJ - BC;
            let ANM = ANK * ANL;
            let ANN = ((ANL * ANL) + AGA).sqrt();
            let ANO = BC + ((ANL + ANN) * KF);
            let ANP = ANH * ANO;
            let ANQ = Lanes([(ANI * ANO), 0.0, 0.0, 0.0]) + (((ANK + ((ANM + ANM) * (JV / (KZ * ANN)))) * KF) * ANH);
            let ANS = FG * ANR;
            let ANT = ANS - N;
            let ANU = Lanes([(HJ * ANR), 0.0, 0.0]);
            let ANV = Lanes([0.0, O[0], O[1]]);
            let ANW = ANT * FB;
            let ANX = ((ANU - ANV) * FB) + Lanes([(HE * ANT), 0.0, 0.0]);
            let ANY = ANX * ANW;
            let ANZ = ((ANW * ANW) + AGA).sqrt();
            let AOA = (ANY + ANY) * (JV / (KZ * ANZ));
            let AOB = (ANW + ANZ) * KF;
            let AOC = (ANX + AOA) * KF;
            let AOD = AOB / ANZ;
            let AOE = (AOC - (AOA * AOD)) / ANZ;
            let AOF = (ANS - (EX * AOB)) / FG;
            let AOG = BC - AOF;
            let AOI = (AOH * (AOG.ln())).exp();
            let AOJ = (AOI * AOD) + (NT * (BC - AOD));
            let AOK = (((((((((ANU - (Lanes([(HA * AOB), 0.0, 0.0]) + (AOC * EX))) - Lanes([(HJ * AOF), 0.0, 0.0])) / FG) * JM) * (JV / AOG)) * AOH) * AOI) * AOD) + (AOE * AOI)) + ((AOE * JM) * NT);
            let AOL = BC / AOJ;
            let AOO = (FL + (AOM * (AOL - BC))) + (AON * (AOJ - BC));
            let AOP = (Lanes([HO, 0.0, 0.0]) + ((((AOK * AOL) * JM) / AOJ) * AOM)) + (AOK * AON);
            let APE;
            let APF;
            if AX != 0.0 {
                let AOZ = FM - N;
                let APA = Lanes([HP, 0.0, 0.0]) - ANV;
                let APB = Lanes([APA[0], APA[1], 0.0, APA[2]]);
                APE = AOZ;
                APF = APB;
            } else {
                let APC = P - FN;
                let APD = Lanes([0.0, Q[0], Q[1], Q[2]]) - Lanes([HQ, 0.0, 0.0, 0.0]);
                APE = APC;
                APF = APD;
            }
            let APW;
            let APX;
            if AT != 0.0 {
                let APG = APE - EX;
                let APH = Lanes([HA, 0.0, 0.0, 0.0]);
                let API = APG * FB;
                let APJ = ((APF - APH) * FB) + Lanes([(HE * APG), 0.0, 0.0, 0.0]);
                let APK = APJ * API;
                let APL = ((API * API) + AGA).sqrt();
                let APM = (API + APL) * KF;
                let APN = EX + (EX * APM);
                let APO = APH + (Lanes([(HA * APM), 0.0, 0.0, 0.0]) + (((APJ + ((APK + APK) * (JV / (KZ * APL)))) * KF) * EX));
                APW = APN;
                APX = APO;
            } else {
                let APQ = APE / APP;
                let APR = APF / APP;
                let APS = APR * APQ;
                let APT = ((APQ * APQ) + parameters[80]).sqrt();
                let APU = APP * ((APQ + APT) * KF);
                let APV = ((APR + ((APS + APS) * (JV / (KZ * APT)))) * KF) * APP;
                APW = APU;
                APX = APV;
            }
            let APY = APW / FO;
            let AQA = (APZ * (APY.ln())).exp();
            let AQB = BC + AQA;
            let AQC = ((AQB.ln()) / APZ).exp();
            let AQD = (APW * FP) / AQC;
            let AQF = (APW - FO) / AQE;
            let AQG = (APX - Lanes([HR, 0.0, 0.0, 0.0])) / AQE;
            let AQH = AQG * AQF;
            let AQI = ((AQF * AQF) + parameters[81]).sqrt();
            let AQJ = BC + (KF * (AQF + AQI));
            let AQK = AQD * AQJ;
            let AQL = (((((APX * FP) + Lanes([(HS * APW), 0.0, 0.0, 0.0])) - (((((((((APX - Lanes([(HR * APY), 0.0, 0.0, 0.0])) / FO) * (JV / APY)) * APZ) * AQA) * (JV / AQB)) / APZ) * AQC) * AQD)) / AQC) * AQJ) + (((AQG + ((AQH + AQH) * (JV / (KZ * AQI)))) * KF) * AQD);
            let AQM = if (if AOO > AR { 1.0 } else { 0.0 }) != 0.0 || staged[62] != 0.0 { 1.0 } else { 0.0 };
            let AQP;
            let AQQ;
            if AQM != 0.0 {
                let AQN = KF * ANP;
                let AQO = ANQ * KF;
                let ARQ;
                let ARR;
                if AT != 0.0 {
                    let ARB = AQO * AQN;
                    let ARC = AOP * AFB;
                    let ARD = AFC * AOO;
                    let ARF = AFG * ARE;
                    let ARG = (((AQN * AQN) + (AOO * AFB)) + (ARE * AFF)).sqrt();
                    let ARH = AQN + ARG;
                    let ARI = AQO + ((((ARB + ARB) + (Lanes([ARC[0], ARC[1], 0.0, ARC[2]]) + Lanes([ARD[0], 0.0, ARD[1], ARD[2]]))) + Lanes([ARF[0], ARF[1], 0.0, ARF[2]])) * (JV / (KZ * ARG)));
                    ARQ = ARH;
                    ARR = ARI;
                } else {
                    let ARJ = AQO * AQN;
                    let ARK = FQ * FL;
                    let ARL = Lanes([(((HT * FL) + (HO * FQ)) * AFB), 0.0, 0.0]) + (AFC * ARK);
                    let ARM = AFG * ARE;
                    let ARN = (((AQN * AQN) + (ARK * AFB)) + (ARE * AFF)).sqrt();
                    let ARO = AQN + ARN;
                    let ARP = AQO + ((((ARJ + ARJ) + Lanes([ARL[0], 0.0, ARL[1], ARL[2]])) + Lanes([ARM[0], ARM[1], 0.0, ARM[2]])) * (JV / (KZ * ARN)));
                    ARQ = ARO;
                    ARR = ARP;
                }
                AQP = ARQ;
                AQQ = ARR;
            } else {
                AQP = ANP;
                AQQ = ANQ;
            }
            let AQR = AFB / AQP;
            let AQS = Lanes([AFC[0], 0.0, AFC[1], AFC[2]]);
            let AQT = (AQS - (AQQ * AQR)) / AQP;
            let AQU = AFF / AQP;
            let AQV = Lanes([AFG[0], AFG[1], 0.0, AFG[2]]);
            let AQW = (AQV - (AQQ * AQU)) / AQP;
            let AQX = AOO * AQR;
            let AQY = AOP * AQR;
            let AQZ = Lanes([AQY[0], AQY[1], 0.0, AQY[2]]) + (AQT * AOO);
            let ARX;
            let ARY;
            if ARA != 0.0 {
                let ARS = FQ * FL;
                let ART = ARS * AQR;
                let ARU = Lanes([(((HT * FL) + (HO * FQ)) * AQR), 0.0, 0.0, 0.0]) + (AQT * ARS);
                ARX = ART;
                ARY = ARU;
            } else {
                let ARV = FQ * AQX;
                let ARW = Lanes([(HT * AQX), 0.0, 0.0, 0.0]) + (AQZ * FQ);
                ARX = ARV;
                ARY = ARW;
            }
            let ASA = ARZ * AQK;
            let ASC = if (if AQR >= ASA { 1.0 } else { 0.0 }) != 0.0 || ASB != 0.0 { 1.0 } else { 0.0 };
            let ASR;
            let ASS;
            let AST;
            let ASU;
            let ASV;
            let ASW;
            let ASX;
            let ASY;
            if ASC != 0.0 {
                let ASD = AQR / AQK;
                let ASE = (AQT - (AQL * ASD)) / AQK;
                let ASG = (ASF * (ASD.ln())).exp();
                let ASI = ASH * ASG;
                let ASJ = (((ASE * (JV / ASD)) * ASF) * ASG) * ASH;
                let ASK = BC + ASF;
                let ASL = (ASI * AQR) / ASK;
                let ASM = ((ASJ * AQR) + (AQT * ASI)) / ASK;
                let ASO = if ASN < (ANG * (QE / QJ)) { 1.0 } else { 0.0 };
                let ATG;
                let ATH;
                let ATI;
                let ATJ;
                if ASO != 0.0 {
                    ATG = AR;
                    ATH = AR;
                    ATI = ASQ;
                    ATJ = ASQ;
                } else {
                    let ATD = (AQR - AQK) / ASN;
                    let ATE = (AQT - AQL) / ASN;
                    let ATF = if ATD < -1e10f64 { 1.0 } else { 0.0 };
                    let AVD;
                    let AVE;
                    if ATF != 0.0 {
                        AVD = AVC;
                        AVE = ASQ;
                    } else {
                        AVD = ATD;
                        AVE = ATE;
                    }
                    let AVF = AVE * AVD;
                    let AVH = ((AVD * AVD) + AVG).sqrt();
                    let AVI = (AVF + AVF) * (JV / (KZ * AVH));
                    let AVJ = AVD + AVH;
                    let AVK = AVE + AVI;
                    let AVL = -2e0f64 / AVJ;
                    let AVM = AVL.exp();
                    let AVN = AUJ * AVM;
                    let AVO = ((((AVK * AVL) * JM) / AVJ) * AVM) * AUJ;
                    let AVP = ASN * AVH;
                    let AVQ = AVP * AVJ;
                    let AVR = (KT * AVN) / AVQ;
                    let AVS = ((AVO * KT) - ((((AVI * ASN) * AVJ) + (AVK * AVP)) * AVR)) / AVQ;
                    ATG = AVN;
                    ATH = AVR;
                    ATI = AVO;
                    ATJ = AVS;
                }
                let ATL = BC - ATK;
                let ATM = ATL * FR;
                let ATN = HU * ATL;
                let ATO = ATI * FB;
                let ATP = (ATG * FB).exp();
                let ATQ = (ATO + Lanes([(HE * ATG), 0.0, 0.0, 0.0])) * ATP;
                let ATR = ATP - BC;
                let ATS = ATM * ATR;
                let ATT = Lanes([(ATN * ATR), 0.0, 0.0, 0.0]) + (ATQ * ATM);
                let ATU = ATM * AQR;
                let ATV = ATU * ATP;
                let ATW = ATV * FB;
                let ATX = ATS + (ATW * ATH);
                let ATY = ATT + (((((((Lanes([(ATN * AQR), 0.0, 0.0, 0.0]) + (AQT * ATM)) * ATP) + (ATQ * ATU)) * FB) + Lanes([(HE * ATV), 0.0, 0.0, 0.0])) * ATH) + (ATJ * ATW));
                let ATZ = BC / ASD;
                let AUA = BC - ATZ;
                let AUB = (((ASE * ATZ) * JM) / ASD) * JM;
                let AUC = AUB * AUA;
                let AUE = ((AUA * AUA) + AUD).sqrt();
                let AUF = (AUC + AUC) * (JV / (KZ * AUE));
                let AUG = BC + ((BC + AUD).sqrt());
                let AUH = (AUA + AUE) / AUG;
                let AUI = (AUB + AUF) / AUG;
                let AUK = ATG - AUJ;
                let AUL = (AUK * FB).exp();
                let AUM = (ATO + Lanes([(HE * AUK), 0.0, 0.0, 0.0])) * AUL;
                let AUN = FR * AUH;
                let AUO = AUN * AUH;
                let AUP = AUO * AUL;
                let AUQ = ((((Lanes([(HU * AUH), 0.0, 0.0, 0.0]) + (AUI * FR)) * AUH) + (AUI * AUN)) * AUL) + (AUM * AUO);
                let AUR = ASD * AUE;
                let AUS = KT / AUR;
                let AUT = FB * AQR;
                let AUU = (BC + AUS) + (AUT * ATH);
                let AUV = AUP * AUU;
                let AUW = (AUQ * AUU) + (((((((ASE * AUE) + (AUF * ASD)) * AUS) * JM) / AUR) + (((Lanes([(HE * AQR), 0.0, 0.0, 0.0]) + (AQT * FB)) * ATH) + (ATJ * AUT))) * AUP);
                let AVB = if (if (if (if AUX < AUY { 1.0 } else { 0.0 }) != 0.0 && (if AUZ < AUY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (AUH * AUX) < AVA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (AUH * AUZ) < AVA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AWH;
                let AWI;
                let AWJ;
                let AWK;
                if AVB != 0.0 {
                    let AVT = ATK * AUP;
                    let AVU = AVT * AQR;
                    let AVV = ((AUQ * ATK) * AQR) + (AQT * AVT);
                    let AVW = ATK * AUV;
                    let AVX = AUW * ATK;
                    AWH = AVU;
                    AWI = AVW;
                    AWJ = AVV;
                    AWK = AVX;
                } else {
                    let AVY = BC - AUH;
                    let AVZ = AUI * JM;
                    let AWA = AVY - BC;
                    let AWB = BC - AUA;
                    let AWC = AUE * AQR;
                    let AWD = (AWA * AWB) / AWC;
                    let AWE = (((AVZ * AWB) + ((AUB * JM) * AWA)) - (((AUF * AQR) + (AQT * AUE)) * AWD)) / AWC;
                    let AWG = if (AWF.abs()) > AMY { 1.0 } else { 0.0 };
                    let AXR;
                    let AXS;
                    let AXT;
                    let AXU;
                    if AWG != 0.0 {
                        let AWR = (AWA * AWQ).exp();
                        let AWS = (AVZ * AWQ) * AWR;
                        let AWU = if AWT < AUY { 1.0 } else { 0.0 };
                        let BAG;
                        let BAH;
                        let BAI;
                        let BAJ;
                        if AWU != 0.0 {
                            let AYH = AWR * AYG;
                            let AYI = AWS * AYG;
                            let AYJ = (BC - AWR) / AYH;
                            let AYK = ((AWS * JM) - (AYI * AYJ)) / AYH;
                            let AYL = AYG * AYJ;
                            let AYM = AYK * AYG;
                            let AYN = BC + AYL;
                            let AYP = AYO * AYG;
                            let AYQ = KF + (AYP * AYJ);
                            let AYR = ((KT * ((AYL * AYQ) - (KF * (AYN.ln())))) / AYG) / AYG;
                            let AYS = (((((AYM * AYQ) + ((AYK * AYP) * AYL)) - ((AYM * (JV / AYN)) * KF)) * KT) / AYG) / AYG;
                            let AYT = -AWQ;
                            let AYU = (AYT * AWD) / AYH;
                            let AYV = BC + AYN;
                            let AYW = AYV * AYJ;
                            let AYX = (AYW * AYU) / AYN;
                            let AYY = (((((AYM * AYJ) + (AYK * AYV)) * AYU) + ((((AWE * AYT) - (AYI * AYU)) / AYH) * AYW)) - (AYM * AYX)) / AYN;
                            BAG = AYR;
                            BAH = AYX;
                            BAI = AYS;
                            BAJ = AYY;
                        } else {
                            let AYZ = AUZ - (AWR * AUX);
                            let AZA = (AWS * AUX) * JM;
                            let AZB = (AWR - BC) / AYZ;
                            let AZC = (AWS - (AZA * AZB)) / AYZ;
                            let AZD = AZC * AUZ;
                            let AZE = BC + (AUZ * AZB);
                            let AZG = AXD * AZF;
                            let AZH = KF - AZG;
                            let AZI = AXD * AZB;
                            let AZJ = AZC * AXD;
                            let AZK = AZG + AZI;
                            let AZL = AZH / AZE;
                            let AZM = AZC * AUX;
                            let AZN = BC + (AUX * AZB);
                            let AZQ = AZO * AZP;
                            let AZR = KF - AZQ;
                            let AZS = AZO * AZB;
                            let AZT = AZC * AZO;
                            let AZU = AZQ + AZS;
                            let AZV = AZR / AZN;
                            let AZW = (((((AZE.ln()) * AZH) * AZF) + (AZK * AZB)) - ((((AZN.ln()) * AZR) * AZP) + (AZU * AZB))) / AWF;
                            let AZX = (((((AZD * (JV / AZE)) * AZH) * AZF) + ((AZJ * AZB) + (AZC * AZK))) - ((((AZM * (JV / AZN)) * AZR) * AZP) + ((AZT * AZB) + (AZC * AZU)))) / AWF;
                            let AZY = AYZ * AYZ;
                            let AZZ = AZA * AYZ;
                            let BAA = (-2e0f64 * AWF) / AZY;
                            let BAB = (BAA * AWR) * AWQ;
                            let BAC = BAB * AWD;
                            let BAD = ((AZL + AZG) + (AZI * KT)) - ((AZV + AZQ) + (AZS * KT));
                            let BAE = (BAD * BAC) / AWF;
                            let BAF = (((((((AZD * AZL) * JM) / AZE) + (AZJ * KT)) - ((((AZM * AZV) * JM) / AZN) + (AZT * KT))) * BAC) + ((((((((((AZZ + AZZ) * BAA) * JM) / AZY) * AWR) + (AWS * BAA)) * AWQ) * AWD) + (AWE * BAB)) * BAD)) / AWF;
                            BAG = AZW;
                            BAH = BAE;
                            BAI = AZX;
                            BAJ = BAF;
                        }
                        AXR = BAG;
                        AXS = BAH;
                        AXT = BAI;
                        AXU = BAJ;
                    } else {
                        let AWV = AVZ * AUX;
                        let AWW = BC + (AVY * AUX);
                        let AWX = (BC - AVY) / AWW;
                        let AWY = ((AVZ * JM) - (AWV * AWX)) / AWW;
                        let AWZ = AWY * AUX;
                        let AXA = BC + (AUX * AWX);
                        let AXB = AWX * AWX;
                        let AXC = AWY * AWX;
                        let AXE = AXD * KT;
                        let AXF = BC + (AXE * AWX);
                        let AXG = (AXB * AXF) / AXA;
                        let AXH = ((((AXC + AXC) * AXF) + ((AWY * AXE) * AXB)) - (AWZ * AXG)) / AXA;
                        let AXI = -AWD;
                        let AXJ = (AXI * AXA) / AWW;
                        let AXK = AXA * AXA;
                        let AXL = AWZ * AXA;
                        let AXM = BC / AXK;
                        let AXN = BC + AXM;
                        let AXO = AWX * AXN;
                        let AXP = AXO * AXJ;
                        let AXQ = (((AWY * AXN) + (((((AXL + AXL) * AXM) * JM) / AXK) * AWX)) * AXJ) + ((((((AWE * JM) * AXA) + (AWZ * AXI)) - (AWV * AXJ)) / AWW) * AXO);
                        AXR = AXG;
                        AXS = AXP;
                        AXT = AXH;
                        AXU = AXQ;
                    }
                    let AXV = ATK * FR;
                    let AXW = AXV * AUL;
                    let AXX = Lanes([((HU * ATK) * AUL), 0.0, 0.0, 0.0]) + (AUM * AXV);
                    let AXY = AXW * AXR;
                    let AXZ = (AXX * AXR) + (AXT * AXW);
                    let AYA = AXY * AQR;
                    let AYB = (AXZ * AQR) + (AQT * AXY);
                    let AYC = AYA * ATH;
                    let AYD = AXW * AQR;
                    let AYE = (AXY + (AYC * FB)) + (AYD * AXS);
                    let AYF = (AXZ + ((((AYB * ATH) + (ATJ * AYA)) * FB) + Lanes([(HE * AYC), 0.0, 0.0, 0.0]))) + ((((AXX * AQR) + (AQT * AXW)) * AXS) + (AXU * AYD));
                    AWH = AYA;
                    AWI = AYE;
                    AWJ = AYB;
                    AWK = AYF;
                }
                let AWL = ATL * AUP;
                let AWM = ATL * AUV;
                let AWN = AUW * ATL;
                let AWO = (ATS * AQR) + (AWL * AQR);
                let AWP = ((ATT * AQR) + (AQT * ATS)) + (((AUQ * ATL) * AQR) + (AQT * AWL));
                let BAX;
                let BAY;
                let BAZ;
                let BBA;
                let BBB;
                let BBC;
                if ARA != 0.0 {
                    let BAK = ((AQX + AWO) + ASL) + AWH;
                    let BAL = ((AQZ + AWP) + ASM) + AWJ;
                    let BAM = ((AOO + (ATX + AWM)) + ASI) + AWI;
                    let BAN = ((Lanes([AOP[0], AOP[1], 0.0, AOP[2]]) + (ATY + AWN)) + ASJ) + AWK;
                    let BAP = ((ARX + (BAO * AWO)) + (FS * ASL)) + (FT * AWH);
                    let BAQ = ((ARY + (AWP * BAO)) + (Lanes([(HV * ASL), 0.0, 0.0, 0.0]) + (ASM * FS))) + (Lanes([(HW * AWH), 0.0, 0.0, 0.0]) + (AWJ * FT));
                    BAX = BAP;
                    BAY = BAK;
                    BAZ = BAM;
                    BBA = BAQ;
                    BBB = BAL;
                    BBC = BAN;
                } else {
                    let BAR = (((FQ * AQX) + AWO) + (FS * ASL)) + (FT * AWH);
                    let BAS = (((Lanes([(HT * AQX), 0.0, 0.0, 0.0]) + (AQZ * FQ)) + AWP) + (Lanes([(HV * ASL), 0.0, 0.0, 0.0]) + (ASM * FS))) + (Lanes([(HW * AWH), 0.0, 0.0, 0.0]) + (AWJ * FT));
                    let BAT = ((AQX + AWO) + ASL) + AWH;
                    let BAU = ((AQZ + AWP) + ASM) + AWJ;
                    let BAV = ((AOO + (ATX + AWM)) + ASI) + AWI;
                    let BAW = ((Lanes([AOP[0], AOP[1], 0.0, AOP[2]]) + (ATY + AWN)) + ASJ) + AWK;
                    BAX = BAR;
                    BAY = BAT;
                    BAZ = BAV;
                    BBA = BAS;
                    BBB = BAU;
                    BBC = BAW;
                }
                ASR = BAX;
                ASS = BAY;
                AST = AWO;
                ASU = BAZ;
                ASV = BBA;
                ASW = BBB;
                ASX = AWP;
                ASY = BBC;
            } else {
                let ASP = Lanes([AOP[0], AOP[1], 0.0, AOP[2]]);
                ASR = ARX;
                ASS = AQX;
                AST = AR;
                ASU = AOO;
                ASV = ARY;
                ASW = AQZ;
                ASX = ASQ;
                ASY = ASP;
            }
            let ASZ = ARE * AQU;
            let ATA = AQW * ARE;
            let ATC = if (if ARA != 0.0 && (if ASR > ((ctx.simparam_or("reltol", ATB)) * AQP) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if AT != 0.0 && (if ASS > ((ctx.simparam_or("reltol", ATB)) * AQP) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BBH;
            let BBI;
            let BBJ;
            let BBK;
            let BBL;
            let BBM;
            let BBN;
            let BBO;
            let BBP;
            let BBQ;
            let BBR;
            let BBS;
            if ATC != 0.0 {
                let BBD = (AQX * ASR).sqrt();
                let BBF = (ANP + BBD) + (BBE * ASZ);
                let BBG = (ANQ + (((AQZ * ASR) + (ASV * AQX)) * (JV / (KZ * BBD)))) + (ATA * BBE);
                let mut BCH = 0.0;
                let mut BCI = 0.0;
                let mut BCJ = 0.0;
                let mut BCK = Lanes([0.0; 4]);
                BCH = BBF;
                BCI = BBF;
                BCJ = AR;
                BCK = BBG;
                loop {
                    let BCL = if (if (BCH.abs()) >= ((ctx.simparam_or("reltol", ATB)) * (BCI.abs())) { 1.0 } else { 0.0 }) != 0.0 && (if BCJ <= 1e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if BCL == 0.0 {
                        break;
                    }
                    let BCM = AFB / BCI;
                    let BCN = (AQS - (BCK * BCM)) / BCI;
                    let BCO = AFF / BCI;
                    let BCP = (AQV - (BCK * BCO)) / BCI;
                    let BCQ = AOO * BCM;
                    let BCR = AOP * BCM;
                    let BCS = Lanes([BCR[0], BCR[1], 0.0, BCR[2]]) + (BCN * AOO);
                    let BDJ;
                    let BDK;
                    let BDL;
                    let BDM;
                    if ARA != 0.0 {
                        let BDA = FQ * FL;
                        let BDB = (HT * FL) + (HO * FQ);
                        let BDC = BDA * BCM;
                        let BDD = Lanes([(BDB * BCM), 0.0, 0.0, 0.0]) + (BCN * BDA);
                        let BDE = Lanes([BDB, 0.0, 0.0]);
                        BDJ = BDC;
                        BDK = BDA;
                        BDL = BDD;
                        BDM = BDE;
                    } else {
                        let BDF = FQ * BCQ;
                        let BDG = Lanes([(HT * BCQ), 0.0, 0.0, 0.0]) + (BCS * FQ);
                        let BDH = FQ * AOO;
                        let BDI = Lanes([(HT * AOO), 0.0, 0.0]) + (AOP * FQ);
                        BDJ = BDF;
                        BDK = BDH;
                        BDL = BDG;
                        BDM = BDI;
                    }
                    let BDN = if (if BCM >= ASA { 1.0 } else { 0.0 }) != 0.0 || ASB != 0.0 { 1.0 } else { 0.0 };
                    let BDY;
                    let BDZ;
                    let BEA;
                    let BEB;
                    if BDN != 0.0 {
                        let BDO = BCM / AQK;
                        let BDP = (BCN - (AQL * BDO)) / AQK;
                        let BDQ = (ASF * (BDO.ln())).exp();
                        let BDR = ASH * BDQ;
                        let BDS = (((BDP * (JV / BDO)) * ASF) * BDQ) * ASH;
                        let BDT = BC + ASF;
                        let BDU = (BDR * BCM) / BDT;
                        let BDV = ((BDS * BCM) + (BCN * BDR)) / BDT;
                        let BDW = if ASN < (ANG * (QE / QJ)) { 1.0 } else { 0.0 };
                        let BER;
                        let BES;
                        let BET;
                        let BEU;
                        if BDW != 0.0 {
                            BER = AR;
                            BES = AR;
                            BET = ASQ;
                            BEU = ASQ;
                        } else {
                            let BEO = (BCM - AQK) / ASN;
                            let BEP = (BCN - AQL) / ASN;
                            let BEQ = if BEO < -1e10f64 { 1.0 } else { 0.0 };
                            let BGH;
                            let BGI;
                            if BEQ != 0.0 {
                                BGH = BGG;
                                BGI = ASQ;
                            } else {
                                BGH = BEO;
                                BGI = BEP;
                            }
                            let BGJ = BGI * BGH;
                            let BGK = ((BGH * BGH) + AVG).sqrt();
                            let BGL = (BGJ + BGJ) * (JV / (KZ * BGK));
                            let BGM = BGH + BGK;
                            let BGN = BGI + BGL;
                            let BGO = -2e0f64 / BGM;
                            let BGP = BGO.exp();
                            let BGQ = AUJ * BGP;
                            let BGR = ((((BGN * BGO) * JM) / BGM) * BGP) * AUJ;
                            let BGS = ASN * BGK;
                            let BGT = BGS * BGM;
                            let BGU = (KT * BGQ) / BGT;
                            let BGV = ((BGR * KT) - ((((BGL * ASN) * BGM) + (BGN * BGS)) * BGU)) / BGT;
                            BER = BGQ;
                            BES = BGU;
                            BET = BGR;
                            BEU = BGV;
                        }
                        let BEV = BC - ATK;
                        let BEW = BEV * FR;
                        let BEX = HU * BEV;
                        let BEY = BET * FB;
                        let BEZ = (BER * FB).exp();
                        let BFA = (BEY + Lanes([(HE * BER), 0.0, 0.0, 0.0])) * BEZ;
                        let BFB = BEZ - BC;
                        let BFC = BEW * BFB;
                        let BFD = Lanes([(BEX * BFB), 0.0, 0.0, 0.0]) + (BFA * BEW);
                        let BFE = BEW * BCM;
                        let BFF = BFE * BEZ;
                        let BFG = BFF * FB;
                        let BFH = BFC + (BFG * BES);
                        let BFI = BFD + (((((((Lanes([(BEX * BCM), 0.0, 0.0, 0.0]) + (BCN * BEW)) * BEZ) + (BFA * BFE)) * FB) + Lanes([(HE * BFF), 0.0, 0.0, 0.0])) * BES) + (BEU * BFG));
                        let BFJ = BC / BDO;
                        let BFK = BC - BFJ;
                        let BFL = (((BDP * BFJ) * JM) / BDO) * JM;
                        let BFM = BFL * BFK;
                        let BFN = ((BFK * BFK) + AUD).sqrt();
                        let BFO = (BFM + BFM) * (JV / (KZ * BFN));
                        let BFP = BC + ((BC + AUD).sqrt());
                        let BFQ = (BFK + BFN) / BFP;
                        let BFR = (BFL + BFO) / BFP;
                        let BFS = BER - AUJ;
                        let BFT = (BFS * FB).exp();
                        let BFU = (BEY + Lanes([(HE * BFS), 0.0, 0.0, 0.0])) * BFT;
                        let BFV = FR * BFQ;
                        let BFW = BFV * BFQ;
                        let BFX = BFW * BFT;
                        let BFY = ((((Lanes([(HU * BFQ), 0.0, 0.0, 0.0]) + (BFR * FR)) * BFQ) + (BFR * BFV)) * BFT) + (BFU * BFW);
                        let BFZ = BDO * BFN;
                        let BGA = KT / BFZ;
                        let BGB = FB * BCM;
                        let BGC = (BC + BGA) + (BGB * BES);
                        let BGD = BFX * BGC;
                        let BGE = (BFY * BGC) + (((((((BDP * BFN) + (BFO * BDO)) * BGA) * JM) / BFZ) + (((Lanes([(HE * BCM), 0.0, 0.0, 0.0]) + (BCN * FB)) * BES) + (BEU * BGB))) * BFX);
                        let BGF = if (if (if (if AUX < AUY { 1.0 } else { 0.0 }) != 0.0 && (if AUZ < AUY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (BFQ * AUX) < AVA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (BFQ * AUZ) < AVA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BHJ;
                        let BHK;
                        let BHL;
                        let BHM;
                        if BGF != 0.0 {
                            let BGW = ATK * BFX;
                            let BGX = BGW * BCM;
                            let BGY = ((BFY * ATK) * BCM) + (BCN * BGW);
                            let BGZ = ATK * BGD;
                            let BHA = BGE * ATK;
                            BHJ = BGX;
                            BHK = BGZ;
                            BHL = BGY;
                            BHM = BHA;
                        } else {
                            let BHB = BC - BFQ;
                            let BHC = BFR * JM;
                            let BHD = BHB - BC;
                            let BHE = BC - BFK;
                            let BHF = BFN * BCM;
                            let BHG = (BHD * BHE) / BHF;
                            let BHH = (((BHC * BHE) + ((BFL * JM) * BHD)) - (((BFO * BCM) + (BCN * BFN)) * BHG)) / BHF;
                            let BHI = if (AWF.abs()) > AMY { 1.0 } else { 0.0 };
                            let BIQ;
                            let BIR;
                            let BIS;
                            let BIT;
                            if BHI != 0.0 {
                                let BHS = (BHD * AWQ).exp();
                                let BHT = (BHC * AWQ) * BHS;
                                let BHU = if AWT < AUY { 1.0 } else { 0.0 };
                                let BLA;
                                let BLB;
                                let BLC;
                                let BLD;
                                if BHU != 0.0 {
                                    let BJF = BHS * AYG;
                                    let BJG = BHT * AYG;
                                    let BJH = (BC - BHS) / BJF;
                                    let BJI = ((BHT * JM) - (BJG * BJH)) / BJF;
                                    let BJJ = AYG * BJH;
                                    let BJK = BJI * AYG;
                                    let BJL = BC + BJJ;
                                    let BJM = AYO * AYG;
                                    let BJN = KF + (BJM * BJH);
                                    let BJO = ((KT * ((BJJ * BJN) - (KF * (BJL.ln())))) / AYG) / AYG;
                                    let BJP = (((((BJK * BJN) + ((BJI * BJM) * BJJ)) - ((BJK * (JV / BJL)) * KF)) * KT) / AYG) / AYG;
                                    let BJQ = -AWQ;
                                    let BJR = (BJQ * BHG) / BJF;
                                    let BJS = BC + BJL;
                                    let BJT = BJS * BJH;
                                    let BJU = (BJT * BJR) / BJL;
                                    let BJV = (((((BJK * BJH) + (BJI * BJS)) * BJR) + ((((BHH * BJQ) - (BJG * BJR)) / BJF) * BJT)) - (BJK * BJU)) / BJL;
                                    BLA = BJO;
                                    BLB = BJU;
                                    BLC = BJP;
                                    BLD = BJV;
                                } else {
                                    let BJW = AUZ - (BHS * AUX);
                                    let BJX = (BHT * AUX) * JM;
                                    let BJY = (BHS - BC) / BJW;
                                    let BJZ = (BHT - (BJX * BJY)) / BJW;
                                    let BKA = BJZ * AUZ;
                                    let BKB = BC + (AUZ * BJY);
                                    let BKC = AXD * AZF;
                                    let BKD = KF - BKC;
                                    let BKE = AXD * BJY;
                                    let BKF = BJZ * AXD;
                                    let BKG = BKC + BKE;
                                    let BKH = BKD / BKB;
                                    let BKI = BJZ * AUX;
                                    let BKJ = BC + (AUX * BJY);
                                    let BKK = AZO * AZP;
                                    let BKL = KF - BKK;
                                    let BKM = AZO * BJY;
                                    let BKN = BJZ * AZO;
                                    let BKO = BKK + BKM;
                                    let BKP = BKL / BKJ;
                                    let BKQ = (((((BKB.ln()) * BKD) * AZF) + (BKG * BJY)) - ((((BKJ.ln()) * BKL) * AZP) + (BKO * BJY))) / AWF;
                                    let BKR = (((((BKA * (JV / BKB)) * BKD) * AZF) + ((BKF * BJY) + (BJZ * BKG))) - ((((BKI * (JV / BKJ)) * BKL) * AZP) + ((BKN * BJY) + (BJZ * BKO)))) / AWF;
                                    let BKS = BJW * BJW;
                                    let BKT = BJX * BJW;
                                    let BKU = (-2e0f64 * AWF) / BKS;
                                    let BKV = (BKU * BHS) * AWQ;
                                    let BKW = BKV * BHG;
                                    let BKX = ((BKH + BKC) + (BKE * KT)) - ((BKP + BKK) + (BKM * KT));
                                    let BKY = (BKX * BKW) / AWF;
                                    let BKZ = (((((((BKA * BKH) * JM) / BKB) + (BKF * KT)) - ((((BKI * BKP) * JM) / BKJ) + (BKN * KT))) * BKW) + ((((((((((BKT + BKT) * BKU) * JM) / BKS) * BHS) + (BHT * BKU)) * AWQ) * BHG) + (BHH * BKV)) * BKX)) / AWF;
                                    BLA = BKQ;
                                    BLB = BKY;
                                    BLC = BKR;
                                    BLD = BKZ;
                                }
                                BIQ = BLA;
                                BIR = BLB;
                                BIS = BLC;
                                BIT = BLD;
                            } else {
                                let BHV = BHC * AUX;
                                let BHW = BC + (BHB * AUX);
                                let BHX = (BC - BHB) / BHW;
                                let BHY = ((BHC * JM) - (BHV * BHX)) / BHW;
                                let BHZ = BHY * AUX;
                                let BIA = BC + (AUX * BHX);
                                let BIB = BHX * BHX;
                                let BIC = BHY * BHX;
                                let BID = AXD * KT;
                                let BIE = BC + (BID * BHX);
                                let BIF = (BIB * BIE) / BIA;
                                let BIG = ((((BIC + BIC) * BIE) + ((BHY * BID) * BIB)) - (BHZ * BIF)) / BIA;
                                let BIH = -BHG;
                                let BII = (BIH * BIA) / BHW;
                                let BIJ = BIA * BIA;
                                let BIK = BHZ * BIA;
                                let BIL = BC / BIJ;
                                let BIM = BC + BIL;
                                let BIN = BHX * BIM;
                                let BIO = BIN * BII;
                                let BIP = (((BHY * BIM) + (((((BIK + BIK) * BIL) * JM) / BIJ) * BHX)) * BII) + ((((((BHH * JM) * BIA) + (BHZ * BIH)) - (BHV * BII)) / BHW) * BIN);
                                BIQ = BIF;
                                BIR = BIO;
                                BIS = BIG;
                                BIT = BIP;
                            }
                            let BIU = ATK * FR;
                            let BIV = BIU * BFT;
                            let BIW = Lanes([((HU * ATK) * BFT), 0.0, 0.0, 0.0]) + (BFU * BIU);
                            let BIX = BIV * BIQ;
                            let BIY = (BIW * BIQ) + (BIS * BIV);
                            let BIZ = BIX * BCM;
                            let BJA = (BIY * BCM) + (BCN * BIX);
                            let BJB = BIZ * BES;
                            let BJC = BIV * BCM;
                            let BJD = (BIX + (BJB * FB)) + (BJC * BIR);
                            let BJE = (BIY + ((((BJA * BES) + (BEU * BIZ)) * FB) + Lanes([(HE * BJB), 0.0, 0.0, 0.0]))) + ((((BIW * BCM) + (BCN * BIV)) * BIR) + (BIT * BJC));
                            BHJ = BIZ;
                            BHK = BJD;
                            BHL = BJA;
                            BHM = BJE;
                        }
                        let BHN = BEV * BFX;
                        let BHO = BEV * BGD;
                        let BHP = BGE * BEV;
                        let BHQ = (BFC * BCM) + (BHN * BCM);
                        let BHR = ((BFD * BCM) + (BCN * BFC)) + (((BFY * BEV) * BCM) + (BCN * BHN));
                        let BLN;
                        let BLO;
                        let BLP;
                        let BLQ;
                        if ARA != 0.0 {
                            let BLE = ((BDJ + (BAO * BHQ)) + (FS * BDU)) + (FT * BHJ);
                            let BLF = ((BDL + (BHR * BAO)) + (Lanes([(HV * BDU), 0.0, 0.0, 0.0]) + (BDV * FS))) + (Lanes([(HW * BHJ), 0.0, 0.0, 0.0]) + (BHL * FT));
                            let BLG = ((BDK + (BAO * (BFH + BHO))) + (FS * BDR)) + (FT * BHK);
                            let BLH = ((Lanes([BDM[0], BDM[1], 0.0, BDM[2]]) + ((BFI + BHP) * BAO)) + (Lanes([(HV * BDR), 0.0, 0.0, 0.0]) + (BDS * FS))) + (Lanes([(HW * BHK), 0.0, 0.0, 0.0]) + (BHM * FT));
                            BLN = BLE;
                            BLO = BLG;
                            BLP = BLF;
                            BLQ = BLH;
                        } else {
                            let BLI = (((FQ * BCQ) + BHQ) + (FS * BDU)) + (FT * BHJ);
                            let BLJ = (((Lanes([(HT * BCQ), 0.0, 0.0, 0.0]) + (BCS * FQ)) + BHR) + (Lanes([(HV * BDU), 0.0, 0.0, 0.0]) + (BDV * FS))) + (Lanes([(HW * BHJ), 0.0, 0.0, 0.0]) + (BHL * FT));
                            let BLK = Lanes([(HT * AOO), 0.0, 0.0]) + (AOP * FQ);
                            let BLL = (((FQ * AOO) + (BFH + BHO)) + (FS * BDR)) + (FT * BHK);
                            let BLM = ((Lanes([BLK[0], BLK[1], 0.0, BLK[2]]) + (BFI + BHP)) + (Lanes([(HV * BDR), 0.0, 0.0, 0.0]) + (BDS * FS))) + (Lanes([(HW * BHK), 0.0, 0.0, 0.0]) + (BHM * FT));
                            BLN = BLI;
                            BLO = BLL;
                            BLP = BLJ;
                            BLQ = BLM;
                        }
                        BDY = BLN;
                        BDZ = BLO;
                        BEA = BLP;
                        BEB = BLQ;
                    } else {
                        let BDX = Lanes([BDM[0], BDM[1], 0.0, BDM[2]]);
                        BDY = BDJ;
                        BDZ = BDK;
                        BEA = BDL;
                        BEB = BDX;
                    }
                    let BEC = BBE * ARE;
                    let BED = BEC * BCO;
                    let BEE = BCP * BEC;
                    let BEF = ((BDZ * BCM) + BED) / BCI;
                    let BEG = BC + BEF;
                    let BEH = (-(BCI - ((ANP + BDY) + BED))) / BEG;
                    let BEI = (((BCK - ((ANQ + BEA) + BEE)) * JM) - ((((((BEB * BCM) + (BCN * BDZ)) + BEE) - (BCK * BEF)) / BCI) * BEH)) / BEG;
                    let BEK = BEJ * BCI;
                    let BEL = BEK.abs();
                    let BEM = (BCK * BEJ) * ((KZ * (if BEK >= 0e0f64 { 1.0 } else { 0.0 })) - JV);
                    let BEN = if (BEH.abs()) > BEL { 1.0 } else { 0.0 };
                    let BLS;
                    let BLT;
                    if BEN != 0.0 {
                        let BLR = if BEH >= AR { 1.0 } else { 0.0 };
                        let BLZ;
                        let BMA;
                        if BLR != 0.0 {
                            BLZ = BEL;
                            BMA = BEM;
                        } else {
                            let BLX = -BEL;
                            let BLY = BEM * JM;
                            BLZ = BLX;
                            BMA = BLY;
                        }
                        BLS = BLZ;
                        BLT = BMA;
                    } else {
                        BLS = BEH;
                        BLT = BEI;
                    }
                    let BLU = BCI + BLS;
                    let BLV = BCK + BLT;
                    let BLW = BCJ + BC;
                    BCH = BLS;
                    BCI = BLU;
                    BCJ = BLW;
                    BCK = BLV;
                }
                let BCT = AFB / BCI;
                let BCU = (AQS - (BCK * BCT)) / BCI;
                let BCV = AFF / BCI;
                let BCW = (AQV - (BCK * BCV)) / BCI;
                let BCX = AOO * BCT;
                let BCY = AOP * BCT;
                let BCZ = Lanes([BCY[0], BCY[1], 0.0, BCY[2]]) + (BCU * AOO);
                let BMB = if (if BCT >= ASA { 1.0 } else { 0.0 }) != 0.0 || ASB != 0.0 { 1.0 } else { 0.0 };
                let BMM;
                let BMN;
                let BMO;
                let BMP;
                let BMQ;
                let BMR;
                if BMB != 0.0 {
                    let BMC = BCT / AQK;
                    let BMD = (BCU - (AQL * BMC)) / AQK;
                    let BME = (ASF * (BMC.ln())).exp();
                    let BMF = ASH * BME;
                    let BMG = (((BMD * (JV / BMC)) * ASF) * BME) * ASH;
                    let BMH = BC + ASF;
                    let BMI = (BMF * BCT) / BMH;
                    let BMJ = ((BMG * BCT) + (BCU * BMF)) / BMH;
                    let BMK = if ASN < (ANG * (QE / QJ)) { 1.0 } else { 0.0 };
                    let BMX;
                    let BMY;
                    let BMZ;
                    let BNA;
                    if BMK != 0.0 {
                        BMX = AR;
                        BMY = AR;
                        BMZ = ASQ;
                        BNA = ASQ;
                    } else {
                        let BMU = (BCT - AQK) / ASN;
                        let BMV = (BCU - AQL) / ASN;
                        let BMW = if BMU < -1e10f64 { 1.0 } else { 0.0 };
                        let BON;
                        let BOO;
                        if BMW != 0.0 {
                            BON = BOM;
                            BOO = ASQ;
                        } else {
                            BON = BMU;
                            BOO = BMV;
                        }
                        let BOP = BOO * BON;
                        let BOQ = ((BON * BON) + AVG).sqrt();
                        let BOR = (BOP + BOP) * (JV / (KZ * BOQ));
                        let BOS = BON + BOQ;
                        let BOT = BOO + BOR;
                        let BOU = -2e0f64 / BOS;
                        let BOV = BOU.exp();
                        let BOW = AUJ * BOV;
                        let BOX = ((((BOT * BOU) * JM) / BOS) * BOV) * AUJ;
                        let BOY = ASN * BOQ;
                        let BOZ = BOY * BOS;
                        let BPA = (KT * BOW) / BOZ;
                        let BPB = ((BOX * KT) - ((((BOR * ASN) * BOS) + (BOT * BOY)) * BPA)) / BOZ;
                        BMX = BOW;
                        BMY = BPA;
                        BMZ = BOX;
                        BNA = BPB;
                    }
                    let BNB = BC - ATK;
                    let BNC = BNB * FR;
                    let BND = HU * BNB;
                    let BNE = BMZ * FB;
                    let BNF = (BMX * FB).exp();
                    let BNG = (BNE + Lanes([(HE * BMX), 0.0, 0.0, 0.0])) * BNF;
                    let BNH = BNF - BC;
                    let BNI = BNC * BNH;
                    let BNJ = Lanes([(BND * BNH), 0.0, 0.0, 0.0]) + (BNG * BNC);
                    let BNK = BNC * BCT;
                    let BNL = BNK * BNF;
                    let BNM = BNL * FB;
                    let BNN = BNI + (BNM * BMY);
                    let BNO = BNJ + (((((((Lanes([(BND * BCT), 0.0, 0.0, 0.0]) + (BCU * BNC)) * BNF) + (BNG * BNK)) * FB) + Lanes([(HE * BNL), 0.0, 0.0, 0.0])) * BMY) + (BNA * BNM));
                    let BNP = BC / BMC;
                    let BNQ = BC - BNP;
                    let BNR = (((BMD * BNP) * JM) / BMC) * JM;
                    let BNS = BNR * BNQ;
                    let BNT = ((BNQ * BNQ) + AUD).sqrt();
                    let BNU = (BNS + BNS) * (JV / (KZ * BNT));
                    let BNV = BC + ((BC + AUD).sqrt());
                    let BNW = (BNQ + BNT) / BNV;
                    let BNX = (BNR + BNU) / BNV;
                    let BNY = BMX - AUJ;
                    let BNZ = (BNY * FB).exp();
                    let BOA = (BNE + Lanes([(HE * BNY), 0.0, 0.0, 0.0])) * BNZ;
                    let BOB = FR * BNW;
                    let BOC = BOB * BNW;
                    let BOD = BOC * BNZ;
                    let BOE = ((((Lanes([(HU * BNW), 0.0, 0.0, 0.0]) + (BNX * FR)) * BNW) + (BNX * BOB)) * BNZ) + (BOA * BOC);
                    let BOF = BMC * BNT;
                    let BOG = KT / BOF;
                    let BOH = FB * BCT;
                    let BOI = (BC + BOG) + (BOH * BMY);
                    let BOJ = BOD * BOI;
                    let BOK = (BOE * BOI) + (((((((BMD * BNT) + (BNU * BMC)) * BOG) * JM) / BOF) + (((Lanes([(HE * BCT), 0.0, 0.0, 0.0]) + (BCU * FB)) * BMY) + (BNA * BOH))) * BOD);
                    let BOL = if (if (if (if AUX < AUY { 1.0 } else { 0.0 }) != 0.0 && (if AUZ < AUY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (BNW * AUX) < AVA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (BNW * AUZ) < AVA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BPP;
                    let BPQ;
                    let BPR;
                    let BPS;
                    if BOL != 0.0 {
                        let BPC = ATK * BOD;
                        let BPD = BPC * BCT;
                        let BPE = ((BOE * ATK) * BCT) + (BCU * BPC);
                        let BPF = ATK * BOJ;
                        let BPG = BOK * ATK;
                        BPP = BPD;
                        BPQ = BPF;
                        BPR = BPE;
                        BPS = BPG;
                    } else {
                        let BPH = BC - BNW;
                        let BPI = BNX * JM;
                        let BPJ = BPH - BC;
                        let BPK = BC - BNQ;
                        let BPL = BNT * BCT;
                        let BPM = (BPJ * BPK) / BPL;
                        let BPN = (((BPI * BPK) + ((BNR * JM) * BPJ)) - (((BNU * BCT) + (BCU * BNT)) * BPM)) / BPL;
                        let BPO = if (AWF.abs()) > AMY { 1.0 } else { 0.0 };
                        let BQW;
                        let BQX;
                        let BQY;
                        let BQZ;
                        if BPO != 0.0 {
                            let BPY = (BPJ * AWQ).exp();
                            let BPZ = (BPI * AWQ) * BPY;
                            let BQA = if AWT < AUY { 1.0 } else { 0.0 };
                            let BTG;
                            let BTH;
                            let BTI;
                            let BTJ;
                            if BQA != 0.0 {
                                let BRL = BPY * AYG;
                                let BRM = BPZ * AYG;
                                let BRN = (BC - BPY) / BRL;
                                let BRO = ((BPZ * JM) - (BRM * BRN)) / BRL;
                                let BRP = AYG * BRN;
                                let BRQ = BRO * AYG;
                                let BRR = BC + BRP;
                                let BRS = AYO * AYG;
                                let BRT = KF + (BRS * BRN);
                                let BRU = ((KT * ((BRP * BRT) - (KF * (BRR.ln())))) / AYG) / AYG;
                                let BRV = (((((BRQ * BRT) + ((BRO * BRS) * BRP)) - ((BRQ * (JV / BRR)) * KF)) * KT) / AYG) / AYG;
                                let BRW = -AWQ;
                                let BRX = (BRW * BPM) / BRL;
                                let BRY = BC + BRR;
                                let BRZ = BRY * BRN;
                                let BSA = (BRZ * BRX) / BRR;
                                let BSB = (((((BRQ * BRN) + (BRO * BRY)) * BRX) + ((((BPN * BRW) - (BRM * BRX)) / BRL) * BRZ)) - (BRQ * BSA)) / BRR;
                                BTG = BRU;
                                BTH = BSA;
                                BTI = BRV;
                                BTJ = BSB;
                            } else {
                                let BSC = AUZ - (BPY * AUX);
                                let BSD = (BPZ * AUX) * JM;
                                let BSE = (BPY - BC) / BSC;
                                let BSF = (BPZ - (BSD * BSE)) / BSC;
                                let BSG = BSF * AUZ;
                                let BSH = BC + (AUZ * BSE);
                                let BSI = AXD * AZF;
                                let BSJ = KF - BSI;
                                let BSK = AXD * BSE;
                                let BSL = BSF * AXD;
                                let BSM = BSI + BSK;
                                let BSN = BSJ / BSH;
                                let BSO = BSF * AUX;
                                let BSP = BC + (AUX * BSE);
                                let BSQ = AZO * AZP;
                                let BSR = KF - BSQ;
                                let BSS = AZO * BSE;
                                let BST = BSF * AZO;
                                let BSU = BSQ + BSS;
                                let BSV = BSR / BSP;
                                let BSW = (((((BSH.ln()) * BSJ) * AZF) + (BSM * BSE)) - ((((BSP.ln()) * BSR) * AZP) + (BSU * BSE))) / AWF;
                                let BSX = (((((BSG * (JV / BSH)) * BSJ) * AZF) + ((BSL * BSE) + (BSF * BSM))) - ((((BSO * (JV / BSP)) * BSR) * AZP) + ((BST * BSE) + (BSF * BSU)))) / AWF;
                                let BSY = BSC * BSC;
                                let BSZ = BSD * BSC;
                                let BTA = (-2e0f64 * AWF) / BSY;
                                let BTB = (BTA * BPY) * AWQ;
                                let BTC = BTB * BPM;
                                let BTD = ((BSN + BSI) + (BSK * KT)) - ((BSV + BSQ) + (BSS * KT));
                                let BTE = (BTD * BTC) / AWF;
                                let BTF = (((((((BSG * BSN) * JM) / BSH) + (BSL * KT)) - ((((BSO * BSV) * JM) / BSP) + (BST * KT))) * BTC) + ((((((((((BSZ + BSZ) * BTA) * JM) / BSY) * BPY) + (BPZ * BTA)) * AWQ) * BPM) + (BPN * BTB)) * BTD)) / AWF;
                                BTG = BSW;
                                BTH = BTE;
                                BTI = BSX;
                                BTJ = BTF;
                            }
                            BQW = BTG;
                            BQX = BTH;
                            BQY = BTI;
                            BQZ = BTJ;
                        } else {
                            let BQB = BPI * AUX;
                            let BQC = BC + (BPH * AUX);
                            let BQD = (BC - BPH) / BQC;
                            let BQE = ((BPI * JM) - (BQB * BQD)) / BQC;
                            let BQF = BQE * AUX;
                            let BQG = BC + (AUX * BQD);
                            let BQH = BQD * BQD;
                            let BQI = BQE * BQD;
                            let BQJ = AXD * KT;
                            let BQK = BC + (BQJ * BQD);
                            let BQL = (BQH * BQK) / BQG;
                            let BQM = ((((BQI + BQI) * BQK) + ((BQE * BQJ) * BQH)) - (BQF * BQL)) / BQG;
                            let BQN = -BPM;
                            let BQO = (BQN * BQG) / BQC;
                            let BQP = BQG * BQG;
                            let BQQ = BQF * BQG;
                            let BQR = BC / BQP;
                            let BQS = BC + BQR;
                            let BQT = BQD * BQS;
                            let BQU = BQT * BQO;
                            let BQV = (((BQE * BQS) + (((((BQQ + BQQ) * BQR) * JM) / BQP) * BQD)) * BQO) + ((((((BPN * JM) * BQG) + (BQF * BQN)) - (BQB * BQO)) / BQC) * BQT);
                            BQW = BQL;
                            BQX = BQU;
                            BQY = BQM;
                            BQZ = BQV;
                        }
                        let BRA = ATK * FR;
                        let BRB = BRA * BNZ;
                        let BRC = Lanes([((HU * ATK) * BNZ), 0.0, 0.0, 0.0]) + (BOA * BRA);
                        let BRD = BRB * BQW;
                        let BRE = (BRC * BQW) + (BQY * BRB);
                        let BRF = BRD * BCT;
                        let BRG = (BRE * BCT) + (BCU * BRD);
                        let BRH = BRF * BMY;
                        let BRI = BRB * BCT;
                        let BRJ = (BRD + (BRH * FB)) + (BRI * BQX);
                        let BRK = (BRE + ((((BRG * BMY) + (BNA * BRF)) * FB) + Lanes([(HE * BRH), 0.0, 0.0, 0.0]))) + ((((BRC * BCT) + (BCU * BRB)) * BQX) + (BQZ * BRI));
                        BPP = BRF;
                        BPQ = BRJ;
                        BPR = BRG;
                        BPS = BRK;
                    }
                    let BPT = BNB * BOD;
                    let BPU = BNB * BOJ;
                    let BPV = BOK * BNB;
                    let BPW = (BNI * BCT) + (BPT * BCT);
                    let BPX = ((BNJ * BCT) + (BCU * BNI)) + (((BOE * BNB) * BCT) + (BCU * BPT));
                    let BTS;
                    let BTT;
                    let BTU;
                    let BTV;
                    if ARA != 0.0 {
                        let BTK = ((BCX + BPW) + BMI) + BPP;
                        let BTL = ((BCZ + BPX) + BMJ) + BPR;
                        let BTM = ((AOO + (BNN + BPU)) + BMF) + BPQ;
                        let BTN = ((Lanes([AOP[0], AOP[1], 0.0, AOP[2]]) + (BNO + BPV)) + BMG) + BPS;
                        BTS = BTK;
                        BTT = BTM;
                        BTU = BTL;
                        BTV = BTN;
                    } else {
                        let BTO = ((BCX + BPW) + BMI) + BPP;
                        let BTP = ((BCZ + BPX) + BMJ) + BPR;
                        let BTQ = ((AOO + (BNN + BPU)) + BMF) + BPQ;
                        let BTR = ((Lanes([AOP[0], AOP[1], 0.0, AOP[2]]) + (BNO + BPV)) + BMG) + BPS;
                        BTS = BTO;
                        BTT = BTQ;
                        BTU = BTP;
                        BTV = BTR;
                    }
                    BMM = BTS;
                    BMN = BPW;
                    BMO = BTT;
                    BMP = BTU;
                    BMQ = BPX;
                    BMR = BTV;
                } else {
                    let BML = Lanes([AOP[0], AOP[1], 0.0, AOP[2]]);
                    BMM = BCX;
                    BMN = AR;
                    BMO = AOO;
                    BMP = BCZ;
                    BMQ = ASQ;
                    BMR = BML;
                }
                let BMS = ARE * BCV;
                let BMT = BCW * ARE;
                BBH = BCT;
                BBI = BCV;
                BBJ = BMM;
                BBK = BMS;
                BBL = BMN;
                BBM = BMO;
                BBN = BCU;
                BBO = BCW;
                BBP = BMP;
                BBQ = BMT;
                BBR = BMQ;
                BBS = BMR;
            } else {
                BBH = AQR;
                BBI = AQU;
                BBJ = ASS;
                BBK = ASZ;
                BBL = AST;
                BBM = ASU;
                BBN = AQT;
                BBO = AQW;
                BBP = ASW;
                BBQ = ATA;
                BBR = ASX;
                BBS = ASY;
            }
            let BBT = BBH - BBI;
            let BBU = BBN - BBO;
            let BBV = AOO * BBH;
            let BBW = AOP * BBH;
            let BBX = ARE * BBI;
            let BBZ = BBY * (((AHB + AHI) + (BBV * FB)) + (BBX * FB));
            let BCA = R - A;
            let BCB = Lanes([S, 0.0]) - Lanes([0.0, D]);
            let BCC = BBZ * BCA;
            let BCD = ((((Lanes([AHD[0], 0.0, AHD[1], AHD[2]]) + Lanes([AHK[0], AHK[1], 0.0, AHK[2]])) + (((Lanes([BBW[0], BBW[1], 0.0, BBW[2]]) + (BBN * AOO)) * FB) + Lanes([(HE * BBV), 0.0, 0.0, 0.0]))) + (((BBO * ARE) * FB) + Lanes([(HE * BBX), 0.0, 0.0, 0.0]))) * BBY) * BCA;
            let BCE = BCB * BBZ;
            let BCF = Lanes([BCD[0], BCD[1], BCD[2], 0.0, BCD[3]]) + Lanes([0.0, 0.0, 0.0, BCE[0], BCE[1]]);
            let BCG = if OB > AR { 1.0 } else { 0.0 };
            let BUB;
            let BUC;
            if BCG != 0.0 {
                let BTX = BTW * EX;
                let BTY = N / BTX;
                let BTZ = (ANV - Lanes([((HA * BTW) * BTY), 0.0, 0.0])) / BTX;
                let BUA = if BTY > AED { 1.0 } else { 0.0 };
                let BUE;
                let BUF;
                let BUG;
                let BUH;
                if BUA != 0.0 {
                    let BUD = BC + (BTY - AED);
                    BUE = BUD;
                    BUF = AED;
                    BUG = BTZ;
                    BUH = AIH;
                } else {
                    BUE = BC;
                    BUF = BTY;
                    BUG = AIH;
                    BUH = BTZ;
                }
                let BUI = rspice_limexp(BUF);
                let BUJ = (BUE * BUI) - BC;
                let BUK = FU * BUJ;
                let BUL = Lanes([(HX * BUJ), 0.0, 0.0]) + (((BUG * BUI) + ((BUH * BUI) * BUE)) * FU);
                BUB = BUK;
                BUC = BUL;
            } else {
                BUB = AR;
                BUC = AIH;
            }
            let BUN;
            let BUO;
            if AZ != 0.0 {
                let BUM = if (if FF > AR { 1.0 } else { 0.0 }) != 0.0 && (if FG > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BVB;
                let BVC;
                if BUM != 0.0 {
                    let BUP = (BC / NC) - BC;
                    let BUQ = AHI / FF;
                    let BUR = (BUP * (BUQ.ln())).exp();
                    let BUS = ((((AHK - Lanes([(HI * BUQ), 0.0, 0.0])) / FF) * (JV / BUQ)) * BUP) * BUR;
                    let BUT = -FV;
                    let BUU = O * BUT;
                    let BUV = FG * BUR;
                    let BUW = (BUT * N) / BUV;
                    let BUX = -FW;
                    let BUY = (BUX * BUR).exp();
                    let BUZ = BUW * BUY;
                    let BVA = ((((Lanes([((HY * JM) * N), 0.0, 0.0]) + Lanes([0.0, BUU[0], BUU[1]])) - ((Lanes([(HJ * BUR), 0.0, 0.0]) + (BUS * FG)) * BUW)) / BUV) * BUY) + (((Lanes([((HZ * JM) * BUR), 0.0, 0.0]) + (BUS * BUX)) * BUY) * BUW);
                    BVB = BUZ;
                    BVC = BVA;
                } else {
                    BVB = AR;
                    BVC = AIH;
                }
                BUN = BVB;
                BUO = BVC;
            } else {
                BUN = AR;
                BUO = AIH;
            }
            let BVG;
            let BVH;
            if AY != 0.0 {
                let BVD = FG - N;
                let BVE = Lanes([HJ, 0.0, 0.0]) - ANV;
                let BVF = if BVD > AR { 1.0 } else { 0.0 };
                let BVO;
                let BVP;
                if BVF != 0.0 {
                    let BVN = if BVM > AR { 1.0 } else { 0.0 };
                    let BWB;
                    let BWC;
                    if BVN != 0.0 {
                        let BVQ = AHI / FF;
                        let BVR = BVM * FO;
                        let BVT = (BVR * FP) + (BVS * BBH);
                        let BVU = (BVQ / AIY).exp();
                        let BVV = (((AHK - Lanes([(HI * BVQ), 0.0, 0.0])) / FF) / AIY) * BVU;
                        let BVW = BBH / BVT;
                        let BVX = (BC - BVW) / AIY;
                        let BVY = (BVU - KT) + (KT * (BVX.cosh()));
                        let BVZ = (AIY * (BVY.ln())).sqrt();
                        let BWA = (((Lanes([BVV[0], BVV[1], 0.0, BVV[2]]) + ((((((BBN - ((Lanes([(((HR * BVM) * FP) + (HS * BVR)), 0.0, 0.0, 0.0]) + (BBN * BVS)) * BVW)) / BVT) * JM) / AIY) * (BVX.sinh())) * KT)) * (JV / BVY)) * AIY) * (JV / (KZ * BVZ));
                        BWB = BVZ;
                        BWC = BWA;
                    } else {
                        BWB = BC;
                        BWC = ASQ;
                    }
                    let BWD = FX / AHI;
                    let BWE = (Lanes([IA, 0.0, 0.0]) - (AHK * BWD)) / AHI;
                    let BWF = FX / FF;
                    let BWG = (IA - (HI * BWF)) / FF;
                    let BWH = if BVD > BWF { 1.0 } else { 0.0 };
                    let BXE;
                    let BXF;
                    if BWH != 0.0 {
                        let BWI = BWE * JM;
                        let BWJ = BWF * BWB;
                        let BWK = (-BWD) / BWJ;
                        let BWL = BWK.exp();
                        let BWM = FY * BWL;
                        let BWN = BWD / BWF;
                        let BWO = BC + BWN;
                        let BWP = BVD - BWF;
                        let BWQ = Lanes([BWG, 0.0, 0.0]);
                        let BWR = BWF + (BWO * BWP);
                        let BWS = BWM * BWR;
                        let BWT = (BWQ + ((((BWE - Lanes([(BWG * BWN), 0.0, 0.0])) / BWF) * BWP) + ((BVE - BWQ) * BWO))) * BWM;
                        let BWU = ((Lanes([(IB * BWL), 0.0, 0.0, 0.0]) + ((((Lanes([BWI[0], BWI[1], 0.0, BWI[2]]) - ((Lanes([(BWG * BWB), 0.0, 0.0, 0.0]) + (BWC * BWF)) * BWK)) / BWJ) * BWL) * FY)) * BWR) + Lanes([BWT[0], BWT[1], 0.0, BWT[2]]);
                        BXE = BWS;
                        BXF = BWU;
                    } else {
                        let BWV = FY * BVD;
                        let BWW = BWE * JM;
                        let BWX = BVD * BWB;
                        let BWY = BVE * BWB;
                        let BWZ = (-BWD) / BWX;
                        let BXA = BWZ.exp();
                        let BXB = BWV * BXA;
                        let BXC = (Lanes([(IB * BVD), 0.0, 0.0]) + (BVE * FY)) * BXA;
                        let BXD = Lanes([BXC[0], BXC[1], 0.0, BXC[2]]) + ((((Lanes([BWW[0], BWW[1], 0.0, BWW[2]]) - ((Lanes([BWY[0], BWY[1], 0.0, BWY[2]]) + (BWC * BVD)) * BWZ)) / BWX) * BXA) * BWV);
                        BXE = BXB;
                        BXF = BXD;
                    }
                    let BXH = if BXG > AR { 1.0 } else { 0.0 };
                    let BXR;
                    let BXS;
                    if BXH != 0.0 {
                        let BXI = BC - (BXG * BXE);
                        let BXJ = (BXF * BXG) * JM;
                        let BXK = BXJ * BXI;
                        let BXL = ((BXI * BXI) + 1e-4f64).sqrt();
                        let BXM = KF * (BXI + BXL);
                        let BXN = (BBH * BXE) / BXM;
                        let BXO = (((BBN * BXE) + (BXF * BBH)) - (((BXJ + ((BXK + BXK) * (JV / (KZ * BXL)))) * KF) * BXN)) / BXM;
                        BXR = BXN;
                        BXS = BXO;
                    } else {
                        let BXP = BBH * BXE;
                        let BXQ = (BBN * BXE) + (BXF * BBH);
                        BXR = BXP;
                        BXS = BXQ;
                    }
                    BVO = BXR;
                    BVP = BXS;
                } else {
                    BVO = AR;
                    BVP = ASQ;
                }
                BVG = BVO;
                BVH = BVP;
            } else {
                BVG = AR;
                BVH = ASQ;
            }
            let BVJ = BBL * BVI;
            let BVK = BBR * BVI;
            let BVL = if FZ > AR { 1.0 } else { 0.0 };
            let BYF;
            let BYG;
            if BVL != 0.0 {
                let BXT = BC + parameters[92];
                let BXU = BXT * FK;
                let BXV = Lanes([AHC[0], 0.0, AHC[1], AHC[2]]);
                let BXW = ((AHA + AHH) + BBJ) / BXU;
                let BXX = (((BXV + Lanes([AHJ[0], AHJ[1], 0.0, AHJ[2]])) + BBP) - Lanes([((HN * BXT) * BXW), 0.0, 0.0, 0.0])) / BXU;
                let BXY = BC + BXW;
                let BXZ = BXX * BXY;
                let BYA = ((BXY * BXY) + AUY).sqrt();
                let BYB = KF * (BXY + BYA);
                let BYC = FZ / BYB;
                let BYD = (Lanes([IC, 0.0, 0.0, 0.0]) - (((BXX + ((BXZ + BXZ) * (JV / (KZ * BYA)))) * KF) * BYC)) / BYB;
                let BYE = if AEG > AR { 1.0 } else { 0.0 };
                let BYO;
                let BYP;
                if BYE != 0.0 {
                    let BYI = AEH * BYC;
                    let BYK = (BYC * AEG) * BYJ;
                    let BYL = BYK * FB;
                    let BYM = ((((BYD * AEG) + Lanes([BYI[0], 0.0, BYI[1], BYI[2]])) * BYJ) * FB) + Lanes([(HE * BYK), 0.0, 0.0, 0.0]);
                    let BYN = if BYL < ARZ { 1.0 } else { 0.0 };
                    let BYY;
                    let BYZ;
                    if BYN != 0.0 {
                        let BYR = BC - (KF * BYL);
                        let BYS = BYC * BYR;
                        let BYT = (BYD * BYR) + (((BYM * KF) * JM) * BYC);
                        BYY = BYS;
                        BYZ = BYT;
                    } else {
                        let BYU = BC + BYL;
                        let BYV = BYU.ln();
                        let BYW = (BYC * BYV) / BYL;
                        let BYX = (((BYD * BYV) + ((BYM * (JV / BYU)) * BYC)) - (BYM * BYW)) / BYL;
                        BYY = BYW;
                        BYZ = BYX;
                    }
                    BYO = BYY;
                    BYP = BYZ;
                } else {
                    BYO = BYC;
                    BYP = BYD;
                }
                let BYQ = if BBJ > AR { 1.0 } else { 0.0 };
                let BZF;
                let BZG;
                if BYQ != 0.0 {
                    let BZB = AHA + (BBJ * BZA);
                    let BZC = AHA + BBJ;
                    let BZD = (BYO * BZB) / BZC;
                    let BZE = (((BYP * BZB) + ((BXV + (BBP * BZA)) * BYO)) - ((BXV + BBP) * BZD)) / BZC;
                    BZF = BZD;
                    BZG = BZE;
                } else {
                    BZF = BYO;
                    BZG = BYP;
                }
                BYF = BZF;
                BYG = BZG;
            } else {
                BYF = AR;
                BYG = ASQ;
            }
            let BZN;
            let BZO;
            if BYH != 0.0 {
                let BZI = BZH * EX;
                let BZJ = T / BZI;
                let BZK = (Lanes([0.0, U[0], U[1]]) - Lanes([((HA * BZH) * BZJ), 0.0, 0.0])) / BZI;
                let BZL = if BZJ > AED { 1.0 } else { 0.0 };
                let BZR;
                let BZS;
                let BZT;
                let BZU;
                if BZL != 0.0 {
                    let BZQ = BC + (BZJ - AED);
                    BZR = BZQ;
                    BZS = AED;
                    BZT = BZK;
                    BZU = BZM;
                } else {
                    BZR = BC;
                    BZS = BZJ;
                    BZT = BZM;
                    BZU = BZK;
                }
                let BZV = rspice_limexp(BZS);
                let BZW = (BZR * BZV) - BC;
                let BZX = GA * BZW;
                let BZY = Lanes([(ID * BZW), 0.0, 0.0]) + (((BZT * BZV) + ((BZU * BZV) * BZR)) * GA);
                BZN = BZX;
                BZO = BZY;
            } else {
                BZN = AR;
                BZO = BZM;
            }
            let CAD;
            let CAE;
            if BZP != 0.0 {
                let BZZ = UD * EX;
                let CAA = T / BZZ;
                let CAB = (Lanes([0.0, U[0], U[1]]) - Lanes([((HA * UD) * CAA), 0.0, 0.0])) / BZZ;
                let CAC = if CAA > AED { 1.0 } else { 0.0 };
                let CAH;
                let CAI;
                let CAJ;
                let CAK;
                if CAC != 0.0 {
                    let CAG = BC + (CAA - AED);
                    CAH = CAG;
                    CAI = AED;
                    CAJ = CAB;
                    CAK = BZM;
                } else {
                    CAH = BC;
                    CAI = CAA;
                    CAJ = BZM;
                    CAK = CAB;
                }
                let CAL = rspice_limexp(CAI);
                let CAM = (CAH * CAL) - BC;
                let CAN = GB * CAM;
                let CAO = Lanes([(IE * CAM), 0.0, 0.0]) + (((CAJ * CAL) + ((CAK * CAL) * CAH)) * GB);
                CAD = CAN;
                CAE = CAO;
            } else {
                CAD = AR;
                CAE = BZM;
            }
            let CAF = if GC > AR { 1.0 } else { 0.0 };
            let CBX;
            let CBY;
            let CBZ;
            let CCA;
            if CAF != 0.0 {
                let CAP = ((-(GE.ln())) / TN).exp();
                let CAQ = BC - CAP;
                let CAR = GD * CAQ;
                let CAS = CAR - T;
                let CAT = Lanes([((IG * CAQ) + ((((((IH * (JV / GE)) * JM) / TN) * CAP) * JM) * GD)), 0.0, 0.0]);
                let CAU = Lanes([0.0, U[0], U[1]]);
                let CAV = CAS * FB;
                let CAW = ((CAT - CAU) * FB) + Lanes([(HE * CAS), 0.0, 0.0]);
                let CAX = CAW * CAV;
                let CAY = ((CAV * CAV) + AGA).sqrt();
                let CAZ = (CAX + CAX) * (JV / (KZ * CAY));
                let CBA = (CAV + CAY) * KF;
                let CBB = (CAW + CAZ) * KF;
                let CBC = CAR - (EX * CBA);
                let CBD = CAT - (Lanes([(HA * CBA), 0.0, 0.0]) + (CBB * EX));
                let CBE = CBA / CAY;
                let CBF = (CBB - (CAZ * CBE)) / CAY;
                let CBG = CBC / GD;
                let CBH = BC - CBG;
                let CBI = CBH.ln();
                let CBJ = (((CBD - Lanes([(IG * CBG), 0.0, 0.0])) / GD) * JM) * (JV / CBH);
                let CBK = -TN;
                let CBL = (CBK * CBI).exp();
                let CBM = BC - CBE;
                let CBN = (CBL * CBE) + (GE * CBM);
                let CBO = GC * CBN;
                let CBP = Lanes([(IF * CBN), 0.0, 0.0]) + ((((((CBJ * CBK) * CBL) * CBE) + (CBF * CBL)) + (Lanes([(IH * CBM), 0.0, 0.0]) + ((CBF * JM) * GE))) * GC);
                let CBQ = BC - TN;
                let CBR = (CBI * CBQ).exp();
                let CBS = BC - CBR;
                let CBT = T - CBC;
                let CBU = ((GD * CBS) / CBQ) + (GE * CBT);
                let CBV = GC * CBU;
                let CBW = Lanes([(IF * CBU), 0.0, 0.0]) + ((((Lanes([(IG * CBS), 0.0, 0.0]) + ((((CBJ * CBQ) * CBR) * JM) * GD)) / CBQ) + (Lanes([(IH * CBT), 0.0, 0.0]) + ((CAU - CBD) * GE))) * GC);
                CBX = CBO;
                CBY = CBV;
                CBZ = CBP;
                CCA = CBW;
            } else {
                CBX = AR;
                CBY = AR;
                CBZ = BZM;
                CCA = BZM;
            }
            let CCD;
            let CCE;
            if BU != 0.0 {
                let CCB = if (if (if BX == BC { 1.0 } else { 0.0 }) != 0.0 && CAF != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if GD > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CCZ;
                let CDA;
                if CCB != 0.0 {
                    let CCL = BC - (BC / TN);
                    let CCM = CBX / GC;
                    let CCN = (CCL * (CCM.ln())).exp();
                    let CCO = ((((CBZ - Lanes([(IF * CCM), 0.0, 0.0])) / GC) * (JV / CCM)) * CCL) * CCN;
                    let CCP = T / GD;
                    let CCQ = -CCP;
                    let CCR = CCQ * GF;
                    let CCS = CCR * CCN;
                    let CCT = (-GG) / CCN;
                    let CCU = CCT.exp();
                    let CCV = CCS * CCU;
                    let CCW = ((((((((Lanes([0.0, U[0], U[1]]) - Lanes([(IG * CCP), 0.0, 0.0])) / GD) * JM) * GF) + Lanes([(II * CCQ), 0.0, 0.0])) * CCN) + (CCO * CCR)) * CCU) + ((((Lanes([(IJ * JM), 0.0, 0.0]) - (CCO * CCT)) / CCN) * CCU) * CCS);
                    let CCX = Lanes([CCW[0], CCW[1], CCW[2], 0.0]);
                    CCZ = CCV;
                    CDA = CCX;
                } else {
                    let CCY = if (if (if BX == AR { 1.0 } else { 0.0 }) != 0.0 && AFH != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if FD > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CDN;
                    let CDO;
                    if CCY != 0.0 {
                        let CDB = BC - (BC / LF);
                        let CDC = AHB / FC;
                        let CDD = (CDB * (CDC.ln())).exp();
                        let CDE = ((((AHD - Lanes([(HF * CDC), 0.0, 0.0])) / FC) * (JV / CDC)) * CDB) * CDD;
                        let CDF = H / FD;
                        let CDG = -CDF;
                        let CDH = CDG * GF;
                        let CDI = CDH * CDD;
                        let CDJ = (-GG) / CDD;
                        let CDK = CDJ.exp();
                        let CDL = CDI * CDK;
                        let CDM = ((((((((Lanes([0.0, I[0], I[1]]) - Lanes([(HG * CDF), 0.0, 0.0])) / FD) * JM) * GF) + Lanes([(II * CDG), 0.0, 0.0])) * CDD) + (CDE * CDH)) * CDK) + ((((Lanes([(IJ * JM), 0.0, 0.0]) - (CDE * CDJ)) / CDD) * CDK) * CDI);
                        CDN = CDL;
                        CDO = CDM;
                    } else {
                        CDN = AR;
                        CDO = AEF;
                    }
                    let CDP = Lanes([CDO[0], CDO[1], 0.0, CDO[2]]);
                    CCZ = CDN;
                    CDA = CDP;
                }
                CCD = CCZ;
                CCE = CDA;
            } else {
                CCD = AR;
                CCE = CCC;
            }
            let CCF = (H / UU).exp();
            let CCG = CCF - BC;
            let CCH = GH * CCG;
            let CCI = ((I / UU) * CCF) * GH;
            let CCJ = Lanes([(IK * CCG), 0.0, 0.0]) + Lanes([0.0, CCI[0], CCI[1]]);
            let CDS;
            let CDT;
            if CCK != 0.0 {
                let CDQ = if GI > AR { 1.0 } else { 0.0 };
                let CER;
                let CES;
                if CDQ != 0.0 {
                    let CDV = WZ / KX;
                    let CDX = CDW - GJ;
                    let CDY = IM * JM;
                    let CDZ = ((-(GK.ln())) / WZ).exp();
                    let CEA = BC - CDZ;
                    let CEB = GJ * CEA;
                    let CEC = (IM * CEA) + ((((((IN * (JV / GK)) * JM) / WZ) * CDZ) * JM) * GJ);
                    let CED = GK * GI;
                    let CEE = (IN * GI) + (IL * GK);
                    let CEF = CDV - WZ;
                    let CEG = CDW / GJ;
                    let CEH = (CEF * (CEG.ln())).exp();
                    let CEI = GI * CEH;
                    let CEJ = (IL * CEH) + (((((((IM * CEG) * JM) / GJ) * (JV / CEG)) * CEF) * CEH) * GI);
                    let CEK = CEB - X;
                    let CEL = Lanes([CEC, 0.0, 0.0]);
                    let CEM = Lanes([0.0, Y[0], Y[1]]);
                    let CEN = CEK * FB;
                    let CEO = ((CEL - CEM) * FB) + Lanes([(HE * CEK), 0.0, 0.0]);
                    let CEP = if CEN < AED { 1.0 } else { 0.0 };
                    let CEY;
                    let CEZ;
                    if CEP != 0.0 {
                        let CET = CEN.exp();
                        let CEU = BC + CET;
                        let CEV = CEU.ln();
                        let CEW = CEB - (EX * CEV);
                        let CEX = CEL - (Lanes([(HA * CEV), 0.0, 0.0]) + (((CEO * CET) * (JV / CEU)) * EX));
                        CEY = CEW;
                        CEZ = CEX;
                    } else {
                        CEY = X;
                        CEZ = CEM;
                    }
                    let CFA = (AIY * CDX) + (KX * EX);
                    let CFB = (CDY * AIY) + (HA * KX);
                    let CFC = (CDX + CEY) / CFA;
                    let CFD = ((Lanes([CDY, 0.0, 0.0]) + CEZ) - Lanes([(CFB * CFC), 0.0, 0.0])) / CFA;
                    let CFE = if CFC < AED { 1.0 } else { 0.0 };
                    let CFM;
                    let CFN;
                    if CFE != 0.0 {
                        let CFF = CFC.exp();
                        let CFG = BC + CFF;
                        let CFH = (-(CDX + CEB)) / CFA;
                        let CFI = CFH.exp();
                        let CFJ = (CFG.ln()) - CFI;
                        let CFK = (-CDX) + (CFA * CFJ);
                        let CFL = Lanes([(CDY * JM), 0.0, 0.0]) + (Lanes([(CFB * CFJ), 0.0, 0.0]) + ((((CFD * CFF) * (JV / CFG)) - Lanes([(((((CDY + CEC) * JM) - (CFB * CFH)) / CFA) * CFI), 0.0, 0.0])) * CFA));
                        CFM = CFK;
                        CFN = CFL;
                    } else {
                        CFM = CEY;
                        CFN = CEZ;
                    }
                    let CFO = X - CEY;
                    let CFP = CEY / GJ;
                    let CFQ = BC - CFP;
                    let CFR = CFM / GJ;
                    let CFS = BC - CFR;
                    let CFT = CFS.ln();
                    let CFU = (((CFN - Lanes([(IM * CFR), 0.0, 0.0])) / GJ) * JM) * (JV / CFS);
                    let CFV = BC - WZ;
                    let CFW = BC - CDV;
                    let CFX = (CFT * CFV).exp();
                    let CFY = BC - CFX;
                    let CFZ = ((CFQ.ln()) * CFW).exp();
                    let CGA = BC - CFZ;
                    let CGB = (CFT * CFW).exp();
                    let CGC = BC - CGB;
                    let CGD = (((GI * CFY) / CFV) + ((CEI * CGA) / CFW)) - ((CEI * CGC) / CFW);
                    let CGE = (CGD * GJ) + (CED * CFO);
                    let CGF = ((((((Lanes([(IL * CFY), 0.0, 0.0]) + ((((CFU * CFV) * CFX) * JM) * GI)) / CFV) + ((Lanes([(CEJ * CGA), 0.0, 0.0]) + ((((((((CEZ - Lanes([(IM * CFP), 0.0, 0.0])) / GJ) * JM) * (JV / CFQ)) * CFW) * CFZ) * JM) * CEI)) / CFW)) - ((Lanes([(CEJ * CGC), 0.0, 0.0]) + ((((CFU * CFW) * CGB) * JM) * CEI)) / CFW)) * GJ) + Lanes([(IM * CGD), 0.0, 0.0])) + (Lanes([(CEE * CFO), 0.0, 0.0]) + ((CEM - CEZ) * CED));
                    CER = CGE;
                    CES = CGF;
                } else {
                    CER = AR;
                    CES = CEQ;
                }
                CDS = CER;
                CDT = CES;
            } else {
                let CDR = if GI > AR { 1.0 } else { 0.0 };
                let CHC;
                let CHD;
                if CDR != 0.0 {
                    let CGG = ((-(GK.ln())) / WZ).exp();
                    let CGH = BC - CGG;
                    let CGI = GJ * CGH;
                    let CGJ = CGI - X;
                    let CGK = Lanes([((IM * CGH) + ((((((IN * (JV / GK)) * JM) / WZ) * CGG) * JM) * GJ)), 0.0, 0.0]);
                    let CGL = Lanes([0.0, Y[0], Y[1]]);
                    let CGM = CGJ * FB;
                    let CGN = ((CGK - CGL) * FB) + Lanes([(HE * CGJ), 0.0, 0.0]);
                    let CGO = CGN * CGM;
                    let CGP = ((CGM * CGM) + AGA).sqrt();
                    let CGQ = (CGM + CGP) * KF;
                    let CGR = CGI - (EX * CGQ);
                    let CGS = CGK - (Lanes([(HA * CGQ), 0.0, 0.0]) + (((CGN + ((CGO + CGO) * (JV / (KZ * CGP)))) * KF) * EX));
                    let CGT = CGR / GJ;
                    let CGU = BC - CGT;
                    let CGV = BC - WZ;
                    let CGW = ((CGU.ln()) * CGV).exp();
                    let CGX = BC - CGW;
                    let CGY = X - CGR;
                    let CGZ = ((GJ * CGX) / CGV) + (GK * CGY);
                    let CHA = GI * CGZ;
                    let CHB = Lanes([(IL * CGZ), 0.0, 0.0]) + ((((Lanes([(IM * CGX), 0.0, 0.0]) + ((((((((CGS - Lanes([(IM * CGT), 0.0, 0.0])) / GJ) * JM) * (JV / CGU)) * CGV) * CGW) * JM) * GJ)) / CGV) + (Lanes([(IN * CGY), 0.0, 0.0]) + ((CGL - CGS) * GK))) * GI);
                    CHC = CHA;
                    CHD = CHB;
                } else {
                    CHC = AR;
                    CHD = CEQ;
                }
                CDS = CHC;
                CDT = CHD;
            }
            let CHJ;
            let CHK;
            if CDU != 0.0 {
                let CHF = CHE * EX;
                let CHG = X / CHF;
                let CHH = (Lanes([0.0, Y[0], Y[1]]) - Lanes([((HA * CHE) * CHG), 0.0, 0.0])) / CHF;
                let CHI = if CHG > AED { 1.0 } else { 0.0 };
                let CHM;
                let CHN;
                let CHO;
                let CHP;
                if CHI != 0.0 {
                    let CHL = BC + (CHG - AED);
                    CHM = CHL;
                    CHN = AED;
                    CHO = CHH;
                    CHP = CEQ;
                } else {
                    CHM = BC;
                    CHN = CHG;
                    CHO = CEQ;
                    CHP = CHH;
                }
                let CHQ = rspice_limexp(CHN);
                let CHR = (CHM * CHQ) - BC;
                let CHS = GL * CHR;
                let CHT = Lanes([(IO * CHR), 0.0, 0.0]) + (((CHO * CHQ) + ((CHP * CHQ) * CHM)) * GL);
                CHJ = CHS;
                CHK = CHT;
            } else {
                CHJ = AR;
                CHK = CEQ;
            }
            let CHW;
            let CHX;
            if CCK != 0.0 {
                let CHU = if GM > AR { 1.0 } else { 0.0 };
                let CIU;
                let CIV;
                if CHU != 0.0 {
                    let CHZ = WZ / KX;
                    let CIA = CDW - GJ;
                    let CIB = IM * JM;
                    let CIC = ((-(GK.ln())) / WZ).exp();
                    let CID = BC - CIC;
                    let CIE = GJ * CID;
                    let CIF = (IM * CID) + ((((((IN * (JV / GK)) * JM) / WZ) * CIC) * JM) * GJ);
                    let CIG = GK * GM;
                    let CIH = (IN * GM) + (IP * GK);
                    let CII = CHZ - WZ;
                    let CIJ = CDW / GJ;
                    let CIK = (CII * (CIJ.ln())).exp();
                    let CIL = GM * CIK;
                    let CIM = (IP * CIK) + (((((((IM * CIJ) * JM) / GJ) * (JV / CIJ)) * CII) * CIK) * GM);
                    let CIN = CIE - AD;
                    let CIO = Lanes([0.0, CIF, 0.0]);
                    let CIP = Lanes([AE[0], 0.0, AE[1]]);
                    let CIQ = CIN * FB;
                    let CIR = ((CIO - CIP) * FB) + Lanes([0.0, (HE * CIN), 0.0]);
                    let CIS = if CIQ < AED { 1.0 } else { 0.0 };
                    let CJB;
                    let CJC;
                    if CIS != 0.0 {
                        let CIW = CIQ.exp();
                        let CIX = BC + CIW;
                        let CIY = CIX.ln();
                        let CIZ = CIE - (EX * CIY);
                        let CJA = CIO - (Lanes([0.0, (HA * CIY), 0.0]) + (((CIR * CIW) * (JV / CIX)) * EX));
                        CJB = CIZ;
                        CJC = CJA;
                    } else {
                        CJB = AD;
                        CJC = CIP;
                    }
                    let CJD = (AIY * CIA) + (KX * EX);
                    let CJE = (CIB * AIY) + (HA * KX);
                    let CJF = (CIA + CJB) / CJD;
                    let CJG = ((Lanes([0.0, CIB, 0.0]) + CJC) - Lanes([0.0, (CJE * CJF), 0.0])) / CJD;
                    let CJH = if CJF < AED { 1.0 } else { 0.0 };
                    let CJP;
                    let CJQ;
                    if CJH != 0.0 {
                        let CJI = CJF.exp();
                        let CJJ = BC + CJI;
                        let CJK = (-(CIA + CIE)) / CJD;
                        let CJL = CJK.exp();
                        let CJM = (CJJ.ln()) - CJL;
                        let CJN = (-CIA) + (CJD * CJM);
                        let CJO = Lanes([0.0, (CIB * JM), 0.0]) + (Lanes([0.0, (CJE * CJM), 0.0]) + ((((CJG * CJI) * (JV / CJJ)) - Lanes([0.0, (((((CIB + CIF) * JM) - (CJE * CJK)) / CJD) * CJL), 0.0])) * CJD));
                        CJP = CJN;
                        CJQ = CJO;
                    } else {
                        CJP = CJB;
                        CJQ = CJC;
                    }
                    let CJR = AD - CJB;
                    let CJS = CJB / GJ;
                    let CJT = BC - CJS;
                    let CJU = CJP / GJ;
                    let CJV = BC - CJU;
                    let CJW = CJV.ln();
                    let CJX = (((CJQ - Lanes([0.0, (IM * CJU), 0.0])) / GJ) * JM) * (JV / CJV);
                    let CJY = BC - WZ;
                    let CJZ = BC - CHZ;
                    let CKA = (CJW * CJY).exp();
                    let CKB = BC - CKA;
                    let CKC = ((CJT.ln()) * CJZ).exp();
                    let CKD = BC - CKC;
                    let CKE = (CJW * CJZ).exp();
                    let CKF = BC - CKE;
                    let CKG = (((GM * CKB) / CJY) + ((CIL * CKD) / CJZ)) - ((CIL * CKF) / CJZ);
                    let CKH = (CKG * GJ) + (CIG * CJR);
                    let CKI = ((((((Lanes([0.0, (IP * CKB), 0.0]) + ((((CJX * CJY) * CKA) * JM) * GM)) / CJY) + ((Lanes([0.0, (CIM * CKD), 0.0]) + ((((((((CJC - Lanes([0.0, (IM * CJS), 0.0])) / GJ) * JM) * (JV / CJT)) * CJZ) * CKC) * JM) * CIL)) / CJZ)) - ((Lanes([0.0, (CIM * CKF), 0.0]) + ((((CJX * CJZ) * CKE) * JM) * CIL)) / CJZ)) * GJ) + Lanes([0.0, (IM * CKG), 0.0])) + (Lanes([0.0, (CIH * CJR), 0.0]) + ((CIP - CJC) * CIG));
                    CIU = CKH;
                    CIV = CKI;
                } else {
                    CIU = AR;
                    CIV = CIT;
                }
                CHW = CIU;
                CHX = CIV;
            } else {
                let CHV = if GM > AR { 1.0 } else { 0.0 };
                let CLF;
                let CLG;
                if CHV != 0.0 {
                    let CKJ = ((-(GK.ln())) / WZ).exp();
                    let CKK = BC - CKJ;
                    let CKL = GJ * CKK;
                    let CKM = CKL - AD;
                    let CKN = Lanes([0.0, ((IM * CKK) + ((((((IN * (JV / GK)) * JM) / WZ) * CKJ) * JM) * GJ)), 0.0]);
                    let CKO = Lanes([AE[0], 0.0, AE[1]]);
                    let CKP = CKM * FB;
                    let CKQ = ((CKN - CKO) * FB) + Lanes([0.0, (HE * CKM), 0.0]);
                    let CKR = CKQ * CKP;
                    let CKS = ((CKP * CKP) + AGA).sqrt();
                    let CKT = (CKP + CKS) * KF;
                    let CKU = CKL - (EX * CKT);
                    let CKV = CKN - (Lanes([0.0, (HA * CKT), 0.0]) + (((CKQ + ((CKR + CKR) * (JV / (KZ * CKS)))) * KF) * EX));
                    let CKW = CKU / GJ;
                    let CKX = BC - CKW;
                    let CKY = BC - WZ;
                    let CKZ = ((CKX.ln()) * CKY).exp();
                    let CLA = BC - CKZ;
                    let CLB = AD - CKU;
                    let CLC = ((GJ * CLA) / CKY) + (GK * CLB);
                    let CLD = GM * CLC;
                    let CLE = Lanes([0.0, (IP * CLC), 0.0]) + ((((Lanes([0.0, (IM * CLA), 0.0]) + ((((((((CKV - Lanes([0.0, (IM * CKW), 0.0])) / GJ) * JM) * (JV / CKX)) * CKY) * CKZ) * JM) * GJ)) / CKY) + (Lanes([0.0, (IN * CLB), 0.0]) + ((CKO - CKV) * GK))) * GM);
                    CLF = CLD;
                    CLG = CLE;
                } else {
                    CLF = AR;
                    CLG = CIT;
                }
                CHW = CLF;
                CHX = CLG;
            }
            let CLJ;
            let CLK;
            if CHY != 0.0 {
                let CLH = if GN > AR { 1.0 } else { 0.0 };
                let CMH;
                let CMI;
                if CLH != 0.0 {
                    let CLL = ZQ / KX;
                    let CLN = CLM - GO;
                    let CLO = IR * JM;
                    let CLP = ((-(GP.ln())) / ZQ).exp();
                    let CLQ = BC - CLP;
                    let CLR = GO * CLQ;
                    let CLS = (IR * CLQ) + ((((((IS * (JV / GP)) * JM) / ZQ) * CLP) * JM) * GO);
                    let CLT = GP * GN;
                    let CLU = (IS * GN) + (IQ * GP);
                    let CLV = CLL - ZQ;
                    let CLW = CLM / GO;
                    let CLX = (CLV * (CLW.ln())).exp();
                    let CLY = GN * CLX;
                    let CLZ = (IQ * CLX) + (((((((IR * CLW) * JM) / GO) * (JV / CLW)) * CLV) * CLX) * GN);
                    let CMA = CLR - AJ;
                    let CMB = Lanes([CLS, 0.0, 0.0]);
                    let CMC = Lanes([0.0, AK[0], AK[1]]);
                    let CMD = CMA * FB;
                    let CME = ((CMB - CMC) * FB) + Lanes([(HE * CMA), 0.0, 0.0]);
                    let CMF = if CMD < AED { 1.0 } else { 0.0 };
                    let CMO;
                    let CMP;
                    if CMF != 0.0 {
                        let CMJ = CMD.exp();
                        let CMK = BC + CMJ;
                        let CML = CMK.ln();
                        let CMM = CLR - (EX * CML);
                        let CMN = CMB - (Lanes([(HA * CML), 0.0, 0.0]) + (((CME * CMJ) * (JV / CMK)) * EX));
                        CMO = CMM;
                        CMP = CMN;
                    } else {
                        CMO = AJ;
                        CMP = CMC;
                    }
                    let CMQ = (AIY * CLN) + (KX * EX);
                    let CMR = (CLO * AIY) + (HA * KX);
                    let CMS = (CLN + CMO) / CMQ;
                    let CMT = ((Lanes([CLO, 0.0, 0.0]) + CMP) - Lanes([(CMR * CMS), 0.0, 0.0])) / CMQ;
                    let CMU = if CMS < AED { 1.0 } else { 0.0 };
                    let CNC;
                    let CND;
                    if CMU != 0.0 {
                        let CMV = CMS.exp();
                        let CMW = BC + CMV;
                        let CMX = (-(CLN + CLR)) / CMQ;
                        let CMY = CMX.exp();
                        let CMZ = (CMW.ln()) - CMY;
                        let CNA = (-CLN) + (CMQ * CMZ);
                        let CNB = Lanes([(CLO * JM), 0.0, 0.0]) + (Lanes([(CMR * CMZ), 0.0, 0.0]) + ((((CMT * CMV) * (JV / CMW)) - Lanes([(((((CLO + CLS) * JM) - (CMR * CMX)) / CMQ) * CMY), 0.0, 0.0])) * CMQ));
                        CNC = CNA;
                        CND = CNB;
                    } else {
                        CNC = CMO;
                        CND = CMP;
                    }
                    let CNE = AJ - CMO;
                    let CNF = CMO / GO;
                    let CNG = BC - CNF;
                    let CNH = CNC / GO;
                    let CNI = BC - CNH;
                    let CNJ = CNI.ln();
                    let CNK = (((CND - Lanes([(IR * CNH), 0.0, 0.0])) / GO) * JM) * (JV / CNI);
                    let CNL = BC - ZQ;
                    let CNM = BC - CLL;
                    let CNN = (CNJ * CNL).exp();
                    let CNO = BC - CNN;
                    let CNP = ((CNG.ln()) * CNM).exp();
                    let CNQ = BC - CNP;
                    let CNR = (CNJ * CNM).exp();
                    let CNS = BC - CNR;
                    let CNT = (((GN * CNO) / CNL) + ((CLY * CNQ) / CNM)) - ((CLY * CNS) / CNM);
                    let CNU = (CNT * GO) + (CLT * CNE);
                    let CNV = ((((((Lanes([(IQ * CNO), 0.0, 0.0]) + ((((CNK * CNL) * CNN) * JM) * GN)) / CNL) + ((Lanes([(CLZ * CNQ), 0.0, 0.0]) + ((((((((CMP - Lanes([(IR * CNF), 0.0, 0.0])) / GO) * JM) * (JV / CNG)) * CNM) * CNP) * JM) * CLY)) / CNM)) - ((Lanes([(CLZ * CNS), 0.0, 0.0]) + ((((CNK * CNM) * CNR) * JM) * CLY)) / CNM)) * GO) + Lanes([(IR * CNT), 0.0, 0.0])) + (Lanes([(CLU * CNE), 0.0, 0.0]) + ((CMC - CMP) * CLT));
                    CMH = CNU;
                    CMI = CNV;
                } else {
                    CMH = AR;
                    CMI = CMG;
                }
                CLJ = CMH;
                CLK = CMI;
            } else {
                let CLI = if GN > AR { 1.0 } else { 0.0 };
                let COS;
                let COT;
                if CLI != 0.0 {
                    let CNW = ((-(GP.ln())) / ZQ).exp();
                    let CNX = BC - CNW;
                    let CNY = GO * CNX;
                    let CNZ = CNY - AJ;
                    let COA = Lanes([((IR * CNX) + ((((((IS * (JV / GP)) * JM) / ZQ) * CNW) * JM) * GO)), 0.0, 0.0]);
                    let COB = Lanes([0.0, AK[0], AK[1]]);
                    let COC = CNZ * FB;
                    let COD = ((COA - COB) * FB) + Lanes([(HE * CNZ), 0.0, 0.0]);
                    let COE = COD * COC;
                    let COF = ((COC * COC) + AGA).sqrt();
                    let COG = (COC + COF) * KF;
                    let COH = CNY - (EX * COG);
                    let COI = COA - (Lanes([(HA * COG), 0.0, 0.0]) + (((COD + ((COE + COE) * (JV / (KZ * COF)))) * KF) * EX));
                    let COJ = COH / GO;
                    let COK = BC - COJ;
                    let COL = BC - ZQ;
                    let COM = ((COK.ln()) * COL).exp();
                    let CON = BC - COM;
                    let COO = AJ - COH;
                    let COP = ((GO * CON) / COL) + (GP * COO);
                    let COQ = GN * COP;
                    let COR = Lanes([(IQ * COP), 0.0, 0.0]) + ((((Lanes([(IR * CON), 0.0, 0.0]) + ((((((((COI - Lanes([(IR * COJ), 0.0, 0.0])) / GO) * JM) * (JV / COK)) * COL) * COM) * JM) * GO)) / COL) + (Lanes([(IS * COO), 0.0, 0.0]) + ((COB - COI) * GP))) * GN);
                    COS = COQ;
                    COT = COR;
                } else {
                    COS = AR;
                    COT = CMG;
                }
                CLJ = COS;
                CLK = COT;
            }
            let COY;
            let COZ;
            if CX != 0.0 {
                let CPD;
                let CPE;
                if COU != 0.0 {
                    let CPB = if GQ > AR { 1.0 } else { 0.0 };
                    let CQB;
                    let CQC;
                    if CPB != 0.0 {
                        let CPF = ADH / KX;
                        let CPH = CPG - GR;
                        let CPI = IU * JM;
                        let CPJ = ((-(GS.ln())) / ADH).exp();
                        let CPK = BC - CPJ;
                        let CPL = GR * CPK;
                        let CPM = (IU * CPK) + ((((((IV * (JV / GS)) * JM) / ADH) * CPJ) * JM) * GR);
                        let CPN = GS * GQ;
                        let CPO = (IV * GQ) + (IT * GS);
                        let CPP = CPF - ADH;
                        let CPQ = CPG / GR;
                        let CPR = (CPP * (CPQ.ln())).exp();
                        let CPS = GQ * CPR;
                        let CPT = (IT * CPR) + (((((((IU * CPQ) * JM) / GR) * (JV / CPQ)) * CPP) * CPR) * GQ);
                        let CPU = CPL - AP;
                        let CPV = Lanes([0.0, 0.0, CPM]);
                        let CPW = Lanes([AQ[0], AQ[1], 0.0]);
                        let CPX = CPU * FB;
                        let CPY = ((CPV - CPW) * FB) + Lanes([0.0, 0.0, (HE * CPU)]);
                        let CPZ = if CPX < AED { 1.0 } else { 0.0 };
                        let CQI;
                        let CQJ;
                        if CPZ != 0.0 {
                            let CQD = CPX.exp();
                            let CQE = BC + CQD;
                            let CQF = CQE.ln();
                            let CQG = CPL - (EX * CQF);
                            let CQH = CPV - (Lanes([0.0, 0.0, (HA * CQF)]) + (((CPY * CQD) * (JV / CQE)) * EX));
                            CQI = CQG;
                            CQJ = CQH;
                        } else {
                            CQI = AP;
                            CQJ = CPW;
                        }
                        let CQK = (AIY * CPH) + (KX * EX);
                        let CQL = (CPI * AIY) + (HA * KX);
                        let CQM = (CPH + CQI) / CQK;
                        let CQN = ((Lanes([0.0, 0.0, CPI]) + CQJ) - Lanes([0.0, 0.0, (CQL * CQM)])) / CQK;
                        let CQO = if CQM < AED { 1.0 } else { 0.0 };
                        let CQW;
                        let CQX;
                        if CQO != 0.0 {
                            let CQP = CQM.exp();
                            let CQQ = BC + CQP;
                            let CQR = (-(CPH + CPL)) / CQK;
                            let CQS = CQR.exp();
                            let CQT = (CQQ.ln()) - CQS;
                            let CQU = (-CPH) + (CQK * CQT);
                            let CQV = Lanes([0.0, 0.0, (CPI * JM)]) + (Lanes([0.0, 0.0, (CQL * CQT)]) + ((((CQN * CQP) * (JV / CQQ)) - Lanes([0.0, 0.0, (((((CPI + CPM) * JM) - (CQL * CQR)) / CQK) * CQS)])) * CQK));
                            CQW = CQU;
                            CQX = CQV;
                        } else {
                            CQW = CQI;
                            CQX = CQJ;
                        }
                        let CQY = AP - CQI;
                        let CQZ = CQI / GR;
                        let CRA = BC - CQZ;
                        let CRB = CQW / GR;
                        let CRC = BC - CRB;
                        let CRD = CRC.ln();
                        let CRE = (((CQX - Lanes([0.0, 0.0, (IU * CRB)])) / GR) * JM) * (JV / CRC);
                        let CRF = BC - ADH;
                        let CRG = BC - CPF;
                        let CRH = (CRD * CRF).exp();
                        let CRI = BC - CRH;
                        let CRJ = ((CRA.ln()) * CRG).exp();
                        let CRK = BC - CRJ;
                        let CRL = (CRD * CRG).exp();
                        let CRM = BC - CRL;
                        let CRN = (((GQ * CRI) / CRF) + ((CPS * CRK) / CRG)) - ((CPS * CRM) / CRG);
                        let CRO = (CRN * GR) + (CPN * CQY);
                        let CRP = ((((((Lanes([0.0, 0.0, (IT * CRI)]) + ((((CRE * CRF) * CRH) * JM) * GQ)) / CRF) + ((Lanes([0.0, 0.0, (CPT * CRK)]) + ((((((((CQJ - Lanes([0.0, 0.0, (IU * CQZ)])) / GR) * JM) * (JV / CRA)) * CRG) * CRJ) * JM) * CPS)) / CRG)) - ((Lanes([0.0, 0.0, (CPT * CRM)]) + ((((CRE * CRG) * CRL) * JM) * CPS)) / CRG)) * GR) + Lanes([0.0, 0.0, (IU * CRN)])) + (Lanes([0.0, 0.0, (CPO * CQY)]) + ((CPW - CQJ) * CPN));
                        CQB = CRO;
                        CQC = CRP;
                    } else {
                        CQB = AR;
                        CQC = CQA;
                    }
                    CPD = CQB;
                    CPE = CQC;
                } else {
                    let CPC = if GQ > AR { 1.0 } else { 0.0 };
                    let CSM;
                    let CSN;
                    if CPC != 0.0 {
                        let CRQ = ((-(GS.ln())) / ADH).exp();
                        let CRR = BC - CRQ;
                        let CRS = GR * CRR;
                        let CRT = CRS - AP;
                        let CRU = Lanes([0.0, 0.0, ((IU * CRR) + ((((((IV * (JV / GS)) * JM) / ADH) * CRQ) * JM) * GR))]);
                        let CRV = Lanes([AQ[0], AQ[1], 0.0]);
                        let CRW = CRT * FB;
                        let CRX = ((CRU - CRV) * FB) + Lanes([0.0, 0.0, (HE * CRT)]);
                        let CRY = CRX * CRW;
                        let CRZ = ((CRW * CRW) + AGA).sqrt();
                        let CSA = (CRW + CRZ) * KF;
                        let CSB = CRS - (EX * CSA);
                        let CSC = CRU - (Lanes([0.0, 0.0, (HA * CSA)]) + (((CRX + ((CRY + CRY) * (JV / (KZ * CRZ)))) * KF) * EX));
                        let CSD = CSB / GR;
                        let CSE = BC - CSD;
                        let CSF = BC - ADH;
                        let CSG = ((CSE.ln()) * CSF).exp();
                        let CSH = BC - CSG;
                        let CSI = AP - CSB;
                        let CSJ = ((GR * CSH) / CSF) + (GS * CSI);
                        let CSK = GQ * CSJ;
                        let CSL = Lanes([0.0, 0.0, (IT * CSJ)]) + ((((Lanes([0.0, 0.0, (IU * CSH)]) + ((((((((CSC - Lanes([0.0, 0.0, (IU * CSD)])) / GR) * JM) * (JV / CSE)) * CSF) * CSG) * JM) * GR)) / CSF) + (Lanes([0.0, 0.0, (IV * CSI)]) + ((CRV - CSC) * GS))) * GQ);
                        CSM = CSK;
                        CSN = CSL;
                    } else {
                        CSM = AR;
                        CSN = CQA;
                    }
                    CPD = CSM;
                    CPE = CSN;
                }
                COY = CPD;
                COZ = CPE;
            } else {
                let COV = ABO * AP;
                let COW = AQ * ABO;
                let COX = Lanes([COW[0], COW[1], 0.0]);
                COY = COV;
                COZ = COX;
            }
            let CTC;
            let CTD;
            let CTE;
            let CTF;
            if CPA != 0.0 {
                let CSP = CSO * EX;
                let CSQ = HA * CSO;
                let CSR = X / CSP;
                let CSS = rspice_limexp(CSR);
                let CST = ((Lanes([0.0, Y[0], Y[1]]) - Lanes([(CSQ * CSR), 0.0, 0.0])) / CSP) * CSS;
                let CSU = AJ / CSP;
                let CSV = rspice_limexp(CSU);
                let CSW = ((Lanes([0.0, AK[0], AK[1]]) - Lanes([(CSQ * CSU), 0.0, 0.0])) / CSP) * CSV;
                let CSX = CSS - CSV;
                let CSY = GT * CSX;
                let CSZ = Lanes([(IW * CSX), 0.0, 0.0, 0.0]) + ((Lanes([CST[0], CST[1], CST[2], 0.0]) - Lanes([CSW[0], CSW[1], 0.0, CSW[2]])) * GT);
                let CTK;
                let CTL;
                if CTA != 0.0 {
                    let CTH = GU * GT;
                    let CTI = CTH * CSS;
                    let CTJ = Lanes([(((IX * GT) + (IW * GU)) * CSS), 0.0, 0.0]) + (CST * CTH);
                    CTK = CTI;
                    CTL = CTJ;
                } else {
                    CTK = AR;
                    CTL = CEQ;
                }
                CTC = CTK;
                CTD = CSY;
                CTE = CTL;
                CTF = CSZ;
            } else {
                CTC = AR;
                CTD = AR;
                CTE = CEQ;
                CTF = CTB;
            }
            let CTR;
            let CTS;
            if CTG != 0.0 {
                let CTN = CTM * EX;
                let CTO = AJ / CTN;
                let CTP = (Lanes([0.0, AK[0], AK[1]]) - Lanes([((HA * CTM) * CTO), 0.0, 0.0])) / CTN;
                let CTQ = if CTO > AED { 1.0 } else { 0.0 };
                let CTV;
                let CTW;
                let CTX;
                let CTY;
                if CTQ != 0.0 {
                    let CTU = BC + (CTO - AED);
                    CTV = CTU;
                    CTW = AED;
                    CTX = CTP;
                    CTY = CMG;
                } else {
                    CTV = BC;
                    CTW = CTO;
                    CTX = CMG;
                    CTY = CTP;
                }
                let CTZ = rspice_limexp(CTW);
                let CUA = (CTV * CTZ) - BC;
                let CUB = GV * CUA;
                let CUC = Lanes([(IY * CUA), 0.0, 0.0]) + (((CTX * CTZ) + ((CTY * CTZ) * CTV)) * GV);
                CTR = CUB;
                CTS = CUC;
            } else {
                CTR = AR;
                CTS = CMG;
            }
            let CUG;
            let CUH;
            if CTT != 0.0 {
                let CUQ;
                let CUR;
                if CUD != 0.0 {
                    let CUJ = Q * BBT;
                    let CUK = FG - N;
                    let CUL = (Lanes([HJ, 0.0, 0.0]) - ANV) * BVG;
                    let CUM = (P * BBT) + (CUK * BVG);
                    let CUN = (Lanes([0.0, CUJ[0], CUJ[1], CUJ[2]]) + (BBU * P)) + (Lanes([CUL[0], CUL[1], 0.0, CUL[2]]) + (BVH * CUK));
                    let CUO = Lanes([0.0, 0.0, 0.0, CUN[0], CUN[1], CUN[2], 0.0, CUN[3], 0.0]);
                    CUQ = CUM;
                    CUR = CUO;
                } else {
                    let CVL;
                    let CVM;
                    if CUP != 0.0 {
                        let CUS = Q * BBT;
                        let CUT = FG - N;
                        let CUU = (Lanes([HJ, 0.0, 0.0]) - ANV) * BVG;
                        let CUV = I * AEG;
                        let CUW = (AEH * H) + Lanes([0.0, CUV[0], CUV[1]]);
                        let CUX = O * BUB;
                        let CUY = (BUC * N) + Lanes([0.0, CUX[0], CUX[1]]);
                        let CUZ = (((Lanes([0.0, CUS[0], CUS[1], CUS[2]]) + (BBU * P)) + (Lanes([CUU[0], CUU[1], 0.0, CUU[2]]) + (BVH * CUT))) + Lanes([CUW[0], 0.0, CUW[1], CUW[2]])) + Lanes([CUY[0], CUY[1], 0.0, CUY[2]]);
                        let CVA = U * BZN;
                        let CVB = (BZO * T) + Lanes([0.0, CVA[0], CVA[1]]);
                        let CVC = Y * CHJ;
                        let CVD = (CHK * X) + Lanes([0.0, CVC[0], CVC[1]]);
                        let CVE = (Lanes([CUZ[0], CUZ[1], CUZ[2], 0.0, CUZ[3]]) + Lanes([CVB[0], 0.0, CVB[1], CVB[2], 0.0])) + Lanes([CVD[0], CVD[1], 0.0, CVD[2], 0.0]);
                        let CVF = AK * CTR;
                        let CVG = (CTS * AJ) + Lanes([0.0, CVF[0], CVF[1]]);
                        let CVH = ((((((P * BBT) + (CUT * BVG)) + (AEG * H)) + (BUB * N)) + (BZN * T)) + (CHJ * X)) + (CTR * AJ);
                        let CVI = Lanes([CVE[0], CVE[1], CVE[2], CVE[3], CVE[4], 0.0]) + Lanes([CVG[0], CVG[1], 0.0, 0.0, 0.0, CVG[2]]);
                        let CVK = if (if BYF >= CVJ { 1.0 } else { 0.0 }) != 0.0 && (if BYF > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CVU;
                        let CVV;
                        if CVK != 0.0 {
                            let CVN = BCB * BCA;
                            let CVO = CVN + CVN;
                            let CVP = (BCA * BCA) / BYF;
                            let CVQ = BYG * CVP;
                            let CVR = (Lanes([0.0, 0.0, 0.0, CVO[0], CVO[1]]) - Lanes([CVQ[0], CVQ[1], CVQ[2], 0.0, CVQ[3]])) / BYF;
                            let CVS = CVH + CVP;
                            let CVT = CVI + Lanes([CVR[0], CVR[1], CVR[2], CVR[3], CVR[4], 0.0]);
                            CVU = CVS;
                            CVV = CVT;
                        } else {
                            CVU = CVH;
                            CVV = CVI;
                        }
                        let CVW = if (if GW >= CVJ { 1.0 } else { 0.0 }) != 0.0 && (if GW > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CWH;
                        let CWI;
                        if CVW != 0.0 {
                            let CVY = B - CVX;
                            let CWA = (Lanes([0.0, E]) - Lanes([CVZ, 0.0])) * CVY;
                            let CWB = CWA + CWA;
                            let CWC = (CVY * CVY) / GW;
                            let CWD = (Lanes([CWB[0], 0.0, CWB[1]]) - Lanes([0.0, (IZ * CWC), 0.0])) / GW;
                            let CWE = CVU + CWC;
                            let CWF = Lanes([0.0, CVV[0], CVV[1], CVV[2], CVV[3], CVV[4], CVV[5]]) + Lanes([CWD[0], CWD[1], 0.0, CWD[2], 0.0, 0.0, 0.0]);
                            CWH = CWE;
                            CWI = CWF;
                        } else {
                            let CWG = Lanes([0.0, CVV[0], CVV[1], CVV[2], CVV[3], CVV[4], CVV[5]]);
                            CWH = CVU;
                            CWI = CWG;
                        }
                        let CWJ = if (if GX >= CVJ { 1.0 } else { 0.0 }) != 0.0 && (if GX > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CWS;
                        let CWT;
                        if CWJ != 0.0 {
                            let CWK = J - AM;
                            let CWL = (Lanes([0.0, L]) - Lanes([AO, 0.0])) * CWK;
                            let CWM = CWL + CWL;
                            let CWN = (CWK * CWK) / GX;
                            let CWO = (Lanes([CWM[0], 0.0, CWM[1]]) - Lanes([0.0, (JA * CWN), 0.0])) / GX;
                            let CWP = CWH + CWN;
                            let CWQ = Lanes([0.0, CWI[0], CWI[1], CWI[2], CWI[3], CWI[4], CWI[5], CWI[6]]) + Lanes([CWO[0], 0.0, CWO[1], CWO[2], 0.0, 0.0, 0.0, 0.0]);
                            CWS = CWP;
                            CWT = CWQ;
                        } else {
                            let CWR = Lanes([0.0, CWI[0], CWI[1], CWI[2], CWI[3], CWI[4], CWI[5], CWI[6]]);
                            CWS = CWH;
                            CWT = CWR;
                        }
                        let CWU = if (if GY >= CVJ { 1.0 } else { 0.0 }) != 0.0 && (if GY > AR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CXD;
                        let CXE;
                        if CWU != 0.0 {
                            let CWV = Z - R;
                            let CWW = (Lanes([AB, 0.0]) - Lanes([0.0, S])) * CWV;
                            let CWX = CWW + CWW;
                            let CWY = (CWV * CWV) / GY;
                            let CWZ = (Lanes([CWX[0], 0.0, CWX[1]]) - Lanes([0.0, (JB * CWY), 0.0])) / GY;
                            let CXA = CWS + CWY;
                            let CXB = Lanes([CWT[0], 0.0, CWT[1], CWT[2], CWT[3], CWT[4], CWT[5], CWT[6], CWT[7]]) + Lanes([0.0, CWZ[0], 0.0, CWZ[1], 0.0, 0.0, CWZ[2], 0.0, 0.0]);
                            CXD = CXA;
                            CXE = CXB;
                        } else {
                            let CXC = Lanes([CWT[0], 0.0, CWT[1], CWT[2], CWT[3], CWT[4], CWT[5], CWT[6], CWT[7]]);
                            CXD = CWS;
                            CXE = CXC;
                        }
                        CVL = CXD;
                        CVM = CXE;
                    } else {
                        CVL = AR;
                        CVM = CUF;
                    }
                    CUQ = CVL;
                    CUR = CVM;
                }
                CUG = CUQ;
                CUH = CUR;
            } else {
                CUG = CUE;
                CUH = CUF;
            }
            let CYT;
            let CYU;
            let CYV;
            let CYW;
            let CYX;
            let CYY;
            let CYZ;
            let CZA;
            let CZB;
            let CZC;
            let CZD;
            let CZE;
            let CZF;
            let CZG;
            let CZH;
            let CZI;
            if CUI != 0.0 {
                let CXH = Lanes([0.0, 0.0, 0.0, 0.0, CXG]);
                let CXI = (CXF - BBH) / BBM;
                let CXJ = BBS * CXI;
                let CXK = CXI * RC;
                let CXL = (((CXH - Lanes([BBN[0], BBN[1], BBN[2], BBN[3], 0.0])) - Lanes([CXJ[0], CXJ[1], CXJ[2], CXJ[3], 0.0])) / BBM) * RC;
                let CXO = Lanes([0.0, CXG]) - Lanes([CXN, 0.0]);
                let CXP = (CXF - CXM) / BBM;
                let CXQ = BBS * CXP;
                let CXR = CXP * RC;
                let CXS = ((Lanes([0.0, 0.0, 0.0, 0.0, CXO[0], CXO[1]]) - Lanes([CXQ[0], CXQ[1], CXQ[2], CXQ[3], 0.0, 0.0])) / BBM) * RC;
                let CXU = (CXT * CXM) * RC;
                let CXV = (CXN * CXT) * RC;
                let CXX = ((CXT * CXF) / CXW) * RC;
                let CXY = ((CXG * CXT) / CXW) * RC;
                let CXZ = RC / BBM;
                let CYB = CYA - BBJ;
                let CYD = Lanes([0.0, 0.0, 0.0, 0.0, CYC]);
                let CYE = CYB * CXZ;
                let CYF = (((BBS * CXZ) * JM) / BBM) * CYB;
                let CYG = ((CYD - Lanes([BBP[0], BBP[1], BBP[2], BBP[3], 0.0])) * CXZ) + Lanes([CYF[0], CYF[1], CYF[2], CYF[3], 0.0]);
                let CYI = (CYH * CYA) * RC;
                let CYJ = (CYC * CYH) * RC;
                let CYK = Lanes([CXL[0], CXL[1], CXL[2], CXL[3], 0.0, CXL[4]]);
                CYT = CYA;
                CYU = CXF;
                CYV = CXK;
                CYW = CXU;
                CYX = CXR;
                CYY = CXX;
                CYZ = CYE;
                CZA = CYI;
                CZB = CYD;
                CZC = CXH;
                CZD = CYK;
                CZE = CXV;
                CZF = CXS;
                CZG = CXY;
                CZH = CYG;
                CZI = CYJ;
            } else {
                let CYL = Lanes([BBP[0], BBP[1], BBP[2], BBP[3], 0.0]);
                let CYM = Lanes([BBN[0], BBN[1], BBN[2], BBN[3], 0.0]);
                let CYN = Lanes([0.0, 0.0, 0.0, 0.0, CXN, 0.0]);
                let CYO = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, CXG]);
                let CYP = Lanes([0.0, 0.0, 0.0, 0.0, CYC]);
                CYT = BBJ;
                CYU = BBH;
                CYV = CXM;
                CYW = AR;
                CYX = CXF;
                CYY = AR;
                CYZ = CYA;
                CZA = AR;
                CZB = CYL;
                CZC = CYM;
                CZD = CYN;
                CZE = CYQ;
                CZF = CYO;
                CZG = CYR;
                CZH = CYP;
                CZI = CYS;
            }
            let CZJ = (AEH + AEX) + CCJ;
            let CZK = F * AS;
            let CZL = (G * (((AEG + AEW) + CCH) + BVJ)) + (AS * C);
            let CZM = ((Lanes([CZJ[0], 0.0, CZJ[1], CZJ[2]]) + BVK) * G) + Lanes([0.0, 0.0, CZK[0], CZK[1]]);
            let CZN = G * (CYT + AHA);
            let CZO = (CZB + Lanes([AHC[0], 0.0, AHC[1], AHC[2], 0.0])) * G;
            let CZP = ddt(20499, CZN);
            let CZR = CZO * CZQ;
            let CZS = M * AS;
            let CZT = (G * (BUB - BVG)) + (AS * K);
            let CZU = ((Lanes([BUC[0], BUC[1], 0.0, BUC[2]]) - BVH) * G) + Lanes([0.0, CZS[0], 0.0, CZS[1]]);
            let CZV = G * (BBK + AHH);
            let CZW = (BBQ + Lanes([AHJ[0], AHJ[1], 0.0, AHJ[2]])) * G;
            let CZX = ddt(20514, CZV);
            let CZY = CZW * CZQ;
            let CZZ = G * CYU;
            let DAA = CZC * G;
            let DAB = G * BBI;
            let DAC = BBO * G;
            let DAJ;
            let DAK;
            let DAL;
            let DAM;
            let DAN;
            let DAO;
            if DAD != 0.0 {
                let DAE = BCA / BYF;
                let DAF = BYG * DAE;
                let DAG = (Lanes([0.0, 0.0, 0.0, BCB[0], BCB[1]]) - Lanes([DAF[0], DAF[1], DAF[2], 0.0, DAF[3]])) / BYF;
                let DAS;
                let DAT;
                let DAU;
                let DAV;
                if DAH != 0.0 {
                    let DAQ = ddt(20535, BCC);
                    let DAR = BCF * CZQ;
                    DAS = DAQ;
                    DAT = BCC;
                    DAU = DAR;
                    DAV = BCF;
                } else {
                    DAS = AR;
                    DAT = AR;
                    DAU = DAI;
                    DAV = DAI;
                }
                DAJ = DAE;
                DAK = DAS;
                DAL = DAT;
                DAM = DAG;
                DAN = DAU;
                DAO = DAV;
            } else {
                DAJ = AR;
                DAK = AR;
                DAL = AR;
                DAM = DAI;
                DAN = DAI;
                DAO = DAI;
            }
            let DBC;
            let DBD;
            let DBE;
            let DBF;
            if DAP != 0.0 {
                let DAX = DAW * CCD;
                let DAY = CCE * DAW;
                DBC = DAX;
                DBD = AR;
                DBE = DAY;
                DBF = CCC;
            } else {
                let DBA = DAZ * CCD;
                let DBB = CCE * DAZ;
                DBC = AR;
                DBD = DBA;
                DBE = CCC;
                DBF = DBB;
            }
            let DBH = DBG * BUN;
            let DBI = BUO * DBG;
            let DBJ = G * (BZN + CAD);
            let DBK = (BZO + CAE) * G;
            let DBL = G * CBY;
            let DBM = CCA * G;
            let DBN = ddt(20560, DBL);
            let DBO = DBM * CZQ;
            let DBP = G * CHJ;
            let DBQ = CHK * G;
            let DBR = G * (CDS + CTC);
            let DBS = (CDT + CTE) * G;
            let DBT = ddt(20569, DBR);
            let DBU = DBS * CZQ;
            let DBW = DBV * V;
            let DBX = W * DBV;
            let DBY = ddt(20573, DBW);
            let DBZ = DBX * CZQ;
            let DCA = G * CHW;
            let DCB = CHX * G;
            let DCC = ddt(20577, DCA);
            let DCD = DCB * CZQ;
            let DCF = DCE * AA;
            let DCG = AC * DCE;
            let DCH = ddt(20581, DCF);
            let DCI = DCG * CZQ;
            let DCO;
            let DCP;
            if DCJ != 0.0 {
                let DCK = Lanes([AB, 0.0]) - Lanes([0.0, S]);
                let DCL = (Z - R) / GY;
                let DCM = (Lanes([DCK[0], 0.0, DCK[1]]) - Lanes([0.0, (JB * DCL), 0.0])) / GY;
                DCO = DCL;
                DCP = DCM;
            } else {
                DCO = AR;
                DCP = DCN;
            }
            let DCV;
            let DCW;
            if DCQ != 0.0 {
                let DCR = Lanes([0.0, E]) - Lanes([CVZ, 0.0]);
                let DCS = (B - CVX) / GW;
                let DCT = (Lanes([DCR[0], 0.0, DCR[1]]) - Lanes([0.0, (IZ * DCS), 0.0])) / GW;
                DCV = DCS;
                DCW = DCT;
            } else {
                DCV = AR;
                DCW = DCU;
            }
            let DDC;
            let DDD;
            if DCX != 0.0 {
                let DCY = Lanes([0.0, L]) - Lanes([AO, 0.0]);
                let DCZ = (J - AM) / GX;
                let DDA = (Lanes([DCY[0], 0.0, DCY[1]]) - Lanes([0.0, (JA * DCZ), 0.0])) / GX;
                DDC = DCZ;
                DDD = DDA;
            } else {
                DDC = AR;
                DDD = DDB;
            }
            let DDF = DDE * (R - CVX);
            let DDG = (Lanes([0.0, S]) - Lanes([CVZ, 0.0])) * DDE;
            let DDH = ddt(20618, DDF);
            let DDI = DDG * CZQ;
            let DDK = DDJ * (Z - CVX);
            let DDL = (Lanes([AB, 0.0]) - Lanes([0.0, CVZ])) * DDJ;
            let DDM = ddt(20622, DDK);
            let DDN = DDL * CZQ;
            let DDP = DDO * (AM - CVX);
            let DDQ = (Lanes([AO, 0.0]) - Lanes([0.0, CVZ])) * DDO;
            let DDR = ddt(20626, DDP);
            let DDS = DDQ * CZQ;
            let DDT = G * CTD;
            let DDU = CTF * G;
            let DDX;
            let DDY;
            let DDZ;
            let DEA;
            let DEB;
            let DEC;
            let DED;
            let DEE;
            if ASB != 0.0 {
                let DET;
                let DEU;
                let DEV;
                let DEW;
                if CTG != 0.0 {
                    let DEO = G * CTR;
                    let DEP = CTS * G;
                    let DEQ = AS * AG;
                    let DER = AI * AS;
                    DET = DEO;
                    DEU = DEQ;
                    DEV = DEP;
                    DEW = DER;
                } else {
                    DET = AR;
                    DEU = AR;
                    DEV = CMG;
                    DEW = DES;
                }
                DDX = DET;
                DDY = DEU;
                DDZ = AR;
                DEA = AR;
                DEB = DEV;
                DEC = DEW;
                DED = CMG;
                DEE = DES;
            } else {
                let DDV = G * CTR;
                let DDW = CTS * G;
                let DEZ;
                let DFA;
                if ARA != 0.0 {
                    let DEX = AS * AG;
                    let DEY = AI * AS;
                    DEZ = DEX;
                    DFA = DEY;
                } else {
                    DEZ = AR;
                    DFA = DES;
                }
                DDX = AR;
                DDY = AR;
                DDZ = DDV;
                DEA = DEZ;
                DEB = CMG;
                DEC = DES;
                DED = DDW;
                DEE = DFA;
            }
            let DEF = G * CLJ;
            let DEG = CLK * G;
            let DEH = ddt(20654, DEF);
            let DEI = DEG * CZQ;
            let DEJ = G * COY;
            let DEK = COZ * G;
            let DEL = ddt(20658, DEJ);
            let DEM = DEK * CZQ;
            let DFI;
            let DFJ;
            let DFK;
            let DFL;
            let DFM;
            let DFN;
            if DEN != 0.0 {
                let DFB = AF - AL;
                let DFC = Lanes([0.0, AH]) - Lanes([AN, 0.0]);
                let DFE = DFB / DFD;
                let DFF = DFC / DFD;
                let DFU;
                let DFV;
                let DFW;
                let DFX;
                if DFG != 0.0 {
                    let DFQ = DFP * DFB;
                    let DFR = DFC * DFP;
                    let DFS = ddt(20675, DFQ);
                    let DFT = DFR * CZQ;
                    DFU = DFS;
                    DFV = DFQ;
                    DFW = DFT;
                    DFX = DFR;
                } else {
                    DFU = AR;
                    DFV = AR;
                    DFW = DFH;
                    DFX = DFH;
                }
                DFI = DFE;
                DFJ = DFU;
                DFK = DFV;
                DFL = DFF;
                DFM = DFW;
                DFN = DFX;
            } else {
                DFI = AR;
                DFJ = AR;
                DFK = AR;
                DFL = DFH;
                DFM = DFH;
                DFN = DFH;
            }
            let DGC;
            let DGD;
            let DGE;
            let DGF;
            let DGG;
            let DGH;
            if DFO != 0.0 {
                let DFY = CZ / GZ;
                let DFZ = DFY - CUG;
                let DGA = Lanes([0.0, 0.0, 0.0, ((KK - (JC * DFY)) / GZ), 0.0, 0.0, 0.0, 0.0, 0.0]) - CUH;
                let DGT;
                let DGU;
                let DGV;
                let DGW;
                if DGB != 0.0 {
                    let DGP = DGO * CZ;
                    let DGQ = KK * DGO;
                    let DGR = ddt(20699, DGP);
                    let DGS = DGQ * CZQ;
                    DGT = DGR;
                    DGU = DGP;
                    DGV = DGS;
                    DGW = DGQ;
                } else {
                    DGT = AR;
                    DGU = AR;
                    DGV = EW;
                    DGW = EW;
                }
                DGC = DFZ;
                DGD = DGT;
                DGE = DGU;
                DGF = DGA;
                DGG = DGV;
                DGH = DGW;
            } else {
                DGC = AR;
                DGD = AR;
                DGE = AR;
                DGF = CUF;
                DGG = EW;
                DGH = EW;
            }
            let DGI = ddt(20703, CYW);
            let DGJ = CZE * CZQ;
            let DGK = ddt(20706, CYY);
            let DGL = CZG * CZQ;
            let DGM = ddt(20709, CZA);
            let DGN = CZI * CZQ;
            let DHH;
            let DHI;
            let DHJ;
            let DHK;
            let DHL;
            let DHM;
            let DHN;
            let DHO;
            let DHP;
            let DHQ;
            let DHR;
            let DHS;
            let DHT;
            let DHU;
            let DHV;
            let DHW;
            let DHX;
            let DHY;
            let DHZ;
            let DIA;
            if DGX != 0.0 {
                let DGY = if AEG > AR { 1.0 } else { 0.0 };
                let DIG;
                let DIH;
                if DGY != 0.0 {
                    let DIC = BBT / AEG;
                    let DID = AEH * DIC;
                    let DIE = (BBU - Lanes([DID[0], 0.0, DID[1], DID[2]])) / AEG;
                    DIG = DIC;
                    DIH = DIE;
                } else {
                    DIG = DIF;
                    DIH = ASQ;
                }
                let DII = BBM * CXT;
                let DIJ = BBS * CXT;
                let DIL = DIG * DIK;
                let DIM = DIH * DIK;
                let DIN = if DIL > AR { 1.0 } else { 0.0 };
                let DIR;
                let DIS;
                if DIN != 0.0 {
                    let DIO = DIL.sqrt();
                    let DIP = BBM * DIO;
                    let DIQ = (BBS * DIO) + ((DIM * (JV / (KZ * DIO))) * BBM);
                    DIR = DIP;
                    DIS = DIQ;
                } else {
                    DIR = AR;
                    DIS = ASQ;
                }
                let DIT = -DGZ;
                let DIU = DHF * JM;
                let DIV = ddt(20916, DGZ);
                let DIW = DIR * DIV;
                let DIX = DIS * DIV;
                let DIY = Lanes([DIX[0], DIX[1], DIX[2], DIX[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, ((DHF * CZQ) * DIR)]);
                let DIZ = DIR * DGZ;
                let DJA = DIS * DGZ;
                let DJB = Lanes([DJA[0], DJA[1], DJA[2], DJA[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, (DHF * DIR)]);
                let DJC = ddt(20924, DHA);
                let DJD = DII * DJC;
                let DJE = DIJ * DJC;
                let DJF = Lanes([DJE[0], DJE[1], DJE[2], DJE[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, ((DHG * CZQ) * DII)]);
                let DJG = DII * DHA;
                let DJH = DIJ * DHA;
                let DJI = Lanes([DJH[0], DJH[1], DJH[2], DJH[3], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, (DHG * DII)]);
                let DJJ = -DHA;
                let DJK = DHG * JM;
                DHH = DIT;
                DHI = DGZ;
                DHJ = DIW;
                DHK = DJD;
                DHL = DJJ;
                DHM = DHA;
                DHN = AR;
                DHO = AR;
                DHP = DIZ;
                DHQ = DJG;
                DHR = DIU;
                DHS = DHF;
                DHT = DIY;
                DHU = DJF;
                DHV = DJK;
                DHW = DHG;
                DHX = DHB;
                DHY = DHE;
                DHZ = DJB;
                DIA = DJI;
            } else {
                DHH = AR;
                DHI = AR;
                DHJ = AR;
                DHK = AR;
                DHL = AR;
                DHM = AR;
                DHN = DGZ;
                DHO = DHA;
                DHP = AR;
                DHQ = AR;
                DHR = DHB;
                DHS = DHB;
                DHT = DHC;
                DHU = DHD;
                DHV = DHE;
                DHW = DHE;
                DHX = DHF;
                DHY = DHG;
                DHZ = DHC;
                DIA = DHD;
            }
            let DIB = if (((((DCC + DCH) + DCO) + DDM) + staged[83]) + branch_unknown_flows[1]) != AR { 1.0 } else { 0.0 };
            let DJL = (((((-AEH[1]) - AEX[1]) - ((-CCE[1]) - CCJ[1])) + (-BVK[2])) - (-BVH[2])) + ((-BZO[1]) - CAE[1]);
            let DJM = if (DJL.abs()) > AS { 1.0 } else { 0.0 };
            if DJM != 0.0 {
            } else {
                let DJN = if DJL >= AR { 1.0 } else { 0.0 };
            }
            let DJO = -BVH[1];
            let DJP = ((((-BUC[1]) - (-BUO[1])) - DJO) + (-BVK[1])) + (-CHK[1]);
            let DJQ = if (DJP.abs()) > AS { 1.0 } else { 0.0 };
            if DJQ != 0.0 {
            } else {
                let DJR = if DJP >= AR { 1.0 } else { 0.0 };
            }
            let DJS = BBU[1] - DJO;
            let DJT = if (DJS.abs()) > AS { 1.0 } else { 0.0 };
            if DJT != 0.0 {
            } else {
                let DJU = if DJS >= AR { 1.0 } else { 0.0 };
            }
            let DJV = CZM[0];
            let DJW = CZM[1];
            let DJX = CZM[2];
            let DJY = CZM[3];
            let DJZ = CZR[0];
            let DKA = CZR[1];
            let DKB = CZR[2];
            let DKC = CZR[3];
            let DKD = CZR[4];
            let DKE = CZU[0];
            let DKF = CZU[1];
            let DKG = CZU[2];
            let DKH = CZU[3];
            let DKI = CZY[0];
            let DKJ = CZY[1];
            let DKK = CZY[2];
            let DKL = CZY[3];
            let DKM = DAA[0];
            let DKN = DAA[1];
            let DKO = DAA[2];
            let DKP = DAA[3];
            let DKQ = DAA[4];
            let DKR = DAC[0];
            let DKS = DAC[1];
            let DKT = DAC[2];
            let DKU = DAC[3];
            let DKV = DAM[0];
            let DKW = DAM[1];
            let DKX = DAM[2];
            let DKY = DAM[3];
            let DKZ = DAM[4];
            let DLA = DAN[0];
            let DLB = DAN[1];
            let DLC = DAN[2];
            let DLD = DAN[3];
            let DLE = DAN[4];
            let DLF = DBE[0];
            let DLG = DBE[1];
            let DLH = DBE[2];
            let DLI = DBE[3];
            let DLJ = DBF[0];
            let DLK = DBF[1];
            let DLL = DBF[2];
            let DLM = DBF[3];
            let DLN = DBI[0];
            let DLO = DBI[1];
            let DLP = DBI[2];
            let DLQ = DBK[0];
            let DLR = DBK[1];
            let DLS = DBK[2];
            let DLT = DBO[0];
            let DLU = DBO[1];
            let DLV = DBO[2];
            let DLW = DBQ[0];
            let DLX = DBQ[1];
            let DLY = DBQ[2];
            let DLZ = DBU[0];
            let DMA = DBU[1];
            let DMB = DBU[2];
            let DMC = DBZ[0];
            let DMD = DBZ[1];
            let DME = DCD[0];
            let DMF = DCD[1];
            let DMG = DCD[2];
            let DMH = DCI[0];
            let DMI = DCI[1];
            let DMJ = DCP[0];
            let DMK = DCP[1];
            let DML = DCP[2];
            let DMM = DCW[0];
            let DMN = DCW[1];
            let DMO = DCW[2];
            let DMP = DDD[0];
            let DMQ = DDD[1];
            let DMR = DDD[2];
            let DMS = DDI[0];
            let DMT = DDI[1];
            let DMU = DDN[0];
            let DMV = DDN[1];
            let DMW = DDS[0];
            let DMX = DDS[1];
            let DMY = DDU[0];
            let DMZ = DDU[1];
            let DNA = DDU[2];
            let DNB = DDU[3];
            let DNC = DEB[0];
            let DND = DEB[1];
            let DNE = DEB[2];
            let DNF = DEC[0];
            let DNG = DEC[1];
            let DNH = DED[0];
            let DNI = DED[1];
            let DNJ = DED[2];
            let DNK = DEE[0];
            let DNL = DEE[1];
            let DNM = DEI[0];
            let DNN = DEI[1];
            let DNO = DEI[2];
            let DNP = DEM[0];
            let DNQ = DEM[1];
            let DNR = DEM[2];
            let DNS = DFL[0];
            let DNT = DFL[1];
            let DNU = DFM[0];
            let DNV = DFM[1];
            let DNW = DGF[0];
            let DNX = DGF[1];
            let DNY = DGF[2];
            let DNZ = DGF[3];
            let DOA = DGF[4];
            let DOB = DGF[5];
            let DOC = DGF[6];
            let DOD = DGF[7];
            let DOE = DGF[8];
            let DOF = DGG;
            let DOG = CZD[0];
            let DOH = CZD[1];
            let DOI = CZD[2];
            let DOJ = CZD[3];
            let DOK = CZD[4];
            let DOL = CZD[5];
            let DOM = DGJ;
            let DON = CZF[0];
            let DOO = CZF[1];
            let DOP = CZF[2];
            let DOQ = CZF[3];
            let DOR = CZF[4];
            let DOS = CZF[5];
            let DOT = DGL;
            let DOU = CZH[0];
            let DOV = CZH[1];
            let DOW = CZH[2];
            let DOX = CZH[3];
            let DOY = CZH[4];
            let DOZ = DGN;
            let DPA = DHR;
            let DPB = DHS;
            let DPC = DHT[0];
            let DPD = DHT[1];
            let DPE = DHT[2];
            let DPF = DHT[3];
            let DPG = DHT[4];
            let DPH = DHU[0];
            let DPI = DHU[1];
            let DPJ = DHU[2];
            let DPK = DHU[3];
            let DPL = DHU[4];
            let DPM = DHV;
            let DPN = DHW;
            let DPO = DHX;
            let DPP = DHY;
            let DPQ = CZO[0];
            let DPR = CZO[1];
            let DPS = CZO[2];
            let DPT = CZO[3];
            let DPU = CZO[4];
            let DPV = CZW[0];
            let DPW = CZW[1];
            let DPX = CZW[2];
            let DPY = CZW[3];
            let DPZ = DAO[0];
            let DQA = DAO[1];
            let DQB = DAO[2];
            let DQC = DAO[3];
            let DQD = DAO[4];
            let DQE = DBM[0];
            let DQF = DBM[1];
            let DQG = DBM[2];
            let DQH = DBS[0];
            let DQI = DBS[1];
            let DQJ = DBS[2];
            let DQK = DBX[0];
            let DQL = DBX[1];
            let DQM = DCB[0];
            let DQN = DCB[1];
            let DQO = DCB[2];
            let DQP = DCG[0];
            let DQQ = DCG[1];
            let DQR = DDG[0];
            let DQS = DDG[1];
            let DQT = DDL[0];
            let DQU = DDL[1];
            let DQV = DDQ[0];
            let DQW = DDQ[1];
            let DQX = DEG[0];
            let DQY = DEG[1];
            let DQZ = DEG[2];
            let DRA = DEK[0];
            let DRB = DEK[1];
            let DRC = DEK[2];
            let DRD = DFN[0];
            let DRE = DFN[1];
            let DRF = DGH;
            let DRG = CZE;
            let DRH = CZG;
            let DRI = CZI;
            let DRJ = DHZ[0];
            let DRK = DHZ[1];
            let DRL = DHZ[2];
            let DRM = DHZ[3];
            let DRN = DHZ[4];
            let DRO = DIA[0];
            let DRP = DIA[1];
            let DRQ = DIA[2];
            let DRR = DIA[3];
            let DRS = DIA[4];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (CZL),
            [4, 5, 6, 8],
            [DJV, DJW, DJX, DJY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (CZP),
            [4, 5, 6, 8, 12],
            [DJZ, DKA, DKB, DKC, DKD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (CZT),
            [4, 5, 6, 8],
            [DKE, DKF, DKG, DKH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (CZX),
            [4, 5, 6, 8],
            [DKI, DKJ, DKK, DKL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (CZZ),
            [4, 5, 6, 8, 11],
            [DKM, DKN, DKO, DKP, DKQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(5),
            multiplicity * (DAB),
            [4, 5, 6, 8],
            [DKR, DKS, DKT, DKU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (DAJ),
            [4, 5, 6, 7, 8],
            [DKV, DKW, DKX, DKY, DKZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (DAK),
            [4, 5, 6, 7, 8],
            [DLA, DLB, DLC, DLD, DLE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(8), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[211],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (DBC),
            [4, 6, 7, 8],
            [DLF, DLG, DLH, DLI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (DBD),
            [4, 6, 7, 8],
            [DLJ, DLK, DLL, DLM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (DBH),
            [4, 5, 8],
            [DLN, DLO, DLP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(6),
            multiplicity * (DBJ),
            [4, 6, 7],
            [DLQ, DLR, DLS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(6),
            multiplicity * (DBN),
            [4, 6, 7],
            [DLT, DLU, DLV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (DBP),
            [4, 5, 7],
            [DLW, DLX, DLY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (DBT),
            [4, 5, 7],
            [DLZ, DMA, DMB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(5),
            multiplicity * (DBY),
            [5, 7],
            [DMC, DMD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (DCC),
            [1, 4, 5],
            [DME, DMF, DMG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (DCH),
            [1, 5],
            [DMH, DMI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(7),
            multiplicity * (DCO),
            [1, 4, 7],
            [DMJ, DMK, DML],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(7), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[212],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(2),
            multiplicity * (DCV),
            [2, 4, 6],
            [DMM, DMN, DMO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(6), Some(2), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[213],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(0),
            multiplicity * (DDC),
            [0, 4, 5],
            [DMP, DMQ, DMR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(0), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[214],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(2),
            multiplicity * (DDH),
            [2, 7],
            [DMS, DMT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (DDM),
            [1, 2],
            [DMU, DMV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(2),
            multiplicity * (DDR),
            [0, 2],
            [DMW, DMX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (DDT),
            [4, 5, 7, 9],
            [DMY, DMZ, DNA, DNB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(5),
            multiplicity * (DDX),
            [4, 5, 9],
            [DNC, DND, DNE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(5),
            multiplicity * (DDY),
            [5, 9],
            [DNF, DNG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(5),
            multiplicity * (DDZ),
            [4, 5, 9],
            [DNH, DNI, DNJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(5),
            multiplicity * (DEA),
            [5, 9],
            [DNK, DNL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(5),
            multiplicity * (DEH),
            [4, 5, 9],
            [DNM, DNN, DNO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(0),
            multiplicity * (DEL),
            [0, 3, 4],
            [DNP, DNQ, DNR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(3),
            multiplicity * (DFI),
            [3, 9],
            [DNS, DNT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(3),
            multiplicity * (DFJ),
            [3, 9],
            [DNU, DNV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(3), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[215],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            None,
            multiplicity * (DGC),
            [0, 1, 2, 4, 5, 6, 7, 8, 9],
            [DNW, DNX, DNY, DNZ, DOA, DOB, DOC, DOD, DOE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (DGD),
            [4],
            [DOF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[216],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (CYV),
            [4, 5, 6, 8, 10, 11],
            [DOG, DOH, DOI, DOJ, DOK, DOL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (DGI),
            [10],
            [DOM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            None,
            multiplicity * (CYX),
            [4, 5, 6, 8, 10, 11],
            [DON, DOO, DOP, DOQ, DOR, DOS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (DGK),
            [11],
            [DOT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (CYZ),
            [4, 5, 6, 8, 12],
            [DOU, DOV, DOW, DOX, DOY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (DGM),
            [12],
            [DOZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(7),
            multiplicity * (staged[83]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[217]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(0),
            multiplicity * (staged[218]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(2),
            multiplicity * (staged[219]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(3),
            multiplicity * (staged[220]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[221]),
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
            Some(6),
            Some(2),
            multiplicity * (staged[223]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[224]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (DRT),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (DRU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (DRV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (DRW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (DRX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(5),
            multiplicity * (DRY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(13),
            None,
            multiplicity * (staged[225]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (DHH),
            [13],
            [DPA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(6),
            multiplicity * (DHI),
            [13],
            [DPB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (DHJ),
            [4, 5, 6, 8, 13],
            [DPC, DPD, DPE, DPF, DPG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (DHK),
            [4, 5, 6, 8, 14],
            [DPH, DPI, DPJ, DPK, DPL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            None,
            multiplicity * (staged[226]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (DHL),
            [14],
            [DPM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            Some(6),
            multiplicity * (DHM),
            [14],
            [DPN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (staged[227]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[228]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (DHN),
            [13],
            [DPO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (DHO),
            [14],
            [DPP],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = CZL;
        self.canonical_reactive[1] = CZN;
        self.canonical_reactive[2] = DPQ;
        self.canonical_reactive[3] = DPR;
        self.canonical_reactive[4] = DPS;
        self.canonical_reactive[5] = DPT;
        self.canonical_reactive[6] = DPU;
        self.canonical_reactive[7] = CZT;
        self.canonical_reactive[8] = CZV;
        self.canonical_reactive[9] = DPV;
        self.canonical_reactive[10] = DPW;
        self.canonical_reactive[11] = DPX;
        self.canonical_reactive[12] = DPY;
        self.canonical_reactive[13] = CZZ;
        self.canonical_reactive[14] = DAB;
        self.canonical_reactive[15] = DAJ;
        self.canonical_reactive[16] = DAL;
        self.canonical_reactive[17] = DPZ;
        self.canonical_reactive[18] = DQA;
        self.canonical_reactive[19] = DQB;
        self.canonical_reactive[20] = DQC;
        self.canonical_reactive[21] = DQD;
        self.canonical_reactive[22] = staged[211];
        self.canonical_reactive[23] = DBC;
        self.canonical_reactive[24] = DBD;
        self.canonical_reactive[25] = DBH;
        self.canonical_reactive[26] = DBJ;
        self.canonical_reactive[27] = DBL;
        self.canonical_reactive[28] = DQE;
        self.canonical_reactive[29] = DQF;
        self.canonical_reactive[30] = DQG;
        self.canonical_reactive[31] = DBP;
        self.canonical_reactive[32] = DBR;
        self.canonical_reactive[33] = DQH;
        self.canonical_reactive[34] = DQI;
        self.canonical_reactive[35] = DQJ;
        self.canonical_reactive[36] = DBW;
        self.canonical_reactive[37] = DQK;
        self.canonical_reactive[38] = DQL;
        self.canonical_reactive[39] = DCA;
        self.canonical_reactive[40] = DQM;
        self.canonical_reactive[41] = DQN;
        self.canonical_reactive[42] = DQO;
        self.canonical_reactive[43] = DCF;
        self.canonical_reactive[44] = DQP;
        self.canonical_reactive[45] = DQQ;
        self.canonical_reactive[46] = DCO;
        self.canonical_reactive[47] = staged[212];
        self.canonical_reactive[48] = DCV;
        self.canonical_reactive[49] = staged[213];
        self.canonical_reactive[50] = DDC;
        self.canonical_reactive[51] = staged[214];
        self.canonical_reactive[52] = DDF;
        self.canonical_reactive[53] = DQR;
        self.canonical_reactive[54] = DQS;
        self.canonical_reactive[55] = DDK;
        self.canonical_reactive[56] = DQT;
        self.canonical_reactive[57] = DQU;
        self.canonical_reactive[58] = DDP;
        self.canonical_reactive[59] = DQV;
        self.canonical_reactive[60] = DQW;
        self.canonical_reactive[61] = DDT;
        self.canonical_reactive[62] = DDX;
        self.canonical_reactive[63] = DDY;
        self.canonical_reactive[64] = DDZ;
        self.canonical_reactive[65] = DEA;
        self.canonical_reactive[66] = DEF;
        self.canonical_reactive[67] = DQX;
        self.canonical_reactive[68] = DQY;
        self.canonical_reactive[69] = DQZ;
        self.canonical_reactive[70] = DEJ;
        self.canonical_reactive[71] = DRA;
        self.canonical_reactive[72] = DRB;
        self.canonical_reactive[73] = DRC;
        self.canonical_reactive[74] = DFI;
        self.canonical_reactive[75] = DFK;
        self.canonical_reactive[76] = DRD;
        self.canonical_reactive[77] = DRE;
        self.canonical_reactive[78] = staged[215];
        self.canonical_reactive[79] = DGC;
        self.canonical_reactive[80] = DGE;
        self.canonical_reactive[81] = DRF;
        self.canonical_reactive[82] = staged[216];
        self.canonical_reactive[83] = CYV;
        self.canonical_reactive[84] = CYW;
        self.canonical_reactive[85] = DRG;
        self.canonical_reactive[86] = CYX;
        self.canonical_reactive[87] = CYY;
        self.canonical_reactive[88] = DRH;
        self.canonical_reactive[89] = CYZ;
        self.canonical_reactive[90] = CZA;
        self.canonical_reactive[91] = DRI;
        self.canonical_reactive[92] = staged[83];
        self.canonical_reactive[93] = staged[217];
        self.canonical_reactive[94] = staged[218];
        self.canonical_reactive[95] = staged[219];
        self.canonical_reactive[96] = staged[220];
        self.canonical_reactive[97] = staged[221];
        self.canonical_reactive[98] = staged[222];
        self.canonical_reactive[99] = staged[223];
        self.canonical_reactive[100] = staged[224];
        self.canonical_reactive[101] = DRT;
        self.canonical_reactive[102] = DRU;
        self.canonical_reactive[103] = DRV;
        self.canonical_reactive[104] = DRW;
        self.canonical_reactive[105] = DRX;
        self.canonical_reactive[106] = DRY;
        self.canonical_reactive[107] = staged[225];
        self.canonical_reactive[108] = DHH;
        self.canonical_reactive[109] = DHI;
        self.canonical_reactive[110] = DHP;
        self.canonical_reactive[111] = DRJ;
        self.canonical_reactive[112] = DRK;
        self.canonical_reactive[113] = DRL;
        self.canonical_reactive[114] = DRM;
        self.canonical_reactive[115] = DRN;
        self.canonical_reactive[116] = DHQ;
        self.canonical_reactive[117] = DRO;
        self.canonical_reactive[118] = DRP;
        self.canonical_reactive[119] = DRQ;
        self.canonical_reactive[120] = DRR;
        self.canonical_reactive[121] = DRS;
        self.canonical_reactive[122] = staged[226];
        self.canonical_reactive[123] = DHL;
        self.canonical_reactive[124] = DHM;
        self.canonical_reactive[125] = staged[227];
        self.canonical_reactive[126] = staged[228];
        self.canonical_reactive[127] = DHN;
        self.canonical_reactive[128] = DHO;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 5, 6, 8, 12],
            &[cached[2], cached[3], cached[4], cached[5], cached[6]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[4, 5, 6, 8],
            &[cached[9], cached[10], cached[11], cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(8),
            &[4, 5, 6, 7, 8],
            &[cached[17], cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(6),
            &[4, 6, 7],
            &[cached[28], cached[29], cached[30]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[4, 5, 7],
            &[cached[33], cached[34], cached[35]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[5, 7],
            &[cached[37], cached[38]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 4, 5],
            &[cached[40], cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(5),
            &[1, 5],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(2),
            &[2, 7],
            &[cached[53], cached[54]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[56], cached[57]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(2),
            &[0, 2],
            &[cached[59], cached[60]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(5),
            &[4, 5, 9],
            &[cached[67], cached[68], cached[69]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(0),
            &[0, 3, 4],
            &[cached[71], cached[72], cached[73]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(3),
            &[3, 9],
            &[cached[76], cached[77]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[81]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[85]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            None,
            &[11],
            &[cached[88]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            None,
            &[12],
            &[cached[91]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 5, 6, 8, 13],
            &[cached[111], cached[112], cached[113], cached[114], cached[115]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 5, 6, 8, 14],
            &[cached[117], cached[118], cached[119], cached[120], cached[121]],
            &[],
            &[],
            multiplicity,
        );
    }

}
