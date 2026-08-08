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
        let mut key = Vec::with_capacity(198);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[302] = values[0];
        self.canonical_staged[303] = values[1];
        self.canonical_staged[304] = values[2];
        self.canonical_staged[305] = values[3];
        self.canonical_staged[44] = values[4];
        self.canonical_staged[306] = values[5];
        self.canonical_staged[18] = values[6];
        self.canonical_staged[307] = values[7];
        self.canonical_staged[20] = values[8];
        self.canonical_staged[308] = values[9];
        self.canonical_staged[22] = values[10];
        self.canonical_staged[309] = values[11];
        self.canonical_staged[14] = values[12];
        self.canonical_staged[310] = values[13];
        self.canonical_staged[15] = values[14];
        self.canonical_staged[311] = values[15];
        self.canonical_staged[16] = values[16];
        self.canonical_staged[312] = values[17];
        self.canonical_staged[313] = values[18];
        self.canonical_staged[17] = values[19];
        self.canonical_staged[314] = values[20];
        self.canonical_staged[315] = values[21];
        self.canonical_staged[19] = values[22];
        self.canonical_staged[316] = values[23];
        self.canonical_staged[317] = values[24];
        self.canonical_staged[21] = values[25];
        self.canonical_staged[318] = values[26];
        self.canonical_staged[11] = values[27];
        self.canonical_staged[319] = values[28];
        self.canonical_staged[12] = values[29];
        self.canonical_staged[320] = values[30];
        self.canonical_staged[13] = values[31];
        self.canonical_staged[321] = values[32];
        self.canonical_staged[69] = values[33];
        self.canonical_staged[322] = values[34];
        self.canonical_staged[85] = values[35];
        self.canonical_staged[323] = values[36];
        self.canonical_staged[99] = values[37];
        self.canonical_staged[324] = values[38];
        self.canonical_staged[325] = values[39];
        self.canonical_staged[326] = values[40];
        self.canonical_staged[70] = values[41];
        self.canonical_staged[327] = values[42];
        self.canonical_staged[86] = values[43];
        self.canonical_staged[328] = values[44];
        self.canonical_staged[100] = values[45];
        self.canonical_staged[329] = values[46];
        self.canonical_staged[330] = values[47];
        self.canonical_staged[331] = values[48];
        self.canonical_staged[332] = values[49];
        self.canonical_staged[75] = values[50];
        self.canonical_staged[333] = values[51];
        self.canonical_staged[91] = values[52];
        self.canonical_staged[334] = values[53];
        self.canonical_staged[105] = values[54];
        self.canonical_staged[335] = values[55];
        self.canonical_staged[29] = values[56];
        self.canonical_staged[336] = values[57];
        self.canonical_staged[30] = values[58];
        self.canonical_staged[337] = values[59];
        self.canonical_staged[31] = values[60];
        self.canonical_staged[338] = values[61];
        self.canonical_staged[82] = values[62];
        self.canonical_staged[339] = values[63];
        self.canonical_staged[96] = values[64];
        self.canonical_staged[340] = values[65];
        self.canonical_staged[110] = values[66];
        self.canonical_staged[341] = values[67];
        self.canonical_staged[36] = values[68];
        self.canonical_staged[342] = values[69];
        self.canonical_staged[38] = values[70];
        self.canonical_staged[343] = values[71];
        self.canonical_staged[37] = values[72];
        self.canonical_staged[344] = values[73];
        self.canonical_staged[39] = values[74];
        self.canonical_staged[345] = values[75];
        self.canonical_staged[35] = values[76];
        self.canonical_staged[346] = values[77];
        self.canonical_staged[347] = values[78];
        self.canonical_staged[348] = values[79];
        self.canonical_staged[349] = values[80];
        self.canonical_staged[6] = values[81];
        self.canonical_staged[350] = values[82];
        self.canonical_staged[8] = values[83];
        self.canonical_staged[351] = values[84];
        self.canonical_staged[10] = values[85];
        self.canonical_staged[352] = values[86];
        self.canonical_staged[50] = values[87];
        self.canonical_staged[353] = values[88];
        self.canonical_staged[48] = values[89];
        self.canonical_staged[354] = values[90];
        self.canonical_staged[53] = values[91];
        self.canonical_staged[355] = values[92];
        self.canonical_staged[51] = values[93];
        self.canonical_staged[356] = values[94];
        self.canonical_staged[56] = values[95];
        self.canonical_staged[357] = values[96];
        self.canonical_staged[54] = values[97];
        self.canonical_staged[358] = values[98];
        self.canonical_staged[359] = values[99];
        self.canonical_staged[360] = values[100];
        self.canonical_staged[261] = values[101];
        self.canonical_staged[361] = values[102];
        self.canonical_staged[362] = values[103];
        self.canonical_staged[363] = values[104];
        self.canonical_staged[41] = values[105];
        self.canonical_staged[364] = values[106];
        self.canonical_staged[365] = values[107];
        self.canonical_staged[366] = values[108];
        self.canonical_staged[62] = values[109];
        self.canonical_staged[367] = values[110];
        self.canonical_staged[181] = values[111];
        self.canonical_staged[0] = values[112];
        self.canonical_staged[67] = values[113];
        self.canonical_staged[1] = values[114];
        self.canonical_staged[2] = values[115];
        self.canonical_staged[3] = values[116];
        self.canonical_staged[4] = values[117];
        self.canonical_staged[5] = values[118];
        self.canonical_staged[7] = values[119];
        self.canonical_staged[9] = values[120];
        self.canonical_staged[68] = values[121];
        self.canonical_staged[84] = values[122];
        self.canonical_staged[98] = values[123];
        self.canonical_staged[23] = values[124];
        self.canonical_staged[24] = values[125];
        self.canonical_staged[25] = values[126];
        self.canonical_staged[73] = values[127];
        self.canonical_staged[89] = values[128];
        self.canonical_staged[103] = values[129];
        self.canonical_staged[76] = values[130];
        self.canonical_staged[92] = values[131];
        self.canonical_staged[106] = values[132];
        self.canonical_staged[72] = values[133];
        self.canonical_staged[88] = values[134];
        self.canonical_staged[102] = values[135];
        self.canonical_staged[26] = values[136];
        self.canonical_staged[27] = values[137];
        self.canonical_staged[28] = values[138];
        self.canonical_staged[80] = values[139];
        self.canonical_staged[83] = values[140];
        self.canonical_staged[97] = values[141];
        self.canonical_staged[111] = values[142];
        self.canonical_staged[42] = values[143];
        self.canonical_staged[32] = values[144];
        self.canonical_staged[33] = values[145];
        self.canonical_staged[34] = values[146];
        self.canonical_staged[40] = values[147];
        self.canonical_staged[64] = values[148];
        self.canonical_staged[65] = values[149];
        self.canonical_staged[49] = values[150];
        self.canonical_staged[52] = values[151];
        self.canonical_staged[55] = values[152];
        self.canonical_staged[57] = values[153];
        self.canonical_staged[58] = values[154];
        self.canonical_staged[278] = values[155];
        self.canonical_staged[397] = values[156];
        self.canonical_staged[400] = values[157];
        self.canonical_staged[398] = values[158];
        self.canonical_staged[59] = values[159];
        self.canonical_staged[60] = values[160];
        self.canonical_staged[61] = values[161];
        self.canonical_staged[262] = values[162];
        self.canonical_staged[263] = values[163];
        self.canonical_staged[264] = values[164];
        self.canonical_staged[840] = values[165];
        self.canonical_staged[841] = values[166];
        self.canonical_staged[843] = values[167];
        self.canonical_staged[266] = values[168];
        self.canonical_staged[844] = values[169];
        self.canonical_staged[267] = values[170];
        self.canonical_staged[268] = values[171];
        self.canonical_staged[269] = values[172];
        self.canonical_staged[272] = values[173];
        self.canonical_staged[845] = values[174];
        self.canonical_staged[279] = values[175];
        self.canonical_staged[846] = values[176];
        self.canonical_staged[280] = values[177];
        self.canonical_staged[282] = values[178];
        self.canonical_staged[847] = values[179];
        self.canonical_staged[848] = values[180];
        self.canonical_staged[286] = values[181];
        self.canonical_staged[851] = values[182];
        self.canonical_staged[852] = values[183];
        self.canonical_staged[853] = values[184];
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
                let A = parameters[6];
                let C = -2.5e2f64;
                let F = parameters[96];
                let I = parameters[5];
                let J = 1e-12f64;
                let L = -2.5e2f64;
                let O = parameters[8];
                let R = parameters[9];
                let S = 1e-18f64;
                let V = parameters[10];
                let Y = parameters[11];
                let Z = 5e-2f64;
                let AC = parameters[12];
                let AF = parameters[13];
                let AI = parameters[14];
                let AK = 9.5e-1f64;
                let AN = parameters[15];
                let AS = parameters[16];
                let AX = parameters[20];
                let AY = 0e0f64;
                let BC = parameters[21];
                let BF = parameters[22];
                let BI = parameters[23];
                let BL = parameters[24];
                let BO = parameters[25];
                let BR = parameters[26];
                let BS = 1e-9f64;
                let BV = parameters[27];
                let BY = parameters[28];
                let CB = parameters[29];
                let CE = parameters[30];
                let CH = parameters[31];
                let CI = 1e-2f64;
                let CL = parameters[32];
                let CO = parameters[33];
                let CR = parameters[34];
                let CU = parameters[35];
                let CX = parameters[36];
                let DA = parameters[43];
                let DB = 1e-1f64;
                let DE = parameters[44];
                let DH = parameters[45];
                let DK = parameters[46];
                let DN = parameters[47];
                let DQ = parameters[48];
                let DT = parameters[49];
                let DW = parameters[50];
                let DZ = parameters[51];
                let EC = parameters[52];
                let EF = parameters[53];
                let EK = parameters[56];
                let EN = parameters[63];
                let EQ = parameters[64];
                let ET = parameters[65];
                let EW = parameters[66];
                let EZ = parameters[67];
                let FC = parameters[68];
                let FF = parameters[69];
                let FI = parameters[70];
                let FL = parameters[71];
                let FO = parameters[72];
                let FQ = -2.5e2f64;
                let FS = parameters[73];
                let FU = -2.5e2f64;
                let FW = parameters[74];
                let FZ = parameters[75];
                let GC = parameters[76];
                let GF = parameters[77];
                let GI = parameters[78];
                let GL = 5e-1f64;
                let GN = 1e0f64;
                let GP = parameters[82];
                let GS = parameters[83];
                let GV = 2.7315e2f64;
                let HN = 1.0447941624768001e-10f64;
                let HX = 3.2e1f64;
                let HY = 9.1093826e-31f64;
                let HZ = 1.6021918e-19f64;
                let ID = parameters[7];
                let IM = 1e6f64;
                let IW = parameters[94];
                let IX = 1e-7f64;
                let IZ = 4e0f64;
                let JG = 1e-6f64;
                let JO = parameters[85];
                let KE = parameters[92];
                let KI = parameters[95];
                let KN = 0e0f64;
                let KO = 0e0f64;
                let KS = 0e0f64;
                let mut oG = 0.0;
                let mut oAL = 0.0;
                let mut oAQ = 0.0;
                let mut oAV = 0.0;
                let mut oJH = 0.0;
                let mut oJI = 0.0;
                let mut oJJ = 0.0;
                let mut oJP = 0.0;
                let mut oJS = 0.0;
                let mut oJU = 0.0;
                let mut oJW = 0.0;
                let mut oJY = 0.0;
                let mut oKA = 0.0;
                let mut oKC = 0.0;
                let mut oKD = 0.0;
                let mut oKF = 0.0;
                let mut oKH = 0.0;
                let mut oKJ = 0.0;
                let mut oKK = 0.0;
                let mut oKL = 0.0;
                let B = if A > -2.5e2f64 { 1.0 } else { 0.0 };
                let D = if B != 0.0 {
                    A
                } else {
                    C
                };
                let E = if (if (if parameter_given[6] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[96] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let H;
                if E != 0.0 {
                    let G = if F > -2.5e2f64 { 1.0 } else { 0.0 };
                    oG = G;
                    let M = if G != 0.0 {
                        F
                    } else {
                        L
                    };
                    H = M;
                } else {
                    H = D;
                }
                let K = if I > J { 1.0 } else { 0.0 };
                let N = if K != 0.0 {
                    I
                } else {
                    J
                };
                let P = if O > J { 1.0 } else { 0.0 };
                let Q = if P != 0.0 {
                    O
                } else {
                    J
                };
                let T = if R > S { 1.0 } else { 0.0 };
                let U = if T != 0.0 {
                    R
                } else {
                    S
                };
                let W = if V > S { 1.0 } else { 0.0 };
                let X = if W != 0.0 {
                    V
                } else {
                    S
                };
                let AA = if Y > Z { 1.0 } else { 0.0 };
                let AB = if AA != 0.0 {
                    Y
                } else {
                    Z
                };
                let AD = if AC > Z { 1.0 } else { 0.0 };
                let AE = if AD != 0.0 {
                    AC
                } else {
                    Z
                };
                let AG = if AF > Z { 1.0 } else { 0.0 };
                let AH = if AG != 0.0 {
                    AF
                } else {
                    Z
                };
                let AJ = if AI > Z { 1.0 } else { 0.0 };
                let AM;
                if AJ != 0.0 {
                    let AL = if AI < AK { 1.0 } else { 0.0 };
                    oAL = AL;
                    let AP = if AL != 0.0 {
                        AI
                    } else {
                        AK
                    };
                    AM = AP;
                } else {
                    AM = Z;
                }
                let AO = if AN > Z { 1.0 } else { 0.0 };
                let AR;
                if AO != 0.0 {
                    let AQ = if AN < AK { 1.0 } else { 0.0 };
                    oAQ = AQ;
                    let AU = if AQ != 0.0 {
                        AN
                    } else {
                        AK
                    };
                    AR = AU;
                } else {
                    AR = Z;
                }
                let AT = if AS > Z { 1.0 } else { 0.0 };
                let AW;
                if AT != 0.0 {
                    let AV = if AS < AK { 1.0 } else { 0.0 };
                    oAV = AV;
                    let BA = if AV != 0.0 {
                        AS
                    } else {
                        AK
                    };
                    AW = BA;
                } else {
                    AW = Z;
                }
                let AZ = if AX > AY { 1.0 } else { 0.0 };
                let BB = if AZ != 0.0 {
                    AX
                } else {
                    AY
                };
                let BD = if BC > AY { 1.0 } else { 0.0 };
                let BE = if BD != 0.0 {
                    BC
                } else {
                    AY
                };
                let BG = if BF > AY { 1.0 } else { 0.0 };
                let BH = if BG != 0.0 {
                    BF
                } else {
                    AY
                };
                let BJ = if BI > AY { 1.0 } else { 0.0 };
                let BK = if BJ != 0.0 {
                    BI
                } else {
                    AY
                };
                let BM = if BL > AY { 1.0 } else { 0.0 };
                let BN = if BM != 0.0 {
                    BL
                } else {
                    AY
                };
                let BP = if BO > AY { 1.0 } else { 0.0 };
                let BQ = if BP != 0.0 {
                    BO
                } else {
                    AY
                };
                let BT = if BR > BS { 1.0 } else { 0.0 };
                let BU = if BT != 0.0 {
                    BR
                } else {
                    BS
                };
                let BW = if BV > BS { 1.0 } else { 0.0 };
                let BX = if BW != 0.0 {
                    BV
                } else {
                    BS
                };
                let BZ = if BY > AY { 1.0 } else { 0.0 };
                let CA = if BZ != 0.0 {
                    BY
                } else {
                    AY
                };
                let CC = if CB > AY { 1.0 } else { 0.0 };
                let CD = if CC != 0.0 {
                    CB
                } else {
                    AY
                };
                let CF = if CE > AY { 1.0 } else { 0.0 };
                let CG = if CF != 0.0 {
                    CE
                } else {
                    AY
                };
                let CJ = if CH > CI { 1.0 } else { 0.0 };
                let CK = if CJ != 0.0 {
                    CH
                } else {
                    CI
                };
                let CM = if CL > CI { 1.0 } else { 0.0 };
                let CN = if CM != 0.0 {
                    CL
                } else {
                    CI
                };
                let CP = if CO > CI { 1.0 } else { 0.0 };
                let CQ = if CP != 0.0 {
                    CO
                } else {
                    CI
                };
                let CS = if CR > AY { 1.0 } else { 0.0 };
                let CT = if CS != 0.0 {
                    CR
                } else {
                    AY
                };
                let CV = if CU > AY { 1.0 } else { 0.0 };
                let CW = if CV != 0.0 {
                    CU
                } else {
                    AY
                };
                let CY = if CX > AY { 1.0 } else { 0.0 };
                let CZ = if CY != 0.0 {
                    CX
                } else {
                    AY
                };
                let DC = if DA > DB { 1.0 } else { 0.0 };
                let DD = if DC != 0.0 {
                    DA
                } else {
                    DB
                };
                let DF = if DE > DB { 1.0 } else { 0.0 };
                let DG = if DF != 0.0 {
                    DE
                } else {
                    DB
                };
                let DI = if DH > DB { 1.0 } else { 0.0 };
                let DJ = if DI != 0.0 {
                    DH
                } else {
                    DB
                };
                let DL = if DK > DB { 1.0 } else { 0.0 };
                let DM = if DL != 0.0 {
                    DK
                } else {
                    DB
                };
                let DO = if DN > DB { 1.0 } else { 0.0 };
                let DP = if DO != 0.0 {
                    DN
                } else {
                    DB
                };
                let DR = if DQ > DB { 1.0 } else { 0.0 };
                let DS = if DR != 0.0 {
                    DQ
                } else {
                    DB
                };
                let DU = if DT > AY { 1.0 } else { 0.0 };
                let DV = if DU != 0.0 {
                    DT
                } else {
                    AY
                };
                let DX = if DW > AY { 1.0 } else { 0.0 };
                let DY = if DX != 0.0 {
                    DW
                } else {
                    AY
                };
                let EA = if DZ > AY { 1.0 } else { 0.0 };
                let EB = if EA != 0.0 {
                    DZ
                } else {
                    AY
                };
                let ED = if EC > AY { 1.0 } else { 0.0 };
                let EE = if ED != 0.0 {
                    EC
                } else {
                    AY
                };
                let EG = if EF > AY { 1.0 } else { 0.0 };
                let EH = if EG != 0.0 {
                    EF
                } else {
                    AY
                };
                let EI = if parameters[55] > DB { 1.0 } else { 0.0 };
                let EJ = if parameters[54] > AY { 1.0 } else { 0.0 };
                let EL = if EK > AY { 1.0 } else { 0.0 };
                let EM = if EL != 0.0 {
                    EK
                } else {
                    AY
                };
                let EO = if EN > DB { 1.0 } else { 0.0 };
                let EP = if EO != 0.0 {
                    EN
                } else {
                    DB
                };
                let ER = if EQ > DB { 1.0 } else { 0.0 };
                let ES = if ER != 0.0 {
                    EQ
                } else {
                    DB
                };
                let EU = if ET > DB { 1.0 } else { 0.0 };
                let EV = if EU != 0.0 {
                    ET
                } else {
                    DB
                };
                let EX = if EW > AY { 1.0 } else { 0.0 };
                let EY = if EX != 0.0 {
                    EW
                } else {
                    AY
                };
                let FA = if EZ > AY { 1.0 } else { 0.0 };
                let FB = if FA != 0.0 {
                    EZ
                } else {
                    AY
                };
                let FD = if FC > AY { 1.0 } else { 0.0 };
                let FE = if FD != 0.0 {
                    FC
                } else {
                    AY
                };
                let FG = if FF > AY { 1.0 } else { 0.0 };
                let FH = if FG != 0.0 {
                    FF
                } else {
                    AY
                };
                let FJ = if FI > AY { 1.0 } else { 0.0 };
                let FK = if FJ != 0.0 {
                    FI
                } else {
                    AY
                };
                let FM = if FL > AY { 1.0 } else { 0.0 };
                let FN = if FM != 0.0 {
                    FL
                } else {
                    AY
                };
                let FP = if FO > -2.5e2f64 { 1.0 } else { 0.0 };
                let FR = if FP != 0.0 {
                    FO
                } else {
                    FQ
                };
                let FT = if FS > -2.5e2f64 { 1.0 } else { 0.0 };
                let FV = if FT != 0.0 {
                    FS
                } else {
                    FU
                };
                let FX = if FW > AY { 1.0 } else { 0.0 };
                let FY = if FX != 0.0 {
                    FW
                } else {
                    AY
                };
                let GA = if FZ > AY { 1.0 } else { 0.0 };
                let GB = if GA != 0.0 {
                    FZ
                } else {
                    AY
                };
                let GD = if GC > DB { 1.0 } else { 0.0 };
                let GE = if GD != 0.0 {
                    GC
                } else {
                    DB
                };
                let GG = if GF > AY { 1.0 } else { 0.0 };
                let GH = if GG != 0.0 {
                    GF
                } else {
                    AY
                };
                let GJ = if GI > AY { 1.0 } else { 0.0 };
                let GK = if GJ != 0.0 {
                    GI
                } else {
                    AY
                };
                let GM = if parameters[81] > GL { 1.0 } else { 0.0 };
                let GO = if GM != 0.0 {
                    GN
                } else {
                    AY
                };
                let GQ = if GP > GL { 1.0 } else { 0.0 };
                let GR = if GQ != 0.0 {
                    GP
                } else {
                    GL
                };
                let GT = if GS > AY { 1.0 } else { 0.0 };
                let GU = if GT != 0.0 {
                    GS
                } else {
                    AY
                };
                let GW = GV + H;
                let GX = 8.61726105451295e-5f64 * GW;
                let GY = GN / GX;
                let GZ = (-((7.02e-4f64 * GW) * GW)) / (1.108e3f64 + GW);
                let HA = GE / 2e0f64;
                let HB = (parameters[17] + GZ) * GY;
                let HC = (parameters[18] + GZ) * GY;
                let HD = (parameters[19] + GZ) * GY;
                let HE = HA / EP;
                let HF = HA / ES;
                let HG = HA / EV;
                let HH = GN - AM;
                let HI = GN - AR;
                let HJ = GN - AW;
                let HK = GN / HH;
                let HL = GN / HI;
                let HM = GN / HJ;
                let HO = HN / Q;
                let HP = (BU * HN) / U;
                let HQ = (BX * HN) / X;
                let HR = GN / HO;
                let HS = GN / HP;
                let HT = GN / HQ;
                let HU = GN / AB;
                let HV = GN / AE;
                let HW = GN / AH;
                let IA = ((HX * CK) * HY) * HZ;
                let IB = ((HX * CN) * HY) * HZ;
                let IC = ((HX * CQ) * HY) * HZ;
                let IE = (ID - GN) / ID;
                let IF = GN / (GN - (IE.powf(DM)));
                let IG = GN / (GN - (IE.powf(DP)));
                let IH = GN / (GN - (IE.powf(DS)));
                let II = GN - (CI * GK);
                let IJ = (-((IF * IF) * (IE.powf((DM - GN))))) * DM;
                let IK = (-((IG * IG) * (IE.powf((DP - GN))))) * DP;
                let IL = (-((IH * IH) * (IE.powf((DS - GN))))) * DS;
                let IN = parameters[87] * IM;
                let IO = parameters[89] * IM;
                let IP = parameters[88] * IM;
                let IQ = if FB > S { 1.0 } else { 0.0 };
                let IR = if FH > J { 1.0 } else { 0.0 };
                let IS = if FN > J { 1.0 } else { 0.0 };
                let IT = FV + GV;
                let IU = FR + GV;
                let IV = HZ * IN;
                let IY = (IW - ((2.0895883249536002e-10f64 / IV).sqrt())) - IX;
                let JA = (IZ * IW) * IX;
                let JB = if JA > AY { 1.0 } else { 0.0 };
                let JD = if JB != 0.0 {
                    JA
                } else {
                    let JC = -JA;
                    JC
                };
                let JE = IW - (GL * (IY + (((IY * IY) + JD).sqrt())));
                let JF = if GO > 9e-1f64 { 1.0 } else { 0.0 };
                if JF != 0.0 {
                    let JH = if ((EP - EV).abs()) > JG { 1.0 } else { 0.0 };
                    oJH = JH;
                    let JI = if ((EP - ES).abs()) > JG { 1.0 } else { 0.0 };
                    oJI = JI;
                    let JJ = if ((EV - ES).abs()) > JG { 1.0 } else { 0.0 };
                    oJJ = JJ;
                } else {
                }
                let JK = if FY > J { 1.0 } else { 0.0 };
                let JL = -1e0f64 * GB;
                let JM = if GB > J { 1.0 } else { 0.0 };
                let JN = if parameters[84] > AY { 1.0 } else { 0.0 };
                let JQ;
                if JN != 0.0 {
                    let JP = if EP < JO { 1.0 } else { 0.0 };
                    oJP = JP;
                    if JP != 0.0 {
                        let JR = (IZ * JO) * CI;
                        let JS = if JR > AY { 1.0 } else { 0.0 };
                        oJS = JS;
                        let JU = if JS != 0.0 {
                            JR
                        } else {
                            let JT = -JR;
                            JT
                        };
                        oJU = JU;
                        let JV = (IZ * EP) * CI;
                        let JW = if JV > AY { 1.0 } else { 0.0 };
                        oJW = JW;
                        let JY = if JW != 0.0 {
                            JV
                        } else {
                            let JX = -JV;
                            JX
                        };
                        oJY = JY;
                        let KA = if JS != 0.0 {
                            JR
                        } else {
                            let JZ = -JR;
                            JZ
                        };
                        oKA = KA;
                        let KC = if JW != 0.0 {
                            JV
                        } else {
                            let KB = -JV;
                            KB
                        };
                        oKC = KC;
                    } else {
                    }
                    let KD = if parameters[91] == AY { 1.0 } else { 0.0 };
                    oKD = KD;
                    let KF = if KE > AY { 1.0 } else { 0.0 };
                    oKF = KF;
                    let KH = if JB != 0.0 {
                        JA
                    } else {
                        let KG = -JA;
                        KG
                    };
                    oKH = KH;
                    let KJ = if KI > AY { 1.0 } else { 0.0 };
                    oKJ = KJ;
                    if KJ != 0.0 {
                        let KK = GN / JE;
                        oKK = KK;
                    } else {
                    }
                    let KL = -IW;
                    oKL = KL;
                    JQ = AY;
                } else {
                    JQ = EM;
                }
                let KM = if JN != 0.0 && (if KE > AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let KP;
                let KQ;
                if KM != 0.0 {
                    KP = AY;
                    KQ = AY;
                } else {
                    KP = KN;
                    KQ = KO;
                }
                let KR = if JN != 0.0 && (if KI > AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let KT = if KR != 0.0 {
                    AY
                } else {
                    KS
                };
            [B, E, oG, K, N, P, Q, T, U, W, X, AA, AB, AD, AE, AG, AH, AJ, oAL, AM, AO, oAQ, AR, AT, oAV, AW, AZ, BB, BD, BE, BG, BH, BJ, BK, BM, BN, BP, BQ, BT, BW, BZ, CA, CC, CD, CF, CG, CJ, CM, CP, CS, CT, CV, CW, CY, CZ, DC, DD, DF, DG, DI, DJ, DL, DM, DO, DP, DR, DS, DU, DV, DX, DY, EA, EB, ED, EE, EG, EH, EI, EJ, EL, EO, EP, ER, ES, EU, EV, EX, EY, FA, FB, FD, FE, FG, FH, FJ, FK, FM, FN, FP, FT, FX, FY, GA, GD, GG, GH, GJ, GM, GQ, GR, GT, GU, GW, GX, HA, HB, HC, HD, HE, HF, HG, HH, HI, HJ, HK, HL, HM, HO, HP, HQ, HR, HS, HT, HU, HV, HW, IA, IB, IC, IE, IF, IG, IH, II, IJ, IK, IL, IN, IO, IP, IQ, IR, IS, IT, IU, IV, JB, GO, JF, oJH, oJI, oJJ, JK, JL, JM, JN, oJP, oJS, oJU, oJW, oJY, oKA, oKC, oKD, oKF, oKH, oKJ, oKK, oKL, KM, KR, JQ, KP, KQ, KT]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 267] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[99];
                let B = 0e0f64;
                let E = staged[41];
                let F = staged[42];
                let H = parameters[100];
                let L = parameters[101];
                let Q = staged[19];
                let R = staged[21];
                let S = 9e-1f64;
                let U = staged[15];
                let V = staged[16];
                let X = staged[17];
                let Y = staged[14];
                let AL = 2e0f64;
                let AM = 1e0f64;
                let AV = 1.6021918e-19f64;
                let AX = staged[398];
                let BC = staged[400];
                let BI = staged[6];
                let BK = staged[10];
                let BM = staged[8];
                let BO = staged[62];
                let BY = 4e0f64;
                let BZ = staged[67];
                let CA = 5e-1f64;
                let CD = staged[68];
                let CF = staged[69];
                let CG = staged[70];
                let CL = staged[23];
                let CO = staged[75];
                let CR = parameters[80];
                let CT = staged[72];
                let CX = staged[76];
                let DA = staged[84];
                let DC = staged[85];
                let DD = staged[86];
                let DI = staged[24];
                let DL = staged[91];
                let DP = staged[88];
                let DT = staged[92];
                let DW = staged[98];
                let DY = staged[99];
                let DZ = staged[100];
                let EE = staged[25];
                let EH = staged[105];
                let EL = staged[102];
                let EP = staged[106];
                let IU = 1e-1f64;
                let IX = -1.000000082740371e-11f64;
                let KT = 2e-1f64;
                let KW = -5.000000413701855e-12f64;
                let MU = 1e0f64;
                let OK = staged[840];
                let OL = staged[845];
                let OM = 1e-23f64;
                let mut oAY = 0.0;
                let mut oAZ = 0.0;
                let mut oBA = 0.0;
                let mut oBB = 0.0;
                let mut oBP = 0.0;
                let mut oBQ = 0.0;
                let mut oBR = 0.0;
                let mut oBS = 0.0;
                let mut oBU = 0.0;
                let mut oBV = 0.0;
                let mut oBW = 0.0;
                let mut oCE = 0.0;
                let mut oCH = 0.0;
                let mut oCI = 0.0;
                let mut oCJ = 0.0;
                let mut oCK = 0.0;
                let mut oCM = 0.0;
                let mut oCN = 0.0;
                let mut oCP = 0.0;
                let mut oCQ = 0.0;
                let mut oCS = 0.0;
                let mut oCY = 0.0;
                let mut oCZ = 0.0;
                let mut oDB = 0.0;
                let mut oDE = 0.0;
                let mut oDF = 0.0;
                let mut oDG = 0.0;
                let mut oDH = 0.0;
                let mut oDJ = 0.0;
                let mut oDK = 0.0;
                let mut oDM = 0.0;
                let mut oDN = 0.0;
                let mut oDO = 0.0;
                let mut oDU = 0.0;
                let mut oDV = 0.0;
                let mut oDX = 0.0;
                let mut oEA = 0.0;
                let mut oEB = 0.0;
                let mut oEC = 0.0;
                let mut oED = 0.0;
                let mut oEF = 0.0;
                let mut oEG = 0.0;
                let mut oEI = 0.0;
                let mut oEJ = 0.0;
                let mut oEK = 0.0;
                let mut oEQ = 0.0;
                let mut oER = 0.0;
                let mut oET = 0.0;
                let mut oEU = 0.0;
                let mut oEV = 0.0;
                let mut oEZ = 0.0;
                let mut oFA = 0.0;
                let mut oFB = 0.0;
                let mut oFC = 0.0;
                let mut oFD = 0.0;
                let mut oFE = 0.0;
                let mut oFF = 0.0;
                let mut oFG = 0.0;
                let mut oFH = 0.0;
                let mut oFI = 0.0;
                let mut oFM = 0.0;
                let mut oFN = 0.0;
                let mut oFO = 0.0;
                let mut oFP = 0.0;
                let mut oFQ = 0.0;
                let mut oFR = 0.0;
                let mut oFS = 0.0;
                let mut oFT = 0.0;
                let mut oFU = 0.0;
                let mut oFV = 0.0;
                let mut oFW = 0.0;
                let mut oFX = 0.0;
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
                let mut oGQ = 0.0;
                let mut oGR = 0.0;
                let mut oGT = 0.0;
                let mut oGU = 0.0;
                let mut oGV = 0.0;
                let mut oGZ = 0.0;
                let mut oHA = 0.0;
                let mut oHB = 0.0;
                let mut oHC = 0.0;
                let mut oHD = 0.0;
                let mut oHE = 0.0;
                let mut oHF = 0.0;
                let mut oHG = 0.0;
                let mut oHH = 0.0;
                let mut oHI = 0.0;
                let mut oHM = 0.0;
                let mut oHN = 0.0;
                let mut oHO = 0.0;
                let mut oHP = 0.0;
                let mut oHQ = 0.0;
                let mut oHR = 0.0;
                let mut oHS = 0.0;
                let mut oHT = 0.0;
                let mut oHU = 0.0;
                let mut oHV = 0.0;
                let mut oHW = 0.0;
                let mut oHX = 0.0;
                let mut oIB = 0.0;
                let mut oIC = 0.0;
                let mut oID = 0.0;
                let mut oIE = 0.0;
                let mut oIF = 0.0;
                let mut oIG = 0.0;
                let mut oIH = 0.0;
                let mut oII = 0.0;
                let mut oIJ = 0.0;
                let mut oIK = 0.0;
                let mut oIL = 0.0;
                let mut oIM = 0.0;
                let mut oIQ = 0.0;
                let mut oIR = 0.0;
                let mut oIT = 0.0;
                let mut oIY = 0.0;
                let mut oIZ = 0.0;
                let mut oJA = 0.0;
                let mut oJB = 0.0;
                let mut oJC = 0.0;
                let mut oJD = 0.0;
                let mut oJE = 0.0;
                let mut oJF = 0.0;
                let mut oJG = 0.0;
                let mut oJH = 0.0;
                let mut oJL = 0.0;
                let mut oJM = 0.0;
                let mut oJN = 0.0;
                let mut oJO = 0.0;
                let mut oJP = 0.0;
                let mut oJQ = 0.0;
                let mut oJR = 0.0;
                let mut oJS = 0.0;
                let mut oJT = 0.0;
                let mut oJU = 0.0;
                let mut oJV = 0.0;
                let mut oJW = 0.0;
                let mut oKA = 0.0;
                let mut oKB = 0.0;
                let mut oKC = 0.0;
                let mut oKD = 0.0;
                let mut oKE = 0.0;
                let mut oKF = 0.0;
                let mut oKG = 0.0;
                let mut oKH = 0.0;
                let mut oKI = 0.0;
                let mut oKJ = 0.0;
                let mut oKK = 0.0;
                let mut oKL = 0.0;
                let mut oKP = 0.0;
                let mut oKQ = 0.0;
                let mut oKS = 0.0;
                let mut oKX = 0.0;
                let mut oKY = 0.0;
                let mut oKZ = 0.0;
                let mut oLA = 0.0;
                let mut oLB = 0.0;
                let mut oLC = 0.0;
                let mut oLD = 0.0;
                let mut oLE = 0.0;
                let mut oLF = 0.0;
                let mut oLG = 0.0;
                let mut oLK = 0.0;
                let mut oLL = 0.0;
                let mut oLM = 0.0;
                let mut oLN = 0.0;
                let mut oLO = 0.0;
                let mut oLP = 0.0;
                let mut oLQ = 0.0;
                let mut oLR = 0.0;
                let mut oLS = 0.0;
                let mut oLT = 0.0;
                let mut oLU = 0.0;
                let mut oLV = 0.0;
                let mut oLZ = 0.0;
                let mut oMA = 0.0;
                let mut oMB = 0.0;
                let mut oMC = 0.0;
                let mut oMD = 0.0;
                let mut oME = 0.0;
                let mut oMF = 0.0;
                let mut oMG = 0.0;
                let mut oMH = 0.0;
                let mut oMI = 0.0;
                let mut oMJ = 0.0;
                let mut oMK = 0.0;
                let mut oMO = 0.0;
                let mut oMP = 0.0;
                let mut oMQ = 0.0;
                let mut oMR = 0.0;
                let mut oMS = 0.0;
                let mut oMT = 0.0;
                let mut oMV = 0.0;
                let mut oMW = 0.0;
                let mut oMX = 0.0;
                let mut oMY = 0.0;
                let mut oMZ = 0.0;
                let mut oNA = 0.0;
                let mut oNB = 0.0;
                let mut oNC = 0.0;
                let mut oND = 0.0;
                let mut oNE = 0.0;
                let mut oNF = 0.0;
                let mut oNG = 0.0;
                let mut oNH = 0.0;
                let mut oNI = 0.0;
                let mut oNJ = 0.0;
                let mut oNK = 0.0;
                let mut oNL = 0.0;
                let mut oNM = 0.0;
                let mut oNN = 0.0;
                let mut oNO = 0.0;
                let mut oNP = 0.0;
                let mut oNQ = 0.0;
                let mut oNR = 0.0;
                let mut oNS = 0.0;
                let mut oNT = 0.0;
                let mut oNU = 0.0;
                let mut oNV = 0.0;
                let mut oNW = 0.0;
                let mut oNX = 0.0;
                let mut oNY = 0.0;
                let mut oNZ = 0.0;
                let mut oOA = 0.0;
                let mut oOB = 0.0;
                let mut oOC = 0.0;
                let mut oOD = 0.0;
                let mut oOE = 0.0;
                let mut oOF = 0.0;
                let mut oOG = 0.0;
                let mut oOH = 0.0;
                let mut oOI = 0.0;
                let mut oOJ = 0.0;
                let mut oON = 0.0;
                let mut oOO = 0.0;
                let mut oOP = 0.0;
                let C = if A > B { 1.0 } else { 0.0 };
                let D = if C != 0.0 {
                    A
                } else {
                    B
                };
                let G = (((D * E) * E) * F) * F;
                let I = if H > B { 1.0 } else { 0.0 };
                let J = if I != 0.0 {
                    H
                } else {
                    B
                };
                let K = (J * E) * F;
                let M = if L > B { 1.0 } else { 0.0 };
                let N = if M != 0.0 {
                    L
                } else {
                    B
                };
                let O = (N * E) * F;
                let P = if G == B { 1.0 } else { 0.0 };
                let Z;
                let AA;
                if P != 0.0 {
                    let T = S * (if Q <= R { Q } else { R });
                    let W = U + V;
                    Z = T;
                    AA = W;
                } else {
                    Z = X;
                    AA = Y;
                }
                let AB = if K == B { 1.0 } else { 0.0 };
                let AE;
                let AF;
                if AB != 0.0 {
                    let AC = S * (if X <= R { X } else { R });
                    let AD = Y + V;
                    AE = AC;
                    AF = AD;
                } else {
                    AE = Q;
                    AF = U;
                }
                let AG = if O == B { 1.0 } else { 0.0 };
                let AJ;
                let AK;
                if AG != 0.0 {
                    let AH = S * (if X <= Q { X } else { Q });
                    let AI = Y + U;
                    AJ = AH;
                    AK = AI;
                } else {
                    AJ = R;
                    AK = V;
                }
                let AN = AM - (AL.powf((-1e0f64 / (if (if Z >= AE { Z } else { AE }) >= AJ { (if Z >= AE { Z } else { AE }) } else { AJ }))));
                let AO = (if (if AA <= AF { AA } else { AF }) <= AK { (if AA <= AF { AA } else { AF }) } else { AK }) - 5e-2f64;
                let AP = if (if G > staged[48] { 1.0 } else { 0.0 }) != 0.0 && staged[49] != 0.0 { 1.0 } else { 0.0 };
                let AQ = if G < staged[50] { 1.0 } else { 0.0 };
                let AR = if (if K > staged[51] { 1.0 } else { 0.0 }) != 0.0 && staged[52] != 0.0 { 1.0 } else { 0.0 };
                let AS = if K < staged[53] { 1.0 } else { 0.0 };
                let AT = if (if O > staged[54] { 1.0 } else { 0.0 }) != 0.0 && staged[55] != 0.0 { 1.0 } else { 0.0 };
                let AU = if O < staged[56] { 1.0 } else { 0.0 };
                let AW = AV * G;
                let BD;
                let BE;
                if AX != 0.0 {
                    let AY = if G > B { 1.0 } else { 0.0 };
                    oAY = AY;
                    let AZ = if O > B { 1.0 } else { 0.0 };
                    oAZ = AZ;
                    let BA = if K > B { 1.0 } else { 0.0 };
                    oBA = BA;
                    let BB = if (if (if (if staged[59] != 0.0 && AY != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AZ != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if staged[60] != 0.0 && AY != 0.0 { 1.0 } else { 0.0 }) != 0.0 && BA != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if staged[61] != 0.0 && AZ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && BA != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBB = BB;
                    let BG;
                    let BH;
                    if BB != 0.0 {
                        BG = B;
                        BH = AM;
                    } else {
                        let BJ = if AY != 0.0 {
                            BI
                        } else {
                            AM
                        };
                        let BL = if AZ != 0.0 {
                            BK
                        } else {
                            BJ
                        };
                        let BN = if BA != 0.0 {
                            BM
                        } else {
                            BL
                        };
                        BG = BC;
                        BH = BN;
                    }
                    BD = BG;
                    BE = BH;
                } else {
                    BD = BC;
                    BE = AM;
                }
                let BF = if BD == AM { 1.0 } else { 0.0 };
                if BF != 0.0 {
                    let BP = -4e-1f64 * BO;
                    oBP = BP;
                    let BQ = -6.5e-1f64 * BO;
                    oBQ = BQ;
                    let BR = -8e-1f64 * BO;
                    oBR = BR;
                    let BS = if (if (if P != 0.0 && AB != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AG != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oBS = BS;
                    let BT;
                    let BU;
                    if BS != 0.0 {
                        let BV = if BP > B { 1.0 } else { 0.0 };
                        oBV = BV;
                        if BV != 0.0 {
                        } else {
                            let BW = -BP;
                            oBW = BW;
                        }
                        let BX = BP - AO;
                        let CB = CA * ((BP + AO) - (((BX * BX) + ((BY * BZ) * BZ)).sqrt()));
                        let CC = CA * (BP - (((BP * BP) + 4e-12f64).sqrt()));
                        BT = CB;
                        BU = CC;
                    } else {
                        BT = B;
                        BU = B;
                    }
                    oBU = BU;
                    if P != 0.0 {
                    } else {
                        let CE = if CD == CA { 1.0 } else { 0.0 };
                        oCE = CE;
                        let CH = if CG == B { 1.0 } else { 0.0 };
                        oCH = CH;
                        let CI = if (if CF == B { 1.0 } else { 0.0 }) != 0.0 && CH != 0.0 { 1.0 } else { 0.0 };
                        oCI = CI;
                        if CI != 0.0 {
                        } else {
                            let CJ = if X == CA { 1.0 } else { 0.0 };
                            oCJ = CJ;
                            if CJ != 0.0 {
                            } else {
                                let CK = AM - (AL * X);
                                oCK = CK;
                            }
                        }
                        if CH != 0.0 {
                        } else {
                            let CM = (-X) * CL;
                            oCM = CM;
                            let CN = if CM == -1e0f64 { 1.0 } else { 0.0 };
                            oCN = CN;
                        }
                        let CP = if CO == B { 1.0 } else { 0.0 };
                        oCP = CP;
                        if CP != 0.0 {
                        } else {
                            let CQ = if X == CA { 1.0 } else { 0.0 };
                            oCQ = CQ;
                            let CW = if CQ != 0.0 {
                                let CU = ((Y - BT) * CT).sqrt();
                                CU
                            } else {
                                let CV = ((Y - BT) * CT).powf(X);
                                CV
                            };
                            let CY = CL * (((Y - BT) * CX) / CW);
                            oCY = CY;
                            let CZ = (BP * CY) * CY;
                            oCZ = CZ;
                        }
                        let CS = if CR == B { 1.0 } else { 0.0 };
                        oCS = CS;
                    }
                    if AB != 0.0 {
                    } else {
                        let DB = if DA == CA { 1.0 } else { 0.0 };
                        oDB = DB;
                        let DE = if DD == B { 1.0 } else { 0.0 };
                        oDE = DE;
                        let DF = if (if DC == B { 1.0 } else { 0.0 }) != 0.0 && DE != 0.0 { 1.0 } else { 0.0 };
                        oDF = DF;
                        if DF != 0.0 {
                        } else {
                            let DG = if Q == CA { 1.0 } else { 0.0 };
                            oDG = DG;
                            if DG != 0.0 {
                            } else {
                                let DH = AM - (AL * Q);
                                oDH = DH;
                            }
                        }
                        if DE != 0.0 {
                        } else {
                            let DJ = (-Q) * DI;
                            oDJ = DJ;
                            let DK = if DJ == -1e0f64 { 1.0 } else { 0.0 };
                            oDK = DK;
                        }
                        let DM = if DL == B { 1.0 } else { 0.0 };
                        oDM = DM;
                        if DM != 0.0 {
                        } else {
                            let DN = if Q == CA { 1.0 } else { 0.0 };
                            oDN = DN;
                            let DS = if DN != 0.0 {
                                let DQ = ((U - BT) * DP).sqrt();
                                DQ
                            } else {
                                let DR = ((U - BT) * DP).powf(Q);
                                DR
                            };
                            let DU = DI * (((U - BT) * DT) / DS);
                            oDU = DU;
                            let DV = (BP * DU) * DU;
                            oDV = DV;
                        }
                        let DO = if CR == B { 1.0 } else { 0.0 };
                        oDO = DO;
                    }
                    if AG != 0.0 {
                    } else {
                        let DX = if DW == CA { 1.0 } else { 0.0 };
                        oDX = DX;
                        let EA = if DZ == B { 1.0 } else { 0.0 };
                        oEA = EA;
                        let EB = if (if DY == B { 1.0 } else { 0.0 }) != 0.0 && EA != 0.0 { 1.0 } else { 0.0 };
                        oEB = EB;
                        if EB != 0.0 {
                        } else {
                            let EC = if R == CA { 1.0 } else { 0.0 };
                            oEC = EC;
                            if EC != 0.0 {
                            } else {
                                let ED = AM - (AL * R);
                                oED = ED;
                            }
                        }
                        if EA != 0.0 {
                        } else {
                            let EF = (-R) * EE;
                            oEF = EF;
                            let EG = if EF == -1e0f64 { 1.0 } else { 0.0 };
                            oEG = EG;
                        }
                        let EI = if EH == B { 1.0 } else { 0.0 };
                        oEI = EI;
                        if EI != 0.0 {
                        } else {
                            let EJ = if R == CA { 1.0 } else { 0.0 };
                            oEJ = EJ;
                            let EO = if EJ != 0.0 {
                                let EM = ((V - BT) * EL).sqrt();
                                EM
                            } else {
                                let EN = ((V - BT) * EL).powf(R);
                                EN
                            };
                            let EQ = EE * (((V - BT) * EP) / EO);
                            oEQ = EQ;
                            let ER = (BP * EQ) * EQ;
                            oER = ER;
                        }
                        let EK = if CR == B { 1.0 } else { 0.0 };
                        oEK = EK;
                    }
                    let ES;
                    let ET;
                    if BS != 0.0 {
                        let EU = if BQ > B { 1.0 } else { 0.0 };
                        oEU = EU;
                        if EU != 0.0 {
                        } else {
                            let EV = -BQ;
                            oEV = EV;
                        }
                        let EW = BQ - AO;
                        let EX = CA * ((BQ + AO) - (((EW * EW) + ((BY * BZ) * BZ)).sqrt()));
                        let EY = CA * (BQ - (((BQ * BQ) + 4e-12f64).sqrt()));
                        ES = EX;
                        ET = EY;
                    } else {
                        ES = B;
                        ET = B;
                    }
                    oET = ET;
                    if P != 0.0 {
                    } else {
                        let EZ = if CD == CA { 1.0 } else { 0.0 };
                        oEZ = EZ;
                        let FA = if CG == B { 1.0 } else { 0.0 };
                        oFA = FA;
                        let FB = if (if CF == B { 1.0 } else { 0.0 }) != 0.0 && FA != 0.0 { 1.0 } else { 0.0 };
                        oFB = FB;
                        if FB != 0.0 {
                        } else {
                            let FC = if X == CA { 1.0 } else { 0.0 };
                            oFC = FC;
                            if FC != 0.0 {
                            } else {
                                let FD = AM - (AL * X);
                                oFD = FD;
                            }
                        }
                        if FA != 0.0 {
                        } else {
                            let FE = (-X) * CL;
                            oFE = FE;
                            let FF = if FE == -1e0f64 { 1.0 } else { 0.0 };
                            oFF = FF;
                        }
                        let FG = if CO == B { 1.0 } else { 0.0 };
                        oFG = FG;
                        if FG != 0.0 {
                        } else {
                            let FH = if X == CA { 1.0 } else { 0.0 };
                            oFH = FH;
                            let FL = if FH != 0.0 {
                                let FJ = ((Y - ES) * CT).sqrt();
                                FJ
                            } else {
                                let FK = ((Y - ES) * CT).powf(X);
                                FK
                            };
                            let FM = CL * (((Y - ES) * CX) / FL);
                            oFM = FM;
                            let FN = (BQ * FM) * FM;
                            oFN = FN;
                        }
                        let FI = if CR == B { 1.0 } else { 0.0 };
                        oFI = FI;
                    }
                    if AB != 0.0 {
                    } else {
                        let FO = if DA == CA { 1.0 } else { 0.0 };
                        oFO = FO;
                        let FP = if DD == B { 1.0 } else { 0.0 };
                        oFP = FP;
                        let FQ = if (if DC == B { 1.0 } else { 0.0 }) != 0.0 && FP != 0.0 { 1.0 } else { 0.0 };
                        oFQ = FQ;
                        if FQ != 0.0 {
                        } else {
                            let FR = if Q == CA { 1.0 } else { 0.0 };
                            oFR = FR;
                            if FR != 0.0 {
                            } else {
                                let FS = AM - (AL * Q);
                                oFS = FS;
                            }
                        }
                        if FP != 0.0 {
                        } else {
                            let FT = (-Q) * DI;
                            oFT = FT;
                            let FU = if FT == -1e0f64 { 1.0 } else { 0.0 };
                            oFU = FU;
                        }
                        let FV = if DL == B { 1.0 } else { 0.0 };
                        oFV = FV;
                        if FV != 0.0 {
                        } else {
                            let FW = if Q == CA { 1.0 } else { 0.0 };
                            oFW = FW;
                            let GA = if FW != 0.0 {
                                let FY = ((U - ES) * DP).sqrt();
                                FY
                            } else {
                                let FZ = ((U - ES) * DP).powf(Q);
                                FZ
                            };
                            let GB = DI * (((U - ES) * DT) / GA);
                            oGB = GB;
                            let GC = (BQ * GB) * GB;
                            oGC = GC;
                        }
                        let FX = if CR == B { 1.0 } else { 0.0 };
                        oFX = FX;
                    }
                    if AG != 0.0 {
                    } else {
                        let GD = if DW == CA { 1.0 } else { 0.0 };
                        oGD = GD;
                        let GE = if DZ == B { 1.0 } else { 0.0 };
                        oGE = GE;
                        let GF = if (if DY == B { 1.0 } else { 0.0 }) != 0.0 && GE != 0.0 { 1.0 } else { 0.0 };
                        oGF = GF;
                        if GF != 0.0 {
                        } else {
                            let GG = if R == CA { 1.0 } else { 0.0 };
                            oGG = GG;
                            if GG != 0.0 {
                            } else {
                                let GH = AM - (AL * R);
                                oGH = GH;
                            }
                        }
                        if GE != 0.0 {
                        } else {
                            let GI = (-R) * EE;
                            oGI = GI;
                            let GJ = if GI == -1e0f64 { 1.0 } else { 0.0 };
                            oGJ = GJ;
                        }
                        let GK = if EH == B { 1.0 } else { 0.0 };
                        oGK = GK;
                        if GK != 0.0 {
                        } else {
                            let GL = if R == CA { 1.0 } else { 0.0 };
                            oGL = GL;
                            let GP = if GL != 0.0 {
                                let GN = ((V - ES) * EL).sqrt();
                                GN
                            } else {
                                let GO = ((V - ES) * EL).powf(R);
                                GO
                            };
                            let GQ = EE * (((V - ES) * EP) / GP);
                            oGQ = GQ;
                            let GR = (BQ * GQ) * GQ;
                            oGR = GR;
                        }
                        let GM = if CR == B { 1.0 } else { 0.0 };
                        oGM = GM;
                    }
                    let GS;
                    let GT;
                    if BS != 0.0 {
                        let GU = if BR > B { 1.0 } else { 0.0 };
                        oGU = GU;
                        if GU != 0.0 {
                        } else {
                            let GV = -BR;
                            oGV = GV;
                        }
                        let GW = BR - AO;
                        let GX = CA * ((BR + AO) - (((GW * GW) + ((BY * BZ) * BZ)).sqrt()));
                        let GY = CA * (BR - (((BR * BR) + 4e-12f64).sqrt()));
                        GS = GX;
                        GT = GY;
                    } else {
                        GS = B;
                        GT = B;
                    }
                    oGT = GT;
                    if P != 0.0 {
                    } else {
                        let GZ = if CD == CA { 1.0 } else { 0.0 };
                        oGZ = GZ;
                        let HA = if CG == B { 1.0 } else { 0.0 };
                        oHA = HA;
                        let HB = if (if CF == B { 1.0 } else { 0.0 }) != 0.0 && HA != 0.0 { 1.0 } else { 0.0 };
                        oHB = HB;
                        if HB != 0.0 {
                        } else {
                            let HC = if X == CA { 1.0 } else { 0.0 };
                            oHC = HC;
                            if HC != 0.0 {
                            } else {
                                let HD = AM - (AL * X);
                                oHD = HD;
                            }
                        }
                        if HA != 0.0 {
                        } else {
                            let HE = (-X) * CL;
                            oHE = HE;
                            let HF = if HE == -1e0f64 { 1.0 } else { 0.0 };
                            oHF = HF;
                        }
                        let HG = if CO == B { 1.0 } else { 0.0 };
                        oHG = HG;
                        if HG != 0.0 {
                        } else {
                            let HH = if X == CA { 1.0 } else { 0.0 };
                            oHH = HH;
                            let HL = if HH != 0.0 {
                                let HJ = ((Y - GS) * CT).sqrt();
                                HJ
                            } else {
                                let HK = ((Y - GS) * CT).powf(X);
                                HK
                            };
                            let HM = CL * (((Y - GS) * CX) / HL);
                            oHM = HM;
                            let HN = (BR * HM) * HM;
                            oHN = HN;
                        }
                        let HI = if CR == B { 1.0 } else { 0.0 };
                        oHI = HI;
                    }
                    if AB != 0.0 {
                    } else {
                        let HO = if DA == CA { 1.0 } else { 0.0 };
                        oHO = HO;
                        let HP = if DD == B { 1.0 } else { 0.0 };
                        oHP = HP;
                        let HQ = if (if DC == B { 1.0 } else { 0.0 }) != 0.0 && HP != 0.0 { 1.0 } else { 0.0 };
                        oHQ = HQ;
                        if HQ != 0.0 {
                        } else {
                            let HR = if Q == CA { 1.0 } else { 0.0 };
                            oHR = HR;
                            if HR != 0.0 {
                            } else {
                                let HS = AM - (AL * Q);
                                oHS = HS;
                            }
                        }
                        if HP != 0.0 {
                        } else {
                            let HT = (-Q) * DI;
                            oHT = HT;
                            let HU = if HT == -1e0f64 { 1.0 } else { 0.0 };
                            oHU = HU;
                        }
                        let HV = if DL == B { 1.0 } else { 0.0 };
                        oHV = HV;
                        if HV != 0.0 {
                        } else {
                            let HW = if Q == CA { 1.0 } else { 0.0 };
                            oHW = HW;
                            let IA = if HW != 0.0 {
                                let HY = ((U - GS) * DP).sqrt();
                                HY
                            } else {
                                let HZ = ((U - GS) * DP).powf(Q);
                                HZ
                            };
                            let IB = DI * (((U - GS) * DT) / IA);
                            oIB = IB;
                            let IC = (BR * IB) * IB;
                            oIC = IC;
                        }
                        let HX = if CR == B { 1.0 } else { 0.0 };
                        oHX = HX;
                    }
                    if AG != 0.0 {
                    } else {
                        let ID = if DW == CA { 1.0 } else { 0.0 };
                        oID = ID;
                        let IE = if DZ == B { 1.0 } else { 0.0 };
                        oIE = IE;
                        let IF = if (if DY == B { 1.0 } else { 0.0 }) != 0.0 && IE != 0.0 { 1.0 } else { 0.0 };
                        oIF = IF;
                        if IF != 0.0 {
                        } else {
                            let IG = if R == CA { 1.0 } else { 0.0 };
                            oIG = IG;
                            if IG != 0.0 {
                            } else {
                                let IH = AM - (AL * R);
                                oIH = IH;
                            }
                        }
                        if IE != 0.0 {
                        } else {
                            let II = (-R) * EE;
                            oII = II;
                            let IJ = if II == -1e0f64 { 1.0 } else { 0.0 };
                            oIJ = IJ;
                        }
                        let IK = if EH == B { 1.0 } else { 0.0 };
                        oIK = IK;
                        if IK != 0.0 {
                        } else {
                            let IL = if R == CA { 1.0 } else { 0.0 };
                            oIL = IL;
                            let IP = if IL != 0.0 {
                                let IN = ((V - GS) * EL).sqrt();
                                IN
                            } else {
                                let IO = ((V - GS) * EL).powf(R);
                                IO
                            };
                            let IQ = EE * (((V - GS) * EP) / IP);
                            oIQ = IQ;
                            let IR = (BR * IQ) * IQ;
                            oIR = IR;
                        }
                        let IM = if CR == B { 1.0 } else { 0.0 };
                        oIM = IM;
                    }
                    let IS;
                    let IT;
                    if BS != 0.0 {
                        let IV = IU - AO;
                        let IW = CA * ((IU + AO) - (((IV * IV) + ((BY * BZ) * BZ)).sqrt()));
                        IS = IW;
                        IT = IX;
                    } else {
                        IS = B;
                        IT = B;
                    }
                    oIT = IT;
                    if P != 0.0 {
                    } else {
                        let IY = if CD == CA { 1.0 } else { 0.0 };
                        oIY = IY;
                        let IZ = if CG == B { 1.0 } else { 0.0 };
                        oIZ = IZ;
                        let JA = if (if CF == B { 1.0 } else { 0.0 }) != 0.0 && IZ != 0.0 { 1.0 } else { 0.0 };
                        oJA = JA;
                        if JA != 0.0 {
                        } else {
                            let JB = if X == CA { 1.0 } else { 0.0 };
                            oJB = JB;
                            if JB != 0.0 {
                            } else {
                                let JC = AM - (AL * X);
                                oJC = JC;
                            }
                        }
                        if IZ != 0.0 {
                        } else {
                            let JD = (-X) * CL;
                            oJD = JD;
                            let JE = if JD == -1e0f64 { 1.0 } else { 0.0 };
                            oJE = JE;
                        }
                        let JF = if CO == B { 1.0 } else { 0.0 };
                        oJF = JF;
                        if JF != 0.0 {
                        } else {
                            let JG = if X == CA { 1.0 } else { 0.0 };
                            oJG = JG;
                            let JK = if JG != 0.0 {
                                let JI = ((Y - IS) * CT).sqrt();
                                JI
                            } else {
                                let JJ = ((Y - IS) * CT).powf(X);
                                JJ
                            };
                            let JL = CL * (((Y - IS) * CX) / JK);
                            oJL = JL;
                            let JM = (IU * JL) * JL;
                            oJM = JM;
                        }
                        let JH = if CR == B { 1.0 } else { 0.0 };
                        oJH = JH;
                    }
                    if AB != 0.0 {
                    } else {
                        let JN = if DA == CA { 1.0 } else { 0.0 };
                        oJN = JN;
                        let JO = if DD == B { 1.0 } else { 0.0 };
                        oJO = JO;
                        let JP = if (if DC == B { 1.0 } else { 0.0 }) != 0.0 && JO != 0.0 { 1.0 } else { 0.0 };
                        oJP = JP;
                        if JP != 0.0 {
                        } else {
                            let JQ = if Q == CA { 1.0 } else { 0.0 };
                            oJQ = JQ;
                            if JQ != 0.0 {
                            } else {
                                let JR = AM - (AL * Q);
                                oJR = JR;
                            }
                        }
                        if JO != 0.0 {
                        } else {
                            let JS = (-Q) * DI;
                            oJS = JS;
                            let JT = if JS == -1e0f64 { 1.0 } else { 0.0 };
                            oJT = JT;
                        }
                        let JU = if DL == B { 1.0 } else { 0.0 };
                        oJU = JU;
                        if JU != 0.0 {
                        } else {
                            let JV = if Q == CA { 1.0 } else { 0.0 };
                            oJV = JV;
                            let JZ = if JV != 0.0 {
                                let JX = ((U - IS) * DP).sqrt();
                                JX
                            } else {
                                let JY = ((U - IS) * DP).powf(Q);
                                JY
                            };
                            let KA = DI * (((U - IS) * DT) / JZ);
                            oKA = KA;
                            let KB = (IU * KA) * KA;
                            oKB = KB;
                        }
                        let JW = if CR == B { 1.0 } else { 0.0 };
                        oJW = JW;
                    }
                    if AG != 0.0 {
                    } else {
                        let KC = if DW == CA { 1.0 } else { 0.0 };
                        oKC = KC;
                        let KD = if DZ == B { 1.0 } else { 0.0 };
                        oKD = KD;
                        let KE = if (if DY == B { 1.0 } else { 0.0 }) != 0.0 && KD != 0.0 { 1.0 } else { 0.0 };
                        oKE = KE;
                        if KE != 0.0 {
                        } else {
                            let KF = if R == CA { 1.0 } else { 0.0 };
                            oKF = KF;
                            if KF != 0.0 {
                            } else {
                                let KG = AM - (AL * R);
                                oKG = KG;
                            }
                        }
                        if KD != 0.0 {
                        } else {
                            let KH = (-R) * EE;
                            oKH = KH;
                            let KI = if KH == -1e0f64 { 1.0 } else { 0.0 };
                            oKI = KI;
                        }
                        let KJ = if EH == B { 1.0 } else { 0.0 };
                        oKJ = KJ;
                        if KJ != 0.0 {
                        } else {
                            let KK = if R == CA { 1.0 } else { 0.0 };
                            oKK = KK;
                            let KO = if KK != 0.0 {
                                let KM = ((V - IS) * EL).sqrt();
                                KM
                            } else {
                                let KN = ((V - IS) * EL).powf(R);
                                KN
                            };
                            let KP = EE * (((V - IS) * EP) / KO);
                            oKP = KP;
                            let KQ = (IU * KP) * KP;
                            oKQ = KQ;
                        }
                        let KL = if CR == B { 1.0 } else { 0.0 };
                        oKL = KL;
                    }
                    let KR;
                    let KS;
                    if BS != 0.0 {
                        let KU = KT - AO;
                        let KV = CA * ((KT + AO) - (((KU * KU) + ((BY * BZ) * BZ)).sqrt()));
                        KR = KV;
                        KS = KW;
                    } else {
                        KR = B;
                        KS = B;
                    }
                    oKS = KS;
                    if P != 0.0 {
                    } else {
                        let KX = if CD == CA { 1.0 } else { 0.0 };
                        oKX = KX;
                        let KY = if CG == B { 1.0 } else { 0.0 };
                        oKY = KY;
                        let KZ = if (if CF == B { 1.0 } else { 0.0 }) != 0.0 && KY != 0.0 { 1.0 } else { 0.0 };
                        oKZ = KZ;
                        if KZ != 0.0 {
                        } else {
                            let LA = if X == CA { 1.0 } else { 0.0 };
                            oLA = LA;
                            if LA != 0.0 {
                            } else {
                                let LB = AM - (AL * X);
                                oLB = LB;
                            }
                        }
                        if KY != 0.0 {
                        } else {
                            let LC = (-X) * CL;
                            oLC = LC;
                            let LD = if LC == -1e0f64 { 1.0 } else { 0.0 };
                            oLD = LD;
                        }
                        let LE = if CO == B { 1.0 } else { 0.0 };
                        oLE = LE;
                        if LE != 0.0 {
                        } else {
                            let LF = if X == CA { 1.0 } else { 0.0 };
                            oLF = LF;
                            let LJ = if LF != 0.0 {
                                let LH = ((Y - KR) * CT).sqrt();
                                LH
                            } else {
                                let LI = ((Y - KR) * CT).powf(X);
                                LI
                            };
                            let LK = CL * (((Y - KR) * CX) / LJ);
                            oLK = LK;
                            let LL = (KT * LK) * LK;
                            oLL = LL;
                        }
                        let LG = if CR == B { 1.0 } else { 0.0 };
                        oLG = LG;
                    }
                    if AB != 0.0 {
                    } else {
                        let LM = if DA == CA { 1.0 } else { 0.0 };
                        oLM = LM;
                        let LN = if DD == B { 1.0 } else { 0.0 };
                        oLN = LN;
                        let LO = if (if DC == B { 1.0 } else { 0.0 }) != 0.0 && LN != 0.0 { 1.0 } else { 0.0 };
                        oLO = LO;
                        if LO != 0.0 {
                        } else {
                            let LP = if Q == CA { 1.0 } else { 0.0 };
                            oLP = LP;
                            if LP != 0.0 {
                            } else {
                                let LQ = AM - (AL * Q);
                                oLQ = LQ;
                            }
                        }
                        if LN != 0.0 {
                        } else {
                            let LR = (-Q) * DI;
                            oLR = LR;
                            let LS = if LR == -1e0f64 { 1.0 } else { 0.0 };
                            oLS = LS;
                        }
                        let LT = if DL == B { 1.0 } else { 0.0 };
                        oLT = LT;
                        if LT != 0.0 {
                        } else {
                            let LU = if Q == CA { 1.0 } else { 0.0 };
                            oLU = LU;
                            let LY = if LU != 0.0 {
                                let LW = ((U - KR) * DP).sqrt();
                                LW
                            } else {
                                let LX = ((U - KR) * DP).powf(Q);
                                LX
                            };
                            let LZ = DI * (((U - KR) * DT) / LY);
                            oLZ = LZ;
                            let MA = (KT * LZ) * LZ;
                            oMA = MA;
                        }
                        let LV = if CR == B { 1.0 } else { 0.0 };
                        oLV = LV;
                    }
                    if AG != 0.0 {
                    } else {
                        let MB = if DW == CA { 1.0 } else { 0.0 };
                        oMB = MB;
                        let MC = if DZ == B { 1.0 } else { 0.0 };
                        oMC = MC;
                        let MD = if (if DY == B { 1.0 } else { 0.0 }) != 0.0 && MC != 0.0 { 1.0 } else { 0.0 };
                        oMD = MD;
                        if MD != 0.0 {
                        } else {
                            let ME = if R == CA { 1.0 } else { 0.0 };
                            oME = ME;
                            if ME != 0.0 {
                            } else {
                                let MF = AM - (AL * R);
                                oMF = MF;
                            }
                        }
                        if MC != 0.0 {
                        } else {
                            let MG = (-R) * EE;
                            oMG = MG;
                            let MH = if MG == -1e0f64 { 1.0 } else { 0.0 };
                            oMH = MH;
                        }
                        let MI = if EH == B { 1.0 } else { 0.0 };
                        oMI = MI;
                        if MI != 0.0 {
                        } else {
                            let MJ = if R == CA { 1.0 } else { 0.0 };
                            oMJ = MJ;
                            let MN = if MJ != 0.0 {
                                let ML = ((V - KR) * EL).sqrt();
                                ML
                            } else {
                                let MM = ((V - KR) * EL).powf(R);
                                MM
                            };
                            let MO = EE * (((V - KR) * EP) / MN);
                            oMO = MO;
                            let MP = (KT * MO) * MO;
                            oMP = MP;
                        }
                        let MK = if CR == B { 1.0 } else { 0.0 };
                        oMK = MK;
                    }
                    if BS != 0.0 {
                        let MQ = CA * staged[44];
                        oMQ = MQ;
                    } else {
                    }
                } else {
                }
                if BF != 0.0 {
                } else {
                    let MR = if (if (if P != 0.0 && AB != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AG != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oMR = MR;
                    if MR != 0.0 {
                        let MS = (BY * BZ) * BZ;
                        oMS = MS;
                    } else {
                    }
                    if P != 0.0 {
                    } else {
                        let MT = if CD == CA { 1.0 } else { 0.0 };
                        oMT = MT;
                        if MT != 0.0 {
                        } else {
                            let MV = CD - MU;
                            oMV = MV;
                        }
                        let MW = if CG == B { 1.0 } else { 0.0 };
                        oMW = MW;
                        let MX = if (if CF == B { 1.0 } else { 0.0 }) != 0.0 && MW != 0.0 { 1.0 } else { 0.0 };
                        oMX = MX;
                        if MX != 0.0 {
                        } else {
                            let MY = if X == CA { 1.0 } else { 0.0 };
                            oMY = MY;
                            if MY != 0.0 {
                            } else {
                                let MZ = AM - (AL * X);
                                oMZ = MZ;
                            }
                            if MY != 0.0 {
                            } else {
                                let NA = X - MU;
                                oNA = NA;
                            }
                        }
                        if MW != 0.0 {
                        } else {
                            let NB = (-X) * CL;
                            oNB = NB;
                            let NC = if NB == -1e0f64 { 1.0 } else { 0.0 };
                            oNC = NC;
                            if NC != 0.0 {
                            } else {
                                let NE = NB - MU;
                                oNE = NE;
                            }
                        }
                        let ND = if CO == B { 1.0 } else { 0.0 };
                        oND = ND;
                        if ND != 0.0 {
                        } else {
                            let NF = if X == CA { 1.0 } else { 0.0 };
                            oNF = NF;
                            if NF != 0.0 {
                            } else {
                                let NH = X - MU;
                                oNH = NH;
                            }
                        }
                        let NG = if CR == B { 1.0 } else { 0.0 };
                        oNG = NG;
                    }
                    if AB != 0.0 {
                    } else {
                        let NI = if DA == CA { 1.0 } else { 0.0 };
                        oNI = NI;
                        if NI != 0.0 {
                        } else {
                            let NJ = DA - MU;
                            oNJ = NJ;
                        }
                        let NK = if DD == B { 1.0 } else { 0.0 };
                        oNK = NK;
                        let NL = if (if DC == B { 1.0 } else { 0.0 }) != 0.0 && NK != 0.0 { 1.0 } else { 0.0 };
                        oNL = NL;
                        if NL != 0.0 {
                        } else {
                            let NM = if Q == CA { 1.0 } else { 0.0 };
                            oNM = NM;
                            if NM != 0.0 {
                            } else {
                                let NN = AM - (AL * Q);
                                oNN = NN;
                            }
                            if NM != 0.0 {
                            } else {
                                let NO = Q - MU;
                                oNO = NO;
                            }
                        }
                        if NK != 0.0 {
                        } else {
                            let NP = (-Q) * DI;
                            oNP = NP;
                            let NQ = if NP == -1e0f64 { 1.0 } else { 0.0 };
                            oNQ = NQ;
                            if NQ != 0.0 {
                            } else {
                                let NS = NP - MU;
                                oNS = NS;
                            }
                        }
                        let NR = if DL == B { 1.0 } else { 0.0 };
                        oNR = NR;
                        if NR != 0.0 {
                        } else {
                            let NT = if Q == CA { 1.0 } else { 0.0 };
                            oNT = NT;
                            if NT != 0.0 {
                            } else {
                                let NV = Q - MU;
                                oNV = NV;
                            }
                        }
                        let NU = if CR == B { 1.0 } else { 0.0 };
                        oNU = NU;
                    }
                    if AG != 0.0 {
                    } else {
                        let NW = if DW == CA { 1.0 } else { 0.0 };
                        oNW = NW;
                        if NW != 0.0 {
                        } else {
                            let NX = DW - MU;
                            oNX = NX;
                        }
                        let NY = if DZ == B { 1.0 } else { 0.0 };
                        oNY = NY;
                        let NZ = if (if DY == B { 1.0 } else { 0.0 }) != 0.0 && NY != 0.0 { 1.0 } else { 0.0 };
                        oNZ = NZ;
                        if NZ != 0.0 {
                        } else {
                            let OA = if R == CA { 1.0 } else { 0.0 };
                            oOA = OA;
                            if OA != 0.0 {
                            } else {
                                let OB = AM - (AL * R);
                                oOB = OB;
                            }
                            if OA != 0.0 {
                            } else {
                                let OC = R - MU;
                                oOC = OC;
                            }
                        }
                        if NY != 0.0 {
                        } else {
                            let OD = (-R) * EE;
                            oOD = OD;
                            let OE = if OD == -1e0f64 { 1.0 } else { 0.0 };
                            oOE = OE;
                            if OE != 0.0 {
                            } else {
                                let OG = OD - MU;
                                oOG = OG;
                            }
                        }
                        let OF = if EH == B { 1.0 } else { 0.0 };
                        oOF = OF;
                        if OF != 0.0 {
                        } else {
                            let OH = if R == CA { 1.0 } else { 0.0 };
                            oOH = OH;
                            if OH != 0.0 {
                            } else {
                                let OJ = R - MU;
                                oOJ = OJ;
                            }
                        }
                        let OI = if CR == B { 1.0 } else { 0.0 };
                        oOI = OI;
                    }
                }
                if OK != 0.0 {
                    if OL != 0.0 {
                        let ON = OM / AW;
                        oON = ON;
                    } else {
                    }
                    if OL != 0.0 {
                        let OO = OM / AW;
                        oOO = OO;
                    } else {
                    }
                    let OP = (-((staged[40] * G) * AV)) * parameters[94];
                    oOP = OP;
                } else {
                }
            [C, G, I, K, M, O, P, AB, AG, AN, AO, AP, AQ, AR, AS, AT, AU, AW, oAY, oAZ, oBA, oBB, BF, oBP, oBQ, oBR, oBS, oBV, oBW, oCE, oCH, oCI, oCJ, oCK, oCM, oCN, oCP, oCQ, oCY, oCZ, oCS, oBU, oDB, oDE, oDF, oDG, oDH, oDJ, oDK, oDM, oDN, oDU, oDV, oDO, oDX, oEA, oEB, oEC, oED, oEF, oEG, oEI, oEJ, oEQ, oER, oEK, oEU, oEV, oEZ, oFA, oFB, oFC, oFD, oFE, oFF, oFG, oFH, oFM, oFN, oFI, oET, oFO, oFP, oFQ, oFR, oFS, oFT, oFU, oFV, oFW, oGB, oGC, oFX, oGD, oGE, oGF, oGG, oGH, oGI, oGJ, oGK, oGL, oGQ, oGR, oGM, oGU, oGV, oGZ, oHA, oHB, oHC, oHD, oHE, oHF, oHG, oHH, oHM, oHN, oHI, oGT, oHO, oHP, oHQ, oHR, oHS, oHT, oHU, oHV, oHW, oIB, oIC, oHX, oID, oIE, oIF, oIG, oIH, oII, oIJ, oIK, oIL, oIQ, oIR, oIM, oIY, oIZ, oJA, oJB, oJC, oJD, oJE, oJF, oJG, oJL, oJM, oJH, oIT, oJN, oJO, oJP, oJQ, oJR, oJS, oJT, oJU, oJV, oKA, oKB, oJW, oKC, oKD, oKE, oKF, oKG, oKH, oKI, oKJ, oKK, oKP, oKQ, oKL, oKX, oKY, oKZ, oLA, oLB, oLC, oLD, oLE, oLF, oLK, oLL, oLG, oKS, oLM, oLN, oLO, oLP, oLQ, oLR, oLS, oLT, oLU, oLZ, oMA, oLV, oMB, oMC, oMD, oME, oMF, oMG, oMH, oMI, oMJ, oMO, oMP, oMK, BE, oMQ, oMR, oMS, oMT, oMW, oMX, oMY, oMZ, oNB, oNC, oND, oNF, oNG, oNI, oNK, oNL, oNM, oNN, oNP, oNQ, oNR, oNT, oNU, oNW, oNY, oNZ, oOA, oOB, oOD, oOE, oOF, oOH, oOI, oON, oOO, oOP, oMV, oNA, oNE, oNH, oNJ, oNO, oNS, oNV, oNX, oOC, oOG, oOJ]
        };
        self.canonical_staged[374] = produced[0];
        self.canonical_staged[43] = produced[1];
        self.canonical_staged[375] = produced[2];
        self.canonical_staged[45] = produced[3];
        self.canonical_staged[376] = produced[4];
        self.canonical_staged[46] = produced[5];
        self.canonical_staged[382] = produced[6];
        self.canonical_staged[383] = produced[7];
        self.canonical_staged[384] = produced[8];
        self.canonical_staged[47] = produced[9];
        self.canonical_staged[217] = produced[10];
        self.canonical_staged[385] = produced[11];
        self.canonical_staged[386] = produced[12];
        self.canonical_staged[387] = produced[13];
        self.canonical_staged[388] = produced[14];
        self.canonical_staged[389] = produced[15];
        self.canonical_staged[390] = produced[16];
        self.canonical_staged[275] = produced[17];
        self.canonical_staged[402] = produced[18];
        self.canonical_staged[403] = produced[19];
        self.canonical_staged[404] = produced[20];
        self.canonical_staged[399] = produced[21];
        self.canonical_staged[401] = produced[22];
        self.canonical_staged[63] = produced[23];
        self.canonical_staged[112] = produced[24];
        self.canonical_staged[130] = produced[25];
        self.canonical_staged[405] = produced[26];
        self.canonical_staged[409] = produced[27];
        self.canonical_staged[66] = produced[28];
        self.canonical_staged[440] = produced[29];
        self.canonical_staged[443] = produced[30];
        self.canonical_staged[441] = produced[31];
        self.canonical_staged[442] = produced[32];
        self.canonical_staged[71] = produced[33];
        self.canonical_staged[74] = produced[34];
        self.canonical_staged[444] = produced[35];
        self.canonical_staged[445] = produced[36];
        self.canonical_staged[449] = produced[37];
        self.canonical_staged[77] = produced[38];
        self.canonical_staged[78] = produced[39];
        self.canonical_staged[79] = produced[40];
        self.canonical_staged[81] = produced[41];
        self.canonical_staged[455] = produced[42];
        self.canonical_staged[458] = produced[43];
        self.canonical_staged[456] = produced[44];
        self.canonical_staged[457] = produced[45];
        self.canonical_staged[87] = produced[46];
        self.canonical_staged[90] = produced[47];
        self.canonical_staged[459] = produced[48];
        self.canonical_staged[460] = produced[49];
        self.canonical_staged[464] = produced[50];
        self.canonical_staged[93] = produced[51];
        self.canonical_staged[94] = produced[52];
        self.canonical_staged[95] = produced[53];
        self.canonical_staged[470] = produced[54];
        self.canonical_staged[473] = produced[55];
        self.canonical_staged[471] = produced[56];
        self.canonical_staged[472] = produced[57];
        self.canonical_staged[101] = produced[58];
        self.canonical_staged[104] = produced[59];
        self.canonical_staged[474] = produced[60];
        self.canonical_staged[475] = produced[61];
        self.canonical_staged[479] = produced[62];
        self.canonical_staged[107] = produced[63];
        self.canonical_staged[108] = produced[64];
        self.canonical_staged[109] = produced[65];
        self.canonical_staged[488] = produced[66];
        self.canonical_staged[113] = produced[67];
        self.canonical_staged[519] = produced[68];
        self.canonical_staged[522] = produced[69];
        self.canonical_staged[520] = produced[70];
        self.canonical_staged[521] = produced[71];
        self.canonical_staged[114] = produced[72];
        self.canonical_staged[115] = produced[73];
        self.canonical_staged[523] = produced[74];
        self.canonical_staged[524] = produced[75];
        self.canonical_staged[528] = produced[76];
        self.canonical_staged[116] = produced[77];
        self.canonical_staged[117] = produced[78];
        self.canonical_staged[118] = produced[79];
        self.canonical_staged[119] = produced[80];
        self.canonical_staged[534] = produced[81];
        self.canonical_staged[537] = produced[82];
        self.canonical_staged[535] = produced[83];
        self.canonical_staged[536] = produced[84];
        self.canonical_staged[120] = produced[85];
        self.canonical_staged[121] = produced[86];
        self.canonical_staged[538] = produced[87];
        self.canonical_staged[539] = produced[88];
        self.canonical_staged[543] = produced[89];
        self.canonical_staged[122] = produced[90];
        self.canonical_staged[123] = produced[91];
        self.canonical_staged[124] = produced[92];
        self.canonical_staged[549] = produced[93];
        self.canonical_staged[552] = produced[94];
        self.canonical_staged[550] = produced[95];
        self.canonical_staged[551] = produced[96];
        self.canonical_staged[125] = produced[97];
        self.canonical_staged[126] = produced[98];
        self.canonical_staged[553] = produced[99];
        self.canonical_staged[554] = produced[100];
        self.canonical_staged[558] = produced[101];
        self.canonical_staged[127] = produced[102];
        self.canonical_staged[128] = produced[103];
        self.canonical_staged[129] = produced[104];
        self.canonical_staged[567] = produced[105];
        self.canonical_staged[131] = produced[106];
        self.canonical_staged[598] = produced[107];
        self.canonical_staged[601] = produced[108];
        self.canonical_staged[599] = produced[109];
        self.canonical_staged[600] = produced[110];
        self.canonical_staged[132] = produced[111];
        self.canonical_staged[133] = produced[112];
        self.canonical_staged[602] = produced[113];
        self.canonical_staged[603] = produced[114];
        self.canonical_staged[607] = produced[115];
        self.canonical_staged[134] = produced[116];
        self.canonical_staged[135] = produced[117];
        self.canonical_staged[136] = produced[118];
        self.canonical_staged[137] = produced[119];
        self.canonical_staged[613] = produced[120];
        self.canonical_staged[616] = produced[121];
        self.canonical_staged[614] = produced[122];
        self.canonical_staged[615] = produced[123];
        self.canonical_staged[138] = produced[124];
        self.canonical_staged[139] = produced[125];
        self.canonical_staged[617] = produced[126];
        self.canonical_staged[618] = produced[127];
        self.canonical_staged[622] = produced[128];
        self.canonical_staged[140] = produced[129];
        self.canonical_staged[141] = produced[130];
        self.canonical_staged[142] = produced[131];
        self.canonical_staged[628] = produced[132];
        self.canonical_staged[631] = produced[133];
        self.canonical_staged[629] = produced[134];
        self.canonical_staged[630] = produced[135];
        self.canonical_staged[143] = produced[136];
        self.canonical_staged[144] = produced[137];
        self.canonical_staged[632] = produced[138];
        self.canonical_staged[633] = produced[139];
        self.canonical_staged[637] = produced[140];
        self.canonical_staged[145] = produced[141];
        self.canonical_staged[146] = produced[142];
        self.canonical_staged[147] = produced[143];
        self.canonical_staged[676] = produced[144];
        self.canonical_staged[679] = produced[145];
        self.canonical_staged[677] = produced[146];
        self.canonical_staged[678] = produced[147];
        self.canonical_staged[148] = produced[148];
        self.canonical_staged[149] = produced[149];
        self.canonical_staged[680] = produced[150];
        self.canonical_staged[681] = produced[151];
        self.canonical_staged[685] = produced[152];
        self.canonical_staged[150] = produced[153];
        self.canonical_staged[151] = produced[154];
        self.canonical_staged[152] = produced[155];
        self.canonical_staged[153] = produced[156];
        self.canonical_staged[691] = produced[157];
        self.canonical_staged[694] = produced[158];
        self.canonical_staged[692] = produced[159];
        self.canonical_staged[693] = produced[160];
        self.canonical_staged[154] = produced[161];
        self.canonical_staged[155] = produced[162];
        self.canonical_staged[695] = produced[163];
        self.canonical_staged[696] = produced[164];
        self.canonical_staged[700] = produced[165];
        self.canonical_staged[156] = produced[166];
        self.canonical_staged[157] = produced[167];
        self.canonical_staged[158] = produced[168];
        self.canonical_staged[706] = produced[169];
        self.canonical_staged[709] = produced[170];
        self.canonical_staged[707] = produced[171];
        self.canonical_staged[708] = produced[172];
        self.canonical_staged[159] = produced[173];
        self.canonical_staged[160] = produced[174];
        self.canonical_staged[710] = produced[175];
        self.canonical_staged[711] = produced[176];
        self.canonical_staged[715] = produced[177];
        self.canonical_staged[161] = produced[178];
        self.canonical_staged[162] = produced[179];
        self.canonical_staged[163] = produced[180];
        self.canonical_staged[754] = produced[181];
        self.canonical_staged[757] = produced[182];
        self.canonical_staged[755] = produced[183];
        self.canonical_staged[756] = produced[184];
        self.canonical_staged[164] = produced[185];
        self.canonical_staged[165] = produced[186];
        self.canonical_staged[758] = produced[187];
        self.canonical_staged[759] = produced[188];
        self.canonical_staged[763] = produced[189];
        self.canonical_staged[166] = produced[190];
        self.canonical_staged[167] = produced[191];
        self.canonical_staged[168] = produced[192];
        self.canonical_staged[169] = produced[193];
        self.canonical_staged[769] = produced[194];
        self.canonical_staged[772] = produced[195];
        self.canonical_staged[770] = produced[196];
        self.canonical_staged[771] = produced[197];
        self.canonical_staged[170] = produced[198];
        self.canonical_staged[171] = produced[199];
        self.canonical_staged[773] = produced[200];
        self.canonical_staged[774] = produced[201];
        self.canonical_staged[778] = produced[202];
        self.canonical_staged[172] = produced[203];
        self.canonical_staged[173] = produced[204];
        self.canonical_staged[174] = produced[205];
        self.canonical_staged[784] = produced[206];
        self.canonical_staged[787] = produced[207];
        self.canonical_staged[785] = produced[208];
        self.canonical_staged[786] = produced[209];
        self.canonical_staged[175] = produced[210];
        self.canonical_staged[176] = produced[211];
        self.canonical_staged[788] = produced[212];
        self.canonical_staged[789] = produced[213];
        self.canonical_staged[793] = produced[214];
        self.canonical_staged[177] = produced[215];
        self.canonical_staged[178] = produced[216];
        self.canonical_staged[179] = produced[217];
        self.canonical_staged[180] = produced[218];
        self.canonical_staged[182] = produced[219];
        self.canonical_staged[807] = produced[220];
        self.canonical_staged[218] = produced[221];
        self.canonical_staged[816] = produced[222];
        self.canonical_staged[819] = produced[223];
        self.canonical_staged[817] = produced[224];
        self.canonical_staged[818] = produced[225];
        self.canonical_staged[221] = produced[226];
        self.canonical_staged[225] = produced[227];
        self.canonical_staged[820] = produced[228];
        self.canonical_staged[821] = produced[229];
        self.canonical_staged[822] = produced[230];
        self.canonical_staged[228] = produced[231];
        self.canonical_staged[824] = produced[232];
        self.canonical_staged[827] = produced[233];
        self.canonical_staged[825] = produced[234];
        self.canonical_staged[826] = produced[235];
        self.canonical_staged[235] = produced[236];
        self.canonical_staged[239] = produced[237];
        self.canonical_staged[828] = produced[238];
        self.canonical_staged[829] = produced[239];
        self.canonical_staged[830] = produced[240];
        self.canonical_staged[242] = produced[241];
        self.canonical_staged[832] = produced[242];
        self.canonical_staged[835] = produced[243];
        self.canonical_staged[833] = produced[244];
        self.canonical_staged[834] = produced[245];
        self.canonical_staged[249] = produced[246];
        self.canonical_staged[253] = produced[247];
        self.canonical_staged[836] = produced[248];
        self.canonical_staged[837] = produced[249];
        self.canonical_staged[838] = produced[250];
        self.canonical_staged[256] = produced[251];
        self.canonical_staged[276] = produced[252];
        self.canonical_staged[277] = produced[253];
        self.canonical_staged[284] = produced[254];
        self.canonical_staged[287] = produced[255];
        self.canonical_staged[288] = produced[256];
        self.canonical_staged[289] = produced[257];
        self.canonical_staged[290] = produced[258];
        self.canonical_staged[291] = produced[259];
        self.canonical_staged[292] = produced[260];
        self.canonical_staged[293] = produced[261];
        self.canonical_staged[294] = produced[262];
        self.canonical_staged[295] = produced[263];
        self.canonical_staged[296] = produced[264];
        self.canonical_staged[297] = produced[265];
        self.canonical_staged[298] = produced[266];
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
        let produced: [f64; 402] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let B = staged[0];
                let E = 1e0f64;
                let L = 5e-1f64;
                let S = staged[6];
                let U = staged[8];
                let W = staged[10];
                let AB = staged[14];
                let AC = 2e0f64;
                let AF = staged[15];
                let AH = staged[16];
                let AJ = 5e-2f64;
                let AQ = staged[17];
                let AS = staged[19];
                let AU = staged[21];
                let BP = 0e0f64;
                let BZ = 1e-1f64;
                let CB = 1e1f64;
                let CX = staged[40];
                let DH = staged[43];
                let DK = staged[44];
                let DM = 1e8f64;
                let DO = staged[45];
                let DT = staged[46];
                let EA = 2.3025850929940458e2f64;
                let EF = staged[382];
                let EG = 3.333333333333333e-1f64;
                let EH = 1e-100f64;
                let EK = 1e100f64;
                let EP = staged[383];
                let ES = staged[384];
                let FN = staged[401];
                let FO = staged[405];
                let GF = staged[63];
                let GR = parameters[85];
                let HB = staged[409];
                let HK = parameters[86];
                let HM = 1e-2f64;
                let HO = 4e0f64;
                let IN = staged[64];
                let JU = staged[65];
                let PT = 3e0f64;
                let QC = staged[441];
                let QF = staged[442];
                let QL = staged[443];
                let QP = staged[72];
                let QT = staged[73];
                let QW = staged[69];
                let QY = staged[68];
                let RA = 6.66666666666667e-1f64;
                let RH = staged[444];
                let RJ = staged[445];
                let RO = 3.75e-1f64;
                let RU = 5.178164370971076e-1f64;
                let SD = 2.9214664e-1f64;
                let SF = 2.6992878119627894e-1f64;
                let SG = 4.3792457880372104e-1f64;
                let SK = staged[70];
                let SR = 1e6f64;
                let SY = staged[75];
                let TE = staged[80];
                let TF = staged[81];
                let TJ = staged[82];
                let TL = staged[83];
                let TV = staged[456];
                let TY = staged[457];
                let UE = staged[458];
                let UI = staged[88];
                let UM = staged[89];
                let UP = staged[85];
                let UR = staged[84];
                let UZ = staged[459];
                let VB = staged[460];
                let VX = staged[86];
                let WK = staged[91];
                let WT = staged[96];
                let WV = staged[97];
                let XG = staged[471];
                let XJ = staged[472];
                let XP = staged[473];
                let XT = staged[102];
                let XX = staged[103];
                let YA = staged[99];
                let YC = staged[98];
                let YK = staged[474];
                let YM = staged[475];
                let ZI = staged[100];
                let ZV = staged[105];
                let AAE = staged[110];
                let AAG = staged[111];
                let AAO = staged[112];
                let ABJ = staged[488];
                let AKE = staged[520];
                let AKH = staged[521];
                let AKN = staged[522];
                let ALE = staged[523];
                let ALG = staged[524];
                let AMT = staged[119];
                let ANH = staged[535];
                let ANK = staged[536];
                let ANQ = staged[537];
                let AOH = staged[538];
                let AOJ = staged[539];
                let AQK = staged[550];
                let AQN = staged[551];
                let AQT = staged[552];
                let ARK = staged[553];
                let ARM = staged[554];
                let ATK = staged[130];
                let AUF = staged[567];
                let BDA = staged[599];
                let BDD = staged[600];
                let BDJ = staged[601];
                let BEA = staged[602];
                let BEC = staged[603];
                let BFP = staged[137];
                let BGD = staged[614];
                let BGG = staged[615];
                let BGM = staged[616];
                let BHD = staged[617];
                let BHF = staged[618];
                let BJG = staged[629];
                let BJJ = staged[630];
                let BJP = staged[631];
                let BKG = staged[632];
                let BKI = staged[633];
                let BNA = 1.0f64;
                let BVV = staged[677];
                let BVY = staged[678];
                let BWE = staged[679];
                let BWV = staged[680];
                let BWX = staged[681];
                let BYK = staged[153];
                let BYY = staged[692];
                let BZB = staged[693];
                let BZH = staged[694];
                let BZY = staged[695];
                let CAA = staged[696];
                let CCB = staged[707];
                let CCE = staged[708];
                let CCK = staged[709];
                let CDB = staged[710];
                let CDD = staged[711];
                let CFB = 2e-1f64;
                let CFW = 1.0f64;
                let COR = staged[755];
                let COU = staged[756];
                let CPA = staged[757];
                let CPR = staged[758];
                let CPT = staged[759];
                let CRG = staged[169];
                let CRU = staged[770];
                let CRX = staged[771];
                let CSD = staged[772];
                let CSU = staged[773];
                let CSW = staged[774];
                let CUX = staged[180];
                let CVB = staged[785];
                let CVE = staged[786];
                let CVK = staged[787];
                let CWB = staged[788];
                let CWD = staged[789];
                let CYM = 1e-3f64;
                let CZY = 1e-21f64;
                let CZZ = staged[182];
                let DAM = staged[807];
                let DAU = 1e0f64;
                let DBE = staged[819];
                let DBG = staged[821];
                let DBK = staged[827];
                let DBM = staged[829];
                let DBQ = staged[835];
                let DBS = staged[837];
                let DBW = staged[840];
                let DBX = staged[841];
                let DCH = 0e0f64;
                let mut oED = 0.0;
                let mut oGG = 0.0;
                let mut oGO = 0.0;
                let mut oGS = 0.0;
                let mut oHD = 0.0;
                let mut oHF = 0.0;
                let mut oHQ = 0.0;
                let mut oHU = 0.0;
                let mut oHZ = 0.0;
                let mut oIL = 0.0;
                let mut oIP = 0.0;
                let mut oIX = 0.0;
                let mut oJB = 0.0;
                let mut oJG = 0.0;
                let mut oJS = 0.0;
                let mut oJW = 0.0;
                let mut oKE = 0.0;
                let mut oKI = 0.0;
                let mut oKN = 0.0;
                let mut oKZ = 0.0;
                let mut oLI = 0.0;
                let mut oLO = 0.0;
                let mut oLV = 0.0;
                let mut oMK = 0.0;
                let mut oMO = 0.0;
                let mut oMW = 0.0;
                let mut oNC = 0.0;
                let mut oNJ = 0.0;
                let mut oNY = 0.0;
                let mut oOC = 0.0;
                let mut oOK = 0.0;
                let mut oOQ = 0.0;
                let mut oOX = 0.0;
                let mut oPM = 0.0;
                let mut oRT = 0.0;
                let mut oRZ = 0.0;
                let mut oSI = 0.0;
                let mut oSS = 0.0;
                let mut oSU = 0.0;
                let mut oSW = 0.0;
                let mut oTG = 0.0;
                let mut oTK = 0.0;
                let mut oVK = 0.0;
                let mut oVP = 0.0;
                let mut oVV = 0.0;
                let mut oWE = 0.0;
                let mut oWG = 0.0;
                let mut oWI = 0.0;
                let mut oWQ = 0.0;
                let mut oWU = 0.0;
                let mut oYV = 0.0;
                let mut oZA = 0.0;
                let mut oZG = 0.0;
                let mut oZP = 0.0;
                let mut oZR = 0.0;
                let mut oZT = 0.0;
                let mut oAAB = 0.0;
                let mut oAAF = 0.0;
                let mut oAAP = 0.0;
                let mut oAAX = 0.0;
                let mut oABA = 0.0;
                let mut oABL = 0.0;
                let mut oABN = 0.0;
                let mut oABV = 0.0;
                let mut oABZ = 0.0;
                let mut oACE = 0.0;
                let mut oACQ = 0.0;
                let mut oACT = 0.0;
                let mut oADB = 0.0;
                let mut oADF = 0.0;
                let mut oADK = 0.0;
                let mut oADW = 0.0;
                let mut oADZ = 0.0;
                let mut oAEH = 0.0;
                let mut oAEL = 0.0;
                let mut oAEQ = 0.0;
                let mut oAFC = 0.0;
                let mut oAFL = 0.0;
                let mut oAFR = 0.0;
                let mut oAFY = 0.0;
                let mut oAGN = 0.0;
                let mut oAGR = 0.0;
                let mut oAGZ = 0.0;
                let mut oAHF = 0.0;
                let mut oAHM = 0.0;
                let mut oAIB = 0.0;
                let mut oAIF = 0.0;
                let mut oAIN = 0.0;
                let mut oAIT = 0.0;
                let mut oAJA = 0.0;
                let mut oAJP = 0.0;
                let mut oALP = 0.0;
                let mut oALU = 0.0;
                let mut oAMA = 0.0;
                let mut oAMI = 0.0;
                let mut oAMK = 0.0;
                let mut oAMM = 0.0;
                let mut oAMU = 0.0;
                let mut oAMX = 0.0;
                let mut oAOS = 0.0;
                let mut oAOX = 0.0;
                let mut oAPD = 0.0;
                let mut oAPL = 0.0;
                let mut oAPN = 0.0;
                let mut oAPP = 0.0;
                let mut oAPW = 0.0;
                let mut oAPZ = 0.0;
                let mut oARV = 0.0;
                let mut oASA = 0.0;
                let mut oASG = 0.0;
                let mut oASO = 0.0;
                let mut oASQ = 0.0;
                let mut oASS = 0.0;
                let mut oASZ = 0.0;
                let mut oATC = 0.0;
                let mut oATL = 0.0;
                let mut oATT = 0.0;
                let mut oATW = 0.0;
                let mut oAUH = 0.0;
                let mut oAUJ = 0.0;
                let mut oAUR = 0.0;
                let mut oAUV = 0.0;
                let mut oAVA = 0.0;
                let mut oAVM = 0.0;
                let mut oAVP = 0.0;
                let mut oAVX = 0.0;
                let mut oAWB = 0.0;
                let mut oAWG = 0.0;
                let mut oAWS = 0.0;
                let mut oAWV = 0.0;
                let mut oAXD = 0.0;
                let mut oAXH = 0.0;
                let mut oAXM = 0.0;
                let mut oAXY = 0.0;
                let mut oAYH = 0.0;
                let mut oAYN = 0.0;
                let mut oAYU = 0.0;
                let mut oAZJ = 0.0;
                let mut oAZN = 0.0;
                let mut oAZV = 0.0;
                let mut oBAB = 0.0;
                let mut oBAI = 0.0;
                let mut oBAX = 0.0;
                let mut oBBB = 0.0;
                let mut oBBJ = 0.0;
                let mut oBBP = 0.0;
                let mut oBBW = 0.0;
                let mut oBCL = 0.0;
                let mut oBEL = 0.0;
                let mut oBEQ = 0.0;
                let mut oBEW = 0.0;
                let mut oBFE = 0.0;
                let mut oBFG = 0.0;
                let mut oBFI = 0.0;
                let mut oBFQ = 0.0;
                let mut oBFT = 0.0;
                let mut oBHO = 0.0;
                let mut oBHT = 0.0;
                let mut oBHZ = 0.0;
                let mut oBIH = 0.0;
                let mut oBIJ = 0.0;
                let mut oBIL = 0.0;
                let mut oBIS = 0.0;
                let mut oBIV = 0.0;
                let mut oBKR = 0.0;
                let mut oBKW = 0.0;
                let mut oBLC = 0.0;
                let mut oBLK = 0.0;
                let mut oBLM = 0.0;
                let mut oBLO = 0.0;
                let mut oBLV = 0.0;
                let mut oBLY = 0.0;
                let mut oBMG = 0.0;
                let mut oBMO = 0.0;
                let mut oBMR = 0.0;
                let mut oBNC = 0.0;
                let mut oBNE = 0.0;
                let mut oBNM = 0.0;
                let mut oBNQ = 0.0;
                let mut oBNV = 0.0;
                let mut oBOH = 0.0;
                let mut oBOK = 0.0;
                let mut oBOS = 0.0;
                let mut oBOW = 0.0;
                let mut oBPB = 0.0;
                let mut oBPN = 0.0;
                let mut oBPQ = 0.0;
                let mut oBPY = 0.0;
                let mut oBQC = 0.0;
                let mut oBQH = 0.0;
                let mut oBQT = 0.0;
                let mut oBRC = 0.0;
                let mut oBRI = 0.0;
                let mut oBRP = 0.0;
                let mut oBSE = 0.0;
                let mut oBSI = 0.0;
                let mut oBSQ = 0.0;
                let mut oBSW = 0.0;
                let mut oBTD = 0.0;
                let mut oBTS = 0.0;
                let mut oBTW = 0.0;
                let mut oBUE = 0.0;
                let mut oBUK = 0.0;
                let mut oBUR = 0.0;
                let mut oBVG = 0.0;
                let mut oBXG = 0.0;
                let mut oBXL = 0.0;
                let mut oBXR = 0.0;
                let mut oBXZ = 0.0;
                let mut oBYB = 0.0;
                let mut oBYD = 0.0;
                let mut oBYL = 0.0;
                let mut oBYO = 0.0;
                let mut oCAJ = 0.0;
                let mut oCAO = 0.0;
                let mut oCAU = 0.0;
                let mut oCBC = 0.0;
                let mut oCBE = 0.0;
                let mut oCBG = 0.0;
                let mut oCBN = 0.0;
                let mut oCBQ = 0.0;
                let mut oCDM = 0.0;
                let mut oCDR = 0.0;
                let mut oCDX = 0.0;
                let mut oCEF = 0.0;
                let mut oCEH = 0.0;
                let mut oCEJ = 0.0;
                let mut oCEQ = 0.0;
                let mut oCET = 0.0;
                let mut oCFC = 0.0;
                let mut oCFK = 0.0;
                let mut oCFN = 0.0;
                let mut oCFY = 0.0;
                let mut oCGA = 0.0;
                let mut oCGI = 0.0;
                let mut oCGM = 0.0;
                let mut oCGR = 0.0;
                let mut oCHD = 0.0;
                let mut oCHG = 0.0;
                let mut oCHO = 0.0;
                let mut oCHS = 0.0;
                let mut oCHX = 0.0;
                let mut oCIJ = 0.0;
                let mut oCIM = 0.0;
                let mut oCIU = 0.0;
                let mut oCIY = 0.0;
                let mut oCJD = 0.0;
                let mut oCJP = 0.0;
                let mut oCJY = 0.0;
                let mut oCKE = 0.0;
                let mut oCKL = 0.0;
                let mut oCLA = 0.0;
                let mut oCLE = 0.0;
                let mut oCLM = 0.0;
                let mut oCLS = 0.0;
                let mut oCLZ = 0.0;
                let mut oCMO = 0.0;
                let mut oCMS = 0.0;
                let mut oCNA = 0.0;
                let mut oCNG = 0.0;
                let mut oCNN = 0.0;
                let mut oCOC = 0.0;
                let mut oCQC = 0.0;
                let mut oCQH = 0.0;
                let mut oCQN = 0.0;
                let mut oCQV = 0.0;
                let mut oCQX = 0.0;
                let mut oCQZ = 0.0;
                let mut oCRH = 0.0;
                let mut oCRK = 0.0;
                let mut oCTF = 0.0;
                let mut oCTK = 0.0;
                let mut oCTQ = 0.0;
                let mut oCTY = 0.0;
                let mut oCUA = 0.0;
                let mut oCUC = 0.0;
                let mut oCUJ = 0.0;
                let mut oCUM = 0.0;
                let mut oCWM = 0.0;
                let mut oCWR = 0.0;
                let mut oCWX = 0.0;
                let mut oCXF = 0.0;
                let mut oCXH = 0.0;
                let mut oCXJ = 0.0;
                let mut oCXQ = 0.0;
                let mut oCXT = 0.0;
                let mut oCYB = 0.0;
                let mut oCYL = 0.0;
                let mut oCYN = 0.0;
                let mut oCYW = 0.0;
                let mut oCZB = 0.0;
                let mut oCZJ = 0.0;
                let mut oCZU = 0.0;
                let mut oCZW = 0.0;
                let mut oDAN = 0.0;
                let mut oDAO = 0.0;
                let mut oDAP = 0.0;
                let mut oDAQ = 0.0;
                let mut oDAR = 0.0;
                let mut oDAS = 0.0;
                let mut oDAT = 0.0;
                let mut oDAV = 0.0;
                let mut oDAW = 0.0;
                let mut oDAX = 0.0;
                let mut oDAY = 0.0;
                let mut oDAZ = 0.0;
                let mut oDBA = 0.0;
                let mut oDBB = 0.0;
                let mut oDBC = 0.0;
                let mut oDBD = 0.0;
                let mut oDBF = 0.0;
                let mut oDBH = 0.0;
                let mut oDBI = 0.0;
                let mut oDBJ = 0.0;
                let mut oDBL = 0.0;
                let mut oDBN = 0.0;
                let mut oDBO = 0.0;
                let mut oDBP = 0.0;
                let mut oDBR = 0.0;
                let mut oDBT = 0.0;
                let mut oDBU = 0.0;
                let mut oDBV = 0.0;
                let mut oDCB = 0.0;
                let mut oDCC = 0.0;
                let mut oDCG = 0.0;
                let A = if (temperature + parameters[102]) >= 2.3149999999999977e1f64 { (temperature + parameters[102]) } else { 2.3149999999999977e1f64 };
                let C = A / B;
                let D = 8.61726105451295e-5f64 * A;
                let F = E / D;
                let G = (-((7.02e-4f64 * A) * A)) / (1.108e3f64 + A);
                let H = parameters[17] + G;
                let I = parameters[18] + G;
                let J = parameters[19] + G;
                let K = C.powf(staged[1]);
                let M = L * (staged[2] - (H * F));
                let N = K * (M.exp());
                let O = L * (staged[3] - (I * F));
                let P = K * (O.exp());
                let Q = L * (staged[4] - (J * F));
                let R = K * (Q.exp());
                let T = (C.powf(staged[5])) * ((M / S).exp());
                let V = (C.powf(staged[7])) * ((O / U).exp());
                let X = (C.powf(staged[9])) * ((Q / W).exp());
                let Y = (staged[11] * T) * T;
                let Z = (staged[12] * V) * V;
                let AA = (staged[13] * X) * X;
                let AD = AC * D;
                let AE = (AB * C) - (AD * (N.ln()));
                let AG = (AF * C) - (AD * (P.ln()));
                let AI = (AH * C) - (AD * (R.ln()));
                let AK = AE + (D * ((E + (((AJ - AE) * F).exp())).ln()));
                let AL = AG + (D * ((E + (((AJ - AG) * F).exp())).ln()));
                let AM = AI + (D * ((E + (((AJ - AI) * F).exp())).ln()));
                let AN = E / AK;
                let AO = E / AL;
                let AP = E / AM;
                let AR = staged[18] * ((AB * AN).powf(AQ));
                let AT = staged[20] * ((AF * AO).powf(AS));
                let AV = staged[22] * ((AH * AP).powf(AU));
                let AW = (AR * AK) * staged[23];
                let AX = (AT * AL) * staged[24];
                let AY = (AV * AM) * staged[25];
                let AZ = AC * AR;
                let BA = AC * AT;
                let BB = AC * AV;
                let BC = if (L * H) >= D { (L * H) } else { D };
                let BD = if (L * I) >= D { (L * I) } else { D };
                let BE = if (L * J) >= D { (L * J) } else { D };
                let BF = BC * F;
                let BG = BD * F;
                let BH = BE * F;
                let BI = ((staged[26] * ((BC * BC) * BC)).sqrt()) / 3.1637150399999996e-34f64;
                let BJ = ((staged[27] * ((BD * BD) * BD)).sqrt()) / 3.1637150399999996e-34f64;
                let BK = ((staged[28] * ((BE * BE) * BE)).sqrt()) / 3.1637150399999996e-34f64;
                let BL = A - B;
                let BM = parameters[37] * (E + (parameters[40] * BL));
                let BN = parameters[38] * (E + (parameters[41] * BL));
                let BO = parameters[39] * (E + (parameters[42] * BL));
                let BQ = if BM > BP { 1.0 } else { 0.0 };
                let BR = if BQ != 0.0 {
                    BM
                } else {
                    BP
                };
                let BS = if BN > BP { 1.0 } else { 0.0 };
                let BT = if BS != 0.0 {
                    BN
                } else {
                    BP
                };
                let BU = if BO > BP { 1.0 } else { 0.0 };
                let BV = if BU != 0.0 {
                    BO
                } else {
                    BP
                };
                let BW = staged[29] * (E + (BL * (parameters[57] + (BL * parameters[58]))));
                let BX = staged[30] * (E + (BL * (parameters[59] + (BL * parameters[60]))));
                let BY = staged[31] * (E + (BL * (parameters[61] + (BL * parameters[62]))));
                let CA = if BW <= BZ { 1.0 } else { 0.0 };
                let CD;
                let CE;
                if CA != 0.0 {
                    CD = CB;
                    CE = BZ;
                } else {
                    let CC = E / BW;
                    CD = CC;
                    CE = BW;
                }
                let CF = if BX <= BZ { 1.0 } else { 0.0 };
                let CH;
                let CI;
                if CF != 0.0 {
                    CH = CB;
                    CI = BZ;
                } else {
                    let CG = E / BX;
                    CH = CG;
                    CI = BX;
                }
                let CJ = if BY <= BZ { 1.0 } else { 0.0 };
                let CL;
                let CM;
                if CJ != 0.0 {
                    CL = CB;
                    CM = BZ;
                } else {
                    let CK = E / BY;
                    CL = CK;
                    CM = BY;
                }
                let CN = staged[32] * CD;
                let CO = staged[33] * CH;
                let CP = staged[34] * CL;
                let CQ = C.powf(staged[35]);
                let CR = staged[36] * CQ;
                let CS = staged[37] * CQ;
                let CT = staged[38] * CQ;
                let CU = staged[39] * CQ;
                let CV = 1.45e16f64 * T;
                let CW = CV * CV;
                let CY = CW / CX;
                let CZ = C.powf(-1.5e0f64);
                let DA = (1.4500000000000002e-1f64 * CZ) / F;
                let DB = (5e-2f64 * CZ) / F;
                let DC = ((parameters[93] * (C.powf(parameters[97]))) * (((AC * DA) * DB) / (DA + DB))).sqrt();
                let DD = S / F;
                let DE = (CX / CY).ln();
                let DF = DD * DE;
                let DG = DD * (DE + (parameters[94] / DC));
                let DI = Y * DH;
                let DJ = if DI > BP { 1.0 } else { 0.0 };
                let DN = if DJ != 0.0 {
                    let DL = (D * (((DK / DI) + E).ln())) * S;
                    DL
                } else {
                    DM
                };
                let DP = Z * DO;
                let DQ = if DP > BP { 1.0 } else { 0.0 };
                let DS = if DQ != 0.0 {
                    let DR = (D * (((DK / DP) + E).ln())) * U;
                    DR
                } else {
                    DM
                };
                let DU = AA * DT;
                let DV = if DU > BP { 1.0 } else { 0.0 };
                let DX = if DV != 0.0 {
                    let DW = (D * (((DK / DU) + E).ln())) * W;
                    DW
                } else {
                    DM
                };
                let DY = if (if DN <= DS { DN } else { DS }) <= DX { (if DN <= DS { DN } else { DS }) } else { DX };
                let DZ = DY * F;
                let EB = if (DZ.abs()) < EA { 1.0 } else { 0.0 };
                let EE;
                if EB != 0.0 {
                    let EC = DZ.exp();
                    EE = EC;
                } else {
                    let ED = if DZ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    oED = ED;
                    let EM = if ED != 0.0 {
                        let EI = EH / (E + ((-2.3025850929940458e2f64 - DZ) * (E + (L * ((-2.3025850929940458e2f64 - DZ) * (E + ((-2.3025850929940458e2f64 - DZ) * EG)))))));
                        EI
                    } else {
                        let EJ = DZ - EA;
                        let EL = EK * (E + (EJ * (E + (L * (EJ * (E + (EJ * EG)))))));
                        EL
                    };
                    EE = EM;
                }
                let EO = if EF != 0.0 {
                    let EN = AL + AM;
                    EN
                } else {
                    AK
                };
                let ER = if EP != 0.0 {
                    let EQ = AK + AM;
                    EQ
                } else {
                    AL
                };
                let EU = if ES != 0.0 {
                    let ET = AK + AL;
                    ET
                } else {
                    AM
                };
                let EV = if (if EO <= ER { EO } else { ER }) <= EU { (if EO <= ER { EO } else { ER }) } else { EU };
                let EW = EV * BZ;
                let EX = EV * staged[47];
                let EY = if A > staged[57] { 1.0 } else { 0.0 };
                let EZ = if A < staged[58] { 1.0 } else { 0.0 };
                let FA = (DI + DP) + DU;
                let FB = if (DH * CR) > BP { 1.0 } else { 0.0 };
                let FD = if FB != 0.0 {
                    let FC = DH / CR;
                    FC
                } else {
                    BP
                };
                let FE = if (DO * CT) > BP { 1.0 } else { 0.0 };
                let FG = if FE != 0.0 {
                    let FF = (DO / CT) + FD;
                    FF
                } else {
                    FD
                };
                let FH = if (DT * CS) > BP { 1.0 } else { 0.0 };
                let FJ = if FH != 0.0 {
                    let FI = (DT / CS) + FG;
                    FI
                } else {
                    FG
                };
                let FK = if FJ > BP { 1.0 } else { 0.0 };
                let FM = if FK != 0.0 {
                    let FL = (E / FJ) + CU;
                    FL
                } else {
                    CU
                };
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
                if FN != 0.0 {
                    let GH;
                    let GI;
                    let GJ;
                    let GK;
                    let GL;
                    let GM;
                    if FO != 0.0 {
                        let GG = if GF < DY { 1.0 } else { 0.0 };
                        oGG = GG;
                        let GT;
                        let GU;
                        let GV;
                        let GW;
                        if GG != 0.0 {
                            let GN = L * (GF * F);
                            let GO = if (GN.abs()) < EA { 1.0 } else { 0.0 };
                            oGO = GO;
                            let HE;
                            if GO != 0.0 {
                                let HC = GN.exp();
                                HE = HC;
                            } else {
                                let HD = if GN < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oHD = HD;
                                let HJ = if HD != 0.0 {
                                    let HG = EH / (E + ((-2.3025850929940458e2f64 - GN) * (E + (L * ((-2.3025850929940458e2f64 - GN) * (E + ((-2.3025850929940458e2f64 - GN) * EG)))))));
                                    HG
                                } else {
                                    let HH = GN - EA;
                                    let HI = EK * (E + (HH * (E + (L * (HH * (E + (HH * EG)))))));
                                    HI
                                };
                                HE = HJ;
                            }
                            let HF = if S < GR { 1.0 } else { 0.0 };
                            oHF = HF;
                            let HR;
                            let HS;
                            if HF != 0.0 {
                                let HL = S - (HK * DF);
                                let HN = (GR - ((HK * (GF - DF)) + S)) - HM;
                                let HP = (HO * GR) * HM;
                                let HQ = if HP > BP { 1.0 } else { 0.0 };
                                oHQ = HQ;
                                let HW = if HQ != 0.0 {
                                    HP
                                } else {
                                    let HV = -HP;
                                    HV
                                };
                                let HX = ((GR - (L * (HN + (((HN * HN) + HW).sqrt())))) - S) - HM;
                                let HY = (HO * S) * HM;
                                let HZ = if HY > BP { 1.0 } else { 0.0 };
                                oHZ = HZ;
                                let IB = if HZ != 0.0 {
                                    HY
                                } else {
                                    let IA = -HY;
                                    IA
                                };
                                let IC = S + (L * (HX + (((HX * HX) + IB).sqrt())));
                                let ID = (GR - HL) - HM;
                                let IF = if HQ != 0.0 {
                                    HP
                                } else {
                                    let IE = -HP;
                                    IE
                                };
                                let IG = ((GR - (L * (ID + (((ID * ID) + IF).sqrt())))) - S) - HM;
                                let II = if HZ != 0.0 {
                                    HY
                                } else {
                                    let IH = -HY;
                                    IH
                                };
                                let IJ = S + (L * (IG + (((IG * IG) + II).sqrt())));
                                HR = IC;
                                HS = IJ;
                            } else {
                                HR = S;
                                HS = S;
                            }
                            let HT = F * ((GF / HR) + ((DF * (HR - HS)) / (HS * GR)));
                            let HU = if (HT.abs()) < EA { 1.0 } else { 0.0 };
                            oHU = HU;
                            let IM;
                            if HU != 0.0 {
                                let IK = HT.exp();
                                IM = IK;
                            } else {
                                let IL = if HT < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oIL = IL;
                                let IT = if IL != 0.0 {
                                    let IQ = EH / (E + ((-2.3025850929940458e2f64 - HT) * (E + (L * ((-2.3025850929940458e2f64 - HT) * (E + ((-2.3025850929940458e2f64 - HT) * EG)))))));
                                    IQ
                                } else {
                                    let IR = HT - EA;
                                    let IS = EK * (E + (IR * (E + (L * (IR * (E + (IR * EG)))))));
                                    IS
                                };
                                IM = IT;
                            }
                            let IO = (U / F) * ((IN / (CW / IN)).ln());
                            let IP = if U < GR { 1.0 } else { 0.0 };
                            oIP = IP;
                            let IY;
                            let IZ;
                            if IP != 0.0 {
                                let IU = U - (HK * IO);
                                let IV = (GR - ((HK * (GF - IO)) + U)) - HM;
                                let IW = (HO * GR) * HM;
                                let IX = if IW > BP { 1.0 } else { 0.0 };
                                oIX = IX;
                                let JD = if IX != 0.0 {
                                    IW
                                } else {
                                    let JC = -IW;
                                    JC
                                };
                                let JE = ((GR - (L * (IV + (((IV * IV) + JD).sqrt())))) - U) - HM;
                                let JF = (HO * U) * HM;
                                let JG = if JF > BP { 1.0 } else { 0.0 };
                                oJG = JG;
                                let JI = if JG != 0.0 {
                                    JF
                                } else {
                                    let JH = -JF;
                                    JH
                                };
                                let JJ = U + (L * (JE + (((JE * JE) + JI).sqrt())));
                                let JK = (GR - IU) - HM;
                                let JM = if IX != 0.0 {
                                    IW
                                } else {
                                    let JL = -IW;
                                    JL
                                };
                                let JN = ((GR - (L * (JK + (((JK * JK) + JM).sqrt())))) - U) - HM;
                                let JP = if JG != 0.0 {
                                    JF
                                } else {
                                    let JO = -JF;
                                    JO
                                };
                                let JQ = U + (L * (JN + (((JN * JN) + JP).sqrt())));
                                IY = JJ;
                                IZ = JQ;
                            } else {
                                IY = U;
                                IZ = U;
                            }
                            let JA = F * ((GF / IY) + ((IO * (IY - IZ)) / (IZ * GR)));
                            let JB = if (JA.abs()) < EA { 1.0 } else { 0.0 };
                            oJB = JB;
                            let JT;
                            if JB != 0.0 {
                                let JR = JA.exp();
                                JT = JR;
                            } else {
                                let JS = if JA < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oJS = JS;
                                let KA = if JS != 0.0 {
                                    let JX = EH / (E + ((-2.3025850929940458e2f64 - JA) * (E + (L * ((-2.3025850929940458e2f64 - JA) * (E + ((-2.3025850929940458e2f64 - JA) * EG)))))));
                                    JX
                                } else {
                                    let JY = JA - EA;
                                    let JZ = EK * (E + (JY * (E + (L * (JY * (E + (JY * EG)))))));
                                    JZ
                                };
                                JT = KA;
                            }
                            let JV = (W / F) * ((JU / (CW / JU)).ln());
                            let JW = if W < GR { 1.0 } else { 0.0 };
                            oJW = JW;
                            let KF;
                            let KG;
                            if JW != 0.0 {
                                let KB = W - (HK * JV);
                                let KC = (GR - ((HK * (GF - JV)) + W)) - HM;
                                let KD = (HO * GR) * HM;
                                let KE = if KD > BP { 1.0 } else { 0.0 };
                                oKE = KE;
                                let KK = if KE != 0.0 {
                                    KD
                                } else {
                                    let KJ = -KD;
                                    KJ
                                };
                                let KL = ((GR - (L * (KC + (((KC * KC) + KK).sqrt())))) - W) - HM;
                                let KM = (HO * W) * HM;
                                let KN = if KM > BP { 1.0 } else { 0.0 };
                                oKN = KN;
                                let KP = if KN != 0.0 {
                                    KM
                                } else {
                                    let KO = -KM;
                                    KO
                                };
                                let KQ = W + (L * (KL + (((KL * KL) + KP).sqrt())));
                                let KR = (GR - KB) - HM;
                                let KT = if KE != 0.0 {
                                    KD
                                } else {
                                    let KS = -KD;
                                    KS
                                };
                                let KU = ((GR - (L * (KR + (((KR * KR) + KT).sqrt())))) - W) - HM;
                                let KW = if KN != 0.0 {
                                    KM
                                } else {
                                    let KV = -KM;
                                    KV
                                };
                                let KX = W + (L * (KU + (((KU * KU) + KW).sqrt())));
                                KF = KQ;
                                KG = KX;
                            } else {
                                KF = W;
                                KG = W;
                            }
                            let KH = F * ((GF / KF) + ((JV * (KF - KG)) / (KG * GR)));
                            let KI = if (KH.abs()) < EA { 1.0 } else { 0.0 };
                            oKI = KI;
                            let LA;
                            if KI != 0.0 {
                                let KY = KH.exp();
                                LA = KY;
                            } else {
                                let KZ = if KH < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oKZ = KZ;
                                let LE = if KZ != 0.0 {
                                    let LB = EH / (E + ((-2.3025850929940458e2f64 - KH) * (E + (L * ((-2.3025850929940458e2f64 - KH) * (E + ((-2.3025850929940458e2f64 - KH) * EG)))))));
                                    LB
                                } else {
                                    let LC = KH - EA;
                                    let LD = EK * (E + (LC * (E + (L * (LC * (E + (LC * EG)))))));
                                    LD
                                };
                                LA = LE;
                            }
                            GT = IM;
                            GU = JT;
                            GV = LA;
                            GW = HE;
                        } else {
                            let GP = GF - DY;
                            let GQ = ((E + (GP * F)) * EE).sqrt();
                            let GS = if S < GR { 1.0 } else { 0.0 };
                            oGS = GS;
                            let LJ;
                            let LK;
                            let LL;
                            if GS != 0.0 {
                                let LF = S - (HK * DF);
                                let LG = (GR - ((HK * (DY - DF)) + S)) - HM;
                                let LH = (HO * GR) * HM;
                                let LI = if LH > BP { 1.0 } else { 0.0 };
                                oLI = LI;
                                let LQ = if LI != 0.0 {
                                    LH
                                } else {
                                    let LP = -LH;
                                    LP
                                };
                                let LR = ((LG * LG) + LQ).sqrt();
                                let LS = L * (E + (LG / LR));
                                let LT = ((GR - (L * (LG + LR))) - S) - HM;
                                let LU = (HO * S) * HM;
                                let LV = if LU > BP { 1.0 } else { 0.0 };
                                oLV = LV;
                                let LX = if LV != 0.0 {
                                    LU
                                } else {
                                    let LW = -LU;
                                    LW
                                };
                                let LY = ((LT * LT) + LX).sqrt();
                                let LZ = L * (E + (LT / LY));
                                let MA = S + (L * (LT + LY));
                                let MB = (GR - LF) - HM;
                                let MD = if LI != 0.0 {
                                    LH
                                } else {
                                    let MC = -LH;
                                    MC
                                };
                                let ME = ((GR - (L * (MB + (((MB * MB) + MD).sqrt())))) - S) - HM;
                                let MG = if LV != 0.0 {
                                    LU
                                } else {
                                    let MF = -LU;
                                    MF
                                };
                                let MH = S + (L * (ME + (((ME * ME) + MG).sqrt())));
                                let MI = (HK * LS) * LZ;
                                LJ = MA;
                                LK = MH;
                                LL = MI;
                            } else {
                                LJ = S;
                                LK = S;
                                LL = BP;
                            }
                            let LM = LK * GR;
                            let LN = F * ((DY / LJ) + ((DF * (LJ - LK)) / LM));
                            let LO = if (LN.abs()) < EA { 1.0 } else { 0.0 };
                            oLO = LO;
                            let ML;
                            if LO != 0.0 {
                                let MJ = LN.exp();
                                ML = MJ;
                            } else {
                                let MK = if LN < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oMK = MK;
                                let MS = if MK != 0.0 {
                                    let MP = EH / (E + ((-2.3025850929940458e2f64 - LN) * (E + (L * ((-2.3025850929940458e2f64 - LN) * (E + ((-2.3025850929940458e2f64 - LN) * EG)))))));
                                    MP
                                } else {
                                    let MQ = LN - EA;
                                    let MR = EK * (E + (MQ * (E + (L * (MQ * (E + (MQ * EG)))))));
                                    MR
                                };
                                ML = MS;
                            }
                            let MM = (E + (GP * (F * (((LJ - (DY * LL)) / (LJ * LJ)) + ((DF * LL) / LM))))) * ML;
                            let MN = (U / F) * ((IN / (CW / IN)).ln());
                            let MO = if U < GR { 1.0 } else { 0.0 };
                            oMO = MO;
                            let MX;
                            let MY;
                            let MZ;
                            if MO != 0.0 {
                                let MT = U - (HK * MN);
                                let MU = (GR - ((HK * (DY - MN)) + U)) - HM;
                                let MV = (HO * GR) * HM;
                                let MW = if MV > BP { 1.0 } else { 0.0 };
                                oMW = MW;
                                let NE = if MW != 0.0 {
                                    MV
                                } else {
                                    let ND = -MV;
                                    ND
                                };
                                let NF = ((MU * MU) + NE).sqrt();
                                let NG = L * (E + (MU / NF));
                                let NH = ((GR - (L * (MU + NF))) - U) - HM;
                                let NI = (HO * U) * HM;
                                let NJ = if NI > BP { 1.0 } else { 0.0 };
                                oNJ = NJ;
                                let NL = if NJ != 0.0 {
                                    NI
                                } else {
                                    let NK = -NI;
                                    NK
                                };
                                let NM = ((NH * NH) + NL).sqrt();
                                let NN = L * (E + (NH / NM));
                                let NO = U + (L * (NH + NM));
                                let NP = (GR - MT) - HM;
                                let NR = if MW != 0.0 {
                                    MV
                                } else {
                                    let NQ = -MV;
                                    NQ
                                };
                                let NS = ((GR - (L * (NP + (((NP * NP) + NR).sqrt())))) - U) - HM;
                                let NU = if NJ != 0.0 {
                                    NI
                                } else {
                                    let NT = -NI;
                                    NT
                                };
                                let NV = U + (L * (NS + (((NS * NS) + NU).sqrt())));
                                let NW = (HK * NG) * NN;
                                MX = NO;
                                MY = NV;
                                MZ = NW;
                            } else {
                                MX = U;
                                MY = U;
                                MZ = BP;
                            }
                            let NA = MY * GR;
                            let NB = F * ((DY / MX) + ((MN * (MX - MY)) / NA));
                            let NC = if (NB.abs()) < EA { 1.0 } else { 0.0 };
                            oNC = NC;
                            let NZ;
                            if NC != 0.0 {
                                let NX = NB.exp();
                                NZ = NX;
                            } else {
                                let NY = if NB < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oNY = NY;
                                let OG = if NY != 0.0 {
                                    let OD = EH / (E + ((-2.3025850929940458e2f64 - NB) * (E + (L * ((-2.3025850929940458e2f64 - NB) * (E + ((-2.3025850929940458e2f64 - NB) * EG)))))));
                                    OD
                                } else {
                                    let OE = NB - EA;
                                    let OF = EK * (E + (OE * (E + (L * (OE * (E + (OE * EG)))))));
                                    OF
                                };
                                NZ = OG;
                            }
                            let OA = (E + (GP * (F * (((MX - (DY * MZ)) / (MX * MX)) + ((MN * MZ) / NA))))) * NZ;
                            let OB = (W / F) * ((JU / (CW / JU)).ln());
                            let OC = if W < GR { 1.0 } else { 0.0 };
                            oOC = OC;
                            let OL;
                            let OM;
                            let ON;
                            if OC != 0.0 {
                                let OH = W - (HK * OB);
                                let OI = (GR - ((HK * (DY - OB)) + W)) - HM;
                                let OJ = (HO * GR) * HM;
                                let OK = if OJ > BP { 1.0 } else { 0.0 };
                                oOK = OK;
                                let OS = if OK != 0.0 {
                                    OJ
                                } else {
                                    let OR = -OJ;
                                    OR
                                };
                                let OT = ((OI * OI) + OS).sqrt();
                                let OU = L * (E + (OI / OT));
                                let OV = ((GR - (L * (OI + OT))) - W) - HM;
                                let OW = (HO * W) * HM;
                                let OX = if OW > BP { 1.0 } else { 0.0 };
                                oOX = OX;
                                let OZ = if OX != 0.0 {
                                    OW
                                } else {
                                    let OY = -OW;
                                    OY
                                };
                                let PA = ((OV * OV) + OZ).sqrt();
                                let PB = L * (E + (OV / PA));
                                let PC = W + (L * (OV + PA));
                                let PD = (GR - OH) - HM;
                                let PF = if OK != 0.0 {
                                    OJ
                                } else {
                                    let PE = -OJ;
                                    PE
                                };
                                let PG = ((GR - (L * (PD + (((PD * PD) + PF).sqrt())))) - W) - HM;
                                let PI = if OX != 0.0 {
                                    OW
                                } else {
                                    let PH = -OW;
                                    PH
                                };
                                let PJ = W + (L * (PG + (((PG * PG) + PI).sqrt())));
                                let PK = (HK * OU) * PB;
                                OL = PC;
                                OM = PJ;
                                ON = PK;
                            } else {
                                OL = W;
                                OM = W;
                                ON = BP;
                            }
                            let OO = OM * GR;
                            let OP = F * ((DY / OL) + ((OB * (OL - OM)) / OO));
                            let OQ = if (OP.abs()) < EA { 1.0 } else { 0.0 };
                            oOQ = OQ;
                            let PN;
                            if OQ != 0.0 {
                                let PL = OP.exp();
                                PN = PL;
                            } else {
                                let PM = if OP < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oPM = PM;
                                let PS = if PM != 0.0 {
                                    let PP = EH / (E + ((-2.3025850929940458e2f64 - OP) * (E + (L * ((-2.3025850929940458e2f64 - OP) * (E + ((-2.3025850929940458e2f64 - OP) * EG)))))));
                                    PP
                                } else {
                                    let PQ = OP - EA;
                                    let PR = EK * (E + (PQ * (E + (L * (PQ * (E + (PQ * EG)))))));
                                    PR
                                };
                                PN = PS;
                            }
                            let PO = (E + (GP * (F * (((OL - (DY * ON)) / (OL * OL)) + ((OB * ON) / OO))))) * PN;
                            GT = MM;
                            GU = OA;
                            GV = PO;
                            GW = GQ;
                        }
                        let GX = GT - E;
                        let GY = GU - E;
                        let GZ = GV - E;
                        let HA = E / GW;
                        let PW = if HB != 0.0 {
                            let PU = AC * (D * (((AC + HA) + (((HA + E) * (HA + PT)).sqrt())).ln()));
                            PU
                        } else {
                            let PV = staged[66] + (AC * (D * ((((AC * GW) + E) + (((E + GW) * (E + (PT * GW))).sqrt())).ln())));
                            PV
                        };
                        let PX = EV - PW;
                        let PY = GF - PX;
                        let PZ = L * ((GF + PX) - (((PY * PY) + ((HO * D) * D)).sqrt()));
                        GH = GX;
                        GI = PZ;
                        GJ = PW;
                        GK = GW;
                        GL = GY;
                        GM = GZ;
                    } else {
                        GH = BP;
                        GI = BP;
                        GJ = BP;
                        GK = BP;
                        GL = BP;
                        GM = BP;
                    }
                    let QA;
                    if EF != 0.0 {
                        QA = BP;
                    } else {
                        let QB = Y * GH;
                        let QG;
                        let QH;
                        let QI;
                        let QJ;
                        let QK;
                        if QC != 0.0 {
                            QG = BP;
                            QH = BP;
                            QI = BP;
                            QJ = BP;
                            QK = BP;
                        } else {
                            let QD = AK - GI;
                            let QE = E - ((E - (GJ / QD)).sqrt());
                            let QN = if QF != 0.0 {
                                BP
                            } else {
                                let QM = ((((QE * QE) * (QE.ln())) / (E - QE)) + QE) * staged[71];
                                QM
                            };
                            let QO = QE + QN;
                            let QS = if QF != 0.0 {
                                let QQ = (QD * QP).sqrt();
                                QQ
                            } else {
                                let QR = (QD * QP).powf(AQ);
                                QR
                            };
                            let QU = QT * QS;
                            let QV = N * ((GK - E) * QU);
                            let QX = QW * (QV * QO);
                            QG = QU;
                            QH = QD;
                            QI = QO;
                            QJ = QV;
                            QK = QX;
                        }
                        let RI;
                        if QL != 0.0 {
                            RI = BP;
                        } else {
                            let QZ = BI * ((QG * QY) / QH);
                            let RB = (RA * BF) / QZ;
                            let RC = RB * RB;
                            let RD = RC * RC;
                            let RE = (RD / (RD + E)).sqrt();
                            let RF = (RE.abs()).sqrt();
                            let RG = RE * RF;
                            let RM = if RH != 0.0 {
                                let RK = E / (E + (QZ * RG));
                                RK
                            } else {
                                let RL = (E + (QZ * RG)).powf(staged[74]);
                                RL
                            };
                            let RN = (QI * RM) / (QI + RM);
                            let RP = (RO * (QZ / RF)).sqrt();
                            let RQ = (((BF * RB) * RF) - (BF * RE)) + (L * (QZ * RG));
                            let RR = (((AC * (RB * RF)) - RE) - E) * RP;
                            let RS = RR * RR;
                            let RT = if RR > BP { 1.0 } else { 0.0 };
                            oRT = RT;
                            let RX = if RT != 0.0 {
                                let RV = E / (E + (RU * RR));
                                RV
                            } else {
                                let RW = E / (E - (RU * RR));
                                RW
                            };
                            let RY = (-RS) + RQ;
                            let RZ = if RY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oRZ = RZ;
                            let SC = if RZ != 0.0 {
                                let SA = RY.exp();
                                SA
                            } else {
                                let SB = EH / (E + ((-2.3025850929940458e2f64 - RY) * (E + (L * ((-2.3025850929940458e2f64 - RY) * (E + ((-2.3025850929940458e2f64 - RY) * EG)))))));
                                SB
                            };
                            let SE = RX * RX;
                            let SH = (((SD * RX) + (SF * SE)) + (SG * (SE * RX))) * SC;
                            let SJ;
                            if RT != 0.0 {
                                SJ = SH;
                            } else {
                                let SI = if RQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oSI = SI;
                                let SO = if SI != 0.0 {
                                    let SM = RQ.exp();
                                    SM
                                } else {
                                    let SN = EH / (E + ((-2.3025850929940458e2f64 - RQ) * (E + (L * ((-2.3025850929940458e2f64 - RQ) * (E + ((-2.3025850929940458e2f64 - RQ) * EG)))))));
                                    SN
                                };
                                let SP = (AC * SO) - SH;
                                SJ = SP;
                            }
                            let SL = SK * ((QJ * (8.86226925452758e-1f64 * ((BF * SJ) / RP))) * RN);
                            RI = SL;
                        }
                        let SQ;
                        if RJ != 0.0 {
                            SQ = BP;
                        } else {
                            let ST = (-BR) / staged[77];
                            let SU = if (ST.abs()) < EA { 1.0 } else { 0.0 };
                            oSU = SU;
                            let SX;
                            if SU != 0.0 {
                                let SV = ST.exp();
                                SX = SV;
                            } else {
                                let SW = if ST < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oSW = SW;
                                let TD = if SW != 0.0 {
                                    let TA = EH / (E + ((-2.3025850929940458e2f64 - ST) * (E + (L * ((-2.3025850929940458e2f64 - ST) * (E + ((-2.3025850929940458e2f64 - ST) * EG)))))));
                                    TA
                                } else {
                                    let TB = ST - EA;
                                    let TC = EK * (E + (TB * (E + (L * (TB * (E + (TB * EG)))))));
                                    TC
                                };
                                SX = TD;
                            }
                            let SZ = SY * (staged[78] * SX);
                            SQ = SZ;
                        }
                        let SS = if (if CE > SR { 1.0 } else { 0.0 }) != 0.0 || staged[79] != 0.0 { 1.0 } else { 0.0 };
                        oSS = SS;
                        let TH;
                        if SS != 0.0 {
                            TH = E;
                        } else {
                            let TG = if TF > ((-TE) * CE) { 1.0 } else { 0.0 };
                            oTG = TG;
                            let TN;
                            if TG != 0.0 {
                                let TK = if TJ == HO { 1.0 } else { 0.0 };
                                oTK = TK;
                                let TR = if TK != 0.0 {
                                    let TO = (TF * CD).abs();
                                    let TP = ((TO * TO) * TO) * TO;
                                    TP
                                } else {
                                    let TQ = ((TF * CD).abs()).powf(TJ);
                                    TQ
                                };
                                let TS = E / (E - TR);
                                TN = TS;
                            } else {
                                let TM = TL + ((TF + (TE * CE)) * CN);
                                TN = TM;
                            }
                            TH = TN;
                        }
                        let TI = (((QB + QK) + RI) + SQ) * TH;
                        QA = TI;
                    }
                    let TT;
                    if EP != 0.0 {
                        TT = BP;
                    } else {
                        let TU = Z * GL;
                        let TZ;
                        let UA;
                        let UB;
                        let UC;
                        let UD;
                        if TV != 0.0 {
                            TZ = BP;
                            UA = BP;
                            UB = BP;
                            UC = BP;
                            UD = BP;
                        } else {
                            let TW = AL - GI;
                            let TX = E - ((E - (GJ / TW)).sqrt());
                            let UG = if TY != 0.0 {
                                BP
                            } else {
                                let UF = ((((TX * TX) * (TX.ln())) / (E - TX)) + TX) * staged[87];
                                UF
                            };
                            let UH = TX + UG;
                            let UL = if TY != 0.0 {
                                let UJ = (TW * UI).sqrt();
                                UJ
                            } else {
                                let UK = (TW * UI).powf(AS);
                                UK
                            };
                            let UN = UM * UL;
                            let UO = P * ((GK - E) * UN);
                            let UQ = UP * (UO * UH);
                            TZ = UN;
                            UA = TW;
                            UB = UH;
                            UC = UO;
                            UD = UQ;
                        }
                        let VA;
                        if UE != 0.0 {
                            VA = BP;
                        } else {
                            let US = BJ * ((TZ * UR) / UA);
                            let UT = (RA * BG) / US;
                            let UU = UT * UT;
                            let UV = UU * UU;
                            let UW = (UV / (UV + E)).sqrt();
                            let UX = (UW.abs()).sqrt();
                            let UY = UW * UX;
                            let VE = if UZ != 0.0 {
                                let VC = E / (E + (US * UY));
                                VC
                            } else {
                                let VD = (E + (US * UY)).powf(staged[90]);
                                VD
                            };
                            let VF = (UB * VE) / (UB + VE);
                            let VG = (RO * (US / UX)).sqrt();
                            let VH = (((BG * UT) * UX) - (BG * UW)) + (L * (US * UY));
                            let VI = (((AC * (UT * UX)) - UW) - E) * VG;
                            let VJ = VI * VI;
                            let VK = if VI > BP { 1.0 } else { 0.0 };
                            oVK = VK;
                            let VN = if VK != 0.0 {
                                let VL = E / (E + (RU * VI));
                                VL
                            } else {
                                let VM = E / (E - (RU * VI));
                                VM
                            };
                            let VO = (-VJ) + VH;
                            let VP = if VO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oVP = VP;
                            let VS = if VP != 0.0 {
                                let VQ = VO.exp();
                                VQ
                            } else {
                                let VR = EH / (E + ((-2.3025850929940458e2f64 - VO) * (E + (L * ((-2.3025850929940458e2f64 - VO) * (E + ((-2.3025850929940458e2f64 - VO) * EG)))))));
                                VR
                            };
                            let VT = VN * VN;
                            let VU = (((SD * VN) + (SF * VT)) + (SG * (VT * VN))) * VS;
                            let VW;
                            if VK != 0.0 {
                                VW = VU;
                            } else {
                                let VV = if VH > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oVV = VV;
                                let WB = if VV != 0.0 {
                                    let VZ = VH.exp();
                                    VZ
                                } else {
                                    let WA = EH / (E + ((-2.3025850929940458e2f64 - VH) * (E + (L * ((-2.3025850929940458e2f64 - VH) * (E + ((-2.3025850929940458e2f64 - VH) * EG)))))));
                                    WA
                                };
                                let WC = (AC * WB) - VU;
                                VW = WC;
                            }
                            let VY = VX * ((UC * (8.86226925452758e-1f64 * ((BG * VW) / VG))) * VF);
                            VA = VY;
                        }
                        let WD;
                        if VB != 0.0 {
                            WD = BP;
                        } else {
                            let WF = (-BT) / staged[93];
                            let WG = if (WF.abs()) < EA { 1.0 } else { 0.0 };
                            oWG = WG;
                            let WJ;
                            if WG != 0.0 {
                                let WH = WF.exp();
                                WJ = WH;
                            } else {
                                let WI = if WF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oWI = WI;
                                let WP = if WI != 0.0 {
                                    let WM = EH / (E + ((-2.3025850929940458e2f64 - WF) * (E + (L * ((-2.3025850929940458e2f64 - WF) * (E + ((-2.3025850929940458e2f64 - WF) * EG)))))));
                                    WM
                                } else {
                                    let WN = WF - EA;
                                    let WO = EK * (E + (WN * (E + (L * (WN * (E + (WN * EG)))))));
                                    WO
                                };
                                WJ = WP;
                            }
                            let WL = WK * (staged[94] * WJ);
                            WD = WL;
                        }
                        let WE = if (if CI > SR { 1.0 } else { 0.0 }) != 0.0 || staged[95] != 0.0 { 1.0 } else { 0.0 };
                        oWE = WE;
                        let WR;
                        if WE != 0.0 {
                            WR = E;
                        } else {
                            let WQ = if TF > ((-TE) * CI) { 1.0 } else { 0.0 };
                            oWQ = WQ;
                            let WX;
                            if WQ != 0.0 {
                                let WU = if WT == HO { 1.0 } else { 0.0 };
                                oWU = WU;
                                let XB = if WU != 0.0 {
                                    let WY = (TF * CH).abs();
                                    let WZ = ((WY * WY) * WY) * WY;
                                    WZ
                                } else {
                                    let XA = ((TF * CH).abs()).powf(WT);
                                    XA
                                };
                                let XC = E / (E - XB);
                                WX = XC;
                            } else {
                                let WW = WV + ((TF + (TE * CI)) * CO);
                                WX = WW;
                            }
                            WR = WX;
                        }
                        let WS = (((TU + UD) + VA) + WD) * WR;
                        TT = WS;
                    }
                    let XD;
                    if ES != 0.0 {
                        XD = BP;
                    } else {
                        let XF = AA * GM;
                        let XK;
                        let XL;
                        let XM;
                        let XN;
                        let XO;
                        if XG != 0.0 {
                            XK = BP;
                            XL = BP;
                            XM = BP;
                            XN = BP;
                            XO = BP;
                        } else {
                            let XH = AM - GI;
                            let XI = E - ((E - (GJ / XH)).sqrt());
                            let XR = if XJ != 0.0 {
                                BP
                            } else {
                                let XQ = ((((XI * XI) * (XI.ln())) / (E - XI)) + XI) * staged[101];
                                XQ
                            };
                            let XS = XI + XR;
                            let XW = if XJ != 0.0 {
                                let XU = (XH * XT).sqrt();
                                XU
                            } else {
                                let XV = (XH * XT).powf(AU);
                                XV
                            };
                            let XY = XX * XW;
                            let XZ = R * ((GK - E) * XY);
                            let YB = YA * (XZ * XS);
                            XK = XY;
                            XL = XH;
                            XM = XS;
                            XN = XZ;
                            XO = YB;
                        }
                        let YL;
                        if XP != 0.0 {
                            YL = BP;
                        } else {
                            let YD = BK * ((XK * YC) / XL);
                            let YE = (RA * BH) / YD;
                            let YF = YE * YE;
                            let YG = YF * YF;
                            let YH = (YG / (YG + E)).sqrt();
                            let YI = (YH.abs()).sqrt();
                            let YJ = YH * YI;
                            let YP = if YK != 0.0 {
                                let YN = E / (E + (YD * YJ));
                                YN
                            } else {
                                let YO = (E + (YD * YJ)).powf(staged[104]);
                                YO
                            };
                            let YQ = (XM * YP) / (XM + YP);
                            let YR = (RO * (YD / YI)).sqrt();
                            let YS = (((BH * YE) * YI) - (BH * YH)) + (L * (YD * YJ));
                            let YT = (((AC * (YE * YI)) - YH) - E) * YR;
                            let YU = YT * YT;
                            let YV = if YT > BP { 1.0 } else { 0.0 };
                            oYV = YV;
                            let YY = if YV != 0.0 {
                                let YW = E / (E + (RU * YT));
                                YW
                            } else {
                                let YX = E / (E - (RU * YT));
                                YX
                            };
                            let YZ = (-YU) + YS;
                            let ZA = if YZ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oZA = ZA;
                            let ZD = if ZA != 0.0 {
                                let ZB = YZ.exp();
                                ZB
                            } else {
                                let ZC = EH / (E + ((-2.3025850929940458e2f64 - YZ) * (E + (L * ((-2.3025850929940458e2f64 - YZ) * (E + ((-2.3025850929940458e2f64 - YZ) * EG)))))));
                                ZC
                            };
                            let ZE = YY * YY;
                            let ZF = (((SD * YY) + (SF * ZE)) + (SG * (ZE * YY))) * ZD;
                            let ZH;
                            if YV != 0.0 {
                                ZH = ZF;
                            } else {
                                let ZG = if YS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oZG = ZG;
                                let ZM = if ZG != 0.0 {
                                    let ZK = YS.exp();
                                    ZK
                                } else {
                                    let ZL = EH / (E + ((-2.3025850929940458e2f64 - YS) * (E + (L * ((-2.3025850929940458e2f64 - YS) * (E + ((-2.3025850929940458e2f64 - YS) * EG)))))));
                                    ZL
                                };
                                let ZN = (AC * ZM) - ZF;
                                ZH = ZN;
                            }
                            let ZJ = ZI * ((XN * (8.86226925452758e-1f64 * ((BH * ZH) / YR))) * YQ);
                            YL = ZJ;
                        }
                        let ZO;
                        if YM != 0.0 {
                            ZO = BP;
                        } else {
                            let ZQ = (-BV) / staged[107];
                            let ZR = if (ZQ.abs()) < EA { 1.0 } else { 0.0 };
                            oZR = ZR;
                            let ZU;
                            if ZR != 0.0 {
                                let ZS = ZQ.exp();
                                ZU = ZS;
                            } else {
                                let ZT = if ZQ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oZT = ZT;
                                let AAA = if ZT != 0.0 {
                                    let ZX = EH / (E + ((-2.3025850929940458e2f64 - ZQ) * (E + (L * ((-2.3025850929940458e2f64 - ZQ) * (E + ((-2.3025850929940458e2f64 - ZQ) * EG)))))));
                                    ZX
                                } else {
                                    let ZY = ZQ - EA;
                                    let ZZ = EK * (E + (ZY * (E + (L * (ZY * (E + (ZY * EG)))))));
                                    ZZ
                                };
                                ZU = AAA;
                            }
                            let ZW = ZV * (staged[108] * ZU);
                            ZO = ZW;
                        }
                        let ZP = if (if CM > SR { 1.0 } else { 0.0 }) != 0.0 || staged[109] != 0.0 { 1.0 } else { 0.0 };
                        oZP = ZP;
                        let AAC;
                        if ZP != 0.0 {
                            AAC = E;
                        } else {
                            let AAB = if TF > ((-TE) * CM) { 1.0 } else { 0.0 };
                            oAAB = AAB;
                            let AAI;
                            if AAB != 0.0 {
                                let AAF = if AAE == HO { 1.0 } else { 0.0 };
                                oAAF = AAF;
                                let AAM = if AAF != 0.0 {
                                    let AAJ = (TF * CL).abs();
                                    let AAK = ((AAJ * AAJ) * AAJ) * AAJ;
                                    AAK
                                } else {
                                    let AAL = ((TF * CL).abs()).powf(AAE);
                                    AAL
                                };
                                let AAN = E / (E - AAM);
                                AAI = AAN;
                            } else {
                                let AAH = AAG + ((TF + (TE * CM)) * CP);
                                AAI = AAH;
                            }
                            AAC = AAI;
                        }
                        let AAD = (((XF + XO) + YL) + ZO) * AAC;
                        XD = AAD;
                    }
                    let XE = ((DH * QA) + (DO * TT)) + (DT * XD);
                    let AAQ;
                    let AAR;
                    let AAS;
                    let AAT;
                    let AAU;
                    let AAV;
                    if FO != 0.0 {
                        let AAP = if AAO < DY { 1.0 } else { 0.0 };
                        oAAP = AAP;
                        let ABB;
                        let ABC;
                        let ABD;
                        let ABE;
                        if AAP != 0.0 {
                            let AAW = L * (AAO * F);
                            let AAX = if (AAW.abs()) < EA { 1.0 } else { 0.0 };
                            oAAX = AAX;
                            let ABM;
                            if AAX != 0.0 {
                                let ABK = AAW.exp();
                                ABM = ABK;
                            } else {
                                let ABL = if AAW < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oABL = ABL;
                                let ABR = if ABL != 0.0 {
                                    let ABO = EH / (E + ((-2.3025850929940458e2f64 - AAW) * (E + (L * ((-2.3025850929940458e2f64 - AAW) * (E + ((-2.3025850929940458e2f64 - AAW) * EG)))))));
                                    ABO
                                } else {
                                    let ABP = AAW - EA;
                                    let ABQ = EK * (E + (ABP * (E + (L * (ABP * (E + (ABP * EG)))))));
                                    ABQ
                                };
                                ABM = ABR;
                            }
                            let ABN = if S < GR { 1.0 } else { 0.0 };
                            oABN = ABN;
                            let ABW;
                            let ABX;
                            if ABN != 0.0 {
                                let ABS = S - (HK * DF);
                                let ABT = (GR - ((HK * (AAO - DF)) + S)) - HM;
                                let ABU = (HO * GR) * HM;
                                let ABV = if ABU > BP { 1.0 } else { 0.0 };
                                oABV = ABV;
                                let ACB = if ABV != 0.0 {
                                    ABU
                                } else {
                                    let ACA = -ABU;
                                    ACA
                                };
                                let ACC = ((GR - (L * (ABT + (((ABT * ABT) + ACB).sqrt())))) - S) - HM;
                                let ACD = (HO * S) * HM;
                                let ACE = if ACD > BP { 1.0 } else { 0.0 };
                                oACE = ACE;
                                let ACG = if ACE != 0.0 {
                                    ACD
                                } else {
                                    let ACF = -ACD;
                                    ACF
                                };
                                let ACH = S + (L * (ACC + (((ACC * ACC) + ACG).sqrt())));
                                let ACI = (GR - ABS) - HM;
                                let ACK = if ABV != 0.0 {
                                    ABU
                                } else {
                                    let ACJ = -ABU;
                                    ACJ
                                };
                                let ACL = ((GR - (L * (ACI + (((ACI * ACI) + ACK).sqrt())))) - S) - HM;
                                let ACN = if ACE != 0.0 {
                                    ACD
                                } else {
                                    let ACM = -ACD;
                                    ACM
                                };
                                let ACO = S + (L * (ACL + (((ACL * ACL) + ACN).sqrt())));
                                ABW = ACH;
                                ABX = ACO;
                            } else {
                                ABW = S;
                                ABX = S;
                            }
                            let ABY = F * ((AAO / ABW) + ((DF * (ABW - ABX)) / (ABX * GR)));
                            let ABZ = if (ABY.abs()) < EA { 1.0 } else { 0.0 };
                            oABZ = ABZ;
                            let ACR;
                            if ABZ != 0.0 {
                                let ACP = ABY.exp();
                                ACR = ACP;
                            } else {
                                let ACQ = if ABY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oACQ = ACQ;
                                let ACX = if ACQ != 0.0 {
                                    let ACU = EH / (E + ((-2.3025850929940458e2f64 - ABY) * (E + (L * ((-2.3025850929940458e2f64 - ABY) * (E + ((-2.3025850929940458e2f64 - ABY) * EG)))))));
                                    ACU
                                } else {
                                    let ACV = ABY - EA;
                                    let ACW = EK * (E + (ACV * (E + (L * (ACV * (E + (ACV * EG)))))));
                                    ACW
                                };
                                ACR = ACX;
                            }
                            let ACS = (U / F) * ((IN / (CW / IN)).ln());
                            let ACT = if U < GR { 1.0 } else { 0.0 };
                            oACT = ACT;
                            let ADC;
                            let ADD;
                            if ACT != 0.0 {
                                let ACY = U - (HK * ACS);
                                let ACZ = (GR - ((HK * (AAO - ACS)) + U)) - HM;
                                let ADA = (HO * GR) * HM;
                                let ADB = if ADA > BP { 1.0 } else { 0.0 };
                                oADB = ADB;
                                let ADH = if ADB != 0.0 {
                                    ADA
                                } else {
                                    let ADG = -ADA;
                                    ADG
                                };
                                let ADI = ((GR - (L * (ACZ + (((ACZ * ACZ) + ADH).sqrt())))) - U) - HM;
                                let ADJ = (HO * U) * HM;
                                let ADK = if ADJ > BP { 1.0 } else { 0.0 };
                                oADK = ADK;
                                let ADM = if ADK != 0.0 {
                                    ADJ
                                } else {
                                    let ADL = -ADJ;
                                    ADL
                                };
                                let ADN = U + (L * (ADI + (((ADI * ADI) + ADM).sqrt())));
                                let ADO = (GR - ACY) - HM;
                                let ADQ = if ADB != 0.0 {
                                    ADA
                                } else {
                                    let ADP = -ADA;
                                    ADP
                                };
                                let ADR = ((GR - (L * (ADO + (((ADO * ADO) + ADQ).sqrt())))) - U) - HM;
                                let ADT = if ADK != 0.0 {
                                    ADJ
                                } else {
                                    let ADS = -ADJ;
                                    ADS
                                };
                                let ADU = U + (L * (ADR + (((ADR * ADR) + ADT).sqrt())));
                                ADC = ADN;
                                ADD = ADU;
                            } else {
                                ADC = U;
                                ADD = U;
                            }
                            let ADE = F * ((AAO / ADC) + ((ACS * (ADC - ADD)) / (ADD * GR)));
                            let ADF = if (ADE.abs()) < EA { 1.0 } else { 0.0 };
                            oADF = ADF;
                            let ADX;
                            if ADF != 0.0 {
                                let ADV = ADE.exp();
                                ADX = ADV;
                            } else {
                                let ADW = if ADE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oADW = ADW;
                                let AED = if ADW != 0.0 {
                                    let AEA = EH / (E + ((-2.3025850929940458e2f64 - ADE) * (E + (L * ((-2.3025850929940458e2f64 - ADE) * (E + ((-2.3025850929940458e2f64 - ADE) * EG)))))));
                                    AEA
                                } else {
                                    let AEB = ADE - EA;
                                    let AEC = EK * (E + (AEB * (E + (L * (AEB * (E + (AEB * EG)))))));
                                    AEC
                                };
                                ADX = AED;
                            }
                            let ADY = (W / F) * ((JU / (CW / JU)).ln());
                            let ADZ = if W < GR { 1.0 } else { 0.0 };
                            oADZ = ADZ;
                            let AEI;
                            let AEJ;
                            if ADZ != 0.0 {
                                let AEE = W - (HK * ADY);
                                let AEF = (GR - ((HK * (AAO - ADY)) + W)) - HM;
                                let AEG = (HO * GR) * HM;
                                let AEH = if AEG > BP { 1.0 } else { 0.0 };
                                oAEH = AEH;
                                let AEN = if AEH != 0.0 {
                                    AEG
                                } else {
                                    let AEM = -AEG;
                                    AEM
                                };
                                let AEO = ((GR - (L * (AEF + (((AEF * AEF) + AEN).sqrt())))) - W) - HM;
                                let AEP = (HO * W) * HM;
                                let AEQ = if AEP > BP { 1.0 } else { 0.0 };
                                oAEQ = AEQ;
                                let AES = if AEQ != 0.0 {
                                    AEP
                                } else {
                                    let AER = -AEP;
                                    AER
                                };
                                let AET = W + (L * (AEO + (((AEO * AEO) + AES).sqrt())));
                                let AEU = (GR - AEE) - HM;
                                let AEW = if AEH != 0.0 {
                                    AEG
                                } else {
                                    let AEV = -AEG;
                                    AEV
                                };
                                let AEX = ((GR - (L * (AEU + (((AEU * AEU) + AEW).sqrt())))) - W) - HM;
                                let AEZ = if AEQ != 0.0 {
                                    AEP
                                } else {
                                    let AEY = -AEP;
                                    AEY
                                };
                                let AFA = W + (L * (AEX + (((AEX * AEX) + AEZ).sqrt())));
                                AEI = AET;
                                AEJ = AFA;
                            } else {
                                AEI = W;
                                AEJ = W;
                            }
                            let AEK = F * ((AAO / AEI) + ((ADY * (AEI - AEJ)) / (AEJ * GR)));
                            let AEL = if (AEK.abs()) < EA { 1.0 } else { 0.0 };
                            oAEL = AEL;
                            let AFD;
                            if AEL != 0.0 {
                                let AFB = AEK.exp();
                                AFD = AFB;
                            } else {
                                let AFC = if AEK < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAFC = AFC;
                                let AFH = if AFC != 0.0 {
                                    let AFE = EH / (E + ((-2.3025850929940458e2f64 - AEK) * (E + (L * ((-2.3025850929940458e2f64 - AEK) * (E + ((-2.3025850929940458e2f64 - AEK) * EG)))))));
                                    AFE
                                } else {
                                    let AFF = AEK - EA;
                                    let AFG = EK * (E + (AFF * (E + (L * (AFF * (E + (AFF * EG)))))));
                                    AFG
                                };
                                AFD = AFH;
                            }
                            ABB = ACR;
                            ABC = ADX;
                            ABD = AFD;
                            ABE = ABM;
                        } else {
                            let AAY = AAO - DY;
                            let AAZ = ((E + (AAY * F)) * EE).sqrt();
                            let ABA = if S < GR { 1.0 } else { 0.0 };
                            oABA = ABA;
                            let AFM;
                            let AFN;
                            let AFO;
                            if ABA != 0.0 {
                                let AFI = S - (HK * DF);
                                let AFJ = (GR - ((HK * (DY - DF)) + S)) - HM;
                                let AFK = (HO * GR) * HM;
                                let AFL = if AFK > BP { 1.0 } else { 0.0 };
                                oAFL = AFL;
                                let AFT = if AFL != 0.0 {
                                    AFK
                                } else {
                                    let AFS = -AFK;
                                    AFS
                                };
                                let AFU = ((AFJ * AFJ) + AFT).sqrt();
                                let AFV = L * (E + (AFJ / AFU));
                                let AFW = ((GR - (L * (AFJ + AFU))) - S) - HM;
                                let AFX = (HO * S) * HM;
                                let AFY = if AFX > BP { 1.0 } else { 0.0 };
                                oAFY = AFY;
                                let AGA = if AFY != 0.0 {
                                    AFX
                                } else {
                                    let AFZ = -AFX;
                                    AFZ
                                };
                                let AGB = ((AFW * AFW) + AGA).sqrt();
                                let AGC = L * (E + (AFW / AGB));
                                let AGD = S + (L * (AFW + AGB));
                                let AGE = (GR - AFI) - HM;
                                let AGG = if AFL != 0.0 {
                                    AFK
                                } else {
                                    let AGF = -AFK;
                                    AGF
                                };
                                let AGH = ((GR - (L * (AGE + (((AGE * AGE) + AGG).sqrt())))) - S) - HM;
                                let AGJ = if AFY != 0.0 {
                                    AFX
                                } else {
                                    let AGI = -AFX;
                                    AGI
                                };
                                let AGK = S + (L * (AGH + (((AGH * AGH) + AGJ).sqrt())));
                                let AGL = (HK * AFV) * AGC;
                                AFM = AGD;
                                AFN = AGK;
                                AFO = AGL;
                            } else {
                                AFM = S;
                                AFN = S;
                                AFO = BP;
                            }
                            let AFP = AFN * GR;
                            let AFQ = F * ((DY / AFM) + ((DF * (AFM - AFN)) / AFP));
                            let AFR = if (AFQ.abs()) < EA { 1.0 } else { 0.0 };
                            oAFR = AFR;
                            let AGO;
                            if AFR != 0.0 {
                                let AGM = AFQ.exp();
                                AGO = AGM;
                            } else {
                                let AGN = if AFQ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAGN = AGN;
                                let AGV = if AGN != 0.0 {
                                    let AGS = EH / (E + ((-2.3025850929940458e2f64 - AFQ) * (E + (L * ((-2.3025850929940458e2f64 - AFQ) * (E + ((-2.3025850929940458e2f64 - AFQ) * EG)))))));
                                    AGS
                                } else {
                                    let AGT = AFQ - EA;
                                    let AGU = EK * (E + (AGT * (E + (L * (AGT * (E + (AGT * EG)))))));
                                    AGU
                                };
                                AGO = AGV;
                            }
                            let AGP = (E + (AAY * (F * (((AFM - (DY * AFO)) / (AFM * AFM)) + ((DF * AFO) / AFP))))) * AGO;
                            let AGQ = (U / F) * ((IN / (CW / IN)).ln());
                            let AGR = if U < GR { 1.0 } else { 0.0 };
                            oAGR = AGR;
                            let AHA;
                            let AHB;
                            let AHC;
                            if AGR != 0.0 {
                                let AGW = U - (HK * AGQ);
                                let AGX = (GR - ((HK * (DY - AGQ)) + U)) - HM;
                                let AGY = (HO * GR) * HM;
                                let AGZ = if AGY > BP { 1.0 } else { 0.0 };
                                oAGZ = AGZ;
                                let AHH = if AGZ != 0.0 {
                                    AGY
                                } else {
                                    let AHG = -AGY;
                                    AHG
                                };
                                let AHI = ((AGX * AGX) + AHH).sqrt();
                                let AHJ = L * (E + (AGX / AHI));
                                let AHK = ((GR - (L * (AGX + AHI))) - U) - HM;
                                let AHL = (HO * U) * HM;
                                let AHM = if AHL > BP { 1.0 } else { 0.0 };
                                oAHM = AHM;
                                let AHO = if AHM != 0.0 {
                                    AHL
                                } else {
                                    let AHN = -AHL;
                                    AHN
                                };
                                let AHP = ((AHK * AHK) + AHO).sqrt();
                                let AHQ = L * (E + (AHK / AHP));
                                let AHR = U + (L * (AHK + AHP));
                                let AHS = (GR - AGW) - HM;
                                let AHU = if AGZ != 0.0 {
                                    AGY
                                } else {
                                    let AHT = -AGY;
                                    AHT
                                };
                                let AHV = ((GR - (L * (AHS + (((AHS * AHS) + AHU).sqrt())))) - U) - HM;
                                let AHX = if AHM != 0.0 {
                                    AHL
                                } else {
                                    let AHW = -AHL;
                                    AHW
                                };
                                let AHY = U + (L * (AHV + (((AHV * AHV) + AHX).sqrt())));
                                let AHZ = (HK * AHJ) * AHQ;
                                AHA = AHR;
                                AHB = AHY;
                                AHC = AHZ;
                            } else {
                                AHA = U;
                                AHB = U;
                                AHC = BP;
                            }
                            let AHD = AHB * GR;
                            let AHE = F * ((DY / AHA) + ((AGQ * (AHA - AHB)) / AHD));
                            let AHF = if (AHE.abs()) < EA { 1.0 } else { 0.0 };
                            oAHF = AHF;
                            let AIC;
                            if AHF != 0.0 {
                                let AIA = AHE.exp();
                                AIC = AIA;
                            } else {
                                let AIB = if AHE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAIB = AIB;
                                let AIJ = if AIB != 0.0 {
                                    let AIG = EH / (E + ((-2.3025850929940458e2f64 - AHE) * (E + (L * ((-2.3025850929940458e2f64 - AHE) * (E + ((-2.3025850929940458e2f64 - AHE) * EG)))))));
                                    AIG
                                } else {
                                    let AIH = AHE - EA;
                                    let AII = EK * (E + (AIH * (E + (L * (AIH * (E + (AIH * EG)))))));
                                    AII
                                };
                                AIC = AIJ;
                            }
                            let AID = (E + (AAY * (F * (((AHA - (DY * AHC)) / (AHA * AHA)) + ((AGQ * AHC) / AHD))))) * AIC;
                            let AIE = (W / F) * ((JU / (CW / JU)).ln());
                            let AIF = if W < GR { 1.0 } else { 0.0 };
                            oAIF = AIF;
                            let AIO;
                            let AIP;
                            let AIQ;
                            if AIF != 0.0 {
                                let AIK = W - (HK * AIE);
                                let AIL = (GR - ((HK * (DY - AIE)) + W)) - HM;
                                let AIM = (HO * GR) * HM;
                                let AIN = if AIM > BP { 1.0 } else { 0.0 };
                                oAIN = AIN;
                                let AIV = if AIN != 0.0 {
                                    AIM
                                } else {
                                    let AIU = -AIM;
                                    AIU
                                };
                                let AIW = ((AIL * AIL) + AIV).sqrt();
                                let AIX = L * (E + (AIL / AIW));
                                let AIY = ((GR - (L * (AIL + AIW))) - W) - HM;
                                let AIZ = (HO * W) * HM;
                                let AJA = if AIZ > BP { 1.0 } else { 0.0 };
                                oAJA = AJA;
                                let AJC = if AJA != 0.0 {
                                    AIZ
                                } else {
                                    let AJB = -AIZ;
                                    AJB
                                };
                                let AJD = ((AIY * AIY) + AJC).sqrt();
                                let AJE = L * (E + (AIY / AJD));
                                let AJF = W + (L * (AIY + AJD));
                                let AJG = (GR - AIK) - HM;
                                let AJI = if AIN != 0.0 {
                                    AIM
                                } else {
                                    let AJH = -AIM;
                                    AJH
                                };
                                let AJJ = ((GR - (L * (AJG + (((AJG * AJG) + AJI).sqrt())))) - W) - HM;
                                let AJL = if AJA != 0.0 {
                                    AIZ
                                } else {
                                    let AJK = -AIZ;
                                    AJK
                                };
                                let AJM = W + (L * (AJJ + (((AJJ * AJJ) + AJL).sqrt())));
                                let AJN = (HK * AIX) * AJE;
                                AIO = AJF;
                                AIP = AJM;
                                AIQ = AJN;
                            } else {
                                AIO = W;
                                AIP = W;
                                AIQ = BP;
                            }
                            let AIR = AIP * GR;
                            let AIS = F * ((DY / AIO) + ((AIE * (AIO - AIP)) / AIR));
                            let AIT = if (AIS.abs()) < EA { 1.0 } else { 0.0 };
                            oAIT = AIT;
                            let AJQ;
                            if AIT != 0.0 {
                                let AJO = AIS.exp();
                                AJQ = AJO;
                            } else {
                                let AJP = if AIS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAJP = AJP;
                                let AJV = if AJP != 0.0 {
                                    let AJS = EH / (E + ((-2.3025850929940458e2f64 - AIS) * (E + (L * ((-2.3025850929940458e2f64 - AIS) * (E + ((-2.3025850929940458e2f64 - AIS) * EG)))))));
                                    AJS
                                } else {
                                    let AJT = AIS - EA;
                                    let AJU = EK * (E + (AJT * (E + (L * (AJT * (E + (AJT * EG)))))));
                                    AJU
                                };
                                AJQ = AJV;
                            }
                            let AJR = (E + (AAY * (F * (((AIO - (DY * AIQ)) / (AIO * AIO)) + ((AIE * AIQ) / AIR))))) * AJQ;
                            ABB = AGP;
                            ABC = AID;
                            ABD = AJR;
                            ABE = AAZ;
                        }
                        let ABF = ABB - E;
                        let ABG = ABC - E;
                        let ABH = ABD - E;
                        let ABI = E / ABE;
                        let AJY = if ABJ != 0.0 {
                            let AJW = AC * (D * (((AC + ABI) + (((ABI + E) * (ABI + PT)).sqrt())).ln()));
                            AJW
                        } else {
                            let AJX = staged[113] + (AC * (D * ((((AC * ABE) + E) + (((E + ABE) * (E + (PT * ABE))).sqrt())).ln())));
                            AJX
                        };
                        let AJZ = EV - AJY;
                        let AKA = AAO - AJZ;
                        let AKB = L * ((AAO + AJZ) - (((AKA * AKA) + ((HO * D) * D)).sqrt()));
                        AAQ = ABF;
                        AAR = AKB;
                        AAS = AJY;
                        AAT = ABE;
                        AAU = ABG;
                        AAV = ABH;
                    } else {
                        AAQ = BP;
                        AAR = BP;
                        AAS = BP;
                        AAT = BP;
                        AAU = BP;
                        AAV = BP;
                    }
                    let AKC;
                    if EF != 0.0 {
                        AKC = BP;
                    } else {
                        let AKD = Y * AAQ;
                        let AKI;
                        let AKJ;
                        let AKK;
                        let AKL;
                        let AKM;
                        if AKE != 0.0 {
                            AKI = BP;
                            AKJ = BP;
                            AKK = BP;
                            AKL = BP;
                            AKM = BP;
                        } else {
                            let AKF = AK - AAR;
                            let AKG = E - ((E - (AAS / AKF)).sqrt());
                            let AKP = if AKH != 0.0 {
                                BP
                            } else {
                                let AKO = ((((AKG * AKG) * (AKG.ln())) / (E - AKG)) + AKG) * staged[114];
                                AKO
                            };
                            let AKQ = AKG + AKP;
                            let AKT = if AKH != 0.0 {
                                let AKR = (AKF * QP).sqrt();
                                AKR
                            } else {
                                let AKS = (AKF * QP).powf(AQ);
                                AKS
                            };
                            let AKU = QT * AKT;
                            let AKV = N * ((AAT - E) * AKU);
                            let AKW = QW * (AKV * AKQ);
                            AKI = AKU;
                            AKJ = AKF;
                            AKK = AKQ;
                            AKL = AKV;
                            AKM = AKW;
                        }
                        let ALF;
                        if AKN != 0.0 {
                            ALF = BP;
                        } else {
                            let AKX = BI * ((AKI * QY) / AKJ);
                            let AKY = (RA * BF) / AKX;
                            let AKZ = AKY * AKY;
                            let ALA = AKZ * AKZ;
                            let ALB = (ALA / (ALA + E)).sqrt();
                            let ALC = (ALB.abs()).sqrt();
                            let ALD = ALB * ALC;
                            let ALJ = if ALE != 0.0 {
                                let ALH = E / (E + (AKX * ALD));
                                ALH
                            } else {
                                let ALI = (E + (AKX * ALD)).powf(staged[115]);
                                ALI
                            };
                            let ALK = (AKK * ALJ) / (AKK + ALJ);
                            let ALL = (RO * (AKX / ALC)).sqrt();
                            let ALM = (((BF * AKY) * ALC) - (BF * ALB)) + (L * (AKX * ALD));
                            let ALN = (((AC * (AKY * ALC)) - ALB) - E) * ALL;
                            let ALO = ALN * ALN;
                            let ALP = if ALN > BP { 1.0 } else { 0.0 };
                            oALP = ALP;
                            let ALS = if ALP != 0.0 {
                                let ALQ = E / (E + (RU * ALN));
                                ALQ
                            } else {
                                let ALR = E / (E - (RU * ALN));
                                ALR
                            };
                            let ALT = (-ALO) + ALM;
                            let ALU = if ALT > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oALU = ALU;
                            let ALX = if ALU != 0.0 {
                                let ALV = ALT.exp();
                                ALV
                            } else {
                                let ALW = EH / (E + ((-2.3025850929940458e2f64 - ALT) * (E + (L * ((-2.3025850929940458e2f64 - ALT) * (E + ((-2.3025850929940458e2f64 - ALT) * EG)))))));
                                ALW
                            };
                            let ALY = ALS * ALS;
                            let ALZ = (((SD * ALS) + (SF * ALY)) + (SG * (ALY * ALS))) * ALX;
                            let AMB;
                            if ALP != 0.0 {
                                AMB = ALZ;
                            } else {
                                let AMA = if ALM > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAMA = AMA;
                                let AMF = if AMA != 0.0 {
                                    let AMD = ALM.exp();
                                    AMD
                                } else {
                                    let AME = EH / (E + ((-2.3025850929940458e2f64 - ALM) * (E + (L * ((-2.3025850929940458e2f64 - ALM) * (E + ((-2.3025850929940458e2f64 - ALM) * EG)))))));
                                    AME
                                };
                                let AMG = (AC * AMF) - ALZ;
                                AMB = AMG;
                            }
                            let AMC = SK * ((AKL * (8.86226925452758e-1f64 * ((BF * AMB) / ALL))) * ALK);
                            ALF = AMC;
                        }
                        let AMH;
                        if ALG != 0.0 {
                            AMH = BP;
                        } else {
                            let AMJ = (-BR) / staged[116];
                            let AMK = if (AMJ.abs()) < EA { 1.0 } else { 0.0 };
                            oAMK = AMK;
                            let AMN;
                            if AMK != 0.0 {
                                let AML = AMJ.exp();
                                AMN = AML;
                            } else {
                                let AMM = if AMJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAMM = AMM;
                                let AMS = if AMM != 0.0 {
                                    let AMP = EH / (E + ((-2.3025850929940458e2f64 - AMJ) * (E + (L * ((-2.3025850929940458e2f64 - AMJ) * (E + ((-2.3025850929940458e2f64 - AMJ) * EG)))))));
                                    AMP
                                } else {
                                    let AMQ = AMJ - EA;
                                    let AMR = EK * (E + (AMQ * (E + (L * (AMQ * (E + (AMQ * EG)))))));
                                    AMR
                                };
                                AMN = AMS;
                            }
                            let AMO = SY * (staged[117] * AMN);
                            AMH = AMO;
                        }
                        let AMI = if (if CE > SR { 1.0 } else { 0.0 }) != 0.0 || staged[118] != 0.0 { 1.0 } else { 0.0 };
                        oAMI = AMI;
                        let AMV;
                        if AMI != 0.0 {
                            AMV = E;
                        } else {
                            let AMU = if AMT > ((-TE) * CE) { 1.0 } else { 0.0 };
                            oAMU = AMU;
                            let AMZ;
                            if AMU != 0.0 {
                                let AMX = if TJ == HO { 1.0 } else { 0.0 };
                                oAMX = AMX;
                                let AND = if AMX != 0.0 {
                                    let ANA = (AMT * CD).abs();
                                    let ANB = ((ANA * ANA) * ANA) * ANA;
                                    ANB
                                } else {
                                    let ANC = ((AMT * CD).abs()).powf(TJ);
                                    ANC
                                };
                                let ANE = E / (E - AND);
                                AMZ = ANE;
                            } else {
                                let AMY = TL + ((AMT + (TE * CE)) * CN);
                                AMZ = AMY;
                            }
                            AMV = AMZ;
                        }
                        let AMW = (((AKD + AKM) + ALF) + AMH) * AMV;
                        AKC = AMW;
                    }
                    let ANF;
                    if EP != 0.0 {
                        ANF = BP;
                    } else {
                        let ANG = Z * AAU;
                        let ANL;
                        let ANM;
                        let ANN;
                        let ANO;
                        let ANP;
                        if ANH != 0.0 {
                            ANL = BP;
                            ANM = BP;
                            ANN = BP;
                            ANO = BP;
                            ANP = BP;
                        } else {
                            let ANI = AL - AAR;
                            let ANJ = E - ((E - (AAS / ANI)).sqrt());
                            let ANS = if ANK != 0.0 {
                                BP
                            } else {
                                let ANR = ((((ANJ * ANJ) * (ANJ.ln())) / (E - ANJ)) + ANJ) * staged[120];
                                ANR
                            };
                            let ANT = ANJ + ANS;
                            let ANW = if ANK != 0.0 {
                                let ANU = (ANI * UI).sqrt();
                                ANU
                            } else {
                                let ANV = (ANI * UI).powf(AS);
                                ANV
                            };
                            let ANX = UM * ANW;
                            let ANY = P * ((AAT - E) * ANX);
                            let ANZ = UP * (ANY * ANT);
                            ANL = ANX;
                            ANM = ANI;
                            ANN = ANT;
                            ANO = ANY;
                            ANP = ANZ;
                        }
                        let AOI;
                        if ANQ != 0.0 {
                            AOI = BP;
                        } else {
                            let AOA = BJ * ((ANL * UR) / ANM);
                            let AOB = (RA * BG) / AOA;
                            let AOC = AOB * AOB;
                            let AOD = AOC * AOC;
                            let AOE = (AOD / (AOD + E)).sqrt();
                            let AOF = (AOE.abs()).sqrt();
                            let AOG = AOE * AOF;
                            let AOM = if AOH != 0.0 {
                                let AOK = E / (E + (AOA * AOG));
                                AOK
                            } else {
                                let AOL = (E + (AOA * AOG)).powf(staged[121]);
                                AOL
                            };
                            let AON = (ANN * AOM) / (ANN + AOM);
                            let AOO = (RO * (AOA / AOF)).sqrt();
                            let AOP = (((BG * AOB) * AOF) - (BG * AOE)) + (L * (AOA * AOG));
                            let AOQ = (((AC * (AOB * AOF)) - AOE) - E) * AOO;
                            let AOR = AOQ * AOQ;
                            let AOS = if AOQ > BP { 1.0 } else { 0.0 };
                            oAOS = AOS;
                            let AOV = if AOS != 0.0 {
                                let AOT = E / (E + (RU * AOQ));
                                AOT
                            } else {
                                let AOU = E / (E - (RU * AOQ));
                                AOU
                            };
                            let AOW = (-AOR) + AOP;
                            let AOX = if AOW > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAOX = AOX;
                            let APA = if AOX != 0.0 {
                                let AOY = AOW.exp();
                                AOY
                            } else {
                                let AOZ = EH / (E + ((-2.3025850929940458e2f64 - AOW) * (E + (L * ((-2.3025850929940458e2f64 - AOW) * (E + ((-2.3025850929940458e2f64 - AOW) * EG)))))));
                                AOZ
                            };
                            let APB = AOV * AOV;
                            let APC = (((SD * AOV) + (SF * APB)) + (SG * (APB * AOV))) * APA;
                            let APE;
                            if AOS != 0.0 {
                                APE = APC;
                            } else {
                                let APD = if AOP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAPD = APD;
                                let API = if APD != 0.0 {
                                    let APG = AOP.exp();
                                    APG
                                } else {
                                    let APH = EH / (E + ((-2.3025850929940458e2f64 - AOP) * (E + (L * ((-2.3025850929940458e2f64 - AOP) * (E + ((-2.3025850929940458e2f64 - AOP) * EG)))))));
                                    APH
                                };
                                let APJ = (AC * API) - APC;
                                APE = APJ;
                            }
                            let APF = VX * ((ANO * (8.86226925452758e-1f64 * ((BG * APE) / AOO))) * AON);
                            AOI = APF;
                        }
                        let APK;
                        if AOJ != 0.0 {
                            APK = BP;
                        } else {
                            let APM = (-BT) / staged[122];
                            let APN = if (APM.abs()) < EA { 1.0 } else { 0.0 };
                            oAPN = APN;
                            let APQ;
                            if APN != 0.0 {
                                let APO = APM.exp();
                                APQ = APO;
                            } else {
                                let APP = if APM < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAPP = APP;
                                let APV = if APP != 0.0 {
                                    let APS = EH / (E + ((-2.3025850929940458e2f64 - APM) * (E + (L * ((-2.3025850929940458e2f64 - APM) * (E + ((-2.3025850929940458e2f64 - APM) * EG)))))));
                                    APS
                                } else {
                                    let APT = APM - EA;
                                    let APU = EK * (E + (APT * (E + (L * (APT * (E + (APT * EG)))))));
                                    APU
                                };
                                APQ = APV;
                            }
                            let APR = WK * (staged[123] * APQ);
                            APK = APR;
                        }
                        let APL = if (if CI > SR { 1.0 } else { 0.0 }) != 0.0 || staged[124] != 0.0 { 1.0 } else { 0.0 };
                        oAPL = APL;
                        let APX;
                        if APL != 0.0 {
                            APX = E;
                        } else {
                            let APW = if AMT > ((-TE) * CI) { 1.0 } else { 0.0 };
                            oAPW = APW;
                            let AQB;
                            if APW != 0.0 {
                                let APZ = if WT == HO { 1.0 } else { 0.0 };
                                oAPZ = APZ;
                                let AQF = if APZ != 0.0 {
                                    let AQC = (AMT * CH).abs();
                                    let AQD = ((AQC * AQC) * AQC) * AQC;
                                    AQD
                                } else {
                                    let AQE = ((AMT * CH).abs()).powf(WT);
                                    AQE
                                };
                                let AQG = E / (E - AQF);
                                AQB = AQG;
                            } else {
                                let AQA = WV + ((AMT + (TE * CI)) * CO);
                                AQB = AQA;
                            }
                            APX = AQB;
                        }
                        let APY = (((ANG + ANP) + AOI) + APK) * APX;
                        ANF = APY;
                    }
                    let AQH;
                    if ES != 0.0 {
                        AQH = BP;
                    } else {
                        let AQJ = AA * AAV;
                        let AQO;
                        let AQP;
                        let AQQ;
                        let AQR;
                        let AQS;
                        if AQK != 0.0 {
                            AQO = BP;
                            AQP = BP;
                            AQQ = BP;
                            AQR = BP;
                            AQS = BP;
                        } else {
                            let AQL = AM - AAR;
                            let AQM = E - ((E - (AAS / AQL)).sqrt());
                            let AQV = if AQN != 0.0 {
                                BP
                            } else {
                                let AQU = ((((AQM * AQM) * (AQM.ln())) / (E - AQM)) + AQM) * staged[125];
                                AQU
                            };
                            let AQW = AQM + AQV;
                            let AQZ = if AQN != 0.0 {
                                let AQX = (AQL * XT).sqrt();
                                AQX
                            } else {
                                let AQY = (AQL * XT).powf(AU);
                                AQY
                            };
                            let ARA = XX * AQZ;
                            let ARB = R * ((AAT - E) * ARA);
                            let ARC = YA * (ARB * AQW);
                            AQO = ARA;
                            AQP = AQL;
                            AQQ = AQW;
                            AQR = ARB;
                            AQS = ARC;
                        }
                        let ARL;
                        if AQT != 0.0 {
                            ARL = BP;
                        } else {
                            let ARD = BK * ((AQO * YC) / AQP);
                            let ARE = (RA * BH) / ARD;
                            let ARF = ARE * ARE;
                            let ARG = ARF * ARF;
                            let ARH = (ARG / (ARG + E)).sqrt();
                            let ARI = (ARH.abs()).sqrt();
                            let ARJ = ARH * ARI;
                            let ARP = if ARK != 0.0 {
                                let ARN = E / (E + (ARD * ARJ));
                                ARN
                            } else {
                                let ARO = (E + (ARD * ARJ)).powf(staged[126]);
                                ARO
                            };
                            let ARQ = (AQQ * ARP) / (AQQ + ARP);
                            let ARR = (RO * (ARD / ARI)).sqrt();
                            let ARS = (((BH * ARE) * ARI) - (BH * ARH)) + (L * (ARD * ARJ));
                            let ART = (((AC * (ARE * ARI)) - ARH) - E) * ARR;
                            let ARU = ART * ART;
                            let ARV = if ART > BP { 1.0 } else { 0.0 };
                            oARV = ARV;
                            let ARY = if ARV != 0.0 {
                                let ARW = E / (E + (RU * ART));
                                ARW
                            } else {
                                let ARX = E / (E - (RU * ART));
                                ARX
                            };
                            let ARZ = (-ARU) + ARS;
                            let ASA = if ARZ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oASA = ASA;
                            let ASD = if ASA != 0.0 {
                                let ASB = ARZ.exp();
                                ASB
                            } else {
                                let ASC = EH / (E + ((-2.3025850929940458e2f64 - ARZ) * (E + (L * ((-2.3025850929940458e2f64 - ARZ) * (E + ((-2.3025850929940458e2f64 - ARZ) * EG)))))));
                                ASC
                            };
                            let ASE = ARY * ARY;
                            let ASF = (((SD * ARY) + (SF * ASE)) + (SG * (ASE * ARY))) * ASD;
                            let ASH;
                            if ARV != 0.0 {
                                ASH = ASF;
                            } else {
                                let ASG = if ARS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oASG = ASG;
                                let ASL = if ASG != 0.0 {
                                    let ASJ = ARS.exp();
                                    ASJ
                                } else {
                                    let ASK = EH / (E + ((-2.3025850929940458e2f64 - ARS) * (E + (L * ((-2.3025850929940458e2f64 - ARS) * (E + ((-2.3025850929940458e2f64 - ARS) * EG)))))));
                                    ASK
                                };
                                let ASM = (AC * ASL) - ASF;
                                ASH = ASM;
                            }
                            let ASI = ZI * ((AQR * (8.86226925452758e-1f64 * ((BH * ASH) / ARR))) * ARQ);
                            ARL = ASI;
                        }
                        let ASN;
                        if ARM != 0.0 {
                            ASN = BP;
                        } else {
                            let ASP = (-BV) / staged[127];
                            let ASQ = if (ASP.abs()) < EA { 1.0 } else { 0.0 };
                            oASQ = ASQ;
                            let AST;
                            if ASQ != 0.0 {
                                let ASR = ASP.exp();
                                AST = ASR;
                            } else {
                                let ASS = if ASP < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oASS = ASS;
                                let ASY = if ASS != 0.0 {
                                    let ASV = EH / (E + ((-2.3025850929940458e2f64 - ASP) * (E + (L * ((-2.3025850929940458e2f64 - ASP) * (E + ((-2.3025850929940458e2f64 - ASP) * EG)))))));
                                    ASV
                                } else {
                                    let ASW = ASP - EA;
                                    let ASX = EK * (E + (ASW * (E + (L * (ASW * (E + (ASW * EG)))))));
                                    ASX
                                };
                                AST = ASY;
                            }
                            let ASU = ZV * (staged[128] * AST);
                            ASN = ASU;
                        }
                        let ASO = if (if CM > SR { 1.0 } else { 0.0 }) != 0.0 || staged[129] != 0.0 { 1.0 } else { 0.0 };
                        oASO = ASO;
                        let ATA;
                        if ASO != 0.0 {
                            ATA = E;
                        } else {
                            let ASZ = if AMT > ((-TE) * CM) { 1.0 } else { 0.0 };
                            oASZ = ASZ;
                            let ATE;
                            if ASZ != 0.0 {
                                let ATC = if AAE == HO { 1.0 } else { 0.0 };
                                oATC = ATC;
                                let ATI = if ATC != 0.0 {
                                    let ATF = (AMT * CL).abs();
                                    let ATG = ((ATF * ATF) * ATF) * ATF;
                                    ATG
                                } else {
                                    let ATH = ((AMT * CL).abs()).powf(AAE);
                                    ATH
                                };
                                let ATJ = E / (E - ATI);
                                ATE = ATJ;
                            } else {
                                let ATD = AAG + ((AMT + (TE * CM)) * CP);
                                ATE = ATD;
                            }
                            ATA = ATE;
                        }
                        let ATB = (((AQJ + AQS) + ARL) + ASN) * ATA;
                        AQH = ATB;
                    }
                    let AQI = ((DH * AKC) + (DO * ANF)) + (DT * AQH);
                    let ATM;
                    let ATN;
                    let ATO;
                    let ATP;
                    let ATQ;
                    let ATR;
                    if FO != 0.0 {
                        let ATL = if ATK < DY { 1.0 } else { 0.0 };
                        oATL = ATL;
                        let ATX;
                        let ATY;
                        let ATZ;
                        let AUA;
                        if ATL != 0.0 {
                            let ATS = L * (ATK * F);
                            let ATT = if (ATS.abs()) < EA { 1.0 } else { 0.0 };
                            oATT = ATT;
                            let AUI;
                            if ATT != 0.0 {
                                let AUG = ATS.exp();
                                AUI = AUG;
                            } else {
                                let AUH = if ATS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAUH = AUH;
                                let AUN = if AUH != 0.0 {
                                    let AUK = EH / (E + ((-2.3025850929940458e2f64 - ATS) * (E + (L * ((-2.3025850929940458e2f64 - ATS) * (E + ((-2.3025850929940458e2f64 - ATS) * EG)))))));
                                    AUK
                                } else {
                                    let AUL = ATS - EA;
                                    let AUM = EK * (E + (AUL * (E + (L * (AUL * (E + (AUL * EG)))))));
                                    AUM
                                };
                                AUI = AUN;
                            }
                            let AUJ = if S < GR { 1.0 } else { 0.0 };
                            oAUJ = AUJ;
                            let AUS;
                            let AUT;
                            if AUJ != 0.0 {
                                let AUO = S - (HK * DF);
                                let AUP = (GR - ((HK * (ATK - DF)) + S)) - HM;
                                let AUQ = (HO * GR) * HM;
                                let AUR = if AUQ > BP { 1.0 } else { 0.0 };
                                oAUR = AUR;
                                let AUX = if AUR != 0.0 {
                                    AUQ
                                } else {
                                    let AUW = -AUQ;
                                    AUW
                                };
                                let AUY = ((GR - (L * (AUP + (((AUP * AUP) + AUX).sqrt())))) - S) - HM;
                                let AUZ = (HO * S) * HM;
                                let AVA = if AUZ > BP { 1.0 } else { 0.0 };
                                oAVA = AVA;
                                let AVC = if AVA != 0.0 {
                                    AUZ
                                } else {
                                    let AVB = -AUZ;
                                    AVB
                                };
                                let AVD = S + (L * (AUY + (((AUY * AUY) + AVC).sqrt())));
                                let AVE = (GR - AUO) - HM;
                                let AVG = if AUR != 0.0 {
                                    AUQ
                                } else {
                                    let AVF = -AUQ;
                                    AVF
                                };
                                let AVH = ((GR - (L * (AVE + (((AVE * AVE) + AVG).sqrt())))) - S) - HM;
                                let AVJ = if AVA != 0.0 {
                                    AUZ
                                } else {
                                    let AVI = -AUZ;
                                    AVI
                                };
                                let AVK = S + (L * (AVH + (((AVH * AVH) + AVJ).sqrt())));
                                AUS = AVD;
                                AUT = AVK;
                            } else {
                                AUS = S;
                                AUT = S;
                            }
                            let AUU = F * ((ATK / AUS) + ((DF * (AUS - AUT)) / (AUT * GR)));
                            let AUV = if (AUU.abs()) < EA { 1.0 } else { 0.0 };
                            oAUV = AUV;
                            let AVN;
                            if AUV != 0.0 {
                                let AVL = AUU.exp();
                                AVN = AVL;
                            } else {
                                let AVM = if AUU < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAVM = AVM;
                                let AVT = if AVM != 0.0 {
                                    let AVQ = EH / (E + ((-2.3025850929940458e2f64 - AUU) * (E + (L * ((-2.3025850929940458e2f64 - AUU) * (E + ((-2.3025850929940458e2f64 - AUU) * EG)))))));
                                    AVQ
                                } else {
                                    let AVR = AUU - EA;
                                    let AVS = EK * (E + (AVR * (E + (L * (AVR * (E + (AVR * EG)))))));
                                    AVS
                                };
                                AVN = AVT;
                            }
                            let AVO = (U / F) * ((IN / (CW / IN)).ln());
                            let AVP = if U < GR { 1.0 } else { 0.0 };
                            oAVP = AVP;
                            let AVY;
                            let AVZ;
                            if AVP != 0.0 {
                                let AVU = U - (HK * AVO);
                                let AVV = (GR - ((HK * (ATK - AVO)) + U)) - HM;
                                let AVW = (HO * GR) * HM;
                                let AVX = if AVW > BP { 1.0 } else { 0.0 };
                                oAVX = AVX;
                                let AWD = if AVX != 0.0 {
                                    AVW
                                } else {
                                    let AWC = -AVW;
                                    AWC
                                };
                                let AWE = ((GR - (L * (AVV + (((AVV * AVV) + AWD).sqrt())))) - U) - HM;
                                let AWF = (HO * U) * HM;
                                let AWG = if AWF > BP { 1.0 } else { 0.0 };
                                oAWG = AWG;
                                let AWI = if AWG != 0.0 {
                                    AWF
                                } else {
                                    let AWH = -AWF;
                                    AWH
                                };
                                let AWJ = U + (L * (AWE + (((AWE * AWE) + AWI).sqrt())));
                                let AWK = (GR - AVU) - HM;
                                let AWM = if AVX != 0.0 {
                                    AVW
                                } else {
                                    let AWL = -AVW;
                                    AWL
                                };
                                let AWN = ((GR - (L * (AWK + (((AWK * AWK) + AWM).sqrt())))) - U) - HM;
                                let AWP = if AWG != 0.0 {
                                    AWF
                                } else {
                                    let AWO = -AWF;
                                    AWO
                                };
                                let AWQ = U + (L * (AWN + (((AWN * AWN) + AWP).sqrt())));
                                AVY = AWJ;
                                AVZ = AWQ;
                            } else {
                                AVY = U;
                                AVZ = U;
                            }
                            let AWA = F * ((ATK / AVY) + ((AVO * (AVY - AVZ)) / (AVZ * GR)));
                            let AWB = if (AWA.abs()) < EA { 1.0 } else { 0.0 };
                            oAWB = AWB;
                            let AWT;
                            if AWB != 0.0 {
                                let AWR = AWA.exp();
                                AWT = AWR;
                            } else {
                                let AWS = if AWA < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAWS = AWS;
                                let AWZ = if AWS != 0.0 {
                                    let AWW = EH / (E + ((-2.3025850929940458e2f64 - AWA) * (E + (L * ((-2.3025850929940458e2f64 - AWA) * (E + ((-2.3025850929940458e2f64 - AWA) * EG)))))));
                                    AWW
                                } else {
                                    let AWX = AWA - EA;
                                    let AWY = EK * (E + (AWX * (E + (L * (AWX * (E + (AWX * EG)))))));
                                    AWY
                                };
                                AWT = AWZ;
                            }
                            let AWU = (W / F) * ((JU / (CW / JU)).ln());
                            let AWV = if W < GR { 1.0 } else { 0.0 };
                            oAWV = AWV;
                            let AXE;
                            let AXF;
                            if AWV != 0.0 {
                                let AXA = W - (HK * AWU);
                                let AXB = (GR - ((HK * (ATK - AWU)) + W)) - HM;
                                let AXC = (HO * GR) * HM;
                                let AXD = if AXC > BP { 1.0 } else { 0.0 };
                                oAXD = AXD;
                                let AXJ = if AXD != 0.0 {
                                    AXC
                                } else {
                                    let AXI = -AXC;
                                    AXI
                                };
                                let AXK = ((GR - (L * (AXB + (((AXB * AXB) + AXJ).sqrt())))) - W) - HM;
                                let AXL = (HO * W) * HM;
                                let AXM = if AXL > BP { 1.0 } else { 0.0 };
                                oAXM = AXM;
                                let AXO = if AXM != 0.0 {
                                    AXL
                                } else {
                                    let AXN = -AXL;
                                    AXN
                                };
                                let AXP = W + (L * (AXK + (((AXK * AXK) + AXO).sqrt())));
                                let AXQ = (GR - AXA) - HM;
                                let AXS = if AXD != 0.0 {
                                    AXC
                                } else {
                                    let AXR = -AXC;
                                    AXR
                                };
                                let AXT = ((GR - (L * (AXQ + (((AXQ * AXQ) + AXS).sqrt())))) - W) - HM;
                                let AXV = if AXM != 0.0 {
                                    AXL
                                } else {
                                    let AXU = -AXL;
                                    AXU
                                };
                                let AXW = W + (L * (AXT + (((AXT * AXT) + AXV).sqrt())));
                                AXE = AXP;
                                AXF = AXW;
                            } else {
                                AXE = W;
                                AXF = W;
                            }
                            let AXG = F * ((ATK / AXE) + ((AWU * (AXE - AXF)) / (AXF * GR)));
                            let AXH = if (AXG.abs()) < EA { 1.0 } else { 0.0 };
                            oAXH = AXH;
                            let AXZ;
                            if AXH != 0.0 {
                                let AXX = AXG.exp();
                                AXZ = AXX;
                            } else {
                                let AXY = if AXG < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAXY = AXY;
                                let AYD = if AXY != 0.0 {
                                    let AYA = EH / (E + ((-2.3025850929940458e2f64 - AXG) * (E + (L * ((-2.3025850929940458e2f64 - AXG) * (E + ((-2.3025850929940458e2f64 - AXG) * EG)))))));
                                    AYA
                                } else {
                                    let AYB = AXG - EA;
                                    let AYC = EK * (E + (AYB * (E + (L * (AYB * (E + (AYB * EG)))))));
                                    AYC
                                };
                                AXZ = AYD;
                            }
                            ATX = AVN;
                            ATY = AWT;
                            ATZ = AXZ;
                            AUA = AUI;
                        } else {
                            let ATU = ATK - DY;
                            let ATV = ((E + (ATU * F)) * EE).sqrt();
                            let ATW = if S < GR { 1.0 } else { 0.0 };
                            oATW = ATW;
                            let AYI;
                            let AYJ;
                            let AYK;
                            if ATW != 0.0 {
                                let AYE = S - (HK * DF);
                                let AYF = (GR - ((HK * (DY - DF)) + S)) - HM;
                                let AYG = (HO * GR) * HM;
                                let AYH = if AYG > BP { 1.0 } else { 0.0 };
                                oAYH = AYH;
                                let AYP = if AYH != 0.0 {
                                    AYG
                                } else {
                                    let AYO = -AYG;
                                    AYO
                                };
                                let AYQ = ((AYF * AYF) + AYP).sqrt();
                                let AYR = L * (E + (AYF / AYQ));
                                let AYS = ((GR - (L * (AYF + AYQ))) - S) - HM;
                                let AYT = (HO * S) * HM;
                                let AYU = if AYT > BP { 1.0 } else { 0.0 };
                                oAYU = AYU;
                                let AYW = if AYU != 0.0 {
                                    AYT
                                } else {
                                    let AYV = -AYT;
                                    AYV
                                };
                                let AYX = ((AYS * AYS) + AYW).sqrt();
                                let AYY = L * (E + (AYS / AYX));
                                let AYZ = S + (L * (AYS + AYX));
                                let AZA = (GR - AYE) - HM;
                                let AZC = if AYH != 0.0 {
                                    AYG
                                } else {
                                    let AZB = -AYG;
                                    AZB
                                };
                                let AZD = ((GR - (L * (AZA + (((AZA * AZA) + AZC).sqrt())))) - S) - HM;
                                let AZF = if AYU != 0.0 {
                                    AYT
                                } else {
                                    let AZE = -AYT;
                                    AZE
                                };
                                let AZG = S + (L * (AZD + (((AZD * AZD) + AZF).sqrt())));
                                let AZH = (HK * AYR) * AYY;
                                AYI = AYZ;
                                AYJ = AZG;
                                AYK = AZH;
                            } else {
                                AYI = S;
                                AYJ = S;
                                AYK = BP;
                            }
                            let AYL = AYJ * GR;
                            let AYM = F * ((DY / AYI) + ((DF * (AYI - AYJ)) / AYL));
                            let AYN = if (AYM.abs()) < EA { 1.0 } else { 0.0 };
                            oAYN = AYN;
                            let AZK;
                            if AYN != 0.0 {
                                let AZI = AYM.exp();
                                AZK = AZI;
                            } else {
                                let AZJ = if AYM < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAZJ = AZJ;
                                let AZR = if AZJ != 0.0 {
                                    let AZO = EH / (E + ((-2.3025850929940458e2f64 - AYM) * (E + (L * ((-2.3025850929940458e2f64 - AYM) * (E + ((-2.3025850929940458e2f64 - AYM) * EG)))))));
                                    AZO
                                } else {
                                    let AZP = AYM - EA;
                                    let AZQ = EK * (E + (AZP * (E + (L * (AZP * (E + (AZP * EG)))))));
                                    AZQ
                                };
                                AZK = AZR;
                            }
                            let AZL = (E + (ATU * (F * (((AYI - (DY * AYK)) / (AYI * AYI)) + ((DF * AYK) / AYL))))) * AZK;
                            let AZM = (U / F) * ((IN / (CW / IN)).ln());
                            let AZN = if U < GR { 1.0 } else { 0.0 };
                            oAZN = AZN;
                            let AZW;
                            let AZX;
                            let AZY;
                            if AZN != 0.0 {
                                let AZS = U - (HK * AZM);
                                let AZT = (GR - ((HK * (DY - AZM)) + U)) - HM;
                                let AZU = (HO * GR) * HM;
                                let AZV = if AZU > BP { 1.0 } else { 0.0 };
                                oAZV = AZV;
                                let BAD = if AZV != 0.0 {
                                    AZU
                                } else {
                                    let BAC = -AZU;
                                    BAC
                                };
                                let BAE = ((AZT * AZT) + BAD).sqrt();
                                let BAF = L * (E + (AZT / BAE));
                                let BAG = ((GR - (L * (AZT + BAE))) - U) - HM;
                                let BAH = (HO * U) * HM;
                                let BAI = if BAH > BP { 1.0 } else { 0.0 };
                                oBAI = BAI;
                                let BAK = if BAI != 0.0 {
                                    BAH
                                } else {
                                    let BAJ = -BAH;
                                    BAJ
                                };
                                let BAL = ((BAG * BAG) + BAK).sqrt();
                                let BAM = L * (E + (BAG / BAL));
                                let BAN = U + (L * (BAG + BAL));
                                let BAO = (GR - AZS) - HM;
                                let BAQ = if AZV != 0.0 {
                                    AZU
                                } else {
                                    let BAP = -AZU;
                                    BAP
                                };
                                let BAR = ((GR - (L * (BAO + (((BAO * BAO) + BAQ).sqrt())))) - U) - HM;
                                let BAT = if BAI != 0.0 {
                                    BAH
                                } else {
                                    let BAS = -BAH;
                                    BAS
                                };
                                let BAU = U + (L * (BAR + (((BAR * BAR) + BAT).sqrt())));
                                let BAV = (HK * BAF) * BAM;
                                AZW = BAN;
                                AZX = BAU;
                                AZY = BAV;
                            } else {
                                AZW = U;
                                AZX = U;
                                AZY = BP;
                            }
                            let AZZ = AZX * GR;
                            let BAA = F * ((DY / AZW) + ((AZM * (AZW - AZX)) / AZZ));
                            let BAB = if (BAA.abs()) < EA { 1.0 } else { 0.0 };
                            oBAB = BAB;
                            let BAY;
                            if BAB != 0.0 {
                                let BAW = BAA.exp();
                                BAY = BAW;
                            } else {
                                let BAX = if BAA < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBAX = BAX;
                                let BBF = if BAX != 0.0 {
                                    let BBC = EH / (E + ((-2.3025850929940458e2f64 - BAA) * (E + (L * ((-2.3025850929940458e2f64 - BAA) * (E + ((-2.3025850929940458e2f64 - BAA) * EG)))))));
                                    BBC
                                } else {
                                    let BBD = BAA - EA;
                                    let BBE = EK * (E + (BBD * (E + (L * (BBD * (E + (BBD * EG)))))));
                                    BBE
                                };
                                BAY = BBF;
                            }
                            let BAZ = (E + (ATU * (F * (((AZW - (DY * AZY)) / (AZW * AZW)) + ((AZM * AZY) / AZZ))))) * BAY;
                            let BBA = (W / F) * ((JU / (CW / JU)).ln());
                            let BBB = if W < GR { 1.0 } else { 0.0 };
                            oBBB = BBB;
                            let BBK;
                            let BBL;
                            let BBM;
                            if BBB != 0.0 {
                                let BBG = W - (HK * BBA);
                                let BBH = (GR - ((HK * (DY - BBA)) + W)) - HM;
                                let BBI = (HO * GR) * HM;
                                let BBJ = if BBI > BP { 1.0 } else { 0.0 };
                                oBBJ = BBJ;
                                let BBR = if BBJ != 0.0 {
                                    BBI
                                } else {
                                    let BBQ = -BBI;
                                    BBQ
                                };
                                let BBS = ((BBH * BBH) + BBR).sqrt();
                                let BBT = L * (E + (BBH / BBS));
                                let BBU = ((GR - (L * (BBH + BBS))) - W) - HM;
                                let BBV = (HO * W) * HM;
                                let BBW = if BBV > BP { 1.0 } else { 0.0 };
                                oBBW = BBW;
                                let BBY = if BBW != 0.0 {
                                    BBV
                                } else {
                                    let BBX = -BBV;
                                    BBX
                                };
                                let BBZ = ((BBU * BBU) + BBY).sqrt();
                                let BCA = L * (E + (BBU / BBZ));
                                let BCB = W + (L * (BBU + BBZ));
                                let BCC = (GR - BBG) - HM;
                                let BCE = if BBJ != 0.0 {
                                    BBI
                                } else {
                                    let BCD = -BBI;
                                    BCD
                                };
                                let BCF = ((GR - (L * (BCC + (((BCC * BCC) + BCE).sqrt())))) - W) - HM;
                                let BCH = if BBW != 0.0 {
                                    BBV
                                } else {
                                    let BCG = -BBV;
                                    BCG
                                };
                                let BCI = W + (L * (BCF + (((BCF * BCF) + BCH).sqrt())));
                                let BCJ = (HK * BBT) * BCA;
                                BBK = BCB;
                                BBL = BCI;
                                BBM = BCJ;
                            } else {
                                BBK = W;
                                BBL = W;
                                BBM = BP;
                            }
                            let BBN = BBL * GR;
                            let BBO = F * ((DY / BBK) + ((BBA * (BBK - BBL)) / BBN));
                            let BBP = if (BBO.abs()) < EA { 1.0 } else { 0.0 };
                            oBBP = BBP;
                            let BCM;
                            if BBP != 0.0 {
                                let BCK = BBO.exp();
                                BCM = BCK;
                            } else {
                                let BCL = if BBO < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBCL = BCL;
                                let BCR = if BCL != 0.0 {
                                    let BCO = EH / (E + ((-2.3025850929940458e2f64 - BBO) * (E + (L * ((-2.3025850929940458e2f64 - BBO) * (E + ((-2.3025850929940458e2f64 - BBO) * EG)))))));
                                    BCO
                                } else {
                                    let BCP = BBO - EA;
                                    let BCQ = EK * (E + (BCP * (E + (L * (BCP * (E + (BCP * EG)))))));
                                    BCQ
                                };
                                BCM = BCR;
                            }
                            let BCN = (E + (ATU * (F * (((BBK - (DY * BBM)) / (BBK * BBK)) + ((BBA * BBM) / BBN))))) * BCM;
                            ATX = AZL;
                            ATY = BAZ;
                            ATZ = BCN;
                            AUA = ATV;
                        }
                        let AUB = ATX - E;
                        let AUC = ATY - E;
                        let AUD = ATZ - E;
                        let AUE = E / AUA;
                        let BCU = if AUF != 0.0 {
                            let BCS = AC * (D * (((AC + AUE) + (((AUE + E) * (AUE + PT)).sqrt())).ln()));
                            BCS
                        } else {
                            let BCT = staged[131] + (AC * (D * ((((AC * AUA) + E) + (((E + AUA) * (E + (PT * AUA))).sqrt())).ln())));
                            BCT
                        };
                        let BCV = EV - BCU;
                        let BCW = ATK - BCV;
                        let BCX = L * ((ATK + BCV) - (((BCW * BCW) + ((HO * D) * D)).sqrt()));
                        ATM = AUB;
                        ATN = BCX;
                        ATO = BCU;
                        ATP = AUA;
                        ATQ = AUC;
                        ATR = AUD;
                    } else {
                        ATM = BP;
                        ATN = BP;
                        ATO = BP;
                        ATP = BP;
                        ATQ = BP;
                        ATR = BP;
                    }
                    let BCY;
                    if EF != 0.0 {
                        BCY = BP;
                    } else {
                        let BCZ = Y * ATM;
                        let BDE;
                        let BDF;
                        let BDG;
                        let BDH;
                        let BDI;
                        if BDA != 0.0 {
                            BDE = BP;
                            BDF = BP;
                            BDG = BP;
                            BDH = BP;
                            BDI = BP;
                        } else {
                            let BDB = AK - ATN;
                            let BDC = E - ((E - (ATO / BDB)).sqrt());
                            let BDL = if BDD != 0.0 {
                                BP
                            } else {
                                let BDK = ((((BDC * BDC) * (BDC.ln())) / (E - BDC)) + BDC) * staged[132];
                                BDK
                            };
                            let BDM = BDC + BDL;
                            let BDP = if BDD != 0.0 {
                                let BDN = (BDB * QP).sqrt();
                                BDN
                            } else {
                                let BDO = (BDB * QP).powf(AQ);
                                BDO
                            };
                            let BDQ = QT * BDP;
                            let BDR = N * ((ATP - E) * BDQ);
                            let BDS = QW * (BDR * BDM);
                            BDE = BDQ;
                            BDF = BDB;
                            BDG = BDM;
                            BDH = BDR;
                            BDI = BDS;
                        }
                        let BEB;
                        if BDJ != 0.0 {
                            BEB = BP;
                        } else {
                            let BDT = BI * ((BDE * QY) / BDF);
                            let BDU = (RA * BF) / BDT;
                            let BDV = BDU * BDU;
                            let BDW = BDV * BDV;
                            let BDX = (BDW / (BDW + E)).sqrt();
                            let BDY = (BDX.abs()).sqrt();
                            let BDZ = BDX * BDY;
                            let BEF = if BEA != 0.0 {
                                let BED = E / (E + (BDT * BDZ));
                                BED
                            } else {
                                let BEE = (E + (BDT * BDZ)).powf(staged[133]);
                                BEE
                            };
                            let BEG = (BDG * BEF) / (BDG + BEF);
                            let BEH = (RO * (BDT / BDY)).sqrt();
                            let BEI = (((BF * BDU) * BDY) - (BF * BDX)) + (L * (BDT * BDZ));
                            let BEJ = (((AC * (BDU * BDY)) - BDX) - E) * BEH;
                            let BEK = BEJ * BEJ;
                            let BEL = if BEJ > BP { 1.0 } else { 0.0 };
                            oBEL = BEL;
                            let BEO = if BEL != 0.0 {
                                let BEM = E / (E + (RU * BEJ));
                                BEM
                            } else {
                                let BEN = E / (E - (RU * BEJ));
                                BEN
                            };
                            let BEP = (-BEK) + BEI;
                            let BEQ = if BEP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oBEQ = BEQ;
                            let BET = if BEQ != 0.0 {
                                let BER = BEP.exp();
                                BER
                            } else {
                                let BES = EH / (E + ((-2.3025850929940458e2f64 - BEP) * (E + (L * ((-2.3025850929940458e2f64 - BEP) * (E + ((-2.3025850929940458e2f64 - BEP) * EG)))))));
                                BES
                            };
                            let BEU = BEO * BEO;
                            let BEV = (((SD * BEO) + (SF * BEU)) + (SG * (BEU * BEO))) * BET;
                            let BEX;
                            if BEL != 0.0 {
                                BEX = BEV;
                            } else {
                                let BEW = if BEI > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBEW = BEW;
                                let BFB = if BEW != 0.0 {
                                    let BEZ = BEI.exp();
                                    BEZ
                                } else {
                                    let BFA = EH / (E + ((-2.3025850929940458e2f64 - BEI) * (E + (L * ((-2.3025850929940458e2f64 - BEI) * (E + ((-2.3025850929940458e2f64 - BEI) * EG)))))));
                                    BFA
                                };
                                let BFC = (AC * BFB) - BEV;
                                BEX = BFC;
                            }
                            let BEY = SK * ((BDH * (8.86226925452758e-1f64 * ((BF * BEX) / BEH))) * BEG);
                            BEB = BEY;
                        }
                        let BFD;
                        if BEC != 0.0 {
                            BFD = BP;
                        } else {
                            let BFF = (-BR) / staged[134];
                            let BFG = if (BFF.abs()) < EA { 1.0 } else { 0.0 };
                            oBFG = BFG;
                            let BFJ;
                            if BFG != 0.0 {
                                let BFH = BFF.exp();
                                BFJ = BFH;
                            } else {
                                let BFI = if BFF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBFI = BFI;
                                let BFO = if BFI != 0.0 {
                                    let BFL = EH / (E + ((-2.3025850929940458e2f64 - BFF) * (E + (L * ((-2.3025850929940458e2f64 - BFF) * (E + ((-2.3025850929940458e2f64 - BFF) * EG)))))));
                                    BFL
                                } else {
                                    let BFM = BFF - EA;
                                    let BFN = EK * (E + (BFM * (E + (L * (BFM * (E + (BFM * EG)))))));
                                    BFN
                                };
                                BFJ = BFO;
                            }
                            let BFK = SY * (staged[135] * BFJ);
                            BFD = BFK;
                        }
                        let BFE = if (if CE > SR { 1.0 } else { 0.0 }) != 0.0 || staged[136] != 0.0 { 1.0 } else { 0.0 };
                        oBFE = BFE;
                        let BFR;
                        if BFE != 0.0 {
                            BFR = E;
                        } else {
                            let BFQ = if BFP > ((-TE) * CE) { 1.0 } else { 0.0 };
                            oBFQ = BFQ;
                            let BFV;
                            if BFQ != 0.0 {
                                let BFT = if TJ == HO { 1.0 } else { 0.0 };
                                oBFT = BFT;
                                let BFZ = if BFT != 0.0 {
                                    let BFW = (BFP * CD).abs();
                                    let BFX = ((BFW * BFW) * BFW) * BFW;
                                    BFX
                                } else {
                                    let BFY = ((BFP * CD).abs()).powf(TJ);
                                    BFY
                                };
                                let BGA = E / (E - BFZ);
                                BFV = BGA;
                            } else {
                                let BFU = TL + ((BFP + (TE * CE)) * CN);
                                BFV = BFU;
                            }
                            BFR = BFV;
                        }
                        let BFS = (((BCZ + BDI) + BEB) + BFD) * BFR;
                        BCY = BFS;
                    }
                    let BGB;
                    if EP != 0.0 {
                        BGB = BP;
                    } else {
                        let BGC = Z * ATQ;
                        let BGH;
                        let BGI;
                        let BGJ;
                        let BGK;
                        let BGL;
                        if BGD != 0.0 {
                            BGH = BP;
                            BGI = BP;
                            BGJ = BP;
                            BGK = BP;
                            BGL = BP;
                        } else {
                            let BGE = AL - ATN;
                            let BGF = E - ((E - (ATO / BGE)).sqrt());
                            let BGO = if BGG != 0.0 {
                                BP
                            } else {
                                let BGN = ((((BGF * BGF) * (BGF.ln())) / (E - BGF)) + BGF) * staged[138];
                                BGN
                            };
                            let BGP = BGF + BGO;
                            let BGS = if BGG != 0.0 {
                                let BGQ = (BGE * UI).sqrt();
                                BGQ
                            } else {
                                let BGR = (BGE * UI).powf(AS);
                                BGR
                            };
                            let BGT = UM * BGS;
                            let BGU = P * ((ATP - E) * BGT);
                            let BGV = UP * (BGU * BGP);
                            BGH = BGT;
                            BGI = BGE;
                            BGJ = BGP;
                            BGK = BGU;
                            BGL = BGV;
                        }
                        let BHE;
                        if BGM != 0.0 {
                            BHE = BP;
                        } else {
                            let BGW = BJ * ((BGH * UR) / BGI);
                            let BGX = (RA * BG) / BGW;
                            let BGY = BGX * BGX;
                            let BGZ = BGY * BGY;
                            let BHA = (BGZ / (BGZ + E)).sqrt();
                            let BHB = (BHA.abs()).sqrt();
                            let BHC = BHA * BHB;
                            let BHI = if BHD != 0.0 {
                                let BHG = E / (E + (BGW * BHC));
                                BHG
                            } else {
                                let BHH = (E + (BGW * BHC)).powf(staged[139]);
                                BHH
                            };
                            let BHJ = (BGJ * BHI) / (BGJ + BHI);
                            let BHK = (RO * (BGW / BHB)).sqrt();
                            let BHL = (((BG * BGX) * BHB) - (BG * BHA)) + (L * (BGW * BHC));
                            let BHM = (((AC * (BGX * BHB)) - BHA) - E) * BHK;
                            let BHN = BHM * BHM;
                            let BHO = if BHM > BP { 1.0 } else { 0.0 };
                            oBHO = BHO;
                            let BHR = if BHO != 0.0 {
                                let BHP = E / (E + (RU * BHM));
                                BHP
                            } else {
                                let BHQ = E / (E - (RU * BHM));
                                BHQ
                            };
                            let BHS = (-BHN) + BHL;
                            let BHT = if BHS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oBHT = BHT;
                            let BHW = if BHT != 0.0 {
                                let BHU = BHS.exp();
                                BHU
                            } else {
                                let BHV = EH / (E + ((-2.3025850929940458e2f64 - BHS) * (E + (L * ((-2.3025850929940458e2f64 - BHS) * (E + ((-2.3025850929940458e2f64 - BHS) * EG)))))));
                                BHV
                            };
                            let BHX = BHR * BHR;
                            let BHY = (((SD * BHR) + (SF * BHX)) + (SG * (BHX * BHR))) * BHW;
                            let BIA;
                            if BHO != 0.0 {
                                BIA = BHY;
                            } else {
                                let BHZ = if BHL > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBHZ = BHZ;
                                let BIE = if BHZ != 0.0 {
                                    let BIC = BHL.exp();
                                    BIC
                                } else {
                                    let BID = EH / (E + ((-2.3025850929940458e2f64 - BHL) * (E + (L * ((-2.3025850929940458e2f64 - BHL) * (E + ((-2.3025850929940458e2f64 - BHL) * EG)))))));
                                    BID
                                };
                                let BIF = (AC * BIE) - BHY;
                                BIA = BIF;
                            }
                            let BIB = VX * ((BGK * (8.86226925452758e-1f64 * ((BG * BIA) / BHK))) * BHJ);
                            BHE = BIB;
                        }
                        let BIG;
                        if BHF != 0.0 {
                            BIG = BP;
                        } else {
                            let BII = (-BT) / staged[140];
                            let BIJ = if (BII.abs()) < EA { 1.0 } else { 0.0 };
                            oBIJ = BIJ;
                            let BIM;
                            if BIJ != 0.0 {
                                let BIK = BII.exp();
                                BIM = BIK;
                            } else {
                                let BIL = if BII < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBIL = BIL;
                                let BIR = if BIL != 0.0 {
                                    let BIO = EH / (E + ((-2.3025850929940458e2f64 - BII) * (E + (L * ((-2.3025850929940458e2f64 - BII) * (E + ((-2.3025850929940458e2f64 - BII) * EG)))))));
                                    BIO
                                } else {
                                    let BIP = BII - EA;
                                    let BIQ = EK * (E + (BIP * (E + (L * (BIP * (E + (BIP * EG)))))));
                                    BIQ
                                };
                                BIM = BIR;
                            }
                            let BIN = WK * (staged[141] * BIM);
                            BIG = BIN;
                        }
                        let BIH = if (if CI > SR { 1.0 } else { 0.0 }) != 0.0 || staged[142] != 0.0 { 1.0 } else { 0.0 };
                        oBIH = BIH;
                        let BIT;
                        if BIH != 0.0 {
                            BIT = E;
                        } else {
                            let BIS = if BFP > ((-TE) * CI) { 1.0 } else { 0.0 };
                            oBIS = BIS;
                            let BIX;
                            if BIS != 0.0 {
                                let BIV = if WT == HO { 1.0 } else { 0.0 };
                                oBIV = BIV;
                                let BJB = if BIV != 0.0 {
                                    let BIY = (BFP * CH).abs();
                                    let BIZ = ((BIY * BIY) * BIY) * BIY;
                                    BIZ
                                } else {
                                    let BJA = ((BFP * CH).abs()).powf(WT);
                                    BJA
                                };
                                let BJC = E / (E - BJB);
                                BIX = BJC;
                            } else {
                                let BIW = WV + ((BFP + (TE * CI)) * CO);
                                BIX = BIW;
                            }
                            BIT = BIX;
                        }
                        let BIU = (((BGC + BGL) + BHE) + BIG) * BIT;
                        BGB = BIU;
                    }
                    let BJD;
                    if ES != 0.0 {
                        BJD = BP;
                    } else {
                        let BJF = AA * ATR;
                        let BJK;
                        let BJL;
                        let BJM;
                        let BJN;
                        let BJO;
                        if BJG != 0.0 {
                            BJK = BP;
                            BJL = BP;
                            BJM = BP;
                            BJN = BP;
                            BJO = BP;
                        } else {
                            let BJH = AM - ATN;
                            let BJI = E - ((E - (ATO / BJH)).sqrt());
                            let BJR = if BJJ != 0.0 {
                                BP
                            } else {
                                let BJQ = ((((BJI * BJI) * (BJI.ln())) / (E - BJI)) + BJI) * staged[143];
                                BJQ
                            };
                            let BJS = BJI + BJR;
                            let BJV = if BJJ != 0.0 {
                                let BJT = (BJH * XT).sqrt();
                                BJT
                            } else {
                                let BJU = (BJH * XT).powf(AU);
                                BJU
                            };
                            let BJW = XX * BJV;
                            let BJX = R * ((ATP - E) * BJW);
                            let BJY = YA * (BJX * BJS);
                            BJK = BJW;
                            BJL = BJH;
                            BJM = BJS;
                            BJN = BJX;
                            BJO = BJY;
                        }
                        let BKH;
                        if BJP != 0.0 {
                            BKH = BP;
                        } else {
                            let BJZ = BK * ((BJK * YC) / BJL);
                            let BKA = (RA * BH) / BJZ;
                            let BKB = BKA * BKA;
                            let BKC = BKB * BKB;
                            let BKD = (BKC / (BKC + E)).sqrt();
                            let BKE = (BKD.abs()).sqrt();
                            let BKF = BKD * BKE;
                            let BKL = if BKG != 0.0 {
                                let BKJ = E / (E + (BJZ * BKF));
                                BKJ
                            } else {
                                let BKK = (E + (BJZ * BKF)).powf(staged[144]);
                                BKK
                            };
                            let BKM = (BJM * BKL) / (BJM + BKL);
                            let BKN = (RO * (BJZ / BKE)).sqrt();
                            let BKO = (((BH * BKA) * BKE) - (BH * BKD)) + (L * (BJZ * BKF));
                            let BKP = (((AC * (BKA * BKE)) - BKD) - E) * BKN;
                            let BKQ = BKP * BKP;
                            let BKR = if BKP > BP { 1.0 } else { 0.0 };
                            oBKR = BKR;
                            let BKU = if BKR != 0.0 {
                                let BKS = E / (E + (RU * BKP));
                                BKS
                            } else {
                                let BKT = E / (E - (RU * BKP));
                                BKT
                            };
                            let BKV = (-BKQ) + BKO;
                            let BKW = if BKV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oBKW = BKW;
                            let BKZ = if BKW != 0.0 {
                                let BKX = BKV.exp();
                                BKX
                            } else {
                                let BKY = EH / (E + ((-2.3025850929940458e2f64 - BKV) * (E + (L * ((-2.3025850929940458e2f64 - BKV) * (E + ((-2.3025850929940458e2f64 - BKV) * EG)))))));
                                BKY
                            };
                            let BLA = BKU * BKU;
                            let BLB = (((SD * BKU) + (SF * BLA)) + (SG * (BLA * BKU))) * BKZ;
                            let BLD;
                            if BKR != 0.0 {
                                BLD = BLB;
                            } else {
                                let BLC = if BKO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBLC = BLC;
                                let BLH = if BLC != 0.0 {
                                    let BLF = BKO.exp();
                                    BLF
                                } else {
                                    let BLG = EH / (E + ((-2.3025850929940458e2f64 - BKO) * (E + (L * ((-2.3025850929940458e2f64 - BKO) * (E + ((-2.3025850929940458e2f64 - BKO) * EG)))))));
                                    BLG
                                };
                                let BLI = (AC * BLH) - BLB;
                                BLD = BLI;
                            }
                            let BLE = ZI * ((BJN * (8.86226925452758e-1f64 * ((BH * BLD) / BKN))) * BKM);
                            BKH = BLE;
                        }
                        let BLJ;
                        if BKI != 0.0 {
                            BLJ = BP;
                        } else {
                            let BLL = (-BV) / staged[145];
                            let BLM = if (BLL.abs()) < EA { 1.0 } else { 0.0 };
                            oBLM = BLM;
                            let BLP;
                            if BLM != 0.0 {
                                let BLN = BLL.exp();
                                BLP = BLN;
                            } else {
                                let BLO = if BLL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBLO = BLO;
                                let BLU = if BLO != 0.0 {
                                    let BLR = EH / (E + ((-2.3025850929940458e2f64 - BLL) * (E + (L * ((-2.3025850929940458e2f64 - BLL) * (E + ((-2.3025850929940458e2f64 - BLL) * EG)))))));
                                    BLR
                                } else {
                                    let BLS = BLL - EA;
                                    let BLT = EK * (E + (BLS * (E + (L * (BLS * (E + (BLS * EG)))))));
                                    BLT
                                };
                                BLP = BLU;
                            }
                            let BLQ = ZV * (staged[146] * BLP);
                            BLJ = BLQ;
                        }
                        let BLK = if (if CM > SR { 1.0 } else { 0.0 }) != 0.0 || staged[147] != 0.0 { 1.0 } else { 0.0 };
                        oBLK = BLK;
                        let BLW;
                        if BLK != 0.0 {
                            BLW = E;
                        } else {
                            let BLV = if BFP > ((-TE) * CM) { 1.0 } else { 0.0 };
                            oBLV = BLV;
                            let BMA;
                            if BLV != 0.0 {
                                let BLY = if AAE == HO { 1.0 } else { 0.0 };
                                oBLY = BLY;
                                let BME = if BLY != 0.0 {
                                    let BMB = (BFP * CL).abs();
                                    let BMC = ((BMB * BMB) * BMB) * BMB;
                                    BMC
                                } else {
                                    let BMD = ((BFP * CL).abs()).powf(AAE);
                                    BMD
                                };
                                let BMF = E / (E - BME);
                                BMA = BMF;
                            } else {
                                let BLZ = AAG + ((BFP + (TE * CM)) * CP);
                                BMA = BLZ;
                            }
                            BLW = BMA;
                        }
                        let BLX = (((BJF + BJO) + BKH) + BLJ) * BLW;
                        BJD = BLX;
                    }
                    let BJE = ((DH * BCY) + (DO * BGB)) + (DT * BJD);
                    let BMH;
                    let BMI;
                    let BMJ;
                    let BMK;
                    let BML;
                    let BMM;
                    if FO != 0.0 {
                        let BMG = if BZ < DY { 1.0 } else { 0.0 };
                        oBMG = BMG;
                        let BMS;
                        let BMT;
                        let BMU;
                        let BMV;
                        if BMG != 0.0 {
                            let BMN = L * (BZ * F);
                            let BMO = if (BMN.abs()) < EA { 1.0 } else { 0.0 };
                            oBMO = BMO;
                            let BND;
                            if BMO != 0.0 {
                                let BNB = BMN.exp();
                                BND = BNB;
                            } else {
                                let BNC = if BMN < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBNC = BNC;
                                let BNI = if BNC != 0.0 {
                                    let BNF = EH / (E + ((-2.3025850929940458e2f64 - BMN) * (E + (L * ((-2.3025850929940458e2f64 - BMN) * (E + ((-2.3025850929940458e2f64 - BMN) * EG)))))));
                                    BNF
                                } else {
                                    let BNG = BMN - EA;
                                    let BNH = EK * (E + (BNG * (E + (L * (BNG * (E + (BNG * EG)))))));
                                    BNH
                                };
                                BND = BNI;
                            }
                            let BNE = if S < GR { 1.0 } else { 0.0 };
                            oBNE = BNE;
                            let BNN;
                            let BNO;
                            if BNE != 0.0 {
                                let BNJ = S - (HK * DF);
                                let BNK = (GR - ((HK * (BZ - DF)) + S)) - HM;
                                let BNL = (HO * GR) * HM;
                                let BNM = if BNL > BP { 1.0 } else { 0.0 };
                                oBNM = BNM;
                                let BNS = if BNM != 0.0 {
                                    BNL
                                } else {
                                    let BNR = -BNL;
                                    BNR
                                };
                                let BNT = ((GR - (L * (BNK + (((BNK * BNK) + BNS).sqrt())))) - S) - HM;
                                let BNU = (HO * S) * HM;
                                let BNV = if BNU > BP { 1.0 } else { 0.0 };
                                oBNV = BNV;
                                let BNX = if BNV != 0.0 {
                                    BNU
                                } else {
                                    let BNW = -BNU;
                                    BNW
                                };
                                let BNY = S + (L * (BNT + (((BNT * BNT) + BNX).sqrt())));
                                let BNZ = (GR - BNJ) - HM;
                                let BOB = if BNM != 0.0 {
                                    BNL
                                } else {
                                    let BOA = -BNL;
                                    BOA
                                };
                                let BOC = ((GR - (L * (BNZ + (((BNZ * BNZ) + BOB).sqrt())))) - S) - HM;
                                let BOE = if BNV != 0.0 {
                                    BNU
                                } else {
                                    let BOD = -BNU;
                                    BOD
                                };
                                let BOF = S + (L * (BOC + (((BOC * BOC) + BOE).sqrt())));
                                BNN = BNY;
                                BNO = BOF;
                            } else {
                                BNN = S;
                                BNO = S;
                            }
                            let BNP = F * ((BZ / BNN) + ((DF * (BNN - BNO)) / (BNO * GR)));
                            let BNQ = if (BNP.abs()) < EA { 1.0 } else { 0.0 };
                            oBNQ = BNQ;
                            let BOI;
                            if BNQ != 0.0 {
                                let BOG = BNP.exp();
                                BOI = BOG;
                            } else {
                                let BOH = if BNP < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBOH = BOH;
                                let BOO = if BOH != 0.0 {
                                    let BOL = EH / (E + ((-2.3025850929940458e2f64 - BNP) * (E + (L * ((-2.3025850929940458e2f64 - BNP) * (E + ((-2.3025850929940458e2f64 - BNP) * EG)))))));
                                    BOL
                                } else {
                                    let BOM = BNP - EA;
                                    let BON = EK * (E + (BOM * (E + (L * (BOM * (E + (BOM * EG)))))));
                                    BON
                                };
                                BOI = BOO;
                            }
                            let BOJ = (U / F) * ((IN / (CW / IN)).ln());
                            let BOK = if U < GR { 1.0 } else { 0.0 };
                            oBOK = BOK;
                            let BOT;
                            let BOU;
                            if BOK != 0.0 {
                                let BOP = U - (HK * BOJ);
                                let BOQ = (GR - ((HK * (BZ - BOJ)) + U)) - HM;
                                let BOR = (HO * GR) * HM;
                                let BOS = if BOR > BP { 1.0 } else { 0.0 };
                                oBOS = BOS;
                                let BOY = if BOS != 0.0 {
                                    BOR
                                } else {
                                    let BOX = -BOR;
                                    BOX
                                };
                                let BOZ = ((GR - (L * (BOQ + (((BOQ * BOQ) + BOY).sqrt())))) - U) - HM;
                                let BPA = (HO * U) * HM;
                                let BPB = if BPA > BP { 1.0 } else { 0.0 };
                                oBPB = BPB;
                                let BPD = if BPB != 0.0 {
                                    BPA
                                } else {
                                    let BPC = -BPA;
                                    BPC
                                };
                                let BPE = U + (L * (BOZ + (((BOZ * BOZ) + BPD).sqrt())));
                                let BPF = (GR - BOP) - HM;
                                let BPH = if BOS != 0.0 {
                                    BOR
                                } else {
                                    let BPG = -BOR;
                                    BPG
                                };
                                let BPI = ((GR - (L * (BPF + (((BPF * BPF) + BPH).sqrt())))) - U) - HM;
                                let BPK = if BPB != 0.0 {
                                    BPA
                                } else {
                                    let BPJ = -BPA;
                                    BPJ
                                };
                                let BPL = U + (L * (BPI + (((BPI * BPI) + BPK).sqrt())));
                                BOT = BPE;
                                BOU = BPL;
                            } else {
                                BOT = U;
                                BOU = U;
                            }
                            let BOV = F * ((BZ / BOT) + ((BOJ * (BOT - BOU)) / (BOU * GR)));
                            let BOW = if (BOV.abs()) < EA { 1.0 } else { 0.0 };
                            oBOW = BOW;
                            let BPO;
                            if BOW != 0.0 {
                                let BPM = BOV.exp();
                                BPO = BPM;
                            } else {
                                let BPN = if BOV < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBPN = BPN;
                                let BPU = if BPN != 0.0 {
                                    let BPR = EH / (E + ((-2.3025850929940458e2f64 - BOV) * (E + (L * ((-2.3025850929940458e2f64 - BOV) * (E + ((-2.3025850929940458e2f64 - BOV) * EG)))))));
                                    BPR
                                } else {
                                    let BPS = BOV - EA;
                                    let BPT = EK * (E + (BPS * (E + (L * (BPS * (E + (BPS * EG)))))));
                                    BPT
                                };
                                BPO = BPU;
                            }
                            let BPP = (W / F) * ((JU / (CW / JU)).ln());
                            let BPQ = if W < GR { 1.0 } else { 0.0 };
                            oBPQ = BPQ;
                            let BPZ;
                            let BQA;
                            if BPQ != 0.0 {
                                let BPV = W - (HK * BPP);
                                let BPW = (GR - ((HK * (BZ - BPP)) + W)) - HM;
                                let BPX = (HO * GR) * HM;
                                let BPY = if BPX > BP { 1.0 } else { 0.0 };
                                oBPY = BPY;
                                let BQE = if BPY != 0.0 {
                                    BPX
                                } else {
                                    let BQD = -BPX;
                                    BQD
                                };
                                let BQF = ((GR - (L * (BPW + (((BPW * BPW) + BQE).sqrt())))) - W) - HM;
                                let BQG = (HO * W) * HM;
                                let BQH = if BQG > BP { 1.0 } else { 0.0 };
                                oBQH = BQH;
                                let BQJ = if BQH != 0.0 {
                                    BQG
                                } else {
                                    let BQI = -BQG;
                                    BQI
                                };
                                let BQK = W + (L * (BQF + (((BQF * BQF) + BQJ).sqrt())));
                                let BQL = (GR - BPV) - HM;
                                let BQN = if BPY != 0.0 {
                                    BPX
                                } else {
                                    let BQM = -BPX;
                                    BQM
                                };
                                let BQO = ((GR - (L * (BQL + (((BQL * BQL) + BQN).sqrt())))) - W) - HM;
                                let BQQ = if BQH != 0.0 {
                                    BQG
                                } else {
                                    let BQP = -BQG;
                                    BQP
                                };
                                let BQR = W + (L * (BQO + (((BQO * BQO) + BQQ).sqrt())));
                                BPZ = BQK;
                                BQA = BQR;
                            } else {
                                BPZ = W;
                                BQA = W;
                            }
                            let BQB = F * ((BZ / BPZ) + ((BPP * (BPZ - BQA)) / (BQA * GR)));
                            let BQC = if (BQB.abs()) < EA { 1.0 } else { 0.0 };
                            oBQC = BQC;
                            let BQU;
                            if BQC != 0.0 {
                                let BQS = BQB.exp();
                                BQU = BQS;
                            } else {
                                let BQT = if BQB < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBQT = BQT;
                                let BQY = if BQT != 0.0 {
                                    let BQV = EH / (E + ((-2.3025850929940458e2f64 - BQB) * (E + (L * ((-2.3025850929940458e2f64 - BQB) * (E + ((-2.3025850929940458e2f64 - BQB) * EG)))))));
                                    BQV
                                } else {
                                    let BQW = BQB - EA;
                                    let BQX = EK * (E + (BQW * (E + (L * (BQW * (E + (BQW * EG)))))));
                                    BQX
                                };
                                BQU = BQY;
                            }
                            BMS = BOI;
                            BMT = BPO;
                            BMU = BQU;
                            BMV = BND;
                        } else {
                            let BMP = BZ - DY;
                            let BMQ = ((E + (BMP * F)) * EE).sqrt();
                            let BMR = if S < GR { 1.0 } else { 0.0 };
                            oBMR = BMR;
                            let BRD;
                            let BRE;
                            let BRF;
                            if BMR != 0.0 {
                                let BQZ = S - (HK * DF);
                                let BRA = (GR - ((HK * (DY - DF)) + S)) - HM;
                                let BRB = (HO * GR) * HM;
                                let BRC = if BRB > BP { 1.0 } else { 0.0 };
                                oBRC = BRC;
                                let BRK = if BRC != 0.0 {
                                    BRB
                                } else {
                                    let BRJ = -BRB;
                                    BRJ
                                };
                                let BRL = ((BRA * BRA) + BRK).sqrt();
                                let BRM = L * (E + (BRA / BRL));
                                let BRN = ((GR - (L * (BRA + BRL))) - S) - HM;
                                let BRO = (HO * S) * HM;
                                let BRP = if BRO > BP { 1.0 } else { 0.0 };
                                oBRP = BRP;
                                let BRR = if BRP != 0.0 {
                                    BRO
                                } else {
                                    let BRQ = -BRO;
                                    BRQ
                                };
                                let BRS = ((BRN * BRN) + BRR).sqrt();
                                let BRT = L * (E + (BRN / BRS));
                                let BRU = S + (L * (BRN + BRS));
                                let BRV = (GR - BQZ) - HM;
                                let BRX = if BRC != 0.0 {
                                    BRB
                                } else {
                                    let BRW = -BRB;
                                    BRW
                                };
                                let BRY = ((GR - (L * (BRV + (((BRV * BRV) + BRX).sqrt())))) - S) - HM;
                                let BSA = if BRP != 0.0 {
                                    BRO
                                } else {
                                    let BRZ = -BRO;
                                    BRZ
                                };
                                let BSB = S + (L * (BRY + (((BRY * BRY) + BSA).sqrt())));
                                let BSC = (HK * BRM) * BRT;
                                BRD = BRU;
                                BRE = BSB;
                                BRF = BSC;
                            } else {
                                BRD = S;
                                BRE = S;
                                BRF = BP;
                            }
                            let BRG = BRE * GR;
                            let BRH = F * ((DY / BRD) + ((DF * (BRD - BRE)) / BRG));
                            let BRI = if (BRH.abs()) < EA { 1.0 } else { 0.0 };
                            oBRI = BRI;
                            let BSF;
                            if BRI != 0.0 {
                                let BSD = BRH.exp();
                                BSF = BSD;
                            } else {
                                let BSE = if BRH < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBSE = BSE;
                                let BSM = if BSE != 0.0 {
                                    let BSJ = EH / (E + ((-2.3025850929940458e2f64 - BRH) * (E + (L * ((-2.3025850929940458e2f64 - BRH) * (E + ((-2.3025850929940458e2f64 - BRH) * EG)))))));
                                    BSJ
                                } else {
                                    let BSK = BRH - EA;
                                    let BSL = EK * (E + (BSK * (E + (L * (BSK * (E + (BSK * EG)))))));
                                    BSL
                                };
                                BSF = BSM;
                            }
                            let BSG = (E + (BMP * (F * (((BRD - (DY * BRF)) / (BRD * BRD)) + ((DF * BRF) / BRG))))) * BSF;
                            let BSH = (U / F) * ((IN / (CW / IN)).ln());
                            let BSI = if U < GR { 1.0 } else { 0.0 };
                            oBSI = BSI;
                            let BSR;
                            let BSS;
                            let BST;
                            if BSI != 0.0 {
                                let BSN = U - (HK * BSH);
                                let BSO = (GR - ((HK * (DY - BSH)) + U)) - HM;
                                let BSP = (HO * GR) * HM;
                                let BSQ = if BSP > BP { 1.0 } else { 0.0 };
                                oBSQ = BSQ;
                                let BSY = if BSQ != 0.0 {
                                    BSP
                                } else {
                                    let BSX = -BSP;
                                    BSX
                                };
                                let BSZ = ((BSO * BSO) + BSY).sqrt();
                                let BTA = L * (E + (BSO / BSZ));
                                let BTB = ((GR - (L * (BSO + BSZ))) - U) - HM;
                                let BTC = (HO * U) * HM;
                                let BTD = if BTC > BP { 1.0 } else { 0.0 };
                                oBTD = BTD;
                                let BTF = if BTD != 0.0 {
                                    BTC
                                } else {
                                    let BTE = -BTC;
                                    BTE
                                };
                                let BTG = ((BTB * BTB) + BTF).sqrt();
                                let BTH = L * (E + (BTB / BTG));
                                let BTI = U + (L * (BTB + BTG));
                                let BTJ = (GR - BSN) - HM;
                                let BTL = if BSQ != 0.0 {
                                    BSP
                                } else {
                                    let BTK = -BSP;
                                    BTK
                                };
                                let BTM = ((GR - (L * (BTJ + (((BTJ * BTJ) + BTL).sqrt())))) - U) - HM;
                                let BTO = if BTD != 0.0 {
                                    BTC
                                } else {
                                    let BTN = -BTC;
                                    BTN
                                };
                                let BTP = U + (L * (BTM + (((BTM * BTM) + BTO).sqrt())));
                                let BTQ = (HK * BTA) * BTH;
                                BSR = BTI;
                                BSS = BTP;
                                BST = BTQ;
                            } else {
                                BSR = U;
                                BSS = U;
                                BST = BP;
                            }
                            let BSU = BSS * GR;
                            let BSV = F * ((DY / BSR) + ((BSH * (BSR - BSS)) / BSU));
                            let BSW = if (BSV.abs()) < EA { 1.0 } else { 0.0 };
                            oBSW = BSW;
                            let BTT;
                            if BSW != 0.0 {
                                let BTR = BSV.exp();
                                BTT = BTR;
                            } else {
                                let BTS = if BSV < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBTS = BTS;
                                let BUA = if BTS != 0.0 {
                                    let BTX = EH / (E + ((-2.3025850929940458e2f64 - BSV) * (E + (L * ((-2.3025850929940458e2f64 - BSV) * (E + ((-2.3025850929940458e2f64 - BSV) * EG)))))));
                                    BTX
                                } else {
                                    let BTY = BSV - EA;
                                    let BTZ = EK * (E + (BTY * (E + (L * (BTY * (E + (BTY * EG)))))));
                                    BTZ
                                };
                                BTT = BUA;
                            }
                            let BTU = (E + (BMP * (F * (((BSR - (DY * BST)) / (BSR * BSR)) + ((BSH * BST) / BSU))))) * BTT;
                            let BTV = (W / F) * ((JU / (CW / JU)).ln());
                            let BTW = if W < GR { 1.0 } else { 0.0 };
                            oBTW = BTW;
                            let BUF;
                            let BUG;
                            let BUH;
                            if BTW != 0.0 {
                                let BUB = W - (HK * BTV);
                                let BUC = (GR - ((HK * (DY - BTV)) + W)) - HM;
                                let BUD = (HO * GR) * HM;
                                let BUE = if BUD > BP { 1.0 } else { 0.0 };
                                oBUE = BUE;
                                let BUM = if BUE != 0.0 {
                                    BUD
                                } else {
                                    let BUL = -BUD;
                                    BUL
                                };
                                let BUN = ((BUC * BUC) + BUM).sqrt();
                                let BUO = L * (E + (BUC / BUN));
                                let BUP = ((GR - (L * (BUC + BUN))) - W) - HM;
                                let BUQ = (HO * W) * HM;
                                let BUR = if BUQ > BP { 1.0 } else { 0.0 };
                                oBUR = BUR;
                                let BUT = if BUR != 0.0 {
                                    BUQ
                                } else {
                                    let BUS = -BUQ;
                                    BUS
                                };
                                let BUU = ((BUP * BUP) + BUT).sqrt();
                                let BUV = L * (E + (BUP / BUU));
                                let BUW = W + (L * (BUP + BUU));
                                let BUX = (GR - BUB) - HM;
                                let BUZ = if BUE != 0.0 {
                                    BUD
                                } else {
                                    let BUY = -BUD;
                                    BUY
                                };
                                let BVA = ((GR - (L * (BUX + (((BUX * BUX) + BUZ).sqrt())))) - W) - HM;
                                let BVC = if BUR != 0.0 {
                                    BUQ
                                } else {
                                    let BVB = -BUQ;
                                    BVB
                                };
                                let BVD = W + (L * (BVA + (((BVA * BVA) + BVC).sqrt())));
                                let BVE = (HK * BUO) * BUV;
                                BUF = BUW;
                                BUG = BVD;
                                BUH = BVE;
                            } else {
                                BUF = W;
                                BUG = W;
                                BUH = BP;
                            }
                            let BUI = BUG * GR;
                            let BUJ = F * ((DY / BUF) + ((BTV * (BUF - BUG)) / BUI));
                            let BUK = if (BUJ.abs()) < EA { 1.0 } else { 0.0 };
                            oBUK = BUK;
                            let BVH;
                            if BUK != 0.0 {
                                let BVF = BUJ.exp();
                                BVH = BVF;
                            } else {
                                let BVG = if BUJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBVG = BVG;
                                let BVM = if BVG != 0.0 {
                                    let BVJ = EH / (E + ((-2.3025850929940458e2f64 - BUJ) * (E + (L * ((-2.3025850929940458e2f64 - BUJ) * (E + ((-2.3025850929940458e2f64 - BUJ) * EG)))))));
                                    BVJ
                                } else {
                                    let BVK = BUJ - EA;
                                    let BVL = EK * (E + (BVK * (E + (L * (BVK * (E + (BVK * EG)))))));
                                    BVL
                                };
                                BVH = BVM;
                            }
                            let BVI = (E + (BMP * (F * (((BUF - (DY * BUH)) / (BUF * BUF)) + ((BTV * BUH) / BUI))))) * BVH;
                            BMS = BSG;
                            BMT = BTU;
                            BMU = BVI;
                            BMV = BMQ;
                        }
                        let BMW = BMS - E;
                        let BMX = BMT - E;
                        let BMY = BMU - E;
                        let BMZ = E / BMV;
                        let BVP = if BNA != 0.0 {
                            let BVN = AC * (D * (((AC + BMZ) + (((BMZ + E) * (BMZ + PT)).sqrt())).ln()));
                            BVN
                        } else {
                            let BVO = -1e-1f64 + (AC * (D * ((((AC * BMV) + E) + (((E + BMV) * (E + (PT * BMV))).sqrt())).ln())));
                            BVO
                        };
                        let BVQ = EV - BVP;
                        let BVR = BZ - BVQ;
                        let BVS = L * ((BZ + BVQ) - (((BVR * BVR) + ((HO * D) * D)).sqrt()));
                        BMH = BMW;
                        BMI = BVS;
                        BMJ = BVP;
                        BMK = BMV;
                        BML = BMX;
                        BMM = BMY;
                    } else {
                        BMH = BP;
                        BMI = BP;
                        BMJ = BP;
                        BMK = BP;
                        BML = BP;
                        BMM = BP;
                    }
                    let BVT;
                    if EF != 0.0 {
                        BVT = BP;
                    } else {
                        let BVU = Y * BMH;
                        let BVZ;
                        let BWA;
                        let BWB;
                        let BWC;
                        let BWD;
                        if BVV != 0.0 {
                            BVZ = BP;
                            BWA = BP;
                            BWB = BP;
                            BWC = BP;
                            BWD = BP;
                        } else {
                            let BVW = AK - BMI;
                            let BVX = E - ((E - (BMJ / BVW)).sqrt());
                            let BWG = if BVY != 0.0 {
                                BP
                            } else {
                                let BWF = ((((BVX * BVX) * (BVX.ln())) / (E - BVX)) + BVX) * staged[148];
                                BWF
                            };
                            let BWH = BVX + BWG;
                            let BWK = if BVY != 0.0 {
                                let BWI = (BVW * QP).sqrt();
                                BWI
                            } else {
                                let BWJ = (BVW * QP).powf(AQ);
                                BWJ
                            };
                            let BWL = QT * BWK;
                            let BWM = N * ((BMK - E) * BWL);
                            let BWN = QW * (BWM * BWH);
                            BVZ = BWL;
                            BWA = BVW;
                            BWB = BWH;
                            BWC = BWM;
                            BWD = BWN;
                        }
                        let BWW;
                        if BWE != 0.0 {
                            BWW = BP;
                        } else {
                            let BWO = BI * ((BVZ * QY) / BWA);
                            let BWP = (RA * BF) / BWO;
                            let BWQ = BWP * BWP;
                            let BWR = BWQ * BWQ;
                            let BWS = (BWR / (BWR + E)).sqrt();
                            let BWT = (BWS.abs()).sqrt();
                            let BWU = BWS * BWT;
                            let BXA = if BWV != 0.0 {
                                let BWY = E / (E + (BWO * BWU));
                                BWY
                            } else {
                                let BWZ = (E + (BWO * BWU)).powf(staged[149]);
                                BWZ
                            };
                            let BXB = (BWB * BXA) / (BWB + BXA);
                            let BXC = (RO * (BWO / BWT)).sqrt();
                            let BXD = (((BF * BWP) * BWT) - (BF * BWS)) + (L * (BWO * BWU));
                            let BXE = (((AC * (BWP * BWT)) - BWS) - E) * BXC;
                            let BXF = BXE * BXE;
                            let BXG = if BXE > BP { 1.0 } else { 0.0 };
                            oBXG = BXG;
                            let BXJ = if BXG != 0.0 {
                                let BXH = E / (E + (RU * BXE));
                                BXH
                            } else {
                                let BXI = E / (E - (RU * BXE));
                                BXI
                            };
                            let BXK = (-BXF) + BXD;
                            let BXL = if BXK > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oBXL = BXL;
                            let BXO = if BXL != 0.0 {
                                let BXM = BXK.exp();
                                BXM
                            } else {
                                let BXN = EH / (E + ((-2.3025850929940458e2f64 - BXK) * (E + (L * ((-2.3025850929940458e2f64 - BXK) * (E + ((-2.3025850929940458e2f64 - BXK) * EG)))))));
                                BXN
                            };
                            let BXP = BXJ * BXJ;
                            let BXQ = (((SD * BXJ) + (SF * BXP)) + (SG * (BXP * BXJ))) * BXO;
                            let BXS;
                            if BXG != 0.0 {
                                BXS = BXQ;
                            } else {
                                let BXR = if BXD > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBXR = BXR;
                                let BXW = if BXR != 0.0 {
                                    let BXU = BXD.exp();
                                    BXU
                                } else {
                                    let BXV = EH / (E + ((-2.3025850929940458e2f64 - BXD) * (E + (L * ((-2.3025850929940458e2f64 - BXD) * (E + ((-2.3025850929940458e2f64 - BXD) * EG)))))));
                                    BXV
                                };
                                let BXX = (AC * BXW) - BXQ;
                                BXS = BXX;
                            }
                            let BXT = SK * ((BWC * (8.86226925452758e-1f64 * ((BF * BXS) / BXC))) * BXB);
                            BWW = BXT;
                        }
                        let BXY;
                        if BWX != 0.0 {
                            BXY = BP;
                        } else {
                            let BYA = (-BR) / staged[150];
                            let BYB = if (BYA.abs()) < EA { 1.0 } else { 0.0 };
                            oBYB = BYB;
                            let BYE;
                            if BYB != 0.0 {
                                let BYC = BYA.exp();
                                BYE = BYC;
                            } else {
                                let BYD = if BYA < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBYD = BYD;
                                let BYJ = if BYD != 0.0 {
                                    let BYG = EH / (E + ((-2.3025850929940458e2f64 - BYA) * (E + (L * ((-2.3025850929940458e2f64 - BYA) * (E + ((-2.3025850929940458e2f64 - BYA) * EG)))))));
                                    BYG
                                } else {
                                    let BYH = BYA - EA;
                                    let BYI = EK * (E + (BYH * (E + (L * (BYH * (E + (BYH * EG)))))));
                                    BYI
                                };
                                BYE = BYJ;
                            }
                            let BYF = SY * (staged[151] * BYE);
                            BXY = BYF;
                        }
                        let BXZ = if (if CE > SR { 1.0 } else { 0.0 }) != 0.0 || staged[152] != 0.0 { 1.0 } else { 0.0 };
                        oBXZ = BXZ;
                        let BYM;
                        if BXZ != 0.0 {
                            BYM = E;
                        } else {
                            let BYL = if BYK > ((-TE) * CE) { 1.0 } else { 0.0 };
                            oBYL = BYL;
                            let BYQ;
                            if BYL != 0.0 {
                                let BYO = if TJ == HO { 1.0 } else { 0.0 };
                                oBYO = BYO;
                                let BYU = if BYO != 0.0 {
                                    let BYR = (BYK * CD).abs();
                                    let BYS = ((BYR * BYR) * BYR) * BYR;
                                    BYS
                                } else {
                                    let BYT = ((BYK * CD).abs()).powf(TJ);
                                    BYT
                                };
                                let BYV = E / (E - BYU);
                                BYQ = BYV;
                            } else {
                                let BYP = TL + ((BYK + (TE * CE)) * CN);
                                BYQ = BYP;
                            }
                            BYM = BYQ;
                        }
                        let BYN = (((BVU + BWD) + BWW) + BXY) * BYM;
                        BVT = BYN;
                    }
                    let BYW;
                    if EP != 0.0 {
                        BYW = BP;
                    } else {
                        let BYX = Z * BML;
                        let BZC;
                        let BZD;
                        let BZE;
                        let BZF;
                        let BZG;
                        if BYY != 0.0 {
                            BZC = BP;
                            BZD = BP;
                            BZE = BP;
                            BZF = BP;
                            BZG = BP;
                        } else {
                            let BYZ = AL - BMI;
                            let BZA = E - ((E - (BMJ / BYZ)).sqrt());
                            let BZJ = if BZB != 0.0 {
                                BP
                            } else {
                                let BZI = ((((BZA * BZA) * (BZA.ln())) / (E - BZA)) + BZA) * staged[154];
                                BZI
                            };
                            let BZK = BZA + BZJ;
                            let BZN = if BZB != 0.0 {
                                let BZL = (BYZ * UI).sqrt();
                                BZL
                            } else {
                                let BZM = (BYZ * UI).powf(AS);
                                BZM
                            };
                            let BZO = UM * BZN;
                            let BZP = P * ((BMK - E) * BZO);
                            let BZQ = UP * (BZP * BZK);
                            BZC = BZO;
                            BZD = BYZ;
                            BZE = BZK;
                            BZF = BZP;
                            BZG = BZQ;
                        }
                        let BZZ;
                        if BZH != 0.0 {
                            BZZ = BP;
                        } else {
                            let BZR = BJ * ((BZC * UR) / BZD);
                            let BZS = (RA * BG) / BZR;
                            let BZT = BZS * BZS;
                            let BZU = BZT * BZT;
                            let BZV = (BZU / (BZU + E)).sqrt();
                            let BZW = (BZV.abs()).sqrt();
                            let BZX = BZV * BZW;
                            let CAD = if BZY != 0.0 {
                                let CAB = E / (E + (BZR * BZX));
                                CAB
                            } else {
                                let CAC = (E + (BZR * BZX)).powf(staged[155]);
                                CAC
                            };
                            let CAE = (BZE * CAD) / (BZE + CAD);
                            let CAF = (RO * (BZR / BZW)).sqrt();
                            let CAG = (((BG * BZS) * BZW) - (BG * BZV)) + (L * (BZR * BZX));
                            let CAH = (((AC * (BZS * BZW)) - BZV) - E) * CAF;
                            let CAI = CAH * CAH;
                            let CAJ = if CAH > BP { 1.0 } else { 0.0 };
                            oCAJ = CAJ;
                            let CAM = if CAJ != 0.0 {
                                let CAK = E / (E + (RU * CAH));
                                CAK
                            } else {
                                let CAL = E / (E - (RU * CAH));
                                CAL
                            };
                            let CAN = (-CAI) + CAG;
                            let CAO = if CAN > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oCAO = CAO;
                            let CAR = if CAO != 0.0 {
                                let CAP = CAN.exp();
                                CAP
                            } else {
                                let CAQ = EH / (E + ((-2.3025850929940458e2f64 - CAN) * (E + (L * ((-2.3025850929940458e2f64 - CAN) * (E + ((-2.3025850929940458e2f64 - CAN) * EG)))))));
                                CAQ
                            };
                            let CAS = CAM * CAM;
                            let CAT = (((SD * CAM) + (SF * CAS)) + (SG * (CAS * CAM))) * CAR;
                            let CAV;
                            if CAJ != 0.0 {
                                CAV = CAT;
                            } else {
                                let CAU = if CAG > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCAU = CAU;
                                let CAZ = if CAU != 0.0 {
                                    let CAX = CAG.exp();
                                    CAX
                                } else {
                                    let CAY = EH / (E + ((-2.3025850929940458e2f64 - CAG) * (E + (L * ((-2.3025850929940458e2f64 - CAG) * (E + ((-2.3025850929940458e2f64 - CAG) * EG)))))));
                                    CAY
                                };
                                let CBA = (AC * CAZ) - CAT;
                                CAV = CBA;
                            }
                            let CAW = VX * ((BZF * (8.86226925452758e-1f64 * ((BG * CAV) / CAF))) * CAE);
                            BZZ = CAW;
                        }
                        let CBB;
                        if CAA != 0.0 {
                            CBB = BP;
                        } else {
                            let CBD = (-BT) / staged[156];
                            let CBE = if (CBD.abs()) < EA { 1.0 } else { 0.0 };
                            oCBE = CBE;
                            let CBH;
                            if CBE != 0.0 {
                                let CBF = CBD.exp();
                                CBH = CBF;
                            } else {
                                let CBG = if CBD < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCBG = CBG;
                                let CBM = if CBG != 0.0 {
                                    let CBJ = EH / (E + ((-2.3025850929940458e2f64 - CBD) * (E + (L * ((-2.3025850929940458e2f64 - CBD) * (E + ((-2.3025850929940458e2f64 - CBD) * EG)))))));
                                    CBJ
                                } else {
                                    let CBK = CBD - EA;
                                    let CBL = EK * (E + (CBK * (E + (L * (CBK * (E + (CBK * EG)))))));
                                    CBL
                                };
                                CBH = CBM;
                            }
                            let CBI = WK * (staged[157] * CBH);
                            CBB = CBI;
                        }
                        let CBC = if (if CI > SR { 1.0 } else { 0.0 }) != 0.0 || staged[158] != 0.0 { 1.0 } else { 0.0 };
                        oCBC = CBC;
                        let CBO;
                        if CBC != 0.0 {
                            CBO = E;
                        } else {
                            let CBN = if BYK > ((-TE) * CI) { 1.0 } else { 0.0 };
                            oCBN = CBN;
                            let CBS;
                            if CBN != 0.0 {
                                let CBQ = if WT == HO { 1.0 } else { 0.0 };
                                oCBQ = CBQ;
                                let CBW = if CBQ != 0.0 {
                                    let CBT = (BYK * CH).abs();
                                    let CBU = ((CBT * CBT) * CBT) * CBT;
                                    CBU
                                } else {
                                    let CBV = ((BYK * CH).abs()).powf(WT);
                                    CBV
                                };
                                let CBX = E / (E - CBW);
                                CBS = CBX;
                            } else {
                                let CBR = WV + ((BYK + (TE * CI)) * CO);
                                CBS = CBR;
                            }
                            CBO = CBS;
                        }
                        let CBP = (((BYX + BZG) + BZZ) + CBB) * CBO;
                        BYW = CBP;
                    }
                    let CBY;
                    if ES != 0.0 {
                        CBY = BP;
                    } else {
                        let CCA = AA * BMM;
                        let CCF;
                        let CCG;
                        let CCH;
                        let CCI;
                        let CCJ;
                        if CCB != 0.0 {
                            CCF = BP;
                            CCG = BP;
                            CCH = BP;
                            CCI = BP;
                            CCJ = BP;
                        } else {
                            let CCC = AM - BMI;
                            let CCD = E - ((E - (BMJ / CCC)).sqrt());
                            let CCM = if CCE != 0.0 {
                                BP
                            } else {
                                let CCL = ((((CCD * CCD) * (CCD.ln())) / (E - CCD)) + CCD) * staged[159];
                                CCL
                            };
                            let CCN = CCD + CCM;
                            let CCQ = if CCE != 0.0 {
                                let CCO = (CCC * XT).sqrt();
                                CCO
                            } else {
                                let CCP = (CCC * XT).powf(AU);
                                CCP
                            };
                            let CCR = XX * CCQ;
                            let CCS = R * ((BMK - E) * CCR);
                            let CCT = YA * (CCS * CCN);
                            CCF = CCR;
                            CCG = CCC;
                            CCH = CCN;
                            CCI = CCS;
                            CCJ = CCT;
                        }
                        let CDC;
                        if CCK != 0.0 {
                            CDC = BP;
                        } else {
                            let CCU = BK * ((CCF * YC) / CCG);
                            let CCV = (RA * BH) / CCU;
                            let CCW = CCV * CCV;
                            let CCX = CCW * CCW;
                            let CCY = (CCX / (CCX + E)).sqrt();
                            let CCZ = (CCY.abs()).sqrt();
                            let CDA = CCY * CCZ;
                            let CDG = if CDB != 0.0 {
                                let CDE = E / (E + (CCU * CDA));
                                CDE
                            } else {
                                let CDF = (E + (CCU * CDA)).powf(staged[160]);
                                CDF
                            };
                            let CDH = (CCH * CDG) / (CCH + CDG);
                            let CDI = (RO * (CCU / CCZ)).sqrt();
                            let CDJ = (((BH * CCV) * CCZ) - (BH * CCY)) + (L * (CCU * CDA));
                            let CDK = (((AC * (CCV * CCZ)) - CCY) - E) * CDI;
                            let CDL = CDK * CDK;
                            let CDM = if CDK > BP { 1.0 } else { 0.0 };
                            oCDM = CDM;
                            let CDP = if CDM != 0.0 {
                                let CDN = E / (E + (RU * CDK));
                                CDN
                            } else {
                                let CDO = E / (E - (RU * CDK));
                                CDO
                            };
                            let CDQ = (-CDL) + CDJ;
                            let CDR = if CDQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oCDR = CDR;
                            let CDU = if CDR != 0.0 {
                                let CDS = CDQ.exp();
                                CDS
                            } else {
                                let CDT = EH / (E + ((-2.3025850929940458e2f64 - CDQ) * (E + (L * ((-2.3025850929940458e2f64 - CDQ) * (E + ((-2.3025850929940458e2f64 - CDQ) * EG)))))));
                                CDT
                            };
                            let CDV = CDP * CDP;
                            let CDW = (((SD * CDP) + (SF * CDV)) + (SG * (CDV * CDP))) * CDU;
                            let CDY;
                            if CDM != 0.0 {
                                CDY = CDW;
                            } else {
                                let CDX = if CDJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCDX = CDX;
                                let CEC = if CDX != 0.0 {
                                    let CEA = CDJ.exp();
                                    CEA
                                } else {
                                    let CEB = EH / (E + ((-2.3025850929940458e2f64 - CDJ) * (E + (L * ((-2.3025850929940458e2f64 - CDJ) * (E + ((-2.3025850929940458e2f64 - CDJ) * EG)))))));
                                    CEB
                                };
                                let CED = (AC * CEC) - CDW;
                                CDY = CED;
                            }
                            let CDZ = ZI * ((CCI * (8.86226925452758e-1f64 * ((BH * CDY) / CDI))) * CDH);
                            CDC = CDZ;
                        }
                        let CEE;
                        if CDD != 0.0 {
                            CEE = BP;
                        } else {
                            let CEG = (-BV) / staged[161];
                            let CEH = if (CEG.abs()) < EA { 1.0 } else { 0.0 };
                            oCEH = CEH;
                            let CEK;
                            if CEH != 0.0 {
                                let CEI = CEG.exp();
                                CEK = CEI;
                            } else {
                                let CEJ = if CEG < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCEJ = CEJ;
                                let CEP = if CEJ != 0.0 {
                                    let CEM = EH / (E + ((-2.3025850929940458e2f64 - CEG) * (E + (L * ((-2.3025850929940458e2f64 - CEG) * (E + ((-2.3025850929940458e2f64 - CEG) * EG)))))));
                                    CEM
                                } else {
                                    let CEN = CEG - EA;
                                    let CEO = EK * (E + (CEN * (E + (L * (CEN * (E + (CEN * EG)))))));
                                    CEO
                                };
                                CEK = CEP;
                            }
                            let CEL = ZV * (staged[162] * CEK);
                            CEE = CEL;
                        }
                        let CEF = if (if CM > SR { 1.0 } else { 0.0 }) != 0.0 || staged[163] != 0.0 { 1.0 } else { 0.0 };
                        oCEF = CEF;
                        let CER;
                        if CEF != 0.0 {
                            CER = E;
                        } else {
                            let CEQ = if BYK > ((-TE) * CM) { 1.0 } else { 0.0 };
                            oCEQ = CEQ;
                            let CEV;
                            if CEQ != 0.0 {
                                let CET = if AAE == HO { 1.0 } else { 0.0 };
                                oCET = CET;
                                let CEZ = if CET != 0.0 {
                                    let CEW = (BYK * CL).abs();
                                    let CEX = ((CEW * CEW) * CEW) * CEW;
                                    CEX
                                } else {
                                    let CEY = ((BYK * CL).abs()).powf(AAE);
                                    CEY
                                };
                                let CFA = E / (E - CEZ);
                                CEV = CFA;
                            } else {
                                let CEU = AAG + ((BYK + (TE * CM)) * CP);
                                CEV = CEU;
                            }
                            CER = CEV;
                        }
                        let CES = (((CCA + CCJ) + CDC) + CEE) * CER;
                        CBY = CES;
                    }
                    let CBZ = ((DH * BVT) + (DO * BYW)) + (DT * CBY);
                    let CFD;
                    let CFE;
                    let CFF;
                    let CFG;
                    let CFH;
                    let CFI;
                    if FO != 0.0 {
                        let CFC = if CFB < DY { 1.0 } else { 0.0 };
                        oCFC = CFC;
                        let CFO;
                        let CFP;
                        let CFQ;
                        let CFR;
                        if CFC != 0.0 {
                            let CFJ = L * (CFB * F);
                            let CFK = if (CFJ.abs()) < EA { 1.0 } else { 0.0 };
                            oCFK = CFK;
                            let CFZ;
                            if CFK != 0.0 {
                                let CFX = CFJ.exp();
                                CFZ = CFX;
                            } else {
                                let CFY = if CFJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCFY = CFY;
                                let CGE = if CFY != 0.0 {
                                    let CGB = EH / (E + ((-2.3025850929940458e2f64 - CFJ) * (E + (L * ((-2.3025850929940458e2f64 - CFJ) * (E + ((-2.3025850929940458e2f64 - CFJ) * EG)))))));
                                    CGB
                                } else {
                                    let CGC = CFJ - EA;
                                    let CGD = EK * (E + (CGC * (E + (L * (CGC * (E + (CGC * EG)))))));
                                    CGD
                                };
                                CFZ = CGE;
                            }
                            let CGA = if S < GR { 1.0 } else { 0.0 };
                            oCGA = CGA;
                            let CGJ;
                            let CGK;
                            if CGA != 0.0 {
                                let CGF = S - (HK * DF);
                                let CGG = (GR - ((HK * (CFB - DF)) + S)) - HM;
                                let CGH = (HO * GR) * HM;
                                let CGI = if CGH > BP { 1.0 } else { 0.0 };
                                oCGI = CGI;
                                let CGO = if CGI != 0.0 {
                                    CGH
                                } else {
                                    let CGN = -CGH;
                                    CGN
                                };
                                let CGP = ((GR - (L * (CGG + (((CGG * CGG) + CGO).sqrt())))) - S) - HM;
                                let CGQ = (HO * S) * HM;
                                let CGR = if CGQ > BP { 1.0 } else { 0.0 };
                                oCGR = CGR;
                                let CGT = if CGR != 0.0 {
                                    CGQ
                                } else {
                                    let CGS = -CGQ;
                                    CGS
                                };
                                let CGU = S + (L * (CGP + (((CGP * CGP) + CGT).sqrt())));
                                let CGV = (GR - CGF) - HM;
                                let CGX = if CGI != 0.0 {
                                    CGH
                                } else {
                                    let CGW = -CGH;
                                    CGW
                                };
                                let CGY = ((GR - (L * (CGV + (((CGV * CGV) + CGX).sqrt())))) - S) - HM;
                                let CHA = if CGR != 0.0 {
                                    CGQ
                                } else {
                                    let CGZ = -CGQ;
                                    CGZ
                                };
                                let CHB = S + (L * (CGY + (((CGY * CGY) + CHA).sqrt())));
                                CGJ = CGU;
                                CGK = CHB;
                            } else {
                                CGJ = S;
                                CGK = S;
                            }
                            let CGL = F * ((CFB / CGJ) + ((DF * (CGJ - CGK)) / (CGK * GR)));
                            let CGM = if (CGL.abs()) < EA { 1.0 } else { 0.0 };
                            oCGM = CGM;
                            let CHE;
                            if CGM != 0.0 {
                                let CHC = CGL.exp();
                                CHE = CHC;
                            } else {
                                let CHD = if CGL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCHD = CHD;
                                let CHK = if CHD != 0.0 {
                                    let CHH = EH / (E + ((-2.3025850929940458e2f64 - CGL) * (E + (L * ((-2.3025850929940458e2f64 - CGL) * (E + ((-2.3025850929940458e2f64 - CGL) * EG)))))));
                                    CHH
                                } else {
                                    let CHI = CGL - EA;
                                    let CHJ = EK * (E + (CHI * (E + (L * (CHI * (E + (CHI * EG)))))));
                                    CHJ
                                };
                                CHE = CHK;
                            }
                            let CHF = (U / F) * ((IN / (CW / IN)).ln());
                            let CHG = if U < GR { 1.0 } else { 0.0 };
                            oCHG = CHG;
                            let CHP;
                            let CHQ;
                            if CHG != 0.0 {
                                let CHL = U - (HK * CHF);
                                let CHM = (GR - ((HK * (CFB - CHF)) + U)) - HM;
                                let CHN = (HO * GR) * HM;
                                let CHO = if CHN > BP { 1.0 } else { 0.0 };
                                oCHO = CHO;
                                let CHU = if CHO != 0.0 {
                                    CHN
                                } else {
                                    let CHT = -CHN;
                                    CHT
                                };
                                let CHV = ((GR - (L * (CHM + (((CHM * CHM) + CHU).sqrt())))) - U) - HM;
                                let CHW = (HO * U) * HM;
                                let CHX = if CHW > BP { 1.0 } else { 0.0 };
                                oCHX = CHX;
                                let CHZ = if CHX != 0.0 {
                                    CHW
                                } else {
                                    let CHY = -CHW;
                                    CHY
                                };
                                let CIA = U + (L * (CHV + (((CHV * CHV) + CHZ).sqrt())));
                                let CIB = (GR - CHL) - HM;
                                let CID = if CHO != 0.0 {
                                    CHN
                                } else {
                                    let CIC = -CHN;
                                    CIC
                                };
                                let CIE = ((GR - (L * (CIB + (((CIB * CIB) + CID).sqrt())))) - U) - HM;
                                let CIG = if CHX != 0.0 {
                                    CHW
                                } else {
                                    let CIF = -CHW;
                                    CIF
                                };
                                let CIH = U + (L * (CIE + (((CIE * CIE) + CIG).sqrt())));
                                CHP = CIA;
                                CHQ = CIH;
                            } else {
                                CHP = U;
                                CHQ = U;
                            }
                            let CHR = F * ((CFB / CHP) + ((CHF * (CHP - CHQ)) / (CHQ * GR)));
                            let CHS = if (CHR.abs()) < EA { 1.0 } else { 0.0 };
                            oCHS = CHS;
                            let CIK;
                            if CHS != 0.0 {
                                let CII = CHR.exp();
                                CIK = CII;
                            } else {
                                let CIJ = if CHR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCIJ = CIJ;
                                let CIQ = if CIJ != 0.0 {
                                    let CIN = EH / (E + ((-2.3025850929940458e2f64 - CHR) * (E + (L * ((-2.3025850929940458e2f64 - CHR) * (E + ((-2.3025850929940458e2f64 - CHR) * EG)))))));
                                    CIN
                                } else {
                                    let CIO = CHR - EA;
                                    let CIP = EK * (E + (CIO * (E + (L * (CIO * (E + (CIO * EG)))))));
                                    CIP
                                };
                                CIK = CIQ;
                            }
                            let CIL = (W / F) * ((JU / (CW / JU)).ln());
                            let CIM = if W < GR { 1.0 } else { 0.0 };
                            oCIM = CIM;
                            let CIV;
                            let CIW;
                            if CIM != 0.0 {
                                let CIR = W - (HK * CIL);
                                let CIS = (GR - ((HK * (CFB - CIL)) + W)) - HM;
                                let CIT = (HO * GR) * HM;
                                let CIU = if CIT > BP { 1.0 } else { 0.0 };
                                oCIU = CIU;
                                let CJA = if CIU != 0.0 {
                                    CIT
                                } else {
                                    let CIZ = -CIT;
                                    CIZ
                                };
                                let CJB = ((GR - (L * (CIS + (((CIS * CIS) + CJA).sqrt())))) - W) - HM;
                                let CJC = (HO * W) * HM;
                                let CJD = if CJC > BP { 1.0 } else { 0.0 };
                                oCJD = CJD;
                                let CJF = if CJD != 0.0 {
                                    CJC
                                } else {
                                    let CJE = -CJC;
                                    CJE
                                };
                                let CJG = W + (L * (CJB + (((CJB * CJB) + CJF).sqrt())));
                                let CJH = (GR - CIR) - HM;
                                let CJJ = if CIU != 0.0 {
                                    CIT
                                } else {
                                    let CJI = -CIT;
                                    CJI
                                };
                                let CJK = ((GR - (L * (CJH + (((CJH * CJH) + CJJ).sqrt())))) - W) - HM;
                                let CJM = if CJD != 0.0 {
                                    CJC
                                } else {
                                    let CJL = -CJC;
                                    CJL
                                };
                                let CJN = W + (L * (CJK + (((CJK * CJK) + CJM).sqrt())));
                                CIV = CJG;
                                CIW = CJN;
                            } else {
                                CIV = W;
                                CIW = W;
                            }
                            let CIX = F * ((CFB / CIV) + ((CIL * (CIV - CIW)) / (CIW * GR)));
                            let CIY = if (CIX.abs()) < EA { 1.0 } else { 0.0 };
                            oCIY = CIY;
                            let CJQ;
                            if CIY != 0.0 {
                                let CJO = CIX.exp();
                                CJQ = CJO;
                            } else {
                                let CJP = if CIX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCJP = CJP;
                                let CJU = if CJP != 0.0 {
                                    let CJR = EH / (E + ((-2.3025850929940458e2f64 - CIX) * (E + (L * ((-2.3025850929940458e2f64 - CIX) * (E + ((-2.3025850929940458e2f64 - CIX) * EG)))))));
                                    CJR
                                } else {
                                    let CJS = CIX - EA;
                                    let CJT = EK * (E + (CJS * (E + (L * (CJS * (E + (CJS * EG)))))));
                                    CJT
                                };
                                CJQ = CJU;
                            }
                            CFO = CHE;
                            CFP = CIK;
                            CFQ = CJQ;
                            CFR = CFZ;
                        } else {
                            let CFL = CFB - DY;
                            let CFM = ((E + (CFL * F)) * EE).sqrt();
                            let CFN = if S < GR { 1.0 } else { 0.0 };
                            oCFN = CFN;
                            let CJZ;
                            let CKA;
                            let CKB;
                            if CFN != 0.0 {
                                let CJV = S - (HK * DF);
                                let CJW = (GR - ((HK * (DY - DF)) + S)) - HM;
                                let CJX = (HO * GR) * HM;
                                let CJY = if CJX > BP { 1.0 } else { 0.0 };
                                oCJY = CJY;
                                let CKG = if CJY != 0.0 {
                                    CJX
                                } else {
                                    let CKF = -CJX;
                                    CKF
                                };
                                let CKH = ((CJW * CJW) + CKG).sqrt();
                                let CKI = L * (E + (CJW / CKH));
                                let CKJ = ((GR - (L * (CJW + CKH))) - S) - HM;
                                let CKK = (HO * S) * HM;
                                let CKL = if CKK > BP { 1.0 } else { 0.0 };
                                oCKL = CKL;
                                let CKN = if CKL != 0.0 {
                                    CKK
                                } else {
                                    let CKM = -CKK;
                                    CKM
                                };
                                let CKO = ((CKJ * CKJ) + CKN).sqrt();
                                let CKP = L * (E + (CKJ / CKO));
                                let CKQ = S + (L * (CKJ + CKO));
                                let CKR = (GR - CJV) - HM;
                                let CKT = if CJY != 0.0 {
                                    CJX
                                } else {
                                    let CKS = -CJX;
                                    CKS
                                };
                                let CKU = ((GR - (L * (CKR + (((CKR * CKR) + CKT).sqrt())))) - S) - HM;
                                let CKW = if CKL != 0.0 {
                                    CKK
                                } else {
                                    let CKV = -CKK;
                                    CKV
                                };
                                let CKX = S + (L * (CKU + (((CKU * CKU) + CKW).sqrt())));
                                let CKY = (HK * CKI) * CKP;
                                CJZ = CKQ;
                                CKA = CKX;
                                CKB = CKY;
                            } else {
                                CJZ = S;
                                CKA = S;
                                CKB = BP;
                            }
                            let CKC = CKA * GR;
                            let CKD = F * ((DY / CJZ) + ((DF * (CJZ - CKA)) / CKC));
                            let CKE = if (CKD.abs()) < EA { 1.0 } else { 0.0 };
                            oCKE = CKE;
                            let CLB;
                            if CKE != 0.0 {
                                let CKZ = CKD.exp();
                                CLB = CKZ;
                            } else {
                                let CLA = if CKD < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCLA = CLA;
                                let CLI = if CLA != 0.0 {
                                    let CLF = EH / (E + ((-2.3025850929940458e2f64 - CKD) * (E + (L * ((-2.3025850929940458e2f64 - CKD) * (E + ((-2.3025850929940458e2f64 - CKD) * EG)))))));
                                    CLF
                                } else {
                                    let CLG = CKD - EA;
                                    let CLH = EK * (E + (CLG * (E + (L * (CLG * (E + (CLG * EG)))))));
                                    CLH
                                };
                                CLB = CLI;
                            }
                            let CLC = (E + (CFL * (F * (((CJZ - (DY * CKB)) / (CJZ * CJZ)) + ((DF * CKB) / CKC))))) * CLB;
                            let CLD = (U / F) * ((IN / (CW / IN)).ln());
                            let CLE = if U < GR { 1.0 } else { 0.0 };
                            oCLE = CLE;
                            let CLN;
                            let CLO;
                            let CLP;
                            if CLE != 0.0 {
                                let CLJ = U - (HK * CLD);
                                let CLK = (GR - ((HK * (DY - CLD)) + U)) - HM;
                                let CLL = (HO * GR) * HM;
                                let CLM = if CLL > BP { 1.0 } else { 0.0 };
                                oCLM = CLM;
                                let CLU = if CLM != 0.0 {
                                    CLL
                                } else {
                                    let CLT = -CLL;
                                    CLT
                                };
                                let CLV = ((CLK * CLK) + CLU).sqrt();
                                let CLW = L * (E + (CLK / CLV));
                                let CLX = ((GR - (L * (CLK + CLV))) - U) - HM;
                                let CLY = (HO * U) * HM;
                                let CLZ = if CLY > BP { 1.0 } else { 0.0 };
                                oCLZ = CLZ;
                                let CMB = if CLZ != 0.0 {
                                    CLY
                                } else {
                                    let CMA = -CLY;
                                    CMA
                                };
                                let CMC = ((CLX * CLX) + CMB).sqrt();
                                let CMD = L * (E + (CLX / CMC));
                                let CME = U + (L * (CLX + CMC));
                                let CMF = (GR - CLJ) - HM;
                                let CMH = if CLM != 0.0 {
                                    CLL
                                } else {
                                    let CMG = -CLL;
                                    CMG
                                };
                                let CMI = ((GR - (L * (CMF + (((CMF * CMF) + CMH).sqrt())))) - U) - HM;
                                let CMK = if CLZ != 0.0 {
                                    CLY
                                } else {
                                    let CMJ = -CLY;
                                    CMJ
                                };
                                let CML = U + (L * (CMI + (((CMI * CMI) + CMK).sqrt())));
                                let CMM = (HK * CLW) * CMD;
                                CLN = CME;
                                CLO = CML;
                                CLP = CMM;
                            } else {
                                CLN = U;
                                CLO = U;
                                CLP = BP;
                            }
                            let CLQ = CLO * GR;
                            let CLR = F * ((DY / CLN) + ((CLD * (CLN - CLO)) / CLQ));
                            let CLS = if (CLR.abs()) < EA { 1.0 } else { 0.0 };
                            oCLS = CLS;
                            let CMP;
                            if CLS != 0.0 {
                                let CMN = CLR.exp();
                                CMP = CMN;
                            } else {
                                let CMO = if CLR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCMO = CMO;
                                let CMW = if CMO != 0.0 {
                                    let CMT = EH / (E + ((-2.3025850929940458e2f64 - CLR) * (E + (L * ((-2.3025850929940458e2f64 - CLR) * (E + ((-2.3025850929940458e2f64 - CLR) * EG)))))));
                                    CMT
                                } else {
                                    let CMU = CLR - EA;
                                    let CMV = EK * (E + (CMU * (E + (L * (CMU * (E + (CMU * EG)))))));
                                    CMV
                                };
                                CMP = CMW;
                            }
                            let CMQ = (E + (CFL * (F * (((CLN - (DY * CLP)) / (CLN * CLN)) + ((CLD * CLP) / CLQ))))) * CMP;
                            let CMR = (W / F) * ((JU / (CW / JU)).ln());
                            let CMS = if W < GR { 1.0 } else { 0.0 };
                            oCMS = CMS;
                            let CNB;
                            let CNC;
                            let CND;
                            if CMS != 0.0 {
                                let CMX = W - (HK * CMR);
                                let CMY = (GR - ((HK * (DY - CMR)) + W)) - HM;
                                let CMZ = (HO * GR) * HM;
                                let CNA = if CMZ > BP { 1.0 } else { 0.0 };
                                oCNA = CNA;
                                let CNI = if CNA != 0.0 {
                                    CMZ
                                } else {
                                    let CNH = -CMZ;
                                    CNH
                                };
                                let CNJ = ((CMY * CMY) + CNI).sqrt();
                                let CNK = L * (E + (CMY / CNJ));
                                let CNL = ((GR - (L * (CMY + CNJ))) - W) - HM;
                                let CNM = (HO * W) * HM;
                                let CNN = if CNM > BP { 1.0 } else { 0.0 };
                                oCNN = CNN;
                                let CNP = if CNN != 0.0 {
                                    CNM
                                } else {
                                    let CNO = -CNM;
                                    CNO
                                };
                                let CNQ = ((CNL * CNL) + CNP).sqrt();
                                let CNR = L * (E + (CNL / CNQ));
                                let CNS = W + (L * (CNL + CNQ));
                                let CNT = (GR - CMX) - HM;
                                let CNV = if CNA != 0.0 {
                                    CMZ
                                } else {
                                    let CNU = -CMZ;
                                    CNU
                                };
                                let CNW = ((GR - (L * (CNT + (((CNT * CNT) + CNV).sqrt())))) - W) - HM;
                                let CNY = if CNN != 0.0 {
                                    CNM
                                } else {
                                    let CNX = -CNM;
                                    CNX
                                };
                                let CNZ = W + (L * (CNW + (((CNW * CNW) + CNY).sqrt())));
                                let COA = (HK * CNK) * CNR;
                                CNB = CNS;
                                CNC = CNZ;
                                CND = COA;
                            } else {
                                CNB = W;
                                CNC = W;
                                CND = BP;
                            }
                            let CNE = CNC * GR;
                            let CNF = F * ((DY / CNB) + ((CMR * (CNB - CNC)) / CNE));
                            let CNG = if (CNF.abs()) < EA { 1.0 } else { 0.0 };
                            oCNG = CNG;
                            let COD;
                            if CNG != 0.0 {
                                let COB = CNF.exp();
                                COD = COB;
                            } else {
                                let COC = if CNF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCOC = COC;
                                let COI = if COC != 0.0 {
                                    let COF = EH / (E + ((-2.3025850929940458e2f64 - CNF) * (E + (L * ((-2.3025850929940458e2f64 - CNF) * (E + ((-2.3025850929940458e2f64 - CNF) * EG)))))));
                                    COF
                                } else {
                                    let COG = CNF - EA;
                                    let COH = EK * (E + (COG * (E + (L * (COG * (E + (COG * EG)))))));
                                    COH
                                };
                                COD = COI;
                            }
                            let COE = (E + (CFL * (F * (((CNB - (DY * CND)) / (CNB * CNB)) + ((CMR * CND) / CNE))))) * COD;
                            CFO = CLC;
                            CFP = CMQ;
                            CFQ = COE;
                            CFR = CFM;
                        }
                        let CFS = CFO - E;
                        let CFT = CFP - E;
                        let CFU = CFQ - E;
                        let CFV = E / CFR;
                        let COL = if CFW != 0.0 {
                            let COJ = AC * (D * (((AC + CFV) + (((CFV + E) * (CFV + PT)).sqrt())).ln()));
                            COJ
                        } else {
                            let COK = -2e-1f64 + (AC * (D * ((((AC * CFR) + E) + (((E + CFR) * (E + (PT * CFR))).sqrt())).ln())));
                            COK
                        };
                        let COM = EV - COL;
                        let CON = CFB - COM;
                        let COO = L * ((CFB + COM) - (((CON * CON) + ((HO * D) * D)).sqrt()));
                        CFD = CFS;
                        CFE = COO;
                        CFF = COL;
                        CFG = CFR;
                        CFH = CFT;
                        CFI = CFU;
                    } else {
                        CFD = BP;
                        CFE = BP;
                        CFF = BP;
                        CFG = BP;
                        CFH = BP;
                        CFI = BP;
                    }
                    let COP;
                    if EF != 0.0 {
                        COP = BP;
                    } else {
                        let COQ = Y * CFD;
                        let COV;
                        let COW;
                        let COX;
                        let COY;
                        let COZ;
                        if COR != 0.0 {
                            COV = BP;
                            COW = BP;
                            COX = BP;
                            COY = BP;
                            COZ = BP;
                        } else {
                            let COS = AK - CFE;
                            let COT = E - ((E - (CFF / COS)).sqrt());
                            let CPC = if COU != 0.0 {
                                BP
                            } else {
                                let CPB = ((((COT * COT) * (COT.ln())) / (E - COT)) + COT) * staged[164];
                                CPB
                            };
                            let CPD = COT + CPC;
                            let CPG = if COU != 0.0 {
                                let CPE = (COS * QP).sqrt();
                                CPE
                            } else {
                                let CPF = (COS * QP).powf(AQ);
                                CPF
                            };
                            let CPH = QT * CPG;
                            let CPI = N * ((CFG - E) * CPH);
                            let CPJ = QW * (CPI * CPD);
                            COV = CPH;
                            COW = COS;
                            COX = CPD;
                            COY = CPI;
                            COZ = CPJ;
                        }
                        let CPS;
                        if CPA != 0.0 {
                            CPS = BP;
                        } else {
                            let CPK = BI * ((COV * QY) / COW);
                            let CPL = (RA * BF) / CPK;
                            let CPM = CPL * CPL;
                            let CPN = CPM * CPM;
                            let CPO = (CPN / (CPN + E)).sqrt();
                            let CPP = (CPO.abs()).sqrt();
                            let CPQ = CPO * CPP;
                            let CPW = if CPR != 0.0 {
                                let CPU = E / (E + (CPK * CPQ));
                                CPU
                            } else {
                                let CPV = (E + (CPK * CPQ)).powf(staged[165]);
                                CPV
                            };
                            let CPX = (COX * CPW) / (COX + CPW);
                            let CPY = (RO * (CPK / CPP)).sqrt();
                            let CPZ = (((BF * CPL) * CPP) - (BF * CPO)) + (L * (CPK * CPQ));
                            let CQA = (((AC * (CPL * CPP)) - CPO) - E) * CPY;
                            let CQB = CQA * CQA;
                            let CQC = if CQA > BP { 1.0 } else { 0.0 };
                            oCQC = CQC;
                            let CQF = if CQC != 0.0 {
                                let CQD = E / (E + (RU * CQA));
                                CQD
                            } else {
                                let CQE = E / (E - (RU * CQA));
                                CQE
                            };
                            let CQG = (-CQB) + CPZ;
                            let CQH = if CQG > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oCQH = CQH;
                            let CQK = if CQH != 0.0 {
                                let CQI = CQG.exp();
                                CQI
                            } else {
                                let CQJ = EH / (E + ((-2.3025850929940458e2f64 - CQG) * (E + (L * ((-2.3025850929940458e2f64 - CQG) * (E + ((-2.3025850929940458e2f64 - CQG) * EG)))))));
                                CQJ
                            };
                            let CQL = CQF * CQF;
                            let CQM = (((SD * CQF) + (SF * CQL)) + (SG * (CQL * CQF))) * CQK;
                            let CQO;
                            if CQC != 0.0 {
                                CQO = CQM;
                            } else {
                                let CQN = if CPZ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCQN = CQN;
                                let CQS = if CQN != 0.0 {
                                    let CQQ = CPZ.exp();
                                    CQQ
                                } else {
                                    let CQR = EH / (E + ((-2.3025850929940458e2f64 - CPZ) * (E + (L * ((-2.3025850929940458e2f64 - CPZ) * (E + ((-2.3025850929940458e2f64 - CPZ) * EG)))))));
                                    CQR
                                };
                                let CQT = (AC * CQS) - CQM;
                                CQO = CQT;
                            }
                            let CQP = SK * ((COY * (8.86226925452758e-1f64 * ((BF * CQO) / CPY))) * CPX);
                            CPS = CQP;
                        }
                        let CQU;
                        if CPT != 0.0 {
                            CQU = BP;
                        } else {
                            let CQW = (-BR) / staged[166];
                            let CQX = if (CQW.abs()) < EA { 1.0 } else { 0.0 };
                            oCQX = CQX;
                            let CRA;
                            if CQX != 0.0 {
                                let CQY = CQW.exp();
                                CRA = CQY;
                            } else {
                                let CQZ = if CQW < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCQZ = CQZ;
                                let CRF = if CQZ != 0.0 {
                                    let CRC = EH / (E + ((-2.3025850929940458e2f64 - CQW) * (E + (L * ((-2.3025850929940458e2f64 - CQW) * (E + ((-2.3025850929940458e2f64 - CQW) * EG)))))));
                                    CRC
                                } else {
                                    let CRD = CQW - EA;
                                    let CRE = EK * (E + (CRD * (E + (L * (CRD * (E + (CRD * EG)))))));
                                    CRE
                                };
                                CRA = CRF;
                            }
                            let CRB = SY * (staged[167] * CRA);
                            CQU = CRB;
                        }
                        let CQV = if (if CE > SR { 1.0 } else { 0.0 }) != 0.0 || staged[168] != 0.0 { 1.0 } else { 0.0 };
                        oCQV = CQV;
                        let CRI;
                        if CQV != 0.0 {
                            CRI = E;
                        } else {
                            let CRH = if CRG > ((-TE) * CE) { 1.0 } else { 0.0 };
                            oCRH = CRH;
                            let CRM;
                            if CRH != 0.0 {
                                let CRK = if TJ == HO { 1.0 } else { 0.0 };
                                oCRK = CRK;
                                let CRQ = if CRK != 0.0 {
                                    let CRN = (CRG * CD).abs();
                                    let CRO = ((CRN * CRN) * CRN) * CRN;
                                    CRO
                                } else {
                                    let CRP = ((CRG * CD).abs()).powf(TJ);
                                    CRP
                                };
                                let CRR = E / (E - CRQ);
                                CRM = CRR;
                            } else {
                                let CRL = TL + ((CRG + (TE * CE)) * CN);
                                CRM = CRL;
                            }
                            CRI = CRM;
                        }
                        let CRJ = (((COQ + COZ) + CPS) + CQU) * CRI;
                        COP = CRJ;
                    }
                    let CRS;
                    if EP != 0.0 {
                        CRS = BP;
                    } else {
                        let CRT = Z * CFH;
                        let CRY;
                        let CRZ;
                        let CSA;
                        let CSB;
                        let CSC;
                        if CRU != 0.0 {
                            CRY = BP;
                            CRZ = BP;
                            CSA = BP;
                            CSB = BP;
                            CSC = BP;
                        } else {
                            let CRV = AL - CFE;
                            let CRW = E - ((E - (CFF / CRV)).sqrt());
                            let CSF = if CRX != 0.0 {
                                BP
                            } else {
                                let CSE = ((((CRW * CRW) * (CRW.ln())) / (E - CRW)) + CRW) * staged[170];
                                CSE
                            };
                            let CSG = CRW + CSF;
                            let CSJ = if CRX != 0.0 {
                                let CSH = (CRV * UI).sqrt();
                                CSH
                            } else {
                                let CSI = (CRV * UI).powf(AS);
                                CSI
                            };
                            let CSK = UM * CSJ;
                            let CSL = P * ((CFG - E) * CSK);
                            let CSM = UP * (CSL * CSG);
                            CRY = CSK;
                            CRZ = CRV;
                            CSA = CSG;
                            CSB = CSL;
                            CSC = CSM;
                        }
                        let CSV;
                        if CSD != 0.0 {
                            CSV = BP;
                        } else {
                            let CSN = BJ * ((CRY * UR) / CRZ);
                            let CSO = (RA * BG) / CSN;
                            let CSP = CSO * CSO;
                            let CSQ = CSP * CSP;
                            let CSR = (CSQ / (CSQ + E)).sqrt();
                            let CSS = (CSR.abs()).sqrt();
                            let CST = CSR * CSS;
                            let CSZ = if CSU != 0.0 {
                                let CSX = E / (E + (CSN * CST));
                                CSX
                            } else {
                                let CSY = (E + (CSN * CST)).powf(staged[171]);
                                CSY
                            };
                            let CTA = (CSA * CSZ) / (CSA + CSZ);
                            let CTB = (RO * (CSN / CSS)).sqrt();
                            let CTC = (((BG * CSO) * CSS) - (BG * CSR)) + (L * (CSN * CST));
                            let CTD = (((AC * (CSO * CSS)) - CSR) - E) * CTB;
                            let CTE = CTD * CTD;
                            let CTF = if CTD > BP { 1.0 } else { 0.0 };
                            oCTF = CTF;
                            let CTI = if CTF != 0.0 {
                                let CTG = E / (E + (RU * CTD));
                                CTG
                            } else {
                                let CTH = E / (E - (RU * CTD));
                                CTH
                            };
                            let CTJ = (-CTE) + CTC;
                            let CTK = if CTJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oCTK = CTK;
                            let CTN = if CTK != 0.0 {
                                let CTL = CTJ.exp();
                                CTL
                            } else {
                                let CTM = EH / (E + ((-2.3025850929940458e2f64 - CTJ) * (E + (L * ((-2.3025850929940458e2f64 - CTJ) * (E + ((-2.3025850929940458e2f64 - CTJ) * EG)))))));
                                CTM
                            };
                            let CTO = CTI * CTI;
                            let CTP = (((SD * CTI) + (SF * CTO)) + (SG * (CTO * CTI))) * CTN;
                            let CTR;
                            if CTF != 0.0 {
                                CTR = CTP;
                            } else {
                                let CTQ = if CTC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCTQ = CTQ;
                                let CTV = if CTQ != 0.0 {
                                    let CTT = CTC.exp();
                                    CTT
                                } else {
                                    let CTU = EH / (E + ((-2.3025850929940458e2f64 - CTC) * (E + (L * ((-2.3025850929940458e2f64 - CTC) * (E + ((-2.3025850929940458e2f64 - CTC) * EG)))))));
                                    CTU
                                };
                                let CTW = (AC * CTV) - CTP;
                                CTR = CTW;
                            }
                            let CTS = VX * ((CSB * (8.86226925452758e-1f64 * ((BG * CTR) / CTB))) * CTA);
                            CSV = CTS;
                        }
                        let CTX;
                        if CSW != 0.0 {
                            CTX = BP;
                        } else {
                            let CTZ = (-BT) / staged[172];
                            let CUA = if (CTZ.abs()) < EA { 1.0 } else { 0.0 };
                            oCUA = CUA;
                            let CUD;
                            if CUA != 0.0 {
                                let CUB = CTZ.exp();
                                CUD = CUB;
                            } else {
                                let CUC = if CTZ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCUC = CUC;
                                let CUI = if CUC != 0.0 {
                                    let CUF = EH / (E + ((-2.3025850929940458e2f64 - CTZ) * (E + (L * ((-2.3025850929940458e2f64 - CTZ) * (E + ((-2.3025850929940458e2f64 - CTZ) * EG)))))));
                                    CUF
                                } else {
                                    let CUG = CTZ - EA;
                                    let CUH = EK * (E + (CUG * (E + (L * (CUG * (E + (CUG * EG)))))));
                                    CUH
                                };
                                CUD = CUI;
                            }
                            let CUE = WK * (staged[173] * CUD);
                            CTX = CUE;
                        }
                        let CTY = if (if CI > SR { 1.0 } else { 0.0 }) != 0.0 || staged[174] != 0.0 { 1.0 } else { 0.0 };
                        oCTY = CTY;
                        let CUK;
                        if CTY != 0.0 {
                            CUK = E;
                        } else {
                            let CUJ = if CRG > ((-TE) * CI) { 1.0 } else { 0.0 };
                            oCUJ = CUJ;
                            let CUO;
                            if CUJ != 0.0 {
                                let CUM = if WT == HO { 1.0 } else { 0.0 };
                                oCUM = CUM;
                                let CUS = if CUM != 0.0 {
                                    let CUP = (CRG * CH).abs();
                                    let CUQ = ((CUP * CUP) * CUP) * CUP;
                                    CUQ
                                } else {
                                    let CUR = ((CRG * CH).abs()).powf(WT);
                                    CUR
                                };
                                let CUT = E / (E - CUS);
                                CUO = CUT;
                            } else {
                                let CUN = WV + ((CRG + (TE * CI)) * CO);
                                CUO = CUN;
                            }
                            CUK = CUO;
                        }
                        let CUL = (((CRT + CSC) + CSV) + CTX) * CUK;
                        CRS = CUL;
                    }
                    let CUU;
                    if ES != 0.0 {
                        CUU = BP;
                    } else {
                        let CVA = AA * CFI;
                        let CVF;
                        let CVG;
                        let CVH;
                        let CVI;
                        let CVJ;
                        if CVB != 0.0 {
                            CVF = BP;
                            CVG = BP;
                            CVH = BP;
                            CVI = BP;
                            CVJ = BP;
                        } else {
                            let CVC = AM - CFE;
                            let CVD = E - ((E - (CFF / CVC)).sqrt());
                            let CVM = if CVE != 0.0 {
                                BP
                            } else {
                                let CVL = ((((CVD * CVD) * (CVD.ln())) / (E - CVD)) + CVD) * staged[175];
                                CVL
                            };
                            let CVN = CVD + CVM;
                            let CVQ = if CVE != 0.0 {
                                let CVO = (CVC * XT).sqrt();
                                CVO
                            } else {
                                let CVP = (CVC * XT).powf(AU);
                                CVP
                            };
                            let CVR = XX * CVQ;
                            let CVS = R * ((CFG - E) * CVR);
                            let CVT = YA * (CVS * CVN);
                            CVF = CVR;
                            CVG = CVC;
                            CVH = CVN;
                            CVI = CVS;
                            CVJ = CVT;
                        }
                        let CWC;
                        if CVK != 0.0 {
                            CWC = BP;
                        } else {
                            let CVU = BK * ((CVF * YC) / CVG);
                            let CVV = (RA * BH) / CVU;
                            let CVW = CVV * CVV;
                            let CVX = CVW * CVW;
                            let CVY = (CVX / (CVX + E)).sqrt();
                            let CVZ = (CVY.abs()).sqrt();
                            let CWA = CVY * CVZ;
                            let CWG = if CWB != 0.0 {
                                let CWE = E / (E + (CVU * CWA));
                                CWE
                            } else {
                                let CWF = (E + (CVU * CWA)).powf(staged[176]);
                                CWF
                            };
                            let CWH = (CVH * CWG) / (CVH + CWG);
                            let CWI = (RO * (CVU / CVZ)).sqrt();
                            let CWJ = (((BH * CVV) * CVZ) - (BH * CVY)) + (L * (CVU * CWA));
                            let CWK = (((AC * (CVV * CVZ)) - CVY) - E) * CWI;
                            let CWL = CWK * CWK;
                            let CWM = if CWK > BP { 1.0 } else { 0.0 };
                            oCWM = CWM;
                            let CWP = if CWM != 0.0 {
                                let CWN = E / (E + (RU * CWK));
                                CWN
                            } else {
                                let CWO = E / (E - (RU * CWK));
                                CWO
                            };
                            let CWQ = (-CWL) + CWJ;
                            let CWR = if CWQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oCWR = CWR;
                            let CWU = if CWR != 0.0 {
                                let CWS = CWQ.exp();
                                CWS
                            } else {
                                let CWT = EH / (E + ((-2.3025850929940458e2f64 - CWQ) * (E + (L * ((-2.3025850929940458e2f64 - CWQ) * (E + ((-2.3025850929940458e2f64 - CWQ) * EG)))))));
                                CWT
                            };
                            let CWV = CWP * CWP;
                            let CWW = (((SD * CWP) + (SF * CWV)) + (SG * (CWV * CWP))) * CWU;
                            let CWY;
                            if CWM != 0.0 {
                                CWY = CWW;
                            } else {
                                let CWX = if CWJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCWX = CWX;
                                let CXC = if CWX != 0.0 {
                                    let CXA = CWJ.exp();
                                    CXA
                                } else {
                                    let CXB = EH / (E + ((-2.3025850929940458e2f64 - CWJ) * (E + (L * ((-2.3025850929940458e2f64 - CWJ) * (E + ((-2.3025850929940458e2f64 - CWJ) * EG)))))));
                                    CXB
                                };
                                let CXD = (AC * CXC) - CWW;
                                CWY = CXD;
                            }
                            let CWZ = ZI * ((CVI * (8.86226925452758e-1f64 * ((BH * CWY) / CWI))) * CWH);
                            CWC = CWZ;
                        }
                        let CXE;
                        if CWD != 0.0 {
                            CXE = BP;
                        } else {
                            let CXG = (-BV) / staged[177];
                            let CXH = if (CXG.abs()) < EA { 1.0 } else { 0.0 };
                            oCXH = CXH;
                            let CXK;
                            if CXH != 0.0 {
                                let CXI = CXG.exp();
                                CXK = CXI;
                            } else {
                                let CXJ = if CXG < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oCXJ = CXJ;
                                let CXP = if CXJ != 0.0 {
                                    let CXM = EH / (E + ((-2.3025850929940458e2f64 - CXG) * (E + (L * ((-2.3025850929940458e2f64 - CXG) * (E + ((-2.3025850929940458e2f64 - CXG) * EG)))))));
                                    CXM
                                } else {
                                    let CXN = CXG - EA;
                                    let CXO = EK * (E + (CXN * (E + (L * (CXN * (E + (CXN * EG)))))));
                                    CXO
                                };
                                CXK = CXP;
                            }
                            let CXL = ZV * (staged[178] * CXK);
                            CXE = CXL;
                        }
                        let CXF = if (if CM > SR { 1.0 } else { 0.0 }) != 0.0 || staged[179] != 0.0 { 1.0 } else { 0.0 };
                        oCXF = CXF;
                        let CXR;
                        if CXF != 0.0 {
                            CXR = E;
                        } else {
                            let CXQ = if CRG > ((-TE) * CM) { 1.0 } else { 0.0 };
                            oCXQ = CXQ;
                            let CXV;
                            if CXQ != 0.0 {
                                let CXT = if AAE == HO { 1.0 } else { 0.0 };
                                oCXT = CXT;
                                let CXZ = if CXT != 0.0 {
                                    let CXW = (CRG * CL).abs();
                                    let CXX = ((CXW * CXW) * CXW) * CXW;
                                    CXX
                                } else {
                                    let CXY = ((CRG * CL).abs()).powf(AAE);
                                    CXY
                                };
                                let CYA = E / (E - CXZ);
                                CXV = CYA;
                            } else {
                                let CXU = AAG + ((CRG + (TE * CM)) * CP);
                                CXV = CXU;
                            }
                            CXR = CXV;
                        }
                        let CXS = (((CVA + CVJ) + CWC) + CXE) * CXR;
                        CUU = CXS;
                    }
                    let CUV = ((DH * COP) + (DO * CRS)) + (DT * CUU);
                    let CUW = BZ * F;
                    let CUY = CBZ - (FA * (((CUW * CUX).exp()) - E));
                    let CUZ = CUV - (FA * ((((CFB * F) * CUX).exp()) - E));
                    let CYC;
                    let CYD;
                    let CYE;
                    let CYF;
                    let CYG;
                    if FO != 0.0 {
                        let CYB = if (if CBZ > BP { 1.0 } else { 0.0 }) != 0.0 && (if CUV > BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oCYB = CYB;
                        let CYO;
                        let CYP;
                        if CYB != 0.0 {
                            let CYN = if (if (if (if (CUY / CBZ) > CYM { 1.0 } else { 0.0 }) != 0.0 || (if (CUZ / CUV) > CYM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CUY > BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CUZ > BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            oCYN = CYN;
                            let CYZ;
                            let CZA;
                            if CYN != 0.0 {
                                let CYX = (D * ((CUY / CUZ).ln())) / -1e-1f64;
                                let CYY = CUY / (((CUW * CYX).exp()) - E);
                                CYZ = CYY;
                                CZA = CYX;
                            } else {
                                CYZ = BP;
                                CZA = E;
                            }
                            CYO = CYZ;
                            CYP = CZA;
                        } else {
                            CYO = BP;
                            CYP = E;
                        }
                        let CYQ = GF * F;
                        let CYR = (XE - (FA * (((CYQ * CUX).exp()) - E))) - (CYO * (((CYQ * CYP).exp()) - E));
                        let CYS = AAO * F;
                        let CYT = (AQI - (FA * (((CYS * CUX).exp()) - E))) - (CYO * (((CYS * CYP).exp()) - E));
                        let CYU = ATK * F;
                        let CYV = (BJE - (FA * (((CYU * CUX).exp()) - E))) - (CYO * (((CYU * CYP).exp()) - E));
                        let CYW = if (if (if XE < BP { 1.0 } else { 0.0 }) != 0.0 && (if AQI < BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BJE < BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oCYW = CYW;
                        let CZC;
                        let CZD;
                        let CZE;
                        if CYW != 0.0 {
                            let CZB = if (if (if (if (if (if (CYR / XE) > CYM { 1.0 } else { 0.0 }) != 0.0 || (if (CYT / AQI) > CYM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (CYV / BJE) > CYM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CYR < BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CYT < BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CYV < BP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            oCZB = CZB;
                            let CZK;
                            let CZL;
                            let CZM;
                            if CZB != 0.0 {
                                let CZF = CYR / CYT;
                                let CZG = GF - AAO;
                                let CZH = AAO - GF;
                                let CZI = (((-D) * (CZF.ln())) / CZG) + (((D * (CZF - E)) * ((CZF.powf((AAO / CZH))) - E)) / ((((CZF.powf((GF / CZG))) * CZH) + (CZF * GF)) - AAO));
                                let CZJ = if ((CYU * CZI).abs()) < 1e-6f64 { 1.0 } else { 0.0 };
                                oCZJ = CZJ;
                                let CZQ;
                                let CZR;
                                let CZS;
                                if CZJ != 0.0 {
                                    let CZN = CYV * ((E / ATK) + ((L * F) * CZI));
                                    let CZO = (((-5e-1f64 * CYV) * CZI) * F) / ATK;
                                    CZQ = CZN;
                                    CZR = E;
                                    CZS = CZO;
                                } else {
                                    let CZP = (-CYV) / (((((-ATK) * F) * CZI).exp()) - E);
                                    CZQ = CZP;
                                    CZR = BP;
                                    CZS = CZI;
                                }
                                CZK = CZQ;
                                CZL = CZR;
                                CZM = CZS;
                            } else {
                                CZK = BP;
                                CZL = BP;
                                CZM = E;
                            }
                            CZC = CZK;
                            CZD = CZL;
                            CZE = CZM;
                        } else {
                            CZC = BP;
                            CZD = BP;
                            CZE = E;
                        }
                        CYC = CYO;
                        CYD = CZC;
                        CYE = CYP;
                        CYF = CZD;
                        CYG = CZE;
                    } else {
                        CYC = BP;
                        CYD = BP;
                        CYE = E;
                        CYF = BP;
                        CYG = E;
                    }
                    let CYH = DH * AR;
                    let CYI = DO * AT;
                    let CYJ = DT * AV;
                    let CYK = staged[181] * ((CYH + CYI) + CYJ);
                    let CYL = if CYH <= CYK { 1.0 } else { 0.0 };
                    oCYL = CYL;
                    let CZT = if CYL != 0.0 {
                        BP
                    } else {
                        E
                    };
                    let CZU = if CYI <= CYK { 1.0 } else { 0.0 };
                    oCZU = CZU;
                    let CZV = if CZU != 0.0 {
                        BP
                    } else {
                        E
                    };
                    let CZW = if CYJ <= CYK { 1.0 } else { 0.0 };
                    oCZW = CZW;
                    let CZX = if CZW != 0.0 {
                        BP
                    } else {
                        E
                    };
                    let DAD;
                    let DAE;
                    let DAF;
                    if FO != 0.0 {
                        let DAA = (CZZ / (FA + CZY)).ln();
                        let DAB = (CZZ / (CYC + CZY)).ln();
                        let DAC = (CZZ / ((CYD.abs()) + CZY)).ln();
                        DAD = DAA;
                        DAE = DAB;
                        DAF = DAC;
                    } else {
                        DAD = BP;
                        DAE = BP;
                        DAF = BP;
                    }
                    let DAG = if DAD <= EA { DAD } else { EA };
                    let DAH = DAG.exp();
                    let DAI = if DAE <= EA { DAE } else { EA };
                    let DAJ = DAI.exp();
                    let DAK = if DAF <= EA { DAF } else { EA };
                    let DAL = DAK.exp();
                    FP = DAG;
                    FQ = DAH;
                    FR = FA;
                    FS = CYE;
                    FT = DAI;
                    FU = DAJ;
                    FV = CYC;
                    FW = CYF;
                    FX = CYD;
                    FY = CYG;
                    FZ = DAK;
                    GA = DAL;
                    GB = CZT;
                    GC = CZV;
                    GD = CZX;
                    GE = CFD;
                } else {
                    FP = BP;
                    FQ = BP;
                    FR = BP;
                    FS = E;
                    FT = BP;
                    FU = BP;
                    FV = BP;
                    FW = BP;
                    FX = BP;
                    FY = E;
                    FZ = BP;
                    GA = BP;
                    GB = E;
                    GC = E;
                    GD = E;
                    GE = BP;
                }
                if FN != 0.0 {
                    let DAN = if FW > BP { 1.0 } else { 0.0 };
                    oDAN = DAN;
                    if DAN != 0.0 {
                    } else {
                        let DAR = -FX;
                        oDAR = DAR;
                    }
                    let DAO = (HO * EW) * EW;
                    oDAO = DAO;
                    let DAP = EW * (EW / EX);
                    oDAP = DAP;
                    let DAQ = if GB > L { 1.0 } else { 0.0 };
                    oDAQ = DAQ;
                    if DAQ != 0.0 {
                        let DAS = if QY == L { 1.0 } else { 0.0 };
                        oDAS = DAS;
                        if DAS != 0.0 {
                        } else {
                            let DAV = QY - DAU;
                            oDAV = DAV;
                        }
                    } else {
                    }
                    let DAT = if GC > L { 1.0 } else { 0.0 };
                    oDAT = DAT;
                    if DAT != 0.0 {
                        let DAW = if UR == L { 1.0 } else { 0.0 };
                        oDAW = DAW;
                        if DAW != 0.0 {
                        } else {
                            let DAY = UR - DAU;
                            oDAY = DAY;
                        }
                    } else {
                    }
                    let DAX = if GD > L { 1.0 } else { 0.0 };
                    oDAX = DAX;
                    if DAX != 0.0 {
                        let DAZ = if YC == L { 1.0 } else { 0.0 };
                        oDAZ = DAZ;
                        if DAZ != 0.0 {
                        } else {
                            let DBA = YC - DAU;
                            oDBA = DBA;
                        }
                    } else {
                    }
                } else {
                    if DAM != 0.0 {
                        let DBB = (HO * EW) * EW;
                        oDBB = DBB;
                        let DBC = EW * (EW / EX);
                        oDBC = DBC;
                        let DBD = (HO * D) * D;
                        oDBD = DBD;
                    } else {
                    }
                    if EF != 0.0 {
                    } else {
                        if DBE != 0.0 {
                        } else {
                            let DBF = RA * BF;
                            oDBF = DBF;
                        }
                        if DBG != 0.0 {
                        } else {
                            let DBI = -BR;
                            oDBI = DBI;
                        }
                        let DBH = if (if CE > SR { 1.0 } else { 0.0 }) != 0.0 || staged[228] != 0.0 { 1.0 } else { 0.0 };
                        oDBH = DBH;
                        if DBH != 0.0 {
                        } else {
                            let DBJ = (-TE) * CE;
                            oDBJ = DBJ;
                        }
                    }
                    if EP != 0.0 {
                    } else {
                        if DBK != 0.0 {
                        } else {
                            let DBL = RA * BG;
                            oDBL = DBL;
                        }
                        if DBM != 0.0 {
                        } else {
                            let DBO = -BT;
                            oDBO = DBO;
                        }
                        let DBN = if (if CI > SR { 1.0 } else { 0.0 }) != 0.0 || staged[242] != 0.0 { 1.0 } else { 0.0 };
                        oDBN = DBN;
                        if DBN != 0.0 {
                        } else {
                            let DBP = (-TE) * CI;
                            oDBP = DBP;
                        }
                    }
                    if ES != 0.0 {
                    } else {
                        if DBQ != 0.0 {
                        } else {
                            let DBR = RA * BH;
                            oDBR = DBR;
                        }
                        if DBS != 0.0 {
                        } else {
                            let DBU = -BV;
                            oDBU = DBU;
                        }
                        let DBT = if (if CM > SR { 1.0 } else { 0.0 }) != 0.0 || staged[256] != 0.0 { 1.0 } else { 0.0 };
                        oDBT = DBT;
                        if DBT != 0.0 {
                        } else {
                            let DBV = (-TE) * CM;
                            oDBV = DBV;
                        }
                    }
                }
                if DBW != 0.0 {
                    let DCB;
                    if DBX != 0.0 {
                        let DCA = S - (HK * DG);
                        let DCD = (GR - DCA) - HM;
                        let DCE = ((GR - (L * (DCD + (((DCD * DCD) + staged[268]).sqrt())))) - S) - HM;
                        let DCF = S + (L * (DCE + (((DCE * DCE) + staged[269]).sqrt())));
                        DCB = DCF;
                    } else {
                        DCB = S;
                    }
                    oDCB = DCB;
                    let DCC = DG - DF;
                    oDCC = DCC;
                    let DCG = (staged[282] / DC).exp();
                    oDCG = DCG;
                } else {
                }
                let DBY = if FM >= parameters[4] { 1.0 } else { 0.0 };
                let DBZ = if (if FM > BP { 1.0 } else { 0.0 }) != 0.0 && DBY != 0.0 { 1.0 } else { 0.0 };
                let DCI = if DBZ != 0.0 {
                    BP
                } else {
                    DCH
                };
            [A, D, F, N, P, R, Y, Z, AA, AK, AL, AM, AN, AO, AP, AW, AX, AY, AZ, BA, BB, BF, BG, BH, BI, BJ, BK, BQ, BS, BU, CA, CF, CJ, CD, CN, CH, CO, CL, CP, CW, CY, DC, DF, DG, DJ, DQ, DV, DY, EB, oED, EV, EX, EY, EZ, FB, FE, FH, FK, oGG, oGO, oHD, oHF, oHQ, oHZ, oHU, oIL, oIP, oIX, oJG, oJB, oJS, oJW, oKE, oKN, oKI, oKZ, EE, oGS, oLI, oLV, oLO, oMK, oMO, oMW, oNJ, oNC, oNY, oOC, oOK, oOX, oOQ, oPM, oRT, oRZ, oSI, oSU, oSW, CE, oSS, oTG, oTK, oVK, oVP, oVV, oWG, oWI, CI, oWE, oWQ, oWU, oYV, oZA, oZG, oZR, oZT, CM, oZP, oAAB, oAAF, oAAP, oAAX, oABL, oABN, oABV, oACE, oABZ, oACQ, oACT, oADB, oADK, oADF, oADW, oADZ, oAEH, oAEQ, oAEL, oAFC, oABA, oAFL, oAFY, oAFR, oAGN, oAGR, oAGZ, oAHM, oAHF, oAIB, oAIF, oAIN, oAJA, oAIT, oAJP, oALP, oALU, oAMA, oAMK, oAMM, oAMI, oAMU, oAMX, oAOS, oAOX, oAPD, oAPN, oAPP, oAPL, oAPW, oAPZ, oARV, oASA, oASG, oASQ, oASS, oASO, oASZ, oATC, oATL, oATT, oAUH, oAUJ, oAUR, oAVA, oAUV, oAVM, oAVP, oAVX, oAWG, oAWB, oAWS, oAWV, oAXD, oAXM, oAXH, oAXY, oATW, oAYH, oAYU, oAYN, oAZJ, oAZN, oAZV, oBAI, oBAB, oBAX, oBBB, oBBJ, oBBW, oBBP, oBCL, oBEL, oBEQ, oBEW, oBFG, oBFI, oBFE, oBFQ, oBFT, oBHO, oBHT, oBHZ, oBIJ, oBIL, oBIH, oBIS, oBIV, oBKR, oBKW, oBLC, oBLM, oBLO, oBLK, oBLV, oBLY, oBMG, oBMO, oBNC, oBNE, oBNM, oBNV, oBNQ, oBOH, oBOK, oBOS, oBPB, oBOW, oBPN, oBPQ, oBPY, oBQH, oBQC, oBQT, oBMR, oBRC, oBRP, oBRI, oBSE, oBSI, oBSQ, oBTD, oBSW, oBTS, oBTW, oBUE, oBUR, oBUK, oBVG, oBXG, oBXL, oBXR, oBYB, oBYD, oBXZ, oBYL, oBYO, oCAJ, oCAO, oCAU, oCBE, oCBG, oCBC, oCBN, oCBQ, oCDM, oCDR, oCDX, oCEH, oCEJ, oCEF, oCEQ, oCET, oCFC, oCFK, oCFY, oCGA, oCGI, oCGR, oCGM, oCHD, oCHG, oCHO, oCHX, oCHS, oCIJ, oCIM, oCIU, oCJD, oCIY, oCJP, oCFN, oCJY, oCKL, oCKE, oCLA, oCLE, oCLM, oCLZ, oCLS, oCMO, oCMS, oCNA, oCNN, oCNG, oCOC, oCQC, oCQH, oCQN, oCQX, oCQZ, oCQV, oCRH, oCRK, oCTF, oCTK, oCTQ, oCUA, oCUC, oCTY, oCUJ, oCUM, oCWM, oCWR, oCWX, oCXH, oCXJ, oCXF, oCXQ, oCXT, oCYB, oCYN, oCYW, oCZB, oCZJ, oCYL, oCZU, oCZW, FP, FQ, FR, FS, FT, FU, FV, oDAN, FX, FY, FZ, GA, oDAR, oDAO, oDAP, oDAQ, oDAS, oDAT, oDAW, oDAX, oDAZ, oDBB, oDBC, oDBD, oDBF, oDBI, oDBH, oDBJ, oDBL, oDBO, oDBN, oDBP, oDBR, oDBU, oDBT, oDBV, GE, oDCC, oDCB, oDCG, FM, DBY, DBZ, DCI, oDAV, oDAY, oDBA]
        };
        self.canonical_staged[273] = produced[0];
        self.canonical_staged[214] = produced[1];
        self.canonical_staged[183] = produced[2];
        self.canonical_staged[222] = produced[3];
        self.canonical_staged[236] = produced[4];
        self.canonical_staged[250] = produced[5];
        self.canonical_staged[219] = produced[6];
        self.canonical_staged[233] = produced[7];
        self.canonical_staged[247] = produced[8];
        self.canonical_staged[220] = produced[9];
        self.canonical_staged[234] = produced[10];
        self.canonical_staged[248] = produced[11];
        self.canonical_staged[199] = produced[12];
        self.canonical_staged[202] = produced[13];
        self.canonical_staged[205] = produced[14];
        self.canonical_staged[200] = produced[15];
        self.canonical_staged[203] = produced[16];
        self.canonical_staged[206] = produced[17];
        self.canonical_staged[201] = produced[18];
        self.canonical_staged[204] = produced[19];
        self.canonical_staged[207] = produced[20];
        self.canonical_staged[226] = produced[21];
        self.canonical_staged[240] = produced[22];
        self.canonical_staged[254] = produced[23];
        self.canonical_staged[223] = produced[24];
        self.canonical_staged[237] = produced[25];
        self.canonical_staged[251] = produced[26];
        self.canonical_staged[368] = produced[27];
        self.canonical_staged[369] = produced[28];
        self.canonical_staged[370] = produced[29];
        self.canonical_staged[371] = produced[30];
        self.canonical_staged[372] = produced[31];
        self.canonical_staged[373] = produced[32];
        self.canonical_staged[230] = produced[33];
        self.canonical_staged[232] = produced[34];
        self.canonical_staged[244] = produced[35];
        self.canonical_staged[246] = produced[36];
        self.canonical_staged[258] = produced[37];
        self.canonical_staged[260] = produced[38];
        self.canonical_staged[212] = produced[39];
        self.canonical_staged[274] = produced[40];
        self.canonical_staged[281] = produced[41];
        self.canonical_staged[211] = produced[42];
        self.canonical_staged[265] = produced[43];
        self.canonical_staged[377] = produced[44];
        self.canonical_staged[378] = produced[45];
        self.canonical_staged[379] = produced[46];
        self.canonical_staged[210] = produced[47];
        self.canonical_staged[380] = produced[48];
        self.canonical_staged[381] = produced[49];
        self.canonical_staged[215] = produced[50];
        self.canonical_staged[197] = produced[51];
        self.canonical_staged[391] = produced[52];
        self.canonical_staged[392] = produced[53];
        self.canonical_staged[393] = produced[54];
        self.canonical_staged[394] = produced[55];
        self.canonical_staged[395] = produced[56];
        self.canonical_staged[396] = produced[57];
        self.canonical_staged[406] = produced[58];
        self.canonical_staged[407] = produced[59];
        self.canonical_staged[410] = produced[60];
        self.canonical_staged[411] = produced[61];
        self.canonical_staged[412] = produced[62];
        self.canonical_staged[414] = produced[63];
        self.canonical_staged[413] = produced[64];
        self.canonical_staged[415] = produced[65];
        self.canonical_staged[416] = produced[66];
        self.canonical_staged[417] = produced[67];
        self.canonical_staged[419] = produced[68];
        self.canonical_staged[418] = produced[69];
        self.canonical_staged[420] = produced[70];
        self.canonical_staged[421] = produced[71];
        self.canonical_staged[422] = produced[72];
        self.canonical_staged[424] = produced[73];
        self.canonical_staged[423] = produced[74];
        self.canonical_staged[425] = produced[75];
        self.canonical_staged[213] = produced[76];
        self.canonical_staged[408] = produced[77];
        self.canonical_staged[426] = produced[78];
        self.canonical_staged[428] = produced[79];
        self.canonical_staged[427] = produced[80];
        self.canonical_staged[429] = produced[81];
        self.canonical_staged[430] = produced[82];
        self.canonical_staged[431] = produced[83];
        self.canonical_staged[433] = produced[84];
        self.canonical_staged[432] = produced[85];
        self.canonical_staged[434] = produced[86];
        self.canonical_staged[435] = produced[87];
        self.canonical_staged[436] = produced[88];
        self.canonical_staged[438] = produced[89];
        self.canonical_staged[437] = produced[90];
        self.canonical_staged[439] = produced[91];
        self.canonical_staged[446] = produced[92];
        self.canonical_staged[447] = produced[93];
        self.canonical_staged[448] = produced[94];
        self.canonical_staged[451] = produced[95];
        self.canonical_staged[452] = produced[96];
        self.canonical_staged[231] = produced[97];
        self.canonical_staged[450] = produced[98];
        self.canonical_staged[453] = produced[99];
        self.canonical_staged[454] = produced[100];
        self.canonical_staged[461] = produced[101];
        self.canonical_staged[462] = produced[102];
        self.canonical_staged[463] = produced[103];
        self.canonical_staged[466] = produced[104];
        self.canonical_staged[467] = produced[105];
        self.canonical_staged[245] = produced[106];
        self.canonical_staged[465] = produced[107];
        self.canonical_staged[468] = produced[108];
        self.canonical_staged[469] = produced[109];
        self.canonical_staged[476] = produced[110];
        self.canonical_staged[477] = produced[111];
        self.canonical_staged[478] = produced[112];
        self.canonical_staged[481] = produced[113];
        self.canonical_staged[482] = produced[114];
        self.canonical_staged[259] = produced[115];
        self.canonical_staged[480] = produced[116];
        self.canonical_staged[483] = produced[117];
        self.canonical_staged[484] = produced[118];
        self.canonical_staged[485] = produced[119];
        self.canonical_staged[486] = produced[120];
        self.canonical_staged[489] = produced[121];
        self.canonical_staged[490] = produced[122];
        self.canonical_staged[491] = produced[123];
        self.canonical_staged[493] = produced[124];
        self.canonical_staged[492] = produced[125];
        self.canonical_staged[494] = produced[126];
        self.canonical_staged[495] = produced[127];
        self.canonical_staged[496] = produced[128];
        self.canonical_staged[498] = produced[129];
        self.canonical_staged[497] = produced[130];
        self.canonical_staged[499] = produced[131];
        self.canonical_staged[500] = produced[132];
        self.canonical_staged[501] = produced[133];
        self.canonical_staged[503] = produced[134];
        self.canonical_staged[502] = produced[135];
        self.canonical_staged[504] = produced[136];
        self.canonical_staged[487] = produced[137];
        self.canonical_staged[505] = produced[138];
        self.canonical_staged[507] = produced[139];
        self.canonical_staged[506] = produced[140];
        self.canonical_staged[508] = produced[141];
        self.canonical_staged[509] = produced[142];
        self.canonical_staged[510] = produced[143];
        self.canonical_staged[512] = produced[144];
        self.canonical_staged[511] = produced[145];
        self.canonical_staged[513] = produced[146];
        self.canonical_staged[514] = produced[147];
        self.canonical_staged[515] = produced[148];
        self.canonical_staged[517] = produced[149];
        self.canonical_staged[516] = produced[150];
        self.canonical_staged[518] = produced[151];
        self.canonical_staged[525] = produced[152];
        self.canonical_staged[526] = produced[153];
        self.canonical_staged[527] = produced[154];
        self.canonical_staged[530] = produced[155];
        self.canonical_staged[531] = produced[156];
        self.canonical_staged[529] = produced[157];
        self.canonical_staged[532] = produced[158];
        self.canonical_staged[533] = produced[159];
        self.canonical_staged[540] = produced[160];
        self.canonical_staged[541] = produced[161];
        self.canonical_staged[542] = produced[162];
        self.canonical_staged[545] = produced[163];
        self.canonical_staged[546] = produced[164];
        self.canonical_staged[544] = produced[165];
        self.canonical_staged[547] = produced[166];
        self.canonical_staged[548] = produced[167];
        self.canonical_staged[555] = produced[168];
        self.canonical_staged[556] = produced[169];
        self.canonical_staged[557] = produced[170];
        self.canonical_staged[560] = produced[171];
        self.canonical_staged[561] = produced[172];
        self.canonical_staged[559] = produced[173];
        self.canonical_staged[562] = produced[174];
        self.canonical_staged[563] = produced[175];
        self.canonical_staged[564] = produced[176];
        self.canonical_staged[565] = produced[177];
        self.canonical_staged[568] = produced[178];
        self.canonical_staged[569] = produced[179];
        self.canonical_staged[570] = produced[180];
        self.canonical_staged[572] = produced[181];
        self.canonical_staged[571] = produced[182];
        self.canonical_staged[573] = produced[183];
        self.canonical_staged[574] = produced[184];
        self.canonical_staged[575] = produced[185];
        self.canonical_staged[577] = produced[186];
        self.canonical_staged[576] = produced[187];
        self.canonical_staged[578] = produced[188];
        self.canonical_staged[579] = produced[189];
        self.canonical_staged[580] = produced[190];
        self.canonical_staged[582] = produced[191];
        self.canonical_staged[581] = produced[192];
        self.canonical_staged[583] = produced[193];
        self.canonical_staged[566] = produced[194];
        self.canonical_staged[584] = produced[195];
        self.canonical_staged[586] = produced[196];
        self.canonical_staged[585] = produced[197];
        self.canonical_staged[587] = produced[198];
        self.canonical_staged[588] = produced[199];
        self.canonical_staged[589] = produced[200];
        self.canonical_staged[591] = produced[201];
        self.canonical_staged[590] = produced[202];
        self.canonical_staged[592] = produced[203];
        self.canonical_staged[593] = produced[204];
        self.canonical_staged[594] = produced[205];
        self.canonical_staged[596] = produced[206];
        self.canonical_staged[595] = produced[207];
        self.canonical_staged[597] = produced[208];
        self.canonical_staged[604] = produced[209];
        self.canonical_staged[605] = produced[210];
        self.canonical_staged[606] = produced[211];
        self.canonical_staged[609] = produced[212];
        self.canonical_staged[610] = produced[213];
        self.canonical_staged[608] = produced[214];
        self.canonical_staged[611] = produced[215];
        self.canonical_staged[612] = produced[216];
        self.canonical_staged[619] = produced[217];
        self.canonical_staged[620] = produced[218];
        self.canonical_staged[621] = produced[219];
        self.canonical_staged[624] = produced[220];
        self.canonical_staged[625] = produced[221];
        self.canonical_staged[623] = produced[222];
        self.canonical_staged[626] = produced[223];
        self.canonical_staged[627] = produced[224];
        self.canonical_staged[634] = produced[225];
        self.canonical_staged[635] = produced[226];
        self.canonical_staged[636] = produced[227];
        self.canonical_staged[639] = produced[228];
        self.canonical_staged[640] = produced[229];
        self.canonical_staged[638] = produced[230];
        self.canonical_staged[641] = produced[231];
        self.canonical_staged[642] = produced[232];
        self.canonical_staged[643] = produced[233];
        self.canonical_staged[644] = produced[234];
        self.canonical_staged[646] = produced[235];
        self.canonical_staged[647] = produced[236];
        self.canonical_staged[648] = produced[237];
        self.canonical_staged[650] = produced[238];
        self.canonical_staged[649] = produced[239];
        self.canonical_staged[651] = produced[240];
        self.canonical_staged[652] = produced[241];
        self.canonical_staged[653] = produced[242];
        self.canonical_staged[655] = produced[243];
        self.canonical_staged[654] = produced[244];
        self.canonical_staged[656] = produced[245];
        self.canonical_staged[657] = produced[246];
        self.canonical_staged[658] = produced[247];
        self.canonical_staged[660] = produced[248];
        self.canonical_staged[659] = produced[249];
        self.canonical_staged[661] = produced[250];
        self.canonical_staged[645] = produced[251];
        self.canonical_staged[662] = produced[252];
        self.canonical_staged[664] = produced[253];
        self.canonical_staged[663] = produced[254];
        self.canonical_staged[665] = produced[255];
        self.canonical_staged[666] = produced[256];
        self.canonical_staged[667] = produced[257];
        self.canonical_staged[669] = produced[258];
        self.canonical_staged[668] = produced[259];
        self.canonical_staged[670] = produced[260];
        self.canonical_staged[671] = produced[261];
        self.canonical_staged[672] = produced[262];
        self.canonical_staged[674] = produced[263];
        self.canonical_staged[673] = produced[264];
        self.canonical_staged[675] = produced[265];
        self.canonical_staged[682] = produced[266];
        self.canonical_staged[683] = produced[267];
        self.canonical_staged[684] = produced[268];
        self.canonical_staged[687] = produced[269];
        self.canonical_staged[688] = produced[270];
        self.canonical_staged[686] = produced[271];
        self.canonical_staged[689] = produced[272];
        self.canonical_staged[690] = produced[273];
        self.canonical_staged[697] = produced[274];
        self.canonical_staged[698] = produced[275];
        self.canonical_staged[699] = produced[276];
        self.canonical_staged[702] = produced[277];
        self.canonical_staged[703] = produced[278];
        self.canonical_staged[701] = produced[279];
        self.canonical_staged[704] = produced[280];
        self.canonical_staged[705] = produced[281];
        self.canonical_staged[712] = produced[282];
        self.canonical_staged[713] = produced[283];
        self.canonical_staged[714] = produced[284];
        self.canonical_staged[717] = produced[285];
        self.canonical_staged[718] = produced[286];
        self.canonical_staged[716] = produced[287];
        self.canonical_staged[719] = produced[288];
        self.canonical_staged[720] = produced[289];
        self.canonical_staged[721] = produced[290];
        self.canonical_staged[722] = produced[291];
        self.canonical_staged[724] = produced[292];
        self.canonical_staged[725] = produced[293];
        self.canonical_staged[726] = produced[294];
        self.canonical_staged[728] = produced[295];
        self.canonical_staged[727] = produced[296];
        self.canonical_staged[729] = produced[297];
        self.canonical_staged[730] = produced[298];
        self.canonical_staged[731] = produced[299];
        self.canonical_staged[733] = produced[300];
        self.canonical_staged[732] = produced[301];
        self.canonical_staged[734] = produced[302];
        self.canonical_staged[735] = produced[303];
        self.canonical_staged[736] = produced[304];
        self.canonical_staged[738] = produced[305];
        self.canonical_staged[737] = produced[306];
        self.canonical_staged[739] = produced[307];
        self.canonical_staged[723] = produced[308];
        self.canonical_staged[740] = produced[309];
        self.canonical_staged[742] = produced[310];
        self.canonical_staged[741] = produced[311];
        self.canonical_staged[743] = produced[312];
        self.canonical_staged[744] = produced[313];
        self.canonical_staged[745] = produced[314];
        self.canonical_staged[747] = produced[315];
        self.canonical_staged[746] = produced[316];
        self.canonical_staged[748] = produced[317];
        self.canonical_staged[749] = produced[318];
        self.canonical_staged[750] = produced[319];
        self.canonical_staged[752] = produced[320];
        self.canonical_staged[751] = produced[321];
        self.canonical_staged[753] = produced[322];
        self.canonical_staged[760] = produced[323];
        self.canonical_staged[761] = produced[324];
        self.canonical_staged[762] = produced[325];
        self.canonical_staged[765] = produced[326];
        self.canonical_staged[766] = produced[327];
        self.canonical_staged[764] = produced[328];
        self.canonical_staged[767] = produced[329];
        self.canonical_staged[768] = produced[330];
        self.canonical_staged[775] = produced[331];
        self.canonical_staged[776] = produced[332];
        self.canonical_staged[777] = produced[333];
        self.canonical_staged[780] = produced[334];
        self.canonical_staged[781] = produced[335];
        self.canonical_staged[779] = produced[336];
        self.canonical_staged[782] = produced[337];
        self.canonical_staged[783] = produced[338];
        self.canonical_staged[790] = produced[339];
        self.canonical_staged[791] = produced[340];
        self.canonical_staged[792] = produced[341];
        self.canonical_staged[795] = produced[342];
        self.canonical_staged[796] = produced[343];
        self.canonical_staged[794] = produced[344];
        self.canonical_staged[797] = produced[345];
        self.canonical_staged[798] = produced[346];
        self.canonical_staged[799] = produced[347];
        self.canonical_staged[801] = produced[348];
        self.canonical_staged[802] = produced[349];
        self.canonical_staged[803] = produced[350];
        self.canonical_staged[804] = produced[351];
        self.canonical_staged[800] = produced[352];
        self.canonical_staged[805] = produced[353];
        self.canonical_staged[806] = produced[354];
        self.canonical_staged[184] = produced[355];
        self.canonical_staged[185] = produced[356];
        self.canonical_staged[186] = produced[357];
        self.canonical_staged[187] = produced[358];
        self.canonical_staged[188] = produced[359];
        self.canonical_staged[189] = produced[360];
        self.canonical_staged[190] = produced[361];
        self.canonical_staged[808] = produced[362];
        self.canonical_staged[192] = produced[363];
        self.canonical_staged[191] = produced[364];
        self.canonical_staged[193] = produced[365];
        self.canonical_staged[194] = produced[366];
        self.canonical_staged[195] = produced[367];
        self.canonical_staged[198] = produced[368];
        self.canonical_staged[196] = produced[369];
        self.canonical_staged[809] = produced[370];
        self.canonical_staged[810] = produced[371];
        self.canonical_staged[811] = produced[372];
        self.canonical_staged[812] = produced[373];
        self.canonical_staged[813] = produced[374];
        self.canonical_staged[814] = produced[375];
        self.canonical_staged[209] = produced[376];
        self.canonical_staged[208] = produced[377];
        self.canonical_staged[216] = produced[378];
        self.canonical_staged[224] = produced[379];
        self.canonical_staged[227] = produced[380];
        self.canonical_staged[823] = produced[381];
        self.canonical_staged[229] = produced[382];
        self.canonical_staged[238] = produced[383];
        self.canonical_staged[241] = produced[384];
        self.canonical_staged[831] = produced[385];
        self.canonical_staged[243] = produced[386];
        self.canonical_staged[252] = produced[387];
        self.canonical_staged[255] = produced[388];
        self.canonical_staged[839] = produced[389];
        self.canonical_staged[257] = produced[390];
        self.canonical_staged[815] = produced[391];
        self.canonical_staged[270] = produced[392];
        self.canonical_staged[271] = produced[393];
        self.canonical_staged[283] = produced[394];
        self.canonical_staged[285] = produced[395];
        self.canonical_staged[849] = produced[396];
        self.canonical_staged[842] = produced[397];
        self.canonical_staged[850] = produced[398];
        self.canonical_staged[299] = produced[399];
        self.canonical_staged[300] = produced[400];
        self.canonical_staged[301] = produced[401];
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
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 80339 => 0usize, 80345 => 1usize, 80360 => 2usize, 80365 => 3usize, 80371 => 4usize, _ => usize::MAX };
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
            let A = staged[382];
            let B = staged[383];
            let C = staged[384];
            let D = staged[401];
            let E = node_potentials[0];
            let F = node_potentials[2];
            let H = 1e0f64;
            let I = 1e0f64;
            let K = staged[183];
            let N = staged[180];
            let R = staged[807];
            let AE = staged[43];
            let AF = staged[45];
            let AG = staged[46];
            let AJ = node_potentials[1];
            let AL = 1e0f64;
            let AP = -1e0f64;
            let AQ = 1e0f64;
            let AS = 1e-100f64;
            let AV = staged[184];
            let AZ = staged[186];
            let BC = staged[187];
            let BG = staged[185];
            let BQ = staged[188];
            let BU = staged[190];
            let BX = staged[808];
            let BY = staged[189];
            let CF = staged[191];
            let CT = staged[197];
            let CX = 2e0f64;
            let CY = 1e0f64;
            let DB = 2e0f64;
            let DE = staged[809];
            let DI = staged[193];
            let DM = staged[195];
            let DP = staged[194];
            let DW = staged[810];
            let DX = 0e0f64;
            let DY = Lanes([0e0f64; 2]);
            let EB = staged[811];
            let EC = staged[199];
            let EG = staged[68];
            let EL = staged[200];
            let EM = staged[201];
            let EP = staged[812];
            let ES = staged[813];
            let ET = staged[202];
            let EX = staged[84];
            let FC = staged[203];
            let FD = staged[204];
            let FG = staged[814];
            let FJ = staged[815];
            let FK = staged[205];
            let FO = staged[98];
            let FT = staged[206];
            let FU = staged[207];
            let GG = staged[210];
            let HA = 5e-1f64;
            let HD = 2.3025850929940458e2f64;
            let HG = staged[213];
            let HJ = staged[6];
            let HK = parameters[85];
            let IJ = 3.333333333333333e-1f64;
            let IS = 1e100f64;
            let IX = staged[211];
            let IY = parameters[86];
            let JB = 1e-2f64;
            let JD = 4e0f64;
            let KO = staged[212];
            let KP = staged[64];
            let KQ = staged[8];
            let MV = staged[65];
            let MW = staged[10];
            let UL = 3e0f64;
            let UP = staged[214];
            let VH = staged[217];
            let VR = staged[816];
            let WH = staged[219];
            let WK = staged[817];
            let WR = staged[818];
            let XC = staged[819];
            let XI = staged[221];
            let XP = staged[72];
            let XT = staged[17];
            let XY = staged[73];
            let YC = staged[222];
            let YF = staged[69];
            let YJ = staged[223];
            let YX = 0e0f64;
            let ZC = staged[820];
            let ZF = staged[821];
            let ZK = staged[225];
            let ZT = 3.75e-1f64;
            let ZW = staged[226];
            let AAH = 5.178164370971076e-1f64;
            let ABF = 2.9214664e-1f64;
            let ABJ = 2.6992878119627894e-1f64;
            let ABK = 4.3792457880372104e-1f64;
            let ABS = 8.86226925452758e-1f64;
            let ABV = staged[70];
            let ACM = staged[822];
            let ACP = staged[823];
            let ACQ = staged[14];
            let ACY = staged[76];
            let ADA = staged[23];
            let ADN = staged[75];
            let AEO = staged[82];
            let AEQ = staged[80];
            let AER = staged[232];
            let AEW = staged[230];
            let AFO = staged[824];
            let AGE = staged[233];
            let AGH = staged[825];
            let AGO = staged[826];
            let AGZ = staged[827];
            let AHF = staged[235];
            let AHM = staged[88];
            let AHQ = staged[19];
            let AHV = staged[89];
            let AHZ = staged[236];
            let AIC = staged[85];
            let AIG = staged[237];
            let AIY = staged[828];
            let AJB = staged[829];
            let AJG = staged[239];
            let AJR = staged[240];
            let ALJ = 8.86226925452758e-1f64;
            let ALM = staged[86];
            let AMD = staged[830];
            let AMG = staged[831];
            let AMH = staged[15];
            let AMP = staged[92];
            let AMR = staged[24];
            let ANE = staged[91];
            let AOF = staged[96];
            let AOH = staged[246];
            let AOM = staged[244];
            let APE = staged[832];
            let APY = staged[247];
            let AQB = staged[833];
            let AQI = staged[834];
            let AQT = staged[835];
            let AQZ = staged[249];
            let ARG = staged[102];
            let ARK = staged[21];
            let ARP = staged[103];
            let ART = staged[250];
            let ARW = staged[99];
            let ASA = staged[251];
            let ASS = staged[836];
            let ASV = staged[837];
            let ATA = staged[253];
            let ATL = staged[254];
            let AVD = 8.86226925452758e-1f64;
            let AVG = staged[100];
            let AVX = staged[838];
            let AWA = staged[839];
            let AWB = staged[16];
            let AWJ = staged[106];
            let AWL = staged[25];
            let AWY = staged[105];
            let AXZ = staged[110];
            let AYB = staged[260];
            let AYG = staged[258];
            let AYZ = staged[840];
            let AZA = staged[841];
            let AZC = Lanes([0e0f64; 3]);
            let AZD = 0e0f64;
            let AZE = Lanes([0e0f64; 3]);
            let AZF = 0e0f64;
            let AZG = Lanes([0e0f64; 3]);
            let AZH = 0e0f64;
            let AZY = staged[842];
            let AZZ = staged[265];
            let BAE = staged[270];
            let BAQ = staged[271];
            let BAX = staged[272];
            let BBS = parameters[90];
            let BBW = parameters[91];
            let BCA = staged[0];
            let BCB = staged[273];
            let BCC = parameters[98];
            let BCJ = parameters[79];
            let BCN = staged[274];
            let BCO = staged[275];
            let BCR = staged[845];
            let BCS = staged[276];
            let BCU = node_potentials[3];
            let BCV = 1e0f64;
            let BCW = parameters[92];
            let BEA = staged[277];
            let BEC = node_potentials[4];
            let BED = 1e0f64;
            let BEY = 2.0895883249536002e-10f64;
            let BEZ = staged[278];
            let BFB = parameters[94];
            let BFI = staged[846];
            let BFJ = staged[280];
            let BFL = node_potentials[5];
            let BFM = 1e0f64;
            let BFN = parameters[95];
            let BFZ = staged[281];
            let BGS = staged[285];
            let BGV = Lanes([0e0f64; 2]);
            let BGY = staged[847];
            let BGZ = ddt_scale();
            let BHA = 1e-12f64;
            let BHR = staged[848];
            let BHS = 1e-13f64;
            let BID = staged[286];
            let BJO = 0e0f64;
            let BJP = 0e0f64;
            let BJQ = 0e0f64;
            let G = E - F;
            let J = Lanes([H, 0.0]) - Lanes([0.0, I]);
            let S;
            let T;
            let U;
            let V;
            let W;
            let X;
            let Y;
            let Z;
            let AA;
            let AB;
            let AC;
            let AD;
            if D != 0.0 {
                let L = G * K;
                let M = J * K;
                let O = L * N;
                let P = M * N;
                let Q = if O < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let AX;
                let AY;
                if Q != 0.0 {
                    let AR = (-2.3025850929940458e2f64 - O) + AQ;
                    let AT = AS / AR;
                    let AU = (((P * AP) * AT) * AP) / AR;
                    AX = AT;
                    AY = AU;
                } else {
                    let AW = if O > AV { 1.0 } else { 0.0 };
                    let BL;
                    let BM;
                    if AW != 0.0 {
                        let BH = BG * ((O - AV) + AQ);
                        let BI = P * BG;
                        BL = BH;
                        BM = BI;
                    } else {
                        let BJ = O.exp();
                        let BK = P * BJ;
                        BL = BJ;
                        BM = BK;
                    }
                    AX = BL;
                    AY = BM;
                }
                let BA = AZ * (AX - AQ);
                let BB = AY * AZ;
                let BD = L * BC;
                let BE = M * BC;
                let BF = if BD < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let BS;
                let BT;
                if BF != 0.0 {
                    let BN = (-2.3025850929940458e2f64 - BD) + AQ;
                    let BO = AS / BN;
                    let BP = (((BE * AP) * BO) * AP) / BN;
                    BS = BO;
                    BT = BP;
                } else {
                    let BR = if BD > BQ { 1.0 } else { 0.0 };
                    let CD;
                    let CE;
                    if BR != 0.0 {
                        let BZ = BY * ((BD - BQ) + AQ);
                        let CA = BE * BY;
                        CD = BZ;
                        CE = CA;
                    } else {
                        let CB = BD.exp();
                        let CC = BE * CB;
                        CD = CB;
                        CE = CC;
                    }
                    BS = CD;
                    BT = CE;
                }
                let BV = BU * (BS - AQ);
                let BW = BT * BU;
                let CM;
                let CN;
                if BX != 0.0 {
                    let CG = staged[192] + (G * CF);
                    let CH = G * CG;
                    let CI = (J * CG) + ((J * CF) * G);
                    CM = CH;
                    CN = CI;
                } else {
                    let CJ = ((-G) * K) * CF;
                    let CK = ((J * AP) * K) * CF;
                    let CL = if CJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    let DK;
                    let DL;
                    if CL != 0.0 {
                        let DF = (-2.3025850929940458e2f64 - CJ) + AQ;
                        let DG = AS / DF;
                        let DH = (((CK * AP) * DG) * AP) / DF;
                        DK = DG;
                        DL = DH;
                    } else {
                        let DJ = if CJ > DI { 1.0 } else { 0.0 };
                        let DU;
                        let DV;
                        if DJ != 0.0 {
                            let DQ = DP * ((CJ - DI) + AQ);
                            let DR = CK * DP;
                            DU = DQ;
                            DV = DR;
                        } else {
                            let DS = CJ.exp();
                            let DT = CK * DS;
                            DU = DS;
                            DV = DT;
                        }
                        DK = DU;
                        DL = DV;
                    }
                    let DN = DM * (DK - AQ);
                    let DO = DL * DM;
                    CM = DN;
                    CN = DO;
                }
                let CO = (BA + BV) + CM;
                let CP = (BB + BW) + CN;
                let CQ = BV + CM;
                let CR = BW + CN;
                let CS = G + staged[196];
                let CU = CT - CS;
                let CV = (J * AP) * CU;
                let CW = ((CU * CU) + staged[198]).sqrt();
                let CZ = (CT + CS) + CW;
                let DA = (G * CT) / CZ;
                let DC = DB * DA;
                let DD = (((J * CT) - ((J + ((CV + CV) * (CY / (CX * CW)))) * DA)) / CZ) * DB;
                let DZ;
                let EA;
                if DE != 0.0 {
                    let EJ;
                    let EK;
                    if DW != 0.0 {
                        let ED = (AQ - (DC * EC)).sqrt();
                        let EE = ((DD * EC) * AP) * (CY / (CX * ED));
                        EJ = ED;
                        EK = EE;
                    } else {
                        let EF = AQ - (DC * EC);
                        let EH = EF.powf(EG);
                        let EI = ((DD * EC) * AP) * (EG * (EF.powf(staged[299])));
                        EJ = EH;
                        EK = EI;
                    }
                    let EN = (EL * (AQ - EJ)) + (EM * (G - DC));
                    let EO = ((EK * AP) * EL) + ((J - DD) * EM);
                    DZ = EN;
                    EA = EO;
                } else {
                    DZ = DX;
                    EA = DY;
                }
                let EQ;
                let ER;
                if EB != 0.0 {
                    let FA;
                    let FB;
                    if EP != 0.0 {
                        let EU = (AQ - (DC * ET)).sqrt();
                        let EV = ((DD * ET) * AP) * (CY / (CX * EU));
                        FA = EU;
                        FB = EV;
                    } else {
                        let EW = AQ - (DC * ET);
                        let EY = EW.powf(EX);
                        let EZ = ((DD * ET) * AP) * (EX * (EW.powf(staged[300])));
                        FA = EY;
                        FB = EZ;
                    }
                    let FE = (FC * (AQ - FA)) + (FD * (G - DC));
                    let FF = ((FB * AP) * FC) + ((J - DD) * FD);
                    EQ = FE;
                    ER = FF;
                } else {
                    EQ = DX;
                    ER = DY;
                }
                let FH;
                let FI;
                if ES != 0.0 {
                    let FR;
                    let FS;
                    if FG != 0.0 {
                        let FL = (AQ - (DC * FK)).sqrt();
                        let FM = ((DD * FK) * AP) * (CY / (CX * FL));
                        FR = FL;
                        FS = FM;
                    } else {
                        let FN = AQ - (DC * FK);
                        let FP = FN.powf(FO);
                        let FQ = ((DD * FK) * AP) * (FO * (FN.powf(staged[301])));
                        FR = FP;
                        FS = FQ;
                    }
                    let FV = (FT * (AQ - FR)) + (FU * (G - DC));
                    let FW = ((FS * AP) * FT) + ((J - DD) * FU);
                    FH = FV;
                    FI = FW;
                } else {
                    FH = DX;
                    FI = DY;
                }
                S = DZ;
                T = EQ;
                U = FH;
                V = FJ;
                W = CO;
                X = CQ;
                Y = EA;
                Z = ER;
                AA = FI;
                AB = DY;
                AC = CP;
                AD = CR;
            } else {
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
                if R != 0.0 {
                    let FX = G + staged[208];
                    let FY = CT - FX;
                    let FZ = J * AP;
                    let GA = FZ * FY;
                    let GB = ((FY * FY) + staged[209]).sqrt();
                    let GC = (CT + FX) + GB;
                    let GD = (G * CT) / GC;
                    let GE = DB * GD;
                    let GF = (((J * CT) - ((J + ((GA + GA) * (CY / (CX * GB)))) * GD)) / GC) * DB;
                    let GH = if G < GG { 1.0 } else { 0.0 };
                    let HM;
                    let HN;
                    let HO;
                    let HP;
                    let HQ;
                    let HR;
                    let HS;
                    let HT;
                    if GH != 0.0 {
                        let HB = HA * (G * K);
                        let HC = (J * K) * HA;
                        let HE = if (HB.abs()) < HD { 1.0 } else { 0.0 };
                        let ID;
                        let IE;
                        if HE != 0.0 {
                            let IA = HB.exp();
                            let IB = HC * IA;
                            ID = IA;
                            IE = IB;
                        } else {
                            let IC = if HB < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let IV;
                            let IW;
                            if IC != 0.0 {
                                let IG = -2.3025850929940458e2f64 - HB;
                                let IH = HC * AP;
                                let II = -2.3025850929940458e2f64 - HB;
                                let IK = AQ + ((-2.3025850929940458e2f64 - HB) * IJ);
                                let IL = AQ + (HA * (II * IK));
                                let IM = AQ + (IG * IL);
                                let IN = AS / IM;
                                let IO = ((((IH * IL) + ((((IH * IK) + ((IH * IJ) * II)) * HA) * IG)) * IN) * AP) / IM;
                                IV = IN;
                                IW = IO;
                            } else {
                                let IP = HB - HD;
                                let IQ = AQ + (IP * IJ);
                                let IR = AQ + (HA * (IP * IQ));
                                let IT = IS * (AQ + (IP * IR));
                                let IU = ((HC * IR) + ((((HC * IQ) + ((HC * IJ) * IP)) * HA) * IP)) * IS;
                                IV = IT;
                                IW = IU;
                            }
                            ID = IV;
                            IE = IW;
                        }
                        let IF = if HJ < HK { 1.0 } else { 0.0 };
                        let JG;
                        let JH;
                        let JI;
                        if IF != 0.0 {
                            let IZ = HJ - (IY * IX);
                            let JA = (J * IY) * AP;
                            let JC = (HK - ((IY * (G - IX)) + HJ)) - JB;
                            let JE = (JD * HK) * JB;
                            let JF = if JE > DX { 1.0 } else { 0.0 };
                            let JP = if JF != 0.0 {
                                JE
                            } else {
                                let JO = -JE;
                                JO
                            };
                            let JQ = JA * JC;
                            let JR = ((JC * JC) + JP).sqrt();
                            let JS = ((JA + ((JQ + JQ) * (CY / (CX * JR)))) * HA) * AP;
                            let JT = ((HK - (HA * (JC + JR))) - HJ) - JB;
                            let JU = (JD * HJ) * JB;
                            let JV = if JU > DX { 1.0 } else { 0.0 };
                            let JX = if JV != 0.0 {
                                JU
                            } else {
                                let JW = -JU;
                                JW
                            };
                            let JY = JS * JT;
                            let JZ = ((JT * JT) + JX).sqrt();
                            let KA = (JS + ((JY + JY) * (CY / (CX * JZ)))) * HA;
                            let KB = HJ + (HA * (JT + JZ));
                            let KC = (HK - IZ) - JB;
                            let KE = if JF != 0.0 {
                                JE
                            } else {
                                let KD = -JE;
                                KD
                            };
                            let KF = ((HK - (HA * (KC + (((KC * KC) + KE).sqrt())))) - HJ) - JB;
                            let KH = if JV != 0.0 {
                                JU
                            } else {
                                let KG = -JU;
                                KG
                            };
                            let KI = HJ + (HA * (KF + (((KF * KF) + KH).sqrt())));
                            JG = KB;
                            JH = KI;
                            JI = KA;
                        } else {
                            JG = HJ;
                            JH = HJ;
                            JI = DY;
                        }
                        let JJ = G / JG;
                        let JK = JH * HK;
                        let JL = K * (JJ + ((IX * (JG - JH)) / JK));
                        let JM = (((J - (JI * JJ)) / JG) + ((JI * IX) / JK)) * K;
                        let JN = if (JL.abs()) < HD { 1.0 } else { 0.0 };
                        let KM;
                        let KN;
                        if JN != 0.0 {
                            let KJ = JL.exp();
                            let KK = JM * KJ;
                            KM = KJ;
                            KN = KK;
                        } else {
                            let KL = if JL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let LG;
                            let LH;
                            if KL != 0.0 {
                                let KT = -2.3025850929940458e2f64 - JL;
                                let KU = JM * AP;
                                let KV = -2.3025850929940458e2f64 - JL;
                                let KW = AQ + ((-2.3025850929940458e2f64 - JL) * IJ);
                                let KX = AQ + (HA * (KV * KW));
                                let KY = AQ + (KT * KX);
                                let KZ = AS / KY;
                                let LA = ((((KU * KX) + ((((KU * KW) + ((KU * IJ) * KV)) * HA) * KT)) * KZ) * AP) / KY;
                                LG = KZ;
                                LH = LA;
                            } else {
                                let LB = JL - HD;
                                let LC = AQ + (LB * IJ);
                                let LD = AQ + (HA * (LB * LC));
                                let LE = IS * (AQ + (LB * LD));
                                let LF = ((JM * LD) + ((((JM * LC) + ((JM * IJ) * LB)) * HA) * LB)) * IS;
                                LG = LE;
                                LH = LF;
                            }
                            KM = LG;
                            KN = LH;
                        }
                        let KR = (KQ / K) * ((KP / (KO / KP)).ln());
                        let KS = if KQ < HK { 1.0 } else { 0.0 };
                        let LN;
                        let LO;
                        let LP;
                        if KS != 0.0 {
                            let LI = KQ - (IY * KR);
                            let LJ = (J * IY) * AP;
                            let LK = (HK - ((IY * (G - KR)) + KQ)) - JB;
                            let LL = (JD * HK) * JB;
                            let LM = if LL > DX { 1.0 } else { 0.0 };
                            let LW = if LM != 0.0 {
                                LL
                            } else {
                                let LV = -LL;
                                LV
                            };
                            let LX = LJ * LK;
                            let LY = ((LK * LK) + LW).sqrt();
                            let LZ = ((LJ + ((LX + LX) * (CY / (CX * LY)))) * HA) * AP;
                            let MA = ((HK - (HA * (LK + LY))) - KQ) - JB;
                            let MB = (JD * KQ) * JB;
                            let MC = if MB > DX { 1.0 } else { 0.0 };
                            let ME = if MC != 0.0 {
                                MB
                            } else {
                                let MD = -MB;
                                MD
                            };
                            let MF = LZ * MA;
                            let MG = ((MA * MA) + ME).sqrt();
                            let MH = (LZ + ((MF + MF) * (CY / (CX * MG)))) * HA;
                            let MI = KQ + (HA * (MA + MG));
                            let MJ = (HK - LI) - JB;
                            let ML = if LM != 0.0 {
                                LL
                            } else {
                                let MK = -LL;
                                MK
                            };
                            let MM = ((HK - (HA * (MJ + (((MJ * MJ) + ML).sqrt())))) - KQ) - JB;
                            let MO = if MC != 0.0 {
                                MB
                            } else {
                                let MN = -MB;
                                MN
                            };
                            let MP = KQ + (HA * (MM + (((MM * MM) + MO).sqrt())));
                            LN = MI;
                            LO = MP;
                            LP = MH;
                        } else {
                            LN = KQ;
                            LO = KQ;
                            LP = DY;
                        }
                        let LQ = G / LN;
                        let LR = LO * HK;
                        let LS = K * (LQ + ((KR * (LN - LO)) / LR));
                        let LT = (((J - (LP * LQ)) / LN) + ((LP * KR) / LR)) * K;
                        let LU = if (LS.abs()) < HD { 1.0 } else { 0.0 };
                        let MT;
                        let MU;
                        if LU != 0.0 {
                            let MQ = LS.exp();
                            let MR = LT * MQ;
                            MT = MQ;
                            MU = MR;
                        } else {
                            let MS = if LS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let NM;
                            let NN;
                            if MS != 0.0 {
                                let MZ = -2.3025850929940458e2f64 - LS;
                                let NA = LT * AP;
                                let NB = -2.3025850929940458e2f64 - LS;
                                let NC = AQ + ((-2.3025850929940458e2f64 - LS) * IJ);
                                let ND = AQ + (HA * (NB * NC));
                                let NE = AQ + (MZ * ND);
                                let NF = AS / NE;
                                let NG = ((((NA * ND) + ((((NA * NC) + ((NA * IJ) * NB)) * HA) * MZ)) * NF) * AP) / NE;
                                NM = NF;
                                NN = NG;
                            } else {
                                let NH = LS - HD;
                                let NI = AQ + (NH * IJ);
                                let NJ = AQ + (HA * (NH * NI));
                                let NK = IS * (AQ + (NH * NJ));
                                let NL = ((LT * NJ) + ((((LT * NI) + ((LT * IJ) * NH)) * HA) * NH)) * IS;
                                NM = NK;
                                NN = NL;
                            }
                            MT = NM;
                            MU = NN;
                        }
                        let MX = (MW / K) * ((MV / (KO / MV)).ln());
                        let MY = if MW < HK { 1.0 } else { 0.0 };
                        let NT;
                        let NU;
                        let NV;
                        if MY != 0.0 {
                            let NO = MW - (IY * MX);
                            let NP = (J * IY) * AP;
                            let NQ = (HK - ((IY * (G - MX)) + MW)) - JB;
                            let NR = (JD * HK) * JB;
                            let NS = if NR > DX { 1.0 } else { 0.0 };
                            let OC = if NS != 0.0 {
                                NR
                            } else {
                                let OB = -NR;
                                OB
                            };
                            let OD = NP * NQ;
                            let OE = ((NQ * NQ) + OC).sqrt();
                            let OF = ((NP + ((OD + OD) * (CY / (CX * OE)))) * HA) * AP;
                            let OG = ((HK - (HA * (NQ + OE))) - MW) - JB;
                            let OH = (JD * MW) * JB;
                            let OI = if OH > DX { 1.0 } else { 0.0 };
                            let OK = if OI != 0.0 {
                                OH
                            } else {
                                let OJ = -OH;
                                OJ
                            };
                            let OL = OF * OG;
                            let OM = ((OG * OG) + OK).sqrt();
                            let ON = (OF + ((OL + OL) * (CY / (CX * OM)))) * HA;
                            let OO = MW + (HA * (OG + OM));
                            let OP = (HK - NO) - JB;
                            let OR = if NS != 0.0 {
                                NR
                            } else {
                                let OQ = -NR;
                                OQ
                            };
                            let OS = ((HK - (HA * (OP + (((OP * OP) + OR).sqrt())))) - MW) - JB;
                            let OU = if OI != 0.0 {
                                OH
                            } else {
                                let OT = -OH;
                                OT
                            };
                            let OV = MW + (HA * (OS + (((OS * OS) + OU).sqrt())));
                            NT = OO;
                            NU = OV;
                            NV = ON;
                        } else {
                            NT = MW;
                            NU = MW;
                            NV = DY;
                        }
                        let NW = G / NT;
                        let NX = NU * HK;
                        let NY = K * (NW + ((MX * (NT - NU)) / NX));
                        let NZ = (((J - (NV * NW)) / NT) + ((NV * MX) / NX)) * K;
                        let OA = if (NY.abs()) < HD { 1.0 } else { 0.0 };
                        let OZ;
                        let PA;
                        if OA != 0.0 {
                            let OW = NY.exp();
                            let OX = NZ * OW;
                            OZ = OW;
                            PA = OX;
                        } else {
                            let OY = if NY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let PO;
                            let PP;
                            if OY != 0.0 {
                                let PB = -2.3025850929940458e2f64 - NY;
                                let PC = NZ * AP;
                                let PD = -2.3025850929940458e2f64 - NY;
                                let PE = AQ + ((-2.3025850929940458e2f64 - NY) * IJ);
                                let PF = AQ + (HA * (PD * PE));
                                let PG = AQ + (PB * PF);
                                let PH = AS / PG;
                                let PI = ((((PC * PF) + ((((PC * PE) + ((PC * IJ) * PD)) * HA) * PB)) * PH) * AP) / PG;
                                PO = PH;
                                PP = PI;
                            } else {
                                let PJ = NY - HD;
                                let PK = AQ + (PJ * IJ);
                                let PL = AQ + (HA * (PJ * PK));
                                let PM = IS * (AQ + (PJ * PL));
                                let PN = ((NZ * PL) + ((((NZ * PK) + ((NZ * IJ) * PJ)) * HA) * PJ)) * IS;
                                PO = PM;
                                PP = PN;
                            }
                            OZ = PO;
                            PA = PP;
                        }
                        HM = KM;
                        HN = MT;
                        HO = OZ;
                        HP = ID;
                        HQ = KN;
                        HR = MU;
                        HS = PA;
                        HT = IE;
                    } else {
                        let HF = G - GG;
                        let HH = ((AQ + (HF * K)) * HG).sqrt();
                        let HI = ((J * K) * HG) * (CY / (CX * HH));
                        let HL = if HJ < HK { 1.0 } else { 0.0 };
                        let PU;
                        let PV;
                        let PW;
                        if HL != 0.0 {
                            let PQ = HJ - (IY * IX);
                            let PR = (HK - ((IY * (GG - IX)) + HJ)) - JB;
                            let PS = (JD * HK) * JB;
                            let PT = if PS > DX { 1.0 } else { 0.0 };
                            let QB = if PT != 0.0 {
                                PS
                            } else {
                                let QA = -PS;
                                QA
                            };
                            let QC = ((PR * PR) + QB).sqrt();
                            let QD = HA * (AQ + (PR / QC));
                            let QE = ((HK - (HA * (PR + QC))) - HJ) - JB;
                            let QF = (JD * HJ) * JB;
                            let QG = if QF > DX { 1.0 } else { 0.0 };
                            let QI = if QG != 0.0 {
                                QF
                            } else {
                                let QH = -QF;
                                QH
                            };
                            let QJ = ((QE * QE) + QI).sqrt();
                            let QK = HA * (AQ + (QE / QJ));
                            let QL = HJ + (HA * (QE + QJ));
                            let QM = (HK - PQ) - JB;
                            let QO = if PT != 0.0 {
                                PS
                            } else {
                                let QN = -PS;
                                QN
                            };
                            let QP = ((HK - (HA * (QM + (((QM * QM) + QO).sqrt())))) - HJ) - JB;
                            let QR = if QG != 0.0 {
                                QF
                            } else {
                                let QQ = -QF;
                                QQ
                            };
                            let QS = HJ + (HA * (QP + (((QP * QP) + QR).sqrt())));
                            let QT = (IY * QD) * QK;
                            PU = QL;
                            PV = QS;
                            PW = QT;
                        } else {
                            PU = HJ;
                            PV = HJ;
                            PW = DX;
                        }
                        let PX = PV * HK;
                        let PY = K * ((GG / PU) + ((IX * (PU - PV)) / PX));
                        let PZ = if (PY.abs()) < HD { 1.0 } else { 0.0 };
                        let QW;
                        if PZ != 0.0 {
                            let QU = PY.exp();
                            QW = QU;
                        } else {
                            let QV = if PY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let RF = if QV != 0.0 {
                                let RC = AS / (AQ + ((-2.3025850929940458e2f64 - PY) * (AQ + (HA * ((-2.3025850929940458e2f64 - PY) * (AQ + ((-2.3025850929940458e2f64 - PY) * IJ)))))));
                                RC
                            } else {
                                let RD = PY - HD;
                                let RE = IS * (AQ + (RD * (AQ + (HA * (RD * (AQ + (RD * IJ)))))));
                                RE
                            };
                            QW = RF;
                        }
                        let QX = K * (((PU - (GG * PW)) / (PU * PU)) + ((IX * PW) / PX));
                        let QY = (AQ + (HF * QX)) * QW;
                        let QZ = (J * QX) * QW;
                        let RA = (KQ / K) * ((KP / (KO / KP)).ln());
                        let RB = if KQ < HK { 1.0 } else { 0.0 };
                        let RK;
                        let RL;
                        let RM;
                        if RB != 0.0 {
                            let RG = KQ - (IY * RA);
                            let RH = (HK - ((IY * (GG - RA)) + KQ)) - JB;
                            let RI = (JD * HK) * JB;
                            let RJ = if RI > DX { 1.0 } else { 0.0 };
                            let RR = if RJ != 0.0 {
                                RI
                            } else {
                                let RQ = -RI;
                                RQ
                            };
                            let RS = ((RH * RH) + RR).sqrt();
                            let RT = HA * (AQ + (RH / RS));
                            let RU = ((HK - (HA * (RH + RS))) - KQ) - JB;
                            let RV = (JD * KQ) * JB;
                            let RW = if RV > DX { 1.0 } else { 0.0 };
                            let RY = if RW != 0.0 {
                                RV
                            } else {
                                let RX = -RV;
                                RX
                            };
                            let RZ = ((RU * RU) + RY).sqrt();
                            let SA = HA * (AQ + (RU / RZ));
                            let SB = KQ + (HA * (RU + RZ));
                            let SC = (HK - RG) - JB;
                            let SE = if RJ != 0.0 {
                                RI
                            } else {
                                let SD = -RI;
                                SD
                            };
                            let SF = ((HK - (HA * (SC + (((SC * SC) + SE).sqrt())))) - KQ) - JB;
                            let SH = if RW != 0.0 {
                                RV
                            } else {
                                let SG = -RV;
                                SG
                            };
                            let SI = KQ + (HA * (SF + (((SF * SF) + SH).sqrt())));
                            let SJ = (IY * RT) * SA;
                            RK = SB;
                            RL = SI;
                            RM = SJ;
                        } else {
                            RK = KQ;
                            RL = KQ;
                            RM = DX;
                        }
                        let RN = RL * HK;
                        let RO = K * ((GG / RK) + ((RA * (RK - RL)) / RN));
                        let RP = if (RO.abs()) < HD { 1.0 } else { 0.0 };
                        let SM;
                        if RP != 0.0 {
                            let SK = RO.exp();
                            SM = SK;
                        } else {
                            let SL = if RO < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let SV = if SL != 0.0 {
                                let SS = AS / (AQ + ((-2.3025850929940458e2f64 - RO) * (AQ + (HA * ((-2.3025850929940458e2f64 - RO) * (AQ + ((-2.3025850929940458e2f64 - RO) * IJ)))))));
                                SS
                            } else {
                                let ST = RO - HD;
                                let SU = IS * (AQ + (ST * (AQ + (HA * (ST * (AQ + (ST * IJ)))))));
                                SU
                            };
                            SM = SV;
                        }
                        let SN = K * (((RK - (GG * RM)) / (RK * RK)) + ((RA * RM) / RN));
                        let SO = (AQ + (HF * SN)) * SM;
                        let SP = (J * SN) * SM;
                        let SQ = (MW / K) * ((MV / (KO / MV)).ln());
                        let SR = if MW < HK { 1.0 } else { 0.0 };
                        let TA;
                        let TB;
                        let TC;
                        if SR != 0.0 {
                            let SW = MW - (IY * SQ);
                            let SX = (HK - ((IY * (GG - SQ)) + MW)) - JB;
                            let SY = (JD * HK) * JB;
                            let SZ = if SY > DX { 1.0 } else { 0.0 };
                            let TH = if SZ != 0.0 {
                                SY
                            } else {
                                let TG = -SY;
                                TG
                            };
                            let TI = ((SX * SX) + TH).sqrt();
                            let TJ = HA * (AQ + (SX / TI));
                            let TK = ((HK - (HA * (SX + TI))) - MW) - JB;
                            let TL = (JD * MW) * JB;
                            let TM = if TL > DX { 1.0 } else { 0.0 };
                            let TO = if TM != 0.0 {
                                TL
                            } else {
                                let TN = -TL;
                                TN
                            };
                            let TP = ((TK * TK) + TO).sqrt();
                            let TQ = HA * (AQ + (TK / TP));
                            let TR = MW + (HA * (TK + TP));
                            let TS = (HK - SW) - JB;
                            let TU = if SZ != 0.0 {
                                SY
                            } else {
                                let TT = -SY;
                                TT
                            };
                            let TV = ((HK - (HA * (TS + (((TS * TS) + TU).sqrt())))) - MW) - JB;
                            let TX = if TM != 0.0 {
                                TL
                            } else {
                                let TW = -TL;
                                TW
                            };
                            let TY = MW + (HA * (TV + (((TV * TV) + TX).sqrt())));
                            let TZ = (IY * TJ) * TQ;
                            TA = TR;
                            TB = TY;
                            TC = TZ;
                        } else {
                            TA = MW;
                            TB = MW;
                            TC = DX;
                        }
                        let TD = TB * HK;
                        let TE = K * ((GG / TA) + ((SQ * (TA - TB)) / TD));
                        let TF = if (TE.abs()) < HD { 1.0 } else { 0.0 };
                        let UC;
                        if TF != 0.0 {
                            let UA = TE.exp();
                            UC = UA;
                        } else {
                            let UB = if TE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let UJ = if UB != 0.0 {
                                let UG = AS / (AQ + ((-2.3025850929940458e2f64 - TE) * (AQ + (HA * ((-2.3025850929940458e2f64 - TE) * (AQ + ((-2.3025850929940458e2f64 - TE) * IJ)))))));
                                UG
                            } else {
                                let UH = TE - HD;
                                let UI = IS * (AQ + (UH * (AQ + (HA * (UH * (AQ + (UH * IJ)))))));
                                UI
                            };
                            UC = UJ;
                        }
                        let UD = K * (((TA - (GG * TC)) / (TA * TA)) + ((SQ * TC) / TD));
                        let UE = (AQ + (HF * UD)) * UC;
                        let UF = (J * UD) * UC;
                        HM = QY;
                        HN = SO;
                        HO = UE;
                        HP = HH;
                        HQ = QZ;
                        HR = SP;
                        HS = UF;
                        HT = HI;
                    }
                    let HU = HM - AQ;
                    let HV = HN - AQ;
                    let HW = HO - AQ;
                    let HX = AQ / HP;
                    let HY = ((HT * HX) * AP) / HP;
                    let HZ = if G > DX { 1.0 } else { 0.0 };
                    let UY;
                    let UZ;
                    if HZ != 0.0 {
                        let UK = HX + AQ;
                        let UM = HX + UL;
                        let UN = (UK * UM).sqrt();
                        let UO = (DB + HX) + UN;
                        let UQ = DB * (UP * (UO.ln()));
                        let UR = (((HY + (((HY * UM) + (HY * UK)) * (CY / (CX * UN)))) * (CY / UO)) * UP) * DB;
                        UY = UQ;
                        UZ = UR;
                    } else {
                        let US = AQ + HP;
                        let UT = AQ + (UL * HP);
                        let UU = (US * UT).sqrt();
                        let UV = ((DB * HP) + AQ) + UU;
                        let UW = (-G) + (DB * (UP * (UV.ln())));
                        let UX = FZ + (((((HT * DB) + (((HT * UT) + ((HT * UL) * US)) * (CY / (CX * UU)))) * (CY / UV)) * UP) * DB);
                        UY = UW;
                        UZ = UX;
                    }
                    let VA = staged[215] - UY;
                    let VB = UZ * AP;
                    let VC = G - VA;
                    let VD = (J - VB) * VC;
                    let VE = ((VC * VC) + staged[216]).sqrt();
                    let VF = HA * ((G + VA) - VE);
                    let VG = ((J + VB) - ((VD + VD) * (CY / (CX * VE)))) * HA;
                    let VI = G - VH;
                    let VJ = J * VI;
                    let VK = ((VI * VI) + staged[218]).sqrt();
                    let VL = HA * ((G + VH) - VK);
                    let VM = (J - ((VJ + VJ) * (CY / (CX * VK)))) * HA;
                    let VN = J * G;
                    let VO = ((G * G) + 4e-12f64).sqrt();
                    let VP = HA * (G - VO);
                    let VQ = (J - ((VN + VN) * (CY / (CX * VO)))) * HA;
                    GI = GE;
                    GJ = HU;
                    GK = VF;
                    GL = UY;
                    GM = HP;
                    GN = VL;
                    GO = VP;
                    GP = HV;
                    GQ = HW;
                    GR = GF;
                    GS = HQ;
                    GT = VG;
                    GU = UZ;
                    GV = HT;
                    GW = VM;
                    GX = VQ;
                    GY = HR;
                    GZ = HS;
                } else {
                    GI = DX;
                    GJ = DX;
                    GK = DX;
                    GL = DX;
                    GM = DX;
                    GN = DX;
                    GO = DX;
                    GP = DX;
                    GQ = DX;
                    GR = DY;
                    GS = DY;
                    GT = DY;
                    GU = DY;
                    GV = DY;
                    GW = DY;
                    GX = DY;
                    GY = DY;
                    GZ = DY;
                }
                let VS;
                let VT;
                let VU;
                let VV;
                let VW;
                let VX;
                if A != 0.0 {
                    VS = DX;
                    VT = DX;
                    VU = DX;
                    VV = DY;
                    VW = DY;
                    VX = DY;
                } else {
                    let WD;
                    let WE;
                    if VR != 0.0 {
                        let VY = (AQ - (GI * EC)).sqrt();
                        let VZ = ((GR * EC) * AP) * (CY / (CX * VY));
                        WD = VY;
                        WE = VZ;
                    } else {
                        let WA = AQ - (GI * EC);
                        let WB = WA.powf(EG);
                        let WC = ((GR * EC) * AP) * (EG * (WA.powf(staged[287])));
                        WD = WB;
                        WE = WC;
                    }
                    let WF = (EL * (AQ - WD)) + (EM * (G - GI));
                    let WG = ((WE * AP) * EL) + ((J - GR) * EM);
                    let WI = WH * GJ;
                    let WJ = GS * WH;
                    let WS;
                    let WT;
                    let WU;
                    let WV;
                    let WW;
                    let WX;
                    let WY;
                    let WZ;
                    let XA;
                    let XB;
                    if WK != 0.0 {
                        WS = DX;
                        WT = DX;
                        WU = DX;
                        WV = DX;
                        WW = DX;
                        WX = DY;
                        WY = DY;
                        WZ = DY;
                        XA = DY;
                        XB = DY;
                    } else {
                        let WL = staged[220] - GK;
                        let WM = GT * AP;
                        let WN = GL / WL;
                        let WO = (AQ - WN).sqrt();
                        let WP = AQ - WO;
                        let WQ = ((((GU - (WM * WN)) / WL) * AP) * (CY / (CX * WO))) * AP;
                        let XL;
                        let XM;
                        if WR != 0.0 {
                            XL = DX;
                            XM = DY;
                        } else {
                            let XD = WP * WP;
                            let XE = WQ * WP;
                            let XF = WP.ln();
                            let XG = AQ - WP;
                            let XH = (XD * XF) / XG;
                            let XJ = (XH + WP) * XI;
                            let XK = ((((((XE + XE) * XF) + ((WQ * (CY / WP)) * XD)) - ((WQ * AP) * XH)) / XG) + WQ) * XI;
                            XL = XJ;
                            XM = XK;
                        }
                        let XN = WP + XL;
                        let XO = WQ + XM;
                        let XW;
                        let XX;
                        if WR != 0.0 {
                            let XQ = (WL * XP).sqrt();
                            let XR = (WM * XP) * (CY / (CX * XQ));
                            XW = XQ;
                            XX = XR;
                        } else {
                            let XS = WL * XP;
                            let XU = XS.powf(XT);
                            let XV = (WM * XP) * (XT * (XS.powf(staged[288])));
                            XW = XU;
                            XX = XV;
                        }
                        let XZ = XY * XW;
                        let YA = XX * XY;
                        let YB = GM - AQ;
                        let YD = YC * (YB * XZ);
                        let YE = ((GV * XZ) + (YA * YB)) * YC;
                        let YG = YF * (YD * XN);
                        let YH = ((YE * XN) + (XO * YD)) * YF;
                        WS = XZ;
                        WT = WL;
                        WU = XN;
                        WV = YD;
                        WW = YG;
                        WX = YA;
                        WY = WM;
                        WZ = XO;
                        XA = YE;
                        XB = YH;
                    }
                    let ZD;
                    let ZE;
                    if XC != 0.0 {
                        ZD = DX;
                        ZE = DY;
                    } else {
                        let YI = (WS * EG) / WT;
                        let YK = YJ * YI;
                        let YL = (((WX * EG) - (WY * YI)) / WT) * YJ;
                        let YM = staged[224] / YK;
                        let YN = ((YL * YM) * AP) / YK;
                        let YO = YM * YM;
                        let YP = YN * YM;
                        let YQ = YO * YO;
                        let YR = (YP + YP) * YO;
                        let YS = YR + YR;
                        let YT = YQ + AQ;
                        let YU = YQ / YT;
                        let YV = YU.sqrt();
                        let YW = ((YS - (YS * YU)) / YT) * (CY / (CX * YV));
                        let YY = (YV.abs()).sqrt();
                        let YZ = (YW * ((CX * (if YV >= YX { 1.0 } else { 0.0 })) - CY)) * (CY / (CX * YY));
                        let ZA = YV * YY;
                        let ZB = (YW * YY) + (YZ * YV);
                        let ZN;
                        let ZO;
                        if ZC != 0.0 {
                            let ZG = AQ + (YK * ZA);
                            let ZH = AQ / ZG;
                            let ZI = ((((YL * ZA) + (ZB * YK)) * ZH) * AP) / ZG;
                            ZN = ZH;
                            ZO = ZI;
                        } else {
                            let ZJ = AQ + (YK * ZA);
                            let ZL = ZJ.powf(ZK);
                            let ZM = ((YL * ZA) + (ZB * YK)) * (ZK * (ZJ.powf(staged[289])));
                            ZN = ZL;
                            ZO = ZM;
                        }
                        let ZP = WU + ZN;
                        let ZQ = (WU * ZN) / ZP;
                        let ZR = (((WZ * ZN) + (ZO * WU)) - ((WZ + ZO) * ZQ)) / ZP;
                        let ZS = YK / YY;
                        let ZU = (ZT * ZS).sqrt();
                        let ZV = (((YL - (YZ * ZS)) / YY) * ZT) * (CY / (CX * ZU));
                        let ZX = ZW * YM;
                        let ZY = ((ZX * YY) - (ZW * YV)) + (HA * (YK * ZA));
                        let ZZ = ((((YN * ZW) * YY) + (YZ * ZX)) - (YW * ZW)) + (((YL * ZA) + (ZB * YK)) * HA);
                        let AAA = ((DB * (YM * YY)) - YV) - AQ;
                        let AAB = AAA * ZU;
                        let AAC = (((((YN * YY) + (YZ * YM)) * DB) - YW) * ZU) + (ZV * AAA);
                        let AAD = AAB * AAB;
                        let AAE = AAC * AAB;
                        let AAF = AAE + AAE;
                        let AAG = if AAB > DX { 1.0 } else { 0.0 };
                        let AAO;
                        let AAP;
                        if AAG != 0.0 {
                            let AAI = AQ + (AAH * AAB);
                            let AAJ = AQ / AAI;
                            let AAK = (((AAC * AAH) * AAJ) * AP) / AAI;
                            AAO = AAJ;
                            AAP = AAK;
                        } else {
                            let AAL = AQ - (AAH * AAB);
                            let AAM = AQ / AAL;
                            let AAN = ((((AAC * AAH) * AP) * AAM) * AP) / AAL;
                            AAO = AAM;
                            AAP = AAN;
                        }
                        let AAQ = (-AAD) + ZY;
                        let AAR = (AAF * AP) + ZZ;
                        let AAS = if AAQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ABD;
                        let ABE;
                        if AAS != 0.0 {
                            let AAT = AAQ.exp();
                            let AAU = AAR * AAT;
                            ABD = AAT;
                            ABE = AAU;
                        } else {
                            let AAV = -2.3025850929940458e2f64 - AAQ;
                            let AAW = AAR * AP;
                            let AAX = -2.3025850929940458e2f64 - AAQ;
                            let AAY = AQ + ((-2.3025850929940458e2f64 - AAQ) * IJ);
                            let AAZ = AQ + (HA * (AAX * AAY));
                            let ABA = AQ + (AAV * AAZ);
                            let ABB = AS / ABA;
                            let ABC = ((((AAW * AAZ) + ((((AAW * AAY) + ((AAW * IJ) * AAX)) * HA) * AAV)) * ABB) * AP) / ABA;
                            ABD = ABB;
                            ABE = ABC;
                        }
                        let ABG = AAO * AAO;
                        let ABH = AAP * AAO;
                        let ABI = ABH + ABH;
                        let ABL = ((ABF * AAO) + (ABJ * ABG)) + (ABK * (ABG * AAO));
                        let ABM = ABL * ABD;
                        let ABN = ((((AAP * ABF) + (ABI * ABJ)) + (((ABI * AAO) + (AAP * ABG)) * ABK)) * ABD) + (ABE * ABL);
                        let ABP;
                        let ABQ;
                        if AAG != 0.0 {
                            ABP = ABM;
                            ABQ = ABN;
                        } else {
                            let ABO = if ZY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ACI;
                            let ACJ;
                            if ABO != 0.0 {
                                let ABY = ZY.exp();
                                let ABZ = ZZ * ABY;
                                ACI = ABY;
                                ACJ = ABZ;
                            } else {
                                let ACA = -2.3025850929940458e2f64 - ZY;
                                let ACB = ZZ * AP;
                                let ACC = -2.3025850929940458e2f64 - ZY;
                                let ACD = AQ + ((-2.3025850929940458e2f64 - ZY) * IJ);
                                let ACE = AQ + (HA * (ACC * ACD));
                                let ACF = AQ + (ACA * ACE);
                                let ACG = AS / ACF;
                                let ACH = ((((ACB * ACE) + ((((ACB * ACD) + ((ACB * IJ) * ACC)) * HA) * ACA)) * ACG) * AP) / ACF;
                                ACI = ACG;
                                ACJ = ACH;
                            }
                            let ACK = (DB * ACI) - ABM;
                            let ACL = (ACJ * DB) - ABN;
                            ABP = ACK;
                            ABQ = ACL;
                        }
                        let ABR = (ZW * ABP) / ZU;
                        let ABT = ABS * ABR;
                        let ABU = WV * ABT;
                        let ABW = ABV * (ABU * ZQ);
                        let ABX = ((((XA * ABT) + (((((ABQ * ZW) - (ZV * ABR)) / ZU) * ABS) * WV)) * ZQ) + (ZR * ABU)) * ABV;
                        ZD = ABW;
                        ZE = ABX;
                    }
                    let ACN;
                    let ACO;
                    if ZF != 0.0 {
                        ACN = DX;
                        ACO = DY;
                    } else {
                        let ACW;
                        let ACX;
                        if ACM != 0.0 {
                            let ACR = ((ACQ - GN) * XP).sqrt();
                            let ACS = ((GW * AP) * XP) * (CY / (CX * ACR));
                            ACW = ACR;
                            ACX = ACS;
                        } else {
                            let ACT = (ACQ - GN) * XP;
                            let ACU = ACT.powf(XT);
                            let ACV = ((GW * AP) * XP) * (XT * (ACT.powf(staged[290])));
                            ACW = ACU;
                            ACX = ACV;
                        }
                        let ACZ = ((ACQ - GN) * ACY) / ACW;
                        let ADB = ADA * ACZ;
                        let ADC = ((((GW * AP) * ACY) - (ACX * ACZ)) / ACW) * ADA;
                        let ADD = staged[227] / ADB;
                        let ADE = ((ADC * ADD) * AP) / ADB;
                        let ADF = if (ADD.abs()) < HD { 1.0 } else { 0.0 };
                        let ADJ;
                        let ADK;
                        if ADF != 0.0 {
                            let ADG = ADD.exp();
                            let ADH = ADE * ADG;
                            ADJ = ADG;
                            ADK = ADH;
                        } else {
                            let ADI = if ADD < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AED;
                            let AEE;
                            if ADI != 0.0 {
                                let ADQ = -2.3025850929940458e2f64 - ADD;
                                let ADR = ADE * AP;
                                let ADS = -2.3025850929940458e2f64 - ADD;
                                let ADT = AQ + ((-2.3025850929940458e2f64 - ADD) * IJ);
                                let ADU = AQ + (HA * (ADS * ADT));
                                let ADV = AQ + (ADQ * ADU);
                                let ADW = AS / ADV;
                                let ADX = ((((ADR * ADU) + ((((ADR * ADT) + ((ADR * IJ) * ADS)) * HA) * ADQ)) * ADW) * AP) / ADV;
                                AED = ADW;
                                AEE = ADX;
                            } else {
                                let ADY = ADD - HD;
                                let ADZ = AQ + (ADY * IJ);
                                let AEA = AQ + (HA * (ADY * ADZ));
                                let AEB = IS * (AQ + (ADY * AEA));
                                let AEC = ((ADE * AEA) + ((((ADE * ADZ) + ((ADE * IJ) * ADY)) * HA) * ADY)) * IS;
                                AED = AEB;
                                AEE = AEC;
                            }
                            ADJ = AED;
                            ADK = AEE;
                        }
                        let ADL = G * ADB;
                        let ADM = ADL * ADB;
                        let ADO = ADN * (ADM * ADJ);
                        let ADP = ((((((J * ADB) + (ADC * G)) * ADB) + (ADC * ADL)) * ADJ) + (ADK * ADM)) * ADN;
                        ACN = ADO;
                        ACO = ADP;
                    }
                    let AEG;
                    let AEH;
                    if ACP != 0.0 {
                        AEG = AQ;
                        AEH = DY;
                    } else {
                        let AEF = if GO > staged[229] { 1.0 } else { 0.0 };
                        let AEU;
                        let AEV;
                        if AEF != 0.0 {
                            let AEP = if AEO == JD { 1.0 } else { 0.0 };
                            let AFJ;
                            let AFK;
                            if AEP != 0.0 {
                                let AEX = GO * AEW;
                                let AEY = AEX.abs();
                                let AEZ = (GX * AEW) * ((CX * (if AEX >= YX { 1.0 } else { 0.0 })) - CY);
                                let AFA = AEY * AEY;
                                let AFB = AEZ * AEY;
                                let AFC = AFA * AEY;
                                let AFD = AFC * AEY;
                                let AFE = ((((AFB + AFB) * AEY) + (AEZ * AFA)) * AEY) + (AEZ * AFC);
                                AFJ = AFD;
                                AFK = AFE;
                            } else {
                                let AFF = GO * AEW;
                                let AFG = AFF.abs();
                                let AFH = AFG.powf(AEO);
                                let AFI = ((GX * AEW) * ((CX * (if AFF >= YX { 1.0 } else { 0.0 })) - CY)) * (AEO * (AFG.powf((AEO - CY))));
                                AFJ = AFH;
                                AFK = AFI;
                            }
                            let AFL = AQ - AFJ;
                            let AFM = AQ / AFL;
                            let AFN = (((AFK * AP) * AFM) * AP) / AFL;
                            AEU = AFM;
                            AEV = AFN;
                        } else {
                            let AES = GX * AER;
                            let AET = staged[83] + ((GO + (AEQ * staged[231])) * AER);
                            AEU = AET;
                            AEV = AES;
                        }
                        AEG = AEU;
                        AEH = AEV;
                    }
                    let AEI = ((WI + WW) + ZD) + ACN;
                    let AEJ = AEI * AEG;
                    let AEK = ((((WJ + XB) + ZE) + ACO) * AEG) + (AEH * AEI);
                    let AEL = (WW + ZD) + ACN;
                    let AEM = AEL * AEG;
                    let AEN = (((XB + ZE) + ACO) * AEG) + (AEH * AEL);
                    VS = AEJ;
                    VT = AEM;
                    VU = WF;
                    VV = AEK;
                    VW = AEN;
                    VX = WG;
                }
                let AFP;
                let AFQ;
                let AFR;
                let AFS;
                let AFT;
                let AFU;
                if B != 0.0 {
                    AFP = DX;
                    AFQ = DX;
                    AFR = DX;
                    AFS = DY;
                    AFT = DY;
                    AFU = DY;
                } else {
                    let AGA;
                    let AGB;
                    if AFO != 0.0 {
                        let AFV = (AQ - (GI * ET)).sqrt();
                        let AFW = ((GR * ET) * AP) * (CY / (CX * AFV));
                        AGA = AFV;
                        AGB = AFW;
                    } else {
                        let AFX = AQ - (GI * ET);
                        let AFY = AFX.powf(EX);
                        let AFZ = ((GR * ET) * AP) * (EX * (AFX.powf(staged[291])));
                        AGA = AFY;
                        AGB = AFZ;
                    }
                    let AGC = (FC * (AQ - AGA)) + (FD * (G - GI));
                    let AGD = ((AGB * AP) * FC) + ((J - GR) * FD);
                    let AGF = AGE * GP;
                    let AGG = GY * AGE;
                    let AGP;
                    let AGQ;
                    let AGR;
                    let AGS;
                    let AGT;
                    let AGU;
                    let AGV;
                    let AGW;
                    let AGX;
                    let AGY;
                    if AGH != 0.0 {
                        AGP = DX;
                        AGQ = DX;
                        AGR = DX;
                        AGS = DX;
                        AGT = DX;
                        AGU = DY;
                        AGV = DY;
                        AGW = DY;
                        AGX = DY;
                        AGY = DY;
                    } else {
                        let AGI = staged[234] - GK;
                        let AGJ = GT * AP;
                        let AGK = GL / AGI;
                        let AGL = (AQ - AGK).sqrt();
                        let AGM = AQ - AGL;
                        let AGN = ((((GU - (AGJ * AGK)) / AGI) * AP) * (CY / (CX * AGL))) * AP;
                        let AHI;
                        let AHJ;
                        if AGO != 0.0 {
                            AHI = DX;
                            AHJ = DY;
                        } else {
                            let AHA = AGM * AGM;
                            let AHB = AGN * AGM;
                            let AHC = AGM.ln();
                            let AHD = AQ - AGM;
                            let AHE = (AHA * AHC) / AHD;
                            let AHG = (AHE + AGM) * AHF;
                            let AHH = ((((((AHB + AHB) * AHC) + ((AGN * (CY / AGM)) * AHA)) - ((AGN * AP) * AHE)) / AHD) + AGN) * AHF;
                            AHI = AHG;
                            AHJ = AHH;
                        }
                        let AHK = AGM + AHI;
                        let AHL = AGN + AHJ;
                        let AHT;
                        let AHU;
                        if AGO != 0.0 {
                            let AHN = (AGI * AHM).sqrt();
                            let AHO = (AGJ * AHM) * (CY / (CX * AHN));
                            AHT = AHN;
                            AHU = AHO;
                        } else {
                            let AHP = AGI * AHM;
                            let AHR = AHP.powf(AHQ);
                            let AHS = (AGJ * AHM) * (AHQ * (AHP.powf(staged[292])));
                            AHT = AHR;
                            AHU = AHS;
                        }
                        let AHW = AHV * AHT;
                        let AHX = AHU * AHV;
                        let AHY = GM - AQ;
                        let AIA = AHZ * (AHY * AHW);
                        let AIB = ((GV * AHW) + (AHX * AHY)) * AHZ;
                        let AID = AIC * (AIA * AHK);
                        let AIE = ((AIB * AHK) + (AHL * AIA)) * AIC;
                        AGP = AHW;
                        AGQ = AGI;
                        AGR = AHK;
                        AGS = AIA;
                        AGT = AID;
                        AGU = AHX;
                        AGV = AGJ;
                        AGW = AHL;
                        AGX = AIB;
                        AGY = AIE;
                    }
                    let AIZ;
                    let AJA;
                    if AGZ != 0.0 {
                        AIZ = DX;
                        AJA = DY;
                    } else {
                        let AIF = (AGP * EX) / AGQ;
                        let AIH = AIG * AIF;
                        let AII = (((AGU * EX) - (AGV * AIF)) / AGQ) * AIG;
                        let AIJ = staged[238] / AIH;
                        let AIK = ((AII * AIJ) * AP) / AIH;
                        let AIL = AIJ * AIJ;
                        let AIM = AIK * AIJ;
                        let AIN = AIL * AIL;
                        let AIO = (AIM + AIM) * AIL;
                        let AIP = AIO + AIO;
                        let AIQ = AIN + AQ;
                        let AIR = AIN / AIQ;
                        let AIS = AIR.sqrt();
                        let AIT = ((AIP - (AIP * AIR)) / AIQ) * (CY / (CX * AIS));
                        let AIU = (AIS.abs()).sqrt();
                        let AIV = (AIT * ((CX * (if AIS >= YX { 1.0 } else { 0.0 })) - CY)) * (CY / (CX * AIU));
                        let AIW = AIS * AIU;
                        let AIX = (AIT * AIU) + (AIV * AIS);
                        let AJJ;
                        let AJK;
                        if AIY != 0.0 {
                            let AJC = AQ + (AIH * AIW);
                            let AJD = AQ / AJC;
                            let AJE = ((((AII * AIW) + (AIX * AIH)) * AJD) * AP) / AJC;
                            AJJ = AJD;
                            AJK = AJE;
                        } else {
                            let AJF = AQ + (AIH * AIW);
                            let AJH = AJF.powf(AJG);
                            let AJI = ((AII * AIW) + (AIX * AIH)) * (AJG * (AJF.powf(staged[293])));
                            AJJ = AJH;
                            AJK = AJI;
                        }
                        let AJL = AGR + AJJ;
                        let AJM = (AGR * AJJ) / AJL;
                        let AJN = (((AGW * AJJ) + (AJK * AGR)) - ((AGW + AJK) * AJM)) / AJL;
                        let AJO = AIH / AIU;
                        let AJP = (ZT * AJO).sqrt();
                        let AJQ = (((AII - (AIV * AJO)) / AIU) * ZT) * (CY / (CX * AJP));
                        let AJS = AJR * AIJ;
                        let AJT = ((AJS * AIU) - (AJR * AIS)) + (HA * (AIH * AIW));
                        let AJU = ((((AIK * AJR) * AIU) + (AIV * AJS)) - (AIT * AJR)) + (((AII * AIW) + (AIX * AIH)) * HA);
                        let AJV = ((DB * (AIJ * AIU)) - AIS) - AQ;
                        let AJW = AJV * AJP;
                        let AJX = (((((AIK * AIU) + (AIV * AIJ)) * DB) - AIT) * AJP) + (AJQ * AJV);
                        let AJY = AJW * AJW;
                        let AJZ = AJX * AJW;
                        let AKA = AJZ + AJZ;
                        let AKB = if AJW > DX { 1.0 } else { 0.0 };
                        let AKI;
                        let AKJ;
                        if AKB != 0.0 {
                            let AKC = AQ + (AAH * AJW);
                            let AKD = AQ / AKC;
                            let AKE = (((AJX * AAH) * AKD) * AP) / AKC;
                            AKI = AKD;
                            AKJ = AKE;
                        } else {
                            let AKF = AQ - (AAH * AJW);
                            let AKG = AQ / AKF;
                            let AKH = ((((AJX * AAH) * AP) * AKG) * AP) / AKF;
                            AKI = AKG;
                            AKJ = AKH;
                        }
                        let AKK = (-AJY) + AJT;
                        let AKL = (AKA * AP) + AJU;
                        let AKM = if AKK > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AKX;
                        let AKY;
                        if AKM != 0.0 {
                            let AKN = AKK.exp();
                            let AKO = AKL * AKN;
                            AKX = AKN;
                            AKY = AKO;
                        } else {
                            let AKP = -2.3025850929940458e2f64 - AKK;
                            let AKQ = AKL * AP;
                            let AKR = -2.3025850929940458e2f64 - AKK;
                            let AKS = AQ + ((-2.3025850929940458e2f64 - AKK) * IJ);
                            let AKT = AQ + (HA * (AKR * AKS));
                            let AKU = AQ + (AKP * AKT);
                            let AKV = AS / AKU;
                            let AKW = ((((AKQ * AKT) + ((((AKQ * AKS) + ((AKQ * IJ) * AKR)) * HA) * AKP)) * AKV) * AP) / AKU;
                            AKX = AKV;
                            AKY = AKW;
                        }
                        let AKZ = AKI * AKI;
                        let ALA = AKJ * AKI;
                        let ALB = ALA + ALA;
                        let ALC = ((ABF * AKI) + (ABJ * AKZ)) + (ABK * (AKZ * AKI));
                        let ALD = ALC * AKX;
                        let ALE = ((((AKJ * ABF) + (ALB * ABJ)) + (((ALB * AKI) + (AKJ * AKZ)) * ABK)) * AKX) + (AKY * ALC);
                        let ALG;
                        let ALH;
                        if AKB != 0.0 {
                            ALG = ALD;
                            ALH = ALE;
                        } else {
                            let ALF = if AJT > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ALZ;
                            let AMA;
                            if ALF != 0.0 {
                                let ALP = AJT.exp();
                                let ALQ = AJU * ALP;
                                ALZ = ALP;
                                AMA = ALQ;
                            } else {
                                let ALR = -2.3025850929940458e2f64 - AJT;
                                let ALS = AJU * AP;
                                let ALT = -2.3025850929940458e2f64 - AJT;
                                let ALU = AQ + ((-2.3025850929940458e2f64 - AJT) * IJ);
                                let ALV = AQ + (HA * (ALT * ALU));
                                let ALW = AQ + (ALR * ALV);
                                let ALX = AS / ALW;
                                let ALY = ((((ALS * ALV) + ((((ALS * ALU) + ((ALS * IJ) * ALT)) * HA) * ALR)) * ALX) * AP) / ALW;
                                ALZ = ALX;
                                AMA = ALY;
                            }
                            let AMB = (DB * ALZ) - ALD;
                            let AMC = (AMA * DB) - ALE;
                            ALG = AMB;
                            ALH = AMC;
                        }
                        let ALI = (AJR * ALG) / AJP;
                        let ALK = ALJ * ALI;
                        let ALL = AGS * ALK;
                        let ALN = ALM * (ALL * AJM);
                        let ALO = ((((AGX * ALK) + (((((ALH * AJR) - (AJQ * ALI)) / AJP) * ALJ) * AGS)) * AJM) + (AJN * ALL)) * ALM;
                        AIZ = ALN;
                        AJA = ALO;
                    }
                    let AME;
                    let AMF;
                    if AJB != 0.0 {
                        AME = DX;
                        AMF = DY;
                    } else {
                        let AMN;
                        let AMO;
                        if AMD != 0.0 {
                            let AMI = ((AMH - GN) * AHM).sqrt();
                            let AMJ = ((GW * AP) * AHM) * (CY / (CX * AMI));
                            AMN = AMI;
                            AMO = AMJ;
                        } else {
                            let AMK = (AMH - GN) * AHM;
                            let AML = AMK.powf(AHQ);
                            let AMM = ((GW * AP) * AHM) * (AHQ * (AMK.powf(staged[294])));
                            AMN = AML;
                            AMO = AMM;
                        }
                        let AMQ = ((AMH - GN) * AMP) / AMN;
                        let AMS = AMR * AMQ;
                        let AMT = ((((GW * AP) * AMP) - (AMO * AMQ)) / AMN) * AMR;
                        let AMU = staged[241] / AMS;
                        let AMV = ((AMT * AMU) * AP) / AMS;
                        let AMW = if (AMU.abs()) < HD { 1.0 } else { 0.0 };
                        let ANA;
                        let ANB;
                        if AMW != 0.0 {
                            let AMX = AMU.exp();
                            let AMY = AMV * AMX;
                            ANA = AMX;
                            ANB = AMY;
                        } else {
                            let AMZ = if AMU < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ANU;
                            let ANV;
                            if AMZ != 0.0 {
                                let ANH = -2.3025850929940458e2f64 - AMU;
                                let ANI = AMV * AP;
                                let ANJ = -2.3025850929940458e2f64 - AMU;
                                let ANK = AQ + ((-2.3025850929940458e2f64 - AMU) * IJ);
                                let ANL = AQ + (HA * (ANJ * ANK));
                                let ANM = AQ + (ANH * ANL);
                                let ANN = AS / ANM;
                                let ANO = ((((ANI * ANL) + ((((ANI * ANK) + ((ANI * IJ) * ANJ)) * HA) * ANH)) * ANN) * AP) / ANM;
                                ANU = ANN;
                                ANV = ANO;
                            } else {
                                let ANP = AMU - HD;
                                let ANQ = AQ + (ANP * IJ);
                                let ANR = AQ + (HA * (ANP * ANQ));
                                let ANS = IS * (AQ + (ANP * ANR));
                                let ANT = ((AMV * ANR) + ((((AMV * ANQ) + ((AMV * IJ) * ANP)) * HA) * ANP)) * IS;
                                ANU = ANS;
                                ANV = ANT;
                            }
                            ANA = ANU;
                            ANB = ANV;
                        }
                        let ANC = G * AMS;
                        let AND = ANC * AMS;
                        let ANF = ANE * (AND * ANA);
                        let ANG = ((((((J * AMS) + (AMT * G)) * AMS) + (AMT * ANC)) * ANA) + (ANB * AND)) * ANE;
                        AME = ANF;
                        AMF = ANG;
                    }
                    let ANX;
                    let ANY;
                    if AMG != 0.0 {
                        ANX = AQ;
                        ANY = DY;
                    } else {
                        let ANW = if GO > staged[243] { 1.0 } else { 0.0 };
                        let AOK;
                        let AOL;
                        if ANW != 0.0 {
                            let AOG = if AOF == JD { 1.0 } else { 0.0 };
                            let AOZ;
                            let APA;
                            if AOG != 0.0 {
                                let AON = GO * AOM;
                                let AOO = AON.abs();
                                let AOP = (GX * AOM) * ((CX * (if AON >= YX { 1.0 } else { 0.0 })) - CY);
                                let AOQ = AOO * AOO;
                                let AOR = AOP * AOO;
                                let AOS = AOQ * AOO;
                                let AOT = AOS * AOO;
                                let AOU = ((((AOR + AOR) * AOO) + (AOP * AOQ)) * AOO) + (AOP * AOS);
                                AOZ = AOT;
                                APA = AOU;
                            } else {
                                let AOV = GO * AOM;
                                let AOW = AOV.abs();
                                let AOX = AOW.powf(AOF);
                                let AOY = ((GX * AOM) * ((CX * (if AOV >= YX { 1.0 } else { 0.0 })) - CY)) * (AOF * (AOW.powf((AOF - CY))));
                                AOZ = AOX;
                                APA = AOY;
                            }
                            let APB = AQ - AOZ;
                            let APC = AQ / APB;
                            let APD = (((APA * AP) * APC) * AP) / APB;
                            AOK = APC;
                            AOL = APD;
                        } else {
                            let AOI = GX * AOH;
                            let AOJ = staged[97] + ((GO + (AEQ * staged[245])) * AOH);
                            AOK = AOJ;
                            AOL = AOI;
                        }
                        ANX = AOK;
                        ANY = AOL;
                    }
                    let ANZ = ((AGF + AGT) + AIZ) + AME;
                    let AOA = ANZ * ANX;
                    let AOB = ((((AGG + AGY) + AJA) + AMF) * ANX) + (ANY * ANZ);
                    let AOC = (AGT + AIZ) + AME;
                    let AOD = AOC * ANX;
                    let AOE = (((AGY + AJA) + AMF) * ANX) + (ANY * AOC);
                    AFP = AOA;
                    AFQ = AOD;
                    AFR = AGC;
                    AFS = AOB;
                    AFT = AOE;
                    AFU = AGD;
                }
                let APF;
                let APG;
                let APH;
                let API;
                let APJ;
                let APK;
                if C != 0.0 {
                    APF = DX;
                    APG = DX;
                    APH = DX;
                    API = DY;
                    APJ = DY;
                    APK = DY;
                } else {
                    let APU;
                    let APV;
                    if APE != 0.0 {
                        let APP = (AQ - (GI * FK)).sqrt();
                        let APQ = ((GR * FK) * AP) * (CY / (CX * APP));
                        APU = APP;
                        APV = APQ;
                    } else {
                        let APR = AQ - (GI * FK);
                        let APS = APR.powf(FO);
                        let APT = ((GR * FK) * AP) * (FO * (APR.powf(staged[295])));
                        APU = APS;
                        APV = APT;
                    }
                    let APW = (FT * (AQ - APU)) + (FU * (G - GI));
                    let APX = ((APV * AP) * FT) + ((J - GR) * FU);
                    let APZ = APY * GQ;
                    let AQA = GZ * APY;
                    let AQJ;
                    let AQK;
                    let AQL;
                    let AQM;
                    let AQN;
                    let AQO;
                    let AQP;
                    let AQQ;
                    let AQR;
                    let AQS;
                    if AQB != 0.0 {
                        AQJ = DX;
                        AQK = DX;
                        AQL = DX;
                        AQM = DX;
                        AQN = DX;
                        AQO = DY;
                        AQP = DY;
                        AQQ = DY;
                        AQR = DY;
                        AQS = DY;
                    } else {
                        let AQC = staged[248] - GK;
                        let AQD = GT * AP;
                        let AQE = GL / AQC;
                        let AQF = (AQ - AQE).sqrt();
                        let AQG = AQ - AQF;
                        let AQH = ((((GU - (AQD * AQE)) / AQC) * AP) * (CY / (CX * AQF))) * AP;
                        let ARC;
                        let ARD;
                        if AQI != 0.0 {
                            ARC = DX;
                            ARD = DY;
                        } else {
                            let AQU = AQG * AQG;
                            let AQV = AQH * AQG;
                            let AQW = AQG.ln();
                            let AQX = AQ - AQG;
                            let AQY = (AQU * AQW) / AQX;
                            let ARA = (AQY + AQG) * AQZ;
                            let ARB = ((((((AQV + AQV) * AQW) + ((AQH * (CY / AQG)) * AQU)) - ((AQH * AP) * AQY)) / AQX) + AQH) * AQZ;
                            ARC = ARA;
                            ARD = ARB;
                        }
                        let ARE = AQG + ARC;
                        let ARF = AQH + ARD;
                        let ARN;
                        let ARO;
                        if AQI != 0.0 {
                            let ARH = (AQC * ARG).sqrt();
                            let ARI = (AQD * ARG) * (CY / (CX * ARH));
                            ARN = ARH;
                            ARO = ARI;
                        } else {
                            let ARJ = AQC * ARG;
                            let ARL = ARJ.powf(ARK);
                            let ARM = (AQD * ARG) * (ARK * (ARJ.powf(staged[296])));
                            ARN = ARL;
                            ARO = ARM;
                        }
                        let ARQ = ARP * ARN;
                        let ARR = ARO * ARP;
                        let ARS = GM - AQ;
                        let ARU = ART * (ARS * ARQ);
                        let ARV = ((GV * ARQ) + (ARR * ARS)) * ART;
                        let ARX = ARW * (ARU * ARE);
                        let ARY = ((ARV * ARE) + (ARF * ARU)) * ARW;
                        AQJ = ARQ;
                        AQK = AQC;
                        AQL = ARE;
                        AQM = ARU;
                        AQN = ARX;
                        AQO = ARR;
                        AQP = AQD;
                        AQQ = ARF;
                        AQR = ARV;
                        AQS = ARY;
                    }
                    let AST;
                    let ASU;
                    if AQT != 0.0 {
                        AST = DX;
                        ASU = DY;
                    } else {
                        let ARZ = (AQJ * FO) / AQK;
                        let ASB = ASA * ARZ;
                        let ASC = (((AQO * FO) - (AQP * ARZ)) / AQK) * ASA;
                        let ASD = staged[252] / ASB;
                        let ASE = ((ASC * ASD) * AP) / ASB;
                        let ASF = ASD * ASD;
                        let ASG = ASE * ASD;
                        let ASH = ASF * ASF;
                        let ASI = (ASG + ASG) * ASF;
                        let ASJ = ASI + ASI;
                        let ASK = ASH + AQ;
                        let ASL = ASH / ASK;
                        let ASM = ASL.sqrt();
                        let ASN = ((ASJ - (ASJ * ASL)) / ASK) * (CY / (CX * ASM));
                        let ASO = (ASM.abs()).sqrt();
                        let ASP = (ASN * ((CX * (if ASM >= YX { 1.0 } else { 0.0 })) - CY)) * (CY / (CX * ASO));
                        let ASQ = ASM * ASO;
                        let ASR = (ASN * ASO) + (ASP * ASM);
                        let ATD;
                        let ATE;
                        if ASS != 0.0 {
                            let ASW = AQ + (ASB * ASQ);
                            let ASX = AQ / ASW;
                            let ASY = ((((ASC * ASQ) + (ASR * ASB)) * ASX) * AP) / ASW;
                            ATD = ASX;
                            ATE = ASY;
                        } else {
                            let ASZ = AQ + (ASB * ASQ);
                            let ATB = ASZ.powf(ATA);
                            let ATC = ((ASC * ASQ) + (ASR * ASB)) * (ATA * (ASZ.powf(staged[297])));
                            ATD = ATB;
                            ATE = ATC;
                        }
                        let ATF = AQL + ATD;
                        let ATG = (AQL * ATD) / ATF;
                        let ATH = (((AQQ * ATD) + (ATE * AQL)) - ((AQQ + ATE) * ATG)) / ATF;
                        let ATI = ASB / ASO;
                        let ATJ = (ZT * ATI).sqrt();
                        let ATK = (((ASC - (ASP * ATI)) / ASO) * ZT) * (CY / (CX * ATJ));
                        let ATM = ATL * ASD;
                        let ATN = ((ATM * ASO) - (ATL * ASM)) + (HA * (ASB * ASQ));
                        let ATO = ((((ASE * ATL) * ASO) + (ASP * ATM)) - (ASN * ATL)) + (((ASC * ASQ) + (ASR * ASB)) * HA);
                        let ATP = ((DB * (ASD * ASO)) - ASM) - AQ;
                        let ATQ = ATP * ATJ;
                        let ATR = (((((ASE * ASO) + (ASP * ASD)) * DB) - ASN) * ATJ) + (ATK * ATP);
                        let ATS = ATQ * ATQ;
                        let ATT = ATR * ATQ;
                        let ATU = ATT + ATT;
                        let ATV = if ATQ > DX { 1.0 } else { 0.0 };
                        let AUC;
                        let AUD;
                        if ATV != 0.0 {
                            let ATW = AQ + (AAH * ATQ);
                            let ATX = AQ / ATW;
                            let ATY = (((ATR * AAH) * ATX) * AP) / ATW;
                            AUC = ATX;
                            AUD = ATY;
                        } else {
                            let ATZ = AQ - (AAH * ATQ);
                            let AUA = AQ / ATZ;
                            let AUB = ((((ATR * AAH) * AP) * AUA) * AP) / ATZ;
                            AUC = AUA;
                            AUD = AUB;
                        }
                        let AUE = (-ATS) + ATN;
                        let AUF = (ATU * AP) + ATO;
                        let AUG = if AUE > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AUR;
                        let AUS;
                        if AUG != 0.0 {
                            let AUH = AUE.exp();
                            let AUI = AUF * AUH;
                            AUR = AUH;
                            AUS = AUI;
                        } else {
                            let AUJ = -2.3025850929940458e2f64 - AUE;
                            let AUK = AUF * AP;
                            let AUL = -2.3025850929940458e2f64 - AUE;
                            let AUM = AQ + ((-2.3025850929940458e2f64 - AUE) * IJ);
                            let AUN = AQ + (HA * (AUL * AUM));
                            let AUO = AQ + (AUJ * AUN);
                            let AUP = AS / AUO;
                            let AUQ = ((((AUK * AUN) + ((((AUK * AUM) + ((AUK * IJ) * AUL)) * HA) * AUJ)) * AUP) * AP) / AUO;
                            AUR = AUP;
                            AUS = AUQ;
                        }
                        let AUT = AUC * AUC;
                        let AUU = AUD * AUC;
                        let AUV = AUU + AUU;
                        let AUW = ((ABF * AUC) + (ABJ * AUT)) + (ABK * (AUT * AUC));
                        let AUX = AUW * AUR;
                        let AUY = ((((AUD * ABF) + (AUV * ABJ)) + (((AUV * AUC) + (AUD * AUT)) * ABK)) * AUR) + (AUS * AUW);
                        let AVA;
                        let AVB;
                        if ATV != 0.0 {
                            AVA = AUX;
                            AVB = AUY;
                        } else {
                            let AUZ = if ATN > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AVT;
                            let AVU;
                            if AUZ != 0.0 {
                                let AVJ = ATN.exp();
                                let AVK = ATO * AVJ;
                                AVT = AVJ;
                                AVU = AVK;
                            } else {
                                let AVL = -2.3025850929940458e2f64 - ATN;
                                let AVM = ATO * AP;
                                let AVN = -2.3025850929940458e2f64 - ATN;
                                let AVO = AQ + ((-2.3025850929940458e2f64 - ATN) * IJ);
                                let AVP = AQ + (HA * (AVN * AVO));
                                let AVQ = AQ + (AVL * AVP);
                                let AVR = AS / AVQ;
                                let AVS = ((((AVM * AVP) + ((((AVM * AVO) + ((AVM * IJ) * AVN)) * HA) * AVL)) * AVR) * AP) / AVQ;
                                AVT = AVR;
                                AVU = AVS;
                            }
                            let AVV = (DB * AVT) - AUX;
                            let AVW = (AVU * DB) - AUY;
                            AVA = AVV;
                            AVB = AVW;
                        }
                        let AVC = (ATL * AVA) / ATJ;
                        let AVE = AVD * AVC;
                        let AVF = AQM * AVE;
                        let AVH = AVG * (AVF * ATG);
                        let AVI = ((((AQR * AVE) + (((((AVB * ATL) - (ATK * AVC)) / ATJ) * AVD) * AQM)) * ATG) + (ATH * AVF)) * AVG;
                        AST = AVH;
                        ASU = AVI;
                    }
                    let AVY;
                    let AVZ;
                    if ASV != 0.0 {
                        AVY = DX;
                        AVZ = DY;
                    } else {
                        let AWH;
                        let AWI;
                        if AVX != 0.0 {
                            let AWC = ((AWB - GN) * ARG).sqrt();
                            let AWD = ((GW * AP) * ARG) * (CY / (CX * AWC));
                            AWH = AWC;
                            AWI = AWD;
                        } else {
                            let AWE = (AWB - GN) * ARG;
                            let AWF = AWE.powf(ARK);
                            let AWG = ((GW * AP) * ARG) * (ARK * (AWE.powf(staged[298])));
                            AWH = AWF;
                            AWI = AWG;
                        }
                        let AWK = ((AWB - GN) * AWJ) / AWH;
                        let AWM = AWL * AWK;
                        let AWN = ((((GW * AP) * AWJ) - (AWI * AWK)) / AWH) * AWL;
                        let AWO = staged[255] / AWM;
                        let AWP = ((AWN * AWO) * AP) / AWM;
                        let AWQ = if (AWO.abs()) < HD { 1.0 } else { 0.0 };
                        let AWU;
                        let AWV;
                        if AWQ != 0.0 {
                            let AWR = AWO.exp();
                            let AWS = AWP * AWR;
                            AWU = AWR;
                            AWV = AWS;
                        } else {
                            let AWT = if AWO < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AXO;
                            let AXP;
                            if AWT != 0.0 {
                                let AXB = -2.3025850929940458e2f64 - AWO;
                                let AXC = AWP * AP;
                                let AXD = -2.3025850929940458e2f64 - AWO;
                                let AXE = AQ + ((-2.3025850929940458e2f64 - AWO) * IJ);
                                let AXF = AQ + (HA * (AXD * AXE));
                                let AXG = AQ + (AXB * AXF);
                                let AXH = AS / AXG;
                                let AXI = ((((AXC * AXF) + ((((AXC * AXE) + ((AXC * IJ) * AXD)) * HA) * AXB)) * AXH) * AP) / AXG;
                                AXO = AXH;
                                AXP = AXI;
                            } else {
                                let AXJ = AWO - HD;
                                let AXK = AQ + (AXJ * IJ);
                                let AXL = AQ + (HA * (AXJ * AXK));
                                let AXM = IS * (AQ + (AXJ * AXL));
                                let AXN = ((AWP * AXL) + ((((AWP * AXK) + ((AWP * IJ) * AXJ)) * HA) * AXJ)) * IS;
                                AXO = AXM;
                                AXP = AXN;
                            }
                            AWU = AXO;
                            AWV = AXP;
                        }
                        let AWW = G * AWM;
                        let AWX = AWW * AWM;
                        let AWZ = AWY * (AWX * AWU);
                        let AXA = ((((((J * AWM) + (AWN * G)) * AWM) + (AWN * AWW)) * AWU) + (AWV * AWX)) * AWY;
                        AVY = AWZ;
                        AVZ = AXA;
                    }
                    let AXR;
                    let AXS;
                    if AWA != 0.0 {
                        AXR = AQ;
                        AXS = DY;
                    } else {
                        let AXQ = if GO > staged[257] { 1.0 } else { 0.0 };
                        let AYE;
                        let AYF;
                        if AXQ != 0.0 {
                            let AYA = if AXZ == JD { 1.0 } else { 0.0 };
                            let AYT;
                            let AYU;
                            if AYA != 0.0 {
                                let AYH = GO * AYG;
                                let AYI = AYH.abs();
                                let AYJ = (GX * AYG) * ((CX * (if AYH >= YX { 1.0 } else { 0.0 })) - CY);
                                let AYK = AYI * AYI;
                                let AYL = AYJ * AYI;
                                let AYM = AYK * AYI;
                                let AYN = AYM * AYI;
                                let AYO = ((((AYL + AYL) * AYI) + (AYJ * AYK)) * AYI) + (AYJ * AYM);
                                AYT = AYN;
                                AYU = AYO;
                            } else {
                                let AYP = GO * AYG;
                                let AYQ = AYP.abs();
                                let AYR = AYQ.powf(AXZ);
                                let AYS = ((GX * AYG) * ((CX * (if AYP >= YX { 1.0 } else { 0.0 })) - CY)) * (AXZ * (AYQ.powf((AXZ - CY))));
                                AYT = AYR;
                                AYU = AYS;
                            }
                            let AYV = AQ - AYT;
                            let AYW = AQ / AYV;
                            let AYX = (((AYU * AP) * AYW) * AP) / AYV;
                            AYE = AYW;
                            AYF = AYX;
                        } else {
                            let AYC = GX * AYB;
                            let AYD = staged[111] + ((GO + (AEQ * staged[259])) * AYB);
                            AYE = AYD;
                            AYF = AYC;
                        }
                        AXR = AYE;
                        AXS = AYF;
                    }
                    let AXT = ((APZ + AQN) + AST) + AVY;
                    let AXU = AXT * AXR;
                    let AXV = ((((AQA + AQS) + ASU) + AVZ) * AXR) + (AXS * AXT);
                    let AXW = (AQN + AST) + AVY;
                    let AXX = AXW * AXR;
                    let AXY = (((AQS + ASU) + AVZ) * AXR) + (AXS * AXW);
                    APF = AXU;
                    APG = AXX;
                    APH = APW;
                    API = AXV;
                    APJ = AXY;
                    APK = APX;
                }
                let APL = ((AE * VS) + (AF * AFP)) + (AG * APF);
                let APM = ((VV * AE) + (AFS * AF)) + (API * AG);
                let APN = ((AE * VT) + (AF * AFQ)) + (AG * APG);
                let APO = ((VW * AE) + (AFT * AF)) + (APJ * AG);
                S = VU;
                T = AFR;
                U = APH;
                V = GJ;
                W = APL;
                X = APN;
                Y = VX;
                Z = AFU;
                AA = APK;
                AB = GS;
                AC = APM;
                AD = APO;
            }
            let AH = ((AE * S) + (AF * T)) + (AG * U);
            let AI = ((Y * AE) + (Z * AF)) + (AA * AG);
            let AK = F - AJ;
            let AM = Lanes([0.0, I]) - Lanes([AL, 0.0]);
            let AN = E - AJ;
            let AO = if (if AN > staged[261] { 1.0 } else { 0.0 }) != 0.0 && staged[262] != 0.0 { 1.0 } else { 0.0 };
            let AYY = if (if AN < staged[263] { 1.0 } else { 0.0 }) != 0.0 && staged[264] != 0.0 { 1.0 } else { 0.0 };
            let AZI;
            let AZJ;
            let AZK;
            let AZL;
            let AZM;
            let AZN;
            let AZO;
            let AZP;
            let AZQ;
            let AZR;
            let AZS;
            let AZT;
            let AZU;
            let AZV;
            if AYZ != 0.0 {
                let BAC;
                let BAD;
                if AZA != 0.0 {
                    let BAA = (J * IY) * AP;
                    let BAB = (HK - ((IY * (G - AZZ)) + HJ)) - JB;
                    let BAG = BAA * BAB;
                    let BAH = ((BAB * BAB) + staged[266]).sqrt();
                    let BAI = ((BAA + ((BAG + BAG) * (CY / (CX * BAH)))) * HA) * AP;
                    let BAJ = ((HK - (HA * (BAB + BAH))) - HJ) - JB;
                    let BAK = BAI * BAJ;
                    let BAL = ((BAJ * BAJ) + staged[267]).sqrt();
                    let BAM = (BAI + ((BAK + BAK) * (CY / (CX * BAL)))) * HA;
                    let BAN = HJ + (HA * (BAJ + BAL));
                    BAC = BAN;
                    BAD = BAM;
                } else {
                    BAC = HJ;
                    BAD = DY;
                }
                let BAF = if (G - BAE) > DX { 1.0 } else { 0.0 };
                let BAV;
                let BAW;
                if BAF != 0.0 {
                    let BAO = G / BAC;
                    let BAP = BAE / BAC;
                    let BAR = BAQ * HK;
                    let BAS = K * ((BAO - BAP) + ((AZZ * (BAC - BAQ)) / BAR));
                    let BAT = ((((J - (BAD * BAO)) / BAC) - (((BAD * BAP) * AP) / BAC)) + ((BAD * AZZ) / BAR)) * K;
                    let BAU = if (BAS.abs()) < HD { 1.0 } else { 0.0 };
                    let BBC;
                    let BBD;
                    if BAU != 0.0 {
                        let BAZ = BAS.exp();
                        let BBA = BAT * BAZ;
                        BBC = BAZ;
                        BBD = BBA;
                    } else {
                        let BBB = if BAS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BBQ;
                        let BBR;
                        if BBB != 0.0 {
                            let BBE = -2.3025850929940458e2f64 - BAS;
                            let BBF = -2.3025850929940458e2f64 - BAS;
                            let BBG = AQ + ((-2.3025850929940458e2f64 - BAS) * IJ);
                            let BBH = AQ + (HA * (BBF * BBG));
                            let BBI = AQ + (BBE * BBH);
                            let BBJ = AS / BBI;
                            let BBK = (((((BAT * AP) * BBH) + (((((BAT * AP) * BBG) + (((BAT * AP) * IJ) * BBF)) * HA) * BBE)) * BBJ) * AP) / BBI;
                            BBQ = BBJ;
                            BBR = BBK;
                        } else {
                            let BBL = BAS - HD;
                            let BBM = AQ + (BBL * IJ);
                            let BBN = AQ + (HA * (BBL * BBM));
                            let BBO = IS * (AQ + (BBL * BBN));
                            let BBP = ((BAT * BBN) + ((((BAT * BBM) + ((BAT * IJ) * BBL)) * HA) * BBL)) * IS;
                            BBQ = BBO;
                            BBR = BBP;
                        }
                        BBC = BBQ;
                        BBD = BBR;
                    }
                    BAV = BBC;
                    BAW = BBD;
                } else {
                    BAV = AQ;
                    BAW = DY;
                }
                let BAY = if BAX != 0.0 || (if G < IX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BCH;
                let BCI;
                if BAY != 0.0 {
                    let BBT = V * BBS;
                    let BBU = AB * BBS;
                    BCH = BBT;
                    BCI = BBU;
                } else {
                    let BBV = V * BBS;
                    let BBX = -BBW;
                    let BBY = G - IX;
                    let BBZ = BBX * BBY;
                    let BCD = (BCC * ((BCA / BCB).ln())).exp();
                    let BCE = ((BBZ * BBY) * BCD).exp();
                    let BCF = BBV * BCE;
                    let BCG = ((AB * BBS) * BCE) + ((((((J * BBX) * BBY) + (J * BBZ)) * BCD) * BCE) * BBV);
                    BCH = BCF;
                    BCI = BCG;
                }
                let BCK = if BCH > BCJ { 1.0 } else { 0.0 };
                let BCL;
                let BCM;
                if BCK != 0.0 {
                    BCL = BCJ;
                    BCM = DY;
                } else {
                    BCL = BCH;
                    BCM = BCI;
                }
                let BCP = BCO * ((BCN * BCL) - BCN);
                let BCQ = (BCM * BCN) * BCO;
                let BDC;
                let BDD;
                let BDE;
                let BDF;
                let BDG;
                let BDH;
                if BCR != 0.0 {
                    let BCT = BCQ * BCS;
                    let BCX = (BCU - (BCP * BCS)) / BCW;
                    let BCY = (Lanes([0.0, 0.0, BCV]) - Lanes([BCT[0], BCT[1], 0.0])) / BCW;
                    let BCZ = BCU / BCS;
                    let BDA = Lanes([0.0, 0.0, (BCV / BCS)]);
                    BDC = BCZ;
                    BDD = BCX;
                    BDE = BCU;
                    BDF = BDA;
                    BDG = BCY;
                    BDH = BCV;
                } else {
                    let BDB = Lanes([BCQ[0], BCQ[1], 0.0]);
                    BDC = BCP;
                    BDD = DX;
                    BDE = DX;
                    BDF = BDB;
                    BDG = AZC;
                    BDH = AZD;
                }
                let BDI = if BAX != 0.0 || (if G < AZZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BDT;
                let BDU;
                if BDI != 0.0 {
                    let BDJ = BAV * BBS;
                    let BDK = BAW * BBS;
                    BDT = BDJ;
                    BDU = BDK;
                } else {
                    let BDL = BAV * BBS;
                    let BDM = -BBW;
                    let BDN = G - AZZ;
                    let BDO = BDM * BDN;
                    let BDP = (BCC * ((BCA / BCB).ln())).exp();
                    let BDQ = ((BDO * BDN) * BDP).exp();
                    let BDR = BDL * BDQ;
                    let BDS = ((BAW * BBS) * BDQ) + ((((((J * BDM) * BDN) + (J * BDO)) * BDP) * BDQ) * BDL);
                    BDT = BDR;
                    BDU = BDS;
                }
                let BDV = if BDT > BCJ { 1.0 } else { 0.0 };
                let BDW;
                let BDX;
                if BDV != 0.0 {
                    BDW = BCJ;
                    BDX = DY;
                } else {
                    BDW = BDT;
                    BDX = BDU;
                }
                let BDY = BCO * ((BCN * BDW) - BCN);
                let BDZ = (BDX * BCN) * BCO;
                let BEJ;
                let BEK;
                let BEL;
                let BEM;
                let BEN;
                let BEO;
                if BCR != 0.0 {
                    let BEB = BDZ * BEA;
                    let BEE = (BEC - (BDY * BEA)) / BCW;
                    let BEF = (Lanes([0.0, 0.0, BED]) - Lanes([BEB[0], BEB[1], 0.0])) / BCW;
                    let BEG = BEC / BEA;
                    let BEH = Lanes([0.0, 0.0, (BED / BEA)]);
                    BEJ = BEG;
                    BEK = BEE;
                    BEL = BEC;
                    BEM = BEH;
                    BEN = BEF;
                    BEO = BED;
                } else {
                    let BEI = Lanes([BDZ[0], BDZ[1], 0.0]);
                    BEJ = BDY;
                    BEK = DX;
                    BEL = DX;
                    BEM = BEI;
                    BEN = AZE;
                    BEO = AZF;
                }
                let BEP = 6e-1f64 - G;
                let BEQ = J * AP;
                let BER = BEQ * BEP;
                let BES = ((BEP * BEP) + 4e-6f64).sqrt();
                let BET = HA * (BEP + BES);
                let BEU = (BEQ + ((BER + BER) * (CY / (CX * BES)))) * HA;
                let BEV = if BET < DX { 1.0 } else { 0.0 };
                let BEW;
                let BEX;
                if BEV != 0.0 {
                    BEW = DX;
                    BEX = DY;
                } else {
                    BEW = BET;
                    BEX = BEU;
                }
                let BFA = ((BEY * BEW) / BEZ).sqrt();
                let BFC = (((BEX * BEY) / BEZ) * (CY / (CX * BFA))) * AP;
                let BFD = (BFB - BFA) - 1e-7f64;
                let BFE = BFC * BFD;
                let BFF = ((BFD * BFD) + staged[279]).sqrt();
                let BFG = BFB - (HA * (BFD + BFF));
                let BFH = ((BFC + ((BFE + BFE) * (CY / (CX * BFF)))) * HA) * AP;
                let BFT;
                let BFU;
                let BFV;
                let BFW;
                let BFX;
                let BFY;
                if BFI != 0.0 {
                    let BFK = BFH * BFJ;
                    let BFO = (BFL - (BFG * BFJ)) / BFN;
                    let BFP = (Lanes([0.0, 0.0, BFM]) - Lanes([BFK[0], BFK[1], 0.0])) / BFN;
                    let BFQ = BFL / BFJ;
                    let BFR = Lanes([0.0, 0.0, (BFM / BFJ)]);
                    BFT = BFQ;
                    BFU = BFO;
                    BFV = BFL;
                    BFW = BFR;
                    BFX = BFP;
                    BFY = BFM;
                } else {
                    let BFS = Lanes([BFH[0], BFH[1], 0.0]);
                    BFT = BFG;
                    BFU = DX;
                    BFV = DX;
                    BFW = BFS;
                    BFX = AZG;
                    BFY = AZH;
                }
                let BGA = BFZ * BDC;
                let BGB = BFW * AP;
                let BGC = ((-BFT) / BFZ).exp();
                let BGD = staged[283] - BGC;
                let BGE = (BDF * BFZ) * BGD;
                let BGF = (((BGB / BFZ) * BGC) * AP) * BGA;
                let BGG = Lanes([BGE[0], BGE[1], BGE[2], 0.0]) + Lanes([BGF[0], BGF[1], 0.0, BGF[2]]);
                let BGH = BFZ * BEJ;
                let BGI = ((-(BFB - BFT)) / BFZ).exp();
                let BGJ = BGI - AQ;
                let BGK = (BEM * BFZ) * BGJ;
                let BGL = (((BGB * AP) / BFZ) * BGI) * BGH;
                let BGM = Lanes([BGK[0], BGK[1], BGK[2], 0.0]) + Lanes([BGL[0], BGL[1], 0.0, BGL[2]]);
                let BGN = AH + (-((staged[284] + (BGA * BGD)) + (BGH * BGJ)));
                let BGO = Lanes([AI[0], AI[1], 0.0, 0.0, 0.0]) + ((Lanes([BGG[0], BGG[1], BGG[2], 0.0, BGG[3]]) + Lanes([BGM[0], BGM[1], 0.0, BGM[2], BGM[3]])) * AP);
                AZI = BDD;
                AZJ = BDE;
                AZK = BEK;
                AZL = BEL;
                AZM = BFU;
                AZN = BFV;
                AZO = BGN;
                AZP = BDG;
                AZQ = BDH;
                AZR = BEN;
                AZS = BEO;
                AZT = BFX;
                AZU = BFY;
                AZV = BGO;
            } else {
                let AZB = Lanes([AI[0], AI[1], 0.0, 0.0, 0.0]);
                AZI = DX;
                AZJ = DX;
                AZK = DX;
                AZL = DX;
                AZM = DX;
                AZN = DX;
                AZO = AH;
                AZP = AZC;
                AZQ = AZD;
                AZR = AZE;
                AZS = AZF;
                AZT = AZG;
                AZU = AZH;
                AZV = AZB;
            }
            let AZW = W - X;
            let AZX = AC - AD;
            let BGP = ctx.simparam_or("gmin", DX);
            let BGQ = BGP * G;
            let BGR = J * BGP;
            let BGW;
            let BGX;
            if AZY != 0.0 {
                let BGT = AK / BGS;
                let BGU = AM / BGS;
                BGW = BGT;
                BGX = BGU;
            } else {
                BGW = DX;
                BGX = BGV;
            }
            let BHJ;
            let BHK;
            let BHL;
            let BHM;
            let BHN;
            let BHO;
            let BHP;
            let BHQ;
            if BGY != 0.0 {
                let BHB = BHA * (AZI + ddt(80339, AZJ));
                let BHC = (AZP + Lanes([0.0, 0.0, (AZQ * BGZ)])) * BHA;
                let BHD = BHA * AZJ;
                let BHE = AZQ * BHA;
                let BHF = BHA * (AZK + ddt(80345, AZL));
                let BHG = (AZR + Lanes([0.0, 0.0, (AZS * BGZ)])) * BHA;
                let BHH = BHA * AZL;
                let BHI = AZS * BHA;
                BHJ = BHB;
                BHK = BHF;
                BHL = BHD;
                BHM = BHH;
                BHN = BHC;
                BHO = BHG;
                BHP = BHE;
                BHQ = BHI;
            } else {
                BHJ = DX;
                BHK = DX;
                BHL = DX;
                BHM = DX;
                BHN = AZC;
                BHO = AZE;
                BHP = AZD;
                BHQ = AZF;
            }
            let BHX;
            let BHY;
            let BHZ;
            let BIA;
            if BHR != 0.0 {
                let BHT = BHS * (AZM + ddt(80360, AZN));
                let BHU = (AZT + Lanes([0.0, 0.0, (AZU * BGZ)])) * BHS;
                let BHV = BHS * AZN;
                let BHW = AZU * BHS;
                BHX = BHT;
                BHY = BHV;
                BHZ = BHU;
                BIA = BHW;
            } else {
                BHX = DX;
                BHY = DX;
                BHZ = AZG;
                BIA = AZH;
            }
            let BIB = ddt(80365, AZO);
            let BIC = AZV * BGZ;
            let BIE = BID * AZW;
            let BIF = AZX * BID;
            let BIG = ddt(80371, BIE);
            let BIH = BIF * BGZ;
            let BII = AC[0];
            let BIJ = AC[1];
            let BIK = BGR[0];
            let BIL = BGR[1];
            let BIM = BGX[0];
            let BIN = BGX[1];
            let BIO = BHN[0];
            let BIP = BHN[1];
            let BIQ = BHN[2];
            let BIR = BHO[0];
            let BIS = BHO[1];
            let BIT = BHO[2];
            let BIU = BHZ[0];
            let BIV = BHZ[1];
            let BIW = BHZ[2];
            let BIX = BIC[0];
            let BIY = BIC[1];
            let BIZ = BIC[2];
            let BJA = BIC[3];
            let BJB = BIC[4];
            let BJC = BIH[0];
            let BJD = BIH[1];
            let BJE = BHP;
            let BJF = BHQ;
            let BJG = BIA;
            let BJH = AZV[0];
            let BJI = AZV[1];
            let BJJ = AZV[2];
            let BJK = AZV[3];
            let BJL = AZV[4];
            let BJM = BIF[0];
            let BJN = BIF[1];
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(2),
            multiplicity * (BJO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(2),
            multiplicity * (BJP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(1),
            multiplicity * (BJQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(2),
            multiplicity * (W),
            [0, 2],
            [BII, BIJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(2),
            multiplicity * (BGQ),
            [0, 2],
            [BIK, BIL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(1),
            multiplicity * (BGW),
            [1, 2],
            [BIM, BIN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(2), Some(1), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[850],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            None,
            multiplicity * (BHJ),
            [0, 2, 3],
            [BIO, BIP, BIQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(4),
            None,
            multiplicity * (BHK),
            [0, 2, 4],
            [BIR, BIS, BIT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(3), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[851],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(4), None, 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[852],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            None,
            multiplicity * (BHX),
            [0, 2, 5],
            [BIU, BIV, BIW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[853],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(2),
            multiplicity * (BIB),
            [0, 2, 3, 4, 5],
            [BIX, BIY, BIZ, BJA, BJB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(2),
            multiplicity * (BIG),
            [0, 2],
            [BJC, BJD],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = BJO;
        self.canonical_reactive[1] = BJP;
        self.canonical_reactive[2] = BJQ;
        self.canonical_reactive[3] = W;
        self.canonical_reactive[4] = BGQ;
        self.canonical_reactive[5] = BGW;
        self.canonical_reactive[6] = staged[850];
        self.canonical_reactive[7] = BHL;
        self.canonical_reactive[8] = BJE;
        self.canonical_reactive[9] = BHM;
        self.canonical_reactive[10] = BJF;
        self.canonical_reactive[11] = staged[851];
        self.canonical_reactive[12] = staged[852];
        self.canonical_reactive[13] = BHY;
        self.canonical_reactive[14] = BJG;
        self.canonical_reactive[15] = staged[853];
        self.canonical_reactive[16] = AZO;
        self.canonical_reactive[17] = BJH;
        self.canonical_reactive[18] = BJI;
        self.canonical_reactive[19] = BJJ;
        self.canonical_reactive[20] = BJK;
        self.canonical_reactive[21] = BJL;
        self.canonical_reactive[22] = BIE;
        self.canonical_reactive[23] = BJM;
        self.canonical_reactive[24] = BJN;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[8]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[10]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            None,
            &[5],
            &[cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(2),
            &[0, 2, 3, 4, 5],
            &[cached[17], cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(2),
            &[0, 2],
            &[cached[23], cached[24]],
            &[],
            &[],
            multiplicity,
        );
    }

}
