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
        let mut key = Vec::with_capacity(2198);
        for index in 0..Self::PARAMETER_COUNT {
            if PARAMETER_MODEL_FLAGS[index] {
                key.push(self.params.values[index].to_bits());
                key.push(u64::from(self.param_given[index]));
            }
        }
        key.into_boxed_slice()
    }

    fn canonical_install_model_values(&mut self, values: Arc<CanonicalModelValues>) {
        self.canonical_staged[448] = values[0];
        self.canonical_staged[161] = values[1];
        self.canonical_staged[164] = values[2];
        self.canonical_staged[239] = values[3];
        self.canonical_staged[449] = values[4];
        self.canonical_staged[0] = values[5];
        self.canonical_staged[1] = values[6];
        self.canonical_staged[2] = values[7];
        self.canonical_staged[3] = values[8];
        self.canonical_staged[461] = values[9];
        self.canonical_staged[463] = values[10];
        self.canonical_staged[467] = values[11];
        self.canonical_staged[468] = values[12];
        self.canonical_staged[4] = values[13];
        self.canonical_staged[5] = values[14];
        self.canonical_staged[6] = values[15];
        self.canonical_staged[7] = values[16];
        self.canonical_staged[8] = values[17];
        self.canonical_staged[9] = values[18];
        self.canonical_staged[10] = values[19];
        self.canonical_staged[469] = values[20];
        self.canonical_staged[470] = values[21];
        self.canonical_staged[11] = values[22];
        self.canonical_staged[12] = values[23];
        self.canonical_staged[13] = values[24];
        self.canonical_staged[14] = values[25];
        self.canonical_staged[15] = values[26];
        self.canonical_staged[16] = values[27];
        self.canonical_staged[17] = values[28];
        self.canonical_staged[18] = values[29];
        self.canonical_staged[19] = values[30];
        self.canonical_staged[20] = values[31];
        self.canonical_staged[21] = values[32];
        self.canonical_staged[22] = values[33];
        self.canonical_staged[23] = values[34];
        self.canonical_staged[24] = values[35];
        self.canonical_staged[25] = values[36];
        self.canonical_staged[26] = values[37];
        self.canonical_staged[27] = values[38];
        self.canonical_staged[28] = values[39];
        self.canonical_staged[29] = values[40];
        self.canonical_staged[30] = values[41];
        self.canonical_staged[31] = values[42];
        self.canonical_staged[32] = values[43];
        self.canonical_staged[33] = values[44];
        self.canonical_staged[34] = values[45];
        self.canonical_staged[35] = values[46];
        self.canonical_staged[36] = values[47];
        self.canonical_staged[37] = values[48];
        self.canonical_staged[38] = values[49];
        self.canonical_staged[39] = values[50];
        self.canonical_staged[40] = values[51];
        self.canonical_staged[41] = values[52];
        self.canonical_staged[42] = values[53];
        self.canonical_staged[43] = values[54];
        self.canonical_staged[44] = values[55];
        self.canonical_staged[45] = values[56];
        self.canonical_staged[46] = values[57];
        self.canonical_staged[47] = values[58];
        self.canonical_staged[48] = values[59];
        self.canonical_staged[49] = values[60];
        self.canonical_staged[471] = values[61];
        self.canonical_staged[472] = values[62];
        self.canonical_staged[50] = values[63];
        self.canonical_staged[51] = values[64];
        self.canonical_staged[52] = values[65];
        self.canonical_staged[486] = values[66];
        self.canonical_staged[488] = values[67];
        self.canonical_staged[502] = values[68];
        self.canonical_staged[53] = values[69];
        self.canonical_staged[54] = values[70];
        self.canonical_staged[297] = values[71];
        self.canonical_staged[56] = values[72];
        self.canonical_staged[55] = values[73];
        self.canonical_staged[57] = values[74];
        self.canonical_staged[58] = values[75];
        self.canonical_staged[59] = values[76];
        self.canonical_staged[826] = values[77];
        self.canonical_staged[61] = values[78];
        self.canonical_staged[295] = values[79];
        self.canonical_staged[62] = values[80];
        self.canonical_staged[66] = values[81];
        self.canonical_staged[70] = values[82];
        self.canonical_staged[71] = values[83];
        self.canonical_staged[828] = values[84];
        self.canonical_staged[182] = values[85];
        self.canonical_staged[829] = values[86];
        self.canonical_staged[95] = values[87];
        self.canonical_staged[123] = values[88];
        self.canonical_staged[124] = values[89];
        self.canonical_staged[125] = values[90];
        self.canonical_staged[126] = values[91];
        self.canonical_staged[128] = values[92];
        self.canonical_staged[129] = values[93];
        self.canonical_staged[130] = values[94];
        self.canonical_staged[133] = values[95];
        self.canonical_staged[134] = values[96];
        self.canonical_staged[135] = values[97];
        self.canonical_staged[132] = values[98];
        self.canonical_staged[851] = values[99];
        self.canonical_staged[867] = values[100];
        self.canonical_staged[855] = values[101];
        self.canonical_staged[146] = values[102];
        self.canonical_staged[147] = values[103];
        self.canonical_staged[148] = values[104];
        self.canonical_staged[149] = values[105];
        self.canonical_staged[178] = values[106];
        self.canonical_staged[180] = values[107];
        self.canonical_staged[286] = values[108];
        self.canonical_staged[183] = values[109];
        self.canonical_staged[192] = values[110];
        self.canonical_staged[858] = values[111];
        self.canonical_staged[863] = values[112];
        self.canonical_staged[409] = values[113];
        self.canonical_staged[210] = values[114];
        self.canonical_staged[868] = values[115];
        self.canonical_staged[212] = values[116];
        self.canonical_staged[871] = values[117];
        self.canonical_staged[213] = values[118];
        self.canonical_staged[214] = values[119];
        self.canonical_staged[215] = values[120];
        self.canonical_staged[869] = values[121];
        self.canonical_staged[216] = values[122];
        self.canonical_staged[874] = values[123];
        self.canonical_staged[217] = values[124];
        self.canonical_staged[218] = values[125];
        self.canonical_staged[219] = values[126];
        self.canonical_staged[873] = values[127];
        self.canonical_staged[221] = values[128];
        self.canonical_staged[220] = values[129];
        self.canonical_staged[222] = values[130];
        self.canonical_staged[223] = values[131];
        self.canonical_staged[224] = values[132];
        self.canonical_staged[226] = values[133];
        self.canonical_staged[225] = values[134];
        self.canonical_staged[227] = values[135];
        self.canonical_staged[228] = values[136];
        self.canonical_staged[229] = values[137];
        self.canonical_staged[231] = values[138];
        self.canonical_staged[230] = values[139];
        self.canonical_staged[232] = values[140];
        self.canonical_staged[233] = values[141];
        self.canonical_staged[234] = values[142];
        self.canonical_staged[866] = values[143];
        self.canonical_staged[235] = values[144];
        self.canonical_staged[236] = values[145];
        self.canonical_staged[876] = values[146];
        self.canonical_staged[238] = values[147];
        self.canonical_staged[240] = values[148];
        self.canonical_staged[877] = values[149];
        self.canonical_staged[878] = values[150];
        self.canonical_staged[242] = values[151];
        self.canonical_staged[243] = values[152];
        self.canonical_staged[244] = values[153];
        self.canonical_staged[879] = values[154];
        self.canonical_staged[881] = values[155];
        self.canonical_staged[883] = values[156];
        self.canonical_staged[249] = values[157];
        self.canonical_staged[251] = values[158];
        self.canonical_staged[885] = values[159];
        self.canonical_staged[252] = values[160];
        self.canonical_staged[253] = values[161];
        self.canonical_staged[884] = values[162];
        self.canonical_staged[258] = values[163];
        self.canonical_staged[265] = values[164];
        self.canonical_staged[887] = values[165];
        self.canonical_staged[886] = values[166];
        self.canonical_staged[296] = values[167];
        self.canonical_staged[308] = values[168];
        self.canonical_staged[299] = values[169];
        self.canonical_staged[300] = values[170];
        self.canonical_staged[890] = values[171];
        self.canonical_staged[302] = values[172];
        self.canonical_staged[303] = values[173];
        self.canonical_staged[891] = values[174];
        self.canonical_staged[305] = values[175];
        self.canonical_staged[306] = values[176];
        self.canonical_staged[892] = values[177];
        self.canonical_staged[301] = values[178];
        self.canonical_staged[304] = values[179];
        self.canonical_staged[307] = values[180];
        self.canonical_staged[310] = values[181];
        self.canonical_staged[311] = values[182];
        self.canonical_staged[895] = values[183];
        self.canonical_staged[313] = values[184];
        self.canonical_staged[314] = values[185];
        self.canonical_staged[896] = values[186];
        self.canonical_staged[316] = values[187];
        self.canonical_staged[317] = values[188];
        self.canonical_staged[897] = values[189];
        self.canonical_staged[312] = values[190];
        self.canonical_staged[315] = values[191];
        self.canonical_staged[318] = values[192];
        self.canonical_staged[898] = values[193];
        self.canonical_staged[900] = values[194];
        self.canonical_staged[325] = values[195];
        self.canonical_staged[326] = values[196];
        self.canonical_staged[333] = values[197];
        self.canonical_staged[904] = values[198];
        self.canonical_staged[905] = values[199];
        self.canonical_staged[338] = values[200];
        self.canonical_staged[339] = values[201];
        self.canonical_staged[340] = values[202];
        self.canonical_staged[902] = values[203];
        self.canonical_staged[906] = values[204];
        self.canonical_staged[907] = values[205];
        self.canonical_staged[351] = values[206];
        self.canonical_staged[353] = values[207];
        self.canonical_staged[354] = values[208];
        self.canonical_staged[909] = values[209];
        self.canonical_staged[360] = values[210];
        self.canonical_staged[361] = values[211];
        self.canonical_staged[362] = values[212];
        self.canonical_staged[363] = values[213];
        self.canonical_staged[364] = values[214];
        self.canonical_staged[910] = values[215];
        self.canonical_staged[911] = values[216];
        self.canonical_staged[912] = values[217];
        self.canonical_staged[369] = values[218];
        self.canonical_staged[375] = values[219];
        self.canonical_staged[380] = values[220];
        self.canonical_staged[914] = values[221];
        self.canonical_staged[915] = values[222];
        self.canonical_staged[408] = values[223];
        self.canonical_staged[928] = values[224];
        self.canonical_staged[929] = values[225];
        self.canonical_staged[930] = values[226];
        self.canonical_staged[931] = values[227];
        self.canonical_staged[932] = values[228];
        self.canonical_staged[933] = values[229];
        self.canonical_staged[934] = values[230];
        self.canonical_staged[935] = values[231];
        self.canonical_staged[936] = values[232];
        self.canonical_staged[937] = values[233];
        self.canonical_staged[954] = values[234];
        self.canonical_staged[425] = values[235];
        self.canonical_staged[426] = values[236];
        self.canonical_staged[428] = values[237];
        self.canonical_staged[429] = values[238];
        self.canonical_staged[430] = values[239];
        self.canonical_staged[431] = values[240];
        self.canonical_staged[432] = values[241];
        self.canonical_staged[433] = values[242];
        self.canonical_staged[434] = values[243];
        self.canonical_staged[435] = values[244];
        self.canonical_staged[436] = values[245];
        self.canonical_staged[437] = values[246];
        self.canonical_staged[439] = values[247];
        self.canonical_staged[440] = values[248];
        self.canonical_staged[442] = values[249];
        self.canonical_staged[443] = values[250];
        self.canonical_staged[444] = values[251];
        self.canonical_staged[445] = values[252];
        self.canonical_staged[446] = values[253];
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
                let A = parameters[39];
                let B = 1e0f64;
                let D = -1e0f64;
                let F = parameters[110];
                let G = 8.85418e-12f64;
                let I = parameters[111];
                let K = parameters[77];
                let O = 3.9e0f64;
                let Q = parameters[78];
                let W = 1e-6f64;
                let Z = 0e0f64;
                let AM = parameters[275];
                let CB = 0.0f64;
                let CC = parameters[49];
                let CD = parameters[909];
                let CG = parameters[42];
                let CI = 0e0f64;
                let CQ = parameters[698];
                let CV = parameters[1128];
                let CY = 1e-38f64;
                let DA = 4.97232e-7f64;
                let DB = 3.42537e-7f64;
                let DD = 7.45669e11f64;
                let DE = 1.16645e12f64;
                let DI = parameters[820];
                let DK = 3.0015e2f64;
                let DP = 2e0f64;
                let DU = parameters[283];
                let DW = 5e-1f64;
                let DZ = parameters[1094];
                let EB = 1e0f64;
                let ER = parameters[1110];
                let FE = 1e1f64;
                let FO = parameters[514];
                let FQ = parameters[1098];
                let FS = parameters[515];
                let FV = 4e0f64;
                let FW = parameters[1107];
                let GC = parameters[516];
                let GG = parameters[517];
                let GO = 2.5e-1f64;
                let GP = parameters[1108];
                let HG = parameters[1118];
                let HI = 1.9e-9f64;
                let HM = parameters[1096];
                let IH = parameters[713];
                let IJ = 1e-1f64;
                let IM = 3.8025850929940455e0f64;
                let IN = 5e-2f64;
                let IQ = parameters[715];
                let IU = 3.8025850929940455e0f64;
                let IX = parameters[717];
                let JB = 3.8025850929940455e0f64;
                let JE = parameters[714];
                let JI = 3.8025850929940455e0f64;
                let JL = parameters[716];
                let JP = 3.8025850929940455e0f64;
                let JS = parameters[718];
                let JW = 3.8025850929940455e0f64;
                let KB = parameters[785];
                let KC = 1.60219e-19f64;
                let KH = parameters[48];
                let KK = 0e0f64;
                let KM = parameters[800];
                let KP = 0e0f64;
                let KQ = parameters[798];
                let KU = 0e0f64;
                let LD = 0e0f64;
                let LE = 0e0f64;
                let LF = 0e0f64;
                let LG = 0e0f64;
                let LH = 0e0f64;
                let LK = 0e0f64;
                let MA = parameters[693];
                let ME = parameters[691];
                let MM = 0e0f64;
                let mut oAN = 0.0;
                let mut oAQ = 0.0;
                let mut oAR = 0.0;
                let mut oCE = 0.0;
                let mut oCK = 0.0;
                let mut oCL = 0.0;
                let mut oCM = 0.0;
                let mut oEC = 0.0;
                let mut oED = 0.0;
                let mut oEE = 0.0;
                let mut oEV = 0.0;
                let mut oFH = 0.0;
                let mut oFI = 0.0;
                let mut oFL = 0.0;
                let mut oFM = 0.0;
                let mut oFN = 0.0;
                let mut oFP = 0.0;
                let mut oFR = 0.0;
                let mut oFT = 0.0;
                let mut oFU = 0.0;
                let mut oFX = 0.0;
                let mut oFY = 0.0;
                let mut oFZ = 0.0;
                let mut oGA = 0.0;
                let mut oGB = 0.0;
                let mut oGD = 0.0;
                let mut oGE = 0.0;
                let mut oGF = 0.0;
                let mut oGH = 0.0;
                let mut oGI = 0.0;
                let mut oGJ = 0.0;
                let mut oGK = 0.0;
                let mut oGL = 0.0;
                let mut oGM = 0.0;
                let mut oGN = 0.0;
                let mut oGQ = 0.0;
                let mut oGR = 0.0;
                let mut oGS = 0.0;
                let mut oGT = 0.0;
                let mut oGU = 0.0;
                let mut oGV = 0.0;
                let mut oGW = 0.0;
                let mut oGX = 0.0;
                let mut oGY = 0.0;
                let mut oGZ = 0.0;
                let mut oHA = 0.0;
                let mut oHB = 0.0;
                let mut oHC = 0.0;
                let mut oHD = 0.0;
                let mut oHE = 0.0;
                let mut oHF = 0.0;
                let mut oHH = 0.0;
                let mut oHJ = 0.0;
                let mut oHK = 0.0;
                let mut oHL = 0.0;
                let mut oHN = 0.0;
                let mut oHO = 0.0;
                let mut oHP = 0.0;
                let mut oHQ = 0.0;
                let mut oHR = 0.0;
                let mut oHV = 0.0;
                let mut oHW = 0.0;
                let mut oHX = 0.0;
                let mut oHY = 0.0;
                let mut oHZ = 0.0;
                let mut oIA = 0.0;
                let mut oIB = 0.0;
                let mut oID = 0.0;
                let mut oIE = 0.0;
                let mut oIF = 0.0;
                let mut oKE = 0.0;
                let mut oKJ = 0.0;
                let mut oKN = 0.0;
                let mut oKO = 0.0;
                let mut oKR = 0.0;
                let mut oKS = 0.0;
                let mut oKT = 0.0;
                let mut oKV = 0.0;
                let mut oLN = 0.0;
                let mut oLO = 0.0;
                let mut oLP = 0.0;
                let mut oLQ = 0.0;
                let mut oLY = 0.0;
                let mut oMB = 0.0;
                let mut oMC = 0.0;
                let mut oMD = 0.0;
                let mut oMF = 0.0;
                let mut oMG = 0.0;
                let mut oMH = 0.0;
                let mut oMK = 0.0;
                let C = if A == B { 1.0 } else { 0.0 };
                let E = if C != 0.0 {
                    B
                } else {
                    D
                };
                let H = F * G;
                let J = I * G;
                let L = J / K;
                let M = F / I;
                let N = if (if parameter_given[78] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let R = if N != 0.0 {
                    let P = ((K * I) / O) - parameters[79];
                    P
                } else {
                    Q
                };
                let S = -parameters[61];
                let T = -parameters[62];
                let U = -parameters[67];
                let V = -parameters[68];
                let X = W / parameters[51];
                let Y = W / parameters[55];
                let AA = if parameters[818] != Z { 1.0 } else { 0.0 };
                let AB = if parameters[819] != Z { 1.0 } else { 0.0 };
                let AC = if parameters[817] == B { 1.0 } else { 0.0 };
                let AD = if parameters[44] != Z { 1.0 } else { 0.0 };
                let AE = X.powf(parameters[82]);
                let AF = X.powf(parameters[84]);
                let AG = Y.powf(parameters[86]);
                let AH = X.powf(parameters[215]);
                let AI = Y.powf(parameters[217]);
                let AJ = X.powf(parameters[225]);
                let AK = X.powf(parameters[235]);
                let AL = if parameters[50] != B { 1.0 } else { 0.0 };
                if AL != 0.0 {
                    let AN = if AM > Z { 1.0 } else { 0.0 };
                    oAN = AN;
                    if AN != 0.0 {
                        let AQ = X.powf(AM);
                        oAQ = AQ;
                    } else {
                        let AR = B - parameters[274];
                        oAR = AR;
                    }
                } else {
                }
                let AO = X.powf(parameters[286]);
                let AP = Y.powf(parameters[288]);
                let AS = X.powf(parameters[303]);
                let AT = Y.powf(parameters[305]);
                let AU = X.powf(parameters[310]);
                let AV = X.powf(parameters[328]);
                let AW = Y.powf(parameters[330]);
                let AX = X.powf(parameters[179]);
                let AY = X.powf(parameters[181]);
                let AZ = X.powf(parameters[462]);
                let BA = X.powf(parameters[258]);
                let BB = X.powf(parameters[480]);
                let BC = X.powf(parameters[342]);
                let BD = X.powf(parameters[244]);
                let BE = Y.powf(parameters[246]);
                let BF = X.powf(parameters[424]);
                let BG = X.powf(parameters[439]);
                let BH = X.powf(parameters[486]);
                let BI = Y.powf(parameters[488]);
                let BJ = Y.powf(parameters[496]);
                let BK = Y.powf(parameters[520]);
                let BL = Y.powf(parameters[523]);
                let BM = X.powf(parameters[94]);
                let BN = X.powf(parameters[96]);
                let BO = Y.powf(parameters[98]);
                let BP = X.powf(parameters[121]);
                let BQ = Y.powf(parameters[123]);
                let BR = X.powf(parameters[131]);
                let BS = Y.powf(parameters[133]);
                let BT = X.powf(parameters[264]);
                let BU = Y.powf(parameters[266]);
                let BV = X.powf(parameters[353]);
                let BW = X.powf(parameters[187]);
                let BX = Y.powf(parameters[189]);
                let BY = X.powf(parameters[197]);
                let BZ = Y.powf(parameters[199]);
                let CA = X.powf(parameters[384]);
                let CF;
                if CB != 0.0 {
                    let CE = if (if CC == Z { 1.0 } else { 0.0 }) != 0.0 || (if CD == Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oCE = CE;
                    let CJ = if CE != 0.0 {
                        CI
                    } else {
                        Z
                    };
                    CF = CJ;
                } else {
                    CF = Z;
                }
                let CH = if CG == B { 1.0 } else { 0.0 };
                if CH != 0.0 {
                    let CK = X.powf(parameters[398]);
                    oCK = CK;
                    let CL = X.powf(parameters[408]);
                    oCL = CL;
                } else {
                    let CM = X.powf(parameters[415]);
                    oCM = CM;
                }
                let CN = if parameters[47] != Z { 1.0 } else { 0.0 };
                let CO = if parameters[46] != Z { 1.0 } else { 0.0 };
                let CP = if parameters[1065] == B { 1.0 } else { 0.0 };
                let CR = parameters[695] - CQ;
                let CS = parameters[697] - CQ;
                let CT = if parameters[1097] == B { 1.0 } else { 0.0 };
                let CU = if CT != 0.0 {
                    let CW = B - CV;
                    CW
                } else {
                    B
                };
                let CX = K * K;
                let CZ = (if (parameters[555] / K) >= CY { (parameters[555] / K) } else { CY }).ln();
                let DC = if C != 0.0 {
                    DA
                } else {
                    DB
                };
                let DF = if C != 0.0 {
                    DD
                } else {
                    DE
                };
                let DG = (-DF) * K;
                let DH = if (if CC != Z { 1.0 } else { 0.0 }) != 0.0 && (if CD > Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DJ = if DI <= -2.7315e2f64 { 1.0 } else { 0.0 };
                let DM = if DJ != 0.0 {
                    DK
                } else {
                    let DL = DI + 2.7315e2f64;
                    DL
                };
                let DN = 8.617087e-5f64 * DM;
                let DO = parameters[109] - (((parameters[821] * DM) * DM) / (DM + parameters[822]));
                let DQ = DP * DN;
                let DR = DP * H;
                let DS = (H / J) * K;
                let DT = if A != B { 1.0 } else { 0.0 };
                let DY = if DT != 0.0 {
                    let DV = 3.333333333333333e-1f64 * DU;
                    DV
                } else {
                    let DX = DW * DU;
                    DX
                };
                let EA = if DZ == B { 1.0 } else { 0.0 };
                if EA != 0.0 {
                    let EC = parameters[1120] - EB;
                    oEC = EC;
                    let ED = -parameters[1121];
                    oED = ED;
                    let EE = ED - EB;
                    oEE = EE;
                } else {
                }
                let EF = DO / DN;
                let EG = DO * parameters[897];
                let EH = DO * parameters[899];
                let EI = DO * parameters[901];
                let EJ = DO * parameters[898];
                let EK = DO * parameters[900];
                let EL = DO * parameters[902];
                let EM = CR + parameters[696];
                let EN = CR + CR;
                let EO = CS + CS;
                let EP = EM + EM;
                let EQ = if parameters[43] == B { 1.0 } else { 0.0 };
                let ES = if ER != Z { 1.0 } else { 0.0 };
                let ET = if parameters[1095] == B { 1.0 } else { 0.0 };
                let EU = if (if (if ES != 0.0 && CH != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ET != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EA != 0.0 { 1.0 } else { 0.0 };
                if EU != 0.0 {
                    let EV = E * (B - (parameters[1111] / ER));
                    oEV = EV;
                } else {
                }
                let EW = DP / parameters[956];
                let EX = EW * 6.931471805599453e-1f64;
                let EY = EW * 6.931471805599453e-1f64;
                let EZ = parameters[868] - EB;
                let FA = 3.20438e-19f64 * H;
                let FB = 3.20438e-19f64 * H;
                let FC = M * K;
                let FD = 1e-8f64 / FC;
                let FF = FE * parameters[433];
                let FG = if (if parameters[1130] == Z { 1.0 } else { 0.0 }) != 0.0 && (if parameters[1131] == Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if FG != 0.0 {
                } else {
                    let FH = parameters[1132] - EB;
                    oFH = FH;
                }
                if CH != 0.0 {
                } else {
                    let FI = if CG == DP { 1.0 } else { 0.0 };
                    oFI = FI;
                }
                let FJ = if CH != 0.0 && EA != 0.0 { 1.0 } else { 0.0 };
                if FJ != 0.0 {
                    let FL = FE * parameters[1103];
                    oFL = FL;
                    if ES != 0.0 {
                        let FM = if parameters[1127] == Z { 1.0 } else { 0.0 };
                        oFM = FM;
                        let FP = if FO != Z { 1.0 } else { 0.0 };
                        oFP = FP;
                        let FR = if (if FQ != Z { 1.0 } else { 0.0 }) != 0.0 && (if FO > Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oFR = FR;
                        if FR != 0.0 {
                            let FT = FE.powf(((DP * (-3e0f64 - (FO.ln()))) / FS));
                            oFT = FT;
                            let FU = FS - EB;
                            oFU = FU;
                        } else {
                        }
                        let FX = FV - FW;
                        oFX = FX;
                        let FY = FX - EB;
                        oFY = FY;
                        let FZ = B / FW;
                        oFZ = FZ;
                        let GA = FZ - EB;
                        oGA = GA;
                        let GB = FW - EB;
                        oGB = GB;
                    } else {
                    }
                    let FN = if parameters[1112] != Z { 1.0 } else { 0.0 };
                    oFN = FN;
                    if FN != 0.0 {
                        let GD = if GC != Z { 1.0 } else { 0.0 };
                        oGD = GD;
                        let GF = if (if FQ != Z { 1.0 } else { 0.0 }) != 0.0 && (if GC > Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oGF = GF;
                        if GF != 0.0 {
                            let GH = FE.powf(((DP * (-3e0f64 - (GC.ln()))) / GG));
                            oGH = GH;
                            let GI = GG - EB;
                            oGI = GI;
                        } else {
                        }
                        let GJ = FV - FW;
                        oGJ = GJ;
                        let GK = GJ - EB;
                        oGK = GK;
                        let GL = B / FW;
                        oGL = GL;
                        let GM = GL - EB;
                        oGM = GM;
                        let GN = FW - EB;
                        oGN = GN;
                    } else {
                    }
                    let GE = if ES != 0.0 && FN != 0.0 { 1.0 } else { 0.0 };
                    oGE = GE;
                    if GE != 0.0 {
                        let GQ = GO * GP;
                        oGQ = GQ;
                        let GR = GQ * GP;
                        oGR = GR;
                        let GS = DW * ((1e0f64 + GR).sqrt());
                        oGS = GS;
                        let GT = -2.5e3f64 * GP;
                        oGT = GT;
                        let GU = DW * ((1e0f64 + GR).sqrt());
                        oGU = GU;
                    } else {
                        if ES != 0.0 {
                            let GV = GO * GP;
                            oGV = GV;
                            let GW = GV * GP;
                            oGW = GW;
                            let GX = DW * ((1e0f64 + GW).sqrt());
                            oGX = GX;
                            let GY = -2.5e3f64 * GP;
                            oGY = GY;
                            let GZ = DW * ((1e0f64 + GW).sqrt());
                            oGZ = GZ;
                        } else {
                        }
                        if FN != 0.0 {
                            let HA = GO * GP;
                            oHA = HA;
                            let HB = HA * GP;
                            oHB = HB;
                            let HC = DW * ((1e0f64 + HB).sqrt());
                            oHC = HC;
                            let HD = -2.5e3f64 * GP;
                            oHD = HD;
                            let HE = DW * ((1e0f64 + HB).sqrt());
                            oHE = HE;
                        } else {
                        }
                    }
                } else {
                }
                let FK = if (if CH != 0.0 && ET != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EA != 0.0 { 1.0 } else { 0.0 };
                if FK != 0.0 {
                    let HF = (3.20438e-19f64 * H) * parameters[1117];
                    oHF = HF;
                    let HH = if HG > Z { 1.0 } else { 0.0 };
                    oHH = HH;
                    if HH != 0.0 {
                        let HJ = HG * HI;
                        oHJ = HJ;
                        let HK = (R * O) / I;
                        oHK = HK;
                    } else {
                        let HL = J / R;
                        oHL = HL;
                    }
                    let HN = if HM == B { 1.0 } else { 0.0 };
                    oHN = HN;
                    if HN != 0.0 {
                        let HO = -E;
                        oHO = HO;
                        if HH != 0.0 {
                            let HP = HG * HI;
                            oHP = HP;
                            let HQ = (R * O) / I;
                            oHQ = HQ;
                        } else {
                            let HR = J / R;
                            oHR = HR;
                        }
                    } else {
                    }
                } else {
                }
                let HS = if DZ == Z { 1.0 } else { 0.0 };
                let HT = if EA != 0.0 && (if FQ == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if HT != 0.0 {
                    let HV = FE * parameters[1106];
                    oHV = HV;
                    let HW = -2.5e3f64 * parameters[504];
                    oHW = HW;
                    let HX = if FO > Z { 1.0 } else { 0.0 };
                    oHX = HX;
                    if HX != 0.0 {
                        let HY = parameters[513] - EB;
                        oHY = HY;
                    } else {
                    }
                    let HZ = 3.20438e-19f64 / H;
                    oHZ = HZ;
                    let IA = parameters[511] - EB;
                    oIA = IA;
                    let IB = parameters[501] / 8e1f64;
                    oIB = IB;
                } else {
                }
                let HU = if CO != 0.0 || CN != 0.0 { 1.0 } else { 0.0 };
                if HU != 0.0 {
                    if CN != 0.0 {
                        let ID = -7.45669e11f64 * K;
                        oID = ID;
                        let IE = -9.82222e11f64 * K;
                        oIE = IE;
                    } else {
                    }
                    if CO != 0.0 {
                        let IF = if parameters[1041] == B { 1.0 } else { 0.0 };
                        oIF = IF;
                    } else {
                    }
                } else {
                }
                let IC = if parameters[45] != Z { 1.0 } else { 0.0 };
                let IG = if CV > Z { 1.0 } else { 0.0 };
                let II = -IH;
                let IK = IJ.powf(II);
                let IL = if IH == B { 1.0 } else { 0.0 };
                let IP = if IL != 0.0 {
                    IM
                } else {
                    let IO = (B / (B - IH)) * (B - (((IN * IH) * (B + IH)) * IK));
                    IO
                };
                let IR = -IQ;
                let IS = IJ.powf(IR);
                let IT = if IQ == B { 1.0 } else { 0.0 };
                let IW = if IT != 0.0 {
                    IU
                } else {
                    let IV = (B / (B - IQ)) * (B - (((IN * IQ) * (B + IQ)) * IS));
                    IV
                };
                let IY = -IX;
                let IZ = IJ.powf(IY);
                let JA = if IX == B { 1.0 } else { 0.0 };
                let JD = if JA != 0.0 {
                    JB
                } else {
                    let JC = (B / (B - IX)) * (B - (((IN * IX) * (B + IX)) * IZ));
                    JC
                };
                let JF = -JE;
                let JG = IJ.powf(JF);
                let JH = if JE == B { 1.0 } else { 0.0 };
                let JK = if JH != 0.0 {
                    JI
                } else {
                    let JJ = (B / (B - JE)) * (B - (((IN * JE) * (B + JE)) * JG));
                    JJ
                };
                let JM = -JL;
                let JN = IJ.powf(JM);
                let JO = if JL == B { 1.0 } else { 0.0 };
                let JR = if JO != 0.0 {
                    JP
                } else {
                    let JQ = (B / (B - JL)) * (B - (((IN * JL) * (B + JL)) * JN));
                    JQ
                };
                let JT = -JS;
                let JU = IJ.powf(JT);
                let JV = if JS == B { 1.0 } else { 0.0 };
                let JY = if JV != 0.0 {
                    JW
                } else {
                    let JX = (B / (B - JS)) * (B - (((IN * JS) * (B + JS)) * JU));
                    JX
                };
                let JZ = if IG != 0.0 && CT != 0.0 { 1.0 } else { 0.0 };
                let KA = if parameters[784] <= Z { 1.0 } else { 0.0 };
                let KD = KB * KC;
                let KF;
                let KG;
                if CP != 0.0 {
                    let KE = (3.20438e-19f64 * H) * parameters[1068];
                    oKE = KE;
                    let KJ = parameters[1067] * KC;
                    oKJ = KJ;
                    KF = KK;
                    KG = Z;
                } else {
                    let KL = if KB > Z { 1.0 } else { 0.0 };
                    let KN = if (if KL != 0.0 || (if parameters[799] > Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if KM > Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oKN = KN;
                    if KN != 0.0 {
                        let KO = if (if parameters[786] != Z { 1.0 } else { 0.0 }) != 0.0 && KL != 0.0 { 1.0 } else { 0.0 };
                        oKO = KO;
                        if KO != 0.0 {
                            let KR = (GO * KQ) * KQ;
                            oKR = KR;
                        } else {
                        }
                        let KS = 1e10f64 * L;
                        oKS = KS;
                        let KT = DW * KM;
                        oKT = KT;
                    } else {
                    }
                    KF = Z;
                    KG = KP;
                }
                let KI = if KH == Z { 1.0 } else { 0.0 };
                let KW;
                let KX;
                let KY;
                let KZ;
                if KI != 0.0 {
                    KW = KU;
                    KX = Z;
                    KY = Z;
                    KZ = Z;
                } else {
                    let KV = if KH == B { 1.0 } else { 0.0 };
                    oKV = KV;
                    let LA;
                    let LB;
                    let LC;
                    if KV != 0.0 {
                        LA = LD;
                        LB = LE;
                        LC = LF;
                    } else {
                        LA = Z;
                        LB = Z;
                        LC = Z;
                    }
                    KW = Z;
                    KX = LA;
                    KY = LB;
                    KZ = LC;
                }
                let LI;
                let LJ;
                if CO != 0.0 {
                    LI = LG;
                    LJ = LH;
                } else {
                    LI = Z;
                    LJ = Z;
                }
                let LL = if CN != 0.0 {
                    LK
                } else {
                    Z
                };
                let LM = if parameters[40] == B { 1.0 } else { 0.0 };
                if LM != 0.0 {
                    let LN = 3.20438e-19f64 * H;
                    oLN = LN;
                    let LO = 3.20438e-19f64 * H;
                    oLO = LO;
                    let LP = L * L;
                    oLP = LP;
                    let LQ = if (if parameters[1134] == Z { 1.0 } else { 0.0 }) != 0.0 && (if parameters[1135] == Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oLQ = LQ;
                } else {
                }
                let LR = -2.5e3f64 * parameters[694];
                let LS = 7e-1f64 * parameters[206];
                let LT = LS - EB;
                let LU = parameters[205] * HI;
                let LV = (R * O) / I;
                let LW = J / R;
                let LX = if (if parameter_given[666] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                if LX != 0.0 {
                    let LY = (((DP * I) * G) / 3.141592653589793e0f64) * ((if (parameters[670] * (B + (4e-7f64 / K))) >= CY { (parameters[670] * (B + (4e-7f64 / K))) } else { CY }).ln());
                    oLY = LY;
                } else {
                }
                let LZ = if parameters[41] == Z { 1.0 } else { 0.0 };
                if LZ != 0.0 {
                } else {
                    let MB = MA - EB;
                    oMB = MB;
                    let MC = B / MA;
                    oMC = MC;
                    let MD = MC - EB;
                    oMD = MD;
                    let MF = ME - EB;
                    oMF = MF;
                    let MG = B / ME;
                    oMG = MG;
                    let MH = MG - EB;
                    oMH = MH;
                }
                let MI = -E;
                let MJ = if EA != 0.0 && ET != 0.0 { 1.0 } else { 0.0 };
                if MJ != 0.0 {
                    let MK = if HM == B { 1.0 } else { 0.0 };
                    oMK = MK;
                } else {
                }
                let ML = if CG != DP { 1.0 } else { 0.0 };
                let MN = if DH != 0.0 {
                    Z
                } else {
                    MM
                };
            [C, H, L, M, N, S, T, U, V, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, oAN, oAQ, oAR, AO, AP, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG, BH, BI, BJ, BK, BL, BM, BN, BO, BP, BQ, BR, BS, BT, BU, BV, BW, BX, BY, BZ, CA, oCE, CH, oCK, oCL, oCM, CN, CO, CP, CR, CS, CT, CX, CZ, DC, DG, DH, DJ, DM, DN, DQ, E, DR, DS, DT, DY, EA, oED, EF, EG, EH, EI, EJ, EK, EL, EM, EN, EO, EP, EQ, ES, EU, oEV, EW, EX, EY, FA, FB, FC, FD, FF, FG, oFI, FJ, oFL, oFM, oFP, oFR, oFT, oFX, oFZ, oFN, oGD, oGF, oGH, oGJ, oGL, oGE, oGQ, oGR, oGS, oGT, oGU, oGV, oGW, oGX, oGY, oGZ, oHA, oHB, oHC, oHD, oHE, FK, oHF, R, oHH, oHJ, oHK, oHL, oHN, oHO, oHP, oHQ, oHR, HS, HT, oHV, oHW, oHX, oHZ, oIB, HU, oID, oIE, oIF, IC, CU, IG, II, IK, IL, IR, IS, IT, IY, IZ, JA, IP, IW, JD, JF, JG, JH, JM, JN, JO, JT, JU, JV, JK, JR, JY, JZ, KA, KD, oKE, oKJ, oKN, oKO, oKR, oKS, oKT, KI, oKV, LM, oLN, oLO, oLP, oLQ, LR, LS, LU, LV, LW, LX, oLY, LZ, oMC, oMG, MI, MJ, oMK, ML, CF, KF, KG, KW, KX, KY, KZ, LI, LJ, LL, MN, oEC, oEE, EZ, oFH, oFU, oFY, oGA, oGB, oGI, oGK, oGM, oGN, oHY, oIA, LT, oMB, oMD, oMF, oMH]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 710] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[52];
                let C = parameters[1];
                let D = parameters[53];
                let G = 0e0f64;
                let I = parameters[2];
                let L = staged[0];
                let N = staged[1];
                let Q = parameters[58];
                let R = parameters[57];
                let S = parameters[59];
                let T = parameters[60];
                let U = staged[2];
                let W = staged[3];
                let Z = parameters[64];
                let AA = parameters[63];
                let AB = parameters[65];
                let AC = parameters[66];
                let AE = 2e0f64;
                let AH = 1e-9f64;
                let AM = parameters[74];
                let AN = parameters[75];
                let AO = parameters[76];
                let BA = 1e-6f64;
                let BG = staged[461];
                let BH = parameters[818];
                let BL = staged[463];
                let BR = parameters[819];
                let CF = staged[467];
                let CI = 1e0f64;
                let IB = staged[468];
                let JM = staged[469];
                let JN = staged[470];
                let JX = staged[12];
                let LB = 5e-1f64;
                let LN = 2.5e-1f64;
                let MV = staged[472];
                let NW = staged[486];
                let NY = staged[488];
                let OK = 6.7e-2f64;
                let OW = staged[502];
                let OX = parameters[1066];
                let OZ = 0e0f64;
                let PA = 0e0f64;
                let PD = if parameter_given[3] { 1.0 } else { 0.0 };
                let PH = parameters[801];
                let PK = parameters[374];
                let PM = parameters[10];
                let PV = if parameter_given[4] { 1.0 } else { 0.0 };
                let PW = parameters[9];
                let PX = 9e0f64;
                let QO = parameters[6];
                let QU = 1.0f64;
                let RE = staged[53];
                let RJ = 1.0f64;
                let RO = 1.0f64;
                let RP = 0.0f64;
                let RR = 5e0f64;
                let RT = 3e0f64;
                let RU = 7e0f64;
                let RY = 4e0f64;
                let RZ = 6e0f64;
                let SE = parameters[696];
                let SM = 8e0f64;
                let TW = 1.0f64;
                let UA = 1.0f64;
                let UB = 0.0f64;
                let VZ = 1.0f64;
                let WD = 1.0f64;
                let WE = 0.0f64;
                let YC = 1.0f64;
                let YG = 1.0f64;
                let YH = 0.0f64;
                let AAD = 1.0f64;
                let AAH = 1.0f64;
                let AAI = staged[54];
                let ABK = 1.0f64;
                let ABO = 1.0f64;
                let ACQ = 1.0f64;
                let ACV = 0.0f64;
                let ADW = 1.0f64;
                let AEB = 0.0f64;
                let AFG = 1.0f64;
                let AFH = 1e1f64;
                let AFS = 1.0f64;
                let AHI = 0.0f64;
                let AHW = 0.0f64;
                let AIB = 1.0f64;
                let AIC = 0.0f64;
                let AKC = 0.0f64;
                let AKG = 1.0f64;
                let AKH = 0.0f64;
                let AMF = 0.0f64;
                let AMJ = 1.0f64;
                let AMK = 0.0f64;
                let AOI = 0.0f64;
                let AOM = 1.0f64;
                let AON = 0.0f64;
                let AQJ = 0.0f64;
                let AQN = 1.0f64;
                let ARP = 0.0f64;
                let ART = 1.0f64;
                let ASV = 0.0f64;
                let ATA = 0.0f64;
                let AUB = 0.0f64;
                let AUG = 0.0f64;
                let AVL = 0.0f64;
                let AVW = 0.0f64;
                let AWL = parameters[1093];
                let AXE = parameters[8];
                let AXQ = 1e6f64;
                let AXR = 1e-38f64;
                let AYB = staged[297];
                let AYH = parameters[14];
                let AYI = parameters[11];
                let AYJ = parameters[13];
                let AYK = parameters[15];
                let AYL = parameters[12];
                let AZK = 1e-3f64;
                let AZS = 1e3f64;
                let AZT = parameters[756];
                let BBF = parameters[16];
                let BBI = parameters[32];
                let BBO = parameters[7];
                let BBU = staged[57];
                let BBZ = staged[59];
                let BCI = staged[66];
                let BCL = 1.60219e-19f64;
                let BCM = staged[70];
                let BCP = 1e0f64;
                let BCW = staged[829];
                let BDZ = staged[134];
                let BEJ = if parameter_given[24] { 1.0 } else { 0.0 };
                let BFQ = staged[135];
                let BII = if parameter_given[25] { 1.0 } else { 0.0 };
                let BIN = if parameter_given[26] { 1.0 } else { 0.0 };
                let BIO = parameters[137];
                let BIS = if parameter_given[27] { 1.0 } else { 0.0 };
                let BIT = parameters[26];
                let BJC = parameters[27];
                let BJH = parameters[17];
                let BJI = parameters[18];
                let BJJ = parameters[19];
                let BJX = staged[851];
                let BKR = parameters[37];
                let BLM = parameters[23];
                let BLO = parameters[20];
                let BLP = parameters[21];
                let BLQ = parameters[22];
                let BLV = parameters[947];
                let BLY = 1e-1f64;
                let BLZ = 1e-2f64;
                let BMC = 5e-2f64;
                let BMM = parameters[35];
                let BMZ = staged[863];
                let BNB = staged[409];
                let BNF = staged[866];
                let BNH = staged[867];
                let BNI = staged[869];
                let BNS = staged[881];
                let BNX = staged[883];
                let BNZ = staged[884];
                let BOB = parameters[28];
                let BOD = staged[886];
                let BOI = staged[887];
                let BOV = staged[898];
                let BOX = parameters[1128];
                let BPK = staged[902];
                let BPP = 1e10f64;
                let BPR = parameters[800];
                let BPX = staged[904];
                let BQD = staged[906];
                let BQF = staged[907];
                let BQP = staged[910];
                let BQQ = staged[911];
                let BQU = staged[912];
                let BRL = parameters[1062];
                let BRS = parameters[957];
                let BRV = 0e0f64;
                let BRW = staged[408];
                let BRY = parameters[1110];
                let BSA = 0e0f64;
                let BSB = 0e0f64;
                let BSJ = 0e0f64;
                let BSK = 0e0f64;
                let BSL = 0e0f64;
                let BSP = 0e0f64;
                let BSQ = parameters[1112];
                let BSS = 0e0f64;
                let BST = 0e0f64;
                let BTB = 0e0f64;
                let BTC = 0e0f64;
                let BTD = 0e0f64;
                let BTH = 0e0f64;
                let BTI = 0e0f64;
                let BTN = 0e0f64;
                let BTO = 0e0f64;
                let BTS = 0e0f64;
                let BTT = 0e0f64;
                let BTU = 0e0f64;
                let BTV = 0e0f64;
                let BTW = 0e0f64;
                let BTX = 0e0f64;
                let BTY = 0e0f64;
                let BTZ = 0e0f64;
                let BUK = 0e0f64;
                let BUL = 0e0f64;
                let mut oAI = 0.0;
                let mut oAL = 0.0;
                let mut oAS = 0.0;
                let mut oAV = 0.0;
                let mut oBI = 0.0;
                let mut oBS = 0.0;
                let mut oND = 0.0;
                let mut oNG = 0.0;
                let mut oNJ = 0.0;
                let mut oNX = 0.0;
                let mut oNZ = 0.0;
                let mut oOA = 0.0;
                let mut oOC = 0.0;
                let mut oOY = 0.0;
                let mut oPI = 0.0;
                let mut oPN = 0.0;
                let mut oPY = 0.0;
                let mut oQG = 0.0;
                let mut oQM = 0.0;
                let mut oQP = 0.0;
                let mut oRB = 0.0;
                let mut oRC = 0.0;
                let mut oRK = 0.0;
                let mut oRN = 0.0;
                let mut oRS = 0.0;
                let mut oRV = 0.0;
                let mut oRX = 0.0;
                let mut oSA = 0.0;
                let mut oSG = 0.0;
                let mut oSI = 0.0;
                let mut oSL = 0.0;
                let mut oSN = 0.0;
                let mut oSS = 0.0;
                let mut oSU = 0.0;
                let mut oSX = 0.0;
                let mut oSY = 0.0;
                let mut oTA = 0.0;
                let mut oTB = 0.0;
                let mut oTG = 0.0;
                let mut oTI = 0.0;
                let mut oTL = 0.0;
                let mut oTM = 0.0;
                let mut oTR = 0.0;
                let mut oTT = 0.0;
                let mut oTX = 0.0;
                let mut oUD = 0.0;
                let mut oUE = 0.0;
                let mut oUG = 0.0;
                let mut oUH = 0.0;
                let mut oUM = 0.0;
                let mut oUO = 0.0;
                let mut oUR = 0.0;
                let mut oUS = 0.0;
                let mut oUX = 0.0;
                let mut oUZ = 0.0;
                let mut oVC = 0.0;
                let mut oVD = 0.0;
                let mut oVF = 0.0;
                let mut oVG = 0.0;
                let mut oVK = 0.0;
                let mut oVM = 0.0;
                let mut oVP = 0.0;
                let mut oVQ = 0.0;
                let mut oVU = 0.0;
                let mut oVW = 0.0;
                let mut oWA = 0.0;
                let mut oWG = 0.0;
                let mut oWH = 0.0;
                let mut oWJ = 0.0;
                let mut oWK = 0.0;
                let mut oWO = 0.0;
                let mut oWQ = 0.0;
                let mut oWT = 0.0;
                let mut oWU = 0.0;
                let mut oWY = 0.0;
                let mut oXA = 0.0;
                let mut oXD = 0.0;
                let mut oXE = 0.0;
                let mut oXG = 0.0;
                let mut oXH = 0.0;
                let mut oXM = 0.0;
                let mut oXO = 0.0;
                let mut oXR = 0.0;
                let mut oXS = 0.0;
                let mut oXX = 0.0;
                let mut oXZ = 0.0;
                let mut oYD = 0.0;
                let mut oYJ = 0.0;
                let mut oYK = 0.0;
                let mut oYM = 0.0;
                let mut oYN = 0.0;
                let mut oYR = 0.0;
                let mut oYT = 0.0;
                let mut oYW = 0.0;
                let mut oYX = 0.0;
                let mut oZB = 0.0;
                let mut oZD = 0.0;
                let mut oZG = 0.0;
                let mut oZH = 0.0;
                let mut oZJ = 0.0;
                let mut oZK = 0.0;
                let mut oZO = 0.0;
                let mut oZQ = 0.0;
                let mut oZT = 0.0;
                let mut oZU = 0.0;
                let mut oZY = 0.0;
                let mut oAAA = 0.0;
                let mut oAAE = 0.0;
                let mut oAAL = 0.0;
                let mut oAAM = 0.0;
                let mut oAAO = 0.0;
                let mut oAAP = 0.0;
                let mut oAAU = 0.0;
                let mut oAAW = 0.0;
                let mut oAAZ = 0.0;
                let mut oABA = 0.0;
                let mut oABF = 0.0;
                let mut oABH = 0.0;
                let mut oABL = 0.0;
                let mut oABP = 0.0;
                let mut oABR = 0.0;
                let mut oABS = 0.0;
                let mut oABU = 0.0;
                let mut oABV = 0.0;
                let mut oABZ = 0.0;
                let mut oACB = 0.0;
                let mut oACE = 0.0;
                let mut oACF = 0.0;
                let mut oACJ = 0.0;
                let mut oACL = 0.0;
                let mut oACR = 0.0;
                let mut oACX = 0.0;
                let mut oACY = 0.0;
                let mut oADA = 0.0;
                let mut oADB = 0.0;
                let mut oADG = 0.0;
                let mut oADI = 0.0;
                let mut oADL = 0.0;
                let mut oADM = 0.0;
                let mut oADR = 0.0;
                let mut oADT = 0.0;
                let mut oADX = 0.0;
                let mut oAEA = 0.0;
                let mut oAEF = 0.0;
                let mut oAEG = 0.0;
                let mut oAEI = 0.0;
                let mut oAEJ = 0.0;
                let mut oAEN = 0.0;
                let mut oAEP = 0.0;
                let mut oAES = 0.0;
                let mut oAET = 0.0;
                let mut oAEX = 0.0;
                let mut oAEZ = 0.0;
                let mut oAFD = 0.0;
                let mut oAFI = 0.0;
                let mut oAFM = 0.0;
                let mut oAFX = 0.0;
                let mut oAGC = 0.0;
                let mut oAGE = 0.0;
                let mut oAGI = 0.0;
                let mut oAGP = 0.0;
                let mut oAGV = 0.0;
                let mut oAHB = 0.0;
                let mut oAHD = 0.0;
                let mut oAHP = 0.0;
                let mut oAHQ = 0.0;
                let mut oAHX = 0.0;
                let mut oAIA = 0.0;
                let mut oAIE = 0.0;
                let mut oAIF = 0.0;
                let mut oAIH = 0.0;
                let mut oAII = 0.0;
                let mut oAIN = 0.0;
                let mut oAIP = 0.0;
                let mut oAIS = 0.0;
                let mut oAIT = 0.0;
                let mut oAIY = 0.0;
                let mut oAJA = 0.0;
                let mut oAJD = 0.0;
                let mut oAJE = 0.0;
                let mut oAJG = 0.0;
                let mut oAJH = 0.0;
                let mut oAJM = 0.0;
                let mut oAJO = 0.0;
                let mut oAJR = 0.0;
                let mut oAJS = 0.0;
                let mut oAJX = 0.0;
                let mut oAJZ = 0.0;
                let mut oAKD = 0.0;
                let mut oAKJ = 0.0;
                let mut oAKK = 0.0;
                let mut oAKM = 0.0;
                let mut oAKN = 0.0;
                let mut oAKS = 0.0;
                let mut oAKU = 0.0;
                let mut oAKX = 0.0;
                let mut oAKY = 0.0;
                let mut oALD = 0.0;
                let mut oALF = 0.0;
                let mut oALI = 0.0;
                let mut oALJ = 0.0;
                let mut oALL = 0.0;
                let mut oALM = 0.0;
                let mut oALQ = 0.0;
                let mut oALS = 0.0;
                let mut oALV = 0.0;
                let mut oALW = 0.0;
                let mut oAMA = 0.0;
                let mut oAMC = 0.0;
                let mut oAMG = 0.0;
                let mut oAMM = 0.0;
                let mut oAMN = 0.0;
                let mut oAMP = 0.0;
                let mut oAMQ = 0.0;
                let mut oAMU = 0.0;
                let mut oAMW = 0.0;
                let mut oAMZ = 0.0;
                let mut oANA = 0.0;
                let mut oANE = 0.0;
                let mut oANG = 0.0;
                let mut oANJ = 0.0;
                let mut oANK = 0.0;
                let mut oANM = 0.0;
                let mut oANN = 0.0;
                let mut oANS = 0.0;
                let mut oANU = 0.0;
                let mut oANX = 0.0;
                let mut oANY = 0.0;
                let mut oAOD = 0.0;
                let mut oAOF = 0.0;
                let mut oAOJ = 0.0;
                let mut oAOP = 0.0;
                let mut oAOQ = 0.0;
                let mut oAOS = 0.0;
                let mut oAOT = 0.0;
                let mut oAOX = 0.0;
                let mut oAOZ = 0.0;
                let mut oAPC = 0.0;
                let mut oAPD = 0.0;
                let mut oAPH = 0.0;
                let mut oAPJ = 0.0;
                let mut oAPM = 0.0;
                let mut oAPN = 0.0;
                let mut oAPP = 0.0;
                let mut oAPQ = 0.0;
                let mut oAPU = 0.0;
                let mut oAPW = 0.0;
                let mut oAPZ = 0.0;
                let mut oAQA = 0.0;
                let mut oAQE = 0.0;
                let mut oAQG = 0.0;
                let mut oAQK = 0.0;
                let mut oAQQ = 0.0;
                let mut oAQR = 0.0;
                let mut oAQT = 0.0;
                let mut oAQU = 0.0;
                let mut oAQZ = 0.0;
                let mut oARB = 0.0;
                let mut oARE = 0.0;
                let mut oARF = 0.0;
                let mut oARK = 0.0;
                let mut oARM = 0.0;
                let mut oARQ = 0.0;
                let mut oARU = 0.0;
                let mut oARW = 0.0;
                let mut oARX = 0.0;
                let mut oARZ = 0.0;
                let mut oASA = 0.0;
                let mut oASE = 0.0;
                let mut oASG = 0.0;
                let mut oASJ = 0.0;
                let mut oASK = 0.0;
                let mut oASO = 0.0;
                let mut oASQ = 0.0;
                let mut oASW = 0.0;
                let mut oATC = 0.0;
                let mut oATD = 0.0;
                let mut oATF = 0.0;
                let mut oATG = 0.0;
                let mut oATL = 0.0;
                let mut oATN = 0.0;
                let mut oATQ = 0.0;
                let mut oATR = 0.0;
                let mut oATW = 0.0;
                let mut oATY = 0.0;
                let mut oAUC = 0.0;
                let mut oAUF = 0.0;
                let mut oAUK = 0.0;
                let mut oAUL = 0.0;
                let mut oAUN = 0.0;
                let mut oAUO = 0.0;
                let mut oAUS = 0.0;
                let mut oAUU = 0.0;
                let mut oAUX = 0.0;
                let mut oAUY = 0.0;
                let mut oAVC = 0.0;
                let mut oAVE = 0.0;
                let mut oAVI = 0.0;
                let mut oAVM = 0.0;
                let mut oAVQ = 0.0;
                let mut oAWB = 0.0;
                let mut oAWG = 0.0;
                let mut oAWI = 0.0;
                let mut oAWM = 0.0;
                let mut oAWN = 0.0;
                let mut oAWR = 0.0;
                let mut oAWU = 0.0;
                let mut oAWW = 0.0;
                let mut oAWX = 0.0;
                let mut oAXH = 0.0;
                let mut oAXJ = 0.0;
                let mut oAXL = 0.0;
                let mut oAXO = 0.0;
                let mut oAXV = 0.0;
                let mut oAYC = 0.0;
                let mut oAYE = 0.0;
                let mut oAYG = 0.0;
                let mut oAYR = 0.0;
                let mut oAZC = 0.0;
                let mut oAZL = 0.0;
                let mut oAZM = 0.0;
                let mut oAZW = 0.0;
                let mut oAZZ = 0.0;
                let mut oBAC = 0.0;
                let mut oBAF = 0.0;
                let mut oBAI = 0.0;
                let mut oBAJ = 0.0;
                let mut oBAR = 0.0;
                let mut oBAU = 0.0;
                let mut oBAX = 0.0;
                let mut oBBG = 0.0;
                let mut oBBP = 0.0;
                let mut oBCF = 0.0;
                let mut oBCG = 0.0;
                let mut oBCJ = 0.0;
                let mut oBCK = 0.0;
                let mut oBDD = 0.0;
                let mut oBDO = 0.0;
                let mut oBEE = 0.0;
                let mut oBEP = 0.0;
                let mut oBEZ = 0.0;
                let mut oBFK = 0.0;
                let mut oBFU = 0.0;
                let mut oBGE = 0.0;
                let mut oBGN = 0.0;
                let mut oBGX = 0.0;
                let mut oBHG = 0.0;
                let mut oBHQ = 0.0;
                let mut oBIP = 0.0;
                let mut oBIQ = 0.0;
                let mut oBIY = 0.0;
                let mut oBIZ = 0.0;
                let mut oBJO = 0.0;
                let mut oBKB = 0.0;
                let mut oBKJ = 0.0;
                let mut oBKK = 0.0;
                let mut oBKP = 0.0;
                let mut oBKS = 0.0;
                let mut oBLC = 0.0;
                let mut oBLN = 0.0;
                let mut oBMK = 0.0;
                let mut oBMT = 0.0;
                let mut oBMV = 0.0;
                let mut oBMX = 0.0;
                let mut oBMY = 0.0;
                let mut oBNC = 0.0;
                let mut oBND = 0.0;
                let mut oBNE = 0.0;
                let mut oBNG = 0.0;
                let mut oBNJ = 0.0;
                let mut oBNK = 0.0;
                let mut oBNN = 0.0;
                let mut oBNO = 0.0;
                let mut oBNP = 0.0;
                let mut oBNQ = 0.0;
                let mut oBNT = 0.0;
                let mut oBNW = 0.0;
                let mut oBNY = 0.0;
                let mut oBOA = 0.0;
                let mut oBOF = 0.0;
                let mut oBOG = 0.0;
                let mut oBOH = 0.0;
                let mut oBOJ = 0.0;
                let mut oBOK = 0.0;
                let mut oBOM = 0.0;
                let mut oBON = 0.0;
                let mut oBOP = 0.0;
                let mut oBOR = 0.0;
                let mut oBOT = 0.0;
                let mut oBOU = 0.0;
                let mut oBOY = 0.0;
                let mut oBOZ = 0.0;
                let mut oBPA = 0.0;
                let mut oBPB = 0.0;
                let mut oBPC = 0.0;
                let mut oBPD = 0.0;
                let mut oBPE = 0.0;
                let mut oBPL = 0.0;
                let mut oBPM = 0.0;
                let mut oBPQ = 0.0;
                let mut oBPS = 0.0;
                let mut oBPT = 0.0;
                let mut oBPU = 0.0;
                let mut oBPV = 0.0;
                let mut oBQA = 0.0;
                let mut oBQB = 0.0;
                let mut oBQC = 0.0;
                let mut oBQE = 0.0;
                let mut oBQG = 0.0;
                let mut oBQH = 0.0;
                let mut oBQI = 0.0;
                let mut oBQL = 0.0;
                let mut oBQM = 0.0;
                let mut oBQW = 0.0;
                let mut oBQX = 0.0;
                let mut oBQY = 0.0;
                let mut oBQZ = 0.0;
                let mut oBRA = 0.0;
                let mut oBRD = 0.0;
                let mut oBRG = 0.0;
                let mut oBRH = 0.0;
                let mut oBRI = 0.0;
                let mut oBRJ = 0.0;
                let mut oBRK = 0.0;
                let mut oBRM = 0.0;
                let mut oBRN = 0.0;
                let mut oBRO = 0.0;
                let mut oBRQ = 0.0;
                let mut oBRR = 0.0;
                let mut oBRT = 0.0;
                let mut oBRU = 0.0;
                let mut oBRZ = 0.0;
                let mut oBSR = 0.0;
                let mut oBTJ = 0.0;
                let mut oBTQ = 0.0;
                let mut oBTR = 0.0;
                let mut oBUI = 0.0;
                let mut oBUO = 0.0;
                let mut oBUP = 0.0;
                let B = parameters[0] * A;
                let E = C * D;
                let F = B + parameters[54];
                let H = if F <= G { 1.0 } else { 0.0 };
                let J = (E / I) + parameters[56];
                let K = if J <= G { 1.0 } else { 0.0 };
                let M = F.powf(L);
                let O = J.powf(N);
                let P = M * O;
                let V = F.powf(U);
                let X = J.powf(W);
                let Y = V * X;
                let AD = ((AA + (Z * V)) + (AB * X)) + (AC * Y);
                let AF = F - (AE * (((R + (Q * M)) + (S * O)) + (T * P)));
                let AG = if AF <= G { 1.0 } else { 0.0 };
                if AG != 0.0 {
                } else {
                    let AI = if AF <= AH { 1.0 } else { 0.0 };
                    oAI = AI;
                }
                let AJ = J - (AE * AD);
                let AK = if AJ <= G { 1.0 } else { 0.0 };
                if AK != 0.0 {
                } else {
                    let AL = if AJ <= AH { 1.0 } else { 0.0 };
                    oAL = AL;
                }
                let AP = ((parameters[73] + (AM * V)) + (AN * X)) + (AO * Y);
                let AQ = F - (AE * (((parameters[69] + (parameters[70] * M)) + (parameters[71] * O)) + (parameters[72] * P)));
                let AR = if AQ <= G { 1.0 } else { 0.0 };
                if AR != 0.0 {
                } else {
                    let AS = if AQ <= AH { 1.0 } else { 0.0 };
                    oAS = AS;
                }
                let AT = J - (AE * AP);
                let AU = if AT <= G { 1.0 } else { 0.0 };
                if AU != 0.0 {
                } else {
                    let AV = if AT <= AH { 1.0 } else { 0.0 };
                    oAV = AV;
                }
                let AW = F.powf(parameters[67]);
                let AX = J.powf(parameters[68]);
                let AY = J - (AE * (((parameters[138] + (AM / AW)) + (AN / AX)) + ((AO / AW) / AX)));
                let AZ = if AY <= G { 1.0 } else { 0.0 };
                let BB = BA / AF;
                let BC = BA / AJ;
                let BD = BA / AQ;
                let BE = BA / AT;
                let BF = BB * BC;
                let BJ;
                let BK;
                if BG != 0.0 {
                    let BI = if BH <= (-F) { 1.0 } else { 0.0 };
                    oBI = BI;
                    let BP;
                    let BQ;
                    if BI != 0.0 {
                        BP = M;
                        BQ = V;
                    } else {
                        let BM = F + BH;
                        let BN = BM.powf(L);
                        let BO = BM.powf(U);
                        BP = BN;
                        BQ = BO;
                    }
                    BJ = BP;
                    BK = BQ;
                } else {
                    BJ = M;
                    BK = V;
                }
                let BT;
                let BU;
                if BL != 0.0 {
                    let BS = if BR <= (-J) { 1.0 } else { 0.0 };
                    oBS = BS;
                    let CB;
                    let CC;
                    if BS != 0.0 {
                        CB = O;
                        CC = X;
                    } else {
                        let BY = J + BR;
                        let BZ = BY.powf(N);
                        let CA = BY.powf(W);
                        CB = BZ;
                        CC = CA;
                    }
                    BT = CB;
                    BU = CC;
                } else {
                    BT = O;
                    BU = X;
                }
                let BV = ((AA + (Z * BK)) + (AB * BU)) + (AC * (BK * BU));
                let BW = (F - (AE * (((R + (Q * BJ)) + (S * BT)) + (T * (BJ * BT))))) + BH;
                let BX = if BW <= G { 1.0 } else { 0.0 };
                let CD = (J - (AE * BV)) + BR;
                let CE = if CD <= G { 1.0 } else { 0.0 };
                let CL;
                let CM;
                if CF != 0.0 {
                    let CG = BA / BW;
                    let CH = BA / CD;
                    CL = CG;
                    CM = CH;
                } else {
                    let CJ = CI / BW;
                    let CK = CI / CD;
                    CL = CJ;
                    CM = CK;
                }
                let CN = CL * CM;
                let CO = ((parameters[116] + (CL * parameters[117])) + (CM * parameters[118])) + (CN * parameters[119]);
                let CP = ((parameters[126] + (CL * parameters[127])) + (CM * parameters[128])) + (CN * parameters[129]);
                let CQ = ((parameters[139] + (CL * parameters[140])) + (CM * parameters[141])) + (CN * parameters[142]);
                let CR = ((parameters[80] + (CL * parameters[89])) + (CM * parameters[90])) + (CN * parameters[91]);
                let CS = ((parameters[92] + (CL * parameters[101])) + (CM * parameters[102])) + (CN * parameters[103]);
                let CT = ((parameters[104] + (CL * parameters[105])) + (CM * parameters[106])) + (CN * parameters[107]);
                let CU = ((parameters[209] + (CL * parameters[210])) + (CM * parameters[211])) + (CN * parameters[212]);
                let CV = ((parameters[213] + (CL * parameters[220])) + (CM * parameters[221])) + (CN * parameters[222]);
                let CW = ((parameters[223] + (CL * parameters[226])) + (CM * parameters[227])) + (CN * parameters[228]);
                let CX = ((parameters[233] + (CL * parameters[236])) + (CM * parameters[237])) + (CN * parameters[238]);
                let CY = ((parameters[143] + (CL * parameters[144])) + (CM * parameters[145])) + (CN * parameters[146]);
                let CZ = ((parameters[147] + (CL * parameters[148])) + (CM * parameters[149])) + (CN * parameters[150]);
                let DA = ((parameters[151] + (CL * parameters[152])) + (CM * parameters[153])) + (CN * parameters[154]);
                let DB = ((parameters[155] + (CL * parameters[156])) + (CM * parameters[157])) + (CN * parameters[158]);
                let DC = ((parameters[159] + (CL * parameters[160])) + (CM * parameters[161])) + (CN * parameters[162]);
                let DD = ((parameters[163] + (CL * parameters[164])) + (CM * parameters[165])) + (CN * parameters[166]);
                let DE = ((parameters[195] + (CL * parameters[202])) + (CM * parameters[203])) + (CN * parameters[204]);
                let DF = ((parameters[185] + (CL * parameters[192])) + (CM * parameters[193])) + (CN * parameters[194]);
                let DG = ((parameters[112] + (CL * parameters[113])) + (CM * parameters[114])) + (CN * parameters[115]);
                let DH = ((parameters[167] + (CL * parameters[168])) + (CM * parameters[169])) + (CN * parameters[170]);
                let DI = ((parameters[171] + (CL * parameters[172])) + (CM * parameters[173])) + (CN * parameters[174]);
                let DJ = ((parameters[180] + (CL * parameters[182])) + (CM * parameters[183])) + (CN * parameters[184]);
                let DK = ((parameters[253] + (CL * parameters[254])) + (CM * parameters[255])) + (CN * parameters[256]);
                let DL = ((parameters[273] + (CL * parameters[276])) + (CM * parameters[277])) + (CN * parameters[278]);
                let DM = ((parameters[284] + (CL * parameters[291])) + (CM * parameters[292])) + (CN * parameters[293]);
                let DN = ((parameters[308] + (CL * parameters[311])) + (CM * parameters[312])) + (CN * parameters[313]);
                let DO = ((parameters[298] + (CL * parameters[299])) + (CM * parameters[300])) + (CN * parameters[301]);
                let DP = ((parameters[318] + (CL * parameters[319])) + (CM * parameters[320])) + (CN * parameters[321]);
                let DQ = ((parameters[326] + (CL * parameters[333])) + (CM * parameters[334])) + (CN * parameters[335]);
                let DR = ((parameters[340] + (CL * parameters[343])) + (CM * parameters[344])) + (CN * parameters[345]);
                let DS = ((parameters[351] + (CL * parameters[354])) + (CM * parameters[355])) + (CN * parameters[356]);
                let DT = ((parameters[393] + (CL * parameters[394])) + (CM * parameters[395])) + (CN * parameters[396]);
                let DU = ((parameters[403] + (CL * parameters[404])) + (CM * parameters[405])) + (CN * parameters[406]);
                let DV = ((parameters[375] + (CL * parameters[376])) + (CM * parameters[377])) + (CN * parameters[378]);
                let DW = ((parameters[379] + (CL * parameters[380])) + (CM * parameters[381])) + (CN * parameters[382]);
                let DX = ((parameters[385] + (CL * parameters[386])) + (CM * parameters[387])) + (CN * parameters[388]);
                let DY = ((parameters[389] + (CL * parameters[390])) + (CM * parameters[391])) + (CN * parameters[392]);
                let DZ = ((parameters[399] + (CL * parameters[400])) + (CM * parameters[401])) + (CN * parameters[402]);
                let EA = ((parameters[413] + (CL * parameters[416])) + (CM * parameters[417])) + (CN * parameters[418]);
                let EB = ((parameters[409] + (CL * parameters[410])) + (CM * parameters[411])) + (CN * parameters[412]);
                let EC = ((parameters[434] + (CL * parameters[435])) + (CM * parameters[436])) + (CN * parameters[437]);
                let ED = ((parameters[460] + (CL * parameters[463])) + (CM * parameters[464])) + (CN * parameters[465]);
                let EE = ((parameters[470] + (CL * parameters[471])) + (CM * parameters[472])) + (CN * parameters[473]);
                let EF = ((parameters[357] + (CL * parameters[358])) + (CM * parameters[359])) + (CN * parameters[360]);
                let EG = ((parameters[361] + (CL * parameters[362])) + (CM * parameters[363])) + (CN * parameters[364]);
                let EH = ((parameters[365] + (CL * parameters[366])) + (CM * parameters[367])) + (CN * parameters[368]);
                let EI = ((parameters[370] + (CL * parameters[371])) + (CM * parameters[372])) + (CN * parameters[373]);
                let EJ = ((parameters[478] + (CL * parameters[481])) + (CM * parameters[482])) + (CN * parameters[483]);
                let EK = ((parameters[474] + (CL * parameters[475])) + (CM * parameters[476])) + (CN * parameters[477]);
                let EL = ((parameters[239] + (CL * parameters[240])) + (CM * parameters[241])) + (CN * parameters[242]);
                let EM = ((parameters[419] + (CL * parameters[420])) + (CM * parameters[421])) + (CN * parameters[422]);
                let EN = ((parameters[259] + (CL * parameters[260])) + (CM * parameters[261])) + (CN * parameters[262]);
                let EO = ((parameters[666] + (CL * parameters[667])) + (CM * parameters[668])) + (CN * parameters[669]);
                let EP = ((parameters[674] + (CL * parameters[675])) + (CM * parameters[676])) + (CN * parameters[677]);
                let EQ = ((parameters[678] + (CL * parameters[679])) + (CM * parameters[680])) + (CN * parameters[681]);
                let ER = ((parameters[682] + (CL * parameters[683])) + (CM * parameters[684])) + (CN * parameters[685]);
                let ES = ((parameters[686] + (CL * parameters[687])) + (CM * parameters[688])) + (CN * parameters[689]);
                let ET = ((parameters[484] + (CL * parameters[489])) + (CM * parameters[490])) + (CN * parameters[491]);
                let EU = ((parameters[494] + (CL * parameters[497])) + (CM * parameters[498])) + (CN * parameters[499]);
                let EV = ((parameters[935] + (CL * parameters[936])) + (CM * parameters[937])) + (CN * parameters[938]);
                let EW = ((parameters[939] + (CL * parameters[940])) + (CM * parameters[941])) + (CN * parameters[942]);
                let EX = ((parameters[943] + (CL * parameters[944])) + (CM * parameters[945])) + (CN * parameters[946]);
                let EY = ((parameters[630] + (CL * parameters[633])) + (CM * parameters[634])) + (CN * parameters[635]);
                let EZ = ((parameters[636] + (CL * parameters[637])) + (CM * parameters[638])) + (CN * parameters[639]);
                let FA = ((parameters[640] + (CL * parameters[641])) + (CM * parameters[642])) + (CN * parameters[643]);
                let FB = ((parameters[644] + (CL * parameters[645])) + (CM * parameters[646])) + (CN * parameters[647]);
                let FC = ((parameters[648] + (CL * parameters[651])) + (CM * parameters[652])) + (CN * parameters[653]);
                let FD = ((parameters[654] + (CL * parameters[655])) + (CM * parameters[656])) + (CN * parameters[657]);
                let FE = ((parameters[658] + (CL * parameters[659])) + (CM * parameters[660])) + (CN * parameters[661]);
                let FF = ((parameters[662] + (CL * parameters[663])) + (CM * parameters[664])) + (CN * parameters[665]);
                let FG = ((parameters[824] + (CL * parameters[825])) + (CM * parameters[826])) + (CN * parameters[827]);
                let FH = ((parameters[829] + (CL * parameters[830])) + (CM * parameters[831])) + (CN * parameters[832]);
                let FI = ((parameters[834] + (CL * parameters[835])) + (CM * parameters[836])) + (CN * parameters[837]);
                let FJ = ((parameters[838] + (CL * parameters[839])) + (CM * parameters[840])) + (CN * parameters[841]);
                let FK = ((parameters[843] + (CL * parameters[844])) + (CM * parameters[845])) + (CN * parameters[846]);
                let FL = ((parameters[847] + (CL * parameters[848])) + (CM * parameters[849])) + (CN * parameters[850]);
                let FM = ((parameters[852] + (CL * parameters[853])) + (CM * parameters[854])) + (CN * parameters[855]);
                let FN = ((parameters[856] + (CL * parameters[857])) + (CM * parameters[858])) + (CN * parameters[859]);
                let FO = ((parameters[862] + (CL * parameters[863])) + (CM * parameters[864])) + (CN * parameters[865]);
                let FP = ((parameters[877] + (CL * parameters[878])) + (CM * parameters[879])) + (CN * parameters[880]);
                let FQ = ((parameters[885] + (CL * parameters[886])) + (CM * parameters[887])) + (CN * parameters[888]);
                let FR = ((parameters[881] + (CL * parameters[882])) + (CM * parameters[883])) + (CN * parameters[884]);
                let FS = ((parameters[537] + (CL * parameters[564])) + (CM * parameters[565])) + (CN * parameters[566]);
                let FT = ((parameters[538] + (CL * parameters[567])) + (CM * parameters[568])) + (CN * parameters[569]);
                let FU = ((parameters[539] + (CL * parameters[570])) + (CM * parameters[571])) + (CN * parameters[572]);
                let FV = ((parameters[540] + (CL * parameters[573])) + (CM * parameters[574])) + (CN * parameters[575]);
                let FW = ((parameters[541] + (CL * parameters[576])) + (CM * parameters[577])) + (CN * parameters[578]);
                let FX = ((parameters[533] + (CL * parameters[579])) + (CM * parameters[580])) + (CN * parameters[581]);
                let FY = ((parameters[534] + (CL * parameters[582])) + (CM * parameters[583])) + (CN * parameters[584]);
                let FZ = ((parameters[535] + (CL * parameters[585])) + (CM * parameters[586])) + (CN * parameters[587]);
                let GA = ((parameters[536] + (CL * parameters[588])) + (CM * parameters[589])) + (CN * parameters[590]);
                let GB = ((parameters[542] + (CL * parameters[591])) + (CM * parameters[592])) + (CN * parameters[593]);
                let GC = ((parameters[543] + (CL * parameters[594])) + (CM * parameters[595])) + (CN * parameters[596]);
                let GD = ((parameters[544] + (CL * parameters[597])) + (CM * parameters[598])) + (CN * parameters[599]);
                let GE = ((parameters[545] + (CL * parameters[600])) + (CM * parameters[601])) + (CN * parameters[602]);
                let GF = ((parameters[546] + (CL * parameters[603])) + (CM * parameters[604])) + (CN * parameters[605]);
                let GG = ((parameters[547] + (CL * parameters[606])) + (CM * parameters[607])) + (CN * parameters[608]);
                let GH = ((parameters[548] + (CL * parameters[609])) + (CM * parameters[610])) + (CN * parameters[611]);
                let GI = ((parameters[549] + (CL * parameters[612])) + (CM * parameters[613])) + (CN * parameters[614]);
                let GJ = ((parameters[550] + (CL * parameters[615])) + (CM * parameters[616])) + (CN * parameters[617]);
                let GK = ((parameters[553] + (CL * parameters[618])) + (CM * parameters[619])) + (CN * parameters[620]);
                let GL = ((parameters[551] + (CL * parameters[621])) + (CM * parameters[622])) + (CN * parameters[623]);
                let GM = ((parameters[552] + (CL * parameters[624])) + (CM * parameters[625])) + (CN * parameters[626]);
                let GN = ((parameters[554] + (CL * parameters[627])) + (CM * parameters[628])) + (CN * parameters[629]);
                let GO = ((parameters[867] + (CL * parameters[870])) + (CM * parameters[871])) + (CN * parameters[872]);
                let GP = ((parameters[873] + (CL * parameters[874])) + (CM * parameters[875])) + (CN * parameters[876]);
                let GQ = ((parameters[425] + (CL * parameters[430])) + (CM * parameters[431])) + (CN * parameters[432]);
                let GR = ((parameters[444] + (CL * parameters[445])) + (CM * parameters[446])) + (CN * parameters[447]);
                let GS = ((parameters[448] + (CL * parameters[449])) + (CM * parameters[450])) + (CN * parameters[451]);
                let GT = ((parameters[452] + (CL * parameters[453])) + (CM * parameters[454])) + (CN * parameters[455]);
                let GU = ((parameters[456] + (CL * parameters[457])) + (CM * parameters[458])) + (CN * parameters[459]);
                let GV = ((parameters[1046] + (CL * parameters[1047])) + (CM * parameters[1048])) + (CN * parameters[1049]);
                let GW = ((parameters[1054] + (CL * parameters[1055])) + (CM * parameters[1056])) + (CN * parameters[1057]);
                let GX = ((parameters[1050] + (CL * parameters[1051])) + (CM * parameters[1052])) + (CN * parameters[1053]);
                let GY = ((parameters[1058] + (CL * parameters[1059])) + (CM * parameters[1060])) + (CN * parameters[1061]);
                let GZ = ((parameters[966] + (CL * parameters[967])) + (CM * parameters[968])) + (CN * parameters[969]);
                let HA = ((parameters[962] + (CL * parameters[963])) + (CM * parameters[964])) + (CN * parameters[965]);
                let HB = ((parameters[970] + (CL * parameters[971])) + (CM * parameters[972])) + (CN * parameters[973]);
                let HC = ((parameters[974] + (CL * parameters[975])) + (CM * parameters[976])) + (CN * parameters[977]);
                let HD = ((parameters[978] + (CL * parameters[979])) + (CM * parameters[980])) + (CN * parameters[981]);
                let HE = ((parameters[982] + (CL * parameters[983])) + (CM * parameters[984])) + (CN * parameters[985]);
                let HF = ((parameters[986] + (CL * parameters[987])) + (CM * parameters[988])) + (CN * parameters[989]);
                let HG = ((parameters[990] + (CL * parameters[991])) + (CM * parameters[992])) + (CN * parameters[993]);
                let HH = ((parameters[994] + (CL * parameters[995])) + (CM * parameters[996])) + (CN * parameters[997]);
                let HI = ((parameters[998] + (CL * parameters[999])) + (CM * parameters[1000])) + (CN * parameters[1001]);
                let HJ = ((parameters[1002] + (CL * parameters[1003])) + (CM * parameters[1004])) + (CN * parameters[1005]);
                let HK = ((parameters[1006] + (CL * parameters[1007])) + (CM * parameters[1008])) + (CN * parameters[1009]);
                let HL = ((parameters[1010] + (CL * parameters[1011])) + (CM * parameters[1012])) + (CN * parameters[1013]);
                let HM = ((parameters[1017] + (CL * parameters[1018])) + (CM * parameters[1019])) + (CN * parameters[1020]);
                let HN = ((parameters[1021] + (CL * parameters[1022])) + (CM * parameters[1023])) + (CN * parameters[1024]);
                let HO = ((parameters[1029] + (CL * parameters[1030])) + (CM * parameters[1031])) + (CN * parameters[1032]);
                let HP = ((parameters[1025] + (CL * parameters[1026])) + (CM * parameters[1027])) + (CN * parameters[1028]);
                let HQ = ((parameters[1033] + (CL * parameters[1034])) + (CM * parameters[1035])) + (CN * parameters[1036]);
                let HR = ((parameters[1037] + (CL * parameters[1038])) + (CM * parameters[1039])) + (CN * parameters[1040]);
                let HS = ((parameters[1069] + (CL * parameters[1070])) + (CM * parameters[1071])) + (CN * parameters[1072]);
                let HT = ((parameters[1073] + (CL * parameters[1074])) + (CM * parameters[1075])) + (CN * parameters[1076]);
                let HU = ((parameters[1077] + (CL * parameters[1078])) + (CM * parameters[1079])) + (CN * parameters[1080]);
                let HV = ((parameters[1081] + (CL * parameters[1082])) + (CM * parameters[1083])) + (CN * parameters[1084]);
                let HW = ((parameters[1085] + (CL * parameters[1086])) + (CM * parameters[1087])) + (CN * parameters[1088]);
                let HX = ((parameters[1089] + (CL * parameters[1090])) + (CM * parameters[1091])) + (CN * parameters[1092]);
                let HY = ((parameters[786] + (CL * parameters[787])) + (CM * parameters[788])) + (CN * parameters[789]);
                let HZ = ((parameters[794] + (CL * parameters[795])) + (CM * parameters[796])) + (CN * parameters[797]);
                let IA = ((parameters[790] + (CL * parameters[791])) + (CM * parameters[792])) + (CN * parameters[793]);
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
                let JD;
                if IB != 0.0 {
                    let IC = ((parameters[229] + (CL * parameters[230])) + (CM * parameters[231])) + (CN * parameters[232]);
                    let ID = ((parameters[175] + (CL * parameters[176])) + (CM * parameters[177])) + (CN * parameters[178]);
                    let IE = ((parameters[279] + (CL * parameters[280])) + (CM * parameters[281])) + (CN * parameters[282]);
                    let IF = ((parameters[294] + (CL * parameters[295])) + (CM * parameters[296])) + (CN * parameters[297]);
                    let IG = ((parameters[314] + (CL * parameters[315])) + (CM * parameters[316])) + (CN * parameters[317]);
                    let IH = ((parameters[322] + (CL * parameters[323])) + (CM * parameters[324])) + (CN * parameters[325]);
                    let II = ((parameters[336] + (CL * parameters[337])) + (CM * parameters[338])) + (CN * parameters[339]);
                    let IJ = ((parameters[346] + (CL * parameters[347])) + (CM * parameters[348])) + (CN * parameters[349]);
                    let IK = ((parameters[466] + (CL * parameters[467])) + (CM * parameters[468])) + (CN * parameters[469]);
                    let IL = ((parameters[249] + (CL * parameters[250])) + (CM * parameters[251])) + (CN * parameters[252]);
                    let IM = ((parameters[426] + (CL * parameters[427])) + (CM * parameters[428])) + (CN * parameters[429]);
                    let IN = ((parameters[440] + (CL * parameters[441])) + (CM * parameters[442])) + (CN * parameters[443]);
                    let IO = ((parameters[525] + (CL * parameters[526])) + (CM * parameters[527])) + (CN * parameters[528]);
                    let IP = ((parameters[529] + (CL * parameters[530])) + (CM * parameters[531])) + (CN * parameters[532]);
                    IQ = IC;
                    IR = IE;
                    IS = IF;
                    IT = IG;
                    IU = II;
                    IV = ID;
                    IW = IK;
                    IX = IJ;
                    IY = IL;
                    IZ = IM;
                    JA = IN;
                    JB = IO;
                    JC = IH;
                    JD = IP;
                } else {
                    IQ = G;
                    IR = G;
                    IS = G;
                    IT = G;
                    IU = G;
                    IV = G;
                    IW = G;
                    IX = G;
                    IY = G;
                    IZ = G;
                    JA = G;
                    JB = G;
                    JC = G;
                    JD = G;
                }
                let JE = CR * ((CI + ((parameters[81] * (if ((BB.powf(parameters[82])) - staged[4]) >= G { ((BB.powf(parameters[82])) - staged[4]) } else { G })) + (parameters[83] * (if ((BB.powf(parameters[84])) - staged[5]) >= G { ((BB.powf(parameters[84])) - staged[5]) } else { G })))) + ((parameters[85] * (if ((BC.powf(parameters[86])) - staged[6]) >= G { ((BC.powf(parameters[86])) - staged[6]) } else { G })) + (parameters[87] * (BF.powf(parameters[88])))));
                let JF = CV * ((CI + (parameters[214] * (if ((BB.powf(parameters[215])) - staged[7]) >= G { ((BB.powf(parameters[215])) - staged[7]) } else { G }))) + ((parameters[216] * (if ((BC.powf(parameters[217])) - staged[8]) >= G { ((BC.powf(parameters[217])) - staged[8]) } else { G })) + (parameters[218] * (BF.powf(parameters[219])))));
                let JG = CI + (parameters[224] * (if ((BB.powf(parameters[225])) - staged[9]) >= G { ((BB.powf(parameters[225])) - staged[9]) } else { G }));
                let JH = CW * JG;
                let JJ = if IB != 0.0 {
                    let JI = IQ * JG;
                    JI
                } else {
                    IQ
                };
                let JK = CX * (CI + (parameters[234] * (if ((BB.powf(parameters[235])) - staged[10]) >= G { ((BB.powf(parameters[235])) - staged[10]) } else { G })));
                let JL = parameters[34] * DL;
                let JR;
                let JS;
                if JM != 0.0 {
                    let JZ;
                    let KA;
                    if JN != 0.0 {
                        let JV = CI - (parameters[274] * (if ((BB.powf(parameters[275])) - staged[11]) >= G { ((BB.powf(parameters[275])) - staged[11]) } else { G }));
                        let JW = JL * JV;
                        let KC = if IB != 0.0 {
                            let KB = IR * JV;
                            KB
                        } else {
                            IR
                        };
                        JZ = JW;
                        KA = KC;
                    } else {
                        let JY = JL * JX;
                        let KE = if IB != 0.0 {
                            let KD = IR * JX;
                            KD
                        } else {
                            IR
                        };
                        JZ = JY;
                        KA = KE;
                    }
                    JR = JZ;
                    JS = KA;
                } else {
                    let JO = -AF;
                    let JP = (CI - (parameters[269] * (rspice_limited_exp((JO / parameters[270]))))) - (parameters[271] * (rspice_limited_exp((JO / parameters[272]))));
                    let JQ = JL * JP;
                    let KG = if IB != 0.0 {
                        let KF = IR * JP;
                        KF
                    } else {
                        IR
                    };
                    JR = JQ;
                    JS = KG;
                }
                let JT = (CI + (parameters[285] * (if ((BB.powf(parameters[286])) - staged[13]) >= G { ((BB.powf(parameters[286])) - staged[13]) } else { G }))) + ((parameters[287] * (if ((BC.powf(parameters[288])) - staged[14]) >= G { ((BC.powf(parameters[288])) - staged[14]) } else { G })) + (parameters[289] * (BF.powf(parameters[290]))));
                let JU = DM * JT;
                let KI = if IB != 0.0 {
                    let KH = IS * JT;
                    KH
                } else {
                    IS
                };
                let KJ = DO * ((CI + (parameters[302] * (if ((BB.powf(parameters[303])) - staged[15]) >= G { ((BB.powf(parameters[303])) - staged[15]) } else { G }))) + ((parameters[304] * (if ((BC.powf(parameters[305])) - staged[16]) >= G { ((BC.powf(parameters[305])) - staged[16]) } else { G })) + (parameters[306] * (BF.powf(parameters[307])))));
                let KK = CI + (parameters[309] * (if ((BB.powf(parameters[310])) - staged[17]) >= G { ((BB.powf(parameters[310])) - staged[17]) } else { G }));
                let KL = DN * KK;
                let KN = if IB != 0.0 {
                    let KM = IT * KK;
                    KM
                } else {
                    IT
                };
                let KO = (CI + (parameters[327] * (if ((BB.powf(parameters[328])) - staged[18]) >= G { ((BB.powf(parameters[328])) - staged[18]) } else { G }))) + ((parameters[329] * (if ((BC.powf(parameters[330])) - staged[19]) >= G { ((BC.powf(parameters[330])) - staged[19]) } else { G })) + (parameters[331] * (BF.powf(parameters[332]))));
                let KP = DQ * KO;
                let KR = if IB != 0.0 {
                    let KQ = IU * KO;
                    KQ
                } else {
                    IU
                };
                let KS = if ((BB.powf(parameters[179])) - staged[20]) >= G { ((BB.powf(parameters[179])) - staged[20]) } else { G };
                let KT = DI * KS;
                let KV = if IB != 0.0 {
                    let KU = IV * KS;
                    KU
                } else {
                    IV
                };
                let KW = DJ * (if ((BB.powf(parameters[181])) - staged[21]) >= G { ((BB.powf(parameters[181])) - staged[21]) } else { G });
                let KX = CI + (parameters[461] * (if ((BB.powf(parameters[462])) - staged[22]) >= G { ((BB.powf(parameters[462])) - staged[22]) } else { G }));
                let KY = ED * KX;
                let LA = if IB != 0.0 {
                    let KZ = IW * KX;
                    KZ
                } else {
                    IW
                };
                let LC = if (DK * (CI + (parameters[257] * (if ((BB.powf(parameters[258])) - staged[23]) >= G { ((BB.powf(parameters[258])) - staged[23]) } else { G })))) <= LB { (DK * (CI + (parameters[257] * (if ((BB.powf(parameters[258])) - staged[23]) >= G { ((BB.powf(parameters[258])) - staged[23]) } else { G })))) } else { LB };
                let LD = EJ * (CI + (parameters[479] * (if ((BB.powf(parameters[480])) - staged[24]) >= G { ((BB.powf(parameters[480])) - staged[24]) } else { G })));
                let LE = CI + (parameters[341] * (if ((BB.powf(parameters[342])) - staged[25]) >= G { ((BB.powf(parameters[342])) - staged[25]) } else { G }));
                let LF = if (DR * LE) >= G { (DR * LE) } else { G };
                let LH = if IB != 0.0 {
                    let LG = if (IX * LE) >= G { (IX * LE) } else { G };
                    LG
                } else {
                    IX
                };
                let LI = (CI + (parameters[243] * (if ((BB.powf(parameters[244])) - staged[26]) >= G { ((BB.powf(parameters[244])) - staged[26]) } else { G }))) + ((parameters[245] * (if ((BC.powf(parameters[246])) - staged[27]) >= G { ((BC.powf(parameters[246])) - staged[27]) } else { G })) + (parameters[247] * (BF.powf(parameters[248]))));
                let LJ = EL * LI;
                let LL = if IB != 0.0 {
                    let LK = IY * LI;
                    LK
                } else {
                    IY
                };
                let LM = CI + (parameters[423] * (if ((BB.powf(parameters[424])) - staged[28]) >= G { ((BB.powf(parameters[424])) - staged[28]) } else { G }));
                let LO = if (EM * LM) >= LN { (EM * LM) } else { LN };
                let LQ = if IB != 0.0 {
                    let LP = if (IZ * LM) >= LN { (IZ * LM) } else { LN };
                    LP
                } else {
                    IZ
                };
                let LR = CI + (parameters[438] * (if ((BB.powf(parameters[439])) - staged[29]) >= G { ((BB.powf(parameters[439])) - staged[29]) } else { G }));
                let LS = EC * LR;
                let LU = if IB != 0.0 {
                    let LT = JA * LR;
                    LT
                } else {
                    JA
                };
                let LV = (CI + (parameters[485] * (if ((BB.powf(parameters[486])) - staged[30]) >= G { ((BB.powf(parameters[486])) - staged[30]) } else { G }))) + (parameters[487] * (if ((BC.powf(parameters[488])) - staged[31]) >= G { ((BC.powf(parameters[488])) - staged[31]) } else { G }));
                let LW = ET * LV;
                let LY = if IB != 0.0 {
                    let LX = JB * LV;
                    LX
                } else {
                    JB
                };
                let LZ = EU * (CI + (parameters[495] * (if ((BC.powf(parameters[496])) - staged[32]) >= G { ((BC.powf(parameters[496])) - staged[32]) } else { G })));
                let MA = parameters[518] * (CI + (parameters[519] * (if ((BC.powf(parameters[520])) - staged[33]) >= G { ((BC.powf(parameters[520])) - staged[33]) } else { G })));
                let MB = parameters[521] * (CI + (parameters[522] * (if ((BC.powf(parameters[523])) - staged[34]) >= G { ((BC.powf(parameters[523])) - staged[34]) } else { G })));
                let MC = EY * ((CI + (parameters[631] * BB)) + (parameters[632] * BC));
                let MD = FC * ((CI + (parameters[649] * BB)) + (parameters[650] * BC));
                let ME = GB * ((CI + (parameters[557] * BB)) + (parameters[558] * BC));
                let MF = GE * ((CI + (parameters[559] * BB)) + (parameters[560] * BC));
                let MG = GH * ((CI + (parameters[561] * BB)) + (parameters[562] * BC));
                let MH = parameters[556] * (CI + (parameters[563] * BB));
                let MI = CS * ((CI + ((parameters[93] * (if ((BD.powf(parameters[94])) - staged[35]) >= G { ((BD.powf(parameters[94])) - staged[35]) } else { G })) + (parameters[95] * (if ((BD.powf(parameters[96])) - staged[36]) >= G { ((BD.powf(parameters[96])) - staged[36]) } else { G })))) + ((parameters[97] * (if ((BE.powf(parameters[98])) - staged[37]) >= G { ((BE.powf(parameters[98])) - staged[37]) } else { G })) + (parameters[99] * ((BE * BD).powf(parameters[100])))));
                let MJ = CO * ((CI + (parameters[120] * (if ((BD.powf(parameters[121])) - staged[38]) >= G { ((BD.powf(parameters[121])) - staged[38]) } else { G }))) + ((parameters[122] * (if ((BE.powf(parameters[123])) - staged[39]) >= G { ((BE.powf(parameters[123])) - staged[39]) } else { G })) + (parameters[124] * (BF.powf(parameters[125])))));
                let MK = CP * ((CI + (parameters[130] * (if ((BD.powf(parameters[131])) - staged[40]) >= G { ((BD.powf(parameters[131])) - staged[40]) } else { G }))) + ((parameters[132] * (if ((BE.powf(parameters[133])) - staged[41]) >= G { ((BE.powf(parameters[133])) - staged[41]) } else { G })) + (parameters[134] * (BF.powf(parameters[135])))));
                let ML = EN * ((CI + (parameters[263] * (if ((BD.powf(parameters[264])) - staged[42]) >= G { ((BD.powf(parameters[264])) - staged[42]) } else { G }))) + ((parameters[265] * (if ((BC.powf(parameters[266])) - staged[43]) >= G { ((BC.powf(parameters[266])) - staged[43]) } else { G })) + (parameters[267] * (BF.powf(parameters[268])))));
                let MM = if (DS * (CI + (parameters[352] * (if ((BD.powf(parameters[353])) - staged[44]) >= G { ((BD.powf(parameters[353])) - staged[44]) } else { G })))) >= G { (DS * (CI + (parameters[352] * (if ((BD.powf(parameters[353])) - staged[44]) >= G { ((BD.powf(parameters[353])) - staged[44]) } else { G })))) } else { G };
                let MN = DF * ((CI + (parameters[186] * (if ((BB.powf(parameters[187])) - staged[45]) >= G { ((BB.powf(parameters[187])) - staged[45]) } else { G }))) + ((parameters[188] * (if ((BC.powf(parameters[189])) - staged[46]) >= G { ((BC.powf(parameters[189])) - staged[46]) } else { G })) + (parameters[190] * (BF.powf(parameters[191])))));
                let MO = DE * ((CI + (parameters[196] * (if ((BB.powf(parameters[197])) - staged[47]) >= G { ((BB.powf(parameters[197])) - staged[47]) } else { G }))) + ((parameters[198] * (if ((BC.powf(parameters[199])) - staged[48]) >= G { ((BC.powf(parameters[199])) - staged[48]) } else { G })) + (parameters[200] * (BF.powf(parameters[201])))));
                let MP = DW * (CI + (parameters[383] * (if ((BB.powf(parameters[384])) - staged[49]) >= G { ((BB.powf(parameters[384])) - staged[49]) } else { G })));
                let MQ = FG * (CI + (BB * parameters[828]));
                let MR = FH * (CI + (BB * parameters[833]));
                let MS = FJ * (CI + (BB * parameters[842]));
                let MT = FN * (CI + (BB * parameters[860]));
                let MU = FO * (CI + (BB * parameters[866]));
                let MZ;
                let NA;
                let NB;
                if MV != 0.0 {
                    let MW = DT * (CI + (parameters[397] * (if ((BB.powf(parameters[398])) - staged[50]) >= G { ((BB.powf(parameters[398])) - staged[50]) } else { G })));
                    let MX = DU * (CI + (parameters[407] * (if ((BB.powf(parameters[408])) - staged[51]) >= G { ((BB.powf(parameters[408])) - staged[51]) } else { G })));
                    MZ = MW;
                    NA = MX;
                    NB = EA;
                } else {
                    let MY = EA * (CI + (parameters[414] * (if ((BB.powf(parameters[415])) - staged[52]) >= G { ((BB.powf(parameters[415])) - staged[52]) } else { G })));
                    MZ = DT;
                    NA = DU;
                    NB = MY;
                }
                let NC = if DP < CI { 1.0 } else { 0.0 };
                let NE;
                if NC != 0.0 {
                    NE = CI;
                } else {
                    let ND = if DP > AE { 1.0 } else { 0.0 };
                    oND = ND;
                    let NF = if ND != 0.0 {
                        AE
                    } else {
                        DP
                    };
                    NE = NF;
                }
                let NH;
                if IB != 0.0 {
                    let NG = if JC < CI { 1.0 } else { 0.0 };
                    oNG = NG;
                    let NK;
                    if NG != 0.0 {
                        NK = CI;
                    } else {
                        let NJ = if JC > AE { 1.0 } else { 0.0 };
                        oNJ = NJ;
                        let NL = if NJ != 0.0 {
                            AE
                        } else {
                            JC
                        };
                        NK = NL;
                    }
                    NH = NK;
                } else {
                    NH = JC;
                }
                let NI = if FA < G { 1.0 } else { 0.0 };
                let NM = if FE < G { 1.0 } else { 0.0 };
                let NN = if ES <= G { 1.0 } else { 0.0 };
                let NO = if ER <= G { 1.0 } else { 0.0 };
                let NP = if EH < G { 1.0 } else { 0.0 };
                let NQ = if CU < G { 1.0 } else { 0.0 };
                let NR = if JF < G { 1.0 } else { 0.0 };
                let NS = if MN < G { 1.0 } else { 0.0 };
                let NT = if CQ <= G { 1.0 } else { 0.0 };
                let NU = if JE <= G { 1.0 } else { 0.0 };
                let NV = if MI <= G { 1.0 } else { 0.0 };
                if NW != 0.0 {
                    let NX = if FW <= G { 1.0 } else { 0.0 };
                    oNX = NX;
                    let NZ = if GA <= G { 1.0 } else { 0.0 };
                    oNZ = NZ;
                } else {
                }
                if NY != 0.0 {
                    let OA = if GK <= G { 1.0 } else { 0.0 };
                    oOA = OA;
                } else {
                }
                let OB = if JH < G { 1.0 } else { 0.0 };
                if IB != 0.0 {
                    let OC = if JJ < G { 1.0 } else { 0.0 };
                    oOC = OC;
                } else {
                }
                let OD = if GL < G { 1.0 } else { 0.0 };
                let OE = if OD != 0.0 {
                    G
                } else {
                    GL
                };
                let OF = if GM < G { 1.0 } else { 0.0 };
                let OG = if OF != 0.0 {
                    G
                } else {
                    GM
                };
                let OH = if GW < G { 1.0 } else { 0.0 };
                let OI = if OH != 0.0 {
                    G
                } else {
                    GW
                };
                let OJ = if JR <= G { 1.0 } else { 0.0 };
                let OL = if OJ != 0.0 {
                    OK
                } else {
                    JR
                };
                let OM = if JU < G { 1.0 } else { 0.0 };
                let ON = if OM != 0.0 {
                    G
                } else {
                    JU
                };
                let OO = if KJ < G { 1.0 } else { 0.0 };
                let OP = if OO != 0.0 {
                    G
                } else {
                    KJ
                };
                let OQ = if KL < G { 1.0 } else { 0.0 };
                let OR = if OQ != 0.0 {
                    G
                } else {
                    KL
                };
                let OS = if NE < G { 1.0 } else { 0.0 };
                let OT = if OS != 0.0 {
                    G
                } else {
                    NE
                };
                let OU = if MA < G { 1.0 } else { 0.0 };
                let OV = if OU != 0.0 {
                    G
                } else {
                    MA
                };
                let PB;
                let PC;
                if OW != 0.0 {
                    let OY = if AF > OX { 1.0 } else { 0.0 };
                    oOY = OY;
                    let PF;
                    let PG;
                    if OY != 0.0 {
                        let PE = AF - OX;
                        PF = PE;
                        PG = OX;
                    } else {
                        PF = AF;
                        PG = AF;
                    }
                    let PI = if PH >= (PF / AE) { 1.0 } else { 0.0 };
                    oPI = PI;
                    let PJ = if PI != 0.0 {
                        G
                    } else {
                        PH
                    };
                    PB = PG;
                    PC = PJ;
                } else {
                    PB = OZ;
                    PC = PA;
                }
                let PO;
                let PP;
                let PQ;
                let PR;
                let PS;
                let PT;
                let PU;
                if PD != 0.0 {
                    let PL = PK * parameters[3];
                    PO = G;
                    PP = G;
                    PQ = G;
                    PR = G;
                    PS = PL;
                    PT = G;
                    PU = G;
                } else {
                    let PN = if (if PM > G { 1.0 } else { 0.0 }) != 0.0 && (if PK > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oPN = PN;
                    let PZ;
                    let QA;
                    let QB;
                    let QC;
                    let QD;
                    let QE;
                    let QF;
                    if PN != 0.0 {
                        let PY = if PW < PX { 1.0 } else { 0.0 };
                        oPY = PY;
                        let QH;
                        let QI;
                        let QJ;
                        let QK;
                        let QL;
                        if PY != 0.0 {
                            let QG = if (I % AE) != G { 1.0 } else { 0.0 };
                            oQG = QG;
                            let QQ;
                            let QR;
                            let QS;
                            let QT;
                            if QG != 0.0 {
                                let QN = AE * (if ((I - CI) / AE) >= G { ((I - CI) / AE) } else { G });
                                QQ = QN;
                                QR = QN;
                                QS = CI;
                                QT = CI;
                            } else {
                                let QP = if QO == CI { 1.0 } else { 0.0 };
                                oQP = QP;
                                let QX;
                                let QY;
                                let QZ;
                                let RA;
                                if QP != 0.0 {
                                    let QV = AE * (if ((I / AE) - CI) >= G { ((I / AE) - CI) } else { G });
                                    QX = I;
                                    QY = QV;
                                    QZ = G;
                                    RA = AE;
                                } else {
                                    let QW = AE * (if ((I / AE) - CI) >= G { ((I / AE) - CI) } else { G });
                                    QX = QW;
                                    QY = I;
                                    QZ = AE;
                                    RA = G;
                                }
                                QQ = QX;
                                QR = QY;
                                QS = QZ;
                                QT = RA;
                            }
                            let RD;
                            if QU != 0.0 {
                                let RB = if QQ == G { 1.0 } else { 0.0 };
                                oRB = RB;
                                let RG = if RB != 0.0 {
                                    G
                                } else {
                                    let RF = (PK * RE) / (AJ * QQ);
                                    RF
                                };
                                RD = RG;
                            } else {
                                let RC = if QR == G { 1.0 } else { 0.0 };
                                oRC = RC;
                                let RI = if RC != 0.0 {
                                    G
                                } else {
                                    let RH = (PK * RE) / (AJ * QR);
                                    RH
                                };
                                RD = RI;
                            }
                            QH = QS;
                            QI = QT;
                            QJ = RD;
                            QK = QQ;
                            QL = QR;
                        } else {
                            QH = G;
                            QI = G;
                            QJ = G;
                            QK = G;
                            QL = G;
                        }
                        let QM = if PW == G { 1.0 } else { 0.0 };
                        oQM = QM;
                        let RL;
                        let RM;
                        if QM != 0.0 {
                            let RQ;
                            if RJ != 0.0 {
                                let RW;
                                if RO != 0.0 {
                                    let RS = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oRS = RS;
                                    let SB;
                                    if RS != 0.0 {
                                        let RX = if QH == G { 1.0 } else { 0.0 };
                                        oRX = RX;
                                        let SD = if RX != 0.0 {
                                            G
                                        } else {
                                            let SC = (PK * RE) / (AJ * QH);
                                            SC
                                        };
                                        SB = SD;
                                    } else {
                                        let SA = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oSA = SA;
                                        let SH;
                                        if SA != 0.0 {
                                            let SF = RE + SE;
                                            let SG = if SF == G { 1.0 } else { 0.0 };
                                            oSG = SG;
                                            let SI = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || SG != 0.0 { 1.0 } else { 0.0 };
                                            oSI = SI;
                                            let SK = if SI != 0.0 {
                                                G
                                            } else {
                                                let SJ = (PK * AJ) / ((RT * QH) * SF);
                                                SJ
                                            };
                                            SH = SK;
                                        } else {
                                            SH = G;
                                        }
                                        SB = SH;
                                    }
                                    RW = SB;
                                } else {
                                    let RV = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oRV = RV;
                                    let SO;
                                    if RV != 0.0 {
                                        let SL = if QH == G { 1.0 } else { 0.0 };
                                        oSL = SL;
                                        let SQ = if SL != 0.0 {
                                            G
                                        } else {
                                            let SP = (PK * RE) / (AJ * QH);
                                            SP
                                        };
                                        SO = SQ;
                                    } else {
                                        let SN = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oSN = SN;
                                        let ST;
                                        if SN != 0.0 {
                                            let SR = RE + SE;
                                            let SS = if SR == G { 1.0 } else { 0.0 };
                                            oSS = SS;
                                            let SU = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || SS != 0.0 { 1.0 } else { 0.0 };
                                            oSU = SU;
                                            let SW = if SU != 0.0 {
                                                G
                                            } else {
                                                let SV = (PK * AJ) / ((RT * QH) * SR);
                                                SV
                                            };
                                            ST = SW;
                                        } else {
                                            ST = G;
                                        }
                                        SO = ST;
                                    }
                                    RW = SO;
                                }
                                RQ = RW;
                            } else {
                                let SZ;
                                if RP != 0.0 {
                                    let SX = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oSX = SX;
                                    let TC;
                                    if SX != 0.0 {
                                        let TA = if QI == G { 1.0 } else { 0.0 };
                                        oTA = TA;
                                        let TE = if TA != 0.0 {
                                            G
                                        } else {
                                            let TD = (PK * RE) / (AJ * QI);
                                            TD
                                        };
                                        TC = TE;
                                    } else {
                                        let TB = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oTB = TB;
                                        let TH;
                                        if TB != 0.0 {
                                            let TF = RE + SE;
                                            let TG = if TF == G { 1.0 } else { 0.0 };
                                            oTG = TG;
                                            let TI = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || TG != 0.0 { 1.0 } else { 0.0 };
                                            oTI = TI;
                                            let TK = if TI != 0.0 {
                                                G
                                            } else {
                                                let TJ = (PK * AJ) / ((RT * QI) * TF);
                                                TJ
                                            };
                                            TH = TK;
                                        } else {
                                            TH = G;
                                        }
                                        TC = TH;
                                    }
                                    SZ = TC;
                                } else {
                                    let SY = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oSY = SY;
                                    let TN;
                                    if SY != 0.0 {
                                        let TL = if QI == G { 1.0 } else { 0.0 };
                                        oTL = TL;
                                        let TP = if TL != 0.0 {
                                            G
                                        } else {
                                            let TO = (PK * RE) / (AJ * QI);
                                            TO
                                        };
                                        TN = TP;
                                    } else {
                                        let TM = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oTM = TM;
                                        let TS;
                                        if TM != 0.0 {
                                            let TQ = RE + SE;
                                            let TR = if TQ == G { 1.0 } else { 0.0 };
                                            oTR = TR;
                                            let TT = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || TR != 0.0 { 1.0 } else { 0.0 };
                                            oTT = TT;
                                            let TV = if TT != 0.0 {
                                                G
                                            } else {
                                                let TU = (PK * AJ) / ((RT * QI) * TQ);
                                                TU
                                            };
                                            TS = TV;
                                        } else {
                                            TS = G;
                                        }
                                        TN = TS;
                                    }
                                    SZ = TN;
                                }
                                RQ = SZ;
                            }
                            RL = QJ;
                            RM = RQ;
                        } else {
                            let RK = if PW == CI { 1.0 } else { 0.0 };
                            oRK = RK;
                            let TY;
                            let TZ;
                            if RK != 0.0 {
                                let UC;
                                if TW != 0.0 {
                                    let UF;
                                    if UA != 0.0 {
                                        let UD = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oUD = UD;
                                        let UI;
                                        if UD != 0.0 {
                                            let UG = if QH == G { 1.0 } else { 0.0 };
                                            oUG = UG;
                                            let UK = if UG != 0.0 {
                                                G
                                            } else {
                                                let UJ = (PK * RE) / (AJ * QH);
                                                UJ
                                            };
                                            UI = UK;
                                        } else {
                                            let UH = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oUH = UH;
                                            let UN;
                                            if UH != 0.0 {
                                                let UL = RE + SE;
                                                let UM = if UL == G { 1.0 } else { 0.0 };
                                                oUM = UM;
                                                let UO = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || UM != 0.0 { 1.0 } else { 0.0 };
                                                oUO = UO;
                                                let UQ = if UO != 0.0 {
                                                    G
                                                } else {
                                                    let UP = (PK * AJ) / ((RT * QH) * UL);
                                                    UP
                                                };
                                                UN = UQ;
                                            } else {
                                                UN = G;
                                            }
                                            UI = UN;
                                        }
                                        UF = UI;
                                    } else {
                                        let UE = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oUE = UE;
                                        let UT;
                                        if UE != 0.0 {
                                            let UR = if QH == G { 1.0 } else { 0.0 };
                                            oUR = UR;
                                            let UV = if UR != 0.0 {
                                                G
                                            } else {
                                                let UU = (PK * RE) / (AJ * QH);
                                                UU
                                            };
                                            UT = UV;
                                        } else {
                                            let US = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oUS = US;
                                            let UY;
                                            if US != 0.0 {
                                                let UW = RE + SE;
                                                let UX = if UW == G { 1.0 } else { 0.0 };
                                                oUX = UX;
                                                let UZ = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || UX != 0.0 { 1.0 } else { 0.0 };
                                                oUZ = UZ;
                                                let VB = if UZ != 0.0 {
                                                    G
                                                } else {
                                                    let VA = (PK * AJ) / ((RT * QH) * UW);
                                                    VA
                                                };
                                                UY = VB;
                                            } else {
                                                UY = G;
                                            }
                                            UT = UY;
                                        }
                                        UF = UT;
                                    }
                                    UC = UF;
                                } else {
                                    let VE;
                                    if UB != 0.0 {
                                        let VC = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oVC = VC;
                                        let VH;
                                        if VC != 0.0 {
                                            let VF = if QI == G { 1.0 } else { 0.0 };
                                            oVF = VF;
                                            let VJ = if VF != 0.0 {
                                                G
                                            } else {
                                                let VI = (PK * RE) / (AJ * QI);
                                                VI
                                            };
                                            VH = VJ;
                                        } else {
                                            let VG = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oVG = VG;
                                            let VL;
                                            if VG != 0.0 {
                                                let VK = if RE == G { 1.0 } else { 0.0 };
                                                oVK = VK;
                                                let VM = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || VK != 0.0 { 1.0 } else { 0.0 };
                                                oVM = VM;
                                                let VO = if VM != 0.0 {
                                                    G
                                                } else {
                                                    let VN = (PK * AJ) / ((RZ * QI) * RE);
                                                    VN
                                                };
                                                VL = VO;
                                            } else {
                                                VL = G;
                                            }
                                            VH = VL;
                                        }
                                        VE = VH;
                                    } else {
                                        let VD = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oVD = VD;
                                        let VR;
                                        if VD != 0.0 {
                                            let VP = if QI == G { 1.0 } else { 0.0 };
                                            oVP = VP;
                                            let VT = if VP != 0.0 {
                                                G
                                            } else {
                                                let VS = (PK * RE) / (AJ * QI);
                                                VS
                                            };
                                            VR = VT;
                                        } else {
                                            let VQ = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oVQ = VQ;
                                            let VV;
                                            if VQ != 0.0 {
                                                let VU = if RE == G { 1.0 } else { 0.0 };
                                                oVU = VU;
                                                let VW = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || VU != 0.0 { 1.0 } else { 0.0 };
                                                oVW = VW;
                                                let VY = if VW != 0.0 {
                                                    G
                                                } else {
                                                    let VX = (PK * AJ) / ((RZ * QI) * RE);
                                                    VX
                                                };
                                                VV = VY;
                                            } else {
                                                VV = G;
                                            }
                                            VR = VV;
                                        }
                                        VE = VR;
                                    }
                                    UC = VE;
                                }
                                TY = QJ;
                                TZ = UC;
                            } else {
                                let TX = if PW == AE { 1.0 } else { 0.0 };
                                oTX = TX;
                                let WB;
                                let WC;
                                if TX != 0.0 {
                                    let WF;
                                    if VZ != 0.0 {
                                        let WI;
                                        if WD != 0.0 {
                                            let WG = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oWG = WG;
                                            let WL;
                                            if WG != 0.0 {
                                                let WJ = if QH == G { 1.0 } else { 0.0 };
                                                oWJ = WJ;
                                                let WN = if WJ != 0.0 {
                                                    G
                                                } else {
                                                    let WM = (PK * RE) / (AJ * QH);
                                                    WM
                                                };
                                                WL = WN;
                                            } else {
                                                let WK = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oWK = WK;
                                                let WP;
                                                if WK != 0.0 {
                                                    let WO = if RE == G { 1.0 } else { 0.0 };
                                                    oWO = WO;
                                                    let WQ = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || WO != 0.0 { 1.0 } else { 0.0 };
                                                    oWQ = WQ;
                                                    let WS = if WQ != 0.0 {
                                                        G
                                                    } else {
                                                        let WR = (PK * AJ) / ((RZ * QH) * RE);
                                                        WR
                                                    };
                                                    WP = WS;
                                                } else {
                                                    WP = G;
                                                }
                                                WL = WP;
                                            }
                                            WI = WL;
                                        } else {
                                            let WH = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oWH = WH;
                                            let WV;
                                            if WH != 0.0 {
                                                let WT = if QH == G { 1.0 } else { 0.0 };
                                                oWT = WT;
                                                let WX = if WT != 0.0 {
                                                    G
                                                } else {
                                                    let WW = (PK * RE) / (AJ * QH);
                                                    WW
                                                };
                                                WV = WX;
                                            } else {
                                                let WU = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oWU = WU;
                                                let WZ;
                                                if WU != 0.0 {
                                                    let WY = if RE == G { 1.0 } else { 0.0 };
                                                    oWY = WY;
                                                    let XA = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || WY != 0.0 { 1.0 } else { 0.0 };
                                                    oXA = XA;
                                                    let XC = if XA != 0.0 {
                                                        G
                                                    } else {
                                                        let XB = (PK * AJ) / ((RZ * QH) * RE);
                                                        XB
                                                    };
                                                    WZ = XC;
                                                } else {
                                                    WZ = G;
                                                }
                                                WV = WZ;
                                            }
                                            WI = WV;
                                        }
                                        WF = WI;
                                    } else {
                                        let XF;
                                        if WE != 0.0 {
                                            let XD = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oXD = XD;
                                            let XI;
                                            if XD != 0.0 {
                                                let XG = if QI == G { 1.0 } else { 0.0 };
                                                oXG = XG;
                                                let XK = if XG != 0.0 {
                                                    G
                                                } else {
                                                    let XJ = (PK * RE) / (AJ * QI);
                                                    XJ
                                                };
                                                XI = XK;
                                            } else {
                                                let XH = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oXH = XH;
                                                let XN;
                                                if XH != 0.0 {
                                                    let XL = RE + SE;
                                                    let XM = if XL == G { 1.0 } else { 0.0 };
                                                    oXM = XM;
                                                    let XO = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || XM != 0.0 { 1.0 } else { 0.0 };
                                                    oXO = XO;
                                                    let XQ = if XO != 0.0 {
                                                        G
                                                    } else {
                                                        let XP = (PK * AJ) / ((RT * QI) * XL);
                                                        XP
                                                    };
                                                    XN = XQ;
                                                } else {
                                                    XN = G;
                                                }
                                                XI = XN;
                                            }
                                            XF = XI;
                                        } else {
                                            let XE = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oXE = XE;
                                            let XT;
                                            if XE != 0.0 {
                                                let XR = if QI == G { 1.0 } else { 0.0 };
                                                oXR = XR;
                                                let XV = if XR != 0.0 {
                                                    G
                                                } else {
                                                    let XU = (PK * RE) / (AJ * QI);
                                                    XU
                                                };
                                                XT = XV;
                                            } else {
                                                let XS = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oXS = XS;
                                                let XY;
                                                if XS != 0.0 {
                                                    let XW = RE + SE;
                                                    let XX = if XW == G { 1.0 } else { 0.0 };
                                                    oXX = XX;
                                                    let XZ = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || XX != 0.0 { 1.0 } else { 0.0 };
                                                    oXZ = XZ;
                                                    let YB = if XZ != 0.0 {
                                                        G
                                                    } else {
                                                        let YA = (PK * AJ) / ((RT * QI) * XW);
                                                        YA
                                                    };
                                                    XY = YB;
                                                } else {
                                                    XY = G;
                                                }
                                                XT = XY;
                                            }
                                            XF = XT;
                                        }
                                        WF = XF;
                                    }
                                    WB = QJ;
                                    WC = WF;
                                } else {
                                    let WA = if PW == RT { 1.0 } else { 0.0 };
                                    oWA = WA;
                                    let YE;
                                    let YF;
                                    if WA != 0.0 {
                                        let YI;
                                        if YC != 0.0 {
                                            let YL;
                                            if YG != 0.0 {
                                                let YJ = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oYJ = YJ;
                                                let YO;
                                                if YJ != 0.0 {
                                                    let YM = if QH == G { 1.0 } else { 0.0 };
                                                    oYM = YM;
                                                    let YQ = if YM != 0.0 {
                                                        G
                                                    } else {
                                                        let YP = (PK * RE) / (AJ * QH);
                                                        YP
                                                    };
                                                    YO = YQ;
                                                } else {
                                                    let YN = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oYN = YN;
                                                    let YS;
                                                    if YN != 0.0 {
                                                        let YR = if RE == G { 1.0 } else { 0.0 };
                                                        oYR = YR;
                                                        let YT = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || YR != 0.0 { 1.0 } else { 0.0 };
                                                        oYT = YT;
                                                        let YV = if YT != 0.0 {
                                                            G
                                                        } else {
                                                            let YU = (PK * AJ) / ((RZ * QH) * RE);
                                                            YU
                                                        };
                                                        YS = YV;
                                                    } else {
                                                        YS = G;
                                                    }
                                                    YO = YS;
                                                }
                                                YL = YO;
                                            } else {
                                                let YK = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oYK = YK;
                                                let YY;
                                                if YK != 0.0 {
                                                    let YW = if QH == G { 1.0 } else { 0.0 };
                                                    oYW = YW;
                                                    let ZA = if YW != 0.0 {
                                                        G
                                                    } else {
                                                        let YZ = (PK * RE) / (AJ * QH);
                                                        YZ
                                                    };
                                                    YY = ZA;
                                                } else {
                                                    let YX = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oYX = YX;
                                                    let ZC;
                                                    if YX != 0.0 {
                                                        let ZB = if RE == G { 1.0 } else { 0.0 };
                                                        oZB = ZB;
                                                        let ZD = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || ZB != 0.0 { 1.0 } else { 0.0 };
                                                        oZD = ZD;
                                                        let ZF = if ZD != 0.0 {
                                                            G
                                                        } else {
                                                            let ZE = (PK * AJ) / ((RZ * QH) * RE);
                                                            ZE
                                                        };
                                                        ZC = ZF;
                                                    } else {
                                                        ZC = G;
                                                    }
                                                    YY = ZC;
                                                }
                                                YL = YY;
                                            }
                                            YI = YL;
                                        } else {
                                            let ZI;
                                            if YH != 0.0 {
                                                let ZG = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oZG = ZG;
                                                let ZL;
                                                if ZG != 0.0 {
                                                    let ZJ = if QI == G { 1.0 } else { 0.0 };
                                                    oZJ = ZJ;
                                                    let ZN = if ZJ != 0.0 {
                                                        G
                                                    } else {
                                                        let ZM = (PK * RE) / (AJ * QI);
                                                        ZM
                                                    };
                                                    ZL = ZN;
                                                } else {
                                                    let ZK = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oZK = ZK;
                                                    let ZP;
                                                    if ZK != 0.0 {
                                                        let ZO = if RE == G { 1.0 } else { 0.0 };
                                                        oZO = ZO;
                                                        let ZQ = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || ZO != 0.0 { 1.0 } else { 0.0 };
                                                        oZQ = ZQ;
                                                        let ZS = if ZQ != 0.0 {
                                                            G
                                                        } else {
                                                            let ZR = (PK * AJ) / ((RZ * QI) * RE);
                                                            ZR
                                                        };
                                                        ZP = ZS;
                                                    } else {
                                                        ZP = G;
                                                    }
                                                    ZL = ZP;
                                                }
                                                ZI = ZL;
                                            } else {
                                                let ZH = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oZH = ZH;
                                                let ZV;
                                                if ZH != 0.0 {
                                                    let ZT = if QI == G { 1.0 } else { 0.0 };
                                                    oZT = ZT;
                                                    let ZX = if ZT != 0.0 {
                                                        G
                                                    } else {
                                                        let ZW = (PK * RE) / (AJ * QI);
                                                        ZW
                                                    };
                                                    ZV = ZX;
                                                } else {
                                                    let ZU = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oZU = ZU;
                                                    let ZZ;
                                                    if ZU != 0.0 {
                                                        let ZY = if RE == G { 1.0 } else { 0.0 };
                                                        oZY = ZY;
                                                        let AAA = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || ZY != 0.0 { 1.0 } else { 0.0 };
                                                        oAAA = AAA;
                                                        let AAC = if AAA != 0.0 {
                                                            G
                                                        } else {
                                                            let AAB = (PK * AJ) / ((RZ * QI) * RE);
                                                            AAB
                                                        };
                                                        ZZ = AAC;
                                                    } else {
                                                        ZZ = G;
                                                    }
                                                    ZV = ZZ;
                                                }
                                                ZI = ZV;
                                            }
                                            YI = ZI;
                                        }
                                        YE = QJ;
                                        YF = YI;
                                    } else {
                                        let YD = if PW == RY { 1.0 } else { 0.0 };
                                        oYD = YD;
                                        let AAF;
                                        let AAG;
                                        if YD != 0.0 {
                                            let AAK;
                                            if AAD != 0.0 {
                                                let AAN;
                                                if AAH != 0.0 {
                                                    let AAL = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAAL = AAL;
                                                    let AAQ;
                                                    if AAL != 0.0 {
                                                        let AAO = if QH == G { 1.0 } else { 0.0 };
                                                        oAAO = AAO;
                                                        let AAS = if AAO != 0.0 {
                                                            G
                                                        } else {
                                                            let AAR = (PK * RE) / (AJ * QH);
                                                            AAR
                                                        };
                                                        AAQ = AAS;
                                                    } else {
                                                        let AAP = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oAAP = AAP;
                                                        let AAV;
                                                        if AAP != 0.0 {
                                                            let AAT = RE + SE;
                                                            let AAU = if AAT == G { 1.0 } else { 0.0 };
                                                            oAAU = AAU;
                                                            let AAW = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || AAU != 0.0 { 1.0 } else { 0.0 };
                                                            oAAW = AAW;
                                                            let AAY = if AAW != 0.0 {
                                                                G
                                                            } else {
                                                                let AAX = (PK * AJ) / ((RT * QH) * AAT);
                                                                AAX
                                                            };
                                                            AAV = AAY;
                                                        } else {
                                                            AAV = G;
                                                        }
                                                        AAQ = AAV;
                                                    }
                                                    AAN = AAQ;
                                                } else {
                                                    let AAM = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAAM = AAM;
                                                    let ABB;
                                                    if AAM != 0.0 {
                                                        let AAZ = if QH == G { 1.0 } else { 0.0 };
                                                        oAAZ = AAZ;
                                                        let ABD = if AAZ != 0.0 {
                                                            G
                                                        } else {
                                                            let ABC = (PK * RE) / (AJ * QH);
                                                            ABC
                                                        };
                                                        ABB = ABD;
                                                    } else {
                                                        let ABA = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oABA = ABA;
                                                        let ABG;
                                                        if ABA != 0.0 {
                                                            let ABE = RE + SE;
                                                            let ABF = if ABE == G { 1.0 } else { 0.0 };
                                                            oABF = ABF;
                                                            let ABH = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || ABF != 0.0 { 1.0 } else { 0.0 };
                                                            oABH = ABH;
                                                            let ABJ = if ABH != 0.0 {
                                                                G
                                                            } else {
                                                                let ABI = (PK * AJ) / ((RT * QH) * ABE);
                                                                ABI
                                                            };
                                                            ABG = ABJ;
                                                        } else {
                                                            ABG = G;
                                                        }
                                                        ABB = ABG;
                                                    }
                                                    AAN = ABB;
                                                }
                                                AAK = AAN;
                                            } else {
                                                let AAJ = (PK * AAI) / AJ;
                                                AAK = AAJ;
                                            }
                                            AAF = QJ;
                                            AAG = AAK;
                                        } else {
                                            let AAE = if PW == RR { 1.0 } else { 0.0 };
                                            oAAE = AAE;
                                            let ABM;
                                            let ABN;
                                            if AAE != 0.0 {
                                                let ABQ;
                                                if ABK != 0.0 {
                                                    let ABT;
                                                    if ABO != 0.0 {
                                                        let ABR = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oABR = ABR;
                                                        let ABW;
                                                        if ABR != 0.0 {
                                                            let ABU = if QH == G { 1.0 } else { 0.0 };
                                                            oABU = ABU;
                                                            let ABY = if ABU != 0.0 {
                                                                G
                                                            } else {
                                                                let ABX = (PK * RE) / (AJ * QH);
                                                                ABX
                                                            };
                                                            ABW = ABY;
                                                        } else {
                                                            let ABV = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oABV = ABV;
                                                            let ACA;
                                                            if ABV != 0.0 {
                                                                let ABZ = if RE == G { 1.0 } else { 0.0 };
                                                                oABZ = ABZ;
                                                                let ACB = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || ABZ != 0.0 { 1.0 } else { 0.0 };
                                                                oACB = ACB;
                                                                let ACD = if ACB != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ACC = (PK * AJ) / ((RZ * QH) * RE);
                                                                    ACC
                                                                };
                                                                ACA = ACD;
                                                            } else {
                                                                ACA = G;
                                                            }
                                                            ABW = ACA;
                                                        }
                                                        ABT = ABW;
                                                    } else {
                                                        let ABS = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oABS = ABS;
                                                        let ACG;
                                                        if ABS != 0.0 {
                                                            let ACE = if QH == G { 1.0 } else { 0.0 };
                                                            oACE = ACE;
                                                            let ACI = if ACE != 0.0 {
                                                                G
                                                            } else {
                                                                let ACH = (PK * RE) / (AJ * QH);
                                                                ACH
                                                            };
                                                            ACG = ACI;
                                                        } else {
                                                            let ACF = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oACF = ACF;
                                                            let ACK;
                                                            if ACF != 0.0 {
                                                                let ACJ = if RE == G { 1.0 } else { 0.0 };
                                                                oACJ = ACJ;
                                                                let ACL = if (if QH == G { 1.0 } else { 0.0 }) != 0.0 || ACJ != 0.0 { 1.0 } else { 0.0 };
                                                                oACL = ACL;
                                                                let ACN = if ACL != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ACM = (PK * AJ) / ((RZ * QH) * RE);
                                                                    ACM
                                                                };
                                                                ACK = ACN;
                                                            } else {
                                                                ACK = G;
                                                            }
                                                            ACG = ACK;
                                                        }
                                                        ABT = ACG;
                                                    }
                                                    ABQ = ABT;
                                                } else {
                                                    let ABP = if QI == G { 1.0 } else { 0.0 };
                                                    oABP = ABP;
                                                    let ACP = if ABP != 0.0 {
                                                        G
                                                    } else {
                                                        let ACO = (PK * AAI) / (AJ * QI);
                                                        ACO
                                                    };
                                                    ABQ = ACP;
                                                }
                                                ABM = QJ;
                                                ABN = ABQ;
                                            } else {
                                                let ABL = if PW == RZ { 1.0 } else { 0.0 };
                                                oABL = ABL;
                                                let ACS;
                                                let ACT;
                                                if ABL != 0.0 {
                                                    let ACW;
                                                    if ACQ != 0.0 {
                                                        let ACU = (PK * AAI) / AJ;
                                                        ACW = ACU;
                                                    } else {
                                                        let ACZ;
                                                        if ACV != 0.0 {
                                                            let ACX = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oACX = ACX;
                                                            let ADC;
                                                            if ACX != 0.0 {
                                                                let ADA = if QI == G { 1.0 } else { 0.0 };
                                                                oADA = ADA;
                                                                let ADE = if ADA != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ADD = (PK * RE) / (AJ * QI);
                                                                    ADD
                                                                };
                                                                ADC = ADE;
                                                            } else {
                                                                let ADB = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oADB = ADB;
                                                                let ADH;
                                                                if ADB != 0.0 {
                                                                    let ADF = RE + SE;
                                                                    let ADG = if ADF == G { 1.0 } else { 0.0 };
                                                                    oADG = ADG;
                                                                    let ADI = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || ADG != 0.0 { 1.0 } else { 0.0 };
                                                                    oADI = ADI;
                                                                    let ADK = if ADI != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let ADJ = (PK * AJ) / ((RT * QI) * ADF);
                                                                        ADJ
                                                                    };
                                                                    ADH = ADK;
                                                                } else {
                                                                    ADH = G;
                                                                }
                                                                ADC = ADH;
                                                            }
                                                            ACZ = ADC;
                                                        } else {
                                                            let ACY = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oACY = ACY;
                                                            let ADN;
                                                            if ACY != 0.0 {
                                                                let ADL = if QI == G { 1.0 } else { 0.0 };
                                                                oADL = ADL;
                                                                let ADP = if ADL != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ADO = (PK * RE) / (AJ * QI);
                                                                    ADO
                                                                };
                                                                ADN = ADP;
                                                            } else {
                                                                let ADM = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oADM = ADM;
                                                                let ADS;
                                                                if ADM != 0.0 {
                                                                    let ADQ = RE + SE;
                                                                    let ADR = if ADQ == G { 1.0 } else { 0.0 };
                                                                    oADR = ADR;
                                                                    let ADT = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || ADR != 0.0 { 1.0 } else { 0.0 };
                                                                    oADT = ADT;
                                                                    let ADV = if ADT != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let ADU = (PK * AJ) / ((RT * QI) * ADQ);
                                                                        ADU
                                                                    };
                                                                    ADS = ADV;
                                                                } else {
                                                                    ADS = G;
                                                                }
                                                                ADN = ADS;
                                                            }
                                                            ACZ = ADN;
                                                        }
                                                        ACW = ACZ;
                                                    }
                                                    ACS = QJ;
                                                    ACT = ACW;
                                                } else {
                                                    let ACR = if PW == RU { 1.0 } else { 0.0 };
                                                    oACR = ACR;
                                                    let ADY;
                                                    let ADZ;
                                                    if ACR != 0.0 {
                                                        let AEC;
                                                        if ADW != 0.0 {
                                                            let AEA = if QH == G { 1.0 } else { 0.0 };
                                                            oAEA = AEA;
                                                            let AEE = if AEA != 0.0 {
                                                                G
                                                            } else {
                                                                let AED = (PK * AAI) / (AJ * QH);
                                                                AED
                                                            };
                                                            AEC = AEE;
                                                        } else {
                                                            let AEH;
                                                            if AEB != 0.0 {
                                                                let AEF = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oAEF = AEF;
                                                                let AEK;
                                                                if AEF != 0.0 {
                                                                    let AEI = if QI == G { 1.0 } else { 0.0 };
                                                                    oAEI = AEI;
                                                                    let AEM = if AEI != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let AEL = (PK * RE) / (AJ * QI);
                                                                        AEL
                                                                    };
                                                                    AEK = AEM;
                                                                } else {
                                                                    let AEJ = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                    oAEJ = AEJ;
                                                                    let AEO;
                                                                    if AEJ != 0.0 {
                                                                        let AEN = if RE == G { 1.0 } else { 0.0 };
                                                                        oAEN = AEN;
                                                                        let AEP = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || AEN != 0.0 { 1.0 } else { 0.0 };
                                                                        oAEP = AEP;
                                                                        let AER = if AEP != 0.0 {
                                                                            G
                                                                        } else {
                                                                            let AEQ = (PK * AJ) / ((RZ * QI) * RE);
                                                                            AEQ
                                                                        };
                                                                        AEO = AER;
                                                                    } else {
                                                                        AEO = G;
                                                                    }
                                                                    AEK = AEO;
                                                                }
                                                                AEH = AEK;
                                                            } else {
                                                                let AEG = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oAEG = AEG;
                                                                let AEU;
                                                                if AEG != 0.0 {
                                                                    let AES = if QI == G { 1.0 } else { 0.0 };
                                                                    oAES = AES;
                                                                    let AEW = if AES != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let AEV = (PK * RE) / (AJ * QI);
                                                                        AEV
                                                                    };
                                                                    AEU = AEW;
                                                                } else {
                                                                    let AET = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                    oAET = AET;
                                                                    let AEY;
                                                                    if AET != 0.0 {
                                                                        let AEX = if RE == G { 1.0 } else { 0.0 };
                                                                        oAEX = AEX;
                                                                        let AEZ = if (if QI == G { 1.0 } else { 0.0 }) != 0.0 || AEX != 0.0 { 1.0 } else { 0.0 };
                                                                        oAEZ = AEZ;
                                                                        let AFB = if AEZ != 0.0 {
                                                                            G
                                                                        } else {
                                                                            let AFA = (PK * AJ) / ((RZ * QI) * RE);
                                                                            AFA
                                                                        };
                                                                        AEY = AFB;
                                                                    } else {
                                                                        AEY = G;
                                                                    }
                                                                    AEU = AEY;
                                                                }
                                                                AEH = AEU;
                                                            }
                                                            AEC = AEH;
                                                        }
                                                        ADY = QJ;
                                                        ADZ = AEC;
                                                    } else {
                                                        let ADX = if PW == SM { 1.0 } else { 0.0 };
                                                        oADX = ADX;
                                                        let AFE;
                                                        let AFF;
                                                        if ADX != 0.0 {
                                                            let AFC = (PK * AAI) / AJ;
                                                            AFE = QJ;
                                                            AFF = AFC;
                                                        } else {
                                                            let AFD = if PW == PX { 1.0 } else { 0.0 };
                                                            oAFD = AFD;
                                                            let AFJ;
                                                            let AFK;
                                                            if AFD != 0.0 {
                                                                let AFO;
                                                                let AFP;
                                                                if AFG != 0.0 {
                                                                    let AFL = ((LB * PK) * RE) / AJ;
                                                                    let AFM = if I == AE { 1.0 } else { 0.0 };
                                                                    oAFM = AFM;
                                                                    let AFR = if AFM != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let AFQ = (PK * RE) / (AJ * (I - AE));
                                                                        AFQ
                                                                    };
                                                                    AFO = AFR;
                                                                    AFP = AFL;
                                                                } else {
                                                                    let AFN = (PK * RE) / (AJ * I);
                                                                    AFO = AFN;
                                                                    AFP = G;
                                                                }
                                                                AFJ = AFO;
                                                                AFK = AFP;
                                                            } else {
                                                                let AFI = if PW == AFH { 1.0 } else { 0.0 };
                                                                oAFI = AFI;
                                                                let AFT;
                                                                let AFU;
                                                                if AFI != 0.0 {
                                                                    let AFY;
                                                                    let AFZ;
                                                                    if AFS != 0.0 {
                                                                        let AFV = (PK * RE) / (AJ * I);
                                                                        AFY = AFV;
                                                                        AFZ = G;
                                                                    } else {
                                                                        let AFW = ((LB * PK) * RE) / AJ;
                                                                        let AFX = if I == AE { 1.0 } else { 0.0 };
                                                                        oAFX = AFX;
                                                                        let AGB = if AFX != 0.0 {
                                                                            G
                                                                        } else {
                                                                            let AGA = (PK * RE) / (AJ * (I - AE));
                                                                            AGA
                                                                        };
                                                                        AFY = AGB;
                                                                        AFZ = AFW;
                                                                    }
                                                                    AFT = AFY;
                                                                    AFU = AFZ;
                                                                } else {
                                                                    AFT = G;
                                                                    AFU = G;
                                                                }
                                                                AFJ = AFT;
                                                                AFK = AFU;
                                                            }
                                                            AFE = AFJ;
                                                            AFF = AFK;
                                                        }
                                                        ADY = AFE;
                                                        ADZ = AFF;
                                                    }
                                                    ACS = ADY;
                                                    ACT = ADZ;
                                                }
                                                ABM = ACS;
                                                ABN = ACT;
                                            }
                                            AAF = ABM;
                                            AAG = ABN;
                                        }
                                        YE = AAF;
                                        YF = AAG;
                                    }
                                    WB = YE;
                                    WC = YF;
                                }
                                TY = WB;
                                TZ = WC;
                            }
                            RL = TY;
                            RM = TZ;
                        }
                        let RN = if RL <= G { 1.0 } else { 0.0 };
                        oRN = RN;
                        let AGD;
                        if RN != 0.0 {
                            AGD = RM;
                        } else {
                            let AGC = if RM <= G { 1.0 } else { 0.0 };
                            oAGC = AGC;
                            let AGG = if AGC != 0.0 {
                                RL
                            } else {
                                let AGF = (RL * RM) / (RL + RM);
                                AGF
                            };
                            AGD = AGG;
                        }
                        let AGE = if AGD == G { 1.0 } else { 0.0 };
                        oAGE = AGE;
                        PZ = QH;
                        QA = QI;
                        QB = RL;
                        QC = RM;
                        QD = AGD;
                        QE = QK;
                        QF = QL;
                    } else {
                        PZ = G;
                        QA = G;
                        QB = G;
                        QC = G;
                        QD = G;
                        QE = G;
                        QF = G;
                    }
                    PO = PZ;
                    PP = QA;
                    PQ = QB;
                    PR = QC;
                    PS = QD;
                    PT = QE;
                    PU = QF;
                }
                let AGJ;
                let AGK;
                let AGL;
                let AGM;
                let AGN;
                if PV != 0.0 {
                    let AGH = PK * parameters[4];
                    AGJ = AGH;
                    AGK = PO;
                    AGL = PT;
                    AGM = PP;
                    AGN = PU;
                } else {
                    let AGI = if (if PM > G { 1.0 } else { 0.0 }) != 0.0 && (if PK > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAGI = AGI;
                    let AGQ;
                    let AGR;
                    let AGS;
                    let AGT;
                    let AGU;
                    if AGI != 0.0 {
                        let AGP = if PW < PX { 1.0 } else { 0.0 };
                        oAGP = AGP;
                        let AGW;
                        let AGX;
                        let AGY;
                        let AGZ;
                        let AHA;
                        if AGP != 0.0 {
                            let AGV = if (I % AE) != G { 1.0 } else { 0.0 };
                            oAGV = AGV;
                            let AHE;
                            let AHF;
                            let AHG;
                            let AHH;
                            if AGV != 0.0 {
                                let AHC = AE * (if ((I - CI) / AE) >= G { ((I - CI) / AE) } else { G });
                                AHE = AHC;
                                AHF = AHC;
                                AHG = CI;
                                AHH = CI;
                            } else {
                                let AHD = if QO == CI { 1.0 } else { 0.0 };
                                oAHD = AHD;
                                let AHL;
                                let AHM;
                                let AHN;
                                let AHO;
                                if AHD != 0.0 {
                                    let AHJ = AE * (if ((I / AE) - CI) >= G { ((I / AE) - CI) } else { G });
                                    AHL = I;
                                    AHM = AHJ;
                                    AHN = G;
                                    AHO = AE;
                                } else {
                                    let AHK = AE * (if ((I / AE) - CI) >= G { ((I / AE) - CI) } else { G });
                                    AHL = AHK;
                                    AHM = I;
                                    AHN = AE;
                                    AHO = G;
                                }
                                AHE = AHL;
                                AHF = AHM;
                                AHG = AHN;
                                AHH = AHO;
                            }
                            let AHR;
                            if AHI != 0.0 {
                                let AHP = if AHE == G { 1.0 } else { 0.0 };
                                oAHP = AHP;
                                let AHT = if AHP != 0.0 {
                                    G
                                } else {
                                    let AHS = (PK * RE) / (AJ * AHE);
                                    AHS
                                };
                                AHR = AHT;
                            } else {
                                let AHQ = if AHF == G { 1.0 } else { 0.0 };
                                oAHQ = AHQ;
                                let AHV = if AHQ != 0.0 {
                                    G
                                } else {
                                    let AHU = (PK * RE) / (AJ * AHF);
                                    AHU
                                };
                                AHR = AHV;
                            }
                            AGW = AHG;
                            AGX = AHH;
                            AGY = AHR;
                            AGZ = AHE;
                            AHA = AHF;
                        } else {
                            AGW = PO;
                            AGX = PP;
                            AGY = PQ;
                            AGZ = PT;
                            AHA = PU;
                        }
                        let AHB = if PW == G { 1.0 } else { 0.0 };
                        oAHB = AHB;
                        let AHY;
                        let AHZ;
                        if AHB != 0.0 {
                            let AID;
                            if AHW != 0.0 {
                                let AIG;
                                if AIB != 0.0 {
                                    let AIE = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oAIE = AIE;
                                    let AIJ;
                                    if AIE != 0.0 {
                                        let AIH = if AGW == G { 1.0 } else { 0.0 };
                                        oAIH = AIH;
                                        let AIL = if AIH != 0.0 {
                                            G
                                        } else {
                                            let AIK = (PK * RE) / (AJ * AGW);
                                            AIK
                                        };
                                        AIJ = AIL;
                                    } else {
                                        let AII = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oAII = AII;
                                        let AIO;
                                        if AII != 0.0 {
                                            let AIM = RE + SE;
                                            let AIN = if AIM == G { 1.0 } else { 0.0 };
                                            oAIN = AIN;
                                            let AIP = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || AIN != 0.0 { 1.0 } else { 0.0 };
                                            oAIP = AIP;
                                            let AIR = if AIP != 0.0 {
                                                G
                                            } else {
                                                let AIQ = (PK * AJ) / ((RT * AGW) * AIM);
                                                AIQ
                                            };
                                            AIO = AIR;
                                        } else {
                                            AIO = G;
                                        }
                                        AIJ = AIO;
                                    }
                                    AIG = AIJ;
                                } else {
                                    let AIF = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oAIF = AIF;
                                    let AIU;
                                    if AIF != 0.0 {
                                        let AIS = if AGW == G { 1.0 } else { 0.0 };
                                        oAIS = AIS;
                                        let AIW = if AIS != 0.0 {
                                            G
                                        } else {
                                            let AIV = (PK * RE) / (AJ * AGW);
                                            AIV
                                        };
                                        AIU = AIW;
                                    } else {
                                        let AIT = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oAIT = AIT;
                                        let AIZ;
                                        if AIT != 0.0 {
                                            let AIX = RE + SE;
                                            let AIY = if AIX == G { 1.0 } else { 0.0 };
                                            oAIY = AIY;
                                            let AJA = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || AIY != 0.0 { 1.0 } else { 0.0 };
                                            oAJA = AJA;
                                            let AJC = if AJA != 0.0 {
                                                G
                                            } else {
                                                let AJB = (PK * AJ) / ((RT * AGW) * AIX);
                                                AJB
                                            };
                                            AIZ = AJC;
                                        } else {
                                            AIZ = G;
                                        }
                                        AIU = AIZ;
                                    }
                                    AIG = AIU;
                                }
                                AID = AIG;
                            } else {
                                let AJF;
                                if AIC != 0.0 {
                                    let AJD = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oAJD = AJD;
                                    let AJI;
                                    if AJD != 0.0 {
                                        let AJG = if AGX == G { 1.0 } else { 0.0 };
                                        oAJG = AJG;
                                        let AJK = if AJG != 0.0 {
                                            G
                                        } else {
                                            let AJJ = (PK * RE) / (AJ * AGX);
                                            AJJ
                                        };
                                        AJI = AJK;
                                    } else {
                                        let AJH = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oAJH = AJH;
                                        let AJN;
                                        if AJH != 0.0 {
                                            let AJL = RE + SE;
                                            let AJM = if AJL == G { 1.0 } else { 0.0 };
                                            oAJM = AJM;
                                            let AJO = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AJM != 0.0 { 1.0 } else { 0.0 };
                                            oAJO = AJO;
                                            let AJQ = if AJO != 0.0 {
                                                G
                                            } else {
                                                let AJP = (PK * AJ) / ((RT * AGX) * AJL);
                                                AJP
                                            };
                                            AJN = AJQ;
                                        } else {
                                            AJN = G;
                                        }
                                        AJI = AJN;
                                    }
                                    AJF = AJI;
                                } else {
                                    let AJE = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    oAJE = AJE;
                                    let AJT;
                                    if AJE != 0.0 {
                                        let AJR = if AGX == G { 1.0 } else { 0.0 };
                                        oAJR = AJR;
                                        let AJV = if AJR != 0.0 {
                                            G
                                        } else {
                                            let AJU = (PK * RE) / (AJ * AGX);
                                            AJU
                                        };
                                        AJT = AJV;
                                    } else {
                                        let AJS = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oAJS = AJS;
                                        let AJY;
                                        if AJS != 0.0 {
                                            let AJW = RE + SE;
                                            let AJX = if AJW == G { 1.0 } else { 0.0 };
                                            oAJX = AJX;
                                            let AJZ = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AJX != 0.0 { 1.0 } else { 0.0 };
                                            oAJZ = AJZ;
                                            let AKB = if AJZ != 0.0 {
                                                G
                                            } else {
                                                let AKA = (PK * AJ) / ((RT * AGX) * AJW);
                                                AKA
                                            };
                                            AJY = AKB;
                                        } else {
                                            AJY = G;
                                        }
                                        AJT = AJY;
                                    }
                                    AJF = AJT;
                                }
                                AID = AJF;
                            }
                            AHY = AGY;
                            AHZ = AID;
                        } else {
                            let AHX = if PW == CI { 1.0 } else { 0.0 };
                            oAHX = AHX;
                            let AKE;
                            let AKF;
                            if AHX != 0.0 {
                                let AKI;
                                if AKC != 0.0 {
                                    let AKL;
                                    if AKG != 0.0 {
                                        let AKJ = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oAKJ = AKJ;
                                        let AKO;
                                        if AKJ != 0.0 {
                                            let AKM = if AGW == G { 1.0 } else { 0.0 };
                                            oAKM = AKM;
                                            let AKQ = if AKM != 0.0 {
                                                G
                                            } else {
                                                let AKP = (PK * RE) / (AJ * AGW);
                                                AKP
                                            };
                                            AKO = AKQ;
                                        } else {
                                            let AKN = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oAKN = AKN;
                                            let AKT;
                                            if AKN != 0.0 {
                                                let AKR = RE + SE;
                                                let AKS = if AKR == G { 1.0 } else { 0.0 };
                                                oAKS = AKS;
                                                let AKU = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || AKS != 0.0 { 1.0 } else { 0.0 };
                                                oAKU = AKU;
                                                let AKW = if AKU != 0.0 {
                                                    G
                                                } else {
                                                    let AKV = (PK * AJ) / ((RT * AGW) * AKR);
                                                    AKV
                                                };
                                                AKT = AKW;
                                            } else {
                                                AKT = G;
                                            }
                                            AKO = AKT;
                                        }
                                        AKL = AKO;
                                    } else {
                                        let AKK = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oAKK = AKK;
                                        let AKZ;
                                        if AKK != 0.0 {
                                            let AKX = if AGW == G { 1.0 } else { 0.0 };
                                            oAKX = AKX;
                                            let ALB = if AKX != 0.0 {
                                                G
                                            } else {
                                                let ALA = (PK * RE) / (AJ * AGW);
                                                ALA
                                            };
                                            AKZ = ALB;
                                        } else {
                                            let AKY = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oAKY = AKY;
                                            let ALE;
                                            if AKY != 0.0 {
                                                let ALC = RE + SE;
                                                let ALD = if ALC == G { 1.0 } else { 0.0 };
                                                oALD = ALD;
                                                let ALF = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || ALD != 0.0 { 1.0 } else { 0.0 };
                                                oALF = ALF;
                                                let ALH = if ALF != 0.0 {
                                                    G
                                                } else {
                                                    let ALG = (PK * AJ) / ((RT * AGW) * ALC);
                                                    ALG
                                                };
                                                ALE = ALH;
                                            } else {
                                                ALE = G;
                                            }
                                            AKZ = ALE;
                                        }
                                        AKL = AKZ;
                                    }
                                    AKI = AKL;
                                } else {
                                    let ALK;
                                    if AKH != 0.0 {
                                        let ALI = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oALI = ALI;
                                        let ALN;
                                        if ALI != 0.0 {
                                            let ALL = if AGX == G { 1.0 } else { 0.0 };
                                            oALL = ALL;
                                            let ALP = if ALL != 0.0 {
                                                G
                                            } else {
                                                let ALO = (PK * RE) / (AJ * AGX);
                                                ALO
                                            };
                                            ALN = ALP;
                                        } else {
                                            let ALM = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oALM = ALM;
                                            let ALR;
                                            if ALM != 0.0 {
                                                let ALQ = if RE == G { 1.0 } else { 0.0 };
                                                oALQ = ALQ;
                                                let ALS = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || ALQ != 0.0 { 1.0 } else { 0.0 };
                                                oALS = ALS;
                                                let ALU = if ALS != 0.0 {
                                                    G
                                                } else {
                                                    let ALT = (PK * AJ) / ((RZ * AGX) * RE);
                                                    ALT
                                                };
                                                ALR = ALU;
                                            } else {
                                                ALR = G;
                                            }
                                            ALN = ALR;
                                        }
                                        ALK = ALN;
                                    } else {
                                        let ALJ = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        oALJ = ALJ;
                                        let ALX;
                                        if ALJ != 0.0 {
                                            let ALV = if AGX == G { 1.0 } else { 0.0 };
                                            oALV = ALV;
                                            let ALZ = if ALV != 0.0 {
                                                G
                                            } else {
                                                let ALY = (PK * RE) / (AJ * AGX);
                                                ALY
                                            };
                                            ALX = ALZ;
                                        } else {
                                            let ALW = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oALW = ALW;
                                            let AMB;
                                            if ALW != 0.0 {
                                                let AMA = if RE == G { 1.0 } else { 0.0 };
                                                oAMA = AMA;
                                                let AMC = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AMA != 0.0 { 1.0 } else { 0.0 };
                                                oAMC = AMC;
                                                let AME = if AMC != 0.0 {
                                                    G
                                                } else {
                                                    let AMD = (PK * AJ) / ((RZ * AGX) * RE);
                                                    AMD
                                                };
                                                AMB = AME;
                                            } else {
                                                AMB = G;
                                            }
                                            ALX = AMB;
                                        }
                                        ALK = ALX;
                                    }
                                    AKI = ALK;
                                }
                                AKE = AGY;
                                AKF = AKI;
                            } else {
                                let AKD = if PW == AE { 1.0 } else { 0.0 };
                                oAKD = AKD;
                                let AMH;
                                let AMI;
                                if AKD != 0.0 {
                                    let AML;
                                    if AMF != 0.0 {
                                        let AMO;
                                        if AMJ != 0.0 {
                                            let AMM = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oAMM = AMM;
                                            let AMR;
                                            if AMM != 0.0 {
                                                let AMP = if AGW == G { 1.0 } else { 0.0 };
                                                oAMP = AMP;
                                                let AMT = if AMP != 0.0 {
                                                    G
                                                } else {
                                                    let AMS = (PK * RE) / (AJ * AGW);
                                                    AMS
                                                };
                                                AMR = AMT;
                                            } else {
                                                let AMQ = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oAMQ = AMQ;
                                                let AMV;
                                                if AMQ != 0.0 {
                                                    let AMU = if RE == G { 1.0 } else { 0.0 };
                                                    oAMU = AMU;
                                                    let AMW = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || AMU != 0.0 { 1.0 } else { 0.0 };
                                                    oAMW = AMW;
                                                    let AMY = if AMW != 0.0 {
                                                        G
                                                    } else {
                                                        let AMX = (PK * AJ) / ((RZ * AGW) * RE);
                                                        AMX
                                                    };
                                                    AMV = AMY;
                                                } else {
                                                    AMV = G;
                                                }
                                                AMR = AMV;
                                            }
                                            AMO = AMR;
                                        } else {
                                            let AMN = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oAMN = AMN;
                                            let ANB;
                                            if AMN != 0.0 {
                                                let AMZ = if AGW == G { 1.0 } else { 0.0 };
                                                oAMZ = AMZ;
                                                let AND = if AMZ != 0.0 {
                                                    G
                                                } else {
                                                    let ANC = (PK * RE) / (AJ * AGW);
                                                    ANC
                                                };
                                                ANB = AND;
                                            } else {
                                                let ANA = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oANA = ANA;
                                                let ANF;
                                                if ANA != 0.0 {
                                                    let ANE = if RE == G { 1.0 } else { 0.0 };
                                                    oANE = ANE;
                                                    let ANG = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || ANE != 0.0 { 1.0 } else { 0.0 };
                                                    oANG = ANG;
                                                    let ANI = if ANG != 0.0 {
                                                        G
                                                    } else {
                                                        let ANH = (PK * AJ) / ((RZ * AGW) * RE);
                                                        ANH
                                                    };
                                                    ANF = ANI;
                                                } else {
                                                    ANF = G;
                                                }
                                                ANB = ANF;
                                            }
                                            AMO = ANB;
                                        }
                                        AML = AMO;
                                    } else {
                                        let ANL;
                                        if AMK != 0.0 {
                                            let ANJ = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oANJ = ANJ;
                                            let ANO;
                                            if ANJ != 0.0 {
                                                let ANM = if AGX == G { 1.0 } else { 0.0 };
                                                oANM = ANM;
                                                let ANQ = if ANM != 0.0 {
                                                    G
                                                } else {
                                                    let ANP = (PK * RE) / (AJ * AGX);
                                                    ANP
                                                };
                                                ANO = ANQ;
                                            } else {
                                                let ANN = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oANN = ANN;
                                                let ANT;
                                                if ANN != 0.0 {
                                                    let ANR = RE + SE;
                                                    let ANS = if ANR == G { 1.0 } else { 0.0 };
                                                    oANS = ANS;
                                                    let ANU = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || ANS != 0.0 { 1.0 } else { 0.0 };
                                                    oANU = ANU;
                                                    let ANW = if ANU != 0.0 {
                                                        G
                                                    } else {
                                                        let ANV = (PK * AJ) / ((RT * AGX) * ANR);
                                                        ANV
                                                    };
                                                    ANT = ANW;
                                                } else {
                                                    ANT = G;
                                                }
                                                ANO = ANT;
                                            }
                                            ANL = ANO;
                                        } else {
                                            let ANK = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            oANK = ANK;
                                            let ANZ;
                                            if ANK != 0.0 {
                                                let ANX = if AGX == G { 1.0 } else { 0.0 };
                                                oANX = ANX;
                                                let AOB = if ANX != 0.0 {
                                                    G
                                                } else {
                                                    let AOA = (PK * RE) / (AJ * AGX);
                                                    AOA
                                                };
                                                ANZ = AOB;
                                            } else {
                                                let ANY = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oANY = ANY;
                                                let AOE;
                                                if ANY != 0.0 {
                                                    let AOC = RE + SE;
                                                    let AOD = if AOC == G { 1.0 } else { 0.0 };
                                                    oAOD = AOD;
                                                    let AOF = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AOD != 0.0 { 1.0 } else { 0.0 };
                                                    oAOF = AOF;
                                                    let AOH = if AOF != 0.0 {
                                                        G
                                                    } else {
                                                        let AOG = (PK * AJ) / ((RT * AGX) * AOC);
                                                        AOG
                                                    };
                                                    AOE = AOH;
                                                } else {
                                                    AOE = G;
                                                }
                                                ANZ = AOE;
                                            }
                                            ANL = ANZ;
                                        }
                                        AML = ANL;
                                    }
                                    AMH = AGY;
                                    AMI = AML;
                                } else {
                                    let AMG = if PW == RT { 1.0 } else { 0.0 };
                                    oAMG = AMG;
                                    let AOK;
                                    let AOL;
                                    if AMG != 0.0 {
                                        let AOO;
                                        if AOI != 0.0 {
                                            let AOR;
                                            if AOM != 0.0 {
                                                let AOP = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oAOP = AOP;
                                                let AOU;
                                                if AOP != 0.0 {
                                                    let AOS = if AGW == G { 1.0 } else { 0.0 };
                                                    oAOS = AOS;
                                                    let AOW = if AOS != 0.0 {
                                                        G
                                                    } else {
                                                        let AOV = (PK * RE) / (AJ * AGW);
                                                        AOV
                                                    };
                                                    AOU = AOW;
                                                } else {
                                                    let AOT = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAOT = AOT;
                                                    let AOY;
                                                    if AOT != 0.0 {
                                                        let AOX = if RE == G { 1.0 } else { 0.0 };
                                                        oAOX = AOX;
                                                        let AOZ = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || AOX != 0.0 { 1.0 } else { 0.0 };
                                                        oAOZ = AOZ;
                                                        let APB = if AOZ != 0.0 {
                                                            G
                                                        } else {
                                                            let APA = (PK * AJ) / ((RZ * AGW) * RE);
                                                            APA
                                                        };
                                                        AOY = APB;
                                                    } else {
                                                        AOY = G;
                                                    }
                                                    AOU = AOY;
                                                }
                                                AOR = AOU;
                                            } else {
                                                let AOQ = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oAOQ = AOQ;
                                                let APE;
                                                if AOQ != 0.0 {
                                                    let APC = if AGW == G { 1.0 } else { 0.0 };
                                                    oAPC = APC;
                                                    let APG = if APC != 0.0 {
                                                        G
                                                    } else {
                                                        let APF = (PK * RE) / (AJ * AGW);
                                                        APF
                                                    };
                                                    APE = APG;
                                                } else {
                                                    let APD = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAPD = APD;
                                                    let API;
                                                    if APD != 0.0 {
                                                        let APH = if RE == G { 1.0 } else { 0.0 };
                                                        oAPH = APH;
                                                        let APJ = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || APH != 0.0 { 1.0 } else { 0.0 };
                                                        oAPJ = APJ;
                                                        let APL = if APJ != 0.0 {
                                                            G
                                                        } else {
                                                            let APK = (PK * AJ) / ((RZ * AGW) * RE);
                                                            APK
                                                        };
                                                        API = APL;
                                                    } else {
                                                        API = G;
                                                    }
                                                    APE = API;
                                                }
                                                AOR = APE;
                                            }
                                            AOO = AOR;
                                        } else {
                                            let APO;
                                            if AON != 0.0 {
                                                let APM = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oAPM = APM;
                                                let APR;
                                                if APM != 0.0 {
                                                    let APP = if AGX == G { 1.0 } else { 0.0 };
                                                    oAPP = APP;
                                                    let APT = if APP != 0.0 {
                                                        G
                                                    } else {
                                                        let APS = (PK * RE) / (AJ * AGX);
                                                        APS
                                                    };
                                                    APR = APT;
                                                } else {
                                                    let APQ = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAPQ = APQ;
                                                    let APV;
                                                    if APQ != 0.0 {
                                                        let APU = if RE == G { 1.0 } else { 0.0 };
                                                        oAPU = APU;
                                                        let APW = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || APU != 0.0 { 1.0 } else { 0.0 };
                                                        oAPW = APW;
                                                        let APY = if APW != 0.0 {
                                                            G
                                                        } else {
                                                            let APX = (PK * AJ) / ((RZ * AGX) * RE);
                                                            APX
                                                        };
                                                        APV = APY;
                                                    } else {
                                                        APV = G;
                                                    }
                                                    APR = APV;
                                                }
                                                APO = APR;
                                            } else {
                                                let APN = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                oAPN = APN;
                                                let AQB;
                                                if APN != 0.0 {
                                                    let APZ = if AGX == G { 1.0 } else { 0.0 };
                                                    oAPZ = APZ;
                                                    let AQD = if APZ != 0.0 {
                                                        G
                                                    } else {
                                                        let AQC = (PK * RE) / (AJ * AGX);
                                                        AQC
                                                    };
                                                    AQB = AQD;
                                                } else {
                                                    let AQA = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAQA = AQA;
                                                    let AQF;
                                                    if AQA != 0.0 {
                                                        let AQE = if RE == G { 1.0 } else { 0.0 };
                                                        oAQE = AQE;
                                                        let AQG = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AQE != 0.0 { 1.0 } else { 0.0 };
                                                        oAQG = AQG;
                                                        let AQI = if AQG != 0.0 {
                                                            G
                                                        } else {
                                                            let AQH = (PK * AJ) / ((RZ * AGX) * RE);
                                                            AQH
                                                        };
                                                        AQF = AQI;
                                                    } else {
                                                        AQF = G;
                                                    }
                                                    AQB = AQF;
                                                }
                                                APO = AQB;
                                            }
                                            AOO = APO;
                                        }
                                        AOK = AGY;
                                        AOL = AOO;
                                    } else {
                                        let AOJ = if PW == RY { 1.0 } else { 0.0 };
                                        oAOJ = AOJ;
                                        let AQL;
                                        let AQM;
                                        if AOJ != 0.0 {
                                            let AQP;
                                            if AQJ != 0.0 {
                                                let AQS;
                                                if AQN != 0.0 {
                                                    let AQQ = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAQQ = AQQ;
                                                    let AQV;
                                                    if AQQ != 0.0 {
                                                        let AQT = if AGW == G { 1.0 } else { 0.0 };
                                                        oAQT = AQT;
                                                        let AQX = if AQT != 0.0 {
                                                            G
                                                        } else {
                                                            let AQW = (PK * RE) / (AJ * AGW);
                                                            AQW
                                                        };
                                                        AQV = AQX;
                                                    } else {
                                                        let AQU = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oAQU = AQU;
                                                        let ARA;
                                                        if AQU != 0.0 {
                                                            let AQY = RE + SE;
                                                            let AQZ = if AQY == G { 1.0 } else { 0.0 };
                                                            oAQZ = AQZ;
                                                            let ARB = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || AQZ != 0.0 { 1.0 } else { 0.0 };
                                                            oARB = ARB;
                                                            let ARD = if ARB != 0.0 {
                                                                G
                                                            } else {
                                                                let ARC = (PK * AJ) / ((RT * AGW) * AQY);
                                                                ARC
                                                            };
                                                            ARA = ARD;
                                                        } else {
                                                            ARA = G;
                                                        }
                                                        AQV = ARA;
                                                    }
                                                    AQS = AQV;
                                                } else {
                                                    let AQR = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    oAQR = AQR;
                                                    let ARG;
                                                    if AQR != 0.0 {
                                                        let ARE = if AGW == G { 1.0 } else { 0.0 };
                                                        oARE = ARE;
                                                        let ARI = if ARE != 0.0 {
                                                            G
                                                        } else {
                                                            let ARH = (PK * RE) / (AJ * AGW);
                                                            ARH
                                                        };
                                                        ARG = ARI;
                                                    } else {
                                                        let ARF = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oARF = ARF;
                                                        let ARL;
                                                        if ARF != 0.0 {
                                                            let ARJ = RE + SE;
                                                            let ARK = if ARJ == G { 1.0 } else { 0.0 };
                                                            oARK = ARK;
                                                            let ARM = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || ARK != 0.0 { 1.0 } else { 0.0 };
                                                            oARM = ARM;
                                                            let ARO = if ARM != 0.0 {
                                                                G
                                                            } else {
                                                                let ARN = (PK * AJ) / ((RT * AGW) * ARJ);
                                                                ARN
                                                            };
                                                            ARL = ARO;
                                                        } else {
                                                            ARL = G;
                                                        }
                                                        ARG = ARL;
                                                    }
                                                    AQS = ARG;
                                                }
                                                AQP = AQS;
                                            } else {
                                                let AQO = (PK * AAI) / AJ;
                                                AQP = AQO;
                                            }
                                            AQL = AGY;
                                            AQM = AQP;
                                        } else {
                                            let AQK = if PW == RR { 1.0 } else { 0.0 };
                                            oAQK = AQK;
                                            let ARR;
                                            let ARS;
                                            if AQK != 0.0 {
                                                let ARV;
                                                if ARP != 0.0 {
                                                    let ARY;
                                                    if ART != 0.0 {
                                                        let ARW = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oARW = ARW;
                                                        let ASB;
                                                        if ARW != 0.0 {
                                                            let ARZ = if AGW == G { 1.0 } else { 0.0 };
                                                            oARZ = ARZ;
                                                            let ASD = if ARZ != 0.0 {
                                                                G
                                                            } else {
                                                                let ASC = (PK * RE) / (AJ * AGW);
                                                                ASC
                                                            };
                                                            ASB = ASD;
                                                        } else {
                                                            let ASA = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oASA = ASA;
                                                            let ASF;
                                                            if ASA != 0.0 {
                                                                let ASE = if RE == G { 1.0 } else { 0.0 };
                                                                oASE = ASE;
                                                                let ASG = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || ASE != 0.0 { 1.0 } else { 0.0 };
                                                                oASG = ASG;
                                                                let ASI = if ASG != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ASH = (PK * AJ) / ((RZ * AGW) * RE);
                                                                    ASH
                                                                };
                                                                ASF = ASI;
                                                            } else {
                                                                ASF = G;
                                                            }
                                                            ASB = ASF;
                                                        }
                                                        ARY = ASB;
                                                    } else {
                                                        let ARX = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        oARX = ARX;
                                                        let ASL;
                                                        if ARX != 0.0 {
                                                            let ASJ = if AGW == G { 1.0 } else { 0.0 };
                                                            oASJ = ASJ;
                                                            let ASN = if ASJ != 0.0 {
                                                                G
                                                            } else {
                                                                let ASM = (PK * RE) / (AJ * AGW);
                                                                ASM
                                                            };
                                                            ASL = ASN;
                                                        } else {
                                                            let ASK = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oASK = ASK;
                                                            let ASP;
                                                            if ASK != 0.0 {
                                                                let ASO = if RE == G { 1.0 } else { 0.0 };
                                                                oASO = ASO;
                                                                let ASQ = if (if AGW == G { 1.0 } else { 0.0 }) != 0.0 || ASO != 0.0 { 1.0 } else { 0.0 };
                                                                oASQ = ASQ;
                                                                let ASS = if ASQ != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ASR = (PK * AJ) / ((RZ * AGW) * RE);
                                                                    ASR
                                                                };
                                                                ASP = ASS;
                                                            } else {
                                                                ASP = G;
                                                            }
                                                            ASL = ASP;
                                                        }
                                                        ARY = ASL;
                                                    }
                                                    ARV = ARY;
                                                } else {
                                                    let ARU = if AGX == G { 1.0 } else { 0.0 };
                                                    oARU = ARU;
                                                    let ASU = if ARU != 0.0 {
                                                        G
                                                    } else {
                                                        let AST = (PK * AAI) / (AJ * AGX);
                                                        AST
                                                    };
                                                    ARV = ASU;
                                                }
                                                ARR = AGY;
                                                ARS = ARV;
                                            } else {
                                                let ARQ = if PW == RZ { 1.0 } else { 0.0 };
                                                oARQ = ARQ;
                                                let ASX;
                                                let ASY;
                                                if ARQ != 0.0 {
                                                    let ATB;
                                                    if ASV != 0.0 {
                                                        let ASZ = (PK * AAI) / AJ;
                                                        ATB = ASZ;
                                                    } else {
                                                        let ATE;
                                                        if ATA != 0.0 {
                                                            let ATC = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oATC = ATC;
                                                            let ATH;
                                                            if ATC != 0.0 {
                                                                let ATF = if AGX == G { 1.0 } else { 0.0 };
                                                                oATF = ATF;
                                                                let ATJ = if ATF != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ATI = (PK * RE) / (AJ * AGX);
                                                                    ATI
                                                                };
                                                                ATH = ATJ;
                                                            } else {
                                                                let ATG = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oATG = ATG;
                                                                let ATM;
                                                                if ATG != 0.0 {
                                                                    let ATK = RE + SE;
                                                                    let ATL = if ATK == G { 1.0 } else { 0.0 };
                                                                    oATL = ATL;
                                                                    let ATN = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || ATL != 0.0 { 1.0 } else { 0.0 };
                                                                    oATN = ATN;
                                                                    let ATP = if ATN != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let ATO = (PK * AJ) / ((RT * AGX) * ATK);
                                                                        ATO
                                                                    };
                                                                    ATM = ATP;
                                                                } else {
                                                                    ATM = G;
                                                                }
                                                                ATH = ATM;
                                                            }
                                                            ATE = ATH;
                                                        } else {
                                                            let ATD = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            oATD = ATD;
                                                            let ATS;
                                                            if ATD != 0.0 {
                                                                let ATQ = if AGX == G { 1.0 } else { 0.0 };
                                                                oATQ = ATQ;
                                                                let ATU = if ATQ != 0.0 {
                                                                    G
                                                                } else {
                                                                    let ATT = (PK * RE) / (AJ * AGX);
                                                                    ATT
                                                                };
                                                                ATS = ATU;
                                                            } else {
                                                                let ATR = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oATR = ATR;
                                                                let ATX;
                                                                if ATR != 0.0 {
                                                                    let ATV = RE + SE;
                                                                    let ATW = if ATV == G { 1.0 } else { 0.0 };
                                                                    oATW = ATW;
                                                                    let ATY = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || ATW != 0.0 { 1.0 } else { 0.0 };
                                                                    oATY = ATY;
                                                                    let AUA = if ATY != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let ATZ = (PK * AJ) / ((RT * AGX) * ATV);
                                                                        ATZ
                                                                    };
                                                                    ATX = AUA;
                                                                } else {
                                                                    ATX = G;
                                                                }
                                                                ATS = ATX;
                                                            }
                                                            ATE = ATS;
                                                        }
                                                        ATB = ATE;
                                                    }
                                                    ASX = AGY;
                                                    ASY = ATB;
                                                } else {
                                                    let ASW = if PW == RU { 1.0 } else { 0.0 };
                                                    oASW = ASW;
                                                    let AUD;
                                                    let AUE;
                                                    if ASW != 0.0 {
                                                        let AUH;
                                                        if AUB != 0.0 {
                                                            let AUF = if AGW == G { 1.0 } else { 0.0 };
                                                            oAUF = AUF;
                                                            let AUJ = if AUF != 0.0 {
                                                                G
                                                            } else {
                                                                let AUI = (PK * AAI) / (AJ * AGW);
                                                                AUI
                                                            };
                                                            AUH = AUJ;
                                                        } else {
                                                            let AUM;
                                                            if AUG != 0.0 {
                                                                let AUK = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oAUK = AUK;
                                                                let AUP;
                                                                if AUK != 0.0 {
                                                                    let AUN = if AGX == G { 1.0 } else { 0.0 };
                                                                    oAUN = AUN;
                                                                    let AUR = if AUN != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let AUQ = (PK * RE) / (AJ * AGX);
                                                                        AUQ
                                                                    };
                                                                    AUP = AUR;
                                                                } else {
                                                                    let AUO = if (if (if PM == RT { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                    oAUO = AUO;
                                                                    let AUT;
                                                                    if AUO != 0.0 {
                                                                        let AUS = if RE == G { 1.0 } else { 0.0 };
                                                                        oAUS = AUS;
                                                                        let AUU = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AUS != 0.0 { 1.0 } else { 0.0 };
                                                                        oAUU = AUU;
                                                                        let AUW = if AUU != 0.0 {
                                                                            G
                                                                        } else {
                                                                            let AUV = (PK * AJ) / ((RZ * AGX) * RE);
                                                                            AUV
                                                                        };
                                                                        AUT = AUW;
                                                                    } else {
                                                                        AUT = G;
                                                                    }
                                                                    AUP = AUT;
                                                                }
                                                                AUM = AUP;
                                                            } else {
                                                                let AUL = if (if (if PM == CI { 1.0 } else { 0.0 }) != 0.0 || (if PM == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                oAUL = AUL;
                                                                let AUZ;
                                                                if AUL != 0.0 {
                                                                    let AUX = if AGX == G { 1.0 } else { 0.0 };
                                                                    oAUX = AUX;
                                                                    let AVB = if AUX != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let AVA = (PK * RE) / (AJ * AGX);
                                                                        AVA
                                                                    };
                                                                    AUZ = AVB;
                                                                } else {
                                                                    let AUY = if (if (if PM == AE { 1.0 } else { 0.0 }) != 0.0 || (if PM == RY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PM == SM { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                    oAUY = AUY;
                                                                    let AVD;
                                                                    if AUY != 0.0 {
                                                                        let AVC = if RE == G { 1.0 } else { 0.0 };
                                                                        oAVC = AVC;
                                                                        let AVE = if (if AGX == G { 1.0 } else { 0.0 }) != 0.0 || AVC != 0.0 { 1.0 } else { 0.0 };
                                                                        oAVE = AVE;
                                                                        let AVG = if AVE != 0.0 {
                                                                            G
                                                                        } else {
                                                                            let AVF = (PK * AJ) / ((RZ * AGX) * RE);
                                                                            AVF
                                                                        };
                                                                        AVD = AVG;
                                                                    } else {
                                                                        AVD = G;
                                                                    }
                                                                    AUZ = AVD;
                                                                }
                                                                AUM = AUZ;
                                                            }
                                                            AUH = AUM;
                                                        }
                                                        AUD = AGY;
                                                        AUE = AUH;
                                                    } else {
                                                        let AUC = if PW == SM { 1.0 } else { 0.0 };
                                                        oAUC = AUC;
                                                        let AVJ;
                                                        let AVK;
                                                        if AUC != 0.0 {
                                                            let AVH = (PK * AAI) / AJ;
                                                            AVJ = AGY;
                                                            AVK = AVH;
                                                        } else {
                                                            let AVI = if PW == PX { 1.0 } else { 0.0 };
                                                            oAVI = AVI;
                                                            let AVN;
                                                            let AVO;
                                                            if AVI != 0.0 {
                                                                let AVS;
                                                                let AVT;
                                                                if AVL != 0.0 {
                                                                    let AVP = ((LB * PK) * RE) / AJ;
                                                                    let AVQ = if I == AE { 1.0 } else { 0.0 };
                                                                    oAVQ = AVQ;
                                                                    let AVV = if AVQ != 0.0 {
                                                                        G
                                                                    } else {
                                                                        let AVU = (PK * RE) / (AJ * (I - AE));
                                                                        AVU
                                                                    };
                                                                    AVS = AVV;
                                                                    AVT = AVP;
                                                                } else {
                                                                    let AVR = (PK * RE) / (AJ * I);
                                                                    AVS = AVR;
                                                                    AVT = G;
                                                                }
                                                                AVN = AVS;
                                                                AVO = AVT;
                                                            } else {
                                                                let AVM = if PW == AFH { 1.0 } else { 0.0 };
                                                                oAVM = AVM;
                                                                let AVX;
                                                                let AVY;
                                                                if AVM != 0.0 {
                                                                    let AWC;
                                                                    let AWD;
                                                                    if AVW != 0.0 {
                                                                        let AVZ = (PK * RE) / (AJ * I);
                                                                        AWC = AVZ;
                                                                        AWD = G;
                                                                    } else {
                                                                        let AWA = ((LB * PK) * RE) / AJ;
                                                                        let AWB = if I == AE { 1.0 } else { 0.0 };
                                                                        oAWB = AWB;
                                                                        let AWF = if AWB != 0.0 {
                                                                            G
                                                                        } else {
                                                                            let AWE = (PK * RE) / (AJ * (I - AE));
                                                                            AWE
                                                                        };
                                                                        AWC = AWF;
                                                                        AWD = AWA;
                                                                    }
                                                                    AVX = AWC;
                                                                    AVY = AWD;
                                                                } else {
                                                                    AVX = G;
                                                                    AVY = PR;
                                                                }
                                                                AVN = AVX;
                                                                AVO = AVY;
                                                            }
                                                            AVJ = AVN;
                                                            AVK = AVO;
                                                        }
                                                        AUD = AVJ;
                                                        AUE = AVK;
                                                    }
                                                    ASX = AUD;
                                                    ASY = AUE;
                                                }
                                                ARR = ASX;
                                                ARS = ASY;
                                            }
                                            AQL = ARR;
                                            AQM = ARS;
                                        }
                                        AOK = AQL;
                                        AOL = AQM;
                                    }
                                    AMH = AOK;
                                    AMI = AOL;
                                }
                                AKE = AMH;
                                AKF = AMI;
                            }
                            AHY = AKE;
                            AHZ = AKF;
                        }
                        let AIA = if AHY <= G { 1.0 } else { 0.0 };
                        oAIA = AIA;
                        let AWH;
                        if AIA != 0.0 {
                            AWH = AHZ;
                        } else {
                            let AWG = if AHZ <= G { 1.0 } else { 0.0 };
                            oAWG = AWG;
                            let AWK = if AWG != 0.0 {
                                AHY
                            } else {
                                let AWJ = (AHY * AHZ) / (AHY + AHZ);
                                AWJ
                            };
                            AWH = AWK;
                        }
                        let AWI = if AWH == G { 1.0 } else { 0.0 };
                        oAWI = AWI;
                        AGQ = AWH;
                        AGR = AGW;
                        AGS = AGZ;
                        AGT = AGX;
                        AGU = AHA;
                    } else {
                        AGQ = G;
                        AGR = PO;
                        AGS = PT;
                        AGT = PP;
                        AGU = PU;
                    }
                    AGJ = AGQ;
                    AGK = AGR;
                    AGL = AGS;
                    AGM = AGT;
                    AGN = AGU;
                }
                let AGO = if parameters[42] == G { 1.0 } else { 0.0 };
                let AWO;
                let AWP;
                if AGO != 0.0 {
                    let AWM = if PS < AWL { 1.0 } else { 0.0 };
                    oAWM = AWM;
                    let AWQ = if AWM != 0.0 {
                        G
                    } else {
                        PS
                    };
                    let AWR = if AGJ < AWL { 1.0 } else { 0.0 };
                    oAWR = AWR;
                    let AWS = if AWR != 0.0 {
                        G
                    } else {
                        AGJ
                    };
                    AWO = AWQ;
                    AWP = AWS;
                } else {
                    let AWN = if PS <= AWL { 1.0 } else { 0.0 };
                    oAWN = AWN;
                    let AWT = if AWN != 0.0 {
                        AWL
                    } else {
                        PS
                    };
                    let AWU = if AGJ <= AWL { 1.0 } else { 0.0 };
                    oAWU = AWU;
                    let AWV = if AWU != 0.0 {
                        AWL
                    } else {
                        AGJ
                    };
                    AWO = AWT;
                    AWP = AWV;
                }
                let AWY;
                let AWZ;
                let AXA;
                let AXB;
                let AXC;
                let AXD;
                if MV != 0.0 {
                    let AWW = if DY <= G { 1.0 } else { 0.0 };
                    oAWW = AWW;
                    let AXG = if AWW != 0.0 {
                        G
                    } else {
                        DY
                    };
                    let AXH = if DZ <= G { 1.0 } else { 0.0 };
                    oAXH = AXH;
                    let AXI = if AXH != 0.0 {
                        G
                    } else {
                        DZ
                    };
                    let AXJ = if MZ <= G { 1.0 } else { 0.0 };
                    oAXJ = AXJ;
                    let AXK = if AXJ != 0.0 {
                        G
                    } else {
                        MZ
                    };
                    let AXL = if NA <= G { 1.0 } else { 0.0 };
                    oAXL = AXL;
                    let AXM = if AXL != 0.0 {
                        G
                    } else {
                        NA
                    };
                    AWY = EB;
                    AWZ = NB;
                    AXA = AXG;
                    AXB = AXK;
                    AXC = AXI;
                    AXD = AXM;
                } else {
                    let AWX = if EB <= G { 1.0 } else { 0.0 };
                    oAWX = AWX;
                    let AXN = if AWX != 0.0 {
                        G
                    } else {
                        EB
                    };
                    let AXO = if NB <= G { 1.0 } else { 0.0 };
                    oAXO = AXO;
                    let AXP = if AXO != 0.0 {
                        G
                    } else {
                        NB
                    };
                    AWY = AXN;
                    AWZ = AXP;
                    AXA = DY;
                    AXB = MZ;
                    AXC = DZ;
                    AXD = NA;
                }
                let AXF = if AXE != G { 1.0 } else { 0.0 };
                let AXW;
                let AXX;
                let AXY;
                let AXZ;
                let AYA;
                if AXF != 0.0 {
                    let AXS = (if (AF * AXQ) >= AXR { (AF * AXQ) } else { AXR }).ln();
                    let AXT = (if (AJ * AXQ) >= AXR { (AJ * AXQ) } else { AXR }).ln();
                    let AXU = (if I >= AXR { I } else { AXR }).ln();
                    let AXV = if (if (if parameter_given[757] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if parameter_given[761] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAXV = AXV;
                    let AYD;
                    if AXV != 0.0 {
                        AYD = CI;
                    } else {
                        let AYC = if (if (if (if parameter_given[773] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[774] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if parameter_given[775] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[776] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oAYC = AYC;
                        let AYF = if AYC != 0.0 {
                            RT
                        } else {
                            RR
                        };
                        AYD = AYF;
                    }
                    let AYE = if AXE == AE { 1.0 } else { 0.0 };
                    oAYE = AYE;
                    let AYM;
                    let AYN;
                    let AYO;
                    let AYP;
                    let AYQ;
                    if AYE != 0.0 {
                        let AYG = if AYD == RR { 1.0 } else { 0.0 };
                        oAYG = AYG;
                        let AZA;
                        let AZB;
                        if AYG != 0.0 {
                            let AYS = rspice_limited_exp((((parameters[777] * AXS) + (parameters[778] * AXT)) + (parameters[779] * AXU)));
                            let AYT = parameters[773] * AYS;
                            let AYU = rspice_limited_exp((((parameters[780] * AXS) + (parameters[781] * AXT)) + (parameters[782] * AXU)));
                            let AYV = parameters[774] * AYU;
                            let AYW = (AYT * AYV) / (AYT + AYV);
                            let AYX = parameters[775] * AYS;
                            let AYY = parameters[776] * AYU;
                            let AYZ = (AYX * AYY) / (AYX + AYY);
                            AZA = AYZ;
                            AZB = AYW;
                        } else {
                            AZA = AYH;
                            AZB = AYK;
                        }
                        let AZC = if (if AYD == RT { 1.0 } else { 0.0 }) != 0.0 || AYG != 0.0 { 1.0 } else { 0.0 };
                        oAZC = AZC;
                        let AZF;
                        let AZG;
                        if AZC != 0.0 {
                            let AZD = parameters[757] * (rspice_limited_exp((((parameters[758] * AXS) + (parameters[759] * AXT)) + (parameters[760] * AXU))));
                            let AZE = parameters[761] * (rspice_limited_exp((((parameters[762] * AXS) + (parameters[763] * AXT)) + (parameters[764] * AXU))));
                            AZF = AZD;
                            AZG = AZE;
                        } else {
                            AZF = AYJ;
                            AZG = AYL;
                        }
                        let AZH = parameters[765] * (rspice_limited_exp((((parameters[766] * AXS) + (parameters[767] * AXT)) + (parameters[768] * AXU))));
                        let AZI = parameters[769] * (rspice_limited_exp((((parameters[770] * AXS) + (parameters[771] * AXT)) + (parameters[772] * AXU))));
                        let AZJ = (AZH * AZI) / (AZH + AZI);
                        AYM = AZA;
                        AYN = AZJ;
                        AYO = AZF;
                        AYP = AZB;
                        AYQ = AZG;
                    } else {
                        AYM = AYH;
                        AYN = AYI;
                        AYO = AYJ;
                        AYP = AYK;
                        AYQ = AYL;
                    }
                    let AYR = if (if AXE == CI { 1.0 } else { 0.0 }) != 0.0 || (if AYE != 0.0 && (if AYD == RR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAYR = AYR;
                    let AZN;
                    let AZO;
                    let AZP;
                    let AZQ;
                    let AZR;
                    if AYR != 0.0 {
                        let AZL = if AYM < AZK { 1.0 } else { 0.0 };
                        oAZL = AZL;
                        let AZV = if AZL != 0.0 {
                            AZS
                        } else {
                            let AZU = AZT + (CI / AYM);
                            AZU
                        };
                        let AZW = if AYN < AZK { 1.0 } else { 0.0 };
                        oAZW = AZW;
                        let AZY = if AZW != 0.0 {
                            AZS
                        } else {
                            let AZX = AZT + (CI / AYN);
                            AZX
                        };
                        let AZZ = if AYO < AZK { 1.0 } else { 0.0 };
                        oAZZ = AZZ;
                        let BAB = if AZZ != 0.0 {
                            AZS
                        } else {
                            let BAA = AZT + (CI / AYO);
                            BAA
                        };
                        let BAC = if AYP < AZK { 1.0 } else { 0.0 };
                        oBAC = BAC;
                        let BAE = if BAC != 0.0 {
                            AZS
                        } else {
                            let BAD = AZT + (CI / AYP);
                            BAD
                        };
                        let BAF = if AYQ < AZK { 1.0 } else { 0.0 };
                        oBAF = BAF;
                        let BAH = if BAF != 0.0 {
                            AZS
                        } else {
                            let BAG = AZT + (CI / AYQ);
                            BAG
                        };
                        AZN = BAB;
                        AZO = BAE;
                        AZP = AZY;
                        AZQ = AZV;
                        AZR = BAH;
                    } else {
                        let AZM = if AYE != 0.0 && (if AYD == RT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oAZM = AZM;
                        let BAK;
                        let BAL;
                        let BAM;
                        let BAN;
                        let BAO;
                        if AZM != 0.0 {
                            let BAI = if AYN < AZK { 1.0 } else { 0.0 };
                            oBAI = BAI;
                            let BAQ = if BAI != 0.0 {
                                AZS
                            } else {
                                let BAP = AZT + (CI / AYN);
                                BAP
                            };
                            let BAR = if AYO < AZK { 1.0 } else { 0.0 };
                            oBAR = BAR;
                            let BAT = if BAR != 0.0 {
                                AZS
                            } else {
                                let BAS = AZT + (CI / AYO);
                                BAS
                            };
                            let BAU = if AYQ < AZK { 1.0 } else { 0.0 };
                            oBAU = BAU;
                            let BAW = if BAU != 0.0 {
                                AZS
                            } else {
                                let BAV = AZT + (CI / AYQ);
                                BAV
                            };
                            BAK = BAT;
                            BAL = AZT;
                            BAM = BAQ;
                            BAN = AZT;
                            BAO = BAW;
                        } else {
                            let BAJ = if AYE != 0.0 && (if AYD == CI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            oBAJ = BAJ;
                            let BAY;
                            let BAZ;
                            let BBA;
                            let BBB;
                            let BBC;
                            if BAJ != 0.0 {
                                let BAX = if AYN < AZK { 1.0 } else { 0.0 };
                                oBAX = BAX;
                                let BBE = if BAX != 0.0 {
                                    AZS
                                } else {
                                    let BBD = AZT + (CI / AYN);
                                    BBD
                                };
                                BAY = AZS;
                                BAZ = AZT;
                                BBA = BBE;
                                BBB = AZT;
                                BBC = AZS;
                            } else {
                                BAY = G;
                                BAZ = G;
                                BBA = G;
                                BBB = G;
                                BBC = G;
                            }
                            BAK = BAY;
                            BAL = BAZ;
                            BAM = BBA;
                            BAN = BBB;
                            BAO = BBC;
                        }
                        AZN = BAK;
                        AZO = BAL;
                        AZP = BAM;
                        AZQ = BAN;
                        AZR = BAO;
                    }
                    AXW = AZN;
                    AXX = AZO;
                    AXY = AZP;
                    AXZ = AZQ;
                    AYA = AZR;
                } else {
                    AXW = G;
                    AXX = G;
                    AXY = G;
                    AXZ = G;
                    AYA = G;
                }
                let BBH;
                if AYB != 0.0 {
                    let BBG = if BBF < AZK { 1.0 } else { 0.0 };
                    oBBG = BBG;
                    let BBM = if BBG != 0.0 {
                        AZS
                    } else {
                        let BBL = AZT + (CI / BBF);
                        BBL
                    };
                    BBH = BBM;
                } else {
                    BBH = G;
                }
                let BBJ = (parameters[700] * (parameters[31] + ((AY / RT) / BBI))) / ((BBI * I) * (F - parameters[699]));
                let BBK = if BBJ > G { 1.0 } else { 0.0 };
                let BBQ;
                if BBK != 0.0 {
                    let BBN = CI / BBJ;
                    BBQ = BBN;
                } else {
                    let BBP = if BBO != G { 1.0 } else { 0.0 };
                    oBBP = BBP;
                    BBQ = AZS;
                }
                let BBR = parameters[77] * GK;
                let BBS = (rspice_limited_exp((GN * staged[55]))) / staged[56];
                let BBT = (rspice_limited_exp((GN * ((if (parameters[555] / BBR) >= AXR { (parameters[555] / BBR) } else { AXR }).ln())))) / (BBR * BBR);
                let BBV = (BBU * AJ) * BBT;
                let BBW = staged[58] * GK;
                let BBX = BBU * ((AJ * AF) * BBS);
                let BBY = parameters[911] + AJ;
                let BCA = if BBZ != 0.0 && (if BBY > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BCD;
                let BCE;
                if BCA != 0.0 {
                    let BCB = (BBY * I) / parameters[909];
                    let BCC = (parameters[910] * BBY) * I;
                    BCD = BCB;
                    BCE = BCC;
                } else {
                    BCD = CI;
                    BCE = G;
                }
                if BCA != 0.0 {
                    let BCF = HA * CQ;
                    oBCF = BCF;
                } else {
                    let BCG = HA * CQ;
                    oBCG = BCG;
                }
                let BCH = if CT > G { 1.0 } else { 0.0 };
                if BCH != 0.0 {
                    let BCJ = -BCI;
                    oBCJ = BCJ;
                    let BCK = (if (CT / CQ) >= AXR { (CT / CQ) } else { AXR }).ln();
                    oBCK = BCK;
                } else {
                }
                let BCN = (BCM / (BCL * JE)).sqrt();
                let BCO = (staged[71] * DG).sqrt();
                let BCQ = MQ - BCP;
                let BCR = MS - BCP;
                let BCS = FL - BCP;
                let BCT = FM - BCP;
                let BCU = -MT;
                let BCV = BCU - BCP;
                let BCX = CI / LC;
                let BCY = FP - BCP;
                let BCZ = ((parameters[741] / AY).sqrt()) + CI;
                let BDA = parameters[739] * BCZ;
                let BDB = parameters[740] * BCZ;
                let BDC = if PW < PX { 1.0 } else { 0.0 };
                let BDE;
                let BDF;
                let BDG;
                let BDH;
                if BDC != 0.0 {
                    let BDD = if (I % AE) != G { 1.0 } else { 0.0 };
                    oBDD = BDD;
                    let BDP;
                    let BDQ;
                    let BDR;
                    let BDS;
                    if BDD != 0.0 {
                        let BDN = AE * (if ((I - CI) / AE) >= G { ((I - CI) / AE) } else { G });
                        BDP = CI;
                        BDQ = BDN;
                        BDR = CI;
                        BDS = BDN;
                    } else {
                        let BDO = if QO == CI { 1.0 } else { 0.0 };
                        oBDO = BDO;
                        let BDV;
                        let BDW;
                        let BDX;
                        let BDY;
                        if BDO != 0.0 {
                            let BDT = AE * (if ((I / AE) - CI) >= G { ((I / AE) - CI) } else { G });
                            BDV = G;
                            BDW = I;
                            BDX = AE;
                            BDY = BDT;
                        } else {
                            let BDU = AE * (if ((I / AE) - CI) >= G { ((I / AE) - CI) } else { G });
                            BDV = AE;
                            BDW = BDU;
                            BDX = G;
                            BDY = I;
                        }
                        BDP = BDV;
                        BDQ = BDW;
                        BDR = BDX;
                        BDS = BDY;
                    }
                    BDE = BDP;
                    BDF = BDQ;
                    BDG = BDR;
                    BDH = BDS;
                } else {
                    BDE = AGK;
                    BDF = AGL;
                    BDG = AGM;
                    BDH = AGN;
                }
                let BDI = staged[132] + AY;
                let BDJ = staged[133] * AY;
                let BDK = RE * AY;
                let BDL = AAI * AY;
                let BDM = if PW == G { 1.0 } else { 0.0 };
                let BEF;
                let BEG;
                let BEH;
                let BEI;
                if BDM != 0.0 {
                    let BEA = (BDE * BDI) + (BDF * BDZ);
                    let BEB = (BDG * BDI) + (BDH * BDZ);
                    let BEC = (BDE * BDJ) + (BDF * BDK);
                    let BED = (BDG * BDJ) + (BDH * BDK);
                    BEF = BEC;
                    BEG = BED;
                    BEH = BEA;
                    BEI = BEB;
                } else {
                    let BEE = if PW == CI { 1.0 } else { 0.0 };
                    oBEE = BEE;
                    let BEQ;
                    let BER;
                    let BES;
                    let BET;
                    if BEE != 0.0 {
                        let BEK = (BDE * BDI) + (BDF * BDZ);
                        let BEL = BDG + BDH;
                        let BEM = BEL * BDZ;
                        let BEN = (BDE * BDJ) + (BDF * BDK);
                        let BEO = BEL * BDK;
                        BEQ = BEN;
                        BER = BEO;
                        BES = BEK;
                        BET = BEM;
                    } else {
                        let BEP = if PW == AE { 1.0 } else { 0.0 };
                        oBEP = BEP;
                        let BFA;
                        let BFB;
                        let BFC;
                        let BFD;
                        if BEP != 0.0 {
                            let BEU = BDE + BDF;
                            let BEV = BEU * BDZ;
                            let BEW = (BDG * BDI) + (BDH * BDZ);
                            let BEX = BEU * BDK;
                            let BEY = (BDG * BDJ) + (BDH * BDK);
                            BFA = BEX;
                            BFB = BEY;
                            BFC = BEV;
                            BFD = BEW;
                        } else {
                            let BEZ = if PW == RT { 1.0 } else { 0.0 };
                            oBEZ = BEZ;
                            let BFL;
                            let BFM;
                            let BFN;
                            let BFO;
                            if BEZ != 0.0 {
                                let BFE = BDE + BDF;
                                let BFF = BFE * BDZ;
                                let BFG = BDG + BDH;
                                let BFH = BFG * BDZ;
                                let BFI = BFE * BDK;
                                let BFJ = BFG * BDK;
                                BFL = BFI;
                                BFM = BFJ;
                                BFN = BFF;
                                BFO = BFH;
                            } else {
                                let BFK = if PW == RY { 1.0 } else { 0.0 };
                                oBFK = BFK;
                                let BFV;
                                let BFW;
                                let BFX;
                                let BFY;
                                if BFK != 0.0 {
                                    let BFP = (BDE * BDI) + (BDF * BDZ);
                                    let BFR = (BDG * BFQ) + (BDH * BDZ);
                                    let BFS = (BDE * BDJ) + (BDF * BDK);
                                    let BFT = (BDG * BDL) + (BDH * BDK);
                                    BFV = BFS;
                                    BFW = BFT;
                                    BFX = BFP;
                                    BFY = BFR;
                                } else {
                                    let BFU = if PW == RR { 1.0 } else { 0.0 };
                                    oBFU = BFU;
                                    let BGF;
                                    let BGG;
                                    let BGH;
                                    let BGI;
                                    if BFU != 0.0 {
                                        let BFZ = BDE + BDF;
                                        let BGA = BFZ * BDZ;
                                        let BGB = (BDG * BFQ) + (BDH * BDZ);
                                        let BGC = BFZ * BDK;
                                        let BGD = (BDG * BDL) + (BDH * BDK);
                                        BGF = BGC;
                                        BGG = BGD;
                                        BGH = BGA;
                                        BGI = BGB;
                                    } else {
                                        let BGE = if PW == RZ { 1.0 } else { 0.0 };
                                        oBGE = BGE;
                                        let BGO;
                                        let BGP;
                                        let BGQ;
                                        let BGR;
                                        if BGE != 0.0 {
                                            let BGJ = (BDE * BFQ) + (BDF * BDZ);
                                            let BGK = (BDG * BDI) + (BDH * BDZ);
                                            let BGL = (BDE * BDL) + (BDF * BDK);
                                            let BGM = (BDG * BDJ) + (BDH * BDK);
                                            BGO = BGL;
                                            BGP = BGM;
                                            BGQ = BGJ;
                                            BGR = BGK;
                                        } else {
                                            let BGN = if PW == RU { 1.0 } else { 0.0 };
                                            oBGN = BGN;
                                            let BGY;
                                            let BGZ;
                                            let BHA;
                                            let BHB;
                                            if BGN != 0.0 {
                                                let BGS = (BDE * BFQ) + (BDF * BDZ);
                                                let BGT = BDG + BDH;
                                                let BGU = BGT * BDZ;
                                                let BGV = (BDE * BDL) + (BDF * BDK);
                                                let BGW = BGT * BDK;
                                                BGY = BGV;
                                                BGZ = BGW;
                                                BHA = BGS;
                                                BHB = BGU;
                                            } else {
                                                let BGX = if PW == SM { 1.0 } else { 0.0 };
                                                oBGX = BGX;
                                                let BHH;
                                                let BHI;
                                                let BHJ;
                                                let BHK;
                                                if BGX != 0.0 {
                                                    let BHC = (BDE * BFQ) + (BDF * BDZ);
                                                    let BHD = (BDG * BFQ) + (BDH * BDZ);
                                                    let BHE = (BDE * BDL) + (BDF * BDK);
                                                    let BHF = (BDG * BDL) + (BDH * BDK);
                                                    BHH = BHE;
                                                    BHI = BHF;
                                                    BHJ = BHC;
                                                    BHK = BHD;
                                                } else {
                                                    let BHG = if PW == PX { 1.0 } else { 0.0 };
                                                    oBHG = BHG;
                                                    let BHR;
                                                    let BHS;
                                                    let BHT;
                                                    let BHU;
                                                    if BHG != 0.0 {
                                                        let BHL = I - CI;
                                                        let BHM = BDI + (BHL * BDZ);
                                                        let BHN = I * BDZ;
                                                        let BHO = BDJ + (BHL * BDK);
                                                        let BHP = I * BDK;
                                                        BHR = BHO;
                                                        BHS = BHP;
                                                        BHT = BHM;
                                                        BHU = BHN;
                                                    } else {
                                                        let BHQ = if PW == AFH { 1.0 } else { 0.0 };
                                                        oBHQ = BHQ;
                                                        let BIA;
                                                        let BIB;
                                                        let BIC;
                                                        let BID;
                                                        if BHQ != 0.0 {
                                                            let BHV = I * BDZ;
                                                            let BHW = I - CI;
                                                            let BHX = BDI + (BHW * BDZ);
                                                            let BHY = I * BDK;
                                                            let BHZ = BDJ + (BHW * BDK);
                                                            BIA = BHY;
                                                            BIB = BHZ;
                                                            BIC = BHV;
                                                            BID = BHX;
                                                        } else {
                                                            BIA = G;
                                                            BIB = G;
                                                            BIC = G;
                                                            BID = G;
                                                        }
                                                        BHR = BIA;
                                                        BHS = BIB;
                                                        BHT = BIC;
                                                        BHU = BID;
                                                    }
                                                    BHH = BHR;
                                                    BHI = BHS;
                                                    BHJ = BHT;
                                                    BHK = BHU;
                                                }
                                                BGY = BHH;
                                                BGZ = BHI;
                                                BHA = BHJ;
                                                BHB = BHK;
                                            }
                                            BGO = BGY;
                                            BGP = BGZ;
                                            BGQ = BHA;
                                            BGR = BHB;
                                        }
                                        BGF = BGO;
                                        BGG = BGP;
                                        BGH = BGQ;
                                        BGI = BGR;
                                    }
                                    BFV = BGF;
                                    BFW = BGG;
                                    BFX = BGH;
                                    BFY = BGI;
                                }
                                BFL = BFV;
                                BFM = BFW;
                                BFN = BFX;
                                BFO = BFY;
                            }
                            BFA = BFL;
                            BFB = BFM;
                            BFC = BFN;
                            BFD = BFO;
                        }
                        BEQ = BFA;
                        BER = BFB;
                        BES = BFC;
                        BET = BFD;
                    }
                    BEF = BEQ;
                    BEG = BER;
                    BEH = BES;
                    BEI = BET;
                }
                let BIF = if BEJ != 0.0 {
                    let BIE = (parameters[24] * D) * A;
                    BIE
                } else {
                    BEF
                };
                let BIG = if BIF < G { 1.0 } else { 0.0 };
                let BIH = if BIG != 0.0 {
                    G
                } else {
                    BIF
                };
                let BIK = if BII != 0.0 {
                    let BIJ = (parameters[25] * D) * A;
                    BIJ
                } else {
                    BEG
                };
                let BIL = if BIK < G { 1.0 } else { 0.0 };
                let BIM = if BIL != 0.0 {
                    G
                } else {
                    BIK
                };
                let BIR;
                if BIN != 0.0 {
                    let BIP = if BIO == G { 1.0 } else { 0.0 };
                    oBIP = BIP;
                    let BIW = if BIP != 0.0 {
                        let BIU = BIT * D;
                        BIU
                    } else {
                        let BIV = if ((BIT * D) - (AY * I)) >= G { ((BIT * D) - (AY * I)) } else { G };
                        BIV
                    };
                    BIR = BIW;
                } else {
                    let BIQ = if BEH < G { 1.0 } else { 0.0 };
                    oBIQ = BIQ;
                    let BIX = if BIQ != 0.0 {
                        G
                    } else {
                        BEH
                    };
                    BIR = BIX;
                }
                let BJA;
                if BIS != 0.0 {
                    let BIY = if BIO == G { 1.0 } else { 0.0 };
                    oBIY = BIY;
                    let BJF = if BIY != 0.0 {
                        let BJD = BJC * D;
                        BJD
                    } else {
                        let BJE = if ((BJC * D) - (AY * I)) >= G { ((BJC * D) - (AY * I)) } else { G };
                        BJE
                    };
                    BJA = BJF;
                } else {
                    let BIZ = if BEI < G { 1.0 } else { 0.0 };
                    oBIZ = BIZ;
                    let BJG = if BIZ != 0.0 {
                        G
                    } else {
                        BEI
                    };
                    BJA = BJG;
                }
                let BJB = AY * I;
                let BJK = if (if (if BJH > G { 1.0 } else { 0.0 }) != 0.0 && (if BJI > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if I == CI { 1.0 } else { 0.0 }) != 0.0 || (if (if I > CI { 1.0 } else { 0.0 }) != 0.0 && (if BJJ > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BJS;
                let BJT;
                let BJU;
                let BJV;
                let BJW;
                if BJK != 0.0 {
                    let BJL = F.powf(parameters[921]);
                    let BJM = J + parameters[914];
                    let BJN = BJM.powf(parameters[922]);
                    let BJO = CI + (((parameters[918] / BJL) + (parameters[919] / BJN)) + (parameters[920] / (BJL * BJN)));
                    oBJO = BJO;
                    let BJP = F.powf(parameters[927]);
                    let BJQ = BJM.powf(parameters[928]);
                    let BJR = CI + (((parameters[924] / BJP) + (parameters[925] / BJQ)) + (parameters[926] / (BJP * BJQ)));
                    let mut BJY = 0.0;
                    let mut BJZ = 0.0;
                    let mut BKA = 0.0;
                    BJY = G;
                    BJZ = G;
                    BKA = G;
                    loop {
                        let BKB = if BJY < I { 1.0 } else { 0.0 };
                        oBKB = BKB;
                        if BKB == 0.0 {
                            break;
                        }
                        let BKC = CI / I;
                        let BKD = LB * B;
                        let BKE = BJY * (BJJ + B);
                        let BKF = BJZ + (BKC / ((BJH + BKD) + BKE));
                        let BKG = BKA + (BKC / ((BJI + BKD) + BKE));
                        let BKH = BJY + CI;
                        BJY = BKH;
                        BJZ = BKF;
                        BKA = BKG;
                    }
                    let BKI = LB * B;
                    let BKJ = (CI / (parameters[912] + BKI)) + (CI / (parameters[913] + BKI));
                    oBKJ = BKJ;
                    let BKK = BJZ + BKA;
                    oBKK = BKK;
                    let BKL = BKK - BKJ;
                    let BKM = (parameters[923] / BJR) * BKL;
                    let BKN = BJR.powf(parameters[930]);
                    let BKO = BJR.powf(parameters[932]);
                    let BKP = (parameters[931] / BKO) * BKL;
                    oBKP = BKP;
                    let BKQ = MO + ((parameters[929] / BKN) * BKL);
                    let BKS = if BKR == CI { 1.0 } else { 0.0 };
                    oBKS = BKS;
                    let BKW;
                    let BKX;
                    let BKY;
                    if BKS != 0.0 {
                        let BKT = (HN / BJR) * BKL;
                        let BKU = (HQ / BKN) * BKL;
                        let BKV = (HR / BKO) * BKL;
                        BKW = BKU;
                        BKX = BKV;
                        BKY = BKT;
                    } else {
                        BKW = G;
                        BKX = G;
                        BKY = G;
                    }
                    let BKZ = HM + BKW;
                    let BLA = HE + BKX;
                    BJS = BKQ;
                    BJT = BKM;
                    BJU = BLA;
                    BJV = BKY;
                    BJW = BKZ;
                } else {
                    BJS = MO;
                    BJT = G;
                    BJU = HE;
                    BJV = G;
                    BJW = HM;
                }
                let BLD;
                let BLE;
                let BLF;
                if BJX != 0.0 {
                    let BLB = C / I;
                    let BLC = if (if (if (if parameter_given[20] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[21] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[22] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBLC = BLC;
                    let BLR;
                    let BLS;
                    let BLT;
                    if BLC != 0.0 {
                        let BLN = if (if parameter_given[23] { 1.0 } else { 0.0 }) != 0.0 && (if BLM > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBLN = BLN;
                        let BMF;
                        let BMG;
                        let BMH;
                        if BLN != 0.0 {
                            let BLU = BLM + BLB;
                            let BLW = CI / BLV;
                            let BLX = (BLV * BLV) / (BLM * BLU);
                            let BMA = BLZ * BLV;
                            let BMB = ((((BLY * BLM) + BMA) * (rspice_limited_exp(((-1e1f64 * BLM) * BLW)))) - (((BLY * BLU) + BMA) * (rspice_limited_exp(((-1e1f64 * BLU) * BLW))))) / BLB;
                            let BMD = 2.5e-3f64 * BLV;
                            let BME = ((((BMC * BLM) + BMD) * (rspice_limited_exp(((-2e1f64 * BLM) * BLW)))) - (((BMC * BLU) + BMD) * (rspice_limited_exp(((-2e1f64 * BLU) * BLW))))) / BLB;
                            BMF = BLX;
                            BMG = BMB;
                            BMH = BME;
                        } else {
                            BMF = BLO;
                            BMG = BLP;
                            BMH = BLQ;
                        }
                        BLR = BMF;
                        BLS = BMG;
                        BLT = BMH;
                    } else {
                        BLR = BLO;
                        BLS = BLP;
                        BLT = BLQ;
                    }
                    BLD = BLR;
                    BLE = BLS;
                    BLF = BLT;
                } else {
                    BLD = G;
                    BLE = G;
                    BLF = G;
                }
                let BLG = (BLD + (parameters[933] * BLE)) + (parameters[934] * BLF);
                let BLH = EV * BLG;
                let BLI = HP * BLG;
                let BLJ = HO * BLG;
                let BLK = CI + (EX * BLG);
                let BLL = BJS + (EW * BLG);
                let BMI = GO + (parameters[869] / AF);
                let BMJ = if CY > G { 1.0 } else { 0.0 };
                if BMJ != 0.0 {
                    let BMK = -CZ;
                    oBMK = BMK;
                } else {
                }
                let BML = DD + (DA / (AF.powf(DB)));
                let BMN = MJ + BMM;
                let BMO = staged[178] * JE;
                let BMP = staged[180] * JE;
                let BMQ = CI / (((AJ * AXQ).powf(DX)) * I);
                let BMR = if EK > G { 1.0 } else { 0.0 };
                let BMS = if LD <= G { 1.0 } else { 0.0 };
                if BMS != 0.0 {
                } else {
                    let BMT = LD * (AF.sqrt());
                    oBMT = BMT;
                }
                let BMU = if EH > G { 1.0 } else { 0.0 };
                if BMU != 0.0 {
                    let BMV = CI + (parameters[369] * AF);
                    oBMV = BMV;
                } else {
                }
                let BMW = if EG > G { 1.0 } else { 0.0 };
                if BMW != 0.0 {
                    let BMX = EF * BCO;
                    oBMX = BMX;
                    let BMY = BMX / 8e1f64;
                    oBMY = BMY;
                } else {
                }
                if MV != 0.0 {
                } else {
                    let BNC;
                    let BND;
                    if BMZ != 0.0 {
                        BNC = G;
                        BND = G;
                    } else {
                        BNC = AWP;
                        BND = AWO;
                    }
                    oBNC = BNC;
                    oBND = BND;
                }
                let BNA = AE * I;
                if BNB != 0.0 {
                    let BNE = JE * parameters[1117];
                    oBNE = BNE;
                    let BNG = (I * AJ) * BCL;
                    oBNG = BNG;
                    if BNH != 0.0 {
                        let BNJ = if staged[212] != 0.0 && (if AXE != AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBNJ = BNJ;
                    } else {
                    }
                    if BNI != 0.0 {
                        let BNK = if staged[216] != 0.0 && (if AXE != AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBNK = BNK;
                    } else {
                    }
                } else {
                }
                if BNF != 0.0 {
                    let BNM = I * AT;
                    let BNN = (((BNM * parameters[1115]) * 8.85418e-12f64) * parameters[111]) / staged[236];
                    oBNN = BNN;
                    let BNO = (BNM * parameters[1116]) * AE;
                    oBNO = BNO;
                } else {
                }
                let BNL = if BBO > CI { 1.0 } else { 0.0 };
                let BNR;
                if BNL != 0.0 {
                    let BNP = parameters[754] * I;
                    oBNP = BNP;
                    let BNQ = if BBO == AE { 1.0 } else { 0.0 };
                    oBNQ = BNQ;
                    let BNU;
                    if BNQ != 0.0 {
                        let BNT = if (CI / BBQ) < AWL { 1.0 } else { 0.0 };
                        oBNT = BNT;
                        let BNW = if BNT != 0.0 {
                            let BNV = CI / AWL;
                            BNV
                        } else {
                            BBQ
                        };
                        oBNW = BNW;
                        BNU = BNW;
                    } else {
                        BNU = BBQ;
                    }
                    BNR = BNU;
                } else {
                    BNR = BBQ;
                }
                if BNS != 0.0 {
                } else {
                    if BCW != 0.0 {
                        let BNY = MB - BCP;
                        oBNY = BNY;
                    } else {
                    }
                }
                if BNX != 0.0 {
                    let BOA = (I * AJ) * BCL;
                    oBOA = BOA;
                } else {
                }
                if BNZ != 0.0 {
                    if NW != 0.0 {
                        let BOE = (I * AJ) * AF;
                        let BOF = (BOE * 4.97232e-7f64) * BBS;
                        oBOF = BOF;
                        let BOG = (BOE * 3.75956e-7f64) * BBS;
                        oBOG = BOG;
                    } else {
                    }
                    if NY != 0.0 {
                        let BOH = I * BBX;
                        oBOH = BOH;
                        let BOJ;
                        if BOI != 0.0 {
                            let BOK = if GG < BLZ { 1.0 } else { 0.0 };
                            oBOK = BOK;
                            let BOL = if BOK != 0.0 {
                                BLZ
                            } else {
                                GG
                            };
                            BOJ = BOL;
                        } else {
                            BOJ = GG;
                        }
                        oBOJ = BOJ;
                        let BOM;
                        if BOI != 0.0 {
                            let BON = if GJ < BLZ { 1.0 } else { 0.0 };
                            oBON = BON;
                            let BOO = if BON != 0.0 {
                                BLZ
                            } else {
                                GJ
                            };
                            BOM = BOO;
                        } else {
                            BOM = GJ;
                        }
                        oBOM = BOM;
                    } else {
                    }
                } else {
                }
                let BOC = BCI * BOB;
                if BOD != 0.0 {
                    let BOP = if MC <= G { 1.0 } else { 0.0 };
                    oBOP = BOP;
                    let BOR = if MD <= G { 1.0 } else { 0.0 };
                    oBOR = BOR;
                } else {
                }
                let BOQ = BOC * I;
                let BOS = if BJA > BJB { 1.0 } else { 0.0 };
                if BOS != 0.0 {
                    let BOT = if staged[308] != 0.0 && AYB != 0.0 { 1.0 } else { 0.0 };
                    oBOT = BOT;
                    if BOT != 0.0 {
                        let BOU = BJA - BJB;
                        oBOU = BOU;
                    } else {
                    }
                } else {
                }
                if BOV != 0.0 {
                    if BOS != 0.0 {
                        let BOY = (BOX * (BJA - BJB)) + BJB;
                        oBOY = BOY;
                    } else {
                    }
                } else {
                }
                let BOW = if parameters[38] != G { 1.0 } else { 0.0 };
                if BOW != 0.0 {
                    let BOZ = (JE / 1e23f64).powf(parameters[954]);
                    oBOZ = BOZ;
                    let BPA = parameters[955] - BCP;
                    oBPA = BPA;
                    let BPB = BCI * parameters[953];
                    oBPB = BPB;
                    let BPC = -BOZ;
                    oBPC = BPC;
                    let BPD = BCI * parameters[952];
                    oBPD = BPD;
                } else {
                }
                let BPF;
                if OW != 0.0 {
                    let BPL = AF - PB;
                    oBPL = BPL;
                    let BPM = if AF != PB { 1.0 } else { 0.0 };
                    oBPM = BPM;
                    if BPM != 0.0 {
                        let BPN = (AF - (AE * PC)) - PB;
                        let BPO = BPN * BPN;
                        let BPQ = (BPP * staged[164]) * BPO;
                        oBPQ = BPQ;
                        let BPS = LB * BPR;
                        oBPS = BPS;
                        let BPT = ((BPP * BPO) * AJ) * I;
                        oBPT = BPT;
                        let BPU = ((AJ * I) * BPN) * BPP;
                        oBPU = BPU;
                    } else {
                    }
                    let BPV = ((AJ * I) * PB) * BPP;
                    oBPV = BPV;
                    BPF = PC;
                } else {
                    let BPE = if PH >= (AF / AE) { 1.0 } else { 0.0 };
                    oBPE = BPE;
                    let BPW = if BPE != 0.0 {
                        G
                    } else {
                        PH
                    };
                    if BPX != 0.0 {
                        let BPY = AF - (AE * BPW);
                        let BPZ = BPY * BPY;
                        let BQA = staged[339] * BPZ;
                        oBQA = BQA;
                        let BQB = ((BPP * BPZ) * AJ) * I;
                        oBQB = BQB;
                        let BQC = ((AJ * I) * BPY) * BPP;
                        oBQC = BQC;
                    } else {
                    }
                    BPF = BPW;
                }
                let BPG = parameters[814] * AF;
                let BPH = parameters[815] * AF;
                let BPI = parameters[1044] * AF;
                let BPJ = ((-AF) / parameters[1042]).exp();
                if BPK != 0.0 {
                } else {
                    if BQD != 0.0 {
                        let BQE = (I * AJ) * 1.2e1f64;
                        oBQE = BQE;
                    } else {
                    }
                }
                if BQF != 0.0 {
                    let BQG = MK + BMM;
                    oBQG = BQG;
                    let BQH = staged[351] * MI;
                    oBQH = BQH;
                    let BQI = staged[353] * CT;
                    oBQI = BQI;
                    let BQL = if BCH != 0.0 {
                        let BQK = MI / CT;
                        BQK
                    } else {
                        G
                    };
                    oBQL = BQL;
                    let BQM = CI + BQL;
                    oBQM = BQM;
                } else {
                }
                let BQJ = if MM != G { 1.0 } else { 0.0 };
                let BQN = (((-I) * AT) * AQ) * staged[364];
                let BQO = (I * AT) * AQ;
                let BQR = if BQP != 0.0 {
                    BQQ
                } else {
                    EO
                };
                let BQS = parameters[671] + BQR;
                let BQT = parameters[672] + BQR;
                if BQU != 0.0 {
                    let BQV = (-AT) * I;
                    let BQW = BQV * BQS;
                    oBQW = BQW;
                    let BQX = BQV * BQT;
                    oBQX = BQX;
                } else {
                    let BQY = (-AT) * I;
                    oBQY = BQY;
                    let BQZ = LB * ER;
                    oBQZ = BQZ;
                    let BRA = LB * ES;
                    oBRA = BRA;
                }
                let BRB = ((staged[380] * I) * AQ) * parameters[673];
                let BRC = if BKR == CI { 1.0 } else { 0.0 };
                let BRE;
                if BRC != 0.0 {
                    let BRD = (BCM / (BCL * HA)).sqrt();
                    oBRD = BRD;
                    let BRG = HG + (HH / AF);
                    oBRG = BRG;
                    let BRH = HJ - BCP;
                    oBRH = BRH;
                    let BRI = BJW + BLJ;
                    oBRI = BRI;
                    let BRJ = (3.20438e-19f64 * staged[161]) * HA;
                    oBRJ = BRJ;
                    let BRK = CI + (parameters[958] * (CI + (parameters[959] * (AF.powf((-parameters[960]))))));
                    oBRK = BRK;
                    let BRM = parameters[785] * BRL;
                    oBRM = BRM;
                    let BRN = parameters[799] * BRL;
                    oBRN = BRN;
                    let BRO = BPR * BRL;
                    oBRO = BRO;
                    let BRP = AF - (AE * BPF);
                    let BRQ = BRM * BCL;
                    oBRQ = BRQ;
                    let BRR = LB * BRO;
                    oBRR = BRR;
                    let BRT = ((BPP * (BRP * BRP)) * BRS) * I;
                    oBRT = BRT;
                    let BRU = ((BRS * I) * BRP) * BPP;
                    oBRU = BRU;
                    BRE = BRV;
                } else {
                    BRE = G;
                }
                let BRF = BCI * parameters[29];
                let BRX = if BRW != 0.0 && (if AWP > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BSC;
                let BSD;
                let BSE;
                let BSF;
                let BSG;
                let BSH;
                if BRX != 0.0 {
                    let BRZ = if BNB != 0.0 && (if BRY > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBRZ = BRZ;
                    let BSM;
                    let BSN;
                    let BSO;
                    if BRZ != 0.0 {
                        BSM = BSJ;
                        BSN = BSK;
                        BSO = G;
                    } else {
                        BSM = G;
                        BSN = G;
                        BSO = BSL;
                    }
                    BSC = BSP;
                    BSD = BSM;
                    BSE = BSN;
                    BSF = BSO;
                    BSG = G;
                    BSH = G;
                } else {
                    BSC = G;
                    BSD = G;
                    BSE = G;
                    BSF = G;
                    BSG = BSA;
                    BSH = BSB;
                }
                let BSI = if BRW != 0.0 && (if AWO > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BSU;
                let BSV;
                let BSW;
                let BSX;
                let BSY;
                let BSZ;
                if BSI != 0.0 {
                    let BSR = if BNB != 0.0 && (if BSQ > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oBSR = BSR;
                    let BTE;
                    let BTF;
                    let BTG;
                    if BSR != 0.0 {
                        BTE = BTB;
                        BTF = BTC;
                        BTG = G;
                    } else {
                        BTE = G;
                        BTF = G;
                        BTG = BTD;
                    }
                    BSU = BTH;
                    BSV = BTE;
                    BSW = BTF;
                    BSX = BTG;
                    BSY = G;
                    BSZ = G;
                } else {
                    BSU = G;
                    BSV = G;
                    BSW = G;
                    BSX = G;
                    BSY = BSS;
                    BSZ = BST;
                }
                let BTA = if BBO == G { 1.0 } else { 0.0 };
                let BTK;
                let BTL;
                if BTA != 0.0 {
                    BTK = BTI;
                    BTL = G;
                } else {
                    let BTJ = if BBO == AE { 1.0 } else { 0.0 };
                    oBTJ = BTJ;
                    BTK = G;
                    BTL = BTN;
                }
                let BTM = if BBO == RT { 1.0 } else { 0.0 };
                let BTP = if BTM != 0.0 {
                    G
                } else {
                    BTO
                };
                if BBZ != 0.0 {
                    if BRX != 0.0 {
                        let BTQ = if BNB != 0.0 && (if BRY > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBTQ = BTQ;
                    } else {
                    }
                    if BSI != 0.0 {
                        let BTR = if BNB != 0.0 && (if BSQ > G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oBTR = BTR;
                    } else {
                    }
                } else {
                }
                let BUA;
                let BUB;
                let BUC;
                let BUD;
                let BUE;
                let BUF;
                let BUG;
                let BUH;
                if AXF != 0.0 {
                    BUA = BTS;
                    BUB = BTT;
                    BUC = BTU;
                    BUD = BTV;
                    BUE = BTW;
                    BUF = G;
                    BUG = G;
                    BUH = G;
                } else {
                    BUA = G;
                    BUB = G;
                    BUC = G;
                    BUD = G;
                    BUE = G;
                    BUF = BTX;
                    BUG = BTY;
                    BUH = BTZ;
                }
                if AXF != 0.0 {
                    let BUI = if parameters[1097] == G { 1.0 } else { 0.0 };
                    oBUI = BUI;
                } else {
                }
                let BUJ = if AXF != 0.0 && AYB != 0.0 { 1.0 } else { 0.0 };
                let BUM;
                let BUN;
                if BUJ != 0.0 {
                    BUM = BUK;
                    BUN = G;
                } else {
                    BUM = G;
                    BUN = BUL;
                }
                if BUJ != 0.0 {
                    let BUO = (CI - BOX) * BOB;
                    oBUO = BUO;
                    let BUP = BOX * BOB;
                    oBUP = BUP;
                } else {
                }
            [H, K, AF, AG, oAI, AJ, AK, oAL, AQ, AR, oAS, AU, oAV, AY, AZ, oBI, oBS, BX, CE, CU, CY, DC, DG, DH, DV, EE, EG, EH, EI, EK, EP, EQ, ER, ES, EZ, FA, FB, FD, FE, FF, FI, FK, FL, FM, FP, FQ, FR, FS, FT, FU, FV, FW, FX, FY, FZ, GA, GC, GD, GF, GI, GP, GQ, GR, GS, GT, GU, GV, GX, GY, GZ, HA, HB, HC, HD, HF, HI, HJ, HK, HL, HS, HT, HU, HV, HW, HX, HY, HZ, IA, JE, JF, JH, JK, KP, KT, KW, KY, LF, LJ, LO, LS, LW, LZ, MB, MC, MD, ME, MF, MG, MH, MI, ML, MM, MN, MP, MQ, MR, MS, MU, NC, oND, oNG, oNJ, NI, NM, NN, NO, NP, NQ, NR, NS, NT, NU, NV, oNX, oNZ, oOA, OB, JJ, oOC, OD, OF, OH, OJ, OM, OO, OQ, OS, OU, oOY, oPI, oPN, oPY, oQG, oQP, oRB, oRC, oQM, oRS, oRX, oSA, oSG, oSI, oRV, oSL, oSN, oSS, oSU, oSX, oTA, oTB, oTG, oTI, oSY, oTL, oTM, oTR, oTT, oRK, oUD, oUG, oUH, oUM, oUO, oUE, oUR, oUS, oUX, oUZ, oVC, oVF, oVG, oVK, oVM, oVD, oVP, oVQ, oVU, oVW, oTX, oWG, oWJ, oWK, oWO, oWQ, oWH, oWT, oWU, oWY, oXA, oXD, oXG, oXH, oXM, oXO, oXE, oXR, oXS, oXX, oXZ, oWA, oYJ, oYM, oYN, oYR, oYT, oYK, oYW, oYX, oZB, oZD, oZG, oZJ, oZK, oZO, oZQ, oZH, oZT, oZU, oZY, oAAA, oYD, oAAL, oAAO, oAAP, oAAU, oAAW, oAAM, oAAZ, oABA, oABF, oABH, oAAE, oABR, oABU, oABV, oABZ, oACB, oABS, oACE, oACF, oACJ, oACL, oABP, oABL, oACX, oADA, oADB, oADG, oADI, oACY, oADL, oADM, oADR, oADT, oACR, oAEA, oAEF, oAEI, oAEJ, oAEN, oAEP, oAEG, oAES, oAET, oAEX, oAEZ, oADX, oAFD, oAFM, oAFI, oAFX, oRN, oAGC, oAGE, oAGI, oAGP, oAGV, oAHD, oAHP, oAHQ, oAHB, oAIE, oAIH, oAII, oAIN, oAIP, oAIF, oAIS, oAIT, oAIY, oAJA, oAJD, oAJG, oAJH, oAJM, oAJO, oAJE, oAJR, oAJS, oAJX, oAJZ, oAHX, oAKJ, oAKM, oAKN, oAKS, oAKU, oAKK, oAKX, oAKY, oALD, oALF, oALI, oALL, oALM, oALQ, oALS, oALJ, oALV, oALW, oAMA, oAMC, oAKD, oAMM, oAMP, oAMQ, oAMU, oAMW, oAMN, oAMZ, oANA, oANE, oANG, oANJ, oANM, oANN, oANS, oANU, oANK, oANX, oANY, oAOD, oAOF, oAMG, oAOP, oAOS, oAOT, oAOX, oAOZ, oAOQ, oAPC, oAPD, oAPH, oAPJ, oAPM, oAPP, oAPQ, oAPU, oAPW, oAPN, oAPZ, oAQA, oAQE, oAQG, oAOJ, oAQQ, oAQT, oAQU, oAQZ, oARB, oAQR, oARE, oARF, oARK, oARM, oAQK, oARW, oARZ, oASA, oASE, oASG, oARX, oASJ, oASK, oASO, oASQ, oARU, oARQ, oATC, oATF, oATG, oATL, oATN, oATD, oATQ, oATR, oATW, oATY, oASW, oAUF, oAUK, oAUN, oAUO, oAUS, oAUU, oAUL, oAUX, oAUY, oAVC, oAVE, oAUC, oAVI, oAVQ, oAVM, oAWB, oAIA, oAWG, oAWI, AGO, oAWM, oAWR, oAWN, oAWU, oAWW, oAXH, oAXJ, oAXL, oAWX, oAXO, AXF, oAXV, oAYC, oAYE, oAYG, oAZC, oAYR, oAZL, oAZW, oAZZ, oBAC, oBAF, oAZM, oBAI, oBAR, oBAU, oBAJ, oBAX, oBBG, BBK, oBBP, BBV, BBW, BCA, oBCF, oBCG, BCH, oBCJ, oBCK, BCN, BCO, KV, OL, ON, OR, OT, OP, JS, KI, KR, KN, NH, BCU, LL, BCX, LU, JD, OI, BDA, BDB, BDC, oBDD, oBDO, BDM, oBEE, oBEP, oBEZ, oBFK, oBFU, oBGE, oBGN, oBGX, oBHG, oBHQ, BIG, BIL, oBIP, oBIQ, oBIY, oBIZ, BIH, BIR, BJB, BIM, BJA, BJK, oBJO, oBKB, oBKJ, oBKK, oBKP, oBKS, oBLC, oBLN, BLH, BLI, BLK, BLL, LA, LH, LQ, LY, BMI, BMJ, oBMK, BML, BMN, BJT, BMO, BMP, BMQ, AWY, AWZ, AWO, AWP, BMR, BMS, oBMT, BMU, oBMV, BMW, oBMX, oBMY, AXA, AXB, AXC, AXD, BNA, oBNE, oBNG, oBNJ, oBNK, oBNN, oBNO, BNL, oBNP, oBNQ, oBNT, oBNW, OV, oBOA, oBOF, oBOG, oBOH, oBOK, oBOJ, OE, oBON, oBOM, OG, BOC, oBOP, oBOR, BOQ, BOS, oBOT, oBOU, oBOY, BOW, oBOZ, oBPB, oBPC, oBPD, PB, oBPL, oBPM, oBPQ, oBPS, oBPT, oBPU, oBPV, oBPE, oBQA, oBQB, oBQC, BPG, BPH, BPI, BPJ, oBQE, oBQG, oBQH, oBQI, oBQL, oBQM, BQJ, BQN, BQO, BQS, BQT, oBQW, oBQX, oBQY, oBQZ, oBRA, BRB, BRC, oBRD, BJU, oBRG, BJV, oBRI, oBRJ, oBRK, oBRM, oBRN, oBRO, oBRQ, oBRR, oBRT, oBRU, BRF, BRX, oBNC, oBRZ, BSI, oBND, oBSR, BTA, oBTJ, BNR, BTM, oBTQ, oBTR, BCD, BCE, AXW, AXX, AXY, AXZ, AYA, oBUI, BUJ, BBH, oBUO, oBUP, BRE, BSC, BSD, BSE, BSF, BSG, BSH, BSU, BSV, BSW, BSX, BSY, BSZ, BTK, BTL, BTP, BUA, BUB, BUC, BUD, BUE, BUF, BUG, BUH, BUM, BUN, BCQ, BCR, BCS, BCT, BCV, BCY, oBNY, oBPA, oBRH]
        };
        self.canonical_staged[450] = produced[0];
        self.canonical_staged[451] = produced[1];
        self.canonical_staged[170] = produced[2];
        self.canonical_staged[452] = produced[3];
        self.canonical_staged[453] = produced[4];
        self.canonical_staged[193] = produced[5];
        self.canonical_staged[454] = produced[6];
        self.canonical_staged[455] = produced[7];
        self.canonical_staged[357] = produced[8];
        self.canonical_staged[456] = produced[9];
        self.canonical_staged[457] = produced[10];
        self.canonical_staged[458] = produced[11];
        self.canonical_staged[459] = produced[12];
        self.canonical_staged[298] = produced[13];
        self.canonical_staged[460] = produced[14];
        self.canonical_staged[462] = produced[15];
        self.canonical_staged[464] = produced[16];
        self.canonical_staged[465] = produced[17];
        self.canonical_staged[466] = produced[18];
        self.canonical_staged[162] = produced[19];
        self.canonical_staged[169] = produced[20];
        self.canonical_staged[171] = produced[21];
        self.canonical_staged[194] = produced[22];
        self.canonical_staged[69] = produced[23];
        self.canonical_staged[184] = produced[24];
        self.canonical_staged[196] = produced[25];
        self.canonical_staged[203] = produced[26];
        self.canonical_staged[200] = produced[27];
        self.canonical_staged[198] = produced[28];
        self.canonical_staged[195] = produced[29];
        self.canonical_staged[373] = produced[30];
        self.canonical_staged[379] = produced[31];
        self.canonical_staged[370] = produced[32];
        self.canonical_staged[376] = produced[33];
        self.canonical_staged[110] = produced[34];
        self.canonical_staged[287] = produced[35];
        self.canonical_staged[285] = produced[36];
        self.canonical_staged[111] = produced[37];
        self.canonical_staged[292] = produced[38];
        self.canonical_staged[291] = produced[39];
        self.canonical_staged[79] = produced[40];
        self.canonical_staged[85] = produced[41];
        self.canonical_staged[83] = produced[42];
        self.canonical_staged[92] = produced[43];
        self.canonical_staged[106] = produced[44];
        self.canonical_staged[109] = produced[45];
        self.canonical_staged[112] = produced[46];
        self.canonical_staged[263] = produced[47];
        self.canonical_staged[262] = produced[48];
        self.canonical_staged[264] = produced[49];
        self.canonical_staged[260] = produced[50];
        self.canonical_staged[261] = produced[51];
        self.canonical_staged[256] = produced[52];
        self.canonical_staged[255] = produced[53];
        self.canonical_staged[257] = produced[54];
        self.canonical_staged[254] = produced[55];
        self.canonical_staged[267] = produced[56];
        self.canonical_staged[269] = produced[57];
        self.canonical_staged[272] = produced[58];
        self.canonical_staged[278] = produced[59];
        self.canonical_staged[166] = produced[60];
        self.canonical_staged[191] = produced[61];
        self.canonical_staged[103] = produced[62];
        self.canonical_staged[102] = produced[63];
        self.canonical_staged[105] = produced[64];
        self.canonical_staged[104] = produced[65];
        self.canonical_staged[114] = produced[66];
        self.canonical_staged[113] = produced[67];
        self.canonical_staged[115] = produced[68];
        self.canonical_staged[384] = produced[69];
        self.canonical_staged[382] = produced[70];
        self.canonical_staged[388] = produced[71];
        self.canonical_staged[389] = produced[72];
        self.canonical_staged[390] = produced[73];
        self.canonical_staged[391] = produced[74];
        self.canonical_staged[392] = produced[75];
        self.canonical_staged[394] = produced[76];
        self.canonical_staged[383] = produced[77];
        self.canonical_staged[385] = produced[78];
        self.canonical_staged[118] = produced[79];
        self.canonical_staged[117] = produced[80];
        self.canonical_staged[120] = produced[81];
        self.canonical_staged[119] = produced[82];
        self.canonical_staged[122] = produced[83];
        self.canonical_staged[121] = produced[84];
        self.canonical_staged[337] = produced[85];
        self.canonical_staged[335] = produced[86];
        self.canonical_staged[336] = produced[87];
        self.canonical_staged[63] = produced[88];
        self.canonical_staged[72] = produced[89];
        self.canonical_staged[151] = produced[90];
        self.canonical_staged[163] = produced[91];
        self.canonical_staged[80] = produced[92];
        self.canonical_staged[73] = produced[93];
        self.canonical_staged[165] = produced[94];
        self.canonical_staged[153] = produced[95];
        self.canonical_staged[155] = produced[96];
        self.canonical_staged[94] = produced[97];
        self.canonical_staged[157] = produced[98];
        self.canonical_staged[100] = produced[99];
        self.canonical_staged[159] = produced[100];
        self.canonical_staged[107] = produced[101];
        self.canonical_staged[248] = produced[102];
        self.canonical_staged[288] = produced[103];
        self.canonical_staged[293] = produced[104];
        self.canonical_staged[268] = produced[105];
        self.canonical_staged[273] = produced[106];
        self.canonical_staged[279] = produced[107];
        self.canonical_staged[271] = produced[108];
        self.canonical_staged[350] = produced[109];
        self.canonical_staged[97] = produced[110];
        self.canonical_staged[359] = produced[111];
        self.canonical_staged[174] = produced[112];
        self.canonical_staged[185] = produced[113];
        self.canonical_staged[75] = produced[114];
        self.canonical_staged[77] = produced[115];
        self.canonical_staged[81] = produced[116];
        self.canonical_staged[99] = produced[117];
        self.canonical_staged[473] = produced[118];
        self.canonical_staged[474] = produced[119];
        self.canonical_staged[475] = produced[120];
        self.canonical_staged[476] = produced[121];
        self.canonical_staged[284] = produced[122];
        self.canonical_staged[290] = produced[123];
        self.canonical_staged[477] = produced[124];
        self.canonical_staged[478] = produced[125];
        self.canonical_staged[479] = produced[126];
        self.canonical_staged[480] = produced[127];
        self.canonical_staged[481] = produced[128];
        self.canonical_staged[482] = produced[129];
        self.canonical_staged[483] = produced[130];
        self.canonical_staged[484] = produced[131];
        self.canonical_staged[485] = produced[132];
        self.canonical_staged[487] = produced[133];
        self.canonical_staged[489] = produced[134];
        self.canonical_staged[490] = produced[135];
        self.canonical_staged[491] = produced[136];
        self.canonical_staged[150] = produced[137];
        self.canonical_staged[492] = produced[138];
        self.canonical_staged[493] = produced[139];
        self.canonical_staged[494] = produced[140];
        self.canonical_staged[495] = produced[141];
        self.canonical_staged[496] = produced[142];
        self.canonical_staged[497] = produced[143];
        self.canonical_staged[498] = produced[144];
        self.canonical_staged[499] = produced[145];
        self.canonical_staged[500] = produced[146];
        self.canonical_staged[501] = produced[147];
        self.canonical_staged[503] = produced[148];
        self.canonical_staged[504] = produced[149];
        self.canonical_staged[505] = produced[150];
        self.canonical_staged[506] = produced[151];
        self.canonical_staged[507] = produced[152];
        self.canonical_staged[509] = produced[153];
        self.canonical_staged[510] = produced[154];
        self.canonical_staged[511] = produced[155];
        self.canonical_staged[508] = produced[156];
        self.canonical_staged[514] = produced[157];
        self.canonical_staged[516] = produced[158];
        self.canonical_staged[517] = produced[159];
        self.canonical_staged[518] = produced[160];
        self.canonical_staged[519] = produced[161];
        self.canonical_staged[515] = produced[162];
        self.canonical_staged[520] = produced[163];
        self.canonical_staged[521] = produced[164];
        self.canonical_staged[522] = produced[165];
        self.canonical_staged[523] = produced[166];
        self.canonical_staged[524] = produced[167];
        self.canonical_staged[526] = produced[168];
        self.canonical_staged[527] = produced[169];
        self.canonical_staged[528] = produced[170];
        self.canonical_staged[529] = produced[171];
        self.canonical_staged[525] = produced[172];
        self.canonical_staged[530] = produced[173];
        self.canonical_staged[531] = produced[174];
        self.canonical_staged[532] = produced[175];
        self.canonical_staged[533] = produced[176];
        self.canonical_staged[512] = produced[177];
        self.canonical_staged[535] = produced[178];
        self.canonical_staged[537] = produced[179];
        self.canonical_staged[538] = produced[180];
        self.canonical_staged[539] = produced[181];
        self.canonical_staged[540] = produced[182];
        self.canonical_staged[536] = produced[183];
        self.canonical_staged[541] = produced[184];
        self.canonical_staged[542] = produced[185];
        self.canonical_staged[543] = produced[186];
        self.canonical_staged[544] = produced[187];
        self.canonical_staged[545] = produced[188];
        self.canonical_staged[547] = produced[189];
        self.canonical_staged[548] = produced[190];
        self.canonical_staged[549] = produced[191];
        self.canonical_staged[550] = produced[192];
        self.canonical_staged[546] = produced[193];
        self.canonical_staged[551] = produced[194];
        self.canonical_staged[552] = produced[195];
        self.canonical_staged[553] = produced[196];
        self.canonical_staged[554] = produced[197];
        self.canonical_staged[534] = produced[198];
        self.canonical_staged[556] = produced[199];
        self.canonical_staged[558] = produced[200];
        self.canonical_staged[559] = produced[201];
        self.canonical_staged[560] = produced[202];
        self.canonical_staged[561] = produced[203];
        self.canonical_staged[557] = produced[204];
        self.canonical_staged[562] = produced[205];
        self.canonical_staged[563] = produced[206];
        self.canonical_staged[564] = produced[207];
        self.canonical_staged[565] = produced[208];
        self.canonical_staged[566] = produced[209];
        self.canonical_staged[568] = produced[210];
        self.canonical_staged[569] = produced[211];
        self.canonical_staged[570] = produced[212];
        self.canonical_staged[571] = produced[213];
        self.canonical_staged[567] = produced[214];
        self.canonical_staged[572] = produced[215];
        self.canonical_staged[573] = produced[216];
        self.canonical_staged[574] = produced[217];
        self.canonical_staged[575] = produced[218];
        self.canonical_staged[555] = produced[219];
        self.canonical_staged[577] = produced[220];
        self.canonical_staged[579] = produced[221];
        self.canonical_staged[580] = produced[222];
        self.canonical_staged[581] = produced[223];
        self.canonical_staged[582] = produced[224];
        self.canonical_staged[578] = produced[225];
        self.canonical_staged[583] = produced[226];
        self.canonical_staged[584] = produced[227];
        self.canonical_staged[585] = produced[228];
        self.canonical_staged[586] = produced[229];
        self.canonical_staged[587] = produced[230];
        self.canonical_staged[589] = produced[231];
        self.canonical_staged[590] = produced[232];
        self.canonical_staged[591] = produced[233];
        self.canonical_staged[592] = produced[234];
        self.canonical_staged[588] = produced[235];
        self.canonical_staged[593] = produced[236];
        self.canonical_staged[594] = produced[237];
        self.canonical_staged[595] = produced[238];
        self.canonical_staged[596] = produced[239];
        self.canonical_staged[576] = produced[240];
        self.canonical_staged[598] = produced[241];
        self.canonical_staged[600] = produced[242];
        self.canonical_staged[601] = produced[243];
        self.canonical_staged[602] = produced[244];
        self.canonical_staged[603] = produced[245];
        self.canonical_staged[599] = produced[246];
        self.canonical_staged[604] = produced[247];
        self.canonical_staged[605] = produced[248];
        self.canonical_staged[606] = produced[249];
        self.canonical_staged[607] = produced[250];
        self.canonical_staged[597] = produced[251];
        self.canonical_staged[610] = produced[252];
        self.canonical_staged[612] = produced[253];
        self.canonical_staged[613] = produced[254];
        self.canonical_staged[614] = produced[255];
        self.canonical_staged[615] = produced[256];
        self.canonical_staged[611] = produced[257];
        self.canonical_staged[616] = produced[258];
        self.canonical_staged[617] = produced[259];
        self.canonical_staged[618] = produced[260];
        self.canonical_staged[619] = produced[261];
        self.canonical_staged[609] = produced[262];
        self.canonical_staged[608] = produced[263];
        self.canonical_staged[621] = produced[264];
        self.canonical_staged[623] = produced[265];
        self.canonical_staged[624] = produced[266];
        self.canonical_staged[625] = produced[267];
        self.canonical_staged[626] = produced[268];
        self.canonical_staged[622] = produced[269];
        self.canonical_staged[627] = produced[270];
        self.canonical_staged[628] = produced[271];
        self.canonical_staged[629] = produced[272];
        self.canonical_staged[630] = produced[273];
        self.canonical_staged[620] = produced[274];
        self.canonical_staged[632] = produced[275];
        self.canonical_staged[633] = produced[276];
        self.canonical_staged[635] = produced[277];
        self.canonical_staged[636] = produced[278];
        self.canonical_staged[637] = produced[279];
        self.canonical_staged[638] = produced[280];
        self.canonical_staged[634] = produced[281];
        self.canonical_staged[639] = produced[282];
        self.canonical_staged[640] = produced[283];
        self.canonical_staged[641] = produced[284];
        self.canonical_staged[642] = produced[285];
        self.canonical_staged[631] = produced[286];
        self.canonical_staged[643] = produced[287];
        self.canonical_staged[645] = produced[288];
        self.canonical_staged[644] = produced[289];
        self.canonical_staged[646] = produced[290];
        self.canonical_staged[513] = produced[291];
        self.canonical_staged[647] = produced[292];
        self.canonical_staged[648] = produced[293];
        self.canonical_staged[649] = produced[294];
        self.canonical_staged[651] = produced[295];
        self.canonical_staged[652] = produced[296];
        self.canonical_staged[654] = produced[297];
        self.canonical_staged[655] = produced[298];
        self.canonical_staged[656] = produced[299];
        self.canonical_staged[653] = produced[300];
        self.canonical_staged[659] = produced[301];
        self.canonical_staged[661] = produced[302];
        self.canonical_staged[662] = produced[303];
        self.canonical_staged[663] = produced[304];
        self.canonical_staged[664] = produced[305];
        self.canonical_staged[660] = produced[306];
        self.canonical_staged[665] = produced[307];
        self.canonical_staged[666] = produced[308];
        self.canonical_staged[667] = produced[309];
        self.canonical_staged[668] = produced[310];
        self.canonical_staged[669] = produced[311];
        self.canonical_staged[671] = produced[312];
        self.canonical_staged[672] = produced[313];
        self.canonical_staged[673] = produced[314];
        self.canonical_staged[674] = produced[315];
        self.canonical_staged[670] = produced[316];
        self.canonical_staged[675] = produced[317];
        self.canonical_staged[676] = produced[318];
        self.canonical_staged[677] = produced[319];
        self.canonical_staged[678] = produced[320];
        self.canonical_staged[657] = produced[321];
        self.canonical_staged[680] = produced[322];
        self.canonical_staged[682] = produced[323];
        self.canonical_staged[683] = produced[324];
        self.canonical_staged[684] = produced[325];
        self.canonical_staged[685] = produced[326];
        self.canonical_staged[681] = produced[327];
        self.canonical_staged[686] = produced[328];
        self.canonical_staged[687] = produced[329];
        self.canonical_staged[688] = produced[330];
        self.canonical_staged[689] = produced[331];
        self.canonical_staged[690] = produced[332];
        self.canonical_staged[692] = produced[333];
        self.canonical_staged[693] = produced[334];
        self.canonical_staged[694] = produced[335];
        self.canonical_staged[695] = produced[336];
        self.canonical_staged[691] = produced[337];
        self.canonical_staged[696] = produced[338];
        self.canonical_staged[697] = produced[339];
        self.canonical_staged[698] = produced[340];
        self.canonical_staged[699] = produced[341];
        self.canonical_staged[679] = produced[342];
        self.canonical_staged[701] = produced[343];
        self.canonical_staged[703] = produced[344];
        self.canonical_staged[704] = produced[345];
        self.canonical_staged[705] = produced[346];
        self.canonical_staged[706] = produced[347];
        self.canonical_staged[702] = produced[348];
        self.canonical_staged[707] = produced[349];
        self.canonical_staged[708] = produced[350];
        self.canonical_staged[709] = produced[351];
        self.canonical_staged[710] = produced[352];
        self.canonical_staged[711] = produced[353];
        self.canonical_staged[713] = produced[354];
        self.canonical_staged[714] = produced[355];
        self.canonical_staged[715] = produced[356];
        self.canonical_staged[716] = produced[357];
        self.canonical_staged[712] = produced[358];
        self.canonical_staged[717] = produced[359];
        self.canonical_staged[718] = produced[360];
        self.canonical_staged[719] = produced[361];
        self.canonical_staged[720] = produced[362];
        self.canonical_staged[700] = produced[363];
        self.canonical_staged[722] = produced[364];
        self.canonical_staged[724] = produced[365];
        self.canonical_staged[725] = produced[366];
        self.canonical_staged[726] = produced[367];
        self.canonical_staged[727] = produced[368];
        self.canonical_staged[723] = produced[369];
        self.canonical_staged[728] = produced[370];
        self.canonical_staged[729] = produced[371];
        self.canonical_staged[730] = produced[372];
        self.canonical_staged[731] = produced[373];
        self.canonical_staged[732] = produced[374];
        self.canonical_staged[734] = produced[375];
        self.canonical_staged[735] = produced[376];
        self.canonical_staged[736] = produced[377];
        self.canonical_staged[737] = produced[378];
        self.canonical_staged[733] = produced[379];
        self.canonical_staged[738] = produced[380];
        self.canonical_staged[739] = produced[381];
        self.canonical_staged[740] = produced[382];
        self.canonical_staged[741] = produced[383];
        self.canonical_staged[721] = produced[384];
        self.canonical_staged[743] = produced[385];
        self.canonical_staged[745] = produced[386];
        self.canonical_staged[746] = produced[387];
        self.canonical_staged[747] = produced[388];
        self.canonical_staged[748] = produced[389];
        self.canonical_staged[744] = produced[390];
        self.canonical_staged[749] = produced[391];
        self.canonical_staged[750] = produced[392];
        self.canonical_staged[751] = produced[393];
        self.canonical_staged[752] = produced[394];
        self.canonical_staged[742] = produced[395];
        self.canonical_staged[755] = produced[396];
        self.canonical_staged[757] = produced[397];
        self.canonical_staged[758] = produced[398];
        self.canonical_staged[759] = produced[399];
        self.canonical_staged[760] = produced[400];
        self.canonical_staged[756] = produced[401];
        self.canonical_staged[761] = produced[402];
        self.canonical_staged[762] = produced[403];
        self.canonical_staged[763] = produced[404];
        self.canonical_staged[764] = produced[405];
        self.canonical_staged[754] = produced[406];
        self.canonical_staged[753] = produced[407];
        self.canonical_staged[766] = produced[408];
        self.canonical_staged[768] = produced[409];
        self.canonical_staged[769] = produced[410];
        self.canonical_staged[770] = produced[411];
        self.canonical_staged[771] = produced[412];
        self.canonical_staged[767] = produced[413];
        self.canonical_staged[772] = produced[414];
        self.canonical_staged[773] = produced[415];
        self.canonical_staged[774] = produced[416];
        self.canonical_staged[775] = produced[417];
        self.canonical_staged[765] = produced[418];
        self.canonical_staged[777] = produced[419];
        self.canonical_staged[778] = produced[420];
        self.canonical_staged[780] = produced[421];
        self.canonical_staged[781] = produced[422];
        self.canonical_staged[782] = produced[423];
        self.canonical_staged[783] = produced[424];
        self.canonical_staged[779] = produced[425];
        self.canonical_staged[784] = produced[426];
        self.canonical_staged[785] = produced[427];
        self.canonical_staged[786] = produced[428];
        self.canonical_staged[787] = produced[429];
        self.canonical_staged[776] = produced[430];
        self.canonical_staged[788] = produced[431];
        self.canonical_staged[790] = produced[432];
        self.canonical_staged[789] = produced[433];
        self.canonical_staged[791] = produced[434];
        self.canonical_staged[658] = produced[435];
        self.canonical_staged[792] = produced[436];
        self.canonical_staged[793] = produced[437];
        self.canonical_staged[650] = produced[438];
        self.canonical_staged[794] = produced[439];
        self.canonical_staged[796] = produced[440];
        self.canonical_staged[795] = produced[441];
        self.canonical_staged[797] = produced[442];
        self.canonical_staged[798] = produced[443];
        self.canonical_staged[801] = produced[444];
        self.canonical_staged[802] = produced[445];
        self.canonical_staged[803] = produced[446];
        self.canonical_staged[799] = produced[447];
        self.canonical_staged[804] = produced[448];
        self.canonical_staged[800] = produced[449];
        self.canonical_staged[805] = produced[450];
        self.canonical_staged[806] = produced[451];
        self.canonical_staged[807] = produced[452];
        self.canonical_staged[808] = produced[453];
        self.canonical_staged[810] = produced[454];
        self.canonical_staged[809] = produced[455];
        self.canonical_staged[811] = produced[456];
        self.canonical_staged[813] = produced[457];
        self.canonical_staged[814] = produced[458];
        self.canonical_staged[815] = produced[459];
        self.canonical_staged[816] = produced[460];
        self.canonical_staged[812] = produced[461];
        self.canonical_staged[817] = produced[462];
        self.canonical_staged[819] = produced[463];
        self.canonical_staged[820] = produced[464];
        self.canonical_staged[818] = produced[465];
        self.canonical_staged[821] = produced[466];
        self.canonical_staged[822] = produced[467];
        self.canonical_staged[823] = produced[468];
        self.canonical_staged[824] = produced[469];
        self.canonical_staged[276] = produced[470];
        self.canonical_staged[275] = produced[471];
        self.canonical_staged[825] = produced[472];
        self.canonical_staged[64] = produced[473];
        self.canonical_staged[65] = produced[474];
        self.canonical_staged[827] = produced[475];
        self.canonical_staged[67] = produced[476];
        self.canonical_staged[68] = produced[477];
        self.canonical_staged[160] = produced[478];
        self.canonical_staged[324] = produced[479];
        self.canonical_staged[74] = produced[480];
        self.canonical_staged[76] = produced[481];
        self.canonical_staged[78] = produced[482];
        self.canonical_staged[82] = produced[483];
        self.canonical_staged[84] = produced[484];
        self.canonical_staged[86] = produced[485];
        self.canonical_staged[87] = produced[486];
        self.canonical_staged[88] = produced[487];
        self.canonical_staged[89] = produced[488];
        self.canonical_staged[90] = produced[489];
        self.canonical_staged[91] = produced[490];
        self.canonical_staged[93] = produced[491];
        self.canonical_staged[96] = produced[492];
        self.canonical_staged[98] = produced[493];
        self.canonical_staged[101] = produced[494];
        self.canonical_staged[108] = produced[495];
        self.canonical_staged[116] = produced[496];
        self.canonical_staged[127] = produced[497];
        self.canonical_staged[131] = produced[498];
        self.canonical_staged[830] = produced[499];
        self.canonical_staged[831] = produced[500];
        self.canonical_staged[833] = produced[501];
        self.canonical_staged[832] = produced[502];
        self.canonical_staged[834] = produced[503];
        self.canonical_staged[835] = produced[504];
        self.canonical_staged[836] = produced[505];
        self.canonical_staged[837] = produced[506];
        self.canonical_staged[838] = produced[507];
        self.canonical_staged[839] = produced[508];
        self.canonical_staged[840] = produced[509];
        self.canonical_staged[841] = produced[510];
        self.canonical_staged[842] = produced[511];
        self.canonical_staged[843] = produced[512];
        self.canonical_staged[844] = produced[513];
        self.canonical_staged[845] = produced[514];
        self.canonical_staged[846] = produced[515];
        self.canonical_staged[847] = produced[516];
        self.canonical_staged[848] = produced[517];
        self.canonical_staged[849] = produced[518];
        self.canonical_staged[136] = produced[519];
        self.canonical_staged[137] = produced[520];
        self.canonical_staged[138] = produced[521];
        self.canonical_staged[139] = produced[522];
        self.canonical_staged[140] = produced[523];
        self.canonical_staged[850] = produced[524];
        self.canonical_staged[141] = produced[525];
        self.canonical_staged[852] = produced[526];
        self.canonical_staged[142] = produced[527];
        self.canonical_staged[143] = produced[528];
        self.canonical_staged[144] = produced[529];
        self.canonical_staged[853] = produced[530];
        self.canonical_staged[854] = produced[531];
        self.canonical_staged[856] = produced[532];
        self.canonical_staged[177] = produced[533];
        self.canonical_staged[397] = produced[534];
        self.canonical_staged[145] = produced[535];
        self.canonical_staged[175] = produced[536];
        self.canonical_staged[152] = produced[537];
        self.canonical_staged[154] = produced[538];
        self.canonical_staged[156] = produced[539];
        self.canonical_staged[158] = produced[540];
        self.canonical_staged[167] = produced[541];
        self.canonical_staged[857] = produced[542];
        self.canonical_staged[168] = produced[543];
        self.canonical_staged[172] = produced[544];
        self.canonical_staged[173] = produced[545];
        self.canonical_staged[176] = produced[546];
        self.canonical_staged[179] = produced[547];
        self.canonical_staged[181] = produced[548];
        self.canonical_staged[188] = produced[549];
        self.canonical_staged[187] = produced[550];
        self.canonical_staged[186] = produced[551];
        self.canonical_staged[189] = produced[552];
        self.canonical_staged[190] = produced[553];
        self.canonical_staged[859] = produced[554];
        self.canonical_staged[860] = produced[555];
        self.canonical_staged[197] = produced[556];
        self.canonical_staged[861] = produced[557];
        self.canonical_staged[199] = produced[558];
        self.canonical_staged[862] = produced[559];
        self.canonical_staged[202] = produced[560];
        self.canonical_staged[201] = produced[561];
        self.canonical_staged[205] = produced[562];
        self.canonical_staged[204] = produced[563];
        self.canonical_staged[207] = produced[564];
        self.canonical_staged[206] = produced[565];
        self.canonical_staged[208] = produced[566];
        self.canonical_staged[209] = produced[567];
        self.canonical_staged[211] = produced[568];
        self.canonical_staged[870] = produced[569];
        self.canonical_staged[872] = produced[570];
        self.canonical_staged[237] = produced[571];
        self.canonical_staged[241] = produced[572];
        self.canonical_staged[875] = produced[573];
        self.canonical_staged[245] = produced[574];
        self.canonical_staged[880] = produced[575];
        self.canonical_staged[882] = produced[576];
        self.canonical_staged[246] = produced[577];
        self.canonical_staged[247] = produced[578];
        self.canonical_staged[250] = produced[579];
        self.canonical_staged[259] = produced[580];
        self.canonical_staged[266] = produced[581];
        self.canonical_staged[270] = produced[582];
        self.canonical_staged[888] = produced[583];
        self.canonical_staged[274] = produced[584];
        self.canonical_staged[277] = produced[585];
        self.canonical_staged[889] = produced[586];
        self.canonical_staged[280] = produced[587];
        self.canonical_staged[281] = produced[588];
        self.canonical_staged[282] = produced[589];
        self.canonical_staged[283] = produced[590];
        self.canonical_staged[289] = produced[591];
        self.canonical_staged[294] = produced[592];
        self.canonical_staged[893] = produced[593];
        self.canonical_staged[894] = produced[594];
        self.canonical_staged[309] = produced[595];
        self.canonical_staged[319] = produced[596];
        self.canonical_staged[899] = produced[597];
        self.canonical_staged[322] = produced[598];
        self.canonical_staged[320] = produced[599];
        self.canonical_staged[321] = produced[600];
        self.canonical_staged[323] = produced[601];
        self.canonical_staged[327] = produced[602];
        self.canonical_staged[328] = produced[603];
        self.canonical_staged[903] = produced[604];
        self.canonical_staged[330] = produced[605];
        self.canonical_staged[329] = produced[606];
        self.canonical_staged[331] = produced[607];
        self.canonical_staged[332] = produced[608];
        self.canonical_staged[334] = produced[609];
        self.canonical_staged[901] = produced[610];
        self.canonical_staged[341] = produced[611];
        self.canonical_staged[342] = produced[612];
        self.canonical_staged[343] = produced[613];
        self.canonical_staged[344] = produced[614];
        self.canonical_staged[345] = produced[615];
        self.canonical_staged[346] = produced[616];
        self.canonical_staged[347] = produced[617];
        self.canonical_staged[348] = produced[618];
        self.canonical_staged[349] = produced[619];
        self.canonical_staged[352] = produced[620];
        self.canonical_staged[355] = produced[621];
        self.canonical_staged[358] = produced[622];
        self.canonical_staged[356] = produced[623];
        self.canonical_staged[908] = produced[624];
        self.canonical_staged[365] = produced[625];
        self.canonical_staged[366] = produced[626];
        self.canonical_staged[371] = produced[627];
        self.canonical_staged[377] = produced[628];
        self.canonical_staged[367] = produced[629];
        self.canonical_staged[368] = produced[630];
        self.canonical_staged[374] = produced[631];
        self.canonical_staged[372] = produced[632];
        self.canonical_staged[378] = produced[633];
        self.canonical_staged[381] = produced[634];
        self.canonical_staged[913] = produced[635];
        self.canonical_staged[387] = produced[636];
        self.canonical_staged[386] = produced[637];
        self.canonical_staged[393] = produced[638];
        self.canonical_staged[395] = produced[639];
        self.canonical_staged[396] = produced[640];
        self.canonical_staged[398] = produced[641];
        self.canonical_staged[399] = produced[642];
        self.canonical_staged[401] = produced[643];
        self.canonical_staged[400] = produced[644];
        self.canonical_staged[402] = produced[645];
        self.canonical_staged[403] = produced[646];
        self.canonical_staged[404] = produced[647];
        self.canonical_staged[405] = produced[648];
        self.canonical_staged[406] = produced[649];
        self.canonical_staged[407] = produced[650];
        self.canonical_staged[916] = produced[651];
        self.canonical_staged[864] = produced[652];
        self.canonical_staged[917] = produced[653];
        self.canonical_staged[918] = produced[654];
        self.canonical_staged[865] = produced[655];
        self.canonical_staged[919] = produced[656];
        self.canonical_staged[920] = produced[657];
        self.canonical_staged[921] = produced[658];
        self.canonical_staged[923] = produced[659];
        self.canonical_staged[922] = produced[660];
        self.canonical_staged[924] = produced[661];
        self.canonical_staged[925] = produced[662];
        self.canonical_staged[410] = produced[663];
        self.canonical_staged[411] = produced[664];
        self.canonical_staged[412] = produced[665];
        self.canonical_staged[413] = produced[666];
        self.canonical_staged[414] = produced[667];
        self.canonical_staged[415] = produced[668];
        self.canonical_staged[416] = produced[669];
        self.canonical_staged[926] = produced[670];
        self.canonical_staged[927] = produced[671];
        self.canonical_staged[417] = produced[672];
        self.canonical_staged[418] = produced[673];
        self.canonical_staged[419] = produced[674];
        self.canonical_staged[938] = produced[675];
        self.canonical_staged[939] = produced[676];
        self.canonical_staged[940] = produced[677];
        self.canonical_staged[941] = produced[678];
        self.canonical_staged[942] = produced[679];
        self.canonical_staged[943] = produced[680];
        self.canonical_staged[944] = produced[681];
        self.canonical_staged[945] = produced[682];
        self.canonical_staged[946] = produced[683];
        self.canonical_staged[947] = produced[684];
        self.canonical_staged[948] = produced[685];
        self.canonical_staged[949] = produced[686];
        self.canonical_staged[950] = produced[687];
        self.canonical_staged[951] = produced[688];
        self.canonical_staged[952] = produced[689];
        self.canonical_staged[953] = produced[690];
        self.canonical_staged[955] = produced[691];
        self.canonical_staged[956] = produced[692];
        self.canonical_staged[957] = produced[693];
        self.canonical_staged[958] = produced[694];
        self.canonical_staged[959] = produced[695];
        self.canonical_staged[960] = produced[696];
        self.canonical_staged[961] = produced[697];
        self.canonical_staged[962] = produced[698];
        self.canonical_staged[963] = produced[699];
        self.canonical_staged[964] = produced[700];
        self.canonical_staged[420] = produced[701];
        self.canonical_staged[421] = produced[702];
        self.canonical_staged[422] = produced[703];
        self.canonical_staged[423] = produced[704];
        self.canonical_staged[424] = produced[705];
        self.canonical_staged[427] = produced[706];
        self.canonical_staged[438] = produced[707];
        self.canonical_staged[441] = produced[708];
        self.canonical_staged[447] = produced[709];
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
                let B = staged[850];
                let C = staged[852];
                let A = temperature + parameters[33];
                if B != 0.0 {
                    loop {
                        if C == 0.0 {
                            break;
                        }
                    }
                } else {
                }
            [A]
        };
        self.canonical_staged[60] = produced[0];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[850];
                let B = staged[852];
                if A != 0.0 {
                    loop {
                        if B == 0.0 {
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
        let multiplicity = self.multiplicity;
        let staged = &*self.canonical_staged;
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 64066 => 0usize, 64101 => 1usize, 64121 => 2usize, 68213 => 3usize, 68215 => 4usize, 68217 => 5usize, 68223 => 6usize, 68230 => 7usize, 68237 => 8usize, 68537 => 9usize, 68626 => 10usize, 68674 => 12usize, 68680 => 13usize, 68646 => 11usize, 68741 => 14usize, 68747 => 15usize, _ => usize::MAX };
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
            let C = staged[468];
            let D = staged[472];
            let E = staged[486];
            let F = staged[488];
            let G = staged[502];
            let H = staged[650];
            let I = staged[800];
            let J = staged[297];
            let K = staged[825];
            let L = node_potentials[4];
            let M = 1e0f64;
            let N = 0e0f64;
            let R = 8.617087e-5f64;
            let U = 1e0f64;
            let W = -1e0f64;
            let Y = staged[61];
            let AC = parameters[821];
            let AJ = 2e0f64;
            let AK = 1e0f64;
            let AL = parameters[108];
            let AN = staged[62];
            let AO = 2e0f64;
            let AV = staged[63];
            let AX = 1e-38f64;
            let BB = 1e-6f64;
            let CA = staged[827];
            let CB = staged[67];
            let CC = staged[68];
            let CH = 4e-1f64;
            let CI = staged[69];
            let CP = parameters[823];
            let CX = 5e-1f64;
            let DC = staged[72];
            let DF = parameters[851];
            let DI = staged[73];
            let DL = staged[74];
            let DQ = staged[75];
            let DT = staged[76];
            let DW = staged[77];
            let EI = staged[78];
            let EL = staged[79];
            let EX = staged[80];
            let FA = staged[81];
            let FD = staged[82];
            let FG = staged[83];
            let FJ = staged[84];
            let FM = staged[85];
            let FY = staged[86];
            let GB = staged[87];
            let GP = staged[92];
            let GS = staged[93];
            let GV = staged[94];
            let GY = 1e2f64;
            let HI = staged[88];
            let HU = staged[89];
            let HX = staged[90];
            let IA = staged[91];
            let IF = staged[829];
            let IG = parameters[1120];
            let IJ = staged[95];
            let IK = parameters[1100];
            let IR = staged[96];
            let IX = staged[97];
            let JF = parameters[861];
            let JG = staged[98];
            let JV = staged[99];
            let KH = staged[100];
            let KN = staged[102];
            let KZ = staged[101];
            let LK = staged[103];
            let LN = staged[104];
            let LZ = staged[105];
            let MC = staged[106];
            let MF = staged[107];
            let MI = staged[108];
            let MN = staged[109];
            let MZ = staged[110];
            let NL = staged[111];
            let NR = staged[112];
            let NV = staged[113];
            let OH = staged[114];
            let OK = staged[115];
            let OW = staged[116];
            let OZ = staged[117];
            let PL = staged[118];
            let PO = staged[119];
            let QA = staged[120];
            let QD = staged[121];
            let QP = staged[122];
            let QS = parameters[889];
            let RE = parameters[701];
            let RQ = parameters[702];
            let RT = parameters[890];
            let SF = parameters[703];
            let SR = parameters[704];
            let SU = parameters[891];
            let TG = parameters[705];
            let TS = parameters[706];
            let TV = parameters[892];
            let TY = 1e-2f64;
            let UV = parameters[893];
            let VU = parameters[894];
            let WW = parameters[895];
            let WX = parameters[725];
            let XB = parameters[719];
            let XE = parameters[721];
            let XH = parameters[723];
            let XK = parameters[896];
            let XL = parameters[726];
            let XP = parameters[720];
            let XS = parameters[722];
            let XV = parameters[724];
            let XY = staged[124];
            let YA = parameters[735];
            let YD = staged[125];
            let YF = parameters[737];
            let YI = staged[126];
            let YK = staged[127];
            let YN = staged[128];
            let YP = parameters[736];
            let YS = staged[129];
            let YU = parameters[738];
            let YX = staged[130];
            let YZ = staged[131];
            let ZC = parameters[903];
            let ZD = parameters[742];
            let ZQ = parameters[905];
            let ZR = parameters[744];
            let AAE = parameters[907];
            let AAF = parameters[746];
            let AAS = parameters[904];
            let AAT = parameters[743];
            let ABG = parameters[906];
            let ABH = parameters[745];
            let ABU = parameters[908];
            let ABV = parameters[747];
            let ACI = staged[136];
            let ACJ = staged[137];
            let ACK = staged[138];
            let ACQ = parameters[731];
            let ACT = parameters[733];
            let ACX = 1e1f64;
            let ADB = 4e0f64;
            let AEN = staged[139];
            let AEO = staged[140];
            let AFQ = parameters[732];
            let AFT = parameters[734];
            let AHL = staged[850];
            let AII = parameters[917];
            let AIJ = staged[141];
            let AIS = staged[852];
            let AIV = staged[142];
            let AIY = staged[143];
            let AJD = parameters[916];
            let AJL = staged[145];
            let AJO = node_potentials[9];
            let AJP = node_potentials[11];
            let AJR = 1e0f64;
            let AJS = 1e0f64;
            let AJU = staged[66];
            let AJX = node_potentials[5];
            let AJY = 1e0f64;
            let AKD = node_potentials[7];
            let AKF = 1e0f64;
            let AKP = node_potentials[12];
            let AKR = 1e0f64;
            let AKV = node_potentials[13];
            let AKX = 1e0f64;
            let ALB = node_potentials[14];
            let ALD = 1e0f64;
            let ALL = node_potentials[10];
            let ALM = 1e0f64;
            let ALT = staged[855];
            let ALU = node_potentials[6];
            let ALV = 1e0f64;
            let ALW = staged[146];
            let AMS = -1e0f64;
            let ANI = parameters[956];
            let ANL = 3.7e1f64;
            let ANQ = staged[147];
            let AOL = parameters[1123];
            let APB = staged[150];
            let APC = staged[151];
            let APH = staged[152];
            let API = staged[153];
            let APL = staged[154];
            let APM = staged[155];
            let APP = staged[156];
            let APQ = staged[157];
            let AQH = staged[158];
            let AQI = staged[159];
            let AQW = Lanes([0e0f64; 4]);
            let ASC = 1.6e1f64;
            let ASG = 5e-2f64;
            let ASQ = staged[160];
            let AST = staged[161];
            let ASV = staged[162];
            let ASX = staged[163];
            let ASZ = staged[164];
            let ATR = staged[165];
            let AUB = staged[166];
            let AUD = parameters[868];
            let AUI = staged[857];
            let AUJ = staged[168];
            let AUP = staged[171];
            let AUR = staged[172];
            let AUY = staged[173];
            let AVC = staged[174];
            let AVD = staged[175];
            let AVK = staged[179];
            let AVU = 1.804851387e-35f64;
            let AVV = Lanes([0e0f64; 3]);
            let AWA = staged[169];
            let AWB = staged[170];
            let AXT = staged[181];
            let AYB = 1.4142135623730951e0f64;
            let AYC = 3e0f64;
            let AYH = 6e0f64;
            let BAG = 2.01491e-1f64;
            let BAH = 4.02982e-1f64;
            let BAJ = 2.446562e0f64;
            let BCB = -1e0f64;
            let BCS = 1.804851387e-35f64;
            let BCT = Lanes([0e0f64; 5]);
            let BDH = -1e2f64;
            let BDI = 2e1f64;
            let BDO = 1.25e0f64;
            let BFW = staged[182];
            let BFX = staged[183];
            let BGZ = staged[184];
            let BHB = staged[185];
            let BHS = staged[191];
            let BHW = 1e-1f64;
            let BIG = staged[192];
            let BIO = staged[186];
            let BIP = staged[187];
            let BIQ = staged[188];
            let BIR = parameters[2];
            let BIV = staged[189];
            let BIW = staged[190];
            let BKA = staged[193];
            let BMQ = staged[858];
            let BNJ = -2e0f64;
            let BNO = -2e0f64;
            let BPE = -2e0f64;
            let BPJ = -2e0f64;
            let BQO = -2e0f64;
            let BQT = -2e0f64;
            let BSE = -2e0f64;
            let BSJ = -2e0f64;
            let BSV = staged[194];
            let BTA = parameters[1130];
            let BTC = parameters[1131];
            let BTE = parameters[1132];
            let BTJ = parameters[1133];
            let BXQ = -1e0f64;
            let BYJ = 1.804851387e-35f64;
            let BYX = -1e2f64;
            let CAM = 3.333333333333333e-1f64;
            let CAT = 8e-1f64;
            let CAU = 1.2e0f64;
            let CCX = staged[859];
            let CCY = staged[195];
            let CDR = staged[196];
            let CDX = staged[860];
            let CEZ = parameters[350];
            let CFG = staged[198];
            let CFK = staged[861];
            let CGR = staged[199];
            let CGS = staged[200];
            let CGX = 5.540622384e34f64;
            let CHE = staged[862];
            let CHY = staged[203];
            let CJS = node_potentials[8];
            let CJT = 1e0f64;
            let CKI = staged[204];
            let CLA = staged[206];
            let CLW = staged[863];
            let CMM = 1e-3f64;
            let CNN = staged[208];
            let CNY = parameters[36];
            let COB = staged[409];
            let COI = staged[864];
            let COJ = staged[865];
            let COK = Lanes([0e0f64; 4]);
            let COL = Lanes([0e0f64; 4]);
            let COS = Lanes([0e0f64; 2]);
            let COT = Lanes([0e0f64; 7]);
            let COU = Lanes([0e0f64; 7]);
            let CPD = staged[866];
            let CPK = parameters[1113];
            let CQJ = staged[210];
            let CQM = parameters[1101];
            let CQO = staged[211];
            let CQR = staged[867];
            let CQU = 0e0f64;
            let CQW = staged[868];
            let CRD = staged[869];
            let CRE = Lanes([0e0f64; 2]);
            let CRS = parameters[1127];
            let CRV = staged[871];
            let CRW = node_potentials[3];
            let CRY = 1e0f64;
            let CSC = parameters[1099];
            let CSH = parameters[515];
            let CSI = parameters[514];
            let CTM = parameters[1124];
            let CTN = parameters[1125];
            let CUE = parameters[1110];
            let CUM = staged[214];
            let CUO = staged[431];
            let CUQ = parameters[1122];
            let CUU = staged[215];
            let CUW = staged[432];
            let CVK = parameters[1107];
            let CVT = 0e0f64;
            let CVU = Lanes([0e0f64; 6]);
            let CWB = staged[873];
            let CWC = staged[874];
            let CWH = parameters[1109];
            let CWJ = parameters[517];
            let CWK = parameters[516];
            let CYB = parameters[1112];
            let CYJ = staged[218];
            let CYL = staged[435];
            let CYR = staged[219];
            let CYT = staged[436];
            let CZV = staged[220];
            let CZY = staged[221];
            let DAD = parameters[1108];
            let DAX = staged[225];
            let DBA = staged[226];
            let DBZ = staged[230];
            let DCC = staged[231];
            let DCX = parameters[1114];
            let DDA = staged[235];
            let DDR = Lanes([0e0f64; 5]);
            let DDS = Lanes([0e0f64; 4]);
            let DEB = staged[875];
            let DHM = -1e0f64;
            let DID = 1.804851387e-35f64;
            let DIR = -1e2f64;
            let DKG = staged[237];
            let DKN = staged[876];
            let DKO = parameters[1119];
            let DKR = staged[239];
            let DKV = staged[877];
            let DKY = staged[241];
            let DLE = staged[878];
            let DLF = staged[242];
            let DPK = -1e0f64;
            let DQC = 1.804851387e-35f64;
            let DQS = -1e2f64;
            let DSQ = staged[879];
            let DSZ = parameters[755];
            let DTB = staged[245];
            let DTE = staged[880];
            let DTH = staged[881];
            let DTK = staged[246];
            let DTT = staged[883];
            let DTU = 8e1f64;
            let DUR = staged[247];
            let DVU = staged[248];
            let DVZ = parameters[492];
            let DWB = parameters[493];
            let DWF = parameters[505];
            let DWG = parameters[506];
            let DXA = parameters[524];
            let DXW = Lanes([0e0f64; 9]);
            let DXZ = parameters[28];
            let DYC = staged[884];
            let DYN = staged[249];
            let DYQ = parameters[1104];
            let DYS = parameters[502];
            let DYT = staged[250];
            let DZA = parameters[504];
            let DZF = 2.5e-1f64;
            let DZN = staged[885];
            let DZO = node_potentials[0];
            let DZP = node_potentials[2];
            let DZQ = 1e0f64;
            let DZR = 1e0f64;
            let DZT = parameters[512];
            let DZW = parameters[503];
            let DZX = parameters[513];
            let EAJ = staged[252];
            let EAQ = parameters[507];
            let EAR = parameters[508];
            let EAU = parameters[509];
            let EAV = parameters[511];
            let EAW = parameters[510];
            let EAX = parameters[500];
            let ECY = 1e-4f64;
            let EDF = Lanes([0e0f64; 4]);
            let EDG = Lanes([0e0f64; 4]);
            let EDR = staged[282];
            let EEC = staged[886];
            let EED = staged[254];
            let EEP = staged[255];
            let EEQ = staged[257];
            let EES = staged[258];
            let EEW = staged[259];
            let EFD = staged[261];
            let EFU = staged[262];
            let EFV = staged[264];
            let EFX = staged[265];
            let EGB = staged[266];
            let EGQ = staged[267];
            let EGR = staged[269];
            let EGT = staged[58];
            let EHA = staged[270];
            let EHL = staged[271];
            let EIY = staged[887];
            let EIZ = staged[272];
            let EJA = staged[273];
            let EJI = staged[274];
            let EJK = staged[275];
            let EJO = staged[276];
            let EJR = staged[277];
            let EKK = staged[278];
            let EKL = staged[279];
            let EKT = staged[280];
            let EKY = staged[281];
            let ELS = staged[294];
            let EMR = staged[286];
            let ENJ = staged[287];
            let ENT = Lanes([0e0f64; 2]);
            let EPE = staged[292];
            let EPO = Lanes([0e0f64; 2]);
            let EQY = Lanes([0e0f64; 3]);
            let ERC = parameters[748];
            let ERJ = staged[295];
            let ERM = 1e3f64;
            let ESC = parameters[750];
            let ETA = parameters[752];
            let ETN = staged[296];
            let EUT = Lanes([0e0f64; 3]);
            let EUU = Lanes([0e0f64; 3]);
            let EVS = parameters[1128];
            let EXA = parameters[749];
            let EYK = parameters[751];
            let EZL = parameters[753];
            let EZR = staged[308];
            let FAP = staged[298];
            let FCW = 9e-1f64;
            let FDD = parameters[713];
            let FDG = staged[300];
            let FDI = 5e0f64;
            let FEB = staged[299];
            let FEU = parameters[715];
            let FEX = staged[303];
            let FFR = staged[302];
            let FGL = staged[893];
            let FGO = parameters[717];
            let FGR = staged[306];
            let FHL = staged[305];
            let FHW = staged[894];
            let FID = staged[309];
            let FIO = Lanes([0e0f64; 5]);
            let FIU = parameters[714];
            let FIX = staged[311];
            let FJC = staged[312];
            let FJS = staged[310];
            let FKL = parameters[716];
            let FKO = staged[314];
            let FKT = staged[315];
            let FLJ = staged[313];
            let FMB = staged[898];
            let FME = parameters[718];
            let FMH = staged[317];
            let FNB = staged[316];
            let FNQ = staged[899];
            let FNR = staged[319];
            let FRB = parameters[955];
            let FRE = staged[320];
            let FRH = staged[321];
            let FRJ = parameters[948];
            let FRK = parameters[949];
            let FRL = staged[322];
            let FRM = staged[323];
            let FRP = parameters[951];
            let FSB = parameters[950];
            let FSK = 1.60219e-19f64;
            let FSP = staged[900];
            let FSQ = staged[324];
            let FTD = parameters[799];
            let FTE = parameters[785];
            let FTV = staged[344];
            let FTW = parameters[811];
            let FTZ = staged[345];
            let FUA = parameters[812];
            let FUC = staged[346];
            let FUD = parameters[1043];
            let FUG = staged[347];
            let FUP = staged[902];
            let FVX = 1.804851387e-35f64;
            let FWC = -1e2f64;
            let FWO = staged[903];
            let FWS = staged[904];
            let FWT = staged[905];
            let FXA = Lanes([0e0f64; 6]);
            let FXB = Lanes([0e0f64; 6]);
            let FXC = staged[906];
            let FZU = 1.2e1f64;
            let GAC = 6e1f64;
            let GAH = 1.44e2f64;
            let GAM = 1.5e1f64;
            let GAO = staged[348];
            let GBC = node_potentials[16];
            let GBF = 1e0f64;
            let GBI = node_potentials[15];
            let GBL = 1e0f64;
            let GBO = ddt_scale();
            let GBZ = parameters[29];
            let GCN = staged[907];
            let GCS = staged[349];
            let GCZ = staged[352];
            let GDF = staged[354];
            let GDP = Lanes([0e0f64; 6]);
            let GEK = staged[908];
            let GEP = staged[356];
            let GGP = parameters[1137];
            let GJQ = 1.804851387e-35f64;
            let GKG = -1e2f64;
            let GNO = staged[357];
            let GOV = staged[909];
            let GOW = parameters[1129];
            let GPB = parameters[1134];
            let GPD = parameters[1135];
            let GPI = parameters[1136];
            let GTY = 1.804851387e-35f64;
            let GUO = -1e2f64;
            let GVR = staged[358];
            let GXI = parameters[136];
            let GZS = staged[359];
            let HDE = 2e-1f64;
            let HDQ = parameters[694];
            let HEB = parameters[208];
            let HEC = parameters[207];
            let HEE = staged[361];
            let HEJ = staged[365];
            let HEL = staged[366];
            let HEV = staged[912];
            let HEW = staged[367];
            let HEZ = staged[368];
            let HFG = 2e-2f64;
            let HFM = parameters[692];
            let HFO = parameters[693];
            let HFQ = staged[369];
            let HFT = staged[370];
            let HFV = staged[371];
            let HFX = staged[372];
            let HFY = staged[373];
            let HFZ = staged[374];
            let HGJ = parameters[690];
            let HGL = parameters[691];
            let HGN = staged[375];
            let HGQ = staged[376];
            let HGS = staged[377];
            let HGU = staged[378];
            let HGV = staged[379];
            let HHC = staged[381];
            let HHG = staged[913];
            let HHO = staged[383];
            let HID = staged[384];
            let HIG = staged[385];
            let HIH = staged[386];
            let HIR = staged[388];
            let HIS = staged[389];
            let HIU = staged[390];
            let HJW = staged[391];
            let HKC = staged[392];
            let HKE = staged[394];
            let HKJ = parameters[1016];
            let HKU = staged[396];
            let HLA = staged[398];
            let HLC = staged[399];
            let HLP = parameters[1014];
            let HPK = -1e0f64;
            let HQG = 1.804851387e-35f64;
            let HQU = -1e2f64;
            let HUF = -1e0f64;
            let HUW = 1.804851387e-35f64;
            let HVK = -1e2f64;
            let HWZ = parameters[957];
            let HXQ = staged[400];
            let HXR = staged[401];
            let HXV = staged[407];
            let HYI = staged[914];
            let HYN = staged[915];
            let IAJ = staged[380];
            let ICF = staged[916];
            let ICN = staged[917];
            let ICO = Lanes([0e0f64; 5]);
            let ICX = staged[918];
            let IDP = staged[919];
            let IDQ = Lanes([0e0f64; 5]);
            let IDZ = staged[920];
            let IEK = Lanes([0e0f64; 7]);
            let IEL = staged[921];
            let IEO = staged[922];
            let IEP = staged[923];
            let IES = 1e0f64;
            let IFD = Lanes([0e0f64; 6]);
            let IFG = staged[59];
            let IFM = Lanes([0e0f64; 10]);
            let IFR = staged[924];
            let IGS = staged[925];
            let IGW = staged[410];
            let IGX = staged[411];
            let IHZ = staged[412];
            let IIC = staged[413];
            let IIF = staged[414];
            let III = staged[415];
            let IIL = staged[416];
            let IIO = Lanes([0e0f64; 2]);
            let IIP = Lanes([0e0f64; 2]);
            let IIQ = Lanes([0e0f64; 2]);
            let IIR = Lanes([0e0f64; 2]);
            let IJJ = staged[926];
            let IKY = staged[927];
            let ILM = Lanes([0e0f64; 4]);
            let ILN = Lanes([0e0f64; 4]);
            let ILO = staged[417];
            let ILR = Lanes([0e0f64; 2]);
            let ILU = staged[418];
            let ILY = staged[419];
            let B = ctx.simparam_or("gmin", A);
            let O;
            let P;
            if K != 0.0 {
                O = L;
                P = M;
            } else {
                O = A;
                P = N;
            }
            let Q = O + staged[60];
            let S = R * Q;
            let T = P * R;
            let V = U / S;
            let X = ((T * V) * W) / S;
            let Z = Q / Y;
            let AA = P / Y;
            let AB = Q - Y;
            let AD = AC * Q;
            let AE = Q + parameters[822];
            let AF = (AD * Q) / AE;
            let AG = parameters[109] - AF;
            let AH = (((((P * AC) * Q) + (P * AD)) - (P * AF)) / AE) * W;
            let AI = Z.sqrt();
            let AM = AL * (Z * AI);
            let AP = AO * S;
            let AQ = AG / AP;
            let AR = (AG / AN) - AQ;
            let AS = rspice_limited_exp(AR);
            let AT = AM * AS;
            let AU = ((((AA * AI) + ((AA * (AK / (AJ * AI))) * Z)) * AL) * AS) + ((((AH / AN) - ((AH - ((T * AO) * AQ)) / AP)) * (rspice_limited_exp_derivative(AR))) * AM);
            let BI;
            let BJ;
            if K != 0.0 {
                let AW = AV / AT;
                let AY = if AW >= AX { AW } else { AX };
                let AZ = AY.ln();
                let BA = (((((AU * AW) * W) / AT) * (if AW >= AX { 1.0 } else { 0.0 })) * (AK / AY)) * AZ;
                let BC = ((AZ * AZ) + BB).sqrt();
                let BD = (BA + BA) * (AK / (AJ * BC));
                BI = BC;
                BJ = BD;
            } else {
                let BE = AV / AT;
                let BF = if BE >= AX { BE } else { AX };
                let BG = BF.ln();
                let BH = ((((AU * BE) * W) / AT) * (if BE >= AX { 1.0 } else { 0.0 })) * (AK / BF);
                BI = BG;
                BJ = BH;
            }
            let BY;
            let BZ;
            if K != 0.0 {
                let BK = AT * AT;
                let BL = AU * AT;
                let BM = staged[64] / BK;
                let BN = if BM >= AX { BM } else { AX };
                let BO = BN.ln();
                let BP = ((((((BL + BL) * BM) * W) / BK) * (if BM >= AX { 1.0 } else { 0.0 })) * (AK / BN)) * BO;
                let BQ = ((BO * BO) + BB).sqrt();
                let BR = (BP + BP) * (AK / (AJ * BQ));
                BY = BQ;
                BZ = BR;
            } else {
                let BS = AT * AT;
                let BT = AU * AT;
                let BU = staged[65] / BS;
                let BV = if BU >= AX { BU } else { AX };
                let BW = BV.ln();
                let BX = (((((BT + BT) * BU) * W) / BS) * (if BU >= AX { 1.0 } else { 0.0 })) * (AK / BV);
                BY = BW;
                BZ = BX;
            }
            let CF;
            let CG;
            if CA != 0.0 {
                let CD = (T * CB) * CC;
                let CE = ((CB * S) * CC) + parameters[5];
                CF = CE;
                CG = CD;
            } else {
                CF = A;
                CG = N;
            }
            let CJ = (CH + (S * BI)) + CI;
            let CK = if CJ >= CH { CJ } else { CH };
            let CL = ((T * BI) + (BJ * S)) * (if CJ >= CH { 1.0 } else { 0.0 });
            let CM = CK.sqrt();
            let CN = CL * (AK / (AJ * CM));
            let CO = Z - U;
            let CQ = AA * CP;
            let CR = U + (CP * CO);
            let CS = if CR < -1e1f64 { 1.0 } else { 0.0 };
            let DA;
            let DB;
            if CS != 0.0 {
                let CT = -1e-6f64 / CR;
                let CU = ((CQ * CT) * W) / CR;
                DA = CT;
                DB = CU;
            } else {
                let CV = CQ * CR;
                let CW = ((CR * CR) + 4e-6f64).sqrt();
                let CY = CX * (CR + CW);
                let CZ = (CQ + ((CV + CV) * (AK / (AJ * CW)))) * CX;
                DA = CY;
                DB = CZ;
            }
            let DD = DC * DA;
            let DE = DB * DC;
            let DG = AA * DF;
            let DH = U + (DF * CO);
            let DJ = DI * DH;
            let DK = DG * DI;
            let DO;
            let DP;
            if C != 0.0 {
                let DM = DL * DH;
                let DN = DG * DL;
                DO = DM;
                DP = DN;
            } else {
                DO = A;
                DP = N;
            }
            let DR = Z.powf(DQ);
            let DS = AA * (DQ * (Z.powf(staged[420])));
            let DU = DT * DR;
            let DV = DS * DT;
            let DX = P * DW;
            let DY = (U + (DW * AB)) - BB;
            let DZ = if DY < -1e1f64 { 1.0 } else { 0.0 };
            let EG;
            let EH;
            if DZ != 0.0 {
                let EA = -1e-6f64 / DY;
                let EB = ((DX * EA) * W) / DY;
                EG = EA;
                EH = EB;
            } else {
                let EC = DX * DY;
                let ED = ((DY * DY) + 4e-6f64).sqrt();
                let EE = CX * (DY + ED);
                let EF = (DX + ((EC + EC) * (AK / (AJ * ED)))) * CX;
                EG = EE;
                EH = EF;
            }
            let EJ = EI * EG;
            let EK = EH * EI;
            let EM = P * EL;
            let EN = (U + (EL * AB)) - BB;
            let EO = if EN < -1e1f64 { 1.0 } else { 0.0 };
            let EV;
            let EW;
            if EO != 0.0 {
                let EP = -1e-6f64 / EN;
                let EQ = ((EM * EP) * W) / EN;
                EV = EP;
                EW = EQ;
            } else {
                let ER = EM * EN;
                let ES = ((EN * EN) + 4e-6f64).sqrt();
                let ET = CX * (EN + ES);
                let EU = (EM + ((ER + ER) * (AK / (AJ * ES)))) * CX;
                EV = ET;
                EW = EU;
            }
            let EY = EX * EV;
            let EZ = EW * EX;
            let FB = Z.powf(FA);
            let FC = AA * (FA * (Z.powf(staged[421])));
            let FE = FD * FB;
            let FF = FC * FD;
            let FH = Z.powf(FG);
            let FI = AA * (FG * (Z.powf(staged[422])));
            let FK = FJ * FH;
            let FL = FI * FJ;
            let FN = AA * FM;
            let FO = U + (FM * CO);
            let FP = if FO < -1e1f64 { 1.0 } else { 0.0 };
            let FW;
            let FX;
            if FP != 0.0 {
                let FQ = -1e-6f64 / FO;
                let FR = ((FN * FQ) * W) / FO;
                FW = FQ;
                FX = FR;
            } else {
                let FS = FN * FO;
                let FT = ((FO * FO) + 4e-6f64).sqrt();
                let FU = CX * (FO + FT);
                let FV = (FN + ((FS + FS) * (AK / (AJ * FT)))) * CX;
                FW = FU;
                FX = FV;
            }
            let FZ = FY * FW;
            let GA = FX * FY;
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
            if C != 0.0 {
                let GC = GB * DR;
                let GD = DS * GB;
                let GE = if DY < -1e1f64 { 1.0 } else { 0.0 };
                let HG;
                let HH;
                if GE != 0.0 {
                    let HA = -1e-6f64 / DY;
                    let HB = ((DX * HA) * W) / DY;
                    HG = HA;
                    HH = HB;
                } else {
                    let HC = DX * DY;
                    let HD = ((DY * DY) + 4e-6f64).sqrt();
                    let HE = CX * (DY + HD);
                    let HF = (DX + ((HC + HC) * (AK / (AJ * HD)))) * CX;
                    HG = HE;
                    HH = HF;
                }
                let HJ = HI * HG;
                let HK = HH * HI;
                let HL = if EN < -1e1f64 { 1.0 } else { 0.0 };
                let HS;
                let HT;
                if HL != 0.0 {
                    let HM = -1e-6f64 / EN;
                    let HN = ((EM * HM) * W) / EN;
                    HS = HM;
                    HT = HN;
                } else {
                    let HO = EM * EN;
                    let HP = ((EN * EN) + 4e-6f64).sqrt();
                    let HQ = CX * (EN + HP);
                    let HR = (EM + ((HO + HO) * (AK / (AJ * HP)))) * CX;
                    HS = HQ;
                    HT = HR;
                }
                let HV = HU * HS;
                let HW = HT * HU;
                let HY = HX * FB;
                let HZ = FC * HX;
                let IB = IA * FH;
                let IC = FI * IA;
                GF = GC;
                GG = HJ;
                GH = HV;
                GI = HY;
                GJ = IB;
                GK = GD;
                GL = HK;
                GM = HW;
                GN = HZ;
                GO = IC;
            } else {
                GF = A;
                GG = A;
                GH = A;
                GI = A;
                GJ = A;
                GK = N;
                GL = N;
                GM = N;
                GN = N;
                GO = N;
            }
            let GQ = Z.powf(GP);
            let GR = AA * (GP * (Z.powf(staged[423])));
            let GT = Z.powf(GS);
            let GU = AA * (GS * (Z.powf(staged[424])));
            let GW = GV * GT;
            let GX = GU * GV;
            let GZ = if GW < GY { 1.0 } else { 0.0 };
            let ID;
            let IE;
            if GZ != 0.0 {
                ID = GY;
                IE = N;
            } else {
                ID = GW;
                IE = GX;
            }
            let IN;
            let IO;
            let IP;
            let IQ;
            if IF != 0.0 {
                let IH = Z.powf(IG);
                let II = AA * (IG * (Z.powf(staged[425])));
                let IL = IK * (Z.powf(IJ));
                let IM = (AA * (IJ * (Z.powf(staged[426])))) * IK;
                IN = IL;
                IO = IH;
                IP = IM;
                IQ = II;
            } else {
                IN = U;
                IO = U;
                IP = N;
                IQ = N;
            }
            let IV;
            let IW;
            if C != 0.0 {
                let IS = IR * GT;
                let IT = GU * IR;
                let IU = if IS < GY { 1.0 } else { 0.0 };
                let JB;
                let JC;
                if IU != 0.0 {
                    JB = GY;
                    JC = N;
                } else {
                    JB = IS;
                    JC = IT;
                }
                IV = JB;
                IW = JC;
            } else {
                IV = A;
                IW = N;
            }
            let IY = IX * GT;
            let IZ = GU * IX;
            let JA = if IY < GY { 1.0 } else { 0.0 };
            let JD;
            let JE;
            if JA != 0.0 {
                JD = GY;
                JE = N;
            } else {
                JD = IY;
                JE = IZ;
            }
            let JH = (P * JF) * JG;
            let JI = (JG * (U + (JF * AB))) - AO;
            let JJ = if JI < -1e1f64 { 1.0 } else { 0.0 };
            let JQ;
            let JR;
            if JJ != 0.0 {
                let JK = -1e-6f64 / JI;
                let JL = ((JH * JK) * W) / JI;
                JQ = JK;
                JR = JL;
            } else {
                let JM = JH * JI;
                let JN = ((JI * JI) + 4e-6f64).sqrt();
                let JO = CX * (JI + JN);
                let JP = (JH + ((JM + JM) * (AK / (AJ * JN)))) * CX;
                JQ = JO;
                JR = JP;
            }
            let JS = JQ + AO;
            let JT = U / JS;
            let JU = ((JR * JT) * W) / JS;
            let JW = (P * JV) * W;
            let JX = (U - (JV * AB)) - BB;
            let JY = if JX < -1e1f64 { 1.0 } else { 0.0 };
            let KF;
            let KG;
            if JY != 0.0 {
                let JZ = -1e-6f64 / JX;
                let KA = ((JW * JZ) * W) / JX;
                KF = JZ;
                KG = KA;
            } else {
                let KB = JW * JX;
                let KC = ((JX * JX) + 4e-6f64).sqrt();
                let KD = CX * (JX + KC);
                let KE = (JW + ((KB + KB) * (AK / (AJ * KC)))) * CX;
                KF = KD;
                KG = KE;
            }
            let KI = KH * KF;
            let KJ = KG * KH;
            let KL;
            let KM;
            if C != 0.0 {
                let KK = if JX < -1e1f64 { 1.0 } else { 0.0 };
                let KX;
                let KY;
                if KK != 0.0 {
                    let KR = -1e-6f64 / JX;
                    let KS = ((JW * KR) * W) / JX;
                    KX = KR;
                    KY = KS;
                } else {
                    let KT = JW * JX;
                    let KU = ((JX * JX) + 4e-6f64).sqrt();
                    let KV = CX * (JX + KU);
                    let KW = (JW + ((KT + KT) * (AK / (AJ * KU)))) * CX;
                    KX = KV;
                    KY = KW;
                }
                let LA = KZ * KX;
                let LB = KY * KZ;
                KL = LA;
                KM = LB;
            } else {
                KL = A;
                KM = N;
            }
            let KO = P * KN;
            let KP = (U + (KN * AB)) - BB;
            let KQ = if KP < -1e1f64 { 1.0 } else { 0.0 };
            let LI;
            let LJ;
            if KQ != 0.0 {
                let LC = -1e-6f64 / KP;
                let LD = ((KO * LC) * W) / KP;
                LI = LC;
                LJ = LD;
            } else {
                let LE = KO * KP;
                let LF = ((KP * KP) + 4e-6f64).sqrt();
                let LG = CX * (KP + LF);
                let LH = (KO + ((LE + LE) * (AK / (AJ * LF)))) * CX;
                LI = LG;
                LJ = LH;
            }
            let LL = LK * LI;
            let LM = LJ * LK;
            let LO = P * LN;
            let LP = (U + (LN * AB)) - BB;
            let LQ = if LP < -1e1f64 { 1.0 } else { 0.0 };
            let LX;
            let LY;
            if LQ != 0.0 {
                let LR = -1e-6f64 / LP;
                let LS = ((LO * LR) * W) / LP;
                LX = LR;
                LY = LS;
            } else {
                let LT = LO * LP;
                let LU = ((LP * LP) + 4e-6f64).sqrt();
                let LV = CX * (LP + LU);
                let LW = (LO + ((LT + LT) * (AK / (AJ * LU)))) * CX;
                LX = LV;
                LY = LW;
            }
            let MA = LZ * LX;
            let MB = LY * LZ;
            let MD = Z.powf(MC);
            let ME = AA * (MC * (Z.powf(staged[427])));
            let MG = MF * MD;
            let MH = ME * MF;
            let ML;
            let MM;
            if C != 0.0 {
                let MJ = MI * MD;
                let MK = ME * MI;
                ML = MJ;
                MM = MK;
            } else {
                ML = A;
                MM = N;
            }
            let MO = P * MN;
            let MP = (U + (MN * AB)) - BB;
            let MQ = if MP < -1e1f64 { 1.0 } else { 0.0 };
            let MX;
            let MY;
            if MQ != 0.0 {
                let MR = -1e-6f64 / MP;
                let MS = ((MO * MR) * W) / MP;
                MX = MR;
                MY = MS;
            } else {
                let MT = MO * MP;
                let MU = ((MP * MP) + 4e-6f64).sqrt();
                let MV = CX * (MP + MU);
                let MW = (MO + ((MT + MT) * (AK / (AJ * MU)))) * CX;
                MX = MV;
                MY = MW;
            }
            let NA = MZ * MX;
            let NB = MY * MZ;
            let NC = if MP < -1e1f64 { 1.0 } else { 0.0 };
            let NJ;
            let NK;
            if NC != 0.0 {
                let ND = -1e-6f64 / MP;
                let NE = ((MO * ND) * W) / MP;
                NJ = ND;
                NK = NE;
            } else {
                let NF = MO * MP;
                let NG = ((MP * MP) + 4e-6f64).sqrt();
                let NH = CX * (MP + NG);
                let NI = (MO + ((NF + NF) * (AK / (AJ * NG)))) * CX;
                NJ = NH;
                NK = NI;
            }
            let NM = NL * NJ;
            let NN = NK * NL;
            let NO = if Z >= AX { Z } else { AX };
            let NP = NO.ln();
            let NQ = (AA * (if Z >= AX { 1.0 } else { 0.0 })) * (AK / NO);
            let NS = NR * NP;
            let NT = rspice_limited_exp(NS);
            let NU = (NQ * NR) * (rspice_limited_exp_derivative(NS));
            let NW = P * NV;
            let NX = (U + (NV * AB)) - BB;
            let NY = if NX < -1e1f64 { 1.0 } else { 0.0 };
            let OF;
            let OG;
            if NY != 0.0 {
                let NZ = -1e-6f64 / NX;
                let OA = ((NW * NZ) * W) / NX;
                OF = NZ;
                OG = OA;
            } else {
                let OB = NW * NX;
                let OC = ((NX * NX) + 4e-6f64).sqrt();
                let OD = CX * (NX + OC);
                let OE = (NW + ((OB + OB) * (AK / (AJ * OC)))) * CX;
                OF = OD;
                OG = OE;
            }
            let OI = OH * OF;
            let OJ = OG * OH;
            let OL = P * OK;
            let OM = (U + (OK * AB)) - BB;
            let ON = if OM < -1e1f64 { 1.0 } else { 0.0 };
            let OU;
            let OV;
            if ON != 0.0 {
                let OO = -1e-6f64 / OM;
                let OP = ((OL * OO) * W) / OM;
                OU = OO;
                OV = OP;
            } else {
                let OQ = OL * OM;
                let OR = ((OM * OM) + 4e-6f64).sqrt();
                let OS = CX * (OM + OR);
                let OT = (OL + ((OQ + OQ) * (AK / (AJ * OR)))) * CX;
                OU = OS;
                OV = OT;
            }
            let OX = OW * OU;
            let OY = OV * OW;
            let PA = P * OZ;
            let PB = (U + (OZ * AB)) - BB;
            let PC = if PB < -1e1f64 { 1.0 } else { 0.0 };
            let PJ;
            let PK;
            if PC != 0.0 {
                let PD = -1e-6f64 / PB;
                let PE = ((PA * PD) * W) / PB;
                PJ = PD;
                PK = PE;
            } else {
                let PF = PA * PB;
                let PG = ((PB * PB) + 4e-6f64).sqrt();
                let PH = CX * (PB + PG);
                let PI = (PA + ((PF + PF) * (AK / (AJ * PG)))) * CX;
                PJ = PH;
                PK = PI;
            }
            let PM = PL * PJ;
            let PN = PK * PL;
            let PP = P * PO;
            let PQ = (U + (PO * AB)) - BB;
            let PR = if PQ < -1e1f64 { 1.0 } else { 0.0 };
            let PY;
            let PZ;
            if PR != 0.0 {
                let PS = -1e-6f64 / PQ;
                let PT = ((PP * PS) * W) / PQ;
                PY = PS;
                PZ = PT;
            } else {
                let PU = PP * PQ;
                let PV = ((PQ * PQ) + 4e-6f64).sqrt();
                let PW = CX * (PQ + PV);
                let PX = (PP + ((PU + PU) * (AK / (AJ * PV)))) * CX;
                PY = PW;
                PZ = PX;
            }
            let QB = QA * PY;
            let QC = PZ * QA;
            let QE = P * QD;
            let QF = (U + (QD * AB)) - BB;
            let QG = if QF < -1e1f64 { 1.0 } else { 0.0 };
            let QN;
            let QO;
            if QG != 0.0 {
                let QH = -1e-6f64 / QF;
                let QI = ((QE * QH) * W) / QF;
                QN = QH;
                QO = QI;
            } else {
                let QJ = QE * QF;
                let QK = ((QF * QF) + 4e-6f64).sqrt();
                let QL = CX * (QF + QK);
                let QM = (QE + ((QJ + QJ) * (AK / (AJ * QK)))) * CX;
                QN = QL;
                QO = QM;
            }
            let QQ = QP * QN;
            let QR = QO * QP;
            let QT = P * QS;
            let QU = (U + (QS * AB)) - BB;
            let QV = if QU < -1e1f64 { 1.0 } else { 0.0 };
            let RC;
            let RD;
            if QV != 0.0 {
                let QW = -1e-6f64 / QU;
                let QX = ((QT * QW) * W) / QU;
                RC = QW;
                RD = QX;
            } else {
                let QY = QT * QU;
                let QZ = ((QU * QU) + 4e-6f64).sqrt();
                let RA = CX * (QU + QZ);
                let RB = (QT + ((QY + QY) * (AK / (AJ * QZ)))) * CX;
                RC = RA;
                RD = RB;
            }
            let RF = RE * RC;
            let RG = RD * RE;
            let RH = if QU < -1e1f64 { 1.0 } else { 0.0 };
            let RO;
            let RP;
            if RH != 0.0 {
                let RI = -1e-6f64 / QU;
                let RJ = ((QT * RI) * W) / QU;
                RO = RI;
                RP = RJ;
            } else {
                let RK = QT * QU;
                let RL = ((QU * QU) + 4e-6f64).sqrt();
                let RM = CX * (QU + RL);
                let RN = (QT + ((RK + RK) * (AK / (AJ * RL)))) * CX;
                RO = RM;
                RP = RN;
            }
            let RR = RQ * RO;
            let RS = RP * RQ;
            let RU = P * RT;
            let RV = (U + (RT * AB)) - BB;
            let RW = if RV < -1e1f64 { 1.0 } else { 0.0 };
            let SD;
            let SE;
            if RW != 0.0 {
                let RX = -1e-6f64 / RV;
                let RY = ((RU * RX) * W) / RV;
                SD = RX;
                SE = RY;
            } else {
                let RZ = RU * RV;
                let SA = ((RV * RV) + 4e-6f64).sqrt();
                let SB = CX * (RV + SA);
                let SC = (RU + ((RZ + RZ) * (AK / (AJ * SA)))) * CX;
                SD = SB;
                SE = SC;
            }
            let SG = SF * SD;
            let SH = SE * SF;
            let SI = if RV < -1e1f64 { 1.0 } else { 0.0 };
            let SP;
            let SQ;
            if SI != 0.0 {
                let SJ = -1e-6f64 / RV;
                let SK = ((RU * SJ) * W) / RV;
                SP = SJ;
                SQ = SK;
            } else {
                let SL = RU * RV;
                let SM = ((RV * RV) + 4e-6f64).sqrt();
                let SN = CX * (RV + SM);
                let SO = (RU + ((SL + SL) * (AK / (AJ * SM)))) * CX;
                SP = SN;
                SQ = SO;
            }
            let SS = SR * SP;
            let ST = SQ * SR;
            let SV = P * SU;
            let SW = (U + (SU * AB)) - BB;
            let SX = if SW < -1e1f64 { 1.0 } else { 0.0 };
            let TE;
            let TF;
            if SX != 0.0 {
                let SY = -1e-6f64 / SW;
                let SZ = ((SV * SY) * W) / SW;
                TE = SY;
                TF = SZ;
            } else {
                let TA = SV * SW;
                let TB = ((SW * SW) + 4e-6f64).sqrt();
                let TC = CX * (SW + TB);
                let TD = (SV + ((TA + TA) * (AK / (AJ * TB)))) * CX;
                TE = TC;
                TF = TD;
            }
            let TH = TG * TE;
            let TI = TF * TG;
            let TJ = if SW < -1e1f64 { 1.0 } else { 0.0 };
            let TQ;
            let TR;
            if TJ != 0.0 {
                let TK = -1e-6f64 / SW;
                let TL = ((SV * TK) * W) / SW;
                TQ = TK;
                TR = TL;
            } else {
                let TM = SV * SW;
                let TN = ((SW * SW) + 4e-6f64).sqrt();
                let TO = CX * (SW + TN);
                let TP = (SV + ((TM + TM) * (AK / (AJ * TN)))) * CX;
                TQ = TO;
                TR = TP;
            }
            let TT = TS * TQ;
            let TU = TR * TS;
            let TW = TV * AB;
            let TX = (P * TV) * W;
            let TZ = (parameters[707] - TW) - TY;
            let UA = if TZ < -1e1f64 { 1.0 } else { 0.0 };
            let UH;
            let UI;
            if UA != 0.0 {
                let UB = -1e-6f64 / TZ;
                let UC = ((TX * UB) * W) / TZ;
                UH = UB;
                UI = UC;
            } else {
                let UD = TX * TZ;
                let UE = ((TZ * TZ) + 4e-6f64).sqrt();
                let UF = CX * (TZ + UE);
                let UG = (TX + ((UD + UD) * (AK / (AJ * UE)))) * CX;
                UH = UF;
                UI = UG;
            }
            let UJ = UH + TY;
            let UK = (parameters[708] - TW) - TY;
            let UL = if UK < -1e1f64 { 1.0 } else { 0.0 };
            let US;
            let UT;
            if UL != 0.0 {
                let UM = -1e-6f64 / UK;
                let UN = ((TX * UM) * W) / UK;
                US = UM;
                UT = UN;
            } else {
                let UO = TX * UK;
                let UP = ((UK * UK) + 4e-6f64).sqrt();
                let UQ = CX * (UK + UP);
                let UR = (TX + ((UO + UO) * (AK / (AJ * UP)))) * CX;
                US = UQ;
                UT = UR;
            }
            let UU = US + TY;
            let UW = UV * AB;
            let UX = (P * UV) * W;
            let UY = (parameters[709] - UW) - TY;
            let UZ = if UY < -1e1f64 { 1.0 } else { 0.0 };
            let VG;
            let VH;
            if UZ != 0.0 {
                let VA = -1e-6f64 / UY;
                let VB = ((UX * VA) * W) / UY;
                VG = VA;
                VH = VB;
            } else {
                let VC = UX * UY;
                let VD = ((UY * UY) + 4e-6f64).sqrt();
                let VE = CX * (UY + VD);
                let VF = (UX + ((VC + VC) * (AK / (AJ * VD)))) * CX;
                VG = VE;
                VH = VF;
            }
            let VI = VG + TY;
            let VJ = (parameters[710] - UW) - TY;
            let VK = if VJ < -1e1f64 { 1.0 } else { 0.0 };
            let VR;
            let VS;
            if VK != 0.0 {
                let VL = -1e-6f64 / VJ;
                let VM = ((UX * VL) * W) / VJ;
                VR = VL;
                VS = VM;
            } else {
                let VN = UX * VJ;
                let VO = ((VJ * VJ) + 4e-6f64).sqrt();
                let VP = CX * (VJ + VO);
                let VQ = (UX + ((VN + VN) * (AK / (AJ * VO)))) * CX;
                VR = VP;
                VS = VQ;
            }
            let VT = VR + TY;
            let VV = VU * AB;
            let VW = (P * VU) * W;
            let VX = (parameters[711] - VV) - TY;
            let VY = if VX < -1e1f64 { 1.0 } else { 0.0 };
            let WF;
            let WG;
            if VY != 0.0 {
                let VZ = -1e-6f64 / VX;
                let WA = ((VW * VZ) * W) / VX;
                WF = VZ;
                WG = WA;
            } else {
                let WB = VW * VX;
                let WC = ((VX * VX) + 4e-6f64).sqrt();
                let WD = CX * (VX + WC);
                let WE = (VW + ((WB + WB) * (AK / (AJ * WC)))) * CX;
                WF = WD;
                WG = WE;
            }
            let WH = WF + TY;
            let WI = (parameters[712] - VV) - TY;
            let WJ = if WI < -1e1f64 { 1.0 } else { 0.0 };
            let WQ;
            let WR;
            if WJ != 0.0 {
                let WK = -1e-6f64 / WI;
                let WL = ((VW * WK) * W) / WI;
                WQ = WK;
                WR = WL;
            } else {
                let WM = VW * WI;
                let WN = ((WI * WI) + 4e-6f64).sqrt();
                let WO = CX * (WI + WN);
                let WP = (VW + ((WM + WM) * (AK / (AJ * WN)))) * CX;
                WQ = WO;
                WR = WP;
            }
            let WS = WQ + TY;
            let WT = AG / S;
            let WU = staged[123] - WT;
            let WV = ((AH - (T * WT)) / S) * W;
            let WY = (WU + (WW * NP)) / WX;
            let WZ = rspice_limited_exp(WY);
            let XA = ((WV + (NQ * WW)) / WX) * (rspice_limited_exp_derivative(WY));
            let XC = XB * WZ;
            let XD = XA * XB;
            let XF = XE * WZ;
            let XG = XA * XE;
            let XI = XH * WZ;
            let XJ = XA * XH;
            let XM = (WU + (XK * NP)) / XL;
            let XN = rspice_limited_exp(XM);
            let XO = ((WV + (NQ * XK)) / XL) * (rspice_limited_exp_derivative(XM));
            let XQ = XP * XN;
            let XR = XO * XP;
            let XT = XS * XN;
            let XU = XO * XS;
            let XW = XV * XN;
            let XX = XO * XV;
            let XZ = (XY * CO) / S;
            let YB = YA * (rspice_limited_exp(XZ));
            let YC = ((((AA * XY) - (T * XZ)) / S) * (rspice_limited_exp_derivative(XZ))) * YA;
            let YE = (YD * CO) / S;
            let YG = YF * (rspice_limited_exp(YE));
            let YH = ((((AA * YD) - (T * YE)) / S) * (rspice_limited_exp_derivative(YE))) * YF;
            let YJ = (YI * CO) / S;
            let YL = YK * (rspice_limited_exp(YJ));
            let YM = ((((AA * YI) - (T * YJ)) / S) * (rspice_limited_exp_derivative(YJ))) * YK;
            let YO = (YN * CO) / S;
            let YQ = YP * (rspice_limited_exp(YO));
            let YR = ((((AA * YN) - (T * YO)) / S) * (rspice_limited_exp_derivative(YO))) * YP;
            let YT = (YS * CO) / S;
            let YV = YU * (rspice_limited_exp(YT));
            let YW = ((((AA * YS) - (T * YT)) / S) * (rspice_limited_exp_derivative(YT))) * YU;
            let YY = (YX * CO) / S;
            let ZA = YZ * (rspice_limited_exp(YY));
            let ZB = ((((AA * YX) - (T * YY)) / S) * (rspice_limited_exp_derivative(YY))) * YZ;
            let ZE = (AA * ZC) * ZD;
            let ZF = (ZD * (U + (ZC * CO))) - TY;
            let ZG = if ZF < -1e1f64 { 1.0 } else { 0.0 };
            let ZN;
            let ZO;
            if ZG != 0.0 {
                let ZH = -1e-6f64 / ZF;
                let ZI = ((ZE * ZH) * W) / ZF;
                ZN = ZH;
                ZO = ZI;
            } else {
                let ZJ = ZE * ZF;
                let ZK = ((ZF * ZF) + 4e-6f64).sqrt();
                let ZL = CX * (ZF + ZK);
                let ZM = (ZE + ((ZJ + ZJ) * (AK / (AJ * ZK)))) * CX;
                ZN = ZL;
                ZO = ZM;
            }
            let ZP = ZN + TY;
            let ZS = (AA * ZQ) * ZR;
            let ZT = (ZR * (U + (ZQ * CO))) - TY;
            let ZU = if ZT < -1e1f64 { 1.0 } else { 0.0 };
            let AAB;
            let AAC;
            if ZU != 0.0 {
                let ZV = -1e-6f64 / ZT;
                let ZW = ((ZS * ZV) * W) / ZT;
                AAB = ZV;
                AAC = ZW;
            } else {
                let ZX = ZS * ZT;
                let ZY = ((ZT * ZT) + 4e-6f64).sqrt();
                let ZZ = CX * (ZT + ZY);
                let AAA = (ZS + ((ZX + ZX) * (AK / (AJ * ZY)))) * CX;
                AAB = ZZ;
                AAC = AAA;
            }
            let AAD = AAB + TY;
            let AAG = (AA * AAE) * AAF;
            let AAH = (AAF * (U + (AAE * CO))) - TY;
            let AAI = if AAH < -1e1f64 { 1.0 } else { 0.0 };
            let AAP;
            let AAQ;
            if AAI != 0.0 {
                let AAJ = -1e-6f64 / AAH;
                let AAK = ((AAG * AAJ) * W) / AAH;
                AAP = AAJ;
                AAQ = AAK;
            } else {
                let AAL = AAG * AAH;
                let AAM = ((AAH * AAH) + 4e-6f64).sqrt();
                let AAN = CX * (AAH + AAM);
                let AAO = (AAG + ((AAL + AAL) * (AK / (AJ * AAM)))) * CX;
                AAP = AAN;
                AAQ = AAO;
            }
            let AAR = AAP + TY;
            let AAU = (AA * AAS) * AAT;
            let AAV = (AAT * (U + (AAS * CO))) - TY;
            let AAW = if AAV < -1e1f64 { 1.0 } else { 0.0 };
            let ABD;
            let ABE;
            if AAW != 0.0 {
                let AAX = -1e-6f64 / AAV;
                let AAY = ((AAU * AAX) * W) / AAV;
                ABD = AAX;
                ABE = AAY;
            } else {
                let AAZ = AAU * AAV;
                let ABA = ((AAV * AAV) + 4e-6f64).sqrt();
                let ABB = CX * (AAV + ABA);
                let ABC = (AAU + ((AAZ + AAZ) * (AK / (AJ * ABA)))) * CX;
                ABD = ABB;
                ABE = ABC;
            }
            let ABF = ABD + TY;
            let ABI = (AA * ABG) * ABH;
            let ABJ = (ABH * (U + (ABG * CO))) - TY;
            let ABK = if ABJ < -1e1f64 { 1.0 } else { 0.0 };
            let ABR;
            let ABS;
            if ABK != 0.0 {
                let ABL = -1e-6f64 / ABJ;
                let ABM = ((ABI * ABL) * W) / ABJ;
                ABR = ABL;
                ABS = ABM;
            } else {
                let ABN = ABI * ABJ;
                let ABO = ((ABJ * ABJ) + 4e-6f64).sqrt();
                let ABP = CX * (ABJ + ABO);
                let ABQ = (ABI + ((ABN + ABN) * (AK / (AJ * ABO)))) * CX;
                ABR = ABP;
                ABS = ABQ;
            }
            let ABT = ABR + TY;
            let ABW = (AA * ABU) * ABV;
            let ABX = (ABV * (U + (ABU * CO))) - TY;
            let ABY = if ABX < -1e1f64 { 1.0 } else { 0.0 };
            let ACF;
            let ACG;
            if ABY != 0.0 {
                let ABZ = -1e-6f64 / ABX;
                let ACA = ((ABW * ABZ) * W) / ABX;
                ACF = ABZ;
                ACG = ACA;
            } else {
                let ACB = ABW * ABX;
                let ACC = ((ABX * ABX) + 4e-6f64).sqrt();
                let ACD = CX * (ABX + ACC);
                let ACE = (ABW + ((ACB + ACB) * (AK / (AJ * ACC)))) * CX;
                ACF = ACD;
                ACG = ACE;
            }
            let ACH = ACF + TY;
            let ACL = ((ACI * XC) + (ACJ * XF)) + (ACK * XI);
            let ACM = ((XD * ACI) + (XG * ACJ)) + (XJ * ACK);
            let ACN = if ACL > A { 1.0 } else { 0.0 };
            let ADX;
            let ADY;
            let ADZ;
            let AEA;
            let AEB;
            let AEC;
            let AED;
            let AEE;
            let AEF;
            let AEG;
            let AEH;
            let AEI;
            let AEJ;
            let AEK;
            let AEL;
            let AEM;
            if ACN != 0.0 {
                let ACO = S * WX;
                let ACP = T * WX;
                let ACR = -ACQ;
                let ACS = ACR / ACO;
                let ACU = (rspice_limited_exp(ACS)) * ACT;
                let ACV = ((((ACP * ACS) * W) / ACO) * (rspice_limited_exp_derivative(ACS))) * ACT;
                let ACW = parameters[727] / ACL;
                let ACY = (U + (if ACW >= ACX { ACW } else { ACX })) - ACU;
                let ACZ = ((((ACM * ACW) * W) / ACL) * (if ACW >= ACX { 1.0 } else { 0.0 })) - ACV;
                let ADA = ACZ * ACY;
                let ADC = ((ACY * ACY) + (ADB * ACU)).sqrt();
                let ADD = CX * (ACY + ADC);
                let ADE = if ADD >= AX { ADD } else { AX };
                let ADF = ADE.ln();
                let ADG = ACO * ADF;
                let ADH = (ACP * ADF) + (((((ACZ + (((ADA + ADA) + (ACV * ADB)) * (AK / (AJ * ADC)))) * CX) * (if ADD >= AX { 1.0 } else { 0.0 })) * (AK / ADE)) * ACO);
                let ADI = ADG / ACO;
                let ADJ = rspice_limited_exp(ADI);
                let ADK = ((ADH - (ACP * ADI)) / ACO) * (rspice_limited_exp_derivative(ADI));
                let ADL = ACU / ADJ;
                let ADM = (ACV - (ADK * ADL)) / ADJ;
                let ADN = ((ADJ - ADL) + ACU) - U;
                let ADO = ACL * ADN;
                let ADP = (ACM * ADN) + (((ADK - ADM) + ACV) * ACL);
                let ADQ = ADJ + ADL;
                let ADR = (ACL * ADQ) / ACO;
                let ADS = (((ACM * ADQ) + ((ADK + ADM) * ACL)) - (ACP * ADR)) / ACO;
                let ADT = parameters[729] / ACL;
                let ADU = ((ACM * ADT) * W) / ACL;
                let ADV = ADT - ACX;
                let ADW = if ADV < -1e1f64 { 1.0 } else { 0.0 };
                let AEY;
                let AEZ;
                if ADW != 0.0 {
                    let AES = -1e-6f64 / ADV;
                    let AET = ((ADU * AES) * W) / ADV;
                    AEY = AES;
                    AEZ = AET;
                } else {
                    let AEU = ADU * ADV;
                    let AEV = ((ADV * ADV) + 4e-6f64).sqrt();
                    let AEW = CX * (ADV + AEV);
                    let AEX = (ADU + ((AEU + AEU) * (AK / (AJ * AEV)))) * CX;
                    AEY = AEW;
                    AEZ = AEX;
                }
                let AFA = ((AEY + ACX) - U) / ACT;
                let AFB = if AFA >= AX { AFA } else { AX };
                let AFC = AFB.ln();
                let AFD = ACR - (ACO * AFC);
                let AFE = ((ACP * AFC) + ((((AEZ / ACT) * (if AFA >= AX { 1.0 } else { 0.0 })) * (AK / AFB)) * ACO)) * W;
                let AFF = (-(ACQ + AFD)) / ACO;
                let AFG = ACT * (rspice_limited_exp(AFF));
                let AFH = ((((AFE * W) - (ACP * AFF)) / ACO) * (rspice_limited_exp_derivative(AFF))) * ACT;
                let AFI = U + AFG;
                let AFJ = ACL * AFI;
                let AFK = (ACM * AFI) + (AFH * ACL);
                let AFL = -ACL;
                let AFM = (AFL * AFG) / ACO;
                let AFN = ((((ACM * W) * AFG) + (AFH * AFL)) - (ACP * AFM)) / ACO;
                ADX = ACO;
                ADY = AFJ;
                ADZ = AFM;
                AEA = AFD;
                AEB = ACU;
                AEC = ADO;
                AED = ADR;
                AEE = ADG;
                AEF = ACP;
                AEG = AFK;
                AEH = AFN;
                AEI = AFE;
                AEJ = ACV;
                AEK = ADP;
                AEL = ADS;
                AEM = ADH;
            } else {
                ADX = A;
                ADY = A;
                ADZ = A;
                AEA = A;
                AEB = A;
                AEC = A;
                AED = A;
                AEE = A;
                AEF = N;
                AEG = N;
                AEH = N;
                AEI = N;
                AEJ = N;
                AEK = N;
                AEL = N;
                AEM = N;
            }
            let AEP = ((AEN * XQ) + (AEO * XT)) + (ACK * XW);
            let AEQ = ((XR * AEN) + (XU * AEO)) + (XX * ACK);
            let AER = if AEP > A { 1.0 } else { 0.0 };
            let AGV;
            let AGW;
            let AGX;
            let AGY;
            let AGZ;
            let AHA;
            let AHB;
            let AHC;
            let AHD;
            let AHE;
            let AHF;
            let AHG;
            let AHH;
            let AHI;
            let AHJ;
            let AHK;
            if AER != 0.0 {
                let AFO = S * XL;
                let AFP = T * XL;
                let AFR = -AFQ;
                let AFS = AFR / AFO;
                let AFU = (rspice_limited_exp(AFS)) * AFT;
                let AFV = ((((AFP * AFS) * W) / AFO) * (rspice_limited_exp_derivative(AFS))) * AFT;
                let AFW = parameters[728] / AEP;
                let AFX = (U + (if AFW >= ACX { AFW } else { ACX })) - AFU;
                let AFY = ((((AEQ * AFW) * W) / AEP) * (if AFW >= ACX { 1.0 } else { 0.0 })) - AFV;
                let AFZ = AFY * AFX;
                let AGA = ((AFX * AFX) + (ADB * AFU)).sqrt();
                let AGB = CX * (AFX + AGA);
                let AGC = if AGB >= AX { AGB } else { AX };
                let AGD = AGC.ln();
                let AGE = AFO * AGD;
                let AGF = (AFP * AGD) + (((((AFY + (((AFZ + AFZ) + (AFV * ADB)) * (AK / (AJ * AGA)))) * CX) * (if AGB >= AX { 1.0 } else { 0.0 })) * (AK / AGC)) * AFO);
                let AGG = AGE / AFO;
                let AGH = rspice_limited_exp(AGG);
                let AGI = ((AGF - (AFP * AGG)) / AFO) * (rspice_limited_exp_derivative(AGG));
                let AGJ = AFU / AGH;
                let AGK = (AFV - (AGI * AGJ)) / AGH;
                let AGL = ((AGH - AGJ) + AFU) - U;
                let AGM = AEP * AGL;
                let AGN = (AEQ * AGL) + (((AGI - AGK) + AFV) * AEP);
                let AGO = AGH + AGJ;
                let AGP = (AEP * AGO) / AFO;
                let AGQ = (((AEQ * AGO) + ((AGI + AGK) * AEP)) - (AFP * AGP)) / AFO;
                let AGR = parameters[730] / AEP;
                let AGS = ((AEQ * AGR) * W) / AEP;
                let AGT = AGR - ACX;
                let AGU = if AGT < -1e1f64 { 1.0 } else { 0.0 };
                let AHS;
                let AHT;
                if AGU != 0.0 {
                    let AHM = -1e-6f64 / AGT;
                    let AHN = ((AGS * AHM) * W) / AGT;
                    AHS = AHM;
                    AHT = AHN;
                } else {
                    let AHO = AGS * AGT;
                    let AHP = ((AGT * AGT) + 4e-6f64).sqrt();
                    let AHQ = CX * (AGT + AHP);
                    let AHR = (AGS + ((AHO + AHO) * (AK / (AJ * AHP)))) * CX;
                    AHS = AHQ;
                    AHT = AHR;
                }
                let AHU = ((AHS + ACX) - U) / AFT;
                let AHV = if AHU >= AX { AHU } else { AX };
                let AHW = AHV.ln();
                let AHX = AFR - (AFO * AHW);
                let AHY = ((AFP * AHW) + ((((AHT / AFT) * (if AHU >= AX { 1.0 } else { 0.0 })) * (AK / AHV)) * AFO)) * W;
                let AHZ = (-(AFQ + AHX)) / AFO;
                let AIA = AFT * (rspice_limited_exp(AHZ));
                let AIB = ((((AHY * W) - (AFP * AHZ)) / AFO) * (rspice_limited_exp_derivative(AHZ))) * AFT;
                let AIC = U + AIA;
                let AID = AEP * AIC;
                let AIE = (AEQ * AIC) + (AIB * AEP);
                let AIF = -AEP;
                let AIG = (AIF * AIA) / AFO;
                let AIH = ((((AEQ * W) * AIA) + (AIB * AIF)) - (AFP * AIG)) / AFO;
                AGV = AFO;
                AGW = AID;
                AGX = AIG;
                AGY = AHX;
                AGZ = AFU;
                AHA = AGM;
                AHB = AGP;
                AHC = AGE;
                AHD = AFP;
                AHE = AIE;
                AHF = AIH;
                AHG = AHY;
                AHH = AFV;
                AHI = AGN;
                AHJ = AGQ;
                AHK = AGF;
            } else {
                AGV = A;
                AGW = A;
                AGX = A;
                AGY = A;
                AGZ = A;
                AHA = A;
                AHB = A;
                AHC = A;
                AHD = N;
                AHE = N;
                AHF = N;
                AHG = N;
                AHH = N;
                AHI = N;
                AHJ = N;
                AHK = N;
            }
            let AIM;
            let AIN;
            let AIO;
            let AIP;
            let AIQ;
            let AIR;
            if AHL != 0.0 {
                let AIK = (AA * AII) * AIJ;
                let AIL = (AIJ * (U + (AII * CO))) + 1e-9f64;
                loop {
                    if AIS == 0.0 {
                        break;
                    }
                }
                let AIT = parameters[915] / AIL;
                let AIU = ((AIK * AIT) * W) / AIL;
                let AIW = AIT * AIV;
                let AIX = AIU * AIV;
                let AIZ = AIT * AIY;
                let AJA = AIU * AIY;
                let AJB = U + AIW;
                let AJC = (U + AIZ) / AJB;
                let AJE = U + (AIW * AJD);
                let AJF = (U + (AIZ * AJD)) / AJE;
                let AJG = DU * AJC;
                let AJH = (DV * AJC) + (((AJA - (AIX * AJC)) / AJB) * DU);
                let AJI = ID * AJF;
                let AJJ = (IE * AJF) + ((((AJA * AJD) - ((AIX * AJD) * AJF)) / AJE) * ID);
                let AJK = DJ + staged[144];
                AIM = AJG;
                AIN = AJK;
                AIO = AJI;
                AIP = AJH;
                AIQ = DK;
                AIR = AJJ;
            } else {
                AIM = DU;
                AIN = DJ;
                AIO = ID;
                AIP = DV;
                AIQ = DK;
                AIR = IE;
            }
            let AJM = AIM * AJL;
            let AJN = AIP * AJL;
            let AJQ = AJO - AJP;
            let AJT = Lanes([AJR, 0.0]) - Lanes([0.0, AJS]);
            let AJV = AJU * AJQ;
            let AJW = AJT * AJU;
            let AJZ = Lanes([AJY, 0.0]);
            let AKA = Lanes([0.0, AJS]);
            let AKB = AJU * (AJX - AJP);
            let AKC = (AJZ - AKA) * AJU;
            let AKE = AKD - AJP;
            let AKG = Lanes([AKF, 0.0]);
            let AKH = Lanes([0.0, AJS]);
            let AKI = AKG - AKH;
            let AKJ = AJU * AKE;
            let AKK = AKI * AJU;
            let AKL = AKB - AKJ;
            let AKM = Lanes([AKC[0], 0.0, AKC[1]]);
            let AKN = Lanes([0.0, AKK[0], AKK[1]]);
            let AKO = AKM - AKN;
            let AKQ = AKP - AKD;
            let AKS = Lanes([0.0, AKR]) - Lanes([AKF, 0.0]);
            let AKT = AJU * AKQ;
            let AKU = AKS * AJU;
            let AKW = AKV - AJX;
            let AKY = Lanes([0.0, AKX]) - Lanes([AJY, 0.0]);
            let AKZ = AJU * AKW;
            let ALA = AKY * AJU;
            let ALC = AKV - ALB;
            let ALE = Lanes([AKX, 0.0]) - Lanes([0.0, ALD]);
            let ALF = AJU * ALC;
            let ALG = ALE * AJU;
            let ALH = AJV - AKB;
            let ALI = Lanes([0.0, AJW[0], AJW[1]]) - Lanes([AKC[0], 0.0, AKC[1]]);
            let ALJ = AJV - AKJ;
            let ALK = Lanes([0.0, AJW[0], AJW[1]]) - Lanes([AKK[0], 0.0, AKK[1]]);
            let ALN = AJU * (ALL - AJX);
            let ALO = (Lanes([0.0, ALM]) - Lanes([AJY, 0.0])) * AJU;
            let ALP = ALL - AKD;
            let ALQ = Lanes([0.0, ALM]) - Lanes([AKF, 0.0]);
            let ALR = AJU * ALP;
            let ALS = ALQ * AJU;
            let AMJ;
            let AMK;
            let AML;
            let AMM;
            let AMN;
            let AMO;
            if ALT != 0.0 {
                let ALX = (Lanes([0.0, ALV]) - Lanes([AJY, 0.0])) * ALW;
                let ALY = AKB + (ALW * (ALU - AJX));
                let ALZ = Lanes([AKC[0], 0.0, AKC[1]]) + Lanes([ALX[0], ALX[1], 0.0]);
                let AMA = Lanes([ALA[0], 0.0, ALA[1]]) + Lanes([AKC[0], AKC[1], 0.0]);
                let AMB = (AKZ + AKB) - ALY;
                let AMC = Lanes([AMA[0], 0.0, AMA[1], AMA[2]]) - Lanes([ALZ[0], ALZ[1], ALZ[2], 0.0]);
                let AMD = Lanes([ALO[0], ALO[1], 0.0]) + Lanes([AKC[0], 0.0, AKC[1]]);
                let AME = (ALN + AKB) - ALY;
                let AMF = Lanes([AMD[0], 0.0, AMD[1], AMD[2]]) - Lanes([ALZ[0], ALZ[1], 0.0, ALZ[2]]);
                AMJ = ALY;
                AMK = AME;
                AML = AMB;
                AMM = ALZ;
                AMN = AMF;
                AMO = AMC;
            } else {
                let AMG = Lanes([AKC[0], 0.0, AKC[1]]);
                let AMH = Lanes([ALO[0], 0.0, ALO[1], 0.0]);
                let AMI = Lanes([ALA[0], 0.0, 0.0, ALA[1]]);
                AMJ = AKB;
                AMK = ALN;
                AML = AKZ;
                AMM = AMG;
                AMN = AMH;
                AMO = AMI;
            }
            let AMP = if AKL < A { 1.0 } else { 0.0 };
            let AMV;
            let AMW;
            let AMX;
            let AMY;
            let AMZ;
            let ANA;
            let ANB;
            let ANC;
            let AND;
            if AMP != 0.0 {
                let AMQ = Lanes([0.0, 0.0, AKK[0], AKK[1]]);
                let AMR = Lanes([AMM[0], AMM[1], 0.0, AMM[2]]);
                AMV = AKJ;
                AMW = AKB;
                AMX = AKJ;
                AMY = AMJ;
                AMZ = AMS;
                ANA = AKN;
                ANB = AKM;
                ANC = AMQ;
                AND = AMR;
            } else {
                let AMT = Lanes([AMM[0], AMM[1], 0.0, AMM[2]]);
                let AMU = Lanes([0.0, 0.0, AKK[0], AKK[1]]);
                AMV = AKB;
                AMW = AKJ;
                AMX = AMJ;
                AMY = AKJ;
                AMZ = U;
                ANA = AKM;
                ANB = AKN;
                ANC = AMT;
                AND = AMU;
            }
            let ANE = AMV - AMW;
            let ANF = ANA - ANB;
            let ANG = AMX - AMY;
            let ANH = ANC - AND;
            let ANJ = ANI * ANG;
            let ANK = ANH * ANI;
            let ANM = if ANJ > ANL { 1.0 } else { 0.0 };
            let ANO;
            let ANP;
            if ANM != 0.0 {
                ANO = ANJ;
                ANP = ANK;
            } else {
                let ANN = if ANJ < -3.7e1f64 { 1.0 } else { 0.0 };
                let AOC;
                let AOD;
                if ANN != 0.0 {
                    let ANW = ANJ.exp();
                    let ANX = ANK * ANW;
                    AOC = ANW;
                    AOD = ANX;
                } else {
                    let ANY = ANJ.exp();
                    let ANZ = U + ANY;
                    let AOA = ANZ.ln();
                    let AOB = (ANK * ANY) * (AK / ANZ);
                    AOC = AOA;
                    AOD = AOB;
                }
                ANO = AOC;
                ANP = AOD;
            }
            let ANR = -(AMY + (CX * (ANG - (((ANQ * ANO) - ANG) - staged[148]))));
            let ANS = (AND + ((ANH - ((ANP * ANQ) - ANH)) * CX)) * W;
            let ANT = ANI * ANE;
            let ANU = ANF * ANI;
            let ANV = if ANT > ANL { 1.0 } else { 0.0 };
            let AOF;
            let AOG;
            if ANV != 0.0 {
                AOF = ANT;
                AOG = ANU;
            } else {
                let AOE = if ANT < -3.7e1f64 { 1.0 } else { 0.0 };
                let AOZ;
                let APA;
                if AOE != 0.0 {
                    let AOT = ANT.exp();
                    let AOU = ANU * AOT;
                    AOZ = AOT;
                    APA = AOU;
                } else {
                    let AOV = ANT.exp();
                    let AOW = U + AOV;
                    let AOX = AOW.ln();
                    let AOY = (ANU * AOV) * (AK / AOW);
                    AOZ = AOX;
                    APA = AOY;
                }
                AOF = AOZ;
                AOG = APA;
            }
            let AOH = (AOG * ANQ) - ANF;
            let AOI = ((ANQ * AOF) - ANE) - staged[149];
            let AOJ = -(AMW + (CX * (ANE - AOI)));
            let AOK = (ANB + ((ANF - AOH) * CX)) * W;
            let AOM = AKO * AOL;
            let AON = (AOL * AKL) / S;
            let AOO = AON.tanh();
            let AOP = (((Lanes([0.0, AOM[0], AOM[1], AOM[2]]) - Lanes([(T * AON), 0.0, 0.0, 0.0])) / S) * (AK - (AOO * AOO))) * CX;
            let AOQ = CX + (CX * AOO);
            let AOR = U - AOQ;
            let AOS = AOP * W;
            let AQX;
            let AQY;
            let AQZ;
            let ARA;
            let ARB;
            let ARC;
            let ARD;
            let ARE;
            let ARF;
            let ARG;
            let ARH;
            let ARI;
            let ARJ;
            let ARK;
            let ARL;
            let ARM;
            let ARN;
            let ARO;
            let ARP;
            let ARQ;
            let ARR;
            let ARS;
            let ART;
            let ARU;
            let ARV;
            let ARW;
            let ARX;
            let ARY;
            if C != 0.0 {
                let APD = (APB * AOR) + (APC * AOQ);
                let APE = (AOS * APB) + (AOP * APC);
                let APF = (DO * AOR) + (AIN * AOQ);
                let APG = (Lanes([(DP * AOR), 0.0, 0.0, 0.0]) + (AOS * DO)) + (Lanes([(AIQ * AOQ), 0.0, 0.0, 0.0]) + (AOP * AIN));
                let APJ = (APH * AOR) + (API * AOQ);
                let APK = (AOS * APH) + (AOP * API);
                let APN = (APL * AOR) + (APM * AOQ);
                let APO = (AOS * APL) + (AOP * APM);
                let APR = (APP * AOR) + (APQ * AOQ);
                let APS = (AOS * APP) + (AOP * APQ);
                let APT = (IV * AOR) + (AIO * AOQ);
                let APU = (Lanes([(IW * AOR), 0.0, 0.0, 0.0]) + (AOS * IV)) + (Lanes([(AIR * AOQ), 0.0, 0.0, 0.0]) + (AOP * AIO));
                let APV = (KL * AOR) + (KI * AOQ);
                let APW = (Lanes([(KM * AOR), 0.0, 0.0, 0.0]) + (AOS * KL)) + (Lanes([(KJ * AOQ), 0.0, 0.0, 0.0]) + (AOP * KI));
                let APX = (GF * AOR) + (AJM * AOQ);
                let APY = (Lanes([(GK * AOR), 0.0, 0.0, 0.0]) + (AOS * GF)) + (Lanes([(AJN * AOQ), 0.0, 0.0, 0.0]) + (AOP * AJM));
                let APZ = (GG * AOR) + (EJ * AOQ);
                let AQA = (Lanes([(GL * AOR), 0.0, 0.0, 0.0]) + (AOS * GG)) + (Lanes([(EK * AOQ), 0.0, 0.0, 0.0]) + (AOP * EJ));
                let AQB = (GH * AOR) + (EY * AOQ);
                let AQC = (Lanes([(GM * AOR), 0.0, 0.0, 0.0]) + (AOS * GH)) + (Lanes([(EZ * AOQ), 0.0, 0.0, 0.0]) + (AOP * EY));
                let AQD = (GI * AOR) + (FE * AOQ);
                let AQE = (Lanes([(GN * AOR), 0.0, 0.0, 0.0]) + (AOS * GI)) + (Lanes([(FF * AOQ), 0.0, 0.0, 0.0]) + (AOP * FE));
                let AQF = (GJ * AOR) + (FK * AOQ);
                let AQG = (Lanes([(GO * AOR), 0.0, 0.0, 0.0]) + (AOS * GJ)) + (Lanes([(FL * AOQ), 0.0, 0.0, 0.0]) + (AOP * FK));
                let AQJ = (AQH * AOR) + (AQI * AOQ);
                let AQK = (AOS * AQH) + (AOP * AQI);
                let AQL = (ML * AOR) + (MG * AOQ);
                let AQM = (Lanes([(MM * AOR), 0.0, 0.0, 0.0]) + (AOS * ML)) + (Lanes([(MH * AOQ), 0.0, 0.0, 0.0]) + (AOP * MG));
                AQX = APD;
                AQY = APF;
                AQZ = AQF;
                ARA = APZ;
                ARB = AQB;
                ARC = AQD;
                ARD = APR;
                ARE = APV;
                ARF = APX;
                ARG = APT;
                ARH = APJ;
                ARI = APN;
                ARJ = AQJ;
                ARK = AQL;
                ARL = APE;
                ARM = APG;
                ARN = AQG;
                ARO = AQA;
                ARP = AQC;
                ARQ = AQE;
                ARR = APS;
                ARS = APW;
                ART = APY;
                ARU = APU;
                ARV = APK;
                ARW = APO;
                ARX = AQK;
                ARY = AQM;
            } else {
                let AQN = Lanes([AIQ, 0.0, 0.0, 0.0]);
                let AQO = Lanes([FL, 0.0, 0.0, 0.0]);
                let AQP = Lanes([EK, 0.0, 0.0, 0.0]);
                let AQQ = Lanes([EZ, 0.0, 0.0, 0.0]);
                let AQR = Lanes([FF, 0.0, 0.0, 0.0]);
                let AQS = Lanes([KJ, 0.0, 0.0, 0.0]);
                let AQT = Lanes([AJN, 0.0, 0.0, 0.0]);
                let AQU = Lanes([AIR, 0.0, 0.0, 0.0]);
                let AQV = Lanes([MH, 0.0, 0.0, 0.0]);
                AQX = APC;
                AQY = AIN;
                AQZ = FK;
                ARA = EJ;
                ARB = EY;
                ARC = FE;
                ARD = APQ;
                ARE = KI;
                ARF = AJM;
                ARG = AIO;
                ARH = API;
                ARI = APM;
                ARJ = AQI;
                ARK = MG;
                ARL = AQW;
                ARM = AQN;
                ARN = AQO;
                ARO = AQP;
                ARP = AQQ;
                ARQ = AQR;
                ARR = AQW;
                ARS = AQS;
                ART = AQT;
                ARU = AQU;
                ARV = AQW;
                ARW = AQW;
                ARX = AQW;
                ARY = AQV;
            }
            let ARZ = CK - AOJ;
            let ASA = Lanes([CL, 0.0, 0.0, 0.0]) - Lanes([0.0, AOK[0], AOK[1], AOK[2]]);
            let ASB = if 0.0f64 != 0.0 && (if ARZ < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ASM;
            let ASN;
            if ASB != 0.0 {
                let ASD = ASC * ARZ;
                let ASE = -1.0000000000000002e-2f64 / ASD;
                let ASF = (((ASA * ASC) * ASE) * W) / ASD;
                ASM = ASE;
                ASN = ASF;
            } else {
                let ASH = ARZ - ASG;
                let ASI = ASA * ASH;
                let ASJ = ((ASH * ASH) + 2.5000000000000005e-3f64).sqrt();
                let ASK = CX * ((ARZ + ASG) + ASJ);
                let ASL = (ASA + ((ASI + ASI) * (AK / (AJ * ASJ)))) * CX;
                ASM = ASK;
                ASN = ASL;
            }
            let ASO = ASM.sqrt();
            let ASP = ASN * (AK / (AJ * ASO));
            let ASR = ASQ * ASO;
            let ASS = ASP * ASQ;
            let ASU = AST / ASR;
            let ASW = AOH * AQX;
            let ASY = AOK * ASX;
            let ATA = ((Lanes([DE, 0.0, 0.0, 0.0]) + ((ARL * AOI) + Lanes([0.0, ASW[0], ASW[1], ASW[2]]))) - Lanes([0.0, ASY[0], ASY[1], ASY[2]])) / ASZ;
            let ATB = U + ((((ASV + DD) + (AQX * AOI)) - (ASX * AOJ)) / ASZ);
            let ATC = if 0.0f64 != 0.0 && (if ATB < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ATL;
            let ATM;
            if ATC != 0.0 {
                let ATD = ASC * ATB;
                let ATE = -2.5000000000000005e-3f64 / ATD;
                let ATF = (((ATA * ASC) * ATE) * W) / ATD;
                ATL = ATE;
                ATM = ATF;
            } else {
                let ATG = ATB - U;
                let ATH = ATA * ATG;
                let ATI = ((ATG * ATG) + 6.250000000000001e-4f64).sqrt();
                let ATJ = CX * ((ATB + U) + ATI);
                let ATK = (ATA + ((ATH + ATH) * (AK / (AJ * ATI)))) * CX;
                ATL = ATJ;
                ATM = ATK;
            }
            let ATN = ATL * S;
            let ATO = (ATM * S) + Lanes([(T * ATL), 0.0, 0.0, 0.0]);
            let ATP = U / ATN;
            let ATQ = ((ATO * ATP) * W) / ATN;
            let ATS = AOK * ATR;
            let ATT = -(AQY + (ATR * AOJ));
            let ATU = ATT * AOI;
            let ATV = AOH * ATT;
            let ATW = (((ARM + Lanes([0.0, ATS[0], ATS[1], ATS[2]])) * W) * AOI) + Lanes([0.0, ATV[0], ATV[1], ATV[2]]);
            let ATX = ATW * ATU;
            let ATY = ((ATU * ATU) + 6.25e-6f64).sqrt();
            let ATZ = (ATW - ((ATX + ATX) * (AK / (AJ * ATY)))) * CX;
            let AUA = (CX * (ATU - ATY)) + 1.25e-3f64;
            let AUC = staged[167] + (AUB * AOJ);
            let AUE = (Z.powf(AUD)) - U;
            let AUF = AUC * AUE;
            let AUG = (AOK * AUB) * AUE;
            let AUH = Lanes([0.0, AUG[0], AUG[1], AUG[2]]) + Lanes([((AA * (AUD * (Z.powf(staged[428])))) * AUC), 0.0, 0.0, 0.0]);
            let AUN;
            let AUO;
            if AUI != 0.0 {
                let AUK = AUJ * AOI;
                let AUL = AOH * AUJ;
                let AUM = if AUK < -8e1f64 { 1.0 } else { 0.0 };
                let AVY;
                let AVZ;
                if AUM != 0.0 {
                    AVY = AVU;
                    AVZ = AVV;
                } else {
                    let AVW = rspice_limited_exp(AUK);
                    let AVX = AUL * (rspice_limited_exp_derivative(AUK));
                    AVY = AVW;
                    AVZ = AVX;
                }
                let AWC = AWB + (AWA * (U + AVY));
                let AWD = -ATN;
                let AWE = AWB / AWC;
                let AWF = if AWE >= AX { AWE } else { AX };
                let AWG = AWF.ln();
                let AWH = AWD * AWG;
                let AWI = ((((((AVZ * AWA) * AWE) * W) / AWC) * (if AWE >= AX { 1.0 } else { 0.0 })) * (AK / AWF)) * AWD;
                let AWJ = ((ATO * W) * AWG) + Lanes([0.0, AWI[0], AWI[1], AWI[2]]);
                AUN = AWH;
                AUO = AWJ;
            } else {
                AUN = A;
                AUO = AQW;
            }
            let AUQ = (AUP * AOI).tanh();
            let AUS = ((AOH * AUP) * (AK - (AUQ * AUQ))) * AUR;
            let AUT = AJW * ATP;
            let AUU = ATQ * AJV;
            let AUV = AMW * ATP;
            let AUW = ANB * ATP;
            let AUX = Lanes([0.0, AUW[0], AUW[1], AUW[2]]) + (ATQ * AMW);
            let AUZ = ATQ * AUY;
            let AVA = ASO - CM;
            let AVB = ASP - Lanes([CN, 0.0, 0.0, 0.0]);
            let AVE = AOK * AVD;
            let AVF = ((ATZ + (AUO - Lanes([0.0, AUS[0], AUS[1], AUS[2]]))) + ((AVB * AVC) - Lanes([0.0, AVE[0], AVE[1], AVE[2]]))) - AUH;
            let AVG = ((((AUA + (AUN - (AUR * AUQ))) + ((AVC * AVA) - (AVD * AOJ))) - AUF) + staged[176]) + staged[177];
            let AVH = (AVF * ATP) + (ATQ * AVG);
            let AVI = ((AJV * ATP) - (AUY * ATP)) - (AVG * ATP);
            let AVJ = ((Lanes([0.0, 0.0, 0.0, AUT[0], AUT[1]]) + Lanes([AUU[0], AUU[1], AUU[2], 0.0, AUU[3]])) - Lanes([AUZ[0], AUZ[1], AUZ[2], 0.0, AUZ[3]])) - Lanes([AVH[0], AVH[1], AVH[2], 0.0, AVH[3]]);
            let AVL = (AVK * V).sqrt();
            let AVM = AVL / ASZ;
            let AVN = ((X * AVK) * (AK / (AJ * AVL))) / ASZ;
            let AVO = AMW * V;
            let AVP = ANB * V;
            let AVQ = Lanes([0.0, AVP[0], AVP[1], AVP[2]]) + Lanes([(X * AMW), 0.0, 0.0, 0.0]);
            let AVR = (AO * BI) + AVO;
            let AVS = Lanes([(BJ * AO), 0.0, 0.0, 0.0]) + AVQ;
            let AVT = if AVR < -1e1f64 { 1.0 } else { 0.0 };
            let AWQ;
            let AWR;
            if AVT != 0.0 {
                let AWK = -1e-6f64 / AVR;
                let AWL = ((AVS * AWK) * W) / AVR;
                AWQ = AWK;
                AWR = AWL;
            } else {
                let AWM = AVS * AVR;
                let AWN = ((AVR * AVR) + 4e-6f64).sqrt();
                let AWO = CX * (AVR + AWN);
                let AWP = (AVS + ((AWM + AWM) * (AK / (AJ * AWN)))) * CX;
                AWQ = AWO;
                AWR = AWP;
            }
            let AWS = AWQ.sqrt();
            let AWT = AO * AWS;
            let AWU = (AWR * (AK / (AJ * AWS))) * AO;
            let AWV = AVM / AWT;
            let AWW = (Lanes([AVN, 0.0, 0.0, 0.0]) - (AWU * AWV)) / AWT;
            let AWX = U + AWV;
            let AWY = (AO * AWX) / AVM;
            let AWZ = AWX / AVM;
            let AXA = AWZ + AWT;
            let AXB = AWY * AXA;
            let AXC = if AXB >= AX { AXB } else { AX };
            let AXD = ((AVR + -6.931471805599453e-1f64) + 1e0f64) + (AXC.ln());
            let AXE = AVS + (((((((AWW * AO) - Lanes([(AVN * AWY), 0.0, 0.0, 0.0])) / AVM) * AXA) + ((((AWW - Lanes([(AVN * AWZ), 0.0, 0.0, 0.0])) / AVM) + AWU) * AWY)) * (if AXB >= AX { 1.0 } else { 0.0 })) * (AK / AXC));
            let AXF = if AXD < -1e1f64 { 1.0 } else { 0.0 };
            let AXM;
            let AXN;
            if AXF != 0.0 {
                let AXG = -1e-6f64 / AXD;
                let AXH = ((AXE * AXG) * W) / AXD;
                AXM = AXG;
                AXN = AXH;
            } else {
                let AXI = AXE * AXD;
                let AXJ = ((AXD * AXD) + 4e-6f64).sqrt();
                let AXK = CX * (AXD + AXJ);
                let AXL = (AXE + ((AXI + AXI) * (AK / (AJ * AXJ)))) * CX;
                AXM = AXK;
                AXN = AXL;
            }
            let AXO = AXM - AVO;
            let AXP = S * AVM;
            let AXQ = AXM.sqrt();
            let AXR = AJU * (((AUY + (AXO * S)) + (AXP * AXQ)) + AVG);
            let AXS = (((((AXN - AVQ) * S) + Lanes([(T * AXO), 0.0, 0.0, 0.0])) + (Lanes([(((T * AVM) + (AVN * S)) * AXQ), 0.0, 0.0, 0.0]) + ((AXN * (AK / (AJ * AXQ))) * AXP))) + AVF) * AJU;
            let AXU = (AXT * ATP).sqrt();
            let AXV = AXU / ASZ;
            let AXW = ((ATQ * AXT) * (AK / (AJ * AXU))) / ASZ;
            let AXX = U / AXV;
            let AXY = ((AXW * AXX) * W) / AXV;
            let AXZ = BI / ATL;
            let AYA = (Lanes([BJ, 0.0, 0.0, 0.0]) - (ATM * AXZ)) / ATL;
            let AYD = (AXW / AYB) * AYC;
            let AYE = (CX * AVI) - (AYC * (U + (AXV / AYB)));
            let AYF = (AVJ * CX) - Lanes([AYD[0], AYD[1], AYD[2], 0.0, AYD[3]]);
            let AYG = AYF * AYE;
            let AYI = ((AYE * AYE) + (AYH * AVI)).sqrt();
            let AYJ = AYE + AYI;
            let AYK = AYF + (((AYG + AYG) + (AVJ * AYH)) * (AK / (AJ * AYI)));
            let AYL = if AVI < A { 1.0 } else { 0.0 };
            let AZF;
            let AZG;
            if AYL != 0.0 {
                let AYM = (AVI - AYJ) / AXV;
                let AYN = AXW * AYM;
                let AYO = (((AVJ - AYK) - Lanes([AYN[0], AYN[1], AYN[2], 0.0, AYN[3]])) / AXV) * AYM;
                let AYP = (U - AYJ) + (AYM * AYM);
                let AYQ = if AYP >= AX { AYP } else { AX };
                let AYR = -(AYQ.ln());
                let AYS = ((((AYK * W) + (AYO + AYO)) * (if AYP >= AX { 1.0 } else { 0.0 })) * (AK / AYQ)) * W;
                AZF = AYR;
                AZG = AYS;
            } else {
                let AYT = -AYJ;
                let AYU = rspice_limited_exp(AYT);
                let AYV = (AYK * W) * (rspice_limited_exp_derivative(AYT));
                let AYW = CX * AXV;
                let AYX = AXW * CX;
                let AYY = AYX * AYW;
                let AYZ = AYY + AYY;
                let AZA = (((AVI - U) + AYU) + (AYW * AYW)).sqrt();
                let AZB = AZA - AYW;
                let AZC = ((((AVJ + AYV) + Lanes([AYZ[0], AYZ[1], AYZ[2], 0.0, AYZ[3]])) * (AK / (AJ * AZA))) - Lanes([AYX[0], AYX[1], AYX[2], 0.0, AYX[3]])) * AZB;
                let AZD = ((AZB * AZB) + U) - AYU;
                let AZE = (AZC + AZC) - AYV;
                AZF = AZD;
                AZG = AZE;
            }
            let AZH = AZF + U;
            let AZI = AZF - U;
            let AZJ = AZI * AZI;
            let AZK = AZG * AZI;
            let AZL = AZK + AZK;
            let AZM = (AZJ + 1e0f64).sqrt();
            let AZN = (CX * (AZH + AZM)).sqrt();
            let AZO = ((AZG + (AZL * (AK / (AJ * AZM)))) * CX) * (AK / (AJ * AZN));
            let AZP = AO * AZN;
            let AZQ = AZO * AO;
            let AZR = AXV / AZP;
            let AZS = Lanes([AXW[0], AXW[1], AXW[2], 0.0, AXW[3]]);
            let AZT = (U + AZR) / AXV;
            let AZU = AXW * AZT;
            let AZV = (((AZS - (AZQ * AZR)) / AZP) - Lanes([AZU[0], AZU[1], AZU[2], 0.0, AZU[3]])) / AXV;
            let AZW = AYA * AO;
            let AZX = AZF - (AO * AXZ);
            let AZY = AZG - Lanes([AZW[0], AZW[1], AZW[2], 0.0, AZW[3]]);
            let AZZ = AZX - AUV;
            let BAA = AZY - Lanes([AUX[0], AUX[1], AUX[2], 0.0, AUX[3]]);
            let BAB = ADB * AZT;
            let BAC = BAB * AZN;
            let BAD = if BAC >= AX { BAC } else { AX };
            let BAE = AZZ - (BAD.ln());
            let BAF = BAA - (((((AZV * ADB) * AZN) + (AZO * BAB)) * (if BAC >= AX { 1.0 } else { 0.0 })) * (AK / BAD));
            let BAI = BAE + BAH;
            let BAK = ((BAE * BAI) + BAJ).sqrt();
            let BAL = CX * ((BAE - BAG) - BAK);
            let BAM = (BAF - (((BAF * BAI) + (BAF * BAE)) * (AK / (AJ * BAK)))) * CX;
            let BAN = if BAL <= -6.8e1f64 { 1.0 } else { 0.0 };
            let BCP;
            let BCQ;
            if BAN != 0.0 {
                let BAO = if BAL < -1.1e2f64 { 1.0 } else { 0.0 };
                let BCV;
                let BCW;
                if BAO != 0.0 {
                    BCV = BCS;
                    BCW = BCT;
                } else {
                    let BCU = if BAL > -9e1f64 { 1.0 } else { 0.0 };
                    let BDU;
                    let BDV;
                    if BCU != 0.0 {
                        let BDF = rspice_limited_exp(BAL);
                        let BDG = BAM * (rspice_limited_exp_derivative(BAL));
                        BDU = BDF;
                        BDV = BDG;
                    } else {
                        let BDJ = (BAL - BDH) / BDI;
                        let BDK = BAM / BDI;
                        let BDL = BDJ * BDJ;
                        let BDM = BDK * BDJ;
                        let BDN = BDM + BDM;
                        let BDP = BDO - BDL;
                        let BDQ = 9.375e-1f64 - (BDL * BDP);
                        let BDR = BDH + (BDI * ((7.8125e-2f64 + (CX * BDJ)) + (BDL * BDQ)));
                        let BDS = rspice_limited_exp(BDR);
                        let BDT = (((BDK * CX) + ((BDN * BDQ) + ((((BDN * BDP) + ((BDN * W) * BDL)) * W) * BDL))) * BDI) * (rspice_limited_exp_derivative(BDR));
                        BDU = BDS;
                        BDV = BDT;
                    }
                    BCV = BDU;
                    BCW = BDV;
                }
                let BCX = AO * AZT;
                let BCY = BCV * AO;
                let BCZ = (BCY * AZT) + AZP;
                let BDA = BCX * BCZ;
                let BDB = if BDA >= AX { BDA } else { AX };
                let BDC = ((U + AZZ) - BAL) - (BDB.ln());
                let BDD = BCV * BDC;
                let BDE = (BCW * BDC) + (((BAA - BAM) - (((((AZV * AO) * BCZ) + (((((BCW * AO) * AZT) + (AZV * BCY)) + AZQ) * BCX)) * (if BDA >= AX { 1.0 } else { 0.0 })) * (AK / BDB))) * BCV);
                BCP = BDD;
                BCQ = BDE;
            } else {
                let BAP = rspice_limited_exp(BAL);
                let BAQ = BAM * (rspice_limited_exp_derivative(BAL));
                let BAR = U / AZN;
                let BAS = AO * BAP;
                let BAT = BAQ * AO;
                let BAU = BAS * AZT;
                let BAV = (BAT * AZT) + (AZV * BAS);
                let BAW = BAU + AZP;
                let BAX = BAU * BAW;
                let BAY = if BAX >= AX { BAX } else { AX };
                let BAZ = 1e0f64 / BAP;
                let BBA = AZT + BAR;
                let BBB = AZV + (((AZO * BAR) * W) / AZN);
                let BBC = (AZT * BAP) + AZN;
                let BBD = BBA / BBC;
                let BBE = (AO + BAZ) + BBD;
                let BBF = ((BAS + (BAY.ln())) - AZZ) / BBE;
                let BBG = BAP - BBF;
                let BBH = BAQ - ((((BAT + ((((BAV * BAW) + ((BAV + AZQ) * BAU)) * (if BAX >= AX { 1.0 } else { 0.0 })) * (AK / BAY))) - BAA) - (((((BAQ * BAZ) * W) / BAP) + ((BBB - ((((AZV * BAP) + (BAQ * AZT)) + AZO) * BBD)) / BBC)) * BBF)) / BBE);
                let BBI = AO * BBG;
                let BBJ = BBH * AO;
                let BBK = BBI * AZT;
                let BBL = (BBJ * AZT) + (AZV * BBI);
                let BBM = BBK + AZP;
                let BBN = BBK * BBM;
                let BBO = if BBN >= AX { BBN } else { AX };
                let BBP = (BBI + (BBO.ln())) - AZZ;
                let BBQ = (BBJ + ((((BBL * BBM) + ((BBL + AZQ) * BBK)) * (if BBN >= AX { 1.0 } else { 0.0 })) * (AK / BBO))) - BAA;
                let BBR = 1e0f64 / BBG;
                let BBS = (AZT * BBG) + AZN;
                let BBT = ((AZV * BBG) + (BBH * AZT)) + AZO;
                let BBU = BBA / BBS;
                let BBV = (BBB - (BBT * BBU)) / BBS;
                let BBW = (AO + BBR) + BBU;
                let BBX = (((BBH * BBR) * W) / BBG) + BBV;
                let BBY = BBV * BBU;
                let BBZ = U / BBG;
                let BCA = (((BBH * BBZ) * W) / BBG) * BBZ;
                let BCC = AZN * AZN;
                let BCD = AZO * AZN;
                let BCE = BCC * AZN;
                let BCF = BCE * BBS;
                let BCG = 1e0f64 / BCF;
                let BCH = ((BCB * (BBZ * BBZ)) - BCG) - (BBU * BBU);
                let BCI = BBP / BBW;
                let BCJ = AO * BBW;
                let BCK = BCJ * BBW;
                let BCL = (BBP * BCH) / BCK;
                let BCM = U + BCL;
                let BCN = BBG - (BCI * BCM);
                let BCO = BBH - ((((BBQ - (BBX * BCI)) / BBW) * BCM) + (((((BBQ * BCH) + (((((BCA + BCA) * BCB) - ((((((((BCD + BCD) * AZN) + (AZO * BCC)) * BBS) + (BBT * BCE)) * BCG) * W) / BCF)) - (BBY + BBY)) * BBP)) - ((((BBX * AO) * BBW) + (BBX * BCJ)) * BCL)) / BCK) * BCI));
                BCP = BCN;
                BCQ = BCO;
            }
            let BCR = if 0.0f64 != 0.0 && (if AZF < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BEC;
            let BED;
            if BCR != 0.0 {
                let BDW = ASC * AZF;
                let BDX = -4e0f64 / BDW;
                let BDY = (((AZG * ASC) * BDX) * W) / BDW;
                BEC = BDX;
                BED = BDY;
            } else {
                let BDZ = (AZJ + 1e0f64).sqrt();
                let BEA = CX * (AZH + BDZ);
                let BEB = (AZG + (AZL * (AK / (AJ * BDZ)))) * CX;
                BEC = BEA;
                BED = BEB;
            }
            let BEE = BEC.sqrt();
            let BEF = BED * (AK / (AJ * BEE));
            let BEG = AO * BCP;
            let BEH = BCQ * AO;
            let BEI = AZF - BEG;
            let BEJ = AZG - BEH;
            let BEK = if 0.0f64 != 0.0 && (if BEI < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BET;
            let BEU;
            if BEK != 0.0 {
                let BEL = ASC * BEI;
                let BEM = -4e0f64 / BEL;
                let BEN = (((BEJ * ASC) * BEM) * W) / BEL;
                BET = BEM;
                BEU = BEN;
            } else {
                let BEO = BEI - U;
                let BEP = BEJ * BEO;
                let BEQ = ((BEO * BEO) + 1e0f64).sqrt();
                let BER = CX * ((BEI + U) + BEQ);
                let BES = (BEJ + ((BEP + BEP) * (AK / (AJ * BEQ)))) * CX;
                BET = BER;
                BEU = BES;
            }
            let BEV = BET.sqrt();
            let BEW = BEE + BEV;
            let BEX = AXV / BEW;
            let BEY = (AZS - ((BEF + (BEU * (AK / (AJ * BEV)))) * BEX)) / BEW;
            let BEZ = U + BEX;
            let BFA = AVI - AZF;
            let BFB = AVJ - AZG;
            let BFC = BEZ - U;
            let BFD = BFA - (BEG * BFC);
            let BFE = ATN * BFD;
            let BFF = ATO * BFD;
            let BFG = Lanes([BFF[0], BFF[1], BFF[2], 0.0, BFF[3]]) + ((BFB - ((BEH * BFC) + (BEY * BEG))) * ATN);
            let BFH = if 1.0f64 != 0.0 && (if BFE < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BFP;
            let BFQ;
            if BFH != 0.0 {
                let BFI = ASC * BFE;
                let BFJ = -1.0000000000000002e-2f64 / BFI;
                let BFK = (((BFG * ASC) * BFJ) * W) / BFI;
                BFP = BFJ;
                BFQ = BFK;
            } else {
                let BFL = BFG * BFE;
                let BFM = ((BFE * BFE) + 2.5000000000000005e-3f64).sqrt();
                let BFN = CX * (BFE + BFM);
                let BFO = (BFG + ((BFL + BFL) * (AK / (AJ * BFM)))) * CX;
                BFP = BFN;
                BFQ = BFO;
            }
            let BFR = AO * BEZ;
            let BFS = BFR * ATN;
            let BFT = ATO * BFR;
            let BFU = BFS * BCP;
            let BFV = ((((BEY * AO) * ATN) + Lanes([BFT[0], BFT[1], BFT[2], 0.0, BFT[3]])) * BCP) + (BCQ * BFS);
            let BFY = BFX * (BFP + (BFW * BFU));
            let BFZ = BFU / BFP;
            let BGA = CX * (U + BFZ);
            let BGB = BGA.powf(AQZ);
            let BGC = AQZ - AK;
            let BGD = ARN * (BGB * (BGA.ln()));
            let BGE = AOK * ARB;
            let BGF = ARA + (ARB * AOJ);
            let BGG = ARO + ((ARP * AOJ) + Lanes([0.0, BGE[0], BGE[1], BGE[2]]));
            let BGH = BFY.powf(FZ);
            let BGI = FZ - AK;
            let BGJ = BGG * BGH;
            let BGK = ARC / BGB;
            let BGL = Lanes([ARQ[0], ARQ[1], ARQ[2], 0.0, ARQ[3]]);
            let BGM = (Lanes([BGJ[0], BGJ[1], BGJ[2], 0.0, BGJ[3]]) + (((((BFQ + (BFV * BFW)) * BFX) * (FZ * (BFY.powf(BGI)))) + Lanes([(GA * (BGH * (BFY.ln()))), 0.0, 0.0, 0.0, 0.0])) * BGF)) + ((BGL - ((((((BFV - (BFQ * BFZ)) / BFP) * CX) * (AQZ * (BGA.powf(BGC)))) + Lanes([BGD[0], BGD[1], BGD[2], 0.0, BGD[3]])) * BGK)) / BGB);
            let BGN = U + ((BGF * BGH) + BGK);
            let BGO = if 0.0f64 != 0.0 && (if BGN < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BGX;
            let BGY;
            if BGO != 0.0 {
                let BGP = ASC * BGN;
                let BGQ = -2.25e-6f64 / BGP;
                let BGR = (((BGM * ASC) * BGQ) * W) / BGP;
                BGX = BGQ;
                BGY = BGR;
            } else {
                let BGS = BGN - U;
                let BGT = BGM * BGS;
                let BGU = ((BGS * BGS) + 5.625e-7f64).sqrt();
                let BGV = CX * ((BGN + U) + BGU);
                let BGW = (BGM + ((BGT + BGT) * (AK / (AJ * BGU)))) * CX;
                BGX = BGV;
                BGY = BGW;
            }
            let BHK;
            let BHL;
            if D != 0.0 {
                BHK = A;
                BHL = BCT;
            } else {
                let BHA = U + (BGZ * BFU);
                let BHC = AVB * BHB;
                let BHD = U / BHA;
                let BHE = BHD + (BHB * AVA);
                let BHF = ((((BFV * BGZ) * BHD) * W) / BHA) + Lanes([BHC[0], BHC[1], BHC[2], 0.0, BHC[3]]);
                let BHG = BHF * BHE;
                let BHH = ((BHE * BHE) + TY).sqrt();
                let BHI = BHE + BHH;
                let BHJ = BHF + ((BHG + BHG) * (AK / (AJ * BHH)));
                let BJA;
                let BJB;
                if H != 0.0 {
                    let BIS = ((BIP + (BIO * BHI)) * BIQ) * BIR;
                    let BIT = BIS * GQ;
                    let BIU = ((((BHJ * BIO) * BIQ) * BIR) * GQ) + Lanes([(GR * BIS), 0.0, 0.0, 0.0, 0.0]);
                    BJA = BIT;
                    BJB = BIU;
                } else {
                    let BIX = (BIV + (((BIP + (BIO * BHI)) * BIQ) * BIR)) + BIW;
                    let BIY = BIX * GQ;
                    let BIZ = ((((BHJ * BIO) * BIQ) * BIR) * GQ) + Lanes([(GR * BIX), 0.0, 0.0, 0.0, 0.0]);
                    BJA = BIY;
                    BJB = BIZ;
                }
                BHK = BJA;
                BHL = BJB;
            }
            let BHM = U / ARD;
            let BHN = ((ARR * BHM) * W) / ARD;
            let BHO = BGX.powf(BHM);
            let BHP = BHM - AK;
            let BHQ = BHN * (BHO * (BGX.ln()));
            let BHR = (BGY * (BHM * (BGX.powf(BHP)))) + Lanes([BHQ[0], BHQ[1], BHQ[2], 0.0, BHQ[3]]);
            let BHT = BHS * AOJ;
            let BHU = AOK * BHS;
            let BHV = BHU * BHT;
            let BHX = (BHW + (BHT * BHT)).sqrt();
            let BHY = U - BHT;
            let BHZ = BHU * W;
            let BIA = BHZ * BHY;
            let BIB = (BIA + BIA) + ((BHV + BHV) * (AK / (AJ * BHX)));
            let BIC = ((BHY * BHY) + BHX).sqrt();
            let BID = AK / (AJ * BIC);
            let BIE = CX * (BHY + BIC);
            let BIF = (BHZ + (BIB * BID)) * CX;
            let BIH = BIG * BCP;
            let BII = BIF * BIH;
            let BIJ = BIF * BCP;
            let BIK = BIG + (BCP * BIE);
            let BIL = (BIH * BIE) / BIK;
            let BIM = ((((BCQ * BIG) * BIE) + Lanes([0.0, BII[0], BII[1], 0.0, BII[2]])) - (((BCQ * BIE) + Lanes([0.0, BIJ[0], BIJ[1], 0.0, BIJ[2]])) * BIL)) / BIK;
            let BIN = if ARE < A { 1.0 } else { 0.0 };
            let BJX;
            let BJY;
            if BIN != 0.0 {
                let BJC = ARF / BHO;
                let BJD = ATO * BJC;
                let BJE = ARG * AWB;
                let BJF = (BJC * ATN) / BJE;
                let BJG = (ARU * AWB) * BJF;
                let BJH = AO * BJF;
                let BJI = ARS * BIL;
                let BJJ = U - (ARE * BIL);
                let BJK = U / BJJ;
                let BJL = BJH * BJK;
                let BJM = ((((((((Lanes([ART[0], ART[1], ART[2], 0.0, ART[3]]) - (BHR * BJC)) / BHO) * ATN) + Lanes([BJD[0], BJD[1], BJD[2], 0.0, BJD[3]])) - Lanes([BJG[0], BJG[1], BJG[2], 0.0, BJG[3]])) / BJE) * AO) * BJK) + ((((((Lanes([BJI[0], BJI[1], BJI[2], 0.0, BJI[3]]) + (BIM * ARE)) * W) * BJK) * W) / BJJ) * BJH);
                BJX = BJL;
                BJY = BJM;
            } else {
                let BJN = ARF / BHO;
                let BJO = ATO * BJN;
                let BJP = ARG * AWB;
                let BJQ = (BJN * ATN) / BJP;
                let BJR = (ARU * AWB) * BJQ;
                let BJS = AO * BJQ;
                let BJT = ARS * BIL;
                let BJU = U + (ARE * BIL);
                let BJV = BJS * BJU;
                let BJW = ((((((((Lanes([ART[0], ART[1], ART[2], 0.0, ART[3]]) - (BHR * BJN)) / BHO) * ATN) + Lanes([BJO[0], BJO[1], BJO[2], 0.0, BJO[3]])) - Lanes([BJR[0], BJR[1], BJR[2], 0.0, BJR[3]])) / BJP) * AO) * BJU) + ((Lanes([BJT[0], BJT[1], BJT[2], 0.0, BJT[3]]) + (BIM * ARE)) * BJS);
                BJX = BJV;
                BJY = BJW;
            }
            let BJZ = if BHK > A { 1.0 } else { 0.0 };
            let BMA;
            let BMB;
            if BJZ != 0.0 {
                let BKB = BKA * AO;
                let BKC = (BKB * BEZ) * ASZ;
                let BKD = BKC * ATN;
                let BKE = ATO * BKC;
                let BKF = BKD * ARG;
                let BKG = ARU * BKD;
                let BKH = BKF * BJX;
                let BKI = AO * ATN;
                let BKJ = (BKH * BHK) / BKI;
                let BKK = (ATO * AO) * BKJ;
                let BKL = (((((((((((BEY * BKB) * ASZ) * ATN) + Lanes([BKE[0], BKE[1], BKE[2], 0.0, BKE[3]])) * ARG) + Lanes([BKG[0], BKG[1], BKG[2], 0.0, BKG[3]])) * BJX) + (BJY * BKF)) * BHK) + (BHL * BKH)) - Lanes([BKK[0], BKK[1], BKK[2], 0.0, BKK[3]])) / BKI;
                let BKM = CX * BJX;
                let BKN = BJY * CX;
                let BKO = BCQ * BCP;
                let BKP = (BCP * BCP) + BCP;
                let BKQ = (BKO + BKO) + BCQ;
                let BKR = U + BCP;
                let BKS = U + (BKM * BKR);
                let BKT = (BKM * BKP) / BKS;
                let BKU = (((BKN * BKP) + (BKQ * BKM)) - (((BKN * BKR) + (BCQ * BKM)) * BKT)) / BKS;
                let BKV = AO * BJX;
                let BKW = BJY * AO;
                let BKX = BCP - BKT;
                let BKY = BKV * BKX;
                let BKZ = (BKW * BKX) + ((BCQ - BKU) * BKV);
                let BLA = BKY * BKY;
                let BLB = BKZ * BKY;
                let BLC = BLB + BLB;
                let BLD = (U + BLA).sqrt();
                let BLE = BLC * (AK / (AJ * BLD));
                let BLF = if BKY != A { 1.0 } else { 0.0 };
                let BMZ;
                let BNA;
                let BNB;
                let BNC;
                if BLF != 0.0 {
                    let BMR = BKY.asinh();
                    let BMS = BKZ * (AK / ((AK + BLA).sqrt()));
                    let BMT = U / BKY;
                    let BMU = BLD + (BMT * BMR);
                    let BMV = BLE + (((((BKZ * BMT) * W) / BKY) * BMR) + (BMS * BMT));
                    BMZ = BMU;
                    BNA = BMR;
                    BNB = BMV;
                    BNC = BMS;
                } else {
                    let BMW = U / BLD;
                    let BMX = BLD + BMW;
                    let BMY = BLE + (((BLE * BMW) * W) / BLD);
                    BMZ = BMX;
                    BNA = A;
                    BNB = BMY;
                    BNC = BCT;
                }
                let BND = BKJ * BKT;
                let BNE = (BCP + BKT) + U;
                let BNF = BKU * BKT;
                let BNG = BKP - ((BKT * BKT) + BKT);
                let BNH = ((BKT * BMZ) + (BND * BNE)) - (BJX * BNG);
                let BNI = (((BKU * BMZ) + (BNB * BKT)) + ((((BKL * BKT) + (BKU * BKJ)) * BNE) + ((BCQ + BKU) * BND))) - ((BJY * BNG) + ((BKQ - ((BNF + BNF) + BKU)) * BJX));
                let BNT;
                let BNU;
                if BLF != 0.0 {
                    let BNK = BNJ * BJX;
                    let BNL = (BKY * BLD) - BNA;
                    let BNM = (BNK * BNL) / BLA;
                    let BNN = ((((BJY * BNJ) * BNL) + ((((BKZ * BLD) + (BLE * BKY)) - BNC) * BNK)) - (BLC * BNM)) / BLA;
                    BNT = BNM;
                    BNU = BNN;
                } else {
                    let BNP = BNO * BJX;
                    let BNQ = BKY / BLD;
                    let BNR = BNP * BNQ;
                    let BNS = ((BJY * BNO) * BNQ) + (((BKZ - (BLE * BNQ)) / BLD) * BNP);
                    BNT = BNR;
                    BNU = BNS;
                }
                let BNV = AO * BKT;
                let BNW = BKU * AO;
                let BNX = (BCP + BNV) + U;
                let BNY = BNV + U;
                let BNZ = (((BKT * BNT) + BMZ) + (BKJ * BNX)) + (BJX * BNY);
                let BOA = BNH / BNZ;
                let BOB = BKT - BOA;
                let BOC = BKU - ((BNI - ((((((BKU * BNT) + (BNU * BKT)) + BNB) + ((BKL * BNX) + ((BCQ + BNW) * BKJ))) + ((BJY * BNY) + (BNW * BJX))) * BOA)) / BNZ);
                let BOD = BCP - BOB;
                let BOE = BKV * BOD;
                let BOF = (BKW * BOD) + ((BCQ - BOC) * BKV);
                let BOG = BOE * BOE;
                let BOH = BOF * BOE;
                let BOI = BOH + BOH;
                let BOJ = (U + BOG).sqrt();
                let BOK = BOI * (AK / (AJ * BOJ));
                let BOL = if BOE != A { 1.0 } else { 0.0 };
                let BOU;
                let BOV;
                let BOW;
                let BOX;
                if BOL != 0.0 {
                    let BOM = BOE.asinh();
                    let BON = BOF * (AK / ((AK + BOG).sqrt()));
                    let BOO = U / BOE;
                    let BOP = BOJ + (BOO * BOM);
                    let BOQ = BOK + (((((BOF * BOO) * W) / BOE) * BOM) + (BON * BOO));
                    BOU = BOP;
                    BOV = BOM;
                    BOW = BOQ;
                    BOX = BON;
                } else {
                    let BOR = U / BOJ;
                    let BOS = BOJ + BOR;
                    let BOT = BOK + (((BOK * BOR) * W) / BOJ);
                    BOU = BOS;
                    BOV = BNA;
                    BOW = BOT;
                    BOX = BNC;
                }
                let BOY = BKJ * BOB;
                let BOZ = (BCP + BOB) + U;
                let BPA = BOC * BOB;
                let BPB = BKP - ((BOB * BOB) + BOB);
                let BPC = ((BOB * BOU) + (BOY * BOZ)) - (BJX * BPB);
                let BPD = (((BOC * BOU) + (BOW * BOB)) + ((((BKL * BOB) + (BOC * BKJ)) * BOZ) + ((BCQ + BOC) * BOY))) - ((BJY * BPB) + ((BKQ - ((BPA + BPA) + BOC)) * BJX));
                let BPO;
                let BPP;
                if BOL != 0.0 {
                    let BPF = BPE * BJX;
                    let BPG = (BOE * BOJ) - BOV;
                    let BPH = (BPF * BPG) / BOG;
                    let BPI = ((((BJY * BPE) * BPG) + ((((BOF * BOJ) + (BOK * BOE)) - BOX) * BPF)) - (BOI * BPH)) / BOG;
                    BPO = BPH;
                    BPP = BPI;
                } else {
                    let BPK = BPJ * BJX;
                    let BPL = BOE / BOJ;
                    let BPM = BPK * BPL;
                    let BPN = ((BJY * BPJ) * BPL) + (((BOF - (BOK * BPL)) / BOJ) * BPK);
                    BPO = BPM;
                    BPP = BPN;
                }
                let BPQ = AO * BOB;
                let BPR = BOC * AO;
                let BPS = (BCP + BPQ) + U;
                let BPT = BPQ + U;
                let BPU = (((BOB * BPO) + BOU) + (BKJ * BPS)) + (BJX * BPT);
                let BPV = BPC / BPU;
                let BPW = BOB - BPV;
                let BPX = BOC - ((BPD - ((((((BOC * BPO) + (BPP * BOB)) + BOW) + ((BKL * BPS) + ((BCQ + BPR) * BKJ))) + ((BJY * BPT) + (BPR * BJX))) * BPV)) / BPU);
                BMA = BPW;
                BMB = BPX;
            } else {
                let BLG = CX * BJX;
                let BLH = BJY * CX;
                let BLI = BCQ * BCP;
                let BLJ = (BCP * BCP) + BCP;
                let BLK = (BLI + BLI) + BCQ;
                let BLL = U + BCP;
                let BLM = U + (BLG * BLL);
                let BLN = (BLG * BLJ) / BLM;
                let BLO = (((BLH * BLJ) + (BLK * BLG)) - (((BLH * BLL) + (BCQ * BLG)) * BLN)) / BLM;
                let BLP = AO * BJX;
                let BLQ = BJY * AO;
                let BLR = BCP - BLN;
                let BLS = BLP * BLR;
                let BLT = (BLQ * BLR) + ((BCQ - BLO) * BLP);
                let BLU = BLS * BLS;
                let BLV = BLT * BLS;
                let BLW = BLV + BLV;
                let BLX = (U + BLU).sqrt();
                let BLY = BLW * (AK / (AJ * BLX));
                let BLZ = if BLS != A { 1.0 } else { 0.0 };
                let BQG;
                let BQH;
                let BQI;
                let BQJ;
                if BLZ != 0.0 {
                    let BPY = BLS.asinh();
                    let BPZ = BLT * (AK / ((AK + BLU).sqrt()));
                    let BQA = U / BLS;
                    let BQB = BLX + (BQA * BPY);
                    let BQC = BLY + (((((BLT * BQA) * W) / BLS) * BPY) + (BPZ * BQA));
                    BQG = BQB;
                    BQH = BPY;
                    BQI = BQC;
                    BQJ = BPZ;
                } else {
                    let BQD = U / BLX;
                    let BQE = BLX + BQD;
                    let BQF = BLY + (((BLY * BQD) * W) / BLX);
                    BQG = BQE;
                    BQH = A;
                    BQI = BQF;
                    BQJ = BCT;
                }
                let BQK = BLO * BLN;
                let BQL = BLJ - ((BLN * BLN) + BLN);
                let BQM = (BLN * BQG) - (BJX * BQL);
                let BQN = ((BLO * BQG) + (BQI * BLN)) - ((BJY * BQL) + ((BLK - ((BQK + BQK) + BLO)) * BJX));
                let BQY;
                let BQZ;
                if BLZ != 0.0 {
                    let BQP = BQO * BJX;
                    let BQQ = (BLS * BLX) - BQH;
                    let BQR = (BQP * BQQ) / BLU;
                    let BQS = ((((BJY * BQO) * BQQ) + ((((BLT * BLX) + (BLY * BLS)) - BQJ) * BQP)) - (BLW * BQR)) / BLU;
                    BQY = BQR;
                    BQZ = BQS;
                } else {
                    let BQU = BQT * BJX;
                    let BQV = BLS / BLX;
                    let BQW = BQU * BQV;
                    let BQX = ((BJY * BQT) * BQV) + (((BLT - (BLY * BQV)) / BLX) * BQU);
                    BQY = BQW;
                    BQZ = BQX;
                }
                let BRA = (AO * BLN) + U;
                let BRB = ((BLN * BQY) + BQG) + (BJX * BRA);
                let BRC = BQM / BRB;
                let BRD = BLN - BRC;
                let BRE = BLO - ((BQN - (((((BLO * BQY) + (BQZ * BLN)) + BQI) + ((BJY * BRA) + ((BLO * AO) * BJX))) * BRC)) / BRB);
                let BRF = BCP - BRD;
                let BRG = BLP * BRF;
                let BRH = (BLQ * BRF) + ((BCQ - BRE) * BLP);
                let BRI = BRG * BRG;
                let BRJ = BRH * BRG;
                let BRK = BRJ + BRJ;
                let BRL = (U + BRI).sqrt();
                let BRM = BRK * (AK / (AJ * BRL));
                let BRN = if BRG != A { 1.0 } else { 0.0 };
                let BRW;
                let BRX;
                let BRY;
                let BRZ;
                if BRN != 0.0 {
                    let BRO = BRG.asinh();
                    let BRP = BRH * (AK / ((AK + BRI).sqrt()));
                    let BRQ = U / BRG;
                    let BRR = BRL + (BRQ * BRO);
                    let BRS = BRM + (((((BRH * BRQ) * W) / BRG) * BRO) + (BRP * BRQ));
                    BRW = BRR;
                    BRX = BRO;
                    BRY = BRS;
                    BRZ = BRP;
                } else {
                    let BRT = U / BRL;
                    let BRU = BRL + BRT;
                    let BRV = BRM + (((BRM * BRT) * W) / BRL);
                    BRW = BRU;
                    BRX = BQH;
                    BRY = BRV;
                    BRZ = BQJ;
                }
                let BSA = BRE * BRD;
                let BSB = BLJ - ((BRD * BRD) + BRD);
                let BSC = (BRD * BRW) - (BJX * BSB);
                let BSD = ((BRE * BRW) + (BRY * BRD)) - ((BJY * BSB) + ((BLK - ((BSA + BSA) + BRE)) * BJX));
                let BSO;
                let BSP;
                if BRN != 0.0 {
                    let BSF = BSE * BJX;
                    let BSG = (BRG * BRL) - BRX;
                    let BSH = (BSF * BSG) / BRI;
                    let BSI = ((((BJY * BSE) * BSG) + ((((BRH * BRL) + (BRM * BRG)) - BRZ) * BSF)) - (BRK * BSH)) / BRI;
                    BSO = BSH;
                    BSP = BSI;
                } else {
                    let BSK = BSJ * BJX;
                    let BSL = BRG / BRL;
                    let BSM = BSK * BSL;
                    let BSN = ((BJY * BSJ) * BSL) + (((BRH - (BRM * BSL)) / BRL) * BSK);
                    BSO = BSM;
                    BSP = BSN;
                }
                let BSQ = (AO * BRD) + U;
                let BSR = ((BRD * BSO) + BRW) + (BJX * BSQ);
                let BSS = BSC / BSR;
                let BST = BRD - BSS;
                let BSU = BRE - ((BSD - (((((BRE * BSO) + (BSP * BRD)) + BRY) + ((BJY * BSQ) + ((BRE * AO) * BJX))) * BSS)) / BSR);
                BMA = BST;
                BMB = BSU;
            }
            let BMC = AO * BMA;
            let BMD = BMB * AO;
            let BME = BMC * BEZ;
            let BMF = BME * AXX;
            let BMG = AXY * BME;
            let BMH = (((BMD * BEZ) + (BEY * BMC)) * AXX) + Lanes([BMG[0], BMG[1], BMG[2], 0.0, BMG[3]]);
            let BMI = AXV / BFC;
            let BMJ = BMF + BMI;
            let BMK = BMF * BMJ;
            let BML = if BMK >= AX { BMK } else { AX };
            let BMM = AZX - (BMC + (BML.ln()));
            let BMN = BMM * ATN;
            let BMO = ATO * BMM;
            let BMP = ((AZY - (BMD + ((((BMH * BMJ) + ((BMH + ((AZS - (BEY * BMI)) / BFC)) * BMF)) * (if BMK >= AX { 1.0 } else { 0.0 })) * (AK / BML)))) * ATN) + Lanes([BMO[0], BMO[1], BMO[2], 0.0, BMO[3]]);
            let BTQ;
            let BTR;
            if BMQ != 0.0 {
                BTQ = U;
                BTR = BCT;
            } else {
                let BSW = (BSV * ASR).sqrt();
                let BSX = AWB + BSW;
                let BSY = AWB / BSX;
                let BSZ = ((((ASS * BSV) * (AK / (AJ * BSW))) * BSY) * W) / BSX;
                let BTB = BSZ * BTA;
                let BTD = BTC * BSY;
                let BTF = BCP.powf(BTE);
                let BTG = BTD * BTF;
                let BTH = (BSZ * BTC) * BTF;
                let BTI = ATO * BTG;
                let BTK = U + (BTJ * AOJ);
                let BTL = ((BTA * BSY) - (BTG * ATN)) / BTK;
                let BTM = (AOK * BTJ) * BTL;
                let BTN = ((Lanes([BTB[0], BTB[1], BTB[2], 0.0, BTB[3]]) - (((Lanes([BTH[0], BTH[1], BTH[2], 0.0, BTH[3]]) + ((BCQ * (BTE * (BCP.powf(staged[429])))) * BTD)) * ATN) + Lanes([BTI[0], BTI[1], BTI[2], 0.0, BTI[3]]))) - Lanes([0.0, BTM[0], BTM[1], 0.0, BTM[2]])) / BTK;
                let BTO = U + BTL;
                let BTP = if 0.0f64 != 0.0 && (if BTO < -1.25e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BUE;
                let BUF;
                if BTP != 0.0 {
                    let BTW = ASC * BTO;
                    let BTX = -2.5e-7f64 / BTW;
                    let BTY = (((BTN * ASC) * BTX) * W) / BTW;
                    BUE = BTX;
                    BUF = BTY;
                } else {
                    let BTZ = BTO - BHW;
                    let BUA = BTN * BTZ;
                    let BUB = ((BTZ * BTZ) + 6.25e-8f64).sqrt();
                    let BUC = CX * ((BTO + BHW) + BUB);
                    let BUD = (BTN + ((BUA + BUA) * (AK / (AJ * BUB)))) * CX;
                    BUE = BUC;
                    BUF = BUD;
                }
                BTQ = BUE;
                BTR = BUF;
            }
            let BTS = BMN - AMW;
            let BTT = Lanes([0.0, ANB[0], ANB[1], 0.0, ANB[2]]);
            let BTU = BMP - BTT;
            let BTV = if 1.0f64 != 0.0 && (if BTS < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BUN;
            let BUO;
            if BTV != 0.0 {
                let BUG = ASC * BTS;
                let BUH = -1e-6f64 / BUG;
                let BUI = (((BTU * ASC) * BUH) * W) / BUG;
                BUN = BUH;
                BUO = BUI;
            } else {
                let BUJ = BTU * BTS;
                let BUK = ((BTS * BTS) + 2.5e-7f64).sqrt();
                let BUL = CX * (BTS + BUK);
                let BUM = (BTU + ((BUJ + BUJ) * (AK / (AJ * BUK)))) * CX;
                BUN = BUL;
                BUO = BUM;
            }
            let BUP = BUN / BTQ;
            let BUQ = (BUO - (BTR * BUP)) / BTQ;
            let BUR = ANE / BUP;
            let BUS = Lanes([0.0, ANF[0], ANF[1], 0.0, ANF[2]]);
            let BUT = BUR + BB;
            let BUU = U / JT;
            let BUV = ((JU * BUU) * W) / JT;
            let BUW = BUT.powf(BUU);
            let BUX = BUU - AK;
            let BUY = U + BUW;
            let BUZ = -JT;
            let BVA = JU * W;
            let BVB = BUY.powf(BUZ);
            let BVC = BUZ - AK;
            let BVD = ANE * BVB;
            let BVE = ANF * BVB;
            let BVF = Lanes([0.0, BVE[0], BVE[1], 0.0, BVE[2]]) + (((((((BUS - (BUQ * BUR)) / BUP) * (BUU * (BUT.powf(BUX)))) + Lanes([(BUV * (BUW * (BUT.ln()))), 0.0, 0.0, 0.0, 0.0])) * (BUZ * (BUY.powf(BVC)))) + Lanes([(BVA * (BVB * (BUY.ln()))), 0.0, 0.0, 0.0, 0.0])) * ANE);
            let BVG = BVD + AMW;
            let BVH = ATQ * BVG;
            let BVI = (AZJ + 1e0f64).sqrt();
            let BVJ = (CX * (AZH + BVI)).sqrt();
            let BVK = ((AZG + (AZL * (AK / (AJ * BVI)))) * CX) * (AK / (AJ * BVJ));
            let BVL = AO * BVJ;
            let BVM = BVK * AO;
            let BVN = AXV / BVL;
            let BVO = (U + BVN) / AXV;
            let BVP = AXW * BVO;
            let BVQ = (((AZS - (BVM * BVN)) / BVL) - Lanes([BVP[0], BVP[1], BVP[2], 0.0, BVP[3]])) / AXV;
            let BVR = AZX - (BVG * ATP);
            let BVS = AZY - (((BVF + BTT) * ATP) + Lanes([BVH[0], BVH[1], BVH[2], 0.0, BVH[3]]));
            let BVT = ADB * BVO;
            let BVU = BVT * BVJ;
            let BVV = if BVU >= AX { BVU } else { AX };
            let BVW = BVR - (BVV.ln());
            let BVX = BVS - (((((BVQ * ADB) * BVJ) + (BVK * BVT)) * (if BVU >= AX { 1.0 } else { 0.0 })) * (AK / BVV));
            let BVY = BVW + BAH;
            let BVZ = ((BVW * BVY) + BAJ).sqrt();
            let BWA = CX * ((BVW - BAG) - BVZ);
            let BWB = (BVX - (((BVX * BVY) + (BVX * BVW)) * (AK / (AJ * BVZ)))) * CX;
            let BWC = if BWA <= -6.8e1f64 { 1.0 } else { 0.0 };
            let BYE;
            let BYF;
            if BWC != 0.0 {
                let BWD = if BWA < -1.1e2f64 { 1.0 } else { 0.0 };
                let BYL;
                let BYM;
                if BWD != 0.0 {
                    BYL = BYJ;
                    BYM = BCT;
                } else {
                    let BYK = if BWA > -9e1f64 { 1.0 } else { 0.0 };
                    let BZI;
                    let BZJ;
                    if BYK != 0.0 {
                        let BYV = rspice_limited_exp(BWA);
                        let BYW = BWB * (rspice_limited_exp_derivative(BWA));
                        BZI = BYV;
                        BZJ = BYW;
                    } else {
                        let BYY = (BWA - BYX) / BDI;
                        let BYZ = BWB / BDI;
                        let BZA = BYY * BYY;
                        let BZB = BYZ * BYY;
                        let BZC = BZB + BZB;
                        let BZD = BDO - BZA;
                        let BZE = 9.375e-1f64 - (BZA * BZD);
                        let BZF = BYX + (BDI * ((7.8125e-2f64 + (CX * BYY)) + (BZA * BZE)));
                        let BZG = rspice_limited_exp(BZF);
                        let BZH = (((BYZ * CX) + ((BZC * BZE) + ((((BZC * BZD) + ((BZC * W) * BZA)) * W) * BZA))) * BDI) * (rspice_limited_exp_derivative(BZF));
                        BZI = BZG;
                        BZJ = BZH;
                    }
                    BYL = BZI;
                    BYM = BZJ;
                }
                let BYN = AO * BVO;
                let BYO = BYL * AO;
                let BYP = (BYO * BVO) + BVL;
                let BYQ = BYN * BYP;
                let BYR = if BYQ >= AX { BYQ } else { AX };
                let BYS = ((U + BVR) - BWA) - (BYR.ln());
                let BYT = BYL * BYS;
                let BYU = (BYM * BYS) + (((BVS - BWB) - (((((BVQ * AO) * BYP) + (((((BYM * AO) * BVO) + (BVQ * BYO)) + BVM) * BYN)) * (if BYQ >= AX { 1.0 } else { 0.0 })) * (AK / BYR))) * BYL);
                BYE = BYT;
                BYF = BYU;
            } else {
                let BWE = rspice_limited_exp(BWA);
                let BWF = BWB * (rspice_limited_exp_derivative(BWA));
                let BWG = U / BVJ;
                let BWH = AO * BWE;
                let BWI = BWF * AO;
                let BWJ = BWH * BVO;
                let BWK = (BWI * BVO) + (BVQ * BWH);
                let BWL = BWJ + BVL;
                let BWM = BWJ * BWL;
                let BWN = if BWM >= AX { BWM } else { AX };
                let BWO = 1e0f64 / BWE;
                let BWP = BVO + BWG;
                let BWQ = BVQ + (((BVK * BWG) * W) / BVJ);
                let BWR = (BVO * BWE) + BVJ;
                let BWS = BWP / BWR;
                let BWT = (AO + BWO) + BWS;
                let BWU = ((BWH + (BWN.ln())) - BVR) / BWT;
                let BWV = BWE - BWU;
                let BWW = BWF - ((((BWI + ((((BWK * BWL) + ((BWK + BVM) * BWJ)) * (if BWM >= AX { 1.0 } else { 0.0 })) * (AK / BWN))) - BVS) - (((((BWF * BWO) * W) / BWE) + ((BWQ - ((((BVQ * BWE) + (BWF * BVO)) + BVK) * BWS)) / BWR)) * BWU)) / BWT);
                let BWX = AO * BWV;
                let BWY = BWW * AO;
                let BWZ = BWX * BVO;
                let BXA = (BWY * BVO) + (BVQ * BWX);
                let BXB = BWZ + BVL;
                let BXC = BWZ * BXB;
                let BXD = if BXC >= AX { BXC } else { AX };
                let BXE = (BWX + (BXD.ln())) - BVR;
                let BXF = (BWY + ((((BXA * BXB) + ((BXA + BVM) * BWZ)) * (if BXC >= AX { 1.0 } else { 0.0 })) * (AK / BXD))) - BVS;
                let BXG = 1e0f64 / BWV;
                let BXH = (BVO * BWV) + BVJ;
                let BXI = ((BVQ * BWV) + (BWW * BVO)) + BVK;
                let BXJ = BWP / BXH;
                let BXK = (BWQ - (BXI * BXJ)) / BXH;
                let BXL = (AO + BXG) + BXJ;
                let BXM = (((BWW * BXG) * W) / BWV) + BXK;
                let BXN = BXK * BXJ;
                let BXO = U / BWV;
                let BXP = (((BWW * BXO) * W) / BWV) * BXO;
                let BXR = BVJ * BVJ;
                let BXS = BVK * BVJ;
                let BXT = BXR * BVJ;
                let BXU = BXT * BXH;
                let BXV = 1e0f64 / BXU;
                let BXW = ((BXQ * (BXO * BXO)) - BXV) - (BXJ * BXJ);
                let BXX = BXE / BXL;
                let BXY = AO * BXL;
                let BXZ = BXY * BXL;
                let BYA = (BXE * BXW) / BXZ;
                let BYB = U + BYA;
                let BYC = BWV - (BXX * BYB);
                let BYD = BWW - ((((BXF - (BXM * BXX)) / BXL) * BYB) + (((((BXF * BXW) + (((((BXP + BXP) * BXQ) - ((((((((BXS + BXS) * BVJ) + (BVK * BXR)) * BXH) + (BXI * BXT)) * BXV) * W) / BXU)) - (BXN + BXN)) * BXE)) - ((((BXM * AO) * BXL) + (BXM * BXY)) * BYA)) / BXZ) * BXX));
                BYE = BYC;
                BYF = BYD;
            }
            let BYG = (AZG - BCQ) - BYF;
            let BYH = ((AZF - BCP) - BYE) - U;
            let BYI = if 0.0f64 != 0.0 && (if BYH < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BZS;
            let BZT;
            if BYI != 0.0 {
                let BZK = ASC * BYH;
                let BZL = -4e0f64 / BZK;
                let BZM = (((BYG * ASC) * BZL) * W) / BZK;
                BZS = BZL;
                BZT = BZM;
            } else {
                let BZN = BYH - U;
                let BZO = BYG * BZN;
                let BZP = ((BZN * BZN) + 1e0f64).sqrt();
                let BZQ = CX * ((BYH + U) + BZP);
                let BZR = (BYG + ((BZO + BZO) * (AK / (AJ * BZP)))) * CX;
                BZS = BZQ;
                BZT = BZR;
            }
            let BZU = BZS.sqrt();
            let BZV = BVJ + BZU;
            let BZW = AXV / BZV;
            let BZX = (AZS - ((BVK + (BZT * (AK / (AJ * BZU)))) * BZW)) / BZV;
            let BZY = U + BZW;
            let BZZ = BCP - BYE;
            let CAA = BCQ - BYF;
            let CAB = BZZ * BZZ;
            let CAC = CAA * BZZ;
            let CAD = CAC + CAC;
            let CAE = (U + BCP) + BYE;
            let CAF = BCQ + BYF;
            let CAG = U / CAE;
            let CAH = ((CAF * CAG) * W) / CAE;
            let CAI = CAB * CAG;
            let CAJ = (CAD * CAG) + (CAH * CAB);
            let CAK = BZY - U;
            let CAL = BCP + BYE;
            let CAN = CAL + (CAM * CAI);
            let CAO = BFA - (CAK * CAN);
            let CAP = CAM * BZY;
            let CAQ = BZX * CAM;
            let CAR = CAI * CAG;
            let CAS = (CAJ * CAG) + (CAH * CAI);
            let CAV = CX * ((U + (CAT * BCP)) + (CAU * BYE));
            let CAW = (BEG + BYE) + (CAV * CAR);
            let CAX = CAP * CAW;
            let CAY = (CAQ * CAW) + (((BEH + BYF) + (((((BCQ * CAT) + (BYF * CAU)) * CX) * CAR) + (CAS * CAV))) * CAP);
            let CAZ = CX * ((U + (CAU * BCP)) + (CAT * BYE));
            let CBA = (BCP + (AO * BYE)) + (CAZ * CAR);
            let CBB = CAP * CBA;
            let CBC = (CAQ * CBA) + (((BCQ + (BYF * AO)) + (((((BCQ * CAU) + (BYF * CAT)) * CX) * CAR) + (CAS * CAZ))) * CAP);
            let CBD = ATN * CAO;
            let CBE = ATO * CAO;
            let CBF = Lanes([CBE[0], CBE[1], CBE[2], 0.0, CBE[3]]) + ((BFB - ((BZX * CAN) + ((CAF + (CAJ * CAM)) * CAK))) * ATN);
            let CBG = if 1.0f64 != 0.0 && (if CBD < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CBO;
            let CBP;
            if CBG != 0.0 {
                let CBH = ASC * CBD;
                let CBI = -1.0000000000000002e-2f64 / CBH;
                let CBJ = (((CBF * ASC) * CBI) * W) / CBH;
                CBO = CBI;
                CBP = CBJ;
            } else {
                let CBK = CBF * CBD;
                let CBL = ((CBD * CBD) + 2.5000000000000005e-3f64).sqrt();
                let CBM = CX * (CBD + CBL);
                let CBN = (CBF + ((CBK + CBK) * (AK / (AJ * CBL)))) * CX;
                CBO = CBM;
                CBP = CBN;
            }
            let CBQ = CAX + CBB;
            let CBR = ATN * CBQ;
            let CBS = ATO * CBQ;
            let CBT = Lanes([CBS[0], CBS[1], CBS[2], 0.0, CBS[3]]) + ((CAY + CBC) * ATN);
            let CBU = BFX * (CBO + (BFW * CBR));
            let CBV = CBR / CBO;
            let CBW = CX * (U + CBV);
            let CBX = CBW.powf(AQZ);
            let CBY = ARN * (CBX * (CBW.ln()));
            let CBZ = CBU.powf(FZ);
            let CCA = BGG * CBZ;
            let CCB = ARC / CBX;
            let CCC = (Lanes([CCA[0], CCA[1], CCA[2], 0.0, CCA[3]]) + (((((CBP + (CBT * BFW)) * BFX) * (FZ * (CBU.powf(BGI)))) + Lanes([(GA * (CBZ * (CBU.ln()))), 0.0, 0.0, 0.0, 0.0])) * BGF)) + ((BGL - ((((((CBT - (CBP * CBV)) / CBO) * CX) * (AQZ * (CBW.powf(BGC)))) + Lanes([CBY[0], CBY[1], CBY[2], 0.0, CBY[3]])) * CCB)) / CBX);
            let CCD = U + ((BGF * CBZ) + CCB);
            let CCE = if 0.0f64 != 0.0 && (if CCD < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CCN;
            let CCO;
            if CCE != 0.0 {
                let CCF = ASC * CCD;
                let CCG = -2.25e-6f64 / CCF;
                let CCH = (((CCC * ASC) * CCG) * W) / CCF;
                CCN = CCG;
                CCO = CCH;
            } else {
                let CCI = CCD - U;
                let CCJ = CCC * CCI;
                let CCK = ((CCI * CCI) + 5.625e-7f64).sqrt();
                let CCL = CX * ((CCD + U) + CCK);
                let CCM = (CCC + ((CCJ + CCJ) * (AK / (AJ * CCK)))) * CX;
                CCN = CCL;
                CCO = CCM;
            }
            let CCP = AO * ARG;
            let CCQ = ARU * AO;
            let CCR = ARF / CCN;
            let CCS = Lanes([ART[0], ART[1], ART[2], 0.0, ART[3]]);
            let CCT = CCP / CCR;
            let CCU = Lanes([CCQ[0], CCQ[1], CCQ[2], 0.0, CCQ[3]]);
            let CCV = CCT * AWB;
            let CCW = ((CCU - (((CCS - (CCO * CCR)) / CCN) * CCT)) / CCR) * AWB;
            let CDG;
            let CDH;
            if CCX != 0.0 {
                let CCZ = (CCY * CBR) / CCV;
                let CDA = ((CBT * CCY) - (CCW * CCZ)) / CCV;
                let CDB = U + CCZ;
                CDG = CDB;
                CDH = CDA;
            } else {
                let CDC = (CCY * CBR) / CCV;
                let CDD = U - CDC;
                let CDE = U / CDD;
                let CDF = ((((((CBT * CCY) - (CCW * CDC)) / CCV) * W) * CDE) * W) / CDD;
                CDG = CDE;
                CDH = CDF;
            }
            let CDI = ANE - BVD;
            let CDJ = BUS - BVF;
            let CDK = ATO * AO;
            let CDL = CBR + (AO * ATN);
            let CDM = CBT + Lanes([CDK[0], CDK[1], CDK[2], 0.0, CDK[3]]);
            let CDN = if ARH > A { 1.0 } else { 0.0 };
            let CDV;
            let CDW;
            if CDN != 0.0 {
                let CDO = BUP + CDL;
                let CDP = CDL / CDO;
                let CDQ = (CDM - ((BUQ + CDM) * CDP)) / CDO;
                let CDS = AOK * CDR;
                let CDT = U + (CDR * AOJ);
                let CDU = if CDT < -1e1f64 { 1.0 } else { 0.0 };
                let CEE;
                let CEF;
                if CDU != 0.0 {
                    let CDY = -1e-6f64 / CDT;
                    let CDZ = ((CDS * CDY) * W) / CDT;
                    CEE = CDY;
                    CEF = CDZ;
                } else {
                    let CEA = CDS * CDT;
                    let CEB = ((CDT * CDT) + 4e-6f64).sqrt();
                    let CEC = CX * (CDT + CEB);
                    let CED = (CDS + ((CEA + CEA) * (AK / (AJ * CEB)))) * CX;
                    CEE = CEC;
                    CEF = CED;
                }
                let CEG = U / CEE;
                let CEH = CDL / ARH;
                let CEI = ARV * CEH;
                let CEJ = CEH * CDP;
                let CEK = CEJ * CDG;
                let CEL = CEK * CEG;
                let CEM = (((CEF * CEG) * W) / CEE) * CEK;
                let CEN = CDI / CEL;
                let CEO = (CDJ - (((((((((CDM - Lanes([CEI[0], CEI[1], CEI[2], 0.0, CEI[3]])) / ARH) * CDP) + (CDQ * CEH)) * CDG) + (CDH * CEJ)) * CEG) + Lanes([0.0, CEM[0], CEM[1], 0.0, CEM[2]])) * CEN)) / CEL;
                let CEP = U + CEN;
                CDV = CEP;
                CDW = CEO;
            } else {
                CDV = U;
                CDW = BCT;
            }
            let CEU;
            let CEV;
            if CDX != 0.0 {
                CEU = U;
                CEV = BCT;
            } else {
                let CEQ = staged[197] / CDL;
                let CER = U + CEQ;
                let CES = U / CER;
                let CET = (((((CDM * CEQ) * W) / CDL) * CES) * W) / CER;
                CEU = CES;
                CEV = CET;
            }
            let CEW = BUP + CCV;
            let CEX = BUQ + CCW;
            let CEY = if ARI > A { 1.0 } else { 0.0 };
            let CFC;
            let CFD;
            if CEY != 0.0 {
                let CFA = if CEZ < A { 1.0 } else { 0.0 };
                let CFV;
                let CFW;
                if CFA != 0.0 {
                    let CFL = (CEZ * CBR) / CCV;
                    let CFM = U - CFL;
                    let CFN = ARI / CFM;
                    let CFO = CFN / CEU;
                    let CFP = (((Lanes([ARW[0], ARW[1], ARW[2], 0.0, ARW[3]]) - (((((CBT * CEZ) - (CCW * CFL)) / CCV) * W) * CFN)) / CFM) - (CEV * CFO)) / CEU;
                    CFV = CFO;
                    CFW = CFP;
                } else {
                    let CFQ = (CEZ * CBR) / CCV;
                    let CFR = U + CFQ;
                    let CFS = ARW * CFR;
                    let CFT = (ARI * CFR) / CEU;
                    let CFU = ((Lanes([CFS[0], CFS[1], CFS[2], 0.0, CFS[3]]) + ((((CBT * CEZ) - (CCW * CFQ)) / CCV) * ARI)) - (CEV * CFT)) / CEU;
                    CFV = CFT;
                    CFW = CFU;
                }
                let CFX = CDI / CFV;
                let CFY = CFX / CEW;
                let CFZ = U + CFY;
                let CGA = if CFZ >= AX { CFZ } else { AX };
                let CGB = CGA.ln();
                let CGC = (CFW * CGB) + (((((((CDJ - (CFW * CFX)) / CFV) - (CEX * CFY)) / CEW) * (if CFZ >= AX { 1.0 } else { 0.0 })) * (AK / CGA)) * CFV);
                let CGD = U + (CFV * CGB);
                CFC = CGD;
                CFD = CGC;
            } else {
                let CFB = if CEZ < A { 1.0 } else { 0.0 };
                let CGO;
                let CGP;
                if CFB != 0.0 {
                    let CGE = (CEZ * CBR) / CCV;
                    let CGF = U - CGE;
                    let CGG = ARI / CGF;
                    let CGH = CGG / CEU;
                    let CGI = (((Lanes([ARW[0], ARW[1], ARW[2], 0.0, ARW[3]]) - (((((CBT * CEZ) - (CCW * CGE)) / CCV) * W) * CGG)) / CGF) - (CEV * CGH)) / CEU;
                    CGO = CGH;
                    CGP = CGI;
                } else {
                    let CGJ = (CEZ * CBR) / CCV;
                    let CGK = U + CGJ;
                    let CGL = ARW * CGK;
                    let CGM = (ARI * CGK) / CEU;
                    let CGN = ((Lanes([CGL[0], CGL[1], CGL[2], 0.0, CGL[3]]) + ((((CBT * CEZ) - (CCW * CGJ)) / CCV) * ARI)) - (CEV * CGM)) / CEU;
                    CGO = CGM;
                    CGP = CGN;
                }
                let CGQ = U + CGO;
                CFC = CGQ;
                CFD = CGP;
            }
            let CFE = CDV * CFC;
            let CFF = (CDW * CFC) + (CFD * CDV);
            let CFH = CFG * ANE;
            let CFI = rspice_limited_exp(CFH);
            let CFJ = (ANF * CFG) * (rspice_limited_exp_derivative(CFH));
            let CGY;
            let CGZ;
            if CFK != 0.0 {
                let CGT = (U + (CGR * CFI)) / CGS;
                let CGU = CGT * CEU;
                let CGV = ((CFJ * CGR) / CGS) * CEU;
                let CGW = Lanes([0.0, CGV[0], CGV[1], 0.0, CGV[2]]) + (CEV * CGT);
                CGY = CGU;
                CGZ = CGW;
            } else {
                CGY = CGX;
                CGZ = BCT;
            }
            let CHA = CDI / CGY;
            let CHB = U + CHA;
            let CHC = CFE * CHB;
            let CHD = (CFF * CHB) + (((CDJ - (CGZ * CHA)) / CGY) * CFE);
            let CHG;
            let CHH;
            if CHE != 0.0 {
                let CHF = if CDI > staged[201] { 1.0 } else { 0.0 };
                let CIC;
                let CID;
                if CHF != 0.0 {
                    let CHX = staged[202] / CDI;
                    let CHZ = (AWB * (rspice_limited_exp(CHX))) / CHY;
                    let CIA = (((((CDJ * CHX) * W) / CDI) * (rspice_limited_exp_derivative(CHX))) * AWB) / CHY;
                    CIC = CHZ;
                    CID = CIA;
                } else {
                    let CIB = (CGX * AWB) / CHY;
                    CIC = CIB;
                    CID = BCT;
                }
                CHG = CIC;
                CHH = CID;
            } else {
                CHG = CGX;
                CHH = BCT;
            }
            let CHI = CDI / CHG;
            let CHJ = (CDJ - (CHH * CHI)) / CHG;
            let CHK = U + CHI;
            let CHL = CHC * CHK;
            let CHM = (CHD * CHK) + (CHJ * CHC);
            let CHN = CCN.powf(BHM);
            let CHO = BHN * (CHN * (CCN.ln()));
            let CHP = (CCO * (BHM * (CCN.powf(BHP)))) + Lanes([CHO[0], CHO[1], CHO[2], 0.0, CHO[3]]);
            let CHQ = (BHZ + (BIB * BID)) * CX;
            let CHR = BIG * CBR;
            let CHS = CHQ * CHR;
            let CHT = CHQ * CBR;
            let CHU = BIG + (CBR * BIE);
            let CHV = (CHR * BIE) / CHU;
            let CHW = ((((CBT * BIG) * BIE) + Lanes([0.0, CHS[0], CHS[1], 0.0, CHS[2]])) - (((CBT * BIE) + Lanes([0.0, CHT[0], CHT[1], 0.0, CHT[2]])) * CHV)) / CHU;
            let CIZ;
            let CJA;
            if BIN != 0.0 {
                let CIE = ARF / CHN;
                let CIF = ATO * CIE;
                let CIG = ARG * AWB;
                let CIH = (CIE * ATN) / CIG;
                let CII = (ARU * AWB) * CIH;
                let CIJ = AO * CIH;
                let CIK = ARS * CHV;
                let CIL = U - (ARE * CHV);
                let CIM = U / CIL;
                let CIN = CIJ * CIM;
                let CIO = ((((((((CCS - (CHP * CIE)) / CHN) * ATN) + Lanes([CIF[0], CIF[1], CIF[2], 0.0, CIF[3]])) - Lanes([CII[0], CII[1], CII[2], 0.0, CII[3]])) / CIG) * AO) * CIM) + ((((((Lanes([CIK[0], CIK[1], CIK[2], 0.0, CIK[3]]) + (CHW * ARE)) * W) * CIM) * W) / CIL) * CIJ);
                CIZ = CIN;
                CJA = CIO;
            } else {
                let CIP = ARF / CHN;
                let CIQ = ATO * CIP;
                let CIR = ARG * AWB;
                let CIS = (CIP * ATN) / CIR;
                let CIT = (ARU * AWB) * CIS;
                let CIU = AO * CIS;
                let CIV = ARS * CHV;
                let CIW = U + (ARE * CHV);
                let CIX = CIU * CIW;
                let CIY = ((((((((CCS - (CHP * CIP)) / CHN) * ATN) + Lanes([CIQ[0], CIQ[1], CIQ[2], 0.0, CIQ[3]])) - Lanes([CIT[0], CIT[1], CIT[2], 0.0, CIT[3]])) / CIR) * AO) * CIW) + ((Lanes([CIV[0], CIV[1], CIV[2], 0.0, CIV[3]]) + (CHW * ARE)) * CIU);
                CIZ = CIX;
                CJA = CIY;
            }
            let CJB = AO * CIZ;
            let CJC = CJB * BZZ;
            let CJD = ((CJA * AO) * BZZ) + (CAA * CJB);
            let CJE = CJC * CJC;
            let CJF = CJD * CJC;
            let CJG = (U + CJE).sqrt();
            let CJH = (CJF + CJF) * (AK / (AJ * CJG));
            let CJI = if CJC != A { 1.0 } else { 0.0 };
            let CJQ;
            let CJR;
            if CJI != 0.0 {
                let CJJ = U / CJC;
                let CJK = CJC.asinh();
                let CJL = CX * (CJG + (CJJ * CJK));
                let CJM = (CJH + (((((CJD * CJJ) * W) / CJC) * CJK) + ((CJD * (AK / ((AK + CJE).sqrt()))) * CJJ))) * CX;
                CJQ = CJL;
                CJR = CJM;
            } else {
                let CJN = U / CJG;
                let CJO = CX * (CJG + CJN);
                let CJP = (CJH + (((CJH * CJN) * W) / CJG)) * CX;
                CJQ = CJO;
                CJR = CJP;
            }
            let CLX;
            let CLY;
            let CLZ;
            let CMA;
            let CMB;
            let CMC;
            if D != 0.0 {
                let CJU = AJU * (CJS - AJP);
                let CJV = (Lanes([CJT, 0.0]) - Lanes([0.0, AJS])) * AJU;
                let CJW = Lanes([0.0, AJW[0], AJW[1]]) - Lanes([CJV[0], 0.0, CJV[1]]);
                let CJX = (AJV - CJU) - CF;
                let CJY = Lanes([0.0, CJW[0], CJW[1], CJW[2]]) - Lanes([CG, 0.0, 0.0, 0.0]);
                let CJZ = CJY * CJX;
                let CKA = ((CJX * CJX) + TY).sqrt();
                let CKB = U + (BGZ * (CX * (CJX + CKA)));
                let CKC = U / CKB;
                let CKD = CJV * BHB;
                let CKE = CKC + (BHB * CJU);
                let CKF = ((((((CJY + ((CJZ + CJZ) * (AK / (AJ * CKA)))) * CX) * BGZ) * CKC) * W) / CKB) + Lanes([0.0, CKD[0], 0.0, CKD[1]]);
                let CKG = CKF * CKE;
                let CKH = ((CKE * CKE) + TY).sqrt();
                let CKJ = BIV + ((staged[205] + (CKI * (CX * (CKE + CKH)))) * BIQ);
                let CKK = GQ * CKJ;
                let CKL = Lanes([(GR * CKJ), 0.0, 0.0, 0.0]) + (((((CKF + ((CKG + CKG) * (AK / (AJ * CKH)))) * CX) * CKI) * BIQ) * GQ);
                let CKM = AJU * (ALU - AJP);
                let CKN = (Lanes([ALV, 0.0]) - Lanes([0.0, AJS])) * AJU;
                let CKO = Lanes([0.0, AJW[0], AJW[1]]) - Lanes([CKN[0], 0.0, CKN[1]]);
                let CKP = (AJV - CKM) - CF;
                let CKQ = Lanes([0.0, CKO[0], CKO[1], CKO[2]]) - Lanes([CG, 0.0, 0.0, 0.0]);
                let CKR = CKQ * CKP;
                let CKS = ((CKP * CKP) + TY).sqrt();
                let CKT = U + (BGZ * (CX * (CKP + CKS)));
                let CKU = U / CKT;
                let CKV = CKN * BHB;
                let CKW = CKU + (BHB * CKM);
                let CKX = ((((((CKQ + ((CKR + CKR) * (AK / (AJ * CKS)))) * CX) * BGZ) * CKU) * W) / CKT) + Lanes([0.0, CKV[0], 0.0, CKV[1]]);
                let CKY = CKX * CKW;
                let CKZ = ((CKW * CKW) + TY).sqrt();
                let CLB = BIW + ((staged[207] + (CLA * (CX * (CKW + CKZ)))) * BIQ);
                let CLC = GQ * CLB;
                let CLD = Lanes([(GR * CLB), 0.0, 0.0, 0.0]) + (((((CKX + ((CKY + CKY) * (AK / (AJ * CKZ)))) * CX) * CLA) * BIQ) * GQ);
                CLX = U;
                CLY = CLC;
                CLZ = CKK;
                CMA = BCT;
                CMB = CLD;
                CMC = CKL;
            } else {
                let CLE = U + (BGZ * CBR);
                let CLF = AVB * BHB;
                let CLG = U / CLE;
                let CLH = CLG + (BHB * AVA);
                let CLI = ((((CBT * BGZ) * CLG) * W) / CLE) + Lanes([CLF[0], CLF[1], CLF[2], 0.0, CLF[3]]);
                let CLJ = CLI * CLH;
                let CLK = ((CLH * CLH) + TY).sqrt();
                let CLL = ((CLI + ((CLJ + CLJ) * (AK / (AJ * CLK)))) * CX) * BIO;
                let CLM = BIP + (BIO * (CX * (CLH + CLK)));
                let CLN = ((GQ * CLM) * BIQ) * BIR;
                let CLO = CJQ * CCN;
                let CLP = ARF / CLO;
                let CLQ = ((CLP * ASZ) * BKA) / AWB;
                let CLR = ((((CCS - (((CJR * CCN) + (CCO * CJQ)) * CLP)) / CLO) * ASZ) * BKA) / AWB;
                let CLS = CLQ * CBR;
                let CLT = CBT * CLQ;
                let CLU = (((CLR * CBR) + CLT) * CLN) + ((((Lanes([(GR * CLM), 0.0, 0.0, 0.0, 0.0]) + (CLL * GQ)) * BIQ) * BIR) * CLS);
                let CLV = U + (CLS * CLN);
                let COG;
                let COH;
                if CLW != 0.0 {
                    let COC = (BIV + ((CLM * BIQ) * BIR)) + BIW;
                    let COD = GQ * COC;
                    let COE = (((CLR * CBR) + CLT) * COD) + ((Lanes([(GR * COC), 0.0, 0.0, 0.0, 0.0]) + (((CLL * BIQ) * BIR) * GQ)) * CLS);
                    let COF = U + (CLS * COD);
                    COG = COF;
                    COH = COE;
                } else {
                    COG = CLV;
                    COH = CLU;
                }
                CLX = COG;
                CLY = COI;
                CLZ = COJ;
                CMA = COH;
                CMB = COK;
                CMC = COL;
            }
            let CMD = AO * ATL;
            let CME = CMD * S;
            let CMF = ((ATM * AO) * S) + Lanes([(T * CMD), 0.0, 0.0, 0.0]);
            let CMG = CBR + CME;
            let CMH = Lanes([CMF[0], CMF[1], CMF[2], 0.0, CMF[3]]);
            let CMI = MA / CMG;
            let CMJ = LL + CMI;
            let CMK = CMJ * BZZ;
            let CML = ((((Lanes([LM, 0.0, 0.0, 0.0, 0.0]) + ((Lanes([MB, 0.0, 0.0, 0.0, 0.0]) - ((CBT + CMH) * CMI)) / CMG)) * BZZ) + (CAA * CMJ)) * BZZ) + (CAA * CMK);
            let CMN = ((CMK * BZZ) + U) - CMM;
            let CMO = CML * CMN;
            let CMP = ((CMN * CMN) + 4e-3f64).sqrt();
            let CMQ = (U + (-1e0f64 + (CX * (CMN + CMP)))).sqrt();
            let CMR = CX * (U + CMQ);
            let CMS = (((CML + ((CMO + CMO) * (AK / (AJ * CMP)))) * CX) * (AK / (AJ * CMQ))) * CX;
            let CMT = CMR - U;
            let CMU = CMS * CMT;
            let CMV = ((CMT * CMT) + 2.5e-5f64).sqrt();
            let CMW = (CX * ((CMR + U) - CMV)) + 2.5e-3f64;
            let CMX = CAL + OX;
            let CMY = BZZ / CMX;
            let CMZ = (CAA - ((CAF + Lanes([OY, 0.0, 0.0, 0.0, 0.0])) * CMY)) / CMX;
            let CNA = OI * CMY;
            let CNB = U + (CNA * CMY);
            let CNC = QQ * BZZ;
            let CND = QB + (CNC * BZZ);
            let CNE = if A >= CND { A } else { CND };
            let CNF = (CNE * CAL) + CME;
            let CNG = PM / CNF;
            let CNH = -CNG;
            let CNI = rspice_limited_exp(CNH);
            let CNJ = CCN * CJQ;
            let CNK = CNJ * CLX;
            let CNL = ARF / CNK;
            let CNM = (CCS - (((((CCO * CJQ) + (CJR * CCN)) * CLX) + (CMA * CNJ)) * CNL)) / CNK;
            let CNO = CNN * BZY;
            let CNP = (((CNO * CNL) * BKA) / AWB) * ASZ;
            let CNQ = CNP * ATN;
            let CNR = ATO * CNP;
            let CNS = CNQ * ATN;
            let CNT = ATO * CNQ;
            let CNU = BZZ * CAE;
            let CNV = CNS * CNU;
            let CNW = (CNV * CHL) / CMW;
            let CNX = CNW * CNB;
            let CNZ = (CNX * CNI) * CNY;
            let COA = ((((((((((((((((((((BZX * CNN) * CNL) + (CNM * CNO)) * BKA) / AWB) * ASZ) * ATN) + Lanes([CNR[0], CNR[1], CNR[2], 0.0, CNR[3]])) * ATN) + Lanes([CNT[0], CNT[1], CNT[2], 0.0, CNT[3]])) * CNU) + (((CAA * CAE) + (CAF * BZZ)) * CNS)) * CHL) + (CHM * CNV)) - (((CMS - ((CMU + CMU) * (AK / (AJ * CMV)))) * CX) * CNW)) / CMW) * CNB) + ((((Lanes([(OJ * CMY), 0.0, 0.0, 0.0, 0.0]) + (CMZ * OI)) * CMY) + (CMZ * CNA)) * CNW)) * CNI) + (((((Lanes([PN, 0.0, 0.0, 0.0, 0.0]) - ((((((Lanes([QC, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(QR * BZZ), 0.0, 0.0, 0.0, 0.0]) + (CAA * QQ)) * BZZ) + (CAA * CNC))) * (AK - (if A >= CND { 1.0 } else { 0.0 }))) * CAL) + (CAF * CNE)) + CMH) * CNG)) / CNF) * W) * (rspice_limited_exp_derivative(CNH))) * CNX)) * CNY;
            let COV;
            let COW;
            let COX;
            let COY;
            let COZ;
            let CPA;
            let CPB;
            let CPC;
            if COB != 0.0 {
                let COM = AT * AT;
                let CON = staged[209] / COM;
                let COO = CON.ln();
                let COP = S * COO;
                let COQ = (T * COO) + ((((((AU * (AO * AT)) * CON) * W) / COM) * (AK / CON)) * S);
                let CPI;
                let CPJ;
                if K != 0.0 {
                    let CPE = COQ * COP;
                    let CPF = ((COP * COP) + BB).sqrt();
                    let CPG = S * CPF;
                    let CPH = (T * CPF) + (((CPE + CPE) * (AK / (AJ * CPF))) * S);
                    CPI = CPG;
                    CPJ = CPH;
                } else {
                    CPI = COP;
                    CPJ = COQ;
                }
                let CPL = U - (CPK * AKJ);
                let CPM = (AKK * CPK) * W;
                let CPN = if 1.0f64 != 0.0 && (if CPL < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CPV;
                let CPW;
                if CPN != 0.0 {
                    let CPO = ASC * CPL;
                    let CPP = -1e-6f64 / CPO;
                    let CPQ = (((CPM * ASC) * CPP) * W) / CPO;
                    CPV = CPP;
                    CPW = CPQ;
                } else {
                    let CPR = CPM * CPL;
                    let CPS = ((CPL * CPL) + 2.5e-7f64).sqrt();
                    let CPT = CX * (CPL + CPS);
                    let CPU = (CPM + ((CPR + CPR) * (AK / (AJ * CPS)))) * CX;
                    CPV = CPT;
                    CPW = CPU;
                }
                let CPX = BCP - parameters[1102];
                let CPY = if 0.0f64 != 0.0 && (if CPX < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CQH;
                let CQI;
                if CPY != 0.0 {
                    let CPZ = ASC * CPX;
                    let CQA = -4e0f64 / CPZ;
                    let CQB = (((BCQ * ASC) * CQA) * W) / CPZ;
                    CQH = CQA;
                    CQI = CQB;
                } else {
                    let CQC = CPX - BHW;
                    let CQD = BCQ * CQC;
                    let CQE = ((CQC * CQC) + 1e0f64).sqrt();
                    let CQF = CX * ((CPX + BHW) + CQE);
                    let CQG = (BCQ + ((CQD + CQD) * (AK / (AJ * CQE)))) * CX;
                    CQH = CQF;
                    CQI = CQG;
                }
                let CQK = CQJ + CQH;
                let CQL = (CQJ * CQH) / CQK;
                let CQN = U + (CQM * CQL);
                let CQP = CQO * (IN * CQN);
                let CQQ = (Lanes([(IP * CQN), 0.0, 0.0, 0.0, 0.0]) + (((((CQI * CQJ) - (CQI * CQL)) / CQK) * CQM) * IN)) * CQO;
                let CQX;
                let CQY;
                let CQZ;
                let CRA;
                let CRB;
                let CRC;
                if CQR != 0.0 {
                    let CQS = ALU - AJX;
                    let CQT = CQS.abs();
                    let CQV = (Lanes([0.0, ALV]) - Lanes([AJY, 0.0])) * ((AJ * (if CQS >= CQU { 1.0 } else { 0.0 })) - AK);
                    let CRH;
                    let CRI;
                    if CQW != 0.0 {
                        CRH = U;
                        CRI = CRE;
                    } else {
                        let CRF = CQT - parameters[1126];
                        let CRG = if 1.0f64 != 0.0 && (if CRF < -1.25e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CRQ;
                        let CRR;
                        if CRG != 0.0 {
                            let CRJ = ASC * CRF;
                            let CRK = -2.5e-1f64 / CRJ;
                            let CRL = (((CQV * ASC) * CRK) * W) / CRJ;
                            CRQ = CRK;
                            CRR = CRL;
                        } else {
                            let CRM = CQV * CRF;
                            let CRN = ((CRF * CRF) + 6.25e-2f64).sqrt();
                            let CRO = CX * (CRF + CRN);
                            let CRP = (CQV + ((CRM + CRM) * (AK / (AJ * CRN)))) * CX;
                            CRQ = CRO;
                            CRR = CRP;
                        }
                        let CRT = CRR * CRS;
                        let CRU = U + (CRQ * CRS);
                        CRH = CRU;
                        CRI = CRT;
                    }
                    let CSU;
                    let CSV;
                    let CSW;
                    let CSX;
                    if CRV != 0.0 {
                        let CRX = AJP - CRW;
                        let CRZ = (Lanes([0.0, AJS]) - Lanes([CRY, 0.0])) * CRX;
                        let CSA = ((CRX * CRX) + staged[213]).sqrt();
                        let CSB = (CRZ + CRZ) * (AK / (AJ * CSA));
                        let CSD = CQP * CSC;
                        let CSE = CSD * CRH;
                        let CSF = (CQQ * CSC) * CRH;
                        let CSG = CRI * CSD;
                        let CSJ = U + (CSI * (CSA.powf(CSH)));
                        let CSK = CSE * CSJ;
                        let CSL = (Lanes([CSF[0], CSF[1], 0.0, CSF[2], CSF[3], CSF[4]]) + Lanes([0.0, CSG[0], CSG[1], 0.0, 0.0, 0.0])) * CSJ;
                        let CSM = ((CSB * (CSH * (CSA.powf(staged[430])))) * CSI) * CSE;
                        let CSN = Lanes([0.0, CSL[0], CSL[1], CSL[2], CSL[3], CSL[4], CSL[5]]) + Lanes([CSM[0], 0.0, 0.0, 0.0, 0.0, 0.0, CSM[1]]);
                        CSU = CSK;
                        CSV = CSA;
                        CSW = CSN;
                        CSX = CSB;
                    } else {
                        let CSO = CQP * CSC;
                        let CSP = CSO * CRH;
                        let CSQ = (CQQ * CSC) * CRH;
                        let CSR = CRI * CSO;
                        let CSS = Lanes([CSQ[0], CSQ[1], 0.0, CSQ[2], CSQ[3], CSQ[4]]) + Lanes([0.0, CSR[0], CSR[1], 0.0, 0.0, 0.0]);
                        let CST = Lanes([0.0, CSS[0], CSS[1], CSS[2], CSS[3], CSS[4], CSS[5]]);
                        CSU = CSP;
                        CSV = A;
                        CSW = CST;
                        CSX = COS;
                    }
                    let CSY = AKJ / CPI;
                    let CSZ = (Lanes([0.0, AKK[0], AKK[1]]) - Lanes([(CPJ * CSY), 0.0, 0.0])) / CPI;
                    let CTA = U + CSY;
                    let CTB = if 1.0f64 != 0.0 && (if CTA < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTJ;
                    let CTK;
                    if CTB != 0.0 {
                        let CTC = ASC * CTA;
                        let CTD = -2.5000000000000005e-3f64 / CTC;
                        let CTE = (((CSZ * ASC) * CTD) * W) / CTC;
                        CTJ = CTD;
                        CTK = CTE;
                    } else {
                        let CTF = CSZ * CTA;
                        let CTG = ((CTA * CTA) + 6.250000000000001e-4f64).sqrt();
                        let CTH = CX * (CTA + CTG);
                        let CTI = (CSZ + ((CTF + CTF) * (AK / (AJ * CTG)))) * CX;
                        CTJ = CTH;
                        CTK = CTI;
                    }
                    let CTL = CTJ.sqrt();
                    let CTO = AKK * CTN;
                    let CTP = (U - (CTM * (CTL - U))) - (CTN * AKJ);
                    let CTQ = (((CTK * (AK / (AJ * CTL))) * CTM) * W) - Lanes([0.0, CTO[0], CTO[1]]);
                    let CTR = if 1.0f64 != 0.0 && (if CTP < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTZ;
                    let CUA;
                    if CTR != 0.0 {
                        let CTS = ASC * CTP;
                        let CTT = -2.5000000000000005e-3f64 / CTS;
                        let CTU = (((CTQ * ASC) * CTT) * W) / CTS;
                        CTZ = CTT;
                        CUA = CTU;
                    } else {
                        let CTV = CTQ * CTP;
                        let CTW = ((CTP * CTP) + 6.250000000000001e-4f64).sqrt();
                        let CTX = CX * (CTP + CTW);
                        let CTY = (CTQ + ((CTV + CTV) * (AK / (AJ * CTW)))) * CX;
                        CTZ = CTX;
                        CUA = CTY;
                    }
                    let CUB = CTZ * CSU;
                    let CUC = CUA * CSU;
                    let CUD = Lanes([0.0, CUC[0], 0.0, 0.0, CUC[1], 0.0, CUC[2]]) + (CSW * CTZ);
                    let CUF = (IO * CUE) * BIQ;
                    let CUG = CUF * CPV;
                    let CUH = CPW * CUF;
                    let CUI = Lanes([(((IQ * CUE) * BIQ) * CPV), 0.0, 0.0]) + Lanes([0.0, CUH[0], CUH[1]]);
                    let CUJ = CUB * CUG;
                    let CUK = CUI * CUB;
                    let CUL = (CUD * CUG) + Lanes([0.0, CUK[0], 0.0, 0.0, CUK[1], 0.0, CUK[2]]);
                    let CUN = CQT.powf(CUM);
                    let CUP = CQV * (CUM * (CQT.powf(CUO)));
                    let CUR = CUN + (CUQ * (CUJ.powf(CUM)));
                    let CUS = Lanes([0.0, 0.0, CUP[0], CUP[1], 0.0, 0.0, 0.0]);
                    let CUT = CUN / CUR;
                    let CUV = CUT.powf(CUU);
                    let CUX = CQV * CUV;
                    let CUY = (CUV * CQT) / CUJ;
                    let CUZ = ((((((CUS - ((CUS + ((CUL * (CUM * (CUJ.powf(CUO)))) * CUQ)) * CUT)) / CUR) * (CUU * (CUT.powf(CUW)))) * CQT) + Lanes([0.0, 0.0, CUX[0], CUX[1], 0.0, 0.0, 0.0])) - (CUL * CUY)) / CUJ;
                    let CVA = if 1.0f64 != 0.0 && (if CUY < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CVI;
                    let CVJ;
                    if CVA != 0.0 {
                        let CVB = ASC * CUY;
                        let CVC = -1e-6f64 / CVB;
                        let CVD = (((CUZ * ASC) * CVC) * W) / CVB;
                        CVI = CVC;
                        CVJ = CVD;
                    } else {
                        let CVE = CUZ * CUY;
                        let CVF = ((CUY * CUY) + 2.5e-7f64).sqrt();
                        let CVG = CX * (CUY + CVF);
                        let CVH = (CUZ + ((CVE + CVE) * (AK / (AJ * CVF)))) * CX;
                        CVI = CVG;
                        CVJ = CVH;
                    }
                    let CVL = U + (CVI.powf(CVK));
                    let CVM = CVL.powf(CUU);
                    let CVN = CUG * CVM;
                    let CVO = CUI * CVM;
                    let CVP = Lanes([0.0, CVO[0], 0.0, 0.0, CVO[1], 0.0, CVO[2]]) + (((CVJ * (CVK * (CVI.powf(staged[433])))) * (CUU * (CVL.powf(CUW)))) * CUG);
                    CQX = CUB;
                    CQY = CSV;
                    CQZ = CVN;
                    CRA = CUD;
                    CRB = CSX;
                    CRC = CVP;
                } else {
                    CQX = A;
                    CQY = A;
                    CQZ = A;
                    CRA = COT;
                    CRB = COS;
                    CRC = COT;
                }
                let CVV;
                let CVW;
                let CVX;
                let CVY;
                let CVZ;
                let CWA;
                if CRD != 0.0 {
                    let CVQ = AKD - CJS;
                    let CVR = CVQ.abs();
                    let CVS = (Lanes([AKF, 0.0]) - Lanes([0.0, CJT])) * ((AJ * (if CVQ >= CQU { 1.0 } else { 0.0 })) - AK);
                    let CWT;
                    let CWU;
                    let CWV;
                    let CWW;
                    if CWC != 0.0 {
                        let CWD = AJP - CRW;
                        let CWE = (Lanes([0.0, AJS]) - Lanes([CRY, 0.0])) * CWD;
                        let CWF = ((CWD * CWD) + staged[217]).sqrt();
                        let CWG = (CWE + CWE) * (AK / (AJ * CWF));
                        let CWI = CQP * CWH;
                        let CWL = U + (CWK * (CWF.powf(CWJ)));
                        let CWM = CWI * CWL;
                        let CWN = (CQQ * CWH) * CWL;
                        let CWO = ((CWG * (CWJ * (CWF.powf(staged[434])))) * CWK) * CWI;
                        let CWP = Lanes([0.0, CWN[0], CWN[1], CWN[2], CWN[3], CWN[4]]) + Lanes([CWO[0], 0.0, 0.0, 0.0, 0.0, CWO[1]]);
                        CWT = CWM;
                        CWU = CWF;
                        CWV = CWP;
                        CWW = CWG;
                    } else {
                        let CWQ = CQP * CWH;
                        let CWR = CQQ * CWH;
                        let CWS = Lanes([0.0, CWR[0], CWR[1], CWR[2], CWR[3], CWR[4]]);
                        CWT = CWQ;
                        CWU = CQY;
                        CWV = CWS;
                        CWW = CRB;
                    }
                    let CWX = AKJ / CPI;
                    let CWY = (Lanes([0.0, AKK[0], AKK[1]]) - Lanes([(CPJ * CWX), 0.0, 0.0])) / CPI;
                    let CWZ = U + CWX;
                    let CXA = if 1.0f64 != 0.0 && (if CWZ < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CXI;
                    let CXJ;
                    if CXA != 0.0 {
                        let CXB = ASC * CWZ;
                        let CXC = -2.5000000000000005e-3f64 / CXB;
                        let CXD = (((CWY * ASC) * CXC) * W) / CXB;
                        CXI = CXC;
                        CXJ = CXD;
                    } else {
                        let CXE = CWY * CWZ;
                        let CXF = ((CWZ * CWZ) + 6.250000000000001e-4f64).sqrt();
                        let CXG = CX * (CWZ + CXF);
                        let CXH = (CWY + ((CXE + CXE) * (AK / (AJ * CXF)))) * CX;
                        CXI = CXG;
                        CXJ = CXH;
                    }
                    let CXK = CXI.sqrt();
                    let CXL = AKK * CTN;
                    let CXM = (U - (CTM * (CXK - U))) - (CTN * AKJ);
                    let CXN = (((CXJ * (AK / (AJ * CXK))) * CTM) * W) - Lanes([0.0, CXL[0], CXL[1]]);
                    let CXO = if 1.0f64 != 0.0 && (if CXM < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CXW;
                    let CXX;
                    if CXO != 0.0 {
                        let CXP = ASC * CXM;
                        let CXQ = -2.5000000000000005e-3f64 / CXP;
                        let CXR = (((CXN * ASC) * CXQ) * W) / CXP;
                        CXW = CXQ;
                        CXX = CXR;
                    } else {
                        let CXS = CXN * CXM;
                        let CXT = ((CXM * CXM) + 6.250000000000001e-4f64).sqrt();
                        let CXU = CX * (CXM + CXT);
                        let CXV = (CXN + ((CXS + CXS) * (AK / (AJ * CXT)))) * CX;
                        CXW = CXU;
                        CXX = CXV;
                    }
                    let CXY = CXW * CWT;
                    let CXZ = CXX * CWT;
                    let CYA = Lanes([0.0, CXZ[0], 0.0, CXZ[1], 0.0, CXZ[2]]) + (CWV * CXW);
                    let CYC = (IO * CYB) * BIQ;
                    let CYD = CYC * CPV;
                    let CYE = CPW * CYC;
                    let CYF = Lanes([(((IQ * CYB) * BIQ) * CPV), 0.0, 0.0]) + Lanes([0.0, CYE[0], CYE[1]]);
                    let CYG = CXY * CYD;
                    let CYH = CYF * CXY;
                    let CYI = (CYA * CYD) + Lanes([0.0, CYH[0], 0.0, CYH[1], 0.0, CYH[2]]);
                    let CYK = CVR.powf(CYJ);
                    let CYM = CVS * (CYJ * (CVR.powf(CYL)));
                    let CYN = (CYI * (CYJ * (CYG.powf(CYL)))) * CUQ;
                    let CYO = CYK + (CUQ * (CYG.powf(CYJ)));
                    let CYP = Lanes([0.0, 0.0, 0.0, CYM[0], CYM[1], 0.0, 0.0]);
                    let CYQ = CYK / CYO;
                    let CYS = CYQ.powf(CYR);
                    let CYU = CVS * CYS;
                    let CYV = (CYS * CVR) / CYG;
                    let CYW = CYI * CYV;
                    let CYX = ((((((CYP - ((CYP + Lanes([CYN[0], CYN[1], CYN[2], CYN[3], 0.0, CYN[4], CYN[5]])) * CYQ)) / CYO) * (CYR * (CYQ.powf(CYT)))) * CVR) + Lanes([0.0, 0.0, 0.0, CYU[0], CYU[1], 0.0, 0.0])) - Lanes([CYW[0], CYW[1], CYW[2], CYW[3], 0.0, CYW[4], CYW[5]])) / CYG;
                    let CYY = if 1.0f64 != 0.0 && (if CYV < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CZG;
                    let CZH;
                    if CYY != 0.0 {
                        let CYZ = ASC * CYV;
                        let CZA = -1e-6f64 / CYZ;
                        let CZB = (((CYX * ASC) * CZA) * W) / CYZ;
                        CZG = CZA;
                        CZH = CZB;
                    } else {
                        let CZC = CYX * CYV;
                        let CZD = ((CYV * CYV) + 2.5e-7f64).sqrt();
                        let CZE = CX * (CYV + CZD);
                        let CZF = (CYX + ((CZC + CZC) * (AK / (AJ * CZD)))) * CX;
                        CZG = CZE;
                        CZH = CZF;
                    }
                    let CZI = U + (CZG.powf(CVK));
                    let CZJ = CZI.powf(CYR);
                    let CZK = CYD * CZJ;
                    let CZL = CYF * CZJ;
                    let CZM = Lanes([0.0, CZL[0], 0.0, CZL[1], 0.0, 0.0, CZL[2]]) + (((CZH * (CVK * (CZG.powf(staged[437])))) * (CYR * (CZI.powf(CYT)))) * CYD);
                    CVV = CXY;
                    CVW = CWU;
                    CVX = CZK;
                    CVY = CYA;
                    CVZ = CWW;
                    CWA = CZM;
                } else {
                    CVV = CVT;
                    CVW = CQY;
                    CVX = A;
                    CVY = CVU;
                    CVZ = CRB;
                    CWA = COU;
                }
                let DAB;
                let DAC;
                if CWB != 0.0 {
                    let CZN = COA * AMZ;
                    let CZO = if CQX <= CVV { CQX } else { CVV };
                    let CZP = Lanes([CVY[0], CVY[1], CVY[2], 0.0, CVY[3], CVY[4], CVY[5]]);
                    let CZQ = CZP + ((CRA - CZP) * (if CQX <= CVV { 1.0 } else { 0.0 }));
                    let CZR = (AMZ * CNZ) / CZO;
                    let CZS = (Lanes([0.0, CZN[0], CZN[1], 0.0, CZN[2], CZN[3], CZN[4]]) - (CZQ * CZR)) / CZO;
                    let CZT = CZR - U;
                    let CZU = CZS * CZT;
                    let CZW = ((CZT * CZT) + CZV).sqrt();
                    let CZX = (CZS - ((CZU + CZU) * (AK / (AJ * CZW)))) * CX;
                    let CZZ = ((((CX * ((CZR + U) - CZW)) + CZY) + staged[222]) - CX) - CZY;
                    let DAA = if 0.0f64 != 0.0 && (if CZZ < staged[223] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DAM;
                    let DAN;
                    if DAA != 0.0 {
                        let DAE = ASC * CZZ;
                        let DAF = ((-DAD) * DAD) / DAE;
                        let DAG = (((CZX * ASC) * DAF) * W) / DAE;
                        DAM = DAF;
                        DAN = DAG;
                    } else {
                        let DAH = CZZ - -1e0f64;
                        let DAI = CZZ - -1e0f64;
                        let DAJ = ((DAH * DAI) + CZV).sqrt();
                        let DAK = CX * ((CZZ + -1e0f64) + DAJ);
                        let DAL = (CZX + (((CZX * DAI) + (CZX * DAH)) * (AK / (AJ * DAJ)))) * CX;
                        DAM = DAK;
                        DAN = DAL;
                    }
                    let DAO = (DAM - staged[224]) + CX;
                    let DAP = AMZ * CZO;
                    let DAQ = DAP * DAO;
                    let DAR = ((CZQ * AMZ) * DAO) + (DAN * DAP);
                    DAB = DAQ;
                    DAC = DAR;
                } else {
                    let DBE;
                    let DBF;
                    if CQR != 0.0 {
                        let DAS = COA * AMZ;
                        let DAT = (AMZ * CNZ) / CQX;
                        let DAU = (Lanes([0.0, DAS[0], DAS[1], 0.0, DAS[2], DAS[3], DAS[4]]) - (CRA * DAT)) / CQX;
                        let DAV = DAT - U;
                        let DAW = DAU * DAV;
                        let DAY = ((DAV * DAV) + DAX).sqrt();
                        let DAZ = (DAU - ((DAW + DAW) * (AK / (AJ * DAY)))) * CX;
                        let DBB = ((((CX * ((DAT + U) - DAY)) + DBA) + staged[227]) - CX) - DBA;
                        let DBC = if 0.0f64 != 0.0 && (if DBB < staged[228] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let DBO;
                        let DBP;
                        if DBC != 0.0 {
                            let DBG = ASC * DBB;
                            let DBH = ((-DAD) * DAD) / DBG;
                            let DBI = (((DAZ * ASC) * DBH) * W) / DBG;
                            DBO = DBH;
                            DBP = DBI;
                        } else {
                            let DBJ = DBB - -1e0f64;
                            let DBK = DBB - -1e0f64;
                            let DBL = ((DBJ * DBK) + DAX).sqrt();
                            let DBM = CX * ((DBB + -1e0f64) + DBL);
                            let DBN = (DAZ + (((DAZ * DBK) + (DAZ * DBJ)) * (AK / (AJ * DBL)))) * CX;
                            DBO = DBM;
                            DBP = DBN;
                        }
                        let DBQ = (DBO - staged[229]) + CX;
                        let DBR = AMZ * CQX;
                        let DBS = DBR * DBQ;
                        let DBT = ((CRA * AMZ) * DBQ) + (DBP * DBR);
                        DBE = DBS;
                        DBF = DBT;
                    } else {
                        let DBD = Lanes([0.0, COA[0], COA[1], 0.0, COA[2], COA[3], COA[4]]);
                        DBE = CNZ;
                        DBF = DBD;
                    }
                    let DCF;
                    let DCG;
                    if CRD != 0.0 {
                        let DBU = (AMZ * DBE) / CVV;
                        let DBV = CVY * DBU;
                        let DBW = ((DBF * AMZ) - Lanes([DBV[0], DBV[1], DBV[2], 0.0, DBV[3], DBV[4], DBV[5]])) / CVV;
                        let DBX = DBU - U;
                        let DBY = DBW * DBX;
                        let DCA = ((DBX * DBX) + DBZ).sqrt();
                        let DCB = (DBW - ((DBY + DBY) * (AK / (AJ * DCA)))) * CX;
                        let DCD = ((((CX * ((DBU + U) - DCA)) + DCC) + staged[232]) - CX) - DCC;
                        let DCE = if 0.0f64 != 0.0 && (if DCD < staged[233] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let DCP;
                        let DCQ;
                        if DCE != 0.0 {
                            let DCH = ASC * DCD;
                            let DCI = ((-DAD) * DAD) / DCH;
                            let DCJ = (((DCB * ASC) * DCI) * W) / DCH;
                            DCP = DCI;
                            DCQ = DCJ;
                        } else {
                            let DCK = DCD - -1e0f64;
                            let DCL = DCD - -1e0f64;
                            let DCM = ((DCK * DCL) + DBZ).sqrt();
                            let DCN = CX * ((DCD + -1e0f64) + DCM);
                            let DCO = (DCB + (((DCB * DCL) + (DCB * DCK)) * (AK / (AJ * DCM)))) * CX;
                            DCP = DCN;
                            DCQ = DCO;
                        }
                        let DCR = (DCP - staged[234]) + CX;
                        let DCS = AMZ * CVV;
                        let DCT = DCS * DCR;
                        let DCU = (CVY * AMZ) * DCR;
                        let DCV = Lanes([DCU[0], DCU[1], DCU[2], 0.0, DCU[3], DCU[4], DCU[5]]) + (DCQ * DCS);
                        DCF = DCT;
                        DCG = DCV;
                    } else {
                        DCF = DBE;
                        DCG = DBF;
                    }
                    DAB = DCF;
                    DAC = DCG;
                }
                COV = DAB;
                COW = CVW;
                COX = CQZ;
                COY = CVX;
                COZ = DAC;
                CPA = CVZ;
                CPB = CRC;
                CPC = CWA;
            } else {
                let COR = Lanes([0.0, COA[0], COA[1], 0.0, COA[2], COA[3], COA[4]]);
                COV = CNZ;
                COW = A;
                COX = A;
                COY = A;
                COZ = COR;
                CPA = COS;
                CPB = COT;
                CPC = COU;
            }
            let DDT;
            let DDU;
            let DDV;
            let DDW;
            let DDX;
            let DDY;
            let DDZ;
            let DEA;
            if CPD != 0.0 {
                let DCW = AMN * W;
                let DCY = ((-AMK) - DCX) / S;
                let DCZ = (Lanes([0.0, DCW[0], DCW[1], DCW[2], DCW[3]]) - Lanes([(T * DCY), 0.0, 0.0, 0.0, 0.0])) / S;
                let DDB = (DDA * V).sqrt();
                let DDC = DDB / ASZ;
                let DDD = ((X * DDA) * (AK / (AJ * DDB))) / ASZ;
                let DDE = parameters[1117] / AT;
                let DDF = if DDE >= AX { DDE } else { AX };
                let DDG = DDF.ln();
                let DDH = ((((AU * DDE) * W) / AT) * (if DDE >= AX { 1.0 } else { 0.0 })) * (AK / DDF);
                let DDI = AYC * (U + (DDC / AYB));
                let DDJ = (DDD / AYB) * AYC;
                let DDK = (CX * DCY) - DDI;
                let DDL = (DCZ * CX) - Lanes([DDJ, 0.0, 0.0, 0.0, 0.0]);
                let DDM = DDL * DDK;
                let DDN = ((DDK * DDK) + (AYH * DCY)).sqrt();
                let DDO = DDK + DDN;
                let DDP = DDL + (((DDM + DDM) + (DCZ * AYH)) * (AK / (AJ * DDN)));
                let DDQ = if DCY < A { 1.0 } else { 0.0 };
                let DET;
                let DEU;
                if DDQ != 0.0 {
                    let DEC = (DCY - DDO) / DDC;
                    let DED = (((DCZ - DDP) - Lanes([(DDD * DEC), 0.0, 0.0, 0.0, 0.0])) / DDC) * DEC;
                    let DEE = (U - DDO) + (DEC * DEC);
                    let DEF = if DEE >= AX { DEE } else { AX };
                    let DEG = -(DEF.ln());
                    let DEH = ((((DDP * W) + (DED + DED)) * (if DEE >= AX { 1.0 } else { 0.0 })) * (AK / DEF)) * W;
                    DET = DEG;
                    DEU = DEH;
                } else {
                    let DEI = -DDO;
                    let DEJ = rspice_limited_exp(DEI);
                    let DEK = (DDP * W) * (rspice_limited_exp_derivative(DEI));
                    let DEL = CX * DDC;
                    let DEM = DDD * CX;
                    let DEN = DEM * DEL;
                    let DEO = (((DCY - U) + DEJ) + (DEL * DEL)).sqrt();
                    let DEP = DEO - DEL;
                    let DEQ = ((((DCZ + DEK) + Lanes([(DEN + DEN), 0.0, 0.0, 0.0, 0.0])) * (AK / (AJ * DEO))) - Lanes([DEM, 0.0, 0.0, 0.0, 0.0])) * DEP;
                    let DER = ((DEP * DEP) + U) - DEJ;
                    let DES = (DEQ + DEQ) - DEK;
                    DET = DER;
                    DEU = DES;
                }
                let DEV = DET + U;
                let DEW = DET - U;
                let DEX = DEW * DEW;
                let DEY = DEU * DEW;
                let DEZ = DEY + DEY;
                let DFA = (DEX + 1e0f64).sqrt();
                let DFB = (CX * (DEV + DFA)).sqrt();
                let DFC = ((DEU + (DEZ * (AK / (AJ * DFA)))) * CX) * (AK / (AJ * DFB));
                let DFD = AO * DFB;
                let DFE = DFC * AO;
                let DFF = DDC / DFD;
                let DFG = Lanes([DDD, 0.0, 0.0, 0.0, 0.0]);
                let DFH = (U + DFF) / DDC;
                let DFI = (((DFG - (DFE * DFF)) / DFD) - Lanes([(DDD * DFH), 0.0, 0.0, 0.0, 0.0])) / DDC;
                let DFJ = AO * DDG;
                let DFK = DDH * AO;
                let DFL = AMJ / S;
                let DFM = (Lanes([0.0, AMM[0], AMM[1], AMM[2]]) - Lanes([(T * DFL), 0.0, 0.0, 0.0])) / S;
                let DFN = (DET - DFJ) - DFL;
                let DFO = (DEU - Lanes([DFK, 0.0, 0.0, 0.0, 0.0])) - Lanes([DFM[0], DFM[1], DFM[2], 0.0, DFM[3]]);
                let DFP = ADB * DFH;
                let DFQ = DFP * DFB;
                let DFR = if DFQ >= AX { DFQ } else { AX };
                let DFS = DFN - (DFR.ln());
                let DFT = DFO - (((((DFI * ADB) * DFB) + (DFC * DFP)) * (if DFQ >= AX { 1.0 } else { 0.0 })) * (AK / DFR));
                let DFU = DFS + BAH;
                let DFV = ((DFS * DFU) + BAJ).sqrt();
                let DFW = CX * ((DFS - BAG) - DFV);
                let DFX = (DFT - (((DFT * DFU) + (DFT * DFS)) * (AK / (AJ * DFV)))) * CX;
                let DFY = if DFW <= -6.8e1f64 { 1.0 } else { 0.0 };
                let DIA;
                let DIB;
                if DFY != 0.0 {
                    let DFZ = if DFW < -1.1e2f64 { 1.0 } else { 0.0 };
                    let DIF;
                    let DIG;
                    if DFZ != 0.0 {
                        DIF = DID;
                        DIG = DDR;
                    } else {
                        let DIE = if DFW > -9e1f64 { 1.0 } else { 0.0 };
                        let DJC;
                        let DJD;
                        if DIE != 0.0 {
                            let DIP = rspice_limited_exp(DFW);
                            let DIQ = DFX * (rspice_limited_exp_derivative(DFW));
                            DJC = DIP;
                            DJD = DIQ;
                        } else {
                            let DIS = (DFW - DIR) / BDI;
                            let DIT = DFX / BDI;
                            let DIU = DIS * DIS;
                            let DIV = DIT * DIS;
                            let DIW = DIV + DIV;
                            let DIX = BDO - DIU;
                            let DIY = 9.375e-1f64 - (DIU * DIX);
                            let DIZ = DIR + (BDI * ((7.8125e-2f64 + (CX * DIS)) + (DIU * DIY)));
                            let DJA = rspice_limited_exp(DIZ);
                            let DJB = (((DIT * CX) + ((DIW * DIY) + ((((DIW * DIX) + ((DIW * W) * DIU)) * W) * DIU))) * BDI) * (rspice_limited_exp_derivative(DIZ));
                            DJC = DJA;
                            DJD = DJB;
                        }
                        DIF = DJC;
                        DIG = DJD;
                    }
                    let DIH = AO * DFH;
                    let DII = DIF * AO;
                    let DIJ = (DII * DFH) + DFD;
                    let DIK = DIH * DIJ;
                    let DIL = if DIK >= AX { DIK } else { AX };
                    let DIM = ((U + DFN) - DFW) - (DIL.ln());
                    let DIN = DIF * DIM;
                    let DIO = (DIG * DIM) + (((DFO - DFX) - (((((DFI * AO) * DIJ) + (((((DIG * AO) * DFH) + (DFI * DII)) + DFE) * DIH)) * (if DIK >= AX { 1.0 } else { 0.0 })) * (AK / DIL))) * DIF);
                    DIA = DIN;
                    DIB = DIO;
                } else {
                    let DGA = rspice_limited_exp(DFW);
                    let DGB = DFX * (rspice_limited_exp_derivative(DFW));
                    let DGC = U / DFB;
                    let DGD = AO * DGA;
                    let DGE = DGB * AO;
                    let DGF = DGD * DFH;
                    let DGG = (DGE * DFH) + (DFI * DGD);
                    let DGH = DGF + DFD;
                    let DGI = DGF * DGH;
                    let DGJ = if DGI >= AX { DGI } else { AX };
                    let DGK = 1e0f64 / DGA;
                    let DGL = DFH + DGC;
                    let DGM = DFI + (((DFC * DGC) * W) / DFB);
                    let DGN = (DFH * DGA) + DFB;
                    let DGO = DGL / DGN;
                    let DGP = (AO + DGK) + DGO;
                    let DGQ = ((DGD + (DGJ.ln())) - DFN) / DGP;
                    let DGR = DGA - DGQ;
                    let DGS = DGB - ((((DGE + ((((DGG * DGH) + ((DGG + DFE) * DGF)) * (if DGI >= AX { 1.0 } else { 0.0 })) * (AK / DGJ))) - DFO) - (((((DGB * DGK) * W) / DGA) + ((DGM - ((((DFI * DGA) + (DGB * DFH)) + DFC) * DGO)) / DGN)) * DGQ)) / DGP);
                    let DGT = AO * DGR;
                    let DGU = DGS * AO;
                    let DGV = DGT * DFH;
                    let DGW = (DGU * DFH) + (DFI * DGT);
                    let DGX = DGV + DFD;
                    let DGY = DGV * DGX;
                    let DGZ = if DGY >= AX { DGY } else { AX };
                    let DHA = (DGT + (DGZ.ln())) - DFN;
                    let DHB = (DGU + ((((DGW * DGX) + ((DGW + DFE) * DGV)) * (if DGY >= AX { 1.0 } else { 0.0 })) * (AK / DGZ))) - DFO;
                    let DHC = 1e0f64 / DGR;
                    let DHD = (DFH * DGR) + DFB;
                    let DHE = ((DFI * DGR) + (DGS * DFH)) + DFC;
                    let DHF = DGL / DHD;
                    let DHG = (DGM - (DHE * DHF)) / DHD;
                    let DHH = (AO + DHC) + DHF;
                    let DHI = (((DGS * DHC) * W) / DGR) + DHG;
                    let DHJ = DHG * DHF;
                    let DHK = U / DGR;
                    let DHL = (((DGS * DHK) * W) / DGR) * DHK;
                    let DHN = DFB * DFB;
                    let DHO = DFC * DFB;
                    let DHP = DHN * DFB;
                    let DHQ = DHP * DHD;
                    let DHR = 1e0f64 / DHQ;
                    let DHS = ((DHM * (DHK * DHK)) - DHR) - (DHF * DHF);
                    let DHT = DHA / DHH;
                    let DHU = AO * DHH;
                    let DHV = DHU * DHH;
                    let DHW = (DHA * DHS) / DHV;
                    let DHX = U + DHW;
                    let DHY = DGR - (DHT * DHX);
                    let DHZ = DGS - ((((DHB - (DHI * DHT)) / DHH) * DHX) + (((((DHB * DHS) + (((((DHL + DHL) * DHM) - ((((((((DHO + DHO) * DFB) + (DFC * DHN)) * DHD) + (DHE * DHP)) * DHR) * W) / DHQ)) - (DHJ + DHJ)) * DHA)) - ((((DHI * AO) * DHH) + (DHI * DHU)) * DHW)) / DHV) * DHT));
                    DIA = DHY;
                    DIB = DHZ;
                }
                let DIC = if 0.0f64 != 0.0 && (if DET < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DJK;
                let DJL;
                if DIC != 0.0 {
                    let DJE = ASC * DET;
                    let DJF = -4e0f64 / DJE;
                    let DJG = (((DEU * ASC) * DJF) * W) / DJE;
                    DJK = DJF;
                    DJL = DJG;
                } else {
                    let DJH = (DEX + 1e0f64).sqrt();
                    let DJI = CX * (DEV + DJH);
                    let DJJ = (DEU + (DEZ * (AK / (AJ * DJH)))) * CX;
                    DJK = DJI;
                    DJL = DJJ;
                }
                let DJM = DJK.sqrt();
                let DJN = DJL * (AK / (AJ * DJM));
                let DJO = DET - (AO * DIA);
                let DJP = DEU - (DIB * AO);
                let DJQ = if 0.0f64 != 0.0 && (if DJO < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DJZ;
                let DKA;
                if DJQ != 0.0 {
                    let DJR = ASC * DJO;
                    let DJS = -4e0f64 / DJR;
                    let DJT = (((DJP * ASC) * DJS) * W) / DJR;
                    DJZ = DJS;
                    DKA = DJT;
                } else {
                    let DJU = DJO - U;
                    let DJV = DJP * DJU;
                    let DJW = ((DJU * DJU) + 1e0f64).sqrt();
                    let DJX = CX * ((DJO + U) + DJW);
                    let DJY = (DJP + ((DJV + DJV) * (AK / (AJ * DJW)))) * CX;
                    DJZ = DJX;
                    DKA = DJY;
                }
                let DKB = DJZ.sqrt();
                let DKC = DJM + DKB;
                let DKD = DDC / DKC;
                let DKE = (DFG - ((DJN + (DKA * (AK / (AJ * DKB)))) * DKD)) / DKC;
                let DKF = U + DKD;
                let DKH = DKG * S;
                let DKI = T * DKG;
                let DKJ = AO * DKF;
                let DKK = (DCY - DJO) - (DKJ * DIA);
                let DKL = DKH * DKK;
                let DKM = Lanes([(DKI * DKK), 0.0, 0.0, 0.0, 0.0]) + (((DCZ - DJP) - (((DKE * AO) * DIA) + (DIB * DKJ))) * DKH);
                let DKW;
                let DKX;
                if DKN != 0.0 {
                    let DKP = U + (DIA / DKO);
                    let DKQ = staged[238] / DKP;
                    let DKS = staged[240] + (DKQ / DKR);
                    let DKT = 3.4531302e-11f64 / DKS;
                    let DKU = (((((((DIB / DKO) * DKQ) * W) / DKP) / DKR) * DKT) * W) / DKS;
                    DKW = DKT;
                    DKX = DKU;
                } else {
                    DKW = DKV;
                    DKX = DDR;
                }
                let DKZ = DKY * DKF;
                let DLA = DKZ * S;
                let DLB = DLA * DKW;
                let DLC = DLB * DIA;
                let DLD = ((((((DKE * DKY) * S) + Lanes([(T * DKZ), 0.0, 0.0, 0.0, 0.0])) * DKW) + (DKX * DLA)) * DIA) + (DIB * DLB);
                let DLQ;
                let DLR;
                let DLS;
                let DLT;
                if DLE != 0.0 {
                    let DLG = ALQ * DLF;
                    let DLH = ((DLF * ALP) - DCX) / S;
                    let DLI = (Lanes([0.0, DLG[0], DLG[1]]) - Lanes([(T * DLH), 0.0, 0.0])) / S;
                    let DLJ = (CX * DLH) - DDI;
                    let DLK = (DLI * CX) - Lanes([DDJ, 0.0, 0.0]);
                    let DLL = DLK * DLJ;
                    let DLM = ((DLJ * DLJ) + (AYH * DLH)).sqrt();
                    let DLN = DLJ + DLM;
                    let DLO = DLK + (((DLL + DLL) + (DLI * AYH)) * (AK / (AJ * DLM)));
                    let DLP = if DLH < A { 1.0 } else { 0.0 };
                    let DML;
                    let DMM;
                    if DLP != 0.0 {
                        let DLU = (DLH - DLN) / DDC;
                        let DLV = (((DLI - DLO) - Lanes([(DDD * DLU), 0.0, 0.0])) / DDC) * DLU;
                        let DLW = (U - DLN) + (DLU * DLU);
                        let DLX = if DLW >= AX { DLW } else { AX };
                        let DLY = -(DLX.ln());
                        let DLZ = ((((DLO * W) + (DLV + DLV)) * (if DLW >= AX { 1.0 } else { 0.0 })) * (AK / DLX)) * W;
                        DML = DLY;
                        DMM = DLZ;
                    } else {
                        let DMA = -DLN;
                        let DMB = rspice_limited_exp(DMA);
                        let DMC = (DLO * W) * (rspice_limited_exp_derivative(DMA));
                        let DMD = CX * DDC;
                        let DME = DDD * CX;
                        let DMF = DME * DMD;
                        let DMG = (((DLH - U) + DMB) + (DMD * DMD)).sqrt();
                        let DMH = DMG - DMD;
                        let DMI = ((((DLI + DMC) + Lanes([(DMF + DMF), 0.0, 0.0])) * (AK / (AJ * DMG))) - Lanes([DME, 0.0, 0.0])) * DMH;
                        let DMJ = ((DMH * DMH) + U) - DMB;
                        let DMK = (DMI + DMI) - DMC;
                        DML = DMJ;
                        DMM = DMK;
                    }
                    let DMN = DML + U;
                    let DMO = DML - U;
                    let DMP = DMO * DMO;
                    let DMQ = DMM * DMO;
                    let DMR = DMQ + DMQ;
                    let DMS = (DMP + 1e0f64).sqrt();
                    let DMT = (CX * (DMN + DMS)).sqrt();
                    let DMU = ((DMM + (DMR * (AK / (AJ * DMS)))) * CX) * (AK / (AJ * DMT));
                    let DMV = AO * DMT;
                    let DMW = DMU * AO;
                    let DMX = DDC / DMV;
                    let DMY = (U + DMX) / DDC;
                    let DMZ = (((Lanes([DDD, 0.0, 0.0]) - (DMW * DMX)) / DMV) - Lanes([(DDD * DMY), 0.0, 0.0])) / DDC;
                    let DNA = DMM - Lanes([DFK, 0.0, 0.0]);
                    let DNB = AKJ / S;
                    let DNC = (Lanes([0.0, AKK[0], AKK[1]]) - Lanes([(T * DNB), 0.0, 0.0])) / S;
                    let DND = (DML - DFJ) - DNB;
                    let DNE = Lanes([DNA[0], DNA[1], DNA[2], 0.0]) - Lanes([DNC[0], DNC[1], 0.0, DNC[2]]);
                    let DNF = ADB * DMY;
                    let DNG = DNF * DMT;
                    let DNH = if DNG >= AX { DNG } else { AX };
                    let DNI = ((((DMZ * ADB) * DMT) + (DMU * DNF)) * (if DNG >= AX { 1.0 } else { 0.0 })) * (AK / DNH);
                    let DNJ = DND - (DNH.ln());
                    let DNK = DNE - Lanes([DNI[0], DNI[1], DNI[2], 0.0]);
                    let DNL = DNJ + BAH;
                    let DNM = ((DNJ * DNL) + BAJ).sqrt();
                    let DNN = CX * ((DNJ - BAG) - DNM);
                    let DNO = (DNK - (((DNK * DNL) + (DNK * DNJ)) * (AK / (AJ * DNM)))) * CX;
                    let DNP = if DNN <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let DPZ;
                    let DQA;
                    if DNP != 0.0 {
                        let DNQ = if DNN < -1.1e2f64 { 1.0 } else { 0.0 };
                        let DQE;
                        let DQF;
                        if DNQ != 0.0 {
                            DQE = DQC;
                            DQF = DDS;
                        } else {
                            let DQD = if DNN > -9e1f64 { 1.0 } else { 0.0 };
                            let DRD;
                            let DRE;
                            if DQD != 0.0 {
                                let DQQ = rspice_limited_exp(DNN);
                                let DQR = DNO * (rspice_limited_exp_derivative(DNN));
                                DRD = DQQ;
                                DRE = DQR;
                            } else {
                                let DQT = (DNN - DQS) / BDI;
                                let DQU = DNO / BDI;
                                let DQV = DQT * DQT;
                                let DQW = DQU * DQT;
                                let DQX = DQW + DQW;
                                let DQY = BDO - DQV;
                                let DQZ = 9.375e-1f64 - (DQV * DQY);
                                let DRA = DQS + (BDI * ((7.8125e-2f64 + (CX * DQT)) + (DQV * DQZ)));
                                let DRB = rspice_limited_exp(DRA);
                                let DRC = (((DQU * CX) + ((DQX * DQZ) + ((((DQX * DQY) + ((DQX * W) * DQV)) * W) * DQV))) * BDI) * (rspice_limited_exp_derivative(DRA));
                                DRD = DRB;
                                DRE = DRC;
                            }
                            DQE = DRD;
                            DQF = DRE;
                        }
                        let DQG = AO * DMY;
                        let DQH = DQE * AO;
                        let DQI = DMZ * DQH;
                        let DQJ = (DQH * DMY) + DMV;
                        let DQK = DQG * DQJ;
                        let DQL = (DMZ * AO) * DQJ;
                        let DQM = if DQK >= AX { DQK } else { AX };
                        let DQN = ((U + DND) - DNN) - (DQM.ln());
                        let DQO = DQE * DQN;
                        let DQP = (DQF * DQN) + (((DNE - DNO) - (((Lanes([DQL[0], DQL[1], DQL[2], 0.0]) + (((((DQF * AO) * DMY) + Lanes([DQI[0], DQI[1], DQI[2], 0.0])) + Lanes([DMW[0], DMW[1], DMW[2], 0.0])) * DQG)) * (if DQK >= AX { 1.0 } else { 0.0 })) * (AK / DQM))) * DQE);
                        DPZ = DQO;
                        DQA = DQP;
                    } else {
                        let DNR = rspice_limited_exp(DNN);
                        let DNS = DNO * (rspice_limited_exp_derivative(DNN));
                        let DNT = U / DMT;
                        let DNU = AO * DNR;
                        let DNV = DNS * AO;
                        let DNW = DNU * DMY;
                        let DNX = DMZ * DNU;
                        let DNY = (DNV * DMY) + Lanes([DNX[0], DNX[1], DNX[2], 0.0]);
                        let DNZ = DNW + DMV;
                        let DOA = Lanes([DMW[0], DMW[1], DMW[2], 0.0]);
                        let DOB = DNW * DNZ;
                        let DOC = if DOB >= AX { DOB } else { AX };
                        let DOD = 1e0f64 / DNR;
                        let DOE = DMY + DNT;
                        let DOF = DMZ + (((DMU * DNT) * W) / DMT);
                        let DOG = DMZ * DNR;
                        let DOH = (DMY * DNR) + DMT;
                        let DOI = Lanes([DMU[0], DMU[1], DMU[2], 0.0]);
                        let DOJ = DOE / DOH;
                        let DOK = Lanes([DOF[0], DOF[1], DOF[2], 0.0]);
                        let DOL = (AO + DOD) + DOJ;
                        let DOM = ((DNU + (DOC.ln())) - DND) / DOL;
                        let DON = DNR - DOM;
                        let DOO = DNS - ((((DNV + ((((DNY * DNZ) + ((DNY + DOA) * DNW)) * (if DOB >= AX { 1.0 } else { 0.0 })) * (AK / DOC))) - DNE) - (((((DNS * DOD) * W) / DNR) + ((DOK - (((Lanes([DOG[0], DOG[1], DOG[2], 0.0]) + (DNS * DMY)) + DOI) * DOJ)) / DOH)) * DOM)) / DOL);
                        let DOP = AO * DON;
                        let DOQ = DOO * AO;
                        let DOR = DOP * DMY;
                        let DOS = DMZ * DOP;
                        let DOT = (DOQ * DMY) + Lanes([DOS[0], DOS[1], DOS[2], 0.0]);
                        let DOU = DOR + DMV;
                        let DOV = DOR * DOU;
                        let DOW = if DOV >= AX { DOV } else { AX };
                        let DOX = (DOP + (DOW.ln())) - DND;
                        let DOY = (DOQ + ((((DOT * DOU) + ((DOT + DOA) * DOR)) * (if DOV >= AX { 1.0 } else { 0.0 })) * (AK / DOW))) - DNE;
                        let DOZ = 1e0f64 / DON;
                        let DPA = DMZ * DON;
                        let DPB = (DMY * DON) + DMT;
                        let DPC = (Lanes([DPA[0], DPA[1], DPA[2], 0.0]) + (DOO * DMY)) + DOI;
                        let DPD = DOE / DPB;
                        let DPE = (DOK - (DPC * DPD)) / DPB;
                        let DPF = (AO + DOZ) + DPD;
                        let DPG = (((DOO * DOZ) * W) / DON) + DPE;
                        let DPH = DPE * DPD;
                        let DPI = U / DON;
                        let DPJ = (((DOO * DPI) * W) / DON) * DPI;
                        let DPL = DMT * DMT;
                        let DPM = DMU * DMT;
                        let DPN = DPL * DMT;
                        let DPO = DPN * DPB;
                        let DPP = (((DPM + DPM) * DMT) + (DMU * DPL)) * DPB;
                        let DPQ = 1e0f64 / DPO;
                        let DPR = ((DPK * (DPI * DPI)) - DPQ) - (DPD * DPD);
                        let DPS = DOX / DPF;
                        let DPT = AO * DPF;
                        let DPU = DPT * DPF;
                        let DPV = (DOX * DPR) / DPU;
                        let DPW = U + DPV;
                        let DPX = DON - (DPS * DPW);
                        let DPY = DOO - ((((DOY - (DPG * DPS)) / DPF) * DPW) + (((((DOY * DPR) + (((((DPJ + DPJ) * DPK) - ((((Lanes([DPP[0], DPP[1], DPP[2], 0.0]) + (DPC * DPN)) * DPQ) * W) / DPO)) - (DPH + DPH)) * DOX)) - ((((DPG * AO) * DPF) + (DPG * DPT)) * DPV)) / DPU) * DPS));
                        DPZ = DPX;
                        DQA = DPY;
                    }
                    let DQB = if 0.0f64 != 0.0 && (if DML < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DRL;
                    let DRM;
                    if DQB != 0.0 {
                        let DRF = ASC * DML;
                        let DRG = -4e0f64 / DRF;
                        let DRH = (((DMM * ASC) * DRG) * W) / DRF;
                        DRL = DRG;
                        DRM = DRH;
                    } else {
                        let DRI = (DMP + 1e0f64).sqrt();
                        let DRJ = CX * (DMN + DRI);
                        let DRK = (DMM + (DMR * (AK / (AJ * DRI)))) * CX;
                        DRL = DRJ;
                        DRM = DRK;
                    }
                    let DRN = DRL.sqrt();
                    let DRO = DRM * (AK / (AJ * DRN));
                    let DRP = DML - (AO * DPZ);
                    let DRQ = Lanes([DMM[0], DMM[1], DMM[2], 0.0]) - (DQA * AO);
                    let DRR = if 0.0f64 != 0.0 && (if DRP < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DSA;
                    let DSB;
                    if DRR != 0.0 {
                        let DRS = ASC * DRP;
                        let DRT = -4e0f64 / DRS;
                        let DRU = (((DRQ * ASC) * DRT) * W) / DRS;
                        DSA = DRT;
                        DSB = DRU;
                    } else {
                        let DRV = DRP - U;
                        let DRW = DRQ * DRV;
                        let DRX = ((DRV * DRV) + 1e0f64).sqrt();
                        let DRY = CX * ((DRP + U) + DRX);
                        let DRZ = (DRQ + ((DRW + DRW) * (AK / (AJ * DRX)))) * CX;
                        DSA = DRY;
                        DSB = DRZ;
                    }
                    let DSC = DSA.sqrt();
                    let DSD = DRN + DSC;
                    let DSE = DDC / DSD;
                    let DSF = (Lanes([DDD, 0.0, 0.0, 0.0]) - ((Lanes([DRO[0], DRO[1], DRO[2], 0.0]) + (DSB * (AK / (AJ * DSC)))) * DSE)) / DSD;
                    let DSG = U + DSE;
                    let DSH = AO * DSG;
                    let DSI = (DLH - DRP) - (DSH * DPZ);
                    let DSJ = DKH * DSI;
                    let DSK = Lanes([(DKI * DSI), 0.0, 0.0, 0.0]) + (((Lanes([DLI[0], DLI[1], DLI[2], 0.0]) - DRQ) - (((DSF * AO) * DPZ) + (DQA * DSH))) * DKH);
                    let DSR;
                    let DSS;
                    if DKN != 0.0 {
                        let DSL = U + (DPZ / DKO);
                        let DSM = staged[243] / DSL;
                        let DSN = staged[244] + (DSM / DKR);
                        let DSO = 3.4531302e-11f64 / DSN;
                        let DSP = (((((((DQA / DKO) * DSM) * W) / DSL) / DKR) * DSO) * W) / DSN;
                        DSR = DSO;
                        DSS = DSP;
                    } else {
                        DSR = DSQ;
                        DSS = DDS;
                    }
                    let DST = DKY * DSG;
                    let DSU = DST * S;
                    let DSV = DSU * DSR;
                    let DSW = DSV * DPZ;
                    let DSX = ((((((DSF * DKY) * S) + Lanes([(T * DST), 0.0, 0.0, 0.0])) * DSR) + (DSS * DSU)) * DPZ) + (DQA * DSV);
                    DLQ = DSW;
                    DLR = DSJ;
                    DLS = DSX;
                    DLT = DSK;
                } else {
                    DLQ = A;
                    DLR = A;
                    DLS = DDS;
                    DLT = DDS;
                }
                DDT = DLC;
                DDU = DKL;
                DDV = DLQ;
                DDW = DLR;
                DDX = DLD;
                DDY = DKM;
                DDZ = DLS;
                DEA = DLT;
            } else {
                DDT = A;
                DDU = A;
                DDV = A;
                DDW = A;
                DDX = DDR;
                DDY = DDR;
                DDZ = DDS;
                DEA = DDS;
            }
            let DTF;
            let DTG;
            if DEB != 0.0 {
                let DSY = ((CNL * BKA) / AWB) * ASZ;
                let DTA = DSZ * S;
                let DTC = DTB * (((((DTA * CNL) * BKA) / AWB) * ASZ) + (DSY * CBR));
                let DTD = (((((Lanes([((T * DSZ) * CNL), 0.0, 0.0, 0.0, 0.0]) + (CNM * DTA)) * BKA) / AWB) * ASZ) + (((((CNM * BKA) / AWB) * ASZ) * CBR) + (CBT * DSY))) * DTB;
                let DTI;
                let DTJ;
                if DTE != 0.0 {
                    let DTL = DTK + DTC;
                    let DTM = (DTK * DTC) / DTL;
                    let DTN = ((DTD * DTK) - (DTD * DTM)) / DTL;
                    DTI = DTM;
                    DTJ = DTN;
                } else {
                    DTI = DTC;
                    DTJ = DTD;
                }
                DTF = DTI;
                DTG = DTJ;
            } else {
                DTF = A;
                DTG = BCT;
            }
            let DTP;
            let DTQ;
            let DTR;
            let DTS;
            if DTH != 0.0 {
                let DTO = if (if ARJ <= A { 1.0 } else { 0.0 }) != 0.0 || (if ARK <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DTW;
                let DTX;
                if DTO != 0.0 {
                    DTW = A;
                    DTX = COT;
                } else {
                    let DTV = if CDI > (ARK / DTU) { 1.0 } else { 0.0 };
                    let DUP;
                    let DUQ;
                    if DTV != 0.0 {
                        let DTY = ARY * W;
                        let DTZ = (-ARK) / CDI;
                        let DUA = ARJ * CDI;
                        let DUB = ARX * CDI;
                        let DUC = DUA * COV;
                        let DUD = (Lanes([DUB[0], DUB[1], DUB[2], 0.0, DUB[3]]) + (CDJ * ARJ)) * COV;
                        let DUE = rspice_limited_exp(DTZ);
                        let DUF = (((Lanes([DTY[0], DTY[1], DTY[2], 0.0, DTY[3]]) - (CDJ * DTZ)) / CDI) * (rspice_limited_exp_derivative(DTZ))) * DUC;
                        let DUG = (DUC * DUE) / CHK;
                        let DUH = CHJ * DUG;
                        let DUI = ((((Lanes([0.0, DUD[0], DUD[1], 0.0, DUD[2], DUD[3], DUD[4]]) + (COZ * DUA)) * DUE) + Lanes([0.0, DUF[0], DUF[1], 0.0, DUF[2], DUF[3], DUF[4]])) - Lanes([0.0, DUH[0], DUH[1], 0.0, DUH[2], DUH[3], DUH[4]])) / CHK;
                        DUP = DUG;
                        DUQ = DUI;
                    } else {
                        let DUJ = ARJ * CDI;
                        let DUK = ARX * CDI;
                        let DUL = (Lanes([DUK[0], DUK[1], DUK[2], 0.0, DUK[3]]) + (CDJ * ARJ)) * COV;
                        let DUM = ((DUJ * COV) * AVU) / CHK;
                        let DUN = CHJ * DUM;
                        let DUO = (((Lanes([0.0, DUL[0], DUL[1], 0.0, DUL[2], DUL[3], DUL[4]]) + (COZ * DUJ)) * AVU) - Lanes([0.0, DUN[0], DUN[1], 0.0, DUN[2], DUN[3], DUN[4]])) / CHK;
                        DUP = DUM;
                        DUQ = DUO;
                    }
                    DTW = DUP;
                    DTX = DUQ;
                }
                DTP = A;
                DTQ = DTW;
                DTR = BCT;
                DTS = DTX;
            } else {
                let DVG;
                let DVH;
                let DVI;
                let DVJ;
                if IF != 0.0 {
                    let DUS = U + (DUR * ANE);
                    let DUT = DUS * BUP;
                    let DUU = (ANF * DUR) * BUP;
                    let DUV = ANE / DUT;
                    let DUW = DUV + BB;
                    let DUX = DUW.powf(BUU);
                    let DUY = U + DUX;
                    let DUZ = DUY.powf(BUZ);
                    let DVA = ANE * DUZ;
                    let DVB = ANF * DUZ;
                    let DVC = Lanes([0.0, DVB[0], DVB[1], 0.0, DVB[2]]) + (((((((BUS - ((Lanes([0.0, DUU[0], DUU[1], 0.0, DUU[2]]) + (BUQ * DUS)) * DUV)) / DUT) * (BUU * (DUW.powf(BUX)))) + Lanes([(BUV * (DUX * (DUW.ln()))), 0.0, 0.0, 0.0, 0.0])) * (BUZ * (DUY.powf(BVC)))) + Lanes([(BVA * (DUZ * (DUY.ln()))), 0.0, 0.0, 0.0, 0.0])) * ANE);
                    let DVD = ANE - DVA;
                    let DVE = BUS - DVC;
                    let DVF = if 1.0f64 != 0.0 && (if DVD < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DVR;
                    let DVS;
                    if DVF != 0.0 {
                        let DVK = ASC * DVD;
                        let DVL = -1e-6f64 / DVK;
                        let DVM = (((DVE * ASC) * DVL) * W) / DVK;
                        DVR = DVL;
                        DVS = DVM;
                    } else {
                        let DVN = DVE * DVD;
                        let DVO = ((DVD * DVD) + 2.5e-7f64).sqrt();
                        let DVP = CX * (DVD + DVO);
                        let DVQ = (DVE + ((DVN + DVN) * (AK / (AJ * DVO)))) * CX;
                        DVR = DVP;
                        DVS = DVQ;
                    }
                    let DVT = CX * ARK;
                    let DVV = U + (DVA.powf(DVU));
                    let DVW = DVT * DVV;
                    let DVX = (ARY * CX) * DVV;
                    let DVY = Lanes([DVX[0], DVX[1], DVX[2], 0.0, DVX[3]]) + ((DVC * (DVU * (DVA.powf(staged[438])))) * DVT);
                    let DWA = DVZ * AOI;
                    let DWC = U + (DWB * (rspice_limited_exp(DWA)));
                    let DWD = ARJ / DWC;
                    let DWE = (((AOH * DVZ) * (rspice_limited_exp_derivative(DWA))) * DWB) * DWD;
                    let DWH = DWG * AOJ;
                    let DWI = (U + (DWF * AOJ)) + (DWH * AOJ);
                    let DWJ = DWD * DWI;
                    let DWK = ((AOK * DWF) + (((AOK * DWG) * AOJ) + (AOK * DWH))) * DWD;
                    let DWL = (((ARX - Lanes([0.0, DWE[0], DWE[1], DWE[2]])) / DWC) * DWI) + Lanes([0.0, DWK[0], DWK[1], DWK[2]]);
                    let DWM = if 1.0f64 != 0.0 && (if DWJ < -2.5e-9f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DWU;
                    let DWV;
                    if DWM != 0.0 {
                        let DWN = ASC * DWJ;
                        let DWO = -1e-24f64 / DWN;
                        let DWP = (((DWL * ASC) * DWO) * W) / DWN;
                        DWU = DWO;
                        DWV = DWP;
                    } else {
                        let DWQ = DWL * DWJ;
                        let DWR = ((DWJ * DWJ) + 2.5e-25f64).sqrt();
                        let DWS = CX * (DWJ + DWR);
                        let DWT = (DWL + ((DWQ + DWQ) * (AK / (AJ * DWR)))) * CX;
                        DWU = DWS;
                        DWV = DWT;
                    }
                    let DWW = if (if ARJ <= A { 1.0 } else { 0.0 }) != 0.0 || (if ARK <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DWY;
                    let DWZ;
                    if DWW != 0.0 {
                        DWY = A;
                        DWZ = COT;
                    } else {
                        let DWX = if DVR > (DVW / DTU) { 1.0 } else { 0.0 };
                        let DXS;
                        let DXT;
                        if DWX != 0.0 {
                            let DXB = DVR.powf(DXA);
                            let DXC = (-DVW) / DXB;
                            let DXD = DWU * DVR;
                            let DXE = DWV * DVR;
                            let DXF = DXD * COV;
                            let DXG = (Lanes([DXE[0], DXE[1], DXE[2], 0.0, DXE[3]]) + (DVS * DWU)) * COV;
                            let DXH = rspice_limited_exp(DXC);
                            let DXI = ((((DVY * W) - ((DVS * (DXA * (DVR.powf((DXA - AK))))) * DXC)) / DXB) * (rspice_limited_exp_derivative(DXC))) * DXF;
                            let DXJ = (DXF * DXH) / CHK;
                            let DXK = CHJ * DXJ;
                            let DXL = ((((Lanes([0.0, DXG[0], DXG[1], 0.0, DXG[2], DXG[3], DXG[4]]) + (COZ * DXD)) * DXH) + Lanes([0.0, DXI[0], DXI[1], 0.0, DXI[2], DXI[3], DXI[4]])) - Lanes([0.0, DXK[0], DXK[1], 0.0, DXK[2], DXK[3], DXK[4]])) / CHK;
                            DXS = DXJ;
                            DXT = DXL;
                        } else {
                            let DXM = DWU * DVR;
                            let DXN = DWV * DVR;
                            let DXO = (Lanes([DXN[0], DXN[1], DXN[2], 0.0, DXN[3]]) + (DVS * DWU)) * COV;
                            let DXP = ((DXM * COV) * AVU) / CHK;
                            let DXQ = CHJ * DXP;
                            let DXR = (((Lanes([0.0, DXO[0], DXO[1], 0.0, DXO[2], DXO[3], DXO[4]]) + (COZ * DXM)) * AVU) - Lanes([0.0, DXQ[0], DXQ[1], 0.0, DXQ[2], DXQ[3], DXQ[4]])) / CHK;
                            DXS = DXP;
                            DXT = DXR;
                        }
                        DWY = DXS;
                        DWZ = DXT;
                    }
                    DVG = DVA;
                    DVH = DWY;
                    DVI = DVC;
                    DVJ = DWZ;
                } else {
                    DVG = A;
                    DVH = A;
                    DVI = BCT;
                    DVJ = COT;
                }
                DTP = DVG;
                DTQ = DVH;
                DTR = DVI;
                DTS = DVJ;
            }
            let DXX;
            let DXY;
            if DTT != 0.0 {
                let DXU = BCP - parameters[1105];
                let DXV = if 0.0f64 != 0.0 && (if DXU < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DYL;
                let DYM;
                if DXV != 0.0 {
                    let DYD = ASC * DXU;
                    let DYE = -4e0f64 / DYD;
                    let DYF = (((BCQ * ASC) * DYE) * W) / DYD;
                    DYL = DYE;
                    DYM = DYF;
                } else {
                    let DYG = DXU - BHW;
                    let DYH = BCQ * DYG;
                    let DYI = ((DYG * DYG) + 1e0f64).sqrt();
                    let DYJ = CX * ((DXU + BHW) + DYI);
                    let DYK = (BCQ + ((DYH + DYH) * (AK / (AJ * DYI)))) * CX;
                    DYL = DYJ;
                    DYM = DYK;
                }
                let DYO = DYN + DYL;
                let DYP = (DYN * DYL) / DYO;
                let DYR = U + (DYQ * DYP);
                let DYU = DYT * (IN * DYR);
                let DYV = (DYS * COV) / DYU;
                let DYW = ((Lanes([(IP * DYR), 0.0, 0.0, 0.0, 0.0]) + (((((DYM * DYN) - (DYM * DYP)) / DYO) * DYQ) * IN)) * DYT) * DYV;
                let DYX = (((COZ * DYS) - Lanes([0.0, DYW[0], DYW[1], 0.0, DYW[2], DYW[3], DYW[4]])) / DYU) / CSC;
                let DYY = (DYV / CSC) - U;
                let DYZ = if 1.0f64 != 0.0 && (if DYY < staged[251] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DZJ;
                let DZK;
                if DYZ != 0.0 {
                    let DZB = ASC * DYY;
                    let DZC = ((-DZA) * DZA) / DZB;
                    let DZD = (((DYX * ASC) * DZC) * W) / DZB;
                    DZJ = DZC;
                    DZK = DZD;
                } else {
                    let DZE = DYX * DYY;
                    let DZG = ((DYY * DYY) + ((DZF * DZA) * DZA)).sqrt();
                    let DZH = CX * (DYY + DZG);
                    let DZI = (DYX + ((DZE + DZE) * (AK / (AJ * DZG)))) * CX;
                    DZJ = DZH;
                    DZK = DZI;
                }
                let DZL = CSC * DZJ;
                let DZM = DZK * CSC;
                let EAH;
                let EAI;
                if DZN != 0.0 {
                    let DZS = (Lanes([DZQ, 0.0]) - Lanes([0.0, DZR])) * AJU;
                    let DZU = DTR * DZT;
                    let DZV = Lanes([DZS[0], DZS[1], 0.0, 0.0, 0.0, 0.0, 0.0]) - Lanes([0.0, 0.0, DZU[0], DZU[1], DZU[2], DZU[3], DZU[4]]);
                    let DZY = (CPA * (DZX * (COW.powf(staged[439])))) * CSI;
                    let DZZ = (((AJU * (DZO - DZP)) - (DZT * DTP)) - DZW) - (CSI * (COW.powf(DZX)));
                    let EAA = Lanes([DZV[0], DZV[1], 0.0, DZV[2], DZV[3], DZV[4], DZV[5], DZV[6]]) - Lanes([0.0, 0.0, DZY[0], 0.0, 0.0, 0.0, 0.0, DZY[1]]);
                    let EAB = if 1.0f64 != 0.0 && (if DZZ < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EBI;
                    let EBJ;
                    if EAB != 0.0 {
                        let EBB = ASC * DZZ;
                        let EBC = -2.5000000000000005e-3f64 / EBB;
                        let EBD = (((EAA * ASC) * EBC) * W) / EBB;
                        EBI = EBC;
                        EBJ = EBD;
                    } else {
                        let EBE = EAA * DZZ;
                        let EBF = ((DZZ * DZZ) + 6.250000000000001e-4f64).sqrt();
                        let EBG = CX * (DZZ + EBF);
                        let EBH = (EAA + ((EBE + EBE) * (AK / (AJ * EBF)))) * CX;
                        EBI = EBG;
                        EBJ = EBH;
                    }
                    EAH = EBI;
                    EAI = EBJ;
                } else {
                    let EAC = (Lanes([DZQ, 0.0]) - Lanes([0.0, DZR])) * AJU;
                    let EAD = DTR * DZT;
                    let EAE = Lanes([EAC[0], EAC[1], 0.0, 0.0, 0.0, 0.0, 0.0]) - Lanes([0.0, 0.0, EAD[0], EAD[1], EAD[2], EAD[3], EAD[4]]);
                    let EAF = ((AJU * (DZO - DZP)) - (DZT * DTP)) - DZW;
                    let EAG = if 1.0f64 != 0.0 && (if EAF < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EBR;
                    let EBS;
                    if EAG != 0.0 {
                        let EBK = ASC * EAF;
                        let EBL = -2.5000000000000005e-3f64 / EBK;
                        let EBM = (((EAE * ASC) * EBL) * W) / EBK;
                        EBR = EBL;
                        EBS = EBM;
                    } else {
                        let EBN = EAE * EAF;
                        let EBO = ((EAF * EAF) + 6.250000000000001e-4f64).sqrt();
                        let EBP = CX * (EAF + EBO);
                        let EBQ = (EAE + ((EBN + EBN) * (AK / (AJ * EBO)))) * CX;
                        EBR = EBP;
                        EBS = EBQ;
                    }
                    let EBT = Lanes([EBS[0], EBS[1], 0.0, EBS[2], EBS[3], EBS[4], EBS[5], EBS[6]]);
                    EAH = EBR;
                    EAI = EBT;
                }
                let EAK = EAJ * DZL;
                let EAL = EAK * EAH;
                let EAM = (DZM * EAJ) * EAH;
                let EAN = EAI * EAK;
                let EAO = EAL.sqrt();
                let EAP = (Lanes([0.0, 0.0, EAM[0], EAM[1], EAM[2], EAM[3], EAM[4], EAM[5], EAM[6]]) + Lanes([EAN[0], EAN[1], EAN[2], EAN[3], EAN[4], 0.0, EAN[5], EAN[6], EAN[7]])) * (CX * (EAL.powf(-5e-1f64)));
                let EAS = EAR * AOJ;
                let EAT = (AOK * EAQ) + (((AOK * EAR) * AOJ) + (AOK * EAS));
                let EAY = EAX * ((U + ((EAQ * AOJ) + (EAS * AOJ))) + ((EAU * EAH) + (EAW * (EAH.powf(EAV)))));
                let EAZ = (Lanes([0.0, 0.0, 0.0, 0.0, EAT[0], EAT[1], 0.0, EAT[2]]) + ((EAI * EAU) + ((EAI * (EAV * (EAH.powf(staged[440])))) * EAW))) * EAX;
                let EBA = if 1.0f64 != 0.0 && (if EAY < -2.5e-9f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ECB;
                let ECC;
                if EBA != 0.0 {
                    let EBU = ASC * EAY;
                    let EBV = -1e-24f64 / EBU;
                    let EBW = (((EAZ * ASC) * EBV) * W) / EBU;
                    ECB = EBV;
                    ECC = EBW;
                } else {
                    let EBX = EAZ * EAY;
                    let EBY = ((EAY * EAY) + 2.5e-25f64).sqrt();
                    let EBZ = CX * (EAY + EBY);
                    let ECA = (EAZ + ((EBX + EBX) * (AK / (AJ * EBY)))) * CX;
                    ECB = EBZ;
                    ECC = ECA;
                }
                let ECD = if EAO > staged[253] { 1.0 } else { 0.0 };
                let ECR;
                let ECS;
                if ECD != 0.0 {
                    let ECE = (-parameters[501]) / EAO;
                    let ECF = ECB * EAO;
                    let ECG = ECC * EAO;
                    let ECH = ECF * COV;
                    let ECI = COZ * ECF;
                    let ECJ = rspice_limited_exp(ECE);
                    let ECK = ECH * ECJ;
                    let ECL = ((((Lanes([ECG[0], ECG[1], ECG[2], ECG[3], ECG[4], 0.0, ECG[5], ECG[6], ECG[7]]) + (EAP * ECB)) * COV) + Lanes([0.0, 0.0, ECI[0], ECI[1], ECI[2], ECI[3], ECI[4], ECI[5], ECI[6]])) * ECJ) + (((((EAP * ECE) * W) / EAO) * (rspice_limited_exp_derivative(ECE))) * ECH);
                    ECR = ECK;
                    ECS = ECL;
                } else {
                    let ECM = ECB * EAO;
                    let ECN = ECC * EAO;
                    let ECO = COZ * ECM;
                    let ECP = (ECM * COV) * AVU;
                    let ECQ = (((Lanes([ECN[0], ECN[1], ECN[2], ECN[3], ECN[4], 0.0, ECN[5], ECN[6], ECN[7]]) + (EAP * ECB)) * COV) + Lanes([0.0, 0.0, ECO[0], ECO[1], ECO[2], ECO[3], ECO[4], ECO[5], ECO[6]])) * AVU;
                    ECR = ECP;
                    ECS = ECQ;
                }
                DXX = ECR;
                DXY = ECS;
            } else {
                DXX = A;
                DXY = DXW;
            }
            let DYA = (DTQ * DXZ) * AJU;
            let DYB = (DTS * DXZ) * AJU;
            let EDH;
            let EDI;
            let EDJ;
            let EDK;
            let EDL;
            let EDM;
            let EDN;
            let EDO;
            let EDP;
            let EDQ;
            if DYC != 0.0 {
                let ECT = (BFA + BCP) + BYE;
                let ECU = ATN * ECT;
                let ECV = ATO * ECT;
                let ECW = Lanes([ECV[0], ECV[1], ECV[2], 0.0, ECV[3]]) + (((BFB + BCQ) + BYF) * ATN);
                let ECX = ECW * ECU;
                let ECZ = ((ECU * ECU) + ECY).sqrt();
                let EDA = (ECX + ECX) * (AK / (AJ * ECZ));
                let EDB = CX * ((-ECU) + ECZ);
                let EDC = ((ECW * W) + EDA) * CX;
                let EDD = CX * (ECU + ECZ);
                let EDE = (ECW + EDA) * CX;
                let EEI;
                let EEJ;
                if E != 0.0 {
                    let EEE = (ECU / EED) / S;
                    let EEF = -EEE;
                    let EEG = (((ECW / EED) - Lanes([(T * EEE), 0.0, 0.0, 0.0, 0.0])) / S) * W;
                    let EEH = if EEF > ANL { 1.0 } else { 0.0 };
                    let EEL;
                    let EEM;
                    if EEH != 0.0 {
                        EEL = EEF;
                        EEM = EEG;
                    } else {
                        let EEK = if EEF < -3.7e1f64 { 1.0 } else { 0.0 };
                        let EFN;
                        let EFO;
                        if EEK != 0.0 {
                            let EFH = EEF.exp();
                            let EFI = EEG * EFH;
                            EFN = EFH;
                            EFO = EFI;
                        } else {
                            let EFJ = EEF.exp();
                            let EFK = U + EFJ;
                            let EFL = EFK.ln();
                            let EFM = (EEG * EFJ) * (AK / EFK);
                            EFN = EFL;
                            EFO = EFM;
                        }
                        EEL = EFN;
                        EEM = EFO;
                    }
                    let EEN = EED * S;
                    let EEO = EEN * EEL;
                    let EER = U + (EEQ * EDB);
                    let EET = EES * (staged[256] - (EEP * EDB));
                    let EEU = EET * EER;
                    let EEV = rspice_limited_exp(EEU);
                    let EEX = EEW * AJV;
                    let EEY = EEX * EEO;
                    let EEZ = (AJW * EEW) * EEO;
                    let EFA = EEY * EEV;
                    let EFB = EFA * NT;
                    let EFC = ((((Lanes([0.0, 0.0, 0.0, EEZ[0], EEZ[1]]) + ((Lanes([((T * EED) * EEL), 0.0, 0.0, 0.0, 0.0]) + (EEM * EEN)) * EEX)) * EEV) + (((((((EDC * EEP) * W) * EES) * EER) + ((EDC * EEQ) * EET)) * (rspice_limited_exp_derivative(EEU))) * EEY)) * NT) + Lanes([(NU * EFA), 0.0, 0.0, 0.0, 0.0]);
                    let EFE = ((ECU - staged[260]) / EFD) / S;
                    let EFF = ((ECW / EFD) - Lanes([(T * EFE), 0.0, 0.0, 0.0, 0.0])) / S;
                    let EFG = if EFE > ANL { 1.0 } else { 0.0 };
                    let EFQ;
                    let EFR;
                    if EFG != 0.0 {
                        EFQ = EFE;
                        EFR = EFF;
                    } else {
                        let EFP = if EFE < -3.7e1f64 { 1.0 } else { 0.0 };
                        let EGO;
                        let EGP;
                        if EFP != 0.0 {
                            let EGI = EFE.exp();
                            let EGJ = EFF * EGI;
                            EGO = EGI;
                            EGP = EGJ;
                        } else {
                            let EGK = EFE.exp();
                            let EGL = U + EGK;
                            let EGM = EGL.ln();
                            let EGN = (EFF * EGK) * (AK / EGL);
                            EGO = EGM;
                            EGP = EGN;
                        }
                        EFQ = EGO;
                        EFR = EGP;
                    }
                    let EFS = EFD * S;
                    let EFT = EFS * EFQ;
                    let EFW = U + (EFV * EDD);
                    let EFY = EFX * (staged[263] - (EFU * EDD));
                    let EFZ = EFY * EFW;
                    let EGA = rspice_limited_exp(EFZ);
                    let EGC = EGB * AJV;
                    let EGD = EGC * EFT;
                    let EGE = (AJW * EGB) * EFT;
                    let EGF = EGD * EGA;
                    let EGG = EFB + (EGF * NT);
                    let EGH = EFC + (((((Lanes([0.0, 0.0, 0.0, EGE[0], EGE[1]]) + ((Lanes([((T * EFD) * EFQ), 0.0, 0.0, 0.0, 0.0]) + (EFR * EFS)) * EGC)) * EGA) + (((((((EDE * EFU) * W) * EFX) * EFW) + ((EDE * EFV) * EFY)) * (rspice_limited_exp_derivative(EFZ))) * EGD)) * NT) + Lanes([(NU * EGF), 0.0, 0.0, 0.0, 0.0]));
                    EEI = EGG;
                    EEJ = EGH;
                } else {
                    EEI = A;
                    EEJ = BCT;
                }
                let EIA;
                let EIB;
                let EIC;
                let EID;
                let EIE;
                let EIF;
                let EIG;
                let EIH;
                if F != 0.0 {
                    let EGS = U + (EGR * EDD);
                    let EGU = EGT * (staged[268] - (EGQ * EDD));
                    let EGV = EGU * EGS;
                    let EGW = BZY * ATN;
                    let EGX = ATO * BZY;
                    let EGY = EGW * CAL;
                    let EGZ = rspice_limited_exp(EGV);
                    let EHB = EHA * (EGY * EGZ);
                    let EHC = AOH * CX;
                    let EHD = (ANB + ANA) * CX;
                    let EHE = (AJV + (CX * AOI)) - (CX * (AMW + AMV));
                    let EHF = EHB * EHE;
                    let EHG = ((Lanes([0.0, 0.0, AJW[0], AJW[1]]) + Lanes([EHC[0], EHC[1], 0.0, EHC[2]])) - Lanes([EHD[0], EHD[1], 0.0, EHD[2]])) * EHB;
                    let EHH = EHF * NT;
                    let EHI = ((((((((((BZX * ATN) + Lanes([EGX[0], EGX[1], EGX[2], 0.0, EGX[3]])) * CAL) + (CAF * EGW)) * EGZ) + (((((((EDE * EGQ) * W) * EGT) * EGS) + ((EDE * EGR) * EGU)) * (rspice_limited_exp_derivative(EGV))) * EGY)) * EHA) * EHE) + Lanes([0.0, EHG[0], EHG[1], EHG[2], EHG[3]])) * NT) + Lanes([(NU * EHF), 0.0, 0.0, 0.0, 0.0]);
                    let EHJ = BVF * BVD;
                    let EHK = ((BVD * BVD) + TY).sqrt();
                    let EHM = EHL * (EHK - BHW);
                    let EHN = ((EHJ + EHJ) * (AK / (AJ * EHK))) * EHL;
                    let EHO = -EHM;
                    let EHP = rspice_limited_exp(EHO);
                    let EHQ = (EHN * W) * (rspice_limited_exp_derivative(EHO));
                    let EHR = EHN + EHQ;
                    let EHS = ((EHM + EHP) - U) + ECY;
                    let EHT = EHM + U;
                    let EHU = ((EHN * EHP) + (EHQ * EHT)) * W;
                    let EHV = (U - (EHT * EHP)) + ECY;
                    let EHW = EHN * EHM;
                    let EHX = EHW + EHW;
                    let EHY = (EHM * EHM) + 2e-4f64;
                    let EHZ = if AMZ > A { 1.0 } else { 0.0 };
                    let EIQ;
                    let EIR;
                    let EIS;
                    let EIT;
                    if EHZ != 0.0 {
                        let EII = (EHH * EHV) / EHY;
                        let EIJ = (((EHI * EHV) + (EHU * EHH)) - (EHX * EII)) / EHY;
                        let EIK = (EHH * EHS) / EHY;
                        let EIL = (((EHI * EHS) + (EHR * EHH)) - (EHX * EIK)) / EHY;
                        EIQ = EIK;
                        EIR = EII;
                        EIS = EIL;
                        EIT = EIJ;
                    } else {
                        let EIM = (EHH * EHV) / EHY;
                        let EIN = (((EHI * EHV) + (EHU * EHH)) - (EHX * EIM)) / EHY;
                        let EIO = (EHH * EHS) / EHY;
                        let EIP = (((EHI * EHS) + (EHR * EHH)) - (EHX * EIO)) / EHY;
                        EIQ = EIM;
                        EIR = EIO;
                        EIS = EIN;
                        EIT = EIP;
                    }
                    let EIU = ALJ - CF;
                    let EIV = (Lanes([0.0, ALK[0], ALK[1], ALK[2]]) - Lanes([CG, 0.0, 0.0, 0.0])) * EIU;
                    let EIW = ((EIU * EIU) + ECY).sqrt();
                    let EIX = (EIV + EIV) * (AK / (AJ * EIW));
                    let EJG;
                    let EJH;
                    if EIY != 0.0 {
                        let EJB = EJA - (EIZ * EIW);
                        let EJC = (EIX * EIZ) * W;
                        let EJD = if EJB < -1e-2f64 { 1.0 } else { 0.0 };
                        let EKI;
                        let EKJ;
                        if EJD != 0.0 {
                            let EKC = -1e-12f64 / EJB;
                            let EKD = ((EJC * EKC) * W) / EJB;
                            EKI = EKC;
                            EKJ = EKD;
                        } else {
                            let EKE = EJC * EJB;
                            let EKF = ((EJB * EJB) + 4e-12f64).sqrt();
                            let EKG = CX * (EJB + EKF);
                            let EKH = (EJC + ((EKE + EKE) * (AK / (AJ * EKF)))) * CX;
                            EKI = EKG;
                            EKJ = EKH;
                        }
                        EJG = EKI;
                        EJH = EKJ;
                    } else {
                        let EJE = EJA - (EIZ * EIW);
                        let EJF = (EIX * EIZ) * W;
                        EJG = EJE;
                        EJH = EJF;
                    }
                    let EJJ = U + (EJI * EIW);
                    let EJL = EJK * EJG;
                    let EJM = EJL * EJJ;
                    let EJN = rspice_limited_exp(EJM);
                    let EJP = (NT * BIR) * EJO;
                    let EJQ = (NU * BIR) * EJO;
                    let EJS = EJP * EJR;
                    let EJT = EJS * ALJ;
                    let EJU = ALK * EJS;
                    let EJV = EJT * EIW;
                    let EJW = EJV * EJN;
                    let EJX = ((((Lanes([((EJQ * EJR) * ALJ), 0.0, 0.0, 0.0]) + Lanes([0.0, EJU[0], EJU[1], EJU[2]])) * EIW) + (EIX * EJT)) * EJN) + (((((EJH * EJK) * EJJ) + ((EIX * EJI) * EJL)) * (rspice_limited_exp_derivative(EJM))) * EJV);
                    let EJY = ALH - CF;
                    let EJZ = (Lanes([0.0, ALI[0], ALI[1], ALI[2]]) - Lanes([CG, 0.0, 0.0, 0.0])) * EJY;
                    let EKA = ((EJY * EJY) + ECY).sqrt();
                    let EKB = (EJZ + EJZ) * (AK / (AJ * EKA));
                    let EKR;
                    let EKS;
                    if EIY != 0.0 {
                        let EKM = EKL - (EKK * EKA);
                        let EKN = (EKB * EKK) * W;
                        let EKO = if EKM < -1e-2f64 { 1.0 } else { 0.0 };
                        let ELL;
                        let ELM;
                        if EKO != 0.0 {
                            let ELF = -1e-12f64 / EKM;
                            let ELG = ((EKN * ELF) * W) / EKM;
                            ELL = ELF;
                            ELM = ELG;
                        } else {
                            let ELH = EKN * EKM;
                            let ELI = ((EKM * EKM) + 4e-12f64).sqrt();
                            let ELJ = CX * (EKM + ELI);
                            let ELK = (EKN + ((ELH + ELH) * (AK / (AJ * ELI)))) * CX;
                            ELL = ELJ;
                            ELM = ELK;
                        }
                        EKR = ELL;
                        EKS = ELM;
                    } else {
                        let EKP = EKL - (EKK * EKA);
                        let EKQ = (EKB * EKK) * W;
                        EKR = EKP;
                        EKS = EKQ;
                    }
                    let EKU = U + (EKT * EKA);
                    let EKV = EJK * EKR;
                    let EKW = EKV * EKU;
                    let EKX = rspice_limited_exp(EKW);
                    let EKZ = EJP * EKY;
                    let ELA = EKZ * ALH;
                    let ELB = ALI * EKZ;
                    let ELC = ELA * EKA;
                    let ELD = ELC * EKX;
                    let ELE = ((((Lanes([((EJQ * EKY) * ALH), 0.0, 0.0, 0.0]) + Lanes([0.0, ELB[0], ELB[1], ELB[2]])) * EKA) + (EKB * ELA)) * EKX) + (((((EKS * EJK) * EKU) + ((EKB * EKT) * EKV)) * (rspice_limited_exp_derivative(EKW))) * ELC);
                    EIA = EJW;
                    EIB = ELD;
                    EIC = EIQ;
                    EID = EIR;
                    EIE = EJX;
                    EIF = ELE;
                    EIG = EIS;
                    EIH = EIT;
                } else {
                    EIA = A;
                    EIB = A;
                    EIC = A;
                    EID = A;
                    EIE = EDF;
                    EIF = EDG;
                    EIG = BCT;
                    EIH = BCT;
                }
                EDH = EIA;
                EDI = EIB;
                EDJ = EEI;
                EDK = EIC;
                EDL = EID;
                EDM = EIE;
                EDN = EIF;
                EDO = EEJ;
                EDP = EIG;
                EDQ = EIH;
            } else {
                EDH = A;
                EDI = A;
                EDJ = A;
                EDK = A;
                EDL = A;
                EDM = EDF;
                EDN = EDG;
                EDO = BCT;
                EDP = BCT;
                EDQ = BCT;
            }
            let EDS = EDR * EDH;
            let EDT = EDM * EDR;
            let EDU = EDR * EDI;
            let EDV = EDN * EDR;
            let EDW = EDR * EDJ;
            let EDX = EDO * EDR;
            let EDY = EDR * EDK;
            let EDZ = EDP * EDR;
            let EEA = EDR * EDL;
            let EEB = EDQ * EDR;
            let ELO;
            let ELP;
            let ELQ;
            let ELR;
            if EEC != 0.0 {
                let ELN = if (if staged[283] != 0.0 || (if NA <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || staged[284] != 0.0 { 1.0 } else { 0.0 };
                let EMV;
                let EMW;
                if ELN != 0.0 {
                    EMV = A;
                    EMW = EDG;
                } else {
                    let EMQ = ALI * W;
                    let EMS = (((-ALH) - staged[285]) + CF) / EMR;
                    let EMT = (Lanes([0.0, EMQ[0], EMQ[1], EMQ[2]]) + Lanes([CG, 0.0, 0.0, 0.0])) / EMR;
                    let EMU = if EMS < -1e2f64 { 1.0 } else { 0.0 };
                    let ENE;
                    let ENF;
                    if EMU != 0.0 {
                        let EMY = -1e-4f64 / EMS;
                        let EMZ = ((EMT * EMY) * W) / EMS;
                        ENE = EMY;
                        ENF = EMZ;
                    } else {
                        let ENA = EMT * EMS;
                        let ENB = ((EMS * EMS) + 4e-4f64).sqrt();
                        let ENC = CX * (EMS + ENB);
                        let END = (EMT + ((ENA + ENA) * (AK / (AJ * ENB)))) * CX;
                        ENE = ENC;
                        ENF = END;
                    }
                    let ENG = ENE + CMM;
                    let ENH = NA / ENG;
                    let ENI = (Lanes([NB, 0.0, 0.0, 0.0]) - (ENF * ENH)) / ENG;
                    let ENK = if ENJ != A { 1.0 } else { 0.0 };
                    let ENU;
                    let ENV;
                    if ENK != 0.0 {
                        let ENL = AKB * AKB;
                        let ENM = AKC * AKB;
                        let ENN = ENL * AKB;
                        let ENO = ((ENM + ENM) * AKB) + (AKC * ENL);
                        let ENP = (ENJ + (ENN.abs())) + ECY;
                        let ENQ = ENN / ENP;
                        let ENR = (ENO - ((ENO * ((AJ * (if ENN >= CQU { 1.0 } else { 0.0 })) - AK)) * ENQ)) / ENP;
                        let ENS = if ENQ < -1e-2f64 { 1.0 } else { 0.0 };
                        let EOK;
                        let EOL;
                        if ENS != 0.0 {
                            let EOE = -1e-12f64 / ENQ;
                            let EOF = ((ENR * EOE) * W) / ENQ;
                            EOK = EOE;
                            EOL = EOF;
                        } else {
                            let EOG = ENR * ENQ;
                            let EOH = ((ENQ * ENQ) + 4e-12f64).sqrt();
                            let EOI = CX * (ENQ + EOH);
                            let EOJ = (ENR + ((EOG + EOG) * (AK / (AJ * EOH)))) * CX;
                            EOK = EOI;
                            EOL = EOJ;
                        }
                        let EOM = EOK - BB;
                        ENU = EOM;
                        ENV = EOL;
                    } else {
                        ENU = U;
                        ENV = ENT;
                    }
                    let ENW = staged[288] * BKA;
                    let ENX = ENW * ENE;
                    let ENY = -ENH;
                    let ENZ = rspice_limited_exp(ENY);
                    let EOA = ENX * ENZ;
                    let EOB = EOA * ENU;
                    let EOC = ENV * EOA;
                    let EOD = ((((ENF * ENW) * ENZ) + (((ENI * W) * (rspice_limited_exp_derivative(ENY))) * ENX)) * ENU) + Lanes([0.0, EOC[0], 0.0, EOC[1]]);
                    EMV = EOB;
                    EMW = EOD;
                }
                let EMX = if (if staged[289] != 0.0 || (if NM <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || staged[290] != 0.0 { 1.0 } else { 0.0 };
                let EOR;
                let EOS;
                if EMX != 0.0 {
                    EOR = A;
                    EOS = EDF;
                } else {
                    let EON = ALK * W;
                    let EOO = (((-ALJ) - staged[291]) + CF) / EMR;
                    let EOP = (Lanes([0.0, EON[0], EON[1], EON[2]]) + Lanes([CG, 0.0, 0.0, 0.0])) / EMR;
                    let EOQ = if EOO < -1e2f64 { 1.0 } else { 0.0 };
                    let EOZ;
                    let EPA;
                    if EOQ != 0.0 {
                        let EOT = -1e-4f64 / EOO;
                        let EOU = ((EOP * EOT) * W) / EOO;
                        EOZ = EOT;
                        EPA = EOU;
                    } else {
                        let EOV = EOP * EOO;
                        let EOW = ((EOO * EOO) + 4e-4f64).sqrt();
                        let EOX = CX * (EOO + EOW);
                        let EOY = (EOP + ((EOV + EOV) * (AK / (AJ * EOW)))) * CX;
                        EOZ = EOX;
                        EPA = EOY;
                    }
                    let EPB = EOZ + CMM;
                    let EPC = NM / EPB;
                    let EPD = (Lanes([NN, 0.0, 0.0, 0.0]) - (EPA * EPC)) / EPB;
                    let EPF = if EPE != A { 1.0 } else { 0.0 };
                    let EPP;
                    let EPQ;
                    if EPF != 0.0 {
                        let EPG = AKJ * AKJ;
                        let EPH = AKK * AKJ;
                        let EPI = EPG * AKJ;
                        let EPJ = ((EPH + EPH) * AKJ) + (AKK * EPG);
                        let EPK = (EPE + (EPI.abs())) + ECY;
                        let EPL = EPI / EPK;
                        let EPM = (EPJ - ((EPJ * ((AJ * (if EPI >= CQU { 1.0 } else { 0.0 })) - AK)) * EPL)) / EPK;
                        let EPN = if EPL < -1e-2f64 { 1.0 } else { 0.0 };
                        let EQF;
                        let EQG;
                        if EPN != 0.0 {
                            let EPZ = -1e-12f64 / EPL;
                            let EQA = ((EPM * EPZ) * W) / EPL;
                            EQF = EPZ;
                            EQG = EQA;
                        } else {
                            let EQB = EPM * EPL;
                            let EQC = ((EPL * EPL) + 4e-12f64).sqrt();
                            let EQD = CX * (EPL + EQC);
                            let EQE = (EPM + ((EQB + EQB) * (AK / (AJ * EQC)))) * CX;
                            EQF = EQD;
                            EQG = EQE;
                        }
                        let EQH = EQF - BB;
                        EPP = EQH;
                        EPQ = EQG;
                    } else {
                        EPP = U;
                        EPQ = EPO;
                    }
                    let EPR = staged[293] * BKA;
                    let EPS = EPR * EOZ;
                    let EPT = -EPC;
                    let EPU = rspice_limited_exp(EPT);
                    let EPV = EPS * EPU;
                    let EPW = EPV * EPP;
                    let EPX = EPQ * EPV;
                    let EPY = ((((EPA * EPR) * EPU) + (((EPD * W) * (rspice_limited_exp_derivative(EPT))) * EPS)) * EPP) + Lanes([0.0, EPX[0], 0.0, EPX[1]]);
                    EOR = EPW;
                    EOS = EPY;
                }
                ELO = EMV;
                ELP = EOR;
                ELQ = EMW;
                ELR = EOS;
            } else {
                ELO = A;
                ELP = A;
                ELQ = EDG;
                ELR = EDF;
            }
            let ELT = ELS * ELO;
            let ELU = ELQ * ELS;
            let ELV = ELS * ELP;
            let ELW = ELR * ELS;
            let ELX = AKT / ADX;
            let ELY = Lanes([0.0, AKU[0], AKU[1]]);
            let ELZ = rspice_limited_exp(ELX);
            let EMA = ((ELY - Lanes([(AEF * ELX), 0.0, 0.0])) / ADX) * (rspice_limited_exp_derivative(ELX));
            let EMB = ELZ - U;
            let EMC = AKT - AEA;
            let EMD = ELY - Lanes([AEI, 0.0, 0.0]);
            let EME = ADY + (ADZ * EMC);
            let EMF = EMB * EME;
            let EMG = (EMA * EME) + ((Lanes([AEG, 0.0, 0.0]) + (Lanes([(AEH * EMC), 0.0, 0.0]) + (EMD * ADZ))) * EMB);
            let EMH = (ACQ + AKT) / ADX;
            let EMI = -EMH;
            let EMJ = ((ELZ + AEB) - U) - (ACT * (rspice_limited_exp(EMI)));
            let EMK = ACL * EMJ;
            let EML = Lanes([(ACM * EMJ), 0.0, 0.0]) + (((EMA + Lanes([AEJ, 0.0, 0.0])) - (((((ELY - Lanes([(AEF * EMH), 0.0, 0.0])) / ADX) * W) * (rspice_limited_exp_derivative(EMI))) * ACT)) * ACL);
            let EMM = AKT - AEE;
            let EMN = ELY - Lanes([AEM, 0.0, 0.0]);
            let EMO = AEC + (AED * EMM);
            let EMP = Lanes([AEK, 0.0, 0.0]) + (Lanes([(AEL * EMM), 0.0, 0.0]) + (EMN * AED));
            let EQZ;
            let ERA;
            if ACN != 0.0 {
                let EQI = EMF / AO;
                let EQJ = EMC / ADX;
                let EQK = EQJ.tanh();
                let EQL = ((EMD - Lanes([(AEF * EQJ), 0.0, 0.0])) / ADX) * (AK - (EQK * EQK));
                let EQM = U - EQK;
                let EQN = EMK / AO;
                let EQO = U + EQK;
                let EQP = ((EQI * EQM) + (EQN * EQO)) / AO;
                let EQQ = EMM / ADX;
                let EQR = EQQ.tanh();
                let EQS = ((EMN - Lanes([(AEF * EQQ), 0.0, 0.0])) / ADX) * (AK - (EQR * EQR));
                let EQT = U - EQR;
                let EQU = EMO / AO;
                let EQV = U + EQR;
                let EQW = (EQP * EQT) + (EQU * EQV);
                let EQX = (((((((EMG / AO) * EQM) + ((EQL * W) * EQI)) + (((EML / AO) * EQO) + (EQL * EQN))) / AO) * EQT) + ((EQS * W) * EQP)) + (((EMP / AO) * EQV) + (EQS * EQU));
                EQZ = EQW;
                ERA = EQX;
            } else {
                EQZ = A;
                ERA = EQY;
            }
            let ERB = if YB > A { 1.0 } else { 0.0 };
            let ERG;
            let ERH;
            if ERB != 0.0 {
                let ERD = ERC - AKT;
                let ERE = AKU * W;
                let ERF = if ERD < (ERC * CMM) { 1.0 } else { 0.0 };
                let ESA;
                let ESB;
                if ERF != 0.0 {
                    let ERK = ERE / ERJ;
                    let ERL = ((-AKT) / ERJ) / ZP;
                    let ERN = ERL * ERM;
                    let ERO = (rspice_limited_exp(ERN)) - U;
                    let ERP = ACI * YB;
                    let ERQ = EQZ - (ERP * ERO);
                    let ERR = ERA - (Lanes([((YC * ACI) * ERO), 0.0, 0.0]) + (((((Lanes([0.0, ERK[0], ERK[1]]) - Lanes([(ZO * ERL), 0.0, 0.0])) / ZP) * ERM) * (rspice_limited_exp_derivative(ERN))) * ERP));
                    ESA = ERQ;
                    ESB = ERR;
                } else {
                    let ERS = ERE / ERJ;
                    let ERT = ((-AKT) / ERJ) / ZP;
                    let ERU = (ERT * ERC) / ERD;
                    let ERV = ERE * ERU;
                    let ERW = (rspice_limited_exp(ERU)) - U;
                    let ERX = ACI * YB;
                    let ERY = EQZ - (ERX * ERW);
                    let ERZ = ERA - (Lanes([((YC * ACI) * ERW), 0.0, 0.0]) + (((((((Lanes([0.0, ERS[0], ERS[1]]) - Lanes([(ZO * ERT), 0.0, 0.0])) / ZP) * ERC) - Lanes([0.0, ERV[0], ERV[1]])) / ERD) * (rspice_limited_exp_derivative(ERU))) * ERX));
                    ESA = ERY;
                    ESB = ERZ;
                }
                ERG = ESA;
                ERH = ESB;
            } else {
                ERG = EQZ;
                ERH = ERA;
            }
            let ERI = if YG > A { 1.0 } else { 0.0 };
            let ESG;
            let ESH;
            if ERI != 0.0 {
                let ESD = ESC - AKT;
                let ESE = AKU * W;
                let ESF = if ESD < (ESC * CMM) { 1.0 } else { 0.0 };
                let ESY;
                let ESZ;
                if ESF != 0.0 {
                    let ESJ = ESE / ERJ;
                    let ESK = ((-AKT) / ERJ) / AAD;
                    let ESL = ESK * ERM;
                    let ESM = (rspice_limited_exp(ESL)) - U;
                    let ESN = ACJ * YG;
                    let ESO = ERG - (ESN * ESM);
                    let ESP = ERH - (Lanes([((YH * ACJ) * ESM), 0.0, 0.0]) + (((((Lanes([0.0, ESJ[0], ESJ[1]]) - Lanes([(AAC * ESK), 0.0, 0.0])) / AAD) * ERM) * (rspice_limited_exp_derivative(ESL))) * ESN));
                    ESY = ESO;
                    ESZ = ESP;
                } else {
                    let ESQ = ESE / ERJ;
                    let ESR = ((-AKT) / ERJ) / AAD;
                    let ESS = (ESR * ESC) / ESD;
                    let EST = ESE * ESS;
                    let ESU = (rspice_limited_exp(ESS)) - U;
                    let ESV = ACJ * YG;
                    let ESW = ERG - (ESV * ESU);
                    let ESX = ERH - (Lanes([((YH * ACJ) * ESU), 0.0, 0.0]) + (((((((Lanes([0.0, ESQ[0], ESQ[1]]) - Lanes([(AAC * ESR), 0.0, 0.0])) / AAD) * ESC) - Lanes([0.0, EST[0], EST[1]])) / ESD) * (rspice_limited_exp_derivative(ESS))) * ESV));
                    ESY = ESW;
                    ESZ = ESX;
                }
                ESG = ESY;
                ESH = ESZ;
            } else {
                ESG = ERG;
                ESH = ERH;
            }
            let ESI = if YL > A { 1.0 } else { 0.0 };
            let ETE;
            let ETF;
            if ESI != 0.0 {
                let ETB = ETA - AKT;
                let ETC = AKU * W;
                let ETD = if ETB < (ETA * CMM) { 1.0 } else { 0.0 };
                let EUQ;
                let EUR;
                if ETD != 0.0 {
                    let EUB = ETC / ERJ;
                    let EUC = ((-AKT) / ERJ) / AAR;
                    let EUD = EUC * ERM;
                    let EUE = (rspice_limited_exp(EUD)) - U;
                    let EUF = ACK * YL;
                    let EUG = ESG - (EUF * EUE);
                    let EUH = ESH - (Lanes([((YM * ACK) * EUE), 0.0, 0.0]) + (((((Lanes([0.0, EUB[0], EUB[1]]) - Lanes([(AAQ * EUC), 0.0, 0.0])) / AAR) * ERM) * (rspice_limited_exp_derivative(EUD))) * EUF));
                    EUQ = EUG;
                    EUR = EUH;
                } else {
                    let EUI = ETC / ERJ;
                    let EUJ = ((-AKT) / ERJ) / AAR;
                    let EUK = (EUJ * ETA) / ETB;
                    let EUL = ETC * EUK;
                    let EUM = (rspice_limited_exp(EUK)) - U;
                    let EUN = ACK * YL;
                    let EUO = ESG - (EUN * EUM);
                    let EUP = ESH - (Lanes([((YM * ACK) * EUM), 0.0, 0.0]) + (((((((Lanes([0.0, EUI[0], EUI[1]]) - Lanes([(AAQ * EUJ), 0.0, 0.0])) / AAR) * ETA) - Lanes([0.0, EUL[0], EUL[1]])) / ETB) * (rspice_limited_exp_derivative(EUK))) * EUN));
                    EUQ = EUO;
                    EUR = EUP;
                }
                ETE = EUQ;
                ETF = EUR;
            } else {
                ETE = ESG;
                ETF = ESH;
            }
            let ETG = AKZ / AGV;
            let ETH = Lanes([0.0, ALA[0], ALA[1]]);
            let ETI = rspice_limited_exp(ETG);
            let ETJ = ((ETH - Lanes([(AHD * ETG), 0.0, 0.0])) / AGV) * (rspice_limited_exp_derivative(ETG));
            let ETK = AKZ - AGY;
            let ETL = ETH - Lanes([AHG, 0.0, 0.0]);
            let ETM = AGW + (AGX * ETK);
            let ETO = ETN * (ETI - U);
            let ETP = ETO * ETM;
            let ETQ = ((ETJ * ETN) * ETM) + ((Lanes([AHE, 0.0, 0.0]) + (Lanes([(AHF * ETK), 0.0, 0.0]) + (ETL * AGX))) * ETO);
            let ETR = (AFQ + AKZ) / AGV;
            let ETS = -ETR;
            let ETT = ETN * AEP;
            let ETU = ((ETI + AGZ) - U) - (AFT * (rspice_limited_exp(ETS)));
            let ETV = ETT * ETU;
            let ETW = Lanes([((AEQ * ETN) * ETU), 0.0, 0.0]) + (((ETJ + Lanes([AHH, 0.0, 0.0])) - (((((ETH - Lanes([(AHD * ETR), 0.0, 0.0])) / AGV) * W) * (rspice_limited_exp_derivative(ETS))) * AFT)) * ETT);
            let ETX = AKZ - AHC;
            let ETY = ETH - Lanes([AHK, 0.0, 0.0]);
            let ETZ = ETN * (AHA + (AHB * ETX));
            let EUA = (Lanes([AHI, 0.0, 0.0]) + (Lanes([(AHJ * ETX), 0.0, 0.0]) + (ETY * AHB))) * ETN;
            let EUV;
            let EUW;
            let EUX;
            let EUY;
            if AER != 0.0 {
                let EUS = if ETN > A { 1.0 } else { 0.0 };
                let EVQ;
                let EVR;
                if EUS != 0.0 {
                    let EVA = ETP / AO;
                    let EVB = ETK / AGV;
                    let EVC = EVB.tanh();
                    let EVD = ((ETL - Lanes([(AHD * EVB), 0.0, 0.0])) / AGV) * (AK - (EVC * EVC));
                    let EVE = U - EVC;
                    let EVF = ETV / AO;
                    let EVG = U + EVC;
                    let EVH = ((EVA * EVE) + (EVF * EVG)) / AO;
                    let EVI = ETX / AGV;
                    let EVJ = EVI.tanh();
                    let EVK = ((ETY - Lanes([(AHD * EVI), 0.0, 0.0])) / AGV) * (AK - (EVJ * EVJ));
                    let EVL = U - EVJ;
                    let EVM = ETZ / AO;
                    let EVN = U + EVJ;
                    let EVO = (EVH * EVL) + (EVM * EVN);
                    let EVP = (((((((ETQ / AO) * EVE) + ((EVD * W) * EVA)) + (((ETW / AO) * EVG) + (EVD * EVF))) / AO) * EVL) + ((EVK * W) * EVH)) + (((EUA / AO) * EVN) + (EVK * EVM));
                    EVQ = EVO;
                    EVR = EVP;
                } else {
                    EVQ = A;
                    EVR = EUT;
                }
                let EVT = if (if EVS > A { 1.0 } else { 0.0 }) != 0.0 && J != 0.0 { 1.0 } else { 0.0 };
                let EWY;
                let EWZ;
                if EVT != 0.0 {
                    let EVU = ALF / AGV;
                    let EVV = Lanes([0.0, ALG[0], ALG[1]]);
                    let EVW = rspice_limited_exp(EVU);
                    let EVX = ((EVV - Lanes([(AHD * EVU), 0.0, 0.0])) / AGV) * (rspice_limited_exp_derivative(EVU));
                    let EVY = ALF - AGY;
                    let EVZ = EVV - Lanes([AHG, 0.0, 0.0]);
                    let EWA = AGW + (AGX * EVY);
                    let EWB = EVS * (EVW - U);
                    let EWC = (AFQ + ALF) / AGV;
                    let EWD = -EWC;
                    let EWE = EVS * AEP;
                    let EWF = ((EVW + AGZ) - U) - (AFT * (rspice_limited_exp(EWD)));
                    let EWG = ALF - AHC;
                    let EWH = EVV - Lanes([AHK, 0.0, 0.0]);
                    let EWI = (EWB * EWA) / AO;
                    let EWJ = EVY / AGV;
                    let EWK = EWJ.tanh();
                    let EWL = ((EVZ - Lanes([(AHD * EWJ), 0.0, 0.0])) / AGV) * (AK - (EWK * EWK));
                    let EWM = U - EWK;
                    let EWN = (EWE * EWF) / AO;
                    let EWO = U + EWK;
                    let EWP = ((EWI * EWM) + (EWN * EWO)) / AO;
                    let EWQ = EWG / AGV;
                    let EWR = EWQ.tanh();
                    let EWS = ((EWH - Lanes([(AHD * EWQ), 0.0, 0.0])) / AGV) * (AK - (EWR * EWR));
                    let EWT = U - EWR;
                    let EWU = (EVS * (AHA + (AHB * EWG))) / AO;
                    let EWV = U + EWR;
                    let EWW = (EWP * EWT) + (EWU * EWV);
                    let EWX = ((((((((((EVX * EVS) * EWA) + ((Lanes([AHE, 0.0, 0.0]) + (Lanes([(AHF * EVY), 0.0, 0.0]) + (EVZ * AGX))) * EWB)) / AO) * EWM) + ((EWL * W) * EWI)) + ((((Lanes([((AEQ * EVS) * EWF), 0.0, 0.0]) + (((EVX + Lanes([AHH, 0.0, 0.0])) - (((((EVV - Lanes([(AHD * EWC), 0.0, 0.0])) / AGV) * W) * (rspice_limited_exp_derivative(EWD))) * AFT)) * EWE)) / AO) * EWO) + (EWL * EWN))) / AO) * EWT) + ((EWS * W) * EWP)) + (((((Lanes([AHI, 0.0, 0.0]) + (Lanes([(AHJ * EWG), 0.0, 0.0]) + (EWH * AHB))) * EVS) / AO) * EWV) + (EWS * EWU));
                    EWY = EWW;
                    EWZ = EWX;
                } else {
                    EWY = A;
                    EWZ = EUU;
                }
                EUV = EVQ;
                EUW = EWY;
                EUX = EVR;
                EUY = EWZ;
            } else {
                EUV = A;
                EUW = A;
                EUX = EUT;
                EUY = EUU;
            }
            let EUZ = if YQ > A { 1.0 } else { 0.0 };
            let EXE;
            let EXF;
            if EUZ != 0.0 {
                let EXB = EXA - AKZ;
                let EXC = ALA * W;
                let EXD = if EXB < (EXA * CMM) { 1.0 } else { 0.0 };
                let EXY;
                let EXZ;
                if EXD != 0.0 {
                    let EXH = EXC / ERJ;
                    let EXI = ((-AKZ) / ERJ) / ABF;
                    let EXJ = EXI * ERM;
                    let EXK = (rspice_limited_exp(EXJ)) - U;
                    let EXL = ETN * AEN;
                    let EXM = EXL * YQ;
                    let EXN = EUV - (EXM * EXK);
                    let EXO = EUX - (Lanes([((YR * EXL) * EXK), 0.0, 0.0]) + (((((Lanes([0.0, EXH[0], EXH[1]]) - Lanes([(ABE * EXI), 0.0, 0.0])) / ABF) * ERM) * (rspice_limited_exp_derivative(EXJ))) * EXM));
                    EXY = EXN;
                    EXZ = EXO;
                } else {
                    let EXP = EXC / ERJ;
                    let EXQ = ((-AKZ) / ERJ) / ABF;
                    let EXR = (EXQ * EXA) / EXB;
                    let EXS = EXC * EXR;
                    let EXT = (rspice_limited_exp(EXR)) - U;
                    let EXU = ETN * AEN;
                    let EXV = EXU * YQ;
                    let EXW = EUV - (EXV * EXT);
                    let EXX = EUX - (Lanes([((YR * EXU) * EXT), 0.0, 0.0]) + (((((((Lanes([0.0, EXP[0], EXP[1]]) - Lanes([(ABE * EXQ), 0.0, 0.0])) / ABF) * EXA) - Lanes([0.0, EXS[0], EXS[1]])) / EXB) * (rspice_limited_exp_derivative(EXR))) * EXV));
                    EXY = EXW;
                    EXZ = EXX;
                }
                EXE = EXY;
                EXF = EXZ;
            } else {
                EXE = EUV;
                EXF = EUX;
            }
            let EXG = if YV > A { 1.0 } else { 0.0 };
            let EYB;
            let EYC;
            if EXG != 0.0 {
                let EYA = if (if EVS > A { 1.0 } else { 0.0 }) != 0.0 && J != 0.0 { 1.0 } else { 0.0 };
                let EYI;
                let EYJ;
                if EYA != 0.0 {
                    let EYE = if AEO > ACK { 1.0 } else { 0.0 };
                    let EYU;
                    let EYV;
                    if EYE != 0.0 {
                        let EYO = ETN * (AEO - ACK);
                        let EYP = EYO * YV;
                        let EYQ = YW * EYO;
                        EYU = EYP;
                        EYV = EYQ;
                    } else {
                        let EYR = ETN * AEO;
                        let EYS = EYR * YV;
                        let EYT = YW * EYR;
                        EYU = EYS;
                        EYV = EYT;
                    }
                    EYI = EYU;
                    EYJ = EYV;
                } else {
                    let EYF = ETN * AEO;
                    let EYG = EYF * YV;
                    let EYH = YW * EYF;
                    EYI = EYG;
                    EYJ = EYH;
                }
                let EYL = EYK - AKZ;
                let EYM = ALA * W;
                let EYN = if EYL < (EYK * CMM) { 1.0 } else { 0.0 };
                let EZJ;
                let EZK;
                if EYN != 0.0 {
                    let EYW = EYM / ERJ;
                    let EYX = ((-AKZ) / ERJ) / ABT;
                    let EYY = EYX * ERM;
                    let EYZ = (rspice_limited_exp(EYY)) - U;
                    let EZA = EXE - (EYI * EYZ);
                    let EZB = EXF - (Lanes([(EYJ * EYZ), 0.0, 0.0]) + (((((Lanes([0.0, EYW[0], EYW[1]]) - Lanes([(ABS * EYX), 0.0, 0.0])) / ABT) * ERM) * (rspice_limited_exp_derivative(EYY))) * EYI));
                    EZJ = EZA;
                    EZK = EZB;
                } else {
                    let EZC = EYM / ERJ;
                    let EZD = ((-AKZ) / ERJ) / ABT;
                    let EZE = (EZD * EYK) / EYL;
                    let EZF = EYM * EZE;
                    let EZG = (rspice_limited_exp(EZE)) - U;
                    let EZH = EXE - (EYI * EZG);
                    let EZI = EXF - (Lanes([(EYJ * EZG), 0.0, 0.0]) + (((((((Lanes([0.0, EZC[0], EZC[1]]) - Lanes([(ABS * EZD), 0.0, 0.0])) / ABT) * EYK) - Lanes([0.0, EZF[0], EZF[1]])) / EYL) * (rspice_limited_exp_derivative(EZE))) * EYI));
                    EZJ = EZH;
                    EZK = EZI;
                }
                EYB = EZJ;
                EYC = EZK;
            } else {
                EYB = EXE;
                EYC = EXF;
            }
            let EYD = if ZA > A { 1.0 } else { 0.0 };
            let EZP;
            let EZQ;
            if EYD != 0.0 {
                let EZM = EZL - AKZ;
                let EZN = ALA * W;
                let EZO = if EZM < (EZL * CMM) { 1.0 } else { 0.0 };
                let FAH;
                let FAI;
                if EZO != 0.0 {
                    let EZS = EZN / ERJ;
                    let EZT = ((-AKZ) / ERJ) / ACH;
                    let EZU = EZT * ERM;
                    let EZV = (rspice_limited_exp(EZU)) - U;
                    let EZW = ACK * ZA;
                    let EZX = EYB - (EZW * EZV);
                    let EZY = EYC - (Lanes([((ZB * ACK) * EZV), 0.0, 0.0]) + (((((Lanes([0.0, EZS[0], EZS[1]]) - Lanes([(ACG * EZT), 0.0, 0.0])) / ACH) * ERM) * (rspice_limited_exp_derivative(EZU))) * EZW));
                    FAH = EZX;
                    FAI = EZY;
                } else {
                    let EZZ = EZN / ERJ;
                    let FAA = ((-AKZ) / ERJ) / ACH;
                    let FAB = (FAA * EZL) / EZM;
                    let FAC = EZN * FAB;
                    let FAD = (rspice_limited_exp(FAB)) - U;
                    let FAE = ACK * ZA;
                    let FAF = EYB - (FAE * FAD);
                    let FAG = EYC - (Lanes([((ZB * ACK) * FAD), 0.0, 0.0]) + (((((((Lanes([0.0, EZZ[0], EZZ[1]]) - Lanes([(ACG * FAA), 0.0, 0.0])) / ACH) * EZL) - Lanes([0.0, FAC[0], FAC[1]])) / EZM) * (rspice_limited_exp_derivative(FAB))) * FAE));
                    FAH = FAF;
                    FAI = FAG;
                }
                EZP = FAH;
                EZQ = FAI;
            } else {
                EZP = EYB;
                EZQ = EYC;
            }
            let FAJ;
            let FAK;
            if EZR != 0.0 {
                let FAV;
                let FAW;
                if EUZ != 0.0 {
                    let FAS = EXA - ALF;
                    let FAT = ALG * W;
                    let FAU = if FAS < (EXA * CMM) { 1.0 } else { 0.0 };
                    let FBO;
                    let FBP;
                    if FAU != 0.0 {
                        let FAX = FAT / ERJ;
                        let FAY = ((-ALF) / ERJ) / ABF;
                        let FAZ = FAY * ERM;
                        let FBA = (rspice_limited_exp(FAZ)) - U;
                        let FBB = EVS * AEN;
                        let FBC = FBB * YQ;
                        let FBD = EUW - (FBC * FBA);
                        let FBE = EUY - (Lanes([((YR * FBB) * FBA), 0.0, 0.0]) + (((((Lanes([0.0, FAX[0], FAX[1]]) - Lanes([(ABE * FAY), 0.0, 0.0])) / ABF) * ERM) * (rspice_limited_exp_derivative(FAZ))) * FBC));
                        FBO = FBD;
                        FBP = FBE;
                    } else {
                        let FBF = FAT / ERJ;
                        let FBG = ((-ALF) / ERJ) / ABF;
                        let FBH = (FBG * EXA) / FAS;
                        let FBI = FAT * FBH;
                        let FBJ = (rspice_limited_exp(FBH)) - U;
                        let FBK = EVS * AEN;
                        let FBL = FBK * YQ;
                        let FBM = EUW - (FBL * FBJ);
                        let FBN = EUY - (Lanes([((YR * FBK) * FBJ), 0.0, 0.0]) + (((((((Lanes([0.0, FBF[0], FBF[1]]) - Lanes([(ABE * FBG), 0.0, 0.0])) / ABF) * EXA) - Lanes([0.0, FBI[0], FBI[1]])) / FAS) * (rspice_limited_exp_derivative(FBH))) * FBL));
                        FBO = FBM;
                        FBP = FBN;
                    }
                    FAV = FBO;
                    FAW = FBP;
                } else {
                    FAV = EUW;
                    FAW = EUY;
                }
                let FBR;
                let FBS;
                if EXG != 0.0 {
                    let FBQ = if AEO > ACK { 1.0 } else { 0.0 };
                    let FBZ;
                    let FCA;
                    if FBQ != 0.0 {
                        let FBT = (EVS * (AEO - ACK)) + ACK;
                        let FBU = FBT * YV;
                        let FBV = YW * FBT;
                        FBZ = FBU;
                        FCA = FBV;
                    } else {
                        let FBW = EVS * AEO;
                        let FBX = FBW * YV;
                        let FBY = YW * FBW;
                        FBZ = FBX;
                        FCA = FBY;
                    }
                    let FCB = EYK - ALF;
                    let FCC = ALG * W;
                    let FCD = if FCB < (EYK * CMM) { 1.0 } else { 0.0 };
                    let FCR;
                    let FCS;
                    if FCD != 0.0 {
                        let FCE = FCC / ERJ;
                        let FCF = ((-ALF) / ERJ) / ABT;
                        let FCG = FCF * ERM;
                        let FCH = (rspice_limited_exp(FCG)) - U;
                        let FCI = FAV - (FBZ * FCH);
                        let FCJ = FAW - (Lanes([(FCA * FCH), 0.0, 0.0]) + (((((Lanes([0.0, FCE[0], FCE[1]]) - Lanes([(ABS * FCF), 0.0, 0.0])) / ABT) * ERM) * (rspice_limited_exp_derivative(FCG))) * FBZ));
                        FCR = FCI;
                        FCS = FCJ;
                    } else {
                        let FCK = FCC / ERJ;
                        let FCL = ((-ALF) / ERJ) / ABT;
                        let FCM = (FCL * EYK) / FCB;
                        let FCN = FCC * FCM;
                        let FCO = (rspice_limited_exp(FCM)) - U;
                        let FCP = FAV - (FBZ * FCO);
                        let FCQ = FAW - (Lanes([(FCA * FCO), 0.0, 0.0]) + (((((((Lanes([0.0, FCK[0], FCK[1]]) - Lanes([(ABS * FCL), 0.0, 0.0])) / ABT) * EYK) - Lanes([0.0, FCN[0], FCN[1]])) / FCB) * (rspice_limited_exp_derivative(FCM))) * FBZ));
                        FCR = FCP;
                        FCS = FCQ;
                    }
                    FBR = FCR;
                    FBS = FCS;
                } else {
                    FBR = FAV;
                    FBS = FAW;
                }
                FAJ = FBR;
                FAK = FBS;
            } else {
                FAJ = EUW;
                FAK = EUY;
            }
            let FAL = RF * ACI;
            let FAM = RG * ACI;
            let FAN = SG * ACJ;
            let FAO = SH * ACJ;
            let FAQ = (TH * FAP) * BIR;
            let FAR = (TI * FAP) * BIR;
            let FCT = if FAL > A { 1.0 } else { 0.0 };
            let FCY;
            let FCZ;
            if FCT != 0.0 {
                let FCU = AKT / UJ;
                let FCV = (ELY - Lanes([(UI * FCU), 0.0, 0.0])) / UJ;
                let FCX = if FCU < FCW { 1.0 } else { 0.0 };
                let FDP;
                let FDQ;
                if FCX != 0.0 {
                    let FDB = U - FCU;
                    let FDC = FCV * W;
                    let FDE = if FDD != U { 1.0 } else { 0.0 };
                    let FDW;
                    let FDX;
                    if FDE != 0.0 {
                        let FDR = if FDD == CX { 1.0 } else { 0.0 };
                        let FEF;
                        let FEG;
                        if FDR != 0.0 {
                            let FDY = FDB.sqrt();
                            let FDZ = U / FDY;
                            let FEA = (((FDC * (AK / (AJ * FDY))) * FDZ) * W) / FDY;
                            FEF = FDZ;
                            FEG = FEA;
                        } else {
                            let FEC = FEB * (FDB.ln());
                            let FED = rspice_limited_exp(FEC);
                            let FEE = ((FDC * (AK / FDB)) * FEB) * (rspice_limited_exp_derivative(FEC));
                            FEF = FED;
                            FEG = FEE;
                        }
                        let FEH = UJ * FAL;
                        let FEI = U - (FDB * FEF);
                        let FEJ = U - FDD;
                        let FEK = (FEH * FEI) / FEJ;
                        let FEL = (Lanes([(((UI * FAL) + (FAM * UJ)) * FEI), 0.0, 0.0]) + ((((FDC * FEF) + (FEG * FDB)) * W) * FEH)) / FEJ;
                        FDW = FEK;
                        FDX = FEL;
                    } else {
                        let FDS = UJ * FAL;
                        let FDT = -(FDB.ln());
                        let FDU = FDS * FDT;
                        let FDV = Lanes([(((UI * FAL) + (FAM * UJ)) * FDT), 0.0, 0.0]) + (((FDC * (AK / FDB)) * W) * FDS);
                        FDW = FDU;
                        FDX = FDV;
                    }
                    FDP = FDW;
                    FDQ = FDX;
                } else {
                    let FDF = FCU - U;
                    let FDH = FDG * FDF;
                    let FDJ = FDI * FDD;
                    let FDK = (FDJ * FDF) + (U + FDD);
                    let FDL = UJ * FAL;
                    let FDM = (FDH * FDK) + staged[301];
                    let FDN = FDL * FDM;
                    let FDO = Lanes([(((UI * FAL) + (FAM * UJ)) * FDM), 0.0, 0.0]) + ((((FCV * FDG) * FDK) + ((FCV * FDJ) * FDH)) * FDL);
                    FDP = FDN;
                    FDQ = FDO;
                }
                FCY = FDP;
                FCZ = FDQ;
            } else {
                FCY = A;
                FCZ = EQY;
            }
            let FDA = if FAN > A { 1.0 } else { 0.0 };
            let FEP;
            let FEQ;
            if FDA != 0.0 {
                let FEM = AKT / VI;
                let FEN = (ELY - Lanes([(VH * FEM), 0.0, 0.0])) / VI;
                let FEO = if FEM < FCW { 1.0 } else { 0.0 };
                let FFF;
                let FFG;
                if FEO != 0.0 {
                    let FES = U - FEM;
                    let FET = FEN * W;
                    let FEV = if FEU != U { 1.0 } else { 0.0 };
                    let FFM;
                    let FFN;
                    if FEV != 0.0 {
                        let FFH = if FEU == CX { 1.0 } else { 0.0 };
                        let FFV;
                        let FFW;
                        if FFH != 0.0 {
                            let FFO = FES.sqrt();
                            let FFP = U / FFO;
                            let FFQ = (((FET * (AK / (AJ * FFO))) * FFP) * W) / FFO;
                            FFV = FFP;
                            FFW = FFQ;
                        } else {
                            let FFS = FFR * (FES.ln());
                            let FFT = rspice_limited_exp(FFS);
                            let FFU = ((FET * (AK / FES)) * FFR) * (rspice_limited_exp_derivative(FFS));
                            FFV = FFT;
                            FFW = FFU;
                        }
                        let FFX = VI * FAN;
                        let FFY = U - (FES * FFV);
                        let FFZ = U - FEU;
                        let FGA = (FFX * FFY) / FFZ;
                        let FGB = (Lanes([(((VH * FAN) + (FAO * VI)) * FFY), 0.0, 0.0]) + ((((FET * FFV) + (FFW * FES)) * W) * FFX)) / FFZ;
                        FFM = FGA;
                        FFN = FGB;
                    } else {
                        let FFI = VI * FAN;
                        let FFJ = -(FES.ln());
                        let FFK = FFI * FFJ;
                        let FFL = Lanes([(((VH * FAN) + (FAO * VI)) * FFJ), 0.0, 0.0]) + (((FET * (AK / FES)) * W) * FFI);
                        FFM = FFK;
                        FFN = FFL;
                    }
                    FFF = FFM;
                    FFG = FFN;
                } else {
                    let FEW = FEM - U;
                    let FEY = FEX * FEW;
                    let FEZ = FDI * FEU;
                    let FFA = (FEZ * FEW) + (U + FEU);
                    let FFB = VI * FAN;
                    let FFC = (FEY * FFA) + staged[304];
                    let FFD = FFB * FFC;
                    let FFE = Lanes([(((VH * FAN) + (FAO * VI)) * FFC), 0.0, 0.0]) + ((((FEN * FEX) * FFA) + ((FEN * FEZ) * FEY)) * FFB);
                    FFF = FFD;
                    FFG = FFE;
                }
                FEP = FFF;
                FEQ = FFG;
            } else {
                FEP = A;
                FEQ = EQY;
            }
            let FER = if FAQ > A { 1.0 } else { 0.0 };
            let FGF;
            let FGG;
            if FER != 0.0 {
                let FGC = AKT / WH;
                let FGD = (ELY - Lanes([(WG * FGC), 0.0, 0.0])) / WH;
                let FGE = if FGC < FCW { 1.0 } else { 0.0 };
                let FGZ;
                let FHA;
                if FGE != 0.0 {
                    let FGM = U - FGC;
                    let FGN = FGD * W;
                    let FGP = if FGO != U { 1.0 } else { 0.0 };
                    let FHG;
                    let FHH;
                    if FGP != 0.0 {
                        let FHB = if FGO == CX { 1.0 } else { 0.0 };
                        let FHP;
                        let FHQ;
                        if FHB != 0.0 {
                            let FHI = FGM.sqrt();
                            let FHJ = U / FHI;
                            let FHK = (((FGN * (AK / (AJ * FHI))) * FHJ) * W) / FHI;
                            FHP = FHJ;
                            FHQ = FHK;
                        } else {
                            let FHM = FHL * (FGM.ln());
                            let FHN = rspice_limited_exp(FHM);
                            let FHO = ((FGN * (AK / FGM)) * FHL) * (rspice_limited_exp_derivative(FHM));
                            FHP = FHN;
                            FHQ = FHO;
                        }
                        let FHR = WH * FAQ;
                        let FHS = U - (FGM * FHP);
                        let FHT = U - FGO;
                        let FHU = (FHR * FHS) / FHT;
                        let FHV = (Lanes([(((WG * FAQ) + (FAR * WH)) * FHS), 0.0, 0.0]) + ((((FGN * FHP) + (FHQ * FGM)) * W) * FHR)) / FHT;
                        FHG = FHU;
                        FHH = FHV;
                    } else {
                        let FHC = WH * FAQ;
                        let FHD = -(FGM.ln());
                        let FHE = FHC * FHD;
                        let FHF = Lanes([(((WG * FAQ) + (FAR * WH)) * FHD), 0.0, 0.0]) + (((FGN * (AK / FGM)) * W) * FHC);
                        FHG = FHE;
                        FHH = FHF;
                    }
                    FGZ = FHG;
                    FHA = FHH;
                } else {
                    let FGQ = FGC - U;
                    let FGS = FGR * FGQ;
                    let FGT = FDI * FGO;
                    let FGU = (FGT * FGQ) + (U + FGO);
                    let FGV = WH * FAQ;
                    let FGW = (FGS * FGU) + staged[307];
                    let FGX = FGV * FGW;
                    let FGY = Lanes([(((WG * FAQ) + (FAR * WH)) * FGW), 0.0, 0.0]) + ((((FGD * FGR) * FGU) + ((FGD * FGT) * FGS)) * FGV);
                    FGZ = FGX;
                    FHA = FGY;
                }
                FGF = FGZ;
                FGG = FHA;
            } else {
                FGF = A;
                FGG = EQY;
            }
            let FGH = (FCY + FEP) + FGF;
            let FGI = (FCZ + FEQ) + FGG;
            let FGJ = (ETN * RR) * AEN;
            let FGK = (RS * ETN) * AEN;
            let FHZ;
            let FIA;
            if FGL != 0.0 {
                let FII;
                let FIJ;
                if FHW != 0.0 {
                    let FIE = (ETN * SS) * FID;
                    let FIF = (ST * ETN) * FID;
                    FII = FIE;
                    FIJ = FIF;
                } else {
                    let FIG = (ETN * SS) * AEO;
                    let FIH = (ST * ETN) * AEO;
                    FII = FIG;
                    FIJ = FIH;
                }
                FHZ = FII;
                FIA = FIJ;
            } else {
                let FHX = (ETN * SS) * AEO;
                let FHY = (ST * ETN) * AEO;
                FHZ = FHX;
                FIA = FHY;
            }
            let FIB = (TT * FAP) * BIR;
            let FIC = (TU * FAP) * BIR;
            let FIK = if FGJ > A { 1.0 } else { 0.0 };
            let FIP;
            let FIQ;
            if FIK != 0.0 {
                let FIL = AML / UU;
                let FIM = (Lanes([0.0, AMO[0], AMO[1], AMO[2], AMO[3]]) - Lanes([(UT * FIL), 0.0, 0.0, 0.0, 0.0])) / UU;
                let FIN = if FIL < FCW { 1.0 } else { 0.0 };
                let FJG;
                let FJH;
                if FIN != 0.0 {
                    let FIS = U - FIL;
                    let FIT = FIM * W;
                    let FIV = if FIU != U { 1.0 } else { 0.0 };
                    let FJN;
                    let FJO;
                    if FIV != 0.0 {
                        let FJI = if FIU == CX { 1.0 } else { 0.0 };
                        let FJW;
                        let FJX;
                        if FJI != 0.0 {
                            let FJP = FIS.sqrt();
                            let FJQ = U / FJP;
                            let FJR = (((FIT * (AK / (AJ * FJP))) * FJQ) * W) / FJP;
                            FJW = FJQ;
                            FJX = FJR;
                        } else {
                            let FJT = FJS * (FIS.ln());
                            let FJU = rspice_limited_exp(FJT);
                            let FJV = ((FIT * (AK / FIS)) * FJS) * (rspice_limited_exp_derivative(FJT));
                            FJW = FJU;
                            FJX = FJV;
                        }
                        let FJY = UU * FGJ;
                        let FJZ = U - (FIS * FJW);
                        let FKA = U - FIU;
                        let FKB = (FJY * FJZ) / FKA;
                        let FKC = (Lanes([(((UT * FGJ) + (FGK * UU)) * FJZ), 0.0, 0.0, 0.0, 0.0]) + ((((FIT * FJW) + (FJX * FIS)) * W) * FJY)) / FKA;
                        FJN = FKB;
                        FJO = FKC;
                    } else {
                        let FJJ = UU * FGJ;
                        let FJK = -(FIS.ln());
                        let FJL = FJJ * FJK;
                        let FJM = Lanes([(((UT * FGJ) + (FGK * UU)) * FJK), 0.0, 0.0, 0.0, 0.0]) + (((FIT * (AK / FIS)) * W) * FJJ);
                        FJN = FJL;
                        FJO = FJM;
                    }
                    FJG = FJN;
                    FJH = FJO;
                } else {
                    let FIW = FIL - U;
                    let FIY = FIX * FIW;
                    let FIZ = FDI * FIU;
                    let FJA = (FIZ * FIW) + (U + FIU);
                    let FJB = UU * FGJ;
                    let FJD = (FIY * FJA) + FJC;
                    let FJE = FJB * FJD;
                    let FJF = Lanes([(((UT * FGJ) + (FGK * UU)) * FJD), 0.0, 0.0, 0.0, 0.0]) + ((((FIM * FIX) * FJA) + ((FIM * FIZ) * FIY)) * FJB);
                    FJG = FJE;
                    FJH = FJF;
                }
                FIP = FJG;
                FIQ = FJH;
            } else {
                FIP = A;
                FIQ = FIO;
            }
            let FIR = if FHZ > A { 1.0 } else { 0.0 };
            let FKG;
            let FKH;
            if FIR != 0.0 {
                let FKD = AML / VT;
                let FKE = (Lanes([0.0, AMO[0], AMO[1], AMO[2], AMO[3]]) - Lanes([(VS * FKD), 0.0, 0.0, 0.0, 0.0])) / VT;
                let FKF = if FKD < FCW { 1.0 } else { 0.0 };
                let FKX;
                let FKY;
                if FKF != 0.0 {
                    let FKJ = U - FKD;
                    let FKK = FKE * W;
                    let FKM = if FKL != U { 1.0 } else { 0.0 };
                    let FLE;
                    let FLF;
                    if FKM != 0.0 {
                        let FKZ = if FKL == CX { 1.0 } else { 0.0 };
                        let FLN;
                        let FLO;
                        if FKZ != 0.0 {
                            let FLG = FKJ.sqrt();
                            let FLH = U / FLG;
                            let FLI = (((FKK * (AK / (AJ * FLG))) * FLH) * W) / FLG;
                            FLN = FLH;
                            FLO = FLI;
                        } else {
                            let FLK = FLJ * (FKJ.ln());
                            let FLL = rspice_limited_exp(FLK);
                            let FLM = ((FKK * (AK / FKJ)) * FLJ) * (rspice_limited_exp_derivative(FLK));
                            FLN = FLL;
                            FLO = FLM;
                        }
                        let FLP = VT * FHZ;
                        let FLQ = U - (FKJ * FLN);
                        let FLR = U - FKL;
                        let FLS = (FLP * FLQ) / FLR;
                        let FLT = (Lanes([(((VS * FHZ) + (FIA * VT)) * FLQ), 0.0, 0.0, 0.0, 0.0]) + ((((FKK * FLN) + (FLO * FKJ)) * W) * FLP)) / FLR;
                        FLE = FLS;
                        FLF = FLT;
                    } else {
                        let FLA = VT * FHZ;
                        let FLB = -(FKJ.ln());
                        let FLC = FLA * FLB;
                        let FLD = Lanes([(((VS * FHZ) + (FIA * VT)) * FLB), 0.0, 0.0, 0.0, 0.0]) + (((FKK * (AK / FKJ)) * W) * FLA);
                        FLE = FLC;
                        FLF = FLD;
                    }
                    FKX = FLE;
                    FKY = FLF;
                } else {
                    let FKN = FKD - U;
                    let FKP = FKO * FKN;
                    let FKQ = FDI * FKL;
                    let FKR = (FKQ * FKN) + (U + FKL);
                    let FKS = VT * FHZ;
                    let FKU = (FKP * FKR) + FKT;
                    let FKV = FKS * FKU;
                    let FKW = Lanes([(((VS * FHZ) + (FIA * VT)) * FKU), 0.0, 0.0, 0.0, 0.0]) + ((((FKE * FKO) * FKR) + ((FKE * FKQ) * FKP)) * FKS);
                    FKX = FKV;
                    FKY = FKW;
                }
                FKG = FKX;
                FKH = FKY;
            } else {
                FKG = A;
                FKH = FIO;
            }
            let FKI = if FIB > A { 1.0 } else { 0.0 };
            let FLX;
            let FLY;
            if FKI != 0.0 {
                let FLU = AML / WS;
                let FLV = (Lanes([0.0, AMO[0], AMO[1], AMO[2], AMO[3]]) - Lanes([(WR * FLU), 0.0, 0.0, 0.0, 0.0])) / WS;
                let FLW = if FLU < FCW { 1.0 } else { 0.0 };
                let FMP;
                let FMQ;
                if FLW != 0.0 {
                    let FMC = U - FLU;
                    let FMD = FLV * W;
                    let FMF = if FME != U { 1.0 } else { 0.0 };
                    let FMW;
                    let FMX;
                    if FMF != 0.0 {
                        let FMR = if FME == CX { 1.0 } else { 0.0 };
                        let FNF;
                        let FNG;
                        if FMR != 0.0 {
                            let FMY = FMC.sqrt();
                            let FMZ = U / FMY;
                            let FNA = (((FMD * (AK / (AJ * FMY))) * FMZ) * W) / FMY;
                            FNF = FMZ;
                            FNG = FNA;
                        } else {
                            let FNC = FNB * (FMC.ln());
                            let FND = rspice_limited_exp(FNC);
                            let FNE = ((FMD * (AK / FMC)) * FNB) * (rspice_limited_exp_derivative(FNC));
                            FNF = FND;
                            FNG = FNE;
                        }
                        let FNH = WS * FIB;
                        let FNI = U - (FMC * FNF);
                        let FNJ = U - FME;
                        let FNK = (FNH * FNI) / FNJ;
                        let FNL = (Lanes([(((WR * FIB) + (FIC * WS)) * FNI), 0.0, 0.0, 0.0, 0.0]) + ((((FMD * FNF) + (FNG * FMC)) * W) * FNH)) / FNJ;
                        FMW = FNK;
                        FMX = FNL;
                    } else {
                        let FMS = WS * FIB;
                        let FMT = -(FMC.ln());
                        let FMU = FMS * FMT;
                        let FMV = Lanes([(((WR * FIB) + (FIC * WS)) * FMT), 0.0, 0.0, 0.0, 0.0]) + (((FMD * (AK / FMC)) * W) * FMS);
                        FMW = FMU;
                        FMX = FMV;
                    }
                    FMP = FMW;
                    FMQ = FMX;
                } else {
                    let FMG = FLU - U;
                    let FMI = FMH * FMG;
                    let FMJ = FDI * FME;
                    let FMK = (FMJ * FMG) + (U + FME);
                    let FML = WS * FIB;
                    let FMM = (FMI * FMK) + staged[318];
                    let FMN = FML * FMM;
                    let FMO = Lanes([(((WR * FIB) + (FIC * WS)) * FMM), 0.0, 0.0, 0.0, 0.0]) + ((((FLV * FMH) * FMK) + ((FLV * FMJ) * FMI)) * FML);
                    FMP = FMN;
                    FMQ = FMO;
                }
                FLX = FMP;
                FLY = FMQ;
            } else {
                FLX = A;
                FLY = FIO;
            }
            let FLZ = (FIP + FKG) + FLX;
            let FMA = (FIQ + FKH) + FLY;
            let FNO;
            let FNP;
            if FMB != 0.0 {
                let FNM = (EVS * RR) * AEN;
                let FNN = (RS * EVS) * AEN;
                let FNW;
                let FNX;
                if FGL != 0.0 {
                    let FNS = SS * FNR;
                    let FNT = ST * FNR;
                    FNW = FNS;
                    FNX = FNT;
                } else {
                    let FNU = (EVS * SS) * AEO;
                    let FNV = (ST * EVS) * AEO;
                    FNW = FNU;
                    FNX = FNV;
                }
                let FNY = if FNM > A { 1.0 } else { 0.0 };
                let FOC;
                let FOD;
                if FNY != 0.0 {
                    let FNZ = ALF / UU;
                    let FOA = (Lanes([0.0, ALG[0], ALG[1]]) - Lanes([(UT * FNZ), 0.0, 0.0])) / UU;
                    let FOB = if FNZ < FCW { 1.0 } else { 0.0 };
                    let FOQ;
                    let FOR;
                    if FOB != 0.0 {
                        let FOF = U - FNZ;
                        let FOG = FOA * W;
                        let FOH = if FIU != U { 1.0 } else { 0.0 };
                        let FOX;
                        let FOY;
                        if FOH != 0.0 {
                            let FOS = if FIU == CX { 1.0 } else { 0.0 };
                            let FPF;
                            let FPG;
                            if FOS != 0.0 {
                                let FOZ = FOF.sqrt();
                                let FPA = U / FOZ;
                                let FPB = (((FOG * (AK / (AJ * FOZ))) * FPA) * W) / FOZ;
                                FPF = FPA;
                                FPG = FPB;
                            } else {
                                let FPC = FJS * (FOF.ln());
                                let FPD = rspice_limited_exp(FPC);
                                let FPE = ((FOG * (AK / FOF)) * FJS) * (rspice_limited_exp_derivative(FPC));
                                FPF = FPD;
                                FPG = FPE;
                            }
                            let FPH = UU * FNM;
                            let FPI = U - (FOF * FPF);
                            let FPJ = U - FIU;
                            let FPK = (FPH * FPI) / FPJ;
                            let FPL = (Lanes([(((UT * FNM) + (FNN * UU)) * FPI), 0.0, 0.0]) + ((((FOG * FPF) + (FPG * FOF)) * W) * FPH)) / FPJ;
                            FOX = FPK;
                            FOY = FPL;
                        } else {
                            let FOT = UU * FNM;
                            let FOU = -(FOF.ln());
                            let FOV = FOT * FOU;
                            let FOW = Lanes([(((UT * FNM) + (FNN * UU)) * FOU), 0.0, 0.0]) + (((FOG * (AK / FOF)) * W) * FOT);
                            FOX = FOV;
                            FOY = FOW;
                        }
                        FOQ = FOX;
                        FOR = FOY;
                    } else {
                        let FOI = FNZ - U;
                        let FOJ = FIX * FOI;
                        let FOK = FDI * FIU;
                        let FOL = (FOK * FOI) + (U + FIU);
                        let FOM = UU * FNM;
                        let FON = (FOJ * FOL) + FJC;
                        let FOO = FOM * FON;
                        let FOP = Lanes([(((UT * FNM) + (FNN * UU)) * FON), 0.0, 0.0]) + ((((FOA * FIX) * FOL) + ((FOA * FOK) * FOJ)) * FOM);
                        FOQ = FOO;
                        FOR = FOP;
                    }
                    FOC = FOQ;
                    FOD = FOR;
                } else {
                    FOC = A;
                    FOD = EUU;
                }
                let FOE = if FNW > A { 1.0 } else { 0.0 };
                let FPP;
                let FPQ;
                if FOE != 0.0 {
                    let FPM = ALF / VT;
                    let FPN = (Lanes([0.0, ALG[0], ALG[1]]) - Lanes([(VS * FPM), 0.0, 0.0])) / VT;
                    let FPO = if FPM < FCW { 1.0 } else { 0.0 };
                    let FQE;
                    let FQF;
                    if FPO != 0.0 {
                        let FPT = U - FPM;
                        let FPU = FPN * W;
                        let FPV = if FKL != U { 1.0 } else { 0.0 };
                        let FQL;
                        let FQM;
                        if FPV != 0.0 {
                            let FQG = if FKL == CX { 1.0 } else { 0.0 };
                            let FQT;
                            let FQU;
                            if FQG != 0.0 {
                                let FQN = FPT.sqrt();
                                let FQO = U / FQN;
                                let FQP = (((FPU * (AK / (AJ * FQN))) * FQO) * W) / FQN;
                                FQT = FQO;
                                FQU = FQP;
                            } else {
                                let FQQ = FLJ * (FPT.ln());
                                let FQR = rspice_limited_exp(FQQ);
                                let FQS = ((FPU * (AK / FPT)) * FLJ) * (rspice_limited_exp_derivative(FQQ));
                                FQT = FQR;
                                FQU = FQS;
                            }
                            let FQV = VT * FNW;
                            let FQW = U - (FPT * FQT);
                            let FQX = U - FKL;
                            let FQY = (FQV * FQW) / FQX;
                            let FQZ = (Lanes([(((VS * FNW) + (FNX * VT)) * FQW), 0.0, 0.0]) + ((((FPU * FQT) + (FQU * FPT)) * W) * FQV)) / FQX;
                            FQL = FQY;
                            FQM = FQZ;
                        } else {
                            let FQH = VT * FNW;
                            let FQI = -(FPT.ln());
                            let FQJ = FQH * FQI;
                            let FQK = Lanes([(((VS * FNW) + (FNX * VT)) * FQI), 0.0, 0.0]) + (((FPU * (AK / FPT)) * W) * FQH);
                            FQL = FQJ;
                            FQM = FQK;
                        }
                        FQE = FQL;
                        FQF = FQM;
                    } else {
                        let FPW = FPM - U;
                        let FPX = FKO * FPW;
                        let FPY = FDI * FKL;
                        let FPZ = (FPY * FPW) + (U + FKL);
                        let FQA = VT * FNW;
                        let FQB = (FPX * FPZ) + FKT;
                        let FQC = FQA * FQB;
                        let FQD = Lanes([(((VS * FNW) + (FNX * VT)) * FQB), 0.0, 0.0]) + ((((FPN * FKO) * FPZ) + ((FPN * FPY) * FPX)) * FQA);
                        FQE = FQC;
                        FQF = FQD;
                    }
                    FPP = FQE;
                    FPQ = FQF;
                } else {
                    FPP = A;
                    FPQ = EUU;
                }
                let FPR = FOC + FPP;
                let FPS = FOD + FPQ;
                FNO = FPR;
                FNP = FPS;
            } else {
                FNO = A;
                FNP = EUU;
            }
            let FSI;
            let FSJ;
            if FNQ != 0.0 {
                let FRA = 3e2f64 / Q;
                let FRC = FRA.powf(FRB);
                let FRD = (((P * FRA) * W) / Q) * (FRB * (FRA.powf(staged[441])));
                let FRF = (AKH - AKG) * FRE;
                let FRG = (FRE * (AJP - AKD)) / S;
                let FRI = FRH * FRC;
                let FRN = FRM * ((AJQ - AXR) - AKE);
                let FRO = (rspice_limited_exp(FRN)).tanh();
                let FRQ = (AMZ * BIR) * BKA;
                let FRR = FRQ * (FRJ * (rspice_limited_exp(FRI)));
                let FRS = rspice_limited_exp(FRG);
                let FRT = FRR * FRS;
                let FRU = (-((FRK * FRC) * FRL)) * AWB;
                let FRV = rspice_limited_exp(FRU);
                let FRW = FRT * FRV;
                let FRX = (FRP * FRO) / S;
                let FRY = rspice_limited_exp(FRX);
                let FRZ = FRW * FRY;
                let FSA = (((Lanes([(((((FRD * FRH) * (rspice_limited_exp_derivative(FRI))) * FRJ) * FRQ) * FRS), 0.0, 0.0]) + ((((Lanes([0.0, FRF[0], FRF[1]]) - Lanes([(T * FRG), 0.0, 0.0])) / S) * (rspice_limited_exp_derivative(FRG))) * FRR)) * FRV) + Lanes([((((((FRD * FRK) * FRL) * W) * AWB) * (rspice_limited_exp_derivative(FRU))) * FRT), 0.0, 0.0])) * FRY;
                let FSC = AOH * FSB;
                let FSD = (FSB * AOI) / S;
                let FSE = (rspice_limited_exp(FSD)) - U;
                let FSF = (((Lanes([0.0, FSC[0], FSC[1], FSC[2]]) - Lanes([(T * FSD), 0.0, 0.0, 0.0])) / S) * (rspice_limited_exp_derivative(FSD))) * FRZ;
                let FSG = EDR * (FRZ * FSE);
                let FSH = (((Lanes([FSA[0], 0.0, FSA[1], 0.0, FSA[2]]) + ((((((((((Lanes([0.0, 0.0, 0.0, AJT[0], AJT[1]]) - Lanes([AXS[0], AXS[1], AXS[2], 0.0, AXS[3]])) - Lanes([0.0, 0.0, AKI[0], 0.0, AKI[1]])) * FRM) * (rspice_limited_exp_derivative(FRN))) * (AK - (FRO * FRO))) * FRP) - Lanes([(T * FRX), 0.0, 0.0, 0.0, 0.0])) / S) * (rspice_limited_exp_derivative(FRX))) * FRW)) * FSE) + Lanes([FSF[0], FSF[1], FSF[2], 0.0, FSF[3]])) * EDR;
                FSI = FSG;
                FSJ = FSH;
            } else {
                FSI = A;
                FSJ = BCT;
            }
            let FSL = (ADB * S) * FSK;
            let FSM = (T * ADB) * FSK;
            let FSN = CCP / CNL;
            let FSO = (CCU - (CNM * FSN)) / CNL;
            let FST;
            if FSP != 0.0 {
                FST = A;
            } else {
                let FSR = FSQ * ((if (((CDI / FSQ) + parameters[784]) / FSN) >= AX { (((CDI / FSQ) + parameters[784]) / FSN) } else { AX }).ln());
                let FSS = if FSR < A { 1.0 } else { 0.0 };
                let FTJ = if FSS != 0.0 {
                    A
                } else {
                    FSR
                };
                FST = FTJ;
            }
            let FSU = S / FSK;
            let FSV = FSU * ((ASZ + ASU) + ASV);
            let FSW = AO * BZY;
            let FSX = BZX * AO;
            let FSY = (FSW * ASZ) * S;
            let FSZ = (((FSY * BYE) * CNI) * CNB) / FSK;
            let FTA = ((4.112842231783458e-57f64 * S) * (COV.abs())) * CNL;
            let FTB = FSK * S;
            let FTC = (FTB * COV) * COV;
            let FTF = (FTE + (FTD * FSZ)) + ((parameters[800] * FSZ) * FSZ);
            let FTG = FSZ + FSV;
            let FTH = FTG * FTG;
            let FTI = staged[325] * S;
            if G != 0.0 {
                let FTK = (AJV - AUY) / S;
                let FTL = ((staged[326] / S).sqrt()) / ASZ;
                let FTM = (parameters[1068] / AT).ln();
                let FTN = (CX * FTK) - (AYC * (U + (FTL / AYB)));
                let FTO = FTN + (((FTN * FTN) + (AYH * FTK)).sqrt());
                let FTP = if FTK < A { 1.0 } else { 0.0 };
                let FUW = if FTP != 0.0 {
                    let FUQ = (FTK - FTO) / FTL;
                    let FUR = -((if ((U - FTO) + (FUQ * FUQ)) >= AX { ((U - FTO) + (FUQ * FUQ)) } else { AX }).ln());
                    FUR
                } else {
                    let FUS = rspice_limited_exp((-FTO));
                    let FUT = CX * FTL;
                    let FUU = ((((FTK - U) + FUS) + (FUT * FUT)).sqrt()) - FUT;
                    let FUV = ((FUU * FUU) + U) - FUS;
                    FUV
                };
                let FUX = FUW + U;
                let FUY = FUW - U;
                let FUZ = FUY * FUY;
                let FVA = (CX * (FUX + ((FUZ + 1e0f64).sqrt()))).sqrt();
                let FVB = AO * FVA;
                let FVC = (U + (FTL / FVB)) / FTL;
                let FVD = (FUW - (AO * FTM)) - AUV;
                let FVE = FVD - ((if ((ADB * FVC) * FVA) >= AX { ((ADB * FVC) * FVA) } else { AX }).ln());
                let FVF = CX * ((FVE - BAG) - (((FVE * (FVE + BAH)) + BAJ).sqrt()));
                let FVG = if FVF <= -6.8e1f64 { 1.0 } else { 0.0 };
                let FVV;
                if FVG != 0.0 {
                    let FVH = if FVF < -1.1e2f64 { 1.0 } else { 0.0 };
                    let FVZ;
                    if FVH != 0.0 {
                        FVZ = FVX;
                    } else {
                        let FVY = if FVF > -9e1f64 { 1.0 } else { 0.0 };
                        let FWG = if FVY != 0.0 {
                            let FWB = rspice_limited_exp(FVF);
                            FWB
                        } else {
                            let FWD = (FVF - FWC) / BDI;
                            let FWE = FWD * FWD;
                            let FWF = rspice_limited_exp((FWC + (BDI * ((7.8125e-2f64 + (CX * FWD)) + (FWE * (9.375e-1f64 - (FWE * (BDO - FWE))))))));
                            FWF
                        };
                        FVZ = FWG;
                    }
                    let FWA = FVZ * (((U + FVD) - FVF) - ((if ((AO * FVC) * (((FVZ * AO) * FVC) + FVB)) >= AX { ((AO * FVC) * (((FVZ * AO) * FVC) + FVB)) } else { AX }).ln()));
                    FVV = FWA;
                } else {
                    let FVI = rspice_limited_exp(FVF);
                    let FVJ = AO * FVI;
                    let FVK = FVJ * FVC;
                    let FVL = FVC + (U / FVA);
                    let FVM = FVI - (((FVJ + ((if (FVK * (FVK + FVB)) >= AX { (FVK * (FVK + FVB)) } else { AX }).ln())) - FVD) / ((AO + (1e0f64 / FVI)) + (FVL / ((FVC * FVI) + FVA))));
                    let FVN = AO * FVM;
                    let FVO = FVN * FVC;
                    let FVP = (FVN + ((if (FVO * (FVO + FVB)) >= AX { (FVO * (FVO + FVB)) } else { AX }).ln())) - FVD;
                    let FVQ = (FVC * FVM) + FVA;
                    let FVR = FVL / FVQ;
                    let FVS = (AO + (1e0f64 / FVM)) + FVR;
                    let FVT = U / FVM;
                    let FVU = FVM - ((FVP / FVS) * (U + ((FVP * (((-1e0f64 * (FVT * FVT)) - (1e0f64 / (((FVA * FVA) * FVA) * FVQ))) - (FVR * FVR))) / ((AO * FVS) * FVS))));
                    FVV = FVU;
                }
                let FVW = if 0.0f64 != 0.0 && (if FUW < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let FWJ = if FVW != 0.0 {
                    let FWH = -4e0f64 / (ASC * FUW);
                    FWH
                } else {
                    let FWI = CX * (FUX + ((FUZ + 1e0f64).sqrt()));
                    FWI
                };
                let FWK = (CNL * ASZ) * BKA;
                let FWL = (COV * staged[328]) / (((FSW * FWK) * ATN) * ATN);
                let FWM = if (U + (ADB * (((FVV * FVV) + FVV) - ((COV * staged[327]) / ((((AO * (U + (FTL / (AO * (FWJ.sqrt()))))) * FWK) * S) * S))))) < U { 1.0 } else { 0.0 };
                let FWN = -5e-1f64 + (CX * ((U + (ADB * (((BYE * BYE) + BYE) + FWL))).sqrt()));
                if FWO != 0.0 {
                    let FWP = (FSY * FWN) / FSK;
                    let FWQ = if ((((FTI / ((staged[332] * FSV) * FSV)) * COV) * COV) + (((FTA / staged[330]) * (((FTE * ((if ((FWP + FSV) / FTG) >= AX { ((FWP + FSV) / FTG) } else { AX }).ln())) + (FTD * (FWP - FSZ))) + (staged[329] * ((FWP * FWP) - (FSZ * FSZ))))) + ((((FTC / staged[331]) * FST) * FTF) / FTH))) > A { 1.0 } else { 0.0 };
                } else {
                }
                let FWR = if ((((staged[333] * S) / ((staged[334] * FSV) * FSV)) * COV) * COV) > A { 1.0 } else { 0.0 };
            } else {
                if FWS != 0.0 {
                    let FWX = if FWT != 0.0 {
                        let FWU = (staged[337] / (U + ((CBR / staged[335]).powf(staged[336])))) / FTE;
                        let FWV = FWU - U;
                        let FWW = FTE * (CX * ((FWU + U) + (((FWV * FWV) + staged[338]).sqrt())));
                        FWW
                    } else {
                        FTE
                    };
                    let FWY = (((FSY * BCP) * CNI) * CNB) / FSK;
                    let FWZ = if ((((((FWX * FSK) * S) / ((staged[343] * FSV) * FSV)) * COV) * COV) + (((FTA / staged[341]) * (((FWX * ((if ((FWY + FSV) / FTG) >= AX { ((FWY + FSV) / FTG) } else { AX }).ln())) + (FTD * (FWY - FSZ))) + (staged[340] * ((FWY * FWY) - (FSZ * FSZ))))) + ((((FTC / staged[342]) * FST) * FTF) / FTH))) > A { 1.0 } else { 0.0 };
                } else {
                }
            }
            let FTQ = CBR / FSN;
            let FTR = FTQ / AWB;
            let FTS = FTR * FTR;
            let FTT = (((CBT - (FSO * FTQ)) / FSN) / AWB) * FTR;
            let FTU = FTT + FTT;
            let FTX = FTW * (U + (FTV * FTS));
            let FTY = (FTU * FTV) * FTW;
            let FUB = FUA * (U + (FTZ * FTS));
            let FUE = FUD * (U + (FUC * FTS));
            let FUF = AYC * FTX;
            let FUH = (((FTY * AYC) * FTX) + (FTY * FUF)) * FUG;
            let FUI = (((FUF * FTX) - U) * FUG) + U;
            let FUJ = FUE * FUE;
            let FUK = ((FTU * FUC) * FUD) * FUE;
            let FUL = FUK + FUK;
            let FUM = FUB * FUB;
            let FUN = ((FTU * FTZ) * FUA) * FUB;
            let FUO = FUN + FUN;
            let FXD;
            let FXE;
            let FXF;
            let FXG;
            let FXH;
            let FXI;
            let FXJ;
            let FXK;
            let FXL;
            let FXM;
            let FXN;
            let FXO;
            let FXP;
            let FXQ;
            let FXR;
            let FXS;
            if FUP != 0.0 {
                FXD = A;
                FXE = A;
                FXF = A;
                FXG = A;
                FXH = A;
                FXI = A;
                FXJ = A;
                FXK = A;
                FXL = FXA;
                FXM = FXB;
                FXN = FXA;
                FXO = FXB;
                FXP = FXB;
                FXQ = FXB;
                FXR = FXB;
                FXS = FXB;
            } else {
                let FYU;
                let FYV;
                let FYW;
                let FYX;
                let FYY;
                let FYZ;
                let FZA;
                let FZB;
                let FZC;
                let FZD;
                let FZE;
                let FZF;
                let FZG;
                let FZH;
                let FZI;
                let FZJ;
                if FXC != 0.0 {
                    let FXT = FSW * ATN;
                    let FXU = ATO * FSW;
                    let FXV = CNL * CJQ;
                    let FXW = (FXV * CHL) * ASZ;
                    let FXX = FXW * FXT;
                    let FXY = ((((((CNM * CJQ) + (CJR * CNL)) * CHL) + (CHM * FXV)) * ASZ) * FXT) + (((FSX * ATN) + Lanes([FXU[0], FXU[1], FXU[2], 0.0, FXU[3]])) * FXW);
                    let FXZ = CX * CAL;
                    let FYA = CAF * CX;
                    let FYB = FXZ + CX;
                    let FYC = FYB * FYB;
                    let FYD = FYA * FYB;
                    let FYE = FYD + FYD;
                    let FYF = FYC * FYB;
                    let FYG = (FYE * FYB) + (FYA * FYC);
                    let FYH = (AYH * FXZ) + CX;
                    let FYI = FYH * CAB;
                    let FYJ = ((FYA * AYH) * CAB) + (CAD * FYH);
                    let FYK = AWB * CJQ;
                    let FYL = CJR * AWB;
                    let FYM = FYK / AWB;
                    let FYN = FYL / AWB;
                    let FYO = BVD / BUP;
                    let FYP = parameters[1045] + CBR;
                    let FYQ = (FUJ * FYO) / FYP;
                    let FYR = ((((FUL * FYO) + (((BVF - (BUQ * FYO)) / BUP) * FUJ)) - (CBT * FYQ)) / FYP) * FUG;
                    let FYS = (((U + FYQ) - U) * FUG) + U;
                    let FYT = if 1.0f64 != 0.0 && (if FYS < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let FZR;
                    let FZS;
                    if FYT != 0.0 {
                        let FZK = ASC * FYS;
                        let FZL = -1.0000000000000002e-2f64 / FZK;
                        let FZM = (((FYR * ASC) * FZL) * W) / FZK;
                        FZR = FZL;
                        FZS = FZM;
                    } else {
                        let FZN = FYR * FYS;
                        let FZO = ((FYS * FYS) + 2.5000000000000005e-3f64).sqrt();
                        let FZP = CX * (FYS + FZO);
                        let FZQ = (FYR + ((FZN + FZN) * (AK / (AJ * FZO)))) * CX;
                        FZR = FZP;
                        FZS = FZQ;
                    }
                    let FZT = ((FXX * BIR) * BKA) / FYK;
                    let FZV = FZU * FYB;
                    let FZW = (CAB * FUI) / FZV;
                    let FZX = (FXZ * FZR) + FZW;
                    let FZY = FZT * FZX;
                    let FZZ = FYK * FYM;
                    let GAA = FZZ * FYM;
                    let GAB = FXZ / FYC;
                    let GAD = GAC * FYC;
                    let GAE = GAD * FYC;
                    let GAF = FYI / GAE;
                    let GAG = CAD * CAB;
                    let GAI = GAH * FYC;
                    let GAJ = GAI * FYF;
                    let GAK = (CAB * CAB) / GAJ;
                    let GAL = (GAB - GAF) + GAK;
                    let GAN = ((GAA * GAL) * GAM) / ADB;
                    let GAP = GAO * FXX;
                    let GAQ = (GAN * FUM) / GAP;
                    let GAR = (((((((((((FYL * FYM) + (FYN * FYK)) * FYM) + (FYN * FZZ)) * GAL) + (((((FYA - (FYE * GAB)) / FYC) - ((FYJ - ((((FYE * GAC) * FYC) + (FYE * GAD)) * GAF)) / GAE)) + (((GAG + GAG) - ((((FYE * GAH) * FYF) + (FYG * GAI)) * GAK)) / GAJ)) * GAA)) * GAM) / ADB) * FUM) + (FUO * GAN)) - ((FXY * GAO) * GAQ)) / GAP;
                    let GAS = (FSL * FZY).sqrt();
                    let GAT = (Lanes([(FSM * FZY), 0.0, 0.0, 0.0, 0.0]) + (((((((FXY * BIR) * BKA) - (FYL * FZT)) / FYK) * FZX) + ((((FYA * FZR) + (FZS * FXZ)) + ((((CAD * FUI) + (FUH * CAB)) - ((FYA * FZU) * FZW)) / FZV)) * FZT)) * FSL)) * (AK / (AJ * GAS));
                    let GAU = if GAQ > A { 1.0 } else { 0.0 };
                    let GAZ;
                    let GBA;
                    if GAU != 0.0 {
                        let GAV = FSL / GAQ;
                        let GAW = GAV.sqrt();
                        let GAX = ((Lanes([FSM, 0.0, 0.0, 0.0, 0.0]) - (GAR * GAV)) / GAQ) * (AK / (AJ * GAW));
                        let GAY = if GAS > A { 1.0 } else { 0.0 };
                        GAZ = GAW;
                        GBA = GAX;
                    } else {
                        GAZ = A;
                        GBA = BCT;
                    }
                    let GBB = -GAZ;
                    let GBD = GBB * GBC;
                    let GBE = (GBA * W) * GBC;
                    let GBG = Lanes([GBE[0], GBE[1], GBE[2], GBE[3], GBE[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (GBF * GBB)]);
                    let GBH = (((GAQ * ASZ) * BKA) * BIR) * AWB;
                    let GBJ = GBH * GBI;
                    let GBK = ((((GAR * ASZ) * BKA) * BIR) * AWB) * GBI;
                    let GBM = Lanes([GBK[0], GBK[1], GBK[2], GBK[3], GBK[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (GBL * GBH)]);
                    let GBN = ddt(64066, GBJ);
                    let GBP = GBM * GBO;
                    let GBQ = DXZ * GAS;
                    let GBR = GBQ * GBC;
                    let GBS = (GAT * DXZ) * GBC;
                    let GBT = Lanes([GBS[0], GBS[1], GBS[2], GBS[3], GBS[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (GBF * GBQ)]);
                    let GBU = U + AMZ;
                    let GBV = ((((GBU * GAQ) * ASZ) * BKA) * BIR) * AWB;
                    let GBW = (((((GAR * GBU) * ASZ) * BKA) * BIR) * AWB) * GBI;
                    let GBX = CX * (GBV * GBI);
                    let GBY = (Lanes([GBW[0], GBW[1], GBW[2], GBW[3], GBW[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (GBL * GBV)])) * CX;
                    let GCA = GBZ * ddt(64101, GBX);
                    let GCB = (GBY * GBO) * GBZ;
                    let GCC = GBZ * GBX;
                    let GCD = GBY * GBZ;
                    let GCE = U - AMZ;
                    let GCF = ((((GCE * GAQ) * ASZ) * BKA) * BIR) * AWB;
                    let GCG = (((((GAR * GCE) * ASZ) * BKA) * BIR) * AWB) * GBI;
                    let GCH = CX * (GCF * GBI);
                    let GCI = (Lanes([GCG[0], GCG[1], GCG[2], GCG[3], GCG[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (GBL * GCF)])) * CX;
                    let GCJ = GBZ * ddt(64121, GCH);
                    let GCK = (GCI * GBO) * GBZ;
                    let GCL = GBZ * GCH;
                    let GCM = GCI * GBZ;
                    FYU = GBD;
                    FYV = GBN;
                    FYW = GBR;
                    FYX = GCA;
                    FYY = GCJ;
                    FYZ = GBJ;
                    FZA = GCC;
                    FZB = GCL;
                    FZC = GBG;
                    FZD = GBP;
                    FZE = GBT;
                    FZF = GCB;
                    FZG = GCK;
                    FZH = GBM;
                    FZI = GCD;
                    FZJ = GCM;
                } else {
                    FYU = A;
                    FYV = A;
                    FYW = A;
                    FYX = A;
                    FYY = A;
                    FYZ = A;
                    FZA = A;
                    FZB = A;
                    FZC = FXA;
                    FZD = FXB;
                    FZE = FXA;
                    FZF = FXB;
                    FZG = FXB;
                    FZH = FXB;
                    FZI = FXB;
                    FZJ = FXB;
                }
                FXD = FYU;
                FXE = FYV;
                FXF = FYW;
                FXG = FYX;
                FXH = FYY;
                FXI = FYZ;
                FXJ = FZA;
                FXK = FZB;
                FXL = FZC;
                FXM = FZD;
                FXN = FZE;
                FXO = FZF;
                FXP = FZG;
                FXQ = FZH;
                FXR = FZI;
                FXS = FZJ;
            }
            let GDQ;
            let GDR;
            let GDS;
            let GDT;
            let GDU;
            let GDV;
            let GDW;
            let GDX;
            let GDY;
            let GDZ;
            let GEA;
            let GEB;
            let GEC;
            let GED;
            let GEE;
            let GEF;
            let GEG;
            let GEH;
            let GEI;
            let GEJ;
            if GCN != 0.0 {
                let GCO = AJW * V;
                let GCP = AMY * V;
                let GCQ = AND * V;
                let GCR = Lanes([0.0, GCQ[0], GCQ[1], GCQ[2], GCQ[3]]) + Lanes([(X * AMY), 0.0, 0.0, 0.0, 0.0]);
                let GCT = (AJV * V) - (GCS * V);
                let GCU = (Lanes([0.0, GCO[0], GCO[1]]) + Lanes([(X * AJV), 0.0, 0.0])) - Lanes([(X * GCS), 0.0, 0.0]);
                let GCV = staged[350] / AT;
                let GCW = if GCV >= AX { GCV } else { AX };
                let GCX = GCW.ln();
                let GCY = ((((AU * GCV) * W) / AT) * (if GCV >= AX { 1.0 } else { 0.0 })) * (AK / GCW);
                let GDA = (GCZ * V).sqrt();
                let GDB = GDA / ASZ;
                let GDC = ((X * GCZ) * (AK / (AJ * GDA))) / ASZ;
                let GDD = U / GDB;
                let GDE = ((GDC * GDD) * W) / GDB;
                let GDG = GDF * S;
                let GDH = staged[355] / GDG;
                let GDI = (((T * GDF) * GDH) * W) / GDG;
                let GEN;
                let GEO;
                if CA != 0.0 {
                    let GEL = U / GDH;
                    let GEM = ((GDI * GEL) * W) / GDH;
                    GEN = GEL;
                    GEO = GEM;
                } else {
                    GEN = A;
                    GEO = N;
                }
                let GEQ = GCT / GEP;
                let GER = GCU / GEP;
                let GES = GDB / GEP;
                let GET = GDC / GEP;
                let GEU = AYC * (U + (GES / AYB));
                let GEV = (CX * GEQ) - GEU;
                let GEW = Lanes([((GET / AYB) * AYC), 0.0, 0.0]);
                let GEX = (GER * CX) - GEW;
                let GEY = GEX * GEV;
                let GEZ = ((GEV * GEV) + (AYH * GEQ)).sqrt();
                let GFA = GEV + GEZ;
                let GFB = GEX + (((GEY + GEY) + (GER * AYH)) * (AK / (AJ * GEZ)));
                let GFC = if GEQ < A { 1.0 } else { 0.0 };
                let GFU;
                let GFV;
                if GFC != 0.0 {
                    let GFD = (GEQ - GFA) / GES;
                    let GFE = (((GER - GFB) - Lanes([(GET * GFD), 0.0, 0.0])) / GES) * GFD;
                    let GFF = (U - GFA) + (GFD * GFD);
                    let GFG = if GFF >= AX { GFF } else { AX };
                    let GFH = -(GFG.ln());
                    let GFI = ((((GFB * W) + (GFE + GFE)) * (if GFF >= AX { 1.0 } else { 0.0 })) * (AK / GFG)) * W;
                    GFU = GFH;
                    GFV = GFI;
                } else {
                    let GFJ = -GFA;
                    let GFK = rspice_limited_exp(GFJ);
                    let GFL = (GFB * W) * (rspice_limited_exp_derivative(GFJ));
                    let GFM = CX * GES;
                    let GFN = GET * CX;
                    let GFO = GFN * GFM;
                    let GFP = (((GEQ - U) + GFK) + (GFM * GFM)).sqrt();
                    let GFQ = GFP - GFM;
                    let GFR = ((((GER + GFL) + Lanes([(GFO + GFO), 0.0, 0.0])) * (AK / (AJ * GFP))) - Lanes([GFN, 0.0, 0.0])) * GFQ;
                    let GFS = ((GFQ * GFQ) + U) - GFK;
                    let GFT = (GFR + GFR) - GFL;
                    GFU = GFS;
                    GFV = GFT;
                }
                let GFW = GFU + U;
                let GFX = GFU - U;
                let GFY = GFX * GFX;
                let GFZ = GFV * GFX;
                let GGA = GFZ + GFZ;
                let GGB = (GFY + 1e0f64).sqrt();
                let GGC = (CX * (GFW + GGB)).sqrt();
                let GGD = ((GFV + (GGA * (AK / (AJ * GGB)))) * CX) * (AK / (AJ * GGC));
                let GGE = AO * GGC;
                let GGF = GGD * AO;
                let GGG = GDB / GGE;
                let GGH = Lanes([GDC, 0.0, 0.0]);
                let GGI = (U + GGG) / GDB;
                let GGJ = (((GGH - (GGF * GGG)) / GGE) - Lanes([(GDC * GGI), 0.0, 0.0])) / GDB;
                let GGK = GFU - (AO * GCX);
                let GGL = GFV - Lanes([(GCY * AO), 0.0, 0.0]);
                let GGM = GGK - GCP;
                let GGN = Lanes([GGL[0], 0.0, 0.0, 0.0, GGL[1], GGL[2]]);
                let GGO = GGN - Lanes([GCR[0], GCR[1], GCR[2], GCR[3], 0.0, GCR[4]]);
                let GGQ = ADB * GGI;
                let GGR = GGQ * GGC;
                let GGS = if GGR >= AX { GGR } else { AX };
                let GGT = ((((GGJ * ADB) * GGC) + (GGD * GGQ)) * (if GGR >= AX { 1.0 } else { 0.0 })) * (AK / GGS);
                let GGU = (GGM / GGP) - (GGS.ln());
                let GGV = (GGO / GGP) - Lanes([GGT[0], 0.0, 0.0, 0.0, GGT[1], GGT[2]]);
                let GGW = GGU + BAH;
                let GGX = ((GGU * GGW) + BAJ).sqrt();
                let GGY = CX * ((GGU - BAG) - GGX);
                let GGZ = (GGV - (((GGV * GGW) + (GGV * GGU)) * (AK / (AJ * GGX)))) * CX;
                let GHA = if GGY <= -6.8e1f64 { 1.0 } else { 0.0 };
                let GJN;
                let GJO;
                if GHA != 0.0 {
                    let GHB = if GGY < -1.1e2f64 { 1.0 } else { 0.0 };
                    let GJS;
                    let GJT;
                    if GHB != 0.0 {
                        GJS = GJQ;
                        GJT = GDP;
                    } else {
                        let GJR = if GGY > -9e1f64 { 1.0 } else { 0.0 };
                        let GKR;
                        let GKS;
                        if GJR != 0.0 {
                            let GKE = rspice_limited_exp(GGY);
                            let GKF = GGZ * (rspice_limited_exp_derivative(GGY));
                            GKR = GKE;
                            GKS = GKF;
                        } else {
                            let GKH = (GGY - GKG) / BDI;
                            let GKI = GGZ / BDI;
                            let GKJ = GKH * GKH;
                            let GKK = GKI * GKH;
                            let GKL = GKK + GKK;
                            let GKM = BDO - GKJ;
                            let GKN = 9.375e-1f64 - (GKJ * GKM);
                            let GKO = GKG + (BDI * ((7.8125e-2f64 + (CX * GKH)) + (GKJ * GKN)));
                            let GKP = rspice_limited_exp(GKO);
                            let GKQ = (((GKI * CX) + ((GKL * GKN) + ((((GKL * GKM) + ((GKL * W) * GKJ)) * W) * GKJ))) * BDI) * (rspice_limited_exp_derivative(GKO));
                            GKR = GKP;
                            GKS = GKQ;
                        }
                        GJS = GKR;
                        GJT = GKS;
                    }
                    let GJU = AO * GGI;
                    let GJV = GJS * AO;
                    let GJW = GGJ * GJV;
                    let GJX = (GJV * GGI) + GGE;
                    let GJY = GJU * GJX;
                    let GJZ = (GGJ * AO) * GJX;
                    let GKA = if GJY >= AX { GJY } else { AX };
                    let GKB = ((U + GGM) - (GGP * GGY)) - (GGP * (GKA.ln()));
                    let GKC = GJS * GKB;
                    let GKD = (GJT * GKB) + (((GGO - (GGZ * GGP)) - ((((Lanes([GJZ[0], 0.0, 0.0, 0.0, GJZ[1], GJZ[2]]) + (((((GJT * AO) * GGI) + Lanes([GJW[0], 0.0, 0.0, 0.0, GJW[1], GJW[2]])) + Lanes([GGF[0], 0.0, 0.0, 0.0, GGF[1], GGF[2]])) * GJU)) * (if GJY >= AX { 1.0 } else { 0.0 })) * (AK / GKA)) * GGP)) * GJS);
                    GJN = GKC;
                    GJO = GKD;
                } else {
                    let GHC = rspice_limited_exp(GGY);
                    let GHD = GGZ * (rspice_limited_exp_derivative(GGY));
                    let GHE = U / GGC;
                    let GHF = AO * GHC;
                    let GHG = GHD * AO;
                    let GHH = GHF * GGI;
                    let GHI = GGJ * GHF;
                    let GHJ = (GHG * GGI) + Lanes([GHI[0], 0.0, 0.0, 0.0, GHI[1], GHI[2]]);
                    let GHK = GHH + GGE;
                    let GHL = Lanes([GGF[0], 0.0, 0.0, 0.0, GGF[1], GGF[2]]);
                    let GHM = GHH * GHK;
                    let GHN = if GHM >= AX { GHM } else { AX };
                    let GHO = GGP / GHC;
                    let GHP = GGI + GHE;
                    let GHQ = GGJ + (((GGD * GHE) * W) / GGC);
                    let GHR = GGP * GHP;
                    let GHS = GHQ * GGP;
                    let GHT = GGJ * GHC;
                    let GHU = (GGI * GHC) + GGC;
                    let GHV = Lanes([GGD[0], 0.0, 0.0, 0.0, GGD[1], GGD[2]]);
                    let GHW = GHR / GHU;
                    let GHX = Lanes([GHS[0], 0.0, 0.0, 0.0, GHS[1], GHS[2]]);
                    let GHY = (AO + GHO) + GHW;
                    let GHZ = ((GHF + (GGP * (GHN.ln()))) - GGM) / GHY;
                    let GIA = GHC - GHZ;
                    let GIB = GHD - ((((GHG + (((((GHJ * GHK) + ((GHJ + GHL) * GHH)) * (if GHM >= AX { 1.0 } else { 0.0 })) * (AK / GHN)) * GGP)) - GGO) - (((((GHD * GHO) * W) / GHC) + ((GHX - (((Lanes([GHT[0], 0.0, 0.0, 0.0, GHT[1], GHT[2]]) + (GHD * GGI)) + GHV) * GHW)) / GHU)) * GHZ)) / GHY);
                    let GIC = AO * GIA;
                    let GID = GIB * AO;
                    let GIE = GIC * GGI;
                    let GIF = GGJ * GIC;
                    let GIG = (GID * GGI) + Lanes([GIF[0], 0.0, 0.0, 0.0, GIF[1], GIF[2]]);
                    let GIH = GIE + GGE;
                    let GII = GIE * GIH;
                    let GIJ = if GII >= AX { GII } else { AX };
                    let GIK = (GIC + (GGP * (GIJ.ln()))) - GGM;
                    let GIL = (GID + (((((GIG * GIH) + ((GIG + GHL) * GIE)) * (if GII >= AX { 1.0 } else { 0.0 })) * (AK / GIJ)) * GGP)) - GGO;
                    let GIM = GGP / GIA;
                    let GIN = GGJ * GIA;
                    let GIO = (GGI * GIA) + GGC;
                    let GIP = (Lanes([GIN[0], 0.0, 0.0, 0.0, GIN[1], GIN[2]]) + (GIB * GGI)) + GHV;
                    let GIQ = GHR / GIO;
                    let GIR = (AO + GIM) + GIQ;
                    let GIS = (((GIB * GIM) * W) / GIA) + ((GHX - (GIP * GIQ)) / GIO);
                    let GIT = GHP / GIO;
                    let GIU = (Lanes([GHQ[0], 0.0, 0.0, 0.0, GHQ[1], GHQ[2]]) - (GIP * GIT)) / GIO;
                    let GIV = GGP * GIT;
                    let GIW = -GGP;
                    let GIX = U / GIA;
                    let GIY = (((GIB * GIX) * W) / GIA) * GIX;
                    let GIZ = GGC * GGC;
                    let GJA = GGD * GGC;
                    let GJB = GIZ * GGC;
                    let GJC = GJB * GIO;
                    let GJD = (((GJA + GJA) * GGC) + (GGD * GIZ)) * GIO;
                    let GJE = GGP / GJC;
                    let GJF = ((GIW * (GIX * GIX)) - GJE) - (GIV * GIT);
                    let GJG = GIK / GIR;
                    let GJH = AO * GIR;
                    let GJI = GJH * GIR;
                    let GJJ = (GIK * GJF) / GJI;
                    let GJK = U + GJJ;
                    let GJL = GIA - (GJG * GJK);
                    let GJM = GIB - ((((GIL - (GIS * GJG)) / GIR) * GJK) + (((((GIL * GJF) + (((((GIY + GIY) * GIW) - ((((Lanes([GJD[0], 0.0, 0.0, 0.0, GJD[1], GJD[2]]) + (GIP * GJB)) * GJE) * W) / GJC)) - (((GIU * GGP) * GIT) + (GIU * GIV))) * GIK)) - ((((GIS * AO) * GIR) + (GIS * GJH)) * GJJ)) / GJI) * GJG));
                    GJN = GJL;
                    GJO = GJM;
                }
                let GJP = if 0.0f64 != 0.0 && (if GFU < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GKZ;
                let GLA;
                if GJP != 0.0 {
                    let GKT = ASC * GFU;
                    let GKU = -4e0f64 / GKT;
                    let GKV = (((GFV * ASC) * GKU) * W) / GKT;
                    GKZ = GKU;
                    GLA = GKV;
                } else {
                    let GKW = (GFY + 1e0f64).sqrt();
                    let GKX = CX * (GFW + GKW);
                    let GKY = (GFV + (GGA * (AK / (AJ * GKW)))) * CX;
                    GKZ = GKX;
                    GLA = GKY;
                }
                let GLB = GKZ.sqrt();
                let GLC = GLA * (AK / (AJ * GLB));
                let GLD = AO * GJN;
                let GLE = GJO * AO;
                let GLF = GFU - GLD;
                let GLG = Lanes([GFV[0], 0.0, 0.0, 0.0, GFV[1], GFV[2]]);
                let GLH = GLG - GLE;
                let GLI = if 0.0f64 != 0.0 && (if GLF < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GLR;
                let GLS;
                if GLI != 0.0 {
                    let GLJ = ASC * GLF;
                    let GLK = -4e0f64 / GLJ;
                    let GLL = (((GLH * ASC) * GLK) * W) / GLJ;
                    GLR = GLK;
                    GLS = GLL;
                } else {
                    let GLM = GLF - U;
                    let GLN = GLH * GLM;
                    let GLO = ((GLM * GLM) + 1e0f64).sqrt();
                    let GLP = CX * ((GLF + U) + GLO);
                    let GLQ = (GLH + ((GLN + GLN) * (AK / (AJ * GLO)))) * CX;
                    GLR = GLP;
                    GLS = GLQ;
                }
                let GLT = GLR.sqrt();
                let GLU = GLB + GLT;
                let GLV = GDB / GLU;
                let GLW = Lanes([GDC, 0.0, 0.0, 0.0, 0.0, 0.0]);
                let GLX = (GLW - ((Lanes([GLC[0], 0.0, 0.0, 0.0, GLC[1], GLC[2]]) + (GLS * (AK / (AJ * GLT)))) * GLV)) / GLU;
                let GLY = U + GLV;
                let GLZ = GCT - GFU;
                let GMA = GCU - GFV;
                let GMB = GLY - U;
                let GMC = GLZ - (GLD * GMB);
                let GMD = Lanes([GMA[0], 0.0, 0.0, 0.0, GMA[1], GMA[2]]);
                let GME = S * GMC;
                let GMF = Lanes([(T * GMC), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((GMD - ((GLE * GMB) + (GLX * GLD))) * S);
                let GMG = if 1.0f64 != 0.0 && (if GME < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GMO;
                let GMP;
                if GMG != 0.0 {
                    let GMH = ASC * GME;
                    let GMI = -1.0000000000000002e-2f64 / GMH;
                    let GMJ = (((GMF * ASC) * GMI) * W) / GMH;
                    GMO = GMI;
                    GMP = GMJ;
                } else {
                    let GMK = GMF * GME;
                    let GML = ((GME * GME) + 2.5000000000000005e-3f64).sqrt();
                    let GMM = CX * (GME + GML);
                    let GMN = (GMF + ((GMK + GMK) * (AK / (AJ * GML)))) * CX;
                    GMO = GMM;
                    GMP = GMN;
                }
                let GMQ = AO * GLY;
                let GMR = GMQ * S;
                let GMS = BFX * (GMO + (BFW * (GMR * GJN)));
                let GMT = ARP * ANR;
                let GMU = ANS * ARB;
                let GMV = ARA + (ARB * ANR);
                let GMW = Lanes([ARO[0], ARO[1], 0.0, ARO[2], ARO[3]]) + (Lanes([GMT[0], GMT[1], 0.0, GMT[2], GMT[3]]) + Lanes([0.0, GMU[0], GMU[1], GMU[2], GMU[3]]));
                let GMX = GMS.powf(FZ);
                let GMY = GMW * GMX;
                let GMZ = Lanes([GMY[0], GMY[1], GMY[2], GMY[3], 0.0, GMY[4]]) + (((((GMP + ((((((GLX * AO) * S) + Lanes([(T * GMQ), 0.0, 0.0, 0.0, 0.0, 0.0])) * GJN) + (GJO * GMR)) * BFW)) * BFX) * (FZ * (GMS.powf(BGI)))) + Lanes([(GA * (GMX * (GMS.ln()))), 0.0, 0.0, 0.0, 0.0, 0.0])) * GMV);
                let GNA = U + (GMV * GMX);
                let GNB = if 0.0f64 != 0.0 && (if GNA < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GNK;
                let GNL;
                if GNB != 0.0 {
                    let GNC = ASC * GNA;
                    let GND = -2.25e-6f64 / GNC;
                    let GNE = (((GMZ * ASC) * GND) * W) / GNC;
                    GNK = GND;
                    GNL = GNE;
                } else {
                    let GNF = GNA - U;
                    let GNG = GMZ * GNF;
                    let GNH = ((GNF * GNF) + 5.625e-7f64).sqrt();
                    let GNI = CX * ((GNA + U) + GNH);
                    let GNJ = (GMZ + ((GNG + GNG) * (AK / (AJ * GNH)))) * CX;
                    GNK = GNI;
                    GNL = GNJ;
                }
                let GNM = ARF / GNK;
                let GNN = Lanes([ART[0], ART[1], 0.0, ART[2], 0.0, ART[3]]);
                let GNP = JD * GNO;
                let GNQ = JE * GNO;
                let GNR = (GNM * S) / GNP;
                let GNS = (((((GNN - (GNL * GNM)) / GNK) * S) + Lanes([(T * GNM), 0.0, 0.0, 0.0, 0.0, 0.0])) - Lanes([(GNQ * GNR), 0.0, 0.0, 0.0, 0.0, 0.0])) / GNP;
                let GNT = GJO * GJN;
                let GNU = (GJN * GJN) + GJN;
                let GNV = U + GJN;
                let GNW = U + (GNR * GNV);
                let GNX = (GNR * GNU) / GNW;
                let GNY = AO * GNX;
                let GNZ = ((((GNS * GNU) + (((GNT + GNT) + GJO) * GNR)) - (((GNS * GNV) + (GJO * GNR)) * GNX)) / GNW) * AO;
                let GOA = GNY * GLY;
                let GOB = GOA * GDD;
                let GOC = (((GNZ * GLY) + (GLX * GNY)) * GDD) + Lanes([(GDE * GOA), 0.0, 0.0, 0.0, 0.0, 0.0]);
                let GOD = GDB / GMB;
                let GOE = GOB + GOD;
                let GOF = GOB * GOE;
                let GOG = if GOF >= AX { GOF } else { AX };
                let GOH = GGK - (GNY + (GOG.ln()));
                let GOI = (GOH * S) - AMY;
                let GOJ = Lanes([0.0, AND[0], AND[1], AND[2], 0.0, AND[3]]);
                let GOK = (((GGN - (GNZ + ((((GOC * GOE) + ((GOC + ((GLW - (GLX * GOD)) / GMB)) * GOB)) * (if GOF >= AX { 1.0 } else { 0.0 })) * (AK / GOG)))) * S) + Lanes([(T * GOH), 0.0, 0.0, 0.0, 0.0, 0.0])) - GOJ;
                let GOL = if 1.0f64 != 0.0 && (if GOI < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GOT;
                let GOU;
                if GOL != 0.0 {
                    let GOM = ASC * GOI;
                    let GON = -1e-6f64 / GOM;
                    let GOO = (((GOK * ASC) * GON) * W) / GOM;
                    GOT = GON;
                    GOU = GOO;
                } else {
                    let GOP = GOK * GOI;
                    let GOQ = ((GOI * GOI) + 2.5e-7f64).sqrt();
                    let GOR = CX * (GOI + GOQ);
                    let GOS = (GOK + ((GOP + GOP) * (AK / (AJ * GOQ)))) * CX;
                    GOT = GOR;
                    GOU = GOS;
                }
                let GPP;
                let GPQ;
                if GOV != 0.0 {
                    GPP = GOW;
                    GPQ = GDP;
                } else {
                    let GOX = (BSV * ASR).sqrt();
                    let GOY = AWB + GOX;
                    let GOZ = AWB / GOY;
                    let GPA = ((((ASS * BSV) * (AK / (AJ * GOX))) * GOZ) * W) / GOY;
                    let GPC = GPA * GPB;
                    let GPE = GPD * GOZ;
                    let GPF = GPE * GJN;
                    let GPG = (GPA * GPD) * GJN;
                    let GPH = ATO * GPF;
                    let GPJ = U + (GPI * AOJ);
                    let GPK = ((GPB * GOZ) - (GPF * ATN)) / GPJ;
                    let GPL = (AOK * GPI) * GPK;
                    let GPM = ((Lanes([GPC[0], GPC[1], 0.0, GPC[2], 0.0, GPC[3]]) - (((Lanes([GPG[0], GPG[1], 0.0, GPG[2], 0.0, GPG[3]]) + (GJO * GPE)) * ATN) + Lanes([GPH[0], GPH[1], 0.0, GPH[2], 0.0, GPH[3]]))) - Lanes([0.0, GPL[0], 0.0, GPL[1], 0.0, GPL[2]])) / GPJ;
                    let GPN = U + GPK;
                    let GPO = if 0.0f64 != 0.0 && (if GPN < -1.25e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GRF;
                    let GRG;
                    if GPO != 0.0 {
                        let GQX = ASC * GPN;
                        let GQY = -2.5e-7f64 / GQX;
                        let GQZ = (((GPM * ASC) * GQY) * W) / GQX;
                        GRF = GQY;
                        GRG = GQZ;
                    } else {
                        let GRA = GPN - BHW;
                        let GRB = GPM * GRA;
                        let GRC = ((GRA * GRA) + 6.25e-8f64).sqrt();
                        let GRD = CX * ((GPN + BHW) + GRC);
                        let GRE = (GPM + ((GRB + GRB) * (AK / (AJ * GRC)))) * CX;
                        GRF = GRD;
                        GRG = GRE;
                    }
                    GPP = GRF;
                    GPQ = GRG;
                }
                let GPR = GOT / GPP;
                let GPS = (GOU - (GPQ * GPR)) / GPP;
                let GPT = ANE / GPR;
                let GPU = GPT + BB;
                let GPV = GPU.powf(BUU);
                let GPW = U + GPV;
                let GPX = GPW.powf(BUZ);
                let GPY = ANG * GPX;
                let GPZ = ANH * GPX;
                let GQA = Lanes([0.0, GPZ[0], GPZ[1], GPZ[2], 0.0, GPZ[3]]) + (((((((Lanes([0.0, ANF[0], 0.0, ANF[1], 0.0, ANF[2]]) - (GPS * GPT)) / GPR) * (BUU * (GPU.powf(BUX)))) + Lanes([(BUV * (GPV * (GPU.ln()))), 0.0, 0.0, 0.0, 0.0, 0.0])) * (BUZ * (GPW.powf(BVC)))) + Lanes([(BVA * (GPX * (GPW.ln()))), 0.0, 0.0, 0.0, 0.0, 0.0])) * ANG);
                let GQB = GPY + AMY;
                let GQC = (GFY + 1e0f64).sqrt();
                let GQD = (CX * (GFW + GQC)).sqrt();
                let GQE = ((GFV + (GGA * (AK / (AJ * GQC)))) * CX) * (AK / (AJ * GQD));
                let GQF = AO * GQD;
                let GQG = GQE * AO;
                let GQH = GDB / GQF;
                let GQI = (U + GQH) / GDB;
                let GQJ = (((GGH - (GQG * GQH)) / GQF) - Lanes([(GDC * GQI), 0.0, 0.0])) / GDB;
                let GQK = GGK - (GQB * V);
                let GQL = GGN - (((GQA + GOJ) * V) + Lanes([(X * GQB), 0.0, 0.0, 0.0, 0.0, 0.0]));
                let GQM = ADB * GQI;
                let GQN = GQM * GQD;
                let GQO = if GQN >= AX { GQN } else { AX };
                let GQP = ((((GQJ * ADB) * GQD) + (GQE * GQM)) * (if GQN >= AX { 1.0 } else { 0.0 })) * (AK / GQO);
                let GQQ = (GQK / GGP) - (GQO.ln());
                let GQR = (GQL / GGP) - Lanes([GQP[0], 0.0, 0.0, 0.0, GQP[1], GQP[2]]);
                let GQS = GQQ + BAH;
                let GQT = ((GQQ * GQS) + BAJ).sqrt();
                let GQU = CX * ((GQQ - BAG) - GQT);
                let GQV = (GQR - (((GQR * GQS) + (GQR * GQQ)) * (AK / (AJ * GQT)))) * CX;
                let GQW = if GQU <= -6.8e1f64 { 1.0 } else { 0.0 };
                let GTT;
                let GTU;
                if GQW != 0.0 {
                    let GRH = if GQU < -1.1e2f64 { 1.0 } else { 0.0 };
                    let GUA;
                    let GUB;
                    if GRH != 0.0 {
                        GUA = GTY;
                        GUB = GDP;
                    } else {
                        let GTZ = if GQU > -9e1f64 { 1.0 } else { 0.0 };
                        let GUZ;
                        let GVA;
                        if GTZ != 0.0 {
                            let GUM = rspice_limited_exp(GQU);
                            let GUN = GQV * (rspice_limited_exp_derivative(GQU));
                            GUZ = GUM;
                            GVA = GUN;
                        } else {
                            let GUP = (GQU - GUO) / BDI;
                            let GUQ = GQV / BDI;
                            let GUR = GUP * GUP;
                            let GUS = GUQ * GUP;
                            let GUT = GUS + GUS;
                            let GUU = BDO - GUR;
                            let GUV = 9.375e-1f64 - (GUR * GUU);
                            let GUW = GUO + (BDI * ((7.8125e-2f64 + (CX * GUP)) + (GUR * GUV)));
                            let GUX = rspice_limited_exp(GUW);
                            let GUY = (((GUQ * CX) + ((GUT * GUV) + ((((GUT * GUU) + ((GUT * W) * GUR)) * W) * GUR))) * BDI) * (rspice_limited_exp_derivative(GUW));
                            GUZ = GUX;
                            GVA = GUY;
                        }
                        GUA = GUZ;
                        GUB = GVA;
                    }
                    let GUC = AO * GQI;
                    let GUD = GUA * AO;
                    let GUE = GQJ * GUD;
                    let GUF = (GUD * GQI) + GQF;
                    let GUG = GUC * GUF;
                    let GUH = (GQJ * AO) * GUF;
                    let GUI = if GUG >= AX { GUG } else { AX };
                    let GUJ = ((U + GQK) - (GGP * GQU)) - (GGP * (GUI.ln()));
                    let GUK = GUA * GUJ;
                    let GUL = (GUB * GUJ) + (((GQL - (GQV * GGP)) - ((((Lanes([GUH[0], 0.0, 0.0, 0.0, GUH[1], GUH[2]]) + (((((GUB * AO) * GQI) + Lanes([GUE[0], 0.0, 0.0, 0.0, GUE[1], GUE[2]])) + Lanes([GQG[0], 0.0, 0.0, 0.0, GQG[1], GQG[2]])) * GUC)) * (if GUG >= AX { 1.0 } else { 0.0 })) * (AK / GUI)) * GGP)) * GUA);
                    GTT = GUK;
                    GTU = GUL;
                } else {
                    let GRI = rspice_limited_exp(GQU);
                    let GRJ = GQV * (rspice_limited_exp_derivative(GQU));
                    let GRK = U / GQD;
                    let GRL = AO * GRI;
                    let GRM = GRJ * AO;
                    let GRN = GRL * GQI;
                    let GRO = GQJ * GRL;
                    let GRP = (GRM * GQI) + Lanes([GRO[0], 0.0, 0.0, 0.0, GRO[1], GRO[2]]);
                    let GRQ = GRN + GQF;
                    let GRR = Lanes([GQG[0], 0.0, 0.0, 0.0, GQG[1], GQG[2]]);
                    let GRS = GRN * GRQ;
                    let GRT = if GRS >= AX { GRS } else { AX };
                    let GRU = GGP / GRI;
                    let GRV = GQI + GRK;
                    let GRW = GQJ + (((GQE * GRK) * W) / GQD);
                    let GRX = GGP * GRV;
                    let GRY = GRW * GGP;
                    let GRZ = GQJ * GRI;
                    let GSA = (GQI * GRI) + GQD;
                    let GSB = Lanes([GQE[0], 0.0, 0.0, 0.0, GQE[1], GQE[2]]);
                    let GSC = GRX / GSA;
                    let GSD = Lanes([GRY[0], 0.0, 0.0, 0.0, GRY[1], GRY[2]]);
                    let GSE = (AO + GRU) + GSC;
                    let GSF = ((GRL + (GGP * (GRT.ln()))) - GQK) / GSE;
                    let GSG = GRI - GSF;
                    let GSH = GRJ - ((((GRM + (((((GRP * GRQ) + ((GRP + GRR) * GRN)) * (if GRS >= AX { 1.0 } else { 0.0 })) * (AK / GRT)) * GGP)) - GQL) - (((((GRJ * GRU) * W) / GRI) + ((GSD - (((Lanes([GRZ[0], 0.0, 0.0, 0.0, GRZ[1], GRZ[2]]) + (GRJ * GQI)) + GSB) * GSC)) / GSA)) * GSF)) / GSE);
                    let GSI = AO * GSG;
                    let GSJ = GSH * AO;
                    let GSK = GSI * GQI;
                    let GSL = GQJ * GSI;
                    let GSM = (GSJ * GQI) + Lanes([GSL[0], 0.0, 0.0, 0.0, GSL[1], GSL[2]]);
                    let GSN = GSK + GQF;
                    let GSO = GSK * GSN;
                    let GSP = if GSO >= AX { GSO } else { AX };
                    let GSQ = (GSI + (GGP * (GSP.ln()))) - GQK;
                    let GSR = (GSJ + (((((GSM * GSN) + ((GSM + GRR) * GSK)) * (if GSO >= AX { 1.0 } else { 0.0 })) * (AK / GSP)) * GGP)) - GQL;
                    let GSS = GGP / GSG;
                    let GST = GQJ * GSG;
                    let GSU = (GQI * GSG) + GQD;
                    let GSV = (Lanes([GST[0], 0.0, 0.0, 0.0, GST[1], GST[2]]) + (GSH * GQI)) + GSB;
                    let GSW = GRX / GSU;
                    let GSX = (AO + GSS) + GSW;
                    let GSY = (((GSH * GSS) * W) / GSG) + ((GSD - (GSV * GSW)) / GSU);
                    let GSZ = GRV / GSU;
                    let GTA = (Lanes([GRW[0], 0.0, 0.0, 0.0, GRW[1], GRW[2]]) - (GSV * GSZ)) / GSU;
                    let GTB = GGP * GSZ;
                    let GTC = -GGP;
                    let GTD = U / GSG;
                    let GTE = (((GSH * GTD) * W) / GSG) * GTD;
                    let GTF = GQD * GQD;
                    let GTG = GQE * GQD;
                    let GTH = GTF * GQD;
                    let GTI = GTH * GSU;
                    let GTJ = (((GTG + GTG) * GQD) + (GQE * GTF)) * GSU;
                    let GTK = GGP / GTI;
                    let GTL = ((GTC * (GTD * GTD)) - GTK) - (GTB * GSZ);
                    let GTM = GSQ / GSX;
                    let GTN = AO * GSX;
                    let GTO = GTN * GSX;
                    let GTP = (GSQ * GTL) / GTO;
                    let GTQ = U + GTP;
                    let GTR = GSG - (GTM * GTQ);
                    let GTS = GSH - ((((GSR - (GSY * GTM)) / GSX) * GTQ) + (((((GSR * GTL) + (((((GTE + GTE) * GTC) - ((((Lanes([GTJ[0], 0.0, 0.0, 0.0, GTJ[1], GTJ[2]]) + (GSV * GTH)) * GTK) * W) / GTI)) - (((GTA * GGP) * GSZ) + (GTA * GTB))) * GSQ)) - ((((GSY * AO) * GSX) + (GSY * GTN)) * GTP)) / GTO) * GTM));
                    GTT = GTR;
                    GTU = GTS;
                }
                let GTV = (GLG - GJO) - GTU;
                let GTW = ((GFU - GJN) - GTT) - U;
                let GTX = if 0.0f64 != 0.0 && (if GTW < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GVJ;
                let GVK;
                if GTX != 0.0 {
                    let GVB = ASC * GTW;
                    let GVC = -4e0f64 / GVB;
                    let GVD = (((GTV * ASC) * GVC) * W) / GVB;
                    GVJ = GVC;
                    GVK = GVD;
                } else {
                    let GVE = GTW - U;
                    let GVF = GTV * GVE;
                    let GVG = ((GVE * GVE) + 1e0f64).sqrt();
                    let GVH = CX * ((GTW + U) + GVG);
                    let GVI = (GTV + ((GVF + GVF) * (AK / (AJ * GVG)))) * CX;
                    GVJ = GVH;
                    GVK = GVI;
                }
                let GVL = GVJ.sqrt();
                let GVM = GVK * (AK / (AJ * GVL));
                let GVN = GQD + GVL;
                let GVO = GDB / GVN;
                let GVP = (GLW - ((Lanes([GQE[0], 0.0, 0.0, 0.0, GQE[1], GQE[2]]) + GVM) * GVO)) / GVN;
                let GVQ = GEP + GVO;
                let GVS = GVR * GVL;
                let GVT = ((GVM * GVR) * GDD) + Lanes([(GDE * GVS), 0.0, 0.0, 0.0, 0.0, 0.0]);
                let GVU = CX + (GVS * GDD);
                let GVV = GVT * GVU;
                let GVW = GJN + GTT;
                let GVX = GJO + GTU;
                let GVY = GVQ * GVW;
                let GVZ = ((GVU * GVU) + (GVY * GEN)).sqrt();
                let GWA = GVU + GVZ;
                let GWB = GVQ / GWA;
                let GWC = (GVP - ((GVT + (((GVV + GVV) + ((((GVP * GVW) + (GVX * GVQ)) * GEN) + Lanes([(GEO * GVY), 0.0, 0.0, 0.0, 0.0, 0.0]))) * (AK / (AJ * GVZ)))) * GWB)) / GWA;
                let GWD = GWB - U;
                let GWE = GLZ - (GLD * GWD);
                let GWF = S * GWE;
                let GWG = Lanes([(T * GWE), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((GMD - ((GLE * GWD) + (GWC * GLD))) * S);
                let GWH = if 1.0f64 != 0.0 && (if GWF < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GWP;
                let GWQ;
                if GWH != 0.0 {
                    let GWI = ASC * GWF;
                    let GWJ = -1.0000000000000002e-2f64 / GWI;
                    let GWK = (((GWG * ASC) * GWJ) * W) / GWI;
                    GWP = GWJ;
                    GWQ = GWK;
                } else {
                    let GWL = GWG * GWF;
                    let GWM = ((GWF * GWF) + 2.5000000000000005e-3f64).sqrt();
                    let GWN = CX * (GWF + GWM);
                    let GWO = (GWG + ((GWL + GWL) * (AK / (AJ * GWM)))) * CX;
                    GWP = GWN;
                    GWQ = GWO;
                }
                let GWR = AO * GTT;
                let GWS = GLZ - (GWR * GWD);
                let GWT = S * GWS;
                let GWU = Lanes([(T * GWS), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((GMD - (((GTU * AO) * GWD) + (GWC * GWR))) * S);
                let GWV = if 1.0f64 != 0.0 && (if GWT < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GXD;
                let GXE;
                if GWV != 0.0 {
                    let GWW = ASC * GWT;
                    let GWX = -1.0000000000000002e-2f64 / GWW;
                    let GWY = (((GWU * ASC) * GWX) * W) / GWW;
                    GXD = GWX;
                    GXE = GWY;
                } else {
                    let GWZ = GWU * GWT;
                    let GXA = ((GWT * GWT) + 2.5000000000000005e-3f64).sqrt();
                    let GXB = CX * (GWT + GXA);
                    let GXC = (GWU + ((GWZ + GWZ) * (AK / (AJ * GXA)))) * CX;
                    GXD = GXB;
                    GXE = GXC;
                }
                let GXF = GWB * S;
                let GXG = BFX * ((CX * (GWP + GXD)) + (BFW * (GXF * GVW)));
                let GXH = (((GWQ + GXE) * CX) + (((((GWC * S) + Lanes([(T * GWB), 0.0, 0.0, 0.0, 0.0, 0.0])) * GVW) + (GVX * GXF)) * BFW)) * BFX;
                let GXJ = (GCT + (GXI * V)) / GEP;
                let GXK = (GCU + Lanes([(X * GXI), 0.0, 0.0])) / GEP;
                let GXL = (CX * GXJ) - GEU;
                let GXM = (GXK * CX) - GEW;
                let GXN = GXM * GXL;
                let GXO = ((GXL * GXL) + (AYH * GXJ)).sqrt();
                let GXP = GXL + GXO;
                let GXQ = GXM + (((GXN + GXN) + (GXK * AYH)) * (AK / (AJ * GXO)));
                let GXR = if GXJ < A { 1.0 } else { 0.0 };
                let GYJ;
                let GYK;
                if GXR != 0.0 {
                    let GXS = (GXJ - GXP) / GES;
                    let GXT = (((GXK - GXQ) - Lanes([(GET * GXS), 0.0, 0.0])) / GES) * GXS;
                    let GXU = (U - GXP) + (GXS * GXS);
                    let GXV = if GXU >= AX { GXU } else { AX };
                    let GXW = -(GXV.ln());
                    let GXX = ((((GXQ * W) + (GXT + GXT)) * (if GXU >= AX { 1.0 } else { 0.0 })) * (AK / GXV)) * W;
                    GYJ = GXW;
                    GYK = GXX;
                } else {
                    let GXY = -GXP;
                    let GXZ = rspice_limited_exp(GXY);
                    let GYA = (GXQ * W) * (rspice_limited_exp_derivative(GXY));
                    let GYB = CX * GES;
                    let GYC = GET * CX;
                    let GYD = GYC * GYB;
                    let GYE = (((GXJ - U) + GXZ) + (GYB * GYB)).sqrt();
                    let GYF = GYE - GYB;
                    let GYG = ((((GXK + GYA) + Lanes([(GYD + GYD), 0.0, 0.0])) * (AK / (AJ * GYE))) - Lanes([GYC, 0.0, 0.0])) * GYF;
                    let GYH = ((GYF * GYF) + U) - GXZ;
                    let GYI = (GYG + GYG) - GYA;
                    GYJ = GYH;
                    GYK = GYI;
                }
                let GYL = GXG.powf(FZ);
                let GYM = GMW * GYL;
                let GYN = Lanes([GYM[0], GYM[1], GYM[2], GYM[3], 0.0, GYM[4]]) + (((GXH * (FZ * (GXG.powf(BGI)))) + Lanes([(GA * (GYL * (GXG.ln()))), 0.0, 0.0, 0.0, 0.0, 0.0])) * GMV);
                let GYO = U + (GMV * GYL);
                let GYP = if 0.0f64 != 0.0 && (if GYO < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GYY;
                let GYZ;
                if GYP != 0.0 {
                    let GYQ = ASC * GYO;
                    let GYR = -2.25e-6f64 / GYQ;
                    let GYS = (((GYN * ASC) * GYR) * W) / GYQ;
                    GYY = GYR;
                    GYZ = GYS;
                } else {
                    let GYT = GYO - U;
                    let GYU = GYN * GYT;
                    let GYV = ((GYT * GYT) + 5.625e-7f64).sqrt();
                    let GYW = CX * ((GYO + U) + GYV);
                    let GYX = (GYN + ((GYU + GYU) * (AK / (AJ * GYV)))) * CX;
                    GYY = GYW;
                    GYZ = GYX;
                }
                let GZA = ARF / GYY;
                let GZB = (GNN - (GYZ * GZA)) / GYY;
                let GZC = AO * GZA;
                let GZD = (GZC * S) / GNP;
                let GZE = GJN - GTT;
                let GZF = GZD * GZE;
                let GZG = ((((((GZB * AO) * S) + Lanes([(T * GZC), 0.0, 0.0, 0.0, 0.0, 0.0])) - Lanes([(GNQ * GZD), 0.0, 0.0, 0.0, 0.0, 0.0])) / GNP) * GZE) + ((GJO - GTU) * GZD);
                let GZH = AO * GZF;
                let GZI = (U + (GZH * GZF)).sqrt();
                let GZJ = CX * (U + GZI);
                let GZK = ((((GZG * AO) * GZF) + (GZG * GZH)) * (AK / (AJ * GZI))) * CX;
                let GZL = (AO * JD) / GZA;
                let GZM = GPR + (GZL * GNO);
                let GZN = GPS + (((Lanes([(JE * AO), 0.0, 0.0, 0.0, 0.0, 0.0]) - (GZB * GZL)) / GZA) * GNO);
                let GZO = ANG - GPY;
                let GZP = Lanes([0.0, ANH[0], ANH[1], ANH[2], 0.0, ANH[3]]) - GQA;
                let GZQ = Lanes([GCU[0], 0.0, 0.0, GCU[1], GCU[2]]);
                let GZR = Lanes([GYK[0], 0.0, 0.0, GYK[1], GYK[2]]);
                GDQ = GZO;
                GDR = GZM;
                GDS = GCT;
                GDT = GYJ;
                GDU = GJN;
                GDV = GTT;
                GDW = GEN;
                GDX = GPP;
                GDY = GZJ;
                GDZ = GWB;
                GEA = GZP;
                GEB = GZN;
                GEC = GZQ;
                GED = GZR;
                GEE = GJO;
                GEF = GTU;
                GEG = GEO;
                GEH = GPQ;
                GEI = GZK;
                GEJ = GWC;
            } else {
                let GDJ = Lanes([CDJ[0], CDJ[1], 0.0, CDJ[2], CDJ[3], CDJ[4]]);
                let GDK = Lanes([CEX[0], CEX[1], 0.0, CEX[2], CEX[3], CEX[4]]);
                let GDL = Lanes([BCQ[0], BCQ[1], 0.0, BCQ[2], BCQ[3], BCQ[4]]);
                let GDM = Lanes([BYF[0], BYF[1], 0.0, BYF[2], BYF[3], BYF[4]]);
                let GDN = Lanes([CJR[0], CJR[1], 0.0, CJR[2], CJR[3], CJR[4]]);
                let GDO = Lanes([BZX[0], BZX[1], 0.0, BZX[2], BZX[3], BZX[4]]);
                GDQ = CDI;
                GDR = CEW;
                GDS = AVI;
                GDT = AZF;
                GDU = BCP;
                GDV = BYE;
                GDW = A;
                GDX = U;
                GDY = CJQ;
                GDZ = BZY;
                GEA = GDJ;
                GEB = GDK;
                GEC = AVJ;
                GED = AZG;
                GEE = GDL;
                GEF = GDM;
                GEG = N;
                GEH = GDP;
                GEI = GDN;
                GEJ = GDO;
            }
            let GZY;
            let GZZ;
            if GEK != 0.0 {
                let GZT = (GDQ / GZS) / GDR;
                let GZU = U + GZT;
                let GZV = if GZU >= AX { GZU } else { AX };
                let GZW = (((((GEA / GZS) - (GEB * GZT)) / GDR) * (if GZU >= AX { 1.0 } else { 0.0 })) * (AK / GZV)) * GZS;
                let GZX = U + (GZS * (GZV.ln()));
                GZY = GZX;
                GZZ = GZW;
            } else {
                GZY = U;
                GZZ = GDP;
            }
            let HAA = GZY * GZY;
            let HAB = GZZ * GZY;
            let HAC = U / GZY;
            let HAD = ((GZZ * HAC) * W) / GZY;
            let HAE = U / HAA;
            let HAF = (((HAB + HAB) * HAE) * W) / HAA;
            let HAG = GZY - U;
            let HAH = GDS - GDT;
            let HAI = GEC - GED;
            let HAJ = GDU - GDV;
            let HAK = GEE - GEF;
            let HAL = HAJ * HAJ;
            let HAM = HAK * HAJ;
            let HAN = HAM + HAM;
            let HAO = HAH + (AO * GDU);
            let HAP = Lanes([HAI[0], HAI[1], 0.0, HAI[2], HAI[3], HAI[4]]);
            let HAQ = HAP + (GEE * AO);
            let HAR = HAH + (AO * GDV);
            let HAS = HAP + (GEF * AO);
            let HAT = if 1.0f64 != 0.0 && (if HAO < -1.25e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HBB;
            let HBC;
            if HAT != 0.0 {
                let HAU = ASC * HAO;
                let HAV = -2.5e-1f64 / HAU;
                let HAW = (((HAQ * ASC) * HAV) * W) / HAU;
                HBB = HAV;
                HBC = HAW;
            } else {
                let HAX = HAQ * HAO;
                let HAY = ((HAO * HAO) + 6.25e-2f64).sqrt();
                let HAZ = CX * (HAO + HAY);
                let HBA = (HAQ + ((HAX + HAX) * (AK / (AJ * HAY)))) * CX;
                HBB = HAZ;
                HBC = HBA;
            }
            let HBD = if 1.0f64 != 0.0 && (if HAR < -1.25e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HBL;
            let HBM;
            if HBD != 0.0 {
                let HBE = ASC * HAR;
                let HBF = -2.5e-1f64 / HBE;
                let HBG = (((HAS * ASC) * HBF) * W) / HBE;
                HBL = HBF;
                HBM = HBG;
            } else {
                let HBH = HAS * HAR;
                let HBI = ((HAR * HAR) + 6.25e-2f64).sqrt();
                let HBJ = CX * (HAR + HBI);
                let HBK = (HAS + ((HBH + HBH) * (AK / (AJ * HBI)))) * CX;
                HBL = HBJ;
                HBM = HBK;
            }
            let HBN = (DZF + (HBB * GDW)).sqrt();
            let HBO = ((HBC * GDW) + Lanes([(GEG * HBB), 0.0, 0.0, 0.0, 0.0, 0.0])) * (AK / (AJ * HBN));
            let HBP = (DZF + (HBL * GDW)).sqrt();
            let HBQ = ((HBM * GDW) + Lanes([(GEG * HBL), 0.0, 0.0, 0.0, 0.0, 0.0])) * (AK / (AJ * HBP));
            let HBR = U + (AO * HBN);
            let HBS = HAO / HBR;
            let HBT = AO * HBP;
            let HBU = HBQ * AO;
            let HBV = U + HBT;
            let HBW = HAR / HBV;
            let HBX = HBN + HBP;
            let HBY = HBO + HBQ;
            let HBZ = HBX * HBX;
            let HCA = HBY * HBX;
            let HCB = HCA + HCA;
            let HCC = HBZ * HBX;
            let HCD = HAL / HCC;
            let HCE = CAM * HCD;
            let HCF = GDX * GDY;
            let HCG = (U + GDU) + GDV;
            let HCH = GEE + GEF;
            let HCI = (HCF * HAC) / HCG;
            let HCJ = (((((GEH * GDY) + (GEI * GDX)) * HAC) + (HAD * HCF)) - (HCH * HCI)) / HCG;
            let HCK = CAT * (HBZ + (HBN * HBP));
            let HCL = (HCK * HCI) + (AO * GDW);
            let HCM = CAM * HAL;
            let HCN = HBT - U;
            let HCO = (HAR * HCN) / HBV;
            let HCP = AO * (GDZ - U);
            let HCQ = GEJ * AO;
            let HCR = (HAH - (HCP * GDV)) + HCO;
            let HCS = GDU + GDV;
            let HCT = HCS + (HCM * HCI);
            let HCU = HCH + (((HAN * CAM) * HCI) + (HCJ * HCM));
            let HCV = (HBS + HBW) + ((HCE * HCL) - (GDZ * HCT));
            let HCW = (HAC * HCV) + (HAG * HCR);
            let HCX = ((HAD * HCV) + (((((HAQ - ((HBO * AO) * HBS)) / HBR) + ((HAS - (HBU * HBW)) / HBV)) + ((((((HAN - (((HCB * HBX) + (HBY * HBZ)) * HCD)) / HCC) * CAM) * HCL) + ((((((HCB + ((HBO * HBP) + (HBQ * HBN))) * CAT) * HCI) + (HCJ * HCK)) + Lanes([(GEG * AO), 0.0, 0.0, 0.0, 0.0, 0.0])) * HCE)) - ((GEJ * HCT) + (HCU * GDZ)))) * HAC)) + ((GZZ * HCR) + (((HAP - ((HCQ * GDV) + (GEF * HCP))) + ((((HAS * HCN) + (HBU * HAR)) - (HBU * HCO)) / HBV)) * HAG));
            let HCY = HAL * HCI;
            let HCZ = GDZ * HAC;
            let HDA = AO * GDZ;
            let HDB = HDA * HAG;
            let HDC = GDZ * HAE;
            let HDD = HAJ / AYH;
            let HDF = (U - (HAJ * HCI)) - (HDE * (HCY * HCI));
            let HDG = (CX * HCS) - (HDD * HDF);
            let HDH = GZY - HAC;
            let HDI = GDZ * HDH;
            let HDJ = (HDC * HDG) + (HDI * GDV);
            let HDK = ((((GEJ * HAE) + (HAF * GDZ)) * HDG) + (((HCH * CX) - (((HAK / AYH) * HDF) + (((((HAK * HCI) + (HCJ * HAJ)) * W) - (((((HAN * HCI) + (HCJ * HAL)) * HCI) + (HCJ * HCY)) * HDE)) * HDD))) * HDC)) + ((((GEJ * HDH) + ((GZZ - HAD) * GDZ)) * GDV) + (GEF * HDI));
            let HDL = ((HCZ * HCT) + (HDB * GDV)) - HDJ;
            let HDM = (((((GEJ * HAC) + (HAD * GDZ)) * HCT) + (HCU * HCZ)) + ((((HCQ * HAG) + (GZZ * HDA)) * GDV) + (GEF * HDB))) - HDK;
            let HDN = S * HCW;
            let HDO = Lanes([(T * HCW), 0.0, 0.0, 0.0, 0.0, 0.0]) + (HCX * S);
            let HDP = if 1.0f64 != 0.0 && (if HDN < staged[360] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HDY;
            let HDZ;
            if HDP != 0.0 {
                let HDR = ASC * HDN;
                let HDS = ((-HDQ) * HDQ) / HDR;
                let HDT = (((HDO * ASC) * HDS) * W) / HDR;
                HDY = HDS;
                HDZ = HDT;
            } else {
                let HDU = HDO * HDN;
                let HDV = ((HDN * HDN) + ((DZF * HDQ) * HDQ)).sqrt();
                let HDW = CX * (HDN + HDV);
                let HDX = (HDO + ((HDU + HDU) * (AK / (AJ * HDV)))) * CX;
                HDY = HDW;
                HDZ = HDX;
            }
            let HEA = HDL + HDJ;
            let HED = ((S * HEA) + (HEB * HDY)) / HEC;
            let HEF = U + (HED.powf(HEE));
            let HEG = staged[362] / HEF;
            let HEH = staged[363] + (HEG / DKR);
            let HEI = 3.4531302e-11f64 / HEH;
            let HEK = HEJ * S;
            let HEM = HEL * HEI;
            let HEN = -(HEM * S);
            let HEO = ((((((((((((((Lanes([(T * HEA), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((HDM + HDK) * S)) + (HDZ * HEB)) / HEC) * (HEE * (HED.powf(staged[442])))) * HEG) * W) / HEF) / DKR) * HEI) * W) / HEH) * HEL) * S) + Lanes([(T * HEM), 0.0, 0.0, 0.0, 0.0, 0.0])) * W;
            let HEP = HEN * HDL;
            let HEQ = (HEO * HDL) + (HDM * HEN);
            let HER = HEN * HDJ;
            let HES = (HEO * HDJ) + (HDK * HEN);
            let HET = -(((HEK * HCW) + HEP) + HER);
            let HEU = (((Lanes([((T * HEJ) * HCW), 0.0, 0.0, 0.0, 0.0, 0.0]) + (HCX * HEK)) + HEQ) + HES) * W;
            let HGY;
            let HGZ;
            let HHA;
            let HHB;
            if HEV != 0.0 {
                let HEX = HEW * ALR;
                let HEY = ALS * HEW;
                let HFA = HEZ * AMK;
                let HFB = AMN * HEZ;
                let HFC = Lanes([0.0, HEY[0], HEY[1]]);
                let HFD = Lanes([0.0, HFB[0], HFB[1], HFB[2], HFB[3]]);
                HGY = HEX;
                HGZ = HFA;
                HHA = HFC;
                HHB = HFD;
            } else {
                let HFE = ALR - CF;
                let HFF = Lanes([0.0, ALS[0], ALS[1]]) - Lanes([CG, 0.0, 0.0]);
                let HFH = HFE + HFG;
                let HFI = HFF * HFH;
                let HFJ = ((HFH * HFH) + 8e-2f64).sqrt();
                let HFK = CX * (HFH - HFJ);
                let HFL = (HFF - ((HFI + HFI) * (AK / (AJ * HFJ)))) * CX;
                let HFN = (-HFK) / HFM;
                let HFP = U + (HFN.powf(HFO));
                let HFR = HFP.powf(HFQ);
                let HFS = HFK / HFR;
                let HFU = (U - ((ADB * HFS) / HFT)).sqrt();
                let HFW = ALS * HFV;
                let HGA = HFZ * ((HFV * ALR) + (HFY * ((HFE - HFK) - (HFX * (-1e0f64 + HFU)))));
                let HGB = (Lanes([0.0, HFW[0], HFW[1]]) + (((HFF - HFL) - (((((((HFL - (((((HFL * W) / HFM) * (HFO * (HFN.powf(staged[443])))) * (HFQ * (HFP.powf(staged[444])))) * HFS)) / HFR) * ADB) / HFT) * W) * (AK / (AJ * HFU))) * HFX)) * HFY)) * HFZ;
                let HGC = AMK - CF;
                let HGD = Lanes([0.0, AMN[0], AMN[1], AMN[2], AMN[3]]) - Lanes([CG, 0.0, 0.0, 0.0, 0.0]);
                let HGE = HGC + HFG;
                let HGF = HGD * HGE;
                let HGG = ((HGE * HGE) + 8e-2f64).sqrt();
                let HGH = CX * (HGE - HGG);
                let HGI = (HGD - ((HGF + HGF) * (AK / (AJ * HGG)))) * CX;
                let HGK = (-HGH) / HGJ;
                let HGM = U + (HGK.powf(HGL));
                let HGO = HGM.powf(HGN);
                let HGP = HGH / HGO;
                let HGR = (U - ((ADB * HGP) / HGQ)).sqrt();
                let HGT = AMN * HGS;
                let HGW = HFZ * ((HGS * AMK) + (HGV * ((HGC - HGH) - (HGU * (-1e0f64 + HGR)))));
                let HGX = (Lanes([0.0, HGT[0], HGT[1], HGT[2], HGT[3]]) + (((HGD - HGI) - (((((((HGI - (((((HGI * W) / HGJ) * (HGL * (HGK.powf(staged[445])))) * (HGN * (HGM.powf(staged[446])))) * HGP)) / HGO) * ADB) / HGQ) * W) * (AK / (AJ * HGR))) * HGU)) * HGV)) * HFZ;
                HGY = HGA;
                HGZ = HGW;
                HHA = HGB;
                HHB = HGX;
            }
            let HHD = HHC * (ALL - AJP);
            let HHE = (Lanes([ALM, 0.0]) - Lanes([0.0, AJS])) * HHC;
            let HHF = Lanes([0.0, 0.0, 0.0, 0.0, HHE[0], HHE[1]]);
            let HHS;
            let HHT;
            if HHG != 0.0 {
                let HHH = staged[382] / AT;
                let HHI = if HHH >= AX { HHH } else { AX };
                let HHJ = HHI.ln();
                let HHK = ((((AU * HHH) * W) / AT) * (if HHH >= AX { 1.0 } else { 0.0 })) * (AK / HHI);
                let HHL = (CH + (S * HHJ)) + CI;
                let HHM = if HHL >= CH { HHL } else { CH };
                let HHN = ((T * HHJ) + (HHK * S)) * (if HHL >= CH { 1.0 } else { 0.0 });
                let HHP = AA * HHO;
                let HHQ = U + (HHO * CO);
                let HHR = if HHQ < -1e1f64 { 1.0 } else { 0.0 };
                let HIB;
                let HIC;
                if HHR != 0.0 {
                    let HHV = -1e-6f64 / HHQ;
                    let HHW = ((HHP * HHV) * W) / HHQ;
                    HIB = HHV;
                    HIC = HHW;
                } else {
                    let HHX = HHP * HHQ;
                    let HHY = ((HHQ * HHQ) + 4e-6f64).sqrt();
                    let HHZ = CX * (HHQ + HHY);
                    let HIA = (HHP + ((HHX + HHX) * (AK / (AJ * HHY)))) * CX;
                    HIB = HHZ;
                    HIC = HIA;
                }
                let HIE = HID * HIB;
                let HIF = HIC * HID;
                let HII = HIH * (U + (HIG * CO));
                let HIJ = (AA * HIG) * HIH;
                let HIK = HHM - AOJ;
                let HIL = if 0.0f64 != 0.0 && (if HIK < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let HIP = if HIL != 0.0 {
                    let HIM = -1.0000000000000002e-2f64 / (ASC * HIK);
                    HIM
                } else {
                    let HIN = HIK - ASG;
                    let HIO = CX * ((HIK + ASG) + (((HIN * HIN) + 2.5000000000000005e-3f64).sqrt()));
                    HIO
                };
                let HIQ = AST / (staged[387] * (HIP.sqrt()));
                let HIT = AOH * HIS;
                let HIV = AOK * HIU;
                let HIW = ((Lanes([HIF, 0.0, 0.0, 0.0]) + Lanes([0.0, HIT[0], HIT[1], HIT[2]])) - Lanes([0.0, HIV[0], HIV[1], HIV[2]])) / ASZ;
                let HIX = U + ((((HIR + HIE) + (HIS * AOI)) - (HIU * AOJ)) / ASZ);
                let HIY = if 0.0f64 != 0.0 && (if HIX < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let HJH;
                let HJI;
                if HIY != 0.0 {
                    let HIZ = ASC * HIX;
                    let HJA = -2.5000000000000005e-3f64 / HIZ;
                    let HJB = (((HIW * ASC) * HJA) * W) / HIZ;
                    HJH = HJA;
                    HJI = HJB;
                } else {
                    let HJC = HIX - U;
                    let HJD = HIW * HJC;
                    let HJE = ((HJC * HJC) + 6.250000000000001e-4f64).sqrt();
                    let HJF = CX * ((HIX + U) + HJE);
                    let HJG = (HIW + ((HJD + HJD) * (AK / (AJ * HJE)))) * CX;
                    HJH = HJF;
                    HJI = HJG;
                }
                let HJJ = HJH * S;
                let HJK = (HJI * S) + Lanes([(T * HJH), 0.0, 0.0, 0.0]);
                let HJL = U / HJJ;
                let HJM = ((HJK * HJL) * W) / HJJ;
                let HJN = AJV * HJL;
                let HJO = AJW * HJL;
                let HJP = HJM * AJV;
                let HJQ = Lanes([0.0, 0.0, 0.0, HJO[0], HJO[1]]) + Lanes([HJP[0], HJP[1], HJP[2], 0.0, HJP[3]]);
                let HJR = AMW * HJL;
                let HJS = ANB * HJL;
                let HJT = Lanes([0.0, HJS[0], HJS[1], HJS[2]]) + (HJM * AMW);
                let HJU = AUY * HJL;
                let HJV = HJM * AUY;
                let HJX = AOK * HJW;
                let HJY = -(HII + (HJW * AOJ));
                let HJZ = HJY * AOI;
                let HKA = AOH * HJY;
                let HKB = (((Lanes([HIJ, 0.0, 0.0, 0.0]) + Lanes([0.0, HJX[0], HJX[1], HJX[2]])) * W) * AOI) + Lanes([0.0, HKA[0], HKA[1], HKA[2]]);
                let HKD = staged[393] + (HKC * AOJ);
                let HKF = (Z.powf(HKE)) - U;
                let HKG = HKD * HKF;
                let HKH = (AOK * HKC) * HKF;
                let HKI = Lanes([0.0, HKH[0], HKH[1], HKH[2]]) + Lanes([((AA * (HKE * (Z.powf(staged[447])))) * HKD), 0.0, 0.0, 0.0]);
                let HKK = FSQ * (U + (HKJ * AOJ));
                let HKL = (AOK * HKJ) * FSQ;
                let HKM = if HKK > A { 1.0 } else { 0.0 };
                let HKQ;
                let HKR;
                if HKM != 0.0 {
                    let HKN = (parameters[1015] * AWB) / HKK;
                    let HKO = ((HKL * HKN) * W) / HKK;
                    let HKP = if HKN < 4e1f64 { 1.0 } else { 0.0 };
                    let HLW;
                    let HLX;
                    if HKP != 0.0 {
                        let HLQ = (HKN.cosh()) - U;
                        let HLR = (CX * HLP) / HLQ;
                        let HLS = (((HKO * (HKN.sinh())) * HLR) * W) / HLQ;
                        HLW = HLR;
                        HLX = HLS;
                    } else {
                        let HLT = -HKN;
                        let HLU = HLP * (rspice_limited_exp(HLT));
                        let HLV = ((HKO * W) * (rspice_limited_exp_derivative(HLT))) * HLP;
                        HLW = HLU;
                        HLX = HLV;
                    }
                    HKQ = HLW;
                    HKR = HLX;
                } else {
                    HKQ = A;
                    HKR = AVV;
                }
                let HKS = BY - HHM;
                let HKT = HKR * HKS;
                let HKV = AOK * HKU;
                let HKW = (((((HJZ - HKG) + (HKQ * HKS)) + parameters[961]) + staged[395]) - (HKU * AOJ)) + staged[397];
                let HKX = ((((HKB - HKI) + (Lanes([0.0, HKT[0], HKT[1], HKT[2]]) + Lanes([((BZ - HHN) * HKQ), 0.0, 0.0, 0.0]))) - Lanes([0.0, HKV[0], HKV[1], HKV[2]])) * HJL) + (HJM * HKW);
                let HKY = (HJN - HJU) - (HKW * HJL);
                let HKZ = (HJQ - Lanes([HJV[0], HJV[1], HJV[2], 0.0, HJV[3]])) - Lanes([HKX[0], HKX[1], HKX[2], 0.0, HKX[3]]);
                let HLB = (HLA * HJL).sqrt();
                let HLD = (HLB / ASZ) * HLC;
                let HLE = (((HJM * HLA) * (AK / (AJ * HLB))) / ASZ) * HLC;
                let HLF = HHJ / HJH;
                let HLG = (Lanes([HHK, 0.0, 0.0, 0.0]) - (HJI * HLF)) / HJH;
                let HLH = (HLE / AYB) * AYC;
                let HLI = (CX * HKY) - (AYC * (U + (HLD / AYB)));
                let HLJ = (HKZ * CX) - Lanes([HLH[0], HLH[1], HLH[2], 0.0, HLH[3]]);
                let HLK = HLJ * HLI;
                let HLL = ((HLI * HLI) + (AYH * HKY)).sqrt();
                let HLM = HLI + HLL;
                let HLN = HLJ + (((HLK + HLK) + (HKZ * AYH)) * (AK / (AJ * HLL)));
                let HLO = if HKY < A { 1.0 } else { 0.0 };
                let HMR;
                let HMS;
                if HLO != 0.0 {
                    let HLY = (HKY - HLM) / HLD;
                    let HLZ = HLE * HLY;
                    let HMA = (((HKZ - HLN) - Lanes([HLZ[0], HLZ[1], HLZ[2], 0.0, HLZ[3]])) / HLD) * HLY;
                    let HMB = (U - HLM) + (HLY * HLY);
                    let HMC = if HMB >= AX { HMB } else { AX };
                    let HMD = -(HMC.ln());
                    let HME = ((((HLN * W) + (HMA + HMA)) * (if HMB >= AX { 1.0 } else { 0.0 })) * (AK / HMC)) * W;
                    HMR = HMD;
                    HMS = HME;
                } else {
                    let HMF = -HLM;
                    let HMG = rspice_limited_exp(HMF);
                    let HMH = (HLN * W) * (rspice_limited_exp_derivative(HMF));
                    let HMI = CX * HLD;
                    let HMJ = HLE * CX;
                    let HMK = HMJ * HMI;
                    let HML = HMK + HMK;
                    let HMM = (((HKY - U) + HMG) + (HMI * HMI)).sqrt();
                    let HMN = HMM - HMI;
                    let HMO = ((((HKZ + HMH) + Lanes([HML[0], HML[1], HML[2], 0.0, HML[3]])) * (AK / (AJ * HMM))) - Lanes([HMJ[0], HMJ[1], HMJ[2], 0.0, HMJ[3]])) * HMN;
                    let HMP = ((HMN * HMN) + U) - HMG;
                    let HMQ = (HMO + HMO) - HMH;
                    HMR = HMP;
                    HMS = HMQ;
                }
                let HMT = HMR + U;
                let HMU = HMR - U;
                let HMV = HMU * HMU;
                let HMW = HMS * HMU;
                let HMX = HMW + HMW;
                let HMY = (HMV + 1e0f64).sqrt();
                let HMZ = (CX * (HMT + HMY)).sqrt();
                let HNA = ((HMS + (HMX * (AK / (AJ * HMY)))) * CX) * (AK / (AJ * HMZ));
                let HNB = AO * HMZ;
                let HNC = HNA * AO;
                let HND = HLD / HNB;
                let HNE = Lanes([HLE[0], HLE[1], HLE[2], 0.0, HLE[3]]);
                let HNF = (U + HND) / HLD;
                let HNG = HLE * HNF;
                let HNH = (((HNE - (HNC * HND)) / HNB) - Lanes([HNG[0], HNG[1], HNG[2], 0.0, HNG[3]])) / HLD;
                let HNI = HLG * AO;
                let HNJ = HMR - (AO * HLF);
                let HNK = HMS - Lanes([HNI[0], HNI[1], HNI[2], 0.0, HNI[3]]);
                let HNL = HNJ - HJR;
                let HNM = HNK - Lanes([HJT[0], HJT[1], HJT[2], 0.0, HJT[3]]);
                let HNN = ADB * HNF;
                let HNO = HNN * HMZ;
                let HNP = if HNO >= AX { HNO } else { AX };
                let HNQ = HNL - (HNP.ln());
                let HNR = HNM - (((((HNH * ADB) * HMZ) + (HNA * HNN)) * (if HNO >= AX { 1.0 } else { 0.0 })) * (AK / HNP));
                let HNS = HNQ + BAH;
                let HNT = ((HNQ * HNS) + BAJ).sqrt();
                let HNU = CX * ((HNQ - BAG) - HNT);
                let HNV = (HNR - (((HNR * HNS) + (HNR * HNQ)) * (AK / (AJ * HNT)))) * CX;
                let HNW = if HNU <= -6.8e1f64 { 1.0 } else { 0.0 };
                let HPY;
                let HPZ;
                if HNW != 0.0 {
                    let HNX = if HNU < -1.1e2f64 { 1.0 } else { 0.0 };
                    let HQI;
                    let HQJ;
                    if HNX != 0.0 {
                        HQI = HQG;
                        HQJ = BCT;
                    } else {
                        let HQH = if HNU > -9e1f64 { 1.0 } else { 0.0 };
                        let HRF;
                        let HRG;
                        if HQH != 0.0 {
                            let HQS = rspice_limited_exp(HNU);
                            let HQT = HNV * (rspice_limited_exp_derivative(HNU));
                            HRF = HQS;
                            HRG = HQT;
                        } else {
                            let HQV = (HNU - HQU) / BDI;
                            let HQW = HNV / BDI;
                            let HQX = HQV * HQV;
                            let HQY = HQW * HQV;
                            let HQZ = HQY + HQY;
                            let HRA = BDO - HQX;
                            let HRB = 9.375e-1f64 - (HQX * HRA);
                            let HRC = HQU + (BDI * ((7.8125e-2f64 + (CX * HQV)) + (HQX * HRB)));
                            let HRD = rspice_limited_exp(HRC);
                            let HRE = (((HQW * CX) + ((HQZ * HRB) + ((((HQZ * HRA) + ((HQZ * W) * HQX)) * W) * HQX))) * BDI) * (rspice_limited_exp_derivative(HRC));
                            HRF = HRD;
                            HRG = HRE;
                        }
                        HQI = HRF;
                        HQJ = HRG;
                    }
                    let HQK = AO * HNF;
                    let HQL = HQI * AO;
                    let HQM = (HQL * HNF) + HNB;
                    let HQN = HQK * HQM;
                    let HQO = if HQN >= AX { HQN } else { AX };
                    let HQP = ((U + HNL) - HNU) - (HQO.ln());
                    let HQQ = HQI * HQP;
                    let HQR = (HQJ * HQP) + (((HNM - HNV) - (((((HNH * AO) * HQM) + (((((HQJ * AO) * HNF) + (HNH * HQL)) + HNC) * HQK)) * (if HQN >= AX { 1.0 } else { 0.0 })) * (AK / HQO))) * HQI);
                    HPY = HQQ;
                    HPZ = HQR;
                } else {
                    let HNY = rspice_limited_exp(HNU);
                    let HNZ = HNV * (rspice_limited_exp_derivative(HNU));
                    let HOA = U / HMZ;
                    let HOB = AO * HNY;
                    let HOC = HNZ * AO;
                    let HOD = HOB * HNF;
                    let HOE = (HOC * HNF) + (HNH * HOB);
                    let HOF = HOD + HNB;
                    let HOG = HOD * HOF;
                    let HOH = if HOG >= AX { HOG } else { AX };
                    let HOI = 1e0f64 / HNY;
                    let HOJ = HNF + HOA;
                    let HOK = HNH + (((HNA * HOA) * W) / HMZ);
                    let HOL = (HNF * HNY) + HMZ;
                    let HOM = HOJ / HOL;
                    let HON = (AO + HOI) + HOM;
                    let HOO = ((HOB + (HOH.ln())) - HNL) / HON;
                    let HOP = HNY - HOO;
                    let HOQ = HNZ - ((((HOC + ((((HOE * HOF) + ((HOE + HNC) * HOD)) * (if HOG >= AX { 1.0 } else { 0.0 })) * (AK / HOH))) - HNM) - (((((HNZ * HOI) * W) / HNY) + ((HOK - ((((HNH * HNY) + (HNZ * HNF)) + HNA) * HOM)) / HOL)) * HOO)) / HON);
                    let HOR = AO * HOP;
                    let HOS = HOQ * AO;
                    let HOT = HOR * HNF;
                    let HOU = (HOS * HNF) + (HNH * HOR);
                    let HOV = HOT + HNB;
                    let HOW = HOT * HOV;
                    let HOX = if HOW >= AX { HOW } else { AX };
                    let HOY = (HOR + (HOX.ln())) - HNL;
                    let HOZ = (HOS + ((((HOU * HOV) + ((HOU + HNC) * HOT)) * (if HOW >= AX { 1.0 } else { 0.0 })) * (AK / HOX))) - HNM;
                    let HPA = 1e0f64 / HOP;
                    let HPB = (HNF * HOP) + HMZ;
                    let HPC = ((HNH * HOP) + (HOQ * HNF)) + HNA;
                    let HPD = HOJ / HPB;
                    let HPE = (HOK - (HPC * HPD)) / HPB;
                    let HPF = (AO + HPA) + HPD;
                    let HPG = (((HOQ * HPA) * W) / HOP) + HPE;
                    let HPH = HPE * HPD;
                    let HPI = U / HOP;
                    let HPJ = (((HOQ * HPI) * W) / HOP) * HPI;
                    let HPL = HMZ * HMZ;
                    let HPM = HNA * HMZ;
                    let HPN = HPL * HMZ;
                    let HPO = HPN * HPB;
                    let HPP = 1e0f64 / HPO;
                    let HPQ = ((HPK * (HPI * HPI)) - HPP) - (HPD * HPD);
                    let HPR = HOY / HPF;
                    let HPS = AO * HPF;
                    let HPT = HPS * HPF;
                    let HPU = (HOY * HPQ) / HPT;
                    let HPV = U + HPU;
                    let HPW = HOP - (HPR * HPV);
                    let HPX = HOQ - ((((HOZ - (HPG * HPR)) / HPF) * HPV) + (((((HOZ * HPQ) + (((((HPJ + HPJ) * HPK) - ((((((((HPM + HPM) * HMZ) + (HNA * HPL)) * HPB) + (HPC * HPN)) * HPP) * W) / HPO)) - (HPH + HPH)) * HOY)) - ((((HPG * AO) * HPF) + (HPG * HPS)) * HPU)) / HPT) * HPR));
                    HPY = HPW;
                    HPZ = HPX;
                }
                let HQA = AO * HJJ;
                let HQB = HJK * AO;
                let HQC = HQB * HPY;
                let HQD = (((HQA * HPY) + HQA) + AMW) - AMW;
                let HQE = (((Lanes([HQC[0], HQC[1], HQC[2], 0.0, HQC[3]]) + (HPZ * HQA)) + Lanes([HQB[0], HQB[1], HQB[2], 0.0, HQB[3]])) + BTT) - BTT;
                let HQF = if 1.0f64 != 0.0 && (if HQD < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let HRO;
                let HRP;
                if HQF != 0.0 {
                    let HRH = ASC * HQD;
                    let HRI = -1e-6f64 / HRH;
                    let HRJ = (((HQE * ASC) * HRI) * W) / HRH;
                    HRO = HRI;
                    HRP = HRJ;
                } else {
                    let HRK = HQE * HQD;
                    let HRL = ((HQD * HQD) + 2.5e-7f64).sqrt();
                    let HRM = CX * (HQD + HRL);
                    let HRN = (HQE + ((HRK + HRK) * (AK / (AJ * HRL)))) * CX;
                    HRO = HRM;
                    HRP = HRN;
                }
                let HRQ = ANE / HRO;
                let HRR = HRQ.powf(BUU);
                let HRS = U + HRR;
                let HRT = HRS.powf(BUZ);
                let HRU = ANF * HRT;
                let HRV = (ANE * HRT) + AMW;
                let HRW = HJM * HRV;
                let HRX = (HMV + 1e0f64).sqrt();
                let HRY = (CX * (HMT + HRX)).sqrt();
                let HRZ = ((HMS + (HMX * (AK / (AJ * HRX)))) * CX) * (AK / (AJ * HRY));
                let HSA = AO * HRY;
                let HSB = HRZ * AO;
                let HSC = HLD / HSA;
                let HSD = (U + HSC) / HLD;
                let HSE = HLE * HSD;
                let HSF = (((HNE - (HSB * HSC)) / HSA) - Lanes([HSE[0], HSE[1], HSE[2], 0.0, HSE[3]])) / HLD;
                let HSG = HNJ - (HRV * HJL);
                let HSH = HNK - ((((Lanes([0.0, HRU[0], HRU[1], 0.0, HRU[2]]) + (((((((BUS - (HRP * HRQ)) / HRO) * (BUU * (HRQ.powf(BUX)))) + Lanes([(BUV * (HRR * (HRQ.ln()))), 0.0, 0.0, 0.0, 0.0])) * (BUZ * (HRS.powf(BVC)))) + Lanes([(BVA * (HRT * (HRS.ln()))), 0.0, 0.0, 0.0, 0.0])) * ANE)) + BTT) * HJL) + Lanes([HRW[0], HRW[1], HRW[2], 0.0, HRW[3]]));
                let HSI = ADB * HSD;
                let HSJ = HSI * HRY;
                let HSK = if HSJ >= AX { HSJ } else { AX };
                let HSL = HSG - (HSK.ln());
                let HSM = HSH - (((((HSF * ADB) * HRY) + (HRZ * HSI)) * (if HSJ >= AX { 1.0 } else { 0.0 })) * (AK / HSK));
                let HSN = HSL + BAH;
                let HSO = ((HSL * HSN) + BAJ).sqrt();
                let HSP = CX * ((HSL - BAG) - HSO);
                let HSQ = (HSM - (((HSM * HSN) + (HSM * HSL)) * (AK / (AJ * HSO)))) * CX;
                let HSR = if HSP <= -6.8e1f64 { 1.0 } else { 0.0 };
                let HUT;
                let HUU;
                if HSR != 0.0 {
                    let HSS = if HSP < -1.1e2f64 { 1.0 } else { 0.0 };
                    let HUY;
                    let HUZ;
                    if HSS != 0.0 {
                        HUY = HUW;
                        HUZ = BCT;
                    } else {
                        let HUX = if HSP > -9e1f64 { 1.0 } else { 0.0 };
                        let HVV;
                        let HVW;
                        if HUX != 0.0 {
                            let HVI = rspice_limited_exp(HSP);
                            let HVJ = HSQ * (rspice_limited_exp_derivative(HSP));
                            HVV = HVI;
                            HVW = HVJ;
                        } else {
                            let HVL = (HSP - HVK) / BDI;
                            let HVM = HSQ / BDI;
                            let HVN = HVL * HVL;
                            let HVO = HVM * HVL;
                            let HVP = HVO + HVO;
                            let HVQ = BDO - HVN;
                            let HVR = 9.375e-1f64 - (HVN * HVQ);
                            let HVS = HVK + (BDI * ((7.8125e-2f64 + (CX * HVL)) + (HVN * HVR)));
                            let HVT = rspice_limited_exp(HVS);
                            let HVU = (((HVM * CX) + ((HVP * HVR) + ((((HVP * HVQ) + ((HVP * W) * HVN)) * W) * HVN))) * BDI) * (rspice_limited_exp_derivative(HVS));
                            HVV = HVT;
                            HVW = HVU;
                        }
                        HUY = HVV;
                        HUZ = HVW;
                    }
                    let HVA = AO * HSD;
                    let HVB = HUY * AO;
                    let HVC = (HVB * HSD) + HSA;
                    let HVD = HVA * HVC;
                    let HVE = if HVD >= AX { HVD } else { AX };
                    let HVF = ((U + HSG) - HSP) - (HVE.ln());
                    let HVG = HUY * HVF;
                    let HVH = (HUZ * HVF) + (((HSH - HSQ) - (((((HSF * AO) * HVC) + (((((HUZ * AO) * HSD) + (HSF * HVB)) + HSB) * HVA)) * (if HVD >= AX { 1.0 } else { 0.0 })) * (AK / HVE))) * HUY);
                    HUT = HVG;
                    HUU = HVH;
                } else {
                    let HST = rspice_limited_exp(HSP);
                    let HSU = HSQ * (rspice_limited_exp_derivative(HSP));
                    let HSV = U / HRY;
                    let HSW = AO * HST;
                    let HSX = HSU * AO;
                    let HSY = HSW * HSD;
                    let HSZ = (HSX * HSD) + (HSF * HSW);
                    let HTA = HSY + HSA;
                    let HTB = HSY * HTA;
                    let HTC = if HTB >= AX { HTB } else { AX };
                    let HTD = 1e0f64 / HST;
                    let HTE = HSD + HSV;
                    let HTF = HSF + (((HRZ * HSV) * W) / HRY);
                    let HTG = (HSD * HST) + HRY;
                    let HTH = HTE / HTG;
                    let HTI = (AO + HTD) + HTH;
                    let HTJ = ((HSW + (HTC.ln())) - HSG) / HTI;
                    let HTK = HST - HTJ;
                    let HTL = HSU - ((((HSX + ((((HSZ * HTA) + ((HSZ + HSB) * HSY)) * (if HTB >= AX { 1.0 } else { 0.0 })) * (AK / HTC))) - HSH) - (((((HSU * HTD) * W) / HST) + ((HTF - ((((HSF * HST) + (HSU * HSD)) + HRZ) * HTH)) / HTG)) * HTJ)) / HTI);
                    let HTM = AO * HTK;
                    let HTN = HTL * AO;
                    let HTO = HTM * HSD;
                    let HTP = (HTN * HSD) + (HSF * HTM);
                    let HTQ = HTO + HSA;
                    let HTR = HTO * HTQ;
                    let HTS = if HTR >= AX { HTR } else { AX };
                    let HTT = (HTM + (HTS.ln())) - HSG;
                    let HTU = (HTN + ((((HTP * HTQ) + ((HTP + HSB) * HTO)) * (if HTR >= AX { 1.0 } else { 0.0 })) * (AK / HTS))) - HSH;
                    let HTV = 1e0f64 / HTK;
                    let HTW = (HSD * HTK) + HRY;
                    let HTX = ((HSF * HTK) + (HTL * HSD)) + HRZ;
                    let HTY = HTE / HTW;
                    let HTZ = (HTF - (HTX * HTY)) / HTW;
                    let HUA = (AO + HTV) + HTY;
                    let HUB = (((HTL * HTV) * W) / HTK) + HTZ;
                    let HUC = HTZ * HTY;
                    let HUD = U / HTK;
                    let HUE = (((HTL * HUD) * W) / HTK) * HUD;
                    let HUG = HRY * HRY;
                    let HUH = HRZ * HRY;
                    let HUI = HUG * HRY;
                    let HUJ = HUI * HTW;
                    let HUK = 1e0f64 / HUJ;
                    let HUL = ((HUF * (HUD * HUD)) - HUK) - (HTY * HTY);
                    let HUM = HTT / HUA;
                    let HUN = AO * HUA;
                    let HUO = HUN * HUA;
                    let HUP = (HTT * HUL) / HUO;
                    let HUQ = U + HUP;
                    let HUR = HTK - (HUM * HUQ);
                    let HUS = HTL - ((((HTU - (HUB * HUM)) / HUA) * HUQ) + (((((HTU * HUL) + (((((HUE + HUE) * HUF) - ((((((((HUH + HUH) * HRY) + (HRZ * HUG)) * HTW) + (HTX * HUI)) * HUK) * W) / HUJ)) - (HUC + HUC)) * HTT)) - ((((HUB * AO) * HUA) + (HUB * HUN)) * HUP)) / HUO) * HUM));
                    HUT = HUR;
                    HUU = HUS;
                }
                let HUV = if 0.0f64 != 0.0 && (if HMR < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let HWD;
                let HWE;
                if HUV != 0.0 {
                    let HVX = ASC * HMR;
                    let HVY = -4e0f64 / HVX;
                    let HVZ = (((HMS * ASC) * HVY) * W) / HVX;
                    HWD = HVY;
                    HWE = HVZ;
                } else {
                    let HWA = (HMV + 1e0f64).sqrt();
                    let HWB = CX * (HMT + HWA);
                    let HWC = (HMS + (HMX * (AK / (AJ * HWA)))) * CX;
                    HWD = HWB;
                    HWE = HWC;
                }
                let HWF = HWD.sqrt();
                let HWG = HWE * (AK / (AJ * HWF));
                let HWH = (HMS - HPZ) - HUU;
                let HWI = ((HMR - HPY) - HUT) - U;
                let HWJ = if 0.0f64 != 0.0 && (if HWI < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let HWS;
                let HWT;
                if HWJ != 0.0 {
                    let HWK = ASC * HWI;
                    let HWL = -4e0f64 / HWK;
                    let HWM = (((HWH * ASC) * HWL) * W) / HWK;
                    HWS = HWL;
                    HWT = HWM;
                } else {
                    let HWN = HWI - U;
                    let HWO = HWH * HWN;
                    let HWP = ((HWN * HWN) + 1e0f64).sqrt();
                    let HWQ = CX * ((HWI + U) + HWP);
                    let HWR = (HWH + ((HWO + HWO) * (AK / (AJ * HWP)))) * CX;
                    HWS = HWQ;
                    HWT = HWR;
                }
                let HWU = HWS.sqrt();
                let HWV = HWF + HWU;
                let HWW = HLD / HWV;
                let HWX = U + HWW;
                let HWY = CNN * HWX;
                let HXA = (((HWY * CNL) * HWZ) / AWB) * ASZ;
                let HXB = HXA * HJJ;
                let HXC = HJK * HXA;
                let HXD = HXB * HJJ;
                let HXE = HJK * HXB;
                let HXF = HPY - HUT;
                let HXG = (U + HPY) + HUT;
                let HXH = HXF * HXG;
                let HXI = HXD * HXH;
                let HXJ = HXI * CHL;
                let HXK = (((((((((((((((HNE - ((HWG + (HWT * (AK / (AJ * HWU)))) * HWW)) / HWV) * CNN) * CNL) + (CNM * HWY)) * HWZ) / AWB) * ASZ) * HJJ) + Lanes([HXC[0], HXC[1], HXC[2], 0.0, HXC[3]])) * HJJ) + Lanes([HXE[0], HXE[1], HXE[2], 0.0, HXE[3]])) * HXH) + ((((HPZ - HUU) * HXG) + ((HPZ + HUU) * HXF)) * HXD)) * CHL) + (CHM * HXI);
                let HXL = HXJ + COV;
                let HXM = Lanes([0.0, HXK[0], HXK[1], 0.0, HXK[2], HXK[3], HXK[4]]) + COZ;
                let HXN = FSU * ((ASZ + HIQ) + HIR);
                let HXO = ((AO * HWX) * ASZ) * S;
                let HXP = (HXO * HUT) / FSK;
                let HXS = HXP + HXN;
                let HXT = (HXO * HPY) / FSK;
                let HXU = if (((((staged[403] * S) / ((staged[406] * HXN) * HXN)) * HXJ) * HXJ) + ((((((4.112842231783458e-57f64 * S) * (HXJ.abs())) * CNL) / HWS) * (((HXR * ((if ((HXT + HXN) / HXS) >= AX { ((HXT + HXN) / HXS) } else { AX }).ln())) + (HXQ * (HXT - HXP))) + (staged[404] * ((HXT * HXT) - (HXP * HXP))))) + ((((((FTB * HXJ) * HXJ) / staged[405]) * FST) * ((HXR + (HXQ * HXP)) + ((staged[402] * HXP) * HXP))) / (HXS * HXS)))) > A { 1.0 } else { 0.0 };
                HHS = HXL;
                HHT = HXM;
            } else {
                HHS = COV;
                HHT = COZ;
            }
            let HHU = if AMZ > A { 1.0 } else { 0.0 };
            let HYE;
            let HYF;
            let HYG;
            let HYH;
            if HHU != 0.0 {
                let HXW = HXV * HEP;
                let HXX = HEQ * HXV;
                let HXY = HXV * HER;
                let HXZ = HES * HXV;
                HYE = HXW;
                HYF = HXY;
                HYG = HXX;
                HYH = HXZ;
            } else {
                let HYA = HXV * HER;
                let HYB = HES * HXV;
                let HYC = HXV * HEP;
                let HYD = HEQ * HXV;
                HYE = HYA;
                HYF = HYC;
                HYG = HYB;
                HYH = HYD;
            }
            let HYP;
            let HYQ;
            let HYR;
            let HYS;
            let HYT;
            let HYU;
            if HYI != 0.0 {
                let HYJ = HHD + DDT;
                let HYK = Lanes([0.0, 0.0, 0.0, HHE[0], HHE[1]]) + DDX;
                let HYL = HGZ + DDU;
                let HYM = HHB + DDY;
                let HZV;
                let HZW;
                let HZX;
                let HZY;
                if HYN != 0.0 {
                    let HZP = HYJ + DDV;
                    let HZQ = Lanes([HYK[0], HYK[1], HYK[2], 0.0, HYK[3], HYK[4]]) + Lanes([DDZ[0], 0.0, 0.0, DDZ[1], DDZ[2], DDZ[3]]);
                    let HZR = HGY + DDW;
                    let HZS = Lanes([HHA[0], HHA[1], HHA[2], 0.0]) + DEA;
                    HZV = HZP;
                    HZW = HZR;
                    HZX = HZQ;
                    HZY = HZS;
                } else {
                    let HZT = Lanes([HYK[0], HYK[1], HYK[2], 0.0, HYK[3], HYK[4]]);
                    let HZU = Lanes([HHA[0], HHA[1], HHA[2], 0.0]);
                    HZV = HYJ;
                    HZW = HGY;
                    HZX = HZT;
                    HZY = HZU;
                }
                HYP = HZV;
                HYQ = HZW;
                HYR = HYL;
                HYS = HZX;
                HYT = HZY;
                HYU = HYM;
            } else {
                let HYO = Lanes([HHA[0], HHA[1], HHA[2], 0.0]);
                HYP = HHD;
                HYQ = HGY;
                HYR = HGZ;
                HYS = HHF;
                HYT = HYO;
                HYU = HHB;
            }
            let HYV = HXV * HET;
            let HYW = HEU * HXV;
            let HYX = HYW[3];
            let HYY = HYW[1];
            let HYZ = HYW[5];
            let HZA = HYW[4];
            let HZB = HYG[3];
            let HZC = HYG[1];
            let HZD = HYG[5];
            let HZE = HYG[4];
            let HZF = HYH[3];
            let HZG = HYH[1];
            let HZH = HYH[5];
            let HZI = HYH[4];
            let HZJ = GBZ * FGH;
            let HZK = FGI * GBZ;
            let HZL = GBZ * FLZ;
            let HZM = FMA * GBZ;
            let HZN = GBZ * FNO;
            let HZO = FNP * GBZ;
            let HZZ = Lanes([EDT[0], 0.0, EDT[1], EDT[2], EDT[3]]);
            let IAA = Lanes([EDV[0], EDV[1], 0.0, EDV[2], EDV[3]]);
            let IAB = AJX - AKD;
            let IAC = Lanes([AJY, 0.0]) - Lanes([0.0, AKF]);
            let IAD = ddt(68213, HYV);
            let IAE = HYW * GBO;
            let IAF = ddt(68215, HYE);
            let IAG = HYG * GBO;
            let IAH = ddt(68217, HYF);
            let IAI = HYH * GBO;
            let IAK = IAJ * HYQ;
            let IAL = HYT * IAJ;
            let IAM = GBZ * ddt(68223, IAK);
            let IAN = (IAL * GBO) * GBZ;
            let IAO = GBZ * IAK;
            let IAP = IAL * GBZ;
            let IAQ = IAJ * HYR;
            let IAR = HYU * IAJ;
            let IAS = GBZ * ddt(68230, IAQ);
            let IAT = (IAR * GBO) * GBZ;
            let IAU = GBZ * IAQ;
            let IAV = IAR * GBZ;
            let IAW = IAJ * HYP;
            let IAX = HYS * IAJ;
            let IAY = GBZ * ddt(68237, IAW);
            let IAZ = (IAX * GBO) * GBZ;
            let IBA = GBZ * IAW;
            let IBB = IAX * GBZ;
            let IBC = EDR * AMZ;
            let IBD = IBC * HHS;
            let IBE = HHT * IBC;
            let IBF;
            let IBG;
            if E != 0.0 {
                IBF = EDW;
                IBG = EDX;
            } else {
                IBF = A;
                IBG = BCT;
            }
            let IBL;
            let IBM;
            let IBN;
            let IBO;
            if F != 0.0 {
                let IBH = EDS + EDY;
                let IBI = HZZ + EDZ;
                let IBJ = EDU + EEA;
                let IBK = IAA + EEB;
                IBL = IBH;
                IBM = IBJ;
                IBN = IBI;
                IBO = IBK;
            } else {
                IBL = A;
                IBM = A;
                IBN = BCT;
                IBO = BCT;
            }
            let IBV;
            let IBW;
            let IBX;
            let IBY;
            let IBZ;
            let ICA;
            let ICB;
            let ICC;
            let ICD;
            let ICE;
            if HHU != 0.0 {
                let IBP = DYA + ELT;
                let IBQ = DYB + Lanes([0.0, ELU[0], ELU[1], 0.0, 0.0, ELU[2], ELU[3]]);
                let IBR = EDR * DXX;
                let IBS = DXY * EDR;
                IBV = IBP;
                IBW = IBR;
                IBX = ELV;
                IBY = A;
                IBZ = A;
                ICA = IBQ;
                ICB = IBS;
                ICC = ELW;
                ICD = EDG;
                ICE = COT;
            } else {
                let IBT = DYA + ELV;
                let IBU = DYB + Lanes([0.0, ELW[0], 0.0, 0.0, ELW[1], ELW[2], ELW[3]]);
                IBV = A;
                IBW = A;
                IBX = A;
                IBY = ELT;
                IBZ = IBT;
                ICA = COT;
                ICB = DXW;
                ICC = EDF;
                ICD = ELU;
                ICE = IBU;
            }
            let ICP;
            let ICQ;
            let ICR;
            let ICS;
            let ICT;
            let ICU;
            let ICV;
            let ICW;
            if ICF != 0.0 {
                let ICG = U / CLY;
                let ICH = ((CMB * ICG) * W) / CLY;
                let ICI = DXZ * (DZO - ALU);
                let ICJ = ICI * ICG;
                let ICK = ((Lanes([DZQ, 0.0]) - Lanes([0.0, ALV])) * DXZ) * ICG;
                let ICL = ICH * ICI;
                let ICM = Lanes([ICK[0], 0.0, ICK[1], 0.0, 0.0]) + Lanes([0.0, ICL[0], ICL[1], ICL[2], ICL[3]]);
                let IDE;
                let IDF;
                let IDG;
                let IDH;
                if ICN != 0.0 {
                    let ICY = U / COX;
                    let ICZ = ((CPB * ICY) * W) / COX;
                    let IDA = DXZ * (ALU - AJX);
                    let IDB = IDA * ICY;
                    let IDC = ((Lanes([0.0, ALV]) - Lanes([AJY, 0.0])) * DXZ) * ICY;
                    let IDD = Lanes([0.0, 0.0, IDC[0], IDC[1], 0.0, 0.0, 0.0]) + (ICZ * IDA);
                    IDE = ICY;
                    IDF = IDB;
                    IDG = ICZ;
                    IDH = IDD;
                } else {
                    IDE = A;
                    IDF = A;
                    IDG = COT;
                    IDH = COT;
                }
                ICP = ICG;
                ICQ = IDE;
                ICR = ICJ;
                ICS = IDF;
                ICT = ICH;
                ICU = IDG;
                ICV = ICM;
                ICW = IDH;
            } else {
                ICP = A;
                ICQ = A;
                ICR = A;
                ICS = A;
                ICT = COK;
                ICU = COT;
                ICV = ICO;
                ICW = COT;
            }
            let IDR;
            let IDS;
            let IDT;
            let IDU;
            let IDV;
            let IDW;
            let IDX;
            let IDY;
            if ICX != 0.0 {
                let IDI = U / CLZ;
                let IDJ = ((CMC * IDI) * W) / CLZ;
                let IDK = DXZ * (DZP - CJS);
                let IDL = IDK * IDI;
                let IDM = ((Lanes([DZR, 0.0]) - Lanes([0.0, CJT])) * DXZ) * IDI;
                let IDN = IDJ * IDK;
                let IDO = Lanes([IDM[0], 0.0, IDM[1], 0.0, 0.0]) + Lanes([0.0, IDN[0], IDN[1], IDN[2], IDN[3]]);
                let IEG;
                let IEH;
                let IEI;
                let IEJ;
                if IDP != 0.0 {
                    let IEA = U / COY;
                    let IEB = ((CPC * IEA) * W) / COY;
                    let IEC = DXZ * (CJS - AKD);
                    let IED = IEC * IEA;
                    let IEE = ((Lanes([0.0, CJT]) - Lanes([AKF, 0.0])) * DXZ) * IEA;
                    let IEF = Lanes([0.0, 0.0, 0.0, IEE[0], IEE[1], 0.0, 0.0]) + (IEB * IEC);
                    IEG = IEA;
                    IEH = IED;
                    IEI = IEB;
                    IEJ = IEF;
                } else {
                    IEG = A;
                    IEH = A;
                    IEI = COU;
                    IEJ = COU;
                }
                IDR = IDI;
                IDS = IEG;
                IDT = IDL;
                IDU = IEH;
                IDV = IDJ;
                IDW = IEI;
                IDX = IDO;
                IDY = IEJ;
            } else {
                IDR = A;
                IDS = A;
                IDT = A;
                IDU = A;
                IDV = COL;
                IDW = COU;
                IDX = IDQ;
                IDY = COU;
            }
            let IEM;
            let IEN;
            if IDZ != 0.0 {
                IEM = A;
                IEN = IEK;
            } else {
                let IEQ;
                let IER;
                if IEL != 0.0 {
                    IEQ = DTF;
                    IER = DTG;
                } else {
                    IEQ = IEP;
                    IER = BCT;
                }
                let IET = DXZ * (node_potentials[1] - ALL);
                let IEU = IET * IEQ;
                let IEV = ((Lanes([IES, 0.0]) - Lanes([0.0, ALM])) * DXZ) * IEQ;
                let IEW = IER * IET;
                let IEX = Lanes([IEV[0], 0.0, 0.0, 0.0, 0.0, IEV[1], 0.0]) + Lanes([0.0, IEW[0], IEW[1], IEW[2], IEW[3], 0.0, IEW[4]]);
                IEM = IEU;
                IEN = IEX;
            }
            let IFE;
            let IFF;
            if IEO != 0.0 {
                let IEY = (ALL - AJO) * DXZ;
                let IEZ = IEY * DTF;
                let IFA = ((Lanes([0.0, ALM]) - Lanes([AJR, 0.0])) * DXZ) * DTF;
                let IFB = DTG * IEY;
                let IFC = Lanes([0.0, 0.0, 0.0, IFA[0], IFA[1], 0.0]) + Lanes([IFB[0], IFB[1], IFB[2], IFB[3], 0.0, IFB[4]]);
                IFE = IEZ;
                IFF = IFC;
            } else {
                IFE = A;
                IFF = IFD;
            }
            let IFN;
            let IFO;
            let IFP;
            let IFQ;
            if IFG != 0.0 {
                let IFH = AJU * AMZ;
                let IFI = IFH * HHS;
                let IFJ = IFI * IAB;
                let IFK = IAC * IFI;
                let IFL = ((HHT * IFH) * IAB) + Lanes([0.0, 0.0, IFK[0], 0.0, IFK[1], 0.0, 0.0]);
                let IFT;
                let IFU;
                if ICF != 0.0 {
                    let IGQ;
                    let IGR;
                    if IFR != 0.0 {
                        let IFV = DZO - ALU;
                        let IFW = IFV * IFV;
                        let IFX = (Lanes([DZQ, 0.0]) - Lanes([0.0, ALV])) * IFV;
                        let IFY = (IFX + IFX) * ICP;
                        let IFZ = ICT * IFW;
                        let IGA = Lanes([IFY[0], 0.0, IFY[1], 0.0, 0.0]) + Lanes([0.0, IFZ[0], IFZ[1], IFZ[2], IFZ[3]]);
                        let IGB = ALU - AJX;
                        let IGC = IGB * IGB;
                        let IGD = (Lanes([0.0, ALV]) - Lanes([AJY, 0.0])) * IGB;
                        let IGE = (IGD + IGD) * ICQ;
                        let IGF = Lanes([0.0, 0.0, IGE[0], IGE[1], 0.0, 0.0, 0.0]) + (ICU * IGC);
                        let IGG = (IFJ + (IFW * ICP)) + (IGC * ICQ);
                        let IGH = (Lanes([0.0, IFL[0], IFL[1], IFL[2], IFL[3], IFL[4], IFL[5], IFL[6]]) + Lanes([IGA[0], 0.0, IGA[1], 0.0, IGA[2], 0.0, IGA[3], IGA[4]])) + Lanes([0.0, IGF[0], IGF[1], IGF[2], IGF[3], IGF[4], IGF[5], IGF[6]]);
                        IGQ = IGG;
                        IGR = IGH;
                    } else {
                        let IGI = DZO - ALU;
                        let IGJ = IGI * IGI;
                        let IGK = (Lanes([DZQ, 0.0]) - Lanes([0.0, ALV])) * IGI;
                        let IGL = (IGK + IGK) * ICP;
                        let IGM = ICT * IGJ;
                        let IGN = Lanes([IGL[0], 0.0, IGL[1], 0.0, 0.0]) + Lanes([0.0, IGM[0], IGM[1], IGM[2], IGM[3]]);
                        let IGO = IFJ + (IGJ * ICP);
                        let IGP = Lanes([0.0, IFL[0], IFL[1], IFL[2], IFL[3], IFL[4], IFL[5], IFL[6]]) + Lanes([IGN[0], 0.0, IGN[1], 0.0, IGN[2], 0.0, IGN[3], IGN[4]]);
                        IGQ = IGO;
                        IGR = IGP;
                    }
                    IFT = IGQ;
                    IFU = IGR;
                } else {
                    let IFS = Lanes([0.0, IFL[0], IFL[1], IFL[2], IFL[3], IFL[4], IFL[5], IFL[6]]);
                    IFT = IFJ;
                    IFU = IFS;
                }
                let IGU;
                let IGV;
                if ICX != 0.0 {
                    let IHX;
                    let IHY;
                    if IGS != 0.0 {
                        let IHC = DZP - CJS;
                        let IHD = IHC * IHC;
                        let IHE = (Lanes([DZR, 0.0]) - Lanes([0.0, CJT])) * IHC;
                        let IHF = (IHE + IHE) * IDR;
                        let IHG = IDV * IHD;
                        let IHH = Lanes([IHF[0], 0.0, IHF[1], 0.0, 0.0]) + Lanes([0.0, IHG[0], IHG[1], IHG[2], IHG[3]]);
                        let IHI = CJS - AKD;
                        let IHJ = IHI * IHI;
                        let IHK = (Lanes([0.0, CJT]) - Lanes([AKF, 0.0])) * IHI;
                        let IHL = (IHK + IHK) * IDS;
                        let IHM = Lanes([0.0, 0.0, 0.0, IHL[0], IHL[1], 0.0, 0.0]) + (IDW * IHJ);
                        let IHN = (IFT + (IHD * IDR)) + (IHJ * IDS);
                        let IHO = (Lanes([IFU[0], 0.0, IFU[1], IFU[2], IFU[3], IFU[4], IFU[5], 0.0, IFU[6], IFU[7]]) + Lanes([0.0, IHH[0], 0.0, IHH[1], 0.0, 0.0, 0.0, IHH[2], IHH[3], IHH[4]])) + Lanes([0.0, 0.0, IHM[0], IHM[1], IHM[2], 0.0, IHM[3], IHM[4], IHM[5], IHM[6]]);
                        IHX = IHN;
                        IHY = IHO;
                    } else {
                        let IHP = DZP - CJS;
                        let IHQ = IHP * IHP;
                        let IHR = (Lanes([DZR, 0.0]) - Lanes([0.0, CJT])) * IHP;
                        let IHS = (IHR + IHR) * IDR;
                        let IHT = IDV * IHQ;
                        let IHU = Lanes([IHS[0], 0.0, IHS[1], 0.0, 0.0]) + Lanes([0.0, IHT[0], IHT[1], IHT[2], IHT[3]]);
                        let IHV = IFT + (IHQ * IDR);
                        let IHW = Lanes([IFU[0], 0.0, IFU[1], IFU[2], IFU[3], IFU[4], IFU[5], 0.0, IFU[6], IFU[7]]) + Lanes([0.0, IHU[0], 0.0, IHU[1], 0.0, 0.0, 0.0, IHU[2], IHU[3], IHU[4]]);
                        IHX = IHV;
                        IHY = IHW;
                    }
                    IGU = IHX;
                    IGV = IHY;
                } else {
                    let IGT = Lanes([IFU[0], 0.0, IFU[1], IFU[2], IFU[3], IFU[4], IFU[5], 0.0, IFU[6], IFU[7]]);
                    IGU = IFT;
                    IGV = IGT;
                }
                let IGY = O * IGX;
                let IGZ = P * IGX;
                let IHA = ((O * IGW) + ddt(68537, IGY)) - IGU;
                let IHB = Lanes([0.0, 0.0, 0.0, ((P * IGW) + (IGZ * GBO)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) - IGV;
                IFN = IHA;
                IFO = IGY;
                IFP = IHB;
                IFQ = IGZ;
            } else {
                IFN = A;
                IFO = A;
                IFP = IFM;
                IFQ = N;
            }
            let IIS;
            let IIT;
            let IIU;
            let IIV;
            let IIW;
            let IIX;
            let IIY;
            let IIZ;
            let IJA;
            let IJB;
            if I != 0.0 {
                let IIA = (DXZ * (AJP - AKP)) * IHZ;
                let IIB = ((Lanes([AJS, 0.0]) - Lanes([0.0, AKR])) * DXZ) * IHZ;
                let IID = (DXZ * (CRW - AKP)) * IIC;
                let IIE = ((Lanes([CRY, 0.0]) - Lanes([0.0, AKR])) * DXZ) * IIC;
                let IIG = (DXZ * (CRW - AJP)) * IIF;
                let IIH = ((Lanes([CRY, 0.0]) - Lanes([0.0, AJS])) * DXZ) * IIF;
                let IIJ = (DXZ * (CRW - AKV)) * III;
                let IIK = ((Lanes([CRY, 0.0]) - Lanes([0.0, AKX])) * DXZ) * III;
                let IIM = (DXZ * (AJP - AKV)) * IIL;
                let IIN = ((Lanes([AJS, 0.0]) - Lanes([0.0, AKX])) * DXZ) * IIL;
                IIS = IIA;
                IIT = IID;
                IIU = IIG;
                IIV = IIJ;
                IIW = IIM;
                IIX = IIB;
                IIY = IIE;
                IIZ = IIH;
                IJA = IIK;
                IJB = IIN;
            } else {
                IIS = A;
                IIT = A;
                IIU = A;
                IIV = A;
                IIW = A;
                IIX = IIO;
                IIY = IIP;
                IIZ = COS;
                IJA = IIQ;
                IJB = IIR;
            }
            let IKA;
            let IKB;
            let IKC;
            let IKD;
            let IKE;
            let IKF;
            let IKG;
            let IKH;
            let IKI;
            let IKJ;
            let IKK;
            let IKL;
            let IKM;
            let IKN;
            let IKO;
            let IKP;
            let IKQ;
            let IKR;
            let IKS;
            let IKT;
            let IKU;
            let IKV;
            let IKW;
            let IKX;
            if I != 0.0 {
                let IJC = (AKS * DXZ) * B;
                let IJD = (EDR * ETE) + ((AKQ * DXZ) * B);
                let IJE = (ETF * EDR) + Lanes([0.0, IJC[0], IJC[1]]);
                let IJF = AJU * ddt(68626, HZJ);
                let IJG = (HZK * GBO) * AJU;
                let IJH = AJU * HZJ;
                let IJI = HZK * AJU;
                let ILG;
                let ILH;
                let ILI;
                let ILJ;
                let ILK;
                let ILL;
                if IJJ != 0.0 {
                    let IKZ = (AKY * DXZ) * B;
                    let ILA = (EDR * EZP) + ((AKW * DXZ) * B);
                    let ILB = (EZQ * EDR) + Lanes([0.0, IKZ[0], IKZ[1]]);
                    let ILC = AJU * ddt(68646, HZL);
                    let ILD = (HZM * GBO) * AJU;
                    let ILE = AJU * HZL;
                    let ILF = HZM * AJU;
                    ILG = ILA;
                    ILH = ILC;
                    ILI = ILE;
                    ILJ = ILB;
                    ILK = ILD;
                    ILL = ILF;
                } else {
                    ILG = A;
                    ILH = A;
                    ILI = A;
                    ILJ = EUT;
                    ILK = FIO;
                    ILL = FIO;
                }
                IKA = IJD;
                IKB = IJF;
                IKC = ILG;
                IKD = ILH;
                IKE = A;
                IKF = A;
                IKG = A;
                IKH = A;
                IKI = IJH;
                IKJ = ILI;
                IKK = A;
                IKL = A;
                IKM = IJE;
                IKN = IJG;
                IKO = ILJ;
                IKP = ILK;
                IKQ = ILM;
                IKR = ILN;
                IKS = EQY;
                IKT = FIO;
                IKU = IJI;
                IKV = ILL;
                IKW = EQY;
                IKX = FIO;
            } else {
                let IJK = ETF * EDR;
                let IJL = ((AKH - AKG) * DXZ) * B;
                let IJM = (EDR * ETE) + (((AJP - AKD) * DXZ) * B);
                let IJN = Lanes([IJK[0], IJK[1], 0.0, IJK[2]]) + Lanes([0.0, IJL[0], IJL[1], 0.0]);
                let IJO = EZQ * EDR;
                let IJP = ((AKA - AJZ) * DXZ) * B;
                let IJQ = (EDR * EZP) + (((AJP - AJX) * DXZ) * B);
                let IJR = Lanes([IJO[0], IJO[1], 0.0, IJO[2]]) + Lanes([0.0, IJP[0], IJP[1], 0.0]);
                let IJS = AJU * ddt(68674, HZJ);
                let IJT = (HZK * GBO) * AJU;
                let IJU = AJU * HZJ;
                let IJV = HZK * AJU;
                let IJW = AJU * ddt(68680, HZL);
                let IJX = (HZM * GBO) * AJU;
                let IJY = AJU * HZL;
                let IJZ = HZM * AJU;
                IKA = A;
                IKB = A;
                IKC = A;
                IKD = A;
                IKE = IJM;
                IKF = IJQ;
                IKG = IJS;
                IKH = IJW;
                IKI = A;
                IKJ = A;
                IKK = IJU;
                IKL = IJY;
                IKM = EQY;
                IKN = EQY;
                IKO = EUT;
                IKP = FIO;
                IKQ = IJN;
                IKR = IJR;
                IKS = IJT;
                IKT = IJX;
                IKU = EQY;
                IKV = FIO;
                IKW = IJV;
                IKX = IJZ;
            }
            let ILS;
            let ILT;
            if IKY != 0.0 {
                let ILP = ((ALB - DZO) * DXZ) * ILO;
                let ILQ = ((Lanes([0.0, ALD]) - Lanes([DZQ, 0.0])) * DXZ) * ILO;
                ILS = ILP;
                ILT = ILQ;
            } else {
                ILS = A;
                ILT = ILR;
            }
            let IMK;
            let IML;
            let IMM;
            let IMN;
            let IMO;
            let IMP;
            let IMQ;
            let IMR;
            let IMS;
            let IMT;
            let IMU;
            let IMV;
            if IKY != 0.0 {
                let ILV = (AKY * ILU) * B;
                let ILW = (EDR * EZP) + ((ILU * AKW) * B);
                let ILX = (EZQ * EDR) + Lanes([0.0, ILV[0], ILV[1]]);
                let ILZ = (ALE * ILY) * B;
                let IMA = (EDR * FAJ) + ((ILY * ALC) * B);
                let IMB = (FAK * EDR) + Lanes([0.0, ILZ[0], ILZ[1]]);
                let IMC = AJU * ddt(68741, HZL);
                let IMD = (HZM * GBO) * AJU;
                let IME = AJU * HZL;
                let IMF = HZM * AJU;
                let IMG = AJU * ddt(68747, HZN);
                let IMH = (HZO * GBO) * AJU;
                let IMI = AJU * HZN;
                let IMJ = HZO * AJU;
                IMK = ILW;
                IML = IMA;
                IMM = IMC;
                IMN = IMG;
                IMO = IME;
                IMP = IMI;
                IMQ = ILX;
                IMR = IMB;
                IMS = IMD;
                IMT = IMH;
                IMU = IMF;
                IMV = IMJ;
            } else {
                IMK = A;
                IML = A;
                IMM = A;
                IMN = A;
                IMO = A;
                IMP = A;
                IMQ = EUT;
                IMR = EUU;
                IMS = FIO;
                IMT = EUU;
                IMU = FIO;
                IMV = EUU;
            }
            let IMW = FSJ[0];
            let IMX = FSJ[1];
            let IMY = FSJ[2];
            let IMZ = FSJ[3];
            let INA = FSJ[4];
            let INB = FXL[0];
            let INC = FXL[1];
            let IND = FXL[2];
            let INE = FXL[3];
            let INF = FXL[4];
            let ING = FXL[5];
            let INH = FXM[0];
            let INI = FXM[1];
            let INJ = FXM[2];
            let INK = FXM[3];
            let INL = FXM[4];
            let INM = FXM[5];
            let INN = FXN[0];
            let INO = FXN[1];
            let INP = FXN[2];
            let INQ = FXN[3];
            let INR = FXN[4];
            let INS = FXN[5];
            let INT = FXO[0];
            let INU = FXO[1];
            let INV = FXO[2];
            let INW = FXO[3];
            let INX = FXO[4];
            let INY = FXO[5];
            let INZ = FXP[0];
            let IOA = FXP[1];
            let IOB = FXP[2];
            let IOC = FXP[3];
            let IOD = FXP[4];
            let IOE = FXP[5];
            let IOF = GBF;
            let IOG = GBL;
            let IOH = IAE[0];
            let IOI = IAE[1];
            let IOJ = IAE[2];
            let IOK = IAE[3];
            let IOL = IAE[4];
            let IOM = IAE[5];
            let ION = IAG[0];
            let IOO = IAG[1];
            let IOP = IAG[2];
            let IOQ = IAG[3];
            let IOR = IAG[4];
            let IOS = IAG[5];
            let IOT = IAI[0];
            let IOU = IAI[1];
            let IOV = IAI[2];
            let IOW = IAI[3];
            let IOX = IAI[4];
            let IOY = IAI[5];
            let IOZ = IAN[0];
            let IPA = IAN[1];
            let IPB = IAN[2];
            let IPC = IAN[3];
            let IPD = IAT[0];
            let IPE = IAT[1];
            let IPF = IAT[2];
            let IPG = IAT[3];
            let IPH = IAT[4];
            let IPI = IAZ[0];
            let IPJ = IAZ[1];
            let IPK = IAZ[2];
            let IPL = IAZ[3];
            let IPM = IAZ[4];
            let IPN = IAZ[5];
            let IPO = IBE[0];
            let IPP = IBE[1];
            let IPQ = IBE[2];
            let IPR = IBE[3];
            let IPS = IBE[4];
            let IPT = IBE[5];
            let IPU = IBE[6];
            let IPV = IBG[0];
            let IPW = IBG[1];
            let IPX = IBG[2];
            let IPY = IBG[3];
            let IPZ = IBG[4];
            let IQA = IBN[0];
            let IQB = IBN[1];
            let IQC = IBN[2];
            let IQD = IBN[3];
            let IQE = IBN[4];
            let IQF = IBO[0];
            let IQG = IBO[1];
            let IQH = IBO[2];
            let IQI = IBO[3];
            let IQJ = IBO[4];
            let IQK = ICA[0];
            let IQL = ICA[1];
            let IQM = ICA[2];
            let IQN = ICA[3];
            let IQO = ICA[4];
            let IQP = ICA[5];
            let IQQ = ICA[6];
            let IQR = ICB[0];
            let IQS = ICB[1];
            let IQT = ICB[2];
            let IQU = ICB[3];
            let IQV = ICB[4];
            let IQW = ICB[5];
            let IQX = ICB[6];
            let IQY = ICB[7];
            let IQZ = ICB[8];
            let IRA = ICC[0];
            let IRB = ICC[1];
            let IRC = ICC[2];
            let IRD = ICC[3];
            let IRE = ICD[0];
            let IRF = ICD[1];
            let IRG = ICD[2];
            let IRH = ICD[3];
            let IRI = ICE[0];
            let IRJ = ICE[1];
            let IRK = ICE[2];
            let IRL = ICE[3];
            let IRM = ICE[4];
            let IRN = ICE[5];
            let IRO = ICE[6];
            let IRP = ICV[0];
            let IRQ = ICV[1];
            let IRR = ICV[2];
            let IRS = ICV[3];
            let IRT = ICV[4];
            let IRU = ICW[0];
            let IRV = ICW[1];
            let IRW = ICW[2];
            let IRX = ICW[3];
            let IRY = ICW[4];
            let IRZ = ICW[5];
            let ISA = ICW[6];
            let ISB = IDX[0];
            let ISC = IDX[1];
            let ISD = IDX[2];
            let ISE = IDX[3];
            let ISF = IDX[4];
            let ISG = IDY[0];
            let ISH = IDY[1];
            let ISI = IDY[2];
            let ISJ = IDY[3];
            let ISK = IDY[4];
            let ISL = IDY[5];
            let ISM = IDY[6];
            let ISN = IEN[0];
            let ISO = IEN[1];
            let ISP = IEN[2];
            let ISQ = IEN[3];
            let ISR = IEN[4];
            let ISS = IEN[5];
            let IST = IEN[6];
            let ISU = IFF[0];
            let ISV = IFF[1];
            let ISW = IFF[2];
            let ISX = IFF[3];
            let ISY = IFF[4];
            let ISZ = IFF[5];
            let ITA = IFP[0];
            let ITB = IFP[1];
            let ITC = IFP[2];
            let ITD = IFP[3];
            let ITE = IFP[4];
            let ITF = IFP[5];
            let ITG = IFP[6];
            let ITH = IFP[7];
            let ITI = IFP[8];
            let ITJ = IFP[9];
            let ITK = IIX[0];
            let ITL = IIX[1];
            let ITM = IIY[0];
            let ITN = IIY[1];
            let ITO = IIZ[0];
            let ITP = IIZ[1];
            let ITQ = IJA[0];
            let ITR = IJA[1];
            let ITS = IJB[0];
            let ITT = IJB[1];
            let ITU = IKM[0];
            let ITV = IKM[1];
            let ITW = IKM[2];
            let ITX = IKN[0];
            let ITY = IKN[1];
            let ITZ = IKN[2];
            let IUA = IKO[0];
            let IUB = IKO[1];
            let IUC = IKO[2];
            let IUD = IKP[0];
            let IUE = IKP[1];
            let IUF = IKP[2];
            let IUG = IKP[3];
            let IUH = IKP[4];
            let IUI = IKQ[0];
            let IUJ = IKQ[1];
            let IUK = IKQ[2];
            let IUL = IKQ[3];
            let IUM = IKR[0];
            let IUN = IKR[1];
            let IUO = IKR[2];
            let IUP = IKR[3];
            let IUQ = IKS[0];
            let IUR = IKS[1];
            let IUS = IKS[2];
            let IUT = IKT[0];
            let IUU = IKT[1];
            let IUV = IKT[2];
            let IUW = IKT[3];
            let IUX = IKT[4];
            let IUY = ILT[0];
            let IUZ = ILT[1];
            let IVA = IMQ[0];
            let IVB = IMQ[1];
            let IVC = IMQ[2];
            let IVD = IMR[0];
            let IVE = IMR[1];
            let IVF = IMR[2];
            let IVG = IMS[0];
            let IVH = IMS[1];
            let IVI = IMS[2];
            let IVJ = IMS[3];
            let IVK = IMS[4];
            let IVL = IMT[0];
            let IVM = IMT[1];
            let IVN = IMT[2];
            let IVO = FXQ[0];
            let IVP = FXQ[1];
            let IVQ = FXQ[2];
            let IVR = FXQ[3];
            let IVS = FXQ[4];
            let IVT = FXQ[5];
            let IVU = FXR[0];
            let IVV = FXR[1];
            let IVW = FXR[2];
            let IVX = FXR[3];
            let IVY = FXR[4];
            let IVZ = FXR[5];
            let IWA = FXS[0];
            let IWB = FXS[1];
            let IWC = FXS[2];
            let IWD = FXS[3];
            let IWE = FXS[4];
            let IWF = FXS[5];
            let IWG = HYW[0];
            let IWH = HYW[2];
            let IWI = HYG[0];
            let IWJ = HYG[2];
            let IWK = HYH[0];
            let IWL = HYH[2];
            let IWM = IAP[0];
            let IWN = IAP[1];
            let IWO = IAP[2];
            let IWP = IAP[3];
            let IWQ = IAV[0];
            let IWR = IAV[1];
            let IWS = IAV[2];
            let IWT = IAV[3];
            let IWU = IAV[4];
            let IWV = IBB[0];
            let IWW = IBB[1];
            let IWX = IBB[2];
            let IWY = IBB[3];
            let IWZ = IBB[4];
            let IXA = IBB[5];
            let IXB = IFQ;
            let IXC = IKU[0];
            let IXD = IKU[1];
            let IXE = IKU[2];
            let IXF = IKV[0];
            let IXG = IKV[1];
            let IXH = IKV[2];
            let IXI = IKV[3];
            let IXJ = IKV[4];
            let IXK = IKW[0];
            let IXL = IKW[1];
            let IXM = IKW[2];
            let IXN = IKX[0];
            let IXO = IKX[1];
            let IXP = IKX[2];
            let IXQ = IKX[3];
            let IXR = IKX[4];
            let IXS = IMU[0];
            let IXT = IMU[1];
            let IXU = IMU[2];
            let IXV = IMU[3];
            let IXW = IMU[4];
            let IXX = IMV[0];
            let IXY = IMV[1];
            let IXZ = IMV[2];
        stamper.stamp_potential_branch_local(Some(4), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[928],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(7),
            multiplicity * (FSI),
            [4, 5, 7, 9, 11],
            [IMW, IMX, IMY, IMZ, INA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (staged[929]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (staged[930]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (staged[931]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(16),
            None,
            multiplicity * (staged[932]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(15),
            None,
            multiplicity * (staged[933]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(15),
            None,
            multiplicity * (FXD),
            [4, 5, 7, 9, 11, 16],
            [INB, INC, IND, INE, INF, ING],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(15),
            None,
            multiplicity * (FXE),
            [4, 5, 7, 9, 11, 15],
            [INH, INI, INJ, INK, INL, INM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (staged[934]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * (FXF),
            [4, 5, 7, 9, 11, 16],
            [INN, INO, INP, INQ, INR, INS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (FXG),
            [4, 5, 7, 9, 11, 15],
            [INT, INU, INV, INW, INX, INY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(5),
            multiplicity * (FXH),
            [4, 5, 7, 9, 11, 15],
            [INZ, IOA, IOB, IOC, IOD, IOE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (GBC),
            [16],
            [IOF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (GBI),
            [15],
            [IOG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (staged[935]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(5),
            multiplicity * (staged[936]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(11),
            multiplicity * (staged[937]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(7),
            multiplicity * (staged[938]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(11),
            multiplicity * (IAD),
            [4, 5, 6, 7, 9, 11],
            [IOH, IOI, IOJ, IOK, IOL, IOM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (IAF),
            [4, 5, 6, 7, 9, 11],
            [ION, IOO, IOP, IOQ, IOR, IOS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(11),
            multiplicity * (IAH),
            [4, 5, 6, 7, 9, 11],
            [IOT, IOU, IOV, IOW, IOX, IOY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(7),
            multiplicity * (IAM),
            [4, 7, 10, 11],
            [IOZ, IPA, IPB, IPC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (IAS),
            [4, 5, 6, 10, 11],
            [IPD, IPE, IPF, IPG, IPH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(11),
            multiplicity * (IAY),
            [4, 5, 6, 7, 10, 11],
            [IPI, IPJ, IPK, IPL, IPM, IPN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(7),
            multiplicity * (IBD),
            [3, 4, 5, 6, 7, 9, 11],
            [IPO, IPP, IPQ, IPR, IPS, IPT, IPU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(11),
            multiplicity * (IBF),
            [4, 5, 7, 9, 11],
            [IPV, IPW, IPX, IPY, IPZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (IBL),
            [4, 5, 7, 9, 11],
            [IQA, IQB, IQC, IQD, IQE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(5),
            multiplicity * (IBM),
            [4, 5, 7, 9, 11],
            [IQF, IQG, IQH, IQI, IQJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(11),
            multiplicity * (IBV),
            [3, 4, 5, 6, 7, 9, 11],
            [IQK, IQL, IQM, IQN, IQO, IQP, IQQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(11),
            multiplicity * (IBW),
            [0, 2, 3, 4, 5, 6, 7, 9, 11],
            [IQR, IQS, IQT, IQU, IQV, IQW, IQX, IQY, IQZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(11),
            multiplicity * (IBX),
            [4, 7, 9, 11],
            [IRA, IRB, IRC, IRD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(11),
            multiplicity * (IBY),
            [4, 5, 9, 11],
            [IRE, IRF, IRG, IRH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(11),
            multiplicity * (IBZ),
            [3, 4, 5, 6, 7, 9, 11],
            [IRI, IRJ, IRK, IRL, IRM, IRN, IRO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(6),
            multiplicity * (ICR),
            [0, 4, 6, 9, 11],
            [IRP, IRQ, IRR, IRS, IRT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(6),
            multiplicity * (staged[939]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(5),
            multiplicity * (ICS),
            [3, 4, 5, 6, 7, 9, 11],
            [IRU, IRV, IRW, IRX, IRY, IRZ, ISA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (staged[940]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (staged[941]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(6), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[942],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(0), Some(6), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[943],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(6), Some(5), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[944],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(8),
            multiplicity * (IDT),
            [2, 4, 8, 9, 11],
            [ISB, ISC, ISD, ISE, ISF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(8),
            multiplicity * (staged[945]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(7),
            multiplicity * (IDU),
            [3, 4, 5, 7, 8, 9, 11],
            [ISG, ISH, ISI, ISJ, ISK, ISL, ISM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(7),
            multiplicity * (staged[946]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(7),
            multiplicity * (staged[947]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(8), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[948],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), Some(8), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[949],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(7), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[950],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(1), Some(10), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            staged[951],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(1),
            Some(10),
            multiplicity * (IEM),
            [1, 4, 5, 7, 9, 10, 11],
            [ISN, ISO, ISP, ISQ, ISR, ISS, IST],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(10),
            multiplicity * (staged[952]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(9),
            multiplicity * (IFE),
            [4, 5, 7, 9, 10, 11],
            [ISU, ISV, ISW, ISX, ISY, ISZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(9), 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            8,
            staged[953],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(4),
            None,
            multiplicity * (IFN),
            [0, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            [ITA, ITB, ITC, ITD, ITE, ITF, ITG, ITH, ITI, ITJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), None, 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            staged[954],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(11),
            Some(12),
            multiplicity * (IIS),
            [11, 12],
            [ITK, ITL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(12),
            multiplicity * (IIT),
            [3, 12],
            [ITM, ITN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(11),
            multiplicity * (IIU),
            [3, 11],
            [ITO, ITP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(13),
            multiplicity * (IIV),
            [3, 13],
            [ITQ, ITR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(11),
            Some(13),
            multiplicity * (IIW),
            [11, 13],
            [ITS, ITT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            Some(11),
            multiplicity * (staged[955]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            Some(3),
            multiplicity * (staged[956]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(11),
            multiplicity * (staged[957]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(13),
            Some(11),
            multiplicity * (staged[958]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(13),
            Some(3),
            multiplicity * (staged[959]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(3), Some(12), 10, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            10,
            staged[960],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(11), 11, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            11,
            staged[961],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(13), 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            staged[962],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(7),
            multiplicity * (IKA),
            [4, 7, 12],
            [ITU, ITV, ITW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(7),
            multiplicity * (IKB),
            [4, 7, 12],
            [ITX, ITY, ITZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(13),
            Some(5),
            multiplicity * (IKC),
            [4, 5, 13],
            [IUA, IUB, IUC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(13),
            Some(5),
            multiplicity * (IKD),
            [4, 5, 6, 11, 13],
            [IUD, IUE, IUF, IUG, IUH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(7),
            multiplicity * (IKE),
            [4, 7, 11, 12],
            [IUI, IUJ, IUK, IUL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(5),
            multiplicity * (IKF),
            [4, 5, 11, 13],
            [IUM, IUN, IUO, IUP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(7),
            multiplicity * (IKG),
            [4, 7, 12],
            [IUQ, IUR, IUS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            Some(5),
            multiplicity * (IKH),
            [4, 5, 6, 11, 13],
            [IUT, IUU, IUV, IUW, IUX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(14),
            Some(0),
            multiplicity * (ILS),
            [0, 14],
            [IUY, IUZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            Some(0),
            multiplicity * (staged[963]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(14), 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            13,
            staged[964],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(13),
            Some(5),
            multiplicity * (IMK),
            [4, 5, 13],
            [IVA, IVB, IVC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(13),
            Some(14),
            multiplicity * (IML),
            [4, 13, 14],
            [IVD, IVE, IVF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(13),
            Some(5),
            multiplicity * (IMM),
            [4, 5, 6, 11, 13],
            [IVG, IVH, IVI, IVJ, IVK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(13),
            Some(14),
            multiplicity * (IMN),
            [4, 13, 14],
            [IVL, IVM, IVN],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = staged[928];
        self.canonical_reactive[1] = FSI;
        self.canonical_reactive[2] = staged[929];
        self.canonical_reactive[3] = staged[930];
        self.canonical_reactive[4] = staged[931];
        self.canonical_reactive[5] = staged[932];
        self.canonical_reactive[6] = staged[933];
        self.canonical_reactive[7] = FXD;
        self.canonical_reactive[8] = FXI;
        self.canonical_reactive[9] = IVO;
        self.canonical_reactive[10] = IVP;
        self.canonical_reactive[11] = IVQ;
        self.canonical_reactive[12] = IVR;
        self.canonical_reactive[13] = IVS;
        self.canonical_reactive[14] = IVT;
        self.canonical_reactive[15] = staged[934];
        self.canonical_reactive[16] = FXF;
        self.canonical_reactive[17] = FXJ;
        self.canonical_reactive[18] = IVU;
        self.canonical_reactive[19] = IVV;
        self.canonical_reactive[20] = IVW;
        self.canonical_reactive[21] = IVX;
        self.canonical_reactive[22] = IVY;
        self.canonical_reactive[23] = IVZ;
        self.canonical_reactive[24] = FXK;
        self.canonical_reactive[25] = IWA;
        self.canonical_reactive[26] = IWB;
        self.canonical_reactive[27] = IWC;
        self.canonical_reactive[28] = IWD;
        self.canonical_reactive[29] = IWE;
        self.canonical_reactive[30] = IWF;
        self.canonical_reactive[31] = GBC;
        self.canonical_reactive[32] = GBI;
        self.canonical_reactive[33] = staged[935];
        self.canonical_reactive[34] = staged[936];
        self.canonical_reactive[35] = staged[937];
        self.canonical_reactive[36] = staged[938];
        self.canonical_reactive[37] = HYV;
        self.canonical_reactive[38] = IWG;
        self.canonical_reactive[39] = HYY;
        self.canonical_reactive[40] = IWH;
        self.canonical_reactive[41] = HYX;
        self.canonical_reactive[42] = HZA;
        self.canonical_reactive[43] = HYZ;
        self.canonical_reactive[44] = HYE;
        self.canonical_reactive[45] = IWI;
        self.canonical_reactive[46] = HZC;
        self.canonical_reactive[47] = IWJ;
        self.canonical_reactive[48] = HZB;
        self.canonical_reactive[49] = HZE;
        self.canonical_reactive[50] = HZD;
        self.canonical_reactive[51] = HYF;
        self.canonical_reactive[52] = IWK;
        self.canonical_reactive[53] = HZG;
        self.canonical_reactive[54] = IWL;
        self.canonical_reactive[55] = HZF;
        self.canonical_reactive[56] = HZI;
        self.canonical_reactive[57] = HZH;
        self.canonical_reactive[58] = IAO;
        self.canonical_reactive[59] = IWM;
        self.canonical_reactive[60] = IWN;
        self.canonical_reactive[61] = IWO;
        self.canonical_reactive[62] = IWP;
        self.canonical_reactive[63] = IAU;
        self.canonical_reactive[64] = IWQ;
        self.canonical_reactive[65] = IWR;
        self.canonical_reactive[66] = IWS;
        self.canonical_reactive[67] = IWT;
        self.canonical_reactive[68] = IWU;
        self.canonical_reactive[69] = IBA;
        self.canonical_reactive[70] = IWV;
        self.canonical_reactive[71] = IWW;
        self.canonical_reactive[72] = IWX;
        self.canonical_reactive[73] = IWY;
        self.canonical_reactive[74] = IWZ;
        self.canonical_reactive[75] = IXA;
        self.canonical_reactive[76] = IBD;
        self.canonical_reactive[77] = IBF;
        self.canonical_reactive[78] = IBL;
        self.canonical_reactive[79] = IBM;
        self.canonical_reactive[80] = IBV;
        self.canonical_reactive[81] = IBW;
        self.canonical_reactive[82] = IBX;
        self.canonical_reactive[83] = IBY;
        self.canonical_reactive[84] = IBZ;
        self.canonical_reactive[85] = ICR;
        self.canonical_reactive[86] = staged[939];
        self.canonical_reactive[87] = ICS;
        self.canonical_reactive[88] = staged[940];
        self.canonical_reactive[89] = staged[941];
        self.canonical_reactive[90] = staged[942];
        self.canonical_reactive[91] = staged[943];
        self.canonical_reactive[92] = staged[944];
        self.canonical_reactive[93] = IDT;
        self.canonical_reactive[94] = staged[945];
        self.canonical_reactive[95] = IDU;
        self.canonical_reactive[96] = staged[946];
        self.canonical_reactive[97] = staged[947];
        self.canonical_reactive[98] = staged[948];
        self.canonical_reactive[99] = staged[949];
        self.canonical_reactive[100] = staged[950];
        self.canonical_reactive[101] = staged[951];
        self.canonical_reactive[102] = IEM;
        self.canonical_reactive[103] = staged[952];
        self.canonical_reactive[104] = IFE;
        self.canonical_reactive[105] = staged[953];
        self.canonical_reactive[106] = IFO;
        self.canonical_reactive[107] = IXB;
        self.canonical_reactive[108] = staged[954];
        self.canonical_reactive[109] = IIS;
        self.canonical_reactive[110] = IIT;
        self.canonical_reactive[111] = IIU;
        self.canonical_reactive[112] = IIV;
        self.canonical_reactive[113] = IIW;
        self.canonical_reactive[114] = staged[955];
        self.canonical_reactive[115] = staged[956];
        self.canonical_reactive[116] = staged[957];
        self.canonical_reactive[117] = staged[958];
        self.canonical_reactive[118] = staged[959];
        self.canonical_reactive[119] = staged[960];
        self.canonical_reactive[120] = staged[961];
        self.canonical_reactive[121] = staged[962];
        self.canonical_reactive[122] = IKA;
        self.canonical_reactive[123] = IKI;
        self.canonical_reactive[124] = IXC;
        self.canonical_reactive[125] = IXD;
        self.canonical_reactive[126] = IXE;
        self.canonical_reactive[127] = IKC;
        self.canonical_reactive[128] = IKJ;
        self.canonical_reactive[129] = IXF;
        self.canonical_reactive[130] = IXG;
        self.canonical_reactive[131] = IXH;
        self.canonical_reactive[132] = IXI;
        self.canonical_reactive[133] = IXJ;
        self.canonical_reactive[134] = IKE;
        self.canonical_reactive[135] = IKF;
        self.canonical_reactive[136] = IKK;
        self.canonical_reactive[137] = IXK;
        self.canonical_reactive[138] = IXL;
        self.canonical_reactive[139] = IXM;
        self.canonical_reactive[140] = IKL;
        self.canonical_reactive[141] = IXN;
        self.canonical_reactive[142] = IXO;
        self.canonical_reactive[143] = IXP;
        self.canonical_reactive[144] = IXQ;
        self.canonical_reactive[145] = IXR;
        self.canonical_reactive[146] = ILS;
        self.canonical_reactive[147] = staged[963];
        self.canonical_reactive[148] = staged[964];
        self.canonical_reactive[149] = IMK;
        self.canonical_reactive[150] = IML;
        self.canonical_reactive[151] = IMO;
        self.canonical_reactive[152] = IXS;
        self.canonical_reactive[153] = IXT;
        self.canonical_reactive[154] = IXU;
        self.canonical_reactive[155] = IXV;
        self.canonical_reactive[156] = IXW;
        self.canonical_reactive[157] = IMP;
        self.canonical_reactive[158] = IXX;
        self.canonical_reactive[159] = IXY;
        self.canonical_reactive[160] = IXZ;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(15),
            None,
            &[4, 5, 7, 9, 11, 15],
            &[cached[9], cached[10], cached[11], cached[12], cached[13], cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(7),
            &[4, 5, 7, 9, 11, 15],
            &[cached[18], cached[19], cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(5),
            &[4, 5, 7, 9, 11, 15],
            &[cached[25], cached[26], cached[27], cached[28], cached[29], cached[30]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(11),
            &[4, 5, 6, 7, 9, 11],
            &[cached[38], cached[39], cached[40], cached[41], cached[42], cached[43]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(11),
            &[4, 5, 6, 7, 9, 11],
            &[cached[45], cached[46], cached[47], cached[48], cached[49], cached[50]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(11),
            &[4, 5, 6, 7, 9, 11],
            &[cached[52], cached[53], cached[54], cached[55], cached[56], cached[57]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(7),
            &[4, 7, 10, 11],
            &[cached[59], cached[60], cached[61], cached[62]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(5),
            &[4, 5, 6, 10, 11],
            &[cached[64], cached[65], cached[66], cached[67], cached[68]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(11),
            &[4, 5, 6, 7, 10, 11],
            &[cached[70], cached[71], cached[72], cached[73], cached[74], cached[75]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[107]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            Some(7),
            &[4, 7, 12],
            &[cached[124], cached[125], cached[126]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            Some(5),
            &[4, 5, 6, 11, 13],
            &[cached[129], cached[130], cached[131], cached[132], cached[133]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(7),
            &[4, 7, 12],
            &[cached[137], cached[138], cached[139]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(5),
            &[4, 5, 6, 11, 13],
            &[cached[141], cached[142], cached[143], cached[144], cached[145]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            Some(5),
            &[4, 5, 6, 11, 13],
            &[cached[152], cached[153], cached[154], cached[155], cached[156]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            Some(14),
            &[4, 13, 14],
            &[cached[158], cached[159], cached[160]],
            &[],
            &[],
            multiplicity,
        );
    }

}
