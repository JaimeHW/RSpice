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
        let mut key = Vec::with_capacity(84);
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
        self.canonical_staged[4] = values[2];
        self.canonical_staged[1] = values[3];
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
                let B = if parameter_given[11] { 1.0 } else { 0.0 };
                let C = 1e0f64;
                let F = 0e0f64;
                let mut oD = 0.0;
                let A = if parameters[15] != 1.002e3f64 { 1.0 } else { 0.0 };
                if B != 0.0 {
                    let D = C - (1e-2f64 * parameters[11]);
                    oD = D;
                } else {
                }
                let E = 2.7315e2f64 + parameters[16];
                let G = if (if parameters[29] > F { 1.0 } else { 0.0 }) != 0.0 || (if parameters[27] > F { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let H = parameters[35] + C;
            [A, oD, E, G, H]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 11] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[3];
                let B = parameters[4];
                let D = parameters[23];
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
                let mut oV = 0.0;
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
                if C != 0.0 {
                } else {
                    let V = if A != 0.0 || B != 0.0 { 1.0 } else { 0.0 };
                    oV = V;
                }
            [C, oE, I, oO, F, oP, oQ, oT, oS, oU, oV]
        };
        self.canonical_staged[9] = produced[0];
        self.canonical_staged[10] = produced[1];
        self.canonical_staged[11] = produced[2];
        self.canonical_staged[12] = produced[3];
        self.canonical_staged[0] = produced[4];
        self.canonical_staged[13] = produced[5];
        self.canonical_staged[14] = produced[6];
        self.canonical_staged[16] = produced[7];
        self.canonical_staged[15] = produced[8];
        self.canonical_staged[17] = produced[9];
        self.canonical_staged[18] = produced[10];
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
        let produced: [f64; 3] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = (temperature + parameters[5]) - 2.7315e2f64;
                let B = if A < parameters[12] { 1.0 } else { 0.0 };
                let C = if A > parameters[13] { 1.0 } else { 0.0 };
            [A, B, C]
        };
        self.canonical_staged[2] = produced[0];
        self.canonical_staged[7] = produced[1];
        self.canonical_staged[8] = produced[2];
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
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = 0usize;
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
            let A = if parameter_given[10] { 1.0 } else { 0.0 };
            let B = parameters[10];
            let C = 1e0f64;
            let F = if parameter_given[11] { 1.0 } else { 0.0 };
            let G = staged[6];
            let H = 0e0f64;
            let I = 1e-2f64;
            let L = if parameter_given[14] { 1.0 } else { 0.0 };
            let M = parameters[14];
            let P = 1e6f64;
            let R = staged[9];
            let S = staged[11];
            let T = staged[12];
            let U = staged[13];
            let AC = parameters[0];
            let AE = parameters[22];
            let AG = 1e99f64;
            let AH = parameters[1];
            let AJ = staged[0];
            let AT = parameters[17];
            let AU = parameters[2];
            let BF = staged[14];
            let BG = staged[15];
            let BP = staged[16];
            let CU = staged[17];
            let EA = parameters[25];
            let EB = parameters[24];
            let EG = staged[1];
            let EJ = parameters[37];
            let EK = parameters[38];
            let EO = parameters[39];
            let EQ = parameters[40];
            let EV = 5e-1f64;
            let FF = 2e0f64;
            let FH = staged[18];
            let FP = parameters[7];
            let FQ = node_potentials[2];
            let FR = 1e0f64;
            let FV = parameters[35];
            let FZ = parameters[36];
            let GI = -1e0f64;
            let GO = 1e1f64;
            let GQ = 1e-1f64;
            let HA = 1e0f64;
            let HB = 1e0f64;
            let HG = parameters[28];
            let HK = 2e0f64;
            let HL = 1e0f64;
            let HM = parameters[26];
            let HS = 3.333333333333333e-1f64;
            let HT = parameters[29];
            let HU = parameters[27];
            let HX = Lanes([0e0f64; 2]);
            let IN = 0e0f64;
            let IQ = Lanes([0e0f64; 3]);
            let IY = ddt_scale();
            let KB = 0e0f64;
            let KC = 0e0f64;
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
            let Q = (K * E) * P;
            let V;
            let W;
            let X;
            let Y;
            let Z;
            let AA;
            if S != 0.0 {
                let AM;
                let AN;
                let AO;
                let AP;
                let AQ;
                let AR;
                if T != 0.0 {
                    let AD = AC * Q;
                    let AF = AD + AE;
                    AM = H;
                    AN = AD;
                    AO = H;
                    AP = H;
                    AQ = AF;
                    AR = AG;
                } else {
                    let AI = AH * Q;
                    let AK = AI + AJ;
                    let AL = if AK < H { 1.0 } else { 0.0 };
                    let AS = if AK > H { 1.0 } else { 0.0 };
                    let BA;
                    let BB;
                    let BC;
                    let BD;
                    if AS != 0.0 {
                        let AV = (AT / AU) * AK;
                        let AW = AV - AE;
                        let AX = if AW <= H { 1.0 } else { 0.0 };
                        let BE = C / AU;
                        BA = AW;
                        BB = AU;
                        BC = AV;
                        BD = BE;
                    } else {
                        let AY = AC * Q;
                        let AZ = AY + AE;
                        BA = AY;
                        BB = H;
                        BC = AZ;
                        BD = AG;
                    }
                    AM = AI;
                    AN = BA;
                    AO = AK;
                    AP = BB;
                    AQ = BC;
                    AR = BD;
                }
                V = AM;
                W = AN;
                X = AO;
                Y = AP;
                Z = AQ;
                AA = AR;
            } else {
                let BH;
                let BI;
                let BJ;
                let BK;
                let BL;
                let BM;
                if U != 0.0 {
                    let BQ;
                    let BR;
                    let BS;
                    let BT;
                    let BU;
                    let BV;
                    if BF != 0.0 {
                        let BN = AC * Q;
                        let BO = BN + AE;
                        BQ = H;
                        BR = BN;
                        BS = H;
                        BT = H;
                        BU = BO;
                        BV = AG;
                    } else {
                        let CB;
                        let CC;
                        let CD;
                        let CE;
                        let CF;
                        let CG;
                        if BP != 0.0 {
                            let BW = AH * Q;
                            let BX = BW + AJ;
                            CB = BW;
                            CC = H;
                            CD = BX;
                            CE = AG;
                            CF = H;
                            CG = H;
                        } else {
                            let BY = AC * Q;
                            let BZ = BY + AE;
                            let CA = if BZ < H { 1.0 } else { 0.0 };
                            let CH = if BZ > H { 1.0 } else { 0.0 };
                            let CN;
                            let CO;
                            let CP;
                            let CQ;
                            if CH != 0.0 {
                                let CI = (AU / AT) * BZ;
                                let CJ = CI - AJ;
                                let CK = if CJ <= H { 1.0 } else { 0.0 };
                                let CR = C / AU;
                                CN = CJ;
                                CO = CI;
                                CP = AU;
                                CQ = CR;
                            } else {
                                let CL = AH * Q;
                                let CM = CL + AJ;
                                CN = CL;
                                CO = CM;
                                CP = AG;
                                CQ = H;
                            }
                            CB = CN;
                            CC = BY;
                            CD = CO;
                            CE = CP;
                            CF = BZ;
                            CG = CQ;
                        }
                        BQ = CB;
                        BR = CC;
                        BS = CD;
                        BT = CE;
                        BU = CF;
                        BV = CG;
                    }
                    BH = BQ;
                    BI = BR;
                    BJ = BS;
                    BK = BT;
                    BL = BU;
                    BM = BV;
                } else {
                    let CV;
                    let CW;
                    let CX;
                    let CY;
                    let CZ;
                    let DA;
                    if BG != 0.0 {
                        let CS = AH * Q;
                        let CT = CS + AJ;
                        CV = CS;
                        CW = H;
                        CX = CT;
                        CY = AG;
                        CZ = H;
                        DA = H;
                    } else {
                        let DG;
                        let DH;
                        let DI;
                        let DJ;
                        let DK;
                        let DL;
                        if CU != 0.0 {
                            let DB = AC * Q;
                            let DC = DB + AE;
                            DG = H;
                            DH = DB;
                            DI = H;
                            DJ = H;
                            DK = DC;
                            DL = AG;
                        } else {
                            let DD = AC * Q;
                            let DE = DD + AE;
                            let DF = if DE < H { 1.0 } else { 0.0 };
                            let DM = AH * Q;
                            let DN = DM + AJ;
                            let DO = if DE > H { 1.0 } else { 0.0 };
                            let DQ;
                            let DR;
                            if DO != 0.0 {
                                let DP = if DN < H { 1.0 } else { 0.0 };
                                let DS = if DN > H { 1.0 } else { 0.0 };
                                let DV;
                                let DW;
                                if DS != 0.0 {
                                    let DT = AT * (DN / DE);
                                    let DU = C / DT;
                                    DV = DT;
                                    DW = DU;
                                } else {
                                    DV = H;
                                    DW = AG;
                                }
                                DQ = DV;
                                DR = DW;
                            } else {
                                DQ = AG;
                                DR = H;
                            }
                            DG = DM;
                            DH = DD;
                            DI = DN;
                            DJ = DQ;
                            DK = DE;
                            DL = DR;
                        }
                        CV = DG;
                        CW = DH;
                        CX = DI;
                        CY = DJ;
                        CZ = DK;
                        DA = DL;
                    }
                    BH = CV;
                    BI = CW;
                    BJ = CX;
                    BK = CY;
                    BL = CZ;
                    BM = DA;
                }
                V = BH;
                W = BI;
                X = BJ;
                Y = BK;
                Z = BL;
                AA = BM;
            }
            let AB = if V < parameters[18] { 1.0 } else { 0.0 };
            let DX = if V > parameters[19] { 1.0 } else { 0.0 };
            let DY = if W < parameters[20] { 1.0 } else { 0.0 };
            let DZ = if W > parameters[21] { 1.0 } else { 0.0 };
            let EE = if EA != 0.0 {
                let EC = X + EB;
                EC
            } else {
                let ED = V + EB;
                ED
            };
            let EF = if Y > H { 1.0 } else { 0.0 };
            let EH = if (if (if EE <= H { 1.0 } else { 0.0 }) != 0.0 && EF != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EG != 0.0 { 1.0 } else { 0.0 };
            let EI = if X > H { 1.0 } else { 0.0 };
            let EL;
            let EM;
            if EI != 0.0 {
                let ET;
                let EU;
                if R != 0.0 {
                    let EP = EJ + (EO / X);
                    let ER = EK + (EQ / X);
                    ET = EP;
                    EU = ER;
                } else {
                    let ES = if parameters[3] != 0.0 || parameters[4] != 0.0 { 1.0 } else { 0.0 };
                    let EY;
                    let EZ;
                    if ES != 0.0 {
                        let EW = EJ + ((EV * EO) / X);
                        let EX = EK + ((EV * EQ) / X);
                        EY = EW;
                        EZ = EX;
                    } else {
                        EY = EJ;
                        EZ = EK;
                    }
                    ET = EY;
                    EU = EZ;
                }
                EL = ET;
                EM = EU;
            } else {
                EL = EJ;
                EM = EK;
            }
            let EN = if Z > H { 1.0 } else { 0.0 };
            let FC;
            let FD;
            if EN != 0.0 {
                let FA = EL + (parameters[41] / Z);
                let FB = EM + (parameters[42] / Z);
                FC = FA;
                FD = FB;
            } else {
                FC = EL;
                FD = EM;
            }
            let FE = if Y > (O / multiplicity) { 1.0 } else { 0.0 };
            let FI;
            if R != 0.0 {
                let FG = FF * (V + W);
                FI = FG;
            } else {
                let FO = if FH != 0.0 {
                    let FM = (FF * V) + W;
                    FM
                } else {
                    let FN = FF * V;
                    FN
                };
                FI = FO;
            }
            let FJ = V * W;
            let FK = (parameters[44] + (parameters[45] * FI)) + (parameters[46] * FJ);
            let FL = (parameters[47] + (parameters[48] * FI)) + (parameters[49] * FJ);
            let FS = FR * FP;
            let FT = staged[2] + (FP * FQ);
            let FU = if FT < staged[3] { 1.0 } else { 0.0 };
            let GB;
            let GC;
            if FU != 0.0 {
                let FW = ((FT - FV) - C).exp();
                let FX = FS * FW;
                let FY = FV + FW;
                GB = FY;
                GC = FX;
            } else {
                let GA = if FT > (FZ - C) { 1.0 } else { 0.0 };
                let GM;
                let GN;
                if GA != 0.0 {
                    let GJ = ((FZ - FT) - C).exp();
                    let GK = FZ - GJ;
                    let GL = ((FS * GI) * GJ) * GI;
                    GM = GK;
                    GN = GL;
                } else {
                    GM = FT;
                    GN = FS;
                }
                GB = GM;
                GC = GN;
            }
            let GD = (GB + 2.7315e2f64) - staged[4];
            let GE = FC + (GD * FD);
            let GF = (GC * GE) + ((GC * FD) * GD);
            let GG = C + (GD * GE);
            let GH = if GG < 1.1e-1f64 { 1.0 } else { 0.0 };
            let GT;
            let GU;
            if GH != 0.0 {
                let GP = ((GO * (GG - I)) - C).exp();
                let GR = ((GF * GO) * GP) * GQ;
                let GS = I + (GQ * GP);
                GT = GS;
                GU = GR;
            } else {
                GT = GG;
                GU = GF;
            }
            let GV = Y * GT;
            let GW = GU * Y;
            let GX = ((GU * (AA / GT)) * GI) / GT;
            let GY = if ((C + (GD * parameters[43])) * parameters[30]) < H { 1.0 } else { 0.0 };
            let GZ = node_potentials[0] - node_potentials[1];
            let HC = Lanes([HA, 0.0]) - Lanes([0.0, HB]);
            let HD = if EF != 0.0 && EG != 0.0 { 1.0 } else { 0.0 };
            let HY;
            let HZ;
            if HD != 0.0 {
                let HE = GZ / EE;
                let HF = HC / EE;
                let HH = HG * HE;
                let HI = (HF * HG) * HH;
                let HJ = (C + (HH * HH)).sqrt();
                let HN = HM * (HE.abs());
                let HO = (HF * ((HK * (if HE >= 0e0f64 { 1.0 } else { 0.0 })) - HL)) * HM;
                let HP = HN * HN;
                let HQ = HO * HN;
                let HR = C + (HP * HN);
                let HV = (((C - HT) - HU) + (HT * HJ)) + (HU * (HR.powf(HS)));
                let HW = (((HI + HI) * (HL / (HK * HJ))) * HT) + (((((HQ + HQ) * HN) + (HO * HP)) * (HS * (HR.powf(-6.666666666666667e-1f64)))) * HU);
                HY = HV;
                HZ = HW;
            } else {
                HY = C;
                HZ = HX;
            }
            let IA = GV * HY;
            let IB = HZ * GV;
            let IC = GZ / IA;
            let ID = (Lanes([HC[0], HC[1], 0.0]) - ((Lanes([0.0, 0.0, (GW * HY)]) + Lanes([IB[0], IB[1], 0.0])) * IC)) / IA;
            let IE = -GZ;
            let IF = IE * IC;
            let IG = (HC * GI) * IC;
            let IH = Lanes([IG[0], IG[1], 0.0]) + (ID * IE);
            let II = FQ * FK;
            let IJ = FR * FK;
            if EN != 0.0 {
                let IK = if ((IC / Z).abs()) > parameters[34] { 1.0 } else { 0.0 };
            } else {
            }
            let IL = FQ * FL;
            let IM = FR * FL;
            let IR;
            let IS;
            let IT;
            let IU;
            let IV;
            let IW;
            if FP != 0.0 {
                IR = II;
                IS = IF;
                IT = H;
                IU = IJ;
                IV = IH;
                IW = IN;
            } else {
                let IO = P * FQ;
                let IP = FR * P;
                IR = H;
                IS = H;
                IT = IO;
                IU = IN;
                IV = IQ;
                IW = IP;
            }
            let JA;
            let JB;
            let JC;
            let JD;
            if FP != 0.0 {
                let IX = ddt(2166, IL);
                let IZ = IM * IY;
                JA = IX;
                JB = IL;
                JC = IZ;
                JD = IM;
            } else {
                JA = H;
                JB = H;
                JC = IN;
                JD = IN;
            }
            let JE = if AA > H { 1.0 } else { 0.0 };
            let JF = if (if parameters[6] != 0.0 && EF != 0.0 { 1.0 } else { 0.0 }) != 0.0 && JE != 0.0 { 1.0 } else { 0.0 };
            if JF != 0.0 {
                let JG = if (if parameters[33] != 0.0 && EI != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EN != 0.0 { 1.0 } else { 0.0 };
                if JG != 0.0 {
                } else {
                    let JI = if (if V > H { 1.0 } else { 0.0 }) != 0.0 && (if W > H { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                }
                let JJ = if IC < H { 1.0 } else { 0.0 };
            } else {
            }
            let JH = if EF != 0.0 && JE != 0.0 { 1.0 } else { 0.0 };
            if JH != 0.0 {
                let JK = ID[0];
                let JL = (GZ * GX) / (FK * HY);
                let JM = C - (GZ * JL);
                let JN = if JM != H { 1.0 } else { 0.0 };
                let JZ = if JN != 0.0 {
                    let JY = (JK + (IC * JL)) / JM;
                    JY
                } else {
                    AG
                };
                let KA = if JZ != H { 1.0 } else { 0.0 };
            } else {
            }
            let JO = ID[0];
            let JP = ID[1];
            let JQ = ID[2];
            let JR = IU;
            let JS = IV[0];
            let JT = IV[1];
            let JU = IV[2];
            let JV = IW;
            let JW = JC;
            let JX = JD;
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(1),
            multiplicity * (IC),
            [0, 1, 2],
            [JO, JP, JQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (IR),
            [2],
            [JR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            None,
            multiplicity * (IS),
            [0, 1, 2],
            [JS, JT, JU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (IT),
            [2],
            [JV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (JA),
            [2],
            [JW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (KB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (KC),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = IC;
        self.canonical_reactive[1] = IR;
        self.canonical_reactive[2] = IS;
        self.canonical_reactive[3] = IT;
        self.canonical_reactive[4] = JB;
        self.canonical_reactive[5] = JX;
        self.canonical_reactive[6] = KB;
        self.canonical_reactive[7] = KC;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[5]],
            &[],
            &[],
            multiplicity,
        );
    }

}
