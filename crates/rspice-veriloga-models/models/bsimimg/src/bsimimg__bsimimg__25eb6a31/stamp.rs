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
        let mut key = Vec::with_capacity(1496);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[267] = values[0];
        self.canonical_staged[268] = values[1];
        self.canonical_staged[269] = values[2];
        self.canonical_staged[107] = values[3];
        self.canonical_staged[270] = values[4];
        self.canonical_staged[0] = values[5];
        self.canonical_staged[1] = values[6];
        self.canonical_staged[2] = values[7];
        self.canonical_staged[3] = values[8];
        self.canonical_staged[253] = values[9];
        self.canonical_staged[283] = values[10];
        self.canonical_staged[103] = values[11];
        self.canonical_staged[338] = values[12];
        self.canonical_staged[7] = values[13];
        self.canonical_staged[122] = values[14];
        self.canonical_staged[184] = values[15];
        self.canonical_staged[284] = values[16];
        self.canonical_staged[285] = values[17];
        self.canonical_staged[4] = values[18];
        self.canonical_staged[286] = values[19];
        self.canonical_staged[5] = values[20];
        self.canonical_staged[287] = values[21];
        self.canonical_staged[6] = values[22];
        self.canonical_staged[296] = values[23];
        self.canonical_staged[298] = values[24];
        self.canonical_staged[300] = values[25];
        self.canonical_staged[64] = values[26];
        self.canonical_staged[8] = values[27];
        self.canonical_staged[307] = values[28];
        self.canonical_staged[233] = values[29];
        self.canonical_staged[125] = values[30];
        self.canonical_staged[242] = values[31];
        self.canonical_staged[62] = values[32];
        self.canonical_staged[127] = values[33];
        self.canonical_staged[309] = values[34];
        self.canonical_staged[312] = values[35];
        self.canonical_staged[315] = values[36];
        self.canonical_staged[313] = values[37];
        self.canonical_staged[314] = values[38];
        self.canonical_staged[10] = values[39];
        self.canonical_staged[9] = values[40];
        self.canonical_staged[11] = values[41];
        self.canonical_staged[12] = values[42];
        self.canonical_staged[13] = values[43];
        self.canonical_staged[14] = values[44];
        self.canonical_staged[326] = values[45];
        self.canonical_staged[322] = values[46];
        self.canonical_staged[323] = values[47];
        self.canonical_staged[17] = values[48];
        self.canonical_staged[19] = values[49];
        self.canonical_staged[22] = values[50];
        self.canonical_staged[68] = values[51];
        self.canonical_staged[63] = values[52];
        self.canonical_staged[67] = values[53];
        self.canonical_staged[325] = values[54];
        self.canonical_staged[157] = values[55];
        self.canonical_staged[83] = values[56];
        self.canonical_staged[82] = values[57];
        self.canonical_staged[86] = values[58];
        self.canonical_staged[87] = values[59];
        self.canonical_staged[101] = values[60];
        self.canonical_staged[104] = values[61];
        self.canonical_staged[112] = values[62];
        self.canonical_staged[109] = values[63];
        self.canonical_staged[108] = values[64];
        self.canonical_staged[111] = values[65];
        self.canonical_staged[110] = values[66];
        self.canonical_staged[113] = values[67];
        self.canonical_staged[114] = values[68];
        self.canonical_staged[115] = values[69];
        self.canonical_staged[116] = values[70];
        self.canonical_staged[117] = values[71];
        self.canonical_staged[118] = values[72];
        self.canonical_staged[119] = values[73];
        self.canonical_staged[120] = values[74];
        self.canonical_staged[121] = values[75];
        self.canonical_staged[128] = values[76];
        self.canonical_staged[131] = values[77];
        self.canonical_staged[327] = values[78];
        self.canonical_staged[141] = values[79];
        self.canonical_staged[150] = values[80];
        self.canonical_staged[151] = values[81];
        self.canonical_staged[152] = values[82];
        self.canonical_staged[153] = values[83];
        self.canonical_staged[154] = values[84];
        self.canonical_staged[328] = values[85];
        self.canonical_staged[155] = values[86];
        self.canonical_staged[156] = values[87];
        self.canonical_staged[329] = values[88];
        self.canonical_staged[160] = values[89];
        self.canonical_staged[161] = values[90];
        self.canonical_staged[162] = values[91];
        self.canonical_staged[165] = values[92];
        self.canonical_staged[173] = values[93];
        self.canonical_staged[176] = values[94];
        self.canonical_staged[334] = values[95];
        self.canonical_staged[182] = values[96];
        self.canonical_staged[189] = values[97];
        self.canonical_staged[190] = values[98];
        self.canonical_staged[192] = values[99];
        self.canonical_staged[339] = values[100];
        self.canonical_staged[205] = values[101];
        self.canonical_staged[211] = values[102];
        self.canonical_staged[340] = values[103];
        self.canonical_staged[217] = values[104];
        self.canonical_staged[341] = values[105];
        self.canonical_staged[342] = values[106];
        self.canonical_staged[343] = values[107];
        self.canonical_staged[344] = values[108];
        self.canonical_staged[246] = values[109];
        self.canonical_staged[249] = values[110];
        self.canonical_staged[346] = values[111];
        self.canonical_staged[347] = values[112];
        self.canonical_staged[348] = values[113];
        self.canonical_staged[349] = values[114];
        self.canonical_staged[350] = values[115];
        self.canonical_staged[351] = values[116];
        self.canonical_staged[352] = values[117];
        self.canonical_staged[354] = values[118];
        self.canonical_staged[355] = values[119];
        self.canonical_staged[356] = values[120];
        self.canonical_staged[357] = values[121];
        self.canonical_staged[358] = values[122];
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
                let A = 1.0f64;
                let B = parameters[18];
                let C = 0e0f64;
                let D = parameters[310];
                let F = parameters[12];
                let G = 1e0f64;
                let I = -1e0f64;
                let K = parameters[13];
                let M = -1e0f64;
                let O = parameters[59];
                let X = parameters[45];
                let Z = parameters[47];
                let AB = parameters[46];
                let AD = parameters[49];
                let AF = 3.9e0f64;
                let AK = parameters[138];
                let AN = parameters[188];
                let AQ = parameters[14];
                let AT = parameters[190];
                let AW = parameters[194];
                let AZ = parameters[198];
                let BD = 1e-38f64;
                let BG = 3.333333333333333e-1f64;
                let BH = 5e-1f64;
                let BK = 1e-8f64;
                let BV = parameters[297];
                let CB = 3.0015e2f64;
                let CC = 2.7315e2f64;
                let CF = 4.97232e-7f64;
                let CG = 3.42537e-7f64;
                let CI = 7.45669e11f64;
                let CJ = 1.16645e12f64;
                let CL = parameters[99];
                let CP = parameters[55];
                let CR = parameters[52];
                let DB = 2e0f64;
                let DY = 2.5e-1f64;
                let DZ = parameters[154];
                let EN = parameters[134];
                let EZ = parameters[290];
                let FD = parameters[292];
                let FH = 0e0f64;
                let FI = 0e0f64;
                let FJ = 0e0f64;
                let FK = 0e0f64;
                let FQ = 0e0f64;
                let FR = 0e0f64;
                let FU = 0e0f64;
                let FV = 0e0f64;
                let FZ = 0e0f64;
                let mut oE = 0.0;
                let mut oAM = 0.0;
                let mut oAP = 0.0;
                let mut oBQ = 0.0;
                let mut oBR = 0.0;
                let mut oBY = 0.0;
                let mut oCU = 0.0;
                let mut oCV = 0.0;
                let mut oCW = 0.0;
                let mut oEB = 0.0;
                let mut oEI = 0.0;
                let mut oEJ = 0.0;
                let mut oEP = 0.0;
                let mut oEU = 0.0;
                let mut oEW = 0.0;
                let mut oEX = 0.0;
                let mut oFB = 0.0;
                let mut oFC = 0.0;
                let mut oFE = 0.0;
                let mut oFF = 0.0;
                let mut oFY = 0.0;
                if A != 0.0 {
                    let E = if (if B == C { 1.0 } else { 0.0 }) != 0.0 || (if D == C { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oE = E;
                } else {
                }
                let H = if F == G { 1.0 } else { 0.0 };
                let J = if H != 0.0 {
                    G
                } else {
                    I
                };
                let L = if K == G { 1.0 } else { 0.0 };
                let N = if L != 0.0 {
                    G
                } else {
                    M
                };
                let P = O * 8.85418e-12f64;
                let Q = if parameters[21] == C { 1.0 } else { 0.0 };
                let R = -parameters[29];
                let S = -parameters[30];
                let T = -parameters[35];
                let U = -parameters[36];
                let V = if parameters[20] == G { 1.0 } else { 0.0 };
                let W = if V != 0.0 && (if parameters[317] != C { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let Y = 3.4531302e-11f64 / X;
                let AA = 3.4531302e-11f64 / Z;
                let AC = 3.4531302e-11f64 / AB;
                let AE = P / AD;
                let AG = O / AF;
                let AH = if (if parameter_given[47] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let AJ = if AH != 0.0 {
                    let AI = ((X * parameters[60]) / AF) - parameters[48];
                    AI
                } else {
                    Z
                };
                let AL = if AK > C { 1.0 } else { 0.0 };
                if AL != 0.0 {
                    let AM = -AK;
                    oAM = AM;
                } else {
                }
                let AO = if AN > C { 1.0 } else { 0.0 };
                if AO != 0.0 {
                    let AP = -AN;
                    oAP = AP;
                } else {
                }
                let AR = if AQ == G { 1.0 } else { 0.0 };
                let AS = -parameters[115];
                let AU = if AT < C { 1.0 } else { 0.0 };
                let AV = if AU != 0.0 {
                    C
                } else {
                    AT
                };
                let AX = if AW < C { 1.0 } else { 0.0 };
                let AY = if AX != 0.0 {
                    C
                } else {
                    AW
                };
                let BA = if AZ < C { 1.0 } else { 0.0 };
                let BB = if BA != 0.0 {
                    C
                } else {
                    AZ
                };
                let BC = AD + (AG * (X + AB));
                let BE = parameters[267] * ((if (G + (AD / AB)) >= BD { (G + (AD / AB)) } else { BD }).ln());
                let BF = if F != G { 1.0 } else { 0.0 };
                let BI = if BF != 0.0 {
                    BG
                } else {
                    BH
                };
                let BJ = AG * X;
                let BL = BK / BJ;
                let BM = (BJ * AD).sqrt();
                let BN = AG * AB;
                let BO = BK / BN;
                let BP = if (if B != C { 1.0 } else { 0.0 }) != 0.0 && (if D > C { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BS;
                let BT;
                let BU;
                if AR != 0.0 {
                    let BQ = if AY <= C { 1.0 } else { 0.0 };
                    oBQ = BQ;
                    let BX = if BQ != 0.0 {
                        C
                    } else {
                        AY
                    };
                    let BY = if BB <= C { 1.0 } else { 0.0 };
                    oBY = BY;
                    let BZ = if BY != 0.0 {
                        C
                    } else {
                        BB
                    };
                    BS = AV;
                    BT = BX;
                    BU = BZ;
                } else {
                    let BR = if AV <= C { 1.0 } else { 0.0 };
                    oBR = BR;
                    let CA = if BR != 0.0 {
                        C
                    } else {
                        AV
                    };
                    BS = CA;
                    BT = AY;
                    BU = BB;
                }
                let BW = if BV <= C { 1.0 } else { 0.0 };
                let CE = if BW != 0.0 {
                    CB
                } else {
                    let CD = BV + CC;
                    CD
                };
                let CH = if H != 0.0 {
                    CF
                } else {
                    CG
                };
                let CK = if H != 0.0 {
                    CI
                } else {
                    CJ
                };
                let CM = CL * CL;
                let CN = (if (parameters[239] / CL) >= BD { (parameters[239] / CL) } else { BD }).ln();
                let CO = parameters[298] + CC;
                let CQ = CP / 5.1728331239999994e-2f64;
                let CS = if CR != C { 1.0 } else { 0.0 };
                let CT = if CS != 0.0 && (if (if parameter_given[58] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if CT != 0.0 {
                    let CU = if K == -1e0f64 { 1.0 } else { 0.0 };
                    oCU = CU;
                    if CU != 0.0 {
                        let CV = BH * CP;
                        oCV = CV;
                    } else {
                        let CW = BH * CP;
                        oCW = CW;
                    }
                } else {
                }
                let CX = (AD * (BJ + (3.75e-1f64 * AD))).sqrt();
                let CY = BJ + AD;
                let CZ = (((AG * AD) * X).sqrt()) - CX;
                let DA = if K == -1e0f64 { 1.0 } else { 0.0 };
                let DC = DB * AC;
                let DD = ((1.60219e-19f64 * CR) * P) / (DC * AC);
                let DE = AC + AE;
                let DF = ((-AC) * AE) / (DE * Y);
                let DG = J * N;
                let DH = Y + ((AE * AC) / DE);
                let DI = G - ((BH * AD) / (AD + BN));
                let DJ = Y / AE;
                let DK = AC / AE;
                let DL = DJ * DJ;
                let DM = DJ / (((DK * DJ) + DK) + DJ);
                let DN = G + DK;
                let DO = G + DJ;
                let DP = DK * DK;
                let DQ = DB * DL;
                let DR = -2e0f64 * DJ;
                let DS = -DJ;
                let DT = -2e0f64 * DJ;
                let DU = -2e0f64 * DJ;
                let DV = -2e0f64 * DJ;
                let DW = -2e0f64 * DJ;
                let DX = 1e-2f64 / Y;
                let EA = (DY * DZ) * DZ;
                if AR != 0.0 {
                } else {
                    let EB = if AQ == C { 1.0 } else { 0.0 };
                    oEB = EB;
                }
                let EC = -2e0f64 * DJ;
                let ED = -2e0f64 * DJ;
                let EE = -2e0f64 * DJ;
                let EF = -2e0f64 * DJ;
                let EG = -2e0f64 * DJ;
                let EH = if parameters[162] != C { 1.0 } else { 0.0 };
                if EH != 0.0 {
                    let EI = DB * Y;
                    oEI = EI;
                } else {
                    let EJ = DB * Y;
                    oEJ = EJ;
                }
                let EK = if parameters[189] != C { 1.0 } else { 0.0 };
                let EL = DB * Y;
                let EM = G + (parameters[109].sqrt());
                let EO = (DY * EN) * EN;
                if AR != 0.0 {
                } else {
                    let EP = if AQ == DB { 1.0 } else { 0.0 };
                    oEP = EP;
                }
                let EQ = X / AB;
                let ER = BH * parameters[265];
                let ES = BH * parameters[266];
                let ET = if parameters[17] != C { 1.0 } else { 0.0 };
                if ET != 0.0 {
                    let EU = -9.82222e11f64 * CL;
                    oEU = EU;
                    let EW = -7.45669e11f64 * CL;
                    oEW = EW;
                } else {
                }
                let EV = if parameters[16] != C { 1.0 } else { 0.0 };
                if EV != 0.0 {
                    let EX = (-CK) * CL;
                    oEX = EX;
                } else {
                }
                let EY = if parameters[15] != C { 1.0 } else { 0.0 };
                let FA = if (if (if parameters[288] > C { 1.0 } else { 0.0 }) != 0.0 || (if parameters[289] > C { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if EZ > C { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if FA != 0.0 {
                    let FB = if parameters[287] <= C { 1.0 } else { 0.0 };
                    oFB = FB;
                    let FC = if parameters[22] == G { 1.0 } else { 0.0 };
                    oFC = FC;
                    if FC != 0.0 {
                        let FE = (DY * FD) * FD;
                        oFE = FE;
                    } else {
                    }
                    let FF = BH * EZ;
                    oFF = FF;
                } else {
                }
                let FG = if AQ == DB { 1.0 } else { 0.0 };
                let FL;
                let FM;
                let FN;
                let FO;
                if FG != 0.0 {
                    FL = FH;
                    FM = FI;
                    FN = C;
                    FO = C;
                } else {
                    FL = C;
                    FM = C;
                    FN = FJ;
                    FO = FK;
                }
                let FP = if parameters[19] == C { 1.0 } else { 0.0 };
                let FS;
                let FT;
                if FP != 0.0 {
                    FS = FQ;
                    FT = C;
                } else {
                    FS = C;
                    FT = FR;
                }
                let FW;
                let FX;
                if ET != 0.0 {
                    FW = FU;
                    FX = FV;
                } else {
                    FW = C;
                    FX = C;
                }
                let GA;
                if BP != 0.0 {
                    let FY = if AQ != DB { 1.0 } else { 0.0 };
                    oFY = FY;
                    GA = C;
                } else {
                    GA = FZ;
                }
            [oE, H, L, P, Q, R, S, T, U, V, W, Y, AA, AC, AE, AG, AH, AL, oAM, AO, oAP, AR, AS, AU, AX, BA, BC, BE, BF, BJ, BL, BM, BN, BO, BP, oBQ, oBY, oBR, BW, CM, CN, CH, CO, CE, CQ, CS, CT, oCU, oCV, oCW, J, CX, CY, CZ, DA, DC, DD, N, DF, DG, DH, DI, DJ, DK, DL, DM, DN, DO, DP, DQ, DR, DS, DT, DU, DV, DW, DX, EA, oEB, BS, EC, ED, EE, EF, EG, EH, oEI, oEJ, EK, EL, BI, EM, EO, BT, BU, oEP, AJ, EQ, ER, ES, ET, oEU, oEW, EV, oEX, EY, FA, oFB, oFC, oFE, oFF, FG, FP, oFY, FL, FM, FN, FO, FS, FT, FW, FX, GA]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 235] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[270];
                let B = parameters[1];
                let C = parameters[2];
                let O = 2e0f64;
                let Q = 0e0f64;
                let S = 1e-9f64;
                let AE = 1e-6f64;
                let BB = 1e0f64;
                let GF = staged[283];
                let GL = staged[285];
                let GW = staged[286];
                let HG = staged[287];
                let HO = 1e6f64;
                let IA = 3e-2f64;
                let JD = staged[7];
                let JE = staged[8];
                let JF = 1e-20f64;
                let JI = 5e-1f64;
                let JL = staged[307];
                let JR = parameters[296];
                let JU = staged[309];
                let KC = 1e-3f64;
                let KS = staged[11];
                let KU = parameters[315];
                let LA = 1e3f64;
                let LC = staged[322];
                let LD = staged[323];
                let LE = 1e0f64;
                let LN = 4e0f64;
                let LU = staged[325];
                let MK = staged[103];
                let MP = staged[327];
                let MQ = staged[141];
                let MZ = staged[334];
                let NJ = staged[22];
                let NT = staged[339];
                let NW = staged[340];
                let NZ = staged[86];
                let OC = staged[341];
                let OF = staged[342];
                let OK = 1e10f64;
                let OP = 0e0f64;
                let mut oT = 0.0;
                let mut oW = 0.0;
                let mut oAA = 0.0;
                let mut oAD = 0.0;
                let mut oBC = 0.0;
                let mut oBN = 0.0;
                let mut oKK = 0.0;
                let mut oKM = 0.0;
                let mut oKO = 0.0;
                let mut oKY = 0.0;
                let mut oLJ = 0.0;
                let mut oLK = 0.0;
                let mut oLV = 0.0;
                let mut oLW = 0.0;
                let mut oMB = 0.0;
                let mut oMC = 0.0;
                let mut oMR = 0.0;
                let mut oMW = 0.0;
                let mut oMY = 0.0;
                let mut oNB = 0.0;
                let mut oNC = 0.0;
                let mut oND = 0.0;
                let mut oNE = 0.0;
                let mut oNF = 0.0;
                let mut oNV = 0.0;
                let mut oNX = 0.0;
                let mut oNY = 0.0;
                let mut oOA = 0.0;
                let mut oOB = 0.0;
                let mut oOD = 0.0;
                let mut oOE = 0.0;
                let mut oOG = 0.0;
                let mut oOI = 0.0;
                let mut oOL = 0.0;
                let mut oOM = 0.0;
                let mut oOO = 0.0;
                let E = if A != 0.0 {
                    let D = B / C;
                    D
                } else {
                    B
                };
                let F = parameters[0] + parameters[23];
                let G = E + parameters[24];
                let H = F.powf(staged[0]);
                let I = G.powf(staged[1]);
                let J = H * I;
                let K = F.powf(staged[2]);
                let L = G.powf(staged[3]);
                let M = K * L;
                let N = ((parameters[31] + (parameters[32] * K)) + (parameters[33] * L)) + (parameters[34] * M);
                let P = F - (O * (((parameters[25] + (parameters[26] * H)) + (parameters[27] * I)) + (parameters[28] * J)));
                let R = if P <= Q { 1.0 } else { 0.0 };
                if R != 0.0 {
                } else {
                    let T = if P <= S { 1.0 } else { 0.0 };
                    oT = T;
                }
                let U = G - (O * N);
                let V = if U <= Q { 1.0 } else { 0.0 };
                if V != 0.0 {
                } else {
                    let W = if U <= S { 1.0 } else { 0.0 };
                    oW = W;
                }
                let X = ((parameters[41] + (parameters[42] * K)) + (parameters[43] * L)) + (parameters[44] * M);
                let Y = F - (O * (((parameters[37] + (parameters[38] * H)) + (parameters[39] * I)) + (parameters[40] * J)));
                let Z = if Y <= Q { 1.0 } else { 0.0 };
                if Z != 0.0 {
                } else {
                    let AA = if Y <= S { 1.0 } else { 0.0 };
                    oAA = AA;
                }
                let AB = G - (O * X);
                let AC = if AB <= Q { 1.0 } else { 0.0 };
                if AC != 0.0 {
                } else {
                    let AD = if AB <= S { 1.0 } else { 0.0 };
                    oAD = AD;
                }
                let AF = AE / P;
                let AG = AE / U;
                let AH = AF * AG;
                let AI = ((parameters[191] + (parameters[319] * AF)) + (parameters[320] * AG)) + (parameters[321] * AH);
                let AJ = ((parameters[199] + (parameters[325] * AF)) + (parameters[326] * AG)) + (parameters[327] * AH);
                let AK = ((parameters[195] + (parameters[322] * AF)) + (parameters[323] * AG)) + (parameters[324] * AH);
                let AL = ((parameters[202] + (parameters[328] * AF)) + (parameters[329] * AG)) + (parameters[330] * AH);
                let AM = ((parameters[203] + (parameters[331] * AF)) + (parameters[332] * AG)) + (parameters[333] * AH);
                let AN = ((parameters[204] + (parameters[334] * AF)) + (parameters[335] * AG)) + (parameters[336] * AH);
                let AO = ((parameters[57] + (parameters[337] * AF)) + (parameters[338] * AG)) + (parameters[339] * AH);
                let AP = ((parameters[58] + (parameters[340] * AF)) + (parameters[341] * AG)) + (parameters[342] * AH);
                let AQ = ((parameters[51] + (parameters[343] * AF)) + (parameters[344] * AG)) + (parameters[345] * AH);
                let AR = ((parameters[50] + (parameters[346] * AF)) + (parameters[347] * AG)) + (parameters[348] * AH);
                let AS = ((parameters[63] + (parameters[349] * AF)) + (parameters[350] * AG)) + (parameters[351] * AH);
                let AT = ((parameters[64] + (parameters[352] * AF)) + (parameters[353] * AG)) + (parameters[354] * AH);
                let AU = ((parameters[65] + (parameters[355] * AF)) + (parameters[356] * AG)) + (parameters[357] * AH);
                let AV = ((parameters[68] + (parameters[358] * AF)) + (parameters[359] * AG)) + (parameters[360] * AH);
                let AW = ((parameters[276] + (parameters[361] * AF)) + (parameters[362] * AG)) + (parameters[363] * AH);
                let AX = ((parameters[291] + (parameters[751] * AF)) + (parameters[752] * AG)) + (parameters[753] * AH);
                let AY = ((parameters[294] + (parameters[757] * AF)) + (parameters[758] * AG)) + (parameters[759] * AH);
                let AZ = ((parameters[293] + (parameters[754] * AF)) + (parameters[755] * AG)) + (parameters[756] * AH);
                let BA = if AW < Q { 1.0 } else { 0.0 };
                let BD;
                if BA != 0.0 {
                    BD = Q;
                } else {
                    let BC = if AW > BB { 1.0 } else { 0.0 };
                    oBC = BC;
                    let BM = if BC != 0.0 {
                        BB
                    } else {
                        AW
                    };
                    BD = BM;
                }
                let BE = ((parameters[277] + (parameters[364] * AF)) + (parameters[365] * AG)) + (parameters[366] * AH);
                let BF = ((parameters[278] + (parameters[367] * AF)) + (parameters[368] * AG)) + (parameters[369] * AH);
                let BG = ((parameters[275] + (parameters[370] * AF)) + (parameters[371] * AG)) + (parameters[372] * AH);
                let BH = ((parameters[272] + (parameters[373] * AF)) + (parameters[374] * AG)) + (parameters[375] * AH);
                let BI = ((parameters[273] + (parameters[376] * AF)) + (parameters[377] * AG)) + (parameters[378] * AH);
                let BJ = ((parameters[274] + (parameters[379] * AF)) + (parameters[380] * AG)) + (parameters[381] * AH);
                let BK = ((parameters[283] + (parameters[382] * AF)) + (parameters[383] * AG)) + (parameters[384] * AH);
                let BL = if BK < Q { 1.0 } else { 0.0 };
                let BO;
                if BL != 0.0 {
                    BO = Q;
                } else {
                    let BN = if BK > BB { 1.0 } else { 0.0 };
                    oBN = BN;
                    let GG = if BN != 0.0 {
                        BB
                    } else {
                        BK
                    };
                    BO = GG;
                }
                let BP = ((parameters[284] + (parameters[385] * AF)) + (parameters[386] * AG)) + (parameters[387] * AH);
                let BQ = ((parameters[285] + (parameters[388] * AF)) + (parameters[389] * AG)) + (parameters[390] * AH);
                let BR = ((parameters[282] + (parameters[391] * AF)) + (parameters[392] * AG)) + (parameters[393] * AH);
                let BS = ((parameters[279] + (parameters[394] * AF)) + (parameters[395] * AG)) + (parameters[396] * AH);
                let BT = ((parameters[280] + (parameters[397] * AF)) + (parameters[398] * AG)) + (parameters[399] * AH);
                let BU = ((parameters[281] + (parameters[400] * AF)) + (parameters[401] * AG)) + (parameters[402] * AH);
                let BV = ((parameters[71] + (parameters[403] * AF)) + (parameters[404] * AG)) + (parameters[405] * AH);
                let BW = ((parameters[72] + (parameters[406] * AF)) + (parameters[407] * AG)) + (parameters[408] * AH);
                let BX = ((parameters[73] + (parameters[409] * AF)) + (parameters[410] * AG)) + (parameters[411] * AH);
                let BY = ((parameters[74] + (parameters[412] * AF)) + (parameters[413] * AG)) + (parameters[414] * AH);
                let BZ = ((parameters[75] + (parameters[415] * AF)) + (parameters[416] * AG)) + (parameters[417] * AH);
                let CA = ((parameters[84] + (parameters[418] * AF)) + (parameters[419] * AG)) + (parameters[420] * AH);
                let CB = ((parameters[76] + (parameters[421] * AF)) + (parameters[422] * AG)) + (parameters[423] * AH);
                let CC = ((parameters[87] + (parameters[430] * AF)) + (parameters[431] * AG)) + (parameters[432] * AH);
                let CD = ((parameters[88] + (parameters[433] * AF)) + (parameters[434] * AG)) + (parameters[435] * AH);
                let CE = ((parameters[61] + (parameters[436] * AF)) + (parameters[437] * AG)) + (parameters[438] * AH);
                let CF = ((parameters[62] + (parameters[439] * AF)) + (parameters[440] * AG)) + (parameters[441] * AH);
                let CG = ((parameters[85] + (parameters[424] * AF)) + (parameters[425] * AG)) + (parameters[426] * AH);
                let CH = ((parameters[86] + (parameters[427] * AF)) + (parameters[428] * AG)) + (parameters[429] * AH);
                let CI = ((parameters[113] + (parameters[460] * AF)) + (parameters[461] * AG)) + (parameters[462] * AH);
                let CJ = ((parameters[89] + (parameters[442] * AF)) + (parameters[443] * AG)) + (parameters[444] * AH);
                let CK = ((parameters[90] + (parameters[445] * AF)) + (parameters[446] * AG)) + (parameters[447] * AH);
                let CL = ((parameters[91] + (parameters[448] * AF)) + (parameters[449] * AG)) + (parameters[450] * AH);
                let CM = ((parameters[92] + (parameters[451] * AF)) + (parameters[452] * AG)) + (parameters[453] * AH);
                let CN = ((parameters[93] + (parameters[454] * AF)) + (parameters[455] * AG)) + (parameters[456] * AH);
                let CO = ((parameters[94] + (parameters[457] * AF)) + (parameters[458] * AG)) + (parameters[459] * AH);
                let CP = ((parameters[116] + (parameters[463] * AF)) + (parameters[464] * AG)) + (parameters[465] * AH);
                let CQ = ((parameters[123] + (parameters[466] * AF)) + (parameters[467] * AG)) + (parameters[468] * AH);
                let CR = ((parameters[124] + (parameters[469] * AF)) + (parameters[470] * AG)) + (parameters[471] * AH);
                let CS = ((parameters[122] + (parameters[472] * AF)) + (parameters[473] * AG)) + (parameters[474] * AH);
                let CT = ((parameters[135] + (parameters[475] * AF)) + (parameters[476] * AG)) + (parameters[477] * AH);
                let CU = ((parameters[139] + (parameters[478] * AF)) + (parameters[479] * AG)) + (parameters[480] * AH);
                let CV = ((parameters[145] + (parameters[481] * AF)) + (parameters[482] * AG)) + (parameters[483] * AH);
                let CW = ((parameters[148] + (parameters[484] * AF)) + (parameters[485] * AG)) + (parameters[486] * AH);
                let CX = ((parameters[155] + (parameters[487] * AF)) + (parameters[488] * AG)) + (parameters[489] * AH);
                let CY = ((parameters[142] + (parameters[490] * AF)) + (parameters[491] * AG)) + (parameters[492] * AH);
                let CZ = ((parameters[163] + (parameters[493] * AF)) + (parameters[494] * AG)) + (parameters[495] * AH);
                let DA = ((parameters[157] + (parameters[496] * AF)) + (parameters[497] * AG)) + (parameters[498] * AH);
                let DB = ((parameters[156] + (parameters[499] * AF)) + (parameters[500] * AG)) + (parameters[501] * AH);
                let DC = ((parameters[158] + (parameters[502] * AF)) + (parameters[503] * AG)) + (parameters[504] * AH);
                let DD = ((parameters[160] + (parameters[505] * AF)) + (parameters[506] * AG)) + (parameters[507] * AH);
                let DE = ((parameters[161] + (parameters[508] * AF)) + (parameters[509] * AG)) + (parameters[510] * AH);
                let DF = ((parameters[136] + (parameters[511] * AF)) + (parameters[512] * AG)) + (parameters[513] * AH);
                let DG = ((parameters[166] + (parameters[514] * AF)) + (parameters[515] * AG)) + (parameters[516] * AH);
                let DH = ((parameters[167] + (parameters[517] * AF)) + (parameters[518] * AG)) + (parameters[519] * AH);
                let DI = ((parameters[173] + (parameters[520] * AF)) + (parameters[521] * AG)) + (parameters[522] * AH);
                let DJ = ((parameters[176] + (parameters[523] * AF)) + (parameters[524] * AG)) + (parameters[525] * AH);
                let DK = ((parameters[182] + (parameters[526] * AF)) + (parameters[527] * AG)) + (parameters[528] * AH);
                let DL = ((parameters[170] + (parameters[529] * AF)) + (parameters[530] * AG)) + (parameters[531] * AH);
                let DM = ((parameters[183] + (parameters[532] * AF)) + (parameters[533] * AG)) + (parameters[534] * AH);
                let DN = ((parameters[186] + (parameters[535] * AF)) + (parameters[536] * AG)) + (parameters[537] * AH);
                let DO = ((parameters[119] + (parameters[538] * AF)) + (parameters[539] * AG)) + (parameters[540] * AH);
                let DP = ((parameters[130] + (parameters[541] * AF)) + (parameters[542] * AG)) + (parameters[543] * AH);
                let DQ = ((parameters[205] + (parameters[544] * AF)) + (parameters[545] * AG)) + (parameters[546] * AH);
                let DR = ((parameters[305] + (parameters[547] * AF)) + (parameters[548] * AG)) + (parameters[549] * AH);
                let DS = ((parameters[306] + (parameters[550] * AF)) + (parameters[551] * AG)) + (parameters[552] * AH);
                let DT = ((parameters[307] + (parameters[553] * AF)) + (parameters[554] * AG)) + (parameters[555] * AH);
                let DU = ((parameters[308] + (parameters[556] * AF)) + (parameters[557] * AG)) + (parameters[558] * AH);
                let DV = ((parameters[210] + (parameters[559] * AF)) + (parameters[560] * AG)) + (parameters[561] * AH);
                let DW = ((parameters[214] + (parameters[562] * AF)) + (parameters[563] * AG)) + (parameters[564] * AH);
                let DX = ((parameters[208] + (parameters[565] * AF)) + (parameters[566] * AG)) + (parameters[567] * AH);
                let DY = ((parameters[206] + (parameters[568] * AF)) + (parameters[569] * AG)) + (parameters[570] * AH);
                let DZ = ((parameters[207] + (parameters[571] * AF)) + (parameters[572] * AG)) + (parameters[573] * AH);
                let EA = ((parameters[209] + (parameters[574] * AF)) + (parameters[575] * AG)) + (parameters[576] * AH);
                let EB = ((parameters[256] + (parameters[577] * AF)) + (parameters[578] * AG)) + (parameters[579] * AH);
                let EC = ((parameters[257] + (parameters[580] * AF)) + (parameters[581] * AG)) + (parameters[582] * AH);
                let ED = ((parameters[258] + (parameters[583] * AF)) + (parameters[584] * AG)) + (parameters[585] * AH);
                let EE = ((parameters[217] + (AF * parameters[706])) + (AG * parameters[707])) + (AH * parameters[708]);
                let EF = ((parameters[218] + (AF * parameters[709])) + (AG * parameters[710])) + (AH * parameters[711]);
                let EG = ((parameters[219] + (AF * parameters[712])) + (AG * parameters[713])) + (AH * parameters[714]);
                let EH = ((parameters[220] + (AF * parameters[715])) + (AG * parameters[716])) + (AH * parameters[717]);
                let EI = ((parameters[221] + (AF * parameters[718])) + (AG * parameters[719])) + (AH * parameters[720]);
                let EJ = ((parameters[222] + (AF * parameters[721])) + (AG * parameters[722])) + (AH * parameters[723]);
                let EK = ((parameters[223] + (AF * parameters[724])) + (AG * parameters[725])) + (AH * parameters[726]);
                let EL = ((parameters[224] + (AF * parameters[727])) + (AG * parameters[728])) + (AH * parameters[729]);
                let EM = ((parameters[225] + (AF * parameters[730])) + (AG * parameters[731])) + (AH * parameters[732]);
                let EN = ((parameters[226] + (parameters[586] * AF)) + (parameters[587] * AG)) + (parameters[588] * AH);
                let EO = ((parameters[227] + (parameters[589] * AF)) + (parameters[590] * AG)) + (parameters[591] * AH);
                let EP = ((parameters[228] + (parameters[592] * AF)) + (parameters[593] * AG)) + (parameters[594] * AH);
                let EQ = ((parameters[230] + (parameters[595] * AF)) + (parameters[596] * AG)) + (parameters[597] * AH);
                let ER = ((parameters[229] + (parameters[598] * AF)) + (parameters[599] * AG)) + (parameters[600] * AH);
                let ES = ((parameters[247] + (parameters[610] * AF)) + (parameters[611] * AG)) + (parameters[612] * AH);
                let ET = ((parameters[250] + (parameters[619] * AF)) + (parameters[620] * AG)) + (parameters[621] * AH);
                let EU = ((parameters[251] + (parameters[622] * AF)) + (parameters[623] * AG)) + (parameters[624] * AH);
                let EV = ((parameters[252] + (parameters[625] * AF)) + (parameters[626] * AG)) + (parameters[627] * AH);
                let EW = ((parameters[253] + (parameters[628] * AF)) + (parameters[629] * AG)) + (parameters[630] * AH);
                let EX = ((parameters[244] + (parameters[601] * AF)) + (parameters[602] * AG)) + (parameters[603] * AH);
                let EY = ((parameters[245] + (parameters[604] * AF)) + (parameters[605] * AG)) + (parameters[606] * AH);
                let EZ = ((parameters[246] + (parameters[607] * AF)) + (parameters[608] * AG)) + (parameters[609] * AH);
                let FA = ((parameters[248] + (parameters[613] * AF)) + (parameters[614] * AG)) + (parameters[615] * AH);
                let FB = ((parameters[254] + (parameters[631] * AF)) + (parameters[632] * AG)) + (parameters[633] * AH);
                let FC = ((parameters[249] + (parameters[616] * AF)) + (parameters[617] * AG)) + (parameters[618] * AH);
                let FD = ((parameters[255] + (parameters[634] * AF)) + (parameters[635] * AG)) + (parameters[636] * AH);
                let FE = ((parameters[231] + (parameters[637] * AF)) + (parameters[638] * AG)) + (parameters[639] * AH);
                let FF = ((parameters[232] + (parameters[643] * AF)) + (parameters[644] * AG)) + (parameters[645] * AH);
                let FG = ((parameters[233] + (parameters[649] * AF)) + (parameters[650] * AG)) + (parameters[651] * AH);
                let FH = ((parameters[242] + (parameters[655] * AF)) + (parameters[656] * AG)) + (parameters[657] * AH);
                let FI = ((parameters[236] + (parameters[640] * AF)) + (parameters[641] * AG)) + (parameters[642] * AH);
                let FJ = ((parameters[237] + (parameters[646] * AF)) + (parameters[647] * AG)) + (parameters[648] * AH);
                let FK = ((parameters[238] + (parameters[652] * AF)) + (parameters[653] * AG)) + (parameters[654] * AH);
                let FL = ((parameters[243] + (parameters[658] * AF)) + (parameters[659] * AG)) + (parameters[660] * AH);
                let FM = ((parameters[240] + (parameters[661] * AF)) + (parameters[662] * AG)) + (parameters[663] * AH);
                let FN = ((parameters[241] + (parameters[664] * AF)) + (parameters[665] * AG)) + (parameters[666] * AH);
                let FO = ((parameters[259] + (parameters[667] * AF)) + (parameters[668] * AG)) + (parameters[669] * AH);
                let FP = ((parameters[260] + (parameters[670] * AF)) + (parameters[671] * AG)) + (parameters[672] * AH);
                let FQ = ((parameters[261] + (parameters[673] * AF)) + (parameters[674] * AG)) + (parameters[675] * AH);
                let FR = ((parameters[262] + (parameters[676] * AF)) + (parameters[677] * AG)) + (parameters[678] * AH);
                let FS = ((parameters[100] + (parameters[679] * AF)) + (parameters[680] * AG)) + (parameters[681] * AH);
                let FT = ((parameters[129] + (parameters[682] * AF)) + (parameters[683] * AG)) + (parameters[684] * AH);
                let FU = ((parameters[103] + (parameters[685] * AF)) + (parameters[686] * AG)) + (parameters[687] * AH);
                let FV = ((parameters[106] + (parameters[688] * AF)) + (parameters[689] * AG)) + (parameters[690] * AH);
                let FW = ((parameters[110] + (parameters[691] * AF)) + (parameters[692] * AG)) + (parameters[693] * AH);
                let FX = ((parameters[111] + (parameters[694] * AF)) + (parameters[695] * AG)) + (parameters[696] * AH);
                let FY = ((parameters[112] + (parameters[697] * AF)) + (parameters[698] * AG)) + (parameters[699] * AH);
                let FZ = ((parameters[137] + (parameters[700] * AF)) + (parameters[701] * AG)) + (parameters[702] * AH);
                let GA = ((parameters[187] + (parameters[703] * AF)) + (parameters[704] * AG)) + (parameters[705] * AH);
                let GB = ((parameters[95] + (parameters[739] * AF)) + (parameters[740] * AG)) + (parameters[741] * AH);
                let GC = ((parameters[96] + (parameters[742] * AF)) + (parameters[743] * AG)) + (parameters[744] * AH);
                let GD = ((parameters[97] + (parameters[745] * AF)) + (parameters[746] * AG)) + (parameters[747] * AH);
                let GE = ((parameters[98] + (parameters[748] * AF)) + (parameters[749] * AG)) + (parameters[750] * AH);
                let GJ;
                let GK;
                if GF != 0.0 {
                    let GH = ((parameters[317] + (parameters[733] * AF)) + (parameters[734] * AG)) + (parameters[735] * AH);
                    let GI = ((parameters[318] + (parameters[736] * AF)) + (parameters[737] * AG)) + (parameters[738] * AH);
                    GJ = GH;
                    GK = GI;
                } else {
                    GJ = Q;
                    GK = Q;
                }
                let GO = if GL != 0.0 {
                    let GM = CT * (BB - (FZ * (P.powf(staged[4]))));
                    GM
                } else {
                    let GN = CT * (BB - FZ);
                    GN
                };
                let GP = -P;
                let GQ = CU + (parameters[140] * (rspice_limited_exp((GP / parameters[141]))));
                let GR = CV + (parameters[146] * (rspice_limited_exp((GP / parameters[147]))));
                let GS = parameters[151] + (parameters[152] * (rspice_limited_exp((GP / parameters[153]))));
                let GT = CW + (parameters[149] * (rspice_limited_exp((GP / parameters[150]))));
                let GU = CY + (parameters[143] * (rspice_limited_exp((GP / parameters[144]))));
                let GV = CZ + (parameters[164] * (rspice_limited_exp((GP / parameters[165]))));
                let GZ = if GW != 0.0 {
                    let GX = DG * (BB - (GA * (P.powf(staged[5]))));
                    GX
                } else {
                    let GY = DG * (BB - GA);
                    GY
                };
                let HA = DH + (parameters[168] * (rspice_limited_exp((GP / parameters[169]))));
                let HB = DI + (parameters[174] * (rspice_limited_exp((GP / parameters[175]))));
                let HC = parameters[179] + (parameters[180] * (rspice_limited_exp((GP / parameters[181]))));
                let HD = DJ + (parameters[177] * (rspice_limited_exp((GP / parameters[178]))));
                let HE = DL + (parameters[171] * (rspice_limited_exp((GP / parameters[172]))));
                let HF = DM + (parameters[184] * (rspice_limited_exp((GP / parameters[185]))));
                let HK;
                let HL;
                let HM;
                if HG != 0.0 {
                    let HH = AK + (parameters[196] * (rspice_limited_exp((GP / parameters[197]))));
                    let HI = AJ + (parameters[200] * (rspice_limited_exp((GP / parameters[201]))));
                    HK = AI;
                    HL = HH;
                    HM = HI;
                } else {
                    let HJ = AI + (parameters[192] * (rspice_limited_exp((GP / parameters[193]))));
                    HK = HJ;
                    HL = AK;
                    HM = AJ;
                }
                let HN = DV + (parameters[211] * (rspice_limited_exp((GP / parameters[212]))));
                let HP = CI + (parameters[114] * ((P * HO).powf(staged[6])));
                let HQ = CP + (parameters[117] * (rspice_limited_exp((GP / parameters[118]))));
                let HR = CQ + (parameters[125] * (rspice_limited_exp((GP / parameters[126]))));
                let HS = CR + (parameters[127] * (rspice_limited_exp((GP / parameters[128]))));
                let HT = FS + (parameters[101] * (rspice_limited_exp((GP / parameters[102]))));
                let HU = FT + (parameters[132] * (rspice_limited_exp((GP / parameters[133]))));
                let HV = FU + (parameters[104] * (rspice_limited_exp((GP / parameters[105]))));
                let HW = FV + (parameters[107] * (rspice_limited_exp((GP / parameters[108]))));
                let HX = parameters[77] + (parameters[79] * (rspice_limited_exp((GP / parameters[80]))));
                let HY = parameters[78] + (parameters[81] * (rspice_limited_exp((GP / parameters[82]))));
                let HZ = if GO < Q { 1.0 } else { 0.0 };
                let IB = if HZ != 0.0 {
                    IA
                } else {
                    GO
                };
                let IC = if GQ < Q { 1.0 } else { 0.0 };
                let ID = if IC != 0.0 {
                    Q
                } else {
                    GQ
                };
                let IE = if GU < Q { 1.0 } else { 0.0 };
                let IF = if IE != 0.0 {
                    Q
                } else {
                    GU
                };
                let IG = if GT < Q { 1.0 } else { 0.0 };
                let IH = if IG != 0.0 {
                    Q
                } else {
                    GT
                };
                let II = if CX < Q { 1.0 } else { 0.0 };
                let IJ = if II != 0.0 {
                    Q
                } else {
                    CX
                };
                let IK = if HU < Q { 1.0 } else { 0.0 };
                let IL = if IK != 0.0 {
                    Q
                } else {
                    HU
                };
                let IM = if BW <= Q { 1.0 } else { 0.0 };
                let IN = if CB <= Q { 1.0 } else { 0.0 };
                let IO = if HK < Q { 1.0 } else { 0.0 };
                let IP = if IO != 0.0 {
                    Q
                } else {
                    HK
                };
                let IQ = if HL < Q { 1.0 } else { 0.0 };
                let IR = if IQ != 0.0 {
                    Q
                } else {
                    HL
                };
                let IS = if HM < Q { 1.0 } else { 0.0 };
                let IT = if IS != 0.0 {
                    Q
                } else {
                    HM
                };
                let IU = if AL < Q { 1.0 } else { 0.0 };
                let IV = if IU != 0.0 {
                    Q
                } else {
                    AL
                };
                let IW = if DY < Q { 1.0 } else { 0.0 };
                let IX = if DZ < Q { 1.0 } else { 0.0 };
                let IY = if DX <= Q { 1.0 } else { 0.0 };
                let IZ = if HP < O { 1.0 } else { 0.0 };
                let JA = if IZ != 0.0 {
                    O
                } else {
                    HP
                };
                let JB = ((BB + (CH / P)).sqrt()) - BB;
                let JC = BB / JA;
                let JG = if ((JD * parameters[3]) + (JE * (if (parameters[5] - B) >= Q { (parameters[5] - B) } else { Q }))) >= JF { ((JD * parameters[3]) + (JE * (if (parameters[5] - B) >= Q { (parameters[5] - B) } else { Q }))) } else { JF };
                let JH = if ((JD * parameters[4]) + (JE * (if (parameters[6] - B) >= Q { (parameters[6] - B) } else { Q }))) >= JF { ((JD * parameters[4]) + (JE * (if (parameters[6] - B) >= Q { (parameters[6] - B) } else { Q }))) } else { JF };
                let JJ = JI * DF;
                let JK = JI * DN;
                let JO;
                let JP;
                if JL != 0.0 {
                    let JM = 3.333333333333333e-1f64 * DF;
                    let JN = 3.333333333333333e-1f64 * DN;
                    JO = JM;
                    JP = JN;
                } else {
                    JO = JJ;
                    JP = JK;
                }
                let JQ = BB / (((U * HO).powf(AN)) * C);
                let JS = if JR >= (P / O) { 1.0 } else { 0.0 };
                let JT = if JS != 0.0 {
                    Q
                } else {
                    JR
                };
                let JY;
                let JZ;
                if JU != 0.0 {
                    let JV = parameters[312] + (U * C);
                    let JW = JV / parameters[310];
                    let JX = parameters[311] * JV;
                    JY = JW;
                    JZ = JX;
                } else {
                    JY = BB;
                    JZ = Q;
                }
                let KA = parameters[215] * parameters[7];
                let KB = parameters[216] * parameters[8];
                let KD = if KA <= KC { 1.0 } else { 0.0 };
                let KE = if KD != 0.0 {
                    KC
                } else {
                    KA
                };
                let KF = if KB <= KC { 1.0 } else { 0.0 };
                let KG = if KF != 0.0 {
                    KC
                } else {
                    KB
                };
                let KH;
                let KI;
                let KJ;
                if HG != 0.0 {
                    let KK = if IR <= Q { 1.0 } else { 0.0 };
                    oKK = KK;
                    let KL = if KK != 0.0 {
                        Q
                    } else {
                        IR
                    };
                    let KM = if IT <= Q { 1.0 } else { 0.0 };
                    oKM = KM;
                    let KN = if KM != 0.0 {
                        Q
                    } else {
                        IT
                    };
                    KH = IP;
                    KI = KL;
                    KJ = KN;
                } else {
                    let KO = if IP <= Q { 1.0 } else { 0.0 };
                    oKO = KO;
                    let KP = if KO != 0.0 {
                        Q
                    } else {
                        IP
                    };
                    KH = KP;
                    KI = IR;
                    KJ = IT;
                }
                let KQ = parameters[99] * FN;
                let KR = (rspice_limited_exp((FM * staged[9]))) / staged[10];
                let KT = (U * KS) * ((rspice_limited_exp((FM * ((if (parameters[239] / KQ) >= 1e-38f64 { (parameters[239] / KQ) } else { 1e-38f64 }).ln())))) / (KQ * KQ));
                let KV = (parameters[316] * (parameters[313] + ((U / 3e0f64) / KU))) / ((KU * C) * (F - parameters[314]));
                let KW = if KV > KC { 1.0 } else { 0.0 };
                let KZ;
                if KW != 0.0 {
                    let KX = BB / KV;
                    KZ = KX;
                } else {
                    let KY = if parameters[19] != Q { 1.0 } else { 0.0 };
                    oKY = KY;
                    KZ = LA;
                }
                let LB = AQ * AR;
                if LC != 0.0 {
                    if LD != 0.0 {
                        let LJ = AP - staged[17];
                        oLJ = LJ;
                    } else {
                        let LK = AP + staged[19];
                        oLK = LK;
                    }
                } else {
                }
                let LF = DB - LE;
                let LG = DD - LE;
                let LH = DE - LE;
                let LI = DO * (BB + (AF * parameters[120]));
                let LL = DP * (BB + (AF * parameters[131]));
                let LM = -CL;
                let LO = (LN * LM) * AE;
                let LP = parameters[301] + (parameters[302] / P);
                let LQ = DR - LE;
                let LR = BW * P;
                let LS = CB * P;
                let LT = DX * P;
                let LX;
                let LY;
                let LZ;
                let MA;
                if LU != 0.0 {
                    let LV = BG * P;
                    oLV = LV;
                    let MB = JI * BI;
                    oMB = MB;
                    LX = BJ;
                    LY = BE;
                    LZ = BF;
                    MA = BD;
                } else {
                    let LW = BR * P;
                    oLW = LW;
                    let MC = JI * BT;
                    oMC = MC;
                    LX = BU;
                    LY = BP;
                    LZ = BQ;
                    MA = BO;
                }
                let MD = -LZ;
                let ME = (LN * MD) * 1e-2f64;
                let MF = staged[87] * MA;
                let MG = -BV;
                let MH = HY - LE;
                let MI = (-CC) / (P + CD);
                let MJ = staged[101] + AS;
                let ML = ((1.60219e-19f64 * AR) * parameters[49]) / MK;
                let MM = ML * staged[104];
                let MN = parameters[303] + (parameters[304] / P);
                let MO = DK - LE;
                if HG != 0.0 {
                } else {
                    if MP != 0.0 {
                    } else {
                        let MR = (KE + KG) + MQ;
                        oMR = MR;
                    }
                }
                let MS = JC - LE;
                let MT = IF - LE;
                let MU = if EA > Q { 1.0 } else { 0.0 };
                let MV = if HN > Q { 1.0 } else { 0.0 };
                if MV != 0.0 {
                    let MW = if parameters[213] < Q { 1.0 } else { 0.0 };
                    oMW = MW;
                    if MW != 0.0 {
                        let MY = BB / HN;
                        oMY = MY;
                    } else {
                    }
                } else {
                }
                let MX = if DW > Q { 1.0 } else { 0.0 };
                if HG != 0.0 {
                } else {
                    let NC;
                    let ND;
                    if MZ != 0.0 {
                        let NB = (KE + KG) + MQ;
                        oNB = NB;
                        NC = Q;
                        ND = Q;
                    } else {
                        NC = KG;
                        ND = KE;
                    }
                    oNC = NC;
                    oND = ND;
                }
                let NA = if GB > Q { 1.0 } else { 0.0 };
                if NA != 0.0 {
                    let NE = GC * ML;
                    oNE = NE;
                    let NF = (staged[182] * 3.9e0f64) / parameters[60];
                    oNF = NF;
                } else {
                }
                let NG = AB * Y;
                let NH = (AB * FO) * MK;
                let NI = (AB * FP) * MK;
                let NK = NJ * AB;
                let NL = NK * parameters[263];
                let NM = NK * parameters[264];
                let NN = AB * FQ;
                let NO = AB * FR;
                let NP = NJ * JG;
                let NQ = NJ * JH;
                let NR = (EB + (EC * P)) / P;
                let NS = if NR <= Q { 1.0 } else { 0.0 };
                if NT != 0.0 {
                    let NU = U * P;
                    let NV = (NU * 3.75956e-7f64) * KR;
                    oNV = NV;
                    let NX = (NU * 4.97232e-7f64) * KR;
                    oNX = NX;
                } else {
                }
                if NW != 0.0 {
                    let NY = ((U * P) * KS) * KR;
                    oNY = NY;
                    let OA = FH * NZ;
                    oOA = OA;
                    let OB = staged[217] * FN;
                    oOB = OB;
                    let OD = FL * NZ;
                    oOD = OD;
                } else {
                }
                if OC != 0.0 {
                    let OE = if EX <= Q { 1.0 } else { 0.0 };
                    oOE = OE;
                    let OG = if ET <= Q { 1.0 } else { 0.0 };
                    oOG = OG;
                } else {
                }
                if OF != 0.0 {
                    let OH = P - (O * JT);
                    let OI = OH * OH;
                    oOI = OI;
                    let OL = ((OK * OI) * U) * C;
                    oOL = OL;
                    let OM = ((U * C) * OH) * OK;
                    oOM = OM;
                } else {
                }
                let OJ = NJ * C;
                let ON = if staged[253] != 0.0 && (if GJ != Q { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if ON != 0.0 {
                    let OO = C * GJ;
                    oOO = OO;
                } else {
                }
                let OQ = if ON != 0.0 {
                    Q
                } else {
                    OP
                };
            [P, R, oT, U, V, oW, Y, Z, oAA, AC, oAD, AM, AO, AP, AQ, AR, AS, AT, AU, AV, AX, AY, AZ, BA, oBC, BH, BL, oBN, BS, BX, BY, BZ, CA, CE, CF, CG, CJ, CK, CL, CM, CN, CO, CS, DA, DB, DC, DD, DE, DK, DQ, DR, DS, DT, DU, DW, DY, DZ, EA, ED, EE, EF, EG, EH, EI, EJ, EK, EL, EM, EN, EO, EP, EQ, ER, ES, ET, EU, EV, EW, EX, EY, EZ, FA, FB, FC, FD, FE, FF, FG, FI, FJ, FK, FW, FX, FY, GB, GD, GE, GR, GS, GV, HA, HB, HC, HD, HE, HF, HN, HQ, HR, HS, HT, HV, HW, HX, HY, HZ, IC, IE, IG, II, IK, IM, IN, IO, IQ, IS, IU, IW, IX, IY, IZ, JB, JA, JC, JQ, JS, KD, KF, oKK, oKM, oKO, KT, KW, oKY, LB, oLJ, oLK, IB, ID, IH, IJ, LI, LL, IL, LM, LO, LP, LR, LS, LT, oLV, oMB, oLW, oMC, LX, LY, MD, ME, MF, MG, MI, MJ, ML, MM, MN, JO, JP, IF, GZ, IV, KH, KE, KG, oMR, MU, MV, oMW, oMY, MX, KI, KJ, oNB, NA, oNE, oNF, NG, NH, NI, NL, NM, NN, NO, NP, NQ, NR, NS, oNV, oNX, oNY, oOA, oOB, oOD, oOE, oOG, oOI, oOL, oOM, OJ, ON, oOO, GK, oNC, oND, KZ, JY, JZ, OQ, LF, LG, LH, LQ, MH, MO, MS, MT]
        };
        self.canonical_staged[144] = produced[0];
        self.canonical_staged[271] = produced[1];
        self.canonical_staged[272] = produced[2];
        self.canonical_staged[148] = produced[3];
        self.canonical_staged[273] = produced[4];
        self.canonical_staged[274] = produced[5];
        self.canonical_staged[166] = produced[6];
        self.canonical_staged[275] = produced[7];
        self.canonical_staged[276] = produced[8];
        self.canonical_staged[277] = produced[9];
        self.canonical_staged[278] = produced[10];
        self.canonical_staged[171] = produced[11];
        self.canonical_staged[21] = produced[12];
        self.canonical_staged[324] = produced[13];
        self.canonical_staged[23] = produced[14];
        self.canonical_staged[16] = produced[15];
        self.canonical_staged[248] = produced[16];
        self.canonical_staged[100] = produced[17];
        self.canonical_staged[98] = produced[18];
        self.canonical_staged[99] = produced[19];
        self.canonical_staged[245] = produced[20];
        self.canonical_staged[243] = produced[21];
        self.canonical_staged[244] = produced[22];
        self.canonical_staged[279] = produced[23];
        self.canonical_staged[280] = produced[24];
        self.canonical_staged[76] = produced[25];
        self.canonical_staged[281] = produced[26];
        self.canonical_staged[282] = produced[27];
        self.canonical_staged[79] = produced[28];
        self.canonical_staged[89] = produced[29];
        self.canonical_staged[39] = produced[30];
        self.canonical_staged[94] = produced[31];
        self.canonical_staged[93] = produced[32];
        self.canonical_staged[66] = produced[33];
        self.canonical_staged[65] = produced[34];
        self.canonical_staged[90] = produced[35];
        self.canonical_staged[44] = produced[36];
        self.canonical_staged[43] = produced[37];
        self.canonical_staged[48] = produced[38];
        self.canonical_staged[45] = produced[39];
        self.canonical_staged[50] = produced[40];
        self.canonical_staged[49] = produced[41];
        self.canonical_staged[51] = produced[42];
        self.canonical_staged[26] = produced[43];
        self.canonical_staged[24] = produced[44];
        self.canonical_staged[28] = produced[45];
        self.canonical_staged[30] = produced[46];
        self.canonical_staged[32] = produced[47];
        self.canonical_staged[132] = produced[48];
        self.canonical_staged[34] = produced[49];
        self.canonical_staged[54] = produced[50];
        self.canonical_staged[56] = produced[51];
        self.canonical_staged[58] = produced[52];
        self.canonical_staged[60] = produced[53];
        self.canonical_staged[170] = produced[54];
        self.canonical_staged[72] = produced[55];
        self.canonical_staged[73] = produced[56];
        self.canonical_staged[167] = produced[57];
        self.canonical_staged[55] = produced[58];
        self.canonical_staged[203] = produced[59];
        self.canonical_staged[202] = produced[60];
        self.canonical_staged[204] = produced[61];
        self.canonical_staged[200] = produced[62];
        self.canonical_staged[201] = produced[63];
        self.canonical_staged[209] = produced[64];
        self.canonical_staged[208] = produced[65];
        self.canonical_staged[210] = produced[66];
        self.canonical_staged[207] = produced[67];
        self.canonical_staged[215] = produced[68];
        self.canonical_staged[214] = produced[69];
        self.canonical_staged[216] = produced[70];
        self.canonical_staged[213] = produced[71];
        self.canonical_staged[219] = produced[72];
        self.canonical_staged[234] = produced[73];
        self.canonical_staged[241] = produced[74];
        self.canonical_staged[59] = produced[75];
        self.canonical_staged[237] = produced[76];
        self.canonical_staged[240] = produced[77];
        self.canonical_staged[235] = produced[78];
        self.canonical_staged[57] = produced[79];
        self.canonical_staged[230] = produced[80];
        self.canonical_staged[231] = produced[81];
        self.canonical_staged[238] = produced[82];
        self.canonical_staged[232] = produced[83];
        self.canonical_staged[239] = produced[84];
        self.canonical_staged[222] = produced[85];
        self.canonical_staged[221] = produced[86];
        self.canonical_staged[223] = produced[87];
        self.canonical_staged[227] = produced[88];
        self.canonical_staged[226] = produced[89];
        self.canonical_staged[228] = produced[90];
        self.canonical_staged[147] = produced[91];
        self.canonical_staged[146] = produced[92];
        self.canonical_staged[145] = produced[93];
        self.canonical_staged[183] = produced[94];
        self.canonical_staged[180] = produced[95];
        self.canonical_staged[181] = produced[96];
        self.canonical_staged[27] = produced[97];
        self.canonical_staged[158] = produced[98];
        self.canonical_staged[129] = produced[99];
        self.canonical_staged[134] = produced[100];
        self.canonical_staged[133] = produced[101];
        self.canonical_staged[159] = produced[102];
        self.canonical_staged[137] = produced[103];
        self.canonical_staged[136] = produced[104];
        self.canonical_staged[135] = produced[105];
        self.canonical_staged[169] = produced[106];
        self.canonical_staged[52] = produced[107];
        self.canonical_staged[163] = produced[108];
        self.canonical_staged[164] = produced[109];
        self.canonical_staged[36] = produced[110];
        self.canonical_staged[37] = produced[111];
        self.canonical_staged[38] = produced[112];
        self.canonical_staged[95] = produced[113];
        self.canonical_staged[96] = produced[114];
        self.canonical_staged[288] = produced[115];
        self.canonical_staged[289] = produced[116];
        self.canonical_staged[290] = produced[117];
        self.canonical_staged[291] = produced[118];
        self.canonical_staged[292] = produced[119];
        self.canonical_staged[293] = produced[120];
        self.canonical_staged[294] = produced[121];
        self.canonical_staged[295] = produced[122];
        self.canonical_staged[297] = produced[123];
        self.canonical_staged[299] = produced[124];
        self.canonical_staged[301] = produced[125];
        self.canonical_staged[302] = produced[126];
        self.canonical_staged[303] = produced[127];
        self.canonical_staged[304] = produced[128];
        self.canonical_staged[305] = produced[129];
        self.canonical_staged[306] = produced[130];
        self.canonical_staged[91] = produced[131];
        self.canonical_staged[42] = produced[132];
        self.canonical_staged[149] = produced[133];
        self.canonical_staged[142] = produced[134];
        self.canonical_staged[308] = produced[135];
        self.canonical_staged[310] = produced[136];
        self.canonical_staged[311] = produced[137];
        self.canonical_staged[316] = produced[138];
        self.canonical_staged[317] = produced[139];
        self.canonical_staged[318] = produced[140];
        self.canonical_staged[61] = produced[141];
        self.canonical_staged[319] = produced[142];
        self.canonical_staged[320] = produced[143];
        self.canonical_staged[15] = produced[144];
        self.canonical_staged[18] = produced[145];
        self.canonical_staged[20] = produced[146];
        self.canonical_staged[25] = produced[147];
        self.canonical_staged[29] = produced[148];
        self.canonical_staged[31] = produced[149];
        self.canonical_staged[33] = produced[150];
        self.canonical_staged[35] = produced[151];
        self.canonical_staged[40] = produced[152];
        self.canonical_staged[41] = produced[153];
        self.canonical_staged[46] = produced[154];
        self.canonical_staged[47] = produced[155];
        self.canonical_staged[53] = produced[156];
        self.canonical_staged[69] = produced[157];
        self.canonical_staged[70] = produced[158];
        self.canonical_staged[71] = produced[159];
        self.canonical_staged[74] = produced[160];
        self.canonical_staged[75] = produced[161];
        self.canonical_staged[77] = produced[162];
        self.canonical_staged[78] = produced[163];
        self.canonical_staged[80] = produced[164];
        self.canonical_staged[81] = produced[165];
        self.canonical_staged[84] = produced[166];
        self.canonical_staged[85] = produced[167];
        self.canonical_staged[88] = produced[168];
        self.canonical_staged[92] = produced[169];
        self.canonical_staged[97] = produced[170];
        self.canonical_staged[102] = produced[171];
        self.canonical_staged[124] = produced[172];
        self.canonical_staged[106] = produced[173];
        self.canonical_staged[105] = produced[174];
        self.canonical_staged[123] = produced[175];
        self.canonical_staged[126] = produced[176];
        self.canonical_staged[130] = produced[177];
        self.canonical_staged[138] = produced[178];
        self.canonical_staged[139] = produced[179];
        self.canonical_staged[140] = produced[180];
        self.canonical_staged[174] = produced[181];
        self.canonical_staged[177] = produced[182];
        self.canonical_staged[143] = produced[183];
        self.canonical_staged[330] = produced[184];
        self.canonical_staged[331] = produced[185];
        self.canonical_staged[332] = produced[186];
        self.canonical_staged[168] = produced[187];
        self.canonical_staged[333] = produced[188];
        self.canonical_staged[172] = produced[189];
        self.canonical_staged[175] = produced[190];
        self.canonical_staged[178] = produced[191];
        self.canonical_staged[335] = produced[192];
        self.canonical_staged[179] = produced[193];
        self.canonical_staged[185] = produced[194];
        self.canonical_staged[186] = produced[195];
        self.canonical_staged[187] = produced[196];
        self.canonical_staged[188] = produced[197];
        self.canonical_staged[191] = produced[198];
        self.canonical_staged[193] = produced[199];
        self.canonical_staged[194] = produced[200];
        self.canonical_staged[195] = produced[201];
        self.canonical_staged[196] = produced[202];
        self.canonical_staged[197] = produced[203];
        self.canonical_staged[199] = produced[204];
        self.canonical_staged[198] = produced[205];
        self.canonical_staged[206] = produced[206];
        self.canonical_staged[212] = produced[207];
        self.canonical_staged[218] = produced[208];
        self.canonical_staged[220] = produced[209];
        self.canonical_staged[224] = produced[210];
        self.canonical_staged[225] = produced[211];
        self.canonical_staged[229] = produced[212];
        self.canonical_staged[236] = produced[213];
        self.canonical_staged[247] = produced[214];
        self.canonical_staged[250] = produced[215];
        self.canonical_staged[251] = produced[216];
        self.canonical_staged[252] = produced[217];
        self.canonical_staged[345] = produced[218];
        self.canonical_staged[255] = produced[219];
        self.canonical_staged[254] = produced[220];
        self.canonical_staged[336] = produced[221];
        self.canonical_staged[337] = produced[222];
        self.canonical_staged[256] = produced[223];
        self.canonical_staged[257] = produced[224];
        self.canonical_staged[258] = produced[225];
        self.canonical_staged[353] = produced[226];
        self.canonical_staged[259] = produced[227];
        self.canonical_staged[260] = produced[228];
        self.canonical_staged[261] = produced[229];
        self.canonical_staged[262] = produced[230];
        self.canonical_staged[263] = produced[231];
        self.canonical_staged[264] = produced[232];
        self.canonical_staged[265] = produced[233];
        self.canonical_staged[266] = produced[234];
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
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = staged[309];
                let mut oB = 0.0;
                if A != 0.0 {
                } else {
                    let B = temperature + parameters[9];
                    oB = B;
                }
            [oB]
        };
        self.canonical_staged[321] = produced[0];
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
        let temperature = ctx.temperature();
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 22330 => 0usize, 22333 => 1usize, 22336 => 2usize, 22339 => 3usize, 22341 => 4usize, 22344 => 5usize, 22348 => 6usize, 22518 => 7usize, _ => usize::MAX };
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
            let A = staged[287];
            let B = staged[309];
            let C = node_potentials[4];
            let E = 1e0f64;
            let F = staged[321];
            let G = 0e0f64;
            let J = staged[12];
            let O = 2e0f64;
            let P = 1e0f64;
            let Q = 5e-1f64;
            let T = staged[13];
            let X = 8.61708e-5f64;
            let AA = parameters[299];
            let AF = -1e0f64;
            let AH = 3.0015e2f64;
            let AL = parameters[54];
            let AN = 2e0f64;
            let AY = 1e-38f64;
            let BS = staged[322];
            let BT = staged[323];
            let BU = staged[324];
            let CA = staged[21];
            let CB = staged[22];
            let CR = staged[24];
            let CS = staged[25];
            let CU = staged[26];
            let CW = 9e-1f64;
            let DA = 1e0f64;
            let DE = parameters[159];
            let DG = 1e-6f64;
            let DK = staged[27];
            let DN = staged[28];
            let DS = staged[29];
            let DV = staged[30];
            let DW = staged[31];
            let DZ = staged[32];
            let EA = staged[33];
            let ED = staged[34];
            let EK = staged[35];
            let ER = staged[36];
            let EU = 1e3f64;
            let FE = staged[37];
            let FL = staged[38];
            let FR = parameters[309];
            let FU = 1e-4f64;
            let FY = staged[39];
            let GB = staged[40];
            let GG = staged[41];
            let GJ = parameters[121];
            let GK = staged[42];
            let GR = staged[43];
            let GU = staged[45];
            let GW = staged[46];
            let HC = staged[49];
            let HF = staged[51];
            let HK = staged[52];
            let HO = staged[53];
            let HR = staged[54];
            let HS = staged[55];
            let HV = staged[56];
            let IA = staged[57];
            let ID = staged[58];
            let II = staged[59];
            let IM = staged[60];
            let IQ = staged[61];
            let IT = node_potentials[8];
            let IU = node_potentials[6];
            let IV = 1e0f64;
            let IW = 1e0f64;
            let IZ = node_potentials[5];
            let JB = 1e0f64;
            let JJ = node_potentials[3];
            let JK = 1e0f64;
            let JV = 0e0f64;
            let KC = -1e0f64;
            let KP = node_potentials[7];
            let KR = 1e0f64;
            let LC = 2e-2f64;
            let LN = staged[62];
            let LP = staged[63];
            let LR = staged[64];
            let LS = staged[65];
            let LU = 3.141592653589793e0f64;
            let LV = staged[67];
            let MB = 4e1f64;
            let MX = parameters[83];
            let NP = staged[72];
            let NT = staged[73];
            let OA = staged[325];
            let OJ = staged[80];
            let OP = staged[326];
            let PI = staged[82];
            let PN = staged[83];
            let PR = Lanes([0e0f64; 3]);
            let PW = staged[84];
            let PX = 1e-2f64;
            let QC = staged[86];
            let QE = staged[88];
            let QW = staged[92];
            let QZ = staged[93];
            let RG = staged[94];
            let RJ = staged[95];
            let RL = staged[96];
            let RO = staged[97];
            let RQ = parameters[70];
            let RT = parameters[66];
            let RU = parameters[67];
            let RX = staged[99];
            let RY = parameters[69];
            let SD = staged[101];
            let SG = staged[105];
            let SO = parameters[10];
            let SQ = 3.20438e-19f64;
            let SR = parameters[49];
            let SS = staged[107];
            let TK = staged[108];
            let TM = 3.947841e1f64;
            let TR = staged[109];
            let TS = staged[110];
            let TV = staged[111];
            let UA = staged[112];
            let UB = staged[113];
            let UG = staged[114];
            let US = 6.534e-2f64;
            let UV = 8.57973e0f64;
            let UY = 7.895683e1f64;
            let VA = -4e0f64;
            let VG = 2.8985507246376816e0f64;
            let VK = 5e1f64;
            let WM = -2e0f64;
            let XA = 0e0f64;
            let XC = -2e0f64;
            let XH = -4e0f64;
            let XL = staged[115];
            let XT = -1e1f64;
            let XV = 1e1f64;
            let YI = -2e0f64;
            let YN = -4e0f64;
            let YY = -1e1f64;
            let ZB = 4e0f64;
            let ZF = 1.05e0f64;
            let AAJ = -5e-1f64;
            let AAM = 2.5e-1f64;
            let ABB = -2.5e-1f64;
            let ACD = staged[116];
            let ACI = staged[117];
            let ADO = -5e-1f64;
            let AEF = -2.5e-1f64;
            let AFF = staged[118];
            let AGP = -5e-1f64;
            let AHG = -2.5e-1f64;
            let AIG = staged[119];
            let AJQ = -5e-1f64;
            let AKH = -2.5e-1f64;
            let ALH = staged[120];
            let AMR = -5e-1f64;
            let ANI = -2.5e-1f64;
            let AOI = staged[121];
            let AQK = staged[103];
            let AQO = staged[122];
            let AQU = staged[7];
            let ARE = staged[123];
            let ARG = staged[124];
            let ARJ = 1e-3f64;
            let ARL = staged[125];
            let ARN = staged[126];
            let ARS = staged[127];
            let ARU = staged[128];
            let ASE = staged[129];
            let ASF = staged[130];
            let ASP = staged[131];
            let ASR = parameters[11];
            let ASV = staged[132];
            let ASX = staged[264];
            let ASY = staged[133];
            let ASZ = staged[134];
            let ATC = staged[135];
            let ATD = staged[136];
            let ATI = staged[137];
            let ATQ = staged[138];
            let AUE = Lanes([0e0f64; 5]);
            let AUF = staged[327];
            let AUK = staged[144];
            let AUN = staged[145];
            let AUP = staged[146];
            let AUQ = staged[147];
            let AUU = staged[139];
            let AVA = staged[140];
            let AVB = staged[141];
            let AVC = staged[142];
            let AVD = parameters[2];
            let AVU = staged[148];
            let AWA = 3e0f64;
            let AWX = staged[149];
            let AYS = -4e0f64;
            let AYY = 2.8985507246376816e0f64;
            let AZZ = -2e0f64;
            let BAO = -2e0f64;
            let BAT = -4e0f64;
            let BBE = -1e1f64;
            let BBS = -2e0f64;
            let BBX = -4e0f64;
            let BCI = -1e1f64;
            let BDQ = -5e-1f64;
            let BEH = -2.5e-1f64;
            let BFJ = staged[150];
            let BGT = -5e-1f64;
            let BHK = -2.5e-1f64;
            let BIK = staged[151];
            let BJU = -5e-1f64;
            let BKL = -2.5e-1f64;
            let BLL = staged[152];
            let BMV = -5e-1f64;
            let BNM = -2.5e-1f64;
            let BOM = staged[153];
            let BPW = -5e-1f64;
            let BQN = -2.5e-1f64;
            let BRN = staged[154];
            let BUE = 6.25e-4f64;
            let BUH = staged[328];
            let BUI = staged[155];
            let BUK = parameters[162];
            let BUP = staged[156];
            let BUU = staged[329];
            let BUV = staged[157];
            let BUX = parameters[189];
            let BWB = staged[158];
            let BWT = staged[159];
            let BXF = staged[160];
            let BXW = staged[161];
            let BYT = staged[162];
            let BYU = staged[163];
            let BYW = staged[164];
            let BZL = staged[166];
            let BZO = staged[330];
            let BZP = staged[167];
            let CAO = staged[331];
            let CAP = staged[332];
            let CAU = staged[333];
            let CAV = parameters[213];
            let CAZ = staged[169];
            let CBM = staged[170];
            let CDB = staged[171];
            let CDH = staged[172];
            let CDX = staged[175];
            let CER = staged[334];
            let CFH = 1.6666666666666666e-1f64;
            let CFK = 1.6666666666666666e-1f64;
            let CFP = staged[335];
            let CFW = staged[336];
            let CFX = staged[337];
            let CFY = Lanes([0e0f64; 4]);
            let CFZ = Lanes([0e0f64; 4]);
            let CGB = staged[338];
            let CGP = staged[187];
            let CGR = staged[188];
            let CGV = staged[189];
            let CGW = parameters[269];
            let CHF = parameters[265];
            let CHH = staged[190];
            let CHI = staged[191];
            let CHL = parameters[271];
            let CHU = parameters[266];
            let CHW = staged[192];
            let CHX = staged[193];
            let CHY = staged[194];
            let CIA = staged[195];
            let CIG = staged[196];
            let CIJ = staged[197];
            let CIQ = staged[339];
            let CIS = staged[199];
            let CIZ = 1.804851387e-35f64;
            let CJE = staged[201];
            let CJL = staged[202];
            let CJM = staged[204];
            let CJO = staged[205];
            let CJS = staged[206];
            let CKD = staged[207];
            let CKM = Lanes([0e0f64; 3]);
            let CKR = 6e-1f64;
            let CLG = staged[340];
            let CLJ = 8e-2f64;
            let CLU = staged[208];
            let CLV = staged[210];
            let CLX = staged[211];
            let CMB = staged[212];
            let CMI = staged[213];
            let CML = staged[214];
            let CMM = staged[216];
            let CMO = staged[217];
            let CMV = staged[218];
            let CND = staged[219];
            let CNW = staged[220];
            let COC = staged[221];
            let COD = staged[223];
            let COF = staged[224];
            let COT = staged[341];
            let COU = parameters[234];
            let CPO = staged[225];
            let CPU = staged[226];
            let CPV = staged[228];
            let CQB = parameters[235];
            let CRA = staged[342];
            let CRF = staged[233];
            let CRP = staged[234];
            let CSU = staged[240];
            let CSX = staged[241];
            let CTL = staged[343];
            let CTM = staged[252];
            let CTS = staged[242];
            let CTW = staged[344];
            let CTY = parameters[288];
            let CUD = 1.60219e-19f64;
            let CUI = parameters[289];
            let CUU = staged[345];
            let CUV = staged[254];
            let CUX = staged[255];
            let CVO = 1e-12f64;
            let CXL = ddt_scale();
            let CYI = staged[346];
            let CYJ = Lanes([0e0f64; 5]);
            let CYK = Lanes([0e0f64; 5]);
            let CYN = node_potentials[0];
            let CYP = 1e0f64;
            let CYU = node_potentials[2];
            let CYW = 1e0f64;
            let CZK = Lanes([0e0f64; 6]);
            let CZN = staged[347];
            let CZO = Lanes([0e0f64; 2]);
            let CZP = 1e0f64;
            let CZQ = staged[256];
            let CZZ = 0e0f64;
            let DAA = 0e0f64;
            let DAB = 0e0f64;
            let DAC = 0e0f64;
            let DAH = staged[348];
            let DAI = Lanes([0e0f64; 7]);
            let DBV = staged[257];
            let DBY = staged[258];
            let DHU = 0e0f64;
            let DHV = 0e0f64;
            let H;
            let I;
            if B != 0.0 {
                let D = (temperature + C) + parameters[9];
                H = D;
                I = E;
            } else {
                H = F;
                I = G;
            }
            let K = if H > J { 1.0 } else { 0.0 };
            let L = H - J;
            let M = I * L;
            let N = ((L * L) + 2.5e-5f64).sqrt();
            let R = Q * ((H + J) - N);
            let S = (I - ((M + M) * (P / (O * N)))) * Q;
            let U = R / T;
            let V = S / T;
            let W = R - T;
            let Y = X * R;
            let Z = S * X;
            let AB = AA * R;
            let AC = R + parameters[300];
            let AD = (AB * R) / AC;
            let AE = parameters[55] - AD;
            let AG = (((((S * AA) * R) + (S * AB)) - (S * AD)) / AC) * AF;
            let AI = R / AH;
            let AJ = S / AH;
            let AK = AI.sqrt();
            let AM = AL * (AI * AK);
            let AO = AN * Y;
            let AP = Z * AN;
            let AQ = AE / AO;
            let AR = staged[14] - AQ;
            let AS = rspice_limited_exp(AR);
            let AT = AM * AS;
            let AU = ((((AJ * AK) + ((AJ * (P / (O * AK))) * AI)) * AL) * AS) + (((((AG - (AP * AQ)) / AO) * AF) * (rspice_limited_exp_derivative(AR))) * AM);
            let AV = AT * AT;
            let AW = AU * AT;
            let AX = staged[15] / AV;
            let AZ = if AX >= AY { AX } else { AY };
            let BA = AZ.ln();
            let BB = Y * BA;
            let BC = (Z * BA) + (((((((AW + AW) * AX) * AF) / AV) * (if AX >= AY { 1.0 } else { 0.0 })) * (P / AZ)) * Y);
            let BD = staged[16] / AT;
            let BE = if BD >= AY { BD } else { AY };
            let BF = BE.ln();
            let BG = Y * BF;
            let BH = (Z * BF) + ((((((AU * BD) * AF) / AT) * (if BD >= AY { 1.0 } else { 0.0 })) * (P / BE)) * Y);
            let BI = Q * AE;
            let BJ = AG * Q;
            let BK = parameters[52] / AT;
            let BL = if BK >= AY { BK } else { AY };
            let BM = BL.ln();
            let BN = (Z * BM) + ((((((AU * BK) * AF) / AT) * (if BK >= AY { 1.0 } else { 0.0 })) * (P / BL)) * Y);
            let BO = BI - (Y * BM);
            let BP = ((BO * BO) + 4e-8f64).sqrt();
            let BQ = BI - (Q * (BO + BP));
            let BR = BJ - (((BJ - BN) + ((((BJ - BN) * BO) + ((BJ - BN) * BO)) * (P / (O * BP)))) * Q);
            let BV;
            let BW;
            if BS != 0.0 {
                let EZ;
                let FA;
                if BT != 0.0 {
                    let EW = staged[18] + BQ;
                    EZ = EW;
                    FA = BR;
                } else {
                    let EX = staged[20] - BQ;
                    let EY = BR * AF;
                    EZ = EX;
                    FA = EY;
                }
                BV = EZ;
                BW = FA;
            } else {
                BV = BU;
                BW = G;
            }
            let BX = AE / AN;
            let BY = AG / AN;
            let BZ = parameters[53] + BX;
            let CC = CB * (CA - BZ);
            let CD = (BY * AF) * CB;
            let CE = CB * (BV - BZ);
            let CF = (BW - BY) * CB;
            let CG = staged[23] / AT;
            let CH = if CG >= AY { CG } else { AY };
            let CI = CH.ln();
            let CJ = Y * CI;
            let CK = (Z * CI) + ((((((AU * CG) * AF) / AT) * (if CG >= AY { 1.0 } else { 0.0 })) * (P / CH)) * Y);
            let CL = BZ - (CB * (if BX <= CJ { BX } else { CJ }));
            let CM = BY - ((CK + ((BY - CK) * (if BX <= CJ { 1.0 } else { 0.0 }))) * CB);
            let CN = CB * (CA - CL);
            let CO = (CM * AF) * CB;
            let CP = CB * (BV - CL);
            let CQ = (BW - CM) * CB;
            let CT = CS * (U.powf(CR));
            let CV = S * CU;
            let CX = CW + (CU * W);
            let CY = CV * CX;
            let CZ = ((CX * CX) + 4e-6f64).sqrt();
            let DB = (DA + (Q * (CX + CZ))) - 9.000011111097395e-1f64;
            let DC = CT * DB;
            let DD = (((V * (CR * (U.powf(staged[259])))) * CS) * DB) + (((CV + ((CY + CY) * (P / (O * CZ)))) * Q) * CT);
            let DF = S * DE;
            let DH = (DA + (DE * W)) - DG;
            let DI = DF * DH;
            let DJ = ((DH * DH) + 4e-6f64).sqrt();
            let DL = DK * (Q * (DH + DJ));
            let DM = ((DF + ((DI + DI) * (P / (O * DJ)))) * Q) * DK;
            let DO = S * DN;
            let DP = (DA + (DN * W)) - DG;
            let DQ = DO * DP;
            let DR = ((DP * DP) + 4e-6f64).sqrt();
            let DT = DS * (Q * (DP + DR));
            let DU = ((DO + ((DQ + DQ) * (P / (O * DR)))) * Q) * DS;
            let DX = DW * (U.powf(DV));
            let DY = (V * (DV * (U.powf(staged[260])))) * DW;
            let EB = EA * (U.powf(DZ));
            let EC = (V * (DZ * (U.powf(staged[261])))) * EA;
            let EE = S * ED;
            let EF = (DA + (ED * W)) - DG;
            let EG = EE * EF;
            let EH = ((EF * EF) + 4e-6f64).sqrt();
            let EI = Q * (EF + EH);
            let EJ = (EE + ((EG + EG) * (P / (O * EH)))) * Q;
            let EL = CW - (EK * W);
            let EM = (S * EK) * AF;
            let EN = EL * EL;
            let EO = EM * EL;
            let EP = EO + EO;
            let EQ = (EN + 4e-6f64).sqrt();
            let ES = ER * ((DA + (Q * (EL + EQ))) - 9.000011111097395e-1f64);
            let ET = ((EM + (EP * (P / (O * EQ)))) * Q) * ER;
            let EV = if ES < EU { 1.0 } else { 0.0 };
            let FB;
            let FC;
            if EV != 0.0 {
                FB = EU;
                FC = G;
            } else {
                FB = ES;
                FC = ET;
            }
            let FD = (EN + 4e-6f64).sqrt();
            let FF = FE * ((DA + (Q * (EL + FD))) - 9.000011111097395e-1f64);
            let FG = ((EM + (EP * (P / (O * FD)))) * Q) * FE;
            let FH = if FF < EU { 1.0 } else { 0.0 };
            let FI;
            let FJ;
            if FH != 0.0 {
                FI = EU;
                FJ = G;
            } else {
                FI = FF;
                FJ = FG;
            }
            let FK = (EN + 4e-6f64).sqrt();
            let FM = FL * ((DA + (Q * (EL + FK))) - 9.000011111097395e-1f64);
            let FN = ((EM + (EP * (P / (O * FK)))) * Q) * FL;
            let FO = if FM < EU { 1.0 } else { 0.0 };
            let FP;
            let FQ;
            if FO != 0.0 {
                FP = EU;
                FQ = G;
            } else {
                FP = FM;
                FQ = FN;
            }
            let FS = FR * W;
            let FT = S * FR;
            let FV = (FS - -9e-1f64) - FU;
            let FW = (FS - -9e-1f64) - FU;
            let FX = ((FV * FW) - -3.6e-4f64).sqrt();
            let FZ = FY * (DA + (-9e-1f64 + (Q * (((FS - -9e-1f64) - FU) + FX))));
            let GA = ((FT + (((FT * FW) + (FT * FV)) * (P / (O * FX)))) * Q) * FY;
            let GC = CW - (GB * W);
            let GD = (S * GB) * AF;
            let GE = GD * GC;
            let GF = ((GC * GC) + 4e-6f64).sqrt();
            let GH = GG * ((DA + (Q * (GC + GF))) - 9.000011111097395e-1f64);
            let GI = ((GD + ((GE + GE) * (P / (O * GF)))) * Q) * GG;
            let GL = (S * GJ) * GK;
            let GM = (GK * (DA + (GJ * W))) - AN;
            let GN = GL * GM;
            let GO = ((GM * GM) + 4e-6f64).sqrt();
            let GP = (GL + ((GN + GN) * (P / (O * GO)))) * Q;
            let GQ = (Q * (GM + GO)) + AN;
            let GS = S * GR;
            let GT = staged[44] + (GR * W);
            let GV = S * GU;
            let GX = ((GU * W) - GW) - DG;
            let GY = GV * GX;
            let GZ = ((GX * GX) - staged[47]).sqrt();
            let HA = (GV + ((GY + GY) * (P / (O * GZ)))) * Q;
            let HB = staged[48] + (GW + (Q * (GX + GZ)));
            let HD = S * HC;
            let HE = staged[50] + (HC * W);
            let HG = (S * HF) * AF;
            let HH = (DA - (HF * W)) - DG;
            let HI = HG * HH;
            let HJ = ((HH * HH) + 4e-6f64).sqrt();
            let HL = HK * (Q * (HH + HJ));
            let HM = ((HG + ((HI + HI) * (P / (O * HJ)))) * Q) * HK;
            let HN = U - DA;
            let HP = HO * HN;
            let HQ = V * HO;
            let HT = HS * (U.powf(HR));
            let HU = (V * (HR * (U.powf(staged[262])))) * HS;
            let HW = S * HV;
            let HX = (DA + (HV * W)) - DG;
            let HY = HW * HX;
            let HZ = ((HX * HX) + 4e-6f64).sqrt();
            let IB = IA * (Q * (HX + HZ));
            let IC = ((HW + ((HY + HY) * (P / (O * HZ)))) * Q) * IA;
            let IE = S * ID;
            let IF = (DA + (ID * W)) - DG;
            let IG = IE * IF;
            let IH = ((IF * IF) + 4e-6f64).sqrt();
            let IJ = II * (Q * (IF + IH));
            let IK = ((IE + ((IG + IG) * (P / (O * IH)))) * Q) * II;
            let IL = if U >= AY { U } else { AY };
            let IN = IM * (IL.ln());
            let IO = rspice_limited_exp(IN);
            let IP = (((V * (if U >= AY { 1.0 } else { 0.0 })) * (P / IL)) * IM) * (rspice_limited_exp_derivative(IN));
            let IR = IQ * IO;
            let IS = IP * IQ;
            let IX = CB * (IT - IU);
            let IY = (Lanes([0.0, IV]) - Lanes([IW, 0.0])) * CB;
            let JA = IZ - IU;
            let JC = Lanes([JB, 0.0]);
            let JD = Lanes([0.0, IW]);
            let JE = JC - JD;
            let JF = CB * JA;
            let JG = JE * CB;
            let JH = CB * (IT - IZ);
            let JI = (Lanes([0.0, IV]) - Lanes([JB, 0.0])) * CB;
            let JL = Lanes([JK, 0.0]);
            let JM = Lanes([0.0, IW]);
            let JN = CB * (JJ - IU);
            let JO = (JL - JM) * CB;
            let JP = Lanes([JK, 0.0]);
            let JQ = Lanes([0.0, JB]);
            let JR = CB * (JJ - IZ);
            let JS = (JP - JQ) * CB;
            let JT = CB * (IT - JJ);
            let JU = (Lanes([0.0, IV]) - Lanes([JK, 0.0])) * CB;
            let JW = if JF < JV { 1.0 } else { 0.0 };
            let KG;
            let KH;
            let KI;
            let KJ;
            let KK;
            let KL;
            let KM;
            let KN;
            let KO;
            if JW != 0.0 {
                let JX = -JF;
                let JY = JG * AF;
                let JZ = Lanes([JS[0], JS[1], 0.0]);
                let KA = Lanes([JI[0], 0.0, JI[1]]);
                let KB = Lanes([JO[0], 0.0, JO[1]]);
                KG = JX;
                KH = JR;
                KI = JH;
                KJ = JN;
                KK = KC;
                KL = JY;
                KM = JZ;
                KN = KA;
                KO = KB;
            } else {
                let KD = Lanes([JO[0], 0.0, JO[1]]);
                let KE = Lanes([0.0, IY[0], IY[1]]);
                let KF = Lanes([JS[0], JS[1], 0.0]);
                KG = JF;
                KH = JN;
                KI = IX;
                KJ = JR;
                KK = DA;
                KL = JG;
                KM = KD;
                KN = KE;
                KO = KF;
            }
            let KQ = KP - IZ;
            let KS = Lanes([0.0, KR]) - Lanes([JB, 0.0]);
            let KT = CB * KQ;
            let KU = KS * CB;
            let KV = KP - IU;
            let KW = Lanes([0.0, KR]) - Lanes([IW, 0.0]);
            let KX = CB * KV;
            let KY = KW * CB;
            let KZ = KL * KG;
            let LA = ((KG * KG) + 4e-4f64).sqrt();
            let LB = (KZ + KZ) * (P / (O * LA));
            let LD = LA - LC;
            let LE = Q * (LD - KG);
            let LF = (LB - KL) * Q;
            let LG = KH + LE;
            let LH = KM + Lanes([0.0, LF[0], LF[1]]);
            let LI = KI - CC;
            let LJ = Lanes([0.0, KN[0], KN[1], KN[2]]) - Lanes([CD, 0.0, 0.0, 0.0]);
            let LK = KH - CE;
            let LL = Lanes([KM[0], 0.0, KM[1], KM[2]]);
            let LM = LL - Lanes([0.0, CF, 0.0, 0.0]);
            let LO = LJ * LN;
            let LQ = LM * LP;
            let LT = staged[66] + (LS * ((((LI * LN) + (LK * LP)) / LR) + LE));
            let LW = ((((((Lanes([0.0, LO[0], LO[1], LO[2], LO[3]]) + Lanes([LQ[0], LQ[1], LQ[2], LQ[3], 0.0])) / LR) + Lanes([0.0, 0.0, LF[0], LF[1], 0.0])) * LS) * (P / (P + (LT * LT)))) / LU) * LV;
            let LX = staged[68] + ((((LT.atan()) / LU) + Q) * LV);
            let LY = staged[69] / LX;
            let LZ = ((LW * LY) * AF) / LX;
            let MA = LY + DG;
            let MC = if MA < MB { 1.0 } else { 0.0 };
            let MJ;
            let MK;
            if MC != 0.0 {
                let MD = (MA.cosh()) - DA;
                let ME = Q / MD;
                let MF = (((LZ * (MA.sinh())) * ME) * AF) / MD;
                MJ = ME;
                MK = MF;
            } else {
                let MG = -MA;
                let MH = rspice_limited_exp(MG);
                let MI = (LZ * AF) * (rspice_limited_exp_derivative(MG));
                MJ = MH;
                MK = MI;
            }
            let ML = staged[70] / LX;
            let MM = ((LW * ML) * AF) / LX;
            let MN = ML + DG;
            let MO = if MN < MB { 1.0 } else { 0.0 };
            let MV;
            let MW;
            if MO != 0.0 {
                let MP = (MN.cosh()) - DA;
                let MQ = Q / MP;
                let MR = (((MM * (MN.sinh())) * MQ) * AF) / MP;
                MV = MQ;
                MW = MR;
            } else {
                let MS = -MN;
                let MT = rspice_limited_exp(MS);
                let MU = (MM * AF) * (rspice_limited_exp_derivative(MS));
                MV = MT;
                MW = MU;
            }
            let NJ;
            let NK;
            if MO != 0.0 {
                let MY = DA + (MX * ((MN.cosh()) - AN));
                let MZ = if MY >= DG { MY } else { DG };
                let NA = DA / MZ;
                let NB = (((((MM * (MN.sinh())) * MX) * (if MY >= DG { 1.0 } else { 0.0 })) * NA) * AF) / MZ;
                NJ = NA;
                NK = NB;
            } else {
                let NC = -MN;
                let ND = rspice_limited_exp(NC);
                let NE = (MM * AF) * (rspice_limited_exp_derivative(NC));
                let NF = ND + MX;
                let NG = if NF >= DG { NF } else { DG };
                let NH = ND / NG;
                let NI = (NE - ((NE * (if NF >= DG { 1.0 } else { 0.0 })) * NH)) / NG;
                NJ = NH;
                NK = NI;
            }
            let NL = staged[71] / LX;
            let NM = ((LW * NL) * AF) / LX;
            let NN = NL + DG;
            let NO = if NN < MB { 1.0 } else { 0.0 };
            let NY;
            let NZ;
            if NO != 0.0 {
                let NQ = (NN.cosh()) - DA;
                let NR = (Q * NP) / NQ;
                let NS = (((NM * (NN.sinh())) * NR) * AF) / NQ;
                let NU = NR + NT;
                NY = NU;
                NZ = NS;
            } else {
                let NV = -NN;
                let NW = ((NM * AF) * (rspice_limited_exp_derivative(NV))) * NP;
                let NX = (NP * (rspice_limited_exp(NV))) + NT;
                NY = NX;
                NZ = NW;
            }
            let OH;
            let OI;
            if OA != 0.0 {
                let OB = staged[74] / LX;
                let OC = ((LW * OB) * AF) / LX;
                let OD = if OB > MB { 1.0 } else { 0.0 };
                let OU;
                let OV;
                if OD != 0.0 {
                    let OQ = (rspice_limited_exp(OB)) / AN;
                    let OR = (OC * (rspice_limited_exp_derivative(OB))) / AN;
                    OU = OQ;
                    OV = OR;
                } else {
                    let OS = OC * (OB.sinh());
                    let OT = (OB.cosh()) - DA;
                    OU = OT;
                    OV = OS;
                }
                let OW = staged[75] / OU;
                let OX = staged[76] - OW;
                let OY = (((OV * OW) * AF) / OU) * AF;
                OH = OX;
                OI = OY;
            } else {
                let OE = staged[77] / LX;
                let OF = ((LW * OE) * AF) / LX;
                let OG = if OE > MB { 1.0 } else { 0.0 };
                let PD;
                let PE;
                if OG != 0.0 {
                    let OZ = (rspice_limited_exp(OE)) / AN;
                    let PA = (OF * (rspice_limited_exp_derivative(OE))) / AN;
                    PD = OZ;
                    PE = PA;
                } else {
                    let PB = OF * (OE.sinh());
                    let PC = (OE.cosh()) - DA;
                    PD = PC;
                    PE = PB;
                }
                let PF = staged[78] / PD;
                let PG = staged[79] - PF;
                let PH = (((PE * PF) * AF) / PD) * AF;
                OH = PG;
                OI = PH;
            }
            let OK = OH - OJ;
            let OL = OI * OK;
            let OM = ((OK * OK) + FU).sqrt();
            let ON = (OI + ((OL + OL) * (P / (O * OM)))) * Q;
            let OO = OJ + (Q * (OK + OM));
            let PS;
            let PT;
            if OP != 0.0 {
                let PJ = PI * ((CB * LG) - staged[81]);
                let PK = (LH * CB) * PI;
                let PL = PK * PJ;
                let PM = ((PJ * PJ) + 4e-4f64).sqrt();
                let PO = (DA + ((Q * (PJ + PM)) / PN)).sqrt();
                let PP = (((PK + ((PL + PL) * (P / (O * PM)))) * Q) / PN) * (P / (O * PO));
                let PQ = PO - DA;
                PS = PQ;
                PT = PP;
            } else {
                PS = JV;
                PT = PR;
            }
            let PU = PN * PS;
            let PV = (((PT * PN) * PS) + (PT * PU)) * AF;
            let PY = ((-(PU * PS)) - PW) - PX;
            let PZ = PV * PY;
            let QA = ((PY * PY) - staged[85]).sqrt();
            let QB = LF * AF;
            let QD = QC * OO;
            let QF = (((PV + ((PZ + PZ) * (P / (O * QA)))) * Q) * AF) * QE;
            let QG = (LK - (QE * (-(PW + (Q * (PY + QA)))))) - (-1.2e0f64 - LE);
            let QH = QD * QG;
            let QI = ((LM - Lanes([QF[0], 0.0, QF[1], QF[2]])) - Lanes([0.0, 0.0, QB[0], QB[1]])) * QD;
            let QJ = ((ON * QC) * QG) + Lanes([QI[0], QI[1], QI[2], QI[3], 0.0]);
            let QK = LH * LG;
            let QL = ((LG * LG) + 4e-6f64).sqrt();
            let QM = Q * (LG + QL);
            let QN = (LH + ((QK + QK) * (P / (O * QL)))) * Q;
            let QO = (4e-1f64 + BG) + staged[89];
            let QP = if QO < JV { 1.0 } else { 0.0 };
            let QU;
            let QV;
            if QP != 0.0 {
                QU = JV;
                QV = G;
            } else {
                let QQ = staged[90] * staged[91];
                let QR = QO.sqrt();
                let QS = QQ * QR;
                let QT = (BH * (P / (O * QR))) * QQ;
                QU = QS;
                QV = QT;
            }
            let QX = QW * MJ;
            let QY = BB - QO;
            let RA = LH * QZ;
            let RB = -(FZ + (QZ * LG));
            let RC = RB * MV;
            let RD = ((Lanes([0.0, GA, 0.0, 0.0]) + Lanes([RA[0], 0.0, RA[1], RA[2]])) * AF) * MV;
            let RE = LD + PX;
            let RF = RE.sqrt();
            let RH = LD + (RG * RF);
            let RI = (LB + ((LB * (P / (O * RF))) * RG)) * RC;
            let RK = RJ * NJ;
            let RM = RE.powf(RL);
            let RN = (LB * (RL * (RE.powf(staged[263])))) * RK;
            let RP = LB * RO;
            let RR = staged[98] + (RQ * QM);
            let RS = LB * RR;
            let RV = RU * LG;
            let RW = (LH * RT) + (((LH * RU) * LG) + (LH * RV));
            let RZ = RY * LG;
            let SA = ((staged[100] + (RX * LG)) + (RZ * LG)) + (RR * LD);
            let SB = (((LH * RX) + (((LH * RY) * LG) + (LH * RZ))) + (((QN * RQ) * LD) + Lanes([0.0, RS[0], RS[1]]))) * MJ;
            let SC = staged[102] + (((RT * LG) + (RV * LG)) + (MJ * SA));
            let SE = (Y * SC) / SD;
            let SF = (Lanes([0.0, (Z * SC), 0.0, 0.0, 0.0]) + ((Lanes([RW[0], 0.0, RW[1], RW[2], 0.0]) + ((MK * SA) + Lanes([SB[0], 0.0, SB[1], SB[2], 0.0]))) * Y)) / SD;
            let SH = SG * LG;
            let SI = (LH * SG) * HN;
            let SJ = Lanes([0.0, HQ, 0.0, 0.0]) + (Lanes([SI[0], 0.0, SI[1], SI[2]]) + Lanes([0.0, (V * SH), 0.0, 0.0]));
            let SK = ((((((QX * QY) + ((RC * RH) + (RK * RM))) + QU) + (RO * LD)) + staged[106]) + (HP + (SH * HN))) + QH;
            let SL = (((((((MK * QW) * QY) + Lanes([0.0, ((BC - BH) * QX), 0.0, 0.0, 0.0])) + ((((Lanes([RD[0], RD[1], RD[2], RD[3], 0.0]) + (MW * RB)) * RH) + Lanes([0.0, 0.0, RI[0], RI[1], 0.0])) + (((NK * RJ) * RM) + Lanes([0.0, 0.0, RN[0], RN[1], 0.0])))) + Lanes([0.0, QV, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, RP[0], RP[1], 0.0])) + Lanes([SJ[0], SJ[1], SJ[2], SJ[3], 0.0])) + QJ;
            let SM = Lanes([0.0, LJ[0], LJ[1], LJ[2], LJ[3]]);
            let SN = SM - SL;
            let SP = (LI - SK) + SO;
            let ST = SS * Y;
            let SU = (((SQ * AT) * SR) * SR) / ST;
            let SV = ((((AU * SQ) * SR) * SR) - ((Z * SS) * SU)) / ST;
            let SW = SU.ln();
            let SX = SV * (P / SU);
            let SY = 3.675753940198048e0f64 - SW;
            let SZ = SX * AF;
            let TA = SP / SE;
            let TB = (SN - (SF * TA)) / SE;
            let TC = LK - SK;
            let TD = Lanes([LM[0], LM[1], LM[2], LM[3], 0.0]) - SL;
            let TE = TC + SO;
            let TF = TE / SE;
            let TG = (TD - (SF * TF)) / SE;
            let TH = TA - SY;
            let TI = Lanes([0.0, SZ, 0.0, 0.0, 0.0]);
            let TJ = TB - TI;
            let TL = TK * TH;
            let TN = (TL * TH) + TM;
            let TO = (TN.ln()) - SW;
            let TP = Lanes([0.0, SX, 0.0, 0.0, 0.0]);
            let TQ = ((((TJ * TK) * TH) + (TJ * TL)) * (P / TN)) - TP;
            let TT = (TO + (TR * TF)) / TS;
            let TU = (TQ + (TG * TR)) / TS;
            let TW = TF + (TV * (TA - TF));
            let TX = if TW <= TO { TW } else { TO };
            let TY = if TX <= SY { TX } else { SY };
            let TZ = TI + (((TQ + (((TG + ((TB - TG) * TV)) - TQ) * (if TW <= TO { 1.0 } else { 0.0 }))) - TI) * (if TX <= SY { 1.0 } else { 0.0 }));
            let UC = (TY + (UA * TA)) / UB;
            let UD = (TZ + (TB * UA)) / UB;
            let UE = TF - TT;
            let UF = TG - TU;
            let UH = UG * UE;
            let UI = TT.exp();
            let UJ = (UH * UE) - (SU * UI);
            let UK = (((UF * UG) * UE) + (UF * UH)) - (Lanes([0.0, (SV * UI), 0.0, 0.0, 0.0]) + ((TU * UI) * SU));
            let UL = if UJ < JV { 1.0 } else { 0.0 };
            let VN;
            let VO;
            if UL != 0.0 {
                let UM = (TF - TY) * TR;
                let UN = (TG - TZ) * TR;
                let UO = MB * UA;
                let UP = UO + UM;
                let UQ = UO * UM;
                let UR = UN * UO;
                let UT = UN * US;
                let UU = (US * UP) + DA;
                let UW = (UN * UV) + UR;
                let UX = ((UP * UV) + UQ) + TM;
                let UZ = (UY * UP) + (TM * UQ);
                let VB = VA * UU;
                let VC = UW * UX;
                let VD = ((VB * UZ) + (UX * UX)).sqrt();
                let VE = AN * UU;
                let VF = ((-UX) + VD) / VE;
                let VH = ((-((TA - (((SY * UB) - TY) / UA)) + AN)) / VG).exp();
                let VI = DA - VH;
                let VJ = VF * VI;
                let VL = if VJ <= VK { VJ } else { VK };
                let VM = ((((((UW * AF) + (((((UT * VA) * UZ) + (((UN * UY) + (UR * TM)) * VB)) + (VC + VC)) * (P / (O * VD)))) - ((UT * AN) * VF)) / VE) * VI) + ((((((TB - ((Lanes([0.0, (SZ * UB), 0.0, 0.0, 0.0]) - TZ) / UA)) * AF) / VG) * VH) * AF) * VF)) * (if VJ <= VK { 1.0 } else { 0.0 });
                VN = VL;
                VO = VM;
            } else {
                VN = UJ;
                VO = UK;
            }
            let VP = if TA >= SY { TA } else { SY };
            let VQ = TI + (TJ * (if TA >= SY { 1.0 } else { 0.0 }));
            let VR = VP - SY;
            let VS = VQ - TI;
            let VT = TK * VR;
            let VU = (VT * VR) + TM;
            let VV = SY * UB;
            let VW = Lanes([0.0, (SZ * UB), 0.0, 0.0, 0.0]);
            let VX = ((VV - TY) / UA) - SY;
            let VY = ((VW - TZ) / UA) - TI;
            let VZ = TK * VX;
            let WA = (VZ * VX) + TM;
            let WB = ((VU.ln()) - SW) - (((WA.ln()) - SW) - SY);
            let WC = (((((VS * TK) * VR) + (VS * VT)) * (P / VU)) - TP) - ((((((VY * TK) * VX) + (VY * VZ)) * (P / WA)) - TP) - TI);
            let WD = VP - WB;
            let WE = VQ - WC;
            let WF = -SU;
            let WG = SV * AF;
            let WH = WB.exp();
            let WI = WF * WH;
            let WJ = Lanes([0.0, (WG * WH), 0.0, 0.0, 0.0]) + ((WC * WH) * WF);
            let WK = TK * WD;
            let WL = WE * TK;
            let WN = (WM * WK) + WI;
            let WO = (-(((WK * WD) + WI) - VN)) / WN;
            let WP = WB + WO;
            let WQ = WC + (((((((WL * WD) + (WE * WK)) + WJ) - VO) * AF) - (((WL * WM) + WJ) * WO)) / WN);
            let WR = VP - WP;
            let WS = VQ - WQ;
            let WT = TK * WR;
            let WU = WS * TK;
            let WV = (WT * WR) - VN;
            let WW = ((WU * WR) + (WS * WT)) - VO;
            let WX = DA / WV;
            let WY = ((WW * WX) * AF) / WV;
            let WZ = WV.abs();
            let XB = ((WZ.ln()) - SW) - WP;
            let XD = XC * WT;
            let XE = (XD * WX) - DA;
            let XF = DA / XE;
            let XG = (((((WU * XC) * WX) + (WY * XD)) * XF) * AF) / XE;
            let XI = XH * WT;
            let XJ = XI * WT;
            let XK = XJ * WX;
            let XM = (XK * WX) + (XL * WX);
            let XN = XB * XF;
            let XO = (((((WW * ((O * (if WV >= XA { 1.0 } else { 0.0 })) - P)) * (P / WZ)) - TP) - WQ) * XF) + (XG * XB);
            let XP = Q * XN;
            let XQ = XP * XN;
            let XR = XQ * XM;
            let XS = (-XN) - (XR * XF);
            let XU = if XS >= XT { XS } else { XT };
            let XW = WP + (if XU <= XV { XU } else { XV });
            let XX = WQ + ((((XO * AF) - (((((((XO * Q) * XN) + (XO * XP)) * XM) + (((((((((WU * XH) * WT) + (WU * XI)) * WX) + (WY * XJ)) * WX) + (WY * XK)) + (WY * XL)) * XQ)) * XF) + (XG * XR))) * (if XS >= XT { 1.0 } else { 0.0 })) * (if XU <= XV { 1.0 } else { 0.0 }));
            let XY = VP - XW;
            let XZ = VQ - XX;
            let YA = TK * XY;
            let YB = XZ * TK;
            let YC = (YA * XY) - VN;
            let YD = ((YB * XY) + (XZ * YA)) - VO;
            let YE = DA / YC;
            let YF = ((YD * YE) * AF) / YC;
            let YG = YC.abs();
            let YH = ((YG.ln()) - SW) - XW;
            let YJ = YI * YA;
            let YK = (YJ * YE) - DA;
            let YL = DA / YK;
            let YM = (((((YB * YI) * YE) + (YF * YJ)) * YL) * AF) / YK;
            let YO = YN * YA;
            let YP = YO * YA;
            let YQ = YP * YE;
            let YR = (YQ * YE) + (XL * YE);
            let YS = YH * YL;
            let YT = (((((YD * ((O * (if YC >= XA { 1.0 } else { 0.0 })) - P)) * (P / YG)) - TP) - XX) * YL) + (YM * YH);
            let YU = Q * YS;
            let YV = YU * YS;
            let YW = YV * YR;
            let YX = (-YS) - (YW * YL);
            let YZ = if YX >= YY { YX } else { YY };
            let ZA = XW + (if YZ <= XV { YZ } else { XV });
            let ZC = SY - ZB;
            let ZD = if ZA >= ZC { ZA } else { ZC };
            let ZE = TI + (((XX + ((((YT * AF) - (((((((YT * Q) * YS) + (YT * YU)) * YR) + (((((((((YB * YN) * YA) + (YB * YO)) * YE) + (YF * YP)) * YE) + (YF * YQ)) + (YF * XL)) * YV)) * YL) + (YM * YW))) * (if YX >= YY { 1.0 } else { 0.0 })) * (if YZ <= XV { 1.0 } else { 0.0 }))) - TI) * (if ZA >= ZC { 1.0 } else { 0.0 }));
            let ZG = (UC - (ZF * ZD)).exp();
            let ZH = DA + ZG;
            let ZI = UC - (ZH.ln());
            let ZJ = if ZI <= ZD { ZI } else { ZD };
            let ZK = ZE + (((UD - (((UD - (ZE * ZF)) * ZG) * (P / ZH))) - ZE) * (if ZI <= ZD { 1.0 } else { 0.0 }));
            let ZL = TA - ZJ;
            let ZM = TB - ZK;
            let ZN = UA * ZL;
            let ZO = ZM * UA;
            let ZP = ZJ.exp();
            let ZQ = WF * ZP;
            let ZR = Lanes([0.0, (WG * ZP), 0.0, 0.0, 0.0]) + ((ZK * ZP) * WF);
            let ZS = ZO * ZN;
            let ZT = (ZN * ZN) + ZQ;
            let ZU = (ZS + ZS) + ZR;
            let ZV = if ZT < JV { 1.0 } else { 0.0 };
            let ABE;
            let ABF;
            let ABG;
            let ABH;
            let ABI;
            let ABJ;
            let ABK;
            let ABL;
            let ABM;
            let ABN;
            if ZV != 0.0 {
                let ZW = (-ZT).sqrt();
                let ZX = (ZU * AF) * (P / (O * ZW));
                let ZY = Q * ZW;
                let ZZ = ZX * Q;
                let AAA = ZY.sin();
                let AAB = ZY.cos();
                let AAC = DA / AAA;
                let AAD = (((ZZ * AAB) * AAC) * AF) / AAA;
                let AAE = AAC * AAC;
                let AAF = AAD * AAC;
                let AAG = AAF + AAF;
                let AAH = AAB * AAC;
                let AAI = ((ZZ * (AF * AAA)) * AAC) + (AAD * AAB);
                let AAK = (AAJ * AAH) / ZW;
                let AAL = ((AAI * AAJ) - (ZX * AAK)) / ZW;
                let AAN = (AAM * AAE) + AAK;
                let AAO = (AAG * AAM) + AAL;
                ABE = ZW;
                ABF = AAH;
                ABG = AAE;
                ABH = AAK;
                ABI = AAN;
                ABJ = ZX;
                ABK = AAI;
                ABL = AAG;
                ABM = AAL;
                ABN = AAO;
            } else {
                let AAP = ZT.sqrt();
                let AAQ = ZU * (P / (O * AAP));
                let AAR = Q * AAP;
                let AAS = AAR.sinh();
                let AAT = DA / AAS;
                let AAU = AAT * AAT;
                let AAV = (((((AAQ * Q) * (AAR.cosh())) * AAT) * AF) / AAS) * AAT;
                let AAW = AAV + AAV;
                let AAX = (DA + AAU).sqrt();
                let AAY = AAW * (P / (O * AAX));
                let AAZ = (Q * AAX) / AAP;
                let ABA = ((AAY * Q) - (AAQ * AAZ)) / AAP;
                let ABC = (ABB * AAU) + AAZ;
                let ABD = (AAW * ABB) + ABA;
                ABE = AAP;
                ABF = AAX;
                ABG = AAU;
                ABH = AAZ;
                ABI = ABC;
                ABJ = AAQ;
                ABK = AAY;
                ABL = AAW;
                ABM = ABA;
                ABN = ABD;
            }
            let ABO = ZN + (ABE * ABF);
            let ABP = ZO + ((ABJ * ABF) + (ABK * ABE));
            let ABQ = DA / ABO;
            let ABR = ((ABP * ABQ) * AF) / ABO;
            let ABS = TF - TA;
            let ABT = TG - TB;
            let ABU = ZT * ABG;
            let ABV = ABU * ABQ;
            let ABW = ABV * ABQ;
            let ABX = ABW.abs();
            let ABY = (ABS + ZL) - (ABX.ln());
            let ABZ = (ABT + ZM) - ((((((((ZU * ABG) + (ABL * ZT)) * ABQ) + (ABR * ABU)) * ABQ) + (ABR * ABV)) * ((O * (if ABW >= XA { 1.0 } else { 0.0 })) - P)) * (P / ABX));
            let ACA = (TR * ABY) + ZN;
            let ACB = DA / ZT;
            let ACC = ACB - ABH;
            let ACE = (ACD * ZN) + ZQ;
            let ACF = (ZO * ACD) + ZR;
            let ACG = ABI * ACE;
            let ACH = (ABN * ACE) + (ACF * ABI);
            let ACJ = ACI + ACG;
            let ACK = (-1e0f64 + (AN * (ACJ * ABQ))) - (ACC * ACE);
            let ACL = ACG - UA;
            let ACM = ((ZQ - (UA * (ZN + ABO))) + (ZN * ACG)) + (TR * ((ACK * ABO) + (ABY * ACL)));
            let ACN = (-(ZQ + (ABO * ACA))) / ACM;
            let ACO = ZJ + ACN;
            let ACP = ZK + ((((ZR + ((ABP * ACA) + (((ABZ * TR) + ZO) * ABO))) * AF) - ((((ZR - ((ZO + ABP) * UA)) + ((ZO * ACG) + (ACH * ZN))) + ((((((((ACH * ABQ) + (ABR * ACJ)) * AN) - ((((((ZU * ACB) * AF) / ZT) - ABM) * ACE) + (ACF * ACC))) * ABO) + (ABP * ACK)) + ((ABZ * ACL) + (ACH * ABY))) * TR)) * ACN)) / ACM);
            let ACQ = TA - ACO;
            let ACR = TB - ACP;
            let ACS = UA * ACQ;
            let ACT = ACR * UA;
            let ACU = ACO.exp();
            let ACV = WF * ACU;
            let ACW = Lanes([0.0, (WG * ACU), 0.0, 0.0, 0.0]) + ((ACP * ACU) * WF);
            let ACX = ACT * ACS;
            let ACY = (ACS * ACS) + ACV;
            let ACZ = (ACX + ACX) + ACW;
            let ADA = if ACY < JV { 1.0 } else { 0.0 };
            let AEI;
            let AEJ;
            let AEK;
            let AEL;
            let AEM;
            let AEN;
            let AEO;
            let AEP;
            let AEQ;
            let AER;
            if ADA != 0.0 {
                let ADB = (-ACY).sqrt();
                let ADC = (ACZ * AF) * (P / (O * ADB));
                let ADD = Q * ADB;
                let ADE = ADC * Q;
                let ADF = ADD.sin();
                let ADG = ADD.cos();
                let ADH = DA / ADF;
                let ADI = (((ADE * ADG) * ADH) * AF) / ADF;
                let ADJ = ADH * ADH;
                let ADK = ADI * ADH;
                let ADL = ADK + ADK;
                let ADM = ADG * ADH;
                let ADN = ((ADE * (AF * ADF)) * ADH) + (ADI * ADG);
                let ADP = (ADO * ADM) / ADB;
                let ADQ = ((ADN * ADO) - (ADC * ADP)) / ADB;
                let ADR = (AAM * ADJ) + ADP;
                let ADS = (ADL * AAM) + ADQ;
                AEI = ADB;
                AEJ = ADM;
                AEK = ADJ;
                AEL = ADP;
                AEM = ADR;
                AEN = ADC;
                AEO = ADN;
                AEP = ADL;
                AEQ = ADQ;
                AER = ADS;
            } else {
                let ADT = ACY.sqrt();
                let ADU = ACZ * (P / (O * ADT));
                let ADV = Q * ADT;
                let ADW = ADV.sinh();
                let ADX = DA / ADW;
                let ADY = ADX * ADX;
                let ADZ = (((((ADU * Q) * (ADV.cosh())) * ADX) * AF) / ADW) * ADX;
                let AEA = ADZ + ADZ;
                let AEB = (DA + ADY).sqrt();
                let AEC = AEA * (P / (O * AEB));
                let AED = (Q * AEB) / ADT;
                let AEE = ((AEC * Q) - (ADU * AED)) / ADT;
                let AEG = (AEF * ADY) + AED;
                let AEH = (AEA * AEF) + AEE;
                AEI = ADT;
                AEJ = AEB;
                AEK = ADY;
                AEL = AED;
                AEM = AEG;
                AEN = ADU;
                AEO = AEC;
                AEP = AEA;
                AEQ = AEE;
                AER = AEH;
            }
            let AES = ACS + (AEI * AEJ);
            let AET = ACT + ((AEN * AEJ) + (AEO * AEI));
            let AEU = DA / AES;
            let AEV = ((AET * AEU) * AF) / AES;
            let AEW = ACY * AEK;
            let AEX = AEW * AEU;
            let AEY = AEX * AEU;
            let AEZ = AEY.abs();
            let AFA = (ABS + ACQ) - (AEZ.ln());
            let AFB = (ABT + ACR) - ((((((((ACZ * AEK) + (AEP * ACY)) * AEU) + (AEV * AEW)) * AEU) + (AEV * AEX)) * ((O * (if AEY >= XA { 1.0 } else { 0.0 })) - P)) * (P / AEZ));
            let AFC = (TR * AFA) + ACS;
            let AFD = DA / ACY;
            let AFE = AFD - AEL;
            let AFG = (AFF * ACS) + ACV;
            let AFH = (ACT * AFF) + ACW;
            let AFI = AEM * AFG;
            let AFJ = (AER * AFG) + (AFH * AEM);
            let AFK = ACI + AFI;
            let AFL = (-1e0f64 + (AN * (AFK * AEU))) - (AFE * AFG);
            let AFM = AFI - UA;
            let AFN = ((ACV - (UA * (ACS + AES))) + (ACS * AFI)) + (TR * ((AFL * AES) + (AFA * AFM)));
            let AFO = (-(ACV + (AES * AFC))) / AFN;
            let AFP = ACO + AFO;
            let AFQ = ACP + ((((ACW + ((AET * AFC) + (((AFB * TR) + ACT) * AES))) * AF) - ((((ACW - ((ACT + AET) * UA)) + ((ACT * AFI) + (AFJ * ACS))) + ((((((((AFJ * AEU) + (AEV * AFK)) * AN) - ((((((ACZ * AFD) * AF) / ACY) - AEQ) * AFG) + (AFH * AFE))) * AES) + (AET * AFL)) + ((AFB * AFM) + (AFJ * AFA))) * TR)) * AFO)) / AFN);
            let AFR = TA - AFP;
            let AFS = TB - AFQ;
            let AFT = UA * AFR;
            let AFU = AFS * UA;
            let AFV = AFP.exp();
            let AFW = WF * AFV;
            let AFX = Lanes([0.0, (WG * AFV), 0.0, 0.0, 0.0]) + ((AFQ * AFV) * WF);
            let AFY = AFU * AFT;
            let AFZ = (AFT * AFT) + AFW;
            let AGA = (AFY + AFY) + AFX;
            let AGB = if AFZ < JV { 1.0 } else { 0.0 };
            let AHJ;
            let AHK;
            let AHL;
            let AHM;
            let AHN;
            let AHO;
            let AHP;
            let AHQ;
            let AHR;
            let AHS;
            if AGB != 0.0 {
                let AGC = (-AFZ).sqrt();
                let AGD = (AGA * AF) * (P / (O * AGC));
                let AGE = Q * AGC;
                let AGF = AGD * Q;
                let AGG = AGE.sin();
                let AGH = AGE.cos();
                let AGI = DA / AGG;
                let AGJ = (((AGF * AGH) * AGI) * AF) / AGG;
                let AGK = AGI * AGI;
                let AGL = AGJ * AGI;
                let AGM = AGL + AGL;
                let AGN = AGH * AGI;
                let AGO = ((AGF * (AF * AGG)) * AGI) + (AGJ * AGH);
                let AGQ = (AGP * AGN) / AGC;
                let AGR = ((AGO * AGP) - (AGD * AGQ)) / AGC;
                let AGS = (AAM * AGK) + AGQ;
                let AGT = (AGM * AAM) + AGR;
                AHJ = AGC;
                AHK = AGN;
                AHL = AGK;
                AHM = AGQ;
                AHN = AGS;
                AHO = AGD;
                AHP = AGO;
                AHQ = AGM;
                AHR = AGR;
                AHS = AGT;
            } else {
                let AGU = AFZ.sqrt();
                let AGV = AGA * (P / (O * AGU));
                let AGW = Q * AGU;
                let AGX = AGW.sinh();
                let AGY = DA / AGX;
                let AGZ = AGY * AGY;
                let AHA = (((((AGV * Q) * (AGW.cosh())) * AGY) * AF) / AGX) * AGY;
                let AHB = AHA + AHA;
                let AHC = (DA + AGZ).sqrt();
                let AHD = AHB * (P / (O * AHC));
                let AHE = (Q * AHC) / AGU;
                let AHF = ((AHD * Q) - (AGV * AHE)) / AGU;
                let AHH = (AHG * AGZ) + AHE;
                let AHI = (AHB * AHG) + AHF;
                AHJ = AGU;
                AHK = AHC;
                AHL = AGZ;
                AHM = AHE;
                AHN = AHH;
                AHO = AGV;
                AHP = AHD;
                AHQ = AHB;
                AHR = AHF;
                AHS = AHI;
            }
            let AHT = AFT + (AHJ * AHK);
            let AHU = AFU + ((AHO * AHK) + (AHP * AHJ));
            let AHV = DA / AHT;
            let AHW = ((AHU * AHV) * AF) / AHT;
            let AHX = AFZ * AHL;
            let AHY = AHX * AHV;
            let AHZ = AHY * AHV;
            let AIA = AHZ.abs();
            let AIB = (ABS + AFR) - (AIA.ln());
            let AIC = (ABT + AFS) - ((((((((AGA * AHL) + (AHQ * AFZ)) * AHV) + (AHW * AHX)) * AHV) + (AHW * AHY)) * ((O * (if AHZ >= XA { 1.0 } else { 0.0 })) - P)) * (P / AIA));
            let AID = (TR * AIB) + AFT;
            let AIE = DA / AFZ;
            let AIF = AIE - AHM;
            let AIH = (AIG * AFT) + AFW;
            let AII = (AFU * AIG) + AFX;
            let AIJ = AHN * AIH;
            let AIK = (AHS * AIH) + (AII * AHN);
            let AIL = ACI + AIJ;
            let AIM = (-1e0f64 + (AN * (AIL * AHV))) - (AIF * AIH);
            let AIN = AIJ - UA;
            let AIO = ((AFW - (UA * (AFT + AHT))) + (AFT * AIJ)) + (TR * ((AIM * AHT) + (AIB * AIN)));
            let AIP = (-(AFW + (AHT * AID))) / AIO;
            let AIQ = AFP + AIP;
            let AIR = AFQ + ((((AFX + ((AHU * AID) + (((AIC * TR) + AFU) * AHT))) * AF) - ((((AFX - ((AFU + AHU) * UA)) + ((AFU * AIJ) + (AIK * AFT))) + ((((((((AIK * AHV) + (AHW * AIL)) * AN) - ((((((AGA * AIE) * AF) / AFZ) - AHR) * AIH) + (AII * AIF))) * AHT) + (AHU * AIM)) + ((AIC * AIN) + (AIK * AIB))) * TR)) * AIP)) / AIO);
            let AIS = TA - AIQ;
            let AIT = TB - AIR;
            let AIU = UA * AIS;
            let AIV = AIT * UA;
            let AIW = AIQ.exp();
            let AIX = WF * AIW;
            let AIY = Lanes([0.0, (WG * AIW), 0.0, 0.0, 0.0]) + ((AIR * AIW) * WF);
            let AIZ = AIV * AIU;
            let AJA = (AIU * AIU) + AIX;
            let AJB = (AIZ + AIZ) + AIY;
            let AJC = if AJA < JV { 1.0 } else { 0.0 };
            let AKK;
            let AKL;
            let AKM;
            let AKN;
            let AKO;
            let AKP;
            let AKQ;
            let AKR;
            let AKS;
            let AKT;
            if AJC != 0.0 {
                let AJD = (-AJA).sqrt();
                let AJE = (AJB * AF) * (P / (O * AJD));
                let AJF = Q * AJD;
                let AJG = AJE * Q;
                let AJH = AJF.sin();
                let AJI = AJF.cos();
                let AJJ = DA / AJH;
                let AJK = (((AJG * AJI) * AJJ) * AF) / AJH;
                let AJL = AJJ * AJJ;
                let AJM = AJK * AJJ;
                let AJN = AJM + AJM;
                let AJO = AJI * AJJ;
                let AJP = ((AJG * (AF * AJH)) * AJJ) + (AJK * AJI);
                let AJR = (AJQ * AJO) / AJD;
                let AJS = ((AJP * AJQ) - (AJE * AJR)) / AJD;
                let AJT = (AAM * AJL) + AJR;
                let AJU = (AJN * AAM) + AJS;
                AKK = AJD;
                AKL = AJO;
                AKM = AJL;
                AKN = AJR;
                AKO = AJT;
                AKP = AJE;
                AKQ = AJP;
                AKR = AJN;
                AKS = AJS;
                AKT = AJU;
            } else {
                let AJV = AJA.sqrt();
                let AJW = AJB * (P / (O * AJV));
                let AJX = Q * AJV;
                let AJY = AJX.sinh();
                let AJZ = DA / AJY;
                let AKA = AJZ * AJZ;
                let AKB = (((((AJW * Q) * (AJX.cosh())) * AJZ) * AF) / AJY) * AJZ;
                let AKC = AKB + AKB;
                let AKD = (DA + AKA).sqrt();
                let AKE = AKC * (P / (O * AKD));
                let AKF = (Q * AKD) / AJV;
                let AKG = ((AKE * Q) - (AJW * AKF)) / AJV;
                let AKI = (AKH * AKA) + AKF;
                let AKJ = (AKC * AKH) + AKG;
                AKK = AJV;
                AKL = AKD;
                AKM = AKA;
                AKN = AKF;
                AKO = AKI;
                AKP = AJW;
                AKQ = AKE;
                AKR = AKC;
                AKS = AKG;
                AKT = AKJ;
            }
            let AKU = AIU + (AKK * AKL);
            let AKV = AIV + ((AKP * AKL) + (AKQ * AKK));
            let AKW = DA / AKU;
            let AKX = ((AKV * AKW) * AF) / AKU;
            let AKY = AJA * AKM;
            let AKZ = AKY * AKW;
            let ALA = AKZ * AKW;
            let ALB = ALA.abs();
            let ALC = (ABS + AIS) - (ALB.ln());
            let ALD = (ABT + AIT) - ((((((((AJB * AKM) + (AKR * AJA)) * AKW) + (AKX * AKY)) * AKW) + (AKX * AKZ)) * ((O * (if ALA >= XA { 1.0 } else { 0.0 })) - P)) * (P / ALB));
            let ALE = (TR * ALC) + AIU;
            let ALF = DA / AJA;
            let ALG = ALF - AKN;
            let ALI = (ALH * AIU) + AIX;
            let ALJ = (AIV * ALH) + AIY;
            let ALK = AKO * ALI;
            let ALL = (AKT * ALI) + (ALJ * AKO);
            let ALM = ACI + ALK;
            let ALN = (-1e0f64 + (AN * (ALM * AKW))) - (ALG * ALI);
            let ALO = ALK - UA;
            let ALP = ((AIX - (UA * (AIU + AKU))) + (AIU * ALK)) + (TR * ((ALN * AKU) + (ALC * ALO)));
            let ALQ = (-(AIX + (AKU * ALE))) / ALP;
            let ALR = AIQ + ALQ;
            let ALS = AIR + ((((AIY + ((AKV * ALE) + (((ALD * TR) + AIV) * AKU))) * AF) - ((((AIY - ((AIV + AKV) * UA)) + ((AIV * ALK) + (ALL * AIU))) + ((((((((ALL * AKW) + (AKX * ALM)) * AN) - ((((((AJB * ALF) * AF) / AJA) - AKS) * ALI) + (ALJ * ALG))) * AKU) + (AKV * ALN)) + ((ALD * ALO) + (ALL * ALC))) * TR)) * ALQ)) / ALP);
            let ALT = TA - ALR;
            let ALU = TB - ALS;
            let ALV = UA * ALT;
            let ALW = ALU * UA;
            let ALX = ALR.exp();
            let ALY = WF * ALX;
            let ALZ = Lanes([0.0, (WG * ALX), 0.0, 0.0, 0.0]) + ((ALS * ALX) * WF);
            let AMA = ALW * ALV;
            let AMB = (ALV * ALV) + ALY;
            let AMC = (AMA + AMA) + ALZ;
            let AMD = if AMB < JV { 1.0 } else { 0.0 };
            let ANL;
            let ANM;
            let ANN;
            let ANO;
            let ANP;
            let ANQ;
            let ANR;
            let ANS;
            let ANT;
            let ANU;
            if AMD != 0.0 {
                let AME = (-AMB).sqrt();
                let AMF = (AMC * AF) * (P / (O * AME));
                let AMG = Q * AME;
                let AMH = AMF * Q;
                let AMI = AMG.sin();
                let AMJ = AMG.cos();
                let AMK = DA / AMI;
                let AML = (((AMH * AMJ) * AMK) * AF) / AMI;
                let AMM = AMK * AMK;
                let AMN = AML * AMK;
                let AMO = AMN + AMN;
                let AMP = AMJ * AMK;
                let AMQ = ((AMH * (AF * AMI)) * AMK) + (AML * AMJ);
                let AMS = (AMR * AMP) / AME;
                let AMT = ((AMQ * AMR) - (AMF * AMS)) / AME;
                let AMU = (AAM * AMM) + AMS;
                let AMV = (AMO * AAM) + AMT;
                ANL = AME;
                ANM = AMP;
                ANN = AMM;
                ANO = AMS;
                ANP = AMU;
                ANQ = AMF;
                ANR = AMQ;
                ANS = AMO;
                ANT = AMT;
                ANU = AMV;
            } else {
                let AMW = AMB.sqrt();
                let AMX = AMC * (P / (O * AMW));
                let AMY = Q * AMW;
                let AMZ = AMY.sinh();
                let ANA = DA / AMZ;
                let ANB = ANA * ANA;
                let ANC = (((((AMX * Q) * (AMY.cosh())) * ANA) * AF) / AMZ) * ANA;
                let AND = ANC + ANC;
                let ANE = (DA + ANB).sqrt();
                let ANF = AND * (P / (O * ANE));
                let ANG = (Q * ANE) / AMW;
                let ANH = ((ANF * Q) - (AMX * ANG)) / AMW;
                let ANJ = (ANI * ANB) + ANG;
                let ANK = (AND * ANI) + ANH;
                ANL = AMW;
                ANM = ANE;
                ANN = ANB;
                ANO = ANG;
                ANP = ANJ;
                ANQ = AMX;
                ANR = ANF;
                ANS = AND;
                ANT = ANH;
                ANU = ANK;
            }
            let ANV = ALV + (ANL * ANM);
            let ANW = ALW + ((ANQ * ANM) + (ANR * ANL));
            let ANX = DA / ANV;
            let ANY = ((ANW * ANX) * AF) / ANV;
            let ANZ = AMB * ANN;
            let AOA = ANZ * ANX;
            let AOB = AOA * ANX;
            let AOC = AOB.abs();
            let AOD = (ABS + ALT) - (AOC.ln());
            let AOE = (ABT + ALU) - ((((((((AMC * ANN) + (ANS * AMB)) * ANX) + (ANY * ANZ)) * ANX) + (ANY * AOA)) * ((O * (if AOB >= XA { 1.0 } else { 0.0 })) - P)) * (P / AOC));
            let AOF = (TR * AOD) + ALV;
            let AOG = DA / AMB;
            let AOH = AOG - ANO;
            let AOJ = (AOI * ALV) + ALY;
            let AOK = (ALW * AOI) + ALZ;
            let AOL = ANP * AOJ;
            let AOM = (ANU * AOJ) + (AOK * ANP);
            let AON = ACI + AOL;
            let AOO = (-1e0f64 + (AN * (AON * ANX))) - (AOH * AOJ);
            let AOP = AOL - UA;
            let AOQ = ((ALY - (UA * (ALV + ANV))) + (ALV * AOL)) + (TR * ((AOO * ANV) + (AOD * AOP)));
            let AOR = (-(ALY + (ANV * AOF))) / AOQ;
            let AOS = ALR + AOR;
            let AOT = ALS + ((((ALZ + ((ANW * AOF) + (((AOE * TR) + ALW) * ANV))) * AF) - ((((ALZ - ((ALW + ANW) * UA)) + ((ALW * AOL) + (AOM * ALV))) + ((((((((AOM * ANX) + (ANY * AON)) * AN) - ((((((AMC * AOG) * AF) / AMB) - ANT) * AOJ) + (AOK * AOH))) * ANV) + (ANW * AOO)) + ((AOE * AOP) + (AOM * AOD))) * TR)) * AOR)) / AOQ);
            let AOU = TA - AOS;
            let AOV = TB - AOT;
            let AOW = AOS.exp();
            let AOX = SU * AOW;
            let AOY = Lanes([0.0, (SV * AOW), 0.0, 0.0, 0.0]) + ((AOT * AOW) * SU);
            let AOZ = TK * AOU;
            let APA = (AOZ * AOU) - AOX;
            let APB = (((AOV * TK) * AOU) + (AOV * AOZ)) - AOY;
            let APC = if APA < JV { 1.0 } else { 0.0 };
            let AQB;
            let AQC;
            let AQD;
            let AQE;
            if APC != 0.0 {
                let APD = (-APA).sqrt();
                let APE = (APB * AF) * (P / (O * APD));
                let APF = Q * APD;
                let APG = APE * Q;
                let APH = APF.tan();
                let API = APF.cos();
                let APJ = APD / APH;
                let APK = (APE - ((APG * (P / (API * API))) * APJ)) / APH;
                let APL = APF.sin();
                let APM = APG * API;
                let APN = -APL;
                let APO = APN * APL;
                let APP = ((APM * AF) * APL) + (APM * APN);
                AQB = APJ;
                AQC = APO;
                AQD = APK;
                AQE = APP;
            } else {
                let APQ = APA.sqrt();
                let APR = APB * (P / (O * APQ));
                let APS = Q * APQ;
                let APT = APR * Q;
                let APU = APS.sinh();
                let APV = APU * APU;
                let APW = (APT * (APS.cosh())) * APU;
                let APX = APW + APW;
                let APY = APS.tanh();
                let APZ = APQ / APY;
                let AQA = (APR - ((APT * (P - (APY * APY))) * APZ)) / APY;
                AQB = APZ;
                AQC = APV;
                AQD = AQA;
                AQE = APX;
            }
            let AQF = AQC * AOX;
            let AQG = APA / AQF;
            let AQH = DA - AQG;
            let AQI = ((UA * AOU) - AQB) / AQH;
            let AQJ = (((AOV * UA) - AQD) - ((((APB - (((AQE * AOX) + (AOY * AQC)) * AQG)) / AQF) * AF) * AQI)) / AQH;
            let AQL = AOU * AQK;
            let AQM = AQL * SE;
            let AQN = ((AOV * AQK) * SE) + (SF * AQL);
            let AQP = AQI * AQO;
            let AQQ = AQP * SE;
            let AQR = ((AQJ * AQO) * SE) + (SF * AQP);
            let AQS = AQQ - AQM;
            let AQT = AQR - AQN;
            let AQV = AQU * SE;
            let AQW = AQS / AQV;
            let AQX = TF - AQW;
            let AQY = TG - ((AQT - ((SF * AQU) * AQW)) / AQV);
            let AQZ = AOS + AQX;
            let ARA = (AQZ * SE) / AN;
            let ARB = (((AOT + AQY) * SE) + (SF * AQZ)) / AN;
            let ARC = AQQ / AQK;
            let ARD = AQR / AQK;
            let ARF = (AQN * ARE) / AQK;
            let ARH = ((ARE * AQM) / AQK) + ARG;
            let ARI = ARF * ARH;
            let ARK = ((ARH * ARH) + ARJ).sqrt();
            let ARM = ARL * (Q * (ARH + ARK));
            let ARO = (AQT * ARN) / AQU;
            let ARP = ((ARN * AQS) / AQU) + ARG;
            let ARQ = ARO * ARP;
            let ARR = ((ARP * ARP) + ARJ).sqrt();
            let ART = ARS * (Q * (ARP + ARR));
            let ARV = ARC / ARU;
            let ARW = Q * (DA + (ARV.abs()));
            let ARX = ((ARD / ARU) * ((O * (if ARV >= XA { 1.0 } else { 0.0 })) - P)) * Q;
            let ARY = ARW.powf(EB);
            let ARZ = EB - P;
            let ASA = KM * DL;
            let ASB = DT + (KH * DL);
            let ASC = Lanes([0.0, DU, 0.0, 0.0]);
            let ASD = ARM.abs();
            let ASG = ASF + (ASE * KH);
            let ASH = ASD.powf(ASG);
            let ASI = (KM * ASE) * (ASH * (ASD.ln()));
            let ASJ = (ASC + (Lanes([ASA[0], 0.0, ASA[1], ASA[2]]) + Lanes([0.0, (DM * KH), 0.0, 0.0]))) * ASH;
            let ASK = DX / ARY;
            let ASL = (Lanes([ASJ[0], ASJ[1], ASJ[2], ASJ[3], 0.0]) + (((((((ARF + ((ARI + ARI) * (P / (O * ARK)))) * Q) * ARL) * ((O * (if ARM >= XA { 1.0 } else { 0.0 })) - P)) * (ASG * (ASD.powf((ASG - P))))) + Lanes([ASI[0], 0.0, ASI[1], ASI[2], 0.0])) * ASB)) + ((Lanes([0.0, DY, 0.0, 0.0, 0.0]) - (((ARX * (EB * (ARW.powf(ARZ)))) + Lanes([0.0, (EC * (ARY * (ARW.ln()))), 0.0, 0.0, 0.0])) * ASK)) / ARY);
            let ASM = DA + ((ASB * ASH) + ASK);
            let ASN = ASM - DA;
            let ASO = ASL * ASN;
            let ASQ = ((ASN * ASN) + ASP).sqrt();
            let ASS = (Q * ((ASM + DA) + ASQ)) / ASR;
            let AST = DC / ASS;
            let ASU = Lanes([0.0, DD, 0.0, 0.0, 0.0]);
            let ASW = ARW.powf(ASV);
            let ATA = ASZ + (KH * ASY);
            let ATB = ART.abs();
            let ATE = ATD + (ATC * KH);
            let ATF = ATB.powf(ATE);
            let ATG = (KM * ATC) * (ATF * (ATB.ln()));
            let ATH = (KM * ASY) * ATF;
            let ATJ = ATI / ASW;
            let ATK = (Lanes([ATH[0], 0.0, ATH[1], ATH[2], 0.0]) + (((((((ARO + ((ARQ + ARQ) * (P / (O * ARR)))) * Q) * ARS) * ((O * (if ART >= XA { 1.0 } else { 0.0 })) - P)) * (ATE * (ATB.powf((ATE - P))))) + Lanes([ATG[0], 0.0, ATG[1], ATG[2], 0.0])) * ATA)) + ((((ARX * (ASV * (ARW.powf(ASX)))) * ATJ) * AF) / ASW);
            let ATL = DA + ((ATA * ATF) + ATJ);
            let ATM = ATL - DA;
            let ATN = ATK * ATM;
            let ATO = ((ATM * ATM) + ASP).sqrt();
            let ATP = (Q * ((ATL + DA) + ATO)) / ASR;
            let ATR = ATQ / ATP;
            let ATS = (SP - (AQM / AQK)) / SE;
            let ATT = ATS.exp();
            let ATU = (((SN - (AQN / AQK)) - (SF * ATS)) / SE) * ATT;
            let ATV = (TC - (AQS / AQU)) / SE;
            let ATW = ATV.exp();
            let ATX = (((TD - (AQT / AQU)) - (SF * ATV)) / SE) * ATW;
            let ATY = ATT + ATW;
            let ATZ = ATU + ATX;
            let AUA = ATT / ATY;
            let AUB = ATW / ATY;
            let AUC = (AUA * AST) + (AUB * ATR);
            let AUD = ((((ATU - (ATZ * AUA)) / ATY) * AST) + (((ASU - ((((ASL + ((ASO + ASO) * (P / (O * ASQ)))) * Q) / ASR) * AST)) / ASS) * AUA)) + ((((ATX - (ATZ * AUB)) / ATY) * ATR) + (((((((ATK + ((ATN + ATN) * (P / (O * ATO)))) * Q) / ASR) * ATR) * AF) / ATP) * AUB));
            let AUG;
            let AUH;
            if A != 0.0 {
                AUG = JV;
                AUH = AUE;
            } else {
                let AVP;
                let AVQ;
                if AUF != 0.0 {
                    let AUV = DA + (AUU * ARC);
                    let AUW = DA / AUV;
                    let AUX = (((ARD * AUU) * AUW) * AF) / AUV;
                    let AUY = AUX * AUW;
                    let AUZ = ((AUW * AUW) + PX).sqrt();
                    let AVE = ((AVB + (AVA * (Q * (AUW + AUZ)))) * AVC) * AVD;
                    let AVF = AVE * EI;
                    let AVG = ((((((AUX + ((AUY + AUY) * (P / (O * AUZ)))) * Q) * AVA) * AVC) * AVD) * EI) + Lanes([0.0, (EJ * AVE), 0.0, 0.0, 0.0]);
                    AVP = AVF;
                    AVQ = AVG;
                } else {
                    let AVH = DA + (AUU * ARC);
                    let AVI = DA / AVH;
                    let AVJ = (((ARD * AUU) * AVI) * AF) / AVH;
                    let AVK = AVJ * AVI;
                    let AVL = ((AVI * AVI) + PX).sqrt();
                    let AVM = ((staged[143] + (AVA * (Q * (AVI + AVL)))) * AVC) * AVD;
                    let AVN = AVM * EI;
                    let AVO = ((((((AVJ + ((AVK + AVK) * (P / (O * AVL)))) * Q) * AVA) * AVC) * AVD) * EI) + Lanes([0.0, (EJ * AVM), 0.0, 0.0, 0.0]);
                    AVP = AVN;
                    AVQ = AVO;
                }
                AUG = AVP;
                AUH = AVQ;
            }
            let AUI = AN * FB;
            let AUJ = AUI / AUC;
            let AUL = AUJ * AUK;
            let AUM = ((Lanes([0.0, (FC * AN), 0.0, 0.0, 0.0]) - (AUD * AUJ)) / AUC) * AUK;
            let AUO = QN * AUN;
            let AUR = AUQ * ((ARC + (AUN * QM)) + (AO * AUP));
            let AUS = ((ARD + Lanes([AUO[0], 0.0, AUO[1], AUO[2], 0.0])) + Lanes([0.0, (AP * AUP), 0.0, 0.0, 0.0])) * AUQ;
            let AUT = if AUG == JV { 1.0 } else { 0.0 };
            let AWM;
            let AWN;
            if AUT != 0.0 {
                let AVR = AUL + AUR;
                let AVS = (AUL * AUR) / AVR;
                let AVT = (((AUM * AUR) + (AUS * AUL)) - ((AUM + AUS) * AVS)) / AVR;
                AWM = AVS;
                AWN = AVT;
            } else {
                let AVV = (AVU * FB) * AQK;
                let AVW = AVV * AUG;
                let AVX = Lanes([0.0, (((FC * AVU) * AQK) * AUG), 0.0, 0.0, 0.0]) + (AUH * AVV);
                let AVY = AN * AVW;
                let AVZ = AVX * AN;
                let AWB = AWA * AUR;
                let AWC = (AUR + AUL) + (AWB * AVW);
                let AWD = (AUS + AUM) + (((AUS * AWA) * AVW) + (AVX * AWB));
                let AWE = AN * AUR;
                let AWF = AUL + (AWE * AVW);
                let AWG = AUR * AWF;
                let AWH = AWD * AWC;
                let AWI = AN * AVY;
                let AWJ = ((AWC * AWC) - (AWI * AWG)).sqrt();
                let AWK = (AWC - AWJ) / AVY;
                let AWL = ((AWD - (((AWH + AWH) - (((AVZ * AN) * AWG) + (((AUS * AWF) + ((AUM + (((AUS * AN) * AVW) + (AVX * AWE))) * AUR)) * AWI))) * (P / (O * AWJ)))) - (AVZ * AWK)) / AVY;
                AWM = AWK;
                AWN = AWL;
            }
            let AWO = AWM - ARJ;
            let AWP = AWN * AWO;
            let AWQ = ((AWO * AWO) + 4.0000000000000007e-10f64).sqrt();
            let AWR = (AWN + ((AWP + AWP) * (P / (O * AWQ)))) * Q;
            let AWS = (Q * (AWO + AWQ)) + ARJ;
            let AWT = KG / AWS;
            let AWU = Lanes([0.0, 0.0, KL[0], KL[1], 0.0]);
            let AWV = AWT.powf(GQ);
            let AWW = DA + AWV;
            let AWY = AWW.powf(AWX);
            let AWZ = KG / AWY;
            let AXA = (AWU - ((((((AWU - (AWR * AWT)) / AWS) * (GQ * (AWT.powf((GQ - P))))) + Lanes([0.0, (GP * (AWV * (AWT.ln()))), 0.0, 0.0, 0.0])) * (AWX * (AWW.powf(staged[265])))) * AWZ)) / AWY;
            let AXB = if AWZ > KG { 1.0 } else { 0.0 };
            let AXC;
            let AXD;
            if AXB != 0.0 {
                AXC = KG;
                AXD = AWU;
            } else {
                AXC = AWZ;
                AXD = AXA;
            }
            let AXE = (SP - AXC) / SE;
            let AXF = ((SN - AXD) - (SF * AXE)) / SE;
            let AXG = (TE - AXC) / SE;
            let AXH = ((TD - AXD) - (SF * AXG)) / SE;
            let AXI = AXE - SY;
            let AXJ = AXF - TI;
            let AXK = TK * AXI;
            let AXL = (AXK * AXI) + TM;
            let AXM = (AXL.ln()) - SW;
            let AXN = ((((AXJ * TK) * AXI) + (AXJ * AXK)) * (P / AXL)) - TP;
            let AXO = ((VV - AQX) / UA) - SY;
            let AXP = ((VW - AQY) / UA) - TI;
            let AXQ = TK * AXO;
            let AXR = (AXQ * AXO) + TM;
            let AXS = ((AXM - (((AXR.ln()) - SW) - SY)) + (TR * AXG)) / TS;
            let AXT = ((AXN - ((((((AXP * TK) * AXO) + (AXP * AXQ)) * (P / AXR)) - TP) - TI)) + (AXH * TR)) / TS;
            let AXU = AXG + (TV * (AXE - AXG));
            let AXV = if AXU <= AXM { AXU } else { AXM };
            let AXW = if AXV <= SY { AXV } else { SY };
            let AXX = TI + (((AXN + (((AXH + ((AXF - AXH) * TV)) - AXN) * (if AXU <= AXM { 1.0 } else { 0.0 }))) - TI) * (if AXV <= SY { 1.0 } else { 0.0 }));
            let AXY = (AXW + (UA * AXE)) / UB;
            let AXZ = (AXX + (AXF * UA)) / UB;
            let AYA = AXG - AXS;
            let AYB = AXH - AXT;
            let AYC = UG * AYA;
            let AYD = AXS.exp();
            let AYE = (AYC * AYA) - (SU * AYD);
            let AYF = (((AYB * UG) * AYA) + (AYB * AYC)) - (Lanes([0.0, (SV * AYD), 0.0, 0.0, 0.0]) + ((AXT * AYD) * SU));
            let AYG = if AYE < JV { 1.0 } else { 0.0 };
            let AZE;
            let AZF;
            if AYG != 0.0 {
                let AYH = (AXG - AXW) * TR;
                let AYI = (AXH - AXX) * TR;
                let AYJ = MB * UA;
                let AYK = AYJ + AYH;
                let AYL = AYJ * AYH;
                let AYM = AYI * AYJ;
                let AYN = AYI * US;
                let AYO = (US * AYK) + DA;
                let AYP = (AYI * UV) + AYM;
                let AYQ = ((AYK * UV) + AYL) + TM;
                let AYR = (UY * AYK) + (TM * AYL);
                let AYT = AYS * AYO;
                let AYU = AYP * AYQ;
                let AYV = ((AYT * AYR) + (AYQ * AYQ)).sqrt();
                let AYW = AN * AYO;
                let AYX = ((-AYQ) + AYV) / AYW;
                let AYZ = ((-((AXE - ((VV - AXW) / UA)) + AN)) / AYY).exp();
                let AZA = DA - AYZ;
                let AZB = AYX * AZA;
                let AZC = if AZB <= VK { AZB } else { VK };
                let AZD = ((((((AYP * AF) + (((((AYN * AYS) * AYR) + (((AYI * UY) + (AYM * TM)) * AYT)) + (AYU + AYU)) * (P / (O * AYV)))) - ((AYN * AN) * AYX)) / AYW) * AZA) + ((((((AXF - ((VW - AXX) / UA)) * AF) / AYY) * AYZ) * AF) * AYX)) * (if AZB <= VK { 1.0 } else { 0.0 });
                AZE = AZC;
                AZF = AZD;
            } else {
                AZE = AYE;
                AZF = AYF;
            }
            let AZG = if AXE >= SY { AXE } else { SY };
            let AZH = TI + (AXJ * (if AXE >= SY { 1.0 } else { 0.0 }));
            let AZI = AZG - SY;
            let AZJ = AZH - TI;
            let AZK = TK * AZI;
            let AZL = (AZK * AZI) + TM;
            let AZM = ((VV - AXW) / UA) - SY;
            let AZN = ((VW - AXX) / UA) - TI;
            let AZO = TK * AZM;
            let AZP = (AZO * AZM) + TM;
            let AZQ = ((AZL.ln()) - SW) - (((AZP.ln()) - SW) - SY);
            let AZR = (((((AZJ * TK) * AZI) + (AZJ * AZK)) * (P / AZL)) - TP) - ((((((AZN * TK) * AZM) + (AZN * AZO)) * (P / AZP)) - TP) - TI);
            let AZS = AZG - AZQ;
            let AZT = AZH - AZR;
            let AZU = AZQ.exp();
            let AZV = WF * AZU;
            let AZW = Lanes([0.0, (WG * AZU), 0.0, 0.0, 0.0]) + ((AZR * AZU) * WF);
            let AZX = TK * AZS;
            let AZY = AZT * TK;
            let BAA = (AZZ * AZX) + AZV;
            let BAB = (-(((AZX * AZS) + AZV) - AZE)) / BAA;
            let BAC = AZQ + BAB;
            let BAD = AZR + (((((((AZY * AZS) + (AZT * AZX)) + AZW) - AZF) * AF) - (((AZY * AZZ) + AZW) * BAB)) / BAA);
            let BAE = AZG - BAC;
            let BAF = AZH - BAD;
            let BAG = TK * BAE;
            let BAH = BAF * TK;
            let BAI = (BAG * BAE) - AZE;
            let BAJ = ((BAH * BAE) + (BAF * BAG)) - AZF;
            let BAK = DA / BAI;
            let BAL = ((BAJ * BAK) * AF) / BAI;
            let BAM = BAI.abs();
            let BAN = ((BAM.ln()) - SW) - BAC;
            let BAP = BAO * BAG;
            let BAQ = (BAP * BAK) - DA;
            let BAR = DA / BAQ;
            let BAS = (((((BAH * BAO) * BAK) + (BAL * BAP)) * BAR) * AF) / BAQ;
            let BAU = BAT * BAG;
            let BAV = BAU * BAG;
            let BAW = BAV * BAK;
            let BAX = (BAW * BAK) + (XL * BAK);
            let BAY = BAN * BAR;
            let BAZ = (((((BAJ * ((O * (if BAI >= XA { 1.0 } else { 0.0 })) - P)) * (P / BAM)) - TP) - BAD) * BAR) + (BAS * BAN);
            let BBA = Q * BAY;
            let BBB = BBA * BAY;
            let BBC = BBB * BAX;
            let BBD = (-BAY) - (BBC * BAR);
            let BBF = if BBD >= BBE { BBD } else { BBE };
            let BBG = BAC + (if BBF <= XV { BBF } else { XV });
            let BBH = BAD + ((((BAZ * AF) - (((((((BAZ * Q) * BAY) + (BAZ * BBA)) * BAX) + (((((((((BAH * BAT) * BAG) + (BAH * BAU)) * BAK) + (BAL * BAV)) * BAK) + (BAL * BAW)) + (BAL * XL)) * BBB)) * BAR) + (BAS * BBC))) * (if BBD >= BBE { 1.0 } else { 0.0 })) * (if BBF <= XV { 1.0 } else { 0.0 }));
            let BBI = AZG - BBG;
            let BBJ = AZH - BBH;
            let BBK = TK * BBI;
            let BBL = BBJ * TK;
            let BBM = (BBK * BBI) - AZE;
            let BBN = ((BBL * BBI) + (BBJ * BBK)) - AZF;
            let BBO = DA / BBM;
            let BBP = ((BBN * BBO) * AF) / BBM;
            let BBQ = BBM.abs();
            let BBR = ((BBQ.ln()) - SW) - BBG;
            let BBT = BBS * BBK;
            let BBU = (BBT * BBO) - DA;
            let BBV = DA / BBU;
            let BBW = (((((BBL * BBS) * BBO) + (BBP * BBT)) * BBV) * AF) / BBU;
            let BBY = BBX * BBK;
            let BBZ = BBY * BBK;
            let BCA = BBZ * BBO;
            let BCB = (BCA * BBO) + (XL * BBO);
            let BCC = BBR * BBV;
            let BCD = (((((BBN * ((O * (if BBM >= XA { 1.0 } else { 0.0 })) - P)) * (P / BBQ)) - TP) - BBH) * BBV) + (BBW * BBR);
            let BCE = Q * BCC;
            let BCF = BCE * BCC;
            let BCG = BCF * BCB;
            let BCH = (-BCC) - (BCG * BBV);
            let BCJ = if BCH >= BCI { BCH } else { BCI };
            let BCK = BBG + (if BCJ <= XV { BCJ } else { XV });
            let BCL = if BCK >= ZC { BCK } else { ZC };
            let BCM = TI + (((BBH + ((((BCD * AF) - (((((((BCD * Q) * BCC) + (BCD * BCE)) * BCB) + (((((((((BBL * BBX) * BBK) + (BBL * BBY)) * BBO) + (BBP * BBZ)) * BBO) + (BBP * BCA)) + (BBP * XL)) * BCF)) * BBV) + (BBW * BCG))) * (if BCH >= BCI { 1.0 } else { 0.0 })) * (if BCJ <= XV { 1.0 } else { 0.0 }))) - TI) * (if BCK >= ZC { 1.0 } else { 0.0 }));
            let BCN = (AXY - (ZF * BCL)).exp();
            let BCO = DA + BCN;
            let BCP = AXY - (BCO.ln());
            let BCQ = if BCP <= BCL { BCP } else { BCL };
            let BCR = BCM + (((AXZ - (((AXZ - (BCM * ZF)) * BCN) * (P / BCO))) - BCM) * (if BCP <= BCL { 1.0 } else { 0.0 }));
            let BCS = AXE - BCQ;
            let BCT = AXF - BCR;
            let BCU = UA * BCS;
            let BCV = BCT * UA;
            let BCW = BCQ.exp();
            let BCX = WF * BCW;
            let BCY = Lanes([0.0, (WG * BCW), 0.0, 0.0, 0.0]) + ((BCR * BCW) * WF);
            let BCZ = BCV * BCU;
            let BDA = (BCU * BCU) + BCX;
            let BDB = (BCZ + BCZ) + BCY;
            let BDC = if BDA < JV { 1.0 } else { 0.0 };
            let BEK;
            let BEL;
            let BEM;
            let BEN;
            let BEO;
            let BEP;
            let BEQ;
            let BER;
            let BES;
            let BET;
            if BDC != 0.0 {
                let BDD = (-BDA).sqrt();
                let BDE = (BDB * AF) * (P / (O * BDD));
                let BDF = Q * BDD;
                let BDG = BDE * Q;
                let BDH = BDF.sin();
                let BDI = BDF.cos();
                let BDJ = DA / BDH;
                let BDK = (((BDG * BDI) * BDJ) * AF) / BDH;
                let BDL = BDJ * BDJ;
                let BDM = BDK * BDJ;
                let BDN = BDM + BDM;
                let BDO = BDI * BDJ;
                let BDP = ((BDG * (AF * BDH)) * BDJ) + (BDK * BDI);
                let BDR = (BDQ * BDO) / BDD;
                let BDS = ((BDP * BDQ) - (BDE * BDR)) / BDD;
                let BDT = (AAM * BDL) + BDR;
                let BDU = (BDN * AAM) + BDS;
                BEK = BDD;
                BEL = BDO;
                BEM = BDL;
                BEN = BDR;
                BEO = BDT;
                BEP = BDE;
                BEQ = BDP;
                BER = BDN;
                BES = BDS;
                BET = BDU;
            } else {
                let BDV = BDA.sqrt();
                let BDW = BDB * (P / (O * BDV));
                let BDX = Q * BDV;
                let BDY = BDX.sinh();
                let BDZ = DA / BDY;
                let BEA = BDZ * BDZ;
                let BEB = (((((BDW * Q) * (BDX.cosh())) * BDZ) * AF) / BDY) * BDZ;
                let BEC = BEB + BEB;
                let BED = (DA + BEA).sqrt();
                let BEE = BEC * (P / (O * BED));
                let BEF = (Q * BED) / BDV;
                let BEG = ((BEE * Q) - (BDW * BEF)) / BDV;
                let BEI = (BEH * BEA) + BEF;
                let BEJ = (BEC * BEH) + BEG;
                BEK = BDV;
                BEL = BED;
                BEM = BEA;
                BEN = BEF;
                BEO = BEI;
                BEP = BDW;
                BEQ = BEE;
                BER = BEC;
                BES = BEG;
                BET = BEJ;
            }
            let BEU = BCU + (BEK * BEL);
            let BEV = BCV + ((BEP * BEL) + (BEQ * BEK));
            let BEW = DA / BEU;
            let BEX = ((BEV * BEW) * AF) / BEU;
            let BEY = AXG - AXE;
            let BEZ = AXH - AXF;
            let BFA = BDA * BEM;
            let BFB = BFA * BEW;
            let BFC = BFB * BEW;
            let BFD = BFC.abs();
            let BFE = (BEY + BCS) - (BFD.ln());
            let BFF = (BEZ + BCT) - ((((((((BDB * BEM) + (BER * BDA)) * BEW) + (BEX * BFA)) * BEW) + (BEX * BFB)) * ((O * (if BFC >= XA { 1.0 } else { 0.0 })) - P)) * (P / BFD));
            let BFG = (TR * BFE) + BCU;
            let BFH = DA / BDA;
            let BFI = BFH - BEN;
            let BFK = (BFJ * BCU) + BCX;
            let BFL = (BCV * BFJ) + BCY;
            let BFM = BEO * BFK;
            let BFN = (BET * BFK) + (BFL * BEO);
            let BFO = ACI + BFM;
            let BFP = (-1e0f64 + (AN * (BFO * BEW))) - (BFI * BFK);
            let BFQ = BFM - UA;
            let BFR = ((BCX - (UA * (BCU + BEU))) + (BCU * BFM)) + (TR * ((BFP * BEU) + (BFE * BFQ)));
            let BFS = (-(BCX + (BEU * BFG))) / BFR;
            let BFT = BCQ + BFS;
            let BFU = BCR + ((((BCY + ((BEV * BFG) + (((BFF * TR) + BCV) * BEU))) * AF) - ((((BCY - ((BCV + BEV) * UA)) + ((BCV * BFM) + (BFN * BCU))) + ((((((((BFN * BEW) + (BEX * BFO)) * AN) - ((((((BDB * BFH) * AF) / BDA) - BES) * BFK) + (BFL * BFI))) * BEU) + (BEV * BFP)) + ((BFF * BFQ) + (BFN * BFE))) * TR)) * BFS)) / BFR);
            let BFV = AXE - BFT;
            let BFW = AXF - BFU;
            let BFX = UA * BFV;
            let BFY = BFW * UA;
            let BFZ = BFT.exp();
            let BGA = WF * BFZ;
            let BGB = Lanes([0.0, (WG * BFZ), 0.0, 0.0, 0.0]) + ((BFU * BFZ) * WF);
            let BGC = BFY * BFX;
            let BGD = (BFX * BFX) + BGA;
            let BGE = (BGC + BGC) + BGB;
            let BGF = if BGD < JV { 1.0 } else { 0.0 };
            let BHN;
            let BHO;
            let BHP;
            let BHQ;
            let BHR;
            let BHS;
            let BHT;
            let BHU;
            let BHV;
            let BHW;
            if BGF != 0.0 {
                let BGG = (-BGD).sqrt();
                let BGH = (BGE * AF) * (P / (O * BGG));
                let BGI = Q * BGG;
                let BGJ = BGH * Q;
                let BGK = BGI.sin();
                let BGL = BGI.cos();
                let BGM = DA / BGK;
                let BGN = (((BGJ * BGL) * BGM) * AF) / BGK;
                let BGO = BGM * BGM;
                let BGP = BGN * BGM;
                let BGQ = BGP + BGP;
                let BGR = BGL * BGM;
                let BGS = ((BGJ * (AF * BGK)) * BGM) + (BGN * BGL);
                let BGU = (BGT * BGR) / BGG;
                let BGV = ((BGS * BGT) - (BGH * BGU)) / BGG;
                let BGW = (AAM * BGO) + BGU;
                let BGX = (BGQ * AAM) + BGV;
                BHN = BGG;
                BHO = BGR;
                BHP = BGO;
                BHQ = BGU;
                BHR = BGW;
                BHS = BGH;
                BHT = BGS;
                BHU = BGQ;
                BHV = BGV;
                BHW = BGX;
            } else {
                let BGY = BGD.sqrt();
                let BGZ = BGE * (P / (O * BGY));
                let BHA = Q * BGY;
                let BHB = BHA.sinh();
                let BHC = DA / BHB;
                let BHD = BHC * BHC;
                let BHE = (((((BGZ * Q) * (BHA.cosh())) * BHC) * AF) / BHB) * BHC;
                let BHF = BHE + BHE;
                let BHG = (DA + BHD).sqrt();
                let BHH = BHF * (P / (O * BHG));
                let BHI = (Q * BHG) / BGY;
                let BHJ = ((BHH * Q) - (BGZ * BHI)) / BGY;
                let BHL = (BHK * BHD) + BHI;
                let BHM = (BHF * BHK) + BHJ;
                BHN = BGY;
                BHO = BHG;
                BHP = BHD;
                BHQ = BHI;
                BHR = BHL;
                BHS = BGZ;
                BHT = BHH;
                BHU = BHF;
                BHV = BHJ;
                BHW = BHM;
            }
            let BHX = BFX + (BHN * BHO);
            let BHY = BFY + ((BHS * BHO) + (BHT * BHN));
            let BHZ = DA / BHX;
            let BIA = ((BHY * BHZ) * AF) / BHX;
            let BIB = BGD * BHP;
            let BIC = BIB * BHZ;
            let BID = BIC * BHZ;
            let BIE = BID.abs();
            let BIF = (BEY + BFV) - (BIE.ln());
            let BIG = (BEZ + BFW) - ((((((((BGE * BHP) + (BHU * BGD)) * BHZ) + (BIA * BIB)) * BHZ) + (BIA * BIC)) * ((O * (if BID >= XA { 1.0 } else { 0.0 })) - P)) * (P / BIE));
            let BIH = (TR * BIF) + BFX;
            let BII = DA / BGD;
            let BIJ = BII - BHQ;
            let BIL = (BIK * BFX) + BGA;
            let BIM = (BFY * BIK) + BGB;
            let BIN = BHR * BIL;
            let BIO = (BHW * BIL) + (BIM * BHR);
            let BIP = ACI + BIN;
            let BIQ = (-1e0f64 + (AN * (BIP * BHZ))) - (BIJ * BIL);
            let BIR = BIN - UA;
            let BIS = ((BGA - (UA * (BFX + BHX))) + (BFX * BIN)) + (TR * ((BIQ * BHX) + (BIF * BIR)));
            let BIT = (-(BGA + (BHX * BIH))) / BIS;
            let BIU = BFT + BIT;
            let BIV = BFU + ((((BGB + ((BHY * BIH) + (((BIG * TR) + BFY) * BHX))) * AF) - ((((BGB - ((BFY + BHY) * UA)) + ((BFY * BIN) + (BIO * BFX))) + ((((((((BIO * BHZ) + (BIA * BIP)) * AN) - ((((((BGE * BII) * AF) / BGD) - BHV) * BIL) + (BIM * BIJ))) * BHX) + (BHY * BIQ)) + ((BIG * BIR) + (BIO * BIF))) * TR)) * BIT)) / BIS);
            let BIW = AXE - BIU;
            let BIX = AXF - BIV;
            let BIY = UA * BIW;
            let BIZ = BIX * UA;
            let BJA = BIU.exp();
            let BJB = WF * BJA;
            let BJC = Lanes([0.0, (WG * BJA), 0.0, 0.0, 0.0]) + ((BIV * BJA) * WF);
            let BJD = BIZ * BIY;
            let BJE = (BIY * BIY) + BJB;
            let BJF = (BJD + BJD) + BJC;
            let BJG = if BJE < JV { 1.0 } else { 0.0 };
            let BKO;
            let BKP;
            let BKQ;
            let BKR;
            let BKS;
            let BKT;
            let BKU;
            let BKV;
            let BKW;
            let BKX;
            if BJG != 0.0 {
                let BJH = (-BJE).sqrt();
                let BJI = (BJF * AF) * (P / (O * BJH));
                let BJJ = Q * BJH;
                let BJK = BJI * Q;
                let BJL = BJJ.sin();
                let BJM = BJJ.cos();
                let BJN = DA / BJL;
                let BJO = (((BJK * BJM) * BJN) * AF) / BJL;
                let BJP = BJN * BJN;
                let BJQ = BJO * BJN;
                let BJR = BJQ + BJQ;
                let BJS = BJM * BJN;
                let BJT = ((BJK * (AF * BJL)) * BJN) + (BJO * BJM);
                let BJV = (BJU * BJS) / BJH;
                let BJW = ((BJT * BJU) - (BJI * BJV)) / BJH;
                let BJX = (AAM * BJP) + BJV;
                let BJY = (BJR * AAM) + BJW;
                BKO = BJH;
                BKP = BJS;
                BKQ = BJP;
                BKR = BJV;
                BKS = BJX;
                BKT = BJI;
                BKU = BJT;
                BKV = BJR;
                BKW = BJW;
                BKX = BJY;
            } else {
                let BJZ = BJE.sqrt();
                let BKA = BJF * (P / (O * BJZ));
                let BKB = Q * BJZ;
                let BKC = BKB.sinh();
                let BKD = DA / BKC;
                let BKE = BKD * BKD;
                let BKF = (((((BKA * Q) * (BKB.cosh())) * BKD) * AF) / BKC) * BKD;
                let BKG = BKF + BKF;
                let BKH = (DA + BKE).sqrt();
                let BKI = BKG * (P / (O * BKH));
                let BKJ = (Q * BKH) / BJZ;
                let BKK = ((BKI * Q) - (BKA * BKJ)) / BJZ;
                let BKM = (BKL * BKE) + BKJ;
                let BKN = (BKG * BKL) + BKK;
                BKO = BJZ;
                BKP = BKH;
                BKQ = BKE;
                BKR = BKJ;
                BKS = BKM;
                BKT = BKA;
                BKU = BKI;
                BKV = BKG;
                BKW = BKK;
                BKX = BKN;
            }
            let BKY = BIY + (BKO * BKP);
            let BKZ = BIZ + ((BKT * BKP) + (BKU * BKO));
            let BLA = DA / BKY;
            let BLB = ((BKZ * BLA) * AF) / BKY;
            let BLC = BJE * BKQ;
            let BLD = BLC * BLA;
            let BLE = BLD * BLA;
            let BLF = BLE.abs();
            let BLG = (BEY + BIW) - (BLF.ln());
            let BLH = (BEZ + BIX) - ((((((((BJF * BKQ) + (BKV * BJE)) * BLA) + (BLB * BLC)) * BLA) + (BLB * BLD)) * ((O * (if BLE >= XA { 1.0 } else { 0.0 })) - P)) * (P / BLF));
            let BLI = (TR * BLG) + BIY;
            let BLJ = DA / BJE;
            let BLK = BLJ - BKR;
            let BLM = (BLL * BIY) + BJB;
            let BLN = (BIZ * BLL) + BJC;
            let BLO = BKS * BLM;
            let BLP = (BKX * BLM) + (BLN * BKS);
            let BLQ = ACI + BLO;
            let BLR = (-1e0f64 + (AN * (BLQ * BLA))) - (BLK * BLM);
            let BLS = BLO - UA;
            let BLT = ((BJB - (UA * (BIY + BKY))) + (BIY * BLO)) + (TR * ((BLR * BKY) + (BLG * BLS)));
            let BLU = (-(BJB + (BKY * BLI))) / BLT;
            let BLV = BIU + BLU;
            let BLW = BIV + ((((BJC + ((BKZ * BLI) + (((BLH * TR) + BIZ) * BKY))) * AF) - ((((BJC - ((BIZ + BKZ) * UA)) + ((BIZ * BLO) + (BLP * BIY))) + ((((((((BLP * BLA) + (BLB * BLQ)) * AN) - ((((((BJF * BLJ) * AF) / BJE) - BKW) * BLM) + (BLN * BLK))) * BKY) + (BKZ * BLR)) + ((BLH * BLS) + (BLP * BLG))) * TR)) * BLU)) / BLT);
            let BLX = AXE - BLV;
            let BLY = AXF - BLW;
            let BLZ = UA * BLX;
            let BMA = BLY * UA;
            let BMB = BLV.exp();
            let BMC = WF * BMB;
            let BMD = Lanes([0.0, (WG * BMB), 0.0, 0.0, 0.0]) + ((BLW * BMB) * WF);
            let BME = BMA * BLZ;
            let BMF = (BLZ * BLZ) + BMC;
            let BMG = (BME + BME) + BMD;
            let BMH = if BMF < JV { 1.0 } else { 0.0 };
            let BNP;
            let BNQ;
            let BNR;
            let BNS;
            let BNT;
            let BNU;
            let BNV;
            let BNW;
            let BNX;
            let BNY;
            if BMH != 0.0 {
                let BMI = (-BMF).sqrt();
                let BMJ = (BMG * AF) * (P / (O * BMI));
                let BMK = Q * BMI;
                let BML = BMJ * Q;
                let BMM = BMK.sin();
                let BMN = BMK.cos();
                let BMO = DA / BMM;
                let BMP = (((BML * BMN) * BMO) * AF) / BMM;
                let BMQ = BMO * BMO;
                let BMR = BMP * BMO;
                let BMS = BMR + BMR;
                let BMT = BMN * BMO;
                let BMU = ((BML * (AF * BMM)) * BMO) + (BMP * BMN);
                let BMW = (BMV * BMT) / BMI;
                let BMX = ((BMU * BMV) - (BMJ * BMW)) / BMI;
                let BMY = (AAM * BMQ) + BMW;
                let BMZ = (BMS * AAM) + BMX;
                BNP = BMI;
                BNQ = BMT;
                BNR = BMQ;
                BNS = BMW;
                BNT = BMY;
                BNU = BMJ;
                BNV = BMU;
                BNW = BMS;
                BNX = BMX;
                BNY = BMZ;
            } else {
                let BNA = BMF.sqrt();
                let BNB = BMG * (P / (O * BNA));
                let BNC = Q * BNA;
                let BND = BNC.sinh();
                let BNE = DA / BND;
                let BNF = BNE * BNE;
                let BNG = (((((BNB * Q) * (BNC.cosh())) * BNE) * AF) / BND) * BNE;
                let BNH = BNG + BNG;
                let BNI = (DA + BNF).sqrt();
                let BNJ = BNH * (P / (O * BNI));
                let BNK = (Q * BNI) / BNA;
                let BNL = ((BNJ * Q) - (BNB * BNK)) / BNA;
                let BNN = (BNM * BNF) + BNK;
                let BNO = (BNH * BNM) + BNL;
                BNP = BNA;
                BNQ = BNI;
                BNR = BNF;
                BNS = BNK;
                BNT = BNN;
                BNU = BNB;
                BNV = BNJ;
                BNW = BNH;
                BNX = BNL;
                BNY = BNO;
            }
            let BNZ = BLZ + (BNP * BNQ);
            let BOA = BMA + ((BNU * BNQ) + (BNV * BNP));
            let BOB = DA / BNZ;
            let BOC = ((BOA * BOB) * AF) / BNZ;
            let BOD = BMF * BNR;
            let BOE = BOD * BOB;
            let BOF = BOE * BOB;
            let BOG = BOF.abs();
            let BOH = (BEY + BLX) - (BOG.ln());
            let BOI = (BEZ + BLY) - ((((((((BMG * BNR) + (BNW * BMF)) * BOB) + (BOC * BOD)) * BOB) + (BOC * BOE)) * ((O * (if BOF >= XA { 1.0 } else { 0.0 })) - P)) * (P / BOG));
            let BOJ = (TR * BOH) + BLZ;
            let BOK = DA / BMF;
            let BOL = BOK - BNS;
            let BON = (BOM * BLZ) + BMC;
            let BOO = (BMA * BOM) + BMD;
            let BOP = BNT * BON;
            let BOQ = (BNY * BON) + (BOO * BNT);
            let BOR = ACI + BOP;
            let BOS = (-1e0f64 + (AN * (BOR * BOB))) - (BOL * BON);
            let BOT = BOP - UA;
            let BOU = ((BMC - (UA * (BLZ + BNZ))) + (BLZ * BOP)) + (TR * ((BOS * BNZ) + (BOH * BOT)));
            let BOV = (-(BMC + (BNZ * BOJ))) / BOU;
            let BOW = BLV + BOV;
            let BOX = BLW + ((((BMD + ((BOA * BOJ) + (((BOI * TR) + BMA) * BNZ))) * AF) - ((((BMD - ((BMA + BOA) * UA)) + ((BMA * BOP) + (BOQ * BLZ))) + ((((((((BOQ * BOB) + (BOC * BOR)) * AN) - ((((((BMG * BOK) * AF) / BMF) - BNX) * BON) + (BOO * BOL))) * BNZ) + (BOA * BOS)) + ((BOI * BOT) + (BOQ * BOH))) * TR)) * BOV)) / BOU);
            let BOY = AXE - BOW;
            let BOZ = AXF - BOX;
            let BPA = UA * BOY;
            let BPB = BOZ * UA;
            let BPC = BOW.exp();
            let BPD = WF * BPC;
            let BPE = Lanes([0.0, (WG * BPC), 0.0, 0.0, 0.0]) + ((BOX * BPC) * WF);
            let BPF = BPB * BPA;
            let BPG = (BPA * BPA) + BPD;
            let BPH = (BPF + BPF) + BPE;
            let BPI = if BPG < JV { 1.0 } else { 0.0 };
            let BQQ;
            let BQR;
            let BQS;
            let BQT;
            let BQU;
            let BQV;
            let BQW;
            let BQX;
            let BQY;
            let BQZ;
            if BPI != 0.0 {
                let BPJ = (-BPG).sqrt();
                let BPK = (BPH * AF) * (P / (O * BPJ));
                let BPL = Q * BPJ;
                let BPM = BPK * Q;
                let BPN = BPL.sin();
                let BPO = BPL.cos();
                let BPP = DA / BPN;
                let BPQ = (((BPM * BPO) * BPP) * AF) / BPN;
                let BPR = BPP * BPP;
                let BPS = BPQ * BPP;
                let BPT = BPS + BPS;
                let BPU = BPO * BPP;
                let BPV = ((BPM * (AF * BPN)) * BPP) + (BPQ * BPO);
                let BPX = (BPW * BPU) / BPJ;
                let BPY = ((BPV * BPW) - (BPK * BPX)) / BPJ;
                let BPZ = (AAM * BPR) + BPX;
                let BQA = (BPT * AAM) + BPY;
                BQQ = BPJ;
                BQR = BPU;
                BQS = BPR;
                BQT = BPX;
                BQU = BPZ;
                BQV = BPK;
                BQW = BPV;
                BQX = BPT;
                BQY = BPY;
                BQZ = BQA;
            } else {
                let BQB = BPG.sqrt();
                let BQC = BPH * (P / (O * BQB));
                let BQD = Q * BQB;
                let BQE = BQD.sinh();
                let BQF = DA / BQE;
                let BQG = BQF * BQF;
                let BQH = (((((BQC * Q) * (BQD.cosh())) * BQF) * AF) / BQE) * BQF;
                let BQI = BQH + BQH;
                let BQJ = (DA + BQG).sqrt();
                let BQK = BQI * (P / (O * BQJ));
                let BQL = (Q * BQJ) / BQB;
                let BQM = ((BQK * Q) - (BQC * BQL)) / BQB;
                let BQO = (BQN * BQG) + BQL;
                let BQP = (BQI * BQN) + BQM;
                BQQ = BQB;
                BQR = BQJ;
                BQS = BQG;
                BQT = BQL;
                BQU = BQO;
                BQV = BQC;
                BQW = BQK;
                BQX = BQI;
                BQY = BQM;
                BQZ = BQP;
            }
            let BRA = BPA + (BQQ * BQR);
            let BRB = BPB + ((BQV * BQR) + (BQW * BQQ));
            let BRC = DA / BRA;
            let BRD = ((BRB * BRC) * AF) / BRA;
            let BRE = BPG * BQS;
            let BRF = BRE * BRC;
            let BRG = BRF * BRC;
            let BRH = BRG.abs();
            let BRI = (BEY + BOY) - (BRH.ln());
            let BRJ = (BEZ + BOZ) - ((((((((BPH * BQS) + (BQX * BPG)) * BRC) + (BRD * BRE)) * BRC) + (BRD * BRF)) * ((O * (if BRG >= XA { 1.0 } else { 0.0 })) - P)) * (P / BRH));
            let BRK = (TR * BRI) + BPA;
            let BRL = DA / BPG;
            let BRM = BRL - BQT;
            let BRO = (BRN * BPA) + BPD;
            let BRP = (BPB * BRN) + BPE;
            let BRQ = BQU * BRO;
            let BRR = (BQZ * BRO) + (BRP * BQU);
            let BRS = ACI + BRQ;
            let BRT = (-1e0f64 + (AN * (BRS * BRC))) - (BRM * BRO);
            let BRU = BRQ - UA;
            let BRV = ((BPD - (UA * (BPA + BRA))) + (BPA * BRQ)) + (TR * ((BRT * BRA) + (BRI * BRU)));
            let BRW = (-(BPD + (BRA * BRK))) / BRV;
            let BRX = BOW + BRW;
            let BRY = BOX + ((((BPE + ((BRB * BRK) + (((BRJ * TR) + BPB) * BRA))) * AF) - ((((BPE - ((BPB + BRB) * UA)) + ((BPB * BRQ) + (BRR * BPA))) + ((((((((BRR * BRC) + (BRD * BRS)) * AN) - ((((((BPH * BRL) * AF) / BPG) - BQY) * BRO) + (BRP * BRM))) * BRA) + (BRB * BRT)) + ((BRJ * BRU) + (BRR * BRI))) * TR)) * BRW)) / BRV);
            let BRZ = AXE - BRX;
            let BSA = AXF - BRY;
            let BSB = BRX.exp();
            let BSC = SU * BSB;
            let BSD = Lanes([0.0, (SV * BSB), 0.0, 0.0, 0.0]) + ((BRY * BSB) * SU);
            let BSE = TK * BRZ;
            let BSF = (BSE * BRZ) - BSC;
            let BSG = (((BSA * TK) * BRZ) + (BSA * BSE)) - BSD;
            let BSH = if BSF < JV { 1.0 } else { 0.0 };
            let BTG;
            let BTH;
            let BTI;
            let BTJ;
            if BSH != 0.0 {
                let BSI = (-BSF).sqrt();
                let BSJ = (BSG * AF) * (P / (O * BSI));
                let BSK = Q * BSI;
                let BSL = BSJ * Q;
                let BSM = BSK.tan();
                let BSN = BSK.cos();
                let BSO = BSI / BSM;
                let BSP = (BSJ - ((BSL * (P / (BSN * BSN))) * BSO)) / BSM;
                let BSQ = BSK.sin();
                let BSR = BSL * BSN;
                let BSS = -BSQ;
                let BST = BSS * BSQ;
                let BSU = ((BSR * AF) * BSQ) + (BSR * BSS);
                BTG = BSO;
                BTH = BST;
                BTI = BSP;
                BTJ = BSU;
            } else {
                let BSV = BSF.sqrt();
                let BSW = BSG * (P / (O * BSV));
                let BSX = Q * BSV;
                let BSY = BSW * Q;
                let BSZ = BSX.sinh();
                let BTA = BSZ * BSZ;
                let BTB = (BSY * (BSX.cosh())) * BSZ;
                let BTC = BTB + BTB;
                let BTD = BSX.tanh();
                let BTE = BSV / BTD;
                let BTF = (BSW - ((BSY * (P - (BTD * BTD))) * BTE)) / BTD;
                BTG = BTE;
                BTH = BTA;
                BTI = BTF;
                BTJ = BTC;
            }
            let BTK = BTH * BSC;
            let BTL = BSF / BTK;
            let BTM = DA - BTL;
            let BTN = ((UA * BRZ) - BTG) / BTM;
            let BTO = (((BSA * UA) - BTI) - ((((BSG - (((BTJ * BSC) + (BSD * BTH)) * BTL)) / BTK) * AF) * BTN)) / BTM;
            let BTP = BRZ * AQK;
            let BTQ = BTP * SE;
            let BTR = ((BSA * AQK) * SE) + (SF * BTP);
            let BTS = BTN * AQO;
            let BTT = BTS * SE;
            let BTU = ((BTO * AQO) * SE) + (SF * BTS);
            let BTV = BTT - BTQ;
            let BTW = BTU - BTR;
            let BTX = BTT / AQK;
            let BTY = BTU / AQK;
            let BTZ = Q * (ARC + BTX);
            let BUA = (ARD + BTY) * Q;
            let BUB = ARC - BTX;
            let BUC = ARD - BTY;
            let BUD = AXC * AXC;
            let BUF = BUD / BUE;
            let BUG = (AXD * (AN * AXC)) / BUE;
            let BUS;
            let BUT;
            if BUH != 0.0 {
                let BUJ = -BUF;
                let BUL = (BUK * (DA - (rspice_limited_exp(BUJ)))) * Q;
                let BUM = AQM - BTQ;
                let BUN = ((AQM + BTQ) / BUI) + ((BUL * BUM) / AQK);
                let BUO = ((AQN + BTR) / BUI) + ((((((((BUG * AF) * (rspice_limited_exp_derivative(BUJ))) * AF) * BUK) * Q) * BUM) + ((AQN - BTR) * BUL)) / AQK);
                BUS = BUN;
                BUT = BUO;
            } else {
                let BUQ = (AQM + BTQ) / BUP;
                let BUR = (AQN + BTR) / BUP;
                BUS = BUQ;
                BUT = BUR;
            }
            let BVE;
            let BVF;
            if BUU != 0.0 {
                let BUW = -BUF;
                let BUY = (BUX * (DA - (rspice_limited_exp(BUW)))) * Q;
                let BUZ = AQS - BTV;
                let BVA = ((AQS + BTV) / BUV) + ((BUY * BUZ) / AQU);
                let BVB = ((AQT + BTW) / BUV) + ((((((((BUG * AF) * (rspice_limited_exp_derivative(BUW))) * AF) * BUX) * Q) * BUZ) + ((AQT - BTW) * BUY)) / AQU);
                BVE = BVA;
                BVF = BVB;
            } else {
                let BVC = (AQS + BTV) / BUV;
                let BVD = (AQT + BTW) / BUV;
                BVE = BVC;
                BVF = BVD;
            }
            let BVG = BUT * ARE;
            let BVH = (ARE * BUS) + ARG;
            let BVI = BVG * BVH;
            let BVJ = ((BVH * BVH) + ARJ).sqrt();
            let BVK = ARL * (Q * (BVH + BVJ));
            let BVL = BVF * ARN;
            let BVM = (ARN * BVE) + ARG;
            let BVN = BVL * BVM;
            let BVO = ((BVM * BVM) + ARJ).sqrt();
            let BVP = ARS * (Q * (BVM + BVO));
            let BVQ = BTZ / ARU;
            let BVR = Q * (DA + (BVQ.abs()));
            let BVS = ((BUA / ARU) * ((O * (if BVQ >= XA { 1.0 } else { 0.0 })) - P)) * Q;
            let BVT = BVR.powf(EB);
            let BVU = LH * DL;
            let BVV = DT + (LG * DL);
            let BVW = BVK.abs();
            let BVX = ASF + (ASE * LG);
            let BVY = BVW.powf(BVX);
            let BVZ = (LH * ASE) * (BVY * (BVW.ln()));
            let BWA = (ASC + (Lanes([BVU[0], 0.0, BVU[1], BVU[2]]) + Lanes([0.0, (DM * LG), 0.0, 0.0]))) * BVY;
            let BWC = LH * BWB;
            let BWD = Lanes([0.0, DY, 0.0, 0.0]) + Lanes([BWC[0], 0.0, BWC[1], BWC[2]]);
            let BWE = (DX + (LG * BWB)) / BVT;
            let BWF = (Lanes([BWA[0], BWA[1], BWA[2], BWA[3], 0.0]) + (((((((BVG + ((BVI + BVI) * (P / (O * BVJ)))) * Q) * ARL) * ((O * (if BVK >= XA { 1.0 } else { 0.0 })) - P)) * (BVX * (BVW.powf((BVX - P))))) + Lanes([BVZ[0], 0.0, BVZ[1], BVZ[2], 0.0])) * BVV)) + ((Lanes([BWD[0], BWD[1], BWD[2], BWD[3], 0.0]) - (((BVS * (EB * (BVR.powf(ARZ)))) + Lanes([0.0, (EC * (BVT * (BVR.ln()))), 0.0, 0.0, 0.0])) * BWE)) / BVT);
            let BWG = DA + ((BVV * BVY) + BWE);
            let BWH = BWG - DA;
            let BWI = BWF * BWH;
            let BWJ = ((BWH * BWH) + ASP).sqrt();
            let BWK = (Q * ((BWG + DA) + BWJ)) / ASR;
            let BWL = DC / BWK;
            let BWM = BVR.powf(ASV);
            let BWN = ASZ + (LG * ASY);
            let BWO = BVP.abs();
            let BWP = ATD + (ATC * LG);
            let BWQ = BWO.powf(BWP);
            let BWR = (LH * ATC) * (BWQ * (BWO.ln()));
            let BWS = (LH * ASY) * BWQ;
            let BWU = LH * BWT;
            let BWV = (ATI + (LG * BWT)) / BWM;
            let BWW = (Lanes([BWS[0], 0.0, BWS[1], BWS[2], 0.0]) + (((((((BVL + ((BVN + BVN) * (P / (O * BVO)))) * Q) * ARS) * ((O * (if BVP >= XA { 1.0 } else { 0.0 })) - P)) * (BWP * (BWO.powf((BWP - P))))) + Lanes([BWR[0], 0.0, BWR[1], BWR[2], 0.0])) * BWN)) + ((Lanes([BWU[0], 0.0, BWU[1], BWU[2], 0.0]) - ((BVS * (ASV * (BVR.powf(ASX)))) * BWV)) / BWM);
            let BWX = DA + ((BWN * BWQ) + BWV);
            let BWY = BWX - DA;
            let BWZ = BWW * BWY;
            let BXA = ((BWY * BWY) + ASP).sqrt();
            let BXB = (Q * ((BWX + DA) + BXA)) / ASR;
            let BXC = ATQ / BXB;
            let BXD = AQM + BTQ;
            let BXE = AQN + BTR;
            let BXG = AQS + BTV;
            let BXH = AQT + BTW;
            let BXI = (SP - (BXD / BXF)) / SE;
            let BXJ = BXI.exp();
            let BXK = (((SN - (BXE / BXF)) - (SF * BXI)) / SE) * BXJ;
            let BXL = (TC - (BXG / BUV)) / SE;
            let BXM = BXL.exp();
            let BXN = (((TD - (BXH / BUV)) - (SF * BXL)) / SE) * BXM;
            let BXO = BXJ + BXM;
            let BXP = BXK + BXN;
            let BXQ = BXJ / BXO;
            let BXR = BXM / BXO;
            let BXS = (BXQ * BWL) + (BXR * BXC);
            let BXT = ((((BXK - (BXP * BXQ)) / BXO) * BWL) + (((ASU - ((((BWF + ((BWI + BWI) * (P / (O * BWJ)))) * Q) / ASR) * BWL)) / BWK) * BXQ)) + ((((BXN - (BXP * BXR)) / BXO) * BXC) + (((((((BWW + ((BWZ + BWZ) * (P / (O * BXA)))) * Q) / ASR) * BXC) * AF) / BXB) * BXR));
            let BXU = ((BXS * AQK) * AVU) / AUK;
            let BXV = ((BXT * AQK) * AVU) / AUK;
            let BXX = ARL * (ARG + (BXW * BTZ));
            let BXY = BXX.abs();
            let BXZ = BXY.powf(ASF);
            let BYA = Lanes([0.0, (DU * BXZ), 0.0, 0.0, 0.0]) + (((((BUA * BXW) * ARL) * ((O * (if BXX >= XA { 1.0 } else { 0.0 })) - P)) * (ASF * (BXY.powf(staged[266])))) * DT);
            let BYB = DA + (DT * BXZ);
            let BYC = BYB - DA;
            let BYD = BYA * BYC;
            let BYE = ((BYC * BYC) + ASP).sqrt();
            let BYF = (Q * ((BYB + DA) + BYE)) / ASR;
            let BYG = (AN * FI) / BXS;
            let BYH = BYG * AUK;
            let BYI = LH * GH;
            let BYJ = Lanes([0.0, (GI * LG), 0.0, 0.0]) + Lanes([BYI[0], 0.0, BYI[1], BYI[2]]);
            let BYK = 8e-1f64 + (GH * LG);
            let BYL = BYJ * BYK;
            let BYM = ((BYK * BYK) + PX).sqrt();
            let BYN = 2e-1f64 + (Q * (BYK + BYM));
            let BYO = BUB / BYH;
            let BYP = BYO * BYN;
            let BYQ = ((BYJ + ((BYL + BYL) * (P / (O * BYM)))) * Q) * BYO;
            let BYR = ((((BUC - ((((Lanes([0.0, (FJ * AN), 0.0, 0.0, 0.0]) - (BXT * BYG)) / BXS) * AUK) * BYO)) / BYH) * BYN) + Lanes([BYQ[0], BYQ[1], BYQ[2], BYQ[3], 0.0])) * BYP;
            let BYS = (parameters[109] + (BYP * BYP)).sqrt();
            let BYV = QN * BYU;
            let BYX = LH * BYW;
            let BYY = Q * ((HL - (BYU * QM)) - (BYW * LG));
            let BYZ = BYY * BTZ;
            let BZA = (((Lanes([0.0, HM, 0.0, 0.0]) - Lanes([BYV[0], 0.0, BYV[1], BYV[2]])) - Lanes([BYX[0], 0.0, BYX[1], BYX[2]])) * Q) * BTZ;
            let BZB = BYZ * BUB;
            let BZC = ((DA + BYS) / BYT) + (BZB * BUB);
            let BZD = (((BYR + BYR) * (P / (O * BYS))) / BYT) + (((((Lanes([BZA[0], BZA[1], BZA[2], BZA[3], 0.0]) + (BUA * BYY)) * BUB) + (BUC * BYZ)) * BUB) + (BUC * BZB));
            let BZE = BZC - DA;
            let BZF = BZD * BZE;
            let BZG = ((BZE * BZE) + staged[165]).sqrt();
            let BZH = Q * ((BZC + DA) + BZG);
            let BZI = (BZD + ((BZF + BZF) * (P / (O * BZG)))) * Q;
            let BZJ = AN * FP;
            let BZK = (BZJ * BYF) / DC;
            let BZM = BZK * BZL;
            let BZN = (((Lanes([0.0, ((FQ * AN) * BYF), 0.0, 0.0, 0.0]) + ((((BYA + ((BYD + BYD) * (P / (O * BYE)))) * Q) / ASR) * BZJ)) - Lanes([0.0, (DD * BZK), 0.0, 0.0, 0.0])) / DC) * BZL;
            let BZX;
            let BZY;
            if BZO != 0.0 {
                let BZQ = (BZP * BTZ) / AUL;
                let BZR = ((BUA * BZP) - (AUM * BZQ)) / AUL;
                let BZS = DA + BZQ;
                BZX = BZS;
                BZY = BZR;
            } else {
                let BZT = (BZP * BTZ) / AUL;
                let BZU = DA - BZT;
                let BZV = DA / BZU;
                let BZW = ((((((BUA * BZP) - (AUM * BZT)) / AUL) * AF) * BZV) * AF) / BZU;
                BZX = BZV;
                BZY = BZW;
            }
            let BZZ = KG - AXC;
            let CAA = AWU - AXD;
            let CAB = BTZ + AO;
            let CAC = BUA + Lanes([0.0, AP, 0.0, 0.0, 0.0]);
            let CAD = if NY > JV { 1.0 } else { 0.0 };
            let CAM;
            let CAN;
            if CAD != 0.0 {
                let CAE = AWS + CAB;
                let CAF = CAB / CAE;
                let CAG = CAB / NY;
                let CAH = CAG * CAF;
                let CAI = CAH * BZX;
                let CAJ = BZZ / CAI;
                let CAK = (CAA - (((((((CAC - (NZ * CAG)) / NY) * CAF) + (((CAC - ((AWR + CAC) * CAF)) / CAE) * CAG)) * BZX) + (BZY * CAH)) * CAJ)) / CAI;
                let CAL = DA + CAJ;
                CAM = CAL;
                CAN = CAK;
            } else {
                CAM = DA;
                CAN = AUE;
            }
            let CAQ;
            let CAR;
            if CAO != 0.0 {
                let CBC;
                let CBD;
                if CAP != 0.0 {
                    let CAW = staged[168] - (CAV * BTZ);
                    let CAX = DA / CAW;
                    let CAY = ((((BUA * CAV) * AF) * CAX) * AF) / CAW;
                    CBC = CAX;
                    CBD = CAY;
                } else {
                    let CBA = CAZ * (DA + (CAV * BTZ));
                    let CBB = (BUA * CAV) * CAZ;
                    CBC = CBA;
                    CBD = CBB;
                }
                let CBE = BZZ / CBC;
                let CBF = AWS + AUL;
                let CBG = CBE / CBF;
                let CBH = DA + CBG;
                let CBI = if CBH >= AY { CBH } else { AY };
                let CBJ = CBI.ln();
                let CBK = (CBD * CBJ) + (((((((CAA - (CBD * CBE)) / CBC) - ((AWR + AUM) * CBG)) / CBF) * (if CBH >= AY { 1.0 } else { 0.0 })) * (P / CBI)) * CBC);
                let CBL = DA + (CBC * CBJ);
                CAQ = CBL;
                CAR = CBK;
            } else {
                CAQ = DA;
                CAR = AUE;
            }
            let CAS = CAM * CAQ;
            let CAT = (CAN * CAQ) + (CAR * CAM);
            let CBT;
            let CBU;
            if CAU != 0.0 {
                let CBN = AWS + BZM;
                let CBO = (BZZ / CBM) / CBN;
                let CBP = DA + CBO;
                let CBQ = if CBP >= AY { CBP } else { AY };
                let CBR = (((((CAA / CBM) - ((AWR + BZN) * CBO)) / CBN) * (if CBP >= AY { 1.0 } else { 0.0 })) * (P / CBQ)) * CBM;
                let CBS = DA + (CBM * (CBQ.ln()));
                CBT = CBS;
                CBU = CBR;
            } else {
                CBT = DA;
                CBU = AUE;
            }
            let CBV = if GT != JV { 1.0 } else { 0.0 };
            let CCE;
            let CCF;
            if CBV != 0.0 {
                let CBW = HE * BUB;
                let CBX = HB + (CBW * BUB);
                let CBY = if JV >= CBX { JV } else { CBX };
                let CBZ = (CBY * BTZ) + (AN * SE);
                let CCA = GT / CBZ;
                let CCB = -CCA;
                let CCC = rspice_limited_exp(CCB);
                let CCD = (((Lanes([0.0, GS, 0.0, 0.0, 0.0]) - ((((((Lanes([0.0, HA, 0.0, 0.0, 0.0]) + (((Lanes([0.0, (HD * BUB), 0.0, 0.0, 0.0]) + (BUC * HE)) * BUB) + (BUC * CBW))) * (P - (if JV >= CBX { 1.0 } else { 0.0 }))) * BTZ) + (BUA * CBY)) + (SF * AN)) * CCA)) / CBZ) * AF) * (rspice_limited_exp_derivative(CCB));
                CCE = CCC;
                CCF = CCD;
            } else {
                CCE = DA;
                CCF = AUE;
            }
            let CCG = AQI - BTN;
            let CCH = AQJ * AQI;
            let CCI = BTO * BTN;
            let CCJ = (AQI * AQI) - (BTN * BTN);
            let CCK = AQO * SE;
            let CCL = SF * AQO;
            let CCM = CCK * AN;
            let CCN = CCM * Y;
            let CCO = CCK * AQO;
            let CCP = (CCO * SE) * Q;
            let CCQ = (CCN * CCG) + ((CCP * CCJ) / AQK);
            let CCR = (((((CCL * AN) * Y) + Lanes([0.0, (Z * CCM), 0.0, 0.0, 0.0])) * CCG) + ((AQJ - BTO) * CCN)) + (((((((CCL * AQO) * SE) + (SF * CCO)) * Q) * CCJ) + (((CCH + CCH) - (CCI + CCI)) * CCP)) / AQK);
            let CCS = BTZ + Y;
            let CCT = BUA + Lanes([0.0, Z, 0.0, 0.0, 0.0]);
            let CES;
            let CET;
            let CEU;
            let CEV;
            let CEW;
            let CEX;
            if A != 0.0 {
                let CCU = IX - CN;
                let CCV = Lanes([0.0, IY[0], IY[1]]) - Lanes([CO, 0.0, 0.0]);
                let CCW = CCV * CCU;
                let CCX = ((CCU * CCU) + FU).sqrt();
                let CCY = DA + (AUU * (Q * (CCU + CCX)));
                let CCZ = DA / CCY;
                let CDA = (((((CCV + ((CCW + CCW) * (P / (O * CCX)))) * Q) * AUU) * CCZ) * AF) / CCY;
                let CDC = (JO * Q) * CDB;
                let CDD = CCZ - ((Q * JN) * CDB);
                let CDE = Lanes([0.0, CDA[0], CDA[1], CDA[2]]) - Lanes([CDC[0], 0.0, CDC[1], 0.0]);
                let CDF = CDE * CDD;
                let CDG = ((CDD * CDD) + PX).sqrt();
                let CDI = staged[174] + ((staged[173] + (CDH * (Q * (CDD + CDG)))) * AVC);
                let CDJ = EI * CDI;
                let CDK = Lanes([0.0, (EJ * CDI), 0.0, 0.0]) + (((((CDE + ((CDF + CDF) * (P / (O * CDG)))) * Q) * CDH) * AVC) * EI);
                let CDL = JH - CN;
                let CDM = Lanes([0.0, JI[0], JI[1]]) - Lanes([CO, 0.0, 0.0]);
                let CDN = CDM * CDL;
                let CDO = ((CDL * CDL) + FU).sqrt();
                let CDP = DA + (AUU * (Q * (CDL + CDO)));
                let CDQ = DA / CDP;
                let CDR = (((((CDM + ((CDN + CDN) * (P / (O * CDO)))) * Q) * AUU) * CDQ) * AF) / CDP;
                let CDS = (JS * Q) * CDB;
                let CDT = CDQ - ((Q * JR) * CDB);
                let CDU = Lanes([0.0, CDR[0], CDR[1], CDR[2]]) - Lanes([CDS[0], 0.0, CDS[1], 0.0]);
                let CDV = CDU * CDT;
                let CDW = ((CDT * CDT) + PX).sqrt();
                let CDY = staged[177] + ((staged[176] + (CDX * (Q * (CDT + CDW)))) * AVC);
                let CDZ = EI * CDY;
                let CEA = Lanes([0.0, (EJ * CDY), 0.0, 0.0]) + (((((CDU + ((CDV + CDV) * (P / (O * CDW)))) * Q) * CDX) * AVC) * EI);
                CES = DA;
                CET = CDZ;
                CEU = CDJ;
                CEV = AUE;
                CEW = CEA;
                CEX = CDK;
            } else {
                let CEB = DA + (AUU * BTZ);
                let CEC = DA / CEB;
                let CED = ((KO + KM) * Q) * CDB;
                let CEE = CEC - ((Q * (KJ + KH)) * CDB);
                let CEF = ((((BUA * AUU) * CEC) * AF) / CEB) - Lanes([CED[0], 0.0, CED[1], CED[2], 0.0]);
                let CEG = CEF * CEE;
                let CEH = ((CEE * CEE) + PX).sqrt();
                let CEI = O * CEH;
                let CEJ = CEE + CEH;
                let CEK = (AVB + (AVA * (Q * CEJ))) * AVC;
                let CEL = EI * CEK;
                let CEM = AVD * BXU;
                let CEN = (CEM * CCS) / BZH;
                let CEO = ((((BXV * AVD) * CCS) + (CCT * CEM)) - (BZI * CEN)) / BZH;
                let CEP = (CEO * CEL) + ((Lanes([0.0, (EJ * CEK), 0.0, 0.0, 0.0]) + (((((CEF + ((CEG + CEG) * (P / CEI))) * Q) * AVA) * AVC) * EI)) * CEN);
                let CEQ = DA + (CEN * CEL);
                let CFU;
                let CFV;
                if CER != 0.0 {
                    let CFQ = staged[178] + (AVA * (Q * CEJ));
                    let CFR = (EI * CFQ) * AVC;
                    let CFS = (CEO * CFR) + (((Lanes([0.0, (EJ * CFQ), 0.0, 0.0, 0.0]) + ((((CEF + ((CEG + CEG) * (P / CEI))) * Q) * AVA) * EI)) * AVC) * CEN);
                    let CFT = DA + (CEN * CFR);
                    CFU = CFT;
                    CFV = CFS;
                } else {
                    CFU = CEQ;
                    CFV = CEP;
                }
                CES = CFU;
                CET = CFW;
                CEU = CFX;
                CEV = CFV;
                CEW = CFY;
                CEX = CFZ;
            }
            let CEY = BXU / AQK;
            let CEZ = CEY * CCQ;
            let CFA = CEZ * CAS;
            let CFB = BZH * CES;
            let CFC = (CFA * CCE) / CFB;
            let CFD = AVD * CFC;
            let CFE = (((((((((BXV / AQK) * CCQ) + (CCR * CEY)) * CAS) + (CAT * CEZ)) * CCE) + (CCF * CFA)) - (((BZI * CES) + (CEV * BZH)) * CFC)) / CFB) * AVD;
            let CFF = BXD / AN;
            let CFG = BXE / AN;
            let CFI = CFH * (AQQ + (AN * BTT));
            let CFJ = (AQR + (BTU * AN)) * CFH;
            let CFL = CFK * ((AN * AQQ) + BTT);
            let CFM = ((AQR * AN) + BTU) * CFK;
            let CFN = BXG / AN;
            let CFO = BXH / AN;
            let CGC = if CFP != 0.0 {
                let CGA = 3.4531302e-11f64 / (staged[185] + (((SR / (DA + (((BTZ + staged[179]) / staged[180]).powf(staged[181])))) * staged[183]) / staged[184]));
                CGA
            } else {
                CGB
            };
            let CGD = staged[186] / CBT;
            let CGE = ((CBU * CGD) * AF) / CBT;
            let CGF = CFF * CGD;
            let CGG = (CFG * CGD) + (CGE * CFF);
            let CGH = -CFI;
            let CGI = CGH * CGD;
            let CGJ = ((CFJ * AF) * CGD) + (CGE * CGH);
            let CGK = CFN * CGD;
            let CGL = (CFO * CGD) + (CGE * CFN);
            let CGM = -CFL;
            let CGN = CGM * CGD;
            let CGO = ((CFM * AF) * CGD) + (CGE * CGM);
            let CGQ = KW * CGP;
            let CGS = KS * CGR;
            let CGT = KX - CN;
            let CGU = Lanes([0.0, KY[0], KY[1]]) - Lanes([CO, 0.0, 0.0]);
            let CGX = ((Lanes([JO[0], 0.0, JO[1]]) - Lanes([0.0, CQ, 0.0])) * CGV) * CGW;
            let CGY = (CGT + LC) + ((CGV * ((JN - CP) - parameters[268])) * CGW);
            let CGZ = Lanes([0.0, CGU[0], CGU[1], CGU[2]]);
            let CHA = CGZ + Lanes([CGX[0], CGX[1], CGX[2], 0.0]);
            let CHB = CHA * CGY;
            let CHC = ((CGY * CGY) + 8e-2f64).sqrt();
            let CHD = Q * (CGY - CHC);
            let CHE = (CHA - ((CHB + CHB) * (P / (O * CHC)))) * Q;
            let CHG = (DA - ((ZB * CHD) / CHF)).sqrt();
            let CHJ = KT - CN;
            let CHK = Lanes([0.0, KU[0], KU[1]]) - Lanes([CO, 0.0, 0.0]);
            let CHM = ((Lanes([JS[0], 0.0, JS[1]]) - Lanes([0.0, CQ, 0.0])) * CGV) * CHL;
            let CHN = (CHJ + LC) + ((CGV * ((JR - CP) - parameters[270])) * CHL);
            let CHO = Lanes([0.0, CHK[0], CHK[1], CHK[2]]);
            let CHP = CHO + Lanes([CHM[0], CHM[1], CHM[2], 0.0]);
            let CHQ = CHP * CHN;
            let CHR = ((CHN * CHN) + 8e-2f64).sqrt();
            let CHS = Q * (CHN - CHR);
            let CHT = (CHP - ((CHQ + CHQ) * (P / (O * CHR)))) * Q;
            let CHV = (DA - ((ZB * CHS) / CHU)).sqrt();
            let CHZ = KW * CHY;
            let CIB = KS * CIA;
            let CIC = ((CGP * KV) + (CHI * ((CGT - CHD) - (CHH * (CHG - DA))))) + (CHY * KV);
            let CID = (Lanes([0.0, 0.0, CGQ[0], CGQ[1]]) + (((CGZ - CHE) - (((((CHE * ZB) / CHF) * AF) * (P / (O * CHG))) * CHH)) * CHI)) + Lanes([0.0, 0.0, CHZ[0], CHZ[1]]);
            let CIE = ((CGR * KQ) + (CHX * ((CHJ - CHS) - (CHW * (CHV - DA))))) + (CIA * KQ);
            let CIF = (Lanes([0.0, 0.0, CGS[0], CGS[1]]) + (((CHO - CHT) - (((((CHT * ZB) / CHU) * AF) * (P / (O * CHV))) * CHW)) * CHX)) + Lanes([0.0, 0.0, CIB[0], CIB[1]]);
            let CIH = CIG * (IU - JJ);
            let CII = (JM - JL) * CIG;
            let CIK = CIJ * (IZ - JJ);
            let CIL = (JQ - JP) * CIJ;
            let CIM = if staged[198] != 0.0 || (if HT <= JV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CIO;
            let CIP;
            if CIM != 0.0 {
                CIO = JV;
                CIP = AUE;
            } else {
                let CIN = if BZZ > (HT / 8e1f64) { 1.0 } else { 0.0 };
                let CJC;
                let CJD;
                if CIN != 0.0 {
                    let CIR = (-HT) / BZZ;
                    let CIT = CIS * BZZ;
                    let CIU = CIT * CFD;
                    let CIV = rspice_limited_exp(CIR);
                    let CIW = CIU * CIV;
                    let CIX = ((((CAA * CIS) * CFD) + (CFE * CIT)) * CIV) + ((((Lanes([0.0, (HU * AF), 0.0, 0.0, 0.0]) - (CAA * CIR)) / BZZ) * (rspice_limited_exp_derivative(CIR))) * CIU);
                    CJC = CIW;
                    CJD = CIX;
                } else {
                    let CIY = CIS * BZZ;
                    let CJA = (CIY * CFD) * CIZ;
                    let CJB = (((CAA * CIS) * CFD) + (CFE * CIY)) * CIZ;
                    CJC = CJA;
                    CJD = CJB;
                }
                CIO = CJC;
                CIP = CJD;
            }
            let CKN;
            let CKO;
            let CKP;
            let CKQ;
            if CIQ != 0.0 {
                let CJF = ((BTZ - staged[200]) / CJE) / Y;
                let CJG = CJE * Y;
                let CJH = DA + (rspice_limited_exp(CJF));
                let CJI = if CJH >= AY { CJH } else { AY };
                let CJJ = CJI.ln();
                let CJK = CJG * CJJ;
                let CJN = DA + (CJM * BTZ);
                let CJP = CJO * (staged[203] - (CJL * BTZ));
                let CJQ = CJP * CJN;
                let CJR = rspice_limited_exp(CJQ);
                let CJT = CJS * JT;
                let CJU = CJT * CJK;
                let CJV = (JU * CJS) * CJK;
                let CJW = CJU * CJR;
                let CJX = CJW * IO;
                let CJY = ((((Lanes([CJV[0], 0.0, 0.0, 0.0, CJV[1]]) + ((Lanes([0.0, ((Z * CJE) * CJJ), 0.0, 0.0, 0.0]) + (((((((BUA / CJE) - Lanes([0.0, (Z * CJF), 0.0, 0.0, 0.0])) / Y) * (rspice_limited_exp_derivative(CJF))) * (if CJH >= AY { 1.0 } else { 0.0 })) * (P / CJI)) * CJG)) * CJT)) * CJR) + (((((((BUA * CJL) * AF) * CJO) * CJN) + ((BUA * CJM) * CJP)) * (rspice_limited_exp_derivative(CJQ))) * CJU)) * IO) + Lanes([0.0, (IP * CJW), 0.0, 0.0, 0.0]);
                let CJZ = CC - BG;
                let CKA = CD - BH;
                let CKB = CJZ - JT;
                let CKC = Lanes([0.0, CKA, 0.0]) - Lanes([JU[0], 0.0, JU[1]]);
                let CKE = (CKB / CKD) / Y;
                let CKF = CKD * Y;
                let CKG = DA + (rspice_limited_exp(CKE));
                let CKH = if CKG >= AY { CKG } else { AY };
                let CKI = CKH.ln();
                let CKJ = CKF * CKI;
                let CKK = Lanes([0.0, ((Z * CKD) * CKI), 0.0]) + (((((((CKC / CKD) - Lanes([0.0, (Z * CKE), 0.0])) / Y) * (rspice_limited_exp_derivative(CKE))) * (if CKG >= AY { 1.0 } else { 0.0 })) * (P / CKH)) * CKF);
                let CKL = if CJZ <= JV { 1.0 } else { 0.0 };
                let CLS;
                let CLT;
                if CKL != 0.0 {
                    let CLH = CKB - LC;
                    let CLI = CKC * CLH;
                    let CLK = ((CLH * CLH) - (CLJ * CJZ)).sqrt();
                    let CLL = Q * (CLH + CLK);
                    let CLM = (CKC + (((CLI + CLI) - Lanes([0.0, (CKA * CLJ), 0.0])) * (P / (O * CLK)))) * Q;
                    CLS = CLL;
                    CLT = CLM;
                } else {
                    let CLN = CKB - LC;
                    let CLO = CKC * CLN;
                    let CLP = ((CLN * CLN) + (CLJ * CJZ)).sqrt();
                    let CLQ = Q * (CLN + CLP);
                    let CLR = (CKC + (((CLO + CLO) + Lanes([0.0, (CKA * CLJ), 0.0])) * (P / (O * CLP)))) * Q;
                    CLS = CLQ;
                    CLT = CLR;
                }
                let CLW = DA + (CLV * CLS);
                let CLY = CLX * (staged[209] - (CLU * CLS));
                let CLZ = CLY * CLW;
                let CMA = rspice_limited_exp(CLZ);
                let CMC = CMB * JT;
                let CMD = CMC * CKJ;
                let CME = (JU * CMB) * CKJ;
                let CMF = CMD * CMA;
                let CMG = CMF * IO;
                let CMH = ((((Lanes([CME[0], 0.0, CME[1]]) + (CKK * CMC)) * CMA) + (((((((CLT * CLU) * AF) * CLX) * CLW) + ((CLT * CLV) * CLY)) * (rspice_limited_exp_derivative(CLZ))) * CMD)) * IO) + Lanes([0.0, (IP * CMF), 0.0]);
                CKN = CJX;
                CKO = CMG;
                CKP = CJY;
                CKQ = CMH;
            } else {
                CKN = JV;
                CKO = JV;
                CKP = AUE;
                CKQ = CKM;
            }
            let CKS = JG * CKR;
            let CKT = (CKR * JF) / Y;
            let CKU = CKT.tanh();
            let CKV = (((Lanes([0.0, CKS[0], CKS[1]]) - Lanes([(Z * CKT), 0.0, 0.0])) / Y) * (P - (CKU * CKU))) * Q;
            let CKW = Q + (Q * CKU);
            let CKX = DA - CKW;
            let CKY = CKN + CKO;
            let CKZ = CKP + Lanes([CKQ[0], CKQ[1], 0.0, 0.0, CKQ[2]]);
            let CLA = CKW * CKY;
            let CLB = CKV * CKY;
            let CLC = Lanes([0.0, CLB[0], CLB[1], CLB[2], 0.0]) + (CKZ * CKW);
            let CLD = CKX * CKY;
            let CLE = (CKV * AF) * CKY;
            let CLF = Lanes([0.0, CLE[0], CLE[1], CLE[2], 0.0]) + (CKZ * CKX);
            let COL;
            let COM;
            let CON;
            let COO;
            let COP;
            let COQ;
            let COR;
            let COS;
            if CLG != 0.0 {
                let CMJ = LI - (CMI * ARA);
                let CMK = SM - (ARB * CMI);
                let CMN = DA + (CMM * CMJ);
                let CMP = CMO * (staged[215] - (CML * CMJ));
                let CMQ = CMP * CMN;
                let CMR = rspice_limited_exp(CMQ);
                let CMS = LB * Q;
                let CMT = (Lanes([JO[0], 0.0, JO[1]]) + Lanes([JS[0], JS[1], 0.0])) * Q;
                let CMU = (JT + (Q * LD)) + (Q * (JN + JR));
                let CMW = CMV * (BTZ * CMR);
                let CMX = CMW * CMU;
                let CMY = ((Lanes([JU[0], 0.0, 0.0, JU[1]]) + Lanes([0.0, CMS[0], CMS[1], 0.0])) + Lanes([CMT[0], CMT[1], CMT[2], 0.0])) * CMW;
                let CMZ = CMX * IO;
                let CNA = ((((((BUA * CMR) + (((((((CMK * CML) * AF) * CMO) * CMN) + ((CMK * CMM) * CMP)) * (rspice_limited_exp_derivative(CMQ))) * BTZ)) * CMV) * CMU) + Lanes([CMY[0], 0.0, CMY[1], CMY[2], CMY[3]])) * IO) + Lanes([0.0, (IP * CMX), 0.0, 0.0, 0.0]);
                let CNB = AXD * AXC;
                let CNC = (BUD + PX).sqrt();
                let CNE = CND * (CNC - 1e-1f64);
                let CNF = ((CNB + CNB) * (P / (O * CNC))) * CND;
                let CNG = -CNE;
                let CNH = rspice_limited_exp(CNG);
                let CNI = (CNF * AF) * (rspice_limited_exp_derivative(CNG));
                let CNJ = ((CNE + CNH) - DA) + FU;
                let CNK = CNE + DA;
                let CNL = (DA - (CNK * CNH)) + FU;
                let CNM = CNF * CNE;
                let CNN = CNM + CNM;
                let CNO = (CNE * CNE) + 2e-4f64;
                let CNP = (CMZ * CNL) / CNO;
                let CNQ = (((CNA * CNL) + ((((CNF * CNH) + (CNI * CNK)) * AF) * CMZ)) - (CNN * CNP)) / CNO;
                let CNR = (CMZ * CNJ) / CNO;
                let CNS = (((CNA * CNJ) + ((CNF + CNI) * CMZ)) - (CNN * CNR)) / CNO;
                let CNT = Lanes([0.0, IY[0], IY[1]]) - Lanes([CO, 0.0, 0.0]);
                let CNU = KH - CP;
                let CNV = LL - Lanes([0.0, CQ, 0.0, 0.0]);
                let CNX = CNV * CNW;
                let CNY = (IX - CN) + (CNW * CNU);
                let CNZ = (Lanes([0.0, CNT[0], 0.0, CNT[1], CNT[2]]) + Lanes([CNX[0], CNX[1], CNX[2], CNX[3], 0.0])) * CNY;
                let COA = ((CNY * CNY) + FU).sqrt();
                let COB = (CNZ + CNZ) * (P / (O * COA));
                let COE = DA + (COD * COA);
                let COG = COF * (staged[222] - (COC * COA));
                let COH = COG * COE;
                let COI = rspice_limited_exp(COH);
                let COJ = (((((COB * COC) * AF) * COF) * COE) + ((COB * COD) * COG)) * (rspice_limited_exp_derivative(COH));
                let COK = if KK > JV { 1.0 } else { 0.0 };
                let CPJ;
                let CPK;
                let CPL;
                let CPM;
                if COK != 0.0 {
                    let COV = IR * COU;
                    let COW = COV * IX;
                    let COX = IY * COV;
                    let COY = COW * COA;
                    let COZ = (Lanes([((IS * COU) * IX), 0.0, 0.0]) + Lanes([0.0, COX[0], COX[1]])) * COA;
                    let CPA = COY * COI;
                    let CPB = ((Lanes([0.0, COZ[0], 0.0, COZ[1], COZ[2]]) + (COB * COW)) * COI) + (COJ * COY);
                    CPJ = CPA;
                    CPK = JV;
                    CPL = CPB;
                    CPM = AUE;
                } else {
                    let CPC = IR * COU;
                    let CPD = CPC * IX;
                    let CPE = IY * CPC;
                    let CPF = CPD * COA;
                    let CPG = (Lanes([((IS * COU) * IX), 0.0, 0.0]) + Lanes([0.0, CPE[0], CPE[1]])) * COA;
                    let CPH = CPF * COI;
                    let CPI = ((Lanes([0.0, CPG[0], 0.0, CPG[1], CPG[2]]) + (COB * CPD)) * COI) + (COJ * CPF);
                    CPJ = JV;
                    CPK = CPH;
                    CPL = AUE;
                    CPM = CPI;
                }
                let CPN = Lanes([0.0, JI[0], JI[1]]) - Lanes([CO, 0.0, 0.0]);
                let CPP = CNV * CPO;
                let CPQ = (JH - CN) + (CPO * CNU);
                let CPR = (Lanes([0.0, CPN[0], CPN[1], 0.0, CPN[2]]) + Lanes([CPP[0], CPP[1], CPP[2], CPP[3], 0.0])) * CPQ;
                let CPS = ((CPQ * CPQ) + FU).sqrt();
                let CPT = (CPR + CPR) * (P / (O * CPS));
                let CPW = DA + (CPV * CPS);
                let CPX = COF * (staged[227] - (CPU * CPS));
                let CPY = CPX * CPW;
                let CPZ = rspice_limited_exp(CPY);
                let CQA = (((((CPT * CPU) * AF) * COF) * CPW) + ((CPT * CPV) * CPX)) * (rspice_limited_exp_derivative(CPY));
                let CQQ;
                let CQR;
                let CQS;
                let CQT;
                if COK != 0.0 {
                    let CQC = IR * CQB;
                    let CQD = CQC * JH;
                    let CQE = JI * CQC;
                    let CQF = CQD * CPS;
                    let CQG = (Lanes([((IS * CQB) * JH), 0.0, 0.0]) + Lanes([0.0, CQE[0], CQE[1]])) * CPS;
                    let CQH = CQF * CPZ;
                    let CQI = ((Lanes([0.0, CQG[0], CQG[1], 0.0, CQG[2]]) + (CPT * CQD)) * CPZ) + (CQA * CQF);
                    CQQ = CPJ;
                    CQR = CQH;
                    CQS = CPL;
                    CQT = CQI;
                } else {
                    let CQJ = IR * CQB;
                    let CQK = CQJ * JH;
                    let CQL = JI * CQJ;
                    let CQM = CQK * CPS;
                    let CQN = (Lanes([((IS * CQB) * JH), 0.0, 0.0]) + Lanes([0.0, CQL[0], CQL[1]])) * CPS;
                    let CQO = CQM * CPZ;
                    let CQP = ((Lanes([0.0, CQN[0], CQN[1], 0.0, CQN[2]]) + (CPT * CQK)) * CPZ) + (CQA * CQM);
                    CQQ = CQO;
                    CQR = CPK;
                    CQS = CQP;
                    CQT = CPM;
                }
                COL = CNP;
                COM = CNR;
                CON = CQQ;
                COO = CQR;
                COP = CNQ;
                COQ = CNS;
                COR = CQS;
                COS = CQT;
            } else {
                COL = JV;
                COM = JV;
                CON = JV;
                COO = JV;
                COP = AUE;
                COQ = AUE;
                COR = AUE;
                COS = AUE;
            }
            let CQV;
            let CQW;
            let CQX;
            let CQY;
            if COT != 0.0 {
                let CQU = if staged[229] != 0.0 || (if IB <= JV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CRZ;
                let CSA;
                if CQU != 0.0 {
                    CRZ = JV;
                    CSA = AUE;
                } else {
                    let CRB = JI * AF;
                    let CRC = Lanes([0.0, CRB[0], CRB[1]]) + Lanes([CO, 0.0, 0.0]);
                    let CRD = staged[231] * QC;
                    let CRE = (LL - Lanes([0.0, CQ, 0.0, 0.0])) * CRD;
                    let CRG = ((((-JH) - staged[230]) + CN) + (CRD * ((KH - CP) - staged[232]))) / CRF;
                    let CRH = (Lanes([0.0, CRC[0], CRC[1], 0.0, CRC[2]]) + Lanes([CRE[0], CRE[1], CRE[2], CRE[3], 0.0])) / CRF;
                    let CRI = CRH * CRG;
                    let CRJ = ((CRG * CRG) + 4e-4f64).sqrt();
                    let CRK = Q * (CRG + CRJ);
                    let CRL = (CRH + ((CRI + CRI) * (P / (O * CRJ)))) * Q;
                    let CRM = CRK + ARJ;
                    let CRN = IB / CRM;
                    let CRO = if CRK >= AY { CRK } else { AY };
                    let CRQ = CRP * (CRO.ln());
                    let CRR = staged[235] * AVU;
                    let CRS = CRR * (rspice_limited_exp(CRQ));
                    let CRT = -CRN;
                    let CRU = rspice_limited_exp(CRT);
                    let CRV = CRS * CRU;
                    let CRW = CRV * JF;
                    let CRX = JG * CRV;
                    let CRY = ((((((((CRL * (if CRK >= AY { 1.0 } else { 0.0 })) * (P / CRO)) * CRP) * (rspice_limited_exp_derivative(CRQ))) * CRR) * CRU) + (((((Lanes([0.0, IC, 0.0, 0.0, 0.0]) - (CRL * CRN)) / CRM) * AF) * (rspice_limited_exp_derivative(CRT))) * CRS)) * JF) + Lanes([0.0, 0.0, CRX[0], CRX[1], 0.0]);
                    CRZ = CRW;
                    CSA = CRY;
                }
                let CSB = if KK > JV { 1.0 } else { 0.0 };
                let CSC;
                let CSD;
                let CSE;
                let CSF;
                if CSB != 0.0 {
                    CSC = CRZ;
                    CSD = JV;
                    CSE = CSA;
                    CSF = AUE;
                } else {
                    CSC = JV;
                    CSD = CRZ;
                    CSE = AUE;
                    CSF = CSA;
                }
                let CSG = if staged[236] != 0.0 || (if IJ <= JV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CTF;
                let CTG;
                if CSG != 0.0 {
                    CTF = JV;
                    CTG = AUE;
                } else {
                    let CSH = IY * AF;
                    let CSI = Lanes([0.0, CSH[0], CSH[1]]) + Lanes([CO, 0.0, 0.0]);
                    let CSJ = staged[238] * QC;
                    let CSK = (LL - Lanes([0.0, CQ, 0.0, 0.0])) * CSJ;
                    let CSL = ((((-IX) - staged[237]) + CN) + (CSJ * ((KH - CP) - staged[239]))) / CRF;
                    let CSM = (Lanes([0.0, CSI[0], 0.0, CSI[1], CSI[2]]) + Lanes([CSK[0], CSK[1], CSK[2], CSK[3], 0.0])) / CRF;
                    let CSN = CSM * CSL;
                    let CSO = ((CSL * CSL) + 4e-4f64).sqrt();
                    let CSP = Q * (CSL + CSO);
                    let CSQ = (CSM + ((CSN + CSN) * (P / (O * CSO)))) * Q;
                    let CSR = CSP + ARJ;
                    let CSS = IJ / CSR;
                    let CST = if CSP >= AY { CSP } else { AY };
                    let CSV = CSU * (CST.ln());
                    let CSW = rspice_limited_exp(CSV);
                    let CSY = ((-JF) * CSX) * AVU;
                    let CSZ = CSY * CSW;
                    let CTA = (((JG * AF) * CSX) * AVU) * CSW;
                    let CTB = -CSS;
                    let CTC = rspice_limited_exp(CTB);
                    let CTD = CSZ * CTC;
                    let CTE = ((Lanes([0.0, 0.0, CTA[0], CTA[1], 0.0]) + (((((CSQ * (if CSP >= AY { 1.0 } else { 0.0 })) * (P / CST)) * CSU) * (rspice_limited_exp_derivative(CSV))) * CSY)) * CTC) + (((((Lanes([0.0, IK, 0.0, 0.0, 0.0]) - (CSQ * CSS)) / CSR) * AF) * (rspice_limited_exp_derivative(CTB))) * CSZ);
                    CTF = CTD;
                    CTG = CTE;
                }
                let CTH;
                let CTI;
                let CTJ;
                let CTK;
                if CSB != 0.0 {
                    CTH = CSC;
                    CTI = CTF;
                    CTJ = CSE;
                    CTK = CTG;
                } else {
                    CTH = CTF;
                    CTI = CSD;
                    CTJ = CTG;
                    CTK = CSF;
                }
                CQV = CTH;
                CQW = CTI;
                CQX = CTJ;
                CQY = CTK;
            } else {
                CQV = JV;
                CQW = JV;
                CQX = AUE;
                CQY = AUE;
            }
            let CQZ = AUI / BXS;
            if CRA != 0.0 {
                let CTV;
                if CTL != 0.0 {
                    CTV = JV;
                } else {
                    let CTT = CTS * ((if (((BZZ / CTS) + parameters[287]) / CQZ) >= AY { (((BZZ / CTS) + parameters[287]) / CQZ) } else { AY }).ln());
                    let CTU = if CTT < JV { 1.0 } else { 0.0 };
                    let CTX = if CTU != 0.0 {
                        JV
                    } else {
                        CTT
                    };
                    CTV = CTX;
                }
                let CUC = if CTW != 0.0 {
                    let CTZ = (staged[245] / (DA + ((BUS / staged[243]).powf(staged[244])))) / CTY;
                    let CUA = CTZ - DA;
                    let CUB = CTY * (Q * ((CTZ + DA) + (((CUA * CUA) + staged[246]).sqrt())));
                    CUB
                } else {
                    CTY
                };
                let CUE = (CGC * ARC) / CUD;
                let CUF = (CGC * BTX) / CUD;
                let CUG = (Y / CUD) * (CGC + staged[248]);
                let CUH = CUF + CUG;
                let CUJ = if ((((((CUC * CUD) * Y) / ((staged[251] * CUG) * CUG)) * CFD) * CFD) + ((((((4.112842231783458e-57f64 * Y) * (CFD.abs())) * BXS) / ((1e10f64 * CGC) * staged[247])) * (((CUC * ((if ((CUE + CUG) / CUH) >= AY { ((CUE + CUG) / CUH) } else { AY }).ln())) + (CUI * (CUE - CUF))) + (staged[249] * ((CUE * CUE) - (CUF * CUF))))) + (((((((CUD * Y) * CFD) * CFD) / staged[250]) * CTV) * ((CUC + (CUI * CUF)) + ((parameters[290] * CUF) * CUF))) / (CUH * CUH)))) > JV { 1.0 } else { 0.0 };
            } else {
            }
            let CTN = CTM * CGF;
            let CTO = CGG * CTM;
            let CTP = AVD * CGK;
            let CTQ = CGL * AVD;
            let CTR = if KK > JV { 1.0 } else { 0.0 };
            let CUO;
            let CUP;
            if CTR != 0.0 {
                let CUK = AVD * CGI;
                let CUL = CGJ * AVD;
                CUO = CUK;
                CUP = CUL;
            } else {
                let CUM = AVD * CGN;
                let CUN = CGO * AVD;
                CUO = CUM;
                CUP = CUN;
            }
            let CUQ = AVD * CIC;
            let CUR = CID * AVD;
            let CUS = AVD * CIE;
            let CUT = CIF * AVD;
            let CVA;
            let CVB;
            if CUU != 0.0 {
                let CUW = CUV * Y;
                let CUY = CUX * ((BXU * BTZ) + (CUW * BXU));
                let CUZ = (((BXV * BTZ) + (BUA * BXU)) + (Lanes([0.0, ((Z * CUV) * BXU), 0.0, 0.0, 0.0]) + (BXV * CUW))) * CUX;
                CVA = CUY;
                CVB = CUZ;
            } else {
                CVA = JV;
                CVB = AUE;
            }
            let CVC = AVD * CQV;
            let CVD = CQX * AVD;
            let CVE = AVD * CQW;
            let CVF = CQY * AVD;
            let CVG = AVD * COL;
            let CVH = COP * AVD;
            let CVI = AVD * COM;
            let CVJ = COQ * AVD;
            let CVK = AVD * CON;
            let CVL = COR * AVD;
            let CVM = AVD * COO;
            let CVN = COS * AVD;
            let CWN;
            let CWO;
            let CWP;
            let CWQ;
            let CWR;
            let CWS;
            let CWT;
            let CWU;
            let CWV;
            let CWW;
            let CWX;
            let CWY;
            let CWZ;
            let CXA;
            let CXB;
            let CXC;
            let CXD;
            let CXE;
            let CXF;
            let CXG;
            if CTR != 0.0 {
                let CVP = ctx.simparam_or("gmin", CVO);
                let CVQ = JE * CVP;
                let CVR = (CB * CFD) + (CVP * JA);
                let CVS = (CFE * CB) + Lanes([0.0, 0.0, CVQ[0], CVQ[1], 0.0]);
                let CVT = CB * (CVC + CIO);
                let CVU = (CVD + CIP) * CB;
                let CVV = CB * CVE;
                let CVW = CVF * CB;
                let CVX = CB * (CVI + CVK);
                let CVY = (CVJ + CVL) * CB;
                let CVZ = CB * (CVG + CVM);
                let CWA = (CVH + CVN) * CB;
                CWN = CVR;
                CWO = CVT;
                CWP = CVV;
                CWQ = CVX;
                CWR = CVZ;
                CWS = JV;
                CWT = JV;
                CWU = JV;
                CWV = JV;
                CWW = JV;
                CWX = CVS;
                CWY = CVU;
                CWZ = CVW;
                CXA = CVY;
                CXB = CWA;
                CXC = AUE;
                CXD = AUE;
                CXE = AUE;
                CXF = AUE;
                CXG = AUE;
            } else {
                let CWB = ctx.simparam_or("gmin", CVO);
                let CWC = (JD - JC) * CWB;
                let CWD = (CB * CFD) + (CWB * (IU - IZ));
                let CWE = (CFE * CB) + Lanes([0.0, 0.0, CWC[0], CWC[1], 0.0]);
                let CWF = CB * (CVC + CIO);
                let CWG = (CVD + CIP) * CB;
                let CWH = CB * CVE;
                let CWI = CVF * CB;
                let CWJ = CB * (CVI + CVK);
                let CWK = (CVJ + CVL) * CB;
                let CWL = CB * (CVG + CVM);
                let CWM = (CVH + CVN) * CB;
                CWN = JV;
                CWO = JV;
                CWP = JV;
                CWQ = JV;
                CWR = JV;
                CWS = CWD;
                CWT = CWF;
                CWU = CWH;
                CWV = CWJ;
                CWW = CWL;
                CWX = AUE;
                CWY = AUE;
                CWZ = AUE;
                CXA = AUE;
                CXB = AUE;
                CXC = CWE;
                CXD = CWG;
                CXE = CWI;
                CXF = CWK;
                CXG = CWM;
            }
            let CXH = CB * CLA;
            let CXI = CLC * CB;
            let CXJ = CB * CLD;
            let CXK = CLF * CB;
            let CXM = CB * ddt(22330, CUO);
            let CXN = (CUP * CXL) * CB;
            let CXO = CB * CUO;
            let CXP = CUP * CB;
            let CXQ = ddt(22333, CTN);
            let CXR = CTO * CXL;
            let CXS = CB * ddt(22336, CTP);
            let CXT = (CTQ * CXL) * CB;
            let CXU = CB * CTP;
            let CXV = CTQ * CB;
            let CXW = ddt(22339, CUQ);
            let CXX = CUR * CXL;
            let CXY = ddt(22341, CUS);
            let CXZ = CUT * CXL;
            let CYA = CB * ddt(22344, CIH);
            let CYB = (CII * CXL) * CB;
            let CYC = CB * CIH;
            let CYD = CII * CB;
            let CYE = CB * ddt(22348, CIK);
            let CYF = (CIL * CXL) * CB;
            let CYG = CB * CIK;
            let CYH = CIL * CB;
            let CZB;
            let CZC;
            let CZD;
            let CZE;
            if CYI != 0.0 {
                CZB = JV;
                CZC = JV;
                CZD = CYJ;
                CZE = CYK;
            } else {
                let CYL = DA / CET;
                let CYM = DA / CEU;
                let CYO = CYN - IZ;
                let CYQ = CYO * CYL;
                let CYR = (Lanes([CYP, 0.0]) - Lanes([0.0, JB])) * CYL;
                let CYS = (((CEW * CYL) * AF) / CET) * CYO;
                let CYT = Lanes([CYR[0], 0.0, 0.0, CYR[1], 0.0]) + Lanes([0.0, CYS[0], CYS[1], CYS[2], CYS[3]]);
                let CYV = CYU - IU;
                let CYX = CYV * CYM;
                let CYY = (Lanes([CYW, 0.0]) - Lanes([0.0, IW])) * CYM;
                let CYZ = (((CEX * CYM) * AF) / CEU) * CYV;
                let CZA = Lanes([CYY[0], 0.0, 0.0, CYY[1], 0.0]) + Lanes([0.0, CYZ[0], CYZ[1], CYZ[2], CYZ[3]]);
                CZB = CYQ;
                CZC = CYX;
                CZD = CYT;
                CZE = CZA;
            }
            let CZL;
            let CZM;
            if CUU != 0.0 {
                let CZF = KP - IT;
                let CZG = CZF * CVA;
                let CZH = (Lanes([KR, 0.0]) - Lanes([0.0, IV])) * CVA;
                let CZI = CVB * CZF;
                let CZJ = Lanes([0.0, 0.0, 0.0, 0.0, CZH[0], CZH[1]]) + Lanes([CZI[0], CZI[1], CZI[2], CZI[3], 0.0, CZI[4]]);
                CZL = CZG;
                CZM = CZJ;
            } else {
                CZL = JV;
                CZM = CZK;
            }
            let CZT;
            let CZU;
            if CZN != 0.0 {
                CZT = JV;
                CZU = CZO;
            } else {
                let CZR = (node_potentials[1] - KP) * CZQ;
                let CZS = (Lanes([CZP, 0.0]) - Lanes([0.0, KR])) * CZQ;
                CZT = CZR;
                CZU = CZS;
            }
            let CZV;
            let CZW;
            let CZX;
            let CZY;
            if CLG != 0.0 {
                let DAD;
                let DAE;
                let DAF;
                let DAG;
                if CTR != 0.0 {
                    DAD = CZZ;
                    DAE = DAA;
                    DAF = JV;
                    DAG = JV;
                } else {
                    DAD = JV;
                    DAE = JV;
                    DAF = DAB;
                    DAG = DAC;
                }
                CZV = DAD;
                CZW = DAE;
                CZX = DAF;
                CZY = DAG;
            } else {
                CZV = JV;
                CZW = JV;
                CZX = JV;
                CZY = JV;
            }
            let DAJ;
            let DAK;
            let DAL;
            let DAM;
            let DAN;
            let DAO;
            let DAP;
            let DAQ;
            let DAR;
            let DAS;
            if B != 0.0 {
                let DBR;
                let DBS;
                let DBT;
                let DBU;
                if DAH != 0.0 {
                    let DAT = CB * KK;
                    let DAU = DAT * JA;
                    let DAV = (JE * DAT) * CFD;
                    let DAW = Lanes([0.0, 0.0, DAV[0], DAV[1], 0.0]) + (CFE * DAU);
                    let DAX = CYN - IZ;
                    let DAY = (Lanes([CYP, 0.0]) - Lanes([0.0, JB])) * DAX;
                    let DAZ = DAY + DAY;
                    let DBA = (DAX * DAX) / CET;
                    let DBB = CEW * DBA;
                    let DBC = (Lanes([DAZ[0], 0.0, 0.0, DAZ[1], 0.0]) - Lanes([0.0, DBB[0], DBB[1], DBB[2], DBB[3]])) / CET;
                    let DBD = Lanes([0.0, DAW[0], DAW[1], DAW[2], DAW[3], DAW[4]]) + Lanes([DBC[0], DBC[1], DBC[2], DBC[3], 0.0, DBC[4]]);
                    let DBE = CYU - IU;
                    let DBF = (Lanes([CYW, 0.0]) - Lanes([0.0, IW])) * DBE;
                    let DBG = DBF + DBF;
                    let DBH = (DBE * DBE) / CEU;
                    let DBI = CEX * DBH;
                    let DBJ = (Lanes([DBG[0], 0.0, 0.0, DBG[1], 0.0]) - Lanes([0.0, DBI[0], DBI[1], DBI[2], DBI[3]])) / CEU;
                    let DBK = -(((DAU * CFD) + DBA) + DBH);
                    let DBL = (Lanes([DBD[0], 0.0, DBD[1], DBD[2], DBD[3], DBD[4], DBD[5]]) + Lanes([0.0, DBJ[0], DBJ[1], DBJ[2], 0.0, DBJ[3], DBJ[4]])) * AF;
                    DBR = DBK;
                    DBS = JV;
                    DBT = DBL;
                    DBU = AUE;
                } else {
                    let DBM = CB * KK;
                    let DBN = DBM * JA;
                    let DBO = (JE * DBM) * CFD;
                    let DBP = -(DBN * CFD);
                    let DBQ = (Lanes([0.0, 0.0, DBO[0], DBO[1], 0.0]) + (CFE * DBN)) * AF;
                    DBR = JV;
                    DBS = DBP;
                    DBT = DAI;
                    DBU = DBQ;
                }
                let DBW = C * DBV;
                let DBX = E * DBV;
                let DBZ = C * DBY;
                let DCA = E * DBY;
                let DCB = ddt(22518, DBZ);
                let DCC = DCA * CXL;
                DAJ = DBR;
                DAK = DBS;
                DAL = DBW;
                DAM = DCB;
                DAN = DBZ;
                DAO = DBT;
                DAP = DBU;
                DAQ = DBX;
                DAR = DCC;
                DAS = DCA;
            } else {
                DAJ = JV;
                DAK = JV;
                DAL = JV;
                DAM = JV;
                DAN = JV;
                DAO = DAI;
                DAP = AUE;
                DAQ = G;
                DAR = G;
                DAS = G;
            }
            let DCD = CTO[2];
            let DCE = CTO[4];
            let DCF = CTO[3];
            let DCG = CTO[0];
            let DCH = CXV[2];
            let DCI = CXV[4];
            let DCJ = CXV[3];
            let DCK = CXV[0];
            let DCL = CXP[2];
            let DCM = CXP[4];
            let DCN = CXP[3];
            let DCO = CXP[0];
            let DCP = CWX[0];
            let DCQ = CWX[1];
            let DCR = CWX[2];
            let DCS = CWX[3];
            let DCT = CWX[4];
            let DCU = CWY[0];
            let DCV = CWY[1];
            let DCW = CWY[2];
            let DCX = CWY[3];
            let DCY = CWY[4];
            let DCZ = CWZ[0];
            let DDA = CWZ[1];
            let DDB = CWZ[2];
            let DDC = CWZ[3];
            let DDD = CWZ[4];
            let DDE = CXA[0];
            let DDF = CXA[1];
            let DDG = CXA[2];
            let DDH = CXA[3];
            let DDI = CXA[4];
            let DDJ = CXB[0];
            let DDK = CXB[1];
            let DDL = CXB[2];
            let DDM = CXB[3];
            let DDN = CXB[4];
            let DDO = CXC[0];
            let DDP = CXC[1];
            let DDQ = CXC[2];
            let DDR = CXC[3];
            let DDS = CXC[4];
            let DDT = CXD[0];
            let DDU = CXD[1];
            let DDV = CXD[2];
            let DDW = CXD[3];
            let DDX = CXD[4];
            let DDY = CXE[0];
            let DDZ = CXE[1];
            let DEA = CXE[2];
            let DEB = CXE[3];
            let DEC = CXE[4];
            let DED = CXF[0];
            let DEE = CXF[1];
            let DEF = CXF[2];
            let DEG = CXF[3];
            let DEH = CXF[4];
            let DEI = CXG[0];
            let DEJ = CXG[1];
            let DEK = CXG[2];
            let DEL = CXG[3];
            let DEM = CXG[4];
            let DEN = CXI[0];
            let DEO = CXI[1];
            let DEP = CXI[2];
            let DEQ = CXI[3];
            let DER = CXI[4];
            let DES = CXK[0];
            let DET = CXK[1];
            let DEU = CXK[2];
            let DEV = CXK[3];
            let DEW = CXK[4];
            let DEX = CXN[0];
            let DEY = CXN[1];
            let DEZ = CXN[2];
            let DFA = CXN[3];
            let DFB = CXN[4];
            let DFC = CXR[0];
            let DFD = CXR[1];
            let DFE = CXR[2];
            let DFF = CXR[3];
            let DFG = CXR[4];
            let DFH = CXT[0];
            let DFI = CXT[1];
            let DFJ = CXT[2];
            let DFK = CXT[3];
            let DFL = CXT[4];
            let DFM = CXX[0];
            let DFN = CXX[1];
            let DFO = CXX[2];
            let DFP = CXX[3];
            let DFQ = CXZ[0];
            let DFR = CXZ[1];
            let DFS = CXZ[2];
            let DFT = CXZ[3];
            let DFU = CYB[0];
            let DFV = CYB[1];
            let DFW = CYF[0];
            let DFX = CYF[1];
            let DFY = CZD[0];
            let DFZ = CZD[1];
            let DGA = CZD[2];
            let DGB = CZD[3];
            let DGC = CZD[4];
            let DGD = CZE[0];
            let DGE = CZE[1];
            let DGF = CZE[2];
            let DGG = CZE[3];
            let DGH = CZE[4];
            let DGI = CZM[0];
            let DGJ = CZM[1];
            let DGK = CZM[2];
            let DGL = CZM[3];
            let DGM = CZM[4];
            let DGN = CZM[5];
            let DGO = CZU[0];
            let DGP = CZU[1];
            let DGQ = DAO[0];
            let DGR = DAO[1];
            let DGS = DAO[2];
            let DGT = DAO[3];
            let DGU = DAO[4];
            let DGV = DAO[5];
            let DGW = DAO[6];
            let DGX = DAP[0];
            let DGY = DAP[1];
            let DGZ = DAP[2];
            let DHA = DAP[3];
            let DHB = DAP[4];
            let DHC = DAQ;
            let DHD = DAR;
            let DHE = CXP[1];
            let DHF = CTO[1];
            let DHG = CXV[1];
            let DHH = CUR[0];
            let DHI = CUR[1];
            let DHJ = CUR[2];
            let DHK = CUR[3];
            let DHL = CUT[0];
            let DHM = CUT[1];
            let DHN = CUT[2];
            let DHO = CUT[3];
            let DHP = CYD[0];
            let DHQ = CYD[1];
            let DHR = CYH[0];
            let DHS = CYH[1];
            let DHT = DAS;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (CWN),
            [3, 4, 5, 6, 8],
            [DCP, DCQ, DCR, DCS, DCT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (CWO),
            [3, 4, 5, 6, 8],
            [DCU, DCV, DCW, DCX, DCY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (CWP),
            [3, 4, 5, 6, 8],
            [DCZ, DDA, DDB, DDC, DDD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (CWQ),
            [3, 4, 5, 6, 8],
            [DDE, DDF, DDG, DDH, DDI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (CWR),
            [3, 4, 5, 6, 8],
            [DDJ, DDK, DDL, DDM, DDN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (CWS),
            [3, 4, 5, 6, 8],
            [DDO, DDP, DDQ, DDR, DDS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (CWT),
            [3, 4, 5, 6, 8],
            [DDT, DDU, DDV, DDW, DDX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (CWU),
            [3, 4, 5, 6, 8],
            [DDY, DDZ, DEA, DEB, DEC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (CWV),
            [3, 4, 5, 6, 8],
            [DED, DEE, DEF, DEG, DEH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (CWW),
            [3, 4, 5, 6, 8],
            [DEI, DEJ, DEK, DEL, DEM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (CXH),
            [3, 4, 5, 6, 8],
            [DEN, DEO, DEP, DEQ, DER],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (CXJ),
            [3, 4, 5, 6, 8],
            [DES, DET, DEU, DEV, DEW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (CXM),
            [3, 4, 5, 6, 8],
            [DEX, DEY, DEZ, DFA, DFB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (CXQ),
            [3, 4, 5, 6, 8],
            [DFC, DFD, DFE, DFF, DFG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(6),
            multiplicity * (CXS),
            [3, 4, 5, 6, 8],
            [DFH, DFI, DFJ, DFK, DFL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (CXW),
            [3, 4, 6, 7],
            [DFM, DFN, DFO, DFP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(5),
            multiplicity * (CXY),
            [3, 4, 5, 7],
            [DFQ, DFR, DFS, DFT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(6),
            Some(3),
            multiplicity * (CYA),
            [3, 6],
            [DFU, DFV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(3),
            multiplicity * (CYE),
            [3, 5],
            [DFW, DFX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(5), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[349],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), Some(6), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[350],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(5),
            multiplicity * (CZB),
            [0, 3, 4, 5, 8],
            [DFY, DFZ, DGA, DGB, DGC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(6),
            multiplicity * (CZC),
            [2, 3, 4, 6, 8],
            [DGD, DGE, DGF, DGG, DGH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(5),
            multiplicity * (staged[351]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(6),
            multiplicity * (staged[352]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * (CZL),
            [3, 4, 5, 6, 7, 8],
            [DGI, DGJ, DGK, DGL, DGM, DGN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(8), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[353],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(7), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[354],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(7),
            multiplicity * (CZT),
            [1, 7],
            [DGO, DGP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(7),
            multiplicity * (staged[355]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (DHU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (DHV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (CZV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (CZW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (CZX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (CZY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (staged[356]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(5),
            multiplicity * (staged[357]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (DAJ),
            [0, 2, 3, 4, 5, 6, 8],
            [DGQ, DGR, DGS, DGT, DGU, DGV, DGW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (DAK),
            [3, 4, 5, 6, 8],
            [DGX, DGY, DGZ, DHA, DHB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (DAL),
            [4],
            [DHC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (DAM),
            [4],
            [DHD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[358],
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = CWN;
        self.canonical_reactive[1] = CWO;
        self.canonical_reactive[2] = CWP;
        self.canonical_reactive[3] = CWQ;
        self.canonical_reactive[4] = CWR;
        self.canonical_reactive[5] = CWS;
        self.canonical_reactive[6] = CWT;
        self.canonical_reactive[7] = CWU;
        self.canonical_reactive[8] = CWV;
        self.canonical_reactive[9] = CWW;
        self.canonical_reactive[10] = CXH;
        self.canonical_reactive[11] = CXJ;
        self.canonical_reactive[12] = CXO;
        self.canonical_reactive[13] = DCO;
        self.canonical_reactive[14] = DHE;
        self.canonical_reactive[15] = DCL;
        self.canonical_reactive[16] = DCN;
        self.canonical_reactive[17] = DCM;
        self.canonical_reactive[18] = CTN;
        self.canonical_reactive[19] = DCG;
        self.canonical_reactive[20] = DHF;
        self.canonical_reactive[21] = DCD;
        self.canonical_reactive[22] = DCF;
        self.canonical_reactive[23] = DCE;
        self.canonical_reactive[24] = CXU;
        self.canonical_reactive[25] = DCK;
        self.canonical_reactive[26] = DHG;
        self.canonical_reactive[27] = DCH;
        self.canonical_reactive[28] = DCJ;
        self.canonical_reactive[29] = DCI;
        self.canonical_reactive[30] = CUQ;
        self.canonical_reactive[31] = DHH;
        self.canonical_reactive[32] = DHI;
        self.canonical_reactive[33] = DHJ;
        self.canonical_reactive[34] = DHK;
        self.canonical_reactive[35] = CUS;
        self.canonical_reactive[36] = DHL;
        self.canonical_reactive[37] = DHM;
        self.canonical_reactive[38] = DHN;
        self.canonical_reactive[39] = DHO;
        self.canonical_reactive[40] = CYC;
        self.canonical_reactive[41] = DHP;
        self.canonical_reactive[42] = DHQ;
        self.canonical_reactive[43] = CYG;
        self.canonical_reactive[44] = DHR;
        self.canonical_reactive[45] = DHS;
        self.canonical_reactive[46] = staged[349];
        self.canonical_reactive[47] = staged[350];
        self.canonical_reactive[48] = CZB;
        self.canonical_reactive[49] = CZC;
        self.canonical_reactive[50] = staged[351];
        self.canonical_reactive[51] = staged[352];
        self.canonical_reactive[52] = CZL;
        self.canonical_reactive[53] = staged[353];
        self.canonical_reactive[54] = staged[354];
        self.canonical_reactive[55] = CZT;
        self.canonical_reactive[56] = staged[355];
        self.canonical_reactive[57] = DHU;
        self.canonical_reactive[58] = DHV;
        self.canonical_reactive[59] = CZV;
        self.canonical_reactive[60] = CZW;
        self.canonical_reactive[61] = CZX;
        self.canonical_reactive[62] = CZY;
        self.canonical_reactive[63] = staged[356];
        self.canonical_reactive[64] = staged[357];
        self.canonical_reactive[65] = DAJ;
        self.canonical_reactive[66] = DAK;
        self.canonical_reactive[67] = DAL;
        self.canonical_reactive[68] = DAN;
        self.canonical_reactive[69] = DHT;
        self.canonical_reactive[70] = staged[358];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6, 8],
            &[cached[13], cached[14], cached[15], cached[16], cached[17]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[3, 4, 5, 6, 8],
            &[cached[19], cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(6),
            &[3, 4, 5, 6, 8],
            &[cached[25], cached[26], cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(6),
            &[3, 4, 6, 7],
            &[cached[31], cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 4, 5, 7],
            &[cached[36], cached[37], cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(3),
            &[3, 6],
            &[cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(3),
            &[3, 5],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[69]],
            &[],
            &[],
            multiplicity,
        );
    }

}
