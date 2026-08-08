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
        let mut key = Vec::with_capacity(112);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[211] = values[0];
        self.canonical_staged[0] = values[1];
        self.canonical_staged[22] = values[2];
        self.canonical_staged[26] = values[3];
        self.canonical_staged[39] = values[4];
        self.canonical_staged[51] = values[5];
        self.canonical_staged[4] = values[6];
        self.canonical_staged[5] = values[7];
        self.canonical_staged[6] = values[8];
        self.canonical_staged[25] = values[9];
        self.canonical_staged[38] = values[10];
        self.canonical_staged[50] = values[11];
        self.canonical_staged[28] = values[12];
        self.canonical_staged[41] = values[13];
        self.canonical_staged[53] = values[14];
        self.canonical_staged[24] = values[15];
        self.canonical_staged[37] = values[16];
        self.canonical_staged[49] = values[17];
        self.canonical_staged[31] = values[18];
        self.canonical_staged[34] = values[19];
        self.canonical_staged[46] = values[20];
        self.canonical_staged[58] = values[21];
        self.canonical_staged[32] = values[22];
        self.canonical_staged[44] = values[23];
        self.canonical_staged[56] = values[24];
        self.canonical_staged[33] = values[25];
        self.canonical_staged[45] = values[26];
        self.canonical_staged[57] = values[27];
        self.canonical_staged[212] = values[28];
        self.canonical_staged[213] = values[29];
        self.canonical_staged[214] = values[30];
        self.canonical_staged[216] = values[31];
        self.canonical_staged[217] = values[32];
        self.canonical_staged[218] = values[33];
        self.canonical_staged[1] = values[34];
        self.canonical_staged[2] = values[35];
        self.canonical_staged[3] = values[36];
        self.canonical_staged[7] = values[37];
        self.canonical_staged[8] = values[38];
        self.canonical_staged[9] = values[39];
        self.canonical_staged[10] = values[40];
        self.canonical_staged[11] = values[41];
        self.canonical_staged[12] = values[42];
        self.canonical_staged[14] = values[43];
        self.canonical_staged[13] = values[44];
        self.canonical_staged[15] = values[45];
        self.canonical_staged[233] = values[46];
        self.canonical_staged[20] = values[47];
        self.canonical_staged[60] = values[48];
        self.canonical_staged[77] = values[49];
        self.canonical_staged[190] = values[50];
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
                let C = 0e0f64;
                let K = parameters[26];
                let O = parameters[23];
                let T = 1.0447941624768001e-10f64;
                let W = parameters[17];
                let AD = parameters[20];
                let AG = parameters[53];
                let AI = parameters[54];
                let AK = parameters[55];
                let AS = parameters[56];
                let AT = parameters[57];
                let AU = parameters[58];
                let AV = parameters[59];
                let BA = 1e-18f64;
                let BC = 0e0f64;
                let BD = 0e0f64;
                let BE = 0e0f64;
                let BF = 0e0f64;
                let BG = 0e0f64;
                let BH = 0e0f64;
                let BI = 0e0f64;
                let BT = 3.2e1f64;
                let BU = 9.1093826e-31f64;
                let BV = 1.6021918e-19f64;
                let CB = 5e-2f64;
                let CH = 9.5e-1f64;
                let CR = parameters[63];
                let mut oBB = 0.0;
                let mut oCC = 0.0;
                let mut oCF = 0.0;
                let mut oCI = 0.0;
                let mut oCP = 0.0;
                let mut oCS = 0.0;
                let mut oCT = 0.0;
                let mut oCU = 0.0;
                let A = if parameters[62] > 5e-1f64 { 1.0 } else { 0.0 };
                let D = if A != 0.0 {
                    B
                } else {
                    C
                };
                let E = 2.7315e2f64 + parameters[13];
                let F = 8.61726105451295e-5f64 * E;
                let G = B / F;
                let H = (-((7.02e-4f64 * E) * E)) / (1.108e3f64 + E);
                let I = parameters[24] + H;
                let J = parameters[25] + H;
                let L = K + H;
                let M = B - parameters[21];
                let N = B - parameters[22];
                let P = B - O;
                let Q = B / M;
                let R = B / N;
                let S = B / P;
                let U = T / parameters[15];
                let V = (parameters[33] * T) / parameters[16];
                let X = (parameters[34] * T) / W;
                let Y = B / U;
                let Z = B / V;
                let AA = B / X;
                let AB = B / parameters[18];
                let AC = B / parameters[19];
                let AE = B / AD;
                let AF = B - (B / parameters[14]);
                let AH = B / (B - (AF.powf(AG)));
                let AJ = B / (B - (AF.powf(AI)));
                let AL = B / (B - (AF.powf(AK)));
                let AM = B / parameters[50];
                let AN = B / parameters[51];
                let AO = B / parameters[52];
                let AP = ((-((AH * AH) * (AF.powf((AG - B))))) * AG) * AM;
                let AQ = ((-((AJ * AJ) * (AF.powf((AI - B))))) * AI) * AN;
                let AR = ((-((AL * AL) * (AF.powf((AK - B))))) * AK) * AO;
                let AW = if (if (if (if AS != B { 1.0 } else { 0.0 }) != 0.0 || (if AT != B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if AU != B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if AV != B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AX = if AW != 0.0 {
                    B
                } else {
                    C
                };
                let AY = if AX == B { 1.0 } else { 0.0 };
                let BJ;
                let BK;
                let BL;
                let BM;
                let BN;
                let BO;
                let BP;
                if AY != 0.0 {
                    let AZ = W * AS;
                    let BB = if AZ > BA { 1.0 } else { 0.0 };
                    oBB = BB;
                    let BZ = if BB != 0.0 {
                        AZ
                    } else {
                        BA
                    };
                    let CA = AD * AT;
                    let CC = if CA > CB { 1.0 } else { 0.0 };
                    oCC = CC;
                    let CD = if CC != 0.0 {
                        CA
                    } else {
                        CB
                    };
                    let CE = O * AU;
                    let CF = if CE > CB { 1.0 } else { 0.0 };
                    oCF = CF;
                    let CG = if CF != 0.0 {
                        CE
                    } else {
                        CB
                    };
                    let CI = if CG < CH { 1.0 } else { 0.0 };
                    oCI = CI;
                    let CJ;
                    if CI != 0.0 {
                        let CO = if CF != 0.0 {
                            CE
                        } else {
                            CB
                        };
                        CJ = CO;
                    } else {
                        CJ = CH;
                    }
                    let CK = K * AV;
                    let CL = CK + H;
                    let CM = B - CJ;
                    let CN = B / CM;
                    BJ = CK;
                    BK = CL;
                    BL = CD;
                    BM = BZ;
                    BN = CJ;
                    BO = CN;
                    BP = CM;
                } else {
                    BJ = BC;
                    BK = BD;
                    BL = BE;
                    BM = BF;
                    BN = BG;
                    BO = BH;
                    BP = BI;
                }
                let BQ = I * G;
                let BR = J * G;
                let BS = L * G;
                let BW = ((BT * parameters[38]) * BU) * BV;
                let BX = ((BT * parameters[39]) * BU) * BV;
                let BY = ((BT * parameters[40]) * BU) * BV;
                if AY != 0.0 {
                    let CP = BK * G;
                    oCP = CP;
                } else {
                }
                let CQ = if D == B { 1.0 } else { 0.0 };
                if CQ != 0.0 {
                    let CS = -4e-1f64 * CR;
                    oCS = CS;
                    let CT = -6.5e-1f64 * CR;
                    oCT = CT;
                    let CU = -8e-1f64 * CR;
                    oCU = CU;
                } else {
                }
            [A, E, F, M, N, P, Q, R, S, U, V, X, Y, Z, AA, AB, AC, AE, AF, AH, AJ, AL, AM, AN, AO, AP, AQ, AR, AW, AY, oBB, oCC, oCF, oCI, BQ, BR, BS, BW, BX, BY, BJ, oCP, BL, BM, BN, BO, CQ, oCS, oCT, oCU, BP]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 279] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[213];
                let B = parameters[3];
                let C = 0e0f64;
                let F = parameters[4];
                let I = parameters[5];
                let L = parameters[6];
                let P = parameters[22];
                let Q = parameters[23];
                let R = 9e-1f64;
                let T = parameters[19];
                let U = parameters[20];
                let W = parameters[21];
                let X = parameters[18];
                let AK = 2e0f64;
                let AL = 1e0f64;
                let AO = staged[233];
                let AS = staged[20];
                let AW = 4e0f64;
                let AX = staged[22];
                let AY = 5e-1f64;
                let BB = parameters[30];
                let BC = parameters[35];
                let BH = staged[4];
                let BK = parameters[41];
                let BN = parameters[50];
                let BO = 1e3f64;
                let BQ = staged[24];
                let BU = staged[28];
                let BX = staged[31];
                let CA = parameters[53];
                let CC = staged[33];
                let CD = staged[34];
                let CG = staged[32];
                let CM = parameters[31];
                let CN = parameters[36];
                let CS = staged[5];
                let CV = parameters[42];
                let CY = parameters[51];
                let DA = staged[37];
                let DE = staged[41];
                let DJ = parameters[54];
                let DL = staged[45];
                let DM = staged[46];
                let DP = staged[44];
                let DV = parameters[32];
                let DW = parameters[37];
                let EB = staged[6];
                let EE = parameters[43];
                let EH = parameters[52];
                let EJ = staged[49];
                let EN = staged[53];
                let ES = parameters[55];
                let EU = staged[57];
                let EV = staged[58];
                let EY = staged[56];
                let FG = staged[60];
                let II = staged[77];
                let LK = 1e-1f64;
                let LN = -1.000000082740371e-11f64;
                let OK = 2e-1f64;
                let ON = -5.000000413701855e-12f64;
                let RS = 1e0f64;
                let SC = staged[26];
                let SS = staged[39];
                let TI = staged[51];
                let mut oAP = 0.0;
                let mut oAT = 0.0;
                let mut oAU = 0.0;
                let mut oBD = 0.0;
                let mut oBE = 0.0;
                let mut oBF = 0.0;
                let mut oBG = 0.0;
                let mut oBI = 0.0;
                let mut oBJ = 0.0;
                let mut oBL = 0.0;
                let mut oBM = 0.0;
                let mut oBP = 0.0;
                let mut oBV = 0.0;
                let mut oBW = 0.0;
                let mut oBY = 0.0;
                let mut oBZ = 0.0;
                let mut oCB = 0.0;
                let mut oCO = 0.0;
                let mut oCP = 0.0;
                let mut oCQ = 0.0;
                let mut oCR = 0.0;
                let mut oCT = 0.0;
                let mut oCU = 0.0;
                let mut oCW = 0.0;
                let mut oCX = 0.0;
                let mut oCZ = 0.0;
                let mut oDF = 0.0;
                let mut oDG = 0.0;
                let mut oDH = 0.0;
                let mut oDI = 0.0;
                let mut oDK = 0.0;
                let mut oDX = 0.0;
                let mut oDY = 0.0;
                let mut oDZ = 0.0;
                let mut oEA = 0.0;
                let mut oEC = 0.0;
                let mut oED = 0.0;
                let mut oEF = 0.0;
                let mut oEG = 0.0;
                let mut oEI = 0.0;
                let mut oEO = 0.0;
                let mut oEP = 0.0;
                let mut oEQ = 0.0;
                let mut oER = 0.0;
                let mut oET = 0.0;
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
                let mut oFY = 0.0;
                let mut oFZ = 0.0;
                let mut oGA = 0.0;
                let mut oGB = 0.0;
                let mut oGC = 0.0;
                let mut oGK = 0.0;
                let mut oGL = 0.0;
                let mut oGM = 0.0;
                let mut oGN = 0.0;
                let mut oGO = 0.0;
                let mut oGP = 0.0;
                let mut oGQ = 0.0;
                let mut oGR = 0.0;
                let mut oGS = 0.0;
                let mut oGW = 0.0;
                let mut oGX = 0.0;
                let mut oGY = 0.0;
                let mut oGZ = 0.0;
                let mut oHA = 0.0;
                let mut oHI = 0.0;
                let mut oHJ = 0.0;
                let mut oHK = 0.0;
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
                let mut oIJ = 0.0;
                let mut oIK = 0.0;
                let mut oIO = 0.0;
                let mut oIP = 0.0;
                let mut oIQ = 0.0;
                let mut oIR = 0.0;
                let mut oIS = 0.0;
                let mut oIT = 0.0;
                let mut oIU = 0.0;
                let mut oIV = 0.0;
                let mut oIW = 0.0;
                let mut oJA = 0.0;
                let mut oJB = 0.0;
                let mut oJC = 0.0;
                let mut oJD = 0.0;
                let mut oJE = 0.0;
                let mut oJM = 0.0;
                let mut oJN = 0.0;
                let mut oJO = 0.0;
                let mut oJP = 0.0;
                let mut oJQ = 0.0;
                let mut oJR = 0.0;
                let mut oJS = 0.0;
                let mut oJT = 0.0;
                let mut oJU = 0.0;
                let mut oJY = 0.0;
                let mut oJZ = 0.0;
                let mut oKA = 0.0;
                let mut oKB = 0.0;
                let mut oKC = 0.0;
                let mut oKK = 0.0;
                let mut oKL = 0.0;
                let mut oKM = 0.0;
                let mut oKN = 0.0;
                let mut oKO = 0.0;
                let mut oKP = 0.0;
                let mut oKQ = 0.0;
                let mut oKR = 0.0;
                let mut oKS = 0.0;
                let mut oKW = 0.0;
                let mut oKX = 0.0;
                let mut oKY = 0.0;
                let mut oKZ = 0.0;
                let mut oLA = 0.0;
                let mut oLO = 0.0;
                let mut oLP = 0.0;
                let mut oLQ = 0.0;
                let mut oLR = 0.0;
                let mut oLS = 0.0;
                let mut oLT = 0.0;
                let mut oLU = 0.0;
                let mut oLV = 0.0;
                let mut oLW = 0.0;
                let mut oMA = 0.0;
                let mut oMB = 0.0;
                let mut oMC = 0.0;
                let mut oMD = 0.0;
                let mut oME = 0.0;
                let mut oMM = 0.0;
                let mut oMN = 0.0;
                let mut oMO = 0.0;
                let mut oMP = 0.0;
                let mut oMQ = 0.0;
                let mut oMR = 0.0;
                let mut oMS = 0.0;
                let mut oMT = 0.0;
                let mut oMU = 0.0;
                let mut oMY = 0.0;
                let mut oMZ = 0.0;
                let mut oNA = 0.0;
                let mut oNB = 0.0;
                let mut oNC = 0.0;
                let mut oNK = 0.0;
                let mut oNL = 0.0;
                let mut oNM = 0.0;
                let mut oNN = 0.0;
                let mut oNO = 0.0;
                let mut oNP = 0.0;
                let mut oNQ = 0.0;
                let mut oNR = 0.0;
                let mut oNS = 0.0;
                let mut oNW = 0.0;
                let mut oNX = 0.0;
                let mut oNY = 0.0;
                let mut oNZ = 0.0;
                let mut oOA = 0.0;
                let mut oOO = 0.0;
                let mut oOP = 0.0;
                let mut oOQ = 0.0;
                let mut oOR = 0.0;
                let mut oOS = 0.0;
                let mut oOT = 0.0;
                let mut oOU = 0.0;
                let mut oOV = 0.0;
                let mut oOW = 0.0;
                let mut oPA = 0.0;
                let mut oPB = 0.0;
                let mut oPC = 0.0;
                let mut oPD = 0.0;
                let mut oPE = 0.0;
                let mut oPM = 0.0;
                let mut oPN = 0.0;
                let mut oPO = 0.0;
                let mut oPP = 0.0;
                let mut oPQ = 0.0;
                let mut oPR = 0.0;
                let mut oPS = 0.0;
                let mut oPT = 0.0;
                let mut oPU = 0.0;
                let mut oPY = 0.0;
                let mut oPZ = 0.0;
                let mut oQA = 0.0;
                let mut oQB = 0.0;
                let mut oQC = 0.0;
                let mut oQK = 0.0;
                let mut oQL = 0.0;
                let mut oQM = 0.0;
                let mut oQN = 0.0;
                let mut oQO = 0.0;
                let mut oQP = 0.0;
                let mut oQQ = 0.0;
                let mut oQR = 0.0;
                let mut oQS = 0.0;
                let mut oQW = 0.0;
                let mut oQX = 0.0;
                let mut oQY = 0.0;
                let mut oQZ = 0.0;
                let mut oRA = 0.0;
                let mut oRI = 0.0;
                let mut oRJ = 0.0;
                let mut oRN = 0.0;
                let mut oRO = 0.0;
                let mut oRP = 0.0;
                let mut oRQ = 0.0;
                let mut oRR = 0.0;
                let mut oRT = 0.0;
                let mut oRU = 0.0;
                let mut oRV = 0.0;
                let mut oRW = 0.0;
                let mut oRX = 0.0;
                let mut oRY = 0.0;
                let mut oRZ = 0.0;
                let mut oSA = 0.0;
                let mut oSB = 0.0;
                let mut oSD = 0.0;
                let mut oSE = 0.0;
                let mut oSF = 0.0;
                let mut oSG = 0.0;
                let mut oSH = 0.0;
                let mut oSI = 0.0;
                let mut oSJ = 0.0;
                let mut oSK = 0.0;
                let mut oSL = 0.0;
                let mut oSM = 0.0;
                let mut oSN = 0.0;
                let mut oSO = 0.0;
                let mut oSP = 0.0;
                let mut oSQ = 0.0;
                let mut oSR = 0.0;
                let mut oST = 0.0;
                let mut oSU = 0.0;
                let mut oSV = 0.0;
                let mut oSW = 0.0;
                let mut oSX = 0.0;
                let mut oSY = 0.0;
                let mut oSZ = 0.0;
                let mut oTA = 0.0;
                let mut oTB = 0.0;
                let mut oTC = 0.0;
                let mut oTD = 0.0;
                let mut oTE = 0.0;
                let mut oTF = 0.0;
                let mut oTG = 0.0;
                let mut oTH = 0.0;
                let mut oTJ = 0.0;
                let mut oTK = 0.0;
                let D = if B > C { 1.0 } else { 0.0 };
                let E = if D != 0.0 {
                    B
                } else {
                    C
                };
                let G = if F > C { 1.0 } else { 0.0 };
                let H = if G != 0.0 {
                    F
                } else {
                    C
                };
                let J = if I > C { 1.0 } else { 0.0 };
                let K = if J != 0.0 {
                    I
                } else {
                    C
                };
                let M = if L > C { 1.0 } else { 0.0 };
                let N = if M != 0.0 {
                    L
                } else {
                    C
                };
                let O = if E == C { 1.0 } else { 0.0 };
                let Y;
                let Z;
                if O != 0.0 {
                    let S = R * (if P <= Q { P } else { Q });
                    let V = T + U;
                    Y = S;
                    Z = V;
                } else {
                    Y = W;
                    Z = X;
                }
                let AA = if H == C { 1.0 } else { 0.0 };
                let AD;
                let AE;
                if AA != 0.0 {
                    let AB = R * (if W <= Q { W } else { Q });
                    let AC = X + U;
                    AD = AB;
                    AE = AC;
                } else {
                    AD = P;
                    AE = T;
                }
                let AF = if K == C { 1.0 } else { 0.0 };
                let AI;
                let AJ;
                if AF != 0.0 {
                    let AG = R * (if W <= P { W } else { P });
                    let AH = X + T;
                    AI = AG;
                    AJ = AH;
                } else {
                    AI = Q;
                    AJ = U;
                }
                let AM = AL - (AK.powf((-1e0f64 / (if (if Y >= AD { Y } else { AD }) >= AI { (if Y >= AD { Y } else { AD }) } else { AI }))));
                let AN = (if (if Z <= AE { Z } else { AE }) <= AJ { (if Z <= AE { Z } else { AE }) } else { AJ }) - 5e-2f64;
                if AO != 0.0 {
                    let AP = if (if (if O != 0.0 && AA != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AF != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oAP = AP;
                    let AQ;
                    let AR;
                    if AP != 0.0 {
                        let AT = if AS > C { 1.0 } else { 0.0 };
                        oAT = AT;
                        if AT != 0.0 {
                        } else {
                            let AU = -AS;
                            oAU = AU;
                        }
                        let AV = AS - AN;
                        let AZ = AY * ((AS + AN) - (((AV * AV) + ((AW * AX) * AX)).sqrt()));
                        let BA = AY * (AS - (((AS * AS) + 4e-12f64).sqrt()));
                        AQ = AZ;
                        AR = BA;
                    } else {
                        AQ = C;
                        AR = C;
                    }
                    if O != 0.0 {
                    } else {
                        let BD = if BC == C { 1.0 } else { 0.0 };
                        oBD = BD;
                        let BE = if (if BB == C { 1.0 } else { 0.0 }) != 0.0 && BD != 0.0 { 1.0 } else { 0.0 };
                        oBE = BE;
                        if BE != 0.0 {
                        } else {
                            let BF = if W == AY { 1.0 } else { 0.0 };
                            oBF = BF;
                            if BF != 0.0 {
                            } else {
                                let BG = AL - (AK * W);
                                oBG = BG;
                            }
                        }
                        if BD != 0.0 {
                        } else {
                            let BI = (-W) * BH;
                            oBI = BI;
                            let BJ = if BI == -1e0f64 { 1.0 } else { 0.0 };
                            oBJ = BJ;
                        }
                        let BL = if BK == C { 1.0 } else { 0.0 };
                        oBL = BL;
                        if BL != 0.0 {
                        } else {
                            let BM = if W == AY { 1.0 } else { 0.0 };
                            oBM = BM;
                            let BT = if BM != 0.0 {
                                let BR = ((X - AQ) * BQ).sqrt();
                                BR
                            } else {
                                let BS = ((X - AQ) * BQ).powf(W);
                                BS
                            };
                            let BV = BH * (((X - AQ) * BU) / BT);
                            oBV = BV;
                            let BW = (AS * BV) * BV;
                            oBW = BW;
                        }
                        let BP = if BN > BO { 1.0 } else { 0.0 };
                        oBP = BP;
                        let BZ;
                        if BP != 0.0 {
                            BZ = AL;
                        } else {
                            let BY = if AR > ((-BX) * BN) { 1.0 } else { 0.0 };
                            oBY = BY;
                            let CF;
                            if BY != 0.0 {
                                let CB = if CA == AW { 1.0 } else { 0.0 };
                                oCB = CB;
                                let CK = if CB != 0.0 {
                                    let CH = AR * CG;
                                    let CI = ((CH * CH) * CH) * CH;
                                    CI
                                } else {
                                    let CJ = ((AR * CG).abs()).powf(CA);
                                    CJ
                                };
                                let CL = AL / (AL - CK);
                                CF = CL;
                            } else {
                                let CE = CD + ((AR + (BX * BN)) * CC);
                                CF = CE;
                            }
                            BZ = CF;
                        }
                        oBZ = BZ;
                    }
                    if AA != 0.0 {
                    } else {
                        let CO = if CN == C { 1.0 } else { 0.0 };
                        oCO = CO;
                        let CP = if (if CM == C { 1.0 } else { 0.0 }) != 0.0 && CO != 0.0 { 1.0 } else { 0.0 };
                        oCP = CP;
                        if CP != 0.0 {
                        } else {
                            let CQ = if P == AY { 1.0 } else { 0.0 };
                            oCQ = CQ;
                            if CQ != 0.0 {
                            } else {
                                let CR = AL - (AK * P);
                                oCR = CR;
                            }
                        }
                        if CO != 0.0 {
                        } else {
                            let CT = (-P) * CS;
                            oCT = CT;
                            let CU = if CT == -1e0f64 { 1.0 } else { 0.0 };
                            oCU = CU;
                        }
                        let CW = if CV == C { 1.0 } else { 0.0 };
                        oCW = CW;
                        if CW != 0.0 {
                        } else {
                            let CX = if P == AY { 1.0 } else { 0.0 };
                            oCX = CX;
                            let DD = if CX != 0.0 {
                                let DB = ((T - AQ) * DA).sqrt();
                                DB
                            } else {
                                let DC = ((T - AQ) * DA).powf(P);
                                DC
                            };
                            let DF = CS * (((T - AQ) * DE) / DD);
                            oDF = DF;
                            let DG = (AS * DF) * DF;
                            oDG = DG;
                        }
                        let CZ = if CY > BO { 1.0 } else { 0.0 };
                        oCZ = CZ;
                        let DI;
                        if CZ != 0.0 {
                            DI = AL;
                        } else {
                            let DH = if AR > ((-BX) * CY) { 1.0 } else { 0.0 };
                            oDH = DH;
                            let DO;
                            if DH != 0.0 {
                                let DK = if DJ == AW { 1.0 } else { 0.0 };
                                oDK = DK;
                                let DT = if DK != 0.0 {
                                    let DQ = AR * DP;
                                    let DR = ((DQ * DQ) * DQ) * DQ;
                                    DR
                                } else {
                                    let DS = ((AR * DP).abs()).powf(DJ);
                                    DS
                                };
                                let DU = AL / (AL - DT);
                                DO = DU;
                            } else {
                                let DN = DM + ((AR + (BX * CY)) * DL);
                                DO = DN;
                            }
                            DI = DO;
                        }
                        oDI = DI;
                    }
                    if AF != 0.0 {
                    } else {
                        let DX = if DW == C { 1.0 } else { 0.0 };
                        oDX = DX;
                        let DY = if (if DV == C { 1.0 } else { 0.0 }) != 0.0 && DX != 0.0 { 1.0 } else { 0.0 };
                        oDY = DY;
                        if DY != 0.0 {
                        } else {
                            let DZ = if Q == AY { 1.0 } else { 0.0 };
                            oDZ = DZ;
                            if DZ != 0.0 {
                            } else {
                                let EA = AL - (AK * Q);
                                oEA = EA;
                            }
                        }
                        if DX != 0.0 {
                        } else {
                            let EC = (-Q) * EB;
                            oEC = EC;
                            let ED = if EC == -1e0f64 { 1.0 } else { 0.0 };
                            oED = ED;
                        }
                        let EF = if EE == C { 1.0 } else { 0.0 };
                        oEF = EF;
                        if EF != 0.0 {
                        } else {
                            let EG = if Q == AY { 1.0 } else { 0.0 };
                            oEG = EG;
                            let EM = if EG != 0.0 {
                                let EK = ((U - AQ) * EJ).sqrt();
                                EK
                            } else {
                                let EL = ((U - AQ) * EJ).powf(Q);
                                EL
                            };
                            let EO = EB * (((U - AQ) * EN) / EM);
                            oEO = EO;
                            let EP = (AS * EO) * EO;
                            oEP = EP;
                        }
                        let EI = if EH > BO { 1.0 } else { 0.0 };
                        oEI = EI;
                        let ER;
                        if EI != 0.0 {
                            ER = AL;
                        } else {
                            let EQ = if AR > ((-BX) * EH) { 1.0 } else { 0.0 };
                            oEQ = EQ;
                            let EX;
                            if EQ != 0.0 {
                                let ET = if ES == AW { 1.0 } else { 0.0 };
                                oET = ET;
                                let FC = if ET != 0.0 {
                                    let EZ = AR * EY;
                                    let FA = ((EZ * EZ) * EZ) * EZ;
                                    FA
                                } else {
                                    let FB = ((AR * EY).abs()).powf(ES);
                                    FB
                                };
                                let FD = AL / (AL - FC);
                                EX = FD;
                            } else {
                                let EW = EV + ((AR + (BX * EH)) * EU);
                                EX = EW;
                            }
                            ER = EX;
                        }
                        oER = ER;
                    }
                    let FE;
                    let FF;
                    if AP != 0.0 {
                        let FH = if FG > C { 1.0 } else { 0.0 };
                        oFH = FH;
                        if FH != 0.0 {
                        } else {
                            let FI = -FG;
                            oFI = FI;
                        }
                        let FJ = FG - AN;
                        let FK = AY * ((FG + AN) - (((FJ * FJ) + ((AW * AX) * AX)).sqrt()));
                        let FL = AY * (FG - (((FG * FG) + 4e-12f64).sqrt()));
                        FE = FK;
                        FF = FL;
                    } else {
                        FE = C;
                        FF = AR;
                    }
                    if O != 0.0 {
                    } else {
                        let FM = if BC == C { 1.0 } else { 0.0 };
                        oFM = FM;
                        let FN = if (if BB == C { 1.0 } else { 0.0 }) != 0.0 && FM != 0.0 { 1.0 } else { 0.0 };
                        oFN = FN;
                        if FN != 0.0 {
                        } else {
                            let FO = if W == AY { 1.0 } else { 0.0 };
                            oFO = FO;
                            if FO != 0.0 {
                            } else {
                                let FP = AL - (AK * W);
                                oFP = FP;
                            }
                        }
                        if FM != 0.0 {
                        } else {
                            let FQ = (-W) * BH;
                            oFQ = FQ;
                            let FR = if FQ == -1e0f64 { 1.0 } else { 0.0 };
                            oFR = FR;
                        }
                        let FS = if BK == C { 1.0 } else { 0.0 };
                        oFS = FS;
                        if FS != 0.0 {
                        } else {
                            let FT = if W == AY { 1.0 } else { 0.0 };
                            oFT = FT;
                            let FX = if FT != 0.0 {
                                let FV = ((X - FE) * BQ).sqrt();
                                FV
                            } else {
                                let FW = ((X - FE) * BQ).powf(W);
                                FW
                            };
                            let FY = BH * (((X - FE) * BU) / FX);
                            oFY = FY;
                            let FZ = (FG * FY) * FY;
                            oFZ = FZ;
                        }
                        let FU = if BN > BO { 1.0 } else { 0.0 };
                        oFU = FU;
                        let GB;
                        if FU != 0.0 {
                            GB = AL;
                        } else {
                            let GA = if FF > ((-BX) * BN) { 1.0 } else { 0.0 };
                            oGA = GA;
                            let GE;
                            if GA != 0.0 {
                                let GC = if CA == AW { 1.0 } else { 0.0 };
                                oGC = GC;
                                let GI = if GC != 0.0 {
                                    let GF = FF * CG;
                                    let GG = ((GF * GF) * GF) * GF;
                                    GG
                                } else {
                                    let GH = ((FF * CG).abs()).powf(CA);
                                    GH
                                };
                                let GJ = AL / (AL - GI);
                                GE = GJ;
                            } else {
                                let GD = CD + ((FF + (BX * BN)) * CC);
                                GE = GD;
                            }
                            GB = GE;
                        }
                        oGB = GB;
                    }
                    if AA != 0.0 {
                    } else {
                        let GK = if CN == C { 1.0 } else { 0.0 };
                        oGK = GK;
                        let GL = if (if CM == C { 1.0 } else { 0.0 }) != 0.0 && GK != 0.0 { 1.0 } else { 0.0 };
                        oGL = GL;
                        if GL != 0.0 {
                        } else {
                            let GM = if P == AY { 1.0 } else { 0.0 };
                            oGM = GM;
                            if GM != 0.0 {
                            } else {
                                let GN = AL - (AK * P);
                                oGN = GN;
                            }
                        }
                        if GK != 0.0 {
                        } else {
                            let GO = (-P) * CS;
                            oGO = GO;
                            let GP = if GO == -1e0f64 { 1.0 } else { 0.0 };
                            oGP = GP;
                        }
                        let GQ = if CV == C { 1.0 } else { 0.0 };
                        oGQ = GQ;
                        if GQ != 0.0 {
                        } else {
                            let GR = if P == AY { 1.0 } else { 0.0 };
                            oGR = GR;
                            let GV = if GR != 0.0 {
                                let GT = ((T - FE) * DA).sqrt();
                                GT
                            } else {
                                let GU = ((T - FE) * DA).powf(P);
                                GU
                            };
                            let GW = CS * (((T - FE) * DE) / GV);
                            oGW = GW;
                            let GX = (FG * GW) * GW;
                            oGX = GX;
                        }
                        let GS = if CY > BO { 1.0 } else { 0.0 };
                        oGS = GS;
                        let GZ;
                        if GS != 0.0 {
                            GZ = AL;
                        } else {
                            let GY = if FF > ((-BX) * CY) { 1.0 } else { 0.0 };
                            oGY = GY;
                            let HC;
                            if GY != 0.0 {
                                let HA = if DJ == AW { 1.0 } else { 0.0 };
                                oHA = HA;
                                let HG = if HA != 0.0 {
                                    let HD = FF * DP;
                                    let HE = ((HD * HD) * HD) * HD;
                                    HE
                                } else {
                                    let HF = ((FF * DP).abs()).powf(DJ);
                                    HF
                                };
                                let HH = AL / (AL - HG);
                                HC = HH;
                            } else {
                                let HB = DM + ((FF + (BX * CY)) * DL);
                                HC = HB;
                            }
                            GZ = HC;
                        }
                        oGZ = GZ;
                    }
                    if AF != 0.0 {
                    } else {
                        let HI = if DW == C { 1.0 } else { 0.0 };
                        oHI = HI;
                        let HJ = if (if DV == C { 1.0 } else { 0.0 }) != 0.0 && HI != 0.0 { 1.0 } else { 0.0 };
                        oHJ = HJ;
                        if HJ != 0.0 {
                        } else {
                            let HK = if Q == AY { 1.0 } else { 0.0 };
                            oHK = HK;
                            if HK != 0.0 {
                            } else {
                                let HL = AL - (AK * Q);
                                oHL = HL;
                            }
                        }
                        if HI != 0.0 {
                        } else {
                            let HM = (-Q) * EB;
                            oHM = HM;
                            let HN = if HM == -1e0f64 { 1.0 } else { 0.0 };
                            oHN = HN;
                        }
                        let HO = if EE == C { 1.0 } else { 0.0 };
                        oHO = HO;
                        if HO != 0.0 {
                        } else {
                            let HP = if Q == AY { 1.0 } else { 0.0 };
                            oHP = HP;
                            let HT = if HP != 0.0 {
                                let HR = ((U - FE) * EJ).sqrt();
                                HR
                            } else {
                                let HS = ((U - FE) * EJ).powf(Q);
                                HS
                            };
                            let HU = EB * (((U - FE) * EN) / HT);
                            oHU = HU;
                            let HV = (FG * HU) * HU;
                            oHV = HV;
                        }
                        let HQ = if EH > BO { 1.0 } else { 0.0 };
                        oHQ = HQ;
                        let HX;
                        if HQ != 0.0 {
                            HX = AL;
                        } else {
                            let HW = if FF > ((-BX) * EH) { 1.0 } else { 0.0 };
                            oHW = HW;
                            let IA;
                            if HW != 0.0 {
                                let HY = if ES == AW { 1.0 } else { 0.0 };
                                oHY = HY;
                                let IE = if HY != 0.0 {
                                    let IB = FF * EY;
                                    let IC = ((IB * IB) * IB) * IB;
                                    IC
                                } else {
                                    let ID = ((FF * EY).abs()).powf(ES);
                                    ID
                                };
                                let IF = AL / (AL - IE);
                                IA = IF;
                            } else {
                                let HZ = EV + ((FF + (BX * EH)) * EU);
                                IA = HZ;
                            }
                            HX = IA;
                        }
                        oHX = HX;
                    }
                    let IG;
                    let IH;
                    if AP != 0.0 {
                        let IJ = if II > C { 1.0 } else { 0.0 };
                        oIJ = IJ;
                        if IJ != 0.0 {
                        } else {
                            let IK = -II;
                            oIK = IK;
                        }
                        let IL = II - AN;
                        let IM = AY * ((II + AN) - (((IL * IL) + ((AW * AX) * AX)).sqrt()));
                        let IN = AY * (II - (((II * II) + 4e-12f64).sqrt()));
                        IG = IM;
                        IH = IN;
                    } else {
                        IG = C;
                        IH = FF;
                    }
                    if O != 0.0 {
                    } else {
                        let IO = if BC == C { 1.0 } else { 0.0 };
                        oIO = IO;
                        let IP = if (if BB == C { 1.0 } else { 0.0 }) != 0.0 && IO != 0.0 { 1.0 } else { 0.0 };
                        oIP = IP;
                        if IP != 0.0 {
                        } else {
                            let IQ = if W == AY { 1.0 } else { 0.0 };
                            oIQ = IQ;
                            if IQ != 0.0 {
                            } else {
                                let IR = AL - (AK * W);
                                oIR = IR;
                            }
                        }
                        if IO != 0.0 {
                        } else {
                            let IS = (-W) * BH;
                            oIS = IS;
                            let IT = if IS == -1e0f64 { 1.0 } else { 0.0 };
                            oIT = IT;
                        }
                        let IU = if BK == C { 1.0 } else { 0.0 };
                        oIU = IU;
                        if IU != 0.0 {
                        } else {
                            let IV = if W == AY { 1.0 } else { 0.0 };
                            oIV = IV;
                            let IZ = if IV != 0.0 {
                                let IX = ((X - IG) * BQ).sqrt();
                                IX
                            } else {
                                let IY = ((X - IG) * BQ).powf(W);
                                IY
                            };
                            let JA = BH * (((X - IG) * BU) / IZ);
                            oJA = JA;
                            let JB = (II * JA) * JA;
                            oJB = JB;
                        }
                        let IW = if BN > BO { 1.0 } else { 0.0 };
                        oIW = IW;
                        let JD;
                        if IW != 0.0 {
                            JD = AL;
                        } else {
                            let JC = if IH > ((-BX) * BN) { 1.0 } else { 0.0 };
                            oJC = JC;
                            let JG;
                            if JC != 0.0 {
                                let JE = if CA == AW { 1.0 } else { 0.0 };
                                oJE = JE;
                                let JK = if JE != 0.0 {
                                    let JH = IH * CG;
                                    let JI = ((JH * JH) * JH) * JH;
                                    JI
                                } else {
                                    let JJ = ((IH * CG).abs()).powf(CA);
                                    JJ
                                };
                                let JL = AL / (AL - JK);
                                JG = JL;
                            } else {
                                let JF = CD + ((IH + (BX * BN)) * CC);
                                JG = JF;
                            }
                            JD = JG;
                        }
                        oJD = JD;
                    }
                    if AA != 0.0 {
                    } else {
                        let JM = if CN == C { 1.0 } else { 0.0 };
                        oJM = JM;
                        let JN = if (if CM == C { 1.0 } else { 0.0 }) != 0.0 && JM != 0.0 { 1.0 } else { 0.0 };
                        oJN = JN;
                        if JN != 0.0 {
                        } else {
                            let JO = if P == AY { 1.0 } else { 0.0 };
                            oJO = JO;
                            if JO != 0.0 {
                            } else {
                                let JP = AL - (AK * P);
                                oJP = JP;
                            }
                        }
                        if JM != 0.0 {
                        } else {
                            let JQ = (-P) * CS;
                            oJQ = JQ;
                            let JR = if JQ == -1e0f64 { 1.0 } else { 0.0 };
                            oJR = JR;
                        }
                        let JS = if CV == C { 1.0 } else { 0.0 };
                        oJS = JS;
                        if JS != 0.0 {
                        } else {
                            let JT = if P == AY { 1.0 } else { 0.0 };
                            oJT = JT;
                            let JX = if JT != 0.0 {
                                let JV = ((T - IG) * DA).sqrt();
                                JV
                            } else {
                                let JW = ((T - IG) * DA).powf(P);
                                JW
                            };
                            let JY = CS * (((T - IG) * DE) / JX);
                            oJY = JY;
                            let JZ = (II * JY) * JY;
                            oJZ = JZ;
                        }
                        let JU = if CY > BO { 1.0 } else { 0.0 };
                        oJU = JU;
                        let KB;
                        if JU != 0.0 {
                            KB = AL;
                        } else {
                            let KA = if IH > ((-BX) * CY) { 1.0 } else { 0.0 };
                            oKA = KA;
                            let KE;
                            if KA != 0.0 {
                                let KC = if DJ == AW { 1.0 } else { 0.0 };
                                oKC = KC;
                                let KI = if KC != 0.0 {
                                    let KF = IH * DP;
                                    let KG = ((KF * KF) * KF) * KF;
                                    KG
                                } else {
                                    let KH = ((IH * DP).abs()).powf(DJ);
                                    KH
                                };
                                let KJ = AL / (AL - KI);
                                KE = KJ;
                            } else {
                                let KD = DM + ((IH + (BX * CY)) * DL);
                                KE = KD;
                            }
                            KB = KE;
                        }
                        oKB = KB;
                    }
                    if AF != 0.0 {
                    } else {
                        let KK = if DW == C { 1.0 } else { 0.0 };
                        oKK = KK;
                        let KL = if (if DV == C { 1.0 } else { 0.0 }) != 0.0 && KK != 0.0 { 1.0 } else { 0.0 };
                        oKL = KL;
                        if KL != 0.0 {
                        } else {
                            let KM = if Q == AY { 1.0 } else { 0.0 };
                            oKM = KM;
                            if KM != 0.0 {
                            } else {
                                let KN = AL - (AK * Q);
                                oKN = KN;
                            }
                        }
                        if KK != 0.0 {
                        } else {
                            let KO = (-Q) * EB;
                            oKO = KO;
                            let KP = if KO == -1e0f64 { 1.0 } else { 0.0 };
                            oKP = KP;
                        }
                        let KQ = if EE == C { 1.0 } else { 0.0 };
                        oKQ = KQ;
                        if KQ != 0.0 {
                        } else {
                            let KR = if Q == AY { 1.0 } else { 0.0 };
                            oKR = KR;
                            let KV = if KR != 0.0 {
                                let KT = ((U - IG) * EJ).sqrt();
                                KT
                            } else {
                                let KU = ((U - IG) * EJ).powf(Q);
                                KU
                            };
                            let KW = EB * (((U - IG) * EN) / KV);
                            oKW = KW;
                            let KX = (II * KW) * KW;
                            oKX = KX;
                        }
                        let KS = if EH > BO { 1.0 } else { 0.0 };
                        oKS = KS;
                        let KZ;
                        if KS != 0.0 {
                            KZ = AL;
                        } else {
                            let KY = if IH > ((-BX) * EH) { 1.0 } else { 0.0 };
                            oKY = KY;
                            let LC;
                            if KY != 0.0 {
                                let LA = if ES == AW { 1.0 } else { 0.0 };
                                oLA = LA;
                                let LG = if LA != 0.0 {
                                    let LD = IH * EY;
                                    let LE = ((LD * LD) * LD) * LD;
                                    LE
                                } else {
                                    let LF = ((IH * EY).abs()).powf(ES);
                                    LF
                                };
                                let LH = AL / (AL - LG);
                                LC = LH;
                            } else {
                                let LB = EV + ((IH + (BX * EH)) * EU);
                                LC = LB;
                            }
                            KZ = LC;
                        }
                        oKZ = KZ;
                    }
                    let LI;
                    let LJ;
                    if AP != 0.0 {
                        let LL = LK - AN;
                        let LM = AY * ((LK + AN) - (((LL * LL) + ((AW * AX) * AX)).sqrt()));
                        LI = LM;
                        LJ = LN;
                    } else {
                        LI = C;
                        LJ = IH;
                    }
                    if O != 0.0 {
                    } else {
                        let LO = if BC == C { 1.0 } else { 0.0 };
                        oLO = LO;
                        let LP = if (if BB == C { 1.0 } else { 0.0 }) != 0.0 && LO != 0.0 { 1.0 } else { 0.0 };
                        oLP = LP;
                        if LP != 0.0 {
                        } else {
                            let LQ = if W == AY { 1.0 } else { 0.0 };
                            oLQ = LQ;
                            if LQ != 0.0 {
                            } else {
                                let LR = AL - (AK * W);
                                oLR = LR;
                            }
                        }
                        if LO != 0.0 {
                        } else {
                            let LS = (-W) * BH;
                            oLS = LS;
                            let LT = if LS == -1e0f64 { 1.0 } else { 0.0 };
                            oLT = LT;
                        }
                        let LU = if BK == C { 1.0 } else { 0.0 };
                        oLU = LU;
                        if LU != 0.0 {
                        } else {
                            let LV = if W == AY { 1.0 } else { 0.0 };
                            oLV = LV;
                            let LZ = if LV != 0.0 {
                                let LX = ((X - LI) * BQ).sqrt();
                                LX
                            } else {
                                let LY = ((X - LI) * BQ).powf(W);
                                LY
                            };
                            let MA = BH * (((X - LI) * BU) / LZ);
                            oMA = MA;
                            let MB = (LK * MA) * MA;
                            oMB = MB;
                        }
                        let LW = if BN > BO { 1.0 } else { 0.0 };
                        oLW = LW;
                        let MD;
                        if LW != 0.0 {
                            MD = AL;
                        } else {
                            let MC = if LJ > ((-BX) * BN) { 1.0 } else { 0.0 };
                            oMC = MC;
                            let MG;
                            if MC != 0.0 {
                                let ME = if CA == AW { 1.0 } else { 0.0 };
                                oME = ME;
                                let MK = if ME != 0.0 {
                                    let MH = LJ * CG;
                                    let MI = ((MH * MH) * MH) * MH;
                                    MI
                                } else {
                                    let MJ = ((LJ * CG).abs()).powf(CA);
                                    MJ
                                };
                                let ML = AL / (AL - MK);
                                MG = ML;
                            } else {
                                let MF = CD + ((LJ + (BX * BN)) * CC);
                                MG = MF;
                            }
                            MD = MG;
                        }
                        oMD = MD;
                    }
                    if AA != 0.0 {
                    } else {
                        let MM = if CN == C { 1.0 } else { 0.0 };
                        oMM = MM;
                        let MN = if (if CM == C { 1.0 } else { 0.0 }) != 0.0 && MM != 0.0 { 1.0 } else { 0.0 };
                        oMN = MN;
                        if MN != 0.0 {
                        } else {
                            let MO = if P == AY { 1.0 } else { 0.0 };
                            oMO = MO;
                            if MO != 0.0 {
                            } else {
                                let MP = AL - (AK * P);
                                oMP = MP;
                            }
                        }
                        if MM != 0.0 {
                        } else {
                            let MQ = (-P) * CS;
                            oMQ = MQ;
                            let MR = if MQ == -1e0f64 { 1.0 } else { 0.0 };
                            oMR = MR;
                        }
                        let MS = if CV == C { 1.0 } else { 0.0 };
                        oMS = MS;
                        if MS != 0.0 {
                        } else {
                            let MT = if P == AY { 1.0 } else { 0.0 };
                            oMT = MT;
                            let MX = if MT != 0.0 {
                                let MV = ((T - LI) * DA).sqrt();
                                MV
                            } else {
                                let MW = ((T - LI) * DA).powf(P);
                                MW
                            };
                            let MY = CS * (((T - LI) * DE) / MX);
                            oMY = MY;
                            let MZ = (LK * MY) * MY;
                            oMZ = MZ;
                        }
                        let MU = if CY > BO { 1.0 } else { 0.0 };
                        oMU = MU;
                        let NB;
                        if MU != 0.0 {
                            NB = AL;
                        } else {
                            let NA = if LJ > ((-BX) * CY) { 1.0 } else { 0.0 };
                            oNA = NA;
                            let NE;
                            if NA != 0.0 {
                                let NC = if DJ == AW { 1.0 } else { 0.0 };
                                oNC = NC;
                                let NI = if NC != 0.0 {
                                    let NF = LJ * DP;
                                    let NG = ((NF * NF) * NF) * NF;
                                    NG
                                } else {
                                    let NH = ((LJ * DP).abs()).powf(DJ);
                                    NH
                                };
                                let NJ = AL / (AL - NI);
                                NE = NJ;
                            } else {
                                let ND = DM + ((LJ + (BX * CY)) * DL);
                                NE = ND;
                            }
                            NB = NE;
                        }
                        oNB = NB;
                    }
                    if AF != 0.0 {
                    } else {
                        let NK = if DW == C { 1.0 } else { 0.0 };
                        oNK = NK;
                        let NL = if (if DV == C { 1.0 } else { 0.0 }) != 0.0 && NK != 0.0 { 1.0 } else { 0.0 };
                        oNL = NL;
                        if NL != 0.0 {
                        } else {
                            let NM = if Q == AY { 1.0 } else { 0.0 };
                            oNM = NM;
                            if NM != 0.0 {
                            } else {
                                let NN = AL - (AK * Q);
                                oNN = NN;
                            }
                        }
                        if NK != 0.0 {
                        } else {
                            let NO = (-Q) * EB;
                            oNO = NO;
                            let NP = if NO == -1e0f64 { 1.0 } else { 0.0 };
                            oNP = NP;
                        }
                        let NQ = if EE == C { 1.0 } else { 0.0 };
                        oNQ = NQ;
                        if NQ != 0.0 {
                        } else {
                            let NR = if Q == AY { 1.0 } else { 0.0 };
                            oNR = NR;
                            let NV = if NR != 0.0 {
                                let NT = ((U - LI) * EJ).sqrt();
                                NT
                            } else {
                                let NU = ((U - LI) * EJ).powf(Q);
                                NU
                            };
                            let NW = EB * (((U - LI) * EN) / NV);
                            oNW = NW;
                            let NX = (LK * NW) * NW;
                            oNX = NX;
                        }
                        let NS = if EH > BO { 1.0 } else { 0.0 };
                        oNS = NS;
                        let NZ;
                        if NS != 0.0 {
                            NZ = AL;
                        } else {
                            let NY = if LJ > ((-BX) * EH) { 1.0 } else { 0.0 };
                            oNY = NY;
                            let OC;
                            if NY != 0.0 {
                                let OA = if ES == AW { 1.0 } else { 0.0 };
                                oOA = OA;
                                let OG = if OA != 0.0 {
                                    let OD = LJ * EY;
                                    let OE = ((OD * OD) * OD) * OD;
                                    OE
                                } else {
                                    let OF = ((LJ * EY).abs()).powf(ES);
                                    OF
                                };
                                let OH = AL / (AL - OG);
                                OC = OH;
                            } else {
                                let OB = EV + ((LJ + (BX * EH)) * EU);
                                OC = OB;
                            }
                            NZ = OC;
                        }
                        oNZ = NZ;
                    }
                    let OI;
                    let OJ;
                    if AP != 0.0 {
                        let OL = OK - AN;
                        let OM = AY * ((OK + AN) - (((OL * OL) + ((AW * AX) * AX)).sqrt()));
                        OI = OM;
                        OJ = ON;
                    } else {
                        OI = C;
                        OJ = LJ;
                    }
                    if O != 0.0 {
                    } else {
                        let OO = if BC == C { 1.0 } else { 0.0 };
                        oOO = OO;
                        let OP = if (if BB == C { 1.0 } else { 0.0 }) != 0.0 && OO != 0.0 { 1.0 } else { 0.0 };
                        oOP = OP;
                        if OP != 0.0 {
                        } else {
                            let OQ = if W == AY { 1.0 } else { 0.0 };
                            oOQ = OQ;
                            if OQ != 0.0 {
                            } else {
                                let OR = AL - (AK * W);
                                oOR = OR;
                            }
                        }
                        if OO != 0.0 {
                        } else {
                            let OS = (-W) * BH;
                            oOS = OS;
                            let OT = if OS == -1e0f64 { 1.0 } else { 0.0 };
                            oOT = OT;
                        }
                        let OU = if BK == C { 1.0 } else { 0.0 };
                        oOU = OU;
                        if OU != 0.0 {
                        } else {
                            let OV = if W == AY { 1.0 } else { 0.0 };
                            oOV = OV;
                            let OZ = if OV != 0.0 {
                                let OX = ((X - OI) * BQ).sqrt();
                                OX
                            } else {
                                let OY = ((X - OI) * BQ).powf(W);
                                OY
                            };
                            let PA = BH * (((X - OI) * BU) / OZ);
                            oPA = PA;
                            let PB = (OK * PA) * PA;
                            oPB = PB;
                        }
                        let OW = if BN > BO { 1.0 } else { 0.0 };
                        oOW = OW;
                        let PD;
                        if OW != 0.0 {
                            PD = AL;
                        } else {
                            let PC = if OJ > ((-BX) * BN) { 1.0 } else { 0.0 };
                            oPC = PC;
                            let PG;
                            if PC != 0.0 {
                                let PE = if CA == AW { 1.0 } else { 0.0 };
                                oPE = PE;
                                let PK = if PE != 0.0 {
                                    let PH = OJ * CG;
                                    let PI = ((PH * PH) * PH) * PH;
                                    PI
                                } else {
                                    let PJ = ((OJ * CG).abs()).powf(CA);
                                    PJ
                                };
                                let PL = AL / (AL - PK);
                                PG = PL;
                            } else {
                                let PF = CD + ((OJ + (BX * BN)) * CC);
                                PG = PF;
                            }
                            PD = PG;
                        }
                        oPD = PD;
                    }
                    if AA != 0.0 {
                    } else {
                        let PM = if CN == C { 1.0 } else { 0.0 };
                        oPM = PM;
                        let PN = if (if CM == C { 1.0 } else { 0.0 }) != 0.0 && PM != 0.0 { 1.0 } else { 0.0 };
                        oPN = PN;
                        if PN != 0.0 {
                        } else {
                            let PO = if P == AY { 1.0 } else { 0.0 };
                            oPO = PO;
                            if PO != 0.0 {
                            } else {
                                let PP = AL - (AK * P);
                                oPP = PP;
                            }
                        }
                        if PM != 0.0 {
                        } else {
                            let PQ = (-P) * CS;
                            oPQ = PQ;
                            let PR = if PQ == -1e0f64 { 1.0 } else { 0.0 };
                            oPR = PR;
                        }
                        let PS = if CV == C { 1.0 } else { 0.0 };
                        oPS = PS;
                        if PS != 0.0 {
                        } else {
                            let PT = if P == AY { 1.0 } else { 0.0 };
                            oPT = PT;
                            let PX = if PT != 0.0 {
                                let PV = ((T - OI) * DA).sqrt();
                                PV
                            } else {
                                let PW = ((T - OI) * DA).powf(P);
                                PW
                            };
                            let PY = CS * (((T - OI) * DE) / PX);
                            oPY = PY;
                            let PZ = (OK * PY) * PY;
                            oPZ = PZ;
                        }
                        let PU = if CY > BO { 1.0 } else { 0.0 };
                        oPU = PU;
                        let QB;
                        if PU != 0.0 {
                            QB = AL;
                        } else {
                            let QA = if OJ > ((-BX) * CY) { 1.0 } else { 0.0 };
                            oQA = QA;
                            let QE;
                            if QA != 0.0 {
                                let QC = if DJ == AW { 1.0 } else { 0.0 };
                                oQC = QC;
                                let QI = if QC != 0.0 {
                                    let QF = OJ * DP;
                                    let QG = ((QF * QF) * QF) * QF;
                                    QG
                                } else {
                                    let QH = ((OJ * DP).abs()).powf(DJ);
                                    QH
                                };
                                let QJ = AL / (AL - QI);
                                QE = QJ;
                            } else {
                                let QD = DM + ((OJ + (BX * CY)) * DL);
                                QE = QD;
                            }
                            QB = QE;
                        }
                        oQB = QB;
                    }
                    if AF != 0.0 {
                    } else {
                        let QK = if DW == C { 1.0 } else { 0.0 };
                        oQK = QK;
                        let QL = if (if DV == C { 1.0 } else { 0.0 }) != 0.0 && QK != 0.0 { 1.0 } else { 0.0 };
                        oQL = QL;
                        if QL != 0.0 {
                        } else {
                            let QM = if Q == AY { 1.0 } else { 0.0 };
                            oQM = QM;
                            if QM != 0.0 {
                            } else {
                                let QN = AL - (AK * Q);
                                oQN = QN;
                            }
                        }
                        if QK != 0.0 {
                        } else {
                            let QO = (-Q) * EB;
                            oQO = QO;
                            let QP = if QO == -1e0f64 { 1.0 } else { 0.0 };
                            oQP = QP;
                        }
                        let QQ = if EE == C { 1.0 } else { 0.0 };
                        oQQ = QQ;
                        if QQ != 0.0 {
                        } else {
                            let QR = if Q == AY { 1.0 } else { 0.0 };
                            oQR = QR;
                            let QV = if QR != 0.0 {
                                let QT = ((U - OI) * EJ).sqrt();
                                QT
                            } else {
                                let QU = ((U - OI) * EJ).powf(Q);
                                QU
                            };
                            let QW = EB * (((U - OI) * EN) / QV);
                            oQW = QW;
                            let QX = (OK * QW) * QW;
                            oQX = QX;
                        }
                        let QS = if EH > BO { 1.0 } else { 0.0 };
                        oQS = QS;
                        let QZ;
                        if QS != 0.0 {
                            QZ = AL;
                        } else {
                            let QY = if OJ > ((-BX) * EH) { 1.0 } else { 0.0 };
                            oQY = QY;
                            let RC;
                            if QY != 0.0 {
                                let RA = if ES == AW { 1.0 } else { 0.0 };
                                oRA = RA;
                                let RG = if RA != 0.0 {
                                    let RD = OJ * EY;
                                    let RE = ((RD * RD) * RD) * RD;
                                    RE
                                } else {
                                    let RF = ((OJ * EY).abs()).powf(ES);
                                    RF
                                };
                                let RH = AL / (AL - RG);
                                RC = RH;
                            } else {
                                let RB = EV + ((OJ + (BX * EH)) * EU);
                                RC = RB;
                            }
                            QZ = RC;
                        }
                        oQZ = QZ;
                    }
                    if AP != 0.0 {
                        let RI = AY * parameters[12];
                        oRI = RI;
                    } else {
                    }
                } else {
                }
                if AO != 0.0 {
                } else {
                    let RJ = if (if (if O != 0.0 && AA != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AF != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oRJ = RJ;
                    if RJ != 0.0 {
                        let RN = (AW * AX) * AX;
                        oRN = RN;
                    } else {
                    }
                    if O != 0.0 {
                    } else {
                        let RO = if BC == C { 1.0 } else { 0.0 };
                        oRO = RO;
                        let RP = if (if BB == C { 1.0 } else { 0.0 }) != 0.0 && RO != 0.0 { 1.0 } else { 0.0 };
                        oRP = RP;
                        if RP != 0.0 {
                        } else {
                            let RQ = if W == AY { 1.0 } else { 0.0 };
                            oRQ = RQ;
                            if RQ != 0.0 {
                            } else {
                                let RR = AL - (AK * W);
                                oRR = RR;
                            }
                            if RQ != 0.0 {
                            } else {
                                let RT = W - RS;
                                oRT = RT;
                            }
                        }
                        if RO != 0.0 {
                        } else {
                            let RU = (-W) * BH;
                            oRU = RU;
                            let RV = if RU == -1e0f64 { 1.0 } else { 0.0 };
                            oRV = RV;
                            if RV != 0.0 {
                            } else {
                                let RX = RU - RS;
                                oRX = RX;
                            }
                        }
                        let RW = if BK == C { 1.0 } else { 0.0 };
                        oRW = RW;
                        if RW != 0.0 {
                        } else {
                            let RY = if W == AY { 1.0 } else { 0.0 };
                            oRY = RY;
                            if RY != 0.0 {
                            } else {
                                let SA = W - RS;
                                oSA = SA;
                            }
                        }
                        let RZ = if BN > BO { 1.0 } else { 0.0 };
                        oRZ = RZ;
                        if RZ != 0.0 {
                        } else {
                            let SB = (-BX) * BN;
                            oSB = SB;
                        }
                        let SD = if SC == AY { 1.0 } else { 0.0 };
                        oSD = SD;
                        if SD != 0.0 {
                        } else {
                            let SE = SC - RS;
                            oSE = SE;
                        }
                    }
                    if AA != 0.0 {
                    } else {
                        let SF = if CN == C { 1.0 } else { 0.0 };
                        oSF = SF;
                        let SG = if (if CM == C { 1.0 } else { 0.0 }) != 0.0 && SF != 0.0 { 1.0 } else { 0.0 };
                        oSG = SG;
                        if SG != 0.0 {
                        } else {
                            let SH = if P == AY { 1.0 } else { 0.0 };
                            oSH = SH;
                            if SH != 0.0 {
                            } else {
                                let SI = AL - (AK * P);
                                oSI = SI;
                            }
                            if SH != 0.0 {
                            } else {
                                let SJ = P - RS;
                                oSJ = SJ;
                            }
                        }
                        if SF != 0.0 {
                        } else {
                            let SK = (-P) * CS;
                            oSK = SK;
                            let SL = if SK == -1e0f64 { 1.0 } else { 0.0 };
                            oSL = SL;
                            if SL != 0.0 {
                            } else {
                                let SN = SK - RS;
                                oSN = SN;
                            }
                        }
                        let SM = if CV == C { 1.0 } else { 0.0 };
                        oSM = SM;
                        if SM != 0.0 {
                        } else {
                            let SO = if P == AY { 1.0 } else { 0.0 };
                            oSO = SO;
                            if SO != 0.0 {
                            } else {
                                let SQ = P - RS;
                                oSQ = SQ;
                            }
                        }
                        let SP = if CY > BO { 1.0 } else { 0.0 };
                        oSP = SP;
                        if SP != 0.0 {
                        } else {
                            let SR = (-BX) * CY;
                            oSR = SR;
                        }
                        let ST = if SS == AY { 1.0 } else { 0.0 };
                        oST = ST;
                        if ST != 0.0 {
                        } else {
                            let SU = SS - RS;
                            oSU = SU;
                        }
                    }
                    if AF != 0.0 {
                    } else {
                        let SV = if DW == C { 1.0 } else { 0.0 };
                        oSV = SV;
                        let SW = if (if DV == C { 1.0 } else { 0.0 }) != 0.0 && SV != 0.0 { 1.0 } else { 0.0 };
                        oSW = SW;
                        if SW != 0.0 {
                        } else {
                            let SX = if Q == AY { 1.0 } else { 0.0 };
                            oSX = SX;
                            if SX != 0.0 {
                            } else {
                                let SY = AL - (AK * Q);
                                oSY = SY;
                            }
                            if SX != 0.0 {
                            } else {
                                let SZ = Q - RS;
                                oSZ = SZ;
                            }
                        }
                        if SV != 0.0 {
                        } else {
                            let TA = (-Q) * EB;
                            oTA = TA;
                            let TB = if TA == -1e0f64 { 1.0 } else { 0.0 };
                            oTB = TB;
                            if TB != 0.0 {
                            } else {
                                let TD = TA - RS;
                                oTD = TD;
                            }
                        }
                        let TC = if EE == C { 1.0 } else { 0.0 };
                        oTC = TC;
                        if TC != 0.0 {
                        } else {
                            let TE = if Q == AY { 1.0 } else { 0.0 };
                            oTE = TE;
                            if TE != 0.0 {
                            } else {
                                let TG = Q - RS;
                                oTG = TG;
                            }
                        }
                        let TF = if EH > BO { 1.0 } else { 0.0 };
                        oTF = TF;
                        if TF != 0.0 {
                        } else {
                            let TH = (-BX) * EH;
                            oTH = TH;
                        }
                        if A != 0.0 {
                        } else {
                            let TJ = if TI == AY { 1.0 } else { 0.0 };
                            oTJ = TJ;
                            if TJ != 0.0 {
                            } else {
                                let TK = TI - RS;
                                oTK = TK;
                            }
                        }
                    }
                }
                let RK = parameters[1] * N;
                let RL = RK * parameters[7];
                let RM = RK * parameters[8];
            [D, E, G, H, J, K, M, O, AA, AF, AM, AN, oAP, oAT, oAU, oBD, oBE, oBF, oBG, oBI, oBJ, oBL, oBM, oBV, oBW, oBP, oBY, oCB, oBZ, oCO, oCP, oCQ, oCR, oCT, oCU, oCW, oCX, oDF, oDG, oCZ, oDH, oDK, oDI, oDX, oDY, oDZ, oEA, oEC, oED, oEF, oEG, oEO, oEP, oEI, oEQ, oET, oER, oFH, oFI, oFM, oFN, oFO, oFP, oFQ, oFR, oFS, oFT, oFY, oFZ, oFU, oGA, oGC, oGB, oGK, oGL, oGM, oGN, oGO, oGP, oGQ, oGR, oGW, oGX, oGS, oGY, oHA, oGZ, oHI, oHJ, oHK, oHL, oHM, oHN, oHO, oHP, oHU, oHV, oHQ, oHW, oHY, oHX, oIJ, oIK, oIO, oIP, oIQ, oIR, oIS, oIT, oIU, oIV, oJA, oJB, oIW, oJC, oJE, oJD, oJM, oJN, oJO, oJP, oJQ, oJR, oJS, oJT, oJY, oJZ, oJU, oKA, oKC, oKB, oKK, oKL, oKM, oKN, oKO, oKP, oKQ, oKR, oKW, oKX, oKS, oKY, oLA, oKZ, oLO, oLP, oLQ, oLR, oLS, oLT, oLU, oLV, oMA, oMB, oLW, oMC, oME, oMD, oMM, oMN, oMO, oMP, oMQ, oMR, oMS, oMT, oMY, oMZ, oMU, oNA, oNC, oNB, oNK, oNL, oNM, oNN, oNO, oNP, oNQ, oNR, oNW, oNX, oNS, oNY, oOA, oNZ, oOO, oOP, oOQ, oOR, oOS, oOT, oOU, oOV, oPA, oPB, oOW, oPC, oPE, oPD, oPM, oPN, oPO, oPP, oPQ, oPR, oPS, oPT, oPY, oPZ, oPU, oQA, oQC, oQB, oQK, oQL, oQM, oQN, oQO, oQP, oQQ, oQR, oQW, oQX, oQS, oQY, oRA, oQZ, oRI, oRJ, oRN, oRO, oRP, oRQ, oRR, oRU, oRV, oRW, oRY, oRZ, oSB, oSD, oSF, oSG, oSH, oSI, oSK, oSL, oSM, oSO, oSP, oSR, oST, oSV, oSW, oSX, oSY, oTA, oTB, oTC, oTE, oTF, oTH, oTJ, RL, RM, oRT, oRX, oSA, oSE, oSJ, oSN, oSQ, oSU, oSZ, oTD, oTG, oTK]
        };
        self.canonical_staged[221] = produced[0];
        self.canonical_staged[16] = produced[1];
        self.canonical_staged[222] = produced[2];
        self.canonical_staged[17] = produced[3];
        self.canonical_staged[223] = produced[4];
        self.canonical_staged[18] = produced[5];
        self.canonical_staged[224] = produced[6];
        self.canonical_staged[230] = produced[7];
        self.canonical_staged[231] = produced[8];
        self.canonical_staged[232] = produced[9];
        self.canonical_staged[19] = produced[10];
        self.canonical_staged[157] = produced[11];
        self.canonical_staged[234] = produced[12];
        self.canonical_staged[237] = produced[13];
        self.canonical_staged[21] = produced[14];
        self.canonical_staged[241] = produced[15];
        self.canonical_staged[239] = produced[16];
        self.canonical_staged[240] = produced[17];
        self.canonical_staged[23] = produced[18];
        self.canonical_staged[27] = produced[19];
        self.canonical_staged[242] = produced[20];
        self.canonical_staged[243] = produced[21];
        self.canonical_staged[247] = produced[22];
        self.canonical_staged[29] = produced[23];
        self.canonical_staged[30] = produced[24];
        self.canonical_staged[248] = produced[25];
        self.canonical_staged[251] = produced[26];
        self.canonical_staged[252] = produced[27];
        self.canonical_staged[35] = produced[28];
        self.canonical_staged[255] = produced[29];
        self.canonical_staged[253] = produced[30];
        self.canonical_staged[254] = produced[31];
        self.canonical_staged[36] = produced[32];
        self.canonical_staged[40] = produced[33];
        self.canonical_staged[256] = produced[34];
        self.canonical_staged[257] = produced[35];
        self.canonical_staged[261] = produced[36];
        self.canonical_staged[42] = produced[37];
        self.canonical_staged[43] = produced[38];
        self.canonical_staged[262] = produced[39];
        self.canonical_staged[265] = produced[40];
        self.canonical_staged[266] = produced[41];
        self.canonical_staged[47] = produced[42];
        self.canonical_staged[269] = produced[43];
        self.canonical_staged[267] = produced[44];
        self.canonical_staged[268] = produced[45];
        self.canonical_staged[48] = produced[46];
        self.canonical_staged[52] = produced[47];
        self.canonical_staged[270] = produced[48];
        self.canonical_staged[271] = produced[49];
        self.canonical_staged[275] = produced[50];
        self.canonical_staged[54] = produced[51];
        self.canonical_staged[55] = produced[52];
        self.canonical_staged[276] = produced[53];
        self.canonical_staged[279] = produced[54];
        self.canonical_staged[280] = produced[55];
        self.canonical_staged[59] = produced[56];
        self.canonical_staged[283] = produced[57];
        self.canonical_staged[61] = produced[58];
        self.canonical_staged[287] = produced[59];
        self.canonical_staged[285] = produced[60];
        self.canonical_staged[286] = produced[61];
        self.canonical_staged[62] = produced[62];
        self.canonical_staged[63] = produced[63];
        self.canonical_staged[288] = produced[64];
        self.canonical_staged[289] = produced[65];
        self.canonical_staged[293] = produced[66];
        self.canonical_staged[64] = produced[67];
        self.canonical_staged[65] = produced[68];
        self.canonical_staged[294] = produced[69];
        self.canonical_staged[297] = produced[70];
        self.canonical_staged[298] = produced[71];
        self.canonical_staged[66] = produced[72];
        self.canonical_staged[301] = produced[73];
        self.canonical_staged[299] = produced[74];
        self.canonical_staged[300] = produced[75];
        self.canonical_staged[67] = produced[76];
        self.canonical_staged[68] = produced[77];
        self.canonical_staged[302] = produced[78];
        self.canonical_staged[303] = produced[79];
        self.canonical_staged[307] = produced[80];
        self.canonical_staged[69] = produced[81];
        self.canonical_staged[70] = produced[82];
        self.canonical_staged[308] = produced[83];
        self.canonical_staged[311] = produced[84];
        self.canonical_staged[312] = produced[85];
        self.canonical_staged[71] = produced[86];
        self.canonical_staged[315] = produced[87];
        self.canonical_staged[313] = produced[88];
        self.canonical_staged[314] = produced[89];
        self.canonical_staged[72] = produced[90];
        self.canonical_staged[73] = produced[91];
        self.canonical_staged[316] = produced[92];
        self.canonical_staged[317] = produced[93];
        self.canonical_staged[321] = produced[94];
        self.canonical_staged[74] = produced[95];
        self.canonical_staged[75] = produced[96];
        self.canonical_staged[322] = produced[97];
        self.canonical_staged[325] = produced[98];
        self.canonical_staged[326] = produced[99];
        self.canonical_staged[76] = produced[100];
        self.canonical_staged[329] = produced[101];
        self.canonical_staged[78] = produced[102];
        self.canonical_staged[333] = produced[103];
        self.canonical_staged[331] = produced[104];
        self.canonical_staged[332] = produced[105];
        self.canonical_staged[79] = produced[106];
        self.canonical_staged[80] = produced[107];
        self.canonical_staged[334] = produced[108];
        self.canonical_staged[335] = produced[109];
        self.canonical_staged[339] = produced[110];
        self.canonical_staged[81] = produced[111];
        self.canonical_staged[82] = produced[112];
        self.canonical_staged[340] = produced[113];
        self.canonical_staged[343] = produced[114];
        self.canonical_staged[344] = produced[115];
        self.canonical_staged[83] = produced[116];
        self.canonical_staged[347] = produced[117];
        self.canonical_staged[345] = produced[118];
        self.canonical_staged[346] = produced[119];
        self.canonical_staged[84] = produced[120];
        self.canonical_staged[85] = produced[121];
        self.canonical_staged[348] = produced[122];
        self.canonical_staged[349] = produced[123];
        self.canonical_staged[353] = produced[124];
        self.canonical_staged[86] = produced[125];
        self.canonical_staged[87] = produced[126];
        self.canonical_staged[354] = produced[127];
        self.canonical_staged[357] = produced[128];
        self.canonical_staged[358] = produced[129];
        self.canonical_staged[88] = produced[130];
        self.canonical_staged[361] = produced[131];
        self.canonical_staged[359] = produced[132];
        self.canonical_staged[360] = produced[133];
        self.canonical_staged[89] = produced[134];
        self.canonical_staged[90] = produced[135];
        self.canonical_staged[362] = produced[136];
        self.canonical_staged[363] = produced[137];
        self.canonical_staged[367] = produced[138];
        self.canonical_staged[91] = produced[139];
        self.canonical_staged[92] = produced[140];
        self.canonical_staged[368] = produced[141];
        self.canonical_staged[371] = produced[142];
        self.canonical_staged[372] = produced[143];
        self.canonical_staged[93] = produced[144];
        self.canonical_staged[378] = produced[145];
        self.canonical_staged[376] = produced[146];
        self.canonical_staged[377] = produced[147];
        self.canonical_staged[94] = produced[148];
        self.canonical_staged[95] = produced[149];
        self.canonical_staged[379] = produced[150];
        self.canonical_staged[380] = produced[151];
        self.canonical_staged[384] = produced[152];
        self.canonical_staged[96] = produced[153];
        self.canonical_staged[97] = produced[154];
        self.canonical_staged[385] = produced[155];
        self.canonical_staged[388] = produced[156];
        self.canonical_staged[389] = produced[157];
        self.canonical_staged[98] = produced[158];
        self.canonical_staged[392] = produced[159];
        self.canonical_staged[390] = produced[160];
        self.canonical_staged[391] = produced[161];
        self.canonical_staged[99] = produced[162];
        self.canonical_staged[100] = produced[163];
        self.canonical_staged[393] = produced[164];
        self.canonical_staged[394] = produced[165];
        self.canonical_staged[398] = produced[166];
        self.canonical_staged[101] = produced[167];
        self.canonical_staged[102] = produced[168];
        self.canonical_staged[399] = produced[169];
        self.canonical_staged[402] = produced[170];
        self.canonical_staged[403] = produced[171];
        self.canonical_staged[103] = produced[172];
        self.canonical_staged[406] = produced[173];
        self.canonical_staged[404] = produced[174];
        self.canonical_staged[405] = produced[175];
        self.canonical_staged[104] = produced[176];
        self.canonical_staged[105] = produced[177];
        self.canonical_staged[407] = produced[178];
        self.canonical_staged[408] = produced[179];
        self.canonical_staged[412] = produced[180];
        self.canonical_staged[106] = produced[181];
        self.canonical_staged[107] = produced[182];
        self.canonical_staged[413] = produced[183];
        self.canonical_staged[416] = produced[184];
        self.canonical_staged[417] = produced[185];
        self.canonical_staged[108] = produced[186];
        self.canonical_staged[423] = produced[187];
        self.canonical_staged[421] = produced[188];
        self.canonical_staged[422] = produced[189];
        self.canonical_staged[109] = produced[190];
        self.canonical_staged[110] = produced[191];
        self.canonical_staged[424] = produced[192];
        self.canonical_staged[425] = produced[193];
        self.canonical_staged[429] = produced[194];
        self.canonical_staged[111] = produced[195];
        self.canonical_staged[112] = produced[196];
        self.canonical_staged[430] = produced[197];
        self.canonical_staged[433] = produced[198];
        self.canonical_staged[434] = produced[199];
        self.canonical_staged[113] = produced[200];
        self.canonical_staged[437] = produced[201];
        self.canonical_staged[435] = produced[202];
        self.canonical_staged[436] = produced[203];
        self.canonical_staged[114] = produced[204];
        self.canonical_staged[115] = produced[205];
        self.canonical_staged[438] = produced[206];
        self.canonical_staged[439] = produced[207];
        self.canonical_staged[443] = produced[208];
        self.canonical_staged[116] = produced[209];
        self.canonical_staged[117] = produced[210];
        self.canonical_staged[444] = produced[211];
        self.canonical_staged[447] = produced[212];
        self.canonical_staged[448] = produced[213];
        self.canonical_staged[118] = produced[214];
        self.canonical_staged[451] = produced[215];
        self.canonical_staged[449] = produced[216];
        self.canonical_staged[450] = produced[217];
        self.canonical_staged[119] = produced[218];
        self.canonical_staged[120] = produced[219];
        self.canonical_staged[452] = produced[220];
        self.canonical_staged[453] = produced[221];
        self.canonical_staged[457] = produced[222];
        self.canonical_staged[121] = produced[223];
        self.canonical_staged[122] = produced[224];
        self.canonical_staged[458] = produced[225];
        self.canonical_staged[461] = produced[226];
        self.canonical_staged[462] = produced[227];
        self.canonical_staged[123] = produced[228];
        self.canonical_staged[124] = produced[229];
        self.canonical_staged[471] = produced[230];
        self.canonical_staged[158] = produced[231];
        self.canonical_staged[481] = produced[232];
        self.canonical_staged[479] = produced[233];
        self.canonical_staged[480] = produced[234];
        self.canonical_staged[161] = produced[235];
        self.canonical_staged[165] = produced[236];
        self.canonical_staged[482] = produced[237];
        self.canonical_staged[483] = produced[238];
        self.canonical_staged[484] = produced[239];
        self.canonical_staged[485] = produced[240];
        self.canonical_staged[168] = produced[241];
        self.canonical_staged[486] = produced[242];
        self.canonical_staged[489] = produced[243];
        self.canonical_staged[487] = produced[244];
        self.canonical_staged[488] = produced[245];
        self.canonical_staged[171] = produced[246];
        self.canonical_staged[175] = produced[247];
        self.canonical_staged[490] = produced[248];
        self.canonical_staged[491] = produced[249];
        self.canonical_staged[492] = produced[250];
        self.canonical_staged[493] = produced[251];
        self.canonical_staged[178] = produced[252];
        self.canonical_staged[494] = produced[253];
        self.canonical_staged[497] = produced[254];
        self.canonical_staged[495] = produced[255];
        self.canonical_staged[496] = produced[256];
        self.canonical_staged[181] = produced[257];
        self.canonical_staged[185] = produced[258];
        self.canonical_staged[498] = produced[259];
        self.canonical_staged[499] = produced[260];
        self.canonical_staged[500] = produced[261];
        self.canonical_staged[501] = produced[262];
        self.canonical_staged[188] = produced[263];
        self.canonical_staged[502] = produced[264];
        self.canonical_staged[194] = produced[265];
        self.canonical_staged[195] = produced[266];
        self.canonical_staged[196] = produced[267];
        self.canonical_staged[197] = produced[268];
        self.canonical_staged[198] = produced[269];
        self.canonical_staged[199] = produced[270];
        self.canonical_staged[200] = produced[271];
        self.canonical_staged[201] = produced[272];
        self.canonical_staged[202] = produced[273];
        self.canonical_staged[203] = produced[274];
        self.canonical_staged[204] = produced[275];
        self.canonical_staged[205] = produced[276];
        self.canonical_staged[206] = produced[277];
        self.canonical_staged[207] = produced[278];
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
        let produced: [f64; 173] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let A = staged[213];
                let C = staged[0];
                let F = 1e0f64;
                let M = 5e-1f64;
                let T = parameters[18];
                let U = 2e0f64;
                let X = parameters[19];
                let Z = parameters[20];
                let AB = 5e-2f64;
                let AI = parameters[21];
                let AK = parameters[22];
                let AM = parameters[23];
                let BH = 0e0f64;
                let BO = staged[12];
                let BV = 0e0f64;
                let BW = 0e0f64;
                let BX = 0e0f64;
                let CB = staged[16];
                let CE = parameters[12];
                let CG = 1e8f64;
                let CI = staged[17];
                let CN = staged[18];
                let CU = 2.3025850929940458e2f64;
                let CZ = staged[230];
                let DA = 3.333333333333333e-1f64;
                let DB = 1e-100f64;
                let DE = 1e100f64;
                let DJ = staged[231];
                let DM = staged[232];
                let DQ = 1e-1f64;
                let DT = staged[233];
                let DU = staged[234];
                let EK = staged[20];
                let EZ = staged[237];
                let FI = 3e0f64;
                let FO = 4e0f64;
                let FR = staged[239];
                let FZ = staged[240];
                let GF = staged[241];
                let GJ = staged[24];
                let GN = staged[25];
                let GQ = parameters[30];
                let GS = staged[26];
                let GU = 6.66666666666667e-1f64;
                let HB = staged[242];
                let HD = staged[243];
                let HI = 3.75e-1f64;
                let HO = 5.178164370971076e-1f64;
                let HX = 2.9214664e-1f64;
                let HZ = 2.6992878119627894e-1f64;
                let IA = 4.3792457880372104e-1f64;
                let IE = parameters[35];
                let IQ = parameters[41];
                let IW = parameters[10];
                let IZ = staged[253];
                let JH = staged[254];
                let JN = staged[255];
                let JR = staged[37];
                let JV = staged[38];
                let JY = parameters[31];
                let KA = staged[39];
                let KI = staged[256];
                let KK = staged[257];
                let LG = parameters[36];
                let LS = parameters[42];
                let MA = staged[267];
                let MJ = staged[268];
                let MP = staged[269];
                let MT = staged[49];
                let MX = staged[50];
                let NA = parameters[32];
                let NC = staged[51];
                let NK = staged[270];
                let NM = staged[271];
                let OI = parameters[37];
                let OU = parameters[43];
                let PB = staged[60];
                let PQ = staged[283];
                let QG = staged[285];
                let QO = staged[286];
                let QU = staged[287];
                let RL = staged[288];
                let RN = staged[289];
                let TB = staged[299];
                let TJ = staged[300];
                let TP = staged[301];
                let UG = staged[302];
                let UI = staged[303];
                let VW = staged[313];
                let WF = staged[314];
                let WL = staged[315];
                let XC = staged[316];
                let XE = staged[317];
                let YR = staged[77];
                let ZG = staged[329];
                let ZW = staged[331];
                let AAE = staged[332];
                let AAK = staged[333];
                let ABB = staged[334];
                let ABD = staged[335];
                let ACR = staged[345];
                let ACZ = staged[346];
                let ADF = staged[347];
                let ADW = staged[348];
                let ADY = staged[349];
                let AFM = staged[359];
                let AFV = staged[360];
                let AGB = staged[361];
                let AGS = staged[362];
                let AGU = staged[363];
                let AIV = 1.0f64;
                let AJL = staged[376];
                let AJT = staged[377];
                let AJZ = staged[378];
                let AKQ = staged[379];
                let AKS = staged[380];
                let AMG = staged[390];
                let AMO = staged[391];
                let AMU = staged[392];
                let ANL = staged[393];
                let ANN = staged[394];
                let APB = staged[404];
                let APK = staged[405];
                let APQ = staged[406];
                let AQH = staged[407];
                let AQJ = staged[408];
                let ARW = 2e-1f64;
                let ASL = 1.0f64;
                let ATB = staged[421];
                let ATJ = staged[422];
                let ATP = staged[423];
                let AUG = staged[424];
                let AUI = staged[425];
                let AVW = staged[435];
                let AWE = staged[436];
                let AWK = staged[437];
                let AXB = staged[438];
                let AXD = staged[439];
                let AYR = staged[449];
                let AZA = staged[450];
                let AZG = staged[451];
                let AZX = staged[452];
                let AZZ = staged[453];
                let BBX = 1e-3f64;
                let BDJ = 1e-21f64;
                let BDK = staged[124];
                let BDX = staged[471];
                let BEF = 1e0f64;
                let BEP = staged[481];
                let BER = staged[483];
                let BET = staged[489];
                let BEV = staged[491];
                let BEX = staged[497];
                let BEZ = staged[499];
                let mut oCX = 0.0;
                let mut oEL = 0.0;
                let mut oER = 0.0;
                let mut oFB = 0.0;
                let mut oHN = 0.0;
                let mut oHT = 0.0;
                let mut oIC = 0.0;
                let mut oIM = 0.0;
                let mut oIO = 0.0;
                let mut oKT = 0.0;
                let mut oKY = 0.0;
                let mut oLE = 0.0;
                let mut oLO = 0.0;
                let mut oLQ = 0.0;
                let mut oNV = 0.0;
                let mut oOA = 0.0;
                let mut oOG = 0.0;
                let mut oOQ = 0.0;
                let mut oOS = 0.0;
                let mut oPC = 0.0;
                let mut oPI = 0.0;
                let mut oPS = 0.0;
                let mut oRW = 0.0;
                let mut oSB = 0.0;
                let mut oSH = 0.0;
                let mut oSQ = 0.0;
                let mut oSS = 0.0;
                let mut oUR = 0.0;
                let mut oUW = 0.0;
                let mut oVC = 0.0;
                let mut oVL = 0.0;
                let mut oVN = 0.0;
                let mut oXN = 0.0;
                let mut oXS = 0.0;
                let mut oXY = 0.0;
                let mut oYH = 0.0;
                let mut oYJ = 0.0;
                let mut oYS = 0.0;
                let mut oYY = 0.0;
                let mut oZI = 0.0;
                let mut oABM = 0.0;
                let mut oABR = 0.0;
                let mut oABX = 0.0;
                let mut oACG = 0.0;
                let mut oACI = 0.0;
                let mut oAEH = 0.0;
                let mut oAEM = 0.0;
                let mut oAES = 0.0;
                let mut oAFB = 0.0;
                let mut oAFD = 0.0;
                let mut oAHD = 0.0;
                let mut oAHI = 0.0;
                let mut oAHO = 0.0;
                let mut oAHX = 0.0;
                let mut oAHZ = 0.0;
                let mut oAIH = 0.0;
                let mut oAIN = 0.0;
                let mut oAIX = 0.0;
                let mut oALB = 0.0;
                let mut oALG = 0.0;
                let mut oALM = 0.0;
                let mut oALV = 0.0;
                let mut oALX = 0.0;
                let mut oANW = 0.0;
                let mut oAOB = 0.0;
                let mut oAOH = 0.0;
                let mut oAOQ = 0.0;
                let mut oAOS = 0.0;
                let mut oAQS = 0.0;
                let mut oAQX = 0.0;
                let mut oARD = 0.0;
                let mut oARM = 0.0;
                let mut oARO = 0.0;
                let mut oARX = 0.0;
                let mut oASD = 0.0;
                let mut oASN = 0.0;
                let mut oAUR = 0.0;
                let mut oAUW = 0.0;
                let mut oAVC = 0.0;
                let mut oAVL = 0.0;
                let mut oAVN = 0.0;
                let mut oAXM = 0.0;
                let mut oAXR = 0.0;
                let mut oAXX = 0.0;
                let mut oAYG = 0.0;
                let mut oAYI = 0.0;
                let mut oBAI = 0.0;
                let mut oBAN = 0.0;
                let mut oBAT = 0.0;
                let mut oBBC = 0.0;
                let mut oBBE = 0.0;
                let mut oBBM = 0.0;
                let mut oBBW = 0.0;
                let mut oBBY = 0.0;
                let mut oBCH = 0.0;
                let mut oBCM = 0.0;
                let mut oBCU = 0.0;
                let mut oBDF = 0.0;
                let mut oBDH = 0.0;
                let mut oBDY = 0.0;
                let mut oBDZ = 0.0;
                let mut oBEA = 0.0;
                let mut oBEB = 0.0;
                let mut oBEC = 0.0;
                let mut oBED = 0.0;
                let mut oBEE = 0.0;
                let mut oBEG = 0.0;
                let mut oBEH = 0.0;
                let mut oBEI = 0.0;
                let mut oBEJ = 0.0;
                let mut oBEK = 0.0;
                let mut oBEL = 0.0;
                let mut oBEM = 0.0;
                let mut oBEN = 0.0;
                let mut oBEO = 0.0;
                let mut oBEQ = 0.0;
                let mut oBES = 0.0;
                let mut oBEU = 0.0;
                let mut oBEW = 0.0;
                let mut oBEY = 0.0;
                let mut oBFA = 0.0;
                let B = if ((temperature + parameters[2]) + parameters[9]) >= 2.3149999999999977e1f64 { ((temperature + parameters[2]) + parameters[9]) } else { 2.3149999999999977e1f64 };
                let D = B / C;
                let E = 8.61726105451295e-5f64 * B;
                let G = F / E;
                let H = (-((7.02e-4f64 * B) * B)) / (1.108e3f64 + B);
                let I = parameters[24] + H;
                let J = parameters[25] + H;
                let K = parameters[26] + H;
                let L = D * (D.sqrt());
                let N = L * ((M * (staged[1] - (I * G))).exp());
                let O = L * ((M * (staged[2] - (J * G))).exp());
                let P = L * ((M * (staged[3] - (K * G))).exp());
                let Q = (parameters[27] * N) * N;
                let R = (parameters[28] * O) * O;
                let S = (parameters[29] * P) * P;
                let V = U * E;
                let W = (T * D) - (V * (N.ln()));
                let Y = (X * D) - (V * (O.ln()));
                let AA = (Z * D) - (V * (P.ln()));
                let AC = W + (E * ((F + (((AB - W) * G).exp())).ln()));
                let AD = Y + (E * ((F + (((AB - Y) * G).exp())).ln()));
                let AE = AA + (E * ((F + (((AB - AA) * G).exp())).ln()));
                let AF = F / AC;
                let AG = F / AD;
                let AH = F / AE;
                let AJ = parameters[15] * ((T * AF).powf(AI));
                let AL = parameters[16] * ((X * AG).powf(AK));
                let AN = parameters[17] * ((Z * AH).powf(AM));
                let AO = (AJ * AC) * staged[4];
                let AP = (AL * AD) * staged[5];
                let AQ = (AN * AE) * staged[6];
                let AR = U * AJ;
                let AS = U * AL;
                let AT = U * AN;
                let AU = if (M * I) >= E { (M * I) } else { E };
                let AV = if (M * J) >= E { (M * J) } else { E };
                let AW = if (M * K) >= E { (M * K) } else { E };
                let AX = AU * G;
                let AY = AV * G;
                let AZ = AW * G;
                let BA = ((staged[7] * ((AU * AU) * AU)).sqrt()) / 3.1637150399999996e-34f64;
                let BB = ((staged[8] * ((AV * AV) * AV)).sqrt()) / 3.1637150399999996e-34f64;
                let BC = ((staged[9] * ((AW * AW) * AW)).sqrt()) / 3.1637150399999996e-34f64;
                let BD = B - C;
                let BE = parameters[44] * (F + (parameters[47] * BD));
                let BF = parameters[45] * (F + (parameters[48] * BD));
                let BG = parameters[46] * (F + (parameters[49] * BD));
                let BI = if BE > BH { 1.0 } else { 0.0 };
                let BJ = if BI != 0.0 {
                    BE
                } else {
                    BH
                };
                let BK = if BF > BH { 1.0 } else { 0.0 };
                let BL = if BK != 0.0 {
                    BF
                } else {
                    BH
                };
                let BM = if BG > BH { 1.0 } else { 0.0 };
                let BN = if BM != 0.0 {
                    BG
                } else {
                    BH
                };
                let BY;
                let BZ;
                let CA;
                if A != 0.0 {
                    let BP = (BO * D) - (V * ((L * ((M * (staged[11] - ((staged[10] + H) * G))).exp())).ln()));
                    let BQ = BP + (E * ((F + (((AB - BP) * G).exp())).ln()));
                    let BR = F / BQ;
                    let BS = staged[14] * ((BO * BR).powf(staged[13]));
                    let BT = (BS * BQ) * staged[15];
                    let BU = U * BS;
                    BY = BR;
                    BZ = BT;
                    CA = BU;
                } else {
                    BY = BV;
                    BZ = BW;
                    CA = BX;
                }
                let CC = Q * CB;
                let CD = if CC > BH { 1.0 } else { 0.0 };
                let CH = if CD != 0.0 {
                    let CF = E * (((CE / CC) + F).ln());
                    CF
                } else {
                    CG
                };
                let CJ = R * CI;
                let CK = if CJ > BH { 1.0 } else { 0.0 };
                let CM = if CK != 0.0 {
                    let CL = E * (((CE / CJ) + F).ln());
                    CL
                } else {
                    CG
                };
                let CO = S * CN;
                let CP = if CO > BH { 1.0 } else { 0.0 };
                let CR = if CP != 0.0 {
                    let CQ = E * (((CE / CO) + F).ln());
                    CQ
                } else {
                    CG
                };
                let CS = if (if CH <= CM { CH } else { CM }) <= CR { (if CH <= CM { CH } else { CM }) } else { CR };
                let CT = CS * G;
                let CV = if (CT.abs()) < CU { 1.0 } else { 0.0 };
                let CY;
                if CV != 0.0 {
                    let CW = CT.exp();
                    CY = CW;
                } else {
                    let CX = if CT < BH { 1.0 } else { 0.0 };
                    oCX = CX;
                    let DG = if CX != 0.0 {
                        let DC = DB / (F + ((-2.3025850929940458e2f64 - CT) * (F + (M * ((-2.3025850929940458e2f64 - CT) * (F + ((-2.3025850929940458e2f64 - CT) * DA)))))));
                        DC
                    } else {
                        let DD = CT - CU;
                        let DF = DE * (F + (DD * (F + (M * (DD * (F + (DD * DA)))))));
                        DF
                    };
                    CY = DG;
                }
                let DI = if CZ != 0.0 {
                    let DH = AD + AE;
                    DH
                } else {
                    AC
                };
                let DL = if DJ != 0.0 {
                    let DK = AC + AE;
                    DK
                } else {
                    AD
                };
                let DO = if DM != 0.0 {
                    let DN = AC + AD;
                    DN
                } else {
                    AE
                };
                let DP = if (if DI <= DL { DI } else { DL }) <= DO { (if DI <= DL { DI } else { DL }) } else { DO };
                let DR = DP * DQ;
                let DS = DP * staged[19];
                let DV;
                let DW;
                let DX;
                let DY;
                let DZ;
                let EA;
                let EB;
                let EC;
                let ED;
                let EE;
                let EF;
                let EG;
                let EH;
                let EI;
                let EJ;
                if DT != 0.0 {
                    let EM;
                    let EN;
                    let EO;
                    let EP;
                    if DU != 0.0 {
                        let EL = if EK < CS { 1.0 } else { 0.0 };
                        oEL = EL;
                        let EV;
                        let EW;
                        let EX;
                        if EL != 0.0 {
                            let EQ = EK * G;
                            let ER = if ((-5e-1f64 * EQ).abs()) < CU { 1.0 } else { 0.0 };
                            oER = ER;
                            let FC;
                            if ER != 0.0 {
                                let FA = (-5e-1f64 * EQ).exp();
                                FC = FA;
                            } else {
                                let FB = if (-5e-1f64 * EQ) < BH { 1.0 } else { 0.0 };
                                oFB = FB;
                                let FH = if FB != 0.0 {
                                    let FF = DB / (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * EQ)) * (F + (M * ((-2.3025850929940458e2f64 - (-5e-1f64 * EQ)) * (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * EQ)) * DA)))))));
                                    FF
                                } else {
                                    let FG = DE * (F + (((-5e-1f64 * EQ) - CU) * (F + (M * (((-5e-1f64 * EQ) - CU) * (F + (((-5e-1f64 * EQ) - CU) * DA)))))));
                                    FG
                                };
                                FC = FH;
                            }
                            let FD = F / FC;
                            let FE = FD * FD;
                            EV = FE;
                            EW = FC;
                            EX = FD;
                        } else {
                            let ES = (F + ((EK - CS) * G)) * CY;
                            let ET = ES.sqrt();
                            let EU = F / ET;
                            EV = ES;
                            EW = EU;
                            EX = ET;
                        }
                        let EY = EV - F;
                        let FL = if EZ != 0.0 {
                            let FJ = U * (E * (((U + EW) + (((EW + F) * (EW + FI)).sqrt())).ln()));
                            FJ
                        } else {
                            let FK = staged[21] + (U * (E * ((((U * EX) + F) + (((F + EX) * (F + (FI * EX))).sqrt())).ln())));
                            FK
                        };
                        let FM = DP - FL;
                        let FN = EK - FM;
                        let FP = M * ((EK + FM) - (((FN * FN) + ((FO * E) * E)).sqrt()));
                        EM = EY;
                        EN = FP;
                        EO = FL;
                        EP = EX;
                    } else {
                        EM = BH;
                        EN = BH;
                        EO = BH;
                        EP = BH;
                    }
                    let FS;
                    let FT;
                    let FU;
                    let FV;
                    let FW;
                    if CZ != 0.0 {
                        FS = BH;
                        FT = BH;
                        FU = BH;
                        FV = BH;
                        FW = BH;
                    } else {
                        let FQ = Q * EM;
                        let GA;
                        let GB;
                        let GC;
                        let GD;
                        let GE;
                        if FR != 0.0 {
                            GA = BH;
                            GB = BH;
                            GC = BH;
                            GD = BH;
                            GE = BH;
                        } else {
                            let FX = AC - EN;
                            let FY = F - ((F - (EO / FX)).sqrt());
                            let GH = if FZ != 0.0 {
                                BH
                            } else {
                                let GG = ((((FY * FY) * (FY.ln())) / (F - FY)) + FY) * staged[23];
                                GG
                            };
                            let GI = FY + GH;
                            let GM = if FZ != 0.0 {
                                let GK = (FX * GJ).sqrt();
                                GK
                            } else {
                                let GL = (FX * GJ).powf(AI);
                                GL
                            };
                            let GO = GN * GM;
                            let GP = N * ((EP - F) * GO);
                            let GR = GQ * (GP * GI);
                            GA = GO;
                            GB = FX;
                            GC = GI;
                            GD = GP;
                            GE = GR;
                        }
                        let HC;
                        if GF != 0.0 {
                            HC = BH;
                        } else {
                            let GT = BA * ((GA * GS) / GB);
                            let GV = (GU * AX) / GT;
                            let GW = GV * GV;
                            let GX = GW * GW;
                            let GY = (GX / (GX + F)).sqrt();
                            let GZ = GY.sqrt();
                            let HA = GY * GZ;
                            let HG = if HB != 0.0 {
                                let HE = F / (F + (GT * HA));
                                HE
                            } else {
                                let HF = (F + (GT * HA)).powf(staged[27]);
                                HF
                            };
                            let HH = (GC * HG) / (GC + HG);
                            let HJ = (HI * (GT / GZ)).sqrt();
                            let HK = (((AX * GV) * GZ) - (AX * GY)) + (M * (GT * HA));
                            let HL = (((U * (GV * GZ)) - GY) - F) * HJ;
                            let HM = HL * HL;
                            let HN = if HL > BH { 1.0 } else { 0.0 };
                            oHN = HN;
                            let HR = if HN != 0.0 {
                                let HP = F / (F + (HO * HL));
                                HP
                            } else {
                                let HQ = F / (F - (HO * HL));
                                HQ
                            };
                            let HS = (-HM) + HK;
                            let HT = if HS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oHT = HT;
                            let HW = if HT != 0.0 {
                                let HU = HS.exp();
                                HU
                            } else {
                                let HV = DB / (F + ((-2.3025850929940458e2f64 - HS) * (F + (M * ((-2.3025850929940458e2f64 - HS) * (F + ((-2.3025850929940458e2f64 - HS) * DA)))))));
                                HV
                            };
                            let HY = HR * HR;
                            let IB = (((HX * HR) + (HZ * HY)) + (IA * (HY * HR))) * HW;
                            let ID;
                            if HN != 0.0 {
                                ID = IB;
                            } else {
                                let IC = if HK > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oIC = IC;
                                let II = if IC != 0.0 {
                                    let IG = HK.exp();
                                    IG
                                } else {
                                    let IH = DB / (F + ((-2.3025850929940458e2f64 - HK) * (F + (M * ((-2.3025850929940458e2f64 - HK) * (F + ((-2.3025850929940458e2f64 - HK) * DA)))))));
                                    IH
                                };
                                let IJ = (U * II) - IB;
                                ID = IJ;
                            }
                            let IF = IE * ((GD * (8.86226925452758e-1f64 * ((AX * ID) / HJ))) * HH);
                            HC = IF;
                        }
                        let IK;
                        if HD != 0.0 {
                            IK = BH;
                        } else {
                            let IL = (-BJ) / staged[29];
                            let IM = if (IL.abs()) < CU { 1.0 } else { 0.0 };
                            oIM = IM;
                            let IP;
                            if IM != 0.0 {
                                let IN = IL.exp();
                                IP = IN;
                            } else {
                                let IO = if IL < BH { 1.0 } else { 0.0 };
                                oIO = IO;
                                let IV = if IO != 0.0 {
                                    let IS = DB / (F + ((-2.3025850929940458e2f64 - IL) * (F + (M * ((-2.3025850929940458e2f64 - IL) * (F + ((-2.3025850929940458e2f64 - IL) * DA)))))));
                                    IS
                                } else {
                                    let IT = IL - CU;
                                    let IU = DE * (F + (IT * (F + (M * (IT * (F + (IT * DA)))))));
                                    IU
                                };
                                IP = IV;
                            }
                            let IR = IQ * (staged[30] * IP);
                            IK = IR;
                        }
                        let IX = (IW * (((FQ + GE) + HC) + IK)) * staged[35];
                        FS = GA;
                        FT = GB;
                        FU = GC;
                        FV = GD;
                        FW = IX;
                    }
                    let JA;
                    let JB;
                    let JC;
                    let JD;
                    let JE;
                    if DJ != 0.0 {
                        JA = FS;
                        JB = FT;
                        JC = FU;
                        JD = FV;
                        JE = BH;
                    } else {
                        let IY = R * EM;
                        let JI;
                        let JJ;
                        let JK;
                        let JL;
                        let JM;
                        if IZ != 0.0 {
                            JI = FS;
                            JJ = FT;
                            JK = FU;
                            JL = FV;
                            JM = BH;
                        } else {
                            let JF = AD - EN;
                            let JG = F - ((F - (EO / JF)).sqrt());
                            let JP = if JH != 0.0 {
                                BH
                            } else {
                                let JO = ((((JG * JG) * (JG.ln())) / (F - JG)) + JG) * staged[36];
                                JO
                            };
                            let JQ = JG + JP;
                            let JU = if JH != 0.0 {
                                let JS = (JF * JR).sqrt();
                                JS
                            } else {
                                let JT = (JF * JR).powf(AK);
                                JT
                            };
                            let JW = JV * JU;
                            let JX = O * ((EP - F) * JW);
                            let JZ = JY * (JX * JQ);
                            JI = JW;
                            JJ = JF;
                            JK = JQ;
                            JL = JX;
                            JM = JZ;
                        }
                        let KJ;
                        if JN != 0.0 {
                            KJ = BH;
                        } else {
                            let KB = BB * ((JI * KA) / JJ);
                            let KC = (GU * AY) / KB;
                            let KD = KC * KC;
                            let KE = KD * KD;
                            let KF = (KE / (KE + F)).sqrt();
                            let KG = KF.sqrt();
                            let KH = KF * KG;
                            let KN = if KI != 0.0 {
                                let KL = F / (F + (KB * KH));
                                KL
                            } else {
                                let KM = (F + (KB * KH)).powf(staged[40]);
                                KM
                            };
                            let KO = (JK * KN) / (JK + KN);
                            let KP = (HI * (KB / KG)).sqrt();
                            let KQ = (((AY * KC) * KG) - (AY * KF)) + (M * (KB * KH));
                            let KR = (((U * (KC * KG)) - KF) - F) * KP;
                            let KS = KR * KR;
                            let KT = if KR > BH { 1.0 } else { 0.0 };
                            oKT = KT;
                            let KW = if KT != 0.0 {
                                let KU = F / (F + (HO * KR));
                                KU
                            } else {
                                let KV = F / (F - (HO * KR));
                                KV
                            };
                            let KX = (-KS) + KQ;
                            let KY = if KX > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oKY = KY;
                            let LB = if KY != 0.0 {
                                let KZ = KX.exp();
                                KZ
                            } else {
                                let LA = DB / (F + ((-2.3025850929940458e2f64 - KX) * (F + (M * ((-2.3025850929940458e2f64 - KX) * (F + ((-2.3025850929940458e2f64 - KX) * DA)))))));
                                LA
                            };
                            let LC = KW * KW;
                            let LD = (((HX * KW) + (HZ * LC)) + (IA * (LC * KW))) * LB;
                            let LF;
                            if KT != 0.0 {
                                LF = LD;
                            } else {
                                let LE = if KQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oLE = LE;
                                let LK = if LE != 0.0 {
                                    let LI = KQ.exp();
                                    LI
                                } else {
                                    let LJ = DB / (F + ((-2.3025850929940458e2f64 - KQ) * (F + (M * ((-2.3025850929940458e2f64 - KQ) * (F + ((-2.3025850929940458e2f64 - KQ) * DA)))))));
                                    LJ
                                };
                                let LL = (U * LK) - LD;
                                LF = LL;
                            }
                            let LH = LG * ((JL * (8.86226925452758e-1f64 * ((AY * LF) / KP))) * KO);
                            KJ = LH;
                        }
                        let LM;
                        if KK != 0.0 {
                            LM = BH;
                        } else {
                            let LN = (-BL) / staged[42];
                            let LO = if (LN.abs()) < CU { 1.0 } else { 0.0 };
                            oLO = LO;
                            let LR;
                            if LO != 0.0 {
                                let LP = LN.exp();
                                LR = LP;
                            } else {
                                let LQ = if LN < BH { 1.0 } else { 0.0 };
                                oLQ = LQ;
                                let LX = if LQ != 0.0 {
                                    let LU = DB / (F + ((-2.3025850929940458e2f64 - LN) * (F + (M * ((-2.3025850929940458e2f64 - LN) * (F + ((-2.3025850929940458e2f64 - LN) * DA)))))));
                                    LU
                                } else {
                                    let LV = LN - CU;
                                    let LW = DE * (F + (LV * (F + (M * (LV * (F + (LV * DA)))))));
                                    LW
                                };
                                LR = LX;
                            }
                            let LT = LS * (staged[43] * LR);
                            LM = LT;
                        }
                        let LY = (IW * (((IY + JM) + KJ) + LM)) * staged[47];
                        JA = JI;
                        JB = JJ;
                        JC = JK;
                        JD = JL;
                        JE = LY;
                    }
                    let MB;
                    let MC;
                    let MD;
                    let ME;
                    let MF;
                    if DM != 0.0 {
                        MB = BH;
                        MC = JA;
                        MD = JB;
                        ME = JC;
                        MF = JD;
                    } else {
                        let LZ = S * EM;
                        let MK;
                        let ML;
                        let MM;
                        let MN;
                        let MO;
                        if MA != 0.0 {
                            MK = JA;
                            ML = JB;
                            MM = JC;
                            MN = JD;
                            MO = BH;
                        } else {
                            let MH = AE - EN;
                            let MI = F - ((F - (EO / MH)).sqrt());
                            let MR = if MJ != 0.0 {
                                BH
                            } else {
                                let MQ = ((((MI * MI) * (MI.ln())) / (F - MI)) + MI) * staged[48];
                                MQ
                            };
                            let MS = MI + MR;
                            let MW = if MJ != 0.0 {
                                let MU = (MH * MT).sqrt();
                                MU
                            } else {
                                let MV = (MH * MT).powf(AM);
                                MV
                            };
                            let MY = MX * MW;
                            let MZ = P * ((EP - F) * MY);
                            let NB = NA * (MZ * MS);
                            MK = MY;
                            ML = MH;
                            MM = MS;
                            MN = MZ;
                            MO = NB;
                        }
                        let NL;
                        if MP != 0.0 {
                            NL = BH;
                        } else {
                            let ND = BC * ((MK * NC) / ML);
                            let NE = (GU * AZ) / ND;
                            let NF = NE * NE;
                            let NG = NF * NF;
                            let NH = (NG / (NG + F)).sqrt();
                            let NI = NH.sqrt();
                            let NJ = NH * NI;
                            let NP = if NK != 0.0 {
                                let NN = F / (F + (ND * NJ));
                                NN
                            } else {
                                let NO = (F + (ND * NJ)).powf(staged[52]);
                                NO
                            };
                            let NQ = (MM * NP) / (MM + NP);
                            let NR = (HI * (ND / NI)).sqrt();
                            let NS = (((AZ * NE) * NI) - (AZ * NH)) + (M * (ND * NJ));
                            let NT = (((U * (NE * NI)) - NH) - F) * NR;
                            let NU = NT * NT;
                            let NV = if NT > BH { 1.0 } else { 0.0 };
                            oNV = NV;
                            let NY = if NV != 0.0 {
                                let NW = F / (F + (HO * NT));
                                NW
                            } else {
                                let NX = F / (F - (HO * NT));
                                NX
                            };
                            let NZ = (-NU) + NS;
                            let OA = if NZ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oOA = OA;
                            let OD = if OA != 0.0 {
                                let OB = NZ.exp();
                                OB
                            } else {
                                let OC = DB / (F + ((-2.3025850929940458e2f64 - NZ) * (F + (M * ((-2.3025850929940458e2f64 - NZ) * (F + ((-2.3025850929940458e2f64 - NZ) * DA)))))));
                                OC
                            };
                            let OE = NY * NY;
                            let OF = (((HX * NY) + (HZ * OE)) + (IA * (OE * NY))) * OD;
                            let OH;
                            if NV != 0.0 {
                                OH = OF;
                            } else {
                                let OG = if NS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oOG = OG;
                                let OM = if OG != 0.0 {
                                    let OK = NS.exp();
                                    OK
                                } else {
                                    let OL = DB / (F + ((-2.3025850929940458e2f64 - NS) * (F + (M * ((-2.3025850929940458e2f64 - NS) * (F + ((-2.3025850929940458e2f64 - NS) * DA)))))));
                                    OL
                                };
                                let ON = (U * OM) - OF;
                                OH = ON;
                            }
                            let OJ = OI * ((MN * (8.86226925452758e-1f64 * ((AZ * OH) / NR))) * NQ);
                            NL = OJ;
                        }
                        let OO;
                        if NM != 0.0 {
                            OO = BH;
                        } else {
                            let OP = (-BN) / staged[54];
                            let OQ = if (OP.abs()) < CU { 1.0 } else { 0.0 };
                            oOQ = OQ;
                            let OT;
                            if OQ != 0.0 {
                                let OR = OP.exp();
                                OT = OR;
                            } else {
                                let OS = if OP < BH { 1.0 } else { 0.0 };
                                oOS = OS;
                                let OZ = if OS != 0.0 {
                                    let OW = DB / (F + ((-2.3025850929940458e2f64 - OP) * (F + (M * ((-2.3025850929940458e2f64 - OP) * (F + ((-2.3025850929940458e2f64 - OP) * DA)))))));
                                    OW
                                } else {
                                    let OX = OP - CU;
                                    let OY = DE * (F + (OX * (F + (M * (OX * (F + (OX * DA)))))));
                                    OY
                                };
                                OT = OZ;
                            }
                            let OV = OU * (staged[55] * OT);
                            OO = OV;
                        }
                        let PA = (IW * (((LZ + MO) + NL) + OO)) * staged[59];
                        MB = PA;
                        MC = MK;
                        MD = ML;
                        ME = MM;
                        MF = MN;
                    }
                    let MG = ((CB * FW) + (CI * JE)) + (CN * MB);
                    let PD;
                    let PE;
                    let PF;
                    let PG;
                    if DU != 0.0 {
                        let PC = if PB < CS { 1.0 } else { 0.0 };
                        oPC = PC;
                        let PM;
                        let PN;
                        let PO;
                        if PC != 0.0 {
                            let PH = PB * G;
                            let PI = if ((-5e-1f64 * PH).abs()) < CU { 1.0 } else { 0.0 };
                            oPI = PI;
                            let PT;
                            if PI != 0.0 {
                                let PR = (-5e-1f64 * PH).exp();
                                PT = PR;
                            } else {
                                let PS = if (-5e-1f64 * PH) < BH { 1.0 } else { 0.0 };
                                oPS = PS;
                                let PY = if PS != 0.0 {
                                    let PW = DB / (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * PH)) * (F + (M * ((-2.3025850929940458e2f64 - (-5e-1f64 * PH)) * (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * PH)) * DA)))))));
                                    PW
                                } else {
                                    let PX = DE * (F + (((-5e-1f64 * PH) - CU) * (F + (M * (((-5e-1f64 * PH) - CU) * (F + (((-5e-1f64 * PH) - CU) * DA)))))));
                                    PX
                                };
                                PT = PY;
                            }
                            let PU = F / PT;
                            let PV = PU * PU;
                            PM = PV;
                            PN = PT;
                            PO = PU;
                        } else {
                            let PJ = (F + ((PB - CS) * G)) * CY;
                            let PK = PJ.sqrt();
                            let PL = F / PK;
                            PM = PJ;
                            PN = PL;
                            PO = PK;
                        }
                        let PP = PM - F;
                        let QB = if PQ != 0.0 {
                            let PZ = U * (E * (((U + PN) + (((PN + F) * (PN + FI)).sqrt())).ln()));
                            PZ
                        } else {
                            let QA = staged[61] + (U * (E * ((((U * PO) + F) + (((F + PO) * (F + (FI * PO))).sqrt())).ln())));
                            QA
                        };
                        let QC = DP - QB;
                        let QD = PB - QC;
                        let QE = M * ((PB + QC) - (((QD * QD) + ((FO * E) * E)).sqrt()));
                        PD = PP;
                        PE = QE;
                        PF = QB;
                        PG = PO;
                    } else {
                        PD = EM;
                        PE = EN;
                        PF = BH;
                        PG = EP;
                    }
                    let QH;
                    let QI;
                    let QJ;
                    let QK;
                    let QL;
                    if CZ != 0.0 {
                        QH = MC;
                        QI = MD;
                        QJ = ME;
                        QK = MF;
                        QL = BH;
                    } else {
                        let QF = Q * PD;
                        let QP;
                        let QQ;
                        let QR;
                        let QS;
                        let QT;
                        if QG != 0.0 {
                            QP = MC;
                            QQ = MD;
                            QR = ME;
                            QS = MF;
                            QT = BH;
                        } else {
                            let QM = AC - PE;
                            let QN = F - ((F - (PF / QM)).sqrt());
                            let QW = if QO != 0.0 {
                                BH
                            } else {
                                let QV = ((((QN * QN) * (QN.ln())) / (F - QN)) + QN) * staged[62];
                                QV
                            };
                            let QX = QN + QW;
                            let RA = if QO != 0.0 {
                                let QY = (QM * GJ).sqrt();
                                QY
                            } else {
                                let QZ = (QM * GJ).powf(AI);
                                QZ
                            };
                            let RB = GN * RA;
                            let RC = N * ((PG - F) * RB);
                            let RD = GQ * (RC * QX);
                            QP = RB;
                            QQ = QM;
                            QR = QX;
                            QS = RC;
                            QT = RD;
                        }
                        let RM;
                        if QU != 0.0 {
                            RM = BH;
                        } else {
                            let RE = BA * ((QP * GS) / QQ);
                            let RF = (GU * AX) / RE;
                            let RG = RF * RF;
                            let RH = RG * RG;
                            let RI = (RH / (RH + F)).sqrt();
                            let RJ = RI.sqrt();
                            let RK = RI * RJ;
                            let RQ = if RL != 0.0 {
                                let RO = F / (F + (RE * RK));
                                RO
                            } else {
                                let RP = (F + (RE * RK)).powf(staged[63]);
                                RP
                            };
                            let RR = (QR * RQ) / (QR + RQ);
                            let RS = (HI * (RE / RJ)).sqrt();
                            let RT = (((AX * RF) * RJ) - (AX * RI)) + (M * (RE * RK));
                            let RU = (((U * (RF * RJ)) - RI) - F) * RS;
                            let RV = RU * RU;
                            let RW = if RU > BH { 1.0 } else { 0.0 };
                            oRW = RW;
                            let RZ = if RW != 0.0 {
                                let RX = F / (F + (HO * RU));
                                RX
                            } else {
                                let RY = F / (F - (HO * RU));
                                RY
                            };
                            let SA = (-RV) + RT;
                            let SB = if SA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oSB = SB;
                            let SE = if SB != 0.0 {
                                let SC = SA.exp();
                                SC
                            } else {
                                let SD = DB / (F + ((-2.3025850929940458e2f64 - SA) * (F + (M * ((-2.3025850929940458e2f64 - SA) * (F + ((-2.3025850929940458e2f64 - SA) * DA)))))));
                                SD
                            };
                            let SF = RZ * RZ;
                            let SG = (((HX * RZ) + (HZ * SF)) + (IA * (SF * RZ))) * SE;
                            let SI;
                            if RW != 0.0 {
                                SI = SG;
                            } else {
                                let SH = if RT > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oSH = SH;
                                let SM = if SH != 0.0 {
                                    let SK = RT.exp();
                                    SK
                                } else {
                                    let SL = DB / (F + ((-2.3025850929940458e2f64 - RT) * (F + (M * ((-2.3025850929940458e2f64 - RT) * (F + ((-2.3025850929940458e2f64 - RT) * DA)))))));
                                    SL
                                };
                                let SN = (U * SM) - SG;
                                SI = SN;
                            }
                            let SJ = IE * ((QS * (8.86226925452758e-1f64 * ((AX * SI) / RS))) * RR);
                            RM = SJ;
                        }
                        let SO;
                        if RN != 0.0 {
                            SO = BH;
                        } else {
                            let SP = (-BJ) / staged[64];
                            let SQ = if (SP.abs()) < CU { 1.0 } else { 0.0 };
                            oSQ = SQ;
                            let ST;
                            if SQ != 0.0 {
                                let SR = SP.exp();
                                ST = SR;
                            } else {
                                let SS = if SP < BH { 1.0 } else { 0.0 };
                                oSS = SS;
                                let SY = if SS != 0.0 {
                                    let SV = DB / (F + ((-2.3025850929940458e2f64 - SP) * (F + (M * ((-2.3025850929940458e2f64 - SP) * (F + ((-2.3025850929940458e2f64 - SP) * DA)))))));
                                    SV
                                } else {
                                    let SW = SP - CU;
                                    let SX = DE * (F + (SW * (F + (M * (SW * (F + (SW * DA)))))));
                                    SX
                                };
                                ST = SY;
                            }
                            let SU = IQ * (staged[65] * ST);
                            SO = SU;
                        }
                        let SZ = (IW * (((QF + QT) + RM) + SO)) * staged[66];
                        QH = QP;
                        QI = QQ;
                        QJ = QR;
                        QK = QS;
                        QL = SZ;
                    }
                    let TC;
                    let TD;
                    let TE;
                    let TF;
                    let TG;
                    if DJ != 0.0 {
                        TC = QH;
                        TD = QI;
                        TE = QJ;
                        TF = QK;
                        TG = BH;
                    } else {
                        let TA = R * PD;
                        let TK;
                        let TL;
                        let TM;
                        let TN;
                        let TO;
                        if TB != 0.0 {
                            TK = QH;
                            TL = QI;
                            TM = QJ;
                            TN = QK;
                            TO = BH;
                        } else {
                            let TH = AD - PE;
                            let TI = F - ((F - (PF / TH)).sqrt());
                            let TR = if TJ != 0.0 {
                                BH
                            } else {
                                let TQ = ((((TI * TI) * (TI.ln())) / (F - TI)) + TI) * staged[67];
                                TQ
                            };
                            let TS = TI + TR;
                            let TV = if TJ != 0.0 {
                                let TT = (TH * JR).sqrt();
                                TT
                            } else {
                                let TU = (TH * JR).powf(AK);
                                TU
                            };
                            let TW = JV * TV;
                            let TX = O * ((PG - F) * TW);
                            let TY = JY * (TX * TS);
                            TK = TW;
                            TL = TH;
                            TM = TS;
                            TN = TX;
                            TO = TY;
                        }
                        let UH;
                        if TP != 0.0 {
                            UH = BH;
                        } else {
                            let TZ = BB * ((TK * KA) / TL);
                            let UA = (GU * AY) / TZ;
                            let UB = UA * UA;
                            let UC = UB * UB;
                            let UD = (UC / (UC + F)).sqrt();
                            let UE = UD.sqrt();
                            let UF = UD * UE;
                            let UL = if UG != 0.0 {
                                let UJ = F / (F + (TZ * UF));
                                UJ
                            } else {
                                let UK = (F + (TZ * UF)).powf(staged[68]);
                                UK
                            };
                            let UM = (TM * UL) / (TM + UL);
                            let UN = (HI * (TZ / UE)).sqrt();
                            let UO = (((AY * UA) * UE) - (AY * UD)) + (M * (TZ * UF));
                            let UP = (((U * (UA * UE)) - UD) - F) * UN;
                            let UQ = UP * UP;
                            let UR = if UP > BH { 1.0 } else { 0.0 };
                            oUR = UR;
                            let UU = if UR != 0.0 {
                                let US = F / (F + (HO * UP));
                                US
                            } else {
                                let UT = F / (F - (HO * UP));
                                UT
                            };
                            let UV = (-UQ) + UO;
                            let UW = if UV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oUW = UW;
                            let UZ = if UW != 0.0 {
                                let UX = UV.exp();
                                UX
                            } else {
                                let UY = DB / (F + ((-2.3025850929940458e2f64 - UV) * (F + (M * ((-2.3025850929940458e2f64 - UV) * (F + ((-2.3025850929940458e2f64 - UV) * DA)))))));
                                UY
                            };
                            let VA = UU * UU;
                            let VB = (((HX * UU) + (HZ * VA)) + (IA * (VA * UU))) * UZ;
                            let VD;
                            if UR != 0.0 {
                                VD = VB;
                            } else {
                                let VC = if UO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oVC = VC;
                                let VH = if VC != 0.0 {
                                    let VF = UO.exp();
                                    VF
                                } else {
                                    let VG = DB / (F + ((-2.3025850929940458e2f64 - UO) * (F + (M * ((-2.3025850929940458e2f64 - UO) * (F + ((-2.3025850929940458e2f64 - UO) * DA)))))));
                                    VG
                                };
                                let VI = (U * VH) - VB;
                                VD = VI;
                            }
                            let VE = LG * ((TN * (8.86226925452758e-1f64 * ((AY * VD) / UN))) * UM);
                            UH = VE;
                        }
                        let VJ;
                        if UI != 0.0 {
                            VJ = BH;
                        } else {
                            let VK = (-BL) / staged[69];
                            let VL = if (VK.abs()) < CU { 1.0 } else { 0.0 };
                            oVL = VL;
                            let VO;
                            if VL != 0.0 {
                                let VM = VK.exp();
                                VO = VM;
                            } else {
                                let VN = if VK < BH { 1.0 } else { 0.0 };
                                oVN = VN;
                                let VT = if VN != 0.0 {
                                    let VQ = DB / (F + ((-2.3025850929940458e2f64 - VK) * (F + (M * ((-2.3025850929940458e2f64 - VK) * (F + ((-2.3025850929940458e2f64 - VK) * DA)))))));
                                    VQ
                                } else {
                                    let VR = VK - CU;
                                    let VS = DE * (F + (VR * (F + (M * (VR * (F + (VR * DA)))))));
                                    VS
                                };
                                VO = VT;
                            }
                            let VP = LS * (staged[70] * VO);
                            VJ = VP;
                        }
                        let VU = (IW * (((TA + TO) + UH) + VJ)) * staged[71];
                        TC = TK;
                        TD = TL;
                        TE = TM;
                        TF = TN;
                        TG = VU;
                    }
                    let VX;
                    let VY;
                    let VZ;
                    let WA;
                    let WB;
                    if DM != 0.0 {
                        VX = BH;
                        VY = TC;
                        VZ = TD;
                        WA = TE;
                        WB = TF;
                    } else {
                        let VV = S * PD;
                        let WG;
                        let WH;
                        let WI;
                        let WJ;
                        let WK;
                        if VW != 0.0 {
                            WG = TC;
                            WH = TD;
                            WI = TE;
                            WJ = TF;
                            WK = BH;
                        } else {
                            let WD = AE - PE;
                            let WE = F - ((F - (PF / WD)).sqrt());
                            let WN = if WF != 0.0 {
                                BH
                            } else {
                                let WM = ((((WE * WE) * (WE.ln())) / (F - WE)) + WE) * staged[72];
                                WM
                            };
                            let WO = WE + WN;
                            let WR = if WF != 0.0 {
                                let WP = (WD * MT).sqrt();
                                WP
                            } else {
                                let WQ = (WD * MT).powf(AM);
                                WQ
                            };
                            let WS = MX * WR;
                            let WT = P * ((PG - F) * WS);
                            let WU = NA * (WT * WO);
                            WG = WS;
                            WH = WD;
                            WI = WO;
                            WJ = WT;
                            WK = WU;
                        }
                        let XD;
                        if WL != 0.0 {
                            XD = BH;
                        } else {
                            let WV = BC * ((WG * NC) / WH);
                            let WW = (GU * AZ) / WV;
                            let WX = WW * WW;
                            let WY = WX * WX;
                            let WZ = (WY / (WY + F)).sqrt();
                            let XA = WZ.sqrt();
                            let XB = WZ * XA;
                            let XH = if XC != 0.0 {
                                let XF = F / (F + (WV * XB));
                                XF
                            } else {
                                let XG = (F + (WV * XB)).powf(staged[73]);
                                XG
                            };
                            let XI = (WI * XH) / (WI + XH);
                            let XJ = (HI * (WV / XA)).sqrt();
                            let XK = (((AZ * WW) * XA) - (AZ * WZ)) + (M * (WV * XB));
                            let XL = (((U * (WW * XA)) - WZ) - F) * XJ;
                            let XM = XL * XL;
                            let XN = if XL > BH { 1.0 } else { 0.0 };
                            oXN = XN;
                            let XQ = if XN != 0.0 {
                                let XO = F / (F + (HO * XL));
                                XO
                            } else {
                                let XP = F / (F - (HO * XL));
                                XP
                            };
                            let XR = (-XM) + XK;
                            let XS = if XR > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oXS = XS;
                            let XV = if XS != 0.0 {
                                let XT = XR.exp();
                                XT
                            } else {
                                let XU = DB / (F + ((-2.3025850929940458e2f64 - XR) * (F + (M * ((-2.3025850929940458e2f64 - XR) * (F + ((-2.3025850929940458e2f64 - XR) * DA)))))));
                                XU
                            };
                            let XW = XQ * XQ;
                            let XX = (((HX * XQ) + (HZ * XW)) + (IA * (XW * XQ))) * XV;
                            let XZ;
                            if XN != 0.0 {
                                XZ = XX;
                            } else {
                                let XY = if XK > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oXY = XY;
                                let YD = if XY != 0.0 {
                                    let YB = XK.exp();
                                    YB
                                } else {
                                    let YC = DB / (F + ((-2.3025850929940458e2f64 - XK) * (F + (M * ((-2.3025850929940458e2f64 - XK) * (F + ((-2.3025850929940458e2f64 - XK) * DA)))))));
                                    YC
                                };
                                let YE = (U * YD) - XX;
                                XZ = YE;
                            }
                            let YA = OI * ((WJ * (8.86226925452758e-1f64 * ((AZ * XZ) / XJ))) * XI);
                            XD = YA;
                        }
                        let YF;
                        if XE != 0.0 {
                            YF = BH;
                        } else {
                            let YG = (-BN) / staged[74];
                            let YH = if (YG.abs()) < CU { 1.0 } else { 0.0 };
                            oYH = YH;
                            let YK;
                            if YH != 0.0 {
                                let YI = YG.exp();
                                YK = YI;
                            } else {
                                let YJ = if YG < BH { 1.0 } else { 0.0 };
                                oYJ = YJ;
                                let YP = if YJ != 0.0 {
                                    let YM = DB / (F + ((-2.3025850929940458e2f64 - YG) * (F + (M * ((-2.3025850929940458e2f64 - YG) * (F + ((-2.3025850929940458e2f64 - YG) * DA)))))));
                                    YM
                                } else {
                                    let YN = YG - CU;
                                    let YO = DE * (F + (YN * (F + (M * (YN * (F + (YN * DA)))))));
                                    YO
                                };
                                YK = YP;
                            }
                            let YL = OU * (staged[75] * YK);
                            YF = YL;
                        }
                        let YQ = (IW * (((VV + WK) + XD) + YF)) * staged[76];
                        VX = YQ;
                        VY = WG;
                        VZ = WH;
                        WA = WI;
                        WB = WJ;
                    }
                    let WC = ((CB * QL) + (CI * TG)) + (CN * VX);
                    let YT;
                    let YU;
                    let YV;
                    let YW;
                    if DU != 0.0 {
                        let YS = if YR < CS { 1.0 } else { 0.0 };
                        oYS = YS;
                        let ZC;
                        let ZD;
                        let ZE;
                        if YS != 0.0 {
                            let YX = YR * G;
                            let YY = if ((-5e-1f64 * YX).abs()) < CU { 1.0 } else { 0.0 };
                            oYY = YY;
                            let ZJ;
                            if YY != 0.0 {
                                let ZH = (-5e-1f64 * YX).exp();
                                ZJ = ZH;
                            } else {
                                let ZI = if (-5e-1f64 * YX) < BH { 1.0 } else { 0.0 };
                                oZI = ZI;
                                let ZO = if ZI != 0.0 {
                                    let ZM = DB / (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * YX)) * (F + (M * ((-2.3025850929940458e2f64 - (-5e-1f64 * YX)) * (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * YX)) * DA)))))));
                                    ZM
                                } else {
                                    let ZN = DE * (F + (((-5e-1f64 * YX) - CU) * (F + (M * (((-5e-1f64 * YX) - CU) * (F + (((-5e-1f64 * YX) - CU) * DA)))))));
                                    ZN
                                };
                                ZJ = ZO;
                            }
                            let ZK = F / ZJ;
                            let ZL = ZK * ZK;
                            ZC = ZL;
                            ZD = ZJ;
                            ZE = ZK;
                        } else {
                            let YZ = (F + ((YR - CS) * G)) * CY;
                            let ZA = YZ.sqrt();
                            let ZB = F / ZA;
                            ZC = YZ;
                            ZD = ZB;
                            ZE = ZA;
                        }
                        let ZF = ZC - F;
                        let ZR = if ZG != 0.0 {
                            let ZP = U * (E * (((U + ZD) + (((ZD + F) * (ZD + FI)).sqrt())).ln()));
                            ZP
                        } else {
                            let ZQ = staged[78] + (U * (E * ((((U * ZE) + F) + (((F + ZE) * (F + (FI * ZE))).sqrt())).ln())));
                            ZQ
                        };
                        let ZS = DP - ZR;
                        let ZT = YR - ZS;
                        let ZU = M * ((YR + ZS) - (((ZT * ZT) + ((FO * E) * E)).sqrt()));
                        YT = ZF;
                        YU = ZU;
                        YV = ZR;
                        YW = ZE;
                    } else {
                        YT = PD;
                        YU = PE;
                        YV = BH;
                        YW = PG;
                    }
                    let ZX;
                    let ZY;
                    let ZZ;
                    let AAA;
                    let AAB;
                    if CZ != 0.0 {
                        ZX = VY;
                        ZY = VZ;
                        ZZ = WA;
                        AAA = WB;
                        AAB = BH;
                    } else {
                        let ZV = Q * YT;
                        let AAF;
                        let AAG;
                        let AAH;
                        let AAI;
                        let AAJ;
                        if ZW != 0.0 {
                            AAF = VY;
                            AAG = VZ;
                            AAH = WA;
                            AAI = WB;
                            AAJ = BH;
                        } else {
                            let AAC = AC - YU;
                            let AAD = F - ((F - (YV / AAC)).sqrt());
                            let AAM = if AAE != 0.0 {
                                BH
                            } else {
                                let AAL = ((((AAD * AAD) * (AAD.ln())) / (F - AAD)) + AAD) * staged[79];
                                AAL
                            };
                            let AAN = AAD + AAM;
                            let AAQ = if AAE != 0.0 {
                                let AAO = (AAC * GJ).sqrt();
                                AAO
                            } else {
                                let AAP = (AAC * GJ).powf(AI);
                                AAP
                            };
                            let AAR = GN * AAQ;
                            let AAS = N * ((YW - F) * AAR);
                            let AAT = GQ * (AAS * AAN);
                            AAF = AAR;
                            AAG = AAC;
                            AAH = AAN;
                            AAI = AAS;
                            AAJ = AAT;
                        }
                        let ABC;
                        if AAK != 0.0 {
                            ABC = BH;
                        } else {
                            let AAU = BA * ((AAF * GS) / AAG);
                            let AAV = (GU * AX) / AAU;
                            let AAW = AAV * AAV;
                            let AAX = AAW * AAW;
                            let AAY = (AAX / (AAX + F)).sqrt();
                            let AAZ = AAY.sqrt();
                            let ABA = AAY * AAZ;
                            let ABG = if ABB != 0.0 {
                                let ABE = F / (F + (AAU * ABA));
                                ABE
                            } else {
                                let ABF = (F + (AAU * ABA)).powf(staged[80]);
                                ABF
                            };
                            let ABH = (AAH * ABG) / (AAH + ABG);
                            let ABI = (HI * (AAU / AAZ)).sqrt();
                            let ABJ = (((AX * AAV) * AAZ) - (AX * AAY)) + (M * (AAU * ABA));
                            let ABK = (((U * (AAV * AAZ)) - AAY) - F) * ABI;
                            let ABL = ABK * ABK;
                            let ABM = if ABK > BH { 1.0 } else { 0.0 };
                            oABM = ABM;
                            let ABP = if ABM != 0.0 {
                                let ABN = F / (F + (HO * ABK));
                                ABN
                            } else {
                                let ABO = F / (F - (HO * ABK));
                                ABO
                            };
                            let ABQ = (-ABL) + ABJ;
                            let ABR = if ABQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oABR = ABR;
                            let ABU = if ABR != 0.0 {
                                let ABS = ABQ.exp();
                                ABS
                            } else {
                                let ABT = DB / (F + ((-2.3025850929940458e2f64 - ABQ) * (F + (M * ((-2.3025850929940458e2f64 - ABQ) * (F + ((-2.3025850929940458e2f64 - ABQ) * DA)))))));
                                ABT
                            };
                            let ABV = ABP * ABP;
                            let ABW = (((HX * ABP) + (HZ * ABV)) + (IA * (ABV * ABP))) * ABU;
                            let ABY;
                            if ABM != 0.0 {
                                ABY = ABW;
                            } else {
                                let ABX = if ABJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oABX = ABX;
                                let ACC = if ABX != 0.0 {
                                    let ACA = ABJ.exp();
                                    ACA
                                } else {
                                    let ACB = DB / (F + ((-2.3025850929940458e2f64 - ABJ) * (F + (M * ((-2.3025850929940458e2f64 - ABJ) * (F + ((-2.3025850929940458e2f64 - ABJ) * DA)))))));
                                    ACB
                                };
                                let ACD = (U * ACC) - ABW;
                                ABY = ACD;
                            }
                            let ABZ = IE * ((AAI * (8.86226925452758e-1f64 * ((AX * ABY) / ABI))) * ABH);
                            ABC = ABZ;
                        }
                        let ACE;
                        if ABD != 0.0 {
                            ACE = BH;
                        } else {
                            let ACF = (-BJ) / staged[81];
                            let ACG = if (ACF.abs()) < CU { 1.0 } else { 0.0 };
                            oACG = ACG;
                            let ACJ;
                            if ACG != 0.0 {
                                let ACH = ACF.exp();
                                ACJ = ACH;
                            } else {
                                let ACI = if ACF < BH { 1.0 } else { 0.0 };
                                oACI = ACI;
                                let ACO = if ACI != 0.0 {
                                    let ACL = DB / (F + ((-2.3025850929940458e2f64 - ACF) * (F + (M * ((-2.3025850929940458e2f64 - ACF) * (F + ((-2.3025850929940458e2f64 - ACF) * DA)))))));
                                    ACL
                                } else {
                                    let ACM = ACF - CU;
                                    let ACN = DE * (F + (ACM * (F + (M * (ACM * (F + (ACM * DA)))))));
                                    ACN
                                };
                                ACJ = ACO;
                            }
                            let ACK = IQ * (staged[82] * ACJ);
                            ACE = ACK;
                        }
                        let ACP = (IW * (((ZV + AAJ) + ABC) + ACE)) * staged[83];
                        ZX = AAF;
                        ZY = AAG;
                        ZZ = AAH;
                        AAA = AAI;
                        AAB = ACP;
                    }
                    let ACS;
                    let ACT;
                    let ACU;
                    let ACV;
                    let ACW;
                    if DJ != 0.0 {
                        ACS = ZX;
                        ACT = ZY;
                        ACU = ZZ;
                        ACV = AAA;
                        ACW = BH;
                    } else {
                        let ACQ = R * YT;
                        let ADA;
                        let ADB;
                        let ADC;
                        let ADD;
                        let ADE;
                        if ACR != 0.0 {
                            ADA = ZX;
                            ADB = ZY;
                            ADC = ZZ;
                            ADD = AAA;
                            ADE = BH;
                        } else {
                            let ACX = AD - YU;
                            let ACY = F - ((F - (YV / ACX)).sqrt());
                            let ADH = if ACZ != 0.0 {
                                BH
                            } else {
                                let ADG = ((((ACY * ACY) * (ACY.ln())) / (F - ACY)) + ACY) * staged[84];
                                ADG
                            };
                            let ADI = ACY + ADH;
                            let ADL = if ACZ != 0.0 {
                                let ADJ = (ACX * JR).sqrt();
                                ADJ
                            } else {
                                let ADK = (ACX * JR).powf(AK);
                                ADK
                            };
                            let ADM = JV * ADL;
                            let ADN = O * ((YW - F) * ADM);
                            let ADO = JY * (ADN * ADI);
                            ADA = ADM;
                            ADB = ACX;
                            ADC = ADI;
                            ADD = ADN;
                            ADE = ADO;
                        }
                        let ADX;
                        if ADF != 0.0 {
                            ADX = BH;
                        } else {
                            let ADP = BB * ((ADA * KA) / ADB);
                            let ADQ = (GU * AY) / ADP;
                            let ADR = ADQ * ADQ;
                            let ADS = ADR * ADR;
                            let ADT = (ADS / (ADS + F)).sqrt();
                            let ADU = ADT.sqrt();
                            let ADV = ADT * ADU;
                            let AEB = if ADW != 0.0 {
                                let ADZ = F / (F + (ADP * ADV));
                                ADZ
                            } else {
                                let AEA = (F + (ADP * ADV)).powf(staged[85]);
                                AEA
                            };
                            let AEC = (ADC * AEB) / (ADC + AEB);
                            let AED = (HI * (ADP / ADU)).sqrt();
                            let AEE = (((AY * ADQ) * ADU) - (AY * ADT)) + (M * (ADP * ADV));
                            let AEF = (((U * (ADQ * ADU)) - ADT) - F) * AED;
                            let AEG = AEF * AEF;
                            let AEH = if AEF > BH { 1.0 } else { 0.0 };
                            oAEH = AEH;
                            let AEK = if AEH != 0.0 {
                                let AEI = F / (F + (HO * AEF));
                                AEI
                            } else {
                                let AEJ = F / (F - (HO * AEF));
                                AEJ
                            };
                            let AEL = (-AEG) + AEE;
                            let AEM = if AEL > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAEM = AEM;
                            let AEP = if AEM != 0.0 {
                                let AEN = AEL.exp();
                                AEN
                            } else {
                                let AEO = DB / (F + ((-2.3025850929940458e2f64 - AEL) * (F + (M * ((-2.3025850929940458e2f64 - AEL) * (F + ((-2.3025850929940458e2f64 - AEL) * DA)))))));
                                AEO
                            };
                            let AEQ = AEK * AEK;
                            let AER = (((HX * AEK) + (HZ * AEQ)) + (IA * (AEQ * AEK))) * AEP;
                            let AET;
                            if AEH != 0.0 {
                                AET = AER;
                            } else {
                                let AES = if AEE > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAES = AES;
                                let AEX = if AES != 0.0 {
                                    let AEV = AEE.exp();
                                    AEV
                                } else {
                                    let AEW = DB / (F + ((-2.3025850929940458e2f64 - AEE) * (F + (M * ((-2.3025850929940458e2f64 - AEE) * (F + ((-2.3025850929940458e2f64 - AEE) * DA)))))));
                                    AEW
                                };
                                let AEY = (U * AEX) - AER;
                                AET = AEY;
                            }
                            let AEU = LG * ((ADD * (8.86226925452758e-1f64 * ((AY * AET) / AED))) * AEC);
                            ADX = AEU;
                        }
                        let AEZ;
                        if ADY != 0.0 {
                            AEZ = BH;
                        } else {
                            let AFA = (-BL) / staged[86];
                            let AFB = if (AFA.abs()) < CU { 1.0 } else { 0.0 };
                            oAFB = AFB;
                            let AFE;
                            if AFB != 0.0 {
                                let AFC = AFA.exp();
                                AFE = AFC;
                            } else {
                                let AFD = if AFA < BH { 1.0 } else { 0.0 };
                                oAFD = AFD;
                                let AFJ = if AFD != 0.0 {
                                    let AFG = DB / (F + ((-2.3025850929940458e2f64 - AFA) * (F + (M * ((-2.3025850929940458e2f64 - AFA) * (F + ((-2.3025850929940458e2f64 - AFA) * DA)))))));
                                    AFG
                                } else {
                                    let AFH = AFA - CU;
                                    let AFI = DE * (F + (AFH * (F + (M * (AFH * (F + (AFH * DA)))))));
                                    AFI
                                };
                                AFE = AFJ;
                            }
                            let AFF = LS * (staged[87] * AFE);
                            AEZ = AFF;
                        }
                        let AFK = (IW * (((ACQ + ADE) + ADX) + AEZ)) * staged[88];
                        ACS = ADA;
                        ACT = ADB;
                        ACU = ADC;
                        ACV = ADD;
                        ACW = AFK;
                    }
                    let AFN;
                    let AFO;
                    let AFP;
                    let AFQ;
                    let AFR;
                    if DM != 0.0 {
                        AFN = BH;
                        AFO = ACS;
                        AFP = ACT;
                        AFQ = ACU;
                        AFR = ACV;
                    } else {
                        let AFL = S * YT;
                        let AFW;
                        let AFX;
                        let AFY;
                        let AFZ;
                        let AGA;
                        if AFM != 0.0 {
                            AFW = ACS;
                            AFX = ACT;
                            AFY = ACU;
                            AFZ = ACV;
                            AGA = BH;
                        } else {
                            let AFT = AE - YU;
                            let AFU = F - ((F - (YV / AFT)).sqrt());
                            let AGD = if AFV != 0.0 {
                                BH
                            } else {
                                let AGC = ((((AFU * AFU) * (AFU.ln())) / (F - AFU)) + AFU) * staged[89];
                                AGC
                            };
                            let AGE = AFU + AGD;
                            let AGH = if AFV != 0.0 {
                                let AGF = (AFT * MT).sqrt();
                                AGF
                            } else {
                                let AGG = (AFT * MT).powf(AM);
                                AGG
                            };
                            let AGI = MX * AGH;
                            let AGJ = P * ((YW - F) * AGI);
                            let AGK = NA * (AGJ * AGE);
                            AFW = AGI;
                            AFX = AFT;
                            AFY = AGE;
                            AFZ = AGJ;
                            AGA = AGK;
                        }
                        let AGT;
                        if AGB != 0.0 {
                            AGT = BH;
                        } else {
                            let AGL = BC * ((AFW * NC) / AFX);
                            let AGM = (GU * AZ) / AGL;
                            let AGN = AGM * AGM;
                            let AGO = AGN * AGN;
                            let AGP = (AGO / (AGO + F)).sqrt();
                            let AGQ = AGP.sqrt();
                            let AGR = AGP * AGQ;
                            let AGX = if AGS != 0.0 {
                                let AGV = F / (F + (AGL * AGR));
                                AGV
                            } else {
                                let AGW = (F + (AGL * AGR)).powf(staged[90]);
                                AGW
                            };
                            let AGY = (AFY * AGX) / (AFY + AGX);
                            let AGZ = (HI * (AGL / AGQ)).sqrt();
                            let AHA = (((AZ * AGM) * AGQ) - (AZ * AGP)) + (M * (AGL * AGR));
                            let AHB = (((U * (AGM * AGQ)) - AGP) - F) * AGZ;
                            let AHC = AHB * AHB;
                            let AHD = if AHB > BH { 1.0 } else { 0.0 };
                            oAHD = AHD;
                            let AHG = if AHD != 0.0 {
                                let AHE = F / (F + (HO * AHB));
                                AHE
                            } else {
                                let AHF = F / (F - (HO * AHB));
                                AHF
                            };
                            let AHH = (-AHC) + AHA;
                            let AHI = if AHH > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAHI = AHI;
                            let AHL = if AHI != 0.0 {
                                let AHJ = AHH.exp();
                                AHJ
                            } else {
                                let AHK = DB / (F + ((-2.3025850929940458e2f64 - AHH) * (F + (M * ((-2.3025850929940458e2f64 - AHH) * (F + ((-2.3025850929940458e2f64 - AHH) * DA)))))));
                                AHK
                            };
                            let AHM = AHG * AHG;
                            let AHN = (((HX * AHG) + (HZ * AHM)) + (IA * (AHM * AHG))) * AHL;
                            let AHP;
                            if AHD != 0.0 {
                                AHP = AHN;
                            } else {
                                let AHO = if AHA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAHO = AHO;
                                let AHT = if AHO != 0.0 {
                                    let AHR = AHA.exp();
                                    AHR
                                } else {
                                    let AHS = DB / (F + ((-2.3025850929940458e2f64 - AHA) * (F + (M * ((-2.3025850929940458e2f64 - AHA) * (F + ((-2.3025850929940458e2f64 - AHA) * DA)))))));
                                    AHS
                                };
                                let AHU = (U * AHT) - AHN;
                                AHP = AHU;
                            }
                            let AHQ = OI * ((AFZ * (8.86226925452758e-1f64 * ((AZ * AHP) / AGZ))) * AGY);
                            AGT = AHQ;
                        }
                        let AHV;
                        if AGU != 0.0 {
                            AHV = BH;
                        } else {
                            let AHW = (-BN) / staged[91];
                            let AHX = if (AHW.abs()) < CU { 1.0 } else { 0.0 };
                            oAHX = AHX;
                            let AIA;
                            if AHX != 0.0 {
                                let AHY = AHW.exp();
                                AIA = AHY;
                            } else {
                                let AHZ = if AHW < BH { 1.0 } else { 0.0 };
                                oAHZ = AHZ;
                                let AIF = if AHZ != 0.0 {
                                    let AIC = DB / (F + ((-2.3025850929940458e2f64 - AHW) * (F + (M * ((-2.3025850929940458e2f64 - AHW) * (F + ((-2.3025850929940458e2f64 - AHW) * DA)))))));
                                    AIC
                                } else {
                                    let AID = AHW - CU;
                                    let AIE = DE * (F + (AID * (F + (M * (AID * (F + (AID * DA)))))));
                                    AIE
                                };
                                AIA = AIF;
                            }
                            let AIB = OU * (staged[92] * AIA);
                            AHV = AIB;
                        }
                        let AIG = (IW * (((AFL + AGA) + AGT) + AHV)) * staged[93];
                        AFN = AIG;
                        AFO = AFW;
                        AFP = AFX;
                        AFQ = AFY;
                        AFR = AFZ;
                    }
                    let AFS = ((CB * AAB) + (CI * ACW)) + (CN * AFN);
                    let AII;
                    let AIJ;
                    let AIK;
                    let AIL;
                    if DU != 0.0 {
                        let AIH = if DQ < CS { 1.0 } else { 0.0 };
                        oAIH = AIH;
                        let AIR;
                        let AIS;
                        let AIT;
                        if AIH != 0.0 {
                            let AIM = DQ * G;
                            let AIN = if ((-5e-1f64 * AIM).abs()) < CU { 1.0 } else { 0.0 };
                            oAIN = AIN;
                            let AIY;
                            if AIN != 0.0 {
                                let AIW = (-5e-1f64 * AIM).exp();
                                AIY = AIW;
                            } else {
                                let AIX = if (-5e-1f64 * AIM) < BH { 1.0 } else { 0.0 };
                                oAIX = AIX;
                                let AJD = if AIX != 0.0 {
                                    let AJB = DB / (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * AIM)) * (F + (M * ((-2.3025850929940458e2f64 - (-5e-1f64 * AIM)) * (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * AIM)) * DA)))))));
                                    AJB
                                } else {
                                    let AJC = DE * (F + (((-5e-1f64 * AIM) - CU) * (F + (M * (((-5e-1f64 * AIM) - CU) * (F + (((-5e-1f64 * AIM) - CU) * DA)))))));
                                    AJC
                                };
                                AIY = AJD;
                            }
                            let AIZ = F / AIY;
                            let AJA = AIZ * AIZ;
                            AIR = AJA;
                            AIS = AIY;
                            AIT = AIZ;
                        } else {
                            let AIO = (F + ((DQ - CS) * G)) * CY;
                            let AIP = AIO.sqrt();
                            let AIQ = F / AIP;
                            AIR = AIO;
                            AIS = AIQ;
                            AIT = AIP;
                        }
                        let AIU = AIR - F;
                        let AJG = if AIV != 0.0 {
                            let AJE = U * (E * (((U + AIS) + (((AIS + F) * (AIS + FI)).sqrt())).ln()));
                            AJE
                        } else {
                            let AJF = -1e-1f64 + (U * (E * ((((U * AIT) + F) + (((F + AIT) * (F + (FI * AIT))).sqrt())).ln())));
                            AJF
                        };
                        let AJH = DP - AJG;
                        let AJI = DQ - AJH;
                        let AJJ = M * ((DQ + AJH) - (((AJI * AJI) + ((FO * E) * E)).sqrt()));
                        AII = AIU;
                        AIJ = AJJ;
                        AIK = AJG;
                        AIL = AIT;
                    } else {
                        AII = YT;
                        AIJ = YU;
                        AIK = BH;
                        AIL = YW;
                    }
                    let AJM;
                    let AJN;
                    let AJO;
                    let AJP;
                    let AJQ;
                    if CZ != 0.0 {
                        AJM = AFO;
                        AJN = AFP;
                        AJO = AFQ;
                        AJP = AFR;
                        AJQ = BH;
                    } else {
                        let AJK = Q * AII;
                        let AJU;
                        let AJV;
                        let AJW;
                        let AJX;
                        let AJY;
                        if AJL != 0.0 {
                            AJU = AFO;
                            AJV = AFP;
                            AJW = AFQ;
                            AJX = AFR;
                            AJY = BH;
                        } else {
                            let AJR = AC - AIJ;
                            let AJS = F - ((F - (AIK / AJR)).sqrt());
                            let AKB = if AJT != 0.0 {
                                BH
                            } else {
                                let AKA = ((((AJS * AJS) * (AJS.ln())) / (F - AJS)) + AJS) * staged[94];
                                AKA
                            };
                            let AKC = AJS + AKB;
                            let AKF = if AJT != 0.0 {
                                let AKD = (AJR * GJ).sqrt();
                                AKD
                            } else {
                                let AKE = (AJR * GJ).powf(AI);
                                AKE
                            };
                            let AKG = GN * AKF;
                            let AKH = N * ((AIL - F) * AKG);
                            let AKI = GQ * (AKH * AKC);
                            AJU = AKG;
                            AJV = AJR;
                            AJW = AKC;
                            AJX = AKH;
                            AJY = AKI;
                        }
                        let AKR;
                        if AJZ != 0.0 {
                            AKR = BH;
                        } else {
                            let AKJ = BA * ((AJU * GS) / AJV);
                            let AKK = (GU * AX) / AKJ;
                            let AKL = AKK * AKK;
                            let AKM = AKL * AKL;
                            let AKN = (AKM / (AKM + F)).sqrt();
                            let AKO = AKN.sqrt();
                            let AKP = AKN * AKO;
                            let AKV = if AKQ != 0.0 {
                                let AKT = F / (F + (AKJ * AKP));
                                AKT
                            } else {
                                let AKU = (F + (AKJ * AKP)).powf(staged[95]);
                                AKU
                            };
                            let AKW = (AJW * AKV) / (AJW + AKV);
                            let AKX = (HI * (AKJ / AKO)).sqrt();
                            let AKY = (((AX * AKK) * AKO) - (AX * AKN)) + (M * (AKJ * AKP));
                            let AKZ = (((U * (AKK * AKO)) - AKN) - F) * AKX;
                            let ALA = AKZ * AKZ;
                            let ALB = if AKZ > BH { 1.0 } else { 0.0 };
                            oALB = ALB;
                            let ALE = if ALB != 0.0 {
                                let ALC = F / (F + (HO * AKZ));
                                ALC
                            } else {
                                let ALD = F / (F - (HO * AKZ));
                                ALD
                            };
                            let ALF = (-ALA) + AKY;
                            let ALG = if ALF > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oALG = ALG;
                            let ALJ = if ALG != 0.0 {
                                let ALH = ALF.exp();
                                ALH
                            } else {
                                let ALI = DB / (F + ((-2.3025850929940458e2f64 - ALF) * (F + (M * ((-2.3025850929940458e2f64 - ALF) * (F + ((-2.3025850929940458e2f64 - ALF) * DA)))))));
                                ALI
                            };
                            let ALK = ALE * ALE;
                            let ALL = (((HX * ALE) + (HZ * ALK)) + (IA * (ALK * ALE))) * ALJ;
                            let ALN;
                            if ALB != 0.0 {
                                ALN = ALL;
                            } else {
                                let ALM = if AKY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oALM = ALM;
                                let ALR = if ALM != 0.0 {
                                    let ALP = AKY.exp();
                                    ALP
                                } else {
                                    let ALQ = DB / (F + ((-2.3025850929940458e2f64 - AKY) * (F + (M * ((-2.3025850929940458e2f64 - AKY) * (F + ((-2.3025850929940458e2f64 - AKY) * DA)))))));
                                    ALQ
                                };
                                let ALS = (U * ALR) - ALL;
                                ALN = ALS;
                            }
                            let ALO = IE * ((AJX * (8.86226925452758e-1f64 * ((AX * ALN) / AKX))) * AKW);
                            AKR = ALO;
                        }
                        let ALT;
                        if AKS != 0.0 {
                            ALT = BH;
                        } else {
                            let ALU = (-BJ) / staged[96];
                            let ALV = if (ALU.abs()) < CU { 1.0 } else { 0.0 };
                            oALV = ALV;
                            let ALY;
                            if ALV != 0.0 {
                                let ALW = ALU.exp();
                                ALY = ALW;
                            } else {
                                let ALX = if ALU < BH { 1.0 } else { 0.0 };
                                oALX = ALX;
                                let AMD = if ALX != 0.0 {
                                    let AMA = DB / (F + ((-2.3025850929940458e2f64 - ALU) * (F + (M * ((-2.3025850929940458e2f64 - ALU) * (F + ((-2.3025850929940458e2f64 - ALU) * DA)))))));
                                    AMA
                                } else {
                                    let AMB = ALU - CU;
                                    let AMC = DE * (F + (AMB * (F + (M * (AMB * (F + (AMB * DA)))))));
                                    AMC
                                };
                                ALY = AMD;
                            }
                            let ALZ = IQ * (staged[97] * ALY);
                            ALT = ALZ;
                        }
                        let AME = (IW * (((AJK + AJY) + AKR) + ALT)) * staged[98];
                        AJM = AJU;
                        AJN = AJV;
                        AJO = AJW;
                        AJP = AJX;
                        AJQ = AME;
                    }
                    let AMH;
                    let AMI;
                    let AMJ;
                    let AMK;
                    let AML;
                    if DJ != 0.0 {
                        AMH = AJM;
                        AMI = AJN;
                        AMJ = AJO;
                        AMK = AJP;
                        AML = BH;
                    } else {
                        let AMF = R * AII;
                        let AMP;
                        let AMQ;
                        let AMR;
                        let AMS;
                        let AMT;
                        if AMG != 0.0 {
                            AMP = AJM;
                            AMQ = AJN;
                            AMR = AJO;
                            AMS = AJP;
                            AMT = BH;
                        } else {
                            let AMM = AD - AIJ;
                            let AMN = F - ((F - (AIK / AMM)).sqrt());
                            let AMW = if AMO != 0.0 {
                                BH
                            } else {
                                let AMV = ((((AMN * AMN) * (AMN.ln())) / (F - AMN)) + AMN) * staged[99];
                                AMV
                            };
                            let AMX = AMN + AMW;
                            let ANA = if AMO != 0.0 {
                                let AMY = (AMM * JR).sqrt();
                                AMY
                            } else {
                                let AMZ = (AMM * JR).powf(AK);
                                AMZ
                            };
                            let ANB = JV * ANA;
                            let ANC = O * ((AIL - F) * ANB);
                            let AND = JY * (ANC * AMX);
                            AMP = ANB;
                            AMQ = AMM;
                            AMR = AMX;
                            AMS = ANC;
                            AMT = AND;
                        }
                        let ANM;
                        if AMU != 0.0 {
                            ANM = BH;
                        } else {
                            let ANE = BB * ((AMP * KA) / AMQ);
                            let ANF = (GU * AY) / ANE;
                            let ANG = ANF * ANF;
                            let ANH = ANG * ANG;
                            let ANI = (ANH / (ANH + F)).sqrt();
                            let ANJ = ANI.sqrt();
                            let ANK = ANI * ANJ;
                            let ANQ = if ANL != 0.0 {
                                let ANO = F / (F + (ANE * ANK));
                                ANO
                            } else {
                                let ANP = (F + (ANE * ANK)).powf(staged[100]);
                                ANP
                            };
                            let ANR = (AMR * ANQ) / (AMR + ANQ);
                            let ANS = (HI * (ANE / ANJ)).sqrt();
                            let ANT = (((AY * ANF) * ANJ) - (AY * ANI)) + (M * (ANE * ANK));
                            let ANU = (((U * (ANF * ANJ)) - ANI) - F) * ANS;
                            let ANV = ANU * ANU;
                            let ANW = if ANU > BH { 1.0 } else { 0.0 };
                            oANW = ANW;
                            let ANZ = if ANW != 0.0 {
                                let ANX = F / (F + (HO * ANU));
                                ANX
                            } else {
                                let ANY = F / (F - (HO * ANU));
                                ANY
                            };
                            let AOA = (-ANV) + ANT;
                            let AOB = if AOA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAOB = AOB;
                            let AOE = if AOB != 0.0 {
                                let AOC = AOA.exp();
                                AOC
                            } else {
                                let AOD = DB / (F + ((-2.3025850929940458e2f64 - AOA) * (F + (M * ((-2.3025850929940458e2f64 - AOA) * (F + ((-2.3025850929940458e2f64 - AOA) * DA)))))));
                                AOD
                            };
                            let AOF = ANZ * ANZ;
                            let AOG = (((HX * ANZ) + (HZ * AOF)) + (IA * (AOF * ANZ))) * AOE;
                            let AOI;
                            if ANW != 0.0 {
                                AOI = AOG;
                            } else {
                                let AOH = if ANT > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAOH = AOH;
                                let AOM = if AOH != 0.0 {
                                    let AOK = ANT.exp();
                                    AOK
                                } else {
                                    let AOL = DB / (F + ((-2.3025850929940458e2f64 - ANT) * (F + (M * ((-2.3025850929940458e2f64 - ANT) * (F + ((-2.3025850929940458e2f64 - ANT) * DA)))))));
                                    AOL
                                };
                                let AON = (U * AOM) - AOG;
                                AOI = AON;
                            }
                            let AOJ = LG * ((AMS * (8.86226925452758e-1f64 * ((AY * AOI) / ANS))) * ANR);
                            ANM = AOJ;
                        }
                        let AOO;
                        if ANN != 0.0 {
                            AOO = BH;
                        } else {
                            let AOP = (-BL) / staged[101];
                            let AOQ = if (AOP.abs()) < CU { 1.0 } else { 0.0 };
                            oAOQ = AOQ;
                            let AOT;
                            if AOQ != 0.0 {
                                let AOR = AOP.exp();
                                AOT = AOR;
                            } else {
                                let AOS = if AOP < BH { 1.0 } else { 0.0 };
                                oAOS = AOS;
                                let AOY = if AOS != 0.0 {
                                    let AOV = DB / (F + ((-2.3025850929940458e2f64 - AOP) * (F + (M * ((-2.3025850929940458e2f64 - AOP) * (F + ((-2.3025850929940458e2f64 - AOP) * DA)))))));
                                    AOV
                                } else {
                                    let AOW = AOP - CU;
                                    let AOX = DE * (F + (AOW * (F + (M * (AOW * (F + (AOW * DA)))))));
                                    AOX
                                };
                                AOT = AOY;
                            }
                            let AOU = LS * (staged[102] * AOT);
                            AOO = AOU;
                        }
                        let AOZ = (IW * (((AMF + AMT) + ANM) + AOO)) * staged[103];
                        AMH = AMP;
                        AMI = AMQ;
                        AMJ = AMR;
                        AMK = AMS;
                        AML = AOZ;
                    }
                    let APC;
                    let APD;
                    let APE;
                    let APF;
                    let APG;
                    if DM != 0.0 {
                        APC = BH;
                        APD = AMH;
                        APE = AMI;
                        APF = AMJ;
                        APG = AMK;
                    } else {
                        let APA = S * AII;
                        let APL;
                        let APM;
                        let APN;
                        let APO;
                        let APP;
                        if APB != 0.0 {
                            APL = AMH;
                            APM = AMI;
                            APN = AMJ;
                            APO = AMK;
                            APP = BH;
                        } else {
                            let API = AE - AIJ;
                            let APJ = F - ((F - (AIK / API)).sqrt());
                            let APS = if APK != 0.0 {
                                BH
                            } else {
                                let APR = ((((APJ * APJ) * (APJ.ln())) / (F - APJ)) + APJ) * staged[104];
                                APR
                            };
                            let APT = APJ + APS;
                            let APW = if APK != 0.0 {
                                let APU = (API * MT).sqrt();
                                APU
                            } else {
                                let APV = (API * MT).powf(AM);
                                APV
                            };
                            let APX = MX * APW;
                            let APY = P * ((AIL - F) * APX);
                            let APZ = NA * (APY * APT);
                            APL = APX;
                            APM = API;
                            APN = APT;
                            APO = APY;
                            APP = APZ;
                        }
                        let AQI;
                        if APQ != 0.0 {
                            AQI = BH;
                        } else {
                            let AQA = BC * ((APL * NC) / APM);
                            let AQB = (GU * AZ) / AQA;
                            let AQC = AQB * AQB;
                            let AQD = AQC * AQC;
                            let AQE = (AQD / (AQD + F)).sqrt();
                            let AQF = AQE.sqrt();
                            let AQG = AQE * AQF;
                            let AQM = if AQH != 0.0 {
                                let AQK = F / (F + (AQA * AQG));
                                AQK
                            } else {
                                let AQL = (F + (AQA * AQG)).powf(staged[105]);
                                AQL
                            };
                            let AQN = (APN * AQM) / (APN + AQM);
                            let AQO = (HI * (AQA / AQF)).sqrt();
                            let AQP = (((AZ * AQB) * AQF) - (AZ * AQE)) + (M * (AQA * AQG));
                            let AQQ = (((U * (AQB * AQF)) - AQE) - F) * AQO;
                            let AQR = AQQ * AQQ;
                            let AQS = if AQQ > BH { 1.0 } else { 0.0 };
                            oAQS = AQS;
                            let AQV = if AQS != 0.0 {
                                let AQT = F / (F + (HO * AQQ));
                                AQT
                            } else {
                                let AQU = F / (F - (HO * AQQ));
                                AQU
                            };
                            let AQW = (-AQR) + AQP;
                            let AQX = if AQW > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAQX = AQX;
                            let ARA = if AQX != 0.0 {
                                let AQY = AQW.exp();
                                AQY
                            } else {
                                let AQZ = DB / (F + ((-2.3025850929940458e2f64 - AQW) * (F + (M * ((-2.3025850929940458e2f64 - AQW) * (F + ((-2.3025850929940458e2f64 - AQW) * DA)))))));
                                AQZ
                            };
                            let ARB = AQV * AQV;
                            let ARC = (((HX * AQV) + (HZ * ARB)) + (IA * (ARB * AQV))) * ARA;
                            let ARE;
                            if AQS != 0.0 {
                                ARE = ARC;
                            } else {
                                let ARD = if AQP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oARD = ARD;
                                let ARI = if ARD != 0.0 {
                                    let ARG = AQP.exp();
                                    ARG
                                } else {
                                    let ARH = DB / (F + ((-2.3025850929940458e2f64 - AQP) * (F + (M * ((-2.3025850929940458e2f64 - AQP) * (F + ((-2.3025850929940458e2f64 - AQP) * DA)))))));
                                    ARH
                                };
                                let ARJ = (U * ARI) - ARC;
                                ARE = ARJ;
                            }
                            let ARF = OI * ((APO * (8.86226925452758e-1f64 * ((AZ * ARE) / AQO))) * AQN);
                            AQI = ARF;
                        }
                        let ARK;
                        if AQJ != 0.0 {
                            ARK = BH;
                        } else {
                            let ARL = (-BN) / staged[106];
                            let ARM = if (ARL.abs()) < CU { 1.0 } else { 0.0 };
                            oARM = ARM;
                            let ARP;
                            if ARM != 0.0 {
                                let ARN = ARL.exp();
                                ARP = ARN;
                            } else {
                                let ARO = if ARL < BH { 1.0 } else { 0.0 };
                                oARO = ARO;
                                let ARU = if ARO != 0.0 {
                                    let ARR = DB / (F + ((-2.3025850929940458e2f64 - ARL) * (F + (M * ((-2.3025850929940458e2f64 - ARL) * (F + ((-2.3025850929940458e2f64 - ARL) * DA)))))));
                                    ARR
                                } else {
                                    let ARS = ARL - CU;
                                    let ART = DE * (F + (ARS * (F + (M * (ARS * (F + (ARS * DA)))))));
                                    ART
                                };
                                ARP = ARU;
                            }
                            let ARQ = OU * (staged[107] * ARP);
                            ARK = ARQ;
                        }
                        let ARV = (IW * (((APA + APP) + AQI) + ARK)) * staged[108];
                        APC = ARV;
                        APD = APL;
                        APE = APM;
                        APF = APN;
                        APG = APO;
                    }
                    let APH = ((CB * AJQ) + (CI * AML)) + (CN * APC);
                    let ARY;
                    let ARZ;
                    let ASA;
                    let ASB;
                    if DU != 0.0 {
                        let ARX = if ARW < CS { 1.0 } else { 0.0 };
                        oARX = ARX;
                        let ASH;
                        let ASI;
                        let ASJ;
                        if ARX != 0.0 {
                            let ASC = ARW * G;
                            let ASD = if ((-5e-1f64 * ASC).abs()) < CU { 1.0 } else { 0.0 };
                            oASD = ASD;
                            let ASO;
                            if ASD != 0.0 {
                                let ASM = (-5e-1f64 * ASC).exp();
                                ASO = ASM;
                            } else {
                                let ASN = if (-5e-1f64 * ASC) < BH { 1.0 } else { 0.0 };
                                oASN = ASN;
                                let AST = if ASN != 0.0 {
                                    let ASR = DB / (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * ASC)) * (F + (M * ((-2.3025850929940458e2f64 - (-5e-1f64 * ASC)) * (F + ((-2.3025850929940458e2f64 - (-5e-1f64 * ASC)) * DA)))))));
                                    ASR
                                } else {
                                    let ASS = DE * (F + (((-5e-1f64 * ASC) - CU) * (F + (M * (((-5e-1f64 * ASC) - CU) * (F + (((-5e-1f64 * ASC) - CU) * DA)))))));
                                    ASS
                                };
                                ASO = AST;
                            }
                            let ASP = F / ASO;
                            let ASQ = ASP * ASP;
                            ASH = ASQ;
                            ASI = ASO;
                            ASJ = ASP;
                        } else {
                            let ASE = (F + ((ARW - CS) * G)) * CY;
                            let ASF = ASE.sqrt();
                            let ASG = F / ASF;
                            ASH = ASE;
                            ASI = ASG;
                            ASJ = ASF;
                        }
                        let ASK = ASH - F;
                        let ASW = if ASL != 0.0 {
                            let ASU = U * (E * (((U + ASI) + (((ASI + F) * (ASI + FI)).sqrt())).ln()));
                            ASU
                        } else {
                            let ASV = -2e-1f64 + (U * (E * ((((U * ASJ) + F) + (((F + ASJ) * (F + (FI * ASJ))).sqrt())).ln())));
                            ASV
                        };
                        let ASX = DP - ASW;
                        let ASY = ARW - ASX;
                        let ASZ = M * ((ARW + ASX) - (((ASY * ASY) + ((FO * E) * E)).sqrt()));
                        ARY = ASK;
                        ARZ = ASZ;
                        ASA = ASW;
                        ASB = ASJ;
                    } else {
                        ARY = AII;
                        ARZ = AIJ;
                        ASA = BH;
                        ASB = AIL;
                    }
                    let ATC;
                    let ATD;
                    let ATE;
                    let ATF;
                    let ATG;
                    if CZ != 0.0 {
                        ATC = APD;
                        ATD = APE;
                        ATE = APF;
                        ATF = APG;
                        ATG = BH;
                    } else {
                        let ATA = Q * ARY;
                        let ATK;
                        let ATL;
                        let ATM;
                        let ATN;
                        let ATO;
                        if ATB != 0.0 {
                            ATK = APD;
                            ATL = APE;
                            ATM = APF;
                            ATN = APG;
                            ATO = BH;
                        } else {
                            let ATH = AC - ARZ;
                            let ATI = F - ((F - (ASA / ATH)).sqrt());
                            let ATR = if ATJ != 0.0 {
                                BH
                            } else {
                                let ATQ = ((((ATI * ATI) * (ATI.ln())) / (F - ATI)) + ATI) * staged[109];
                                ATQ
                            };
                            let ATS = ATI + ATR;
                            let ATV = if ATJ != 0.0 {
                                let ATT = (ATH * GJ).sqrt();
                                ATT
                            } else {
                                let ATU = (ATH * GJ).powf(AI);
                                ATU
                            };
                            let ATW = GN * ATV;
                            let ATX = N * ((ASB - F) * ATW);
                            let ATY = GQ * (ATX * ATS);
                            ATK = ATW;
                            ATL = ATH;
                            ATM = ATS;
                            ATN = ATX;
                            ATO = ATY;
                        }
                        let AUH;
                        if ATP != 0.0 {
                            AUH = BH;
                        } else {
                            let ATZ = BA * ((ATK * GS) / ATL);
                            let AUA = (GU * AX) / ATZ;
                            let AUB = AUA * AUA;
                            let AUC = AUB * AUB;
                            let AUD = (AUC / (AUC + F)).sqrt();
                            let AUE = AUD.sqrt();
                            let AUF = AUD * AUE;
                            let AUL = if AUG != 0.0 {
                                let AUJ = F / (F + (ATZ * AUF));
                                AUJ
                            } else {
                                let AUK = (F + (ATZ * AUF)).powf(staged[110]);
                                AUK
                            };
                            let AUM = (ATM * AUL) / (ATM + AUL);
                            let AUN = (HI * (ATZ / AUE)).sqrt();
                            let AUO = (((AX * AUA) * AUE) - (AX * AUD)) + (M * (ATZ * AUF));
                            let AUP = (((U * (AUA * AUE)) - AUD) - F) * AUN;
                            let AUQ = AUP * AUP;
                            let AUR = if AUP > BH { 1.0 } else { 0.0 };
                            oAUR = AUR;
                            let AUU = if AUR != 0.0 {
                                let AUS = F / (F + (HO * AUP));
                                AUS
                            } else {
                                let AUT = F / (F - (HO * AUP));
                                AUT
                            };
                            let AUV = (-AUQ) + AUO;
                            let AUW = if AUV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAUW = AUW;
                            let AUZ = if AUW != 0.0 {
                                let AUX = AUV.exp();
                                AUX
                            } else {
                                let AUY = DB / (F + ((-2.3025850929940458e2f64 - AUV) * (F + (M * ((-2.3025850929940458e2f64 - AUV) * (F + ((-2.3025850929940458e2f64 - AUV) * DA)))))));
                                AUY
                            };
                            let AVA = AUU * AUU;
                            let AVB = (((HX * AUU) + (HZ * AVA)) + (IA * (AVA * AUU))) * AUZ;
                            let AVD;
                            if AUR != 0.0 {
                                AVD = AVB;
                            } else {
                                let AVC = if AUO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAVC = AVC;
                                let AVH = if AVC != 0.0 {
                                    let AVF = AUO.exp();
                                    AVF
                                } else {
                                    let AVG = DB / (F + ((-2.3025850929940458e2f64 - AUO) * (F + (M * ((-2.3025850929940458e2f64 - AUO) * (F + ((-2.3025850929940458e2f64 - AUO) * DA)))))));
                                    AVG
                                };
                                let AVI = (U * AVH) - AVB;
                                AVD = AVI;
                            }
                            let AVE = IE * ((ATN * (8.86226925452758e-1f64 * ((AX * AVD) / AUN))) * AUM);
                            AUH = AVE;
                        }
                        let AVJ;
                        if AUI != 0.0 {
                            AVJ = BH;
                        } else {
                            let AVK = (-BJ) / staged[111];
                            let AVL = if (AVK.abs()) < CU { 1.0 } else { 0.0 };
                            oAVL = AVL;
                            let AVO;
                            if AVL != 0.0 {
                                let AVM = AVK.exp();
                                AVO = AVM;
                            } else {
                                let AVN = if AVK < BH { 1.0 } else { 0.0 };
                                oAVN = AVN;
                                let AVT = if AVN != 0.0 {
                                    let AVQ = DB / (F + ((-2.3025850929940458e2f64 - AVK) * (F + (M * ((-2.3025850929940458e2f64 - AVK) * (F + ((-2.3025850929940458e2f64 - AVK) * DA)))))));
                                    AVQ
                                } else {
                                    let AVR = AVK - CU;
                                    let AVS = DE * (F + (AVR * (F + (M * (AVR * (F + (AVR * DA)))))));
                                    AVS
                                };
                                AVO = AVT;
                            }
                            let AVP = IQ * (staged[112] * AVO);
                            AVJ = AVP;
                        }
                        let AVU = (IW * (((ATA + ATO) + AUH) + AVJ)) * staged[113];
                        ATC = ATK;
                        ATD = ATL;
                        ATE = ATM;
                        ATF = ATN;
                        ATG = AVU;
                    }
                    let AVX;
                    let AVY;
                    let AVZ;
                    let AWA;
                    let AWB;
                    if DJ != 0.0 {
                        AVX = ATC;
                        AVY = ATD;
                        AVZ = ATE;
                        AWA = ATF;
                        AWB = BH;
                    } else {
                        let AVV = R * ARY;
                        let AWF;
                        let AWG;
                        let AWH;
                        let AWI;
                        let AWJ;
                        if AVW != 0.0 {
                            AWF = ATC;
                            AWG = ATD;
                            AWH = ATE;
                            AWI = ATF;
                            AWJ = BH;
                        } else {
                            let AWC = AD - ARZ;
                            let AWD = F - ((F - (ASA / AWC)).sqrt());
                            let AWM = if AWE != 0.0 {
                                BH
                            } else {
                                let AWL = ((((AWD * AWD) * (AWD.ln())) / (F - AWD)) + AWD) * staged[114];
                                AWL
                            };
                            let AWN = AWD + AWM;
                            let AWQ = if AWE != 0.0 {
                                let AWO = (AWC * JR).sqrt();
                                AWO
                            } else {
                                let AWP = (AWC * JR).powf(AK);
                                AWP
                            };
                            let AWR = JV * AWQ;
                            let AWS = O * ((ASB - F) * AWR);
                            let AWT = JY * (AWS * AWN);
                            AWF = AWR;
                            AWG = AWC;
                            AWH = AWN;
                            AWI = AWS;
                            AWJ = AWT;
                        }
                        let AXC;
                        if AWK != 0.0 {
                            AXC = BH;
                        } else {
                            let AWU = BB * ((AWF * KA) / AWG);
                            let AWV = (GU * AY) / AWU;
                            let AWW = AWV * AWV;
                            let AWX = AWW * AWW;
                            let AWY = (AWX / (AWX + F)).sqrt();
                            let AWZ = AWY.sqrt();
                            let AXA = AWY * AWZ;
                            let AXG = if AXB != 0.0 {
                                let AXE = F / (F + (AWU * AXA));
                                AXE
                            } else {
                                let AXF = (F + (AWU * AXA)).powf(staged[115]);
                                AXF
                            };
                            let AXH = (AWH * AXG) / (AWH + AXG);
                            let AXI = (HI * (AWU / AWZ)).sqrt();
                            let AXJ = (((AY * AWV) * AWZ) - (AY * AWY)) + (M * (AWU * AXA));
                            let AXK = (((U * (AWV * AWZ)) - AWY) - F) * AXI;
                            let AXL = AXK * AXK;
                            let AXM = if AXK > BH { 1.0 } else { 0.0 };
                            oAXM = AXM;
                            let AXP = if AXM != 0.0 {
                                let AXN = F / (F + (HO * AXK));
                                AXN
                            } else {
                                let AXO = F / (F - (HO * AXK));
                                AXO
                            };
                            let AXQ = (-AXL) + AXJ;
                            let AXR = if AXQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oAXR = AXR;
                            let AXU = if AXR != 0.0 {
                                let AXS = AXQ.exp();
                                AXS
                            } else {
                                let AXT = DB / (F + ((-2.3025850929940458e2f64 - AXQ) * (F + (M * ((-2.3025850929940458e2f64 - AXQ) * (F + ((-2.3025850929940458e2f64 - AXQ) * DA)))))));
                                AXT
                            };
                            let AXV = AXP * AXP;
                            let AXW = (((HX * AXP) + (HZ * AXV)) + (IA * (AXV * AXP))) * AXU;
                            let AXY;
                            if AXM != 0.0 {
                                AXY = AXW;
                            } else {
                                let AXX = if AXJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oAXX = AXX;
                                let AYC = if AXX != 0.0 {
                                    let AYA = AXJ.exp();
                                    AYA
                                } else {
                                    let AYB = DB / (F + ((-2.3025850929940458e2f64 - AXJ) * (F + (M * ((-2.3025850929940458e2f64 - AXJ) * (F + ((-2.3025850929940458e2f64 - AXJ) * DA)))))));
                                    AYB
                                };
                                let AYD = (U * AYC) - AXW;
                                AXY = AYD;
                            }
                            let AXZ = LG * ((AWI * (8.86226925452758e-1f64 * ((AY * AXY) / AXI))) * AXH);
                            AXC = AXZ;
                        }
                        let AYE;
                        if AXD != 0.0 {
                            AYE = BH;
                        } else {
                            let AYF = (-BL) / staged[116];
                            let AYG = if (AYF.abs()) < CU { 1.0 } else { 0.0 };
                            oAYG = AYG;
                            let AYJ;
                            if AYG != 0.0 {
                                let AYH = AYF.exp();
                                AYJ = AYH;
                            } else {
                                let AYI = if AYF < BH { 1.0 } else { 0.0 };
                                oAYI = AYI;
                                let AYO = if AYI != 0.0 {
                                    let AYL = DB / (F + ((-2.3025850929940458e2f64 - AYF) * (F + (M * ((-2.3025850929940458e2f64 - AYF) * (F + ((-2.3025850929940458e2f64 - AYF) * DA)))))));
                                    AYL
                                } else {
                                    let AYM = AYF - CU;
                                    let AYN = DE * (F + (AYM * (F + (M * (AYM * (F + (AYM * DA)))))));
                                    AYN
                                };
                                AYJ = AYO;
                            }
                            let AYK = LS * (staged[117] * AYJ);
                            AYE = AYK;
                        }
                        let AYP = (IW * (((AVV + AWJ) + AXC) + AYE)) * staged[118];
                        AVX = AWF;
                        AVY = AWG;
                        AVZ = AWH;
                        AWA = AWI;
                        AWB = AYP;
                    }
                    let AYS;
                    if DM != 0.0 {
                        AYS = BH;
                    } else {
                        let AYQ = S * ARY;
                        let AZB;
                        let AZC;
                        let AZD;
                        let AZE;
                        let AZF;
                        if AYR != 0.0 {
                            AZB = AVX;
                            AZC = AVY;
                            AZD = AVZ;
                            AZE = AWA;
                            AZF = BH;
                        } else {
                            let AYY = AE - ARZ;
                            let AYZ = F - ((F - (ASA / AYY)).sqrt());
                            let AZI = if AZA != 0.0 {
                                BH
                            } else {
                                let AZH = ((((AYZ * AYZ) * (AYZ.ln())) / (F - AYZ)) + AYZ) * staged[119];
                                AZH
                            };
                            let AZJ = AYZ + AZI;
                            let AZM = if AZA != 0.0 {
                                let AZK = (AYY * MT).sqrt();
                                AZK
                            } else {
                                let AZL = (AYY * MT).powf(AM);
                                AZL
                            };
                            let AZN = MX * AZM;
                            let AZO = P * ((ASB - F) * AZN);
                            let AZP = NA * (AZO * AZJ);
                            AZB = AZN;
                            AZC = AYY;
                            AZD = AZJ;
                            AZE = AZO;
                            AZF = AZP;
                        }
                        let AZY;
                        if AZG != 0.0 {
                            AZY = BH;
                        } else {
                            let AZQ = BC * ((AZB * NC) / AZC);
                            let AZR = (GU * AZ) / AZQ;
                            let AZS = AZR * AZR;
                            let AZT = AZS * AZS;
                            let AZU = (AZT / (AZT + F)).sqrt();
                            let AZV = AZU.sqrt();
                            let AZW = AZU * AZV;
                            let BAC = if AZX != 0.0 {
                                let BAA = F / (F + (AZQ * AZW));
                                BAA
                            } else {
                                let BAB = (F + (AZQ * AZW)).powf(staged[120]);
                                BAB
                            };
                            let BAD = (AZD * BAC) / (AZD + BAC);
                            let BAE = (HI * (AZQ / AZV)).sqrt();
                            let BAF = (((AZ * AZR) * AZV) - (AZ * AZU)) + (M * (AZQ * AZW));
                            let BAG = (((U * (AZR * AZV)) - AZU) - F) * BAE;
                            let BAH = BAG * BAG;
                            let BAI = if BAG > BH { 1.0 } else { 0.0 };
                            oBAI = BAI;
                            let BAL = if BAI != 0.0 {
                                let BAJ = F / (F + (HO * BAG));
                                BAJ
                            } else {
                                let BAK = F / (F - (HO * BAG));
                                BAK
                            };
                            let BAM = (-BAH) + BAF;
                            let BAN = if BAM > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            oBAN = BAN;
                            let BAQ = if BAN != 0.0 {
                                let BAO = BAM.exp();
                                BAO
                            } else {
                                let BAP = DB / (F + ((-2.3025850929940458e2f64 - BAM) * (F + (M * ((-2.3025850929940458e2f64 - BAM) * (F + ((-2.3025850929940458e2f64 - BAM) * DA)))))));
                                BAP
                            };
                            let BAR = BAL * BAL;
                            let BAS = (((HX * BAL) + (HZ * BAR)) + (IA * (BAR * BAL))) * BAQ;
                            let BAU;
                            if BAI != 0.0 {
                                BAU = BAS;
                            } else {
                                let BAT = if BAF > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                                oBAT = BAT;
                                let BAY = if BAT != 0.0 {
                                    let BAW = BAF.exp();
                                    BAW
                                } else {
                                    let BAX = DB / (F + ((-2.3025850929940458e2f64 - BAF) * (F + (M * ((-2.3025850929940458e2f64 - BAF) * (F + ((-2.3025850929940458e2f64 - BAF) * DA)))))));
                                    BAX
                                };
                                let BAZ = (U * BAY) - BAS;
                                BAU = BAZ;
                            }
                            let BAV = OI * ((AZE * (8.86226925452758e-1f64 * ((AZ * BAU) / BAE))) * BAD);
                            AZY = BAV;
                        }
                        let BBA;
                        if AZZ != 0.0 {
                            BBA = BH;
                        } else {
                            let BBB = (-BN) / staged[121];
                            let BBC = if (BBB.abs()) < CU { 1.0 } else { 0.0 };
                            oBBC = BBC;
                            let BBF;
                            if BBC != 0.0 {
                                let BBD = BBB.exp();
                                BBF = BBD;
                            } else {
                                let BBE = if BBB < BH { 1.0 } else { 0.0 };
                                oBBE = BBE;
                                let BBK = if BBE != 0.0 {
                                    let BBH = DB / (F + ((-2.3025850929940458e2f64 - BBB) * (F + (M * ((-2.3025850929940458e2f64 - BBB) * (F + ((-2.3025850929940458e2f64 - BBB) * DA)))))));
                                    BBH
                                } else {
                                    let BBI = BBB - CU;
                                    let BBJ = DE * (F + (BBI * (F + (M * (BBI * (F + (BBI * DA)))))));
                                    BBJ
                                };
                                BBF = BBK;
                            }
                            let BBG = OU * (staged[122] * BBF);
                            BBA = BBG;
                        }
                        let BBL = (IW * (((AYQ + AZF) + AZY) + BBA)) * staged[123];
                        AYS = BBL;
                    }
                    let AYT = ((CB * ATG) + (CI * AWB)) + (CN * AYS);
                    let AYU = (CC + CJ) + CO;
                    let AYV = DQ * G;
                    let AYW = APH - (AYU * ((AYV.exp()) - F));
                    let AYX = AYT - (AYU * (((ARW * G).exp()) - F));
                    let BBN;
                    let BBO;
                    let BBP;
                    let BBQ;
                    let BBR;
                    if DU != 0.0 {
                        let BBM = if (if APH > BH { 1.0 } else { 0.0 }) != 0.0 && (if AYT > BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBBM = BBM;
                        let BBZ;
                        let BCA;
                        if BBM != 0.0 {
                            let BBY = if (if (if (if (if (AYW / APH) > BBX { 1.0 } else { 0.0 }) != 0.0 || (if (AYX / AYT) > BBX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AYW > BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AYX > BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AYX > AYW { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            oBBY = BBY;
                            let BCK;
                            let BCL;
                            if BBY != 0.0 {
                                let BCI = (E * ((AYW / AYX).ln())) / -1e-1f64;
                                let BCJ = AYW / (((AYV * BCI).exp()) - F);
                                BCK = BCJ;
                                BCL = BCI;
                            } else {
                                BCK = BH;
                                BCL = F;
                            }
                            BBZ = BCK;
                            BCA = BCL;
                        } else {
                            BBZ = BH;
                            BCA = F;
                        }
                        let BCB = EK * G;
                        let BCC = (MG - (AYU * ((BCB.exp()) - F))) - (BBZ * (((BCB * BCA).exp()) - F));
                        let BCD = PB * G;
                        let BCE = (WC - (AYU * ((BCD.exp()) - F))) - (BBZ * (((BCD * BCA).exp()) - F));
                        let BCF = YR * G;
                        let BCG = (AFS - (AYU * ((BCF.exp()) - F))) - (BBZ * (((BCF * BCA).exp()) - F));
                        let BCH = if (if (if MG < BH { 1.0 } else { 0.0 }) != 0.0 && (if WC < BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AFS < BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBCH = BCH;
                        let BCN;
                        let BCO;
                        let BCP;
                        if BCH != 0.0 {
                            let BCM = if (if (if (if (if (if (BCC / MG) > BBX { 1.0 } else { 0.0 }) != 0.0 || (if (BCE / WC) > BBX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (BCG / AFS) > BBX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BCC < BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BCE < BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BCG < BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            oBCM = BCM;
                            let BCV;
                            let BCW;
                            let BCX;
                            if BCM != 0.0 {
                                let BCQ = BCC / BCE;
                                let BCR = EK - PB;
                                let BCS = PB - EK;
                                let BCT = (((-E) * (BCQ.ln())) / BCR) + (((E * (BCQ - F)) * ((BCQ.powf((PB / BCS))) - F)) / ((((BCQ.powf((EK / BCR))) * BCS) + (BCQ * EK)) - PB));
                                let BCU = if ((BCF * BCT).abs()) < 1e-6f64 { 1.0 } else { 0.0 };
                                oBCU = BCU;
                                let BDB;
                                let BDC;
                                let BDD;
                                if BCU != 0.0 {
                                    let BCY = BCG * ((F / YR) + ((M * G) * BCT));
                                    let BCZ = (((-5e-1f64 * BCG) * BCT) * G) / YR;
                                    BDB = BCY;
                                    BDC = F;
                                    BDD = BCZ;
                                } else {
                                    let BDA = (-BCG) / (((((-YR) * G) * BCT).exp()) - F);
                                    BDB = BDA;
                                    BDC = BH;
                                    BDD = BCT;
                                }
                                BCV = BDB;
                                BCW = BDC;
                                BCX = BDD;
                            } else {
                                BCV = BH;
                                BCW = BH;
                                BCX = F;
                            }
                            BCN = BCV;
                            BCO = BCW;
                            BCP = BCX;
                        } else {
                            BCN = BH;
                            BCO = BH;
                            BCP = F;
                        }
                        BBN = BBZ;
                        BBO = BCN;
                        BBP = BCA;
                        BBQ = BCO;
                        BBR = BCP;
                    } else {
                        BBN = BH;
                        BBO = BH;
                        BBP = F;
                        BBQ = BH;
                        BBR = F;
                    }
                    let BBS = CB * AJ;
                    let BBT = CI * AL;
                    let BBU = CN * AN;
                    let BBV = parameters[64] * ((BBS + BBT) + BBU);
                    let BBW = if BBS <= BBV { 1.0 } else { 0.0 };
                    oBBW = BBW;
                    let BDE = if BBW != 0.0 {
                        BH
                    } else {
                        F
                    };
                    let BDF = if BBT <= BBV { 1.0 } else { 0.0 };
                    oBDF = BDF;
                    let BDG = if BDF != 0.0 {
                        BH
                    } else {
                        F
                    };
                    let BDH = if BBU <= BBV { 1.0 } else { 0.0 };
                    oBDH = BDH;
                    let BDI = if BDH != 0.0 {
                        BH
                    } else {
                        F
                    };
                    let BDO;
                    let BDP;
                    let BDQ;
                    if DU != 0.0 {
                        let BDL = (BDK / (AYU + BDJ)).ln();
                        let BDM = (BDK / (BBN + BDJ)).ln();
                        let BDN = (BDK / ((BBO.abs()) + BDJ)).ln();
                        BDO = BDL;
                        BDP = BDM;
                        BDQ = BDN;
                    } else {
                        BDO = BH;
                        BDP = BH;
                        BDQ = BH;
                    }
                    let BDR = if BDO <= CU { BDO } else { CU };
                    let BDS = BDR.exp();
                    let BDT = if BDP <= CU { BDP } else { CU };
                    let BDU = BDT.exp();
                    let BDV = if BDQ <= CU { BDQ } else { CU };
                    let BDW = BDV.exp();
                    DV = BDR;
                    DW = BDS;
                    DX = AYU;
                    DY = BBP;
                    DZ = BDT;
                    EA = BDU;
                    EB = BBN;
                    EC = BBQ;
                    ED = BBO;
                    EE = BBR;
                    EF = BDV;
                    EG = BDW;
                    EH = BDE;
                    EI = BDG;
                    EJ = BDI;
                } else {
                    DV = BH;
                    DW = BH;
                    DX = BH;
                    DY = F;
                    DZ = BH;
                    EA = BH;
                    EB = BH;
                    EC = BH;
                    ED = BH;
                    EE = F;
                    EF = BH;
                    EG = BH;
                    EH = F;
                    EI = F;
                    EJ = F;
                }
                if DT != 0.0 {
                    let BDY = if EC > BH { 1.0 } else { 0.0 };
                    oBDY = BDY;
                    if BDY != 0.0 {
                    } else {
                        let BEC = -ED;
                        oBEC = BEC;
                    }
                    let BDZ = (FO * DR) * DR;
                    oBDZ = BDZ;
                    let BEA = DR * (DR / DS);
                    oBEA = BEA;
                    let BEB = if EH > M { 1.0 } else { 0.0 };
                    oBEB = BEB;
                    if BEB != 0.0 {
                        let BED = if GS == M { 1.0 } else { 0.0 };
                        oBED = BED;
                        if BED != 0.0 {
                        } else {
                            let BEG = GS - BEF;
                            oBEG = BEG;
                        }
                    } else {
                    }
                    let BEE = if EI > M { 1.0 } else { 0.0 };
                    oBEE = BEE;
                    if BEE != 0.0 {
                        let BEH = if KA == M { 1.0 } else { 0.0 };
                        oBEH = BEH;
                        if BEH != 0.0 {
                        } else {
                            let BEJ = KA - BEF;
                            oBEJ = BEJ;
                        }
                    } else {
                    }
                    let BEI = if EJ > M { 1.0 } else { 0.0 };
                    oBEI = BEI;
                    if BEI != 0.0 {
                        let BEK = if NC == M { 1.0 } else { 0.0 };
                        oBEK = BEK;
                        if BEK != 0.0 {
                        } else {
                            let BEL = NC - BEF;
                            oBEL = BEL;
                        }
                    } else {
                    }
                } else {
                    if BDX != 0.0 {
                        let BEM = (FO * DR) * DR;
                        oBEM = BEM;
                        let BEN = DR * (DR / DS);
                        oBEN = BEN;
                        let BEO = (FO * E) * E;
                        oBEO = BEO;
                    } else {
                    }
                    if CZ != 0.0 {
                    } else {
                        if BEP != 0.0 {
                        } else {
                            let BEQ = GU * AX;
                            oBEQ = BEQ;
                        }
                        if BER != 0.0 {
                        } else {
                            let BES = -BJ;
                            oBES = BES;
                        }
                    }
                    if DJ != 0.0 {
                    } else {
                        if BET != 0.0 {
                        } else {
                            let BEU = GU * AY;
                            oBEU = BEU;
                        }
                        if BEV != 0.0 {
                        } else {
                            let BEW = -BL;
                            oBEW = BEW;
                        }
                    }
                    if DM != 0.0 {
                    } else {
                        if BEX != 0.0 {
                        } else {
                            let BEY = GU * AZ;
                            oBEY = BEY;
                        }
                        if BEZ != 0.0 {
                        } else {
                            let BFA = -BN;
                            oBFA = BFA;
                        }
                    }
                }
            [E, G, N, O, P, Q, R, S, AC, AD, AE, AF, AG, AH, AO, AP, AQ, AR, AS, AT, AX, AY, AZ, BA, BB, BC, BI, BK, BM, CD, CK, CP, CS, CV, oCX, DP, DR, DS, oEL, oER, oFB, CY, oHN, oHT, oIC, oIM, oIO, oKT, oKY, oLE, oLO, oLQ, oNV, oOA, oOG, oOQ, oOS, oPC, oPI, oPS, oRW, oSB, oSH, oSQ, oSS, oUR, oUW, oVC, oVL, oVN, oXN, oXS, oXY, oYH, oYJ, oYS, oYY, oZI, oABM, oABR, oABX, oACG, oACI, oAEH, oAEM, oAES, oAFB, oAFD, oAHD, oAHI, oAHO, oAHX, oAHZ, oAIH, oAIN, oAIX, oALB, oALG, oALM, oALV, oALX, oANW, oAOB, oAOH, oAOQ, oAOS, oAQS, oAQX, oARD, oARM, oARO, oARX, oASD, oASN, oAUR, oAUW, oAVC, oAVL, oAVN, oAXM, oAXR, oAXX, oAYG, oAYI, oBAI, oBAN, oBAT, oBBC, oBBE, oBBM, oBBY, oBCH, oBCM, oBCU, oBBW, oBDF, oBDH, DV, DW, DX, DY, DZ, EA, EB, oBDY, ED, EE, EF, EG, oBEC, oBDZ, oBEA, oBEB, oBED, oBEE, oBEH, oBEI, oBEK, oBEM, oBEN, oBEO, oBEQ, oBES, oBEU, oBEW, oBEY, oBFA, BY, BZ, CA, oBEG, oBEJ, oBEL]
        };
        self.canonical_staged[154] = produced[0];
        self.canonical_staged[125] = produced[1];
        self.canonical_staged[162] = produced[2];
        self.canonical_staged[172] = produced[3];
        self.canonical_staged[182] = produced[4];
        self.canonical_staged[159] = produced[5];
        self.canonical_staged[169] = produced[6];
        self.canonical_staged[179] = produced[7];
        self.canonical_staged[160] = produced[8];
        self.canonical_staged[170] = produced[9];
        self.canonical_staged[180] = produced[10];
        self.canonical_staged[141] = produced[11];
        self.canonical_staged[144] = produced[12];
        self.canonical_staged[147] = produced[13];
        self.canonical_staged[142] = produced[14];
        self.canonical_staged[145] = produced[15];
        self.canonical_staged[148] = produced[16];
        self.canonical_staged[143] = produced[17];
        self.canonical_staged[146] = produced[18];
        self.canonical_staged[149] = produced[19];
        self.canonical_staged[166] = produced[20];
        self.canonical_staged[176] = produced[21];
        self.canonical_staged[186] = produced[22];
        self.canonical_staged[163] = produced[23];
        self.canonical_staged[173] = produced[24];
        self.canonical_staged[183] = produced[25];
        self.canonical_staged[215] = produced[26];
        self.canonical_staged[219] = produced[27];
        self.canonical_staged[220] = produced[28];
        self.canonical_staged[225] = produced[29];
        self.canonical_staged[226] = produced[30];
        self.canonical_staged[227] = produced[31];
        self.canonical_staged[152] = produced[32];
        self.canonical_staged[228] = produced[33];
        self.canonical_staged[229] = produced[34];
        self.canonical_staged[155] = produced[35];
        self.canonical_staged[189] = produced[36];
        self.canonical_staged[139] = produced[37];
        self.canonical_staged[235] = produced[38];
        self.canonical_staged[236] = produced[39];
        self.canonical_staged[238] = produced[40];
        self.canonical_staged[153] = produced[41];
        self.canonical_staged[244] = produced[42];
        self.canonical_staged[245] = produced[43];
        self.canonical_staged[246] = produced[44];
        self.canonical_staged[249] = produced[45];
        self.canonical_staged[250] = produced[46];
        self.canonical_staged[258] = produced[47];
        self.canonical_staged[259] = produced[48];
        self.canonical_staged[260] = produced[49];
        self.canonical_staged[263] = produced[50];
        self.canonical_staged[264] = produced[51];
        self.canonical_staged[272] = produced[52];
        self.canonical_staged[273] = produced[53];
        self.canonical_staged[274] = produced[54];
        self.canonical_staged[277] = produced[55];
        self.canonical_staged[278] = produced[56];
        self.canonical_staged[281] = produced[57];
        self.canonical_staged[282] = produced[58];
        self.canonical_staged[284] = produced[59];
        self.canonical_staged[290] = produced[60];
        self.canonical_staged[291] = produced[61];
        self.canonical_staged[292] = produced[62];
        self.canonical_staged[295] = produced[63];
        self.canonical_staged[296] = produced[64];
        self.canonical_staged[304] = produced[65];
        self.canonical_staged[305] = produced[66];
        self.canonical_staged[306] = produced[67];
        self.canonical_staged[309] = produced[68];
        self.canonical_staged[310] = produced[69];
        self.canonical_staged[318] = produced[70];
        self.canonical_staged[319] = produced[71];
        self.canonical_staged[320] = produced[72];
        self.canonical_staged[323] = produced[73];
        self.canonical_staged[324] = produced[74];
        self.canonical_staged[327] = produced[75];
        self.canonical_staged[328] = produced[76];
        self.canonical_staged[330] = produced[77];
        self.canonical_staged[336] = produced[78];
        self.canonical_staged[337] = produced[79];
        self.canonical_staged[338] = produced[80];
        self.canonical_staged[341] = produced[81];
        self.canonical_staged[342] = produced[82];
        self.canonical_staged[350] = produced[83];
        self.canonical_staged[351] = produced[84];
        self.canonical_staged[352] = produced[85];
        self.canonical_staged[355] = produced[86];
        self.canonical_staged[356] = produced[87];
        self.canonical_staged[364] = produced[88];
        self.canonical_staged[365] = produced[89];
        self.canonical_staged[366] = produced[90];
        self.canonical_staged[369] = produced[91];
        self.canonical_staged[370] = produced[92];
        self.canonical_staged[373] = produced[93];
        self.canonical_staged[374] = produced[94];
        self.canonical_staged[375] = produced[95];
        self.canonical_staged[381] = produced[96];
        self.canonical_staged[382] = produced[97];
        self.canonical_staged[383] = produced[98];
        self.canonical_staged[386] = produced[99];
        self.canonical_staged[387] = produced[100];
        self.canonical_staged[395] = produced[101];
        self.canonical_staged[396] = produced[102];
        self.canonical_staged[397] = produced[103];
        self.canonical_staged[400] = produced[104];
        self.canonical_staged[401] = produced[105];
        self.canonical_staged[409] = produced[106];
        self.canonical_staged[410] = produced[107];
        self.canonical_staged[411] = produced[108];
        self.canonical_staged[414] = produced[109];
        self.canonical_staged[415] = produced[110];
        self.canonical_staged[418] = produced[111];
        self.canonical_staged[419] = produced[112];
        self.canonical_staged[420] = produced[113];
        self.canonical_staged[426] = produced[114];
        self.canonical_staged[427] = produced[115];
        self.canonical_staged[428] = produced[116];
        self.canonical_staged[431] = produced[117];
        self.canonical_staged[432] = produced[118];
        self.canonical_staged[440] = produced[119];
        self.canonical_staged[441] = produced[120];
        self.canonical_staged[442] = produced[121];
        self.canonical_staged[445] = produced[122];
        self.canonical_staged[446] = produced[123];
        self.canonical_staged[454] = produced[124];
        self.canonical_staged[455] = produced[125];
        self.canonical_staged[456] = produced[126];
        self.canonical_staged[459] = produced[127];
        self.canonical_staged[460] = produced[128];
        self.canonical_staged[463] = produced[129];
        self.canonical_staged[465] = produced[130];
        self.canonical_staged[466] = produced[131];
        self.canonical_staged[467] = produced[132];
        self.canonical_staged[468] = produced[133];
        self.canonical_staged[464] = produced[134];
        self.canonical_staged[469] = produced[135];
        self.canonical_staged[470] = produced[136];
        self.canonical_staged[126] = produced[137];
        self.canonical_staged[127] = produced[138];
        self.canonical_staged[128] = produced[139];
        self.canonical_staged[129] = produced[140];
        self.canonical_staged[130] = produced[141];
        self.canonical_staged[131] = produced[142];
        self.canonical_staged[132] = produced[143];
        self.canonical_staged[472] = produced[144];
        self.canonical_staged[134] = produced[145];
        self.canonical_staged[133] = produced[146];
        self.canonical_staged[135] = produced[147];
        self.canonical_staged[136] = produced[148];
        self.canonical_staged[137] = produced[149];
        self.canonical_staged[140] = produced[150];
        self.canonical_staged[138] = produced[151];
        self.canonical_staged[473] = produced[152];
        self.canonical_staged[474] = produced[153];
        self.canonical_staged[475] = produced[154];
        self.canonical_staged[476] = produced[155];
        self.canonical_staged[477] = produced[156];
        self.canonical_staged[478] = produced[157];
        self.canonical_staged[151] = produced[158];
        self.canonical_staged[150] = produced[159];
        self.canonical_staged[156] = produced[160];
        self.canonical_staged[164] = produced[161];
        self.canonical_staged[167] = produced[162];
        self.canonical_staged[174] = produced[163];
        self.canonical_staged[177] = produced[164];
        self.canonical_staged[184] = produced[165];
        self.canonical_staged[187] = produced[166];
        self.canonical_staged[191] = produced[167];
        self.canonical_staged[192] = produced[168];
        self.canonical_staged[193] = produced[169];
        self.canonical_staged[208] = produced[170];
        self.canonical_staged[209] = produced[171];
        self.canonical_staged[210] = produced[172];
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
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
            let A = staged[213];
            let B = staged[230];
            let C = staged[231];
            let D = staged[232];
            let E = staged[233];
            let F = 1e0f64;
            let G = 1e0f64;
            let H = parameters[1];
            let K = staged[125];
            let O = staged[471];
            let X = staged[16];
            let Y = staged[17];
            let Z = staged[18];
            let AA = staged[194];
            let AD = staged[195];
            let AH = ddt_scale();
            let AJ = -1e0f64;
            let AK = 1e0f64;
            let AM = 1e-100f64;
            let AP = staged[126];
            let AT = staged[128];
            let AW = staged[129];
            let BA = staged[127];
            let BK = staged[130];
            let BO = staged[132];
            let BR = staged[472];
            let BS = staged[131];
            let BZ = staged[133];
            let CL = staged[139];
            let CP = 2e0f64;
            let CQ = 1e0f64;
            let CT = 2e0f64;
            let CW = staged[473];
            let DA = staged[135];
            let DE = staged[137];
            let DH = staged[136];
            let DO = staged[474];
            let DP = 0e0f64;
            let DQ = Lanes([0e0f64; 2]);
            let DT = staged[475];
            let DU = staged[141];
            let DY = staged[26];
            let ED = staged[142];
            let EE = staged[143];
            let EH = staged[476];
            let EK = staged[477];
            let EL = staged[144];
            let EP = staged[39];
            let EU = staged[145];
            let EV = staged[146];
            let EY = staged[478];
            let FB = staged[147];
            let FF = staged[51];
            let FK = staged[148];
            let FL = staged[149];
            let FX = staged[152];
            let GP = 2.3025850929940458e2f64;
            let GR = staged[153];
            let HG = -5e-1f64;
            let HR = -5e-1f64;
            let HT = -5e-1f64;
            let HV = -5e-1f64;
            let HW = 3.333333333333333e-1f64;
            let HY = 5e-1f64;
            let ID = -5e-1f64;
            let IF = -5e-1f64;
            let IH = -5e-1f64;
            let IK = 1e100f64;
            let IQ = 3e0f64;
            let IU = staged[154];
            let JM = staged[157];
            let JW = staged[159];
            let JZ = staged[479];
            let KS = staged[480];
            let LD = staged[481];
            let LJ = staged[161];
            let LQ = staged[24];
            let LU = parameters[21];
            let LZ = staged[25];
            let MD = staged[162];
            let MG = parameters[30];
            let MK = staged[163];
            let NC = staged[482];
            let NF = staged[483];
            let NK = staged[165];
            let NT = 3.75e-1f64;
            let NW = staged[166];
            let OH = 5.178164370971076e-1f64;
            let PF = 2.9214664e-1f64;
            let PJ = 2.6992878119627894e-1f64;
            let PK = 4.3792457880372104e-1f64;
            let PS = 8.86226925452758e-1f64;
            let PV = parameters[35];
            let QM = staged[484];
            let QP = staged[485];
            let QQ = parameters[18];
            let QY = staged[28];
            let RA = staged[4];
            let RN = parameters[41];
            let SI = parameters[10];
            let SM = staged[486];
            let SN = parameters[53];
            let SO = 4e0f64;
            let SQ = staged[31];
            let SR = staged[33];
            let SW = staged[32];
            let TG = 0e0f64;
            let TV = parameters[11];
            let TY = staged[169];
            let UB = staged[487];
            let UU = staged[488];
            let VF = staged[489];
            let VL = staged[171];
            let VS = staged[37];
            let VW = parameters[22];
            let WB = staged[38];
            let WF = staged[172];
            let WI = parameters[31];
            let WM = staged[173];
            let XE = staged[490];
            let XH = staged[491];
            let XM = staged[175];
            let XX = staged[176];
            let ZP = 8.86226925452758e-1f64;
            let ZS = parameters[36];
            let AAJ = staged[492];
            let AAM = staged[493];
            let AAN = parameters[19];
            let AAV = staged[41];
            let AAX = staged[5];
            let ABK = parameters[42];
            let ACI = staged[494];
            let ACJ = parameters[54];
            let ACL = staged[45];
            let ACQ = staged[44];
            let ADQ = staged[179];
            let ADT = staged[495];
            let AEG = staged[496];
            let AER = staged[497];
            let AEX = staged[181];
            let AFE = staged[49];
            let AFI = parameters[23];
            let AFN = staged[50];
            let AFR = staged[182];
            let AFU = parameters[32];
            let AFY = staged[183];
            let AGQ = staged[498];
            let AGT = staged[499];
            let AGY = staged[185];
            let AHJ = staged[186];
            let AJB = 8.86226925452758e-1f64;
            let AJE = parameters[37];
            let AJV = staged[500];
            let AJY = staged[501];
            let AJZ = parameters[20];
            let AKH = staged[53];
            let AKJ = staged[6];
            let AKW = parameters[43];
            let ALU = parameters[55];
            let ALW = staged[57];
            let AMB = staged[56];
            let AMS = parameters[60];
            let AMU = staged[502];
            let AMX = parameters[61];
            let ANE = staged[189];
            let AOV = staged[190];
            let AOX = staged[191];
            let APF = staged[192];
            let APG = staged[193];
            let APY = 0e0f64;
            let I = H * (node_potentials[0] - node_potentials[1]);
            let J = (Lanes([F, 0.0]) - Lanes([0.0, G])) * H;
            let P;
            let Q;
            let R;
            let S;
            let T;
            let U;
            let V;
            let W;
            if E != 0.0 {
                let L = I * K;
                let M = J * K;
                let N = if L < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let AR;
                let AS;
                if N != 0.0 {
                    let AL = (-2.3025850929940458e2f64 - L) + AK;
                    let AN = AM / AL;
                    let AO = (((M * AJ) * AN) * AJ) / AL;
                    AR = AN;
                    AS = AO;
                } else {
                    let AQ = if L > AP { 1.0 } else { 0.0 };
                    let BF;
                    let BG;
                    if AQ != 0.0 {
                        let BB = BA * ((L - AP) + AK);
                        let BC = M * BA;
                        BF = BB;
                        BG = BC;
                    } else {
                        let BD = L.exp();
                        let BE = M * BD;
                        BF = BD;
                        BG = BE;
                    }
                    AR = BF;
                    AS = BG;
                }
                let AU = AT * (AR - AK);
                let AV = AS * AT;
                let AX = L * AW;
                let AY = M * AW;
                let AZ = if AX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let BM;
                let BN;
                if AZ != 0.0 {
                    let BH = (-2.3025850929940458e2f64 - AX) + AK;
                    let BI = AM / BH;
                    let BJ = (((AY * AJ) * BI) * AJ) / BH;
                    BM = BI;
                    BN = BJ;
                } else {
                    let BL = if AX > BK { 1.0 } else { 0.0 };
                    let BX;
                    let BY;
                    if BL != 0.0 {
                        let BT = BS * ((AX - BK) + AK);
                        let BU = AY * BS;
                        BX = BT;
                        BY = BU;
                    } else {
                        let BV = AX.exp();
                        let BW = AY * BV;
                        BX = BV;
                        BY = BW;
                    }
                    BM = BX;
                    BN = BY;
                }
                let BP = BO * (BM - AK);
                let BQ = BN * BO;
                let CG;
                let CH;
                if BR != 0.0 {
                    let CA = staged[134] + (I * BZ);
                    let CB = I * CA;
                    let CC = (J * CA) + ((J * BZ) * I);
                    CG = CB;
                    CH = CC;
                } else {
                    let CD = ((-I) * K) * BZ;
                    let CE = ((J * AJ) * K) * BZ;
                    let CF = if CD < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    let DC;
                    let DD;
                    if CF != 0.0 {
                        let CX = (-2.3025850929940458e2f64 - CD) + AK;
                        let CY = AM / CX;
                        let CZ = (((CE * AJ) * CY) * AJ) / CX;
                        DC = CY;
                        DD = CZ;
                    } else {
                        let DB = if CD > DA { 1.0 } else { 0.0 };
                        let DM;
                        let DN;
                        if DB != 0.0 {
                            let DI = DH * ((CD - DA) + AK);
                            let DJ = CE * DH;
                            DM = DI;
                            DN = DJ;
                        } else {
                            let DK = CD.exp();
                            let DL = CE * DK;
                            DM = DK;
                            DN = DL;
                        }
                        DC = DM;
                        DD = DN;
                    }
                    let DF = DE * (DC - AK);
                    let DG = DD * DE;
                    CG = DF;
                    CH = DG;
                }
                let CI = (AU + BP) + CG;
                let CJ = (AV + BQ) + CH;
                let CK = I + staged[138];
                let CM = CL - CK;
                let CN = (J * AJ) * CM;
                let CO = ((CM * CM) + staged[140]).sqrt();
                let CR = (CL + CK) + CO;
                let CS = (I * CL) / CR;
                let CU = CT * CS;
                let CV = (((J * CL) - ((J + ((CN + CN) * (CQ / (CP * CO)))) * CS)) / CR) * CT;
                let DR;
                let DS;
                if CW != 0.0 {
                    let EB;
                    let EC;
                    if DO != 0.0 {
                        let DV = (AK - (CU * DU)).sqrt();
                        let DW = ((CV * DU) * AJ) * (CQ / (CP * DV));
                        EB = DV;
                        EC = DW;
                    } else {
                        let DX = AK - (CU * DU);
                        let DZ = DX.powf(DY);
                        let EA = ((CV * DU) * AJ) * (DY * (DX.powf(staged[208])));
                        EB = DZ;
                        EC = EA;
                    }
                    let EF = (ED * (AK - EB)) + (EE * (I - CU));
                    let EG = ((EC * AJ) * ED) + ((J - CV) * EE);
                    DR = EF;
                    DS = EG;
                } else {
                    DR = DP;
                    DS = DQ;
                }
                let EI;
                let EJ;
                if DT != 0.0 {
                    let ES;
                    let ET;
                    if EH != 0.0 {
                        let EM = (AK - (CU * EL)).sqrt();
                        let EN = ((CV * EL) * AJ) * (CQ / (CP * EM));
                        ES = EM;
                        ET = EN;
                    } else {
                        let EO = AK - (CU * EL);
                        let EQ = EO.powf(EP);
                        let ER = ((CV * EL) * AJ) * (EP * (EO.powf(staged[209])));
                        ES = EQ;
                        ET = ER;
                    }
                    let EW = (EU * (AK - ES)) + (EV * (I - CU));
                    let EX = ((ET * AJ) * EU) + ((J - CV) * EV);
                    EI = EW;
                    EJ = EX;
                } else {
                    EI = DP;
                    EJ = DQ;
                }
                let EZ;
                let FA;
                if EK != 0.0 {
                    let FI;
                    let FJ;
                    if EY != 0.0 {
                        let FC = (AK - (CU * FB)).sqrt();
                        let FD = ((CV * FB) * AJ) * (CQ / (CP * FC));
                        FI = FC;
                        FJ = FD;
                    } else {
                        let FE = AK - (CU * FB);
                        let FG = FE.powf(FF);
                        let FH = ((CV * FB) * AJ) * (FF * (FE.powf(staged[210])));
                        FI = FG;
                        FJ = FH;
                    }
                    let FM = (FK * (AK - FI)) + (FL * (I - CU));
                    let FN = ((FJ * AJ) * FK) + ((J - CV) * FL);
                    EZ = FM;
                    FA = FN;
                } else {
                    EZ = DP;
                    FA = DQ;
                }
                P = DR;
                Q = EI;
                R = EZ;
                S = CI;
                T = DS;
                U = EJ;
                V = FA;
                W = CJ;
            } else {
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
                if O != 0.0 {
                    let FO = I + staged[150];
                    let FP = CL - FO;
                    let FQ = J * AJ;
                    let FR = FQ * FP;
                    let FS = ((FP * FP) + staged[151]).sqrt();
                    let FT = (CL + FO) + FS;
                    let FU = (I * CL) / FT;
                    let FV = CT * FU;
                    let FW = (((J * CL) - ((J + ((FR + FR) * (CQ / (CP * FS)))) * FU)) / FT) * CT;
                    let FY = if I < FX { 1.0 } else { 0.0 };
                    let GY;
                    let GZ;
                    let HA;
                    let HB;
                    let HC;
                    let HD;
                    if FY != 0.0 {
                        let GN = I * K;
                        let GO = J * K;
                        let GQ = if ((-5e-1f64 * GN).abs()) < GP { 1.0 } else { 0.0 };
                        let HK;
                        let HL;
                        if GQ != 0.0 {
                            let HH = (HG * GN).exp();
                            let HI = (GO * HG) * HH;
                            HK = HH;
                            HL = HI;
                        } else {
                            let HJ = if (-5e-1f64 * GN) < DP { 1.0 } else { 0.0 };
                            let IN;
                            let IO;
                            if HJ != 0.0 {
                                let HS = -2.3025850929940458e2f64 - (HR * GN);
                                let HU = -2.3025850929940458e2f64 - (HT * GN);
                                let HX = AK + ((-2.3025850929940458e2f64 - (HV * GN)) * HW);
                                let HZ = AK + (HY * (HU * HX));
                                let IA = AK + (HS * HZ);
                                let IB = AM / IA;
                                let IC = ((((((GO * HR) * AJ) * HZ) + ((((((GO * HT) * AJ) * HX) + ((((GO * HV) * AJ) * HW) * HU)) * HY) * HS)) * IB) * AJ) / IA;
                                IN = IB;
                                IO = IC;
                            } else {
                                let IE = (ID * GN) - GP;
                                let IG = (IF * GN) - GP;
                                let II = AK + (((IH * GN) - GP) * HW);
                                let IJ = AK + (HY * (IG * II));
                                let IL = IK * (AK + (IE * IJ));
                                let IM = (((GO * ID) * IJ) + (((((GO * IF) * II) + (((GO * IH) * HW) * IG)) * HY) * IE)) * IK;
                                IN = IL;
                                IO = IM;
                            }
                            HK = IN;
                            HL = IO;
                        }
                        let HM = AK / HK;
                        let HN = ((HL * HM) * AJ) / HK;
                        let HO = HM * HM;
                        let HP = HN * HM;
                        let HQ = HP + HP;
                        GY = HO;
                        GZ = HK;
                        HA = HM;
                        HB = HQ;
                        HC = HL;
                        HD = HN;
                    } else {
                        let GS = (AK + ((I - FX) * K)) * GR;
                        let GT = (J * K) * GR;
                        let GU = GS.sqrt();
                        let GV = GT * (CQ / (CP * GU));
                        let GW = AK / GU;
                        let GX = ((GV * GW) * AJ) / GU;
                        GY = GS;
                        GZ = GW;
                        HA = GU;
                        HB = GT;
                        HC = GX;
                        HD = GV;
                    }
                    let HE = GY - AK;
                    let HF = if I > DP { 1.0 } else { 0.0 };
                    let JD;
                    let JE;
                    if HF != 0.0 {
                        let IP = GZ + AK;
                        let IR = GZ + IQ;
                        let IS = (IP * IR).sqrt();
                        let IT = (CT + GZ) + IS;
                        let IV = CT * (IU * (IT.ln()));
                        let IW = (((HC + (((HC * IR) + (HC * IP)) * (CQ / (CP * IS)))) * (CQ / IT)) * IU) * CT;
                        JD = IV;
                        JE = IW;
                    } else {
                        let IX = AK + HA;
                        let IY = AK + (IQ * HA);
                        let IZ = (IX * IY).sqrt();
                        let JA = ((CT * HA) + AK) + IZ;
                        let JB = (-I) + (CT * (IU * (JA.ln())));
                        let JC = FQ + (((((HD * CT) + (((HD * IY) + ((HD * IQ) * IX)) * (CQ / (CP * IZ)))) * (CQ / JA)) * IU) * CT);
                        JD = JB;
                        JE = JC;
                    }
                    let JF = staged[155] - JD;
                    let JG = JE * AJ;
                    let JH = I - JF;
                    let JI = (J - JG) * JH;
                    let JJ = ((JH * JH) + staged[156]).sqrt();
                    let JK = HY * ((I + JF) - JJ);
                    let JL = ((J + JG) - ((JI + JI) * (CQ / (CP * JJ)))) * HY;
                    let JN = I - JM;
                    let JO = J * JN;
                    let JP = ((JN * JN) + staged[158]).sqrt();
                    let JQ = HY * ((I + JM) - JP);
                    let JR = (J - ((JO + JO) * (CQ / (CP * JP)))) * HY;
                    let JS = J * I;
                    let JT = ((I * I) + 4e-12f64).sqrt();
                    let JU = HY * (I - JT);
                    let JV = (J - ((JS + JS) * (CQ / (CP * JT)))) * HY;
                    FZ = HE;
                    GA = JK;
                    GB = JD;
                    GC = HA;
                    GD = JQ;
                    GE = JU;
                    GF = FV;
                    GG = HB;
                    GH = JL;
                    GI = JE;
                    GJ = HD;
                    GK = JR;
                    GL = JV;
                    GM = FW;
                } else {
                    FZ = DP;
                    GA = DP;
                    GB = DP;
                    GC = DP;
                    GD = DP;
                    GE = DP;
                    GF = DP;
                    GG = DQ;
                    GH = DQ;
                    GI = DQ;
                    GJ = DQ;
                    GK = DQ;
                    GL = DQ;
                    GM = DQ;
                }
                let KA;
                let KB;
                let KC;
                let KD;
                let KE;
                let KF;
                let KG;
                let KH;
                let KI;
                let KJ;
                let KK;
                let KL;
                if B != 0.0 {
                    KA = DP;
                    KB = DP;
                    KC = DP;
                    KD = DP;
                    KE = DP;
                    KF = DP;
                    KG = DQ;
                    KH = DQ;
                    KI = DQ;
                    KJ = DQ;
                    KK = DQ;
                    KL = DQ;
                } else {
                    let JX = JW * FZ;
                    let JY = GG * JW;
                    let KT;
                    let KU;
                    let KV;
                    let KW;
                    let KX;
                    let KY;
                    let KZ;
                    let LA;
                    let LB;
                    let LC;
                    if JZ != 0.0 {
                        KT = DP;
                        KU = DP;
                        KV = DP;
                        KW = DP;
                        KX = DP;
                        KY = DQ;
                        KZ = DQ;
                        LA = DQ;
                        LB = DQ;
                        LC = DQ;
                    } else {
                        let KM = staged[160] - GA;
                        let KN = GH * AJ;
                        let KO = GB / KM;
                        let KP = (AK - KO).sqrt();
                        let KQ = AK - KP;
                        let KR = ((((GI - (KN * KO)) / KM) * AJ) * (CQ / (CP * KP))) * AJ;
                        let LM;
                        let LN;
                        if KS != 0.0 {
                            LM = DP;
                            LN = DQ;
                        } else {
                            let LE = KQ * KQ;
                            let LF = KR * KQ;
                            let LG = KQ.ln();
                            let LH = AK - KQ;
                            let LI = (LE * LG) / LH;
                            let LK = (LI + KQ) * LJ;
                            let LL = ((((((LF + LF) * LG) + ((KR * (CQ / KQ)) * LE)) - ((KR * AJ) * LI)) / LH) + KR) * LJ;
                            LM = LK;
                            LN = LL;
                        }
                        let LO = KQ + LM;
                        let LP = KR + LN;
                        let LX;
                        let LY;
                        if KS != 0.0 {
                            let LR = (KM * LQ).sqrt();
                            let LS = (KN * LQ) * (CQ / (CP * LR));
                            LX = LR;
                            LY = LS;
                        } else {
                            let LT = KM * LQ;
                            let LV = LT.powf(LU);
                            let LW = (KN * LQ) * (LU * (LT.powf(staged[196])));
                            LX = LV;
                            LY = LW;
                        }
                        let MA = LZ * LX;
                        let MB = LY * LZ;
                        let MC = GC - AK;
                        let ME = MD * (MC * MA);
                        let MF = ((GJ * MA) + (MB * MC)) * MD;
                        let MH = MG * (ME * LO);
                        let MI = ((MF * LO) + (LP * ME)) * MG;
                        KT = MA;
                        KU = KM;
                        KV = LO;
                        KW = ME;
                        KX = MH;
                        KY = MB;
                        KZ = KN;
                        LA = LP;
                        LB = MF;
                        LC = MI;
                    }
                    let ND;
                    let NE;
                    if LD != 0.0 {
                        ND = DP;
                        NE = DQ;
                    } else {
                        let MJ = (KT * DY) / KU;
                        let ML = MK * MJ;
                        let MM = (((KY * DY) - (KZ * MJ)) / KU) * MK;
                        let MN = staged[164] / ML;
                        let MO = ((MM * MN) * AJ) / ML;
                        let MP = MN * MN;
                        let MQ = MO * MN;
                        let MR = MP * MP;
                        let MS = (MQ + MQ) * MP;
                        let MT = MS + MS;
                        let MU = MR + AK;
                        let MV = MR / MU;
                        let MW = MV.sqrt();
                        let MX = ((MT - (MT * MV)) / MU) * (CQ / (CP * MW));
                        let MY = MW.sqrt();
                        let MZ = MX * (CQ / (CP * MY));
                        let NA = MW * MY;
                        let NB = (MX * MY) + (MZ * MW);
                        let NN;
                        let NO;
                        if NC != 0.0 {
                            let NG = AK + (ML * NA);
                            let NH = AK / NG;
                            let NI = ((((MM * NA) + (NB * ML)) * NH) * AJ) / NG;
                            NN = NH;
                            NO = NI;
                        } else {
                            let NJ = AK + (ML * NA);
                            let NL = NJ.powf(NK);
                            let NM = ((MM * NA) + (NB * ML)) * (NK * (NJ.powf(staged[197])));
                            NN = NL;
                            NO = NM;
                        }
                        let NP = KV + NN;
                        let NQ = (KV * NN) / NP;
                        let NR = (((LA * NN) + (NO * KV)) - ((LA + NO) * NQ)) / NP;
                        let NS = ML / MY;
                        let NU = (NT * NS).sqrt();
                        let NV = (((MM - (MZ * NS)) / MY) * NT) * (CQ / (CP * NU));
                        let NX = NW * MN;
                        let NY = ((NX * MY) - (NW * MW)) + (HY * (ML * NA));
                        let NZ = ((((MO * NW) * MY) + (MZ * NX)) - (MX * NW)) + (((MM * NA) + (NB * ML)) * HY);
                        let OA = ((CT * (MN * MY)) - MW) - AK;
                        let OB = OA * NU;
                        let OC = (((((MO * MY) + (MZ * MN)) * CT) - MX) * NU) + (NV * OA);
                        let OD = OB * OB;
                        let OE = OC * OB;
                        let OF = OE + OE;
                        let OG = if OB > DP { 1.0 } else { 0.0 };
                        let OO;
                        let OP;
                        if OG != 0.0 {
                            let OI = AK + (OH * OB);
                            let OJ = AK / OI;
                            let OK = (((OC * OH) * OJ) * AJ) / OI;
                            OO = OJ;
                            OP = OK;
                        } else {
                            let OL = AK - (OH * OB);
                            let OM = AK / OL;
                            let ON = ((((OC * OH) * AJ) * OM) * AJ) / OL;
                            OO = OM;
                            OP = ON;
                        }
                        let OQ = (-OD) + NY;
                        let OR = (OF * AJ) + NZ;
                        let OS = if OQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let PD;
                        let PE;
                        if OS != 0.0 {
                            let OT = OQ.exp();
                            let OU = OR * OT;
                            PD = OT;
                            PE = OU;
                        } else {
                            let OV = -2.3025850929940458e2f64 - OQ;
                            let OW = OR * AJ;
                            let OX = -2.3025850929940458e2f64 - OQ;
                            let OY = AK + ((-2.3025850929940458e2f64 - OQ) * HW);
                            let OZ = AK + (HY * (OX * OY));
                            let PA = AK + (OV * OZ);
                            let PB = AM / PA;
                            let PC = ((((OW * OZ) + ((((OW * OY) + ((OW * HW) * OX)) * HY) * OV)) * PB) * AJ) / PA;
                            PD = PB;
                            PE = PC;
                        }
                        let PG = OO * OO;
                        let PH = OP * OO;
                        let PI = PH + PH;
                        let PL = ((PF * OO) + (PJ * PG)) + (PK * (PG * OO));
                        let PM = PL * PD;
                        let PN = ((((OP * PF) + (PI * PJ)) + (((PI * OO) + (OP * PG)) * PK)) * PD) + (PE * PL);
                        let PP;
                        let PQ;
                        if OG != 0.0 {
                            PP = PM;
                            PQ = PN;
                        } else {
                            let PO = if NY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let QI;
                            let QJ;
                            if PO != 0.0 {
                                let PY = NY.exp();
                                let PZ = NZ * PY;
                                QI = PY;
                                QJ = PZ;
                            } else {
                                let QA = -2.3025850929940458e2f64 - NY;
                                let QB = NZ * AJ;
                                let QC = -2.3025850929940458e2f64 - NY;
                                let QD = AK + ((-2.3025850929940458e2f64 - NY) * HW);
                                let QE = AK + (HY * (QC * QD));
                                let QF = AK + (QA * QE);
                                let QG = AM / QF;
                                let QH = ((((QB * QE) + ((((QB * QD) + ((QB * HW) * QC)) * HY) * QA)) * QG) * AJ) / QF;
                                QI = QG;
                                QJ = QH;
                            }
                            let QK = (CT * QI) - PM;
                            let QL = (QJ * CT) - PN;
                            PP = QK;
                            PQ = QL;
                        }
                        let PR = (NW * PP) / NU;
                        let PT = PS * PR;
                        let PU = KW * PT;
                        let PW = PV * (PU * NQ);
                        let PX = ((((LB * PT) + (((((PQ * NW) - (NV * PR)) / NU) * PS) * KW)) * NQ) + (NR * PU)) * PV;
                        ND = PW;
                        NE = PX;
                    }
                    let QN;
                    let QO;
                    if NF != 0.0 {
                        QN = DP;
                        QO = DQ;
                    } else {
                        let QW;
                        let QX;
                        if QM != 0.0 {
                            let QR = ((QQ - GD) * LQ).sqrt();
                            let QS = ((GK * AJ) * LQ) * (CQ / (CP * QR));
                            QW = QR;
                            QX = QS;
                        } else {
                            let QT = (QQ - GD) * LQ;
                            let QU = QT.powf(LU);
                            let QV = ((GK * AJ) * LQ) * (LU * (QT.powf(staged[198])));
                            QW = QU;
                            QX = QV;
                        }
                        let QZ = ((QQ - GD) * QY) / QW;
                        let RB = RA * QZ;
                        let RC = ((((GK * AJ) * QY) - (QX * QZ)) / QW) * RA;
                        let RD = staged[167] / RB;
                        let RE = ((RC * RD) * AJ) / RB;
                        let RF = if (RD.abs()) < GP { 1.0 } else { 0.0 };
                        let RJ;
                        let RK;
                        if RF != 0.0 {
                            let RG = RD.exp();
                            let RH = RE * RG;
                            RJ = RG;
                            RK = RH;
                        } else {
                            let RI = if RD < DP { 1.0 } else { 0.0 };
                            let SD;
                            let SE;
                            if RI != 0.0 {
                                let RQ = -2.3025850929940458e2f64 - RD;
                                let RR = RE * AJ;
                                let RS = -2.3025850929940458e2f64 - RD;
                                let RT = AK + ((-2.3025850929940458e2f64 - RD) * HW);
                                let RU = AK + (HY * (RS * RT));
                                let RV = AK + (RQ * RU);
                                let RW = AM / RV;
                                let RX = ((((RR * RU) + ((((RR * RT) + ((RR * HW) * RS)) * HY) * RQ)) * RW) * AJ) / RV;
                                SD = RW;
                                SE = RX;
                            } else {
                                let RY = RD - GP;
                                let RZ = AK + (RY * HW);
                                let SA = AK + (HY * (RY * RZ));
                                let SB = IK * (AK + (RY * SA));
                                let SC = ((RE * SA) + ((((RE * RZ) + ((RE * HW) * RY)) * HY) * RY)) * IK;
                                SD = SB;
                                SE = SC;
                            }
                            RJ = SD;
                            RK = SE;
                        }
                        let RL = I * RB;
                        let RM = RL * RB;
                        let RO = RN * (RM * RJ);
                        let RP = ((((((J * RB) + (RC * I)) * RB) + (RC * RL)) * RJ) + (RK * RM)) * RN;
                        QN = RO;
                        QO = RP;
                    }
                    let SG;
                    let SH;
                    if QP != 0.0 {
                        SG = AK;
                        SH = DQ;
                    } else {
                        let SF = if GE > staged[168] { 1.0 } else { 0.0 };
                        let SU;
                        let SV;
                        if SF != 0.0 {
                            let SP = if SN == SO { 1.0 } else { 0.0 };
                            let TJ;
                            let TK;
                            if SP != 0.0 {
                                let SX = GE * SW;
                                let SY = GL * SW;
                                let SZ = SX * SX;
                                let TA = SY * SX;
                                let TB = SZ * SX;
                                let TC = TB * SX;
                                let TD = ((((TA + TA) * SX) + (SY * SZ)) * SX) + (SY * TB);
                                TJ = TC;
                                TK = TD;
                            } else {
                                let TE = GE * SW;
                                let TF = TE.abs();
                                let TH = TF.powf(SN);
                                let TI = ((GL * SW) * ((CP * (if TE >= TG { 1.0 } else { 0.0 })) - CQ)) * (SN * (TF.powf((SN - CQ))));
                                TJ = TH;
                                TK = TI;
                            }
                            let TL = AK - TJ;
                            let TM = AK / TL;
                            let TN = (((TK * AJ) * TM) * AJ) / TL;
                            SU = TM;
                            SV = TN;
                        } else {
                            let SS = GL * SR;
                            let ST = staged[34] + ((GE + (SQ * parameters[50])) * SR);
                            SU = ST;
                            SV = SS;
                        }
                        SG = SU;
                        SH = SV;
                    }
                    let SJ = SI * (((JX + KX) + ND) + QN);
                    let SK = SJ * SG;
                    let SL = (((((JY + LC) + NE) + QO) * SI) * SG) + (SH * SJ);
                    let TT;
                    let TU;
                    if SM != 0.0 {
                        let TO = (AK - (GF * DU)).sqrt();
                        let TP = ((GM * DU) * AJ) * (CQ / (CP * TO));
                        TT = TO;
                        TU = TP;
                    } else {
                        let TQ = AK - (GF * DU);
                        let TR = TQ.powf(DY);
                        let TS = ((GM * DU) * AJ) * (DY * (TQ.powf(staged[199])));
                        TT = TR;
                        TU = TS;
                    }
                    let TW = TV * ((ED * (AK - TT)) + (EE * (I - GF)));
                    let TX = (((TU * AJ) * ED) + ((J - GM) * EE)) * TV;
                    KA = KT;
                    KB = KU;
                    KC = KV;
                    KD = KW;
                    KE = SK;
                    KF = TW;
                    KG = KY;
                    KH = KZ;
                    KI = LA;
                    KJ = LB;
                    KK = SL;
                    KL = TX;
                }
                let UC;
                let UD;
                let UE;
                let UF;
                let UG;
                let UH;
                let UI;
                let UJ;
                let UK;
                let UL;
                let UM;
                let UN;
                if C != 0.0 {
                    UC = KA;
                    UD = KB;
                    UE = KC;
                    UF = KD;
                    UG = DP;
                    UH = DP;
                    UI = KG;
                    UJ = KH;
                    UK = KI;
                    UL = KJ;
                    UM = DQ;
                    UN = DQ;
                } else {
                    let TZ = TY * FZ;
                    let UA = GG * TY;
                    let UV;
                    let UW;
                    let UX;
                    let UY;
                    let UZ;
                    let VA;
                    let VB;
                    let VC;
                    let VD;
                    let VE;
                    if UB != 0.0 {
                        UV = KA;
                        UW = KB;
                        UX = KC;
                        UY = KD;
                        UZ = DP;
                        VA = KG;
                        VB = KH;
                        VC = KI;
                        VD = KJ;
                        VE = DQ;
                    } else {
                        let UO = staged[170] - GA;
                        let UP = GH * AJ;
                        let UQ = GB / UO;
                        let UR = (AK - UQ).sqrt();
                        let US = AK - UR;
                        let UT = ((((GI - (UP * UQ)) / UO) * AJ) * (CQ / (CP * UR))) * AJ;
                        let VO;
                        let VP;
                        if UU != 0.0 {
                            VO = DP;
                            VP = DQ;
                        } else {
                            let VG = US * US;
                            let VH = UT * US;
                            let VI = US.ln();
                            let VJ = AK - US;
                            let VK = (VG * VI) / VJ;
                            let VM = (VK + US) * VL;
                            let VN = ((((((VH + VH) * VI) + ((UT * (CQ / US)) * VG)) - ((UT * AJ) * VK)) / VJ) + UT) * VL;
                            VO = VM;
                            VP = VN;
                        }
                        let VQ = US + VO;
                        let VR = UT + VP;
                        let VZ;
                        let WA;
                        if UU != 0.0 {
                            let VT = (UO * VS).sqrt();
                            let VU = (UP * VS) * (CQ / (CP * VT));
                            VZ = VT;
                            WA = VU;
                        } else {
                            let VV = UO * VS;
                            let VX = VV.powf(VW);
                            let VY = (UP * VS) * (VW * (VV.powf(staged[200])));
                            VZ = VX;
                            WA = VY;
                        }
                        let WC = WB * VZ;
                        let WD = WA * WB;
                        let WE = GC - AK;
                        let WG = WF * (WE * WC);
                        let WH = ((GJ * WC) + (WD * WE)) * WF;
                        let WJ = WI * (WG * VQ);
                        let WK = ((WH * VQ) + (VR * WG)) * WI;
                        UV = WC;
                        UW = UO;
                        UX = VQ;
                        UY = WG;
                        UZ = WJ;
                        VA = WD;
                        VB = UP;
                        VC = VR;
                        VD = WH;
                        VE = WK;
                    }
                    let XF;
                    let XG;
                    if VF != 0.0 {
                        XF = DP;
                        XG = DQ;
                    } else {
                        let WL = (UV * EP) / UW;
                        let WN = WM * WL;
                        let WO = (((VA * EP) - (VB * WL)) / UW) * WM;
                        let WP = staged[174] / WN;
                        let WQ = ((WO * WP) * AJ) / WN;
                        let WR = WP * WP;
                        let WS = WQ * WP;
                        let WT = WR * WR;
                        let WU = (WS + WS) * WR;
                        let WV = WU + WU;
                        let WW = WT + AK;
                        let WX = WT / WW;
                        let WY = WX.sqrt();
                        let WZ = ((WV - (WV * WX)) / WW) * (CQ / (CP * WY));
                        let XA = WY.sqrt();
                        let XB = WZ * (CQ / (CP * XA));
                        let XC = WY * XA;
                        let XD = (WZ * XA) + (XB * WY);
                        let XP;
                        let XQ;
                        if XE != 0.0 {
                            let XI = AK + (WN * XC);
                            let XJ = AK / XI;
                            let XK = ((((WO * XC) + (XD * WN)) * XJ) * AJ) / XI;
                            XP = XJ;
                            XQ = XK;
                        } else {
                            let XL = AK + (WN * XC);
                            let XN = XL.powf(XM);
                            let XO = ((WO * XC) + (XD * WN)) * (XM * (XL.powf(staged[201])));
                            XP = XN;
                            XQ = XO;
                        }
                        let XR = UX + XP;
                        let XS = (UX * XP) / XR;
                        let XT = (((VC * XP) + (XQ * UX)) - ((VC + XQ) * XS)) / XR;
                        let XU = WN / XA;
                        let XV = (NT * XU).sqrt();
                        let XW = (((WO - (XB * XU)) / XA) * NT) * (CQ / (CP * XV));
                        let XY = XX * WP;
                        let XZ = ((XY * XA) - (XX * WY)) + (HY * (WN * XC));
                        let YA = ((((WQ * XX) * XA) + (XB * XY)) - (WZ * XX)) + (((WO * XC) + (XD * WN)) * HY);
                        let YB = ((CT * (WP * XA)) - WY) - AK;
                        let YC = YB * XV;
                        let YD = (((((WQ * XA) + (XB * WP)) * CT) - WZ) * XV) + (XW * YB);
                        let YE = YC * YC;
                        let YF = YD * YC;
                        let YG = YF + YF;
                        let YH = if YC > DP { 1.0 } else { 0.0 };
                        let YO;
                        let YP;
                        if YH != 0.0 {
                            let YI = AK + (OH * YC);
                            let YJ = AK / YI;
                            let YK = (((YD * OH) * YJ) * AJ) / YI;
                            YO = YJ;
                            YP = YK;
                        } else {
                            let YL = AK - (OH * YC);
                            let YM = AK / YL;
                            let YN = ((((YD * OH) * AJ) * YM) * AJ) / YL;
                            YO = YM;
                            YP = YN;
                        }
                        let YQ = (-YE) + XZ;
                        let YR = (YG * AJ) + YA;
                        let YS = if YQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ZD;
                        let ZE;
                        if YS != 0.0 {
                            let YT = YQ.exp();
                            let YU = YR * YT;
                            ZD = YT;
                            ZE = YU;
                        } else {
                            let YV = -2.3025850929940458e2f64 - YQ;
                            let YW = YR * AJ;
                            let YX = -2.3025850929940458e2f64 - YQ;
                            let YY = AK + ((-2.3025850929940458e2f64 - YQ) * HW);
                            let YZ = AK + (HY * (YX * YY));
                            let ZA = AK + (YV * YZ);
                            let ZB = AM / ZA;
                            let ZC = ((((YW * YZ) + ((((YW * YY) + ((YW * HW) * YX)) * HY) * YV)) * ZB) * AJ) / ZA;
                            ZD = ZB;
                            ZE = ZC;
                        }
                        let ZF = YO * YO;
                        let ZG = YP * YO;
                        let ZH = ZG + ZG;
                        let ZI = ((PF * YO) + (PJ * ZF)) + (PK * (ZF * YO));
                        let ZJ = ZI * ZD;
                        let ZK = ((((YP * PF) + (ZH * PJ)) + (((ZH * YO) + (YP * ZF)) * PK)) * ZD) + (ZE * ZI);
                        let ZM;
                        let ZN;
                        if YH != 0.0 {
                            ZM = ZJ;
                            ZN = ZK;
                        } else {
                            let ZL = if XZ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AAF;
                            let AAG;
                            if ZL != 0.0 {
                                let ZV = XZ.exp();
                                let ZW = YA * ZV;
                                AAF = ZV;
                                AAG = ZW;
                            } else {
                                let ZX = -2.3025850929940458e2f64 - XZ;
                                let ZY = YA * AJ;
                                let ZZ = -2.3025850929940458e2f64 - XZ;
                                let AAA = AK + ((-2.3025850929940458e2f64 - XZ) * HW);
                                let AAB = AK + (HY * (ZZ * AAA));
                                let AAC = AK + (ZX * AAB);
                                let AAD = AM / AAC;
                                let AAE = ((((ZY * AAB) + ((((ZY * AAA) + ((ZY * HW) * ZZ)) * HY) * ZX)) * AAD) * AJ) / AAC;
                                AAF = AAD;
                                AAG = AAE;
                            }
                            let AAH = (CT * AAF) - ZJ;
                            let AAI = (AAG * CT) - ZK;
                            ZM = AAH;
                            ZN = AAI;
                        }
                        let ZO = (XX * ZM) / XV;
                        let ZQ = ZP * ZO;
                        let ZR = UY * ZQ;
                        let ZT = ZS * (ZR * XS);
                        let ZU = ((((VD * ZQ) + (((((ZN * XX) - (XW * ZO)) / XV) * ZP) * UY)) * XS) + (XT * ZR)) * ZS;
                        XF = ZT;
                        XG = ZU;
                    }
                    let AAK;
                    let AAL;
                    if XH != 0.0 {
                        AAK = DP;
                        AAL = DQ;
                    } else {
                        let AAT;
                        let AAU;
                        if AAJ != 0.0 {
                            let AAO = ((AAN - GD) * VS).sqrt();
                            let AAP = ((GK * AJ) * VS) * (CQ / (CP * AAO));
                            AAT = AAO;
                            AAU = AAP;
                        } else {
                            let AAQ = (AAN - GD) * VS;
                            let AAR = AAQ.powf(VW);
                            let AAS = ((GK * AJ) * VS) * (VW * (AAQ.powf(staged[202])));
                            AAT = AAR;
                            AAU = AAS;
                        }
                        let AAW = ((AAN - GD) * AAV) / AAT;
                        let AAY = AAX * AAW;
                        let AAZ = ((((GK * AJ) * AAV) - (AAU * AAW)) / AAT) * AAX;
                        let ABA = staged[177] / AAY;
                        let ABB = ((AAZ * ABA) * AJ) / AAY;
                        let ABC = if (ABA.abs()) < GP { 1.0 } else { 0.0 };
                        let ABG;
                        let ABH;
                        if ABC != 0.0 {
                            let ABD = ABA.exp();
                            let ABE = ABB * ABD;
                            ABG = ABD;
                            ABH = ABE;
                        } else {
                            let ABF = if ABA < DP { 1.0 } else { 0.0 };
                            let ACA;
                            let ACB;
                            if ABF != 0.0 {
                                let ABN = -2.3025850929940458e2f64 - ABA;
                                let ABO = ABB * AJ;
                                let ABP = -2.3025850929940458e2f64 - ABA;
                                let ABQ = AK + ((-2.3025850929940458e2f64 - ABA) * HW);
                                let ABR = AK + (HY * (ABP * ABQ));
                                let ABS = AK + (ABN * ABR);
                                let ABT = AM / ABS;
                                let ABU = ((((ABO * ABR) + ((((ABO * ABQ) + ((ABO * HW) * ABP)) * HY) * ABN)) * ABT) * AJ) / ABS;
                                ACA = ABT;
                                ACB = ABU;
                            } else {
                                let ABV = ABA - GP;
                                let ABW = AK + (ABV * HW);
                                let ABX = AK + (HY * (ABV * ABW));
                                let ABY = IK * (AK + (ABV * ABX));
                                let ABZ = ((ABB * ABX) + ((((ABB * ABW) + ((ABB * HW) * ABV)) * HY) * ABV)) * IK;
                                ACA = ABY;
                                ACB = ABZ;
                            }
                            ABG = ACA;
                            ABH = ACB;
                        }
                        let ABI = I * AAY;
                        let ABJ = ABI * AAY;
                        let ABL = ABK * (ABJ * ABG);
                        let ABM = ((((((J * AAY) + (AAZ * I)) * AAY) + (AAZ * ABI)) * ABG) + (ABH * ABJ)) * ABK;
                        AAK = ABL;
                        AAL = ABM;
                    }
                    let ACD;
                    let ACE;
                    if AAM != 0.0 {
                        ACD = AK;
                        ACE = DQ;
                    } else {
                        let ACC = if GE > staged[178] { 1.0 } else { 0.0 };
                        let ACO;
                        let ACP;
                        if ACC != 0.0 {
                            let ACK = if ACJ == SO { 1.0 } else { 0.0 };
                            let ADC;
                            let ADD;
                            if ACK != 0.0 {
                                let ACR = GE * ACQ;
                                let ACS = GL * ACQ;
                                let ACT = ACR * ACR;
                                let ACU = ACS * ACR;
                                let ACV = ACT * ACR;
                                let ACW = ACV * ACR;
                                let ACX = ((((ACU + ACU) * ACR) + (ACS * ACT)) * ACR) + (ACS * ACV);
                                ADC = ACW;
                                ADD = ACX;
                            } else {
                                let ACY = GE * ACQ;
                                let ACZ = ACY.abs();
                                let ADA = ACZ.powf(ACJ);
                                let ADB = ((GL * ACQ) * ((CP * (if ACY >= TG { 1.0 } else { 0.0 })) - CQ)) * (ACJ * (ACZ.powf((ACJ - CQ))));
                                ADC = ADA;
                                ADD = ADB;
                            }
                            let ADE = AK - ADC;
                            let ADF = AK / ADE;
                            let ADG = (((ADD * AJ) * ADF) * AJ) / ADE;
                            ACO = ADF;
                            ACP = ADG;
                        } else {
                            let ACM = GL * ACL;
                            let ACN = staged[46] + ((GE + (SQ * parameters[51])) * ACL);
                            ACO = ACN;
                            ACP = ACM;
                        }
                        ACD = ACO;
                        ACE = ACP;
                    }
                    let ACF = SI * (((TZ + UZ) + XF) + AAK);
                    let ACG = ACF * ACD;
                    let ACH = (((((UA + VE) + XG) + AAL) * SI) * ACD) + (ACE * ACF);
                    let ADM;
                    let ADN;
                    if ACI != 0.0 {
                        let ADH = (AK - (GF * EL)).sqrt();
                        let ADI = ((GM * EL) * AJ) * (CQ / (CP * ADH));
                        ADM = ADH;
                        ADN = ADI;
                    } else {
                        let ADJ = AK - (GF * EL);
                        let ADK = ADJ.powf(EP);
                        let ADL = ((GM * EL) * AJ) * (EP * (ADJ.powf(staged[203])));
                        ADM = ADK;
                        ADN = ADL;
                    }
                    let ADO = TV * ((EU * (AK - ADM)) + (EV * (I - GF)));
                    let ADP = (((ADN * AJ) * EU) + ((J - GM) * EV)) * TV;
                    UC = UV;
                    UD = UW;
                    UE = UX;
                    UF = UY;
                    UG = ACG;
                    UH = ADO;
                    UI = VA;
                    UJ = VB;
                    UK = VC;
                    UL = VD;
                    UM = ACH;
                    UN = ADP;
                }
                let ADU;
                let ADV;
                let ADW;
                let ADX;
                if D != 0.0 {
                    ADU = DP;
                    ADV = DP;
                    ADW = DQ;
                    ADX = DQ;
                } else {
                    let ADR = ADQ * FZ;
                    let ADS = GG * ADQ;
                    let AEH;
                    let AEI;
                    let AEJ;
                    let AEK;
                    let AEL;
                    let AEM;
                    let AEN;
                    let AEO;
                    let AEP;
                    let AEQ;
                    if ADT != 0.0 {
                        AEH = UC;
                        AEI = UD;
                        AEJ = UE;
                        AEK = UF;
                        AEL = DP;
                        AEM = UI;
                        AEN = UJ;
                        AEO = UK;
                        AEP = UL;
                        AEQ = DQ;
                    } else {
                        let AEA = staged[180] - GA;
                        let AEB = GH * AJ;
                        let AEC = GB / AEA;
                        let AED = (AK - AEC).sqrt();
                        let AEE = AK - AED;
                        let AEF = ((((GI - (AEB * AEC)) / AEA) * AJ) * (CQ / (CP * AED))) * AJ;
                        let AFA;
                        let AFB;
                        if AEG != 0.0 {
                            AFA = DP;
                            AFB = DQ;
                        } else {
                            let AES = AEE * AEE;
                            let AET = AEF * AEE;
                            let AEU = AEE.ln();
                            let AEV = AK - AEE;
                            let AEW = (AES * AEU) / AEV;
                            let AEY = (AEW + AEE) * AEX;
                            let AEZ = ((((((AET + AET) * AEU) + ((AEF * (CQ / AEE)) * AES)) - ((AEF * AJ) * AEW)) / AEV) + AEF) * AEX;
                            AFA = AEY;
                            AFB = AEZ;
                        }
                        let AFC = AEE + AFA;
                        let AFD = AEF + AFB;
                        let AFL;
                        let AFM;
                        if AEG != 0.0 {
                            let AFF = (AEA * AFE).sqrt();
                            let AFG = (AEB * AFE) * (CQ / (CP * AFF));
                            AFL = AFF;
                            AFM = AFG;
                        } else {
                            let AFH = AEA * AFE;
                            let AFJ = AFH.powf(AFI);
                            let AFK = (AEB * AFE) * (AFI * (AFH.powf(staged[204])));
                            AFL = AFJ;
                            AFM = AFK;
                        }
                        let AFO = AFN * AFL;
                        let AFP = AFM * AFN;
                        let AFQ = GC - AK;
                        let AFS = AFR * (AFQ * AFO);
                        let AFT = ((GJ * AFO) + (AFP * AFQ)) * AFR;
                        let AFV = AFU * (AFS * AFC);
                        let AFW = ((AFT * AFC) + (AFD * AFS)) * AFU;
                        AEH = AFO;
                        AEI = AEA;
                        AEJ = AFC;
                        AEK = AFS;
                        AEL = AFV;
                        AEM = AFP;
                        AEN = AEB;
                        AEO = AFD;
                        AEP = AFT;
                        AEQ = AFW;
                    }
                    let AGR;
                    let AGS;
                    if AER != 0.0 {
                        AGR = DP;
                        AGS = DQ;
                    } else {
                        let AFX = (AEH * FF) / AEI;
                        let AFZ = AFY * AFX;
                        let AGA = (((AEM * FF) - (AEN * AFX)) / AEI) * AFY;
                        let AGB = staged[184] / AFZ;
                        let AGC = ((AGA * AGB) * AJ) / AFZ;
                        let AGD = AGB * AGB;
                        let AGE = AGC * AGB;
                        let AGF = AGD * AGD;
                        let AGG = (AGE + AGE) * AGD;
                        let AGH = AGG + AGG;
                        let AGI = AGF + AK;
                        let AGJ = AGF / AGI;
                        let AGK = AGJ.sqrt();
                        let AGL = ((AGH - (AGH * AGJ)) / AGI) * (CQ / (CP * AGK));
                        let AGM = AGK.sqrt();
                        let AGN = AGL * (CQ / (CP * AGM));
                        let AGO = AGK * AGM;
                        let AGP = (AGL * AGM) + (AGN * AGK);
                        let AHB;
                        let AHC;
                        if AGQ != 0.0 {
                            let AGU = AK + (AFZ * AGO);
                            let AGV = AK / AGU;
                            let AGW = ((((AGA * AGO) + (AGP * AFZ)) * AGV) * AJ) / AGU;
                            AHB = AGV;
                            AHC = AGW;
                        } else {
                            let AGX = AK + (AFZ * AGO);
                            let AGZ = AGX.powf(AGY);
                            let AHA = ((AGA * AGO) + (AGP * AFZ)) * (AGY * (AGX.powf(staged[205])));
                            AHB = AGZ;
                            AHC = AHA;
                        }
                        let AHD = AEJ + AHB;
                        let AHE = (AEJ * AHB) / AHD;
                        let AHF = (((AEO * AHB) + (AHC * AEJ)) - ((AEO + AHC) * AHE)) / AHD;
                        let AHG = AFZ / AGM;
                        let AHH = (NT * AHG).sqrt();
                        let AHI = (((AGA - (AGN * AHG)) / AGM) * NT) * (CQ / (CP * AHH));
                        let AHK = AHJ * AGB;
                        let AHL = ((AHK * AGM) - (AHJ * AGK)) + (HY * (AFZ * AGO));
                        let AHM = ((((AGC * AHJ) * AGM) + (AGN * AHK)) - (AGL * AHJ)) + (((AGA * AGO) + (AGP * AFZ)) * HY);
                        let AHN = ((CT * (AGB * AGM)) - AGK) - AK;
                        let AHO = AHN * AHH;
                        let AHP = (((((AGC * AGM) + (AGN * AGB)) * CT) - AGL) * AHH) + (AHI * AHN);
                        let AHQ = AHO * AHO;
                        let AHR = AHP * AHO;
                        let AHS = AHR + AHR;
                        let AHT = if AHO > DP { 1.0 } else { 0.0 };
                        let AIA;
                        let AIB;
                        if AHT != 0.0 {
                            let AHU = AK + (OH * AHO);
                            let AHV = AK / AHU;
                            let AHW = (((AHP * OH) * AHV) * AJ) / AHU;
                            AIA = AHV;
                            AIB = AHW;
                        } else {
                            let AHX = AK - (OH * AHO);
                            let AHY = AK / AHX;
                            let AHZ = ((((AHP * OH) * AJ) * AHY) * AJ) / AHX;
                            AIA = AHY;
                            AIB = AHZ;
                        }
                        let AIC = (-AHQ) + AHL;
                        let AID = (AHS * AJ) + AHM;
                        let AIE = if AIC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AIP;
                        let AIQ;
                        if AIE != 0.0 {
                            let AIF = AIC.exp();
                            let AIG = AID * AIF;
                            AIP = AIF;
                            AIQ = AIG;
                        } else {
                            let AIH = -2.3025850929940458e2f64 - AIC;
                            let AII = AID * AJ;
                            let AIJ = -2.3025850929940458e2f64 - AIC;
                            let AIK = AK + ((-2.3025850929940458e2f64 - AIC) * HW);
                            let AIL = AK + (HY * (AIJ * AIK));
                            let AIM = AK + (AIH * AIL);
                            let AIN = AM / AIM;
                            let AIO = ((((AII * AIL) + ((((AII * AIK) + ((AII * HW) * AIJ)) * HY) * AIH)) * AIN) * AJ) / AIM;
                            AIP = AIN;
                            AIQ = AIO;
                        }
                        let AIR = AIA * AIA;
                        let AIS = AIB * AIA;
                        let AIT = AIS + AIS;
                        let AIU = ((PF * AIA) + (PJ * AIR)) + (PK * (AIR * AIA));
                        let AIV = AIU * AIP;
                        let AIW = ((((AIB * PF) + (AIT * PJ)) + (((AIT * AIA) + (AIB * AIR)) * PK)) * AIP) + (AIQ * AIU);
                        let AIY;
                        let AIZ;
                        if AHT != 0.0 {
                            AIY = AIV;
                            AIZ = AIW;
                        } else {
                            let AIX = if AHL > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AJR;
                            let AJS;
                            if AIX != 0.0 {
                                let AJH = AHL.exp();
                                let AJI = AHM * AJH;
                                AJR = AJH;
                                AJS = AJI;
                            } else {
                                let AJJ = -2.3025850929940458e2f64 - AHL;
                                let AJK = AHM * AJ;
                                let AJL = -2.3025850929940458e2f64 - AHL;
                                let AJM = AK + ((-2.3025850929940458e2f64 - AHL) * HW);
                                let AJN = AK + (HY * (AJL * AJM));
                                let AJO = AK + (AJJ * AJN);
                                let AJP = AM / AJO;
                                let AJQ = ((((AJK * AJN) + ((((AJK * AJM) + ((AJK * HW) * AJL)) * HY) * AJJ)) * AJP) * AJ) / AJO;
                                AJR = AJP;
                                AJS = AJQ;
                            }
                            let AJT = (CT * AJR) - AIV;
                            let AJU = (AJS * CT) - AIW;
                            AIY = AJT;
                            AIZ = AJU;
                        }
                        let AJA = (AHJ * AIY) / AHH;
                        let AJC = AJB * AJA;
                        let AJD = AEK * AJC;
                        let AJF = AJE * (AJD * AHE);
                        let AJG = ((((AEP * AJC) + (((((AIZ * AHJ) - (AHI * AJA)) / AHH) * AJB) * AEK)) * AHE) + (AHF * AJD)) * AJE;
                        AGR = AJF;
                        AGS = AJG;
                    }
                    let AJW;
                    let AJX;
                    if AGT != 0.0 {
                        AJW = DP;
                        AJX = DQ;
                    } else {
                        let AKF;
                        let AKG;
                        if AJV != 0.0 {
                            let AKA = ((AJZ - GD) * AFE).sqrt();
                            let AKB = ((GK * AJ) * AFE) * (CQ / (CP * AKA));
                            AKF = AKA;
                            AKG = AKB;
                        } else {
                            let AKC = (AJZ - GD) * AFE;
                            let AKD = AKC.powf(AFI);
                            let AKE = ((GK * AJ) * AFE) * (AFI * (AKC.powf(staged[206])));
                            AKF = AKD;
                            AKG = AKE;
                        }
                        let AKI = ((AJZ - GD) * AKH) / AKF;
                        let AKK = AKJ * AKI;
                        let AKL = ((((GK * AJ) * AKH) - (AKG * AKI)) / AKF) * AKJ;
                        let AKM = staged[187] / AKK;
                        let AKN = ((AKL * AKM) * AJ) / AKK;
                        let AKO = if (AKM.abs()) < GP { 1.0 } else { 0.0 };
                        let AKS;
                        let AKT;
                        if AKO != 0.0 {
                            let AKP = AKM.exp();
                            let AKQ = AKN * AKP;
                            AKS = AKP;
                            AKT = AKQ;
                        } else {
                            let AKR = if AKM < DP { 1.0 } else { 0.0 };
                            let ALM;
                            let ALN;
                            if AKR != 0.0 {
                                let AKZ = -2.3025850929940458e2f64 - AKM;
                                let ALA = AKN * AJ;
                                let ALB = -2.3025850929940458e2f64 - AKM;
                                let ALC = AK + ((-2.3025850929940458e2f64 - AKM) * HW);
                                let ALD = AK + (HY * (ALB * ALC));
                                let ALE = AK + (AKZ * ALD);
                                let ALF = AM / ALE;
                                let ALG = ((((ALA * ALD) + ((((ALA * ALC) + ((ALA * HW) * ALB)) * HY) * AKZ)) * ALF) * AJ) / ALE;
                                ALM = ALF;
                                ALN = ALG;
                            } else {
                                let ALH = AKM - GP;
                                let ALI = AK + (ALH * HW);
                                let ALJ = AK + (HY * (ALH * ALI));
                                let ALK = IK * (AK + (ALH * ALJ));
                                let ALL = ((AKN * ALJ) + ((((AKN * ALI) + ((AKN * HW) * ALH)) * HY) * ALH)) * IK;
                                ALM = ALK;
                                ALN = ALL;
                            }
                            AKS = ALM;
                            AKT = ALN;
                        }
                        let AKU = I * AKK;
                        let AKV = AKU * AKK;
                        let AKX = AKW * (AKV * AKS);
                        let AKY = ((((((J * AKK) + (AKL * I)) * AKK) + (AKL * AKU)) * AKS) + (AKT * AKV)) * AKW;
                        AJW = AKX;
                        AJX = AKY;
                    }
                    let ALP;
                    let ALQ;
                    if AJY != 0.0 {
                        ALP = AK;
                        ALQ = DQ;
                    } else {
                        let ALO = if GE > staged[188] { 1.0 } else { 0.0 };
                        let ALZ;
                        let AMA;
                        if ALO != 0.0 {
                            let ALV = if ALU == SO { 1.0 } else { 0.0 };
                            let AMN;
                            let AMO;
                            if ALV != 0.0 {
                                let AMC = GE * AMB;
                                let AMD = GL * AMB;
                                let AME = AMC * AMC;
                                let AMF = AMD * AMC;
                                let AMG = AME * AMC;
                                let AMH = AMG * AMC;
                                let AMI = ((((AMF + AMF) * AMC) + (AMD * AME)) * AMC) + (AMD * AMG);
                                AMN = AMH;
                                AMO = AMI;
                            } else {
                                let AMJ = GE * AMB;
                                let AMK = AMJ.abs();
                                let AML = AMK.powf(ALU);
                                let AMM = ((GL * AMB) * ((CP * (if AMJ >= TG { 1.0 } else { 0.0 })) - CQ)) * (ALU * (AMK.powf((ALU - CQ))));
                                AMN = AML;
                                AMO = AMM;
                            }
                            let AMP = AK - AMN;
                            let AMQ = AK / AMP;
                            let AMR = (((AMO * AJ) * AMQ) * AJ) / AMP;
                            ALZ = AMQ;
                            AMA = AMR;
                        } else {
                            let ALX = GL * ALW;
                            let ALY = staged[58] + ((GE + (SQ * parameters[52])) * ALW);
                            ALZ = ALY;
                            AMA = ALX;
                        }
                        ALP = ALZ;
                        ALQ = AMA;
                    }
                    let ALR = SI * (((ADR + AEL) + AGR) + AJW);
                    let ALS = ALR * ALP;
                    let ALT = (((((ADS + AEQ) + AGS) + AJX) * SI) * ALP) + (ALQ * ALR);
                    let AMV;
                    let AMW;
                    if A != 0.0 {
                        let AMT = if I < AMS { 1.0 } else { 0.0 };
                        let ANC;
                        let AND;
                        if AMT != 0.0 {
                            let AMY = (I - AMS) / AMX;
                            let AMZ = J / AMX;
                            let ANA = if AMY < -3.7e1f64 { 1.0 } else { 0.0 };
                            let ANU;
                            let ANV;
                            if ANA != 0.0 {
                                ANU = AMS;
                                ANV = DQ;
                            } else {
                                let ANQ = AMY.exp();
                                let ANR = AK + ANQ;
                                let ANS = ((AMZ * ANQ) * (CQ / ANR)) * AMX;
                                let ANT = AMS + ((ANR.ln()) * AMX);
                                ANU = ANT;
                                ANV = ANS;
                            }
                            ANC = ANU;
                            AND = ANV;
                        } else {
                            let ANB = if ((I - AMS) / AMX) > 3.7e1f64 { 1.0 } else { 0.0 };
                            let AOA;
                            let AOB;
                            if ANB != 0.0 {
                                AOA = I;
                                AOB = J;
                            } else {
                                let ANW = ((AMS - I) / AMX).exp();
                                let ANX = AK + ANW;
                                let ANY = I + ((ANX.ln()) * AMX);
                                let ANZ = J + (((((J * AJ) / AMX) * ANW) * (CQ / ANX)) * AMX);
                                AOA = ANY;
                                AOB = ANZ;
                            }
                            ANC = AOA;
                            AND = AOB;
                        }
                        let ANF = (SO * ANE) * ANE;
                        let ANG = ANE * (ANE / CL);
                        let ANH = ANC + ANG;
                        let ANI = CL - ANH;
                        let ANJ = (AND * AJ) * ANI;
                        let ANK = ((ANI * ANI) + ANF).sqrt();
                        let ANL = (CL + ANH) + ANK;
                        let ANM = (ANC * CL) / ANL;
                        let ANN = CT * ANM;
                        let ANO = (((AND * CL) - ((AND + ((ANJ + ANJ) * (CQ / (CP * ANK)))) * ANM)) / ANL) * CT;
                        let ANP = if FF == HY { 1.0 } else { 0.0 };
                        let AOH;
                        let AOI;
                        if ANP != 0.0 {
                            let AOC = (AK - (ANN * FB)).sqrt();
                            let AOD = ((ANO * FB) * AJ) * (CQ / (CP * AOC));
                            AOH = AOC;
                            AOI = AOD;
                        } else {
                            let AOE = AK - (ANN * FB);
                            let AOF = AOE.powf(FF);
                            let AOG = ((ANO * FB) * AJ) * (FF * (AOE.powf((FF - CQ))));
                            AOH = AOF;
                            AOI = AOG;
                        }
                        let AOJ = TV * ((FK * (AK - AOH)) + (FL * (ANC - ANN)));
                        let AOK = (((AOI * AJ) * FK) + ((AND - ANO) * FL)) * TV;
                        let AOL = (I + AMS) - ANC;
                        let AOM = J - AND;
                        let AON = AOL + ANG;
                        let AOO = CL - AON;
                        let AOP = (AOM * AJ) * AOO;
                        let AOQ = ((AOO * AOO) + ANF).sqrt();
                        let AOR = (CL + AON) + AOQ;
                        let AOS = (AOL * CL) / AOR;
                        let AOT = CT * AOS;
                        let AOU = (((AOM * CL) - ((AOM + ((AOP + AOP) * (CQ / (CP * AOQ)))) * AOS)) / AOR) * CT;
                        let AOW = if AOV == HY { 1.0 } else { 0.0 };
                        let APD;
                        let APE;
                        if AOW != 0.0 {
                            let AOY = (AK - (AOT * AOX)).sqrt();
                            let AOZ = ((AOU * AOX) * AJ) * (CQ / (CP * AOY));
                            APD = AOY;
                            APE = AOZ;
                        } else {
                            let APA = AK - (AOT * AOX);
                            let APB = APA.powf(AOV);
                            let APC = ((AOU * AOX) * AJ) * (AOV * (APA.powf((AOV - CQ))));
                            APD = APB;
                            APE = APC;
                        }
                        let APH = AOJ + (TV * ((APF * (AK - APD)) + (APG * (AOL - AOT))));
                        let API = AOK + ((((APE * AJ) * APF) + ((AOM - AOU) * APG)) * TV);
                        AMV = APH;
                        AMW = API;
                    } else {
                        let APO;
                        let APP;
                        if AMU != 0.0 {
                            let APJ = (AK - (GF * FB)).sqrt();
                            let APK = ((GM * FB) * AJ) * (CQ / (CP * APJ));
                            APO = APJ;
                            APP = APK;
                        } else {
                            let APL = AK - (GF * FB);
                            let APM = APL.powf(FF);
                            let APN = ((GM * FB) * AJ) * (FF * (APL.powf(staged[207])));
                            APO = APM;
                            APP = APN;
                        }
                        let APQ = TV * ((FK * (AK - APO)) + (FL * (I - GF)));
                        let APR = (((APP * AJ) * FK) + ((J - GM) * FL)) * TV;
                        AMV = APQ;
                        AMW = APR;
                    }
                    ADU = ALS;
                    ADV = AMV;
                    ADW = ALT;
                    ADX = AMW;
                }
                let ADY = ((X * KE) + (Y * UG)) + (Z * ADU);
                let ADZ = ((KK * X) + (UM * Y)) + (ADW * Z);
                P = KF;
                Q = UH;
                R = ADV;
                S = ADY;
                T = KL;
                U = UN;
                V = ADX;
                W = ADZ;
            }
            let AB = AA * S;
            let AC = W * AA;
            let AE = AD * (((X * P) + (Y * Q)) + (Z * R));
            let AF = (((T * X) + (U * Y)) + (V * Z)) * AD;
            let AG = ddt(37556, AE);
            let AI = AF * AH;
            let APS = AC[0];
            let APT = AC[1];
            let APU = AI[0];
            let APV = AI[1];
            let APW = AF[0];
            let APX = AF[1];
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(1),
            multiplicity * (AB),
            [0, 1],
            [APS, APT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(1),
            multiplicity * (AG),
            [0, 1],
            [APU, APV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(1),
            multiplicity * (APY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = AB;
        self.canonical_reactive[1] = AE;
        self.canonical_reactive[2] = APW;
        self.canonical_reactive[3] = APX;
        self.canonical_reactive[4] = APY;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(0),
            Some(1),
            &[0, 1],
            &[cached[2], cached[3]],
            &[],
            &[],
            multiplicity,
        );
    }

}
