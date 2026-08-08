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
        let mut key = Vec::with_capacity(72);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[5] = values[0];
        self.canonical_staged[6] = values[1];
        self.canonical_staged[1] = values[2];
        self.canonical_staged[0] = values[3];
        self.canonical_staged[3] = values[4];
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
                let B = if parameter_given[10] { 1.0 } else { 0.0 };
                let C = 1e0f64;
                let G = 0e0f64;
                let mut oD = 0.0;
                let A = if parameters[14] != 1.002e3f64 { 1.0 } else { 0.0 };
                if B != 0.0 {
                    let D = C - (1e-2f64 * parameters[10]);
                    oD = D;
                } else {
                }
                let E = 2.7315e2f64 + parameters[15];
                let F = parameters[34] + C;
                let H = if (if parameters[28] > G { 1.0 } else { 0.0 }) != 0.0 || (if parameters[26] > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            [A, oD, E, F, H]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 10] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[3];
                let B = parameters[4];
                let D = parameters[22];
                let G = if parameter_given[1] { 1.0 } else { 0.0 };
                let H = if parameter_given[2] { 1.0 } else { 0.0 };
                let K = 0e0f64;
                let M = parameters[2];
                let N = parameters[1];
                let R = parameters[0];
                let mut oE = 0.0;
                let mut oO = 0.0;
                let mut oP = 0.0;
                let mut oQ = 0.0;
                let mut oS = 0.0;
                let mut oT = 0.0;
                let mut oU = 0.0;
                let C = if A != 0.0 && B != 0.0 { 1.0 } else { 0.0 };
                let F;
                if C != 0.0 {
                    F = D;
                } else {
                    let E = if A != 0.0 || B != 0.0 { 1.0 } else { 0.0 };
                    oE = E;
                    let L = if E != 0.0 {
                        let J = D * 5e-1f64;
                        J
                    } else {
                        K
                    };
                    F = L;
                }
                let I = if (if G != 0.0 && H != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[0] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if I != 0.0 {
                    let O = if (if M == K { 1.0 } else { 0.0 }) != 0.0 || (if N == K { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oO = O;
                } else {
                    let P = if H != 0.0 && (if G == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oP = P;
                    if P != 0.0 {
                        let Q = if M == K { 1.0 } else { 0.0 };
                        oQ = Q;
                        if Q != 0.0 {
                        } else {
                            let T = if R == K { 1.0 } else { 0.0 };
                            oT = T;
                        }
                    } else {
                        let S = if R == K { 1.0 } else { 0.0 };
                        oS = S;
                        if S != 0.0 {
                        } else {
                            let U = if N == K { 1.0 } else { 0.0 };
                            oU = U;
                        }
                    }
                }
            [C, oE, I, oO, F, oP, oQ, oT, oS, oU]
        };
        self.canonical_staged[12] = produced[0];
        self.canonical_staged[13] = produced[1];
        self.canonical_staged[14] = produced[2];
        self.canonical_staged[15] = produced[3];
        self.canonical_staged[2] = produced[4];
        self.canonical_staged[16] = produced[5];
        self.canonical_staged[17] = produced[6];
        self.canonical_staged[19] = produced[7];
        self.canonical_staged[18] = produced[8];
        self.canonical_staged[20] = produced[9];
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
        let produced: [f64; 6] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = 2.7315e2f64;
                let F = parameters[34];
                let G = 1e0f64;
                let I = parameters[35];
                let mut oJ = 0.0;
                let B = (temperature + parameters[5]) - A;
                let C = if B < parameters[11] { 1.0 } else { 0.0 };
                let D = if B > parameters[12] { 1.0 } else { 0.0 };
                let E = if B < staged[0] { 1.0 } else { 0.0 };
                let K;
                if E != 0.0 {
                    let H = F + (((B - F) - G).exp());
                    K = H;
                } else {
                    let J = if B > (I - G) { 1.0 } else { 0.0 };
                    oJ = J;
                    let O = if J != 0.0 {
                        let N = I - (((I - B) - G).exp());
                        N
                    } else {
                        B
                    };
                    K = O;
                }
                let L = (K + A) - staged[1];
                let M = if ((G + (L * parameters[42])) * parameters[29]) < 0e0f64 { 1.0 } else { 0.0 };
            [C, D, E, oJ, L, M]
        };
        self.canonical_staged[7] = produced[0];
        self.canonical_staged[8] = produced[1];
        self.canonical_staged[9] = produced[2];
        self.canonical_staged[10] = produced[3];
        self.canonical_staged[4] = produced[4];
        self.canonical_staged[11] = produced[5];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        self.canonical_model_stage(ctx);
        self.canonical_instance_stage(ctx);
        self.canonical_temperature_stage(ctx);
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
            let A = if parameter_given[9] { 1.0 } else { 0.0 };
            let B = parameters[9];
            let C = 1e0f64;
            let F = if parameter_given[10] { 1.0 } else { 0.0 };
            let G = staged[6];
            let H = 0e0f64;
            let I = 1e-2f64;
            let L = if parameter_given[13] { 1.0 } else { 0.0 };
            let M = parameters[13];
            let Q = staged[12];
            let R = staged[14];
            let S = staged[15];
            let T = staged[16];
            let AB = parameters[0];
            let AD = parameters[21];
            let AF = 1e99f64;
            let AG = parameters[1];
            let AI = staged[2];
            let AS = parameters[16];
            let AT = parameters[2];
            let BE = staged[17];
            let BF = staged[18];
            let BO = staged[19];
            let CT = staged[20];
            let DZ = parameters[24];
            let EA = parameters[23];
            let EF = staged[3];
            let EI = parameters[36];
            let EJ = parameters[37];
            let EN = parameters[38];
            let EP = parameters[39];
            let EU = 5e-1f64;
            let FE = staged[4];
            let FL = 1e0f64;
            let FM = 1e0f64;
            let FR = parameters[27];
            let FV = 2e0f64;
            let FW = 1e0f64;
            let FX = parameters[25];
            let GD = 3.333333333333333e-1f64;
            let GE = parameters[28];
            let GF = parameters[26];
            let GI = Lanes([0e0f64; 2]);
            let GY = 0e0f64;
            let GZ = 0e0f64;
            let E = if A != 0.0 {
                B
            } else {
                let D = ctx.simparam_or("scale", C);
                D
            };
            let K = if F != 0.0 {
                G
            } else {
                let J = C - (I * (ctx.simparam_or("shrink", H)));
                J
            };
            let O = if L != 0.0 {
                M
            } else {
                let N = ctx.simparam_or("rthresh", 1e-3f64);
                N
            };
            let P = (K * E) * 1e6f64;
            let U;
            let V;
            let W;
            let X;
            let Y;
            let Z;
            if R != 0.0 {
                let AL;
                let AM;
                let AN;
                let AO;
                let AP;
                let AQ;
                if S != 0.0 {
                    let AC = AB * P;
                    let AE = AC + AD;
                    AL = H;
                    AM = AC;
                    AN = H;
                    AO = H;
                    AP = AE;
                    AQ = AF;
                } else {
                    let AH = AG * P;
                    let AJ = AH + AI;
                    let AK = if AJ < H { 1.0 } else { 0.0 };
                    let AR = if AJ > H { 1.0 } else { 0.0 };
                    let AZ;
                    let BA;
                    let BB;
                    let BC;
                    if AR != 0.0 {
                        let AU = (AS / AT) * AJ;
                        let AV = AU - AD;
                        let AW = if AV <= H { 1.0 } else { 0.0 };
                        let BD = C / AT;
                        AZ = AV;
                        BA = AT;
                        BB = AU;
                        BC = BD;
                    } else {
                        let AX = AB * P;
                        let AY = AX + AD;
                        AZ = AX;
                        BA = H;
                        BB = AY;
                        BC = AF;
                    }
                    AL = AH;
                    AM = AZ;
                    AN = AJ;
                    AO = BA;
                    AP = BB;
                    AQ = BC;
                }
                U = AL;
                V = AM;
                W = AN;
                X = AO;
                Y = AP;
                Z = AQ;
            } else {
                let BG;
                let BH;
                let BI;
                let BJ;
                let BK;
                let BL;
                if T != 0.0 {
                    let BP;
                    let BQ;
                    let BR;
                    let BS;
                    let BT;
                    let BU;
                    if BE != 0.0 {
                        let BM = AB * P;
                        let BN = BM + AD;
                        BP = H;
                        BQ = BM;
                        BR = H;
                        BS = H;
                        BT = BN;
                        BU = AF;
                    } else {
                        let CA;
                        let CB;
                        let CC;
                        let CD;
                        let CE;
                        let CF;
                        if BO != 0.0 {
                            let BV = AG * P;
                            let BW = BV + AI;
                            CA = BV;
                            CB = H;
                            CC = BW;
                            CD = AF;
                            CE = H;
                            CF = H;
                        } else {
                            let BX = AB * P;
                            let BY = BX + AD;
                            let BZ = if BY < H { 1.0 } else { 0.0 };
                            let CG = if BY > H { 1.0 } else { 0.0 };
                            let CM;
                            let CN;
                            let CO;
                            let CP;
                            if CG != 0.0 {
                                let CH = (AT / AS) * BY;
                                let CI = CH - AI;
                                let CJ = if CI <= H { 1.0 } else { 0.0 };
                                let CQ = C / AT;
                                CM = CI;
                                CN = CH;
                                CO = AT;
                                CP = CQ;
                            } else {
                                let CK = AG * P;
                                let CL = CK + AI;
                                CM = CK;
                                CN = CL;
                                CO = AF;
                                CP = H;
                            }
                            CA = CM;
                            CB = BX;
                            CC = CN;
                            CD = CO;
                            CE = BY;
                            CF = CP;
                        }
                        BP = CA;
                        BQ = CB;
                        BR = CC;
                        BS = CD;
                        BT = CE;
                        BU = CF;
                    }
                    BG = BP;
                    BH = BQ;
                    BI = BR;
                    BJ = BS;
                    BK = BT;
                    BL = BU;
                } else {
                    let CU;
                    let CV;
                    let CW;
                    let CX;
                    let CY;
                    let CZ;
                    if BF != 0.0 {
                        let CR = AG * P;
                        let CS = CR + AI;
                        CU = CR;
                        CV = H;
                        CW = CS;
                        CX = AF;
                        CY = H;
                        CZ = H;
                    } else {
                        let DF;
                        let DG;
                        let DH;
                        let DI;
                        let DJ;
                        let DK;
                        if CT != 0.0 {
                            let DA = AB * P;
                            let DB = DA + AD;
                            DF = H;
                            DG = DA;
                            DH = H;
                            DI = H;
                            DJ = DB;
                            DK = AF;
                        } else {
                            let DC = AB * P;
                            let DD = DC + AD;
                            let DE = if DD < H { 1.0 } else { 0.0 };
                            let DL = AG * P;
                            let DM = DL + AI;
                            let DN = if DD > H { 1.0 } else { 0.0 };
                            let DP;
                            let DQ;
                            if DN != 0.0 {
                                let DO = if DM < H { 1.0 } else { 0.0 };
                                let DR = if DM > H { 1.0 } else { 0.0 };
                                let DU;
                                let DV;
                                if DR != 0.0 {
                                    let DS = AS * (DM / DD);
                                    let DT = C / DS;
                                    DU = DS;
                                    DV = DT;
                                } else {
                                    DU = H;
                                    DV = AF;
                                }
                                DP = DU;
                                DQ = DV;
                            } else {
                                DP = AF;
                                DQ = H;
                            }
                            DF = DL;
                            DG = DC;
                            DH = DM;
                            DI = DP;
                            DJ = DD;
                            DK = DQ;
                        }
                        CU = DF;
                        CV = DG;
                        CW = DH;
                        CX = DI;
                        CY = DJ;
                        CZ = DK;
                    }
                    BG = CU;
                    BH = CV;
                    BI = CW;
                    BJ = CX;
                    BK = CY;
                    BL = CZ;
                }
                U = BG;
                V = BH;
                W = BI;
                X = BJ;
                Y = BK;
                Z = BL;
            }
            let AA = if U < parameters[17] { 1.0 } else { 0.0 };
            let DW = if U > parameters[18] { 1.0 } else { 0.0 };
            let DX = if V < parameters[19] { 1.0 } else { 0.0 };
            let DY = if V > parameters[20] { 1.0 } else { 0.0 };
            let ED = if DZ != 0.0 {
                let EB = W + EA;
                EB
            } else {
                let EC = U + EA;
                EC
            };
            let EE = if X > H { 1.0 } else { 0.0 };
            let EG = if (if (if ED <= H { 1.0 } else { 0.0 }) != 0.0 && EE != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EF != 0.0 { 1.0 } else { 0.0 };
            let EH = if W > H { 1.0 } else { 0.0 };
            let EK;
            let EL;
            if EH != 0.0 {
                let ES;
                let ET;
                if Q != 0.0 {
                    let EO = EI + (EN / W);
                    let EQ = EJ + (EP / W);
                    ES = EO;
                    ET = EQ;
                } else {
                    let ER = if parameters[3] != 0.0 || parameters[4] != 0.0 { 1.0 } else { 0.0 };
                    let EX;
                    let EY;
                    if ER != 0.0 {
                        let EV = EI + ((EU * EN) / W);
                        let EW = EJ + ((EU * EP) / W);
                        EX = EV;
                        EY = EW;
                    } else {
                        EX = EI;
                        EY = EJ;
                    }
                    ES = EX;
                    ET = EY;
                }
                EK = ES;
                EL = ET;
            } else {
                EK = EI;
                EL = EJ;
            }
            let EM = if Y > H { 1.0 } else { 0.0 };
            let FB;
            let FC;
            if EM != 0.0 {
                let EZ = EK + (parameters[40] / Y);
                let FA = EL + (parameters[41] / Y);
                FB = EZ;
                FC = FA;
            } else {
                FB = EK;
                FC = EL;
            }
            let FD = if X > (O / multiplicity) { 1.0 } else { 0.0 };
            let FF = C + (FE * (FB + (FE * FC)));
            let FG = if FF < 1.1e-1f64 { 1.0 } else { 0.0 };
            let FI = if FG != 0.0 {
                let FH = I + (1e-1f64 * (((1e1f64 * (FF - I)) - C).exp()));
                FH
            } else {
                FF
            };
            let FJ = X * FI;
            let FK = node_potentials[0] - node_potentials[1];
            let FN = Lanes([FL, 0.0]) - Lanes([0.0, FM]);
            let FO = if EE != 0.0 && EF != 0.0 { 1.0 } else { 0.0 };
            let GJ;
            let GK;
            if FO != 0.0 {
                let FP = FK / ED;
                let FQ = FN / ED;
                let FS = FR * FP;
                let FT = (FQ * FR) * FS;
                let FU = (C + (FS * FS)).sqrt();
                let FY = FX * (FP.abs());
                let FZ = (FQ * ((FV * (if FP >= 0e0f64 { 1.0 } else { 0.0 })) - FW)) * FX;
                let GA = FY * FY;
                let GB = FZ * FY;
                let GC = C + (GA * FY);
                let GG = (((C - GE) - GF) + (GE * FU)) + (GF * (GC.powf(GD)));
                let GH = (((FT + FT) * (FW / (FV * FU))) * GE) + (((((GB + GB) * FY) + (FZ * GA)) * (GD * (GC.powf(-6.666666666666667e-1f64)))) * GF);
                GJ = GG;
                GK = GH;
            } else {
                GJ = C;
                GK = GI;
            }
            let GL = FJ * GJ;
            let GM = FK / GL;
            let GN = (FN - ((GK * FJ) * GM)) / GL;
            if EM != 0.0 {
                let GO = if ((GM / Y).abs()) > parameters[33] { 1.0 } else { 0.0 };
            } else {
            }
            let GP = if Z > H { 1.0 } else { 0.0 };
            let GQ = if (if parameters[6] != 0.0 && EE != 0.0 { 1.0 } else { 0.0 }) != 0.0 && GP != 0.0 { 1.0 } else { 0.0 };
            if GQ != 0.0 {
                let GR = if (if parameters[32] != 0.0 && EH != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EM != 0.0 { 1.0 } else { 0.0 };
                if GR != 0.0 {
                } else {
                    let GT = if (if U > H { 1.0 } else { 0.0 }) != 0.0 && (if V > H { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                }
                let GU = if GM < H { 1.0 } else { 0.0 };
            } else {
            }
            let GS = if EE != 0.0 && GP != 0.0 { 1.0 } else { 0.0 };
            if GS != 0.0 {
                let GV = if GN[0] != H { 1.0 } else { 0.0 };
            } else {
            }
            let GW = GN[0];
            let GX = GN[1];
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(1),
            multiplicity * (GM),
            [0, 1],
            [GW, GX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (GY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (GZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
    }

}
