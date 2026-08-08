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
        let mut key = Vec::with_capacity(628);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[171] = values[0];
        self.canonical_staged[172] = values[1];
        self.canonical_staged[173] = values[2];
        self.canonical_staged[174] = values[3];
        self.canonical_staged[175] = values[4];
        self.canonical_staged[176] = values[5];
        self.canonical_staged[177] = values[6];
        self.canonical_staged[178] = values[7];
        self.canonical_staged[179] = values[8];
        self.canonical_staged[180] = values[9];
        self.canonical_staged[31] = values[10];
        self.canonical_staged[9] = values[11];
        self.canonical_staged[32] = values[12];
        self.canonical_staged[16] = values[13];
        self.canonical_staged[6] = values[14];
        self.canonical_staged[11] = values[15];
        self.canonical_staged[3] = values[16];
        self.canonical_staged[4] = values[17];
        self.canonical_staged[131] = values[18];
        self.canonical_staged[12] = values[19];
        self.canonical_staged[129] = values[20];
        self.canonical_staged[5] = values[21];
        self.canonical_staged[8] = values[22];
        self.canonical_staged[70] = values[23];
        self.canonical_staged[0] = values[24];
        self.canonical_staged[130] = values[25];
        self.canonical_staged[25] = values[26];
        self.canonical_staged[1] = values[27];
        self.canonical_staged[2] = values[28];
        self.canonical_staged[7] = values[29];
        self.canonical_staged[10] = values[30];
        self.canonical_staged[13] = values[31];
        self.canonical_staged[181] = values[32];
        self.canonical_staged[14] = values[33];
        self.canonical_staged[15] = values[34];
        self.canonical_staged[17] = values[35];
        self.canonical_staged[18] = values[36];
        self.canonical_staged[19] = values[37];
        self.canonical_staged[20] = values[38];
        self.canonical_staged[21] = values[39];
        self.canonical_staged[22] = values[40];
        self.canonical_staged[190] = values[41];
        self.canonical_staged[192] = values[42];
        self.canonical_staged[193] = values[43];
        self.canonical_staged[194] = values[44];
        self.canonical_staged[26] = values[45];
        self.canonical_staged[24] = values[46];
        self.canonical_staged[35] = values[47];
        self.canonical_staged[38] = values[48];
        self.canonical_staged[39] = values[49];
        self.canonical_staged[197] = values[50];
        self.canonical_staged[198] = values[51];
        self.canonical_staged[40] = values[52];
        self.canonical_staged[199] = values[53];
        self.canonical_staged[41] = values[54];
        self.canonical_staged[42] = values[55];
        self.canonical_staged[53] = values[56];
        self.canonical_staged[50] = values[57];
        self.canonical_staged[81] = values[58];
        self.canonical_staged[77] = values[59];
        self.canonical_staged[98] = values[60];
        self.canonical_staged[83] = values[61];
        self.canonical_staged[78] = values[62];
        self.canonical_staged[206] = values[63];
        self.canonical_staged[48] = values[64];
        self.canonical_staged[55] = values[65];
        self.canonical_staged[207] = values[66];
        self.canonical_staged[61] = values[67];
        self.canonical_staged[63] = values[68];
        self.canonical_staged[208] = values[69];
        self.canonical_staged[210] = values[70];
        self.canonical_staged[67] = values[71];
        self.canonical_staged[68] = values[72];
        self.canonical_staged[211] = values[73];
        self.canonical_staged[212] = values[74];
        self.canonical_staged[75] = values[75];
        self.canonical_staged[82] = values[76];
        self.canonical_staged[85] = values[77];
        self.canonical_staged[86] = values[78];
        self.canonical_staged[101] = values[79];
        self.canonical_staged[105] = values[80];
        self.canonical_staged[109] = values[81];
        self.canonical_staged[113] = values[82];
        self.canonical_staged[213] = values[83];
        self.canonical_staged[214] = values[84];
        self.canonical_staged[118] = values[85];
        self.canonical_staged[215] = values[86];
        self.canonical_staged[216] = values[87];
        self.canonical_staged[119] = values[88];
        self.canonical_staged[217] = values[89];
        self.canonical_staged[218] = values[90];
        self.canonical_staged[120] = values[91];
        self.canonical_staged[219] = values[92];
        self.canonical_staged[220] = values[93];
        self.canonical_staged[121] = values[94];
        self.canonical_staged[221] = values[95];
        self.canonical_staged[222] = values[96];
        self.canonical_staged[142] = values[97];
        self.canonical_staged[223] = values[98];
        self.canonical_staged[140] = values[99];
        self.canonical_staged[135] = values[100];
        self.canonical_staged[136] = values[101];
        self.canonical_staged[224] = values[102];
        self.canonical_staged[225] = values[103];
        self.canonical_staged[149] = values[104];
        self.canonical_staged[144] = values[105];
        self.canonical_staged[145] = values[106];
        self.canonical_staged[226] = values[107];
        self.canonical_staged[227] = values[108];
        self.canonical_staged[228] = values[109];
        self.canonical_staged[229] = values[110];
        self.canonical_staged[230] = values[111];
        self.canonical_staged[231] = values[112];
        self.canonical_staged[232] = values[113];
        self.canonical_staged[233] = values[114];
        self.canonical_staged[234] = values[115];
        self.canonical_staged[235] = values[116];
        self.canonical_staged[236] = values[117];
        self.canonical_staged[155] = values[118];
        self.canonical_staged[158] = values[119];
        self.canonical_staged[159] = values[120];
        self.canonical_staged[161] = values[121];
        self.canonical_staged[163] = values[122];
        self.canonical_staged[164] = values[123];
        self.canonical_staged[165] = values[124];
        self.canonical_staged[166] = values[125];
        self.canonical_staged[167] = values[126];
        self.canonical_staged[168] = values[127];
        self.canonical_staged[169] = values[128];
        self.canonical_staged[170] = values[129];
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
                let A = 0e0f64;
                let E = parameters[17];
                let F = 1e0f64;
                let J = parameters[18];
                let V = if parameter_given[177] { 1.0 } else { 0.0 };
                let W = parameters[177];
                let X = parameters[227];
                let Y = parameters[230];
                let AF = 1.0f64;
                let AH = 1e-2f64;
                let AJ = 1e-6f64;
                let AP = parameters[237];
                let AX = 1e4f64;
                let BC = 2e0f64;
                let BF = 1.0f64;
                let BG = 0.0f64;
                let BK = 1.0f64;
                let BM = 0.0f64;
                let BO = 3e0f64;
                let BP = 0.0f64;
                let BR = 4e0f64;
                let BY = 4e25f64;
                let BZ = -4e25f64;
                let CB = 1.0f64;
                let CC = 4e25f64;
                let CD = -4e25f64;
                let CF = 1e-3f64;
                let CL = parameters[32];
                let CO = parameters[58];
                let CV = parameters[25];
                let CY = parameters[24];
                let CZ = parameters[31];
                let DF = 1e0f64;
                let DI = parameters[38];
                let DV = 3.453133e-11f64;
                let DW = parameters[226];
                let DZ = parameters[229];
                let EC = 1.034943e-10f64;
                let EH = parameters[254];
                let EK = 1.0f64;
                let EL = 2e-3f64;
                let EM = -2e-3f64;
                let ES = parameters[72];
                let EZ = 1.0f64;
                let FA = 2e-1f64;
                let FB = -2e-1f64;
                let FE = 0.0f64;
                let FH = 1e2f64;
                let FL = 1.17e1f64;
                let FQ = parameters[114];
                let GI = parameters[240];
                let GL = parameters[312];
                let GO = parameters[314];
                let GQ = parameters[313];
                let GS = parameters[322];
                let GW = parameters[320];
                let HE = 0e0f64;
                let HG = 0e0f64;
                let HI = 0e0f64;
                let HK = 0e0f64;
                let HM = 0e0f64;
                let HO = 0e0f64;
                let HR = 0e0f64;
                let HT = 0e0f64;
                let HU = 0e0f64;
                let mut oC = 0.0;
                let mut oH = 0.0;
                let mut oL = 0.0;
                let mut oO = 0.0;
                let mut oBV = 0.0;
                let mut oCN = 0.0;
                let mut oDN = 0.0;
                let mut oDP = 0.0;
                let mut oDS = 0.0;
                let mut oDU = 0.0;
                let mut oEP = 0.0;
                let mut oEQ = 0.0;
                let mut oEU = 0.0;
                let mut oEV = 0.0;
                let mut oFC = 0.0;
                let mut oFF = 0.0;
                let mut oFG = 0.0;
                let mut oFM = 0.0;
                let mut oFO = 0.0;
                let mut oFS = 0.0;
                let mut oFU = 0.0;
                let mut oFV = 0.0;
                let mut oFW = 0.0;
                let mut oFY = 0.0;
                let mut oFZ = 0.0;
                let mut oGA = 0.0;
                let mut oGC = 0.0;
                let mut oGD = 0.0;
                let mut oGE = 0.0;
                let mut oGF = 0.0;
                let mut oGG = 0.0;
                let mut oGJ = 0.0;
                let mut oGK = 0.0;
                let mut oGN = 0.0;
                let mut oGP = 0.0;
                let mut oGT = 0.0;
                let mut oGU = 0.0;
                let mut oGV = 0.0;
                let mut oGX = 0.0;
                let mut oGY = 0.0;
                let mut oGZ = 0.0;
                let mut oHA = 0.0;
                let mut oHB = 0.0;
                let mut oHC = 0.0;
                let B = if parameters[239] != A { 1.0 } else { 0.0 };
                let D;
                if B != 0.0 {
                    let C = if parameters[274] <= A { 1.0 } else { 0.0 };
                    oC = C;
                    let G = if C != 0.0 {
                        F
                    } else {
                        A
                    };
                    D = G;
                } else {
                    D = A;
                }
                let I;
                if E != 0.0 {
                    let H = if parameters[207] <= A { 1.0 } else { 0.0 };
                    oH = H;
                    let K = if H != 0.0 {
                        F
                    } else {
                        D
                    };
                    I = K;
                } else {
                    I = D;
                }
                let M;
                if J != 0.0 {
                    let L = if parameters[228] <= A { 1.0 } else { 0.0 };
                    oL = L;
                    let N = if L != 0.0 {
                        F
                    } else {
                        I
                    };
                    M = N;
                } else {
                    M = I;
                }
                let P;
                if J != 0.0 {
                    let O = if parameters[201] <= A { 1.0 } else { 0.0 };
                    oO = O;
                    let R = if O != 0.0 {
                        F
                    } else {
                        M
                    };
                    P = R;
                } else {
                    P = M;
                }
                let Q = if (if parameters[165] == A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[167] < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let S = if Q != 0.0 {
                    F
                } else {
                    P
                };
                let T = if (if parameters[162] == A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[164] < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let U = if T != 0.0 {
                    F
                } else {
                    S
                };
                let AA = if V != 0.0 {
                    W
                } else {
                    let Z = 5e9f64 / (X * Y);
                    Z
                };
                let AB = if (if AA < 2.1e0f64 { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                let AG;
                if AB != 0.0 {
                    let AC = 2.1e0f64 - AA;
                    let AD = AC * AC;
                    let AE = (AD * AD) + 1.0000000000000005e-4f64;
                    let BI;
                    if AF != 0.0 {
                        let BL;
                        if BG != 0.0 {
                            BL = F;
                        } else {
                            let BN;
                            if BK != 0.0 {
                                BN = BC;
                            } else {
                                let BQ;
                                if BM != 0.0 {
                                    BQ = BO;
                                } else {
                                    let BS = if BP != 0.0 {
                                        BR
                                    } else {
                                        A
                                    };
                                    BQ = BS;
                                }
                                BN = BQ;
                            }
                            BL = BN;
                        }
                        let mut BT = 0.0;
                        let mut BU = 0.0;
                        BT = A;
                        BU = AE;
                        loop {
                            let BV = if BT < BL { 1.0 } else { 0.0 };
                            oBV = BV;
                            if BV == 0.0 {
                                break;
                            }
                            let BW = BU.sqrt();
                            let BX = BT + F;
                            BT = BX;
                            BU = BW;
                        }
                        BI = BU;
                    } else {
                        let BH = AE.powf(2.5e-1f64);
                        BI = BH;
                    }
                    let BJ = 2.1e0f64 - ((AC * 1e-1f64) * (F / (BI + 1e-50f64)));
                    AG = BJ;
                } else {
                    AG = AA;
                }
                let AI = parameters[34] * AH;
                let AK = parameters[59] / AJ;
                let AL = parameters[101] * AH;
                let AM = parameters[192] / AJ;
                let AN = Y / AJ;
                let AO = parameters[231] / AJ;
                let AQ = AP * AH;
                let AR = parameters[238] / AH;
                let AS = parameters[40] / AJ;
                let AT = parameters[236] / AJ;
                let AU = parameters[197] / AH;
                let AV = parameters[306] / AJ;
                let AW = parameters[307] / AJ;
                let AY = parameters[189] * AX;
                let AZ = parameters[147] / AJ;
                let BA = parameters[196] / 1e1f64;
                let BB = parameters[222] + 2.7315e2f64;
                let BD = BC * parameters[41];
                let BE = BC * parameters[42];
                let CA = if BF != 0.0 {
                    BY
                } else {
                    BZ
                };
                let CE = if CB != 0.0 {
                    CC
                } else {
                    CD
                };
                let CG = AO * CF;
                let CH = (BR * AT) * CG;
                let CI = if CH > A { 1.0 } else { 0.0 };
                let CK = if CI != 0.0 {
                    CH
                } else {
                    let CJ = -CH;
                    CJ
                };
                if CL != 0.0 {
                    let CN = if CI != 0.0 {
                        CH
                    } else {
                        let CM = -CH;
                        CM
                    };
                    oCN = CN;
                } else {
                }
                let CP = if CO <= A { 1.0 } else { 0.0 };
                let CQ = -parameters[242];
                let CR = -parameters[244];
                let CS = -parameters[247];
                let CT = BC * CO;
                let CU = if CO > A { 1.0 } else { 0.0 };
                let CW = if CV == F { 1.0 } else { 0.0 };
                let CX = if parameters[28] != 0.0 && (if AP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DA = if CZ >= 5e0f64 { 1.0 } else { 0.0 };
                let DB = if CZ >= 6e0f64 { 1.0 } else { 0.0 };
                let DC = parameters[37] - (BB * (9.025e-5f64 + (BB * 1e-7f64)));
                let DD = BB * BB;
                let DE = 1.6021918e-19f64 / (1.3806226e-23f64 * BB);
                let DG = parameters[202] - DF;
                let DH = (DC / BC) * DE;
                let DJ = DI / (parameters[251] + parameters[252]);
                let DK = ((DI * CF) + 2.2204460492503132e-17f64).abs();
                let DL = if DI > A { 1.0 } else { 0.0 };
                if DL != 0.0 {
                    let DM = (BR * DI) * DK;
                    let DN = if DM > A { 1.0 } else { 0.0 };
                    oDN = DN;
                    let DS = if DN != 0.0 {
                        DM
                    } else {
                        let DR = -DM;
                        DR
                    };
                    oDS = DS;
                } else {
                    let DO = (BR * DI) * DK;
                    let DP = if DO > A { 1.0 } else { 0.0 };
                    oDP = DP;
                    let DU = if DP != 0.0 {
                        DO
                    } else {
                        let DT = -DO;
                        DT
                    };
                    oDU = DU;
                }
                let DQ = -parameters[49];
                let DX = DV / DW;
                let DY = DW / DV;
                let EA = DV / DZ;
                let EB = DZ / DV;
                let ED = EC / X;
                let EE = F / ED;
                let EF = EB + EE;
                let EG = parameters[255] * 5e-1f64;
                let EI = if EH > EG { 1.0 } else { 0.0 };
                let EJ = if EI != 0.0 {
                    EG
                } else {
                    EH
                };
                let EN = if EK != 0.0 {
                    EL
                } else {
                    EM
                };
                let EO = if CO != A { 1.0 } else { 0.0 };
                if EO != 0.0 {
                    let EP = (BC * X) / (CO * CO);
                    oEP = EP;
                    let EQ = parameters[68] / CO;
                    oEQ = EQ;
                } else {
                }
                let ER = if parameters[297] != A { 1.0 } else { 0.0 };
                let ET = if ES > A { 1.0 } else { 0.0 };
                if ET != 0.0 {
                    let EU = BC * parameters[74];
                    oEU = EU;
                    let EV = ES * X;
                    oEV = EV;
                } else {
                }
                let EW = if parameters[75] == A { 1.0 } else { 0.0 };
                let EX = if EW != 0.0 {
                    A
                } else {
                    F
                };
                let EY = if EX == A { 1.0 } else { 0.0 };
                if EY != 0.0 {
                } else {
                    let FC = if EZ != 0.0 {
                        FA
                    } else {
                        FB
                    };
                    oFC = FC;
                }
                let FD = 9.9e-1f64 * X;
                if FE != 0.0 {
                } else {
                    let FF = X / EC;
                    oFF = FF;
                    let FG = F / EA;
                    oFG = FG;
                }
                let FI = DZ * FH;
                let FJ = parameters[216].sqrt();
                let FK = parameters[85] - DF;
                if CL != 0.0 {
                    let FM = FL * FI;
                    oFM = FM;
                } else {
                }
                let FN = parameters[94] - DF;
                if CL != 0.0 {
                    let FO = FL * FI;
                    oFO = FO;
                } else {
                }
                let FP = parameters[275] - DF;
                let FR = if (if 9.999999999999978e-1f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if FR != 0.0 {
                } else {
                    let FS = if (if 1.9999999999999978e0f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oFS = FS;
                    if FS != 0.0 {
                    } else {
                        let FU = FQ - F;
                        oFU = FU;
                        let FV = FU - DF;
                        oFV = FV;
                    }
                }
                let FT = if (if 9.999999999999978e-1f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if FT != 0.0 {
                } else {
                    let FW = if (if 1.9999999999999978e0f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oFW = FW;
                    if FW != 0.0 {
                    } else {
                        let FY = (-1e0f64 / FQ) - F;
                        oFY = FY;
                        let FZ = FY - DF;
                        oFZ = FZ;
                    }
                }
                let FX = if (if 9.999999999999978e-1f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if FX != 0.0 {
                } else {
                    let GA = if (if 1.9999999999999978e0f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oGA = GA;
                    if GA != 0.0 {
                    } else {
                        let GC = FQ - F;
                        oGC = GC;
                        let GD = GC - DF;
                        oGD = GD;
                    }
                }
                let GB = if (if 9.999999999999978e-1f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GB != 0.0 {
                } else {
                    let GE = if (if 1.9999999999999978e0f64 <= FQ { 1.0 } else { 0.0 }) != 0.0 && (if FQ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oGE = GE;
                    if GE != 0.0 {
                    } else {
                        let GF = (-1e0f64 / FQ) - F;
                        oGF = GF;
                        let GG = GF - DF;
                        oGG = GG;
                    }
                }
                if B != 0.0 {
                    let GJ = GI - DF;
                    oGJ = GJ;
                } else {
                }
                let GH = if parameters[246] != A { 1.0 } else { 0.0 };
                if B != 0.0 {
                    let GK = GI - DF;
                    oGK = GK;
                } else {
                }
                let GM = if GL == F { 1.0 } else { 0.0 };
                if GM != 0.0 {
                    let GN = parameters[315] / AJ;
                    oGN = GN;
                    let GP = if GO > A { 1.0 } else { 0.0 };
                    oGP = GP;
                    let GT = ((GS * GS) + (DI * DI)).sqrt();
                    oGT = GT;
                    let GU = parameters[317] / AX;
                    oGU = GU;
                    let GV = parameters[319] / FH;
                    oGV = GV;
                    let GX = GW - DF;
                    oGX = GX;
                } else {
                }
                let GR = if GQ == F { 1.0 } else { 0.0 };
                if GR != 0.0 {
                    let GY = if GO > A { 1.0 } else { 0.0 };
                    oGY = GY;
                    let GZ = ((GS * GS) + (DI * DI)).sqrt();
                    oGZ = GZ;
                    let HA = parameters[316] / AX;
                    oHA = HA;
                    let HB = parameters[318] / FH;
                    oHB = HB;
                    let HC = GW - DF;
                    oHC = HC;
                } else {
                }
                let HD = if CZ >= 8e0f64 { 1.0 } else { 0.0 };
                let HF = if GL != 0.0 {
                    A
                } else {
                    HE
                };
                let HH = if GQ != 0.0 {
                    A
                } else {
                    HG
                };
                let HJ = if GL != 0.0 {
                    HI
                } else {
                    A
                };
                let HL = if GQ != 0.0 {
                    HK
                } else {
                    A
                };
                let HN = if CV != 0.0 {
                    A
                } else {
                    HM
                };
                let HP = if CX != 0.0 {
                    A
                } else {
                    HO
                };
                let HQ = if (if parameters[27] != 0.0 && parameters[15] != 0.0 { 1.0 } else { 0.0 }) != 0.0 && parameters[16] != 0.0 { 1.0 } else { 0.0 };
                let HS = if HQ != 0.0 {
                    A
                } else {
                    HR
                };
                let HV;
                let HW;
                if CY != 0.0 {
                    HV = A;
                    HW = A;
                } else {
                    HV = HT;
                    HW = HU;
                }
            [B, oC, oH, oL, oO, Q, T, U, AB, oBV, AI, AK, AL, AM, AN, AO, AQ, AR, AS, AT, AU, AV, AW, AY, AZ, BA, BB, BD, BE, CA, CE, CG, CI, CK, oCN, CP, CQ, CR, CS, CT, CU, CW, CX, DA, DB, DC, DD, DH, DJ, DK, DL, oDN, oDS, oDP, oDU, DQ, DX, DY, EA, EB, ED, EE, EF, EI, EJ, EN, EO, oEP, oEQ, ER, ET, oEU, oEV, EW, EY, oFC, FD, oFF, oFG, AG, FJ, oFM, oFO, FR, oFS, oFU, FT, oFW, oFY, FX, oGA, oGC, GB, oGE, oGF, GH, GM, oGN, oGP, oGT, oGU, oGV, GR, oGY, oGZ, oHA, oHB, HD, HQ, HF, HH, HJ, HL, HN, HP, HS, HV, HW, DG, FK, FN, FP, oFV, oFZ, oGD, oGG, oGJ, oGK, oGX, oHC]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 106] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[179];
                let B = 1.0f64;
                let D = parameters[5];
                let F = parameters[0];
                let G = 1e6f64;
                let O = 1e0f64;
                let R = 2e0f64;
                let AC = staged[180];
                let AD = 5e-1f64;
                let AN = staged[12];
                let AO = staged[13];
                let AR = parameters[32];
                let AX = parameters[6];
                let AY = 0e0f64;
                let AZ = parameters[7];
                let BA = parameters[8];
                let BN = parameters[165];
                let BP = parameters[167];
                let BV = 1e-2f64;
                let BX = 4e0f64;
                let CI = parameters[168];
                let CJ = parameters[170];
                let CO = parameters[58];
                let CT = 1.6021918e-19f64;
                let CV = 1.034943e-10f64;
                let DF = 1.04e16f64;
                let DK = parameters[115];
                let DN = staged[190];
                let DO = parameters[2];
                let DQ = 1e-3f64;
                let DS = 1e3f64;
                let ED = staged[192];
                let EF = 1e3f64;
                let EK = parameters[162];
                let EL = parameters[164];
                let FC = staged[197];
                let FD = parameters[38];
                let FE = staged[39];
                let FJ = staged[42];
                let FM = 1e-12f64;
                let GE = 1e-9f64;
                let GP = staged[207];
                let GT = staged[208];
                let HB = staged[210];
                let HG = 0.0f64;
                let HJ = 1e0f64;
                let HP = 1.034943e-12f64;
                let HX = staged[222];
                let HY = staged[223];
                let HZ = staged[224];
                let IA = parameters[314];
                let ID = parameters[331];
                let IE = parameters[330];
                let IG = parameters[329];
                let IH = parameters[328];
                let IJ = parameters[327];
                let IK = parameters[326];
                let IN = staged[225];
                let mut oBH = 0.0;
                let mut oBR = 0.0;
                let mut oCD = 0.0;
                let mut oDR = 0.0;
                let mut oGQ = 0.0;
                let mut oGR = 0.0;
                let mut oGS = 0.0;
                let mut oGX = 0.0;
                let mut oGZ = 0.0;
                let mut oHC = 0.0;
                let mut oHH = 0.0;
                let mut oHV = 0.0;
                let mut oIC = 0.0;
                let mut oIF = 0.0;
                let mut oII = 0.0;
                let mut oIL = 0.0;
                let mut oIM = 0.0;
                let mut oIP = 0.0;
                let mut oIQ = 0.0;
                let mut oIR = 0.0;
                let mut oIS = 0.0;
                let mut oIT = 0.0;
                if A != 0.0 {
                    if B != 0.0 {
                        loop {
                            if AC == 0.0 {
                                break;
                            }
                        }
                    } else {
                    }
                } else {
                }
                let C = parameters[9] + 2.7315e2f64;
                let E = parameters[1] / D;
                let H = F * G;
                let I = E * G;
                let J = I * H;
                let K = parameters[62] / (J.powf(parameters[63]));
                let L = F + K;
                let M = parameters[64] / (J.powf(parameters[65]));
                let N = L * G;
                let P = (E + K) * G;
                let Q = (staged[0] * (O + (parameters[148] / (N.powf(parameters[149]))))) * (O + (parameters[150] / (P.powf(parameters[151]))));
                let S = R * ((parameters[152] * (O + (parameters[154] / (N.powf(parameters[155]))))) * (O + (parameters[156] / (P.powf(parameters[157])))));
                let T = S * parameters[153];
                let U = (E - staged[1]) - T;
                let V = (E - staged[2]) - T;
                let W = U * D;
                let X = V * D;
                let Y = staged[3] / W;
                let Z = staged[4] * X;
                let AA = (parameters[11] + (parameters[304] * parameters[12])) + (parameters[305] * parameters[13]);
                let AB = ((staged[6] + (staged[5] * AA)) - 1e21f64) - 1e4f64;
                let AE = 1e21f64 + (AD * (AB + (((AB * AB) + staged[7]).sqrt())));
                let AF = ((staged[9] + (staged[8] * AA)) - 1e21f64) - 1e4f64;
                let AG = 1e21f64 + (AD * (AF + (((AF * AF) + staged[10]).sqrt())));
                let AH = (parameters[86] * (H.powf(parameters[88]))) * (O + (parameters[90] / (H.powf(parameters[91]))));
                let AI = (parameters[87] * (H.powf(parameters[89]))) * (O + (parameters[92] / (H.powf(parameters[93]))));
                let AJ = (parameters[289] * (H.powf(parameters[291]))) * (O + (parameters[293] / (H.powf(parameters[294]))));
                let AK = (parameters[290] * (H.powf(parameters[292]))) * (O + (parameters[295] / (H.powf(parameters[296]))));
                let AL = (parameters[106] * (O + (parameters[107] / (H.powf(parameters[110]))))) * (O + (parameters[108] / (I.powf(parameters[109]))));
                let AM = (parameters[283] * (O + (parameters[285] / (H.powf(parameters[286]))))) * (O + (parameters[287] / (I.powf(parameters[288]))));
                let AP = ((staged[11] * (O + (parameters[232] / (H.powf(parameters[233]))))) - AN) - AO;
                let AQ = AN + (AD * (AP + (((AP * AP) + staged[14]).sqrt())));
                let AT;
                if AR != 0.0 {
                    let AS = ((AQ * (O + (parameters[234] / (I.powf(parameters[235]))))) - AN) - AO;
                    let BC = AN + (AD * (AS + (((AS * AS) + staged[15]).sqrt())));
                    AT = BC;
                } else {
                    AT = AQ;
                }
                let AU = AG * (O + (parameters[60] / (I.powf(parameters[61]))));
                let AV = AD * F;
                let AW = R / ((O / (parameters[43] + AV)) + (O / (parameters[44] + AV)));
                let BB = if (if (if AX > AY { 1.0 } else { 0.0 }) != 0.0 && (if AZ > AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if D == O { 1.0 } else { 0.0 }) != 0.0 || (if (if D > O { 1.0 } else { 0.0 }) != 0.0 && (if BA > AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BD;
                if BB != 0.0 {
                    let mut BF = 0.0;
                    let mut BG = 0.0;
                    BF = AY;
                    BG = AY;
                    loop {
                        let BH = if BF < D { 1.0 } else { 0.0 };
                        oBH = BH;
                        if BH == 0.0 {
                            break;
                        }
                        let BI = BF * (BA + F);
                        let BJ = (BG + (O / ((AX + AV) + BI))) + (O / ((AZ + AV) + BI));
                        let BK = BF + O;
                        BF = BK;
                        BG = BJ;
                    }
                    let BL = (R * D) / BG;
                    BD = BL;
                } else {
                    BD = AY;
                }
                let BE = if BD > AY { 1.0 } else { 0.0 };
                let BS;
                let BT;
                if BE != 0.0 {
                    let BM = O / (O + parameters[166]);
                    let BO = BN / BD;
                    let BQ = if BP == AY { 1.0 } else { 0.0 };
                    let BR = if (if BO == AY { 1.0 } else { 0.0 }) != 0.0 && BQ != 0.0 { 1.0 } else { 0.0 };
                    oBR = BR;
                    let CB = if BR != 0.0 {
                        O
                    } else {
                        let CA = BO.powf(BP);
                        CA
                    };
                    let CC = BN / AW;
                    let CD = if (if CC == AY { 1.0 } else { 0.0 }) != 0.0 && BQ != 0.0 { 1.0 } else { 0.0 };
                    oCD = CD;
                    let CF = if CD != 0.0 {
                        O
                    } else {
                        let CE = CC.powf(BP);
                        CE
                    };
                    let CG = (AU * (O + (BM * CB))) / (O + (BM * CF));
                    let CH = O / (O + parameters[169]);
                    let CK = (AE * (O + (CH * ((CI / BD).powf(CJ))))) / (O + (CH * ((CI / AW).powf(CJ))));
                    BS = CK;
                    BT = CG;
                } else {
                    BS = AE;
                    BT = AU;
                }
                let BU = staged[16] / BS;
                let BW = (BU - (O + (parameters[190] / (I.powf(parameters[191]))))) - BV;
                let BY = (BX * BU) * BV;
                let BZ = if BY > AY { 1.0 } else { 0.0 };
                let CM = if BZ != 0.0 {
                    BY
                } else {
                    let CL = -BY;
                    CL
                };
                let CN = BS * (BU - (AD * (BW + (((BW * BW) + CM).sqrt()))));
                let CP = if (if F > CO { 1.0 } else { 0.0 }) != 0.0 || staged[17] != 0.0 { 1.0 } else { 0.0 };
                let CS = if CP != 0.0 {
                    let CQ = ((CN * (F - CO)) + (BT * CO)) / F;
                    CQ
                } else {
                    let CR = BT + (((BT - CN) * (CO - F)) / CO);
                    CR
                };
                let CU = CT * CS;
                let CW = CU * CV;
                let CX = R * CW;
                let CY = (CT * AT) * CV;
                let CZ = parameters[239] * (H.powf(staged[18]));
                let DA = parameters[243] * (H.powf(staged[19]));
                let DB = parameters[246] * ((H + parameters[248]).powf(staged[20]));
                let DC = if (if F <= staged[21] { 1.0 } else { 0.0 }) != 0.0 && staged[22] != 0.0 { 1.0 } else { 0.0 };
                let DE = if DC != 0.0 {
                    let DD = ((((R * BT) - (((BT - CN) * F) / CO)) - CN) / CN).ln();
                    DD
                } else {
                    AY
                };
                let DG = 5.1702525384001115e-2f64 * ((CS / DF).ln());
                let DH = 5.1702525384001115e-2f64 * ((CN / DF).ln());
                let DI = ((O + (O / H)).powf(parameters[77])) * parameters[75];
                let DJ = parameters[116] * H;
                let DL = (((DJ * DK) / (DJ + DK)) + parameters[117]) + 1e-50f64;
                let DM = O + ((H.powf(parameters[179])) * parameters[180]);
                let DT;
                if DN != 0.0 {
                    let DP = (parameters[48] * (parameters[3] + (U / (3e0f64 * DO)))) / ((DO * (F - parameters[4])) * D);
                    let DR = if DP > DQ { 1.0 } else { 0.0 };
                    oDR = DR;
                    let EG = if DR != 0.0 {
                        let EE = O / DP;
                        EE
                    } else {
                        EF
                    };
                    DT = EG;
                } else {
                    DT = DS;
                }
                let DU = O + (parameters[131] / (I.powf(parameters[132])));
                let DV = parameters[125] * (O + (parameters[126] / (H.powf(parameters[127]))));
                let DW = H / (H + parameters[124]);
                let DX = parameters[118] * (O + (parameters[120] / (H.powf(parameters[121]))));
                let DY = parameters[119] * (O + (parameters[122] / H));
                let DZ = ((1e4f64 * X) * parameters[46]) / (H.powf(parameters[47]));
                let EA = parameters[133] * (O + (parameters[134] / (H.powf(parameters[135]))));
                let EB = parameters[128] * (O + (parameters[129] / (H.powf(parameters[130]))));
                let EC = (1.2919089961638799e9f64 / CS).sqrt();
                let EH = ((parameters[249] * (O + (parameters[95] / (I.powf(parameters[96]))))) * (O + (parameters[97] / (H.powf(parameters[98]))))) * (O + (parameters[99] / (J.powf(parameters[100]))));
                let EI = ((parameters[276] * (O + (parameters[277] / (I.powf(parameters[278]))))) * (O + (parameters[281] / (H.powf(parameters[282]))))) * (O + (parameters[279] / (J.powf(parameters[280]))));
                let EQ;
                let ER;
                if BE != 0.0 {
                    let EJ = O / (O + parameters[163]);
                    let EM = O + (EJ * ((EK / BD).powf(EL)));
                    let EN = O + (EJ * ((EK / AW).powf(EL)));
                    let EO = (EH * EM) / EN;
                    let EP = (EI * EM) / EN;
                    EQ = EO;
                    ER = EP;
                } else {
                    EQ = EH;
                    ER = EI;
                }
                let ES = parameters[111] * (O + (parameters[112] / (H.powf(parameters[113]))));
                let ET = (((O + (parameters[181] / (H.powf(parameters[182])))) * (O + (parameters[185] / (H.powf(parameters[186]))))) * (O + (parameters[187] / (I.powf(parameters[188]))))) * (O + (parameters[183] / (J.powf(parameters[184]))));
                let EU = (AD * (ET + (((ET * ET) + 4e-6f64).sqrt()))) + 1e-13f64;
                let EV = if EU < AY { 1.0 } else { 0.0 };
                let EW = if EV != 0.0 {
                    AY
                } else {
                    EU
                };
                let EX = EW * staged[31];
                let EY = staged[32] * (O + (parameters[102] / (H.powf(parameters[103]))));
                let EZ = ((3.2043836e-19f64 * Q) * CV).sqrt();
                let FA = O / (Q * Q);
                let FB = staged[38] * F;
                let FH;
                if FC != 0.0 {
                    let FF = (FD - FB) - FE;
                    let FQ = FD - (AD * (FF + (((FF * FF) + staged[40]).sqrt())));
                    FH = FQ;
                } else {
                    let FG = (FB - FD) - FE;
                    let FR = FD + (AD * (FG + (((FG * FG) + staged[41]).sqrt())));
                    FH = FR;
                }
                let FI = F - (R * FH);
                let FK = FJ * (O + (parameters[52] / (H.powf(parameters[53]))));
                let FL = -(parameters[49] + (parameters[54] * H));
                let FN = ((FJ * (O + (parameters[50] / (H.powf(parameters[51]))))) - FK) - FM;
                let FO = (BX * FK) * FM;
                let FP = if FO > AY { 1.0 } else { 0.0 };
                let FT = if FP != 0.0 {
                    FO
                } else {
                    let FS = -FO;
                    FS
                };
                let FU = ((FK + (AD * (FN + (((FN * FN) + FT).sqrt())))) - FL) - FM;
                let FV = (BX * FL) * FM;
                let FW = if FV > AY { 1.0 } else { 0.0 };
                let FY = if FW != 0.0 {
                    FV
                } else {
                    let FX = -FV;
                    FX
                };
                let FZ = -(FL + (AD * (FU + (((FU * FU) + FY).sqrt()))));
                let GA = CV / CU;
                let GB = CU * 1.414213562373095e0f64;
                let GC = R * CY;
                let GD = (-1.6021918e-19f64 * CN) * parameters[227];
                let GF = if U < GE { 1.0 } else { 0.0 };
                let GG = if GF != 0.0 {
                    O
                } else {
                    AY
                };
                let GH = if V < GE { 1.0 } else { 0.0 };
                let GI = if GH != 0.0 {
                    O
                } else {
                    GG
                };
                let GJ = if FI < GE { 1.0 } else { 0.0 };
                let GK = if GJ != 0.0 {
                    O
                } else {
                    GI
                };
                let GL = DG + FZ;
                let GM = (CX * DG).sqrt();
                let GN = 9.5e-1f64 * DG;
                let GO = (3.8e0f64 * DG) * DQ;
                if GP != 0.0 {
                    let GQ = (((3.2043836e-19f64 * CN) * CV) * DH).sqrt();
                    oGQ = GQ;
                    let GR = DH + FZ;
                    oGR = GR;
                    let GS = parameters[55] - DG;
                    oGS = GS;
                } else {
                }
                if GT != 0.0 {
                    let GW = (BX * DG) * 5e-3f64;
                    let GX = if GW > AY { 1.0 } else { 0.0 };
                    oGX = GX;
                    let GZ = if GX != 0.0 {
                        GW
                    } else {
                        let GY = -GW;
                        GY
                    };
                    oGZ = GZ;
                } else {
                }
                let GU = F - parameters[57];
                let GV = GU * GU;
                let HA = parameters[71] / F;
                if HB != 0.0 {
                    let HC = staged[68] / (AV + parameters[56]);
                    oHC = HC;
                } else {
                }
                let HD = staged[70] / U;
                let HE = parameters[104] / I;
                let HF = (CN / AT).ln();
                if HG != 0.0 {
                    let HH = ((-GD) * staged[83]) / R;
                    oHH = HH;
                } else {
                }
                let HI = DL - O;
                let HK = HI - HJ;
                let HL = (O / DL) - O;
                let HM = HL - HJ;
                let HN = -X;
                let HO = HN * FI;
                let HQ = (parameters[81] * (O + (parameters[82] / (H.powf(parameters[83]))))) / HP;
                let HR = (parameters[78] * (O + (parameters[79] / (H.powf(parameters[80]))))) / HP;
                let HS = O + (parameters[300] / (H.powf(parameters[301])));
                let HT = parameters[299] * HS;
                let HU = AL - HJ;
                if AR != 0.0 {
                } else {
                    let HV = parameters[302] * HS;
                    oHV = HV;
                }
                let HW = AM - HJ;
                if HX != 0.0 {
                    let IC = if HY != 0.0 {
                        let IB = IA * parameters[308];
                        IB
                    } else {
                        AY
                    };
                    oIC = IC;
                    let IF = O + (IE / (H.powf(ID)));
                    oIF = IF;
                    let II = O + (IH / (H.powf(IG)));
                    oII = II;
                    let IL = O + (IK / (I.powf(IJ)));
                    oIL = IL;
                    let IM = (CT / parameters[311]) * staged[140];
                    oIM = IM;
                } else {
                }
                if HZ != 0.0 {
                    let IP = if IN != 0.0 {
                        let IO = IA * parameters[309];
                        IO
                    } else {
                        AY
                    };
                    oIP = IP;
                    let IQ = O + (IE / (H.powf(ID)));
                    oIQ = IQ;
                    let IR = O + (IH / (H.powf(IG)));
                    oIR = IR;
                    let IS = O + (IK / (I.powf(IJ)));
                    oIS = IS;
                    let IT = (CT / parameters[310]) * staged[149];
                    oIT = IT;
                } else {
                }
                let IV;
                let IW;
                if ED != 0.0 {
                    let IU = O / Y;
                    IV = Z;
                    IW = IU;
                } else {
                    IV = AY;
                    IW = AY;
                }
            [C, H, L, M, S, U, W, X, AH, AI, AJ, AK, AL, AM, BB, oBH, BE, oBR, oCD, BZ, CN, CP, CS, CU, CW, AT, CY, CZ, DA, DB, DC, DG, DI, DM, oDR, DU, DV, DW, DX, DY, DZ, EA, EB, EC, ES, EQ, ER, DE, EV, EX, EY, EZ, FA, FI, FP, FW, FZ, GA, GB, GC, GD, GF, GH, GJ, GK, GL, GM, GN, GO, oGQ, oGR, oGS, oGX, oGZ, GV, HA, oHC, HD, HE, HF, oHH, HI, HL, HN, HO, HQ, HR, HT, oHV, oIC, oIF, oII, oIL, oIM, oIP, oIQ, oIR, oIS, oIT, DT, IV, IW, HK, HM, HU, HW]
        };
        self.canonical_staged[195] = produced[0];
        self.canonical_staged[128] = produced[1];
        self.canonical_staged[125] = produced[2];
        self.canonical_staged[73] = produced[3];
        self.canonical_staged[127] = produced[4];
        self.canonical_staged[88] = produced[5];
        self.canonical_staged[90] = produced[6];
        self.canonical_staged[104] = produced[7];
        self.canonical_staged[112] = produced[8];
        self.canonical_staged[111] = produced[9];
        self.canonical_staged[117] = produced[10];
        self.canonical_staged[116] = produced[11];
        self.canonical_staged[110] = produced[12];
        self.canonical_staged[115] = produced[13];
        self.canonical_staged[182] = produced[14];
        self.canonical_staged[184] = produced[15];
        self.canonical_staged[183] = produced[16];
        self.canonical_staged[185] = produced[17];
        self.canonical_staged[187] = produced[18];
        self.canonical_staged[186] = produced[19];
        self.canonical_staged[43] = produced[20];
        self.canonical_staged[188] = produced[21];
        self.canonical_staged[132] = produced[22];
        self.canonical_staged[49] = produced[23];
        self.canonical_staged[92] = produced[24];
        self.canonical_staged[47] = produced[25];
        self.canonical_staged[79] = produced[26];
        self.canonical_staged[122] = produced[27];
        self.canonical_staged[123] = produced[28];
        self.canonical_staged[124] = produced[29];
        self.canonical_staged[189] = produced[30];
        self.canonical_staged[52] = produced[31];
        self.canonical_staged[74] = produced[32];
        self.canonical_staged[102] = produced[33];
        self.canonical_staged[191] = produced[34];
        self.canonical_staged[96] = produced[35];
        self.canonical_staged[93] = produced[36];
        self.canonical_staged[95] = produced[37];
        self.canonical_staged[91] = produced[38];
        self.canonical_staged[97] = produced[39];
        self.canonical_staged[134] = produced[40];
        self.canonical_staged[87] = produced[41];
        self.canonical_staged[94] = produced[42];
        self.canonical_staged[126] = produced[43];
        self.canonical_staged[27] = produced[44];
        self.canonical_staged[28] = produced[45];
        self.canonical_staged[29] = produced[46];
        self.canonical_staged[30] = produced[47];
        self.canonical_staged[196] = produced[48];
        self.canonical_staged[34] = produced[49];
        self.canonical_staged[33] = produced[50];
        self.canonical_staged[36] = produced[51];
        self.canonical_staged[37] = produced[52];
        self.canonical_staged[89] = produced[53];
        self.canonical_staged[200] = produced[54];
        self.canonical_staged[201] = produced[55];
        self.canonical_staged[51] = produced[56];
        self.canonical_staged[44] = produced[57];
        self.canonical_staged[45] = produced[58];
        self.canonical_staged[46] = produced[59];
        self.canonical_staged[80] = produced[60];
        self.canonical_staged[202] = produced[61];
        self.canonical_staged[203] = produced[62];
        self.canonical_staged[204] = produced[63];
        self.canonical_staged[205] = produced[64];
        self.canonical_staged[54] = produced[65];
        self.canonical_staged[56] = produced[66];
        self.canonical_staged[57] = produced[67];
        self.canonical_staged[58] = produced[68];
        self.canonical_staged[59] = produced[69];
        self.canonical_staged[60] = produced[70];
        self.canonical_staged[62] = produced[71];
        self.canonical_staged[209] = produced[72];
        self.canonical_staged[64] = produced[73];
        self.canonical_staged[65] = produced[74];
        self.canonical_staged[66] = produced[75];
        self.canonical_staged[69] = produced[76];
        self.canonical_staged[71] = produced[77];
        self.canonical_staged[72] = produced[78];
        self.canonical_staged[76] = produced[79];
        self.canonical_staged[84] = produced[80];
        self.canonical_staged[99] = produced[81];
        self.canonical_staged[100] = produced[82];
        self.canonical_staged[133] = produced[83];
        self.canonical_staged[103] = produced[84];
        self.canonical_staged[108] = produced[85];
        self.canonical_staged[107] = produced[86];
        self.canonical_staged[106] = produced[87];
        self.canonical_staged[114] = produced[88];
        self.canonical_staged[143] = produced[89];
        self.canonical_staged[137] = produced[90];
        self.canonical_staged[139] = produced[91];
        self.canonical_staged[138] = produced[92];
        self.canonical_staged[141] = produced[93];
        self.canonical_staged[151] = produced[94];
        self.canonical_staged[146] = produced[95];
        self.canonical_staged[148] = produced[96];
        self.canonical_staged[147] = produced[97];
        self.canonical_staged[150] = produced[98];
        self.canonical_staged[152] = produced[99];
        self.canonical_staged[153] = produced[100];
        self.canonical_staged[154] = produced[101];
        self.canonical_staged[156] = produced[102];
        self.canonical_staged[157] = produced[103];
        self.canonical_staged[160] = produced[104];
        self.canonical_staged[162] = produced[105];
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
                let A = staged[179];
                let B = 1.0f64;
                let C = staged[180];
                let D = staged[182];
                let E = staged[184];
                let F = if parameter_given[9] { 1.0 } else { 0.0 };
                let G = staged[195];
                let H = temperature;
                if A != 0.0 {
                    if B != 0.0 {
                        loop {
                            if C == 0.0 {
                                break;
                            }
                        }
                    } else {
                    }
                } else {
                }
                if D != 0.0 {
                    loop {
                        if E == 0.0 {
                            break;
                        }
                    }
                } else {
                }
                let I = if F != 0.0 {
                    G
                } else {
                    H
                };
                let J = I + parameters[10];
            [J]
        };
        self.canonical_staged[23] = produced[0];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[179];
                let B = 1.0f64;
                let C = staged[180];
                let D = staged[182];
                let E = staged[184];
                if A != 0.0 {
                    if B != 0.0 {
                        loop {
                            if C == 0.0 {
                                break;
                            }
                        }
                    } else {
                    }
                } else {
                }
                if D != 0.0 {
                    loop {
                        if E == 0.0 {
                            break;
                        }
                    }
                } else {
                }
            [0.0]
        };
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_model_stage(ctx);
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        self.canonical_timestep_stage(ctx);
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 45332 => 0usize, 45338 => 1usize, 45344 => 2usize, 45375 => 3usize, 45379 => 4usize, 45429 => 5usize, 45445 => 6usize, 45453 => 7usize, 45459 => 8usize, _ => usize::MAX };
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
            let A = staged[171];
            let B = staged[179];
            let C = 1.0f64;
            let D = staged[180];
            let E = parameters[32];
            let F = staged[182];
            let G = staged[184];
            let H = node_potentials[5];
            let I = node_potentials[12];
            let J = 1e0f64;
            let K = 1e0f64;
            let L = parameters[33];
            let O = node_potentials[11];
            let P = 1e0f64;
            let S = node_potentials[6];
            let T = 1e0f64;
            let W = node_potentials[2];
            let X = 1e0f64;
            let AA = node_potentials[0];
            let AB = 1e0f64;
            let AG = staged[192];
            let AH = node_potentials[4];
            let AI = 0e0f64;
            let AK = 0e0f64;
            let AN = parameters[24];
            let AO = 1e0f64;
            let AR = 1e-9f64;
            let AS = node_potentials[8];
            let AU = 1e0f64;
            let AW = node_potentials[9];
            let AY = 1e0f64;
            let BA = 0e0f64;
            let BB = 0e0f64;
            let BL = 1e0f64;
            let BP = -1e0f64;
            let BZ = -1e0f64;
            let CR = staged[25];
            let CT = parameters[35];
            let CU = parameters[36];
            let CX = 1.3806226e-23f64;
            let CZ = 1.6021918e-19f64;
            let DJ = parameters[202];
            let DK = parameters[201];
            let DO = parameters[253];
            let DS = 1e0f64;
            let DU = staged[28];
            let DX = staged[29];
            let EA = staged[30];
            let ED = 4e-1f64;
            let EG = 1e-2f64;
            let EH = 1e-1f64;
            let EN = staged[33];
            let ET = 2e0f64;
            let EY = 1.5e0f64;
            let EZ = 1.04e16f64;
            let FB = 2e0f64;
            let FH = staged[36];
            let FQ = staged[37];
            let FV = staged[43];
            let GA = staged[44];
            let GC = staged[45];
            let GF = staged[46];
            let GM = staged[47];
            let GR = staged[48];
            let GU = parameters[255];
            let HG = 1.0f64;
            let HH = Lanes([0e0f64; 3]);
            let HN = parameters[216];
            let HQ = 1.984126984126984e-4f64;
            let HZ = 1e-12f64;
            let IB = 0.0f64;
            let IC = 1.25e-1f64;
            let IH = 1e-50f64;
            let IS = 0.0f64;
            let IU = 1.0f64;
            let IW = 3e0f64;
            let IX = 0.0f64;
            let IZ = 4e0f64;
            let JR = staged[49];
            let JS = 1.034943e-10f64;
            let JU = staged[50];
            let JW = staged[51];
            let KF = 5e-1f64;
            let KJ = Lanes([0e0f64; 5]);
            let KP = 5e-2f64;
            let KR = 1.0f64;
            let KS = 2.0000000000000004e-2f64;
            let KT = -2.0000000000000004e-2f64;
            let LL = parameters[193];
            let LM = parameters[195];
            let LN = parameters[194];
            let LQ = staged[52];
            let LR = staged[53];
            let LS = staged[54];
            let LY = Lanes([0e0f64; 4]);
            let MN = 1e-3f64;
            let MZ = 1e-4f64;
            let NJ = parameters[226];
            let NO = 3.453133e-11f64;
            let OC = staged[56];
            let OG = staged[57];
            let OP = staged[207];
            let OQ = staged[59];
            let OS = staged[61];
            let OT = staged[62];
            let OV = staged[63];
            let OW = parameters[67];
            let PG = staged[208];
            let PH = 2.5e-1f64;
            let PL = 5e-3f64;
            let PS = parameters[227];
            let PX = staged[65];
            let QF = -1e0f64;
            let RH = parameters[297];
            let RM = staged[66];
            let RN = parameters[70];
            let RO = parameters[250];
            let RT = staged[210];
            let RU = parameters[73];
            let RW = staged[69];
            let RZ = Lanes([0e0f64; 4]);
            let SK = staged[212];
            let SS = staged[76];
            let TB = parameters[29];
            let TL = 3.7037037037037035e-2f64;
            let TQ = 1.48148111111111e-1f64;
            let UA = staged[74];
            let UR = staged[77];
            let VF = staged[78];
            let VV = 2.220446049250313e-15f64;
            let WJ = 8e-4f64;
            let WV = 5e2f64;
            let WZ = 1e-8f64;
            let XE = staged[79];
            let XV = staged[80];
            let YF = staged[81];
            let YN = staged[83];
            let ZC = parameters[298];
            let ZM = 0.0f64;
            let AAD = 2e-1f64;
            let ABC = -1e-1f64;
            let ACJ = 2.220446049250313e-15f64;
            let ADL = 2.220446049250313e-15f64;
            let AEP = -1e0f64;
            let AHA = 1e-13f64;
            let ALL = 2.220446049250313e-15f64;
            let ALU = staged[85];
            let ALV = staged[86];
            let ANK = 1.5e-1f64;
            let ANV = 1.0f64;
            let ANY = 1.0f64;
            let AOI = 0.0f64;
            let AOK = 0.0f64;
            let AOM = 0.0f64;
            let APS = 2.220446049250313e-15f64;
            let AQQ = 2.220446049250313e-15f64;
            let ART = 1.0f64;
            let AUK = 1e-10f64;
            let AVA = 1.0f64;
            let AWU = 0.0f64;
            let AWV = 1e-10f64;
            let AXK = 0.0f64;
            let AZJ = 1.0f64;
            let AZM = 1.0f64;
            let AZW = 0.0f64;
            let AZY = 0.0f64;
            let BAA = 0.0f64;
            let BBN = parameters[136];
            let BCB = Lanes([0e0f64; 6]);
            let BCU = 1.0f64;
            let BDF = 3.0000000000000002e-2f64;
            let BDJ = staged[89];
            let BEB = 2.220446049250313e-15f64;
            let BEO = 1.3e0f64;
            let BES = 3e-2f64;
            let BFB = 1e2f64;
            let BFD = staged[90];
            let BFK = parameters[144];
            let BFQ = staged[91];
            let BFZ = 4.12e0f64;
            let BGP = parameters[142];
            let BHB = 7.38905609893065e0f64;
            let BHV = staged[92];
            let BHY = staged[93];
            let BIO = staged[94];
            let BIT = parameters[123];
            let BIV = staged[95];
            let BIW = staged[96];
            let BJI = staged[97];
            let BJQ = parameters[140];
            let BJW = parameters[139];
            let BKM = parameters[27];
            let BKV = node_potentials[10];
            let BKX = 1e0f64;
            let BNR = 8e1f64;
            let BOF = 1.4142135623730951e0f64;
            let BOI = 1.4142135623730951e0f64;
            let BOP = 5.540622384e34f64;
            let BRZ = 1.4142135623730951e0f64;
            let BSC = 1.4142135623730951e0f64;
            let BVY = 1.4142135623730951e0f64;
            let BWB = 1.4142135623730951e0f64;
            let BXW = 0e0f64;
            let BYB = staged[98];
            let BYJ = 0e0f64;
            let CAK = 0e0f64;
            let CAX = 2.5e1f64;
            let CAY = 4e1f64;
            let CBD = 2e1f64;
            let CBG = 1e1f64;
            let CBJ = 5e0f64;
            let CEN = staged[99];
            let CER = staged[100];
            let CFV = 1.15e0f64;
            let CFZ = 1.15e0f64;
            let CGJ = 1.15e0f64;
            let CGV = 5e-13f64;
            let CHW = 2.220446049250313e-15f64;
            let CIU = 2.220446049250313e-15f64;
            let CKH = 2e2f64;
            let CUF = 1.4142135623730951e0f64;
            let CUI = 1.4142135623730951e0f64;
            let CXZ = 1.4142135623730951e0f64;
            let CYC = 1.4142135623730951e0f64;
            let DBZ = 1.4142135623730951e0f64;
            let DCC = 1.4142135623730951e0f64;
            let DJK = -5e-1f64;
            let DJQ = 1e-18f64;
            let DKI = -5e-1f64;
            let DKL = -5e-1f64;
            let DKP = staged[101];
            let DKQ = parameters[178];
            let DLF = parameters[176];
            let DLQ = staged[102];
            let DMN = 1e9f64;
            let DNB = staged[103];
            let DNE = staged[104];
            let DNK = parameters[217];
            let DNN = 1.984126984126984e-4f64;
            let DNX = 2.220446049250313e-15f64;
            let DOC = 1e4f64;
            let DPA = staged[105];
            let DPC = parameters[85];
            let DPD = staged[158];
            let DPE = parameters[84];
            let DPG = staged[106];
            let DPH = staged[107];
            let DPI = staged[108];
            let DPP = 3.9e0f64;
            let DPQ = staged[109];
            let DQG = parameters[94];
            let DQI = staged[110];
            let DQJ = staged[111];
            let DQK = 1e11f64;
            let DQN = parameters[105];
            let DQS = staged[113];
            let DRN = staged[114];
            let DRO = -5e-1f64;
            let DRT = parameters[275];
            let DRV = staged[115];
            let DRW = staged[116];
            let DRZ = parameters[284];
            let DSR = staged[213];
            let DSS = staged[214];
            let DSX = staged[215];
            let DSY = staged[118];
            let DTF = staged[216];
            let DTU = staged[217];
            let DTY = staged[119];
            let DUE = staged[218];
            let DUJ = staged[219];
            let DUK = staged[120];
            let DUR = staged[220];
            let DVF = staged[121];
            let DVN = 1.984126984126984e-4f64;
            let DVX = 1.1e0f64;
            let DWH = staged[221];
            let DWK = staged[122];
            let DWM = parameters[240];
            let DWQ = parameters[241];
            let DWS = staged[123];
            let DWZ = staged[124];
            let DYF = 1.984126984126984e-4f64;
            let EAM = 1.0f64;
            let EAS = 0.0f64;
            let EAT = 2.5e-1f64;
            let EBD = 1.0f64;
            let EBF = 0.0f64;
            let EBH = 0.0f64;
            let EBV = staged[126];
            let EBZ = parameters[159];
            let ECE = parameters[161];
            let ECF = parameters[160];
            let EDD = -1e0f64;
            let EGV = 1.0f64;
            let EGW = 0.0f64;
            let EGX = 1.25e-1f64;
            let EHO = 0.0f64;
            let EHQ = 1.0f64;
            let EHS = 0.0f64;
            let EJZ = parameters[145];
            let ELA = parameters[146];
            let ENE = Lanes([0e0f64; 3]);
            let ENR = parameters[206];
            let ENT = parameters[205];
            let ENV = parameters[207];
            let EOH = parameters[212];
            let EOJ = parameters[260];
            let EOO = 1e6f64;
            let EOP = staged[128];
            let EPN = parameters[209];
            let EPP = parameters[208];
            let EQF = parameters[257];
            let EQS = -1e0f64;
            let ERH = -1e0f64;
            let ERO = parameters[261];
            let ESB = parameters[263];
            let ESJ = parameters[265];
            let EST = parameters[262];
            let ESX = parameters[269];
            let ETJ = parameters[271];
            let ETR = parameters[273];
            let EUG = parameters[270];
            let EVC = parameters[199];
            let EVD = parameters[198];
            let EVG = parameters[200];
            let EVH = parameters[228];
            let EVT = staged[129];
            let EVZ = staged[130];
            let EYI = parameters[45];
            let EYP = parameters[175];
            let EYR = staged[131];
            let EZG = 0e0f64;
            let EZJ = 1e0f64;
            let EZL = if parameter_given[173] { 1.0 } else { 0.0 };
            let EZO = if parameter_given[174] { 1.0 } else { 0.0 };
            let FAF = Lanes([0e0f64; 3]);
            let FAM = -0e0f64;
            let FBA = parameters[39];
            let FBL = 4.242640687119285e0f64;
            let FBO = 8e0f64;
            let FBX = 9e0f64;
            let FDM = 3.333333333333333e-1f64;
            let FDP = 1.2e1f64;
            let FDQ = 1.414213562373095e0f64;
            let FEB = Lanes([0e0f64; 4]);
            let FEC = 2.220446049250313e-15f64;
            let FEQ = 2.220446049250313e-15f64;
            let FFF = -1.047839336957922e-1f64;
            let FFG = 5.286687693921294e-4f64;
            let FFH = 1.8773541122053122e-2f64;
            let FFK = 2.8160311683079683e-2f64;
            let FFL = 7.930031540881942e-4f64;
            let FGB = parameters[30];
            let FHX = 6.0000000000000005e-2f64;
            let FIJ = 2.220446049250313e-15f64;
            let FKH = 6.115288895133179e-3f64;
            let FKJ = 2.9693154855771e-1f64;
            let FKX = 6.36964918866352e-5f64;
            let FKZ = 1.78800506338833e-2f64;
            let FLC = 7.07106781186548e-1f64;
            let FNE = 4.1e1f64;
            let FNR = 5e-2f64;
            let FOB = -1e0f64;
            let FPK = 0e0f64;
            let FPR = 1e0f64;
            let FPW = -0e0f64;
            let FQL = 4.242640687119285e0f64;
            let FSP = 2.220446049250313e-15f64;
            let FTD = 2.220446049250313e-15f64;
            let FTS = -1.047839336957922e-1f64;
            let FTT = 5.286687693921294e-4f64;
            let FTU = 1.8773541122053122e-2f64;
            let FTX = 2.8160311683079683e-2f64;
            let FTY = 7.930031540881942e-4f64;
            let FWJ = 6.0000000000000005e-2f64;
            let FWV = 2.220446049250313e-15f64;
            let GBF = 4.1e1f64;
            let GBS = 5e-2f64;
            let GCC = -1e0f64;
            let GDM = parameters[174];
            let GDN = parameters[173];
            let GDO = staged[133];
            let GEU = parameters[223];
            let GEV = parameters[224];
            let GFF = parameters[225];
            let GFM = 1e5f64;
            let GFP = parameters[114];
            let GGR = 6e0f64;
            let GGZ = 1.5e1f64;
            let GHS = 4.2e1f64;
            let GIP = 3.872983346207417e0f64;
            let GJI = if parameter_given[172] { 1.0 } else { 0.0 };
            let GJM = Lanes([0e0f64; 4]);
            let GJW = -5e-1f64;
            let GJX = -5e-1f64;
            let GKH = parameters[303];
            let GKQ = Lanes([0e0f64; 8]);
            let GLX = staged[134];
            let GNS = 5.5224904e-23f64;
            let GOA = 1.898893985185185e-20f64;
            let GOQ = 6.666666666666667e-1f64;
            let GPR = 5e-1f64;
            let GQA = 5e-1f64;
            let GQF = staged[222];
            let GQG = Lanes([0e0f64; 3]);
            let GQJ = staged[224];
            let GQL = parameters[320];
            let GQO = 1.8e0f64;
            let GQP = parameters[321];
            let GQS = parameters[325];
            let GQV = staged[137];
            let GQY = staged[138];
            let GQZ = staged[139];
            let GRC = parameters[311];
            let GSF = staged[141];
            let GSG = staged[142];
            let GTD = Lanes([0e0f64; 3]);
            let GTN = staged[146];
            let GTQ = staged[147];
            let GTR = staged[148];
            let GTU = parameters[310];
            let GUX = staged[150];
            let GVV = Lanes([0e0f64; 2]);
            let GVW = Lanes([0e0f64; 7]);
            let GVX = Lanes([0e0f64; 7]);
            let GWZ = 5e-1f64;
            let GYO = parameters[312];
            let GYU = parameters[313];
            let GZC = ddt_scale();
            let GZT = node_potentials[7];
            let GZW = 1e0f64;
            let HAI = parameters[25];
            let HAJ = 1e0f64;
            let HAK = staged[152];
            let HAN = Lanes([0e0f64; 2]);
            let HAQ = staged[153];
            let HAT = staged[154];
            let HBA = staged[227];
            let HBF = 0e0f64;
            let HII = 0e0f64;
            let HIJ = 0e0f64;
            let HIK = 0e0f64;
            let HIL = 0e0f64;
            let HIM = 0e0f64;
            let HIN = 0e0f64;
            let HIO = 0e0f64;
            if B != 0.0 {
                if C != 0.0 {
                    loop {
                        if D == 0.0 {
                            break;
                        }
                    }
                } else {
                }
            } else {
            }
            if F != 0.0 {
                loop {
                    if G == 0.0 {
                        break;
                    }
                }
            } else {
            }
            let M = L * (H - I);
            let N = (Lanes([J, 0.0]) - Lanes([0.0, K])) * L;
            let Q = L * (O - I);
            let R = (Lanes([P, 0.0]) - Lanes([0.0, K])) * L;
            let U = L * (S - I);
            let V = (Lanes([T, 0.0]) - Lanes([0.0, K])) * L;
            let Y = L * (H - W);
            let Z = (Lanes([0.0, J]) - Lanes([X, 0.0])) * L;
            let AC = L * (AA - W);
            let AD = (Lanes([AB, 0.0]) - Lanes([0.0, X])) * L;
            let AE = L * (S - W);
            let AF = (Lanes([0.0, T]) - Lanes([X, 0.0])) * L;
            let AL;
            let AM;
            if AG != 0.0 {
                let AJ = if AH > AI { 1.0 } else { 0.0 };
                let AP;
                let AQ;
                if AJ != 0.0 {
                    AP = AH;
                    AQ = AO;
                } else {
                    AP = AI;
                    AQ = AK;
                }
                AL = AP;
                AM = AQ;
            } else {
                AL = AI;
                AM = AK;
            }
            let BC;
            let BD;
            let BE;
            let BF;
            if AN != 0.0 {
                let AT = AR * AS;
                let AV = AU * AR;
                let AX = AR * AW;
                let AZ = AY * AR;
                BC = AT;
                BD = AX;
                BE = AV;
                BF = AZ;
            } else {
                BC = AI;
                BD = AI;
                BE = BA;
                BF = BB;
            }
            let BG = if Q >= AI { 1.0 } else { 0.0 };
            let CA;
            let CB;
            let CC;
            let CD;
            let CE;
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
            if BG != 0.0 {
                let BH = Lanes([V[0], 0.0, V[1]]);
                let BI = Lanes([N[0], 0.0, N[1]]);
                let BJ = Lanes([0.0, Z[0], Z[1]]);
                let BK = Lanes([0.0, AF[0], AF[1]]);
                CA = U;
                CB = Q;
                CC = M;
                CD = BL;
                CE = AI;
                CF = Y;
                CG = AC;
                CH = BL;
                CI = AE;
                CJ = BH;
                CK = R;
                CL = BI;
                CM = BJ;
                CN = AD;
                CO = BK;
            } else {
                let BM = M - Q;
                let BN = Lanes([N[0], 0.0, N[1]]) - Lanes([0.0, R[0], R[1]]);
                let BO = -Q;
                let BQ = R * BP;
                let BR = U - Q;
                let BS = Lanes([V[0], 0.0, V[1]]) - Lanes([0.0, R[0], R[1]]);
                let BT = Y - AC;
                let BU = Lanes([0.0, Z[0], Z[1]]) - Lanes([AD[0], AD[1], 0.0]);
                let BV = -AC;
                let BW = AD * BP;
                let BX = AE - AC;
                let BY = Lanes([0.0, AF[0], AF[1]]) - Lanes([AD[0], AD[1], 0.0]);
                CA = BR;
                CB = BO;
                CC = BM;
                CD = AI;
                CE = BL;
                CF = BT;
                CG = BV;
                CH = BZ;
                CI = BX;
                CJ = BS;
                CK = BQ;
                CL = BN;
                CM = BU;
                CN = BW;
                CO = BY;
            }
            let CP = staged[23] + AL;
            let CQ = AM * CP;
            let CS = CP - CR;
            let CV = (staged[26] - (CT * CS)) - (CU * ((CP * CP) - staged[24]));
            let CW = ((AM * CT) * BP) - ((CQ + CQ) * CU);
            let CY = CX * CP;
            let DA = CZ / CY;
            let DB = (((AM * CX) * DA) * BP) / CY;
            let DC = DA * DA;
            let DD = DB * DA;
            let DE = DD + DD;
            let DF = BL / DA;
            let DG = ((DB * DF) * BP) / DA;
            let DH = CP / CR;
            let DI = AM / CR;
            let DL = DK * (DH.powf(DJ));
            let DM = (DI * (DJ * (DH.powf(staged[155])))) * DK;
            let DN = DH - BL;
            let DP = DO * DN;
            let DQ = staged[27] + (DP * DN);
            let DR = DH.powf(DQ);
            let DT = (DI * (DQ * (DH.powf((DQ - DS))))) + ((((DI * DO) * DN) + (DI * DP)) * (DR * (DH.ln())));
            let DV = DR / DU;
            let DW = DT / DU;
            let DY = DR / DX;
            let DZ = DT / DX;
            let EB = EA * DF;
            let EC = DG * EA;
            let EE = ED * DH;
            let EF = DI * ED;
            let EI = EH * DH;
            let EJ = EI * DH;
            let EK = ((DI * EH) * DH) + (DI * EI);
            let EL = BL - DH;
            let EM = DI * BP;
            let EO = ((1.8000000000000002e-2f64 + (EE * EG)) + (EJ * EG)) - (EN * EL);
            let EP = staged[34] / EO;
            let EQ = EG * EP;
            let ER = ((((((EF * EG) + (EK * EG)) - (EM * EN)) * EP) * BP) / EO) * EG;
            let ES = CV.sqrt();
            let EU = CW * (DS / (ET * ES));
            let EV = CV * ES;
            let EW = (CW * ES) + (EU * CV);
            let EX = DH.sqrt();
            let FA = EZ * (DH * EX);
            let FC = (-CV) / FB;
            let FD = ((FC * DA) + staged[35]).exp();
            let FE = FA * FD;
            let FF = (((DI * (EY * EX)) * EZ) * FD) + ((((((CW * BP) / FB) * DA) + (DB * FC)) * FD) * FA);
            let FG = DF.sqrt();
            let FI = FH * FG;
            let FJ = (DG * (DS / (ET * FG))) * FH;
            let FK = FI * FI;
            let FL = FJ * FI;
            let FM = FL + FL;
            let FN = FE * FE;
            let FO = FF * FE;
            let FP = FO + FO;
            let FR = FN * FQ;
            let FS = FP * FQ;
            let FT = FB * DF;
            let FU = DG * FB;
            let FW = FV / FE;
            let FX = FW.ln();
            let FY = FT * FX;
            let FZ = (FU * FX) + (((((FF * FW) * BP) / FE) * (DS / FW)) * FT);
            let GB = (GA * DF).sqrt();
            let GD = GC * GB;
            let GE = ((DG * GA) * (DS / (ET * GB))) * GC;
            let GG = (GF * DF).sqrt();
            let GH = (DG * GF) * (DS / (ET * GG));
            let GI = FE / FV;
            let GJ = GI * GI;
            let GK = (FF / FV) * GI;
            let GL = GK + GK;
            let GN = FE / GM;
            let GO = GN * GN;
            let GP = (FF / GM) * GN;
            let GQ = GP + GP;
            let GS = if CA > GR { 1.0 } else { 0.0 };
            let HI;
            let HJ;
            let HK;
            let HL;
            if GS != 0.0 {
                let GT = CA - GR;
                let GV = GU - GR;
                let GW = GT * GT;
                let GX = CJ * GT;
                let GY = GX + GX;
                let GZ = GV * GV;
                let HA = GW * GW;
                let HB = GY * GW;
                let HC = HA * GW;
                let HD = ((((HB + HB) * GW) + (GY * HA)) * GW) + (GY * HC);
                let HE = ((GZ * GZ) * GZ) * GZ;
                let HF = (HC * GW) + HE;
                let IF;
                let IG;
                if HG != 0.0 {
                    let IT;
                    if IB != 0.0 {
                        IT = BL;
                    } else {
                        let IV;
                        if IS != 0.0 {
                            IV = FB;
                        } else {
                            let IY;
                            if IU != 0.0 {
                                IY = IW;
                            } else {
                                let JA = if IX != 0.0 {
                                    IZ
                                } else {
                                    AI
                                };
                                IY = JA;
                            }
                            IV = IY;
                        }
                        IT = IV;
                    }
                    let mut JB = 0.0;
                    let mut JC = 0.0;
                    let mut JD = Lanes([0.0; 3]);
                    JB = AI;
                    JC = HF;
                    JD = HD;
                    loop {
                        let JE = if JB < IT { 1.0 } else { 0.0 };
                        if JE == 0.0 {
                            break;
                        }
                        let JF = JC.sqrt();
                        let JG = JD * (DS / (ET * JF));
                        let JH = JB + BL;
                        JB = JH;
                        JC = JF;
                        JD = JG;
                    }
                    IF = JC;
                    IG = JD;
                } else {
                    let ID = HF.powf(IC);
                    let IE = HD * (IC * (HF.powf(-8.75e-1f64)));
                    IF = ID;
                    IG = IE;
                }
                let II = IF + IH;
                let IJ = BL / II;
                let IK = ((IG * IJ) * BP) / II;
                let IL = GT * GV;
                let IM = ((CJ * GV) * IJ) + (IK * IL);
                let IN = GV * HE;
                let IO = HF + IH;
                let IP = (IN * IJ) / IO;
                let IQ = ((IK * IN) - (HD * IP)) / IO;
                let IR = GR + (IL * IJ);
                HI = IR;
                HJ = IP;
                HK = IM;
                HL = IQ;
            } else {
                HI = CA;
                HJ = BL;
                HK = CJ;
                HL = HH;
            }
            let HM = CK * HJ;
            let HO = (FB * ((HJ * CB) / FB)) / HN;
            let HP = ((((HL * CB) + Lanes([0.0, HM[0], HM[1]])) / FB) * FB) / HN;
            let HR = 1.388888888888889e-3f64 + (HO * HQ);
            let HS = 8.333333333333333e-3f64 + (HO * HR);
            let HT = 4.1666666666666664e-2f64 + (HO * HS);
            let HU = 1.6666666666666666e-1f64 + (HO * HT);
            let HV = 5e-1f64 + (HO * HU);
            let HW = BL + (HO * HV);
            let HX = HN / HW;
            let HY = ((((HP * HV) + (((HP * HU) + (((HP * HT) + (((HP * HS) + (((HP * HR) + ((HP * HQ) * HO)) * HO)) * HO)) * HO)) * HO)) * HX) * BP) / HW;
            let IA = if HX < HZ { 1.0 } else { 0.0 };
            let JI;
            let JJ;
            if IA != 0.0 {
                JI = HZ;
                JJ = HH;
            } else {
                JI = HX;
                JJ = HY;
            }
            let JK = HI + JI;
            let JL = HK + JJ;
            let JM = CB + (FB * JI);
            let JN = Lanes([0.0, CK[0], CK[1]]) + (JJ * FB);
            let JO = CC + JI;
            let JP = Lanes([CL[0], 0.0, CL[1], CL[2]]);
            let JQ = JP + Lanes([0.0, JJ[0], JJ[1], JJ[2]]);
            let JT = (FB * JR) * JS;
            let JV = (JT * JU) * JU;
            let JX = CC - JW;
            let JY = FB / JV;
            let JZ = Lanes([0.0, CL[0], CL[1], CL[2]]) - Lanes([DG, 0.0, 0.0, 0.0]);
            let KA = Lanes([0.0, 0.0, HK[0], HK[1], HK[2]]);
            let KB = (Lanes([JZ[0], JZ[1], 0.0, JZ[2], JZ[3]]) - KA) * JY;
            let KC = BL + (JY * ((JX - DF) - HI));
            let KD = KB * KC;
            let KE = ((KC * KC) + 4e-6f64).sqrt();
            let KG = (KB + ((KD + KD) * (DS / (ET * KE)))) * KF;
            let KH = (KF * (KC + KE)) + 1e-13f64;
            let KI = if KH < AI { 1.0 } else { 0.0 };
            let KK;
            let KL;
            if KI != 0.0 {
                KK = AI;
                KL = KJ;
            } else {
                KK = KH;
                KL = KG;
            }
            let KM = (KK + IH).sqrt();
            let KN = Lanes([0.0, CL[0], 0.0, CL[1], CL[2]]);
            let KO = (KN + (((KL * (DS / (ET * KM))) * BP) * JV)) - Lanes([FZ, 0.0, 0.0, 0.0, 0.0]);
            let KQ = (((JX + (JV * (BL - KM))) - FY) - EH) - KP;
            let KU = if KR != 0.0 {
                KS
            } else {
                KT
            };
            let KV = KO * KQ;
            let KW = ((KQ * KQ) + KU).sqrt();
            let KX = EH + (KF * (KQ + KW));
            let KY = CB / KX;
            let KZ = Lanes([0.0, 0.0, 0.0, CK[0], CK[1]]);
            let LA = (KZ - (((KO + ((KV + KV) * (DS / (ET * KW)))) * KF) * KY)) / KX;
            let LB = KY * KY;
            let LC = LA * KY;
            let LD = LC + LC;
            let LE = LD * LB;
            let LF = (((BL + KY) + LB) + (LB * KY)) + (LB * LB);
            let LG = BL / LF;
            let LH = BL - LG;
            let LI = LH * LH;
            let LJ = (((((((LA + LD) + ((LD * KY) + (LA * LB))) + (LE + LE)) * LG) * BP) / LF) * BP) * LH;
            let LK = LJ + LJ;
            let LO = if (if (if LL == AI { 1.0 } else { 0.0 }) != 0.0 && (if LM == AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if LN == AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let LP = if LO != 0.0 {
                AI
            } else {
                BL
            };
            let LT = LS + (((JT * LQ).sqrt()) / LR);
            let LU = if LP == AI { 1.0 } else { 0.0 };
            let MG;
            let MH;
            let MI;
            let MJ;
            let MK;
            let ML;
            if LU != 0.0 {
                let LV = (GD * JU) * JU;
                let LW = LV * GD;
                let LX = Lanes([((((GE * JU) * JU) * GD) + (GE * LV)), 0.0, 0.0, 0.0, 0.0]);
                MG = JU;
                MH = LW;
                MI = LR;
                MJ = LY;
                MK = LX;
                ML = LY;
            } else {
                let LZ = JP - Lanes([0.0, HK[0], HK[1], HK[2]]);
                let MA = ((CC - HI) - LT) + LN;
                let MB = LZ * MA;
                let MC = ((MA * MA) + 4e-8f64).sqrt();
                let MD = (LZ + ((MB + MB) * (DS / (ET * MC)))) * KF;
                let ME = (KF * (MA + MC)) + 1.0000000000000002e-14f64;
                let MF = if ME < AI { 1.0 } else { 0.0 };
                let MP;
                let MQ;
                if MF != 0.0 {
                    MP = AI;
                    MQ = LY;
                } else {
                    MP = ME;
                    MQ = MD;
                }
                let MR = BL / MP;
                let MS = ((MQ * MR) * BP) / MP;
                let MT = FB * (LT.abs());
                let MU = (JW - LT) + LN;
                let MV = if MU > MT { 1.0 } else { 0.0 };
                let MW = if MV != 0.0 {
                    MU
                } else {
                    MT
                };
                let MX = BL / MW;
                let MY = MS * BP;
                let NA = (MX - MR) - MZ;
                let NB = (IZ * MX) * MZ;
                let NC = if NB > AI { 1.0 } else { 0.0 };
                let NE = if NC != 0.0 {
                    NB
                } else {
                    let ND = -NB;
                    ND
                };
                let NF = MY * NA;
                let NG = ((NA * NA) + NE).sqrt();
                let NH = (((MY + ((NF + NF) * (DS / (ET * NG)))) * KF) * BP) * LL;
                let NI = (LL * (MX - (KF * (NA + NG)))) + LM;
                let NK = if (NI * 1e12f64) < NJ { 1.0 } else { 0.0 };
                let NL;
                let NM;
                if NK != 0.0 {
                    NL = AI;
                    NM = LY;
                } else {
                    NL = NI;
                    NM = NH;
                }
                let NN = NJ + NL;
                let NP = NO / NN;
                let NQ = ((NM * NP) * BP) / NN;
                let NR = NN / NO;
                let NS = NM / NO;
                let NT = GD * GD;
                let NU = GE * GD;
                let NV = NT * NR;
                let NW = NS * NT;
                let NX = NV * NR;
                let NY = NS * NV;
                let NZ = ((Lanes([((NU + NU) * NR), 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, NW[0], NW[1], NW[2], NW[3]])) * NR) + Lanes([0.0, NY[0], NY[1], NY[2], NY[3]]);
                MG = NR;
                MH = NX;
                MI = NP;
                MJ = NS;
                MK = NZ;
                ML = NQ;
            }
            let MM = JL * BP;
            let MO = (KF - JK) - MN;
            let OA = MM * MO;
            let OB = ((MO * MO) + staged[55]).sqrt();
            let OD = MJ * OC;
            let OE = (LS + (OC * MG)) + EB;
            let OF = Lanes([0.0, OD[0], OD[1], OD[2], OD[3]]) + Lanes([EC, 0.0, 0.0, 0.0, 0.0]);
            let OH = (((MM + ((OA + OA) * (DS / (ET * OB)))) * KF) * BP) * BP;
            let OI = (OG - (KF - (KF * (MO + OB)))) - MN;
            let OJ = OH * OI;
            let OK = ((OI * OI) + staged[58]).sqrt();
            let OL = LQ - (OG - (KF * (OI + OK)));
            let OM = (((OH + ((OJ + OJ) * (DS / (ET * OK)))) * KF) * BP) * BP;
            let ON = OL.sqrt();
            let OO = OM * (DS / (ET * ON));
            let PE;
            let PF;
            if OP != 0.0 {
                let OR = MJ * OQ;
                let OU = ((JS * MG) * OS) * OT;
                let OX = (parameters[66] + (OV * OL)) + (OW * JM);
                let OY = OE - (staged[60] + (OQ * MG));
                let OZ = OY * OU;
                let PA = (((MJ * JS) * OS) * OT) * OY;
                let PB = OZ * OX;
                let PC = ((OM * OV) + (JN * OW)) * OZ;
                let PD = ((((OF - Lanes([0.0, OR[0], OR[1], OR[2], OR[3]])) * OU) + Lanes([0.0, PA[0], PA[1], PA[2], PA[3]])) * OX) + Lanes([0.0, 0.0, PC[0], PC[1], PC[2]]);
                PE = PB;
                PF = PD;
            } else {
                PE = AI;
                PF = KJ;
            }
            let PO;
            let PP;
            if PG != 0.0 {
                let PI = Lanes([DG, 0.0, 0.0, 0.0, 0.0]) - (((MK * DA) + Lanes([(DB * MH), 0.0, 0.0, 0.0, 0.0])) * PH);
                let PJ = ((DF - ((MH * DA) * PH)) + JW) + IH;
                let PK = Lanes([0.0, JQ[0], JQ[1], JQ[2], JQ[3]]) - PI;
                let PM = (JO - PJ) - PL;
                let PN = if PJ >= AI { 1.0 } else { 0.0 };
                let QG = if PN != 0.0 {
                    BL
                } else {
                    QF
                };
                let QH = PK * PM;
                let QI = QG * IZ;
                let QJ = ((PM * PM) + ((QI * PJ) * PL)).sqrt();
                let QK = PI + ((PK + (((QH + QH) + ((PI * QI) * PL)) * (DS / (ET * QJ)))) * KF);
                let QL = (PJ + (KF * (PM + QJ))) - JW;
                let QM = IZ / MH;
                let QN = QM * DF;
                let QO = QN * DF;
                let QP = (DA * QL) - BL;
                let QQ = ((Lanes([(DB * QL), 0.0, 0.0, 0.0, 0.0]) + (QK * DA)) * QO) + ((((((((MK * QM) * BP) / MH) * DF) + Lanes([(DG * QM), 0.0, 0.0, 0.0, 0.0])) * DF) + Lanes([(DG * QN), 0.0, 0.0, 0.0, 0.0])) * QP);
                let QR = BL + (QP * QO);
                let QS = QQ * QR;
                let QT = ((QR * QR) + 4e-6f64).sqrt();
                let QU = (QQ + ((QS + QS) * (DS / (ET * QT)))) * KF;
                let QV = (KF * (QR + QT)) + 1e-13f64;
                let QW = if QV < AI { 1.0 } else { 0.0 };
                let QX;
                let QY;
                if QW != 0.0 {
                    QX = AI;
                    QY = KJ;
                } else {
                    QX = QV;
                    QY = QU;
                }
                let QZ = (QX + 2.220446049250313e-15f64).sqrt();
                let RA = MH * KF;
                let RB = RA * DA;
                let RC = BL - QZ;
                let RD = (QK + (((((MK * KF) * DA) + Lanes([(DB * RA), 0.0, 0.0, 0.0, 0.0])) * RC) + (((QY * (DS / (ET * QZ))) * BP) * RB))) * BP;
                let RE = (LQ - (QL + (RB * RC))) - PL;
                let RF = RD * RE;
                let RG = ((RE * RE) + staged[64]).sqrt();
                let RI = (((RD + ((RF + RF) * (DS / (ET * RG)))) * KF) * BP) * RH;
                let RJ = LQ + (RH * ((LQ - (KF * (RE + RG))) - LQ));
                PO = RJ;
                PP = RI;
            } else {
                PO = LQ;
                PP = KJ;
            }
            let PQ = MG * JS;
            let PR = MJ * JS;
            let PT = (PQ * PS) * FB;
            let PU = parameters[55] - PO;
            let PV = PP * BP;
            let PW = ((PR * PS) * FB) * PU;
            let PY = (PT * PU) / PX;
            let PZ = (Lanes([0.0, PW[0], PW[1], PW[2], PW[3]]) + (PV * PT)) / PX;
            let QA = HK * HI;
            let QB = ((HI * HI) + 4e-6f64).sqrt();
            let QC = (HK + ((QA + QA) * (DS / (ET * QB)))) * KF;
            let QD = (KF * (HI + QB)) + 1e-13f64;
            let QE = if QD < AI { 1.0 } else { 0.0 };
            let RK;
            let RL;
            if QE != 0.0 {
                RK = AI;
                RL = HH;
            } else {
                RK = QD;
                RL = QC;
            }
            let RP = ((parameters[69] + (RM * OL)) + (RN * JM)) + (RO * RK);
            let RQ = PY * RP;
            let RR = (((OM * RM) + (JN * RN)) + (RL * RO)) * PY;
            let RS = (PZ * RP) + Lanes([0.0, 0.0, RR[0], RR[1], RR[2]]);
            let SA;
            let SB;
            if RT != 0.0 {
                let RV = JN * RU;
                let RX = (((CV + FY) - staged[67]) + (RU * JM)) * RW;
                let RY = (Lanes([(CW + FZ), 0.0, 0.0, 0.0]) + Lanes([0.0, RV[0], RV[1], RV[2]])) * RW;
                SA = RX;
                SB = RY;
            } else {
                SA = AI;
                SB = RZ;
            }
            let SC = MI + staged[71];
            let SD = BL / SC;
            let SE = (MJ - (((ML * SD) * BP) / SC)) * OC;
            let SF = RQ + PE;
            let SG = RS + PF;
            let SH = (SG + Lanes([0.0, SE[0], SE[1], SE[2], SE[3]])) + Lanes([SB[0], 0.0, SB[1], SB[2], SB[3]]);
            let SI = ((SF + ((OC * (MG - SD)) + staged[72])) + SA) + staged[73];
            let SJ = OE - SI;
            let SN;
            let SO;
            if SK != 0.0 {
                SN = AI;
                SO = LY;
            } else {
                let SL = JO - parameters[76];
                let SM = if SL < -3e0f64 { 1.0 } else { 0.0 };
                let TD;
                let TE;
                if SM != 0.0 {
                    TD = AI;
                    TE = LY;
                } else {
                    let TC = if SL < AI { 1.0 } else { 0.0 };
                    let TW;
                    let TX;
                    if TC != 0.0 {
                        let TM = 3.333333333333333e-1f64 + (SL * TL);
                        let TN = BL + (SL * TM);
                        let TO = (JQ * TN) + (((JQ * TM) + ((JQ * TL) * SL)) * SL);
                        let TP = BL + (SL * TN);
                        TW = TP;
                        TX = TO;
                    } else {
                        let TR = 4.02052934513951e-2f64 + (SL * TQ);
                        let TS = 3.333333333333333e-1f64 + (SL * TR);
                        let TT = BL + (SL * TS);
                        let TU = (JQ * TT) + (((JQ * TS) + (((JQ * TR) + ((JQ * TQ) * SL)) * SL)) * SL);
                        let TV = BL + (SL * TT);
                        TW = TV;
                        TX = TU;
                    }
                    TD = TW;
                    TE = TX;
                }
                let TF = TD - BL;
                let TG = TE * TF;
                let TH = ((TF * TF) + 4.000000000000001e-2f64).sqrt();
                let TI = (TE + ((TG + TG) * (DS / (ET * TH)))) * KF;
                let TJ = (KF * (TF + TH)) + 1.0000000000000001e-11f64;
                let TK = if TJ < AI { 1.0 } else { 0.0 };
                let TY;
                let TZ;
                if TK != 0.0 {
                    TY = AI;
                    TZ = LY;
                } else {
                    TY = TJ;
                    TZ = TI;
                }
                let UB = (TZ * UA) * BP;
                let UC = (BL - (TY * UA)) - KP;
                let UD = UB * UC;
                let UE = ((UC * UC) + staged[75]).sqrt();
                let UF = BL - (KF * (UC + UE));
                let UG = ((UB + ((UD + UD) * (DS / (ET * UE)))) * KF) * BP;
                SN = UF;
                SO = UG;
            }
            let SP = (JX + SI) - SN;
            let SQ = Lanes([0.0, SO[0], SO[1], SO[2], SO[3]]);
            let SR = (KN + SH) - SQ;
            let ST = DF * SS;
            let SU = DG * SS;
            let SV = (JW - SI) + SN;
            let SW = GD * MG;
            let SX = MJ * GD;
            let SY = SW * SW;
            let SZ = (Lanes([(GE * MG), 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, SX[0], SX[1], SX[2], SX[3]])) * SW;
            let TA = SZ + SZ;
            let UL;
            let UM;
            if TB != 0.0 {
                let UH = JK + ST;
                let UI = Lanes([0.0, JL[0], JL[1], JL[2]]) + Lanes([SU, 0.0, 0.0, 0.0]);
                UL = UH;
                UM = UI;
            } else {
                let UJ = HI + ST;
                let UK = Lanes([0.0, HK[0], HK[1], HK[2]]) + Lanes([SU, 0.0, 0.0, 0.0]);
                UL = UJ;
                UM = UK;
            }
            let UN = if UL < AI { 1.0 } else { 0.0 };
            if UN != 0.0 {
                let UO = GM / FV;
                let UP = UO + BL;
                let UQ = (DF - UL) + (UO * (DF + UL));
                let US = ((GG * GG) * UR) * UR;
                let UT = US * DA;
                let UU = ((FB * UQ) * UP) - UT;
                let UV = if ((UU * UU) - (((IZ * UP) * UP) * (((UQ * UQ) + (UT * UL)) + US))) >= IH { 1.0 } else { 0.0 };
            } else {
                let UW = GD * GD;
                let UX = -(DF + (FB * UL));
                let UY = BL + ((UW * DA) / ((GG * GG) * DA));
                let UZ = (((UW * UR) * UR) * DA) - ((FB * UX) * UY);
                let VA = if ((UZ * UZ) - ((((IZ * UY) * UY) * UX) * UX)) >= IH { 1.0 } else { 0.0 };
            }
            let VB = FB / DA;
            let VC = ((DB * VB) * BP) / DA;
            let VD = VB * ((GM / FE).ln());
            let VE = GH * GG;
            let VG = ((GG * GG) * VF) * VF;
            let VH = ((VE + VE) * VF) * VF;
            let VI = -UL;
            let VJ = UM * BP;
            let VK = VG * DA;
            let VL = (VH * DA) + (DB * VG);
            let VM = (FB * VI) + VK;
            let VN = (VJ * FB) + Lanes([VL, 0.0, 0.0, 0.0]);
            let VO = VI * VI;
            let VP = VJ * VI;
            let VQ = VP + VP;
            let VR = Lanes([VH, 0.0, 0.0, 0.0]);
            let VS = (VQ + VR) * IZ;
            let VT = (VM * VM) - (IZ * (VO + VG));
            let VU = if VT >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
            let VW = if VU != 0.0 {
                VT
            } else {
                VV
            };
            let VX = (VM - (VW.sqrt())) / FB;
            let VY = VO / VG;
            let VZ = (VQ - Lanes([(VH * VY), 0.0, 0.0, 0.0])) / VG;
            let WA = VY / GO;
            let WB = Lanes([(GQ * WA), 0.0, 0.0, 0.0]);
            let WC = DS / WA;
            let WD = FB / VI;
            let WE = DA + WD;
            let WF = Lanes([DB, 0.0, 0.0, 0.0]);
            let WG = (WA.ln()) / WE;
            let WH = (WF + (((VJ * WD) * BP) / VI)) * WG;
            let WI = if VX < VD { 1.0 } else { 0.0 };
            let WN;
            if WI != 0.0 {
                WN = VX;
            } else {
                let WK = (WG - VX) - WJ;
                let WL = (IZ * WG) * WJ;
                let WM = if WL > AI { 1.0 } else { 0.0 };
                let WP = if WM != 0.0 {
                    WL
                } else {
                    let WO = -WL;
                    WO
                };
                let WQ = WG - (KF * (WK + (((WK * WK) + WP).sqrt())));
                WN = WQ;
            }
            let mut WR = 0.0;
            let mut WS = 0.0;
            let mut WT = 0.0;
            let mut WU = 0.0;
            WR = AI;
            WS = WN;
            WT = AI;
            WU = AI;
            loop {
                let WW = if WR < WV { 1.0 } else { 0.0 };
                if WW == 0.0 {
                    break;
                }
                let WX = DA * WS;
                let WY = (-WX).exp();
                let XA = if WS > WZ { 1.0 } else { 0.0 };
                let XH;
                let XI;
                if XA != 0.0 {
                    let XC = WX.exp();
                    let XD = (-GG) * ((((WY + WX) - BL) + (GO * (XC - BL))).sqrt());
                    let XF = (XE / XD) * (((-WY) + BL) + (GO * XC));
                    XH = XD;
                    XI = XF;
                } else {
                    let XG = if WS < -1e-8f64 { 1.0 } else { 0.0 };
                    let XR;
                    let XS;
                    if XG != 0.0 {
                        let XN = GG * (((WY + WX) - BL).sqrt());
                        let XO = (XE / XN) * ((-WY) + BL);
                        XR = XN;
                        XS = XO;
                    } else {
                        let XP = ((-((XE / DA).sqrt())) * DA) * WS;
                        let XQ = -((XE * DA).sqrt());
                        XR = XP;
                        XS = XQ;
                    }
                    XH = XR;
                    XI = XS;
                }
                let XJ = ((XH * XH) + 4e-12f64).sqrt();
                let XK = KF * (BL + (XH / XJ));
                let XL = (KF * (XH + XJ)) + 1e-16f64;
                let XM = if XL < AI { 1.0 } else { 0.0 };
                let XT;
                let XU;
                if XM != 0.0 {
                    XT = AI;
                    XU = AI;
                } else {
                    XT = XL;
                    XU = XK;
                }
                let XW = -XV;
                let XX = (XW - XT) - AR;
                let XY = (IZ * XW) * AR;
                let XZ = if XY > AI { 1.0 } else { 0.0 };
                let YB = if XZ != 0.0 {
                    XY
                } else {
                    let YA = -XY;
                    YA
                };
                let YC = ((XX * XX) + YB).sqrt();
                let YD = XW - (KF * (XX + YC));
                let YE = ((((YD * YD) / FB) / JS) / CZ) / FV;
                let YG = WS - (((((-WS) + (XH / YF)) - UL) + YE) / ((-1e0f64 + (XI / YF)) + (((FB * YE) * (XU * (XI * (KF * (BL + (XX / YC)))))) / YD)));
                let YH = if ((YG - WS).abs()) < MN { 1.0 } else { 0.0 };
                let YI = if YH != 0.0 {
                    WV
                } else {
                    WR
                };
                let YJ = YI + BL;
                WR = YJ;
                WS = YG;
                WT = YE;
                WU = XH;
            }
            let XB = if (((1.2919089961638799e9f64 * WT) / FV).sqrt()) > staged[82] { 1.0 } else { 0.0 };
            let ZF;
            let ZG;
            let ZH;
            let ZI;
            let ZJ;
            let ZK;
            let ZL;
            if XB != 0.0 {
                let YK = BL / MI;
                let YL = ((ML * YK) * BP) / MI;
                let YM = BL / YF;
                let YO = (YK + YN) + YM;
                let YP = BL / YO;
                let YQ = YL * YP;
                let YR = (YQ * BP) / YO;
                let YS = BL - (YP * YK);
                let YT = VI + ((YM + (KF * YN)) * (-XV));
                let YU = YP * YT;
                let YV = YR * YT;
                let YW = VJ * YP;
                let YX = YL * YU;
                let YY = (YK * YU) / YS;
                let YZ = (((YR * YK) + YQ) * BP) * YY;
                let ZA = ((Lanes([0.0, YX[0], YX[1], YX[2], YX[3]]) + ((Lanes([0.0, YV[0], YV[1], YV[2], YV[3]]) + Lanes([YW[0], 0.0, YW[1], YW[2], YW[3]])) * YK)) - Lanes([0.0, YZ[0], YZ[1], YZ[2], YZ[3]])) / YS;
                let ZB = SV + YY;
                let ZD = SP - (ZC * YY);
                let ZE = SR - (ZA * ZC);
                ZF = ZB;
                ZG = ZD;
                ZH = YY;
                ZI = ZD;
                ZJ = ZE;
                ZK = ZA;
                ZL = ZE;
            } else {
                ZF = SV;
                ZG = SP;
                ZH = AI;
                ZI = SP;
                ZJ = SR;
                ZK = KJ;
                ZL = SR;
            }
            let ZP;
            let ZQ;
            let ZR;
            let ZS;
            let ZT;
            let ZU;
            let ZV;
            let ZW;
            let ZX;
            let ZY;
            let ZZ;
            let AAA;
            let AAB;
            let AAC;
            if ZM != 0.0 {
                let ZN = (staged[84] + DF) - (WU * YN);
                let ZO = Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]]);
                ZP = AI;
                ZQ = AI;
                ZR = UL;
                ZS = AI;
                ZT = AI;
                ZU = AI;
                ZV = ZN;
                ZW = AI;
                ZX = KJ;
                ZY = KJ;
                ZZ = ZO;
                AAA = KJ;
                AAB = KJ;
                AAC = KJ;
            } else {
                let AAF;
                if UN != 0.0 {
                    let mut AAH = 0.0;
                    let mut AAI = 0.0;
                    AAH = BL;
                    AAI = AI;
                    loop {
                        let AAJ = if AAH <= WV { 1.0 } else { 0.0 };
                        if AAJ == 0.0 {
                            break;
                        }
                        let AAK = YF / (3.3163543761348e-29f64 * GM);
                        let AAL = BL + (YF * YN);
                        let AAM = FB * AAK;
                        let AAN = AAM * MI;
                        let AAO = AAN * MI;
                        let AAP = (FB * YF) * MI;
                        let AAQ = ((AAP * FB) * AAK) * MI;
                        let AAR = ((((YF * YF) + ((((AAL * AAL) - ((IZ * AAK) * (YF * ((((KF * (-XV)) * YN) + DF) + UL)))) * MI) * MI)) + (AAP * (AAL + (AAM * XV)))) + (AAQ * AAI)).sqrt();
                        let AAS = BL / AAO;
                        let AAT = (-(AAS * ((((YF + (AAL * MI)) + (AAN * XV)) + (AAO * AAI)) - AAR))) / (AAS * (AAO - (AAQ / (FB * AAR))));
                        let AAU = if (AAT.abs()) < HZ { 1.0 } else { 0.0 };
                        let AAW;
                        let AAX;
                        if AAU != 0.0 {
                            AAW = AAT;
                            AAX = WV;
                        } else {
                            let AAV = if AAT > EH { 1.0 } else { 0.0 };
                            let ABB;
                            if AAV != 0.0 {
                                ABB = EH;
                            } else {
                                let ABA = if AAT < -1e-1f64 { 1.0 } else { 0.0 };
                                let ABD = if ABA != 0.0 {
                                    ABC
                                } else {
                                    AAT
                                };
                                ABB = ABD;
                            }
                            AAW = ABB;
                            AAX = AAH;
                        }
                        let AAY = AAI + AAW;
                        let AAZ = AAX + BL;
                        AAH = AAZ;
                        AAI = AAY;
                    }
                    AAF = AAI;
                } else {
                    AAF = AI;
                }
                let AAG = if CC < (ZF + AAF) { 1.0 } else { 0.0 };
                let ABF;
                let ABG;
                let ABH;
                if AAG != 0.0 {
                    let ABE = if (((1.2919089961638799e9f64 * WT) / FV).sqrt()) < PS { 1.0 } else { 0.0 };
                    let ACG;
                    let ACH;
                    if ABE != 0.0 {
                        let ABO = VI + 2.220446049250313e-15f64;
                        let ABP = (FB * ABO) + VK;
                        let ABQ = VN * ABP;
                        let ABR = ABO * ABO;
                        let ABS = VJ * ABO;
                        let ABT = ABS + ABS;
                        let ABU = (ABP * ABP) - (IZ * (ABR + VG));
                        let ABV = (ABQ + ABQ) - ((ABT + VR) * IZ);
                        let ABW = if ABU >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let ACK;
                        let ACL;
                        if ABW != 0.0 {
                            ACK = ABU;
                            ACL = ABV;
                        } else {
                            ACK = ACJ;
                            ACL = RZ;
                        }
                        let ACM = ACK.sqrt();
                        let ACN = (ABP - ACM) / FB;
                        let ACO = (VN - (ACL * (DS / (ET * ACM)))) / FB;
                        let ACP = ABR / VG;
                        let ACQ = ACP / GO;
                        let ACR = FB / ABO;
                        let ACS = DA + ACR;
                        let ACT = (ACQ.ln()) / ACS;
                        let ACU = ((((((ABT - Lanes([(VH * ACP), 0.0, 0.0, 0.0])) / VG) - Lanes([(GQ * ACQ), 0.0, 0.0, 0.0])) / GO) * (DS / ACQ)) - ((WF + (((VJ * ACR) * BP) / ABO)) * ACT)) / ACS;
                        let ACV = if ACN < VD { 1.0 } else { 0.0 };
                        let ADB;
                        let ADC;
                        if ACV != 0.0 {
                            ADB = ACN;
                            ADC = ACO;
                        } else {
                            let ACW = ACU - ACO;
                            let ACX = (ACT - ACN) - WJ;
                            let ACY = (IZ * ACT) * WJ;
                            let ACZ = (ACU * IZ) * WJ;
                            let ADA = if ACY > AI { 1.0 } else { 0.0 };
                            let ADF;
                            let ADG;
                            if ADA != 0.0 {
                                ADF = ACY;
                                ADG = ACZ;
                            } else {
                                let ADD = -ACY;
                                let ADE = ACZ * BP;
                                ADF = ADD;
                                ADG = ADE;
                            }
                            let ADH = ACW * ACX;
                            let ADI = ((ACX * ACX) + ADF).sqrt();
                            let ADJ = ACT - (KF * (ACX + ADI));
                            let ADK = ACU - ((ACW + (((ADH + ADH) + ADG) * (DS / (ET * ADI)))) * KF);
                            ADB = ADJ;
                            ADC = ADK;
                        }
                        ACG = ADB;
                        ACH = ADC;
                    } else {
                        let ABX = -(UL - (((XV / FB) * PS) / JS));
                        let ABY = (FB * ABX) + VK;
                        let ABZ = VN * ABY;
                        let ACA = ABX * ABX;
                        let ACB = VJ * ABX;
                        let ACC = ACB + ACB;
                        let ACD = (ABY * ABY) - (IZ * (ACA + VG));
                        let ACE = (ABZ + ABZ) - ((ACC + VR) * IZ);
                        let ACF = if ACD >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let ADM;
                        let ADN;
                        if ACF != 0.0 {
                            ADM = ACD;
                            ADN = ACE;
                        } else {
                            ADM = ADL;
                            ADN = RZ;
                        }
                        let ADO = ADM.sqrt();
                        let ADP = (ABY - ADO) / FB;
                        let ADQ = (VN - (ADN * (DS / (ET * ADO)))) / FB;
                        let ADR = ACA / VG;
                        let ADS = ADR / GO;
                        let ADT = FB / ABX;
                        let ADU = DA + ADT;
                        let ADV = (ADS.ln()) / ADU;
                        let ADW = ((((((ACC - Lanes([(VH * ADR), 0.0, 0.0, 0.0])) / VG) - Lanes([(GQ * ADS), 0.0, 0.0, 0.0])) / GO) * (DS / ADS)) - ((WF + (((VJ * ADT) * BP) / ABX)) * ADV)) / ADU;
                        let ADX = if ADP < VD { 1.0 } else { 0.0 };
                        let AED;
                        let AEE;
                        if ADX != 0.0 {
                            AED = ADP;
                            AEE = ADQ;
                        } else {
                            let ADY = ADW - ADQ;
                            let ADZ = (ADV - ADP) - WJ;
                            let AEA = (IZ * ADV) * WJ;
                            let AEB = (ADW * IZ) * WJ;
                            let AEC = if AEA > AI { 1.0 } else { 0.0 };
                            let AEH;
                            let AEI;
                            if AEC != 0.0 {
                                AEH = AEA;
                                AEI = AEB;
                            } else {
                                let AEF = -AEA;
                                let AEG = AEB * BP;
                                AEH = AEF;
                                AEI = AEG;
                            }
                            let AEJ = ADY * ADZ;
                            let AEK = ((ADZ * ADZ) + AEH).sqrt();
                            let AEL = ADV - (KF * (ADZ + AEK));
                            let AEM = ADW - ((ADY + (((AEJ + AEJ) + AEI) * (DS / (ET * AEK)))) * KF);
                            AED = AEL;
                            AEE = AEM;
                        }
                        ACG = AED;
                        ACH = AEE;
                    }
                    let ACI = if (((1.2919089961638799e9f64 * WT) / FV).sqrt()) < PS { 1.0 } else { 0.0 };
                    let AEN;
                    let AEO;
                    if ACI != 0.0 {
                        let mut AEQ = 0.0;
                        let mut AER = 0.0;
                        let mut AES = 0.0;
                        let mut AET = Lanes([0.0; 4]);
                        let mut AEU = Lanes([0.0; 4]);
                        AEQ = AI;
                        AER = ACG;
                        AES = AI;
                        AET = ACH;
                        AEU = RZ;
                        loop {
                            let AEV = if AEQ < WV { 1.0 } else { 0.0 };
                            if AEV == 0.0 {
                                break;
                            }
                            let AEW = DA * AER;
                            let AEX = Lanes([(DB * AER), 0.0, 0.0, 0.0]) + (AET * DA);
                            let AEY = (-AEW).exp();
                            let AEZ = (AEX * BP) * AEY;
                            let AFA = if AER > WZ { 1.0 } else { 0.0 };
                            let AFN;
                            let AFO;
                            let AFP;
                            let AFQ;
                            if AFA != 0.0 {
                                let AFB = AEW.exp();
                                let AFC = -GG;
                                let AFD = AFB - BL;
                                let AFE = (AEX * AFB) * GO;
                                let AFF = (((AEY + AEW) - BL) + (GO * AFD)).sqrt();
                                let AFG = AFC * AFF;
                                let AFH = Lanes([((GH * BP) * AFF), 0.0, 0.0, 0.0]) + ((((AEZ + AEX) + (Lanes([(GQ * AFD), 0.0, 0.0, 0.0]) + AFE)) * (DS / (ET * AFF))) * AFC);
                                let AFI = XE / AFG;
                                let AFJ = ((-AEY) + BL) + (GO * AFB);
                                let AFK = AFI * AFJ;
                                let AFL = ((((AFH * AFI) * BP) / AFG) * AFJ) + (((AEZ * BP) + (Lanes([(GQ * AFB), 0.0, 0.0, 0.0]) + AFE)) * AFI);
                                AFN = AFG;
                                AFO = AFK;
                                AFP = AFH;
                                AFQ = AFL;
                            } else {
                                let AFM = if AER < -1e-8f64 { 1.0 } else { 0.0 };
                                let AGQ;
                                let AGR;
                                let AGS;
                                let AGT;
                                if AFM != 0.0 {
                                    let AGA = ((AEY + AEW) - BL).sqrt();
                                    let AGB = GG * AGA;
                                    let AGC = Lanes([(GH * AGA), 0.0, 0.0, 0.0]) + (((AEZ + AEX) * (DS / (ET * AGA))) * GG);
                                    let AGD = XE / AGB;
                                    let AGE = (-AEY) + BL;
                                    let AGF = AGD * AGE;
                                    let AGG = ((((AGC * AGD) * BP) / AGB) * AGE) + ((AEZ * BP) * AGD);
                                    AGQ = AGB;
                                    AGR = AGF;
                                    AGS = AGC;
                                    AGT = AGG;
                                } else {
                                    let AGH = XE / DA;
                                    let AGI = AGH.sqrt();
                                    let AGJ = -AGI;
                                    let AGK = AGJ * DA;
                                    let AGL = AGK * AER;
                                    let AGM = Lanes([((((((((DB * AGH) * BP) / DA) * (DS / (ET * AGI))) * BP) * DA) + (DB * AGJ)) * AER), 0.0, 0.0, 0.0]) + (AET * AGK);
                                    let AGN = (XE * DA).sqrt();
                                    let AGO = -AGN;
                                    let AGP = Lanes([(((DB * XE) * (DS / (ET * AGN))) * BP), 0.0, 0.0, 0.0]);
                                    AGQ = AGL;
                                    AGR = AGO;
                                    AGS = AGM;
                                    AGT = AGP;
                                }
                                AFN = AGQ;
                                AFO = AGR;
                                AFP = AGS;
                                AFQ = AGT;
                            }
                            let AFR = AFP * AFN;
                            let AFS = ((AFN * AFN) + 4.0000000000000004e-20f64).sqrt();
                            let AFT = (AFR + AFR) * (DS / (ET * AFS));
                            let AFU = AFN / AFS;
                            let AFV = KF * (BL + AFU);
                            let AFW = ((AFP - (AFT * AFU)) / AFS) * KF;
                            let AFX = (AFP + AFT) * KF;
                            let AFY = (KF * (AFN + AFS)) + 1.0000000000000001e-20f64;
                            let AFZ = if AFY < AI { 1.0 } else { 0.0 };
                            let AGU;
                            let AGV;
                            let AGW;
                            let AGX;
                            if AFZ != 0.0 {
                                AGU = AI;
                                AGV = AI;
                                AGW = RZ;
                                AGX = RZ;
                            } else {
                                AGU = AFY;
                                AGV = AFV;
                                AGW = AFX;
                                AGX = AFW;
                            }
                            let AGY = -XV;
                            let AGZ = AGW * BP;
                            let AHB = (AGY - AGU) - AHA;
                            let AHC = (IZ * AGY) * AHA;
                            let AHD = if AHC > AI { 1.0 } else { 0.0 };
                            let AHF = if AHD != 0.0 {
                                AHC
                            } else {
                                let AHE = -AHC;
                                AHE
                            };
                            let AHG = AGZ * AHB;
                            let AHH = ((AHB * AHB) + AHF).sqrt();
                            let AHI = (AHG + AHG) * (DS / (ET * AHH));
                            let AHJ = AHB / AHH;
                            let AHK = KF * (BL + AHJ);
                            let AHL = AGY - (KF * (AHB + AHH));
                            let AHM = ((AGZ + AHI) * KF) * BP;
                            let AHN = AFO * AHK;
                            let AHO = AGV * AHN;
                            let AHP = AHM * AHL;
                            let AHQ = ((((AHL * AHL) / FB) / JS) / CZ) / FV;
                            let AHR = ((((AHP + AHP) / FB) / JS) / CZ) / FV;
                            let AHS = FB * AHQ;
                            let AHT = (AHS * AHO) / AHL;
                            let AHU = (-1e0f64 + (AFO / YF)) + AHT;
                            let AHV = ((((-AER) + (AFN / YF)) - UL) + AHQ) / AHU;
                            let AHW = AER - AHV;
                            let AHX = AET - ((((((AET * BP) + (AFP / YF)) - UM) + AHR) - (((AFQ / YF) + (((((AHR * FB) * AHO) + (((AGX * AHN) + (((AFQ * AHK) + ((((AGZ - (AHI * AHJ)) / AHH) * KF) * AFO)) * AGV)) * AHS)) - (AHM * AHT)) / AHL)) * AHV)) / AHU);
                            let AHY = if ((AHW - AER).abs()) < MN { 1.0 } else { 0.0 };
                            let AHZ = if AHY != 0.0 {
                                WV
                            } else {
                                AEQ
                            };
                            let AIA = AHZ + BL;
                            AEQ = AIA;
                            AER = AHW;
                            AES = AFN;
                            AET = AHX;
                            AEU = AFP;
                        }
                        AEN = AES;
                        AEO = AEU;
                    } else {
                        let mut AIB = 0.0;
                        let mut AIC = 0.0;
                        let mut AID = 0.0;
                        let mut AIE = Lanes([0.0; 4]);
                        let mut AIF = Lanes([0.0; 4]);
                        AIB = AI;
                        AIC = ACG;
                        AID = AI;
                        AIE = ACH;
                        AIF = RZ;
                        loop {
                            let AIG = if AIB < WV { 1.0 } else { 0.0 };
                            if AIG == 0.0 {
                                break;
                            }
                            let AIH = DA * AIC;
                            let AII = Lanes([(DB * AIC), 0.0, 0.0, 0.0]) + (AIE * DA);
                            let AIJ = (-AIH).exp();
                            let AIK = (AII * BP) * AIJ;
                            let AIL = if AIC > WZ { 1.0 } else { 0.0 };
                            let AIY;
                            let AIZ;
                            let AJA;
                            let AJB;
                            if AIL != 0.0 {
                                let AIM = AIH.exp();
                                let AIN = -GG;
                                let AIO = AIM - BL;
                                let AIP = (AII * AIM) * GO;
                                let AIQ = (((AIJ + AIH) - BL) + (GO * AIO)).sqrt();
                                let AIR = AIN * AIQ;
                                let AIS = Lanes([((GH * BP) * AIQ), 0.0, 0.0, 0.0]) + ((((AIK + AII) + (Lanes([(GQ * AIO), 0.0, 0.0, 0.0]) + AIP)) * (DS / (ET * AIQ))) * AIN);
                                let AIT = XE / AIR;
                                let AIU = ((-AIJ) + BL) + (GO * AIM);
                                let AIV = AIT * AIU;
                                let AIW = ((((AIS * AIT) * BP) / AIR) * AIU) + (((AIK * BP) + (Lanes([(GQ * AIM), 0.0, 0.0, 0.0]) + AIP)) * AIT);
                                AIY = AIR;
                                AIZ = AIV;
                                AJA = AIS;
                                AJB = AIW;
                            } else {
                                let AIX = if AIC < -1e-8f64 { 1.0 } else { 0.0 };
                                let AKB;
                                let AKC;
                                let AKD;
                                let AKE;
                                if AIX != 0.0 {
                                    let AJL = ((AIJ + AIH) - BL).sqrt();
                                    let AJM = GG * AJL;
                                    let AJN = Lanes([(GH * AJL), 0.0, 0.0, 0.0]) + (((AIK + AII) * (DS / (ET * AJL))) * GG);
                                    let AJO = XE / AJM;
                                    let AJP = (-AIJ) + BL;
                                    let AJQ = AJO * AJP;
                                    let AJR = ((((AJN * AJO) * BP) / AJM) * AJP) + ((AIK * BP) * AJO);
                                    AKB = AJM;
                                    AKC = AJQ;
                                    AKD = AJN;
                                    AKE = AJR;
                                } else {
                                    let AJS = XE / DA;
                                    let AJT = AJS.sqrt();
                                    let AJU = -AJT;
                                    let AJV = AJU * DA;
                                    let AJW = AJV * AIC;
                                    let AJX = Lanes([((((((((DB * AJS) * BP) / DA) * (DS / (ET * AJT))) * BP) * DA) + (DB * AJU)) * AIC), 0.0, 0.0, 0.0]) + (AIE * AJV);
                                    let AJY = (XE * DA).sqrt();
                                    let AJZ = -AJY;
                                    let AKA = Lanes([(((DB * XE) * (DS / (ET * AJY))) * BP), 0.0, 0.0, 0.0]);
                                    AKB = AJW;
                                    AKC = AJZ;
                                    AKD = AJX;
                                    AKE = AKA;
                                }
                                AIY = AKB;
                                AIZ = AKC;
                                AJA = AKD;
                                AJB = AKE;
                            }
                            let AJC = AJA * AIY;
                            let AJD = ((AIY * AIY) + 4.0000000000000004e-20f64).sqrt();
                            let AJE = (AJC + AJC) * (DS / (ET * AJD));
                            let AJF = AIY / AJD;
                            let AJG = KF * (BL + AJF);
                            let AJH = ((AJA - (AJE * AJF)) / AJD) * KF;
                            let AJI = (AJA + AJE) * KF;
                            let AJJ = (KF * (AIY + AJD)) + 1.0000000000000001e-20f64;
                            let AJK = if AJJ < AI { 1.0 } else { 0.0 };
                            let AKF;
                            let AKG;
                            let AKH;
                            let AKI;
                            if AJK != 0.0 {
                                AKF = AI;
                                AKG = AI;
                                AKH = RZ;
                                AKI = RZ;
                            } else {
                                AKF = AJJ;
                                AKG = AJG;
                                AKH = AJI;
                                AKI = AJH;
                            }
                            let AKJ = -XV;
                            let AKK = AKH * BP;
                            let AKL = (AKJ - AKF) - AHA;
                            let AKM = (IZ * AKJ) * AHA;
                            let AKN = if AKM > AI { 1.0 } else { 0.0 };
                            let AKP = if AKN != 0.0 {
                                AKM
                            } else {
                                let AKO = -AKM;
                                AKO
                            };
                            let AKQ = AKK * AKL;
                            let AKR = ((AKL * AKL) + AKP).sqrt();
                            let AKS = (AKQ + AKQ) * (DS / (ET * AKR));
                            let AKT = AKL / AKR;
                            let AKU = KF * (BL + AKT);
                            let AKV = AKJ - (KF * (AKL + AKR));
                            let AKW = ((AKK + AKS) * KF) * BP;
                            let AKX = AIZ * AKU;
                            let AKY = AKG * AKX;
                            let AKZ = AKW * AKV;
                            let ALA = ((((AKV * AKV) / FB) / JS) / CZ) / FV;
                            let ALB = ((((AKZ + AKZ) / FB) / JS) / CZ) / FV;
                            let ALC = FB * ALA;
                            let ALD = (ALC * AKY) / AKV;
                            let ALE = ((-1e0f64 + (AIZ / YF)) + ((AIZ * PS) / JS)) + ALD;
                            let ALF = (((((AI - AIC) + (AIY / YF)) + (((AIY + (XV / FB)) * PS) / JS)) - UL) + ALA) / ALE;
                            let ALG = AIC - ALF;
                            let ALH = AIE - (((((((AIE * BP) + (AJA / YF)) + ((AJA * PS) / JS)) - UM) + ALB) - ((((AJB / YF) + ((AJB * PS) / JS)) + (((((ALB * FB) * AKY) + (((AKI * AKX) + (((AJB * AKU) + ((((AKK - (AKS * AKT)) / AKR) * KF) * AIZ)) * AKG)) * ALC)) - (AKW * ALD)) / AKV)) * ALF)) / ALE);
                            let ALI = if ((ALG - AIC).abs()) < MN { 1.0 } else { 0.0 };
                            let ALJ = if ALI != 0.0 {
                                WV
                            } else {
                                AIB
                            };
                            let ALK = ALJ + BL;
                            AIB = ALK;
                            AIC = ALG;
                            AID = AIY;
                            AIE = ALH;
                            AIF = AJA;
                        }
                        AEN = AID;
                        AEO = AIF;
                    }
                    ABF = AEN;
                    ABG = AEP;
                    ABH = AEO;
                } else {
                    ABF = AI;
                    ABG = AI;
                    ABH = RZ;
                }
                let ABI = ZG - HI;
                let ABJ = SY * DC;
                let ABK = (IZ * ((DA * ABI) - BL)) / ABJ;
                let ABL = (((Lanes([(DB * ABI), 0.0, 0.0, 0.0, 0.0]) + ((ZJ - KA) * DA)) * IZ) - (((TA * DC) + Lanes([(DE * SY), 0.0, 0.0, 0.0, 0.0])) * ABK)) / ABJ;
                let ABM = BL + ABK;
                let ABN = if ABM >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let ALM;
                let ALN;
                if ABN != 0.0 {
                    ALM = ABM;
                    ALN = ABL;
                } else {
                    ALM = ALL;
                    ALN = KJ;
                }
                let ALO = (SY * DA) * KF;
                let ALP = ALM.sqrt();
                let ALQ = BL - ALP;
                let ALR = ZG + (ALO * ALQ);
                let ALS = ZJ + (((((TA * DA) + Lanes([(DB * SY), 0.0, 0.0, 0.0, 0.0])) * KF) * ALQ) + (((ALN * (DS / (ET * ALP))) * BP) * ALO));
                let ALT = BL / MI;
                let ALW = (ALT + ALU) + ALV;
                let ALX = BL / ALW;
                let ALY = (((((ML * ALT) * BP) / MI) * ALX) * BP) / ALW;
                let ALZ = CC - ZH;
                let AMA = if ALZ <= SJ { 1.0 } else { 0.0 };
                let AMG;
                let AMH;
                if AMA != 0.0 {
                    let AMB = if ALR > AI { 1.0 } else { 0.0 };
                    let AMQ;
                    let AMR;
                    if AMB != 0.0 {
                        let AMN = ((CZ * FV) * FB) * JS;
                        let AMO = (AMN * ALR).sqrt();
                        let AMP = (ALS * AMN) * (DS / (ET * AMO));
                        AMQ = AMO;
                        AMR = AMP;
                    } else {
                        AMQ = AI;
                        AMR = KJ;
                    }
                    let AMS = if XV <= AMQ { 1.0 } else { 0.0 };
                    let AMT;
                    let AMU;
                    if AMS != 0.0 {
                        AMT = XV;
                        AMU = KJ;
                    } else {
                        AMT = AMQ;
                        AMU = AMR;
                    }
                    let AMV = ALV + (KF * ALU);
                    let AMW = (ZG - UL) + (AMV * (-AMT));
                    let AMX = ALX * AMW;
                    let AMY = ALY * AMW;
                    let AMZ = Lanes([0.0, AMY[0], AMY[1], AMY[2], AMY[3]]) + (((ZJ - Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]])) + ((AMU * BP) * AMV)) * ALX);
                    AMG = AMX;
                    AMH = AMZ;
                } else {
                    let AMC = (ZG - UL) + ((ALV + (KF * ALU)) * (-XV));
                    let AMD = ALX * AMC;
                    let AME = ALY * AMC;
                    let AMF = Lanes([0.0, AME[0], AME[1], AME[2], AME[3]]) + ((ZJ - Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]])) * ALX);
                    AMG = AMD;
                    AMH = AMF;
                }
                let AMI = AMG / MI;
                let AMJ = ML * AMI;
                let AMK = ZG - AMI;
                let AML = ZJ - ((AMH - Lanes([0.0, AMJ[0], AMJ[1], AMJ[2], AMJ[3]])) / MI);
                let AMM = if ALZ > SJ { 1.0 } else { 0.0 };
                let ANN;
                let ANO;
                if AMM != 0.0 {
                    let ANA = BL / GJ;
                    let ANB = ANA / MH;
                    let ANC = ZG - ZH;
                    let AND = ZJ - ZK;
                    let ANE = ANB * ANC;
                    let ANF = ANE * ANC;
                    let ANG = FB / ANC;
                    let ANH = DA + ANG;
                    let ANI = (ANF.ln()) / ANH;
                    let ANJ = ((((((((Lanes([(((GL * ANA) * BP) / GJ), 0.0, 0.0, 0.0, 0.0]) - (MK * ANB)) / MH) * ANC) + (AND * ANB)) * ANC) + (AND * ANE)) * (DS / ANF)) - ((Lanes([DB, 0.0, 0.0, 0.0, 0.0]) + (((AND * ANG) * BP) / ANC)) * ANI)) / ANH;
                    let ANL = ANI - ANK;
                    let ANM = if (if AMK > ANL { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                    let ANW;
                    let ANX;
                    if ANM != 0.0 {
                        let ANQ = AML - ANJ;
                        let ANR = (AMK - ANI) + ANK;
                        let ANS = ANQ * ANR;
                        let ANT = ANS + ANS;
                        let ANU = (ANR * ANR) + 2.25e-2f64;
                        let AOB;
                        let AOC;
                        if ANV != 0.0 {
                            let AOJ;
                            if ANY != 0.0 {
                                AOJ = BL;
                            } else {
                                let AOL;
                                if AOI != 0.0 {
                                    AOL = FB;
                                } else {
                                    let AON;
                                    if AOK != 0.0 {
                                        AON = IW;
                                    } else {
                                        let AOO = if AOM != 0.0 {
                                            IZ
                                        } else {
                                            AI
                                        };
                                        AON = AOO;
                                    }
                                    AOL = AON;
                                }
                                AOJ = AOL;
                            }
                            let mut AOP = 0.0;
                            let mut AOQ = 0.0;
                            let mut AOR = Lanes([0.0; 5]);
                            AOP = AI;
                            AOQ = ANU;
                            AOR = ANT;
                            loop {
                                let AOS = if AOP < AOJ { 1.0 } else { 0.0 };
                                if AOS == 0.0 {
                                    break;
                                }
                                let AOT = AOQ.sqrt();
                                let AOU = AOR * (DS / (ET * AOT));
                                let AOV = AOP + BL;
                                AOP = AOV;
                                AOQ = AOT;
                                AOR = AOU;
                            }
                            AOB = AOQ;
                            AOC = AOR;
                        } else {
                            let ANZ = ANU.sqrt();
                            let AOA = ANT * (5e-1f64 * (ANU.powf(-5e-1f64)));
                            AOB = ANZ;
                            AOC = AOA;
                        }
                        let AOD = AOB + IH;
                        let AOE = BL / AOD;
                        let AOF = ANR * ANK;
                        let AOG = ANL + (AOF * AOE);
                        let AOH = ANJ + (((ANQ * ANK) * AOE) + ((((AOC * AOE) * BP) / AOD) * AOF));
                        ANW = AOG;
                        ANX = AOH;
                    } else {
                        ANW = AMK;
                        ANX = AML;
                    }
                    ANN = ANW;
                    ANO = ANX;
                } else {
                    ANN = AMK;
                    ANO = AML;
                }
                let ANP = if ANN > AI { 1.0 } else { 0.0 };
                let AOX = if ANP != 0.0 {
                    let AOW = ((1.2919089961638799e9f64 * ANN) / FV).sqrt();
                    AOW
                } else {
                    AI
                };
                let AOY = if AOX < PS { 1.0 } else { 0.0 };
                let AOZ = if AOY != 0.0 {
                    BL
                } else {
                    FB
                };
                let APA = if AOZ == BL { 1.0 } else { 0.0 };
                let APP;
                let APQ;
                if APA != 0.0 {
                    let APB = VN * VM;
                    let APC = (APB + APB) - VS;
                    let APD = if VT >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let APT;
                    let APU;
                    if APD != 0.0 {
                        APT = VT;
                        APU = APC;
                    } else {
                        APT = APS;
                        APU = RZ;
                    }
                    let APV = APT.sqrt();
                    let APW = (VM - APV) / FB;
                    let APX = (VN - (APU * (DS / (ET * APV)))) / FB;
                    let APY = ((((VZ - WB) / GO) * WC) - WH) / WE;
                    let APZ = if APW < VD { 1.0 } else { 0.0 };
                    let AQF;
                    let AQG;
                    if APZ != 0.0 {
                        AQF = APW;
                        AQG = APX;
                    } else {
                        let AQA = APY - APX;
                        let AQB = (WG - APW) - WJ;
                        let AQC = (IZ * WG) * WJ;
                        let AQD = (APY * IZ) * WJ;
                        let AQE = if AQC > AI { 1.0 } else { 0.0 };
                        let AQK;
                        let AQL;
                        if AQE != 0.0 {
                            AQK = AQC;
                            AQL = AQD;
                        } else {
                            let AQI = -AQC;
                            let AQJ = AQD * BP;
                            AQK = AQI;
                            AQL = AQJ;
                        }
                        let AQM = AQA * AQB;
                        let AQN = ((AQB * AQB) + AQK).sqrt();
                        let AQO = WG - (KF * (AQB + AQN));
                        let AQP = APY - ((AQA + (((AQM + AQM) + AQL) * (DS / (ET * AQN)))) * KF);
                        AQF = AQO;
                        AQG = AQP;
                    }
                    let AQH = Lanes([AQG[0], 0.0, AQG[1], AQG[2], AQG[3]]);
                    APP = AQF;
                    APQ = AQH;
                } else {
                    let APE = -((UL - ANN) - (((XV / FB) * PS) / JS));
                    let APF = (Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]]) - ANO) * BP;
                    let APG = (FB * APE) + VK;
                    let APH = (APF * FB) + Lanes([VL, 0.0, 0.0, 0.0, 0.0]);
                    let API = APH * APG;
                    let APJ = APE * APE;
                    let APK = APF * APE;
                    let APL = APK + APK;
                    let APM = (APG * APG) - (IZ * (APJ + VG));
                    let APN = (API + API) - ((APL + Lanes([VH, 0.0, 0.0, 0.0, 0.0])) * IZ);
                    let APO = if APM >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let AQR;
                    let AQS;
                    if APO != 0.0 {
                        AQR = APM;
                        AQS = APN;
                    } else {
                        AQR = AQQ;
                        AQS = KJ;
                    }
                    let AQT = AQR.sqrt();
                    let AQU = (APG - AQT) / FB;
                    let AQV = (APH - (AQS * (DS / (ET * AQT)))) / FB;
                    let AQW = APJ / VG;
                    let AQX = AQW / GO;
                    let AQY = FB / APE;
                    let AQZ = DA + AQY;
                    let ARA = (AQX.ln()) / AQZ;
                    let ARB = ((((((APL - Lanes([(VH * AQW), 0.0, 0.0, 0.0, 0.0])) / VG) - Lanes([(GQ * AQX), 0.0, 0.0, 0.0, 0.0])) / GO) * (DS / AQX)) - ((Lanes([DB, 0.0, 0.0, 0.0, 0.0]) + (((APF * AQY) * BP) / APE)) * ARA)) / AQZ;
                    let ARC = if AQU < VD { 1.0 } else { 0.0 };
                    let ARI;
                    let ARJ;
                    if ARC != 0.0 {
                        ARI = AQU;
                        ARJ = AQV;
                    } else {
                        let ARD = ARB - AQV;
                        let ARE = (ARA - AQU) - WJ;
                        let ARF = (IZ * ARA) * WJ;
                        let ARG = (ARB * IZ) * WJ;
                        let ARH = if ARF > AI { 1.0 } else { 0.0 };
                        let ARM;
                        let ARN;
                        if ARH != 0.0 {
                            ARM = ARF;
                            ARN = ARG;
                        } else {
                            let ARK = -ARF;
                            let ARL = ARG * BP;
                            ARM = ARK;
                            ARN = ARL;
                        }
                        let ARO = ARD * ARE;
                        let ARP = ((ARE * ARE) + ARM).sqrt();
                        let ARQ = ARA - (KF * (ARE + ARP));
                        let ARR = ARB - ((ARD + (((ARO + ARO) + ARN) * (DS / (ET * ARP)))) * KF);
                        ARI = ARQ;
                        ARJ = ARR;
                    }
                    APP = ARI;
                    APQ = ARJ;
                }
                let APR = if APA != 0.0 && AI != 0.0 { 1.0 } else { 0.0 };
                let ARU;
                let ARV;
                let ARW;
                let ARX;
                let ARY;
                let ARZ;
                if APR != 0.0 {
                    let ARS = Lanes([ABH[0], 0.0, ABH[1], ABH[2], ABH[3]]);
                    let mut ASG = 0.0;
                    let mut ASH = 0.0;
                    let mut ASI = 0.0;
                    let mut ASJ = Lanes([0.0; 5]);
                    let mut ASK = Lanes([0.0; 5]);
                    ASG = AI;
                    ASH = APP;
                    ASI = ABF;
                    ASJ = APQ;
                    ASK = ARS;
                    loop {
                        let ASL = if ASG < WV { 1.0 } else { 0.0 };
                        if ASL == 0.0 {
                            break;
                        }
                        let ASM = DA * ASH;
                        let ASN = Lanes([(DB * ASH), 0.0, 0.0, 0.0, 0.0]) + (ASJ * DA);
                        let ASO = (-ASM).exp();
                        let ASP = (ASN * BP) * ASO;
                        let ASQ = if ASH > WZ { 1.0 } else { 0.0 };
                        let ATF;
                        let ATG;
                        let ATH;
                        let ATI;
                        if ASQ != 0.0 {
                            let AST = ASM.exp();
                            let ASU = -GG;
                            let ASV = AST - BL;
                            let ASW = (ASN * AST) * GO;
                            let ASX = (((ASO + ASM) - BL) + (GO * ASV)).sqrt();
                            let ASY = ASU * ASX;
                            let ASZ = Lanes([((GH * BP) * ASX), 0.0, 0.0, 0.0, 0.0]) + ((((ASP + ASN) + (Lanes([(GQ * ASV), 0.0, 0.0, 0.0, 0.0]) + ASW)) * (DS / (ET * ASX))) * ASU);
                            let ATA = XE / ASY;
                            let ATB = ((-ASO) + BL) + (GO * AST);
                            let ATC = ATA * ATB;
                            let ATD = ((((ASZ * ATA) * BP) / ASY) * ATB) + (((ASP * BP) + (Lanes([(GQ * AST), 0.0, 0.0, 0.0, 0.0]) + ASW)) * ATA);
                            ATF = ASY;
                            ATG = ATC;
                            ATH = ASZ;
                            ATI = ATD;
                        } else {
                            let ATE = if ASH < -1e-8f64 { 1.0 } else { 0.0 };
                            let AUE;
                            let AUF;
                            let AUG;
                            let AUH;
                            if ATE != 0.0 {
                                let ATO = ((ASO + ASM) - BL).sqrt();
                                let ATP = GG * ATO;
                                let ATQ = Lanes([(GH * ATO), 0.0, 0.0, 0.0, 0.0]) + (((ASP + ASN) * (DS / (ET * ATO))) * GG);
                                let ATR = XE / ATP;
                                let ATS = (-ASO) + BL;
                                let ATT = ATR * ATS;
                                let ATU = ((((ATQ * ATR) * BP) / ATP) * ATS) + ((ASP * BP) * ATR);
                                AUE = ATP;
                                AUF = ATT;
                                AUG = ATQ;
                                AUH = ATU;
                            } else {
                                let ATV = XE / DA;
                                let ATW = ATV.sqrt();
                                let ATX = -ATW;
                                let ATY = ATX * DA;
                                let ATZ = ATY * ASH;
                                let AUA = Lanes([((((((((DB * ATV) * BP) / DA) * (DS / (ET * ATW))) * BP) * DA) + (DB * ATX)) * ASH), 0.0, 0.0, 0.0, 0.0]) + (ASJ * ATY);
                                let AUB = (XE * DA).sqrt();
                                let AUC = -AUB;
                                let AUD = Lanes([(((DB * XE) * (DS / (ET * AUB))) * BP), 0.0, 0.0, 0.0, 0.0]);
                                AUE = ATZ;
                                AUF = AUC;
                                AUG = AUA;
                                AUH = AUD;
                            }
                            ATF = AUE;
                            ATG = AUF;
                            ATH = AUG;
                            ATI = AUH;
                        }
                        let ATJ = -1e0f64 + (ATG / YF);
                        let ATK = (((-ASH) + (ATF / YF)) - UL) / ATJ;
                        let ATL = ASH - ATK;
                        let ATM = ASJ - (((((ASJ * BP) + (ATH / YF)) - Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]])) - ((ATI / YF) * ATK)) / ATJ);
                        let ATN = if ((ATL - ASH).abs()) < MN { 1.0 } else { 0.0 };
                        let AUI = if ATN != 0.0 {
                            WV
                        } else {
                            ASG
                        };
                        let AUJ = AUI + BL;
                        ASG = AUJ;
                        ASH = ATL;
                        ASI = ATF;
                        ASJ = ATM;
                        ASK = ATH;
                    }
                    let ASR = UL + ASH;
                    let ASS = Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]]) + ASJ;
                    ARU = ASR;
                    ARV = ASI;
                    ARW = AI;
                    ARX = ASS;
                    ARY = ASK;
                    ARZ = KJ;
                } else {
                    let AUL;
                    let AUM;
                    let AUN;
                    if ART != 0.0 {
                        AUL = AMK;
                        AUM = AUK;
                        AUN = AML;
                    } else {
                        AUL = ANN;
                        AUM = MN;
                        AUN = ANO;
                    }
                    let AUO = Lanes([ABH[0], 0.0, ABH[1], ABH[2], ABH[3]]);
                    let mut AUP = 0.0;
                    let mut AUQ = 0.0;
                    let mut AUR = 0.0;
                    let mut AUS = Lanes([0.0; 5]);
                    let mut AUT = Lanes([0.0; 5]);
                    AUP = AI;
                    AUQ = APP;
                    AUR = ABF;
                    AUS = APQ;
                    AUT = AUO;
                    loop {
                        let AUU = if AUP < WV { 1.0 } else { 0.0 };
                        if AUU == 0.0 {
                            break;
                        }
                        let AUV = DA * AUQ;
                        let AUW = Lanes([(DB * AUQ), 0.0, 0.0, 0.0, 0.0]) + (AUS * DA);
                        let AUX = (-AUV).exp();
                        let AUY = (AUW * BP) * AUX;
                        let AUZ = if AUQ > WZ { 1.0 } else { 0.0 };
                        let AVN;
                        let AVO;
                        let AVP;
                        let AVQ;
                        if AUZ != 0.0 {
                            let AVB = AUV.exp();
                            let AVC = -GG;
                            let AVD = AVB - BL;
                            let AVE = (AUW * AVB) * GO;
                            let AVF = (((AUX + AUV) - BL) + (GO * AVD)).sqrt();
                            let AVG = AVC * AVF;
                            let AVH = Lanes([((GH * BP) * AVF), 0.0, 0.0, 0.0, 0.0]) + ((((AUY + AUW) + (Lanes([(GQ * AVD), 0.0, 0.0, 0.0, 0.0]) + AVE)) * (DS / (ET * AVF))) * AVC);
                            let AVI = XE / AVG;
                            let AVJ = ((-AUX) + BL) + (GO * AVB);
                            let AVK = AVI * AVJ;
                            let AVL = ((((AVH * AVI) * BP) / AVG) * AVJ) + (((AUY * BP) + (Lanes([(GQ * AVB), 0.0, 0.0, 0.0, 0.0]) + AVE)) * AVI);
                            AVN = AVG;
                            AVO = AVK;
                            AVP = AVH;
                            AVQ = AVL;
                        } else {
                            let AVM = if AUQ < -1e-8f64 { 1.0 } else { 0.0 };
                            let AWM;
                            let AWN;
                            let AWO;
                            let AWP;
                            if AVM != 0.0 {
                                let AVW = ((AUX + AUV) - BL).sqrt();
                                let AVX = GG * AVW;
                                let AVY = Lanes([(GH * AVW), 0.0, 0.0, 0.0, 0.0]) + (((AUY + AUW) * (DS / (ET * AVW))) * GG);
                                let AVZ = XE / AVX;
                                let AWA = (-AUX) + BL;
                                let AWB = AVZ * AWA;
                                let AWC = ((((AVY * AVZ) * BP) / AVX) * AWA) + ((AUY * BP) * AVZ);
                                AWM = AVX;
                                AWN = AWB;
                                AWO = AVY;
                                AWP = AWC;
                            } else {
                                let AWD = XE / DA;
                                let AWE = AWD.sqrt();
                                let AWF = -AWE;
                                let AWG = AWF * DA;
                                let AWH = AWG * AUQ;
                                let AWI = Lanes([((((((((DB * AWD) * BP) / DA) * (DS / (ET * AWE))) * BP) * DA) + (DB * AWF)) * AUQ), 0.0, 0.0, 0.0, 0.0]) + (AUS * AWG);
                                let AWJ = (XE * DA).sqrt();
                                let AWK = -AWJ;
                                let AWL = Lanes([(((DB * XE) * (DS / (ET * AWJ))) * BP), 0.0, 0.0, 0.0, 0.0]);
                                AWM = AWH;
                                AWN = AWK;
                                AWO = AWI;
                                AWP = AWL;
                            }
                            AVN = AWM;
                            AVO = AWN;
                            AVP = AWO;
                            AVQ = AWP;
                        }
                        let AVR = (-1e0f64 + (AVO / YF)) + ((AVO * PS) / JS);
                        let AVS = ((((AUL - AUQ) + (AVN / YF)) + (((AVN + (XV / FB)) * PS) / JS)) - UL) / AVR;
                        let AVT = AUQ - AVS;
                        let AVU = AUS - ((((((AUN - AUS) + (AVP / YF)) + ((AVP * PS) / JS)) - Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]])) - (((AVQ / YF) + ((AVQ * PS) / JS)) * AVS)) / AVR);
                        let AVV = if ((AVT - AUQ).abs()) < AUM { 1.0 } else { 0.0 };
                        let AWQ = if AVV != 0.0 {
                            WV
                        } else {
                            AUP
                        };
                        let AWR = AWQ + BL;
                        AUP = AWR;
                        AUQ = AVT;
                        AUR = AVN;
                        AUS = AVU;
                        AUT = AVP;
                    }
                    let AWS;
                    let AWT;
                    if AVA != 0.0 {
                        AWS = AUR;
                        AWT = AUT;
                    } else {
                        AWS = AI;
                        AWT = KJ;
                    }
                    let AWW;
                    let AWX;
                    let AWY;
                    if AWU != 0.0 {
                        AWW = AMK;
                        AWX = AWV;
                        AWY = AML;
                    } else {
                        AWW = ANN;
                        AWX = MN;
                        AWY = ANO;
                    }
                    let mut AWZ = 0.0;
                    let mut AXA = 0.0;
                    let mut AXB = 0.0;
                    let mut AXC = Lanes([0.0; 5]);
                    let mut AXD = Lanes([0.0; 5]);
                    AWZ = AI;
                    AXA = AUQ;
                    AXB = AUR;
                    AXC = AUS;
                    AXD = AUT;
                    loop {
                        let AXE = if AWZ < WV { 1.0 } else { 0.0 };
                        if AXE == 0.0 {
                            break;
                        }
                        let AXF = DA * AXA;
                        let AXG = Lanes([(DB * AXA), 0.0, 0.0, 0.0, 0.0]) + (AXC * DA);
                        let AXH = (-AXF).exp();
                        let AXI = (AXG * BP) * AXH;
                        let AXJ = if AXA > WZ { 1.0 } else { 0.0 };
                        let AXX;
                        let AXY;
                        let AXZ;
                        let AYA;
                        if AXJ != 0.0 {
                            let AXL = AXF.exp();
                            let AXM = -GG;
                            let AXN = AXL - BL;
                            let AXO = (AXG * AXL) * GO;
                            let AXP = (((AXH + AXF) - BL) + (GO * AXN)).sqrt();
                            let AXQ = AXM * AXP;
                            let AXR = Lanes([((GH * BP) * AXP), 0.0, 0.0, 0.0, 0.0]) + ((((AXI + AXG) + (Lanes([(GQ * AXN), 0.0, 0.0, 0.0, 0.0]) + AXO)) * (DS / (ET * AXP))) * AXM);
                            let AXS = XE / AXQ;
                            let AXT = ((-AXH) + BL) + (GO * AXL);
                            let AXU = AXS * AXT;
                            let AXV = ((((AXR * AXS) * BP) / AXQ) * AXT) + (((AXI * BP) + (Lanes([(GQ * AXL), 0.0, 0.0, 0.0, 0.0]) + AXO)) * AXS);
                            AXX = AXQ;
                            AXY = AXU;
                            AXZ = AXR;
                            AYA = AXV;
                        } else {
                            let AXW = if AXA < -1e-8f64 { 1.0 } else { 0.0 };
                            let AYW;
                            let AYX;
                            let AYY;
                            let AYZ;
                            if AXW != 0.0 {
                                let AYG = ((AXH + AXF) - BL).sqrt();
                                let AYH = GG * AYG;
                                let AYI = Lanes([(GH * AYG), 0.0, 0.0, 0.0, 0.0]) + (((AXI + AXG) * (DS / (ET * AYG))) * GG);
                                let AYJ = XE / AYH;
                                let AYK = (-AXH) + BL;
                                let AYL = AYJ * AYK;
                                let AYM = ((((AYI * AYJ) * BP) / AYH) * AYK) + ((AXI * BP) * AYJ);
                                AYW = AYH;
                                AYX = AYL;
                                AYY = AYI;
                                AYZ = AYM;
                            } else {
                                let AYN = XE / DA;
                                let AYO = AYN.sqrt();
                                let AYP = -AYO;
                                let AYQ = AYP * DA;
                                let AYR = AYQ * AXA;
                                let AYS = Lanes([((((((((DB * AYN) * BP) / DA) * (DS / (ET * AYO))) * BP) * DA) + (DB * AYP)) * AXA), 0.0, 0.0, 0.0, 0.0]) + (AXC * AYQ);
                                let AYT = (XE * DA).sqrt();
                                let AYU = -AYT;
                                let AYV = Lanes([(((DB * XE) * (DS / (ET * AYT))) * BP), 0.0, 0.0, 0.0, 0.0]);
                                AYW = AYR;
                                AYX = AYU;
                                AYY = AYS;
                                AYZ = AYV;
                            }
                            AXX = AYW;
                            AXY = AYX;
                            AXZ = AYY;
                            AYA = AYZ;
                        }
                        let AYB = (-1e0f64 + (AXY / YF)) + ((AXY * PS) / JS);
                        let AYC = ((((AWW - AXA) + (AXX / YF)) + (((AXX + (XV / FB)) * PS) / JS)) - UL) / AYB;
                        let AYD = AXA - AYC;
                        let AYE = AXC - ((((((AWY - AXC) + (AXZ / YF)) + ((AXZ * PS) / JS)) - Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]])) - (((AYA / YF) + ((AYA * PS) / JS)) * AYC)) / AYB);
                        let AYF = if ((AYD - AXA).abs()) < AWX { 1.0 } else { 0.0 };
                        let AZA = if AYF != 0.0 {
                            WV
                        } else {
                            AWZ
                        };
                        let AZB = AZA + BL;
                        AWZ = AZB;
                        AXA = AYD;
                        AXB = AXX;
                        AXC = AYE;
                        AXD = AXZ;
                    }
                    let AZC;
                    let AZD;
                    if AXK != 0.0 {
                        AZC = AXB;
                        AZD = AXD;
                    } else {
                        AZC = AWS;
                        AZD = AWT;
                    }
                    ARU = AXA;
                    ARV = AXB;
                    ARW = AZC;
                    ARX = AXC;
                    ARY = AXD;
                    ARZ = AZD;
                }
                let ASA = Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]]) + ARX;
                let ASB = (UL + ARU) - EG;
                let ASC = ASB - (ARV / YF);
                let ASD = ASA - (ARY / YF);
                let ASE = ANN - ANK;
                let ASF = if (if ASC > ASE { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                let AZK;
                let AZL;
                if ASF != 0.0 {
                    let AZE = ASD - ANO;
                    let AZF = (ASC - ANN) + ANK;
                    let AZG = AZE * AZF;
                    let AZH = AZG + AZG;
                    let AZI = (AZF * AZF) + 2.25e-2f64;
                    let AZP;
                    let AZQ;
                    if AZJ != 0.0 {
                        let AZX;
                        if AZM != 0.0 {
                            AZX = BL;
                        } else {
                            let AZZ;
                            if AZW != 0.0 {
                                AZZ = FB;
                            } else {
                                let BAB;
                                if AZY != 0.0 {
                                    BAB = IW;
                                } else {
                                    let BAC = if BAA != 0.0 {
                                        IZ
                                    } else {
                                        AI
                                    };
                                    BAB = BAC;
                                }
                                AZZ = BAB;
                            }
                            AZX = AZZ;
                        }
                        let mut BAD = 0.0;
                        let mut BAE = 0.0;
                        let mut BAF = Lanes([0.0; 5]);
                        BAD = AI;
                        BAE = AZI;
                        BAF = AZH;
                        loop {
                            let BAG = if BAD < AZX { 1.0 } else { 0.0 };
                            if BAG == 0.0 {
                                break;
                            }
                            let BAH = BAE.sqrt();
                            let BAI = BAF * (DS / (ET * BAH));
                            let BAJ = BAD + BL;
                            BAD = BAJ;
                            BAE = BAH;
                            BAF = BAI;
                        }
                        AZP = BAE;
                        AZQ = BAF;
                    } else {
                        let AZN = AZI.sqrt();
                        let AZO = AZH * (5e-1f64 * (AZI.powf(-5e-1f64)));
                        AZP = AZN;
                        AZQ = AZO;
                    }
                    let AZR = AZP + IH;
                    let AZS = BL / AZR;
                    let AZT = AZF * ANK;
                    let AZU = ASE + (AZT * AZS);
                    let AZV = ANO + (((AZE * ANK) * AZS) + ((((AZQ * AZS) * BP) / AZR) * AZT));
                    AZK = AZU;
                    AZL = AZV;
                } else {
                    AZK = ASC;
                    AZL = ASD;
                }
                ZP = ANN;
                ZQ = AZK;
                ZR = ASB;
                ZS = ARW;
                ZT = AMK;
                ZU = ABG;
                ZV = AI;
                ZW = ARV;
                ZX = ANO;
                ZY = AZL;
                ZZ = ASA;
                AAA = ARZ;
                AAB = AML;
                AAC = ARY;
            }
            let AAE = if (if parameters[15] == BL { 1.0 } else { 0.0 }) != 0.0 && (if CC > (ZF + AAD) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BCC;
            let BCD;
            let BCE;
            let BCF;
            let BCG;
            let BCH;
            let BCI;
            let BCJ;
            let BCK;
            let BCL;
            let BCM;
            let BCN;
            let BCO;
            let BCP;
            let BCQ;
            let BCR;
            if AAE != 0.0 {
                let BAK = ((JO - staged[87]) + SI) - SN;
                let BAL = (Lanes([0.0, JQ[0], JQ[1], JQ[2], JQ[3]]) + SH) - SQ;
                let BAM = ((3.2043836e-19f64 * FV) * JS) / DA;
                let BAN = BAM.sqrt();
                let BAO = (((DB * BAM) * BP) / DA) * (DS / (ET * BAN));
                let BAP = (FN / FV) / FV;
                let BAQ = (FP / FV) / FV;
                let BAR = BAO * BAN;
                let BAS = (BAN * BAN) / MI;
                let BAT = ML * BAS;
                let BAU = BAS / MI;
                let BAV = ML * BAU;
                let BAW = (((Lanes([(BAR + BAR), 0.0, 0.0, 0.0, 0.0]) - Lanes([0.0, BAT[0], BAT[1], BAT[2], BAT[3]])) / MI) - Lanes([0.0, BAV[0], BAV[1], BAV[2], BAV[3]])) / MI;
                let BAX = (BAU * DA) / FB;
                let BAY = ((BAW * DA) + Lanes([(DB * BAU), 0.0, 0.0, 0.0, 0.0])) / FB;
                let BAZ = (BAX * DA) * FB;
                let BBA = (IZ * ((DA * BAK) - BL)) / BAZ;
                let BBB = (BL + BBA).sqrt();
                let BBC = BL - BBB;
                let BBD = BL / BAP;
                let BBE = BBD / BAU;
                let BBF = BAK * BAK;
                let BBG = BAL * BAK;
                let BBH = BBE * BBF;
                let BBI = FB / BAK;
                let BBJ = DA + BBI;
                let BBK = (BBH.ln()) / BBJ;
                let BBL = ((((((Lanes([(((BAQ * BBD) * BP) / BAP), 0.0, 0.0, 0.0, 0.0]) - (BAW * BBE)) / BAU) * BBF) + ((BBG + BBG) * BBE)) * (DS / BBH)) - ((Lanes([DB, 0.0, 0.0, 0.0, 0.0]) + (((BAL * BBI) * BP) / BAK)) * BBK)) / BBJ;
                let BBM = BBL - (BAL + ((BAY * BBC) + (((((((Lanes([(DB * BAK), 0.0, 0.0, 0.0, 0.0]) + (BAL * DA)) * IZ) - ((((BAY * DA) + Lanes([(DB * BAX), 0.0, 0.0, 0.0, 0.0])) * FB) * BBA)) / BAZ) * (DS / (ET * BBB))) * BP) * BAX)));
                let BBO = (BBK - (BAK + (BAX * BBC))) - BBN;
                let BBP = BBM * BBO;
                let BBQ = IZ * BBN;
                let BBR = ((BBO * BBO) + (BBQ * BBK)).sqrt();
                let BBS = BBK - (KF * (BBO + BBR));
                let BBT = BBL - ((BBM + (((BBP + BBP) + (BBL * BBQ)) * (DS / (ET * BBR)))) * KF);
                let BBU = DA * BBS;
                let BBV = Lanes([(DB * BBS), 0.0, 0.0, 0.0, 0.0]) + (BBT * DA);
                let BBW = BBU.exp();
                let BBX = BBU - BL;
                let BBY = BBX + (BAP * BBW);
                let BBZ = BBV + (Lanes([(BAQ * BBW), 0.0, 0.0, 0.0, 0.0]) + ((BBV * BBW) * BAP));
                let BCA = if (if BBY > AI { 1.0 } else { 0.0 }) != 0.0 && (if BBX > AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BDR;
                let BDS;
                let BDT;
                let BDU;
                let BDV;
                let BDW;
                let BDX;
                let BDY;
                let BDZ;
                let BEA;
                if BCA != 0.0 {
                    let BCV = BBY.sqrt();
                    let BCW = BBX.sqrt();
                    let BCX = BCV - BCW;
                    let BCY = BAN * BCX;
                    let BCZ = (FB * staged[88]) / DA;
                    let BDA = -DA;
                    let BDB = DB * BP;
                    let BDC = JN * BDA;
                    let BDD = (BDA * JM).exp();
                    let BDE = -(BDD - BL);
                    let BDG = BCZ * BDF;
                    let BDH = BDG * BCY;
                    let BDI = (((Lanes([(BDB * JM), 0.0, 0.0, 0.0]) + Lanes([0.0, BDC[0], BDC[1], BDC[2]])) * BDD) * BP) * BDH;
                    let BDK = (BDH * BDE) / BDJ;
                    let BDL = (((Lanes([(((((DB * BCZ) * BP) / DA) * BDF) * BCY), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([(BAO * BCX), 0.0, 0.0, 0.0, 0.0]) + (((BBZ * (DS / (ET * BCV))) - (BBV * (DS / (ET * BCW)))) * BAN)) * BDG)) * BDE) + Lanes([BDI[0], 0.0, BDI[1], BDI[2], BDI[3]])) / BDJ;
                    let BDM = SY * DC;
                    let BDN = (IZ * ((DA * ZG) - BL)) / BDM;
                    let BDO = (((Lanes([(DB * ZG), 0.0, 0.0, 0.0, 0.0]) + (ZJ * DA)) * IZ) - (((TA * DC) + Lanes([(DE * SY), 0.0, 0.0, 0.0, 0.0])) * BDN)) / BDM;
                    let BDP = BL + BDN;
                    let BDQ = if BDP < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let BEC;
                    let BED;
                    if BDQ != 0.0 {
                        BEC = BEB;
                        BED = KJ;
                    } else {
                        BEC = BDP;
                        BED = BDO;
                    }
                    let BEE = (SY * DA) * KF;
                    let BEF = BEC.sqrt();
                    let BEG = BL - BEF;
                    let BEH = ZG + (BEE * BEG);
                    let BEI = ZJ + (((((TA * DA) + Lanes([(DB * SY), 0.0, 0.0, 0.0, 0.0])) * KF) * BEG) + (((BED * (DS / (ET * BEF))) * BP) * BEE));
                    let BEJ = BEH - BBS;
                    let BEK = BEI - BBT;
                    let BEL = if BEJ < AI { 1.0 } else { 0.0 };
                    let BEM;
                    let BEN;
                    if BEL != 0.0 {
                        BEM = AI;
                        BEN = KJ;
                    } else {
                        BEM = BEJ;
                        BEN = BEK;
                    }
                    let BEP = BEO * BEM;
                    let BEQ = BEN * BEO;
                    let BER = BEQ - Lanes([0.0, 0.0, JN[0], JN[1], JN[2]]);
                    let BET = (BEP - JM) - BES;
                    let BEU = BER * BET;
                    let BEV = ((BET * BET) + ((IZ * BEP) * BES)).sqrt();
                    let BEW = BEP - (KF * (BET + BEV));
                    let BEX = BEQ - ((BER + (((BEU + BEU) + ((BEQ * IZ) * BES)) * (DS / (ET * BEV)))) * KF);
                    let BEY = if BEW > BEM { 1.0 } else { 0.0 };
                    let BEZ;
                    let BFA;
                    if BEY != 0.0 {
                        BEZ = BEM;
                        BFA = BEN;
                    } else {
                        BEZ = BEW;
                        BFA = BEX;
                    }
                    let BFC = NJ * BFB;
                    let BFE = BFD * BFB;
                    let BFF = BDJ * BFB;
                    let BFG = if parameters[26] == AI { 1.0 } else { 0.0 };
                    let BFO;
                    let BFP;
                    if BFG != 0.0 {
                        BFO = AI;
                        BFP = KJ;
                    } else {
                        let BFH = ((parameters[141] * CZ) * BFE) * BFF;
                        let BFI = BFH / ES;
                        let BFJ = ((EU * BFI) * BP) / ES;
                        let BFL = JL * BFK;
                        let BFM = (-(((((BFK * JK) + RQ) + PE) + CV) + parameters[143])) / BFC;
                        let BFN = ((((Lanes([0.0, 0.0, BFL[0], BFL[1], BFL[2]]) + RS) + PF) + Lanes([CW, 0.0, 0.0, 0.0, 0.0])) * BP) / BFC;
                        let mut BFS = 0.0;
                        let mut BFT = 0.0;
                        let mut BFU = Lanes([0.0; 5]);
                        BFS = AI;
                        BFT = AI;
                        BFU = KJ;
                        loop {
                            let BFV = if BFS <= 9.9e1f64 { 1.0 } else { 0.0 };
                            if BFV == 0.0 {
                                break;
                            }
                            let BFW = BFS / BFB;
                            let BFX = (ZI + JI) - ((BEZ * BFW) + BBS);
                            let BFY = (ZL + Lanes([0.0, 0.0, JJ[0], JJ[1], JJ[2]])) - ((BFA * BFW) + BBT);
                            let BGA = BL - (BFX / BFZ);
                            let BGB = (BFY / BFZ) * BP;
                            let BGC = BFM + (BFX / BFC);
                            let BGD = BFN + (BFY / BFC);
                            let BGE = BGC * BGC;
                            let BGF = BGD * BGC;
                            let BGG = BGF + BGF;
                            let BGH = BGB * BGA;
                            let BGI = ((BGA * BGA) + 4e-6f64).sqrt();
                            let BGJ = (BGB + ((BGH + BGH) * (DS / (ET * BGI)))) * KF;
                            let BGK = (KF * (BGA + BGI)) + 1e-13f64;
                            let BGL = if BGK < AI { 1.0 } else { 0.0 };
                            let BGM;
                            let BGN;
                            if BGL != 0.0 {
                                BGM = AI;
                                BGN = KJ;
                            } else {
                                BGM = BGK;
                                BGN = BGJ;
                            }
                            let BGO = BGM.sqrt();
                            let BGQ = BGP * (BL - (BGO * BGM));
                            let BGR = ((((BGN * (DS / (ET * BGO))) * BGM) + (BGN * BGO)) * BP) * BGP;
                            let BGS = (-BGQ) / BGC;
                            let BGT = ((BGR * BP) - (BGD * BGS)) / BGC;
                            let BGU = if BGS < -3.4e1f64 { 1.0 } else { 0.0 };
                            let BGX;
                            let BGY;
                            if BGU != 0.0 {
                                BGX = AI;
                                BGY = KJ;
                            } else {
                                let BGV = BGS.exp();
                                let BGW = BGT * BGV;
                                BGX = BGV;
                                BGY = BGW;
                            }
                            let BGZ = PH * BFI;
                            let BHA = BGZ * BGQ;
                            let BHC = (BHA * BGQ) * BHB;
                            let BHD = (((Lanes([((BFJ * PH) * BGQ), 0.0, 0.0, 0.0, 0.0]) + (BGR * BGZ)) * BGQ) + (BGR * BHA)) * BHB;
                            let BHE = if ((FB * BGC) + BGQ) < AI { 1.0 } else { 0.0 };
                            let BHJ;
                            let BHK;
                            if BHE != 0.0 {
                                BHJ = BHC;
                                BHK = BHD;
                            } else {
                                let BHF = BFH * BGE;
                                let BHG = BHF * BGX;
                                let BHH = ((BGG * BFH) * BGX) + (BGY * BHF);
                                let BHI = if (if BHG < BHC { 1.0 } else { 0.0 }) != 0.0 || (if BGC < AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let BHO;
                                let BHP;
                                if BHI != 0.0 {
                                    BHO = BHC;
                                    BHP = BHD;
                                } else {
                                    BHO = BHG;
                                    BHP = BHH;
                                }
                                BHJ = BHO;
                                BHK = BHP;
                            }
                            let BHL = BFT + BHJ;
                            let BHM = BFU + BHK;
                            let BHN = if BHJ < AR { 1.0 } else { 0.0 };
                            let BHQ = if BHN != 0.0 {
                                BFB
                            } else {
                                BFS
                            };
                            let BHR = BHQ + BL;
                            BFS = BHR;
                            BFT = BHL;
                            BFU = BHM;
                        }
                        BFO = BFT;
                        BFP = BFU;
                    }
                    let BFR = if (if BFQ <= AI { 1.0 } else { 0.0 }) != 0.0 || (if EQ <= AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BIJ;
                    let BIK;
                    if BFR != 0.0 {
                        BIJ = AI;
                        BIK = KJ;
                    } else {
                        let BHS = MI * MI;
                        let BHT = ML * MI;
                        let BHU = BHT + BHT;
                        let BHW = FB / BHV;
                        let BHX = BHW * BHS;
                        let BHZ = JL * BHY;
                        let BIA = (BAK - DF) - (BHY * JK);
                        let BIB = (BHU * BHW) * BIA;
                        let BIC = Lanes([0.0, BIB[0], BIB[1], BIB[2], BIB[3]]) + (((BAL - Lanes([DG, 0.0, 0.0, 0.0, 0.0])) - Lanes([0.0, 0.0, BHZ[0], BHZ[1], BHZ[2]])) * BHX);
                        let BID = BL + (BHX * BIA);
                        let BIE = BIC * BID;
                        let BIF = ((BID * BID) + 4e-6f64).sqrt();
                        let BIG = (BIC + ((BIE + BIE) * (DS / (ET * BIF)))) * KF;
                        let BIH = (KF * (BID + BIF)) + 1e-13f64;
                        let BII = if BIH < AI { 1.0 } else { 0.0 };
                        let BIM;
                        let BIN;
                        if BII != 0.0 {
                            BIM = AI;
                            BIN = KJ;
                        } else {
                            BIM = BIH;
                            BIN = BIG;
                        }
                        let BIP = BHV / BHS;
                        let BIQ = (BIM + IH).sqrt();
                        let BIR = BL - BIQ;
                        let BIS = (((BHU * BIP) * BP) / BHS) * BIR;
                        let BIU = JN * BIT;
                        let BIX = BIV * BIW;
                        let BIY = ((BIT * JM) + BBS) - (BIX * ((BAK * BIO) + (BIP * BIR)));
                        let BIZ = (Lanes([0.0, 0.0, BIU[0], BIU[1], BIU[2]]) + BBT) - (((BAL * BIO) + (Lanes([0.0, BIS[0], BIS[1], BIS[2], BIS[3]]) + (((BIN * (DS / (ET * BIQ))) * BP) * BIP))) * BIX);
                        let BJA = BIZ * BIY;
                        let BJB = ((BIY * BIY) + 4e-4f64).sqrt();
                        let BJC = (BIZ + ((BJA + BJA) * (DS / (ET * BJB)))) * KF;
                        let BJD = (KF * (BIY + BJB)) + 1e-12f64;
                        let BJE = if BJD < AI { 1.0 } else { 0.0 };
                        let BJF;
                        let BJG;
                        if BJE != 0.0 {
                            BJF = AI;
                            BJG = KJ;
                        } else {
                            BJF = BJD;
                            BJG = BJC;
                        }
                        let BJH = BJF + IH;
                        let BJJ = (-BJI) / BJH;
                        let BJK = BJJ.exp();
                        let BJL = BFQ * BJH;
                        let BJM = BJL * BDK;
                        let BJN = BJM * BJK;
                        let BJO = ((((BJG * BFQ) * BDK) + (BDL * BJL)) * BJK) + (((((BJG * BJJ) * BP) / BJH) * BJK) * BJM);
                        BIJ = BJN;
                        BIK = BJO;
                    }
                    let BIL = if parameters[16] == BL { 1.0 } else { 0.0 };
                    let BKN;
                    let BKO;
                    let BKP;
                    let BKQ;
                    if BIL != 0.0 {
                        let BJP = (CZ * PS) * BFD;
                        let BJR = (BDA * BJQ).exp();
                        let BJS = 4.1046315303568966e26f64 + (2.4665765749313358e0f64 * FV);
                        let BJT = (BJP * BJR) * BJS;
                        let BJU = 2.1633307652783932e-2f64 / BJT;
                        let BJV = BIJ + BFO;
                        let BJX = BJW * DF;
                        let BJY = BL + (BJV * BJU);
                        let BJZ = BJY.ln();
                        let BKA = 3.3163543761348e-29f64 * FV;
                        let BKB = (BKA * DF).sqrt();
                        let BKC = BBS - (BJX * BJZ);
                        let BKD = BBT - (Lanes([((DG * BJW) * BJZ), 0.0, 0.0, 0.0, 0.0]) + (((((BIK + BFP) * BJU) + Lanes([((((((((BDB * BJQ) * BJR) * BJP) * BJS) * BJU) * BP) / BJT) * BJV), 0.0, 0.0, 0.0, 0.0])) * (DS / BJY)) * BJX));
                        let BKE = (BDA * BKC).exp();
                        let BKF = ((BKE - BL) + (DA * BKC)).sqrt();
                        let BKG = (BDA * BBS).exp();
                        let BKH = ((BKG - BL) + BBU).sqrt();
                        let BKI = -BKB;
                        let BKJ = BKF - BKH;
                        let BKK = BKI * BKJ;
                        let BKL = Lanes([((((DG * BKA) * (DS / (ET * BKB))) * BP) * BKJ), 0.0, 0.0, 0.0, 0.0]) + ((((((Lanes([(BDB * BKC), 0.0, 0.0, 0.0, 0.0]) + (BKD * BDA)) * BKE) + (Lanes([(DB * BKC), 0.0, 0.0, 0.0, 0.0]) + (BKD * DA))) * (DS / (ET * BKF))) - ((((Lanes([(BDB * BBS), 0.0, 0.0, 0.0, 0.0]) + (BBT * BDA)) * BKG) + BBV) * (DS / (ET * BKH)))) * BKI);
                        let BLD;
                        let BLE;
                        let BLF;
                        let BLG;
                        if BKM != 0.0 {
                            let BKR = BIJ + parameters[138];
                            let BKS = parameters[137] / BKR;
                            let BKT = BKS * MI;
                            let BKU = ML * BKS;
                            let BKW = AR * BKV;
                            let BKY = Lanes([0.0, 0.0, 0.0, (BKX * AR), 0.0, 0.0]);
                            let BKZ = (BKW - BKK) / BKT;
                            let BLA = (((((BIK * BKS) * BP) / BKR) * MI) + Lanes([0.0, BKU[0], BKU[1], BKU[2], BKU[3]])) * BKZ;
                            let BLB = ((BKY - Lanes([BKL[0], BKL[1], BKL[2], 0.0, BKL[3], BKL[4]])) - Lanes([BLA[0], BLA[1], BLA[2], 0.0, BLA[3], BLA[4]])) / BKT;
                            BLD = BKW;
                            BLE = BKZ;
                            BLF = BKY;
                            BLG = BLB;
                        } else {
                            let BLC = Lanes([BKL[0], BKL[1], BKL[2], 0.0, BKL[3], BKL[4]]);
                            BLD = BKK;
                            BLE = AI;
                            BLF = BLC;
                            BLG = BCB;
                        }
                        BKN = BLD;
                        BKO = BLE;
                        BKP = BLF;
                        BKQ = BLG;
                    } else {
                        BKN = AI;
                        BKO = AI;
                        BKP = BCB;
                        BKQ = BCB;
                    }
                    BDR = BKN;
                    BDS = BEH;
                    BDT = BIJ;
                    BDU = BBS;
                    BDV = BKO;
                    BDW = BKP;
                    BDX = BEI;
                    BDY = BIK;
                    BDZ = BBT;
                    BEA = BKQ;
                } else {
                    BDR = AI;
                    BDS = ZT;
                    BDT = AI;
                    BDU = AI;
                    BDV = AI;
                    BDW = BCB;
                    BDX = AAB;
                    BDY = KJ;
                    BDZ = KJ;
                    BEA = BCB;
                }
                BCC = BDR;
                BCD = BDS;
                BCE = BAP;
                BCF = BAN;
                BCG = BDT;
                BCH = BAK;
                BCI = BDU;
                BCJ = BDV;
                BCK = BDW;
                BCL = BDX;
                BCM = BAQ;
                BCN = BAO;
                BCO = BDY;
                BCP = BAL;
                BCQ = BDZ;
                BCR = BEA;
            } else {
                BCC = AI;
                BCD = ZT;
                BCE = FR;
                BCF = FI;
                BCG = AI;
                BCH = AI;
                BCI = AI;
                BCJ = AI;
                BCK = BCB;
                BCL = AAB;
                BCM = FS;
                BCN = FJ;
                BCO = KJ;
                BCP = KJ;
                BCQ = KJ;
                BCR = BCB;
            }
            let BCS = ZR - UL;
            let BCT = ZZ - Lanes([UM[0], 0.0, UM[1], UM[2], UM[3]]);
            let BLM;
            let BLN;
            let BLO;
            if BCU != 0.0 {
                let BLH = -XV;
                let BLI = -3.7477e0f64 * XV;
                BLM = XV;
                BLN = BLI;
                BLO = BLH;
            } else {
                let BLJ = EY * XV;
                let BLK = -BLJ;
                let BLL = -4.8303e0f64 * XV;
                BLM = BLJ;
                BLN = BLL;
                BLO = BLK;
            }
            let BLP = Lanes([BCT[0], BCT[1], BCT[2], 0.0, BCT[3], BCT[4]]);
            let BLQ = Lanes([ZX[0], ZX[1], ZX[2], 0.0, ZX[3], ZX[4]]);
            let BLR = Lanes([ZY[0], ZY[1], ZY[2], 0.0, ZY[3], ZY[4]]);
            let BLS = Lanes([AAC[0], AAC[1], AAC[2], 0.0, AAC[3], AAC[4]]);
            let mut BLT = 0.0;
            let mut BLU = 0.0;
            let mut BLV = 0.0;
            let mut BLW = 0.0;
            let mut BLX = 0.0;
            let mut BLY = 0.0;
            let mut BLZ = 0.0;
            let mut BMA = 0.0;
            let mut BMB = 0.0;
            let mut BMC = 0.0;
            let mut BMD = 0.0;
            let mut BME = 0.0;
            let mut BMF = Lanes([0.0; 6]);
            let mut BMG = Lanes([0.0; 6]);
            let mut BMH = Lanes([0.0; 6]);
            let mut BMI = Lanes([0.0; 6]);
            let mut BMJ = Lanes([0.0; 6]);
            let mut BMK = Lanes([0.0; 6]);
            let mut BML = Lanes([0.0; 6]);
            let mut BMM = Lanes([0.0; 6]);
            let mut BMN = Lanes([0.0; 6]);
            BLT = BL;
            BLU = BCS;
            BLV = ZP;
            BLW = ZQ;
            BLX = AI;
            BLY = AI;
            BLZ = AI;
            BMA = AI;
            BMB = ZQ;
            BMC = AI;
            BMD = ZW;
            BME = AI;
            BMF = BLP;
            BMG = BLQ;
            BMH = BLR;
            BMI = BCB;
            BMJ = BCB;
            BMK = BLR;
            BML = BCB;
            BMM = BLS;
            BMN = BCB;
            loop {
                let BMO = if BLT <= WV { 1.0 } else { 0.0 };
                if BMO == 0.0 {
                    break;
                }
                let BMP = DA * BLU;
                let BMQ = Lanes([(DB * BLU), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BMF * DA);
                let BMR = (-BMP).exp();
                let BMS = (BMQ * BP) * BMR;
                let BMT = if BLU < -1e-8f64 { 1.0 } else { 0.0 };
                let BNE;
                let BNF;
                let BNG;
                let BNH;
                if BMT != 0.0 {
                    let BMV = BMP.exp();
                    let BMW = BMV - BL;
                    let BMX = (BMQ * BMV) * GO;
                    let BMY = (((BMR + BMP) - BL) + (GO * BMW)).sqrt();
                    let BMZ = GG * BMY;
                    let BNA = Lanes([(GH * BMY), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((BMS + BMQ) + (Lanes([(GQ * BMW), 0.0, 0.0, 0.0, 0.0, 0.0]) + BMX)) * (DS / (ET * BMY))) * GG);
                    let BNB = (XE * (((-BMR) + BL) + (GO * BMV))) / BMZ;
                    let BNC = ((((BMS * BP) + (Lanes([(GQ * BMV), 0.0, 0.0, 0.0, 0.0, 0.0]) + BMX)) * XE) - (BNA * BNB)) / BMZ;
                    BNE = BMZ;
                    BNF = BNB;
                    BNG = BNA;
                    BNH = BNC;
                } else {
                    let BND = if BLU > 1e-9f64 { 1.0 } else { 0.0 };
                    let BOL;
                    let BOM;
                    let BON;
                    let BOO;
                    if BND != 0.0 {
                        let BNT = BMP.exp();
                        let BNU = BMQ * BNT;
                        let BNV = -GG;
                        let BNW = (BNT - BMP) - BL;
                        let BNX = (((BMR + BMP) - BL) + (GO * BNW)).sqrt();
                        let BNY = BNV * BNX;
                        let BNZ = Lanes([((GH * BP) * BNX), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((BMS + BMQ) + (Lanes([(GQ * BNW), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BNU - BMQ) * GO))) * (DS / (ET * BNX))) * BNV);
                        let BOA = BNT - BL;
                        let BOB = (XE * (((-BMR) + BL) + (GO * BOA))) / BNY;
                        let BOC = ((((BMS * BP) + (Lanes([(GQ * BOA), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BNU * GO))) * XE) - (BNZ * BOB)) / BNY;
                        BOL = BNY;
                        BOM = BOB;
                        BON = BNZ;
                        BOO = BOC;
                    } else {
                        let BOD = -GG;
                        let BOE = GH * BP;
                        let BOG = (BOD * BMP) / BOF;
                        let BOH = (Lanes([(BOE * BMP), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BMQ * BOD)) / BOF;
                        let BOJ = (BOD * DA) / BOI;
                        let BOK = Lanes([(((BOE * DA) + (DB * BOD)) / BOI), 0.0, 0.0, 0.0, 0.0, 0.0]);
                        BOL = BOG;
                        BOM = BOJ;
                        BON = BOH;
                        BOO = BOK;
                    }
                    BNE = BOL;
                    BNF = BOM;
                    BNG = BON;
                    BNH = BOO;
                }
                let BNI = ((BLU - (BNE / YF)) + HI) + ST;
                let BNJ = ((BMF - (BNG / YF)) + Lanes([0.0, 0.0, HK[0], 0.0, HK[1], HK[2]])) + Lanes([SU, 0.0, 0.0, 0.0, 0.0, 0.0]);
                let BNK = BL - (BNF / YF);
                let BNL = (BNH / YF) * BP;
                let BNM = BLV - BLW;
                let BNN = DA * BNM;
                let BNO = Lanes([(DB * BNM), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BMG - BMH) * DA);
                let BNP = -BNN;
                let BNQ = BNO * BP;
                let BNS = if BNP >= BNR { 1.0 } else { 0.0 };
                let BOU;
                let BOV;
                let BOW;
                let BOX;
                if BNS != 0.0 {
                    let BOQ = BOP * ((BL + BNP) - BNR);
                    let BOR = BNQ * BOP;
                    BOU = BOQ;
                    BOV = BOP;
                    BOW = BOR;
                    BOX = BCB;
                } else {
                    let BOS = BNP.exp();
                    let BOT = BNQ * BOS;
                    BOU = BOS;
                    BOV = BOS;
                    BOW = BOT;
                    BOX = BOT;
                }
                let BOY = if BNM < -1e-8f64 { 1.0 } else { 0.0 };
                let BPL;
                let BPM;
                let BPN;
                let BPO;
                let BPP;
                let BPQ;
                let BPR;
                let BPS;
                let BPT;
                let BPU;
                let BPV;
                let BPW;
                if BOY != 0.0 {
                    let BOZ = ((BOU + BNN) - BL).sqrt();
                    let BPA = (BOW + BNO) * (DS / (ET * BOZ));
                    let BPB = GD * BOZ;
                    let BPC = Lanes([(GE * BOZ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPA * GD);
                    let BPD = GD * DA;
                    let BPE = (-BOV) + BL;
                    let BPF = FB * BOZ;
                    let BPG = (BPD * BPE) / BPF;
                    let BPH = ((Lanes([(((GE * DA) + (DB * GD)) * BPE), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BOX * BP) * BPD)) - ((BPA * FB) * BPG)) / BPF;
                    let BPI = -BPG;
                    let BPJ = BPH * BP;
                    BPL = AI;
                    BPM = BPB;
                    BPN = AI;
                    BPO = BPG;
                    BPP = AI;
                    BPQ = BPI;
                    BPR = BCB;
                    BPS = BPC;
                    BPT = BCB;
                    BPU = BPH;
                    BPV = BCB;
                    BPW = BPJ;
                } else {
                    let BPK = if BNM > 1e-8f64 { 1.0 } else { 0.0 };
                    let BSI;
                    let BSJ;
                    let BSK;
                    let BSL;
                    let BSM;
                    let BSN;
                    let BSO;
                    let BSP;
                    let BSQ;
                    let BSR;
                    let BSS;
                    let BST;
                    if BPK != 0.0 {
                        let BQD = ((BOU + BNN) - BL).sqrt();
                        let BQE = (BOW + BNO) * (DS / (ET * BQD));
                        let BQF = -GD;
                        let BQG = GE * BP;
                        let BQH = BQF * BQD;
                        let BQI = Lanes([(BQG * BQD), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BQE * BQF);
                        let BQJ = BQF * DA;
                        let BQK = (-BOV) + BL;
                        let BQL = FB * BQD;
                        let BQM = (BQJ * BQK) / BQL;
                        let BQN = ((Lanes([(((BQG * DA) + (DB * BQF)) * BQK), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BOX * BP) * BQJ)) - ((BQE * FB) * BQM)) / BQL;
                        let BQO = -BQM;
                        let BQP = BQN * BP;
                        let BQQ = BNN.exp();
                        let BQR = BNO * BQQ;
                        let BQS = (DA * BLW).exp();
                        let BQT = (Lanes([(DB * BLW), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BMH * DA)) * BQS;
                        let BQU = BQI * BQH;
                        let BQV = GD * GD;
                        let BQW = GE * GD;
                        let BQX = BQW + BQW;
                        let BQY = (BQH * BQH) / BQV;
                        let BQZ = FB * GJ;
                        let BRA = BQZ * BQS;
                        let BRB = (BQQ - BNN) - BL;
                        let BRC = (BQY + (BRA * BRB)).sqrt();
                        let BRD = ((((BQU + BQU) - Lanes([(BQX * BQY), 0.0, 0.0, 0.0, 0.0, 0.0])) / BQV) + (((Lanes([((GL * FB) * BQS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BQT * BQZ)) * BRB) + ((BQR - BNO) * BRA))) * (DS / (ET * BRC));
                        let BRE = FB * BQH;
                        let BRF = BQI * FB;
                        let BRG = (BRE * BQM) / BQV;
                        let BRH = FB * DA;
                        let BRI = BRH * GJ;
                        let BRJ = BRI * BQS;
                        let BRK = Lanes([((((DB * FB) * GJ) + (GL * BRH)) * BQS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BQT * BRI);
                        let BRL = BQQ - BL;
                        let BRM = FB * BRC;
                        let BRN = BRD * FB;
                        let BRO = (BRG + (BRJ * BRL)) / BRM;
                        let BRP = (BRE * BQO) / BQV;
                        let BRQ = (BRP - (BRJ * BNN)) / BRM;
                        let BRR = (BQF * BRC) - BQH;
                        let BRS = (Lanes([(BQG * BRC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BRD * BQF)) - BQI;
                        let BRT = (BQF * BRO) - BQM;
                        let BRU = (Lanes([(BQG * BRO), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((BRF * BQM) + (BQN * BRE)) - Lanes([(BQX * BRG), 0.0, 0.0, 0.0, 0.0, 0.0])) / BQV) + ((BRK * BRL) + (BQR * BRJ))) - (BRN * BRO)) / BRM) * BQF)) - BQN;
                        let BRV = (BQF * BRQ) - BQO;
                        let BRW = (Lanes([(BQG * BRQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((BRF * BQO) + (BQP * BRE)) - Lanes([(BQX * BRP), 0.0, 0.0, 0.0, 0.0, 0.0])) / BQV) - ((BRK * BNN) + (BNO * BRJ))) - (BRN * BRQ)) / BRM) * BQF)) - BQP;
                        BSI = BRR;
                        BSJ = BQH;
                        BSK = BRT;
                        BSL = BQM;
                        BSM = BRV;
                        BSN = BQO;
                        BSO = BRS;
                        BSP = BQI;
                        BSQ = BRU;
                        BSR = BQN;
                        BSS = BRW;
                        BST = BQP;
                    } else {
                        let BRX = -GD;
                        let BRY = GE * BP;
                        let BSA = (BRX * BNN) / BRZ;
                        let BSB = (Lanes([(BRY * BNN), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BNO * BRX)) / BRZ;
                        let BSD = (BRX * DA) / BSC;
                        let BSE = ((BRY * DA) + (DB * BRX)) / BSC;
                        let BSF = -BSD;
                        let BSG = Lanes([BSE, 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let BSH = Lanes([(BSE * BP), 0.0, 0.0, 0.0, 0.0, 0.0]);
                        BSI = AI;
                        BSJ = BSA;
                        BSK = AI;
                        BSL = BSD;
                        BSM = AI;
                        BSN = BSF;
                        BSO = BCB;
                        BSP = BSB;
                        BSQ = BCB;
                        BSR = BSG;
                        BSS = BCB;
                        BST = BSH;
                    }
                    BPL = BSI;
                    BPM = BSJ;
                    BPN = BSK;
                    BPO = BSL;
                    BPP = BSM;
                    BPQ = BSN;
                    BPR = BSO;
                    BPS = BSP;
                    BPT = BSQ;
                    BPU = BSR;
                    BPV = BSS;
                    BPW = BST;
                }
                let BPX = BNI - BLW;
                let BPY = DA * BPX;
                let BPZ = Lanes([(DB * BPX), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BNJ - BMH) * DA);
                let BQA = -BPY;
                let BQB = BPZ * BP;
                let BQC = if BQA >= BNR { 1.0 } else { 0.0 };
                let BSY;
                let BSZ;
                let BTA;
                let BTB;
                if BQC != 0.0 {
                    let BSU = BOP * ((BL + BQA) - BNR);
                    let BSV = BQB * BOP;
                    BSY = BSU;
                    BSZ = BOP;
                    BTA = BSV;
                    BTB = BCB;
                } else {
                    let BSW = BQA.exp();
                    let BSX = BQB * BSW;
                    BSY = BSW;
                    BSZ = BSW;
                    BTA = BSX;
                    BTB = BSX;
                }
                let BTC = if BPX < -1e-8f64 { 1.0 } else { 0.0 };
                let BTP;
                let BTQ;
                let BTR;
                let BTS;
                let BTT;
                let BTU;
                let BTV;
                let BTW;
                let BTX;
                let BTY;
                let BTZ;
                let BUA;
                if BTC != 0.0 {
                    let BTD = ((BSY + BPY) - BL).sqrt();
                    let BTE = (BTA + BPZ) * (DS / (ET * BTD));
                    let BTF = GD * BTD;
                    let BTG = Lanes([(GE * BTD), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BTE * GD);
                    let BTH = GD * DA;
                    let BTI = (-BSZ) + BL;
                    let BTJ = FB * BTD;
                    let BTK = (BTH * BTI) / BTJ;
                    let BTL = ((Lanes([(((GE * DA) + (DB * GD)) * BTI), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BTB * BP) * BTH)) - ((BTE * FB) * BTK)) / BTJ;
                    let BTM = -BTK;
                    let BTN = BTL * BP;
                    BTP = AI;
                    BTQ = BTF;
                    BTR = AI;
                    BTS = BTM;
                    BTT = AI;
                    BTU = BTK;
                    BTV = BCB;
                    BTW = BTG;
                    BTX = BCB;
                    BTY = BTN;
                    BTZ = BCB;
                    BUA = BTL;
                } else {
                    let BTO = if BPX > 1e-8f64 { 1.0 } else { 0.0 };
                    let BWH;
                    let BWI;
                    let BWJ;
                    let BWK;
                    let BWL;
                    let BWM;
                    let BWN;
                    let BWO;
                    let BWP;
                    let BWQ;
                    let BWR;
                    let BWS;
                    if BTO != 0.0 {
                        let BUC = ((BSY + BPY) - BL).sqrt();
                        let BUD = (BTA + BPZ) * (DS / (ET * BUC));
                        let BUE = -GD;
                        let BUF = GE * BP;
                        let BUG = BUE * BUC;
                        let BUH = Lanes([(BUF * BUC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BUD * BUE);
                        let BUI = BUE * DA;
                        let BUJ = (-BSZ) + BL;
                        let BUK = FB * BUC;
                        let BUL = (BUI * BUJ) / BUK;
                        let BUM = ((Lanes([(((BUF * DA) + (DB * BUE)) * BUJ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BTB * BP) * BUI)) - ((BUD * FB) * BUL)) / BUK;
                        let BUN = -BUL;
                        let BUO = BUM * BP;
                        let BUP = BPY.exp();
                        let BUQ = BPZ * BUP;
                        let BUR = (DA * BLW).exp();
                        let BUS = (Lanes([(DB * BLW), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BMH * DA)) * BUR;
                        let BUT = BUH * BUG;
                        let BUU = GD * GD;
                        let BUV = GE * GD;
                        let BUW = BUV + BUV;
                        let BUX = (BUG * BUG) / BUU;
                        let BUY = FB * GJ;
                        let BUZ = BUY * BUR;
                        let BVA = (BUP - BPY) - BL;
                        let BVB = (BUX + (BUZ * BVA)).sqrt();
                        let BVC = ((((BUT + BUT) - Lanes([(BUW * BUX), 0.0, 0.0, 0.0, 0.0, 0.0])) / BUU) + (((Lanes([((GL * FB) * BUR), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BUS * BUY)) * BVA) + ((BUQ - BPZ) * BUZ))) * (DS / (ET * BVB));
                        let BVD = FB * BUG;
                        let BVE = BUH * FB;
                        let BVF = (BVD * BUL) / BUU;
                        let BVG = FB * DA;
                        let BVH = BVG * GJ;
                        let BVI = BVH * BUR;
                        let BVJ = Lanes([((((DB * FB) * GJ) + (GL * BVG)) * BUR), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BUS * BVH);
                        let BVK = BUP - BL;
                        let BVL = FB * BVB;
                        let BVM = BVC * FB;
                        let BVN = (BVF + (BVI * BVK)) / BVL;
                        let BVO = (BVD * BUN) / BUU;
                        let BVP = (BVO - (BVI * BPY)) / BVL;
                        let BVQ = (BUE * BVB) - BUG;
                        let BVR = (Lanes([(BUF * BVB), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BVC * BUE)) - BUH;
                        let BVS = (BUE * BVN) - BUL;
                        let BVT = (Lanes([(BUF * BVN), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((BVE * BUL) + (BUM * BVD)) - Lanes([(BUW * BVF), 0.0, 0.0, 0.0, 0.0, 0.0])) / BUU) + ((BVJ * BVK) + (BUQ * BVI))) - (BVM * BVN)) / BVL) * BUE)) - BUM;
                        let BVU = (BUE * BVP) - BUN;
                        let BVV = (Lanes([(BUF * BVP), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((BVE * BUN) + (BUO * BVD)) - Lanes([(BUW * BVO), 0.0, 0.0, 0.0, 0.0, 0.0])) / BUU) - ((BVJ * BPY) + (BPZ * BVI))) - (BVM * BVP)) / BVL) * BUE)) - BUO;
                        BWH = BVQ;
                        BWI = BUG;
                        BWJ = BVU;
                        BWK = BUN;
                        BWL = BVS;
                        BWM = BUL;
                        BWN = BVR;
                        BWO = BUH;
                        BWP = BVV;
                        BWQ = BUO;
                        BWR = BVT;
                        BWS = BUM;
                    } else {
                        let BVW = -GD;
                        let BVX = GE * BP;
                        let BVZ = (BVW * BPY) / BVY;
                        let BWA = (Lanes([(BVX * BPY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPZ * BVW)) / BVY;
                        let BWC = (BVW * DA) / BWB;
                        let BWD = ((BVX * DA) + (DB * BVW)) / BWB;
                        let BWE = -BWC;
                        let BWF = Lanes([(BWD * BP), 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let BWG = Lanes([BWD, 0.0, 0.0, 0.0, 0.0, 0.0]);
                        BWH = AI;
                        BWI = BVZ;
                        BWJ = AI;
                        BWK = BWE;
                        BWL = AI;
                        BWM = BWC;
                        BWN = BCB;
                        BWO = BWA;
                        BWP = BCB;
                        BWQ = BWF;
                        BWR = BCB;
                        BWS = BWG;
                    }
                    BTP = BWH;
                    BTQ = BWI;
                    BTR = BWJ;
                    BTS = BWK;
                    BTT = BWL;
                    BTU = BWM;
                    BTV = BWN;
                    BTW = BWO;
                    BTX = BWP;
                    BTY = BWQ;
                    BTZ = BWR;
                    BUA = BWS;
                }
                let BUB = if BLX == BL { 1.0 } else { 0.0 };
                let BXJ;
                let BXK;
                let BXL;
                let BXM;
                let BXN;
                let BXO;
                let BXP;
                let BXQ;
                let BXR;
                if BUB != 0.0 {
                    BXJ = WV;
                    BXK = BLU;
                    BXL = BLV;
                    BXM = BLW;
                    BXN = BLX;
                    BXO = BLT;
                    BXP = BMF;
                    BXQ = BMG;
                    BXR = BMH;
                } else {
                    let BWT = (((((BNE + BPL) + BPM) + BTP) + BTQ) + BCC) / MI;
                    let BWU = ML * BWT;
                    let BWV = (BLV - ZG) - BWT;
                    let BWW = (BMG - Lanes([ZJ[0], ZJ[1], ZJ[2], 0.0, ZJ[3], ZJ[4]])) - (((((((BNG + BPR) + BPS) + BTV) + BTW) + BCK) - Lanes([0.0, BWU[0], BWU[1], 0.0, BWU[2], BWU[3]])) / MI);
                    let BWX = (BPN + BPO) / MI;
                    let BWY = ML * BWX;
                    let BWZ = BL - BWX;
                    let BXA = (((BPT + BPU) - Lanes([0.0, BWY[0], BWY[1], 0.0, BWY[2], BWY[3]])) / MI) * BP;
                    let BXB = (-(((BPP + BPQ) + BTR) + BTS)) / MI;
                    let BXC = ML * BXB;
                    let BXD = (((((BPV + BPW) + BTX) + BTY) * BP) - Lanes([0.0, BXC[0], BXC[1], 0.0, BXC[2], BXC[3]])) / MI;
                    let BXE = BTT + BTU;
                    let BXF = (-(BNF + (BXE * BNK))) / MI;
                    let BXG = ML * BXF;
                    let BXH = (((BNH + (((BTZ + BUA) * BNK) + (BNL * BXE))) * BP) - Lanes([0.0, BXG[0], BXG[1], 0.0, BXG[2], BXG[3]])) / MI;
                    let BXI = if BNE <= BLM { 1.0 } else { 0.0 };
                    if BXI != 0.0 {
                    } else {
                        let BXT = if BNE <= BLN { 1.0 } else { 0.0 };
                    }
                    let BXU = (-ZS) / XV;
                    let BXV = (AAA * BP) / XV;
                    let BXX = (-(BXU * BXW)).exp();
                    let BXY = BL + BXX;
                    let BXZ = BL / BXY;
                    let BYA = (((((((BXV * BXW) * BP) * BXX) * BXZ) * BP) / BXY) * BLO) * BP;
                    let BYC = (BPM + (-(XV + (BXZ * BLO)))) / BYB;
                    let BYD = (BPS + Lanes([BYA[0], BYA[1], BYA[2], 0.0, BYA[3], BYA[4]])) / BYB;
                    let BYE = BPO / BYB;
                    let BYF = BPU / BYB;
                    let BYG = BPQ / BYB;
                    let BYH = BPW / BYB;
                    let BYI = AI / BYB;
                    let BYK = (-(BXU * BYJ)).exp();
                    let BYL = BL + BYK;
                    let BYM = BL / BYL;
                    let BYN = ((((((BXV * BYJ) * BP) * BYK) * BYM) * BP) / BYL) * BLO;
                    let BYO = (BTQ + (BYM * BLO)) / BYB;
                    let BYP = (BTW + Lanes([BYN[0], BYN[1], BYN[2], 0.0, BYN[3], BYN[4]])) / BYB;
                    let BYQ = BTS / BYB;
                    let BYR = BTY / BYB;
                    let BYS = (BTU * BNK) / BYB;
                    let BYT = ((BUA * BNK) + (BNL * BTU)) / BYB;
                    let BYU = BWZ * BYG;
                    let BYV = (BXA * BYG) + (BYH * BWZ);
                    let BYW = BWZ * BYI;
                    let BYX = BXA * BYI;
                    let BYY = BXB * BYE;
                    let BYZ = (BXD * BYE) + (BYF * BXB);
                    let BZA = BXF * BYE;
                    let BZB = (BXH * BYE) + (BYF * BXF);
                    let BZC = (((BYU * BYS) - (BYW * BYQ)) - (BYY * BYS)) + (BZA * BYQ);
                    let BZD = ((((BYV * BYS) + (BYT * BYU)) - ((BYX * BYQ) + (BYR * BYW))) - ((BYZ * BYS) + (BYT * BYY))) + ((BZB * BYQ) + (BYR * BZA));
                    let BZE = if BZC > AI { 1.0 } else { 0.0 };
                    let BZL;
                    let BZM;
                    if BZE != 0.0 {
                        let BZF = BZC + IH;
                        let BZG = BL / BZF;
                        let BZH = ((BZD * BZG) * BP) / BZF;
                        BZL = BZG;
                        BZM = BZH;
                    } else {
                        let BZI = BZC - IH;
                        let BZJ = BL / BZI;
                        let BZK = ((BZD * BZJ) * BP) / BZI;
                        BZL = BZJ;
                        BZM = BZK;
                    }
                    let BZN = (BYG * BYS) - (BYI * BYQ);
                    let BZO = (BXF * BYQ) - (BXB * BYS);
                    let BZP = (BXB * BYI) - (BXF * BYG);
                    let BZQ = -BYE;
                    let BZR = BZQ * BYS;
                    let BZS = BWZ * BYS;
                    let BZT = BZA - BYW;
                    let BZU = BYE * BYQ;
                    let BZV = -BWZ;
                    let BZW = BZV * BYQ;
                    let BZX = BYU - BYY;
                    let BZY = -BZL;
                    let BZZ = BZM * BP;
                    let CAA = ((BZN * BWV) + (BZO * BYC)) + (BZP * BYO);
                    let CAB = BZY * CAA;
                    let CAC = (BZZ * CAA) + ((((((((BYH * BYS) + (BYT * BYG)) - (BYR * BYI)) * BWV) + (BWW * BZN)) + (((((BXH * BYQ) + (BYR * BXF)) - ((BXD * BYS) + (BYT * BXB))) * BYC) + (BYD * BZO))) + ((((BXD * BYI) - ((BXH * BYG) + (BYH * BXF))) * BYO) + (BYP * BZP))) * BZY);
                    let CAD = ((BZR * BWV) + (BZS * BYC)) + (BZT * BYO);
                    let CAE = BZY * CAD;
                    let CAF = (BZZ * CAD) + ((((((((BYF * BP) * BYS) + (BYT * BZQ)) * BWV) + (BWW * BZR)) + ((((BXA * BYS) + (BYT * BWZ)) * BYC) + (BYD * BZS))) + (((BZB - BYX) * BYO) + (BYP * BZT))) * BZY);
                    let CAG = ((BZU * BWV) + (BZW * BYC)) + (BZX * BYO);
                    let CAH = BZY * CAG;
                    let CAI = (BZZ * CAG) + (((((((BYF * BYQ) + (BYR * BYE)) * BWV) + (BWW * BZU)) + (((((BXA * BP) * BYQ) + (BYR * BZV)) * BYC) + (BYD * BZW))) + (((BYV - BYZ) * BYO) + (BYP * BZX))) * BZY);
                    let CAJ = CAB.abs();
                    let CAL = CAC * ((ET * (if CAB >= CAK { 1.0 } else { 0.0 })) - DS);
                    let CAM = CAE.abs();
                    let CAN = CAF * ((ET * (if CAE >= CAK { 1.0 } else { 0.0 })) - DS);
                    let CAO = if CAJ < CAM { 1.0 } else { 0.0 };
                    let CAP;
                    let CAQ;
                    if CAO != 0.0 {
                        CAP = CAM;
                        CAQ = CAN;
                    } else {
                        CAP = CAJ;
                        CAQ = CAL;
                    }
                    let CAR = CAH.abs();
                    let CAS = CAI * ((ET * (if CAH >= CAK { 1.0 } else { 0.0 })) - DS);
                    let CAT = if CAP < CAR { 1.0 } else { 0.0 };
                    let CAU;
                    let CAV;
                    if CAT != 0.0 {
                        CAU = CAR;
                        CAV = CAS;
                    } else {
                        CAU = CAP;
                        CAV = CAQ;
                    }
                    let CAW = if BLT > BNR { 1.0 } else { 0.0 };
                    let CBA;
                    if CAW != 0.0 {
                        CBA = CAX;
                    } else {
                        let CAZ = if BLT > CAY { 1.0 } else { 0.0 };
                        let CBF;
                        if CAZ != 0.0 {
                            CBF = CAX;
                        } else {
                            let CBE = if BLT > CBD { 1.0 } else { 0.0 };
                            let CBI;
                            if CBE != 0.0 {
                                CBI = CAX;
                            } else {
                                let CBH = if BLT > CBG { 1.0 } else { 0.0 };
                                let CBK = if CBH != 0.0 {
                                    CBJ
                                } else {
                                    BL
                                };
                                CBI = CBK;
                            }
                            CBF = CBI;
                        }
                        CBA = CBF;
                    }
                    let CBB = EH / CBA;
                    let CBC = if CAU > CBB { 1.0 } else { 0.0 };
                    let CBT;
                    let CBU;
                    let CBV;
                    let CBW;
                    let CBX;
                    let CBY;
                    if CBC != 0.0 {
                        let CBL = CBB / CAU;
                        let CBM = ((CAV * CBL) * BP) / CAU;
                        let CBN = CAB * CBL;
                        let CBO = (CAC * CBL) + (CBM * CAB);
                        let CBP = CAE * CBL;
                        let CBQ = (CAF * CBL) + (CBM * CAE);
                        let CBR = CAH * CBL;
                        let CBS = (CAI * CBL) + (CBM * CAH);
                        CBT = CBN;
                        CBU = CBP;
                        CBV = CBR;
                        CBW = CBO;
                        CBX = CBQ;
                        CBY = CBS;
                    } else {
                        CBT = CAB;
                        CBU = CAE;
                        CBV = CAH;
                        CBW = CAC;
                        CBX = CAF;
                        CBY = CAI;
                    }
                    let CBZ = BLV + CBT;
                    let CCA = BMG + CBW;
                    let CCB = BLW + CBU;
                    let CCC = BMH + CBX;
                    let CCD = BLU + CBV;
                    let CCE = BMF + CBY;
                    let CCF = if CAU < (HZ * CBA) { 1.0 } else { 0.0 };
                    let CCG = if CCF != 0.0 {
                        BL
                    } else {
                        BLX
                    };
                    BXJ = BLT;
                    BXK = CCD;
                    BXL = CBZ;
                    BXM = CCB;
                    BXN = CCG;
                    BXO = BLY;
                    BXP = CCE;
                    BXQ = CCA;
                    BXR = CCC;
                }
                let BXS = BXJ + BL;
                BLT = BXS;
                BLU = BXK;
                BLV = BXL;
                BLW = BXM;
                BLX = BXN;
                BLY = BXO;
                BLZ = BPL;
                BMA = BTP;
                BMB = BNI;
                BMC = BPM;
                BMD = BNE;
                BME = BTQ;
                BMF = BXP;
                BMG = BXQ;
                BMH = BXR;
                BMI = BPR;
                BMJ = BTV;
                BMK = BNJ;
                BML = BPS;
                BMM = BNG;
                BMN = BTW;
            }
            let BMU = if BLY > AI { 1.0 } else { 0.0 };
            let CCH;
            let CCI;
            if BMU != 0.0 {
                CCH = BLY;
                CCI = AI;
            } else {
                CCH = BLT;
                CCI = BLY;
            }
            let CCJ = if CCH > WV { 1.0 } else { 0.0 };
            let CCK;
            let CCL;
            let CCM;
            let CCN;
            let CCO;
            let CCP;
            let CCQ;
            let CCR;
            if CCJ != 0.0 {
                CCK = ZP;
                CCL = ZQ;
                CCM = ZQ;
                CCN = BCS;
                CCO = BLQ;
                CCP = BLR;
                CCQ = BLR;
                CCR = BLP;
            } else {
                CCK = BLV;
                CCL = BMB;
                CCM = BLW;
                CCN = BLU;
                CCO = BMG;
                CCP = BMK;
                CCQ = BMH;
                CCR = BMF;
            }
            let CCS = -BLZ;
            let CCT = BMI * BP;
            let CCU = if CCS <= IH { 1.0 } else { 0.0 };
            let CCV;
            let CCW;
            let CCX;
            if CCU != 0.0 {
                CCV = IH;
                CCW = BL;
                CCX = BCB;
            } else {
                CCV = CCS;
                CCW = AI;
                CCX = CCT;
            }
            let CCY = -BMA;
            let CCZ = BMJ * BP;
            let CDA = if CCY <= IH { 1.0 } else { 0.0 };
            let CDB;
            let CDC;
            if CDA != 0.0 {
                CDB = IH;
                CDC = BCB;
            } else {
                CDB = CCY;
                CDC = CCZ;
            }
            let CDD = CCV * MG;
            let CDE = MJ * CCV;
            let CDF = (CCX * MG) + Lanes([0.0, CDE[0], CDE[1], 0.0, CDE[2], CDE[3]]);
            let CDG = MI * MI;
            let CDH = ML * MI;
            let CDI = CDH + CDH;
            let CDJ = BHV / CDG;
            let CDK = ((CDI * CDJ) * BP) / CDG;
            let CDL = ZG - DF;
            let CDM = Lanes([DG, 0.0, 0.0, 0.0, 0.0]);
            let CDN = FB / CDJ;
            let CDO = (((CDK * CDN) * BP) / CDJ) * CDL;
            let CDP = Lanes([0.0, CDO[0], CDO[1], CDO[2], CDO[3]]) + ((ZJ - CDM) * CDN);
            let CDQ = BL + (CDN * CDL);
            let CDR = CDP * CDQ;
            let CDS = ((CDQ * CDQ) + 1.0000000000000002e-2f64).sqrt();
            let CDT = (CDP + ((CDR + CDR) * (DS / (ET * CDS)))) * KF;
            let CDU = (KF * (CDQ + CDS)) + 5.0000000000000005e-12f64;
            let CDV = if CDU < AI { 1.0 } else { 0.0 };
            let CDW;
            let CDX;
            if CDV != 0.0 {
                CDW = AI;
                CDX = KJ;
            } else {
                CDW = CDU;
                CDX = CDT;
            }
            let CDY = CDW.sqrt();
            let CDZ = BL - CDY;
            let CEA = CDK * CDZ;
            let CEB = ZG + (CDJ * CDZ);
            let CEC = ZJ + (Lanes([0.0, CEA[0], CEA[1], CEA[2], CEA[3]]) + (((CDX * (DS / (ET * CDY))) * BP) * CDJ));
            let CED = CEC * CEB;
            let CEE = ((CEB * CEB) + 4e-4f64).sqrt();
            let CEF = (CEC + ((CED + CED) * (DS / (ET * CEE)))) * KF;
            let CEG = (KF * (CEB + CEE)) + 1e-12f64;
            let CEH = if CEG < AI { 1.0 } else { 0.0 };
            let CEI;
            let CEJ;
            if CEH != 0.0 {
                CEI = AI;
                CEJ = KJ;
            } else {
                CEI = CEG;
                CEJ = CEF;
            }
            let CEK = CB / CEI;
            let CEL = (KZ - (CEJ * CEK)) / CEI;
            let CEM = CEK + IH;
            let CEO = CEM.powf(CEN);
            let CEP = ((CEL * (CEN * (CEM.powf(staged[156])))) * CEM) + (CEL * CEO);
            let CEQ = BL + (CEO * CEM);
            let CES = CEQ.powf(CER);
            let CET = CES * CEQ;
            let CEU = CB / CET;
            let CEV = (KZ - ((((CEP * (CER * (CEQ.powf(staged[157])))) * CEQ) + (CEP * CES)) * CEU)) / CET;
            let CEW = if CEU < AI { 1.0 } else { 0.0 };
            let CEY;
            let CEZ;
            let CFA;
            let CFB;
            let CFC;
            let CFD;
            let CFE;
            let CFF;
            if CEW != 0.0 {
                CEY = CCK;
                CEZ = CCL;
                CFA = CCN;
                CFB = AI;
                CFC = CCO;
                CFD = CCP;
                CFE = CCR;
                CFF = BCB;
            } else {
                let CEX = if 0.0f64 != 0.0 || (if CDD < HZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CFL;
                let CFM;
                let CFN;
                let CFO;
                let CFP;
                let CFQ;
                let CFR;
                let CFS;
                if CEX != 0.0 {
                    let CFH = Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]]);
                    CFL = AI;
                    CFM = AI;
                    CFN = UL;
                    CFO = AI;
                    CFP = BCB;
                    CFQ = BCB;
                    CFR = CFH;
                    CFS = BCB;
                } else {
                    let CFI = BCD - CCK;
                    let CFJ = Lanes([BCL[0], BCL[1], BCL[2], 0.0, BCL[3], BCL[4]]) - CCO;
                    let CFK = if CFI >= AI { 1.0 } else { 0.0 };
                    let CFT;
                    let CFU;
                    if CFK != 0.0 {
                        CFT = CFI;
                        CFU = CFJ;
                    } else {
                        CFT = AI;
                        CFU = BCB;
                    }
                    let CFW = Lanes([CEV[0], CEV[1], CEV[2], 0.0, CEV[3], CEV[4]]);
                    let CFX = (CFU * CFV) - CFW;
                    let CFY = ((CFV * CFT) - CEU) - BES;
                    let CGA = (IZ * (CFZ * CFT)) * BES;
                    let CGB = ((CFU * CFZ) * IZ) * BES;
                    let CGC = if CGA > AI { 1.0 } else { 0.0 };
                    let CGF;
                    let CGG;
                    if CGC != 0.0 {
                        CGF = CGA;
                        CGG = CGB;
                    } else {
                        let CGD = -CGA;
                        let CGE = CGB * BP;
                        CGF = CGD;
                        CGG = CGE;
                    }
                    let CGH = CFX * CFY;
                    let CGI = ((CFY * CFY) + CGF).sqrt();
                    let CGK = (CGJ * CFT) - (KF * (CFY + CGI));
                    let CGL = (CFU * CGJ) - ((CFX + (((CGH + CGH) + CGG) * (DS / (ET * CGI)))) * KF);
                    let CGM = if CGK <= CFT { 1.0 } else { 0.0 };
                    let CGN;
                    let CGO;
                    if CGM != 0.0 {
                        CGN = CGK;
                        CGO = CGL;
                    } else {
                        CGN = CFT;
                        CGO = CFU;
                    }
                    let CGP = if CGN < AI { 1.0 } else { 0.0 };
                    let CGR;
                    let CGS;
                    if CGP != 0.0 {
                        CGR = AI;
                        CGS = BCB;
                    } else {
                        let CGQ = if CGN > CEU { 1.0 } else { 0.0 };
                        let CGX;
                        let CGY;
                        if CGQ != 0.0 {
                            CGX = CEU;
                            CGY = CFW;
                        } else {
                            CGX = CGN;
                            CGY = CGO;
                        }
                        CGR = CGX;
                        CGS = CGY;
                    }
                    let CGT = CCK + CGR;
                    let CGU = CCO + CGS;
                    let CGW = if CGT < CGV { 1.0 } else { 0.0 };
                    let CGZ;
                    let CHA;
                    if CGW != 0.0 {
                        CGZ = CGV;
                        CHA = BCB;
                    } else {
                        CGZ = CGT;
                        CHA = CGU;
                    }
                    let CHB = if ZU == -1e0f64 { 1.0 } else { 0.0 };
                    let CHC;
                    let CHD;
                    if CHB != 0.0 {
                        CHC = CCK;
                        CHD = CCO;
                    } else {
                        CHC = CGZ;
                        CHD = CHA;
                    }
                    let CHE = if CHC < ZV { 1.0 } else { 0.0 };
                    let CHT;
                    let CHU;
                    if CHE != 0.0 {
                        let CHF = VN * VM;
                        let CHG = (CHF + CHF) - VS;
                        let CHH = if VT >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let CHX;
                        let CHY;
                        if CHH != 0.0 {
                            CHX = VT;
                            CHY = CHG;
                        } else {
                            CHX = CHW;
                            CHY = RZ;
                        }
                        let CHZ = CHX.sqrt();
                        let CIA = (VM - CHZ) / FB;
                        let CIB = (VN - (CHY * (DS / (ET * CHZ)))) / FB;
                        let CIC = ((((VZ - WB) / GO) * WC) - WH) / WE;
                        let CID = if CIA < VD { 1.0 } else { 0.0 };
                        let CIJ;
                        let CIK;
                        if CID != 0.0 {
                            CIJ = CIA;
                            CIK = CIB;
                        } else {
                            let CIE = CIC - CIB;
                            let CIF = (WG - CIA) - WJ;
                            let CIG = (IZ * WG) * WJ;
                            let CIH = (CIC * IZ) * WJ;
                            let CII = if CIG > AI { 1.0 } else { 0.0 };
                            let CIO;
                            let CIP;
                            if CII != 0.0 {
                                CIO = CIG;
                                CIP = CIH;
                            } else {
                                let CIM = -CIG;
                                let CIN = CIH * BP;
                                CIO = CIM;
                                CIP = CIN;
                            }
                            let CIQ = CIE * CIF;
                            let CIR = ((CIF * CIF) + CIO).sqrt();
                            let CIS = WG - (KF * (CIF + CIR));
                            let CIT = CIC - ((CIE + (((CIQ + CIQ) + CIP) * (DS / (ET * CIR)))) * KF);
                            CIJ = CIS;
                            CIK = CIT;
                        }
                        let CIL = Lanes([CIK[0], 0.0, CIK[1], 0.0, CIK[2], CIK[3]]);
                        CHT = CIJ;
                        CHU = CIL;
                    } else {
                        let CHI = -((UL - CHC) - ((XV / FB) * YN));
                        let CHJ = (Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]]) - CHD) * BP;
                        let CHK = (FB * CHI) + VK;
                        let CHL = (CHJ * FB) + Lanes([VL, 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let CHM = CHL * CHK;
                        let CHN = CHI * CHI;
                        let CHO = CHJ * CHI;
                        let CHP = CHO + CHO;
                        let CHQ = (CHK * CHK) - (IZ * (CHN + VG));
                        let CHR = (CHM + CHM) - ((CHP + Lanes([VH, 0.0, 0.0, 0.0, 0.0, 0.0])) * IZ);
                        let CHS = if CHQ >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let CIV;
                        let CIW;
                        if CHS != 0.0 {
                            CIV = CHQ;
                            CIW = CHR;
                        } else {
                            CIV = CIU;
                            CIW = BCB;
                        }
                        let CIX = CIV.sqrt();
                        let CIY = (CHK - CIX) / FB;
                        let CIZ = (CHL - (CIW * (DS / (ET * CIX)))) / FB;
                        let CJA = CHN / VG;
                        let CJB = CJA / GO;
                        let CJC = FB / CHI;
                        let CJD = DA + CJC;
                        let CJE = (CJB.ln()) / CJD;
                        let CJF = ((((((CHP - Lanes([(VH * CJA), 0.0, 0.0, 0.0, 0.0, 0.0])) / VG) - Lanes([(GQ * CJB), 0.0, 0.0, 0.0, 0.0, 0.0])) / GO) * (DS / CJB)) - ((Lanes([DB, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CHJ * CJC) * BP) / CHI)) * CJE)) / CJD;
                        let CJG = if CIY < VD { 1.0 } else { 0.0 };
                        let CJM;
                        let CJN;
                        if CJG != 0.0 {
                            CJM = CIY;
                            CJN = CIZ;
                        } else {
                            let CJH = CJF - CIZ;
                            let CJI = (CJE - CIY) - WJ;
                            let CJJ = (IZ * CJE) * WJ;
                            let CJK = (CJF * IZ) * WJ;
                            let CJL = if CJJ > AI { 1.0 } else { 0.0 };
                            let CJQ;
                            let CJR;
                            if CJL != 0.0 {
                                CJQ = CJJ;
                                CJR = CJK;
                            } else {
                                let CJO = -CJJ;
                                let CJP = CJK * BP;
                                CJQ = CJO;
                                CJR = CJP;
                            }
                            let CJS = CJH * CJI;
                            let CJT = ((CJI * CJI) + CJQ).sqrt();
                            let CJU = CJE - (KF * (CJI + CJT));
                            let CJV = CJF - ((CJH + (((CJS + CJS) + CJR) * (DS / (ET * CJT)))) * KF);
                            CJM = CJU;
                            CJN = CJV;
                        }
                        CHT = CJM;
                        CHU = CJN;
                    }
                    let CHV = if CHE != 0.0 && AI != 0.0 { 1.0 } else { 0.0 };
                    let CJW;
                    let CJX;
                    let CJY;
                    let CJZ;
                    let CKA;
                    let CKB;
                    if CHV != 0.0 {
                        let mut CKC = 0.0;
                        let mut CKD = 0.0;
                        let mut CKE = 0.0;
                        let mut CKF = Lanes([0.0; 6]);
                        let mut CKG = Lanes([0.0; 6]);
                        CKC = AI;
                        CKD = CHT;
                        CKE = AI;
                        CKF = CHU;
                        CKG = BCB;
                        loop {
                            let CKI = if CKC < CKH { 1.0 } else { 0.0 };
                            if CKI == 0.0 {
                                break;
                            }
                            let CKJ = DA * CKD;
                            let CKK = Lanes([(DB * CKD), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CKF * DA);
                            let CKL = (-CKJ).exp();
                            let CKM = (CKK * BP) * CKL;
                            let CKN = if CKD > WZ { 1.0 } else { 0.0 };
                            let CLE;
                            let CLF;
                            let CLG;
                            let CLH;
                            if CKN != 0.0 {
                                let CKS = CKJ.exp();
                                let CKT = -GG;
                                let CKU = CKS - BL;
                                let CKV = (CKK * CKS) * GO;
                                let CKW = (((CKL + CKJ) - BL) + (GO * CKU)).sqrt();
                                let CKX = CKT * CKW;
                                let CKY = Lanes([((GH * BP) * CKW), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CKM + CKK) + (Lanes([(GQ * CKU), 0.0, 0.0, 0.0, 0.0, 0.0]) + CKV)) * (DS / (ET * CKW))) * CKT);
                                let CKZ = XE / CKX;
                                let CLA = ((-CKL) + BL) + (GO * CKS);
                                let CLB = CKZ * CLA;
                                let CLC = ((((CKY * CKZ) * BP) / CKX) * CLA) + (((CKM * BP) + (Lanes([(GQ * CKS), 0.0, 0.0, 0.0, 0.0, 0.0]) + CKV)) * CKZ);
                                CLE = CKX;
                                CLF = CLB;
                                CLG = CKY;
                                CLH = CLC;
                            } else {
                                let CLD = if CKD < -1e-8f64 { 1.0 } else { 0.0 };
                                let CMH;
                                let CMI;
                                let CMJ;
                                let CMK;
                                if CLD != 0.0 {
                                    let CLR = ((CKL + CKJ) - BL).sqrt();
                                    let CLS = GG * CLR;
                                    let CLT = Lanes([(GH * CLR), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CKM + CKK) * (DS / (ET * CLR))) * GG);
                                    let CLU = XE / CLS;
                                    let CLV = (-CKL) + BL;
                                    let CLW = CLU * CLV;
                                    let CLX = ((((CLT * CLU) * BP) / CLS) * CLV) + ((CKM * BP) * CLU);
                                    CMH = CLS;
                                    CMI = CLW;
                                    CMJ = CLT;
                                    CMK = CLX;
                                } else {
                                    let CLY = XE / DA;
                                    let CLZ = CLY.sqrt();
                                    let CMA = -CLZ;
                                    let CMB = CMA * DA;
                                    let CMC = CMB * CKD;
                                    let CMD = Lanes([((((((((DB * CLY) * BP) / DA) * (DS / (ET * CLZ))) * BP) * DA) + (DB * CMA)) * CKD), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CKF * CMB);
                                    let CME = (XE * DA).sqrt();
                                    let CMF = -CME;
                                    let CMG = Lanes([(((DB * XE) * (DS / (ET * CME))) * BP), 0.0, 0.0, 0.0, 0.0, 0.0]);
                                    CMH = CMC;
                                    CMI = CMF;
                                    CMJ = CMD;
                                    CMK = CMG;
                                }
                                CLE = CMH;
                                CLF = CMI;
                                CLG = CMJ;
                                CLH = CMK;
                            }
                            let CLI = CLG * CLE;
                            let CLJ = ((CLE * CLE) + 4e-12f64).sqrt();
                            let CLK = (CLI + CLI) * (DS / (ET * CLJ));
                            let CLL = CLE / CLJ;
                            let CLM = KF * (BL + CLL);
                            let CLN = ((CLG - (CLK * CLL)) / CLJ) * KF;
                            let CLO = (CLG + CLK) * KF;
                            let CLP = (KF * (CLE + CLJ)) + 1e-16f64;
                            let CLQ = if CLP < AI { 1.0 } else { 0.0 };
                            let CML;
                            let CMM;
                            let CMN;
                            let CMO;
                            if CLQ != 0.0 {
                                CML = AI;
                                CMM = AI;
                                CMN = BCB;
                                CMO = BCB;
                            } else {
                                CML = CLP;
                                CMM = CLM;
                                CMN = CLO;
                                CMO = CLN;
                            }
                            let CMP = -XV;
                            let CMQ = CMN * BP;
                            let CMR = (CMP - CML) - AR;
                            let CMS = (IZ * CMP) * AR;
                            let CMT = if CMS > AI { 1.0 } else { 0.0 };
                            let CMV = if CMT != 0.0 {
                                CMS
                            } else {
                                let CMU = -CMS;
                                CMU
                            };
                            let CMW = CMQ * CMR;
                            let CMX = ((CMR * CMR) + CMV).sqrt();
                            let CMY = (CMW + CMW) * (DS / (ET * CMX));
                            let CMZ = CMR / CMX;
                            let CNA = KF * (BL + CMZ);
                            let CNB = CMP - (KF * (CMR + CMX));
                            let CNC = ((CMQ + CMY) * KF) * BP;
                            let CND = CLF * CNA;
                            let CNE = CMM * CND;
                            let CNF = CNC * CNB;
                            let CNG = ((((CNB * CNB) / FB) / JS) / CZ) / FV;
                            let CNH = ((((CNF + CNF) / FB) / JS) / CZ) / FV;
                            let CNI = FB * CNG;
                            let CNJ = (CNI * CNE) / CNB;
                            let CNK = (-1e0f64 + (CLF / YF)) + CNJ;
                            let CNL = ((((-CKD) + (CLE / YF)) - UL) + CNG) / CNK;
                            let CNM = CKD - CNL;
                            let CNN = CKF - ((((((CKF * BP) + (CLG / YF)) - Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]])) + CNH) - (((CLH / YF) + (((((CNH * FB) * CNE) + (((CMO * CND) + (((CLH * CNA) + ((((CMQ - (CMY * CMZ)) / CMX) * KF) * CLF)) * CMM)) * CNI)) - (CNC * CNJ)) / CNB)) * CNL)) / CNK);
                            let CNO = if ((CNM - CKD).abs()) < HZ { 1.0 } else { 0.0 };
                            let CNP = if CNO != 0.0 {
                                CKH
                            } else {
                                CKC
                            };
                            let CNQ = CNP + BL;
                            CKC = CNQ;
                            CKD = CNM;
                            CKE = CLE;
                            CKF = CNN;
                            CKG = CLG;
                        }
                        let CKO = UL + CKD;
                        let CKP = Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]]) + CKF;
                        let CKQ = CKO - (CKE / YF);
                        let CKR = CKP - (CKG / YF);
                        CJW = CKQ;
                        CJX = CKO;
                        CJY = CKE;
                        CJZ = CKR;
                        CKA = CKP;
                        CKB = CKG;
                    } else {
                        let mut CNR = 0.0;
                        let mut CNS = 0.0;
                        let mut CNT = 0.0;
                        let mut CNU = Lanes([0.0; 6]);
                        let mut CNV = Lanes([0.0; 6]);
                        CNR = AI;
                        CNS = CHT;
                        CNT = AI;
                        CNU = CHU;
                        CNV = BCB;
                        loop {
                            let CNW = if CNR < CKH { 1.0 } else { 0.0 };
                            if CNW == 0.0 {
                                break;
                            }
                            let CNX = DA * CNS;
                            let CNY = Lanes([(DB * CNS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CNU * DA);
                            let CNZ = (-CNX).exp();
                            let COA = (CNY * BP) * CNZ;
                            let COB = if CNS > WZ { 1.0 } else { 0.0 };
                            let COS;
                            let COT;
                            let COU;
                            let COV;
                            if COB != 0.0 {
                                let COG = CNX.exp();
                                let COH = -GG;
                                let COI = COG - BL;
                                let COJ = (CNY * COG) * GO;
                                let COK = (((CNZ + CNX) - BL) + (GO * COI)).sqrt();
                                let COL = COH * COK;
                                let COM = Lanes([((GH * BP) * COK), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((COA + CNY) + (Lanes([(GQ * COI), 0.0, 0.0, 0.0, 0.0, 0.0]) + COJ)) * (DS / (ET * COK))) * COH);
                                let CON = XE / COL;
                                let COO = ((-CNZ) + BL) + (GO * COG);
                                let COP = CON * COO;
                                let COQ = ((((COM * CON) * BP) / COL) * COO) + (((COA * BP) + (Lanes([(GQ * COG), 0.0, 0.0, 0.0, 0.0, 0.0]) + COJ)) * CON);
                                COS = COL;
                                COT = COP;
                                COU = COM;
                                COV = COQ;
                            } else {
                                let COR = if CNS < -1e-8f64 { 1.0 } else { 0.0 };
                                let CPV;
                                let CPW;
                                let CPX;
                                let CPY;
                                if COR != 0.0 {
                                    let CPF = ((CNZ + CNX) - BL).sqrt();
                                    let CPG = GG * CPF;
                                    let CPH = Lanes([(GH * CPF), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((COA + CNY) * (DS / (ET * CPF))) * GG);
                                    let CPI = XE / CPG;
                                    let CPJ = (-CNZ) + BL;
                                    let CPK = CPI * CPJ;
                                    let CPL = ((((CPH * CPI) * BP) / CPG) * CPJ) + ((COA * BP) * CPI);
                                    CPV = CPG;
                                    CPW = CPK;
                                    CPX = CPH;
                                    CPY = CPL;
                                } else {
                                    let CPM = XE / DA;
                                    let CPN = CPM.sqrt();
                                    let CPO = -CPN;
                                    let CPP = CPO * DA;
                                    let CPQ = CPP * CNS;
                                    let CPR = Lanes([((((((((DB * CPM) * BP) / DA) * (DS / (ET * CPN))) * BP) * DA) + (DB * CPO)) * CNS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CNU * CPP);
                                    let CPS = (XE * DA).sqrt();
                                    let CPT = -CPS;
                                    let CPU = Lanes([(((DB * XE) * (DS / (ET * CPS))) * BP), 0.0, 0.0, 0.0, 0.0, 0.0]);
                                    CPV = CPQ;
                                    CPW = CPT;
                                    CPX = CPR;
                                    CPY = CPU;
                                }
                                COS = CPV;
                                COT = CPW;
                                COU = CPX;
                                COV = CPY;
                            }
                            let COW = COU * COS;
                            let COX = ((COS * COS) + 4e-12f64).sqrt();
                            let COY = (COW + COW) * (DS / (ET * COX));
                            let COZ = COS / COX;
                            let CPA = KF * (BL + COZ);
                            let CPB = ((COU - (COY * COZ)) / COX) * KF;
                            let CPC = (COU + COY) * KF;
                            let CPD = (KF * (COS + COX)) + 1e-16f64;
                            let CPE = if CPD < AI { 1.0 } else { 0.0 };
                            let CPZ;
                            let CQA;
                            let CQB;
                            let CQC;
                            if CPE != 0.0 {
                                CPZ = AI;
                                CQA = AI;
                                CQB = BCB;
                                CQC = BCB;
                            } else {
                                CPZ = CPD;
                                CQA = CPA;
                                CQB = CPC;
                                CQC = CPB;
                            }
                            let CQD = -XV;
                            let CQE = CQB * BP;
                            let CQF = (CQD - CPZ) - AR;
                            let CQG = (IZ * CQD) * AR;
                            let CQH = if CQG > AI { 1.0 } else { 0.0 };
                            let CQJ = if CQH != 0.0 {
                                CQG
                            } else {
                                let CQI = -CQG;
                                CQI
                            };
                            let CQK = CQE * CQF;
                            let CQL = ((CQF * CQF) + CQJ).sqrt();
                            let CQM = (CQK + CQK) * (DS / (ET * CQL));
                            let CQN = CQF / CQL;
                            let CQO = KF * (BL + CQN);
                            let CQP = CQD - (KF * (CQF + CQL));
                            let CQQ = ((CQE + CQM) * KF) * BP;
                            let CQR = COT * CQO;
                            let CQS = CQA * CQR;
                            let CQT = CQQ * CQP;
                            let CQU = ((((CQP * CQP) / FB) / JS) / CZ) / FV;
                            let CQV = ((((CQT + CQT) / FB) / JS) / CZ) / FV;
                            let CQW = FB * CQU;
                            let CQX = (CQW * CQS) / CQP;
                            let CQY = ((-1e0f64 + (COT / YF)) + (COT * YN)) + CQX;
                            let CQZ = (((((CHC - CNS) + (COS / YF)) + ((COS + (XV / FB)) * YN)) - UL) + CQU) / CQY;
                            let CRA = CNS - CQZ;
                            let CRB = CNU - (((((((CHD - CNU) + (COU / YF)) + (COU * YN)) - Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]])) + CQV) - ((((COV / YF) + (COV * YN)) + (((((CQV * FB) * CQS) + (((CQC * CQR) + (((COV * CQO) + ((((CQE - (CQM * CQN)) / CQL) * KF) * COT)) * CQA)) * CQW)) - (CQQ * CQX)) / CQP)) * CQZ)) / CQY);
                            let CRC = if ((CRA - CNS).abs()) < HZ { 1.0 } else { 0.0 };
                            let CRD = if CRC != 0.0 {
                                CKH
                            } else {
                                CNR
                            };
                            let CRE = CRD + BL;
                            CNR = CRE;
                            CNS = CRA;
                            CNT = COS;
                            CNU = CRB;
                            CNV = COU;
                        }
                        let COC = UL + CNS;
                        let COD = Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]]) + CNU;
                        let COE = COC - (CNT / YF);
                        let COF = COD - (CNV / YF);
                        CJW = COE;
                        CJX = COC;
                        CJY = CNT;
                        CJZ = COF;
                        CKA = COD;
                        CKB = CNV;
                    }
                    CFL = CHC;
                    CFM = CJW;
                    CFN = CJX;
                    CFO = CJY;
                    CFP = CHD;
                    CFQ = CJZ;
                    CFR = CKA;
                    CFS = CKB;
                }
                CEY = CFL;
                CEZ = CFM;
                CFA = CFN;
                CFB = CFO;
                CFC = CFP;
                CFD = CFQ;
                CFE = CFR;
                CFF = CFS;
            }
            let CFG = if CDD < HZ { 1.0 } else { 0.0 };
            let CRI;
            let CRJ;
            let CRK;
            let CRL;
            let CRM;
            let CRN;
            let CRO;
            let CRP;
            if CFG != 0.0 {
                CRI = CCK;
                CRJ = CCL;
                CRK = CCN;
                CRL = CCM;
                CRM = CCO;
                CRN = CCP;
                CRO = CCR;
                CRP = CCQ;
            } else {
                let CRF = CFA - UL;
                let CRG = CFE - Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]]);
                let CRH = if CEZ < CEY { 1.0 } else { 0.0 };
                let CRR;
                let CRS;
                if CRH != 0.0 {
                    CRR = CEZ;
                    CRS = CFD;
                } else {
                    CRR = CEY;
                    CRS = CFC;
                }
                CRI = CEY;
                CRJ = CEZ;
                CRK = CRF;
                CRL = CRR;
                CRM = CFC;
                CRN = CFD;
                CRO = CRG;
                CRP = CRS;
            }
            let CRQ = if ZU < AI { 1.0 } else { 0.0 };
            let CRT = if CRQ != 0.0 {
                BL
            } else {
                AI
            };
            let mut CRU = 0.0;
            let mut CRV = 0.0;
            let mut CRW = 0.0;
            let mut CRX = 0.0;
            let mut CRY = 0.0;
            let mut CRZ = 0.0;
            let mut CSA = 0.0;
            let mut CSB = 0.0;
            let mut CSC = 0.0;
            let mut CSD = 0.0;
            let mut CSE = 0.0;
            let mut CSF = 0.0;
            let mut CSG = Lanes([0.0; 6]);
            let mut CSH = Lanes([0.0; 6]);
            let mut CSI = Lanes([0.0; 6]);
            let mut CSJ = Lanes([0.0; 6]);
            let mut CSK = Lanes([0.0; 6]);
            let mut CSL = Lanes([0.0; 6]);
            let mut CSM = Lanes([0.0; 6]);
            let mut CSN = Lanes([0.0; 6]);
            let mut CSO = Lanes([0.0; 6]);
            CRU = BL;
            CRV = CRK;
            CRW = CRI;
            CRX = CRL;
            CRY = CRT;
            CRZ = CCI;
            CSA = CRJ;
            CSB = AI;
            CSC = AI;
            CSD = AI;
            CSE = CFB;
            CSF = AI;
            CSG = CRO;
            CSH = CRM;
            CSI = CRP;
            CSJ = CRN;
            CSK = BCB;
            CSL = BCB;
            CSM = BCB;
            CSN = CFF;
            CSO = BCB;
            loop {
                let CSP = if CRU <= CKH { 1.0 } else { 0.0 };
                if CSP == 0.0 {
                    break;
                }
                let CSQ = DA * CRV;
                let CSR = Lanes([(DB * CRV), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CSG * DA);
                let CSS = (-CSQ).exp();
                let CST = (CSR * BP) * CSS;
                let CSU = if CRV < -1e-8f64 { 1.0 } else { 0.0 };
                let CTF;
                let CTG;
                let CTH;
                let CTI;
                if CSU != 0.0 {
                    let CSW = CSQ.exp();
                    let CSX = CSW - BL;
                    let CSY = (CSR * CSW) * GO;
                    let CSZ = (((CSS + CSQ) - BL) + (GO * CSX)).sqrt();
                    let CTA = GG * CSZ;
                    let CTB = Lanes([(GH * CSZ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CST + CSR) + (Lanes([(GQ * CSX), 0.0, 0.0, 0.0, 0.0, 0.0]) + CSY)) * (DS / (ET * CSZ))) * GG);
                    let CTC = (XE * (((-CSS) + BL) + (GO * CSW))) / CTA;
                    let CTD = ((((CST * BP) + (Lanes([(GQ * CSW), 0.0, 0.0, 0.0, 0.0, 0.0]) + CSY)) * XE) - (CTB * CTC)) / CTA;
                    CTF = CTA;
                    CTG = CTC;
                    CTH = CTB;
                    CTI = CTD;
                } else {
                    let CTE = if CRV > 1e-9f64 { 1.0 } else { 0.0 };
                    let CUL;
                    let CUM;
                    let CUN;
                    let CUO;
                    if CTE != 0.0 {
                        let CTT = CSQ.exp();
                        let CTU = CSR * CTT;
                        let CTV = -GG;
                        let CTW = (CTT - CSQ) - BL;
                        let CTX = (((CSS + CSQ) - BL) + (GO * CTW)).sqrt();
                        let CTY = CTV * CTX;
                        let CTZ = Lanes([((GH * BP) * CTX), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CST + CSR) + (Lanes([(GQ * CTW), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CTU - CSR) * GO))) * (DS / (ET * CTX))) * CTV);
                        let CUA = CTT - BL;
                        let CUB = (XE * (((-CSS) + BL) + (GO * CUA))) / CTY;
                        let CUC = ((((CST * BP) + (Lanes([(GQ * CUA), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CTU * GO))) * XE) - (CTZ * CUB)) / CTY;
                        CUL = CTY;
                        CUM = CUB;
                        CUN = CTZ;
                        CUO = CUC;
                    } else {
                        let CUD = -GG;
                        let CUE = GH * BP;
                        let CUG = (CUD * CSQ) / CUF;
                        let CUH = (Lanes([(CUE * CSQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CSR * CUD)) / CUF;
                        let CUJ = (CUD * DA) / CUI;
                        let CUK = Lanes([(((CUE * DA) + (DB * CUD)) / CUI), 0.0, 0.0, 0.0, 0.0, 0.0]);
                        CUL = CUG;
                        CUM = CUJ;
                        CUN = CUH;
                        CUO = CUK;
                    }
                    CTF = CUL;
                    CTG = CUM;
                    CTH = CUN;
                    CTI = CUO;
                }
                let CTJ = ((CRV - (CTF / YF)) + HI) + ST;
                let CTK = ((CSG - (CTH / YF)) + Lanes([0.0, 0.0, HK[0], 0.0, HK[1], HK[2]])) + Lanes([SU, 0.0, 0.0, 0.0, 0.0, 0.0]);
                let CTL = BL - (CTG / YF);
                let CTM = (CTI / YF) * BP;
                let CTN = CRW - CRX;
                let CTO = DA * CTN;
                let CTP = Lanes([(DB * CTN), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CSH - CSI) * DA);
                let CTQ = -CTO;
                let CTR = CTP * BP;
                let CTS = if CTQ >= BNR { 1.0 } else { 0.0 };
                let CUT;
                let CUU;
                let CUV;
                let CUW;
                if CTS != 0.0 {
                    let CUP = BOP * ((BL + CTQ) - BNR);
                    let CUQ = CTR * BOP;
                    CUT = CUP;
                    CUU = BOP;
                    CUV = CUQ;
                    CUW = BCB;
                } else {
                    let CUR = CTQ.exp();
                    let CUS = CTR * CUR;
                    CUT = CUR;
                    CUU = CUR;
                    CUV = CUS;
                    CUW = CUS;
                }
                let CUX = if CTN < -1e-8f64 { 1.0 } else { 0.0 };
                let CVK;
                let CVL;
                let CVM;
                let CVN;
                let CVO;
                let CVP;
                let CVQ;
                let CVR;
                let CVS;
                let CVT;
                let CVU;
                let CVV;
                if CUX != 0.0 {
                    let CUY = ((CUT + CTO) - BL).sqrt();
                    let CUZ = (CUV + CTP) * (DS / (ET * CUY));
                    let CVA = GD * CUY;
                    let CVB = Lanes([(GE * CUY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CUZ * GD);
                    let CVC = GD * DA;
                    let CVD = (-CUU) + BL;
                    let CVE = FB * CUY;
                    let CVF = (CVC * CVD) / CVE;
                    let CVG = ((Lanes([(((GE * DA) + (DB * GD)) * CVD), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CUW * BP) * CVC)) - ((CUZ * FB) * CVF)) / CVE;
                    let CVH = -CVF;
                    let CVI = CVG * BP;
                    CVK = AI;
                    CVL = CVA;
                    CVM = AI;
                    CVN = CVF;
                    CVO = AI;
                    CVP = CVH;
                    CVQ = BCB;
                    CVR = CVB;
                    CVS = BCB;
                    CVT = CVG;
                    CVU = BCB;
                    CVV = CVI;
                } else {
                    let CVJ = if CTN > 1e-8f64 { 1.0 } else { 0.0 };
                    let CYI;
                    let CYJ;
                    let CYK;
                    let CYL;
                    let CYM;
                    let CYN;
                    let CYO;
                    let CYP;
                    let CYQ;
                    let CYR;
                    let CYS;
                    let CYT;
                    if CVJ != 0.0 {
                        let CWC = ((CUT + CTO) - BL).sqrt();
                        let CWD = (CUV + CTP) * (DS / (ET * CWC));
                        let CWE = -GD;
                        let CWF = GE * BP;
                        let CWG = CWE * CWC;
                        let CWH = Lanes([(CWF * CWC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CWD * CWE);
                        let CWI = CWE * DA;
                        let CWJ = (-CUU) + BL;
                        let CWK = FB * CWC;
                        let CWL = (CWI * CWJ) / CWK;
                        let CWM = ((Lanes([(((CWF * DA) + (DB * CWE)) * CWJ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CUW * BP) * CWI)) - ((CWD * FB) * CWL)) / CWK;
                        let CWN = -CWL;
                        let CWO = CWM * BP;
                        let CWP = CTO.exp();
                        let CWQ = CTP * CWP;
                        let CWR = CRX - CEU;
                        let CWS = (DA * CWR).exp();
                        let CWT = (Lanes([(DB * CWR), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CSI - Lanes([CEV[0], CEV[1], CEV[2], 0.0, CEV[3], CEV[4]])) * DA)) * CWS;
                        let CWU = CWH * CWG;
                        let CWV = GD * GD;
                        let CWW = GE * GD;
                        let CWX = CWW + CWW;
                        let CWY = (CWG * CWG) / CWV;
                        let CWZ = FB * GJ;
                        let CXA = CWZ * CWS;
                        let CXB = (CWP - CTO) - BL;
                        let CXC = (CWY + (CXA * CXB)).sqrt();
                        let CXD = ((((CWU + CWU) - Lanes([(CWX * CWY), 0.0, 0.0, 0.0, 0.0, 0.0])) / CWV) + (((Lanes([((GL * FB) * CWS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CWT * CWZ)) * CXB) + ((CWQ - CTP) * CXA))) * (DS / (ET * CXC));
                        let CXE = FB * CWG;
                        let CXF = CWH * FB;
                        let CXG = (CXE * CWL) / CWV;
                        let CXH = FB * DA;
                        let CXI = CXH * GJ;
                        let CXJ = CXI * CWS;
                        let CXK = Lanes([((((DB * FB) * GJ) + (GL * CXH)) * CWS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CWT * CXI);
                        let CXL = CWP - BL;
                        let CXM = FB * CXC;
                        let CXN = CXD * FB;
                        let CXO = (CXG + (CXJ * CXL)) / CXM;
                        let CXP = (CXE * CWN) / CWV;
                        let CXQ = (CXP - (CXJ * CTO)) / CXM;
                        let CXR = (CWE * CXC) - CWG;
                        let CXS = (Lanes([(CWF * CXC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CXD * CWE)) - CWH;
                        let CXT = (CWE * CXO) - CWL;
                        let CXU = (Lanes([(CWF * CXO), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((CXF * CWL) + (CWM * CXE)) - Lanes([(CWX * CXG), 0.0, 0.0, 0.0, 0.0, 0.0])) / CWV) + ((CXK * CXL) + (CWQ * CXJ))) - (CXN * CXO)) / CXM) * CWE)) - CWM;
                        let CXV = (CWE * CXQ) - CWN;
                        let CXW = (Lanes([(CWF * CXQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((CXF * CWN) + (CWO * CXE)) - Lanes([(CWX * CXP), 0.0, 0.0, 0.0, 0.0, 0.0])) / CWV) - ((CXK * CTO) + (CTP * CXJ))) - (CXN * CXQ)) / CXM) * CWE)) - CWO;
                        CYI = CXR;
                        CYJ = CWG;
                        CYK = CXT;
                        CYL = CWL;
                        CYM = CXV;
                        CYN = CWN;
                        CYO = CXS;
                        CYP = CWH;
                        CYQ = CXU;
                        CYR = CWM;
                        CYS = CXW;
                        CYT = CWO;
                    } else {
                        let CXX = -GD;
                        let CXY = GE * BP;
                        let CYA = (CXX * CTO) / CXZ;
                        let CYB = (Lanes([(CXY * CTO), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CTP * CXX)) / CXZ;
                        let CYD = (CXX * DA) / CYC;
                        let CYE = ((CXY * DA) + (DB * CXX)) / CYC;
                        let CYF = -CYD;
                        let CYG = Lanes([CYE, 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let CYH = Lanes([(CYE * BP), 0.0, 0.0, 0.0, 0.0, 0.0]);
                        CYI = AI;
                        CYJ = CYA;
                        CYK = AI;
                        CYL = CYD;
                        CYM = AI;
                        CYN = CYF;
                        CYO = BCB;
                        CYP = CYB;
                        CYQ = BCB;
                        CYR = CYG;
                        CYS = BCB;
                        CYT = CYH;
                    }
                    CVK = CYI;
                    CVL = CYJ;
                    CVM = CYK;
                    CVN = CYL;
                    CVO = CYM;
                    CVP = CYN;
                    CVQ = CYO;
                    CVR = CYP;
                    CVS = CYQ;
                    CVT = CYR;
                    CVU = CYS;
                    CVV = CYT;
                }
                let CVW = CTJ - CRX;
                let CVX = DA * CVW;
                let CVY = Lanes([(DB * CVW), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CTK - CSI) * DA);
                let CVZ = -CVX;
                let CWA = CVY * BP;
                let CWB = if CVZ >= BNR { 1.0 } else { 0.0 };
                let CYY;
                let CYZ;
                let CZA;
                let CZB;
                if CWB != 0.0 {
                    let CYU = BOP * ((BL + CVZ) - BNR);
                    let CYV = CWA * BOP;
                    CYY = CYU;
                    CYZ = BOP;
                    CZA = CYV;
                    CZB = BCB;
                } else {
                    let CYW = CVZ.exp();
                    let CYX = CWA * CYW;
                    CYY = CYW;
                    CYZ = CYW;
                    CZA = CYX;
                    CZB = CYX;
                }
                let CZC = if CVW < -1e-8f64 { 1.0 } else { 0.0 };
                let CZP;
                let CZQ;
                let CZR;
                let CZS;
                let CZT;
                let CZU;
                let CZV;
                let CZW;
                let CZX;
                let CZY;
                let CZZ;
                let DAA;
                if CZC != 0.0 {
                    let CZD = ((CYY + CVX) - BL).sqrt();
                    let CZE = (CZA + CVY) * (DS / (ET * CZD));
                    let CZF = GD * CZD;
                    let CZG = Lanes([(GE * CZD), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CZE * GD);
                    let CZH = GD * DA;
                    let CZI = (-CYZ) + BL;
                    let CZJ = FB * CZD;
                    let CZK = (CZH * CZI) / CZJ;
                    let CZL = ((Lanes([(((GE * DA) + (DB * GD)) * CZI), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CZB * BP) * CZH)) - ((CZE * FB) * CZK)) / CZJ;
                    let CZM = -CZK;
                    let CZN = CZL * BP;
                    CZP = AI;
                    CZQ = CZF;
                    CZR = AI;
                    CZS = CZM;
                    CZT = AI;
                    CZU = CZK;
                    CZV = BCB;
                    CZW = CZG;
                    CZX = BCB;
                    CZY = CZN;
                    CZZ = BCB;
                    DAA = CZL;
                } else {
                    let CZO = if CVW > 1e-8f64 { 1.0 } else { 0.0 };
                    let DCI;
                    let DCJ;
                    let DCK;
                    let DCL;
                    let DCM;
                    let DCN;
                    let DCO;
                    let DCP;
                    let DCQ;
                    let DCR;
                    let DCS;
                    let DCT;
                    if CZO != 0.0 {
                        let DAC = ((CYY + CVX) - BL).sqrt();
                        let DAD = (CZA + CVY) * (DS / (ET * DAC));
                        let DAE = -GD;
                        let DAF = GE * BP;
                        let DAG = DAE * DAC;
                        let DAH = Lanes([(DAF * DAC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DAD * DAE);
                        let DAI = DAE * DA;
                        let DAJ = (-CYZ) + BL;
                        let DAK = FB * DAC;
                        let DAL = (DAI * DAJ) / DAK;
                        let DAM = ((Lanes([(((DAF * DA) + (DB * DAE)) * DAJ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CZB * BP) * DAI)) - ((DAD * FB) * DAL)) / DAK;
                        let DAN = -DAL;
                        let DAO = DAM * BP;
                        let DAP = CVX.exp();
                        let DAQ = CVY * DAP;
                        let DAR = CRX - CEU;
                        let DAS = (DA * DAR).exp();
                        let DAT = (Lanes([(DB * DAR), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CSI - Lanes([CEV[0], CEV[1], CEV[2], 0.0, CEV[3], CEV[4]])) * DA)) * DAS;
                        let DAU = DAH * DAG;
                        let DAV = GD * GD;
                        let DAW = GE * GD;
                        let DAX = DAW + DAW;
                        let DAY = (DAG * DAG) / DAV;
                        let DAZ = FB * GJ;
                        let DBA = DAZ * DAS;
                        let DBB = (DAP - CVX) - BL;
                        let DBC = (DAY + (DBA * DBB)).sqrt();
                        let DBD = ((((DAU + DAU) - Lanes([(DAX * DAY), 0.0, 0.0, 0.0, 0.0, 0.0])) / DAV) + (((Lanes([((GL * FB) * DAS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DAT * DAZ)) * DBB) + ((DAQ - CVY) * DBA))) * (DS / (ET * DBC));
                        let DBE = FB * DAG;
                        let DBF = DAH * FB;
                        let DBG = (DBE * DAL) / DAV;
                        let DBH = FB * DA;
                        let DBI = DBH * GJ;
                        let DBJ = DBI * DAS;
                        let DBK = Lanes([((((DB * FB) * GJ) + (GL * DBH)) * DAS), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DAT * DBI);
                        let DBL = DAP - BL;
                        let DBM = FB * DBC;
                        let DBN = DBD * FB;
                        let DBO = (DBG + (DBJ * DBL)) / DBM;
                        let DBP = (DBE * DAN) / DAV;
                        let DBQ = (DBP - (DBJ * CVX)) / DBM;
                        let DBR = (DAE * DBC) - DAG;
                        let DBS = (Lanes([(DAF * DBC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DBD * DAE)) - DAH;
                        let DBT = (DAE * DBO) - DAL;
                        let DBU = (Lanes([(DAF * DBO), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((DBF * DAL) + (DAM * DBE)) - Lanes([(DAX * DBG), 0.0, 0.0, 0.0, 0.0, 0.0])) / DAV) + ((DBK * DBL) + (DAQ * DBJ))) - (DBN * DBO)) / DBM) * DAE)) - DAM;
                        let DBV = (DAE * DBQ) - DAN;
                        let DBW = (Lanes([(DAF * DBQ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((((DBF * DAN) + (DAO * DBE)) - Lanes([(DAX * DBP), 0.0, 0.0, 0.0, 0.0, 0.0])) / DAV) - ((DBK * CVX) + (CVY * DBJ))) - (DBN * DBQ)) / DBM) * DAE)) - DAO;
                        DCI = DBR;
                        DCJ = DAG;
                        DCK = DBV;
                        DCL = DAN;
                        DCM = DBT;
                        DCN = DAL;
                        DCO = DBS;
                        DCP = DAH;
                        DCQ = DBW;
                        DCR = DAO;
                        DCS = DBU;
                        DCT = DAM;
                    } else {
                        let DBX = -GD;
                        let DBY = GE * BP;
                        let DCA = (DBX * CVX) / DBZ;
                        let DCB = (Lanes([(DBY * CVX), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CVY * DBX)) / DBZ;
                        let DCD = (DBX * DA) / DCC;
                        let DCE = ((DBY * DA) + (DB * DBX)) / DCC;
                        let DCF = -DCD;
                        let DCG = Lanes([(DCE * BP), 0.0, 0.0, 0.0, 0.0, 0.0]);
                        let DCH = Lanes([DCE, 0.0, 0.0, 0.0, 0.0, 0.0]);
                        DCI = AI;
                        DCJ = DCA;
                        DCK = AI;
                        DCL = DCF;
                        DCM = AI;
                        DCN = DCD;
                        DCO = BCB;
                        DCP = DCB;
                        DCQ = BCB;
                        DCR = DCG;
                        DCS = BCB;
                        DCT = DCH;
                    }
                    CZP = DCI;
                    CZQ = DCJ;
                    CZR = DCK;
                    CZS = DCL;
                    CZT = DCM;
                    CZU = DCN;
                    CZV = DCO;
                    CZW = DCP;
                    CZX = DCQ;
                    CZY = DCR;
                    CZZ = DCS;
                    DAA = DCT;
                }
                let DAB = if CRY == BL { 1.0 } else { 0.0 };
                let DDK;
                let DDL;
                let DDM;
                let DDN;
                let DDO;
                let DDP;
                let DDQ;
                let DDR;
                let DDS;
                if DAB != 0.0 {
                    DDK = CKH;
                    DDL = CRV;
                    DDM = CRW;
                    DDN = CRX;
                    DDO = CRY;
                    DDP = CRU;
                    DDQ = CSG;
                    DDR = CSH;
                    DDS = CSI;
                } else {
                    let DCU = (((((CTF + CVK) + CVL) + CZP) + CZQ) + BCC) / MI;
                    let DCV = ML * DCU;
                    let DCW = (CRW - ZG) - DCU;
                    let DCX = (CSH - Lanes([ZJ[0], ZJ[1], ZJ[2], 0.0, ZJ[3], ZJ[4]])) - (((((((CTH + CVQ) + CVR) + CZV) + CZW) + BCK) - Lanes([0.0, DCV[0], DCV[1], 0.0, DCV[2], DCV[3]])) / MI);
                    let DCY = (CVM + CVN) / MI;
                    let DCZ = ML * DCY;
                    let DDA = BL - DCY;
                    let DDB = (((CVS + CVT) - Lanes([0.0, DCZ[0], DCZ[1], 0.0, DCZ[2], DCZ[3]])) / MI) * BP;
                    let DDC = (-(((CVO + CVP) + CZR) + CZS)) / MI;
                    let DDD = ML * DDC;
                    let DDE = (((((CVU + CVV) + CZX) + CZY) * BP) - Lanes([0.0, DDD[0], DDD[1], 0.0, DDD[2], DDD[3]])) / MI;
                    let DDF = CZT + CZU;
                    let DDG = (-(CTG + (DDF * CTL))) / MI;
                    let DDH = ML * DDG;
                    let DDI = (((CTI + (((CZZ + DAA) * CTL) + (CTM * DDF))) * BP) - Lanes([0.0, DDH[0], DDH[1], 0.0, DDH[2], DDH[3]])) / MI;
                    let DDJ = if CTF <= BLM { 1.0 } else { 0.0 };
                    if DDJ != 0.0 {
                    } else {
                        let DDU = if CTF <= BLN { 1.0 } else { 0.0 };
                    }
                    let DDV = (-ZS) / XV;
                    let DDW = (AAA * BP) / XV;
                    let DDX = (-(DDV * BXW)).exp();
                    let DDY = BL + DDX;
                    let DDZ = BL / DDY;
                    let DEA = (((((((DDW * BXW) * BP) * DDX) * DDZ) * BP) / DDY) * BLO) * BP;
                    let DEB = (CVL + (-(XV + (DDZ * BLO)))) / BYB;
                    let DEC = (CVR + Lanes([DEA[0], DEA[1], DEA[2], 0.0, DEA[3], DEA[4]])) / BYB;
                    let DED = CVN / BYB;
                    let DEE = CVT / BYB;
                    let DEF = CVP / BYB;
                    let DEG = CVV / BYB;
                    let DEH = AI / BYB;
                    let DEI = (-(DDV * BYJ)).exp();
                    let DEJ = BL + DEI;
                    let DEK = BL / DEJ;
                    let DEL = ((((((DDW * BYJ) * BP) * DEI) * DEK) * BP) / DEJ) * BLO;
                    let DEM = (CZQ + (DEK * BLO)) / BYB;
                    let DEN = (CZW + Lanes([DEL[0], DEL[1], DEL[2], 0.0, DEL[3], DEL[4]])) / BYB;
                    let DEO = CZS / BYB;
                    let DEP = CZY / BYB;
                    let DEQ = (CZU * CTL) / BYB;
                    let DER = ((DAA * CTL) + (CTM * CZU)) / BYB;
                    let DES = DDA * DEF;
                    let DET = (DDB * DEF) + (DEG * DDA);
                    let DEU = DDA * DEH;
                    let DEV = DDB * DEH;
                    let DEW = DDC * DED;
                    let DEX = (DDE * DED) + (DEE * DDC);
                    let DEY = DDG * DED;
                    let DEZ = (DDI * DED) + (DEE * DDG);
                    let DFA = (((DES * DEQ) - (DEU * DEO)) - (DEW * DEQ)) + (DEY * DEO);
                    let DFB = ((((DET * DEQ) + (DER * DES)) - ((DEV * DEO) + (DEP * DEU))) - ((DEX * DEQ) + (DER * DEW))) + ((DEZ * DEO) + (DEP * DEY));
                    let DFC = if DFA > AI { 1.0 } else { 0.0 };
                    let DFJ;
                    let DFK;
                    if DFC != 0.0 {
                        let DFD = DFA + IH;
                        let DFE = BL / DFD;
                        let DFF = ((DFB * DFE) * BP) / DFD;
                        DFJ = DFE;
                        DFK = DFF;
                    } else {
                        let DFG = DFA - IH;
                        let DFH = BL / DFG;
                        let DFI = ((DFB * DFH) * BP) / DFG;
                        DFJ = DFH;
                        DFK = DFI;
                    }
                    let DFL = (DEF * DEQ) - (DEH * DEO);
                    let DFM = (DDG * DEO) - (DDC * DEQ);
                    let DFN = (DDC * DEH) - (DDG * DEF);
                    let DFO = -DED;
                    let DFP = DFO * DEQ;
                    let DFQ = DDA * DEQ;
                    let DFR = DEY - DEU;
                    let DFS = DED * DEO;
                    let DFT = -DDA;
                    let DFU = DFT * DEO;
                    let DFV = DES - DEW;
                    let DFW = -DFJ;
                    let DFX = DFK * BP;
                    let DFY = ((DFL * DCW) + (DFM * DEB)) + (DFN * DEM);
                    let DFZ = DFW * DFY;
                    let DGA = (DFX * DFY) + ((((((((DEG * DEQ) + (DER * DEF)) - (DEP * DEH)) * DCW) + (DCX * DFL)) + (((((DDI * DEO) + (DEP * DDG)) - ((DDE * DEQ) + (DER * DDC))) * DEB) + (DEC * DFM))) + ((((DDE * DEH) - ((DDI * DEF) + (DEG * DDG))) * DEM) + (DEN * DFN))) * DFW);
                    let DGB = ((DFP * DCW) + (DFQ * DEB)) + (DFR * DEM);
                    let DGC = DFW * DGB;
                    let DGD = (DFX * DGB) + ((((((((DEE * BP) * DEQ) + (DER * DFO)) * DCW) + (DCX * DFP)) + ((((DDB * DEQ) + (DER * DDA)) * DEB) + (DEC * DFQ))) + (((DEZ - DEV) * DEM) + (DEN * DFR))) * DFW);
                    let DGE = ((DFS * DCW) + (DFU * DEB)) + (DFV * DEM);
                    let DGF = DFW * DGE;
                    let DGG = (DFX * DGE) + (((((((DEE * DEO) + (DEP * DED)) * DCW) + (DCX * DFS)) + (((((DDB * BP) * DEO) + (DEP * DFT)) * DEB) + (DEC * DFU))) + (((DET - DEX) * DEM) + (DEN * DFV))) * DFW);
                    let DGH = DFZ.abs();
                    let DGI = DGA * ((ET * (if DFZ >= CAK { 1.0 } else { 0.0 })) - DS);
                    let DGJ = DGC.abs();
                    let DGK = DGD * ((ET * (if DGC >= CAK { 1.0 } else { 0.0 })) - DS);
                    let DGL = if DGH < DGJ { 1.0 } else { 0.0 };
                    let DGM;
                    let DGN;
                    if DGL != 0.0 {
                        DGM = DGJ;
                        DGN = DGK;
                    } else {
                        DGM = DGH;
                        DGN = DGI;
                    }
                    let DGO = DGF.abs();
                    let DGP = DGG * ((ET * (if DGF >= CAK { 1.0 } else { 0.0 })) - DS);
                    let DGQ = if DGM < DGO { 1.0 } else { 0.0 };
                    let DGR;
                    let DGS;
                    if DGQ != 0.0 {
                        DGR = DGO;
                        DGS = DGP;
                    } else {
                        DGR = DGM;
                        DGS = DGN;
                    }
                    let DGT = if CRU > BNR { 1.0 } else { 0.0 };
                    let DGV;
                    if DGT != 0.0 {
                        DGV = CAX;
                    } else {
                        let DGU = if CRU > CAY { 1.0 } else { 0.0 };
                        let DGZ;
                        if DGU != 0.0 {
                            DGZ = CAX;
                        } else {
                            let DGY = if CRU > CBD { 1.0 } else { 0.0 };
                            let DHB;
                            if DGY != 0.0 {
                                DHB = CAX;
                            } else {
                                let DHA = if CRU > CBG { 1.0 } else { 0.0 };
                                let DHC = if DHA != 0.0 {
                                    CBJ
                                } else {
                                    BL
                                };
                                DHB = DHC;
                            }
                            DGZ = DHB;
                        }
                        DGV = DGZ;
                    }
                    let DGW = EH / DGV;
                    let DGX = if DGR > DGW { 1.0 } else { 0.0 };
                    let DHL;
                    let DHM;
                    let DHN;
                    let DHO;
                    let DHP;
                    let DHQ;
                    if DGX != 0.0 {
                        let DHD = DGW / DGR;
                        let DHE = ((DGS * DHD) * BP) / DGR;
                        let DHF = DFZ * DHD;
                        let DHG = (DGA * DHD) + (DHE * DFZ);
                        let DHH = DGC * DHD;
                        let DHI = (DGD * DHD) + (DHE * DGC);
                        let DHJ = DGF * DHD;
                        let DHK = (DGG * DHD) + (DHE * DGF);
                        DHL = DHF;
                        DHM = DHH;
                        DHN = DHJ;
                        DHO = DHG;
                        DHP = DHI;
                        DHQ = DHK;
                    } else {
                        DHL = DFZ;
                        DHM = DGC;
                        DHN = DGF;
                        DHO = DGA;
                        DHP = DGD;
                        DHQ = DGG;
                    }
                    let DHR = CRW + DHL;
                    let DHS = CSH + DHO;
                    let DHT = CRX + DHM;
                    let DHU = CSI + DHP;
                    let DHV = CRV + DHN;
                    let DHW = CSG + DHQ;
                    let DHX = if DGR < (HZ * DGV) { 1.0 } else { 0.0 };
                    let DHY = if DHX != 0.0 {
                        BL
                    } else {
                        CRY
                    };
                    DDK = CRU;
                    DDL = DHV;
                    DDM = DHR;
                    DDN = DHT;
                    DDO = DHY;
                    DDP = CRZ;
                    DDQ = DHW;
                    DDR = DHS;
                    DDS = DHU;
                }
                let DDT = DDK + BL;
                CRU = DDT;
                CRV = DDL;
                CRW = DDM;
                CRX = DDN;
                CRY = DDO;
                CRZ = DDP;
                CSA = CTJ;
                CSB = CVK;
                CSC = CZP;
                CSD = CVL;
                CSE = CTF;
                CSF = CZQ;
                CSG = DDQ;
                CSH = DDR;
                CSI = DDS;
                CSJ = CTK;
                CSK = CVQ;
                CSL = CZV;
                CSM = CVR;
                CSN = CTH;
                CSO = CZW;
            }
            let CSV = if CRZ > AI { 1.0 } else { 0.0 };
            let DHZ = if CSV != 0.0 {
                CRZ
            } else {
                CRU
            };
            let DIA = if DHZ > CKH { 1.0 } else { 0.0 };
            let DIB;
            let DIC;
            let DID;
            let DIE;
            let DIF;
            let DIG;
            if DIA != 0.0 {
                DIB = CRI;
                DIC = CRJ;
                DID = CRK;
                DIE = CRM;
                DIF = CRN;
                DIG = CRO;
            } else {
                DIB = CRW;
                DIC = CSA;
                DID = CRV;
                DIE = CSH;
                DIF = CSJ;
                DIG = CSG;
            }
            let DIH = DIB - CCK;
            let DII = DIE - CCO;
            let DIJ = if (if ZU <= -1e0f64 { 1.0 } else { 0.0 }) != 0.0 || (if CCK < AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DIK = if DIJ != 0.0 {
                BL
            } else {
                CCW
            };
            let DIL = DIC - CCL;
            let DIM = DIF - CCP;
            let DIN = CSB - BLZ;
            let DIO = CSK - BMI;
            let DIP = CSB + BLZ;
            let DIQ = CSK + BMI;
            let DIR = DA * DIP;
            let DIS = DIN - ((DIR * DIH) * KF);
            let DIT = DIO - ((((Lanes([(DB * DIP), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DIQ * DA)) * DIH) + (DII * DIR)) * KF);
            let DIU = CSC + BMA;
            let DIV = CSL + BMJ;
            let DIW = DA * DIU;
            let DIX = (CSC - BMA) - ((DIW * DIL) * KF);
            let DIY = (CSL - BMJ) - ((((Lanes([(DB * DIU), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DIV * DA)) * DIL) + (DIM * DIW)) * KF);
            let DIZ = if CB == AI { 1.0 } else { 0.0 };
            let DJA = if (if DIS < AI { 1.0 } else { 0.0 }) != 0.0 || DIZ != 0.0 { 1.0 } else { 0.0 };
            let DJB;
            let DJC;
            if DJA != 0.0 {
                DJB = AI;
                DJC = BCB;
            } else {
                DJB = DIS;
                DJC = DIT;
            }
            let DJD = if (if DIX < AI { 1.0 } else { 0.0 }) != 0.0 || DIZ != 0.0 { 1.0 } else { 0.0 };
            let DJE;
            let DJF;
            if DJD != 0.0 {
                DJE = AI;
                DJF = BCB;
            } else {
                DJE = DIX;
                DJF = DIY;
            }
            let DJG = DJB + DJE;
            let DJH = DJC + DJF;
            let DJI = CSD + BMC;
            let DJJ = CSM + BML;
            let DJL = DJK * DJI;
            let DJM = DJJ * DJK;
            let DJN = DIH + HZ;
            let DJO = -DIN;
            let DJP = DIO * BP;
            let DJR = if (-DJO) < DJQ { 1.0 } else { 0.0 };
            let DJS;
            let DJT;
            if DJR != 0.0 {
                DJS = AI;
                DJT = BCB;
            } else {
                DJS = DJO;
                DJT = DJP;
            }
            let DJU = DA * MI;
            let DJV = ML * DA;
            let DJW = Lanes([(DB * MI), 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, DJV[0], DJV[1], DJV[2], DJV[3]]);
            let DJX = DJU * DJN;
            let DJY = DJW * DJN;
            let DJZ = DJX * DJN;
            let DKA = (FB * (-DJS)) / DJZ;
            let DKB = BL + DKA;
            let DKC = (DKB * DJN) / CDD;
            let DKD = BL - DKC;
            let DKE = ((((((((DJT * BP) * FB) - ((((Lanes([DJY[0], DJY[1], DJY[2], 0.0, DJY[3], DJY[4]]) + (DII * DJU)) * DJN) + (DII * DJX)) * DKA)) / DJZ) * DJN) + (DII * DKB)) - (CDF * DKC)) / CDD) * BP;
            let DKF = if DKD <= AI { 1.0 } else { 0.0 };
            let DKG;
            let DKH;
            if DKF != 0.0 {
                DKG = AI;
                DKH = BCB;
            } else {
                DKG = DKD;
                DKH = DKE;
            }
            let DKJ = DKI * DIP;
            let DKK = DIQ * DKI;
            let DKM = DKL * DIU;
            let DKN = DIV * DKL;
            let DKO = if DIK == AI { 1.0 } else { 0.0 };
            let DKS;
            let DKT;
            let DKU;
            let DKV;
            if DKO != 0.0 {
                let DKR = if (if DKP < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if DKQ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DLM;
                let DLN;
                let DLO;
                let DLP;
                if DKR != 0.0 {
                    let DKZ = CCK + JM;
                    let DLA = CCO + Lanes([0.0, 0.0, JN[0], 0.0, JN[1], JN[2]]);
                    let DLB = if DIB > (DKZ - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                    let DLU;
                    let DLV;
                    if DLB != 0.0 {
                        let DLT = DKZ - 2.220446049250313e-15f64;
                        DLU = DLT;
                        DLV = DLA;
                    } else {
                        DLU = DIB;
                        DLV = DIE;
                    }
                    DLM = AI;
                    DLN = DLU;
                    DLO = BCB;
                    DLP = DLV;
                } else {
                    let DLC = (DKP * JR) + ((DKQ * CCV) / PS);
                    let DLD = JS / DLC;
                    let DLE = ((((CCX * DKQ) / PS) * DLD) * BP) / DLC;
                    let DLG = BL - DLF;
                    let DLH = (DLF * (CB + CCK)) + (DLG * DIB);
                    let DLI = ((Lanes([0.0, 0.0, 0.0, 0.0, CK[0], CK[1]]) + CCO) * DLF) + (DIE * DLG);
                    let DLJ = CCK + JM;
                    let DLK = CCO + Lanes([0.0, 0.0, JN[0], 0.0, JN[1], JN[2]]);
                    let DLL = if DLH > (DLJ - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                    let DLX;
                    let DLY;
                    if DLL != 0.0 {
                        let DLW = DLJ - 2.220446049250313e-15f64;
                        DLX = DLW;
                        DLY = DLK;
                    } else {
                        DLX = DLH;
                        DLY = DLI;
                    }
                    let DLZ = DLX - DIB;
                    let DMA = DLY - DIE;
                    let DMB = DMA * DLZ;
                    let DMC = ((DLZ * DLZ) + 4e-6f64).sqrt();
                    let DMD = (DMA + ((DMB + DMB) * (DS / (ET * DMC)))) * KF;
                    let DME = (KF * (DLZ + DMC)) + 1e-13f64;
                    let DMF = if DME < AI { 1.0 } else { 0.0 };
                    let DMG;
                    let DMH;
                    if DMF != 0.0 {
                        DMG = AI;
                        DMH = BCB;
                    } else {
                        DMG = DME;
                        DMH = DMD;
                    }
                    let DMI = DA * CCV;
                    let DMJ = DJG / DMI;
                    let DMK = FB * (JR / JS);
                    let DML = DMK * DMG;
                    let DMM = DMH * DMK;
                    let DMO = (((FB * DMJ) + (DML * DLD)) + (DMN * DLD)) / BDJ;
                    let DMP = DMO * DLD;
                    let DMQ = (((((((DJH - ((Lanes([(DB * CCV), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CCX * DA)) * DMJ)) / DMI) * FB) + ((DMM * DLD) + (DLE * DML))) + (DLE * DMN)) / BDJ) * DLD) + (DLE * DMO);
                    let DMR = IZ * (DML + DMN);
                    let DMS = DMR * DLD;
                    let DMT = DMQ * DMP;
                    let DMU = ((DMP * DMP) + (DMS * DLD)).sqrt();
                    let DMV = KF * ((-DMP) + DMU);
                    let DMW = LI * DMV;
                    let DMX = LK * DMV;
                    let DMY = Lanes([DMX[0], DMX[1], DMX[2], 0.0, DMX[3], DMX[4]]) + ((((DMQ * BP) + (((DMT + DMT) + (((((DMM * IZ) * DLD) + (DLE * DMR)) * DLD) + (DLE * DMS))) * (DS / (ET * DMU)))) * KF) * LI);
                    DLM = DMW;
                    DLN = DLX;
                    DLO = DMY;
                    DLP = DLY;
                }
                let DLR = DLM * DLQ;
                let DLS = DLO * DLQ;
                DKS = DLR;
                DKT = DLN;
                DKU = DLS;
                DKV = DLP;
            } else {
                DKS = AI;
                DKT = AI;
                DKU = BCB;
                DKV = BCB;
            }
            let DKW = BDJ - DKS;
            let DKX = DKU * BP;
            let DKY = if DKW < AR { 1.0 } else { 0.0 };
            let DMZ;
            let DNA;
            if DKY != 0.0 {
                DMZ = AR;
                DNA = BCB;
            } else {
                DMZ = DKW;
                DNA = DKX;
            }
            let DNC = DNB * (DKJ + DKM);
            let DND = (DKK + DKN) * DNB;
            let DNF = ((KF * (BMD + CSE)) * BDJ) * DNE;
            let DNG = (((BMM + CSN) * KF) * BDJ) * DNE;
            let DNH = CB - DIH;
            let DNI = Lanes([0.0, 0.0, 0.0, 0.0, CK[0], CK[1]]);
            let DNJ = DNI - DII;
            let DNL = (FB * (DNH / FB)) / DNK;
            let DNM = ((DNJ / FB) * FB) / DNK;
            let DNO = 1.388888888888889e-3f64 + (DNL * DNN);
            let DNP = 8.333333333333333e-3f64 + (DNL * DNO);
            let DNQ = 4.1666666666666664e-2f64 + (DNL * DNP);
            let DNR = 1.6666666666666666e-1f64 + (DNL * DNQ);
            let DNS = 5e-1f64 + (DNL * DNR);
            let DNT = BL + (DNL * DNS);
            let DNU = DNK / DNT;
            let DNV = ((((DNM * DNS) + (((DNM * DNR) + (((DNM * DNQ) + (((DNM * DNP) + (((DNM * DNO) + ((DNM * DNN) * DNL)) * DNL)) * DNL)) * DNL)) * DNL)) * DNU) * BP) / DNT;
            let DNW = if DNU < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
            let DNY;
            let DNZ;
            if DNW != 0.0 {
                DNY = DNX;
                DNZ = BCB;
            } else {
                DNY = DNU;
                DNZ = DNV;
            }
            let DOA = CCK + DNY;
            let DOB = CCO + DNZ;
            let DOD = CSB / DOC;
            let DOE = CSK / DOC;
            let DOF = CSC / DOC;
            let DOG = CSL / DOC;
            let DOH = BME / DOC;
            let DOI = BMN / DOC;
            let DOJ = CSF / DOC;
            let DOK = CSO / DOC;
            let DOL = DKJ / DOC;
            let DOM = DKK / DOC;
            let DON = DKM / DOC;
            let DOO = DKN / DOC;
            let DOP = DJL / DOC;
            let DOQ = DJM / DOC;
            let DOR = DII * DIH;
            let DOS = ((DIH * DIH) + 4e-12f64).sqrt();
            let DOT = (DII + ((DOR + DOR) * (DS / (ET * DOS)))) * KF;
            let DOU = (KF * (DIH + DOS)) + 1e-16f64;
            let DOV = if DOU < AI { 1.0 } else { 0.0 };
            let DOW;
            let DOX;
            if DOV != 0.0 {
                DOW = AI;
                DOX = BCB;
            } else {
                DOW = DOU;
                DOX = DOT;
            }
            let DOY = DOX * DOW;
            let DOZ = ((DOW * DOW) + HN).sqrt();
            let DPB = DOZ - DPA;
            let DPF = BL + ((DPB.powf(DPC)) * DPE);
            let DPJ = ((DPH * DOP) + (DPI * (DOL - (DPG * DOF)))) / DPF;
            let DPK = (((DOQ * DPH) + ((DOM - (DOG * DPG)) * DPI)) - (((((DOY + DOY) * (DS / (ET * DOZ))) * (DPC * (DPB.powf(DPD)))) * DPE) * DPJ)) / DPF;
            let DPT;
            let DPU;
            let DPV;
            let DPW;
            let DPX;
            let DPY;
            if E != 0.0 {
                let DPL = (CCL + DIC) * KF;
                let DPM = (CCP + DIF) * KF;
                let DPN = (CCN + DID) * KF;
                let DPO = (CCR + DIG) * KF;
                let DPR = DPJ + ((DPP * ((DPL - DPN) - UL)) / DPQ);
                let DPS = DPK + ((((DPM - DPO) - Lanes([UM[0], 0.0, UM[1], 0.0, UM[2], UM[3]])) * DPP) / DPQ);
                DPT = DPR;
                DPU = DPL;
                DPV = DPN;
                DPW = DPS;
                DPX = DPM;
                DPY = DPO;
            } else {
                DPT = DPJ;
                DPU = AI;
                DPV = AI;
                DPW = DPK;
                DPX = BCB;
                DPY = BCB;
            }
            let DPZ = DPW * DPT;
            let DQA = ((DPT * DPT) + 3.6e7f64).sqrt();
            let DQB = (DPW + ((DPZ + DPZ) * (DS / (ET * DQA)))) * KF;
            let DQC = (KF * (DPT + DQA)) + 3e-7f64;
            let DQD = if DQC < AI { 1.0 } else { 0.0 };
            let DQE;
            let DQF;
            if DQD != 0.0 {
                DQE = AI;
                DQF = BCB;
            } else {
                DQE = DQC;
                DQF = DQB;
            }
            let DQH = DQE.powf(DQG);
            let DQL = staged[112] + ((DQJ * (DOL / CZ)) / DQK);
            let DQM = BL / DQL;
            let DQO = (DQM + (DV * DQH)) + ((DQE.powf(DQI)) / DQN);
            let DQP = BL / DQO;
            let DQQ = DQP * MZ;
            let DQR = (((((((((((DOM / CZ) * DQJ) / DQK) * DQM) * BP) / DQL) + (Lanes([(DW * DQH), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((DQF * (DQG * (DQE.powf(staged[159])))) * DV))) + ((DQF * (DQI * (DQE.powf(staged[160])))) / DQN)) * DQP) * BP) / DQO) * MZ;
            let DRA;
            let DRB;
            if E != 0.0 {
                let DQT = (DPP * (DPU - DPV)) / DQS;
                let DQU = ((DPX - DPY) * DPP) / DQS;
                DRA = DQT;
                DRB = DQU;
            } else {
                let DQV = DIM * DIL;
                let DQW = ((DIL * DIL) + 4e-12f64).sqrt();
                let DQX = (DIM + ((DQV + DQV) * (DS / (ET * DQW)))) * KF;
                let DQY = (KF * (DIL + DQW)) + 1e-16f64;
                let DQZ = if DQY < AI { 1.0 } else { 0.0 };
                let DRH;
                let DRI;
                if DQZ != 0.0 {
                    DRH = AI;
                    DRI = BCB;
                } else {
                    DRH = DQY;
                    DRI = DQX;
                }
                let DRJ = DRI * DRH;
                let DRK = ((DRH * DRH) + HN).sqrt();
                let DRL = DRK - DPA;
                let DRM = BL + ((DRL.powf(DPC)) * DPE);
                let DRP = ((DPH * (DRO * (DOJ + DOH))) + (DPI * (DON - (DRN * DOD)))) / DRM;
                let DRQ = (((((DOK + DOI) * DRO) * DPH) + ((DOO - (DOE * DRN)) * DPI)) - (((((DRJ + DRJ) * (DS / (ET * DRK))) * (DPC * (DRL.powf(DPD)))) * DPE) * DRP)) / DRM;
                DRA = DRP;
                DRB = DRQ;
            }
            let DRC = DRB * DRA;
            let DRD = ((DRA * DRA) + 3.6e3f64).sqrt();
            let DRE = (DRB + ((DRC + DRC) * (DS / (ET * DRD)))) * KF;
            let DRF = (KF * (DRA + DRD)) + 3e-9f64;
            let DRG = if DRF < AI { 1.0 } else { 0.0 };
            let DRR;
            let DRS;
            if DRG != 0.0 {
                DRR = AI;
                DRS = BCB;
            } else {
                DRR = DRF;
                DRS = DRE;
            }
            let DRU = DRR.powf(DRT);
            let DRX = staged[117] + ((DRW * (DON / CZ)) / DQK);
            let DRY = BL / DRX;
            let DSA = (DRY + (DY * DRU)) + ((DRR.powf(DRV)) / DRZ);
            let DSB = BL / DSA;
            let DSC = DSB * MZ;
            let DSD = (((((((((((DOO / CZ) * DRW) / DQK) * DRY) * BP) / DRX) + (Lanes([(DZ * DRU), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((DRS * (DRT * (DRR.powf(staged[161])))) * DY))) + ((DRS * (DRV * (DRR.powf(staged[162])))) / DRZ)) * DSB) * BP) / DSA) * MZ;
            let DSE = AAD * EQ;
            let DSF = DSE / DQQ;
            let DSG = Lanes([(ER * AAD), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let DSH = CCV + IH;
            let DSI = DA * DSH;
            let DSJ = DSI * DMZ;
            let DSK = DJB / DSJ;
            let DSL = ((DJC - ((((Lanes([(DB * DSH), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CCX * DA)) * DMZ) + (DNA * DSI)) * DSK)) / DSJ) * DSK;
            let DSM = ((DSG - (DQR * DSF)) / DQQ) * DSF;
            let DSN = ((DSK * DSK) + (DSF * DSF)).sqrt();
            let DSO = ((DSL + DSL) + (DSM + DSM)) * (DS / (ET * DSN));
            let DSP = (DQQ * DSN) / EQ;
            let DSQ = (((DQR * DSN) + (DSO * DQQ)) - Lanes([(ER * DSP), 0.0, 0.0, 0.0, 0.0, 0.0])) / EQ;
            let DST;
            let DSU;
            if DSR != 0.0 {
                DST = BL;
                DSU = BCB;
            } else {
                let DTB;
                let DTC;
                if DSS != 0.0 {
                    DTB = DSP;
                    DTC = DSQ;
                } else {
                    let DSZ = DSP.powf(DSY);
                    let DTA = DSQ * (DSY * (DSP.powf(staged[163])));
                    DTB = DSZ;
                    DTC = DTA;
                }
                DST = DTB;
                DSU = DTC;
            }
            let DSV = (DSQ * DST) + (DSU * DSP);
            let DSW = BL + (DSP * DST);
            let DTG;
            let DTH;
            if DSX != 0.0 {
                let DTD = BL / DSW;
                let DTE = ((DSV * DTD) * BP) / DSW;
                DTG = DTD;
                DTH = DTE;
            } else {
                let DUC;
                let DUD;
                if DTF != 0.0 {
                    let DTV = DSW.sqrt();
                    let DTW = BL / DTV;
                    let DTX = (((DSV * (DS / (ET * DTV))) * DTW) * BP) / DTV;
                    DUC = DTW;
                    DUD = DTX;
                } else {
                    let DTZ = DSW.powf(DTY);
                    let DUA = DSW * DTZ;
                    let DUB = (DSV * DTZ) + ((DSV * (DTY * (DSW.powf(staged[164])))) * DSW);
                    DUC = DUA;
                    DUD = DUB;
                }
                DTG = DUC;
                DTH = DUD;
            }
            let DTI = DQQ * DTG;
            let DTJ = (DQR * DTG) + (DTH * DQQ);
            let DTK = DSE / DSC;
            let DTL = CDB + IH;
            let DTM = DA * DTL;
            let DTN = DTM * DMZ;
            let DTO = DJE / DTN;
            let DTP = ((DJF - ((((Lanes([(DB * DTL), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CDC * DA)) * DMZ) + (DNA * DTM)) * DTO)) / DTN) * DTO;
            let DTQ = ((DSG - (DSD * DTK)) / DSC) * DTK;
            let DTR = ((DTO * DTO) + (DTK * DTK)).sqrt();
            let DTS = (DSC * DTR) / EQ;
            let DTT = (((DSD * DTR) + ((((DTP + DTP) + (DTQ + DTQ)) * (DS / (ET * DTR))) * DSC)) - Lanes([(ER * DTS), 0.0, 0.0, 0.0, 0.0, 0.0])) / EQ;
            let DUF;
            let DUG;
            if DTU != 0.0 {
                DUF = BL;
                DUG = BCB;
            } else {
                let DUN;
                let DUO;
                if DUE != 0.0 {
                    DUN = DTS;
                    DUO = DTT;
                } else {
                    let DUL = DTS.powf(DUK);
                    let DUM = DTT * (DUK * (DTS.powf(staged[165])));
                    DUN = DUL;
                    DUO = DUM;
                }
                DUF = DUN;
                DUG = DUO;
            }
            let DUH = (DTT * DUF) + (DUG * DTS);
            let DUI = BL + (DTS * DUF);
            let DUS;
            let DUT;
            if DUJ != 0.0 {
                let DUP = BL / DUI;
                let DUQ = ((DUH * DUP) * BP) / DUI;
                DUS = DUP;
                DUT = DUQ;
            } else {
                let DVJ;
                let DVK;
                if DUR != 0.0 {
                    let DVC = DUI.sqrt();
                    let DVD = BL / DVC;
                    let DVE = (((DUH * (DS / (ET * DVC))) * DVD) * BP) / DVC;
                    DVJ = DVD;
                    DVK = DVE;
                } else {
                    let DVG = DUI.powf(DVF);
                    let DVH = DUI * DVG;
                    let DVI = (DUH * DVG) + ((DUH * (DVF * (DUI.powf(staged[166])))) * DUI);
                    DVJ = DVH;
                    DVK = DVI;
                }
                DUS = DVJ;
                DUT = DVK;
            }
            let DUU = DSC * DUS;
            let DUV = (DSD * DUS) + (DUT * DSC);
            let DUW = (BFD * DF) / DKW;
            let DUX = (Lanes([(DG * BFD), 0.0, 0.0, 0.0, 0.0, 0.0]) - (DKX * DUW)) / DKW;
            let DUY = DUW * DJB;
            let DUZ = DUW * DJE;
            let DVA = (DUY * DTI) + (DUZ * DUU);
            let DVB = ((((DUX * DJB) + (DJC * DUW)) * DTI) + (DTJ * DUY)) + ((((DUX * DJE) + (DJF * DUW)) * DUU) + (DUV * DUZ));
            let DWF;
            let DWG;
            if A != 0.0 {
                let DVL = (FB * (KF * DNH)) / EG;
                let DVM = ((DNJ * KF) * FB) / EG;
                let DVO = 1.388888888888889e-3f64 + (DVL * DVN);
                let DVP = 8.333333333333333e-3f64 + (DVL * DVO);
                let DVQ = 4.1666666666666664e-2f64 + (DVL * DVP);
                let DVR = 1.6666666666666666e-1f64 + (DVL * DVQ);
                let DVS = 5e-1f64 + (DVL * DVR);
                let DVT = BL + (DVL * DVS);
                let DVU = EG / DVT;
                let DVV = CCK + DVU;
                let DVW = CCO + (((((DVM * DVS) + (((DVM * DVR) + (((DVM * DVQ) + (((DVM * DVP) + (((DVM * DVO) + ((DVM * DVN) * DVL)) * DVL)) * DVL)) * DVL)) * DVL)) * DVU) * BP) / DVT);
                let DVY = DVX - DVV;
                let DVZ = DVW * BP;
                let DWA = DVZ * DVY;
                let DWB = ((DVY * DVY) + 1.0000000000000002e-2f64).sqrt();
                let DWC = (DVZ + ((DWA + DWA) * (DS / (ET * DWB)))) * KF;
                let DWD = (KF * (DVY + DWB)) + 5.0000000000000005e-12f64;
                let DWE = if DWD < AI { 1.0 } else { 0.0 };
                let DWI;
                let DWJ;
                if DWE != 0.0 {
                    DWI = AI;
                    DWJ = BCB;
                } else {
                    DWI = DWD;
                    DWJ = DWC;
                }
                let DWL = DJU * DWK;
                let DWN = DWI.powf(DWM);
                let DWO = DWL * DWN;
                let DWP = (DJW * DWK) * DWN;
                let DWR = JN * DWQ;
                let DWT = JM * DWS;
                let DWU = DVV - JK;
                let DWV = (JN * DWS) * DWU;
                let DWW = (BL + (JM * DWQ)) + (DWT * DWU);
                let DWX = DWO * DWW;
                let DWY = ((Lanes([DWP[0], DWP[1], DWP[2], 0.0, DWP[3], DWP[4]]) + ((DWJ * (DWM * (DWI.powf(staged[167])))) * DWL)) * DWW) + ((Lanes([0.0, 0.0, DWR[0], 0.0, DWR[1], DWR[2]]) + (Lanes([0.0, 0.0, DWV[0], 0.0, DWV[1], DWV[2]]) + ((DVW - Lanes([0.0, 0.0, JL[0], 0.0, JL[1], JL[2]])) * DWT))) * DWO);
                DWF = DWX;
                DWG = DWY;
            } else {
                DWF = AI;
                DWG = BCB;
            }
            let DXE;
            let DXF;
            if DWH != 0.0 {
                let DXA = DJU * DWZ;
                let DXB = DXA * JM;
                let DXC = JN * DXA;
                let DXD = ((DJW * DWZ) * JM) + Lanes([0.0, 0.0, DXC[0], DXC[1], DXC[2]]);
                DXE = DXB;
                DXF = DXD;
            } else {
                DXE = AI;
                DXF = KJ;
            }
            let DXG = DWF + DXE;
            let DXH = Lanes([DXF[0], DXF[1], DXF[2], 0.0, DXF[3], DXF[4]]);
            let DXI = DWG + DXH;
            let DXJ = if DXG > AI { 1.0 } else { 0.0 };
            let DXX;
            let DXY;
            let DXZ;
            let DYA;
            let DYB;
            let DYC;
            if DXJ != 0.0 {
                let DXK = DIH * DXG;
                let DXL = DUW * DXK;
                let DXM = DXL * DTI;
                let DXN = (((DUX * DXK) + (((DII * DXG) + (DXI * DIH)) * DUW)) * DTI) + (DTJ * DXL);
                let DXO = -parameters[245];
                let DXP = (DXO * UL).exp();
                let DXQ = BL + DXP;
                let DXR = BL / DXQ;
                let DXS = ((((UM * DXO) * DXP) * DXR) * BP) / DXQ;
                let DXT = BL - DXR;
                let DXU = DXT * DXM;
                let DXV = (DXS * BP) * DXM;
                let DXW = Lanes([DXV[0], 0.0, DXV[1], 0.0, DXV[2], DXV[3]]) + (DXN * DXT);
                DXX = DXM;
                DXY = DXR;
                DXZ = DXU;
                DYA = DXN;
                DYB = DXS;
                DYC = DXW;
            } else {
                DXX = AI;
                DXY = AI;
                DXZ = AI;
                DYA = BCB;
                DYB = RZ;
                DYC = BCB;
            }
            let DYW;
            let DYX;
            if A != 0.0 {
                let DYD = (FB * (KF * (CB - DIL))) / EG;
                let DYE = (((DNI - DIM) * KF) * FB) / EG;
                let DYG = 1.388888888888889e-3f64 + (DYD * DYF);
                let DYH = 8.333333333333333e-3f64 + (DYD * DYG);
                let DYI = 4.1666666666666664e-2f64 + (DYD * DYH);
                let DYJ = 1.6666666666666666e-1f64 + (DYD * DYI);
                let DYK = 5e-1f64 + (DYD * DYJ);
                let DYL = BL + (DYD * DYK);
                let DYM = EG / DYL;
                let DYN = CCL + DYM;
                let DYO = CCP + (((((DYE * DYK) + (((DYE * DYJ) + (((DYE * DYI) + (((DYE * DYH) + (((DYE * DYG) + ((DYE * DYF) * DYD)) * DYD)) * DYD)) * DYD)) * DYD)) * DYM) * BP) / DYL);
                let DYP = DVX - DYN;
                let DYQ = DYO * BP;
                let DYR = DYQ * DYP;
                let DYS = ((DYP * DYP) + 1.0000000000000002e-2f64).sqrt();
                let DYT = (DYQ + ((DYR + DYR) * (DS / (ET * DYS)))) * KF;
                let DYU = (KF * (DYP + DYS)) + 5.0000000000000005e-12f64;
                let DYV = if DYU < AI { 1.0 } else { 0.0 };
                let DZB;
                let DZC;
                if DYV != 0.0 {
                    DZB = AI;
                    DZC = BCB;
                } else {
                    DZB = DYU;
                    DZC = DYT;
                }
                let DZD = DJU * DWK;
                let DZE = DZB.powf(DWM);
                let DZF = DZD * DZE;
                let DZG = (DJW * DWK) * DZE;
                let DZH = JN * DWQ;
                let DZI = JM * DWS;
                let DZJ = DYN - JK;
                let DZK = (JN * DWS) * DZJ;
                let DZL = (BL + (JM * DWQ)) + (DZI * DZJ);
                let DZM = DZF * DZL;
                let DZN = ((Lanes([DZG[0], DZG[1], DZG[2], 0.0, DZG[3], DZG[4]]) + ((DZC * (DWM * (DZB.powf(staged[168])))) * DZD)) * DZL) + ((Lanes([0.0, 0.0, DZH[0], 0.0, DZH[1], DZH[2]]) + (Lanes([0.0, 0.0, DZK[0], 0.0, DZK[1], DZK[2]]) + ((DYO - Lanes([0.0, 0.0, JL[0], 0.0, JL[1], JL[2]])) * DZI))) * DZF);
                DYW = DZM;
                DYX = DZN;
            } else {
                DYW = AI;
                DYX = BCB;
            }
            let DYY = DYW + DXE;
            let DYZ = DYX + DXH;
            let DZA = if DYY > AI { 1.0 } else { 0.0 };
            let DZX;
            let DZY;
            if DZA != 0.0 {
                let DZO = DIL * DYY;
                let DZP = DUW * DZO;
                let DZQ = DZP * DUU;
                let DZR = (((DUX * DZO) + (((DIM * DYY) + (DYZ * DIL)) * DUW)) * DUU) + (DUV * DZP);
                let DZS = DXX * KP;
                let DZT = DYA * KP;
                let DZU = DXX - DZS;
                let DZV = DYA - DZT;
                let DZW = if (if DZQ > DZU { 1.0 } else { 0.0 }) != 0.0 && (if DZS >= AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EAN;
                let EAO;
                if DZW != 0.0 {
                    let EAC = (DZQ - DXX) + DZS;
                    let EAD = (DZR - DYA) + DZT;
                    let EAE = EAC * EAC;
                    let EAF = EAD * EAC;
                    let EAG = DZS * DZS;
                    let EAH = DZT * DZS;
                    let EAI = (EAF + EAF) * EAE;
                    let EAJ = (EAH + EAH) * EAG;
                    let EAK = (EAE * EAE) + (EAG * EAG);
                    let EAL = (EAI + EAI) + (EAJ + EAJ);
                    let EAW;
                    let EAX;
                    if EAM != 0.0 {
                        let EBE;
                        if EAS != 0.0 {
                            EBE = BL;
                        } else {
                            let EBG;
                            if EBD != 0.0 {
                                EBG = FB;
                            } else {
                                let EBI;
                                if EBF != 0.0 {
                                    EBI = IW;
                                } else {
                                    let EBJ = if EBH != 0.0 {
                                        IZ
                                    } else {
                                        AI
                                    };
                                    EBI = EBJ;
                                }
                                EBG = EBI;
                            }
                            EBE = EBG;
                        }
                        let mut EBK = 0.0;
                        let mut EBL = 0.0;
                        let mut EBM = Lanes([0.0; 6]);
                        EBK = AI;
                        EBL = EAK;
                        EBM = EAL;
                        loop {
                            let EBN = if EBK < EBE { 1.0 } else { 0.0 };
                            if EBN == 0.0 {
                                break;
                            }
                            let EBO = EBL.sqrt();
                            let EBP = EBM * (DS / (ET * EBO));
                            let EBQ = EBK + BL;
                            EBK = EBQ;
                            EBL = EBO;
                            EBM = EBP;
                        }
                        EAW = EBL;
                        EAX = EBM;
                    } else {
                        let EAU = EAK.powf(EAT);
                        let EAV = EAL * (EAT * (EAK.powf(-7.5e-1f64)));
                        EAW = EAU;
                        EAX = EAV;
                    }
                    let EAY = EAW + IH;
                    let EAZ = BL / EAY;
                    let EBA = EAC * DZS;
                    let EBB = DZU + (EBA * EAZ);
                    let EBC = DZV + ((((EAD * DZS) + (DZT * EAC)) * EAZ) + ((((EAX * EAZ) * BP) / EAY) * EBA));
                    EAN = EBB;
                    EAO = EBC;
                } else {
                    EAN = DZQ;
                    EAO = DZR;
                }
                let EAP = DXY * EAN;
                let EAQ = DYB * EAN;
                let EAR = Lanes([EAQ[0], 0.0, EAQ[1], 0.0, EAQ[2], EAQ[3]]) + (EAO * DXY);
                DZX = EAP;
                DZY = EAR;
            } else {
                DZX = AI;
                DZY = BCB;
            }
            let DZZ = DVA + (DXZ + DZX);
            let EAA = DVB + (DYC + DZY);
            let EAB = if parameters[22] != AI { 1.0 } else { 0.0 };
            let ECZ;
            let EDA;
            if EAB != 0.0 {
                let EBR = staged[125] - parameters[57];
                let EBS = BL / (EBR * EBR);
                let EBT = FB * PU;
                let EBU = PR * EBT;
                let EBW = ((EBT * PQ) * EBV) * EBS;
                let EBX = EBW * ON;
                let EBY = OO * EBW;
                let ECA = parameters[158] + (EBZ * JM);
                let ECB = EBX * ECA;
                let ECC = (JN * EBZ) * EBX;
                let ECD = ((((((((PV * FB) * PQ) + Lanes([0.0, EBU[0], EBU[1], EBU[2], EBU[3]])) * EBV) * EBS) * ON) + Lanes([0.0, 0.0, EBY[0], EBY[1], EBY[2]])) * ECA) + Lanes([0.0, 0.0, ECC[0], ECC[1], ECC[2]]);
                let ECG = (CK * ECE) * BP;
                let ECH = JQ + Lanes([0.0, 0.0, ECG[0], ECG[1]]);
                let ECI = ((JO - JW) + (ECF - (ECE * CB))) + ECB;
                let ECJ = Lanes([0.0, ECH[0], ECH[1], ECH[2], ECH[3]]) + ECD;
                let ECK = FK * MG;
                let ECL = MJ * FK;
                let ECM = ECK * MG;
                let ECN = MJ * ECK;
                let ECO = ((Lanes([(FM * MG), 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, ECL[0], ECL[1], ECL[2], ECL[3]])) * MG) + Lanes([0.0, ECN[0], ECN[1], ECN[2], ECN[3]]);
                let ECP = (ECM * DA) * KF;
                let ECQ = ((ECO * DA) + Lanes([(DB * ECM), 0.0, 0.0, 0.0, 0.0])) * KF;
                let ECR = (ECP * DA) * FB;
                let ECS = ((ECQ * DA) + Lanes([(DB * ECP), 0.0, 0.0, 0.0, 0.0])) * FB;
                let ECT = DA * PH;
                let ECU = (CDM - ((ECO * ECT) + Lanes([((DB * PH) * ECM), 0.0, 0.0, 0.0, 0.0]))) - ECD;
                let ECV = ((((DF - (ECM * ECT)) + JW) - ECF) - ECB) + IH;
                let ECW = Lanes([0.0, JQ[0], JQ[1], JQ[2], JQ[3]]) - ECU;
                let ECX = (JO - ECV) - PL;
                let ECY = if ECV >= AI { 1.0 } else { 0.0 };
                let EDE = if ECY != 0.0 {
                    BL
                } else {
                    EDD
                };
                let EDF = ECW * ECX;
                let EDG = EDE * IZ;
                let EDH = ((ECX * ECX) + ((EDG * ECV) * PL)).sqrt();
                let EDI = ((((ECV + (KF * (ECX + EDH))) - JW) + ECF) + ECB) - JK;
                let EDJ = Lanes([0.0, 0.0, JL[0], JL[1], JL[2]]);
                let EDK = (DA * EDI) - BL;
                let EDL = IZ / ECR;
                let EDM = ((Lanes([(DB * EDI), 0.0, 0.0, 0.0, 0.0]) + ((((ECU + ((ECW + (((EDF + EDF) + ((ECU * EDG) * PL)) * (DS / (ET * EDH)))) * KF)) + ECD) - EDJ) * DA)) * EDL) + ((((ECS * EDL) * BP) / ECR) * EDK);
                let EDN = BL + (EDK * EDL);
                let EDO = EDM * EDN;
                let EDP = ((EDN * EDN) + 4e-4f64).sqrt();
                let EDQ = (EDM + ((EDO + EDO) * (DS / (ET * EDP)))) * KF;
                let EDR = (KF * (EDN + EDP)) + 1e-12f64;
                let EDS = if EDR < AI { 1.0 } else { 0.0 };
                let EDT;
                let EDU;
                if EDS != 0.0 {
                    EDT = AI;
                    EDU = KJ;
                } else {
                    EDT = EDR;
                    EDU = EDQ;
                }
                let EDV = (EDT + IH).sqrt();
                let EDW = BL - EDV;
                let EDX = ECI + (ECP * EDW);
                let EDY = ECJ + ((ECQ * EDW) + (((EDU * (DS / (ET * EDV))) * BP) * ECP));
                let EDZ = ECI + IH;
                let EEA = FB / EDZ;
                let EEB = DA + EEA;
                let EEC = BL / EEB;
                let EED = BL / BCE;
                let EEE = EED / ECM;
                let EEF = ECI * ECI;
                let EEG = ECJ * ECI;
                let EEH = EEE * EEF;
                let EEI = EEH.ln();
                let EEJ = EEI * EEC;
                let EEK = ((((((Lanes([(((BCM * EED) * BP) / BCE), 0.0, 0.0, 0.0, 0.0]) - (ECO * EEE)) / ECM) * EEF) + ((EEG + EEG) * EEE)) * (DS / EEH)) * EEC) + (((((Lanes([DB, 0.0, 0.0, 0.0, 0.0]) + (((ECJ * EEA) * BP) / EDZ)) * EEC) * BP) / EEB) * EEI);
                let EEL = EEK - EDY;
                let EEM = (EEJ - EDX) - BBN;
                let EEN = EEL * EEM;
                let EEO = IZ * BBN;
                let EEP = (EEM * EEM) + (EEO * EEJ);
                let EEQ = (EEN + EEN) + (EEK * EEO);
                let EER = EEQ * EEP;
                let EES = ((EEP * EEP) + 4e-12f64).sqrt();
                let EET = (EEQ + ((EER + EER) * (DS / (ET * EES)))) * KF;
                let EEU = (KF * (EEP + EES)) + 1e-16f64;
                let EEV = if EEU < AI { 1.0 } else { 0.0 };
                let EEW;
                let EEX;
                if EEV != 0.0 {
                    EEW = AI;
                    EEX = KJ;
                } else {
                    EEW = EEU;
                    EEX = EET;
                }
                let EEY = EEW.sqrt();
                let EEZ = EEJ - (KF * (EEM + EEY));
                let EFA = EEK - ((EEL + (EEX * (DS / (ET * EEY)))) * KF);
                let EFB = (DA * EEZ).exp();
                let EFC = EEZ - JK;
                let EFD = Lanes([(DB * EFC), 0.0, 0.0, 0.0, 0.0]) + ((EFA - EDJ) * DA);
                let EFE = (DA * EFC) - BL;
                let EFF = EFE + (BCE * EFB);
                let EFG = EFD + (Lanes([(BCM * EFB), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(DB * EEZ), 0.0, 0.0, 0.0, 0.0]) + (EFA * DA)) * EFB) * BCE));
                let EFH = EFG * EFF;
                let EFI = ((EFF * EFF) + 4e-4f64).sqrt();
                let EFJ = (EFG + ((EFH + EFH) * (DS / (ET * EFI)))) * KF;
                let EFK = (KF * (EFF + EFI)) + 1e-12f64;
                let EFL = if EFK < AI { 1.0 } else { 0.0 };
                let EFM;
                let EFN;
                if EFL != 0.0 {
                    EFM = AI;
                    EFN = KJ;
                } else {
                    EFM = EFK;
                    EFN = EFJ;
                }
                let EFO = (EFM + 2.220446049250313e-15f64).sqrt();
                let EFP = EFN * (DS / (ET * EFO));
                let EFQ = EFD * EFE;
                let EFR = ((EFE * EFE) + 4e-4f64).sqrt();
                let EFS = (EFD + ((EFQ + EFQ) * (DS / (ET * EFR)))) * KF;
                let EFT = (KF * (EFE + EFR)) + 1e-12f64;
                let EFU = if EFT < AI { 1.0 } else { 0.0 };
                let EFV;
                let EFW;
                if EFU != 0.0 {
                    EFV = AI;
                    EFW = KJ;
                } else {
                    EFV = EFT;
                    EFW = EFS;
                }
                let EFX = (EFV + 2.220446049250313e-15f64).sqrt();
                let EFY = EFO - EFX;
                let EFZ = BCF * EFY;
                let EGA = Lanes([(BCN * EFY), 0.0, 0.0, 0.0, 0.0]) + ((EFP - (EFW * (DS / (ET * EFX)))) * BCF);
                let EGB = EDX - EEZ;
                let EGC = EDY - EFA;
                let EGD = EGC * EGB;
                let EGE = ((EGB * EGB) + 4.000000000000001e-2f64).sqrt();
                let EGF = (EGC + ((EGD + EGD) * (DS / (ET * EGE)))) * KF;
                let EGG = (KF * (EGB + EGE)) + 1.0000000000000001e-11f64;
                let EGH = if EGG < AI { 1.0 } else { 0.0 };
                let EGI;
                let EGJ;
                if EGH != 0.0 {
                    EGI = AI;
                    EGJ = KJ;
                } else {
                    EGI = EGG;
                    EGJ = EGF;
                }
                let EGK = EGI + 2.220446049250313e-15f64;
                let EGL = CB / EGK;
                let EGM = (KZ - (EGJ * EGL)) / EGK;
                let EGN = EGL * EGL;
                let EGO = EGM * EGL;
                let EGP = EGO + EGO;
                let EGQ = EGN * EGN;
                let EGR = EGP * EGN;
                let EGS = EGQ * EGN;
                let EGT = ((((EGR + EGR) * EGN) + (EGP * EGQ)) * EGN) + (EGP * EGS);
                let EGU = (EGS * EGN) + 1e0f64;
                let EHA;
                let EHB;
                if EGV != 0.0 {
                    let EHP;
                    if EGW != 0.0 {
                        EHP = BL;
                    } else {
                        let EHR;
                        if EHO != 0.0 {
                            EHR = FB;
                        } else {
                            let EHT;
                            if EHQ != 0.0 {
                                EHT = IW;
                            } else {
                                let EHU = if EHS != 0.0 {
                                    IZ
                                } else {
                                    AI
                                };
                                EHT = EHU;
                            }
                            EHR = EHT;
                        }
                        EHP = EHR;
                    }
                    let mut EHV = 0.0;
                    let mut EHW = 0.0;
                    let mut EHX = Lanes([0.0; 5]);
                    EHV = AI;
                    EHW = EGU;
                    EHX = EGT;
                    loop {
                        let EHY = if EHV < EHP { 1.0 } else { 0.0 };
                        if EHY == 0.0 {
                            break;
                        }
                        let EHZ = EHW.sqrt();
                        let EIA = EHX * (DS / (ET * EHZ));
                        let EIB = EHV + BL;
                        EHV = EIB;
                        EHW = EHZ;
                        EHX = EIA;
                    }
                    EHA = EHW;
                    EHB = EHX;
                } else {
                    let EGY = EGU.powf(EGX);
                    let EGZ = EGT * (EGX * (EGU.powf(-8.75e-1f64)));
                    EHA = EGY;
                    EHB = EGZ;
                }
                let EHC = EHA + IH;
                let EHD = BL / EHC;
                let EHE = EGL * EHD;
                let EHF = staged[127] * parameters[5];
                let EHG = EHF * DF;
                let EHH = EHG * DTI;
                let EHI = EHH * EFZ;
                let EHJ = EGA * EHH;
                let EHK = ((EGM * EHD) + ((((EHB * EHD) * BP) / EHC) * EGL)) * EHI;
                let EHL = (EHI * EHE) / DMZ;
                let EHM = DZZ + EHL;
                let EHN = EAA + (((((((Lanes([((DG * EHF) * DTI), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DTJ * EHG)) * EFZ) + Lanes([EHJ[0], EHJ[1], EHJ[2], 0.0, EHJ[3], EHJ[4]])) * EHE) + Lanes([EHK[0], EHK[1], EHK[2], 0.0, EHK[3], EHK[4]])) - (DNA * EHL)) / DMZ);
                ECZ = EHM;
                EDA = EHN;
            } else {
                ECZ = DZZ;
                EDA = EAA;
            }
            let EDB = if parameters[23] != AI { 1.0 } else { 0.0 };
            let EDC = if (if parameters[20] != AI { 1.0 } else { 0.0 }) != 0.0 && EDB != 0.0 { 1.0 } else { 0.0 };
            let EIP;
            let EIQ;
            let EIR;
            let EIS;
            let EIT;
            let EIU;
            let EIV;
            if EDC != 0.0 {
                let EIC = CDD * CDD;
                let EID = CDF * CDD;
                let EIE = EID + EID;
                let EIF = FT * MG;
                let EIG = MJ * FT;
                let EIH = (Lanes([(FU * MG), 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, EIG[0], EIG[1], EIG[2], EIG[3]])) * DJG;
                let EII = EIC - (EIF * DJG);
                let EIJ = EIE - (Lanes([EIH[0], EIH[1], EIH[2], 0.0, EIH[3], EIH[4]]) + (DJH * EIF));
                let EIK = EIE * EIC;
                let EIL = ((EIC * EIC) + 4e-6f64).sqrt();
                let EIM = (EIE + ((EIK + EIK) * (DS / (ET * EIL)))) * KF;
                let EIN = (KF * (EIC + EIL)) + 1e-13f64;
                let EIO = if EIN < AI { 1.0 } else { 0.0 };
                let EIX;
                let EIY;
                if EIO != 0.0 {
                    EIX = AI;
                    EIY = BCB;
                } else {
                    EIX = EIN;
                    EIY = EIM;
                }
                let EIZ = EIJ * EII;
                let EJA = ((EII * EII) + 4e-6f64).sqrt();
                let EJB = (EIJ + ((EIZ + EIZ) * (DS / (ET * EJA)))) * KF;
                let EJC = (KF * (EII + EJA)) + 1e-13f64;
                let EJD = if EJC < AI { 1.0 } else { 0.0 };
                let EJE;
                let EJF;
                if EJD != 0.0 {
                    EJE = AI;
                    EJF = BCB;
                } else {
                    EJE = EJC;
                    EJF = EJB;
                }
                let EJG = EIX - EJE;
                let EJH = EIY - EJF;
                let EJI = if (if CCV < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 || (if EJG < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EJJ = if EJI != 0.0 {
                    AI
                } else {
                    BL
                };
                EIP = EJJ;
                EIQ = EJE;
                EIR = EIX;
                EIS = EJG;
                EIT = EJF;
                EIU = EIY;
                EIV = EJH;
            } else {
                EIP = AI;
                EIQ = AI;
                EIR = AI;
                EIS = AI;
                EIT = BCB;
                EIU = BCB;
                EIV = BCB;
            }
            let EIW = if BCG > AI { 1.0 } else { 0.0 };
            let EJX;
            let EJY;
            if EIW != 0.0 {
                let EJK = FB / BHV;
                let EJL = EJK * CDG;
                let EJM = JL * BHY;
                let EJN = (BCH - DF) - (BHY * JK);
                let EJO = (CDI * EJK) * EJN;
                let EJP = Lanes([0.0, EJO[0], EJO[1], EJO[2], EJO[3]]) + (((BCP - CDM) - Lanes([0.0, 0.0, EJM[0], EJM[1], EJM[2]])) * EJL);
                let EJQ = BL + (EJL * EJN);
                let EJR = EJP * EJQ;
                let EJS = ((EJQ * EJQ) + 4e-6f64).sqrt();
                let EJT = (EJP + ((EJR + EJR) * (DS / (ET * EJS)))) * KF;
                let EJU = (KF * (EJQ + EJS)) + 1e-13f64;
                let EJV = if EJU < AI { 1.0 } else { 0.0 };
                let EKB;
                let EKC;
                if EJV != 0.0 {
                    EKB = AI;
                    EKC = KJ;
                } else {
                    EKB = EJU;
                    EKC = EJT;
                }
                let EKD = (EKB + IH).sqrt();
                let EKE = BL - EKD;
                let EKF = CDK * EKE;
                let EKG = JN * BIT;
                let EKH = BIV * BIW;
                let EKI = ((BIT * JM) + BCI) - (EKH * ((BCH * BIO) + (CDJ * EKE)));
                let EKJ = (Lanes([0.0, 0.0, EKG[0], EKG[1], EKG[2]]) + BCQ) - (((BCP * BIO) + (Lanes([0.0, EKF[0], EKF[1], EKF[2], EKF[3]]) + (((EKC * (DS / (ET * EKD))) * BP) * CDJ))) * EKH);
                let EKK = EKJ * EKI;
                let EKL = ((EKI * EKI) + 4e-4f64).sqrt();
                let EKM = (EKJ + ((EKK + EKK) * (DS / (ET * EKL)))) * KF;
                let EKN = (KF * (EKI + EKL)) + 1e-12f64;
                let EKO = if EKN < AI { 1.0 } else { 0.0 };
                let EKP;
                let EKQ;
                if EKO != 0.0 {
                    EKP = AI;
                    EKQ = KJ;
                } else {
                    EKP = EKN;
                    EKQ = EKM;
                }
                let EKR = EKP + IH;
                let EKS = (-BJI) / EKR;
                let EKT = EKS.exp();
                let EKU = BFQ * EKR;
                let EKV = EKU * ECZ;
                let EKW = (EKQ * BFQ) * ECZ;
                let EKX = EKV * EKT;
                let EKY = ((((EKQ * EKS) * BP) / EKR) * EKT) * EKV;
                let EKZ = ((Lanes([EKW[0], EKW[1], EKW[2], 0.0, EKW[3], EKW[4]]) + (EDA * EKU)) * EKT) + Lanes([EKY[0], EKY[1], EKY[2], 0.0, EKY[3], EKY[4]]);
                EJX = EKX;
                EJY = EKZ;
            } else {
                let EJW = Lanes([BCO[0], BCO[1], BCO[2], 0.0, BCO[3], BCO[4]]);
                EJX = BCG;
                EJY = EJW;
            }
            let EKA = if (if DKO != 0.0 && (if EJX > AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if EJZ != AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ELM;
            let ELN;
            if EKA != 0.0 {
                let ELB = EJZ * (BL + (ELA * SI));
                let ELC = ELB * EJX;
                let ELD = ((SH * ELA) * EJZ) * EJX;
                let ELE = Lanes([ELD[0], ELD[1], ELD[2], 0.0, ELD[3], ELD[4]]) + (EJY * ELB);
                let ELF = Lanes([(DB * CCK), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CCO * DA);
                let ELG = (DA * CCK) - BL;
                let ELH = ELF * ELG;
                let ELI = ((ELG * ELG) + 4.000000000000001e-2f64).sqrt();
                let ELJ = (ELF + ((ELH + ELH) * (DS / (ET * ELI)))) * KF;
                let ELK = (KF * (ELG + ELI)) + 1.0000000000000001e-11f64;
                let ELL = if ELK < AI { 1.0 } else { 0.0 };
                let ELY;
                let ELZ;
                if ELL != 0.0 {
                    ELY = AI;
                    ELZ = BCB;
                } else {
                    ELY = ELK;
                    ELZ = ELJ;
                }
                let EMA = ELY.sqrt();
                let EMB = ELZ * (DS / (ET * EMA));
                let EMC = ELY * EMA;
                let EMD = (ELZ * EMA) + (EMB * ELY);
                let EME = Lanes([(DB * DIB), 0.0, 0.0, 0.0, 0.0, 0.0]) + (DIE * DA);
                let EMF = (DA * DIB) - BL;
                let EMG = EME * EMF;
                let EMH = ((EMF * EMF) + 4.000000000000001e-2f64).sqrt();
                let EMI = (EME + ((EMG + EMG) * (DS / (ET * EMH)))) * KF;
                let EMJ = (KF * (EMF + EMH)) + 1.0000000000000001e-11f64;
                let EMK = if EMJ < AI { 1.0 } else { 0.0 };
                let EML;
                let EMM;
                if EMK != 0.0 {
                    EML = AI;
                    EMM = BCB;
                } else {
                    EML = EMJ;
                    EMM = EMI;
                }
                let EMN = EML.sqrt();
                let EMO = EMM * (DS / (ET * EMN));
                let EMP = EML * EMN;
                let EMQ = DA * ELC;
                let EMR = Lanes([(DB * ELC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (ELE * DA);
                let EMS = EMQ / ELY;
                let EMT = (EMR - (ELZ * EMS)) / ELY;
                let EMU = EMQ / EML;
                let EMV = (EMR - (EMM * EMU)) / EML;
                let EMW = (EMP * EMU) - (EMC * EMS);
                let EMX = GD * KF;
                let EMY = -EMN;
                let EMZ = (EMY * EMU) + (EMA * EMS);
                let ENA = (GD * EMW) + (EMX * EMZ);
                let ENB = DUW * ENA;
                let ENC = ENB * DTI;
                let END = (((DUX * ENA) + (((Lanes([(GE * EMW), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((EMM * EMN) + (EMO * EML)) * EMU) + (EMV * EMP)) - ((EMD * EMS) + (EMT * EMC))) * GD)) + (Lanes([((GE * KF) * EMZ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((((EMO * BP) * EMU) + (EMV * EMY)) + ((EMB * EMS) + (EMT * EMA))) * EMX))) * DUW)) * DTI) + (DTJ * ENB);
                ELM = ENC;
                ELN = END;
            } else {
                ELM = AI;
                ELN = BCB;
            }
            let ELO = NJ * BFB;
            let ELP = MI / DOC;
            let ELQ = ML / DOC;
            let ELR = BDJ * BFB;
            let ELS = BFD * BFB;
            let ELT = DSN / BFB;
            let ELU = DSO / BFB;
            let ELV = GD / DOC;
            let ELW = GE / DOC;
            let ELX = if parameters[17] == AI { 1.0 } else { 0.0 };
            let ENF;
            let ENG;
            let ENH;
            let ENI;
            let ENJ;
            let ENK;
            let ENL;
            let ENM;
            let ENN;
            if ELX != 0.0 {
                ENF = AI;
                ENG = AI;
                ENH = AI;
                ENI = AI;
                ENJ = AI;
                ENK = LY;
                ENL = BCB;
                ENM = ENE;
                ENN = ENE;
            } else {
                let EOE;
                let EOF;
                if DKO != 0.0 {
                    let ENP = -parameters[258];
                    let ENQ = HK * ENP;
                    let ENS = Lanes([0.0, JQ[0], JQ[1], JQ[2], JQ[3]]) + ((Lanes([0.0, 0.0, ENQ[0], ENQ[1], ENQ[2]]) + ((SH - SQ) * ENR)) / ELR);
                    let ENU = ((JO - (parameters[256] * JW)) + (((ENP * HI) + (ENR * (SI - SN))) / ELR)) - (((DOA + JM) - 2.220446049250313e-15f64) * ENT);
                    let ENW = BL + (ELT / ENV);
                    let ENX = (ENW * ENU) / ELO;
                    let ENY = (((ELU / ENV) * ENU) + ((Lanes([ENS[0], ENS[1], ENS[2], 0.0, ENS[3], ENS[4]]) - ((DOB + Lanes([0.0, 0.0, JN[0], 0.0, JN[1], JN[2]])) * ENT)) * ENW)) / ELO;
                    let ENZ = ENY * ENX;
                    let EOA = ((ENX * ENX) + 4e-4f64).sqrt();
                    let EOB = (ENY + ((ENZ + ENZ) * (DS / (ET * EOA)))) * KF;
                    let EOC = (KF * (ENX + EOA)) + 1e-12f64;
                    let EOD = if EOC < AI { 1.0 } else { 0.0 };
                    let EOV;
                    let EOW;
                    if EOD != 0.0 {
                        EOV = AI;
                        EOW = BCB;
                    } else {
                        EOV = EOC;
                        EOW = EOB;
                    }
                    let EOX = JQ * JO;
                    let EOY = ((JO * JO) + 4e-6f64).sqrt();
                    let EOZ = (JQ + ((EOX + EOX) * (DS / (ET * EOY)))) * KF;
                    let EPA = (KF * (JO + EOY)) + 1e-13f64;
                    let EPB = if EPA < AI { 1.0 } else { 0.0 };
                    let EPC;
                    let EPD;
                    if EPB != 0.0 {
                        EPC = AI;
                        EPD = LY;
                    } else {
                        EPC = EPA;
                        EPD = EOZ;
                    }
                    let EPE = (EPC - HN) / EH;
                    let EPF = (EPD / EH) * EPE;
                    let EPG = BL + (EPE * EPE);
                    let EPH = BL / EPG;
                    let EPI = BL - EPH;
                    let EPJ = EOV * EPI;
                    let EPK = (((((EPF + EPF) * EPH) * BP) / EPG) * BP) * EOV;
                    let EPL = (EOW * EPI) + Lanes([0.0, EPK[0], EPK[1], 0.0, EPK[2], EPK[3]]);
                    let EPM = ELR * ELS;
                    let EPO = EPN / (EPN + EPM);
                    let EPQ = EPP + JM;
                    let EPR = EPP / EPQ;
                    let EPS = ((JN * EPR) * BP) / EPQ;
                    let EPT = EPL * EPJ;
                    let EPU = (EPJ * EPJ) + IH;
                    let EPV = BL / EPU;
                    let EPW = -parameters[204];
                    let EPX = EPW * EV;
                    let EPY = EPX * EPV;
                    let EPZ = Lanes([((EW * EPW) * EPV), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((((EPT + EPT) * EPV) * BP) / EPU) * EPX);
                    let EQA = if EPY < -3.4e1f64 { 1.0 } else { 0.0 };
                    let EQQ;
                    let EQR;
                    if EQA != 0.0 {
                        EQQ = AI;
                        EQR = BCB;
                    } else {
                        let EQB = parameters[203] / ES;
                        let EQC = (EQB * CZ) * EPM;
                        let EQD = ELQ * HZ;
                        let EQE = (DOL + (ELP * HZ)) / ELV;
                        let EQG = EQE.powf(EQF);
                        let EQH = EPY.exp();
                        let EQI = EQH * EQC;
                        let EQJ = EQI * EQG;
                        let EQK = EQJ * EPJ;
                        let EQL = EQK * EPJ;
                        let EQM = EPO * EPR;
                        let EQN = EQM * EQL;
                        let EQO = (EPS * EPO) * EQL;
                        let EQP = Lanes([0.0, 0.0, EQO[0], 0.0, EQO[1], EQO[2]]) + ((((((((((EPZ * EQH) * EQC) + Lanes([((((((EU * EQB) * BP) / ES) * CZ) * EPM) * EQH), 0.0, 0.0, 0.0, 0.0, 0.0])) * EQG) + (((((DOM + Lanes([0.0, EQD[0], EQD[1], 0.0, EQD[2], EQD[3]])) - Lanes([(ELW * EQE), 0.0, 0.0, 0.0, 0.0, 0.0])) / ELV) * (EQF * (EQE.powf((EQF - DS))))) * EQI)) * EPJ) + (EPL * EQJ)) * EPJ) + (EPL * EQK)) * EQM);
                        EQQ = EQN;
                        EQR = EQP;
                    }
                    EOE = EQQ;
                    EOF = EQR;
                } else {
                    EOE = AI;
                    EOF = BCB;
                }
                let EOG = -parameters[211];
                let EOI = (ELO * ((EOG * CC) + EOH)).exp();
                let EOK = EOJ * CC;
                let EOL = (BL / ELO) / ELO;
                let EOM = (CL * EOJ) * EOK;
                let EON = (EOK * EOK) * EOL;
                let EOQ = ((parameters[210] / EOO) * ELS) * (EOP.powf(parameters[259]));
                let EOR = EOQ * EOI;
                let EOS = EOR * EON;
                let EOT = (((((CL * EOG) * ELO) * EOI) * EOQ) * EON) + (((EOM + EOM) * EOL) * EOR);
                let EOU = if EOK >= AI { 1.0 } else { 0.0 };
                let EQV;
                let EQW;
                if EOU != 0.0 {
                    let EQT = EOS * EQS;
                    let EQU = EOT * EQS;
                    EQV = EQT;
                    EQW = EQU;
                } else {
                    EQV = EOS;
                    EQW = EOT;
                }
                let EQX = CC - CB;
                let EQY = CL - Lanes([0.0, CK[0], CK[1]]);
                let EQZ = (ELO * ((EOG * EQX) + EOH)).exp();
                let ERA = EOJ * EQX;
                let ERB = (EQY * EOJ) * ERA;
                let ERC = (ERA * ERA) * EOL;
                let ERD = EOQ * EQZ;
                let ERE = ERD * ERC;
                let ERF = (((((EQY * EOG) * ELO) * EQZ) * EOQ) * ERC) + (((ERB + ERB) * EOL) * ERD);
                let ERG = if ERA >= AI { 1.0 } else { 0.0 };
                let ERK;
                let ERL;
                if ERG != 0.0 {
                    let ERI = ERE * ERH;
                    let ERJ = ERF * ERH;
                    ERK = ERI;
                    ERL = ERJ;
                } else {
                    ERK = ERE;
                    ERL = ERF;
                }
                let ERM = -CC;
                let ERN = CL * BP;
                let ERP = HK * ERO;
                let ERQ = Lanes([ERN[0], 0.0, ERN[1], ERN[2]]);
                let ERR = (((ERM + (ERO * HI)) + JW) + parameters[215]) / ELO;
                let ERS = (ERQ + Lanes([0.0, ERP[0], ERP[1], ERP[2]])) / ELO;
                let ERT = ERS * ERR;
                let ERU = ((ERR * ERR) + 4e-4f64).sqrt();
                let ERV = (ERS + ((ERT + ERT) * (DS / (ET * ERU)))) * KF;
                let ERW = (KF * (ERR + ERU)) + 1e-12f64;
                let ERX = if ERW < AI { 1.0 } else { 0.0 };
                let ERY;
                let ERZ;
                if ERX != 0.0 {
                    ERY = AI;
                    ERZ = LY;
                } else {
                    ERY = ERW;
                    ERZ = ERV;
                }
                let ESA = ERY + IH;
                let ESC = ESA.powf(ESB);
                let ESD = (-parameters[214]) / ESC;
                let ESE = (((ERZ * (ESB * (ESA.powf((ESB - DS))))) * ESD) * BP) / ESC;
                let ESF = if ESD < -3.4e1f64 { 1.0 } else { 0.0 };
                let ESO;
                let ESP;
                if ESF != 0.0 {
                    ESO = AI;
                    ESP = LY;
                } else {
                    let ESG = ESD.exp();
                    let ESH = ESE * ESG;
                    let ESI = EOP + parameters[264];
                    let ESK = ESI * MN;
                    let ESL = (ESI - ESJ) - ESK;
                    let ESM = (IZ * ESJ) * ESK;
                    let ESN = if ESM > AI { 1.0 } else { 0.0 };
                    let ESR = if ESN != 0.0 {
                        ESM
                    } else {
                        let ESQ = -ESM;
                        ESQ
                    };
                    let ESS = (((ESJ + (KF * (ESL + (((ESL * ESL) + ESR).sqrt())))) * parameters[213]) / EOO) * ELS;
                    let ESU = ESS * (ESA.powf(EST));
                    let ESV = ESU * ESG;
                    let ESW = (((ERZ * (EST * (ESA.powf((EST - DS))))) * ESS) * ESG) + (ESH * ESU);
                    let ESY = HK * ESX;
                    let ESZ = (((ERM + (ESX * HI)) + JW) + parameters[268]) / ELO;
                    let ETA = (ERQ + Lanes([0.0, ESY[0], ESY[1], ESY[2]])) / ELO;
                    let ETB = ETA * ESZ;
                    let ETC = ((ESZ * ESZ) + 4e-4f64).sqrt();
                    let ETD = (ETA + ((ETB + ETB) * (DS / (ET * ETC)))) * KF;
                    let ETE = (KF * (ESZ + ETC)) + 1e-12f64;
                    let ETF = if ETE < AI { 1.0 } else { 0.0 };
                    let ETG;
                    let ETH;
                    if ETF != 0.0 {
                        ETG = AI;
                        ETH = LY;
                    } else {
                        ETG = ETE;
                        ETH = ETD;
                    }
                    let ETI = ETG + IH;
                    let ETK = ETI.powf(ETJ);
                    let ETL = (-parameters[267]) / ETK;
                    let ETM = (((ETH * (ETJ * (ETI.powf((ETJ - DS))))) * ETL) * BP) / ETK;
                    let ETN = if ETL < -3.4e1f64 { 1.0 } else { 0.0 };
                    let ETW;
                    let ETX;
                    if ETN != 0.0 {
                        ETW = AI;
                        ETX = LY;
                    } else {
                        let ETO = ETL.exp();
                        let ETP = ETM * ETO;
                        let ETQ = EOP + parameters[272];
                        let ETS = ETQ * MN;
                        let ETT = (ETQ - ETR) - ETS;
                        let ETU = (IZ * ETR) * ETS;
                        let ETV = if ETU > AI { 1.0 } else { 0.0 };
                        let EUE = if ETV != 0.0 {
                            ETU
                        } else {
                            let EUD = -ETU;
                            EUD
                        };
                        let EUF = (((ETR + (KF * (ETT + (((ETT * ETT) + EUE).sqrt())))) * parameters[266]) / EOO) * ELS;
                        let EUH = EUF * (ETI.powf(EUG));
                        let EUI = EUH * ETO;
                        let EUJ = (((ETH * (EUG * (ETI.powf((EUG - DS))))) * EUF) * ETO) + (ETP * EUH);
                        ETW = EUI;
                        ETX = EUJ;
                    }
                    let ETY = -ESV;
                    let ETZ = ESW * BP;
                    let EUA = ETY * MN;
                    let EUB = ETZ * MN;
                    let EUC = if EUA < IH { 1.0 } else { 0.0 };
                    let EUK;
                    let EUL;
                    if EUC != 0.0 {
                        EUK = IH;
                        EUL = LY;
                    } else {
                        EUK = EUA;
                        EUL = EUB;
                    }
                    let EUM = -ETW;
                    let EUN = ETX * BP;
                    let EUO = (ETY - EUM) - EUK;
                    let EUP = (ETZ - EUN) - EUL;
                    let EUQ = IZ * EUM;
                    let EUR = EUQ * EUK;
                    let EUS = ((EUN * IZ) * EUK) + (EUL * EUQ);
                    let EUT = if EUR > AI { 1.0 } else { 0.0 };
                    let EUW;
                    let EUX;
                    if EUT != 0.0 {
                        EUW = EUR;
                        EUX = EUS;
                    } else {
                        let EUU = -EUR;
                        let EUV = EUS * BP;
                        EUW = EUU;
                        EUX = EUV;
                    }
                    let EUY = EUP * EUO;
                    let EUZ = ((EUO * EUO) + EUW).sqrt();
                    let EVA = -(EUM + (KF * (EUO + EUZ)));
                    let EVB = (EUN + ((EUP + (((EUY + EUY) + EUX) * (DS / (ET * EUZ)))) * KF)) * BP;
                    ESO = EVA;
                    ESP = EVB;
                }
                ENF = ESO;
                ENG = KF;
                ENH = EOE;
                ENI = ERK;
                ENJ = EQV;
                ENK = ESP;
                ENL = EOF;
                ENM = ERL;
                ENN = EQW;
            }
            let ENO = if parameters[18] == AI { 1.0 } else { 0.0 };
            let EVP;
            let EVQ;
            if ENO != 0.0 {
                EVP = AI;
                EVQ = KJ;
            } else {
                let EVE = CK * EVD;
                let EVF = Lanes([0.0, EVE[0], EVE[1]]) - CL;
                let EVI = (((EVD * (CB + EVC)) - CC) - (SF * EVG)) / EVH;
                let EVJ = (Lanes([0.0, EVF[0], 0.0, EVF[1], EVF[2]]) - (SG * EVG)) / EVH;
                let EVK = EVJ * EVI;
                let EVL = ((EVI * EVI) + 4e-4f64).sqrt();
                let EVM = (EVJ + ((EVK + EVK) * (DS / (ET * EVL)))) * KF;
                let EVN = (KF * (EVI + EVL)) + 1e-12f64;
                let EVO = if EVN < AI { 1.0 } else { 0.0 };
                let EVR;
                let EVS;
                if EVO != 0.0 {
                    EVR = AI;
                    EVS = KJ;
                } else {
                    EVR = EVN;
                    EVS = EVM;
                }
                let EVU = -EVT;
                let EVV = EVR + IH;
                let EVW = (EVU * EV) / EVV;
                let EVX = (Lanes([(EW * EVU), 0.0, 0.0, 0.0, 0.0]) - (EVS * EVW)) / EVV;
                let EVY = if EVW < -3.4e1f64 { 1.0 } else { 0.0 };
                let EWQ;
                let EWR;
                if EVY != 0.0 {
                    EWQ = AI;
                    EWR = KJ;
                } else {
                    let EWA = EVZ / ES;
                    let EWB = (EWA * CZ) * BFD;
                    let EWC = EWB * EVR;
                    let EWD = EWC * EVR;
                    let EWE = EVW.exp();
                    let EWF = -DA;
                    let EWG = CK * EWF;
                    let EWH = (EWF * CB).exp();
                    let EWI = BL + EWH;
                    let EWJ = (EWD * EWE) / EWI;
                    let EWK = ((Lanes([((DB * BP) * CB), 0.0, 0.0]) + Lanes([0.0, EWG[0], EWG[1]])) * EWH) * EWJ;
                    let EWL = (-BDJ) / DL;
                    let EWM = EWL.exp();
                    let EWN = BL - EWM;
                    let EWO = EWJ / EWN;
                    let EWP = ((((((((Lanes([((((((EU * EWA) * BP) / ES) * CZ) * BFD) * EVR), 0.0, 0.0, 0.0, 0.0]) + (EVS * EWB)) * EVR) + (EVS * EWC)) * EWE) + ((EVX * EWE) * EWD)) - Lanes([EWK[0], 0.0, 0.0, EWK[1], EWK[2]])) / EWI) - Lanes([((((((DM * EWL) * BP) / DL) * EWM) * BP) * EWO), 0.0, 0.0, 0.0, 0.0])) / EWN;
                    EWQ = EWO;
                    EWR = EWP;
                }
                EVP = EWQ;
                EVQ = EWR;
            }
            let EXB;
            let EXC;
            if ENO != 0.0 {
                EXB = AI;
                EXC = KJ;
            } else {
                let EWS = (CK * BP) * EVD;
                let EWT = Lanes([0.0, EWS[0], EWS[1]]) - (CL - Lanes([0.0, CK[0], CK[1]]));
                let EWU = (((EVD * ((-CB) + EVC)) - (CC - CB)) - (SF * EVG)) / EVH;
                let EWV = (Lanes([0.0, EWT[0], 0.0, EWT[1], EWT[2]]) - (SG * EVG)) / EVH;
                let EWW = EWV * EWU;
                let EWX = ((EWU * EWU) + 4e-4f64).sqrt();
                let EWY = (EWV + ((EWW + EWW) * (DS / (ET * EWX)))) * KF;
                let EWZ = (KF * (EWU + EWX)) + 1e-12f64;
                let EXA = if EWZ < AI { 1.0 } else { 0.0 };
                let EXE;
                let EXF;
                if EXA != 0.0 {
                    EXE = AI;
                    EXF = KJ;
                } else {
                    EXE = EWZ;
                    EXF = EWY;
                }
                let EXG = -EVT;
                let EXH = EXE + IH;
                let EXI = (EXG * EV) / EXH;
                let EXJ = (Lanes([(EW * EXG), 0.0, 0.0, 0.0, 0.0]) - (EXF * EXI)) / EXH;
                let EXK = if EXI < -3.4e1f64 { 1.0 } else { 0.0 };
                let EYA;
                let EYB;
                if EXK != 0.0 {
                    EYA = AI;
                    EYB = KJ;
                } else {
                    let EXL = EVZ / ES;
                    let EXM = (EXL * CZ) * BFD;
                    let EXN = EXM * EXE;
                    let EXO = EXN * EXE;
                    let EXP = EXI.exp();
                    let EXQ = CK * DA;
                    let EXR = (DA * CB).exp();
                    let EXS = BL + EXR;
                    let EXT = (EXO * EXP) / EXS;
                    let EXU = ((Lanes([(DB * CB), 0.0, 0.0]) + Lanes([0.0, EXQ[0], EXQ[1]])) * EXR) * EXT;
                    let EXV = (-BDJ) / DL;
                    let EXW = EXV.exp();
                    let EXX = BL - EXW;
                    let EXY = EXT / EXX;
                    let EXZ = ((((((((Lanes([((((((EU * EXL) * BP) / ES) * CZ) * BFD) * EXE), 0.0, 0.0, 0.0, 0.0]) + (EXF * EXM)) * EXE) + (EXF * EXN)) * EXP) + ((EXJ * EXP) * EXO)) - Lanes([EXU[0], 0.0, 0.0, EXU[1], EXU[2]])) / EXS) - Lanes([((((((DM * EXV) * BP) / DL) * EXW) * BP) * EXY), 0.0, 0.0, 0.0, 0.0])) / EXX;
                    EYA = EXY;
                    EYB = EXZ;
                }
                EXB = EYA;
                EXC = EYB;
            }
            let EXD = if DIK != AI { 1.0 } else { 0.0 };
            let EYK;
            let EYL;
            let EYM;
            let EYN;
            if EXD != 0.0 {
                let EYC = CB + CCK;
                let EYD = DNI + CCO;
                let EYE = BL - DLF;
                let EYF = (DLF * EYC) + (EYE * DIB);
                let EYG = (EYD * DLF) + (DIE * EYE);
                let EYH = if EYF > (EYC - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                let EYU;
                let EYV;
                if EYH != 0.0 {
                    let EYT = EYC - 2.220446049250313e-15f64;
                    EYU = EYT;
                    EYV = EYD;
                } else {
                    EYU = EYF;
                    EYV = EYG;
                }
                EYK = EYU;
                EYL = AI;
                EYM = EYV;
                EYN = BCB;
            } else {
                let EYJ = if EYI != AI { 1.0 } else { 0.0 };
                let EYX;
                let EYY;
                if EYJ != 0.0 {
                    let EYW = if DJG > 1e-15f64 { 1.0 } else { 0.0 };
                    let EZB;
                    let EZC;
                    if EYW != 0.0 {
                        let EYZ = ((DJG * DF) / BDJ) / CCV;
                        let EZA = ((((DJH * DF) + Lanes([(DG * DJG), 0.0, 0.0, 0.0, 0.0, 0.0])) / BDJ) - (CCX * EYZ)) / CCV;
                        EZB = EYZ;
                        EZC = EZA;
                    } else {
                        EZB = AI;
                        EZC = BCB;
                    }
                    EYX = EZB;
                    EYY = EZC;
                } else {
                    EYX = AI;
                    EYY = BCB;
                }
                EYK = DKT;
                EYL = EYX;
                EYM = DKV;
                EYN = EYY;
            }
            let EYO = BL / LR;
            let EYQ = if EYP > AI { 1.0 } else { 0.0 };
            let EYS = if (if (if parameters[19] >= BL { 1.0 } else { 0.0 }) != 0.0 && EYQ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if EYR > AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let EZR;
            let EZS;
            let EZT;
            let EZU;
            let EZV;
            let EZW;
            let EZX;
            let EZY;
            let EZZ;
            let FAA;
            let FAB;
            let FAC;
            if EYS != 0.0 {
                let EZD = (EYR / staged[132]).sqrt();
                let EZE = GD * EZD;
                let EZF = GE * EZD;
                let EZH = CD + (EZG * CE);
                let EZI = CE + (EZG * CD);
                let FAG;
                let FAH;
                if EZJ != 0.0 {
                    let FAD = (CD * CF) + (CE * (CF - CG));
                    let FAE = (CM * CD) + ((CM - Lanes([CN[0], CN[1], 0.0])) * CE);
                    FAG = FAD;
                    FAH = FAE;
                } else {
                    FAG = AI;
                    FAH = FAF;
                }
                let FAK;
                let FAL;
                if EZG != 0.0 {
                    let FAI = (CE * CF) + (CD * (CF - CG));
                    let FAJ = (CM * CE) + ((CM - Lanes([CN[0], CN[1], 0.0])) * CD);
                    FAK = FAI;
                    FAL = FAJ;
                } else {
                    FAK = FAG;
                    FAL = FAH;
                }
                let FAN = if FAM > GR { 1.0 } else { 0.0 };
                let FAS = if FAN != 0.0 {
                    let FAO = GU - GR;
                    let FAP = (FAM - GR) / FAO;
                    let FAQ = FAP * FAP;
                    let FAR = GR + (FAO * (BL - (BL / ((((BL + FAP) + FAQ) + (FAQ * FAP)) + (FAQ * FAQ)))));
                    FAR
                } else {
                    FAM
                };
                let FAT = (-FAS) - HZ;
                let FAU = EZE * EYO;
                let FAV = EZF * EYO;
                let FAW = FAU * FAU;
                let FAX = FAV * FAU;
                let FAY = FAX + FAX;
                let FAZ = FAL * BP;
                let FBB = (-FAK) + FBA;
                let FBC = EYR / FE;
                let FBD = FBC.ln();
                let FBE = VB * FBD;
                let FBF = (VC * FBD) + (((((FF * FBC) * BP) / FE) * (DS / FBC)) * VB);
                let FBG = -FAT;
                let FBH = if FBB < FBG { 1.0 } else { 0.0 };
                let FCT;
                let FCU;
                let FCV;
                let FCW;
                let FCX;
                let FCY;
                let FCZ;
                let FDA;
                let FDB;
                let FDC;
                if FBH != 0.0 {
                    let FBI = DA * EZE;
                    let FBJ = LR / FBI;
                    let FBK = ((((DB * EZE) + (EZF * DA)) * FBJ) * BP) / FBI;
                    let FBM = FBK * FBL;
                    let FBN = FB + (FBL * FBJ);
                    let FBP = FBO * FBN;
                    let FBQ = FBP * FBN;
                    let FBR = FBQ * FBN;
                    let FBS = ((((FBM * FBO) * FBN) + (FBM * FBP)) * FBN) + (FBM * FBQ);
                    let FBT = CV - FBE;
                    let FBU = CW - FBF;
                    let FBV = FBB + FAT;
                    let FBW = FAZ * DA;
                    let FBY = FBX * FBJ;
                    let FBZ = (DA * FBV) - FB;
                    let FCA = FBY * FBZ;
                    let FCB = Lanes([0.0, 0.0, ((FBK * FBX) * FBZ), 0.0]) + ((Lanes([0.0, 0.0, (DB * FBV), 0.0]) + Lanes([FBW[0], FBW[1], 0.0, FBW[2]])) * FBY);
                    let FCC = 9.899494936611664e0f64 - FCA;
                    let FCD = FCB * BP;
                    let FCE = FCC * FCC;
                    let FCF = FCD * FCC;
                    let FCG = FCF + FCF;
                    let FCH = if FBR < (FCE * WZ) { 1.0 } else { 0.0 };
                    let FDK;
                    let FDL;
                    if FCH != 0.0 {
                        let FDE = (KF * FBR) / FCC;
                        let FDF = ((-9.899494936611664e0f64 + FCC) + FDE) + FCA;
                        let FDG = (FCD + ((Lanes([0.0, 0.0, (FBS * KF), 0.0]) - (FCD * FDE)) / FCC)) + FCB;
                        FDK = FDF;
                        FDL = FDG;
                    } else {
                        let FDH = (FBR + FCE).sqrt();
                        let FDI = (-9.899494936611664e0f64 + FDH) + FCA;
                        let FDJ = ((Lanes([0.0, 0.0, FBS, 0.0]) + FCG) * (DS / (ET * FDH))) + FCB;
                        FDK = FDI;
                        FDL = FDJ;
                    }
                    let FDN = FDK.powf(FDM);
                    let FDO = FDL * (FDM * (FDK.powf(-6.666666666666667e-1f64)));
                    let FDR = FDQ * FDN;
                    let FDS = (((-5.65685424949238e0f64 - (FDP * FBJ)) + (FB * FDN)) + (FDR * FDN)) / FDN;
                    let FDT = (((((Lanes([0.0, 0.0, ((FBK * FDP) * BP), 0.0]) + (FDO * FB)) + (((FDO * FDQ) * FDN) + (FDO * FDR))) - (FDO * FDS)) / FDN) * DF) + Lanes([0.0, 0.0, (DG * FDS), 0.0]);
                    let FDU = ((FDS * DF) - FAT) + FAT;
                    let FDV = FDU / FBT;
                    let FDW = ((FDT - Lanes([0.0, 0.0, (FBU * FDV), 0.0])) / FBT) * FDV;
                    let FDX = (BL + (FDV * FDV)).sqrt();
                    let FDY = FDU / FDX;
                    let FDZ = LR * (FBB - (FDY - FAT));
                    let FEA = (Lanes([FAZ[0], FAZ[1], 0.0, FAZ[2]]) - ((FDT - (((FDW + FDW) * (DS / (ET * FDX))) * FDY)) / FDX)) * LR;
                    FCT = FDZ;
                    FCU = FDZ;
                    FCV = AI;
                    FCW = AI;
                    FCX = AI;
                    FCY = FEA;
                    FCZ = FEA;
                    FDA = FEB;
                    FDB = FEB;
                    FDC = FEB;
                } else {
                    let FCI = FBB + FAT;
                    let FCJ = FAZ * DA;
                    let FCK = Lanes([FCJ[0], FCJ[1], 0.0, FCJ[2]]);
                    let FCL = Lanes([0.0, 0.0, (DB * FCI), 0.0]) + FCK;
                    let FCM = (DA * FCI) - BL;
                    let FCN = FAW * DC;
                    let FCO = (FAY * DC) + (DE * FAW);
                    let FCP = (IZ * (FCM + 4.9787068367863944e-2f64)) / FCN;
                    let FCQ = ((FCL * IZ) - Lanes([0.0, 0.0, (FCO * FCP), 0.0])) / FCN;
                    let FCR = BL + FCP;
                    let FCS = if FCR < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let FED;
                    let FEE;
                    if FCS != 0.0 {
                        FED = FEC;
                        FEE = FEB;
                    } else {
                        FED = FCR;
                        FEE = FCQ;
                    }
                    let FEF = (FAW * DA) / FB;
                    let FEG = ((FAY * DA) + (DB * FAW)) / FB;
                    let FEH = FED.sqrt();
                    let FEI = BL - FEH;
                    let FEJ = Lanes([FAZ[0], FAZ[1], 0.0, FAZ[2]]);
                    let FEK = (FBB + (FEF * FEI)) + FAT;
                    let FEL = (-(DA * FEK)).exp();
                    let FEM = (IZ * (FCM + FEL)) / FCN;
                    let FEN = (((FCL + (((Lanes([0.0, 0.0, (DB * FEK), 0.0]) + ((FEJ + (Lanes([0.0, 0.0, (FEG * FEI), 0.0]) + (((FEE * (DS / (ET * FEH))) * BP) * FEF))) * DA)) * BP) * FEL)) * IZ) - Lanes([0.0, 0.0, (FCO * FEM), 0.0])) / FCN;
                    let FEO = BL + FEM;
                    let FEP = if FEO < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let FER;
                    let FES;
                    if FEP != 0.0 {
                        FER = FEQ;
                        FES = FEB;
                    } else {
                        FER = FEO;
                        FES = FEN;
                    }
                    let FET = FER.sqrt();
                    let FEU = BL - FET;
                    let FEV = (FBB + (FEF * FEU)) + FAT;
                    let FEW = DA * FEV;
                    let FEX = Lanes([0.0, 0.0, (DB * FEV), 0.0]) + ((FEJ + (Lanes([0.0, 0.0, (FEG * FEU), 0.0]) + (((FES * (DS / (ET * FET))) * BP) * FEF))) * DA);
                    let FEY = if FEW < IW { 1.0 } else { 0.0 };
                    let FFZ;
                    let FGA;
                    if FEY != 0.0 {
                        let FEZ = DA * FAU;
                        let FFA = BL / FEZ;
                        let FFB = ((((DB * FAU) + (FAV * DA)) * FFA) * BP) / FEZ;
                        let FFC = 7.071067811865476e-1f64 + FFA;
                        let FFD = FAZ * BP;
                        let FFE = (-FCI) / FAU;
                        let FFI = (-5.151950988020902e1f64 - ((FFF * FFC) / FFG)) + (FFE / FFH);
                        let FFJ = Lanes([0.0, 0.0, (((FFB * FFF) / FFG) * BP), 0.0]) + (((Lanes([FFD[0], FFD[1], 0.0, FFD[2]]) - Lanes([0.0, 0.0, (FAV * FFE), 0.0])) / FAU) / FFH);
                        let FFM = ((FFK * FFC) - 1.0979672760764175e-2f64) / FFL;
                        let FFN = (FFB * FFK) / FFL;
                        let FFO = FFJ * FFI;
                        let FFP = FFM * FFM;
                        let FFQ = FFN * FFM;
                        let FFR = ((FFI * FFI) + (FFP * FFM)).sqrt();
                        let FFS = ((FFO + FFO) + Lanes([0.0, 0.0, (((FFQ + FFQ) * FFM) + (FFN * FFP)), 0.0])) * (DS / (ET * FFR));
                        let FFT = (-FFI) + FFR;
                        let FFU = FFI + FFR;
                        let FFV = ((FFT.powf(FDM)) + (-(FFU.powf(FDM)))) - -3.7209791878387604e0f64;
                        let FFW = ((FFV * DF) - FAT) + FAT;
                        let FFX = DA * FFW;
                        let FFY = Lanes([0.0, 0.0, (DB * FFW), 0.0]) + (((((((FFJ * BP) + FFS) * (FDM * (FFT.powf(-6.666666666666667e-1f64)))) + (((FFJ + FFS) * (FDM * (FFU.powf(-6.666666666666667e-1f64)))) * BP)) * DF) + Lanes([0.0, 0.0, (DG * FFV), 0.0])) * DA);
                        FFZ = FFX;
                        FGA = FFY;
                    } else {
                        FFZ = FEW;
                        FGA = FEX;
                    }
                    let FGC = if FGB > AI { 1.0 } else { 0.0 };
                    let FHB;
                    let FHC;
                    if FGC != 0.0 {
                        let FGD = FCI + EH;
                        let FGE = (DA * FBG).exp();
                        let FGF = FGE + IH;
                        let FGG = FE / EYR;
                        let FGH = FGG * FGG;
                        let FGI = (FF / EYR) * FGG;
                        let FGJ = FGI + FGI;
                        let FGK = FGH * FGF;
                        let FGL = DA * FGD;
                        let FGM = Lanes([0.0, 0.0, (DB * FGD), 0.0]) + FCK;
                        let FGN = FGK * FCN;
                        let FGO = FGM * FGL;
                        let FGP = FGN + (FGL * FGL);
                        let FGQ = Lanes([0.0, 0.0, ((((FGJ * FGF) + (((DB * FBG) * FGE) * FGH)) * FCN) + (FCO * FGK)), 0.0]);
                        let FGR = FGH * FCN;
                        let FGS = FGR.ln();
                        let FGT = Lanes([0.0, 0.0, (((FGJ * FCN) + (FCO * FGH)) * (DS / FGR)), 0.0]);
                        let FGU = DA * FAT;
                        let FGV = Lanes([0.0, 0.0, (DB * FAT), 0.0]);
                        let FGW = FGM - ((((FGQ + (FGO + FGO)) * (DS / FGP)) - FGT) + FGV);
                        let FGX = (FGL - (((FGP.ln()) - FGS) + FGU)) - BL;
                        let FGY = IZ * FGL;
                        let FGZ = FGM * IZ;
                        let FHA = if FGY > AI { 1.0 } else { 0.0 };
                        let FHM;
                        let FHN;
                        if FHA != 0.0 {
                            FHM = FGY;
                            FHN = FGZ;
                        } else {
                            let FHK = -FGY;
                            let FHL = FGZ * BP;
                            FHM = FHK;
                            FHN = FHL;
                        }
                        let FHO = FGW * FGX;
                        let FHP = ((FGX * FGX) + FHM).sqrt();
                        let FHQ = (FGL - (FGL - (KF * (FGX + FHP)))) + (DA * EH);
                        let FHR = ((FGM - (FGM - ((FGW + (((FHO + FHO) + FHN) * (DS / (ET * FHP)))) * KF))) + Lanes([0.0, 0.0, (DB * EH), 0.0])) * FHQ;
                        let FHS = FGN + (FHQ * FHQ);
                        let FHT = ((FHS.ln()) - FGS) + FGU;
                        let FHU = (((FGQ + (FHR + FHR)) * (DS / FHS)) - FGT) + FGV;
                        let FHV = FHU - FGA;
                        let FHW = (FHT - FFZ) - 6.0000000000000005e-2f64;
                        let FHY = (IZ * FHT) * FHX;
                        let FHZ = (FHU * IZ) * FHX;
                        let FIA = if FHY > AI { 1.0 } else { 0.0 };
                        let FID;
                        let FIE;
                        if FIA != 0.0 {
                            FID = FHY;
                            FIE = FHZ;
                        } else {
                            let FIB = -FHY;
                            let FIC = FHZ * BP;
                            FID = FIB;
                            FIE = FIC;
                        }
                        let FIF = FHV * FHW;
                        let FIG = ((FHW * FHW) + FID).sqrt();
                        let FIH = FHT - (KF * (FHW + FIG));
                        let FII = FHU - ((FHV + (((FIF + FIF) + FIE) * (DS / (ET * FIG)))) * KF);
                        FHB = FIH;
                        FHC = FII;
                    } else {
                        FHB = FFZ;
                        FHC = FGA;
                    }
                    let FHD = FHB / DA;
                    let FHE = (FHC - Lanes([0.0, 0.0, (DB * FHD), 0.0])) / DA;
                    let FHF = FHD - FAT;
                    let FHG = (-FHB).exp();
                    let FHH = (FHB - BL) + FHG;
                    let FHI = FHC + ((FHC * BP) * FHG);
                    let FHJ = if FHH < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let FIK;
                    let FIL;
                    if FHJ != 0.0 {
                        FIK = FIJ;
                        FIL = FEB;
                    } else {
                        FIK = FHH;
                        FIL = FHI;
                    }
                    let FIM = FIK.sqrt();
                    let FIN = EZE * FIM;
                    let FIO = Lanes([0.0, 0.0, (EZF * FIM), 0.0]) + ((FIL * (DS / (ET * FIM))) * EZE);
                    let FIP = LR * (FBB - FHF);
                    let FIQ = (FEJ - FHE) * LR;
                    let FIR = if FGB == BL { 1.0 } else { 0.0 };
                    let FJA;
                    let FJB;
                    let FJC;
                    let FJD;
                    let FJE;
                    let FJF;
                    let FJG;
                    let FJH;
                    let FJI;
                    let FJJ;
                    if FIR != 0.0 {
                        let FIS = (DA * FBG).exp();
                        let FIT = (DB * FBG) * FIS;
                        let FIU = FE / EYR;
                        let FIV = FIU * FIU;
                        let FIW = (FF / EYR) * FIU;
                        let FIX = FIW + FIW;
                        let FIY = FIV * FIS;
                        let FIZ = (FIX * FIS) + (FIT * FIV);
                        let mut FJK = 0.0;
                        let mut FJL = 0.0;
                        let mut FJM = 0.0;
                        let mut FJN = 0.0;
                        let mut FJO = 0.0;
                        let mut FJP = 0.0;
                        let mut FJQ = 0.0;
                        let mut FJR = Lanes([0.0; 4]);
                        let mut FJS = Lanes([0.0; 4]);
                        let mut FJT = Lanes([0.0; 4]);
                        let mut FJU = Lanes([0.0; 4]);
                        let mut FJV = Lanes([0.0; 4]);
                        FJK = BL;
                        FJL = FHF;
                        FJM = AI;
                        FJN = FHB;
                        FJO = AI;
                        FJP = AI;
                        FJQ = AI;
                        FJR = FHE;
                        FJS = FHC;
                        FJT = FEB;
                        FJU = FEB;
                        FJV = FEB;
                        loop {
                            let FJW = if FJK <= 4.1e1f64 { 1.0 } else { 0.0 };
                            if FJW == 0.0 {
                                break;
                            }
                            let FJX = FJL + FAT;
                            let FJY = DA * FJX;
                            let FJZ = FJR * DA;
                            let FKA = Lanes([0.0, 0.0, (DB * FJX), 0.0]) + FJZ;
                            let FKB = if FJY < CBJ { 1.0 } else { 0.0 };
                            let FLS;
                            let FLT;
                            let FLU;
                            let FLV;
                            let FLW;
                            let FLX;
                            let FLY;
                            let FLZ;
                            if FKB != 0.0 {
                                let FKD = FJY * FJY;
                                let FKE = FKA * FJY;
                                let FKF = FKE + FKE;
                                let FKG = FKD * FJY;
                                let FKI = -7.053654284009761e-2f64 + (FJY * FKH);
                                let FKK = FKJ + (FJY * FKI);
                                let FKL = FKG * FKK;
                                let FKM = (((FKF * FJY) + (FKA * FKD)) * FKK) + (((FKA * FKI) + ((FKA * FKH) * FJY)) * FKG);
                                let FKN = FJY * CBJ;
                                let FKO = FKA * CBJ;
                                let FKP = -2.8214617136039044e-1f64 + (FKN * FKH);
                                let FKQ = 8.907946456731299e-1f64 + (FJY * FKP);
                                let FKR = FKD * FKQ;
                                let FKS = FIY * FKL;
                                let FKT = FKS * FKL;
                                let FKU = ((Lanes([0.0, 0.0, (FIZ * FKL), 0.0]) + (FKM * FIY)) * FKL) + (FKM * FKS);
                                let FKV = (FIY * DA) * FB;
                                let FKW = FKV * FKL;
                                let FKY = -1.63730162779191e-3f64 + (FJY * FKX);
                                let FLA = FKZ + (FJY * FKY);
                                let FLB = -1.17851130197758e-1f64 + (FJY * FLA);
                                let FLD = FLC + (FJY * FLB);
                                let FLE = FJY * FLD;
                                let FLF = (FKA * FLD) + (((FKA * FLB) + (((FKA * FLA) + (((FKA * FKY) + ((FKA * FKX) * FJY)) * FJY)) * FJY)) * FJY);
                                let FLG = -6.54920651116764e-3f64 + (FKN * FKX);
                                let FLH = 5.3640151901649905e-2f64 + (FJY * FLG);
                                let FLI = -2.35702260395516e-1f64 + (FJY * FLH);
                                let FLJ = FLC + (FJY * FLI);
                                let FLK = FLF * FLE;
                                let FLL = (((FLE * FLE) + FKT) + IH).sqrt();
                                let FLM = ((FLK + FLK) + FKU) * (DS / (ET * FLL));
                                let FLN = (DA * FLJ) * FB;
                                let FLO = FLL + FLL;
                                let FLP = ((FLN * FLE) + (FKW * FKR)) / FLO;
                                let FLQ = ((((((Lanes([0.0, 0.0, (DB * FLJ), 0.0]) + (((FKA * FLI) + (((FKA * FLH) + (((FKA * FLG) + ((FKO * FKX) * FJY)) * FJY)) * FJY)) * DA)) * FB) * FLE) + (FLF * FLN)) + (((Lanes([0.0, 0.0, ((((FIZ * DA) + (DB * FIY)) * FB) * FKL), 0.0]) + (FKM * FKV)) * FKR) + (((FKF * FKQ) + (((FKA * FKP) + ((FKO * FKH) * FJY)) * FKD)) * FKW))) - ((FLM + FLM) * FLP)) / FLO;
                                FLS = FLL;
                                FLT = FLP;
                                FLU = FLE;
                                FLV = FKT;
                                FLW = FLM;
                                FLX = FLQ;
                                FLY = FLF;
                                FLZ = FKU;
                            } else {
                                let FLR = if FJY < BNR { 1.0 } else { 0.0 };
                                let FMV;
                                let FMW;
                                let FMX;
                                let FMY;
                                if FLR != 0.0 {
                                    let FMF = FJY.exp();
                                    let FMG = FKA * FMF;
                                    let FMH = FMF - BL;
                                    let FMI = FIY * FMH;
                                    let FMJ = Lanes([0.0, 0.0, (FIZ * FMH), 0.0]) + (FMG * FIY);
                                    let FMK = FIY * DA;
                                    let FML = FMK * FMF;
                                    let FMM = Lanes([0.0, 0.0, (((FIZ * DA) + (DB * FIY)) * FMF), 0.0]) + (FMG * FMK);
                                    FMV = FMI;
                                    FMW = FML;
                                    FMX = FMJ;
                                    FMY = FMM;
                                } else {
                                    let FMN = (DA * FJL).exp();
                                    let FMO = (Lanes([0.0, 0.0, (DB * FJL), 0.0]) + FJZ) * FMN;
                                    let FMP = FMN - FIS;
                                    let FMQ = FIV * FMP;
                                    let FMR = Lanes([0.0, 0.0, (FIX * FMP), 0.0]) + ((FMO - Lanes([0.0, 0.0, FIT, 0.0])) * FIV);
                                    let FMS = FIV * DA;
                                    let FMT = FMS * FMN;
                                    let FMU = Lanes([0.0, 0.0, (((FIX * DA) + (DB * FIV)) * FMN), 0.0]) + (FMO * FMS);
                                    FMV = FMQ;
                                    FMW = FMT;
                                    FMX = FMR;
                                    FMY = FMU;
                                }
                                let FMZ = ((FJY - BL) + FMV).sqrt();
                                let FNA = (FKA + FMX) * (DS / (ET * FMZ));
                                let FNB = (DA + FMW) / FMZ;
                                let FNC = FNB * KF;
                                let FND = (((Lanes([0.0, 0.0, DB, 0.0]) + FMY) - (FNA * FNB)) / FMZ) * KF;
                                FLS = FMZ;
                                FLT = FNC;
                                FLU = FJO;
                                FLV = FMV;
                                FLW = FNA;
                                FLX = FND;
                                FLY = FJT;
                                FLZ = FMX;
                            }
                            let FMA = (FBB - FJL) - (FAU * FLS);
                            let FMB = (FEJ - FJR) - (Lanes([0.0, 0.0, (FAV * FLS), 0.0]) + (FLW * FAU));
                            let FMC = -1e0f64 - (FAU * FLT);
                            let FMD = (Lanes([0.0, 0.0, (FAV * FLT), 0.0]) + (FLX * FAU)) * BP;
                            let FME = if FJM == BL { 1.0 } else { 0.0 };
                            let FNK;
                            let FNL;
                            let FNM;
                            let FNN;
                            if FME != 0.0 {
                                FNK = FNE;
                                FNL = FJL;
                                FNM = FJM;
                                FNN = FJR;
                            } else {
                                let FNF = (-FMA) / FMC;
                                let FNG = ((FMB * BP) - (FMD * FNF)) / FMC;
                                let FNH = FJL.abs();
                                let FNI = FJR * ((ET * (if FJL >= CAK { 1.0 } else { 0.0 })) - DS);
                                let FNJ = if BL >= FNH { 1.0 } else { 0.0 };
                                let FNP;
                                let FNQ;
                                if FNJ != 0.0 {
                                    FNP = BL;
                                    FNQ = FEB;
                                } else {
                                    FNP = FNH;
                                    FNQ = FNI;
                                }
                                let FNS = FNR * (BL + FNP);
                                let FNT = FNQ * FNR;
                                let FNU = if (FNF.abs()) > FNS { 1.0 } else { 0.0 };
                                let FNW;
                                let FNX;
                                if FNU != 0.0 {
                                    let FNV = if FNF >= AI { 1.0 } else { 0.0 };
                                    let FOC = if FNV != 0.0 {
                                        BL
                                    } else {
                                        FOB
                                    };
                                    let FOD = FNS * FOC;
                                    let FOE = FNT * FOC;
                                    FNW = FOD;
                                    FNX = FOE;
                                } else {
                                    FNW = FNF;
                                    FNX = FNG;
                                }
                                let FNY = FJL + FNW;
                                let FNZ = FJR + FNX;
                                let FOA = if (if (FNW.abs()) <= HZ { 1.0 } else { 0.0 }) != 0.0 && (if (FMA.abs()) <= WZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let FOF = if FOA != 0.0 {
                                    BL
                                } else {
                                    FJM
                                };
                                FNK = FJK;
                                FNL = FNY;
                                FNM = FOF;
                                FNN = FNZ;
                            }
                            let FNO = FNK + BL;
                            FJK = FNO;
                            FJL = FNL;
                            FJM = FNM;
                            FJN = FJY;
                            FJO = FLU;
                            FJP = FLS;
                            FJQ = FLV;
                            FJR = FNN;
                            FJS = FKA;
                            FJT = FLY;
                            FJU = FLW;
                            FJV = FLZ;
                        }
                        let FKC = if FJM == AI { 1.0 } else { 0.0 };
                        let FOG = if FJN < CBJ { 1.0 } else { 0.0 };
                        let FOK;
                        let FOL;
                        if FOG != 0.0 {
                            let FOH = if FJN < IW { 1.0 } else { 0.0 };
                            let FOT = FJO + 2.220446049250313e-15f64;
                            FOK = FOT;
                            FOL = FJT;
                        } else {
                            let FOI = (FJN - BL).sqrt();
                            let FOJ = FJS * (DS / (ET * FOI));
                            FOK = FOI;
                            FOL = FOJ;
                        }
                        let FOM = EZE * FOK;
                        let FON = Lanes([0.0, 0.0, (EZF * FOK), 0.0]) + (FOL * EZE);
                        let FOO = FJP + FOK;
                        let FOP = BL / FOO;
                        let FOQ = EZE * FJQ;
                        let FOR = FOM + (FOQ * FOP);
                        let FOS = FON + (((Lanes([0.0, 0.0, (EZF * FJQ), 0.0]) + (FJV * EZE)) * FOP) + (((((FJU + FOL) * FOP) * BP) / FOO) * FOQ));
                        FJA = FOR;
                        FJB = FOM;
                        FJC = FJO;
                        FJD = FJP;
                        FJE = FJQ;
                        FJF = FOS;
                        FJG = FON;
                        FJH = FJT;
                        FJI = FJU;
                        FJJ = FJV;
                    } else {
                        FJA = FIP;
                        FJB = FIN;
                        FJC = AI;
                        FJD = AI;
                        FJE = AI;
                        FJF = FIQ;
                        FJG = FIO;
                        FJH = FEB;
                        FJI = FEB;
                        FJJ = FEB;
                    }
                    FCT = FJA;
                    FCU = FJB;
                    FCV = FJC;
                    FCW = FJD;
                    FCX = FJE;
                    FCY = FJF;
                    FCZ = FJG;
                    FDA = FJH;
                    FDB = FJI;
                    FDC = FJJ;
                }
                let FDD = DNE * EYP;
                let FOY;
                let FOZ;
                let FPA;
                let FPB;
                if EZH != 0.0 {
                    let FOU = FDD * FCT;
                    let FOV = FCY * FDD;
                    let FOW = FDD * FCU;
                    let FOX = FCZ * FDD;
                    FOY = FOU;
                    FOZ = FOW;
                    FPA = FOV;
                    FPB = FOX;
                } else {
                    FOY = AI;
                    FOZ = AI;
                    FPA = FEB;
                    FPB = FEB;
                }
                let FPG;
                let FPH;
                let FPI;
                let FPJ;
                if EZI != 0.0 {
                    let FPC = FDD * FCT;
                    let FPD = FCY * FDD;
                    let FPE = FDD * FCU;
                    let FPF = FCZ * FDD;
                    FPG = FPC;
                    FPH = FPE;
                    FPI = FPD;
                    FPJ = FPF;
                } else {
                    FPG = AI;
                    FPH = AI;
                    FPI = FEB;
                    FPJ = FEB;
                }
                let FPL = (FPK * CD) + CE;
                let FPM = (FPK * CE) + CD;
                let FPP;
                let FPQ;
                if FPK != 0.0 {
                    let FPN = (CD * CF) + (CE * (CF - CG));
                    let FPO = (CM * CD) + ((CM - Lanes([CN[0], CN[1], 0.0])) * CE);
                    FPP = FPN;
                    FPQ = FPO;
                } else {
                    FPP = FAK;
                    FPQ = FAL;
                }
                let FPU;
                let FPV;
                if FPR != 0.0 {
                    let FPS = (CE * CF) + (CD * (CF - CG));
                    let FPT = (CM * CE) + ((CM - Lanes([CN[0], CN[1], 0.0])) * CD);
                    FPU = FPS;
                    FPV = FPT;
                } else {
                    FPU = FPP;
                    FPV = FPQ;
                }
                let FPX = if FPW > GR { 1.0 } else { 0.0 };
                let FQC = if FPX != 0.0 {
                    let FPY = GU - GR;
                    let FPZ = (FPW - GR) / FPY;
                    let FQA = FPZ * FPZ;
                    let FQB = GR + (FPY * (BL - (BL / ((((BL + FPZ) + FQA) + (FQA * FPZ)) + (FQA * FQA)))));
                    FQB
                } else {
                    FPW
                };
                let FQD = (-FQC) - HZ;
                let FQE = FPV * BP;
                let FQF = (-FPU) + FBA;
                let FQG = -FQD;
                let FQH = if FQF < FQG { 1.0 } else { 0.0 };
                let FRR;
                let FRS;
                let FRT;
                let FRU;
                if FQH != 0.0 {
                    let FQI = DA * EZE;
                    let FQJ = LR / FQI;
                    let FQK = ((((DB * EZE) + (EZF * DA)) * FQJ) * BP) / FQI;
                    let FQM = FQK * FQL;
                    let FQN = FB + (FQL * FQJ);
                    let FQO = FBO * FQN;
                    let FQP = FQO * FQN;
                    let FQQ = FQP * FQN;
                    let FQR = ((((FQM * FBO) * FQN) + (FQM * FQO)) * FQN) + (FQM * FQP);
                    let FQS = CV - FBE;
                    let FQT = CW - FBF;
                    let FQU = FQF + FQD;
                    let FQV = FQE * DA;
                    let FQW = FBX * FQJ;
                    let FQX = (DA * FQU) - FB;
                    let FQY = FQW * FQX;
                    let FQZ = Lanes([0.0, 0.0, ((FQK * FBX) * FQX), 0.0]) + ((Lanes([0.0, 0.0, (DB * FQU), 0.0]) + Lanes([FQV[0], FQV[1], 0.0, FQV[2]])) * FQW);
                    let FRA = 9.899494936611664e0f64 - FQY;
                    let FRB = FQZ * BP;
                    let FRC = FRA * FRA;
                    let FRD = FRB * FRA;
                    let FRE = FRD + FRD;
                    let FRF = if FQQ < (FRC * WZ) { 1.0 } else { 0.0 };
                    let FSB;
                    let FSC;
                    if FRF != 0.0 {
                        let FRV = (KF * FQQ) / FRA;
                        let FRW = ((-9.899494936611664e0f64 + FRA) + FRV) + FQY;
                        let FRX = (FRB + ((Lanes([0.0, 0.0, (FQR * KF), 0.0]) - (FRB * FRV)) / FRA)) + FQZ;
                        FSB = FRW;
                        FSC = FRX;
                    } else {
                        let FRY = (FQQ + FRC).sqrt();
                        let FRZ = (-9.899494936611664e0f64 + FRY) + FQY;
                        let FSA = ((Lanes([0.0, 0.0, FQR, 0.0]) + FRE) * (DS / (ET * FRY))) + FQZ;
                        FSB = FRZ;
                        FSC = FSA;
                    }
                    let FSD = FSB.powf(FDM);
                    let FSE = FSC * (FDM * (FSB.powf(-6.666666666666667e-1f64)));
                    let FSF = FDQ * FSD;
                    let FSG = (((-5.65685424949238e0f64 - (FDP * FQJ)) + (FB * FSD)) + (FSF * FSD)) / FSD;
                    let FSH = (((((Lanes([0.0, 0.0, ((FQK * FDP) * BP), 0.0]) + (FSE * FB)) + (((FSE * FDQ) * FSD) + (FSE * FSF))) - (FSE * FSG)) / FSD) * DF) + Lanes([0.0, 0.0, (DG * FSG), 0.0]);
                    let FSI = ((FSG * DF) - FQD) + FQD;
                    let FSJ = FSI / FQS;
                    let FSK = ((FSH - Lanes([0.0, 0.0, (FQT * FSJ), 0.0])) / FQS) * FSJ;
                    let FSL = (BL + (FSJ * FSJ)).sqrt();
                    let FSM = FSI / FSL;
                    let FSN = LR * (FQF - (FSM - FQD));
                    let FSO = (Lanes([FQE[0], FQE[1], 0.0, FQE[2]]) - ((FSH - (((FSK + FSK) * (DS / (ET * FSL))) * FSM)) / FSL)) * LR;
                    FRR = FSN;
                    FRS = FSN;
                    FRT = FSO;
                    FRU = FSO;
                } else {
                    let FRG = FQF + FQD;
                    let FRH = FQE * DA;
                    let FRI = Lanes([FRH[0], FRH[1], 0.0, FRH[2]]);
                    let FRJ = Lanes([0.0, 0.0, (DB * FRG), 0.0]) + FRI;
                    let FRK = (DA * FRG) - BL;
                    let FRL = FAW * DC;
                    let FRM = (FAY * DC) + (DE * FAW);
                    let FRN = (IZ * (FRK + 4.9787068367863944e-2f64)) / FRL;
                    let FRO = ((FRJ * IZ) - Lanes([0.0, 0.0, (FRM * FRN), 0.0])) / FRL;
                    let FRP = BL + FRN;
                    let FRQ = if FRP < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let FSQ;
                    let FSR;
                    if FRQ != 0.0 {
                        FSQ = FSP;
                        FSR = FEB;
                    } else {
                        FSQ = FRP;
                        FSR = FRO;
                    }
                    let FSS = (FAW * DA) / FB;
                    let FST = ((FAY * DA) + (DB * FAW)) / FB;
                    let FSU = FSQ.sqrt();
                    let FSV = BL - FSU;
                    let FSW = Lanes([FQE[0], FQE[1], 0.0, FQE[2]]);
                    let FSX = (FQF + (FSS * FSV)) + FQD;
                    let FSY = (-(DA * FSX)).exp();
                    let FSZ = (IZ * (FRK + FSY)) / FRL;
                    let FTA = (((FRJ + (((Lanes([0.0, 0.0, (DB * FSX), 0.0]) + ((FSW + (Lanes([0.0, 0.0, (FST * FSV), 0.0]) + (((FSR * (DS / (ET * FSU))) * BP) * FSS))) * DA)) * BP) * FSY)) * IZ) - Lanes([0.0, 0.0, (FRM * FSZ), 0.0])) / FRL;
                    let FTB = BL + FSZ;
                    let FTC = if FTB < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let FTE;
                    let FTF;
                    if FTC != 0.0 {
                        FTE = FTD;
                        FTF = FEB;
                    } else {
                        FTE = FTB;
                        FTF = FTA;
                    }
                    let FTG = FTE.sqrt();
                    let FTH = BL - FTG;
                    let FTI = (FQF + (FSS * FTH)) + FQD;
                    let FTJ = DA * FTI;
                    let FTK = Lanes([0.0, 0.0, (DB * FTI), 0.0]) + ((FSW + (Lanes([0.0, 0.0, (FST * FTH), 0.0]) + (((FTF * (DS / (ET * FTG))) * BP) * FSS))) * DA);
                    let FTL = if FTJ < IW { 1.0 } else { 0.0 };
                    let FUM;
                    let FUN;
                    if FTL != 0.0 {
                        let FTM = DA * FAU;
                        let FTN = BL / FTM;
                        let FTO = ((((DB * FAU) + (FAV * DA)) * FTN) * BP) / FTM;
                        let FTP = 7.071067811865476e-1f64 + FTN;
                        let FTQ = FQE * BP;
                        let FTR = (-FRG) / FAU;
                        let FTV = (-5.151950988020902e1f64 - ((FTS * FTP) / FTT)) + (FTR / FTU);
                        let FTW = Lanes([0.0, 0.0, (((FTO * FTS) / FTT) * BP), 0.0]) + (((Lanes([FTQ[0], FTQ[1], 0.0, FTQ[2]]) - Lanes([0.0, 0.0, (FAV * FTR), 0.0])) / FAU) / FTU);
                        let FTZ = ((FTX * FTP) - 1.0979672760764175e-2f64) / FTY;
                        let FUA = (FTO * FTX) / FTY;
                        let FUB = FTW * FTV;
                        let FUC = FTZ * FTZ;
                        let FUD = FUA * FTZ;
                        let FUE = ((FTV * FTV) + (FUC * FTZ)).sqrt();
                        let FUF = ((FUB + FUB) + Lanes([0.0, 0.0, (((FUD + FUD) * FTZ) + (FUA * FUC)), 0.0])) * (DS / (ET * FUE));
                        let FUG = (-FTV) + FUE;
                        let FUH = FTV + FUE;
                        let FUI = ((FUG.powf(FDM)) + (-(FUH.powf(FDM)))) - -3.7209791878387604e0f64;
                        let FUJ = ((FUI * DF) - FQD) + FQD;
                        let FUK = DA * FUJ;
                        let FUL = Lanes([0.0, 0.0, (DB * FUJ), 0.0]) + (((((((FTW * BP) + FUF) * (FDM * (FUG.powf(-6.666666666666667e-1f64)))) + (((FTW + FUF) * (FDM * (FUH.powf(-6.666666666666667e-1f64)))) * BP)) * DF) + Lanes([0.0, 0.0, (DG * FUI), 0.0])) * DA);
                        FUM = FUK;
                        FUN = FUL;
                    } else {
                        FUM = FTJ;
                        FUN = FTK;
                    }
                    let FUO = if FGB > AI { 1.0 } else { 0.0 };
                    let FVN;
                    let FVO;
                    if FUO != 0.0 {
                        let FUP = FRG + EH;
                        let FUQ = (DA * FQG).exp();
                        let FUR = FUQ + IH;
                        let FUS = FE / EYR;
                        let FUT = FUS * FUS;
                        let FUU = (FF / EYR) * FUS;
                        let FUV = FUU + FUU;
                        let FUW = FUT * FUR;
                        let FUX = DA * FUP;
                        let FUY = Lanes([0.0, 0.0, (DB * FUP), 0.0]) + FRI;
                        let FUZ = FUW * FRL;
                        let FVA = FUY * FUX;
                        let FVB = FUZ + (FUX * FUX);
                        let FVC = Lanes([0.0, 0.0, ((((FUV * FUR) + (((DB * FQG) * FUQ) * FUT)) * FRL) + (FRM * FUW)), 0.0]);
                        let FVD = FUT * FRL;
                        let FVE = FVD.ln();
                        let FVF = Lanes([0.0, 0.0, (((FUV * FRL) + (FRM * FUT)) * (DS / FVD)), 0.0]);
                        let FVG = DA * FQD;
                        let FVH = Lanes([0.0, 0.0, (DB * FQD), 0.0]);
                        let FVI = FUY - ((((FVC + (FVA + FVA)) * (DS / FVB)) - FVF) + FVH);
                        let FVJ = (FUX - (((FVB.ln()) - FVE) + FVG)) - BL;
                        let FVK = IZ * FUX;
                        let FVL = FUY * IZ;
                        let FVM = if FVK > AI { 1.0 } else { 0.0 };
                        let FVY;
                        let FVZ;
                        if FVM != 0.0 {
                            FVY = FVK;
                            FVZ = FVL;
                        } else {
                            let FVW = -FVK;
                            let FVX = FVL * BP;
                            FVY = FVW;
                            FVZ = FVX;
                        }
                        let FWA = FVI * FVJ;
                        let FWB = ((FVJ * FVJ) + FVY).sqrt();
                        let FWC = (FUX - (FUX - (KF * (FVJ + FWB)))) + (DA * EH);
                        let FWD = ((FUY - (FUY - ((FVI + (((FWA + FWA) + FVZ) * (DS / (ET * FWB)))) * KF))) + Lanes([0.0, 0.0, (DB * EH), 0.0])) * FWC;
                        let FWE = FUZ + (FWC * FWC);
                        let FWF = ((FWE.ln()) - FVE) + FVG;
                        let FWG = (((FVC + (FWD + FWD)) * (DS / FWE)) - FVF) + FVH;
                        let FWH = FWG - FUN;
                        let FWI = (FWF - FUM) - 6.0000000000000005e-2f64;
                        let FWK = (IZ * FWF) * FWJ;
                        let FWL = (FWG * IZ) * FWJ;
                        let FWM = if FWK > AI { 1.0 } else { 0.0 };
                        let FWP;
                        let FWQ;
                        if FWM != 0.0 {
                            FWP = FWK;
                            FWQ = FWL;
                        } else {
                            let FWN = -FWK;
                            let FWO = FWL * BP;
                            FWP = FWN;
                            FWQ = FWO;
                        }
                        let FWR = FWH * FWI;
                        let FWS = ((FWI * FWI) + FWP).sqrt();
                        let FWT = FWF - (KF * (FWI + FWS));
                        let FWU = FWG - ((FWH + (((FWR + FWR) + FWQ) * (DS / (ET * FWS)))) * KF);
                        FVN = FWT;
                        FVO = FWU;
                    } else {
                        FVN = FUM;
                        FVO = FUN;
                    }
                    let FVP = FVN / DA;
                    let FVQ = (FVO - Lanes([0.0, 0.0, (DB * FVP), 0.0])) / DA;
                    let FVR = FVP - FQD;
                    let FVS = (-FVN).exp();
                    let FVT = (FVN - BL) + FVS;
                    let FVU = FVO + ((FVO * BP) * FVS);
                    let FVV = if FVT < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let FWW;
                    let FWX;
                    if FVV != 0.0 {
                        FWW = FWV;
                        FWX = FEB;
                    } else {
                        FWW = FVT;
                        FWX = FVU;
                    }
                    let FWY = FWW.sqrt();
                    let FWZ = EZE * FWY;
                    let FXA = Lanes([0.0, 0.0, (EZF * FWY), 0.0]) + ((FWX * (DS / (ET * FWY))) * EZE);
                    let FXB = LR * (FQF - FVR);
                    let FXC = (FSW - FVQ) * LR;
                    let FXD = if FGB == BL { 1.0 } else { 0.0 };
                    let FXM;
                    let FXN;
                    let FXO;
                    let FXP;
                    if FXD != 0.0 {
                        let FXE = (DA * FQG).exp();
                        let FXF = (DB * FQG) * FXE;
                        let FXG = FE / EYR;
                        let FXH = FXG * FXG;
                        let FXI = (FF / EYR) * FXG;
                        let FXJ = FXI + FXI;
                        let FXK = FXH * FXE;
                        let FXL = (FXJ * FXE) + (FXF * FXH);
                        let mut FXQ = 0.0;
                        let mut FXR = 0.0;
                        let mut FXS = 0.0;
                        let mut FXT = 0.0;
                        let mut FXU = 0.0;
                        let mut FXV = 0.0;
                        let mut FXW = 0.0;
                        let mut FXX = Lanes([0.0; 4]);
                        let mut FXY = Lanes([0.0; 4]);
                        let mut FXZ = Lanes([0.0; 4]);
                        let mut FYA = Lanes([0.0; 4]);
                        let mut FYB = Lanes([0.0; 4]);
                        FXQ = BL;
                        FXR = FVR;
                        FXS = AI;
                        FXT = FVN;
                        FXU = FCV;
                        FXV = FCW;
                        FXW = FCX;
                        FXX = FVQ;
                        FXY = FVO;
                        FXZ = FDA;
                        FYA = FDB;
                        FYB = FDC;
                        loop {
                            let FYC = if FXQ <= 4.1e1f64 { 1.0 } else { 0.0 };
                            if FYC == 0.0 {
                                break;
                            }
                            let FYD = FXR + FQD;
                            let FYE = DA * FYD;
                            let FYF = FXX * DA;
                            let FYG = Lanes([0.0, 0.0, (DB * FYD), 0.0]) + FYF;
                            let FYH = if FYE < CBJ { 1.0 } else { 0.0 };
                            let FZT;
                            let FZU;
                            let FZV;
                            let FZW;
                            let FZX;
                            let FZY;
                            let FZZ;
                            let GAA;
                            if FYH != 0.0 {
                                let FYJ = FYE * FYE;
                                let FYK = FYG * FYE;
                                let FYL = FYK + FYK;
                                let FYM = FYJ * FYE;
                                let FYN = -7.053654284009761e-2f64 + (FYE * FKH);
                                let FYO = FKJ + (FYE * FYN);
                                let FYP = FYM * FYO;
                                let FYQ = (((FYL * FYE) + (FYG * FYJ)) * FYO) + (((FYG * FYN) + ((FYG * FKH) * FYE)) * FYM);
                                let FYR = FYE * CBJ;
                                let FYS = FYG * CBJ;
                                let FYT = -2.8214617136039044e-1f64 + (FYR * FKH);
                                let FYU = 8.907946456731299e-1f64 + (FYE * FYT);
                                let FYV = FYJ * FYU;
                                let FYW = FXK * FYP;
                                let FYX = FYW * FYP;
                                let FYY = ((Lanes([0.0, 0.0, (FXL * FYP), 0.0]) + (FYQ * FXK)) * FYP) + (FYQ * FYW);
                                let FYZ = (FXK * DA) * FB;
                                let FZA = FYZ * FYP;
                                let FZB = -1.63730162779191e-3f64 + (FYE * FKX);
                                let FZC = FKZ + (FYE * FZB);
                                let FZD = -1.17851130197758e-1f64 + (FYE * FZC);
                                let FZE = FLC + (FYE * FZD);
                                let FZF = FYE * FZE;
                                let FZG = (FYG * FZE) + (((FYG * FZD) + (((FYG * FZC) + (((FYG * FZB) + ((FYG * FKX) * FYE)) * FYE)) * FYE)) * FYE);
                                let FZH = -6.54920651116764e-3f64 + (FYR * FKX);
                                let FZI = 5.3640151901649905e-2f64 + (FYE * FZH);
                                let FZJ = -2.35702260395516e-1f64 + (FYE * FZI);
                                let FZK = FLC + (FYE * FZJ);
                                let FZL = FZG * FZF;
                                let FZM = (((FZF * FZF) + FYX) + IH).sqrt();
                                let FZN = ((FZL + FZL) + FYY) * (DS / (ET * FZM));
                                let FZO = (DA * FZK) * FB;
                                let FZP = FZM + FZM;
                                let FZQ = ((FZO * FZF) + (FZA * FYV)) / FZP;
                                let FZR = ((((((Lanes([0.0, 0.0, (DB * FZK), 0.0]) + (((FYG * FZJ) + (((FYG * FZI) + (((FYG * FZH) + ((FYS * FKX) * FYE)) * FYE)) * FYE)) * DA)) * FB) * FZF) + (FZG * FZO)) + (((Lanes([0.0, 0.0, ((((FXL * DA) + (DB * FXK)) * FB) * FYP), 0.0]) + (FYQ * FYZ)) * FYV) + (((FYL * FYU) + (((FYG * FYT) + ((FYS * FKH) * FYE)) * FYJ)) * FZA))) - ((FZN + FZN) * FZQ)) / FZP;
                                FZT = FZM;
                                FZU = FZQ;
                                FZV = FZF;
                                FZW = FYX;
                                FZX = FZN;
                                FZY = FZR;
                                FZZ = FZG;
                                GAA = FYY;
                            } else {
                                let FZS = if FYE < BNR { 1.0 } else { 0.0 };
                                let GAW;
                                let GAX;
                                let GAY;
                                let GAZ;
                                if FZS != 0.0 {
                                    let GAG = FYE.exp();
                                    let GAH = FYG * GAG;
                                    let GAI = GAG - BL;
                                    let GAJ = FXK * GAI;
                                    let GAK = Lanes([0.0, 0.0, (FXL * GAI), 0.0]) + (GAH * FXK);
                                    let GAL = FXK * DA;
                                    let GAM = GAL * GAG;
                                    let GAN = Lanes([0.0, 0.0, (((FXL * DA) + (DB * FXK)) * GAG), 0.0]) + (GAH * GAL);
                                    GAW = GAJ;
                                    GAX = GAM;
                                    GAY = GAK;
                                    GAZ = GAN;
                                } else {
                                    let GAO = (DA * FXR).exp();
                                    let GAP = (Lanes([0.0, 0.0, (DB * FXR), 0.0]) + FYF) * GAO;
                                    let GAQ = GAO - FXE;
                                    let GAR = FXH * GAQ;
                                    let GAS = Lanes([0.0, 0.0, (FXJ * GAQ), 0.0]) + ((GAP - Lanes([0.0, 0.0, FXF, 0.0])) * FXH);
                                    let GAT = FXH * DA;
                                    let GAU = GAT * GAO;
                                    let GAV = Lanes([0.0, 0.0, (((FXJ * DA) + (DB * FXH)) * GAO), 0.0]) + (GAP * GAT);
                                    GAW = GAR;
                                    GAX = GAU;
                                    GAY = GAS;
                                    GAZ = GAV;
                                }
                                let GBA = ((FYE - BL) + GAW).sqrt();
                                let GBB = (FYG + GAY) * (DS / (ET * GBA));
                                let GBC = (DA + GAX) / GBA;
                                let GBD = GBC * KF;
                                let GBE = (((Lanes([0.0, 0.0, DB, 0.0]) + GAZ) - (GBB * GBC)) / GBA) * KF;
                                FZT = GBA;
                                FZU = GBD;
                                FZV = FXU;
                                FZW = GAW;
                                FZX = GBB;
                                FZY = GBE;
                                FZZ = FXZ;
                                GAA = GAY;
                            }
                            let GAB = (FQF - FXR) - (FAU * FZT);
                            let GAC = (FSW - FXX) - (Lanes([0.0, 0.0, (FAV * FZT), 0.0]) + (FZX * FAU));
                            let GAD = -1e0f64 - (FAU * FZU);
                            let GAE = (Lanes([0.0, 0.0, (FAV * FZU), 0.0]) + (FZY * FAU)) * BP;
                            let GAF = if FXS == BL { 1.0 } else { 0.0 };
                            let GBL;
                            let GBM;
                            let GBN;
                            let GBO;
                            if GAF != 0.0 {
                                GBL = GBF;
                                GBM = FXR;
                                GBN = FXS;
                                GBO = FXX;
                            } else {
                                let GBG = (-GAB) / GAD;
                                let GBH = ((GAC * BP) - (GAE * GBG)) / GAD;
                                let GBI = FXR.abs();
                                let GBJ = FXX * ((ET * (if FXR >= CAK { 1.0 } else { 0.0 })) - DS);
                                let GBK = if BL >= GBI { 1.0 } else { 0.0 };
                                let GBQ;
                                let GBR;
                                if GBK != 0.0 {
                                    GBQ = BL;
                                    GBR = FEB;
                                } else {
                                    GBQ = GBI;
                                    GBR = GBJ;
                                }
                                let GBT = GBS * (BL + GBQ);
                                let GBU = GBR * GBS;
                                let GBV = if (GBG.abs()) > GBT { 1.0 } else { 0.0 };
                                let GBX;
                                let GBY;
                                if GBV != 0.0 {
                                    let GBW = if GBG >= AI { 1.0 } else { 0.0 };
                                    let GCD = if GBW != 0.0 {
                                        BL
                                    } else {
                                        GCC
                                    };
                                    let GCE = GBT * GCD;
                                    let GCF = GBU * GCD;
                                    GBX = GCE;
                                    GBY = GCF;
                                } else {
                                    GBX = GBG;
                                    GBY = GBH;
                                }
                                let GBZ = FXR + GBX;
                                let GCA = FXX + GBY;
                                let GCB = if (if (GBX.abs()) <= HZ { 1.0 } else { 0.0 }) != 0.0 && (if (GAB.abs()) <= WZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let GCG = if GCB != 0.0 {
                                    BL
                                } else {
                                    FXS
                                };
                                GBL = FXQ;
                                GBM = GBZ;
                                GBN = GCG;
                                GBO = GCA;
                            }
                            let GBP = GBL + BL;
                            FXQ = GBP;
                            FXR = GBM;
                            FXS = GBN;
                            FXT = FYE;
                            FXU = FZV;
                            FXV = FZT;
                            FXW = FZW;
                            FXX = GBO;
                            FXY = FYG;
                            FXZ = FZZ;
                            FYA = FZX;
                            FYB = GAA;
                        }
                        let FYI = if FXS == AI { 1.0 } else { 0.0 };
                        let GCH = if FXT < CBJ { 1.0 } else { 0.0 };
                        let GCL;
                        let GCM;
                        if GCH != 0.0 {
                            let GCI = if FXT < IW { 1.0 } else { 0.0 };
                            let GCU = FXU + 2.220446049250313e-15f64;
                            GCL = GCU;
                            GCM = FXZ;
                        } else {
                            let GCJ = (FXT - BL).sqrt();
                            let GCK = FXY * (DS / (ET * GCJ));
                            GCL = GCJ;
                            GCM = GCK;
                        }
                        let GCN = EZE * GCL;
                        let GCO = Lanes([0.0, 0.0, (EZF * GCL), 0.0]) + (GCM * EZE);
                        let GCP = FXV + GCL;
                        let GCQ = BL / GCP;
                        let GCR = EZE * FXW;
                        let GCS = GCN + (GCR * GCQ);
                        let GCT = GCO + (((Lanes([0.0, 0.0, (EZF * FXW), 0.0]) + (FYB * EZE)) * GCQ) + (((((FYA + GCM) * GCQ) * BP) / GCP) * GCR));
                        FXM = GCS;
                        FXN = GCN;
                        FXO = GCT;
                        FXP = GCO;
                    } else {
                        FXM = FXB;
                        FXN = FWZ;
                        FXO = FXC;
                        FXP = FXA;
                    }
                    FRR = FXM;
                    FRS = FXN;
                    FRT = FXO;
                    FRU = FXP;
                }
                let GCZ;
                let GDA;
                let GDB;
                let GDC;
                if FPL != 0.0 {
                    let GCV = FDD * FRR;
                    let GCW = FRT * FDD;
                    let GCX = FDD * FRS;
                    let GCY = FRU * FDD;
                    GCZ = GCV;
                    GDA = GCX;
                    GDB = GCW;
                    GDC = GCY;
                } else {
                    GCZ = FOY;
                    GDA = FOZ;
                    GDB = FPA;
                    GDC = FPB;
                }
                let GDH;
                let GDI;
                let GDJ;
                let GDK;
                if FPM != 0.0 {
                    let GDD = FDD * FRR;
                    let GDE = FRT * FDD;
                    let GDF = FDD * FRS;
                    let GDG = FRU * FDD;
                    GDH = GDD;
                    GDI = GDF;
                    GDJ = GDE;
                    GDK = GDG;
                } else {
                    GDH = FPG;
                    GDI = FPH;
                    GDJ = FPI;
                    GDK = FPJ;
                }
                let GDL = (CE * EZO) + (CD * EZL);
                let GDS;
                let GDT;
                if GDL != 0.0 {
                    let GDP = -(((CE * GDM) + (CD * GDN)) * GDO);
                    let GDQ = GDP * (CC - CB);
                    let GDR = (CL - Lanes([0.0, CK[0], CK[1]])) * GDP;
                    GDS = GDQ;
                    GDT = GDR;
                } else {
                    GDS = AI;
                    GDT = ENE;
                }
                let GDU = (CD * EZO) + (CE * EZL);
                let GDY;
                let GDZ;
                if GDU != 0.0 {
                    let GDV = -(((CD * GDM) + (CE * GDN)) * GDO);
                    let GDW = GDV * CC;
                    let GDX = CL * GDV;
                    GDY = GDW;
                    GDZ = GDX;
                } else {
                    GDY = AI;
                    GDZ = ENE;
                }
                EZR = GDS;
                EZS = GDY;
                EZT = GCZ;
                EZU = GDH;
                EZV = GDI;
                EZW = GDA;
                EZX = GDT;
                EZY = GDZ;
                EZZ = GDB;
                FAA = GDJ;
                FAB = GDK;
                FAC = GDC;
            } else {
                let EZK = if CH == BL { 1.0 } else { 0.0 };
                let EZM = if EZL == 0.0 { 1.0 } else { 0.0 };
                let EZN = if CH != BL { 1.0 } else { 0.0 };
                let EZP = if EZO == 0.0 { 1.0 } else { 0.0 };
                let EZQ = if (if EZK != 0.0 && EZM != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if EZN != 0.0 && EZP != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GEB;
                if EZQ != 0.0 {
                    let GEH = if EYQ != 0.0 {
                        let GEG = ((-LR) * EYP) * DNE;
                        GEG
                    } else {
                        AI
                    };
                    GEB = GEH;
                } else {
                    let GEA = ((CE * GDM) + (CD * GDN)) * GDO;
                    GEB = GEA;
                }
                let GEC = -GEB;
                let GED = GEC * (CC - CB);
                let GEE = (CL - Lanes([0.0, CK[0], CK[1]])) * GEC;
                let GEF = if (if EZK != 0.0 && EZP != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if EZN != 0.0 && EZM != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GEK = if GEF != 0.0 {
                    let GEI = ((-LR) * EYP) * DNE;
                    GEI
                } else {
                    let GEJ = ((CD * GDM) + (CE * GDN)) * GDO;
                    GEJ
                };
                let GEL = -GEK;
                let GEM = GEL * CC;
                let GEN = CL * GEL;
                EZR = GED;
                EZS = GEM;
                EZT = AI;
                EZU = AI;
                EZV = AI;
                EZW = AI;
                EZX = GEE;
                EZY = GEN;
                EZZ = FEB;
                FAA = FEB;
                FAB = FEB;
                FAC = FEB;
            }
            let GEO;
            let GEP;
            let GEQ;
            let GER;
            if AN != 0.0 {
                let GFD;
                let GFE;
                if DKO != 0.0 {
                    let GEW = GEU * GEV;
                    let GEX = GEW * DMZ;
                    let GEY = GEV * DMZ;
                    let GEZ = (((DTI * CDD) * GEU) + (GEY * DMZ)) + IH;
                    let GFA = (GEX * DMZ) / GEZ;
                    let GFB = ((((DNA * GEW) * DMZ) + (DNA * GEX)) - (((((DTJ * CDD) + (CDF * DTI)) * GEU) + (((DNA * GEV) * DMZ) + (DNA * GEY))) * GFA)) / GEZ;
                    GFD = GFA;
                    GFE = GFB;
                } else {
                    let GFC = GEU + IH;
                    GFD = GFC;
                    GFE = BCB;
                }
                let GFG = (GFF * MI) / DOC;
                let GFH = (ML * GFF) / DOC;
                GEO = GFD;
                GEP = GFG;
                GEQ = GFE;
                GER = GFH;
            } else {
                GEO = AI;
                GEP = AI;
                GEQ = BCB;
                GER = LY;
            }
            let GES = if DIK == 0.0 { 1.0 } else { 0.0 };
            let GET = if (if parameters[21] != AI { 1.0 } else { 0.0 }) != 0.0 && GES != 0.0 { 1.0 } else { 0.0 };
            if GET != 0.0 {
                let GFI = CCV / CZ;
                let GFJ = if (((((((-2e0f64 * DNC) / CZ) / DMZ) / DNE) - GFI) - GFI).abs()) > 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
            } else {
            }
            let GFK = if EDB != 0.0 && GES != 0.0 { 1.0 } else { 0.0 };
            let GFR;
            let GFS;
            let GFT;
            let GFU;
            if GFK != 0.0 {
                let GFL = (EYK - CCK) / DMZ;
                let GFN = (DQQ * GFL) / GFM;
                let GFO = ((DQR * GFL) + ((((EYM - CCO) - (DNA * GFL)) / DMZ) * DQQ)) / GFM;
                let GFQ = if (if 9.999999999999978e-1f64 <= GFP { 1.0 } else { 0.0 }) != 0.0 && (if GFP <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GFX;
                let GFY;
                if GFQ != 0.0 {
                    GFX = BL;
                    GFY = BCB;
                } else {
                    let GFW = if (if 1.9999999999999978e0f64 <= GFP { 1.0 } else { 0.0 }) != 0.0 && (if GFP <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GHK;
                    let GHL;
                    if GFW != 0.0 {
                        GHK = GFN;
                        GHL = GFO;
                    } else {
                        let GHH = GFP - BL;
                        let GHI = GFN.powf(GHH);
                        let GHJ = GFO * (GHH * (GFN.powf((GHH - DS))));
                        GHK = GHI;
                        GHL = GHJ;
                    }
                    GFX = GHK;
                    GFY = GHL;
                }
                let GFZ = (GFO * GFX) + (GFY * GFN);
                let GGA = BL + (GFN * GFX);
                let GGB = (-1e0f64 / GFP) - BL;
                let GGC = GGA.powf(GGB);
                let GGD = DQQ * GGA;
                let GGE = GGD * GGC;
                let GGF = (((DQR * GGA) + (GFZ * DQQ)) * GGC) + ((GFZ * (GGB * (GGA.powf((GGB - DS))))) * GGD);
                let GGG = (DTI + GGE) / FB;
                let GGH = (DTJ + GGF) / FB;
                let GGI = DKG * DKG;
                let GGJ = DKH * DKG;
                let GGK = GGJ + GGJ;
                let GGL = BFD * MI;
                let GGM = GGL * CDD;
                let GGN = (ML * BFD) * CDD;
                let GGO = GGM * DTI;
                let GGP = IW * DKG;
                let GGQ = DKH * IW;
                let GGS = (BL + GGP) + (GGR * GGI);
                let GGT = GGS * GGE;
                let GGU = (IW + (IZ * DKG)) + (IW * GGI);
                let GGV = GGU * GGE;
                let GGW = (GGR + GGP) + GGI;
                let GGX = GGW * DTI;
                let GGY = ((GGT * GGE) + (GGV * DTI)) + (GGX * DTI);
                let GHA = GGZ * DMZ;
                let GHB = BL + DKG;
                let GHC = GHA * GHB;
                let GHD = GHC * GGG;
                let GHE = GHD * GGG;
                let GHF = (GGO * GGY) / GHE;
                let GHG = ((((((Lanes([0.0, GGN[0], GGN[1], 0.0, GGN[2], GGN[3]]) + (CDF * GGL)) * DTI) + (DTJ * GGM)) * GGY) + ((((((((GGQ + (GGK * GGR)) * GGE) + (GGF * GGS)) * GGE) + (GGF * GGT)) + ((((((DKH * IZ) + (GGK * IW)) * GGE) + (GGF * GGU)) * DTI) + (DTJ * GGV))) + (((((GGQ + GGK) * DTI) + (DTJ * GGW)) * DTI) + (DTJ * GGX))) * GGO)) - ((((((((DNA * GGZ) * GHB) + (DKH * GHA)) * GGG) + (GGH * GHC)) * GGG) + (GGH * GHD)) * GHF)) / GHE;
                GFR = GHF;
                GFS = GGE;
                GFT = GHG;
                GFU = GGF;
            } else {
                GFR = AI;
                GFS = AI;
                GFT = BCB;
                GFU = BCB;
            }
            let GFV = if (if EDC != 0.0 && (if EIP == BL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && GES != 0.0 { 1.0 } else { 0.0 };
            let GIY;
            let GIZ;
            let GJA;
            let GJB;
            let GJC;
            let GJD;
            let GJE;
            let GJF;
            if GFV != 0.0 {
                let GHM = EIQ.sqrt();
                let GHN = EIT * (DS / (ET * GHM));
                let GHO = CDD + GHM;
                let GHP = CDF + GHN;
                let GHQ = EIU * EIR;
                let GHR = EIT * EIQ;
                let GHT = GHS * EIR;
                let GHU = CBD * GHM;
                let GHV = GHU * CDD;
                let GHW = EIR + EIQ;
                let GHX = ((GHT * EIQ) + (IZ * ((EIR * EIR) + (EIQ * EIQ)))) + (GHV * GHW);
                let GHY = ((((EIU * GHS) * EIQ) + (EIT * GHT)) + (((GHQ + GHQ) + (GHR + GHR)) * IZ)) + (((((GHN * CBD) * CDD) + (CDF * GHU)) * GHW) + ((EIU + EIT) * GHV));
                let GHZ = GHO * GHO;
                let GIA = GHP * GHO;
                let GIB = GHZ * GHZ;
                let GIC = (GIA + GIA) * GHZ;
                let GID = GIB * GHO;
                let GIE = GHX / GID;
                let GIF = (GHY - ((((GIC + GIC) * GHO) + (GHP * GIB)) * GIE)) / GID;
                let GIG = BFD / DMZ;
                let GIH = GIG * DTI;
                let GII = GIH * MI;
                let GIJ = ML * GIH;
                let GIK = ((((((DNA * GIG) * BP) / DMZ) * DTI) + (DTJ * GIG)) * MI) + Lanes([0.0, GIJ[0], GIJ[1], 0.0, GIJ[2], GIJ[3]]);
                let GIL = GII * CDD;
                let GIM = GFR / GIL;
                let GIN = IZ * CDD;
                let GIO = (EIR + (GIN * GHM)) + EIQ;
                let GIQ = GIP * EIS;
                let GIR = GGR * GHO;
                let GIS = GIM * GHO;
                let GIT = GIS * CDD;
                let GIU = (GIT * GHX).sqrt();
                let GIV = GIR * GIU;
                let GIW = (GIQ * GIO) / GIV;
                let GIX = ((((EIV * GIP) * GIO) + (((EIU + (((CDF * IZ) * GHM) + (GHN * GIN))) + EIT) * GIQ)) - ((((GHP * GGR) * GIU) + ((((((((((GFT - (((GIK * CDD) + (CDF * GII)) * GIM)) / GIL) * GHO) + (GHP * GIM)) * CDD) + (CDF * GIS)) * GHX) + (GHY * GIT)) * (DS / (ET * GIU))) * GIR)) * GIW)) / GIV;
                GIY = GII;
                GIZ = GHM;
                GJA = GIE;
                GJB = GIW;
                GJC = GIK;
                GJD = GHN;
                GJE = GIF;
                GJF = GIX;
            } else {
                GIY = HZ;
                GIZ = AI;
                GJA = AI;
                GJB = AI;
                GJC = BCB;
                GJD = BCB;
                GJE = BCB;
                GJF = BCB;
            }
            let GJG = ECZ + ELM;
            let GJH = EDA + ELN;
            let GJN;
            let GJO;
            if GJI != 0.0 {
                let GJJ = (-parameters[172]) * parameters[0];
                let GJK = GJJ * (CF - CI);
                let GJL = (Lanes([CM[0], CM[1], CM[2], 0.0]) - Lanes([CO[0], CO[1], 0.0, CO[2]])) * GJJ;
                GJN = GJK;
                GJO = GJL;
            } else {
                GJN = AI;
                GJO = GJM;
            }
            let GJP = (2.1983327444149834e-11f64 * DNE) * ((BL + (parameters[171] / NJ)).ln());
            let GJQ = (CM - Lanes([CN[0], CN[1], 0.0])) * GJP;
            let GJR = CM * GJP;
            let GJS = EZR + (GJP * (CF - CG));
            let GJT = Lanes([0.0, 0.0, EZX[0], EZX[1], EZX[2]]) + Lanes([GJQ[0], GJQ[1], GJQ[2], 0.0, 0.0]);
            let GJU = EZS + (GJP * CF);
            let GJV = Lanes([0.0, 0.0, EZY[0], EZY[1], EZY[2]]) + Lanes([GJR[0], GJR[1], GJR[2], 0.0, 0.0]);
            let GJY = DNB * (EH * YF);
            let GJZ = GJY * CI;
            let GKA = CO * GJY;
            let GKB = GJY * (CI - CG);
            let GKC = (CO - Lanes([CN[0], CN[1], 0.0])) * GJY;
            let GKD = DNB * (GJW * DJI);
            let GKE = (DJJ * GJW) * DNB;
            let GKF = DNB * (GJX * (BME + CSF));
            let GKG = ((BMN + CSO) * GJX) * DNB;
            let GKK;
            let GKL;
            let GKM;
            let GKN;
            if GKH != 0.0 {
                GKK = DNC;
                GKL = AI;
                GKM = DND;
                GKN = BCB;
            } else {
                let GKI = (DNC + GKD) + GKF;
                let GKJ = (DND + GKE) + GKG;
                GKK = GKI;
                GKL = DNF;
                GKM = GKJ;
                GKN = DNG;
            }
            let GKO = GKK * KF;
            let GKP = GKM * KF;
            let GKY;
            let GKZ;
            let GLA;
            let GLB;
            let GLC;
            let GLD;
            let GLE;
            let GLF;
            let GLG;
            let GLH;
            if AN != 0.0 {
                GKY = AI;
                GKZ = AI;
                GLA = AI;
                GLB = GKK;
                GLC = GKL;
                GLD = BCB;
                GLE = GKQ;
                GLF = GKQ;
                GLG = GKM;
                GLH = GKN;
            } else {
                let GKR = (-GKL) - GKK;
                let GKS = (GKN * BP) - GKM;
                let GKT = GKO + GKB;
                let GKU = Lanes([0.0, 0.0, GKP[0], GKP[1], GKP[2], GKP[3], GKP[4], GKP[5]]) + Lanes([GKC[0], GKC[1], 0.0, 0.0, GKC[2], 0.0, 0.0, 0.0]);
                let GKV = GKM - GKP;
                let GKW = (GKK - GKO) + GJZ;
                let GKX = Lanes([0.0, 0.0, GKV[0], GKV[1], GKV[2], GKV[3], GKV[4], GKV[5]]) + Lanes([GKA[0], GKA[1], 0.0, 0.0, GKA[2], 0.0, 0.0, 0.0]);
                GKY = GKR;
                GKZ = GKT;
                GLA = GKW;
                GLB = AI;
                GLC = AI;
                GLD = GKS;
                GLE = GKU;
                GLF = GKX;
                GLG = BCB;
                GLH = BCB;
            }
            let GLI = if EYI == AI { 1.0 } else { 0.0 };
            let GLM;
            let GLN;
            if GLI != 0.0 {
                GLM = AI;
                GLN = BCB;
            } else {
                let GLJ = (EYL * BDJ) + CCK;
                let GLK = (EYN * BDJ) + CCO;
                let GLL = if GLJ > EYK { 1.0 } else { 0.0 };
                let GLP;
                let GLQ;
                if GLL != 0.0 {
                    GLP = EYK;
                    GLQ = EYM;
                } else {
                    GLP = GLJ;
                    GLQ = GLK;
                }
                let GLR = CB + CCK;
                let GLS = DNI + CCO;
                let GLT = BL - DLF;
                let GLU = (JS * DNE) * (((2.069886e-10f64 / JR).sqrt()) * 1.3e0f64);
                let GLV = (((GLR - ((DLF * GLR) + (GLT * GLP))) / EYI) - EYL) * GLU;
                let GLW = (((GLS - ((GLS * DLF) + (GLQ * GLT))) / EYI) - EYN) * GLU;
                GLM = GLV;
                GLN = GLW;
            }
            let GLO = if parameters[46] != AI { 1.0 } else { 0.0 };
            let GMB;
            let GMC;
            if GLO != 0.0 {
                let GLY = HK * GLX;
                let GLZ = GLM + (GLX * HI);
                let GMA = GLN + Lanes([0.0, 0.0, GLY[0], 0.0, GLY[1], GLY[2]]);
                GMB = GLZ;
                GMC = GMA;
            } else {
                GMB = GLM;
                GMC = GLN;
            }
            let GMD = if parameters[14] == BL { 1.0 } else { 0.0 };
            let GMR;
            let GMS;
            let GMT;
            let GMU;
            let GMV;
            let GMW;
            if GMD != 0.0 {
                let GME = GJT + GJV;
                let GMF = Lanes([GME[0], GME[1], GME[2], 0.0, GME[3], GME[4]]) - Lanes([GJO[0], GJO[1], GJO[2], GJO[3], 0.0, 0.0]);
                let GMG = Lanes([0.0, 0.0, GMC[0], GMC[1], GMC[2], GMC[3], GMC[4], GMC[5]]);
                let GMH = GKY + (((((GJS + GJU) - GJN) - GMB) - EZT) - EZU);
                let GMI = Lanes([0.0, 0.0, GLD[0], GLD[1], GLD[2], GLD[3], GLD[4], GLD[5]]) + (((Lanes([GMF[0], GMF[1], 0.0, GMF[2], GMF[3], 0.0, GMF[4], GMF[5]]) - GMG) - Lanes([EZZ[0], EZZ[1], EZZ[2], EZZ[3], 0.0, 0.0, 0.0, 0.0])) - Lanes([FAA[0], FAA[1], FAA[2], FAA[3], 0.0, 0.0, 0.0, 0.0]));
                let GMJ = GJT * BP;
                let GMK = GKZ + (((-GJS) + GMB) + EZV);
                let GML = GLE + ((Lanes([GMJ[0], GMJ[1], 0.0, GMJ[2], 0.0, 0.0, GMJ[3], GMJ[4]]) + GMG) + Lanes([FAB[0], FAB[1], FAB[2], FAB[3], 0.0, 0.0, 0.0, 0.0]));
                let GMM = GJV * BP;
                let GMN = Lanes([GMM[0], GMM[1], 0.0, GMM[2], GMM[3], GMM[4]]) + Lanes([FAC[0], FAC[1], FAC[2], FAC[3], 0.0, 0.0]);
                let GMO = GLA + ((-GJU) + EZW);
                let GMP = GLF + Lanes([GMN[0], GMN[1], GMN[2], GMN[3], 0.0, 0.0, GMN[4], GMN[5]]);
                GMR = GMH;
                GMS = GMK;
                GMT = GMO;
                GMU = GMI;
                GMV = GML;
                GMW = GMP;
            } else {
                let GMQ = Lanes([0.0, 0.0, GLD[0], GLD[1], GLD[2], GLD[3], GLD[4], GLD[5]]);
                GMR = GKY;
                GMS = GKZ;
                GMT = GLA;
                GMU = GMQ;
                GMV = GLE;
                GMW = GLF;
            }
            let GMX = -ENF;
            let GMY = ENK * BP;
            let GMZ = if CH == BL { 1.0 } else { 0.0 };
            let GNF;
            let GNG;
            if GMZ != 0.0 {
                let GNA = (ENG * ENH) - ENI;
                let GNB = (ENL * ENG) - Lanes([0.0, ENM[0], 0.0, 0.0, ENM[1], ENM[2]]);
                GNF = GNA;
                GNG = GNB;
            } else {
                let GNC = BL - ENG;
                let GND = (GNC * ENH) - ENJ;
                let GNE = (ENL * GNC) - Lanes([0.0, ENN[0], 0.0, 0.0, ENN[1], ENN[2]]);
                GNF = GND;
                GNG = GNE;
            }
            let GNM;
            let GNN;
            if GMZ != 0.0 {
                let GNH = BL - ENG;
                let GNI = (GNH * ENH) - ENJ;
                let GNJ = (ENL * GNH) - Lanes([0.0, ENN[0], 0.0, 0.0, ENN[1], ENN[2]]);
                GNM = GNI;
                GNN = GNJ;
            } else {
                let GNK = (ENG * ENH) - ENI;
                let GNL = (ENL * ENG) - Lanes([0.0, ENM[0], 0.0, 0.0, ENM[1], ENM[2]]);
                GNM = GNK;
                GNN = GNL;
            }
            let GNO;
            let GNP;
            if GMZ != 0.0 {
                GNO = EVP;
                GNP = EVQ;
            } else {
                GNO = EXB;
                GNP = EXC;
            }
            let GNQ;
            let GNR;
            if GMZ != 0.0 {
                GNQ = EXB;
                GNR = EXC;
            } else {
                GNQ = EVP;
                GNR = EVQ;
            }
            let GNT = GNS * CP;
            let GNU = AM * GNS;
            let GNV = L * GMU[6];
            let GNW = L * GMU[7];
            let GNX = if CH > AI { 1.0 } else { 0.0 };
            let GNY = if GNX != 0.0 {
                GNW
            } else {
                GNV
            };
            let GOE;
            let GOF;
            let GOG;
            let GOH;
            if GFV != 0.0 {
                let GNZ = ((1e-6f64 * MI) * DNE) * BDJ;
                let GOB = (((GOA * DF) * GNY) * GNY) / GIY;
                let GOC = (Lanes([(((DG * GOA) * GNY) * GNY), 0.0, 0.0, 0.0, 0.0, 0.0]) - (GJC * GOB)) / GIY;
                let GOD = if (if EIS > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if CB > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GOZ;
                let GPA;
                if GOD != 0.0 {
                    let GOL = DQQ / DTI;
                    let GOM = (DQR - (DTJ * GOL)) / DTI;
                    let GON = DQQ / GFS;
                    let GOO = (GON - GOL) / CB;
                    let GOP = CK * GOO;
                    let GOR = GOQ * GOO;
                    let GOS = (EIR + (CDD * GIZ)) + EIQ;
                    let GOT = CDD + GIZ;
                    let GOU = (GOR * GOS) / GOT;
                    let GOV = GOL + GOU;
                    let GOW = GOM + ((((((((((DQR - (GFU * GON)) / GFS) - GOM) - Lanes([0.0, 0.0, 0.0, 0.0, GOP[0], GOP[1]])) / CB) * GOQ) * GOS) + (((EIU + ((CDF * GIZ) + (GJD * CDD))) + EIT) * GOR)) - ((CDF + GJD) * GOU)) / GOT);
                    GOZ = GOV;
                    GPA = GOW;
                } else {
                    let GOX = DQQ / GFS;
                    let GOY = (DQR - (GFU * GOX)) / GFS;
                    GOZ = GOX;
                    GPA = GOY;
                }
                let GPB = GOB * GJA;
                let GPC = GPB * GOZ;
                let GPD = (((GOC * GJA) + (GJE * GOB)) * GOZ) + (GPA * GPB);
                let GPE = if GPC < AI { 1.0 } else { 0.0 };
                let GPF;
                let GPG;
                if GPE != 0.0 {
                    GPF = AI;
                    GPG = BCB;
                } else {
                    GPF = GPC;
                    GPG = GPD;
                }
                let GPH = if (-GNY) > GNZ { 1.0 } else { 0.0 };
                let GPI;
                let GPJ;
                if GPH != 0.0 {
                    GPI = GPF;
                    GPJ = GPG;
                } else {
                    GPI = AI;
                    GPJ = BCB;
                }
                let GPK;
                let GPL;
                if GPH != 0.0 {
                    GPK = GJB;
                    GPL = GJF;
                } else {
                    GPK = AI;
                    GPL = BCB;
                }
                GOE = GPK;
                GOF = GPI;
                GOG = GPL;
                GOH = GPJ;
            } else {
                GOE = AI;
                GOF = AI;
                GOG = BCB;
                GOH = BCB;
            }
            let GOI = GNT * GFR;
            let GOJ = Lanes([(GNU * GFR), 0.0, 0.0, 0.0, 0.0, 0.0]) + (GFT * GNT);
            let GOK = if (if GOI > AI { 1.0 } else { 0.0 }) != 0.0 && (if GOF > AI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GPP;
            let GPQ;
            if GOK != 0.0 {
                let GPM = GOF / GOI;
                let GPN = GPM.sqrt();
                let GPO = ((GOH - (GOJ * GPM)) / GOI) * (DS / (ET * GPN));
                GPP = GPN;
                GPQ = GPO;
            } else {
                GPP = AI;
                GPQ = BCB;
            }
            let GPW;
            let GPX;
            if GNX != 0.0 {
                let GPS = GPP * GPR;
                let GPT = GPQ * GPR;
                GPW = GPS;
                GPX = GPT;
            } else {
                let GPU = GPP * KF;
                let GPV = GPQ * KF;
                GPW = GPU;
                GPX = GPV;
            }
            let GQD;
            let GQE;
            if GNX != 0.0 {
                let GPY = GPP * KF;
                let GPZ = GPQ * KF;
                GQD = GPY;
                GQE = GPZ;
            } else {
                let GQB = GPP * GQA;
                let GQC = GPQ * GQA;
                GQD = GQB;
                GQE = GQC;
            }
            let GQH;
            let GQI;
            if GQF != 0.0 {
                let GQK = L * (I - W);
                let GQM = DH.powf(GQL);
                let GQN = staged[135] / GQM;
                let GQQ = ((GQO + EE) + EJ) - (GQP * EL);
                let GQR = staged[136] / GQQ;
                let GQT = AM * GQS;
                let GQU = parameters[324] + (GQS * CS);
                let GQW = GQN * GQV;
                let GQX = ((((DI * (GQL * (DH.powf(staged[169])))) * GQN) * BP) / GQM) * GQV;
                let GRA = ((((((EF + EK) - (EM * GQP)) * GQR) * BP) / GQQ) * GQY) * GQZ;
                let GRB = ((GQR * GQY) * GQZ) + IH;
                let GRD = GQK / GRC;
                let GRE = GQW * GRD;
                let GRF = (((Lanes([0.0, K]) - Lanes([X, 0.0])) * L) / GRC) * GQW;
                let GRG = Lanes([0.0, (GQX * GRD), 0.0]) + Lanes([GRF[0], 0.0, GRF[1]]);
                let GRH = if GQK >= AI { 1.0 } else { 0.0 };
                let GRM;
                let GRN;
                if GRH != 0.0 {
                    let GRI = GRE / GRB;
                    let GRJ = (GRG - Lanes([0.0, (GRA * GRI), 0.0])) / GRB;
                    GRM = GRI;
                    GRN = GRJ;
                } else {
                    let GRK = (-GRE) / GRB;
                    let GRL = ((GRG * BP) - Lanes([0.0, (GRA * GRK), 0.0])) / GRB;
                    GRM = GRK;
                    GRN = GRL;
                }
                let GRO = if (if 9.999999999999978e-1f64 <= GQU { 1.0 } else { 0.0 }) != 0.0 && (if GQU <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GRQ;
                let GRR;
                if GRO != 0.0 {
                    GRQ = BL;
                    GRR = GQG;
                } else {
                    let GRP = if (if 1.9999999999999978e0f64 <= GQU { 1.0 } else { 0.0 }) != 0.0 && (if GQU <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GRY;
                    let GRZ;
                    if GRP != 0.0 {
                        GRY = GRM;
                        GRZ = GRN;
                    } else {
                        let GRV = GQU - BL;
                        let GRW = GRM.powf(GRV);
                        let GRX = (GRN * (GRV * (GRM.powf((GRV - DS))))) + Lanes([0.0, (GQT * (GRW * (GRM.ln()))), 0.0]);
                        GRY = GRW;
                        GRZ = GRX;
                    }
                    GRQ = GRY;
                    GRR = GRZ;
                }
                let GRS = (GRN * GRQ) + (GRR * GRM);
                let GRT = BL + (GRM * GRQ);
                let GRU = if (if 9.999999999999978e-1f64 <= GQU { 1.0 } else { 0.0 }) != 0.0 && (if GQU <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GSD;
                let GSE;
                if GRU != 0.0 {
                    let GSA = BL / GRT;
                    let GSB = ((GRS * GSA) * BP) / GRT;
                    GSD = GSA;
                    GSE = GSB;
                } else {
                    let GSC = if (if 1.9999999999999978e0f64 <= GQU { 1.0 } else { 0.0 }) != 0.0 && (if GQU <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GSS;
                    let GST;
                    if GSC != 0.0 {
                        let GSK = GRT.sqrt();
                        let GSL = BL / GSK;
                        let GSM = (((GRS * (DS / (ET * GSK))) * GSL) * BP) / GSK;
                        GSS = GSL;
                        GST = GSM;
                    } else {
                        let GSN = -1e0f64 / GQU;
                        let GSO = GSN - BL;
                        let GSP = GRT.powf(GSO);
                        let GSQ = GRT * GSP;
                        let GSR = (GRS * GSP) + (((GRS * (GSO * (GRT.powf((GSO - DS))))) + Lanes([0.0, ((((GQT * GSN) * BP) / GQU) * (GSP * (GRT.ln()))), 0.0])) * GRT);
                        GSS = GSQ;
                        GST = GSR;
                    }
                    GSD = GSS;
                    GSE = GST;
                }
                let GSH = (GSF * (GQW * GSD)) * GSG;
                let GSI = ((Lanes([0.0, (GQX * GSD), 0.0]) + (GSE * GQW)) * GSF) * GSG;
                let GSJ = if GSH <= AI { 1.0 } else { 0.0 };
                let GSU;
                let GSV;
                if GSJ != 0.0 {
                    GSU = IH;
                    GSV = GQG;
                } else {
                    GSU = GSH;
                    GSV = GSI;
                }
                let GSW = BL / GSU;
                let GSX = (((GSV * GSW) * BP) / GSU) / BFD;
                let GSY = (GSW / BFD) + staged[143];
                let GSZ = if (if GSY > MZ { 1.0 } else { 0.0 }) != 0.0 && EDB != 0.0 { 1.0 } else { 0.0 };
                let GTA = if GSY < MZ { 1.0 } else { 0.0 };
                let GTB;
                let GTC;
                if GTA != 0.0 {
                    GTB = MZ;
                    GTC = GQG;
                } else {
                    GTB = GSY;
                    GTC = GSX;
                }
                GQH = GTB;
                GQI = GTC;
            } else {
                GQH = AI;
                GQI = GQG;
            }
            let GTE;
            let GTF;
            if GQJ != 0.0 {
                let GTG = L * (AA - O);
                let GTH = DH.powf(GQL);
                let GTI = staged[144] / GTH;
                let GTJ = ((GQO + EE) + EJ) - (GQP * EL);
                let GTK = staged[145] / GTJ;
                let GTL = AM * GQS;
                let GTM = parameters[323] + (GQS * CS);
                let GTO = GTI * GTN;
                let GTP = ((((DI * (GQL * (DH.powf(staged[170])))) * GTI) * BP) / GTH) * GTN;
                let GTS = ((((((EF + EK) - (EM * GQP)) * GTK) * BP) / GTJ) * GTQ) * GTR;
                let GTT = ((GTK * GTQ) * GTR) + IH;
                let GTV = GTG / GTU;
                let GTW = GTO * GTV;
                let GTX = (((Lanes([AB, 0.0]) - Lanes([0.0, P])) * L) / GTU) * GTO;
                let GTY = Lanes([0.0, (GTP * GTV), 0.0]) + Lanes([GTX[0], 0.0, GTX[1]]);
                let GTZ = if GTG >= AI { 1.0 } else { 0.0 };
                let GUE;
                let GUF;
                if GTZ != 0.0 {
                    let GUA = GTW / GTT;
                    let GUB = (GTY - Lanes([0.0, (GTS * GUA), 0.0])) / GTT;
                    GUE = GUA;
                    GUF = GUB;
                } else {
                    let GUC = (-GTW) / GTT;
                    let GUD = ((GTY * BP) - Lanes([0.0, (GTS * GUC), 0.0])) / GTT;
                    GUE = GUC;
                    GUF = GUD;
                }
                let GUG = if (if 9.999999999999978e-1f64 <= GTM { 1.0 } else { 0.0 }) != 0.0 && (if GTM <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GUI;
                let GUJ;
                if GUG != 0.0 {
                    GUI = BL;
                    GUJ = GTD;
                } else {
                    let GUH = if (if 1.9999999999999978e0f64 <= GTM { 1.0 } else { 0.0 }) != 0.0 && (if GTM <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GUQ;
                    let GUR;
                    if GUH != 0.0 {
                        GUQ = GUE;
                        GUR = GUF;
                    } else {
                        let GUN = GTM - BL;
                        let GUO = GUE.powf(GUN);
                        let GUP = (GUF * (GUN * (GUE.powf((GUN - DS))))) + Lanes([0.0, (GTL * (GUO * (GUE.ln()))), 0.0]);
                        GUQ = GUO;
                        GUR = GUP;
                    }
                    GUI = GUQ;
                    GUJ = GUR;
                }
                let GUK = (GUF * GUI) + (GUJ * GUE);
                let GUL = BL + (GUE * GUI);
                let GUM = if (if 9.999999999999978e-1f64 <= GTM { 1.0 } else { 0.0 }) != 0.0 && (if GTM <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GUV;
                let GUW;
                if GUM != 0.0 {
                    let GUS = BL / GUL;
                    let GUT = ((GUK * GUS) * BP) / GUL;
                    GUV = GUS;
                    GUW = GUT;
                } else {
                    let GUU = if (if 1.9999999999999978e0f64 <= GTM { 1.0 } else { 0.0 }) != 0.0 && (if GTM <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GVJ;
                    let GVK;
                    if GUU != 0.0 {
                        let GVB = GUL.sqrt();
                        let GVC = BL / GVB;
                        let GVD = (((GUK * (DS / (ET * GVB))) * GVC) * BP) / GVB;
                        GVJ = GVC;
                        GVK = GVD;
                    } else {
                        let GVE = -1e0f64 / GTM;
                        let GVF = GVE - BL;
                        let GVG = GUL.powf(GVF);
                        let GVH = GUL * GVG;
                        let GVI = (GUK * GVG) + (((GUK * (GVF * (GUL.powf((GVF - DS))))) + Lanes([0.0, ((((GTL * GVE) * BP) / GTM) * (GVG * (GUL.ln()))), 0.0])) * GUL);
                        GVJ = GVH;
                        GVK = GVI;
                    }
                    GUV = GVJ;
                    GUW = GVK;
                }
                let GUY = (GUX * (GTO * GUV)) * EYR;
                let GUZ = ((Lanes([0.0, (GTP * GUV), 0.0]) + (GUW * GTO)) * GUX) * EYR;
                let GVA = if GUY <= AI { 1.0 } else { 0.0 };
                let GVL;
                let GVM;
                if GVA != 0.0 {
                    GVL = IH;
                    GVM = GTD;
                } else {
                    GVL = GUY;
                    GVM = GUZ;
                }
                let GVN = BL / GVL;
                let GVO = (((GVM * GVN) * BP) / GVL) / BFD;
                let GVP = (GVN / BFD) + staged[151];
                let GVQ = if (if GVP > MZ { 1.0 } else { 0.0 }) != 0.0 && EDB != 0.0 { 1.0 } else { 0.0 };
                let GVR = if GVP < MZ { 1.0 } else { 0.0 };
                let GVS;
                let GVT;
                if GVR != 0.0 {
                    GVS = MZ;
                    GVT = GTD;
                } else {
                    GVS = GVP;
                    GVT = GVO;
                }
                GTE = GVS;
                GTF = GVT;
            } else {
                GTE = AI;
                GTF = GTD;
            }
            let GVY;
            let GVZ;
            let GWA;
            let GWB;
            let GWC;
            let GWD;
            let GWE;
            let GWF;
            let GWG;
            let GWH;
            let GWI;
            let GWJ;
            if AN != 0.0 {
                let GVU = if GEO < DJQ { 1.0 } else { 0.0 };
                let GWK;
                let GWL;
                if GVU != 0.0 {
                    GWK = DJQ;
                    GWL = BCB;
                } else {
                    GWK = GEO;
                    GWL = GEQ;
                }
                let GWM = if GEP < DJQ { 1.0 } else { 0.0 };
                let GWN;
                let GWO;
                if GWM != 0.0 {
                    GWN = DJQ;
                    GWO = LY;
                } else {
                    GWN = GEP;
                    GWO = GER;
                }
                let GWP = (BC - GLB) / GWK;
                let GWQ = GWL * GWP;
                let GWR = ((Lanes([0.0, 0.0, 0.0, BE, 0.0, 0.0, 0.0]) - Lanes([GLG[0], GLG[1], GLG[2], 0.0, GLG[3], GLG[4], GLG[5]])) - Lanes([GWQ[0], GWQ[1], GWQ[2], 0.0, GWQ[3], GWQ[4], GWQ[5]])) / GWK;
                let GWS = (BD - GLC) / GWN;
                let GWT = GWO * GWS;
                let GWU = ((Lanes([0.0, 0.0, 0.0, BF, 0.0, 0.0, 0.0]) - Lanes([GLH[0], GLH[1], GLH[2], 0.0, GLH[3], GLH[4], GLH[5]])) - Lanes([0.0, GWT[0], GWT[1], 0.0, 0.0, GWT[2], GWT[3]])) / GWN;
                let GWV = (-BC) - BD;
                let GWW = Lanes([(BE * BP), 0.0]) - Lanes([0.0, BF]);
                let GWX = BC * KF;
                let GWY = BE * KF;
                let GXA = BC * GWZ;
                let GXB = BE * GWZ;
                GVY = GWX;
                GVZ = GXA;
                GWA = GWV;
                GWB = BD;
                GWC = GWP;
                GWD = GWS;
                GWE = GWY;
                GWF = GXB;
                GWG = GWW;
                GWH = BF;
                GWI = GWR;
                GWJ = GWU;
            } else {
                GVY = AI;
                GVZ = AI;
                GWA = AI;
                GWB = AI;
                GWC = AI;
                GWD = AI;
                GWE = BA;
                GWF = BA;
                GWG = GVV;
                GWH = BB;
                GWI = GVW;
                GWJ = GVX;
            }
            let GXI;
            let GXJ;
            let GXK;
            let GXL;
            let GXM;
            let GXN;
            let GXO;
            let GXP;
            let GXQ;
            let GXR;
            let GXS;
            let GXT;
            if GMZ != 0.0 {
                let GXC = -((GMR + GMS) + GMT);
                let GXD = ((GMU + GMV) + GMW) * BP;
                GXI = GJG;
                GXJ = EJX;
                GXK = AI;
                GXL = GMS;
                GXM = GVY;
                GXN = GXC;
                GXO = GJH;
                GXP = EJY;
                GXQ = BCB;
                GXR = GMV;
                GXS = GWE;
                GXT = GXD;
            } else {
                let GXE = -GJG;
                let GXF = GJH * BP;
                let GXG = -((GMR + GMS) + GMT);
                let GXH = ((GMU + GMV) + GMW) * BP;
                let GXU;
                let GXV;
                if AN != 0.0 {
                    GXU = GVZ;
                    GXV = GWF;
                } else {
                    GXU = GVY;
                    GXV = GWE;
                }
                GXI = GXE;
                GXJ = AI;
                GXK = EJX;
                GXL = GMT;
                GXM = GXU;
                GXN = GXG;
                GXO = GXF;
                GXP = BCB;
                GXQ = EJY;
                GXR = GMW;
                GXS = GXV;
                GXT = GXH;
            }
            let GXZ;
            let GYA;
            if AG != 0.0 {
                let GXW = GJG * CB;
                let GXX = CK * GJG;
                let GXY = (GJH * CB) + Lanes([0.0, 0.0, 0.0, 0.0, GXX[0], GXX[1]]);
                GXZ = GXW;
                GYA = GXY;
            } else {
                GXZ = AI;
                GYA = BCB;
            }
            let GYB = if CH != BL { 1.0 } else { 0.0 };
            let GYC = L * GXI;
            let GYD = GXO * L;
            let GYE = L * (GNO + GXJ);
            let GYF = (Lanes([GNP[0], GNP[1], GNP[2], 0.0, GNP[3], GNP[4]]) + GXP) * L;
            let GYG = L * (GNQ + GXK);
            let GYH = (Lanes([GNR[0], GNR[1], GNR[2], 0.0, GNR[3], GNR[4]]) + GXQ) * L;
            let GYI = L * GNM;
            let GYJ = GNN * L;
            let GYK = L * GNF;
            let GYL = GNG * L;
            let GYM = L * GMX;
            let GYN = GMY * L;
            let GYS;
            let GYT;
            if GYO != 0.0 {
                let GYP = Lanes([0.0, K]) - Lanes([X, 0.0]);
                let GYQ = (I - W) / GQH;
                let GYR = (Lanes([GYP[0], 0.0, GYP[1]]) - (GQI * GYQ)) / GQH;
                GYS = GYQ;
                GYT = GYR;
            } else {
                GYS = AI;
                GYT = GQG;
            }
            let GYY;
            let GYZ;
            if GYU != 0.0 {
                let GYV = Lanes([AB, 0.0]) - Lanes([0.0, P]);
                let GYW = (AA - O) / GTE;
                let GYX = (Lanes([GYV[0], 0.0, GYV[1]]) - (GTF * GYW)) / GTE;
                GYY = GYW;
                GYZ = GYX;
            } else {
                GYY = AI;
                GYZ = GTD;
            }
            let GZA = GMR + GWA;
            let GZB = Lanes([GMU[0], GMU[1], GMU[2], GMU[3], GMU[4], 0.0, 0.0, GMU[5], GMU[6], GMU[7]]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, GWG[0], GWG[1], 0.0, 0.0, 0.0]);
            let GZD = L * ddt(45332, GZA);
            let GZE = (GZB * GZC) * L;
            let GZF = L * GZA;
            let GZG = GZB * L;
            let GZH = GXL + GXM;
            let GZI = Lanes([GXR[0], GXR[1], GXR[2], GXR[3], GXR[4], 0.0, GXR[5], GXR[6], GXR[7]]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, GXS, 0.0, 0.0, 0.0]);
            let GZJ = L * ddt(45338, GZH);
            let GZK = (GZI * GZC) * L;
            let GZL = L * GZH;
            let GZM = GZI * L;
            let GZN = GXN + GWB;
            let GZO = Lanes([GXT[0], GXT[1], GXT[2], GXT[3], GXT[4], 0.0, GXT[5], GXT[6], GXT[7]]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, GWH, 0.0, 0.0, 0.0]);
            let GZP = L * ddt(45344, GZN);
            let GZQ = (GZO * GZC) * L;
            let GZR = L * GZN;
            let GZS = GZO * L;
            let GZU = GOE * GZT;
            let GZV = GOG * GZT;
            let GZX = Lanes([GZV[0], GZV[1], GZV[2], 0.0, GZV[3], GZV[4], GZV[5]]) + Lanes([0.0, 0.0, 0.0, (GZW * GOE), 0.0, 0.0, 0.0]);
            let GZY = GZT * GPW;
            let GZZ = GPX * GZT;
            let HAA = Lanes([0.0, 0.0, 0.0, (GZW * GPW), 0.0, 0.0, 0.0]) + Lanes([GZZ[0], GZZ[1], GZZ[2], 0.0, GZZ[3], GZZ[4], GZZ[5]]);
            let HAB = ddt(45375, GZY);
            let HAC = HAA * GZC;
            let HAD = GZT * GQD;
            let HAE = GQE * GZT;
            let HAF = Lanes([0.0, 0.0, 0.0, (GZW * GQD), 0.0, 0.0, 0.0]) + Lanes([HAE[0], HAE[1], HAE[2], 0.0, HAE[3], HAE[4], HAE[5]]);
            let HAG = ddt(45379, HAD);
            let HAH = HAF * GZC;
            let HAO;
            let HAP;
            if HAI != 0.0 {
                let HAL = HAK * (node_potentials[1] - H);
                let HAM = (Lanes([HAJ, 0.0]) - Lanes([0.0, J])) * HAK;
                HAO = HAL;
                HAP = HAM;
            } else {
                HAO = AI;
                HAP = HAN;
            }
            let HAW;
            let HAX;
            let HAY;
            let HAZ;
            if AG != 0.0 {
                let HAR = HAQ * AH;
                let HAS = AO * HAQ;
                let HAU = ((-GXZ) + ddt(45429, HAR)) + (AH * HAT);
                let HAV = ((GYA * BP) + Lanes([(HAS * GZC), 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([(AO * HAT), 0.0, 0.0, 0.0, 0.0, 0.0]);
                HAW = HAU;
                HAX = HAR;
                HAY = HAV;
                HAZ = HAS;
            } else {
                HAW = AI;
                HAX = AI;
                HAY = BCB;
                HAZ = AK;
            }
            let HBG;
            let HBH;
            let HBI;
            let HBJ;
            if HBA != 0.0 {
                let HBB = AR * BKV;
                let HBC = BKX * AR;
                let HBD = BCJ + ddt(45445, HBB);
                let HBE = BCR + Lanes([0.0, 0.0, 0.0, (HBC * GZC), 0.0, 0.0]);
                HBG = HBD;
                HBH = HBB;
                HBI = HBE;
                HBJ = HBC;
            } else {
                HBG = AI;
                HBH = AI;
                HBI = BCB;
                HBJ = HBF;
            }
            let HBS;
            let HBT;
            let HBU;
            let HBV;
            let HBW;
            let HBX;
            let HBY;
            let HBZ;
            if AN != 0.0 {
                let HBK = AR * AS;
                let HBL = AU * AR;
                let HBM = GWC + ddt(45453, HBK);
                let HBN = GWI + Lanes([0.0, 0.0, 0.0, (HBL * GZC), 0.0, 0.0, 0.0]);
                let HBO = AR * AW;
                let HBP = AY * AR;
                let HBQ = GWD + ddt(45459, HBO);
                let HBR = GWJ + Lanes([0.0, 0.0, 0.0, (HBP * GZC), 0.0, 0.0, 0.0]);
                HBS = HBM;
                HBT = HBQ;
                HBU = HBK;
                HBV = HBO;
                HBW = HBN;
                HBX = HBR;
                HBY = HBL;
                HBZ = HBP;
            } else {
                HBS = AI;
                HBT = AI;
                HBU = AI;
                HBV = AI;
                HBW = GVW;
                HBX = GVX;
                HBY = BA;
                HBZ = BB;
            }
            let HCA = GYD[0];
            let HCB = GYD[1];
            let HCC = GYD[2];
            let HCD = GYD[3];
            let HCE = GYD[4];
            let HCF = GYD[5];
            let HCG = GYF[0];
            let HCH = GYF[1];
            let HCI = GYF[2];
            let HCJ = GYF[3];
            let HCK = GYF[4];
            let HCL = GYF[5];
            let HCM = GYH[0];
            let HCN = GYH[1];
            let HCO = GYH[2];
            let HCP = GYH[3];
            let HCQ = GYH[4];
            let HCR = GYH[5];
            let HCS = GYJ[0];
            let HCT = GYJ[1];
            let HCU = GYJ[2];
            let HCV = GYJ[3];
            let HCW = GYJ[4];
            let HCX = GYJ[5];
            let HCY = GYL[0];
            let HCZ = GYL[1];
            let HDA = GYL[2];
            let HDB = GYL[3];
            let HDC = GYL[4];
            let HDD = GYL[5];
            let HDE = GYN[0];
            let HDF = GYN[1];
            let HDG = GYN[2];
            let HDH = GYN[3];
            let HDI = GYT[0];
            let HDJ = GYT[1];
            let HDK = GYT[2];
            let HDL = GYZ[0];
            let HDM = GYZ[1];
            let HDN = GYZ[2];
            let HDO = GZE[0];
            let HDP = GZE[1];
            let HDQ = GZE[2];
            let HDR = GZE[3];
            let HDS = GZE[4];
            let HDT = GZE[5];
            let HDU = GZE[6];
            let HDV = GZE[7];
            let HDW = GZE[8];
            let HDX = GZE[9];
            let HDY = GZK[0];
            let HDZ = GZK[1];
            let HEA = GZK[2];
            let HEB = GZK[3];
            let HEC = GZK[4];
            let HED = GZK[5];
            let HEE = GZK[6];
            let HEF = GZK[7];
            let HEG = GZK[8];
            let HEH = GZQ[0];
            let HEI = GZQ[1];
            let HEJ = GZQ[2];
            let HEK = GZQ[3];
            let HEL = GZQ[4];
            let HEM = GZQ[5];
            let HEN = GZQ[6];
            let HEO = GZQ[7];
            let HEP = GZQ[8];
            let HEQ = GZW;
            let HER = GZX[0];
            let HES = GZX[1];
            let HET = GZX[2];
            let HEU = GZX[3];
            let HEV = GZX[4];
            let HEW = GZX[5];
            let HEX = GZX[6];
            let HEY = HAC[0];
            let HEZ = HAC[1];
            let HFA = HAC[2];
            let HFB = HAC[3];
            let HFC = HAC[4];
            let HFD = HAC[5];
            let HFE = HAC[6];
            let HFF = HAH[0];
            let HFG = HAH[1];
            let HFH = HAH[2];
            let HFI = HAH[3];
            let HFJ = HAH[4];
            let HFK = HAH[5];
            let HFL = HAH[6];
            let HFM = HAP[0];
            let HFN = HAP[1];
            let HFO = HAY[0];
            let HFP = HAY[1];
            let HFQ = HAY[2];
            let HFR = HAY[3];
            let HFS = HAY[4];
            let HFT = HAY[5];
            let HFU = HBI[0];
            let HFV = HBI[1];
            let HFW = HBI[2];
            let HFX = HBI[3];
            let HFY = HBI[4];
            let HFZ = HBI[5];
            let HGA = HBW[0];
            let HGB = HBW[1];
            let HGC = HBW[2];
            let HGD = HBW[3];
            let HGE = HBW[4];
            let HGF = HBW[5];
            let HGG = HBW[6];
            let HGH = HBX[0];
            let HGI = HBX[1];
            let HGJ = HBX[2];
            let HGK = HBX[3];
            let HGL = HBX[4];
            let HGM = HBX[5];
            let HGN = HBX[6];
            let HGO = GZG[0];
            let HGP = GZG[1];
            let HGQ = GZG[2];
            let HGR = GZG[3];
            let HGS = GZG[4];
            let HGT = GZG[5];
            let HGU = GZG[6];
            let HGV = GZG[7];
            let HGW = GZG[8];
            let HGX = GZG[9];
            let HGY = GZM[0];
            let HGZ = GZM[1];
            let HHA = GZM[2];
            let HHB = GZM[3];
            let HHC = GZM[4];
            let HHD = GZM[5];
            let HHE = GZM[6];
            let HHF = GZM[7];
            let HHG = GZM[8];
            let HHH = GZS[0];
            let HHI = GZS[1];
            let HHJ = GZS[2];
            let HHK = GZS[3];
            let HHL = GZS[4];
            let HHM = GZS[5];
            let HHN = GZS[6];
            let HHO = GZS[7];
            let HHP = GZS[8];
            let HHQ = HAA[0];
            let HHR = HAA[1];
            let HHS = HAA[2];
            let HHT = HAA[3];
            let HHU = HAA[4];
            let HHV = HAA[5];
            let HHW = HAA[6];
            let HHX = HAF[0];
            let HHY = HAF[1];
            let HHZ = HAF[2];
            let HIA = HAF[3];
            let HIB = HAF[4];
            let HIC = HAF[5];
            let HID = HAF[6];
            let HIE = HAZ;
            let HIF = HBJ;
            let HIG = HBY;
            let HIH = HBZ;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(12),
            multiplicity * (GYC),
            [4, 5, 6, 10, 11, 12],
            [HCA, HCB, HCC, HCD, HCE, HCF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(12),
            multiplicity * (GYE),
            [4, 5, 6, 10, 11, 12],
            [HCG, HCH, HCI, HCJ, HCK, HCL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(11),
            multiplicity * (GYG),
            [4, 5, 6, 10, 11, 12],
            [HCM, HCN, HCO, HCP, HCQ, HCR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(12),
            multiplicity * (GYI),
            [4, 5, 6, 10, 11, 12],
            [HCS, HCT, HCU, HCV, HCW, HCX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(11),
            multiplicity * (GYK),
            [4, 5, 6, 10, 11, 12],
            [HCY, HCZ, HDA, HDB, HDC, HDD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (GYM),
            [5, 6, 11, 12],
            [HDE, HDF, HDG, HDH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(2),
            multiplicity * (GYS),
            [2, 4, 12],
            [HDI, HDJ, HDK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(12), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[228],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(11),
            multiplicity * (GYY),
            [0, 4, 11],
            [HDL, HDM, HDN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(11), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[229],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (GZD),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [HDO, HDP, HDQ, HDR, HDS, HDT, HDU, HDV, HDW, HDX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (GZJ),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [HDY, HDZ, HEA, HEB, HEC, HED, HEE, HEF, HEG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(12),
            multiplicity * (GZP),
            [0, 2, 4, 5, 6, 9, 10, 11, 12],
            [HEH, HEI, HEJ, HEK, HEL, HEM, HEN, HEO, HEP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(12),
            multiplicity * (HII),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (GZT),
            [7],
            [HEQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            None,
            multiplicity * (HIJ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(12),
            multiplicity * (HIK),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(12),
            multiplicity * (GZU),
            [4, 5, 6, 7, 10, 11, 12],
            [HER, HES, HET, HEU, HEV, HEW, HEX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(12),
            multiplicity * (HAB),
            [4, 5, 6, 7, 10, 11, 12],
            [HEY, HEZ, HFA, HFB, HFC, HFD, HFE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(11),
            multiplicity * (HAG),
            [4, 5, 6, 7, 10, 11, 12],
            [HFF, HFG, HFH, HFI, HFJ, HFK, HFL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            Some(2),
            multiplicity * (staged[230]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(11),
            multiplicity * (staged[231]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(11),
            multiplicity * (HIL),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(12),
            multiplicity * (HIM),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (HIN),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(5),
            multiplicity * (HAO),
            [1, 5],
            [HFM, HFN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(5), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[232],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(6), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            HIO,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            None,
            multiplicity * (HAW),
            [4, 5, 6, 10, 11, 12],
            [HFO, HFP, HFQ, HFR, HFS, HFT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[233],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (HBG),
            [4, 5, 6, 10, 11, 12],
            [HFU, HFV, HFW, HFX, HFY, HFZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), None, 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[234],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            None,
            multiplicity * (HBS),
            [4, 5, 6, 8, 10, 11, 12],
            [HGA, HGB, HGC, HGD, HGE, HGF, HGG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(9),
            None,
            multiplicity * (HBT),
            [4, 5, 6, 9, 10, 11, 12],
            [HGH, HGI, HGJ, HGK, HGL, HGM, HGN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(8), None, 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[235],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(9), None, 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            staged[236],
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = GYC;
        self.canonical_reactive[1] = GYE;
        self.canonical_reactive[2] = GYG;
        self.canonical_reactive[3] = GYI;
        self.canonical_reactive[4] = GYK;
        self.canonical_reactive[5] = GYM;
        self.canonical_reactive[6] = GYS;
        self.canonical_reactive[7] = staged[228];
        self.canonical_reactive[8] = GYY;
        self.canonical_reactive[9] = staged[229];
        self.canonical_reactive[10] = GZF;
        self.canonical_reactive[11] = HGO;
        self.canonical_reactive[12] = HGP;
        self.canonical_reactive[13] = HGQ;
        self.canonical_reactive[14] = HGR;
        self.canonical_reactive[15] = HGS;
        self.canonical_reactive[16] = HGT;
        self.canonical_reactive[17] = HGU;
        self.canonical_reactive[18] = HGV;
        self.canonical_reactive[19] = HGW;
        self.canonical_reactive[20] = HGX;
        self.canonical_reactive[21] = GZL;
        self.canonical_reactive[22] = HGY;
        self.canonical_reactive[23] = HGZ;
        self.canonical_reactive[24] = HHA;
        self.canonical_reactive[25] = HHB;
        self.canonical_reactive[26] = HHC;
        self.canonical_reactive[27] = HHD;
        self.canonical_reactive[28] = HHE;
        self.canonical_reactive[29] = HHF;
        self.canonical_reactive[30] = HHG;
        self.canonical_reactive[31] = GZR;
        self.canonical_reactive[32] = HHH;
        self.canonical_reactive[33] = HHI;
        self.canonical_reactive[34] = HHJ;
        self.canonical_reactive[35] = HHK;
        self.canonical_reactive[36] = HHL;
        self.canonical_reactive[37] = HHM;
        self.canonical_reactive[38] = HHN;
        self.canonical_reactive[39] = HHO;
        self.canonical_reactive[40] = HHP;
        self.canonical_reactive[41] = HII;
        self.canonical_reactive[42] = GZT;
        self.canonical_reactive[43] = HIJ;
        self.canonical_reactive[44] = HIK;
        self.canonical_reactive[45] = GZU;
        self.canonical_reactive[46] = GZY;
        self.canonical_reactive[47] = HHQ;
        self.canonical_reactive[48] = HHR;
        self.canonical_reactive[49] = HHS;
        self.canonical_reactive[50] = HHT;
        self.canonical_reactive[51] = HHU;
        self.canonical_reactive[52] = HHV;
        self.canonical_reactive[53] = HHW;
        self.canonical_reactive[54] = HAD;
        self.canonical_reactive[55] = HHX;
        self.canonical_reactive[56] = HHY;
        self.canonical_reactive[57] = HHZ;
        self.canonical_reactive[58] = HIA;
        self.canonical_reactive[59] = HIB;
        self.canonical_reactive[60] = HIC;
        self.canonical_reactive[61] = HID;
        self.canonical_reactive[62] = staged[230];
        self.canonical_reactive[63] = staged[231];
        self.canonical_reactive[64] = HIL;
        self.canonical_reactive[65] = HIM;
        self.canonical_reactive[66] = HIN;
        self.canonical_reactive[67] = HAO;
        self.canonical_reactive[68] = staged[232];
        self.canonical_reactive[69] = HIO;
        self.canonical_reactive[70] = HAX;
        self.canonical_reactive[71] = HIE;
        self.canonical_reactive[72] = staged[233];
        self.canonical_reactive[73] = HBH;
        self.canonical_reactive[74] = HIF;
        self.canonical_reactive[75] = staged[234];
        self.canonical_reactive[76] = HBU;
        self.canonical_reactive[77] = HIG;
        self.canonical_reactive[78] = HBV;
        self.canonical_reactive[79] = HIH;
        self.canonical_reactive[80] = staged[235];
        self.canonical_reactive[81] = staged[236];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(12),
            &[0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            &[cached[11], cached[12], cached[13], cached[14], cached[15], cached[16], cached[17], cached[18], cached[19], cached[20]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(12),
            &[0, 2, 4, 5, 6, 8, 10, 11, 12],
            &[cached[22], cached[23], cached[24], cached[25], cached[26], cached[27], cached[28], cached[29], cached[30]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(12),
            &[0, 2, 4, 5, 6, 9, 10, 11, 12],
            &[cached[32], cached[33], cached[34], cached[35], cached[36], cached[37], cached[38], cached[39], cached[40]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(12),
            &[4, 5, 6, 7, 10, 11, 12],
            &[cached[47], cached[48], cached[49], cached[50], cached[51], cached[52], cached[53]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(11),
            &[4, 5, 6, 7, 10, 11, 12],
            &[cached[55], cached[56], cached[57], cached[58], cached[59], cached[60], cached[61]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[71]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[74]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            None,
            &[8],
            &[cached[77]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            None,
            &[9],
            &[cached[79]],
            &[],
            &[],
            multiplicity,
        );
    }

}
