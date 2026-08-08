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
        let mut key = Vec::with_capacity(944);
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
        self.canonical_staged[306] = values[1];
        self.canonical_staged[308] = values[2];
        self.canonical_staged[307] = values[3];
        self.canonical_staged[309] = values[4];
        self.canonical_staged[312] = values[5];
        self.canonical_staged[319] = values[6];
        self.canonical_staged[324] = values[7];
        self.canonical_staged[315] = values[8];
        self.canonical_staged[321] = values[9];
        self.canonical_staged[325] = values[10];
        self.canonical_staged[326] = values[11];
        self.canonical_staged[313] = values[12];
        self.canonical_staged[314] = values[13];
        self.canonical_staged[330] = values[14];
        self.canonical_staged[331] = values[15];
        self.canonical_staged[332] = values[16];
        self.canonical_staged[333] = values[17];
        self.canonical_staged[334] = values[18];
        self.canonical_staged[335] = values[19];
        self.canonical_staged[1] = values[20];
        self.canonical_staged[2] = values[21];
        self.canonical_staged[310] = values[22];
        self.canonical_staged[336] = values[23];
        self.canonical_staged[3] = values[24];
        self.canonical_staged[4] = values[25];
        self.canonical_staged[5] = values[26];
        self.canonical_staged[6] = values[27];
        self.canonical_staged[7] = values[28];
        self.canonical_staged[8] = values[29];
        self.canonical_staged[339] = values[30];
        self.canonical_staged[340] = values[31];
        self.canonical_staged[342] = values[32];
        self.canonical_staged[343] = values[33];
        self.canonical_staged[344] = values[34];
        self.canonical_staged[345] = values[35];
        self.canonical_staged[346] = values[36];
        self.canonical_staged[347] = values[37];
        self.canonical_staged[10] = values[38];
        self.canonical_staged[9] = values[39];
        self.canonical_staged[12] = values[40];
        self.canonical_staged[11] = values[41];
        self.canonical_staged[13] = values[42];
        self.canonical_staged[14] = values[43];
        self.canonical_staged[15] = values[44];
        self.canonical_staged[348] = values[45];
        self.canonical_staged[349] = values[46];
        self.canonical_staged[17] = values[47];
        self.canonical_staged[16] = values[48];
        self.canonical_staged[350] = values[49];
        self.canonical_staged[351] = values[50];
        self.canonical_staged[352] = values[51];
        self.canonical_staged[19] = values[52];
        self.canonical_staged[18] = values[53];
        self.canonical_staged[20] = values[54];
        self.canonical_staged[353] = values[55];
        self.canonical_staged[354] = values[56];
        self.canonical_staged[355] = values[57];
        self.canonical_staged[21] = values[58];
        self.canonical_staged[22] = values[59];
        self.canonical_staged[23] = values[60];
        self.canonical_staged[356] = values[61];
        self.canonical_staged[357] = values[62];
        self.canonical_staged[358] = values[63];
        self.canonical_staged[359] = values[64];
        self.canonical_staged[360] = values[65];
        self.canonical_staged[26] = values[66];
        self.canonical_staged[25] = values[67];
        self.canonical_staged[24] = values[68];
        self.canonical_staged[27] = values[69];
        self.canonical_staged[28] = values[70];
        self.canonical_staged[361] = values[71];
        self.canonical_staged[362] = values[72];
        self.canonical_staged[363] = values[73];
        self.canonical_staged[364] = values[74];
        self.canonical_staged[365] = values[75];
        self.canonical_staged[33] = values[76];
        self.canonical_staged[30] = values[77];
        self.canonical_staged[29] = values[78];
        self.canonical_staged[32] = values[79];
        self.canonical_staged[31] = values[80];
        self.canonical_staged[366] = values[81];
        self.canonical_staged[367] = values[82];
        self.canonical_staged[368] = values[83];
        self.canonical_staged[369] = values[84];
        self.canonical_staged[370] = values[85];
        self.canonical_staged[35] = values[86];
        self.canonical_staged[34] = values[87];
        self.canonical_staged[36] = values[88];
        self.canonical_staged[38] = values[89];
        self.canonical_staged[37] = values[90];
        self.canonical_staged[39] = values[91];
        self.canonical_staged[40] = values[92];
        self.canonical_staged[41] = values[93];
        self.canonical_staged[42] = values[94];
        self.canonical_staged[43] = values[95];
        self.canonical_staged[44] = values[96];
        self.canonical_staged[372] = values[97];
        self.canonical_staged[45] = values[98];
        self.canonical_staged[46] = values[99];
        self.canonical_staged[311] = values[100];
        self.canonical_staged[119] = values[101];
        self.canonical_staged[118] = values[102];
        self.canonical_staged[221] = values[103];
        self.canonical_staged[56] = values[104];
        self.canonical_staged[63] = values[105];
        self.canonical_staged[55] = values[106];
        self.canonical_staged[58] = values[107];
        self.canonical_staged[57] = values[108];
        self.canonical_staged[101] = values[109];
        self.canonical_staged[386] = values[110];
        self.canonical_staged[59] = values[111];
        self.canonical_staged[60] = values[112];
        self.canonical_staged[62] = values[113];
        self.canonical_staged[61] = values[114];
        self.canonical_staged[64] = values[115];
        self.canonical_staged[65] = values[116];
        self.canonical_staged[316] = values[117];
        self.canonical_staged[317] = values[118];
        self.canonical_staged[72] = values[119];
        self.canonical_staged[75] = values[120];
        self.canonical_staged[73] = values[121];
        self.canonical_staged[388] = values[122];
        self.canonical_staged[76] = values[123];
        self.canonical_staged[318] = values[124];
        self.canonical_staged[389] = values[125];
        self.canonical_staged[80] = values[126];
        self.canonical_staged[81] = values[127];
        self.canonical_staged[390] = values[128];
        self.canonical_staged[391] = values[129];
        self.canonical_staged[82] = values[130];
        self.canonical_staged[83] = values[131];
        self.canonical_staged[85] = values[132];
        self.canonical_staged[87] = values[133];
        self.canonical_staged[88] = values[134];
        self.canonical_staged[320] = values[135];
        self.canonical_staged[91] = values[136];
        self.canonical_staged[92] = values[137];
        self.canonical_staged[93] = values[138];
        self.canonical_staged[94] = values[139];
        self.canonical_staged[97] = values[140];
        self.canonical_staged[98] = values[141];
        self.canonical_staged[99] = values[142];
        self.canonical_staged[183] = values[143];
        self.canonical_staged[184] = values[144];
        self.canonical_staged[392] = values[145];
        self.canonical_staged[178] = values[146];
        self.canonical_staged[179] = values[147];
        self.canonical_staged[102] = values[148];
        self.canonical_staged[322] = values[149];
        self.canonical_staged[323] = values[150];
        self.canonical_staged[106] = values[151];
        self.canonical_staged[112] = values[152];
        self.canonical_staged[220] = values[153];
        self.canonical_staged[206] = values[154];
        self.canonical_staged[214] = values[155];
        self.canonical_staged[218] = values[156];
        self.canonical_staged[393] = values[157];
        self.canonical_staged[219] = values[158];
        self.canonical_staged[210] = values[159];
        self.canonical_staged[394] = values[160];
        self.canonical_staged[208] = values[161];
        self.canonical_staged[209] = values[162];
        self.canonical_staged[395] = values[163];
        self.canonical_staged[207] = values[164];
        self.canonical_staged[115] = values[165];
        self.canonical_staged[116] = values[166];
        self.canonical_staged[117] = values[167];
        self.canonical_staged[120] = values[168];
        self.canonical_staged[121] = values[169];
        self.canonical_staged[122] = values[170];
        self.canonical_staged[124] = values[171];
        self.canonical_staged[123] = values[172];
        self.canonical_staged[125] = values[173];
        self.canonical_staged[128] = values[174];
        self.canonical_staged[133] = values[175];
        self.canonical_staged[132] = values[176];
        self.canonical_staged[260] = values[177];
        self.canonical_staged[135] = values[178];
        self.canonical_staged[137] = values[179];
        self.canonical_staged[400] = values[180];
        self.canonical_staged[139] = values[181];
        self.canonical_staged[140] = values[182];
        self.canonical_staged[143] = values[183];
        self.canonical_staged[144] = values[184];
        self.canonical_staged[151] = values[185];
        self.canonical_staged[158] = values[186];
        self.canonical_staged[170] = values[187];
        self.canonical_staged[174] = values[188];
        self.canonical_staged[175] = values[189];
        self.canonical_staged[177] = values[190];
        self.canonical_staged[180] = values[191];
        self.canonical_staged[181] = values[192];
        self.canonical_staged[182] = values[193];
        self.canonical_staged[186] = values[194];
        self.canonical_staged[452] = values[195];
        self.canonical_staged[453] = values[196];
        self.canonical_staged[185] = values[197];
        self.canonical_staged[187] = values[198];
        self.canonical_staged[188] = values[199];
        self.canonical_staged[189] = values[200];
        self.canonical_staged[190] = values[201];
        self.canonical_staged[454] = values[202];
        self.canonical_staged[194] = values[203];
        self.canonical_staged[455] = values[204];
        self.canonical_staged[195] = values[205];
        self.canonical_staged[196] = values[206];
        self.canonical_staged[456] = values[207];
        self.canonical_staged[457] = values[208];
        self.canonical_staged[198] = values[209];
        self.canonical_staged[199] = values[210];
        self.canonical_staged[201] = values[211];
        self.canonical_staged[202] = values[212];
        self.canonical_staged[204] = values[213];
        self.canonical_staged[211] = values[214];
        self.canonical_staged[212] = values[215];
        self.canonical_staged[213] = values[216];
        self.canonical_staged[215] = values[217];
        self.canonical_staged[217] = values[218];
        self.canonical_staged[224] = values[219];
        self.canonical_staged[460] = values[220];
        self.canonical_staged[230] = values[221];
        self.canonical_staged[237] = values[222];
        self.canonical_staged[238] = values[223];
        self.canonical_staged[461] = values[224];
        self.canonical_staged[246] = values[225];
        self.canonical_staged[462] = values[226];
        self.canonical_staged[327] = values[227];
        self.canonical_staged[328] = values[228];
        self.canonical_staged[329] = values[229];
        self.canonical_staged[250] = values[230];
        self.canonical_staged[466] = values[231];
        self.canonical_staged[467] = values[232];
        self.canonical_staged[259] = values[233];
        self.canonical_staged[264] = values[234];
        self.canonical_staged[269] = values[235];
        self.canonical_staged[470] = values[236];
        self.canonical_staged[282] = values[237];
        self.canonical_staged[283] = values[238];
        self.canonical_staged[284] = values[239];
        self.canonical_staged[286] = values[240];
        self.canonical_staged[287] = values[241];
        self.canonical_staged[291] = values[242];
        self.canonical_staged[304] = values[243];
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
                let B = 1e0f64;
                let D = parameters[0];
                let E = 0e0f64;
                let H = parameters[5];
                let J = parameters[45];
                let L = 2e0f64;
                let O = parameters[201];
                let CM = parameters[7];
                let CO = -1e0f64;
                let CQ = 1e19f64;
                let CR = 1e6f64;
                let CT = parameters[46];
                let CV = -1e0f64;
                let CX = 1e16f64;
                let CY = 1e21f64;
                let DD = parameters[58];
                let DE = parameters[57];
                let DF = parameters[44];
                let DG = parameters[41];
                let DJ = parameters[63];
                let DK = parameters[62];
                let DQ = parameters[11];
                let DT = parameters[52];
                let DU = parameters[51];
                let DV = parameters[97];
                let DW = parameters[93];
                let DX = parameters[98];
                let EH = parameters[126];
                let EI = parameters[128];
                let EJ = parameters[130];
                let EK = parameters[43];
                let EL = parameters[42];
                let EM = parameters[47];
                let EN = parameters[65];
                let EO = parameters[80];
                let EP = parameters[79];
                let EQ = parameters[82];
                let ER = parameters[81];
                let ES = parameters[76];
                let ET = parameters[75];
                let EU = parameters[85];
                let EV = parameters[77];
                let EW = parameters[78];
                let EX = parameters[86];
                let EY = parameters[89];
                let EZ = parameters[109];
                let FA = parameters[123];
                let FB = parameters[118];
                let FC = parameters[48];
                let FD = parameters[111];
                let FE = parameters[110];
                let FF = parameters[113];
                let FG = parameters[112];
                let FH = parameters[115];
                let FI = parameters[114];
                let FJ = parameters[103];
                let FK = parameters[119];
                let FL = parameters[129];
                let FM = parameters[127];
                let FN = parameters[148];
                let FO = parameters[149];
                let FP = parameters[134];
                let FQ = parameters[136];
                let FR = parameters[173];
                let FS = parameters[175];
                let FT = parameters[60];
                let FU = parameters[67];
                let FV = parameters[55];
                let FW = parameters[56];
                let FX = parameters[84];
                let FY = parameters[72];
                let FZ = parameters[73];
                let GA = parameters[90];
                let GB = parameters[91];
                let GC = parameters[92];
                let GD = parameters[88];
                let GE = parameters[95];
                let GF = parameters[96];
                let GG = parameters[100];
                let GH = parameters[101];
                let GI = parameters[102];
                let GJ = parameters[167];
                let GK = parameters[122];
                let GL = parameters[117];
                let GM = parameters[131];
                let GN = parameters[144];
                let GO = parameters[138];
                let GP = parameters[139];
                let GQ = parameters[155];
                let GR = parameters[166];
                let GS = parameters[171];
                let GT = parameters[156];
                let GW = parameters[157];
                let GZ = parameters[158];
                let HD = parameters[159];
                let HH = parameters[160];
                let HK = parameters[161];
                let HN = parameters[162];
                let HP = -1e0f64;
                let HS = parameters[202];
                let HU = -1e0f64;
                let HZ = parameters[215];
                let IA = parameters[200];
                let IB = parameters[197];
                let ID = parameters[199];
                let IE = 1.04479e-10f64;
                let IF = 1.43438e-10f64;
                let IH = 3.45313e-11f64;
                let II = parameters[198];
                let IJ = 4e-10f64;
                let IL = parameters[224];
                let IX = parameters[397];
                let IY = parameters[207];
                let JB = parameters[398];
                let JC = parameters[208];
                let JF = parameters[399];
                let JG = parameters[209];
                let JJ = parameters[402];
                let JK = parameters[212];
                let JN = parameters[403];
                let JO = parameters[213];
                let JR = parameters[400];
                let JS = parameters[210];
                let JV = parameters[401];
                let JW = parameters[211];
                let JZ = parameters[404];
                let KA = parameters[214];
                let KD = parameters[405];
                let KH = parameters[406];
                let KK = parameters[407];
                let KL = parameters[225];
                let KO = parameters[408];
                let KP = parameters[226];
                let KT = parameters[409];
                let KU = parameters[231];
                let KX = parameters[410];
                let KY = parameters[232];
                let LB = parameters[411];
                let LC = parameters[233];
                let LF = parameters[412];
                let LG = parameters[289];
                let LJ = parameters[413];
                let LK = parameters[290];
                let LN = parameters[414];
                let LO = parameters[291];
                let LR = parameters[415];
                let LS = parameters[292];
                let LV = parameters[416];
                let LW = parameters[293];
                let LZ = parameters[417];
                let MA = parameters[300];
                let MD = parameters[418];
                let ME = parameters[301];
                let MH = parameters[419];
                let MI = parameters[302];
                let ML = parameters[420];
                let MM = parameters[303];
                let MP = parameters[421];
                let MQ = parameters[304];
                let MT = parameters[422];
                let MU = parameters[305];
                let MX = parameters[423];
                let MY = parameters[306];
                let NB = parameters[424];
                let NC = parameters[307];
                let NF = parameters[425];
                let NG = parameters[308];
                let NJ = parameters[426];
                let NK = parameters[309];
                let NT = parameters[343];
                let NU = parameters[345];
                let NV = parameters[347];
                let NW = parameters[203];
                let NX = parameters[236];
                let NY = parameters[271];
                let NZ = parameters[270];
                let OA = parameters[273];
                let OB = parameters[272];
                let OC = parameters[267];
                let OD = parameters[266];
                let OE = parameters[280];
                let OF = parameters[268];
                let OG = parameters[269];
                let OH = parameters[281];
                let OI = parameters[285];
                let OJ = parameters[324];
                let OK = parameters[338];
                let OL = parameters[333];
                let OM = parameters[204];
                let ON = parameters[326];
                let OO = parameters[325];
                let OP = parameters[328];
                let OQ = parameters[327];
                let OR = parameters[330];
                let OS = parameters[329];
                let OT = parameters[318];
                let OU = parameters[334];
                let OV = parameters[346];
                let OW = parameters[344];
                let OX = parameters[387];
                let OY = parameters[388];
                let OZ = parameters[355];
                let PA = parameters[361];
                let PB = parameters[443];
                let PC = parameters[447];
                let PD = parameters[229];
                let PE = parameters[239];
                let PF = parameters[222];
                let PG = parameters[223];
                let PH = parameters[279];
                let PI = parameters[260];
                let PJ = parameters[261];
                let PK = parameters[286];
                let PL = parameters[287];
                let PM = parameters[288];
                let PN = parameters[284];
                let PO = parameters[298];
                let PP = parameters[299];
                let PQ = parameters[315];
                let PR = parameters[316];
                let PS = parameters[317];
                let PT = parameters[432];
                let PU = parameters[337];
                let PV = parameters[332];
                let PW = parameters[348];
                let PX = parameters[376];
                let PY = parameters[366];
                let PZ = parameters[367];
                let QA = parameters[396];
                let QB = parameters[431];
                let QC = parameters[438];
                let QM = 5e-1f64;
                let QP = parameters[13];
                let RK = parameters[14];
                let RR = 4e-1f64;
                let RS = 1.27520989e0f64;
                let RV = 1.5412087e0f64;
                let mut oK = 0.0;
                let mut oM = 0.0;
                let mut oN = 0.0;
                let mut oP = 0.0;
                let mut oCU = 0.0;
                let mut oDC = 0.0;
                let mut oDH = 0.0;
                let mut oDL = 0.0;
                let mut oDM = 0.0;
                let mut oDN = 0.0;
                let mut oDO = 0.0;
                let mut oDR = 0.0;
                let mut oDS = 0.0;
                let mut oDY = 0.0;
                let mut oDZ = 0.0;
                let mut oEA = 0.0;
                let mut oEB = 0.0;
                let mut oEC = 0.0;
                let mut oED = 0.0;
                let mut oEE = 0.0;
                let mut oEF = 0.0;
                let mut oEG = 0.0;
                let mut oGV = 0.0;
                let mut oGY = 0.0;
                let mut oHC = 0.0;
                let mut oHG = 0.0;
                let mut oHJ = 0.0;
                let mut oHM = 0.0;
                let mut oHT = 0.0;
                let mut oIC = 0.0;
                let mut oIK = 0.0;
                let mut oIM = 0.0;
                let mut oIO = 0.0;
                let mut oIP = 0.0;
                let mut oIQ = 0.0;
                let mut oIS = 0.0;
                let mut oIT = 0.0;
                let mut oIU = 0.0;
                let mut oIV = 0.0;
                let mut oIW = 0.0;
                let mut oIZ = 0.0;
                let mut oJA = 0.0;
                let mut oJD = 0.0;
                let mut oJE = 0.0;
                let mut oJH = 0.0;
                let mut oJI = 0.0;
                let mut oJL = 0.0;
                let mut oJM = 0.0;
                let mut oJP = 0.0;
                let mut oJQ = 0.0;
                let mut oJT = 0.0;
                let mut oJU = 0.0;
                let mut oJX = 0.0;
                let mut oJY = 0.0;
                let mut oKB = 0.0;
                let mut oKC = 0.0;
                let mut oKF = 0.0;
                let mut oKG = 0.0;
                let mut oKJ = 0.0;
                let mut oKM = 0.0;
                let mut oKN = 0.0;
                let mut oKQ = 0.0;
                let mut oKR = 0.0;
                let mut oKS = 0.0;
                let mut oKV = 0.0;
                let mut oKW = 0.0;
                let mut oKZ = 0.0;
                let mut oLA = 0.0;
                let mut oLD = 0.0;
                let mut oLE = 0.0;
                let mut oLH = 0.0;
                let mut oLI = 0.0;
                let mut oLL = 0.0;
                let mut oLM = 0.0;
                let mut oLP = 0.0;
                let mut oLQ = 0.0;
                let mut oLT = 0.0;
                let mut oLU = 0.0;
                let mut oLX = 0.0;
                let mut oLY = 0.0;
                let mut oMB = 0.0;
                let mut oMC = 0.0;
                let mut oMF = 0.0;
                let mut oMG = 0.0;
                let mut oMJ = 0.0;
                let mut oMK = 0.0;
                let mut oMN = 0.0;
                let mut oMO = 0.0;
                let mut oMR = 0.0;
                let mut oMS = 0.0;
                let mut oMV = 0.0;
                let mut oMW = 0.0;
                let mut oMZ = 0.0;
                let mut oNA = 0.0;
                let mut oND = 0.0;
                let mut oNE = 0.0;
                let mut oNH = 0.0;
                let mut oNI = 0.0;
                let mut oNL = 0.0;
                let mut oNM = 0.0;
                let mut oNN = 0.0;
                let mut oNO = 0.0;
                let mut oNQ = 0.0;
                let mut oNR = 0.0;
                let mut oNS = 0.0;
                let mut oRL = 0.0;
                let mut oRT = 0.0;
                let mut oRW = 0.0;
                let mut oSV = 0.0;
                let mut oSW = 0.0;
                let mut oSX = 0.0;
                let mut oSY = 0.0;
                let mut oSZ = 0.0;
                let mut oTA = 0.0;
                let mut oTC = 0.0;
                let mut oTD = 0.0;
                let mut oTO = 0.0;
                let mut oTQ = 0.0;
                let mut oTR = 0.0;
                let mut oTS = 0.0;
                let mut oTX = 0.0;
                let mut oTY = 0.0;
                let mut oTZ = 0.0;
                let mut oUA = 0.0;
                let A = 2.7315e2f64 + parameters[15];
                let C = if parameters[10] == B { 1.0 } else { 0.0 };
                let F = if D == E { 1.0 } else { 0.0 };
                let G = if (if F != 0.0 && (if parameters[172] > E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if D > E { 1.0 } else { 0.0 }) != 0.0 && (if parameters[439] > E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let I = if G != 0.0 {
                    H
                } else {
                    E
                };
                let Q;
                let R;
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
                let AE;
                let AF;
                let AG;
                let AH;
                let AI;
                let AJ;
                let AK;
                let AL;
                let AM;
                let AN;
                let AO;
                let AP;
                let AQ;
                let AR;
                let AS;
                let AT;
                let AU;
                let AV;
                let AW;
                let AX;
                let AY;
                let AZ;
                let BA;
                let BB;
                let BC;
                let BD;
                let BE;
                let BF;
                let BG;
                let BH;
                let BI;
                let BJ;
                let BK;
                let BL;
                let BM;
                let BN;
                let BO;
                let BP;
                let BQ;
                let BR;
                let BS;
                let BT;
                let BU;
                let BV;
                let BW;
                let BX;
                let BY;
                let BZ;
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
                if F != 0.0 {
                    let K = if J < E { 1.0 } else { 0.0 };
                    oK = K;
                    let CP = if K != 0.0 {
                        CO
                    } else {
                        B
                    };
                    let CS = (if (J.abs()) <= CQ { (J.abs()) } else { CQ }) * CR;
                    let CU = if CT < E { 1.0 } else { 0.0 };
                    oCU = CU;
                    let CW = if CU != 0.0 {
                        CV
                    } else {
                        B
                    };
                    let CZ = (if (if (CT.abs()) >= CX { (CT.abs()) } else { CX }) <= CY { (if (CT.abs()) >= CX { (CT.abs()) } else { CX }) } else { CY }) * CR;
                    let DA = parameters[49] * CR;
                    let DB = parameters[50] * CR;
                    let DC = parameters[54] * CR;
                    oDC = DC;
                    let DH = ((DD * DE) * DF) / DG;
                    oDH = DH;
                    let DI = parameters[59] * CR;
                    let DL = ((DJ * DK) * DF) / DG;
                    oDL = DL;
                    let DM = parameters[69] * parameters[68];
                    oDM = DM;
                    let DN = ((parameters[141] * parameters[140]) * DF) / DG;
                    oDN = DN;
                    let DO = ((parameters[143] * parameters[142]) * DF) / DG;
                    oDO = DO;
                    let DP = parameters[153] * CR;
                    let DR = if DQ > E { 1.0 } else { 0.0 };
                    oDR = DR;
                    let DY;
                    let DZ;
                    let EA;
                    let EB;
                    let EC;
                    let ED;
                    let EE;
                    let EF;
                    let EG;
                    if DR != 0.0 {
                        let DS = if (if parameter_given[156] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oDS = DS;
                        let GU = if DS != 0.0 {
                            GT
                        } else {
                            DU
                        };
                        let GV = if (if parameter_given[157] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oGV = GV;
                        let GX = if GV != 0.0 {
                            GW
                        } else {
                            DT
                        };
                        let GY = if (if parameter_given[158] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oGY = GY;
                        let HA = if GY != 0.0 {
                            GZ
                        } else {
                            DE
                        };
                        let HB = ((DD * HA) * DF) / DG;
                        let HC = if (if parameter_given[159] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oHC = HC;
                        let HE = if HC != 0.0 {
                            HD
                        } else {
                            DK
                        };
                        let HF = ((DJ * HE) * DF) / DG;
                        let HG = if (if parameter_given[160] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oHG = HG;
                        let HI = if HG != 0.0 {
                            HH
                        } else {
                            DW
                        };
                        let HJ = if (if parameter_given[161] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oHJ = HJ;
                        let HL = if HJ != 0.0 {
                            HK
                        } else {
                            DV
                        };
                        let HM = if (if parameter_given[162] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oHM = HM;
                        let HO = if HM != 0.0 {
                            HN
                        } else {
                            DX
                        };
                        DY = HE;
                        DZ = HF;
                        EA = GX;
                        EB = GU;
                        EC = HL;
                        ED = HI;
                        EE = HA;
                        EF = HB;
                        EG = HO;
                    } else {
                        DY = DK;
                        DZ = DL;
                        EA = DT;
                        EB = DU;
                        EC = DV;
                        ED = DW;
                        EE = DE;
                        EF = DH;
                        EG = DX;
                    }
                    oDY = DY;
                    oDZ = DZ;
                    oEA = EA;
                    oEB = EB;
                    oEC = EC;
                    oED = ED;
                    oEE = EE;
                    oEF = EF;
                    oEG = EG;
                    Q = DA;
                    R = EH;
                    S = EI;
                    T = EJ;
                    U = EK;
                    V = CS;
                    W = EL;
                    X = CP;
                    Y = DG;
                    Z = DF;
                    AA = EM;
                    AB = DI;
                    AC = EN;
                    AD = CZ;
                    AE = CW;
                    AF = EO;
                    AG = EP;
                    AH = EQ;
                    AI = ER;
                    AJ = ES;
                    AK = ET;
                    AL = EU;
                    AM = EV;
                    AN = EW;
                    AO = EX;
                    AP = EY;
                    AQ = EZ;
                    AR = FA;
                    AS = FB;
                    AT = FC;
                    AU = FD;
                    AV = FE;
                    AW = FF;
                    AX = FG;
                    AY = FH;
                    AZ = FI;
                    BA = FJ;
                    BB = FK;
                    BC = FL;
                    BD = FM;
                    BE = FN;
                    BF = FO;
                    BG = FP;
                    BH = FQ;
                    BI = DP;
                    BJ = FR;
                    BK = FS;
                    BL = FT;
                    BM = FU;
                    BN = FV;
                    BO = FW;
                    BP = FX;
                    BQ = FY;
                    BR = FZ;
                    BS = GA;
                    BT = GB;
                    BU = GC;
                    BV = GD;
                    BW = GE;
                    BX = GF;
                    BY = GG;
                    BZ = GH;
                    CA = GI;
                    CB = GJ;
                    CC = DB;
                    CD = GK;
                    CE = GL;
                    CF = GM;
                    CG = GN;
                    CH = GO;
                    CI = GP;
                    CJ = GQ;
                    CK = GR;
                    CL = GS;
                } else {
                    let M = L * parameters[190];
                    oM = M;
                    let N = L * parameters[194];
                    oN = N;
                    let P = if O < E { 1.0 } else { 0.0 };
                    oP = P;
                    let HQ = if P != 0.0 {
                        HP
                    } else {
                        B
                    };
                    let HR = (if (O.abs()) <= CQ { (O.abs()) } else { CQ }) * CR;
                    let HT = if HS < E { 1.0 } else { 0.0 };
                    oHT = HT;
                    let HV = if HT != 0.0 {
                        HU
                    } else {
                        B
                    };
                    let HW = (if (if (HS.abs()) >= CX { (HS.abs()) } else { CX }) <= CY { (if (HS.abs()) >= CX { (HS.abs()) } else { CX }) } else { CY }) * CR;
                    let HX = parameters[205] * CR;
                    let HY = parameters[206] * CR;
                    let IC = (HZ * IA) / IB;
                    oIC = IC;
                    let IG = (IE * (B - ID)) + (IF * ID);
                    let IK = (((IG / IH) * II) * (IB + IJ)).sqrt();
                    oIK = IK;
                    let IM = IL * L;
                    oIM = IM;
                    let IN = parameters[228] * CR;
                    let IO = L * parameters[353];
                    oIO = IO;
                    let IP = parameters[368] * L;
                    oIP = IP;
                    let IQ = parameters[377] * parameters[378];
                    oIQ = IQ;
                    let IR = parameters[394] * CR;
                    let IS = if DQ > E { 1.0 } else { 0.0 };
                    oIS = IS;
                    if IS != 0.0 {
                        let IT = if (if parameter_given[397] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oIT = IT;
                        let IZ = if IT != 0.0 {
                            IX
                        } else {
                            IY
                        };
                        oIZ = IZ;
                        let JA = if (if parameter_given[398] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJA = JA;
                        let JD = if JA != 0.0 {
                            JB
                        } else {
                            JC
                        };
                        oJD = JD;
                        let JE = if (if parameter_given[399] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJE = JE;
                        let JH = if JE != 0.0 {
                            JF
                        } else {
                            JG
                        };
                        oJH = JH;
                        let JI = if (if parameter_given[402] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJI = JI;
                        let JL = if JI != 0.0 {
                            JJ
                        } else {
                            JK
                        };
                        oJL = JL;
                        let JM = if (if parameter_given[403] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJM = JM;
                        let JP = if JM != 0.0 {
                            JN
                        } else {
                            JO
                        };
                        oJP = JP;
                        let JQ = if (if parameter_given[400] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJQ = JQ;
                        let JT = if JQ != 0.0 {
                            JR
                        } else {
                            JS
                        };
                        oJT = JT;
                        let JU = if (if parameter_given[401] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJU = JU;
                        let JX = if JU != 0.0 {
                            JV
                        } else {
                            JW
                        };
                        oJX = JX;
                        let JY = if (if parameter_given[404] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oJY = JY;
                        let KB = if JY != 0.0 {
                            JZ
                        } else {
                            KA
                        };
                        oKB = KB;
                        let KC = if (if parameter_given[405] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oKC = KC;
                        let KE = if KC != 0.0 {
                            KD
                        } else {
                            HZ
                        };
                        let KF = (KE * IA) / IB;
                        oKF = KF;
                        let KG = if (if parameter_given[406] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oKG = KG;
                        let KI = if KG != 0.0 {
                            KH
                        } else {
                            IL
                        };
                        let KJ = if (if parameter_given[407] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oKJ = KJ;
                        let KM = if KJ != 0.0 {
                            KK
                        } else {
                            KL
                        };
                        oKM = KM;
                        let KN = if (if parameter_given[408] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oKN = KN;
                        let KQ = if KN != 0.0 {
                            KO
                        } else {
                            KP
                        };
                        oKQ = KQ;
                        let KR = KI * L;
                        oKR = KR;
                        let KS = if (if parameter_given[409] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oKS = KS;
                        let KV = if KS != 0.0 {
                            KT
                        } else {
                            KU
                        };
                        oKV = KV;
                        let KW = if (if parameter_given[410] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oKW = KW;
                        let KZ = if KW != 0.0 {
                            KX
                        } else {
                            KY
                        };
                        oKZ = KZ;
                        let LA = if (if parameter_given[411] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLA = LA;
                        let LD = if LA != 0.0 {
                            LB
                        } else {
                            LC
                        };
                        oLD = LD;
                        let LE = if (if parameter_given[412] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLE = LE;
                        let LH = if LE != 0.0 {
                            LF
                        } else {
                            LG
                        };
                        oLH = LH;
                        let LI = if (if parameter_given[413] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLI = LI;
                        let LL = if LI != 0.0 {
                            LJ
                        } else {
                            LK
                        };
                        oLL = LL;
                        let LM = if (if parameter_given[414] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLM = LM;
                        let LP = if LM != 0.0 {
                            LN
                        } else {
                            LO
                        };
                        oLP = LP;
                        let LQ = if (if parameter_given[415] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLQ = LQ;
                        let LT = if LQ != 0.0 {
                            LR
                        } else {
                            LS
                        };
                        oLT = LT;
                        let LU = if (if parameter_given[416] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLU = LU;
                        let LX = if LU != 0.0 {
                            LV
                        } else {
                            LW
                        };
                        oLX = LX;
                        let LY = if (if parameter_given[417] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oLY = LY;
                        let MB = if LY != 0.0 {
                            LZ
                        } else {
                            MA
                        };
                        oMB = MB;
                        let MC = if (if parameter_given[418] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oMC = MC;
                        let MF = if MC != 0.0 {
                            MD
                        } else {
                            ME
                        };
                        oMF = MF;
                        let MG = if (if parameter_given[419] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oMG = MG;
                        let MJ = if MG != 0.0 {
                            MH
                        } else {
                            MI
                        };
                        oMJ = MJ;
                        let MK = if (if parameter_given[420] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oMK = MK;
                        let MN = if MK != 0.0 {
                            ML
                        } else {
                            MM
                        };
                        oMN = MN;
                        let MO = if (if parameter_given[421] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oMO = MO;
                        let MR = if MO != 0.0 {
                            MP
                        } else {
                            MQ
                        };
                        oMR = MR;
                        let MS = if (if parameter_given[422] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oMS = MS;
                        let MV = if MS != 0.0 {
                            MT
                        } else {
                            MU
                        };
                        oMV = MV;
                        let MW = if (if parameter_given[423] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oMW = MW;
                        let MZ = if MW != 0.0 {
                            MX
                        } else {
                            MY
                        };
                        oMZ = MZ;
                        let NA = if (if parameter_given[424] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oNA = NA;
                        let ND = if NA != 0.0 {
                            NB
                        } else {
                            NC
                        };
                        oND = ND;
                        let NE = if (if parameter_given[425] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oNE = NE;
                        let NH = if NE != 0.0 {
                            NF
                        } else {
                            NG
                        };
                        oNH = NH;
                        let NI = if (if parameter_given[426] { 1.0 } else { 0.0 }) == B { 1.0 } else { 0.0 };
                        oNI = NI;
                        let NL = if NI != 0.0 {
                            NJ
                        } else {
                            NK
                        };
                        oNL = NL;
                    } else {
                    }
                    let IU = IH / IB;
                    oIU = IU;
                    let IV = parameters[430] * 1e-6f64;
                    oIV = IV;
                    let IW = (parameters[437] * IG) * II;
                    oIW = IW;
                    let NM = parameters[449] - L;
                    oNM = NM;
                    let NN = parameters[486] + parameters[487];
                    oNN = NN;
                    let NO = if parameters[490] >= E { parameters[490] } else { E };
                    oNO = NO;
                    let NP = if parameters[491] >= E { parameters[491] } else { E };
                    let NQ = if CM == E { 1.0 } else { 0.0 };
                    oNQ = NQ;
                    let NR = if NQ != 0.0 {
                        NO
                    } else {
                        NP
                    };
                    oNR = NR;
                    let NS = if parameters[457] > E { 1.0 } else { 0.0 };
                    oNS = NS;
                    Q = HX;
                    R = NT;
                    S = NU;
                    T = NV;
                    U = ID;
                    V = HR;
                    W = II;
                    X = HQ;
                    Y = IB;
                    Z = IA;
                    AA = NW;
                    AB = IN;
                    AC = NX;
                    AD = HW;
                    AE = HV;
                    AF = NY;
                    AG = NZ;
                    AH = OA;
                    AI = OB;
                    AJ = OC;
                    AK = OD;
                    AL = OE;
                    AM = OF;
                    AN = OG;
                    AO = OH;
                    AP = OI;
                    AQ = OJ;
                    AR = OK;
                    AS = OL;
                    AT = OM;
                    AU = ON;
                    AV = OO;
                    AW = OP;
                    AX = OQ;
                    AY = OR;
                    AZ = OS;
                    BA = OT;
                    BB = OU;
                    BC = OV;
                    BD = OW;
                    BE = OX;
                    BF = OY;
                    BG = OZ;
                    BH = PA;
                    BI = IR;
                    BJ = PB;
                    BK = PC;
                    BL = PD;
                    BM = PE;
                    BN = PF;
                    BO = PG;
                    BP = PH;
                    BQ = PI;
                    BR = PJ;
                    BS = PK;
                    BT = PL;
                    BU = PM;
                    BV = PN;
                    BW = PO;
                    BX = PP;
                    BY = PQ;
                    BZ = PR;
                    CA = PS;
                    CB = PT;
                    CC = HY;
                    CD = PU;
                    CE = PV;
                    CF = PW;
                    CG = PX;
                    CH = PY;
                    CI = PZ;
                    CJ = QA;
                    CK = QB;
                    CL = QC;
                }
                let CN = if CM == E { 1.0 } else { 0.0 };
                let QD;
                let QE;
                let QF;
                let QG;
                if CN != 0.0 {
                    QD = S;
                    QE = R;
                    QF = Q;
                    QG = T;
                } else {
                    QD = BC;
                    QE = BD;
                    QF = CC;
                    QG = CF;
                }
                let QH = B - U;
                let QI = (IE * QH) + (IF * U);
                let QJ = -4e-1f64 * QH;
                let QK = B / (B + ((1e1f64 * U).sqrt()));
                let QL = 5e-2f64 * U;
                let QN = (((1.602176565e-19f64 * V) * QM) * W) / IH;
                let QO = if X > E { 1.0 } else { 0.0 };
                let QX;
                let QY;
                if QO != 0.0 {
                    let QQ = QP * IJ;
                    let QR = QN * (Y + QQ);
                    let QS = QN * (Z + QQ);
                    QX = QR;
                    QY = QS;
                } else {
                    let QT = -QN;
                    let QU = QP * IJ;
                    let QV = QT * (Y + QU);
                    let QW = QT * (Z + QU);
                    QX = QV;
                    QY = QW;
                }
                let QZ = IH / Y;
                let RA = IH / Z;
                let RB = QI / W;
                let RC = RB * RB;
                let RD = (8.010882825e-20f64 * AB) * W;
                let RE = (3.20435313e-19f64 * AD) * IE;
                let RF = if parameters[2] > E { 1.0 } else { 0.0 };
                let RG = if parameters[9] > E { 1.0 } else { 0.0 };
                let RH = 3.20435313e-19f64 * QI;
                let RI = (1e18f64 * W) * W;
                let RJ = if QP > E { 1.0 } else { 0.0 };
                let RM;
                if RJ != 0.0 {
                    let RL = if RK == B { 1.0 } else { 0.0 };
                    oRL = RL;
                    let RX = if RL != 0.0 {
                        let RQ = 4.09618895e-1f64 / RI;
                        let RT = (RR * QP) * RS;
                        oRT = RT;
                        RQ
                    } else {
                        let RU = 7.23134895e-1f64 / RI;
                        let RW = (RR * QP) * RV;
                        oRW = RW;
                        RU
                    };
                    RM = RX;
                } else {
                    RM = E;
                }
                let RN = B / (QM * AM);
                let RO = RN / AN;
                let RP = if RK == B { 1.0 } else { 0.0 };
                let SA = if RP != 0.0 {
                    let RY = QM * AO;
                    RY
                } else {
                    let RZ = 3.333333333333e-1f64 * AO;
                    RZ
                };
                let SB = B - SA;
                let SC = -AQ;
                let SD = -AR;
                let SE = B / AS;
                let SF = ((1.3333333333332e0f64 * ((2.9189679640027008e-49f64 * AS).sqrt())) / 1.054571726e-34f64) * AT;
                let SG = if AU < E { 1.0 } else { 0.0 };
                let SI = if SG != 0.0 {
                    let SH = (-4.95e-1f64 * AV) / AU;
                    SH
                } else {
                    E
                };
                let SJ = if AW < E { 1.0 } else { 0.0 };
                let SL = if SJ != 0.0 {
                    let SK = (-4.95e-1f64 * AX) / AW;
                    SK
                } else {
                    E
                };
                let SM = if AY < E { 1.0 } else { 0.0 };
                let SO = if SM != 0.0 {
                    let SN = (-4.95e-1f64 * AZ) / AY;
                    SN
                } else {
                    E
                };
                let SP = 4e-18f64 / (AT * AT);
                let SQ = AT * 5e8f64;
                let SR = -BF;
                let SS = 4.0054414125e-20f64 * BI;
                let ST = (((QI / IH) * W) * (Y + IJ)).sqrt();
                let SU = if I > E { 1.0 } else { 0.0 };
                if SU != 0.0 {
                    let SV = -4e-1f64 * QH;
                    oSV = SV;
                    let SW = (8.010882825e-20f64 * AB) * W;
                    oSW = SW;
                    if RJ != 0.0 {
                        if RP != 0.0 {
                            let SY = (RR * QP) * RS;
                            oSY = SY;
                        } else {
                            let SZ = (RR * QP) * RV;
                            oSZ = SZ;
                        }
                    } else {
                    }
                    let SX = 4.0054414125e-20f64 * BI;
                    oSX = SX;
                } else {
                }
                if RF != 0.0 {
                    let TA = RK * AE;
                    oTA = TA;
                } else {
                }
                let TB = if BS == E { 1.0 } else { 0.0 };
                if TB != 0.0 {
                } else {
                    let TC = if BS < E { 1.0 } else { 0.0 };
                    oTC = TC;
                }
                if TB != 0.0 {
                } else {
                    let TD = if BS < E { 1.0 } else { 0.0 };
                    oTD = TD;
                }
                let TE = if BY > E { 1.0 } else { 0.0 };
                let TF = if BW < E { 1.0 } else { 0.0 };
                let TG = if BX < E { 1.0 } else { 0.0 };
                let TH = RK * CB;
                let TI = (3.20435313e-19f64 * Q) * QI;
                let TJ = if parameters[3] > E { 1.0 } else { 0.0 };
                let TK = if parameters[4] > E { 1.0 } else { 0.0 };
                let TL = (3.20435313e-19f64 * QF) * QI;
                let TM = if parameters[12] > E { 1.0 } else { 0.0 };
                let TN = if parameters[8] != E { 1.0 } else { 0.0 };
                if SU != 0.0 {
                    let TO = 1e8f64 * parameters[16];
                    oTO = TO;
                } else {
                }
                let TP = if DQ > E { 1.0 } else { 0.0 };
                if TP != 0.0 {
                    if RF != 0.0 {
                        let TQ = RK * AE;
                        oTQ = TQ;
                    } else {
                    }
                    if TB != 0.0 {
                    } else {
                        let TR = if BS < E { 1.0 } else { 0.0 };
                        oTR = TR;
                    }
                    if TB != 0.0 {
                    } else {
                        let TS = if BS < E { 1.0 } else { 0.0 };
                        oTS = TS;
                    }
                } else {
                }
                let TT = if parameters[6] > E { 1.0 } else { 0.0 };
                let TU = -4e-1f64 * QH;
                let TV = AA * A;
                let TW = (8.010882825e-20f64 * AB) * W;
                if RJ != 0.0 {
                    if RP != 0.0 {
                        let TX = (RR * QP) * RS;
                        oTX = TX;
                    } else {
                        let TY = (RR * QP) * RV;
                        oTY = TY;
                    }
                } else {
                }
                if RF != 0.0 {
                    let TZ = RK * AE;
                    oTZ = TZ;
                } else {
                }
                if TP != 0.0 {
                    if RF != 0.0 {
                        let UA = RK * AE;
                        oUA = UA;
                    } else {
                    }
                } else {
                }
            [A, C, F, G, oK, oCU, oDC, oDH, oDL, oDM, oDN, oDO, oDR, oDS, oGV, oGY, oHC, oHG, oHJ, oHM, oM, oN, oP, oHT, oIC, oIK, oIM, oIO, oIP, oIQ, oIS, oIT, oJA, oJE, oJI, oJM, oJQ, oJU, oJD, oJH, oJT, oJX, oIZ, oJL, oJP, oJY, oKC, oKB, oKF, oKG, oKJ, oKN, oKR, oKM, oKQ, oKS, oKW, oLA, oKZ, oLD, oKV, oLE, oLI, oLM, oLQ, oLU, oLH, oLL, oLP, oLT, oLX, oLY, oMC, oMG, oMK, oMO, oMB, oMF, oMJ, oMN, oMR, oMS, oMW, oNA, oNE, oNI, oMV, oMZ, oNL, oND, oNH, oIU, oIV, oIW, oNM, oNN, oNO, oNQ, oNR, oNS, CN, R, S, T, U, QI, QJ, QK, QL, W, QO, QZ, RA, RB, AA, RC, RD, oDY, oDZ, AC, AD, RE, RF, AE, oEA, RG, RH, RI, RJ, oRL, oRT, oRW, RM, QX, QY, oEB, AF, AG, AH, AI, AJ, AK, AL, RN, RO, RP, SA, SB, AP, oEC, oED, SC, SD, AS, SE, SF, AU, SG, AV, AW, SJ, AX, AY, SM, AZ, BA, BB, SP, SQ, QD, QE, BE, SR, BG, BH, BI, SS, ST, BJ, BK, SU, oSV, oSW, oSY, oSZ, oSX, oTA, BL, BM, BN, BO, BP, BQ, BR, BS, TB, oTC, BT, BU, BV, BW, BX, oTD, BY, TE, BZ, CA, TF, TG, TH, TI, TJ, TK, TL, SO, SL, CD, CE, SI, QG, TM, CG, CH, CI, TN, oTO, TP, oEE, oEF, oEG, oTQ, oTR, oTS, CJ, CK, CL, TT, TU, TV, TW, oTX, oTY, oTZ, oUA]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 138] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = 1e0f64;
                let B = staged[308];
                let C = parameters[29];
                let E = 1e-9f64;
                let G = parameters[23];
                let I = parameters[22];
                let K = parameters[25];
                let M = parameters[24];
                let O = parameters[30];
                let Q = 1e-6f64;
                let R = parameters[20];
                let AL = 5e-1f64;
                let DG = staged[311];
                let DH = parameters[105];
                let DI = parameters[120];
                let DJ = parameters[107];
                let DK = parameters[124];
                let DL = parameters[132];
                let DM = parameters[163];
                let DN = parameters[168];
                let DO = parameters[61];
                let DP = parameters[64];
                let DQ = parameters[62];
                let DR = staged[315];
                let DS = staged[316];
                let DT = staged[317];
                let DU = parameters[52];
                let DV = staged[318];
                let DW = staged[319];
                let DX = parameters[53];
                let DY = parameters[51];
                let DZ = staged[320];
                let EA = parameters[70];
                let EB = parameters[74];
                let EC = parameters[71];
                let ED = parameters[83];
                let EE = parameters[87];
                let EF = parameters[97];
                let EG = staged[322];
                let EH = parameters[94];
                let EI = parameters[99];
                let EJ = parameters[104];
                let EK = parameters[106];
                let EL = parameters[108];
                let EM = parameters[121];
                let EN = parameters[125];
                let EO = parameters[137];
                let EP = parameters[135];
                let EQ = parameters[146];
                let ER = parameters[145];
                let ES = parameters[151];
                let ET = parameters[154];
                let EU = parameters[172];
                let EV = parameters[176];
                let EW = parameters[183];
                let EX = parameters[184];
                let EY = parameters[185];
                let EZ = parameters[186];
                let FA = parameters[57];
                let FB = staged[324];
                let FC = parameters[98];
                let FD = parameters[66];
                let FE = parameters[164];
                let FF = parameters[116];
                let FG = parameters[133];
                let FH = parameters[140];
                let FI = staged[325];
                let FJ = parameters[142];
                let FK = staged[326];
                let FL = parameters[150];
                let FM = parameters[147];
                let FN = staged[327];
                let FO = staged[328];
                let FP = staged[329];
                let FQ = parameters[169];
                let FR = parameters[165];
                let FS = parameters[152];
                let FT = parameters[170];
                let FU = parameters[174];
                let GB = 0e0f64;
                let GC = 5e0f64;
                let GE = parameters[227];
                let GF = parameters[200];
                let GG = parameters[197];
                let GO = parameters[234];
                let GR = 1e-3f64;
                let GX = 8e1f64;
                let GZ = 3.333333333333e-1f64;
                let HA = 1.80485e-35f64;
                let HJ = parameters[240];
                let HM = 1e-10f64;
                let HX = 1.6e1f64;
                let JA = staged[339];
                let JU = parameters[28];
                let KP = parameters[37];
                let KV = 2e0f64;
                let LF = parameters[26];
                let LG = parameters[27];
                let LR = parameters[478];
                let ML = parameters[458];
                let MN = parameters[459];
                let MP = parameters[460];
                let MV = parameters[461];
                let NA = 1e-20f64;
                let NT = parameters[477];
                let PT = staged[59];
                let PV = staged[60];
                let PZ = staged[62];
                let QG = staged[388];
                let QH = staged[389];
                let QJ = parameters[14];
                let QL = 6.931471805599e-1f64;
                let QM = 3.75e-1f64;
                let QP = staged[117];
                let RH = staged[400];
                let RN = 1e-8f64;
                let RV = 2.5e-1f64;
                let RY = staged[202];
                let SD = staged[460];
                let SJ = staged[462];
                let SY = staged[269];
                let TD = 0e0f64;
                let TE = 0e0f64;
                let TI = 0e0f64;
                let TJ = 0e0f64;
                let TN = 0e0f64;
                let TO = 0e0f64;
                let TS = 0e0f64;
                let TT = 0e0f64;
                let mut oGV = 0.0;
                let mut oHE = 0.0;
                let mut oHL = 0.0;
                let mut oHN = 0.0;
                let mut oHO = 0.0;
                let mut oHU = 0.0;
                let mut oHV = 0.0;
                let mut oJB = 0.0;
                let mut oJI = 0.0;
                let mut oJV = 0.0;
                let mut oKJ = 0.0;
                let mut oKS = 0.0;
                let mut oLH = 0.0;
                let mut oLI = 0.0;
                let mut oME = 0.0;
                let mut oMT = 0.0;
                let mut oMW = 0.0;
                let mut oMX = 0.0;
                let mut oNQ = 0.0;
                let mut oNV = 0.0;
                let mut oNW = 0.0;
                let mut oNZ = 0.0;
                let mut oOF = 0.0;
                let mut oOS = 0.0;
                let mut oOY = 0.0;
                let mut oRI = 0.0;
                let mut oRO = 0.0;
                let mut oRS = 0.0;
                let mut oRT = 0.0;
                let mut oRU = 0.0;
                let mut oRW = 0.0;
                let mut oRX = 0.0;
                let mut oSE = 0.0;
                let mut oSF = 0.0;
                let mut oSG = 0.0;
                let mut oSH = 0.0;
                let mut oSI = 0.0;
                let mut oSO = 0.0;
                let mut oSS = 0.0;
                let mut oST = 0.0;
                let mut oSU = 0.0;
                let mut oSV = 0.0;
                let mut oSW = 0.0;
                let mut oTC = 0.0;
                let mut oTH = 0.0;
                let mut oTM = 0.0;
                let mut oTR = 0.0;
                let AN;
                let AO;
                let AP;
                let AQ;
                let AR;
                let AS;
                let AT;
                let AU;
                let AV;
                let AW;
                let AX;
                let AY;
                let AZ;
                let BA;
                let BB;
                let BC;
                let BD;
                let BE;
                let BF;
                let BG;
                let BH;
                let BI;
                let BJ;
                let BK;
                let BL;
                let BM;
                let BN;
                let BO;
                let BP;
                let BQ;
                let BR;
                let BS;
                let BT;
                let BU;
                let BV;
                let BW;
                let BX;
                let BY;
                let BZ;
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
                let CP;
                let CQ;
                let CR;
                let CS;
                let CT;
                let CU;
                let CV;
                let CW;
                let CX;
                let CY;
                let CZ;
                let DA;
                let DB;
                let DC;
                let DD;
                let DE;
                let DF;
                if B != 0.0 {
                    AN = DH;
                    AO = DI;
                    AP = DJ;
                    AQ = DK;
                    AR = DL;
                    AS = DM;
                    AT = DN;
                    AU = DO;
                    AV = DP;
                    AW = DQ;
                    AX = DR;
                    AY = DS;
                    AZ = DT;
                    BA = DU;
                    BB = DV;
                    BC = DW;
                    BD = DX;
                    BE = DY;
                    BF = DZ;
                    BG = EA;
                    BH = EB;
                    BI = EC;
                    BJ = ED;
                    BK = EE;
                    BL = EF;
                    BM = EG;
                    BN = EH;
                    BO = EI;
                    BP = EJ;
                    BQ = EK;
                    BR = EL;
                    BS = EM;
                    BT = EN;
                    BU = EO;
                    BV = EP;
                    BW = EQ;
                    BX = ER;
                    BY = ES;
                    BZ = ET;
                    CA = EU;
                    CB = EV;
                    CC = EW;
                    CD = EX;
                    CE = EY;
                    CF = EZ;
                    CG = FA;
                    CH = FB;
                    CI = FC;
                    CJ = FD;
                    CK = FE;
                    CL = FF;
                    CM = FG;
                    CN = FH;
                    CO = FI;
                    CP = FJ;
                    CQ = FK;
                    CR = FL;
                    CS = FM;
                    CT = FN;
                    CU = FO;
                    CV = FP;
                    CW = FQ;
                    CX = FR;
                    CY = FS;
                    CZ = FT;
                    DA = I;
                    DB = M;
                    DC = G;
                    DD = K;
                    DE = FU;
                    DF = O;
                } else {
                    let D = A / C;
                    let F = if (parameters[21] * D) >= E { (parameters[21] * D) } else { E };
                    let H = G * D;
                    let J = I * D;
                    let L = K * D;
                    let N = M * D;
                    let P = O * C;
                    let S = Q / R;
                    let T = Q / F;
                    let U = (parameters[191] * (A + (parameters[193] * T))) * (A + (parameters[192] * S));
                    let V = R + ((parameters[187] * (A + (parameters[188] * S))) * (A + (parameters[189] * T)));
                    let W = V - staged[1];
                    let X = if W >= E { W } else { E };
                    let Y = F + U;
                    let Z = Y - staged[2];
                    let AA = if Z >= E { Z } else { E };
                    let AB = if (W + parameters[195]) >= E { (W + parameters[195]) } else { E };
                    let AC = if (Z + parameters[196]) >= E { (Z + parameters[196]) } else { E };
                    let AD = Q / X;
                    let AE = Q / AA;
                    let AF = AD * AE;
                    let AG = if V >= E { V } else { E };
                    let AH = AG / Q;
                    let AI = if Y >= E { Y } else { E };
                    let AJ = AI / Q;
                    let AK = if (AG + parameters[489]) >= E { (AG + parameters[489]) } else { E };
                    let AM = if (parameters[38] - (AL * U)) >= E { (parameters[38] - (AL * U)) } else { E };
                    let FV = (parameters[208] * (AD.powf(parameters[209]))) / (A + (parameters[210] * (AD.powf(parameters[211]))));
                    let FW = ((parameters[207] + FV) + (parameters[212] * AE)) + (parameters[213] * AF);
                    let FX = parameters[214] + (staged[3] * FV);
                    let FY = ((parameters[216] * (A + (parameters[217] * AD))) * (A + (parameters[218] * AE))) * (A + (parameters[219] * AF));
                    let FZ = if (if ((parameters[220] * (A + (parameters[221] * AD))) * 1e6f64) >= 1e25f64 { ((parameters[220] * (A + (parameters[221] * AD))) * 1e6f64) } else { 1e25f64 }) <= 1e28f64 { (if ((parameters[220] * (A + (parameters[221] * AD))) * 1e6f64) >= 1e25f64 { ((parameters[220] * (A + (parameters[221] * AD))) * 1e6f64) } else { 1e25f64 }) } else { 1e28f64 };
                    let GA = staged[4] / X;
                    let GD = if (if ((staged[5] * (GA.powf(parameters[225]))) * (A + (parameters[226] * AE))) >= GB { ((staged[5] * (GA.powf(parameters[225]))) * (A + (parameters[226] * AE))) } else { GB }) <= GC { (if ((staged[5] * (GA.powf(parameters[225]))) * (A + (parameters[226] * AE))) >= GB { ((staged[5] * (GA.powf(parameters[225]))) * (A + (parameters[226] * AE))) } else { GB }) } else { GC };
                    let GH = ((GE * GD) * GF) / GG;
                    let GI = if (if (parameters[230] * AE) >= -1e0f64 { (parameters[230] * AE) } else { -1e0f64 }) <= A { (if (parameters[230] * AE) >= -1e0f64 { (parameters[230] * AE) } else { -1e0f64 }) } else { A };
                    let GJ = GA.powf(parameters[232]);
                    let GK = A + (parameters[233] * AE);
                    let GL = GJ * GK;
                    let GM = parameters[231] * GL;
                    let GN = if GM >= GB { GM } else { GB };
                    let GP = ((GO * GN) * GF) / GG;
                    let GQ = parameters[235] * GL;
                    let GS = (parameters[237] * AD) / (if (A + (parameters[238] * AE)) >= GR { (A + (parameters[238] * AE)) } else { GR });
                    let GT = -X;
                    let GU = GT / (parameters[243] * (if (A + (parameters[244] * AE)) >= GR { (A + (parameters[244] * AE)) } else { GR }));
                    let GV = if GU > -8e1f64 { 1.0 } else { 0.0 };
                    oGV = GV;
                    let HC = if GV != 0.0 {
                        let GW = GU.exp();
                        GW
                    } else {
                        let GY = (-GU) - GX;
                        let HB = HA / (A + (GY * (A + ((AL * GY) * (A + (GY * GZ))))));
                        HB
                    };
                    let HD = GT / parameters[246];
                    let HE = if HD > -8e1f64 { 1.0 } else { 0.0 };
                    oHE = HE;
                    let HI = if HE != 0.0 {
                        let HF = HD.exp();
                        HF
                    } else {
                        let HG = (-HD) - GX;
                        let HH = HA / (A + (HG * (A + ((AL * HG) * (A + (HG * GZ))))));
                        HH
                    };
                    let HK = (HJ / (if ((A + (((parameters[241] * (A + (parameters[242] * AE))) * (HC - A)) / GU)) + ((parameters[245] * (HI - A)) / HD)) >= Q { ((A + (((parameters[241] * (A + (parameters[242] * AE))) * (HC - A)) / GU)) + ((parameters[245] * (HI - A)) / HD)) } else { Q })) * (if ((A + (parameters[247] * AE)) + ((parameters[248] * AE) * ((A + (AA / parameters[249])).ln()))) >= Q { ((A + (parameters[247] * AE)) + ((parameters[248] * AE) * ((A + (AA / parameters[249])).ln()))) } else { Q });
                    let HL = (HK * AA) / X;
                    oHL = HL;
                    let HN = if HL >= HM { HL } else { HM };
                    oHN = HN;
                    let HO = parameters[250] * HN;
                    oHO = HO;
                    let HP = ((parameters[251] * (A + (parameters[252] * AD))) * (A + (parameters[253] * AE))) * (A + (parameters[254] * AF));
                    let HQ = if (((parameters[255] + (parameters[256] * (AD.powf(parameters[257])))) * (A + (parameters[258] * AE))) * (A + (parameters[259] * AF))) >= GB { (((parameters[255] + (parameters[256] * (AD.powf(parameters[257])))) * (A + (parameters[258] * AE))) * (A + (parameters[259] * AF))) } else { GB };
                    let HR = ((parameters[262] * (A + (parameters[263] * AD))) * (A + (parameters[264] * AE))) * (A + (parameters[265] * AF));
                    let HS = ((parameters[274] + (parameters[275] * (AD.powf(parameters[276])))) * (A + (parameters[277] * AE))) * (A + (parameters[278] * AF));
                    let HT = if ((parameters[282] * AE) * (A + (parameters[283] * AE))) >= GB { ((parameters[282] * AE) * (A + (parameters[283] * AE))) } else { GB };
                    let HU = ((HK * (parameters[289] + (parameters[290] * (AD.powf(parameters[291]))))) * (A + (parameters[292] * AE))) * (A + (parameters[293] * AF));
                    oHU = HU;
                    let HV = if HU >= GB { HU } else { GB };
                    oHV = HV;
                    let HW = ((parameters[294] * (A + (parameters[295] * AD))) * (A + (parameters[296] * AE))) * (A + (parameters[297] * AF));
                    let HY = if (if (parameters[300] / (A + ((parameters[301] * (AD.powf(parameters[302]))) / (A + (parameters[303] * (AD.powf(parameters[304]))))))) >= A { (parameters[300] / (A + ((parameters[301] * (AD.powf(parameters[302]))) / (A + (parameters[303] * (AD.powf(parameters[304]))))))) } else { A }) <= HX { (if (parameters[300] / (A + ((parameters[301] * (AD.powf(parameters[302]))) / (A + (parameters[303] * (AD.powf(parameters[304]))))))) >= A { (parameters[300] / (A + ((parameters[301] * (AD.powf(parameters[302]))) / (A + (parameters[303] * (AD.powf(parameters[304]))))))) } else { A }) } else { HX };
                    let HZ = if (((parameters[305] * (AD.powf(parameters[306]))) * (A + (parameters[309] * AE))) / (A + (parameters[307] * (AD.powf(parameters[308]))))) >= GB { (((parameters[305] * (AD.powf(parameters[306]))) * (A + (parameters[309] * AE))) / (A + (parameters[307] * (AD.powf(parameters[308]))))) } else { GB };
                    let IA = if (((parameters[310] * (AD.powf(parameters[311]))) * (A + (parameters[314] * AE))) / (A + (parameters[312] * (AD.powf(parameters[313]))))) >= GB { (((parameters[310] * (AD.powf(parameters[311]))) * (A + (parameters[314] * AE))) / (A + (parameters[312] * (AD.powf(parameters[313]))))) } else { GB };
                    let IB = parameters[319] / AF;
                    let IC = parameters[320] / AE;
                    let ID = parameters[321] / AE;
                    let IE = parameters[335] / AE;
                    let IF = parameters[336] / AE;
                    let IG = parameters[322] / AE;
                    let IH = parameters[323] / AE;
                    let II = parameters[331] * AD;
                    let IJ = if (parameters[339] + (parameters[341] / AE)) >= GB { (parameters[339] + (parameters[341] / AE)) } else { GB };
                    let IK = if (parameters[340] + (parameters[342] / AE)) >= GB { (parameters[340] + (parameters[342] / AE)) } else { GB };
                    let IL = parameters[349] + (parameters[351] * AD);
                    let IM = parameters[350] + (parameters[352] * AD);
                    let IN = if ((parameters[384] * (A + (parameters[385] * AD))) * (A + (parameters[386] * AE))) >= GB { ((parameters[384] * (A + (parameters[385] * AD))) * (A + (parameters[386] * AE))) } else { GB };
                    let IO = if ((parameters[389] * (A + (parameters[390] * AD))) * (A + (parameters[391] * AE))) >= GB { ((parameters[389] * (A + (parameters[390] * AD))) * (A + (parameters[391] * AE))) } else { GB };
                    let IP = ((parameters[356] + (parameters[357] * (AD.powf(parameters[358])))) + (parameters[359] * AE)) + (parameters[360] * AF);
                    let IQ = ((parameters[362] * (A + (parameters[363] * AD))) * (A + (parameters[364] * AE))) * (A + (parameters[365] * AF));
                    let IR = if (if ((staged[7] * (GA.powf(parameters[369]))) * (A + (parameters[370] * AE))) >= GB { ((staged[7] * (GA.powf(parameters[369]))) * (A + (parameters[370] * AE))) } else { GB }) <= GC { (if ((staged[7] * (GA.powf(parameters[369]))) * (A + (parameters[370] * AE))) >= GB { ((staged[7] * (GA.powf(parameters[369]))) * (A + (parameters[370] * AE))) } else { GB }) } else { GC };
                    let IS = ((parameters[371] * IR) * GF) / GG;
                    let IT = if (parameters[372] * ((GA.powf(parameters[373])) * (A + (parameters[374] * AE)))) >= GB { (parameters[372] * ((GA.powf(parameters[373])) * (A + (parameters[374] * AE)))) } else { GB };
                    let IU = ((parameters[375] * IT) * GF) / GG;
                    let IV = ((HJ * (staged[6] + (parameters[354] * AA))) / ((if (A + ((staged[8] / X) * (A - ((GT / parameters[378]).exp())))) >= 1e-15f64 { (A + ((staged[8] / X) * (A - ((GT / parameters[378]).exp())))) } else { 1e-15f64 }) * X)) * (A + (parameters[379] * AE));
                    let IW = ((parameters[380] + (parameters[381] * AD)) + (parameters[382] * AE)) + ((parameters[383] * AD) * AE);
                    let IX = AC * AB;
                    let IY = if (parameters[392] + (parameters[393] * AH)) >= GB { (parameters[392] + (parameters[393] * AH)) } else { GB };
                    let IZ = (parameters[395] * AC) / Q;
                    let JB;
                    let JC;
                    let JD;
                    let JE;
                    let JF;
                    let JG;
                    let JH;
                    let JI;
                    let JJ;
                    let JK;
                    let JL;
                    if JA != 0.0 {
                        let JW = (staged[10] * (AD.powf(staged[9]))) / (A + (staged[12] * (AD.powf(staged[11]))));
                        let JX = ((staged[13] + JW) + (staged[14] * AE)) + (staged[15] * AF);
                        let JY = staged[17] + (staged[16] * JW);
                        let JZ = if (if ((staged[19] * (GA.powf(staged[18]))) * (A + (staged[20] * AE))) >= GB { ((staged[19] * (GA.powf(staged[18]))) * (A + (staged[20] * AE))) } else { GB }) <= GC { (if ((staged[19] * (GA.powf(staged[18]))) * (A + (staged[20] * AE))) >= GB { ((staged[19] * (GA.powf(staged[18]))) * (A + (staged[20] * AE))) } else { GB }) } else { GC };
                        let KA = ((GE * JZ) * GF) / GG;
                        let KB = staged[23] * ((GA.powf(staged[21])) * (A + (staged[22] * AE)));
                        let KC = if KB >= GB { KB } else { GB };
                        let KD = ((GO * KC) * GF) / GG;
                        let KE = ((HK * (staged[26] + (staged[25] * (AD.powf(staged[24]))))) * (A + (staged[27] * AE))) * (A + (staged[28] * AF));
                        let KF = if KE >= GB { KE } else { GB };
                        let KG = if (if (staged[33] / (A + ((staged[30] * (AD.powf(staged[29]))) / (A + (staged[32] * (AD.powf(staged[31]))))))) >= A { (staged[33] / (A + ((staged[30] * (AD.powf(staged[29]))) / (A + (staged[32] * (AD.powf(staged[31]))))))) } else { A }) <= HX { (if (staged[33] / (A + ((staged[30] * (AD.powf(staged[29]))) / (A + (staged[32] * (AD.powf(staged[31]))))))) >= A { (staged[33] / (A + ((staged[30] * (AD.powf(staged[29]))) / (A + (staged[32] * (AD.powf(staged[31]))))))) } else { A }) } else { HX };
                        let KH = if (((staged[35] * (AD.powf(staged[34]))) * (A + (staged[36] * AE))) / (A + (staged[38] * (AD.powf(staged[37]))))) >= GB { (((staged[35] * (AD.powf(staged[34]))) * (A + (staged[36] * AE))) / (A + (staged[38] * (AD.powf(staged[37]))))) } else { GB };
                        JB = KE;
                        JC = JX;
                        JD = JY;
                        JE = KB;
                        JF = KC;
                        JG = KD;
                        JH = KG;
                        JI = KF;
                        JJ = JZ;
                        JK = KA;
                        JL = KH;
                    } else {
                        JB = HU;
                        JC = FW;
                        JD = FX;
                        JE = GM;
                        JF = GN;
                        JG = GP;
                        JH = HY;
                        JI = HV;
                        JJ = GD;
                        JK = GH;
                        JL = HZ;
                    }
                    oJB = JB;
                    oJI = JI;
                    let JM = staged[39] * AC;
                    let JN = JM * parameters[427];
                    let JO = JM * parameters[428];
                    let JP = parameters[429] / (if (A + (staged[40] / AC)) >= GR { (A + (staged[40] / AC)) } else { GR });
                    let JQ = if (parameters[433] + (parameters[435] * AJ)) >= GB { (parameters[433] + (parameters[435] * AJ)) } else { GB };
                    let JR = if (parameters[434] + (parameters[436] * AJ)) >= GB { (parameters[434] + (parameters[436] * AJ)) } else { GB };
                    let JS = (staged[41] * AA) / X;
                    let JT = if (((A + (parameters[440] * AH)) + (parameters[441] * AJ)) + ((parameters[442] * AH) * AJ)) >= HM { (((A + (parameters[440] * AH)) + (parameters[441] * AJ)) + ((parameters[442] * AH) * AJ)) } else { HM };
                    let JV = if (if C > A { 1.0 } else { 0.0 }) != 0.0 && (if JU > GB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oJV = JV;
                    let KK;
                    if JV != 0.0 {
                        let KI = (-(JU + R)) / parameters[445];
                        let KJ = if (KI.abs()) < GX { 1.0 } else { 0.0 };
                        oKJ = KJ;
                        let KT;
                        if KJ != 0.0 {
                            let KR = KI.exp();
                            KT = KR;
                        } else {
                            let KS = if KI < -8e1f64 { 1.0 } else { 0.0 };
                            oKS = KS;
                            let LB = if KS != 0.0 {
                                let KX = (-KI) - GX;
                                let KY = HA / (A + (KX * (A + ((AL * KX) * (A + (KX * GZ))))));
                                KY
                            } else {
                                let KZ = KI - GX;
                                let LA = 5.54062e34f64 * (A + (KZ * (A + ((AL * KZ) * (A + (KZ * GZ))))));
                                LA
                            };
                            KT = LB;
                        }
                        let KU = A - KT;
                        let KW = (((KV * parameters[446]) * KT) * (KU - ((A - (KT.powf(C))) / C))) / (KU * KU);
                        KK = KW;
                    } else {
                        KK = GB;
                    }
                    let KL = JT / (A + KK);
                    let KM = if (parameters[439] / KL) >= Q { (parameters[439] / KL) } else { Q };
                    let KN = if (parameters[444] * KL) >= GB { (parameters[444] * KL) } else { GB };
                    let KO = ((((parameters[448] * HL) * HL) * AE) * AE) * (AD.powf(staged[42]));
                    let KQ = if ((((parameters[488] * (((GZ * AI) / KP) + AM)) / (KP * AK)) + (staged[43] / (AI * AG))) + (C * parameters[485])) >= GB { ((((parameters[488] * (((GZ * AI) / KP) + AM)) / (KP * AK)) + (staged[43] / (AI * AG))) + (C * parameters[485])) } else { GB };
                    let LC = (C * parameters[39]) * staged[44];
                    let LD = (C * parameters[40]) * staged[45];
                    let LE = C * parameters[492];
                    let LH = if (if (if staged[46] != 0.0 && (if LF > GB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if LG > GB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if C == A { 1.0 } else { 0.0 }) != 0.0 || JV != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oLH = LH;
                    let LJ;
                    let LK;
                    let LL;
                    let LM;
                    let LN;
                    let LO;
                    let LP;
                    let LQ;
                    if LH != 0.0 {
                        let LI = if parameters[457] == A { 1.0 } else { 0.0 };
                        oLI = LI;
                        let LT;
                        let LU;
                        let LV;
                        let LW;
                        let LX;
                        let LY;
                        let LZ;
                        let MA;
                        if LI != 0.0 {
                            let mut MB = 0.0;
                            let mut MC = 0.0;
                            let mut MD = 0.0;
                            MB = GB;
                            MC = GB;
                            MD = GB;
                            loop {
                                let ME = if MB < (C - AL) { 1.0 } else { 0.0 };
                                oME = ME;
                                if ME == 0.0 {
                                    break;
                                }
                                let MF = AL * R;
                                let MG = MB * (JU + R);
                                let MH = MC + (A / ((LF + MF) + MG));
                                let MI = MD + (A / ((LG + MF) + MG));
                                let MJ = MB + A;
                                MB = MJ;
                                MC = MH;
                                MD = MI;
                            }
                            let MK = AL * R;
                            let MM = A / (ML + MK);
                            let MO = A / (MN + MK);
                            let MQ = if (Y + MP) >= E { (Y + MP) } else { E };
                            let MR = A / (AG.powf(parameters[467]));
                            let MS = A / (MQ.powf(parameters[468]));
                            let MT = ((A + (parameters[464] * MR)) + (parameters[465] * MS)) + ((parameters[466] * MR) * MS);
                            oMT = MT;
                            let MU = (MC / C) + (MD / C);
                            let MW = MV * MU;
                            oMW = MW;
                            let MX = MV * (MM + MO);
                            oMX = MX;
                            let MY = A / (AG.powf(parameters[473]));
                            let MZ = A / (MQ.powf(parameters[474]));
                            let NB = if (((A + (parameters[470] * MY)) + (parameters[471] * MZ)) + ((parameters[472] * MY) * MZ)) >= NA { (((A + (parameters[470] * MY)) + (parameters[471] * MZ)) + ((parameters[472] * MY) * MZ)) } else { NA };
                            let NC = (MU - MM) - MO;
                            let ND = (parameters[469] * NC) / NB;
                            let NE = FW + ND;
                            let NF = FX + ND;
                            let NG = JC + ND;
                            let NH = JD + ND;
                            let NI = (parameters[475] * NC) / (NB.powf(parameters[476]));
                            let NJ = if (GM + NI) >= GB { (GM + NI) } else { GB };
                            let NK = if (JE + NI) >= GB { (JE + NI) } else { GB };
                            let NL = (GO * GF) / GG;
                            let NM = NJ * NL;
                            let NN = NK * NL;
                            LT = NJ;
                            LU = NM;
                            LV = NK;
                            LW = NN;
                            LX = NF;
                            LY = NH;
                            LZ = NE;
                            MA = NG;
                        } else {
                            let LS = -1e0f64 / LR;
                            let mut NO = 0.0;
                            let mut NP = 0.0;
                            NO = GB;
                            NP = GB;
                            loop {
                                let NQ = if NO < (C - AL) { 1.0 } else { 0.0 };
                                oNQ = NQ;
                                if NQ == 0.0 {
                                    break;
                                }
                                let NR = AL * R;
                                let NS = JU + R;
                                let NU = (-((LF + NR) + (NO * NS))) / NT;
                                let NV = if NU > -8e1f64 { 1.0 } else { 0.0 };
                                oNV = NV;
                                let OD = if NV != 0.0 {
                                    let OA = NU.exp();
                                    OA
                                } else {
                                    let OB = (-NU) - GX;
                                    let OC = HA / (A + (OB * (A + ((AL * OB) * (A + (OB * GZ))))));
                                    OC
                                };
                                let OE = (-((LG + NR) + (((C - A) - NO) * NS))) / NT;
                                let OF = if OE > -8e1f64 { 1.0 } else { 0.0 };
                                oOF = OF;
                                let OJ = if OF != 0.0 {
                                    let OG = OE.exp();
                                    OG
                                } else {
                                    let OH = (-OE) - GX;
                                    let OI = HA / (A + (OH * (A + ((AL * OH) * (A + (OH * GZ))))));
                                    OI
                                };
                                let OK = -LR;
                                let OL = NP + ((AL * (((A - OD).powf(OK)) + ((A - OJ).powf(OK)))).powf(LS));
                                let OM = NO + A;
                                NO = OM;
                                NP = OL;
                            }
                            let NW = A - (NP / C);
                            oNW = NW;
                            let NX = AL * R;
                            let NY = (-(ML + NX)) / NT;
                            let NZ = if NY > -8e1f64 { 1.0 } else { 0.0 };
                            oNZ = NZ;
                            let OQ = if NZ != 0.0 {
                                let ON = NY.exp();
                                ON
                            } else {
                                let OO = (-NY) - GX;
                                let OP = HA / (A + (OO * (A + ((AL * OO) * (A + (OO * GZ))))));
                                OP
                            };
                            let OR = (-(MN + NX)) / NT;
                            let OS = if OR > -8e1f64 { 1.0 } else { 0.0 };
                            oOS = OS;
                            let OW = if OS != 0.0 {
                                let OT = OR.exp();
                                OT
                            } else {
                                let OU = (-OR) - GX;
                                let OV = HA / (A + (OU * (A + ((AL * OU) * (A + (OU * GZ))))));
                                OV
                            };
                            let OX = -LR;
                            let OY = A - ((AL * (((A - OQ).powf(OX)) + ((A - OW).powf(OX)))).powf(LS));
                            oOY = OY;
                            let OZ = NW - OY;
                            let PA = (parameters[479] * OZ) / (if (A + ((parameters[480] * (if (Y + MP) >= E { (Y + MP) } else { E })) / Q)) >= NA { (A + ((parameters[480] * (if (Y + MP) >= E { (Y + MP) } else { E })) / Q)) } else { NA });
                            let PB = FW + PA;
                            let PC = FX + PA;
                            let PD = JC + PA;
                            let PE = JD + PA;
                            let PF = ((parameters[481] * OZ) * GJ) * GK;
                            let PG = if (GM + PF) >= GB { (GM + PF) } else { GB };
                            let PH = if (JE + PF) >= GB { (JE + PF) } else { GB };
                            let PI = (GO * GF) / GG;
                            let PJ = PG * PI;
                            let PK = PH * PI;
                            LT = PG;
                            LU = PJ;
                            LV = PH;
                            LW = PK;
                            LX = PC;
                            LY = PE;
                            LZ = PB;
                            MA = PD;
                        }
                        LJ = LT;
                        LK = LU;
                        LL = LV;
                        LM = LW;
                        LN = LX;
                        LO = LY;
                        LP = LZ;
                        LQ = MA;
                    } else {
                        LJ = GN;
                        LK = GP;
                        LL = JF;
                        LM = JG;
                        LN = FX;
                        LO = JD;
                        LP = FW;
                        LQ = JC;
                    }
                    AN = IC;
                    AO = IE;
                    AP = IG;
                    AQ = IJ;
                    AR = IL;
                    AS = JN;
                    AT = JQ;
                    AU = GI;
                    AV = GQ;
                    AW = LJ;
                    AX = LK;
                    AY = LL;
                    AZ = LM;
                    BA = LN;
                    BB = LO;
                    BC = FZ;
                    BD = FY;
                    BE = LP;
                    BF = LQ;
                    BG = HP;
                    BH = HR;
                    BI = HQ;
                    BJ = HS;
                    BK = HT;
                    BL = HY;
                    BM = JH;
                    BN = HW;
                    BO = IA;
                    BP = IB;
                    BQ = ID;
                    BR = IH;
                    BS = IF;
                    BT = IK;
                    BU = IQ;
                    BV = IP;
                    BW = IW;
                    BX = IV;
                    BY = IX;
                    BZ = IZ;
                    CA = KM;
                    CB = KO;
                    CC = KQ;
                    CD = LC;
                    CE = LD;
                    CF = LE;
                    CG = GD;
                    CH = GH;
                    CI = HZ;
                    CJ = GS;
                    CK = JO;
                    CL = II;
                    CM = IM;
                    CN = IR;
                    CO = IS;
                    CP = IT;
                    CQ = IU;
                    CR = IO;
                    CS = IN;
                    CT = JJ;
                    CU = JK;
                    CV = JL;
                    CW = JR;
                    CX = JP;
                    CY = IY;
                    CZ = JS;
                    DA = J;
                    DB = N;
                    DC = H;
                    DD = L;
                    DE = KN;
                    DF = P;
                }
                let PL;
                let PM;
                let PN;
                let PO;
                let PP;
                let PQ;
                let PR;
                if DG != 0.0 {
                    PL = AN;
                    PM = AP;
                    PN = AO;
                    PO = AQ;
                    PP = AS;
                    PQ = AR;
                    PR = AT;
                } else {
                    PL = BQ;
                    PM = BR;
                    PN = BS;
                    PO = BT;
                    PP = CK;
                    PQ = CM;
                    PR = CW;
                }
                let PS = if AU > GB { 1.0 } else { 0.0 };
                let PX;
                let PY;
                if PS != 0.0 {
                    let PU = PT * (A + AU);
                    PX = PU;
                    PY = PV;
                } else {
                    let PW = PV * (A - AU);
                    PX = PT;
                    PY = PW;
                }
                let QA = PX / PZ;
                let QB = PY / PZ;
                let QC = A / QA;
                let QD = A / ((A + QC) + (A / QB));
                let QE = PX + PY;
                let QF = staged[65] / QE;
                let QI = ((staged[80] * BC).sqrt()) / PT;
                let QK = QJ * BD;
                let QN = ((QM * (((((HX / BL) * QL).exp()) - A).ln())).exp()) - A;
                let QO = ((QM * (((((HX / BM) * QL).exp()) - A).ln())).exp()) - A;
                let QQ = AQ * QP;
                let QR = PO * QP;
                let QS = QJ * BU;
                let QT = BZ * 1.25e-6f64;
                let QU = 9.10938291e-19f64 * CB;
                let QV = if CC > GB { 1.0 } else { 0.0 };
                let QX = if QV != 0.0 {
                    let QW = A / CC;
                    QW
                } else {
                    GB
                };
                let QY = if CD > GB { 1.0 } else { 0.0 };
                let RA = if QY != 0.0 {
                    let QZ = A / CD;
                    QZ
                } else {
                    GB
                };
                let RB = if CE > GB { 1.0 } else { 0.0 };
                let RD = if RB != 0.0 {
                    let RC = A / CE;
                    RC
                } else {
                    GB
                };
                let RE = if CF > GB { 1.0 } else { 0.0 };
                let RG = if RE != 0.0 {
                    let RF = A / CF;
                    RF
                } else {
                    GB
                };
                if RH != 0.0 {
                    let RI = staged[140] / QE;
                    oRI = RI;
                } else {
                }
                if QG != 0.0 {
                    let RJ = A + QA;
                    let RK = A + QB;
                    let RL = RJ / RK;
                    let RM = RL.ln();
                    let RO = if RM > RN { 1.0 } else { 0.0 };
                    oRO = RO;
                    let RR = if RO != 0.0 {
                        let RP = ((KV * RM) * (RL + A)) / (RL - A);
                        RP
                    } else {
                        let RQ = KV * (KV + RM);
                        RQ
                    };
                    let RS = A / RK;
                    oRS = RS;
                    let RT = (QA + (QB * RS)) * RR;
                    oRT = RT;
                    let RU = (QB + (QA * (A / RJ))) * RR;
                    oRU = RU;
                } else {
                }
                if QH != 0.0 {
                    let RW = (RV * QI) * QI;
                    oRW = RW;
                    let RX = AL * QI;
                    oRX = RX;
                } else {
                }
                let RZ = if RY != 0.0 && (if QQ > GB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let SA = if AS > GB { 1.0 } else { 0.0 };
                let SB = if RY != 0.0 && (if QR > GB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let SC = if PP > GB { 1.0 } else { 0.0 };
                if SD != 0.0 {
                    let SE = A / (A + CN);
                    oSE = SE;
                    let SF = A / (A + CO);
                    oSF = SF;
                    let SG = A / (QA / SE);
                    oSG = SG;
                    let SH = A / (QB / SF);
                    oSH = SH;
                    let SI = A / ((A + SG) + SH);
                    oSI = SI;
                } else {
                }
                if SJ != 0.0 {
                    if QG != 0.0 {
                        let SK = A + QA;
                        let SL = A + QB;
                        let SM = SK / SL;
                        let SN = SM.ln();
                        let SO = if SN > RN { 1.0 } else { 0.0 };
                        oSO = SO;
                        let SR = if SO != 0.0 {
                            let SP = ((KV * SN) * (SM + A)) / (SM - A);
                            SP
                        } else {
                            let SQ = KV * (KV + SN);
                            SQ
                        };
                        let SS = A / SL;
                        oSS = SS;
                        let ST = (QA + (QB * SS)) * SR;
                        oST = ST;
                        let SU = (QB + (QA * (A / SK))) * SR;
                        oSU = SU;
                    } else {
                    }
                    if QH != 0.0 {
                        let SV = (RV * QI) * QI;
                        oSV = SV;
                        let SW = AL * QI;
                        oSW = SW;
                    } else {
                    }
                } else {
                }
                let SX = if BZ > GB { 1.0 } else { 0.0 };
                let SZ = -((PV * DA) + (SY * DB));
                let TA = -((PV * DC) + (SY * DD));
                let TB = parameters[31] * DF;
                let TF;
                let TG;
                if QV != 0.0 {
                    let TC = TB * QX;
                    oTC = TC;
                    TF = TD;
                    TG = GB;
                } else {
                    TF = GB;
                    TG = TE;
                }
                let TK;
                let TL;
                if QY != 0.0 {
                    let TH = TB * RA;
                    oTH = TH;
                    TK = TI;
                    TL = GB;
                } else {
                    TK = GB;
                    TL = TJ;
                }
                let TP;
                let TQ;
                if RB != 0.0 {
                    let TM = TB * RD;
                    oTM = TM;
                    TP = TN;
                    TQ = GB;
                } else {
                    TP = GB;
                    TQ = TO;
                }
                let TU;
                let TV;
                if RE != 0.0 {
                    let TR = TB * RG;
                    oTR = TR;
                    TU = TS;
                    TV = GB;
                } else {
                    TU = GB;
                    TV = TT;
                }
                let TW = parameters[32] * DF;
                let TX = if CB > GB { 1.0 } else { 0.0 };
                let TY = staged[284] / QE;
            [oGV, oHE, oHL, oHN, oHO, oHU, oHV, oJV, oKJ, oKS, oLH, oLI, oME, oMT, oMW, oMX, oJB, oNQ, oNV, oOF, oNW, oNZ, oOS, oOY, AN, AO, AP, AR, AS, AT, PS, PX, QA, PY, QB, QC, QD, QF, AV, AW, AX, AY, AZ, BA, BB, BC, QK, BE, BF, BG, BH, BI, BJ, BK, QN, QO, BN, oJI, BO, BP, PL, PM, PN, QQ, QR, QS, BV, BW, BX, BY, QT, CA, QU, QV, QY, RB, RE, oRI, CG, CH, CI, oRO, oRS, oRT, oRU, CJ, oRW, oRX, RZ, SA, SB, PP, SC, CL, PQ, oSE, oSF, CP, CQ, oSG, oSH, oSI, CR, CS, CT, CU, CV, oSO, oSS, oST, oSU, oSV, oSW, SX, PR, CX, CY, CZ, SZ, TA, DE, DF, TB, oTC, oTH, oTM, oTR, TW, TX, TY, TF, TG, TK, TL, TP, TQ, TU, TV]
        };
        self.canonical_staged[337] = produced[0];
        self.canonical_staged[338] = produced[1];
        self.canonical_staged[50] = produced[2];
        self.canonical_staged[376] = produced[3];
        self.canonical_staged[377] = produced[4];
        self.canonical_staged[51] = produced[5];
        self.canonical_staged[378] = produced[6];
        self.canonical_staged[341] = produced[7];
        self.canonical_staged[371] = produced[8];
        self.canonical_staged[373] = produced[9];
        self.canonical_staged[374] = produced[10];
        self.canonical_staged[375] = produced[11];
        self.canonical_staged[380] = produced[12];
        self.canonical_staged[47] = produced[13];
        self.canonical_staged[48] = produced[14];
        self.canonical_staged[49] = produced[15];
        self.canonical_staged[52] = produced[16];
        self.canonical_staged[381] = produced[17];
        self.canonical_staged[382] = produced[18];
        self.canonical_staged[384] = produced[19];
        self.canonical_staged[53] = produced[20];
        self.canonical_staged[383] = produced[21];
        self.canonical_staged[385] = produced[22];
        self.canonical_staged[54] = produced[23];
        self.canonical_staged[108] = produced[24];
        self.canonical_staged[113] = produced[25];
        self.canonical_staged[110] = produced[26];
        self.canonical_staged[222] = produced[27];
        self.canonical_staged[265] = produced[28];
        self.canonical_staged[261] = produced[29];
        self.canonical_staged[387] = produced[30];
        self.canonical_staged[200] = produced[31];
        self.canonical_staged[169] = produced[32];
        self.canonical_staged[74] = produced[33];
        self.canonical_staged[156] = produced[34];
        self.canonical_staged[153] = produced[35];
        self.canonical_staged[152] = produced[36];
        self.canonical_staged[66] = produced[37];
        self.canonical_staged[67] = produced[38];
        self.canonical_staged[68] = produced[39];
        self.canonical_staged[69] = produced[40];
        self.canonical_staged[70] = produced[41];
        self.canonical_staged[71] = produced[42];
        self.canonical_staged[77] = produced[43];
        self.canonical_staged[78] = produced[44];
        self.canonical_staged[79] = produced[45];
        self.canonical_staged[84] = produced[46];
        self.canonical_staged[86] = produced[47];
        self.canonical_staged[89] = produced[48];
        self.canonical_staged[90] = produced[49];
        self.canonical_staged[95] = produced[50];
        self.canonical_staged[96] = produced[51];
        self.canonical_staged[100] = produced[52];
        self.canonical_staged[103] = produced[53];
        self.canonical_staged[191] = produced[54];
        self.canonical_staged[255] = produced[55];
        self.canonical_staged[104] = produced[56];
        self.canonical_staged[379] = produced[57];
        self.canonical_staged[105] = produced[58];
        self.canonical_staged[107] = produced[59];
        self.canonical_staged[109] = produced[60];
        self.canonical_staged[111] = produced[61];
        self.canonical_staged[114] = produced[62];
        self.canonical_staged[223] = produced[63];
        self.canonical_staged[226] = produced[64];
        self.canonical_staged[126] = produced[65];
        self.canonical_staged[127] = produced[66];
        self.canonical_staged[129] = produced[67];
        self.canonical_staged[130] = produced[68];
        self.canonical_staged[131] = produced[69];
        self.canonical_staged[134] = produced[70];
        self.canonical_staged[136] = produced[71];
        self.canonical_staged[280] = produced[72];
        self.canonical_staged[396] = produced[73];
        self.canonical_staged[397] = produced[74];
        self.canonical_staged[398] = produced[75];
        self.canonical_staged[399] = produced[76];
        self.canonical_staged[141] = produced[77];
        self.canonical_staged[171] = produced[78];
        self.canonical_staged[172] = produced[79];
        self.canonical_staged[197] = produced[80];
        self.canonical_staged[449] = produced[81];
        self.canonical_staged[157] = produced[82];
        self.canonical_staged[154] = produced[83];
        self.canonical_staged[155] = produced[84];
        self.canonical_staged[173] = produced[85];
        self.canonical_staged[192] = produced[86];
        self.canonical_staged[193] = produced[87];
        self.canonical_staged[203] = produced[88];
        self.canonical_staged[458] = produced[89];
        self.canonical_staged[205] = produced[90];
        self.canonical_staged[266] = produced[91];
        self.canonical_staged[459] = produced[92];
        self.canonical_staged[216] = produced[93];
        self.canonical_staged[225] = produced[94];
        self.canonical_staged[235] = produced[95];
        self.canonical_staged[236] = produced[96];
        self.canonical_staged[233] = produced[97];
        self.canonical_staged[234] = produced[98];
        self.canonical_staged[241] = produced[99];
        self.canonical_staged[242] = produced[100];
        self.canonical_staged[240] = produced[101];
        self.canonical_staged[244] = produced[102];
        self.canonical_staged[245] = produced[103];
        self.canonical_staged[253] = produced[104];
        self.canonical_staged[254] = produced[105];
        self.canonical_staged[258] = produced[106];
        self.canonical_staged[463] = produced[107];
        self.canonical_staged[249] = produced[108];
        self.canonical_staged[247] = produced[109];
        self.canonical_staged[248] = produced[110];
        self.canonical_staged[256] = produced[111];
        self.canonical_staged[257] = produced[112];
        self.canonical_staged[468] = produced[113];
        self.canonical_staged[262] = produced[114];
        self.canonical_staged[263] = produced[115];
        self.canonical_staged[267] = produced[116];
        self.canonical_staged[268] = produced[117];
        self.canonical_staged[270] = produced[118];
        self.canonical_staged[271] = produced[119];
        self.canonical_staged[272] = produced[120];
        self.canonical_staged[274] = produced[121];
        self.canonical_staged[273] = produced[122];
        self.canonical_staged[275] = produced[123];
        self.canonical_staged[276] = produced[124];
        self.canonical_staged[277] = produced[125];
        self.canonical_staged[278] = produced[126];
        self.canonical_staged[279] = produced[127];
        self.canonical_staged[469] = produced[128];
        self.canonical_staged[285] = produced[129];
        self.canonical_staged[471] = produced[130];
        self.canonical_staged[472] = produced[131];
        self.canonical_staged[473] = produced[132];
        self.canonical_staged[474] = produced[133];
        self.canonical_staged[475] = produced[134];
        self.canonical_staged[476] = produced[135];
        self.canonical_staged[477] = produced[136];
        self.canonical_staged[478] = produced[137];
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
        let produced: [f64; 97] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let B = staged[306];
                let E = 5e-1f64;
                let G = 8.617332384961e-5f64;
                let H = 1e1f64;
                let J = 6e2f64;
                let L = 1e-2f64;
                let N = 1e0f64;
                let T = staged[0];
                let Z = staged[308];
                let AE = parameters[68];
                let AF = staged[321];
                let AG = parameters[93];
                let AH = staged[323];
                let AI = staged[374];
                let AJ = staged[375];
                let AK = staged[376];
                let AL = staged[377];
                let AM = staged[378];
                let AN = staged[379];
                let AW = staged[380];
                let BB = staged[50];
                let BD = 1e-10f64;
                let BF = parameters[250];
                let BH = parameters[462];
                let BJ = staged[51];
                let BK = 0e0f64;
                let BM = staged[52];
                let BO = staged[381];
                let BW = parameters[484];
                let CA = 4.73e-4f64;
                let CC = 1.17e0f64;
                let CE = 4.774e-4f64;
                let CG = 7.44e-1f64;
                let CH = staged[56];
                let CM = staged[57];
                let CV = staged[63];
                let CX = staged[64];
                let CY = 6.931471805599e-1f64;
                let DG = staged[72];
                let DP = 2e0f64;
                let DR = staged[388];
                let DT = staged[77];
                let DV = staged[78];
                let DZ = staged[389];
                let EA = staged[79];
                let ED = 2.97e3f64;
                let EF = 1.5e1f64;
                let EH = 1e-6f64;
                let EK = staged[390];
                let EL = staged[391];
                let EN = staged[85];
                let EP = parameters[34];
                let ER = staged[86];
                let ES = staged[87];
                let ET = parameters[14];
                let EV = staged[88];
                let EX = staged[89];
                let FB = parameters[35];
                let FK = staged[392];
                let FL = staged[81];
                let GD = staged[115];
                let GI = staged[120];
                let HA = staged[400];
                let HN = 8e1f64;
                let HT = 3.333333333333e-1f64;
                let HU = 1.80485e-35f64;
                let HX = 5.54062e34f64;
                let IB = staged[460];
                let IF = staged[462];
                let mut oHH = 0.0;
                let mut oHO = 0.0;
                let mut oHQ = 0.0;
                let mut oHR = 0.0;
                let mut oIA = 0.0;
                let mut oIC = 0.0;
                let mut oID = 0.0;
                let mut oIE = 0.0;
                let mut oIH = 0.0;
                let mut oIJ = 0.0;
                let mut oIK = 0.0;
                let mut oIQ = 0.0;
                let mut oJI = 0.0;
                let mut oJK = 0.0;
                let mut oJL = 0.0;
                let mut oJM = 0.0;
                let A = if (temperature + parameters[36]) <= 1e3f64 { (temperature + parameters[36]) } else { 1e3f64 };
                let Q;
                let R;
                if B != 0.0 {
                    let C = parameters[17] + (parameters[18] * A);
                    let D = A - C;
                    let F = E * ((A + C) + (((D * D) + parameters[19]).sqrt()));
                    let I = H / (F * G);
                    let K = I - J;
                    let M = E * ((I + J) + (((K * K) + L).sqrt()));
                    Q = F;
                    R = M;
                } else {
                    let O = A - N;
                    let P = E * ((A + N) + (((O * O) + 1e-3f64).sqrt()));
                    Q = P;
                    R = J;
                }
                let S = Q * Q;
                let U = Q - T;
                let V = Q / T;
                let W = T / Q;
                let X = Q * G;
                let Y = N / X;
                let AA;
                let AB;
                let AC;
                let AD;
                if Z != 0.0 {
                    AA = AE;
                    AB = AF;
                    AC = AG;
                    AD = AH;
                } else {
                    let AO;
                    let AP;
                    let AQ;
                    let AR;
                    if AI != 0.0 {
                        let AS;
                        let AT;
                        let AU;
                        let AV;
                        if AJ != 0.0 {
                            loop {
                                if AW == 0.0 {
                                    break;
                                }
                            }
                            let AX = staged[47] * (N + (parameters[463] * (V - N)));
                            let AY = staged[48] / AX;
                            let AZ = staged[49] / AX;
                            let BA = N + AY;
                            let BC = N + AZ;
                            let BE = if ((BB * BA) / BC) >= BD { ((BB * BA) / BC) } else { BD };
                            let BG = BF * BE;
                            let BI = (BA * (N + (BH * AZ))) / (BC * (N + (BH * AY)));
                            let BL = if (BJ * BI) >= BK { (BJ * BI) } else { BK };
                            let BN = if (BM * BI) >= BK { (BM * BI) } else { BK };
                            AS = BE;
                            AT = BG;
                            AU = BL;
                            AV = BN;
                        } else {
                            loop {
                                if BO == 0.0 {
                                    break;
                                }
                            }
                            let BP = parameters[482] / (N + (parameters[483] * (V - N)));
                            let BQ = BP * staged[53];
                            let BR = BP * staged[54];
                            let BS = N + BQ;
                            let BT = N + BR;
                            let BU = if ((BB * BS) / BT) >= BD { ((BB * BS) / BT) } else { BD };
                            let BV = BF * BU;
                            let BX = (BS * (N + (BW * BR))) / (BT * (N + (BW * BQ)));
                            let BY = if (BJ * BX) >= BK { (BJ * BX) } else { BK };
                            let BZ = if (BM * BX) >= BK { (BM * BX) } else { BK };
                            AS = BU;
                            AT = BV;
                            AU = BY;
                            AV = BZ;
                        }
                        AO = AS;
                        AP = AT;
                        AQ = AU;
                        AR = AV;
                    } else {
                        AO = AK;
                        AP = AL;
                        AQ = AM;
                        AR = AN;
                    }
                    AA = AO;
                    AB = AP;
                    AC = AQ;
                    AD = AR;
                }
                let CB = 6.36e2f64 + Q;
                let CD = CC - ((CA * S) / CB);
                let CF = 2.35e2f64 + Q;
                let CI = (((CG - ((CE * S) / CF)) - CD) + staged[55]) * CH;
                let CJ = E * (CD + CI);
                let CK = CJ * Y;
                let CL = E * CI;
                let CN = CM - CL;
                let CO = (Q * 3.3333333333e-3f64).sqrt();
                let CP = ((4.05e25f64 * CO) * CO) * CO;
                let CQ = CP * staged[58];
                let CR = CP * ((CL * Y).exp());
                let CS = X * (N + (staged[61] * W));
                let CT = N / CS;
                let CU = CJ * CT;
                let CW = ((3.20435313e-19f64 * CQ) * CV) * CT;
                let CZ = ((CX / CW).ln()) - CY;
                let DA = staged[66] * CT;
                let DB = staged[67] * U;
                let DC = staged[68] + DB;
                let DD = staged[69] + DB;
                let DE = staged[70] + DB;
                let DF = staged[71] + DB;
                let DH = DG * CT;
                let DI = ((staged[73] * Y).sqrt()) / staged[74];
                let DJ = DI * DI;
                let DK = N / DJ;
                let DL = N + (DI / 1.4142135623731e0f64);
                let DM = N / DL;
                let DN = 1e-5f64 * DL;
                let DO = ((staged[75] / CR).ln()) + CK;
                let DQ = DP * DO;
                let DX;
                let DY;
                if DR != 0.0 {
                    let DS = (staged[76] * X) * DO;
                    let DU = DT + DS;
                    let DW = DV + DS;
                    DX = DU;
                    DY = DW;
                } else {
                    DX = DT;
                    DY = DV;
                }
                let EC = if DZ != 0.0 {
                    let EB = X * (((EA / CR).ln()) + CK);
                    EB
                } else {
                    BK
                };
                let EJ = if B != 0.0 {
                    let EE = ED / Q;
                    let EG = EF - EE;
                    let EI = E * ((EF + EE) + (((EG * EG) + EH).sqrt()));
                    EI
                } else {
                    EF
                };
                let EM;
                if EK != 0.0 {
                    let FO = if EL != 0.0 {
                        let FM = staged[82] * ((-3.333333333333e-1f64 * ((CS * FL).ln())).exp());
                        FM
                    } else {
                        let FN = staged[83] * ((-3.333333333333e-1f64 * ((CS * FL).ln())).exp());
                        FN
                    };
                    EM = FO;
                } else {
                    EM = BK;
                }
                let EO = (staged[84] * U) + EN;
                let EQ = (EO + EP) - EC;
                let EU = (ET * ((ER + CN) + ES)) + EQ;
                let EW = (ET * ((DX + CN) + EV)) + EO;
                let EY = (ET * ((EX + CN) + ES)) + EQ;
                let EZ = (ET * ((DY + CN) + EV)) + EO;
                let FA = W.ln();
                let FC = ((staged[90] * FA).exp()) * FB;
                let FD = AA * FC;
                let FE = AB * FC;
                let FF = staged[94] * ((staged[93] * FA).exp());
                let FG = staged[96] * ((staged[95] * FA).exp());
                let FH = staged[98] * ((staged[97] * FA).exp());
                let FI = staged[100] * ((staged[99] * FA).exp());
                let FJ = ((1e-8f64 * CS) / staged[101]) * (staged[92] * ((staged[91] * FA).exp()));
                let FP = (DP * (staged[103] * ((staged[102] * FA).exp()))) * CS;
                let FQ = (staged[104] * FA).exp();
                let FR = ((AC * FQ) * FC) * CS;
                let FS = ((AD * FQ) * FC) * CS;
                let FT = staged[105] * CT;
                let FU = (staged[106] * FA).exp();
                let FV = staged[107] * FU;
                let FW = staged[108] * FU;
                let FX = staged[109] * FU;
                let FY = staged[110] * FU;
                let FZ = staged[111] * FU;
                let GA = (staged[112] * FA).exp();
                let GB = staged[113] * GA;
                let GC = staged[114] * GA;
                let GE = GD * CS;
                let GF = GD * X;
                let GG = N / (N + (staged[116] * CU));
                let GH = N + (staged[118] * U);
                let GJ = (staged[119] * (E * (GH + (((GH * GH) + L).sqrt())))) * GI;
                let GK = N + (staged[121] * U);
                let GL = (staged[122] * (E * (GK + (((GK * GK) + L).sqrt())))) * GI;
                let GM = staged[124] * ((staged[123] * FA).exp());
                let GN = X * (N + (staged[125] * W));
                let GO = N / GN;
                let GP = ((3.20435313e-19f64 * CQ) * CV) * GO;
                let GQ = (staged[126] * U) + EN;
                let GR = (((ET * ((staged[127] + CN) + ES)) + GQ) + EP) - EC;
                let GS = (ET * ((staged[128] + CN) + EV)) + GQ;
                let GT = staged[130] * (((staged[129] * FA).exp()) * FB);
                let GU = staged[131] * CS;
                let GV = staged[132] / (CV * CS);
                let GW = (staged[133] / CQ).ln();
                let GX = staged[134] * CS;
                let GY = staged[136] * ((staged[135] * FA).exp());
                let GZ = staged[137] * (5.5225952e-23f64 * Q);
                let HB;
                let HC;
                if HA != 0.0 {
                    let HG = if B != 0.0 {
                        let HD = H / X;
                        let HE = HD - J;
                        let HF = E * ((HD + J) + (((HE * HE) + L).sqrt()));
                        HF
                    } else {
                        J
                    };
                    if DZ != 0.0 {
                        let HH = ((EA / CR).ln()) + CK;
                        oHH = HH;
                    } else {
                    }
                    let HL = if B != 0.0 {
                        let HI = ED / Q;
                        let HJ = EF - HI;
                        let HK = E * ((EF + HI) + (((HJ * HJ) + EH).sqrt()));
                        HK
                    } else {
                        EJ
                    };
                    HB = HL;
                    HC = HG;
                } else {
                    HB = EJ;
                    HC = R;
                }
                if DR != 0.0 {
                    let HM = -DQ;
                    let HO = if (HM.abs()) < HN { 1.0 } else { 0.0 };
                    oHO = HO;
                    let HR;
                    if HO != 0.0 {
                        let HP = HM.exp();
                        HR = HP;
                    } else {
                        let HQ = if HM < -8e1f64 { 1.0 } else { 0.0 };
                        oHQ = HQ;
                        let HZ = if HQ != 0.0 {
                            let HS = (-HM) - HN;
                            let HV = HU / (N + (HS * (N + ((E * HS) * (N + (HS * HT))))));
                            HV
                        } else {
                            let HW = HM - HN;
                            let HY = HX * (N + (HW * (N + ((E * HW) * (N + (HW * HT))))));
                            HY
                        };
                        HR = HZ;
                    }
                    oHR = HR;
                } else {
                }
                if EK != 0.0 {
                    let IA = HB * HB;
                    oIA = IA;
                } else {
                }
                if IB != 0.0 {
                    let IC = staged[230] * GO;
                    oIC = IC;
                    let ID = DP * IC;
                    oID = ID;
                    let IE = ((GN * GN) * GT) * staged[200];
                    oIE = IE;
                } else {
                }
                if IF != 0.0 {
                    if DR != 0.0 {
                        let IG = -DQ;
                        let IH = if (IG.abs()) < HN { 1.0 } else { 0.0 };
                        oIH = IH;
                        let IK;
                        if IH != 0.0 {
                            let II = IG.exp();
                            IK = II;
                        } else {
                            let IJ = if IG < -8e1f64 { 1.0 } else { 0.0 };
                            oIJ = IJ;
                            let IP = if IJ != 0.0 {
                                let IL = (-IG) - HN;
                                let IM = HU / (N + (IL * (N + ((E * IL) * (N + (IL * HT))))));
                                IM
                            } else {
                                let IN = IG - HN;
                                let IO = HX * (N + (IN * (N + ((E * IN) * (N + (IN * HT))))));
                                IO
                            };
                            IK = IP;
                        }
                        oIK = IK;
                    } else {
                    }
                    if EK != 0.0 {
                        let IQ = HB * HB;
                        oIQ = IQ;
                    } else {
                    }
                } else {
                }
                let IR = CC - (((CA * Q) * Q) / CB);
                let IS = (((CG - (((CE * Q) * Q) / CF)) - IR) + staged[282]) * CH;
                let IT = (E * (IR + IS)) * Y;
                let IU = CM - (E * IS);
                let IV = Y / (N + (staged[283] / Q));
                let IW = ((3.20435313e-19f64 * CQ) * CV) * IV;
                let IX = (((CX / IW).ln()) - CY) + IT;
                let IY = staged[285] * IV;
                let IZ = DG * IV;
                let JB = if DZ != 0.0 {
                    let JA = (N / Y) * ((EA / CR).ln());
                    JA
                } else {
                    BK
                };
                let JC;
                if EK != 0.0 {
                    let JH = if FK != 0.0 {
                        let JF = staged[286] * ((-3.333333333333e-1f64 * ((FL / IV).ln())).exp());
                        JF
                    } else {
                        let JG = staged[287] * ((-3.333333333333e-1f64 * ((FL / IV).ln())).exp());
                        JG
                    };
                    JC = JH;
                } else {
                    JC = BK;
                }
                let JD = (((ET * ((ER + IU) + ES)) + EO) + EP) - JB;
                let JE = (ET * ((DX + IU) + EV)) + EO;
                if EK != 0.0 {
                    let JI = HB * HB;
                    oJI = JI;
                } else {
                }
                let JJ = DP * IZ;
                if IF != 0.0 {
                    let JK = (((ET * ((EX + IU) + ES)) + EO) + EP) - JB;
                    oJK = JK;
                    let JL = (ET * ((DY + IU) + EV)) + EO;
                    oJL = JL;
                    if EK != 0.0 {
                        let JM = HB * HB;
                        oJM = JM;
                    } else {
                    }
                } else {
                }
            [Q, X, Y, CJ, CK, CS, CT, CW, CZ, DA, DC, DD, DE, DF, DH, DI, DJ, DK, DL, DM, DN, DQ, EC, EU, DX, EW, EY, DY, EZ, FA, AA, FD, AB, FE, FF, FG, FH, FI, FJ, FP, AC, FR, AD, FS, FT, FV, FW, FX, FY, FZ, GB, GC, GE, GF, GG, GJ, GL, GM, GO, GP, GR, GS, GU, GV, GW, GX, GY, GZ, oHH, oHO, oHQ, oHR, HB, oIA, EM, HC, oIC, oID, oIE, oIH, oIJ, oIK, oIQ, IT, IV, IW, IX, IY, IZ, JD, JE, oJI, JC, JJ, oJK, oJL, oJM]
        };
        self.canonical_staged[138] = produced[0];
        self.canonical_staged[427] = produced[1];
        self.canonical_staged[422] = produced[2];
        self.canonical_staged[433] = produced[3];
        self.canonical_staged[405] = produced[4];
        self.canonical_staged[421] = produced[5];
        self.canonical_staged[401] = produced[6];
        self.canonical_staged[406] = produced[7];
        self.canonical_staged[408] = produced[8];
        self.canonical_staged[409] = produced[9];
        self.canonical_staged[298] = produced[10];
        self.canonical_staged[299] = produced[11];
        self.canonical_staged[441] = produced[12];
        self.canonical_staged[442] = produced[13];
        self.canonical_staged[410] = produced[14];
        self.canonical_staged[162] = produced[15];
        self.canonical_staged[163] = produced[16];
        self.canonical_staged[164] = produced[17];
        self.canonical_staged[165] = produced[18];
        self.canonical_staged[160] = produced[19];
        self.canonical_staged[159] = produced[20];
        self.canonical_staged[166] = produced[21];
        self.canonical_staged[448] = produced[22];
        self.canonical_staged[402] = produced[23];
        self.canonical_staged[145] = produced[24];
        self.canonical_staged[403] = produced[25];
        self.canonical_staged[439] = produced[26];
        self.canonical_staged[146] = produced[27];
        self.canonical_staged[440] = produced[28];
        self.canonical_staged[411] = produced[29];
        self.canonical_staged[147] = produced[30];
        self.canonical_staged[418] = produced[31];
        self.canonical_staged[148] = produced[32];
        self.canonical_staged[419] = produced[33];
        self.canonical_staged[416] = produced[34];
        self.canonical_staged[413] = produced[35];
        self.canonical_staged[414] = produced[36];
        self.canonical_staged[412] = produced[37];
        self.canonical_staged[417] = produced[38];
        self.canonical_staged[415] = produced[39];
        self.canonical_staged[149] = produced[40];
        self.canonical_staged[404] = produced[41];
        self.canonical_staged[150] = produced[42];
        self.canonical_staged[443] = produced[43];
        self.canonical_staged[420] = produced[44];
        self.canonical_staged[431] = produced[45];
        self.canonical_staged[423] = produced[46];
        self.canonical_staged[425] = produced[47];
        self.canonical_staged[424] = produced[48];
        self.canonical_staged[426] = produced[49];
        self.canonical_staged[429] = produced[50];
        self.canonical_staged[430] = produced[51];
        self.canonical_staged[432] = produced[52];
        self.canonical_staged[428] = produced[53];
        self.canonical_staged[434] = produced[54];
        self.canonical_staged[435] = produced[55];
        self.canonical_staged[436] = produced[56];
        self.canonical_staged[437] = produced[57];
        self.canonical_staged[227] = produced[58];
        self.canonical_staged[239] = produced[59];
        self.canonical_staged[228] = produced[60];
        self.canonical_staged[229] = produced[61];
        self.canonical_staged[444] = produced[62];
        self.canonical_staged[445] = produced[63];
        self.canonical_staged[446] = produced[64];
        self.canonical_staged[447] = produced[65];
        self.canonical_staged[438] = produced[66];
        self.canonical_staged[281] = produced[67];
        self.canonical_staged[142] = produced[68];
        self.canonical_staged[450] = produced[69];
        self.canonical_staged[451] = produced[70];
        self.canonical_staged[161] = produced[71];
        self.canonical_staged[167] = produced[72];
        self.canonical_staged[168] = produced[73];
        self.canonical_staged[407] = produced[74];
        self.canonical_staged[176] = produced[75];
        self.canonical_staged[231] = produced[76];
        self.canonical_staged[232] = produced[77];
        self.canonical_staged[243] = produced[78];
        self.canonical_staged[464] = produced[79];
        self.canonical_staged[465] = produced[80];
        self.canonical_staged[251] = produced[81];
        self.canonical_staged[252] = produced[82];
        self.canonical_staged[301] = produced[83];
        self.canonical_staged[288] = produced[84];
        self.canonical_staged[300] = produced[85];
        self.canonical_staged[294] = produced[86];
        self.canonical_staged[295] = produced[87];
        self.canonical_staged[296] = produced[88];
        self.canonical_staged[289] = produced[89];
        self.canonical_staged[290] = produced[90];
        self.canonical_staged[292] = produced[91];
        self.canonical_staged[293] = produced[92];
        self.canonical_staged[297] = produced[93];
        self.canonical_staged[302] = produced[94];
        self.canonical_staged[303] = produced[95];
        self.canonical_staged[305] = produced[96];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[308];
                let B = staged[374];
                let C = staged[375];
                let D = staged[380];
                let E = staged[381];
                if A != 0.0 {
                } else {
                    if B != 0.0 {
                        if C != 0.0 {
                            loop {
                                if D == 0.0 {
                                    break;
                                }
                            }
                        } else {
                            loop {
                                if E == 0.0 {
                                    break;
                                }
                            }
                        }
                    } else {
                    }
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 80177 => 0usize, 80179 => 1usize, 80182 => 2usize, 80187 => 3usize, 80189 => 4usize, 80192 => 5usize, 80197 => 6usize, 80199 => 7usize, 80204 => 8usize, 80208 => 9usize, 80210 => 10usize, 80213 => 11usize, 80216 => 12usize, 80221 => 13usize, 80223 => 14usize, 80227 => 15usize, 80679 => 16usize, 80684 => 17usize, 80689 => 18usize, _ => usize::MAX };
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
            let A = staged[306];
            let B = 0e0f64;
            let D = 1e0f64;
            let E = staged[308];
            let F = staged[374];
            let G = staged[375];
            let H = staged[380];
            let I = staged[381];
            let J = staged[388];
            let K = staged[389];
            let L = staged[390];
            let M = staged[392];
            let N = staged[393];
            let O = staged[396];
            let P = staged[397];
            let Q = staged[398];
            let R = staged[399];
            let S = staged[400];
            let T = node_potentials[4];
            let W = 1e0f64;
            let Z = staged[0];
            let AC = -1e0f64;
            let AE = 8.617332384961e-5f64;
            let AJ = staged[401];
            let AK = staged[402];
            let AL = staged[403];
            let AM = staged[298];
            let AN = staged[299];
            let AO = staged[404];
            let AP = staged[405];
            let AQ = staged[406];
            let AR = staged[407];
            let AS = staged[408];
            let AT = staged[409];
            let AU = staged[410];
            let AV = staged[411];
            let AW = staged[412];
            let AX = staged[413];
            let AY = staged[414];
            let AZ = staged[415];
            let BA = staged[416];
            let BB = staged[417];
            let BC = staged[418];
            let BD = staged[419];
            let BE = staged[420];
            let BF = staged[421];
            let BG = staged[422];
            let BH = staged[423];
            let BI = staged[424];
            let BJ = staged[425];
            let BK = staged[426];
            let BL = staged[427];
            let BM = staged[428];
            let BN = staged[429];
            let BO = staged[430];
            let BP = staged[431];
            let BQ = staged[432];
            let BR = staged[433];
            let BS = staged[434];
            let BT = staged[435];
            let BU = staged[436];
            let BV = staged[437];
            let BW = staged[438];
            let BX = staged[439];
            let BY = staged[440];
            let BZ = staged[441];
            let CA = staged[442];
            let CB = staged[443];
            let CC = staged[444];
            let CD = staged[445];
            let CE = staged[446];
            let CF = staged[447];
            let CG = 0e0f64;
            let GD = 4.73e-4f64;
            let GI = 4.774e-4f64;
            let GL = staged[56];
            let GO = 5e-1f64;
            let GV = 3.3333333333e-3f64;
            let GX = 2e0f64;
            let GY = 1e0f64;
            let HA = 4.05e25f64;
            let HD = staged[58];
            let HG = staged[61];
            let HO = 3.20435313e-19f64;
            let HP = staged[63];
            let HT = staged[64];
            let HW = 6.931471805599e-1f64;
            let HY = staged[141];
            let IB = staged[67];
            let IG = staged[72];
            let IL = staged[142];
            let IO = staged[448];
            let IT = staged[84];
            let IX = staged[87];
            let IY = parameters[14];
            let JC = staged[88];
            let JJ = staged[90];
            let JL = parameters[35];
            let JO = staged[147];
            let JR = staged[148];
            let JU = staged[91];
            let JW = staged[92];
            let JY = staged[93];
            let KA = staged[94];
            let KD = staged[95];
            let KF = staged[96];
            let KI = staged[97];
            let KK = staged[98];
            let KN = staged[99];
            let KP = staged[100];
            let KS = 1e-8f64;
            let KT = staged[101];
            let KX = staged[102];
            let KZ = staged[103];
            let LA = 2e0f64;
            let LE = staged[104];
            let LH = staged[149];
            let LM = staged[150];
            let LR = staged[105];
            let LU = staged[106];
            let LX = staged[107];
            let MA = staged[108];
            let MD = staged[109];
            let MG = staged[110];
            let MJ = staged[111];
            let MM = staged[112];
            let MP = staged[113];
            let MS = staged[114];
            let MV = staged[115];
            let NA = staged[116];
            let NE = staged[118];
            let NI = 1e-2f64;
            let NK = staged[119];
            let NL = staged[120];
            let NO = staged[121];
            let NT = staged[122];
            let NW = staged[123];
            let NY = staged[124];
            let OB = staged[131];
            let OK = staged[134];
            let ON = staged[135];
            let OP = staged[136];
            let OS = staged[81];
            let OU = -3.333333333333e-1f64;
            let OW = staged[143];
            let PA = -3.333333333333e-1f64;
            let PC = staged[144];
            let PH = node_potentials[9];
            let PI = node_potentials[6];
            let PK = 1e0f64;
            let PL = 1e0f64;
            let PN = node_potentials[7];
            let PP = 1e0f64;
            let PR = node_potentials[8];
            let PT = 1e0f64;
            let QQ = -1e0f64;
            let SI = staged[152];
            let SO = staged[153];
            let SQ = 1.5e0f64;
            let SY = 8e1f64;
            let TG = staged[156];
            let TI = staged[157];
            let TT = staged[158];
            let TZ = staged[159];
            let UB = staged[160];
            let UC = 1.666666666667e-1f64;
            let UD = 1.4142135623731e0f64;
            let UG = staged[161];
            let UI = staged[162];
            let UT = 1.25e0f64;
            let UW = 1e1f64;
            let UX = 6e0f64;
            let VA = 6.4e1f64;
            let VH = staged[163];
            let VM = staged[164];
            let WD = 3.333333333333e-1f64;
            let WK = 7.32464877560822e-1f64;
            let WM = staged[165];
            let XB = 5.54062e34f64;
            let XQ = 4e0f64;
            let XR = 8e0f64;
            let XS = 1.2e1f64;
            let YT = 1.80485e-35f64;
            let YY = 2.5e-1f64;
            let ZC = staged[166];
            let ZD = 3e0f64;
            let ZH = 5e0f64;
            let AAD = 1e-40f64;
            let ADO = staged[167];
            let ADR = staged[168];
            let AEA = -3.333333333333e-1f64;
            let AEE = -3.333333333333e-1f64;
            let AEK = staged[62];
            let AEN = staged[169];
            let AFD = Lanes([0e0f64; 5]);
            let AGL = staged[170];
            let AGT = staged[171];
            let AGX = staged[172];
            let AHH = staged[173];
            let AHL = staged[174];
            let AIA = staged[175];
            let AID = staged[176];
            let AIJ = staged[177];
            let APH = 0e0f64;
            let AQJ = 5e-3f64;
            let ASC = 1.66666666667e-2f64;
            let ASE = 2.38095238095e-2f64;
            let ASH = 2.5e-2f64;
            let ASO = 3.33333333333e-2f64;
            let ASR = 3.57142857143e-2f64;
            let ASZ = 7.14285714286e-2f64;
            let ATB = 5e-2f64;
            let ATD = 4.20875420875421e-2f64;
            let ATG = 5.5555555556e-3f64;
            let ATM = -5e-1f64;
            let ATQ = -5e-1f64;
            let ATS = 1.3888888889e-3f64;
            let ATV = 7.5e-2f64;
            let AVB = 1.01e0f64;
            let AVN = 3.96825396825397e-2f64;
            let AYW = 1e-200f64;
            let AZH = 6.5345483024e-2f64;
            let AZK = 8.5797362674e0f64;
            let AZL = 3.9478417604e1f64;
            let BAH = 2.3025850929941e0f64;
            let BIV = -5e-1f64;
            let BIZ = -5e-1f64;
            let BJB = 1.3888888889e-3f64;
            let BSX = -5e-1f64;
            let BTB = -5e-1f64;
            let BTD = 1.3888888889e-3f64;
            let CDR = -5e-1f64;
            let CDV = -5e-1f64;
            let CDX = 1.3888888889e-3f64;
            let CJQ = 1e-80f64;
            let CLQ = 9e-1f64;
            let CPW = 1e-6f64;
            let CQZ = -4e0f64;
            let CSR = staged[178];
            let CSS = staged[179];
            let CTI = staged[180];
            let CTP = 2e-1f64;
            let CTX = staged[181];
            let CTY = staged[182];
            let CUD = staged[183];
            let CUE = staged[184];
            let CUK = staged[452];
            let CUL = staged[453];
            let CUP = staged[187];
            let CUY = staged[188];
            let CVQ = 7e-3f64;
            let CVS = 1e-12f64;
            let CVU = staged[185];
            let CVW = staged[186];
            let CWH = 8.333333333335e-2f64;
            let CZQ = 1e2f64;
            let CZU = staged[189];
            let CZW = 9.4e-1f64;
            let DAU = staged[190];
            let DCH = 1e-14f64;
            let DCU = 1.48148148148e-1f64;
            let DCX = 1e-20f64;
            let DEB = 4.7e-1f64;
            let DEL = 3.6e1f64;
            let DFZ = staged[191];
            let DGB = 2.666666666667e0f64;
            let DGF = -6.25e-2f64;
            let DND = -5e-1f64;
            let DNH = -5e-1f64;
            let DNJ = 1.3888888889e-3f64;
            let ECC = -5e-1f64;
            let ECG = -5e-1f64;
            let ECI = 1.3888888889e-3f64;
            let EME = -5e-1f64;
            let EMI = -5e-1f64;
            let EMK = 1.3888888889e-3f64;
            let EWY = -5e-1f64;
            let EXC = -5e-1f64;
            let EXE = 1.3888888889e-3f64;
            let FKD = -4e0f64;
            let FLA = 1e-5f64;
            let FOO = staged[454];
            let FPL = staged[455];
            let FPX = staged[194];
            let FQH = staged[195];
            let FQJ = staged[196];
            let FQS = staged[197];
            let FRC = staged[456];
            let FRN = staged[457];
            let FSI = 6e-1f64;
            let FSM = 6e1f64;
            let FSO = -1.666666666667e-1f64;
            let FST = -1.666666666667e-1f64;
            let FTT = 1e-3f64;
            let FVY = -4.1666666666675e-2f64;
            let FWG = 1e-30f64;
            let FWI = -2e0f64;
            let FWO = -2e0f64;
            let GAI = staged[198];
            let GAP = staged[199];
            let GAR = staged[200];
            let GBC = 7.324648775608221e-1f64;
            let GBH = staged[201];
            let GBI = staged[203];
            let GBL = Lanes([0e0f64; 3]);
            let GBO = staged[458];
            let GHY = staged[204];
            let GIO = staged[205];
            let GOX = Lanes([0e0f64; 4]);
            let GPA = staged[459];
            let HCR = 1e-4f64;
            let HCT = staged[206];
            let HDL = staged[207];
            let HDM = staged[208];
            let HDP = staged[209];
            let HDQ = staged[210];
            let HDT = staged[211];
            let HDU = staged[212];
            let HEB = staged[213];
            let HED = staged[214];
            let HIZ = staged[215];
            let HJA = staged[216];
            let HWF = staged[217];
            let HYA = staged[218];
            let HYB = staged[219];
            let IBO = 4e-1f64;
            let IBP = 2.85714285714e-2f64;
            let IEA = staged[221];
            let IER = staged[222];
            let IGW = staged[224];
            let IHH = staged[460];
            let IHN = staged[225];
            let IJN = staged[227];
            let IJU = staged[231];
            let IJW = staged[232];
            let IJZ = staged[233];
            let IKB = staged[234];
            let IKD = staged[235];
            let IKG = staged[236];
            let IKL = staged[237];
            let IKT = staged[238];
            let ILC = staged[240];
            let ILI = staged[461];
            let ILJ = staged[241];
            let ILK = staged[242];
            let IOE = staged[243];
            let IPG = staged[244];
            let IPM = -1e0f64;
            let IPY = staged[245];
            let IRH = staged[462];
            let IRI = parameters[16];
            let IRO = 5e-1f64;
            let IUP = staged[259];
            let IVY = staged[249];
            let IWJ = staged[250];
            let IWS = staged[251];
            let JFI = staged[252];
            let JFR = -3.333333333333e-1f64;
            let JFV = -3.333333333333e-1f64;
            let JIF = staged[253];
            let JIJ = staged[254];
            let JTZ = -5e-1f64;
            let JUD = -5e-1f64;
            let JUF = 1.3888888889e-3f64;
            let KJA = -5e-1f64;
            let KJE = -5e-1f64;
            let KJG = 1.3888888889e-3f64;
            let KTC = -5e-1f64;
            let KTG = -5e-1f64;
            let KTI = 1.3888888889e-3f64;
            let LDW = -5e-1f64;
            let LEA = -5e-1f64;
            let LEC = 1.3888888889e-3f64;
            let LRB = -4e0f64;
            let LUA = staged[466];
            let LVQ = 8.333333333335e-2f64;
            let MDB = 4.7e-1f64;
            let MEX = staged[255];
            let MFC = -6.25e-2f64;
            let MMA = -5e-1f64;
            let MME = -5e-1f64;
            let MMG = 1.3888888889e-3f64;
            let NAZ = -5e-1f64;
            let NBD = -5e-1f64;
            let NBF = 1.3888888889e-3f64;
            let NLB = -5e-1f64;
            let NLF = -5e-1f64;
            let NLH = 1.3888888889e-3f64;
            let NVV = -5e-1f64;
            let NVZ = -5e-1f64;
            let NWB = 1.3888888889e-3f64;
            let OJA = -4e0f64;
            let ONJ = staged[467];
            let OPF = staged[258];
            let OQU = -1.666666666667e-1f64;
            let OQZ = -1.666666666667e-1f64;
            let OUB = -4.1666666666675e-2f64;
            let OUK = -2e0f64;
            let OUQ = -2e0f64;
            let OYE = staged[468];
            let OYM = 9e0f64;
            let OZA = staged[260];
            let PDC = staged[261];
            let PDF = staged[262];
            let PDI = staged[263];
            let PDK = staged[264];
            let PDT = staged[265];
            let PDY = staged[266];
            let PED = staged[267];
            let PEG = staged[268];
            let PEJ = staged[270];
            let PEM = staged[271];
            let PEP = staged[272];
            let PEU = staged[273];
            let PFF = staged[274];
            let PGC = 1e0f64;
            let PGD = staged[275];
            let PGG = Lanes([0e0f64; 2]);
            let PGJ = 1e0f64;
            let PGK = staged[276];
            let PGN = Lanes([0e0f64; 2]);
            let PGQ = 1e0f64;
            let PGR = staged[277];
            let PGU = Lanes([0e0f64; 2]);
            let PGX = 1e0f64;
            let PGY = staged[278];
            let PHB = Lanes([0e0f64; 2]);
            let PHE = staged[279];
            let PJB = ddt_scale();
            let PKJ = -5e-1f64;
            let PKS = -1.666666666667e-1f64;
            let PLS = staged[469];
            let PMG = staged[470];
            let PMJ = 1.6e0f64;
            let PMX = node_potentials[5];
            let PNA = 1e0f64;
            let POD = 1.92e1f64;
            let POK = staged[288];
            let POT = 3.8e1f64;
            let POU = staged[289];
            let POX = staged[291];
            let PPI = staged[292];
            let PPL = staged[293];
            let PQD = staged[294];
            let PQG = staged[295];
            let PRM = staged[301];
            let PSA = staged[304];
            let PSL = staged[305];
            let PZV = 0e0f64;
            let PZW = 0e0f64;
            let PZX = 0e0f64;
            let PZY = 0e0f64;
            let PZZ = 0e0f64;
            let QAA = 0e0f64;
            let QAB = 0e0f64;
            let QAC = 0e0f64;
            let QAD = 0e0f64;
            let C = ctx.simparam_or("gmin", B);
            if E != 0.0 {
            } else {
                if F != 0.0 {
                    if G != 0.0 {
                        loop {
                            if H == 0.0 {
                                break;
                            }
                        }
                    } else {
                        loop {
                            if I == 0.0 {
                                break;
                            }
                        }
                    }
                } else {
                }
            }
            let CH;
            let CI;
            let CJ;
            let CK;
            let CL;
            let CM;
            let CN;
            let CO;
            let CP;
            let CQ;
            let CR;
            let CS;
            let CT;
            let CU;
            let CV;
            let CW;
            let CX;
            let CY;
            let CZ;
            let DA;
            let DB;
            let DC;
            let DD;
            let DE;
            let DF;
            let DG;
            let DH;
            let DI;
            let DJ;
            let DK;
            let DL;
            let DM;
            let DN;
            let DO;
            let DP;
            let DQ;
            let DR;
            let DS;
            let DT;
            let DU;
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
            let EK;
            let EL;
            let EM;
            let EN;
            let EO;
            let EP;
            let EQ;
            let ER;
            let ES;
            let ET;
            let EU;
            let EV;
            let EW;
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
            if S != 0.0 {
                let U = staged[138] + T;
                let V = U * U;
                let X = W * U;
                let Y = X + X;
                let AA = U - Z;
                let AB = Z / U;
                let AD = ((W * AB) * AC) / U;
                let AF = U * AE;
                let AG = W * AE;
                let AH = D / AF;
                let AI = ((AG * AH) * AC) / AF;
                let GE = 6.36e2f64 + U;
                let GF = (GD * V) / GE;
                let GG = 1.17e0f64 - GF;
                let GH = (((Y * GD) - (W * GF)) / GE) * AC;
                let GJ = 2.35e2f64 + U;
                let GK = (GI * V) / GJ;
                let GM = (((7.44e-1f64 - GK) - GG) + staged[139]) * GL;
                let GN = (((((Y * GI) - (W * GK)) / GJ) * AC) - GH) * GL;
                let GP = GO * (GG + GM);
                let GQ = (GH + GN) * GO;
                let GR = GP * AH;
                let GS = (GQ * AH) + (AI * GP);
                let GT = staged[57] - (GO * GM);
                let GU = (GN * GO) * AC;
                let GW = (U * GV).sqrt();
                let GZ = (W * GV) * (GY / (GX * GW));
                let HB = HA * GW;
                let HC = HB * GW;
                let HE = (HC * GW) * HD;
                let HF = (((((GZ * HA) * GW) + (GZ * HB)) * GW) + (GZ * HC)) * HD;
                let HH = D + (HG * AB);
                let HI = AF * HH;
                let HJ = (AG * HH) + ((AD * HG) * AF);
                let HK = D / HI;
                let HL = ((HJ * HK) * AC) / HI;
                let HM = GP * HK;
                let HN = (GQ * HK) + (HL * GP);
                let HQ = (HO * HE) * HP;
                let HR = HQ * HK;
                let HS = (((HF * HO) * HP) * HK) + (HL * HQ);
                let HU = HT / HR;
                let HV = (((HS * HU) * AC) / HR) * (GY / HU);
                let HX = (HU.ln()) - HW;
                let HZ = HY * HK;
                let IA = HL * HY;
                let IC = IB * AA;
                let ID = W * IB;
                let IE = staged[68] + IC;
                let IF = staged[69] + IC;
                let IH = IG * HK;
                let II = HL * IG;
                let IJ = staged[70] + IC;
                let IK = staged[71] + IC;
                let IP;
                let IQ;
                if K != 0.0 {
                    let IM = AF * IL;
                    let IN = AG * IL;
                    IP = IM;
                    IQ = IN;
                } else {
                    IP = IO;
                    IQ = CG;
                }
                let IR;
                let IS;
                if L != 0.0 {
                    let PF;
                    let PG;
                    if M != 0.0 {
                        let OT = HI * OS;
                        let OV = (OU * (OT.ln())).exp();
                        let OX = OW * OV;
                        let OY = ((((HJ * OS) * (GY / OT)) * OU) * OV) * OW;
                        PF = OX;
                        PG = OY;
                    } else {
                        let OZ = HI * OS;
                        let PB = (PA * (OZ.ln())).exp();
                        let PD = PC * PB;
                        let PE = ((((HJ * OS) * (GY / OZ)) * PA) * PB) * PC;
                        PF = PD;
                        PG = PE;
                    }
                    IR = PF;
                    IS = PG;
                } else {
                    IR = B;
                    IS = CG;
                }
                let IU = W * IT;
                let IV = (IT * AA) + staged[85];
                let IW = (IV + parameters[34]) - IP;
                let IZ = GU * IY;
                let JA = (IY * ((staged[86] + GT) + IX)) + IW;
                let JB = IZ + (IU - IQ);
                let JD = (IY * ((staged[145] + GT) + JC)) + IV;
                let JE = IZ + IU;
                let JF = (IY * ((staged[89] + GT) + IX)) + IW;
                let JG = (IY * ((staged[146] + GT) + JC)) + IV;
                let JH = AB.ln();
                let JI = AD * (GY / AB);
                let JK = (JJ * JH).exp();
                let JM = JK * JL;
                let JN = ((JI * JJ) * JK) * JL;
                let JP = JO * JM;
                let JQ = JN * JO;
                let JS = JR * JM;
                let JT = JN * JR;
                let JV = (JU * JH).exp();
                let JX = JW * JV;
                let JZ = (JY * JH).exp();
                let KB = KA * JZ;
                let KC = ((JI * JY) * JZ) * KA;
                let KE = (KD * JH).exp();
                let KG = KF * KE;
                let KH = ((JI * KD) * KE) * KF;
                let KJ = (KI * JH).exp();
                let KL = KK * KJ;
                let KM = ((JI * KI) * KJ) * KK;
                let KO = (KN * JH).exp();
                let KQ = KP * KO;
                let KR = ((JI * KN) * KO) * KP;
                let KU = (KS * HI) / KT;
                let KV = KU * JX;
                let KW = (((HJ * KS) / KT) * JX) + ((((JI * JU) * JV) * JW) * KU);
                let KY = (KX * JH).exp();
                let LB = LA * (KZ * KY);
                let LC = LB * HI;
                let LD = (((((JI * KX) * KY) * KZ) * LA) * HI) + (HJ * LB);
                let LF = (LE * JH).exp();
                let LG = (JI * LE) * LF;
                let LI = LH * LF;
                let LJ = LI * JM;
                let LK = LJ * HI;
                let LL = ((((LG * LH) * JM) + (JN * LI)) * HI) + (HJ * LJ);
                let LN = LM * LF;
                let LO = LN * JM;
                let LP = LO * HI;
                let LQ = ((((LG * LM) * JM) + (JN * LN)) * HI) + (HJ * LO);
                let LS = LR * HK;
                let LT = HL * LR;
                let LV = (LU * JH).exp();
                let LW = (JI * LU) * LV;
                let LY = LX * LV;
                let LZ = LW * LX;
                let MB = MA * LV;
                let MC = LW * MA;
                let ME = MD * LV;
                let MF = LW * MD;
                let MH = MG * LV;
                let MI = LW * MG;
                let MK = MJ * LV;
                let ML = LW * MJ;
                let MN = (MM * JH).exp();
                let MO = (JI * MM) * MN;
                let MQ = MP * MN;
                let MR = MO * MP;
                let MT = MS * MN;
                let MU = MO * MS;
                let MW = MV * HI;
                let MX = HJ * MV;
                let MY = MV * AF;
                let MZ = AG * MV;
                let NB = D + (NA * HM);
                let NC = D / NB;
                let ND = (((HN * NA) * NC) * AC) / NB;
                let NF = W * NE;
                let NG = D + (NE * AA);
                let NH = NF * NG;
                let NJ = ((NG * NG) + NI).sqrt();
                let NM = (NK * (GO * (NG + NJ))) * NL;
                let NN = (((NF + ((NH + NH) * (GY / (GX * NJ)))) * GO) * NK) * NL;
                let NP = W * NO;
                let NQ = D + (NO * AA);
                let NR = NP * NQ;
                let NS = ((NQ * NQ) + NI).sqrt();
                let NU = (NT * (GO * (NQ + NS))) * NL;
                let NV = (((NP + ((NR + NR) * (GY / (GX * NS)))) * GO) * NT) * NL;
                let NX = (NW * JH).exp();
                let NZ = NY * NX;
                let OA = ((JI * NW) * NX) * NY;
                let OC = OB * HI;
                let OD = HJ * OB;
                let OE = HP * HI;
                let OF = staged[151] / OE;
                let OG = (((HJ * HP) * OF) * AC) / OE;
                let OH = staged[133] / HE;
                let OI = OH.ln();
                let OJ = (((HF * OH) * AC) / HE) * (GY / OH);
                let OL = OK * HI;
                let OM = HJ * OK;
                let OO = (ON * JH).exp();
                let OQ = OP * OO;
                let OR = ((JI * ON) * OO) * OP;
                CH = HK;
                CI = JA;
                CJ = JD;
                CK = IE;
                CL = IF;
                CM = LK;
                CN = GR;
                CO = HR;
                CP = IR;
                CQ = HX;
                CR = HZ;
                CS = IH;
                CT = JH;
                CU = KQ;
                CV = KG;
                CW = KL;
                CX = LC;
                CY = KB;
                CZ = KV;
                DA = JP;
                DB = JS;
                DC = LS;
                DD = HI;
                DE = AH;
                DF = MB;
                DG = MH;
                DH = ME;
                DI = MK;
                DJ = AF;
                DK = MY;
                DL = MQ;
                DM = MT;
                DN = LY;
                DO = MW;
                DP = GP;
                DQ = NC;
                DR = NM;
                DS = NU;
                DT = NZ;
                DU = OQ;
                DV = T;
                DW = JF;
                DX = JG;
                DY = IJ;
                DZ = IK;
                EA = LP;
                EB = OC;
                EC = OF;
                ED = OI;
                EE = OL;
                EF = HL;
                EG = JB;
                EH = JE;
                EI = ID;
                EJ = ID;
                EK = LL;
                EL = GS;
                EM = HS;
                EN = IS;
                EO = HV;
                EP = IA;
                EQ = II;
                ER = JI;
                ES = KR;
                ET = KH;
                EU = KM;
                EV = LD;
                EW = KC;
                EX = KW;
                EY = JQ;
                EZ = JT;
                FA = LT;
                FB = HJ;
                FC = AI;
                FD = MC;
                FE = MI;
                FF = MF;
                FG = ML;
                FH = AG;
                FI = MZ;
                FJ = MR;
                FK = MU;
                FL = LZ;
                FM = MX;
                FN = GQ;
                FO = ND;
                FP = NN;
                FQ = NV;
                FR = OA;
                FS = OR;
                FT = W;
                FU = JB;
                FV = JE;
                FW = ID;
                FX = ID;
                FY = LQ;
                FZ = OD;
                GA = OG;
                GB = OJ;
                GC = OM;
            } else {
                CH = AJ;
                CI = AK;
                CJ = AL;
                CK = AM;
                CL = AN;
                CM = AO;
                CN = AP;
                CO = AQ;
                CP = AR;
                CQ = AS;
                CR = AT;
                CS = AU;
                CT = AV;
                CU = AW;
                CV = AX;
                CW = AY;
                CX = AZ;
                CY = BA;
                CZ = BB;
                DA = BC;
                DB = BD;
                DC = BE;
                DD = BF;
                DE = BG;
                DF = BH;
                DG = BI;
                DH = BJ;
                DI = BK;
                DJ = BL;
                DK = BM;
                DL = BN;
                DM = BO;
                DN = BP;
                DO = BQ;
                DP = BR;
                DQ = BS;
                DR = BT;
                DS = BU;
                DT = BV;
                DU = BW;
                DV = B;
                DW = BX;
                DX = BY;
                DY = BZ;
                DZ = CA;
                EA = CB;
                EB = CC;
                EC = CD;
                ED = CE;
                EE = CF;
                EF = CG;
                EG = CG;
                EH = CG;
                EI = CG;
                EJ = CG;
                EK = CG;
                EL = CG;
                EM = CG;
                EN = CG;
                EO = CG;
                EP = CG;
                EQ = CG;
                ER = CG;
                ES = CG;
                ET = CG;
                EU = CG;
                EV = CG;
                EW = CG;
                EX = CG;
                EY = CG;
                EZ = CG;
                FA = CG;
                FB = CG;
                FC = CG;
                FD = CG;
                FE = CG;
                FF = CG;
                FG = CG;
                FH = CG;
                FI = CG;
                FJ = CG;
                FK = CG;
                FL = CG;
                FM = CG;
                FN = CG;
                FO = CG;
                FP = CG;
                FQ = CG;
                FR = CG;
                FS = CG;
                FT = CG;
                FU = CG;
                FV = CG;
                FW = CG;
                FX = CG;
                FY = CG;
                FZ = CG;
                GA = CG;
                GB = CG;
                GC = CG;
            }
            let QB;
            let QC;
            let QD;
            let QE;
            let QF;
            let QG;
            if M != 0.0 {
                let PJ = PH - PI;
                let PM = Lanes([0.0, PK]) - Lanes([PL, 0.0]);
                let PO = PN - PI;
                let PQ = Lanes([0.0, PP]) - Lanes([PL, 0.0]);
                let PS = PI - PR;
                let PU = Lanes([PL, 0.0]) - Lanes([0.0, PT]);
                QB = PO;
                QC = PJ;
                QD = PS;
                QE = PQ;
                QF = PM;
                QG = PU;
            } else {
                let PV = -(PH - PI);
                let PW = (Lanes([0.0, PK]) - Lanes([PL, 0.0])) * AC;
                let PX = -(PN - PI);
                let PY = (Lanes([0.0, PP]) - Lanes([PL, 0.0])) * AC;
                let PZ = -(PI - PR);
                let QA = (Lanes([PL, 0.0]) - Lanes([0.0, PT])) * AC;
                QB = PX;
                QC = PV;
                QD = PZ;
                QE = PY;
                QF = PW;
                QG = QA;
            }
            let QH = -QB;
            let QI = QE * AC;
            let QJ = QC + QH;
            let QK = Lanes([QF[0], 0.0, QF[1]]);
            let QL = QK + Lanes([QI[0], QI[1], 0.0]);
            let QM = QB + QD;
            let QN = Lanes([QG[0], 0.0, QG[1]]);
            let QO = Lanes([QE[0], QE[1], 0.0]) + QN;
            let QP = if QB < B { 1.0 } else { 0.0 };
            let QR;
            let QS;
            let QT;
            let QU;
            let QV;
            let QW;
            let QX;
            if QP != 0.0 {
                QR = QJ;
                QS = QM;
                QT = QH;
                QU = QQ;
                QV = QL;
                QW = QO;
                QX = QI;
            } else {
                QR = QC;
                QS = QD;
                QT = QB;
                QU = D;
                QV = QK;
                QW = QN;
                QX = QE;
            }
            let QY = QR + QS;
            let QZ = Lanes([QV[0], QV[1], 0.0, QV[2]]) + Lanes([QW[0], QW[1], QW[2], 0.0]);
            let RA = QT * CH;
            let RB = QX * CH;
            let RC = Lanes([0.0, RB[0], RB[1]]) + Lanes([(EF * QT), 0.0, 0.0]);
            let RD = QX * QT;
            let RE = ((QT * QT) + NI).sqrt();
            let RF = (RD + RD) * (GY / (GX * RE));
            let RG = RE - 1e-1f64;
            let RH = RG * CH;
            let RI = RF * CH;
            let RJ = Lanes([0.0, RI[0], RI[1]]) + Lanes([(EF * RG), 0.0, 0.0]);
            let RK = GO * (RA - RH);
            let RL = (RC - RJ) * GO;
            let RM = QR - CI;
            let RN = Lanes([0.0, QV[0], QV[1], QV[2]]);
            let RO = Lanes([RL[0], RL[1], RL[2], 0.0]);
            let RP = ((RM * CH) - RK) - CN;
            let RQ = Lanes([EL, 0.0, 0.0, 0.0]);
            let RR = ((((RN - Lanes([EG, 0.0, 0.0, 0.0])) * CH) + Lanes([(EF * RM), 0.0, 0.0, 0.0])) - RO) - RQ;
            let RS = -QS;
            let RT = QW * AC;
            let RU = RS - CJ;
            let RV = Lanes([0.0, RT[0], RT[1], RT[2]]);
            let RW = (RU * CH) - RK;
            let RX = Lanes([RL[0], RL[1], RL[2], 0.0]);
            let RY = (((RV - Lanes([EH, 0.0, 0.0, 0.0])) * CH) + Lanes([(EF * RU), 0.0, 0.0, 0.0])) - RX;
            let RZ = RW - CN;
            let SA = Lanes([EL, 0.0, 0.0, 0.0]);
            let SB = RY - SA;
            let SD;
            let SE;
            if J != 0.0 {
                let SL = CO / HT;
                let SM = EM / HT;
                let SN = Lanes([RR[0], RR[1], RR[2], 0.0, RR[3]]);
                let SP = staged[154] / SL;
                let SR = (SP.ln()) + SQ;
                let SS = staged[155] / SL;
                let ST = (((SM * SS) * AC) / SL) * (GY / SS);
                let SU = (SS.ln()) + SQ;
                let SV = Lanes([((((SM * SP) * AC) / SL) * (GY / SP)), 0.0, 0.0, 0.0, 0.0]);
                let SW = (SR - (RP - ((SI * (RP - RZ)) * SO))) / SQ;
                let SX = (SV - (SN - (((SN - Lanes([SB[0], SB[1], SB[2], SB[3], 0.0])) * SI) * SO))) / SQ;
                let SZ = if SW < SY { 1.0 } else { 0.0 };
                let TE;
                let TF;
                if SZ != 0.0 {
                    let TA = SW.exp();
                    let TB = D + TA;
                    let TC = TB.ln();
                    let TD = (SX * TA) * (GY / TB);
                    TE = TC;
                    TF = TD;
                } else {
                    TE = SW;
                    TF = SX;
                }
                let TH = SB * TG;
                let TJ = Lanes([ST, 0.0, 0.0, 0.0, 0.0]);
                let TK = (SU - (((TG * RZ) + (SR - (SQ * TE))) * TI)) / SQ;
                let TL = (TJ - ((Lanes([TH[0], TH[1], TH[2], TH[3], 0.0]) + (SV - (TF * SQ))) * TI)) / SQ;
                let TM = if TK < SY { 1.0 } else { 0.0 };
                let TR;
                let TS;
                if TM != 0.0 {
                    let TN = TK.exp();
                    let TO = D + TN;
                    let TP = TO.ln();
                    let TQ = (TL * TN) * (GY / TO);
                    TR = TP;
                    TS = TQ;
                } else {
                    TR = TK;
                    TS = TL;
                }
                let TU = TT * RZ;
                let TV = SB * TT;
                let TW = (TT * (SU - (SQ * TR))) - TU;
                let TX = Lanes([TV[0], TV[1], TV[2], TV[3], 0.0]);
                let TY = ((TJ - (TS * SQ)) * TT) - TX;
                let UA = if (TW.abs()) <= TZ { 1.0 } else { 0.0 };
                let UN;
                let UO;
                if UA != 0.0 {
                    let UE = ((UB * UB) * UC) / UD;
                    let UF = TW * UB;
                    let UH = D - UG;
                    let UJ = D + (((TW * UH) * UI) * UE);
                    let UK = UF * UJ;
                    let UL = ((TY * UB) * UJ) + ((((TY * UH) * UI) * UE) * UF);
                    UN = UK;
                    UO = UL;
                } else {
                    let UM = if TW < (-TZ) { 1.0 } else { 0.0 };
                    let WT;
                    let WU;
                    if UM != 0.0 {
                        let UR = -TW;
                        let US = TY * AC;
                        let UU = UT * (UR * UB);
                        let UV = (US * UB) * UT;
                        let UY = UU - UX;
                        let UZ = UV * UY;
                        let VB = ((UY * UY) + VA).sqrt();
                        let VC = GO * ((UU + UW) - VB);
                        let VD = (UV - ((UZ + UZ) * (GY / (GX * VB)))) * GO;
                        let VE = UR - VC;
                        let VF = US - VD;
                        let VG = VF * VE;
                        let VI = (VE * VE) + (VH * (VC + D));
                        let VJ = (VG + VG) + (VD * VH);
                        let VK = VF * LA;
                        let VL = (LA * VE) - VH;
                        let VN = VI * VM;
                        let VO = (-VC) + (VN.ln());
                        let VP = (VD * AC) + ((VJ * VM) * (GY / VN));
                        let VQ = VI + VL;
                        let VR = VJ + VK;
                        let VS = VR * VQ;
                        let VT = GO * VL;
                        let VU = (VT * VL) - VI;
                        let VV = (VQ * VQ) + (VO * VU);
                        let VW = (VS + VS) + ((VP * VU) + (((((VK * GO) * VL) + (VK * VT)) - VJ) * VO));
                        let VX = VI * VQ;
                        let VY = VQ / VV;
                        let VZ = VY * VO;
                        let WA = VZ * VO;
                        let WB = WA * VL;
                        let WC = VK * VL;
                        let WE = ((VL * VL) * WD) - VI;
                        let WF = VV + (WB * WE);
                        let WG = (VX * VO) / WF;
                        let WH = VC + WG;
                        let WI = VD + ((((((VJ * VQ) + (VR * VI)) * VO) + (VP * VX)) - ((VW + ((((((((((VR - (VW * VY)) / VV) * VO) + (VP * VY)) * VO) + (VP * VZ)) * VL) + (VK * WA)) * WE) + ((((WC + WC) * WD) - VJ) * WB))) * WG)) / WF);
                        let WJ = if WH < SY { 1.0 } else { 0.0 };
                        let XE;
                        let XF;
                        if WJ != 0.0 {
                            let WV = WH.exp();
                            let WW = WI * WV;
                            XE = WV;
                            XF = WW;
                        } else {
                            let WX = WH - SY;
                            let WY = GO * WX;
                            let WZ = D + (WX * WD);
                            let XA = D + (WY * WZ);
                            let XC = XB * (D + (WX * XA));
                            let XD = ((WI * XA) + ((((WI * GO) * WZ) + ((WI * WD) * WY)) * WX)) * XB;
                            XE = XC;
                            XF = XD;
                        }
                        let XG = D / XE;
                        let XH = WH * WH;
                        let XI = WI * WH;
                        let XJ = LA + XH;
                        let XK = D / XJ;
                        let XL = (XI + XI) * XK;
                        let XM = (XL * AC) / XJ;
                        let XN = XH * XK;
                        let XO = XL + (XM * XH);
                        let XP = WH * XK;
                        let XT = (XR * XK) - (XS * XN);
                        let XU = XT * XK;
                        let XV = UR - WH;
                        let XW = US - WI;
                        let XX = UG * XG;
                        let XY = (((XF * XG) * AC) / XE) * UG;
                        let XZ = (LA * XV) + (VH * (((XE - D) - XX) + (UG * (D - (XQ * (XP * XK))))));
                        let YA = (XW * LA) + (((XF - XY) + (((((((WI * XK) + (XM * WH)) * XK) + (XM * XP)) * XQ) * AC) * UG)) * VH);
                        let YB = XW * XV;
                        let YC = (XV * XV) - (VH * ((((XE - WH) - D) + XX) + (UG * ((WH - D) - XN))));
                        let YD = (YB + YB) - ((((XF - WI) + XY) + ((WI - XO) * UG)) * VH);
                        let YE = LA - (VH * ((XE + XX) - (UG * (XU * XK))));
                        let YF = YA * XZ;
                        let YG = ((XZ * XZ) - (LA * (YC * YE))).sqrt();
                        let YH = XZ + YG;
                        let YI = YC / YH;
                        let YJ = (-WH) - (LA * YI);
                        let YK = (WI * AC) - (((YD - ((YA + (((YF + YF) - (((YD * YE) + (((((XF + XY) - (((((((XM * XR) - (XO * XS)) * XK) + (XM * XT)) * XK) + (XM * XU)) * UG)) * VH) * AC) * YC)) * LA)) * (GY / (GX * YG)))) * YI)) / YH) * LA);
                        WT = YJ;
                        WU = YK;
                    } else {
                        let WL = D / (UT + (UI * WK));
                        let WN = (((UT * WM) * WL) - D) * WL;
                        let WO = TW * UB;
                        let WP = D + (WN * TW);
                        let WQ = -(WO * WP);
                        let WR = (((TY * UB) * WP) + ((TY * WN) * WO)) * AC;
                        let WS = if WQ > -8e1f64 { 1.0 } else { 0.0 };
                        let YW;
                        let YX;
                        if WS != 0.0 {
                            let YL = WQ.exp();
                            let YM = WR * YL;
                            YW = YL;
                            YX = YM;
                        } else {
                            let YN = WR * AC;
                            let YO = (-WQ) - SY;
                            let YP = GO * YO;
                            let YQ = D + (YO * WD);
                            let YR = D + (YP * YQ);
                            let YS = D + (YO * YR);
                            let YU = YT / YS;
                            let YV = ((((YN * YR) + ((((YN * GO) * YQ) + ((YN * WD) * YP)) * YO)) * YU) * AC) / YS;
                            YW = YU;
                            YX = YV;
                        }
                        let YZ = ((TW + (VH * YY)) - (D - YW)).sqrt();
                        let ZA = (TW + (VH * GO)) - (UI * YZ);
                        let ZB = TY - (((TY - (YX * AC)) * (GY / (GX * YZ))) * UI);
                        let ZE = ZC + ZD;
                        let ZF = ZA - ZE;
                        let ZG = ZB * ZF;
                        let ZI = ((ZF * ZF) + ZH).sqrt();
                        let ZJ = (ZB - ((ZG + ZG) * (GY / (GX * ZI)))) * GO;
                        let ZK = (GO * ((ZA + ZE) - ZI)) - (GO * (ZE - (((ZE * ZE) + ZH).sqrt())));
                        let ZL = TW - ZK;
                        let ZM = TY - ZJ;
                        let ZN = ZJ * AC;
                        let ZO = (-ZK).exp();
                        let ZP = ZN * ZO;
                        let ZQ = ZK * ZK;
                        let ZR = ZJ * ZK;
                        let ZS = LA + ZQ;
                        let ZT = D / ZS;
                        let ZU = (ZR + ZR) * ZT;
                        let ZV = (ZU * AC) / ZS;
                        let ZW = ZQ * ZT;
                        let ZX = ZU + (ZV * ZQ);
                        let ZY = ZK * ZT;
                        let ZZ = (XR * ZT) - (XS * ZW);
                        let AAA = ZZ * ZT;
                        let AAB = ZM * ZL;
                        let AAC = (ZL * ZL) - (VH * (((ZO + ZK) - D) - (UG * ((ZK + D) + ZW))));
                        let AAE = if AAD >= AAC { AAD } else { AAC };
                        let AAF = ((AAB + AAB) - (((ZP + ZJ) - ((ZJ + ZX) * UG)) * VH)) * (GY - (if AAD >= AAC { 1.0 } else { 0.0 }));
                        let AAG = D - (GO * (VH * (ZO - (UG * (AAA * ZT)))));
                        let AAH = (LA * ZL) + (VH * ((D - ZO) - (UG * (D + (XQ * (ZY * ZT))))));
                        let AAI = (ZM * LA) + (((ZP * AC) - ((((((ZJ * ZT) + (ZV * ZK)) * ZT) + (ZV * ZY)) * XQ) * UG)) * VH);
                        let AAJ = AAE / VH;
                        let AAK = (ZC - ZK) + (AAJ.ln());
                        let AAL = ZN + ((AAF / VH) * (GY / AAJ));
                        let AAM = AAE + AAH;
                        let AAN = AAF + AAI;
                        let AAO = AAN * AAM;
                        let AAP = GO * AAH;
                        let AAQ = AAE * AAG;
                        let AAR = (AAF * AAG) + (((((ZP - (((((((ZV * XR) - (ZX * XS)) * ZT) + (ZV * ZZ)) * ZT) + (ZV * AAA)) * UG)) * VH) * GO) * AC) * AAE);
                        let AAS = (AAP * AAH) - AAQ;
                        let AAT = (AAM * AAM) + (AAK * AAS);
                        let AAU = (AAO + AAO) + ((AAL * AAS) + (((((AAI * GO) * AAH) + (AAI * AAP)) - AAR) * AAK));
                        let AAV = AAE * AAM;
                        let AAW = AAM / AAT;
                        let AAX = AAW * AAK;
                        let AAY = AAX * AAK;
                        let AAZ = AAY * AAH;
                        let ABA = AAI * AAH;
                        let ABB = ((AAH * AAH) * WD) - AAQ;
                        let ABC = AAT + (AAZ * ABB);
                        let ABD = (AAV * AAK) / ABC;
                        let ABE = ZK + ABD;
                        let ABF = ZJ + ((((((AAF * AAM) + (AAN * AAE)) * AAK) + (AAL * AAV)) - ((AAU + ((((((((((AAN - (AAU * AAW)) / AAT) * AAK) + (AAL * AAW)) * AAK) + (AAL * AAX)) * AAH) + (AAI * AAY)) * ABB) + ((((ABA + ABA) * WD) - AAR) * AAZ))) * ABD)) / ABC);
                        let ABG = if ABE < SY { 1.0 } else { 0.0 };
                        let ABO;
                        let ABP;
                        let ABQ;
                        let ABR;
                        if ABG != 0.0 {
                            let ABH = ABE.exp();
                            let ABI = ABF * ABH;
                            let ABJ = D / ABH;
                            let ABK = ((ABI * ABJ) * AC) / ABH;
                            let ABL = UG * ABH;
                            let ABM = ABI * UG;
                            ABO = ABJ;
                            ABP = ABL;
                            ABQ = ABK;
                            ABR = ABM;
                        } else {
                            let ABN = if ABE > (ZC - SY) { 1.0 } else { 0.0 };
                            let ADK;
                            let ADL;
                            let ADM;
                            let ADN;
                            if ABN != 0.0 {
                                let ACR = (ABE - ZC).exp();
                                let ACS = ABF * ACR;
                                let ACT = UG / ACR;
                                let ACU = ((ACS * ACT) * AC) / ACR;
                                ADK = ACT;
                                ADL = ACR;
                                ADM = ACU;
                                ADN = ACS;
                            } else {
                                let ACV = ABF * AC;
                                let ACW = (ZC - ABE) - SY;
                                let ACX = GO * ACW;
                                let ACY = D + (ACW * WD);
                                let ACZ = D + (ACX * ACY);
                                let ADA = D + (ACW * ACZ);
                                let ADB = YT / ADA;
                                let ADC = ((((ACV * ACZ) + ((((ACV * GO) * ACY) + ((ACV * WD) * ACX)) * ACW)) * ADB) * AC) / ADA;
                                let ADD = ABE - SY;
                                let ADE = GO * ADD;
                                let ADF = D + (ADD * WD);
                                let ADG = D + (ADE * ADF);
                                let ADH = D + (ADD * ADG);
                                let ADI = YT / ADH;
                                let ADJ = ((((ABF * ADG) + ((((ABF * GO) * ADF) + ((ABF * WD) * ADE)) * ADD)) * ADI) * AC) / ADH;
                                ADK = ADI;
                                ADL = ADB;
                                ADM = ADJ;
                                ADN = ADC;
                            }
                            ABO = ADK;
                            ABP = ADL;
                            ABQ = ADM;
                            ABR = ADN;
                        }
                        let ABS = ABE * ABE;
                        let ABT = ABF * ABE;
                        let ABU = LA + ABS;
                        let ABV = D / ABU;
                        let ABW = (ABT + ABT) * ABV;
                        let ABX = (ABW * AC) / ABU;
                        let ABY = ABS * ABV;
                        let ABZ = ABW + (ABX * ABS);
                        let ACA = ABE * ABV;
                        let ACB = (XR * ABV) - (XS * ABY);
                        let ACC = ACB * ABV;
                        let ACD = TW - ABE;
                        let ACE = TY - ABF;
                        let ACF = (LA * ACD) + (VH * (((D - ABO) + ABP) - (UG * (D + (XQ * (ACA * ABV))))));
                        let ACG = (ACE * LA) + ((((ABQ * AC) + ABR) - ((((((ABF * ABV) + (ABX * ABE)) * ABV) + (ABX * ACA)) * XQ) * UG)) * VH);
                        let ACH = ACE * ACD;
                        let ACI = (ACD * ACD) - (VH * ((((ABO + ABE) - D) + ABP) - (UG * ((ABE + D) + ABY))));
                        let ACJ = (ACH + ACH) - ((((ABQ + ABF) + ABR) - ((ABF + ABZ) * UG)) * VH);
                        let ACK = LA - (VH * ((ABO + ABP) - (UG * (ACC * ABV))));
                        let ACL = ACG * ACF;
                        let ACM = ((ACF * ACF) - (LA * (ACI * ACK))).sqrt();
                        let ACN = ACF + ACM;
                        let ACO = ACI / ACN;
                        let ACP = ABE + (LA * ACO);
                        let ACQ = ABF + (((ACJ - ((ACG + (((ACL + ACL) - (((ACJ * ACK) + (((((ABQ + ABR) - (((((((ABX * XR) - (ABZ * XS)) * ABV) + (ABX * ACB)) * ABV) + (ABX * ACC)) * UG)) * VH) * AC) * ACI)) * LA)) * (GY / (GX * ACM)))) * ACO)) / ACN) * LA);
                        WT = ACP;
                        WU = ACQ;
                    }
                    UN = WT;
                    UO = WU;
                }
                let UP = TT * (UN + TU);
                let UQ = (UO + TX) * TT;
                SD = UP;
                SE = UQ;
            } else {
                let SC = Lanes([SB[0], SB[1], SB[2], SB[3], 0.0]);
                SD = RZ;
                SE = SC;
            }
            let SF = RP - SD;
            let SG = Lanes([RR[0], RR[1], RR[2], 0.0, RR[3]]);
            let SH = SG - SE;
            let SJ = SI * SF;
            let SK = SH * SI;
            let AFE;
            let AFF;
            let AFG;
            let AFH;
            let AFI;
            let AFJ;
            let AFK;
            let AFL;
            let AFM;
            let AFN;
            let AFO;
            let AFP;
            if L != 0.0 {
                let ADP = SJ - ADO;
                let ADQ = SK * ADP;
                let ADS = ((ADP * ADP) + ADR).sqrt();
                let ADT = GO * ((SJ + ADO) + ADS);
                let ADU = -SJ;
                let ADV = SK * AC;
                let ADW = ADU - ADO;
                let ADX = ADV * ADW;
                let ADY = ((ADW * ADW) + ADR).sqrt();
                let ADZ = GO * ((ADU + ADO) + ADY);
                let AEB = (AEA * (ADT.ln())).exp();
                let AEC = CP * AEB;
                let AED = Lanes([(EN * AEB), 0.0, 0.0, 0.0, 0.0]) + ((((((SK + ((ADQ + ADQ) * (GY / (GX * ADS)))) * GO) * (GY / ADT)) * AEA) * AEB) * CP);
                let AEF = (AEE * (ADZ.ln())).exp();
                let AEG = CP * AEF;
                let AEH = Lanes([(EN * AEF), 0.0, 0.0, 0.0, 0.0]) + ((((((ADV + ((ADX + ADX) * (GY / (GX * ADY)))) * GO) * (GY / ADZ)) * AEE) * AEF) * CP);
                let AEI = (D - AEC) - AEG;
                let AEJ = (AED * AC) - AEH;
                let AEL = AEK / AEI;
                let AEM = ((AEJ * AEL) * AC) / AEI;
                let AEO = D + (AEN * AEC);
                let AEP = D + (TG * AEG);
                let AEQ = (AEN * AEI) / AEO;
                let AER = ((AEJ * AEN) - ((AED * AEN) * AEQ)) / AEO;
                let AES = (TG * AEI) / AEP;
                let AET = ((AEJ * TG) - ((AEH * TG) * AES)) / AEP;
                let AEU = D / AEQ;
                let AEV = D / AES;
                let AEW = (D + AEU) + AEV;
                let AEX = D / AEW;
                let AEY = ((((((AER * AEU) * AC) / AEQ) + (((AET * AEV) * AC) / AES)) * AEX) * AC) / AEW;
                let AEZ = (AER * AEC) + (AED * AEQ);
                let AFA = D + (AEQ * AEC);
                let AFB = (AET * AEG) + (AEH * AES);
                let AFC = D + (AES * AEG);
                AFE = AEX;
                AFF = AEQ;
                AFG = AES;
                AFH = AEL;
                AFI = AFA;
                AFJ = AFC;
                AFK = AEY;
                AFL = AER;
                AFM = AET;
                AFN = AEM;
                AFO = AEZ;
                AFP = AFB;
            } else {
                AFE = SI;
                AFF = AEN;
                AFG = TG;
                AFH = AEK;
                AFI = D;
                AFJ = D;
                AFK = AFD;
                AFL = AFD;
                AFM = AFD;
                AFN = AFD;
                AFO = AFD;
                AFP = AFD;
            }
            let AFQ = AFE * SF;
            let AFR = (AFK * SF) + (SH * AFE);
            let AFS = if AFQ > B { 1.0 } else { 0.0 };
            let AFX;
            let AFY;
            if AFS != 0.0 {
                let AFT = -AFQ;
                let AFU = AFR * AC;
                let AFV = if AFT < SY { 1.0 } else { 0.0 };
                let AJV;
                let AJW;
                if AFV != 0.0 {
                    let AJR = AFT.exp();
                    let AJS = D + AJR;
                    let AJT = AJS.ln();
                    let AJU = (AFU * AJR) * (GY / AJS);
                    AJV = AJT;
                    AJW = AJU;
                } else {
                    AJV = AFT;
                    AJW = AFU;
                }
                let AJX = AFQ / AFF;
                let AJY = (SG - ((AFR - (AFL * AJX)) / AFF)) + AJW;
                let AJZ = ((RP - AJX) + AJV) - HW;
                AFX = AJZ;
                AFY = AJY;
            } else {
                let AFW = if AFQ < SY { 1.0 } else { 0.0 };
                let AKE;
                let AKF;
                if AFW != 0.0 {
                    let AKA = AFQ.exp();
                    let AKB = D + AKA;
                    let AKC = AKB.ln();
                    let AKD = (AFR * AKA) * (GY / AKB);
                    AKE = AKC;
                    AKF = AKD;
                } else {
                    AKE = AFQ;
                    AKF = AFR;
                }
                let AKG = AFQ / AFG;
                let AKH = (SE + ((AFR - (AFM * AKG)) / AFG)) + AKF;
                let AKI = ((SD + AKG) + AKE) - HW;
                AFX = AKI;
                AFY = AKH;
            }
            let AFZ = Lanes([EO, 0.0, 0.0, 0.0, 0.0]);
            let AGA = AFX - CQ;
            let AGB = (AFY - AFZ) * AGA;
            let AGC = ((AGA * AGA) + XQ).sqrt();
            let AGD = GO * ((AFX + CQ) - AGC);
            let AGE = ((AFY + AFZ) - ((AGB + AGB) * (GY / (GX * AGC)))) * GO;
            let AGF = (LA * (CQ - AGD)) / CR;
            let AGG = (D + AGF).sqrt();
            let AGH = ((((AFZ - AGE) * LA) - Lanes([(EP * AGF), 0.0, 0.0, 0.0, 0.0])) / CR) * (GY / (GX * AGG));
            let AGI = AGG - D;
            let AGJ = AGD + (CR * AGI);
            let AGK = AGE + (Lanes([(EP * AGI), 0.0, 0.0, 0.0, 0.0]) + (AGH * CR));
            let AGM = RY * AGL;
            let AGN = D + (AGL * RW);
            let AGO = AGN - GO;
            let AGP = AGM * AGO;
            let AGQ = ((AGO * AGO) + NI).sqrt();
            let AGR = GO * ((AGN + GO) + AGQ);
            let AGS = (AGM + ((AGP + AGP) * (GY / (GX * AGQ)))) * GO;
            let AGU = D + (AGT * AGR);
            let AGV = D / AGU;
            let AGW = (((AGS * AGT) * AGV) * AC) / AGU;
            let AGY = D + (AGX * AGR);
            let AGZ = D / AGY;
            let AHA = (((AGS * AGX) * AGZ) * AC) / AGY;
            let AHB = LA * CS;
            let AHC = RH / CS;
            let AHD = (D + AHC).sqrt();
            let AHE = AHD - D;
            let AHF = AHB * AHE;
            let AHG = Lanes([((EQ * LA) * AHE), 0.0, 0.0]) + ((((RJ - Lanes([(EQ * AHC), 0.0, 0.0])) / CS) * (GY / (GX * AHD))) * AHB);
            let AHI = D + (AHH * AGI);
            let AHJ = AHF * AHI;
            let AHK = AHG * AHI;
            let AHM = D + (AHL * RW);
            let AHN = AHJ * AHM;
            let AHO = (RY * AHL) * AHJ;
            let AHP = ((Lanes([AHK[0], AHK[1], AHK[2], 0.0, 0.0]) + ((AGH * AHH) * AHF)) * AHM) + Lanes([AHO[0], AHO[1], AHO[2], AHO[3], 0.0]);
            let AHQ = CK * AHN;
            let AHR = (RP - AGJ) + AHQ;
            let AHS = AGW * AHR;
            let AHT = ((AHR * AGV) + AGJ) + RK;
            let AHU = Lanes([RL[0], RL[1], RL[2], 0.0, 0.0]);
            let AHV = (((((SG - AGK) + (Lanes([(EI * AHN), 0.0, 0.0, 0.0, 0.0]) + (AHP * CK))) * AGV) + Lanes([AHS[0], AHS[1], AHS[2], AHS[3], 0.0])) + AGK) + AHU;
            let AHW = (SD - AGJ) + (CL * AHN);
            let AHX = AHA * AHW;
            let AHY = ((AHW * AGZ) + AGJ) + RK;
            let AHZ = (((((SE - AGK) + (Lanes([(EJ * AHN), 0.0, 0.0, 0.0, 0.0]) + (AHP * CL))) * AGZ) + Lanes([AHX[0], AHX[1], AHX[2], AHX[3], 0.0])) + AGK) + AHU;
            let AIB = AHY + (AIA * (AHT - AHY));
            let AIC = AHZ + ((AHV - AHZ) * AIA);
            let AIE = AIB - AID;
            let AIF = AIC * AIE;
            let AIG = ((AIE * AIE) + NI).sqrt();
            let AIH = GO * ((AIB + AID) - AIG);
            let AII = (AIC - ((AIF + AIF) * (GY / (GX * AIG)))) * GO;
            let AIK = AHT + (AIJ * (AHY - AHT));
            let AIL = AHV + ((AHZ - AHV) * AIJ);
            let AIM = AIK - AID;
            let AIN = AIL * AIM;
            let AIO = ((AIM * AIM) + NI).sqrt();
            let AIP = GO * ((AIK + AID) - AIO);
            let AIQ = (AIL - ((AIN + AIN) * (GY / (GX * AIO)))) * GO;
            let AIR = AFF / AGV;
            let AIS = AGW * AIR;
            let AIT = (AFL - Lanes([AIS[0], AIS[1], AIS[2], AIS[3], 0.0])) / AGV;
            let AIU = AFG / AGZ;
            let AIV = AHA * AIU;
            let AIW = (AFM - Lanes([AIV[0], AIV[1], AIV[2], AIV[3], 0.0])) / AGZ;
            let AIX = D / AIR;
            let AIY = ((AIT * AIX) * AC) / AIR;
            let AIZ = D / AIU;
            let AJA = ((AIW * AIZ) * AC) / AIU;
            let AJB = (D + AIX) + AIZ;
            let AJC = D / AJB;
            let AJD = (((AIY + AJA) * AJC) * AC) / AJB;
            let AJE = AFH * AFH;
            let AJF = AFN * AFH;
            let AJG = AJF + AJF;
            let AJH = CO / AJE;
            let AJI = Lanes([EM, 0.0, 0.0, 0.0, 0.0]);
            let AJJ = (AJI - (AJG * AJH)) / AJE;
            let AJK = D + AIR;
            let AJL = D + AIU;
            let AJM = AJK / AJL;
            let AJN = (AIT - (AIW * AJM)) / AJL;
            let AJO = AJM.ln();
            let AJP = AJN * (GY / AJM);
            let AJQ = if AJO > KS { 1.0 } else { 0.0 };
            let AKQ;
            let AKR;
            if AJQ != 0.0 {
                let AKJ = LA * AJO;
                let AKK = AJM + D;
                let AKL = AJM - D;
                let AKM = (AKJ * AKK) / AKL;
                let AKN = ((((AJP * LA) * AKK) + (AJN * AKJ)) - (AJN * AKM)) / AKL;
                AKQ = AKM;
                AKR = AKN;
            } else {
                let AKO = LA * (LA + AJO);
                let AKP = AJP * LA;
                AKQ = AKO;
                AKR = AKP;
            }
            let AKS = AIH - AIP;
            let AKT = AJC * AKS;
            let AKU = (AJD * AKS) + ((AII - AIQ) * AJC);
            let AKV = AKT * AKT;
            let AKW = AKU * AKT;
            let AKX = AKW + AKW;
            let AKY = AKT * AIX;
            let AKZ = (AKU * AIX) + (AIY * AKT);
            let ALA = AIH - AKY;
            let ALB = AII - AKZ;
            let ALC = AKT * AIZ;
            let ALD = (AKU * AIZ) + (AJA * AKT);
            let ALE = AIP + ALC;
            let ALF = AIQ + ALD;
            let ALG = D / AJK;
            let ALH = AIT * ALG;
            let ALI = (ALH * AC) / AJK;
            let ALJ = D / AJL;
            let ALK = AIW * ALJ;
            let ALL = (ALK * AC) / AJL;
            let ALM = AIR + (AIU * ALJ);
            let ALN = AIT + (ALK + (ALL * AIU));
            let ALO = AKR * ALM;
            let ALP = (ALM * AKQ) / AJH;
            let ALQ = AJJ * ALP;
            let ALR = ALP.ln();
            let ALS = GY / ALP;
            let ALT = ((((ALN * AKQ) + ALO) - ALQ) / AJH) * ALS;
            let ALU = ALR + ZD;
            let ALV = AIU + (AIR * ALG);
            let ALW = AIW + (ALH + (ALI * AIR));
            let ALX = AKR * ALV;
            let ALY = (ALV * AKQ) / AJH;
            let ALZ = AJJ * ALY;
            let AMA = ALY.ln();
            let AMB = GY / ALY;
            let AMC = ((((ALW * AKQ) + ALX) - ALZ) / AJH) * AMB;
            let AMD = AMA + ZD;
            let AME = (ALU - ALA) * WD;
            let AMF = (ALT - ALB) * WD;
            let AMG = if AME < SY { 1.0 } else { 0.0 };
            let AML;
            let AMM;
            if AMG != 0.0 {
                let AMH = AME.exp();
                let AMI = D + AMH;
                let AMJ = AMI.ln();
                let AMK = (AMF * AMH) * (GY / AMI);
                AML = AMJ;
                AMM = AMK;
            } else {
                AML = AME;
                AMM = AMF;
            }
            let AMN = ALU - (ZD * AML);
            let AMO = ALT - (AMM * ZD);
            let AMP = (AMD - ALE) * WD;
            let AMQ = (AMC - ALF) * WD;
            let AMR = if AMP < SY { 1.0 } else { 0.0 };
            let AMW;
            let AMX;
            if AMR != 0.0 {
                let AMS = AMP.exp();
                let AMT = D + AMS;
                let AMU = AMT.ln();
                let AMV = (AMQ * AMS) * (GY / AMT);
                AMW = AMU;
                AMX = AMV;
            } else {
                AMW = AMP;
                AMX = AMQ;
            }
            let AMY = AIR * AIH;
            let AMZ = (AIT * AIH) + (AII * AIR);
            let ANA = AMY + (AMD - (ZD * AMW));
            let ANB = AIU * AIP;
            let ANC = (AIW * AIP) + (AIQ * AIU);
            let AND = ANB + AMN;
            let ANE = AND * ALJ;
            let ANF = ((ANC + AMO) * ALJ) + (ALL * AND);
            let ANG = (ALU - (ANA * ALG)) * WD;
            let ANH = (ALT - (((AMZ + (AMC - (AMX * ZD))) * ALG) + (ALI * ANA))) * WD;
            let ANI = if ANG < SY { 1.0 } else { 0.0 };
            let ANN;
            let ANO;
            if ANI != 0.0 {
                let ANJ = ANG.exp();
                let ANK = D + ANJ;
                let ANL = ANK.ln();
                let ANM = (ANH * ANJ) * (GY / ANK);
                ANN = ANL;
                ANO = ANM;
            } else {
                ANN = ANG;
                ANO = ANH;
            }
            let ANP = ALU - (ZD * ANN);
            let ANQ = ALT - (ANO * ZD);
            let ANR = (AMD - ANE) * WD;
            let ANS = (AMC - ANF) * WD;
            let ANT = if ANR < SY { 1.0 } else { 0.0 };
            let ANY;
            let ANZ;
            if ANT != 0.0 {
                let ANU = ANR.exp();
                let ANV = D + ANU;
                let ANW = ANV.ln();
                let ANX = (ANS * ANU) * (GY / ANV);
                ANY = ANW;
                ANZ = ANX;
            } else {
                ANY = ANR;
                ANZ = ANS;
            }
            let AOA = AIH - ANP;
            let AOB = AII - ANQ;
            let AOC = AIP - (AMD - (ZD * ANY));
            let AOD = AIQ - (AMC - (ANZ * ZD));
            let AOE = AIR * AOA;
            let AOF = (AIT * AOA) + (AOB * AIR);
            let AOG = AIH - AOA;
            let AOH = AII - AOB;
            let AOI = if AOG < SY { 1.0 } else { 0.0 };
            let AOR;
            let AOS;
            if AOI != 0.0 {
                let AOJ = AOG.exp();
                let AOK = AOH * AOJ;
                AOR = AOJ;
                AOS = AOK;
            } else {
                let AOL = AOG - SY;
                let AOM = GO * AOL;
                let AON = D + (AOL * WD);
                let AOO = D + (AOM * AON);
                let AOP = XB * (D + (AOL * AOO));
                let AOQ = ((AOH * AOO) + ((((AOH * GO) * AON) + ((AOH * WD) * AOM)) * AOL)) * XB;
                AOR = AOP;
                AOS = AOQ;
            }
            let AOT = AJH * AOR;
            let AOU = (AJJ * AOR) + (AOS * AJH);
            let AOV = AOF * AOE;
            let AOW = (AOE * AOE) - AOT;
            let AOX = (AOV + AOV) - AOU;
            let AOY = LA * AIR;
            let AOZ = AIT * LA;
            let APA = (AOY * AOE) + AOT;
            let APB = ((AOZ * AOE) + (AOF * AOY)) + AOU;
            let APC = AOY * AIR;
            let APD = (AOZ * AIR) + (AIT * AOY);
            let APE = APC - AOT;
            let APF = APD - AOU;
            let APG = if AOW < -5e-3f64 { 1.0 } else { 0.0 };
            let AQL;
            let AQM;
            let AQN;
            let AQO;
            let AQP;
            let AQQ;
            let AQR;
            let AQS;
            let AQT;
            let AQU;
            let AQV;
            let AQW;
            let AQX;
            let AQY;
            if APG != 0.0 {
                let API = (AOW.abs()).sqrt();
                let APJ = (AOX * ((GX * (if AOW >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * API));
                let APK = GO * API;
                let APL = APK.tan();
                let APM = APK.cos();
                let APN = API / APL;
                let APO = (APJ - (((APJ * GO) * (GY / (APM * APM))) * APN)) / APL;
                let APP = (YY * APA) / AOW;
                let APQ = ((APB * YY) - (AOX * APP)) / AOW;
                let APR = LA - APN;
                let APS = AOW + (APN * APR);
                let APT = APS * APP;
                let APU = ((AOX + ((APO * APR) + ((APO * AC) * APN))) * APP) + (APQ * APS);
                let APV = LA * APT;
                let APW = D + APN;
                let APX = APA - (APV * APW);
                let APY = (APT * APE) / APA;
                let APZ = (APX * APP) + APY;
                let AQA = (((APB - (((APU * LA) * APW) + (APO * APV))) * APP) + (APQ * APX)) + ((((APU * APE) + (APF * APT)) - (APB * APY)) / APA);
                let AQB = D - (GO * APN);
                let AQC = (APO * GO) * AC;
                let AQD = APA / AOW;
                let AQE = AQD * AQB;
                let AQF = (((APB - (AOX * AQD)) / AOW) * AQB) + (AQC * AQD);
                let AQG = AQE + (GO * APT);
                let AQH = ((APE * AQB) - (APA * AQG)) / AOW;
                let AQI = ((((APF * AQB) + (AQC * APE)) - ((APB * AQG) + ((AQF + (APU * GO)) * APA))) - (AOX * AQH)) / AOW;
                AQL = B;
                AQM = API;
                AQN = APN;
                AQO = APT;
                AQP = APZ;
                AQQ = AQE;
                AQR = AQH;
                AQS = AFD;
                AQT = APJ;
                AQU = APO;
                AQV = APU;
                AQW = AQA;
                AQX = AQF;
                AQY = AQI;
            } else {
                let AQK = if AOW > AQJ { 1.0 } else { 0.0 };
                let AUA;
                let AUB;
                let AUC;
                let AUD;
                let AUE;
                let AUF;
                let AUG;
                let AUH;
                let AUI;
                let AUJ;
                let AUK;
                let AUL;
                let AUM;
                let AUN;
                if AQK != 0.0 {
                    let ARA = (AOW.abs()).sqrt();
                    let ARB = (AOX * ((GX * (if AOW >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * ARA));
                    let ARC = (-ARA).exp();
                    let ARD = (ARB * AC) * ARC;
                    let ARE = D + ARC;
                    let ARF = D - ARC;
                    let ARG = (ARA * ARE) / ARF;
                    let ARH = (((ARB * ARE) + (ARD * ARA)) - ((ARD * AC) * ARG)) / ARF;
                    let ARI = (YY * APA) / AOW;
                    let ARJ = ((APB * YY) - (AOX * ARI)) / AOW;
                    let ARK = LA - ARG;
                    let ARL = AOW + (ARG * ARK);
                    let ARM = ARL * ARI;
                    let ARN = ((AOX + ((ARH * ARK) + ((ARH * AC) * ARG))) * ARI) + (ARJ * ARL);
                    let ARO = LA * ARM;
                    let ARP = D + ARG;
                    let ARQ = APA - (ARO * ARP);
                    let ARR = (ARM * APE) / APA;
                    let ARS = (ARQ * ARI) + ARR;
                    let ART = (((APB - (((ARN * LA) * ARP) + (ARH * ARO))) * ARI) + (ARJ * ARQ)) + ((((ARN * APE) + (APF * ARM)) - (APB * ARR)) / APA);
                    let ARU = D - (GO * ARG);
                    let ARV = (ARH * GO) * AC;
                    let ARW = APA / AOW;
                    let ARX = ARW * ARU;
                    let ARY = (((APB - (AOX * ARW)) / AOW) * ARU) + (ARV * ARW);
                    let ARZ = ARX + (GO * ARM);
                    let ASA = ((APE * ARU) - (APA * ARZ)) / AOW;
                    let ASB = ((((APF * ARU) + (ARV * APE)) - ((APB * ARZ) + ((ARY + (ARN * GO)) * APA))) - (AOX * ASA)) / AOW;
                    AUA = ARC;
                    AUB = ARA;
                    AUC = ARG;
                    AUD = ARM;
                    AUE = ARS;
                    AUF = ARX;
                    AUG = ASA;
                    AUH = ARD;
                    AUI = ARB;
                    AUJ = ARH;
                    AUK = ARN;
                    AUL = ART;
                    AUM = ARY;
                    AUN = ASB;
                } else {
                    let ASD = AOW * ASC;
                    let ASF = AOW * ASE;
                    let ASG = AOX * ASE;
                    let ASI = D - (AOW * ASH);
                    let ASJ = D - (ASF * ASI);
                    let ASK = UC * (D - (ASD * ASJ));
                    let ASL = ((((AOX * ASC) * ASJ) + ((((ASG * ASI) + (((AOX * ASH) * AC) * ASF)) * AC) * ASD)) * AC) * UC;
                    let ASM = (AOX * ASK) + (ASL * AOW);
                    let ASN = LA + (AOW * ASK);
                    let ASP = AOW * ASO;
                    let ASQ = AOX * ASO;
                    let ASS = AOW * ASR;
                    let AST = D - ASP;
                    let ASU = D - (ASS * AST);
                    let ASV = UC * (D - (ASP * ASU));
                    let ASW = (((ASQ * ASU) + (((((AOX * ASR) * AST) + ((ASQ * AC) * ASS)) * AC) * ASP)) * AC) * UC;
                    let ASX = APA * ASV;
                    let ASY = (APB * ASV) + (ASW * APA);
                    let ATA = AOW * ASZ;
                    let ATC = ATB * AOW;
                    let ATE = D - (ATD * AOW);
                    let ATF = D - (ATC * ATE);
                    let ATH = ATG * (D - (ATA * ATF));
                    let ATI = APA * APA;
                    let ATJ = APB * APA;
                    let ATK = (APE * ASV) - (ATI * ATH);
                    let ATL = ((APF * ASV) + (ASW * APE)) - (((ATJ + ATJ) * ATH) + ((((((AOX * ASZ) * ATF) + (((((AOX * ATB) * ATE) + (((AOX * ATD) * AC) * ATC)) * AC) * ATA)) * AC) * ATG) * ATI));
                    let ATN = ATM * APA;
                    let ATO = ATN * ASK;
                    let ATP = ((APB * ATM) * ASK) + (ASL * ATN);
                    let ATR = ATQ * APE;
                    let ATT = ATS * APA;
                    let ATU = ATT * APA;
                    let ATW = LA - (ATV * AOW);
                    let ATX = D - (ASF * ATW);
                    let ATY = (ATR * ASK) + (ATU * ATX);
                    let ATZ = (((APF * ATQ) * ASK) + (ASL * ATR)) + (((((APB * ATS) * APA) + (APB * ATT)) * ATX) + ((((ASG * ATW) + (((AOX * ATV) * AC) * ASF)) * AC) * ATU));
                    AUA = B;
                    AUB = B;
                    AUC = ASN;
                    AUD = ASX;
                    AUE = ATK;
                    AUF = ATO;
                    AUG = ATY;
                    AUH = AFD;
                    AUI = AFD;
                    AUJ = ASM;
                    AUK = ASY;
                    AUL = ATL;
                    AUM = ATP;
                    AUN = ATZ;
                }
                AQL = AUA;
                AQM = AUB;
                AQN = AUC;
                AQO = AUD;
                AQP = AUE;
                AQQ = AUF;
                AQR = AUG;
                AQS = AUH;
                AQT = AUI;
                AQU = AUJ;
                AQV = AUK;
                AQW = AUL;
                AQX = AUM;
                AQY = AUN;
            }
            let AQZ = if AOW > AQJ { 1.0 } else { 0.0 };
            let AUX;
            let AUY;
            let AUZ;
            let AVA;
            if AQZ != 0.0 {
                let AUO = LA - AQL;
                let AUP = D - (AQL * AUO);
                let AUQ = (XQ * AOW) / AUP;
                let AUR = ((AOX * XQ) - ((((AQS * AUO) + ((AQS * AC) * AQL)) * AC) * AUQ)) / AUP;
                let AUS = AUQ * AQL;
                let AUT = (AUR * AQL) + (AQS * AUQ);
                let AUU = (AUQ.ln()) - AQM;
                let AUV = (AUR * (GY / AUQ)) - AQT;
                AUX = AUS;
                AUY = AUU;
                AUZ = AUT;
                AVA = AUV;
            } else {
                let AUW = if AOW < -5e-3f64 { 1.0 } else { 0.0 };
                let AVU;
                let AVV;
                let AVW;
                let AVX;
                if AUW != 0.0 {
                    let AVD = GO * AQM;
                    let AVE = AVD.sin();
                    let AVF = AVE * AVE;
                    let AVG = ((AQT * GO) * (AVD.cos())) * AVE;
                    let AVH = (-AOW) / AVF;
                    let AVI = ((AOX * AC) - ((AVG + AVG) * AVH)) / AVF;
                    let AVJ = AVH.ln();
                    let AVK = AVI * (GY / AVH);
                    AVU = AVH;
                    AVV = AVJ;
                    AVW = AVI;
                    AVX = AVK;
                } else {
                    let AVL = AOW * WD;
                    let AVM = ATB * AOW;
                    let AVO = D - (AVN * AOW);
                    let AVP = D - (AVM * AVO);
                    let AVQ = XQ - (AVL * AVP);
                    let AVR = (((AOX * WD) * AVP) + (((((AOX * ATB) * AVO) + (((AOX * AVN) * AC) * AVM)) * AC) * AVL)) * AC;
                    let AVS = AVQ.ln();
                    let AVT = AVR * (GY / AVQ);
                    AVU = AVQ;
                    AVV = AVS;
                    AVW = AVR;
                    AVX = AVT;
                }
                AUX = AVU;
                AUY = AVV;
                AUZ = AVW;
                AVA = AVX;
            }
            let AVC = if ((AVB * AOE) + AQN) > B { 1.0 } else { 0.0 };
            let AWT;
            let AWU;
            let AWV;
            let AWW;
            let AWX;
            let AWY;
            if AVC != 0.0 {
                let AVY = AOE + AQN;
                let AVZ = AOF + AQU;
                let AWA = AIR + AQO;
                let AWB = AIT + AQV;
                AWT = AVY;
                AWU = AWA;
                AWV = AQP;
                AWW = AVZ;
                AWX = AWB;
                AWY = AQW;
            } else {
                let AWC = AOE - AQN;
                let AWD = D / AWC;
                let AWE = (((AOF - AQU) * AWD) * AC) / AWC;
                let AWF = AQO - AIR;
                let AWG = AQV - AIT;
                let AWH = AOT - AUX;
                let AWI = AWH * AWD;
                let AWJ = ((AOU - AUZ) * AWD) + (AWE * AWH);
                let AWK = ((AWF * AWI) - AOT) - (AQQ * AUX);
                let AWL = AWK * AWD;
                let AWM = (((((AWG * AWI) + (AWJ * AWF)) - AOU) - ((AQX * AUX) + (AUZ * AQQ))) * AWD) + (AWE * AWK);
                let AWN = LA * AWF;
                let AWO = AQX * AQQ;
                let AWP = AQR + (AQQ * AQQ);
                let AWQ = (((AQP * AWI) + (AWN * AWL)) + AOT) - (AWP * AUX);
                let AWR = AWQ * AWD;
                let AWS = ((((((AQW * AWI) + (AWJ * AQP)) + (((AWG * LA) * AWL) + (AWM * AWN))) + AOU) - (((AQY + (AWO + AWO)) * AUX) + (AUZ * AWP))) * AWD) + (AWE * AWQ);
                AWT = AWI;
                AWU = AWL;
                AWV = AWR;
                AWW = AWJ;
                AWX = AWM;
                AWY = AWS;
            }
            let AWZ = if AWT > B { 1.0 } else { 0.0 };
            let AXT;
            let AXU;
            let AXV;
            let AXW;
            let AXX;
            let AXY;
            if AWZ != 0.0 {
                let AXA = AWT.ln();
                let AXB = AWW * (GY / AWT);
                let AXC = D / AWT;
                let AXD = ((AWW * AXC) * AC) / AWT;
                let AXE = AWU * AXC;
                let AXF = (AWX * AXC) + (AXD * AWU);
                let AXG = AXF * AXE;
                let AXH = (AWV * AXC) - (AXE * AXE);
                let AXI = ((AWY * AXC) + (AXD * AWV)) - (AXG + AXG);
                AXT = AXA;
                AXU = AXE;
                AXV = AXH;
                AXW = AXB;
                AXX = AXF;
                AXY = AXI;
            } else {
                let AXJ = -AOE;
                let AXK = (AOE + HW) + (AXJ.ln());
                let AXL = AOF + ((AOF * AC) * (GY / AXJ));
                let AXM = D / AOA;
                let AXN = ((AOB * AXM) * AC) / AOA;
                let AXO = AIR + AXM;
                let AXP = AIT + AXN;
                let AXQ = -AXM;
                let AXR = AXQ * AXM;
                let AXS = ((AXN * AC) * AXM) + (AXN * AXQ);
                AXT = AXK;
                AXU = AXO;
                AXV = AXR;
                AXW = AXL;
                AXX = AXP;
                AXY = AXS;
            }
            let AXZ = AIP - AIH;
            let AYA = AIQ - AII;
            let AYB = ((AXZ + AOA) + (LA * AXT)) - AUY;
            let AYC = (D + (LA * AXU)) - AQQ;
            let AYD = (LA * AXV) - AQR;
            let AYE = AOE + (AIU * AYB);
            let AYF = AOF + ((AIW * AYB) + ((((AYA + AOB) + (AXW * LA)) - AVA) * AIU));
            let AYG = AIR + (AIU * AYC);
            let AYH = AIT + ((AIW * AYC) + (((AXX * LA) - AQX) * AIU));
            let AYI = AIU * AYD;
            let AYJ = (AYE * AWT) - AOT;
            let AYK = ((AYF * AWT) + (AWW * AYE)) - AOU;
            let AYL = ((AYG * AWT) + (AYE * AWU)) + AOT;
            let AYM = (((AYH * AWT) + (AWW * AYG)) + ((AYF * AWU) + (AWX * AYE))) + AOU;
            let AYN = LA * AYG;
            let AYO = (((AYI * AWT) + (AYN * AWU)) + (AYE * AWV)) - AOT;
            let AYP = AYM * AYL;
            let AYQ = GO * AYJ;
            let AYR = (AYL * AYL) - (AYQ * AYO);
            let AYS = (AYP + AYP) - (((AYK * GO) * AYO) + ((((((((AIW * AYD) + (((AXY * LA) - AQY) * AIU)) * AWT) + (AWW * AYI)) + (((AYH * LA) * AWU) + (AWX * AYN))) + ((AYF * AWV) + (AWY * AYE))) - AOU) * AYQ));
            let AYT = -AYJ;
            let AYU = AYT * AYL;
            let AYV = AYS * AYR;
            let AYX = (AYR * AYR) + AYW;
            let AYY = (AYU * AYR) / AYX;
            let AYZ = AOA + AYY;
            let AZA = AOB + (((((((AYK * AC) * AYL) + (AYM * AYT)) * AYR) + (AYS * AYU)) - ((AYV + AYV) * AYY)) / AYX);
            let AZB = AIR * AYZ;
            let AZC = (AIT * AYZ) + (AZA * AIR);
            let AZD = AIU * AOC;
            let AZE = (AIW * AOC) + (AOD * AIU);
            let AZF = AZB + AZD;
            let AZG = AZC + AZE;
            let AZI = AZG * AZH;
            let AZJ = D + (AZH * AZF);
            let AZM = AZB * AZD;
            let AZN = (AZC * AZD) + (AZE * AZB);
            let AZO = (AZL + (AZK * AZF)) + AZM;
            let AZP = (AZG * AZK) + AZN;
            let AZQ = AZL * ((LA * AZF) + AZM);
            let AZR = AZP * AZO;
            let AZS = XQ * AZJ;
            let AZT = ((AZO * AZO) - (AZS * AZQ)).sqrt();
            let AZU = LA * AZJ;
            let AZV = (AZT - AZO) / AZU;
            let AZW = AZC * AZB;
            let AZX = (AZB * AZB) - AZV;
            let AZY = (AZW + AZW) - ((((((AZR + AZR) - (((AZI * XQ) * AZQ) + ((((AZG * LA) + AZN) * AZL) * AZS))) * (GY / (GX * AZT))) - AZP) - ((AZI * LA) * AZV)) / AZU);
            let AZZ = if AZX > B { 1.0 } else { 0.0 };
            let BAJ;
            let BAK;
            if AZZ != 0.0 {
                let BAA = AZX / AJH;
                let BAB = ((BAA.ln()) - AIH) + AYZ;
                let BAC = AZX * BAB;
                let BAD = (AZY * BAB) + ((((((AZY - (AJJ * BAA)) / AJH) * (GY / BAA)) - AII) + AZA) * AZX);
                let BAE = (AOY * AZB) + AZX;
                let BAF = ((AOZ * AZB) + (AZC * AOY)) + AZY;
                let BAG = (AIH - AYZ) - ALU;
                let BAI = if (if (if (if BAC < B { 1.0 } else { 0.0 }) != 0.0 && (if BAE > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((BAG + BAH) + (AIR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BAG > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BBG;
                let BBH;
                if BAI != 0.0 {
                    let BBD = BAC / BAE;
                    let BBE = AYZ - BBD;
                    let BBF = AZA - ((BAD - (BAF * BBD)) / BAE);
                    BBG = BBE;
                    BBH = BBF;
                } else {
                    BBG = AYZ;
                    BBH = AZA;
                }
                BAJ = BBG;
                BAK = BBH;
            } else {
                BAJ = AYZ;
                BAK = AZA;
            }
            let BAL = AIR * BAJ;
            let BAM = (AIT * BAJ) + (BAK * AIR);
            let BAN = BAL + AZD;
            let BAO = BAM + AZE;
            let BAP = BAO * AZH;
            let BAQ = D + (AZH * BAN);
            let BAR = BAL * AZD;
            let BAS = (BAM * AZD) + (AZE * BAL);
            let BAT = (AZL + (AZK * BAN)) + BAR;
            let BAU = (BAO * AZK) + BAS;
            let BAV = AZL * ((LA * BAN) + BAR);
            let BAW = BAU * BAT;
            let BAX = XQ * BAQ;
            let BAY = ((BAT * BAT) - (BAX * BAV)).sqrt();
            let BAZ = LA * BAQ;
            let BBA = (BAY - BAT) / BAZ;
            let BBB = (((((BAW + BAW) - (((BAP * XQ) * BAV) + ((((BAO * LA) + BAS) * AZL) * BAX))) * (GY / (GX * BAY))) - BAU) - ((BAP * LA) * BBA)) / BAZ;
            let BBC = if BBA < -5e-3f64 { 1.0 } else { 0.0 };
            let BBT;
            let BBU;
            let BBV;
            let BBW;
            let BBX;
            let BBY;
            let BBZ;
            let BCA;
            if BBC != 0.0 {
                let BBI = (BBA.abs()).sqrt();
                let BBJ = (BBB * ((GX * (if BBA >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BBI));
                let BBK = GO * BBI;
                let BBL = BBK.tan();
                let BBM = BBK.cos();
                let BBN = BBI / BBL;
                let BBO = (BBJ - (((BBJ * GO) * (GY / (BBM * BBM))) * BBN)) / BBL;
                let BBP = LA - BBN;
                let BBQ = (YY * (BBA + (BBN * BBP))) / BBA;
                let BBR = (((BBB + ((BBO * BBP) + ((BBO * AC) * BBN))) * YY) - (BBB * BBQ)) / BBA;
                BBT = BBN;
                BBU = BBQ;
                BBV = AQL;
                BBW = BBI;
                BBX = BBO;
                BBY = BBR;
                BBZ = AQS;
                BCA = BBJ;
            } else {
                let BBS = if BBA > AQJ { 1.0 } else { 0.0 };
                let BDF;
                let BDG;
                let BDH;
                let BDI;
                let BDJ;
                let BDK;
                let BDL;
                let BDM;
                if BBS != 0.0 {
                    let BCH = (BBA.abs()).sqrt();
                    let BCI = (BBB * ((GX * (if BBA >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BCH));
                    let BCJ = (-BCH).exp();
                    let BCK = (BCI * AC) * BCJ;
                    let BCL = D + BCJ;
                    let BCM = D - BCJ;
                    let BCN = (BCH * BCL) / BCM;
                    let BCO = (((BCI * BCL) + (BCK * BCH)) - ((BCK * AC) * BCN)) / BCM;
                    let BCP = LA - BCN;
                    let BCQ = (YY * (BBA + (BCN * BCP))) / BBA;
                    let BCR = (((BBB + ((BCO * BCP) + ((BCO * AC) * BCN))) * YY) - (BBB * BCQ)) / BBA;
                    BDF = BCN;
                    BDG = BCQ;
                    BDH = BCJ;
                    BDI = BCH;
                    BDJ = BCO;
                    BDK = BCR;
                    BDL = BCK;
                    BDM = BCI;
                } else {
                    let BCS = BBA * UC;
                    let BCT = BBA * ASC;
                    let BCU = D - (BBA * ASE);
                    let BCV = D - (BCT * BCU);
                    let BCW = ((BBB * UC) * BCV) + (((((BBB * ASC) * BCU) + (((BBB * ASE) * AC) * BCT)) * AC) * BCS);
                    let BCX = LA + (BCS * BCV);
                    let BCY = BBA * ASO;
                    let BCZ = BBB * ASO;
                    let BDA = BBA * ASR;
                    let BDB = D - BCY;
                    let BDC = D - (BDA * BDB);
                    let BDD = UC * (D - (BCY * BDC));
                    let BDE = (((BCZ * BDC) + (((((BBB * ASR) * BDB) + ((BCZ * AC) * BDA)) * AC) * BCY)) * AC) * UC;
                    BDF = BCX;
                    BDG = BDD;
                    BDH = AQL;
                    BDI = AQM;
                    BDJ = BCW;
                    BDK = BDE;
                    BDL = AQS;
                    BDM = AQT;
                }
                BBT = BDF;
                BBU = BDG;
                BBV = BDH;
                BBW = BDI;
                BBX = BDJ;
                BBY = BDK;
                BBZ = BDL;
                BCA = BDM;
            }
            let BCB = (BAN * BBU) + D;
            let BCC = (((BAN * BBT) + BAR) + BBA) / BCB;
            let BCD = BAM * BAL;
            let BCE = (BAL * BAL) - (BBA - BCC);
            let BCF = (BCD + BCD) - (BBB - ((((((BAO * BBT) + (BBX * BAN)) + BAS) + BBB) - (((BAO * BBU) + (BBY * BAN)) * BCC)) / BCB));
            let BCG = if BCE > B { 1.0 } else { 0.0 };
            let BDV;
            let BDW;
            if BCG != 0.0 {
                let BDN = BCE / AJH;
                let BDO = ((BDN.ln()) - AIH) + BAJ;
                let BDP = BCE * BDO;
                let BDQ = (BCF * BDO) + ((((((BCF - (AJJ * BDN)) / AJH) * (GY / BDN)) - AII) + BAK) * BCE);
                let BDR = (AOY * BAL) + BCE;
                let BDS = ((AOZ * BAL) + (BAM * AOY)) + BCF;
                let BDT = (AIH - BAJ) - ALU;
                let BDU = if (if (if (if BDP < B { 1.0 } else { 0.0 }) != 0.0 && (if BDR > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((BDT + BAH) + (AIR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BDT > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BEF;
                let BEG;
                if BDU != 0.0 {
                    let BEC = BDP / BDR;
                    let BED = BAJ - BEC;
                    let BEE = BAK - ((BDQ - (BDS * BEC)) / BDR);
                    BEF = BED;
                    BEG = BEE;
                } else {
                    BEF = BAJ;
                    BEG = BAK;
                }
                BDV = BEF;
                BDW = BEG;
            } else {
                BDV = BAJ;
                BDW = BAK;
            }
            let BDX = AIR * BDV;
            let BDY = (AIT * BDV) + (BDW * AIR);
            let BDZ = AIH - BDV;
            let BEA = AII - BDW;
            let BEB = if BDZ < SY { 1.0 } else { 0.0 };
            let BEP;
            let BEQ;
            if BEB != 0.0 {
                let BEH = BDZ.exp();
                let BEI = BEA * BEH;
                BEP = BEH;
                BEQ = BEI;
            } else {
                let BEJ = BDZ - SY;
                let BEK = GO * BEJ;
                let BEL = D + (BEJ * WD);
                let BEM = D + (BEK * BEL);
                let BEN = XB * (D + (BEJ * BEM));
                let BEO = ((BEA * BEM) + ((((BEA * GO) * BEL) + ((BEA * WD) * BEK)) * BEJ)) * XB;
                BEP = BEN;
                BEQ = BEO;
            }
            let BER = AJH * BEP;
            let BES = (AJJ * BEP) + (BEQ * AJH);
            let BET = BDY * BDX;
            let BEU = (BDX * BDX) - BER;
            let BEV = (BET + BET) - BES;
            let BEW = (AOY * BDX) + BER;
            let BEX = ((AOZ * BDX) + (BDY * AOY)) + BES;
            let BEY = APC - BER;
            let BEZ = APD - BES;
            let BFA = if BEU < -5e-3f64 { 1.0 } else { 0.0 };
            let BGD;
            let BGE;
            let BGF;
            let BGG;
            let BGH;
            let BGI;
            let BGJ;
            let BGK;
            let BGL;
            let BGM;
            let BGN;
            let BGO;
            let BGP;
            let BGQ;
            if BFA != 0.0 {
                let BFB = (BEU.abs()).sqrt();
                let BFC = (BEV * ((GX * (if BEU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BFB));
                let BFD = GO * BFB;
                let BFE = BFD.tan();
                let BFF = BFD.cos();
                let BFG = BFB / BFE;
                let BFH = (BFC - (((BFC * GO) * (GY / (BFF * BFF))) * BFG)) / BFE;
                let BFI = (YY * BEW) / BEU;
                let BFJ = ((BEX * YY) - (BEV * BFI)) / BEU;
                let BFK = LA - BFG;
                let BFL = BEU + (BFG * BFK);
                let BFM = BFL * BFI;
                let BFN = ((BEV + ((BFH * BFK) + ((BFH * AC) * BFG))) * BFI) + (BFJ * BFL);
                let BFO = LA * BFM;
                let BFP = D + BFG;
                let BFQ = BEW - (BFO * BFP);
                let BFR = (BFM * BEY) / BEW;
                let BFS = (BFQ * BFI) + BFR;
                let BFT = (((BEX - (((BFN * LA) * BFP) + (BFH * BFO))) * BFI) + (BFJ * BFQ)) + ((((BFN * BEY) + (BEZ * BFM)) - (BEX * BFR)) / BEW);
                let BFU = D - (GO * BFG);
                let BFV = (BFH * GO) * AC;
                let BFW = BEW / BEU;
                let BFX = BFW * BFU;
                let BFY = (((BEX - (BEV * BFW)) / BEU) * BFU) + (BFV * BFW);
                let BFZ = BFX + (GO * BFM);
                let BGA = ((BEY * BFU) - (BEW * BFZ)) / BEU;
                let BGB = ((((BEZ * BFU) + (BFV * BEY)) - ((BEX * BFZ) + ((BFY + (BFN * GO)) * BEW))) - (BEV * BGA)) / BEU;
                BGD = BBV;
                BGE = BFB;
                BGF = BFG;
                BGG = BFM;
                BGH = BFS;
                BGI = BFX;
                BGJ = BGA;
                BGK = BBZ;
                BGL = BFC;
                BGM = BFH;
                BGN = BFN;
                BGO = BFT;
                BGP = BFY;
                BGQ = BGB;
            } else {
                let BGC = if BEU > AQJ { 1.0 } else { 0.0 };
                let BJI;
                let BJJ;
                let BJK;
                let BJL;
                let BJM;
                let BJN;
                let BJO;
                let BJP;
                let BJQ;
                let BJR;
                let BJS;
                let BJT;
                let BJU;
                let BJV;
                if BGC != 0.0 {
                    let BGS = (BEU.abs()).sqrt();
                    let BGT = (BEV * ((GX * (if BEU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BGS));
                    let BGU = (-BGS).exp();
                    let BGV = (BGT * AC) * BGU;
                    let BGW = D + BGU;
                    let BGX = D - BGU;
                    let BGY = (BGS * BGW) / BGX;
                    let BGZ = (((BGT * BGW) + (BGV * BGS)) - ((BGV * AC) * BGY)) / BGX;
                    let BHA = (YY * BEW) / BEU;
                    let BHB = ((BEX * YY) - (BEV * BHA)) / BEU;
                    let BHC = LA - BGY;
                    let BHD = BEU + (BGY * BHC);
                    let BHE = BHD * BHA;
                    let BHF = ((BEV + ((BGZ * BHC) + ((BGZ * AC) * BGY))) * BHA) + (BHB * BHD);
                    let BHG = LA * BHE;
                    let BHH = D + BGY;
                    let BHI = BEW - (BHG * BHH);
                    let BHJ = (BHE * BEY) / BEW;
                    let BHK = (BHI * BHA) + BHJ;
                    let BHL = (((BEX - (((BHF * LA) * BHH) + (BGZ * BHG))) * BHA) + (BHB * BHI)) + ((((BHF * BEY) + (BEZ * BHE)) - (BEX * BHJ)) / BEW);
                    let BHM = D - (GO * BGY);
                    let BHN = (BGZ * GO) * AC;
                    let BHO = BEW / BEU;
                    let BHP = BHO * BHM;
                    let BHQ = (((BEX - (BEV * BHO)) / BEU) * BHM) + (BHN * BHO);
                    let BHR = BHP + (GO * BHE);
                    let BHS = ((BEY * BHM) - (BEW * BHR)) / BEU;
                    let BHT = ((((BEZ * BHM) + (BHN * BEY)) - ((BEX * BHR) + ((BHQ + (BHF * GO)) * BEW))) - (BEV * BHS)) / BEU;
                    BJI = BGU;
                    BJJ = BGS;
                    BJK = BGY;
                    BJL = BHE;
                    BJM = BHK;
                    BJN = BHP;
                    BJO = BHS;
                    BJP = BGV;
                    BJQ = BGT;
                    BJR = BGZ;
                    BJS = BHF;
                    BJT = BHL;
                    BJU = BHQ;
                    BJV = BHT;
                } else {
                    let BHU = BEU * ASC;
                    let BHV = BEU * ASE;
                    let BHW = BEV * ASE;
                    let BHX = D - (BEU * ASH);
                    let BHY = D - (BHV * BHX);
                    let BHZ = UC * (D - (BHU * BHY));
                    let BIA = ((((BEV * ASC) * BHY) + ((((BHW * BHX) + (((BEV * ASH) * AC) * BHV)) * AC) * BHU)) * AC) * UC;
                    let BIB = (BEV * BHZ) + (BIA * BEU);
                    let BIC = LA + (BEU * BHZ);
                    let BID = BEU * ASO;
                    let BIE = BEV * ASO;
                    let BIF = BEU * ASR;
                    let BIG = D - BID;
                    let BIH = D - (BIF * BIG);
                    let BII = UC * (D - (BID * BIH));
                    let BIJ = (((BIE * BIH) + (((((BEV * ASR) * BIG) + ((BIE * AC) * BIF)) * AC) * BID)) * AC) * UC;
                    let BIK = BEW * BII;
                    let BIL = (BEX * BII) + (BIJ * BEW);
                    let BIM = BEU * ASZ;
                    let BIN = ATB * BEU;
                    let BIO = D - (ATD * BEU);
                    let BIP = D - (BIN * BIO);
                    let BIQ = ATG * (D - (BIM * BIP));
                    let BIR = BEW * BEW;
                    let BIS = BEX * BEW;
                    let BIT = (BEY * BII) - (BIR * BIQ);
                    let BIU = ((BEZ * BII) + (BIJ * BEY)) - (((BIS + BIS) * BIQ) + ((((((BEV * ASZ) * BIP) + (((((BEV * ATB) * BIO) + (((BEV * ATD) * AC) * BIN)) * AC) * BIM)) * AC) * ATG) * BIR));
                    let BIW = BIV * BEW;
                    let BIX = BIW * BHZ;
                    let BIY = ((BEX * BIV) * BHZ) + (BIA * BIW);
                    let BJA = BIZ * BEY;
                    let BJC = BJB * BEW;
                    let BJD = BJC * BEW;
                    let BJE = LA - (ATV * BEU);
                    let BJF = D - (BHV * BJE);
                    let BJG = (BJA * BHZ) + (BJD * BJF);
                    let BJH = (((BEZ * BIZ) * BHZ) + (BIA * BJA)) + (((((BEX * BJB) * BEW) + (BEX * BJC)) * BJF) + ((((BHW * BJE) + (((BEV * ATV) * AC) * BHV)) * AC) * BJD));
                    BJI = BBV;
                    BJJ = BBW;
                    BJK = BIC;
                    BJL = BIK;
                    BJM = BIT;
                    BJN = BIX;
                    BJO = BJG;
                    BJP = BBZ;
                    BJQ = BCA;
                    BJR = BIB;
                    BJS = BIL;
                    BJT = BIU;
                    BJU = BIY;
                    BJV = BJH;
                }
                BGD = BJI;
                BGE = BJJ;
                BGF = BJK;
                BGG = BJL;
                BGH = BJM;
                BGI = BJN;
                BGJ = BJO;
                BGK = BJP;
                BGL = BJQ;
                BGM = BJR;
                BGN = BJS;
                BGO = BJT;
                BGP = BJU;
                BGQ = BJV;
            }
            let BGR = if BEU > AQJ { 1.0 } else { 0.0 };
            let BKF;
            let BKG;
            let BKH;
            let BKI;
            if BGR != 0.0 {
                let BJW = LA - BGD;
                let BJX = D - (BGD * BJW);
                let BJY = (XQ * BEU) / BJX;
                let BJZ = ((BEV * XQ) - ((((BGK * BJW) + ((BGK * AC) * BGD)) * AC) * BJY)) / BJX;
                let BKA = BJY * BGD;
                let BKB = (BJZ * BGD) + (BGK * BJY);
                let BKC = (BJY.ln()) - BGE;
                let BKD = (BJZ * (GY / BJY)) - BGL;
                BKF = BKA;
                BKG = BKC;
                BKH = BKB;
                BKI = BKD;
            } else {
                let BKE = if BEU < -5e-3f64 { 1.0 } else { 0.0 };
                let BLA;
                let BLB;
                let BLC;
                let BLD;
                if BKE != 0.0 {
                    let BKK = GO * BGE;
                    let BKL = BKK.sin();
                    let BKM = BKL * BKL;
                    let BKN = ((BGL * GO) * (BKK.cos())) * BKL;
                    let BKO = (-BEU) / BKM;
                    let BKP = ((BEV * AC) - ((BKN + BKN) * BKO)) / BKM;
                    let BKQ = BKO.ln();
                    let BKR = BKP * (GY / BKO);
                    BLA = BKO;
                    BLB = BKQ;
                    BLC = BKP;
                    BLD = BKR;
                } else {
                    let BKS = BEU * WD;
                    let BKT = ATB * BEU;
                    let BKU = D - (AVN * BEU);
                    let BKV = D - (BKT * BKU);
                    let BKW = XQ - (BKS * BKV);
                    let BKX = (((BEV * WD) * BKV) + (((((BEV * ATB) * BKU) + (((BEV * AVN) * AC) * BKT)) * AC) * BKS)) * AC;
                    let BKY = BKW.ln();
                    let BKZ = BKX * (GY / BKW);
                    BLA = BKW;
                    BLB = BKY;
                    BLC = BKX;
                    BLD = BKZ;
                }
                BKF = BLA;
                BKG = BLB;
                BKH = BLC;
                BKI = BLD;
            }
            let BKJ = if ((AVB * BDX) + BGF) > B { 1.0 } else { 0.0 };
            let BLZ;
            let BMA;
            let BMB;
            let BMC;
            let BMD;
            let BME;
            if BKJ != 0.0 {
                let BLE = BDX + BGF;
                let BLF = BDY + BGM;
                let BLG = AIR + BGG;
                let BLH = AIT + BGN;
                BLZ = BLE;
                BMA = BLG;
                BMB = BGH;
                BMC = BLF;
                BMD = BLH;
                BME = BGO;
            } else {
                let BLI = BDX - BGF;
                let BLJ = D / BLI;
                let BLK = (((BDY - BGM) * BLJ) * AC) / BLI;
                let BLL = BGG - AIR;
                let BLM = BGN - AIT;
                let BLN = BER - BKF;
                let BLO = BLN * BLJ;
                let BLP = ((BES - BKH) * BLJ) + (BLK * BLN);
                let BLQ = ((BLL * BLO) - BER) - (BGI * BKF);
                let BLR = BLQ * BLJ;
                let BLS = (((((BLM * BLO) + (BLP * BLL)) - BES) - ((BGP * BKF) + (BKH * BGI))) * BLJ) + (BLK * BLQ);
                let BLT = LA * BLL;
                let BLU = BGP * BGI;
                let BLV = BGJ + (BGI * BGI);
                let BLW = (((BGH * BLO) + (BLT * BLR)) + BER) - (BLV * BKF);
                let BLX = BLW * BLJ;
                let BLY = ((((((BGO * BLO) + (BLP * BGH)) + (((BLM * LA) * BLR) + (BLS * BLT))) + BES) - (((BGQ + (BLU + BLU)) * BKF) + (BKH * BLV))) * BLJ) + (BLK * BLW);
                BLZ = BLO;
                BMA = BLR;
                BMB = BLX;
                BMC = BLP;
                BMD = BLS;
                BME = BLY;
            }
            let BMF = if BLZ > B { 1.0 } else { 0.0 };
            let BMZ;
            let BNA;
            let BNB;
            let BNC;
            let BND;
            let BNE;
            if BMF != 0.0 {
                let BMG = BLZ.ln();
                let BMH = BMC * (GY / BLZ);
                let BMI = D / BLZ;
                let BMJ = ((BMC * BMI) * AC) / BLZ;
                let BMK = BMA * BMI;
                let BML = (BMD * BMI) + (BMJ * BMA);
                let BMM = BML * BMK;
                let BMN = (BMB * BMI) - (BMK * BMK);
                let BMO = ((BME * BMI) + (BMJ * BMB)) - (BMM + BMM);
                BMZ = BMG;
                BNA = BMK;
                BNB = BMN;
                BNC = BMH;
                BND = BML;
                BNE = BMO;
            } else {
                let BMP = -BDX;
                let BMQ = (BDX + HW) + (BMP.ln());
                let BMR = BDY + ((BDY * AC) * (GY / BMP));
                let BMS = D / BDV;
                let BMT = ((BDW * BMS) * AC) / BDV;
                let BMU = AIR + BMS;
                let BMV = AIT + BMT;
                let BMW = -BMS;
                let BMX = BMW * BMS;
                let BMY = ((BMT * AC) * BMS) + (BMT * BMW);
                BMZ = BMQ;
                BNA = BMU;
                BNB = BMX;
                BNC = BMR;
                BND = BMV;
                BNE = BMY;
            }
            let BNF = ((AXZ + BDV) + (LA * BMZ)) - BKG;
            let BNG = (D + (LA * BNA)) - BGI;
            let BNH = (LA * BNB) - BGJ;
            let BNI = BDX + (AIU * BNF);
            let BNJ = BDY + ((AIW * BNF) + ((((AYA + BDW) + (BNC * LA)) - BKI) * AIU));
            let BNK = AIR + (AIU * BNG);
            let BNL = AIT + ((AIW * BNG) + (((BND * LA) - BGP) * AIU));
            let BNM = AIU * BNH;
            let BNN = (BNI * BLZ) - BER;
            let BNO = ((BNJ * BLZ) + (BMC * BNI)) - BES;
            let BNP = ((BNK * BLZ) + (BNI * BMA)) + BER;
            let BNQ = (((BNL * BLZ) + (BMC * BNK)) + ((BNJ * BMA) + (BMD * BNI))) + BES;
            let BNR = LA * BNK;
            let BNS = (((BNM * BLZ) + (BNR * BMA)) + (BNI * BMB)) - BER;
            let BNT = BNQ * BNP;
            let BNU = GO * BNN;
            let BNV = (BNP * BNP) - (BNU * BNS);
            let BNW = (BNT + BNT) - (((BNO * GO) * BNS) + ((((((((AIW * BNH) + (((BNE * LA) - BGQ) * AIU)) * BLZ) + (BMC * BNM)) + (((BNL * LA) * BMA) + (BMD * BNR))) + ((BNJ * BMB) + (BME * BNI))) - BES) * BNU));
            let BNX = -BNN;
            let BNY = BNX * BNP;
            let BNZ = BNW * BNV;
            let BOA = (BNV * BNV) + AYW;
            let BOB = (BNY * BNV) / BOA;
            let BOC = BDV + BOB;
            let BOD = BDW + (((((((BNO * AC) * BNP) + (BNQ * BNX)) * BNV) + (BNW * BNY)) - ((BNZ + BNZ) * BOB)) / BOA);
            let BOE = AIR * BOC;
            let BOF = (AIT * BOC) + (BOD * AIR);
            let BOG = AIH - BOC;
            let BOH = AII - BOD;
            let BOI = if BOG < SY { 1.0 } else { 0.0 };
            let BOR;
            let BOS;
            if BOI != 0.0 {
                let BOJ = BOG.exp();
                let BOK = BOH * BOJ;
                BOR = BOJ;
                BOS = BOK;
            } else {
                let BOL = BOG - SY;
                let BOM = GO * BOL;
                let BON = D + (BOL * WD);
                let BOO = D + (BOM * BON);
                let BOP = XB * (D + (BOL * BOO));
                let BOQ = ((BOH * BOO) + ((((BOH * GO) * BON) + ((BOH * WD) * BOM)) * BOL)) * XB;
                BOR = BOP;
                BOS = BOQ;
            }
            let BOT = AJH * BOR;
            let BOU = (AJJ * BOR) + (BOS * AJH);
            let BOV = BOF * BOE;
            let BOW = (BOE * BOE) - BOT;
            let BOX = (BOV + BOV) - BOU;
            let BOY = (AOY * BOE) + BOT;
            let BOZ = ((AOZ * BOE) + (BOF * AOY)) + BOU;
            let BPA = APC - BOT;
            let BPB = APD - BOU;
            let BPC = if BOW < -5e-3f64 { 1.0 } else { 0.0 };
            let BQF;
            let BQG;
            let BQH;
            let BQI;
            let BQJ;
            let BQK;
            let BQL;
            let BQM;
            let BQN;
            let BQO;
            let BQP;
            let BQQ;
            let BQR;
            let BQS;
            if BPC != 0.0 {
                let BPD = (BOW.abs()).sqrt();
                let BPE = (BOX * ((GX * (if BOW >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BPD));
                let BPF = GO * BPD;
                let BPG = BPF.tan();
                let BPH = BPF.cos();
                let BPI = BPD / BPG;
                let BPJ = (BPE - (((BPE * GO) * (GY / (BPH * BPH))) * BPI)) / BPG;
                let BPK = (YY * BOY) / BOW;
                let BPL = ((BOZ * YY) - (BOX * BPK)) / BOW;
                let BPM = LA - BPI;
                let BPN = BOW + (BPI * BPM);
                let BPO = BPN * BPK;
                let BPP = ((BOX + ((BPJ * BPM) + ((BPJ * AC) * BPI))) * BPK) + (BPL * BPN);
                let BPQ = LA * BPO;
                let BPR = D + BPI;
                let BPS = BOY - (BPQ * BPR);
                let BPT = (BPO * BPA) / BOY;
                let BPU = (BPS * BPK) + BPT;
                let BPV = (((BOZ - (((BPP * LA) * BPR) + (BPJ * BPQ))) * BPK) + (BPL * BPS)) + ((((BPP * BPA) + (BPB * BPO)) - (BOZ * BPT)) / BOY);
                let BPW = D - (GO * BPI);
                let BPX = (BPJ * GO) * AC;
                let BPY = BOY / BOW;
                let BPZ = BPY * BPW;
                let BQA = (((BOZ - (BOX * BPY)) / BOW) * BPW) + (BPX * BPY);
                let BQB = BPZ + (GO * BPO);
                let BQC = ((BPA * BPW) - (BOY * BQB)) / BOW;
                let BQD = ((((BPB * BPW) + (BPX * BPA)) - ((BOZ * BQB) + ((BQA + (BPP * GO)) * BOY))) - (BOX * BQC)) / BOW;
                BQF = BGD;
                BQG = BPD;
                BQH = BPI;
                BQI = BPO;
                BQJ = BPU;
                BQK = BPZ;
                BQL = BQC;
                BQM = BGK;
                BQN = BPE;
                BQO = BPJ;
                BQP = BPP;
                BQQ = BPV;
                BQR = BQA;
                BQS = BQD;
            } else {
                let BQE = if BOW > AQJ { 1.0 } else { 0.0 };
                let BTK;
                let BTL;
                let BTM;
                let BTN;
                let BTO;
                let BTP;
                let BTQ;
                let BTR;
                let BTS;
                let BTT;
                let BTU;
                let BTV;
                let BTW;
                let BTX;
                if BQE != 0.0 {
                    let BQU = (BOW.abs()).sqrt();
                    let BQV = (BOX * ((GX * (if BOW >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BQU));
                    let BQW = (-BQU).exp();
                    let BQX = (BQV * AC) * BQW;
                    let BQY = D + BQW;
                    let BQZ = D - BQW;
                    let BRA = (BQU * BQY) / BQZ;
                    let BRB = (((BQV * BQY) + (BQX * BQU)) - ((BQX * AC) * BRA)) / BQZ;
                    let BRC = (YY * BOY) / BOW;
                    let BRD = ((BOZ * YY) - (BOX * BRC)) / BOW;
                    let BRE = LA - BRA;
                    let BRF = BOW + (BRA * BRE);
                    let BRG = BRF * BRC;
                    let BRH = ((BOX + ((BRB * BRE) + ((BRB * AC) * BRA))) * BRC) + (BRD * BRF);
                    let BRI = LA * BRG;
                    let BRJ = D + BRA;
                    let BRK = BOY - (BRI * BRJ);
                    let BRL = (BRG * BPA) / BOY;
                    let BRM = (BRK * BRC) + BRL;
                    let BRN = (((BOZ - (((BRH * LA) * BRJ) + (BRB * BRI))) * BRC) + (BRD * BRK)) + ((((BRH * BPA) + (BPB * BRG)) - (BOZ * BRL)) / BOY);
                    let BRO = D - (GO * BRA);
                    let BRP = (BRB * GO) * AC;
                    let BRQ = BOY / BOW;
                    let BRR = BRQ * BRO;
                    let BRS = (((BOZ - (BOX * BRQ)) / BOW) * BRO) + (BRP * BRQ);
                    let BRT = BRR + (GO * BRG);
                    let BRU = ((BPA * BRO) - (BOY * BRT)) / BOW;
                    let BRV = ((((BPB * BRO) + (BRP * BPA)) - ((BOZ * BRT) + ((BRS + (BRH * GO)) * BOY))) - (BOX * BRU)) / BOW;
                    BTK = BQW;
                    BTL = BQU;
                    BTM = BRA;
                    BTN = BRG;
                    BTO = BRM;
                    BTP = BRR;
                    BTQ = BRU;
                    BTR = BQX;
                    BTS = BQV;
                    BTT = BRB;
                    BTU = BRH;
                    BTV = BRN;
                    BTW = BRS;
                    BTX = BRV;
                } else {
                    let BRW = BOW * ASC;
                    let BRX = BOW * ASE;
                    let BRY = BOX * ASE;
                    let BRZ = D - (BOW * ASH);
                    let BSA = D - (BRX * BRZ);
                    let BSB = UC * (D - (BRW * BSA));
                    let BSC = ((((BOX * ASC) * BSA) + ((((BRY * BRZ) + (((BOX * ASH) * AC) * BRX)) * AC) * BRW)) * AC) * UC;
                    let BSD = (BOX * BSB) + (BSC * BOW);
                    let BSE = LA + (BOW * BSB);
                    let BSF = BOW * ASO;
                    let BSG = BOX * ASO;
                    let BSH = BOW * ASR;
                    let BSI = D - BSF;
                    let BSJ = D - (BSH * BSI);
                    let BSK = UC * (D - (BSF * BSJ));
                    let BSL = (((BSG * BSJ) + (((((BOX * ASR) * BSI) + ((BSG * AC) * BSH)) * AC) * BSF)) * AC) * UC;
                    let BSM = BOY * BSK;
                    let BSN = (BOZ * BSK) + (BSL * BOY);
                    let BSO = BOW * ASZ;
                    let BSP = ATB * BOW;
                    let BSQ = D - (ATD * BOW);
                    let BSR = D - (BSP * BSQ);
                    let BSS = ATG * (D - (BSO * BSR));
                    let BST = BOY * BOY;
                    let BSU = BOZ * BOY;
                    let BSV = (BPA * BSK) - (BST * BSS);
                    let BSW = ((BPB * BSK) + (BSL * BPA)) - (((BSU + BSU) * BSS) + ((((((BOX * ASZ) * BSR) + (((((BOX * ATB) * BSQ) + (((BOX * ATD) * AC) * BSP)) * AC) * BSO)) * AC) * ATG) * BST));
                    let BSY = BSX * BOY;
                    let BSZ = BSY * BSB;
                    let BTA = ((BOZ * BSX) * BSB) + (BSC * BSY);
                    let BTC = BTB * BPA;
                    let BTE = BTD * BOY;
                    let BTF = BTE * BOY;
                    let BTG = LA - (ATV * BOW);
                    let BTH = D - (BRX * BTG);
                    let BTI = (BTC * BSB) + (BTF * BTH);
                    let BTJ = (((BPB * BTB) * BSB) + (BSC * BTC)) + (((((BOZ * BTD) * BOY) + (BOZ * BTE)) * BTH) + ((((BRY * BTG) + (((BOX * ATV) * AC) * BRX)) * AC) * BTF));
                    BTK = BGD;
                    BTL = BGE;
                    BTM = BSE;
                    BTN = BSM;
                    BTO = BSV;
                    BTP = BSZ;
                    BTQ = BTI;
                    BTR = BGK;
                    BTS = BGL;
                    BTT = BSD;
                    BTU = BSN;
                    BTV = BSW;
                    BTW = BTA;
                    BTX = BTJ;
                }
                BQF = BTK;
                BQG = BTL;
                BQH = BTM;
                BQI = BTN;
                BQJ = BTO;
                BQK = BTP;
                BQL = BTQ;
                BQM = BTR;
                BQN = BTS;
                BQO = BTT;
                BQP = BTU;
                BQQ = BTV;
                BQR = BTW;
                BQS = BTX;
            }
            let BQT = if BOW > AQJ { 1.0 } else { 0.0 };
            let BUH;
            let BUI;
            let BUJ;
            let BUK;
            if BQT != 0.0 {
                let BTY = LA - BQF;
                let BTZ = D - (BQF * BTY);
                let BUA = (XQ * BOW) / BTZ;
                let BUB = ((BOX * XQ) - ((((BQM * BTY) + ((BQM * AC) * BQF)) * AC) * BUA)) / BTZ;
                let BUC = BUA * BQF;
                let BUD = (BUB * BQF) + (BQM * BUA);
                let BUE = (BUA.ln()) - BQG;
                let BUF = (BUB * (GY / BUA)) - BQN;
                BUH = BUC;
                BUI = BUE;
                BUJ = BUD;
                BUK = BUF;
            } else {
                let BUG = if BOW < -5e-3f64 { 1.0 } else { 0.0 };
                let BVC;
                let BVD;
                let BVE;
                let BVF;
                if BUG != 0.0 {
                    let BUM = GO * BQG;
                    let BUN = BUM.sin();
                    let BUO = BUN * BUN;
                    let BUP = ((BQN * GO) * (BUM.cos())) * BUN;
                    let BUQ = (-BOW) / BUO;
                    let BUR = ((BOX * AC) - ((BUP + BUP) * BUQ)) / BUO;
                    let BUS = BUQ.ln();
                    let BUT = BUR * (GY / BUQ);
                    BVC = BUQ;
                    BVD = BUS;
                    BVE = BUR;
                    BVF = BUT;
                } else {
                    let BUU = BOW * WD;
                    let BUV = ATB * BOW;
                    let BUW = D - (AVN * BOW);
                    let BUX = D - (BUV * BUW);
                    let BUY = XQ - (BUU * BUX);
                    let BUZ = (((BOX * WD) * BUX) + (((((BOX * ATB) * BUW) + (((BOX * AVN) * AC) * BUV)) * AC) * BUU)) * AC;
                    let BVA = BUY.ln();
                    let BVB = BUZ * (GY / BUY);
                    BVC = BUY;
                    BVD = BVA;
                    BVE = BUZ;
                    BVF = BVB;
                }
                BUH = BVC;
                BUI = BVD;
                BUJ = BVE;
                BUK = BVF;
            }
            let BUL = if ((AVB * BOE) + BQH) > B { 1.0 } else { 0.0 };
            let BWB;
            let BWC;
            let BWD;
            let BWE;
            let BWF;
            let BWG;
            if BUL != 0.0 {
                let BVG = BOE + BQH;
                let BVH = BOF + BQO;
                let BVI = AIR + BQI;
                let BVJ = AIT + BQP;
                BWB = BVG;
                BWC = BVI;
                BWD = BQJ;
                BWE = BVH;
                BWF = BVJ;
                BWG = BQQ;
            } else {
                let BVK = BOE - BQH;
                let BVL = D / BVK;
                let BVM = (((BOF - BQO) * BVL) * AC) / BVK;
                let BVN = BQI - AIR;
                let BVO = BQP - AIT;
                let BVP = BOT - BUH;
                let BVQ = BVP * BVL;
                let BVR = ((BOU - BUJ) * BVL) + (BVM * BVP);
                let BVS = ((BVN * BVQ) - BOT) - (BQK * BUH);
                let BVT = BVS * BVL;
                let BVU = (((((BVO * BVQ) + (BVR * BVN)) - BOU) - ((BQR * BUH) + (BUJ * BQK))) * BVL) + (BVM * BVS);
                let BVV = LA * BVN;
                let BVW = BQR * BQK;
                let BVX = BQL + (BQK * BQK);
                let BVY = (((BQJ * BVQ) + (BVV * BVT)) + BOT) - (BVX * BUH);
                let BVZ = BVY * BVL;
                let BWA = ((((((BQQ * BVQ) + (BVR * BQJ)) + (((BVO * LA) * BVT) + (BVU * BVV))) + BOU) - (((BQS + (BVW + BVW)) * BUH) + (BUJ * BVX))) * BVL) + (BVM * BVY);
                BWB = BVQ;
                BWC = BVT;
                BWD = BVZ;
                BWE = BVR;
                BWF = BVU;
                BWG = BWA;
            }
            let BWH = if BWB > B { 1.0 } else { 0.0 };
            let BXB;
            let BXC;
            let BXD;
            let BXE;
            let BXF;
            let BXG;
            if BWH != 0.0 {
                let BWI = BWB.ln();
                let BWJ = BWE * (GY / BWB);
                let BWK = D / BWB;
                let BWL = ((BWE * BWK) * AC) / BWB;
                let BWM = BWC * BWK;
                let BWN = (BWF * BWK) + (BWL * BWC);
                let BWO = BWN * BWM;
                let BWP = (BWD * BWK) - (BWM * BWM);
                let BWQ = ((BWG * BWK) + (BWL * BWD)) - (BWO + BWO);
                BXB = BWI;
                BXC = BWM;
                BXD = BWP;
                BXE = BWJ;
                BXF = BWN;
                BXG = BWQ;
            } else {
                let BWR = -BOE;
                let BWS = (BOE + HW) + (BWR.ln());
                let BWT = BOF + ((BOF * AC) * (GY / BWR));
                let BWU = D / BOC;
                let BWV = ((BOD * BWU) * AC) / BOC;
                let BWW = AIR + BWU;
                let BWX = AIT + BWV;
                let BWY = -BWU;
                let BWZ = BWY * BWU;
                let BXA = ((BWV * AC) * BWU) + (BWV * BWY);
                BXB = BWS;
                BXC = BWW;
                BXD = BWZ;
                BXE = BWT;
                BXF = BWX;
                BXG = BXA;
            }
            let BXH = ((AXZ + BOC) + (LA * BXB)) - BUI;
            let BXI = (D + (LA * BXC)) - BQK;
            let BXJ = (LA * BXD) - BQL;
            let BXK = BOE + (AIU * BXH);
            let BXL = BOF + ((AIW * BXH) + ((((AYA + BOD) + (BXE * LA)) - BUK) * AIU));
            let BXM = AIR + (AIU * BXI);
            let BXN = AIT + ((AIW * BXI) + (((BXF * LA) - BQR) * AIU));
            let BXO = AIU * BXJ;
            let BXP = (BXK * BWB) - BOT;
            let BXQ = ((BXL * BWB) + (BWE * BXK)) - BOU;
            let BXR = ((BXM * BWB) + (BXK * BWC)) + BOT;
            let BXS = (((BXN * BWB) + (BWE * BXM)) + ((BXL * BWC) + (BWF * BXK))) + BOU;
            let BXT = LA * BXM;
            let BXU = (((BXO * BWB) + (BXT * BWC)) + (BXK * BWD)) - BOT;
            let BXV = BXS * BXR;
            let BXW = GO * BXP;
            let BXX = (BXR * BXR) - (BXW * BXU);
            let BXY = (BXV + BXV) - (((BXQ * GO) * BXU) + ((((((((AIW * BXJ) + (((BXG * LA) - BQS) * AIU)) * BWB) + (BWE * BXO)) + (((BXN * LA) * BWC) + (BWF * BXT))) + ((BXL * BWD) + (BWG * BXK))) - BOU) * BXW));
            let BXZ = -BXP;
            let BYA = BXZ * BXR;
            let BYB = BXY * BXX;
            let BYC = (BXX * BXX) + AYW;
            let BYD = (BYA * BXX) / BYC;
            let BYE = BOC + BYD;
            let BYF = BOD + (((((((BXQ * AC) * BXR) + (BXS * BXZ)) * BXX) + (BXY * BYA)) - ((BYB + BYB) * BYD)) / BYC);
            let BYH;
            let BYI;
            let BYJ;
            let BYK;
            let BYL;
            let BYM;
            if A != 0.0 {
                let BYG = if (BYD.abs()) > NI { 1.0 } else { 0.0 };
                let BYX;
                let BYY;
                let BYZ;
                let BZA;
                let BZB;
                let BZC;
                if BYG != 0.0 {
                    let BYS = AIR * BYE;
                    let BYT = (AIT * BYE) + (BYF * AIR);
                    let BYU = AIH - BYE;
                    let BYV = AII - BYF;
                    let BYW = if BYU < SY { 1.0 } else { 0.0 };
                    let BZL;
                    let BZM;
                    if BYW != 0.0 {
                        let BZD = BYU.exp();
                        let BZE = BYV * BZD;
                        BZL = BZD;
                        BZM = BZE;
                    } else {
                        let BZF = BYU - SY;
                        let BZG = GO * BZF;
                        let BZH = D + (BZF * WD);
                        let BZI = D + (BZG * BZH);
                        let BZJ = XB * (D + (BZF * BZI));
                        let BZK = ((BYV * BZI) + ((((BYV * GO) * BZH) + ((BYV * WD) * BZG)) * BZF)) * XB;
                        BZL = BZJ;
                        BZM = BZK;
                    }
                    let BZN = AJH * BZL;
                    let BZO = (AJJ * BZL) + (BZM * AJH);
                    let BZP = BYT * BYS;
                    let BZQ = (BYS * BYS) - BZN;
                    let BZR = (BZP + BZP) - BZO;
                    let BZS = (AOY * BYS) + BZN;
                    let BZT = ((AOZ * BYS) + (BYT * AOY)) + BZO;
                    let BZU = APC - BZN;
                    let BZV = APD - BZO;
                    let BZW = if BZQ < -5e-3f64 { 1.0 } else { 0.0 };
                    let CAZ;
                    let CBA;
                    let CBB;
                    let CBC;
                    let CBD;
                    let CBE;
                    let CBF;
                    let CBG;
                    let CBH;
                    let CBI;
                    let CBJ;
                    let CBK;
                    let CBL;
                    let CBM;
                    if BZW != 0.0 {
                        let BZX = (BZQ.abs()).sqrt();
                        let BZY = (BZR * ((GX * (if BZQ >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * BZX));
                        let BZZ = GO * BZX;
                        let CAA = BZZ.tan();
                        let CAB = BZZ.cos();
                        let CAC = BZX / CAA;
                        let CAD = (BZY - (((BZY * GO) * (GY / (CAB * CAB))) * CAC)) / CAA;
                        let CAE = (YY * BZS) / BZQ;
                        let CAF = ((BZT * YY) - (BZR * CAE)) / BZQ;
                        let CAG = LA - CAC;
                        let CAH = BZQ + (CAC * CAG);
                        let CAI = CAH * CAE;
                        let CAJ = ((BZR + ((CAD * CAG) + ((CAD * AC) * CAC))) * CAE) + (CAF * CAH);
                        let CAK = LA * CAI;
                        let CAL = D + CAC;
                        let CAM = BZS - (CAK * CAL);
                        let CAN = (CAI * BZU) / BZS;
                        let CAO = (CAM * CAE) + CAN;
                        let CAP = (((BZT - (((CAJ * LA) * CAL) + (CAD * CAK))) * CAE) + (CAF * CAM)) + ((((CAJ * BZU) + (BZV * CAI)) - (BZT * CAN)) / BZS);
                        let CAQ = D - (GO * CAC);
                        let CAR = (CAD * GO) * AC;
                        let CAS = BZS / BZQ;
                        let CAT = CAS * CAQ;
                        let CAU = (((BZT - (BZR * CAS)) / BZQ) * CAQ) + (CAR * CAS);
                        let CAV = CAT + (GO * CAI);
                        let CAW = ((BZU * CAQ) - (BZS * CAV)) / BZQ;
                        let CAX = ((((BZV * CAQ) + (CAR * BZU)) - ((BZT * CAV) + ((CAU + (CAJ * GO)) * BZS))) - (BZR * CAW)) / BZQ;
                        CAZ = BQF;
                        CBA = BZX;
                        CBB = CAC;
                        CBC = CAI;
                        CBD = CAO;
                        CBE = CAT;
                        CBF = CAW;
                        CBG = BQM;
                        CBH = BZY;
                        CBI = CAD;
                        CBJ = CAJ;
                        CBK = CAP;
                        CBL = CAU;
                        CBM = CAX;
                    } else {
                        let CAY = if BZQ > AQJ { 1.0 } else { 0.0 };
                        let CEE;
                        let CEF;
                        let CEG;
                        let CEH;
                        let CEI;
                        let CEJ;
                        let CEK;
                        let CEL;
                        let CEM;
                        let CEN;
                        let CEO;
                        let CEP;
                        let CEQ;
                        let CER;
                        if CAY != 0.0 {
                            let CBO = (BZQ.abs()).sqrt();
                            let CBP = (BZR * ((GX * (if BZQ >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * CBO));
                            let CBQ = (-CBO).exp();
                            let CBR = (CBP * AC) * CBQ;
                            let CBS = D + CBQ;
                            let CBT = D - CBQ;
                            let CBU = (CBO * CBS) / CBT;
                            let CBV = (((CBP * CBS) + (CBR * CBO)) - ((CBR * AC) * CBU)) / CBT;
                            let CBW = (YY * BZS) / BZQ;
                            let CBX = ((BZT * YY) - (BZR * CBW)) / BZQ;
                            let CBY = LA - CBU;
                            let CBZ = BZQ + (CBU * CBY);
                            let CCA = CBZ * CBW;
                            let CCB = ((BZR + ((CBV * CBY) + ((CBV * AC) * CBU))) * CBW) + (CBX * CBZ);
                            let CCC = LA * CCA;
                            let CCD = D + CBU;
                            let CCE = BZS - (CCC * CCD);
                            let CCF = (CCA * BZU) / BZS;
                            let CCG = (CCE * CBW) + CCF;
                            let CCH = (((BZT - (((CCB * LA) * CCD) + (CBV * CCC))) * CBW) + (CBX * CCE)) + ((((CCB * BZU) + (BZV * CCA)) - (BZT * CCF)) / BZS);
                            let CCI = D - (GO * CBU);
                            let CCJ = (CBV * GO) * AC;
                            let CCK = BZS / BZQ;
                            let CCL = CCK * CCI;
                            let CCM = (((BZT - (BZR * CCK)) / BZQ) * CCI) + (CCJ * CCK);
                            let CCN = CCL + (GO * CCA);
                            let CCO = ((BZU * CCI) - (BZS * CCN)) / BZQ;
                            let CCP = ((((BZV * CCI) + (CCJ * BZU)) - ((BZT * CCN) + ((CCM + (CCB * GO)) * BZS))) - (BZR * CCO)) / BZQ;
                            CEE = CBQ;
                            CEF = CBO;
                            CEG = CBU;
                            CEH = CCA;
                            CEI = CCG;
                            CEJ = CCL;
                            CEK = CCO;
                            CEL = CBR;
                            CEM = CBP;
                            CEN = CBV;
                            CEO = CCB;
                            CEP = CCH;
                            CEQ = CCM;
                            CER = CCP;
                        } else {
                            let CCQ = BZQ * ASC;
                            let CCR = BZQ * ASE;
                            let CCS = BZR * ASE;
                            let CCT = D - (BZQ * ASH);
                            let CCU = D - (CCR * CCT);
                            let CCV = UC * (D - (CCQ * CCU));
                            let CCW = ((((BZR * ASC) * CCU) + ((((CCS * CCT) + (((BZR * ASH) * AC) * CCR)) * AC) * CCQ)) * AC) * UC;
                            let CCX = (BZR * CCV) + (CCW * BZQ);
                            let CCY = LA + (BZQ * CCV);
                            let CCZ = BZQ * ASO;
                            let CDA = BZR * ASO;
                            let CDB = BZQ * ASR;
                            let CDC = D - CCZ;
                            let CDD = D - (CDB * CDC);
                            let CDE = UC * (D - (CCZ * CDD));
                            let CDF = (((CDA * CDD) + (((((BZR * ASR) * CDC) + ((CDA * AC) * CDB)) * AC) * CCZ)) * AC) * UC;
                            let CDG = BZS * CDE;
                            let CDH = (BZT * CDE) + (CDF * BZS);
                            let CDI = BZQ * ASZ;
                            let CDJ = ATB * BZQ;
                            let CDK = D - (ATD * BZQ);
                            let CDL = D - (CDJ * CDK);
                            let CDM = ATG * (D - (CDI * CDL));
                            let CDN = BZS * BZS;
                            let CDO = BZT * BZS;
                            let CDP = (BZU * CDE) - (CDN * CDM);
                            let CDQ = ((BZV * CDE) + (CDF * BZU)) - (((CDO + CDO) * CDM) + ((((((BZR * ASZ) * CDL) + (((((BZR * ATB) * CDK) + (((BZR * ATD) * AC) * CDJ)) * AC) * CDI)) * AC) * ATG) * CDN));
                            let CDS = CDR * BZS;
                            let CDT = CDS * CCV;
                            let CDU = ((BZT * CDR) * CCV) + (CCW * CDS);
                            let CDW = CDV * BZU;
                            let CDY = CDX * BZS;
                            let CDZ = CDY * BZS;
                            let CEA = LA - (ATV * BZQ);
                            let CEB = D - (CCR * CEA);
                            let CEC = (CDW * CCV) + (CDZ * CEB);
                            let CED = (((BZV * CDV) * CCV) + (CCW * CDW)) + (((((BZT * CDX) * BZS) + (BZT * CDY)) * CEB) + ((((CCS * CEA) + (((BZR * ATV) * AC) * CCR)) * AC) * CDZ));
                            CEE = BQF;
                            CEF = BQG;
                            CEG = CCY;
                            CEH = CDG;
                            CEI = CDP;
                            CEJ = CDT;
                            CEK = CEC;
                            CEL = BQM;
                            CEM = BQN;
                            CEN = CCX;
                            CEO = CDH;
                            CEP = CDQ;
                            CEQ = CDU;
                            CER = CED;
                        }
                        CAZ = CEE;
                        CBA = CEF;
                        CBB = CEG;
                        CBC = CEH;
                        CBD = CEI;
                        CBE = CEJ;
                        CBF = CEK;
                        CBG = CEL;
                        CBH = CEM;
                        CBI = CEN;
                        CBJ = CEO;
                        CBK = CEP;
                        CBL = CEQ;
                        CBM = CER;
                    }
                    let CBN = if BZQ > AQJ { 1.0 } else { 0.0 };
                    let CFB;
                    let CFC;
                    let CFD;
                    let CFE;
                    if CBN != 0.0 {
                        let CES = LA - CAZ;
                        let CET = D - (CAZ * CES);
                        let CEU = (XQ * BZQ) / CET;
                        let CEV = ((BZR * XQ) - ((((CBG * CES) + ((CBG * AC) * CAZ)) * AC) * CEU)) / CET;
                        let CEW = CEU * CAZ;
                        let CEX = (CEV * CAZ) + (CBG * CEU);
                        let CEY = (CEU.ln()) - CBA;
                        let CEZ = (CEV * (GY / CEU)) - CBH;
                        CFB = CEW;
                        CFC = CEY;
                        CFD = CEX;
                        CFE = CEZ;
                    } else {
                        let CFA = if BZQ < -5e-3f64 { 1.0 } else { 0.0 };
                        let CFW;
                        let CFX;
                        let CFY;
                        let CFZ;
                        if CFA != 0.0 {
                            let CFG = GO * CBA;
                            let CFH = CFG.sin();
                            let CFI = CFH * CFH;
                            let CFJ = ((CBH * GO) * (CFG.cos())) * CFH;
                            let CFK = (-BZQ) / CFI;
                            let CFL = ((BZR * AC) - ((CFJ + CFJ) * CFK)) / CFI;
                            let CFM = CFK.ln();
                            let CFN = CFL * (GY / CFK);
                            CFW = CFK;
                            CFX = CFM;
                            CFY = CFL;
                            CFZ = CFN;
                        } else {
                            let CFO = BZQ * WD;
                            let CFP = ATB * BZQ;
                            let CFQ = D - (AVN * BZQ);
                            let CFR = D - (CFP * CFQ);
                            let CFS = XQ - (CFO * CFR);
                            let CFT = (((BZR * WD) * CFR) + (((((BZR * ATB) * CFQ) + (((BZR * AVN) * AC) * CFP)) * AC) * CFO)) * AC;
                            let CFU = CFS.ln();
                            let CFV = CFT * (GY / CFS);
                            CFW = CFS;
                            CFX = CFU;
                            CFY = CFT;
                            CFZ = CFV;
                        }
                        CFB = CFW;
                        CFC = CFX;
                        CFD = CFY;
                        CFE = CFZ;
                    }
                    let CFF = if ((AVB * BYS) + CBB) > B { 1.0 } else { 0.0 };
                    let CGV;
                    let CGW;
                    let CGX;
                    let CGY;
                    let CGZ;
                    let CHA;
                    if CFF != 0.0 {
                        let CGA = BYS + CBB;
                        let CGB = BYT + CBI;
                        let CGC = AIR + CBC;
                        let CGD = AIT + CBJ;
                        CGV = CGA;
                        CGW = CGC;
                        CGX = CBD;
                        CGY = CGB;
                        CGZ = CGD;
                        CHA = CBK;
                    } else {
                        let CGE = BYS - CBB;
                        let CGF = D / CGE;
                        let CGG = (((BYT - CBI) * CGF) * AC) / CGE;
                        let CGH = CBC - AIR;
                        let CGI = CBJ - AIT;
                        let CGJ = BZN - CFB;
                        let CGK = CGJ * CGF;
                        let CGL = ((BZO - CFD) * CGF) + (CGG * CGJ);
                        let CGM = ((CGH * CGK) - BZN) - (CBE * CFB);
                        let CGN = CGM * CGF;
                        let CGO = (((((CGI * CGK) + (CGL * CGH)) - BZO) - ((CBL * CFB) + (CFD * CBE))) * CGF) + (CGG * CGM);
                        let CGP = LA * CGH;
                        let CGQ = CBL * CBE;
                        let CGR = CBF + (CBE * CBE);
                        let CGS = (((CBD * CGK) + (CGP * CGN)) + BZN) - (CGR * CFB);
                        let CGT = CGS * CGF;
                        let CGU = ((((((CBK * CGK) + (CGL * CBD)) + (((CGI * LA) * CGN) + (CGO * CGP))) + BZO) - (((CBM + (CGQ + CGQ)) * CFB) + (CFD * CGR))) * CGF) + (CGG * CGS);
                        CGV = CGK;
                        CGW = CGN;
                        CGX = CGT;
                        CGY = CGL;
                        CGZ = CGO;
                        CHA = CGU;
                    }
                    let CHB = if CGV > B { 1.0 } else { 0.0 };
                    let CHV;
                    let CHW;
                    let CHX;
                    let CHY;
                    let CHZ;
                    let CIA;
                    if CHB != 0.0 {
                        let CHC = CGV.ln();
                        let CHD = CGY * (GY / CGV);
                        let CHE = D / CGV;
                        let CHF = ((CGY * CHE) * AC) / CGV;
                        let CHG = CGW * CHE;
                        let CHH = (CGZ * CHE) + (CHF * CGW);
                        let CHI = CHH * CHG;
                        let CHJ = (CGX * CHE) - (CHG * CHG);
                        let CHK = ((CHA * CHE) + (CHF * CGX)) - (CHI + CHI);
                        CHV = CHC;
                        CHW = CHG;
                        CHX = CHJ;
                        CHY = CHD;
                        CHZ = CHH;
                        CIA = CHK;
                    } else {
                        let CHL = -BYS;
                        let CHM = (BYS + HW) + (CHL.ln());
                        let CHN = BYT + ((BYT * AC) * (GY / CHL));
                        let CHO = D / BYE;
                        let CHP = ((BYF * CHO) * AC) / BYE;
                        let CHQ = AIR + CHO;
                        let CHR = AIT + CHP;
                        let CHS = -CHO;
                        let CHT = CHS * CHO;
                        let CHU = ((CHP * AC) * CHO) + (CHP * CHS);
                        CHV = CHM;
                        CHW = CHQ;
                        CHX = CHT;
                        CHY = CHN;
                        CHZ = CHR;
                        CIA = CHU;
                    }
                    let CIB = ((AXZ + BYE) + (LA * CHV)) - CFC;
                    let CIC = (D + (LA * CHW)) - CBE;
                    let CID = (LA * CHX) - CBF;
                    let CIE = BYS + (AIU * CIB);
                    let CIF = BYT + ((AIW * CIB) + ((((AYA + BYF) + (CHY * LA)) - CFE) * AIU));
                    let CIG = AIR + (AIU * CIC);
                    let CIH = AIT + ((AIW * CIC) + (((CHZ * LA) - CBL) * AIU));
                    let CII = AIU * CID;
                    let CIJ = (CIE * CGV) - BZN;
                    let CIK = ((CIF * CGV) + (CGY * CIE)) - BZO;
                    let CIL = ((CIG * CGV) + (CIE * CGW)) + BZN;
                    let CIM = (((CIH * CGV) + (CGY * CIG)) + ((CIF * CGW) + (CGZ * CIE))) + BZO;
                    let CIN = LA * CIG;
                    let CIO = (((CII * CGV) + (CIN * CGW)) + (CIE * CGX)) - BZN;
                    let CIP = CIM * CIL;
                    let CIQ = GO * CIJ;
                    let CIR = (CIL * CIL) - (CIQ * CIO);
                    let CIS = (CIP + CIP) - (((CIK * GO) * CIO) + ((((((((AIW * CID) + (((CIA * LA) - CBM) * AIU)) * CGV) + (CGY * CII)) + (((CIH * LA) * CGW) + (CGZ * CIN))) + ((CIF * CGX) + (CHA * CIE))) - BZO) * CIQ));
                    let CIT = -CIJ;
                    let CIU = CIT * CIL;
                    let CIV = CIS * CIR;
                    let CIW = (CIR * CIR) + AYW;
                    let CIX = (CIU * CIR) / CIW;
                    let CIY = BYE + CIX;
                    let CIZ = BYF + (((((((CIK * AC) * CIL) + (CIM * CIT)) * CIR) + (CIS * CIU)) - ((CIV + CIV) * CIX)) / CIW);
                    BYX = CIY;
                    BYY = CAZ;
                    BYZ = CBA;
                    BZA = CIZ;
                    BZB = CBG;
                    BZC = CBH;
                } else {
                    BYX = BYE;
                    BYY = BQF;
                    BYZ = BQG;
                    BZA = BYF;
                    BZB = BQM;
                    BZC = BQN;
                }
                BYH = BYX;
                BYI = BYY;
                BYJ = BYZ;
                BYK = BZA;
                BYL = BZB;
                BYM = BZC;
            } else {
                BYH = BYE;
                BYI = BQF;
                BYJ = BQG;
                BYK = BYF;
                BYL = BQM;
                BYM = BQN;
            }
            let BYN = AIR * BYH;
            let BYO = (AIT * BYH) + (BYK * AIR);
            let BYP = AIH - BYH;
            let BYQ = AII - BYK;
            let BYR = if BYP < SY { 1.0 } else { 0.0 };
            let CJI;
            let CJJ;
            if BYR != 0.0 {
                let CJA = BYP.exp();
                let CJB = BYQ * CJA;
                CJI = CJA;
                CJJ = CJB;
            } else {
                let CJC = BYP - SY;
                let CJD = GO * CJC;
                let CJE = D + (CJC * WD);
                let CJF = D + (CJD * CJE);
                let CJG = XB * (D + (CJC * CJF));
                let CJH = ((BYQ * CJF) + ((((BYQ * GO) * CJE) + ((BYQ * WD) * CJD)) * CJC)) * XB;
                CJI = CJG;
                CJJ = CJH;
            }
            let CJK = AJH * CJI;
            let CJL = (AJJ * CJI) + (CJJ * AJH);
            let CJM = BYO * BYN;
            let CJN = (BYN * BYN) - CJK;
            let CJO = (CJM + CJM) - CJL;
            let CJP = if CJK <= B { 1.0 } else { 0.0 };
            let CJW;
            let CJX;
            let CJY;
            let CJZ;
            let CKA;
            let CKB;
            if CJP != 0.0 {
                let CJR = CJQ - BYN;
                let CJS = BYO * AC;
                let CJT = CJR / AIU;
                let CJU = (CJS - (AIW * CJT)) / AIU;
                CJW = CJT;
                CJX = CJQ;
                CJY = CJR;
                CJZ = CJU;
                CKA = AFD;
                CKB = CJS;
            } else {
                let CJV = if CJN < -5e-3f64 { 1.0 } else { 0.0 };
                let CKN;
                let CKO;
                let CKP;
                let CKQ;
                let CKR;
                let CKS;
                if CJV != 0.0 {
                    let CKF = (CJN.abs()).sqrt();
                    let CKG = (CJO * ((GX * (if CJN >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * CKF));
                    let CKH = GO * CKF;
                    let CKI = CKH.tan();
                    let CKJ = CKH.cos();
                    let CKK = CKF / CKI;
                    let CKL = (CKG - (((CKG * GO) * (GY / (CKJ * CKJ))) * CKK)) / CKI;
                    CKN = CKK;
                    CKO = BYI;
                    CKP = CKF;
                    CKQ = CKL;
                    CKR = BYL;
                    CKS = CKG;
                } else {
                    let CKM = if CJN > AQJ { 1.0 } else { 0.0 };
                    let CLI;
                    let CLJ;
                    let CLK;
                    let CLL;
                    let CLM;
                    let CLN;
                    if CKM != 0.0 {
                        let CKU = (CJN.abs()).sqrt();
                        let CKV = (CJO * ((GX * (if CJN >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * CKU));
                        let CKW = (-CKU).exp();
                        let CKX = (CKV * AC) * CKW;
                        let CKY = D + CKW;
                        let CKZ = D - CKW;
                        let CLA = (CKU * CKY) / CKZ;
                        let CLB = (((CKV * CKY) + (CKX * CKU)) - ((CKX * AC) * CLA)) / CKZ;
                        CLI = CLA;
                        CLJ = CKW;
                        CLK = CKU;
                        CLL = CLB;
                        CLM = CKX;
                        CLN = CKV;
                    } else {
                        let CLC = CJN * UC;
                        let CLD = CJN * ASC;
                        let CLE = D - (CJN * ASE);
                        let CLF = D - (CLD * CLE);
                        let CLG = ((CJO * UC) * CLF) + (((((CJO * ASC) * CLE) + (((CJO * ASE) * AC) * CLD)) * AC) * CLC);
                        let CLH = LA + (CLC * CLF);
                        CLI = CLH;
                        CLJ = BYI;
                        CLK = BYJ;
                        CLL = CLG;
                        CLM = BYL;
                        CLN = BYM;
                    }
                    CKN = CLI;
                    CKO = CLJ;
                    CKP = CLK;
                    CKQ = CLL;
                    CKR = CLM;
                    CKS = CLN;
                }
                let CKT = if ((AVB * BYN) + CKN) > B { 1.0 } else { 0.0 };
                let CLT;
                let CLU;
                let CLV;
                let CLW;
                let CLX;
                let CLY;
                if CKT != 0.0 {
                    let CLO = BYN + CKN;
                    let CLP = BYO + CKQ;
                    let CLR = if (CJK * BYN) < (((CLQ * BYN) * BYN) * CLO) { 1.0 } else { 0.0 };
                    let CMH;
                    let CMI;
                    let CMJ;
                    let CMK;
                    let CML;
                    let CMM;
                    if CLR != 0.0 {
                        let CLZ = CJK / CLO;
                        let CMA = (CJL - (CLP * CLZ)) / CLO;
                        let CMB = CLZ + CJQ;
                        let CMC = CMB - BYN;
                        let CMD = CMA - BYO;
                        let CME = CMC / AIU;
                        let CMF = (CMD - (AIW * CME)) / AIU;
                        CMH = CME;
                        CMI = CMB;
                        CMJ = CMC;
                        CMK = CMF;
                        CML = CMA;
                        CMM = CMD;
                    } else {
                        let CMG = if CJN > AQJ { 1.0 } else { 0.0 };
                        let CMT;
                        let CMU;
                        if CMG != 0.0 {
                            let CMN = LA - CKO;
                            let CMO = D - (CKO * CMN);
                            let CMP = (XQ * CJN) / CMO;
                            let CMQ = (CMP.ln()) - CKP;
                            let CMR = ((((CJO * XQ) - ((((CKR * CMN) + ((CKR * AC) * CKO)) * AC) * CMP)) / CMO) * (GY / CMP)) - CKS;
                            CMT = CMQ;
                            CMU = CMR;
                        } else {
                            let CMS = if CJN < -5e-3f64 { 1.0 } else { 0.0 };
                            let CNP;
                            let CNQ;
                            if CMS != 0.0 {
                                let CNB = GO * CKP;
                                let CNC = CNB.sin();
                                let CND = CNC * CNC;
                                let CNE = ((CKS * GO) * (CNB.cos())) * CNC;
                                let CNF = (-CJN) / CND;
                                let CNG = CNF.ln();
                                let CNH = (((CJO * AC) - ((CNE + CNE) * CNF)) / CND) * (GY / CNF);
                                CNP = CNG;
                                CNQ = CNH;
                            } else {
                                let CNI = CJN * WD;
                                let CNJ = ATB * CJN;
                                let CNK = D - (AVN * CJN);
                                let CNL = D - (CNJ * CNK);
                                let CNM = XQ - (CNI * CNL);
                                let CNN = CNM.ln();
                                let CNO = ((((CJO * WD) * CNL) + (((((CJO * ATB) * CNK) + (((CJO * AVN) * AC) * CNJ)) * AC) * CNI)) * AC) * (GY / CNM);
                                CNP = CNN;
                                CNQ = CNO;
                            }
                            CMT = CNP;
                            CMU = CNQ;
                        }
                        let CMV = ((AXZ + BYH) + (LA * (CLO.ln()))) - CMT;
                        let CMW = ((AYA + BYK) + ((CLP * (GY / CLO)) * LA)) - CMU;
                        let CMX = AIU * CMV;
                        let CMY = (AIW * CMV) + (CMW * AIU);
                        let CMZ = BYN + CMX;
                        let CNA = BYO + CMY;
                        CMH = CMV;
                        CMI = CMZ;
                        CMJ = CMX;
                        CMK = CMW;
                        CML = CNA;
                        CMM = CMY;
                    }
                    CLT = CMH;
                    CLU = CMI;
                    CLV = CMJ;
                    CLW = CMK;
                    CLX = CML;
                    CLY = CMM;
                } else {
                    let CLS = if CJN > AQJ { 1.0 } else { 0.0 };
                    let CNV;
                    let CNW;
                    if CLS != 0.0 {
                        let CNR = (BYH - AIH) - CKP;
                        let CNS = (BYK - AII) - CKS;
                        let CNT = if CNR < SY { 1.0 } else { 0.0 };
                        let CON;
                        let COO;
                        if CNT != 0.0 {
                            let COF = CNR.exp();
                            let COG = CNS * COF;
                            CON = COF;
                            COO = COG;
                        } else {
                            let COH = CNR - SY;
                            let COI = GO * COH;
                            let COJ = D + (COH * WD);
                            let COK = D + (COI * COJ);
                            let COL = XB * (D + (COH * COK));
                            let COM = ((CNS * COK) + ((((CNS * GO) * COJ) + ((CNS * WD) * COI)) * COH)) * XB;
                            CON = COL;
                            COO = COM;
                        }
                        let COP = CON / AJH;
                        let COQ = XQ * CJN;
                        let COR = LA - CKO;
                        let COS = D - (CKO * COR);
                        let COT = (COQ * COP) / COS;
                        let COU = ((((CJO * XQ) * COP) + (((COO - (AJJ * COP)) / AJH) * COQ)) - ((((CKR * COR) + ((CKR * AC) * CKO)) * AC) * COT)) / COS;
                        CNV = COT;
                        CNW = COU;
                    } else {
                        let CNU = if CJN < -5e-3f64 { 1.0 } else { 0.0 };
                        let CPI;
                        let CPJ;
                        if CNU != 0.0 {
                            let COV = GO * CKP;
                            let COW = COV.sin();
                            let COX = COW * COW;
                            let COY = ((CKS * GO) * (COV.cos())) * COW;
                            let COZ = (-CJN) / COX;
                            let CPA = COZ / CJK;
                            let CPB = ((((CJO * AC) - ((COY + COY) * COZ)) / COX) - (CJL * CPA)) / CJK;
                            CPI = CPA;
                            CPJ = CPB;
                        } else {
                            let CPC = CJN * WD;
                            let CPD = ATB * CJN;
                            let CPE = D - (AVN * CJN);
                            let CPF = D - (CPD * CPE);
                            let CPG = (XQ - (CPC * CPF)) / CJK;
                            let CPH = (((((CJO * WD) * CPF) + (((((CJO * ATB) * CPE) + (((CJO * AVN) * AC) * CPD)) * AC) * CPC)) * AC) - (CJL * CPG)) / CJK;
                            CPI = CPG;
                            CPJ = CPH;
                        }
                        CNV = CPI;
                        CNW = CPJ;
                    }
                    let CNX = D - CNV;
                    let CNY = (BYN - CKN) / CNX;
                    let CNZ = ((BYO - CKQ) - ((CNW * AC) * CNY)) / CNX;
                    let COA = CNY + CJQ;
                    let COB = COA - BYN;
                    let COC = CNZ - BYO;
                    let COD = COB / AIU;
                    let COE = (COC - (AIW * COD)) / AIU;
                    CLT = COD;
                    CLU = COA;
                    CLV = COB;
                    CLW = COE;
                    CLX = CNZ;
                    CLY = COC;
                }
                CJW = CLT;
                CJX = CLU;
                CJY = CLV;
                CJZ = CLW;
                CKA = CLX;
                CKB = CLY;
            }
            let CKC = AIP - CJW;
            let CKD = AIQ - CJZ;
            let CKE = if CKC < SY { 1.0 } else { 0.0 };
            let CPS;
            let CPT;
            if CKE != 0.0 {
                let CPK = CKC.exp();
                let CPL = CKD * CPK;
                CPS = CPK;
                CPT = CPL;
            } else {
                let CPM = CKC - SY;
                let CPN = GO * CPM;
                let CPO = D + (CPM * WD);
                let CPP = D + (CPN * CPO);
                let CPQ = XB * (D + (CPM * CPP));
                let CPR = ((CKD * CPP) + ((((CKD * GO) * CPO) + ((CKD * WD) * CPN)) * CPM)) * XB;
                CPS = CPQ;
                CPT = CPR;
            }
            let CPU = AJH * CPS;
            let CPV = (AJJ * CPS) + (CPT * AJH);
            let CPX = if CJX > CPW { 1.0 } else { 0.0 };
            let CQJ;
            let CQK;
            let CQL;
            let CQM;
            let CQN;
            let CQO;
            let CQP;
            let CQQ;
            if CPX != 0.0 {
                let CPY = CJK * AIX;
                let CPZ = (CJL * AIX) + (AIY * CJK);
                let CQA = CPU * AIZ;
                let CQB = (CPV * AIZ) + (AJA * CPU);
                let CQC = CPY + (LA * BYN);
                let CQD = CPZ + (BYO * LA);
                let CQE = CQA + (LA * CJY);
                let CQF = CQB + (CKB * LA);
                let CQG = ((LA * CJX) + CPY) + CQA;
                let CQH = ((CKA * LA) + CPZ) + CQB;
                let CQI = if (CJN.abs()) > AQJ { 1.0 } else { 0.0 };
                let CRS;
                let CRT;
                if CQI != 0.0 {
                    let CQW = LA * (BYH + LA);
                    let CQX = LA * (CJW + LA);
                    let CQY = ((CQC * CQE) + (CQW * CQE)) + (CQX * CQC);
                    let CRA = CQZ * CJN;
                    let CRB = CJX * CQY;
                    let CRC = (CRA * CQG) / CRB;
                    let CRD = ((((CJO * CQZ) * CQG) + (CQH * CRA)) - (((CKA * CQY) + (((((CQD * CQE) + (CQF * CQC)) + (((BYK * LA) * CQE) + (CQF * CQW))) + (((CJZ * LA) * CQC) + (CQD * CQX))) * CJX)) * CRC)) / CRB;
                    CRS = CRC;
                    CRT = CRD;
                } else {
                    let CRE = CJN * ASO;
                    let CRF = CJO * ASO;
                    let CRG = CJN * ASR;
                    let CRH = D - CRE;
                    let CRI = D - (CRG * CRH);
                    let CRJ = UC * (D - (CRE * CRI));
                    let CRK = CQC * CQE;
                    let CRL = CRK * CJX;
                    let CRM = D + (CJX * CRJ);
                    let CRN = ((CQC * CJK) + (CQE * CPU)) + (CRL * CRM);
                    let CRO = CJK * CPU;
                    let CRP = CJX * CRN;
                    let CRQ = (CRO * CQG) / CRP;
                    let CRR = (((((CJL * CPU) + (CPV * CJK)) * CQG) + (CQH * CRO)) - (((CKA * CRN) + (((((CQD * CJK) + (CJL * CQC)) + ((CQF * CPU) + (CPV * CQE))) + ((((((CQD * CQE) + (CQF * CQC)) * CJX) + (CKA * CRK)) * CRM) + (((CKA * CRJ) + (((((CRF * CRI) + (((((CJO * ASR) * CRH) + ((CRF * AC) * CRG)) * AC) * CRE)) * AC) * UC) * CJX)) * CRL))) * CJX)) * CRQ)) / CRP;
                    CRS = CRQ;
                    CRT = CRR;
                }
                CQJ = CRS;
                CQK = CQG;
                CQL = CQC;
                CQM = CQE;
                CQN = CRT;
                CQO = CQH;
                CQP = CQD;
                CQQ = CQF;
            } else {
                CQJ = B;
                CQK = B;
                CQL = B;
                CQM = B;
                CQN = AFD;
                CQO = AFD;
                CQP = AFD;
                CQQ = AFD;
            }
            let CQR = CJX.ln();
            let CQS = CKA * (GY / CJX);
            let CQT = BYN / LA;
            let CQU = BYO / LA;
            let CQV = if CQT < SY { 1.0 } else { 0.0 };
            let CRY;
            let CRZ;
            if CQV != 0.0 {
                let CRU = CQT.exp();
                let CRV = D + CRU;
                let CRW = CRV.ln();
                let CRX = (CQU * CRU) * (GY / CRV);
                CRY = CRW;
                CRZ = CRX;
            } else {
                CRY = CQT;
                CRZ = CQU;
            }
            let CSA = LA * CRY;
            let CSB = CRZ * LA;
            let CSC = CJY / LA;
            let CSD = CKB / LA;
            let CSE = if CSC < SY { 1.0 } else { 0.0 };
            let CSJ;
            let CSK;
            if CSE != 0.0 {
                let CSF = CSC.exp();
                let CSG = D + CSF;
                let CSH = CSG.ln();
                let CSI = (CSD * CSF) * (GY / CSG);
                CSJ = CSH;
                CSK = CSI;
            } else {
                CSJ = CSC;
                CSK = CSD;
            }
            let CSL = LA * CSJ;
            let CSM = CSK * LA;
            let CSN = CSL - CJY;
            let CSO = CSM - CKB;
            let CSP = CSA - BYN;
            let CSQ = CSB - BYO;
            let CST = (CSR * CSA) + (CSS * CSN);
            let CSU = (CSB * CSR) + (CSO * CSS);
            let CSV = (CSR * CSL) + (CSS * CSP);
            let CSW = (CSM * CSR) + (CSQ * CSS);
            let CSX = CSA + CSL;
            let CSY = CJX / CSX;
            let CSZ = (CKA - ((CSB + CSM) * CSY)) / CSX;
            let CTA = CSA * JO;
            let CTB = (JJ * CT).exp();
            let CTC = (ER * JJ) * CTB;
            let CTD = CTA * CTB;
            let CTE = ((CSB * JO) * CTB) + Lanes([(CTC * CTA), 0.0, 0.0, 0.0, 0.0]);
            let CTF = CSL * JR;
            let CTG = CTF * CTB;
            let CTH = ((CSM * JR) * CTB) + Lanes([(CTC * CTF), 0.0, 0.0, 0.0, 0.0]);
            let CTJ = CSN + (CTI * CSP);
            let CTK = CU * CTJ;
            let CTL = Lanes([(ES * CTJ), 0.0, 0.0, 0.0, 0.0]) + ((CSO + (CSQ * CTI)) * CU);
            let CTM = D + CTK;
            let CTN = CTL * CTM;
            let CTO = ((CTM * CTM) + NI).sqrt();
            let CTQ = CTL * CTP;
            let CTR = D + (CTP * CTK);
            let CTS = CTQ * CTR;
            let CTT = ((CTR * CTR) + NI).sqrt();
            let CTU = GO * (CTR + CTT);
            let CTV = (GO * (CTM + CTO)) / CTU;
            let CTW = (((CTL + ((CTN + CTN) * (GY / (GX * CTO)))) * GO) - (((CTQ + ((CTS + CTS) * (GY / (GX * CTT)))) * GO) * CTV)) / CTU;
            let CTZ = (D + (CTX * CSN)) + (CTY * CSP);
            let CUA = CV * CTZ;
            let CUB = -CW;
            let CUC = EU * AC;
            let CUF = (D + ((CSA * CSY) * CUD)) + ((CSL * CSY) * CUE);
            let CUG = CUF.ln();
            let CUH = (CUB * CUG).exp();
            let CUI = CUA * CUH;
            let CUJ = ((Lanes([(ET * CTZ), 0.0, 0.0, 0.0, 0.0]) + (((CSO * CTX) + (CSQ * CTY)) * CV)) * CUH) + (((Lanes([(CUC * CUG), 0.0, 0.0, 0.0, 0.0]) + ((((((CSB * CSY) + (CSZ * CSA)) * CUD) + (((CSM * CSY) + (CSZ * CSL)) * CUE)) * (GY / CUF)) * CUB)) * CUH) * CUA);
            let CUM;
            let CUN;
            if CUK != 0.0 {
                CUM = D;
                CUN = AFD;
            } else {
                let CWE;
                let CWF;
                if CUL != 0.0 {
                    let CVT = CJX + CVS;
                    let CVV = (CVU * (CVT.ln())).exp();
                    let CVX = D - (CVW * CVV);
                    let CVY = ((((CKA * (GY / CVT)) * CVU) * CVV) * CVW) * AC;
                    CWE = CVX;
                    CWF = CVY;
                } else {
                    let CVZ = CJX + CVS;
                    let CWA = (CVU * (CVZ.ln())).exp();
                    let CWB = D + (CVW * CWA);
                    let CWC = D / CWB;
                    let CWD = ((((((CKA * (GY / CVZ)) * CVU) * CWA) * CVW) * CWC) * AC) / CWB;
                    CWE = CWC;
                    CWF = CWD;
                }
                CUM = CWE;
                CUN = CWF;
            }
            let CUO = (CX * AFH) * GO;
            let CUQ = D - (CUP * RW);
            let CUR = (RY * CUP) * AC;
            let CUS = CUR * CUQ;
            let CUT = ((CUQ * CUQ) + NI).sqrt();
            let CUU = CUQ + CUT;
            let CUV = CUO * CUU;
            let CUW = (CUR + ((CUS + CUS) * (GY / (GX * CUT)))) * CUO;
            let CUX = (((Lanes([(EV * AFH), 0.0, 0.0, 0.0, 0.0]) + (AFN * CX)) * GO) * CUU) + Lanes([CUW[0], CUW[1], CUW[2], CUW[3], 0.0]);
            let CUZ = (CJX * CUM) + CUY;
            let CVA = CUV * CUZ;
            let CVB = (CUX * CUZ) + (((CKA * CUM) + (CUN * CJX)) * CUV);
            let CVC = (CZ * CST) + CPW;
            let CVD = CVC.ln();
            let CVE = (CY * CVD).exp();
            let CVF = ((D + CVE) + CUI) + (DA * CVA);
            let CVG = (CZ * CSV) + CPW;
            let CVH = CVG.ln();
            let CVI = (CY * CVH).exp();
            let CVJ = ((D + CVI) + CUI) + (DB * CVA);
            let CVK = CTD + CTG;
            let CVL = CTD / CVF;
            let CVM = CTG / CVJ;
            let CVN = CVL + CVM;
            let CVO = (CTV * CVK) / CVN;
            let CVP = (((CTW * CVK) + ((CTE + CTH) * CTV)) - ((((CTE - (((((Lanes([(EW * CVD), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * CST), 0.0, 0.0, 0.0, 0.0]) + (CSU * CZ)) * (GY / CVC)) * CY)) * CVE) + CUJ) + (Lanes([(EY * CVA), 0.0, 0.0, 0.0, 0.0]) + (CVB * DA))) * CVL)) / CVF) + ((CTH - (((((Lanes([(EW * CVH), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * CSV), 0.0, 0.0, 0.0, 0.0]) + (CSW * CZ)) * (GY / CVG)) * CY)) * CVI) + CUJ) + (Lanes([(EZ * CVA), 0.0, 0.0, 0.0, 0.0]) + (CVB * DB))) * CVM)) / CVJ)) * CVO)) / CVN;
            let CVR = if (AKT.abs()) > CVQ { 1.0 } else { 0.0 };
            let CXN;
            let CXO;
            let CXP;
            let CXQ;
            let CXR;
            let CXS;
            let CXT;
            let CXU;
            let CXV;
            let CXW;
            let CXX;
            let CXY;
            if CVR != 0.0 {
                let CWG = if AKT > B { 1.0 } else { 0.0 };
                let CYX;
                let CYY;
                let CYZ;
                let CZA;
                let CZB;
                let CZC;
                if CWG != 0.0 {
                    let CYB = (-AKT).exp();
                    let CYC = (AKU * AC) * CYB;
                    let CYD = D - CYB;
                    let CYE = AKT / CYD;
                    let CYF = (AKU - ((CYC * AC) * CYE)) / CYD;
                    let CYG = CYB * CYE;
                    let CYH = (CYC * CYE) + (CYF * CYB);
                    let CYI = CJX * CYE;
                    let CYJ = AJH / CYI;
                    let CYK = ((CYJ.ln()) - HW) + ALA;
                    let CYL = (((AJJ - (((CKA * CYE) + (CYF * CJX)) * CYJ)) / CYI) * (GY / CYJ)) + ALB;
                    CYX = CYE;
                    CYY = CYG;
                    CYZ = CYK;
                    CZA = CYF;
                    CZB = CYH;
                    CZC = CYL;
                } else {
                    let CYM = AKT.exp();
                    let CYN = CYM - D;
                    let CYO = AKT / CYN;
                    let CYP = (AKU * CYM) * CYO;
                    let CYQ = (AKU - CYP) / CYN;
                    let CYR = CYM * CYO;
                    let CYS = CYP + (CYQ * CYM);
                    let CYT = CJX * CYO;
                    let CYU = AJH / CYT;
                    let CYV = ((CYU.ln()) - HW) + ALE;
                    let CYW = (((AJJ - (((CKA * CYO) + (CYQ * CJX)) * CYU)) / CYT) * (GY / CYU)) + ALF;
                    CYX = CYR;
                    CYY = CYO;
                    CYZ = CYV;
                    CZA = CYS;
                    CZB = CYQ;
                    CZC = CYW;
                }
                let CZD = (D - CYX) - ALC;
                let CZE = AJC * CZD;
                let CZF = (-AKT) / CZE;
                let CZG = ((AKU * AC) - (((AJD * CZD) + (((CZA * AC) - ALD) * AJC)) * CZF)) / CZE;
                let CZH = (D - CYY) + AKY;
                let CZI = AJC * CZH;
                let CZJ = AKT / CZI;
                let CZK = (AKU - (((AJD * CZH) + (((CZB * AC) + AKZ) * AJC)) * CZJ)) / CZI;
                let CZL = ((CYY * AIZ) + GO) / CZJ;
                let CZM = ((CYX * AIX) + GO) / CZF;
                let CZN = CZL - CZM;
                let CZO = AKT / CZN;
                let CZP = (AKU - ((((((CZB * AIZ) + (AJA * CYY)) - (CZK * CZL)) / CZJ) - ((((CZA * AIX) + (AIY * CYX)) - (CZG * CZM)) / CZF)) * CZO)) / CZN;
                CXN = CZO;
                CXO = CYZ;
                CXP = CYX;
                CXQ = CZF;
                CXR = CYY;
                CXS = CZJ;
                CXT = CZP;
                CXU = CZC;
                CXV = CZA;
                CXW = CZG;
                CXX = CZB;
                CXY = CZK;
            } else {
                let CWI = CWH * AKV;
                let CWJ = AKX * CWH;
                let CWK = GO * AKT;
                let CWL = AKU * GO;
                let CWM = (D + CWK) + CWI;
                let CWN = CWL + CWJ;
                let CWO = (D - CWK) + CWI;
                let CWP = (CWL * AC) + CWJ;
                let CWQ = UC * CWK;
                let CWR = CWL * UC;
                let CWS = (GO + AIZ) + CWQ;
                let CWT = AJC * CWS;
                let CWU = D / CWT;
                let CWV = ((((AJD * CWS) + ((AJA + CWR) * AJC)) * CWU) * AC) / CWT;
                let CWW = (GO + AIX) - CWQ;
                let CWX = AJC * CWW;
                let CWY = D / CWX;
                let CWZ = ((((AJD * CWW) + ((AIY - CWR) * AJC)) * CWY) * AC) / CWX;
                let CXA = D - (GO * CWI);
                let CXB = CJX * CXA;
                let CXC = AJH / CXB;
                let CXD = ((CXC.ln()) - HW) + (GO * (ALA + ALE));
                let CXE = (((AJJ - (((CKA * CXA) + (((CWJ * GO) * AC) * CJX)) * CXC)) / CXB) * (GY / CXC)) + ((ALB + ALF) * GO);
                let CXF = AIR * AIU;
                let CXG = (XS * AJC) / CXF;
                let CXH = AIX - AIZ;
                let CXI = AJC * CXH;
                let CXJ = WD * (CTP - (YY * AJC));
                let CXK = (((XQ - (ZD * AJC)) + CXG) + (CXI * AKT)) + (CXJ * AKV);
                let CXL = -1.2e1f64 / CXK;
                let CXM = (((((((AJD * ZD) * AC) + (((AJD * XS) - (((AIT * AIU) + (AIW * AIR)) * CXG)) / CXF)) + ((((AJD * CXH) + ((AIY - AJA) * AJC)) * AKT) + (AKU * CXI))) + (((((AJD * YY) * AC) * WD) * AKV) + (AKX * CXJ))) * CXL) * AC) / CXK;
                CXN = CXL;
                CXO = CXD;
                CXP = CWM;
                CXQ = CWU;
                CXR = CWO;
                CXS = CWY;
                CXT = CXM;
                CXU = CXE;
                CXV = CWN;
                CXW = CWV;
                CXX = CWP;
                CXY = CWZ;
            }
            let CXZ = D / CXN;
            let CYA = ((CXT * CXZ) * AC) / CXN;
            let DAC;
            let DAD;
            let DAE;
            let DAF;
            let DAG;
            let DAH;
            if CPX != 0.0 {
                let CZR = CZQ + CSA;
                let CZS = (CZQ * CSA) / CZR;
                let CZT = ((CSB * CZQ) - (CSB * CZS)) / CZR;
                let CZV = if CZU < B { 1.0 } else { 0.0 };
                let DAP;
                let DAQ;
                if CZV != 0.0 {
                    let DAK = D - (CZU * CZS);
                    let DAL = D / DAK;
                    let DAM = ((((CZT * CZU) * AC) * DAL) * AC) / DAK;
                    DAP = DAL;
                    DAQ = DAM;
                } else {
                    let DAN = CZT * CZU;
                    let DAO = D + (CZU * CZS);
                    DAP = DAO;
                    DAQ = DAN;
                }
                let DAR = CZQ + CSL;
                let DAS = (CZQ * CSL) / DAR;
                let DAT = ((CSM * CZQ) - (CSM * DAS)) / DAR;
                let DAV = if DAU < B { 1.0 } else { 0.0 };
                let DBB;
                let DBC;
                if DAV != 0.0 {
                    let DAW = D - (DAU * DAS);
                    let DAX = D / DAW;
                    let DAY = ((((DAT * DAU) * AC) * DAX) * AC) / DAW;
                    DBB = DAX;
                    DBC = DAY;
                } else {
                    let DAZ = DAT * DAU;
                    let DBA = D + (DAU * DAS);
                    DBB = DBA;
                    DBC = DAZ;
                }
                let DBD = CQL * CQM;
                let DBE = (CQJ * CQK) / DBD;
                let DBF = CJK / CQL;
                let DBG = CPU / CQM;
                let DBH = (DBF + DBG) / CJX;
                let DBI = DBE - DBH;
                let DBJ = ((((CQN * CQK) + (CQO * CQJ)) - (((CQP * CQM) + (CQQ * CQL)) * DBE)) / DBD) - (((((CJL - (CQP * DBF)) / CQL) + ((CPV - (CQQ * DBG)) / CQM)) - (CKA * DBH)) / CJX);
                let DBK = DBI + D;
                let DBL = (DBI * CJX) / DBK;
                let DBM = (((DBJ * CJX) + (CKA * DBI)) - (DBJ * DBL)) / DBK;
                let DBN = CXN - DBL;
                let DBO = CXT - DBM;
                let DBP = (CJX + (CXN * CXO)) / DBN;
                let DBQ = ((CKA + ((CXT * CXO) + (CXU * CXN))) - (DBO * DBP)) / DBN;
                let DBR = DBQ * DBP;
                let DBS = ((DBP * DBP) + CPW).sqrt();
                let DBT = GO * (DBP + DBS);
                let DBU = CM / CVO;
                let DBV = DBU * GO;
                let DBW = DAP + DBB;
                let DBX = DBV * DBW;
                let DBY = ((((Lanes([EK, 0.0, 0.0, 0.0, 0.0]) - (CVP * DBU)) / CVO) * GO) * DBW) + ((DAQ + DBC) * DBV);
                let DBZ = CJX / DBL;
                let DCA = D - DBZ;
                let DCB = ((CKA - (DBM * DBZ)) / DBL) * AC;
                let DCC = D + CXO;
                let DCD = (LA * DBL) - CJX;
                let DCE = ((DCD * CXZ) - LA) - CXO;
                let DCF = DCE * DBT;
                let DCG = ((((((DBM * LA) - CKA) * CXZ) + (CYA * DCD)) - CXU) * DBT) + (((DBQ + ((DBR + DBR) * (GY / (GX * DBS)))) * GO) * DCE);
                let DCI = if DBX > DCH { 1.0 } else { 0.0 };
                let DDR;
                let DDS;
                let DDT;
                let DDU;
                if DCI != 0.0 {
                    let DCJ = DBX * DBX;
                    let DCK = DBY * DBX;
                    let DCL = LA / DCJ;
                    let DCM = (((DCK + DCK) * DCL) * AC) / DCJ;
                    let DCN = DCL * DCA;
                    let DCO = (DCM * DCA) + (DCB * DCL);
                    let DCP = DCL + DCF;
                    let DCQ = DCM + DCG;
                    let DCR = DCL * DCC;
                    let DCS = (DCM * DCC) + (CXU * DCL);
                    let DCT = DCO * DCN;
                    let DCV = DCU * DCL;
                    let DCW = DCV * DCL;
                    let DCY = (((DCN * DCN) + (DCW * DCL)) + DCX).sqrt();
                    let DCZ = ((DCT + DCT) + (((((DCM * DCU) * DCL) + (DCM * DCV)) * DCL) + (DCM * DCW))) * (GY / (GX * DCY));
                    let DDA = DCS * DCR;
                    let DDB = DCU * DCP;
                    let DDC = DDB * DCP;
                    let DDD = (((DCR * DCR) + (DDC * DCP)) + DCX).sqrt();
                    let DDE = ((DDA + DDA) + (((((DCQ * DCU) * DCP) + (DCQ * DDB)) * DCP) + (DCQ * DDC))) * (GY / (GX * DDD));
                    let DDF = GO * (DCY + DCN);
                    let DDG = (WD * (DDF.ln())).exp();
                    let DDH = GO * (DCY - DCN);
                    let DDI = (WD * (DDH.ln())).exp();
                    let DDJ = DDG - DDI;
                    let DDK = (((((DCZ + DCO) * GO) * (GY / DDF)) * WD) * DDG) - (((((DCZ - DCO) * GO) * (GY / DDH)) * WD) * DDI);
                    let DDL = GO * (DDD + DCR);
                    let DDM = (WD * (DDL.ln())).exp();
                    let DDN = GO * (DDD - DCR);
                    let DDO = (WD * (DDN.ln())).exp();
                    let DDP = DDM - DDO;
                    let DDQ = (((((DDE + DCS) * GO) * (GY / DDL)) * WD) * DDM) - (((((DDE - DCS) * GO) * (GY / DDN)) * WD) * DDO);
                    DDR = DDJ;
                    DDS = DDP;
                    DDT = DDK;
                    DDU = DDQ;
                } else {
                    DDR = DCA;
                    DDS = DCC;
                    DDT = DCB;
                    DDU = CXU;
                }
                let DDV = DBN * DBN;
                let DDW = DBO * DBN;
                let DDX = DDW + DDW;
                let DDY = DDR - DDS;
                let DDZ = (DDT - DDU) * DDY;
                let DEA = ((DDY * DDY) + (UW * DDV)).sqrt();
                let DEC = DEB * ((DDR + DDS) + DEA);
                let DED = ((DDT + DDU) + (((DDZ + DDZ) + (DDX * UW)) * (GY / (GX * DEA)))) * DEB;
                let DEE = CJX + (DBL * DEC);
                let DEF = CKA + ((DBM * DEC) + (DED * DBL));
                let DEG = DEC - CXO;
                let DEH = CXN * DEG;
                let DEI = (CXT * DEG) + ((DED - CXU) * CXN);
                let DEJ = DEE - DEH;
                let DEK = (DEF - DEI) * DEJ;
                let DEM = ((DEJ * DEJ) + (DEL * DDV)).sqrt();
                let DEN = GO * ((DEE + DEH) + DEM);
                let DEO = ((DEF + DEI) + (((DEK + DEK) + (DDX * DEL)) * (GY / (GX * DEM)))) * GO;
                DAC = DEN;
                DAD = DEC;
                DAE = DBL;
                DAF = DEO;
                DAG = DED;
                DAH = DBM;
            } else {
                let CZX = CZW * (D + CXO);
                let CZY = CXU * CZW;
                let CZZ = CZX - (GO * CXO);
                let DAA = (GO * CJX) + (CXN * CZZ);
                let DAB = (CKA * GO) + ((CXT * CZZ) + ((CZY - (CXU * GO)) * CXN));
                DAC = DAA;
                DAD = CZX;
                DAE = CXN;
                DAF = DAB;
                DAG = CZY;
                DAH = CXT;
            }
            let DAI = DAC - GO;
            let DAJ = if DAI < SY { 1.0 } else { 0.0 };
            let DET;
            let DEU;
            if DAJ != 0.0 {
                let DEP = DAI.exp();
                let DEQ = D + DEP;
                let DER = DEQ.ln();
                let DES = (DAF * DEP) * (GY / DEQ);
                DET = DER;
                DEU = DES;
            } else {
                DET = DAI;
                DEU = DAF;
            }
            let DEV = DET + GO;
            let DEW = CJX / DEV;
            let DEX = DAG + (((CKA - (DEU * DEW)) / DEV) * (GY / DEW));
            let DEY = (DAD + (DEW.ln())) - UX;
            let DEZ = if DEY < SY { 1.0 } else { 0.0 };
            let DFE;
            let DFF;
            if DEZ != 0.0 {
                let DFA = DEY.exp();
                let DFB = D + DFA;
                let DFC = DFB.ln();
                let DFD = (DEX * DFA) * (GY / DFB);
                DFE = DFC;
                DFF = DFD;
            } else {
                DFE = DEY;
                DFF = DEX;
            }
            let DFG = AID - (DFE + UX);
            let DFH = DFF * AC;
            let DFI = if DFG < SY { 1.0 } else { 0.0 };
            let DFN;
            let DFO;
            if DFI != 0.0 {
                let DFJ = DFG.exp();
                let DFK = D + DFJ;
                let DFL = DFK.ln();
                let DFM = (DFH * DFJ) * (GY / DFK);
                DFN = DFL;
                DFO = DFM;
            } else {
                DFN = DFG;
                DFO = DFH;
            }
            let DFP = AID - DFN;
            let DFQ = RA / DFP;
            let DFR = Lanes([RC[0], RC[1], RC[2], 0.0, 0.0]);
            let DFS = DFQ * DFQ;
            let DFT = ((DFR - ((DFO * AC) * DFQ)) / DFP) * DFQ;
            let DFU = DFS * DFS;
            let DFV = (DFT + DFT) * DFS;
            let DFW = DFV + DFV;
            let DFX = DFU * DFU;
            let DFY = DFW * DFU;
            let DGA = D + (DFZ * DFU);
            let DGC = (DGB * (DGA.ln())).exp();
            let DGD = (DFY + DFY) * DFX;
            let DGE = DGC + (DFX * DFX);
            let DGG = (DGF * (DGE.ln())).exp();
            let DGH = RA * DGG;
            let DGI = RC * DGG;
            let DGJ = Lanes([DGI[0], DGI[1], DGI[2], 0.0, 0.0]) + (((((((((DFW * DFZ) * (GY / DGA)) * DGB) * DGC) + (DGD + DGD)) * (GY / DGE)) * DGF) * DGG) * RA);
            let DGK = (((((ALN * AKQ) + ALO) - ALQ) / AJH) * ALS) + DGJ;
            let DGL = (ALR + DGH) + ZD;
            let DGM = (((((ALW * AKQ) + ALX) - ALZ) / AJH) * AMB) + DGJ;
            let DGN = (AMA + DGH) + ZD;
            let DGO = (DGL - ALA) * WD;
            let DGP = (DGK - ALB) * WD;
            let DGQ = if DGO < SY { 1.0 } else { 0.0 };
            let DGV;
            let DGW;
            if DGQ != 0.0 {
                let DGR = DGO.exp();
                let DGS = D + DGR;
                let DGT = DGS.ln();
                let DGU = (DGP * DGR) * (GY / DGS);
                DGV = DGT;
                DGW = DGU;
            } else {
                DGV = DGO;
                DGW = DGP;
            }
            let DGX = DGL - (ZD * DGV);
            let DGY = DGK - (DGW * ZD);
            let DGZ = (DGN - ALE) * WD;
            let DHA = (DGM - ALF) * WD;
            let DHB = if DGZ < SY { 1.0 } else { 0.0 };
            let DHG;
            let DHH;
            if DHB != 0.0 {
                let DHC = DGZ.exp();
                let DHD = D + DHC;
                let DHE = DHD.ln();
                let DHF = (DHA * DHC) * (GY / DHD);
                DHG = DHE;
                DHH = DHF;
            } else {
                DHG = DGZ;
                DHH = DHA;
            }
            let DHI = AMY + (DGN - (ZD * DHG));
            let DHJ = ANB + DGX;
            let DHK = DHJ * ALJ;
            let DHL = ((ANC + DGY) * ALJ) + (ALL * DHJ);
            let DHM = (DGL - (DHI * ALG)) * WD;
            let DHN = (DGK - (((AMZ + (DGM - (DHH * ZD))) * ALG) + (ALI * DHI))) * WD;
            let DHO = if DHM < SY { 1.0 } else { 0.0 };
            let DHT;
            let DHU;
            if DHO != 0.0 {
                let DHP = DHM.exp();
                let DHQ = D + DHP;
                let DHR = DHQ.ln();
                let DHS = (DHN * DHP) * (GY / DHQ);
                DHT = DHR;
                DHU = DHS;
            } else {
                DHT = DHM;
                DHU = DHN;
            }
            let DHV = DGL - (ZD * DHT);
            let DHW = DGK - (DHU * ZD);
            let DHX = (DGN - DHK) * WD;
            let DHY = (DGM - DHL) * WD;
            let DHZ = if DHX < SY { 1.0 } else { 0.0 };
            let DIE;
            let DIF;
            if DHZ != 0.0 {
                let DIA = DHX.exp();
                let DIB = D + DIA;
                let DIC = DIB.ln();
                let DID = (DHY * DIA) * (GY / DIB);
                DIE = DIC;
                DIF = DID;
            } else {
                DIE = DHX;
                DIF = DHY;
            }
            let DIG = AIH - DHV;
            let DIH = AII - DHW;
            let DII = AIP - (DGN - (ZD * DIE));
            let DIJ = AIQ - (DGM - (DIF * ZD));
            let DIK = AIR * DIG;
            let DIL = (AIT * DIG) + (DIH * AIR);
            let DIM = (AIH - DIG) - DGH;
            let DIN = (AII - DIH) - DGJ;
            let DIO = if DIM < SY { 1.0 } else { 0.0 };
            let DIX;
            let DIY;
            if DIO != 0.0 {
                let DIP = DIM.exp();
                let DIQ = DIN * DIP;
                DIX = DIP;
                DIY = DIQ;
            } else {
                let DIR = DIM - SY;
                let DIS = GO * DIR;
                let DIT = D + (DIR * WD);
                let DIU = D + (DIS * DIT);
                let DIV = XB * (D + (DIR * DIU));
                let DIW = ((DIN * DIU) + ((((DIN * GO) * DIT) + ((DIN * WD) * DIS)) * DIR)) * XB;
                DIX = DIV;
                DIY = DIW;
            }
            let DIZ = AJH * DIX;
            let DJA = (AJJ * DIX) + (DIY * AJH);
            let DJB = DIL * DIK;
            let DJC = (DIK * DIK) - DIZ;
            let DJD = (DJB + DJB) - DJA;
            let DJE = (AOY * DIK) + DIZ;
            let DJF = ((AOZ * DIK) + (DIL * AOY)) + DJA;
            let DJG = APC - DIZ;
            let DJH = APD - DJA;
            let DJI = if DJC < -5e-3f64 { 1.0 } else { 0.0 };
            let DKL;
            let DKM;
            let DKN;
            let DKO;
            let DKP;
            let DKQ;
            let DKR;
            let DKS;
            let DKT;
            let DKU;
            let DKV;
            let DKW;
            let DKX;
            let DKY;
            if DJI != 0.0 {
                let DJJ = (DJC.abs()).sqrt();
                let DJK = (DJD * ((GX * (if DJC >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * DJJ));
                let DJL = GO * DJJ;
                let DJM = DJL.tan();
                let DJN = DJL.cos();
                let DJO = DJJ / DJM;
                let DJP = (DJK - (((DJK * GO) * (GY / (DJN * DJN))) * DJO)) / DJM;
                let DJQ = (YY * DJE) / DJC;
                let DJR = ((DJF * YY) - (DJD * DJQ)) / DJC;
                let DJS = LA - DJO;
                let DJT = DJC + (DJO * DJS);
                let DJU = DJT * DJQ;
                let DJV = ((DJD + ((DJP * DJS) + ((DJP * AC) * DJO))) * DJQ) + (DJR * DJT);
                let DJW = LA * DJU;
                let DJX = D + DJO;
                let DJY = DJE - (DJW * DJX);
                let DJZ = (DJU * DJG) / DJE;
                let DKA = (DJY * DJQ) + DJZ;
                let DKB = (((DJF - (((DJV * LA) * DJX) + (DJP * DJW))) * DJQ) + (DJR * DJY)) + ((((DJV * DJG) + (DJH * DJU)) - (DJF * DJZ)) / DJE);
                let DKC = D - (GO * DJO);
                let DKD = (DJP * GO) * AC;
                let DKE = DJE / DJC;
                let DKF = DKE * DKC;
                let DKG = (((DJF - (DJD * DKE)) / DJC) * DKC) + (DKD * DKE);
                let DKH = DKF + (GO * DJU);
                let DKI = ((DJG * DKC) - (DJE * DKH)) / DJC;
                let DKJ = ((((DJH * DKC) + (DKD * DJG)) - ((DJF * DKH) + ((DKG + (DJV * GO)) * DJE))) - (DJD * DKI)) / DJC;
                DKL = B;
                DKM = DJJ;
                DKN = DJO;
                DKO = DJU;
                DKP = DKA;
                DKQ = DKF;
                DKR = DKI;
                DKS = AFD;
                DKT = DJK;
                DKU = DJP;
                DKV = DJV;
                DKW = DKB;
                DKX = DKG;
                DKY = DKJ;
            } else {
                let DKK = if DJC > AQJ { 1.0 } else { 0.0 };
                let DNQ;
                let DNR;
                let DNS;
                let DNT;
                let DNU;
                let DNV;
                let DNW;
                let DNX;
                let DNY;
                let DNZ;
                let DOA;
                let DOB;
                let DOC;
                let DOD;
                if DKK != 0.0 {
                    let DLA = (DJC.abs()).sqrt();
                    let DLB = (DJD * ((GX * (if DJC >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * DLA));
                    let DLC = (-DLA).exp();
                    let DLD = (DLB * AC) * DLC;
                    let DLE = D + DLC;
                    let DLF = D - DLC;
                    let DLG = (DLA * DLE) / DLF;
                    let DLH = (((DLB * DLE) + (DLD * DLA)) - ((DLD * AC) * DLG)) / DLF;
                    let DLI = (YY * DJE) / DJC;
                    let DLJ = ((DJF * YY) - (DJD * DLI)) / DJC;
                    let DLK = LA - DLG;
                    let DLL = DJC + (DLG * DLK);
                    let DLM = DLL * DLI;
                    let DLN = ((DJD + ((DLH * DLK) + ((DLH * AC) * DLG))) * DLI) + (DLJ * DLL);
                    let DLO = LA * DLM;
                    let DLP = D + DLG;
                    let DLQ = DJE - (DLO * DLP);
                    let DLR = (DLM * DJG) / DJE;
                    let DLS = (DLQ * DLI) + DLR;
                    let DLT = (((DJF - (((DLN * LA) * DLP) + (DLH * DLO))) * DLI) + (DLJ * DLQ)) + ((((DLN * DJG) + (DJH * DLM)) - (DJF * DLR)) / DJE);
                    let DLU = D - (GO * DLG);
                    let DLV = (DLH * GO) * AC;
                    let DLW = DJE / DJC;
                    let DLX = DLW * DLU;
                    let DLY = (((DJF - (DJD * DLW)) / DJC) * DLU) + (DLV * DLW);
                    let DLZ = DLX + (GO * DLM);
                    let DMA = ((DJG * DLU) - (DJE * DLZ)) / DJC;
                    let DMB = ((((DJH * DLU) + (DLV * DJG)) - ((DJF * DLZ) + ((DLY + (DLN * GO)) * DJE))) - (DJD * DMA)) / DJC;
                    DNQ = DLC;
                    DNR = DLA;
                    DNS = DLG;
                    DNT = DLM;
                    DNU = DLS;
                    DNV = DLX;
                    DNW = DMA;
                    DNX = DLD;
                    DNY = DLB;
                    DNZ = DLH;
                    DOA = DLN;
                    DOB = DLT;
                    DOC = DLY;
                    DOD = DMB;
                } else {
                    let DMC = DJC * ASC;
                    let DMD = DJC * ASE;
                    let DME = DJD * ASE;
                    let DMF = D - (DJC * ASH);
                    let DMG = D - (DMD * DMF);
                    let DMH = UC * (D - (DMC * DMG));
                    let DMI = ((((DJD * ASC) * DMG) + ((((DME * DMF) + (((DJD * ASH) * AC) * DMD)) * AC) * DMC)) * AC) * UC;
                    let DMJ = (DJD * DMH) + (DMI * DJC);
                    let DMK = LA + (DJC * DMH);
                    let DML = DJC * ASO;
                    let DMM = DJD * ASO;
                    let DMN = DJC * ASR;
                    let DMO = D - DML;
                    let DMP = D - (DMN * DMO);
                    let DMQ = UC * (D - (DML * DMP));
                    let DMR = (((DMM * DMP) + (((((DJD * ASR) * DMO) + ((DMM * AC) * DMN)) * AC) * DML)) * AC) * UC;
                    let DMS = DJE * DMQ;
                    let DMT = (DJF * DMQ) + (DMR * DJE);
                    let DMU = DJC * ASZ;
                    let DMV = ATB * DJC;
                    let DMW = D - (ATD * DJC);
                    let DMX = D - (DMV * DMW);
                    let DMY = ATG * (D - (DMU * DMX));
                    let DMZ = DJE * DJE;
                    let DNA = DJF * DJE;
                    let DNB = (DJG * DMQ) - (DMZ * DMY);
                    let DNC = ((DJH * DMQ) + (DMR * DJG)) - (((DNA + DNA) * DMY) + ((((((DJD * ASZ) * DMX) + (((((DJD * ATB) * DMW) + (((DJD * ATD) * AC) * DMV)) * AC) * DMU)) * AC) * ATG) * DMZ));
                    let DNE = DND * DJE;
                    let DNF = DNE * DMH;
                    let DNG = ((DJF * DND) * DMH) + (DMI * DNE);
                    let DNI = DNH * DJG;
                    let DNK = DNJ * DJE;
                    let DNL = DNK * DJE;
                    let DNM = LA - (ATV * DJC);
                    let DNN = D - (DMD * DNM);
                    let DNO = (DNI * DMH) + (DNL * DNN);
                    let DNP = (((DJH * DNH) * DMH) + (DMI * DNI)) + (((((DJF * DNJ) * DJE) + (DJF * DNK)) * DNN) + ((((DME * DNM) + (((DJD * ATV) * AC) * DMD)) * AC) * DNL));
                    DNQ = B;
                    DNR = B;
                    DNS = DMK;
                    DNT = DMS;
                    DNU = DNB;
                    DNV = DNF;
                    DNW = DNO;
                    DNX = AFD;
                    DNY = AFD;
                    DNZ = DMJ;
                    DOA = DMT;
                    DOB = DNC;
                    DOC = DNG;
                    DOD = DNP;
                }
                DKL = DNQ;
                DKM = DNR;
                DKN = DNS;
                DKO = DNT;
                DKP = DNU;
                DKQ = DNV;
                DKR = DNW;
                DKS = DNX;
                DKT = DNY;
                DKU = DNZ;
                DKV = DOA;
                DKW = DOB;
                DKX = DOC;
                DKY = DOD;
            }
            let DKZ = if DJC > AQJ { 1.0 } else { 0.0 };
            let DON;
            let DOO;
            let DOP;
            let DOQ;
            if DKZ != 0.0 {
                let DOE = LA - DKL;
                let DOF = D - (DKL * DOE);
                let DOG = (XQ * DJC) / DOF;
                let DOH = ((DJD * XQ) - ((((DKS * DOE) + ((DKS * AC) * DKL)) * AC) * DOG)) / DOF;
                let DOI = DOG * DKL;
                let DOJ = (DOH * DKL) + (DKS * DOG);
                let DOK = (DOG.ln()) - DKM;
                let DOL = (DOH * (GY / DOG)) - DKT;
                DON = DOI;
                DOO = DOK;
                DOP = DOJ;
                DOQ = DOL;
            } else {
                let DOM = if DJC < -5e-3f64 { 1.0 } else { 0.0 };
                let DPI;
                let DPJ;
                let DPK;
                let DPL;
                if DOM != 0.0 {
                    let DOS = GO * DKM;
                    let DOT = DOS.sin();
                    let DOU = DOT * DOT;
                    let DOV = ((DKT * GO) * (DOS.cos())) * DOT;
                    let DOW = (-DJC) / DOU;
                    let DOX = ((DJD * AC) - ((DOV + DOV) * DOW)) / DOU;
                    let DOY = DOW.ln();
                    let DOZ = DOX * (GY / DOW);
                    DPI = DOW;
                    DPJ = DOY;
                    DPK = DOX;
                    DPL = DOZ;
                } else {
                    let DPA = DJC * WD;
                    let DPB = ATB * DJC;
                    let DPC = D - (AVN * DJC);
                    let DPD = D - (DPB * DPC);
                    let DPE = XQ - (DPA * DPD);
                    let DPF = (((DJD * WD) * DPD) + (((((DJD * ATB) * DPC) + (((DJD * AVN) * AC) * DPB)) * AC) * DPA)) * AC;
                    let DPG = DPE.ln();
                    let DPH = DPF * (GY / DPE);
                    DPI = DPE;
                    DPJ = DPG;
                    DPK = DPF;
                    DPL = DPH;
                }
                DON = DPI;
                DOO = DPJ;
                DOP = DPK;
                DOQ = DPL;
            }
            let DOR = if ((AVB * DIK) + DKN) > B { 1.0 } else { 0.0 };
            let DQH;
            let DQI;
            let DQJ;
            let DQK;
            let DQL;
            let DQM;
            if DOR != 0.0 {
                let DPM = DIK + DKN;
                let DPN = DIL + DKU;
                let DPO = AIR + DKO;
                let DPP = AIT + DKV;
                DQH = DPM;
                DQI = DPO;
                DQJ = DKP;
                DQK = DPN;
                DQL = DPP;
                DQM = DKW;
            } else {
                let DPQ = DIK - DKN;
                let DPR = D / DPQ;
                let DPS = (((DIL - DKU) * DPR) * AC) / DPQ;
                let DPT = DKO - AIR;
                let DPU = DKV - AIT;
                let DPV = DIZ - DON;
                let DPW = DPV * DPR;
                let DPX = ((DJA - DOP) * DPR) + (DPS * DPV);
                let DPY = ((DPT * DPW) - DIZ) - (DKQ * DON);
                let DPZ = DPY * DPR;
                let DQA = (((((DPU * DPW) + (DPX * DPT)) - DJA) - ((DKX * DON) + (DOP * DKQ))) * DPR) + (DPS * DPY);
                let DQB = LA * DPT;
                let DQC = DKX * DKQ;
                let DQD = DKR + (DKQ * DKQ);
                let DQE = (((DKP * DPW) + (DQB * DPZ)) + DIZ) - (DQD * DON);
                let DQF = DQE * DPR;
                let DQG = ((((((DKW * DPW) + (DPX * DKP)) + (((DPU * LA) * DPZ) + (DQA * DQB))) + DJA) - (((DKY + (DQC + DQC)) * DON) + (DOP * DQD))) * DPR) + (DPS * DQE);
                DQH = DPW;
                DQI = DPZ;
                DQJ = DQF;
                DQK = DPX;
                DQL = DQA;
                DQM = DQG;
            }
            let DQN = if DQH > B { 1.0 } else { 0.0 };
            let DRH;
            let DRI;
            let DRJ;
            let DRK;
            let DRL;
            let DRM;
            if DQN != 0.0 {
                let DQO = DQH.ln();
                let DQP = DQK * (GY / DQH);
                let DQQ = D / DQH;
                let DQR = ((DQK * DQQ) * AC) / DQH;
                let DQS = DQI * DQQ;
                let DQT = (DQL * DQQ) + (DQR * DQI);
                let DQU = DQT * DQS;
                let DQV = (DQJ * DQQ) - (DQS * DQS);
                let DQW = ((DQM * DQQ) + (DQR * DQJ)) - (DQU + DQU);
                DRH = DQO;
                DRI = DQS;
                DRJ = DQV;
                DRK = DQP;
                DRL = DQT;
                DRM = DQW;
            } else {
                let DQX = -DIK;
                let DQY = (DIK + HW) + (DQX.ln());
                let DQZ = DIL + ((DIL * AC) * (GY / DQX));
                let DRA = D / DIG;
                let DRB = ((DIH * DRA) * AC) / DIG;
                let DRC = AIR + DRA;
                let DRD = AIT + DRB;
                let DRE = -DRA;
                let DRF = DRE * DRA;
                let DRG = ((DRB * AC) * DRA) + (DRB * DRE);
                DRH = DQY;
                DRI = DRC;
                DRJ = DRF;
                DRK = DQZ;
                DRL = DRD;
                DRM = DRG;
            }
            let DRN = ((AXZ + DIG) + (LA * DRH)) - DOO;
            let DRO = (D + (LA * DRI)) - DKQ;
            let DRP = (LA * DRJ) - DKR;
            let DRQ = DIK + (AIU * DRN);
            let DRR = DIL + ((AIW * DRN) + ((((AYA + DIH) + (DRK * LA)) - DOQ) * AIU));
            let DRS = AIR + (AIU * DRO);
            let DRT = AIT + ((AIW * DRO) + (((DRL * LA) - DKX) * AIU));
            let DRU = AIU * DRP;
            let DRV = (DRQ * DQH) - DIZ;
            let DRW = ((DRR * DQH) + (DQK * DRQ)) - DJA;
            let DRX = ((DRS * DQH) + (DRQ * DQI)) + DIZ;
            let DRY = (((DRT * DQH) + (DQK * DRS)) + ((DRR * DQI) + (DQL * DRQ))) + DJA;
            let DRZ = LA * DRS;
            let DSA = (((DRU * DQH) + (DRZ * DQI)) + (DRQ * DQJ)) - DIZ;
            let DSB = DRY * DRX;
            let DSC = GO * DRV;
            let DSD = (DRX * DRX) - (DSC * DSA);
            let DSE = (DSB + DSB) - (((DRW * GO) * DSA) + ((((((((AIW * DRP) + (((DRM * LA) - DKY) * AIU)) * DQH) + (DQK * DRU)) + (((DRT * LA) * DQI) + (DQL * DRZ))) + ((DRR * DQJ) + (DQM * DRQ))) - DJA) * DSC));
            let DSF = -DRV;
            let DSG = DSF * DRX;
            let DSH = DSE * DSD;
            let DSI = (DSD * DSD) + AYW;
            let DSJ = (DSG * DSD) / DSI;
            let DSK = DIG + DSJ;
            let DSL = DIH + (((((((DRW * AC) * DRX) + (DRY * DSF)) * DSD) + (DSE * DSG)) - ((DSH + DSH) * DSJ)) / DSI);
            let DSM = AIR * DSK;
            let DSN = (AIT * DSK) + (DSL * AIR);
            let DSO = AIU * DII;
            let DSP = (AIW * DII) + (DIJ * AIU);
            let DSQ = DSM + DSO;
            let DSR = DSN + DSP;
            let DSS = DSR * AZH;
            let DST = D + (AZH * DSQ);
            let DSU = DSM * DSO;
            let DSV = (DSN * DSO) + (DSP * DSM);
            let DSW = (AZL + (AZK * DSQ)) + DSU;
            let DSX = (DSR * AZK) + DSV;
            let DSY = AZL * ((LA * DSQ) + DSU);
            let DSZ = DSX * DSW;
            let DTA = XQ * DST;
            let DTB = ((DSW * DSW) - (DTA * DSY)).sqrt();
            let DTC = LA * DST;
            let DTD = (DTB - DSW) / DTC;
            let DTE = DSN * DSM;
            let DTF = (DSM * DSM) - DTD;
            let DTG = (DTE + DTE) - ((((((DSZ + DSZ) - (((DSS * XQ) * DSY) + ((((DSR * LA) + DSV) * AZL) * DTA))) * (GY / (GX * DTB))) - DSX) - ((DSS * LA) * DTD)) / DTC);
            let DTH = if DTF > B { 1.0 } else { 0.0 };
            let DTQ;
            let DTR;
            if DTH != 0.0 {
                let DTI = DTF / AJH;
                let DTJ = (((DTI.ln()) + DGH) - AIH) + DSK;
                let DTK = DTF * DTJ;
                let DTL = (DTG * DTJ) + (((((((DTG - (AJJ * DTI)) / AJH) * (GY / DTI)) + DGJ) - AII) + DSL) * DTF);
                let DTM = (AOY * DSM) + DTF;
                let DTN = ((AOZ * DSM) + (DSN * AOY)) + DTG;
                let DTO = (AIH - DSK) - DGL;
                let DTP = if (if (if (if DTK < B { 1.0 } else { 0.0 }) != 0.0 && (if DTM > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((DTO + BAH) + (AIR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DTO > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DUN;
                let DUO;
                if DTP != 0.0 {
                    let DUK = DTK / DTM;
                    let DUL = DSK - DUK;
                    let DUM = DSL - ((DTL - (DTN * DUK)) / DTM);
                    DUN = DUL;
                    DUO = DUM;
                } else {
                    DUN = DSK;
                    DUO = DSL;
                }
                DTQ = DUN;
                DTR = DUO;
            } else {
                DTQ = DSK;
                DTR = DSL;
            }
            let DTS = AIR * DTQ;
            let DTT = (AIT * DTQ) + (DTR * AIR);
            let DTU = DTS + DSO;
            let DTV = DTT + DSP;
            let DTW = DTV * AZH;
            let DTX = D + (AZH * DTU);
            let DTY = DTS * DSO;
            let DTZ = (DTT * DSO) + (DSP * DTS);
            let DUA = (AZL + (AZK * DTU)) + DTY;
            let DUB = (DTV * AZK) + DTZ;
            let DUC = AZL * ((LA * DTU) + DTY);
            let DUD = DUB * DUA;
            let DUE = XQ * DTX;
            let DUF = ((DUA * DUA) - (DUE * DUC)).sqrt();
            let DUG = LA * DTX;
            let DUH = (DUF - DUA) / DUG;
            let DUI = (((((DUD + DUD) - (((DTW * XQ) * DUC) + ((((DTV * LA) + DTZ) * AZL) * DUE))) * (GY / (GX * DUF))) - DUB) - ((DTW * LA) * DUH)) / DUG;
            let DUJ = if DUH < -5e-3f64 { 1.0 } else { 0.0 };
            let DVA;
            let DVB;
            let DVC;
            let DVD;
            let DVE;
            let DVF;
            let DVG;
            let DVH;
            if DUJ != 0.0 {
                let DUP = (DUH.abs()).sqrt();
                let DUQ = (DUI * ((GX * (if DUH >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * DUP));
                let DUR = GO * DUP;
                let DUS = DUR.tan();
                let DUT = DUR.cos();
                let DUU = DUP / DUS;
                let DUV = (DUQ - (((DUQ * GO) * (GY / (DUT * DUT))) * DUU)) / DUS;
                let DUW = LA - DUU;
                let DUX = (YY * (DUH + (DUU * DUW))) / DUH;
                let DUY = (((DUI + ((DUV * DUW) + ((DUV * AC) * DUU))) * YY) - (DUI * DUX)) / DUH;
                DVA = DUU;
                DVB = DUX;
                DVC = DKL;
                DVD = DUP;
                DVE = DUV;
                DVF = DUY;
                DVG = DKS;
                DVH = DUQ;
            } else {
                let DUZ = if DUH > AQJ { 1.0 } else { 0.0 };
                let DWM;
                let DWN;
                let DWO;
                let DWP;
                let DWQ;
                let DWR;
                let DWS;
                let DWT;
                if DUZ != 0.0 {
                    let DVO = (DUH.abs()).sqrt();
                    let DVP = (DUI * ((GX * (if DUH >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * DVO));
                    let DVQ = (-DVO).exp();
                    let DVR = (DVP * AC) * DVQ;
                    let DVS = D + DVQ;
                    let DVT = D - DVQ;
                    let DVU = (DVO * DVS) / DVT;
                    let DVV = (((DVP * DVS) + (DVR * DVO)) - ((DVR * AC) * DVU)) / DVT;
                    let DVW = LA - DVU;
                    let DVX = (YY * (DUH + (DVU * DVW))) / DUH;
                    let DVY = (((DUI + ((DVV * DVW) + ((DVV * AC) * DVU))) * YY) - (DUI * DVX)) / DUH;
                    DWM = DVU;
                    DWN = DVX;
                    DWO = DVQ;
                    DWP = DVO;
                    DWQ = DVV;
                    DWR = DVY;
                    DWS = DVR;
                    DWT = DVP;
                } else {
                    let DVZ = DUH * UC;
                    let DWA = DUH * ASC;
                    let DWB = D - (DUH * ASE);
                    let DWC = D - (DWA * DWB);
                    let DWD = ((DUI * UC) * DWC) + (((((DUI * ASC) * DWB) + (((DUI * ASE) * AC) * DWA)) * AC) * DVZ);
                    let DWE = LA + (DVZ * DWC);
                    let DWF = DUH * ASO;
                    let DWG = DUI * ASO;
                    let DWH = DUH * ASR;
                    let DWI = D - DWF;
                    let DWJ = D - (DWH * DWI);
                    let DWK = UC * (D - (DWF * DWJ));
                    let DWL = (((DWG * DWJ) + (((((DUI * ASR) * DWI) + ((DWG * AC) * DWH)) * AC) * DWF)) * AC) * UC;
                    DWM = DWE;
                    DWN = DWK;
                    DWO = DKL;
                    DWP = DKM;
                    DWQ = DWD;
                    DWR = DWL;
                    DWS = DKS;
                    DWT = DKT;
                }
                DVA = DWM;
                DVB = DWN;
                DVC = DWO;
                DVD = DWP;
                DVE = DWQ;
                DVF = DWR;
                DVG = DWS;
                DVH = DWT;
            }
            let DVI = (DTU * DVB) + D;
            let DVJ = (((DTU * DVA) + DTY) + DUH) / DVI;
            let DVK = DTT * DTS;
            let DVL = (DTS * DTS) - (DUH - DVJ);
            let DVM = (DVK + DVK) - (DUI - ((((((DTV * DVA) + (DVE * DTU)) + DTZ) + DUI) - (((DTV * DVB) + (DVF * DTU)) * DVJ)) / DVI));
            let DVN = if DVL > B { 1.0 } else { 0.0 };
            let DXC;
            let DXD;
            if DVN != 0.0 {
                let DWU = DVL / AJH;
                let DWV = (((DWU.ln()) + DGH) - AIH) + DTQ;
                let DWW = DVL * DWV;
                let DWX = (DVM * DWV) + (((((((DVM - (AJJ * DWU)) / AJH) * (GY / DWU)) + DGJ) - AII) + DTR) * DVL);
                let DWY = (AOY * DTS) + DVL;
                let DWZ = ((AOZ * DTS) + (DTT * AOY)) + DVM;
                let DXA = (AIH - DTQ) - DGL;
                let DXB = if (if (if (if DWW < B { 1.0 } else { 0.0 }) != 0.0 && (if DWY > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((DXA + BAH) + (AIR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DXA > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DXM;
                let DXN;
                if DXB != 0.0 {
                    let DXJ = DWW / DWY;
                    let DXK = DTQ - DXJ;
                    let DXL = DTR - ((DWX - (DWZ * DXJ)) / DWY);
                    DXM = DXK;
                    DXN = DXL;
                } else {
                    DXM = DTQ;
                    DXN = DTR;
                }
                DXC = DXM;
                DXD = DXN;
            } else {
                DXC = DTQ;
                DXD = DTR;
            }
            let DXE = AIR * DXC;
            let DXF = (AIT * DXC) + (DXD * AIR);
            let DXG = (AIH - DXC) - DGH;
            let DXH = (AII - DXD) - DGJ;
            let DXI = if DXG < SY { 1.0 } else { 0.0 };
            let DXW;
            let DXX;
            if DXI != 0.0 {
                let DXO = DXG.exp();
                let DXP = DXH * DXO;
                DXW = DXO;
                DXX = DXP;
            } else {
                let DXQ = DXG - SY;
                let DXR = GO * DXQ;
                let DXS = D + (DXQ * WD);
                let DXT = D + (DXR * DXS);
                let DXU = XB * (D + (DXQ * DXT));
                let DXV = ((DXH * DXT) + ((((DXH * GO) * DXS) + ((DXH * WD) * DXR)) * DXQ)) * XB;
                DXW = DXU;
                DXX = DXV;
            }
            let DXY = AJH * DXW;
            let DXZ = (AJJ * DXW) + (DXX * AJH);
            let DYA = DXF * DXE;
            let DYB = (DXE * DXE) - DXY;
            let DYC = (DYA + DYA) - DXZ;
            let DYD = (AOY * DXE) + DXY;
            let DYE = ((AOZ * DXE) + (DXF * AOY)) + DXZ;
            let DYF = APC - DXY;
            let DYG = APD - DXZ;
            let DYH = if DYB < -5e-3f64 { 1.0 } else { 0.0 };
            let DZK;
            let DZL;
            let DZM;
            let DZN;
            let DZO;
            let DZP;
            let DZQ;
            let DZR;
            let DZS;
            let DZT;
            let DZU;
            let DZV;
            let DZW;
            let DZX;
            if DYH != 0.0 {
                let DYI = (DYB.abs()).sqrt();
                let DYJ = (DYC * ((GX * (if DYB >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * DYI));
                let DYK = GO * DYI;
                let DYL = DYK.tan();
                let DYM = DYK.cos();
                let DYN = DYI / DYL;
                let DYO = (DYJ - (((DYJ * GO) * (GY / (DYM * DYM))) * DYN)) / DYL;
                let DYP = (YY * DYD) / DYB;
                let DYQ = ((DYE * YY) - (DYC * DYP)) / DYB;
                let DYR = LA - DYN;
                let DYS = DYB + (DYN * DYR);
                let DYT = DYS * DYP;
                let DYU = ((DYC + ((DYO * DYR) + ((DYO * AC) * DYN))) * DYP) + (DYQ * DYS);
                let DYV = LA * DYT;
                let DYW = D + DYN;
                let DYX = DYD - (DYV * DYW);
                let DYY = (DYT * DYF) / DYD;
                let DYZ = (DYX * DYP) + DYY;
                let DZA = (((DYE - (((DYU * LA) * DYW) + (DYO * DYV))) * DYP) + (DYQ * DYX)) + ((((DYU * DYF) + (DYG * DYT)) - (DYE * DYY)) / DYD);
                let DZB = D - (GO * DYN);
                let DZC = (DYO * GO) * AC;
                let DZD = DYD / DYB;
                let DZE = DZD * DZB;
                let DZF = (((DYE - (DYC * DZD)) / DYB) * DZB) + (DZC * DZD);
                let DZG = DZE + (GO * DYT);
                let DZH = ((DYF * DZB) - (DYD * DZG)) / DYB;
                let DZI = ((((DYG * DZB) + (DZC * DYF)) - ((DYE * DZG) + ((DZF + (DYU * GO)) * DYD))) - (DYC * DZH)) / DYB;
                DZK = DVC;
                DZL = DYI;
                DZM = DYN;
                DZN = DYT;
                DZO = DYZ;
                DZP = DZE;
                DZQ = DZH;
                DZR = DVG;
                DZS = DYJ;
                DZT = DYO;
                DZU = DYU;
                DZV = DZA;
                DZW = DZF;
                DZX = DZI;
            } else {
                let DZJ = if DYB > AQJ { 1.0 } else { 0.0 };
                let ECP;
                let ECQ;
                let ECR;
                let ECS;
                let ECT;
                let ECU;
                let ECV;
                let ECW;
                let ECX;
                let ECY;
                let ECZ;
                let EDA;
                let EDB;
                let EDC;
                if DZJ != 0.0 {
                    let DZZ = (DYB.abs()).sqrt();
                    let EAA = (DYC * ((GX * (if DYB >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * DZZ));
                    let EAB = (-DZZ).exp();
                    let EAC = (EAA * AC) * EAB;
                    let EAD = D + EAB;
                    let EAE = D - EAB;
                    let EAF = (DZZ * EAD) / EAE;
                    let EAG = (((EAA * EAD) + (EAC * DZZ)) - ((EAC * AC) * EAF)) / EAE;
                    let EAH = (YY * DYD) / DYB;
                    let EAI = ((DYE * YY) - (DYC * EAH)) / DYB;
                    let EAJ = LA - EAF;
                    let EAK = DYB + (EAF * EAJ);
                    let EAL = EAK * EAH;
                    let EAM = ((DYC + ((EAG * EAJ) + ((EAG * AC) * EAF))) * EAH) + (EAI * EAK);
                    let EAN = LA * EAL;
                    let EAO = D + EAF;
                    let EAP = DYD - (EAN * EAO);
                    let EAQ = (EAL * DYF) / DYD;
                    let EAR = (EAP * EAH) + EAQ;
                    let EAS = (((DYE - (((EAM * LA) * EAO) + (EAG * EAN))) * EAH) + (EAI * EAP)) + ((((EAM * DYF) + (DYG * EAL)) - (DYE * EAQ)) / DYD);
                    let EAT = D - (GO * EAF);
                    let EAU = (EAG * GO) * AC;
                    let EAV = DYD / DYB;
                    let EAW = EAV * EAT;
                    let EAX = (((DYE - (DYC * EAV)) / DYB) * EAT) + (EAU * EAV);
                    let EAY = EAW + (GO * EAL);
                    let EAZ = ((DYF * EAT) - (DYD * EAY)) / DYB;
                    let EBA = ((((DYG * EAT) + (EAU * DYF)) - ((DYE * EAY) + ((EAX + (EAM * GO)) * DYD))) - (DYC * EAZ)) / DYB;
                    ECP = EAB;
                    ECQ = DZZ;
                    ECR = EAF;
                    ECS = EAL;
                    ECT = EAR;
                    ECU = EAW;
                    ECV = EAZ;
                    ECW = EAC;
                    ECX = EAA;
                    ECY = EAG;
                    ECZ = EAM;
                    EDA = EAS;
                    EDB = EAX;
                    EDC = EBA;
                } else {
                    let EBB = DYB * ASC;
                    let EBC = DYB * ASE;
                    let EBD = DYC * ASE;
                    let EBE = D - (DYB * ASH);
                    let EBF = D - (EBC * EBE);
                    let EBG = UC * (D - (EBB * EBF));
                    let EBH = ((((DYC * ASC) * EBF) + ((((EBD * EBE) + (((DYC * ASH) * AC) * EBC)) * AC) * EBB)) * AC) * UC;
                    let EBI = (DYC * EBG) + (EBH * DYB);
                    let EBJ = LA + (DYB * EBG);
                    let EBK = DYB * ASO;
                    let EBL = DYC * ASO;
                    let EBM = DYB * ASR;
                    let EBN = D - EBK;
                    let EBO = D - (EBM * EBN);
                    let EBP = UC * (D - (EBK * EBO));
                    let EBQ = (((EBL * EBO) + (((((DYC * ASR) * EBN) + ((EBL * AC) * EBM)) * AC) * EBK)) * AC) * UC;
                    let EBR = DYD * EBP;
                    let EBS = (DYE * EBP) + (EBQ * DYD);
                    let EBT = DYB * ASZ;
                    let EBU = ATB * DYB;
                    let EBV = D - (ATD * DYB);
                    let EBW = D - (EBU * EBV);
                    let EBX = ATG * (D - (EBT * EBW));
                    let EBY = DYD * DYD;
                    let EBZ = DYE * DYD;
                    let ECA = (DYF * EBP) - (EBY * EBX);
                    let ECB = ((DYG * EBP) + (EBQ * DYF)) - (((EBZ + EBZ) * EBX) + ((((((DYC * ASZ) * EBW) + (((((DYC * ATB) * EBV) + (((DYC * ATD) * AC) * EBU)) * AC) * EBT)) * AC) * ATG) * EBY));
                    let ECD = ECC * DYD;
                    let ECE = ECD * EBG;
                    let ECF = ((DYE * ECC) * EBG) + (EBH * ECD);
                    let ECH = ECG * DYF;
                    let ECJ = ECI * DYD;
                    let ECK = ECJ * DYD;
                    let ECL = LA - (ATV * DYB);
                    let ECM = D - (EBC * ECL);
                    let ECN = (ECH * EBG) + (ECK * ECM);
                    let ECO = (((DYG * ECG) * EBG) + (EBH * ECH)) + (((((DYE * ECI) * DYD) + (DYE * ECJ)) * ECM) + ((((EBD * ECL) + (((DYC * ATV) * AC) * EBC)) * AC) * ECK));
                    ECP = DVC;
                    ECQ = DVD;
                    ECR = EBJ;
                    ECS = EBR;
                    ECT = ECA;
                    ECU = ECE;
                    ECV = ECN;
                    ECW = DVG;
                    ECX = DVH;
                    ECY = EBI;
                    ECZ = EBS;
                    EDA = ECB;
                    EDB = ECF;
                    EDC = ECO;
                }
                DZK = ECP;
                DZL = ECQ;
                DZM = ECR;
                DZN = ECS;
                DZO = ECT;
                DZP = ECU;
                DZQ = ECV;
                DZR = ECW;
                DZS = ECX;
                DZT = ECY;
                DZU = ECZ;
                DZV = EDA;
                DZW = EDB;
                DZX = EDC;
            }
            let DZY = if DYB > AQJ { 1.0 } else { 0.0 };
            let EDM;
            let EDN;
            let EDO;
            let EDP;
            if DZY != 0.0 {
                let EDD = LA - DZK;
                let EDE = D - (DZK * EDD);
                let EDF = (XQ * DYB) / EDE;
                let EDG = ((DYC * XQ) - ((((DZR * EDD) + ((DZR * AC) * DZK)) * AC) * EDF)) / EDE;
                let EDH = EDF * DZK;
                let EDI = (EDG * DZK) + (DZR * EDF);
                let EDJ = (EDF.ln()) - DZL;
                let EDK = (EDG * (GY / EDF)) - DZS;
                EDM = EDH;
                EDN = EDJ;
                EDO = EDI;
                EDP = EDK;
            } else {
                let EDL = if DYB < -5e-3f64 { 1.0 } else { 0.0 };
                let EEH;
                let EEI;
                let EEJ;
                let EEK;
                if EDL != 0.0 {
                    let EDR = GO * DZL;
                    let EDS = EDR.sin();
                    let EDT = EDS * EDS;
                    let EDU = ((DZS * GO) * (EDR.cos())) * EDS;
                    let EDV = (-DYB) / EDT;
                    let EDW = ((DYC * AC) - ((EDU + EDU) * EDV)) / EDT;
                    let EDX = EDV.ln();
                    let EDY = EDW * (GY / EDV);
                    EEH = EDV;
                    EEI = EDX;
                    EEJ = EDW;
                    EEK = EDY;
                } else {
                    let EDZ = DYB * WD;
                    let EEA = ATB * DYB;
                    let EEB = D - (AVN * DYB);
                    let EEC = D - (EEA * EEB);
                    let EED = XQ - (EDZ * EEC);
                    let EEE = (((DYC * WD) * EEC) + (((((DYC * ATB) * EEB) + (((DYC * AVN) * AC) * EEA)) * AC) * EDZ)) * AC;
                    let EEF = EED.ln();
                    let EEG = EEE * (GY / EED);
                    EEH = EED;
                    EEI = EEF;
                    EEJ = EEE;
                    EEK = EEG;
                }
                EDM = EEH;
                EDN = EEI;
                EDO = EEJ;
                EDP = EEK;
            }
            let EDQ = if ((AVB * DXE) + DZM) > B { 1.0 } else { 0.0 };
            let EFG;
            let EFH;
            let EFI;
            let EFJ;
            let EFK;
            let EFL;
            if EDQ != 0.0 {
                let EEL = DXE + DZM;
                let EEM = DXF + DZT;
                let EEN = AIR + DZN;
                let EEO = AIT + DZU;
                EFG = EEL;
                EFH = EEN;
                EFI = DZO;
                EFJ = EEM;
                EFK = EEO;
                EFL = DZV;
            } else {
                let EEP = DXE - DZM;
                let EEQ = D / EEP;
                let EER = (((DXF - DZT) * EEQ) * AC) / EEP;
                let EES = DZN - AIR;
                let EET = DZU - AIT;
                let EEU = DXY - EDM;
                let EEV = EEU * EEQ;
                let EEW = ((DXZ - EDO) * EEQ) + (EER * EEU);
                let EEX = ((EES * EEV) - DXY) - (DZP * EDM);
                let EEY = EEX * EEQ;
                let EEZ = (((((EET * EEV) + (EEW * EES)) - DXZ) - ((DZW * EDM) + (EDO * DZP))) * EEQ) + (EER * EEX);
                let EFA = LA * EES;
                let EFB = DZW * DZP;
                let EFC = DZQ + (DZP * DZP);
                let EFD = (((DZO * EEV) + (EFA * EEY)) + DXY) - (EFC * EDM);
                let EFE = EFD * EEQ;
                let EFF = ((((((DZV * EEV) + (EEW * DZO)) + (((EET * LA) * EEY) + (EEZ * EFA))) + DXZ) - (((DZX + (EFB + EFB)) * EDM) + (EDO * EFC))) * EEQ) + (EER * EFD);
                EFG = EEV;
                EFH = EEY;
                EFI = EFE;
                EFJ = EEW;
                EFK = EEZ;
                EFL = EFF;
            }
            let EFM = if EFG > B { 1.0 } else { 0.0 };
            let EGG;
            let EGH;
            let EGI;
            let EGJ;
            let EGK;
            let EGL;
            if EFM != 0.0 {
                let EFN = EFG.ln();
                let EFO = EFJ * (GY / EFG);
                let EFP = D / EFG;
                let EFQ = ((EFJ * EFP) * AC) / EFG;
                let EFR = EFH * EFP;
                let EFS = (EFK * EFP) + (EFQ * EFH);
                let EFT = EFS * EFR;
                let EFU = (EFI * EFP) - (EFR * EFR);
                let EFV = ((EFL * EFP) + (EFQ * EFI)) - (EFT + EFT);
                EGG = EFN;
                EGH = EFR;
                EGI = EFU;
                EGJ = EFO;
                EGK = EFS;
                EGL = EFV;
            } else {
                let EFW = -DXE;
                let EFX = (DXE + HW) + (EFW.ln());
                let EFY = DXF + ((DXF * AC) * (GY / EFW));
                let EFZ = D / DXC;
                let EGA = ((DXD * EFZ) * AC) / DXC;
                let EGB = AIR + EFZ;
                let EGC = AIT + EGA;
                let EGD = -EFZ;
                let EGE = EGD * EFZ;
                let EGF = ((EGA * AC) * EFZ) + (EGA * EGD);
                EGG = EFX;
                EGH = EGB;
                EGI = EGE;
                EGJ = EFY;
                EGK = EGC;
                EGL = EGF;
            }
            let EGM = ((AXZ + DXC) + (LA * EGG)) - EDN;
            let EGN = (D + (LA * EGH)) - DZP;
            let EGO = (LA * EGI) - DZQ;
            let EGP = DXE + (AIU * EGM);
            let EGQ = DXF + ((AIW * EGM) + ((((AYA + DXD) + (EGJ * LA)) - EDP) * AIU));
            let EGR = AIR + (AIU * EGN);
            let EGS = AIT + ((AIW * EGN) + (((EGK * LA) - DZW) * AIU));
            let EGT = AIU * EGO;
            let EGU = (EGP * EFG) - DXY;
            let EGV = ((EGQ * EFG) + (EFJ * EGP)) - DXZ;
            let EGW = ((EGR * EFG) + (EGP * EFH)) + DXY;
            let EGX = (((EGS * EFG) + (EFJ * EGR)) + ((EGQ * EFH) + (EFK * EGP))) + DXZ;
            let EGY = LA * EGR;
            let EGZ = (((EGT * EFG) + (EGY * EFH)) + (EGP * EFI)) - DXY;
            let EHA = EGX * EGW;
            let EHB = GO * EGU;
            let EHC = (EGW * EGW) - (EHB * EGZ);
            let EHD = (EHA + EHA) - (((EGV * GO) * EGZ) + ((((((((AIW * EGO) + (((EGL * LA) - DZX) * AIU)) * EFG) + (EFJ * EGT)) + (((EGS * LA) * EFH) + (EFK * EGY))) + ((EGQ * EFI) + (EFL * EGP))) - DXZ) * EHB));
            let EHE = -EGU;
            let EHF = EHE * EGW;
            let EHG = EHD * EHC;
            let EHH = (EHC * EHC) + AYW;
            let EHI = (EHF * EHC) / EHH;
            let EHJ = DXC + EHI;
            let EHK = DXD + (((((((EGV * AC) * EGW) + (EGX * EHE)) * EHC) + (EHD * EHF)) - ((EHG + EHG) * EHI)) / EHH);
            let EHL = AIR * EHJ;
            let EHM = (AIT * EHJ) + (EHK * AIR);
            let EHN = (AIH - EHJ) - DGH;
            let EHO = (AII - EHK) - DGJ;
            let EHP = if EHN < SY { 1.0 } else { 0.0 };
            let EHY;
            let EHZ;
            if EHP != 0.0 {
                let EHQ = EHN.exp();
                let EHR = EHO * EHQ;
                EHY = EHQ;
                EHZ = EHR;
            } else {
                let EHS = EHN - SY;
                let EHT = GO * EHS;
                let EHU = D + (EHS * WD);
                let EHV = D + (EHT * EHU);
                let EHW = XB * (D + (EHS * EHV));
                let EHX = ((EHO * EHV) + ((((EHO * GO) * EHU) + ((EHO * WD) * EHT)) * EHS)) * XB;
                EHY = EHW;
                EHZ = EHX;
            }
            let EIA = AJH * EHY;
            let EIB = (AJJ * EHY) + (EHZ * AJH);
            let EIC = EHM * EHL;
            let EID = (EHL * EHL) - EIA;
            let EIE = (EIC + EIC) - EIB;
            let EIF = (AOY * EHL) + EIA;
            let EIG = ((AOZ * EHL) + (EHM * AOY)) + EIB;
            let EIH = APC - EIA;
            let EII = APD - EIB;
            let EIJ = if EID < -5e-3f64 { 1.0 } else { 0.0 };
            let EJM;
            let EJN;
            let EJO;
            let EJP;
            let EJQ;
            let EJR;
            let EJS;
            let EJT;
            let EJU;
            let EJV;
            let EJW;
            let EJX;
            let EJY;
            let EJZ;
            if EIJ != 0.0 {
                let EIK = (EID.abs()).sqrt();
                let EIL = (EIE * ((GX * (if EID >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * EIK));
                let EIM = GO * EIK;
                let EIN = EIM.tan();
                let EIO = EIM.cos();
                let EIP = EIK / EIN;
                let EIQ = (EIL - (((EIL * GO) * (GY / (EIO * EIO))) * EIP)) / EIN;
                let EIR = (YY * EIF) / EID;
                let EIS = ((EIG * YY) - (EIE * EIR)) / EID;
                let EIT = LA - EIP;
                let EIU = EID + (EIP * EIT);
                let EIV = EIU * EIR;
                let EIW = ((EIE + ((EIQ * EIT) + ((EIQ * AC) * EIP))) * EIR) + (EIS * EIU);
                let EIX = LA * EIV;
                let EIY = D + EIP;
                let EIZ = EIF - (EIX * EIY);
                let EJA = (EIV * EIH) / EIF;
                let EJB = (EIZ * EIR) + EJA;
                let EJC = (((EIG - (((EIW * LA) * EIY) + (EIQ * EIX))) * EIR) + (EIS * EIZ)) + ((((EIW * EIH) + (EII * EIV)) - (EIG * EJA)) / EIF);
                let EJD = D - (GO * EIP);
                let EJE = (EIQ * GO) * AC;
                let EJF = EIF / EID;
                let EJG = EJF * EJD;
                let EJH = (((EIG - (EIE * EJF)) / EID) * EJD) + (EJE * EJF);
                let EJI = EJG + (GO * EIV);
                let EJJ = ((EIH * EJD) - (EIF * EJI)) / EID;
                let EJK = ((((EII * EJD) + (EJE * EIH)) - ((EIG * EJI) + ((EJH + (EIW * GO)) * EIF))) - (EIE * EJJ)) / EID;
                EJM = DZK;
                EJN = EIK;
                EJO = EIP;
                EJP = EIV;
                EJQ = EJB;
                EJR = EJG;
                EJS = EJJ;
                EJT = DZR;
                EJU = EIL;
                EJV = EIQ;
                EJW = EIW;
                EJX = EJC;
                EJY = EJH;
                EJZ = EJK;
            } else {
                let EJL = if EID > AQJ { 1.0 } else { 0.0 };
                let EMR;
                let EMS;
                let EMT;
                let EMU;
                let EMV;
                let EMW;
                let EMX;
                let EMY;
                let EMZ;
                let ENA;
                let ENB;
                let ENC;
                let END;
                let ENE;
                if EJL != 0.0 {
                    let EKB = (EID.abs()).sqrt();
                    let EKC = (EIE * ((GX * (if EID >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * EKB));
                    let EKD = (-EKB).exp();
                    let EKE = (EKC * AC) * EKD;
                    let EKF = D + EKD;
                    let EKG = D - EKD;
                    let EKH = (EKB * EKF) / EKG;
                    let EKI = (((EKC * EKF) + (EKE * EKB)) - ((EKE * AC) * EKH)) / EKG;
                    let EKJ = (YY * EIF) / EID;
                    let EKK = ((EIG * YY) - (EIE * EKJ)) / EID;
                    let EKL = LA - EKH;
                    let EKM = EID + (EKH * EKL);
                    let EKN = EKM * EKJ;
                    let EKO = ((EIE + ((EKI * EKL) + ((EKI * AC) * EKH))) * EKJ) + (EKK * EKM);
                    let EKP = LA * EKN;
                    let EKQ = D + EKH;
                    let EKR = EIF - (EKP * EKQ);
                    let EKS = (EKN * EIH) / EIF;
                    let EKT = (EKR * EKJ) + EKS;
                    let EKU = (((EIG - (((EKO * LA) * EKQ) + (EKI * EKP))) * EKJ) + (EKK * EKR)) + ((((EKO * EIH) + (EII * EKN)) - (EIG * EKS)) / EIF);
                    let EKV = D - (GO * EKH);
                    let EKW = (EKI * GO) * AC;
                    let EKX = EIF / EID;
                    let EKY = EKX * EKV;
                    let EKZ = (((EIG - (EIE * EKX)) / EID) * EKV) + (EKW * EKX);
                    let ELA = EKY + (GO * EKN);
                    let ELB = ((EIH * EKV) - (EIF * ELA)) / EID;
                    let ELC = ((((EII * EKV) + (EKW * EIH)) - ((EIG * ELA) + ((EKZ + (EKO * GO)) * EIF))) - (EIE * ELB)) / EID;
                    EMR = EKD;
                    EMS = EKB;
                    EMT = EKH;
                    EMU = EKN;
                    EMV = EKT;
                    EMW = EKY;
                    EMX = ELB;
                    EMY = EKE;
                    EMZ = EKC;
                    ENA = EKI;
                    ENB = EKO;
                    ENC = EKU;
                    END = EKZ;
                    ENE = ELC;
                } else {
                    let ELD = EID * ASC;
                    let ELE = EID * ASE;
                    let ELF = EIE * ASE;
                    let ELG = D - (EID * ASH);
                    let ELH = D - (ELE * ELG);
                    let ELI = UC * (D - (ELD * ELH));
                    let ELJ = ((((EIE * ASC) * ELH) + ((((ELF * ELG) + (((EIE * ASH) * AC) * ELE)) * AC) * ELD)) * AC) * UC;
                    let ELK = (EIE * ELI) + (ELJ * EID);
                    let ELL = LA + (EID * ELI);
                    let ELM = EID * ASO;
                    let ELN = EIE * ASO;
                    let ELO = EID * ASR;
                    let ELP = D - ELM;
                    let ELQ = D - (ELO * ELP);
                    let ELR = UC * (D - (ELM * ELQ));
                    let ELS = (((ELN * ELQ) + (((((EIE * ASR) * ELP) + ((ELN * AC) * ELO)) * AC) * ELM)) * AC) * UC;
                    let ELT = EIF * ELR;
                    let ELU = (EIG * ELR) + (ELS * EIF);
                    let ELV = EID * ASZ;
                    let ELW = ATB * EID;
                    let ELX = D - (ATD * EID);
                    let ELY = D - (ELW * ELX);
                    let ELZ = ATG * (D - (ELV * ELY));
                    let EMA = EIF * EIF;
                    let EMB = EIG * EIF;
                    let EMC = (EIH * ELR) - (EMA * ELZ);
                    let EMD = ((EII * ELR) + (ELS * EIH)) - (((EMB + EMB) * ELZ) + ((((((EIE * ASZ) * ELY) + (((((EIE * ATB) * ELX) + (((EIE * ATD) * AC) * ELW)) * AC) * ELV)) * AC) * ATG) * EMA));
                    let EMF = EME * EIF;
                    let EMG = EMF * ELI;
                    let EMH = ((EIG * EME) * ELI) + (ELJ * EMF);
                    let EMJ = EMI * EIH;
                    let EML = EMK * EIF;
                    let EMM = EML * EIF;
                    let EMN = LA - (ATV * EID);
                    let EMO = D - (ELE * EMN);
                    let EMP = (EMJ * ELI) + (EMM * EMO);
                    let EMQ = (((EII * EMI) * ELI) + (ELJ * EMJ)) + (((((EIG * EMK) * EIF) + (EIG * EML)) * EMO) + ((((ELF * EMN) + (((EIE * ATV) * AC) * ELE)) * AC) * EMM));
                    EMR = DZK;
                    EMS = DZL;
                    EMT = ELL;
                    EMU = ELT;
                    EMV = EMC;
                    EMW = EMG;
                    EMX = EMP;
                    EMY = DZR;
                    EMZ = DZS;
                    ENA = ELK;
                    ENB = ELU;
                    ENC = EMD;
                    END = EMH;
                    ENE = EMQ;
                }
                EJM = EMR;
                EJN = EMS;
                EJO = EMT;
                EJP = EMU;
                EJQ = EMV;
                EJR = EMW;
                EJS = EMX;
                EJT = EMY;
                EJU = EMZ;
                EJV = ENA;
                EJW = ENB;
                EJX = ENC;
                EJY = END;
                EJZ = ENE;
            }
            let EKA = if EID > AQJ { 1.0 } else { 0.0 };
            let ENO;
            let ENP;
            let ENQ;
            let ENR;
            if EKA != 0.0 {
                let ENF = LA - EJM;
                let ENG = D - (EJM * ENF);
                let ENH = (XQ * EID) / ENG;
                let ENI = ((EIE * XQ) - ((((EJT * ENF) + ((EJT * AC) * EJM)) * AC) * ENH)) / ENG;
                let ENJ = ENH * EJM;
                let ENK = (ENI * EJM) + (EJT * ENH);
                let ENL = (ENH.ln()) - EJN;
                let ENM = (ENI * (GY / ENH)) - EJU;
                ENO = ENJ;
                ENP = ENL;
                ENQ = ENK;
                ENR = ENM;
            } else {
                let ENN = if EID < -5e-3f64 { 1.0 } else { 0.0 };
                let EOJ;
                let EOK;
                let EOL;
                let EOM;
                if ENN != 0.0 {
                    let ENT = GO * EJN;
                    let ENU = ENT.sin();
                    let ENV = ENU * ENU;
                    let ENW = ((EJU * GO) * (ENT.cos())) * ENU;
                    let ENX = (-EID) / ENV;
                    let ENY = ((EIE * AC) - ((ENW + ENW) * ENX)) / ENV;
                    let ENZ = ENX.ln();
                    let EOA = ENY * (GY / ENX);
                    EOJ = ENX;
                    EOK = ENZ;
                    EOL = ENY;
                    EOM = EOA;
                } else {
                    let EOB = EID * WD;
                    let EOC = ATB * EID;
                    let EOD = D - (AVN * EID);
                    let EOE = D - (EOC * EOD);
                    let EOF = XQ - (EOB * EOE);
                    let EOG = (((EIE * WD) * EOE) + (((((EIE * ATB) * EOD) + (((EIE * AVN) * AC) * EOC)) * AC) * EOB)) * AC;
                    let EOH = EOF.ln();
                    let EOI = EOG * (GY / EOF);
                    EOJ = EOF;
                    EOK = EOH;
                    EOL = EOG;
                    EOM = EOI;
                }
                ENO = EOJ;
                ENP = EOK;
                ENQ = EOL;
                ENR = EOM;
            }
            let ENS = if ((AVB * EHL) + EJO) > B { 1.0 } else { 0.0 };
            let EPI;
            let EPJ;
            let EPK;
            let EPL;
            let EPM;
            let EPN;
            if ENS != 0.0 {
                let EON = EHL + EJO;
                let EOO = EHM + EJV;
                let EOP = AIR + EJP;
                let EOQ = AIT + EJW;
                EPI = EON;
                EPJ = EOP;
                EPK = EJQ;
                EPL = EOO;
                EPM = EOQ;
                EPN = EJX;
            } else {
                let EOR = EHL - EJO;
                let EOS = D / EOR;
                let EOT = (((EHM - EJV) * EOS) * AC) / EOR;
                let EOU = EJP - AIR;
                let EOV = EJW - AIT;
                let EOW = EIA - ENO;
                let EOX = EOW * EOS;
                let EOY = ((EIB - ENQ) * EOS) + (EOT * EOW);
                let EOZ = ((EOU * EOX) - EIA) - (EJR * ENO);
                let EPA = EOZ * EOS;
                let EPB = (((((EOV * EOX) + (EOY * EOU)) - EIB) - ((EJY * ENO) + (ENQ * EJR))) * EOS) + (EOT * EOZ);
                let EPC = LA * EOU;
                let EPD = EJY * EJR;
                let EPE = EJS + (EJR * EJR);
                let EPF = (((EJQ * EOX) + (EPC * EPA)) + EIA) - (EPE * ENO);
                let EPG = EPF * EOS;
                let EPH = ((((((EJX * EOX) + (EOY * EJQ)) + (((EOV * LA) * EPA) + (EPB * EPC))) + EIB) - (((EJZ + (EPD + EPD)) * ENO) + (ENQ * EPE))) * EOS) + (EOT * EPF);
                EPI = EOX;
                EPJ = EPA;
                EPK = EPG;
                EPL = EOY;
                EPM = EPB;
                EPN = EPH;
            }
            let EPO = if EPI > B { 1.0 } else { 0.0 };
            let EQI;
            let EQJ;
            let EQK;
            let EQL;
            let EQM;
            let EQN;
            if EPO != 0.0 {
                let EPP = EPI.ln();
                let EPQ = EPL * (GY / EPI);
                let EPR = D / EPI;
                let EPS = ((EPL * EPR) * AC) / EPI;
                let EPT = EPJ * EPR;
                let EPU = (EPM * EPR) + (EPS * EPJ);
                let EPV = EPU * EPT;
                let EPW = (EPK * EPR) - (EPT * EPT);
                let EPX = ((EPN * EPR) + (EPS * EPK)) - (EPV + EPV);
                EQI = EPP;
                EQJ = EPT;
                EQK = EPW;
                EQL = EPQ;
                EQM = EPU;
                EQN = EPX;
            } else {
                let EPY = -EHL;
                let EPZ = (EHL + HW) + (EPY.ln());
                let EQA = EHM + ((EHM * AC) * (GY / EPY));
                let EQB = D / EHJ;
                let EQC = ((EHK * EQB) * AC) / EHJ;
                let EQD = AIR + EQB;
                let EQE = AIT + EQC;
                let EQF = -EQB;
                let EQG = EQF * EQB;
                let EQH = ((EQC * AC) * EQB) + (EQC * EQF);
                EQI = EPZ;
                EQJ = EQD;
                EQK = EQG;
                EQL = EQA;
                EQM = EQE;
                EQN = EQH;
            }
            let EQO = ((AXZ + EHJ) + (LA * EQI)) - ENP;
            let EQP = (D + (LA * EQJ)) - EJR;
            let EQQ = (LA * EQK) - EJS;
            let EQR = EHL + (AIU * EQO);
            let EQS = EHM + ((AIW * EQO) + ((((AYA + EHK) + (EQL * LA)) - ENR) * AIU));
            let EQT = AIR + (AIU * EQP);
            let EQU = AIT + ((AIW * EQP) + (((EQM * LA) - EJY) * AIU));
            let EQV = AIU * EQQ;
            let EQW = (EQR * EPI) - EIA;
            let EQX = ((EQS * EPI) + (EPL * EQR)) - EIB;
            let EQY = ((EQT * EPI) + (EQR * EPJ)) + EIA;
            let EQZ = (((EQU * EPI) + (EPL * EQT)) + ((EQS * EPJ) + (EPM * EQR))) + EIB;
            let ERA = LA * EQT;
            let ERB = (((EQV * EPI) + (ERA * EPJ)) + (EQR * EPK)) - EIA;
            let ERC = EQZ * EQY;
            let ERD = GO * EQW;
            let ERE = (EQY * EQY) - (ERD * ERB);
            let ERF = (ERC + ERC) - (((EQX * GO) * ERB) + ((((((((AIW * EQQ) + (((EQN * LA) - EJZ) * AIU)) * EPI) + (EPL * EQV)) + (((EQU * LA) * EPJ) + (EPM * ERA))) + ((EQS * EPK) + (EPN * EQR))) - EIB) * ERD));
            let ERG = -EQW;
            let ERH = ERG * EQY;
            let ERI = ERF * ERE;
            let ERJ = (ERE * ERE) + AYW;
            let ERK = (ERH * ERE) / ERJ;
            let ERL = EHJ + ERK;
            let ERM = EHK + (((((((EQX * AC) * EQY) + (EQZ * ERG)) * ERE) + (ERF * ERH)) - ((ERI + ERI) * ERK)) / ERJ);
            let ERO;
            let ERP;
            let ERQ;
            let ERR;
            let ERS;
            let ERT;
            if A != 0.0 {
                let ERN = if (ERK.abs()) > NI { 1.0 } else { 0.0 };
                let ESE;
                let ESF;
                let ESG;
                let ESH;
                let ESI;
                let ESJ;
                if ERN != 0.0 {
                    let ERZ = AIR * ERL;
                    let ESA = (AIT * ERL) + (ERM * AIR);
                    let ESB = (AIH - ERL) - DGH;
                    let ESC = (AII - ERM) - DGJ;
                    let ESD = if ESB < SY { 1.0 } else { 0.0 };
                    let ESS;
                    let EST;
                    if ESD != 0.0 {
                        let ESK = ESB.exp();
                        let ESL = ESC * ESK;
                        ESS = ESK;
                        EST = ESL;
                    } else {
                        let ESM = ESB - SY;
                        let ESN = GO * ESM;
                        let ESO = D + (ESM * WD);
                        let ESP = D + (ESN * ESO);
                        let ESQ = XB * (D + (ESM * ESP));
                        let ESR = ((ESC * ESP) + ((((ESC * GO) * ESO) + ((ESC * WD) * ESN)) * ESM)) * XB;
                        ESS = ESQ;
                        EST = ESR;
                    }
                    let ESU = AJH * ESS;
                    let ESV = (AJJ * ESS) + (EST * AJH);
                    let ESW = ESA * ERZ;
                    let ESX = (ERZ * ERZ) - ESU;
                    let ESY = (ESW + ESW) - ESV;
                    let ESZ = (AOY * ERZ) + ESU;
                    let ETA = ((AOZ * ERZ) + (ESA * AOY)) + ESV;
                    let ETB = APC - ESU;
                    let ETC = APD - ESV;
                    let ETD = if ESX < -5e-3f64 { 1.0 } else { 0.0 };
                    let EUG;
                    let EUH;
                    let EUI;
                    let EUJ;
                    let EUK;
                    let EUL;
                    let EUM;
                    let EUN;
                    let EUO;
                    let EUP;
                    let EUQ;
                    let EUR;
                    let EUS;
                    let EUT;
                    if ETD != 0.0 {
                        let ETE = (ESX.abs()).sqrt();
                        let ETF = (ESY * ((GX * (if ESX >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * ETE));
                        let ETG = GO * ETE;
                        let ETH = ETG.tan();
                        let ETI = ETG.cos();
                        let ETJ = ETE / ETH;
                        let ETK = (ETF - (((ETF * GO) * (GY / (ETI * ETI))) * ETJ)) / ETH;
                        let ETL = (YY * ESZ) / ESX;
                        let ETM = ((ETA * YY) - (ESY * ETL)) / ESX;
                        let ETN = LA - ETJ;
                        let ETO = ESX + (ETJ * ETN);
                        let ETP = ETO * ETL;
                        let ETQ = ((ESY + ((ETK * ETN) + ((ETK * AC) * ETJ))) * ETL) + (ETM * ETO);
                        let ETR = LA * ETP;
                        let ETS = D + ETJ;
                        let ETT = ESZ - (ETR * ETS);
                        let ETU = (ETP * ETB) / ESZ;
                        let ETV = (ETT * ETL) + ETU;
                        let ETW = (((ETA - (((ETQ * LA) * ETS) + (ETK * ETR))) * ETL) + (ETM * ETT)) + ((((ETQ * ETB) + (ETC * ETP)) - (ETA * ETU)) / ESZ);
                        let ETX = D - (GO * ETJ);
                        let ETY = (ETK * GO) * AC;
                        let ETZ = ESZ / ESX;
                        let EUA = ETZ * ETX;
                        let EUB = (((ETA - (ESY * ETZ)) / ESX) * ETX) + (ETY * ETZ);
                        let EUC = EUA + (GO * ETP);
                        let EUD = ((ETB * ETX) - (ESZ * EUC)) / ESX;
                        let EUE = ((((ETC * ETX) + (ETY * ETB)) - ((ETA * EUC) + ((EUB + (ETQ * GO)) * ESZ))) - (ESY * EUD)) / ESX;
                        EUG = EJM;
                        EUH = ETE;
                        EUI = ETJ;
                        EUJ = ETP;
                        EUK = ETV;
                        EUL = EUA;
                        EUM = EUD;
                        EUN = EJT;
                        EUO = ETF;
                        EUP = ETK;
                        EUQ = ETQ;
                        EUR = ETW;
                        EUS = EUB;
                        EUT = EUE;
                    } else {
                        let EUF = if ESX > AQJ { 1.0 } else { 0.0 };
                        let EXL;
                        let EXM;
                        let EXN;
                        let EXO;
                        let EXP;
                        let EXQ;
                        let EXR;
                        let EXS;
                        let EXT;
                        let EXU;
                        let EXV;
                        let EXW;
                        let EXX;
                        let EXY;
                        if EUF != 0.0 {
                            let EUV = (ESX.abs()).sqrt();
                            let EUW = (ESY * ((GX * (if ESX >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * EUV));
                            let EUX = (-EUV).exp();
                            let EUY = (EUW * AC) * EUX;
                            let EUZ = D + EUX;
                            let EVA = D - EUX;
                            let EVB = (EUV * EUZ) / EVA;
                            let EVC = (((EUW * EUZ) + (EUY * EUV)) - ((EUY * AC) * EVB)) / EVA;
                            let EVD = (YY * ESZ) / ESX;
                            let EVE = ((ETA * YY) - (ESY * EVD)) / ESX;
                            let EVF = LA - EVB;
                            let EVG = ESX + (EVB * EVF);
                            let EVH = EVG * EVD;
                            let EVI = ((ESY + ((EVC * EVF) + ((EVC * AC) * EVB))) * EVD) + (EVE * EVG);
                            let EVJ = LA * EVH;
                            let EVK = D + EVB;
                            let EVL = ESZ - (EVJ * EVK);
                            let EVM = (EVH * ETB) / ESZ;
                            let EVN = (EVL * EVD) + EVM;
                            let EVO = (((ETA - (((EVI * LA) * EVK) + (EVC * EVJ))) * EVD) + (EVE * EVL)) + ((((EVI * ETB) + (ETC * EVH)) - (ETA * EVM)) / ESZ);
                            let EVP = D - (GO * EVB);
                            let EVQ = (EVC * GO) * AC;
                            let EVR = ESZ / ESX;
                            let EVS = EVR * EVP;
                            let EVT = (((ETA - (ESY * EVR)) / ESX) * EVP) + (EVQ * EVR);
                            let EVU = EVS + (GO * EVH);
                            let EVV = ((ETB * EVP) - (ESZ * EVU)) / ESX;
                            let EVW = ((((ETC * EVP) + (EVQ * ETB)) - ((ETA * EVU) + ((EVT + (EVI * GO)) * ESZ))) - (ESY * EVV)) / ESX;
                            EXL = EUX;
                            EXM = EUV;
                            EXN = EVB;
                            EXO = EVH;
                            EXP = EVN;
                            EXQ = EVS;
                            EXR = EVV;
                            EXS = EUY;
                            EXT = EUW;
                            EXU = EVC;
                            EXV = EVI;
                            EXW = EVO;
                            EXX = EVT;
                            EXY = EVW;
                        } else {
                            let EVX = ESX * ASC;
                            let EVY = ESX * ASE;
                            let EVZ = ESY * ASE;
                            let EWA = D - (ESX * ASH);
                            let EWB = D - (EVY * EWA);
                            let EWC = UC * (D - (EVX * EWB));
                            let EWD = ((((ESY * ASC) * EWB) + ((((EVZ * EWA) + (((ESY * ASH) * AC) * EVY)) * AC) * EVX)) * AC) * UC;
                            let EWE = (ESY * EWC) + (EWD * ESX);
                            let EWF = LA + (ESX * EWC);
                            let EWG = ESX * ASO;
                            let EWH = ESY * ASO;
                            let EWI = ESX * ASR;
                            let EWJ = D - EWG;
                            let EWK = D - (EWI * EWJ);
                            let EWL = UC * (D - (EWG * EWK));
                            let EWM = (((EWH * EWK) + (((((ESY * ASR) * EWJ) + ((EWH * AC) * EWI)) * AC) * EWG)) * AC) * UC;
                            let EWN = ESZ * EWL;
                            let EWO = (ETA * EWL) + (EWM * ESZ);
                            let EWP = ESX * ASZ;
                            let EWQ = ATB * ESX;
                            let EWR = D - (ATD * ESX);
                            let EWS = D - (EWQ * EWR);
                            let EWT = ATG * (D - (EWP * EWS));
                            let EWU = ESZ * ESZ;
                            let EWV = ETA * ESZ;
                            let EWW = (ETB * EWL) - (EWU * EWT);
                            let EWX = ((ETC * EWL) + (EWM * ETB)) - (((EWV + EWV) * EWT) + ((((((ESY * ASZ) * EWS) + (((((ESY * ATB) * EWR) + (((ESY * ATD) * AC) * EWQ)) * AC) * EWP)) * AC) * ATG) * EWU));
                            let EWZ = EWY * ESZ;
                            let EXA = EWZ * EWC;
                            let EXB = ((ETA * EWY) * EWC) + (EWD * EWZ);
                            let EXD = EXC * ETB;
                            let EXF = EXE * ESZ;
                            let EXG = EXF * ESZ;
                            let EXH = LA - (ATV * ESX);
                            let EXI = D - (EVY * EXH);
                            let EXJ = (EXD * EWC) + (EXG * EXI);
                            let EXK = (((ETC * EXC) * EWC) + (EWD * EXD)) + (((((ETA * EXE) * ESZ) + (ETA * EXF)) * EXI) + ((((EVZ * EXH) + (((ESY * ATV) * AC) * EVY)) * AC) * EXG));
                            EXL = EJM;
                            EXM = EJN;
                            EXN = EWF;
                            EXO = EWN;
                            EXP = EWW;
                            EXQ = EXA;
                            EXR = EXJ;
                            EXS = EJT;
                            EXT = EJU;
                            EXU = EWE;
                            EXV = EWO;
                            EXW = EWX;
                            EXX = EXB;
                            EXY = EXK;
                        }
                        EUG = EXL;
                        EUH = EXM;
                        EUI = EXN;
                        EUJ = EXO;
                        EUK = EXP;
                        EUL = EXQ;
                        EUM = EXR;
                        EUN = EXS;
                        EUO = EXT;
                        EUP = EXU;
                        EUQ = EXV;
                        EUR = EXW;
                        EUS = EXX;
                        EUT = EXY;
                    }
                    let EUU = if ESX > AQJ { 1.0 } else { 0.0 };
                    let EYI;
                    let EYJ;
                    let EYK;
                    let EYL;
                    if EUU != 0.0 {
                        let EXZ = LA - EUG;
                        let EYA = D - (EUG * EXZ);
                        let EYB = (XQ * ESX) / EYA;
                        let EYC = ((ESY * XQ) - ((((EUN * EXZ) + ((EUN * AC) * EUG)) * AC) * EYB)) / EYA;
                        let EYD = EYB * EUG;
                        let EYE = (EYC * EUG) + (EUN * EYB);
                        let EYF = (EYB.ln()) - EUH;
                        let EYG = (EYC * (GY / EYB)) - EUO;
                        EYI = EYD;
                        EYJ = EYF;
                        EYK = EYE;
                        EYL = EYG;
                    } else {
                        let EYH = if ESX < -5e-3f64 { 1.0 } else { 0.0 };
                        let EZD;
                        let EZE;
                        let EZF;
                        let EZG;
                        if EYH != 0.0 {
                            let EYN = GO * EUH;
                            let EYO = EYN.sin();
                            let EYP = EYO * EYO;
                            let EYQ = ((EUO * GO) * (EYN.cos())) * EYO;
                            let EYR = (-ESX) / EYP;
                            let EYS = ((ESY * AC) - ((EYQ + EYQ) * EYR)) / EYP;
                            let EYT = EYR.ln();
                            let EYU = EYS * (GY / EYR);
                            EZD = EYR;
                            EZE = EYT;
                            EZF = EYS;
                            EZG = EYU;
                        } else {
                            let EYV = ESX * WD;
                            let EYW = ATB * ESX;
                            let EYX = D - (AVN * ESX);
                            let EYY = D - (EYW * EYX);
                            let EYZ = XQ - (EYV * EYY);
                            let EZA = (((ESY * WD) * EYY) + (((((ESY * ATB) * EYX) + (((ESY * AVN) * AC) * EYW)) * AC) * EYV)) * AC;
                            let EZB = EYZ.ln();
                            let EZC = EZA * (GY / EYZ);
                            EZD = EYZ;
                            EZE = EZB;
                            EZF = EZA;
                            EZG = EZC;
                        }
                        EYI = EZD;
                        EYJ = EZE;
                        EYK = EZF;
                        EYL = EZG;
                    }
                    let EYM = if ((AVB * ERZ) + EUI) > B { 1.0 } else { 0.0 };
                    let FAC;
                    let FAD;
                    let FAE;
                    let FAF;
                    let FAG;
                    let FAH;
                    if EYM != 0.0 {
                        let EZH = ERZ + EUI;
                        let EZI = ESA + EUP;
                        let EZJ = AIR + EUJ;
                        let EZK = AIT + EUQ;
                        FAC = EZH;
                        FAD = EZJ;
                        FAE = EUK;
                        FAF = EZI;
                        FAG = EZK;
                        FAH = EUR;
                    } else {
                        let EZL = ERZ - EUI;
                        let EZM = D / EZL;
                        let EZN = (((ESA - EUP) * EZM) * AC) / EZL;
                        let EZO = EUJ - AIR;
                        let EZP = EUQ - AIT;
                        let EZQ = ESU - EYI;
                        let EZR = EZQ * EZM;
                        let EZS = ((ESV - EYK) * EZM) + (EZN * EZQ);
                        let EZT = ((EZO * EZR) - ESU) - (EUL * EYI);
                        let EZU = EZT * EZM;
                        let EZV = (((((EZP * EZR) + (EZS * EZO)) - ESV) - ((EUS * EYI) + (EYK * EUL))) * EZM) + (EZN * EZT);
                        let EZW = LA * EZO;
                        let EZX = EUS * EUL;
                        let EZY = EUM + (EUL * EUL);
                        let EZZ = (((EUK * EZR) + (EZW * EZU)) + ESU) - (EZY * EYI);
                        let FAA = EZZ * EZM;
                        let FAB = ((((((EUR * EZR) + (EZS * EUK)) + (((EZP * LA) * EZU) + (EZV * EZW))) + ESV) - (((EUT + (EZX + EZX)) * EYI) + (EYK * EZY))) * EZM) + (EZN * EZZ);
                        FAC = EZR;
                        FAD = EZU;
                        FAE = FAA;
                        FAF = EZS;
                        FAG = EZV;
                        FAH = FAB;
                    }
                    let FAI = if FAC > B { 1.0 } else { 0.0 };
                    let FBC;
                    let FBD;
                    let FBE;
                    let FBF;
                    let FBG;
                    let FBH;
                    if FAI != 0.0 {
                        let FAJ = FAC.ln();
                        let FAK = FAF * (GY / FAC);
                        let FAL = D / FAC;
                        let FAM = ((FAF * FAL) * AC) / FAC;
                        let FAN = FAD * FAL;
                        let FAO = (FAG * FAL) + (FAM * FAD);
                        let FAP = FAO * FAN;
                        let FAQ = (FAE * FAL) - (FAN * FAN);
                        let FAR = ((FAH * FAL) + (FAM * FAE)) - (FAP + FAP);
                        FBC = FAJ;
                        FBD = FAN;
                        FBE = FAQ;
                        FBF = FAK;
                        FBG = FAO;
                        FBH = FAR;
                    } else {
                        let FAS = -ERZ;
                        let FAT = (ERZ + HW) + (FAS.ln());
                        let FAU = ESA + ((ESA * AC) * (GY / FAS));
                        let FAV = D / ERL;
                        let FAW = ((ERM * FAV) * AC) / ERL;
                        let FAX = AIR + FAV;
                        let FAY = AIT + FAW;
                        let FAZ = -FAV;
                        let FBA = FAZ * FAV;
                        let FBB = ((FAW * AC) * FAV) + (FAW * FAZ);
                        FBC = FAT;
                        FBD = FAX;
                        FBE = FBA;
                        FBF = FAU;
                        FBG = FAY;
                        FBH = FBB;
                    }
                    let FBI = ((AXZ + ERL) + (LA * FBC)) - EYJ;
                    let FBJ = (D + (LA * FBD)) - EUL;
                    let FBK = (LA * FBE) - EUM;
                    let FBL = ERZ + (AIU * FBI);
                    let FBM = ESA + ((AIW * FBI) + ((((AYA + ERM) + (FBF * LA)) - EYL) * AIU));
                    let FBN = AIR + (AIU * FBJ);
                    let FBO = AIT + ((AIW * FBJ) + (((FBG * LA) - EUS) * AIU));
                    let FBP = AIU * FBK;
                    let FBQ = (FBL * FAC) - ESU;
                    let FBR = ((FBM * FAC) + (FAF * FBL)) - ESV;
                    let FBS = ((FBN * FAC) + (FBL * FAD)) + ESU;
                    let FBT = (((FBO * FAC) + (FAF * FBN)) + ((FBM * FAD) + (FAG * FBL))) + ESV;
                    let FBU = LA * FBN;
                    let FBV = (((FBP * FAC) + (FBU * FAD)) + (FBL * FAE)) - ESU;
                    let FBW = FBT * FBS;
                    let FBX = GO * FBQ;
                    let FBY = (FBS * FBS) - (FBX * FBV);
                    let FBZ = (FBW + FBW) - (((FBR * GO) * FBV) + ((((((((AIW * FBK) + (((FBH * LA) - EUT) * AIU)) * FAC) + (FAF * FBP)) + (((FBO * LA) * FAD) + (FAG * FBU))) + ((FBM * FAE) + (FAH * FBL))) - ESV) * FBX));
                    let FCA = -FBQ;
                    let FCB = FCA * FBS;
                    let FCC = FBZ * FBY;
                    let FCD = (FBY * FBY) + AYW;
                    let FCE = (FCB * FBY) / FCD;
                    let FCF = ERL + FCE;
                    let FCG = ERM + (((((((FBR * AC) * FBS) + (FBT * FCA)) * FBY) + (FBZ * FCB)) - ((FCC + FCC) * FCE)) / FCD);
                    ESE = FCF;
                    ESF = EUG;
                    ESG = EUH;
                    ESH = FCG;
                    ESI = EUN;
                    ESJ = EUO;
                } else {
                    ESE = ERL;
                    ESF = EJM;
                    ESG = EJN;
                    ESH = ERM;
                    ESI = EJT;
                    ESJ = EJU;
                }
                ERO = ESE;
                ERP = ESF;
                ERQ = ESG;
                ERR = ESH;
                ERS = ESI;
                ERT = ESJ;
            } else {
                ERO = ERL;
                ERP = EJM;
                ERQ = EJN;
                ERR = ERM;
                ERS = EJT;
                ERT = EJU;
            }
            let ERU = AIR * ERO;
            let ERV = (AIT * ERO) + (ERR * AIR);
            let ERW = (AIH - ERO) - DGH;
            let ERX = (AII - ERR) - DGJ;
            let ERY = if ERW < SY { 1.0 } else { 0.0 };
            let FCP;
            let FCQ;
            if ERY != 0.0 {
                let FCH = ERW.exp();
                let FCI = ERX * FCH;
                FCP = FCH;
                FCQ = FCI;
            } else {
                let FCJ = ERW - SY;
                let FCK = GO * FCJ;
                let FCL = D + (FCJ * WD);
                let FCM = D + (FCK * FCL);
                let FCN = XB * (D + (FCJ * FCM));
                let FCO = ((ERX * FCM) + ((((ERX * GO) * FCL) + ((ERX * WD) * FCK)) * FCJ)) * XB;
                FCP = FCN;
                FCQ = FCO;
            }
            let FCR = AJH * FCP;
            let FCS = (AJJ * FCP) + (FCQ * AJH);
            let FCT = ERV * ERU;
            let FCU = (ERU * ERU) - FCR;
            let FCV = (FCT + FCT) - FCS;
            let FCW = if FCR <= B { 1.0 } else { 0.0 };
            let FDC;
            let FDD;
            let FDE;
            let FDF;
            let FDG;
            let FDH;
            if FCW != 0.0 {
                let FCX = CJQ - ERU;
                let FCY = ERV * AC;
                let FCZ = FCX / AIU;
                let FDA = (FCY - (AIW * FCZ)) / AIU;
                FDC = FCZ;
                FDD = FCX;
                FDE = CJQ;
                FDF = FDA;
                FDG = FCY;
                FDH = AFD;
            } else {
                let FDB = if FCU < -5e-3f64 { 1.0 } else { 0.0 };
                let FDT;
                let FDU;
                let FDV;
                let FDW;
                let FDX;
                let FDY;
                if FDB != 0.0 {
                    let FDL = (FCU.abs()).sqrt();
                    let FDM = (FCV * ((GX * (if FCU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * FDL));
                    let FDN = GO * FDL;
                    let FDO = FDN.tan();
                    let FDP = FDN.cos();
                    let FDQ = FDL / FDO;
                    let FDR = (FDM - (((FDM * GO) * (GY / (FDP * FDP))) * FDQ)) / FDO;
                    FDT = FDQ;
                    FDU = ERP;
                    FDV = FDL;
                    FDW = FDR;
                    FDX = ERS;
                    FDY = FDM;
                } else {
                    let FDS = if FCU > AQJ { 1.0 } else { 0.0 };
                    let FEO;
                    let FEP;
                    let FEQ;
                    let FER;
                    let FES;
                    let FET;
                    if FDS != 0.0 {
                        let FEA = (FCU.abs()).sqrt();
                        let FEB = (FCV * ((GX * (if FCU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * FEA));
                        let FEC = (-FEA).exp();
                        let FED = (FEB * AC) * FEC;
                        let FEE = D + FEC;
                        let FEF = D - FEC;
                        let FEG = (FEA * FEE) / FEF;
                        let FEH = (((FEB * FEE) + (FED * FEA)) - ((FED * AC) * FEG)) / FEF;
                        FEO = FEG;
                        FEP = FEC;
                        FEQ = FEA;
                        FER = FEH;
                        FES = FED;
                        FET = FEB;
                    } else {
                        let FEI = FCU * UC;
                        let FEJ = FCU * ASC;
                        let FEK = D - (FCU * ASE);
                        let FEL = D - (FEJ * FEK);
                        let FEM = ((FCV * UC) * FEL) + (((((FCV * ASC) * FEK) + (((FCV * ASE) * AC) * FEJ)) * AC) * FEI);
                        let FEN = LA + (FEI * FEL);
                        FEO = FEN;
                        FEP = ERP;
                        FEQ = ERQ;
                        FER = FEM;
                        FES = ERS;
                        FET = ERT;
                    }
                    FDT = FEO;
                    FDU = FEP;
                    FDV = FEQ;
                    FDW = FER;
                    FDX = FES;
                    FDY = FET;
                }
                let FDZ = if ((AVB * ERU) + FDT) > B { 1.0 } else { 0.0 };
                let FEY;
                let FEZ;
                let FFA;
                let FFB;
                let FFC;
                let FFD;
                if FDZ != 0.0 {
                    let FEU = ERU + FDT;
                    let FEV = ERV + FDW;
                    let FEW = if (FCR * ERU) < (((CLQ * ERU) * ERU) * FEU) { 1.0 } else { 0.0 };
                    let FFM;
                    let FFN;
                    let FFO;
                    let FFP;
                    let FFQ;
                    let FFR;
                    if FEW != 0.0 {
                        let FFE = FCR / FEU;
                        let FFF = (FCS - (FEV * FFE)) / FEU;
                        let FFG = FFE + CJQ;
                        let FFH = FFG - ERU;
                        let FFI = FFF - ERV;
                        let FFJ = FFH / AIU;
                        let FFK = (FFI - (AIW * FFJ)) / AIU;
                        FFM = FFJ;
                        FFN = FFH;
                        FFO = FFG;
                        FFP = FFK;
                        FFQ = FFI;
                        FFR = FFF;
                    } else {
                        let FFL = if FCU > AQJ { 1.0 } else { 0.0 };
                        let FFY;
                        let FFZ;
                        if FFL != 0.0 {
                            let FFS = LA - FDU;
                            let FFT = D - (FDU * FFS);
                            let FFU = (XQ * FCU) / FFT;
                            let FFV = (FFU.ln()) - FDV;
                            let FFW = ((((FCV * XQ) - ((((FDX * FFS) + ((FDX * AC) * FDU)) * AC) * FFU)) / FFT) * (GY / FFU)) - FDY;
                            FFY = FFV;
                            FFZ = FFW;
                        } else {
                            let FFX = if FCU < -5e-3f64 { 1.0 } else { 0.0 };
                            let FGU;
                            let FGV;
                            if FFX != 0.0 {
                                let FGG = GO * FDV;
                                let FGH = FGG.sin();
                                let FGI = FGH * FGH;
                                let FGJ = ((FDY * GO) * (FGG.cos())) * FGH;
                                let FGK = (-FCU) / FGI;
                                let FGL = FGK.ln();
                                let FGM = (((FCV * AC) - ((FGJ + FGJ) * FGK)) / FGI) * (GY / FGK);
                                FGU = FGL;
                                FGV = FGM;
                            } else {
                                let FGN = FCU * WD;
                                let FGO = ATB * FCU;
                                let FGP = D - (AVN * FCU);
                                let FGQ = D - (FGO * FGP);
                                let FGR = XQ - (FGN * FGQ);
                                let FGS = FGR.ln();
                                let FGT = ((((FCV * WD) * FGQ) + (((((FCV * ATB) * FGP) + (((FCV * AVN) * AC) * FGO)) * AC) * FGN)) * AC) * (GY / FGR);
                                FGU = FGS;
                                FGV = FGT;
                            }
                            FFY = FGU;
                            FFZ = FGV;
                        }
                        let FGA = ((AXZ + ERO) + (LA * (FEU.ln()))) - FFY;
                        let FGB = ((AYA + ERR) + ((FEV * (GY / FEU)) * LA)) - FFZ;
                        let FGC = AIU * FGA;
                        let FGD = (AIW * FGA) + (FGB * AIU);
                        let FGE = ERU + FGC;
                        let FGF = ERV + FGD;
                        FFM = FGA;
                        FFN = FGC;
                        FFO = FGE;
                        FFP = FGB;
                        FFQ = FGD;
                        FFR = FGF;
                    }
                    FEY = FFM;
                    FEZ = FFN;
                    FFA = FFO;
                    FFB = FFP;
                    FFC = FFQ;
                    FFD = FFR;
                } else {
                    let FEX = if FCU > AQJ { 1.0 } else { 0.0 };
                    let FHA;
                    let FHB;
                    if FEX != 0.0 {
                        let FGW = ((ERO + DGH) - AIH) - FDV;
                        let FGX = ((ERR + DGJ) - AII) - FDY;
                        let FGY = if FGW < SY { 1.0 } else { 0.0 };
                        let FHS;
                        let FHT;
                        if FGY != 0.0 {
                            let FHK = FGW.exp();
                            let FHL = FGX * FHK;
                            FHS = FHK;
                            FHT = FHL;
                        } else {
                            let FHM = FGW - SY;
                            let FHN = GO * FHM;
                            let FHO = D + (FHM * WD);
                            let FHP = D + (FHN * FHO);
                            let FHQ = XB * (D + (FHM * FHP));
                            let FHR = ((FGX * FHP) + ((((FGX * GO) * FHO) + ((FGX * WD) * FHN)) * FHM)) * XB;
                            FHS = FHQ;
                            FHT = FHR;
                        }
                        let FHU = FHS / AJH;
                        let FHV = XQ * FCU;
                        let FHW = LA - FDU;
                        let FHX = D - (FDU * FHW);
                        let FHY = (FHV * FHU) / FHX;
                        let FHZ = ((((FCV * XQ) * FHU) + (((FHT - (AJJ * FHU)) / AJH) * FHV)) - ((((FDX * FHW) + ((FDX * AC) * FDU)) * AC) * FHY)) / FHX;
                        FHA = FHY;
                        FHB = FHZ;
                    } else {
                        let FGZ = if FCU < -5e-3f64 { 1.0 } else { 0.0 };
                        let FIN;
                        let FIO;
                        if FGZ != 0.0 {
                            let FIA = GO * FDV;
                            let FIB = FIA.sin();
                            let FIC = FIB * FIB;
                            let FID = ((FDY * GO) * (FIA.cos())) * FIB;
                            let FIE = (-FCU) / FIC;
                            let FIF = FIE / FCR;
                            let FIG = ((((FCV * AC) - ((FID + FID) * FIE)) / FIC) - (FCS * FIF)) / FCR;
                            FIN = FIF;
                            FIO = FIG;
                        } else {
                            let FIH = FCU * WD;
                            let FII = ATB * FCU;
                            let FIJ = D - (AVN * FCU);
                            let FIK = D - (FII * FIJ);
                            let FIL = (XQ - (FIH * FIK)) / FCR;
                            let FIM = (((((FCV * WD) * FIK) + (((((FCV * ATB) * FIJ) + (((FCV * AVN) * AC) * FII)) * AC) * FIH)) * AC) - (FCS * FIL)) / FCR;
                            FIN = FIL;
                            FIO = FIM;
                        }
                        FHA = FIN;
                        FHB = FIO;
                    }
                    let FHC = D - FHA;
                    let FHD = (ERU - FDT) / FHC;
                    let FHE = ((ERV - FDW) - ((FHB * AC) * FHD)) / FHC;
                    let FHF = FHD + CJQ;
                    let FHG = FHF - ERU;
                    let FHH = FHE - ERV;
                    let FHI = FHG / AIU;
                    let FHJ = (FHH - (AIW * FHI)) / AIU;
                    FEY = FHI;
                    FEZ = FHG;
                    FFA = FHF;
                    FFB = FHJ;
                    FFC = FHH;
                    FFD = FHE;
                }
                FDC = FEY;
                FDD = FEZ;
                FDE = FFA;
                FDF = FFB;
                FDG = FFC;
                FDH = FFD;
            }
            let FDI = (AIP - FDC) - DGH;
            let FDJ = (AIQ - FDF) - DGJ;
            let FDK = if FDI < SY { 1.0 } else { 0.0 };
            let FIX;
            let FIY;
            if FDK != 0.0 {
                let FIP = FDI.exp();
                let FIQ = FDJ * FIP;
                FIX = FIP;
                FIY = FIQ;
            } else {
                let FIR = FDI - SY;
                let FIS = GO * FIR;
                let FIT = D + (FIR * WD);
                let FIU = D + (FIS * FIT);
                let FIV = XB * (D + (FIR * FIU));
                let FIW = ((FDJ * FIU) + ((((FDJ * GO) * FIT) + ((FDJ * WD) * FIS)) * FIR)) * XB;
                FIX = FIV;
                FIY = FIW;
            }
            let FIZ = AJH * FIX;
            let FJA = (AJJ * FIX) + (FIY * AJH);
            let FJM;
            let FJN;
            let FJO;
            let FJP;
            let FJQ;
            let FJR;
            let FJS;
            let FJT;
            if CPX != 0.0 {
                let FJB = FCR * AIX;
                let FJC = (FCS * AIX) + (AIY * FCR);
                let FJD = FIZ * AIZ;
                let FJE = (FJA * AIZ) + (AJA * FIZ);
                let FJF = FJB + (LA * ERU);
                let FJG = FJC + (ERV * LA);
                let FJH = FJD + (LA * FDD);
                let FJI = FJE + (FDG * LA);
                let FJJ = ((LA * FDE) + FJB) + FJD;
                let FJK = ((FDH * LA) + FJC) + FJE;
                let FJL = if (FCU.abs()) > AQJ { 1.0 } else { 0.0 };
                let FKW;
                let FKX;
                if FJL != 0.0 {
                    let FKA = LA * (ERO + LA);
                    let FKB = LA * (FDC + LA);
                    let FKC = ((FJF * FJH) + (FKA * FJH)) + (FKB * FJF);
                    let FKE = FKD * FCU;
                    let FKF = FDE * FKC;
                    let FKG = (FKE * FJJ) / FKF;
                    let FKH = ((((FCV * FKD) * FJJ) + (FJK * FKE)) - (((FDH * FKC) + (((((FJG * FJH) + (FJI * FJF)) + (((ERR * LA) * FJH) + (FJI * FKA))) + (((FDF * LA) * FJF) + (FJG * FKB))) * FDE)) * FKG)) / FKF;
                    FKW = FKG;
                    FKX = FKH;
                } else {
                    let FKI = FCU * ASO;
                    let FKJ = FCV * ASO;
                    let FKK = FCU * ASR;
                    let FKL = D - FKI;
                    let FKM = D - (FKK * FKL);
                    let FKN = UC * (D - (FKI * FKM));
                    let FKO = FJF * FJH;
                    let FKP = FKO * FDE;
                    let FKQ = D + (FDE * FKN);
                    let FKR = ((FJF * FCR) + (FJH * FIZ)) + (FKP * FKQ);
                    let FKS = FCR * FIZ;
                    let FKT = FDE * FKR;
                    let FKU = (FKS * FJJ) / FKT;
                    let FKV = (((((FCS * FIZ) + (FJA * FCR)) * FJJ) + (FJK * FKS)) - (((FDH * FKR) + (((((FJG * FCR) + (FCS * FJF)) + ((FJI * FIZ) + (FJA * FJH))) + ((((((FJG * FJH) + (FJI * FJF)) * FDE) + (FDH * FKO)) * FKQ) + (((FDH * FKN) + (((((FKJ * FKM) + (((((FCV * ASR) * FKL) + ((FKJ * AC) * FKK)) * AC) * FKI)) * AC) * UC) * FDE)) * FKP))) * FDE)) * FKU)) / FKT;
                    FKW = FKU;
                    FKX = FKV;
                }
                FJM = FJH;
                FJN = FJF;
                FJO = FKW;
                FJP = FJJ;
                FJQ = FJI;
                FJR = FJG;
                FJS = FKX;
                FJT = FJK;
            } else {
                FJM = B;
                FJN = B;
                FJO = B;
                FJP = B;
                FJQ = AFD;
                FJR = AFD;
                FJS = AFD;
                FJT = AFD;
            }
            let FJU = DGH + (FDE.ln());
            let FJV = DGJ + (FDH * (GY / FDE));
            let FJW = GO * (CJX + FDE);
            let FJX = (CKA + FDH) * GO;
            let FJY = FJU - CQR;
            let FJZ = FJV - CQS;
            let FLN;
            let FLO;
            if K != 0.0 {
                let FKY = (GO * (BYN + ERU)) / AIR;
                let FKZ = (((BYO + ERV) * GO) - (AIT * FKY)) / AIR;
                let FLB = FKY - FLA;
                let FLC = FKZ * FLB;
                let FLD = ((FLB * FLB) + D).sqrt();
                let FLE = GO * ((FKY + FLA) + FLD);
                let FLF = (FKZ + ((FLC + FLC) * (GY / (GX * FLD)))) * GO;
                let FLG = FLE / CH;
                let FLH = (FLG + staged[192]).sqrt();
                let FLI = FLH - staged[193];
                let FLJ = FLI * FLI;
                let FLK = (FLJ * CH) / FLE;
                let FLL = D - FLK;
                let FLM = ((((((((FLF - Lanes([(EF * FLG), 0.0, 0.0, 0.0, 0.0])) / CH) * (GY / (GX * FLH))) * (LA * FLI)) * CH) + Lanes([(EF * FLJ), 0.0, 0.0, 0.0, 0.0])) - (FLF * FLK)) / FLE) * AC;
                FLN = FLL;
                FLO = FLM;
            } else {
                FLN = D;
                FLO = AFD;
            }
            let FLP = ERU / LA;
            let FLQ = ERV / LA;
            let FLR = if FLP < SY { 1.0 } else { 0.0 };
            let FLW;
            let FLX;
            if FLR != 0.0 {
                let FLS = FLP.exp();
                let FLT = D + FLS;
                let FLU = FLT.ln();
                let FLV = (FLQ * FLS) * (GY / FLT);
                FLW = FLU;
                FLX = FLV;
            } else {
                FLW = FLP;
                FLX = FLQ;
            }
            let FLY = LA * FLW;
            let FLZ = FLX * LA;
            let FMA = FDD / LA;
            let FMB = FDG / LA;
            let FMC = if FMA < SY { 1.0 } else { 0.0 };
            let FMH;
            let FMI;
            if FMC != 0.0 {
                let FMD = FMA.exp();
                let FME = D + FMD;
                let FMF = FME.ln();
                let FMG = (FMB * FMD) * (GY / FME);
                FMH = FMF;
                FMI = FMG;
            } else {
                FMH = FMA;
                FMI = FMB;
            }
            let FMJ = LA * FMH;
            let FMK = FMI * LA;
            let FML = FMJ - FDD;
            let FMM = FMK - FDG;
            let FMN = FLY - ERU;
            let FMO = FLZ - ERV;
            let FMP = GO * (CSA + FLY);
            let FMQ = (CSB + FLZ) * GO;
            let FMR = GO * (CSL + FMJ);
            let FMS = (CSM + FMK) * GO;
            let FMT = FMP + FMR;
            let FMU = FMQ + FMS;
            let FMV = D / FMT;
            let FMW = ((FMU * FMV) * AC) / FMT;
            let FMX = FJW * FMP;
            let FMY = FMX * FMV;
            let FMZ = (((FJX * FMP) + (FMQ * FJW)) * FMV) + (FMW * FMX);
            let FNA = FJW * FMR;
            let FNB = FNA * FMV;
            let FNC = (((FJX * FMR) + (FMS * FJW)) * FMV) + (FMW * FNA);
            let FND = GO * (CSN + FML);
            let FNE = (CSO + FMM) * GO;
            let FNF = GO * (CSP + FMN);
            let FNG = (CSQ + FMO) * GO;
            let FNH = GO * (CST + ((CSR * FLY) + (CSS * FML)));
            let FNI = (CSU + ((FLZ * CSR) + (FMM * CSS))) * GO;
            let FNJ = GO * (CSV + ((CSR * FMJ) + (CSS * FMN)));
            let FNK = (CSW + ((FMK * CSR) + (FMO * CSS))) * GO;
            let FNL = FMP * JO;
            let FNM = FNL * CTB;
            let FNN = FNM * FLN;
            let FNO = ((((FMQ * JO) * CTB) + Lanes([(CTC * FNL), 0.0, 0.0, 0.0, 0.0])) * FLN) + (FLO * FNM);
            let FNP = FMR * JR;
            let FNQ = FNP * CTB;
            let FNR = ((FMS * JR) * CTB) + Lanes([(CTC * FNP), 0.0, 0.0, 0.0, 0.0]);
            let FNS = FNN + FNQ;
            let FNT = FNO + FNR;
            let FNU = FND + (CTI * FNF);
            let FNV = CU * FNU;
            let FNW = Lanes([(ES * FNU), 0.0, 0.0, 0.0, 0.0]) + ((FNE + (FNG * CTI)) * CU);
            let FNX = D + FNV;
            let FNY = FNW * FNX;
            let FNZ = ((FNX * FNX) + NI).sqrt();
            let FOA = FNW * CTP;
            let FOB = D + (CTP * FNV);
            let FOC = FOA * FOB;
            let FOD = ((FOB * FOB) + NI).sqrt();
            let FOE = GO * (FOB + FOD);
            let FOF = (GO * (FNX + FNZ)) / FOE;
            let FOG = (((FNW + ((FNY + FNY) * (GY / (GX * FNZ)))) * GO) - (((FOA + ((FOC + FOC) * (GY / (GX * FOD)))) * GO) * FOF)) / FOE;
            let FOH = (D + (CTX * FND)) + (CTY * FNF);
            let FOI = CV * FOH;
            let FOJ = (D + (FMY * CUD)) + (FNB * CUE);
            let FOK = FOJ.ln();
            let FOL = (CUB * FOK).exp();
            let FOM = FOI * FOL;
            let FON = ((Lanes([(ET * FOH), 0.0, 0.0, 0.0, 0.0]) + (((FNE * CTX) + (FNG * CTY)) * CV)) * FOL) + (((Lanes([(CUC * FOK), 0.0, 0.0, 0.0, 0.0]) + ((((FMZ * CUD) + (FNC * CUE)) * (GY / FOJ)) * CUB)) * FOL) * FOI);
            let FOP;
            let FOQ;
            if CUK != 0.0 {
                FOP = D;
                FOQ = AFD;
            } else {
                let FPV;
                let FPW;
                if FOO != 0.0 {
                    let FPM = FJW + CVS;
                    let FPN = (CVU * (FPM.ln())).exp();
                    let FPO = D - (CVW * FPN);
                    let FPP = ((((FJX * (GY / FPM)) * CVU) * FPN) * CVW) * AC;
                    FPV = FPO;
                    FPW = FPP;
                } else {
                    let FPQ = FJW + CVS;
                    let FPR = (CVU * (FPQ.ln())).exp();
                    let FPS = D + (CVW * FPR);
                    let FPT = D / FPS;
                    let FPU = ((((((FJX * (GY / FPQ)) * CVU) * FPR) * CVW) * FPT) * AC) / FPS;
                    FPV = FPT;
                    FPW = FPU;
                }
                FOP = FPV;
                FOQ = FPW;
            }
            let FOR = (FJW * FOP) + CUY;
            let FOS = CUV * FOR;
            let FOT = (CUX * FOR) + (((FJX * FOP) + (FOQ * FJW)) * CUV);
            let FOU = (CZ * FNH) + CPW;
            let FOV = FOU.ln();
            let FOW = (CY * FOV).exp();
            let FOX = ((D + FOW) + FOM) + (DA * FOS);
            let FOY = (CZ * FNJ) + CPW;
            let FOZ = FOY.ln();
            let FPA = (CY * FOZ).exp();
            let FPB = ((D + FPA) + FOM) + (DB * FOS);
            let FPC = FNN / FOX;
            let FPD = FNQ / FPB;
            let FPE = FPC + FPD;
            let FPF = (FOF * FNS) / FPE;
            let FPG = (((FOG * FNS) + (FNT * FOF)) - ((((FNO - (((((Lanes([(EW * FOV), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * FNH), 0.0, 0.0, 0.0, 0.0]) + (FNI * CZ)) * (GY / FOU)) * CY)) * FOW) + FON) + (Lanes([(EY * FOS), 0.0, 0.0, 0.0, 0.0]) + (FOT * DA))) * FPC)) / FOX) + ((FNR - (((((Lanes([(EW * FOZ), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * FNJ), 0.0, 0.0, 0.0, 0.0]) + (FNK * CZ)) * (GY / FOY)) * CY)) * FPA) + FON) + (Lanes([(EZ * FOS), 0.0, 0.0, 0.0, 0.0]) + (FOT * DB))) * FPD)) / FPB)) * FPF)) / FPE;
            let FPH = XQ + FJW;
            let FPI = D / FPH;
            let FPJ = FJX * FPI;
            let FPK = (FPJ * AC) / FPH;
            let FQD;
            let FQE;
            if FPL != 0.0 {
                let FPY = D + (FPX * FNB);
                let FPZ = D / FPY;
                let FQA = (((FNC * FPX) * FPZ) * AC) / FPY;
                FQD = FPZ;
                FQE = FQA;
            } else {
                let FQB = D - (FPX * FNB);
                let FQC = (FNC * FPX) * AC;
                FQD = FQB;
                FQE = FQC;
            }
            let FQF = FJW * FPI;
            let FQG = FQF * FQD;
            let FQI = FQH * CH;
            let FQK = FQJ * FJW;
            let FQL = FQI + (FQK * FJW);
            let FQM = Lanes([(EF * FQH), 0.0, 0.0, 0.0, 0.0]);
            let FQN = (RA - DGH) / FQL;
            let FQO = D + FQN;
            let FQP = FQO.ln();
            let FQQ = FQP * FQG;
            let FQR = (((((DFR - DGJ) - ((FQM + (((FJX * FQJ) * FJW) + (FJX * FQK))) * FQN)) / FQL) * (GY / FQO)) * FQG) + ((((FPJ + (FPK * FJW)) * FQD) + (FQE * FQF)) * FQP);
            let FQT = FQS * FQQ;
            let FQU = FQR * FQS;
            let FQV = D + FQT;
            let FQW = D + (FQT * FQV);
            let FQX = D / FQW;
            let FQY = ((((FQU * FQV) + (FQU * FQT)) * FQX) * AC) / FQW;
            let FQZ = CZQ + FMP;
            let FRA = (CZQ * FMP) / FQZ;
            let FRB = ((FMQ * CZQ) - (FMQ * FRA)) / FQZ;
            let FRI;
            let FRJ;
            if FRC != 0.0 {
                let FRD = D - (CZU * FRA);
                let FRE = D / FRD;
                let FRF = ((((FRB * CZU) * AC) * FRE) * AC) / FRD;
                FRI = FRE;
                FRJ = FRF;
            } else {
                let FRG = FRB * CZU;
                let FRH = D + (CZU * FRA);
                FRI = FRH;
                FRJ = FRG;
            }
            let FRK = CZQ + FMR;
            let FRL = (CZQ * FMR) / FRK;
            let FRM = ((FMS * CZQ) - (FMS * FRL)) / FRK;
            let FRT;
            let FRU;
            if FRN != 0.0 {
                let FRO = D - (DAU * FRL);
                let FRP = D / FRO;
                let FRQ = ((((FRM * DAU) * AC) * FRP) * AC) / FRO;
                FRT = FRP;
                FRU = FRQ;
            } else {
                let FRR = FRM * DAU;
                let FRS = D + (DAU * FRL);
                FRT = FRS;
                FRU = FRR;
            }
            let FRV = (CM * FJY) * GO;
            let FRW = FRI + FRT;
            let FRX = FRV * FRW;
            let FRY = FPF * FQX;
            let FRZ = (FPG * FQX) + (FQY * FPF);
            let FSA = FRX / FRY;
            let FSB = FSA * FSA;
            let FSC = ((((((Lanes([(EK * FJY), 0.0, 0.0, 0.0, 0.0]) + (FJZ * CM)) * GO) * FRW) + ((FRJ + FRU) * FRV)) - (FRZ * FSA)) / FRY) * FSA;
            let FSD = FSC + FSC;
            let FSE = (D + FSB).sqrt();
            let FSF = FSD * (GY / (GX * FSE));
            let FSG = (D + (SQ * FSB)) / FSE;
            let FSH = ((FSD * SQ) - (FSF * FSG)) / FSE;
            let FTA;
            let FTB;
            let FTC;
            let FTD;
            if L != 0.0 {
                let FSJ = FSI * CP;
                let FSK = EN * FSI;
                let FSL = FMQ * FMP;
                let FSN = (FMP * FMP) + FSM;
                let FSP = (FSO * (FSN.ln())).exp();
                let FSQ = FSJ * FSP;
                let FSR = FMS * FMR;
                let FSS = (FMR * FMR) + FSM;
                let FSU = (FST * (FSS.ln())).exp();
                let FSV = FSJ * FSU;
                let FSW = (D + (AIR * FSQ)) / AFI;
                let FSX = (((AIT * FSQ) + ((Lanes([(FSK * FSP), 0.0, 0.0, 0.0, 0.0]) + (((((FSL + FSL) * (GY / FSN)) * FSO) * FSP) * FSJ)) * AIR)) - (AFO * FSW)) / AFI;
                let FSY = (D + (AIU * FSV)) / AFJ;
                let FSZ = (((AIW * FSV) + ((Lanes([(FSK * FSU), 0.0, 0.0, 0.0, 0.0]) + (((((FSR + FSR) * (GY / FSS)) * FST) * FSU) * FSJ)) * AIU)) - (AFP * FSY)) / AFJ;
                FTA = FSW;
                FTB = FSY;
                FTC = FSX;
                FTD = FSZ;
            } else {
                FTA = D;
                FTB = D;
                FTC = AFD;
                FTD = AFD;
            }
            let FTF;
            let FTG;
            let FTH;
            let FTI;
            if CPX != 0.0 {
                let FTE = if FDE > CPW { 1.0 } else { 0.0 };
                let FTM;
                let FTN;
                if FTE != 0.0 {
                    let FTL = if (FJM.abs()) < NI { 1.0 } else { 0.0 };
                    let FVB;
                    let FVC;
                    if FTL != 0.0 {
                        let FTV = LA + FDC;
                        let FTW = FTV * FJN;
                        let FTX = ((LA + ERO) + (GO * FJN)) / FTW;
                        let FTY = ((ERR + (FJR * GO)) - (((FDF * FJN) + (FJR * FTV)) * FTX)) / FTW;
                        let FTZ = FTX * FJM;
                        let FUA = (FTY * FJM) + (FJQ * FTX);
                        let FUB = FTZ * FTZ;
                        let FUC = FUA * FTZ;
                        let FUD = FUC + FUC;
                        let FUE = ((D - FTZ) + FUB) - (FTZ * FUB);
                        let FUF = LA * FCU;
                        let FUG = D / FJN;
                        let FUH = FTX - FUG;
                        let FUI = FUF * FUH;
                        let FUJ = (FDD - (FUI * FUE)) / FTV;
                        let FUK = ((FJO * FDE) - FCR) / FJN;
                        let FUL = (FUK - FUJ) / FDE;
                        let FUM = FDH * FUL;
                        let FUN = (((((((FJS * FDE) + (FDH * FJO)) - FCS) - (FJR * FUK)) / FJN) - (((FDG - (((((FCV * LA) * FUH) + ((FTY - (((FJR * FUG) * AC) / FJN)) * FUF)) * FUE) + ((((FUA * AC) + FUD) - ((FUA * FUB) + (FUD * FTZ))) * FUI))) - (FDF * FUJ)) / FTV)) - FUM) / FDE;
                        let FUO = FUL + D;
                        let FUP = (FUL * FDE) / FUO;
                        let FUQ = (((FUN * FDE) + FUM) - (FUN * FUP)) / FUO;
                        FVB = FUP;
                        FVC = FUQ;
                    } else {
                        let FUR = FJN * FJM;
                        let FUS = (FJO * FJP) / FUR;
                        let FUT = FCR / FJN;
                        let FUU = FIZ / FJM;
                        let FUV = (FUT + FUU) / FDE;
                        let FUW = FUS - FUV;
                        let FUX = ((((FJS * FJP) + (FJT * FJO)) - (((FJR * FJM) + (FJQ * FJN)) * FUS)) / FUR) - (((((FCS - (FJR * FUT)) / FJN) + ((FJA - (FJQ * FUU)) / FJM)) - (FDH * FUV)) / FDE);
                        let FUY = FUW + D;
                        let FUZ = (FUW * FDE) / FUY;
                        let FVA = (((FUX * FDE) + (FDH * FUW)) - (FUX * FUZ)) / FUY;
                        FVB = FUZ;
                        FVC = FVA;
                    }
                    FTM = FVB;
                    FTN = FVC;
                } else {
                    FTM = CXN;
                    FTN = CXT;
                }
                let FTO = FTM - DAE;
                let FTP = FTN - DAH;
                let FTQ = DEL * FTO;
                let FTR = ((FTP * DEL) * FTO) + (FTP * FTQ);
                let FTS = D + (FTQ * FTO);
                let FTU = if (FTO.abs()) > FTT { 1.0 } else { 0.0 };
                let FWE;
                let FWF;
                if FTU != 0.0 {
                    let FVD = FDE - CJX;
                    let FVE = FDH - CKA;
                    let FVF = FVD - (FTM * FJY);
                    let FVG = FVE - ((FTN * FJY) + (FJZ * FTM));
                    let FVH = FVD - (DAE * FJY);
                    let FVI = FVE - ((DAH * FJY) + (FJZ * DAE));
                    let FVJ = FVG * FVF;
                    let FVK = ((FVF * FVF) + FTS).sqrt();
                    let FVL = ((FVJ + FVJ) + FTR) * (GY / (GX * FVK));
                    let FVM = FVI * FVH;
                    let FVN = ((FVH * FVH) + FTS).sqrt();
                    let FVO = ((FVM + FVM) + FTR) * (GY / (GX * FVN));
                    let FVP = YY / FTO;
                    let FVQ = FVF + FVK;
                    let FVR = (FVH + FVN) / FVQ;
                    let FVS = FVR.ln();
                    let FVT = ((FVN * FVF) - (FVK * FVH)) + (FTS * FVS);
                    let FVU = FVP * FVT;
                    let FVV = ((((FTP * FVP) * AC) / FTO) * FVT) + (((((FVO * FVF) + (FVG * FVN)) - ((FVL * FVH) + (FVI * FVK))) + ((FTR * FVS) + (((((FVI + FVO) - ((FVG + FVL) * FVR)) / FVQ) * (GY / FVR)) * FTS))) * FVP);
                    FWE = FVU;
                    FWF = FVV;
                } else {
                    let FVW = FJY * FTO;
                    let FVX = (FJZ * FTO) + (FTP * FJY);
                    let FVZ = FVY * FJY;
                    let FWA = FVZ * FVW;
                    let FWB = FTS.sqrt();
                    let FWC = (FWA * FVW) / FWB;
                    let FWD = ((((((FJZ * FVY) * FVW) + (FVX * FVZ)) * FVW) + (FVX * FWA)) - ((FTR * (GY / (GX * FWB))) * FWC)) / FWB;
                    FWE = FWC;
                    FWF = FWD;
                }
                FTF = FWE;
                FTG = FTM;
                FTH = FWF;
                FTI = FTN;
            } else {
                FTF = B;
                FTG = CXN;
                FTH = AFD;
                FTI = CXT;
            }
            let FTJ = (((FJW * FJY) + FTF) + CJX) - FDE;
            let FTK = ((((FJX * FJY) + (FJZ * FJW)) + FTH) + CKA) - FDH;
            let FXR;
            let FXS;
            let FXT;
            let FXU;
            if CPX != 0.0 {
                let FWH = if FTJ > FWG { 1.0 } else { 0.0 };
                let FZH;
                let FZI;
                let FZJ;
                let FZK;
                if FWH != 0.0 {
                    let FYR = CJK / CJX;
                    let FYS = FYR - CQJ;
                    let FYT = CQL / FYS;
                    let FYU = FCR / FDE;
                    let FYV = FYU - FJO;
                    let FYW = FJN / FYV;
                    let FYX = (FYT - FYW) / FTJ;
                    let FYY = ((((CQP - ((((CJL - (CKA * FYR)) / CJX) - CQN) * FYT)) / FYS) - ((FJR - ((((FCS - (FDH * FYU)) / FDE) - FJS) * FYW)) / FYV)) - (FTK * FYX)) / FTJ;
                    let FYZ = CPU / CJX;
                    let FZA = FYZ - CQJ;
                    let FZB = CQM / FZA;
                    let FZC = FIZ / FDE;
                    let FZD = FZC - FJO;
                    let FZE = FJM / FZD;
                    let FZF = (FZB - FZE) / FTJ;
                    let FZG = ((((CQQ - ((((CPV - (CKA * FYZ)) / CJX) - CQN) * FZB)) / FZA) - ((FJQ - ((((FJA - (FDH * FZC)) / FDE) - FJS) * FZE)) / FZD)) - (FTK * FZF)) / FTJ;
                    FZH = FYX;
                    FZI = FZF;
                    FZJ = FYY;
                    FZK = FZG;
                } else {
                    FZH = B;
                    FZI = B;
                    FZJ = AFD;
                    FZK = AFD;
                }
                FXR = FZH;
                FXS = FZI;
                FXT = FZJ;
                FXU = FZK;
            } else {
                let FWJ = FWI * CXP;
                let FWK = AIX / CXQ;
                let FWL = FWK + CXZ;
                let FWM = FWJ * FWL;
                let FWN = ((CXV * FWI) * FWL) + ((((AIY - (CXW * FWK)) / CXQ) + CYA) * FWJ);
                let FWP = FWO * CXR;
                let FWQ = AIZ / CXS;
                let FWR = FWQ + CXZ;
                let FWS = FWP * FWR;
                let FWT = ((CXX * FWO) * FWR) + ((((AJA - (CXY * FWQ)) / CXS) + CYA) * FWP);
                let FWU = FWS - FWM;
                let FWV = FWU * CXZ;
                let FWW = ((FWT - FWN) * CXZ) + (CYA * FWU);
                let FWX = FWM * AIX;
                let FWY = (FWN * AIX) + (AIY * FWM);
                let FWZ = FWS * AIZ;
                let FXA = (FWT * AIZ) + (AJA * FWS);
                let FXB = FWX + FWZ;
                let FXC = FWY + FXA;
                let FXD = (((CXV * AIX) + (AIY * CXP)) + ((CXX * AIZ) + (AJA * CXR))) * LA;
                let FXE = ZD + (LA * ((CXP * AIX) + (CXR * AIZ)));
                let FXF = FXB / CXQ;
                let FXG = ((FWZ + FWV) - FXF) / FXE;
                let FXH = FXB / CXS;
                let FXI = ((FWX - FWV) - FXH) / FXE;
                let FXJ = -CXQ;
                let FXK = (FXG * CXQ) + CXZ;
                let FXL = FXJ * FXK;
                let FXM = ((CXW * AC) * FXK) + ((((((((FXA + FWW) - ((FXC - (CXW * FXF)) / CXQ)) - (FXD * FXG)) / FXE) * CXQ) + (CXW * FXG)) + CYA) * FXJ);
                let FXN = -CXS;
                let FXO = (FXI * CXS) + CXZ;
                let FXP = FXN * FXO;
                let FXQ = ((CXY * AC) * FXO) + ((((((((FWY - FWW) - ((FXC - (CXY * FXH)) / CXS)) - (FXD * FXI)) / FXE) * CXS) + (CXY * FXI)) + CYA) * FXN);
                FXR = FXL;
                FXS = FXP;
                FXT = FXM;
                FXU = FXQ;
            }
            let FXV = FXR * FSG;
            let FXW = (FXT * FSG) + (FSH * FXR);
            let FXX = FXS * FSG;
            let FXY = GO * (ERU - BYN);
            let FXZ = (ERV - BYO) * GO;
            let FYA = GO * (FDD - CJY);
            let FYB = (FDG - CKB) * GO;
            let FYC = FXY * FXV;
            let FYD = (FXZ * FXV) + (FXW * FXY);
            let FYE = FYA * FXX;
            let FYF = (FYB * FXX) + (((FXU * FSG) + (FSH * FXS)) * FYA);
            let FYG = (FNS * JL) / FMT;
            let FYH = ((FNT * JL) - (FMU * FYG)) / FMT;
            let FYI = FQS + (DC * FPI);
            let FYJ = FYI * FQQ;
            let FYK = ((Lanes([(FA * FPI), 0.0, 0.0, 0.0, 0.0]) + (FPK * DC)) * FQQ) + (FQR * FYI);
            let FYL = D + FYJ;
            let FYM = D + (FYJ * FYL);
            let FYN = FYM * FQX;
            let FYO = (((FYK * FYL) + (FYK * FYJ)) * FQX) + (FQY * FYM);
            let FYP = FRY * FSE;
            let FYQ = (FRZ * FSE) + (FSF * FRY);
            let FZQ;
            let FZR;
            if L != 0.0 {
                let FZL = FMP / FTA;
                let FZM = FMR / FTB;
                let FZN = FZL + FZM;
                let FZO = FMT / FZN;
                let FZP = (FMU - ((((FMQ - (FTC * FZL)) / FTA) + ((FMS - (FTD * FZM)) / FTB)) * FZO)) / FZN;
                FZQ = FZO;
                FZR = FZP;
            } else {
                FZQ = D;
                FZR = AFD;
            }
            let FZS = DD * DD;
            let FZT = FB * DD;
            let FZU = FZS * FYG;
            let FZV = FZU * AFH;
            let FZW = FZV * FTJ;
            let FZX = (FZW * FYN) / FYP;
            let FZY = FZX / FZQ;
            let FZZ = ((((((((((Lanes([((FZT + FZT) * FYG), 0.0, 0.0, 0.0, 0.0]) + (FYH * FZS)) * AFH) + (AFN * FZU)) * FTJ) + (FTK * FZV)) * FYN) + (FYO * FZW)) - (FYQ * FZX)) / FYP) - (FZR * FZY)) / FZQ;
            let GAA = -QC;
            let GAB = GAA * DE;
            let GAC = (QF * AC) * DE;
            let GAD = Lanes([0.0, GAC[0], GAC[1]]) + Lanes([(FC * GAA), 0.0, 0.0]);
            let GAE = -QJ;
            let GAF = GAE * DE;
            let GAG = (QL * AC) * DE;
            let GAH = Lanes([0.0, GAG[0], GAG[1], GAG[2]]) + Lanes([(FC * GAE), 0.0, 0.0, 0.0]);
            let GAJ = (GAI * DE) + CN;
            let GAK = (FC * GAI) + EL;
            let GAL = GAB + GAJ;
            let GAM = GAD + Lanes([GAK, 0.0, 0.0]);
            let GAN = GAF + GAJ;
            let GAO = GAH + Lanes([GAK, 0.0, 0.0, 0.0]);
            let GAQ = (GAP * DE).sqrt();
            let GAS = GAQ / GAR;
            let GAT = ((FC * GAP) * (GY / (GX * GAQ))) / GAR;
            let GAU = GAS * GAS;
            let GAV = GAT * GAS;
            let GAW = GAV + GAV;
            let GAX = GAT / UD;
            let GAY = D + (GAS / UD);
            let GAZ = FLA * GAY;
            let GBA = D / GAY;
            let GBB = ((GAX * GBA) * AC) / GAY;
            let GBD = UT + (GAS * GBC);
            let GBE = D / GBD;
            let GBF = (((GAT * GBC) * GBE) * AC) / GBD;
            let GBG = if (if DF > B { 1.0 } else { 0.0 }) != 0.0 || (if DG > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GBJ = if (if GBH != 0.0 && GBG != 0.0 { 1.0 } else { 0.0 }) != 0.0 || GBI != 0.0 { 1.0 } else { 0.0 };
            let GBM;
            let GBN;
            if GBJ != 0.0 {
                let GBK = if (GAB.abs()) <= GAZ { 1.0 } else { 0.0 };
                let GBT;
                let GBU;
                if GBK != 0.0 {
                    let GBP = -GAB;
                    let GBQ = GBP * GBA;
                    let GBR = ((GAD * AC) * GBA) + Lanes([(GBB * GBP), 0.0, 0.0]);
                    GBT = GBQ;
                    GBU = GBR;
                } else {
                    let GBS = if GAB < (-GAZ) { 1.0 } else { 0.0 };
                    let GDR;
                    let GDS;
                    if GBS != 0.0 {
                        let GBV = -GAB;
                        let GBW = GAD * AC;
                        let GBX = UT * GBV;
                        let GBY = GBX * GBA;
                        let GBZ = ((GBW * UT) * GBA) + Lanes([(GBB * GBX), 0.0, 0.0]);
                        let GCA = GBY - UX;
                        let GCB = GBZ * GCA;
                        let GCC = ((GCA * GCA) + VA).sqrt();
                        let GCD = GO * ((GBY + UW) - GCC);
                        let GCE = (GBZ - ((GCB + GCB) * (GY / (GX * GCC)))) * GO;
                        let GCF = GBV - GCD;
                        let GCG = GBW - GCE;
                        let GCH = GCG * GCF;
                        let GCI = GCD + D;
                        let GCJ = (GCF * GCF) + (GAU * GCI);
                        let GCK = (GCH + GCH) + (Lanes([(GAW * GCI), 0.0, 0.0]) + (GCE * GAU));
                        let GCL = (LA * GCF) - GAU;
                        let GCM = (GCG * LA) - Lanes([GAW, 0.0, 0.0]);
                        let GCN = GCJ / GAU;
                        let GCO = (GCN.ln()) - GCD;
                        let GCP = (((GCK - Lanes([(GAW * GCN), 0.0, 0.0])) / GAU) * (GY / GCN)) - GCE;
                        let GCQ = GCJ + GCL;
                        let GCR = GCK + GCM;
                        let GCS = GCR * GCQ;
                        let GCT = GO * GCL;
                        let GCU = (GCT * GCL) - GCJ;
                        let GCV = (GCQ * GCQ) + (GCO * GCU);
                        let GCW = (GCS + GCS) + ((GCP * GCU) + (((((GCM * GO) * GCL) + (GCM * GCT)) - GCK) * GCO));
                        let GCX = GCQ / GCV;
                        let GCY = GCX * GCO;
                        let GCZ = GCY * GCO;
                        let GDA = GCZ * GCL;
                        let GDB = GCM * GCL;
                        let GDC = ((GCL * GCL) * WD) - GCJ;
                        let GDD = GCV + (GDA * GDC);
                        let GDE = GCJ * GCQ;
                        let GDF = (GDE * GCO) / GDD;
                        let GDG = GCD + GDF;
                        let GDH = GCE + ((((((GCK * GCQ) + (GCR * GCJ)) * GCO) + (GCP * GDE)) - ((GCW + ((((((((((GCR - (GCW * GCX)) / GCV) * GCO) + (GCP * GCX)) * GCO) + (GCP * GCY)) * GCL) + (GCM * GCZ)) * GDC) + ((((GDB + GDB) * WD) - GCK) * GDA))) * GDF)) / GDD);
                        let GDI = if (GDG.abs()) < SY { 1.0 } else { 0.0 };
                        let GDY;
                        let GDZ;
                        if GDI != 0.0 {
                            let GDV = GDG.exp();
                            let GDW = GDH * GDV;
                            GDY = GDV;
                            GDZ = GDW;
                        } else {
                            let GDX = if GDG < -8e1f64 { 1.0 } else { 0.0 };
                            let GFF;
                            let GFG;
                            if GDX != 0.0 {
                                let GER = GDH * AC;
                                let GES = (-GDG) - SY;
                                let GET = GO * GES;
                                let GEU = D + (GES * WD);
                                let GEV = D + (GET * GEU);
                                let GEW = D + (GES * GEV);
                                let GEX = YT / GEW;
                                let GEY = ((((GER * GEV) + ((((GER * GO) * GEU) + ((GER * WD) * GET)) * GES)) * GEX) * AC) / GEW;
                                GFF = GEX;
                                GFG = GEY;
                            } else {
                                let GEZ = GDG - SY;
                                let GFA = GO * GEZ;
                                let GFB = D + (GEZ * WD);
                                let GFC = D + (GFA * GFB);
                                let GFD = XB * (D + (GEZ * GFC));
                                let GFE = ((GDH * GFC) + ((((GDH * GO) * GFB) + ((GDH * WD) * GFA)) * GEZ)) * XB;
                                GFF = GFD;
                                GFG = GFE;
                            }
                            GDY = GFF;
                            GDZ = GFG;
                        }
                        let GEA = GBV - GDG;
                        let GEB = GBW - GDH;
                        let GEC = GDY - D;
                        let GED = (LA * GEA) + (GAU * GEC);
                        let GEE = (GEB * LA) + (Lanes([(GAW * GEC), 0.0, 0.0]) + (GDZ * GAU));
                        let GEF = GEB * GEA;
                        let GEG = (GDG + D) - GDY;
                        let GEH = (GEA * GEA) + (GAU * GEG);
                        let GEI = (GEF + GEF) + (Lanes([(GAW * GEG), 0.0, 0.0]) + ((GDH - GDZ) * GAU));
                        let GEJ = GAU * GO;
                        let GEK = D - (GEJ * GDY);
                        let GEL = GEE * GED;
                        let GEM = ((GED * GED) - (XQ * (GEK * GEH))).sqrt();
                        let GEN = GED + GEM;
                        let GEO = (LA * GEH) / GEN;
                        let GEP = -(GDG + GEO);
                        let GEQ = (GDH + (((GEI * LA) - ((GEE + (((GEL + GEL) - (((((Lanes([((GAW * GO) * GDY), 0.0, 0.0]) + (GDZ * GEJ)) * AC) * GEH) + (GEI * GEK)) * XQ)) * (GY / (GX * GEM)))) * GEO)) / GEN)) * AC;
                        GDR = GEP;
                        GDS = GEQ;
                    } else {
                        let GDJ = GAY * UT;
                        let GDK = (GDJ * GBE) - D;
                        let GDL = GDK * GBE;
                        let GDM = GAB * GBA;
                        let GDN = D + (GDL * GAB);
                        let GDO = -(GDM * GDN);
                        let GDP = ((((GAD * GBA) + Lanes([(GBB * GAB), 0.0, 0.0])) * GDN) + ((Lanes([((((((GAX * UT) * GBE) + (GBF * GDJ)) * GBE) + (GBF * GDK)) * GAB), 0.0, 0.0]) + (GAD * GDL)) * GDM)) * AC;
                        let GDQ = if (GDO.abs()) < SY { 1.0 } else { 0.0 };
                        let GFK;
                        let GFL;
                        if GDQ != 0.0 {
                            let GFH = GDO.exp();
                            let GFI = GDP * GFH;
                            GFK = GFH;
                            GFL = GFI;
                        } else {
                            let GFJ = if GDO < -8e1f64 { 1.0 } else { 0.0 };
                            let GGI;
                            let GGJ;
                            if GFJ != 0.0 {
                                let GFU = GDP * AC;
                                let GFV = (-GDO) - SY;
                                let GFW = GO * GFV;
                                let GFX = D + (GFV * WD);
                                let GFY = D + (GFW * GFX);
                                let GFZ = D + (GFV * GFY);
                                let GGA = YT / GFZ;
                                let GGB = ((((GFU * GFY) + ((((GFU * GO) * GFX) + ((GFU * WD) * GFW)) * GFV)) * GGA) * AC) / GFZ;
                                GGI = GGA;
                                GGJ = GGB;
                            } else {
                                let GGC = GDO - SY;
                                let GGD = GO * GGC;
                                let GGE = D + (GGC * WD);
                                let GGF = D + (GGD * GGE);
                                let GGG = XB * (D + (GGC * GGF));
                                let GGH = ((GDP * GGF) + ((((GDP * GO) * GGE) + ((GDP * WD) * GGD)) * GGC)) * XB;
                                GGI = GGG;
                                GGJ = GGH;
                            }
                            GFK = GGI;
                            GFL = GGJ;
                        }
                        let GFM = GAU * GO;
                        let GFN = GAW * GO;
                        let GFO = ((GAB + (GAU * YY)) - (D - GFK)).sqrt();
                        let GFP = (GAB + GFM) - (GAS * GFO);
                        let GFQ = (GAD + Lanes([GFN, 0.0, 0.0])) - (Lanes([(GAT * GFO), 0.0, 0.0]) + ((((GAD + Lanes([(GAW * YY), 0.0, 0.0])) - (GFL * AC)) * (GY / (GX * GFO))) * GAS));
                        let GFR = -GFP;
                        let GFS = GFQ * AC;
                        let GFT = if (GFR.abs()) < SY { 1.0 } else { 0.0 };
                        let GGN;
                        let GGO;
                        if GFT != 0.0 {
                            let GGK = GFR.exp();
                            let GGL = GFS * GGK;
                            GGN = GGK;
                            GGO = GGL;
                        } else {
                            let GGM = if GFR < -8e1f64 { 1.0 } else { 0.0 };
                            let GHT;
                            let GHU;
                            if GGM != 0.0 {
                                let GHF = GFS * AC;
                                let GHG = (-GFR) - SY;
                                let GHH = GO * GHG;
                                let GHI = D + (GHG * WD);
                                let GHJ = D + (GHH * GHI);
                                let GHK = D + (GHG * GHJ);
                                let GHL = YT / GHK;
                                let GHM = ((((GHF * GHJ) + ((((GHF * GO) * GHI) + ((GHF * WD) * GHH)) * GHG)) * GHL) * AC) / GHK;
                                GHT = GHL;
                                GHU = GHM;
                            } else {
                                let GHN = GFR - SY;
                                let GHO = GO * GHN;
                                let GHP = D + (GHN * WD);
                                let GHQ = D + (GHO * GHP);
                                let GHR = XB * (D + (GHN * GHQ));
                                let GHS = ((GFS * GHQ) + ((((GFS * GO) * GHP) + ((GFS * WD) * GHO)) * GHN)) * XB;
                                GHT = GHR;
                                GHU = GHS;
                            }
                            GGN = GHT;
                            GGO = GHU;
                        }
                        let GGP = GAB - GFP;
                        let GGQ = GAD - GFQ;
                        let GGR = D - GGN;
                        let GGS = (LA * GGP) + (GAU * GGR);
                        let GGT = (GGQ * LA) + (Lanes([(GAW * GGR), 0.0, 0.0]) + ((GGO * AC) * GAU));
                        let GGU = GGQ * GGP;
                        let GGV = (GFP - D) + GGN;
                        let GGW = (GGP * GGP) - (GAU * GGV);
                        let GGX = (GGU + GGU) - (Lanes([(GAW * GGV), 0.0, 0.0]) + ((GFQ + GGO) * GAU));
                        let GGY = D - (GFM * GGN);
                        let GGZ = GGT * GGS;
                        let GHA = ((GGS * GGS) - (XQ * (GGY * GGW))).sqrt();
                        let GHB = GGS + GHA;
                        let GHC = (LA * GGW) / GHB;
                        let GHD = GFP + GHC;
                        let GHE = GFQ + (((GGX * LA) - ((GGT + (((GGZ + GGZ) - (((((Lanes([(GFN * GGN), 0.0, 0.0]) + (GGO * GFM)) * AC) * GGW) + (GGX * GGY)) * XQ)) * (GY / (GX * GHA)))) * GHC)) / GHB);
                        GDR = GHD;
                        GDS = GHE;
                    }
                    let GDT = -GDR;
                    let GDU = GDS * AC;
                    GBT = GDT;
                    GBU = GDU;
                }
                GBM = GBT;
                GBN = GBU;
            } else {
                GBM = B;
                GBN = GBL;
            }
            let GHW;
            let GHX;
            if GBO != 0.0 {
                let GHV = if (GAL.abs()) <= GAZ { 1.0 } else { 0.0 };
                let GIU;
                let GIV;
                if GHV != 0.0 {
                    let GIQ = -GAL;
                    let GIR = GIQ * GBA;
                    let GIS = ((GAM * AC) * GBA) + Lanes([(GBB * GIQ), 0.0, 0.0]);
                    GIU = GIR;
                    GIV = GIS;
                } else {
                    let GIT = if GAL < (-GAZ) { 1.0 } else { 0.0 };
                    let GKS;
                    let GKT;
                    if GIT != 0.0 {
                        let GIW = -GAL;
                        let GIX = GAM * AC;
                        let GIY = UT * GIW;
                        let GIZ = GIY * GBA;
                        let GJA = ((GIX * UT) * GBA) + Lanes([(GBB * GIY), 0.0, 0.0]);
                        let GJB = GIZ - UX;
                        let GJC = GJA * GJB;
                        let GJD = ((GJB * GJB) + VA).sqrt();
                        let GJE = GO * ((GIZ + UW) - GJD);
                        let GJF = (GJA - ((GJC + GJC) * (GY / (GX * GJD)))) * GO;
                        let GJG = GIW - GJE;
                        let GJH = GIX - GJF;
                        let GJI = GJH * GJG;
                        let GJJ = GJE + D;
                        let GJK = (GJG * GJG) + (GAU * GJJ);
                        let GJL = (GJI + GJI) + (Lanes([(GAW * GJJ), 0.0, 0.0]) + (GJF * GAU));
                        let GJM = (LA * GJG) - GAU;
                        let GJN = (GJH * LA) - Lanes([GAW, 0.0, 0.0]);
                        let GJO = GJK / GAU;
                        let GJP = (GJO.ln()) - GJE;
                        let GJQ = (((GJL - Lanes([(GAW * GJO), 0.0, 0.0])) / GAU) * (GY / GJO)) - GJF;
                        let GJR = GJK + GJM;
                        let GJS = GJL + GJN;
                        let GJT = GJS * GJR;
                        let GJU = GO * GJM;
                        let GJV = (GJU * GJM) - GJK;
                        let GJW = (GJR * GJR) + (GJP * GJV);
                        let GJX = (GJT + GJT) + ((GJQ * GJV) + (((((GJN * GO) * GJM) + (GJN * GJU)) - GJL) * GJP));
                        let GJY = GJR / GJW;
                        let GJZ = GJY * GJP;
                        let GKA = GJZ * GJP;
                        let GKB = GKA * GJM;
                        let GKC = GJN * GJM;
                        let GKD = ((GJM * GJM) * WD) - GJK;
                        let GKE = GJW + (GKB * GKD);
                        let GKF = GJK * GJR;
                        let GKG = (GKF * GJP) / GKE;
                        let GKH = GJE + GKG;
                        let GKI = GJF + ((((((GJL * GJR) + (GJS * GJK)) * GJP) + (GJQ * GKF)) - ((GJX + ((((((((((GJS - (GJX * GJY)) / GJW) * GJP) + (GJQ * GJY)) * GJP) + (GJQ * GJZ)) * GJM) + (GJN * GKA)) * GKD) + ((((GKC + GKC) * WD) - GJL) * GKB))) * GKG)) / GKE);
                        let GKJ = if (GKH.abs()) < SY { 1.0 } else { 0.0 };
                        let GKZ;
                        let GLA;
                        if GKJ != 0.0 {
                            let GKW = GKH.exp();
                            let GKX = GKI * GKW;
                            GKZ = GKW;
                            GLA = GKX;
                        } else {
                            let GKY = if GKH < -8e1f64 { 1.0 } else { 0.0 };
                            let GMG;
                            let GMH;
                            if GKY != 0.0 {
                                let GLS = GKI * AC;
                                let GLT = (-GKH) - SY;
                                let GLU = GO * GLT;
                                let GLV = D + (GLT * WD);
                                let GLW = D + (GLU * GLV);
                                let GLX = D + (GLT * GLW);
                                let GLY = YT / GLX;
                                let GLZ = ((((GLS * GLW) + ((((GLS * GO) * GLV) + ((GLS * WD) * GLU)) * GLT)) * GLY) * AC) / GLX;
                                GMG = GLY;
                                GMH = GLZ;
                            } else {
                                let GMA = GKH - SY;
                                let GMB = GO * GMA;
                                let GMC = D + (GMA * WD);
                                let GMD = D + (GMB * GMC);
                                let GME = XB * (D + (GMA * GMD));
                                let GMF = ((GKI * GMD) + ((((GKI * GO) * GMC) + ((GKI * WD) * GMB)) * GMA)) * XB;
                                GMG = GME;
                                GMH = GMF;
                            }
                            GKZ = GMG;
                            GLA = GMH;
                        }
                        let GLB = GIW - GKH;
                        let GLC = GIX - GKI;
                        let GLD = GKZ - D;
                        let GLE = (LA * GLB) + (GAU * GLD);
                        let GLF = (GLC * LA) + (Lanes([(GAW * GLD), 0.0, 0.0]) + (GLA * GAU));
                        let GLG = GLC * GLB;
                        let GLH = (GKH + D) - GKZ;
                        let GLI = (GLB * GLB) + (GAU * GLH);
                        let GLJ = (GLG + GLG) + (Lanes([(GAW * GLH), 0.0, 0.0]) + ((GKI - GLA) * GAU));
                        let GLK = GAU * GO;
                        let GLL = D - (GLK * GKZ);
                        let GLM = GLF * GLE;
                        let GLN = ((GLE * GLE) - (XQ * (GLL * GLI))).sqrt();
                        let GLO = GLE + GLN;
                        let GLP = (LA * GLI) / GLO;
                        let GLQ = -(GKH + GLP);
                        let GLR = (GKI + (((GLJ * LA) - ((GLF + (((GLM + GLM) - (((((Lanes([((GAW * GO) * GKZ), 0.0, 0.0]) + (GLA * GLK)) * AC) * GLI) + (GLJ * GLL)) * XQ)) * (GY / (GX * GLN)))) * GLP)) / GLO)) * AC;
                        GKS = GLQ;
                        GKT = GLR;
                    } else {
                        let GKK = GAY * UT;
                        let GKL = (GKK * GBE) - D;
                        let GKM = GKL * GBE;
                        let GKN = GAL * GBA;
                        let GKO = D + (GKM * GAL);
                        let GKP = -(GKN * GKO);
                        let GKQ = ((((GAM * GBA) + Lanes([(GBB * GAL), 0.0, 0.0])) * GKO) + ((Lanes([((((((GAX * UT) * GBE) + (GBF * GKK)) * GBE) + (GBF * GKL)) * GAL), 0.0, 0.0]) + (GAM * GKM)) * GKN)) * AC;
                        let GKR = if (GKP.abs()) < SY { 1.0 } else { 0.0 };
                        let GML;
                        let GMM;
                        if GKR != 0.0 {
                            let GMI = GKP.exp();
                            let GMJ = GKQ * GMI;
                            GML = GMI;
                            GMM = GMJ;
                        } else {
                            let GMK = if GKP < -8e1f64 { 1.0 } else { 0.0 };
                            let GNJ;
                            let GNK;
                            if GMK != 0.0 {
                                let GMV = GKQ * AC;
                                let GMW = (-GKP) - SY;
                                let GMX = GO * GMW;
                                let GMY = D + (GMW * WD);
                                let GMZ = D + (GMX * GMY);
                                let GNA = D + (GMW * GMZ);
                                let GNB = YT / GNA;
                                let GNC = ((((GMV * GMZ) + ((((GMV * GO) * GMY) + ((GMV * WD) * GMX)) * GMW)) * GNB) * AC) / GNA;
                                GNJ = GNB;
                                GNK = GNC;
                            } else {
                                let GND = GKP - SY;
                                let GNE = GO * GND;
                                let GNF = D + (GND * WD);
                                let GNG = D + (GNE * GNF);
                                let GNH = XB * (D + (GND * GNG));
                                let GNI = ((GKQ * GNG) + ((((GKQ * GO) * GNF) + ((GKQ * WD) * GNE)) * GND)) * XB;
                                GNJ = GNH;
                                GNK = GNI;
                            }
                            GML = GNJ;
                            GMM = GNK;
                        }
                        let GMN = GAU * GO;
                        let GMO = GAW * GO;
                        let GMP = ((GAL + (GAU * YY)) - (D - GML)).sqrt();
                        let GMQ = (GAL + GMN) - (GAS * GMP);
                        let GMR = (GAM + Lanes([GMO, 0.0, 0.0])) - (Lanes([(GAT * GMP), 0.0, 0.0]) + ((((GAM + Lanes([(GAW * YY), 0.0, 0.0])) - (GMM * AC)) * (GY / (GX * GMP))) * GAS));
                        let GMS = -GMQ;
                        let GMT = GMR * AC;
                        let GMU = if (GMS.abs()) < SY { 1.0 } else { 0.0 };
                        let GNO;
                        let GNP;
                        if GMU != 0.0 {
                            let GNL = GMS.exp();
                            let GNM = GMT * GNL;
                            GNO = GNL;
                            GNP = GNM;
                        } else {
                            let GNN = if GMS < -8e1f64 { 1.0 } else { 0.0 };
                            let GOU;
                            let GOV;
                            if GNN != 0.0 {
                                let GOG = GMT * AC;
                                let GOH = (-GMS) - SY;
                                let GOI = GO * GOH;
                                let GOJ = D + (GOH * WD);
                                let GOK = D + (GOI * GOJ);
                                let GOL = D + (GOH * GOK);
                                let GOM = YT / GOL;
                                let GON = ((((GOG * GOK) + ((((GOG * GO) * GOJ) + ((GOG * WD) * GOI)) * GOH)) * GOM) * AC) / GOL;
                                GOU = GOM;
                                GOV = GON;
                            } else {
                                let GOO = GMS - SY;
                                let GOP = GO * GOO;
                                let GOQ = D + (GOO * WD);
                                let GOR = D + (GOP * GOQ);
                                let GOS = XB * (D + (GOO * GOR));
                                let GOT = ((GMT * GOR) + ((((GMT * GO) * GOQ) + ((GMT * WD) * GOP)) * GOO)) * XB;
                                GOU = GOS;
                                GOV = GOT;
                            }
                            GNO = GOU;
                            GNP = GOV;
                        }
                        let GNQ = GAL - GMQ;
                        let GNR = GAM - GMR;
                        let GNS = D - GNO;
                        let GNT = (LA * GNQ) + (GAU * GNS);
                        let GNU = (GNR * LA) + (Lanes([(GAW * GNS), 0.0, 0.0]) + ((GNP * AC) * GAU));
                        let GNV = GNR * GNQ;
                        let GNW = (GMQ - D) + GNO;
                        let GNX = (GNQ * GNQ) - (GAU * GNW);
                        let GNY = (GNV + GNV) - (Lanes([(GAW * GNW), 0.0, 0.0]) + ((GMR + GNP) * GAU));
                        let GNZ = D - (GMN * GNO);
                        let GOA = GNU * GNT;
                        let GOB = ((GNT * GNT) - (XQ * (GNZ * GNX))).sqrt();
                        let GOC = GNT + GOB;
                        let GOD = (LA * GNX) / GOC;
                        let GOE = GMQ + GOD;
                        let GOF = GMR + (((GNY * LA) - ((GNU + (((GOA + GOA) - (((((Lanes([(GMO * GNO), 0.0, 0.0]) + (GNP * GMN)) * AC) * GNX) + (GNY * GNZ)) * XQ)) * (GY / (GX * GOB)))) * GOD)) / GOC);
                        GKS = GOE;
                        GKT = GOF;
                    }
                    let GKU = -GKS;
                    let GKV = GKT * AC;
                    GIU = GKU;
                    GIV = GKV;
                }
                GHW = GIU;
                GHX = GIV;
            } else {
                GHW = B;
                GHX = GBL;
            }
            let GHZ = (GHY * DE).sqrt();
            let GIA = GHZ / GAR;
            let GIB = ((FC * GHY) * (GY / (GX * GHZ))) / GAR;
            let GIC = GIA * GIA;
            let GID = GIB * GIA;
            let GIE = GID + GID;
            let GIF = GIB / UD;
            let GIG = D + (GIA / UD);
            let GIH = FLA * GIG;
            let GII = D / GIG;
            let GIJ = ((GIF * GII) * AC) / GIG;
            let GIK = UT + (GIA * GBC);
            let GIL = D / GIK;
            let GIM = (((GIB * GBC) * GIL) * AC) / GIK;
            let GIN = if (if DH > B { 1.0 } else { 0.0 }) != 0.0 || (if DI > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GIP = if (if GBH != 0.0 && GIN != 0.0 { 1.0 } else { 0.0 }) != 0.0 || GIO != 0.0 { 1.0 } else { 0.0 };
            let GOY;
            let GOZ;
            if GIP != 0.0 {
                let GOW = if (GAF.abs()) <= GIH { 1.0 } else { 0.0 };
                let GPF;
                let GPG;
                if GOW != 0.0 {
                    let GPB = -GAF;
                    let GPC = GPB * GII;
                    let GPD = ((GAH * AC) * GII) + Lanes([(GIJ * GPB), 0.0, 0.0, 0.0]);
                    GPF = GPC;
                    GPG = GPD;
                } else {
                    let GPE = if GAF < (-GIH) { 1.0 } else { 0.0 };
                    let GRD;
                    let GRE;
                    if GPE != 0.0 {
                        let GPH = -GAF;
                        let GPI = GAH * AC;
                        let GPJ = UT * GPH;
                        let GPK = GPJ * GII;
                        let GPL = ((GPI * UT) * GII) + Lanes([(GIJ * GPJ), 0.0, 0.0, 0.0]);
                        let GPM = GPK - UX;
                        let GPN = GPL * GPM;
                        let GPO = ((GPM * GPM) + VA).sqrt();
                        let GPP = GO * ((GPK + UW) - GPO);
                        let GPQ = (GPL - ((GPN + GPN) * (GY / (GX * GPO)))) * GO;
                        let GPR = GPH - GPP;
                        let GPS = GPI - GPQ;
                        let GPT = GPS * GPR;
                        let GPU = GPP + D;
                        let GPV = (GPR * GPR) + (GIC * GPU);
                        let GPW = (GPT + GPT) + (Lanes([(GIE * GPU), 0.0, 0.0, 0.0]) + (GPQ * GIC));
                        let GPX = (LA * GPR) - GIC;
                        let GPY = (GPS * LA) - Lanes([GIE, 0.0, 0.0, 0.0]);
                        let GPZ = GPV / GIC;
                        let GQA = (GPZ.ln()) - GPP;
                        let GQB = (((GPW - Lanes([(GIE * GPZ), 0.0, 0.0, 0.0])) / GIC) * (GY / GPZ)) - GPQ;
                        let GQC = GPV + GPX;
                        let GQD = GPW + GPY;
                        let GQE = GQD * GQC;
                        let GQF = GO * GPX;
                        let GQG = (GQF * GPX) - GPV;
                        let GQH = (GQC * GQC) + (GQA * GQG);
                        let GQI = (GQE + GQE) + ((GQB * GQG) + (((((GPY * GO) * GPX) + (GPY * GQF)) - GPW) * GQA));
                        let GQJ = GQC / GQH;
                        let GQK = GQJ * GQA;
                        let GQL = GQK * GQA;
                        let GQM = GQL * GPX;
                        let GQN = GPY * GPX;
                        let GQO = ((GPX * GPX) * WD) - GPV;
                        let GQP = GQH + (GQM * GQO);
                        let GQQ = GPV * GQC;
                        let GQR = (GQQ * GQA) / GQP;
                        let GQS = GPP + GQR;
                        let GQT = GPQ + ((((((GPW * GQC) + (GQD * GPV)) * GQA) + (GQB * GQQ)) - ((GQI + ((((((((((GQD - (GQI * GQJ)) / GQH) * GQA) + (GQB * GQJ)) * GQA) + (GQB * GQK)) * GPX) + (GPY * GQL)) * GQO) + ((((GQN + GQN) * WD) - GPW) * GQM))) * GQR)) / GQP);
                        let GQU = if (GQS.abs()) < SY { 1.0 } else { 0.0 };
                        let GRK;
                        let GRL;
                        if GQU != 0.0 {
                            let GRH = GQS.exp();
                            let GRI = GQT * GRH;
                            GRK = GRH;
                            GRL = GRI;
                        } else {
                            let GRJ = if GQS < -8e1f64 { 1.0 } else { 0.0 };
                            let GSR;
                            let GSS;
                            if GRJ != 0.0 {
                                let GSD = GQT * AC;
                                let GSE = (-GQS) - SY;
                                let GSF = GO * GSE;
                                let GSG = D + (GSE * WD);
                                let GSH = D + (GSF * GSG);
                                let GSI = D + (GSE * GSH);
                                let GSJ = YT / GSI;
                                let GSK = ((((GSD * GSH) + ((((GSD * GO) * GSG) + ((GSD * WD) * GSF)) * GSE)) * GSJ) * AC) / GSI;
                                GSR = GSJ;
                                GSS = GSK;
                            } else {
                                let GSL = GQS - SY;
                                let GSM = GO * GSL;
                                let GSN = D + (GSL * WD);
                                let GSO = D + (GSM * GSN);
                                let GSP = XB * (D + (GSL * GSO));
                                let GSQ = ((GQT * GSO) + ((((GQT * GO) * GSN) + ((GQT * WD) * GSM)) * GSL)) * XB;
                                GSR = GSP;
                                GSS = GSQ;
                            }
                            GRK = GSR;
                            GRL = GSS;
                        }
                        let GRM = GPH - GQS;
                        let GRN = GPI - GQT;
                        let GRO = GRK - D;
                        let GRP = (LA * GRM) + (GIC * GRO);
                        let GRQ = (GRN * LA) + (Lanes([(GIE * GRO), 0.0, 0.0, 0.0]) + (GRL * GIC));
                        let GRR = GRN * GRM;
                        let GRS = (GQS + D) - GRK;
                        let GRT = (GRM * GRM) + (GIC * GRS);
                        let GRU = (GRR + GRR) + (Lanes([(GIE * GRS), 0.0, 0.0, 0.0]) + ((GQT - GRL) * GIC));
                        let GRV = GIC * GO;
                        let GRW = D - (GRV * GRK);
                        let GRX = GRQ * GRP;
                        let GRY = ((GRP * GRP) - (XQ * (GRW * GRT))).sqrt();
                        let GRZ = GRP + GRY;
                        let GSA = (LA * GRT) / GRZ;
                        let GSB = -(GQS + GSA);
                        let GSC = (GQT + (((GRU * LA) - ((GRQ + (((GRX + GRX) - (((((Lanes([((GIE * GO) * GRK), 0.0, 0.0, 0.0]) + (GRL * GRV)) * AC) * GRT) + (GRU * GRW)) * XQ)) * (GY / (GX * GRY)))) * GSA)) / GRZ)) * AC;
                        GRD = GSB;
                        GRE = GSC;
                    } else {
                        let GQV = GIG * UT;
                        let GQW = (GQV * GIL) - D;
                        let GQX = GQW * GIL;
                        let GQY = GAF * GII;
                        let GQZ = D + (GQX * GAF);
                        let GRA = -(GQY * GQZ);
                        let GRB = ((((GAH * GII) + Lanes([(GIJ * GAF), 0.0, 0.0, 0.0])) * GQZ) + ((Lanes([((((((GIF * UT) * GIL) + (GIM * GQV)) * GIL) + (GIM * GQW)) * GAF), 0.0, 0.0, 0.0]) + (GAH * GQX)) * GQY)) * AC;
                        let GRC = if (GRA.abs()) < SY { 1.0 } else { 0.0 };
                        let GSW;
                        let GSX;
                        if GRC != 0.0 {
                            let GST = GRA.exp();
                            let GSU = GRB * GST;
                            GSW = GST;
                            GSX = GSU;
                        } else {
                            let GSV = if GRA < -8e1f64 { 1.0 } else { 0.0 };
                            let GTU;
                            let GTV;
                            if GSV != 0.0 {
                                let GTG = GRB * AC;
                                let GTH = (-GRA) - SY;
                                let GTI = GO * GTH;
                                let GTJ = D + (GTH * WD);
                                let GTK = D + (GTI * GTJ);
                                let GTL = D + (GTH * GTK);
                                let GTM = YT / GTL;
                                let GTN = ((((GTG * GTK) + ((((GTG * GO) * GTJ) + ((GTG * WD) * GTI)) * GTH)) * GTM) * AC) / GTL;
                                GTU = GTM;
                                GTV = GTN;
                            } else {
                                let GTO = GRA - SY;
                                let GTP = GO * GTO;
                                let GTQ = D + (GTO * WD);
                                let GTR = D + (GTP * GTQ);
                                let GTS = XB * (D + (GTO * GTR));
                                let GTT = ((GRB * GTR) + ((((GRB * GO) * GTQ) + ((GRB * WD) * GTP)) * GTO)) * XB;
                                GTU = GTS;
                                GTV = GTT;
                            }
                            GSW = GTU;
                            GSX = GTV;
                        }
                        let GSY = GIC * GO;
                        let GSZ = GIE * GO;
                        let GTA = ((GAF + (GIC * YY)) - (D - GSW)).sqrt();
                        let GTB = (GAF + GSY) - (GIA * GTA);
                        let GTC = (GAH + Lanes([GSZ, 0.0, 0.0, 0.0])) - (Lanes([(GIB * GTA), 0.0, 0.0, 0.0]) + ((((GAH + Lanes([(GIE * YY), 0.0, 0.0, 0.0])) - (GSX * AC)) * (GY / (GX * GTA))) * GIA));
                        let GTD = -GTB;
                        let GTE = GTC * AC;
                        let GTF = if (GTD.abs()) < SY { 1.0 } else { 0.0 };
                        let GTZ;
                        let GUA;
                        if GTF != 0.0 {
                            let GTW = GTD.exp();
                            let GTX = GTE * GTW;
                            GTZ = GTW;
                            GUA = GTX;
                        } else {
                            let GTY = if GTD < -8e1f64 { 1.0 } else { 0.0 };
                            let GVF;
                            let GVG;
                            if GTY != 0.0 {
                                let GUR = GTE * AC;
                                let GUS = (-GTD) - SY;
                                let GUT = GO * GUS;
                                let GUU = D + (GUS * WD);
                                let GUV = D + (GUT * GUU);
                                let GUW = D + (GUS * GUV);
                                let GUX = YT / GUW;
                                let GUY = ((((GUR * GUV) + ((((GUR * GO) * GUU) + ((GUR * WD) * GUT)) * GUS)) * GUX) * AC) / GUW;
                                GVF = GUX;
                                GVG = GUY;
                            } else {
                                let GUZ = GTD - SY;
                                let GVA = GO * GUZ;
                                let GVB = D + (GUZ * WD);
                                let GVC = D + (GVA * GVB);
                                let GVD = XB * (D + (GUZ * GVC));
                                let GVE = ((GTE * GVC) + ((((GTE * GO) * GVB) + ((GTE * WD) * GVA)) * GUZ)) * XB;
                                GVF = GVD;
                                GVG = GVE;
                            }
                            GTZ = GVF;
                            GUA = GVG;
                        }
                        let GUB = GAF - GTB;
                        let GUC = GAH - GTC;
                        let GUD = D - GTZ;
                        let GUE = (LA * GUB) + (GIC * GUD);
                        let GUF = (GUC * LA) + (Lanes([(GIE * GUD), 0.0, 0.0, 0.0]) + ((GUA * AC) * GIC));
                        let GUG = GUC * GUB;
                        let GUH = (GTB - D) + GTZ;
                        let GUI = (GUB * GUB) - (GIC * GUH);
                        let GUJ = (GUG + GUG) - (Lanes([(GIE * GUH), 0.0, 0.0, 0.0]) + ((GTC + GUA) * GIC));
                        let GUK = D - (GSY * GTZ);
                        let GUL = GUF * GUE;
                        let GUM = ((GUE * GUE) - (XQ * (GUK * GUI))).sqrt();
                        let GUN = GUE + GUM;
                        let GUO = (LA * GUI) / GUN;
                        let GUP = GTB + GUO;
                        let GUQ = GTC + (((GUJ * LA) - ((GUF + (((GUL + GUL) - (((((Lanes([(GSZ * GTZ), 0.0, 0.0, 0.0]) + (GUA * GSY)) * AC) * GUI) + (GUJ * GUK)) * XQ)) * (GY / (GX * GUM)))) * GUO)) / GUN);
                        GRD = GUP;
                        GRE = GUQ;
                    }
                    let GRF = -GRD;
                    let GRG = GRE * AC;
                    GPF = GRF;
                    GPG = GRG;
                }
                GOY = GPF;
                GOZ = GPG;
            } else {
                GOY = B;
                GOZ = GOX;
            }
            let GVI;
            let GVJ;
            if GPA != 0.0 {
                let GVH = if (GAN.abs()) <= GIH { 1.0 } else { 0.0 };
                let GWC;
                let GWD;
                if GVH != 0.0 {
                    let GVY = -GAN;
                    let GVZ = GVY * GII;
                    let GWA = ((GAO * AC) * GII) + Lanes([(GIJ * GVY), 0.0, 0.0, 0.0]);
                    GWC = GVZ;
                    GWD = GWA;
                } else {
                    let GWB = if GAN < (-GIH) { 1.0 } else { 0.0 };
                    let GYA;
                    let GYB;
                    if GWB != 0.0 {
                        let GWE = -GAN;
                        let GWF = GAO * AC;
                        let GWG = UT * GWE;
                        let GWH = GWG * GII;
                        let GWI = ((GWF * UT) * GII) + Lanes([(GIJ * GWG), 0.0, 0.0, 0.0]);
                        let GWJ = GWH - UX;
                        let GWK = GWI * GWJ;
                        let GWL = ((GWJ * GWJ) + VA).sqrt();
                        let GWM = GO * ((GWH + UW) - GWL);
                        let GWN = (GWI - ((GWK + GWK) * (GY / (GX * GWL)))) * GO;
                        let GWO = GWE - GWM;
                        let GWP = GWF - GWN;
                        let GWQ = GWP * GWO;
                        let GWR = GWM + D;
                        let GWS = (GWO * GWO) + (GIC * GWR);
                        let GWT = (GWQ + GWQ) + (Lanes([(GIE * GWR), 0.0, 0.0, 0.0]) + (GWN * GIC));
                        let GWU = (LA * GWO) - GIC;
                        let GWV = (GWP * LA) - Lanes([GIE, 0.0, 0.0, 0.0]);
                        let GWW = GWS / GIC;
                        let GWX = (GWW.ln()) - GWM;
                        let GWY = (((GWT - Lanes([(GIE * GWW), 0.0, 0.0, 0.0])) / GIC) * (GY / GWW)) - GWN;
                        let GWZ = GWS + GWU;
                        let GXA = GWT + GWV;
                        let GXB = GXA * GWZ;
                        let GXC = GO * GWU;
                        let GXD = (GXC * GWU) - GWS;
                        let GXE = (GWZ * GWZ) + (GWX * GXD);
                        let GXF = (GXB + GXB) + ((GWY * GXD) + (((((GWV * GO) * GWU) + (GWV * GXC)) - GWT) * GWX));
                        let GXG = GWZ / GXE;
                        let GXH = GXG * GWX;
                        let GXI = GXH * GWX;
                        let GXJ = GXI * GWU;
                        let GXK = GWV * GWU;
                        let GXL = ((GWU * GWU) * WD) - GWS;
                        let GXM = GXE + (GXJ * GXL);
                        let GXN = GWS * GWZ;
                        let GXO = (GXN * GWX) / GXM;
                        let GXP = GWM + GXO;
                        let GXQ = GWN + ((((((GWT * GWZ) + (GXA * GWS)) * GWX) + (GWY * GXN)) - ((GXF + ((((((((((GXA - (GXF * GXG)) / GXE) * GWX) + (GWY * GXG)) * GWX) + (GWY * GXH)) * GWU) + (GWV * GXI)) * GXL) + ((((GXK + GXK) * WD) - GWT) * GXJ))) * GXO)) / GXM);
                        let GXR = if (GXP.abs()) < SY { 1.0 } else { 0.0 };
                        let GYH;
                        let GYI;
                        if GXR != 0.0 {
                            let GYE = GXP.exp();
                            let GYF = GXQ * GYE;
                            GYH = GYE;
                            GYI = GYF;
                        } else {
                            let GYG = if GXP < -8e1f64 { 1.0 } else { 0.0 };
                            let GZO;
                            let GZP;
                            if GYG != 0.0 {
                                let GZA = GXQ * AC;
                                let GZB = (-GXP) - SY;
                                let GZC = GO * GZB;
                                let GZD = D + (GZB * WD);
                                let GZE = D + (GZC * GZD);
                                let GZF = D + (GZB * GZE);
                                let GZG = YT / GZF;
                                let GZH = ((((GZA * GZE) + ((((GZA * GO) * GZD) + ((GZA * WD) * GZC)) * GZB)) * GZG) * AC) / GZF;
                                GZO = GZG;
                                GZP = GZH;
                            } else {
                                let GZI = GXP - SY;
                                let GZJ = GO * GZI;
                                let GZK = D + (GZI * WD);
                                let GZL = D + (GZJ * GZK);
                                let GZM = XB * (D + (GZI * GZL));
                                let GZN = ((GXQ * GZL) + ((((GXQ * GO) * GZK) + ((GXQ * WD) * GZJ)) * GZI)) * XB;
                                GZO = GZM;
                                GZP = GZN;
                            }
                            GYH = GZO;
                            GYI = GZP;
                        }
                        let GYJ = GWE - GXP;
                        let GYK = GWF - GXQ;
                        let GYL = GYH - D;
                        let GYM = (LA * GYJ) + (GIC * GYL);
                        let GYN = (GYK * LA) + (Lanes([(GIE * GYL), 0.0, 0.0, 0.0]) + (GYI * GIC));
                        let GYO = GYK * GYJ;
                        let GYP = (GXP + D) - GYH;
                        let GYQ = (GYJ * GYJ) + (GIC * GYP);
                        let GYR = (GYO + GYO) + (Lanes([(GIE * GYP), 0.0, 0.0, 0.0]) + ((GXQ - GYI) * GIC));
                        let GYS = GIC * GO;
                        let GYT = D - (GYS * GYH);
                        let GYU = GYN * GYM;
                        let GYV = ((GYM * GYM) - (XQ * (GYT * GYQ))).sqrt();
                        let GYW = GYM + GYV;
                        let GYX = (LA * GYQ) / GYW;
                        let GYY = -(GXP + GYX);
                        let GYZ = (GXQ + (((GYR * LA) - ((GYN + (((GYU + GYU) - (((((Lanes([((GIE * GO) * GYH), 0.0, 0.0, 0.0]) + (GYI * GYS)) * AC) * GYQ) + (GYR * GYT)) * XQ)) * (GY / (GX * GYV)))) * GYX)) / GYW)) * AC;
                        GYA = GYY;
                        GYB = GYZ;
                    } else {
                        let GXS = GIG * UT;
                        let GXT = (GXS * GIL) - D;
                        let GXU = GXT * GIL;
                        let GXV = GAN * GII;
                        let GXW = D + (GXU * GAN);
                        let GXX = -(GXV * GXW);
                        let GXY = ((((GAO * GII) + Lanes([(GIJ * GAN), 0.0, 0.0, 0.0])) * GXW) + ((Lanes([((((((GIF * UT) * GIL) + (GIM * GXS)) * GIL) + (GIM * GXT)) * GAN), 0.0, 0.0, 0.0]) + (GAO * GXU)) * GXV)) * AC;
                        let GXZ = if (GXX.abs()) < SY { 1.0 } else { 0.0 };
                        let GZT;
                        let GZU;
                        if GXZ != 0.0 {
                            let GZQ = GXX.exp();
                            let GZR = GXY * GZQ;
                            GZT = GZQ;
                            GZU = GZR;
                        } else {
                            let GZS = if GXX < -8e1f64 { 1.0 } else { 0.0 };
                            let HAR;
                            let HAS;
                            if GZS != 0.0 {
                                let HAD = GXY * AC;
                                let HAE = (-GXX) - SY;
                                let HAF = GO * HAE;
                                let HAG = D + (HAE * WD);
                                let HAH = D + (HAF * HAG);
                                let HAI = D + (HAE * HAH);
                                let HAJ = YT / HAI;
                                let HAK = ((((HAD * HAH) + ((((HAD * GO) * HAG) + ((HAD * WD) * HAF)) * HAE)) * HAJ) * AC) / HAI;
                                HAR = HAJ;
                                HAS = HAK;
                            } else {
                                let HAL = GXX - SY;
                                let HAM = GO * HAL;
                                let HAN = D + (HAL * WD);
                                let HAO = D + (HAM * HAN);
                                let HAP = XB * (D + (HAL * HAO));
                                let HAQ = ((GXY * HAO) + ((((GXY * GO) * HAN) + ((GXY * WD) * HAM)) * HAL)) * XB;
                                HAR = HAP;
                                HAS = HAQ;
                            }
                            GZT = HAR;
                            GZU = HAS;
                        }
                        let GZV = GIC * GO;
                        let GZW = GIE * GO;
                        let GZX = ((GAN + (GIC * YY)) - (D - GZT)).sqrt();
                        let GZY = (GAN + GZV) - (GIA * GZX);
                        let GZZ = (GAO + Lanes([GZW, 0.0, 0.0, 0.0])) - (Lanes([(GIB * GZX), 0.0, 0.0, 0.0]) + ((((GAO + Lanes([(GIE * YY), 0.0, 0.0, 0.0])) - (GZU * AC)) * (GY / (GX * GZX))) * GIA));
                        let HAA = -GZY;
                        let HAB = GZZ * AC;
                        let HAC = if (HAA.abs()) < SY { 1.0 } else { 0.0 };
                        let HAW;
                        let HAX;
                        if HAC != 0.0 {
                            let HAT = HAA.exp();
                            let HAU = HAB * HAT;
                            HAW = HAT;
                            HAX = HAU;
                        } else {
                            let HAV = if HAA < -8e1f64 { 1.0 } else { 0.0 };
                            let HCC;
                            let HCD;
                            if HAV != 0.0 {
                                let HBO = HAB * AC;
                                let HBP = (-HAA) - SY;
                                let HBQ = GO * HBP;
                                let HBR = D + (HBP * WD);
                                let HBS = D + (HBQ * HBR);
                                let HBT = D + (HBP * HBS);
                                let HBU = YT / HBT;
                                let HBV = ((((HBO * HBS) + ((((HBO * GO) * HBR) + ((HBO * WD) * HBQ)) * HBP)) * HBU) * AC) / HBT;
                                HCC = HBU;
                                HCD = HBV;
                            } else {
                                let HBW = HAA - SY;
                                let HBX = GO * HBW;
                                let HBY = D + (HBW * WD);
                                let HBZ = D + (HBX * HBY);
                                let HCA = XB * (D + (HBW * HBZ));
                                let HCB = ((HAB * HBZ) + ((((HAB * GO) * HBY) + ((HAB * WD) * HBX)) * HBW)) * XB;
                                HCC = HCA;
                                HCD = HCB;
                            }
                            HAW = HCC;
                            HAX = HCD;
                        }
                        let HAY = GAN - GZY;
                        let HAZ = GAO - GZZ;
                        let HBA = D - HAW;
                        let HBB = (LA * HAY) + (GIC * HBA);
                        let HBC = (HAZ * LA) + (Lanes([(GIE * HBA), 0.0, 0.0, 0.0]) + ((HAX * AC) * GIC));
                        let HBD = HAZ * HAY;
                        let HBE = (GZY - D) + HAW;
                        let HBF = (HAY * HAY) - (GIC * HBE);
                        let HBG = (HBD + HBD) - (Lanes([(GIE * HBE), 0.0, 0.0, 0.0]) + ((GZZ + HAX) * GIC));
                        let HBH = D - (GZV * HAW);
                        let HBI = HBC * HBB;
                        let HBJ = ((HBB * HBB) - (XQ * (HBH * HBF))).sqrt();
                        let HBK = HBB + HBJ;
                        let HBL = (LA * HBF) / HBK;
                        let HBM = GZY + HBL;
                        let HBN = GZZ + (((HBG * LA) - ((HBC + (((HBI + HBI) - (((((Lanes([(GZW * HAW), 0.0, 0.0, 0.0]) + (HAX * GZV)) * AC) * HBF) + (HBG * HBH)) * XQ)) * (GY / (GX * HBJ)))) * HBL)) / HBK);
                        GYA = HBM;
                        GYB = HBN;
                    }
                    let GYC = -GYA;
                    let GYD = GYB * AC;
                    GWC = GYC;
                    GWD = GYD;
                }
                GVI = GWC;
                GVJ = GWD;
            } else {
                GVI = B;
                GVJ = GOX;
            }
            let GVK = -DJ;
            let GVL = FH * AC;
            let GVM = GAB + GBM;
            let GVN = GVK * GVM;
            let GVO = Lanes([(GVL * GVM), 0.0, 0.0]) + ((GAD + GBN) * GVK);
            let GVP = GAF + GOY;
            let GVQ = GVK * GVP;
            let GVR = Lanes([(GVL * GVP), 0.0, 0.0, 0.0]) + ((GAH + GOZ) * GVK);
            let GVS = GAL + GHW;
            let GVT = GVK * GVS;
            let GVU = Lanes([(GVL * GVS), 0.0, 0.0]) + ((GAM + GHX) * GVK);
            let GVV = GAN + GVI;
            let GVW = GVK * GVV;
            let GVX = Lanes([(GVL * GVV), 0.0, 0.0, 0.0]) + ((GAO + GVJ) * GVK);
            let HCE;
            let HCF;
            let HCG;
            let HCH;
            if GBH != 0.0 {
                let HCZ;
                let HDA;
                if GBG != 0.0 {
                    let HCJ = GVN + DK;
                    let HCK = GVO + Lanes([FI, 0.0, 0.0]);
                    let HCL = B - HCJ;
                    let HCM = (HCK * AC) * HCL;
                    let HCN = ((HCL * HCL) + NI).sqrt();
                    let HCO = GO * (HCJ - HCN);
                    let HCP = (HCK - ((HCM + HCM) * (GY / (GX * HCN)))) * GO;
                    let HCQ = GVO * GVN;
                    let HCS = ((GVN * GVN) + HCR).sqrt();
                    let HCU = HCS * HCT;
                    let HCV = ((HCQ + HCQ) * (GY / (GX * HCS))) * HCT;
                    let HCW = GO * GAB;
                    let HCX = GAD * GO;
                    let HCY = if (HCW.abs()) < SY { 1.0 } else { 0.0 };
                    let HDE;
                    let HDF;
                    if HCY != 0.0 {
                        let HDB = HCW.exp();
                        let HDC = HCX * HDB;
                        HDE = HDB;
                        HDF = HDC;
                    } else {
                        let HDD = if HCW < -8e1f64 { 1.0 } else { 0.0 };
                        let HEV;
                        let HEW;
                        if HDD != 0.0 {
                            let HEH = HCX * AC;
                            let HEI = (-HCW) - SY;
                            let HEJ = GO * HEI;
                            let HEK = D + (HEI * WD);
                            let HEL = D + (HEJ * HEK);
                            let HEM = D + (HEI * HEL);
                            let HEN = YT / HEM;
                            let HEO = ((((HEH * HEL) + ((((HEH * GO) * HEK) + ((HEH * WD) * HEJ)) * HEI)) * HEN) * AC) / HEM;
                            HEV = HEN;
                            HEW = HEO;
                        } else {
                            let HEP = HCW - SY;
                            let HEQ = GO * HEP;
                            let HER = D + (HEP * WD);
                            let HES = D + (HEQ * HER);
                            let HET = XB * (D + (HEP * HES));
                            let HEU = ((HCX * HES) + ((((HCX * GO) * HER) + ((HCX * WD) * HEQ)) * HEP)) * XB;
                            HEV = HET;
                            HEW = HEU;
                        }
                        HDE = HEV;
                        HDF = HEW;
                    }
                    let HDG = D + HDE;
                    let HDH = D / HDG;
                    let HDI = ((HDF * HDH) * AC) / HDG;
                    let HDJ = D - HDH;
                    let HDK = HDI * AC;
                    let HDN = (HDL * HDH) + (HDM * HDJ);
                    let HDO = (HDI * HDL) + (HDK * HDM);
                    let HDR = (HDP * HDH) + (HDQ * HDJ);
                    let HDS = (HDI * HDP) + (HDK * HDQ);
                    let HDV = (HDT * HDH) + (HDU * HDJ);
                    let HDW = (HDI * HDT) + (HDK * HDU);
                    let HDX = (DG * HDH) + (DF * HDJ);
                    let HDY = (Lanes([(FE * HDH), 0.0, 0.0]) + (HDI * DG)) + (Lanes([(FD * HDJ), 0.0, 0.0]) + (HDK * DF));
                    let HDZ = (DL * HDJ) * CPW;
                    let HEA = (Lanes([(FJ * HDJ), 0.0, 0.0]) + (HDK * DL)) * CPW;
                    let HEC = (-1e0f64 * HEB) / HCU;
                    let HEE = HED * HEC;
                    let HEF = (((HCV * HEC) * AC) / HCU) * HED;
                    let HEG = if HDR < B { 1.0 } else { 0.0 };
                    let HFC;
                    let HFD;
                    if HEG != 0.0 {
                        let HEX = HCU - HDV;
                        let HEY = (HCV - HDW) * HEX;
                        let HEZ = ((HEX * HEX) + CPW).sqrt();
                        let HFA = GO * ((HCU + HDV) - HEZ);
                        let HFB = ((HCV + HDW) - ((HEY + HEY) * (GY / (GX * HEZ)))) * GO;
                        HFC = HFA;
                        HFD = HFB;
                    } else {
                        HFC = HCU;
                        HFD = HCV;
                    }
                    let HFE = (ZD + GBM) + (HCO * DE);
                    let HFF = GBN + ((HCP * DE) + Lanes([(FC * HCO), 0.0, 0.0]));
                    let HFG = if (HFE.abs()) < SY { 1.0 } else { 0.0 };
                    let HFK;
                    let HFL;
                    if HFG != 0.0 {
                        let HFH = HFE.exp();
                        let HFI = HFF * HFH;
                        HFK = HFH;
                        HFL = HFI;
                    } else {
                        let HFJ = if HFE < -8e1f64 { 1.0 } else { 0.0 };
                        let HGD;
                        let HGE;
                        if HFJ != 0.0 {
                            let HFP = HFF * AC;
                            let HFQ = (-HFE) - SY;
                            let HFR = GO * HFQ;
                            let HFS = D + (HFQ * WD);
                            let HFT = D + (HFR * HFS);
                            let HFU = D + (HFQ * HFT);
                            let HFV = YT / HFU;
                            let HFW = ((((HFP * HFT) + ((((HFP * GO) * HFS) + ((HFP * WD) * HFR)) * HFQ)) * HFV) * AC) / HFU;
                            HGD = HFV;
                            HGE = HFW;
                        } else {
                            let HFX = HFE - SY;
                            let HFY = GO * HFX;
                            let HFZ = D + (HFX * WD);
                            let HGA = D + (HFY * HFZ);
                            let HGB = XB * (D + (HFX * HGA));
                            let HGC = ((HFF * HGA) + ((((HFF * GO) * HFZ) + ((HFF * WD) * HFY)) * HFX)) * XB;
                            HGD = HGB;
                            HGE = HGC;
                        }
                        HFK = HGD;
                        HFL = HGE;
                    }
                    let HFM = HFE + GAB;
                    let HFN = HFF + GAD;
                    let HFO = if (HFM.abs()) < SY { 1.0 } else { 0.0 };
                    let HGI;
                    let HGJ;
                    if HFO != 0.0 {
                        let HGF = HFM.exp();
                        let HGG = HFN * HGF;
                        HGI = HGF;
                        HGJ = HGG;
                    } else {
                        let HGH = if HFM < -8e1f64 { 1.0 } else { 0.0 };
                        let HHC;
                        let HHD;
                        if HGH != 0.0 {
                            let HGO = HFN * AC;
                            let HGP = (-HFM) - SY;
                            let HGQ = GO * HGP;
                            let HGR = D + (HGP * WD);
                            let HGS = D + (HGQ * HGR);
                            let HGT = D + (HGP * HGS);
                            let HGU = YT / HGT;
                            let HGV = ((((HGO * HGS) + ((((HGO * GO) * HGR) + ((HGO * WD) * HGQ)) * HGP)) * HGU) * AC) / HGT;
                            HHC = HGU;
                            HHD = HGV;
                        } else {
                            let HGW = HFM - SY;
                            let HGX = GO * HGW;
                            let HGY = D + (HGW * WD);
                            let HGZ = D + (HGX * HGY);
                            let HHA = XB * (D + (HGW * HGZ));
                            let HHB = ((HFN * HGZ) + ((((HFN * GO) * HGY) + ((HFN * WD) * HGX)) * HGW)) * XB;
                            HHC = HHA;
                            HHD = HHB;
                        }
                        HGI = HHC;
                        HGJ = HHD;
                    }
                    let HGK = HDN + (HDR * HFC);
                    let HGL = HED * (-1.5e0f64 + (HFC * HGK));
                    let HGM = ((HFD * HGK) + ((HDO + ((HDS * HFC) + (HFD * HDR))) * HFC)) * HED;
                    let HGN = if HGL > B { 1.0 } else { 0.0 };
                    let HHK;
                    let HHL;
                    if HGN != 0.0 {
                        let HHE = GO * HGL;
                        let HHF = D + (HGL * WD);
                        let HHG = D + (HHE * HHF);
                        let HHH = (HGM * HHG) + ((((HGM * GO) * HHF) + ((HGM * WD) * HHE)) * HGL);
                        let HHI = D + (HGL * HHG);
                        HHK = HHI;
                        HHL = HHH;
                    } else {
                        let HHJ = if HGL > -8e1f64 { 1.0 } else { 0.0 };
                        let HHX;
                        let HHY;
                        if HHJ != 0.0 {
                            let HHN = HGL.exp();
                            let HHO = HGM * HHN;
                            HHX = HHN;
                            HHY = HHO;
                        } else {
                            let HHP = HGM * AC;
                            let HHQ = (-HGL) - SY;
                            let HHR = GO * HHQ;
                            let HHS = D + (HHQ * WD);
                            let HHT = D + (HHR * HHS);
                            let HHU = D + (HHQ * HHT);
                            let HHV = YT / HHU;
                            let HHW = ((((HHP * HHT) + ((((HHP * GO) * HHS) + ((HHP * WD) * HHR)) * HHQ)) * HHV) * AC) / HHU;
                            HHX = HHV;
                            HHY = HHW;
                        }
                        HHK = HHX;
                        HHL = HHY;
                    }
                    let HHM = if HEE > B { 1.0 } else { 0.0 };
                    let HIF;
                    let HIG;
                    if HHM != 0.0 {
                        let HHZ = GO * HEE;
                        let HIA = D + (HEE * WD);
                        let HIB = D + (HHZ * HIA);
                        let HIC = (HEF * HIB) + ((((HEF * GO) * HIA) + ((HEF * WD) * HHZ)) * HEE);
                        let HID = D + (HEE * HIB);
                        HIF = HID;
                        HIG = HIC;
                    } else {
                        let HIE = if HEE > -8e1f64 { 1.0 } else { 0.0 };
                        let HIV;
                        let HIW;
                        if HIE != 0.0 {
                            let HIL = HEE.exp();
                            let HIM = HEF * HIL;
                            HIV = HIL;
                            HIW = HIM;
                        } else {
                            let HIN = HEF * AC;
                            let HIO = (-HEE) - SY;
                            let HIP = GO * HIO;
                            let HIQ = D + (HIO * WD);
                            let HIR = D + (HIP * HIQ);
                            let HIS = D + (HIO * HIR);
                            let HIT = YT / HIS;
                            let HIU = ((((HIN * HIR) + ((((HIN * GO) * HIQ) + ((HIN * WD) * HIP)) * HIO)) * HIT) * AC) / HIS;
                            HIV = HIT;
                            HIW = HIU;
                        }
                        HIF = HIV;
                        HIG = HIW;
                    }
                    let HIH = D + HGI;
                    let HII = (D + HFK) / HIH;
                    let HIJ = (HFL - (HGJ * HII)) / HIH;
                    let HIK = if HII < CJQ { 1.0 } else { 0.0 };
                    let HIX;
                    let HIY;
                    if HIK != 0.0 {
                        HIX = CJQ;
                        HIY = GBL;
                    } else {
                        HIX = HII;
                        HIY = HIJ;
                    }
                    let HJB = HJA * (QJ - HIZ);
                    let HJC = QL * HJA;
                    let HJD = if (HJB.abs()) < SY { 1.0 } else { 0.0 };
                    let HJH;
                    let HJI;
                    if HJD != 0.0 {
                        let HJE = HJB.exp();
                        let HJF = HJC * HJE;
                        HJH = HJE;
                        HJI = HJF;
                    } else {
                        let HJG = if HJB < -8e1f64 { 1.0 } else { 0.0 };
                        let HKB;
                        let HKC;
                        if HJG != 0.0 {
                            let HJN = HJC * AC;
                            let HJO = (-HJB) - SY;
                            let HJP = GO * HJO;
                            let HJQ = D + (HJO * WD);
                            let HJR = D + (HJP * HJQ);
                            let HJS = D + (HJO * HJR);
                            let HJT = YT / HJS;
                            let HJU = ((((HJN * HJR) + ((((HJN * GO) * HJQ) + ((HJN * WD) * HJP)) * HJO)) * HJT) * AC) / HJS;
                            HKB = HJT;
                            HKC = HJU;
                        } else {
                            let HJV = HJB - SY;
                            let HJW = GO * HJV;
                            let HJX = D + (HJV * WD);
                            let HJY = D + (HJW * HJX);
                            let HJZ = XB * (D + (HJV * HJY));
                            let HKA = ((HJC * HJY) + ((((HJC * GO) * HJX) + ((HJC * WD) * HJW)) * HJV)) * XB;
                            HKB = HJZ;
                            HKC = HKA;
                        }
                        HJH = HKB;
                        HJI = HKC;
                    }
                    let HJJ = QI * HJA;
                    let HJK = (HJA * QH) + HJB;
                    let HJL = Lanes([HJJ[0], HJJ[1], 0.0]) + HJC;
                    let HJM = if (HJK.abs()) < SY { 1.0 } else { 0.0 };
                    let HKG;
                    let HKH;
                    if HJM != 0.0 {
                        let HKD = HJK.exp();
                        let HKE = HJL * HKD;
                        HKG = HKD;
                        HKH = HKE;
                    } else {
                        let HKF = if HJK < -8e1f64 { 1.0 } else { 0.0 };
                        let HLM;
                        let HLN;
                        if HKF != 0.0 {
                            let HKY = HJL * AC;
                            let HKZ = (-HJK) - SY;
                            let HLA = GO * HKZ;
                            let HLB = D + (HKZ * WD);
                            let HLC = D + (HLA * HLB);
                            let HLD = D + (HKZ * HLC);
                            let HLE = YT / HLD;
                            let HLF = ((((HKY * HLC) + ((((HKY * GO) * HLB) + ((HKY * WD) * HLA)) * HKZ)) * HLE) * AC) / HLD;
                            HLM = HLE;
                            HLN = HLF;
                        } else {
                            let HLG = HJK - SY;
                            let HLH = GO * HLG;
                            let HLI = D + (HLG * WD);
                            let HLJ = D + (HLH * HLI);
                            let HLK = XB * (D + (HLG * HLJ));
                            let HLL = ((HJL * HLJ) + ((((HJL * GO) * HLI) + ((HJL * WD) * HLH)) * HLG)) * XB;
                            HLM = HLK;
                            HLN = HLL;
                        }
                        HKG = HLM;
                        HKH = HLN;
                    }
                    let HKI = HDX * HHK;
                    let HKJ = HIX.ln();
                    let HKK = HKI * HKJ;
                    let HKL = D + HJH;
                    let HKM = ((((HDY * HHK) + (HHL * HDX)) * HKJ) + ((HIY * (GY / HIX)) * HKI)) * HKL;
                    let HKN = HJI * HKK;
                    let HKO = D + HKG;
                    let HKP = (HKK * HKL) / HKO;
                    let HKQ = HKH * HKP;
                    let HKR = HDZ * HIF;
                    let HKS = ((HEA * HIF) + (HIG * HDZ)) * HKL;
                    let HKT = HJI * HKR;
                    let HKU = (HKR * HKL) / HKO;
                    let HKV = HKH * HKU;
                    let HKW = HKP - HKU;
                    let HKX = (((Lanes([HKM[0], HKM[1], 0.0, HKM[2]]) + Lanes([0.0, HKN[0], HKN[1], HKN[2]])) - Lanes([0.0, HKQ[0], HKQ[1], HKQ[2]])) / HKO) - (((Lanes([HKS[0], HKS[1], 0.0, HKS[2]]) + Lanes([0.0, HKT[0], HKT[1], HKT[2]])) - Lanes([0.0, HKV[0], HKV[1], HKV[2]])) / HKO);
                    HCZ = HKW;
                    HDA = HKX;
                } else {
                    HCZ = B;
                    HDA = GOX;
                }
                let HMC;
                let HMD;
                if GIN != 0.0 {
                    let HLO = GVQ + DK;
                    let HLP = GVR + Lanes([FI, 0.0, 0.0, 0.0]);
                    let HLQ = B - HLO;
                    let HLR = (HLP * AC) * HLQ;
                    let HLS = ((HLQ * HLQ) + NI).sqrt();
                    let HLT = GO * (HLO - HLS);
                    let HLU = (HLP - ((HLR + HLR) * (GY / (GX * HLS)))) * GO;
                    let HLV = GVR * GVQ;
                    let HLW = ((GVQ * GVQ) + HCR).sqrt();
                    let HLX = HLW * HCT;
                    let HLY = ((HLV + HLV) * (GY / (GX * HLW))) * HCT;
                    let HLZ = GO * GAF;
                    let HMA = GAH * GO;
                    let HMB = if (HLZ.abs()) < SY { 1.0 } else { 0.0 };
                    let HMI;
                    let HMJ;
                    if HMB != 0.0 {
                        let HMF = HLZ.exp();
                        let HMG = HMA * HMF;
                        HMI = HMF;
                        HMJ = HMG;
                    } else {
                        let HMH = if HLZ < -8e1f64 { 1.0 } else { 0.0 };
                        let HNR;
                        let HNS;
                        if HMH != 0.0 {
                            let HND = HMA * AC;
                            let HNE = (-HLZ) - SY;
                            let HNF = GO * HNE;
                            let HNG = D + (HNE * WD);
                            let HNH = D + (HNF * HNG);
                            let HNI = D + (HNE * HNH);
                            let HNJ = YT / HNI;
                            let HNK = ((((HND * HNH) + ((((HND * GO) * HNG) + ((HND * WD) * HNF)) * HNE)) * HNJ) * AC) / HNI;
                            HNR = HNJ;
                            HNS = HNK;
                        } else {
                            let HNL = HLZ - SY;
                            let HNM = GO * HNL;
                            let HNN = D + (HNL * WD);
                            let HNO = D + (HNM * HNN);
                            let HNP = XB * (D + (HNL * HNO));
                            let HNQ = ((HMA * HNO) + ((((HMA * GO) * HNN) + ((HMA * WD) * HNM)) * HNL)) * XB;
                            HNR = HNP;
                            HNS = HNQ;
                        }
                        HMI = HNR;
                        HMJ = HNS;
                    }
                    let HMK = D + HMI;
                    let HML = D / HMK;
                    let HMM = ((HMJ * HML) * AC) / HMK;
                    let HMN = D - HML;
                    let HMO = HMM * AC;
                    let HMP = (HDL * HML) + (HDM * HMN);
                    let HMQ = (HMM * HDL) + (HMO * HDM);
                    let HMR = (HDP * HML) + (HDQ * HMN);
                    let HMS = (HMM * HDP) + (HMO * HDQ);
                    let HMT = (HDT * HML) + (HDU * HMN);
                    let HMU = (HMM * HDT) + (HMO * HDU);
                    let HMV = (DI * HML) + (DH * HMN);
                    let HMW = (Lanes([(FG * HML), 0.0, 0.0, 0.0]) + (HMM * DI)) + (Lanes([(FF * HMN), 0.0, 0.0, 0.0]) + (HMO * DH));
                    let HMX = (DM * HMN) * CPW;
                    let HMY = (Lanes([(FK * HMN), 0.0, 0.0, 0.0]) + (HMO * DM)) * CPW;
                    let HMZ = (-1e0f64 * HEB) / HLX;
                    let HNA = HED * HMZ;
                    let HNB = (((HLY * HMZ) * AC) / HLX) * HED;
                    let HNC = if HMR < B { 1.0 } else { 0.0 };
                    let HNY;
                    let HNZ;
                    if HNC != 0.0 {
                        let HNT = HLX - HMT;
                        let HNU = (HLY - HMU) * HNT;
                        let HNV = ((HNT * HNT) + CPW).sqrt();
                        let HNW = GO * ((HLX + HMT) - HNV);
                        let HNX = ((HLY + HMU) - ((HNU + HNU) * (GY / (GX * HNV)))) * GO;
                        HNY = HNW;
                        HNZ = HNX;
                    } else {
                        HNY = HLX;
                        HNZ = HLY;
                    }
                    let HOA = (ZD + GOY) + (HLT * DE);
                    let HOB = GOZ + ((HLU * DE) + Lanes([(FC * HLT), 0.0, 0.0, 0.0]));
                    let HOC = if (HOA.abs()) < SY { 1.0 } else { 0.0 };
                    let HOG;
                    let HOH;
                    if HOC != 0.0 {
                        let HOD = HOA.exp();
                        let HOE = HOB * HOD;
                        HOG = HOD;
                        HOH = HOE;
                    } else {
                        let HOF = if HOA < -8e1f64 { 1.0 } else { 0.0 };
                        let HOZ;
                        let HPA;
                        if HOF != 0.0 {
                            let HOL = HOB * AC;
                            let HOM = (-HOA) - SY;
                            let HON = GO * HOM;
                            let HOO = D + (HOM * WD);
                            let HOP = D + (HON * HOO);
                            let HOQ = D + (HOM * HOP);
                            let HOR = YT / HOQ;
                            let HOS = ((((HOL * HOP) + ((((HOL * GO) * HOO) + ((HOL * WD) * HON)) * HOM)) * HOR) * AC) / HOQ;
                            HOZ = HOR;
                            HPA = HOS;
                        } else {
                            let HOT = HOA - SY;
                            let HOU = GO * HOT;
                            let HOV = D + (HOT * WD);
                            let HOW = D + (HOU * HOV);
                            let HOX = XB * (D + (HOT * HOW));
                            let HOY = ((HOB * HOW) + ((((HOB * GO) * HOV) + ((HOB * WD) * HOU)) * HOT)) * XB;
                            HOZ = HOX;
                            HPA = HOY;
                        }
                        HOG = HOZ;
                        HOH = HPA;
                    }
                    let HOI = HOA + GAF;
                    let HOJ = HOB + GAH;
                    let HOK = if (HOI.abs()) < SY { 1.0 } else { 0.0 };
                    let HPE;
                    let HPF;
                    if HOK != 0.0 {
                        let HPB = HOI.exp();
                        let HPC = HOJ * HPB;
                        HPE = HPB;
                        HPF = HPC;
                    } else {
                        let HPD = if HOI < -8e1f64 { 1.0 } else { 0.0 };
                        let HPY;
                        let HPZ;
                        if HPD != 0.0 {
                            let HPK = HOJ * AC;
                            let HPL = (-HOI) - SY;
                            let HPM = GO * HPL;
                            let HPN = D + (HPL * WD);
                            let HPO = D + (HPM * HPN);
                            let HPP = D + (HPL * HPO);
                            let HPQ = YT / HPP;
                            let HPR = ((((HPK * HPO) + ((((HPK * GO) * HPN) + ((HPK * WD) * HPM)) * HPL)) * HPQ) * AC) / HPP;
                            HPY = HPQ;
                            HPZ = HPR;
                        } else {
                            let HPS = HOI - SY;
                            let HPT = GO * HPS;
                            let HPU = D + (HPS * WD);
                            let HPV = D + (HPT * HPU);
                            let HPW = XB * (D + (HPS * HPV));
                            let HPX = ((HOJ * HPV) + ((((HOJ * GO) * HPU) + ((HOJ * WD) * HPT)) * HPS)) * XB;
                            HPY = HPW;
                            HPZ = HPX;
                        }
                        HPE = HPY;
                        HPF = HPZ;
                    }
                    let HPG = HMP + (HMR * HNY);
                    let HPH = HED * (-1.5e0f64 + (HNY * HPG));
                    let HPI = ((HNZ * HPG) + ((HMQ + ((HMS * HNY) + (HNZ * HMR))) * HNY)) * HED;
                    let HPJ = if HPH > B { 1.0 } else { 0.0 };
                    let HQG;
                    let HQH;
                    if HPJ != 0.0 {
                        let HQA = GO * HPH;
                        let HQB = D + (HPH * WD);
                        let HQC = D + (HQA * HQB);
                        let HQD = (HPI * HQC) + ((((HPI * GO) * HQB) + ((HPI * WD) * HQA)) * HPH);
                        let HQE = D + (HPH * HQC);
                        HQG = HQE;
                        HQH = HQD;
                    } else {
                        let HQF = if HPH > -8e1f64 { 1.0 } else { 0.0 };
                        let HQT;
                        let HQU;
                        if HQF != 0.0 {
                            let HQJ = HPH.exp();
                            let HQK = HPI * HQJ;
                            HQT = HQJ;
                            HQU = HQK;
                        } else {
                            let HQL = HPI * AC;
                            let HQM = (-HPH) - SY;
                            let HQN = GO * HQM;
                            let HQO = D + (HQM * WD);
                            let HQP = D + (HQN * HQO);
                            let HQQ = D + (HQM * HQP);
                            let HQR = YT / HQQ;
                            let HQS = ((((HQL * HQP) + ((((HQL * GO) * HQO) + ((HQL * WD) * HQN)) * HQM)) * HQR) * AC) / HQQ;
                            HQT = HQR;
                            HQU = HQS;
                        }
                        HQG = HQT;
                        HQH = HQU;
                    }
                    let HQI = if HNA > B { 1.0 } else { 0.0 };
                    let HRB;
                    let HRC;
                    if HQI != 0.0 {
                        let HQV = GO * HNA;
                        let HQW = D + (HNA * WD);
                        let HQX = D + (HQV * HQW);
                        let HQY = (HNB * HQX) + ((((HNB * GO) * HQW) + ((HNB * WD) * HQV)) * HNA);
                        let HQZ = D + (HNA * HQX);
                        HRB = HQZ;
                        HRC = HQY;
                    } else {
                        let HRA = if HNA > -8e1f64 { 1.0 } else { 0.0 };
                        let HRR;
                        let HRS;
                        if HRA != 0.0 {
                            let HRH = HNA.exp();
                            let HRI = HNB * HRH;
                            HRR = HRH;
                            HRS = HRI;
                        } else {
                            let HRJ = HNB * AC;
                            let HRK = (-HNA) - SY;
                            let HRL = GO * HRK;
                            let HRM = D + (HRK * WD);
                            let HRN = D + (HRL * HRM);
                            let HRO = D + (HRK * HRN);
                            let HRP = YT / HRO;
                            let HRQ = ((((HRJ * HRN) + ((((HRJ * GO) * HRM) + ((HRJ * WD) * HRL)) * HRK)) * HRP) * AC) / HRO;
                            HRR = HRP;
                            HRS = HRQ;
                        }
                        HRB = HRR;
                        HRC = HRS;
                    }
                    let HRD = D + HPE;
                    let HRE = (D + HOG) / HRD;
                    let HRF = (HOH - (HPF * HRE)) / HRD;
                    let HRG = if HRE < CJQ { 1.0 } else { 0.0 };
                    let HRT;
                    let HRU;
                    if HRG != 0.0 {
                        HRT = CJQ;
                        HRU = GOX;
                    } else {
                        HRT = HRE;
                        HRU = HRF;
                    }
                    let HRV = HJA * (QC - HIZ);
                    let HRW = QF * HJA;
                    let HRX = if (HRV.abs()) < SY { 1.0 } else { 0.0 };
                    let HSB;
                    let HSC;
                    if HRX != 0.0 {
                        let HRY = HRV.exp();
                        let HRZ = HRW * HRY;
                        HSB = HRY;
                        HSC = HRZ;
                    } else {
                        let HSA = if HRV < -8e1f64 { 1.0 } else { 0.0 };
                        let HSV;
                        let HSW;
                        if HSA != 0.0 {
                            let HSH = HRW * AC;
                            let HSI = (-HRV) - SY;
                            let HSJ = GO * HSI;
                            let HSK = D + (HSI * WD);
                            let HSL = D + (HSJ * HSK);
                            let HSM = D + (HSI * HSL);
                            let HSN = YT / HSM;
                            let HSO = ((((HSH * HSL) + ((((HSH * GO) * HSK) + ((HSH * WD) * HSJ)) * HSI)) * HSN) * AC) / HSM;
                            HSV = HSN;
                            HSW = HSO;
                        } else {
                            let HSP = HRV - SY;
                            let HSQ = GO * HSP;
                            let HSR = D + (HSP * WD);
                            let HSS = D + (HSQ * HSR);
                            let HST = XB * (D + (HSP * HSS));
                            let HSU = ((HRW * HSS) + ((((HRW * GO) * HSR) + ((HRW * WD) * HSQ)) * HSP)) * XB;
                            HSV = HST;
                            HSW = HSU;
                        }
                        HSB = HSV;
                        HSC = HSW;
                    }
                    let HSD = QE * HJA;
                    let HSE = (HJA * QB) + HRV;
                    let HSF = Lanes([HSD[0], HSD[1], 0.0]) + Lanes([HRW[0], 0.0, HRW[1]]);
                    let HSG = if (HSE.abs()) < SY { 1.0 } else { 0.0 };
                    let HTA;
                    let HTB;
                    if HSG != 0.0 {
                        let HSX = HSE.exp();
                        let HSY = HSF * HSX;
                        HTA = HSX;
                        HTB = HSY;
                    } else {
                        let HSZ = if HSE < -8e1f64 { 1.0 } else { 0.0 };
                        let HUE;
                        let HUF;
                        if HSZ != 0.0 {
                            let HTQ = HSF * AC;
                            let HTR = (-HSE) - SY;
                            let HTS = GO * HTR;
                            let HTT = D + (HTR * WD);
                            let HTU = D + (HTS * HTT);
                            let HTV = D + (HTR * HTU);
                            let HTW = YT / HTV;
                            let HTX = ((((HTQ * HTU) + ((((HTQ * GO) * HTT) + ((HTQ * WD) * HTS)) * HTR)) * HTW) * AC) / HTV;
                            HUE = HTW;
                            HUF = HTX;
                        } else {
                            let HTY = HSE - SY;
                            let HTZ = GO * HTY;
                            let HUA = D + (HTY * WD);
                            let HUB = D + (HTZ * HUA);
                            let HUC = XB * (D + (HTY * HUB));
                            let HUD = ((HSF * HUB) + ((((HSF * GO) * HUA) + ((HSF * WD) * HTZ)) * HTY)) * XB;
                            HUE = HUC;
                            HUF = HUD;
                        }
                        HTA = HUE;
                        HTB = HUF;
                    }
                    let HTC = HMV * HQG;
                    let HTD = HRT.ln();
                    let HTE = HTC * HTD;
                    let HTF = D + HSB;
                    let HTG = HSC * HTE;
                    let HTH = D + HTA;
                    let HTI = (HTE * HTF) / HTH;
                    let HTJ = HTB * HTI;
                    let HTK = HMX * HRB;
                    let HTL = HSC * HTK;
                    let HTM = (HTK * HTF) / HTH;
                    let HTN = HTB * HTM;
                    let HTO = HTI - HTM;
                    let HTP = ((((((((HMW * HQG) + (HQH * HMV)) * HTD) + ((HRU * (GY / HRT)) * HTC)) * HTF) + Lanes([0.0, HTG[0], 0.0, HTG[1]])) - Lanes([0.0, HTJ[0], HTJ[1], HTJ[2]])) / HTH) - ((((((HMY * HRB) + (HRC * HMX)) * HTF) + Lanes([0.0, HTL[0], 0.0, HTL[1]])) - Lanes([0.0, HTN[0], HTN[1], HTN[2]])) / HTH);
                    HMC = HTO;
                    HMD = HTP;
                } else {
                    HMC = B;
                    HMD = GOX;
                }
                let HME = if DN > B { 1.0 } else { 0.0 };
                let HUM;
                let HUN;
                let HUO;
                let HUP;
                if HME != 0.0 {
                    let HUG = -FXY;
                    let HUH = HUG * AIX;
                    let HUI = ((FXZ * AC) * AIX) + (AIY * HUG);
                    let HUJ = (LA * HUH) - DGH;
                    let HUK = (HUI * LA) - DGJ;
                    let HUL = if (HUJ.abs()) < SY { 1.0 } else { 0.0 };
                    let HUU;
                    let HUV;
                    if HUL != 0.0 {
                        let HUR = HUJ.exp();
                        let HUS = HUK * HUR;
                        HUU = HUR;
                        HUV = HUS;
                    } else {
                        let HUT = if HUJ < -8e1f64 { 1.0 } else { 0.0 };
                        let HWD;
                        let HWE;
                        if HUT != 0.0 {
                            let HVP = HUK * AC;
                            let HVQ = (-HUJ) - SY;
                            let HVR = GO * HVQ;
                            let HVS = D + (HVQ * WD);
                            let HVT = D + (HVR * HVS);
                            let HVU = D + (HVQ * HVT);
                            let HVV = YT / HVU;
                            let HVW = ((((HVP * HVT) + ((((HVP * GO) * HVS) + ((HVP * WD) * HVR)) * HVQ)) * HVV) * AC) / HVU;
                            HWD = HVV;
                            HWE = HVW;
                        } else {
                            let HVX = HUJ - SY;
                            let HVY = GO * HVX;
                            let HVZ = D + (HVX * WD);
                            let HWA = D + (HVY * HVZ);
                            let HWB = XB * (D + (HVX * HWA));
                            let HWC = ((HUK * HWA) + ((((HUK * GO) * HVZ) + ((HUK * WD) * HVY)) * HVX)) * XB;
                            HWD = HWB;
                            HWE = HWC;
                        }
                        HUU = HWD;
                        HUV = HWE;
                    }
                    let HUW = D + HUU;
                    let HUX = (HUH + HW) - (HUW.ln());
                    let HUY = DD * HUX;
                    let HUZ = Lanes([(FB * HUX), 0.0, 0.0, 0.0, 0.0]) + ((HUI - (HUV * (GY / HUW))) * DD);
                    let HVA = GO * (BYH + ERO);
                    let HVB = (BYK + ERR) * GO;
                    let HVC = DD * HVA;
                    let HVD = Lanes([(FB * HVA), 0.0, 0.0, 0.0, 0.0]) + (HVB * DD);
                    let HVE = HVC + DO;
                    let HVF = HVD + Lanes([FM, 0.0, 0.0, 0.0, 0.0]);
                    let HVG = B - HVE;
                    let HVH = (HVF * AC) * HVG;
                    let HVI = ((HVG * HVG) + NI).sqrt();
                    let HVJ = GO * (HVE - HVI);
                    let HVK = (HVF - ((HVH + HVH) * (GY / (GX * HVI)))) * GO;
                    let HVL = HVD * HVC;
                    let HVM = ((HVC * HVC) + HCR).sqrt();
                    let HVN = HVM * HCT;
                    let HVO = ((HVL + HVL) * (GY / (GX * HVM))) * HCT;
                    let HWL;
                    let HWM;
                    if N != 0.0 {
                        let HWG = HVN - HWF;
                        let HWH = HVO * HWG;
                        let HWI = ((HWG * HWG) + CPW).sqrt();
                        let HWJ = GO * ((HVN + HWF) - HWI);
                        let HWK = (HVO - ((HWH + HWH) * (GY / (GX * HWI)))) * GO;
                        HWL = HWJ;
                        HWM = HWK;
                    } else {
                        HWL = HVN;
                        HWM = HVO;
                    }
                    let HWN = AIH + CN;
                    let HWO = (HVJ - DP) - HUY;
                    let HWP = (HWN - HVA) + (HWO * CH);
                    let HWQ = HWP * DQ;
                    let HWR = ((((AII + Lanes([EL, 0.0, 0.0, 0.0, 0.0])) - HVB) + ((((HVK - Lanes([FN, 0.0, 0.0, 0.0, 0.0])) - HUZ) * CH) + Lanes([(EF * HWO), 0.0, 0.0, 0.0, 0.0]))) * DQ) + Lanes([(FO * HWP), 0.0, 0.0, 0.0, 0.0]);
                    let HWS = if (HWQ.abs()) < SY { 1.0 } else { 0.0 };
                    let HWW;
                    let HWX;
                    if HWS != 0.0 {
                        let HWT = HWQ.exp();
                        let HWU = HWR * HWT;
                        HWW = HWT;
                        HWX = HWU;
                    } else {
                        let HWV = if HWQ < -8e1f64 { 1.0 } else { 0.0 };
                        let HXR;
                        let HXS;
                        if HWV != 0.0 {
                            let HXD = HWR * AC;
                            let HXE = (-HWQ) - SY;
                            let HXF = GO * HXE;
                            let HXG = D + (HXE * WD);
                            let HXH = D + (HXF * HXG);
                            let HXI = D + (HXE * HXH);
                            let HXJ = YT / HXI;
                            let HXK = ((((HXD * HXH) + ((((HXD * GO) * HXG) + ((HXD * WD) * HXF)) * HXE)) * HXJ) * AC) / HXI;
                            HXR = HXJ;
                            HXS = HXK;
                        } else {
                            let HXL = HWQ - SY;
                            let HXM = GO * HXL;
                            let HXN = D + (HXL * WD);
                            let HXO = D + (HXM * HXN);
                            let HXP = XB * (D + (HXL * HXO));
                            let HXQ = ((HWR * HXO) + ((((HWR * GO) * HXN) + ((HWR * WD) * HXM)) * HXL)) * XB;
                            HXR = HXP;
                            HXS = HXQ;
                        }
                        HWW = HXR;
                        HWX = HXS;
                    }
                    let HWY = -(QR - HUY);
                    let HWZ = HWY * CH;
                    let HXA = HWZ * DQ;
                    let HXB = (((((Lanes([0.0, QV[0], QV[1], 0.0, QV[2]]) - HUZ) * AC) * CH) + Lanes([(EF * HWY), 0.0, 0.0, 0.0, 0.0])) * DQ) + Lanes([(FO * HWZ), 0.0, 0.0, 0.0, 0.0]);
                    let HXC = if (HXA.abs()) < SY { 1.0 } else { 0.0 };
                    let HXW;
                    let HXX;
                    if HXC != 0.0 {
                        let HXT = HXA.exp();
                        let HXU = HXB * HXT;
                        HXW = HXT;
                        HXX = HXU;
                    } else {
                        let HXV = if HXA < -8e1f64 { 1.0 } else { 0.0 };
                        let HYU;
                        let HYV;
                        if HXV != 0.0 {
                            let HYG = HXB * AC;
                            let HYH = (-HXA) - SY;
                            let HYI = GO * HYH;
                            let HYJ = D + (HYH * WD);
                            let HYK = D + (HYI * HYJ);
                            let HYL = D + (HYH * HYK);
                            let HYM = YT / HYL;
                            let HYN = ((((HYG * HYK) + ((((HYG * GO) * HYJ) + ((HYG * WD) * HYI)) * HYH)) * HYM) * AC) / HYL;
                            HYU = HYM;
                            HYV = HYN;
                        } else {
                            let HYO = HXA - SY;
                            let HYP = GO * HYO;
                            let HYQ = D + (HYO * WD);
                            let HYR = D + (HYP * HYQ);
                            let HYS = XB * (D + (HYO * HYR));
                            let HYT = ((HXB * HYR) + ((((HXB * GO) * HYQ) + ((HXB * WD) * HYP)) * HYO)) * XB;
                            HYU = HYS;
                            HYV = HYT;
                        }
                        HXW = HYU;
                        HXX = HYV;
                    }
                    let HXY = HWW * HXW;
                    let HXZ = (HWX * HXW) + (HXX * HWW);
                    let HYC = HYB + (HYA * HWL);
                    let HYD = HED * (-1.5e0f64 + (HWL * HYC));
                    let HYE = ((HWM * HYC) + ((HWM * HYA) * HWL)) * HED;
                    let HYF = if HYD > B { 1.0 } else { 0.0 };
                    let HZC;
                    let HZD;
                    if HYF != 0.0 {
                        let HYW = GO * HYD;
                        let HYX = D + (HYD * WD);
                        let HYY = D + (HYW * HYX);
                        let HYZ = (HYE * HYY) + ((((HYE * GO) * HYX) + ((HYE * WD) * HYW)) * HYD);
                        let HZA = D + (HYD * HYY);
                        HZC = HZA;
                        HZD = HYZ;
                    } else {
                        let HZB = if (HYD.abs()) < SY { 1.0 } else { 0.0 };
                        let HZO;
                        let HZP;
                        if HZB != 0.0 {
                            let HZL = HYD.exp();
                            let HZM = HYE * HZL;
                            HZO = HZL;
                            HZP = HZM;
                        } else {
                            let HZN = if HYD < -8e1f64 { 1.0 } else { 0.0 };
                            let IAE;
                            let IAF;
                            if HZN != 0.0 {
                                let HZQ = HYE * AC;
                                let HZR = (-HYD) - SY;
                                let HZS = GO * HZR;
                                let HZT = D + (HZR * WD);
                                let HZU = D + (HZS * HZT);
                                let HZV = D + (HZR * HZU);
                                let HZW = YT / HZV;
                                let HZX = ((((HZQ * HZU) + ((((HZQ * GO) * HZT) + ((HZQ * WD) * HZS)) * HZR)) * HZW) * AC) / HZV;
                                IAE = HZW;
                                IAF = HZX;
                            } else {
                                let HZY = HYD - SY;
                                let HZZ = GO * HZY;
                                let IAA = D + (HZY * WD);
                                let IAB = D + (HZZ * IAA);
                                let IAC = XB * (D + (HZY * IAB));
                                let IAD = ((HYE * IAB) + ((((HYE * GO) * IAA) + ((HYE * WD) * HZZ)) * HZY)) * XB;
                                IAE = IAC;
                                IAF = IAD;
                            }
                            HZO = IAE;
                            HZP = IAF;
                        }
                        HZC = HZO;
                        HZD = HZP;
                    }
                    let HZE = DN * HZC;
                    let HZF = D + HXY;
                    let HZG = (D + HWW) / HZF;
                    let HZH = HZG.ln();
                    let HZI = HZE * HZH;
                    let HZJ = ((Lanes([(FL * HZC), 0.0, 0.0, 0.0, 0.0]) + (HZD * DN)) * HZH) + ((((HWX - (HXZ * HZG)) / HZF) * (GY / HZG)) * HZE);
                    let HZK = if (if HWN <= B { 1.0 } else { 0.0 }) != 0.0 || (if (if HYB == B { 1.0 } else { 0.0 }) != 0.0 && (if HYA == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let IAX;
                    let IAY;
                    let IAZ;
                    let IBA;
                    if HZK != 0.0 {
                        IAX = D;
                        IAY = GO;
                        IAZ = AFD;
                        IBA = AFD;
                    } else {
                        let IAG = LA * HYA;
                        let IAH = (HYB + (IAG * HWL)) * HED;
                        let IAI = staged[220] / IAH;
                        let IAJ = IAI * CH;
                        let IAK = ((((((HWM * IAG) * HED) * IAI) * AC) / IAH) * CH) + Lanes([(EF * IAI), 0.0, 0.0, 0.0, 0.0]);
                        let IAL = HUH / IAJ;
                        let IAM = (HUI - (IAK * IAL)) / IAJ;
                        let IAN = IAJ * FXV;
                        let IAO = IAN * AIR;
                        let IAP = (((IAK * FXV) + (FXW * IAJ)) * AIR) + (AIT * IAN);
                        let IAQ = D - IAO;
                        let IAR = IAP * AC;
                        let IAS = (IAO * IAQ) * GO;
                        let IAT = ((IAP * IAQ) + (IAR * IAO)) * GO;
                        let IAU = GO - (ZD * IAS);
                        let IAV = (IAT * ZD) * AC;
                        let IAW = if IAL < FTT { 1.0 } else { 0.0 };
                        let IBZ;
                        let ICA;
                        let ICB;
                        let ICC;
                        if IAW != 0.0 {
                            let IBF = IAL * IAL;
                            let IBG = IAM * IAL;
                            let IBH = IBG + IBG;
                            let IBI = IBF * UC;
                            let IBJ = ATB + (CTP * IAO);
                            let IBK = (UC + (IAO * WD)) + (IBI * IBJ);
                            let IBL = (IBH * IBK) + (((IAP * WD) + (((IBH * UC) * IBJ) + ((IAP * CTP) * IBI))) * IBF);
                            let IBM = D + (IBF * IBK);
                            let IBN = IAL * UC;
                            let IBQ = IBP * IBF;
                            let IBR = 1.25e-1f64 + IAS;
                            let IBS = (IBO * (IAS + YY)) + (IBQ * IBR);
                            let IBT = D + (IBF * IBS);
                            let IBU = (GO * IBM) - (IBN * IBT);
                            let IBV = (IBL * GO) - (((IAM * UC) * IBT) + (((IBH * IBS) + (((IAT * IBO) + (((IBH * IBP) * IBR) + (IAT * IBQ))) * IBF)) * IBN));
                            IBZ = IBM;
                            ICA = IBU;
                            ICB = IBL;
                            ICC = IBV;
                        } else {
                            let IBW = D / IAL;
                            let IBX = ((IAM * IBW) * AC) / IAL;
                            let IBY = if (IAL.abs()) < SY { 1.0 } else { 0.0 };
                            let ICG;
                            let ICH;
                            if IBY != 0.0 {
                                let ICD = IAL.exp();
                                let ICE = IAM * ICD;
                                ICG = ICD;
                                ICH = ICE;
                            } else {
                                let ICF = if IAL < -8e1f64 { 1.0 } else { 0.0 };
                                let IDK;
                                let IDL;
                                if ICF != 0.0 {
                                    let ICW = IAM * AC;
                                    let ICX = (-IAL) - SY;
                                    let ICY = GO * ICX;
                                    let ICZ = D + (ICX * WD);
                                    let IDA = D + (ICY * ICZ);
                                    let IDB = D + (ICX * IDA);
                                    let IDC = YT / IDB;
                                    let IDD = ((((ICW * IDA) + ((((ICW * GO) * ICZ) + ((ICW * WD) * ICY)) * ICX)) * IDC) * AC) / IDB;
                                    IDK = IDC;
                                    IDL = IDD;
                                } else {
                                    let IDE = IAL - SY;
                                    let IDF = GO * IDE;
                                    let IDG = D + (IDE * WD);
                                    let IDH = D + (IDF * IDG);
                                    let IDI = XB * (D + (IDE * IDH));
                                    let IDJ = ((IAM * IDH) + ((((IAM * GO) * IDG) + ((IAM * WD) * IDF)) * IDE)) * XB;
                                    IDK = IDI;
                                    IDL = IDJ;
                                }
                                ICG = IDK;
                                ICH = IDL;
                            }
                            let ICI = D / ICG;
                            let ICJ = ((ICH * ICI) * AC) / ICG;
                            let ICK = ICG - ICI;
                            let ICL = ICH - ICJ;
                            let ICM = ICG + ICI;
                            let ICN = ICH + ICJ;
                            let ICO = IAQ * ICK;
                            let ICP = GO * ((ICO * IBW) + (IAO * ICM));
                            let ICQ = (((((IAR * ICK) + (ICL * IAQ)) * IBW) + (IBX * ICO)) + ((IAP * ICM) + (ICN * IAO))) * GO;
                            let ICR = IAU * IBW;
                            let ICS = IAS - (ICR * IBW);
                            let ICT = IAU * ICM;
                            let ICU = GO * ((ICP - (ICK * ICS)) - (ICT * IBW));
                            let ICV = ((ICQ - ((ICL * ICS) + ((IAT - ((((IAV * IBW) + (IBX * IAU)) * IBW) + (IBX * ICR))) * ICK))) - ((((IAV * ICM) + (ICN * IAU)) * IBW) + (IBX * ICT))) * GO;
                            IBZ = ICP;
                            ICA = ICU;
                            ICB = ICQ;
                            ICC = ICV;
                        }
                        IAX = IBZ;
                        IAY = ICA;
                        IAZ = ICB;
                        IBA = ICC;
                    }
                    let IBB = HZI * IAY;
                    let IBC = (HZJ * IAY) + (IBA * HZI);
                    let IBD = (HZI * IAX) - IBB;
                    let IBE = ((HZJ * IAX) + (IAZ * HZI)) - IBC;
                    HUM = IBB;
                    HUN = IBD;
                    HUO = IBC;
                    HUP = IBE;
                } else {
                    HUM = B;
                    HUN = B;
                    HUO = AFD;
                    HUP = AFD;
                }
                let HUQ = if QU < B { 1.0 } else { 0.0 };
                let IDU;
                let IDV;
                let IDW;
                let IDX;
                if HUQ != 0.0 {
                    let IDM = HUM + HCZ;
                    let IDN = HUO + Lanes([HDA[0], HDA[1], HDA[2], 0.0, HDA[3]]);
                    let IDO = HUN + HMC;
                    let IDP = HUP + Lanes([HMD[0], HMD[1], HMD[2], 0.0, HMD[3]]);
                    IDU = IDM;
                    IDV = IDO;
                    IDW = IDN;
                    IDX = IDP;
                } else {
                    let IDQ = HUN + HCZ;
                    let IDR = HUP + Lanes([HDA[0], HDA[1], HDA[2], 0.0, HDA[3]]);
                    let IDS = HUM + HMC;
                    let IDT = HUO + Lanes([HMD[0], HMD[1], HMD[2], 0.0, HMD[3]]);
                    IDU = IDQ;
                    IDV = IDS;
                    IDW = IDR;
                    IDX = IDT;
                }
                HCE = IDU;
                HCF = IDV;
                HCG = IDW;
                HCH = IDX;
            } else {
                HCE = B;
                HCF = B;
                HCG = AFD;
                HCH = AFD;
            }
            let HCI = if GBI != 0.0 && (if GVN < B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IEJ;
            let IEK;
            if HCI != 0.0 {
                let IDY = GVO * GVN;
                let IDZ = IDY + IDY;
                let IEB = IEA * IEA;
                let IEC = IEB * QD;
                let IED = ((QG * IEB) * QD) + (QG * IEC);
                let IEE = (((GVN * GVN) + (IEC * QD)) + CPW).sqrt();
                let IEF = (Lanes([IDZ[0], IDZ[1], 0.0, IDZ[2]]) + Lanes([0.0, IED[0], IED[1], 0.0])) * (GY / (GX * IEE));
                let IEG = (-DR) / IEE;
                let IEH = (Lanes([(FP * AC), 0.0, 0.0, 0.0]) - (IEF * IEG)) / IEE;
                let IEI = if (IEG.abs()) < SY { 1.0 } else { 0.0 };
                let IEP;
                let IEQ;
                if IEI != 0.0 {
                    let IEM = IEG.exp();
                    let IEN = IEH * IEM;
                    IEP = IEM;
                    IEQ = IEN;
                } else {
                    let IEO = if IEG < -8e1f64 { 1.0 } else { 0.0 };
                    let IFJ;
                    let IFK;
                    if IEO != 0.0 {
                        let IEV = IEH * AC;
                        let IEW = (-IEG) - SY;
                        let IEX = GO * IEW;
                        let IEY = D + (IEW * WD);
                        let IEZ = D + (IEX * IEY);
                        let IFA = D + (IEW * IEZ);
                        let IFB = YT / IFA;
                        let IFC = ((((IEV * IEZ) + ((((IEV * GO) * IEY) + ((IEV * WD) * IEX)) * IEW)) * IFB) * AC) / IFA;
                        IFJ = IFB;
                        IFK = IFC;
                    } else {
                        let IFD = IEG - SY;
                        let IFE = GO * IFD;
                        let IFF = D + (IFD * WD);
                        let IFG = D + (IFE * IFF);
                        let IFH = XB * (D + (IFD * IFG));
                        let IFI = ((IEH * IFG) + ((((IEH * GO) * IFF) + ((IEH * WD) * IFE)) * IFD)) * XB;
                        IFJ = IFH;
                        IFK = IFI;
                    }
                    IEP = IFJ;
                    IEQ = IFK;
                }
                let IES = IER * QH;
                let IET = QI * IER;
                let IEU = if (IES.abs()) < SY { 1.0 } else { 0.0 };
                let IFO;
                let IFP;
                if IEU != 0.0 {
                    let IFL = IES.exp();
                    let IFM = IET * IFL;
                    IFO = IFL;
                    IFP = IFM;
                } else {
                    let IFN = if IES < -8e1f64 { 1.0 } else { 0.0 };
                    let IGS;
                    let IGT;
                    if IFN != 0.0 {
                        let IGE = IET * AC;
                        let IGF = (-IES) - SY;
                        let IGG = GO * IGF;
                        let IGH = D + (IGF * WD);
                        let IGI = D + (IGG * IGH);
                        let IGJ = D + (IGF * IGI);
                        let IGK = YT / IGJ;
                        let IGL = ((((IGE * IGI) + ((((IGE * GO) * IGH) + ((IGE * WD) * IGG)) * IGF)) * IGK) * AC) / IGJ;
                        IGS = IGK;
                        IGT = IGL;
                    } else {
                        let IGM = IES - SY;
                        let IGN = GO * IGM;
                        let IGO = D + (IGM * WD);
                        let IGP = D + (IGN * IGO);
                        let IGQ = XB * (D + (IGM * IGP));
                        let IGR = ((IET * IGP) + ((((IET * GO) * IGO) + ((IET * WD) * IGN)) * IGM)) * XB;
                        IGS = IGQ;
                        IGT = IGR;
                    }
                    IFO = IGS;
                    IFP = IGT;
                }
                let IFQ = -staged[223];
                let IFR = IFQ * QH;
                let IFS = IFR * GVN;
                let IFT = (QI * IFQ) * GVN;
                let IFU = GVO * IFR;
                let IFV = IFS * IEE;
                let IFW = (Lanes([0.0, IFT[0], IFT[1], 0.0]) + Lanes([IFU[0], IFU[1], 0.0, IFU[2]])) * IEE;
                let IFX = IEF * IFS;
                let IFY = IEQ * IFV;
                let IFZ = (IFV * IEP) * GO;
                let IGA = D + IFO;
                let IGB = IFZ * IGA;
                let IGC = IFP * IFZ;
                let IGD = (((((Lanes([IFW[0], IFW[1], IFW[2], 0.0, IFW[3]]) + Lanes([IFX[0], IFX[1], 0.0, IFX[2], IFX[3]])) * IEP) + Lanes([IFY[0], IFY[1], 0.0, IFY[2], IFY[3]])) * GO) * IGA) + Lanes([0.0, IGC[0], IGC[1], 0.0, 0.0]);
                IEJ = IGB;
                IEK = IGD;
            } else {
                IEJ = B;
                IEK = AFD;
            }
            let IEL = if GIO != 0.0 && (if GVQ < B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IHF;
            let IHG;
            if IEL != 0.0 {
                let IGU = GVR * GVQ;
                let IGV = IGU + IGU;
                let IGX = IGW * IGW;
                let IGY = IGX * QM;
                let IGZ = ((QO * IGX) * QM) + (QO * IGY);
                let IHA = (((GVQ * GVQ) + (IGY * QM)) + CPW).sqrt();
                let IHB = (Lanes([IGV[0], IGV[1], IGV[2], 0.0, IGV[3]]) + Lanes([0.0, IGZ[0], IGZ[1], IGZ[2], 0.0])) * (GY / (GX * IHA));
                let IHC = (-DS) / IHA;
                let IHD = (Lanes([(FQ * AC), 0.0, 0.0, 0.0, 0.0]) - (IHB * IHC)) / IHA;
                let IHE = if (IHC.abs()) < SY { 1.0 } else { 0.0 };
                let IHL;
                let IHM;
                if IHE != 0.0 {
                    let IHI = IHC.exp();
                    let IHJ = IHD * IHI;
                    IHL = IHI;
                    IHM = IHJ;
                } else {
                    let IHK = if IHC < -8e1f64 { 1.0 } else { 0.0 };
                    let IIF;
                    let IIG;
                    if IHK != 0.0 {
                        let IHR = IHD * AC;
                        let IHS = (-IHC) - SY;
                        let IHT = GO * IHS;
                        let IHU = D + (IHS * WD);
                        let IHV = D + (IHT * IHU);
                        let IHW = D + (IHS * IHV);
                        let IHX = YT / IHW;
                        let IHY = ((((IHR * IHV) + ((((IHR * GO) * IHU) + ((IHR * WD) * IHT)) * IHS)) * IHX) * AC) / IHW;
                        IIF = IHX;
                        IIG = IHY;
                    } else {
                        let IHZ = IHC - SY;
                        let IIA = GO * IHZ;
                        let IIB = D + (IHZ * WD);
                        let IIC = D + (IIA * IIB);
                        let IID = XB * (D + (IHZ * IIC));
                        let IIE = ((IHD * IIC) + ((((IHD * GO) * IIB) + ((IHD * WD) * IIA)) * IHZ)) * XB;
                        IIF = IID;
                        IIG = IIE;
                    }
                    IHL = IIF;
                    IHM = IIG;
                }
                let IHO = IHN * QB;
                let IHP = QE * IHN;
                let IHQ = if (IHO.abs()) < SY { 1.0 } else { 0.0 };
                let IIK;
                let IIL;
                if IHQ != 0.0 {
                    let IIH = IHO.exp();
                    let III = IHP * IIH;
                    IIK = IIH;
                    IIL = III;
                } else {
                    let IIJ = if IHO < -8e1f64 { 1.0 } else { 0.0 };
                    let IJL;
                    let IJM;
                    if IIJ != 0.0 {
                        let IIX = IHP * AC;
                        let IIY = (-IHO) - SY;
                        let IIZ = GO * IIY;
                        let IJA = D + (IIY * WD);
                        let IJB = D + (IIZ * IJA);
                        let IJC = D + (IIY * IJB);
                        let IJD = YT / IJC;
                        let IJE = ((((IIX * IJB) + ((((IIX * GO) * IJA) + ((IIX * WD) * IIZ)) * IIY)) * IJD) * AC) / IJC;
                        IJL = IJD;
                        IJM = IJE;
                    } else {
                        let IJF = IHO - SY;
                        let IJG = GO * IJF;
                        let IJH = D + (IJF * WD);
                        let IJI = D + (IJG * IJH);
                        let IJJ = XB * (D + (IJF * IJI));
                        let IJK = ((IHP * IJI) + ((((IHP * GO) * IJH) + ((IHP * WD) * IJG)) * IJF)) * XB;
                        IJL = IJJ;
                        IJM = IJK;
                    }
                    IIK = IJL;
                    IIL = IJM;
                }
                let IIM = -staged[226];
                let IIN = IIM * QB;
                let IIO = IIN * GVQ;
                let IIP = (QE * IIM) * GVQ;
                let IIQ = IIO * IHA;
                let IIR = (Lanes([0.0, IIP[0], IIP[1], 0.0]) + (GVR * IIN)) * IHA;
                let IIS = (IIQ * IHL) * GO;
                let IIT = D + IIK;
                let IIU = IIS * IIT;
                let IIV = IIL * IIS;
                let IIW = (((((Lanes([IIR[0], IIR[1], IIR[2], 0.0, IIR[3]]) + (IHB * IIO)) * IHL) + (IHM * IIQ)) * GO) * IIT) + Lanes([0.0, IIV[0], IIV[1], 0.0, 0.0]);
                IHF = IIU;
                IHG = IIW;
            } else {
                IHF = B;
                IHG = AFD;
            }
            let ILG;
            let ILH;
            if IHH != 0.0 {
                let IJO = RG * IJN;
                let IJP = RF * IJN;
                let IJQ = GO * ((QT * IJN) - IJO);
                let IJR = ((QX * IJN) - IJP) * GO;
                let IJS = (QV * IJN) - Lanes([IJR[0], IJR[1], 0.0]);
                let IJT = (RT * IJN) - Lanes([IJR[0], IJR[1], 0.0]);
                let IJV = (D + (IJO / IJU)).sqrt();
                let IJX = IJW * (IJV - D);
                let IJY = ((IJP / IJU) * (GY / (GX * IJV))) * IJW;
                let IKA = IJY * IJZ;
                let IKC = IJY * IKB;
                let IKE = ((((((QR - staged[228]) * IJN) - IJQ) - CN) + (IJZ * IJX)) * IKD) + IJQ;
                let IKF = (((Lanes([0.0, IJS[0], IJS[1], IJS[2]]) - RQ) + Lanes([0.0, IKA[0], IKA[1], 0.0])) * IKD) + Lanes([0.0, IJR[0], IJR[1], 0.0]);
                let IKH = ((((((RS - staged[229]) * IJN) - IJQ) - CN) + (IKB * IJX)) * IKG) + IJQ;
                let IKI = (((Lanes([0.0, IJT[0], IJT[1], IJT[2]]) - SA) + Lanes([0.0, IKC[0], IKC[1], 0.0])) * IKG) + Lanes([0.0, IJR[0], IJR[1], 0.0]);
                let IKJ = Lanes([IKF[0], IKF[1], IKF[2], 0.0, IKF[3]]);
                let IKK = Lanes([IKI[0], IKI[1], IKI[2], IKI[3], 0.0]);
                let IKM = IKH + (IKL * (IKE - IKH));
                let IKN = IKK + ((IKJ - IKK) * IKL);
                let IKO = IKM - AID;
                let IKP = IKN * IKO;
                let IKQ = ((IKO * IKO) + NI).sqrt();
                let IKR = GO * ((IKM + AID) - IKQ);
                let IKS = (IKN - ((IKP + IKP) * (GY / (GX * IKQ)))) * GO;
                let IKU = IKE + (IKT * (IKH - IKE));
                let IKV = IKJ + ((IKK - IKJ) * IKT);
                let IKW = IKU - AID;
                let IKX = IKV * IKW;
                let IKY = ((IKW * IKW) + NI).sqrt();
                let IKZ = GO * ((IKU + AID) - IKY);
                let ILA = staged[239] / AJE;
                let ILB = ((AJG * ILA) * AC) / AJE;
                let ILD = ILC * (IKR - IKZ);
                let ILE = (IKS - ((IKV - ((IKX + IKX) * (GY / (GX * IKY)))) * GO)) * ILC;
                let ILF = if ((IKZ - IKR).abs()) <= CVS { 1.0 } else { 0.0 };
                let ILX;
                let ILY;
                if ILF != 0.0 {
                    let ILL = ((ILK + (((GO * ILJ) * ILC) * ILJ)) - (((GO * ILK) * ILC) * ILK)) - (GO / ILC);
                    let ILM = GO * (((D - (ILC * ILJ)) - (ILC * ILK)) - (ILL * ILD));
                    let ILN = (ILM * ILA) / ILC;
                    let ILO = (((((ILE * ILL) * AC) * GO) * ILA) + (ILB * ILM)) / ILC;
                    ILX = ILN;
                    ILY = ILO;
                } else {
                    let ILP = -ILJ;
                    let ILQ = (ILP * ILD).exp();
                    let ILR = ILK - (D / ILC);
                    let ILS = (ILR * ILD).exp();
                    let ILT = ILQ - ILS;
                    let ILU = LA * ILD;
                    let ILV = (ILA * ILT) / ILU;
                    let ILW = (((ILB * ILT) + ((((ILE * ILP) * ILQ) - ((ILE * ILR) * ILS)) * ILA)) - ((ILE * LA) * ILV)) / ILU;
                    ILX = ILV;
                    ILY = ILW;
                }
                let ILZ = if IKR < SY { 1.0 } else { 0.0 };
                let IML;
                let IMM;
                if ILZ != 0.0 {
                    let IMA = IKR.exp();
                    let IMB = D + (ILX * IMA);
                    let IMC = IMB.ln();
                    let IMD = ((ILY * IMA) + ((IKS * IMA) * ILX)) * (GY / IMB);
                    let IME = D + IMC;
                    let IMF = LA + IMC;
                    let IMG = (IME.ln()) / IMF;
                    let IMH = D - IMG;
                    let IMI = IMC * IMH;
                    let IMJ = (IMD * IMH) + (((((IMD * (GY / IME)) - (IMD * IMG)) / IMF) * AC) * IMC);
                    IML = IMI;
                    IMM = IMJ;
                } else {
                    let IMK = if IKR < B { 1.0 } else { 0.0 };
                    let IMZ;
                    let INA;
                    if IMK != 0.0 {
                        let IMQ = if IKR > -8e1f64 { 1.0 } else { 0.0 };
                        let INL;
                        let INM;
                        if IMQ != 0.0 {
                            let INB = IKR.exp();
                            let INC = IKS * INB;
                            INL = INB;
                            INM = INC;
                        } else {
                            let IND = IKS * AC;
                            let INE = (-IKR) - SY;
                            let INF = GO * INE;
                            let ING = D + (INE * WD);
                            let INH = D + (INF * ING);
                            let INI = D + (INE * INH);
                            let INJ = YT / INI;
                            let INK = ((((IND * INH) + ((((IND * GO) * ING) + ((IND * WD) * INF)) * INE)) * INJ) * AC) / INI;
                            INL = INJ;
                            INM = INK;
                        }
                        let INN = ILX * INL;
                        let INO = (ILY * INL) + (INM * ILX);
                        IMZ = INN;
                        INA = INO;
                    } else {
                        let IMR = (ILX.ln()) + IKR;
                        let IMS = (ILY * (GY / ILX)) + IKS;
                        let IMT = D + IMR;
                        let IMU = LA + IMR;
                        let IMV = (IMT.ln()) / IMU;
                        let IMW = D - IMV;
                        let IMX = IMR * IMW;
                        let IMY = (IMS * IMW) + (((((IMS * (GY / IMT)) - (IMS * IMV)) / IMU) * AC) * IMR);
                        IMZ = IMX;
                        INA = IMY;
                    }
                    IML = IMZ;
                    IMM = INA;
                }
                let IMN = IKR - DGH;
                let IMO = IKS - DGJ;
                let IMP = if IMN < SY { 1.0 } else { 0.0 };
                let IOA;
                let IOB;
                if IMP != 0.0 {
                    let INP = IMN.exp();
                    let INQ = D + (ILX * INP);
                    let INR = INQ.ln();
                    let INS = ((ILY * INP) + ((IMO * INP) * ILX)) * (GY / INQ);
                    let INT = D + INR;
                    let INU = LA + INR;
                    let INV = (INT.ln()) / INU;
                    let INW = D - INV;
                    let INX = INR * INW;
                    let INY = (INS * INW) + (((((INS * (GY / INT)) - (INS * INV)) / INU) * AC) * INR);
                    IOA = INX;
                    IOB = INY;
                } else {
                    let INZ = if IMN < B { 1.0 } else { 0.0 };
                    let IOQ;
                    let IOR;
                    if INZ != 0.0 {
                        let IOH = if IMN > -8e1f64 { 1.0 } else { 0.0 };
                        let IPC;
                        let IPD;
                        if IOH != 0.0 {
                            let IOS = IMN.exp();
                            let IOT = IMO * IOS;
                            IPC = IOS;
                            IPD = IOT;
                        } else {
                            let IOU = IMO * AC;
                            let IOV = (-IMN) - SY;
                            let IOW = GO * IOV;
                            let IOX = D + (IOV * WD);
                            let IOY = D + (IOW * IOX);
                            let IOZ = D + (IOV * IOY);
                            let IPA = YT / IOZ;
                            let IPB = ((((IOU * IOY) + ((((IOU * GO) * IOX) + ((IOU * WD) * IOW)) * IOV)) * IPA) * AC) / IOZ;
                            IPC = IPA;
                            IPD = IPB;
                        }
                        let IPE = ILX * IPC;
                        let IPF = (ILY * IPC) + (IPD * ILX);
                        IOQ = IPE;
                        IOR = IPF;
                    } else {
                        let IOI = (ILX.ln()) + IMN;
                        let IOJ = (ILY * (GY / ILX)) + IMO;
                        let IOK = D + IOI;
                        let IOL = LA + IOI;
                        let IOM = (IOK.ln()) / IOL;
                        let ION = D - IOM;
                        let IOO = IOI * ION;
                        let IOP = (IOJ * ION) + (((((IOJ * (GY / IOK)) - (IOJ * IOM)) / IOL) * AC) * IOI);
                        IOQ = IOO;
                        IOR = IOP;
                    }
                    IOA = IOQ;
                    IOB = IOR;
                }
                let IOC = (GO * (IML + IOA)) + D;
                let IOD = IML - IOA;
                let IOF = (IOE * (IOC * IOD)) / FPF;
                let IOG = ((((((IMM + IOB) * GO) * IOD) + ((IMM - IOB) * IOC)) * IOE) - (FPG * IOF)) / FPF;
                ILG = IOF;
                ILH = IOG;
            } else {
                ILG = B;
                ILH = AFD;
            }
            let IPK;
            let IPL;
            if ILI != 0.0 {
                let IPH = (RA - (IPG * DGH)) / CH;
                let IPI = ((DFR - (DGJ * IPG)) - Lanes([(EF * IPH), 0.0, 0.0, 0.0, 0.0])) / CH;
                let IPJ = if IPH > B { 1.0 } else { 0.0 };
                let IPR;
                let IPS;
                if IPJ != 0.0 {
                    let IPN = IPH + FWG;
                    let IPO = (IPM * DT) / IPN;
                    let IPP = (Lanes([(FR * IPM), 0.0, 0.0, 0.0, 0.0]) - (IPI * IPO)) / IPN;
                    let IPQ = if (IPO.abs()) < SY { 1.0 } else { 0.0 };
                    let IPW;
                    let IPX;
                    if IPQ != 0.0 {
                        let IPT = IPO.exp();
                        let IPU = IPP * IPT;
                        IPW = IPT;
                        IPX = IPU;
                    } else {
                        let IPV = if IPO < -8e1f64 { 1.0 } else { 0.0 };
                        let IQS;
                        let IQT;
                        if IPV != 0.0 {
                            let IQE = IPP * AC;
                            let IQF = (-IPO) - SY;
                            let IQG = GO * IQF;
                            let IQH = D + (IQF * WD);
                            let IQI = D + (IQG * IQH);
                            let IQJ = D + (IQF * IQI);
                            let IQK = YT / IQJ;
                            let IQL = ((((IQE * IQI) + ((((IQE * GO) * IQH) + ((IQE * WD) * IQG)) * IQF)) * IQK) * AC) / IQJ;
                            IQS = IQK;
                            IQT = IQL;
                        } else {
                            let IQM = IPO - SY;
                            let IQN = GO * IQM;
                            let IQO = D + (IQM * WD);
                            let IQP = D + (IQN * IQO);
                            let IQQ = XB * (D + (IQM * IQP));
                            let IQR = ((IPP * IQP) + ((((IPP * GO) * IQO) + ((IPP * WD) * IQN)) * IQM)) * XB;
                            IQS = IQQ;
                            IQT = IQR;
                        }
                        IPW = IQS;
                        IPX = IQT;
                    }
                    let IPZ = IPY * IPH;
                    let IQA = IPZ * IPW;
                    let IQB = FZY + ILG;
                    let IQC = IQA * IQB;
                    let IQD = ((((IPI * IPY) * IPW) + (IPX * IPZ)) * IQB) + ((FZZ + ILH) * IQA);
                    IPR = IQC;
                    IPS = IQD;
                } else {
                    IPR = B;
                    IPS = AFD;
                }
                IPK = IPR;
                IPL = IPS;
            } else {
                IPK = B;
                IPL = AFD;
            }
            let IRD;
            let IRE;
            let IRF;
            let IRG;
            if S != 0.0 {
                let IQU = FZY + ILG;
                let IQV = IQU * QT;
                let IQW = QX * IQU;
                let IQX = IQV.abs();
                let IQY = IQX * DU;
                let IQZ = (((((FZZ + ILH) * QT) + Lanes([0.0, IQW[0], IQW[1], 0.0, 0.0])) * ((GX * (if IQV >= APH { 1.0 } else { 0.0 })) - GY)) * DU) + Lanes([(FS * IQX), 0.0, 0.0, 0.0, 0.0]);
                let IRA = if IQY > staged[246] { 1.0 } else { 0.0 };
                let IRR;
                let IRS;
                if IRA != 0.0 {
                    let IRJ = (-(IRI + (YY / IRI))) / DU;
                    let IRK = Lanes([(((FS * IRJ) * AC) / DU), 0.0, 0.0, 0.0, 0.0]);
                    IRR = IRJ;
                    IRS = IRK;
                } else {
                    let IRL = IQY - IRI;
                    let IRM = IQZ * IRL;
                    let IRN = ((IRL * IRL) + D).sqrt();
                    let IRP = (-((IRO * ((IQY + IRI) - IRN)) + (YY / IRI))) / DU;
                    let IRQ = ((((IQZ - ((IRM + IRM) * (GY / (GX * IRN)))) * IRO) * AC) - Lanes([(FS * IRP), 0.0, 0.0, 0.0, 0.0])) / DU;
                    IRR = IRP;
                    IRS = IRQ;
                }
                let IRT = DV / DU;
                let IRU = (FT - (FS * IRT)) / DU;
                IRD = IRR;
                IRE = IRT;
                IRF = IRS;
                IRG = IRU;
            } else {
                let IRB = FTT * T;
                let IRC = W * FTT;
                IRD = B;
                IRE = IRB;
                IRF = AFD;
                IRG = IRC;
            }
            let ISD;
            let ISE;
            let ISF;
            let ISG;
            let ISH;
            let ISI;
            let ISJ;
            let ISK;
            let ISL;
            let ISM;
            let ISN;
            let ISO;
            let ISP;
            let ISQ;
            let ISR;
            let ISS;
            let IST;
            let ISU;
            let ISV;
            let ISW;
            let ISX;
            let ISY;
            let ISZ;
            let ITA;
            let ITB;
            let ITC;
            let ITD;
            let ITE;
            let ITF;
            let ITG;
            let ITH;
            let ITI;
            let ITJ;
            let ITK;
            let ITL;
            let ITM;
            let ITN;
            let ITO;
            let ITP;
            let ITQ;
            let ITR;
            let ITS;
            let ITT;
            let ITU;
            let ITV;
            let ITW;
            let ITX;
            let ITY;
            let ITZ;
            let IUA;
            let IUB;
            let IUC;
            let IUD;
            let IUE;
            let IUF;
            let IUG;
            let IUH;
            let IUI;
            let IUJ;
            let IUK;
            let IUL;
            let IUM;
            let IUN;
            let IUO;
            if IRH != 0.0 {
                let IRV = QR - DW;
                let IRW = ((IRV * CH) - RK) - CN;
                let IRX = ((((RN - Lanes([FU, 0.0, 0.0, 0.0])) * CH) + Lanes([(EF * IRV), 0.0, 0.0, 0.0])) - RO) - RQ;
                let IRY = RS - DX;
                let IRZ = (IRY * CH) - RK;
                let ISA = (((RV - Lanes([FV, 0.0, 0.0, 0.0])) * CH) + Lanes([(EF * IRY), 0.0, 0.0, 0.0])) - RX;
                let ISB = IRZ - CN;
                let ISC = ISA - SA;
                let IUY;
                let IUZ;
                if J != 0.0 {
                    let IVF = CO / HT;
                    let IVG = EM / HT;
                    let IVH = Lanes([IRX[0], IRX[1], IRX[2], 0.0, IRX[3]]);
                    let IVI = staged[247] / IVF;
                    let IVJ = (IVI.ln()) + SQ;
                    let IVK = staged[248] / IVF;
                    let IVL = (((IVG * IVK) * AC) / IVF) * (GY / IVK);
                    let IVM = (IVK.ln()) + SQ;
                    let IVN = Lanes([((((IVG * IVI) * AC) / IVF) * (GY / IVI)), 0.0, 0.0, 0.0, 0.0]);
                    let IVO = (IVJ - (IRW - ((SI * (IRW - ISB)) * SO))) / SQ;
                    let IVP = (IVN - (IVH - (((IVH - Lanes([ISC[0], ISC[1], ISC[2], ISC[3], 0.0])) * SI) * SO))) / SQ;
                    let IVQ = if IVO < SY { 1.0 } else { 0.0 };
                    let IVV;
                    let IVW;
                    if IVQ != 0.0 {
                        let IVR = IVO.exp();
                        let IVS = D + IVR;
                        let IVT = IVS.ln();
                        let IVU = (IVP * IVR) * (GY / IVS);
                        IVV = IVT;
                        IVW = IVU;
                    } else {
                        IVV = IVO;
                        IVW = IVP;
                    }
                    let IVX = ISC * TG;
                    let IVZ = Lanes([IVL, 0.0, 0.0, 0.0, 0.0]);
                    let IWA = (IVM - (((TG * ISB) + (IVJ - (SQ * IVV))) * IVY)) / SQ;
                    let IWB = (IVZ - ((Lanes([IVX[0], IVX[1], IVX[2], IVX[3], 0.0]) + (IVN - (IVW * SQ))) * IVY)) / SQ;
                    let IWC = if IWA < SY { 1.0 } else { 0.0 };
                    let IWH;
                    let IWI;
                    if IWC != 0.0 {
                        let IWD = IWA.exp();
                        let IWE = D + IWD;
                        let IWF = IWE.ln();
                        let IWG = (IWB * IWD) * (GY / IWE);
                        IWH = IWF;
                        IWI = IWG;
                    } else {
                        IWH = IWA;
                        IWI = IWB;
                    }
                    let IWK = IWJ * ISB;
                    let IWL = ISC * IWJ;
                    let IWM = (IWJ * (IVM - (SQ * IWH))) - IWK;
                    let IWN = Lanes([IWL[0], IWL[1], IWL[2], IWL[3], 0.0]);
                    let IWO = ((IVZ - (IWI * SQ)) * IWJ) - IWN;
                    let IWP = if (IWM.abs()) <= TZ { 1.0 } else { 0.0 };
                    let IWY;
                    let IWZ;
                    if IWP != 0.0 {
                        let IWQ = ((UB * UB) * UC) / UD;
                        let IWR = IWM * UB;
                        let IWT = D - IWS;
                        let IWU = D + (((IWM * IWT) * UI) * IWQ);
                        let IWV = IWR * IWU;
                        let IWW = ((IWO * UB) * IWU) + ((((IWO * IWT) * UI) * IWQ) * IWR);
                        IWY = IWV;
                        IWZ = IWW;
                    } else {
                        let IWX = if IWM < (-TZ) { 1.0 } else { 0.0 };
                        let IYV;
                        let IYW;
                        if IWX != 0.0 {
                            let IXC = -IWM;
                            let IXD = IWO * AC;
                            let IXE = UT * (IXC * UB);
                            let IXF = (IXD * UB) * UT;
                            let IXG = IXE - UX;
                            let IXH = IXF * IXG;
                            let IXI = ((IXG * IXG) + VA).sqrt();
                            let IXJ = GO * ((IXE + UW) - IXI);
                            let IXK = (IXF - ((IXH + IXH) * (GY / (GX * IXI)))) * GO;
                            let IXL = IXC - IXJ;
                            let IXM = IXD - IXK;
                            let IXN = IXM * IXL;
                            let IXO = (IXL * IXL) + (VH * (IXJ + D));
                            let IXP = (IXN + IXN) + (IXK * VH);
                            let IXQ = IXM * LA;
                            let IXR = (LA * IXL) - VH;
                            let IXS = IXO * VM;
                            let IXT = (-IXJ) + (IXS.ln());
                            let IXU = (IXK * AC) + ((IXP * VM) * (GY / IXS));
                            let IXV = IXO + IXR;
                            let IXW = IXP + IXQ;
                            let IXX = IXW * IXV;
                            let IXY = GO * IXR;
                            let IXZ = (IXY * IXR) - IXO;
                            let IYA = (IXV * IXV) + (IXT * IXZ);
                            let IYB = (IXX + IXX) + ((IXU * IXZ) + (((((IXQ * GO) * IXR) + (IXQ * IXY)) - IXP) * IXT));
                            let IYC = IXO * IXV;
                            let IYD = IXV / IYA;
                            let IYE = IYD * IXT;
                            let IYF = IYE * IXT;
                            let IYG = IYF * IXR;
                            let IYH = IXQ * IXR;
                            let IYI = ((IXR * IXR) * WD) - IXO;
                            let IYJ = IYA + (IYG * IYI);
                            let IYK = (IYC * IXT) / IYJ;
                            let IYL = IXJ + IYK;
                            let IYM = IXK + ((((((IXP * IXV) + (IXW * IXO)) * IXT) + (IXU * IYC)) - ((IYB + ((((((((((IXW - (IYB * IYD)) / IYA) * IXT) + (IXU * IYD)) * IXT) + (IXU * IYE)) * IXR) + (IXQ * IYF)) * IYI) + ((((IYH + IYH) * WD) - IXP) * IYG))) * IYK)) / IYJ);
                            let IYN = if IYL < SY { 1.0 } else { 0.0 };
                            let IZF;
                            let IZG;
                            if IYN != 0.0 {
                                let IYX = IYL.exp();
                                let IYY = IYM * IYX;
                                IZF = IYX;
                                IZG = IYY;
                            } else {
                                let IYZ = IYL - SY;
                                let IZA = GO * IYZ;
                                let IZB = D + (IYZ * WD);
                                let IZC = D + (IZA * IZB);
                                let IZD = XB * (D + (IYZ * IZC));
                                let IZE = ((IYM * IZC) + ((((IYM * GO) * IZB) + ((IYM * WD) * IZA)) * IYZ)) * XB;
                                IZF = IZD;
                                IZG = IZE;
                            }
                            let IZH = D / IZF;
                            let IZI = IYL * IYL;
                            let IZJ = IYM * IYL;
                            let IZK = LA + IZI;
                            let IZL = D / IZK;
                            let IZM = (IZJ + IZJ) * IZL;
                            let IZN = (IZM * AC) / IZK;
                            let IZO = IZI * IZL;
                            let IZP = IZM + (IZN * IZI);
                            let IZQ = IYL * IZL;
                            let IZR = (XR * IZL) - (XS * IZO);
                            let IZS = IZR * IZL;
                            let IZT = IXC - IYL;
                            let IZU = IXD - IYM;
                            let IZV = IWS * IZH;
                            let IZW = (((IZG * IZH) * AC) / IZF) * IWS;
                            let IZX = (LA * IZT) + (VH * (((IZF - D) - IZV) + (IWS * (D - (XQ * (IZQ * IZL))))));
                            let IZY = (IZU * LA) + (((IZG - IZW) + (((((((IYM * IZL) + (IZN * IYL)) * IZL) + (IZN * IZQ)) * XQ) * AC) * IWS)) * VH);
                            let IZZ = IZU * IZT;
                            let JAA = (IZT * IZT) - (VH * ((((IZF - IYL) - D) + IZV) + (IWS * ((IYL - D) - IZO))));
                            let JAB = (IZZ + IZZ) - ((((IZG - IYM) + IZW) + ((IYM - IZP) * IWS)) * VH);
                            let JAC = LA - (VH * ((IZF + IZV) - (IWS * (IZS * IZL))));
                            let JAD = IZY * IZX;
                            let JAE = ((IZX * IZX) - (LA * (JAA * JAC))).sqrt();
                            let JAF = IZX + JAE;
                            let JAG = JAA / JAF;
                            let JAH = (-IYL) - (LA * JAG);
                            let JAI = (IYM * AC) - (((JAB - ((IZY + (((JAD + JAD) - (((JAB * JAC) + (((((IZG + IZW) - (((((((IZN * XR) - (IZP * XS)) * IZL) + (IZN * IZR)) * IZL) + (IZN * IZS)) * IWS)) * VH) * AC) * JAA)) * LA)) * (GY / (GX * JAE)))) * JAG)) / JAF) * LA);
                            IYV = JAH;
                            IYW = JAI;
                        } else {
                            let IYO = D / (UT + (UI * WK));
                            let IYP = (((UT * WM) * IYO) - D) * IYO;
                            let IYQ = IWM * UB;
                            let IYR = D + (IYP * IWM);
                            let IYS = -(IYQ * IYR);
                            let IYT = (((IWO * UB) * IYR) + ((IWO * IYP) * IYQ)) * AC;
                            let IYU = if IYS > -8e1f64 { 1.0 } else { 0.0 };
                            let JAT;
                            let JAU;
                            if IYU != 0.0 {
                                let JAJ = IYS.exp();
                                let JAK = IYT * JAJ;
                                JAT = JAJ;
                                JAU = JAK;
                            } else {
                                let JAL = IYT * AC;
                                let JAM = (-IYS) - SY;
                                let JAN = GO * JAM;
                                let JAO = D + (JAM * WD);
                                let JAP = D + (JAN * JAO);
                                let JAQ = D + (JAM * JAP);
                                let JAR = YT / JAQ;
                                let JAS = ((((JAL * JAP) + ((((JAL * GO) * JAO) + ((JAL * WD) * JAN)) * JAM)) * JAR) * AC) / JAQ;
                                JAT = JAR;
                                JAU = JAS;
                            }
                            let JAV = ((IWM + (VH * YY)) - (D - JAT)).sqrt();
                            let JAW = (IWM + (VH * GO)) - (UI * JAV);
                            let JAX = IWO - (((IWO - (JAU * AC)) * (GY / (GX * JAV))) * UI);
                            let JAY = ZC + ZD;
                            let JAZ = JAW - JAY;
                            let JBA = JAX * JAZ;
                            let JBB = ((JAZ * JAZ) + ZH).sqrt();
                            let JBC = (JAX - ((JBA + JBA) * (GY / (GX * JBB)))) * GO;
                            let JBD = (GO * ((JAW + JAY) - JBB)) - (GO * (JAY - (((JAY * JAY) + ZH).sqrt())));
                            let JBE = IWM - JBD;
                            let JBF = IWO - JBC;
                            let JBG = JBC * AC;
                            let JBH = (-JBD).exp();
                            let JBI = JBG * JBH;
                            let JBJ = JBD * JBD;
                            let JBK = JBC * JBD;
                            let JBL = LA + JBJ;
                            let JBM = D / JBL;
                            let JBN = (JBK + JBK) * JBM;
                            let JBO = (JBN * AC) / JBL;
                            let JBP = JBJ * JBM;
                            let JBQ = JBN + (JBO * JBJ);
                            let JBR = JBD * JBM;
                            let JBS = (XR * JBM) - (XS * JBP);
                            let JBT = JBS * JBM;
                            let JBU = JBF * JBE;
                            let JBV = (JBE * JBE) - (VH * (((JBH + JBD) - D) - (IWS * ((JBD + D) + JBP))));
                            let JBW = if AAD >= JBV { AAD } else { JBV };
                            let JBX = ((JBU + JBU) - (((JBI + JBC) - ((JBC + JBQ) * IWS)) * VH)) * (GY - (if AAD >= JBV { 1.0 } else { 0.0 }));
                            let JBY = D - (GO * (VH * (JBH - (IWS * (JBT * JBM)))));
                            let JBZ = (LA * JBE) + (VH * ((D - JBH) - (IWS * (D + (XQ * (JBR * JBM))))));
                            let JCA = (JBF * LA) + (((JBI * AC) - ((((((JBC * JBM) + (JBO * JBD)) * JBM) + (JBO * JBR)) * XQ) * IWS)) * VH);
                            let JCB = JBW / VH;
                            let JCC = (ZC - JBD) + (JCB.ln());
                            let JCD = JBG + ((JBX / VH) * (GY / JCB));
                            let JCE = JBW + JBZ;
                            let JCF = JBX + JCA;
                            let JCG = JCF * JCE;
                            let JCH = GO * JBZ;
                            let JCI = JBW * JBY;
                            let JCJ = (JBX * JBY) + (((((JBI - (((((((JBO * XR) - (JBQ * XS)) * JBM) + (JBO * JBS)) * JBM) + (JBO * JBT)) * IWS)) * VH) * GO) * AC) * JBW);
                            let JCK = (JCH * JBZ) - JCI;
                            let JCL = (JCE * JCE) + (JCC * JCK);
                            let JCM = (JCG + JCG) + ((JCD * JCK) + (((((JCA * GO) * JBZ) + (JCA * JCH)) - JCJ) * JCC));
                            let JCN = JBW * JCE;
                            let JCO = JCE / JCL;
                            let JCP = JCO * JCC;
                            let JCQ = JCP * JCC;
                            let JCR = JCQ * JBZ;
                            let JCS = JCA * JBZ;
                            let JCT = ((JBZ * JBZ) * WD) - JCI;
                            let JCU = JCL + (JCR * JCT);
                            let JCV = (JCN * JCC) / JCU;
                            let JCW = JBD + JCV;
                            let JCX = JBC + ((((((JBX * JCE) + (JCF * JBW)) * JCC) + (JCD * JCN)) - ((JCM + ((((((((((JCF - (JCM * JCO)) / JCL) * JCC) + (JCD * JCO)) * JCC) + (JCD * JCP)) * JBZ) + (JCA * JCQ)) * JCT) + ((((JCS + JCS) * WD) - JCJ) * JCR))) * JCV)) / JCU);
                            let JCY = if JCW < SY { 1.0 } else { 0.0 };
                            let JDG;
                            let JDH;
                            let JDI;
                            let JDJ;
                            if JCY != 0.0 {
                                let JCZ = JCW.exp();
                                let JDA = JCX * JCZ;
                                let JDB = D / JCZ;
                                let JDC = ((JDA * JDB) * AC) / JCZ;
                                let JDD = IWS * JCZ;
                                let JDE = JDA * IWS;
                                JDG = JDB;
                                JDH = JDD;
                                JDI = JDC;
                                JDJ = JDE;
                            } else {
                                let JDF = if JCW > (ZC - SY) { 1.0 } else { 0.0 };
                                let JFC;
                                let JFD;
                                let JFE;
                                let JFF;
                                if JDF != 0.0 {
                                    let JEJ = (JCW - ZC).exp();
                                    let JEK = JCX * JEJ;
                                    let JEL = IWS / JEJ;
                                    let JEM = ((JEK * JEL) * AC) / JEJ;
                                    JFC = JEL;
                                    JFD = JEJ;
                                    JFE = JEM;
                                    JFF = JEK;
                                } else {
                                    let JEN = JCX * AC;
                                    let JEO = (ZC - JCW) - SY;
                                    let JEP = GO * JEO;
                                    let JEQ = D + (JEO * WD);
                                    let JER = D + (JEP * JEQ);
                                    let JES = D + (JEO * JER);
                                    let JET = YT / JES;
                                    let JEU = ((((JEN * JER) + ((((JEN * GO) * JEQ) + ((JEN * WD) * JEP)) * JEO)) * JET) * AC) / JES;
                                    let JEV = JCW - SY;
                                    let JEW = GO * JEV;
                                    let JEX = D + (JEV * WD);
                                    let JEY = D + (JEW * JEX);
                                    let JEZ = D + (JEV * JEY);
                                    let JFA = YT / JEZ;
                                    let JFB = ((((JCX * JEY) + ((((JCX * GO) * JEX) + ((JCX * WD) * JEW)) * JEV)) * JFA) * AC) / JEZ;
                                    JFC = JFA;
                                    JFD = JET;
                                    JFE = JFB;
                                    JFF = JEU;
                                }
                                JDG = JFC;
                                JDH = JFD;
                                JDI = JFE;
                                JDJ = JFF;
                            }
                            let JDK = JCW * JCW;
                            let JDL = JCX * JCW;
                            let JDM = LA + JDK;
                            let JDN = D / JDM;
                            let JDO = (JDL + JDL) * JDN;
                            let JDP = (JDO * AC) / JDM;
                            let JDQ = JDK * JDN;
                            let JDR = JDO + (JDP * JDK);
                            let JDS = JCW * JDN;
                            let JDT = (XR * JDN) - (XS * JDQ);
                            let JDU = JDT * JDN;
                            let JDV = IWM - JCW;
                            let JDW = IWO - JCX;
                            let JDX = (LA * JDV) + (VH * (((D - JDG) + JDH) - (IWS * (D + (XQ * (JDS * JDN))))));
                            let JDY = (JDW * LA) + ((((JDI * AC) + JDJ) - ((((((JCX * JDN) + (JDP * JCW)) * JDN) + (JDP * JDS)) * XQ) * IWS)) * VH);
                            let JDZ = JDW * JDV;
                            let JEA = (JDV * JDV) - (VH * ((((JDG + JCW) - D) + JDH) - (IWS * ((JCW + D) + JDQ))));
                            let JEB = (JDZ + JDZ) - ((((JDI + JCX) + JDJ) - ((JCX + JDR) * IWS)) * VH);
                            let JEC = LA - (VH * ((JDG + JDH) - (IWS * (JDU * JDN))));
                            let JED = JDY * JDX;
                            let JEE = ((JDX * JDX) - (LA * (JEA * JEC))).sqrt();
                            let JEF = JDX + JEE;
                            let JEG = JEA / JEF;
                            let JEH = JCW + (LA * JEG);
                            let JEI = JCX + (((JEB - ((JDY + (((JED + JED) - (((JEB * JEC) + (((((JDI + JDJ) - (((((((JDP * XR) - (JDR * XS)) * JDN) + (JDP * JDT)) * JDN) + (JDP * JDU)) * IWS)) * VH) * AC) * JEA)) * LA)) * (GY / (GX * JEE)))) * JEG)) / JEF) * LA);
                            IYV = JEH;
                            IYW = JEI;
                        }
                        IWY = IYV;
                        IWZ = IYW;
                    }
                    let IXA = IWJ * (IWY + IWK);
                    let IXB = (IWZ + IWN) * IWJ;
                    IUY = IXA;
                    IUZ = IXB;
                } else {
                    let IUX = Lanes([ISC[0], ISC[1], ISC[2], ISC[3], 0.0]);
                    IUY = ISB;
                    IUZ = IUX;
                }
                let IVA = IRW - IUY;
                let IVB = Lanes([IRX[0], IRX[1], IRX[2], 0.0, IRX[3]]);
                let IVC = IVB - IUZ;
                let IVD = SI * IVA;
                let IVE = IVC * SI;
                let JGS;
                let JGT;
                let JGU;
                let JGV;
                let JGW;
                let JGX;
                let JGY;
                let JGZ;
                let JHA;
                let JHB;
                let JHC;
                let JHD;
                if L != 0.0 {
                    let JFG = IVD - ADO;
                    let JFH = IVE * JFG;
                    let JFJ = ((JFG * JFG) + JFI).sqrt();
                    let JFK = GO * ((IVD + ADO) + JFJ);
                    let JFL = -IVD;
                    let JFM = IVE * AC;
                    let JFN = JFL - ADO;
                    let JFO = JFM * JFN;
                    let JFP = ((JFN * JFN) + JFI).sqrt();
                    let JFQ = GO * ((JFL + ADO) + JFP);
                    let JFS = (JFR * (JFK.ln())).exp();
                    let JFT = CP * JFS;
                    let JFU = Lanes([(EN * JFS), 0.0, 0.0, 0.0, 0.0]) + ((((((IVE + ((JFH + JFH) * (GY / (GX * JFJ)))) * GO) * (GY / JFK)) * JFR) * JFS) * CP);
                    let JFW = (JFV * (JFQ.ln())).exp();
                    let JFX = CP * JFW;
                    let JFY = Lanes([(EN * JFW), 0.0, 0.0, 0.0, 0.0]) + ((((((JFM + ((JFO + JFO) * (GY / (GX * JFP)))) * GO) * (GY / JFQ)) * JFV) * JFW) * CP);
                    let JFZ = (D - JFT) - JFX;
                    let JGA = (JFU * AC) - JFY;
                    let JGB = AEK / JFZ;
                    let JGC = ((JGA * JGB) * AC) / JFZ;
                    let JGD = D + (AEN * JFT);
                    let JGE = D + (TG * JFX);
                    let JGF = (AEN * JFZ) / JGD;
                    let JGG = ((JGA * AEN) - ((JFU * AEN) * JGF)) / JGD;
                    let JGH = (TG * JFZ) / JGE;
                    let JGI = ((JGA * TG) - ((JFY * TG) * JGH)) / JGE;
                    let JGJ = D / JGF;
                    let JGK = D / JGH;
                    let JGL = (D + JGJ) + JGK;
                    let JGM = D / JGL;
                    let JGN = ((((((JGG * JGJ) * AC) / JGF) + (((JGI * JGK) * AC) / JGH)) * JGM) * AC) / JGL;
                    let JGO = (JGG * JFT) + (JFU * JGF);
                    let JGP = D + (JGF * JFT);
                    let JGQ = (JGI * JFX) + (JFY * JGH);
                    let JGR = D + (JGH * JFX);
                    JGS = JGM;
                    JGT = JGF;
                    JGU = JGH;
                    JGV = JGB;
                    JGW = JGP;
                    JGX = JGR;
                    JGY = JGN;
                    JGZ = JGG;
                    JHA = JGI;
                    JHB = JGC;
                    JHC = JGO;
                    JHD = JGQ;
                } else {
                    JGS = SI;
                    JGT = AEN;
                    JGU = TG;
                    JGV = AEK;
                    JGW = D;
                    JGX = D;
                    JGY = AFD;
                    JGZ = AFD;
                    JHA = AFD;
                    JHB = AFD;
                    JHC = AFD;
                    JHD = AFD;
                }
                let JHE = JGS * IVA;
                let JHF = (JGY * IVA) + (IVC * JGS);
                let JHG = if JHE > B { 1.0 } else { 0.0 };
                let JHL;
                let JHM;
                if JHG != 0.0 {
                    let JHH = -JHE;
                    let JHI = JHF * AC;
                    let JHJ = if JHH < SY { 1.0 } else { 0.0 };
                    let JKT;
                    let JKU;
                    if JHJ != 0.0 {
                        let JKP = JHH.exp();
                        let JKQ = D + JKP;
                        let JKR = JKQ.ln();
                        let JKS = (JHI * JKP) * (GY / JKQ);
                        JKT = JKR;
                        JKU = JKS;
                    } else {
                        JKT = JHH;
                        JKU = JHI;
                    }
                    let JKV = JHE / JGT;
                    let JKW = (IVB - ((JHF - (JGZ * JKV)) / JGT)) + JKU;
                    let JKX = ((IRW - JKV) + JKT) - HW;
                    JHL = JKX;
                    JHM = JKW;
                } else {
                    let JHK = if JHE < SY { 1.0 } else { 0.0 };
                    let JLC;
                    let JLD;
                    if JHK != 0.0 {
                        let JKY = JHE.exp();
                        let JKZ = D + JKY;
                        let JLA = JKZ.ln();
                        let JLB = (JHF * JKY) * (GY / JKZ);
                        JLC = JLA;
                        JLD = JLB;
                    } else {
                        JLC = JHE;
                        JLD = JHF;
                    }
                    let JLE = JHE / JGU;
                    let JLF = (IUZ + ((JHF - (JHA * JLE)) / JGU)) + JLD;
                    let JLG = ((IUY + JLE) + JLC) - HW;
                    JHL = JLG;
                    JHM = JLF;
                }
                let JHN = JHL - CQ;
                let JHO = (JHM - AFZ) * JHN;
                let JHP = ((JHN * JHN) + XQ).sqrt();
                let JHQ = GO * ((JHL + CQ) - JHP);
                let JHR = ((JHM + AFZ) - ((JHO + JHO) * (GY / (GX * JHP)))) * GO;
                let JHS = (LA * (CQ - JHQ)) / CR;
                let JHT = (D + JHS).sqrt();
                let JHU = ((((AFZ - JHR) * LA) - Lanes([(EP * JHS), 0.0, 0.0, 0.0, 0.0])) / CR) * (GY / (GX * JHT));
                let JHV = JHT - D;
                let JHW = JHQ + (CR * JHV);
                let JHX = JHR + (Lanes([(EP * JHV), 0.0, 0.0, 0.0, 0.0]) + (JHU * CR));
                let JHY = ISA * AGL;
                let JHZ = D + (AGL * IRZ);
                let JIA = JHZ - GO;
                let JIB = JHY * JIA;
                let JIC = ((JIA * JIA) + NI).sqrt();
                let JID = GO * ((JHZ + GO) + JIC);
                let JIE = (JHY + ((JIB + JIB) * (GY / (GX * JIC)))) * GO;
                let JIG = D + (JIF * JID);
                let JIH = D / JIG;
                let JII = (((JIE * JIF) * JIH) * AC) / JIG;
                let JIK = D + (JIJ * JID);
                let JIL = D / JIK;
                let JIM = (((JIE * JIJ) * JIL) * AC) / JIK;
                let JIN = D + (AHH * JHV);
                let JIO = AHF * JIN;
                let JIP = AHG * JIN;
                let JIQ = D + (AHL * IRZ);
                let JIR = JIO * JIQ;
                let JIS = (ISA * AHL) * JIO;
                let JIT = ((Lanes([JIP[0], JIP[1], JIP[2], 0.0, 0.0]) + ((JHU * AHH) * AHF)) * JIQ) + Lanes([JIS[0], JIS[1], JIS[2], JIS[3], 0.0]);
                let JIU = DY * JIR;
                let JIV = (IRW - JHW) + JIU;
                let JIW = JII * JIV;
                let JIX = ((JIV * JIH) + JHW) + RK;
                let JIY = (((((IVB - JHX) + (Lanes([(FW * JIR), 0.0, 0.0, 0.0, 0.0]) + (JIT * DY))) * JIH) + Lanes([JIW[0], JIW[1], JIW[2], JIW[3], 0.0])) + JHX) + AHU;
                let JIZ = (IUY - JHW) + (DZ * JIR);
                let JJA = JIM * JIZ;
                let JJB = ((JIZ * JIL) + JHW) + RK;
                let JJC = (((((IUZ - JHX) + (Lanes([(FX * JIR), 0.0, 0.0, 0.0, 0.0]) + (JIT * DZ))) * JIL) + Lanes([JJA[0], JJA[1], JJA[2], JJA[3], 0.0])) + JHX) + AHU;
                let JJD = JJB + (AIA * (JIX - JJB));
                let JJE = JJC + ((JIY - JJC) * AIA);
                let JJF = JJD - AID;
                let JJG = JJE * JJF;
                let JJH = ((JJF * JJF) + NI).sqrt();
                let JJI = GO * ((JJD + AID) - JJH);
                let JJJ = (JJE - ((JJG + JJG) * (GY / (GX * JJH)))) * GO;
                let JJK = JIX + (AIJ * (JJB - JIX));
                let JJL = JIY + ((JJC - JIY) * AIJ);
                let JJM = JJK - AID;
                let JJN = JJL * JJM;
                let JJO = ((JJM * JJM) + NI).sqrt();
                let JJP = GO * ((JJK + AID) - JJO);
                let JJQ = (JJL - ((JJN + JJN) * (GY / (GX * JJO)))) * GO;
                let JJR = JGT / JIH;
                let JJS = JII * JJR;
                let JJT = (JGZ - Lanes([JJS[0], JJS[1], JJS[2], JJS[3], 0.0])) / JIH;
                let JJU = JGU / JIL;
                let JJV = JIM * JJU;
                let JJW = (JHA - Lanes([JJV[0], JJV[1], JJV[2], JJV[3], 0.0])) / JIL;
                let JJX = D / JJR;
                let JJY = ((JJT * JJX) * AC) / JJR;
                let JJZ = D / JJU;
                let JKA = ((JJW * JJZ) * AC) / JJU;
                let JKB = (D + JJX) + JJZ;
                let JKC = D / JKB;
                let JKD = (((JJY + JKA) * JKC) * AC) / JKB;
                let JKE = JGV * JGV;
                let JKF = JHB * JGV;
                let JKG = CO / JKE;
                let JKH = (AJI - ((JKF + JKF) * JKG)) / JKE;
                let JKI = D + JJR;
                let JKJ = D + JJU;
                let JKK = JKI / JKJ;
                let JKL = (JJT - (JJW * JKK)) / JKJ;
                let JKM = JKK.ln();
                let JKN = JKL * (GY / JKK);
                let JKO = if JKM > KS { 1.0 } else { 0.0 };
                let JLO;
                let JLP;
                if JKO != 0.0 {
                    let JLH = LA * JKM;
                    let JLI = JKK + D;
                    let JLJ = JKK - D;
                    let JLK = (JLH * JLI) / JLJ;
                    let JLL = ((((JKN * LA) * JLI) + (JKL * JLH)) - (JKL * JLK)) / JLJ;
                    JLO = JLK;
                    JLP = JLL;
                } else {
                    let JLM = LA * (LA + JKM);
                    let JLN = JKN * LA;
                    JLO = JLM;
                    JLP = JLN;
                }
                let JLQ = JJI - JJP;
                let JLR = JKC * JLQ;
                let JLS = (JKD * JLQ) + ((JJJ - JJQ) * JKC);
                let JLT = JLR * JLR;
                let JLU = JLS * JLR;
                let JLV = JLU + JLU;
                let JLW = JLR * JJX;
                let JLX = (JLS * JJX) + (JJY * JLR);
                let JLY = JJI - JLW;
                let JLZ = JJJ - JLX;
                let JMA = JLR * JJZ;
                let JMB = (JLS * JJZ) + (JKA * JLR);
                let JMC = JJP + JMA;
                let JMD = JJQ + JMB;
                let JME = D / JKI;
                let JMF = JJT * JME;
                let JMG = (JMF * AC) / JKI;
                let JMH = D / JKJ;
                let JMI = JJW * JMH;
                let JMJ = (JMI * AC) / JKJ;
                let JMK = JJR + (JJU * JMH);
                let JML = JJT + (JMI + (JMJ * JJU));
                let JMM = JLP * JMK;
                let JMN = (JMK * JLO) / JKG;
                let JMO = JKH * JMN;
                let JMP = JMN.ln();
                let JMQ = GY / JMN;
                let JMR = ((((JML * JLO) + JMM) - JMO) / JKG) * JMQ;
                let JMS = JMP + ZD;
                let JMT = JJU + (JJR * JME);
                let JMU = JJW + (JMF + (JMG * JJR));
                let JMV = JLP * JMT;
                let JMW = (JMT * JLO) / JKG;
                let JMX = JKH * JMW;
                let JMY = JMW.ln();
                let JMZ = GY / JMW;
                let JNA = ((((JMU * JLO) + JMV) - JMX) / JKG) * JMZ;
                let JNB = JMY + ZD;
                let JNC = (JMS - JLY) * WD;
                let JND = (JMR - JLZ) * WD;
                let JNE = if JNC < SY { 1.0 } else { 0.0 };
                let JNJ;
                let JNK;
                if JNE != 0.0 {
                    let JNF = JNC.exp();
                    let JNG = D + JNF;
                    let JNH = JNG.ln();
                    let JNI = (JND * JNF) * (GY / JNG);
                    JNJ = JNH;
                    JNK = JNI;
                } else {
                    JNJ = JNC;
                    JNK = JND;
                }
                let JNL = JMS - (ZD * JNJ);
                let JNM = JMR - (JNK * ZD);
                let JNN = (JNB - JMC) * WD;
                let JNO = (JNA - JMD) * WD;
                let JNP = if JNN < SY { 1.0 } else { 0.0 };
                let JNU;
                let JNV;
                if JNP != 0.0 {
                    let JNQ = JNN.exp();
                    let JNR = D + JNQ;
                    let JNS = JNR.ln();
                    let JNT = (JNO * JNQ) * (GY / JNR);
                    JNU = JNS;
                    JNV = JNT;
                } else {
                    JNU = JNN;
                    JNV = JNO;
                }
                let JNW = JJR * JJI;
                let JNX = (JJT * JJI) + (JJJ * JJR);
                let JNY = JNW + (JNB - (ZD * JNU));
                let JNZ = JJU * JJP;
                let JOA = (JJW * JJP) + (JJQ * JJU);
                let JOB = JNZ + JNL;
                let JOC = JOB * JMH;
                let JOD = ((JOA + JNM) * JMH) + (JMJ * JOB);
                let JOE = (JMS - (JNY * JME)) * WD;
                let JOF = (JMR - (((JNX + (JNA - (JNV * ZD))) * JME) + (JMG * JNY))) * WD;
                let JOG = if JOE < SY { 1.0 } else { 0.0 };
                let JOL;
                let JOM;
                if JOG != 0.0 {
                    let JOH = JOE.exp();
                    let JOI = D + JOH;
                    let JOJ = JOI.ln();
                    let JOK = (JOF * JOH) * (GY / JOI);
                    JOL = JOJ;
                    JOM = JOK;
                } else {
                    JOL = JOE;
                    JOM = JOF;
                }
                let JON = JMS - (ZD * JOL);
                let JOO = JMR - (JOM * ZD);
                let JOP = (JNB - JOC) * WD;
                let JOQ = (JNA - JOD) * WD;
                let JOR = if JOP < SY { 1.0 } else { 0.0 };
                let JOW;
                let JOX;
                if JOR != 0.0 {
                    let JOS = JOP.exp();
                    let JOT = D + JOS;
                    let JOU = JOT.ln();
                    let JOV = (JOQ * JOS) * (GY / JOT);
                    JOW = JOU;
                    JOX = JOV;
                } else {
                    JOW = JOP;
                    JOX = JOQ;
                }
                let JOY = JJI - JON;
                let JOZ = JJJ - JOO;
                let JPA = JJP - (JNB - (ZD * JOW));
                let JPB = JJQ - (JNA - (JOX * ZD));
                let JPC = JJR * JOY;
                let JPD = (JJT * JOY) + (JOZ * JJR);
                let JPE = JJI - JOY;
                let JPF = JJJ - JOZ;
                let JPG = if JPE < SY { 1.0 } else { 0.0 };
                let JPP;
                let JPQ;
                if JPG != 0.0 {
                    let JPH = JPE.exp();
                    let JPI = JPF * JPH;
                    JPP = JPH;
                    JPQ = JPI;
                } else {
                    let JPJ = JPE - SY;
                    let JPK = GO * JPJ;
                    let JPL = D + (JPJ * WD);
                    let JPM = D + (JPK * JPL);
                    let JPN = XB * (D + (JPJ * JPM));
                    let JPO = ((JPF * JPM) + ((((JPF * GO) * JPL) + ((JPF * WD) * JPK)) * JPJ)) * XB;
                    JPP = JPN;
                    JPQ = JPO;
                }
                let JPR = JKG * JPP;
                let JPS = (JKH * JPP) + (JPQ * JKG);
                let JPT = JPD * JPC;
                let JPU = (JPC * JPC) - JPR;
                let JPV = (JPT + JPT) - JPS;
                let JPW = LA * JJR;
                let JPX = JJT * LA;
                let JPY = (JPW * JPC) + JPR;
                let JPZ = ((JPX * JPC) + (JPD * JPW)) + JPS;
                let JQA = JPW * JJR;
                let JQB = (JPX * JJR) + (JJT * JPW);
                let JQC = JQA - JPR;
                let JQD = JQB - JPS;
                let JQE = if JPU < -5e-3f64 { 1.0 } else { 0.0 };
                let JRH;
                let JRI;
                let JRJ;
                let JRK;
                let JRL;
                let JRM;
                let JRN;
                let JRO;
                let JRP;
                let JRQ;
                let JRR;
                let JRS;
                let JRT;
                let JRU;
                if JQE != 0.0 {
                    let JQF = (JPU.abs()).sqrt();
                    let JQG = (JPV * ((GX * (if JPU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * JQF));
                    let JQH = GO * JQF;
                    let JQI = JQH.tan();
                    let JQJ = JQH.cos();
                    let JQK = JQF / JQI;
                    let JQL = (JQG - (((JQG * GO) * (GY / (JQJ * JQJ))) * JQK)) / JQI;
                    let JQM = (YY * JPY) / JPU;
                    let JQN = ((JPZ * YY) - (JPV * JQM)) / JPU;
                    let JQO = LA - JQK;
                    let JQP = JPU + (JQK * JQO);
                    let JQQ = JQP * JQM;
                    let JQR = ((JPV + ((JQL * JQO) + ((JQL * AC) * JQK))) * JQM) + (JQN * JQP);
                    let JQS = LA * JQQ;
                    let JQT = D + JQK;
                    let JQU = JPY - (JQS * JQT);
                    let JQV = (JQQ * JQC) / JPY;
                    let JQW = (JQU * JQM) + JQV;
                    let JQX = (((JPZ - (((JQR * LA) * JQT) + (JQL * JQS))) * JQM) + (JQN * JQU)) + ((((JQR * JQC) + (JQD * JQQ)) - (JPZ * JQV)) / JPY);
                    let JQY = D - (GO * JQK);
                    let JQZ = (JQL * GO) * AC;
                    let JRA = JPY / JPU;
                    let JRB = JRA * JQY;
                    let JRC = (((JPZ - (JPV * JRA)) / JPU) * JQY) + (JQZ * JRA);
                    let JRD = JRB + (GO * JQQ);
                    let JRE = ((JQC * JQY) - (JPY * JRD)) / JPU;
                    let JRF = ((((JQD * JQY) + (JQZ * JQC)) - ((JPZ * JRD) + ((JRC + (JQR * GO)) * JPY))) - (JPV * JRE)) / JPU;
                    JRH = B;
                    JRI = JQF;
                    JRJ = JQK;
                    JRK = JQQ;
                    JRL = JQW;
                    JRM = JRB;
                    JRN = JRE;
                    JRO = AFD;
                    JRP = JQG;
                    JRQ = JQL;
                    JRR = JQR;
                    JRS = JQX;
                    JRT = JRC;
                    JRU = JRF;
                } else {
                    let JRG = if JPU > AQJ { 1.0 } else { 0.0 };
                    let JUM;
                    let JUN;
                    let JUO;
                    let JUP;
                    let JUQ;
                    let JUR;
                    let JUS;
                    let JUT;
                    let JUU;
                    let JUV;
                    let JUW;
                    let JUX;
                    let JUY;
                    let JUZ;
                    if JRG != 0.0 {
                        let JRW = (JPU.abs()).sqrt();
                        let JRX = (JPV * ((GX * (if JPU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * JRW));
                        let JRY = (-JRW).exp();
                        let JRZ = (JRX * AC) * JRY;
                        let JSA = D + JRY;
                        let JSB = D - JRY;
                        let JSC = (JRW * JSA) / JSB;
                        let JSD = (((JRX * JSA) + (JRZ * JRW)) - ((JRZ * AC) * JSC)) / JSB;
                        let JSE = (YY * JPY) / JPU;
                        let JSF = ((JPZ * YY) - (JPV * JSE)) / JPU;
                        let JSG = LA - JSC;
                        let JSH = JPU + (JSC * JSG);
                        let JSI = JSH * JSE;
                        let JSJ = ((JPV + ((JSD * JSG) + ((JSD * AC) * JSC))) * JSE) + (JSF * JSH);
                        let JSK = LA * JSI;
                        let JSL = D + JSC;
                        let JSM = JPY - (JSK * JSL);
                        let JSN = (JSI * JQC) / JPY;
                        let JSO = (JSM * JSE) + JSN;
                        let JSP = (((JPZ - (((JSJ * LA) * JSL) + (JSD * JSK))) * JSE) + (JSF * JSM)) + ((((JSJ * JQC) + (JQD * JSI)) - (JPZ * JSN)) / JPY);
                        let JSQ = D - (GO * JSC);
                        let JSR = (JSD * GO) * AC;
                        let JSS = JPY / JPU;
                        let JST = JSS * JSQ;
                        let JSU = (((JPZ - (JPV * JSS)) / JPU) * JSQ) + (JSR * JSS);
                        let JSV = JST + (GO * JSI);
                        let JSW = ((JQC * JSQ) - (JPY * JSV)) / JPU;
                        let JSX = ((((JQD * JSQ) + (JSR * JQC)) - ((JPZ * JSV) + ((JSU + (JSJ * GO)) * JPY))) - (JPV * JSW)) / JPU;
                        JUM = JRY;
                        JUN = JRW;
                        JUO = JSC;
                        JUP = JSI;
                        JUQ = JSO;
                        JUR = JST;
                        JUS = JSW;
                        JUT = JRZ;
                        JUU = JRX;
                        JUV = JSD;
                        JUW = JSJ;
                        JUX = JSP;
                        JUY = JSU;
                        JUZ = JSX;
                    } else {
                        let JSY = JPU * ASC;
                        let JSZ = JPU * ASE;
                        let JTA = JPV * ASE;
                        let JTB = D - (JPU * ASH);
                        let JTC = D - (JSZ * JTB);
                        let JTD = UC * (D - (JSY * JTC));
                        let JTE = ((((JPV * ASC) * JTC) + ((((JTA * JTB) + (((JPV * ASH) * AC) * JSZ)) * AC) * JSY)) * AC) * UC;
                        let JTF = (JPV * JTD) + (JTE * JPU);
                        let JTG = LA + (JPU * JTD);
                        let JTH = JPU * ASO;
                        let JTI = JPV * ASO;
                        let JTJ = JPU * ASR;
                        let JTK = D - JTH;
                        let JTL = D - (JTJ * JTK);
                        let JTM = UC * (D - (JTH * JTL));
                        let JTN = (((JTI * JTL) + (((((JPV * ASR) * JTK) + ((JTI * AC) * JTJ)) * AC) * JTH)) * AC) * UC;
                        let JTO = JPY * JTM;
                        let JTP = (JPZ * JTM) + (JTN * JPY);
                        let JTQ = JPU * ASZ;
                        let JTR = ATB * JPU;
                        let JTS = D - (ATD * JPU);
                        let JTT = D - (JTR * JTS);
                        let JTU = ATG * (D - (JTQ * JTT));
                        let JTV = JPY * JPY;
                        let JTW = JPZ * JPY;
                        let JTX = (JQC * JTM) - (JTV * JTU);
                        let JTY = ((JQD * JTM) + (JTN * JQC)) - (((JTW + JTW) * JTU) + ((((((JPV * ASZ) * JTT) + (((((JPV * ATB) * JTS) + (((JPV * ATD) * AC) * JTR)) * AC) * JTQ)) * AC) * ATG) * JTV));
                        let JUA = JTZ * JPY;
                        let JUB = JUA * JTD;
                        let JUC = ((JPZ * JTZ) * JTD) + (JTE * JUA);
                        let JUE = JUD * JQC;
                        let JUG = JUF * JPY;
                        let JUH = JUG * JPY;
                        let JUI = LA - (ATV * JPU);
                        let JUJ = D - (JSZ * JUI);
                        let JUK = (JUE * JTD) + (JUH * JUJ);
                        let JUL = (((JQD * JUD) * JTD) + (JTE * JUE)) + (((((JPZ * JUF) * JPY) + (JPZ * JUG)) * JUJ) + ((((JTA * JUI) + (((JPV * ATV) * AC) * JSZ)) * AC) * JUH));
                        JUM = B;
                        JUN = B;
                        JUO = JTG;
                        JUP = JTO;
                        JUQ = JTX;
                        JUR = JUB;
                        JUS = JUK;
                        JUT = AFD;
                        JUU = AFD;
                        JUV = JTF;
                        JUW = JTP;
                        JUX = JTY;
                        JUY = JUC;
                        JUZ = JUL;
                    }
                    JRH = JUM;
                    JRI = JUN;
                    JRJ = JUO;
                    JRK = JUP;
                    JRL = JUQ;
                    JRM = JUR;
                    JRN = JUS;
                    JRO = JUT;
                    JRP = JUU;
                    JRQ = JUV;
                    JRR = JUW;
                    JRS = JUX;
                    JRT = JUY;
                    JRU = JUZ;
                }
                let JRV = if JPU > AQJ { 1.0 } else { 0.0 };
                let JVJ;
                let JVK;
                let JVL;
                let JVM;
                if JRV != 0.0 {
                    let JVA = LA - JRH;
                    let JVB = D - (JRH * JVA);
                    let JVC = (XQ * JPU) / JVB;
                    let JVD = ((JPV * XQ) - ((((JRO * JVA) + ((JRO * AC) * JRH)) * AC) * JVC)) / JVB;
                    let JVE = JVC * JRH;
                    let JVF = (JVD * JRH) + (JRO * JVC);
                    let JVG = (JVC.ln()) - JRI;
                    let JVH = (JVD * (GY / JVC)) - JRP;
                    JVJ = JVE;
                    JVK = JVG;
                    JVL = JVF;
                    JVM = JVH;
                } else {
                    let JVI = if JPU < -5e-3f64 { 1.0 } else { 0.0 };
                    let JWE;
                    let JWF;
                    let JWG;
                    let JWH;
                    if JVI != 0.0 {
                        let JVO = GO * JRI;
                        let JVP = JVO.sin();
                        let JVQ = JVP * JVP;
                        let JVR = ((JRP * GO) * (JVO.cos())) * JVP;
                        let JVS = (-JPU) / JVQ;
                        let JVT = ((JPV * AC) - ((JVR + JVR) * JVS)) / JVQ;
                        let JVU = JVS.ln();
                        let JVV = JVT * (GY / JVS);
                        JWE = JVS;
                        JWF = JVU;
                        JWG = JVT;
                        JWH = JVV;
                    } else {
                        let JVW = JPU * WD;
                        let JVX = ATB * JPU;
                        let JVY = D - (AVN * JPU);
                        let JVZ = D - (JVX * JVY);
                        let JWA = XQ - (JVW * JVZ);
                        let JWB = (((JPV * WD) * JVZ) + (((((JPV * ATB) * JVY) + (((JPV * AVN) * AC) * JVX)) * AC) * JVW)) * AC;
                        let JWC = JWA.ln();
                        let JWD = JWB * (GY / JWA);
                        JWE = JWA;
                        JWF = JWC;
                        JWG = JWB;
                        JWH = JWD;
                    }
                    JVJ = JWE;
                    JVK = JWF;
                    JVL = JWG;
                    JVM = JWH;
                }
                let JVN = if ((AVB * JPC) + JRJ) > B { 1.0 } else { 0.0 };
                let JXD;
                let JXE;
                let JXF;
                let JXG;
                let JXH;
                let JXI;
                if JVN != 0.0 {
                    let JWI = JPC + JRJ;
                    let JWJ = JPD + JRQ;
                    let JWK = JJR + JRK;
                    let JWL = JJT + JRR;
                    JXD = JWI;
                    JXE = JWK;
                    JXF = JRL;
                    JXG = JWJ;
                    JXH = JWL;
                    JXI = JRS;
                } else {
                    let JWM = JPC - JRJ;
                    let JWN = D / JWM;
                    let JWO = (((JPD - JRQ) * JWN) * AC) / JWM;
                    let JWP = JRK - JJR;
                    let JWQ = JRR - JJT;
                    let JWR = JPR - JVJ;
                    let JWS = JWR * JWN;
                    let JWT = ((JPS - JVL) * JWN) + (JWO * JWR);
                    let JWU = ((JWP * JWS) - JPR) - (JRM * JVJ);
                    let JWV = JWU * JWN;
                    let JWW = (((((JWQ * JWS) + (JWT * JWP)) - JPS) - ((JRT * JVJ) + (JVL * JRM))) * JWN) + (JWO * JWU);
                    let JWX = LA * JWP;
                    let JWY = JRT * JRM;
                    let JWZ = JRN + (JRM * JRM);
                    let JXA = (((JRL * JWS) + (JWX * JWV)) + JPR) - (JWZ * JVJ);
                    let JXB = JXA * JWN;
                    let JXC = ((((((JRS * JWS) + (JWT * JRL)) + (((JWQ * LA) * JWV) + (JWW * JWX))) + JPS) - (((JRU + (JWY + JWY)) * JVJ) + (JVL * JWZ))) * JWN) + (JWO * JXA);
                    JXD = JWS;
                    JXE = JWV;
                    JXF = JXB;
                    JXG = JWT;
                    JXH = JWW;
                    JXI = JXC;
                }
                let JXJ = if JXD > B { 1.0 } else { 0.0 };
                let JYD;
                let JYE;
                let JYF;
                let JYG;
                let JYH;
                let JYI;
                if JXJ != 0.0 {
                    let JXK = JXD.ln();
                    let JXL = JXG * (GY / JXD);
                    let JXM = D / JXD;
                    let JXN = ((JXG * JXM) * AC) / JXD;
                    let JXO = JXE * JXM;
                    let JXP = (JXH * JXM) + (JXN * JXE);
                    let JXQ = JXP * JXO;
                    let JXR = (JXF * JXM) - (JXO * JXO);
                    let JXS = ((JXI * JXM) + (JXN * JXF)) - (JXQ + JXQ);
                    JYD = JXK;
                    JYE = JXO;
                    JYF = JXR;
                    JYG = JXL;
                    JYH = JXP;
                    JYI = JXS;
                } else {
                    let JXT = -JPC;
                    let JXU = (JPC + HW) + (JXT.ln());
                    let JXV = JPD + ((JPD * AC) * (GY / JXT));
                    let JXW = D / JOY;
                    let JXX = ((JOZ * JXW) * AC) / JOY;
                    let JXY = JJR + JXW;
                    let JXZ = JJT + JXX;
                    let JYA = -JXW;
                    let JYB = JYA * JXW;
                    let JYC = ((JXX * AC) * JXW) + (JXX * JYA);
                    JYD = JXU;
                    JYE = JXY;
                    JYF = JYB;
                    JYG = JXV;
                    JYH = JXZ;
                    JYI = JYC;
                }
                let JYJ = JJP - JJI;
                let JYK = JJQ - JJJ;
                let JYL = ((JYJ + JOY) + (LA * JYD)) - JVK;
                let JYM = (D + (LA * JYE)) - JRM;
                let JYN = (LA * JYF) - JRN;
                let JYO = JPC + (JJU * JYL);
                let JYP = JPD + ((JJW * JYL) + ((((JYK + JOZ) + (JYG * LA)) - JVM) * JJU));
                let JYQ = JJR + (JJU * JYM);
                let JYR = JJT + ((JJW * JYM) + (((JYH * LA) - JRT) * JJU));
                let JYS = JJU * JYN;
                let JYT = (JYO * JXD) - JPR;
                let JYU = ((JYP * JXD) + (JXG * JYO)) - JPS;
                let JYV = ((JYQ * JXD) + (JYO * JXE)) + JPR;
                let JYW = (((JYR * JXD) + (JXG * JYQ)) + ((JYP * JXE) + (JXH * JYO))) + JPS;
                let JYX = LA * JYQ;
                let JYY = (((JYS * JXD) + (JYX * JXE)) + (JYO * JXF)) - JPR;
                let JYZ = JYW * JYV;
                let JZA = GO * JYT;
                let JZB = (JYV * JYV) - (JZA * JYY);
                let JZC = (JYZ + JYZ) - (((JYU * GO) * JYY) + ((((((((JJW * JYN) + (((JYI * LA) - JRU) * JJU)) * JXD) + (JXG * JYS)) + (((JYR * LA) * JXE) + (JXH * JYX))) + ((JYP * JXF) + (JXI * JYO))) - JPS) * JZA));
                let JZD = -JYT;
                let JZE = JZD * JYV;
                let JZF = JZC * JZB;
                let JZG = (JZB * JZB) + AYW;
                let JZH = (JZE * JZB) / JZG;
                let JZI = JOY + JZH;
                let JZJ = JOZ + (((((((JYU * AC) * JYV) + (JYW * JZD)) * JZB) + (JZC * JZE)) - ((JZF + JZF) * JZH)) / JZG);
                let JZK = JJR * JZI;
                let JZL = (JJT * JZI) + (JZJ * JJR);
                let JZM = JJU * JPA;
                let JZN = (JJW * JPA) + (JPB * JJU);
                let JZO = JZK + JZM;
                let JZP = JZL + JZN;
                let JZQ = JZP * AZH;
                let JZR = D + (AZH * JZO);
                let JZS = JZK * JZM;
                let JZT = (JZL * JZM) + (JZN * JZK);
                let JZU = (AZL + (AZK * JZO)) + JZS;
                let JZV = (JZP * AZK) + JZT;
                let JZW = AZL * ((LA * JZO) + JZS);
                let JZX = JZV * JZU;
                let JZY = XQ * JZR;
                let JZZ = ((JZU * JZU) - (JZY * JZW)).sqrt();
                let KAA = LA * JZR;
                let KAB = (JZZ - JZU) / KAA;
                let KAC = JZL * JZK;
                let KAD = (JZK * JZK) - KAB;
                let KAE = (KAC + KAC) - ((((((JZX + JZX) - (((JZQ * XQ) * JZW) + ((((JZP * LA) + JZT) * AZL) * JZY))) * (GY / (GX * JZZ))) - JZV) - ((JZQ * LA) * KAB)) / KAA);
                let KAF = if KAD > B { 1.0 } else { 0.0 };
                let KAO;
                let KAP;
                if KAF != 0.0 {
                    let KAG = KAD / JKG;
                    let KAH = ((KAG.ln()) - JJI) + JZI;
                    let KAI = KAD * KAH;
                    let KAJ = (KAE * KAH) + ((((((KAE - (JKH * KAG)) / JKG) * (GY / KAG)) - JJJ) + JZJ) * KAD);
                    let KAK = (JPW * JZK) + KAD;
                    let KAL = ((JPX * JZK) + (JZL * JPW)) + KAE;
                    let KAM = (JJI - JZI) - JMS;
                    let KAN = if (if (if (if KAI < B { 1.0 } else { 0.0 }) != 0.0 && (if KAK > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((KAM + BAH) + (JJR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if KAM > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let KBL;
                    let KBM;
                    if KAN != 0.0 {
                        let KBI = KAI / KAK;
                        let KBJ = JZI - KBI;
                        let KBK = JZJ - ((KAJ - (KAL * KBI)) / KAK);
                        KBL = KBJ;
                        KBM = KBK;
                    } else {
                        KBL = JZI;
                        KBM = JZJ;
                    }
                    KAO = KBL;
                    KAP = KBM;
                } else {
                    KAO = JZI;
                    KAP = JZJ;
                }
                let KAQ = JJR * KAO;
                let KAR = (JJT * KAO) + (KAP * JJR);
                let KAS = KAQ + JZM;
                let KAT = KAR + JZN;
                let KAU = KAT * AZH;
                let KAV = D + (AZH * KAS);
                let KAW = KAQ * JZM;
                let KAX = (KAR * JZM) + (JZN * KAQ);
                let KAY = (AZL + (AZK * KAS)) + KAW;
                let KAZ = (KAT * AZK) + KAX;
                let KBA = AZL * ((LA * KAS) + KAW);
                let KBB = KAZ * KAY;
                let KBC = XQ * KAV;
                let KBD = ((KAY * KAY) - (KBC * KBA)).sqrt();
                let KBE = LA * KAV;
                let KBF = (KBD - KAY) / KBE;
                let KBG = (((((KBB + KBB) - (((KAU * XQ) * KBA) + ((((KAT * LA) + KAX) * AZL) * KBC))) * (GY / (GX * KBD))) - KAZ) - ((KAU * LA) * KBF)) / KBE;
                let KBH = if KBF < -5e-3f64 { 1.0 } else { 0.0 };
                let KBY;
                let KBZ;
                let KCA;
                let KCB;
                let KCC;
                let KCD;
                let KCE;
                let KCF;
                if KBH != 0.0 {
                    let KBN = (KBF.abs()).sqrt();
                    let KBO = (KBG * ((GX * (if KBF >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * KBN));
                    let KBP = GO * KBN;
                    let KBQ = KBP.tan();
                    let KBR = KBP.cos();
                    let KBS = KBN / KBQ;
                    let KBT = (KBO - (((KBO * GO) * (GY / (KBR * KBR))) * KBS)) / KBQ;
                    let KBU = LA - KBS;
                    let KBV = (YY * (KBF + (KBS * KBU))) / KBF;
                    let KBW = (((KBG + ((KBT * KBU) + ((KBT * AC) * KBS))) * YY) - (KBG * KBV)) / KBF;
                    KBY = KBS;
                    KBZ = KBV;
                    KCA = JRH;
                    KCB = KBN;
                    KCC = KBT;
                    KCD = KBW;
                    KCE = JRO;
                    KCF = KBO;
                } else {
                    let KBX = if KBF > AQJ { 1.0 } else { 0.0 };
                    let KDK;
                    let KDL;
                    let KDM;
                    let KDN;
                    let KDO;
                    let KDP;
                    let KDQ;
                    let KDR;
                    if KBX != 0.0 {
                        let KCM = (KBF.abs()).sqrt();
                        let KCN = (KBG * ((GX * (if KBF >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * KCM));
                        let KCO = (-KCM).exp();
                        let KCP = (KCN * AC) * KCO;
                        let KCQ = D + KCO;
                        let KCR = D - KCO;
                        let KCS = (KCM * KCQ) / KCR;
                        let KCT = (((KCN * KCQ) + (KCP * KCM)) - ((KCP * AC) * KCS)) / KCR;
                        let KCU = LA - KCS;
                        let KCV = (YY * (KBF + (KCS * KCU))) / KBF;
                        let KCW = (((KBG + ((KCT * KCU) + ((KCT * AC) * KCS))) * YY) - (KBG * KCV)) / KBF;
                        KDK = KCS;
                        KDL = KCV;
                        KDM = KCO;
                        KDN = KCM;
                        KDO = KCT;
                        KDP = KCW;
                        KDQ = KCP;
                        KDR = KCN;
                    } else {
                        let KCX = KBF * UC;
                        let KCY = KBF * ASC;
                        let KCZ = D - (KBF * ASE);
                        let KDA = D - (KCY * KCZ);
                        let KDB = ((KBG * UC) * KDA) + (((((KBG * ASC) * KCZ) + (((KBG * ASE) * AC) * KCY)) * AC) * KCX);
                        let KDC = LA + (KCX * KDA);
                        let KDD = KBF * ASO;
                        let KDE = KBG * ASO;
                        let KDF = KBF * ASR;
                        let KDG = D - KDD;
                        let KDH = D - (KDF * KDG);
                        let KDI = UC * (D - (KDD * KDH));
                        let KDJ = (((KDE * KDH) + (((((KBG * ASR) * KDG) + ((KDE * AC) * KDF)) * AC) * KDD)) * AC) * UC;
                        KDK = KDC;
                        KDL = KDI;
                        KDM = JRH;
                        KDN = JRI;
                        KDO = KDB;
                        KDP = KDJ;
                        KDQ = JRO;
                        KDR = JRP;
                    }
                    KBY = KDK;
                    KBZ = KDL;
                    KCA = KDM;
                    KCB = KDN;
                    KCC = KDO;
                    KCD = KDP;
                    KCE = KDQ;
                    KCF = KDR;
                }
                let KCG = (KAS * KBZ) + D;
                let KCH = (((KAS * KBY) + KAW) + KBF) / KCG;
                let KCI = KAR * KAQ;
                let KCJ = (KAQ * KAQ) - (KBF - KCH);
                let KCK = (KCI + KCI) - (KBG - ((((((KAT * KBY) + (KCC * KAS)) + KAX) + KBG) - (((KAT * KBZ) + (KCD * KAS)) * KCH)) / KCG));
                let KCL = if KCJ > B { 1.0 } else { 0.0 };
                let KEA;
                let KEB;
                if KCL != 0.0 {
                    let KDS = KCJ / JKG;
                    let KDT = ((KDS.ln()) - JJI) + KAO;
                    let KDU = KCJ * KDT;
                    let KDV = (KCK * KDT) + ((((((KCK - (JKH * KDS)) / JKG) * (GY / KDS)) - JJJ) + KAP) * KCJ);
                    let KDW = (JPW * KAQ) + KCJ;
                    let KDX = ((JPX * KAQ) + (KAR * JPW)) + KCK;
                    let KDY = (JJI - KAO) - JMS;
                    let KDZ = if (if (if (if KDU < B { 1.0 } else { 0.0 }) != 0.0 && (if KDW > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((KDY + BAH) + (JJR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if KDY > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let KEK;
                    let KEL;
                    if KDZ != 0.0 {
                        let KEH = KDU / KDW;
                        let KEI = KAO - KEH;
                        let KEJ = KAP - ((KDV - (KDX * KEH)) / KDW);
                        KEK = KEI;
                        KEL = KEJ;
                    } else {
                        KEK = KAO;
                        KEL = KAP;
                    }
                    KEA = KEK;
                    KEB = KEL;
                } else {
                    KEA = KAO;
                    KEB = KAP;
                }
                let KEC = JJR * KEA;
                let KED = (JJT * KEA) + (KEB * JJR);
                let KEE = JJI - KEA;
                let KEF = JJJ - KEB;
                let KEG = if KEE < SY { 1.0 } else { 0.0 };
                let KEU;
                let KEV;
                if KEG != 0.0 {
                    let KEM = KEE.exp();
                    let KEN = KEF * KEM;
                    KEU = KEM;
                    KEV = KEN;
                } else {
                    let KEO = KEE - SY;
                    let KEP = GO * KEO;
                    let KEQ = D + (KEO * WD);
                    let KER = D + (KEP * KEQ);
                    let KES = XB * (D + (KEO * KER));
                    let KET = ((KEF * KER) + ((((KEF * GO) * KEQ) + ((KEF * WD) * KEP)) * KEO)) * XB;
                    KEU = KES;
                    KEV = KET;
                }
                let KEW = JKG * KEU;
                let KEX = (JKH * KEU) + (KEV * JKG);
                let KEY = KED * KEC;
                let KEZ = (KEC * KEC) - KEW;
                let KFA = (KEY + KEY) - KEX;
                let KFB = (JPW * KEC) + KEW;
                let KFC = ((JPX * KEC) + (KED * JPW)) + KEX;
                let KFD = JQA - KEW;
                let KFE = JQB - KEX;
                let KFF = if KEZ < -5e-3f64 { 1.0 } else { 0.0 };
                let KGI;
                let KGJ;
                let KGK;
                let KGL;
                let KGM;
                let KGN;
                let KGO;
                let KGP;
                let KGQ;
                let KGR;
                let KGS;
                let KGT;
                let KGU;
                let KGV;
                if KFF != 0.0 {
                    let KFG = (KEZ.abs()).sqrt();
                    let KFH = (KFA * ((GX * (if KEZ >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * KFG));
                    let KFI = GO * KFG;
                    let KFJ = KFI.tan();
                    let KFK = KFI.cos();
                    let KFL = KFG / KFJ;
                    let KFM = (KFH - (((KFH * GO) * (GY / (KFK * KFK))) * KFL)) / KFJ;
                    let KFN = (YY * KFB) / KEZ;
                    let KFO = ((KFC * YY) - (KFA * KFN)) / KEZ;
                    let KFP = LA - KFL;
                    let KFQ = KEZ + (KFL * KFP);
                    let KFR = KFQ * KFN;
                    let KFS = ((KFA + ((KFM * KFP) + ((KFM * AC) * KFL))) * KFN) + (KFO * KFQ);
                    let KFT = LA * KFR;
                    let KFU = D + KFL;
                    let KFV = KFB - (KFT * KFU);
                    let KFW = (KFR * KFD) / KFB;
                    let KFX = (KFV * KFN) + KFW;
                    let KFY = (((KFC - (((KFS * LA) * KFU) + (KFM * KFT))) * KFN) + (KFO * KFV)) + ((((KFS * KFD) + (KFE * KFR)) - (KFC * KFW)) / KFB);
                    let KFZ = D - (GO * KFL);
                    let KGA = (KFM * GO) * AC;
                    let KGB = KFB / KEZ;
                    let KGC = KGB * KFZ;
                    let KGD = (((KFC - (KFA * KGB)) / KEZ) * KFZ) + (KGA * KGB);
                    let KGE = KGC + (GO * KFR);
                    let KGF = ((KFD * KFZ) - (KFB * KGE)) / KEZ;
                    let KGG = ((((KFE * KFZ) + (KGA * KFD)) - ((KFC * KGE) + ((KGD + (KFS * GO)) * KFB))) - (KFA * KGF)) / KEZ;
                    KGI = KCA;
                    KGJ = KFG;
                    KGK = KFL;
                    KGL = KFR;
                    KGM = KFX;
                    KGN = KGC;
                    KGO = KGF;
                    KGP = KCE;
                    KGQ = KFH;
                    KGR = KFM;
                    KGS = KFS;
                    KGT = KFY;
                    KGU = KGD;
                    KGV = KGG;
                } else {
                    let KGH = if KEZ > AQJ { 1.0 } else { 0.0 };
                    let KJN;
                    let KJO;
                    let KJP;
                    let KJQ;
                    let KJR;
                    let KJS;
                    let KJT;
                    let KJU;
                    let KJV;
                    let KJW;
                    let KJX;
                    let KJY;
                    let KJZ;
                    let KKA;
                    if KGH != 0.0 {
                        let KGX = (KEZ.abs()).sqrt();
                        let KGY = (KFA * ((GX * (if KEZ >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * KGX));
                        let KGZ = (-KGX).exp();
                        let KHA = (KGY * AC) * KGZ;
                        let KHB = D + KGZ;
                        let KHC = D - KGZ;
                        let KHD = (KGX * KHB) / KHC;
                        let KHE = (((KGY * KHB) + (KHA * KGX)) - ((KHA * AC) * KHD)) / KHC;
                        let KHF = (YY * KFB) / KEZ;
                        let KHG = ((KFC * YY) - (KFA * KHF)) / KEZ;
                        let KHH = LA - KHD;
                        let KHI = KEZ + (KHD * KHH);
                        let KHJ = KHI * KHF;
                        let KHK = ((KFA + ((KHE * KHH) + ((KHE * AC) * KHD))) * KHF) + (KHG * KHI);
                        let KHL = LA * KHJ;
                        let KHM = D + KHD;
                        let KHN = KFB - (KHL * KHM);
                        let KHO = (KHJ * KFD) / KFB;
                        let KHP = (KHN * KHF) + KHO;
                        let KHQ = (((KFC - (((KHK * LA) * KHM) + (KHE * KHL))) * KHF) + (KHG * KHN)) + ((((KHK * KFD) + (KFE * KHJ)) - (KFC * KHO)) / KFB);
                        let KHR = D - (GO * KHD);
                        let KHS = (KHE * GO) * AC;
                        let KHT = KFB / KEZ;
                        let KHU = KHT * KHR;
                        let KHV = (((KFC - (KFA * KHT)) / KEZ) * KHR) + (KHS * KHT);
                        let KHW = KHU + (GO * KHJ);
                        let KHX = ((KFD * KHR) - (KFB * KHW)) / KEZ;
                        let KHY = ((((KFE * KHR) + (KHS * KFD)) - ((KFC * KHW) + ((KHV + (KHK * GO)) * KFB))) - (KFA * KHX)) / KEZ;
                        KJN = KGZ;
                        KJO = KGX;
                        KJP = KHD;
                        KJQ = KHJ;
                        KJR = KHP;
                        KJS = KHU;
                        KJT = KHX;
                        KJU = KHA;
                        KJV = KGY;
                        KJW = KHE;
                        KJX = KHK;
                        KJY = KHQ;
                        KJZ = KHV;
                        KKA = KHY;
                    } else {
                        let KHZ = KEZ * ASC;
                        let KIA = KEZ * ASE;
                        let KIB = KFA * ASE;
                        let KIC = D - (KEZ * ASH);
                        let KID = D - (KIA * KIC);
                        let KIE = UC * (D - (KHZ * KID));
                        let KIF = ((((KFA * ASC) * KID) + ((((KIB * KIC) + (((KFA * ASH) * AC) * KIA)) * AC) * KHZ)) * AC) * UC;
                        let KIG = (KFA * KIE) + (KIF * KEZ);
                        let KIH = LA + (KEZ * KIE);
                        let KII = KEZ * ASO;
                        let KIJ = KFA * ASO;
                        let KIK = KEZ * ASR;
                        let KIL = D - KII;
                        let KIM = D - (KIK * KIL);
                        let KIN = UC * (D - (KII * KIM));
                        let KIO = (((KIJ * KIM) + (((((KFA * ASR) * KIL) + ((KIJ * AC) * KIK)) * AC) * KII)) * AC) * UC;
                        let KIP = KFB * KIN;
                        let KIQ = (KFC * KIN) + (KIO * KFB);
                        let KIR = KEZ * ASZ;
                        let KIS = ATB * KEZ;
                        let KIT = D - (ATD * KEZ);
                        let KIU = D - (KIS * KIT);
                        let KIV = ATG * (D - (KIR * KIU));
                        let KIW = KFB * KFB;
                        let KIX = KFC * KFB;
                        let KIY = (KFD * KIN) - (KIW * KIV);
                        let KIZ = ((KFE * KIN) + (KIO * KFD)) - (((KIX + KIX) * KIV) + ((((((KFA * ASZ) * KIU) + (((((KFA * ATB) * KIT) + (((KFA * ATD) * AC) * KIS)) * AC) * KIR)) * AC) * ATG) * KIW));
                        let KJB = KJA * KFB;
                        let KJC = KJB * KIE;
                        let KJD = ((KFC * KJA) * KIE) + (KIF * KJB);
                        let KJF = KJE * KFD;
                        let KJH = KJG * KFB;
                        let KJI = KJH * KFB;
                        let KJJ = LA - (ATV * KEZ);
                        let KJK = D - (KIA * KJJ);
                        let KJL = (KJF * KIE) + (KJI * KJK);
                        let KJM = (((KFE * KJE) * KIE) + (KIF * KJF)) + (((((KFC * KJG) * KFB) + (KFC * KJH)) * KJK) + ((((KIB * KJJ) + (((KFA * ATV) * AC) * KIA)) * AC) * KJI));
                        KJN = KCA;
                        KJO = KCB;
                        KJP = KIH;
                        KJQ = KIP;
                        KJR = KIY;
                        KJS = KJC;
                        KJT = KJL;
                        KJU = KCE;
                        KJV = KCF;
                        KJW = KIG;
                        KJX = KIQ;
                        KJY = KIZ;
                        KJZ = KJD;
                        KKA = KJM;
                    }
                    KGI = KJN;
                    KGJ = KJO;
                    KGK = KJP;
                    KGL = KJQ;
                    KGM = KJR;
                    KGN = KJS;
                    KGO = KJT;
                    KGP = KJU;
                    KGQ = KJV;
                    KGR = KJW;
                    KGS = KJX;
                    KGT = KJY;
                    KGU = KJZ;
                    KGV = KKA;
                }
                let KGW = if KEZ > AQJ { 1.0 } else { 0.0 };
                let KKK;
                let KKL;
                let KKM;
                let KKN;
                if KGW != 0.0 {
                    let KKB = LA - KGI;
                    let KKC = D - (KGI * KKB);
                    let KKD = (XQ * KEZ) / KKC;
                    let KKE = ((KFA * XQ) - ((((KGP * KKB) + ((KGP * AC) * KGI)) * AC) * KKD)) / KKC;
                    let KKF = KKD * KGI;
                    let KKG = (KKE * KGI) + (KGP * KKD);
                    let KKH = (KKD.ln()) - KGJ;
                    let KKI = (KKE * (GY / KKD)) - KGQ;
                    KKK = KKF;
                    KKL = KKH;
                    KKM = KKG;
                    KKN = KKI;
                } else {
                    let KKJ = if KEZ < -5e-3f64 { 1.0 } else { 0.0 };
                    let KLF;
                    let KLG;
                    let KLH;
                    let KLI;
                    if KKJ != 0.0 {
                        let KKP = GO * KGJ;
                        let KKQ = KKP.sin();
                        let KKR = KKQ * KKQ;
                        let KKS = ((KGQ * GO) * (KKP.cos())) * KKQ;
                        let KKT = (-KEZ) / KKR;
                        let KKU = ((KFA * AC) - ((KKS + KKS) * KKT)) / KKR;
                        let KKV = KKT.ln();
                        let KKW = KKU * (GY / KKT);
                        KLF = KKT;
                        KLG = KKV;
                        KLH = KKU;
                        KLI = KKW;
                    } else {
                        let KKX = KEZ * WD;
                        let KKY = ATB * KEZ;
                        let KKZ = D - (AVN * KEZ);
                        let KLA = D - (KKY * KKZ);
                        let KLB = XQ - (KKX * KLA);
                        let KLC = (((KFA * WD) * KLA) + (((((KFA * ATB) * KKZ) + (((KFA * AVN) * AC) * KKY)) * AC) * KKX)) * AC;
                        let KLD = KLB.ln();
                        let KLE = KLC * (GY / KLB);
                        KLF = KLB;
                        KLG = KLD;
                        KLH = KLC;
                        KLI = KLE;
                    }
                    KKK = KLF;
                    KKL = KLG;
                    KKM = KLH;
                    KKN = KLI;
                }
                let KKO = if ((AVB * KEC) + KGK) > B { 1.0 } else { 0.0 };
                let KME;
                let KMF;
                let KMG;
                let KMH;
                let KMI;
                let KMJ;
                if KKO != 0.0 {
                    let KLJ = KEC + KGK;
                    let KLK = KED + KGR;
                    let KLL = JJR + KGL;
                    let KLM = JJT + KGS;
                    KME = KLJ;
                    KMF = KLL;
                    KMG = KGM;
                    KMH = KLK;
                    KMI = KLM;
                    KMJ = KGT;
                } else {
                    let KLN = KEC - KGK;
                    let KLO = D / KLN;
                    let KLP = (((KED - KGR) * KLO) * AC) / KLN;
                    let KLQ = KGL - JJR;
                    let KLR = KGS - JJT;
                    let KLS = KEW - KKK;
                    let KLT = KLS * KLO;
                    let KLU = ((KEX - KKM) * KLO) + (KLP * KLS);
                    let KLV = ((KLQ * KLT) - KEW) - (KGN * KKK);
                    let KLW = KLV * KLO;
                    let KLX = (((((KLR * KLT) + (KLU * KLQ)) - KEX) - ((KGU * KKK) + (KKM * KGN))) * KLO) + (KLP * KLV);
                    let KLY = LA * KLQ;
                    let KLZ = KGU * KGN;
                    let KMA = KGO + (KGN * KGN);
                    let KMB = (((KGM * KLT) + (KLY * KLW)) + KEW) - (KMA * KKK);
                    let KMC = KMB * KLO;
                    let KMD = ((((((KGT * KLT) + (KLU * KGM)) + (((KLR * LA) * KLW) + (KLX * KLY))) + KEX) - (((KGV + (KLZ + KLZ)) * KKK) + (KKM * KMA))) * KLO) + (KLP * KMB);
                    KME = KLT;
                    KMF = KLW;
                    KMG = KMC;
                    KMH = KLU;
                    KMI = KLX;
                    KMJ = KMD;
                }
                let KMK = if KME > B { 1.0 } else { 0.0 };
                let KNE;
                let KNF;
                let KNG;
                let KNH;
                let KNI;
                let KNJ;
                if KMK != 0.0 {
                    let KML = KME.ln();
                    let KMM = KMH * (GY / KME);
                    let KMN = D / KME;
                    let KMO = ((KMH * KMN) * AC) / KME;
                    let KMP = KMF * KMN;
                    let KMQ = (KMI * KMN) + (KMO * KMF);
                    let KMR = KMQ * KMP;
                    let KMS = (KMG * KMN) - (KMP * KMP);
                    let KMT = ((KMJ * KMN) + (KMO * KMG)) - (KMR + KMR);
                    KNE = KML;
                    KNF = KMP;
                    KNG = KMS;
                    KNH = KMM;
                    KNI = KMQ;
                    KNJ = KMT;
                } else {
                    let KMU = -KEC;
                    let KMV = (KEC + HW) + (KMU.ln());
                    let KMW = KED + ((KED * AC) * (GY / KMU));
                    let KMX = D / KEA;
                    let KMY = ((KEB * KMX) * AC) / KEA;
                    let KMZ = JJR + KMX;
                    let KNA = JJT + KMY;
                    let KNB = -KMX;
                    let KNC = KNB * KMX;
                    let KND = ((KMY * AC) * KMX) + (KMY * KNB);
                    KNE = KMV;
                    KNF = KMZ;
                    KNG = KNC;
                    KNH = KMW;
                    KNI = KNA;
                    KNJ = KND;
                }
                let KNK = ((JYJ + KEA) + (LA * KNE)) - KKL;
                let KNL = (D + (LA * KNF)) - KGN;
                let KNM = (LA * KNG) - KGO;
                let KNN = KEC + (JJU * KNK);
                let KNO = KED + ((JJW * KNK) + ((((JYK + KEB) + (KNH * LA)) - KKN) * JJU));
                let KNP = JJR + (JJU * KNL);
                let KNQ = JJT + ((JJW * KNL) + (((KNI * LA) - KGU) * JJU));
                let KNR = JJU * KNM;
                let KNS = (KNN * KME) - KEW;
                let KNT = ((KNO * KME) + (KMH * KNN)) - KEX;
                let KNU = ((KNP * KME) + (KNN * KMF)) + KEW;
                let KNV = (((KNQ * KME) + (KMH * KNP)) + ((KNO * KMF) + (KMI * KNN))) + KEX;
                let KNW = LA * KNP;
                let KNX = (((KNR * KME) + (KNW * KMF)) + (KNN * KMG)) - KEW;
                let KNY = KNV * KNU;
                let KNZ = GO * KNS;
                let KOA = (KNU * KNU) - (KNZ * KNX);
                let KOB = (KNY + KNY) - (((KNT * GO) * KNX) + ((((((((JJW * KNM) + (((KNJ * LA) - KGV) * JJU)) * KME) + (KMH * KNR)) + (((KNQ * LA) * KMF) + (KMI * KNW))) + ((KNO * KMG) + (KMJ * KNN))) - KEX) * KNZ));
                let KOC = -KNS;
                let KOD = KOC * KNU;
                let KOE = KOB * KOA;
                let KOF = (KOA * KOA) + AYW;
                let KOG = (KOD * KOA) / KOF;
                let KOH = KEA + KOG;
                let KOI = KEB + (((((((KNT * AC) * KNU) + (KNV * KOC)) * KOA) + (KOB * KOD)) - ((KOE + KOE) * KOG)) / KOF);
                let KOJ = JJR * KOH;
                let KOK = (JJT * KOH) + (KOI * JJR);
                let KOL = JJI - KOH;
                let KOM = JJJ - KOI;
                let KON = if KOL < SY { 1.0 } else { 0.0 };
                let KOW;
                let KOX;
                if KON != 0.0 {
                    let KOO = KOL.exp();
                    let KOP = KOM * KOO;
                    KOW = KOO;
                    KOX = KOP;
                } else {
                    let KOQ = KOL - SY;
                    let KOR = GO * KOQ;
                    let KOS = D + (KOQ * WD);
                    let KOT = D + (KOR * KOS);
                    let KOU = XB * (D + (KOQ * KOT));
                    let KOV = ((KOM * KOT) + ((((KOM * GO) * KOS) + ((KOM * WD) * KOR)) * KOQ)) * XB;
                    KOW = KOU;
                    KOX = KOV;
                }
                let KOY = JKG * KOW;
                let KOZ = (JKH * KOW) + (KOX * JKG);
                let KPA = KOK * KOJ;
                let KPB = (KOJ * KOJ) - KOY;
                let KPC = (KPA + KPA) - KOZ;
                let KPD = (JPW * KOJ) + KOY;
                let KPE = ((JPX * KOJ) + (KOK * JPW)) + KOZ;
                let KPF = JQA - KOY;
                let KPG = JQB - KOZ;
                let KPH = if KPB < -5e-3f64 { 1.0 } else { 0.0 };
                let KQK;
                let KQL;
                let KQM;
                let KQN;
                let KQO;
                let KQP;
                let KQQ;
                let KQR;
                let KQS;
                let KQT;
                let KQU;
                let KQV;
                let KQW;
                let KQX;
                if KPH != 0.0 {
                    let KPI = (KPB.abs()).sqrt();
                    let KPJ = (KPC * ((GX * (if KPB >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * KPI));
                    let KPK = GO * KPI;
                    let KPL = KPK.tan();
                    let KPM = KPK.cos();
                    let KPN = KPI / KPL;
                    let KPO = (KPJ - (((KPJ * GO) * (GY / (KPM * KPM))) * KPN)) / KPL;
                    let KPP = (YY * KPD) / KPB;
                    let KPQ = ((KPE * YY) - (KPC * KPP)) / KPB;
                    let KPR = LA - KPN;
                    let KPS = KPB + (KPN * KPR);
                    let KPT = KPS * KPP;
                    let KPU = ((KPC + ((KPO * KPR) + ((KPO * AC) * KPN))) * KPP) + (KPQ * KPS);
                    let KPV = LA * KPT;
                    let KPW = D + KPN;
                    let KPX = KPD - (KPV * KPW);
                    let KPY = (KPT * KPF) / KPD;
                    let KPZ = (KPX * KPP) + KPY;
                    let KQA = (((KPE - (((KPU * LA) * KPW) + (KPO * KPV))) * KPP) + (KPQ * KPX)) + ((((KPU * KPF) + (KPG * KPT)) - (KPE * KPY)) / KPD);
                    let KQB = D - (GO * KPN);
                    let KQC = (KPO * GO) * AC;
                    let KQD = KPD / KPB;
                    let KQE = KQD * KQB;
                    let KQF = (((KPE - (KPC * KQD)) / KPB) * KQB) + (KQC * KQD);
                    let KQG = KQE + (GO * KPT);
                    let KQH = ((KPF * KQB) - (KPD * KQG)) / KPB;
                    let KQI = ((((KPG * KQB) + (KQC * KPF)) - ((KPE * KQG) + ((KQF + (KPU * GO)) * KPD))) - (KPC * KQH)) / KPB;
                    KQK = KGI;
                    KQL = KPI;
                    KQM = KPN;
                    KQN = KPT;
                    KQO = KPZ;
                    KQP = KQE;
                    KQQ = KQH;
                    KQR = KGP;
                    KQS = KPJ;
                    KQT = KPO;
                    KQU = KPU;
                    KQV = KQA;
                    KQW = KQF;
                    KQX = KQI;
                } else {
                    let KQJ = if KPB > AQJ { 1.0 } else { 0.0 };
                    let KTP;
                    let KTQ;
                    let KTR;
                    let KTS;
                    let KTT;
                    let KTU;
                    let KTV;
                    let KTW;
                    let KTX;
                    let KTY;
                    let KTZ;
                    let KUA;
                    let KUB;
                    let KUC;
                    if KQJ != 0.0 {
                        let KQZ = (KPB.abs()).sqrt();
                        let KRA = (KPC * ((GX * (if KPB >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * KQZ));
                        let KRB = (-KQZ).exp();
                        let KRC = (KRA * AC) * KRB;
                        let KRD = D + KRB;
                        let KRE = D - KRB;
                        let KRF = (KQZ * KRD) / KRE;
                        let KRG = (((KRA * KRD) + (KRC * KQZ)) - ((KRC * AC) * KRF)) / KRE;
                        let KRH = (YY * KPD) / KPB;
                        let KRI = ((KPE * YY) - (KPC * KRH)) / KPB;
                        let KRJ = LA - KRF;
                        let KRK = KPB + (KRF * KRJ);
                        let KRL = KRK * KRH;
                        let KRM = ((KPC + ((KRG * KRJ) + ((KRG * AC) * KRF))) * KRH) + (KRI * KRK);
                        let KRN = LA * KRL;
                        let KRO = D + KRF;
                        let KRP = KPD - (KRN * KRO);
                        let KRQ = (KRL * KPF) / KPD;
                        let KRR = (KRP * KRH) + KRQ;
                        let KRS = (((KPE - (((KRM * LA) * KRO) + (KRG * KRN))) * KRH) + (KRI * KRP)) + ((((KRM * KPF) + (KPG * KRL)) - (KPE * KRQ)) / KPD);
                        let KRT = D - (GO * KRF);
                        let KRU = (KRG * GO) * AC;
                        let KRV = KPD / KPB;
                        let KRW = KRV * KRT;
                        let KRX = (((KPE - (KPC * KRV)) / KPB) * KRT) + (KRU * KRV);
                        let KRY = KRW + (GO * KRL);
                        let KRZ = ((KPF * KRT) - (KPD * KRY)) / KPB;
                        let KSA = ((((KPG * KRT) + (KRU * KPF)) - ((KPE * KRY) + ((KRX + (KRM * GO)) * KPD))) - (KPC * KRZ)) / KPB;
                        KTP = KRB;
                        KTQ = KQZ;
                        KTR = KRF;
                        KTS = KRL;
                        KTT = KRR;
                        KTU = KRW;
                        KTV = KRZ;
                        KTW = KRC;
                        KTX = KRA;
                        KTY = KRG;
                        KTZ = KRM;
                        KUA = KRS;
                        KUB = KRX;
                        KUC = KSA;
                    } else {
                        let KSB = KPB * ASC;
                        let KSC = KPB * ASE;
                        let KSD = KPC * ASE;
                        let KSE = D - (KPB * ASH);
                        let KSF = D - (KSC * KSE);
                        let KSG = UC * (D - (KSB * KSF));
                        let KSH = ((((KPC * ASC) * KSF) + ((((KSD * KSE) + (((KPC * ASH) * AC) * KSC)) * AC) * KSB)) * AC) * UC;
                        let KSI = (KPC * KSG) + (KSH * KPB);
                        let KSJ = LA + (KPB * KSG);
                        let KSK = KPB * ASO;
                        let KSL = KPC * ASO;
                        let KSM = KPB * ASR;
                        let KSN = D - KSK;
                        let KSO = D - (KSM * KSN);
                        let KSP = UC * (D - (KSK * KSO));
                        let KSQ = (((KSL * KSO) + (((((KPC * ASR) * KSN) + ((KSL * AC) * KSM)) * AC) * KSK)) * AC) * UC;
                        let KSR = KPD * KSP;
                        let KSS = (KPE * KSP) + (KSQ * KPD);
                        let KST = KPB * ASZ;
                        let KSU = ATB * KPB;
                        let KSV = D - (ATD * KPB);
                        let KSW = D - (KSU * KSV);
                        let KSX = ATG * (D - (KST * KSW));
                        let KSY = KPD * KPD;
                        let KSZ = KPE * KPD;
                        let KTA = (KPF * KSP) - (KSY * KSX);
                        let KTB = ((KPG * KSP) + (KSQ * KPF)) - (((KSZ + KSZ) * KSX) + ((((((KPC * ASZ) * KSW) + (((((KPC * ATB) * KSV) + (((KPC * ATD) * AC) * KSU)) * AC) * KST)) * AC) * ATG) * KSY));
                        let KTD = KTC * KPD;
                        let KTE = KTD * KSG;
                        let KTF = ((KPE * KTC) * KSG) + (KSH * KTD);
                        let KTH = KTG * KPF;
                        let KTJ = KTI * KPD;
                        let KTK = KTJ * KPD;
                        let KTL = LA - (ATV * KPB);
                        let KTM = D - (KSC * KTL);
                        let KTN = (KTH * KSG) + (KTK * KTM);
                        let KTO = (((KPG * KTG) * KSG) + (KSH * KTH)) + (((((KPE * KTI) * KPD) + (KPE * KTJ)) * KTM) + ((((KSD * KTL) + (((KPC * ATV) * AC) * KSC)) * AC) * KTK));
                        KTP = KGI;
                        KTQ = KGJ;
                        KTR = KSJ;
                        KTS = KSR;
                        KTT = KTA;
                        KTU = KTE;
                        KTV = KTN;
                        KTW = KGP;
                        KTX = KGQ;
                        KTY = KSI;
                        KTZ = KSS;
                        KUA = KTB;
                        KUB = KTF;
                        KUC = KTO;
                    }
                    KQK = KTP;
                    KQL = KTQ;
                    KQM = KTR;
                    KQN = KTS;
                    KQO = KTT;
                    KQP = KTU;
                    KQQ = KTV;
                    KQR = KTW;
                    KQS = KTX;
                    KQT = KTY;
                    KQU = KTZ;
                    KQV = KUA;
                    KQW = KUB;
                    KQX = KUC;
                }
                let KQY = if KPB > AQJ { 1.0 } else { 0.0 };
                let KUM;
                let KUN;
                let KUO;
                let KUP;
                if KQY != 0.0 {
                    let KUD = LA - KQK;
                    let KUE = D - (KQK * KUD);
                    let KUF = (XQ * KPB) / KUE;
                    let KUG = ((KPC * XQ) - ((((KQR * KUD) + ((KQR * AC) * KQK)) * AC) * KUF)) / KUE;
                    let KUH = KUF * KQK;
                    let KUI = (KUG * KQK) + (KQR * KUF);
                    let KUJ = (KUF.ln()) - KQL;
                    let KUK = (KUG * (GY / KUF)) - KQS;
                    KUM = KUH;
                    KUN = KUJ;
                    KUO = KUI;
                    KUP = KUK;
                } else {
                    let KUL = if KPB < -5e-3f64 { 1.0 } else { 0.0 };
                    let KVH;
                    let KVI;
                    let KVJ;
                    let KVK;
                    if KUL != 0.0 {
                        let KUR = GO * KQL;
                        let KUS = KUR.sin();
                        let KUT = KUS * KUS;
                        let KUU = ((KQS * GO) * (KUR.cos())) * KUS;
                        let KUV = (-KPB) / KUT;
                        let KUW = ((KPC * AC) - ((KUU + KUU) * KUV)) / KUT;
                        let KUX = KUV.ln();
                        let KUY = KUW * (GY / KUV);
                        KVH = KUV;
                        KVI = KUX;
                        KVJ = KUW;
                        KVK = KUY;
                    } else {
                        let KUZ = KPB * WD;
                        let KVA = ATB * KPB;
                        let KVB = D - (AVN * KPB);
                        let KVC = D - (KVA * KVB);
                        let KVD = XQ - (KUZ * KVC);
                        let KVE = (((KPC * WD) * KVC) + (((((KPC * ATB) * KVB) + (((KPC * AVN) * AC) * KVA)) * AC) * KUZ)) * AC;
                        let KVF = KVD.ln();
                        let KVG = KVE * (GY / KVD);
                        KVH = KVD;
                        KVI = KVF;
                        KVJ = KVE;
                        KVK = KVG;
                    }
                    KUM = KVH;
                    KUN = KVI;
                    KUO = KVJ;
                    KUP = KVK;
                }
                let KUQ = if ((AVB * KOJ) + KQM) > B { 1.0 } else { 0.0 };
                let KWG;
                let KWH;
                let KWI;
                let KWJ;
                let KWK;
                let KWL;
                if KUQ != 0.0 {
                    let KVL = KOJ + KQM;
                    let KVM = KOK + KQT;
                    let KVN = JJR + KQN;
                    let KVO = JJT + KQU;
                    KWG = KVL;
                    KWH = KVN;
                    KWI = KQO;
                    KWJ = KVM;
                    KWK = KVO;
                    KWL = KQV;
                } else {
                    let KVP = KOJ - KQM;
                    let KVQ = D / KVP;
                    let KVR = (((KOK - KQT) * KVQ) * AC) / KVP;
                    let KVS = KQN - JJR;
                    let KVT = KQU - JJT;
                    let KVU = KOY - KUM;
                    let KVV = KVU * KVQ;
                    let KVW = ((KOZ - KUO) * KVQ) + (KVR * KVU);
                    let KVX = ((KVS * KVV) - KOY) - (KQP * KUM);
                    let KVY = KVX * KVQ;
                    let KVZ = (((((KVT * KVV) + (KVW * KVS)) - KOZ) - ((KQW * KUM) + (KUO * KQP))) * KVQ) + (KVR * KVX);
                    let KWA = LA * KVS;
                    let KWB = KQW * KQP;
                    let KWC = KQQ + (KQP * KQP);
                    let KWD = (((KQO * KVV) + (KWA * KVY)) + KOY) - (KWC * KUM);
                    let KWE = KWD * KVQ;
                    let KWF = ((((((KQV * KVV) + (KVW * KQO)) + (((KVT * LA) * KVY) + (KVZ * KWA))) + KOZ) - (((KQX + (KWB + KWB)) * KUM) + (KUO * KWC))) * KVQ) + (KVR * KWD);
                    KWG = KVV;
                    KWH = KVY;
                    KWI = KWE;
                    KWJ = KVW;
                    KWK = KVZ;
                    KWL = KWF;
                }
                let KWM = if KWG > B { 1.0 } else { 0.0 };
                let KXG;
                let KXH;
                let KXI;
                let KXJ;
                let KXK;
                let KXL;
                if KWM != 0.0 {
                    let KWN = KWG.ln();
                    let KWO = KWJ * (GY / KWG);
                    let KWP = D / KWG;
                    let KWQ = ((KWJ * KWP) * AC) / KWG;
                    let KWR = KWH * KWP;
                    let KWS = (KWK * KWP) + (KWQ * KWH);
                    let KWT = KWS * KWR;
                    let KWU = (KWI * KWP) - (KWR * KWR);
                    let KWV = ((KWL * KWP) + (KWQ * KWI)) - (KWT + KWT);
                    KXG = KWN;
                    KXH = KWR;
                    KXI = KWU;
                    KXJ = KWO;
                    KXK = KWS;
                    KXL = KWV;
                } else {
                    let KWW = -KOJ;
                    let KWX = (KOJ + HW) + (KWW.ln());
                    let KWY = KOK + ((KOK * AC) * (GY / KWW));
                    let KWZ = D / KOH;
                    let KXA = ((KOI * KWZ) * AC) / KOH;
                    let KXB = JJR + KWZ;
                    let KXC = JJT + KXA;
                    let KXD = -KWZ;
                    let KXE = KXD * KWZ;
                    let KXF = ((KXA * AC) * KWZ) + (KXA * KXD);
                    KXG = KWX;
                    KXH = KXB;
                    KXI = KXE;
                    KXJ = KWY;
                    KXK = KXC;
                    KXL = KXF;
                }
                let KXM = ((JYJ + KOH) + (LA * KXG)) - KUN;
                let KXN = (D + (LA * KXH)) - KQP;
                let KXO = (LA * KXI) - KQQ;
                let KXP = KOJ + (JJU * KXM);
                let KXQ = KOK + ((JJW * KXM) + ((((JYK + KOI) + (KXJ * LA)) - KUP) * JJU));
                let KXR = JJR + (JJU * KXN);
                let KXS = JJT + ((JJW * KXN) + (((KXK * LA) - KQW) * JJU));
                let KXT = JJU * KXO;
                let KXU = (KXP * KWG) - KOY;
                let KXV = ((KXQ * KWG) + (KWJ * KXP)) - KOZ;
                let KXW = ((KXR * KWG) + (KXP * KWH)) + KOY;
                let KXX = (((KXS * KWG) + (KWJ * KXR)) + ((KXQ * KWH) + (KWK * KXP))) + KOZ;
                let KXY = LA * KXR;
                let KXZ = (((KXT * KWG) + (KXY * KWH)) + (KXP * KWI)) - KOY;
                let KYA = KXX * KXW;
                let KYB = GO * KXU;
                let KYC = (KXW * KXW) - (KYB * KXZ);
                let KYD = (KYA + KYA) - (((KXV * GO) * KXZ) + ((((((((JJW * KXO) + (((KXL * LA) - KQX) * JJU)) * KWG) + (KWJ * KXT)) + (((KXS * LA) * KWH) + (KWK * KXY))) + ((KXQ * KWI) + (KWL * KXP))) - KOZ) * KYB));
                let KYE = -KXU;
                let KYF = KYE * KXW;
                let KYG = KYD * KYC;
                let KYH = (KYC * KYC) + AYW;
                let KYI = (KYF * KYC) / KYH;
                let KYJ = KOH + KYI;
                let KYK = KOI + (((((((KXV * AC) * KXW) + (KXX * KYE)) * KYC) + (KYD * KYF)) - ((KYG + KYG) * KYI)) / KYH);
                let KYM;
                let KYN;
                let KYO;
                let KYP;
                let KYQ;
                let KYR;
                if A != 0.0 {
                    let KYL = if (KYI.abs()) > NI { 1.0 } else { 0.0 };
                    let KZC;
                    let KZD;
                    let KZE;
                    let KZF;
                    let KZG;
                    let KZH;
                    if KYL != 0.0 {
                        let KYX = JJR * KYJ;
                        let KYY = (JJT * KYJ) + (KYK * JJR);
                        let KYZ = JJI - KYJ;
                        let KZA = JJJ - KYK;
                        let KZB = if KYZ < SY { 1.0 } else { 0.0 };
                        let KZQ;
                        let KZR;
                        if KZB != 0.0 {
                            let KZI = KYZ.exp();
                            let KZJ = KZA * KZI;
                            KZQ = KZI;
                            KZR = KZJ;
                        } else {
                            let KZK = KYZ - SY;
                            let KZL = GO * KZK;
                            let KZM = D + (KZK * WD);
                            let KZN = D + (KZL * KZM);
                            let KZO = XB * (D + (KZK * KZN));
                            let KZP = ((KZA * KZN) + ((((KZA * GO) * KZM) + ((KZA * WD) * KZL)) * KZK)) * XB;
                            KZQ = KZO;
                            KZR = KZP;
                        }
                        let KZS = JKG * KZQ;
                        let KZT = (JKH * KZQ) + (KZR * JKG);
                        let KZU = KYY * KYX;
                        let KZV = (KYX * KYX) - KZS;
                        let KZW = (KZU + KZU) - KZT;
                        let KZX = (JPW * KYX) + KZS;
                        let KZY = ((JPX * KYX) + (KYY * JPW)) + KZT;
                        let KZZ = JQA - KZS;
                        let LAA = JQB - KZT;
                        let LAB = if KZV < -5e-3f64 { 1.0 } else { 0.0 };
                        let LBE;
                        let LBF;
                        let LBG;
                        let LBH;
                        let LBI;
                        let LBJ;
                        let LBK;
                        let LBL;
                        let LBM;
                        let LBN;
                        let LBO;
                        let LBP;
                        let LBQ;
                        let LBR;
                        if LAB != 0.0 {
                            let LAC = (KZV.abs()).sqrt();
                            let LAD = (KZW * ((GX * (if KZV >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * LAC));
                            let LAE = GO * LAC;
                            let LAF = LAE.tan();
                            let LAG = LAE.cos();
                            let LAH = LAC / LAF;
                            let LAI = (LAD - (((LAD * GO) * (GY / (LAG * LAG))) * LAH)) / LAF;
                            let LAJ = (YY * KZX) / KZV;
                            let LAK = ((KZY * YY) - (KZW * LAJ)) / KZV;
                            let LAL = LA - LAH;
                            let LAM = KZV + (LAH * LAL);
                            let LAN = LAM * LAJ;
                            let LAO = ((KZW + ((LAI * LAL) + ((LAI * AC) * LAH))) * LAJ) + (LAK * LAM);
                            let LAP = LA * LAN;
                            let LAQ = D + LAH;
                            let LAR = KZX - (LAP * LAQ);
                            let LAS = (LAN * KZZ) / KZX;
                            let LAT = (LAR * LAJ) + LAS;
                            let LAU = (((KZY - (((LAO * LA) * LAQ) + (LAI * LAP))) * LAJ) + (LAK * LAR)) + ((((LAO * KZZ) + (LAA * LAN)) - (KZY * LAS)) / KZX);
                            let LAV = D - (GO * LAH);
                            let LAW = (LAI * GO) * AC;
                            let LAX = KZX / KZV;
                            let LAY = LAX * LAV;
                            let LAZ = (((KZY - (KZW * LAX)) / KZV) * LAV) + (LAW * LAX);
                            let LBA = LAY + (GO * LAN);
                            let LBB = ((KZZ * LAV) - (KZX * LBA)) / KZV;
                            let LBC = ((((LAA * LAV) + (LAW * KZZ)) - ((KZY * LBA) + ((LAZ + (LAO * GO)) * KZX))) - (KZW * LBB)) / KZV;
                            LBE = KQK;
                            LBF = LAC;
                            LBG = LAH;
                            LBH = LAN;
                            LBI = LAT;
                            LBJ = LAY;
                            LBK = LBB;
                            LBL = KQR;
                            LBM = LAD;
                            LBN = LAI;
                            LBO = LAO;
                            LBP = LAU;
                            LBQ = LAZ;
                            LBR = LBC;
                        } else {
                            let LBD = if KZV > AQJ { 1.0 } else { 0.0 };
                            let LEJ;
                            let LEK;
                            let LEL;
                            let LEM;
                            let LEN;
                            let LEO;
                            let LEP;
                            let LEQ;
                            let LER;
                            let LES;
                            let LET;
                            let LEU;
                            let LEV;
                            let LEW;
                            if LBD != 0.0 {
                                let LBT = (KZV.abs()).sqrt();
                                let LBU = (KZW * ((GX * (if KZV >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * LBT));
                                let LBV = (-LBT).exp();
                                let LBW = (LBU * AC) * LBV;
                                let LBX = D + LBV;
                                let LBY = D - LBV;
                                let LBZ = (LBT * LBX) / LBY;
                                let LCA = (((LBU * LBX) + (LBW * LBT)) - ((LBW * AC) * LBZ)) / LBY;
                                let LCB = (YY * KZX) / KZV;
                                let LCC = ((KZY * YY) - (KZW * LCB)) / KZV;
                                let LCD = LA - LBZ;
                                let LCE = KZV + (LBZ * LCD);
                                let LCF = LCE * LCB;
                                let LCG = ((KZW + ((LCA * LCD) + ((LCA * AC) * LBZ))) * LCB) + (LCC * LCE);
                                let LCH = LA * LCF;
                                let LCI = D + LBZ;
                                let LCJ = KZX - (LCH * LCI);
                                let LCK = (LCF * KZZ) / KZX;
                                let LCL = (LCJ * LCB) + LCK;
                                let LCM = (((KZY - (((LCG * LA) * LCI) + (LCA * LCH))) * LCB) + (LCC * LCJ)) + ((((LCG * KZZ) + (LAA * LCF)) - (KZY * LCK)) / KZX);
                                let LCN = D - (GO * LBZ);
                                let LCO = (LCA * GO) * AC;
                                let LCP = KZX / KZV;
                                let LCQ = LCP * LCN;
                                let LCR = (((KZY - (KZW * LCP)) / KZV) * LCN) + (LCO * LCP);
                                let LCS = LCQ + (GO * LCF);
                                let LCT = ((KZZ * LCN) - (KZX * LCS)) / KZV;
                                let LCU = ((((LAA * LCN) + (LCO * KZZ)) - ((KZY * LCS) + ((LCR + (LCG * GO)) * KZX))) - (KZW * LCT)) / KZV;
                                LEJ = LBV;
                                LEK = LBT;
                                LEL = LBZ;
                                LEM = LCF;
                                LEN = LCL;
                                LEO = LCQ;
                                LEP = LCT;
                                LEQ = LBW;
                                LER = LBU;
                                LES = LCA;
                                LET = LCG;
                                LEU = LCM;
                                LEV = LCR;
                                LEW = LCU;
                            } else {
                                let LCV = KZV * ASC;
                                let LCW = KZV * ASE;
                                let LCX = KZW * ASE;
                                let LCY = D - (KZV * ASH);
                                let LCZ = D - (LCW * LCY);
                                let LDA = UC * (D - (LCV * LCZ));
                                let LDB = ((((KZW * ASC) * LCZ) + ((((LCX * LCY) + (((KZW * ASH) * AC) * LCW)) * AC) * LCV)) * AC) * UC;
                                let LDC = (KZW * LDA) + (LDB * KZV);
                                let LDD = LA + (KZV * LDA);
                                let LDE = KZV * ASO;
                                let LDF = KZW * ASO;
                                let LDG = KZV * ASR;
                                let LDH = D - LDE;
                                let LDI = D - (LDG * LDH);
                                let LDJ = UC * (D - (LDE * LDI));
                                let LDK = (((LDF * LDI) + (((((KZW * ASR) * LDH) + ((LDF * AC) * LDG)) * AC) * LDE)) * AC) * UC;
                                let LDL = KZX * LDJ;
                                let LDM = (KZY * LDJ) + (LDK * KZX);
                                let LDN = KZV * ASZ;
                                let LDO = ATB * KZV;
                                let LDP = D - (ATD * KZV);
                                let LDQ = D - (LDO * LDP);
                                let LDR = ATG * (D - (LDN * LDQ));
                                let LDS = KZX * KZX;
                                let LDT = KZY * KZX;
                                let LDU = (KZZ * LDJ) - (LDS * LDR);
                                let LDV = ((LAA * LDJ) + (LDK * KZZ)) - (((LDT + LDT) * LDR) + ((((((KZW * ASZ) * LDQ) + (((((KZW * ATB) * LDP) + (((KZW * ATD) * AC) * LDO)) * AC) * LDN)) * AC) * ATG) * LDS));
                                let LDX = LDW * KZX;
                                let LDY = LDX * LDA;
                                let LDZ = ((KZY * LDW) * LDA) + (LDB * LDX);
                                let LEB = LEA * KZZ;
                                let LED = LEC * KZX;
                                let LEE = LED * KZX;
                                let LEF = LA - (ATV * KZV);
                                let LEG = D - (LCW * LEF);
                                let LEH = (LEB * LDA) + (LEE * LEG);
                                let LEI = (((LAA * LEA) * LDA) + (LDB * LEB)) + (((((KZY * LEC) * KZX) + (KZY * LED)) * LEG) + ((((LCX * LEF) + (((KZW * ATV) * AC) * LCW)) * AC) * LEE));
                                LEJ = KQK;
                                LEK = KQL;
                                LEL = LDD;
                                LEM = LDL;
                                LEN = LDU;
                                LEO = LDY;
                                LEP = LEH;
                                LEQ = KQR;
                                LER = KQS;
                                LES = LDC;
                                LET = LDM;
                                LEU = LDV;
                                LEV = LDZ;
                                LEW = LEI;
                            }
                            LBE = LEJ;
                            LBF = LEK;
                            LBG = LEL;
                            LBH = LEM;
                            LBI = LEN;
                            LBJ = LEO;
                            LBK = LEP;
                            LBL = LEQ;
                            LBM = LER;
                            LBN = LES;
                            LBO = LET;
                            LBP = LEU;
                            LBQ = LEV;
                            LBR = LEW;
                        }
                        let LBS = if KZV > AQJ { 1.0 } else { 0.0 };
                        let LFG;
                        let LFH;
                        let LFI;
                        let LFJ;
                        if LBS != 0.0 {
                            let LEX = LA - LBE;
                            let LEY = D - (LBE * LEX);
                            let LEZ = (XQ * KZV) / LEY;
                            let LFA = ((KZW * XQ) - ((((LBL * LEX) + ((LBL * AC) * LBE)) * AC) * LEZ)) / LEY;
                            let LFB = LEZ * LBE;
                            let LFC = (LFA * LBE) + (LBL * LEZ);
                            let LFD = (LEZ.ln()) - LBF;
                            let LFE = (LFA * (GY / LEZ)) - LBM;
                            LFG = LFB;
                            LFH = LFD;
                            LFI = LFC;
                            LFJ = LFE;
                        } else {
                            let LFF = if KZV < -5e-3f64 { 1.0 } else { 0.0 };
                            let LGB;
                            let LGC;
                            let LGD;
                            let LGE;
                            if LFF != 0.0 {
                                let LFL = GO * LBF;
                                let LFM = LFL.sin();
                                let LFN = LFM * LFM;
                                let LFO = ((LBM * GO) * (LFL.cos())) * LFM;
                                let LFP = (-KZV) / LFN;
                                let LFQ = ((KZW * AC) - ((LFO + LFO) * LFP)) / LFN;
                                let LFR = LFP.ln();
                                let LFS = LFQ * (GY / LFP);
                                LGB = LFP;
                                LGC = LFR;
                                LGD = LFQ;
                                LGE = LFS;
                            } else {
                                let LFT = KZV * WD;
                                let LFU = ATB * KZV;
                                let LFV = D - (AVN * KZV);
                                let LFW = D - (LFU * LFV);
                                let LFX = XQ - (LFT * LFW);
                                let LFY = (((KZW * WD) * LFW) + (((((KZW * ATB) * LFV) + (((KZW * AVN) * AC) * LFU)) * AC) * LFT)) * AC;
                                let LFZ = LFX.ln();
                                let LGA = LFY * (GY / LFX);
                                LGB = LFX;
                                LGC = LFZ;
                                LGD = LFY;
                                LGE = LGA;
                            }
                            LFG = LGB;
                            LFH = LGC;
                            LFI = LGD;
                            LFJ = LGE;
                        }
                        let LFK = if ((AVB * KYX) + LBG) > B { 1.0 } else { 0.0 };
                        let LHA;
                        let LHB;
                        let LHC;
                        let LHD;
                        let LHE;
                        let LHF;
                        if LFK != 0.0 {
                            let LGF = KYX + LBG;
                            let LGG = KYY + LBN;
                            let LGH = JJR + LBH;
                            let LGI = JJT + LBO;
                            LHA = LGF;
                            LHB = LGH;
                            LHC = LBI;
                            LHD = LGG;
                            LHE = LGI;
                            LHF = LBP;
                        } else {
                            let LGJ = KYX - LBG;
                            let LGK = D / LGJ;
                            let LGL = (((KYY - LBN) * LGK) * AC) / LGJ;
                            let LGM = LBH - JJR;
                            let LGN = LBO - JJT;
                            let LGO = KZS - LFG;
                            let LGP = LGO * LGK;
                            let LGQ = ((KZT - LFI) * LGK) + (LGL * LGO);
                            let LGR = ((LGM * LGP) - KZS) - (LBJ * LFG);
                            let LGS = LGR * LGK;
                            let LGT = (((((LGN * LGP) + (LGQ * LGM)) - KZT) - ((LBQ * LFG) + (LFI * LBJ))) * LGK) + (LGL * LGR);
                            let LGU = LA * LGM;
                            let LGV = LBQ * LBJ;
                            let LGW = LBK + (LBJ * LBJ);
                            let LGX = (((LBI * LGP) + (LGU * LGS)) + KZS) - (LGW * LFG);
                            let LGY = LGX * LGK;
                            let LGZ = ((((((LBP * LGP) + (LGQ * LBI)) + (((LGN * LA) * LGS) + (LGT * LGU))) + KZT) - (((LBR + (LGV + LGV)) * LFG) + (LFI * LGW))) * LGK) + (LGL * LGX);
                            LHA = LGP;
                            LHB = LGS;
                            LHC = LGY;
                            LHD = LGQ;
                            LHE = LGT;
                            LHF = LGZ;
                        }
                        let LHG = if LHA > B { 1.0 } else { 0.0 };
                        let LIA;
                        let LIB;
                        let LIC;
                        let LID;
                        let LIE;
                        let LIF;
                        if LHG != 0.0 {
                            let LHH = LHA.ln();
                            let LHI = LHD * (GY / LHA);
                            let LHJ = D / LHA;
                            let LHK = ((LHD * LHJ) * AC) / LHA;
                            let LHL = LHB * LHJ;
                            let LHM = (LHE * LHJ) + (LHK * LHB);
                            let LHN = LHM * LHL;
                            let LHO = (LHC * LHJ) - (LHL * LHL);
                            let LHP = ((LHF * LHJ) + (LHK * LHC)) - (LHN + LHN);
                            LIA = LHH;
                            LIB = LHL;
                            LIC = LHO;
                            LID = LHI;
                            LIE = LHM;
                            LIF = LHP;
                        } else {
                            let LHQ = -KYX;
                            let LHR = (KYX + HW) + (LHQ.ln());
                            let LHS = KYY + ((KYY * AC) * (GY / LHQ));
                            let LHT = D / KYJ;
                            let LHU = ((KYK * LHT) * AC) / KYJ;
                            let LHV = JJR + LHT;
                            let LHW = JJT + LHU;
                            let LHX = -LHT;
                            let LHY = LHX * LHT;
                            let LHZ = ((LHU * AC) * LHT) + (LHU * LHX);
                            LIA = LHR;
                            LIB = LHV;
                            LIC = LHY;
                            LID = LHS;
                            LIE = LHW;
                            LIF = LHZ;
                        }
                        let LIG = ((JYJ + KYJ) + (LA * LIA)) - LFH;
                        let LIH = (D + (LA * LIB)) - LBJ;
                        let LII = (LA * LIC) - LBK;
                        let LIJ = KYX + (JJU * LIG);
                        let LIK = KYY + ((JJW * LIG) + ((((JYK + KYK) + (LID * LA)) - LFJ) * JJU));
                        let LIL = JJR + (JJU * LIH);
                        let LIM = JJT + ((JJW * LIH) + (((LIE * LA) - LBQ) * JJU));
                        let LIN = JJU * LII;
                        let LIO = (LIJ * LHA) - KZS;
                        let LIP = ((LIK * LHA) + (LHD * LIJ)) - KZT;
                        let LIQ = ((LIL * LHA) + (LIJ * LHB)) + KZS;
                        let LIR = (((LIM * LHA) + (LHD * LIL)) + ((LIK * LHB) + (LHE * LIJ))) + KZT;
                        let LIS = LA * LIL;
                        let LIT = (((LIN * LHA) + (LIS * LHB)) + (LIJ * LHC)) - KZS;
                        let LIU = LIR * LIQ;
                        let LIV = GO * LIO;
                        let LIW = (LIQ * LIQ) - (LIV * LIT);
                        let LIX = (LIU + LIU) - (((LIP * GO) * LIT) + ((((((((JJW * LII) + (((LIF * LA) - LBR) * JJU)) * LHA) + (LHD * LIN)) + (((LIM * LA) * LHB) + (LHE * LIS))) + ((LIK * LHC) + (LHF * LIJ))) - KZT) * LIV));
                        let LIY = -LIO;
                        let LIZ = LIY * LIQ;
                        let LJA = LIX * LIW;
                        let LJB = (LIW * LIW) + AYW;
                        let LJC = (LIZ * LIW) / LJB;
                        let LJD = KYJ + LJC;
                        let LJE = KYK + (((((((LIP * AC) * LIQ) + (LIR * LIY)) * LIW) + (LIX * LIZ)) - ((LJA + LJA) * LJC)) / LJB);
                        KZC = LJD;
                        KZD = LBE;
                        KZE = LBF;
                        KZF = LJE;
                        KZG = LBL;
                        KZH = LBM;
                    } else {
                        KZC = KYJ;
                        KZD = KQK;
                        KZE = KQL;
                        KZF = KYK;
                        KZG = KQR;
                        KZH = KQS;
                    }
                    KYM = KZC;
                    KYN = KZD;
                    KYO = KZE;
                    KYP = KZF;
                    KYQ = KZG;
                    KYR = KZH;
                } else {
                    KYM = KYJ;
                    KYN = KQK;
                    KYO = KQL;
                    KYP = KYK;
                    KYQ = KQR;
                    KYR = KQS;
                }
                let KYS = JJR * KYM;
                let KYT = (JJT * KYM) + (KYP * JJR);
                let KYU = JJI - KYM;
                let KYV = JJJ - KYP;
                let KYW = if KYU < SY { 1.0 } else { 0.0 };
                let LJN;
                let LJO;
                if KYW != 0.0 {
                    let LJF = KYU.exp();
                    let LJG = KYV * LJF;
                    LJN = LJF;
                    LJO = LJG;
                } else {
                    let LJH = KYU - SY;
                    let LJI = GO * LJH;
                    let LJJ = D + (LJH * WD);
                    let LJK = D + (LJI * LJJ);
                    let LJL = XB * (D + (LJH * LJK));
                    let LJM = ((KYV * LJK) + ((((KYV * GO) * LJJ) + ((KYV * WD) * LJI)) * LJH)) * XB;
                    LJN = LJL;
                    LJO = LJM;
                }
                let LJP = JKG * LJN;
                let LJQ = (JKH * LJN) + (LJO * JKG);
                let LJR = KYT * KYS;
                let LJS = (KYS * KYS) - LJP;
                let LJT = (LJR + LJR) - LJQ;
                let LJU = if LJP <= B { 1.0 } else { 0.0 };
                let LKA;
                let LKB;
                let LKC;
                let LKD;
                let LKE;
                let LKF;
                if LJU != 0.0 {
                    let LJV = CJQ - KYS;
                    let LJW = KYT * AC;
                    let LJX = LJV / JJU;
                    let LJY = (LJW - (JJW * LJX)) / JJU;
                    LKA = LJX;
                    LKB = CJQ;
                    LKC = LJV;
                    LKD = LJY;
                    LKE = AFD;
                    LKF = LJW;
                } else {
                    let LJZ = if LJS < -5e-3f64 { 1.0 } else { 0.0 };
                    let LKR;
                    let LKS;
                    let LKT;
                    let LKU;
                    let LKV;
                    let LKW;
                    if LJZ != 0.0 {
                        let LKJ = (LJS.abs()).sqrt();
                        let LKK = (LJT * ((GX * (if LJS >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * LKJ));
                        let LKL = GO * LKJ;
                        let LKM = LKL.tan();
                        let LKN = LKL.cos();
                        let LKO = LKJ / LKM;
                        let LKP = (LKK - (((LKK * GO) * (GY / (LKN * LKN))) * LKO)) / LKM;
                        LKR = LKO;
                        LKS = KYN;
                        LKT = LKJ;
                        LKU = LKP;
                        LKV = KYQ;
                        LKW = LKK;
                    } else {
                        let LKQ = if LJS > AQJ { 1.0 } else { 0.0 };
                        let LLM;
                        let LLN;
                        let LLO;
                        let LLP;
                        let LLQ;
                        let LLR;
                        if LKQ != 0.0 {
                            let LKY = (LJS.abs()).sqrt();
                            let LKZ = (LJT * ((GX * (if LJS >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * LKY));
                            let LLA = (-LKY).exp();
                            let LLB = (LKZ * AC) * LLA;
                            let LLC = D + LLA;
                            let LLD = D - LLA;
                            let LLE = (LKY * LLC) / LLD;
                            let LLF = (((LKZ * LLC) + (LLB * LKY)) - ((LLB * AC) * LLE)) / LLD;
                            LLM = LLE;
                            LLN = LLA;
                            LLO = LKY;
                            LLP = LLF;
                            LLQ = LLB;
                            LLR = LKZ;
                        } else {
                            let LLG = LJS * UC;
                            let LLH = LJS * ASC;
                            let LLI = D - (LJS * ASE);
                            let LLJ = D - (LLH * LLI);
                            let LLK = ((LJT * UC) * LLJ) + (((((LJT * ASC) * LLI) + (((LJT * ASE) * AC) * LLH)) * AC) * LLG);
                            let LLL = LA + (LLG * LLJ);
                            LLM = LLL;
                            LLN = KYN;
                            LLO = KYO;
                            LLP = LLK;
                            LLQ = KYQ;
                            LLR = KYR;
                        }
                        LKR = LLM;
                        LKS = LLN;
                        LKT = LLO;
                        LKU = LLP;
                        LKV = LLQ;
                        LKW = LLR;
                    }
                    let LKX = if ((AVB * KYS) + LKR) > B { 1.0 } else { 0.0 };
                    let LLW;
                    let LLX;
                    let LLY;
                    let LLZ;
                    let LMA;
                    let LMB;
                    if LKX != 0.0 {
                        let LLS = KYS + LKR;
                        let LLT = KYT + LKU;
                        let LLU = if (LJP * KYS) < (((CLQ * KYS) * KYS) * LLS) { 1.0 } else { 0.0 };
                        let LMK;
                        let LML;
                        let LMM;
                        let LMN;
                        let LMO;
                        let LMP;
                        if LLU != 0.0 {
                            let LMC = LJP / LLS;
                            let LMD = (LJQ - (LLT * LMC)) / LLS;
                            let LME = LMC + CJQ;
                            let LMF = LME - KYS;
                            let LMG = LMD - KYT;
                            let LMH = LMF / JJU;
                            let LMI = (LMG - (JJW * LMH)) / JJU;
                            LMK = LMH;
                            LML = LME;
                            LMM = LMF;
                            LMN = LMI;
                            LMO = LMD;
                            LMP = LMG;
                        } else {
                            let LMJ = if LJS > AQJ { 1.0 } else { 0.0 };
                            let LMW;
                            let LMX;
                            if LMJ != 0.0 {
                                let LMQ = LA - LKS;
                                let LMR = D - (LKS * LMQ);
                                let LMS = (XQ * LJS) / LMR;
                                let LMT = (LMS.ln()) - LKT;
                                let LMU = ((((LJT * XQ) - ((((LKV * LMQ) + ((LKV * AC) * LKS)) * AC) * LMS)) / LMR) * (GY / LMS)) - LKW;
                                LMW = LMT;
                                LMX = LMU;
                            } else {
                                let LMV = if LJS < -5e-3f64 { 1.0 } else { 0.0 };
                                let LNS;
                                let LNT;
                                if LMV != 0.0 {
                                    let LNE = GO * LKT;
                                    let LNF = LNE.sin();
                                    let LNG = LNF * LNF;
                                    let LNH = ((LKW * GO) * (LNE.cos())) * LNF;
                                    let LNI = (-LJS) / LNG;
                                    let LNJ = LNI.ln();
                                    let LNK = (((LJT * AC) - ((LNH + LNH) * LNI)) / LNG) * (GY / LNI);
                                    LNS = LNJ;
                                    LNT = LNK;
                                } else {
                                    let LNL = LJS * WD;
                                    let LNM = ATB * LJS;
                                    let LNN = D - (AVN * LJS);
                                    let LNO = D - (LNM * LNN);
                                    let LNP = XQ - (LNL * LNO);
                                    let LNQ = LNP.ln();
                                    let LNR = ((((LJT * WD) * LNO) + (((((LJT * ATB) * LNN) + (((LJT * AVN) * AC) * LNM)) * AC) * LNL)) * AC) * (GY / LNP);
                                    LNS = LNQ;
                                    LNT = LNR;
                                }
                                LMW = LNS;
                                LMX = LNT;
                            }
                            let LMY = ((JYJ + KYM) + (LA * (LLS.ln()))) - LMW;
                            let LMZ = ((JYK + KYP) + ((LLT * (GY / LLS)) * LA)) - LMX;
                            let LNA = JJU * LMY;
                            let LNB = (JJW * LMY) + (LMZ * JJU);
                            let LNC = KYS + LNA;
                            let LND = KYT + LNB;
                            LMK = LMY;
                            LML = LNC;
                            LMM = LNA;
                            LMN = LMZ;
                            LMO = LND;
                            LMP = LNB;
                        }
                        LLW = LMK;
                        LLX = LML;
                        LLY = LMM;
                        LLZ = LMN;
                        LMA = LMO;
                        LMB = LMP;
                    } else {
                        let LLV = if LJS > AQJ { 1.0 } else { 0.0 };
                        let LNY;
                        let LNZ;
                        if LLV != 0.0 {
                            let LNU = (KYM - JJI) - LKT;
                            let LNV = (KYP - JJJ) - LKW;
                            let LNW = if LNU < SY { 1.0 } else { 0.0 };
                            let LOQ;
                            let LOR;
                            if LNW != 0.0 {
                                let LOI = LNU.exp();
                                let LOJ = LNV * LOI;
                                LOQ = LOI;
                                LOR = LOJ;
                            } else {
                                let LOK = LNU - SY;
                                let LOL = GO * LOK;
                                let LOM = D + (LOK * WD);
                                let LON = D + (LOL * LOM);
                                let LOO = XB * (D + (LOK * LON));
                                let LOP = ((LNV * LON) + ((((LNV * GO) * LOM) + ((LNV * WD) * LOL)) * LOK)) * XB;
                                LOQ = LOO;
                                LOR = LOP;
                            }
                            let LOS = LOQ / JKG;
                            let LOT = XQ * LJS;
                            let LOU = LA - LKS;
                            let LOV = D - (LKS * LOU);
                            let LOW = (LOT * LOS) / LOV;
                            let LOX = ((((LJT * XQ) * LOS) + (((LOR - (JKH * LOS)) / JKG) * LOT)) - ((((LKV * LOU) + ((LKV * AC) * LKS)) * AC) * LOW)) / LOV;
                            LNY = LOW;
                            LNZ = LOX;
                        } else {
                            let LNX = if LJS < -5e-3f64 { 1.0 } else { 0.0 };
                            let LPL;
                            let LPM;
                            if LNX != 0.0 {
                                let LOY = GO * LKT;
                                let LOZ = LOY.sin();
                                let LPA = LOZ * LOZ;
                                let LPB = ((LKW * GO) * (LOY.cos())) * LOZ;
                                let LPC = (-LJS) / LPA;
                                let LPD = LPC / LJP;
                                let LPE = ((((LJT * AC) - ((LPB + LPB) * LPC)) / LPA) - (LJQ * LPD)) / LJP;
                                LPL = LPD;
                                LPM = LPE;
                            } else {
                                let LPF = LJS * WD;
                                let LPG = ATB * LJS;
                                let LPH = D - (AVN * LJS);
                                let LPI = D - (LPG * LPH);
                                let LPJ = (XQ - (LPF * LPI)) / LJP;
                                let LPK = (((((LJT * WD) * LPI) + (((((LJT * ATB) * LPH) + (((LJT * AVN) * AC) * LPG)) * AC) * LPF)) * AC) - (LJQ * LPJ)) / LJP;
                                LPL = LPJ;
                                LPM = LPK;
                            }
                            LNY = LPL;
                            LNZ = LPM;
                        }
                        let LOA = D - LNY;
                        let LOB = (KYS - LKR) / LOA;
                        let LOC = ((KYT - LKU) - ((LNZ * AC) * LOB)) / LOA;
                        let LOD = LOB + CJQ;
                        let LOE = LOD - KYS;
                        let LOF = LOC - KYT;
                        let LOG = LOE / JJU;
                        let LOH = (LOF - (JJW * LOG)) / JJU;
                        LLW = LOG;
                        LLX = LOD;
                        LLY = LOE;
                        LLZ = LOH;
                        LMA = LOC;
                        LMB = LOF;
                    }
                    LKA = LLW;
                    LKB = LLX;
                    LKC = LLY;
                    LKD = LLZ;
                    LKE = LMA;
                    LKF = LMB;
                }
                let LKG = JJP - LKA;
                let LKH = JJQ - LKD;
                let LKI = if LKG < SY { 1.0 } else { 0.0 };
                let LPV;
                let LPW;
                if LKI != 0.0 {
                    let LPN = LKG.exp();
                    let LPO = LKH * LPN;
                    LPV = LPN;
                    LPW = LPO;
                } else {
                    let LPP = LKG - SY;
                    let LPQ = GO * LPP;
                    let LPR = D + (LPP * WD);
                    let LPS = D + (LPQ * LPR);
                    let LPT = XB * (D + (LPP * LPS));
                    let LPU = ((LKH * LPS) + ((((LKH * GO) * LPR) + ((LKH * WD) * LPQ)) * LPP)) * XB;
                    LPV = LPT;
                    LPW = LPU;
                }
                let LPX = JKG * LPV;
                let LPY = (JKH * LPV) + (LPW * JKG);
                let LPZ = if LKB > CPW { 1.0 } else { 0.0 };
                let LQL;
                let LQM;
                let LQN;
                let LQO;
                let LQP;
                let LQQ;
                let LQR;
                let LQS;
                if LPZ != 0.0 {
                    let LQA = LJP * JJX;
                    let LQB = (LJQ * JJX) + (JJY * LJP);
                    let LQC = LPX * JJZ;
                    let LQD = (LPY * JJZ) + (JKA * LPX);
                    let LQE = LQA + (LA * KYS);
                    let LQF = LQB + (KYT * LA);
                    let LQG = LQC + (LA * LKC);
                    let LQH = LQD + (LKF * LA);
                    let LQI = ((LA * LKB) + LQA) + LQC;
                    let LQJ = ((LKE * LA) + LQB) + LQD;
                    let LQK = if (LJS.abs()) > AQJ { 1.0 } else { 0.0 };
                    let LRU;
                    let LRV;
                    if LQK != 0.0 {
                        let LQY = LA * (KYM + LA);
                        let LQZ = LA * (LKA + LA);
                        let LRA = ((LQE * LQG) + (LQY * LQG)) + (LQZ * LQE);
                        let LRC = LRB * LJS;
                        let LRD = LKB * LRA;
                        let LRE = (LRC * LQI) / LRD;
                        let LRF = ((((LJT * LRB) * LQI) + (LQJ * LRC)) - (((LKE * LRA) + (((((LQF * LQG) + (LQH * LQE)) + (((KYP * LA) * LQG) + (LQH * LQY))) + (((LKD * LA) * LQE) + (LQF * LQZ))) * LKB)) * LRE)) / LRD;
                        LRU = LRE;
                        LRV = LRF;
                    } else {
                        let LRG = LJS * ASO;
                        let LRH = LJT * ASO;
                        let LRI = LJS * ASR;
                        let LRJ = D - LRG;
                        let LRK = D - (LRI * LRJ);
                        let LRL = UC * (D - (LRG * LRK));
                        let LRM = LQE * LQG;
                        let LRN = LRM * LKB;
                        let LRO = D + (LKB * LRL);
                        let LRP = ((LQE * LJP) + (LQG * LPX)) + (LRN * LRO);
                        let LRQ = LJP * LPX;
                        let LRR = LKB * LRP;
                        let LRS = (LRQ * LQI) / LRR;
                        let LRT = (((((LJQ * LPX) + (LPY * LJP)) * LQI) + (LQJ * LRQ)) - (((LKE * LRP) + (((((LQF * LJP) + (LJQ * LQE)) + ((LQH * LPX) + (LPY * LQG))) + ((((((LQF * LQG) + (LQH * LQE)) * LKB) + (LKE * LRM)) * LRO) + (((LKE * LRL) + (((((LRH * LRK) + (((((LJT * ASR) * LRJ) + ((LRH * AC) * LRI)) * AC) * LRG)) * AC) * UC) * LKB)) * LRN))) * LKB)) * LRS)) / LRR;
                        LRU = LRS;
                        LRV = LRT;
                    }
                    LQL = LRU;
                    LQM = LQI;
                    LQN = LQE;
                    LQO = LQG;
                    LQP = LRV;
                    LQQ = LQJ;
                    LQR = LQF;
                    LQS = LQH;
                } else {
                    LQL = B;
                    LQM = B;
                    LQN = B;
                    LQO = B;
                    LQP = AFD;
                    LQQ = AFD;
                    LQR = AFD;
                    LQS = AFD;
                }
                let LQT = LKB.ln();
                let LQU = LKE * (GY / LKB);
                let LQV = KYS / LA;
                let LQW = KYT / LA;
                let LQX = if LQV < SY { 1.0 } else { 0.0 };
                let LSA;
                let LSB;
                if LQX != 0.0 {
                    let LRW = LQV.exp();
                    let LRX = D + LRW;
                    let LRY = LRX.ln();
                    let LRZ = (LQW * LRW) * (GY / LRX);
                    LSA = LRY;
                    LSB = LRZ;
                } else {
                    LSA = LQV;
                    LSB = LQW;
                }
                let LSC = LA * LSA;
                let LSD = LSB * LA;
                let LSE = LKC / LA;
                let LSF = LKF / LA;
                let LSG = if LSE < SY { 1.0 } else { 0.0 };
                let LSL;
                let LSM;
                if LSG != 0.0 {
                    let LSH = LSE.exp();
                    let LSI = D + LSH;
                    let LSJ = LSI.ln();
                    let LSK = (LSF * LSH) * (GY / LSI);
                    LSL = LSJ;
                    LSM = LSK;
                } else {
                    LSL = LSE;
                    LSM = LSF;
                }
                let LSN = LA * LSL;
                let LSO = LSM * LA;
                let LSP = LSN - LKC;
                let LSQ = LSO - LKF;
                let LSR = LSC - KYS;
                let LSS = LSD - KYT;
                let LST = (CSR * LSC) + (CSS * LSP);
                let LSU = (LSD * CSR) + (LSQ * CSS);
                let LSV = (CSR * LSN) + (CSS * LSR);
                let LSW = (LSO * CSR) + (LSS * CSS);
                let LSX = LSC + LSN;
                let LSY = LKB / LSX;
                let LSZ = (LKE - ((LSD + LSO) * LSY)) / LSX;
                let LTA = LSC * JO;
                let LTB = LTA * CTB;
                let LTC = ((LSD * JO) * CTB) + Lanes([(CTC * LTA), 0.0, 0.0, 0.0, 0.0]);
                let LTD = LSN * JR;
                let LTE = LTD * CTB;
                let LTF = ((LSO * JR) * CTB) + Lanes([(CTC * LTD), 0.0, 0.0, 0.0, 0.0]);
                let LTG = LSP + (CTI * LSR);
                let LTH = CU * LTG;
                let LTI = Lanes([(ES * LTG), 0.0, 0.0, 0.0, 0.0]) + ((LSQ + (LSS * CTI)) * CU);
                let LTJ = D + LTH;
                let LTK = LTI * LTJ;
                let LTL = ((LTJ * LTJ) + NI).sqrt();
                let LTM = LTI * CTP;
                let LTN = D + (CTP * LTH);
                let LTO = LTM * LTN;
                let LTP = ((LTN * LTN) + NI).sqrt();
                let LTQ = GO * (LTN + LTP);
                let LTR = (GO * (LTJ + LTL)) / LTQ;
                let LTS = (((LTI + ((LTK + LTK) * (GY / (GX * LTL)))) * GO) - (((LTM + ((LTO + LTO) * (GY / (GX * LTP)))) * GO) * LTR)) / LTQ;
                let LTT = (D + (CTX * LSP)) + (CTY * LSR);
                let LTU = CV * LTT;
                let LTV = (D + ((LSC * LSY) * CUD)) + ((LSN * LSY) * CUE);
                let LTW = LTV.ln();
                let LTX = (CUB * LTW).exp();
                let LTY = LTU * LTX;
                let LTZ = ((Lanes([(ET * LTT), 0.0, 0.0, 0.0, 0.0]) + (((LSQ * CTX) + (LSS * CTY)) * CV)) * LTX) + (((Lanes([(CUC * LTW), 0.0, 0.0, 0.0, 0.0]) + ((((((LSD * LSY) + (LSZ * LSC)) * CUD) + (((LSO * LSY) + (LSZ * LSN)) * CUE)) * (GY / LTV)) * CUB)) * LTX) * LTU);
                let LUB;
                let LUC;
                if CUK != 0.0 {
                    LUB = D;
                    LUC = AFD;
                } else {
                    let LVN;
                    let LVO;
                    if LUA != 0.0 {
                        let LVE = LKB + CVS;
                        let LVF = (CVU * (LVE.ln())).exp();
                        let LVG = D - (CVW * LVF);
                        let LVH = ((((LKE * (GY / LVE)) * CVU) * LVF) * CVW) * AC;
                        LVN = LVG;
                        LVO = LVH;
                    } else {
                        let LVI = LKB + CVS;
                        let LVJ = (CVU * (LVI.ln())).exp();
                        let LVK = D + (CVW * LVJ);
                        let LVL = D / LVK;
                        let LVM = ((((((LKE * (GY / LVI)) * CVU) * LVJ) * CVW) * LVL) * AC) / LVK;
                        LVN = LVL;
                        LVO = LVM;
                    }
                    LUB = LVN;
                    LUC = LVO;
                }
                let LUD = (CX * JGV) * GO;
                let LUE = D - (CUP * IRZ);
                let LUF = (ISA * CUP) * AC;
                let LUG = LUF * LUE;
                let LUH = ((LUE * LUE) + NI).sqrt();
                let LUI = LUE + LUH;
                let LUJ = LUD * LUI;
                let LUK = (LUF + ((LUG + LUG) * (GY / (GX * LUH)))) * LUD;
                let LUL = (((Lanes([(EV * JGV), 0.0, 0.0, 0.0, 0.0]) + (JHB * CX)) * GO) * LUI) + Lanes([LUK[0], LUK[1], LUK[2], LUK[3], 0.0]);
                let LUM = (LKB * LUB) + CUY;
                let LUN = LUJ * LUM;
                let LUO = (LUL * LUM) + (((LKE * LUB) + (LUC * LKB)) * LUJ);
                let LUP = (CZ * LST) + CPW;
                let LUQ = LUP.ln();
                let LUR = (CY * LUQ).exp();
                let LUS = ((D + LUR) + LTY) + (DA * LUN);
                let LUT = (CZ * LSV) + CPW;
                let LUU = LUT.ln();
                let LUV = (CY * LUU).exp();
                let LUW = ((D + LUV) + LTY) + (DB * LUN);
                let LUX = LTB + LTE;
                let LUY = LTB / LUS;
                let LUZ = LTE / LUW;
                let LVA = LUY + LUZ;
                let LVB = (LTR * LUX) / LVA;
                let LVC = (((LTS * LUX) + ((LTC + LTF) * LTR)) - ((((LTC - (((((Lanes([(EW * LUQ), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * LST), 0.0, 0.0, 0.0, 0.0]) + (LSU * CZ)) * (GY / LUP)) * CY)) * LUR) + LTZ) + (Lanes([(EY * LUN), 0.0, 0.0, 0.0, 0.0]) + (LUO * DA))) * LUY)) / LUS) + ((LTF - (((((Lanes([(EW * LUU), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * LSV), 0.0, 0.0, 0.0, 0.0]) + (LSW * CZ)) * (GY / LUT)) * CY)) * LUV) + LTZ) + (Lanes([(EZ * LUN), 0.0, 0.0, 0.0, 0.0]) + (LUO * DB))) * LUZ)) / LUW)) * LVB)) / LVA;
                let LVD = if (JLR.abs()) > CVQ { 1.0 } else { 0.0 };
                let LWW;
                let LWX;
                let LWY;
                let LWZ;
                let LXA;
                let LXB;
                let LXC;
                let LXD;
                let LXE;
                let LXF;
                let LXG;
                let LXH;
                if LVD != 0.0 {
                    let LVP = if JLR > B { 1.0 } else { 0.0 };
                    let LYG;
                    let LYH;
                    let LYI;
                    let LYJ;
                    let LYK;
                    let LYL;
                    if LVP != 0.0 {
                        let LXK = (-JLR).exp();
                        let LXL = (JLS * AC) * LXK;
                        let LXM = D - LXK;
                        let LXN = JLR / LXM;
                        let LXO = (JLS - ((LXL * AC) * LXN)) / LXM;
                        let LXP = LXK * LXN;
                        let LXQ = (LXL * LXN) + (LXO * LXK);
                        let LXR = LKB * LXN;
                        let LXS = JKG / LXR;
                        let LXT = ((LXS.ln()) - HW) + JLY;
                        let LXU = (((JKH - (((LKE * LXN) + (LXO * LKB)) * LXS)) / LXR) * (GY / LXS)) + JLZ;
                        LYG = LXN;
                        LYH = LXP;
                        LYI = LXT;
                        LYJ = LXO;
                        LYK = LXQ;
                        LYL = LXU;
                    } else {
                        let LXV = JLR.exp();
                        let LXW = LXV - D;
                        let LXX = JLR / LXW;
                        let LXY = (JLS * LXV) * LXX;
                        let LXZ = (JLS - LXY) / LXW;
                        let LYA = LXV * LXX;
                        let LYB = LXY + (LXZ * LXV);
                        let LYC = LKB * LXX;
                        let LYD = JKG / LYC;
                        let LYE = ((LYD.ln()) - HW) + JMC;
                        let LYF = (((JKH - (((LKE * LXX) + (LXZ * LKB)) * LYD)) / LYC) * (GY / LYD)) + JMD;
                        LYG = LYA;
                        LYH = LXX;
                        LYI = LYE;
                        LYJ = LYB;
                        LYK = LXZ;
                        LYL = LYF;
                    }
                    let LYM = (D - LYG) - JMA;
                    let LYN = JKC * LYM;
                    let LYO = (-JLR) / LYN;
                    let LYP = ((JLS * AC) - (((JKD * LYM) + (((LYJ * AC) - JMB) * JKC)) * LYO)) / LYN;
                    let LYQ = (D - LYH) + JLW;
                    let LYR = JKC * LYQ;
                    let LYS = JLR / LYR;
                    let LYT = (JLS - (((JKD * LYQ) + (((LYK * AC) + JLX) * JKC)) * LYS)) / LYR;
                    let LYU = ((LYH * JJZ) + GO) / LYS;
                    let LYV = ((LYG * JJX) + GO) / LYO;
                    let LYW = LYU - LYV;
                    let LYX = JLR / LYW;
                    let LYY = (JLS - ((((((LYK * JJZ) + (JKA * LYH)) - (LYT * LYU)) / LYS) - ((((LYJ * JJX) + (JJY * LYG)) - (LYP * LYV)) / LYO)) * LYX)) / LYW;
                    LWW = LYX;
                    LWX = LYI;
                    LWY = LYG;
                    LWZ = LYO;
                    LXA = LYH;
                    LXB = LYS;
                    LXC = LYY;
                    LXD = LYL;
                    LXE = LYJ;
                    LXF = LYP;
                    LXG = LYK;
                    LXH = LYT;
                } else {
                    let LVR = LVQ * JLT;
                    let LVS = JLV * LVQ;
                    let LVT = GO * JLR;
                    let LVU = JLS * GO;
                    let LVV = (D + LVT) + LVR;
                    let LVW = LVU + LVS;
                    let LVX = (D - LVT) + LVR;
                    let LVY = (LVU * AC) + LVS;
                    let LVZ = UC * LVT;
                    let LWA = LVU * UC;
                    let LWB = (GO + JJZ) + LVZ;
                    let LWC = JKC * LWB;
                    let LWD = D / LWC;
                    let LWE = ((((JKD * LWB) + ((JKA + LWA) * JKC)) * LWD) * AC) / LWC;
                    let LWF = (GO + JJX) - LVZ;
                    let LWG = JKC * LWF;
                    let LWH = D / LWG;
                    let LWI = ((((JKD * LWF) + ((JJY - LWA) * JKC)) * LWH) * AC) / LWG;
                    let LWJ = D - (GO * LVR);
                    let LWK = LKB * LWJ;
                    let LWL = JKG / LWK;
                    let LWM = ((LWL.ln()) - HW) + (GO * (JLY + JMC));
                    let LWN = (((JKH - (((LKE * LWJ) + (((LVS * GO) * AC) * LKB)) * LWL)) / LWK) * (GY / LWL)) + ((JLZ + JMD) * GO);
                    let LWO = JJR * JJU;
                    let LWP = (XS * JKC) / LWO;
                    let LWQ = JJX - JJZ;
                    let LWR = JKC * LWQ;
                    let LWS = WD * (CTP - (YY * JKC));
                    let LWT = (((XQ - (ZD * JKC)) + LWP) + (LWR * JLR)) + (LWS * JLT);
                    let LWU = -1.2e1f64 / LWT;
                    let LWV = (((((((JKD * ZD) * AC) + (((JKD * XS) - (((JJT * JJU) + (JJW * JJR)) * LWP)) / LWO)) + ((((JKD * LWQ) + ((JJY - JKA) * JKC)) * JLR) + (JLS * LWR))) + (((((JKD * YY) * AC) * WD) * JLT) + (JLV * LWS))) * LWU) * AC) / LWT;
                    LWW = LWU;
                    LWX = LWM;
                    LWY = LVV;
                    LWZ = LWD;
                    LXA = LVX;
                    LXB = LWH;
                    LXC = LWV;
                    LXD = LWN;
                    LXE = LVW;
                    LXF = LWE;
                    LXG = LVY;
                    LXH = LWI;
                }
                let LXI = D / LWW;
                let LXJ = ((LXC * LXI) * AC) / LWW;
                let LZH;
                let LZI;
                let LZJ;
                let LZK;
                let LZL;
                let LZM;
                if LPZ != 0.0 {
                    let LYZ = CZQ + LSC;
                    let LZA = (CZQ * LSC) / LYZ;
                    let LZB = ((LSD * CZQ) - (LSD * LZA)) / LYZ;
                    let LZU;
                    let LZV;
                    if FRC != 0.0 {
                        let LZP = D - (CZU * LZA);
                        let LZQ = D / LZP;
                        let LZR = ((((LZB * CZU) * AC) * LZQ) * AC) / LZP;
                        LZU = LZQ;
                        LZV = LZR;
                    } else {
                        let LZS = LZB * CZU;
                        let LZT = D + (CZU * LZA);
                        LZU = LZT;
                        LZV = LZS;
                    }
                    let LZW = CZQ + LSN;
                    let LZX = (CZQ * LSN) / LZW;
                    let LZY = ((LSO * CZQ) - (LSO * LZX)) / LZW;
                    let MAE;
                    let MAF;
                    if FRN != 0.0 {
                        let LZZ = D - (DAU * LZX);
                        let MAA = D / LZZ;
                        let MAB = ((((LZY * DAU) * AC) * MAA) * AC) / LZZ;
                        MAE = MAA;
                        MAF = MAB;
                    } else {
                        let MAC = LZY * DAU;
                        let MAD = D + (DAU * LZX);
                        MAE = MAD;
                        MAF = MAC;
                    }
                    let MAG = LQN * LQO;
                    let MAH = (LQL * LQM) / MAG;
                    let MAI = LJP / LQN;
                    let MAJ = LPX / LQO;
                    let MAK = (MAI + MAJ) / LKB;
                    let MAL = MAH - MAK;
                    let MAM = ((((LQP * LQM) + (LQQ * LQL)) - (((LQR * LQO) + (LQS * LQN)) * MAH)) / MAG) - (((((LJQ - (LQR * MAI)) / LQN) + ((LPY - (LQS * MAJ)) / LQO)) - (LKE * MAK)) / LKB);
                    let MAN = MAL + D;
                    let MAO = (MAL * LKB) / MAN;
                    let MAP = (((MAM * LKB) + (LKE * MAL)) - (MAM * MAO)) / MAN;
                    let MAQ = LWW - MAO;
                    let MAR = LXC - MAP;
                    let MAS = (LKB + (LWW * LWX)) / MAQ;
                    let MAT = ((LKE + ((LXC * LWX) + (LXD * LWW))) - (MAR * MAS)) / MAQ;
                    let MAU = MAT * MAS;
                    let MAV = ((MAS * MAS) + CPW).sqrt();
                    let MAW = GO * (MAS + MAV);
                    let MAX = EA / LVB;
                    let MAY = MAX * GO;
                    let MAZ = LZU + MAE;
                    let MBA = MAY * MAZ;
                    let MBB = ((((Lanes([FY, 0.0, 0.0, 0.0, 0.0]) - (LVC * MAX)) / LVB) * GO) * MAZ) + ((LZV + MAF) * MAY);
                    let MBC = LKB / MAO;
                    let MBD = D - MBC;
                    let MBE = ((LKE - (MAP * MBC)) / MAO) * AC;
                    let MBF = D + LWX;
                    let MBG = (LA * MAO) - LKB;
                    let MBH = ((MBG * LXI) - LA) - LWX;
                    let MBI = MBH * MAW;
                    let MBJ = ((((((MAP * LA) - LKE) * LXI) + (LXJ * MBG)) - LXD) * MAW) + (((MAT + ((MAU + MAU) * (GY / (GX * MAV)))) * GO) * MBH);
                    let MBK = if MBA > DCH { 1.0 } else { 0.0 };
                    let MCR;
                    let MCS;
                    let MCT;
                    let MCU;
                    if MBK != 0.0 {
                        let MBL = MBA * MBA;
                        let MBM = MBB * MBA;
                        let MBN = LA / MBL;
                        let MBO = (((MBM + MBM) * MBN) * AC) / MBL;
                        let MBP = MBN * MBD;
                        let MBQ = (MBO * MBD) + (MBE * MBN);
                        let MBR = MBN + MBI;
                        let MBS = MBO + MBJ;
                        let MBT = MBN * MBF;
                        let MBU = (MBO * MBF) + (LXD * MBN);
                        let MBV = MBQ * MBP;
                        let MBW = DCU * MBN;
                        let MBX = MBW * MBN;
                        let MBY = (((MBP * MBP) + (MBX * MBN)) + DCX).sqrt();
                        let MBZ = ((MBV + MBV) + (((((MBO * DCU) * MBN) + (MBO * MBW)) * MBN) + (MBO * MBX))) * (GY / (GX * MBY));
                        let MCA = MBU * MBT;
                        let MCB = DCU * MBR;
                        let MCC = MCB * MBR;
                        let MCD = (((MBT * MBT) + (MCC * MBR)) + DCX).sqrt();
                        let MCE = ((MCA + MCA) + (((((MBS * DCU) * MBR) + (MBS * MCB)) * MBR) + (MBS * MCC))) * (GY / (GX * MCD));
                        let MCF = GO * (MBY + MBP);
                        let MCG = (WD * (MCF.ln())).exp();
                        let MCH = GO * (MBY - MBP);
                        let MCI = (WD * (MCH.ln())).exp();
                        let MCJ = MCG - MCI;
                        let MCK = (((((MBZ + MBQ) * GO) * (GY / MCF)) * WD) * MCG) - (((((MBZ - MBQ) * GO) * (GY / MCH)) * WD) * MCI);
                        let MCL = GO * (MCD + MBT);
                        let MCM = (WD * (MCL.ln())).exp();
                        let MCN = GO * (MCD - MBT);
                        let MCO = (WD * (MCN.ln())).exp();
                        let MCP = MCM - MCO;
                        let MCQ = (((((MCE + MBU) * GO) * (GY / MCL)) * WD) * MCM) - (((((MCE - MBU) * GO) * (GY / MCN)) * WD) * MCO);
                        MCR = MCJ;
                        MCS = MCP;
                        MCT = MCK;
                        MCU = MCQ;
                    } else {
                        MCR = MBD;
                        MCS = MBF;
                        MCT = MBE;
                        MCU = LXD;
                    }
                    let MCV = MAQ * MAQ;
                    let MCW = MAR * MAQ;
                    let MCX = MCW + MCW;
                    let MCY = MCR - MCS;
                    let MCZ = (MCT - MCU) * MCY;
                    let MDA = ((MCY * MCY) + (UW * MCV)).sqrt();
                    let MDC = MDB * ((MCR + MCS) + MDA);
                    let MDD = ((MCT + MCU) + (((MCZ + MCZ) + (MCX * UW)) * (GY / (GX * MDA)))) * MDB;
                    let MDE = LKB + (MAO * MDC);
                    let MDF = LKE + ((MAP * MDC) + (MDD * MAO));
                    let MDG = MDC - LWX;
                    let MDH = LWW * MDG;
                    let MDI = (LXC * MDG) + ((MDD - LXD) * LWW);
                    let MDJ = MDE - MDH;
                    let MDK = (MDF - MDI) * MDJ;
                    let MDL = ((MDJ * MDJ) + (DEL * MCV)).sqrt();
                    let MDM = GO * ((MDE + MDH) + MDL);
                    let MDN = ((MDF + MDI) + (((MDK + MDK) + (MCX * DEL)) * (GY / (GX * MDL)))) * GO;
                    LZH = MDM;
                    LZI = MDC;
                    LZJ = MAO;
                    LZK = MDN;
                    LZL = MDD;
                    LZM = MAP;
                } else {
                    let LZC = CZW * (D + LWX);
                    let LZD = LXD * CZW;
                    let LZE = LZC - (GO * LWX);
                    let LZF = (GO * LKB) + (LWW * LZE);
                    let LZG = (LKE * GO) + ((LXC * LZE) + ((LZD - (LXD * GO)) * LWW));
                    LZH = LZF;
                    LZI = LZC;
                    LZJ = LWW;
                    LZK = LZG;
                    LZL = LZD;
                    LZM = LXC;
                }
                let LZN = LZH - GO;
                let LZO = if LZN < SY { 1.0 } else { 0.0 };
                let MDS;
                let MDT;
                if LZO != 0.0 {
                    let MDO = LZN.exp();
                    let MDP = D + MDO;
                    let MDQ = MDP.ln();
                    let MDR = (LZK * MDO) * (GY / MDP);
                    MDS = MDQ;
                    MDT = MDR;
                } else {
                    MDS = LZN;
                    MDT = LZK;
                }
                let MDU = MDS + GO;
                let MDV = LKB / MDU;
                let MDW = LZL + (((LKE - (MDT * MDV)) / MDU) * (GY / MDV));
                let MDX = (LZI + (MDV.ln())) - UX;
                let MDY = if MDX < SY { 1.0 } else { 0.0 };
                let MED;
                let MEE;
                if MDY != 0.0 {
                    let MDZ = MDX.exp();
                    let MEA = D + MDZ;
                    let MEB = MEA.ln();
                    let MEC = (MDW * MDZ) * (GY / MEA);
                    MED = MEB;
                    MEE = MEC;
                } else {
                    MED = MDX;
                    MEE = MDW;
                }
                let MEF = AID - (MED + UX);
                let MEG = MEE * AC;
                let MEH = if MEF < SY { 1.0 } else { 0.0 };
                let MEM;
                let MEN;
                if MEH != 0.0 {
                    let MEI = MEF.exp();
                    let MEJ = D + MEI;
                    let MEK = MEJ.ln();
                    let MEL = (MEG * MEI) * (GY / MEJ);
                    MEM = MEK;
                    MEN = MEL;
                } else {
                    MEM = MEF;
                    MEN = MEG;
                }
                let MEO = AID - MEM;
                let MEP = RA / MEO;
                let MEQ = MEP * MEP;
                let MER = ((DFR - ((MEN * AC) * MEP)) / MEO) * MEP;
                let MES = MEQ * MEQ;
                let MET = (MER + MER) * MEQ;
                let MEU = MET + MET;
                let MEV = MES * MES;
                let MEW = MEU * MES;
                let MEY = D + (MEX * MES);
                let MEZ = (DGB * (MEY.ln())).exp();
                let MFA = (MEW + MEW) * MEV;
                let MFB = MEZ + (MEV * MEV);
                let MFD = (MFC * (MFB.ln())).exp();
                let MFE = RA * MFD;
                let MFF = RC * MFD;
                let MFG = Lanes([MFF[0], MFF[1], MFF[2], 0.0, 0.0]) + (((((((((MEU * MEX) * (GY / MEY)) * DGB) * MEZ) + (MFA + MFA)) * (GY / MFB)) * MFC) * MFD) * RA);
                let MFH = (((((JML * JLO) + JMM) - JMO) / JKG) * JMQ) + MFG;
                let MFI = (JMP + MFE) + ZD;
                let MFJ = (((((JMU * JLO) + JMV) - JMX) / JKG) * JMZ) + MFG;
                let MFK = (JMY + MFE) + ZD;
                let MFL = (MFI - JLY) * WD;
                let MFM = (MFH - JLZ) * WD;
                let MFN = if MFL < SY { 1.0 } else { 0.0 };
                let MFS;
                let MFT;
                if MFN != 0.0 {
                    let MFO = MFL.exp();
                    let MFP = D + MFO;
                    let MFQ = MFP.ln();
                    let MFR = (MFM * MFO) * (GY / MFP);
                    MFS = MFQ;
                    MFT = MFR;
                } else {
                    MFS = MFL;
                    MFT = MFM;
                }
                let MFU = MFI - (ZD * MFS);
                let MFV = MFH - (MFT * ZD);
                let MFW = (MFK - JMC) * WD;
                let MFX = (MFJ - JMD) * WD;
                let MFY = if MFW < SY { 1.0 } else { 0.0 };
                let MGD;
                let MGE;
                if MFY != 0.0 {
                    let MFZ = MFW.exp();
                    let MGA = D + MFZ;
                    let MGB = MGA.ln();
                    let MGC = (MFX * MFZ) * (GY / MGA);
                    MGD = MGB;
                    MGE = MGC;
                } else {
                    MGD = MFW;
                    MGE = MFX;
                }
                let MGF = JNW + (MFK - (ZD * MGD));
                let MGG = JNZ + MFU;
                let MGH = MGG * JMH;
                let MGI = ((JOA + MFV) * JMH) + (JMJ * MGG);
                let MGJ = (MFI - (MGF * JME)) * WD;
                let MGK = (MFH - (((JNX + (MFJ - (MGE * ZD))) * JME) + (JMG * MGF))) * WD;
                let MGL = if MGJ < SY { 1.0 } else { 0.0 };
                let MGQ;
                let MGR;
                if MGL != 0.0 {
                    let MGM = MGJ.exp();
                    let MGN = D + MGM;
                    let MGO = MGN.ln();
                    let MGP = (MGK * MGM) * (GY / MGN);
                    MGQ = MGO;
                    MGR = MGP;
                } else {
                    MGQ = MGJ;
                    MGR = MGK;
                }
                let MGS = MFI - (ZD * MGQ);
                let MGT = MFH - (MGR * ZD);
                let MGU = (MFK - MGH) * WD;
                let MGV = (MFJ - MGI) * WD;
                let MGW = if MGU < SY { 1.0 } else { 0.0 };
                let MHB;
                let MHC;
                if MGW != 0.0 {
                    let MGX = MGU.exp();
                    let MGY = D + MGX;
                    let MGZ = MGY.ln();
                    let MHA = (MGV * MGX) * (GY / MGY);
                    MHB = MGZ;
                    MHC = MHA;
                } else {
                    MHB = MGU;
                    MHC = MGV;
                }
                let MHD = JJI - MGS;
                let MHE = JJJ - MGT;
                let MHF = JJP - (MFK - (ZD * MHB));
                let MHG = JJQ - (MFJ - (MHC * ZD));
                let MHH = JJR * MHD;
                let MHI = (JJT * MHD) + (MHE * JJR);
                let MHJ = (JJI - MHD) - MFE;
                let MHK = (JJJ - MHE) - MFG;
                let MHL = if MHJ < SY { 1.0 } else { 0.0 };
                let MHU;
                let MHV;
                if MHL != 0.0 {
                    let MHM = MHJ.exp();
                    let MHN = MHK * MHM;
                    MHU = MHM;
                    MHV = MHN;
                } else {
                    let MHO = MHJ - SY;
                    let MHP = GO * MHO;
                    let MHQ = D + (MHO * WD);
                    let MHR = D + (MHP * MHQ);
                    let MHS = XB * (D + (MHO * MHR));
                    let MHT = ((MHK * MHR) + ((((MHK * GO) * MHQ) + ((MHK * WD) * MHP)) * MHO)) * XB;
                    MHU = MHS;
                    MHV = MHT;
                }
                let MHW = JKG * MHU;
                let MHX = (JKH * MHU) + (MHV * JKG);
                let MHY = MHI * MHH;
                let MHZ = (MHH * MHH) - MHW;
                let MIA = (MHY + MHY) - MHX;
                let MIB = (JPW * MHH) + MHW;
                let MIC = ((JPX * MHH) + (MHI * JPW)) + MHX;
                let MID = JQA - MHW;
                let MIE = JQB - MHX;
                let MIF = if MHZ < -5e-3f64 { 1.0 } else { 0.0 };
                let MJI;
                let MJJ;
                let MJK;
                let MJL;
                let MJM;
                let MJN;
                let MJO;
                let MJP;
                let MJQ;
                let MJR;
                let MJS;
                let MJT;
                let MJU;
                let MJV;
                if MIF != 0.0 {
                    let MIG = (MHZ.abs()).sqrt();
                    let MIH = (MIA * ((GX * (if MHZ >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * MIG));
                    let MII = GO * MIG;
                    let MIJ = MII.tan();
                    let MIK = MII.cos();
                    let MIL = MIG / MIJ;
                    let MIM = (MIH - (((MIH * GO) * (GY / (MIK * MIK))) * MIL)) / MIJ;
                    let MIN = (YY * MIB) / MHZ;
                    let MIO = ((MIC * YY) - (MIA * MIN)) / MHZ;
                    let MIP = LA - MIL;
                    let MIQ = MHZ + (MIL * MIP);
                    let MIR = MIQ * MIN;
                    let MIS = ((MIA + ((MIM * MIP) + ((MIM * AC) * MIL))) * MIN) + (MIO * MIQ);
                    let MIT = LA * MIR;
                    let MIU = D + MIL;
                    let MIV = MIB - (MIT * MIU);
                    let MIW = (MIR * MID) / MIB;
                    let MIX = (MIV * MIN) + MIW;
                    let MIY = (((MIC - (((MIS * LA) * MIU) + (MIM * MIT))) * MIN) + (MIO * MIV)) + ((((MIS * MID) + (MIE * MIR)) - (MIC * MIW)) / MIB);
                    let MIZ = D - (GO * MIL);
                    let MJA = (MIM * GO) * AC;
                    let MJB = MIB / MHZ;
                    let MJC = MJB * MIZ;
                    let MJD = (((MIC - (MIA * MJB)) / MHZ) * MIZ) + (MJA * MJB);
                    let MJE = MJC + (GO * MIR);
                    let MJF = ((MID * MIZ) - (MIB * MJE)) / MHZ;
                    let MJG = ((((MIE * MIZ) + (MJA * MID)) - ((MIC * MJE) + ((MJD + (MIS * GO)) * MIB))) - (MIA * MJF)) / MHZ;
                    MJI = B;
                    MJJ = MIG;
                    MJK = MIL;
                    MJL = MIR;
                    MJM = MIX;
                    MJN = MJC;
                    MJO = MJF;
                    MJP = AFD;
                    MJQ = MIH;
                    MJR = MIM;
                    MJS = MIS;
                    MJT = MIY;
                    MJU = MJD;
                    MJV = MJG;
                } else {
                    let MJH = if MHZ > AQJ { 1.0 } else { 0.0 };
                    let MMN;
                    let MMO;
                    let MMP;
                    let MMQ;
                    let MMR;
                    let MMS;
                    let MMT;
                    let MMU;
                    let MMV;
                    let MMW;
                    let MMX;
                    let MMY;
                    let MMZ;
                    let MNA;
                    if MJH != 0.0 {
                        let MJX = (MHZ.abs()).sqrt();
                        let MJY = (MIA * ((GX * (if MHZ >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * MJX));
                        let MJZ = (-MJX).exp();
                        let MKA = (MJY * AC) * MJZ;
                        let MKB = D + MJZ;
                        let MKC = D - MJZ;
                        let MKD = (MJX * MKB) / MKC;
                        let MKE = (((MJY * MKB) + (MKA * MJX)) - ((MKA * AC) * MKD)) / MKC;
                        let MKF = (YY * MIB) / MHZ;
                        let MKG = ((MIC * YY) - (MIA * MKF)) / MHZ;
                        let MKH = LA - MKD;
                        let MKI = MHZ + (MKD * MKH);
                        let MKJ = MKI * MKF;
                        let MKK = ((MIA + ((MKE * MKH) + ((MKE * AC) * MKD))) * MKF) + (MKG * MKI);
                        let MKL = LA * MKJ;
                        let MKM = D + MKD;
                        let MKN = MIB - (MKL * MKM);
                        let MKO = (MKJ * MID) / MIB;
                        let MKP = (MKN * MKF) + MKO;
                        let MKQ = (((MIC - (((MKK * LA) * MKM) + (MKE * MKL))) * MKF) + (MKG * MKN)) + ((((MKK * MID) + (MIE * MKJ)) - (MIC * MKO)) / MIB);
                        let MKR = D - (GO * MKD);
                        let MKS = (MKE * GO) * AC;
                        let MKT = MIB / MHZ;
                        let MKU = MKT * MKR;
                        let MKV = (((MIC - (MIA * MKT)) / MHZ) * MKR) + (MKS * MKT);
                        let MKW = MKU + (GO * MKJ);
                        let MKX = ((MID * MKR) - (MIB * MKW)) / MHZ;
                        let MKY = ((((MIE * MKR) + (MKS * MID)) - ((MIC * MKW) + ((MKV + (MKK * GO)) * MIB))) - (MIA * MKX)) / MHZ;
                        MMN = MJZ;
                        MMO = MJX;
                        MMP = MKD;
                        MMQ = MKJ;
                        MMR = MKP;
                        MMS = MKU;
                        MMT = MKX;
                        MMU = MKA;
                        MMV = MJY;
                        MMW = MKE;
                        MMX = MKK;
                        MMY = MKQ;
                        MMZ = MKV;
                        MNA = MKY;
                    } else {
                        let MKZ = MHZ * ASC;
                        let MLA = MHZ * ASE;
                        let MLB = MIA * ASE;
                        let MLC = D - (MHZ * ASH);
                        let MLD = D - (MLA * MLC);
                        let MLE = UC * (D - (MKZ * MLD));
                        let MLF = ((((MIA * ASC) * MLD) + ((((MLB * MLC) + (((MIA * ASH) * AC) * MLA)) * AC) * MKZ)) * AC) * UC;
                        let MLG = (MIA * MLE) + (MLF * MHZ);
                        let MLH = LA + (MHZ * MLE);
                        let MLI = MHZ * ASO;
                        let MLJ = MIA * ASO;
                        let MLK = MHZ * ASR;
                        let MLL = D - MLI;
                        let MLM = D - (MLK * MLL);
                        let MLN = UC * (D - (MLI * MLM));
                        let MLO = (((MLJ * MLM) + (((((MIA * ASR) * MLL) + ((MLJ * AC) * MLK)) * AC) * MLI)) * AC) * UC;
                        let MLP = MIB * MLN;
                        let MLQ = (MIC * MLN) + (MLO * MIB);
                        let MLR = MHZ * ASZ;
                        let MLS = ATB * MHZ;
                        let MLT = D - (ATD * MHZ);
                        let MLU = D - (MLS * MLT);
                        let MLV = ATG * (D - (MLR * MLU));
                        let MLW = MIB * MIB;
                        let MLX = MIC * MIB;
                        let MLY = (MID * MLN) - (MLW * MLV);
                        let MLZ = ((MIE * MLN) + (MLO * MID)) - (((MLX + MLX) * MLV) + ((((((MIA * ASZ) * MLU) + (((((MIA * ATB) * MLT) + (((MIA * ATD) * AC) * MLS)) * AC) * MLR)) * AC) * ATG) * MLW));
                        let MMB = MMA * MIB;
                        let MMC = MMB * MLE;
                        let MMD = ((MIC * MMA) * MLE) + (MLF * MMB);
                        let MMF = MME * MID;
                        let MMH = MMG * MIB;
                        let MMI = MMH * MIB;
                        let MMJ = LA - (ATV * MHZ);
                        let MMK = D - (MLA * MMJ);
                        let MML = (MMF * MLE) + (MMI * MMK);
                        let MMM = (((MIE * MME) * MLE) + (MLF * MMF)) + (((((MIC * MMG) * MIB) + (MIC * MMH)) * MMK) + ((((MLB * MMJ) + (((MIA * ATV) * AC) * MLA)) * AC) * MMI));
                        MMN = B;
                        MMO = B;
                        MMP = MLH;
                        MMQ = MLP;
                        MMR = MLY;
                        MMS = MMC;
                        MMT = MML;
                        MMU = AFD;
                        MMV = AFD;
                        MMW = MLG;
                        MMX = MLQ;
                        MMY = MLZ;
                        MMZ = MMD;
                        MNA = MMM;
                    }
                    MJI = MMN;
                    MJJ = MMO;
                    MJK = MMP;
                    MJL = MMQ;
                    MJM = MMR;
                    MJN = MMS;
                    MJO = MMT;
                    MJP = MMU;
                    MJQ = MMV;
                    MJR = MMW;
                    MJS = MMX;
                    MJT = MMY;
                    MJU = MMZ;
                    MJV = MNA;
                }
                let MJW = if MHZ > AQJ { 1.0 } else { 0.0 };
                let MNK;
                let MNL;
                let MNM;
                let MNN;
                if MJW != 0.0 {
                    let MNB = LA - MJI;
                    let MNC = D - (MJI * MNB);
                    let MND = (XQ * MHZ) / MNC;
                    let MNE = ((MIA * XQ) - ((((MJP * MNB) + ((MJP * AC) * MJI)) * AC) * MND)) / MNC;
                    let MNF = MND * MJI;
                    let MNG = (MNE * MJI) + (MJP * MND);
                    let MNH = (MND.ln()) - MJJ;
                    let MNI = (MNE * (GY / MND)) - MJQ;
                    MNK = MNF;
                    MNL = MNH;
                    MNM = MNG;
                    MNN = MNI;
                } else {
                    let MNJ = if MHZ < -5e-3f64 { 1.0 } else { 0.0 };
                    let MOF;
                    let MOG;
                    let MOH;
                    let MOI;
                    if MNJ != 0.0 {
                        let MNP = GO * MJJ;
                        let MNQ = MNP.sin();
                        let MNR = MNQ * MNQ;
                        let MNS = ((MJQ * GO) * (MNP.cos())) * MNQ;
                        let MNT = (-MHZ) / MNR;
                        let MNU = ((MIA * AC) - ((MNS + MNS) * MNT)) / MNR;
                        let MNV = MNT.ln();
                        let MNW = MNU * (GY / MNT);
                        MOF = MNT;
                        MOG = MNV;
                        MOH = MNU;
                        MOI = MNW;
                    } else {
                        let MNX = MHZ * WD;
                        let MNY = ATB * MHZ;
                        let MNZ = D - (AVN * MHZ);
                        let MOA = D - (MNY * MNZ);
                        let MOB = XQ - (MNX * MOA);
                        let MOC = (((MIA * WD) * MOA) + (((((MIA * ATB) * MNZ) + (((MIA * AVN) * AC) * MNY)) * AC) * MNX)) * AC;
                        let MOD = MOB.ln();
                        let MOE = MOC * (GY / MOB);
                        MOF = MOB;
                        MOG = MOD;
                        MOH = MOC;
                        MOI = MOE;
                    }
                    MNK = MOF;
                    MNL = MOG;
                    MNM = MOH;
                    MNN = MOI;
                }
                let MNO = if ((AVB * MHH) + MJK) > B { 1.0 } else { 0.0 };
                let MPE;
                let MPF;
                let MPG;
                let MPH;
                let MPI;
                let MPJ;
                if MNO != 0.0 {
                    let MOJ = MHH + MJK;
                    let MOK = MHI + MJR;
                    let MOL = JJR + MJL;
                    let MOM = JJT + MJS;
                    MPE = MOJ;
                    MPF = MOL;
                    MPG = MJM;
                    MPH = MOK;
                    MPI = MOM;
                    MPJ = MJT;
                } else {
                    let MON = MHH - MJK;
                    let MOO = D / MON;
                    let MOP = (((MHI - MJR) * MOO) * AC) / MON;
                    let MOQ = MJL - JJR;
                    let MOR = MJS - JJT;
                    let MOS = MHW - MNK;
                    let MOT = MOS * MOO;
                    let MOU = ((MHX - MNM) * MOO) + (MOP * MOS);
                    let MOV = ((MOQ * MOT) - MHW) - (MJN * MNK);
                    let MOW = MOV * MOO;
                    let MOX = (((((MOR * MOT) + (MOU * MOQ)) - MHX) - ((MJU * MNK) + (MNM * MJN))) * MOO) + (MOP * MOV);
                    let MOY = LA * MOQ;
                    let MOZ = MJU * MJN;
                    let MPA = MJO + (MJN * MJN);
                    let MPB = (((MJM * MOT) + (MOY * MOW)) + MHW) - (MPA * MNK);
                    let MPC = MPB * MOO;
                    let MPD = ((((((MJT * MOT) + (MOU * MJM)) + (((MOR * LA) * MOW) + (MOX * MOY))) + MHX) - (((MJV + (MOZ + MOZ)) * MNK) + (MNM * MPA))) * MOO) + (MOP * MPB);
                    MPE = MOT;
                    MPF = MOW;
                    MPG = MPC;
                    MPH = MOU;
                    MPI = MOX;
                    MPJ = MPD;
                }
                let MPK = if MPE > B { 1.0 } else { 0.0 };
                let MQE;
                let MQF;
                let MQG;
                let MQH;
                let MQI;
                let MQJ;
                if MPK != 0.0 {
                    let MPL = MPE.ln();
                    let MPM = MPH * (GY / MPE);
                    let MPN = D / MPE;
                    let MPO = ((MPH * MPN) * AC) / MPE;
                    let MPP = MPF * MPN;
                    let MPQ = (MPI * MPN) + (MPO * MPF);
                    let MPR = MPQ * MPP;
                    let MPS = (MPG * MPN) - (MPP * MPP);
                    let MPT = ((MPJ * MPN) + (MPO * MPG)) - (MPR + MPR);
                    MQE = MPL;
                    MQF = MPP;
                    MQG = MPS;
                    MQH = MPM;
                    MQI = MPQ;
                    MQJ = MPT;
                } else {
                    let MPU = -MHH;
                    let MPV = (MHH + HW) + (MPU.ln());
                    let MPW = MHI + ((MHI * AC) * (GY / MPU));
                    let MPX = D / MHD;
                    let MPY = ((MHE * MPX) * AC) / MHD;
                    let MPZ = JJR + MPX;
                    let MQA = JJT + MPY;
                    let MQB = -MPX;
                    let MQC = MQB * MPX;
                    let MQD = ((MPY * AC) * MPX) + (MPY * MQB);
                    MQE = MPV;
                    MQF = MPZ;
                    MQG = MQC;
                    MQH = MPW;
                    MQI = MQA;
                    MQJ = MQD;
                }
                let MQK = ((JYJ + MHD) + (LA * MQE)) - MNL;
                let MQL = (D + (LA * MQF)) - MJN;
                let MQM = (LA * MQG) - MJO;
                let MQN = MHH + (JJU * MQK);
                let MQO = MHI + ((JJW * MQK) + ((((JYK + MHE) + (MQH * LA)) - MNN) * JJU));
                let MQP = JJR + (JJU * MQL);
                let MQQ = JJT + ((JJW * MQL) + (((MQI * LA) - MJU) * JJU));
                let MQR = JJU * MQM;
                let MQS = (MQN * MPE) - MHW;
                let MQT = ((MQO * MPE) + (MPH * MQN)) - MHX;
                let MQU = ((MQP * MPE) + (MQN * MPF)) + MHW;
                let MQV = (((MQQ * MPE) + (MPH * MQP)) + ((MQO * MPF) + (MPI * MQN))) + MHX;
                let MQW = LA * MQP;
                let MQX = (((MQR * MPE) + (MQW * MPF)) + (MQN * MPG)) - MHW;
                let MQY = MQV * MQU;
                let MQZ = GO * MQS;
                let MRA = (MQU * MQU) - (MQZ * MQX);
                let MRB = (MQY + MQY) - (((MQT * GO) * MQX) + ((((((((JJW * MQM) + (((MQJ * LA) - MJV) * JJU)) * MPE) + (MPH * MQR)) + (((MQQ * LA) * MPF) + (MPI * MQW))) + ((MQO * MPG) + (MPJ * MQN))) - MHX) * MQZ));
                let MRC = -MQS;
                let MRD = MRC * MQU;
                let MRE = MRB * MRA;
                let MRF = (MRA * MRA) + AYW;
                let MRG = (MRD * MRA) / MRF;
                let MRH = MHD + MRG;
                let MRI = MHE + (((((((MQT * AC) * MQU) + (MQV * MRC)) * MRA) + (MRB * MRD)) - ((MRE + MRE) * MRG)) / MRF);
                let MRJ = JJR * MRH;
                let MRK = (JJT * MRH) + (MRI * JJR);
                let MRL = JJU * MHF;
                let MRM = (JJW * MHF) + (MHG * JJU);
                let MRN = MRJ + MRL;
                let MRO = MRK + MRM;
                let MRP = MRO * AZH;
                let MRQ = D + (AZH * MRN);
                let MRR = MRJ * MRL;
                let MRS = (MRK * MRL) + (MRM * MRJ);
                let MRT = (AZL + (AZK * MRN)) + MRR;
                let MRU = (MRO * AZK) + MRS;
                let MRV = AZL * ((LA * MRN) + MRR);
                let MRW = MRU * MRT;
                let MRX = XQ * MRQ;
                let MRY = ((MRT * MRT) - (MRX * MRV)).sqrt();
                let MRZ = LA * MRQ;
                let MSA = (MRY - MRT) / MRZ;
                let MSB = MRK * MRJ;
                let MSC = (MRJ * MRJ) - MSA;
                let MSD = (MSB + MSB) - ((((((MRW + MRW) - (((MRP * XQ) * MRV) + ((((MRO * LA) + MRS) * AZL) * MRX))) * (GY / (GX * MRY))) - MRU) - ((MRP * LA) * MSA)) / MRZ);
                let MSE = if MSC > B { 1.0 } else { 0.0 };
                let MSN;
                let MSO;
                if MSE != 0.0 {
                    let MSF = MSC / JKG;
                    let MSG = (((MSF.ln()) + MFE) - JJI) + MRH;
                    let MSH = MSC * MSG;
                    let MSI = (MSD * MSG) + (((((((MSD - (JKH * MSF)) / JKG) * (GY / MSF)) + MFG) - JJJ) + MRI) * MSC);
                    let MSJ = (JPW * MRJ) + MSC;
                    let MSK = ((JPX * MRJ) + (MRK * JPW)) + MSD;
                    let MSL = (JJI - MRH) - MFI;
                    let MSM = if (if (if (if MSH < B { 1.0 } else { 0.0 }) != 0.0 && (if MSJ > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((MSL + BAH) + (JJR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if MSL > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let MTK;
                    let MTL;
                    if MSM != 0.0 {
                        let MTH = MSH / MSJ;
                        let MTI = MRH - MTH;
                        let MTJ = MRI - ((MSI - (MSK * MTH)) / MSJ);
                        MTK = MTI;
                        MTL = MTJ;
                    } else {
                        MTK = MRH;
                        MTL = MRI;
                    }
                    MSN = MTK;
                    MSO = MTL;
                } else {
                    MSN = MRH;
                    MSO = MRI;
                }
                let MSP = JJR * MSN;
                let MSQ = (JJT * MSN) + (MSO * JJR);
                let MSR = MSP + MRL;
                let MSS = MSQ + MRM;
                let MST = MSS * AZH;
                let MSU = D + (AZH * MSR);
                let MSV = MSP * MRL;
                let MSW = (MSQ * MRL) + (MRM * MSP);
                let MSX = (AZL + (AZK * MSR)) + MSV;
                let MSY = (MSS * AZK) + MSW;
                let MSZ = AZL * ((LA * MSR) + MSV);
                let MTA = MSY * MSX;
                let MTB = XQ * MSU;
                let MTC = ((MSX * MSX) - (MTB * MSZ)).sqrt();
                let MTD = LA * MSU;
                let MTE = (MTC - MSX) / MTD;
                let MTF = (((((MTA + MTA) - (((MST * XQ) * MSZ) + ((((MSS * LA) + MSW) * AZL) * MTB))) * (GY / (GX * MTC))) - MSY) - ((MST * LA) * MTE)) / MTD;
                let MTG = if MTE < -5e-3f64 { 1.0 } else { 0.0 };
                let MTX;
                let MTY;
                let MTZ;
                let MUA;
                let MUB;
                let MUC;
                let MUD;
                let MUE;
                if MTG != 0.0 {
                    let MTM = (MTE.abs()).sqrt();
                    let MTN = (MTF * ((GX * (if MTE >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * MTM));
                    let MTO = GO * MTM;
                    let MTP = MTO.tan();
                    let MTQ = MTO.cos();
                    let MTR = MTM / MTP;
                    let MTS = (MTN - (((MTN * GO) * (GY / (MTQ * MTQ))) * MTR)) / MTP;
                    let MTT = LA - MTR;
                    let MTU = (YY * (MTE + (MTR * MTT))) / MTE;
                    let MTV = (((MTF + ((MTS * MTT) + ((MTS * AC) * MTR))) * YY) - (MTF * MTU)) / MTE;
                    MTX = MTR;
                    MTY = MTU;
                    MTZ = MJI;
                    MUA = MTM;
                    MUB = MTS;
                    MUC = MTV;
                    MUD = MJP;
                    MUE = MTN;
                } else {
                    let MTW = if MTE > AQJ { 1.0 } else { 0.0 };
                    let MVJ;
                    let MVK;
                    let MVL;
                    let MVM;
                    let MVN;
                    let MVO;
                    let MVP;
                    let MVQ;
                    if MTW != 0.0 {
                        let MUL = (MTE.abs()).sqrt();
                        let MUM = (MTF * ((GX * (if MTE >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * MUL));
                        let MUN = (-MUL).exp();
                        let MUO = (MUM * AC) * MUN;
                        let MUP = D + MUN;
                        let MUQ = D - MUN;
                        let MUR = (MUL * MUP) / MUQ;
                        let MUS = (((MUM * MUP) + (MUO * MUL)) - ((MUO * AC) * MUR)) / MUQ;
                        let MUT = LA - MUR;
                        let MUU = (YY * (MTE + (MUR * MUT))) / MTE;
                        let MUV = (((MTF + ((MUS * MUT) + ((MUS * AC) * MUR))) * YY) - (MTF * MUU)) / MTE;
                        MVJ = MUR;
                        MVK = MUU;
                        MVL = MUN;
                        MVM = MUL;
                        MVN = MUS;
                        MVO = MUV;
                        MVP = MUO;
                        MVQ = MUM;
                    } else {
                        let MUW = MTE * UC;
                        let MUX = MTE * ASC;
                        let MUY = D - (MTE * ASE);
                        let MUZ = D - (MUX * MUY);
                        let MVA = ((MTF * UC) * MUZ) + (((((MTF * ASC) * MUY) + (((MTF * ASE) * AC) * MUX)) * AC) * MUW);
                        let MVB = LA + (MUW * MUZ);
                        let MVC = MTE * ASO;
                        let MVD = MTF * ASO;
                        let MVE = MTE * ASR;
                        let MVF = D - MVC;
                        let MVG = D - (MVE * MVF);
                        let MVH = UC * (D - (MVC * MVG));
                        let MVI = (((MVD * MVG) + (((((MTF * ASR) * MVF) + ((MVD * AC) * MVE)) * AC) * MVC)) * AC) * UC;
                        MVJ = MVB;
                        MVK = MVH;
                        MVL = MJI;
                        MVM = MJJ;
                        MVN = MVA;
                        MVO = MVI;
                        MVP = MJP;
                        MVQ = MJQ;
                    }
                    MTX = MVJ;
                    MTY = MVK;
                    MTZ = MVL;
                    MUA = MVM;
                    MUB = MVN;
                    MUC = MVO;
                    MUD = MVP;
                    MUE = MVQ;
                }
                let MUF = (MSR * MTY) + D;
                let MUG = (((MSR * MTX) + MSV) + MTE) / MUF;
                let MUH = MSQ * MSP;
                let MUI = (MSP * MSP) - (MTE - MUG);
                let MUJ = (MUH + MUH) - (MTF - ((((((MSS * MTX) + (MUB * MSR)) + MSW) + MTF) - (((MSS * MTY) + (MUC * MSR)) * MUG)) / MUF));
                let MUK = if MUI > B { 1.0 } else { 0.0 };
                let MVZ;
                let MWA;
                if MUK != 0.0 {
                    let MVR = MUI / JKG;
                    let MVS = (((MVR.ln()) + MFE) - JJI) + MSN;
                    let MVT = MUI * MVS;
                    let MVU = (MUJ * MVS) + (((((((MUJ - (JKH * MVR)) / JKG) * (GY / MVR)) + MFG) - JJJ) + MSO) * MUI);
                    let MVV = (JPW * MSP) + MUI;
                    let MVW = ((JPX * MSP) + (MSQ * JPW)) + MUJ;
                    let MVX = (JJI - MSN) - MFI;
                    let MVY = if (if (if (if MVT < B { 1.0 } else { 0.0 }) != 0.0 && (if MVV > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((MVX + BAH) + (JJR.ln())) > B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if MVX > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let MWJ;
                    let MWK;
                    if MVY != 0.0 {
                        let MWG = MVT / MVV;
                        let MWH = MSN - MWG;
                        let MWI = MSO - ((MVU - (MVW * MWG)) / MVV);
                        MWJ = MWH;
                        MWK = MWI;
                    } else {
                        MWJ = MSN;
                        MWK = MSO;
                    }
                    MVZ = MWJ;
                    MWA = MWK;
                } else {
                    MVZ = MSN;
                    MWA = MSO;
                }
                let MWB = JJR * MVZ;
                let MWC = (JJT * MVZ) + (MWA * JJR);
                let MWD = (JJI - MVZ) - MFE;
                let MWE = (JJJ - MWA) - MFG;
                let MWF = if MWD < SY { 1.0 } else { 0.0 };
                let MWT;
                let MWU;
                if MWF != 0.0 {
                    let MWL = MWD.exp();
                    let MWM = MWE * MWL;
                    MWT = MWL;
                    MWU = MWM;
                } else {
                    let MWN = MWD - SY;
                    let MWO = GO * MWN;
                    let MWP = D + (MWN * WD);
                    let MWQ = D + (MWO * MWP);
                    let MWR = XB * (D + (MWN * MWQ));
                    let MWS = ((MWE * MWQ) + ((((MWE * GO) * MWP) + ((MWE * WD) * MWO)) * MWN)) * XB;
                    MWT = MWR;
                    MWU = MWS;
                }
                let MWV = JKG * MWT;
                let MWW = (JKH * MWT) + (MWU * JKG);
                let MWX = MWC * MWB;
                let MWY = (MWB * MWB) - MWV;
                let MWZ = (MWX + MWX) - MWW;
                let MXA = (JPW * MWB) + MWV;
                let MXB = ((JPX * MWB) + (MWC * JPW)) + MWW;
                let MXC = JQA - MWV;
                let MXD = JQB - MWW;
                let MXE = if MWY < -5e-3f64 { 1.0 } else { 0.0 };
                let MYH;
                let MYI;
                let MYJ;
                let MYK;
                let MYL;
                let MYM;
                let MYN;
                let MYO;
                let MYP;
                let MYQ;
                let MYR;
                let MYS;
                let MYT;
                let MYU;
                if MXE != 0.0 {
                    let MXF = (MWY.abs()).sqrt();
                    let MXG = (MWZ * ((GX * (if MWY >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * MXF));
                    let MXH = GO * MXF;
                    let MXI = MXH.tan();
                    let MXJ = MXH.cos();
                    let MXK = MXF / MXI;
                    let MXL = (MXG - (((MXG * GO) * (GY / (MXJ * MXJ))) * MXK)) / MXI;
                    let MXM = (YY * MXA) / MWY;
                    let MXN = ((MXB * YY) - (MWZ * MXM)) / MWY;
                    let MXO = LA - MXK;
                    let MXP = MWY + (MXK * MXO);
                    let MXQ = MXP * MXM;
                    let MXR = ((MWZ + ((MXL * MXO) + ((MXL * AC) * MXK))) * MXM) + (MXN * MXP);
                    let MXS = LA * MXQ;
                    let MXT = D + MXK;
                    let MXU = MXA - (MXS * MXT);
                    let MXV = (MXQ * MXC) / MXA;
                    let MXW = (MXU * MXM) + MXV;
                    let MXX = (((MXB - (((MXR * LA) * MXT) + (MXL * MXS))) * MXM) + (MXN * MXU)) + ((((MXR * MXC) + (MXD * MXQ)) - (MXB * MXV)) / MXA);
                    let MXY = D - (GO * MXK);
                    let MXZ = (MXL * GO) * AC;
                    let MYA = MXA / MWY;
                    let MYB = MYA * MXY;
                    let MYC = (((MXB - (MWZ * MYA)) / MWY) * MXY) + (MXZ * MYA);
                    let MYD = MYB + (GO * MXQ);
                    let MYE = ((MXC * MXY) - (MXA * MYD)) / MWY;
                    let MYF = ((((MXD * MXY) + (MXZ * MXC)) - ((MXB * MYD) + ((MYC + (MXR * GO)) * MXA))) - (MWZ * MYE)) / MWY;
                    MYH = MTZ;
                    MYI = MXF;
                    MYJ = MXK;
                    MYK = MXQ;
                    MYL = MXW;
                    MYM = MYB;
                    MYN = MYE;
                    MYO = MUD;
                    MYP = MXG;
                    MYQ = MXL;
                    MYR = MXR;
                    MYS = MXX;
                    MYT = MYC;
                    MYU = MYF;
                } else {
                    let MYG = if MWY > AQJ { 1.0 } else { 0.0 };
                    let NBM;
                    let NBN;
                    let NBO;
                    let NBP;
                    let NBQ;
                    let NBR;
                    let NBS;
                    let NBT;
                    let NBU;
                    let NBV;
                    let NBW;
                    let NBX;
                    let NBY;
                    let NBZ;
                    if MYG != 0.0 {
                        let MYW = (MWY.abs()).sqrt();
                        let MYX = (MWZ * ((GX * (if MWY >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * MYW));
                        let MYY = (-MYW).exp();
                        let MYZ = (MYX * AC) * MYY;
                        let MZA = D + MYY;
                        let MZB = D - MYY;
                        let MZC = (MYW * MZA) / MZB;
                        let MZD = (((MYX * MZA) + (MYZ * MYW)) - ((MYZ * AC) * MZC)) / MZB;
                        let MZE = (YY * MXA) / MWY;
                        let MZF = ((MXB * YY) - (MWZ * MZE)) / MWY;
                        let MZG = LA - MZC;
                        let MZH = MWY + (MZC * MZG);
                        let MZI = MZH * MZE;
                        let MZJ = ((MWZ + ((MZD * MZG) + ((MZD * AC) * MZC))) * MZE) + (MZF * MZH);
                        let MZK = LA * MZI;
                        let MZL = D + MZC;
                        let MZM = MXA - (MZK * MZL);
                        let MZN = (MZI * MXC) / MXA;
                        let MZO = (MZM * MZE) + MZN;
                        let MZP = (((MXB - (((MZJ * LA) * MZL) + (MZD * MZK))) * MZE) + (MZF * MZM)) + ((((MZJ * MXC) + (MXD * MZI)) - (MXB * MZN)) / MXA);
                        let MZQ = D - (GO * MZC);
                        let MZR = (MZD * GO) * AC;
                        let MZS = MXA / MWY;
                        let MZT = MZS * MZQ;
                        let MZU = (((MXB - (MWZ * MZS)) / MWY) * MZQ) + (MZR * MZS);
                        let MZV = MZT + (GO * MZI);
                        let MZW = ((MXC * MZQ) - (MXA * MZV)) / MWY;
                        let MZX = ((((MXD * MZQ) + (MZR * MXC)) - ((MXB * MZV) + ((MZU + (MZJ * GO)) * MXA))) - (MWZ * MZW)) / MWY;
                        NBM = MYY;
                        NBN = MYW;
                        NBO = MZC;
                        NBP = MZI;
                        NBQ = MZO;
                        NBR = MZT;
                        NBS = MZW;
                        NBT = MYZ;
                        NBU = MYX;
                        NBV = MZD;
                        NBW = MZJ;
                        NBX = MZP;
                        NBY = MZU;
                        NBZ = MZX;
                    } else {
                        let MZY = MWY * ASC;
                        let MZZ = MWY * ASE;
                        let NAA = MWZ * ASE;
                        let NAB = D - (MWY * ASH);
                        let NAC = D - (MZZ * NAB);
                        let NAD = UC * (D - (MZY * NAC));
                        let NAE = ((((MWZ * ASC) * NAC) + ((((NAA * NAB) + (((MWZ * ASH) * AC) * MZZ)) * AC) * MZY)) * AC) * UC;
                        let NAF = (MWZ * NAD) + (NAE * MWY);
                        let NAG = LA + (MWY * NAD);
                        let NAH = MWY * ASO;
                        let NAI = MWZ * ASO;
                        let NAJ = MWY * ASR;
                        let NAK = D - NAH;
                        let NAL = D - (NAJ * NAK);
                        let NAM = UC * (D - (NAH * NAL));
                        let NAN = (((NAI * NAL) + (((((MWZ * ASR) * NAK) + ((NAI * AC) * NAJ)) * AC) * NAH)) * AC) * UC;
                        let NAO = MXA * NAM;
                        let NAP = (MXB * NAM) + (NAN * MXA);
                        let NAQ = MWY * ASZ;
                        let NAR = ATB * MWY;
                        let NAS = D - (ATD * MWY);
                        let NAT = D - (NAR * NAS);
                        let NAU = ATG * (D - (NAQ * NAT));
                        let NAV = MXA * MXA;
                        let NAW = MXB * MXA;
                        let NAX = (MXC * NAM) - (NAV * NAU);
                        let NAY = ((MXD * NAM) + (NAN * MXC)) - (((NAW + NAW) * NAU) + ((((((MWZ * ASZ) * NAT) + (((((MWZ * ATB) * NAS) + (((MWZ * ATD) * AC) * NAR)) * AC) * NAQ)) * AC) * ATG) * NAV));
                        let NBA = NAZ * MXA;
                        let NBB = NBA * NAD;
                        let NBC = ((MXB * NAZ) * NAD) + (NAE * NBA);
                        let NBE = NBD * MXC;
                        let NBG = NBF * MXA;
                        let NBH = NBG * MXA;
                        let NBI = LA - (ATV * MWY);
                        let NBJ = D - (MZZ * NBI);
                        let NBK = (NBE * NAD) + (NBH * NBJ);
                        let NBL = (((MXD * NBD) * NAD) + (NAE * NBE)) + (((((MXB * NBF) * MXA) + (MXB * NBG)) * NBJ) + ((((NAA * NBI) + (((MWZ * ATV) * AC) * MZZ)) * AC) * NBH));
                        NBM = MTZ;
                        NBN = MUA;
                        NBO = NAG;
                        NBP = NAO;
                        NBQ = NAX;
                        NBR = NBB;
                        NBS = NBK;
                        NBT = MUD;
                        NBU = MUE;
                        NBV = NAF;
                        NBW = NAP;
                        NBX = NAY;
                        NBY = NBC;
                        NBZ = NBL;
                    }
                    MYH = NBM;
                    MYI = NBN;
                    MYJ = NBO;
                    MYK = NBP;
                    MYL = NBQ;
                    MYM = NBR;
                    MYN = NBS;
                    MYO = NBT;
                    MYP = NBU;
                    MYQ = NBV;
                    MYR = NBW;
                    MYS = NBX;
                    MYT = NBY;
                    MYU = NBZ;
                }
                let MYV = if MWY > AQJ { 1.0 } else { 0.0 };
                let NCJ;
                let NCK;
                let NCL;
                let NCM;
                if MYV != 0.0 {
                    let NCA = LA - MYH;
                    let NCB = D - (MYH * NCA);
                    let NCC = (XQ * MWY) / NCB;
                    let NCD = ((MWZ * XQ) - ((((MYO * NCA) + ((MYO * AC) * MYH)) * AC) * NCC)) / NCB;
                    let NCE = NCC * MYH;
                    let NCF = (NCD * MYH) + (MYO * NCC);
                    let NCG = (NCC.ln()) - MYI;
                    let NCH = (NCD * (GY / NCC)) - MYP;
                    NCJ = NCE;
                    NCK = NCG;
                    NCL = NCF;
                    NCM = NCH;
                } else {
                    let NCI = if MWY < -5e-3f64 { 1.0 } else { 0.0 };
                    let NDE;
                    let NDF;
                    let NDG;
                    let NDH;
                    if NCI != 0.0 {
                        let NCO = GO * MYI;
                        let NCP = NCO.sin();
                        let NCQ = NCP * NCP;
                        let NCR = ((MYP * GO) * (NCO.cos())) * NCP;
                        let NCS = (-MWY) / NCQ;
                        let NCT = ((MWZ * AC) - ((NCR + NCR) * NCS)) / NCQ;
                        let NCU = NCS.ln();
                        let NCV = NCT * (GY / NCS);
                        NDE = NCS;
                        NDF = NCU;
                        NDG = NCT;
                        NDH = NCV;
                    } else {
                        let NCW = MWY * WD;
                        let NCX = ATB * MWY;
                        let NCY = D - (AVN * MWY);
                        let NCZ = D - (NCX * NCY);
                        let NDA = XQ - (NCW * NCZ);
                        let NDB = (((MWZ * WD) * NCZ) + (((((MWZ * ATB) * NCY) + (((MWZ * AVN) * AC) * NCX)) * AC) * NCW)) * AC;
                        let NDC = NDA.ln();
                        let NDD = NDB * (GY / NDA);
                        NDE = NDA;
                        NDF = NDC;
                        NDG = NDB;
                        NDH = NDD;
                    }
                    NCJ = NDE;
                    NCK = NDF;
                    NCL = NDG;
                    NCM = NDH;
                }
                let NCN = if ((AVB * MWB) + MYJ) > B { 1.0 } else { 0.0 };
                let NED;
                let NEE;
                let NEF;
                let NEG;
                let NEH;
                let NEI;
                if NCN != 0.0 {
                    let NDI = MWB + MYJ;
                    let NDJ = MWC + MYQ;
                    let NDK = JJR + MYK;
                    let NDL = JJT + MYR;
                    NED = NDI;
                    NEE = NDK;
                    NEF = MYL;
                    NEG = NDJ;
                    NEH = NDL;
                    NEI = MYS;
                } else {
                    let NDM = MWB - MYJ;
                    let NDN = D / NDM;
                    let NDO = (((MWC - MYQ) * NDN) * AC) / NDM;
                    let NDP = MYK - JJR;
                    let NDQ = MYR - JJT;
                    let NDR = MWV - NCJ;
                    let NDS = NDR * NDN;
                    let NDT = ((MWW - NCL) * NDN) + (NDO * NDR);
                    let NDU = ((NDP * NDS) - MWV) - (MYM * NCJ);
                    let NDV = NDU * NDN;
                    let NDW = (((((NDQ * NDS) + (NDT * NDP)) - MWW) - ((MYT * NCJ) + (NCL * MYM))) * NDN) + (NDO * NDU);
                    let NDX = LA * NDP;
                    let NDY = MYT * MYM;
                    let NDZ = MYN + (MYM * MYM);
                    let NEA = (((MYL * NDS) + (NDX * NDV)) + MWV) - (NDZ * NCJ);
                    let NEB = NEA * NDN;
                    let NEC = ((((((MYS * NDS) + (NDT * MYL)) + (((NDQ * LA) * NDV) + (NDW * NDX))) + MWW) - (((MYU + (NDY + NDY)) * NCJ) + (NCL * NDZ))) * NDN) + (NDO * NEA);
                    NED = NDS;
                    NEE = NDV;
                    NEF = NEB;
                    NEG = NDT;
                    NEH = NDW;
                    NEI = NEC;
                }
                let NEJ = if NED > B { 1.0 } else { 0.0 };
                let NFD;
                let NFE;
                let NFF;
                let NFG;
                let NFH;
                let NFI;
                if NEJ != 0.0 {
                    let NEK = NED.ln();
                    let NEL = NEG * (GY / NED);
                    let NEM = D / NED;
                    let NEN = ((NEG * NEM) * AC) / NED;
                    let NEO = NEE * NEM;
                    let NEP = (NEH * NEM) + (NEN * NEE);
                    let NEQ = NEP * NEO;
                    let NER = (NEF * NEM) - (NEO * NEO);
                    let NES = ((NEI * NEM) + (NEN * NEF)) - (NEQ + NEQ);
                    NFD = NEK;
                    NFE = NEO;
                    NFF = NER;
                    NFG = NEL;
                    NFH = NEP;
                    NFI = NES;
                } else {
                    let NET = -MWB;
                    let NEU = (MWB + HW) + (NET.ln());
                    let NEV = MWC + ((MWC * AC) * (GY / NET));
                    let NEW = D / MVZ;
                    let NEX = ((MWA * NEW) * AC) / MVZ;
                    let NEY = JJR + NEW;
                    let NEZ = JJT + NEX;
                    let NFA = -NEW;
                    let NFB = NFA * NEW;
                    let NFC = ((NEX * AC) * NEW) + (NEX * NFA);
                    NFD = NEU;
                    NFE = NEY;
                    NFF = NFB;
                    NFG = NEV;
                    NFH = NEZ;
                    NFI = NFC;
                }
                let NFJ = ((JYJ + MVZ) + (LA * NFD)) - NCK;
                let NFK = (D + (LA * NFE)) - MYM;
                let NFL = (LA * NFF) - MYN;
                let NFM = MWB + (JJU * NFJ);
                let NFN = MWC + ((JJW * NFJ) + ((((JYK + MWA) + (NFG * LA)) - NCM) * JJU));
                let NFO = JJR + (JJU * NFK);
                let NFP = JJT + ((JJW * NFK) + (((NFH * LA) - MYT) * JJU));
                let NFQ = JJU * NFL;
                let NFR = (NFM * NED) - MWV;
                let NFS = ((NFN * NED) + (NEG * NFM)) - MWW;
                let NFT = ((NFO * NED) + (NFM * NEE)) + MWV;
                let NFU = (((NFP * NED) + (NEG * NFO)) + ((NFN * NEE) + (NEH * NFM))) + MWW;
                let NFV = LA * NFO;
                let NFW = (((NFQ * NED) + (NFV * NEE)) + (NFM * NEF)) - MWV;
                let NFX = NFU * NFT;
                let NFY = GO * NFR;
                let NFZ = (NFT * NFT) - (NFY * NFW);
                let NGA = (NFX + NFX) - (((NFS * GO) * NFW) + ((((((((JJW * NFL) + (((NFI * LA) - MYU) * JJU)) * NED) + (NEG * NFQ)) + (((NFP * LA) * NEE) + (NEH * NFV))) + ((NFN * NEF) + (NEI * NFM))) - MWW) * NFY));
                let NGB = -NFR;
                let NGC = NGB * NFT;
                let NGD = NGA * NFZ;
                let NGE = (NFZ * NFZ) + AYW;
                let NGF = (NGC * NFZ) / NGE;
                let NGG = MVZ + NGF;
                let NGH = MWA + (((((((NFS * AC) * NFT) + (NFU * NGB)) * NFZ) + (NGA * NGC)) - ((NGD + NGD) * NGF)) / NGE);
                let NGI = JJR * NGG;
                let NGJ = (JJT * NGG) + (NGH * JJR);
                let NGK = (JJI - NGG) - MFE;
                let NGL = (JJJ - NGH) - MFG;
                let NGM = if NGK < SY { 1.0 } else { 0.0 };
                let NGV;
                let NGW;
                if NGM != 0.0 {
                    let NGN = NGK.exp();
                    let NGO = NGL * NGN;
                    NGV = NGN;
                    NGW = NGO;
                } else {
                    let NGP = NGK - SY;
                    let NGQ = GO * NGP;
                    let NGR = D + (NGP * WD);
                    let NGS = D + (NGQ * NGR);
                    let NGT = XB * (D + (NGP * NGS));
                    let NGU = ((NGL * NGS) + ((((NGL * GO) * NGR) + ((NGL * WD) * NGQ)) * NGP)) * XB;
                    NGV = NGT;
                    NGW = NGU;
                }
                let NGX = JKG * NGV;
                let NGY = (JKH * NGV) + (NGW * JKG);
                let NGZ = NGJ * NGI;
                let NHA = (NGI * NGI) - NGX;
                let NHB = (NGZ + NGZ) - NGY;
                let NHC = (JPW * NGI) + NGX;
                let NHD = ((JPX * NGI) + (NGJ * JPW)) + NGY;
                let NHE = JQA - NGX;
                let NHF = JQB - NGY;
                let NHG = if NHA < -5e-3f64 { 1.0 } else { 0.0 };
                let NIJ;
                let NIK;
                let NIL;
                let NIM;
                let NIN;
                let NIO;
                let NIP;
                let NIQ;
                let NIR;
                let NIS;
                let NIT;
                let NIU;
                let NIV;
                let NIW;
                if NHG != 0.0 {
                    let NHH = (NHA.abs()).sqrt();
                    let NHI = (NHB * ((GX * (if NHA >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * NHH));
                    let NHJ = GO * NHH;
                    let NHK = NHJ.tan();
                    let NHL = NHJ.cos();
                    let NHM = NHH / NHK;
                    let NHN = (NHI - (((NHI * GO) * (GY / (NHL * NHL))) * NHM)) / NHK;
                    let NHO = (YY * NHC) / NHA;
                    let NHP = ((NHD * YY) - (NHB * NHO)) / NHA;
                    let NHQ = LA - NHM;
                    let NHR = NHA + (NHM * NHQ);
                    let NHS = NHR * NHO;
                    let NHT = ((NHB + ((NHN * NHQ) + ((NHN * AC) * NHM))) * NHO) + (NHP * NHR);
                    let NHU = LA * NHS;
                    let NHV = D + NHM;
                    let NHW = NHC - (NHU * NHV);
                    let NHX = (NHS * NHE) / NHC;
                    let NHY = (NHW * NHO) + NHX;
                    let NHZ = (((NHD - (((NHT * LA) * NHV) + (NHN * NHU))) * NHO) + (NHP * NHW)) + ((((NHT * NHE) + (NHF * NHS)) - (NHD * NHX)) / NHC);
                    let NIA = D - (GO * NHM);
                    let NIB = (NHN * GO) * AC;
                    let NIC = NHC / NHA;
                    let NID = NIC * NIA;
                    let NIE = (((NHD - (NHB * NIC)) / NHA) * NIA) + (NIB * NIC);
                    let NIF = NID + (GO * NHS);
                    let NIG = ((NHE * NIA) - (NHC * NIF)) / NHA;
                    let NIH = ((((NHF * NIA) + (NIB * NHE)) - ((NHD * NIF) + ((NIE + (NHT * GO)) * NHC))) - (NHB * NIG)) / NHA;
                    NIJ = MYH;
                    NIK = NHH;
                    NIL = NHM;
                    NIM = NHS;
                    NIN = NHY;
                    NIO = NID;
                    NIP = NIG;
                    NIQ = MYO;
                    NIR = NHI;
                    NIS = NHN;
                    NIT = NHT;
                    NIU = NHZ;
                    NIV = NIE;
                    NIW = NIH;
                } else {
                    let NII = if NHA > AQJ { 1.0 } else { 0.0 };
                    let NLO;
                    let NLP;
                    let NLQ;
                    let NLR;
                    let NLS;
                    let NLT;
                    let NLU;
                    let NLV;
                    let NLW;
                    let NLX;
                    let NLY;
                    let NLZ;
                    let NMA;
                    let NMB;
                    if NII != 0.0 {
                        let NIY = (NHA.abs()).sqrt();
                        let NIZ = (NHB * ((GX * (if NHA >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * NIY));
                        let NJA = (-NIY).exp();
                        let NJB = (NIZ * AC) * NJA;
                        let NJC = D + NJA;
                        let NJD = D - NJA;
                        let NJE = (NIY * NJC) / NJD;
                        let NJF = (((NIZ * NJC) + (NJB * NIY)) - ((NJB * AC) * NJE)) / NJD;
                        let NJG = (YY * NHC) / NHA;
                        let NJH = ((NHD * YY) - (NHB * NJG)) / NHA;
                        let NJI = LA - NJE;
                        let NJJ = NHA + (NJE * NJI);
                        let NJK = NJJ * NJG;
                        let NJL = ((NHB + ((NJF * NJI) + ((NJF * AC) * NJE))) * NJG) + (NJH * NJJ);
                        let NJM = LA * NJK;
                        let NJN = D + NJE;
                        let NJO = NHC - (NJM * NJN);
                        let NJP = (NJK * NHE) / NHC;
                        let NJQ = (NJO * NJG) + NJP;
                        let NJR = (((NHD - (((NJL * LA) * NJN) + (NJF * NJM))) * NJG) + (NJH * NJO)) + ((((NJL * NHE) + (NHF * NJK)) - (NHD * NJP)) / NHC);
                        let NJS = D - (GO * NJE);
                        let NJT = (NJF * GO) * AC;
                        let NJU = NHC / NHA;
                        let NJV = NJU * NJS;
                        let NJW = (((NHD - (NHB * NJU)) / NHA) * NJS) + (NJT * NJU);
                        let NJX = NJV + (GO * NJK);
                        let NJY = ((NHE * NJS) - (NHC * NJX)) / NHA;
                        let NJZ = ((((NHF * NJS) + (NJT * NHE)) - ((NHD * NJX) + ((NJW + (NJL * GO)) * NHC))) - (NHB * NJY)) / NHA;
                        NLO = NJA;
                        NLP = NIY;
                        NLQ = NJE;
                        NLR = NJK;
                        NLS = NJQ;
                        NLT = NJV;
                        NLU = NJY;
                        NLV = NJB;
                        NLW = NIZ;
                        NLX = NJF;
                        NLY = NJL;
                        NLZ = NJR;
                        NMA = NJW;
                        NMB = NJZ;
                    } else {
                        let NKA = NHA * ASC;
                        let NKB = NHA * ASE;
                        let NKC = NHB * ASE;
                        let NKD = D - (NHA * ASH);
                        let NKE = D - (NKB * NKD);
                        let NKF = UC * (D - (NKA * NKE));
                        let NKG = ((((NHB * ASC) * NKE) + ((((NKC * NKD) + (((NHB * ASH) * AC) * NKB)) * AC) * NKA)) * AC) * UC;
                        let NKH = (NHB * NKF) + (NKG * NHA);
                        let NKI = LA + (NHA * NKF);
                        let NKJ = NHA * ASO;
                        let NKK = NHB * ASO;
                        let NKL = NHA * ASR;
                        let NKM = D - NKJ;
                        let NKN = D - (NKL * NKM);
                        let NKO = UC * (D - (NKJ * NKN));
                        let NKP = (((NKK * NKN) + (((((NHB * ASR) * NKM) + ((NKK * AC) * NKL)) * AC) * NKJ)) * AC) * UC;
                        let NKQ = NHC * NKO;
                        let NKR = (NHD * NKO) + (NKP * NHC);
                        let NKS = NHA * ASZ;
                        let NKT = ATB * NHA;
                        let NKU = D - (ATD * NHA);
                        let NKV = D - (NKT * NKU);
                        let NKW = ATG * (D - (NKS * NKV));
                        let NKX = NHC * NHC;
                        let NKY = NHD * NHC;
                        let NKZ = (NHE * NKO) - (NKX * NKW);
                        let NLA = ((NHF * NKO) + (NKP * NHE)) - (((NKY + NKY) * NKW) + ((((((NHB * ASZ) * NKV) + (((((NHB * ATB) * NKU) + (((NHB * ATD) * AC) * NKT)) * AC) * NKS)) * AC) * ATG) * NKX));
                        let NLC = NLB * NHC;
                        let NLD = NLC * NKF;
                        let NLE = ((NHD * NLB) * NKF) + (NKG * NLC);
                        let NLG = NLF * NHE;
                        let NLI = NLH * NHC;
                        let NLJ = NLI * NHC;
                        let NLK = LA - (ATV * NHA);
                        let NLL = D - (NKB * NLK);
                        let NLM = (NLG * NKF) + (NLJ * NLL);
                        let NLN = (((NHF * NLF) * NKF) + (NKG * NLG)) + (((((NHD * NLH) * NHC) + (NHD * NLI)) * NLL) + ((((NKC * NLK) + (((NHB * ATV) * AC) * NKB)) * AC) * NLJ));
                        NLO = MYH;
                        NLP = MYI;
                        NLQ = NKI;
                        NLR = NKQ;
                        NLS = NKZ;
                        NLT = NLD;
                        NLU = NLM;
                        NLV = MYO;
                        NLW = MYP;
                        NLX = NKH;
                        NLY = NKR;
                        NLZ = NLA;
                        NMA = NLE;
                        NMB = NLN;
                    }
                    NIJ = NLO;
                    NIK = NLP;
                    NIL = NLQ;
                    NIM = NLR;
                    NIN = NLS;
                    NIO = NLT;
                    NIP = NLU;
                    NIQ = NLV;
                    NIR = NLW;
                    NIS = NLX;
                    NIT = NLY;
                    NIU = NLZ;
                    NIV = NMA;
                    NIW = NMB;
                }
                let NIX = if NHA > AQJ { 1.0 } else { 0.0 };
                let NML;
                let NMM;
                let NMN;
                let NMO;
                if NIX != 0.0 {
                    let NMC = LA - NIJ;
                    let NMD = D - (NIJ * NMC);
                    let NME = (XQ * NHA) / NMD;
                    let NMF = ((NHB * XQ) - ((((NIQ * NMC) + ((NIQ * AC) * NIJ)) * AC) * NME)) / NMD;
                    let NMG = NME * NIJ;
                    let NMH = (NMF * NIJ) + (NIQ * NME);
                    let NMI = (NME.ln()) - NIK;
                    let NMJ = (NMF * (GY / NME)) - NIR;
                    NML = NMG;
                    NMM = NMI;
                    NMN = NMH;
                    NMO = NMJ;
                } else {
                    let NMK = if NHA < -5e-3f64 { 1.0 } else { 0.0 };
                    let NNG;
                    let NNH;
                    let NNI;
                    let NNJ;
                    if NMK != 0.0 {
                        let NMQ = GO * NIK;
                        let NMR = NMQ.sin();
                        let NMS = NMR * NMR;
                        let NMT = ((NIR * GO) * (NMQ.cos())) * NMR;
                        let NMU = (-NHA) / NMS;
                        let NMV = ((NHB * AC) - ((NMT + NMT) * NMU)) / NMS;
                        let NMW = NMU.ln();
                        let NMX = NMV * (GY / NMU);
                        NNG = NMU;
                        NNH = NMW;
                        NNI = NMV;
                        NNJ = NMX;
                    } else {
                        let NMY = NHA * WD;
                        let NMZ = ATB * NHA;
                        let NNA = D - (AVN * NHA);
                        let NNB = D - (NMZ * NNA);
                        let NNC = XQ - (NMY * NNB);
                        let NND = (((NHB * WD) * NNB) + (((((NHB * ATB) * NNA) + (((NHB * AVN) * AC) * NMZ)) * AC) * NMY)) * AC;
                        let NNE = NNC.ln();
                        let NNF = NND * (GY / NNC);
                        NNG = NNC;
                        NNH = NNE;
                        NNI = NND;
                        NNJ = NNF;
                    }
                    NML = NNG;
                    NMM = NNH;
                    NMN = NNI;
                    NMO = NNJ;
                }
                let NMP = if ((AVB * NGI) + NIL) > B { 1.0 } else { 0.0 };
                let NOF;
                let NOG;
                let NOH;
                let NOI;
                let NOJ;
                let NOK;
                if NMP != 0.0 {
                    let NNK = NGI + NIL;
                    let NNL = NGJ + NIS;
                    let NNM = JJR + NIM;
                    let NNN = JJT + NIT;
                    NOF = NNK;
                    NOG = NNM;
                    NOH = NIN;
                    NOI = NNL;
                    NOJ = NNN;
                    NOK = NIU;
                } else {
                    let NNO = NGI - NIL;
                    let NNP = D / NNO;
                    let NNQ = (((NGJ - NIS) * NNP) * AC) / NNO;
                    let NNR = NIM - JJR;
                    let NNS = NIT - JJT;
                    let NNT = NGX - NML;
                    let NNU = NNT * NNP;
                    let NNV = ((NGY - NMN) * NNP) + (NNQ * NNT);
                    let NNW = ((NNR * NNU) - NGX) - (NIO * NML);
                    let NNX = NNW * NNP;
                    let NNY = (((((NNS * NNU) + (NNV * NNR)) - NGY) - ((NIV * NML) + (NMN * NIO))) * NNP) + (NNQ * NNW);
                    let NNZ = LA * NNR;
                    let NOA = NIV * NIO;
                    let NOB = NIP + (NIO * NIO);
                    let NOC = (((NIN * NNU) + (NNZ * NNX)) + NGX) - (NOB * NML);
                    let NOD = NOC * NNP;
                    let NOE = ((((((NIU * NNU) + (NNV * NIN)) + (((NNS * LA) * NNX) + (NNY * NNZ))) + NGY) - (((NIW + (NOA + NOA)) * NML) + (NMN * NOB))) * NNP) + (NNQ * NOC);
                    NOF = NNU;
                    NOG = NNX;
                    NOH = NOD;
                    NOI = NNV;
                    NOJ = NNY;
                    NOK = NOE;
                }
                let NOL = if NOF > B { 1.0 } else { 0.0 };
                let NPF;
                let NPG;
                let NPH;
                let NPI;
                let NPJ;
                let NPK;
                if NOL != 0.0 {
                    let NOM = NOF.ln();
                    let NON = NOI * (GY / NOF);
                    let NOO = D / NOF;
                    let NOP = ((NOI * NOO) * AC) / NOF;
                    let NOQ = NOG * NOO;
                    let NOR = (NOJ * NOO) + (NOP * NOG);
                    let NOS = NOR * NOQ;
                    let NOT = (NOH * NOO) - (NOQ * NOQ);
                    let NOU = ((NOK * NOO) + (NOP * NOH)) - (NOS + NOS);
                    NPF = NOM;
                    NPG = NOQ;
                    NPH = NOT;
                    NPI = NON;
                    NPJ = NOR;
                    NPK = NOU;
                } else {
                    let NOV = -NGI;
                    let NOW = (NGI + HW) + (NOV.ln());
                    let NOX = NGJ + ((NGJ * AC) * (GY / NOV));
                    let NOY = D / NGG;
                    let NOZ = ((NGH * NOY) * AC) / NGG;
                    let NPA = JJR + NOY;
                    let NPB = JJT + NOZ;
                    let NPC = -NOY;
                    let NPD = NPC * NOY;
                    let NPE = ((NOZ * AC) * NOY) + (NOZ * NPC);
                    NPF = NOW;
                    NPG = NPA;
                    NPH = NPD;
                    NPI = NOX;
                    NPJ = NPB;
                    NPK = NPE;
                }
                let NPL = ((JYJ + NGG) + (LA * NPF)) - NMM;
                let NPM = (D + (LA * NPG)) - NIO;
                let NPN = (LA * NPH) - NIP;
                let NPO = NGI + (JJU * NPL);
                let NPP = NGJ + ((JJW * NPL) + ((((JYK + NGH) + (NPI * LA)) - NMO) * JJU));
                let NPQ = JJR + (JJU * NPM);
                let NPR = JJT + ((JJW * NPM) + (((NPJ * LA) - NIV) * JJU));
                let NPS = JJU * NPN;
                let NPT = (NPO * NOF) - NGX;
                let NPU = ((NPP * NOF) + (NOI * NPO)) - NGY;
                let NPV = ((NPQ * NOF) + (NPO * NOG)) + NGX;
                let NPW = (((NPR * NOF) + (NOI * NPQ)) + ((NPP * NOG) + (NOJ * NPO))) + NGY;
                let NPX = LA * NPQ;
                let NPY = (((NPS * NOF) + (NPX * NOG)) + (NPO * NOH)) - NGX;
                let NPZ = NPW * NPV;
                let NQA = GO * NPT;
                let NQB = (NPV * NPV) - (NQA * NPY);
                let NQC = (NPZ + NPZ) - (((NPU * GO) * NPY) + ((((((((JJW * NPN) + (((NPK * LA) - NIW) * JJU)) * NOF) + (NOI * NPS)) + (((NPR * LA) * NOG) + (NOJ * NPX))) + ((NPP * NOH) + (NOK * NPO))) - NGY) * NQA));
                let NQD = -NPT;
                let NQE = NQD * NPV;
                let NQF = NQC * NQB;
                let NQG = (NQB * NQB) + AYW;
                let NQH = (NQE * NQB) / NQG;
                let NQI = NGG + NQH;
                let NQJ = NGH + (((((((NPU * AC) * NPV) + (NPW * NQD)) * NQB) + (NQC * NQE)) - ((NQF + NQF) * NQH)) / NQG);
                let NQL;
                let NQM;
                let NQN;
                let NQO;
                let NQP;
                let NQQ;
                if A != 0.0 {
                    let NQK = if (NQH.abs()) > NI { 1.0 } else { 0.0 };
                    let NRB;
                    let NRC;
                    let NRD;
                    let NRE;
                    let NRF;
                    let NRG;
                    if NQK != 0.0 {
                        let NQW = JJR * NQI;
                        let NQX = (JJT * NQI) + (NQJ * JJR);
                        let NQY = (JJI - NQI) - MFE;
                        let NQZ = (JJJ - NQJ) - MFG;
                        let NRA = if NQY < SY { 1.0 } else { 0.0 };
                        let NRP;
                        let NRQ;
                        if NRA != 0.0 {
                            let NRH = NQY.exp();
                            let NRI = NQZ * NRH;
                            NRP = NRH;
                            NRQ = NRI;
                        } else {
                            let NRJ = NQY - SY;
                            let NRK = GO * NRJ;
                            let NRL = D + (NRJ * WD);
                            let NRM = D + (NRK * NRL);
                            let NRN = XB * (D + (NRJ * NRM));
                            let NRO = ((NQZ * NRM) + ((((NQZ * GO) * NRL) + ((NQZ * WD) * NRK)) * NRJ)) * XB;
                            NRP = NRN;
                            NRQ = NRO;
                        }
                        let NRR = JKG * NRP;
                        let NRS = (JKH * NRP) + (NRQ * JKG);
                        let NRT = NQX * NQW;
                        let NRU = (NQW * NQW) - NRR;
                        let NRV = (NRT + NRT) - NRS;
                        let NRW = (JPW * NQW) + NRR;
                        let NRX = ((JPX * NQW) + (NQX * JPW)) + NRS;
                        let NRY = JQA - NRR;
                        let NRZ = JQB - NRS;
                        let NSA = if NRU < -5e-3f64 { 1.0 } else { 0.0 };
                        let NTD;
                        let NTE;
                        let NTF;
                        let NTG;
                        let NTH;
                        let NTI;
                        let NTJ;
                        let NTK;
                        let NTL;
                        let NTM;
                        let NTN;
                        let NTO;
                        let NTP;
                        let NTQ;
                        if NSA != 0.0 {
                            let NSB = (NRU.abs()).sqrt();
                            let NSC = (NRV * ((GX * (if NRU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * NSB));
                            let NSD = GO * NSB;
                            let NSE = NSD.tan();
                            let NSF = NSD.cos();
                            let NSG = NSB / NSE;
                            let NSH = (NSC - (((NSC * GO) * (GY / (NSF * NSF))) * NSG)) / NSE;
                            let NSI = (YY * NRW) / NRU;
                            let NSJ = ((NRX * YY) - (NRV * NSI)) / NRU;
                            let NSK = LA - NSG;
                            let NSL = NRU + (NSG * NSK);
                            let NSM = NSL * NSI;
                            let NSN = ((NRV + ((NSH * NSK) + ((NSH * AC) * NSG))) * NSI) + (NSJ * NSL);
                            let NSO = LA * NSM;
                            let NSP = D + NSG;
                            let NSQ = NRW - (NSO * NSP);
                            let NSR = (NSM * NRY) / NRW;
                            let NSS = (NSQ * NSI) + NSR;
                            let NST = (((NRX - (((NSN * LA) * NSP) + (NSH * NSO))) * NSI) + (NSJ * NSQ)) + ((((NSN * NRY) + (NRZ * NSM)) - (NRX * NSR)) / NRW);
                            let NSU = D - (GO * NSG);
                            let NSV = (NSH * GO) * AC;
                            let NSW = NRW / NRU;
                            let NSX = NSW * NSU;
                            let NSY = (((NRX - (NRV * NSW)) / NRU) * NSU) + (NSV * NSW);
                            let NSZ = NSX + (GO * NSM);
                            let NTA = ((NRY * NSU) - (NRW * NSZ)) / NRU;
                            let NTB = ((((NRZ * NSU) + (NSV * NRY)) - ((NRX * NSZ) + ((NSY + (NSN * GO)) * NRW))) - (NRV * NTA)) / NRU;
                            NTD = NIJ;
                            NTE = NSB;
                            NTF = NSG;
                            NTG = NSM;
                            NTH = NSS;
                            NTI = NSX;
                            NTJ = NTA;
                            NTK = NIQ;
                            NTL = NSC;
                            NTM = NSH;
                            NTN = NSN;
                            NTO = NST;
                            NTP = NSY;
                            NTQ = NTB;
                        } else {
                            let NTC = if NRU > AQJ { 1.0 } else { 0.0 };
                            let NWI;
                            let NWJ;
                            let NWK;
                            let NWL;
                            let NWM;
                            let NWN;
                            let NWO;
                            let NWP;
                            let NWQ;
                            let NWR;
                            let NWS;
                            let NWT;
                            let NWU;
                            let NWV;
                            if NTC != 0.0 {
                                let NTS = (NRU.abs()).sqrt();
                                let NTT = (NRV * ((GX * (if NRU >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * NTS));
                                let NTU = (-NTS).exp();
                                let NTV = (NTT * AC) * NTU;
                                let NTW = D + NTU;
                                let NTX = D - NTU;
                                let NTY = (NTS * NTW) / NTX;
                                let NTZ = (((NTT * NTW) + (NTV * NTS)) - ((NTV * AC) * NTY)) / NTX;
                                let NUA = (YY * NRW) / NRU;
                                let NUB = ((NRX * YY) - (NRV * NUA)) / NRU;
                                let NUC = LA - NTY;
                                let NUD = NRU + (NTY * NUC);
                                let NUE = NUD * NUA;
                                let NUF = ((NRV + ((NTZ * NUC) + ((NTZ * AC) * NTY))) * NUA) + (NUB * NUD);
                                let NUG = LA * NUE;
                                let NUH = D + NTY;
                                let NUI = NRW - (NUG * NUH);
                                let NUJ = (NUE * NRY) / NRW;
                                let NUK = (NUI * NUA) + NUJ;
                                let NUL = (((NRX - (((NUF * LA) * NUH) + (NTZ * NUG))) * NUA) + (NUB * NUI)) + ((((NUF * NRY) + (NRZ * NUE)) - (NRX * NUJ)) / NRW);
                                let NUM = D - (GO * NTY);
                                let NUN = (NTZ * GO) * AC;
                                let NUO = NRW / NRU;
                                let NUP = NUO * NUM;
                                let NUQ = (((NRX - (NRV * NUO)) / NRU) * NUM) + (NUN * NUO);
                                let NUR = NUP + (GO * NUE);
                                let NUS = ((NRY * NUM) - (NRW * NUR)) / NRU;
                                let NUT = ((((NRZ * NUM) + (NUN * NRY)) - ((NRX * NUR) + ((NUQ + (NUF * GO)) * NRW))) - (NRV * NUS)) / NRU;
                                NWI = NTU;
                                NWJ = NTS;
                                NWK = NTY;
                                NWL = NUE;
                                NWM = NUK;
                                NWN = NUP;
                                NWO = NUS;
                                NWP = NTV;
                                NWQ = NTT;
                                NWR = NTZ;
                                NWS = NUF;
                                NWT = NUL;
                                NWU = NUQ;
                                NWV = NUT;
                            } else {
                                let NUU = NRU * ASC;
                                let NUV = NRU * ASE;
                                let NUW = NRV * ASE;
                                let NUX = D - (NRU * ASH);
                                let NUY = D - (NUV * NUX);
                                let NUZ = UC * (D - (NUU * NUY));
                                let NVA = ((((NRV * ASC) * NUY) + ((((NUW * NUX) + (((NRV * ASH) * AC) * NUV)) * AC) * NUU)) * AC) * UC;
                                let NVB = (NRV * NUZ) + (NVA * NRU);
                                let NVC = LA + (NRU * NUZ);
                                let NVD = NRU * ASO;
                                let NVE = NRV * ASO;
                                let NVF = NRU * ASR;
                                let NVG = D - NVD;
                                let NVH = D - (NVF * NVG);
                                let NVI = UC * (D - (NVD * NVH));
                                let NVJ = (((NVE * NVH) + (((((NRV * ASR) * NVG) + ((NVE * AC) * NVF)) * AC) * NVD)) * AC) * UC;
                                let NVK = NRW * NVI;
                                let NVL = (NRX * NVI) + (NVJ * NRW);
                                let NVM = NRU * ASZ;
                                let NVN = ATB * NRU;
                                let NVO = D - (ATD * NRU);
                                let NVP = D - (NVN * NVO);
                                let NVQ = ATG * (D - (NVM * NVP));
                                let NVR = NRW * NRW;
                                let NVS = NRX * NRW;
                                let NVT = (NRY * NVI) - (NVR * NVQ);
                                let NVU = ((NRZ * NVI) + (NVJ * NRY)) - (((NVS + NVS) * NVQ) + ((((((NRV * ASZ) * NVP) + (((((NRV * ATB) * NVO) + (((NRV * ATD) * AC) * NVN)) * AC) * NVM)) * AC) * ATG) * NVR));
                                let NVW = NVV * NRW;
                                let NVX = NVW * NUZ;
                                let NVY = ((NRX * NVV) * NUZ) + (NVA * NVW);
                                let NWA = NVZ * NRY;
                                let NWC = NWB * NRW;
                                let NWD = NWC * NRW;
                                let NWE = LA - (ATV * NRU);
                                let NWF = D - (NUV * NWE);
                                let NWG = (NWA * NUZ) + (NWD * NWF);
                                let NWH = (((NRZ * NVZ) * NUZ) + (NVA * NWA)) + (((((NRX * NWB) * NRW) + (NRX * NWC)) * NWF) + ((((NUW * NWE) + (((NRV * ATV) * AC) * NUV)) * AC) * NWD));
                                NWI = NIJ;
                                NWJ = NIK;
                                NWK = NVC;
                                NWL = NVK;
                                NWM = NVT;
                                NWN = NVX;
                                NWO = NWG;
                                NWP = NIQ;
                                NWQ = NIR;
                                NWR = NVB;
                                NWS = NVL;
                                NWT = NVU;
                                NWU = NVY;
                                NWV = NWH;
                            }
                            NTD = NWI;
                            NTE = NWJ;
                            NTF = NWK;
                            NTG = NWL;
                            NTH = NWM;
                            NTI = NWN;
                            NTJ = NWO;
                            NTK = NWP;
                            NTL = NWQ;
                            NTM = NWR;
                            NTN = NWS;
                            NTO = NWT;
                            NTP = NWU;
                            NTQ = NWV;
                        }
                        let NTR = if NRU > AQJ { 1.0 } else { 0.0 };
                        let NXF;
                        let NXG;
                        let NXH;
                        let NXI;
                        if NTR != 0.0 {
                            let NWW = LA - NTD;
                            let NWX = D - (NTD * NWW);
                            let NWY = (XQ * NRU) / NWX;
                            let NWZ = ((NRV * XQ) - ((((NTK * NWW) + ((NTK * AC) * NTD)) * AC) * NWY)) / NWX;
                            let NXA = NWY * NTD;
                            let NXB = (NWZ * NTD) + (NTK * NWY);
                            let NXC = (NWY.ln()) - NTE;
                            let NXD = (NWZ * (GY / NWY)) - NTL;
                            NXF = NXA;
                            NXG = NXC;
                            NXH = NXB;
                            NXI = NXD;
                        } else {
                            let NXE = if NRU < -5e-3f64 { 1.0 } else { 0.0 };
                            let NYA;
                            let NYB;
                            let NYC;
                            let NYD;
                            if NXE != 0.0 {
                                let NXK = GO * NTE;
                                let NXL = NXK.sin();
                                let NXM = NXL * NXL;
                                let NXN = ((NTL * GO) * (NXK.cos())) * NXL;
                                let NXO = (-NRU) / NXM;
                                let NXP = ((NRV * AC) - ((NXN + NXN) * NXO)) / NXM;
                                let NXQ = NXO.ln();
                                let NXR = NXP * (GY / NXO);
                                NYA = NXO;
                                NYB = NXQ;
                                NYC = NXP;
                                NYD = NXR;
                            } else {
                                let NXS = NRU * WD;
                                let NXT = ATB * NRU;
                                let NXU = D - (AVN * NRU);
                                let NXV = D - (NXT * NXU);
                                let NXW = XQ - (NXS * NXV);
                                let NXX = (((NRV * WD) * NXV) + (((((NRV * ATB) * NXU) + (((NRV * AVN) * AC) * NXT)) * AC) * NXS)) * AC;
                                let NXY = NXW.ln();
                                let NXZ = NXX * (GY / NXW);
                                NYA = NXW;
                                NYB = NXY;
                                NYC = NXX;
                                NYD = NXZ;
                            }
                            NXF = NYA;
                            NXG = NYB;
                            NXH = NYC;
                            NXI = NYD;
                        }
                        let NXJ = if ((AVB * NQW) + NTF) > B { 1.0 } else { 0.0 };
                        let NYZ;
                        let NZA;
                        let NZB;
                        let NZC;
                        let NZD;
                        let NZE;
                        if NXJ != 0.0 {
                            let NYE = NQW + NTF;
                            let NYF = NQX + NTM;
                            let NYG = JJR + NTG;
                            let NYH = JJT + NTN;
                            NYZ = NYE;
                            NZA = NYG;
                            NZB = NTH;
                            NZC = NYF;
                            NZD = NYH;
                            NZE = NTO;
                        } else {
                            let NYI = NQW - NTF;
                            let NYJ = D / NYI;
                            let NYK = (((NQX - NTM) * NYJ) * AC) / NYI;
                            let NYL = NTG - JJR;
                            let NYM = NTN - JJT;
                            let NYN = NRR - NXF;
                            let NYO = NYN * NYJ;
                            let NYP = ((NRS - NXH) * NYJ) + (NYK * NYN);
                            let NYQ = ((NYL * NYO) - NRR) - (NTI * NXF);
                            let NYR = NYQ * NYJ;
                            let NYS = (((((NYM * NYO) + (NYP * NYL)) - NRS) - ((NTP * NXF) + (NXH * NTI))) * NYJ) + (NYK * NYQ);
                            let NYT = LA * NYL;
                            let NYU = NTP * NTI;
                            let NYV = NTJ + (NTI * NTI);
                            let NYW = (((NTH * NYO) + (NYT * NYR)) + NRR) - (NYV * NXF);
                            let NYX = NYW * NYJ;
                            let NYY = ((((((NTO * NYO) + (NYP * NTH)) + (((NYM * LA) * NYR) + (NYS * NYT))) + NRS) - (((NTQ + (NYU + NYU)) * NXF) + (NXH * NYV))) * NYJ) + (NYK * NYW);
                            NYZ = NYO;
                            NZA = NYR;
                            NZB = NYX;
                            NZC = NYP;
                            NZD = NYS;
                            NZE = NYY;
                        }
                        let NZF = if NYZ > B { 1.0 } else { 0.0 };
                        let NZZ;
                        let OAA;
                        let OAB;
                        let OAC;
                        let OAD;
                        let OAE;
                        if NZF != 0.0 {
                            let NZG = NYZ.ln();
                            let NZH = NZC * (GY / NYZ);
                            let NZI = D / NYZ;
                            let NZJ = ((NZC * NZI) * AC) / NYZ;
                            let NZK = NZA * NZI;
                            let NZL = (NZD * NZI) + (NZJ * NZA);
                            let NZM = NZL * NZK;
                            let NZN = (NZB * NZI) - (NZK * NZK);
                            let NZO = ((NZE * NZI) + (NZJ * NZB)) - (NZM + NZM);
                            NZZ = NZG;
                            OAA = NZK;
                            OAB = NZN;
                            OAC = NZH;
                            OAD = NZL;
                            OAE = NZO;
                        } else {
                            let NZP = -NQW;
                            let NZQ = (NQW + HW) + (NZP.ln());
                            let NZR = NQX + ((NQX * AC) * (GY / NZP));
                            let NZS = D / NQI;
                            let NZT = ((NQJ * NZS) * AC) / NQI;
                            let NZU = JJR + NZS;
                            let NZV = JJT + NZT;
                            let NZW = -NZS;
                            let NZX = NZW * NZS;
                            let NZY = ((NZT * AC) * NZS) + (NZT * NZW);
                            NZZ = NZQ;
                            OAA = NZU;
                            OAB = NZX;
                            OAC = NZR;
                            OAD = NZV;
                            OAE = NZY;
                        }
                        let OAF = ((JYJ + NQI) + (LA * NZZ)) - NXG;
                        let OAG = (D + (LA * OAA)) - NTI;
                        let OAH = (LA * OAB) - NTJ;
                        let OAI = NQW + (JJU * OAF);
                        let OAJ = NQX + ((JJW * OAF) + ((((JYK + NQJ) + (OAC * LA)) - NXI) * JJU));
                        let OAK = JJR + (JJU * OAG);
                        let OAL = JJT + ((JJW * OAG) + (((OAD * LA) - NTP) * JJU));
                        let OAM = JJU * OAH;
                        let OAN = (OAI * NYZ) - NRR;
                        let OAO = ((OAJ * NYZ) + (NZC * OAI)) - NRS;
                        let OAP = ((OAK * NYZ) + (OAI * NZA)) + NRR;
                        let OAQ = (((OAL * NYZ) + (NZC * OAK)) + ((OAJ * NZA) + (NZD * OAI))) + NRS;
                        let OAR = LA * OAK;
                        let OAS = (((OAM * NYZ) + (OAR * NZA)) + (OAI * NZB)) - NRR;
                        let OAT = OAQ * OAP;
                        let OAU = GO * OAN;
                        let OAV = (OAP * OAP) - (OAU * OAS);
                        let OAW = (OAT + OAT) - (((OAO * GO) * OAS) + ((((((((JJW * OAH) + (((OAE * LA) - NTQ) * JJU)) * NYZ) + (NZC * OAM)) + (((OAL * LA) * NZA) + (NZD * OAR))) + ((OAJ * NZB) + (NZE * OAI))) - NRS) * OAU));
                        let OAX = -OAN;
                        let OAY = OAX * OAP;
                        let OAZ = OAW * OAV;
                        let OBA = (OAV * OAV) + AYW;
                        let OBB = (OAY * OAV) / OBA;
                        let OBC = NQI + OBB;
                        let OBD = NQJ + (((((((OAO * AC) * OAP) + (OAQ * OAX)) * OAV) + (OAW * OAY)) - ((OAZ + OAZ) * OBB)) / OBA);
                        NRB = OBC;
                        NRC = NTD;
                        NRD = NTE;
                        NRE = OBD;
                        NRF = NTK;
                        NRG = NTL;
                    } else {
                        NRB = NQI;
                        NRC = NIJ;
                        NRD = NIK;
                        NRE = NQJ;
                        NRF = NIQ;
                        NRG = NIR;
                    }
                    NQL = NRB;
                    NQM = NRC;
                    NQN = NRD;
                    NQO = NRE;
                    NQP = NRF;
                    NQQ = NRG;
                } else {
                    NQL = NQI;
                    NQM = NIJ;
                    NQN = NIK;
                    NQO = NQJ;
                    NQP = NIQ;
                    NQQ = NIR;
                }
                let NQR = JJR * NQL;
                let NQS = (JJT * NQL) + (NQO * JJR);
                let NQT = (JJI - NQL) - MFE;
                let NQU = (JJJ - NQO) - MFG;
                let NQV = if NQT < SY { 1.0 } else { 0.0 };
                let OBM;
                let OBN;
                if NQV != 0.0 {
                    let OBE = NQT.exp();
                    let OBF = NQU * OBE;
                    OBM = OBE;
                    OBN = OBF;
                } else {
                    let OBG = NQT - SY;
                    let OBH = GO * OBG;
                    let OBI = D + (OBG * WD);
                    let OBJ = D + (OBH * OBI);
                    let OBK = XB * (D + (OBG * OBJ));
                    let OBL = ((NQU * OBJ) + ((((NQU * GO) * OBI) + ((NQU * WD) * OBH)) * OBG)) * XB;
                    OBM = OBK;
                    OBN = OBL;
                }
                let OBO = JKG * OBM;
                let OBP = (JKH * OBM) + (OBN * JKG);
                let OBQ = NQS * NQR;
                let OBR = (NQR * NQR) - OBO;
                let OBS = (OBQ + OBQ) - OBP;
                let OBT = if OBO <= B { 1.0 } else { 0.0 };
                let OBZ;
                let OCA;
                let OCB;
                let OCC;
                let OCD;
                let OCE;
                if OBT != 0.0 {
                    let OBU = CJQ - NQR;
                    let OBV = NQS * AC;
                    let OBW = OBU / JJU;
                    let OBX = (OBV - (JJW * OBW)) / JJU;
                    OBZ = OBW;
                    OCA = OBU;
                    OCB = CJQ;
                    OCC = OBX;
                    OCD = OBV;
                    OCE = AFD;
                } else {
                    let OBY = if OBR < -5e-3f64 { 1.0 } else { 0.0 };
                    let OCQ;
                    let OCR;
                    let OCS;
                    let OCT;
                    let OCU;
                    let OCV;
                    if OBY != 0.0 {
                        let OCI = (OBR.abs()).sqrt();
                        let OCJ = (OBS * ((GX * (if OBR >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * OCI));
                        let OCK = GO * OCI;
                        let OCL = OCK.tan();
                        let OCM = OCK.cos();
                        let OCN = OCI / OCL;
                        let OCO = (OCJ - (((OCJ * GO) * (GY / (OCM * OCM))) * OCN)) / OCL;
                        OCQ = OCN;
                        OCR = NQM;
                        OCS = OCI;
                        OCT = OCO;
                        OCU = NQP;
                        OCV = OCJ;
                    } else {
                        let OCP = if OBR > AQJ { 1.0 } else { 0.0 };
                        let ODL;
                        let ODM;
                        let ODN;
                        let ODO;
                        let ODP;
                        let ODQ;
                        if OCP != 0.0 {
                            let OCX = (OBR.abs()).sqrt();
                            let OCY = (OBS * ((GX * (if OBR >= APH { 1.0 } else { 0.0 })) - GY)) * (GY / (GX * OCX));
                            let OCZ = (-OCX).exp();
                            let ODA = (OCY * AC) * OCZ;
                            let ODB = D + OCZ;
                            let ODC = D - OCZ;
                            let ODD = (OCX * ODB) / ODC;
                            let ODE = (((OCY * ODB) + (ODA * OCX)) - ((ODA * AC) * ODD)) / ODC;
                            ODL = ODD;
                            ODM = OCZ;
                            ODN = OCX;
                            ODO = ODE;
                            ODP = ODA;
                            ODQ = OCY;
                        } else {
                            let ODF = OBR * UC;
                            let ODG = OBR * ASC;
                            let ODH = D - (OBR * ASE);
                            let ODI = D - (ODG * ODH);
                            let ODJ = ((OBS * UC) * ODI) + (((((OBS * ASC) * ODH) + (((OBS * ASE) * AC) * ODG)) * AC) * ODF);
                            let ODK = LA + (ODF * ODI);
                            ODL = ODK;
                            ODM = NQM;
                            ODN = NQN;
                            ODO = ODJ;
                            ODP = NQP;
                            ODQ = NQQ;
                        }
                        OCQ = ODL;
                        OCR = ODM;
                        OCS = ODN;
                        OCT = ODO;
                        OCU = ODP;
                        OCV = ODQ;
                    }
                    let OCW = if ((AVB * NQR) + OCQ) > B { 1.0 } else { 0.0 };
                    let ODV;
                    let ODW;
                    let ODX;
                    let ODY;
                    let ODZ;
                    let OEA;
                    if OCW != 0.0 {
                        let ODR = NQR + OCQ;
                        let ODS = NQS + OCT;
                        let ODT = if (OBO * NQR) < (((CLQ * NQR) * NQR) * ODR) { 1.0 } else { 0.0 };
                        let OEJ;
                        let OEK;
                        let OEL;
                        let OEM;
                        let OEN;
                        let OEO;
                        if ODT != 0.0 {
                            let OEB = OBO / ODR;
                            let OEC = (OBP - (ODS * OEB)) / ODR;
                            let OED = OEB + CJQ;
                            let OEE = OED - NQR;
                            let OEF = OEC - NQS;
                            let OEG = OEE / JJU;
                            let OEH = (OEF - (JJW * OEG)) / JJU;
                            OEJ = OEG;
                            OEK = OEE;
                            OEL = OED;
                            OEM = OEH;
                            OEN = OEF;
                            OEO = OEC;
                        } else {
                            let OEI = if OBR > AQJ { 1.0 } else { 0.0 };
                            let OEV;
                            let OEW;
                            if OEI != 0.0 {
                                let OEP = LA - OCR;
                                let OEQ = D - (OCR * OEP);
                                let OER = (XQ * OBR) / OEQ;
                                let OES = (OER.ln()) - OCS;
                                let OET = ((((OBS * XQ) - ((((OCU * OEP) + ((OCU * AC) * OCR)) * AC) * OER)) / OEQ) * (GY / OER)) - OCV;
                                OEV = OES;
                                OEW = OET;
                            } else {
                                let OEU = if OBR < -5e-3f64 { 1.0 } else { 0.0 };
                                let OFR;
                                let OFS;
                                if OEU != 0.0 {
                                    let OFD = GO * OCS;
                                    let OFE = OFD.sin();
                                    let OFF = OFE * OFE;
                                    let OFG = ((OCV * GO) * (OFD.cos())) * OFE;
                                    let OFH = (-OBR) / OFF;
                                    let OFI = OFH.ln();
                                    let OFJ = (((OBS * AC) - ((OFG + OFG) * OFH)) / OFF) * (GY / OFH);
                                    OFR = OFI;
                                    OFS = OFJ;
                                } else {
                                    let OFK = OBR * WD;
                                    let OFL = ATB * OBR;
                                    let OFM = D - (AVN * OBR);
                                    let OFN = D - (OFL * OFM);
                                    let OFO = XQ - (OFK * OFN);
                                    let OFP = OFO.ln();
                                    let OFQ = ((((OBS * WD) * OFN) + (((((OBS * ATB) * OFM) + (((OBS * AVN) * AC) * OFL)) * AC) * OFK)) * AC) * (GY / OFO);
                                    OFR = OFP;
                                    OFS = OFQ;
                                }
                                OEV = OFR;
                                OEW = OFS;
                            }
                            let OEX = ((JYJ + NQL) + (LA * (ODR.ln()))) - OEV;
                            let OEY = ((JYK + NQO) + ((ODS * (GY / ODR)) * LA)) - OEW;
                            let OEZ = JJU * OEX;
                            let OFA = (JJW * OEX) + (OEY * JJU);
                            let OFB = NQR + OEZ;
                            let OFC = NQS + OFA;
                            OEJ = OEX;
                            OEK = OEZ;
                            OEL = OFB;
                            OEM = OEY;
                            OEN = OFA;
                            OEO = OFC;
                        }
                        ODV = OEJ;
                        ODW = OEK;
                        ODX = OEL;
                        ODY = OEM;
                        ODZ = OEN;
                        OEA = OEO;
                    } else {
                        let ODU = if OBR > AQJ { 1.0 } else { 0.0 };
                        let OFX;
                        let OFY;
                        if ODU != 0.0 {
                            let OFT = ((NQL + MFE) - JJI) - OCS;
                            let OFU = ((NQO + MFG) - JJJ) - OCV;
                            let OFV = if OFT < SY { 1.0 } else { 0.0 };
                            let OGP;
                            let OGQ;
                            if OFV != 0.0 {
                                let OGH = OFT.exp();
                                let OGI = OFU * OGH;
                                OGP = OGH;
                                OGQ = OGI;
                            } else {
                                let OGJ = OFT - SY;
                                let OGK = GO * OGJ;
                                let OGL = D + (OGJ * WD);
                                let OGM = D + (OGK * OGL);
                                let OGN = XB * (D + (OGJ * OGM));
                                let OGO = ((OFU * OGM) + ((((OFU * GO) * OGL) + ((OFU * WD) * OGK)) * OGJ)) * XB;
                                OGP = OGN;
                                OGQ = OGO;
                            }
                            let OGR = OGP / JKG;
                            let OGS = XQ * OBR;
                            let OGT = LA - OCR;
                            let OGU = D - (OCR * OGT);
                            let OGV = (OGS * OGR) / OGU;
                            let OGW = ((((OBS * XQ) * OGR) + (((OGQ - (JKH * OGR)) / JKG) * OGS)) - ((((OCU * OGT) + ((OCU * AC) * OCR)) * AC) * OGV)) / OGU;
                            OFX = OGV;
                            OFY = OGW;
                        } else {
                            let OFW = if OBR < -5e-3f64 { 1.0 } else { 0.0 };
                            let OHK;
                            let OHL;
                            if OFW != 0.0 {
                                let OGX = GO * OCS;
                                let OGY = OGX.sin();
                                let OGZ = OGY * OGY;
                                let OHA = ((OCV * GO) * (OGX.cos())) * OGY;
                                let OHB = (-OBR) / OGZ;
                                let OHC = OHB / OBO;
                                let OHD = ((((OBS * AC) - ((OHA + OHA) * OHB)) / OGZ) - (OBP * OHC)) / OBO;
                                OHK = OHC;
                                OHL = OHD;
                            } else {
                                let OHE = OBR * WD;
                                let OHF = ATB * OBR;
                                let OHG = D - (AVN * OBR);
                                let OHH = D - (OHF * OHG);
                                let OHI = (XQ - (OHE * OHH)) / OBO;
                                let OHJ = (((((OBS * WD) * OHH) + (((((OBS * ATB) * OHG) + (((OBS * AVN) * AC) * OHF)) * AC) * OHE)) * AC) - (OBP * OHI)) / OBO;
                                OHK = OHI;
                                OHL = OHJ;
                            }
                            OFX = OHK;
                            OFY = OHL;
                        }
                        let OFZ = D - OFX;
                        let OGA = (NQR - OCQ) / OFZ;
                        let OGB = ((NQS - OCT) - ((OFY * AC) * OGA)) / OFZ;
                        let OGC = OGA + CJQ;
                        let OGD = OGC - NQR;
                        let OGE = OGB - NQS;
                        let OGF = OGD / JJU;
                        let OGG = (OGE - (JJW * OGF)) / JJU;
                        ODV = OGF;
                        ODW = OGD;
                        ODX = OGC;
                        ODY = OGG;
                        ODZ = OGE;
                        OEA = OGB;
                    }
                    OBZ = ODV;
                    OCA = ODW;
                    OCB = ODX;
                    OCC = ODY;
                    OCD = ODZ;
                    OCE = OEA;
                }
                let OCF = (JJP - OBZ) - MFE;
                let OCG = (JJQ - OCC) - MFG;
                let OCH = if OCF < SY { 1.0 } else { 0.0 };
                let OHU;
                let OHV;
                if OCH != 0.0 {
                    let OHM = OCF.exp();
                    let OHN = OCG * OHM;
                    OHU = OHM;
                    OHV = OHN;
                } else {
                    let OHO = OCF - SY;
                    let OHP = GO * OHO;
                    let OHQ = D + (OHO * WD);
                    let OHR = D + (OHP * OHQ);
                    let OHS = XB * (D + (OHO * OHR));
                    let OHT = ((OCG * OHR) + ((((OCG * GO) * OHQ) + ((OCG * WD) * OHP)) * OHO)) * XB;
                    OHU = OHS;
                    OHV = OHT;
                }
                let OHW = JKG * OHU;
                let OHX = (JKH * OHU) + (OHV * JKG);
                let OIJ;
                let OIK;
                let OIL;
                let OIM;
                let OIN;
                let OIO;
                let OIP;
                let OIQ;
                if LPZ != 0.0 {
                    let OHY = OBO * JJX;
                    let OHZ = (OBP * JJX) + (JJY * OBO);
                    let OIA = OHW * JJZ;
                    let OIB = (OHX * JJZ) + (JKA * OHW);
                    let OIC = OHY + (LA * NQR);
                    let OID = OHZ + (NQS * LA);
                    let OIE = OIA + (LA * OCA);
                    let OIF = OIB + (OCD * LA);
                    let OIG = ((LA * OCB) + OHY) + OIA;
                    let OIH = ((OCE * LA) + OHZ) + OIB;
                    let OII = if (OBR.abs()) > AQJ { 1.0 } else { 0.0 };
                    let OJT;
                    let OJU;
                    if OII != 0.0 {
                        let OIX = LA * (NQL + LA);
                        let OIY = LA * (OBZ + LA);
                        let OIZ = ((OIC * OIE) + (OIX * OIE)) + (OIY * OIC);
                        let OJB = OJA * OBR;
                        let OJC = OCB * OIZ;
                        let OJD = (OJB * OIG) / OJC;
                        let OJE = ((((OBS * OJA) * OIG) + (OIH * OJB)) - (((OCE * OIZ) + (((((OID * OIE) + (OIF * OIC)) + (((NQO * LA) * OIE) + (OIF * OIX))) + (((OCC * LA) * OIC) + (OID * OIY))) * OCB)) * OJD)) / OJC;
                        OJT = OJD;
                        OJU = OJE;
                    } else {
                        let OJF = OBR * ASO;
                        let OJG = OBS * ASO;
                        let OJH = OBR * ASR;
                        let OJI = D - OJF;
                        let OJJ = D - (OJH * OJI);
                        let OJK = UC * (D - (OJF * OJJ));
                        let OJL = OIC * OIE;
                        let OJM = OJL * OCB;
                        let OJN = D + (OCB * OJK);
                        let OJO = ((OIC * OBO) + (OIE * OHW)) + (OJM * OJN);
                        let OJP = OBO * OHW;
                        let OJQ = OCB * OJO;
                        let OJR = (OJP * OIG) / OJQ;
                        let OJS = (((((OBP * OHW) + (OHX * OBO)) * OIG) + (OIH * OJP)) - (((OCE * OJO) + (((((OID * OBO) + (OBP * OIC)) + ((OIF * OHW) + (OHX * OIE))) + ((((((OID * OIE) + (OIF * OIC)) * OCB) + (OCE * OJL)) * OJN) + (((OCE * OJK) + (((((OJG * OJJ) + (((((OBS * ASR) * OJI) + ((OJG * AC) * OJH)) * AC) * OJF)) * AC) * UC) * OCB)) * OJM))) * OCB)) * OJR)) / OJQ;
                        OJT = OJR;
                        OJU = OJS;
                    }
                    OIJ = OIE;
                    OIK = OIC;
                    OIL = OJT;
                    OIM = OIG;
                    OIN = OIF;
                    OIO = OID;
                    OIP = OJU;
                    OIQ = OIH;
                } else {
                    OIJ = B;
                    OIK = B;
                    OIL = B;
                    OIM = B;
                    OIN = AFD;
                    OIO = AFD;
                    OIP = AFD;
                    OIQ = AFD;
                }
                let OIR = MFE + (OCB.ln());
                let OIS = MFG + (OCE * (GY / OCB));
                let OIT = GO * (LKB + OCB);
                let OIU = (LKE + OCE) * GO;
                let OIV = OIR - LQT;
                let OIW = OIS - LQU;
                let OKJ;
                let OKK;
                if K != 0.0 {
                    let OJV = (GO * (KYS + NQR)) / JJR;
                    let OJW = (((KYT + NQS) * GO) - (JJT * OJV)) / JJR;
                    let OJX = OJV - FLA;
                    let OJY = OJW * OJX;
                    let OJZ = ((OJX * OJX) + D).sqrt();
                    let OKA = GO * ((OJV + FLA) + OJZ);
                    let OKB = (OJW + ((OJY + OJY) * (GY / (GX * OJZ)))) * GO;
                    let OKC = OKA / CH;
                    let OKD = (OKC + staged[256]).sqrt();
                    let OKE = OKD - staged[257];
                    let OKF = OKE * OKE;
                    let OKG = (OKF * CH) / OKA;
                    let OKH = D - OKG;
                    let OKI = ((((((((OKB - Lanes([(EF * OKC), 0.0, 0.0, 0.0, 0.0])) / CH) * (GY / (GX * OKD))) * (LA * OKE)) * CH) + Lanes([(EF * OKF), 0.0, 0.0, 0.0, 0.0])) - (OKB * OKG)) / OKA) * AC;
                    OKJ = OKH;
                    OKK = OKI;
                } else {
                    OKJ = D;
                    OKK = AFD;
                }
                let OKL = NQR / LA;
                let OKM = NQS / LA;
                let OKN = if OKL < SY { 1.0 } else { 0.0 };
                let OKS;
                let OKT;
                if OKN != 0.0 {
                    let OKO = OKL.exp();
                    let OKP = D + OKO;
                    let OKQ = OKP.ln();
                    let OKR = (OKM * OKO) * (GY / OKP);
                    OKS = OKQ;
                    OKT = OKR;
                } else {
                    OKS = OKL;
                    OKT = OKM;
                }
                let OKU = LA * OKS;
                let OKV = OKT * LA;
                let OKW = OCA / LA;
                let OKX = OCD / LA;
                let OKY = if OKW < SY { 1.0 } else { 0.0 };
                let OLD;
                let OLE;
                if OKY != 0.0 {
                    let OKZ = OKW.exp();
                    let OLA = D + OKZ;
                    let OLB = OLA.ln();
                    let OLC = (OKX * OKZ) * (GY / OLA);
                    OLD = OLB;
                    OLE = OLC;
                } else {
                    OLD = OKW;
                    OLE = OKX;
                }
                let OLF = LA * OLD;
                let OLG = OLE * LA;
                let OLH = OLF - OCA;
                let OLI = OLG - OCD;
                let OLJ = OKU - NQR;
                let OLK = OKV - NQS;
                let OLL = GO * (LSC + OKU);
                let OLM = (LSD + OKV) * GO;
                let OLN = GO * (LSN + OLF);
                let OLO = (LSO + OLG) * GO;
                let OLP = OLL + OLN;
                let OLQ = D / OLP;
                let OLR = (((OLM + OLO) * OLQ) * AC) / OLP;
                let OLS = OIT * OLL;
                let OLT = OLS * OLQ;
                let OLU = (((OIU * OLL) + (OLM * OIT)) * OLQ) + (OLR * OLS);
                let OLV = OIT * OLN;
                let OLW = OLV * OLQ;
                let OLX = (((OIU * OLN) + (OLO * OIT)) * OLQ) + (OLR * OLV);
                let OLY = GO * (LSP + OLH);
                let OLZ = (LSQ + OLI) * GO;
                let OMA = GO * (LSR + OLJ);
                let OMB = (LSS + OLK) * GO;
                let OMC = GO * (LST + ((CSR * OKU) + (CSS * OLH)));
                let OMD = (LSU + ((OKV * CSR) + (OLI * CSS))) * GO;
                let OME = GO * (LSV + ((CSR * OLF) + (CSS * OLJ)));
                let OMF = (LSW + ((OLG * CSR) + (OLK * CSS))) * GO;
                let OMG = OLL * JO;
                let OMH = OMG * CTB;
                let OMI = OMH * OKJ;
                let OMJ = ((((OLM * JO) * CTB) + Lanes([(CTC * OMG), 0.0, 0.0, 0.0, 0.0])) * OKJ) + (OKK * OMH);
                let OMK = OLN * JR;
                let OML = OMK * CTB;
                let OMM = ((OLO * JR) * CTB) + Lanes([(CTC * OMK), 0.0, 0.0, 0.0, 0.0]);
                let OMN = OMI + OML;
                let OMO = OMJ + OMM;
                let OMP = OLY + (CTI * OMA);
                let OMQ = CU * OMP;
                let OMR = Lanes([(ES * OMP), 0.0, 0.0, 0.0, 0.0]) + ((OLZ + (OMB * CTI)) * CU);
                let OMS = D + OMQ;
                let OMT = OMR * OMS;
                let OMU = ((OMS * OMS) + NI).sqrt();
                let OMV = OMR * CTP;
                let OMW = D + (CTP * OMQ);
                let OMX = OMV * OMW;
                let OMY = ((OMW * OMW) + NI).sqrt();
                let OMZ = GO * (OMW + OMY);
                let ONA = (GO * (OMS + OMU)) / OMZ;
                let ONB = (((OMR + ((OMT + OMT) * (GY / (GX * OMU)))) * GO) - (((OMV + ((OMX + OMX) * (GY / (GX * OMY)))) * GO) * ONA)) / OMZ;
                let ONC = (D + (CTX * OLY)) + (CTY * OMA);
                let OND = CV * ONC;
                let ONE = (D + (OLT * CUD)) + (OLW * CUE);
                let ONF = ONE.ln();
                let ONG = (CUB * ONF).exp();
                let ONH = OND * ONG;
                let ONI = ((Lanes([(ET * ONC), 0.0, 0.0, 0.0, 0.0]) + (((OLZ * CTX) + (OMB * CTY)) * CV)) * ONG) + (((Lanes([(CUC * ONF), 0.0, 0.0, 0.0, 0.0]) + ((((OLU * CUD) + (OLX * CUE)) * (GY / ONE)) * CUB)) * ONG) * OND);
                let ONK;
                let ONL;
                if CUK != 0.0 {
                    ONK = D;
                    ONL = AFD;
                } else {
                    let OOP;
                    let OOQ;
                    if ONJ != 0.0 {
                        let OOG = OIT + CVS;
                        let OOH = (CVU * (OOG.ln())).exp();
                        let OOI = D - (CVW * OOH);
                        let OOJ = ((((OIU * (GY / OOG)) * CVU) * OOH) * CVW) * AC;
                        OOP = OOI;
                        OOQ = OOJ;
                    } else {
                        let OOK = OIT + CVS;
                        let OOL = (CVU * (OOK.ln())).exp();
                        let OOM = D + (CVW * OOL);
                        let OON = D / OOM;
                        let OOO = ((((((OIU * (GY / OOK)) * CVU) * OOL) * CVW) * OON) * AC) / OOM;
                        OOP = OON;
                        OOQ = OOO;
                    }
                    ONK = OOP;
                    ONL = OOQ;
                }
                let ONM = (OIT * ONK) + CUY;
                let ONN = LUJ * ONM;
                let ONO = (LUL * ONM) + (((OIU * ONK) + (ONL * OIT)) * LUJ);
                let ONP = (CZ * OMC) + CPW;
                let ONQ = ONP.ln();
                let ONR = (CY * ONQ).exp();
                let ONS = ((D + ONR) + ONH) + (DA * ONN);
                let ONT = (CZ * OME) + CPW;
                let ONU = ONT.ln();
                let ONV = (CY * ONU).exp();
                let ONW = ((D + ONV) + ONH) + (DB * ONN);
                let ONX = OMI / ONS;
                let ONY = OML / ONW;
                let ONZ = ONX + ONY;
                let OOA = (ONA * OMN) / ONZ;
                let OOB = (((ONB * OMN) + (OMO * ONA)) - ((((OMJ - (((((Lanes([(EW * ONQ), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * OMC), 0.0, 0.0, 0.0, 0.0]) + (OMD * CZ)) * (GY / ONP)) * CY)) * ONR) + ONI) + (Lanes([(EY * ONN), 0.0, 0.0, 0.0, 0.0]) + (ONO * DA))) * ONX)) / ONS) + ((OMM - (((((Lanes([(EW * ONU), 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(EX * OME), 0.0, 0.0, 0.0, 0.0]) + (OMF * CZ)) * (GY / ONT)) * CY)) * ONV) + ONI) + (Lanes([(EZ * ONN), 0.0, 0.0, 0.0, 0.0]) + (ONO * DB))) * ONY)) / ONW)) * OOA)) / ONZ;
                let OOC = XQ + OIT;
                let OOD = D / OOC;
                let OOE = OIU * OOD;
                let OOF = (OOE * AC) / OOC;
                let OOW;
                let OOX;
                if FPL != 0.0 {
                    let OOR = D + (FPX * OLW);
                    let OOS = D / OOR;
                    let OOT = (((OLX * FPX) * OOS) * AC) / OOR;
                    OOW = OOS;
                    OOX = OOT;
                } else {
                    let OOU = D - (FPX * OLW);
                    let OOV = (OLX * FPX) * AC;
                    OOW = OOU;
                    OOX = OOV;
                }
                let OOY = OIT * OOD;
                let OOZ = OOY * OOW;
                let OPA = FQJ * OIT;
                let OPB = FQI + (OPA * OIT);
                let OPC = (RA - MFE) / OPB;
                let OPD = D + OPC;
                let OPE = OPD.ln();
                let OPG = OPF * (OPE * OOZ);
                let OPH = ((((((DFR - MFG) - ((FQM + (((OIU * FQJ) * OIT) + (OIU * OPA))) * OPC)) / OPB) * (GY / OPD)) * OOZ) + ((((OOE + (OOF * OIT)) * OOW) + (OOX * OOY)) * OPE)) * OPF;
                let OPI = D + OPG;
                let OPJ = D + (OPG * OPI);
                let OPK = D / OPJ;
                let OPL = ((((OPH * OPI) + (OPH * OPG)) * OPK) * AC) / OPJ;
                let OPM = CZQ + OLL;
                let OPN = (CZQ * OLL) / OPM;
                let OPO = ((OLM * CZQ) - (OLM * OPN)) / OPM;
                let OPU;
                let OPV;
                if FRC != 0.0 {
                    let OPP = D - (CZU * OPN);
                    let OPQ = D / OPP;
                    let OPR = ((((OPO * CZU) * AC) * OPQ) * AC) / OPP;
                    OPU = OPQ;
                    OPV = OPR;
                } else {
                    let OPS = OPO * CZU;
                    let OPT = D + (CZU * OPN);
                    OPU = OPT;
                    OPV = OPS;
                }
                let OPW = CZQ + OLN;
                let OPX = (CZQ * OLN) / OPW;
                let OPY = ((OLO * CZQ) - (OLO * OPX)) / OPW;
                let OQE;
                let OQF;
                if FRN != 0.0 {
                    let OPZ = D - (DAU * OPX);
                    let OQA = D / OPZ;
                    let OQB = ((((OPY * DAU) * AC) * OQA) * AC) / OPZ;
                    OQE = OQA;
                    OQF = OQB;
                } else {
                    let OQC = OPY * DAU;
                    let OQD = D + (DAU * OPX);
                    OQE = OQD;
                    OQF = OQC;
                }
                let OQG = (EA * OIV) * GO;
                let OQH = OPU + OQE;
                let OQI = OOA * OPK;
                let OQJ = (OQG * OQH) / OQI;
                let OQK = OQJ * OQJ;
                let OQL = ((((((Lanes([(FY * OIV), 0.0, 0.0, 0.0, 0.0]) + (OIW * EA)) * GO) * OQH) + ((OPV + OQF) * OQG)) - (((OOB * OPK) + (OPL * OOA)) * OQJ)) / OQI) * OQJ;
                let OQM = OQL + OQL;
                let OQN = (D + OQK).sqrt();
                let OQO = (D + (SQ * OQK)) / OQN;
                let OQP = ((OQM * SQ) - ((OQM * (GY / (GX * OQN))) * OQO)) / OQN;
                let ORG;
                let ORH;
                let ORI;
                let ORJ;
                if L != 0.0 {
                    let OQQ = FSI * CP;
                    let OQR = EN * FSI;
                    let OQS = OLM * OLL;
                    let OQT = (OLL * OLL) + FSM;
                    let OQV = (OQU * (OQT.ln())).exp();
                    let OQW = OQQ * OQV;
                    let OQX = OLO * OLN;
                    let OQY = (OLN * OLN) + FSM;
                    let ORA = (OQZ * (OQY.ln())).exp();
                    let ORB = OQQ * ORA;
                    let ORC = (D + (JJR * OQW)) / JGW;
                    let ORD = (((JJT * OQW) + ((Lanes([(OQR * OQV), 0.0, 0.0, 0.0, 0.0]) + (((((OQS + OQS) * (GY / OQT)) * OQU) * OQV) * OQQ)) * JJR)) - (JHC * ORC)) / JGW;
                    let ORE = (D + (JJU * ORB)) / JGX;
                    let ORF = (((JJW * ORB) + ((Lanes([(OQR * ORA), 0.0, 0.0, 0.0, 0.0]) + (((((OQX + OQX) * (GY / OQY)) * OQZ) * ORA) * OQQ)) * JJU)) - (JHD * ORE)) / JGX;
                    ORG = ORC;
                    ORH = ORE;
                    ORI = ORD;
                    ORJ = ORF;
                } else {
                    ORG = D;
                    ORH = D;
                    ORI = AFD;
                    ORJ = AFD;
                }
                let ORL;
                let ORM;
                if LPZ != 0.0 {
                    let ORK = if OCB > CPW { 1.0 } else { 0.0 };
                    let ORQ;
                    let ORR;
                    if ORK != 0.0 {
                        let ORP = if (OIJ.abs()) < NI { 1.0 } else { 0.0 };
                        let OTE;
                        let OTF;
                        if ORP != 0.0 {
                            let ORY = LA + OBZ;
                            let ORZ = ORY * OIK;
                            let OSA = ((LA + NQL) + (GO * OIK)) / ORZ;
                            let OSB = ((NQO + (OIO * GO)) - (((OCC * OIK) + (OIO * ORY)) * OSA)) / ORZ;
                            let OSC = OSA * OIJ;
                            let OSD = (OSB * OIJ) + (OIN * OSA);
                            let OSE = OSC * OSC;
                            let OSF = OSD * OSC;
                            let OSG = OSF + OSF;
                            let OSH = ((D - OSC) + OSE) - (OSC * OSE);
                            let OSI = LA * OBR;
                            let OSJ = D / OIK;
                            let OSK = OSA - OSJ;
                            let OSL = OSI * OSK;
                            let OSM = (OCA - (OSL * OSH)) / ORY;
                            let OSN = ((OIL * OCB) - OBO) / OIK;
                            let OSO = (OSN - OSM) / OCB;
                            let OSP = OCE * OSO;
                            let OSQ = (((((((OIP * OCB) + (OCE * OIL)) - OBP) - (OIO * OSN)) / OIK) - (((OCD - (((((OBS * LA) * OSK) + ((OSB - (((OIO * OSJ) * AC) / OIK)) * OSI)) * OSH) + ((((OSD * AC) + OSG) - ((OSD * OSE) + (OSG * OSC))) * OSL))) - (OCC * OSM)) / ORY)) - OSP) / OCB;
                            let OSR = OSO + D;
                            let OSS = (OSO * OCB) / OSR;
                            let OST = (((OSQ * OCB) + OSP) - (OSQ * OSS)) / OSR;
                            OTE = OSS;
                            OTF = OST;
                        } else {
                            let OSU = OIK * OIJ;
                            let OSV = (OIL * OIM) / OSU;
                            let OSW = OBO / OIK;
                            let OSX = OHW / OIJ;
                            let OSY = (OSW + OSX) / OCB;
                            let OSZ = OSV - OSY;
                            let OTA = ((((OIP * OIM) + (OIQ * OIL)) - (((OIO * OIJ) + (OIN * OIK)) * OSV)) / OSU) - (((((OBP - (OIO * OSW)) / OIK) + ((OHX - (OIN * OSX)) / OIJ)) - (OCE * OSY)) / OCB);
                            let OTB = OSZ + D;
                            let OTC = (OSZ * OCB) / OTB;
                            let OTD = (((OTA * OCB) + (OCE * OSZ)) - (OTA * OTC)) / OTB;
                            OTE = OTC;
                            OTF = OTD;
                        }
                        ORQ = OTE;
                        ORR = OTF;
                    } else {
                        ORQ = LWW;
                        ORR = LXC;
                    }
                    let ORS = ORQ - LZJ;
                    let ORT = ORR - LZM;
                    let ORU = DEL * ORS;
                    let ORV = ((ORT * DEL) * ORS) + (ORT * ORU);
                    let ORW = D + (ORU * ORS);
                    let ORX = if (ORS.abs()) > FTT { 1.0 } else { 0.0 };
                    let OUH;
                    let OUI;
                    if ORX != 0.0 {
                        let OTG = OCB - LKB;
                        let OTH = OCE - LKE;
                        let OTI = OTG - (ORQ * OIV);
                        let OTJ = OTH - ((ORR * OIV) + (OIW * ORQ));
                        let OTK = OTG - (LZJ * OIV);
                        let OTL = OTH - ((LZM * OIV) + (OIW * LZJ));
                        let OTM = OTJ * OTI;
                        let OTN = ((OTI * OTI) + ORW).sqrt();
                        let OTO = ((OTM + OTM) + ORV) * (GY / (GX * OTN));
                        let OTP = OTL * OTK;
                        let OTQ = ((OTK * OTK) + ORW).sqrt();
                        let OTR = ((OTP + OTP) + ORV) * (GY / (GX * OTQ));
                        let OTS = YY / ORS;
                        let OTT = OTI + OTN;
                        let OTU = (OTK + OTQ) / OTT;
                        let OTV = OTU.ln();
                        let OTW = ((OTQ * OTI) - (OTN * OTK)) + (ORW * OTV);
                        let OTX = OTS * OTW;
                        let OTY = ((((ORT * OTS) * AC) / ORS) * OTW) + (((((OTR * OTI) + (OTJ * OTQ)) - ((OTO * OTK) + (OTL * OTN))) + ((ORV * OTV) + (((((OTL + OTR) - ((OTJ + OTO) * OTU)) / OTT) * (GY / OTU)) * ORW))) * OTS);
                        OUH = OTX;
                        OUI = OTY;
                    } else {
                        let OTZ = OIV * ORS;
                        let OUA = (OIW * ORS) + (ORT * OIV);
                        let OUC = OUB * OIV;
                        let OUD = OUC * OTZ;
                        let OUE = ORW.sqrt();
                        let OUF = (OUD * OTZ) / OUE;
                        let OUG = ((((((OIW * OUB) * OTZ) + (OUA * OUC)) * OTZ) + (OUA * OUD)) - ((ORV * (GY / (GX * OUE))) * OUF)) / OUE;
                        OUH = OUF;
                        OUI = OUG;
                    }
                    ORL = OUH;
                    ORM = OUI;
                } else {
                    ORL = B;
                    ORM = AFD;
                }
                let ORN = (((OIT * OIV) + ORL) + LKB) - OCB;
                let ORO = ((((OIU * OIV) + (OIW * OIT)) + ORM) + LKE) - OCE;
                let OVT;
                let OVU;
                let OVV;
                let OVW;
                if LPZ != 0.0 {
                    let OUJ = if ORN > FWG { 1.0 } else { 0.0 };
                    let OWX;
                    let OWY;
                    let OWZ;
                    let OXA;
                    if OUJ != 0.0 {
                        let OWH = LJP / LKB;
                        let OWI = OWH - LQL;
                        let OWJ = LQN / OWI;
                        let OWK = OBO / OCB;
                        let OWL = OWK - OIL;
                        let OWM = OIK / OWL;
                        let OWN = (OWJ - OWM) / ORN;
                        let OWO = ((((LQR - ((((LJQ - (LKE * OWH)) / LKB) - LQP) * OWJ)) / OWI) - ((OIO - ((((OBP - (OCE * OWK)) / OCB) - OIP) * OWM)) / OWL)) - (ORO * OWN)) / ORN;
                        let OWP = LPX / LKB;
                        let OWQ = OWP - LQL;
                        let OWR = LQO / OWQ;
                        let OWS = OHW / OCB;
                        let OWT = OWS - OIL;
                        let OWU = OIJ / OWT;
                        let OWV = (OWR - OWU) / ORN;
                        let OWW = ((((LQS - ((((LPY - (LKE * OWP)) / LKB) - LQP) * OWR)) / OWQ) - ((OIN - ((((OHX - (OCE * OWS)) / OCB) - OIP) * OWU)) / OWT)) - (ORO * OWV)) / ORN;
                        OWX = OWN;
                        OWY = OWV;
                        OWZ = OWO;
                        OXA = OWW;
                    } else {
                        OWX = B;
                        OWY = B;
                        OWZ = AFD;
                        OXA = AFD;
                    }
                    OVT = OWX;
                    OVU = OWY;
                    OVV = OWZ;
                    OVW = OXA;
                } else {
                    let OUL = OUK * LWY;
                    let OUM = JJX / LWZ;
                    let OUN = OUM + LXI;
                    let OUO = OUL * OUN;
                    let OUP = ((LXE * OUK) * OUN) + ((((JJY - (LXF * OUM)) / LWZ) + LXJ) * OUL);
                    let OUR = OUQ * LXA;
                    let OUS = JJZ / LXB;
                    let OUT = OUS + LXI;
                    let OUU = OUR * OUT;
                    let OUV = ((LXG * OUQ) * OUT) + ((((JKA - (LXH * OUS)) / LXB) + LXJ) * OUR);
                    let OUW = OUU - OUO;
                    let OUX = OUW * LXI;
                    let OUY = ((OUV - OUP) * LXI) + (LXJ * OUW);
                    let OUZ = OUO * JJX;
                    let OVA = (OUP * JJX) + (JJY * OUO);
                    let OVB = OUU * JJZ;
                    let OVC = (OUV * JJZ) + (JKA * OUU);
                    let OVD = OUZ + OVB;
                    let OVE = OVA + OVC;
                    let OVF = (((LXE * JJX) + (JJY * LWY)) + ((LXG * JJZ) + (JKA * LXA))) * LA;
                    let OVG = ZD + (LA * ((LWY * JJX) + (LXA * JJZ)));
                    let OVH = OVD / LWZ;
                    let OVI = ((OVB + OUX) - OVH) / OVG;
                    let OVJ = OVD / LXB;
                    let OVK = ((OUZ - OUX) - OVJ) / OVG;
                    let OVL = -LWZ;
                    let OVM = (OVI * LWZ) + LXI;
                    let OVN = OVL * OVM;
                    let OVO = ((LXF * AC) * OVM) + ((((((((OVC + OUY) - ((OVE - (LXF * OVH)) / LWZ)) - (OVF * OVI)) / OVG) * LWZ) + (LXF * OVI)) + LXJ) * OVL);
                    let OVP = -LXB;
                    let OVQ = (OVK * LXB) + LXI;
                    let OVR = OVP * OVQ;
                    let OVS = ((LXH * AC) * OVQ) + ((((((((OVA - OUY) - ((OVE - (LXH * OVJ)) / LXB)) - (OVF * OVK)) / OVG) * LXB) + (LXH * OVK)) + LXJ) * OVP);
                    OVT = OVN;
                    OVU = OVR;
                    OVV = OVO;
                    OVW = OVS;
                }
                let OVX = OVT * OQO;
                let OVY = OVU * OQO;
                let OVZ = GO * (NQR - KYS);
                let OWA = (NQS - KYT) * GO;
                let OWB = GO * (OCA - LKC);
                let OWC = (OCD - LKF) * GO;
                let OWD = OVZ * OVX;
                let OWE = (OWA * OVX) + (((OVV * OQO) + (OQP * OVT)) * OVZ);
                let OWF = OWB * OVY;
                let OWG = (OWC * OVY) + (((OVW * OQO) + (OQP * OVU)) * OWB);
                ISD = JHE;
                ISE = JLR;
                ISF = OIT;
                ISG = KYS;
                ISH = NQR;
                ISI = LKC;
                ISJ = OCA;
                ISK = OLT;
                ISL = ORG;
                ISM = OLW;
                ISN = ORH;
                ISO = OVZ;
                ISP = OWD;
                ISQ = OKJ;
                ISR = OWB;
                ISS = OWF;
                IST = JGV;
                ISU = LQT;
                ISV = OIR;
                ISW = JKC;
                ISX = JJZ;
                ISY = JJR;
                ISZ = JJX;
                ITA = JIH;
                ITB = JJU;
                ITC = JIL;
                ITD = JHV;
                ITE = IRZ;
                ITF = OQK;
                ITG = JLO;
                ITH = JKG;
                ITI = JJP;
                ITJ = JJB;
                ITK = JHW;
                ITL = JIU;
                ITM = JHF;
                ITN = JLS;
                ITO = OIU;
                ITP = KYT;
                ITQ = NQS;
                ITR = LKF;
                ITS = OCD;
                ITT = OLU;
                ITU = ORI;
                ITV = OLX;
                ITW = ORJ;
                ITX = OWA;
                ITY = OWE;
                ITZ = OKK;
                IUA = OWC;
                IUB = OWG;
                IUC = JHB;
                IUD = LQU;
                IUE = OIS;
                IUF = JKD;
                IUG = JKA;
                IUH = JJT;
                IUI = JJY;
                IUJ = JII;
                IUK = JJW;
                IUL = JIM;
                IUM = JHU;
                IUN = ISA;
                IUO = OQM;
            } else {
                ISD = AFQ;
                ISE = AKT;
                ISF = FJW;
                ISG = BYN;
                ISH = ERU;
                ISI = CJY;
                ISJ = FDD;
                ISK = FMY;
                ISL = FTA;
                ISM = FNB;
                ISN = FTB;
                ISO = FXY;
                ISP = FYC;
                ISQ = FLN;
                ISR = FYA;
                ISS = FYE;
                IST = AFH;
                ISU = CQR;
                ISV = FJU;
                ISW = AJC;
                ISX = AIZ;
                ISY = AIR;
                ISZ = AIX;
                ITA = AGV;
                ITB = AIU;
                ITC = AGZ;
                ITD = AGI;
                ITE = RW;
                ITF = FSB;
                ITG = AKQ;
                ITH = AJH;
                ITI = AIP;
                ITJ = AHY;
                ITK = AGJ;
                ITL = AHQ;
                ITM = AFR;
                ITN = AKU;
                ITO = FJX;
                ITP = BYO;
                ITQ = ERV;
                ITR = CKB;
                ITS = FDG;
                ITT = FMZ;
                ITU = FTC;
                ITV = FNC;
                ITW = FTD;
                ITX = FXZ;
                ITY = FYD;
                ITZ = FLO;
                IUA = FYB;
                IUB = FYF;
                IUC = AFN;
                IUD = CQS;
                IUE = FJV;
                IUF = AJD;
                IUG = AJA;
                IUH = AIT;
                IUI = AIY;
                IUJ = AGW;
                IUK = AIW;
                IUL = AHA;
                IUM = AGH;
                IUN = RY;
                IUO = FSD;
            }
            let IUQ = D + (YY * ISF);
            let IUR = (IUP * (ISD - ISE)) / IUQ;
            let IUS = (((ITM - ITN) * IUP) - ((ITO * YY) * IUR)) / IUQ;
            let IUT = (GO * (ISG + ISH)) + IUR;
            let IUU = ((ITP + ITQ) * GO) + IUS;
            let IUV = (GO * (ISI + ISJ)) - IUR;
            let IUW = ((ITR + ITS) * GO) - IUS;
            let OXH;
            let OXI;
            let OXJ;
            let OXK;
            if L != 0.0 {
                let OXB = ISK / ISL;
                let OXC = (IUT + OXB) - ISK;
                let OXD = (IUU + ((ITT - (ITU * OXB)) / ISL)) - ITT;
                let OXE = ISM / ISN;
                let OXF = (IUV + OXE) - ISM;
                let OXG = (IUW + ((ITV - (ITW * OXE)) / ISN)) - ITV;
                OXH = OXC;
                OXI = OXF;
                OXJ = OXD;
                OXK = OXG;
            } else {
                OXH = IUT;
                OXI = IUV;
                OXJ = IUU;
                OXK = IUW;
            }
            let OXL = ISO * UC;
            let OXM = D - (CTP * ISP);
            let OXN = D + (ISP * OXM);
            let OXO = GO * OXH;
            let OXP = (OXH * ISQ) + ((ISO * ISP) * WD);
            let OXQ = ISR * UC;
            let OXR = D - (CTP * ISS);
            let OXS = D + (ISS * OXR);
            let OXT = OXI + ((ISR * ISS) * WD);
            let OXU = IST * EB;
            let OXV = (IUC * EB) + Lanes([(FZ * IST), 0.0, 0.0, 0.0, 0.0]);
            let OXW = OXU * OXP;
            let OXX = (OXV * OXP) + ((((OXJ * ISQ) + (ITZ * OXH)) + (((ITX * ISP) + (ITY * ISO)) * WD)) * OXU);
            let OXY = OXU * OXT;
            let OXZ = (OXV * OXT) + ((OXK + (((IUA * ISS) + (IUB * ISR)) * WD)) * OXU);
            let OYA = -OXU;
            let OYB = ((OXO * ISQ) + (OXL * OXN)) + ((GO * OXI) + (OXQ * OXS));
            let OYC = OYA * OYB;
            let OYD = ((OXV * AC) * OYB) + ((((((OXJ * GO) * ISQ) + (ITZ * OXO)) + (((ITX * UC) * OXN) + (((ITY * OXM) + (((ITY * CTP) * AC) * ISP)) * OXL))) + ((OXK * GO) + (((IUA * UC) * OXS) + (((IUB * OXR) + (((IUB * CTP) * AC) * ISS)) * OXQ)))) * OYA);
            let PCU;
            let PCV;
            let PCW;
            let PCX;
            let PCY;
            let PCZ;
            let PDA;
            let PDB;
            if OYE != 0.0 {
                let OYF = CQ + 1.3862943611198e0f64;
                let OYG = ISU + OYF;
                let OYH = IUD + AFZ;
                let OYI = ISV + OYF;
                let OYJ = IUE + AFZ;
                let OYK = OYG - CQ;
                let OYL = (OYH - AFZ) * OYK;
                let OYN = ((OYK * OYK) + OYM).sqrt();
                let OYO = GO * ((OYG + CQ) - OYN);
                let OYP = ((OYH + AFZ) - ((OYL + OYL) * (GY / (GX * OYN)))) * GO;
                let OYQ = CQ + RA;
                let OYR = Lanes([EO, 0.0, 0.0]) + RC;
                let OYS = Lanes([OYR[0], OYR[1], OYR[2], 0.0, 0.0]);
                let OYT = OYI - OYQ;
                let OYU = (OYJ - OYS) * OYT;
                let OYV = ((OYT * OYT) + OYM).sqrt();
                let OYW = GO * ((OYI + OYQ) - OYV);
                let OYX = ((OYJ + OYS) - ((OYU + OYU) * (GY / (GX * OYV)))) * GO;
                let OYY = GO + ISX;
                let OYZ = (ISW * OYY).sqrt();
                let OZB = OZA * OYZ;
                let OZC = (((IUF * OYY) + (IUG * ISW)) * (GY / (GX * OYZ))) * OZA;
                let OZD = ISW * ISY;
                let OZE = OZD * ISX;
                let OZF = GO + ISZ;
                let OZG = (OZE * OZF).sqrt();
                let OZH = OZA * OZG;
                let OZI = (((((((IUF * ISY) + (IUH * ISW)) * ISX) + (IUG * OZD)) * OZF) + (IUI * OZE)) * (GY / (GX * OZG))) * OZA;
                let OZJ = OZB * OZB;
                let OZK = OZC * OZB;
                let OZL = OZJ * EC;
                let OZM = ((OZK + OZK) * EC) + Lanes([(GA * OZJ), 0.0, 0.0, 0.0, 0.0]);
                let OZN = OZH * OZH;
                let OZO = OZI * OZH;
                let OZP = OZN * EC;
                let OZQ = ((OZO + OZO) * EC) + Lanes([(GA * OZN), 0.0, 0.0, 0.0, 0.0]);
                let OZR = ED - OYO;
                let OZS = Lanes([GB, 0.0, 0.0, 0.0, 0.0]) - OYP;
                let OZT = Lanes([GB, 0.0, 0.0]) + RC;
                let OZU = (ED + RA) - OYW;
                let OZV = Lanes([OZT[0], OZT[1], OZT[2], 0.0, 0.0]) - OYX;
                let OZW = LA * OZL;
                let OZX = OZM * LA;
                let OZY = OZR / OZL;
                let OZZ = (D + OZY).sqrt();
                let PAA = OZZ - D;
                let PAB = OYO + (OZW * PAA);
                let PAC = OYP + ((OZX * PAA) + ((((OZS - (OZM * OZY)) / OZL) * (GY / (GX * OZZ))) * OZW));
                let PAD = OZU / OZL;
                let PAE = (D + PAD).sqrt();
                let PAF = PAE - D;
                let PAG = OYW + (OZW * PAF);
                let PAH = OYX + ((OZX * PAF) + ((((OZV - (OZM * PAD)) / OZL) * (GY / (GX * PAE))) * OZW));
                let PAI = LA * OZP;
                let PAJ = OZQ * LA;
                let PAK = OZR / OZP;
                let PAL = (D + PAK).sqrt();
                let PAM = PAL - D;
                let PAN = OYO + (PAI * PAM);
                let PAO = OYP + ((PAJ * PAM) + ((((OZS - (OZQ * PAK)) / OZP) * (GY / (GX * PAL))) * PAI));
                let PAP = OZU / OZP;
                let PAQ = (D + PAP).sqrt();
                let PAR = PAQ - D;
                let PAS = OYW + (PAI * PAR);
                let PAT = OYX + ((PAJ * PAR) + ((((OZV - (OZQ * PAP)) / OZP) * (GY / (GX * PAQ))) * PAI));
                let PAU = -(EE * IST);
                let PAV = (Lanes([(GC * IST), 0.0, 0.0, 0.0, 0.0]) + (IUC * EE)) * AC;
                let PAW = PAU * OZB;
                let PAX = PAW * ISY;
                let PAY = PAX * ITA;
                let PAZ = IUJ * PAX;
                let PBA = (((((PAV * OZB) + (OZC * PAU)) * ISY) + (IUH * PAW)) * ITA) + Lanes([PAZ[0], PAZ[1], PAZ[2], PAZ[3], 0.0]);
                let PBB = PAU * OZH;
                let PBC = PBB * ITB;
                let PBD = PBC * ITC;
                let PBE = IUL * PBC;
                let PBF = (((((PAV * OZH) + (OZI * PAU)) * ITB) + (IUK * PBB)) * ITC) + Lanes([PBE[0], PBE[1], PBE[2], PBE[3], 0.0]);
                let PBG = PAB - OYG;
                let PBH = PAC - OYH;
                let PBI = PBH * PBG;
                let PBJ = ((PBG * PBG) + D).sqrt();
                let PBK = GO * (PBG + PBJ);
                let PBL = (PBH + ((PBI + PBI) * (GY / (GX * PBJ)))) * GO;
                let PBM = PAY * PBK;
                let PBN = PAB - OYO;
                let PBO = (PBM * PBK) / PBN;
                let PBP = (((((PBA * PBK) + (PBL * PAY)) * PBK) + (PBL * PBM)) - ((PAC - OYP) * PBO)) / PBN;
                let PBQ = PAG - OYI;
                let PBR = PAH - OYJ;
                let PBS = PBR * PBQ;
                let PBT = ((PBQ * PBQ) + D).sqrt();
                let PBU = GO * (PBQ + PBT);
                let PBV = (PBR + ((PBS + PBS) * (GY / (GX * PBT)))) * GO;
                let PBW = PAY * PBU;
                let PBX = PAG - OYW;
                let PBY = (PBW * PBU) / PBX;
                let PBZ = (((((PBA * PBU) + (PBV * PAY)) * PBU) + (PBV * PBW)) - ((PAH - OYX) * PBY)) / PBX;
                let PCA = PAN - OYG;
                let PCB = PAO - OYH;
                let PCC = PCB * PCA;
                let PCD = ((PCA * PCA) + D).sqrt();
                let PCE = GO * (PCA + PCD);
                let PCF = (PCB + ((PCC + PCC) * (GY / (GX * PCD)))) * GO;
                let PCG = PBD * PCE;
                let PCH = PAN - OYO;
                let PCI = (PCG * PCE) / PCH;
                let PCJ = (((((PBF * PCE) + (PCF * PBD)) * PCE) + (PCF * PCG)) - ((PAO - OYP) * PCI)) / PCH;
                let PCK = PAS - OYI;
                let PCL = PAT - OYJ;
                let PCM = PCL * PCK;
                let PCN = ((PCK * PCK) + D).sqrt();
                let PCO = GO * (PCK + PCN);
                let PCP = (PCL + ((PCM + PCM) * (GY / (GX * PCN)))) * GO;
                let PCQ = PBD * PCO;
                let PCR = PAS - OYW;
                let PCS = (PCQ * PCO) / PCR;
                let PCT = (((((PBF * PCO) + (PCP * PBD)) * PCO) + (PCP * PCQ)) - ((PAT - OYX) * PCS)) / PCR;
                PCU = PBO;
                PCV = PBY;
                PCW = PCI;
                PCX = PCS;
                PCY = PBP;
                PCZ = PBZ;
                PDA = PCJ;
                PDB = PCT;
            } else {
                PCU = B;
                PCV = B;
                PCW = B;
                PCX = B;
                PCY = AFD;
                PCZ = AFD;
                PDA = AFD;
                PDB = AFD;
            }
            let PDD = PDC * QC;
            let PDE = QF * PDC;
            let PDG = PDF * QJ;
            let PDH = QL * PDF;
            let PDJ = PDI * ITD;
            let PDL = D - (PDK * ITE);
            let PDM = ((IUN * PDK) * AC) * PDJ;
            let PDN = D - (PDJ * PDL);
            let PDO = (((IUM * PDI) * PDL) + Lanes([PDM[0], PDM[1], PDM[2], PDM[3], 0.0])) * AC;
            let PDP = PDO * PDN;
            let PDQ = ((PDN * PDN) + CTP).sqrt();
            let PDR = GO * (PDN + PDQ);
            let PDS = (PDO + ((PDP + PDP) * (GY / (GX * PDQ)))) * GO;
            let PDU = PDT * GVT;
            let PDV = PDU * PDR;
            let PDW = (GVU * PDT) * PDR;
            let PDX = Lanes([PDW[0], PDW[1], 0.0, 0.0, PDW[2]]) + (PDS * PDU);
            let PDZ = PDY * GVW;
            let PEA = PDZ * PDR;
            let PEB = (GVX * PDY) * PDR;
            let PEC = Lanes([PEB[0], PEB[1], PEB[2], 0.0, PEB[3]]) + (PDS * PDZ);
            let PEE = PED * QY;
            let PEF = QZ * PED;
            let PEH = PEG * QT;
            let PEI = QX * PEG;
            let PEK = PEJ * QD;
            let PEL = QG * PEJ;
            let PEN = PEM * QM;
            let PEO = QO * PEM;
            let PES;
            let PET;
            if S != 0.0 {
                let PEQ = PEP * DV;
                let PER = FT * PEP;
                PES = PEQ;
                PET = PER;
            } else {
                PES = B;
                PET = CG;
            }
            let PEV = PEU * ((FZY + ILG) + IPK);
            let PEW = ((FZZ + ILH) + IPL) * PEU;
            let PEX = PEU * HCE;
            let PEY = HCG * PEU;
            let PEZ = PEU * HCF;
            let PFA = HCH * PEU;
            let PFB = PEU * IHF;
            let PFC = IHG * PEU;
            let PFD = PEU * IEJ;
            let PFE = IEK * PEU;
            let PFG = PFF * IRD;
            let PFH = IRF * PFF;
            let PFI = PFF * IRE;
            let PFJ = IRG * PFF;
            let PFK = if QU < B { 1.0 } else { 0.0 };
            let PFP;
            let PFQ;
            let PFR;
            let PFS;
            if PFK != 0.0 {
                let PFL = IY * PEV;
                let PFM = PEW * IY;
                PFP = PFL;
                PFQ = B;
                PFR = PFM;
                PFS = AFD;
            } else {
                let PFN = IY * PEV;
                let PFO = PEW * IY;
                PFP = B;
                PFQ = PFN;
                PFR = AFD;
                PFS = PFO;
            }
            let PFT = IY * (PFB - PFD);
            let PFU = (PFC - PFE) * IY;
            let PFV = IY * PEX;
            let PFW = PEY * IY;
            let PFX = IY * PEZ;
            let PFY = PFA * IY;
            let PFZ = parameters[31] * C;
            let PGA = PFZ * (PN - PI);
            let PGB = (Lanes([0.0, PP]) - Lanes([PL, 0.0])) * PFZ;
            let PGH;
            let PGI;
            if O != 0.0 {
                let PGE = PGD * (node_potentials[1] - PH);
                let PGF = (Lanes([PGC, 0.0]) - Lanes([0.0, PK])) * PGD;
                PGH = PGE;
                PGI = PGF;
            } else {
                PGH = B;
                PGI = PGG;
            }
            let PGO;
            let PGP;
            if P != 0.0 {
                let PGL = PGK * (node_potentials[2] - PI);
                let PGM = (Lanes([PGJ, 0.0]) - Lanes([0.0, PL])) * PGK;
                PGO = PGL;
                PGP = PGM;
            } else {
                PGO = B;
                PGP = PGN;
            }
            let PGV;
            let PGW;
            if Q != 0.0 {
                let PGS = PGR * (node_potentials[0] - PN);
                let PGT = (Lanes([PGQ, 0.0]) - Lanes([0.0, PP])) * PGR;
                PGV = PGS;
                PGW = PGT;
            } else {
                PGV = B;
                PGW = PGU;
            }
            let PHC;
            let PHD;
            if R != 0.0 {
                let PGZ = PGY * (node_potentials[3] - PR);
                let PHA = (Lanes([PGX, 0.0]) - Lanes([0.0, PT])) * PGY;
                PHC = PGZ;
                PHD = PHA;
            } else {
                PHC = B;
                PHD = PHB;
            }
            let PHF = PHE * OXW;
            let PHG = OXX * PHE;
            let PHH = PHE * OXY;
            let PHI = OXZ * PHE;
            let PHJ = PHE * OYC;
            let PHK = OYD * PHE;
            let PHL = -((PHF + PHH) + PHJ);
            let PHM = ((PHG + PHI) + PHK) * AC;
            let PHN = PHE * PCU;
            let PHO = PCY * PHE;
            let PHP = PHE * PCV;
            let PHQ = PCZ * PHE;
            let PHR = PHE * PCW;
            let PHS = PDA * PHE;
            let PHT = PHE * PCX;
            let PHU = PDB * PHE;
            let PHV = PHE * PDD;
            let PHW = PDE * PHE;
            let PHX = PHE * PDG;
            let PHY = PDH * PHE;
            let PHZ = PHE * PDV;
            let PIA = PDX * PHE;
            let PIB = PHE * PEA;
            let PIC = PEC * PHE;
            let PID = PHE * PEE;
            let PIE = PEF * PHE;
            let PIF = PHE * PEK;
            let PIG = PEL * PHE;
            let PIH = PHE * PEN;
            let PII = PEO * PHE;
            let PIJ = PHE * PEH;
            let PIK = PEI * PHE;
            let PIL = PFF * PES;
            let PIM = PET * PFF;
            let PIP;
            let PIQ;
            let PIR;
            let PIS;
            let PIT;
            let PIU;
            let PIV;
            let PIW;
            let PIX;
            let PIY;
            let PIZ;
            let PJA;
            if PFK != 0.0 {
                let PIN = -PIJ;
                let PIO = PIK * AC;
                PIP = PHT;
                PIQ = PHN;
                PIR = PHR;
                PIS = PHP;
                PIT = PHL;
                PIU = PIN;
                PIV = PHU;
                PIW = PHO;
                PIX = PHS;
                PIY = PHQ;
                PIZ = PHM;
                PJA = PIO;
            } else {
                PIP = PHR;
                PIQ = PHP;
                PIR = PHT;
                PIS = PHN;
                PIT = PHJ;
                PIU = PIJ;
                PIV = PHS;
                PIW = PHQ;
                PIX = PHU;
                PIY = PHO;
                PIZ = PHK;
                PJA = PIK;
            }
            let PJC = PIG * PJB;
            let PJD = IY * ((ddt(80177, PHH) + ddt(80179, PIF)) + ddt(80182, PIP));
            let PJE = (((PHI * PJB) + Lanes([0.0, PJC[0], 0.0, PJC[1], 0.0])) + (PIV * PJB)) * IY;
            let PJF = IY * ((PHH + PIF) + PIP);
            let PJG = ((PHI + Lanes([0.0, PIG[0], 0.0, PIG[1], 0.0])) + PIV) * IY;
            let PJH = PHY * PJB;
            let PJI = Lanes([0.0, PHY[0], PHY[1], 0.0, PHY[2]]);
            let PJJ = IY * ((ddt(80187, PHX) + ddt(80189, PIB)) + ddt(80192, PIQ));
            let PJK = ((Lanes([0.0, PJH[0], PJH[1], 0.0, PJH[2]]) + (PIC * PJB)) + (PIW * PJB)) * IY;
            let PJL = IY * ((PHX + PIB) + PIQ);
            let PJM = ((PJI + PIC) + PIW) * IY;
            let PJN = PII * PJB;
            let PJO = IY * (ddt(80197, PIH) + ddt(80199, PIR));
            let PJP = (Lanes([0.0, PJN[0], PJN[1], PJN[2], 0.0]) + (PIX * PJB)) * IY;
            let PJQ = IY * (PIH + PIR);
            let PJR = (Lanes([0.0, PII[0], PII[1], PII[2], 0.0]) + PIX) * IY;
            let PJS = IY * ddt(80204, PID);
            let PJT = (PIE * PJB) * IY;
            let PJU = IY * PID;
            let PJV = PIE * IY;
            let PJW = PHW * PJB;
            let PJX = PHG + Lanes([0.0, PHW[0], 0.0, 0.0, PHW[1]]);
            let PJY = IY * (((ddt(80208, PHF) + ddt(80210, PHV)) + ddt(80213, PHZ)) + ddt(80216, PIS));
            let PJZ = ((((PHG * PJB) + Lanes([0.0, PJW[0], 0.0, 0.0, PJW[1]])) + (PIA * PJB)) + (PIY * PJB)) * IY;
            let PKA = IY * (((PHF + PHV) + PHZ) + PIS);
            let PKB = ((PJX + PIA) + PIY) * IY;
            let PKC = PJA * PJB;
            let PKD = IY * (ddt(80221, PIT) + ddt(80223, PIU));
            let PKE = ((PIZ * PJB) + Lanes([0.0, PKC[0], PKC[1], 0.0, 0.0])) * IY;
            let PKF = IY * (PIT + PIU);
            let PKG = (PIZ + Lanes([0.0, PJA[0], PJA[1], 0.0, 0.0])) * IY;
            let PKH = ddt(80227, PIL);
            let PKI = PIM * PJB;
            let PKK = FJW + (PKJ * (DAE + FTG));
            let PKL = FJX + ((DAH + FTI) * PKJ);
            let PKM = FJW / PKK;
            let PKN = (FJX - (PKL * PKM)) / PKK;
            let PKO = PKN * PKM;
            let PKP = ((PKM * PKM) + DCX).sqrt();
            let PKQ = GO * (PKM + PKP);
            let PKR = (PKN + ((PKO + PKO) * (GY / (GX * PKP)))) * GO;
            let PKT = PKS * FXY;
            let PKU = PKT * FXR;
            let PKV = ((FXZ * PKS) * FXR) + (FXT * PKT);
            let PKW = PKU * PKU;
            let PKX = PKV * PKU;
            let PKY = PKX + PKX;
            let PKZ = FSG - D;
            let PLA = XS * PKZ;
            let PLB = D - (PLA * PKW);
            let PLC = if PLB >= DCX { PLB } else { DCX };
            let PLD = ((((FSH * XS) * PKW) + (PKY * PLA)) * AC) * (if PLB >= DCX { 1.0 } else { 0.0 });
            let PLE = PLC * PLC;
            let PLF = PLD * PLC;
            let PLG = D / PLE;
            let PLH = (((PLF + PLF) * PLG) * AC) / PLE;
            let PLI = FYG * AFH;
            let PLJ = PLI * DD;
            let PLK = PLJ * PKK;
            let PLL = (PLK * FYN) / FYP;
            let PLM = PLL / FZQ;
            let PLN = (((((((((((FYH * AFH) + (AFN * FYG)) * DD) + Lanes([(FB * PLI), 0.0, 0.0, 0.0, 0.0])) * PKK) + (PKL * PLJ)) * FYN) + (FYO * PLK)) - (FYQ * PLL)) / FYP) - (FZR * PLM)) / FZQ;
            let PLO = XS * PKW;
            let PLP = PKY * XS;
            let PLQ = D + PKQ;
            let PLR = (PLM * PLG) * (if ((PKQ + PLO) - (((LA * PLQ) * PLO) * PKZ)) >= AAD { ((PKQ + PLO) - (((LA * PLQ) * PLO) * PKZ)) } else { AAD });
            let PLV = if PLS != 0.0 {
                let PLT = FRX / FPF;
                let PLU = PLR + (((((staged[280] * FZY) * DGH) * DJ) / (((D + (PLT * PLT)) * PLC) * PLC)) / staged[281]);
                PLU
            } else {
                PLR
            };
            let PLW = ((ISY * IST) * OB) / ISL;
            let PLX = D + ITF;
            let PLY = PLX * PLW;
            let PLZ = (IUO * PLW) + ((((((IUH * IST) + (IUC * ISY)) * OB) - (ITU * PLW)) / ISL) * PLX);
            let PMA = YY * QU;
            let PMB = GO - (PMA * PKU);
            let PMC = PLY * PMB;
            let PMD = (PLZ * PMB) + (((PKV * PMA) * AC) * PLY);
            let PME = PLY - PMC;
            let PMF = PLZ - PMD;
            let PMT;
            let PMU;
            let PMV;
            let PMW;
            if PMG != 0.0 {
                let PMH = (PKQ + CTP) - PLO;
                let PMI = PKR - PLP;
                let PMK = PMJ * PKW;
                let PML = PLQ - PLO;
                let PMM = PMK * PML;
                let PMN = ((PKQ / XS) - (PKW * PMH)) - (PMM * PKZ);
                let PMO = if PMN >= AAD { PMN } else { AAD };
                let PMP = PLM * PLC;
                let PMQ = (PMP * PLC) / PMO;
                let PMR = (((((PLN * PLC) + (PLD * PLM)) * PLC) + (PLD * PMP)) - (((((PKR / XS) - ((PKY * PMH) + (PMI * PKW))) - (((((PKY * PMJ) * PML) + (PMI * PMK)) * PKZ) + (FSH * PMM))) * (if PMN >= AAD { 1.0 } else { 0.0 })) * PMQ)) / PMO;
                let PMS = if PLV > B { 1.0 } else { 0.0 };
                let POI;
                let POJ;
                if PMS != 0.0 {
                    let POC = PLG * PKU;
                    let POE = (PKQ + (POD * PKW)) - (PKQ * PLO);
                    let POF = (D - PLO) - (POE * PKZ);
                    let POG = POC * POF;
                    let POH = (((PLH * PKU) + (PKV * PLG)) * POF) + (((PLP * AC) - ((((PKR + (PKY * POD)) - ((PKR * PLO) + (PLP * PKQ))) * PKZ) + (FSH * POE))) * POC);
                    POI = POG;
                    POJ = POH;
                } else {
                    POI = B;
                    POJ = AFD;
                }
                PMT = PMQ;
                PMU = POI;
                PMV = PMR;
                PMW = POJ;
            } else {
                PMT = D;
                PMU = B;
                PMV = AFD;
                PMW = AFD;
            }
            let PMY = PMT * PMX;
            let PMZ = PMV * PMX;
            let PNB = Lanes([PMZ[0], 0.0, PMZ[1], PMZ[2], PMZ[3], PMZ[4]]) + Lanes([0.0, (PNA * PMT), 0.0, 0.0, 0.0, 0.0]);
            let PNC = PLY * PMX;
            let PND = PLZ * PMX;
            let PNE = Lanes([PND[0], 0.0, PND[1], PND[2], PND[3], PND[4]]) + Lanes([0.0, (PNA * PLY), 0.0, 0.0, 0.0, 0.0]);
            let PNF = ddt(80679, PNC);
            let PNG = PNE * PJB;
            let PNH = -PME;
            let PNI = PNH * PMX;
            let PNJ = (PMF * AC) * PMX;
            let PNK = Lanes([PNJ[0], 0.0, PNJ[1], PNJ[2], PNJ[3], PNJ[4]]) + Lanes([0.0, (PNA * PNH), 0.0, 0.0, 0.0, 0.0]);
            let PNL = ddt(80684, PNI);
            let PNM = PNK * PJB;
            let PNN = -PMC;
            let PNO = PNN * PMX;
            let PNP = (PMD * AC) * PMX;
            let PNQ = Lanes([PNP[0], 0.0, PNP[1], PNP[2], PNP[3], PNP[4]]) + Lanes([0.0, (PNA * PNN), 0.0, 0.0, 0.0, 0.0]);
            let PNR = ddt(80689, PNO);
            let PNS = PNQ * PJB;
            let PNT = QU * PMU;
            let PNU = PMW * QU;
            let PNV = PMY + PNF;
            let PNW = PNT * PNV;
            let PNX = PNU * PNV;
            let PNY = Lanes([PNX[0], 0.0, PNX[1], PNX[2], PNX[3], PNX[4]]) + ((PNB + PNG) * PNT);
            let PNZ = PNT * PNC;
            let POA = PNU * PNC;
            let POB = Lanes([POA[0], 0.0, POA[1], POA[2], POA[3], POA[4]]) + (PNE * PNT);
            let POL = RG * POK;
            let POM = GO * ((QT * POK) - POL);
            let PON = (AIU / AIR) / AJL;
            let POO = (AIR / AIU) / AJK;
            let POP = D + PON;
            let POQ = (POP * (((((AIR * POP) * AKQ) / AJH).ln()) + LA)) - (AIP * PON);
            let POR = ((D + (D / POO)) * (((((AIU * (D + POO)) * AKQ) / AJH).ln()) + LA)) - (AIP / POO);
            let POS = POQ - POR;
            let POV = ((((DD * ((((((((GO * ((POQ + POR) - (((POS * POS) + POT).sqrt()))) - AHY) / AIA) + AHY) - AGJ) / AGV) - AHQ) + AGJ)) + CI) - POU) * POK) - POM;
            let POW = ((RS - staged[290]) * POK) - POM;
            let PPA;
            if J != 0.0 {
                let POY = (POX * (POV - POW)) / UI;
                let POZ = if POY < B { 1.0 } else { 0.0 };
                let PPF = if POZ != 0.0 {
                    let PPD = -2e0f64 * ((D - POY).ln());
                    PPD
                } else {
                    let PPE = (POY * POY) / (D + ((LA * POY) / UI));
                    PPE
                };
                let PPG = POW + (POX * PPF);
                PPA = PPG;
            } else {
                PPA = POW;
            }
            let PPB = POV - PPA;
            let PPC = SI * PPB;
            let PPT;
            let PPU;
            let PPV;
            let PPW;
            if L != 0.0 {
                let PPH = PPC - ADO;
                let PPJ = -PPC;
                let PPK = PPJ - ADO;
                let PPM = PPL * ((-3.333333333333e-1f64 * ((GO * ((PPC + ADO) + (((PPH * PPH) + PPI).sqrt()))).ln())).exp());
                let PPN = PPL * ((-3.333333333333e-1f64 * ((GO * ((PPJ + ADO) + (((PPK * PPK) + PPI).sqrt()))).ln())).exp());
                let PPO = (D - PPM) - PPN;
                let PPP = AEK / PPO;
                let PPQ = (AEN * PPO) / (D + (AEN * PPM));
                let PPR = (TG * PPO) / (D + (TG * PPN));
                let PPS = D / ((D + (D / PPQ)) + (D / PPR));
                PPT = PPS;
                PPU = PPQ;
                PPV = PPR;
                PPW = PPP;
            } else {
                PPT = SI;
                PPU = AEN;
                PPV = TG;
                PPW = AEK;
            }
            let PPX = PPT * PPB;
            let PPY = if PPX > B { 1.0 } else { 0.0 };
            let PQC;
            if PPY != 0.0 {
                let PPZ = -PPX;
                let PQA = if PPZ < SY { 1.0 } else { 0.0 };
                let PRB = if PQA != 0.0 {
                    let PRA = (D + (PPZ.exp())).ln();
                    PRA
                } else {
                    PPZ
                };
                let PRC = ((POV - (PPX / PPU)) + PRB) - HW;
                PQC = PRC;
            } else {
                let PQB = if PPX < SY { 1.0 } else { 0.0 };
                let PRE = if PQB != 0.0 {
                    let PRD = (D + (PPX.exp())).ln();
                    PRD
                } else {
                    PPX
                };
                let PRF = ((PPA + (PPX / PPV)) + PRE) - HW;
                PQC = PRF;
            }
            let PQE = PQC - PQD;
            let PQF = GO * ((PQC + PQD) - (((PQE * PQE) + XQ).sqrt()));
            let PQH = ((D + ((LA * (PQD - PQF)) / PQG)).sqrt()) - D;
            let PQI = PQF + (PQG * PQH);
            let PQJ = D + (AGL * POW);
            let PQK = PQJ - GO;
            let PQL = GO * ((PQJ + GO) + (((PQK * PQK) + NI).sqrt()));
            let PQM = D / (D + (AGT * PQL));
            let PQN = D / (D + (AGX * PQL));
            let PQO = ((staged[297] * (((D + (POL / staged[296])).sqrt()) - D)) * (D + (AHH * PQH))) * (D + (AHL * POW));
            let PQP = AM * PQO;
            let PQQ = ((((POV - PQI) + PQP) * PQM) + PQI) + POM;
            let PQR = PQQ + (AIJ * ((((((PPA - PQI) + (AN * PQO)) * PQN) + PQI) + POM) - PQQ));
            let PQS = PPU / PQM;
            let PQT = PPV / PQN;
            let PQU = staged[300] / (PPW * PPW);
            let PQV = D + PQS;
            let PQW = D + PQT;
            let PQX = PQV / PQW;
            let PQY = PQX.ln();
            let PQZ = if PQY > KS { 1.0 } else { 0.0 };
            let PRI = if PQZ != 0.0 {
                let PRG = ((LA * PQY) * (PQX + D)) / (PQX - D);
                PRG
            } else {
                let PRH = LA * (LA + PQY);
                PRH
            };
            let PRJ = (PQT / PQS) / PQW;
            let PRK = (PQS / PQT) / PQV;
            let PRL = D + PRJ;
            let PRN = (PRL * ((((((PQS * PRL) * PRI) / PQU).ln()) + LA) + PRM)) - (PQR * PRJ);
            let PRO = ((D + (D / PRK)) * ((((((PQT * (D + PRK)) * PRI) / PQU).ln()) + LA) + PRM)) - (PQR / PRK);
            let PRP = PRN - PRO;
            let PRQ = (((((((((GO * ((PRN + PRO) - (((PRP * PRP) + POT).sqrt()))) - AHY) / AIA) + AHY) - PQI) / PQM) - PQP) + PQI) / POK) + POU;
            if IRH != 0.0 {
                let PRR = (ITB / ISY) / (D + ITB);
                let PRS = (ISY / ITB) / (D + ISY);
                let PRT = D + PRR;
                let PRU = (PRT * (((((ISY * PRT) * ITG) / ITH).ln()) + LA)) - (ITI * PRR);
                let PRV = ((D + (D / PRS)) * (((((ITB * (D + PRS)) * ITG) / ITH).ln()) + LA)) - (ITI / PRS);
                let PRW = PRU - PRV;
                let PRX = ((((DD * ((((((((GO * ((PRU + PRV) - (((PRW * PRW) + POT).sqrt()))) - ITJ) / AIA) + ITJ) - ITK) / ITA) - ITL) + ITK)) + DW) - staged[302]) * POK) - POM;
                let PRY = ((RS - staged[303]) * POK) - POM;
                let PSD;
                if J != 0.0 {
                    let PSB = (PSA * (PRX - PRY)) / UI;
                    let PSC = if PSB < B { 1.0 } else { 0.0 };
                    let PSI = if PSC != 0.0 {
                        let PSG = -2e0f64 * ((D - PSB).ln());
                        PSG
                    } else {
                        let PSH = (PSB * PSB) / (D + ((LA * PSB) / UI));
                        PSH
                    };
                    let PSJ = PRY + (PSA * PSI);
                    PSD = PSJ;
                } else {
                    PSD = PRY;
                }
                let PSE = PRX - PSD;
                let PSF = SI * PSE;
                let PSU;
                let PSV;
                let PSW;
                if L != 0.0 {
                    let PSK = PSF - ADO;
                    let PSM = -PSF;
                    let PSN = PSM - ADO;
                    let PSO = PPL * ((-3.333333333333e-1f64 * ((GO * ((PSF + ADO) + (((PSK * PSK) + PSL).sqrt()))).ln())).exp());
                    let PSP = PPL * ((-3.333333333333e-1f64 * ((GO * ((PSM + ADO) + (((PSN * PSN) + PSL).sqrt()))).ln())).exp());
                    let PSQ = (D - PSO) - PSP;
                    let PSR = (AEN * PSQ) / (D + (AEN * PSO));
                    let PSS = (TG * PSQ) / (D + (TG * PSP));
                    let PST = D / ((D + (D / PSR)) + (D / PSS));
                    PSU = PST;
                    PSV = PSR;
                    PSW = PSS;
                } else {
                    PSU = SI;
                    PSV = AEN;
                    PSW = TG;
                }
                let PSX = PSU * PSE;
                let PSY = if PSX > B { 1.0 } else { 0.0 };
                if PSY != 0.0 {
                    let PSZ = if (-PSX) < SY { 1.0 } else { 0.0 };
                } else {
                    let PTA = if PSX < SY { 1.0 } else { 0.0 };
                }
                let PTB = D + (AGL * PRY);
                let PTC = PTB - GO;
                let PTD = GO * ((PTB + GO) + (((PTC * PTC) + NI).sqrt()));
                let PTE = if (((D + (PSV / (D / (D + (JIF * PTD))))) / (D + (PSW / (D / (D + (JIJ * PTD)))))).ln()) > KS { 1.0 } else { 0.0 };
            } else {
            }
            let PRZ = QR - PRQ;
            let PTF;
            let PTG;
            let PTH;
            let PTI;
            let PTJ;
            let PTK;
            if PFK != 0.0 {
                PTF = PFD;
                PTG = PFB;
                PTH = PEX;
                PTI = PFE;
                PTJ = PFC;
                PTK = PEY;
            } else {
                PTF = PFB;
                PTG = PFD;
                PTH = PEZ;
                PTI = PFC;
                PTJ = PFE;
                PTK = PFA;
            }
            let PTL = ((PEV + PTF) - PTG) - PTH;
            let PTM = ((PEW + PTI) - PTJ) - PTK;
            let PTP = if PFK != 0.0 {
                let PTN = IY * PTM[1];
                PTN
            } else {
                let PTO = IY * PTM[2];
                PTO
            };
            let PTQ = (((((PJX + PJI) + PIA) + PIC) + Lanes([0.0, PIE[0], PIE[1], PIE[2], PIE[3]])) + PIY) + PIW;
            let PTR = IY * PTQ[4];
            let PTS = if (PTP.abs()) < AAD { 1.0 } else { 0.0 };
            let PTT = if (PRZ.abs()) < AAD { 1.0 } else { 0.0 };
            let PTU = if (PTR.abs()) < AAD { 1.0 } else { 0.0 };
            let PTV = if (PTL.abs()) < AAD { 1.0 } else { 0.0 };
            let PTW = PFR[0];
            let PTX = PFR[1];
            let PTY = PFR[2];
            let PTZ = PFR[3];
            let PUA = PFR[4];
            let PUB = PFS[0];
            let PUC = PFS[1];
            let PUD = PFS[2];
            let PUE = PFS[3];
            let PUF = PFS[4];
            let PUG = PFU[0];
            let PUH = PFU[1];
            let PUI = PFU[2];
            let PUJ = PFU[3];
            let PUK = PFU[4];
            let PUL = PFW[0];
            let PUM = PFW[1];
            let PUN = PFW[2];
            let PUO = PFW[3];
            let PUP = PFW[4];
            let PUQ = PFY[0];
            let PUR = PFY[1];
            let PUS = PFY[2];
            let PUT = PFY[3];
            let PUU = PFY[4];
            let PUV = PGB[0];
            let PUW = PGB[1];
            let PUX = PFH[0];
            let PUY = PFH[1];
            let PUZ = PFH[2];
            let PVA = PFH[3];
            let PVB = PFH[4];
            let PVC = PFJ;
            let PVD = PGI[0];
            let PVE = PGI[1];
            let PVF = PGP[0];
            let PVG = PGP[1];
            let PVH = PGW[0];
            let PVI = PGW[1];
            let PVJ = PHD[0];
            let PVK = PHD[1];
            let PVL = PJE[0];
            let PVM = PJE[1];
            let PVN = PJE[2];
            let PVO = PJE[3];
            let PVP = PJE[4];
            let PVQ = PJK[0];
            let PVR = PJK[1];
            let PVS = PJK[2];
            let PVT = PJK[3];
            let PVU = PJK[4];
            let PVV = PJP[0];
            let PVW = PJP[1];
            let PVX = PJP[2];
            let PVY = PJP[3];
            let PVZ = PJP[4];
            let PWA = PJT[0];
            let PWB = PJT[1];
            let PWC = PJT[2];
            let PWD = PJT[3];
            let PWE = PJZ[0];
            let PWF = PJZ[1];
            let PWG = PJZ[2];
            let PWH = PJZ[3];
            let PWI = PJZ[4];
            let PWJ = PKE[0];
            let PWK = PKE[1];
            let PWL = PKE[2];
            let PWM = PKE[3];
            let PWN = PKE[4];
            let PWO = PKI;
            let PWP = PNB[0];
            let PWQ = PNB[1];
            let PWR = PNB[2];
            let PWS = PNB[3];
            let PWT = PNB[4];
            let PWU = PNB[5];
            let PWV = PNG[0];
            let PWW = PNG[1];
            let PWX = PNG[2];
            let PWY = PNG[3];
            let PWZ = PNG[4];
            let PXA = PNG[5];
            let PXB = PNM[0];
            let PXC = PNM[1];
            let PXD = PNM[2];
            let PXE = PNM[3];
            let PXF = PNM[4];
            let PXG = PNM[5];
            let PXH = PNS[0];
            let PXI = PNS[1];
            let PXJ = PNS[2];
            let PXK = PNS[3];
            let PXL = PNS[4];
            let PXM = PNS[5];
            let PXN = PNY[0];
            let PXO = PNY[1];
            let PXP = PNY[2];
            let PXQ = PNY[3];
            let PXR = PNY[4];
            let PXS = PNY[5];
            let PXT = PJG[0];
            let PXU = PJG[1];
            let PXV = PJG[2];
            let PXW = PJG[3];
            let PXX = PJG[4];
            let PXY = PJM[0];
            let PXZ = PJM[1];
            let PYA = PJM[2];
            let PYB = PJM[3];
            let PYC = PJM[4];
            let PYD = PJR[0];
            let PYE = PJR[1];
            let PYF = PJR[2];
            let PYG = PJR[3];
            let PYH = PJR[4];
            let PYI = PJV[0];
            let PYJ = PJV[1];
            let PYK = PJV[2];
            let PYL = PJV[3];
            let PYM = PKB[0];
            let PYN = PKB[1];
            let PYO = PKB[2];
            let PYP = PKB[3];
            let PYQ = PKB[4];
            let PYR = PKG[0];
            let PYS = PKG[1];
            let PYT = PKG[2];
            let PYU = PKG[3];
            let PYV = PKG[4];
            let PYW = PIM;
            let PYX = PNE[0];
            let PYY = PNE[1];
            let PYZ = PNE[2];
            let PZA = PNE[3];
            let PZB = PNE[4];
            let PZC = PNE[5];
            let PZD = PNK[0];
            let PZE = PNK[1];
            let PZF = PNK[2];
            let PZG = PNK[3];
            let PZH = PNK[4];
            let PZI = PNK[5];
            let PZJ = PNQ[0];
            let PZK = PNQ[1];
            let PZL = PNQ[2];
            let PZM = PNQ[3];
            let PZN = PNQ[4];
            let PZO = PNQ[5];
            let PZP = POB[0];
            let PZQ = POB[1];
            let PZR = POB[2];
            let PZS = POB[3];
            let PZT = POB[4];
            let PZU = POB[5];
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (PFP),
            [4, 6, 7, 8, 9],
            [PTW, PTX, PTY, PTZ, PUA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (PFQ),
            [4, 6, 7, 8, 9],
            [PUB, PUC, PUD, PUE, PUF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (PFT),
            [4, 6, 7, 8, 9],
            [PUG, PUH, PUI, PUJ, PUK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (PFV),
            [4, 6, 7, 8, 9],
            [PUL, PUM, PUN, PUO, PUP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (PFX),
            [4, 6, 7, 8, 9],
            [PUQ, PUR, PUS, PUT, PUU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(8),
            multiplicity * (PZV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (PZW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(8),
            multiplicity * (PZX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(7),
            Some(6),
            multiplicity * (PGA),
            [6, 7],
            [PUV, PUW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (PFG),
            [4, 6, 7, 8, 9],
            [PUX, PUY, PUZ, PVA, PVB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (PFI),
            [4],
            [PVC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(9),
            multiplicity * (PGH),
            [1, 9],
            [PVD, PVE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (staged[471]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(9), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[472],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(2),
            Some(6),
            multiplicity * (PGO),
            [2, 6],
            [PVF, PVG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(6),
            multiplicity * (staged[473]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(2), Some(6), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[474],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(0),
            Some(7),
            multiplicity * (PGV),
            [0, 7],
            [PVH, PVI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(7),
            multiplicity * (staged[475]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(7), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[476],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(8),
            multiplicity * (PHC),
            [3, 8],
            [PVJ, PVK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(8),
            multiplicity * (staged[477]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(3), Some(8), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[478],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (PJD),
            [4, 6, 7, 8, 9],
            [PVL, PVM, PVN, PVO, PVP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (PJJ),
            [4, 6, 7, 8, 9],
            [PVQ, PVR, PVS, PVT, PVU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (PJO),
            [4, 6, 7, 8, 9],
            [PVV, PVW, PVX, PVY, PVZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(8),
            multiplicity * (PJS),
            [6, 7, 8, 9],
            [PWA, PWB, PWC, PWD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (PJY),
            [4, 6, 7, 8, 9],
            [PWE, PWF, PWG, PWH, PWI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(6),
            multiplicity * (PKD),
            [4, 6, 7, 8, 9],
            [PWJ, PWK, PWL, PWM, PWN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (PKH),
            [4],
            [PWO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            None,
            multiplicity * (PZY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (PMY),
            [4, 5, 6, 7, 8, 9],
            [PWP, PWQ, PWR, PWS, PWT, PWU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (PNF),
            [4, 5, 6, 7, 8, 9],
            [PWV, PWW, PWX, PWY, PWZ, PXA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(6),
            multiplicity * (PNL),
            [4, 5, 6, 7, 8, 9],
            [PXB, PXC, PXD, PXE, PXF, PXG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (PNR),
            [4, 5, 6, 7, 8, 9],
            [PXH, PXI, PXJ, PXK, PXL, PXM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (PZZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (PNW),
            [4, 5, 6, 7, 8, 9],
            [PXN, PXO, PXP, PXQ, PXR, PXS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (QAA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(6),
            multiplicity * (QAB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (QAC),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (QAD),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = PFP;
        self.canonical_reactive[1] = PFQ;
        self.canonical_reactive[2] = PFT;
        self.canonical_reactive[3] = PFV;
        self.canonical_reactive[4] = PFX;
        self.canonical_reactive[5] = PZV;
        self.canonical_reactive[6] = PZW;
        self.canonical_reactive[7] = PZX;
        self.canonical_reactive[8] = PGA;
        self.canonical_reactive[9] = PFG;
        self.canonical_reactive[10] = PFI;
        self.canonical_reactive[11] = PGH;
        self.canonical_reactive[12] = staged[471];
        self.canonical_reactive[13] = staged[472];
        self.canonical_reactive[14] = PGO;
        self.canonical_reactive[15] = staged[473];
        self.canonical_reactive[16] = staged[474];
        self.canonical_reactive[17] = PGV;
        self.canonical_reactive[18] = staged[475];
        self.canonical_reactive[19] = staged[476];
        self.canonical_reactive[20] = PHC;
        self.canonical_reactive[21] = staged[477];
        self.canonical_reactive[22] = staged[478];
        self.canonical_reactive[23] = PJF;
        self.canonical_reactive[24] = PXT;
        self.canonical_reactive[25] = PXU;
        self.canonical_reactive[26] = PXV;
        self.canonical_reactive[27] = PXW;
        self.canonical_reactive[28] = PXX;
        self.canonical_reactive[29] = PJL;
        self.canonical_reactive[30] = PXY;
        self.canonical_reactive[31] = PXZ;
        self.canonical_reactive[32] = PYA;
        self.canonical_reactive[33] = PYB;
        self.canonical_reactive[34] = PYC;
        self.canonical_reactive[35] = PJQ;
        self.canonical_reactive[36] = PYD;
        self.canonical_reactive[37] = PYE;
        self.canonical_reactive[38] = PYF;
        self.canonical_reactive[39] = PYG;
        self.canonical_reactive[40] = PYH;
        self.canonical_reactive[41] = PJU;
        self.canonical_reactive[42] = PYI;
        self.canonical_reactive[43] = PYJ;
        self.canonical_reactive[44] = PYK;
        self.canonical_reactive[45] = PYL;
        self.canonical_reactive[46] = PKA;
        self.canonical_reactive[47] = PYM;
        self.canonical_reactive[48] = PYN;
        self.canonical_reactive[49] = PYO;
        self.canonical_reactive[50] = PYP;
        self.canonical_reactive[51] = PYQ;
        self.canonical_reactive[52] = PKF;
        self.canonical_reactive[53] = PYR;
        self.canonical_reactive[54] = PYS;
        self.canonical_reactive[55] = PYT;
        self.canonical_reactive[56] = PYU;
        self.canonical_reactive[57] = PYV;
        self.canonical_reactive[58] = PIL;
        self.canonical_reactive[59] = PYW;
        self.canonical_reactive[60] = PZY;
        self.canonical_reactive[61] = PMY;
        self.canonical_reactive[62] = PNC;
        self.canonical_reactive[63] = PYX;
        self.canonical_reactive[64] = PYY;
        self.canonical_reactive[65] = PYZ;
        self.canonical_reactive[66] = PZA;
        self.canonical_reactive[67] = PZB;
        self.canonical_reactive[68] = PZC;
        self.canonical_reactive[69] = PNI;
        self.canonical_reactive[70] = PZD;
        self.canonical_reactive[71] = PZE;
        self.canonical_reactive[72] = PZF;
        self.canonical_reactive[73] = PZG;
        self.canonical_reactive[74] = PZH;
        self.canonical_reactive[75] = PZI;
        self.canonical_reactive[76] = PNO;
        self.canonical_reactive[77] = PZJ;
        self.canonical_reactive[78] = PZK;
        self.canonical_reactive[79] = PZL;
        self.canonical_reactive[80] = PZM;
        self.canonical_reactive[81] = PZN;
        self.canonical_reactive[82] = PZO;
        self.canonical_reactive[83] = PZZ;
        self.canonical_reactive[84] = PNZ;
        self.canonical_reactive[85] = PZP;
        self.canonical_reactive[86] = PZQ;
        self.canonical_reactive[87] = PZR;
        self.canonical_reactive[88] = PZS;
        self.canonical_reactive[89] = PZT;
        self.canonical_reactive[90] = PZU;
        self.canonical_reactive[91] = QAA;
        self.canonical_reactive[92] = QAB;
        self.canonical_reactive[93] = QAC;
        self.canonical_reactive[94] = QAD;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 6, 7, 8, 9],
            &[cached[24], cached[25], cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(7),
            &[4, 6, 7, 8, 9],
            &[cached[30], cached[31], cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(7),
            &[4, 6, 7, 8, 9],
            &[cached[36], cached[37], cached[38], cached[39], cached[40]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(8),
            &[6, 7, 8, 9],
            &[cached[42], cached[43], cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(6),
            &[4, 6, 7, 8, 9],
            &[cached[47], cached[48], cached[49], cached[50], cached[51]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(6),
            &[4, 6, 7, 8, 9],
            &[cached[53], cached[54], cached[55], cached[56], cached[57]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[59]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            None,
            &[4, 5, 6, 7, 8, 9],
            &[cached[63], cached[64], cached[65], cached[66], cached[67], cached[68]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(6),
            &[4, 5, 6, 7, 8, 9],
            &[cached[70], cached[71], cached[72], cached[73], cached[74], cached[75]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(7),
            &[4, 5, 6, 7, 8, 9],
            &[cached[77], cached[78], cached[79], cached[80], cached[81], cached[82]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(6),
            &[4, 5, 6, 7, 8, 9],
            &[cached[85], cached[86], cached[87], cached[88], cached[89], cached[90]],
            &[],
            &[],
            multiplicity,
        );
    }

}
