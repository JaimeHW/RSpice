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
        let mut key = Vec::with_capacity(98);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[0] = values[0];
        self.canonical_staged[1] = values[1];
        self.canonical_staged[2] = values[2];
        self.canonical_staged[3] = values[3];
        self.canonical_staged[4] = values[4];
        self.canonical_staged[16] = values[5];
        self.canonical_staged[5] = values[6];
        self.canonical_staged[17] = values[7];
        self.canonical_staged[18] = values[8];
        self.canonical_staged[19] = values[9];
        self.canonical_staged[21] = values[10];
        self.canonical_staged[6] = values[11];
        self.canonical_staged[7] = values[12];
        self.canonical_staged[23] = values[13];
        self.canonical_staged[24] = values[14];
        self.canonical_staged[25] = values[15];
        self.canonical_staged[26] = values[16];
        self.canonical_staged[27] = values[17];
        self.canonical_staged[28] = values[18];
        self.canonical_staged[9] = values[19];
        self.canonical_staged[10] = values[20];
        self.canonical_staged[11] = values[21];
        self.canonical_staged[12] = values[22];
        self.canonical_staged[13] = values[23];
        self.canonical_staged[14] = values[24];
        self.canonical_staged[15] = values[25];
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
                let B = 3.0015e2f64;
                let E = parameters[49];
                let F = 1e0f64;
                let H = parameters[51];
                let J = 1e0f64;
                let O = parameters[31];
                let Q = parameters[39];
                let W = 0e0f64;
                let X = 0e0f64;
                let Z = parameters[30];
                let AC = 0e0f64;
                let AO = 0e0f64;
                let AP = 0e0f64;
                let AQ = 0e0f64;
                let mut oV = 0.0;
                let mut oAD = 0.0;
                let mut oAK = 0.0;
                let A = parameters[25] + 2.7315e2f64;
                let C = A / B;
                let D = 4e-4f64 * (A - B);
                let G = E - F;
                let I = H - F;
                let K = J / E;
                let L = K - F;
                let M = J / H;
                let N = M - F;
                let P = if O == J { 1.0 } else { 0.0 };
                let R = Q - F;
                let S = J / Q;
                let T = S - F;
                let U = if parameters[32] == J { 1.0 } else { 0.0 };
                let Y = if U != 0.0 {
                    let V = parameters[44] - F;
                    oV = V;
                    W
                } else {
                    X
                };
                let AA = if parameters[33] > W { 1.0 } else { 0.0 };
                let AB = if (if Z == J { 1.0 } else { 0.0 }) != 0.0 && AA != 0.0 { 1.0 } else { 0.0 };
                let AE;
                let AF;
                let AG;
                let AH;
                if AB != 0.0 {
                    AE = AC;
                    AF = W;
                    AG = W;
                    AH = W;
                } else {
                    let AD = if (if (if Z == 2e0f64 { 1.0 } else { 0.0 }) != 0.0 && AA != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[35] > W { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAD = AD;
                    let AL;
                    let AM;
                    let AN;
                    if AD != 0.0 {
                        AL = W;
                        AM = W;
                        AN = W;
                    } else {
                        let AK = if Z == -1e0f64 { 1.0 } else { 0.0 };
                        oAK = AK;
                        let AR;
                        let AS;
                        let AT;
                        if AK != 0.0 {
                            AR = AO;
                            AS = W;
                            AT = W;
                        } else {
                            AR = W;
                            AS = AP;
                            AT = AQ;
                        }
                        AL = AR;
                        AM = AS;
                        AN = AT;
                    }
                    AE = W;
                    AF = AL;
                    AG = AM;
                    AH = AN;
                }
                let AI = parameters[12] + (O * parameters[13]);
                let AJ = parameters[14] + (O * parameters[15]);
                let AU = if (if (if parameters[28] > W { 1.0 } else { 0.0 }) != 0.0 && (if parameters[27] > W { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > W { 1.0 } else { 0.0 };
            [A, C, D, K, M, P, S, U, AB, oAD, oAK, AI, AJ, AU, Y, AE, AF, AG, AH, G, I, L, N, R, T, oV]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 7] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let D = 0e0f64;
                let E = parameters[46];
                let G = 0e0f64;
                let K = 0e0f64;
                let L = 0e0f64;
                let O = 0e0f64;
                let A = parameters[43] * parameters[42];
                let B = staged[6] / A;
                let C = staged[7] / A;
                let F = if (if B > D { 1.0 } else { 0.0 }) != 0.0 && (if B >= E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let H;
                let I;
                if F != 0.0 {
                    H = K;
                    I = D;
                } else {
                    H = D;
                    I = G;
                }
                let J = if (if C > D { 1.0 } else { 0.0 }) != 0.0 && (if C >= E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let M;
                let N;
                if J != 0.0 {
                    M = O;
                    N = D;
                } else {
                    M = D;
                    N = L;
                }
            [A, F, J, H, I, M, N]
        };
        self.canonical_staged[8] = produced[0];
        self.canonical_staged[20] = produced[1];
        self.canonical_staged[22] = produced[2];
        self.canonical_staged[29] = produced[3];
        self.canonical_staged[30] = produced[4];
        self.canonical_staged[31] = produced[5];
        self.canonical_staged[32] = produced[6];
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
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
            [0.0]
        };
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 1504 => 0usize, 1615 => 1usize, 1641 => 2usize, 1648 => 3usize, 1767 => 4usize, 1773 => 5usize, _ => usize::MAX };
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
            let A = node_potentials[2];
            let D = 1.7314999999999998e2f64;
            let G = 1.3e3f64;
            let H = 0e0f64;
            let L = 1e0f64;
            let M = 1.7314999999999998e2f64;
            let Q = 8.6170869e-5f64;
            let T = staged[0];
            let X = 1e0f64;
            let Z = parameters[22];
            let AA = 1e0f64;
            let AC = parameters[21];
            let AE = parameters[23];
            let AG = parameters[0];
            let AK = parameters[2];
            let AN = parameters[7];
            let AO = parameters[47];
            let AR = parameters[6];
            let AS = parameters[5];
            let AV = parameters[10];
            let AW = parameters[9];
            let AZ = 3.0015e2f64;
            let BC = 7.02e-4f64;
            let BG = -1e0f64;
            let BH = 1.3806226e-23f64;
            let BL = 1.5e0f64;
            let BM = 1.6021918e-19f64;
            let BQ = parameters[17];
            let BR = staged[1];
            let BV = parameters[18];
            let CB = 4e-4f64;
            let CF = node_potentials[3];
            let CG = node_potentials[4];
            let CI = 1e0f64;
            let CJ = 1e0f64;
            let CL = parameters[29];
            let CO = node_potentials[0];
            let CQ = 1e0f64;
            let CU = node_potentials[1];
            let CW = 1e0f64;
            let DA = 0e0f64;
            let DC = parameters[1];
            let DH = parameters[11];
            let DO = 8e1f64;
            let DQ = Lanes([0e0f64; 3]);
            let EC = 3.7e1f64;
            let EW = 0e0f64;
            let EX = 2e0f64;
            let FA = parameters[8];
            let FN = parameters[4];
            let FP = 1e-3f64;
            let FR = -1e0f64;
            let FT = parameters[3];
            let GE = parameters[48];
            let GH = parameters[49];
            let GJ = parameters[50];
            let GM = parameters[51];
            let GO = parameters[37];
            let GQ = parameters[12];
            let GS = staged[3];
            let GX = parameters[38];
            let GZ = parameters[14];
            let HB = staged[4];
            let HG = staged[16];
            let HY = parameters[40];
            let IB = parameters[39];
            let ID = staged[5];
            let IE = parameters[41];
            let IF = parameters[19];
            let IM = staged[17];
            let IS = node_potentials[6];
            let IU = 1e0f64;
            let IV = ddt_scale();
            let JC = parameters[20];
            let JE = parameters[44];
            let JJ = Lanes([0e0f64; 5]);
            let JK = 0e0f64;
            let JL = Lanes([0e0f64; 3]);
            let JW = parameters[24];
            let LA = staged[18];
            let LE = -1e0f64;
            let LH = parameters[33];
            let LK = parameters[34];
            let LP = Lanes([0e0f64; 2]);
            let LQ = 0e0f64;
            let LR = staged[19];
            let MT = staged[20];
            let MX = -1e0f64;
            let NA = node_potentials[5];
            let NB = 1e0f64;
            let NI = parameters[35];
            let NL = parameters[36];
            let NQ = staged[21];
            let OK = -1e0f64;
            let OP = staged[8];
            let OS = parameters[46];
            let OU = Lanes([0e0f64; 4]);
            let OX = staged[22];
            let PG = Lanes([0e0f64; 3]);
            let SN = 0e0f64;
            let SO = 0e0f64;
            let B = (temperature + A) + parameters[45];
            let C = if B > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
            let E = if C != 0.0 {
                B
            } else {
                D
            };
            let F = if 1.3e3f64 < E { 1.0 } else { 0.0 };
            let J;
            let K;
            if F != 0.0 {
                J = G;
                K = H;
            } else {
                let I = if B > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let N;
                let O;
                if I != 0.0 {
                    N = B;
                    O = L;
                } else {
                    N = M;
                    O = H;
                }
                J = N;
                K = O;
            }
            let P = if J > parameters[26] { 1.0 } else { 0.0 };
            let R = Q * J;
            let S = K * Q;
            let U = J / T;
            let V = K / T;
            let W = U.ln();
            let Y = V * (X / U);
            let AB = U - AA;
            let AD = (AC * AB) / R;
            let AF = ((Z * W) + AD).exp();
            let AH = AG * AF;
            let AI = (((Y * Z) + (((V * AC) - (S * AD)) / R)) * AF) * AG;
            let AJ = (AE * W).exp();
            let AL = AK * AJ;
            let AM = ((Y * AE) * AJ) * AK;
            let AP = AO * (AA + (AN * AB));
            let AQ = (V * AN) * AO;
            let AT = AS * (AA + (AR * AB));
            let AU = (V * AR) * AS;
            let AX = AW * (AA + (AV * AB));
            let AY = (V * AV) * AW;
            let BA = J / AZ;
            let BB = K / AZ;
            let BD = BC * J;
            let BE = 1.108e3f64 + J;
            let BF = (BD * J) / BE;
            let BI = BH * (J + J);
            let BJ = (-(1.16e0f64 - BF)) / BI;
            let BK = -(R + R);
            let BN = (BL * (BA.ln())) + (BM * (BJ + 1.3454442398941469e20f64));
            let BO = BK * BN;
            let BP = (((S + S) * BG) * BN) + ((((BB * (X / BA)) * BL) + ((((((((((K * BC) * J) + (K * BD)) - (K * BF)) / BE) * BG) * BG) - (((K + K) * BH) * BJ)) / BI) * BM)) * BK);
            let BS = (BQ - BO) / BR;
            let BT = (BP * BG) / BR;
            let BU = (BQ - BS) / BS;
            let BW = AA + (BV * (staged[2] - BU));
            let BX = parameters[16] / BW;
            let BY = (BA * BS) + BO;
            let BZ = ((BB * BS) + (BT * BA)) + BP;
            let CA = (BY - BS) / BS;
            let CC = AA + (BV * ((CB * (J - AZ)) - CA));
            let CD = BX * CC;
            let CE = (((((((((BT * BG) - (BT * BU)) / BS) * BG) * BV) * BX) * BG) / BW) * CC) + ((((K * CB) - (((BZ - BT) - (BT * CA)) / BS)) * BV) * BX);
            let CH = CF - CG;
            let CK = Lanes([CI, 0.0]) - Lanes([0.0, CJ]);
            let CM = CL * CH;
            let CN = CK * CL;
            let CP = CO - CF;
            let CR = Lanes([CQ, 0.0]) - Lanes([0.0, CI]);
            let CS = CL * CP;
            let CT = CR * CL;
            let CV = CU - CG;
            let CX = Lanes([CW, 0.0]) - Lanes([0.0, CJ]);
            let CY = CL * CV;
            let CZ = CX * CL;
            let DB = if AH > DA { 1.0 } else { 0.0 };
            let DR;
            let DS;
            if DB != 0.0 {
                let DD = DC * R;
                let DE = CM / DD;
                let DF = (Lanes([0.0, CN[0], CN[1]]) - Lanes([((S * DC) * DE), 0.0, 0.0])) / DD;
                let DG = CN * BG;
                let DI = DH * R;
                let DJ = S * DH;
                let DK = ((-CM) - AT) / DI;
                let DL = ((Lanes([0.0, DG[0], DG[1]]) - Lanes([AU, 0.0, 0.0])) - Lanes([(DJ * DK), 0.0, 0.0])) / DI;
                let DM = (-AT) / DI;
                let DN = ((AU * BG) - (DJ * DM)) / DI;
                let DP = if DE > DO { 1.0 } else { 0.0 };
                let DV;
                let DW;
                let DX;
                let DY;
                if DP != 0.0 {
                    let DU = AA + (DE - DO);
                    DV = DU;
                    DW = DO;
                    DX = DF;
                    DY = DQ;
                } else {
                    DV = AA;
                    DW = DE;
                    DX = DQ;
                    DY = DF;
                }
                let DZ = DW.exp();
                let EA = DV * DZ;
                let EB = (DX * DZ) + ((DY * DZ) * DV);
                let ED = if DK >= EC { 1.0 } else { 0.0 };
                let EF;
                let EG;
                if ED != 0.0 {
                    EF = DK;
                    EG = DL;
                } else {
                    let EE = if DK <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let EO;
                    let EP;
                    if EE != 0.0 {
                        let EI = DK.exp();
                        let EJ = DL * EI;
                        EO = EI;
                        EP = EJ;
                    } else {
                        let EK = DK.exp();
                        let EL = EK + AA;
                        let EM = EL.ln();
                        let EN = (DL * EK) * (X / EL);
                        EO = EM;
                        EP = EN;
                    }
                    EF = EO;
                    EG = EP;
                }
                let EH = if DM >= EC { 1.0 } else { 0.0 };
                let ER;
                let ES;
                if EH != 0.0 {
                    ER = DM;
                    ES = DN;
                } else {
                    let EQ = if DM <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let FL;
                    let FM;
                    if EQ != 0.0 {
                        let FF = DM.exp();
                        let FG = DN * FF;
                        FL = FF;
                        FM = FG;
                    } else {
                        let FH = DM.exp();
                        let FI = FH + AA;
                        let FJ = FI.ln();
                        let FK = (DN * FH) * (X / FI);
                        FL = FJ;
                        FM = FK;
                    }
                    ER = FL;
                    ES = FM;
                }
                let ET = EF - ER;
                let EU = EA - AA;
                let EV = CM.abs();
                let EY = EV.powf(AX);
                let EZ = (CN * ((EX * (if CM >= EW { 1.0 } else { 0.0 })) - X)) * (AX * (EV.powf((AX - X))));
                let FB = AA + (FA * EY);
                let FC = (AP * ET) / FB;
                let FD = (AH * EU) - FC;
                let FE = (Lanes([(AI * EU), 0.0, 0.0]) + (EB * AH)) - (((Lanes([(AQ * ET), 0.0, 0.0]) + ((EG - Lanes([ES, 0.0, 0.0])) * AP)) - (((Lanes([0.0, EZ[0], EZ[1]]) + Lanes([(AY * (EY * (EV.ln()))), 0.0, 0.0])) * FA) * FC)) / FB);
                DR = FD;
                DS = FE;
            } else {
                DR = DA;
                DS = DQ;
            }
            let DT = if AL > DA { 1.0 } else { 0.0 };
            let GA;
            let GB;
            if DT != 0.0 {
                let FO = FN - CM;
                let FQ = if FO >= FP { FO } else { FP };
                let FS = (CN * FR) * FN;
                let FU = FT * R;
                let FV = FU * FQ;
                let FW = ((CN * BG) * (if FO >= FP { 1.0 } else { 0.0 })) * FU;
                let FX = ((FR * CM) * FN) / FV;
                let FY = (Lanes([0.0, FS[0], FS[1]]) - ((Lanes([((S * FT) * FQ), 0.0, 0.0]) + Lanes([0.0, FW[0], FW[1]])) * FX)) / FV;
                let FZ = if FX > DO { 1.0 } else { 0.0 };
                let HI;
                let HJ;
                let HK;
                let HL;
                if FZ != 0.0 {
                    let HH = AA + (FX - DO);
                    HI = HH;
                    HJ = DO;
                    HK = FY;
                    HL = DQ;
                } else {
                    HI = AA;
                    HJ = FX;
                    HK = DQ;
                    HL = FY;
                }
                let HM = HJ.exp();
                let HN = (HI * HM) - AA;
                let HO = AL * HN;
                let HP = Lanes([(AM * HN), 0.0, 0.0]) + (((HK * HM) + ((HL * HM) * HI)) * AL);
                GA = HO;
                GB = HP;
            } else {
                GA = DA;
                GB = DQ;
            }
            let GC = DR - GA;
            let GD = DS - GB;
            let GF = CS / GE;
            let GG = GF.abs();
            let GI = AA + (GG.powf(GH));
            let GK = CY / GJ;
            let GL = GK.abs();
            let GN = AA + (GL.powf(GM));
            let GP = (W * GO).exp();
            let GR = GQ * GP;
            let GT = GI.powf(GS);
            let GU = GR * GT;
            let GV = ((((CT / GE) * ((EX * (if GF >= EW { 1.0 } else { 0.0 })) - X)) * (GH * (GG.powf(staged[9])))) * (GS * (GI.powf(staged[11])))) * GR;
            let GW = Lanes([0.0, ((((Y * GO) * GP) * GQ) * GT), 0.0]) + Lanes([GV[0], 0.0, GV[1]]);
            let GY = (W * GX).exp();
            let HA = GZ * GY;
            let HC = GN.powf(HB);
            let HD = HA * HC;
            let HE = ((((CZ / GJ) * ((EX * (if GK >= EW { 1.0 } else { 0.0 })) - X)) * (GM * (GL.powf(staged[10])))) * (HB * (GN.powf(staged[12])))) * HA;
            let HF = Lanes([0.0, ((((Y * GX) * GY) * GZ) * HC), 0.0]) + Lanes([HE[0], 0.0, HE[1]]);
            let HS;
            let HT;
            let HU;
            let HV;
            if HG != 0.0 {
                let HQ = GU + parameters[13];
                let HR = HD + parameters[15];
                HS = HQ;
                HT = HR;
                HU = GW;
                HV = HF;
            } else {
                HS = GU;
                HT = HD;
                HU = GW;
                HV = HF;
            }
            let HW = CO - CU;
            let HX = Lanes([CQ, 0.0]) - Lanes([0.0, CW]);
            let HZ = HW / HY;
            let IA = HZ.abs();
            let IC = AA + (IA.powf(IB));
            let IG = IF * (AA + (IE * ((IC.powf(ID)) - AA)));
            let IH = (((((HX / HY) * ((EX * (if HZ >= EW { 1.0 } else { 0.0 })) - X)) * (IB * (IA.powf(staged[13])))) * (ID * (IC.powf(staged[14])))) * IE) * IF;
            let II = IG * DR;
            let IJ = IH * DR;
            let IK = DS * IG;
            let IL = Lanes([IJ[0], IJ[1], 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, IK[0], IK[1], IK[2]]);
            let JM;
            let JN;
            let JO;
            let JP;
            let JQ;
            let JR;
            let JS;
            let JT;
            let JU;
            let JV;
            if IM != 0.0 {
                let IN = -DR;
                let IO = IN * IG;
                let IP = (DS * BG) * IG;
                let IQ = IH * IN;
                let IR = Lanes([0.0, 0.0, IP[0], IP[1], IP[2]]) + Lanes([IQ[0], IQ[1], 0.0, 0.0, 0.0]);
                let IT = ddt(1504, IS);
                let IW = IG * IT;
                let IX = IH * IT;
                let IY = Lanes([IX[0], IX[1], 0.0]) + Lanes([0.0, 0.0, ((IU * IV) * IG)]);
                let IZ = IG * IS;
                let JA = IH * IS;
                let JB = Lanes([JA[0], JA[1], 0.0]) + Lanes([0.0, 0.0, (IU * IG)]);
                let JD = (IS.abs()) / JC;
                let JF = AA + (JD.powf(JE));
                let JG = HS / JF;
                let JH = (Lanes([HU[0], HU[1], HU[2], 0.0]) - Lanes([0.0, 0.0, 0.0, ((((IU * ((EX * (if IS >= EW { 1.0 } else { 0.0 })) - X)) / JC) * (JE * (JD.powf(staged[15])))) * JG)])) / JF;
                JM = JG;
                JN = IO;
                JO = IS;
                JP = IW;
                JQ = IZ;
                JR = JH;
                JS = IR;
                JT = IU;
                JU = IY;
                JV = JB;
            } else {
                let JI = Lanes([HU[0], HU[1], HU[2], 0.0]);
                JM = HS;
                JN = DA;
                JO = DA;
                JP = DA;
                JQ = DA;
                JR = JI;
                JS = JJ;
                JT = JK;
                JU = JL;
                JV = JL;
            }
            let JX = CM + ((-BY) * JW);
            let JY = Lanes([0.0, CN[0], CN[1]]);
            let JZ = JY + Lanes([((BZ * BG) * JW), 0.0, 0.0]);
            let KA = if JX > DA { 1.0 } else { 0.0 };
            let KT;
            let KU;
            let KV;
            let KW;
            if KA != 0.0 {
                let KB = AA - JW;
                let KC = ((-1e0f64 - BV) * (KB.ln())).exp();
                let KD = AA - ((KC * KB) * KB);
                let KE = AA - BV;
                let KF = (BY * KD) / KE;
                let KG = 5e-1f64 * BV;
                let KH = (KG * JX) / BY;
                let KI = KB + KH;
                let KJ = (JX * KI) * KC;
                let KK = ((JZ * KI) + ((((JZ * KG) - Lanes([(BZ * KH), 0.0, 0.0])) / BY) * JX)) * KC;
                let KL = Lanes([((BZ * KD) / KE), 0.0, 0.0]);
                KT = KF;
                KU = KJ;
                KV = KL;
                KW = KK;
            } else {
                let KM = AA - BV;
                let KN = CM / BY;
                let KO = AA - KN;
                let KP = (KM * (KO.ln())).exp();
                let KQ = AA - KP;
                let KR = (BY * KQ) / KM;
                let KS = (Lanes([(BZ * KQ), 0.0, 0.0]) + ((((((((JY - Lanes([(BZ * KN), 0.0, 0.0])) / BY) * BG) * (X / KO)) * KM) * KP) * BG) * BY)) / KM;
                KT = KR;
                KU = DA;
                KV = KS;
                KW = DQ;
            }
            let KX = KT + KU;
            let KY = CD * KX;
            let KZ = Lanes([(CE * KX), 0.0, 0.0]) + ((KV + KW) * CD);
            let LS;
            let LT;
            let LU;
            let LV;
            let LW;
            let LX;
            let LY;
            let LZ;
            let MA;
            let MB;
            let MC;
            let MD;
            let ME;
            let MF;
            let MG;
            let MH;
            let MI;
            let MJ;
            let MK;
            let ML;
            let MM;
            let MN;
            let MO;
            let MP;
            if LA != 0.0 {
                let LB = GC * HW;
                let LC = GD * HW;
                let LD = HX * GC;
                let LF = LE * (LB.abs());
                let LG = ((Lanes([0.0, 0.0, LC[0], LC[1], LC[2]]) + Lanes([LD[0], LD[1], 0.0, 0.0, 0.0])) * ((EX * (if LB >= EW { 1.0 } else { 0.0 })) - X)) * LE;
                let LI = A / LH;
                let LJ = L / LH;
                let LL = A * LK;
                let LM = L * LK;
                let LN = ddt(1615, LL);
                let LO = LM * IV;
                LS = LF;
                LT = LI;
                LU = LN;
                LV = DA;
                LW = DA;
                LX = DA;
                LY = DA;
                LZ = DA;
                MA = DA;
                MB = LL;
                MC = DA;
                MD = DA;
                ME = LG;
                MF = LJ;
                MG = LO;
                MH = JJ;
                MI = LP;
                MJ = H;
                MK = LQ;
                ML = LQ;
                MM = JJ;
                MN = LM;
                MO = H;
                MP = LQ;
            } else {
                let NR;
                let NS;
                let NT;
                let NU;
                let NV;
                let NW;
                let NX;
                let NY;
                let NZ;
                let OA;
                let OB;
                let OC;
                let OD;
                let OE;
                let OF;
                let OG;
                if LR != 0.0 {
                    let MU = GC * HW;
                    let MV = GD * HW;
                    let MW = HX * GC;
                    let MY = MX * (MU.abs());
                    let MZ = ((Lanes([0.0, 0.0, MV[0], MV[1], MV[2]]) + Lanes([MW[0], MW[1], 0.0, 0.0, 0.0])) * ((EX * (if MU >= EW { 1.0 } else { 0.0 })) - X)) * MX;
                    let NC = (A - NA) / LH;
                    let ND = (Lanes([L, 0.0]) - Lanes([0.0, NB])) / LH;
                    let NE = LK * A;
                    let NF = L * LK;
                    let NG = ddt(1641, NE);
                    let NH = NF * IV;
                    let NJ = NA / NI;
                    let NK = NB / NI;
                    let NM = NL * NA;
                    let NN = NB * NL;
                    let NO = ddt(1648, NM);
                    let NP = NN * IV;
                    NR = MY;
                    NS = NC;
                    NT = NG;
                    NU = NJ;
                    NV = NO;
                    NW = DA;
                    NX = NE;
                    NY = NM;
                    NZ = MZ;
                    OA = ND;
                    OB = NH;
                    OC = NK;
                    OD = NP;
                    OE = JJ;
                    OF = NF;
                    OG = NN;
                } else {
                    let ON;
                    let OO;
                    if NQ != 0.0 {
                        let OH = GC * HW;
                        let OI = GD * HW;
                        let OJ = HX * GC;
                        let OL = OK * (OH.abs());
                        let OM = ((Lanes([0.0, 0.0, OI[0], OI[1], OI[2]]) + Lanes([OJ[0], OJ[1], 0.0, 0.0, 0.0])) * ((EX * (if OH >= EW { 1.0 } else { 0.0 })) - X)) * OK;
                        ON = OL;
                        OO = OM;
                    } else {
                        ON = DA;
                        OO = JJ;
                    }
                    NR = DA;
                    NS = DA;
                    NT = DA;
                    NU = DA;
                    NV = DA;
                    NW = ON;
                    NX = DA;
                    NY = DA;
                    NZ = JJ;
                    OA = LP;
                    OB = H;
                    OC = LQ;
                    OD = LQ;
                    OE = OO;
                    OF = H;
                    OG = LQ;
                }
                LS = DA;
                LT = DA;
                LU = DA;
                LV = NR;
                LW = NS;
                LX = NT;
                LY = NU;
                LZ = NV;
                MA = NW;
                MB = DA;
                MC = NX;
                MD = NY;
                ME = JJ;
                MF = H;
                MG = H;
                MH = NZ;
                MI = OA;
                MJ = OB;
                MK = OC;
                ML = OD;
                MM = OE;
                MN = H;
                MO = OF;
                MP = OG;
            }
            let MQ = ctx.simparam_or("gmin", DA);
            let MR = MQ * CH;
            let MS = CK * MQ;
            let OV;
            let OW;
            if MT != 0.0 {
                let OQ = JM / OP;
                let OR = JR / OP;
                let OT = if OQ > OS { 1.0 } else { 0.0 };
                let OY;
                let OZ;
                if OT != 0.0 {
                    OY = OQ;
                    OZ = OR;
                } else {
                    OY = OS;
                    OZ = OU;
                }
                let PA = CP / OY;
                let PB = (Lanes([CR[0], 0.0, CR[1], 0.0]) - (OZ * PA)) / OY;
                let PC = if OQ >= OS { 1.0 } else { 0.0 };
                OV = PA;
                OW = PB;
            } else {
                OV = DA;
                OW = OU;
            }
            let PH;
            let PI;
            if OX != 0.0 {
                let PD = HT / OP;
                let PE = HV / OP;
                let PF = if PD > OS { 1.0 } else { 0.0 };
                let PU;
                let PV;
                if PF != 0.0 {
                    PU = PD;
                    PV = PE;
                } else {
                    PU = OS;
                    PV = PG;
                }
                let PW = CV / PU;
                let PX = (Lanes([CX[0], 0.0, CX[1]]) - (PV * PW)) / PU;
                let PY = if PD >= OS { 1.0 } else { 0.0 };
                PH = PW;
                PI = PX;
            } else {
                PH = DA;
                PI = PG;
            }
            let PJ = CL * GC;
            let PK = PJ * OP;
            let PL = (GD * CL) * OP;
            let PM = (CL * KY) * OP;
            let PN = (KZ * CL) * OP;
            let PO = ddt(1767, PM);
            let PP = PN * IV;
            let PQ = (CL * II) * OP;
            let PR = (IL * CL) * OP;
            let PS = ddt(1773, PQ);
            let PT = PR * IV;
            let PZ = if PJ >= DA { 1.0 } else { 0.0 };
            let QA = JS[0];
            let QB = JS[1];
            let QC = JS[2];
            let QD = JS[3];
            let QE = JS[4];
            let QF = JT;
            let QG = JU[0];
            let QH = JU[1];
            let QI = JU[2];
            let QJ = ME[0];
            let QK = ME[1];
            let QL = ME[2];
            let QM = ME[3];
            let QN = ME[4];
            let QO = MF;
            let QP = MG;
            let QQ = MH[0];
            let QR = MH[1];
            let QS = MH[2];
            let QT = MH[3];
            let QU = MH[4];
            let QV = MI[0];
            let QW = MI[1];
            let QX = MJ;
            let QY = MK;
            let QZ = ML;
            let RA = MM[0];
            let RB = MM[1];
            let RC = MM[2];
            let RD = MM[3];
            let RE = MM[4];
            let RF = MS[0];
            let RG = MS[1];
            let RH = OW[0];
            let RI = OW[1];
            let RJ = OW[2];
            let RK = OW[3];
            let RL = PI[0];
            let RM = PI[1];
            let RN = PI[2];
            let RO = PL[0];
            let RP = PL[1];
            let RQ = PL[2];
            let RR = PP[0];
            let RS = PP[1];
            let RT = PP[2];
            let RU = PT[0];
            let RV = PT[1];
            let RW = PT[2];
            let RX = PT[3];
            let RY = PT[4];
            let RZ = JV[0];
            let SA = JV[1];
            let SB = JV[2];
            let SC = MN;
            let SD = MO;
            let SE = MP;
            let SF = PN[0];
            let SG = PN[1];
            let SH = PN[2];
            let SI = PR[0];
            let SJ = PR[1];
            let SK = PR[2];
            let SL = PR[3];
            let SM = PR[4];
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            None,
            multiplicity * (JN),
            [0, 1, 2, 3, 4],
            [QA, QB, QC, QD, QE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (JO),
            [6],
            [QF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            None,
            multiplicity * (JP),
            [0, 1, 6],
            [QG, QH, QI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(6), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[24],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (LS),
            [0, 1, 2, 3, 4],
            [QJ, QK, QL, QM, QN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (LT),
            [2],
            [QO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (LU),
            [2],
            [QP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[25],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (LV),
            [0, 1, 2, 3, 4],
            [QQ, QR, QS, QT, QU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(5),
            multiplicity * (LW),
            [2, 5],
            [QV, QW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(2),
            None,
            multiplicity * (LX),
            [2],
            [QX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            None,
            multiplicity * (LY),
            [5],
            [QY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            None,
            multiplicity * (LZ),
            [5],
            [QZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            None,
            multiplicity * (MA),
            [0, 1, 2, 3, 4],
            [RA, RB, RC, RD, RE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), None, 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[26],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[27],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[28],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(4),
            multiplicity * (MR),
            [3, 4],
            [RF, RG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(0),
            Some(3),
            multiplicity * (OV),
            [0, 2, 3, 6],
            [RH, RI, RJ, RK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(3),
            multiplicity * (staged[29]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(3), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[30],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (PH),
            [1, 2, 4],
            [RL, RM, RN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(4),
            multiplicity * (staged[31]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(4), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[32],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(4),
            multiplicity * (PK),
            [2, 3, 4],
            [RO, RP, RQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(4),
            multiplicity * (PO),
            [2, 3, 4],
            [RR, RS, RT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(4),
            multiplicity * (PS),
            [0, 1, 2, 3, 4],
            [RU, RV, RW, RX, RY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (SN),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(4),
            multiplicity * (SO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = JN;
        self.canonical_reactive[1] = JO;
        self.canonical_reactive[2] = JQ;
        self.canonical_reactive[3] = RZ;
        self.canonical_reactive[4] = SA;
        self.canonical_reactive[5] = SB;
        self.canonical_reactive[6] = staged[24];
        self.canonical_reactive[7] = LS;
        self.canonical_reactive[8] = LT;
        self.canonical_reactive[9] = MB;
        self.canonical_reactive[10] = SC;
        self.canonical_reactive[11] = staged[25];
        self.canonical_reactive[12] = LV;
        self.canonical_reactive[13] = LW;
        self.canonical_reactive[14] = MC;
        self.canonical_reactive[15] = SD;
        self.canonical_reactive[16] = LY;
        self.canonical_reactive[17] = MD;
        self.canonical_reactive[18] = SE;
        self.canonical_reactive[19] = MA;
        self.canonical_reactive[20] = staged[26];
        self.canonical_reactive[21] = staged[27];
        self.canonical_reactive[22] = staged[28];
        self.canonical_reactive[23] = MR;
        self.canonical_reactive[24] = OV;
        self.canonical_reactive[25] = staged[29];
        self.canonical_reactive[26] = staged[30];
        self.canonical_reactive[27] = PH;
        self.canonical_reactive[28] = staged[31];
        self.canonical_reactive[29] = staged[32];
        self.canonical_reactive[30] = PK;
        self.canonical_reactive[31] = PM;
        self.canonical_reactive[32] = SF;
        self.canonical_reactive[33] = SG;
        self.canonical_reactive[34] = SH;
        self.canonical_reactive[35] = PQ;
        self.canonical_reactive[36] = SI;
        self.canonical_reactive[37] = SJ;
        self.canonical_reactive[38] = SK;
        self.canonical_reactive[39] = SL;
        self.canonical_reactive[40] = SM;
        self.canonical_reactive[41] = SN;
        self.canonical_reactive[42] = SO;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            None,
            &[0, 1, 6],
            &[cached[3], cached[4], cached[5]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[10]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            None,
            &[2],
            &[cached[15]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            None,
            &[5],
            &[cached[18]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(4),
            &[2, 3, 4],
            &[cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(4),
            &[0, 1, 2, 3, 4],
            &[cached[36], cached[37], cached[38], cached[39], cached[40]],
            &[],
            &[],
            multiplicity,
        );
    }

}
