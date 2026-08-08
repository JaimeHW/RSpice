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
        let mut key = Vec::with_capacity(1924);
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
        self.canonical_staged[440] = values[1];
        self.canonical_staged[441] = values[2];
        self.canonical_staged[442] = values[3];
        self.canonical_staged[443] = values[4];
        self.canonical_staged[444] = values[5];
        self.canonical_staged[445] = values[6];
        self.canonical_staged[446] = values[7];
        self.canonical_staged[447] = values[8];
        self.canonical_staged[448] = values[9];
        self.canonical_staged[80] = values[10];
        self.canonical_staged[449] = values[11];
        self.canonical_staged[450] = values[12];
        self.canonical_staged[451] = values[13];
        self.canonical_staged[23] = values[14];
        self.canonical_staged[130] = values[15];
        self.canonical_staged[66] = values[16];
        self.canonical_staged[452] = values[17];
        self.canonical_staged[1] = values[18];
        self.canonical_staged[2] = values[19];
        self.canonical_staged[460] = values[20];
        self.canonical_staged[816] = values[21];
        self.canonical_staged[461] = values[22];
        self.canonical_staged[462] = values[23];
        self.canonical_staged[3] = values[24];
        self.canonical_staged[315] = values[25];
        self.canonical_staged[464] = values[26];
        self.canonical_staged[466] = values[27];
        self.canonical_staged[470] = values[28];
        self.canonical_staged[471] = values[29];
        self.canonical_staged[20] = values[30];
        self.canonical_staged[21] = values[31];
        self.canonical_staged[22] = values[32];
        self.canonical_staged[472] = values[33];
        self.canonical_staged[473] = values[34];
        self.canonical_staged[56] = values[35];
        self.canonical_staged[24] = values[36];
        self.canonical_staged[498] = values[37];
        self.canonical_staged[55] = values[38];
        self.canonical_staged[504] = values[39];
        self.canonical_staged[57] = values[40];
        self.canonical_staged[58] = values[41];
        self.canonical_staged[507] = values[42];
        self.canonical_staged[60] = values[43];
        self.canonical_staged[63] = values[44];
        self.canonical_staged[65] = values[45];
        self.canonical_staged[67] = values[46];
        self.canonical_staged[69] = values[47];
        self.canonical_staged[71] = values[48];
        self.canonical_staged[73] = values[49];
        self.canonical_staged[74] = values[50];
        self.canonical_staged[512] = values[51];
        self.canonical_staged[517] = values[52];
        self.canonical_staged[75] = values[53];
        self.canonical_staged[76] = values[54];
        self.canonical_staged[77] = values[55];
        self.canonical_staged[303] = values[56];
        self.canonical_staged[518] = values[57];
        self.canonical_staged[519] = values[58];
        self.canonical_staged[522] = values[59];
        self.canonical_staged[520] = values[60];
        self.canonical_staged[78] = values[61];
        self.canonical_staged[526] = values[62];
        self.canonical_staged[527] = values[63];
        self.canonical_staged[528] = values[64];
        self.canonical_staged[81] = values[65];
        self.canonical_staged[529] = values[66];
        self.canonical_staged[530] = values[67];
        self.canonical_staged[532] = values[68];
        self.canonical_staged[86] = values[69];
        self.canonical_staged[535] = values[70];
        self.canonical_staged[91] = values[71];
        self.canonical_staged[542] = values[72];
        self.canonical_staged[543] = values[73];
        self.canonical_staged[103] = values[74];
        self.canonical_staged[548] = values[75];
        self.canonical_staged[104] = values[76];
        self.canonical_staged[105] = values[77];
        self.canonical_staged[552] = values[78];
        self.canonical_staged[106] = values[79];
        self.canonical_staged[107] = values[80];
        self.canonical_staged[108] = values[81];
        self.canonical_staged[112] = values[82];
        self.canonical_staged[110] = values[83];
        self.canonical_staged[122] = values[84];
        self.canonical_staged[136] = values[85];
        self.canonical_staged[572] = values[86];
        self.canonical_staged[583] = values[87];
        self.canonical_staged[584] = values[88];
        self.canonical_staged[585] = values[89];
        self.canonical_staged[586] = values[90];
        self.canonical_staged[589] = values[91];
        self.canonical_staged[590] = values[92];
        self.canonical_staged[592] = values[93];
        self.canonical_staged[593] = values[94];
        self.canonical_staged[607] = values[95];
        self.canonical_staged[232] = values[96];
        self.canonical_staged[139] = values[97];
        self.canonical_staged[624] = values[98];
        self.canonical_staged[625] = values[99];
        self.canonical_staged[626] = values[100];
        self.canonical_staged[637] = values[101];
        self.canonical_staged[644] = values[102];
        self.canonical_staged[645] = values[103];
        self.canonical_staged[655] = values[104];
        self.canonical_staged[656] = values[105];
        self.canonical_staged[657] = values[106];
        self.canonical_staged[658] = values[107];
        self.canonical_staged[659] = values[108];
        self.canonical_staged[660] = values[109];
        self.canonical_staged[661] = values[110];
        self.canonical_staged[662] = values[111];
        self.canonical_staged[663] = values[112];
        self.canonical_staged[664] = values[113];
        self.canonical_staged[665] = values[114];
        self.canonical_staged[666] = values[115];
        self.canonical_staged[667] = values[116];
        self.canonical_staged[669] = values[117];
        self.canonical_staged[672] = values[118];
        self.canonical_staged[673] = values[119];
        self.canonical_staged[674] = values[120];
        self.canonical_staged[675] = values[121];
        self.canonical_staged[676] = values[122];
        self.canonical_staged[677] = values[123];
        self.canonical_staged[678] = values[124];
        self.canonical_staged[681] = values[125];
        self.canonical_staged[682] = values[126];
        self.canonical_staged[685] = values[127];
        self.canonical_staged[686] = values[128];
        self.canonical_staged[687] = values[129];
        self.canonical_staged[688] = values[130];
        self.canonical_staged[689] = values[131];
        self.canonical_staged[690] = values[132];
        self.canonical_staged[691] = values[133];
        self.canonical_staged[692] = values[134];
        self.canonical_staged[693] = values[135];
        self.canonical_staged[694] = values[136];
        self.canonical_staged[695] = values[137];
        self.canonical_staged[696] = values[138];
        self.canonical_staged[697] = values[139];
        self.canonical_staged[698] = values[140];
        self.canonical_staged[699] = values[141];
        self.canonical_staged[700] = values[142];
        self.canonical_staged[701] = values[143];
        self.canonical_staged[702] = values[144];
        self.canonical_staged[703] = values[145];
        self.canonical_staged[704] = values[146];
        self.canonical_staged[705] = values[147];
        self.canonical_staged[706] = values[148];
        self.canonical_staged[707] = values[149];
        self.canonical_staged[708] = values[150];
        self.canonical_staged[709] = values[151];
        self.canonical_staged[710] = values[152];
        self.canonical_staged[711] = values[153];
        self.canonical_staged[712] = values[154];
        self.canonical_staged[713] = values[155];
        self.canonical_staged[714] = values[156];
        self.canonical_staged[715] = values[157];
        self.canonical_staged[716] = values[158];
        self.canonical_staged[717] = values[159];
        self.canonical_staged[718] = values[160];
        self.canonical_staged[719] = values[161];
        self.canonical_staged[142] = values[162];
        self.canonical_staged[158] = values[163];
        self.canonical_staged[768] = values[164];
        self.canonical_staged[165] = values[165];
        self.canonical_staged[164] = values[166];
        self.canonical_staged[792] = values[167];
        self.canonical_staged[255] = values[168];
        self.canonical_staged[254] = values[169];
        self.canonical_staged[794] = values[170];
        self.canonical_staged[256] = values[171];
        self.canonical_staged[795] = values[172];
        self.canonical_staged[796] = values[173];
        self.canonical_staged[312] = values[174];
        self.canonical_staged[801] = values[175];
        self.canonical_staged[356] = values[176];
        self.canonical_staged[349] = values[177];
        self.canonical_staged[817] = values[178];
        self.canonical_staged[818] = values[179];
        self.canonical_staged[827] = values[180];
        self.canonical_staged[829] = values[181];
        self.canonical_staged[369] = values[182];
        self.canonical_staged[387] = values[183];
        self.canonical_staged[388] = values[184];
        self.canonical_staged[841] = values[185];
        self.canonical_staged[843] = values[186];
        self.canonical_staged[393] = values[187];
        self.canonical_staged[399] = values[188];
        self.canonical_staged[848] = values[189];
        self.canonical_staged[849] = values[190];
        self.canonical_staged[850] = values[191];
        self.canonical_staged[851] = values[192];
        self.canonical_staged[853] = values[193];
        self.canonical_staged[854] = values[194];
        self.canonical_staged[852] = values[195];
        self.canonical_staged[855] = values[196];
        self.canonical_staged[856] = values[197];
        self.canonical_staged[857] = values[198];
        self.canonical_staged[858] = values[199];
        self.canonical_staged[428] = values[200];
        self.canonical_staged[430] = values[201];
        self.canonical_staged[861] = values[202];
        self.canonical_staged[859] = values[203];
        self.canonical_staged[860] = values[204];
        self.canonical_staged[862] = values[205];
        self.canonical_staged[863] = values[206];
        self.canonical_staged[864] = values[207];
        self.canonical_staged[865] = values[208];
        self.canonical_staged[866] = values[209];
        self.canonical_staged[867] = values[210];
        self.canonical_staged[868] = values[211];
        self.canonical_staged[869] = values[212];
        self.canonical_staged[870] = values[213];
        self.canonical_staged[871] = values[214];
        self.canonical_staged[872] = values[215];
        self.canonical_staged[873] = values[216];
        self.canonical_staged[874] = values[217];
        self.canonical_staged[875] = values[218];
        self.canonical_staged[876] = values[219];
        self.canonical_staged[877] = values[220];
        self.canonical_staged[878] = values[221];
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
                let J = parameters[39];
                let K = 8.85418e-12f64;
                let L = parameters[45];
                let O = parameters[43];
                let Q = 3.9e0f64;
                let R = 3.453133e-11f64;
                let S = parameters[64];
                let U = 1.03594e-10f64;
                let V = parameters[44];
                let W = 5.753e-12f64;
                let AC = if parameter_given[203] { 1.0 } else { 0.0 };
                let AD = parameters[203];
                let AE = 1e0f64;
                let AH = if parameter_given[125] { 1.0 } else { 0.0 };
                let AI = parameters[125];
                let AJ = parameters[207];
                let AK = 0e0f64;
                let AL = if parameter_given[207] { 1.0 } else { 0.0 };
                let AO = if parameter_given[124] { 1.0 } else { 0.0 };
                let AQ = 6e-1f64;
                let AR = parameters[149];
                let AU = parameters[124];
                let AX = parameters[171];
                let AY = 1e-1f64;
                let BE = parameters[172];
                let BL = 8.617087e-5f64;
                let BQ = 2e0f64;
                let BV = parameters[359];
                let BY = parameters[40];
                let CA = parameters[35];
                let CC = parameters[335];
                let CF = parameters[368];
                let CG = parameters[364];
                let CI = parameters[367];
                let CK = parameters[410];
                let CQ = parameters[337];
                let CV = if parameter_given[84] { 1.0 } else { 0.0 };
                let CZ = parameters[146];
                let DC = parameters[147];
                let DH = parameters[34];
                let DP = 1.60219e-19f64;
                let DR = 1e-38f64;
                let DT = 5e-1f64;
                let DV = parameters[51];
                let DY = -8.749823353377374e1f64;
                let EC = parameters[992];
                let ED = parameters[991];
                let EF = parameters[994];
                let EG = parameters[993];
                let EL = if parameter_given[89] { 1.0 } else { 0.0 };
                let EM = if parameter_given[93] { 1.0 } else { 0.0 };
                let ES = parameters[87];
                let EU = 7.7348e-4f64;
                let FC = if parameter_given[107] { 1.0 } else { 0.0 };
                let FF = parameters[221];
                let FI = parameters[360];
                let FO = -8.749823353377374e1f64;
                let FQ = parameters[344];
                let FS = parameters[323];
                let FT = 1e-15f64;
                let FX = parameters[67];
                let FZ = parameters[55];
                let GD = parameters[58];
                let GH = parameters[38];
                let GJ = parameters[60];
                let GU = parameters[59];
                let GV = 3e0f64;
                let GX = parameters[343];
                let HB = parameters[61];
                let HD = 1e-9f64;
                let HF = 1e23f64;
                let JC = parameters[353];
                let JE = parameters[354];
                let KI = parameters[126];
                let KQ = parameters[37];
                let KT = parameters[213];
                let KV = 0e0f64;
                let LA = parameters[243];
                let LF = 0e0f64;
                let LI = 0e0f64;
                let LQ = 0e0f64;
                let LR = 0e0f64;
                let LS = 0e0f64;
                let LT = 0e0f64;
                let LY = 1.0f64;
                let LZ = 0e0f64;
                let MA = 0e0f64;
                let MG = 0e0f64;
                let MH = 0e0f64;
                let ML = 0e0f64;
                let MO = 0e0f64;
                let MQ = 0e0f64;
                let MR = 0e0f64;
                let MS = 0e0f64;
                let MT = 0e0f64;
                let mut oAM = 0.0;
                let mut oAV = 0.0;
                let mut oBR = 0.0;
                let mut oCE = 0.0;
                let mut oCY = 0.0;
                let mut oDI = 0.0;
                let mut oDJ = 0.0;
                let mut oDL = 0.0;
                let mut oDN = 0.0;
                let mut oDO = 0.0;
                let mut oDU = 0.0;
                let mut oDW = 0.0;
                let mut oEO = 0.0;
                let mut oEP = 0.0;
                let mut oER = 0.0;
                let mut oEV = 0.0;
                let mut oEW = 0.0;
                let mut oEY = 0.0;
                let mut oEZ = 0.0;
                let mut oFA = 0.0;
                let mut oFD = 0.0;
                let mut oFY = 0.0;
                let mut oGA = 0.0;
                let mut oGB = 0.0;
                let mut oGC = 0.0;
                let mut oGE = 0.0;
                let mut oGF = 0.0;
                let mut oGG = 0.0;
                let mut oHE = 0.0;
                let mut oHG = 0.0;
                let mut oHH = 0.0;
                let mut oHI = 0.0;
                let mut oHJ = 0.0;
                let mut oHK = 0.0;
                let mut oHL = 0.0;
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
                let mut oHY = 0.0;
                let mut oHZ = 0.0;
                let mut oIA = 0.0;
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
                let mut oIN = 0.0;
                let mut oIO = 0.0;
                let mut oIP = 0.0;
                let mut oIQ = 0.0;
                let mut oIR = 0.0;
                let mut oIS = 0.0;
                let mut oIT = 0.0;
                let mut oIU = 0.0;
                let mut oIV = 0.0;
                let mut oIW = 0.0;
                let mut oIX = 0.0;
                let mut oIY = 0.0;
                let mut oIZ = 0.0;
                let mut oJA = 0.0;
                let mut oJB = 0.0;
                let mut oJD = 0.0;
                let mut oJF = 0.0;
                let mut oJG = 0.0;
                let mut oJH = 0.0;
                let mut oJI = 0.0;
                let mut oJJ = 0.0;
                let mut oJK = 0.0;
                let mut oJL = 0.0;
                let mut oJM = 0.0;
                let mut oJN = 0.0;
                let mut oJO = 0.0;
                let mut oJP = 0.0;
                let mut oJV = 0.0;
                let mut oJW = 0.0;
                let mut oKA = 0.0;
                let mut oKB = 0.0;
                let mut oKG = 0.0;
                let mut oKJ = 0.0;
                let mut oKK = 0.0;
                let mut oKL = 0.0;
                let mut oKM = 0.0;
                let mut oKN = 0.0;
                let mut oKO = 0.0;
                let mut oKP = 0.0;
                let mut oKW = 0.0;
                let mut oLC = 0.0;
                let mut oLG = 0.0;
                let mut oLK = 0.0;
                let mut oLM = 0.0;
                let mut oLN = 0.0;
                let mut oLO = 0.0;
                let mut oLP = 0.0;
                let A = parameters[123] + 2.7315e2f64;
                let B = if (if parameter_given[973] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[965] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let C = if (if parameter_given[976] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[966] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let D = if (if parameter_given[979] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[967] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let E = if (if parameter_given[982] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[968] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let F = if (if parameter_given[974] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[969] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let G = if (if parameter_given[977] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[970] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let H = if (if parameter_given[980] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[971] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let I = if (if parameter_given[983] { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[972] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let X;
                let Y;
                let Z;
                let AA;
                let AB;
                if J != 0.0 {
                    let M = K * L;
                    let N = (3.20438e-13f64 * M).sqrt();
                    let P = 3.4531302e-11f64 / O;
                    X = P;
                    Y = M;
                    Z = Q;
                    AA = O;
                    AB = N;
                } else {
                    let T = R / S;
                    X = T;
                    Y = U;
                    Z = V;
                    AA = S;
                    AB = W;
                }
                let AG = if AC != 0.0 {
                    AD
                } else {
                    let AF = 2.1983327444149834e-11f64 * ((AE + (4e-7f64 / S)).ln());
                    AF
                };
                let AN;
                if AH != 0.0 {
                    AN = AI;
                } else {
                    let AM = if AL != 0.0 && (if AJ > AK { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAM = AM;
                    let AT = if AM != 0.0 {
                        let AP = (AJ * X) - parameters[201];
                        AP
                    } else {
                        let AS = (AQ * AR) * X;
                        AS
                    };
                    AN = AT;
                }
                let AW;
                if AO != 0.0 {
                    AW = AU;
                } else {
                    let AV = if AL != 0.0 && (if AJ > AK { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oAV = AV;
                    let BC = if AV != 0.0 {
                        let BA = (AJ * X) - parameters[200];
                        BA
                    } else {
                        let BB = (AQ * AR) * X;
                        BB
                    };
                    AW = BC;
                }
                let AZ = if AX < AY { 1.0 } else { 0.0 };
                let BD = if AZ != 0.0 {
                    AY
                } else {
                    AX
                };
                let BF = if BE < AY { 1.0 } else { 0.0 };
                let BG = if BF != 0.0 {
                    AY
                } else {
                    BE
                };
                let BJ = if J != 0.0 {
                    let BH = ((Y / (Z * K)) * AA).sqrt();
                    BH
                } else {
                    let BI = (3.000000289592089e0f64 * S).sqrt();
                    BI
                };
                let BK = if J == AK { 1.0 } else { 0.0 };
                let BS;
                let BT;
                let BU;
                if BK != 0.0 {
                    let BM = BL * A;
                    let BN = 1.16e0f64 - (((7.02e-4f64 * A) * A) / (A + 1.108e3f64));
                    BS = BM;
                    BT = BN;
                    BU = BN;
                } else {
                    let BO = BL * A;
                    let BP = parameters[47] - (((parameters[48] * A) * A) / (A + parameters[49]));
                    let BR = BP / (BQ * BO);
                    oBR = BR;
                    BS = BO;
                    BT = BP;
                    BU = BP;
                }
                let BW = BQ * BV;
                let BX = if parameters[63] == AE { 1.0 } else { 0.0 };
                let BZ = if BY == AK { 1.0 } else { 0.0 };
                let CB = if BZ != 0.0 && (if CA >= 4.1e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CD = if CC == AK { 1.0 } else { 0.0 };
                if CD != 0.0 {
                } else {
                    let CE = BQ * CC;
                    oCE = CE;
                }
                let CH = CF / CG;
                let CJ = ((CH.powf(CI)) / CG) / CG;
                let CL = if CK == AE { 1.0 } else { 0.0 };
                let CM = if AN < AK { 1.0 } else { 0.0 };
                let CN = if CM != 0.0 {
                    AK
                } else {
                    AN
                };
                let CO = if AW < AK { 1.0 } else { 0.0 };
                let CP = if CO != 0.0 {
                    AK
                } else {
                    AW
                };
                let CR = if CQ < AK { 1.0 } else { 0.0 };
                let CS = if CR != 0.0 {
                    AK
                } else {
                    CQ
                };
                let CT = CN + AG;
                let CU = CP + AG;
                let CW = if (if (if parameter_given[81] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && CV != 0.0 { 1.0 } else { 0.0 };
                if CW != 0.0 {
                    let CX = parameters[84] * X;
                    let CY = (3.021e22f64 * CX) * CX;
                    oCY = CY;
                } else {
                }
                let DA = R / CZ;
                let DE = if J != 0.0 {
                    let DB = U / parameters[148];
                    DB
                } else {
                    let DD = U / DC;
                    DD
                };
                let DF = if (if parameter_given[340] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let DG = if (if parameter_given[341] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                if DG != 0.0 {
                    let DI = if DH > AK { 1.0 } else { 0.0 };
                    oDI = DI;
                    let DJ = if DH < AK { 1.0 } else { 0.0 };
                    oDJ = DJ;
                } else {
                }
                let DK = if (if parameter_given[342] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                if DK != 0.0 {
                    let DL = BQ * Y;
                    oDL = DL;
                } else {
                }
                let DM = BQ * Y;
                if BK != 0.0 {
                    let DN = 1.17e1f64 / Z;
                    oDN = DN;
                } else {
                    let DO = Z * K;
                    oDO = DO;
                }
                let DQ = DP * Y;
                if BK != 0.0 {
                } else {
                    let DU = DT * BT;
                    oDU = DU;
                    let DW = DV + DU;
                    oDW = DW;
                }
                let DS = if CH > DR { 1.0 } else { 0.0 };
                let DZ = if DS != 0.0 {
                    let DX = CH.ln();
                    DX
                } else {
                    DY
                };
                let EA = (((CI * DZ).exp()) / CG) / CG;
                let EB = if DH == AE { 1.0 } else { 0.0 };
                let EE = if EB != 0.0 {
                    EC
                } else {
                    ED
                };
                let EH = if EB != 0.0 {
                    EF
                } else {
                    EG
                };
                let EI = (-EH) * CG;
                let EJ = EE * EA;
                let EK = EH * (-CG);
                let EN = if EL != 0.0 || EM != 0.0 { 1.0 } else { 0.0 };
                let EQ;
                if EN != 0.0 {
                    let EO = if EL == 0.0 { 1.0 } else { 0.0 };
                    oEO = EO;
                    let ER = if EM == 0.0 { 1.0 } else { 0.0 };
                    oER = ER;
                    EQ = ES;
                } else {
                    let EP = if (if parameter_given[86] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oEP = EP;
                    if EP != 0.0 {
                        let EV = if J != 0.0 {
                            let ET = (DP / DM) * 1e6f64;
                            ET
                        } else {
                            EU
                        };
                        oEV = EV;
                    } else {
                    }
                    let EW = if ES > AK { 1.0 } else { 0.0 };
                    oEW = EW;
                    let EY = if EW != 0.0 {
                        let EX = -ES;
                        EX
                    } else {
                        ES
                    };
                    oEY = EY;
                    let EZ = if CV == 0.0 { 1.0 } else { 0.0 };
                    oEZ = EZ;
                    let FA = if (if parameter_given[85] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oFA = FA;
                    EQ = EY;
                }
                let FB = if (if parameter_given[108] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                if FB != 0.0 {
                    let FD = if FC != 0.0 || (if parameter_given[106] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oFD = FD;
                } else {
                }
                let FE = if FC == 0.0 { 1.0 } else { 0.0 };
                let FG = if FF < AK { 1.0 } else { 0.0 };
                let FH = if FG != 0.0 {
                    AK
                } else {
                    FF
                };
                let FJ = if (if FI < AE { 1.0 } else { 0.0 }) != 0.0 || (if FI > BQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let FK = if FJ != 0.0 {
                    AE
                } else {
                    FI
                };
                let FL = FK * (AE + (DC / CZ));
                let FM = if FL > DR { 1.0 } else { 0.0 };
                let FP = if FM != 0.0 {
                    let FN = FL.ln();
                    FN
                } else {
                    FO
                };
                let FR = FQ * FP;
                let FU = if FS < FT { 1.0 } else { 0.0 };
                let FV = if FU != 0.0 {
                    FT
                } else {
                    FS
                };
                let FW = AE / FV;
                if BK != 0.0 {
                    let FY = S - FX;
                    oFY = FY;
                } else {
                    let GA = BL * FZ;
                    oGA = GA;
                    let GB = BQ * GA;
                    oGB = GB;
                    let GC = DH * parameters[54];
                    oGC = GC;
                    let GE = GD * K;
                    oGE = GE;
                    let GF = if GE != AK { 1.0 } else { 0.0 };
                    oGF = GF;
                    let GG = (FZ / A) - AE;
                    oGG = GG;
                }
                let GI = Y * BS;
                let GK = if GJ == 4e0f64 { 1.0 } else { 0.0 };
                let GL = if parameters[270] < AK { 1.0 } else { 0.0 };
                let GM = if S <= AK { 1.0 } else { 0.0 };
                let GN = if parameters[52] <= AK { 1.0 } else { 0.0 };
                let GO = if parameters[53] <= AK { 1.0 } else { 0.0 };
                let GP = if GD < AK { 1.0 } else { 0.0 };
                let GQ = if parameters[66] <= AK { 1.0 } else { 0.0 };
                let GR = if (S - FX) <= AK { 1.0 } else { 0.0 };
                let GS = if CZ <= AK { 1.0 } else { 0.0 };
                let GT = if parameters[204] < AK { 1.0 } else { 0.0 };
                let GW = if GU == GV { 1.0 } else { 0.0 };
                let GY = if (if GX <= AK { 1.0 } else { 0.0 }) != 0.0 || (if GX >= AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GZ = if CF < AK { 1.0 } else { 0.0 };
                let HA = if CG <= AK { 1.0 } else { 0.0 };
                let HC = if (if CA >= 4.4e0f64 { 1.0 } else { 0.0 }) != 0.0 || HB != 0.0 { 1.0 } else { 0.0 };
                if HB != 0.0 {
                    let HE = if S < HD { 1.0 } else { 0.0 };
                    oHE = HE;
                    let HG = if parameters[83] > HF { 1.0 } else { 0.0 };
                    oHG = HG;
                    let HH = if parameters[82] > HF { 1.0 } else { 0.0 };
                    oHH = HH;
                    let HI = if parameters[309] < AK { 1.0 } else { 0.0 };
                    oHI = HI;
                    let HJ = if parameters[310] < AK { 1.0 } else { 0.0 };
                    oHJ = HJ;
                    let HK = if parameters[162] < AK { 1.0 } else { 0.0 };
                    oHK = HK;
                    let HL = if parameters[163] < AK { 1.0 } else { 0.0 };
                    oHL = HL;
                    let HM = if parameters[315] < AK { 1.0 } else { 0.0 };
                    oHM = HM;
                    let HN = if parameters[316] < AK { 1.0 } else { 0.0 };
                    oHN = HN;
                    let HO = if parameters[317] < AK { 1.0 } else { 0.0 };
                    oHO = HO;
                    let HP = if parameters[318] < AK { 1.0 } else { 0.0 };
                    oHP = HP;
                    let HQ = if parameters[319] < AK { 1.0 } else { 0.0 };
                    oHQ = HQ;
                    let HR = if parameters[320] < AK { 1.0 } else { 0.0 };
                    oHR = HR;
                    let HS = if parameters[321] < AK { 1.0 } else { 0.0 };
                    oHS = HS;
                    let HT = if parameters[322] < AK { 1.0 } else { 0.0 };
                    oHT = HT;
                    let HU = if parameters[338] < AK { 1.0 } else { 0.0 };
                    oHU = HU;
                    let HV = if FQ < AK { 1.0 } else { 0.0 };
                    oHV = HV;
                    let HW = if parameters[365] < AK { 1.0 } else { 0.0 };
                    oHW = HW;
                    let HX = if CC < AK { 1.0 } else { 0.0 };
                    oHX = HX;
                    let HY = if parameters[336] < AK { 1.0 } else { 0.0 };
                    oHY = HY;
                    let HZ = if parameters[366] < AK { 1.0 } else { 0.0 };
                    oHZ = HZ;
                    let IA = if CI < AK { 1.0 } else { 0.0 };
                    oIA = IA;
                    let IB = if parameters[369] < AK { 1.0 } else { 0.0 };
                    oIB = IB;
                    let IC = if parameters[370] < AK { 1.0 } else { 0.0 };
                    oIC = IC;
                    let ID = if parameters[373] < AK { 1.0 } else { 0.0 };
                    oID = ID;
                    let IE = if parameters[374] < AK { 1.0 } else { 0.0 };
                    oIE = IE;
                    let IF = if parameters[377] < AK { 1.0 } else { 0.0 };
                    oIF = IF;
                    let IG = if parameters[381] < AK { 1.0 } else { 0.0 };
                    oIG = IG;
                    let IH = if parameters[382] <= AK { 1.0 } else { 0.0 };
                    oIH = IH;
                    let II = if parameters[287] < AK { 1.0 } else { 0.0 };
                    oII = II;
                    let IJ = if parameters[288] < AK { 1.0 } else { 0.0 };
                    oIJ = IJ;
                    let IK = if parameters[289] < AK { 1.0 } else { 0.0 };
                    oIK = IK;
                    let IL = if parameters[290] < AK { 1.0 } else { 0.0 };
                    oIL = IL;
                    let IM = if parameters[291] < AK { 1.0 } else { 0.0 };
                    oIM = IM;
                    let IN = if parameters[292] < AK { 1.0 } else { 0.0 };
                    oIN = IN;
                    let IO = if parameters[293] < AK { 1.0 } else { 0.0 };
                    oIO = IO;
                    let IP = if parameters[296] < AK { 1.0 } else { 0.0 };
                    oIP = IP;
                    let IQ = if parameters[298] < AK { 1.0 } else { 0.0 };
                    oIQ = IQ;
                    let IR = if parameters[299] < AK { 1.0 } else { 0.0 };
                    oIR = IR;
                    let IS = if parameters[300] < AK { 1.0 } else { 0.0 };
                    oIS = IS;
                    let IT = if parameters[301] < AK { 1.0 } else { 0.0 };
                    oIT = IT;
                    let IU = if parameters[324] < AK { 1.0 } else { 0.0 };
                    oIU = IU;
                    let IV = if parameters[325] < AK { 1.0 } else { 0.0 };
                    oIV = IV;
                    let IW = if parameters[326] < AK { 1.0 } else { 0.0 };
                    oIW = IW;
                    let IX = if parameters[327] < AK { 1.0 } else { 0.0 };
                    oIX = IX;
                    let IY = if parameters[328] < AK { 1.0 } else { 0.0 };
                    oIY = IY;
                    let IZ = if parameters[332] < AK { 1.0 } else { 0.0 };
                    oIZ = IZ;
                    let JA = if parameters[333] < AK { 1.0 } else { 0.0 };
                    oJA = JA;
                    let JB = if parameters[334] < AK { 1.0 } else { 0.0 };
                    oJB = JB;
                    let JD = if (if JC < AY { 1.0 } else { 0.0 }) != 0.0 || (if JC > 1.6e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oJD = JD;
                    let JF = if (if JE < 5e0f64 { 1.0 } else { 0.0 }) != 0.0 || (if JE > 2.5e1f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oJF = JF;
                    let JG = if BV < AK { 1.0 } else { 0.0 };
                    oJG = JG;
                    let JH = if parameters[150] < AK { 1.0 } else { 0.0 };
                    oJH = JH;
                    let JI = if parameters[151] < AK { 1.0 } else { 0.0 };
                    oJI = JI;
                    let JJ = if (parameters[152].abs()) < HD { 1.0 } else { 0.0 };
                    oJJ = JJ;
                    let JK = if parameters[974] < AK { 1.0 } else { 0.0 };
                    oJK = JK;
                    let JL = if parameters[156] < AK { 1.0 } else { 0.0 };
                    oJL = JL;
                    let JM = if parameters[157] < AK { 1.0 } else { 0.0 };
                    oJM = JM;
                    let JN = if (parameters[158].abs()) < HD { 1.0 } else { 0.0 };
                    oJN = JN;
                    let JO = if parameters[975] < AK { 1.0 } else { 0.0 };
                    oJO = JO;
                    let JP = if parameters[308] < AK { 1.0 } else { 0.0 };
                    oJP = JP;
                } else {
                }
                let JQ = if CA < 4.2e0f64 { 1.0 } else { 0.0 };
                let JS = if BK != 0.0 {
                    Y
                } else {
                    let JR = GD * K;
                    JR
                };
                let JT = if JS != AK { 1.0 } else { 0.0 };
                let JU = if CK == BQ { 1.0 } else { 0.0 };
                let JY = if J != 0.0 {
                    let JV = BQ * DH;
                    oJV = JV;
                    let JW = parameters[50] - DV;
                    oJW = JW;
                    let JX = (O * L) / Q;
                    JX
                } else {
                    S
                };
                let JZ = if GJ == AE { 1.0 } else { 0.0 };
                if JZ != 0.0 {
                } else {
                    let KA = if GJ == BQ { 1.0 } else { 0.0 };
                    oKA = KA;
                    if KA != 0.0 {
                    } else {
                        let KB = if GJ == GV { 1.0 } else { 0.0 };
                        oKB = KB;
                    }
                }
                let KC = if parameters[362] != AK { 1.0 } else { 0.0 };
                let KD = if KC != 0.0 || (if parameters[363] != AK { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let KE = -X;
                let KF = parameters[348] * X;
                if BZ != 0.0 {
                } else {
                    let KG = if BY == AE { 1.0 } else { 0.0 };
                    oKG = KG;
                }
                let KH = if GU == BQ { 1.0 } else { 0.0 };
                if KH != 0.0 {
                    let KJ = if KI > DT { 1.0 } else { 0.0 };
                    oKJ = KJ;
                    if KJ != 0.0 {
                    } else {
                        let KK = if KI < DT { 1.0 } else { 0.0 };
                        oKK = KK;
                    }
                } else {
                    if GW != 0.0 {
                        if BK != 0.0 {
                        } else {
                            let KL = Z * K;
                            oKL = KL;
                        }
                        let KM = parameters[57] * 7e-1f64;
                        oKM = KM;
                        let KN = parameters[56] * 1.9e-9f64;
                        oKN = KN;
                        let KO = if KI > DT { 1.0 } else { 0.0 };
                        oKO = KO;
                        if KO != 0.0 {
                        } else {
                            let KP = if KI < DT { 1.0 } else { 0.0 };
                            oKP = KP;
                        }
                    } else {
                    }
                }
                let KR = if KQ == GV { 1.0 } else { 0.0 };
                let KS = if CK != BQ { 1.0 } else { 0.0 };
                let KU = if KT == AK { 1.0 } else { 0.0 };
                let KX;
                let KY;
                let KZ;
                if KU != 0.0 {
                    KX = KV;
                    KY = AK;
                    KZ = AK;
                } else {
                    let KW = if KT == AE { 1.0 } else { 0.0 };
                    oKW = KW;
                    let LD;
                    let LE;
                    if KW != 0.0 {
                        LD = LF;
                        LE = AK;
                    } else {
                        let LC = if KT == GV { 1.0 } else { 0.0 };
                        oLC = LC;
                        let LH;
                        if LC != 0.0 {
                            LH = AK;
                        } else {
                            let LG = if KT == BQ { 1.0 } else { 0.0 };
                            oLG = LG;
                            let LJ = if LG != 0.0 {
                                LI
                            } else {
                                AK
                            };
                            LH = LJ;
                        }
                        LD = AK;
                        LE = LH;
                    }
                    KX = AK;
                    KY = LD;
                    KZ = LE;
                }
                let LB = if LA == AE { 1.0 } else { 0.0 };
                if LB != 0.0 {
                } else {
                    let LK = if LA == BQ { 1.0 } else { 0.0 };
                    oLK = LK;
                }
                let LL = if parameters[212] == AK { 1.0 } else { 0.0 };
                if LL != 0.0 {
                    let LM = if parameters[244] > AK { 1.0 } else { 0.0 };
                    oLM = LM;
                } else {
                    let LN = if parameters[282] <= AK { 1.0 } else { 0.0 };
                    oLN = LN;
                    let LO = parameters[211] * DT;
                    oLO = LO;
                    let LP = parameters[209] * 1.3806503e-23f64;
                    oLP = LP;
                }
                let LU;
                let LV;
                let LW;
                let LX;
                if KS != 0.0 {
                    LU = LQ;
                    LV = LR;
                    LW = AK;
                    LX = AK;
                } else {
                    LU = AK;
                    LV = AK;
                    LW = LS;
                    LX = LT;
                }
                let MB;
                let MC;
                if LY != 0.0 {
                    MB = LZ;
                    MC = AK;
                } else {
                    MB = AK;
                    MC = MA;
                }
                let MD = if KQ == AK { 1.0 } else { 0.0 };
                let ME = if KQ == BQ { 1.0 } else { 0.0 };
                let MF = if MD != 0.0 || ME != 0.0 { 1.0 } else { 0.0 };
                let MI;
                let MJ;
                if MF != 0.0 {
                    MI = MG;
                    MJ = AK;
                } else {
                    MI = AK;
                    MJ = MH;
                }
                let MK = if MD != 0.0 || (if KQ == AE { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let MM;
                let MN;
                if MK != 0.0 {
                    MM = ML;
                    MN = AK;
                } else {
                    let MP = if ME != 0.0 {
                        MO
                    } else {
                        AK
                    };
                    MM = AK;
                    MN = MP;
                }
                let MU;
                let MV;
                let MW;
                let MX;
                if GH != 0.0 {
                    MU = MQ;
                    MV = MR;
                    MW = AK;
                    MX = AK;
                } else {
                    MU = AK;
                    MV = AK;
                    MW = MS;
                    MX = MT;
                }
            [A, B, C, D, E, F, G, H, I, oAM, X, oAV, AZ, BF, Y, Z, AA, BK, oBR, BW, BX, BZ, CB, CD, oCE, CJ, CL, CM, CO, CR, CT, CU, CS, CW, oCY, DA, DE, DF, AB, DG, oDI, oDJ, DK, oDL, DM, oDN, oDO, DQ, BS, oDU, oDW, DS, EB, EE, EI, EJ, EK, EN, oEO, oER, oEP, oEV, oEW, oEZ, oFA, oEY, FB, oFD, FE, BJ, FG, FH, FJ, FM, FR, FU, FV, FW, oFY, oGA, oGB, oGC, oGE, oGF, oGG, GI, GK, GL, GM, GN, GO, GP, GQ, GR, GS, GT, GW, GY, GZ, HA, HC, oHE, oHG, oHH, oHI, oHJ, oHK, oHL, oHM, oHN, oHO, oHP, oHQ, oHR, oHS, oHT, oHU, oHV, oHW, oHX, oHY, oHZ, oIA, oIB, oIC, oID, oIE, oIF, oIG, oIH, oII, oIJ, oIK, oIL, oIM, oIN, oIO, oIP, oIQ, oIR, oIS, oIT, oIU, oIV, oIW, oIX, oIY, oIZ, oJA, oJB, oJD, oJF, oJG, oJH, oJI, oJJ, oJK, oJL, oJM, oJN, oJO, oJP, BU, EQ, JQ, JS, JT, JU, oJV, oJW, JZ, JY, oKA, oKB, KC, KD, KE, KF, oKG, KH, oKJ, oKK, oKL, oKM, oKN, oKO, oKP, BD, BG, KR, KS, KU, oKW, oLC, oLG, LB, oLK, LL, oLM, oLN, oLO, oLP, ME, MF, MK, KX, KY, KZ, LU, LV, LW, LX, MB, MC, MI, MJ, MM, MN, MU, MV, MW, MX]
        };
        let values = canonical_model_cache_intern(key, Arc::new(produced));
        self.canonical_install_model_values(values);
    }

    fn canonical_instance_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        if self.canonical_instance_valid {
            return;
        }
        let produced: [f64; 524] = {
            let parameters = &self.params.values;
            let parameter_given = &*self.param_given;
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = parameters[39];
                let B = staged[452];
                let D = parameters[2];
                let E = parameters[3];
                let G = parameters[1];
                let O = 0e0f64;
                let W = 2e0f64;
                let Z = parameters[24];
                let AE = parameters[25];
                let AG = parameters[26];
                let AI = parameters[27];
                let AV = 1e0f64;
                let AX = staged[460];
                let AY = 1e-6f64;
                let BW = parameters[973];
                let IV = 3.141592653589793e0f64;
                let IW = 5e-1f64;
                let IZ = 1e6f64;
                let JC = parameters[16];
                let JE = parameters[17];
                let JG = staged[462];
                let JH = parameters[366];
                let JN = staged[464];
                let JS = staged[472];
                let JT = staged[473];
                let JV = parameters[23];
                let JY = 1e-1f64;
                let JZ = 1.60219e-19f64;
                let KA = staged[23];
                let KB = parameters[148];
                let KE = parameters[147];
                let KN = staged[24];
                let KP = 3e0f64;
                let KX = parameters[34];
                let LA = 1e-38f64;
                let LE = staged[498];
                let LG = -8.749823353377374e1f64;
                let LK = 1e20f64;
                let LQ = -8.749823353377374e1f64;
                let LS = staged[55];
                let LT = staged[56];
                let LV = staged[504];
                let LX = staged[507];
                let MB = staged[63];
                let MD = parameters[64];
                let MF = staged[66];
                let MP = -8.749823353377374e1f64;
                let MS = parameters[364];
                let MW = -8.749823353377374e1f64;
                let MZ = staged[75];
                let NF = staged[518];
                let NG = staged[519];
                let NH = staged[520];
                let NL = 1e-8f64;
                let NN = 5.3e-1f64;
                let NP = staged[522];
                let NQ = -1.86e-2f64;
                let NS = parameters[84];
                let NT = parameters[85];
                let NU = parameters[88];
                let NW = staged[527];
                let NX = staged[80];
                let OA = staged[528];
                let OG = staged[529];
                let OH = staged[530];
                let ON = -8.749823353377374e1f64;
                let OZ = parameters[219];
                let PA = parameters[220];
                let PC = parameters[4];
                let PD = parameters[5];
                let PE = parameters[6];
                let PG = parameters[223];
                let PQ = -1e0f64;
                let QG = parameters[237];
                let QI = parameters[239];
                let QJ = parameters[241];
                let QO = staged[103];
                let QV = parameters[128];
                let QX = 1e-3f64;
                let RD = staged[104];
                let RF = 1e2f64;
                let RP = 3.720075976e-44f64;
                let RU = 1e18f64;
                let RV = 1e25f64;
                let RX = parameters[52];
                let SD = parameters[53];
                let SG = -8.749823353377374e1f64;
                let SU = parameters[407];
                let SY = parameters[37];
                let TB = parameters[38];
                let TC = 1e3f64;
                let TD = parameters[20];
                let TI = parameters[242];
                let TL = parameters[21];
                let TP = staged[572];
                let UF = staged[583];
                let UH = staged[584];
                let UJ = staged[585];
                let UL = staged[586];
                let VB = 4e0f64;
                let VG = 5e0f64;
                let VM = staged[232];
                let VS = staged[626];
                let VT = 1e-2f64;
                let WE = parameters[61];
                let WF = 5e-8f64;
                let WI = 1e-7f64;
                let WN = 1e21f64;
                let WS = 1e1f64;
                let XM = staged[0];
                let XT = -8.749823353377374e1f64;
                let XY = staged[130];
                let YE = 1e0f64;
                let YH = parameters[222];
                let YQ = 7.7348e-4f64;
                let ABN = staged[794];
                let ABO = staged[795];
                let ABP = staged[796];
                let ABV = 4e-4f64;
                let ACL = parameters[363];
                let ACT = parameters[29];
                let ADE = 1.0f64;
                let ADQ = 1e3f64;
                let ADY = parameters[28];
                let AEA = staged[349];
                let AEE = staged[816];
                let AEF = staged[817];
                let AEG = staged[818];
                let AEO = staged[827];
                let AER = staged[829];
                let AEV = parameters[348];
                let AEW = parameters[31];
                let AFJ = staged[841];
                let AFL = staged[843];
                let AFV = parameters[174];
                let AFZ = staged[848];
                let AGL = staged[856];
                let AGM = 1e10f64;
                let AGP = 0e0f64;
                let AGR = 0e0f64;
                let mut oJO = 0.0;
                let mut oKD = 0.0;
                let mut oKG = 0.0;
                let mut oKR = 0.0;
                let mut oKT = 0.0;
                let mut oKY = 0.0;
                let mut oLB = 0.0;
                let mut oLC = 0.0;
                let mut oLD = 0.0;
                let mut oLH = 0.0;
                let mut oLJ = 0.0;
                let mut oLL = 0.0;
                let mut oLM = 0.0;
                let mut oLN = 0.0;
                let mut oLO = 0.0;
                let mut oLR = 0.0;
                let mut oLW = 0.0;
                let mut oLY = 0.0;
                let mut oMK = 0.0;
                let mut oMM = 0.0;
                let mut oMN = 0.0;
                let mut oNO = 0.0;
                let mut oNR = 0.0;
                let mut oNV = 0.0;
                let mut oOC = 0.0;
                let mut oOD = 0.0;
                let mut oOI = 0.0;
                let mut oPH = 0.0;
                let mut oPR = 0.0;
                let mut oPS = 0.0;
                let mut oPX = 0.0;
                let mut oQD = 0.0;
                let mut oQF = 0.0;
                let mut oQH = 0.0;
                let mut oRI = 0.0;
                let mut oRW = 0.0;
                let mut oRY = 0.0;
                let mut oRZ = 0.0;
                let mut oSA = 0.0;
                let mut oSC = 0.0;
                let mut oSE = 0.0;
                let mut oSI = 0.0;
                let mut oSJ = 0.0;
                let mut oSK = 0.0;
                let mut oSL = 0.0;
                let mut oSM = 0.0;
                let mut oSN = 0.0;
                let mut oSZ = 0.0;
                let mut oTE = 0.0;
                let mut oTM = 0.0;
                let mut oTQ = 0.0;
                let mut oTU = 0.0;
                let mut oTY = 0.0;
                let mut oVC = 0.0;
                let mut oVE = 0.0;
                let mut oVF = 0.0;
                let mut oVI = 0.0;
                let mut oVJ = 0.0;
                let mut oVN = 0.0;
                let mut oVO = 0.0;
                let mut oVU = 0.0;
                let mut oVY = 0.0;
                let mut oWG = 0.0;
                let mut oWH = 0.0;
                let mut oWJ = 0.0;
                let mut oWK = 0.0;
                let mut oWL = 0.0;
                let mut oWM = 0.0;
                let mut oWO = 0.0;
                let mut oWP = 0.0;
                let mut oWQ = 0.0;
                let mut oWR = 0.0;
                let mut oWT = 0.0;
                let mut oWU = 0.0;
                let mut oWV = 0.0;
                let mut oWW = 0.0;
                let mut oWX = 0.0;
                let mut oWY = 0.0;
                let mut oWZ = 0.0;
                let mut oXA = 0.0;
                let mut oXB = 0.0;
                let mut oXC = 0.0;
                let mut oXD = 0.0;
                let mut oXE = 0.0;
                let mut oXF = 0.0;
                let mut oXG = 0.0;
                let mut oXH = 0.0;
                let mut oXI = 0.0;
                let mut oXN = 0.0;
                let mut oXO = 0.0;
                let mut oXQ = 0.0;
                let mut oXR = 0.0;
                let mut oXU = 0.0;
                let mut oXV = 0.0;
                let mut oXW = 0.0;
                let mut oXX = 0.0;
                let mut oXZ = 0.0;
                let mut oYA = 0.0;
                let mut oYB = 0.0;
                let mut oYC = 0.0;
                let mut oYD = 0.0;
                let mut oYF = 0.0;
                let mut oYG = 0.0;
                let mut oYI = 0.0;
                let mut oYJ = 0.0;
                let mut oYK = 0.0;
                let mut oYL = 0.0;
                let mut oYM = 0.0;
                let mut oYN = 0.0;
                let mut oYO = 0.0;
                let mut oYS = 0.0;
                let mut oYT = 0.0;
                let mut oYW = 0.0;
                let mut oYY = 0.0;
                let mut oYZ = 0.0;
                let mut oZB = 0.0;
                let mut oZC = 0.0;
                let mut oZF = 0.0;
                let mut oZG = 0.0;
                let mut oZH = 0.0;
                let mut oZI = 0.0;
                let mut oZJ = 0.0;
                let mut oZK = 0.0;
                let mut oZL = 0.0;
                let mut oZM = 0.0;
                let mut oZN = 0.0;
                let mut oZO = 0.0;
                let mut oZP = 0.0;
                let mut oZQ = 0.0;
                let mut oZR = 0.0;
                let mut oZT = 0.0;
                let mut oZU = 0.0;
                let mut oZW = 0.0;
                let mut oZX = 0.0;
                let mut oAAA = 0.0;
                let mut oAAB = 0.0;
                let mut oAAC = 0.0;
                let mut oAAD = 0.0;
                let mut oAAE = 0.0;
                let mut oAAF = 0.0;
                let mut oAAH = 0.0;
                let mut oAAI = 0.0;
                let mut oAAK = 0.0;
                let mut oAAL = 0.0;
                let mut oAAO = 0.0;
                let mut oAAP = 0.0;
                let mut oAAQ = 0.0;
                let mut oAAR = 0.0;
                let mut oAAS = 0.0;
                let mut oAAW = 0.0;
                let mut oABB = 0.0;
                let mut oABE = 0.0;
                let mut oABG = 0.0;
                let mut oABI = 0.0;
                let mut oABK = 0.0;
                let mut oABL = 0.0;
                let mut oABM = 0.0;
                let mut oABQ = 0.0;
                let mut oABR = 0.0;
                let mut oABT = 0.0;
                let mut oABU = 0.0;
                let mut oABW = 0.0;
                let mut oABX = 0.0;
                let mut oABY = 0.0;
                let mut oACC = 0.0;
                let mut oACG = 0.0;
                let mut oACH = 0.0;
                let mut oACI = 0.0;
                let mut oACJ = 0.0;
                let mut oACK = 0.0;
                let mut oACN = 0.0;
                let mut oACO = 0.0;
                let mut oACP = 0.0;
                let mut oACQ = 0.0;
                let mut oACR = 0.0;
                let mut oACS = 0.0;
                let mut oACW = 0.0;
                let mut oACX = 0.0;
                let mut oACY = 0.0;
                let mut oACZ = 0.0;
                let mut oADA = 0.0;
                let mut oADC = 0.0;
                let mut oADD = 0.0;
                let mut oADF = 0.0;
                let mut oADH = 0.0;
                let mut oADI = 0.0;
                let mut oADK = 0.0;
                let mut oADL = 0.0;
                let mut oADM = 0.0;
                let mut oADN = 0.0;
                let mut oADO = 0.0;
                let mut oADP = 0.0;
                let mut oADS = 0.0;
                let mut oADT = 0.0;
                let mut oADU = 0.0;
                let mut oADV = 0.0;
                let mut oAEH = 0.0;
                let mut oAEI = 0.0;
                let mut oAEK = 0.0;
                let mut oAEL = 0.0;
                let mut oAEM = 0.0;
                let mut oAEN = 0.0;
                let mut oAEP = 0.0;
                let mut oAEQ = 0.0;
                let mut oAES = 0.0;
                let mut oAET = 0.0;
                let mut oAEU = 0.0;
                let mut oAEX = 0.0;
                let mut oAEY = 0.0;
                let mut oAEZ = 0.0;
                let mut oAFA = 0.0;
                let mut oAFB = 0.0;
                let mut oAFC = 0.0;
                let mut oAFD = 0.0;
                let mut oAFE = 0.0;
                let mut oAFF = 0.0;
                let mut oAFG = 0.0;
                let mut oAFH = 0.0;
                let mut oAFI = 0.0;
                let mut oAFK = 0.0;
                let mut oAFM = 0.0;
                let mut oAFN = 0.0;
                let mut oAFO = 0.0;
                let mut oAFP = 0.0;
                let mut oAFQ = 0.0;
                let mut oAFR = 0.0;
                let mut oAFS = 0.0;
                let mut oAFU = 0.0;
                let mut oAFW = 0.0;
                let mut oAFX = 0.0;
                let mut oAFY = 0.0;
                let mut oAGB = 0.0;
                let mut oAGC = 0.0;
                let mut oAGD = 0.0;
                let mut oAGE = 0.0;
                let mut oAGG = 0.0;
                let mut oAGH = 0.0;
                let mut oAGI = 0.0;
                let mut oAGJ = 0.0;
                let mut oAGN = 0.0;
                let mut oAGO = 0.0;
                let C = parameters[18] * parameters[336];
                let F = D / E;
                let H = G.powf(parameters[180]);
                let I = F.powf(parameters[183]);
                let J = H * I;
                let K = parameters[177] + (((parameters[178] / H) + (parameters[181] / I)) + (parameters[184] / J));
                let L = ((parameters[179] / H) + (parameters[182] / I)) + (parameters[185] / J);
                let M = parameters[207] + L;
                let N = parameters[392] + L;
                let P = if N < O { 1.0 } else { 0.0 };
                let Q = if P != 0.0 {
                    O
                } else {
                    N
                };
                let R = G.powf(parameters[192]);
                let S = F.powf(parameters[195]);
                let T = R * S;
                let U = parameters[187] + (((parameters[190] / R) + (parameters[193] / S)) + (parameters[196] / T));
                let V = parameters[206] + (((parameters[191] / R) + (parameters[194] / S)) + (parameters[197] / T));
                let X = G - (W * K);
                let Y = if X <= O { 1.0 } else { 0.0 };
                let AA = F - (Z * parameters[290]);
                let AB = W - Z;
                let AC = AA - (AB * U);
                let AD = if AC <= O { 1.0 } else { 0.0 };
                let AF = AC / AE;
                let AH = AF + AG;
                let AJ = AF + AI;
                let AK = G - (W * M);
                let AL = if AK <= O { 1.0 } else { 0.0 };
                let AM = AA - (AB * V);
                let AN = if AM <= O { 1.0 } else { 0.0 };
                let AO = AM / AE;
                let AP = AO + AG;
                let AQ = AO + AI;
                let AR = AK - parameters[347];
                let AS = if AR <= O { 1.0 } else { 0.0 };
                let AT = AR + staged[2];
                let AU = if AT <= O { 1.0 } else { 0.0 };
                let AW = AV + ((parameters[204] / X).powf(parameters[205]));
                let BF;
                let BG;
                let BH;
                if AX != 0.0 {
                    let AZ = AY / X;
                    let BA = AY / AC;
                    let BB = 1e-12f64 / (X * AC);
                    BF = AZ;
                    BG = BA;
                    BH = BB;
                } else {
                    let BC = AV / X;
                    let BD = AV / AC;
                    let BE = AV / (X * AC);
                    BF = BC;
                    BG = BD;
                    BH = BE;
                }
                let BI = ((parameters[81] + (parameters[461] * BF)) + (parameters[642] * BG)) + (parameters[823] * BH);
                let BJ = ((parameters[80] + (parameters[462] * BF)) + (parameters[643] * BG)) + (parameters[824] * BH);
                let BK = ((parameters[82] + (parameters[463] * BF)) + (parameters[644] * BG)) + (parameters[826] * BH);
                let BL = ((parameters[83] + (parameters[464] * BF)) + (parameters[645] * BG)) + (parameters[825] * BH);
                let BM = ((parameters[107] + (parameters[465] * BF)) + (parameters[646] * BG)) + (parameters[827] * BH);
                let BN = ((parameters[108] + (parameters[466] * BF)) + (parameters[647] * BG)) + (parameters[828] * BH);
                let BO = ((parameters[89] + (parameters[467] * BF)) + (parameters[648] * BG)) + (parameters[829] * BH);
                let BP = ((parameters[93] + (parameters[470] * BF)) + (parameters[651] * BG)) + (parameters[832] * BH);
                let BQ = ((parameters[287] + (parameters[468] * BF)) + (parameters[649] * BG)) + (parameters[830] * BH);
                let BR = ((parameters[288] + (parameters[469] * BF)) + (parameters[650] * BG)) + (parameters[831] * BH);
                let BS = ((parameters[94] + (parameters[471] * BF)) + (parameters[652] * BG)) + (parameters[833] * BH);
                let BT = ((parameters[95] + (parameters[472] * BF)) + (parameters[653] * BG)) + (parameters[834] * BH);
                let BU = ((parameters[358] + (parameters[473] * BF)) + (parameters[654] * BG)) + (parameters[835] * BH);
                let BV = ((parameters[96] + (parameters[474] * BF)) + (parameters[655] * BG)) + (parameters[836] * BH);
                let BX = ((BW + (parameters[976] * BF)) + (parameters[979] * BG)) + (parameters[982] * BH);
                let BY = ((parameters[97] + (parameters[475] * BF)) + (parameters[656] * BG)) + (parameters[837] * BH);
                let BZ = ((parameters[98] + (parameters[476] * BF)) + (parameters[657] * BG)) + (parameters[838] * BH);
                let CA = ((parameters[99] + (parameters[477] * BF)) + (parameters[658] * BG)) + (parameters[839] * BH);
                let CB = ((parameters[100] + (parameters[478] * BF)) + (parameters[659] * BG)) + (parameters[840] * BH);
                let CC = ((parameters[101] + (parameters[479] * BF)) + (parameters[660] * BG)) + (parameters[841] * BH);
                let CD = ((parameters[102] + (parameters[480] * BF)) + (parameters[661] * BG)) + (parameters[842] * BH);
                let CE = ((parameters[103] + (parameters[481] * BF)) + (parameters[662] * BG)) + (parameters[843] * BH);
                let CF = ((parameters[115] + (parameters[482] * BF)) + (parameters[663] * BG)) + (parameters[844] * BH);
                let CG = ((parameters[109] + (parameters[484] * BF)) + (parameters[665] * BG)) + (parameters[846] * BH);
                let CH = ((parameters[111] + (parameters[485] * BF)) + (parameters[666] * BG)) + (parameters[847] * BH);
                let CI = ((parameters[113] + (parameters[486] * BF)) + (parameters[667] * BG)) + (parameters[848] * BH);
                let CJ = ((parameters[73] + (parameters[491] * BF)) + (parameters[672] * BG)) + (parameters[853] * BH);
                let CK = ((parameters[75] + (parameters[492] * BF)) + (parameters[673] * BG)) + (parameters[854] * BH);
                let CL = ((parameters[76] + (parameters[493] * BF)) + (parameters[674] * BG)) + (parameters[855] * BH);
                let CM = ((parameters[198] + (parameters[494] * BF)) + (parameters[675] * BG)) + (parameters[856] * BH);
                let CN = ((parameters[199] + (parameters[495] * BF)) + (parameters[676] * BG)) + (parameters[857] * BH);
                let CO = ((parameters[79] + (parameters[496] * BF)) + (parameters[677] * BG)) + (parameters[858] * BH);
                let CP = ((parameters[289] + (parameters[497] * BF)) + (parameters[678] * BG)) + (parameters[859] * BH);
                let CQ = ((parameters[77] + (parameters[498] * BF)) + (parameters[679] * BG)) + (parameters[860] * BH);
                let CR = ((parameters[78] + (parameters[499] * BF)) + (parameters[680] * BG)) + (parameters[861] * BH);
                let CS = ((parameters[129] + (parameters[500] * BF)) + (parameters[681] * BG)) + (parameters[862] * BH);
                let CT = ((parameters[130] + (parameters[501] * BF)) + (parameters[682] * BG)) + (parameters[863] * BH);
                let CU = ((parameters[131] + (parameters[502] * BF)) + (parameters[683] * BG)) + (parameters[864] * BH);
                let CV = ((parameters[135] + (parameters[503] * BF)) + (parameters[684] * BG)) + (parameters[865] * BH);
                let CW = ((parameters[134] + (parameters[504] * BF)) + (parameters[685] * BG)) + (parameters[866] * BH);
                let CX = ((parameters[186] + (parameters[505] * BF)) + (parameters[686] * BG)) + (parameters[867] * BH);
                let CY = ((parameters[72] + (parameters[506] * BF)) + (parameters[687] * BG)) + (parameters[868] * BH);
                let CZ = ((parameters[188] + (parameters[507] * BF)) + (parameters[688] * BG)) + (parameters[869] * BH);
                let DA = ((parameters[189] + (parameters[508] * BF)) + (parameters[689] * BG)) + (parameters[870] * BH);
                let DB = ((parameters[122] + (parameters[509] * BF)) + (parameters[690] * BG)) + (parameters[871] * BH);
                let DC = ((parameters[137] + (parameters[510] * BF)) + (parameters[691] * BG)) + (parameters[872] * BH);
                let DD = ((parameters[138] + (parameters[511] * BF)) + (parameters[692] * BG)) + (parameters[873] * BH);
                let DE = ((parameters[139] + (parameters[512] * BF)) + (parameters[693] * BG)) + (parameters[874] * BH);
                let DF = ((parameters[140] + (parameters[513] * BF)) + (parameters[694] * BG)) + (parameters[875] * BH);
                let DG = ((parameters[105] + (parameters[514] * BF)) + (parameters[695] * BG)) + (parameters[876] * BH);
                let DH = ((parameters[71] + (parameters[515] * BF)) + (parameters[696] * BG)) + (parameters[877] * BH);
                let DI = ((parameters[68] + (parameters[516] * BF)) + (parameters[697] * BG)) + (parameters[878] * BH);
                let DJ = ((parameters[69] + (parameters[517] * BF)) + (parameters[698] * BG)) + (parameters[879] * BH);
                let DK = ((parameters[70] + (parameters[518] * BF)) + (parameters[699] * BG)) + (parameters[880] * BH);
                let DL = ((parameters[141] + (parameters[519] * BF)) + (parameters[700] * BG)) + (parameters[881] * BH);
                let DM = ((parameters[142] + (parameters[520] * BF)) + (parameters[701] * BG)) + (parameters[882] * BH);
                let DN = ((parameters[143] + (parameters[521] * BF)) + (parameters[702] * BG)) + (parameters[883] * BH);
                let DO = ((parameters[144] + (parameters[522] * BF)) + (parameters[703] * BG)) + (parameters[884] * BH);
                let DP = ((parameters[104] + (parameters[523] * BF)) + (parameters[704] * BG)) + (parameters[885] * BH);
                let DQ = ((parameters[145] + (parameters[524] * BF)) + (parameters[705] * BG)) + (parameters[886] * BH);
                let DR = ((parameters[127] + (parameters[525] * BF)) + (parameters[706] * BG)) + (parameters[887] * BH);
                let DS = ((parameters[208] + (parameters[526] * BF)) + (parameters[707] * BG)) + (parameters[888] * BH);
                let DT = ((parameters[301] + (parameters[527] * BF)) + (parameters[708] * BG)) + (parameters[889] * BH);
                let DU = ((parameters[302] + (parameters[530] * BF)) + (parameters[711] * BG)) + (parameters[892] * BH);
                let DV = ((parameters[303] + (parameters[529] * BF)) + (parameters[710] * BG)) + (parameters[891] * BH);
                let DW = ((parameters[304] + (parameters[532] * BF)) + (parameters[713] * BG)) + (parameters[894] * BH);
                let DX = ((parameters[305] + (parameters[528] * BF)) + (parameters[709] * BG)) + (parameters[890] * BH);
                let DY = ((parameters[306] + (parameters[531] * BF)) + (parameters[712] * BG)) + (parameters[893] * BH);
                let DZ = ((parameters[291] + (parameters[533] * BF)) + (parameters[714] * BG)) + (parameters[895] * BH);
                let EA = ((parameters[292] + (parameters[534] * BF)) + (parameters[715] * BG)) + (parameters[896] * BH);
                let EB = ((parameters[293] + (parameters[535] * BF)) + (parameters[716] * BG)) + (parameters[897] * BH);
                let EC = ((parameters[294] + (parameters[536] * BF)) + (parameters[717] * BG)) + (parameters[898] * BH);
                let ED = ((parameters[296] + (parameters[537] * BF)) + (parameters[718] * BG)) + (parameters[899] * BH);
                let EE = ((parameters[308] + (parameters[538] * BF)) + (parameters[719] * BG)) + (parameters[900] * BH);
                let EF = ((parameters[297] + (parameters[539] * BF)) + (parameters[720] * BG)) + (parameters[901] * BH);
                let EG = ((parameters[298] + (parameters[540] * BF)) + (parameters[721] * BG)) + (parameters[902] * BH);
                let EH = ((parameters[299] + (parameters[541] * BF)) + (parameters[722] * BG)) + (parameters[903] * BH);
                let EI = ((parameters[300] + (parameters[542] * BF)) + (parameters[723] * BG)) + (parameters[904] * BH);
                let EJ = ((parameters[150] + (parameters[543] * BF)) + (parameters[724] * BG)) + (parameters[905] * BH);
                let EK = ((parameters[151] + (parameters[544] * BF)) + (parameters[725] * BG)) + (parameters[906] * BH);
                let EL = ((parameters[152] + (parameters[545] * BF)) + (parameters[726] * BG)) + (parameters[907] * BH);
                let EM = ((parameters[974] + (parameters[977] * BF)) + (parameters[980] * BG)) + (parameters[983] * BH);
                let EN = ((parameters[153] + (parameters[546] * BF)) + (parameters[727] * BG)) + (parameters[908] * BH);
                let EO = ((parameters[154] + (parameters[547] * BF)) + (parameters[728] * BG)) + (parameters[909] * BH);
                let EP = ((parameters[155] + (parameters[548] * BF)) + (parameters[729] * BG)) + (parameters[910] * BH);
                let EQ = ((parameters[156] + (parameters[549] * BF)) + (parameters[730] * BG)) + (parameters[911] * BH);
                let ER = ((parameters[157] + (parameters[550] * BF)) + (parameters[731] * BG)) + (parameters[912] * BH);
                let ES = ((parameters[158] + (parameters[551] * BF)) + (parameters[732] * BG)) + (parameters[913] * BH);
                let ET = ((parameters[975] + (parameters[978] * BF)) + (parameters[981] * BG)) + (parameters[984] * BH);
                let EU = ((parameters[159] + (parameters[552] * BF)) + (parameters[733] * BG)) + (parameters[914] * BH);
                let EV = ((parameters[160] + (parameters[553] * BF)) + (parameters[734] * BG)) + (parameters[915] * BH);
                let EW = ((parameters[161] + (parameters[554] * BF)) + (parameters[735] * BG)) + (parameters[916] * BH);
                let EX = ((parameters[309] + (parameters[555] * BF)) + (parameters[736] * BG)) + (parameters[917] * BH);
                let EY = ((parameters[310] + (parameters[556] * BF)) + (parameters[737] * BG)) + (parameters[918] * BH);
                let EZ = ((parameters[162] + (parameters[557] * BF)) + (parameters[738] * BG)) + (parameters[919] * BH);
                let FA = ((parameters[163] + (parameters[558] * BF)) + (parameters[739] * BG)) + (parameters[920] * BH);
                let FB = ((parameters[311] + (parameters[559] * BF)) + (parameters[740] * BG)) + (parameters[921] * BH);
                let FC = ((parameters[312] + (parameters[560] * BF)) + (parameters[741] * BG)) + (parameters[922] * BH);
                let FD = ((parameters[313] + (parameters[561] * BF)) + (parameters[742] * BG)) + (parameters[923] * BH);
                let FE = ((parameters[314] + (parameters[562] * BF)) + (parameters[743] * BG)) + (parameters[924] * BH);
                let FF = ((parameters[315] + (parameters[563] * BF)) + (parameters[744] * BG)) + (parameters[925] * BH);
                let FG = ((parameters[316] + (parameters[564] * BF)) + (parameters[745] * BG)) + (parameters[926] * BH);
                let FH = ((parameters[317] + (parameters[565] * BF)) + (parameters[746] * BG)) + (parameters[927] * BH);
                let FI = ((parameters[318] + (parameters[566] * BF)) + (parameters[747] * BG)) + (parameters[928] * BH);
                let FJ = ((parameters[319] + (parameters[567] * BF)) + (parameters[748] * BG)) + (parameters[929] * BH);
                let FK = ((parameters[321] + (parameters[569] * BF)) + (parameters[750] * BG)) + (parameters[931] * BH);
                let FL = ((parameters[320] + (parameters[568] * BF)) + (parameters[749] * BG)) + (parameters[930] * BH);
                let FM = ((parameters[322] + (parameters[570] * BF)) + (parameters[751] * BG)) + (parameters[932] * BH);
                let FN = ((parameters[324] + (parameters[571] * BF)) + (parameters[752] * BG)) + (parameters[933] * BH);
                let FO = ((parameters[325] + (parameters[572] * BF)) + (parameters[753] * BG)) + (parameters[934] * BH);
                let FP = ((parameters[326] + (parameters[573] * BF)) + (parameters[754] * BG)) + (parameters[935] * BH);
                let FQ = ((parameters[327] + (parameters[574] * BF)) + (parameters[755] * BG)) + (parameters[936] * BH);
                let FR = ((parameters[328] + (parameters[575] * BF)) + (parameters[756] * BG)) + (parameters[937] * BH);
                let FS = ((parameters[329] + (parameters[576] * BF)) + (parameters[757] * BG)) + (parameters[938] * BH);
                let FT = ((parameters[331] + (parameters[577] * BF)) + (parameters[758] * BG)) + (parameters[939] * BH);
                let FU = ((parameters[332] + (parameters[578] * BF)) + (parameters[759] * BG)) + (parameters[940] * BH);
                let FV = ((parameters[333] + (parameters[579] * BF)) + (parameters[760] * BG)) + (parameters[941] * BH);
                let FW = ((parameters[334] + (parameters[580] * BF)) + (parameters[761] * BG)) + (parameters[942] * BH);
                let FX = ((parameters[149] + (parameters[422] * BF)) + (parameters[603] * BG)) + (parameters[784] * BH);
                let FY = ((parameters[371] + (parameters[423] * BF)) + (parameters[604] * BG)) + (parameters[785] * BH);
                let FZ = ((parameters[375] + (parameters[425] * BF)) + (parameters[606] * BG)) + (parameters[787] * BH);
                let GA = ((parameters[372] + (parameters[424] * BF)) + (parameters[605] * BG)) + (parameters[786] * BH);
                let GB = ((parameters[376] + (parameters[426] * BF)) + (parameters[607] * BG)) + (parameters[788] * BH);
                let GC = ((parameters[339] + (parameters[433] * BF)) + (parameters[614] * BG)) + (parameters[795] * BH);
                let GD = ((parameters[345] + (parameters[443] * BF)) + (parameters[624] * BG)) + (parameters[805] * BH);
                let GE = ((parameters[346] + (parameters[444] * BF)) + (parameters[625] * BG)) + (parameters[806] * BH);
                let GF = ((parameters[164] + (parameters[445] * BF)) + (parameters[626] * BG)) + (parameters[807] * BH);
                let GG = ((parameters[165] + (parameters[446] * BF)) + (parameters[627] * BG)) + (parameters[808] * BH);
                let GH = ((parameters[166] + (parameters[447] * BF)) + (parameters[628] * BG)) + (parameters[809] * BH);
                let GI = ((parameters[167] + (parameters[448] * BF)) + (parameters[629] * BG)) + (parameters[810] * BH);
                let GJ = ((parameters[168] + (parameters[449] * BF)) + (parameters[630] * BG)) + (parameters[811] * BH);
                let GK = ((parameters[169] + (parameters[450] * BF)) + (parameters[631] * BG)) + (parameters[812] * BH);
                let GL = ((parameters[170] + (parameters[451] * BF)) + (parameters[632] * BG)) + (parameters[813] * BH);
                let GM = ((parameters[201] + (parameters[431] * BF)) + (parameters[612] * BG)) + (parameters[793] * BH);
                let GN = ((parameters[200] + (parameters[430] * BF)) + (parameters[611] * BG)) + (parameters[792] * BH);
                let GO = ((parameters[202] + (parameters[432] * BF)) + (parameters[613] * BG)) + (parameters[794] * BH);
                let GP = ((parameters[117] + (parameters[434] * BF)) + (parameters[615] * BG)) + (parameters[796] * BH);
                let GQ = ((parameters[120] + (parameters[487] * BF)) + (parameters[668] * BG)) + (parameters[849] * BH);
                let GR = ((parameters[121] + (parameters[488] * BF)) + (parameters[669] * BG)) + (parameters[850] * BH);
                let GS = ((parameters[116] + (parameters[483] * BF)) + (parameters[664] * BG)) + (parameters[845] * BH);
                let GT = ((parameters[118] + (parameters[490] * BF)) + (parameters[671] * BG)) + (parameters[852] * BH);
                let GU = ((parameters[119] + (parameters[489] * BF)) + (parameters[670] * BG)) + (parameters[851] * BH);
                let GV = ((parameters[90] + (parameters[435] * BF)) + (parameters[616] * BG)) + (parameters[797] * BH);
                let GW = ((parameters[92] + (parameters[437] * BF)) + (parameters[618] * BG)) + (parameters[799] * BH);
                let GX = ((parameters[91] + (parameters[436] * BF)) + (parameters[617] * BG)) + (parameters[798] * BH);
                let GY = ((parameters[110] + (parameters[438] * BF)) + (parameters[619] * BG)) + (parameters[800] * BH);
                let GZ = ((parameters[112] + (parameters[439] * BF)) + (parameters[620] * BG)) + (parameters[801] * BH);
                let HA = ((parameters[114] + (parameters[440] * BF)) + (parameters[621] * BG)) + (parameters[802] * BH);
                let HB = ((parameters[74] + (parameters[441] * BF)) + (parameters[622] * BG)) + (parameters[803] * BH);
                let HC = ((parameters[136] + (parameters[442] * BF)) + (parameters[623] * BG)) + (parameters[804] * BH);
                let HD = ((parameters[389] + (parameters[458] * BF)) + (parameters[639] * BG)) + (parameters[820] * BH);
                let HE = ((parameters[383] + (parameters[452] * BF)) + (parameters[633] * BG)) + (parameters[814] * BH);
                let HF = ((parameters[384] + (parameters[453] * BF)) + (parameters[634] * BG)) + (parameters[815] * BH);
                let HG = ((parameters[385] + (parameters[454] * BF)) + (parameters[635] * BG)) + (parameters[816] * BH);
                let HH = ((parameters[386] + (parameters[455] * BF)) + (parameters[636] * BG)) + (parameters[817] * BH);
                let HI = ((parameters[387] + (parameters[456] * BF)) + (parameters[637] * BG)) + (parameters[818] * BH);
                let HJ = ((parameters[388] + (parameters[457] * BF)) + (parameters[638] * BG)) + (parameters[819] * BH);
                let HK = ((parameters[390] + (parameters[459] * BF)) + (parameters[640] * BG)) + (parameters[821] * BH);
                let HL = ((parameters[391] + (parameters[460] * BF)) + (parameters[641] * BG)) + (parameters[822] * BH);
                let HM = ((parameters[404] + (parameters[588] * BF)) + (parameters[769] * BG)) + (parameters[950] * BH);
                let HN = ((parameters[405] + (parameters[589] * BF)) + (parameters[770] * BG)) + (parameters[951] * BH);
                let HO = ((parameters[395] + (parameters[590] * BF)) + (parameters[771] * BG)) + (parameters[952] * BH);
                let HP = ((parameters[412] + (parameters[591] * BF)) + (parameters[772] * BG)) + (parameters[953] * BH);
                let HQ = ((parameters[413] + (parameters[592] * BF)) + (parameters[773] * BG)) + (parameters[954] * BH);
                let HR = ((parameters[396] + (parameters[593] * BF)) + (parameters[774] * BG)) + (parameters[955] * BH);
                let HS = ((parameters[397] + (parameters[594] * BF)) + (parameters[775] * BG)) + (parameters[956] * BH);
                let HT = ((parameters[398] + (parameters[595] * BF)) + (parameters[776] * BG)) + (parameters[957] * BH);
                let HU = ((parameters[399] + (parameters[596] * BF)) + (parameters[777] * BG)) + (parameters[958] * BH);
                let HV = ((parameters[400] + (parameters[597] * BF)) + (parameters[778] * BG)) + (parameters[959] * BH);
                let HW = ((parameters[401] + (parameters[598] * BF)) + (parameters[779] * BG)) + (parameters[960] * BH);
                let HX = ((parameters[402] + (parameters[599] * BF)) + (parameters[780] * BG)) + (parameters[961] * BH);
                let HY = ((parameters[403] + (parameters[600] * BF)) + (parameters[781] * BG)) + (parameters[962] * BH);
                let HZ = ((parameters[393] + (parameters[601] * BF)) + (parameters[782] * BG)) + (parameters[963] * BH);
                let IA = ((parameters[394] + (parameters[602] * BF)) + (parameters[783] * BG)) + (parameters[964] * BH);
                let IB = ((parameters[340] + (parameters[581] * BF)) + (parameters[762] * BG)) + (parameters[943] * BH);
                let IC = ((parameters[341] + (parameters[582] * BF)) + (parameters[763] * BG)) + (parameters[944] * BH);
                let ID = ((parameters[357] + (parameters[583] * BF)) + (parameters[764] * BG)) + (parameters[945] * BH);
                let IE = (((parameters[353] + (parameters[584] * BF)) + (parameters[765] * BG)) + (parameters[946] * BH)) * ((BI / 2e16f64).powf(-2.5e-1f64));
                let IF = ((parameters[354] + (parameters[585] * BF)) + (parameters[766] * BG)) + (parameters[947] * BH);
                let IG = ((parameters[355] + (parameters[586] * BF)) + (parameters[767] * BG)) + (parameters[948] * BH);
                let IH = ((parameters[356] + (parameters[587] * BF)) + (parameters[768] * BG)) + (parameters[949] * BH);
                let II = ((parameters[245] + (parameters[246] * BF)) + (parameters[247] * BG)) + (parameters[248] * BH);
                let IJ = ((parameters[249] + (parameters[250] * BF)) + (parameters[251] * BG)) + (parameters[252] * BH);
                let IK = ((parameters[253] + (parameters[254] * BF)) + (parameters[255] * BG)) + (parameters[256] * BH);
                let IL = ((parameters[257] + (parameters[258] * BF)) + (parameters[259] * BG)) + (parameters[260] * BH);
                let IM = ((parameters[261] + (parameters[262] * BF)) + (parameters[263] * BG)) + (parameters[264] * BH);
                let IN = ((parameters[414] + (parameters[415] * BF)) + (parameters[416] * BG)) + (parameters[417] * BH);
                let IO = ((parameters[418] + (parameters[419] * BF)) + (parameters[420] * BG)) + (parameters[421] * BH);
                let IP = ((parameters[272] + (parameters[273] * BF)) + (parameters[276] * BG)) + (parameters[279] * BH);
                let IQ = ((parameters[269] + (parameters[274] * BF)) + (parameters[277] * BG)) + (parameters[280] * BH);
                let IR = ((parameters[271] + (parameters[275] * BF)) + (parameters[278] * BG)) + (parameters[281] * BH);
                let IS = ((parameters[378] + (parameters[427] * BF)) + (parameters[608] * BG)) + (parameters[789] * BH);
                let IT = ((parameters[379] + (parameters[428] * BF)) + (parameters[609] * BG)) + (parameters[790] * BH);
                let IU = ((parameters[380] + (parameters[429] * BF)) + (parameters[610] * BG)) + (parameters[791] * BH);
                let IX = IW + (((((parameters[265] + (parameters[266] * BF)) + (parameters[267] * BG)) + (parameters[268] * BH)).atan()) / IV);
                let IY = IW + ((IN.atan()) / IV);
                let JA = (AC * IZ).powf(CX);
                let JB = E * (AC + parameters[365]);
                let JD = (JC / JB) * AE;
                let JF = (JE * JB) / AE;
                let JJ = if JG != 0.0 {
                    O
                } else {
                    let JI = (((((parameters[19] * parameters[335]) * JH) / (staged[3] + (JH * X))) * AC) / AE) / E;
                    JI
                };
                let JK = if CF > AV { 1.0 } else { 0.0 };
                let JM = if JK != 0.0 {
                    let JL = CF / 1e4f64;
                    JL
                } else {
                    CF
                };
                if JN != 0.0 {
                    let JO = JA * E;
                    oJO = JO;
                } else {
                }
                let JP = staged[20] * AP;
                let JQ = staged[21] * AQ;
                let JR = (staged[22] * AK) * E;
                let JU = if JS != 0.0 {
                    JT
                } else {
                    BI
                };
                let JW = if JV == W { 1.0 } else { 0.0 };
                let JX;
                if JW != 0.0 {
                    let KH;
                    if A != 0.0 {
                        let KC = ((((parameters[47] - JY) / JZ) * 2e-6f64) * KA) / (KB * KB);
                        let KD = if JU > KC { 1.0 } else { 0.0 };
                        oKD = KD;
                        let KI = if KD != 0.0 {
                            KC
                        } else {
                            JU
                        };
                        KH = KI;
                    } else {
                        let KF = (1.2732572291675768e13f64 * KA) / (KE * KE);
                        let KG = if JU > KF { 1.0 } else { 0.0 };
                        oKG = KG;
                        let KJ = if KG != 0.0 {
                            KF
                        } else {
                            JU
                        };
                        KH = KJ;
                    }
                    JX = KH;
                } else {
                    JX = JU;
                }
                let KM = if A != 0.0 {
                    let KK = (((JZ * JX) * (AV + (BW / G))) * IZ) * KB;
                    KK
                } else {
                    let KL = (((JZ * JX) * (AV + (BW / G))) * IZ) * KE;
                    KL
                };
                let KO = (8e-1f64 - ((IW * KM) / KN)) + HO;
                let KQ = if JV == KP { 1.0 } else { 0.0 };
                let KS;
                if KQ != 0.0 {
                    let KR = if KO > IA { 1.0 } else { 0.0 };
                    oKR = KR;
                    let KU;
                    if KR != 0.0 {
                        KU = W;
                    } else {
                        let KT = if KO < HZ { 1.0 } else { 0.0 };
                        oKT = KT;
                        let KV = if KT != 0.0 {
                            O
                        } else {
                            AV
                        };
                        KU = KV;
                    }
                    KS = KU;
                } else {
                    KS = JV;
                }
                let KW = if BJ > O { 1.0 } else { 0.0 };
                if KW != 0.0 {
                    let KY = -KX;
                    oKY = KY;
                    let KZ = JX / BJ;
                    let LB = if KZ > LA { 1.0 } else { 0.0 };
                    oLB = LB;
                    let LH = if LB != 0.0 {
                        let LF = KZ.ln();
                        LF
                    } else {
                        LG
                    };
                    oLH = LH;
                } else {
                    let LC = -KX;
                    oLC = LC;
                    let LD = (-JX) * BJ;
                    oLD = LD;
                }
                if LE != 0.0 {
                    if KW != 0.0 {
                        let LJ = -KX;
                        oLJ = LJ;
                        let LL = LK * BJ;
                        oLL = LL;
                    } else {
                        let LM = if BJ < O { 1.0 } else { 0.0 };
                        oLM = LM;
                        if LM != 0.0 {
                            let LN = -KX;
                            oLN = LN;
                            let LO = if (-1e20f64 / BJ) > LA { 1.0 } else { 0.0 };
                            oLO = LO;
                            let LR = if LO != 0.0 {
                                let LP = (-1e20f64 / BJ).ln();
                                LP
                            } else {
                                LQ
                            };
                            oLR = LR;
                        } else {
                        }
                    }
                } else {
                }
                let LI = BJ.abs();
                let LU = (LS * (LI.sqrt())) / LT;
                if LV != 0.0 {
                    let LW = if (if KW != 0.0 && staged[57] != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if BJ < O { 1.0 } else { 0.0 }) != 0.0 && staged[58] != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oLW = LW;
                } else {
                }
                if LX != 0.0 {
                    let LY = (JZ * LI) * IZ;
                    oLY = LY;
                } else {
                }
                let LZ = JZ * JX;
                let MA = LZ * IZ;
                let MC = (MB / MA).sqrt();
                let MH = if B != 0.0 {
                    let ME = ((staged[65] * FX) * MD).sqrt();
                    ME
                } else {
                    let MG = (((KA * FX) * MF) / staged[67]).sqrt();
                    MG
                };
                let MI = LK * JX;
                let MJ = ((staged[69] * JX) * IZ) / W;
                if B != 0.0 {
                    let MK = if BK > O { 1.0 } else { 0.0 };
                    oMK = MK;
                    let MN;
                    if MK != 0.0 {
                        let ML = BK / LK;
                        let MM = if ML > LA { 1.0 } else { 0.0 };
                        oMM = MM;
                        let MQ = if MM != 0.0 {
                            let MO = ML.ln();
                            MO
                        } else {
                            MP
                        };
                        let MR = staged[71] * MQ;
                        MN = MR;
                    } else {
                        MN = O;
                    }
                    oMN = MN;
                } else {
                }
                let MT = parameters[368] / (MS * HL);
                let MU = if MT > LA { 1.0 } else { 0.0 };
                let MX = if MU != 0.0 {
                    let MV = MT.ln();
                    MV
                } else {
                    MW
                };
                let MY = (((((parameters[367] * MX).exp()) / MS) / MS) / HL) / HL;
                let NA = ((MZ * AJ) * Q) * MY;
                let NB = ((MZ * AH) * Q) * MY;
                let NC = staged[76] * HL;
                let ND = parameters[30] / E;
                let NE = staged[77] * ((AF * X) + ND);
                let NI;
                let NJ;
                if NF != 0.0 {
                    let NO = if NG != 0.0 {
                        NN
                    } else {
                        BO
                    };
                    oNO = NO;
                    let NR = if NP != 0.0 {
                        NQ
                    } else {
                        BP
                    };
                    oNR = NR;
                    NI = NS;
                    NJ = NT;
                } else {
                    if NH != 0.0 {
                        let NV = ((staged[78] * JX) * NU) * NU;
                        oNV = NV;
                    } else {
                    }
                    let NZ = if NW != 0.0 {
                        let NY = (LS * (JX.sqrt())) / NX;
                        NY
                    } else {
                        NS
                    };
                    let OC = if OA != 0.0 {
                        let OB = (LS * (BJ.sqrt())) / NX;
                        OB
                    } else {
                        NT
                    };
                    oOC = OC;
                    let OD = NZ - OC;
                    oOD = OD;
                    NI = NZ;
                    NJ = OC;
                }
                let NK = AC + BR;
                let NM = if NK < NL { 1.0 } else { 0.0 };
                let OE = if NM != 0.0 {
                    NL
                } else {
                    NK
                };
                let OF = AV + (BQ / OE);
                if OG != 0.0 {
                    if OH != 0.0 {
                        let OI = KX * BM;
                        oOI = OI;
                    } else {
                    }
                } else {
                }
                let OJ = (-5e-1f64 * DG) * X;
                let OK = (-5e-1f64 * DP) * X;
                let OL = if X > LA { 1.0 } else { 0.0 };
                let OO = if OL != 0.0 {
                    let OM = X.ln();
                    OM
                } else {
                    ON
                };
                let OP = IK / ((IL * OO).exp());
                let OQ = G.powf(parameters[226]);
                let OR = F + staged[91];
                let OS = OR.powf(parameters[227]);
                let OT = AV + (((parameters[230] / OQ) + (parameters[231] / OS)) + (parameters[232] / (OQ * OS)));
                let OU = G.powf(parameters[228]);
                let OV = OR.powf(parameters[229]);
                let OW = AV + (((parameters[233] / OU) + (parameters[234] / OV)) + (parameters[235] / (OU * OV)));
                let OX = ((OW * OW) + 1e-9f64).sqrt();
                let OY = IW * G;
                let PB = (AV / (OZ + OY)) + (AV / (PA + OY));
                let PF = if (if (if PC > O { 1.0 } else { 0.0 }) != 0.0 && (if PD > O { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if E == AV { 1.0 } else { 0.0 }) != 0.0 || (if (if E > AV { 1.0 } else { 0.0 }) != 0.0 && (if PE > O { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let PI;
                let PJ;
                let PK;
                let PL;
                let PM;
                if PF != 0.0 {
                    let PH = if PG < -1e0f64 { 1.0 } else { 0.0 };
                    oPH = PH;
                    let PS;
                    if PH != 0.0 {
                        PS = PQ;
                    } else {
                        let PR = if PG > AV { 1.0 } else { 0.0 };
                        oPR = PR;
                        let PT = if PR != 0.0 {
                            AV
                        } else {
                            PG
                        };
                        PS = PT;
                    }
                    oPS = PS;
                    let mut PU = 0.0;
                    let mut PV = 0.0;
                    let mut PW = 0.0;
                    PU = O;
                    PV = O;
                    PW = O;
                    loop {
                        let PX = if PU < E { 1.0 } else { 0.0 };
                        oPX = PX;
                        if PX == 0.0 {
                            break;
                        }
                        let PY = AV / E;
                        let PZ = PU * (PE + G);
                        let QA = PV + (PY / ((PC + OY) + PZ));
                        let QB = PW + (PY / ((PD + OY) + PZ));
                        let QC = PU + AV;
                        PU = QC;
                        PV = QA;
                        PW = QB;
                    }
                    let QD = PV + PW;
                    oQD = QD;
                    let QE = QD - PB;
                    let QF = (parameters[224] / OX) * QE;
                    oQF = QF;
                    let QH = (parameters[236] / (OX.powf(QG))) * QE;
                    oQH = QH;
                    let QK = DC + ((parameters[238] / (OX.powf(QI))) * QE);
                    let QL = DE + ((parameters[240] / (OX.powf(QJ))) * QE);
                    PI = PB;
                    PJ = QD;
                    PK = PS;
                    PL = QK;
                    PM = QL;
                } else {
                    PI = O;
                    PJ = O;
                    PK = O;
                    PL = DC;
                    PM = DE;
                }
                let PN = KX * parameters[22];
                let PO = LT * parameters[8];
                let PP = LT * parameters[7];
                let QM = parameters[10] - D;
                let QN = if QM > O { 1.0 } else { 0.0 };
                let QQ = if QN != 0.0 {
                    let QP = QO * QM;
                    QP
                } else {
                    O
                };
                let QR = parameters[9] - D;
                let QS = if QR > O { 1.0 } else { 0.0 };
                let QU = if QS != 0.0 {
                    let QT = QO * QR;
                    QT
                } else {
                    O
                };
                let QW = QV * parameters[11];
                let QY = if QW <= QX { 1.0 } else { 0.0 };
                let QZ = if QY != 0.0 {
                    QX
                } else {
                    QW
                };
                let RA = QV * parameters[12];
                let RB = if RA <= QX { 1.0 } else { 0.0 };
                let RC = if RB != 0.0 {
                    QX
                } else {
                    RA
                };
                let RE = (((-5e-1f64 * X) * X) / RD) / RD;
                let RG = if RE > RF { 1.0 } else { 0.0 };
                let RJ;
                if RG != 0.0 {
                    let RH = 2.688117142e43f64 * ((AV + RE) - RF);
                    RJ = RH;
                } else {
                    let RI = if RE < -1e2f64 { 1.0 } else { 0.0 };
                    oRI = RI;
                    let RR = if RI != 0.0 {
                        RP
                    } else {
                        let RQ = RE.exp();
                        RQ
                    };
                    RJ = RR;
                }
                let RK = FS * ((AV / X) + staged[105]);
                let RL = RK.powf(FR);
                let RM = AV + (parameters[330] * (RK.powf(GC)));
                let RN = FT + (FU * X);
                let RO = if RN < AV { 1.0 } else { 0.0 };
                let RS = if RO != 0.0 {
                    AV
                } else {
                    RN
                };
                if B != 0.0 {
                } else {
                    let RW = if (if BK > RU { 1.0 } else { 0.0 }) != 0.0 && (if BK < RV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oRW = RW;
                    let RY = (-5e-1f64 * CA) * RX;
                    oRY = RY;
                    let RZ = CY * KA;
                    oRZ = RZ;
                    let SA = if II > O { 1.0 } else { 0.0 };
                    oSA = SA;
                    if SA != 0.0 {
                        let SB = RX / (RX + (W * II));
                        let SC = if SB > LA { 1.0 } else { 0.0 };
                        oSC = SC;
                        let SH = if SC != 0.0 {
                            let SF = SB.ln();
                            SF
                        } else {
                            SG
                        };
                        let SI = staged[106] * SH;
                        oSI = SI;
                    } else {
                    }
                    let SE = ((-5e-1f64 * CD) * SD) * RX;
                    oSE = SE;
                    let SJ = ((AV + (BX / RX)).sqrt()) - AV;
                    oSJ = SJ;
                    let SK = (GV + (GX / RX)) * staged[122];
                    oSK = SK;
                    let SL = SD + BV;
                    oSL = SL;
                    let SM = (AV + (BY / RX)).sqrt();
                    oSM = SM;
                    let SN = AV - IX;
                    oSN = SN;
                }
                let RT = ((-5e-1f64 * CD) * AC) * X;
                let SO = (-5e-1f64 * CA) * X;
                let SP = AC + BV;
                let SQ = AV + (BX / X);
                let SR = (SQ.sqrt()) - AV;
                let SS = GV + (GX / X);
                let ST = ((LZ * SQ) * IZ) * KE;
                let SV = (parameters[406] * (parameters[408] + ((AF / KP) / SU))) / ((SU * E) * (G - parameters[409]));
                let SW = if SV > O { 1.0 } else { 0.0 };
                let TA;
                if SW != 0.0 {
                    let SX = AV / SV;
                    TA = SX;
                } else {
                    let SZ = if SY != O { 1.0 } else { 0.0 };
                    oSZ = SZ;
                    TA = TC;
                }
                let TF;
                let TG;
                if TB != 0.0 {
                    let TE = if TD < QX { 1.0 } else { 0.0 };
                    oTE = TE;
                    let TK = if TE != 0.0 {
                        TC
                    } else {
                        let TJ = TI + (AV / TD);
                        TJ
                    };
                    let TM = if TL < QX { 1.0 } else { 0.0 };
                    oTM = TM;
                    let TO = if TM != 0.0 {
                        TC
                    } else {
                        let TN = TI + (AV / TL);
                        TN
                    };
                    TF = TK;
                    TG = TO;
                } else {
                    TF = O;
                    TG = O;
                }
                let TH = ((staged[136] / MA).sqrt()) / KP;
                if TP != 0.0 {
                    let TQ = CA * X;
                    oTQ = TQ;
                } else {
                }
                let TR = -X;
                let TS = if BX < TR { 1.0 } else { 0.0 };
                let TT = if TS != 0.0 {
                    AV
                } else {
                    O
                };
                let TV;
                if PF != 0.0 {
                    let TU = if OZ <= O { 1.0 } else { 0.0 };
                    oTU = TU;
                    let TX = if TU != 0.0 {
                        AV
                    } else {
                        TT
                    };
                    let TY = if PA <= O { 1.0 } else { 0.0 };
                    oTY = TY;
                    let TZ = if TY != 0.0 {
                        AV
                    } else {
                        TX
                    };
                    TV = TZ;
                } else {
                    TV = TT;
                }
                let TW = if BY < TR { 1.0 } else { 0.0 };
                let UA = if TW != 0.0 {
                    AV
                } else {
                    TV
                };
                let UB = if IP < O { 1.0 } else { 0.0 };
                let UC = if UB != 0.0 {
                    AV
                } else {
                    UA
                };
                let UD = if IQ < O { 1.0 } else { 0.0 };
                let UE = if UD != 0.0 {
                    AV
                } else {
                    UC
                };
                let UG = if UF != 0.0 {
                    AV
                } else {
                    UE
                };
                let UI = if UH != 0.0 {
                    AV
                } else {
                    UG
                };
                let UK = if UJ != 0.0 {
                    AV
                } else {
                    UI
                };
                let UM = if UL != 0.0 {
                    AV
                } else {
                    UK
                };
                let UN = if E < AV { 1.0 } else { 0.0 };
                let UO = if JX <= O { 1.0 } else { 0.0 };
                let UP = if BK < O { 1.0 } else { 0.0 };
                let UQ = if BK > RV { 1.0 } else { 0.0 };
                let UR = if CA < O { 1.0 } else { 0.0 };
                let US = if CD < O { 1.0 } else { 0.0 };
                let UT = -AC;
                let UU = if BV == UT { 1.0 } else { 0.0 };
                let UV = if DG < O { 1.0 } else { 0.0 };
                let UW = if CN == UT { 1.0 } else { 0.0 };
                let UX = if DR < O { 1.0 } else { 0.0 };
                let UY = if DL <= O { 1.0 } else { 0.0 };
                let UZ = if DP < O { 1.0 } else { 0.0 };
                let VA = if IG < JY { 1.0 } else { 0.0 };
                if VA != 0.0 {
                } else {
                    let VC = if IG > VB { 1.0 } else { 0.0 };
                    oVC = VC;
                }
                let VD = if IH < JY { 1.0 } else { 0.0 };
                if VD != 0.0 {
                } else {
                    let VE = if IH > VB { 1.0 } else { 0.0 };
                    oVE = VE;
                }
                if PF != 0.0 {
                    let VF = if QG <= O { 1.0 } else { 0.0 };
                    oVF = VF;
                    let VI = if QI <= O { 1.0 } else { 0.0 };
                    oVI = VI;
                    let VJ = if QJ <= O { 1.0 } else { 0.0 };
                    oVJ = VJ;
                } else {
                }
                let VH = if IF < VG { 1.0 } else { 0.0 };
                let VK = if IF > 2.5e1f64 { 1.0 } else { 0.0 };
                let VL = if HY < VG { 1.0 } else { 0.0 };
                if VM != 0.0 {
                    let VN = if IE < JY { 1.0 } else { 0.0 };
                    oVN = VN;
                    if VN != 0.0 {
                    } else {
                        let VO = if IE > 1.6e0f64 { 1.0 } else { 0.0 };
                        oVO = VO;
                    }
                } else {
                }
                let VP = if HD <= O { 1.0 } else { 0.0 };
                let VQ = if HL <= O { 1.0 } else { 0.0 };
                let VR = if HK <= O { 1.0 } else { 0.0 };
                let VV;
                let VW;
                if VS != 0.0 {
                    let VU = if CR < VT { 1.0 } else { 0.0 };
                    oVU = VU;
                    let VZ;
                    let WA;
                    if VU != 0.0 {
                        VZ = CQ;
                        WA = VT;
                    } else {
                        let VY = if CR > AV { 1.0 } else { 0.0 };
                        oVY = VY;
                        let WB;
                        let WC;
                        if VY != 0.0 {
                            WB = O;
                            WC = AV;
                        } else {
                            WB = CQ;
                            WC = CR;
                        }
                        VZ = WB;
                        WA = WC;
                    }
                    VV = VZ;
                    VW = WA;
                } else {
                    VV = CQ;
                    VW = CR;
                }
                let VX = if CS < O { 1.0 } else { 0.0 };
                let WD = if VX != 0.0 {
                    O
                } else {
                    CS
                };
                if WE != 0.0 {
                    let WG = if X <= WF { 1.0 } else { 0.0 };
                    oWG = WG;
                    let WH = if AK <= WF { 1.0 } else { 0.0 };
                    oWH = WH;
                    let WJ = if AC <= WI { 1.0 } else { 0.0 };
                    oWJ = WJ;
                    let WK = if AM <= WI { 1.0 } else { 0.0 };
                    oWK = WK;
                    let WL = if BX < O { 1.0 } else { 0.0 };
                    oWL = WL;
                    let WM = if JX <= 1e15f64 { 1.0 } else { 0.0 };
                    oWM = WM;
                    if WM != 0.0 {
                    } else {
                        let WO = if JX >= WN { 1.0 } else { 0.0 };
                        oWO = WO;
                    }
                    let WP = if LI >= WN { 1.0 } else { 0.0 };
                    oWP = WP;
                    let WQ = if (if BK > O { 1.0 } else { 0.0 }) != 0.0 && (if BK <= RU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oWQ = WQ;
                    let WR = if BZ < O { 1.0 } else { 0.0 };
                    oWR = WR;
                    let WT = if ((AY / SP).abs()) > WS { 1.0 } else { 0.0 };
                    oWT = WT;
                    let WU = if CY < O { 1.0 } else { 0.0 };
                    oWU = WU;
                    let WV = if DI < O { 1.0 } else { 0.0 };
                    oWV = WV;
                    let WW = if DK < O { 1.0 } else { 0.0 };
                    oWW = WW;
                    let WX = if DC < O { 1.0 } else { 0.0 };
                    oWX = WX;
                    let WY = if DE < O { 1.0 } else { 0.0 };
                    oWY = WY;
                    let WZ = if ((AY / (CN + AC)).abs()) > WS { 1.0 } else { 0.0 };
                    oWZ = WZ;
                    let XA = if DM < O { 1.0 } else { 0.0 };
                    oXA = XA;
                    let XB = if DN < O { 1.0 } else { 0.0 };
                    oXB = XB;
                    let XC = if JC < O { 1.0 } else { 0.0 };
                    oXC = XC;
                    let XD = if JE < O { 1.0 } else { 0.0 };
                    oXD = XD;
                    let XE = if FY < O { 1.0 } else { 0.0 };
                    oXE = XE;
                    let XF = if GA < O { 1.0 } else { 0.0 };
                    oXF = XF;
                    let XG = if FZ < O { 1.0 } else { 0.0 };
                    oXG = XG;
                    let XH = if GB < O { 1.0 } else { 0.0 };
                    oXH = XH;
                    let XI = if FX > KE { 1.0 } else { 0.0 };
                    oXI = XI;
                } else {
                }
                let XJ = if parameters[33] == AV { 1.0 } else { 0.0 };
                let XK = if JC != O { 1.0 } else { 0.0 };
                let XL = if XJ != 0.0 && XK != 0.0 { 1.0 } else { 0.0 };
                if XL != 0.0 {
                    if B != 0.0 {
                    } else {
                        let XN = AV / (((XM * XM) * XM).sqrt());
                        oXN = XN;
                        let XO = staged[142] / (W * (8.617087e-5f64 * XM));
                        oXO = XO;
                    }
                    if KW != 0.0 {
                        let XP = JX / BJ;
                        let XQ = if XP > LA { 1.0 } else { 0.0 };
                        oXQ = XQ;
                        let XU = if XQ != 0.0 {
                            let XS = XP.ln();
                            XS
                        } else {
                            XT
                        };
                        oXU = XU;
                        let XV = -KX;
                        oXV = XV;
                    } else {
                        let XR = (-JX) * BJ;
                        oXR = XR;
                        let XW = -KX;
                        oXW = XW;
                    }
                    let XX = MJ.sqrt();
                    oXX = XX;
                    let XZ = (KA / (XY * 8.85418e-12f64)) * MF;
                    oXZ = XZ;
                    let YA = (-5e-1f64 * DG) * X;
                    oYA = YA;
                    let YB = (-5e-1f64 * DP) * X;
                    oYB = YB;
                    let YC = if GF == GG { 1.0 } else { 0.0 };
                    oYC = YC;
                    let YD = if GF == GJ { 1.0 } else { 0.0 };
                    oYD = YD;
                    let YF = GP - YE;
                    oYF = YF;
                    let YG = if parameters[35] < 4.2e0f64 { 1.0 } else { 0.0 };
                    oYG = YG;
                    let YI = YH * PI;
                    oYI = YI;
                    let YJ = YH * PJ;
                    oYJ = YJ;
                    let YK = if parameters[410] != AV { 1.0 } else { 0.0 };
                    oYK = YK;
                    if YK != 0.0 {
                    } else {
                        let YL = JA * E;
                        oYL = YL;
                    }
                } else {
                }
                if NF != 0.0 {
                    let YM = if (if parameter_given[89] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oYM = YM;
                    let YO = if (if parameter_given[93] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oYO = YO;
                } else {
                    let YN = if (if parameter_given[86] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                    oYN = YN;
                    if YN != 0.0 {
                        let YR = if A != 0.0 {
                            let YP = (JZ / MB) * IZ;
                            YP
                        } else {
                            YQ
                        };
                        let YS = ((YR * JX) * NU) * NU;
                        oYS = YS;
                    } else {
                    }
                }
                if OG != 0.0 {
                    let YT = if (if parameter_given[107] { 1.0 } else { 0.0 }) != 0.0 || (if parameter_given[106] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oYT = YT;
                } else {
                }
                let YU = if (if BK > RU { 1.0 } else { 0.0 }) != 0.0 && (if BK < RV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let YV = if KS == O { 1.0 } else { 0.0 };
                if YV != 0.0 {
                } else {
                    let YW = if parameters[411] == O { 1.0 } else { 0.0 };
                    oYW = YW;
                    if YW != 0.0 {
                        let YX = ((-HX) * X) / MH;
                        let YY = HW * (((IW * YX).exp()) + (W * (YX.exp())));
                        oYY = YY;
                        let YZ = (IW * ST) / KN;
                        oYZ = YZ;
                        let ZA = ((-HV) * X) / MH;
                        let ZB = (HT - (HU * (((IW * ZA).exp()) + (W * (ZA.exp()))))) / (AV + (KN / LT));
                        oZB = ZB;
                        let ZC = AV / (AV + (LT / KN));
                        oZC = ZC;
                    } else {
                        let ZD = AV / ((KN + LT) + HQ);
                        let ZE = ((-HX) * X) / MH;
                        let ZF = HW * (((IW * ZE).exp()) + (W * (ZE.exp())));
                        oZF = ZF;
                        let ZG = (IW * ST) / KN;
                        oZG = ZG;
                        let ZH = KN * ZD;
                        oZH = ZH;
                        let ZI = HQ * ZD;
                        oZI = ZI;
                        let ZJ = LT * ZD;
                        oZJ = ZJ;
                    }
                    let ZK = (-5e-1f64 * CA) * X;
                    oZK = ZK;
                    let ZL = CY * KA;
                    oZL = ZL;
                    let ZM = if II > O { 1.0 } else { 0.0 };
                    oZM = ZM;
                    if ZM != 0.0 {
                        let ZN = -IJ;
                        oZN = ZN;
                    } else {
                    }
                    let ZO = ((-5e-1f64 * CD) * AC) * X;
                    oZO = ZO;
                    let ZP = (AV + (BY / X)).sqrt();
                    oZP = ZP;
                    let ZQ = W * IM;
                    oZQ = ZQ;
                    let ZR = NX / (NX + (AV / ((AV / KN) + (AV / LT))));
                    oZR = ZR;
                    if YW != 0.0 {
                        let ZS = ((-HX) * X) / MH;
                        let ZT = HW * (((IW * ZS).exp()) + (W * (ZS.exp())));
                        oZT = ZT;
                        let ZU = (IW * ST) / KN;
                        oZU = ZU;
                        let ZV = ((-HV) * X) / MH;
                        let ZW = (HT - (HU * (((IW * ZV).exp()) + (W * (ZV.exp()))))) / (AV + (KN / LT));
                        oZW = ZW;
                        let ZX = AV / (AV + (LT / KN));
                        oZX = ZX;
                    } else {
                        let ZY = AV / ((KN + LT) + HQ);
                        let ZZ = ((-HX) * X) / MH;
                        let AAA = HW * (((IW * ZZ).exp()) + (W * (ZZ.exp())));
                        oAAA = AAA;
                        let AAB = (IW * ST) / KN;
                        oAAB = AAB;
                        let AAC = KN * ZY;
                        oAAC = AAC;
                        let AAD = HQ * ZY;
                        oAAD = AAD;
                        let AAE = LT * ZY;
                        oAAE = AAE;
                    }
                    let AAF = if KS == W { 1.0 } else { 0.0 };
                    oAAF = AAF;
                    if YW != 0.0 {
                        let AAG = ((-HX) * X) / MH;
                        let AAH = HW * (((IW * AAG).exp()) + (W * (AAG.exp())));
                        oAAH = AAH;
                        let AAI = (IW * ST) / KN;
                        oAAI = AAI;
                        let AAJ = ((-HV) * X) / MH;
                        let AAK = (HT - (HU * (((IW * AAJ).exp()) + (W * (AAJ.exp()))))) / (AV + (KN / LT));
                        oAAK = AAK;
                        let AAL = AV / (AV + (LT / KN));
                        oAAL = AAL;
                    } else {
                        let AAM = AV / ((KN + LT) + HQ);
                        let AAN = ((-HX) * X) / MH;
                        let AAO = HW * (((IW * AAN).exp()) + (W * (AAN.exp())));
                        oAAO = AAO;
                        let AAP = (IW * ST) / KN;
                        oAAP = AAP;
                        let AAQ = KN * AAM;
                        oAAQ = AAQ;
                        let AAR = HQ * AAM;
                        oAAR = AAR;
                        let AAS = LT * AAM;
                        oAAS = AAS;
                    }
                }
                let AAT = (-5e-1f64 * CA) * X;
                let AAU = CY * KA;
                let AAV = if II > O { 1.0 } else { 0.0 };
                if AAV != 0.0 {
                    let AAW = -IJ;
                    oAAW = AAW;
                } else {
                }
                let AAX = ((-5e-1f64 * CD) * AC) * X;
                let AAY = (AV + (BY / X)).sqrt();
                let AAZ = W * IM;
                let ABA = (-5e-1f64 * CA) * X;
                if AAV != 0.0 {
                    let ABB = -IJ;
                    oABB = ABB;
                } else {
                }
                let ABC = ((-5e-1f64 * CD) * AC) * X;
                let ABD = if (if VM != 0.0 && XJ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && XK != 0.0 { 1.0 } else { 0.0 };
                if ABD != 0.0 {
                    let ABE = (-5e-1f64 * CA) * X;
                    oABE = ABE;
                    let ABG = ((-5e-1f64 * CD) * AC) * X;
                    oABG = ABG;
                } else {
                }
                let ABF = AV - IX;
                let ABH = if IP <= O { 1.0 } else { 0.0 };
                if ABH != 0.0 {
                } else {
                    let ABI = IP * (X.sqrt());
                    oABI = ABI;
                }
                let ABJ = if CK == O { 1.0 } else { 0.0 };
                if ABJ != 0.0 {
                } else {
                    let ABK = CM / (AC + CN);
                    oABK = ABK;
                    let ABL = CL * CK;
                    oABL = ABL;
                }
                if ABJ != 0.0 {
                } else {
                    let ABM = CM / (AC + CN);
                    oABM = ABM;
                }
                if ABN != 0.0 {
                } else {
                    if ABO != 0.0 {
                    } else {
                        if ABP != 0.0 {
                        } else {
                            let ABQ = GU - YE;
                            oABQ = ABQ;
                            let ABR = GR - YE;
                            oABR = ABR;
                        }
                    }
                }
                let ABS = if VV == O { 1.0 } else { 0.0 };
                if ABS != 0.0 {
                } else {
                    let ABT = if VV > O { 1.0 } else { 0.0 };
                    oABT = ABT;
                    if ABT != 0.0 {
                        let ABU = AV - VW;
                        oABU = ABU;
                        let ABW = ABV * ABU;
                        oABW = ABW;
                        let ABX = VW + ABU;
                        oABX = ABX;
                    } else {
                        let ABY = ABV * VW;
                        oABY = ABY;
                    }
                }
                let ABZ = VB * DR;
                let ACA = if DL > O { 1.0 } else { 0.0 };
                let ACB = if IQ > RP { 1.0 } else { 0.0 };
                if ACB != 0.0 {
                    let ACC = AV + (parameters[270] * X);
                    oACC = ACC;
                } else {
                }
                let ACD = if KS != W { 1.0 } else { 0.0 };
                if ACD != 0.0 {
                    let ACG = if B != 0.0 {
                        let ACE = (1.17e1f64 / XY) * MF;
                        ACE
                    } else {
                        let ACF = (parameters[45] * MF) / XY;
                        ACF
                    };
                    oACG = ACG;
                    let ACH = if parameters[41] == O { 1.0 } else { 0.0 };
                    oACH = ACH;
                    let ACI = AJ * KE;
                    oACI = ACI;
                    let ACJ = AH * KE;
                    oACJ = ACJ;
                    let ACK = AF * KE;
                    oACK = ACK;
                } else {
                }
                if ACL != 0.0 {
                    let ACN = (HE * HG) - HF;
                    oACN = ACN;
                    let ACO = HF * HG;
                    oACO = ACO;
                    let ACP = -HK;
                    oACP = ACP;
                    let ACQ = (HH * HJ) - HI;
                    oACQ = ACQ;
                    let ACR = HI * HJ;
                    oACR = ACR;
                } else {
                }
                let ACM = if staged[312] != 0.0 && ACD != 0.0 { 1.0 } else { 0.0 };
                if ACM != 0.0 {
                    let ACS = (VB * parameters[382]) * parameters[381];
                    oACS = ACS;
                    let ACW = if parameters[373] != O { 1.0 } else { 0.0 };
                    oACW = ACW;
                    let ACX = parameters[988] * MS;
                    oACX = ACX;
                    let ACY = if parameters[377] != O { 1.0 } else { 0.0 };
                    oACY = ACY;
                    let ACZ = parameters[990] * MS;
                    oACZ = ACZ;
                } else {
                }
                let ACU = if ACT > O { 1.0 } else { 0.0 };
                let ACV = if (if ACM != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                if ACD != 0.0 {
                    let ADA = if parameters[42] == O { 1.0 } else { 0.0 };
                    oADA = ADA;
                    if ADA != 0.0 {
                        let ADC = if DS <= O { 1.0 } else { 0.0 };
                        oADC = ADC;
                        if ADC != 0.0 {
                        } else {
                            let ADF = ED / X;
                            oADF = ADF;
                            let ADG = EE * X;
                            let ADH = (EF * ADG) / (AV + ADG);
                            oADH = ADH;
                        }
                    } else {
                        let ADD = if DS <= O { 1.0 } else { 0.0 };
                        oADD = ADD;
                        if ADD != 0.0 {
                        } else {
                            let ADI = ED / X;
                            oADI = ADI;
                            let ADJ = EE * X;
                            let ADK = (EF * ADJ) / (AV + ADJ);
                            oADK = ADK;
                        }
                        let ADL = (DV + (DU * X)) / X;
                        oADL = ADL;
                        let ADM = DY - AV;
                        oADM = ADM;
                    }
                    if ADE != 0.0 {
                    } else {
                        let ADN = if JJ < QX { 1.0 } else { 0.0 };
                        oADN = ADN;
                        if ADN != 0.0 {
                            let ADO = if C <= QX { 1.0 } else { 0.0 };
                            oADO = ADO;
                            let ADS = if ADO != 0.0 {
                                ADQ
                            } else {
                                let ADR = AV / C;
                                ADR
                            };
                            oADS = ADS;
                        } else {
                            let ADP = JJ + C;
                            oADP = ADP;
                        }
                    }
                } else {
                }
                let ADB = if SY > AV { 1.0 } else { 0.0 };
                if ADB != 0.0 {
                    let ADT = if E != AV { 1.0 } else { 0.0 };
                    oADT = ADT;
                    let ADU = if SY == W { 1.0 } else { 0.0 };
                    oADU = ADU;
                } else {
                }
                if JN != 0.0 {
                    let ADV = -CV;
                    oADV = ADV;
                } else {
                }
                let ADW = if E != AV { 1.0 } else { 0.0 };
                let ADX = AO * E;
                let ADZ = NX * ((ADX * AK) + ADY);
                let AEB = AEA * ((ADX * AR) + ADY);
                let AEC = NX * ACT;
                let AED = AEA * ACT;
                if AEE != 0.0 {
                } else {
                    if AEF != 0.0 {
                    } else {
                        let AEH = AV - IY;
                        oAEH = AEH;
                    }
                }
                if AEG != 0.0 {
                    let AEI = if KS == W { 1.0 } else { 0.0 };
                    oAEI = AEI;
                    if AEI != 0.0 {
                    } else {
                        let AEK = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                        oAEK = AEK;
                        let AEL = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                        oAEL = AEL;
                    }
                    if AEI != 0.0 {
                    } else {
                        let AEM = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                        oAEM = AEM;
                    }
                    let AEN = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                    oAEN = AEN;
                    if AEO != 0.0 {
                        let AEP = -ADZ;
                        oAEP = AEP;
                        let AEQ = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                        oAEQ = AEQ;
                    } else {
                        if AER != 0.0 {
                            let AES = IW * ADZ;
                            oAES = AES;
                            let AET = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                            oAET = AET;
                            if AET != 0.0 {
                                let AEU = IW * AEC;
                                oAEU = AEU;
                            } else {
                            }
                        } else {
                        }
                    }
                    if AEI != 0.0 {
                    } else {
                        let AEX = ((BU * AEV) * LT) * ((ADX * AT) + AEW);
                        oAEX = AEX;
                    }
                } else {
                    if VM != 0.0 {
                        let AEY = ADZ * MF;
                        oAEY = AEY;
                        let AEZ = AEB * MD;
                        oAEZ = AEZ;
                        if ACU != 0.0 {
                            let AFA = AEC * MD;
                            oAFA = AFA;
                            let AFB = AED * MD;
                            oAFB = AFB;
                        } else {
                        }
                        let AFC = if KS == W { 1.0 } else { 0.0 };
                        oAFC = AFC;
                        if AFC != 0.0 {
                        } else {
                            let AFD = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                            oAFD = AFD;
                            let AFE = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                            oAFE = AFE;
                            let AFF = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                            oAFF = AFF;
                        }
                        let AFG = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                        oAFG = AFG;
                        let AFH = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                        oAFH = AFH;
                        if AFC != 0.0 {
                        } else {
                            let AFI = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                            oAFI = AFI;
                        }
                        if AFJ != 0.0 {
                            let AFK = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                            oAFK = AFK;
                        } else {
                            if AFL != 0.0 {
                                let AFM = if (if ACD != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ACU != 0.0 { 1.0 } else { 0.0 };
                                oAFM = AFM;
                            } else {
                            }
                        }
                        if AFC != 0.0 {
                        } else {
                            let AFN = ((BU * AEV) * LT) * ((ADX * AT) + AEW);
                            oAFN = AFN;
                        }
                    } else {
                    }
                }
                let AEJ = if KS == W { 1.0 } else { 0.0 };
                if AEJ != 0.0 {
                } else {
                    let AFO = -parameters[350];
                    oAFO = AFO;
                    let AFP = (((parameters[175] * AQ) * KE) * E) / WI;
                    oAFP = AFP;
                    let AFQ = AFP * parameters[349];
                    oAFQ = AFQ;
                    let AFR = (((parameters[176] * AP) * KE) * E) / WI;
                    oAFR = AFR;
                    let AFS = AFR * parameters[351];
                    oAFS = AFS;
                    let AFU = -parameters[352];
                    oAFU = AFU;
                    let AFW = if AFV == IW { 1.0 } else { 0.0 };
                    oAFW = AFW;
                    if AFW != 0.0 {
                    } else {
                        let AFX = -AFV;
                        oAFX = AFX;
                    }
                    let AFY = AV - AFV;
                    oAFY = AFY;
                }
                let AFT = -KX;
                let AGA = AP * GM;
                if AFZ != 0.0 {
                    let AGB = JP + AGA;
                    oAGB = AGB;
                    let AGC = IW * GO;
                    oAGC = AGC;
                } else {
                    let AGD = JP + AGA;
                    oAGD = AGD;
                    let AGE = IW * GO;
                    oAGE = AGE;
                }
                let AGF = AQ * GN;
                if AFZ != 0.0 {
                    let AGG = JQ + AGF;
                    oAGG = AGG;
                    let AGH = IW * GO;
                    oAGH = AGH;
                } else {
                    let AGI = JQ + AGF;
                    oAGI = AGI;
                    let AGJ = IW * GO;
                    oAGJ = AGJ;
                }
                let AGK = E * AC;
                if AGL != 0.0 {
                } else {
                    let AGN = ((AGM * X) * X) * AGK;
                    oAGN = AGN;
                    let AGO = (AGK * X) * AGM;
                    oAGO = AGO;
                }
                let AGQ = if AEJ != 0.0 {
                    AGP
                } else {
                    O
                };
                let AGS = if XL != 0.0 {
                    O
                } else {
                    AGR
                };
            [P, X, Y, AB, AC, AD, AH, AJ, AL, AN, AS, AU, AW, BJ, BK, BL, BM, BN, BQ, BS, BT, BZ, CB, CC, CE, CG, CH, CI, CJ, CK, CO, CP, CS, CT, CU, CV, CW, CY, CZ, DA, DB, DD, DF, DH, DI, DJ, DK, DL, DM, DN, DO, DQ, DR, DS, DT, DW, DX, DZ, EA, EB, EC, EG, EH, EI, EJ, EK, EL, EM, EN, EO, EP, EQ, ER, ES, ET, EU, EV, EW, EX, EY, EZ, FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK, FL, FM, FN, FO, FP, FQ, FV, FW, FX, FY, FZ, GA, GB, GD, GE, GF, GG, GH, GI, GJ, GK, GL, GO, GP, GQ, GR, GS, GT, GU, GW, GY, GZ, HA, HB, HC, HD, HE, HH, HM, HN, HO, HP, HR, HS, HY, IB, IC, ID, IE, IF, IG, IH, II, IO, IQ, IR, IS, IT, IU, IX, IY, JA, JD, JF, JK, JM, oJO, JR, JW, oKD, oKG, JX, KQ, oKR, oKT, KW, oKY, oLB, oLH, oLC, oLD, oLJ, oLL, oLM, oLN, oLO, oLR, LI, LU, oLW, oLY, MC, MI, MJ, oMK, oMM, MU, NA, NB, NC, ND, NE, oNV, oOC, oOD, NK, NM, oNO, OF, oOI, OJ, OK, OL, OP, OT, PB, PF, oPH, oPR, oPX, oQD, oPS, oQF, oQH, oNR, PN, PO, PP, QN, QS, QY, RB, RG, oRI, RJ, RL, RM, RO, oRW, oRY, oRZ, oSA, oSC, oSI, oSE, oSJ, oSK, oSL, oSM, oSN, RT, SO, SP, SR, SS, ST, SW, oSZ, oTE, oTM, TH, oTQ, TS, oTU, oTY, TW, UB, UD, UN, UO, UP, UQ, UR, US, UU, UV, UW, UX, UY, UZ, VA, oVC, VD, oVE, oVF, oVI, oVJ, VH, VK, VL, oVN, oVO, VP, VQ, VR, oVU, oVY, VX, oWG, oWH, oWJ, oWK, oWL, oWM, oWO, oWP, oWQ, oWR, oWT, oWU, oWV, oWW, oWX, oWY, oWZ, oXA, oXB, oXC, oXD, oXE, oXF, oXG, oXH, oXI, UM, XL, oXN, oXO, oXQ, oXU, oXV, oXR, oXW, oXX, oXZ, oYA, oYB, oYC, oYD, oYG, oYI, oYJ, PK, oYK, WD, oYL, oYM, oYO, oYN, oYS, NI, NJ, oYT, YU, YV, oYW, MH, oYY, oYZ, oZB, oZC, oZF, oZG, oZH, oZI, oZJ, oZK, oZL, oZM, oZN, oZO, PL, PM, oZP, oZQ, oZR, oZT, oZU, oZW, oZX, oAAA, oAAB, oAAC, oAAD, oAAE, oAAF, oAAH, oAAI, oAAK, oAAL, oAAO, oAAP, oAAQ, oAAR, oAAS, AAT, AAU, AAV, oAAW, AAX, AAY, AAZ, ABA, oABB, ABC, ABD, oABE, oABG, ABF, ABH, oABI, QZ, RC, ABJ, oABK, oABL, oABM, VV, ABS, VW, oABT, oABU, oABW, oABX, oABY, ABZ, ACA, ACB, oACC, ACD, oACH, oACG, oMN, oACI, oACJ, oACK, RS, oACN, oACO, oACP, oACQ, oACR, ACM, oACS, oACW, oACX, oACY, oACZ, ACU, ACV, oADA, oADC, oADF, oADH, oADD, oADI, oADK, oADL, oADM, oADN, oADO, oADS, oADP, ADB, oADT, oADU, TA, oADV, ADW, ADZ, AEB, AEC, AED, oAEH, oAEI, oAEK, oAEL, oAEM, oAEN, oAEP, oAEQ, oAES, oAET, oAEU, oAEX, oAEY, oAEZ, oAFA, oAFB, oAFC, oAFD, oAFE, oAFF, oAFG, oAFH, oAFI, oAFK, oAFM, oAFN, AEJ, oAFO, oAFP, oAFQ, oAFR, oAFS, oAFU, oAFW, oAFX, oAFY, AFT, QQ, QU, AGA, oAGB, oAGC, oAGD, oAGE, AGF, oAGG, oAGH, oAGI, oAGJ, AGK, oAGN, oAGO, TF, TG, AGQ, AGS, oYF, oABQ, oABR]
        };
        self.canonical_staged[453] = produced[0];
        self.canonical_staged[186] = produced[1];
        self.canonical_staged[454] = produced[2];
        self.canonical_staged[239] = produced[3];
        self.canonical_staged[240] = produced[4];
        self.canonical_staged[455] = produced[5];
        self.canonical_staged[778] = produced[6];
        self.canonical_staged[773] = produced[7];
        self.canonical_staged[456] = produced[8];
        self.canonical_staged[457] = produced[9];
        self.canonical_staged[458] = produced[10];
        self.canonical_staged[459] = produced[11];
        self.canonical_staged[362] = produced[12];
        self.canonical_staged[100] = produced[13];
        self.canonical_staged[111] = produced[14];
        self.canonical_staged[72] = produced[15];
        self.canonical_staged[533] = produced[16];
        self.canonical_staged[531] = produced[17];
        self.canonical_staged[161] = produced[18];
        self.canonical_staged[126] = produced[19];
        self.canonical_staged[199] = produced[20];
        self.canonical_staged[118] = produced[21];
        self.canonical_staged[178] = produced[22];
        self.canonical_staged[120] = produced[23];
        self.canonical_staged[179] = produced[24];
        self.canonical_staged[5] = produced[25];
        self.canonical_staged[7] = produced[26];
        self.canonical_staged[9] = produced[27];
        self.canonical_staged[13] = produced[28];
        self.canonical_staged[249] = produced[29];
        self.canonical_staged[245] = produced[30];
        self.canonical_staged[246] = produced[31];
        self.canonical_staged[15] = produced[32];
        self.canonical_staged[19] = produced[33];
        self.canonical_staged[17] = produced[34];
        self.canonical_staged[242] = produced[35];
        self.canonical_staged[241] = produced[36];
        self.canonical_staged[138] = produced[37];
        self.canonical_staged[237] = produced[38];
        self.canonical_staged[238] = produced[39];
        self.canonical_staged[129] = produced[40];
        self.canonical_staged[190] = produced[41];
        self.canonical_staged[192] = produced[42];
        self.canonical_staged[116] = produced[43];
        self.canonical_staged[115] = produced[44];
        self.canonical_staged[182] = produced[45];
        self.canonical_staged[183] = produced[46];
        self.canonical_staged[273] = produced[47];
        self.canonical_staged[89] = produced[48];
        self.canonical_staged[90] = produced[49];
        self.canonical_staged[275] = produced[50];
        self.canonical_staged[279] = produced[51];
        self.canonical_staged[270] = produced[52];
        self.canonical_staged[335] = produced[53];
        self.canonical_staged[336] = produced[54];
        self.canonical_staged[339] = produced[55];
        self.canonical_staged[340] = produced[56];
        self.canonical_staged[334] = produced[57];
        self.canonical_staged[332] = produced[58];
        self.canonical_staged[333] = produced[59];
        self.canonical_staged[326] = produced[60];
        self.canonical_staged[328] = produced[61];
        self.canonical_staged[329] = produced[62];
        self.canonical_staged[330] = produced[63];
        self.canonical_staged[775] = produced[64];
        self.canonical_staged[776] = produced[65];
        self.canonical_staged[777] = produced[66];
        self.canonical_staged[774] = produced[67];
        self.canonical_staged[782] = produced[68];
        self.canonical_staged[784] = produced[69];
        self.canonical_staged[783] = produced[70];
        self.canonical_staged[770] = produced[71];
        self.canonical_staged[771] = produced[72];
        self.canonical_staged[772] = produced[73];
        self.canonical_staged[769] = produced[74];
        self.canonical_staged[779] = produced[75];
        self.canonical_staged[781] = produced[76];
        self.canonical_staged[780] = produced[77];
        self.canonical_staged[295] = produced[78];
        self.canonical_staged[297] = produced[79];
        self.canonical_staged[26] = produced[80];
        self.canonical_staged[36] = produced[81];
        self.canonical_staged[29] = produced[82];
        self.canonical_staged[39] = produced[83];
        self.canonical_staged[285] = produced[84];
        self.canonical_staged[288] = produced[85];
        self.canonical_staged[31] = produced[86];
        self.canonical_staged[41] = produced[87];
        self.canonical_staged[32] = produced[88];
        self.canonical_staged[42] = produced[89];
        self.canonical_staged[33] = produced[90];
        self.canonical_staged[35] = produced[91];
        self.canonical_staged[43] = produced[92];
        self.canonical_staged[45] = produced[93];
        self.canonical_staged[287] = produced[94];
        self.canonical_staged[289] = produced[95];
        self.canonical_staged[296] = produced[96];
        self.canonical_staged[298] = produced[97];
        self.canonical_staged[30] = produced[98];
        self.canonical_staged[40] = produced[99];
        self.canonical_staged[248] = produced[100];
        self.canonical_staged[317] = produced[101];
        self.canonical_staged[320] = produced[102];
        self.canonical_staged[316] = produced[103];
        self.canonical_staged[319] = produced[104];
        self.canonical_staged[284] = produced[105];
        self.canonical_staged[286] = produced[106];
        self.canonical_staged[25] = produced[107];
        self.canonical_staged[27] = produced[108];
        self.canonical_staged[28] = produced[109];
        self.canonical_staged[34] = produced[110];
        self.canonical_staged[37] = produced[111];
        self.canonical_staged[38] = produced[112];
        self.canonical_staged[44] = produced[113];
        self.canonical_staged[416] = produced[114];
        self.canonical_staged[10] = produced[115];
        self.canonical_staged[262] = produced[116];
        self.canonical_staged[261] = produced[117];
        self.canonical_staged[258] = produced[118];
        self.canonical_staged[260] = produced[119];
        self.canonical_staged[259] = produced[120];
        self.canonical_staged[188] = produced[121];
        self.canonical_staged[4] = produced[122];
        self.canonical_staged[6] = produced[123];
        self.canonical_staged[8] = produced[124];
        self.canonical_staged[12] = produced[125];
        self.canonical_staged[14] = produced[126];
        self.canonical_staged[299] = produced[127];
        self.canonical_staged[301] = produced[128];
        self.canonical_staged[307] = produced[129];
        self.canonical_staged[346] = produced[130];
        self.canonical_staged[345] = produced[131];
        self.canonical_staged[168] = produced[132];
        self.canonical_staged[171] = produced[133];
        self.canonical_staged[200] = produced[134];
        self.canonical_staged[201] = produced[135];
        self.canonical_staged[202] = produced[136];
        self.canonical_staged[499] = produced[137];
        self.canonical_staged[506] = produced[138];
        self.canonical_staged[352] = produced[139];
        self.canonical_staged[375] = produced[140];
        self.canonical_staged[383] = produced[141];
        self.canonical_staged[350] = produced[142];
        self.canonical_staged[351] = produced[143];
        self.canonical_staged[185] = produced[144];
        self.canonical_staged[355] = produced[145];
        self.canonical_staged[278] = produced[146];
        self.canonical_staged[276] = produced[147];
        self.canonical_staged[323] = produced[148];
        self.canonical_staged[325] = produced[149];
        self.canonical_staged[324] = produced[150];
        self.canonical_staged[127] = produced[151];
        self.canonical_staged[353] = produced[152];
        self.canonical_staged[16] = produced[153];
        self.canonical_staged[435] = produced[154];
        self.canonical_staged[436] = produced[155];
        self.canonical_staged[463] = produced[156];
        self.canonical_staged[11] = produced[157];
        self.canonical_staged[18] = produced[158];
        self.canonical_staged[432] = produced[159];
        self.canonical_staged[474] = produced[160];
        self.canonical_staged[475] = produced[161];
        self.canonical_staged[476] = produced[162];
        self.canonical_staged[62] = produced[163];
        self.canonical_staged[477] = produced[164];
        self.canonical_staged[478] = produced[165];
        self.canonical_staged[480] = produced[166];
        self.canonical_staged[99] = produced[167];
        self.canonical_staged[46] = produced[168];
        self.canonical_staged[496] = produced[169];
        self.canonical_staged[47] = produced[170];
        self.canonical_staged[48] = produced[171];
        self.canonical_staged[49] = produced[172];
        self.canonical_staged[51] = produced[173];
        self.canonical_staged[50] = produced[174];
        self.canonical_staged[502] = produced[175];
        self.canonical_staged[53] = produced[176];
        self.canonical_staged[503] = produced[177];
        self.canonical_staged[52] = produced[178];
        self.canonical_staged[54] = produced[179];
        self.canonical_staged[59] = produced[180];
        self.canonical_staged[505] = produced[181];
        self.canonical_staged[61] = produced[182];
        self.canonical_staged[64] = produced[183];
        self.canonical_staged[68] = produced[184];
        self.canonical_staged[70] = produced[185];
        self.canonical_staged[510] = produced[186];
        self.canonical_staged[513] = produced[187];
        self.canonical_staged[516] = produced[188];
        self.canonical_staged[310] = produced[189];
        self.canonical_staged[311] = produced[190];
        self.canonical_staged[309] = produced[191];
        self.canonical_staged[314] = produced[192];
        self.canonical_staged[304] = produced[193];
        self.canonical_staged[79] = produced[194];
        self.canonical_staged[83] = produced[195];
        self.canonical_staged[82] = produced[196];
        self.canonical_staged[764] = produced[197];
        self.canonical_staged[521] = produced[198];
        self.canonical_staged[523] = produced[199];
        self.canonical_staged[84] = produced[200];
        self.canonical_staged[85] = produced[201];
        self.canonical_staged[87] = produced[202];
        self.canonical_staged[88] = produced[203];
        self.canonical_staged[534] = produced[204];
        self.canonical_staged[195] = produced[205];
        self.canonical_staged[92] = produced[206];
        self.canonical_staged[93] = produced[207];
        self.canonical_staged[536] = produced[208];
        self.canonical_staged[537] = produced[209];
        self.canonical_staged[539] = produced[210];
        self.canonical_staged[540] = produced[211];
        self.canonical_staged[94] = produced[212];
        self.canonical_staged[95] = produced[213];
        self.canonical_staged[96] = produced[214];
        self.canonical_staged[97] = produced[215];
        self.canonical_staged[524] = produced[216];
        self.canonical_staged[98] = produced[217];
        self.canonical_staged[101] = produced[218];
        self.canonical_staged[102] = produced[219];
        self.canonical_staged[544] = produced[220];
        self.canonical_staged[545] = produced[221];
        self.canonical_staged[546] = produced[222];
        self.canonical_staged[547] = produced[223];
        self.canonical_staged[549] = produced[224];
        self.canonical_staged[550] = produced[225];
        self.canonical_staged[290] = produced[226];
        self.canonical_staged[292] = produced[227];
        self.canonical_staged[293] = produced[228];
        self.canonical_staged[551] = produced[229];
        self.canonical_staged[109] = produced[230];
        self.canonical_staged[113] = produced[231];
        self.canonical_staged[114] = produced[232];
        self.canonical_staged[557] = produced[233];
        self.canonical_staged[558] = produced[234];
        self.canonical_staged[117] = produced[235];
        self.canonical_staged[119] = produced[236];
        self.canonical_staged[121] = produced[237];
        self.canonical_staged[123] = produced[238];
        self.canonical_staged[124] = produced[239];
        self.canonical_staged[125] = produced[240];
        self.canonical_staged[128] = produced[241];
        self.canonical_staged[131] = produced[242];
        self.canonical_staged[132] = produced[243];
        self.canonical_staged[133] = produced[244];
        self.canonical_staged[134] = produced[245];
        self.canonical_staged[135] = produced[246];
        self.canonical_staged[177] = produced[247];
        self.canonical_staged[567] = produced[248];
        self.canonical_staged[568] = produced[249];
        self.canonical_staged[569] = produced[250];
        self.canonical_staged[570] = produced[251];
        self.canonical_staged[376] = produced[252];
        self.canonical_staged[137] = produced[253];
        self.canonical_staged[574] = produced[254];
        self.canonical_staged[578] = produced[255];
        self.canonical_staged[580] = produced[256];
        self.canonical_staged[579] = produced[257];
        self.canonical_staged[581] = produced[258];
        self.canonical_staged[582] = produced[259];
        self.canonical_staged[591] = produced[260];
        self.canonical_staged[594] = produced[261];
        self.canonical_staged[595] = produced[262];
        self.canonical_staged[596] = produced[263];
        self.canonical_staged[597] = produced[264];
        self.canonical_staged[598] = produced[265];
        self.canonical_staged[599] = produced[266];
        self.canonical_staged[600] = produced[267];
        self.canonical_staged[601] = produced[268];
        self.canonical_staged[603] = produced[269];
        self.canonical_staged[605] = produced[270];
        self.canonical_staged[606] = produced[271];
        self.canonical_staged[608] = produced[272];
        self.canonical_staged[609] = produced[273];
        self.canonical_staged[610] = produced[274];
        self.canonical_staged[611] = produced[275];
        self.canonical_staged[612] = produced[276];
        self.canonical_staged[614] = produced[277];
        self.canonical_staged[615] = produced[278];
        self.canonical_staged[613] = produced[279];
        self.canonical_staged[616] = produced[280];
        self.canonical_staged[617] = produced[281];
        self.canonical_staged[618] = produced[282];
        self.canonical_staged[620] = produced[283];
        self.canonical_staged[621] = produced[284];
        self.canonical_staged[622] = produced[285];
        self.canonical_staged[623] = produced[286];
        self.canonical_staged[627] = produced[287];
        self.canonical_staged[629] = produced[288];
        self.canonical_staged[628] = produced[289];
        self.canonical_staged[631] = produced[290];
        self.canonical_staged[633] = produced[291];
        self.canonical_staged[634] = produced[292];
        self.canonical_staged[635] = produced[293];
        self.canonical_staged[636] = produced[294];
        self.canonical_staged[638] = produced[295];
        self.canonical_staged[639] = produced[296];
        self.canonical_staged[640] = produced[297];
        self.canonical_staged[641] = produced[298];
        self.canonical_staged[642] = produced[299];
        self.canonical_staged[643] = produced[300];
        self.canonical_staged[646] = produced[301];
        self.canonical_staged[647] = produced[302];
        self.canonical_staged[648] = produced[303];
        self.canonical_staged[649] = produced[304];
        self.canonical_staged[650] = produced[305];
        self.canonical_staged[651] = produced[306];
        self.canonical_staged[653] = produced[307];
        self.canonical_staged[654] = produced[308];
        self.canonical_staged[670] = produced[309];
        self.canonical_staged[671] = produced[310];
        self.canonical_staged[679] = produced[311];
        self.canonical_staged[680] = produced[312];
        self.canonical_staged[683] = produced[313];
        self.canonical_staged[684] = produced[314];
        self.canonical_staged[720] = produced[315];
        self.canonical_staged[588] = produced[316];
        self.canonical_staged[721] = produced[317];
        self.canonical_staged[141] = produced[318];
        self.canonical_staged[143] = produced[319];
        self.canonical_staged[752] = produced[320];
        self.canonical_staged[145] = produced[321];
        self.canonical_staged[144] = produced[322];
        self.canonical_staged[146] = produced[323];
        self.canonical_staged[147] = produced[324];
        self.canonical_staged[148] = produced[325];
        self.canonical_staged[149] = produced[326];
        self.canonical_staged[150] = produced[327];
        self.canonical_staged[151] = produced[328];
        self.canonical_staged[753] = produced[329];
        self.canonical_staged[754] = produced[330];
        self.canonical_staged[755] = produced[331];
        self.canonical_staged[152] = produced[332];
        self.canonical_staged[153] = produced[333];
        self.canonical_staged[154] = produced[334];
        self.canonical_staged[756] = produced[335];
        self.canonical_staged[155] = produced[336];
        self.canonical_staged[156] = produced[337];
        self.canonical_staged[757] = produced[338];
        self.canonical_staged[759] = produced[339];
        self.canonical_staged[758] = produced[340];
        self.canonical_staged[157] = produced[341];
        self.canonical_staged[762] = produced[342];
        self.canonical_staged[763] = produced[343];
        self.canonical_staged[765] = produced[344];
        self.canonical_staged[163] = produced[345];
        self.canonical_staged[785] = produced[346];
        self.canonical_staged[786] = produced[347];
        self.canonical_staged[274] = produced[348];
        self.canonical_staged[166] = produced[349];
        self.canonical_staged[167] = produced[350];
        self.canonical_staged[169] = produced[351];
        self.canonical_staged[170] = produced[352];
        self.canonical_staged[172] = produced[353];
        self.canonical_staged[173] = produced[354];
        self.canonical_staged[174] = produced[355];
        self.canonical_staged[175] = produced[356];
        self.canonical_staged[176] = produced[357];
        self.canonical_staged[180] = produced[358];
        self.canonical_staged[181] = produced[359];
        self.canonical_staged[787] = produced[360];
        self.canonical_staged[184] = produced[361];
        self.canonical_staged[187] = produced[362];
        self.canonical_staged[191] = produced[363];
        self.canonical_staged[193] = produced[364];
        self.canonical_staged[197] = produced[365];
        self.canonical_staged[194] = produced[366];
        self.canonical_staged[204] = produced[367];
        self.canonical_staged[205] = produced[368];
        self.canonical_staged[206] = produced[369];
        self.canonical_staged[207] = produced[370];
        self.canonical_staged[208] = produced[371];
        self.canonical_staged[209] = produced[372];
        self.canonical_staged[210] = produced[373];
        self.canonical_staged[211] = produced[374];
        self.canonical_staged[212] = produced[375];
        self.canonical_staged[213] = produced[376];
        self.canonical_staged[788] = produced[377];
        self.canonical_staged[214] = produced[378];
        self.canonical_staged[215] = produced[379];
        self.canonical_staged[216] = produced[380];
        self.canonical_staged[217] = produced[381];
        self.canonical_staged[218] = produced[382];
        self.canonical_staged[219] = produced[383];
        self.canonical_staged[220] = produced[384];
        self.canonical_staged[221] = produced[385];
        self.canonical_staged[222] = produced[386];
        self.canonical_staged[223] = produced[387];
        self.canonical_staged[224] = produced[388];
        self.canonical_staged[789] = produced[389];
        self.canonical_staged[225] = produced[390];
        self.canonical_staged[226] = produced[391];
        self.canonical_staged[228] = produced[392];
        self.canonical_staged[227] = produced[393];
        self.canonical_staged[229] = produced[394];
        self.canonical_staged[230] = produced[395];
        self.canonical_staged[231] = produced[396];
        self.canonical_staged[790] = produced[397];
        self.canonical_staged[233] = produced[398];
        self.canonical_staged[234] = produced[399];
        self.canonical_staged[235] = produced[400];
        self.canonical_staged[791] = produced[401];
        self.canonical_staged[236] = produced[402];
        self.canonical_staged[243] = produced[403];
        self.canonical_staged[244] = produced[404];
        self.canonical_staged[793] = produced[405];
        self.canonical_staged[250] = produced[406];
        self.canonical_staged[251] = produced[407];
        self.canonical_staged[253] = produced[408];
        self.canonical_staged[264] = produced[409];
        self.canonical_staged[797] = produced[410];
        self.canonical_staged[268] = produced[411];
        self.canonical_staged[798] = produced[412];
        self.canonical_staged[265] = produced[413];
        self.canonical_staged[266] = produced[414];
        self.canonical_staged[267] = produced[415];
        self.canonical_staged[269] = produced[416];
        self.canonical_staged[271] = produced[417];
        self.canonical_staged[272] = produced[418];
        self.canonical_staged[799] = produced[419];
        self.canonical_staged[277] = produced[420];
        self.canonical_staged[800] = produced[421];
        self.canonical_staged[802] = produced[422];
        self.canonical_staged[280] = produced[423];
        self.canonical_staged[514] = produced[424];
        self.canonical_staged[282] = produced[425];
        self.canonical_staged[283] = produced[426];
        self.canonical_staged[291] = produced[427];
        self.canonical_staged[294] = produced[428];
        self.canonical_staged[300] = produced[429];
        self.canonical_staged[302] = produced[430];
        self.canonical_staged[305] = produced[431];
        self.canonical_staged[306] = produced[432];
        self.canonical_staged[308] = produced[433];
        self.canonical_staged[804] = produced[434];
        self.canonical_staged[313] = produced[435];
        self.canonical_staged[805] = produced[436];
        self.canonical_staged[318] = produced[437];
        self.canonical_staged[806] = produced[438];
        self.canonical_staged[321] = produced[439];
        self.canonical_staged[819] = produced[440];
        self.canonical_staged[322] = produced[441];
        self.canonical_staged[807] = produced[442];
        self.canonical_staged[809] = produced[443];
        self.canonical_staged[327] = produced[444];
        self.canonical_staged[331] = produced[445];
        self.canonical_staged[810] = produced[446];
        self.canonical_staged[337] = produced[447];
        self.canonical_staged[338] = produced[448];
        self.canonical_staged[342] = produced[449];
        self.canonical_staged[341] = produced[450];
        self.canonical_staged[811] = produced[451];
        self.canonical_staged[812] = produced[452];
        self.canonical_staged[343] = produced[453];
        self.canonical_staged[344] = produced[454];
        self.canonical_staged[808] = produced[455];
        self.canonical_staged[813] = produced[456];
        self.canonical_staged[814] = produced[457];
        self.canonical_staged[347] = produced[458];
        self.canonical_staged[348] = produced[459];
        self.canonical_staged[815] = produced[460];
        self.canonical_staged[363] = produced[461];
        self.canonical_staged[357] = produced[462];
        self.canonical_staged[364] = produced[463];
        self.canonical_staged[358] = produced[464];
        self.canonical_staged[354] = produced[465];
        self.canonical_staged[820] = produced[466];
        self.canonical_staged[822] = produced[467];
        self.canonical_staged[824] = produced[468];
        self.canonical_staged[825] = produced[469];
        self.canonical_staged[826] = produced[470];
        self.canonical_staged[365] = produced[471];
        self.canonical_staged[828] = produced[472];
        self.canonical_staged[366] = produced[473];
        self.canonical_staged[830] = produced[474];
        self.canonical_staged[367] = produced[475];
        self.canonical_staged[368] = produced[476];
        self.canonical_staged[370] = produced[477];
        self.canonical_staged[371] = produced[478];
        self.canonical_staged[372] = produced[479];
        self.canonical_staged[373] = produced[480];
        self.canonical_staged[831] = produced[481];
        self.canonical_staged[834] = produced[482];
        self.canonical_staged[835] = produced[483];
        self.canonical_staged[837] = produced[484];
        self.canonical_staged[838] = produced[485];
        self.canonical_staged[839] = produced[486];
        self.canonical_staged[840] = produced[487];
        self.canonical_staged[842] = produced[488];
        self.canonical_staged[844] = produced[489];
        self.canonical_staged[391] = produced[490];
        self.canonical_staged[821] = produced[491];
        self.canonical_staged[392] = produced[492];
        self.canonical_staged[395] = produced[493];
        self.canonical_staged[394] = produced[494];
        self.canonical_staged[397] = produced[495];
        self.canonical_staged[396] = produced[496];
        self.canonical_staged[398] = produced[497];
        self.canonical_staged[846] = produced[498];
        self.canonical_staged[400] = produced[499];
        self.canonical_staged[401] = produced[500];
        self.canonical_staged[402] = produced[501];
        self.canonical_staged[414] = produced[502];
        self.canonical_staged[415] = produced[503];
        self.canonical_staged[419] = produced[504];
        self.canonical_staged[417] = produced[505];
        self.canonical_staged[418] = produced[506];
        self.canonical_staged[420] = produced[507];
        self.canonical_staged[421] = produced[508];
        self.canonical_staged[424] = produced[509];
        self.canonical_staged[422] = produced[510];
        self.canonical_staged[423] = produced[511];
        self.canonical_staged[425] = produced[512];
        self.canonical_staged[426] = produced[513];
        self.canonical_staged[427] = produced[514];
        self.canonical_staged[429] = produced[515];
        self.canonical_staged[431] = produced[516];
        self.canonical_staged[433] = produced[517];
        self.canonical_staged[434] = produced[518];
        self.canonical_staged[879] = produced[519];
        self.canonical_staged[880] = produced[520];
        self.canonical_staged[437] = produced[521];
        self.canonical_staged[438] = produced[522];
        self.canonical_staged[439] = produced[523];
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
        let produced: [f64; 135] = {
            let parameters = &self.params.values;
            let multiplicity = self.multiplicity;
            let temperature = ctx.temperature();
            let staged = &*self.canonical_staged;
                let C = staged[452];
                let D = 8.617087e-5f64;
                let H = 2e0f64;
                let P = 1e0f64;
                let Y = staged[464];
                let AB = 0e0f64;
                let AK = staged[18];
                let AX = staged[26];
                let AZ = 1e2f64;
                let BB = 2.688117142e43f64;
                let BH = 3.720075976e-44f64;
                let CG = staged[36];
                let DO = staged[99];
                let DU = 1e-38f64;
                let DX = staged[498];
                let EA = -8.749823353377374e1f64;
                let ED = staged[499];
                let EK = staged[502];
                let EN = -8.749823353377374e1f64;
                let EP = 3e-1f64;
                let EU = -8.749823353377374e1f64;
                let EX = staged[504];
                let EY = staged[505];
                let EZ = staged[506];
                let FB = staged[507];
                let FC = staged[59];
                let FG = staged[23];
                let FI = staged[56];
                let FK = parameters[342];
                let FP = -8.749823353377374e1f64;
                let FY = -8.749823353377374e1f64;
                let GF = staged[514];
                let GH = -8.749823353377374e1f64;
                let GJ = staged[71];
                let GL = staged[73];
                let GO = parameters[34];
                let GQ = staged[517];
                let GR = staged[518];
                let GS = staged[520];
                let GW = staged[523];
                let GX = staged[524];
                let GY = parameters[86];
                let HE = staged[81];
                let HJ = staged[529];
                let HK = staged[530];
                let HL = staged[531];
                let HN = staged[532];
                let HP = -1e0f64;
                let HS = staged[533];
                let HU = parameters[64];
                let HV = parameters[66];
                let IE = staged[536];
                let IL = staged[98];
                let IQ = staged[540];
                let IT = staged[95];
                let IX = staged[100];
                let JH = parameters[343];
                let JJ = staged[101];
                let JQ = 3e0f64;
                let JS = staged[102];
                let KT = staged[552];
                let KZ = -8.749823353377374e1f64;
                let LB = staged[106];
                let LE = -8.749823353377374e1f64;
                let LI = staged[108];
                let LK = staged[80];
                let LN = 5e-1f64;
                let LO = parameters[986];
                let LX = 3.720075976e-44f64;
                let LZ = staged[115];
                let MA = staged[116];
                let ME = 8e0f64;
                let MH = staged[557];
                let MJ = staged[118];
                let MQ = 3.720075976e-44f64;
                let MS = staged[120];
                let MT = staged[66];
                let MV = staged[126];
                let MY = staged[127];
                let NA = staged[128];
                let NB = staged[129];
                let NG = 4e0f64;
                let NO = -8.749823353377374e1f64;
                let NS = 1e6f64;
                let OB = -8.749823353377374e1f64;
                let OH = 3.720075976e-44f64;
                let OO = 3.720075976e-44f64;
                let PA = staged[572];
                let PH = 3.7200759757663865e-44f64;
                let PZ = staged[588];
                let QB = staged[589];
                let QD = staged[590];
                let QF = staged[591];
                let QH = staged[592];
                let QJ = staged[593];
                let QL = staged[594];
                let QN = staged[595];
                let QP = staged[596];
                let QR = staged[597];
                let QT = staged[598];
                let QV = staged[599];
                let QX = staged[600];
                let QZ = staged[601];
                let RD = staged[603];
                let RH = staged[605];
                let RJ = staged[606];
                let RL = staged[607];
                let RN = staged[232];
                let RQ = staged[621];
                let RS = staged[622];
                let RU = staged[623];
                let RW = staged[624];
                let RY = staged[625];
                let SA = staged[628];
                let SB = 1e-3f64;
                let SE = parameters[61];
                let SI = staged[721];
                let SJ = staged[757];
                let SK = 5.3e-1f64;
                let SM = staged[765];
                let SO = staged[785];
                let SQ = staged[793];
                let SR = staged[228];
                let SU = staged[801];
                let SW = staged[818];
                let SX = staged[819];
                let SY = staged[820];
                let TC = staged[824];
                let TD = staged[358];
                let TN = staged[364];
                let TQ = staged[831];
                let mut oAC = 0.0;
                let mut oAI = 0.0;
                let mut oAP = 0.0;
                let mut oAR = 0.0;
                let mut oBD = 0.0;
                let mut oBL = 0.0;
                let mut oBS = 0.0;
                let mut oCD = 0.0;
                let mut oCM = 0.0;
                let mut oCT = 0.0;
                let mut oDA = 0.0;
                let mut oDL = 0.0;
                let mut oDV = 0.0;
                let mut oEJ = 0.0;
                let mut oGD = 0.0;
                let mut oGM = 0.0;
                let mut oHA = 0.0;
                let mut oIY = 0.0;
                let mut oLJ = 0.0;
                let mut oLU = 0.0;
                let mut oMC = 0.0;
                let mut oMM = 0.0;
                let mut oND = 0.0;
                let mut oNE = 0.0;
                let mut oNI = 0.0;
                let mut oNL = 0.0;
                let mut oNW = 0.0;
                let mut oNZ = 0.0;
                let mut oPC = 0.0;
                let mut oPK = 0.0;
                let mut oPQ = 0.0;
                let mut oPS = 0.0;
                let mut oSC = 0.0;
                let mut oSG = 0.0;
                let mut oSH = 0.0;
                let mut oSL = 0.0;
                let mut oSN = 0.0;
                let mut oSP = 0.0;
                let mut oSS = 0.0;
                let mut oST = 0.0;
                let mut oSV = 0.0;
                let mut oSZ = 0.0;
                let mut oTA = 0.0;
                let mut oTB = 0.0;
                let mut oTE = 0.0;
                let mut oTH = 0.0;
                let mut oTI = 0.0;
                let mut oTJ = 0.0;
                let mut oTK = 0.0;
                let mut oTO = 0.0;
                let mut oTP = 0.0;
                let mut oTR = 0.0;
                let mut oTS = 0.0;
                let mut oTT = 0.0;
                let mut oTU = 0.0;
                let mut oTV = 0.0;
                let mut oTW = 0.0;
                let mut oTX = 0.0;
                let mut oUB = 0.0;
                let mut oUC = 0.0;
                let mut oUE = 0.0;
                let A = temperature + parameters[0];
                let B = A / staged[0];
                let M;
                let N;
                let O;
                if C != 0.0 {
                    let E = D * A;
                    let F = 1.16e0f64 - (((7.02e-4f64 * A) * A) / (A + 1.108e3f64));
                    let G = A / 3.0015e2f64;
                    let I = ((1.45e10f64 * G) * (G.sqrt())) * ((2.15565981e1f64 - (F / (H * E))).exp());
                    M = E;
                    N = I;
                    O = F;
                } else {
                    let J = D * A;
                    let K = parameters[47] - (((parameters[48] * A) * A) / (A + parameters[49]));
                    let L = ((parameters[46] * B) * (B.sqrt())) * ((staged[1] - (K / (H * J))).exp());
                    M = J;
                    N = L;
                    O = K;
                }
                let Q = B - P;
                let R = staged[5] + (staged[4] * Q);
                let S = staged[7] + (staged[6] * Q);
                let T = staged[9] + (staged[8] * Q);
                let U = staged[11] * (B.powf(staged[10]));
                let V = staged[13] - (staged[12] * Q);
                let W = staged[14] * Q;
                let X = (staged[15] + W) / staged[16];
                let AD;
                let AE;
                let AF;
                let AG;
                if Y != 0.0 {
                    let Z = staged[17] + W;
                    let AA = parameters[133] + W;
                    let AC = if Z < AB { 1.0 } else { 0.0 };
                    oAC = AC;
                    let AH = if AC != 0.0 {
                        AB
                    } else {
                        Z
                    };
                    let AI = if AA < AB { 1.0 } else { 0.0 };
                    oAI = AI;
                    let AJ = if AI != 0.0 {
                        AB
                    } else {
                        AA
                    };
                    let AL = AH / AK;
                    let AM = AJ / AK;
                    let AN = staged[19] + W;
                    let AO = parameters[132] + W;
                    let AP = if AN < AB { 1.0 } else { 0.0 };
                    oAP = AP;
                    let AQ = if AP != 0.0 {
                        AB
                    } else {
                        AN
                    };
                    let AR = if AO < AB { 1.0 } else { 0.0 };
                    oAR = AR;
                    let AS = if AR != 0.0 {
                        AB
                    } else {
                        AO
                    };
                    let AT = AQ / AK;
                    let AU = AS / AK;
                    AD = AL;
                    AE = AT;
                    AF = AM;
                    AG = AU;
                } else {
                    AD = AB;
                    AE = AB;
                    AF = AB;
                    AG = AB;
                }
                let AV = (1.115e0f64 / M) * Q;
                let AW = staged[25] * AV;
                let AY = AW / AX;
                let BA = if AY > AZ { 1.0 } else { 0.0 };
                let BE;
                if BA != 0.0 {
                    let BC = BB * ((P + AY) - AZ);
                    BE = BC;
                } else {
                    let BD = if AY < -1e2f64 { 1.0 } else { 0.0 };
                    oBD = BD;
                    let BJ = if BD != 0.0 {
                        BH
                    } else {
                        let BI = AY.exp();
                        BI
                    };
                    BE = BJ;
                }
                let BF = (staged[27] * AV) / AX;
                let BG = if BF > AZ { 1.0 } else { 0.0 };
                let BM;
                if BG != 0.0 {
                    let BK = BB * ((P + BF) - AZ);
                    BM = BK;
                } else {
                    let BL = if BF < -1e2f64 { 1.0 } else { 0.0 };
                    oBL = BL;
                    let BQ = if BL != 0.0 {
                        BH
                    } else {
                        let BP = BF.exp();
                        BP
                    };
                    BM = BQ;
                }
                let BN = (staged[28] * AV) / staged[29];
                let BO = if BN > AZ { 1.0 } else { 0.0 };
                let BT;
                if BO != 0.0 {
                    let BR = BB * ((P + BN) - AZ);
                    BT = BR;
                } else {
                    let BS = if BN < -1e2f64 { 1.0 } else { 0.0 };
                    oBS = BS;
                    let CB = if BS != 0.0 {
                        BH
                    } else {
                        let CA = BN.exp();
                        CA
                    };
                    BT = CB;
                }
                let BU = staged[30] * BE;
                let BV = staged[31] * BE;
                let BW = staged[32] * BM;
                let BX = staged[33] * BT;
                let BY = staged[34] * Q;
                let BZ = if BY > AZ { 1.0 } else { 0.0 };
                let CE;
                if BZ != 0.0 {
                    let CC = BB * ((P + BY) - AZ);
                    CE = CC;
                } else {
                    let CD = if BY < -1e2f64 { 1.0 } else { 0.0 };
                    oCD = CD;
                    let CK = if CD != 0.0 {
                        BH
                    } else {
                        let CJ = BY.exp();
                        CJ
                    };
                    CE = CK;
                }
                let CF = staged[35] * CE;
                let CH = AW / CG;
                let CI = if CH > AZ { 1.0 } else { 0.0 };
                let CN;
                if CI != 0.0 {
                    let CL = BB * ((P + CH) - AZ);
                    CN = CL;
                } else {
                    let CM = if CH < -1e2f64 { 1.0 } else { 0.0 };
                    oCM = CM;
                    let CR = if CM != 0.0 {
                        BH
                    } else {
                        let CQ = CH.exp();
                        CQ
                    };
                    CN = CR;
                }
                let CO = (staged[37] * AV) / CG;
                let CP = if CO > AZ { 1.0 } else { 0.0 };
                let CU;
                if CP != 0.0 {
                    let CS = BB * ((P + CO) - AZ);
                    CU = CS;
                } else {
                    let CT = if CO < -1e2f64 { 1.0 } else { 0.0 };
                    oCT = CT;
                    let CY = if CT != 0.0 {
                        BH
                    } else {
                        let CX = CO.exp();
                        CX
                    };
                    CU = CY;
                }
                let CV = (staged[38] * AV) / staged[39];
                let CW = if CV > AZ { 1.0 } else { 0.0 };
                let DB;
                if CW != 0.0 {
                    let CZ = BB * ((P + CV) - AZ);
                    DB = CZ;
                } else {
                    let DA = if CV < -1e2f64 { 1.0 } else { 0.0 };
                    oDA = DA;
                    let DJ = if DA != 0.0 {
                        BH
                    } else {
                        let DI = CV.exp();
                        DI
                    };
                    DB = DJ;
                }
                let DC = staged[40] * CN;
                let DD = staged[41] * CN;
                let DE = staged[42] * CU;
                let DF = staged[43] * DB;
                let DG = staged[44] * Q;
                let DH = if DG > AZ { 1.0 } else { 0.0 };
                let DM;
                if DH != 0.0 {
                    let DK = BB * ((P + DG) - AZ);
                    DM = DK;
                } else {
                    let DL = if DG < -1e2f64 { 1.0 } else { 0.0 };
                    oDL = DL;
                    let DQ = if DL != 0.0 {
                        BH
                    } else {
                        let DP = DG.exp();
                        DP
                    };
                    DM = DQ;
                }
                let DN = staged[45] * DM;
                let DW;
                if DO != 0.0 {
                    let DR = staged[46] * M;
                    let DY = DR * staged[47];
                    DW = DY;
                } else {
                    let DS = staged[48] * M;
                    let DT = (staged[49] / N) / N;
                    let DV = if DT > DU { 1.0 } else { 0.0 };
                    oDV = DV;
                    let EB = if DV != 0.0 {
                        let DZ = DT.ln();
                        DZ
                    } else {
                        EA
                    };
                    let EC = DS * EB;
                    DW = EC;
                }
                let EE;
                if DX != 0.0 {
                    let EL;
                    if DO != 0.0 {
                        let EI = (staged[50] / N) / N;
                        let EJ = if EI > DU { 1.0 } else { 0.0 };
                        oEJ = EJ;
                        let EO = if EJ != 0.0 {
                            let EM = EI.ln();
                            EM
                        } else {
                            EN
                        };
                        let EQ = staged[51] * ((M * EO) - EP);
                        EL = EQ;
                    } else {
                        let ER = if EK != 0.0 {
                            let ES = staged[53] * ((M * staged[52]) + EP);
                            ES
                        } else {
                            ED
                        };
                        EL = ER;
                    }
                    EE = EL;
                } else {
                    EE = ED;
                }
                let EF = H * M;
                let EG = staged[54] / N;
                let EH = if EG > DU { 1.0 } else { 0.0 };
                let EV = if EH != 0.0 {
                    let ET = EG.ln();
                    ET
                } else {
                    EU
                };
                let EW = EF * EV;
                let FA;
                if EX != 0.0 {
                    let FF = if EY != 0.0 {
                        let FD = (EE + EW) + (FC * (EW.sqrt()));
                        FD
                    } else {
                        let FE = (EE - EW) - (FC * (EW.sqrt()));
                        FE
                    };
                    FA = FF;
                } else {
                    FA = EZ;
                }
                let FL = if FB != 0.0 {
                    let FH = FG / (((staged[60] * EW) / staged[61]).sqrt());
                    let FJ = (FH * FI) / (FH + FI);
                    FJ
                } else {
                    FK
                };
                let FM = staged[62] / N;
                let FN = if FM > DU { 1.0 } else { 0.0 };
                let FQ = if FN != 0.0 {
                    let FO = FM.ln();
                    FO
                } else {
                    FP
                };
                let FR = EF * FQ;
                let FS = FR.sqrt();
                let FT = staged[64] * FS;
                let FU = FT.sqrt();
                let FV = staged[68] / (N * N);
                let FW = if FV > DU { 1.0 } else { 0.0 };
                let FZ = if FW != 0.0 {
                    let FX = FV.ln();
                    FX
                } else {
                    FY
                };
                let GA = M * FZ;
                let GB = (staged[70] / FR).sqrt();
                let GE;
                if C != 0.0 {
                    GE = GF;
                } else {
                    let GC = staged[72] / N;
                    let GD = if GC > DU { 1.0 } else { 0.0 };
                    oGD = GD;
                    let GI = if GD != 0.0 {
                        let GG = GC.ln();
                        GG
                    } else {
                        GH
                    };
                    let GK = GJ * GI;
                    let GM = if GK > GL { 1.0 } else { 0.0 };
                    oGM = GM;
                    let GN = if GM != 0.0 {
                        GL
                    } else {
                        GK
                    };
                    let GP = parameters[50] - (staged[74] - (GO * GN));
                    GE = GP;
                }
                let GT;
                let GU;
                let GV;
                if GR != 0.0 {
                    GT = GW;
                    GU = GX;
                    GV = GY;
                } else {
                    let GZ = if GS != 0.0 {
                        let HB = FR - staged[79];
                        HB
                    } else {
                        GY
                    };
                    let HA = if GZ > AB { 1.0 } else { 0.0 };
                    oHA = HA;
                    let HD = if HA != 0.0 {
                        let HC = -GZ;
                        HC
                    } else {
                        GZ
                    };
                    let HF = (FR - HE).sqrt();
                    let HG = (staged[82] * (((FR - HD).sqrt()) - FS)) / ((H * (FS * (HF - FS))) + HE);
                    let HH = staged[83] - ((H * HG) * HF);
                    GT = HH;
                    GU = HG;
                    GV = HD;
                }
                let HI = GT * staged[84];
                let HM;
                if HJ != 0.0 {
                    let HQ = if HK != 0.0 {
                        let HO = (staged[85] - FR) - (HI * FS);
                        HO
                    } else {
                        HP
                    };
                    HM = HQ;
                } else {
                    HM = HL;
                }
                let HT = if HN != 0.0 {
                    let HR = GO * ((HM + FR) + (HI * FS));
                    HR
                } else {
                    HS
                };
                let HW = (HI * HU) / HV;
                let HX = staged[86] * FU;
                let HY = (staged[87] / HX).exp();
                let HZ = HY + ((H * HY) * HY);
                let IA = (staged[88] / HX).exp();
                let IB = (staged[89] * (IA + ((H * IA) * IA))) + staged[90];
                let IC = parameters[222] / ((staged[92] * (P + (parameters[225] * Q))) + 1e-9f64);
                let ID = IC * staged[93];
                let IF;
                let IG;
                let IH;
                let II;
                if IE != 0.0 {
                    loop {
                        if IQ == 0.0 {
                            break;
                        }
                    }
                    let IR = IC * staged[94];
                    let IS = U * ((P + IR) / (P + ID));
                    let IU = V * ((P + (IT * IR)) / (P + (IT * ID)));
                    let IV = HT + staged[96];
                    let IW = GU + staged[97];
                    IF = IW;
                    IG = IV;
                    IH = IS;
                    II = IU;
                } else {
                    IF = GU;
                    IG = HT;
                    IH = U;
                    II = V;
                }
                let IJ = (IF * HU) / HV;
                let IK = IG + parameters[22];
                let IM = HM + IL;
                let IN = FL * parameters[8];
                let IO = FL * parameters[7];
                let IP = if FL > AB { 1.0 } else { 0.0 };
                let IZ;
                let JA;
                let JB;
                let JC;
                let JD;
                let JE;
                let JF;
                if IP != 0.0 {
                    let IY = if (if DO != 0.0 && (if GO > AB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if IX < AB { 1.0 } else { 0.0 }) != 0.0 && (if GO < AB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oIY = IY;
                    let KM;
                    let KN;
                    let KO;
                    let KP;
                    let KQ;
                    let KR;
                    let KS;
                    if IY != 0.0 {
                        let JG = FA - EE;
                        let JI = EE + (JH * JG);
                        let JK = JJ - IN;
                        let JL = (JK / JG) / JG;
                        let JM = JL / JH;
                        let JN = P - JH;
                        let JO = JL / JN;
                        let JP = P + JH;
                        let JR = (((JG * JK) * JP) / JQ) - (IN * EE);
                        let JT = JS - IO;
                        let JU = (JT / JG) / JG;
                        let JV = JU / JH;
                        let JW = JU / JN;
                        let JX = (((JG * JT) * JP) / JQ) - (IO * EE);
                        KM = JI;
                        KN = JM;
                        KO = JR;
                        KP = JO;
                        KQ = JV;
                        KR = JX;
                        KS = JW;
                    } else {
                        let JY = EE - FA;
                        let JZ = FA + (JH * JY);
                        let KA = IN - JJ;
                        let KB = (KA / JY) / JY;
                        let KC = KB / JH;
                        let KD = P - JH;
                        let KE = KB / KD;
                        let KF = P + JH;
                        let KG = (((JY * KA) * KF) / JQ) - (JJ * FA);
                        let KH = IO - JS;
                        let KI = (KH / JY) / JY;
                        let KJ = KI / JH;
                        let KK = KI / KD;
                        let KL = (((JY * KH) * KF) / JQ) - (JS * FA);
                        KM = JZ;
                        KN = KC;
                        KO = KG;
                        KP = KE;
                        KQ = KJ;
                        KR = KL;
                        KS = KK;
                    }
                    IZ = KM;
                    JA = KN;
                    JB = KO;
                    JC = KP;
                    JD = KQ;
                    JE = KR;
                    JF = KS;
                } else {
                    IZ = AB;
                    JA = AB;
                    JB = AB;
                    JC = AB;
                    JD = AB;
                    JE = AB;
                    JF = AB;
                }
                let KU;
                if C != 0.0 {
                    KU = KT;
                } else {
                    let LA = if FW != 0.0 {
                        let KY = FV.ln();
                        KY
                    } else {
                        KZ
                    };
                    let LC = LB * LA;
                    let LF = if FN != 0.0 {
                        let LD = FM.ln();
                        LD
                    } else {
                        LE
                    };
                    let LG = staged[107] * LF;
                    let LH = LG.sqrt();
                    let LJ = if (if staged[109] != 0.0 && (if LI > (IM + LG) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && staged[110] != 0.0 { 1.0 } else { 0.0 };
                    oLJ = LJ;
                    let LR = if LJ != 0.0 {
                        let LL = ((1.60219e-13f64 * FG) * staged[111]) / (LK * LK);
                        let LM = LL * (((P + ((H * (LI - staged[112])) / LL)).sqrt()) - P);
                        let LP = (LO - (((LN * LM) * LM) / LL)) - 5e-2f64;
                        let LQ = LI - (LO - (LN * (LP + (((LP * LP) + 2.24e-1f64).sqrt()))));
                        LQ
                    } else {
                        LI
                    };
                    let LS = LC - LG;
                    let LT = staged[113] / HX;
                    let LU = if LT > -1e2f64 { 1.0 } else { 0.0 };
                    oLU = LU;
                    let LY = if LU != 0.0 {
                        let LV = LT.exp();
                        let LW = LV * (P + (H * LV));
                        LW
                    } else {
                        LX
                    };
                    let MB = (((staged[114] / FT) + (LZ * LY)) + MA) / LK;
                    let MC = if MB >= -5e-1f64 { 1.0 } else { 0.0 };
                    oMC = MC;
                    let MG = if MC != 0.0 {
                        let MD = P + MB;
                        MD
                    } else {
                        let MF = (P + (JQ * MB)) * (P / (JQ + (ME * MB)));
                        MF
                    };
                    let MI = if MH != 0.0 {
                        let MN = MG * staged[117];
                        MN
                    } else {
                        AB
                    };
                    let MK = (MJ * LY) * LS;
                    let ML = staged[119] / HX;
                    let MM = if ML > -1e2f64 { 1.0 } else { 0.0 };
                    oMM = MM;
                    let MR = if MM != 0.0 {
                        let MO = ML.exp();
                        let MP = MO * (P + (H * MO));
                        MP
                    } else {
                        MQ
                    };
                    let MU = GO * IK;
                    let MW = LR - ((((((MU + (((HW * LH) - (HI * LH)) * staged[125])) - MK) - ((MS * MR) * LS)) + (MV * ((MT * LG) / staged[124]))) + (((HW * staged[121]) * LH) + staged[123])) - MI);
                    let MX = MG * LB;
                    let MZ = (MY * MW) / MX;
                    let NC = (NB - (NA * MW)) / MX;
                    let ND = if MZ > AZ { 1.0 } else { 0.0 };
                    oND = ND;
                    let NF;
                    if ND != 0.0 {
                        NF = MW;
                    } else {
                        let NE = if NC > AZ { 1.0 } else { 0.0 };
                        oNE = NE;
                        let NM;
                        if NE != 0.0 {
                            let NJ = ((LB * GB) / LK) * (((MW - NB) / MX).exp());
                            NM = NJ;
                        } else {
                            let NK = P + (MZ.exp());
                            let NL = if NK > DU { 1.0 } else { 0.0 };
                            oNL = NL;
                            let NP = if NL != 0.0 {
                                let NN = NK.ln();
                                NN
                            } else {
                                NO
                            };
                            let NQ = (MX * NP) / (MY - ((MX * ((((-LK) / (LB * GB)) * (NC.exp())) * NA)) / NA));
                            NM = NQ;
                        }
                        NF = NM;
                    }
                    let NH = NG * ((MU - IM) - LG);
                    let NI = if NH < AB { 1.0 } else { 0.0 };
                    oNI = NI;
                    let NR = if NI != 0.0 {
                        AB
                    } else {
                        NH
                    };
                    let mut NT = 0.0;
                    let mut NU = 0.0;
                    let mut NV = 0.0;
                    NT = AB;
                    NU = MT;
                    NV = NS;
                    loop {
                        let NW = if (if NT <= NG { 1.0 } else { 0.0 }) != 0.0 && (if ((NU - NV).abs()) > 1e-12f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        oNW = NW;
                        if NW == 0.0 {
                            break;
                        }
                        let NX = (NF + NR) / (2e8f64 * NU);
                        let NY = parameters[57] * 7e-1f64;
                        let NZ = if NX > DU { 1.0 } else { 0.0 };
                        oNZ = NZ;
                        let OC = if NZ != 0.0 {
                            let OA = NX.ln();
                            OA
                        } else {
                            OB
                        };
                        let OD = MT - ((staged[130] / parameters[45]) * ((parameters[56] * 1.9e-9f64) / (P + ((NY * OC).exp()))));
                        let OE = NT + P;
                        let edge0 = OE;
                        let edge1 = OD;
                        let edge2 = NU;
                        NT = edge0;
                        NU = edge1;
                        NV = edge2;
                    }
                    KU = NU;
                }
                let KV = GA - FR;
                let KW = staged[131] / HX;
                let KX = if KW > -1e2f64 { 1.0 } else { 0.0 };
                let OI = if KX != 0.0 {
                    let OF = KW.exp();
                    let OG = OF * (P + (H * OF));
                    OG
                } else {
                    OH
                };
                let OJ = (MS * OI) * KV;
                let OK = staged[132] / HX;
                let OL = if OK > -1e2f64 { 1.0 } else { 0.0 };
                let OP = if OL != 0.0 {
                    let OM = OK.exp();
                    let ON = OM * (P + (H * OM));
                    ON
                } else {
                    OO
                };
                let OQ = HW * staged[134];
                let OR = ((((((GO * HT) - OJ) - ((MJ * OP) * KV)) + (MV * ((KU * FR) / staged[133]))) + ((OQ * FS) + (staged[135] * Q))) - FR) - (GT * FS);
                let OS = OR + IL;
                let OT = GO * IK;
                let OU = (OT - IM) - FR;
                let OV = OU + OU;
                let OW = 2.5e0f64 * OU;
                let OX = if GQ != 0.0 {
                    OV
                } else {
                    OW
                };
                let OY = if OX < AB { 1.0 } else { 0.0 };
                let OZ = if OY != 0.0 {
                    AB
                } else {
                    OX
                };
                let PD;
                if PA != 0.0 {
                    let PB = staged[137] / HX;
                    let PC = if PB < AZ { 1.0 } else { 0.0 };
                    oPC = PC;
                    let PI = if PC != 0.0 {
                        let PE = PB.exp();
                        let PF = PE - P;
                        let PG = PE / ((PF * PF) + ((H * PE) * BH));
                        PG
                    } else {
                        PH
                    };
                    let PJ = (((staged[138] * (FG / FT)) + (LZ * PI)) + MA) / LK;
                    let PK = if PJ >= -5e-1f64 { 1.0 } else { 0.0 };
                    oPK = PK;
                    let PN = if PK != 0.0 {
                        let PL = P + PJ;
                        PL
                    } else {
                        let PM = (P + (JQ * PJ)) * (P / (JQ + (ME * PJ)));
                        PM
                    };
                    let PO = PN * GJ;
                    let PP = NB / PO;
                    let PQ = if PP < -1e2f64 { 1.0 } else { 0.0 };
                    oPQ = PQ;
                    let PT;
                    if PQ != 0.0 {
                        let PR = MY + (((LK * BH) / GB) * PN);
                        PT = PR;
                    } else {
                        let PS = if PP > AZ { 1.0 } else { 0.0 };
                        oPS = PS;
                        let PX = if PS != 0.0 {
                            let PV = MY + (((LK * BB) / GB) * PN);
                            PV
                        } else {
                            let PW = MY + ((((PP.exp()) * LK) / GB) * PN);
                            PW
                        };
                        PT = PX;
                    }
                    let PU = (PO * 6.931471805599453e-1f64) / PT;
                    PD = PU;
                } else {
                    PD = AB;
                }
                let PY = if KU <= AB { 1.0 } else { 0.0 };
                let QA = if PY != 0.0 {
                    P
                } else {
                    PZ
                };
                let QC = if QB != 0.0 {
                    P
                } else {
                    QA
                };
                let QE = if QD != 0.0 {
                    P
                } else {
                    QC
                };
                let QG = if QF != 0.0 {
                    P
                } else {
                    QE
                };
                let QI = if QH != 0.0 {
                    P
                } else {
                    QG
                };
                let QK = if QJ != 0.0 {
                    P
                } else {
                    QI
                };
                let QM = if QL != 0.0 {
                    P
                } else {
                    QK
                };
                let QO = if QN != 0.0 {
                    P
                } else {
                    QM
                };
                let QQ = if QP != 0.0 {
                    P
                } else {
                    QO
                };
                let QS = if QR != 0.0 {
                    P
                } else {
                    QQ
                };
                let QU = if QT != 0.0 {
                    P
                } else {
                    QS
                };
                let QW = if QV != 0.0 {
                    P
                } else {
                    QU
                };
                let QY = if QX != 0.0 {
                    P
                } else {
                    QW
                };
                let RA = if QZ != 0.0 {
                    P
                } else {
                    QY
                };
                let RB = if U <= AB { 1.0 } else { 0.0 };
                let RC = if RB != 0.0 {
                    P
                } else {
                    RA
                };
                let RE = if RD != 0.0 {
                    P
                } else {
                    RC
                };
                let RF = if V <= AB { 1.0 } else { 0.0 };
                let RG = if RF != 0.0 {
                    P
                } else {
                    RE
                };
                let RI = if RH != 0.0 {
                    P
                } else {
                    RG
                };
                let RK = if RJ != 0.0 {
                    P
                } else {
                    RI
                };
                let RM = if RL != 0.0 {
                    P
                } else {
                    RK
                };
                let RO = if IP != 0.0 && staged[139] != 0.0 { 1.0 } else { 0.0 };
                let RP = if RO != 0.0 {
                    P
                } else {
                    RM
                };
                let RR = if RQ != 0.0 {
                    P
                } else {
                    RP
                };
                let RT = if RS != 0.0 {
                    P
                } else {
                    RR
                };
                let RV = if RU != 0.0 {
                    P
                } else {
                    RT
                };
                let RX = if RW != 0.0 {
                    P
                } else {
                    RV
                };
                let RZ = if RY != 0.0 {
                    P
                } else {
                    RX
                };
                let SD;
                if SA != 0.0 {
                    SD = AB;
                } else {
                    let SC = if (if X < SB { 1.0 } else { 0.0 }) != 0.0 && (if X != AB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oSC = SC;
                    let SF = if SC != 0.0 {
                        AB
                    } else {
                        X
                    };
                    SD = SF;
                }
                if SE != 0.0 {
                    let SG = if V < 1e3f64 { 1.0 } else { 0.0 };
                    oSG = SG;
                    let SH = if FL < AB { 1.0 } else { 0.0 };
                    oSH = SH;
                } else {
                }
                if GR != 0.0 {
                    let SL = if SJ != 0.0 {
                        SK
                    } else {
                        GT
                    };
                    oSL = SL;
                } else {
                }
                if HJ != 0.0 {
                    if SM != 0.0 {
                        let SN = (IM - HM) + OT;
                        oSN = SN;
                    } else {
                    }
                } else {
                }
                if SO != 0.0 {
                } else {
                    let SP = staged[202] * HW;
                    oSP = SP;
                }
                if SQ != 0.0 {
                } else {
                    let SS = (LN * HW) * SR;
                    oSS = SS;
                }
                if SQ != 0.0 {
                } else {
                    let ST = (LN * HW) * SR;
                    oST = ST;
                }
                if SU != 0.0 {
                    let SV = if HW == AB { 1.0 } else { 0.0 };
                    oSV = SV;
                } else {
                }
                if SW != 0.0 {
                    if SY != 0.0 {
                    } else {
                        let SZ = LN * HW;
                        oSZ = SZ;
                        let TA = if HW == AB { 1.0 } else { 0.0 };
                        oTA = TA;
                        let TB = staged[357] * HW;
                        oTB = TB;
                        if TC != 0.0 {
                            let TE = TD * HW;
                            oTE = TE;
                        } else {
                        }
                    }
                } else {
                    if RN != 0.0 {
                        let TH = if C != 0.0 {
                            let TF = 3.453133e-11f64 / KU;
                            TF
                        } else {
                            let TG = staged[369] / KU;
                            TG
                        };
                        oTH = TH;
                        let TI = staged[370] / KU;
                        oTI = TI;
                        let TJ = staged[371] / KU;
                        oTJ = TJ;
                        let TK = 1e8f64 * KU;
                        oTK = TK;
                        let TO;
                        let TP;
                        if SX != 0.0 {
                            let TL = staged[372] / KU;
                            let TM = staged[373] / KU;
                            TO = TM;
                            TP = TL;
                        } else {
                            TO = TD;
                            TP = TN;
                        }
                        oTO = TO;
                        oTP = TP;
                        if TQ != 0.0 {
                        } else {
                            if SI != 0.0 {
                            } else {
                                let TS = OS + staged[352];
                                oTS = TS;
                            }
                            let TT = SB * KU;
                            oTT = TT;
                            let TU = (NG * TT) * staged[376];
                            oTU = TU;
                            let TV = LN * HW;
                            oTV = TV;
                            let TW = if HW == AB { 1.0 } else { 0.0 };
                            oTW = TW;
                        }
                        let TR = if HW <= AB { 1.0 } else { 0.0 };
                        oTR = TR;
                        let UA = if TR != 0.0 {
                            let TX = 2.5e-1f64 * staged[383];
                            oTX = TX;
                            let TY = LN * FS;
                            TY
                        } else {
                            let TZ = HW * FS;
                            TZ
                        };
                        let UB = H * UA;
                        oUB = UB;
                        let UC = TK + TK;
                        oUC = UC;
                    } else {
                    }
                }
                let UD = if FL != AB { 1.0 } else { 0.0 };
                if UD != 0.0 {
                    let UE = if (if DO != 0.0 && (if GO > AB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if IX < AB { 1.0 } else { 0.0 }) != 0.0 && (if GO < AB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    oUE = UE;
                } else {
                }
            [A, R, S, T, oAC, oAI, oAP, oAR, M, BA, oBD, BG, oBL, BO, oBS, BU, BV, BW, BX, BZ, oCD, CF, CI, oCM, CP, oCT, CW, oDA, DC, DD, DE, DF, DH, oDL, DN, oDV, oEJ, EH, EE, FN, FR, FS, FT, FW, GA, GB, oGD, oGM, oHA, HW, HZ, IB, GU, IF, IJ, IK, IM, IN, IO, IP, oIY, FA, oLJ, oLU, oMC, oMM, oND, oNE, oNL, oNI, oNW, oNZ, KX, OL, OQ, OY, oPC, oPK, oPQ, oPS, PY, RB, RF, RO, oSC, oSG, oSH, RZ, SD, AD, AE, AF, AG, DW, O, IH, II, GV, oSL, oSN, oSP, oSS, oST, OZ, PD, GE, oSV, oSZ, oTA, oTB, oTE, oTI, oTJ, oTK, oTS, oTT, oTU, oTH, oTO, oTV, oTW, oTR, oTX, oUB, oUC, oTP, UD, oUE, IZ, JA, JB, JC, JD, JE, JF]
        };
        self.canonical_staged[140] = produced[0];
        self.canonical_staged[732] = produced[1];
        self.canonical_staged[734] = produced[2];
        self.canonical_staged[733] = produced[3];
        self.canonical_staged[465] = produced[4];
        self.canonical_staged[467] = produced[5];
        self.canonical_staged[468] = produced[6];
        self.canonical_staged[469] = produced[7];
        self.canonical_staged[725] = produced[8];
        self.canonical_staged[479] = produced[9];
        self.canonical_staged[481] = produced[10];
        self.canonical_staged[482] = produced[11];
        self.canonical_staged[483] = produced[12];
        self.canonical_staged[484] = produced[13];
        self.canonical_staged[485] = produced[14];
        self.canonical_staged[744] = produced[15];
        self.canonical_staged[742] = produced[16];
        self.canonical_staged[738] = produced[17];
        self.canonical_staged[740] = produced[18];
        self.canonical_staged[486] = produced[19];
        self.canonical_staged[487] = produced[20];
        self.canonical_staged[746] = produced[21];
        self.canonical_staged[488] = produced[22];
        self.canonical_staged[489] = produced[23];
        self.canonical_staged[490] = produced[24];
        self.canonical_staged[491] = produced[25];
        self.canonical_staged[492] = produced[26];
        self.canonical_staged[493] = produced[27];
        self.canonical_staged[745] = produced[28];
        self.canonical_staged[743] = produced[29];
        self.canonical_staged[739] = produced[30];
        self.canonical_staged[741] = produced[31];
        self.canonical_staged[494] = produced[32];
        self.canonical_staged[495] = produced[33];
        self.canonical_staged[747] = produced[34];
        self.canonical_staged[497] = produced[35];
        self.canonical_staged[501] = produced[36];
        self.canonical_staged[500] = produced[37];
        self.canonical_staged[403] = produced[38];
        self.canonical_staged[508] = produced[39];
        self.canonical_staged[722] = produced[40];
        self.canonical_staged[723] = produced[41];
        self.canonical_staged[727] = produced[42];
        self.canonical_staged[509] = produced[43];
        self.canonical_staged[726] = produced[44];
        self.canonical_staged[729] = produced[45];
        self.canonical_staged[511] = produced[46];
        self.canonical_staged[515] = produced[47];
        self.canonical_staged[525] = produced[48];
        self.canonical_staged[196] = produced[49];
        self.canonical_staged[728] = produced[50];
        self.canonical_staged[737] = produced[51];
        self.canonical_staged[160] = produced[52];
        self.canonical_staged[159] = produced[53];
        self.canonical_staged[198] = produced[54];
        self.canonical_staged[767] = produced[55];
        self.canonical_staged[766] = produced[56];
        self.canonical_staged[407] = produced[57];
        self.canonical_staged[411] = produced[58];
        self.canonical_staged[538] = produced[59];
        self.canonical_staged[541] = produced[60];
        self.canonical_staged[406] = produced[61];
        self.canonical_staged[554] = produced[62];
        self.canonical_staged[555] = produced[63];
        self.canonical_staged[556] = produced[64];
        self.canonical_staged[559] = produced[65];
        self.canonical_staged[560] = produced[66];
        self.canonical_staged[561] = produced[67];
        self.canonical_staged[563] = produced[68];
        self.canonical_staged[562] = produced[69];
        self.canonical_staged[564] = produced[70];
        self.canonical_staged[565] = produced[71];
        self.canonical_staged[553] = produced[72];
        self.canonical_staged[566] = produced[73];
        self.canonical_staged[189] = produced[74];
        self.canonical_staged[571] = produced[75];
        self.canonical_staged[573] = produced[76];
        self.canonical_staged[575] = produced[77];
        self.canonical_staged[576] = produced[78];
        self.canonical_staged[577] = produced[79];
        self.canonical_staged[587] = produced[80];
        self.canonical_staged[602] = produced[81];
        self.canonical_staged[604] = produced[82];
        self.canonical_staged[619] = produced[83];
        self.canonical_staged[630] = produced[84];
        self.canonical_staged[652] = produced[85];
        self.canonical_staged[668] = produced[86];
        self.canonical_staged[632] = produced[87];
        self.canonical_staged[730] = produced[88];
        self.canonical_staged[750] = produced[89];
        self.canonical_staged[748] = produced[90];
        self.canonical_staged[751] = produced[91];
        self.canonical_staged[749] = produced[92];
        self.canonical_staged[724] = produced[93];
        self.canonical_staged[731] = produced[94];
        self.canonical_staged[735] = produced[95];
        self.canonical_staged[736] = produced[96];
        self.canonical_staged[761] = produced[97];
        self.canonical_staged[760] = produced[98];
        self.canonical_staged[162] = produced[99];
        self.canonical_staged[203] = produced[100];
        self.canonical_staged[247] = produced[101];
        self.canonical_staged[252] = produced[102];
        self.canonical_staged[257] = produced[103];
        self.canonical_staged[263] = produced[104];
        self.canonical_staged[281] = produced[105];
        self.canonical_staged[803] = produced[106];
        self.canonical_staged[359] = produced[107];
        self.canonical_staged[823] = produced[108];
        self.canonical_staged[360] = produced[109];
        self.canonical_staged[361] = produced[110];
        self.canonical_staged[389] = produced[111];
        self.canonical_staged[380] = produced[112];
        self.canonical_staged[374] = produced[113];
        self.canonical_staged[833] = produced[114];
        self.canonical_staged[377] = produced[115];
        self.canonical_staged[378] = produced[116];
        self.canonical_staged[379] = produced[117];
        self.canonical_staged[381] = produced[118];
        self.canonical_staged[382] = produced[119];
        self.canonical_staged[836] = produced[120];
        self.canonical_staged[832] = produced[121];
        self.canonical_staged[384] = produced[122];
        self.canonical_staged[385] = produced[123];
        self.canonical_staged[386] = produced[124];
        self.canonical_staged[390] = produced[125];
        self.canonical_staged[845] = produced[126];
        self.canonical_staged[847] = produced[127];
        self.canonical_staged[404] = produced[128];
        self.canonical_staged[405] = produced[129];
        self.canonical_staged[408] = produced[130];
        self.canonical_staged[409] = produced[131];
        self.canonical_staged[410] = produced[132];
        self.canonical_staged[412] = produced[133];
        self.canonical_staged[413] = produced[134];
        self.canonical_temperature = temperature;
        self.canonical_thermal_voltage = thermal_voltage;
        self.canonical_temperature_valid = true;
    }

    fn canonical_timestep_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {
        let produced: [f64; 1] = {
            let multiplicity = self.multiplicity;
            let staged = &*self.canonical_staged;
                let A = staged[452];
                let B = staged[536];
                let C = staged[540];
                let D = staged[564];
                if B != 0.0 {
                    loop {
                        if C == 0.0 {
                            break;
                        }
                    }
                } else {
                }
                if A != 0.0 {
                } else {
                    loop {
                        if D == 0.0 {
                            break;
                        }
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
            let slot = match operator { 45451 => 0usize, 45453 => 1usize, 45456 => 2usize, 45460 => 3usize, 45464 => 4usize, 45468 => 5usize, 45475 => 6usize, 45479 => 7usize, 45484 => 8usize, 45487 => 9usize, 45491 => 10usize, 45496 => 11usize, 45498 => 12usize, 45500 => 13usize, 45590 => 14usize, _ => usize::MAX };
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
            let A = parameters[39];
            let B = staged[452];
            let C = staged[464];
            let D = staged[99];
            let E = staged[517];
            let F = staged[518];
            let G = staged[521];
            let H = staged[529];
            let I = staged[532];
            let J = staged[536];
            let K = staged[540];
            let L = staged[564];
            let M = parameters[38];
            let N = staged[572];
            let O = staged[232];
            let P = staged[721];
            let Q = node_potentials[6];
            let R = 1e0f64;
            let S = 0e0f64;
            let T = 0e0f64;
            let X = staged[0];
            let AA = 1e0f64;
            let AC = staged[722];
            let AD = staged[723];
            let AE = staged[724];
            let AF = staged[725];
            let AG = staged[726];
            let AH = staged[727];
            let AI = staged[728];
            let AJ = staged[729];
            let AK = staged[730];
            let AL = staged[731];
            let AM = staged[732];
            let AN = staged[733];
            let AO = staged[734];
            let AP = staged[735];
            let AQ = staged[736];
            let AR = staged[737];
            let AS = staged[738];
            let AT = staged[739];
            let AU = staged[740];
            let AV = staged[741];
            let AW = staged[742];
            let AX = staged[743];
            let AY = staged[744];
            let AZ = staged[745];
            let BA = staged[746];
            let BB = staged[747];
            let BC = staged[748];
            let BD = staged[749];
            let BE = staged[750];
            let BF = staged[751];
            let DO = 8.617087e-5f64;
            let DT = 7.02e-4f64;
            let DW = -1e0f64;
            let DZ = 2e0f64;
            let EA = 1e0f64;
            let EB = 1.45e10f64;
            let ED = 1.9230584e-4f64;
            let EG = 2e0f64;
            let EO = parameters[48];
            let EV = parameters[46];
            let EX = staged[141];
            let FG = staged[68];
            let FJ = 1e-38f64;
            let FV = 3.720075976020836e-44f64;
            let GH = -8.749823353377374e1f64;
            let GO = -8.749823353377374e1f64;
            let HB = staged[62];
            let HF = staged[144];
            let HG = staged[145];
            let HL = -8.749823353377374e1f64;
            let HO = staged[147];
            let HU = -8.749823353377374e1f64;
            let IB = staged[64];
            let IG = staged[149];
            let IT = staged[89];
            let IZ = staged[25];
            let JC = staged[26];
            let JF = 1e2f64;
            let JH = 2.688117142e43f64;
            let JN = staged[753];
            let JO = 3.720075976e-44f64;
            let JT = staged[27];
            let JZ = staged[28];
            let KA = staged[29];
            let KS = staged[30];
            let KV = staged[31];
            let KY = staged[32];
            let LB = staged[33];
            let LE = staged[34];
            let LR = staged[35];
            let LU = staged[36];
            let MH = staged[754];
            let MM = staged[37];
            let MS = staged[38];
            let MT = staged[39];
            let NL = staged[40];
            let NO = staged[41];
            let NR = staged[42];
            let NU = staged[43];
            let NX = staged[44];
            let OK = staged[45];
            let ON = staged[10];
            let OO = staged[11];
            let OR = staged[755];
            let OW = parameters[225];
            let OX = staged[92];
            let OZ = 1e-9f64;
            let PN = staged[12];
            let PP = staged[154];
            let PU = staged[756];
            let PV = staged[14];
            let PW = staged[16];
            let QA = staged[156];
            let QQ = staged[4];
            let QT = staged[6];
            let QW = staged[8];
            let QZ = staged[758];
            let RC = staged[760];
            let RD = staged[761];
            let RM = staged[158];
            let RR = staged[55];
            let RS = staged[80];
            let RU = staged[762];
            let RY = staged[763];
            let SK = 1e-8f64;
            let SL = staged[764];
            let SQ = staged[765];
            let SR = staged[766];
            let SY = parameters[34];
            let TB = staged[767];
            let TE = staged[768];
            let TT = node_potentials[7];
            let TU = node_potentials[8];
            let TV = 1e0f64;
            let TW = 1e0f64;
            let TZ = node_potentials[5];
            let UA = 1e0f64;
            let UD = node_potentials[9];
            let UE = 1e0f64;
            let UH = node_potentials[3];
            let UI = 1e0f64;
            let UL = node_potentials[4];
            let UM = 1e0f64;
            let UR = node_potentials[11];
            let US = 1e0f64;
            let UV = node_potentials[12];
            let UW = 1e0f64;
            let UZ = node_potentials[10];
            let VA = 1e0f64;
            let VU = staged[769];
            let VV = staged[770];
            let VW = staged[771];
            let VX = staged[772];
            let VY = staged[773];
            let VZ = staged[774];
            let WA = staged[775];
            let WB = staged[776];
            let WC = staged[777];
            let WD = staged[778];
            let WE = staged[779];
            let WF = staged[780];
            let WG = staged[781];
            let WH = staged[782];
            let WI = staged[783];
            let WJ = staged[784];
            let WM = -1e0f64;
            let XU = staged[163];
            let XV = staged[164];
            let XX = staged[165];
            let XY = staged[111];
            let YE = 5e-1f64;
            let YG = parameters[986];
            let YI = 5e-2f64;
            let YL = 2.24e-1f64;
            let ZO = staged[785];
            let ZQ = staged[786];
            let ZX = 5e0f64;
            let ZY = 1e-3f64;
            let AAC = 1.5e0f64;
            let AAE = 2e-3f64;
            let AAK = 9.5e-1f64;
            let AAR = 8e-3f64;
            let ABR = 1.60219e-19f64;
            let ABV = staged[178];
            let ABZ = staged[166];
            let ACA = staged[168];
            let ACD = staged[169];
            let ACE = staged[170];
            let ACI = staged[171];
            let ACJ = staged[172];
            let ACK = staged[174];
            let ACL = staged[175];
            let ACP = staged[176];
            let ACX = 5e-3f64;
            let ADA = 2.5e-5f64;
            let ADE = staged[24];
            let ADF = staged[177];
            let ADI = 2e-2f64;
            let AEB = 8e0f64;
            let AEC = 3e0f64;
            let AEK = staged[86];
            let AEP = staged[179];
            let AFL = 3.720075976e-44f64;
            let AFM = Lanes([0e0f64; 4]);
            let AFQ = staged[182];
            let AFR = staged[115];
            let AFS = staged[183];
            let AFV = staged[116];
            let AGH = staged[787];
            let AGI = staged[184];
            let AGO = staged[118];
            let AGV = Lanes([0e0f64; 2]);
            let AHA = staged[185];
            let AHB = staged[186];
            let AHI = -8.749823353377374e1f64;
            let AHV = 3.720075976e-44f64;
            let AHY = staged[120];
            let AIC = staged[188];
            let AID = staged[135];
            let AIF = staged[189];
            let AII = staged[66];
            let AIJ = staged[133];
            let AIM = staged[190];
            let AIO = staged[191];
            let AIQ = 1e-4f64;
            let AIS = 2e4f64;
            let AIV = 2e-4f64;
            let AJF = staged[192];
            let AJH = staged[193];
            let AJT = staged[194];
            let AJW = staged[195];
            let AKA = staged[196];
            let AKB = staged[197];
            let AKC = staged[198];
            let AKE = staged[199];
            let AKF = staged[126];
            let AKQ = staged[200];
            let AKT = staged[201];
            let ALJ = Lanes([0e0f64; 5]);
            let ALX = staged[203];
            let AMQ = -8.749823353377374e1f64;
            let AMU = staged[204];
            let AMX = staged[205];
            let ANA = staged[207];
            let ANC = staged[208];
            let ANF = staged[209];
            let ANG = staged[211];
            let ANH = staged[212];
            let ANL = staged[213];
            let ANT = staged[788];
            let AOA = 1e-2f64;
            let AQI = -8.749823353377374e1f64;
            let AQN = staged[214];
            let AQQ = staged[216];
            let AQS = staged[217];
            let AQV = staged[218];
            let AQW = staged[220];
            let AQX = staged[221];
            let ARB = staged[222];
            let ATO = 3.720075976e-44f64;
            let ATP = Lanes([0e0f64; 6]);
            let ATS = staged[224];
            let AUJ = staged[789];
            let AUK = staged[225];
            let AVG = -8.749823353377374e1f64;
            let AVT = 3.720075976e-44f64;
            let AWW = staged[227];
            let AXI = staged[228];
            let AXR = staged[23];
            let AZF = 3.720075976e-44f64;
            let AZV = staged[230];
            let BAR = -8.749823353377374e1f64;
            let BBE = 3.720075976e-44f64;
            let BCE = staged[790];
            let BCS = staged[127];
            let BCV = staged[235];
            let BCW = staged[129];
            let BDF = 3.720075976e-44f64;
            let BDT = 3.720075976e-44f64;
            let BEE = staged[791];
            let BFE = staged[237];
            let BFF = staged[238];
            let BFG = staged[239];
            let BFJ = 2e-8f64;
            let BFS = staged[241];
            let BFT = staged[242];
            let BFZ = staged[792];
            let BGD = 2e1f64;
            let BGE = 1.7e1f64;
            let BGH = 8e-1f64;
            let BGO = staged[243];
            let BGP = staged[244];
            let BGT = staged[793];
            let BGU = staged[245];
            let BHH = -4e0f64;
            let BHN = staged[246];
            let BHV = 1.414213562373095e0f64;
            let BHX = 7.071067811865475e-1f64;
            let BIG = staged[248];
            let BIL = staged[249];
            let BIR = staged[251];
            let BIX = 2e2f64;
            let BJT = -4e0f64;
            let BKB = 7.071067811865475e-1f64;
            let BKG = staged[255];
            let BKL = staged[794];
            let BKM = staged[256];
            let BKS = staged[795];
            let BLC = staged[796];
            let BLM = 6e0f64;
            let BLU = -8.749823353377374e1f64;
            let BLX = staged[258];
            let BMC = staged[259];
            let BMD = staged[260];
            let BMG = staged[261];
            let BMH = staged[262];
            let BMK = staged[263];
            let BMQ = -8.749823353377374e1f64;
            let BMY = 1e1f64;
            let BNO = staged[797];
            let BNP = staged[268];
            let BNQ = staged[798];
            let BNV = staged[264];
            let BPN = staged[271];
            let BQG = staged[273];
            let BQH = staged[274];
            let BQY = staged[275];
            let BRE = staged[276];
            let BRY = staged[799];
            let BRZ = staged[277];
            let BSA = staged[278];
            let BTR = parameters[25];
            let BTX = staged[800];
            let BTY = Lanes([0e0f64; 5]);
            let BTZ = Lanes([0e0f64; 3]);
            let BUA = Lanes([0e0f64; 3]);
            let BUB = Lanes([0e0f64; 5]);
            let BUQ = staged[801];
            let BUR = staged[802];
            let BVC = staged[280];
            let BVG = staged[281];
            let BWB = 0e0f64;
            let BWH = 1e-6f64;
            let CAA = Lanes([0e0f64; 3]);
            let CBI = staged[282];
            let CBQ = staged[283];
            let CBY = parameters[995];
            let CCA = staged[284];
            let CCD = staged[286];
            let CCR = staged[287];
            let CDA = 1e3f64;
            let CFD = staged[289];
            let CHB = 1e-5f64;
            let CIG = staged[290];
            let CII = staged[291];
            let CIL = staged[292];
            let CIY = staged[293];
            let CJI = staged[294];
            let CJN = 4e0f64;
            let CKE = staged[296];
            let CLB = staged[298];
            let CLP = Lanes([0e0f64; 2]);
            let CNC = Lanes([0e0f64; 2]);
            let COL = parameters[363];
            let CON = 8e-2f64;
            let COR = 8e-2f64;
            let CPE = staged[803];
            let CPV = staged[299];
            let CQD = Lanes([0e0f64; 2]);
            let CQE = Lanes([0e0f64; 3]);
            let CQO = staged[804];
            let CQU = staged[300];
            let CQV = staged[302];
            let CQX = staged[303];
            let CRB = 0e0f64;
            let CRO = staged[304];
            let CRT = staged[305];
            let CSW = staged[306];
            let CSX = staged[307];
            let CSY = staged[308];
            let CTA = staged[309];
            let CTL = staged[310];
            let CUG = staged[311];
            let CUO = parameters[381];
            let CUQ = parameters[382];
            let CUT = staged[313];
            let CUX = parameters[370];
            let CVQ = staged[805];
            let CVV = parameters[373];
            let CWF = parameters[987];
            let CWG = staged[315];
            let CWJ = staged[316];
            let CWK = staged[318];
            let CXD = parameters[374];
            let CXT = staged[806];
            let CXY = parameters[377];
            let CYG = parameters[989];
            let CYJ = staged[319];
            let CYK = staged[321];
            let CZE = parameters[985];
            let CZM = Lanes([0e0f64; 3]);
            let CZS = parameters[991];
            let CZT = parameters[992];
            let CZV = parameters[993];
            let CZW = parameters[994];
            let DAB = staged[323];
            let DAC = staged[324];
            let DAD = staged[325];
            let DAW = staged[807];
            let DAX = Lanes([0e0f64; 8]);
            let DAY = Lanes([0e0f64; 2]);
            let DBD = staged[808];
            let DBE = staged[809];
            let DBF = staged[810];
            let DBI = 1.0f64;
            let DBJ = parameters[295];
            let DBK = staged[326];
            let DBL = staged[328];
            let DBO = staged[329];
            let DBQ = staged[330];
            let DBT = staged[331];
            let DBY = staged[332];
            let DBZ = staged[333];
            let DCA = staged[334];
            let DCK = staged[335];
            let DDK = staged[338];
            let DDV = parameters[307];
            let DDW = staged[339];
            let DFD = staged[341];
            let DFW = staged[811];
            let DFZ = staged[344];
            let DGE = staged[343];
            let DGH = staged[345];
            let DGJ = staged[346];
            let DGM = staged[813];
            let DGP = parameters[3];
            let DGU = staged[814];
            let DGV = staged[347];
            let DHF = staged[348];
            let DII = Lanes([0e0f64; 4]);
            let DIR = staged[815];
            let DKU = staged[350];
            let DKY = staged[351];
            let DLC = staged[816];
            let DLE = staged[817];
            let DLJ = staged[818];
            let DLN = staged[352];
            let DMA = -8.749823353377374e1f64;
            let DMF = staged[819];
            let DMS = -8.749823353377374e1f64;
            let DMZ = staged[353];
            let DND = staged[354];
            let DNE = staged[355];
            let DOB = -8.749823353377374e1f64;
            let DOS = -8.749823353377374e1f64;
            let DPN = -8.749823353377374e1f64;
            let DPR = staged[356];
            let DQW = -8.749823353377374e1f64;
            let DRG = staged[820];
            let DRP = staged[821];
            let DRT = 8e-2f64;
            let DSA = staged[362];
            let DSI = 8e-2f64;
            let DSN = 3.2e-1f64;
            let DSR = 3.2e-1f64;
            let DSY = staged[357];
            let DTB = staged[822];
            let DTM = staged[823];
            let DTO = 8e0f64;
            let DTS = 8e0f64;
            let DTZ = staged[358];
            let DUF = staged[359];
            let DUG = staged[360];
            let DUJ = staged[824];
            let DVB = staged[361];
            let DVJ = 8e-2f64;
            let DVR = 1e-20f64;
            let DVS = 1.2e1f64;
            let DWB = staged[825];
            let DWL = staged[363];
            let DWO = staged[826];
            let DXG = staged[364];
            let DXP = staged[827];
            let DXR = 2.5e-1f64;
            let DXU = staged[365];
            let DXX = staged[828];
            let DXY = staged[829];
            let DYR = 1.5e1f64;
            let DYW = staged[830];
            let DYX = -5e-1f64;
            let DZR = staged[368];
            let EAI = staged[831];
            let EAT = staged[832];
            let EAW = staged[833];
            let EBF = 8e-2f64;
            let EBJ = 8e-2f64;
            let ECA = staged[374];
            let ECB = staged[375];
            let ECG = 2e0f64;
            let ECK = 2e0f64;
            let ECS = staged[376];
            let ECZ = staged[377];
            let EDC = staged[378];
            let EDG = 1e-15f64;
            let EDT = staged[379];
            let EDZ = staged[834];
            let EFA = staged[380];
            let EFD = staged[381];
            let EFL = staged[835];
            let EFT = staged[836];
            let EFY = staged[382];
            let EGC = staged[837];
            let EHA = staged[384];
            let EHD = staged[383];
            let EHI = staged[385];
            let EHQ = -8.749823353377374e1f64;
            let EIG = staged[386];
            let EIM = -8.749823353377374e1f64;
            let EIT = -8.749823353377374e1f64;
            let EIW = staged[387];
            let EIZ = staged[388];
            let EJH = staged[389];
            let EJM = staged[838];
            let EKH = 8e-2f64;
            let EKW = staged[839];
            let EKZ = -8.749823353377374e1f64;
            let ELL = staged[390];
            let ELX = 8e-2f64;
            let ENB = staged[840];
            let ENE = staged[841];
            let ENR = staged[842];
            let ENS = staged[843];
            let EOQ = staged[844];
            let EOR = -5e-1f64;
            let EPL = staged[391];
            let EPV = staged[392];
            let EPY = staged[394];
            let EQB = staged[396];
            let EQE = 9e-1f64;
            let EQM = staged[402];
            let EQR = staged[845];
            let EQZ = 0.0f64;
            let ERL = -8.749823353377374e1f64;
            let ERO = -0e0f64;
            let ERW = parameters[338];
            let ERZ = staged[398];
            let ESM = staged[846];
            let ESU = staged[401];
            let ESZ = -8.749823353377374e1f64;
            let ETC = staged[400];
            let ETM = staged[847];
            let ETN = staged[101];
            let ETQ = staged[102];
            let ETX = staged[414];
            let EUA = staged[415];
            let EUD = staged[848];
            let EUE = staged[403];
            let EUG = staged[406];
            let EUM = staged[404];
            let EUS = staged[405];
            let EVD = staged[407];
            let EVE = staged[408];
            let EVF = staged[409];
            let EWY = staged[410];
            let EXJ = staged[411];
            let EXK = staged[412];
            let EXL = staged[413];
            let EZD = staged[416];
            let EZG = staged[417];
            let EZI = staged[418];
            let EZJ = staged[419];
            let EZM = staged[420];
            let EZO = staged[421];
            let FAF = staged[422];
            let FAH = staged[423];
            let FAI = staged[424];
            let FAL = staged[425];
            let FAN = staged[426];
            let FBG = staged[849];
            let FBH = staged[850];
            let FBI = staged[851];
            let FBQ = staged[856];
            let FBR = staged[857];
            let FBS = staged[858];
            let FCG = -8.749823353377374e1f64;
            let FCK = parameters[209];
            let FCO = parameters[210];
            let FCS = 1e0f64;
            let FCX = 1e0f64;
            let FDC = Lanes([0e0f64; 6]);
            let FDD = Lanes([0e0f64; 5]);
            let FFU = 1.0f64;
            let FGA = ddt_scale();
            let FHC = staged[432];
            let FHH = Lanes([0e0f64; 4]);
            let FHI = Lanes([0e0f64; 3]);
            let FHJ = Lanes([0e0f64; 2]);
            let FHW = Lanes([0e0f64; 2]);
            let FIZ = staged[859];
            let FJA = Lanes([0e0f64; 2]);
            let FJB = 1e0f64;
            let FJG = staged[860];
            let FJH = Lanes([0e0f64; 7]);
            let FJP = staged[433];
            let FJS = staged[434];
            let FJV = Lanes([0e0f64; 2]);
            let FJW = Lanes([0e0f64; 2]);
            let FKD = staged[435];
            let FKG = staged[436];
            let FSW = 0e0f64;
            let FSX = 0e0f64;
            let FSY = 0e0f64;
            let FSZ = 0e0f64;
            let FTA = 0e0f64;
            let FTB = 0e0f64;
            if J != 0.0 {
                loop {
                    if K == 0.0 {
                        break;
                    }
                }
            } else {
            }
            if B != 0.0 {
            } else {
                loop {
                    if L == 0.0 {
                        break;
                    }
                }
            }
            let U;
            let V;
            if P != 0.0 {
                U = Q;
                V = R;
            } else {
                U = S;
                V = T;
            }
            let W = U + staged[140];
            let Y = W / X;
            let Z = V / X;
            let AB = Y - AA;
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
            let DG;
            let DH;
            let DI;
            let DJ;
            let DK;
            let DL;
            let DM;
            let DN;
            if P != 0.0 {
                let FL;
                let FM;
                let FN;
                let FO;
                let FP;
                let FQ;
                let FR;
                let FS;
                if B != 0.0 {
                    let DP = DO * W;
                    let DQ = V * DO;
                    let DR = 1.108e3f64 + W;
                    let DS = V * W;
                    let DU = (DT * (W * W)) / DR;
                    let DV = 1.16e0f64 - DU;
                    let DX = ((((DS + DS) * DT) - (V * DU)) / DR) * DW;
                    let DY = W.sqrt();
                    let EC = EB * W;
                    let EE = (EC * DY) * ED;
                    let EF = (((V * EB) * DY) + ((V * (EA / (DZ * DY))) * EC)) * ED;
                    let EH = EG * DP;
                    let EI = DV / EH;
                    let EJ = 2.15565981e1f64 - EI;
                    let EK = ((DX - ((DQ * EG) * EI)) / EH) * DW;
                    let EL = if EJ > -1e2f64 { 1.0 } else { 0.0 };
                    let FW;
                    let FX;
                    if EL != 0.0 {
                        let FT = EJ.exp();
                        let FU = EK * FT;
                        FW = FT;
                        FX = FU;
                    } else {
                        FW = FV;
                        FX = T;
                    }
                    let FY = EE * FW;
                    let FZ = (EF * FW) + (FX * EE);
                    let GA = FY * FY;
                    let GB = FZ * FY;
                    let GC = FG / GA;
                    let GD = (((GB + GB) * GC) * DW) / GA;
                    let GE = if GC > FJ { 1.0 } else { 0.0 };
                    let GI;
                    let GJ;
                    if GE != 0.0 {
                        let GF = GC.ln();
                        let GG = GD * (EA / GC);
                        GI = GF;
                        GJ = GG;
                    } else {
                        GI = GH;
                        GJ = T;
                    }
                    let GK = DP * GI;
                    let GL = (DQ * GI) + (GJ * DP);
                    FL = DP;
                    FM = FY;
                    FN = GK;
                    FO = DV;
                    FP = DQ;
                    FQ = FZ;
                    FR = GL;
                    FS = DX;
                } else {
                    let EM = DO * W;
                    let EN = V * DO;
                    let EP = EO * W;
                    let EQ = W + parameters[49];
                    let ER = (EP * W) / EQ;
                    let ES = parameters[47] - ER;
                    let ET = (((((V * EO) * W) + (V * EP)) - (V * ER)) / EQ) * DW;
                    let EU = W.sqrt();
                    let EW = EV * W;
                    let EY = (EW * EU) * EX;
                    let EZ = EG * EM;
                    let FA = ES / EZ;
                    let FB = (staged[143] - FA).exp();
                    let FC = EY * FB;
                    let FD = (((((V * EV) * EU) + ((V * (EA / (DZ * EU))) * EW)) * EX) * FB) + (((((ET - ((EN * EG) * FA)) / EZ) * DW) * FB) * EY);
                    let FE = FC * FC;
                    let FF = FD * FC;
                    let FH = FG / FE;
                    let FI = (((FF + FF) * FH) * DW) / FE;
                    let FK = if FH > FJ { 1.0 } else { 0.0 };
                    let GP;
                    let GQ;
                    if FK != 0.0 {
                        let GM = FH.ln();
                        let GN = FI * (EA / FH);
                        GP = GM;
                        GQ = GN;
                    } else {
                        GP = GO;
                        GQ = T;
                    }
                    let GR = EM * GP;
                    let GS = (EN * GP) + (GQ * EM);
                    FL = EM;
                    FM = FC;
                    FN = GR;
                    FO = ES;
                    FP = EN;
                    FQ = FD;
                    FR = GS;
                    FS = ET;
                }
                let GX;
                let GY;
                if D != 0.0 {
                    let HH = (HF * FL) * HG;
                    let HI = (FP * HF) * HG;
                    GX = HH;
                    GY = HI;
                } else {
                    let GT = staged[146] / FM;
                    let GU = GT / FM;
                    let GV = ((((FQ * GT) * DW) / FM) - (FQ * GU)) / FM;
                    let GW = if GU > FJ { 1.0 } else { 0.0 };
                    let HM;
                    let HN;
                    if GW != 0.0 {
                        let HJ = GU.ln();
                        let HK = GV * (EA / GU);
                        HM = HJ;
                        HN = HK;
                    } else {
                        HM = HL;
                        HN = T;
                    }
                    let HP = HO * FL;
                    let HQ = HP * HM;
                    let HR = ((FP * HO) * HM) + (HN * HP);
                    GX = HQ;
                    GY = HR;
                }
                let GZ = EG * FL;
                let HA = FP * EG;
                let HC = HB / FM;
                let HD = ((FQ * HC) * DW) / FM;
                let HE = if HC > FJ { 1.0 } else { 0.0 };
                let HV;
                let HW;
                if HE != 0.0 {
                    let HS = HC.ln();
                    let HT = HD * (EA / HC);
                    HV = HS;
                    HW = HT;
                } else {
                    HV = HU;
                    HW = T;
                }
                let HX = GZ * HV;
                let HY = (HA * HV) + (HW * GZ);
                let HZ = HX.sqrt();
                let IA = HY * (EA / (DZ * HZ));
                let IC = IB * HZ;
                let ID = IA * IB;
                let IE = staged[148] / HZ;
                let IF = ((IA * IE) * DW) / HZ;
                let IH = (IG * IC).sqrt();
                let II = (ID * IG) * (EA / (DZ * IH));
                let IJ = staged[150] / IH;
                let IK = IJ.exp();
                let IL = (((II * IJ) * DW) / IH) * IK;
                let IM = EG * IK;
                let IN = IK + (IM * IK);
                let IO = IL + (((IL * EG) * IK) + (IL * IM));
                let IP = staged[151] / IH;
                let IQ = IP.exp();
                let IR = (((II * IP) * DW) / IH) * IQ;
                let IS = EG * IQ;
                let IU = (IR + (((IR * EG) * IQ) + (IR * IS))) * IT;
                let IV = (IT * (IQ + (IS * IQ))) + staged[90];
                let IW = 1.115e0f64 / FL;
                let IX = IW * AB;
                let IY = ((((FP * IW) * DW) / FL) * AB) + (Z * IW);
                let JA = IZ * IX;
                let JB = IY * IZ;
                let JD = JA / JC;
                let JE = JB / JC;
                let JG = if JD > JF { 1.0 } else { 0.0 };
                let JL;
                let JM;
                if JG != 0.0 {
                    let JI = JH * ((AA + JD) - JF);
                    let JJ = JE * JH;
                    JL = JI;
                    JM = JJ;
                } else {
                    let JK = if JD < -1e2f64 { 1.0 } else { 0.0 };
                    let JR;
                    let JS;
                    if JK != 0.0 {
                        JR = JO;
                        JS = T;
                    } else {
                        let JP = JD.exp();
                        let JQ = JE * JP;
                        JR = JP;
                        JS = JQ;
                    }
                    JL = JR;
                    JM = JS;
                }
                let JX;
                let JY;
                if JN != 0.0 {
                    JX = JL;
                    JY = JM;
                } else {
                    let JU = (JT * IX) / JC;
                    let JV = (IY * JT) / JC;
                    let JW = if JU > JF { 1.0 } else { 0.0 };
                    let KH;
                    let KI;
                    if JW != 0.0 {
                        let KE = JH * ((AA + JU) - JF);
                        let KF = JV * JH;
                        KH = KE;
                        KI = KF;
                    } else {
                        let KG = if JU < -1e2f64 { 1.0 } else { 0.0 };
                        let KL;
                        let KM;
                        if KG != 0.0 {
                            KL = JO;
                            KM = T;
                        } else {
                            let KJ = JU.exp();
                            let KK = JV * KJ;
                            KL = KJ;
                            KM = KK;
                        }
                        KH = KL;
                        KI = KM;
                    }
                    JX = KH;
                    JY = KI;
                }
                let KB = (JZ * IX) / KA;
                let KC = (IY * JZ) / KA;
                let KD = if KB > JF { 1.0 } else { 0.0 };
                let KQ;
                let KR;
                if KD != 0.0 {
                    let KN = JH * ((AA + KB) - JF);
                    let KO = KC * JH;
                    KQ = KN;
                    KR = KO;
                } else {
                    let KP = if KB < -1e2f64 { 1.0 } else { 0.0 };
                    let LK;
                    let LL;
                    if KP != 0.0 {
                        LK = JO;
                        LL = T;
                    } else {
                        let LI = KB.exp();
                        let LJ = KC * LI;
                        LK = LI;
                        LL = LJ;
                    }
                    KQ = LK;
                    KR = LL;
                }
                let KT = KS * JL;
                let KU = JM * KS;
                let KW = KV * JL;
                let KX = JM * KV;
                let KZ = KY * JX;
                let LA = JY * KY;
                let LC = LB * KQ;
                let LD = KR * LB;
                let LF = LE * AB;
                let LG = Z * LE;
                let LH = if LF > JF { 1.0 } else { 0.0 };
                let LP;
                let LQ;
                if LH != 0.0 {
                    let LM = JH * ((AA + LF) - JF);
                    let LN = LG * JH;
                    LP = LM;
                    LQ = LN;
                } else {
                    let LO = if LF < -1e2f64 { 1.0 } else { 0.0 };
                    let MA;
                    let MB;
                    if LO != 0.0 {
                        MA = JO;
                        MB = T;
                    } else {
                        let LY = LF.exp();
                        let LZ = LG * LY;
                        MA = LY;
                        MB = LZ;
                    }
                    LP = MA;
                    LQ = MB;
                }
                let LS = LR * LP;
                let LT = LQ * LR;
                let LV = JA / LU;
                let LW = JB / LU;
                let LX = if LV > JF { 1.0 } else { 0.0 };
                let MF;
                let MG;
                if LX != 0.0 {
                    let MC = JH * ((AA + LV) - JF);
                    let MD = LW * JH;
                    MF = MC;
                    MG = MD;
                } else {
                    let ME = if LV < -1e2f64 { 1.0 } else { 0.0 };
                    let MK;
                    let ML;
                    if ME != 0.0 {
                        MK = JO;
                        ML = T;
                    } else {
                        let MI = LV.exp();
                        let MJ = LW * MI;
                        MK = MI;
                        ML = MJ;
                    }
                    MF = MK;
                    MG = ML;
                }
                let MQ;
                let MR;
                if MH != 0.0 {
                    MQ = MF;
                    MR = MG;
                } else {
                    let MN = (MM * IX) / LU;
                    let MO = (IY * MM) / LU;
                    let MP = if MN > JF { 1.0 } else { 0.0 };
                    let NA;
                    let NB;
                    if MP != 0.0 {
                        let MX = JH * ((AA + MN) - JF);
                        let MY = MO * JH;
                        NA = MX;
                        NB = MY;
                    } else {
                        let MZ = if MN < -1e2f64 { 1.0 } else { 0.0 };
                        let NE;
                        let NF;
                        if MZ != 0.0 {
                            NE = JO;
                            NF = T;
                        } else {
                            let NC = MN.exp();
                            let ND = MO * NC;
                            NE = NC;
                            NF = ND;
                        }
                        NA = NE;
                        NB = NF;
                    }
                    MQ = NA;
                    MR = NB;
                }
                let MU = (MS * IX) / MT;
                let MV = (IY * MS) / MT;
                let MW = if MU > JF { 1.0 } else { 0.0 };
                let NJ;
                let NK;
                if MW != 0.0 {
                    let NG = JH * ((AA + MU) - JF);
                    let NH = MV * JH;
                    NJ = NG;
                    NK = NH;
                } else {
                    let NI = if MU < -1e2f64 { 1.0 } else { 0.0 };
                    let OD;
                    let OE;
                    if NI != 0.0 {
                        OD = JO;
                        OE = T;
                    } else {
                        let OB = MU.exp();
                        let OC = MV * OB;
                        OD = OB;
                        OE = OC;
                    }
                    NJ = OD;
                    NK = OE;
                }
                let NM = NL * MF;
                let NN = MG * NL;
                let NP = NO * MF;
                let NQ = MG * NO;
                let NS = NR * MQ;
                let NT = MR * NR;
                let NV = NU * NJ;
                let NW = NK * NU;
                let NY = NX * AB;
                let NZ = Z * NX;
                let OA = if NY > JF { 1.0 } else { 0.0 };
                let OI;
                let OJ;
                if OA != 0.0 {
                    let OF = JH * ((AA + NY) - JF);
                    let OG = NZ * JH;
                    OI = OF;
                    OJ = OG;
                } else {
                    let OH = if NY < -1e2f64 { 1.0 } else { 0.0 };
                    let OU;
                    let OV;
                    if OH != 0.0 {
                        OU = JO;
                        OV = T;
                    } else {
                        let OS = NY.exp();
                        let OT = NZ * OS;
                        OU = OS;
                        OV = OT;
                    }
                    OI = OU;
                    OJ = OV;
                }
                let OL = OK * OI;
                let OM = OJ * OK;
                let OP = OO * (Y.powf(ON));
                let OQ = (Z * (ON * (Y.powf(staged[437])))) * OO;
                let PD;
                let PE;
                if OR != 0.0 {
                    let OY = (Z * OW) * OX;
                    let PA = (OX * (AA + (OW * Y))) + OZ;
                    PD = PA;
                    PE = OY;
                } else {
                    let PB = (Z * OW) * OX;
                    let PC = (OX * (AA + (OW * AB))) + OZ;
                    PD = PC;
                    PE = PB;
                }
                let PF = staged[152] / PD;
                let PG = ((PE * PF) * DW) / PD;
                let PH = staged[153] / PD;
                let PI = ((PE * PH) * DW) / PD;
                let PJ = AA + PF;
                let PK = (AA + PH) / PJ;
                let PL = OP * PK;
                let PM = (OQ * PK) + (((PI - (PG * PK)) / PJ) * OP);
                let PO = staged[13] - (PN * AB);
                let PQ = AA + (PP * PF);
                let PR = (AA + (PP * PH)) / PQ;
                let PS = PO * PR;
                let PT = (((Z * PN) * DW) * PR) + ((((PI * PP) - ((PG * PP) * PR)) / PQ) * PO);
                let QG;
                let QH;
                let QI;
                let QJ;
                let QK;
                let QL;
                let QM;
                let QN;
                let QO;
                let QP;
                if PU != 0.0 {
                    let PX = (staged[155] + (PV * AB)) / PW;
                    let PY = (Z * PV) / PW;
                    QG = PX;
                    QH = S;
                    QI = BD;
                    QJ = S;
                    QK = BF;
                    QL = PY;
                    QM = T;
                    QN = T;
                    QO = T;
                    QP = T;
                } else {
                    let PZ = PV * AB;
                    let QB = (staged[17] + PZ) / QA;
                    let QC = (Z * PV) / QA;
                    let QD = (parameters[133] + PZ) / QA;
                    let QE = (staged[19] + PZ) / QA;
                    let QF = (parameters[132] + PZ) / QA;
                    QG = S;
                    QH = QE;
                    QI = QF;
                    QJ = QB;
                    QK = QD;
                    QL = T;
                    QM = QC;
                    QN = QC;
                    QO = QC;
                    QP = QC;
                }
                let QR = Z * QQ;
                let QS = staged[5] + (QQ * AB);
                let QU = Z * QT;
                let QV = staged[7] + (QT * AB);
                let QX = Z * QW;
                let QY = staged[9] + (QW * AB);
                BG = HX;
                BH = HZ;
                BI = GX;
                BJ = FL;
                BK = FN;
                BL = IC;
                BM = IN;
                BN = IE;
                BO = QG;
                BP = FO;
                BQ = QS;
                BR = QY;
                BS = QV;
                BT = PL;
                BU = PS;
                BV = IV;
                BW = KZ;
                BX = NS;
                BY = LC;
                BZ = NV;
                CA = KW;
                CB = NP;
                CC = KT;
                CD = NM;
                CE = LS;
                CF = OL;
                CG = QH;
                CH = QI;
                CI = QJ;
                CJ = QK;
                CK = HY;
                CL = IA;
                CM = GY;
                CN = FP;
                CO = FR;
                CP = ID;
                CQ = IO;
                CR = IF;
                CS = QL;
                CT = FS;
                CU = QR;
                CV = QX;
                CW = QU;
                CX = PM;
                CY = PT;
                CZ = IU;
                DA = LA;
                DB = NT;
                DC = LD;
                DD = NW;
                DE = KX;
                DF = NQ;
                DG = KU;
                DH = NN;
                DI = LT;
                DJ = OM;
                DK = QM;
                DL = QN;
                DM = QO;
                DN = QP;
            } else {
                BG = AC;
                BH = AD;
                BI = AE;
                BJ = AF;
                BK = AG;
                BL = AH;
                BM = AI;
                BN = AJ;
                BO = AK;
                BP = AL;
                BQ = AM;
                BR = AN;
                BS = AO;
                BT = AP;
                BU = AQ;
                BV = AR;
                BW = AS;
                BX = AT;
                BY = AU;
                BZ = AV;
                CA = AW;
                CB = AX;
                CC = AY;
                CD = AZ;
                CE = BA;
                CF = BB;
                CG = BC;
                CH = BD;
                CI = BE;
                CJ = BF;
                CK = T;
                CL = T;
                CM = T;
                CN = T;
                CO = T;
                CP = T;
                CQ = T;
                CR = T;
                CS = T;
                CT = T;
                CU = T;
                CV = T;
                CW = T;
                CX = T;
                CY = T;
                CZ = T;
                DA = T;
                DB = T;
                DC = T;
                DD = T;
                DE = T;
                DF = T;
                DG = T;
                DH = T;
                DI = T;
                DJ = T;
                DK = T;
                DL = T;
                DM = T;
                DN = T;
            }
            let RA;
            let RB;
            if F != 0.0 {
                RA = RC;
                RB = T;
            } else {
                let RE;
                let RF;
                if QZ != 0.0 {
                    let RH = BG - staged[157];
                    RE = RH;
                    RF = CK;
                } else {
                    RE = RD;
                    RF = T;
                }
                let RG = if RE > S { 1.0 } else { 0.0 };
                let RK;
                let RL;
                if RG != 0.0 {
                    let RI = -RE;
                    let RJ = RF * DW;
                    RK = RI;
                    RL = RJ;
                } else {
                    RK = RE;
                    RL = RF;
                }
                let RN = if RM > S { 1.0 } else { 0.0 };
                let RP = if RN != 0.0 {
                    let RO = -RM;
                    RO
                } else {
                    RM
                };
                let RQ = if (if parameter_given[84] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let RV = if RQ != 0.0 {
                    let RT = (RR * (HB.sqrt())) / RS;
                    RT
                } else {
                    RU
                };
                let RW = if (if parameter_given[85] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let RZ = if RW != 0.0 {
                    let RX = (RR * (staged[100].sqrt())) / RS;
                    RX
                } else {
                    RY
                };
                let SA = RV - RZ;
                let SB = (BG - RK).sqrt();
                let SC = (BG - RP).sqrt();
                let SD = CK * (EA / (DZ * SC));
                let SE = SC - BH;
                let SF = (EG * (BH * SE)) + RP;
                let SG = (SA * (SB - BH)) / SF;
                let SH = EG * ((staged[159] - staged[160]) + SG);
                let SI = RZ - (SH * SC);
                let SJ = (((((((((CK - RL) * (EA / (DZ * SB))) - CL) * SA) - ((((CL * SE) + ((SD - CL) * BH)) * EG) * SG)) / SF) * EG) * SC) + (SD * SH)) * DW;
                RA = SI;
                RB = SJ;
            }
            let SM = if G != 0.0 {
                SK
            } else {
                SL
            };
            let SN = AA + (staged[161] / SM);
            let SO = RA * SN;
            let SP = RB * SN;
            let SS;
            let ST;
            if H != 0.0 {
                let SW;
                let SX;
                if SQ != 0.0 {
                    let SU = (staged[162] - BG) - (SO * BH);
                    let SV = (CK * DW) - ((SP * BH) + (CL * SO));
                    SW = SU;
                    SX = SV;
                } else {
                    SW = SR;
                    SX = T;
                }
                SS = SW;
                ST = SX;
            } else {
                SS = SR;
                ST = T;
            }
            let TC;
            let TD;
            if I != 0.0 {
                let SZ = SY * ((SS + BG) + (SO * BH));
                let TA = ((ST + CK) + ((SP * BH) + (CL * SO))) * SY;
                TC = SZ;
                TD = TA;
            } else {
                TC = TB;
                TD = T;
            }
            let TF;
            let TG;
            let TH;
            let TI;
            let TJ;
            let TK;
            let TL;
            let TM;
            let TN;
            let TO;
            let TP;
            let TQ;
            let TR;
            let TS;
            if TE != 0.0 {
                let VQ;
                let VR;
                let VS;
                let VT;
                if N != 0.0 {
                    VQ = AM;
                    VR = AN;
                    VS = T;
                    VT = T;
                } else {
                    VQ = BQ;
                    VR = BR;
                    VS = CU;
                    VT = CV;
                }
                TF = AI;
                TG = AJ;
                TH = VQ;
                TI = VR;
                TJ = AR;
                TK = BE;
                TL = BF;
                TM = T;
                TN = T;
                TO = VS;
                TP = VT;
                TQ = T;
                TR = T;
                TS = T;
            } else {
                TF = BM;
                TG = BN;
                TH = BQ;
                TI = BR;
                TJ = BV;
                TK = CI;
                TL = CJ;
                TM = CQ;
                TN = CR;
                TO = CU;
                TP = CV;
                TQ = CZ;
                TR = DM;
                TS = DN;
            }
            let TX = SY * (TT - TU);
            let TY = (Lanes([TV, 0.0]) - Lanes([0.0, TW])) * SY;
            let UB = SY * (TZ - TU);
            let UC = (Lanes([UA, 0.0]) - Lanes([0.0, TW])) * SY;
            let UF = SY * (UD - TU);
            let UG = (Lanes([0.0, UE]) - Lanes([TW, 0.0])) * SY;
            let UJ = SY * (UH - TU);
            let UK = (Lanes([UI, 0.0]) - Lanes([0.0, TW])) * SY;
            let UN = SY * (TZ - UL);
            let UO = (Lanes([0.0, UA]) - Lanes([UM, 0.0])) * SY;
            let UP = SY * (UD - UL);
            let UQ = (Lanes([0.0, UE]) - Lanes([UM, 0.0])) * SY;
            let UT = SY * (UR - TU);
            let UU = (Lanes([0.0, US]) - Lanes([TW, 0.0])) * SY;
            let UX = SY * (UV - TT);
            let UY = (Lanes([0.0, UW]) - Lanes([TV, 0.0])) * SY;
            let VB = SY * (UZ - TU);
            let VC = (Lanes([0.0, VA]) - Lanes([TW, 0.0])) * SY;
            let VD = UB - TX;
            let VE = Lanes([UC[0], 0.0, UC[1]]);
            let VF = VE - Lanes([0.0, TY[0], TY[1]]);
            let VG = UF - TX;
            let VH = Lanes([0.0, UG[0], UG[1]]);
            let VI = VH - Lanes([TY[0], TY[1], 0.0]);
            let VJ = UJ - TX;
            let VK = Lanes([UK[0], 0.0, UK[1]]);
            let VL = Lanes([0.0, TY[0], TY[1]]);
            let VM = VK - VL;
            let VN = VB - TX;
            let VO = Lanes([0.0, VC[0], VC[1]]) - Lanes([TY[0], TY[1], 0.0]);
            let VP = if TX >= S { 1.0 } else { 0.0 };
            let WN;
            let WO;
            let WP;
            let WQ;
            let WR;
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
            let XC;
            let XD;
            let XE;
            let XF;
            let XG;
            let XH;
            let XI;
            let XJ;
            let XK;
            let XL;
            let XM;
            let XN;
            let XO;
            let XP;
            if VP != 0.0 {
                WN = UJ;
                WO = UF;
                WP = VG;
                WQ = UB;
                WR = TX;
                WS = VU;
                WT = VV;
                WU = VW;
                WV = VX;
                WW = VY;
                WX = VZ;
                WY = WA;
                WZ = WB;
                XA = WC;
                XB = WD;
                XC = VD;
                XD = WE;
                XE = WF;
                XF = WG;
                XG = WH;
                XH = WI;
                XI = WJ;
                XJ = AA;
                XK = VK;
                XL = VH;
                XM = VI;
                XN = VE;
                XO = TY;
                XP = VF;
            } else {
                let WK = -TX;
                let WL = TY * DW;
                WN = VJ;
                WO = VG;
                WP = UF;
                WQ = VD;
                WR = WK;
                WS = VZ;
                WT = WA;
                WU = WB;
                WV = WC;
                WW = WD;
                WX = VU;
                WY = VV;
                WZ = VW;
                XA = VX;
                XB = VY;
                XC = UB;
                XD = WH;
                XE = WI;
                XF = WJ;
                XG = WE;
                XH = WF;
                XI = WG;
                XJ = WM;
                XK = VM;
                XL = VI;
                XM = VH;
                XN = VF;
                XO = WL;
                XP = VE;
            }
            let XQ = WN - BI;
            let XR = Lanes([XK[0], 0.0, XK[1], XK[2]]) - Lanes([0.0, CM, 0.0, 0.0]);
            let XS = SS + BG;
            let XT = ST + CK;
            let XW = if (if XU != 0.0 && (if WO > XS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && XV != 0.0 { 1.0 } else { 0.0 };
            let YQ;
            let YR;
            if XW != 0.0 {
                let XZ = ((1.60219e-13f64 * XX) * XY) / (RS * RS);
                let YA = Lanes([0.0, XL[0], XL[1], XL[2]]);
                let YB = (AA + ((EG * (WO - XS)) / XZ)).sqrt();
                let YC = XZ * (YB - AA);
                let YD = ((((YA - Lanes([XT, 0.0, 0.0, 0.0])) * EG) / XZ) * (EA / (DZ * YB))) * XZ;
                let YF = YE * YC;
                let YH = ((((YD * YE) * YC) + (YD * YF)) / XZ) * DW;
                let YJ = (YG - ((YF * YC) / XZ)) - YI;
                let YK = YH * YJ;
                let YM = ((YJ * YJ) + YL).sqrt();
                let YN = WO - (YG - (YE * (YJ + YM)));
                let YO = YA - (((YH + ((YK + YK) * (EA / (DZ * YM)))) * YE) * DW);
                YQ = YN;
                YR = YO;
            } else {
                let YP = Lanes([0.0, XL[0], XL[1], XL[2]]);
                YQ = WO;
                YR = YP;
            }
            let YS = if (if XU != 0.0 && (if WP > XS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && XV != 0.0 { 1.0 } else { 0.0 };
            let ZG;
            let ZH;
            if YS != 0.0 {
                let YT = ((1.60219e-13f64 * XX) * XY) / (RS * RS);
                let YU = Lanes([0.0, XM[0], XM[1], XM[2]]);
                let YV = (AA + ((EG * (WP - XS)) / YT)).sqrt();
                let YW = YT * (YV - AA);
                let YX = ((((YU - Lanes([XT, 0.0, 0.0, 0.0])) * EG) / YT) * (EA / (DZ * YV))) * YT;
                let YY = YE * YW;
                let YZ = ((((YX * YE) * YW) + (YX * YY)) / YT) * DW;
                let ZA = (YG - ((YY * YW) / YT)) - YI;
                let ZB = YZ * ZA;
                let ZC = ((ZA * ZA) + YL).sqrt();
                let ZD = WP - (YG - (YE * (ZA + ZC)));
                let ZE = YU - (((YZ + ((ZB + ZB) * (EA / (DZ * ZC)))) * YE) * DW);
                ZG = ZD;
                ZH = ZE;
            } else {
                let ZF = Lanes([0.0, XM[0], XM[1], XM[2]]);
                ZG = WP;
                ZH = ZF;
            }
            let ZK;
            let ZL;
            if P != 0.0 {
                let ZI = DO * W;
                let ZJ = V * DO;
                ZK = ZI;
                ZL = ZJ;
            } else {
                ZK = BJ;
                ZL = CN;
            }
            let ZM = BK - BG;
            let ZN = CO - CK;
            let ZR;
            let ZS;
            let ZT;
            let ZU;
            let ZV;
            let ZW;
            if ZO != 0.0 {
                let ZP = Lanes([0.0, XN[0], 0.0, XN[1], XN[2], 0.0]);
                ZR = WQ;
                ZS = WQ;
                ZT = WQ;
                ZU = ZP;
                ZV = ZP;
                ZW = ZP;
            } else {
                let ACS;
                let ACT;
                let ACU;
                let ACV;
                if ZQ != 0.0 {
                    let ACB = ((BG - staged[167]) + ACA) + (ABZ * ZM);
                    let ACC = CK + (ZN * ABZ);
                    let ACF = (ACE * ACB) + (ACD * XQ);
                    let ACG = Lanes([0.0, (ACC * ACE), 0.0, 0.0]) + (XR * ACD);
                    let ACH = Lanes([ACC, 0.0, 0.0]);
                    ACS = ACB;
                    ACT = ACF;
                    ACU = ACH;
                    ACV = ACG;
                } else {
                    let ACM = (XO * ACJ) * ACL;
                    let ACN = (ACK * ((BG - staged[173]) + ACA)) + (ACL * (ACJ * (WR + ACI)));
                    let ACO = Lanes([(CK * ACK), 0.0, 0.0]) + Lanes([0.0, ACM[0], ACM[1]]);
                    let ACQ = ACN + (ACP * XQ);
                    let ACR = Lanes([0.0, ACO[0], ACO[1], ACO[2]]) + (XR * ACP);
                    ACS = ACN;
                    ACT = ACQ;
                    ACU = ACO;
                    ACV = ACR;
                }
                let ACW = Lanes([0.0, ACU[0], ACU[1], ACU[2]]) - ACV;
                let ACY = (ACS - ACT) - ACX;
                let ACZ = ACW * ACY;
                let ADB = ((ACY * ACY) + ADA).sqrt();
                let ADC = YE * (ACY + ADB);
                let ADD = (ACW + ((ACZ + ACZ) * (EA / (DZ * ADB)))) * YE;
                let ADG = (ADC * ADE) / ADF;
                let ADH = YE * ADC;
                let ADJ = BG - ADI;
                let ADK = Lanes([0.0, CK, 0.0, 0.0]);
                let ADL = ADK - (ACV - (((ADD * YE) * ADG) + (((ADD * ADE) / ADF) * ADH)));
                let ADM = (ADJ - (ACT - (ADH * ADG))) - ACX;
                let ADN = ADL * ADM;
                let ADO = ((ADM * ADM) + 2e-2f64).sqrt();
                let ADP = ADJ - (YE * (ADM + ADO));
                let ADQ = ADK - ((ADL + ((ADN + ADN) * (EA / (DZ * ADO)))) * YE);
                let ADR = (BG - ADP).sqrt();
                let ADS = (ADK - ADQ) * (EA / (DZ * ADR));
                let ADT = (BL * ADR) / BH;
                let ADU = ((Lanes([0.0, (CP * ADR), 0.0, 0.0]) + (ADS * BL)) - Lanes([0.0, (CL * ADT), 0.0, 0.0])) / BH;
                let ADV = ADT.sqrt();
                let ADW = ADU * (EA / (DZ * ADV));
                let ADX = ABV * ADP;
                let ADY = ADQ * ABV;
                let ADZ = if ADX >= -5e-1f64 { 1.0 } else { 0.0 };
                let AEI;
                let AEJ;
                if ADZ != 0.0 {
                    let AEA = AA + ADX;
                    AEI = AEA;
                    AEJ = ADY;
                } else {
                    let AED = AEC + (AEB * ADX);
                    let AEE = AA / AED;
                    let AEF = AA + (AEC * ADX);
                    let AEG = AEF * AEE;
                    let AEH = ((ADY * AEC) * AEE) + (((((ADY * AEB) * AEE) * DW) / AED) * AEF);
                    AEI = AEG;
                    AEJ = AEH;
                }
                let AEL = AEK * ADV;
                let AEM = ADW * AEK;
                let AEN = AEL * AEI;
                let AEO = (AEM * AEI) + (AEJ * AEL);
                let AEQ = AEP * ADP;
                let AER = ADQ * AEP;
                let AES = if AEQ >= -5e-1f64 { 1.0 } else { 0.0 };
                let AEZ;
                let AFA;
                if AES != 0.0 {
                    let AET = AA + AEQ;
                    AEZ = AET;
                    AFA = AER;
                } else {
                    let AEU = AEC + (AEB * AEQ);
                    let AEV = AA / AEU;
                    let AEW = AA + (AEC * AEQ);
                    let AEX = AEW * AEV;
                    let AEY = ((AER * AEC) * AEV) + (((((AER * AEB) * AEV) * DW) / AEU) * AEW);
                    AEZ = AEX;
                    AFA = AEY;
                }
                let AFB = AEL * AEZ;
                let AFC = (AEM * AEZ) + (AFA * AEL);
                let AFD = staged[180] / AEN;
                let AFE = ((AEO * AFD) * DW) / AEN;
                let AFF = if AFD > -1e2f64 { 1.0 } else { 0.0 };
                let AFN;
                let AFO;
                if AFF != 0.0 {
                    let AFG = AFD.exp();
                    let AFH = AFE * AFG;
                    let AFI = AA + (EG * AFG);
                    let AFJ = AFG * AFI;
                    let AFK = (AFH * AFI) + ((AFH * EG) * AFG);
                    AFN = AFJ;
                    AFO = AFK;
                } else {
                    AFN = AFL;
                    AFO = AFM;
                }
                let AFP = staged[181] / ADT;
                let AFT = XO * AFS;
                let AFU = (AFR + (AFQ * ADP)) + (AFS * WR);
                let AFW = ((AFP + (AFU * AFN)) + AFV) / RS;
                let AFX = ((((ADU * AFP) * DW) / ADT) + ((((ADQ * AFQ) + Lanes([0.0, 0.0, AFT[0], AFT[1]])) * AFN) + (AFO * AFU))) / RS;
                let AFY = if AFW >= -5e-1f64 { 1.0 } else { 0.0 };
                let AGF;
                let AGG;
                if AFY != 0.0 {
                    let AFZ = AA + AFW;
                    AGF = AFZ;
                    AGG = AFX;
                } else {
                    let AGA = AEC + (AEB * AFW);
                    let AGB = AA / AGA;
                    let AGC = AA + (AEC * AFW);
                    let AGD = AGC * AGB;
                    let AGE = ((AFX * AEC) * AGB) + (((((AFX * AEB) * AGB) * DW) / AGA) * AGC);
                    AGF = AGD;
                    AGG = AGE;
                }
                let AGM;
                let AGN;
                if AGH != 0.0 {
                    let AGJ = AGI * WR;
                    let AGK = XO * AGI;
                    let AGL = if AGJ < -1e2f64 { 1.0 } else { 0.0 };
                    let AGY;
                    let AGZ;
                    if AGL != 0.0 {
                        AGY = JO;
                        AGZ = AGV;
                    } else {
                        let AGW = AGJ.exp();
                        let AGX = AGK * AGW;
                        AGY = AGW;
                        AGZ = AGX;
                    }
                    let AHC = AHB + (AHA * (AA + AGY));
                    let AHD = AHB / AHC;
                    let AHE = (((AGZ * AHA) * AHD) * DW) / AHC;
                    let AHF = if AHD > FJ { 1.0 } else { 0.0 };
                    let AHJ;
                    let AHK;
                    if AHF != 0.0 {
                        let AHG = AHD.ln();
                        let AHH = AHE * (EA / AHD);
                        AHJ = AHG;
                        AHK = AHH;
                    } else {
                        AHJ = AHI;
                        AHK = AGV;
                    }
                    let AHL = ZK * AHJ;
                    let AHM = AHK * ZK;
                    let AHN = AGF * AHL;
                    let AHO = (Lanes([(ZL * AHJ), 0.0, 0.0]) + Lanes([0.0, AHM[0], AHM[1]])) * AGF;
                    let AHP = (AGG * AHL) + Lanes([0.0, AHO[0], AHO[1], AHO[2]]);
                    AGM = AHN;
                    AGN = AHP;
                } else {
                    AGM = S;
                    AGN = AFM;
                }
                let AGP = AGO * AFN;
                let AGQ = AGP * ZM;
                let AGR = ((AFO * AGO) * ZM) + Lanes([0.0, (ZN * AGP), 0.0, 0.0]);
                let AGS = staged[187] / AFB;
                let AGT = ((AFC * AGS) * DW) / AFB;
                let AGU = if AGS > -1e2f64 { 1.0 } else { 0.0 };
                let AHW;
                let AHX;
                if AGU != 0.0 {
                    let AHQ = AGS.exp();
                    let AHR = AGT * AHQ;
                    let AHS = AA + (EG * AHQ);
                    let AHT = AHQ * AHS;
                    let AHU = (AHR * AHS) + ((AHR * EG) * AHQ);
                    AHW = AHT;
                    AHX = AHU;
                } else {
                    AHW = AHV;
                    AHX = AFM;
                }
                let AHZ = AHY * AHW;
                let AIA = AHZ * ZM;
                let AIB = ((AHX * AHY) * ZM) + Lanes([0.0, (ZN * AHZ), 0.0, 0.0]);
                let AIE = AID + (AIC * ADP);
                let AIG = (AIF * BH) + (AIE * AB);
                let AIH = Lanes([0.0, (CL * AIF), 0.0, 0.0]) + (((ADQ * AIC) * AB) + Lanes([0.0, (Z * AIE), 0.0, 0.0]));
                let AIK = (AII * BG) / AIJ;
                let AIL = (CK * AII) / AIJ;
                let AIN = ADQ * AIM;
                let AIP = AIO + (AIM * ADP);
                let AIR = if AIP < AIQ { 1.0 } else { 0.0 };
                let AIZ;
                let AJA;
                if AIR != 0.0 {
                    let AIT = AEC - (AIS * AIP);
                    let AIU = AA / AIT;
                    let AIW = AIV - AIP;
                    let AIX = AIW * AIU;
                    let AIY = ((AIN * DW) * AIU) + ((((((AIN * AIS) * DW) * AIU) * DW) / AIT) * AIW);
                    AIZ = AIX;
                    AJA = AIY;
                } else {
                    AIZ = AIP;
                    AJA = AIN;
                }
                let AJB = AIZ * TF;
                let AJC = AJB * WR;
                let AJD = XO * AJB;
                let AJE = (((AJA * TF) + Lanes([0.0, (TM * AIZ), 0.0, 0.0])) * WR) + Lanes([0.0, 0.0, AJD[0], AJD[1]]);
                let AJG = ADQ * AJF;
                let AJI = AJH + (AJF * ADP);
                let AJJ = if AJI < AIQ { 1.0 } else { 0.0 };
                let AJP;
                let AJQ;
                if AJJ != 0.0 {
                    let AJK = AEC - (AIS * AJI);
                    let AJL = AA / AJK;
                    let AJM = AIV - AJI;
                    let AJN = AJM * AJL;
                    let AJO = ((AJG * DW) * AJL) + ((((((AJG * AIS) * DW) * AJL) * DW) / AJK) * AJM);
                    AJP = AJN;
                    AJQ = AJO;
                } else {
                    AJP = AJI;
                    AJQ = AJG;
                }
                let AJR = AJP * TF;
                let AJS = XO * AJR;
                let AJU = (AJT * WR).exp();
                let AJV = (XO * AJT) * AJU;
                let AJX = AJU + AA;
                let AJY = (AJW * (AJU - AA)) / AJX;
                let AJZ = ((AJV * AJW) - (AJV * AJY)) / AJX;
                let AKD = ((Lanes([0.0, (TD * SY), 0.0, 0.0]) + (((ADS * AKA) - Lanes([0.0, ((SP * BH) + (CL * SO)), 0.0, 0.0])) * AKB)) - (ADQ * AKC)) - AGR;
                let AKG = AKF + (AKE * ADP);
                let AKH = ((ADQ * AKE) * AIK) + Lanes([0.0, (AIL * AKG), 0.0, 0.0]);
                let AKI = (((((SY * TC) + (((AKA * ADR) - (SO * BH)) * AKB)) - (AKC * ADP)) - AGQ) - AIA) + (AKG * AIK);
                let AKJ = (((AKI + AIG) - AJC) - AGM) - AJY;
                let AKK = Lanes([0.0, 0.0, AJZ[0], AJZ[1]]);
                let AKL = (((((AKD - AIB) + AKH) + AIH) - AJE) - AGN) - AKK;
                let AKM = (((AKI + AIG) - (AJR * WR)) - AGM) - AJY;
                let AKN = (((((AKD - AIB) + AKH) + AIH) - ((((AJQ * TF) + Lanes([0.0, (TM * AJP), 0.0, 0.0])) * WR) + Lanes([0.0, 0.0, AJS[0], AJS[1]]))) - AGN) - AKK;
                let AKO = Lanes([AKL[0], AKL[1], AKL[2], AKL[3], 0.0]);
                let AKP = Lanes([0.0, YR[0], YR[1], YR[2], YR[3]]);
                let AKR = AKQ * ZK;
                let AKS = ZL * AKQ;
                let AKU = ((AKJ - YQ) - AKT) / AKR;
                let AKV = ((AKO - AKP) - Lanes([0.0, (AKS * AKU), 0.0, 0.0, 0.0])) / AKR;
                let AKW = if AKU > JF { 1.0 } else { 0.0 };
                let ALA;
                let ALB;
                if AKW != 0.0 {
                    let AKX = JH * ((AA + AKU) - JF);
                    let AKY = AKV * JH;
                    ALA = AKX;
                    ALB = AKY;
                } else {
                    let AKZ = if AKU < -1e2f64 { 1.0 } else { 0.0 };
                    let ALM;
                    let ALN;
                    if AKZ != 0.0 {
                        ALM = JO;
                        ALN = ALJ;
                    } else {
                        let ALK = AKU.exp();
                        let ALL = AKV * ALK;
                        ALM = ALK;
                        ALN = ALL;
                    }
                    ALA = ALM;
                    ALB = ALN;
                }
                let ALC = AA + ALA;
                let ALD = ALC.ln();
                let ALE = AKR * ALD;
                let ALF = Lanes([0.0, (AKS * ALD), 0.0, 0.0, 0.0]) + ((ALB * (EA / ALC)) * AKR);
                let ALG = ((YQ - AKJ) - AKT) / AKR;
                let ALH = ((AKP - AKO) - Lanes([0.0, (AKS * ALG), 0.0, 0.0, 0.0])) / AKR;
                let ALI = if ALG > JF { 1.0 } else { 0.0 };
                let ALR;
                let ALS;
                if ALI != 0.0 {
                    let ALO = JH * ((AA + ALG) - JF);
                    let ALP = ALH * JH;
                    ALR = ALO;
                    ALS = ALP;
                } else {
                    let ALQ = if ALG < -1e2f64 { 1.0 } else { 0.0 };
                    let AMM;
                    let AMN;
                    if ALQ != 0.0 {
                        AMM = JO;
                        AMN = ALJ;
                    } else {
                        let AMK = ALG.exp();
                        let AML = ALH * AMK;
                        AMM = AMK;
                        AMN = AML;
                    }
                    ALR = AMM;
                    ALS = AMN;
                }
                let ALT = AA + ALR;
                let ALU = ALT.ln();
                let ALV = AKR * ALU;
                let ALW = Lanes([0.0, (AKS * ALU), 0.0, 0.0, 0.0]) + ((ALS * (EA / ALT)) * AKR);
                let ALY = ALX * ZK;
                let ALZ = ALY * ZK;
                let AMA = ((ZL * ALX) * ZK) + (ZL * ALY);
                let AMB = EG * SO;
                let AMC = BG.sqrt();
                let AMD = AMB * AMC;
                let AME = ALV + AMD;
                let AMF = Lanes([0.0, (((SP * EG) * AMC) + ((CK * (EA / (DZ * AMC))) * AMB)), 0.0, 0.0, 0.0]);
                let AMG = (ALV * AME) / ALZ;
                let AMH = (((ALW * AME) + ((ALW + AMF) * ALV)) - Lanes([0.0, (AMA * AMG), 0.0, 0.0, 0.0])) / ALZ;
                let AMI = AA + AMG;
                let AMJ = if AMI > FJ { 1.0 } else { 0.0 };
                let AMR;
                let AMS;
                if AMJ != 0.0 {
                    let AMO = AMI.ln();
                    let AMP = AMH * (EA / AMI);
                    AMR = AMO;
                    AMS = AMP;
                } else {
                    AMR = AMQ;
                    AMS = ALJ;
                }
                let AMT = Lanes([0.0, CK, 0.0, 0.0, 0.0]);
                let AMV = (BG + (ZK * AMR)) - (AMU * ALE);
                let AMW = (AMT + (Lanes([0.0, (ZL * AMR), 0.0, 0.0, 0.0]) + (AMS * ZK))) - (ALF * AMU);
                let ANP;
                let ANQ;
                let ANR;
                let ANS;
                if ZQ != 0.0 {
                    let AMY = ((AMV - staged[206]) + ACA) + (AMX * ZM);
                    let AMZ = AMW + Lanes([0.0, (ZN * AMX), 0.0, 0.0, 0.0]);
                    let ANB = XR * ANA;
                    let AND = (ANC * AMY) + (ANA * XQ);
                    let ANE = (AMZ * ANC) + Lanes([ANB[0], ANB[1], ANB[2], ANB[3], 0.0]);
                    ANP = AND;
                    ANQ = AMY;
                    ANR = ANE;
                    ANS = AMZ;
                } else {
                    let ANI = (XO * ANF) * ANH;
                    let ANJ = (ANG * ((AMV - staged[210]) + ACA)) + (ANH * (ANF * (WR + ACI)));
                    let ANK = (AMW * ANG) + Lanes([0.0, 0.0, ANI[0], ANI[1], 0.0]);
                    let ANM = XR * ANL;
                    let ANN = ANJ + (ANL * XQ);
                    let ANO = ANK + Lanes([ANM[0], ANM[1], ANM[2], ANM[3], 0.0]);
                    ANP = ANN;
                    ANQ = ANJ;
                    ANR = ANO;
                    ANS = ANK;
                }
                let AOG;
                let AOH;
                let AOI;
                let AOJ;
                if ANT != 0.0 {
                    let ANU = ANP + ADI;
                    let ANV = Lanes([ANR[0], 0.0, ANR[1], ANR[2], ANR[3], ANR[4]]);
                    AOG = ANU;
                    AOH = ANU;
                    AOI = ANV;
                    AOJ = ANV;
                } else {
                    let ANW = ANP + ADI;
                    let ANX = Lanes([0.0, XN[0], 0.0, XN[1], XN[2], 0.0]);
                    let ANY = Lanes([ANR[0], 0.0, ANR[1], ANR[2], ANR[3], ANR[4]]);
                    let ANZ = ANX - ANY;
                    let AOB = (WQ - ANW) - AOA;
                    let AOC = ANZ * AOB;
                    let AOD = ((AOB * AOB) + AIQ).sqrt();
                    let AOE = ANW + (YE * (AOB + AOD));
                    let AOF = ANY + ((ANZ + ((AOC + AOC) * (EA / (DZ * AOD)))) * YE);
                    AOG = AOE;
                    AOH = WQ;
                    AOI = AOF;
                    AOJ = ANX;
                }
                let AOK = Lanes([ANS[0], 0.0, ANS[1], ANS[2], ANS[3], ANS[4]]) - AOI;
                let AOL = (ANQ - AOG) - ACX;
                let AOM = AOK * AOL;
                let AON = ((AOL * AOL) + ADA).sqrt();
                let AOO = YE * (AOL + AON);
                let AOP = (AOK + ((AOM + AOM) * (EA / (DZ * AON)))) * YE;
                let AOQ = (AOO * ADE) / ADF;
                let AOR = YE * AOO;
                let AOS = AOG - (AOR * AOQ);
                let AOT = AOI - (((AOP * YE) * AOQ) + (((AOP * ADE) / ADF) * AOR));
                let AOU = Lanes([AKN[0], AKN[1], AKN[2], AKN[3], 0.0]);
                let AOV = ((AKM - YQ) - AKT) / AKR;
                let AOW = ((AOU - AKP) - Lanes([0.0, (AKS * AOV), 0.0, 0.0, 0.0])) / AKR;
                let AOX = if AOV > JF { 1.0 } else { 0.0 };
                let APB;
                let APC;
                if AOX != 0.0 {
                    let AOY = JH * ((AA + AOV) - JF);
                    let AOZ = AOW * JH;
                    APB = AOY;
                    APC = AOZ;
                } else {
                    let APA = if AOV < -1e2f64 { 1.0 } else { 0.0 };
                    let APM;
                    let APN;
                    if APA != 0.0 {
                        APM = JO;
                        APN = ALJ;
                    } else {
                        let APK = AOV.exp();
                        let APL = AOW * APK;
                        APM = APK;
                        APN = APL;
                    }
                    APB = APM;
                    APC = APN;
                }
                let APD = AA + APB;
                let APE = APD.ln();
                let APF = AKR * APE;
                let APG = Lanes([0.0, (AKS * APE), 0.0, 0.0, 0.0]) + ((APC * (EA / APD)) * AKR);
                let APH = ((YQ - AKM) - AKT) / AKR;
                let API = ((AKP - AOU) - Lanes([0.0, (AKS * APH), 0.0, 0.0, 0.0])) / AKR;
                let APJ = if APH > JF { 1.0 } else { 0.0 };
                let APR;
                let APS;
                if APJ != 0.0 {
                    let APO = JH * ((AA + APH) - JF);
                    let APP = API * JH;
                    APR = APO;
                    APS = APP;
                } else {
                    let APQ = if APH < -1e2f64 { 1.0 } else { 0.0 };
                    let AQE;
                    let AQF;
                    if APQ != 0.0 {
                        AQE = JO;
                        AQF = ALJ;
                    } else {
                        let AQC = APH.exp();
                        let AQD = API * AQC;
                        AQE = AQC;
                        AQF = AQD;
                    }
                    APR = AQE;
                    APS = AQF;
                }
                let APT = AA + APR;
                let APU = APT.ln();
                let APV = AKR * APU;
                let APW = Lanes([0.0, (AKS * APU), 0.0, 0.0, 0.0]) + ((APS * (EA / APT)) * AKR);
                let APX = APV + AMD;
                let APY = (APV * APX) / ALZ;
                let APZ = (((APW * APX) + ((APW + AMF) * APV)) - Lanes([0.0, (AMA * APY), 0.0, 0.0, 0.0])) / ALZ;
                let AQA = AA + APY;
                let AQB = if AQA > FJ { 1.0 } else { 0.0 };
                let AQJ;
                let AQK;
                if AQB != 0.0 {
                    let AQG = AQA.ln();
                    let AQH = APZ * (EA / AQA);
                    AQJ = AQG;
                    AQK = AQH;
                } else {
                    AQJ = AQI;
                    AQK = ALJ;
                }
                let AQL = (BG + (ZK * AQJ)) - (AMU * APF);
                let AQM = (AMT + (Lanes([0.0, (ZL * AQJ), 0.0, 0.0, 0.0]) + (AQK * ZK))) - (APG * AMU);
                let ARF;
                let ARG;
                let ARH;
                let ARI;
                if ZQ != 0.0 {
                    let AQO = ((AQL - staged[215]) + ACA) + (AQN * ZM);
                    let AQP = AQM + Lanes([0.0, (ZN * AQN), 0.0, 0.0, 0.0]);
                    let AQR = XR * AQQ;
                    let AQT = (AQS * AQO) + (AQQ * XQ);
                    let AQU = (AQP * AQS) + Lanes([AQR[0], AQR[1], AQR[2], AQR[3], 0.0]);
                    ARF = AQT;
                    ARG = AQO;
                    ARH = AQU;
                    ARI = AQP;
                } else {
                    let AQY = (XO * AQV) * AQX;
                    let AQZ = (AQW * ((AQL - staged[219]) + ACA)) + (AQX * (AQV * (WR + ACI)));
                    let ARA = (AQM * AQW) + Lanes([0.0, 0.0, AQY[0], AQY[1], 0.0]);
                    let ARC = XR * ARB;
                    let ARD = AQZ + (ARB * XQ);
                    let ARE = ARA + Lanes([ARC[0], ARC[1], ARC[2], ARC[3], 0.0]);
                    ARF = ARD;
                    ARG = AQZ;
                    ARH = ARE;
                    ARI = ARA;
                }
                let ART;
                let ARU;
                let ARV;
                let ARW;
                if ANT != 0.0 {
                    let ARJ = ARF + ADI;
                    let ARK = Lanes([ARH[0], 0.0, ARH[1], ARH[2], ARH[3], ARH[4]]);
                    ART = ARJ;
                    ARU = ARJ;
                    ARV = ARK;
                    ARW = ARK;
                } else {
                    let ARL = ARF + ADI;
                    let ARM = Lanes([ARH[0], 0.0, ARH[1], ARH[2], ARH[3], ARH[4]]);
                    let ARN = AOJ - ARM;
                    let ARO = (AOH - ARL) - AOA;
                    let ARP = ARN * ARO;
                    let ARQ = ((ARO * ARO) + AIQ).sqrt();
                    let ARR = ARL + (YE * (ARO + ARQ));
                    let ARS = ARM + ((ARN + ((ARP + ARP) * (EA / (DZ * ARQ)))) * YE);
                    ART = ARR;
                    ARU = AOH;
                    ARV = ARS;
                    ARW = AOJ;
                }
                let ARX = Lanes([ARI[0], 0.0, ARI[1], ARI[2], ARI[3], ARI[4]]) - ARV;
                let ARY = (ARG - ART) - ACX;
                let ARZ = ARX * ARY;
                let ASA = ((ARY * ARY) + ADA).sqrt();
                let ASB = YE * (ARY + ASA);
                let ASC = (ARX + ((ARZ + ARZ) * (EA / (DZ * ASA)))) * YE;
                let ASD = (ASB * ADE) / ADF;
                let ASE = YE * ASB;
                let ASF = ART - (ASE * ASD);
                let ASG = ARV - (((ASC * YE) * ASD) + (((ASC * ADE) / ADF) * ASE));
                ZR = AOS;
                ZS = ASF;
                ZT = ARU;
                ZU = AOT;
                ZV = ASG;
                ZW = ARW;
            }
            let ZZ = (ZR + ZX) - ZY;
            let AAA = ZU * ZZ;
            let AAB = ((ZZ * ZZ) - -2e-2f64).sqrt();
            let AAD = ((ZU + ((AAA + AAA) * (EA / (DZ * AAB)))) * YE) * DW;
            let AAF = (AAC - (-5e0f64 + (YE * (ZZ + AAB)))) - AAE;
            let AAG = AAD * AAF;
            let AAH = ((AAF * AAF) + 1.2e-2f64).sqrt();
            let AAI = AAC - (YE * (AAF + AAH));
            let AAJ = ((AAD + ((AAG + AAG) * (EA / (DZ * AAH)))) * YE) * DW;
            let AAL = AAK * BG;
            let AAM = CK * AAK;
            let AAN = Lanes([0.0, 0.0, AAM, 0.0, 0.0, 0.0]);
            let AAO = AAN - AAJ;
            let AAP = (AAL - AAI) - AAE;
            let AAQ = AAO * AAP;
            let AAS = AAR * AAL;
            let AAT = Lanes([0.0, 0.0, (AAM * AAR), 0.0, 0.0, 0.0]);
            let AAU = ((AAP * AAP) + AAS).sqrt();
            let AAV = AAL - (YE * (AAP + AAU));
            let AAW = AAN - ((AAO + (((AAQ + AAQ) + AAT) * (EA / (DZ * AAU)))) * YE);
            let AAX = (ZS + ZX) - ZY;
            let AAY = ZV * AAX;
            let AAZ = ((AAX * AAX) - -2e-2f64).sqrt();
            let ABA = ((ZV + ((AAY + AAY) * (EA / (DZ * AAZ)))) * YE) * DW;
            let ABB = (AAC - (-5e0f64 + (YE * (AAX + AAZ)))) - AAE;
            let ABC = ABA * ABB;
            let ABD = ((ABB * ABB) + 1.2e-2f64).sqrt();
            let ABE = AAC - (YE * (ABB + ABD));
            let ABF = ((ABA + ((ABC + ABC) * (EA / (DZ * ABD)))) * YE) * DW;
            let ABG = AAN - ABF;
            let ABH = (AAL - ABE) - AAE;
            let ABI = ABG * ABH;
            let ABJ = ((ABH * ABH) + AAS).sqrt();
            let ABK = AAL - (YE * (ABH + ABJ));
            let ABL = AAN - ((ABG + (((ABI + ABI) + AAT) * (EA / (DZ * ABJ)))) * YE);
            let ABM = Lanes([0.0, 0.0, CK, 0.0, 0.0, 0.0]);
            let ABN = (BG - AAV).sqrt();
            let ABO = (ABM - AAW) * (EA / (DZ * ABN));
            let ABP = (BL * ABN) / BH;
            let ABQ = ((Lanes([0.0, 0.0, (CP * ABN), 0.0, 0.0, 0.0]) + (ABO * BL)) - Lanes([0.0, 0.0, (CL * ABP), 0.0, 0.0, 0.0])) / BH;
            let ABS = BJ / ABR;
            let ABT = ABP.sqrt();
            let ABU = ABQ * (EA / (DZ * ABT));
            let ABW = ABV * AAV;
            let ABX = AAW * ABV;
            let ABY = if ABW >= -5e-1f64 { 1.0 } else { 0.0 };
            let ASN;
            let ASO;
            if ABY != 0.0 {
                let ASH = AA + ABW;
                ASN = ASH;
                ASO = ABX;
            } else {
                let ASI = AEC + (AEB * ABW);
                let ASJ = AA / ASI;
                let ASK = AA + (AEC * ABW);
                let ASL = ASK * ASJ;
                let ASM = ((ABX * AEC) * ASJ) + (((((ABX * AEB) * ASJ) * DW) / ASI) * ASK);
                ASN = ASL;
                ASO = ASM;
            }
            let ASP = AEK * ABT;
            let ASQ = ABU * AEK;
            let ASR = ASP * ASN;
            let ASS = (ASQ * ASN) + (ASO * ASP);
            let AST = AEP * AAV;
            let ASU = AAW * AEP;
            let ASV = if AST >= -5e-1f64 { 1.0 } else { 0.0 };
            let ATC;
            let ATD;
            if ASV != 0.0 {
                let ASW = AA + AST;
                ATC = ASW;
                ATD = ASU;
            } else {
                let ASX = AEC + (AEB * AST);
                let ASY = AA / ASX;
                let ASZ = AA + (AEC * AST);
                let ATA = ASZ * ASY;
                let ATB = ((ASU * AEC) * ASY) + (((((ASU * AEB) * ASY) * DW) / ASX) * ASZ);
                ATC = ATA;
                ATD = ATB;
            }
            let ATE = ASP * ATC;
            let ATF = (ASQ * ATC) + (ATD * ASP);
            let ATG = staged[223] / ASR;
            let ATH = ((ASS * ATG) * DW) / ASR;
            let ATI = if ATG > -1e2f64 { 1.0 } else { 0.0 };
            let ATQ;
            let ATR;
            if ATI != 0.0 {
                let ATJ = ATG.exp();
                let ATK = ATH * ATJ;
                let ATL = AA + (EG * ATJ);
                let ATM = ATJ * ATL;
                let ATN = (ATK * ATL) + ((ATK * EG) * ATJ);
                ATQ = ATM;
                ATR = ATN;
            } else {
                ATQ = ATO;
                ATR = ATP;
            }
            let ATT = ATS / ABP;
            let ATU = AFS * WR;
            let ATV = XO * AFS;
            let ATW = (AFR + (AFQ * AAV)) + ATU;
            let ATX = Lanes([0.0, 0.0, 0.0, ATV[0], ATV[1], 0.0]);
            let ATY = ((ATT + (ATW * ATQ)) + AFV) / RS;
            let ATZ = ((((ABQ * ATT) * DW) / ABP) + ((((AAW * AFQ) + ATX) * ATQ) + (ATR * ATW))) / RS;
            let AUA = if ATY >= -5e-1f64 { 1.0 } else { 0.0 };
            let AUH;
            let AUI;
            if AUA != 0.0 {
                let AUB = AA + ATY;
                AUH = AUB;
                AUI = ATZ;
            } else {
                let AUC = AEC + (AEB * ATY);
                let AUD = AA / AUC;
                let AUE = AA + (AEC * ATY);
                let AUF = AUE * AUD;
                let AUG = ((ATZ * AEC) * AUD) + (((((ATZ * AEB) * AUD) * DW) / AUC) * AUE);
                AUH = AUF;
                AUI = AUG;
            }
            let AUO;
            let AUP;
            if AUJ != 0.0 {
                let AUL = AUK * WR;
                let AUM = XO * AUK;
                let AUN = if AUL < -1e2f64 { 1.0 } else { 0.0 };
                let AUY;
                let AUZ;
                if AUN != 0.0 {
                    AUY = JO;
                    AUZ = AGV;
                } else {
                    let AUW = AUL.exp();
                    let AUX = AUM * AUW;
                    AUY = AUW;
                    AUZ = AUX;
                }
                let AVA = AHB + (AHA * (AA + AUY));
                let AVB = AHB / AVA;
                let AVC = (((AUZ * AHA) * AVB) * DW) / AVA;
                let AVD = if AVB > FJ { 1.0 } else { 0.0 };
                let AVH;
                let AVI;
                if AVD != 0.0 {
                    let AVE = AVB.ln();
                    let AVF = AVC * (EA / AVB);
                    AVH = AVE;
                    AVI = AVF;
                } else {
                    AVH = AVG;
                    AVI = AGV;
                }
                let AVJ = ZK * AVH;
                let AVK = AVI * ZK;
                let AVL = AUH * AVJ;
                let AVM = (Lanes([(ZL * AVH), 0.0, 0.0]) + Lanes([0.0, AVK[0], AVK[1]])) * AUH;
                let AVN = (AUI * AVJ) + Lanes([0.0, 0.0, AVM[0], AVM[1], AVM[2], 0.0]);
                AUO = AVL;
                AUP = AVN;
            } else {
                AUO = S;
                AUP = ATP;
            }
            let AUQ = AGO * ATQ;
            let AUR = AUQ * ZM;
            let AUS = ((ATR * AGO) * ZM) + Lanes([0.0, 0.0, (ZN * AUQ), 0.0, 0.0, 0.0]);
            let AUT = staged[226] / ATE;
            let AUU = ((ATF * AUT) * DW) / ATE;
            let AUV = if AUT > -1e2f64 { 1.0 } else { 0.0 };
            let AVU;
            let AVV;
            if AUV != 0.0 {
                let AVO = AUT.exp();
                let AVP = AUU * AVO;
                let AVQ = AA + (EG * AVO);
                let AVR = AVO * AVQ;
                let AVS = (AVP * AVQ) + ((AVP * EG) * AVO);
                AVU = AVR;
                AVV = AVS;
            } else {
                AVU = AVT;
                AVV = ATP;
            }
            let AVW = AHY * AVU;
            let AVX = AVW * ZM;
            let AVY = ((AVV * AHY) * ZM) + Lanes([0.0, 0.0, (ZN * AVW), 0.0, 0.0, 0.0]);
            let AVZ = AID + (AIC * AAV);
            let AWA = AIF * BH;
            let AWB = CL * AIF;
            let AWC = AWA + (AVZ * AB);
            let AWD = Lanes([0.0, 0.0, AWB, 0.0, 0.0, 0.0]);
            let AWE = AWD + (((AAW * AIC) * AB) + Lanes([0.0, 0.0, (Z * AVZ), 0.0, 0.0, 0.0]));
            let AWF = (AII * BG) / AIJ;
            let AWG = (CK * AII) / AIJ;
            let AWH = AAW * AIM;
            let AWI = AIO + (AIM * AAV);
            let AWJ = if AWI < AIQ { 1.0 } else { 0.0 };
            let AWP;
            let AWQ;
            if AWJ != 0.0 {
                let AWK = AEC - (AIS * AWI);
                let AWL = AA / AWK;
                let AWM = AIV - AWI;
                let AWN = AWM * AWL;
                let AWO = ((AWH * DW) * AWL) + ((((((AWH * AIS) * DW) * AWL) * DW) / AWK) * AWM);
                AWP = AWN;
                AWQ = AWO;
            } else {
                AWP = AWI;
                AWQ = AWH;
            }
            let AWR = AWP * TF;
            let AWS = XO * AWR;
            let AWT = 2.2361e0f64 / BH;
            let AWU = ((CL * AWT) * DW) / BH;
            let AWV = AAI - AAV;
            let AWX = (AWW * WR).exp();
            let AWY = (XO * AWW) * AWX;
            let AWZ = AWX + AA;
            let AXA = (AJW * (AWX - AA)) / AWZ;
            let AXB = (AWY * AJW) - (AWY * AXA);
            let AXC = AXB / AWZ;
            let AXD = SY * TC;
            let AXE = TD * SY;
            let AXF = SO * BH;
            let AXG = (SP * BH) + (CL * SO);
            let AXH = Lanes([0.0, 0.0, AXG, 0.0, 0.0, 0.0]);
            let AXJ = Lanes([0.0, 0.0, AXE, 0.0, 0.0, 0.0]);
            let AXK = AKF + (AKE * AAV);
            let AXL = ((((((((AXD + (((AKA * (ABN - (AWT * AWV))) - AXF) * AXI)) - (AKC * AAV)) - AUR) - AVX) + (AXK * AWF)) + AWC) - (AWR * WR)) - AUO) - AXA;
            let AXM = ((((((((AXJ + ((((ABO - (Lanes([0.0, 0.0, (AWU * AWV), 0.0, 0.0, 0.0]) + ((AAJ - AAW) * AWT))) * AKA) - AXH) * AXI)) - (AAW * AKC)) - AUS) - AVY) + (((AAW * AKE) * AWF) + Lanes([0.0, 0.0, (AWG * AXK), 0.0, 0.0, 0.0]))) + AWE) - ((((AWQ * TF) + Lanes([0.0, 0.0, (TM * AWP), 0.0, 0.0, 0.0])) * WR) + Lanes([0.0, 0.0, 0.0, AWS[0], AWS[1], 0.0]))) - AUP) - Lanes([0.0, 0.0, 0.0, AXC[0], AXC[1], 0.0]);
            let AXN = (BG - ABK).sqrt();
            let AXO = (ABM - ABL) * (EA / (DZ * AXN));
            let AXP = (BL * AXN) / BH;
            let AXQ = ((Lanes([0.0, 0.0, (CP * AXN), 0.0, 0.0, 0.0]) + (AXO * BL)) - Lanes([0.0, 0.0, (CL * AXP), 0.0, 0.0, 0.0])) / BH;
            let AXS = ABS * ((RS + (AXR / AXP)) + AFV);
            let AXT = AXP.sqrt();
            let AXU = AXQ * (EA / (DZ * AXT));
            let AXV = ABV * ABK;
            let AXW = ABL * ABV;
            let AXX = if AXV >= -5e-1f64 { 1.0 } else { 0.0 };
            let AYE;
            let AYF;
            if AXX != 0.0 {
                let AXY = AA + AXV;
                AYE = AXY;
                AYF = AXW;
            } else {
                let AXZ = AEC + (AEB * AXV);
                let AYA = AA / AXZ;
                let AYB = AA + (AEC * AXV);
                let AYC = AYB * AYA;
                let AYD = ((AXW * AEC) * AYA) + (((((AXW * AEB) * AYA) * DW) / AXZ) * AYB);
                AYE = AYC;
                AYF = AYD;
            }
            let AYG = AEK * AXT;
            let AYH = AXU * AEK;
            let AYI = AYG * AYE;
            let AYJ = (AYH * AYE) + (AYF * AYG);
            let AYK = AEP * ABK;
            let AYL = ABL * AEP;
            let AYM = if AYK >= -5e-1f64 { 1.0 } else { 0.0 };
            let AYT;
            let AYU;
            if AYM != 0.0 {
                let AYN = AA + AYK;
                AYT = AYN;
                AYU = AYL;
            } else {
                let AYO = AEC + (AEB * AYK);
                let AYP = AA / AYO;
                let AYQ = AA + (AEC * AYK);
                let AYR = AYQ * AYP;
                let AYS = ((AYL * AEC) * AYP) + (((((AYL * AEB) * AYP) * DW) / AYO) * AYQ);
                AYT = AYR;
                AYU = AYS;
            }
            let AYV = AYG * AYT;
            let AYW = (AYH * AYT) + (AYU * AYG);
            let AYX = staged[229] / AYI;
            let AYY = ((AYJ * AYX) * DW) / AYI;
            let AYZ = if AYX > -1e2f64 { 1.0 } else { 0.0 };
            let AZG;
            let AZH;
            if AYZ != 0.0 {
                let AZA = AYX.exp();
                let AZB = AYY * AZA;
                let AZC = AA + (EG * AZA);
                let AZD = AZA * AZC;
                let AZE = (AZB * AZC) + ((AZB * EG) * AZA);
                AZG = AZD;
                AZH = AZE;
            } else {
                AZG = AZF;
                AZH = ATP;
            }
            let AZI = ATS / AXP;
            let AZJ = (AFR + (AFQ * ABK)) + ATU;
            let AZK = ((AZI + (AZJ * AZG)) + AFV) / RS;
            let AZL = ((((AXQ * AZI) * DW) / AXP) + ((((ABL * AFQ) + ATX) * AZG) + (AZH * AZJ))) / RS;
            let AZM = if AZK >= -5e-1f64 { 1.0 } else { 0.0 };
            let AZT;
            let AZU;
            if AZM != 0.0 {
                let AZN = AA + AZK;
                AZT = AZN;
                AZU = AZL;
            } else {
                let AZO = AEC + (AEB * AZK);
                let AZP = AA / AZO;
                let AZQ = AA + (AEC * AZK);
                let AZR = AZQ * AZP;
                let AZS = ((AZL * AEC) * AZP) + (((((AZL * AEB) * AZP) * DW) / AZO) * AZQ);
                AZT = AZR;
                AZU = AZS;
            }
            let AZZ;
            let BAA;
            if AUJ != 0.0 {
                let AZW = AZV * WR;
                let AZX = XO * AZV;
                let AZY = if AZW < -1e2f64 { 1.0 } else { 0.0 };
                let BAJ;
                let BAK;
                if AZY != 0.0 {
                    BAJ = JO;
                    BAK = AGV;
                } else {
                    let BAH = AZW.exp();
                    let BAI = AZX * BAH;
                    BAJ = BAH;
                    BAK = BAI;
                }
                let BAL = AHB + (AHA * (AA + BAJ));
                let BAM = AHB / BAL;
                let BAN = (((BAK * AHA) * BAM) * DW) / BAL;
                let BAO = if BAM > FJ { 1.0 } else { 0.0 };
                let BAS;
                let BAT;
                if BAO != 0.0 {
                    let BAP = BAM.ln();
                    let BAQ = BAN * (EA / BAM);
                    BAS = BAP;
                    BAT = BAQ;
                } else {
                    BAS = BAR;
                    BAT = AGV;
                }
                let BAU = ZK * BAS;
                let BAV = BAT * ZK;
                let BAW = AZT * BAU;
                let BAX = (Lanes([(ZL * BAS), 0.0, 0.0]) + Lanes([0.0, BAV[0], BAV[1]])) * AZT;
                let BAY = (AZU * BAU) + Lanes([0.0, 0.0, BAX[0], BAX[1], BAX[2], 0.0]);
                AZZ = BAW;
                BAA = BAY;
            } else {
                AZZ = S;
                BAA = ATP;
            }
            let BAB = AGO * AZG;
            let BAC = BAB * ZM;
            let BAD = ((AZH * AGO) * ZM) + Lanes([0.0, 0.0, (ZN * BAB), 0.0, 0.0, 0.0]);
            let BAE = staged[231] / AYV;
            let BAF = ((AYW * BAE) * DW) / AYV;
            let BAG = if BAE > -1e2f64 { 1.0 } else { 0.0 };
            let BBF;
            let BBG;
            if BAG != 0.0 {
                let BAZ = BAE.exp();
                let BBA = BAF * BAZ;
                let BBB = AA + (EG * BAZ);
                let BBC = BAZ * BBB;
                let BBD = (BBA * BBB) + ((BBA * EG) * BAZ);
                BBF = BBC;
                BBG = BBD;
            } else {
                BBF = BBE;
                BBG = ATP;
            }
            let BBH = AHY * BBF;
            let BBI = BBH * ZM;
            let BBJ = ((BBG * AHY) * ZM) + Lanes([0.0, 0.0, (ZN * BBH), 0.0, 0.0, 0.0]);
            let BBK = AID + (AIC * ABK);
            let BBL = AWA + (BBK * AB);
            let BBM = AWD + (((ABL * AIC) * AB) + Lanes([0.0, 0.0, (Z * BBK), 0.0, 0.0, 0.0]));
            let BBN = ABL * AJF;
            let BBO = AJH + (AJF * ABK);
            let BBP = if BBO < AIQ { 1.0 } else { 0.0 };
            let BBV;
            let BBW;
            if BBP != 0.0 {
                let BBQ = AEC - (AIS * BBO);
                let BBR = AA / BBQ;
                let BBS = AIV - BBO;
                let BBT = BBS * BBR;
                let BBU = ((BBN * DW) * BBR) + ((((((BBN * AIS) * DW) * BBR) * DW) / BBQ) * BBS);
                BBV = BBT;
                BBW = BBU;
            } else {
                BBV = BBO;
                BBW = BBN;
            }
            let BBX = BBV * TF;
            let BBY = XO * BBX;
            let BBZ = ABE - ABK;
            let BCA = AXB / AWZ;
            let BCB = AKF + (AKE * ABK);
            let BCC = ((((((((AXD + (((AKA * (AXN - (AWT * BBZ))) - AXF) * AXI)) - (AKC * ABK)) - BAC) - BBI) + (BCB * AWF)) + BBL) - (BBX * WR)) - AZZ) - AXA;
            let BCD = ((((((((AXJ + ((((AXO - (Lanes([0.0, 0.0, (AWU * BBZ), 0.0, 0.0, 0.0]) + ((ABF - ABL) * AWT))) * AKA) - AXH) * AXI)) - (ABL * AKC)) - BAD) - BBJ) + (((ABL * AKE) * AWF) + Lanes([0.0, 0.0, (AWG * BCB), 0.0, 0.0, 0.0]))) + BBM) - ((((BBW * TF) + Lanes([0.0, 0.0, (TM * BBV), 0.0, 0.0, 0.0])) * WR) + Lanes([0.0, 0.0, 0.0, BBY[0], BBY[1], 0.0]))) - BAA) - Lanes([0.0, 0.0, 0.0, BCA[0], BCA[1], 0.0]);
            let BCL;
            let BCM;
            if BCE != 0.0 {
                let BCF = BL.sqrt();
                let BCG = AEK * BCF;
                let BCH = (CP * (EA / (DZ * BCF))) * AEK;
                let BCI = staged[233] / BCG;
                let BCJ = ((BCH * BCI) * DW) / BCG;
                let BCK = if BCI > -1e2f64 { 1.0 } else { 0.0 };
                let BDG;
                let BDH;
                if BCK != 0.0 {
                    let BDA = BCI.exp();
                    let BDB = BCJ * BDA;
                    let BDC = AA + (EG * BDA);
                    let BDD = BDA * BDC;
                    let BDE = (BDB * BDC) + ((BDB * EG) * BDA);
                    BDG = BDD;
                    BDH = BDE;
                } else {
                    BDG = BDF;
                    BDH = T;
                }
                let BDI = AGO * BDG;
                let BDJ = BDI * ZM;
                let BDK = ((BDH * AGO) * ZM) + (ZN * BDI);
                let BDL = staged[234] / BCG;
                let BDM = ((BCH * BDL) * DW) / BCG;
                let BDN = if BDL > -1e2f64 { 1.0 } else { 0.0 };
                let BDU;
                let BDV;
                if BDN != 0.0 {
                    let BDO = BDL.exp();
                    let BDP = BDM * BDO;
                    let BDQ = AA + (EG * BDO);
                    let BDR = BDO * BDQ;
                    let BDS = (BDP * BDQ) + ((BDP * EG) * BDO);
                    BDU = BDR;
                    BDV = BDS;
                } else {
                    BDU = BDT;
                    BDV = T;
                }
                let BDW = AHY * BDU;
                let BDX = (((AXD - BDJ) - (BDW * ZM)) + (AKF * AWF)) + (AWA + (AID * AB));
                let BDY = (((AXE - BDK) - (((BDV * AHY) * ZM) + (ZN * BDW))) + (AWG * AKF)) + (AWB + (Z * AID));
                BCL = BDX;
                BCM = BDY;
            } else {
                BCL = S;
                BCM = T;
            }
            let BCN = YQ - AXL;
            let BCO = Lanes([0.0, 0.0, YR[0], YR[1], YR[2], YR[3]]);
            let BCP = BCO - AXM;
            let BCQ = AUH * ZK;
            let BCR = (AUI * ZK) + Lanes([0.0, 0.0, (ZL * AUH), 0.0, 0.0, 0.0]);
            let BCT = (BCS * BCN) / BCQ;
            let BCU = ((BCP * BCS) - (BCR * BCT)) / BCQ;
            let BCX = (BCW - (BCV * BCN)) / BCQ;
            let BCY = (((BCP * BCV) * DW) - (BCR * BCX)) / BCQ;
            let BCZ = if BCT > JF { 1.0 } else { 0.0 };
            let BEA;
            let BEB;
            if BCZ != 0.0 {
                BEA = BCN;
                BEB = BCP;
            } else {
                let BDZ = if BCX > JF { 1.0 } else { 0.0 };
                let BEU;
                let BEV;
                if BDZ != 0.0 {
                    let BEF = (BCN - BCW) / BCQ;
                    let BEG = BEF.exp();
                    let BEH = (ZK * TG) / RS;
                    let BEI = BEH * BEG;
                    let BEJ = Lanes([0.0, 0.0, ((((ZL * TG) + (TN * ZK)) / RS) * BEG), 0.0, 0.0, 0.0]) + ((((BCP - (BCR * BEF)) / BCQ) * BEG) * BEH);
                    BEU = BEI;
                    BEV = BEJ;
                } else {
                    let BEK = BCT.exp();
                    let BEL = AA + BEK;
                    let BEM = BEL.ln();
                    let BEN = ZK * TG;
                    let BEO = (-RS) / BEN;
                    let BEP = BCX.exp();
                    let BEQ = (BEO * BEP) * BCV;
                    let BER = BCS - ((BCQ * BEQ) / BCV);
                    let BES = (BCQ * BEM) / BER;
                    let BET = (((BCR * BEM) + (((BCU * BEK) * (EA / BEL)) * BCQ)) - (((((BCR * BEQ) + (((Lanes([0.0, 0.0, ((((((ZL * TG) + (TN * ZK)) * BEO) * DW) / BEN) * BEP), 0.0, 0.0, 0.0]) + ((BCY * BEP) * BEO)) * BCV) * BCQ)) / BCV) * DW) * BES)) / BER;
                    BEU = BES;
                    BEV = BET;
                }
                BEA = BEU;
                BEB = BEV;
            }
            let BEC = BEA + (EG * ZK);
            let BED = BEB + Lanes([0.0, 0.0, (ZL * EG), 0.0, 0.0, 0.0]);
            let BFA;
            let BFB;
            if BEE != 0.0 {
                BFA = AA;
                BFB = ATP;
            } else {
                let BEW = staged[236] / BEC;
                let BEX = AA + BEW;
                let BEY = AA / BEX;
                let BEZ = (((((BED * BEW) * DW) / BEC) * BEY) * DW) / BEX;
                BFA = BEY;
                BFB = BEZ;
            }
            let BFC = ABN - BH;
            let BFD = ABO - Lanes([0.0, 0.0, CL, 0.0, 0.0, 0.0]);
            let BFH = staged[240] - (BFG * ((BFE * BEA) + (BFF * BFC)));
            let BFI = (((BEB * BFE) + (BFD * BFF)) * BFG) * DW;
            let BFK = if BFH < BFJ { 1.0 } else { 0.0 };
            let BFQ;
            let BFR;
            if BFK != 0.0 {
                let BFL = 6e-8f64 - (EG * BFH);
                let BFM = AA / BFL;
                let BFN = BFJ * (4e-8f64 - BFH);
                let BFO = BFN * BFM;
                let BFP = (((BFI * DW) * BFJ) * BFM) + ((((((BFI * EG) * DW) * BFM) * DW) / BFL) * BFN);
                BFQ = BFO;
                BFR = BFP;
            } else {
                BFQ = BFH;
                BFR = BFI;
            }
            let BFX;
            let BFY;
            if C != 0.0 {
                BFX = S;
                BFY = ATP;
            } else {
                let BFU = (BFS * BEA) + (BFT * BFC);
                let BFV = (BEB * BFS) + (BFD * BFT);
                let BFW = if BFU >= -9e-1f64 { 1.0 } else { 0.0 };
                let BGM;
                let BGN;
                if BFW != 0.0 {
                    let BGA = AA + BFU;
                    let BGB = BO * BGA;
                    let BGC = Lanes([0.0, 0.0, (CS * BGA), 0.0, 0.0, 0.0]) + (BFV * BO);
                    BGM = BGB;
                    BGN = BGC;
                } else {
                    let BGF = BGE + (BGD * BFU);
                    let BGG = AA / BGF;
                    let BGI = BGH + BFU;
                    let BGJ = BO * BGI;
                    let BGK = BGJ * BGG;
                    let BGL = ((Lanes([0.0, 0.0, (CS * BGI), 0.0, 0.0, 0.0]) + (BFV * BO)) * BGG) + (((((BFV * BGD) * BGG) * DW) / BGF) * BGJ);
                    BGM = BGK;
                    BGN = BGL;
                }
                BFX = BGM;
                BFY = BGN;
            }
            let BGR;
            let BGS;
            if BFZ != 0.0 {
                let BGQ = (BGO + BFX) + BGP;
                BGR = BGQ;
                BGS = BFY;
            } else {
                BGR = BFX;
                BGS = BFY;
            }
            let BGY;
            let BGZ;
            let BHA;
            let BHB;
            let BHC;
            if BGT != 0.0 {
                BGY = AA;
                BGZ = AA;
                BHA = S;
                BHB = ATP;
                BHC = ATP;
            } else {
                let BGV = BGU * AAI;
                let BGW = AAJ * BGU;
                let BGX = if BGV >= -5e-1f64 { 1.0 } else { 0.0 };
                let BHK;
                let BHL;
                let BHM;
                if BGX != 0.0 {
                    let BHE = AA + BGV;
                    let BHF = AA / BHE;
                    let BHG = ((BGW * BHF) * DW) / BHE;
                    BHK = BHF;
                    BHL = S;
                    BHM = BHG;
                } else {
                    let BHI = BHH * BGV;
                    let BHJ = BGW * BHH;
                    BHK = BHI;
                    BHL = BHH;
                    BHM = BHJ;
                }
                let BHO = BG + BHN;
                let BHP = (AAI * BHK) / BHO;
                let BHQ = (((AAJ * BHK) + (BHM * AAI)) - Lanes([0.0, 0.0, (CK * BHP), 0.0, 0.0, 0.0])) / BHO;
                let BHR = if BHP < YE { 1.0 } else { 0.0 };
                let BHZ;
                let BIA;
                let BIB;
                if BHR != 0.0 {
                    let BHS = (AA - BHP).sqrt();
                    let BHT = AA / BHS;
                    let BHU = ((((BHQ * DW) * (EA / (DZ * BHS))) * BHT) * DW) / BHS;
                    BHZ = BHT;
                    BIA = BHL;
                    BIB = BHU;
                } else {
                    let BHW = BHQ * BHV;
                    let BHY = (BHV * BHP) + BHX;
                    BHZ = BHY;
                    BIA = BHX;
                    BIB = BHW;
                }
                let BIC = BHO.sqrt();
                let BID = staged[247] / BIC;
                let BIE = BID * BHZ;
                let BIF = Lanes([0.0, 0.0, (((((CK * (EA / (DZ * BIC))) * BID) * DW) / BIC) * BHZ), 0.0, 0.0, 0.0]) + (BIB * BID);
                let BIH = (BIG * ABP).sqrt();
                let BII = AHB + (EG * BIH);
                let BIJ = AHB / BII;
                let BIK = (((((ABQ * BIG) * (EA / (DZ * BIH))) * EG) * BIJ) * DW) / BII;
                let BIM = (BIL * BIJ) + staged[250];
                let BIN = BIJ * BIJ;
                let BIO = BIK * BIJ;
                let BIP = (BIF * BIM) + ((BIK * BIL) * BIE);
                let BIQ = AA + (BIE * BIM);
                let BIS = BIR * (BIJ * BIN);
                let BIT = -BIE;
                let BIU = BIT * BIS;
                let BIV = BIQ + (BIU * BEA);
                let BIW = BIP + (((((BIF * DW) * BIS) + ((((BIK * BIN) + ((BIO + BIO) * BIJ)) * BIR) * BIT)) * BEA) + (BEB * BIU));
                BGY = BIQ;
                BGZ = BIV;
                BHA = BIA;
                BHB = BIP;
                BHC = BIW;
            }
            let BHD = if BGY < AOA { 1.0 } else { 0.0 };
            let BJD;
            let BJE;
            if BHD != 0.0 {
                let BIY = AEC - (BIX * BGY);
                let BIZ = AA / BIY;
                let BJA = ADI - BGY;
                let BJB = BJA * BIZ;
                let BJC = ((BHB * DW) * BIZ) + ((((((BHB * BIX) * DW) * BIZ) * DW) / BIY) * BJA);
                BJD = BJB;
                BJE = BJC;
            } else {
                BJD = BGY;
                BJE = BHB;
            }
            let BJF = if BGZ < AOA { 1.0 } else { 0.0 };
            let BJL;
            let BJM;
            if BJF != 0.0 {
                let BJG = AEC - (BIX * BGZ);
                let BJH = AA / BJG;
                let BJI = ADI - BGZ;
                let BJJ = BJI * BJH;
                let BJK = ((BHC * DW) * BJH) + ((((((BHC * BIX) * DW) * BJH) * DW) / BJG) * BJI);
                BJL = BJJ;
                BJM = BJK;
            } else {
                BJL = BGZ;
                BJM = BHC;
            }
            let BJP;
            let BJQ;
            if BGT != 0.0 {
                BJP = AA;
                BJQ = BHA;
            } else {
                let BJN = BGU * ABE;
                let BJO = if BJN >= -5e-1f64 { 1.0 } else { 0.0 };
                let BJV;
                let BJW;
                if BJO != 0.0 {
                    let BJS = AA / (AA + BJN);
                    BJV = BJS;
                    BJW = BHA;
                } else {
                    let BJU = BJT * BJN;
                    BJV = BJU;
                    BJW = BJT;
                }
                let BJX = BG + BHN;
                let BJY = (ABE * BJV) / BJX;
                let BJZ = if BJY < YE { 1.0 } else { 0.0 };
                let BKD;
                let BKE;
                if BJZ != 0.0 {
                    let BKA = AA / ((AA - BJY).sqrt());
                    BKD = BKA;
                    BKE = BJW;
                } else {
                    let BKC = (1.414213562373095e0f64 * BJY) + BKB;
                    BKD = BKC;
                    BKE = BKB;
                }
                let BKF = AA + (((staged[252] / (BJX.sqrt())) * BKD) * ((BIL * (AHB / (AHB + (EG * ((BIG * AXP).sqrt()))))) + staged[253]));
                BJP = BKF;
                BJQ = BKE;
            }
            let BJR = if BJP < AOA { 1.0 } else { 0.0 };
            let BKJ;
            let BKK;
            if A != 0.0 {
                let BKH = BKG * ((staged[254] - (YE * BP)) + 4.5e-1f64);
                let BKI = ((CT * YE) * DW) * BKG;
                BKJ = BKH;
                BKK = BKI;
            } else {
                BKJ = S;
                BKK = T;
            }
            let BKT;
            let BKU;
            if BKL != 0.0 {
                let BKN = (((BEA + AXL) + AXL) - BKJ) / BKM;
                let BKO = (((BEB + AXM) + AXM) - Lanes([0.0, 0.0, BKK, 0.0, 0.0, 0.0])) / BKM;
                let BKP = (TH + (TI * AAV)) + (BS * BKN);
                let BKQ = BKN * BKP;
                let BKR = (BKO * BKP) + (((Lanes([0.0, 0.0, TO, 0.0, 0.0, 0.0]) + (Lanes([0.0, 0.0, (TP * AAV), 0.0, 0.0, 0.0]) + (AAW * TI))) + (Lanes([0.0, 0.0, (CW * BKN), 0.0, 0.0, 0.0]) + (BKO * BS))) * BKN);
                BKT = BKQ;
                BKU = BKR;
            } else {
                let BLD;
                let BLE;
                if BKS != 0.0 {
                    let BKW = BEA - BKJ;
                    let BKX = BEB - Lanes([0.0, 0.0, BKK, 0.0, 0.0, 0.0]);
                    let BKY = BKW / AII;
                    let BKZ = (TH + (TI * AAV)) + ((BS * BKW) / AII);
                    let BLA = BKY * BKZ;
                    let BLB = ((BKX / AII) * BKZ) + (((Lanes([0.0, 0.0, TO, 0.0, 0.0, 0.0]) + (Lanes([0.0, 0.0, (TP * AAV), 0.0, 0.0, 0.0]) + (AAW * TI))) + ((Lanes([0.0, 0.0, (CW * BKW), 0.0, 0.0, 0.0]) + (BKX * BS)) / AII)) * BKY);
                    BLD = BLA;
                    BLE = BLB;
                } else {
                    let BLQ;
                    let BLR;
                    if BLC != 0.0 {
                        let BLF = AA + (TI * AAV);
                        let BLG = (((BEA + AXL) + AXL) - BKJ) / BKM;
                        let BLH = (((BEB + AXM) + AXM) - Lanes([0.0, 0.0, BKK, 0.0, 0.0, 0.0])) / BKM;
                        let BLI = TH + (BS * BLG);
                        let BLJ = BLG * BLI;
                        let BLK = BLJ * BLF;
                        let BLL = (((BLH * BLI) + ((Lanes([0.0, 0.0, TO, 0.0, 0.0, 0.0]) + (Lanes([0.0, 0.0, (CW * BLG), 0.0, 0.0, 0.0]) + (BLH * BS))) * BLG)) * BLF) + ((Lanes([0.0, 0.0, (TP * AAV), 0.0, 0.0, 0.0]) + (AAW * TI)) * BLJ);
                        BLQ = BLK;
                        BLR = BLL;
                    } else {
                        let BLN = (((BEA + staged[257]) * SK) / AII) / BLM;
                        let BLO = ((BEB * SK) / AII) / BLM;
                        let BLP = if BLN > FJ { 1.0 } else { 0.0 };
                        let BLV;
                        let BLW;
                        if BLP != 0.0 {
                            let BLS = BLN.ln();
                            let BLT = BLO * (EA / BLN);
                            BLV = BLS;
                            BLW = BLT;
                        } else {
                            BLV = BLU;
                            BLW = ATP;
                        }
                        let BLY = (BLX * BLV).exp();
                        let BLZ = (BLW * BLX) * BLY;
                        let BMA = TH + (TI * AAV);
                        let BMB = Lanes([0.0, 0.0, TO, 0.0, 0.0, 0.0]) + (Lanes([0.0, 0.0, (TP * AAV), 0.0, 0.0, 0.0]) + (AAW * TI));
                        let BME = BMD * (Y.powf(BMC));
                        let BMF = (Z * (BMC * (Y.powf(staged[438])))) * BMD;
                        let BMI = BMH * (Y.powf(BMG));
                        let BMJ = (Z * (BMG * (Y.powf(staged[439])))) * BMH;
                        let BML = BEB / BMK;
                        let BMM = AA + (BEA / BMK);
                        let BMN = if BMM > FJ { 1.0 } else { 0.0 };
                        let BMR;
                        let BMS;
                        if BMN != 0.0 {
                            let BMO = BMM.ln();
                            let BMP = BML * (EA / BMM);
                            BMR = BMO;
                            BMS = BMP;
                        } else {
                            BMR = BMQ;
                            BMS = ATP;
                        }
                        let BMT = (BME * BMR).exp();
                        let BMU = BMI / BMT;
                        let BMV = (BLY * BMA) + BMU;
                        let BMW = ((BLZ * BMA) + (BMB * BLY)) + ((Lanes([0.0, 0.0, BMJ, 0.0, 0.0, 0.0]) - (((Lanes([0.0, 0.0, (BMF * BMR), 0.0, 0.0, 0.0]) + (BMS * BME)) * BMT) * BMU)) / BMT);
                        BLQ = BMV;
                        BLR = BMW;
                    }
                    BLD = BLQ;
                    BLE = BLR;
                }
                BKT = BLD;
                BKU = BLE;
            }
            let BKV = if BKT >= -8e-1f64 { 1.0 } else { 0.0 };
            let BNE;
            let BNF;
            if BKV != 0.0 {
                let BMX = AA + BKT;
                BNE = BMX;
                BNF = BKU;
            } else {
                let BMZ = 7e0f64 + (BMY * BKT);
                let BNA = AA / BMZ;
                let BNB = 6e-1f64 + BKT;
                let BNC = BNB * BNA;
                let BND = (BKU * BNA) + (((((BKU * BMY) * BNA) * DW) / BMZ) * BNB);
                BNE = BNC;
                BNF = BND;
            }
            let BNG = BT / BNE;
            let BNH = (Lanes([0.0, 0.0, CX, 0.0, 0.0, 0.0]) - (BNF * BNG)) / BNE;
            let BNI = (BFQ * BU) * RS;
            let BNJ = BNI * BGR;
            let BNK = ((((BFR * BU) + Lanes([0.0, 0.0, (CY * BFQ), 0.0, 0.0, 0.0])) * RS) * BGR) + (BGS * BNI);
            let BNL = (EG * BU) / BNG;
            let BNM = BNL * AHB;
            let BNN = ((Lanes([0.0, 0.0, (CY * EG), 0.0, 0.0, 0.0]) - (BNH * BNL)) / BNG) * AHB;
            let BNR;
            let BNS;
            if BNO != 0.0 {
                BNR = BNP;
                BNS = ATP;
            } else {
                let BOI;
                let BOJ;
                if BNQ != 0.0 {
                    let BNW = (BEB * BNV) * DW;
                    let BNX = (staged[265] - (BNV * BEA)) - AIQ;
                    let BNY = BNW * BNX;
                    let BNZ = ((BNX * BNX) + staged[266]).sqrt();
                    let BOA = staged[267] - (YE * (BNX + BNZ));
                    let BOB = ((BNW + ((BNY + BNY) * (EA / (DZ * BNZ)))) * YE) * DW;
                    BOI = BOA;
                    BOJ = BOB;
                } else {
                    let BOC = BEB * BNV;
                    let BOD = (BNP + (BNV * BEA)) - AIQ;
                    let BOE = BOC * BOD;
                    let BOF = ((BOD * BOD) + staged[269]).sqrt();
                    let BOG = YE * (BOD + BOF);
                    let BOH = (BOC + ((BOE + BOE) * (EA / (DZ * BOF)))) * YE;
                    BOI = BOG;
                    BOJ = BOH;
                }
                BNR = BOI;
                BNS = BOJ;
            }
            let BNT = BJL / BEC;
            let BNU = if (if BGR == S { 1.0 } else { 0.0 }) != 0.0 && (if BNR == AA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BPH;
            let BPI;
            if BNU != 0.0 {
                let BOK = (BJL * BNM) + BEC;
                let BOL = AA / BOK;
                let BOM = BNM * BEC;
                let BON = BOM * BOL;
                let BOO = (((BNN * BEC) + (BED * BNM)) * BOL) + (((((((BJM * BNM) + (BNN * BJL)) + BED) * BOL) * DW) / BOK) * BOM);
                BPH = BON;
                BPI = BOO;
            } else {
                let BOP = BJL * BNJ;
                let BOQ = (BJM * BNJ) + (BNK * BJL);
                let BOR = EG * BJL;
                let BOS = AA / BNR;
                let BOT = (BOP - AA) + BOS;
                let BOU = BOR * BOT;
                let BOV = ((BJM * EG) * BOT) + ((BOQ + (((BNS * BOS) * DW) / BNR)) * BOR);
                let BOW = EG / BNR;
                let BOX = BOW - AA;
                let BOY = ((BEC * BOX) + (BJL * BNM)) + (AEC * (BEC * BOP));
                let BOZ = (((BED * BOX) + ((((BNS * BOW) * DW) / BNR) * BEC)) + ((BJM * BNM) + (BNN * BJL))) + (((BED * BOP) + (BOQ * BEC)) * AEC);
                let BPA = BNM + (EG * (BEC * BNJ));
                let BPB = BEC * BPA;
                let BPC = BOZ * BOY;
                let BPD = EG * BOU;
                let BPE = ((BOY * BOY) - (BPD * BPB)).sqrt();
                let BPF = (BOY - BPE) / BOU;
                let BPG = ((BOZ - (((BPC + BPC) - (((BOV * EG) * BPB) + (((BED * BPA) + ((BNN + (((BED * BNJ) + (BNK * BEC)) * EG)) * BEC)) * BPD))) * (EA / (DZ * BPE)))) - (BOV * BPF)) / BOU;
                BPH = BPF;
                BPI = BPG;
            }
            let BPJ = Lanes([0.0, 0.0, 0.0, XO[0], XO[1], 0.0]);
            let BPK = BPI - BPJ;
            let BPL = (BPH - WR) - staged[270];
            let BPM = BPK * BPL;
            let BPO = ((BPL * BPL) + (BPN * BPH)).sqrt();
            let BPP = BPH - (YE * (BPL + BPO));
            let BPQ = BPI - ((BPK + (((BPM + BPM) + (BPI * BPN)) * (EA / (DZ * BPO)))) * YE);
            let BPR = if BPP > WR { 1.0 } else { 0.0 };
            let BPS;
            let BPT;
            if BPR != 0.0 {
                BPS = WR;
                BPT = BPJ;
            } else {
                BPS = BPP;
                BPT = BPQ;
            }
            let BPU = WR - BPS;
            let BPV = BPJ - BPT;
            let BPW = YE * BJL;
            let BPX = BJM * YE;
            let BPY = (BPW * BPH) / BEC;
            let BPZ = AA - BPY;
            let BQA = EG * (BNJ * BEA);
            let BQB = EG / BNR;
            let BQC = (BQB - AA) + (BNJ * BJL);
            let BQD = ((BNM + BPH) + (BQA * BPZ)) / BQC;
            let BQE = (((BNN + BPI) + (((((BNK * BEA) + (BEB * BNJ)) * EG) * BPZ) + ((((((BPX * BPH) + (BPI * BPW)) - (BED * BPY)) / BEC) * DW) * BQA))) - (((((BNS * BQB) * DW) / BNR) + ((BNK * BJL) + (BJM * BNJ))) * BQD)) / BQC;
            let BQF = if staged[272] != 0.0 && (if BPU > 1e-10f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BQP;
            let BQQ;
            if BQF != 0.0 {
                let BQI = (BQG * BJL) * BQH;
                let BQJ = AA / BQI;
                let BQK = BEA / BNM;
                let BQL = AHB * (BJL + BQK);
                let BQM = BQJ * BQL;
                let BQN = BQM * BPU;
                let BQO = ((((((((BJM * BQG) * BQH) * BQJ) * DW) / BQI) * BQL) + (((BJM + ((BEB - (BNN * BQK)) / BNM)) * AHB) * BQJ)) * BPU) + (BPV * BQM);
                BQP = BQN;
                BQQ = BQO;
            } else {
                BQP = JH;
                BQQ = ATP;
            }
            let BQR = if TJ > S { 1.0 } else { 0.0 };
            let BRC;
            let BRD;
            if BQR != 0.0 {
                let BQS = BJL * BPH;
                let BQT = (BJM * BPH) + (BPI * BJL);
                let BQU = BEC + BQS;
                let BQV = (BEC * BQS) / BQU;
                let BQW = (BEC - BQV) / TJ;
                let BQX = ((BED - ((((BED * BQS) + (BQT * BEC)) - ((BED + BQT) * BQV)) / BQU)) - Lanes([0.0, 0.0, (TQ * BQW), 0.0, 0.0, 0.0])) / TJ;
                let BQZ = BQY * AAV;
                let BRA = AAW * BQY;
                let BRB = if BQZ >= -9e-1f64 { 1.0 } else { 0.0 };
                let BRS;
                let BRT;
                if BRB != 0.0 {
                    let BRI = AA + BQZ;
                    let BRJ = AA / BRI;
                    let BRK = BQW * BRJ;
                    let BRL = (BQX * BRJ) + ((((BRA * BRJ) * DW) / BRI) * BQW);
                    BRS = BRK;
                    BRT = BRL;
                } else {
                    let BRM = BGH + BQZ;
                    let BRN = AA / BRM;
                    let BRO = BGE + (BGD * BQZ);
                    let BRP = BRO * BRN;
                    let BRQ = BQW * BRP;
                    let BRR = (BQX * BRP) + ((((BRA * BGD) * BRN) + ((((BRA * BRN) * DW) / BRM) * BRO)) * BQW);
                    BRS = BRQ;
                    BRT = BRR;
                }
                BRC = BRS;
                BRD = BRT;
            } else {
                BRC = JH;
                BRD = ATP;
            }
            let BRF = BRE * WR;
            let BRG = XO * BRE;
            let BRH = if BRF > JF { 1.0 } else { 0.0 };
            let BRW;
            let BRX;
            if BRH != 0.0 {
                BRW = JH;
                BRX = AGV;
            } else {
                let BRU = BRF.exp();
                let BRV = BRG * BRU;
                BRW = BRU;
                BRX = BRV;
            }
            let BSF;
            let BSG;
            if BRY != 0.0 {
                let BSB = (AA + (BRZ * BRW)) / BSA;
                let BSC = BSB * BFA;
                let BSD = ((BRX * BRZ) / BSA) * BFA;
                let BSE = Lanes([0.0, 0.0, 0.0, BSD[0], BSD[1], 0.0]) + (BFB * BSB);
                BSF = BSC;
                BSG = BSE;
            } else {
                BSF = JH;
                BSG = ATP;
            }
            let BSH = staged[279] / BNM;
            let BSI = BSH * BEA;
            let BSJ = ((((BNN * BSH) * DW) / BNM) * BEA) + (BEB * BSH);
            let BSK = if BSI > -9e-1f64 { 1.0 } else { 0.0 };
            let BSR;
            let BSS;
            if BSK != 0.0 {
                let BSL = AA + BSI;
                BSR = BSL;
                BSS = BSJ;
            } else {
                let BSM = BGE + (BGD * BSI);
                let BSN = AA / BSM;
                let BSO = BGH + BSI;
                let BSP = BSO * BSN;
                let BSQ = (BSJ * BSN) + (((((BSJ * BGD) * BSN) * DW) / BSM) * BSO);
                BSR = BSP;
                BSS = BSQ;
            }
            let BST = BQP + BRC;
            let BSU = (BQP * BRC) / BST;
            let BSV = (((BQQ * BRC) + (BRD * BQP)) - ((BQQ + BRD) * BSU)) / BST;
            let BSW = BSU + BSF;
            let BSX = (BSU * BSF) / BSW;
            let BSY = BQD + (BSR * BSX);
            let BSZ = (RS * BFQ) / AHB;
            let BTA = BNG * BSZ;
            let BTB = (BNH * BSZ) + (((BFR * RS) / AHB) * BNG);
            let BTC = (BPW * BPS) / BEC;
            let BTD = AA - BTC;
            let BTE = BEA * BTD;
            let BTF = BPS / BNM;
            let BTG = AA + BTF;
            let BTH = (BTA * BTE) / BTG;
            let BTI = (((BTB * BTE) + (((BEB * BTD) + ((((((BPX * BPS) + (BPT * BPW)) - (BED * BTC)) / BEC) * DW) * BEA)) * BTA)) - (((BPT - (BNN * BTF)) / BNM) * BTH)) / BTG;
            let BTJ = (BTI * BGR) + (BGS * BTH);
            let BTK = AA + (BTH * BGR);
            let BTL = BPS / BTK;
            let BTM = BTH * BTL;
            let BTN = BTH / BTK;
            let BTO = BPU / BSY;
            let BTP = (BPV - ((BQE + ((BSS * BSX) + (((((BSV * BSF) + (BSG * BSU)) - ((BSV + BSG) * BSX)) / BSW) * BSR))) * BTO)) / BSY;
            let BTQ = AA + BTO;
            let BTS = (BTM * BTQ) / BTR;
            let BTT = ((((BTI * BTL) + (((BPT - (BTJ * BTL)) / BTK) * BTH)) * BTQ) + (BTP * BTM)) / BTR;
            let BTU = (BTN * BTQ) / BTR;
            let BTV = ((((BTI - (BTJ * BTN)) / BTK) * BTQ) + (BTP * BTN)) / BTR;
            let BTW = if BTU < OZ { 1.0 } else { 0.0 };
            let BUC;
            let BUD;
            let BUE;
            let BUF;
            let BUG;
            let BUH;
            let BUI;
            let BUJ;
            let BUK;
            let BUL;
            let BUM;
            let BUN;
            let BUO;
            let BUP;
            if BTX != 0.0 {
                let BUS;
                let BUT;
                let BUU;
                let BUV;
                if BUR != 0.0 {
                    let BVJ;
                    let BVK;
                    if B != 0.0 {
                        let BVB = XO * DW;
                        let BVD = (((-WR) - ZG) - WS) / BVC;
                        let BVE = (Lanes([0.0, BVB[0], BVB[1], 0.0]) - ZH) / BVC;
                        BVJ = BVD;
                        BVK = BVE;
                    } else {
                        let BVF = XO * DW;
                        let BVH = ((((-WR) - ZG) - WS) + BVG) / BVC;
                        let BVI = (Lanes([0.0, BVF[0], BVF[1], 0.0]) - ZH) / BVC;
                        BVJ = BVH;
                        BVK = BVI;
                    }
                    let BVL = if (if (if WT <= S { 1.0 } else { 0.0 }) != 0.0 || (if WU <= S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if WV < S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BWM;
                    let BWN;
                    if BVL != 0.0 {
                        BWM = S;
                        BWN = ATP;
                    } else {
                        let BVM = BVK * BVJ;
                        let BVN = ((BVJ * BVJ) + 4e-4f64).sqrt();
                        let BVO = YE * (BVJ + BVN);
                        let BVP = (BVK + ((BVM + BVM) * (EA / (DZ * BVN)))) * YE;
                        let BVQ = BVO + ZY;
                        let BVR = WU / BVQ;
                        let BVS = WW * WT;
                        let BVT = BVS * BVO;
                        let BVU = (-BVR).exp();
                        let BVV = BVT * BVU;
                        let BVW = ZT * ZT;
                        let BVX = ZW * ZT;
                        let BVY = -ZT;
                        let BVZ = BVY * BVW;
                        let BWA = ((ZW * DW) * BVW) + ((BVX + BVX) * BVY);
                        let BWC = (WV + (BVZ.abs())) + OZ;
                        let BWD = BVZ / BWC;
                        let BWE = (BWA - ((BWA * ((DZ * (if BVZ >= BWB { 1.0 } else { 0.0 })) - EA)) * BWD)) / BWC;
                        let BWF = BWE * BWD;
                        let BWG = ((BWD * BWD) + 4e-12f64).sqrt();
                        let BWI = (YE * (BWD + BWG)) - BWH;
                        let BWJ = BVV * BWI;
                        let BWK = (((BVP * BVS) * BVU) + ((((((BVP * BVR) * DW) / BVQ) * DW) * BVU) * BVT)) * BWI;
                        let BWL = Lanes([0.0, 0.0, BWK[0], BWK[1], BWK[2], BWK[3]]) + (((BWE + ((BWF + BWF) * (EA / (DZ * BWG)))) * YE) * BVV);
                        BWM = BWJ;
                        BWN = BWL;
                    }
                    let BWS;
                    let BWT;
                    if B != 0.0 {
                        let BWO = ((WR - YQ) - WX) / BVC;
                        let BWP = (Lanes([0.0, XO[0], XO[1], 0.0]) - YR) / BVC;
                        BWS = BWO;
                        BWT = BWP;
                    } else {
                        let BWQ = (((WR - YQ) - WX) + BVG) / BVC;
                        let BWR = (Lanes([0.0, XO[0], XO[1], 0.0]) - YR) / BVC;
                        BWS = BWQ;
                        BWT = BWR;
                    }
                    let BWU = if (if (if WY <= S { 1.0 } else { 0.0 }) != 0.0 || (if WZ <= S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if XA < S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BXU;
                    let BXV;
                    if BWU != 0.0 {
                        BXU = S;
                        BXV = BUB;
                    } else {
                        let BWV = BWT * BWS;
                        let BWW = ((BWS * BWS) + 4e-4f64).sqrt();
                        let BWX = YE * (BWS + BWW);
                        let BWY = (BWT + ((BWV + BWV) * (EA / (DZ * BWW)))) * YE;
                        let BWZ = BWX + ZY;
                        let BXA = WZ / BWZ;
                        let BXB = XB * WY;
                        let BXC = BXB * BWX;
                        let BXD = (-BXA).exp();
                        let BXE = BXC * BXD;
                        let BXF = XC * XC;
                        let BXG = XP * XC;
                        let BXH = -XC;
                        let BXI = BXH * BXF;
                        let BXJ = ((XP * DW) * BXF) + ((BXG + BXG) * BXH);
                        let BXK = (XA + (BXI.abs())) + OZ;
                        let BXL = BXI / BXK;
                        let BXM = (BXJ - ((BXJ * ((DZ * (if BXI >= BWB { 1.0 } else { 0.0 })) - EA)) * BXL)) / BXK;
                        let BXN = BXM * BXL;
                        let BXO = ((BXL * BXL) + 4e-12f64).sqrt();
                        let BXP = (YE * (BXL + BXO)) - BWH;
                        let BXQ = BXE * BXP;
                        let BXR = (((BWY * BXB) * BXD) + ((((((BWY * BXA) * DW) / BWZ) * DW) * BXD) * BXC)) * BXP;
                        let BXS = ((BXM + ((BXN + BXN) * (EA / (DZ * BXO)))) * YE) * BXE;
                        let BXT = Lanes([0.0, BXR[0], BXR[1], BXR[2], BXR[3]]) + Lanes([BXS[0], 0.0, BXS[1], BXS[2], 0.0]);
                        BXU = BXQ;
                        BXV = BXT;
                    }
                    BUS = BXU;
                    BUT = BWM;
                    BUU = BXV;
                    BUV = BWN;
                } else {
                    let BYC;
                    let BYD;
                    if B != 0.0 {
                        let BXW = XO * DW;
                        let BXX = (((-WR) - (XD * ZG)) - WS) / BVC;
                        let BXY = (Lanes([0.0, BXW[0], BXW[1], 0.0]) - (ZH * XD)) / BVC;
                        BYC = BXX;
                        BYD = BXY;
                    } else {
                        let BXZ = XO * DW;
                        let BYA = ((((-WR) - (XD * ZG)) - WS) + BVG) / BVC;
                        let BYB = (Lanes([0.0, BXZ[0], BXZ[1], 0.0]) - (ZH * XD)) / BVC;
                        BYC = BYA;
                        BYD = BYB;
                    }
                    let BYE = if (if (if WT <= S { 1.0 } else { 0.0 }) != 0.0 || (if WU <= S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if WV < S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BYS;
                    let BYT;
                    if BYE != 0.0 {
                        BYS = S;
                        BYT = ATP;
                    } else {
                        let BYF = BYD * BYC;
                        let BYG = ((BYC * BYC) + 4e-4f64).sqrt();
                        let BYH = YE * (BYC + BYG);
                        let BYI = (BYD + ((BYF + BYF) * (EA / (DZ * BYG)))) * YE;
                        let BYJ = BYH + ZY;
                        let BYK = WU / BYJ;
                        let BYL = WW * WT;
                        let BYM = BYL * BYH;
                        let BYN = (-BYK).exp();
                        let BYO = BYM * BYN;
                        let BYP = ((BYI * BYL) * BYN) + ((((((BYI * BYK) * DW) / BYJ) * DW) * BYN) * BYM);
                        let BYQ = ZT - XE;
                        let BYR = if BYQ >= -1e-2f64 { 1.0 } else { 0.0 };
                        let BYX;
                        let BYY;
                        if BYR != 0.0 {
                            let BYU = (-XF) * JF;
                            BYX = BYU;
                            BYY = ATP;
                        } else {
                            let BYV = XF / BYQ;
                            let BYW = ((ZW * BYV) * DW) / BYQ;
                            BYX = BYV;
                            BYY = BYW;
                        }
                        let BYZ = BYX.exp();
                        let BZA = BYO * BYZ;
                        let BZB = BYP * BYZ;
                        let BZC = Lanes([0.0, 0.0, BZB[0], BZB[1], BZB[2], BZB[3]]) + ((BYY * BYZ) * BYO);
                        BYS = BZA;
                        BYT = BZC;
                    }
                    let BZH;
                    let BZI;
                    if B != 0.0 {
                        let BZD = ((WR - (XG * YQ)) - WX) / BVC;
                        let BZE = (Lanes([0.0, XO[0], XO[1], 0.0]) - (YR * XG)) / BVC;
                        BZH = BZD;
                        BZI = BZE;
                    } else {
                        let BZF = (((WR - (XG * YQ)) - WX) + BVG) / BVC;
                        let BZG = (Lanes([0.0, XO[0], XO[1], 0.0]) - (YR * XG)) / BVC;
                        BZH = BZF;
                        BZI = BZG;
                    }
                    let BZJ = if (if (if WY <= S { 1.0 } else { 0.0 }) != 0.0 || (if WZ <= S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if XA < S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BZX;
                    let BZY;
                    if BZJ != 0.0 {
                        BZX = S;
                        BZY = BUB;
                    } else {
                        let BZK = BZI * BZH;
                        let BZL = ((BZH * BZH) + 4e-4f64).sqrt();
                        let BZM = YE * (BZH + BZL);
                        let BZN = (BZI + ((BZK + BZK) * (EA / (DZ * BZL)))) * YE;
                        let BZO = BZM + ZY;
                        let BZP = WZ / BZO;
                        let BZQ = XB * WY;
                        let BZR = BZQ * BZM;
                        let BZS = (-BZP).exp();
                        let BZT = BZR * BZS;
                        let BZU = ((BZN * BZQ) * BZS) + ((((((BZN * BZP) * DW) / BZO) * DW) * BZS) * BZR);
                        let BZV = XC - XH;
                        let BZW = if BZV >= -1e-2f64 { 1.0 } else { 0.0 };
                        let CAD;
                        let CAE;
                        if BZW != 0.0 {
                            let BZZ = (-XI) * JF;
                            CAD = BZZ;
                            CAE = CAA;
                        } else {
                            let CAB = XI / BZV;
                            let CAC = ((XP * CAB) * DW) / BZV;
                            CAD = CAB;
                            CAE = CAC;
                        }
                        let CAF = CAD.exp();
                        let CAG = BZT * CAF;
                        let CAH = BZU * CAF;
                        let CAI = (CAE * CAF) * BZT;
                        let CAJ = Lanes([0.0, CAH[0], CAH[1], CAH[2], CAH[3]]) + Lanes([CAI[0], 0.0, CAI[1], CAI[2], 0.0]);
                        BZX = CAG;
                        BZY = CAJ;
                    }
                    BUS = BZX;
                    BUT = BYS;
                    BUU = BZY;
                    BUV = BYT;
                }
                let BUW = ZK * JC;
                let BUX = UT / BUW;
                let BUY = Lanes([0.0, UU[0], UU[1]]);
                let BUZ = (BUY - Lanes([((ZL * JC) * BUX), 0.0, 0.0])) / BUW;
                let BVA = if BUX > JF { 1.0 } else { 0.0 };
                let CAN;
                let CAO;
                if BVA != 0.0 {
                    let CAK = JH * ((AA + BUX) - JF);
                    let CAL = BUZ * JH;
                    CAN = CAK;
                    CAO = CAL;
                } else {
                    let CAM = if BUX < -1e2f64 { 1.0 } else { 0.0 };
                    let CAW;
                    let CAX;
                    if CAM != 0.0 {
                        CAW = JO;
                        CAX = BTZ;
                    } else {
                        let CAU = BUX.exp();
                        let CAV = BUZ * CAU;
                        CAW = CAU;
                        CAX = CAV;
                    }
                    CAN = CAW;
                    CAO = CAX;
                }
                let CAP = ZK * LU;
                let CAQ = UX / CAP;
                let CAR = Lanes([0.0, UY[0], UY[1]]);
                let CAS = (CAR - Lanes([((ZL * LU) * CAQ), 0.0, 0.0])) / CAP;
                let CAT = if CAQ > JF { 1.0 } else { 0.0 };
                let CBB;
                let CBC;
                if CAT != 0.0 {
                    let CAY = JH * ((AA + CAQ) - JF);
                    let CAZ = CAS * JH;
                    CBB = CAY;
                    CBC = CAZ;
                } else {
                    let CBA = if CAQ < -1e2f64 { 1.0 } else { 0.0 };
                    let CBG;
                    let CBH;
                    if CBA != 0.0 {
                        CBG = JO;
                        CBH = BUA;
                    } else {
                        let CBE = CAQ.exp();
                        let CBF = CAS * CBE;
                        CBG = CBE;
                        CBH = CBF;
                    }
                    CBB = CBG;
                    CBC = CBH;
                }
                let CBD = if BW == S { 1.0 } else { 0.0 };
                let CBN;
                let CBO;
                if CBD != 0.0 {
                    CBN = S;
                    CBO = BTZ;
                } else {
                    let CBJ = CBI * BW;
                    let CBK = CAN - AA;
                    let CBL = CBJ * CBK;
                    let CBM = Lanes([((DA * CBI) * CBK), 0.0, 0.0]) + (CAO * CBJ);
                    CBN = CBL;
                    CBO = CBM;
                }
                let CBP = if BX == S { 1.0 } else { 0.0 };
                let CBV;
                let CBW;
                if CBP != 0.0 {
                    CBV = S;
                    CBW = BUA;
                } else {
                    let CBR = CBQ * BX;
                    let CBS = CBB - AA;
                    let CBT = CBR * CBS;
                    let CBU = Lanes([((DB * CBQ) * CBS), 0.0, 0.0]) + (CBC * CBR);
                    CBV = CBT;
                    CBW = CBU;
                }
                let CBX = if BY == S { 1.0 } else { 0.0 };
                let CCJ;
                let CCK;
                if CBX != 0.0 {
                    CCJ = S;
                    CCK = BTZ;
                } else {
                    let CBZ = CBY * KA;
                    let CCB = CBZ * (AA + (CCA * AB));
                    let CCC = CBY * staged[285];
                    let CCE = CCC * (AA + (CCD * AB));
                    let CCF = (Z * CCD) * CCC;
                    let CCG = UT / CCB;
                    let CCH = (BUY - Lanes([(((Z * CCA) * CBZ) * CCG), 0.0, 0.0])) / CCB;
                    let CCI = if CCG > JF { 1.0 } else { 0.0 };
                    let CCP;
                    let CCQ;
                    if CCI != 0.0 {
                        let CCM = JH * ((AA + CCG) - JF);
                        let CCN = CCH * JH;
                        CCP = CCM;
                        CCQ = CCN;
                    } else {
                        let CCO = if CCG < -1e2f64 { 1.0 } else { 0.0 };
                        let CCX;
                        let CCY;
                        if CCO != 0.0 {
                            CCX = JO;
                            CCY = BTZ;
                        } else {
                            let CCV = CCG.exp();
                            let CCW = CCH * CCV;
                            CCX = CCV;
                            CCY = CCW;
                        }
                        CCP = CCX;
                        CCQ = CCY;
                    }
                    let CCS = CCR - UT;
                    let CCT = UU * DW;
                    let CCU = if CCS < ZY { 1.0 } else { 0.0 };
                    let CDL;
                    let CDM;
                    if CCU != 0.0 {
                        let CCZ = (-UT) / CCE;
                        let CDB = (CCZ * CCR) * CDA;
                        let CDC = (((Lanes([0.0, CCT[0], CCT[1]]) - Lanes([(CCF * CCZ), 0.0, 0.0])) / CCE) * CCR) * CDA;
                        let CDD = if CDB > JF { 1.0 } else { 0.0 };
                        let CDU;
                        let CDV;
                        if CDD != 0.0 {
                            let CDR = JH * ((AA + CDB) - JF);
                            let CDS = CDC * JH;
                            CDU = CDR;
                            CDV = CDS;
                        } else {
                            let CDT = if CDB < -1e2f64 { 1.0 } else { 0.0 };
                            let CEA;
                            let CEB;
                            if CDT != 0.0 {
                                CEA = JO;
                                CEB = BTZ;
                            } else {
                                let CDY = CDB.exp();
                                let CDZ = CDC * CDY;
                                CEA = CDY;
                                CEB = CDZ;
                            }
                            CDU = CEA;
                            CDV = CEB;
                        }
                        let CDW = -CDU;
                        let CDX = CDV * DW;
                        CDL = CDW;
                        CDM = CDX;
                    } else {
                        let CDE = AA / CCS;
                        let CDF = (-UT) / CCE;
                        let CDG = CDF * CCR;
                        let CDH = CDG * CDE;
                        let CDI = (((CCT * CDE) * DW) / CCS) * CDG;
                        let CDJ = ((((Lanes([0.0, CCT[0], CCT[1]]) - Lanes([(CCF * CDF), 0.0, 0.0])) / CCE) * CCR) * CDE) + Lanes([0.0, CDI[0], CDI[1]]);
                        let CDK = if CDH > JF { 1.0 } else { 0.0 };
                        let CEF;
                        let CEG;
                        if CDK != 0.0 {
                            let CEC = JH * ((AA + CDH) - JF);
                            let CED = CDJ * JH;
                            CEF = CEC;
                            CEG = CED;
                        } else {
                            let CEE = if CDH < -1e2f64 { 1.0 } else { 0.0 };
                            let CEL;
                            let CEM;
                            if CEE != 0.0 {
                                CEL = JO;
                                CEM = BTZ;
                            } else {
                                let CEJ = CDH.exp();
                                let CEK = CDJ * CEJ;
                                CEL = CEJ;
                                CEM = CEK;
                            }
                            CEF = CEL;
                            CEG = CEM;
                        }
                        let CEH = -CEF;
                        let CEI = CEG * DW;
                        CDL = CEH;
                        CDM = CEI;
                    }
                    let CDN = CBI * BY;
                    let CDO = CCP + CDL;
                    let CDP = CDN * CDO;
                    let CDQ = Lanes([((DC * CBI) * CDO), 0.0, 0.0]) + ((CCQ + CDM) * CDN);
                    CCJ = CDP;
                    CCK = CDQ;
                }
                let CCL = if BZ == S { 1.0 } else { 0.0 };
                let CEV;
                let CEW;
                if CCL != 0.0 {
                    CEV = S;
                    CEW = BUA;
                } else {
                    let CEN = CBY * MT;
                    let CEO = CEN * (AA + (CCA * AB));
                    let CEP = CBY * staged[288];
                    let CEQ = CEP * (AA + (CCD * AB));
                    let CER = (Z * CCD) * CEP;
                    let CES = UX / CEO;
                    let CET = (CAR - Lanes([(((Z * CCA) * CEN) * CES), 0.0, 0.0])) / CEO;
                    let CEU = if CES > JF { 1.0 } else { 0.0 };
                    let CFB;
                    let CFC;
                    if CEU != 0.0 {
                        let CEY = JH * ((AA + CES) - JF);
                        let CEZ = CET * JH;
                        CFB = CEY;
                        CFC = CEZ;
                    } else {
                        let CFA = if CES < -1e2f64 { 1.0 } else { 0.0 };
                        let CFJ;
                        let CFK;
                        if CFA != 0.0 {
                            CFJ = JO;
                            CFK = BUA;
                        } else {
                            let CFH = CES.exp();
                            let CFI = CET * CFH;
                            CFJ = CFH;
                            CFK = CFI;
                        }
                        CFB = CFJ;
                        CFC = CFK;
                    }
                    let CFE = CFD - UX;
                    let CFF = UY * DW;
                    let CFG = if CFE < ZY { 1.0 } else { 0.0 };
                    let CFW;
                    let CFX;
                    if CFG != 0.0 {
                        let CFL = (-UX) / CEQ;
                        let CFM = (CFL * CFD) * CDA;
                        let CFN = (((Lanes([0.0, CFF[0], CFF[1]]) - Lanes([(CER * CFL), 0.0, 0.0])) / CEQ) * CFD) * CDA;
                        let CFO = if CFM > JF { 1.0 } else { 0.0 };
                        let CGF;
                        let CGG;
                        if CFO != 0.0 {
                            let CGC = JH * ((AA + CFM) - JF);
                            let CGD = CFN * JH;
                            CGF = CGC;
                            CGG = CGD;
                        } else {
                            let CGE = if CFM < -1e2f64 { 1.0 } else { 0.0 };
                            let CGL;
                            let CGM;
                            if CGE != 0.0 {
                                CGL = JO;
                                CGM = BUA;
                            } else {
                                let CGJ = CFM.exp();
                                let CGK = CFN * CGJ;
                                CGL = CGJ;
                                CGM = CGK;
                            }
                            CGF = CGL;
                            CGG = CGM;
                        }
                        let CGH = -CGF;
                        let CGI = CGG * DW;
                        CFW = CGH;
                        CFX = CGI;
                    } else {
                        let CFP = AA / CFE;
                        let CFQ = (-UX) / CEQ;
                        let CFR = CFQ * CFD;
                        let CFS = CFR * CFP;
                        let CFT = (((CFF * CFP) * DW) / CFE) * CFR;
                        let CFU = ((((Lanes([0.0, CFF[0], CFF[1]]) - Lanes([(CER * CFQ), 0.0, 0.0])) / CEQ) * CFD) * CFP) + Lanes([0.0, CFT[0], CFT[1]]);
                        let CFV = if CFS > JF { 1.0 } else { 0.0 };
                        let CGQ;
                        let CGR;
                        if CFV != 0.0 {
                            let CGN = JH * ((AA + CFS) - JF);
                            let CGO = CFU * JH;
                            CGQ = CGN;
                            CGR = CGO;
                        } else {
                            let CGP = if CFS < -1e2f64 { 1.0 } else { 0.0 };
                            let CGW;
                            let CGX;
                            if CGP != 0.0 {
                                CGW = JO;
                                CGX = BUA;
                            } else {
                                let CGU = CFS.exp();
                                let CGV = CFU * CGU;
                                CGW = CGU;
                                CGX = CGV;
                            }
                            CGQ = CGW;
                            CGR = CGX;
                        }
                        let CGS = -CGQ;
                        let CGT = CGR * DW;
                        CFW = CGS;
                        CFX = CGT;
                    }
                    let CFY = CBQ * BZ;
                    let CFZ = CFB + CFW;
                    let CGA = CFY * CFZ;
                    let CGB = Lanes([((DD * CBQ) * CFZ), 0.0, 0.0]) + ((CFC + CFX) * CFY);
                    CEV = CGA;
                    CEW = CGB;
                }
                let CEX = if (if CA == S { 1.0 } else { 0.0 }) != 0.0 && (if CB == S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CHD;
                let CHE;
                let CHF;
                let CHG;
                let CHH;
                let CHI;
                let CHJ;
                let CHK;
                let CHL;
                let CHM;
                if CEX != 0.0 {
                    CHD = S;
                    CHE = S;
                    CHF = S;
                    CHG = S;
                    CHH = S;
                    CHI = BTZ;
                    CHJ = BUA;
                    CHK = BTY;
                    CHL = BTZ;
                    CHM = BUA;
                } else {
                    let CGY = CAN - AA;
                    let CGZ = CC * CGY;
                    let CHA = Lanes([(DG * CGY), 0.0, 0.0]) + (CAO * CC);
                    let CHC = if CGZ < CHB { 1.0 } else { 0.0 };
                    let CHR;
                    let CHS;
                    let CHT;
                    let CHU;
                    if CHC != 0.0 {
                        CHR = AA;
                        CHS = S;
                        CHT = BTZ;
                        CHU = BTZ;
                    } else {
                        let CHO = (AA + CGZ).sqrt();
                        let CHP = AA / CHO;
                        let CHQ = (((CHA * (EA / (DZ * CHO))) * CHP) * DW) / CHO;
                        CHR = CHP;
                        CHS = CGZ;
                        CHT = CHQ;
                        CHU = CHA;
                    }
                    let CHV = CBB - AA;
                    let CHW = CD * CHV;
                    let CHX = Lanes([(DH * CHV), 0.0, 0.0]) + (CBC * CD);
                    let CHY = if CHW < CHB { 1.0 } else { 0.0 };
                    let CIC;
                    let CID;
                    let CIE;
                    let CIF;
                    if CHY != 0.0 {
                        CIC = AA;
                        CID = S;
                        CIE = BUA;
                        CIF = BUA;
                    } else {
                        let CHZ = (AA + CHW).sqrt();
                        let CIA = AA / CHZ;
                        let CIB = (((CHX * (EA / (DZ * CHZ))) * CIA) * DW) / CHZ;
                        CIC = CIA;
                        CID = CHW;
                        CIE = CIB;
                        CIF = CHX;
                    }
                    let CIH = AA - CIG;
                    let CIJ = CII * CA;
                    let CIK = DE * CII;
                    let CIM = CIH * (CIJ * CIL);
                    let CIN = CIM * CGY;
                    let CIO = CIN * CHR;
                    let CIP = ((Lanes([(((CIK * CIL) * CIH) * CGY), 0.0, 0.0]) + (CAO * CIM)) * CHR) + (CHT * CIN);
                    let CIQ = CII * CB;
                    let CIR = DF * CII;
                    let CIS = CIQ * CIL;
                    let CIT = CIR * CIL;
                    let CIU = CIH * CIS;
                    let CIV = CIU * CHV;
                    let CIW = CIV * CIC;
                    let CIX = ((Lanes([((CIT * CIH) * CHV), 0.0, 0.0]) + (CBC * CIU)) * CIC) + (CIE * CIV);
                    let CIZ = CIJ * CIY;
                    let CJA = CIZ * CGY;
                    let CJB = CJA * CHR;
                    let CJC = ((Lanes([((CIK * CIY) * CGY), 0.0, 0.0]) + (CAO * CIZ)) * CHR) + (CHT * CJA);
                    let CJD = CIQ * CIY;
                    let CJE = CJD * CHV;
                    let CJF = CJE * CIC;
                    let CJG = ((Lanes([((CIR * CIY) * CHV), 0.0, 0.0]) + (CBC * CJD)) * CIC) + (CIE * CJE);
                    let CJH = if parameters[14] == AA { 1.0 } else { 0.0 };
                    let CJS;
                    let CJT;
                    if CJH != 0.0 {
                        CJS = S;
                        CJT = BTY;
                    } else {
                        let CJJ = (Lanes([0.0, UU[0], UU[1], 0.0]) + Lanes([UY[0], 0.0, 0.0, UY[1]])) / CJI;
                        let CJK = AA + ((UT + UX) / CJI);
                        let CJL = CJJ * CJK;
                        let CJM = CJL + CJL;
                        let CJO = ((CJK * CJK) + (CJN * (CHS + CID))).sqrt();
                        let CJP = (CJK + CJO) / EG;
                        let CJQ = (Lanes([0.0, CJJ[0], CJJ[1], CJJ[2], CJJ[3]]) + ((Lanes([0.0, CJM[0], CJM[1], CJM[2], CJM[3]]) + ((Lanes([CHU[0], 0.0, CHU[1], CHU[2], 0.0]) + Lanes([CIF[0], CIF[1], 0.0, 0.0, CIF[2]])) * CJN)) * (EA / (DZ * CJO)))) / EG;
                        let CJR = if CJP < 1e-1f64 { 1.0 } else { 0.0 };
                        let CJW;
                        let CJX;
                        if CJR != 0.0 {
                            CJW = BMY;
                            CJX = BTY;
                        } else {
                            let CJU = AA / CJP;
                            let CJV = ((CJQ * CJU) * DW) / CJP;
                            CJW = CJU;
                            CJX = CJV;
                        }
                        let CJY = CIG * CIS;
                        let CJZ = CAN - CBB;
                        let CKA = CJY * CJZ;
                        let CKB = CKA * CJW;
                        let CKC = ((Lanes([((CIT * CIG) * CJZ), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([CAO[0], 0.0, CAO[1], CAO[2], 0.0]) - Lanes([CBC[0], CBC[1], 0.0, 0.0, CBC[2]])) * CJY)) * CJW) + (CJX * CKA);
                        CJS = CKB;
                        CJT = CKC;
                    }
                    CHD = CIO;
                    CHE = CIW;
                    CHF = CJS;
                    CHG = CJB;
                    CHH = CJF;
                    CHI = CIP;
                    CHJ = CIX;
                    CHK = CJT;
                    CHL = CJC;
                    CHM = CJG;
                }
                let CHN = if (if CE == S { 1.0 } else { 0.0 }) != 0.0 && (if CF == S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CKI;
                let CKJ;
                let CKK;
                let CKL;
                if CHN != 0.0 {
                    CKI = S;
                    CKJ = S;
                    CKK = BTZ;
                    CKL = BUA;
                } else {
                    let CKD = CBY * staged[295];
                    let CKF = CKE - UT;
                    let CKG = UU * DW;
                    let CKH = if CKF < ZY { 1.0 } else { 0.0 };
                    let CKY;
                    let CKZ;
                    if CKH != 0.0 {
                        let CKQ = (((-UT) / CKD) * CKE) * CDA;
                        let CKR = ((CKG / CKD) * CKE) * CDA;
                        let CKS = if CKQ > JF { 1.0 } else { 0.0 };
                        let CLI;
                        let CLJ;
                        if CKS != 0.0 {
                            let CLF = JH * ((AA + CKQ) - JF);
                            let CLG = CKR * JH;
                            CLI = CLF;
                            CLJ = CLG;
                        } else {
                            let CLH = if CKQ < -1e2f64 { 1.0 } else { 0.0 };
                            let CLS;
                            let CLT;
                            if CLH != 0.0 {
                                CLS = JO;
                                CLT = CLP;
                            } else {
                                let CLQ = CKQ.exp();
                                let CLR = CKR * CLQ;
                                CLS = CLQ;
                                CLT = CLR;
                            }
                            CLI = CLS;
                            CLJ = CLT;
                        }
                        let CLK = CBI * CE;
                        let CLL = AA - CLI;
                        let CLM = CLK * CLL;
                        let CLN = (CLJ * DW) * CLK;
                        let CLO = Lanes([((DI * CBI) * CLL), 0.0, 0.0]) + Lanes([0.0, CLN[0], CLN[1]]);
                        CKY = CLM;
                        CKZ = CLO;
                    } else {
                        let CKT = AA / CKF;
                        let CKU = ((-UT) / CKD) * CKE;
                        let CKV = CKU * CKT;
                        let CKW = (((CKG / CKD) * CKE) * CKT) + ((((CKG * CKT) * DW) / CKF) * CKU);
                        let CKX = if CKV > JF { 1.0 } else { 0.0 };
                        let CLX;
                        let CLY;
                        if CKX != 0.0 {
                            let CLU = JH * ((AA + CKV) - JF);
                            let CLV = CKW * JH;
                            CLX = CLU;
                            CLY = CLV;
                        } else {
                            let CLW = if CKV < -1e2f64 { 1.0 } else { 0.0 };
                            let CMG;
                            let CMH;
                            if CLW != 0.0 {
                                CMG = JO;
                                CMH = CLP;
                            } else {
                                let CME = CKV.exp();
                                let CMF = CKW * CME;
                                CMG = CME;
                                CMH = CMF;
                            }
                            CLX = CMG;
                            CLY = CMH;
                        }
                        let CLZ = CBI * CE;
                        let CMA = AA - CLX;
                        let CMB = CLZ * CMA;
                        let CMC = (CLY * DW) * CLZ;
                        let CMD = Lanes([((DI * CBI) * CMA), 0.0, 0.0]) + Lanes([0.0, CMC[0], CMC[1]]);
                        CKY = CMB;
                        CKZ = CMD;
                    }
                    let CLA = CBY * staged[297];
                    let CLC = CLB - UX;
                    let CLD = UY * DW;
                    let CLE = if CLC < ZY { 1.0 } else { 0.0 };
                    let CMQ;
                    let CMR;
                    if CLE != 0.0 {
                        let CMI = (((-UX) / CLA) * CLB) * CDA;
                        let CMJ = ((CLD / CLA) * CLB) * CDA;
                        let CMK = if CMI > JF { 1.0 } else { 0.0 };
                        let CMV;
                        let CMW;
                        if CMK != 0.0 {
                            let CMS = JH * ((AA + CMI) - JF);
                            let CMT = CMJ * JH;
                            CMV = CMS;
                            CMW = CMT;
                        } else {
                            let CMU = if CMI < -1e2f64 { 1.0 } else { 0.0 };
                            let CNF;
                            let CNG;
                            if CMU != 0.0 {
                                CNF = JO;
                                CNG = CNC;
                            } else {
                                let CND = CMI.exp();
                                let CNE = CMJ * CND;
                                CNF = CND;
                                CNG = CNE;
                            }
                            CMV = CNF;
                            CMW = CNG;
                        }
                        let CMX = CBQ * CF;
                        let CMY = AA - CMV;
                        let CMZ = CMX * CMY;
                        let CNA = (CMW * DW) * CMX;
                        let CNB = Lanes([((DJ * CBQ) * CMY), 0.0, 0.0]) + Lanes([0.0, CNA[0], CNA[1]]);
                        CMQ = CMZ;
                        CMR = CNB;
                    } else {
                        let CML = AA / CLC;
                        let CMM = ((-UX) / CLA) * CLB;
                        let CMN = CMM * CML;
                        let CMO = (((CLD / CLA) * CLB) * CML) + ((((CLD * CML) * DW) / CLC) * CMM);
                        let CMP = if CMN > JF { 1.0 } else { 0.0 };
                        let CNK;
                        let CNL;
                        if CMP != 0.0 {
                            let CNH = JH * ((AA + CMN) - JF);
                            let CNI = CMO * JH;
                            CNK = CNH;
                            CNL = CNI;
                        } else {
                            let CNJ = if CMN < -1e2f64 { 1.0 } else { 0.0 };
                            let CNT;
                            let CNU;
                            if CNJ != 0.0 {
                                CNT = JO;
                                CNU = CNC;
                            } else {
                                let CNR = CMN.exp();
                                let CNS = CMO * CNR;
                                CNT = CNR;
                                CNU = CNS;
                            }
                            CNK = CNT;
                            CNL = CNU;
                        }
                        let CNM = CBQ * CF;
                        let CNN = AA - CNK;
                        let CNO = CNM * CNN;
                        let CNP = (CNL * DW) * CNM;
                        let CNQ = Lanes([((DJ * CBQ) * CNN), 0.0, 0.0]) + Lanes([0.0, CNP[0], CNP[1]]);
                        CMQ = CNO;
                        CMR = CNQ;
                    }
                    CKI = CKY;
                    CKJ = CMQ;
                    CKK = CKZ;
                    CKL = CMR;
                }
                let CKM = ((CBN + CCJ) + CHD) + CKI;
                let CKN = ((CBO + CCK) + CHI) + CKK;
                let CKO = ((CBV + CEV) + CHE) + CKJ;
                let CKP = ((CBW + CEW) + CHJ) + CKL;
                BUC = CHF;
                BUD = CKM;
                BUE = CKO;
                BUF = BUS;
                BUG = BUT;
                BUH = CHG;
                BUI = CHH;
                BUJ = CHK;
                BUK = CKN;
                BUL = CKP;
                BUM = BUU;
                BUN = BUV;
                BUO = CHL;
                BUP = CHM;
            } else {
                BUC = S;
                BUD = S;
                BUE = S;
                BUF = S;
                BUG = S;
                BUH = S;
                BUI = S;
                BUJ = BTY;
                BUK = BTZ;
                BUL = BUA;
                BUM = BUB;
                BUN = ATP;
                BUO = BTZ;
                BUP = BUA;
            }
            let COD;
            let COE;
            let COF;
            let COG;
            let COH;
            let COI;
            let COJ;
            let COK;
            if BUQ != 0.0 {
                let CNV = YQ - ZT;
                let CNW = BCO - ZW;
                let CNX = (AXD - BG) - AXF;
                let CNY = (AXE - CK) - AXG;
                let CNZ = Lanes([CNY, 0.0, 0.0, 0.0]) - YR;
                let COA = Lanes([0.0, 0.0, CNZ[0], CNZ[1], CNZ[2], CNZ[3]]) + ZW;
                let COB = ((CNX - YQ) + ZT) - ADI;
                let COC = if CNX <= S { 1.0 } else { 0.0 };
                let COU;
                let COV;
                if COC != 0.0 {
                    let COM = COA * COB;
                    let COO = ((COB * COB) - (CON * CNX)).sqrt();
                    let COP = ((COM + COM) - Lanes([0.0, 0.0, (CNY * CON), 0.0, 0.0, 0.0])) * (EA / (DZ * COO));
                    COU = COO;
                    COV = COP;
                } else {
                    let COQ = COA * COB;
                    let COS = ((COB * COB) + (COR * CNX)).sqrt();
                    let COT = ((COQ + COQ) + Lanes([0.0, 0.0, (CNY * COR), 0.0, 0.0, 0.0])) * (EA / (DZ * COS));
                    COU = COS;
                    COV = COT;
                }
                let COW = CNX - (YE * (COB + COU));
                let COX = Lanes([0.0, 0.0, CNY, 0.0, 0.0, 0.0]);
                let COY = COX - ((COA + COV) * YE);
                let COZ = CNX - COW;
                let CPA = COX - COY;
                let CPB = if COZ < S { 1.0 } else { 0.0 };
                let CPC;
                let CPD;
                if CPB != 0.0 {
                    CPC = S;
                    CPD = ATP;
                } else {
                    CPC = COZ;
                    CPD = CPA;
                }
                let CPI;
                let CPJ;
                if CPE != 0.0 {
                    CPI = S;
                    CPJ = ATP;
                } else {
                    let CPF = ((YQ - BEA) - COW) - AAV;
                    let CPG = ((BCO - BEB) - COY) - AAW;
                    let CPH = if CPF < S { 1.0 } else { 0.0 };
                    let CPQ;
                    let CPR;
                    if CPH != 0.0 {
                        let CPK = CPF / AKA;
                        let CPL = CPG / AKA;
                        CPQ = CPK;
                        CPR = CPL;
                    } else {
                        let CPM = AKA / EG;
                        let CPN = (AA + (((CJN * CPF) / AKA) / AKA)).sqrt();
                        let CPO = CPM * (-1e0f64 + CPN);
                        let CPP = ((((CPG * CJN) / AKA) / AKA) * (EA / (DZ * CPN))) * CPM;
                        CPQ = CPO;
                        CPR = CPP;
                    }
                    let CPS = CPR * CPQ;
                    let CPT = (YQ - ((CPQ * CPQ) + ZT)) - CNX;
                    let CPU = (BCO - ((CPS + CPS) + ZW)) - COX;
                    CPI = CPT;
                    CPJ = CPU;
                }
                COD = CPI;
                COE = CNV;
                COF = CPC;
                COG = CNX;
                COH = CPJ;
                COI = CNW;
                COJ = CPD;
                COK = CNY;
            } else {
                COD = S;
                COE = S;
                COF = S;
                COG = S;
                COH = ATP;
                COI = ATP;
                COJ = ATP;
                COK = T;
            }
            let CQF;
            let CQG;
            let CQH;
            let CQI;
            let CQJ;
            let CQK;
            let CQL;
            let CQM;
            let CQN;
            if COL != 0.0 {
                let CPW = ZK * CPV;
                let CPX = ZL * CPV;
                let CPY = YQ - AXD;
                let CPZ = YR - Lanes([AXE, 0.0, 0.0, 0.0]);
                let CQA = CPY / CPW;
                let CQB = (CPZ - Lanes([(CPX * CQA), 0.0, 0.0, 0.0])) / CPW;
                let CQC = if CQA > JF { 1.0 } else { 0.0 };
                let CQQ;
                let CQR;
                if CQC != 0.0 {
                    CQQ = CPY;
                    CQR = CPZ;
                } else {
                    let CQP = if CQA < -1e2f64 { 1.0 } else { 0.0 };
                    let CRJ;
                    let CRK;
                    if CQP != 0.0 {
                        let CRC = CPW * CRB;
                        let CRD = Lanes([(CPX * CRB), 0.0, 0.0, 0.0]);
                        CRJ = CRC;
                        CRK = CRD;
                    } else {
                        let CRE = CQA.exp();
                        let CRF = AA + CRE;
                        let CRG = CRF.ln();
                        let CRH = CPW * CRG;
                        let CRI = Lanes([(CPX * CRG), 0.0, 0.0, 0.0]) + (((CQB * CRE) * (EA / CRF)) * CPW);
                        CRJ = CRH;
                        CRK = CRI;
                    }
                    CQQ = CRJ;
                    CQR = CRK;
                }
                let CQS = YQ * CQQ;
                let CQT = (YR * CQQ) + (CQR * YQ);
                let CQW = CQV * COD;
                let CQY = CQX * ((staged[301] + (CQU * COD)) - (CQW * COD));
                let CQZ = ((COH * CQU) - (((COH * CQV) * COD) + (COH * CQW))) * CQX;
                let CRA = if CQY > JF { 1.0 } else { 0.0 };
                let CRM;
                let CRN;
                if CRA != 0.0 {
                    CRM = JH;
                    CRN = ATP;
                } else {
                    let CRL = if CQY < -1e2f64 { 1.0 } else { 0.0 };
                    let CSC;
                    let CSD;
                    if CRL != 0.0 {
                        CSC = JO;
                        CSD = ATP;
                    } else {
                        let CSA = CQY.exp();
                        let CSB = CQZ * CSA;
                        CSC = CSA;
                        CSD = CSB;
                    }
                    CRM = CSC;
                    CRN = CSD;
                }
                let CRP = CRO * CQS;
                let CRQ = CRP * CRM;
                let CRR = (CQT * CRO) * CRM;
                let CRS = Lanes([0.0, 0.0, CRR[0], CRR[1], CRR[2], CRR[3]]) + (CRN * CRP);
                let CRU = CRT * WR;
                let CRV = XO * CRT;
                let CRW = CRV * CRU;
                let CRX = CRW + CRW;
                let CRY = (CRU * CRU) + AIV;
                let CRZ = if CRU > JF { 1.0 } else { 0.0 };
                let CSF;
                let CSG;
                if CRZ != 0.0 {
                    CSF = JH;
                    CSG = AGV;
                } else {
                    let CSE = if CRU < -1e2f64 { 1.0 } else { 0.0 };
                    let CTG;
                    let CTH;
                    if CSE != 0.0 {
                        CTG = JO;
                        CTH = AGV;
                    } else {
                        let CTE = CRU.exp();
                        let CTF = CRV * CTE;
                        CTG = CTE;
                        CTH = CTF;
                    }
                    CSF = CTG;
                    CSG = CTH;
                }
                let CSH = CSF - AA;
                let CSI = ((CSH + AIQ) - CRU) / CRY;
                let CSJ = CRQ * CSI;
                let CSK = (((CSG - CRV) - (CRX * CSI)) / CRY) * CRQ;
                let CSL = (CRS * CSI) + Lanes([0.0, 0.0, 0.0, CSK[0], CSK[1], 0.0]);
                let CSM = ((CRU * CSF) - (CSH - AIQ)) / CRY;
                let CSN = CRQ * CSM;
                let CSO = (((((CRV * CSF) + (CSG * CRU)) - CSG) - (CRX * CSM)) / CRY) * CRQ;
                let CSP = (CRS * CSM) + Lanes([0.0, 0.0, 0.0, CSO[0], CSO[1], 0.0]);
                let CSQ = UF - BVG;
                let CSR = UG * CSQ;
                let CSS = ((CSQ * CSQ) + AIQ).sqrt();
                let CST = (CSR + CSR) * (EA / (DZ * CSS));
                let CSU = UF * CSS;
                let CSV = (UG * CSS) + (CST * UF);
                let CSZ = CSY * CSS;
                let CTB = CTA * ((CSX + (CSW * CSS)) - (CSZ * CSS));
                let CTC = ((CST * CSW) - (((CST * CSY) * CSS) + (CST * CSZ))) * CTA;
                let CTD = if CTB > JF { 1.0 } else { 0.0 };
                let CTJ;
                let CTK;
                if CTD != 0.0 {
                    CTJ = JH;
                    CTK = CQD;
                } else {
                    let CTI = if CTB < -1e2f64 { 1.0 } else { 0.0 };
                    let CUB;
                    let CUC;
                    if CTI != 0.0 {
                        CUB = JO;
                        CUC = CQD;
                    } else {
                        let CTZ = CTB.exp();
                        let CUA = CTC * CTZ;
                        CUB = CTZ;
                        CUC = CUA;
                    }
                    CTJ = CUB;
                    CTK = CUC;
                }
                let CTM = CTL * CSU;
                let CTN = CTM * CTJ;
                let CTO = ((CSV * CTL) * CTJ) + (CTK * CTM);
                let CTP = VG - BVG;
                let CTQ = VI * CTP;
                let CTR = ((CTP * CTP) + AIQ).sqrt();
                let CTS = (CTQ + CTQ) * (EA / (DZ * CTR));
                let CTT = VG * CTR;
                let CTU = (VI * CTR) + (CTS * VG);
                let CTV = CSY * CTR;
                let CTW = CTA * ((CSX + (CSW * CTR)) - (CTV * CTR));
                let CTX = ((CTS * CSW) - (((CTS * CSY) * CTR) + (CTS * CTV))) * CTA;
                let CTY = if CTW > JF { 1.0 } else { 0.0 };
                let CUE;
                let CUF;
                if CTY != 0.0 {
                    CUE = JH;
                    CUF = CQE;
                } else {
                    let CUD = if CTW < -1e2f64 { 1.0 } else { 0.0 };
                    let CUM;
                    let CUN;
                    if CUD != 0.0 {
                        CUM = JO;
                        CUN = CQE;
                    } else {
                        let CUK = CTW.exp();
                        let CUL = CTX * CUK;
                        CUM = CUK;
                        CUN = CUL;
                    }
                    CUE = CUM;
                    CUF = CUN;
                }
                let CUH = CUG * CTT;
                let CUI = CUH * CUE;
                let CUJ = ((CTU * CUG) * CUE) + (CUF * CUH);
                CQF = CSJ;
                CQG = CSN;
                CQH = CTN;
                CQI = CUI;
                CQJ = CTA;
                CQK = CSL;
                CQL = CSP;
                CQM = CTO;
                CQN = CUJ;
            } else {
                CQF = S;
                CQG = S;
                CQH = S;
                CQI = S;
                CQJ = BJQ;
                CQK = ATP;
                CQL = ATP;
                CQM = CQD;
                CQN = CQE;
            }
            let CVB;
            let CVC;
            let CVD;
            let CVE;
            if CQO != 0.0 {
                let CUP = COH * DW;
                let CUR = (CUO - COD) - CUQ;
                let CUS = CUP * CUR;
                let CUU = ((CUR * CUR) + CUT).sqrt();
                let CUV = CUO - (YE * (CUR + CUU));
                let CUW = ((CUP + ((CUS + CUS) * (EA / (DZ * CUU)))) * YE) * DW;
                let CUY = (CUV - parameters[369]) / CUX;
                let CUZ = CUW / CUX;
                let CVA = if CUY > JF { 1.0 } else { 0.0 };
                let CVL;
                let CVM;
                if CVA != 0.0 {
                    let CVI = JH * ((AA + CUY) - JF);
                    let CVJ = CUZ * JH;
                    CVL = CVI;
                    CVM = CVJ;
                } else {
                    let CVK = if CUY < -1e2f64 { 1.0 } else { 0.0 };
                    let CVT;
                    let CVU;
                    if CVK != 0.0 {
                        CVT = JO;
                        CVU = ATP;
                    } else {
                        let CVR = CUY.exp();
                        let CVS = CUZ * CVR;
                        CVT = CVR;
                        CVU = CVS;
                    }
                    CVL = CVT;
                    CVM = CVU;
                }
                let CVN = AA + CVL;
                let CVO = CUX * (CVN.ln());
                let CVP = (CVM * (EA / CVN)) * CUX;
                let CVY;
                let CVZ;
                if CVQ != 0.0 {
                    let CVW = AA - (CUV / CVV);
                    let CVX = (CUW / CVV) * DW;
                    CVY = CVW;
                    CVZ = CVX;
                } else {
                    CVY = AA;
                    CVZ = ATP;
                }
                let CWA = if CVY < AOA { 1.0 } else { 0.0 };
                let CWB;
                let CWC;
                if CWA != 0.0 {
                    CWB = AOA;
                    CWC = ATP;
                } else {
                    CWB = CVY;
                    CWC = CVZ;
                }
                let CWD = (BFR * AHB) / BTR;
                let CWE = ((AHB * BFQ) / BTR) + staged[314];
                let CWH = (CWE * CWF) * CWG;
                let CWI = (CWD * CWF) * CWG;
                let CWL = (CWK * (staged[317] - (CWJ * CUV))) / CWB;
                let CWM = ((((CUW * CWJ) * DW) * CWK) - (CWC * CWL)) / CWB;
                let CWN = if CWL > JF { 1.0 } else { 0.0 };
                let CWR;
                let CWS;
                if CWN != 0.0 {
                    let CWO = JH * ((AA + CWL) - JF);
                    let CWP = CWM * JH;
                    CWR = CWO;
                    CWS = CWP;
                } else {
                    let CWQ = if CWL < -1e2f64 { 1.0 } else { 0.0 };
                    let CXJ;
                    let CXK;
                    if CWQ != 0.0 {
                        CXJ = JO;
                        CXK = ATP;
                    } else {
                        let CXH = CWL.exp();
                        let CXI = CWM * CXH;
                        CXJ = CXH;
                        CXK = CXI;
                    }
                    CWR = CXJ;
                    CWS = CXK;
                }
                let CWT = CWH * COE;
                let CWU = CWT * CVO;
                let CWV = CWU * CWR;
                let CWW = (((((CWI * COE) + (COI * CWH)) * CVO) + (CVP * CWT)) * CWR) + (CWS * CWU);
                let CWX = COJ * DW;
                let CWY = (CUO - COF) - CUQ;
                let CWZ = CWX * CWY;
                let CXA = ((CWY * CWY) + CUT).sqrt();
                let CXB = CUO - (YE * (CWY + CXA));
                let CXC = ((CWX + ((CWZ + CWZ) * (EA / (DZ * CXA)))) * YE) * DW;
                let CXE = ((-COE) + COG) / CXD;
                let CXF = ((COI * DW) + Lanes([0.0, 0.0, COK, 0.0, 0.0, 0.0])) / CXD;
                let CXG = if CXE > JF { 1.0 } else { 0.0 };
                let CXO;
                let CXP;
                if CXG != 0.0 {
                    let CXL = JH * ((AA + CXE) - JF);
                    let CXM = CXF * JH;
                    CXO = CXL;
                    CXP = CXM;
                } else {
                    let CXN = if CXE < -1e2f64 { 1.0 } else { 0.0 };
                    let CXW;
                    let CXX;
                    if CXN != 0.0 {
                        CXW = JO;
                        CXX = ATP;
                    } else {
                        let CXU = CXE.exp();
                        let CXV = CXF * CXU;
                        CXW = CXU;
                        CXX = CXV;
                    }
                    CXO = CXW;
                    CXP = CXX;
                }
                let CXQ = AA + CXO;
                let CXR = CXD * (CXQ.ln());
                let CXS = (CXP * (EA / CXQ)) * CXD;
                let CYB;
                let CYC;
                if CXT != 0.0 {
                    let CXZ = AA - (CXB / CXY);
                    let CYA = (CXC / CXY) * DW;
                    CYB = CXZ;
                    CYC = CYA;
                } else {
                    CYB = AA;
                    CYC = ATP;
                }
                let CYD = if CYB < AOA { 1.0 } else { 0.0 };
                let CYE;
                let CYF;
                if CYD != 0.0 {
                    CYE = AOA;
                    CYF = ATP;
                } else {
                    CYE = CYB;
                    CYF = CYC;
                }
                let CYH = (CWE * CYG) * CWG;
                let CYI = (CWD * CYG) * CWG;
                let CYL = (CYK * (staged[320] - (CYJ * CXB))) / CYE;
                let CYM = ((((CXC * CYJ) * DW) * CYK) - (CYF * CYL)) / CYE;
                let CYN = if CYL > JF { 1.0 } else { 0.0 };
                let CYR;
                let CYS;
                if CYN != 0.0 {
                    let CYO = JH * ((AA + CYL) - JF);
                    let CYP = CYM * JH;
                    CYR = CYO;
                    CYS = CYP;
                } else {
                    let CYQ = if CYL < -1e2f64 { 1.0 } else { 0.0 };
                    let CZA;
                    let CZB;
                    if CYQ != 0.0 {
                        CZA = JO;
                        CZB = ATP;
                    } else {
                        let CYY = CYL.exp();
                        let CYZ = CYM * CYY;
                        CZA = CYY;
                        CZB = CYZ;
                    }
                    CYR = CZA;
                    CYS = CZB;
                }
                let CYT = CYH * COE;
                let CYU = CYT * CXR;
                let CYV = CYU * CYR;
                let CYW = (((((CYI * COE) + (COI * CYH)) * CXR) + (CXS * CYT)) * CYR) + (CYS * CYU);
                let CYX = if COE >= S { 1.0 } else { 0.0 };
                let CZC;
                let CZD;
                if CYX != 0.0 {
                    CZC = CWV;
                    CZD = CWW;
                } else {
                    CZC = CYV;
                    CZD = CYW;
                }
                let CZF = COG + CZE;
                CVB = CZC;
                CVC = CZF;
                CVD = CZD;
                CVE = COK;
            } else {
                CVB = S;
                CVC = S;
                CVD = ATP;
                CVE = T;
            }
            let CVF = SY * CVB;
            let CVG = CVD * SY;
            let CVH = if staged[322] != 0.0 && (if UP < CVC { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CZN;
            let CZO;
            let CZP;
            if CVH != 0.0 {
                let CZG = UP - CVC;
                let CZH = Lanes([UQ[0], 0.0, UQ[1]]) - Lanes([0.0, CVE, 0.0]);
                let CZI = CZH * CZG;
                let CZJ = ((CZG * CZG) + AIQ).sqrt();
                let CZK = YE * (((-CZG) + CZJ) - AOA);
                let CZL = ((CZH * DW) + ((CZI + CZI) * (EA / (DZ * CZJ)))) * YE;
                let CZU = if E != 0.0 {
                    CZS
                } else {
                    CZT
                };
                let CZX = if E != 0.0 {
                    CZV
                } else {
                    CZW
                };
                let CZY = UP * CZK;
                let CZZ = UQ * CZK;
                let DAA = Lanes([CZZ[0], 0.0, CZZ[1]]) + (CZL * UP);
                let DAE = (DAB * DAC) - DAD;
                let DAF = DAD * DAC;
                let DAG = (-CZX) * parameters[364];
                let DAH = DAF * CZK;
                let DAI = DAG * ((DAB + (DAE * CZK)) - (DAH * CZK));
                let DAJ = ((CZL * DAE) - (((CZL * DAF) * CZK) + (CZL * DAH))) * DAG;
                let DAK = if DAI > JF { 1.0 } else { 0.0 };
                let DAM;
                let DAN;
                if DAK != 0.0 {
                    DAM = JH;
                    DAN = CZM;
                } else {
                    let DAL = if DAI < -1e2f64 { 1.0 } else { 0.0 };
                    let DAU;
                    let DAV;
                    if DAL != 0.0 {
                        DAU = JO;
                        DAV = CZM;
                    } else {
                        let DAS = DAI.exp();
                        let DAT = DAJ * DAS;
                        DAU = DAS;
                        DAV = DAT;
                    }
                    DAM = DAU;
                    DAN = DAV;
                }
                let DAO = (CZU * parameters[29]) * CWG;
                let DAP = DAO * CZY;
                let DAQ = DAP * DAM;
                let DAR = ((DAA * DAO) * DAM) + (DAN * DAP);
                CZN = DAQ;
                CZO = CZX;
                CZP = DAR;
            } else {
                CZN = S;
                CZO = CQJ;
                CZP = CZM;
            }
            let CZQ = SY * CZN;
            let CZR = CZP * SY;
            let DAZ;
            let DBA;
            let DBB;
            let DBC;
            if BTX != 0.0 {
                let DBG;
                let DBH;
                if DAW != 0.0 {
                    let DCF;
                    let DCG;
                    if DBE != 0.0 {
                        DCF = S;
                        DCG = DAX;
                    } else {
                        let DBM = AA + (DBL * BEA);
                        let DBN = AA / DBM;
                        let DBP = DBN + DBO;
                        let DBR = AA + (DBQ * WR);
                        let DBS = AA / DBR;
                        let DBU = DBT * (BCN * DBP);
                        let DBV = ((((XO * DBQ) * DBS) * DW) / DBR) * DBU;
                        let DBW = WR - (((DBK * (AA + (DBJ * AB))) - staged[327]) + (DBU * DBS));
                        let DBX = BPJ - (Lanes([0.0, 0.0, ((Z * DBJ) * DBK), 0.0, 0.0, 0.0]) + (((((BCP * DBP) + (((((BEB * DBL) * DBN) * DW) / DBM) * BCN)) * DBT) * DBS) + Lanes([0.0, 0.0, 0.0, DBV[0], DBV[1], 0.0])));
                        let DCB = DCA * DBW;
                        let DCC = (DBZ + (DBY * DBW)) + (DCB * DBW);
                        let DCD = (DBX * DBY) + (((DBX * DCA) * DBW) + (DBX * DCB));
                        let DCE = if DCC < CHB { 1.0 } else { 0.0 };
                        let DCH;
                        let DCI;
                        if DCE != 0.0 {
                            DCH = CHB;
                            DCI = ATP;
                        } else {
                            DCH = DCC;
                            DCI = DCD;
                        }
                        let DCJ = if (if DCH < (DBW / JF) { 1.0 } else { 0.0 }) != 0.0 && (if DBW > S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let DCN;
                        let DCO;
                        if DCJ != 0.0 {
                            let DCL = DCK * JH;
                            DCN = DCL;
                            DCO = ATP;
                        } else {
                            let DCM = if (if DCH < ((-DBW) / JF) { 1.0 } else { 0.0 }) != 0.0 && (if DBW < S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let DCV;
                            let DCW;
                            if DCM != 0.0 {
                                let DCQ = DCK * JO;
                                DCV = DCQ;
                                DCW = ATP;
                            } else {
                                let DCR = DBW / DCH;
                                let DCS = DCR.exp();
                                let DCT = DCK * DCS;
                                let DCU = (((DBX - (DCI * DCR)) / DCH) * DCS) * DCK;
                                DCV = DCT;
                                DCW = DCU;
                            }
                            DCN = DCV;
                            DCO = DCW;
                        }
                        let DCP = if DCN > BMY { 1.0 } else { 0.0 };
                        let DCX;
                        let DCY;
                        if DCP != 0.0 {
                            DCX = BMY;
                            DCY = ATP;
                        } else {
                            DCX = DCN;
                            DCY = DCO;
                        }
                        let DCZ = staged[336] * XJ;
                        let DDA = BUJ * DCZ;
                        let DDB = BTS + (DCZ * BUC);
                        let DDC = DCX * DDB;
                        let DDD = DCY * DDB;
                        let DDE = Lanes([DDD[0], DDD[1], DDD[2], DDD[3], DDD[4], DDD[5], 0.0, 0.0]) + ((Lanes([BTT[0], BTT[1], BTT[2], BTT[3], BTT[4], BTT[5], 0.0, 0.0]) + Lanes([0.0, 0.0, DDA[0], DDA[1], DDA[2], 0.0, DDA[3], DDA[4]])) * DCX);
                        DCF = DDC;
                        DCG = DDE;
                    }
                    DBG = DCF;
                    DBH = DCG;
                } else {
                    let DDT;
                    let DDU;
                    if DBF != 0.0 {
                        DDT = S;
                        DDU = ATP;
                    } else {
                        let DDF = AA + (DBL * BEA);
                        let DDG = AA / DDF;
                        let DDH = DDG + DBO;
                        let DDI = AA + (DBQ * WR);
                        let DDJ = AA / DDI;
                        let DDL = DDK * (BCN * DDH);
                        let DDM = ((((XO * DBQ) * DDJ) * DW) / DDI) * DDL;
                        let DDN = WR - (((DBK * (AA + (DBJ * AB))) - staged[337]) + (DDL * DDJ));
                        let DDO = BPJ - (Lanes([0.0, 0.0, ((Z * DBJ) * DBK), 0.0, 0.0, 0.0]) + (((((BCP * DDH) + (((((BEB * DBL) * DDG) * DW) / DDF) * BCN)) * DDK) * DDJ) + Lanes([0.0, 0.0, 0.0, DDM[0], DDM[1], 0.0])));
                        let DDP = DCA * DDN;
                        let DDQ = (DBZ + (DBY * DDN)) + (DDP * DDN);
                        let DDR = (DDO * DBY) + (((DDO * DCA) * DDN) + (DDO * DDP));
                        let DDS = if DDQ < CHB { 1.0 } else { 0.0 };
                        let DEA;
                        let DEB;
                        if DDS != 0.0 {
                            DEA = CHB;
                            DEB = ATP;
                        } else {
                            DEA = DDQ;
                            DEB = DDR;
                        }
                        let DEC = if (if DEA < (DDN / JF) { 1.0 } else { 0.0 }) != 0.0 && (if DDN > S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let DEF;
                        let DEG;
                        if DEC != 0.0 {
                            let DED = DCK * JH;
                            DEF = DED;
                            DEG = ATP;
                        } else {
                            let DEE = if (if DEA < ((-DDN) / JF) { 1.0 } else { 0.0 }) != 0.0 && (if DDN < S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let DEN;
                            let DEO;
                            if DEE != 0.0 {
                                let DEI = DCK * JO;
                                DEN = DEI;
                                DEO = ATP;
                            } else {
                                let DEJ = DDN / DEA;
                                let DEK = DEJ.exp();
                                let DEL = DCK * DEK;
                                let DEM = (((DDO - (DEB * DEJ)) / DEA) * DEK) * DCK;
                                DEN = DEL;
                                DEO = DEM;
                            }
                            DEF = DEN;
                            DEG = DEO;
                        }
                        let DEH = if DEF > BMY { 1.0 } else { 0.0 };
                        let DEP;
                        let DEQ;
                        if DEH != 0.0 {
                            DEP = BMY;
                            DEQ = ATP;
                        } else {
                            DEP = DEF;
                            DEQ = DEG;
                        }
                        let DER = DEP * BTS;
                        let DES = (DEQ * BTS) + (BTT * DEP);
                        DDT = DER;
                        DDU = DES;
                    }
                    let DDX = DDW * (AA + (DDV * AB));
                    let DDY = (Z * DDV) * DDW;
                    let DDZ = if XJ > S { 1.0 } else { 0.0 };
                    let DEZ;
                    let DFA;
                    if DDZ != 0.0 {
                        let DET = DDX - UX;
                        let DEU = Lanes([DDY, 0.0, 0.0]) - Lanes([0.0, UY[0], UY[1]]);
                        let DEV = Lanes([DEU[0], DEU[1], 0.0, 0.0, DEU[2]]);
                        DEZ = DET;
                        DFA = DEV;
                    } else {
                        let DEW = DDX - UT;
                        let DEX = Lanes([DDY, 0.0, 0.0]) - Lanes([0.0, UU[0], UU[1]]);
                        let DEY = Lanes([DEX[0], 0.0, DEX[1], DEX[2], 0.0]);
                        DEZ = DEW;
                        DFA = DEY;
                    }
                    let DFB = if DEZ <= S { 1.0 } else { 0.0 };
                    let DFG;
                    let DFH;
                    if DFB != 0.0 {
                        DFG = S;
                        DFH = BTY;
                    } else {
                        let DFC = -staged[340];
                        let DFE = DFC * (DEZ.powf(DFD));
                        let DFF = (DFA * (DFD * (DEZ.powf((DFD - EA))))) * DFC;
                        DFG = DFE;
                        DFH = DFF;
                    }
                    let DFI = if DFG > JF { 1.0 } else { 0.0 };
                    let DFK;
                    let DFL;
                    if DFI != 0.0 {
                        DFK = JH;
                        DFL = BTY;
                    } else {
                        let DFJ = if DFG < -1e2f64 { 1.0 } else { 0.0 };
                        let DFU;
                        let DFV;
                        if DFJ != 0.0 {
                            DFU = JO;
                            DFV = BTY;
                        } else {
                            let DFS = DFG.exp();
                            let DFT = DFH * DFS;
                            DFU = DFS;
                            DFV = DFT;
                        }
                        DFK = DFU;
                        DFL = DFV;
                    }
                    let DFM = staged[342] * XJ;
                    let DFN = DFM * BUC;
                    let DFO = DFN * DEZ;
                    let DFP = ((((BUJ * DFM) * DEZ) + (DFA * DFN)) * DFK) + (DFL * DFO);
                    let DFQ = DDT + (DFO * DFK);
                    let DFR = Lanes([DDU[0], DDU[1], DDU[2], DDU[3], DDU[4], DDU[5], 0.0, 0.0]) + Lanes([0.0, 0.0, DFP[0], DFP[1], DFP[2], 0.0, DFP[3], DFP[4]]);
                    DBG = DFQ;
                    DBH = DFR;
                }
                let DFX;
                let DFY;
                if DBI != 0.0 {
                    DFX = S;
                    DFY = DAY;
                } else {
                    let DGC;
                    let DGD;
                    if DFW != 0.0 {
                        let DGF = UN * DGE;
                        let DGG = UO * DGE;
                        DGC = DGF;
                        DGD = DGG;
                    } else {
                        let DGA = UN / DFZ;
                        let DGB = UO / DFZ;
                        DGC = DGA;
                        DGD = DGB;
                    }
                    DFX = DGC;
                    DFY = DGD;
                }
                DAZ = DBG;
                DBA = DFX;
                DBB = DBH;
                DBC = DFY;
            } else {
                DAZ = S;
                DBA = S;
                DBB = DAX;
                DBC = DAY;
            }
            let DGN;
            let DGO;
            if DBD != 0.0 {
                let DGI = DGH * BJ;
                let DGK = DGJ * ((DGI * BTA) + BTU);
                let DGL = ((Lanes([0.0, 0.0, ((CN * DGH) * BTA), 0.0, 0.0, 0.0]) + (BTB * DGI)) + BTV) * DGJ;
                let DGS;
                let DGT;
                if DGM != 0.0 {
                    let DGQ = DGK * DGP;
                    let DGR = DGL * DGP;
                    DGS = DGQ;
                    DGT = DGR;
                } else {
                    DGS = DGK;
                    DGT = DGL;
                }
                let DGZ;
                let DHA;
                if DGU != 0.0 {
                    let DGW = DGV + DGS;
                    let DGX = (DGV * DGS) / DGW;
                    let DGY = ((DGT * DGV) - (DGT * DGX)) / DGW;
                    DGZ = DGX;
                    DHA = DGY;
                } else {
                    DGZ = DGS;
                    DHA = DGT;
                }
                DGN = DGZ;
                DGO = DHA;
            } else {
                DGN = S;
                DGO = ATP;
            }
            let DIJ;
            let DIK;
            let DIL;
            let DIM;
            if C != 0.0 {
                let DHB = UF - BVG;
                let DHC = UG * DHB;
                let DHD = ((DHB * DHB) + AIQ).sqrt();
                let DHE = AA + (BFS * (YE * (DHB + DHD)));
                let DHG = UC * DHF;
                let DHH = AA / DHE;
                let DHI = (((((UG + ((DHC + DHC) * (EA / (DZ * DHD)))) * YE) * BFS) * DHH) * DW) / DHE;
                let DHJ = DHH + (DHF * UB);
                let DHK = Lanes([0.0, DHI[0], DHI[1]]) + Lanes([DHG[0], DHG[1], 0.0]);
                let DHL = DHK * DHJ;
                let DHM = ((DHJ * DHJ) + AOA).sqrt();
                let DHN = DHJ + DHM;
                let DHO = CG * YE;
                let DHP = (DHK + ((DHL + DHL) * (EA / (DZ * DHM)))) * DHO;
                let DHQ = Lanes([0.0, DL, 0.0, 0.0]) + (Lanes([DHP[0], 0.0, DHP[1], DHP[2]]) + Lanes([0.0, ((DK * YE) * DHN), 0.0, 0.0]));
                let DHR = (CH + (DHN * DHO)) + BGP;
                let DHS = VG - BVG;
                let DHT = VI * DHS;
                let DHU = ((DHS * DHS) + AIQ).sqrt();
                let DHV = AA + (BFS * (YE * (DHS + DHU)));
                let DHW = VF * DHF;
                let DHX = AA / DHV;
                let DHY = (((((VI + ((DHT + DHT) * (EA / (DZ * DHU)))) * YE) * BFS) * DHX) * DW) / DHV;
                let DHZ = DHX + (DHF * VD);
                let DIA = Lanes([0.0, DHY[0], DHY[1], DHY[2]]) + Lanes([DHW[0], DHW[1], DHW[2], 0.0]);
                let DIB = DIA * DHZ;
                let DIC = ((DHZ * DHZ) + AOA).sqrt();
                let DID = DHZ + DIC;
                let DIE = TK * YE;
                let DIF = (DIA + ((DIB + DIB) * (EA / (DZ * DIC)))) * DIE;
                let DIG = Lanes([0.0, TS, 0.0, 0.0, 0.0]) + (Lanes([DIF[0], 0.0, DIF[1], DIF[2], DIF[3]]) + Lanes([0.0, ((TR * YE) * DID), 0.0, 0.0, 0.0]));
                let DIH = (TL + (DID * DIE)) + BGO;
                DIJ = DIH;
                DIK = DHR;
                DIL = DIG;
                DIM = DHQ;
            } else {
                DIJ = BGO;
                DIK = BGP;
                DIL = BUB;
                DIM = DII;
            }
            let DIN;
            let DIO;
            let DIP;
            let DIQ;
            if BFZ != 0.0 {
                DIN = S;
                DIO = S;
                DIP = BUB;
                DIQ = DII;
            } else {
                DIN = DIJ;
                DIO = DIK;
                DIP = DIL;
                DIQ = DIM;
            }
            let DJQ;
            let DJR;
            let DJS;
            let DJT;
            let DJU;
            let DJV;
            let DJW;
            let DJX;
            let DJY;
            let DJZ;
            let DKA;
            let DKB;
            let DKC;
            let DKD;
            let DKE;
            let DKF;
            let DKG;
            let DKH;
            let DKI;
            let DKJ;
            let DKK;
            let DKL;
            let DKM;
            let DKN;
            if DIR != 0.0 {
                let DIS = BTS * DGP;
                let DIT = BTT * DGP;
                let DIU = BUC * DGP;
                let DIV = BUJ * DGP;
                let DIW = BUD * DGP;
                let DIX = BUK * DGP;
                let DIY = BUE * DGP;
                let DIZ = BUL * DGP;
                let DJA = CQF * DGP;
                let DJB = CQK * DGP;
                let DJC = CQG * DGP;
                let DJD = CQL * DGP;
                let DJE = CQH * DGP;
                let DJF = CQM * DGP;
                let DJG = CQI * DGP;
                let DJH = CQN * DGP;
                let DJI = DAZ * DGP;
                let DJJ = DBB * DGP;
                let DJK = CVF * DGP;
                let DJL = CVG * DGP;
                let DJM = BUF * DGP;
                let DJN = BUM * DGP;
                let DJO = BUG * DGP;
                let DJP = BUN * DGP;
                DJQ = DIS;
                DJR = DIU;
                DJS = DIY;
                DJT = DJI;
                DJU = DJM;
                DJV = DIW;
                DJW = DJO;
                DJX = DJC;
                DJY = DJA;
                DJZ = DJG;
                DKA = DJE;
                DKB = DJK;
                DKC = DIT;
                DKD = DIV;
                DKE = DIZ;
                DKF = DJJ;
                DKG = DJN;
                DKH = DIX;
                DKI = DJP;
                DKJ = DJD;
                DKK = DJB;
                DKL = DJH;
                DKM = DJF;
                DKN = DJL;
            } else {
                DJQ = BTS;
                DJR = BUC;
                DJS = BUE;
                DJT = DAZ;
                DJU = BUF;
                DJV = BUD;
                DJW = BUG;
                DJX = CQG;
                DJY = CQF;
                DJZ = CQI;
                DKA = CQH;
                DKB = CVF;
                DKC = BTT;
                DKD = BUJ;
                DKE = BUL;
                DKF = DBB;
                DKG = BUM;
                DKH = BUK;
                DKI = BUN;
                DKJ = CQL;
                DKK = CQK;
                DKL = CQN;
                DKM = CQM;
                DKN = CVG;
            }
            let DKO = if XJ > S { 1.0 } else { 0.0 };
            let DKP = YQ - BCC;
            let DKQ = BCO - BCD;
            let DKR = AZT * ZK;
            let DKS = (BCS * DKP) / DKR;
            let DKT = ((DKQ * BCS) - (((AZU * ZK) + Lanes([0.0, 0.0, (ZL * AZT), 0.0, 0.0, 0.0])) * DKS)) / DKR;
            let DKV = AZT * DKU;
            let DKW = DKV * ZK;
            let DKX = ((AZU * DKU) * ZK) + Lanes([0.0, 0.0, (ZL * DKV), 0.0, 0.0, 0.0]);
            let DKZ = AZT * DKY;
            let DLA = DKZ * ZK;
            let DLB = ((AZU * DKY) * ZK) + Lanes([0.0, 0.0, (ZL * DKZ), 0.0, 0.0, 0.0]);
            let DLF;
            let DLG;
            let DLH;
            let DLI;
            if DLC != 0.0 {
                let DLD = if (if DKS > -1e2f64 { 1.0 } else { 0.0 }) != 0.0 && (if DKS < JF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DLU;
                let DLV;
                let DLW;
                let DLX;
                if DLD != 0.0 {
                    let DLK = DKS.exp();
                    let DLL = DLK * DLK;
                    let DLM = (DKT * DLK) * DLK;
                    let DLO = DLN / DKW;
                    let DLP = (-DLO).exp();
                    let DLQ = DLL * DLP;
                    let DLR = ((DLM + DLM) * DLP) + ((((((DKX * DLO) * DW) / DKW) * DW) * DLP) * DLL);
                    let DLS = AA + DLQ;
                    let DLT = if DLS > FJ { 1.0 } else { 0.0 };
                    let DMB;
                    let DMC;
                    if DLT != 0.0 {
                        let DLY = DLS.ln();
                        let DLZ = DLR * (EA / DLS);
                        DMB = DLY;
                        DMC = DLZ;
                    } else {
                        DMB = DMA;
                        DMC = ATP;
                    }
                    let DMD = DKW * DMB;
                    let DME = (DKX * DMB) + (DMC * DKW);
                    let DMO;
                    let DMP;
                    if DMF != 0.0 {
                        let DMG = (-CZE) / DLA;
                        let DMH = ZK * ZK;
                        let DMI = ZL * ZK;
                        let DMJ = DMG / DMH;
                        let DMK = DMJ.exp();
                        let DML = (DLR * DMK) + (((((((DLB * DMG) * DW) / DLA) - Lanes([0.0, 0.0, ((DMI + DMI) * DMJ), 0.0, 0.0, 0.0])) / DMH) * DMK) * DLQ);
                        let DMM = AA + (DLQ * DMK);
                        let DMN = if DMM > FJ { 1.0 } else { 0.0 };
                        let DMT;
                        let DMU;
                        if DMN != 0.0 {
                            let DMQ = DMM.ln();
                            let DMR = DML * (EA / DMM);
                            DMT = DMQ;
                            DMU = DMR;
                        } else {
                            DMT = DMS;
                            DMU = ATP;
                        }
                        let DMV = DLA * DMT;
                        let DMW = (DLB * DMT) + (DMU * DLA);
                        DMO = DMV;
                        DMP = DMW;
                    } else {
                        DMO = S;
                        DMP = ATP;
                    }
                    DLU = DMD;
                    DLV = DMO;
                    DLW = DME;
                    DLX = DMP;
                } else {
                    DLU = BEA;
                    DLV = S;
                    DLW = BEB;
                    DLX = ATP;
                }
                DLF = DLU;
                DLG = DLV;
                DLH = DLW;
                DLI = DLX;
            } else {
                let DNJ;
                let DNK;
                let DNL;
                let DNM;
                if DLE != 0.0 {
                    let DMX = if (if DKS > -1e2f64 { 1.0 } else { 0.0 }) != 0.0 && (if DKS < JF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DNV;
                    let DNW;
                    let DNX;
                    let DNY;
                    if DMX != 0.0 {
                        let DNN = BCS * DKU;
                        let DNO = (DKS / DNN).exp();
                        let DNP = DLN / DKW;
                        let DNQ = (-DNP).exp();
                        let DNR = DNO * DNQ;
                        let DNS = (((DKT / DNN) * DNO) * DNQ) + ((((((DKX * DNP) * DW) / DKW) * DW) * DNQ) * DNO);
                        let DNT = AA + DNR;
                        let DNU = if DNT > FJ { 1.0 } else { 0.0 };
                        let DOC;
                        let DOD;
                        if DNU != 0.0 {
                            let DNZ = DNT.ln();
                            let DOA = DNS * (EA / DNT);
                            DOC = DNZ;
                            DOD = DOA;
                        } else {
                            DOC = DOB;
                            DOD = ATP;
                        }
                        let DOE = DKW * DOC;
                        let DOF = (DKX * DOC) + (DOD * DKW);
                        let DOO;
                        let DOP;
                        if DMF != 0.0 {
                            let DOG = (-CZE) / DLA;
                            let DOH = ZK * ZK;
                            let DOI = ZL * ZK;
                            let DOJ = DOG / DOH;
                            let DOK = DOJ.exp();
                            let DOL = (DNS * DOK) + (((((((DLB * DOG) * DW) / DLA) - Lanes([0.0, 0.0, ((DOI + DOI) * DOJ), 0.0, 0.0, 0.0])) / DOH) * DOK) * DNR);
                            let DOM = AA + (DNR * DOK);
                            let DON = if DOM > FJ { 1.0 } else { 0.0 };
                            let DOT;
                            let DOU;
                            if DON != 0.0 {
                                let DOQ = DOM.ln();
                                let DOR = DOL * (EA / DOM);
                                DOT = DOQ;
                                DOU = DOR;
                            } else {
                                DOT = DOS;
                                DOU = ATP;
                            }
                            let DOV = DLA * DOT;
                            let DOW = (DLB * DOT) + (DOU * DLA);
                            DOO = DOV;
                            DOP = DOW;
                        } else {
                            DOO = S;
                            DOP = ATP;
                        }
                        DNV = DOE;
                        DNW = DOO;
                        DNX = DOF;
                        DNY = DOP;
                    } else {
                        DNV = BEA;
                        DNW = S;
                        DNX = BEB;
                        DNY = ATP;
                    }
                    DNJ = DNV;
                    DNK = DNW;
                    DNL = DNX;
                    DNM = DNY;
                } else {
                    let DMY = DKP - DLN;
                    let DNA = DKQ * DMZ;
                    let DNB = (DMZ * DMY) / DKW;
                    let DNC = (DNA - (DKX * DNB)) / DKW;
                    let DNF = (DKQ * DND) * DW;
                    let DNG = (DNE - (DND * DMY)) / DKW;
                    let DNH = (DNF - (DKX * DNG)) / DKW;
                    let DNI = if DNB > JF { 1.0 } else { 0.0 };
                    let DOY;
                    let DOZ;
                    if DNI != 0.0 {
                        DOY = DMY;
                        DOZ = DKQ;
                    } else {
                        let DOX = if DNG > JF { 1.0 } else { 0.0 };
                        let DPJ;
                        let DPK;
                        if DOX != 0.0 {
                            let DPA = (DMY - DNE) / DKW;
                            let DPB = DPA.exp();
                            let DPC = (ZK * TG) / RS;
                            let DPD = DPC * DPB;
                            let DPE = Lanes([0.0, 0.0, ((((ZL * TG) + (TN * ZK)) / RS) * DPB), 0.0, 0.0, 0.0]) + ((((DKQ - (DKX * DPA)) / DKW) * DPB) * DPC);
                            DPJ = DPD;
                            DPK = DPE;
                        } else {
                            let DPF = DNB.exp();
                            let DPG = DNC * DPF;
                            let DPH = AA + DPF;
                            let DPI = if DPH > FJ { 1.0 } else { 0.0 };
                            let DPO;
                            let DPP;
                            if DPI != 0.0 {
                                let DPL = DPH.ln();
                                let DPM = DPG * (EA / DPH);
                                DPO = DPL;
                                DPP = DPM;
                            } else {
                                DPO = DPN;
                                DPP = ATP;
                            }
                            let DPQ = ZK * TG;
                            let DPS = DPR / DPQ;
                            let DPT = DNG.exp();
                            let DPU = (DPS * DPT) * DND;
                            let DPV = DMZ - ((DKW * DPU) / DND);
                            let DPW = (DKW * DPO) / DPV;
                            let DPX = (((DKX * DPO) + (DPP * DKW)) - (((((DKX * DPU) + (((Lanes([0.0, 0.0, ((((((ZL * TG) + (TN * ZK)) * DPS) * DW) / DPQ) * DPT), 0.0, 0.0, 0.0]) + ((DNH * DPT) * DPS)) * DND) * DKW)) / DND) * DW) * DPW)) / DPV;
                            DPJ = DPW;
                            DPK = DPX;
                        }
                        DOY = DPJ;
                        DOZ = DPK;
                    }
                    let DQE;
                    let DQF;
                    if DMF != 0.0 {
                        let DPY = DMY - CZE;
                        let DPZ = (DMZ * DPY) / DLA;
                        let DQA = (DNA - (DLB * DPZ)) / DLA;
                        let DQB = (DNE - (DND * DPY)) / DLA;
                        let DQC = (DNF - (DLB * DQB)) / DLA;
                        let DQD = if DPZ > JF { 1.0 } else { 0.0 };
                        let DQH;
                        let DQI;
                        if DQD != 0.0 {
                            DQH = DPY;
                            DQI = DKQ;
                        } else {
                            let DQG = if DQB > JF { 1.0 } else { 0.0 };
                            let DQS;
                            let DQT;
                            if DQG != 0.0 {
                                let DQJ = ((DMY - DNE) - CZE) / DLA;
                                let DQK = DQJ.exp();
                                let DQL = (ZK * TG) / RS;
                                let DQM = DQL * DQK;
                                let DQN = Lanes([0.0, 0.0, ((((ZL * TG) + (TN * ZK)) / RS) * DQK), 0.0, 0.0, 0.0]) + ((((DKQ - (DLB * DQJ)) / DLA) * DQK) * DQL);
                                DQS = DQM;
                                DQT = DQN;
                            } else {
                                let DQO = DPZ.exp();
                                let DQP = DQA * DQO;
                                let DQQ = AA + DQO;
                                let DQR = if DQQ > FJ { 1.0 } else { 0.0 };
                                let DQX;
                                let DQY;
                                if DQR != 0.0 {
                                    let DQU = DQQ.ln();
                                    let DQV = DQP * (EA / DQQ);
                                    DQX = DQU;
                                    DQY = DQV;
                                } else {
                                    DQX = DQW;
                                    DQY = ATP;
                                }
                                let DQZ = ZK * TG;
                                let DRA = DPR / DQZ;
                                let DRB = DQB.exp();
                                let DRC = (DRA * DRB) * DND;
                                let DRD = DMZ - ((DLA * DRC) / DND);
                                let DRE = (DLA * DQX) / DRD;
                                let DRF = (((DLB * DQX) + (DQY * DLA)) - (((((DLB * DRC) + (((Lanes([0.0, 0.0, ((((((ZL * TG) + (TN * ZK)) * DRA) * DW) / DQZ) * DRB), 0.0, 0.0, 0.0]) + ((DQC * DRB) * DRA)) * DND) * DLA)) / DND) * DW) * DRE)) / DRD;
                                DQS = DRE;
                                DQT = DRF;
                            }
                            DQH = DQS;
                            DQI = DQT;
                        }
                        DQE = DQH;
                        DQF = DQI;
                    } else {
                        DQE = S;
                        DQF = ATP;
                    }
                    DNJ = DOY;
                    DNK = DQE;
                    DNL = DOZ;
                    DNM = DQF;
                }
                DLF = DNJ;
                DLG = DNK;
                DLH = DNL;
                DLI = DNM;
            }
            let DRH;
            let DRI;
            let DRJ;
            let DRK;
            let DRL;
            let DRM;
            let DRN;
            let DRO;
            if DLJ != 0.0 {
                let DRW;
                let DRX;
                let DRY;
                let DRZ;
                if DRG != 0.0 {
                    DRW = S;
                    DRX = S;
                    DRY = ATP;
                    DRZ = ATP;
                } else {
                    let DRQ = (BCD - ABM) - (Lanes([0.0, 0.0, (SP * AXN), 0.0, 0.0, 0.0]) + (AXO * SO));
                    let DRR = ((BCC - BG) - (SO * AXN)) + DLN;
                    let DRS = (DRQ - BCO) + ABL;
                    let DRU = ((DRR - YQ) + ABK) - DRT;
                    let DRV = if DRR <= S { 1.0 } else { 0.0 };
                    let DSU;
                    let DSV;
                    if DRV != 0.0 {
                        let DSM = DRS * DRU;
                        let DSO = ((DRU * DRU) - (DSN * DRR)).sqrt();
                        let DSP = ((DSM + DSM) - (DRQ * DSN)) * (EA / (DZ * DSO));
                        DSU = DSO;
                        DSV = DSP;
                    } else {
                        let DSQ = DRS * DRU;
                        let DSS = ((DRU * DRU) + (DSR * DRR)).sqrt();
                        let DST = ((DSQ + DSQ) + (DRQ * DSR)) * (EA / (DZ * DSS));
                        DSU = DSS;
                        DSV = DST;
                    }
                    let DSW = DRR - (YE * (DRU + DSU));
                    let DSX = DRQ - ((DRS + DSV) * YE);
                    let DSZ = DSY * (DSW - DRR);
                    let DTA = (DSX - DRQ) * DSY;
                    let DTG;
                    let DTH;
                    let DTI;
                    let DTJ;
                    if DTB != 0.0 {
                        let DTC = DRR + CZE;
                        let DTD = (DRQ - Lanes([0.0, 0.0, 0.0, XL[0], XL[1], XL[2]])) + ABL;
                        let DTE = ((DTC - WO) + ABK) - DRT;
                        let DTF = if DTC <= S { 1.0 } else { 0.0 };
                        let DTV;
                        let DTW;
                        if DTF != 0.0 {
                            let DTN = DTD * DTE;
                            let DTP = ((DTE * DTE) - (DTO * DTC)).sqrt();
                            let DTQ = ((DTN + DTN) - (DRQ * DTO)) * (EA / (DZ * DTP));
                            DTV = DTP;
                            DTW = DTQ;
                        } else {
                            let DTR = DTD * DTE;
                            let DTT = ((DTE * DTE) + (DTS * DTC)).sqrt();
                            let DTU = ((DTR + DTR) + (DRQ * DTS)) * (EA / (DZ * DTT));
                            DTV = DTT;
                            DTW = DTU;
                        }
                        let DTX = DTC - (YE * (DTE + DTV));
                        let DTY = DRQ - ((DTD + DTW) * YE);
                        let DUA = DSZ + (DTZ * (DTX - DTC));
                        let DUB = DTA + ((DTY - DRQ) * DTZ);
                        DTG = DTX;
                        DTH = DUA;
                        DTI = DTY;
                        DTJ = DUB;
                    } else {
                        DTG = S;
                        DTH = DSZ;
                        DTI = ATP;
                        DTJ = DTA;
                    }
                    let DTK = ((YQ - DSW) - ABK) - DLF;
                    let DTL = ((BCO - DSX) - ABL) - DLH;
                    let DUD;
                    let DUE;
                    if DTM != 0.0 {
                        DUD = S;
                        DUE = ATP;
                    } else {
                        let DUC = if DTK < S { 1.0 } else { 0.0 };
                        let DUO;
                        let DUP;
                        if DUC != 0.0 {
                            let DUK = DTL / AKA;
                            let DUL = DUF + (DTK / AKA);
                            DUO = DUL;
                            DUP = DUK;
                        } else {
                            let DUM = ((DUF * DUF) + DTK).sqrt();
                            let DUN = DTL * (EA / (DZ * DUM));
                            DUO = DUM;
                            DUP = DUN;
                        }
                        DUD = DUO;
                        DUE = DUP;
                    }
                    let DUH = DUG * (DUD - DUF);
                    let DUI = DUE * DUG;
                    let DUT;
                    let DUU;
                    if DUJ != 0.0 {
                        let DUQ = ((WO - DTG) - ABK) - DLG;
                        let DUR = ((Lanes([0.0, 0.0, 0.0, XL[0], XL[1], XL[2]]) - DTI) - ABL) - DLI;
                        let DUS = if DUQ < S { 1.0 } else { 0.0 };
                        let DUZ;
                        let DVA;
                        if DUS != 0.0 {
                            let DUV = DUR / AKA;
                            let DUW = DUF + (DUQ / AKA);
                            DUZ = DUW;
                            DVA = DUV;
                        } else {
                            let DUX = ((DUF * DUF) + DUQ).sqrt();
                            let DUY = DUR * (EA / (DZ * DUX));
                            DUZ = DUX;
                            DVA = DUY;
                        }
                        let DVC = DUH + (DVB * (DUZ - DUF));
                        let DVD = DUI + (DVA * DVB);
                        DUT = DVC;
                        DUU = DVD;
                    } else {
                        DUT = DUH;
                        DUU = DUI;
                    }
                    DRW = DTH;
                    DRX = DUT;
                    DRY = DTJ;
                    DRZ = DUU;
                }
                let DSB = BJD * DSA;
                let DSC = BJE * DSA;
                let DSD = DLF / DSB;
                let DSE = (DLH - (DSC * DSD)) / DSB;
                let DSF = DSE - BPJ;
                let DSG = (DSD - WR) - ADI;
                let DSH = DSF * DSG;
                let DSJ = ((DSG * DSG) + (DSI * DSD)).sqrt();
                let DSK = DSD - (YE * (DSG + DSJ));
                let DSL = DSE - ((DSF + (((DSH + DSH) + (DSE * DSI)) * (EA / (DZ * DSJ)))) * YE);
                let DVN;
                let DVO;
                if DMF != 0.0 {
                    let DVE = DLG / DSB;
                    let DVF = (DLI - (DSC * DVE)) / DSB;
                    let DVG = DVF - BPJ;
                    let DVH = (DVE - WR) - ADI;
                    let DVI = DVG * DVH;
                    let DVK = ((DVH * DVH) + (DVJ * DVE)).sqrt();
                    let DVL = DVE - (YE * (DVH + DVK));
                    let DVM = DVF - ((DVG + (((DVI + DVI) + (DVF * DVJ)) * (EA / (DZ * DVK)))) * YE);
                    DVN = DVL;
                    DVO = DVM;
                } else {
                    DVN = S;
                    DVO = ATP;
                }
                let DWC;
                let DWD;
                if DRG != 0.0 {
                    DWC = S;
                    DWD = ATP;
                } else {
                    let DVP = DSB * DSK;
                    let DVQ = (DSC * DSK) + (DSL * DSB);
                    let DVT = DVS * ((DLF - (YE * DVP)) + DVR);
                    let DVU = DSK / DVT;
                    let DVV = AA - DSB;
                    let DVW = DSC * DW;
                    let DVX = DSY * DVV;
                    let DVY = (YE * DSK) - (DVP * DVU);
                    let DVZ = DVX * DVY;
                    let DWA = ((DVW * DSY) * DVY) + (((DSL * YE) - ((DVQ * DVU) + (((DSL - (((DLH - (DVQ * YE)) * DVS) * DVU)) / DVT) * DVP))) * DVX);
                    let DWX;
                    let DWY;
                    if DWB != 0.0 {
                        let DWP = DSB * DVN;
                        let DWQ = (DSC * DVN) + (DVO * DSB);
                        let DWR = DVS * ((DLG - (YE * DWP)) + DVR);
                        let DWS = DVN / DWR;
                        let DWT = DTZ * DVV;
                        let DWU = (YE * DVN) - (DWP * DWS);
                        let DWV = DVZ + (DWT * DWU);
                        let DWW = DWA + (((DVW * DTZ) * DWU) + (((DVO * YE) - ((DWQ * DWS) + (((DVO - (((DLI - (DWQ * YE)) * DVS) * DWS)) / DWR) * DWP))) * DWT));
                        DWX = DWV;
                        DWY = DWW;
                    } else {
                        DWX = DVZ;
                        DWY = DWA;
                    }
                    DWC = DWX;
                    DWD = DWY;
                }
                let DWE = DSB * DSK;
                let DWF = (DSC * DSK) + (DSL * DSB);
                let DWG = DLF - (YE * DWE);
                let DWH = DLH - (DWF * YE);
                let DWI = DVS * (DWG + DVR);
                let DWJ = DWH * DVS;
                let DWK = DWE / DWI;
                let DWM = DWL * (DWG + (DWE * DWK));
                let DWN = (DWH + ((DWF * DWK) + (((DWF - (DWJ * DWK)) / DWI) * DWE))) * DWL;
                let DXJ;
                let DXK;
                let DXL;
                let DXM;
                let DXN;
                let DXO;
                if DWO != 0.0 {
                    let DWZ = DSB * DVN;
                    let DXA = (DSC * DVN) + (DVO * DSB);
                    let DXB = DLG - (YE * DWZ);
                    let DXC = DLI - (DXA * YE);
                    let DXD = DVS * (DXB + DVR);
                    let DXE = DXC * DVS;
                    let DXF = DWZ / DXD;
                    let DXH = DWM + (DXG * (DXB + (DWZ * DXF)));
                    let DXI = DWN + ((DXC + ((DXA * DXF) + (((DXA - (DXE * DXF)) / DXD) * DWZ))) * DXG);
                    DXJ = DXD;
                    DXK = DWZ;
                    DXL = DXH;
                    DXM = DXE;
                    DXN = DXA;
                    DXO = DXI;
                } else {
                    DXJ = CZO;
                    DXK = S;
                    DXL = DWM;
                    DXM = ATP;
                    DXN = ATP;
                    DXO = DWN;
                }
                let DXZ;
                let DYA;
                if DXP != 0.0 {
                    let DXQ = DWI + DWI;
                    let DXS = DWF * DWE;
                    let DXT = (DWE * DWE) / DXQ;
                    let DXV = DXU * (((YE * DLF) + (DXR * DWE)) - DXT);
                    let DXW = (((DLH * YE) + (DWF * DXR)) - (((DXS + DXS) - ((DWJ + DWJ) * DXT)) / DXQ)) * DXU;
                    let DYG;
                    let DYH;
                    if DXX != 0.0 {
                        let DYB = DXJ + DXJ;
                        let DYC = DXN * DXK;
                        let DYD = (DXK * DXK) / DYB;
                        let DYE = DXV - (DXG * (((YE * DLG) + (DXR * DXK)) - DYD));
                        let DYF = DXW - ((((DLI * YE) + (DXN * DXR)) - (((DYC + DYC) - ((DXM + DXM) * DYD)) / DYB)) * DXG);
                        DYG = DYE;
                        DYH = DYF;
                    } else {
                        DYG = DXV;
                        DYH = DXW;
                    }
                    DXZ = DYG;
                    DYA = DYH;
                } else {
                    let DZA;
                    let DZB;
                    if DXY != 0.0 {
                        let DYI = DWI / DVS;
                        let DYJ = DYI * DYI;
                        let DYK = (DWJ / DVS) * DYI;
                        let DYL = staged[366] / DYJ;
                        let DYM = EG * DWE;
                        let DYN = DYM * DWE;
                        let DYO = ((DWF * EG) * DWE) + (DWF * DYM);
                        let DYP = DLF - ((CJN * DWE) / AEC);
                        let DYQ = (DYN / AEC) + (DLF * DYP);
                        let DYS = (DLF * DYQ) - ((DYN * DWE) / DYR);
                        let DYT = -DYL;
                        let DYU = DYT * DYS;
                        let DYV = ((((((DYK + DYK) * DYL) * DW) / DYJ) * DW) * DYS) + ((((DLH * DYQ) + (((DYO / AEC) + ((DLH * DYP) + ((DLH - ((DWF * CJN) / AEC)) * DLF))) * DLF)) - (((DYO * DWE) + (DWF * DYN)) / DYR)) * DYT);
                        let DZP;
                        let DZQ;
                        if DYW != 0.0 {
                            let DZC = DXJ / DVS;
                            let DZD = DZC * DZC;
                            let DZE = (DXM / DVS) * DZC;
                            let DZF = staged[367] / DZD;
                            let DZG = EG * DXK;
                            let DZH = DZG * DXK;
                            let DZI = ((DXN * EG) * DXK) + (DXN * DZG);
                            let DZJ = DLG - ((CJN * DXK) / AEC);
                            let DZK = (DZH / AEC) + (DLG * DZJ);
                            let DZL = (DLG * DZK) - ((DZH * DXK) / DYR);
                            let DZM = -DZF;
                            let DZN = DYU + (DZM * DZL);
                            let DZO = DYV + (((((((DZE + DZE) * DZF) * DW) / DZD) * DW) * DZL) + ((((DLI * DZK) + (((DZI / AEC) + ((DLI * DZJ) + ((DLI - ((DXN * CJN) / AEC)) * DLG))) * DLG)) - (((DZI * DXK) + (DXN * DZH)) / DYR)) * DZM));
                            DZP = DZN;
                            DZQ = DZO;
                        } else {
                            DZP = DYU;
                            DZQ = DYV;
                        }
                        DZA = DZP;
                        DZB = DZQ;
                    } else {
                        let DYY = DYX * (DXL + DWC);
                        let DYZ = (DXO + DWD) * DYX;
                        DZA = DYY;
                        DZB = DYZ;
                    }
                    DXZ = DZA;
                    DYA = DZB;
                }
                let DZU;
                let DZV;
                if DRG != 0.0 {
                    DZU = S;
                    DZV = ATP;
                } else {
                    let DZS = DZR * (XQ - ZT);
                    let DZT = (Lanes([XR[0], 0.0, XR[1], XR[2], XR[3], 0.0]) - ZW) * DZR;
                    DZU = DZS;
                    DZV = DZT;
                }
                let DZW = (DXL + DRW) + DRX;
                let DZX = (DXO + DRY) + DRZ;
                let DZY = -(((DZW + DXZ) + (((DWC - DRW) - DRX) - DZU)) + DZU);
                let DZZ = (((DZX + DYA) + (((DWD - DRY) - DRZ) - DZV)) + DZV) * DW;
                DRH = DZW;
                DRI = DZY;
                DRJ = DXZ;
                DRK = DZU;
                DRL = DZX;
                DRM = DZZ;
                DRN = DYA;
                DRO = DZV;
            } else {
                let EAA;
                let EAB;
                let EAC;
                let EAD;
                let EAE;
                let EAF;
                let EAG;
                let EAH;
                if O != 0.0 {
                    let EAJ;
                    let EAK;
                    let EAL;
                    let EAM;
                    let EAN;
                    let EAO;
                    let EAP;
                    let EAQ;
                    let EAR;
                    let EAS;
                    if EAI != 0.0 {
                        EAJ = S;
                        EAK = S;
                        EAL = S;
                        EAM = S;
                        EAN = S;
                        EAO = T;
                        EAP = T;
                        EAQ = ATP;
                        EAR = ATP;
                        EAS = ATP;
                    } else {
                        let EAX;
                        let EAY;
                        if P != 0.0 {
                            let EAU = (BCM - CK) - AXG;
                            let EAV = ((BCL - BG) - AXF) + DLN;
                            EAX = EAV;
                            EAY = EAU;
                        } else {
                            EAX = EAW;
                            EAY = T;
                        }
                        let EAZ = Lanes([EAY, 0.0, 0.0, 0.0]);
                        let EBA = EAZ - YR;
                        let EBB = Lanes([0.0, 0.0, EBA[0], EBA[1], EBA[2], EBA[3]]) + ABL;
                        let EBC = ((EAX - YQ) + ABK) - ADI;
                        let EBD = if EAX <= S { 1.0 } else { 0.0 };
                        let EBM;
                        let EBN;
                        if EBD != 0.0 {
                            let EBE = EBB * EBC;
                            let EBG = ((EBC * EBC) - (EBF * EAX)).sqrt();
                            let EBH = ((EBE + EBE) - Lanes([0.0, 0.0, (EAY * EBF), 0.0, 0.0, 0.0])) * (EA / (DZ * EBG));
                            EBM = EBG;
                            EBN = EBH;
                        } else {
                            let EBI = EBB * EBC;
                            let EBK = ((EBC * EBC) + (EBJ * EAX)).sqrt();
                            let EBL = ((EBI + EBI) + Lanes([0.0, 0.0, (EAY * EBJ), 0.0, 0.0, 0.0])) * (EA / (DZ * EBK));
                            EBM = EBK;
                            EBN = EBL;
                        }
                        let EBO = EAX - (YE * (EBC + EBM));
                        let EBP = Lanes([0.0, 0.0, EAY, 0.0, 0.0, 0.0]);
                        let EBQ = EBP - ((EBB + EBN) * YE);
                        let EBW;
                        let EBX;
                        let EBY;
                        let EBZ;
                        if DMF != 0.0 {
                            let EBR = EAX + CZE;
                            let EBS = EAZ - Lanes([0.0, XL[0], XL[1], XL[2]]);
                            let EBT = Lanes([0.0, 0.0, EBS[0], EBS[1], EBS[2], EBS[3]]) + ABL;
                            let EBU = ((EBR - WO) + ABK) - ADI;
                            let EBV = if EBR <= S { 1.0 } else { 0.0 };
                            let ECN;
                            let ECO;
                            if EBV != 0.0 {
                                let ECF = EBT * EBU;
                                let ECH = ((EBU * EBU) - (ECG * EBR)).sqrt();
                                let ECI = ((ECF + ECF) - Lanes([0.0, 0.0, (EAY * ECG), 0.0, 0.0, 0.0])) * (EA / (DZ * ECH));
                                ECN = ECH;
                                ECO = ECI;
                            } else {
                                let ECJ = EBT * EBU;
                                let ECL = ((EBU * EBU) + (ECK * EBR)).sqrt();
                                let ECM = ((ECJ + ECJ) + Lanes([0.0, 0.0, (EAY * ECK), 0.0, 0.0, 0.0])) * (EA / (DZ * ECL));
                                ECN = ECL;
                                ECO = ECM;
                            }
                            let ECP = EBR - (YE * (EBU + ECN));
                            let ECQ = EBP - ((EBT + ECO) * YE);
                            EBW = EBR;
                            EBX = ECP;
                            EBY = EAY;
                            EBZ = ECQ;
                        } else {
                            EBW = S;
                            EBX = S;
                            EBY = T;
                            EBZ = ATP;
                        }
                        let ECC = (((YQ - ABK) - EAX) / ECA) * ECB;
                        let ECD = (((BCO - ABL) - EBP) / ECA) * ECB;
                        let ECE = if (if -1e2f64 < ECC { 1.0 } else { 0.0 }) != 0.0 && (if ECC < JF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let ECW;
                        let ECX;
                        if ECE != 0.0 {
                            let ECR = ECC.exp();
                            let ECT = ECS * ECR;
                            let ECU = (ECD * ECR) * ECS;
                            ECW = ECT;
                            ECX = ECU;
                        } else {
                            let ECV = if ECC <= -1e2f64 { 1.0 } else { 0.0 };
                            let EDK = if ECV != 0.0 {
                                let EDI = ECS * JO;
                                EDI
                            } else {
                                let EDJ = ECS * JH;
                                EDJ
                            };
                            ECW = EDK;
                            ECX = ATP;
                        }
                        let ECY = ECX * DW;
                        let EDA = (ECS - ECW) - ECZ;
                        let EDB = ECY * EDA;
                        let EDD = ((EDA * EDA) + EDC).sqrt();
                        let EDE = ECS - (YE * (EDA + EDD));
                        let EDF = ((ECY + ((EDB + EDB) * (EA / (DZ * EDD)))) * YE) * DW;
                        let EDH = if EDE < EDG { 1.0 } else { 0.0 };
                        let EDL;
                        let EDM;
                        if EDH != 0.0 {
                            EDL = EDG;
                            EDM = ATP;
                        } else {
                            EDL = EDE;
                            EDM = EDF;
                        }
                        let EDQ;
                        let EDR;
                        if DMF != 0.0 {
                            let EDN = (((WO - ABK) - EBW) / ECA) * ECB;
                            let EDO = (((Lanes([0.0, 0.0, 0.0, XL[0], XL[1], XL[2]]) - ABL) - Lanes([0.0, 0.0, EBY, 0.0, 0.0, 0.0])) / ECA) * ECB;
                            let EDP = if (if -1e2f64 < EDN { 1.0 } else { 0.0 }) != 0.0 && (if EDN < JF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let EEE;
                            let EEF;
                            if EDP != 0.0 {
                                let EEA = EDN.exp();
                                let EEB = ECS * EEA;
                                let EEC = (EDO * EEA) * ECS;
                                EEE = EEB;
                                EEF = EEC;
                            } else {
                                let EED = if EDN <= -1e2f64 { 1.0 } else { 0.0 };
                                let EEP = if EED != 0.0 {
                                    let EEN = ECS * JO;
                                    EEN
                                } else {
                                    let EEO = ECS * JH;
                                    EEO
                                };
                                EEE = EEP;
                                EEF = ATP;
                            }
                            let EEG = EEF * DW;
                            let EEH = (ECS - EEE) - ECZ;
                            let EEI = EEG * EEH;
                            let EEJ = ((EEH * EEH) + EDC).sqrt();
                            let EEK = ECS - (YE * (EEH + EEJ));
                            let EEL = ((EEG + ((EEI + EEI) * (EA / (DZ * EEJ)))) * YE) * DW;
                            let EEM = if EEK < EDG { 1.0 } else { 0.0 };
                            let EEQ;
                            let EER;
                            if EEM != 0.0 {
                                EEQ = EDG;
                                EER = ATP;
                            } else {
                                EEQ = EEK;
                                EER = EEL;
                            }
                            EDQ = EEQ;
                            EDR = EER;
                        } else {
                            EDQ = S;
                            EDR = ATP;
                        }
                        let EDS = AXR / EDL;
                        let EDU = EDT + EDS;
                        let EDV = EDT / EDU;
                        let EDW = (((EDM * EDS) * DW) / EDL) * EDV;
                        let EDX = EDV * EDS;
                        let EDY = (((EDW * DW) / EDU) * EDS) + EDW;
                        let EEY;
                        let EEZ;
                        if EDZ != 0.0 {
                            let EES = AXR / EDQ;
                            let EET = EDT + EES;
                            let EEU = EDT / EET;
                            let EEV = (((EDR * EES) * DW) / EDQ) * EEU;
                            let EEW = EEU * EES;
                            let EEX = (((EEV * DW) / EET) * EES) + EEV;
                            EEY = EEW;
                            EEZ = EEX;
                        } else {
                            EEY = S;
                            EEZ = ATP;
                        }
                        let EFB = (EFA * EDX) / EDT;
                        let EFC = (EDY * EFA) / EDT;
                        let EFG;
                        let EFH;
                        if DMF != 0.0 {
                            let EFE = (EFD * EEY) / EDT;
                            let EFF = (EEZ * EFD) / EDT;
                            EFG = EFE;
                            EFH = EFF;
                        } else {
                            EFG = S;
                            EFH = ATP;
                        }
                        let EFI = EBO - EAX;
                        let EFJ = EFB * EFI;
                        let EFK = (EFC * EFI) + ((EBQ - EBP) * EFB);
                        let EFP;
                        let EFQ;
                        if EFL != 0.0 {
                            let EFM = EBX - EBW;
                            let EFN = EFJ + (EFG * EFM);
                            let EFO = EFK + ((EFH * EFM) + ((EBZ - Lanes([0.0, 0.0, EBY, 0.0, 0.0, 0.0])) * EFG));
                            EFP = EFN;
                            EFQ = EFO;
                        } else {
                            EFP = EFJ;
                            EFQ = EFK;
                        }
                        let EFR = ((YQ - EBO) - ABK) - DLF;
                        let EFS = ((BCO - EBQ) - ABL) - DLH;
                        let EFV;
                        let EFW;
                        if EFT != 0.0 {
                            EFV = S;
                            EFW = ATP;
                        } else {
                            let EFU = if EFR < S { 1.0 } else { 0.0 };
                            let EGH;
                            let EGI;
                            if EFU != 0.0 {
                                let EGD = EFS / AKA;
                                let EGE = EFY + (EFR / AKA);
                                EGH = EGE;
                                EGI = EGD;
                            } else {
                                let EGF = ((EFY * EFY) + EFR).sqrt();
                                let EGG = EFS * (EA / (DZ * EGF));
                                EGH = EGF;
                                EGI = EGG;
                            }
                            EFV = EGH;
                            EFW = EGI;
                        }
                        let EFX = EFB * AKA;
                        let EFZ = EFV - EFY;
                        let EGA = EFX * EFZ;
                        let EGB = ((EFC * AKA) * EFZ) + (EFW * EFX);
                        let EGL;
                        let EGM;
                        if EGC != 0.0 {
                            let EGJ = ((WO - EBX) - ABK) - DLG;
                            let EGK = ((Lanes([0.0, 0.0, 0.0, XL[0], XL[1], XL[2]]) - EBZ) - ABL) - DLI;
                            let EGO;
                            let EGP;
                            if EFT != 0.0 {
                                EGO = S;
                                EGP = ATP;
                            } else {
                                let EGN = if EGJ < S { 1.0 } else { 0.0 };
                                let EGY;
                                let EGZ;
                                if EGN != 0.0 {
                                    let EGU = EGK / AKA;
                                    let EGV = EFY + (EGJ / AKA);
                                    EGY = EGV;
                                    EGZ = EGU;
                                } else {
                                    let EGW = ((EFY * EFY) + EGJ).sqrt();
                                    let EGX = EGK * (EA / (DZ * EGW));
                                    EGY = EGW;
                                    EGZ = EGX;
                                }
                                EGO = EGY;
                                EGP = EGZ;
                            }
                            let EGQ = EFG * AKA;
                            let EGR = EGO - EFY;
                            let EGS = EGA + (EGQ * EGR);
                            let EGT = EGB + (((EFH * AKA) * EGR) + (EGP * EGQ));
                            EGL = EGS;
                            EGM = EGT;
                        } else {
                            EGL = EGA;
                            EGM = EGB;
                        }
                        EAJ = EAX;
                        EAK = EBW;
                        EAL = EFG;
                        EAM = EFP;
                        EAN = EGL;
                        EAO = EAY;
                        EAP = EBY;
                        EAQ = EFH;
                        EAR = EFQ;
                        EAS = EGM;
                    }
                    let EHG;
                    let EHH;
                    if EAT != 0.0 {
                        let EHB = EHA * ZK;
                        let EHC = ZL * EHA;
                        EHG = EHB;
                        EHH = EHC;
                    } else {
                        let EHE = ((EHD * ZK) * AKA) * AKA;
                        let EHF = ((ZL * EHD) * AKA) * AKA;
                        EHG = EHE;
                        EHH = EHF;
                    }
                    let EHJ = EHI + DLF;
                    let EHK = (EHJ * DLF) / EHG;
                    let EHL = (((DLH * DLF) + (DLH * EHJ)) - Lanes([0.0, 0.0, (EHH * EHK), 0.0, 0.0, 0.0])) / EHG;
                    let EHM = AA + EHK;
                    let EHN = if EHM > FJ { 1.0 } else { 0.0 };
                    let EHR;
                    let EHS;
                    if EHN != 0.0 {
                        let EHO = EHM.ln();
                        let EHP = EHL * (EA / EHM);
                        EHR = EHO;
                        EHS = EHP;
                    } else {
                        EHR = EHQ;
                        EHS = ATP;
                    }
                    let EHT = ZK * EHR;
                    let EHU = Lanes([0.0, 0.0, (ZL * EHR), 0.0, 0.0, 0.0]) + (EHS * ZK);
                    let EIA;
                    let EIB;
                    if DMF != 0.0 {
                        let EHV = EHI + DLG;
                        let EHW = (EHV * DLG) / EHG;
                        let EHX = (((DLI * DLG) + (DLI * EHV)) - Lanes([0.0, 0.0, (EHH * EHW), 0.0, 0.0, 0.0])) / EHG;
                        let EHY = AA + EHW;
                        let EHZ = if EHY > FJ { 1.0 } else { 0.0 };
                        let EIN;
                        let EIO;
                        if EHZ != 0.0 {
                            let EIK = EHY.ln();
                            let EIL = EHX * (EA / EHY);
                            EIN = EIK;
                            EIO = EIL;
                        } else {
                            EIN = EIM;
                            EIO = ATP;
                        }
                        let EIP = ZK * EIN;
                        let EIQ = Lanes([0.0, 0.0, (ZL * EIN), 0.0, 0.0, 0.0]) + (EIO * ZK);
                        EIA = EIP;
                        EIB = EIQ;
                    } else {
                        EIA = S;
                        EIB = ATP;
                    }
                    let EIC = CJN * ((BCC - EAJ) - BG);
                    let EID = ((BCD - Lanes([0.0, 0.0, EAO, 0.0, 0.0, 0.0])) - ABM) * CJN;
                    let EIE = EID * EIC;
                    let EIF = ((EIC * EIC) + AIQ).sqrt();
                    let EIH = (DLF + (YE * (EIC + EIF))) / EIG;
                    let EII = (DLH + ((EID + ((EIE + EIE) * (EA / (DZ * EIF)))) * YE)) / EIG;
                    let EIJ = if EIH > FJ { 1.0 } else { 0.0 };
                    let EIU;
                    let EIV;
                    if EIJ != 0.0 {
                        let EIR = EIH.ln();
                        let EIS = EII * (EA / EIH);
                        EIU = EIR;
                        EIV = EIS;
                    } else {
                        EIU = EIT;
                        EIV = ATP;
                    }
                    let EIX = (EIW * EIU).exp();
                    let EIY = AA + EIX;
                    let EJA = EIZ / EIY;
                    let EJB = AXR / EJA;
                    let EJC = EDT + EJB;
                    let EJD = EDT / EJC;
                    let EJE = ((((((((EIV * EIW) * EIX) * EJA) * DW) / EIY) * EJB) * DW) / EJA) * EJD;
                    let EJF = EJD * EJB;
                    let EJG = (((EJE * DW) / EJC) * EJB) + EJE;
                    let EJI = (EJH * EJF) / EDT;
                    let EJJ = (EJG * EJH) / EDT;
                    let EJK = (EFA * EJF) / EDT;
                    let EJL = (EJG * EFA) / EDT;
                    let EJU;
                    let EJV;
                    let EJW;
                    let EJX;
                    if EJM != 0.0 {
                        let EJN = CJN * (((BCC + CZE) - EAK) - BG);
                        let EJO = ((BCD - Lanes([0.0, 0.0, EAP, 0.0, 0.0, 0.0])) - ABM) * CJN;
                        let EJP = EJO * EJN;
                        let EJQ = ((EJN * EJN) + AIQ).sqrt();
                        let EJR = (DLG + (YE * (EJN + EJQ))) / EIG;
                        let EJS = (DLI + ((EJO + ((EJP + EJP) * (EA / (DZ * EJQ)))) * YE)) / EIG;
                        let EJT = if EJR > FJ { 1.0 } else { 0.0 };
                        let ELA;
                        let ELB;
                        if EJT != 0.0 {
                            let EKX = EJR.ln();
                            let EKY = EJS * (EA / EJR);
                            ELA = EKX;
                            ELB = EKY;
                        } else {
                            ELA = EKZ;
                            ELB = ATP;
                        }
                        let ELC = (EIW * ELA).exp();
                        let ELD = AA + ELC;
                        let ELE = EIZ / ELD;
                        let ELF = AXR / ELE;
                        let ELG = EDT + ELF;
                        let ELH = EDT / ELG;
                        let ELI = ((((((((ELB * EIW) * ELC) * ELE) * DW) / ELD) * ELF) * DW) / ELE) * ELH;
                        let ELJ = ELH * ELF;
                        let ELK = (((ELI * DW) / ELG) * ELF) + ELI;
                        let ELM = (ELL * ELJ) / EDT;
                        let ELN = (ELK * ELL) / EDT;
                        let ELO = (EFD * ELJ) / EDT;
                        let ELP = (ELK * EFD) / EDT;
                        EJU = ELM;
                        EJV = ELO;
                        EJW = ELN;
                        EJX = ELP;
                    } else {
                        EJU = S;
                        EJV = EAL;
                        EJW = ATP;
                        EJX = EAQ;
                    }
                    let EJY = DLF - EHT;
                    let EJZ = DLH - EHU;
                    let EKA = BJD * DSA;
                    let EKB = BJE * DSA;
                    let EKC = EJY / EKA;
                    let EKD = (EJZ - (EKB * EKC)) / EKA;
                    let EKE = EKD - BPJ;
                    let EKF = (EKC - WR) - ADI;
                    let EKG = EKE * EKF;
                    let EKI = ((EKF * EKF) + (EKH * EKC)).sqrt();
                    let EKJ = EKC - (YE * (EKF + EKI));
                    let EKK = EKD - ((EKE + (((EKG + EKG) + (EKD * EKH)) * (EA / (DZ * EKI)))) * YE);
                    let EKL = EKA * EKJ;
                    let EKM = (EKB * EKJ) + (EKK * EKA);
                    let EKN = YE * EKL;
                    let EKO = EKM * YE;
                    let EKP = DVS * ((EJY - EKN) + DVR);
                    let EKQ = (EJZ - EKO) * DVS;
                    let EKR = EKL / EKP;
                    let EKS = YE - EKR;
                    let EKT = EJY - (EKL * EKS);
                    let EKU = EJI * EKT;
                    let EKV = (EJJ * EKT) + ((EJZ - ((EKM * EKS) + ((((EKM - (EKQ * EKR)) / EKP) * DW) * EKL))) * EJI);
                    let EMK;
                    let EML;
                    let EMM;
                    let EMN;
                    let EMO;
                    let EMP;
                    let EMQ;
                    let EMR;
                    let EMS;
                    let EMT;
                    if EKW != 0.0 {
                        let ELQ = DLG - EIA;
                        let ELR = DLI - EIB;
                        let ELS = ELQ / EKA;
                        let ELT = (ELR - (EKB * ELS)) / EKA;
                        let ELU = ELT - BPJ;
                        let ELV = (ELS - WR) - ADI;
                        let ELW = ELU * ELV;
                        let ELY = ((ELV * ELV) + (ELX * ELS)).sqrt();
                        let ELZ = ELS - (YE * (ELV + ELY));
                        let EMA = ELT - ((ELU + (((ELW + ELW) + (ELT * ELX)) * (EA / (DZ * ELY)))) * YE);
                        let EMB = EKA * ELZ;
                        let EMC = (EKB * ELZ) + (EMA * EKA);
                        let EMD = DVS * ((ELQ - (YE * EMB)) + DVR);
                        let EME = (ELR - (EMC * YE)) * DVS;
                        let EMF = EMB / EMD;
                        let EMG = YE - EMF;
                        let EMH = ELQ - (EMB * EMG);
                        let EMI = EKU + (EJU * EMH);
                        let EMJ = EKV + ((EJW * EMH) + ((ELR - ((EMC * EMG) + ((((EMC - (EME * EMF)) / EMD) * DW) * EMB))) * EJU));
                        EMK = ELZ;
                        EML = EMB;
                        EMM = EMD;
                        EMN = ELQ;
                        EMO = EMI;
                        EMP = EMA;
                        EMQ = EMC;
                        EMR = EME;
                        EMS = ELR;
                        EMT = EMJ;
                    } else {
                        EMK = S;
                        EML = S;
                        EMM = S;
                        EMN = CZO;
                        EMO = EKU;
                        EMP = ATP;
                        EMQ = ATP;
                        EMR = ATP;
                        EMS = ATP;
                        EMT = EKV;
                    }
                    let ENC;
                    let END;
                    if EAI != 0.0 {
                        ENC = S;
                        END = ATP;
                    } else {
                        let EMU = AA - EKA;
                        let EMV = EKB * DW;
                        let EMW = EJK * EMU;
                        let EMX = (EKL * EKJ) / EKP;
                        let EMY = (YE * EKJ) - EMX;
                        let EMZ = EMW * EMY;
                        let ENA = (((EJL * EMU) + (EMV * EJK)) * EMY) + (((EKK * YE) - ((((EKM * EKJ) + (EKK * EKL)) - (EKQ * EMX)) / EKP)) * EMW);
                        let ENK;
                        let ENL;
                        if ENB != 0.0 {
                            let ENF = EJV * EMU;
                            let ENG = (EML * EMK) / EMM;
                            let ENH = (YE * EMK) - ENG;
                            let ENI = EMZ + (ENF * ENH);
                            let ENJ = ENA + ((((EJX * EMU) + (EMV * EJV)) * ENH) + (((EMP * YE) - ((((EMQ * EMK) + (EMP * EML)) - (EMR * ENG)) / EMM)) * ENF));
                            ENK = ENI;
                            ENL = ENJ;
                        } else {
                            ENK = EMZ;
                            ENL = ENA;
                        }
                        ENC = ENK;
                        END = ENL;
                    }
                    let ENT;
                    let ENU;
                    if ENE != 0.0 {
                        let ENM = -EJI;
                        let ENN = (EKN * EKL) / EKP;
                        let ENO = ((EJY / EG) + (EKL / CJN)) - ENN;
                        let ENP = ENM * ENO;
                        let ENQ = ((EJJ * DW) * ENO) + ((((EJZ / EG) + (EKM / CJN)) - ((((EKO * EKL) + (EKM * EKN)) - (EKQ * ENN)) / EKP)) * ENM);
                        let EOB;
                        let EOC;
                        if ENR != 0.0 {
                            let ENV = -EJU;
                            let ENW = YE * EML;
                            let ENX = (ENW * EML) / EMM;
                            let ENY = (((DLG - EIA) / EG) + (EML / CJN)) - ENX;
                            let ENZ = ENP + (ENV * ENY);
                            let EOA = ENQ + (((EJW * DW) * ENY) + (((((DLI - EIB) / EG) + (EMQ / CJN)) - (((((EMQ * YE) * EML) + (EMQ * ENW)) - (EMR * ENX)) / EMM)) * ENV));
                            EOB = ENZ;
                            EOC = EOA;
                        } else {
                            EOB = ENP;
                            EOC = ENQ;
                        }
                        ENT = EOB;
                        ENU = EOC;
                    } else {
                        let EOU;
                        let EOV;
                        if ENS != 0.0 {
                            let EOD = EKP / DVS;
                            let EOE = EOD * EOD;
                            let EOF = (EKQ / DVS) * EOD;
                            let EOG = (YE * EJI) / EOE;
                            let EOH = EG * EKL;
                            let EOI = EOH * EKL;
                            let EOJ = ((EKM * EG) * EKL) + (EKM * EOH);
                            let EOK = EJY - ((CJN * EKL) / AEC);
                            let EOL = (EOI / AEC) + (EJY * EOK);
                            let EOM = (EJY * EOL) - ((EOI * EKL) / DYR);
                            let EON = -EOG;
                            let EOO = EON * EOM;
                            let EOP = (((((EJJ * YE) - ((EOF + EOF) * EOG)) / EOE) * DW) * EOM) + ((((EJZ * EOL) + (((EOJ / AEC) + ((EJZ * EOK) + ((EJZ - ((EKM * CJN) / AEC)) * EJY))) * EJY)) - (((EOJ * EKL) + (EKM * EOI)) / DYR)) * EON);
                            let EPJ;
                            let EPK;
                            if EOQ != 0.0 {
                                let EOW = EMM / DVS;
                                let EOX = EOW * EOW;
                                let EOY = (EMR / DVS) * EOW;
                                let EOZ = (YE * EJU) / EOX;
                                let EPA = EG * EML;
                                let EPB = EPA * EML;
                                let EPC = ((EMQ * EG) * EML) + (EMQ * EPA);
                                let EPD = EMN - ((CJN * EML) / AEC);
                                let EPE = (EPB / AEC) + (EMN * EPD);
                                let EPF = (EMN * EPE) - ((EPB * EML) / DYR);
                                let EPG = -EOZ;
                                let EPH = EOO + (EPG * EPF);
                                let EPI = EOP + ((((((EJW * YE) - ((EOY + EOY) * EOZ)) / EOX) * DW) * EPF) + ((((EMS * EPE) + (((EPC / AEC) + ((EMS * EPD) + ((EMS - ((EMQ * CJN) / AEC)) * EMN))) * EMN)) - (((EPC * EML) + (EMQ * EPB)) / DYR)) * EPG));
                                EPJ = EPH;
                                EPK = EPI;
                            } else {
                                EPJ = EOO;
                                EPK = EOP;
                            }
                            EOU = EPJ;
                            EOV = EPK;
                        } else {
                            let EOS = EOR * EMO;
                            let EOT = EMT * EOR;
                            EOU = EOS;
                            EOV = EOT;
                        }
                        ENT = EOU;
                        ENU = EOV;
                    }
                    let EPO;
                    let EPP;
                    if EAI != 0.0 {
                        EPO = S;
                        EPP = ATP;
                    } else {
                        let EPM = EPL * (XQ - ZT);
                        let EPN = (Lanes([XR[0], 0.0, XR[1], XR[2], XR[3], 0.0]) - ZW) * EPL;
                        EPO = EPM;
                        EPP = EPN;
                    }
                    let EPQ = ((EMO + EAM) + EAN) - ENC;
                    let EPR = ((EMT + EAR) + EAS) - END;
                    let EPS = -(((EPQ + (((ENC - EAM) - EAN) - EPO)) + EPO) + ENT);
                    let EPT = (((EPR + (((END - EAR) - EAS) - EPP)) + EPP) + ENU) * DW;
                    EAA = EPQ;
                    EAB = EPS;
                    EAC = ENT;
                    EAD = EPO;
                    EAE = EPR;
                    EAF = EPT;
                    EAG = ENU;
                    EAH = EPP;
                } else {
                    EAA = S;
                    EAB = S;
                    EAC = S;
                    EAD = S;
                    EAE = ATP;
                    EAF = ATP;
                    EAG = ATP;
                    EAH = ATP;
                }
                DRH = EAA;
                DRI = EAB;
                DRJ = EAC;
                DRK = EAD;
                DRL = EAE;
                DRM = EAF;
                DRN = EAG;
                DRO = EAH;
            }
            let EQI;
            let EQJ;
            let EQK;
            let EQL;
            if DRP != 0.0 {
                EQI = S;
                EQJ = S;
                EQK = BUA;
                EQL = BTZ;
            } else {
                let EPU = W - X;
                let EPW = V * EPV;
                let EPX = staged[393] + (EPV * EPU);
                let EPZ = V * EPY;
                let EQA = staged[395] + (EPY * EPU);
                let EQC = V * EQB;
                let EQD = staged[397] + (EQB * EPU);
                let EQF = EQE * EPX;
                let EQG = EPW * EQE;
                let EQH = if UT > EQF { 1.0 } else { 0.0 };
                let EQU;
                let EQV;
                if EQH != 0.0 {
                    let EQS = Lanes([EQG, 0.0, 0.0]);
                    EQU = EQF;
                    EQV = EQS;
                } else {
                    let EQT = Lanes([0.0, UU[0], UU[1]]);
                    EQU = UT;
                    EQV = EQT;
                }
                let EQW = EQU / EPX;
                let EQX = AA - EQW;
                let EQY = ((EQV - Lanes([(EPW * EQW), 0.0, 0.0])) / EPX) * DW;
                let ERE;
                let ERF;
                if EQZ != 0.0 {
                    let ERA = EQX.sqrt();
                    let ERB = AA / ERA;
                    let ERC = (((EQY * (EA / (DZ * ERA))) * ERB) * DW) / ERA;
                    ERE = ERB;
                    ERF = ERC;
                } else {
                    let ERD = if EQX > FJ { 1.0 } else { 0.0 };
                    let ERM;
                    let ERN;
                    if ERD != 0.0 {
                        let ERJ = EQX.ln();
                        let ERK = EQY * (EA / EQX);
                        ERM = ERJ;
                        ERN = ERK;
                    } else {
                        ERM = ERL;
                        ERN = BTZ;
                    }
                    let ERP = (ERO * ERM).exp();
                    let ERQ = (ERN * ERO) * ERP;
                    ERE = ERP;
                    ERF = ERQ;
                }
                let ERG = AA - (EQX * ERE);
                let ERH = ERG * EPX;
                let ERI = ((((EQY * ERE) + (ERF * EQX)) * DW) * EPX) + Lanes([(EPW * ERG), 0.0, 0.0]);
                let ERU;
                let ERV;
                if EQH != 0.0 {
                    let ERR = UT - EQF;
                    let ERS = ERH + (ERE * ERR);
                    let ERT = ERI + ((ERF * ERR) + ((Lanes([0.0, UU[0], UU[1]]) - Lanes([EQG, 0.0, 0.0])) * ERE));
                    ERU = ERS;
                    ERV = ERT;
                } else {
                    ERU = ERH;
                    ERV = ERI;
                }
                let ERX = (EQA * ERU) + ((ERW * BUH) * DGP);
                let ERY = (Lanes([(EPZ * ERU), 0.0, 0.0]) + (ERV * EQA)) + ((BUO * ERW) * DGP);
                let ESA = V * ERZ;
                let ESB = staged[399] + (ERZ * EPU);
                let ESC = EQE * ESB;
                let ESD = ESA * EQE;
                let ESE = if UX > ESC { 1.0 } else { 0.0 };
                let ESH;
                let ESI;
                if ESE != 0.0 {
                    let ESF = Lanes([ESD, 0.0, 0.0]);
                    ESH = ESC;
                    ESI = ESF;
                } else {
                    let ESG = Lanes([0.0, UY[0], UY[1]]);
                    ESH = UX;
                    ESI = ESG;
                }
                let ESJ = ESH / ESB;
                let ESK = AA - ESJ;
                let ESL = ((ESI - Lanes([(ESA * ESJ), 0.0, 0.0])) / ESB) * DW;
                let ESR;
                let ESS;
                if ESM != 0.0 {
                    let ESN = ESK.sqrt();
                    let ESO = AA / ESN;
                    let ESP = (((ESL * (EA / (DZ * ESN))) * ESO) * DW) / ESN;
                    ESR = ESO;
                    ESS = ESP;
                } else {
                    let ESQ = if ESK > FJ { 1.0 } else { 0.0 };
                    let ETA;
                    let ETB;
                    if ESQ != 0.0 {
                        let ESX = ESK.ln();
                        let ESY = ESL * (EA / ESK);
                        ETA = ESX;
                        ETB = ESY;
                    } else {
                        ETA = ESZ;
                        ETB = BUA;
                    }
                    let ETD = (ETC * ETA).exp();
                    let ETE = (ETB * ETC) * ETD;
                    ESR = ETD;
                    ESS = ETE;
                }
                let EST = AA - (ESK * ESR);
                let ESV = (EST * ESB) / ESU;
                let ESW = (((((ESL * ESR) + (ESS * ESK)) * DW) * ESB) + Lanes([(ESA * EST), 0.0, 0.0])) / ESU;
                let ETI;
                let ETJ;
                if ESE != 0.0 {
                    let ETF = UX - ESC;
                    let ETG = ESV + (ESR * ETF);
                    let ETH = ESW + ((ESS * ETF) + ((Lanes([0.0, UY[0], UY[1]]) - Lanes([ESD, 0.0, 0.0])) * ESR));
                    ETI = ETG;
                    ETJ = ETH;
                } else {
                    ETI = ESV;
                    ETJ = ESW;
                }
                let ETK = (EQD * ETI) + ((ERW * BUI) * DGP);
                let ETL = (Lanes([(EQC * ETI), 0.0, 0.0]) + (ETJ * EQD)) + ((BUP * ERW) * DGP);
                EQI = ETK;
                EQJ = ERX;
                EQK = ETL;
                EQL = ERY;
            }
            let EQN = EQM * UJ;
            let EQO = UK * EQM;
            let EQP = SY * (TX - UJ);
            let EQQ = (VL - VK) * SY;
            let ETT;
            let ETU;
            let ETV;
            let ETW;
            if EQR != 0.0 {
                let EUI;
                let EUJ;
                if ETM != 0.0 {
                    let EUF = if EQN < EUE { 1.0 } else { 0.0 };
                    let EUO;
                    let EUP;
                    if EUF != 0.0 {
                        let EUK = ETN * (EQN - EUE);
                        let EUL = EQO * ETN;
                        EUO = EUK;
                        EUP = EUL;
                    } else {
                        let EUN = if EQN < EUM { 1.0 } else { 0.0 };
                        let EUY;
                        let EUZ;
                        if EUN != 0.0 {
                            let EUQ = EQN - EUE;
                            let EUR = EQO * EUQ;
                            let EUT = EUS / AEC;
                            let EUU = ETN - (EUT * (EUQ * EUQ));
                            let EUV = EUQ * EUU;
                            let EUW = (EQO * EUU) + ((((EUR + EUR) * EUT) * DW) * EUQ);
                            EUY = EUV;
                            EUZ = EUW;
                        } else {
                            let EUX = if EQN < EUG { 1.0 } else { 0.0 };
                            let EVM;
                            let EVN;
                            if EUX != 0.0 {
                                let EVA = EQN - EUG;
                                let EVB = EVA * EVA;
                                let EVC = EQO * EVA;
                                let EVG = EVF / AEC;
                                let EVH = EVG * EVA;
                                let EVI = ((EVD * EQN) + EVE) + (EVH * EVB);
                                let EVJ = (EQO * EVD) + (((EQO * EVG) * EVB) + ((EVC + EVC) * EVH));
                                EVM = EVI;
                                EVN = EVJ;
                            } else {
                                let EVK = EQO * EVD;
                                let EVL = (EVD * EQN) + EVE;
                                EVM = EVL;
                                EVN = EVK;
                            }
                            EUY = EVM;
                            EUZ = EVN;
                        }
                        EUO = EUY;
                        EUP = EUZ;
                    }
                    EUI = EUO;
                    EUJ = EUP;
                } else {
                    let EUH = if EQN < EUG { 1.0 } else { 0.0 };
                    let EVR;
                    let EVS;
                    if EUH != 0.0 {
                        let EVO = EVD * (EQN - EUG);
                        let EVP = EQO * EVD;
                        EVR = EVO;
                        EVS = EVP;
                    } else {
                        let EVQ = if EQN < EUM { 1.0 } else { 0.0 };
                        let EWA;
                        let EWB;
                        if EVQ != 0.0 {
                            let EVT = EQN - EUG;
                            let EVU = EQO * EVT;
                            let EVV = EUS / AEC;
                            let EVW = EVD - (EVV * (EVT * EVT));
                            let EVX = EVT * EVW;
                            let EVY = (EQO * EVW) + ((((EVU + EVU) * EVV) * DW) * EVT);
                            EWA = EVX;
                            EWB = EVY;
                        } else {
                            let EVZ = if EQN < EUE { 1.0 } else { 0.0 };
                            let EWL;
                            let EWM;
                            if EVZ != 0.0 {
                                let EWC = EQN - EUE;
                                let EWD = EWC * EWC;
                                let EWE = EQO * EWC;
                                let EWF = EVF / AEC;
                                let EWG = EWF * EWC;
                                let EWH = ((ETN * EQN) + EVE) + (EWG * EWD);
                                let EWI = (EQO * ETN) + (((EQO * EWF) * EWD) + ((EWE + EWE) * EWG));
                                EWL = EWH;
                                EWM = EWI;
                            } else {
                                let EWJ = EQO * ETN;
                                let EWK = (ETN * EQN) + EVE;
                                EWL = EWK;
                                EWM = EWJ;
                            }
                            EWA = EWL;
                            EWB = EWM;
                        }
                        EVR = EWA;
                        EVS = EWB;
                    }
                    EUI = EVR;
                    EUJ = EVS;
                }
                let EWP;
                let EWQ;
                if ETM != 0.0 {
                    let EWN = if EQP < EUE { 1.0 } else { 0.0 };
                    let EWU;
                    let EWV;
                    if EWN != 0.0 {
                        let EWR = ETQ * (EQP - EUE);
                        let EWS = EQQ * ETQ;
                        EWU = EWR;
                        EWV = EWS;
                    } else {
                        let EWT = if EQP < EUM { 1.0 } else { 0.0 };
                        let EXE;
                        let EXF;
                        if EWT != 0.0 {
                            let EWW = EQP - EUE;
                            let EWX = EQQ * EWW;
                            let EWZ = EWY / AEC;
                            let EXA = ETQ - (EWZ * (EWW * EWW));
                            let EXB = EWW * EXA;
                            let EXC = (EQQ * EXA) + ((((EWX + EWX) * EWZ) * DW) * EWW);
                            EXE = EXB;
                            EXF = EXC;
                        } else {
                            let EXD = if EQP < EUG { 1.0 } else { 0.0 };
                            let EXS;
                            let EXT;
                            if EXD != 0.0 {
                                let EXG = EQP - EUG;
                                let EXH = EXG * EXG;
                                let EXI = EQQ * EXG;
                                let EXM = EXL / AEC;
                                let EXN = EXM * EXG;
                                let EXO = ((EXJ * EQP) + EXK) + (EXN * EXH);
                                let EXP = (EQQ * EXJ) + (((EQQ * EXM) * EXH) + ((EXI + EXI) * EXN));
                                EXS = EXO;
                                EXT = EXP;
                            } else {
                                let EXQ = EQQ * EXJ;
                                let EXR = (EXJ * EQP) + EXK;
                                EXS = EXR;
                                EXT = EXQ;
                            }
                            EXE = EXS;
                            EXF = EXT;
                        }
                        EWU = EXE;
                        EWV = EXF;
                    }
                    EWP = EWU;
                    EWQ = EWV;
                } else {
                    let EWO = if EQP < EUG { 1.0 } else { 0.0 };
                    let EXX;
                    let EXY;
                    if EWO != 0.0 {
                        let EXU = EXJ * (EQP - EUG);
                        let EXV = EQQ * EXJ;
                        EXX = EXU;
                        EXY = EXV;
                    } else {
                        let EXW = if EQP < EUM { 1.0 } else { 0.0 };
                        let EYG;
                        let EYH;
                        if EXW != 0.0 {
                            let EXZ = EQP - EUG;
                            let EYA = EQQ * EXZ;
                            let EYB = EWY / AEC;
                            let EYC = EXJ - (EYB * (EXZ * EXZ));
                            let EYD = EXZ * EYC;
                            let EYE = (EQQ * EYC) + ((((EYA + EYA) * EYB) * DW) * EXZ);
                            EYG = EYD;
                            EYH = EYE;
                        } else {
                            let EYF = if EQP < EUE { 1.0 } else { 0.0 };
                            let EYR;
                            let EYS;
                            if EYF != 0.0 {
                                let EYI = EQP - EUE;
                                let EYJ = EYI * EYI;
                                let EYK = EQQ * EYI;
                                let EYL = EXL / AEC;
                                let EYM = EYL * EYI;
                                let EYN = ((ETQ * EQP) + EXK) + (EYM * EYJ);
                                let EYO = (EQQ * ETQ) + (((EQQ * EYL) * EYJ) + ((EYK + EYK) * EYM));
                                EYR = EYN;
                                EYS = EYO;
                            } else {
                                let EYP = EQQ * ETQ;
                                let EYQ = (ETQ * EQP) + EXK;
                                EYR = EYQ;
                                EYS = EYP;
                            }
                            EYG = EYR;
                            EYH = EYS;
                        }
                        EXX = EYG;
                        EXY = EYH;
                    }
                    EWP = EXX;
                    EWQ = EXY;
                }
                ETT = EUI;
                ETU = EWP;
                ETV = EUJ;
                ETW = EWQ;
            } else {
                let ETO = ETN * EQN;
                let ETP = EQO * ETN;
                let ETR = ETQ * EQP;
                let ETS = EQQ * ETQ;
                ETT = ETO;
                ETU = ETR;
                ETV = ETP;
                ETW = ETS;
            }
            let ETY = ETT + (ETX * EQN);
            let ETZ = ETV + (EQO * ETX);
            let EUB = ETU + (EUA * EQP);
            let EUC = ETW + (EQQ * EUA);
            let EYX;
            let EYY;
            if EUD != 0.0 {
                let EYT = VN + ADI;
                let EYU = Lanes([VO[0], VO[1], 0.0, VO[2]]);
                EYX = EYT;
                EYY = EYU;
            } else {
                let EYV = VG + ADI;
                let EYW = Lanes([VI[0], VI[1], VI[2], 0.0]);
                EYX = EYV;
                EYY = EYW;
            }
            let EYZ = EYY * EYX;
            let EZA = ((EYX * EYX) + 8e-2f64).sqrt();
            let EZB = YE * (EYX - EZA);
            let EZC = (EYY - ((EYZ + EYZ) * (EA / (DZ * EZA)))) * YE;
            let EZE = (AA - ((CJN * EZB) / EZD)).sqrt();
            let EZF = (((EZC * CJN) / EZD) * DW) * (EA / (DZ * EZE));
            let EZR;
            let EZS;
            if EUD != 0.0 {
                let EZH = VO * EZG;
                let EZK = (EZG * VN) - (EZJ * (EZB + (EZI * (EZE - AA))));
                let EZL = Lanes([EZH[0], EZH[1], 0.0, EZH[2]]) - ((EZC + (EZF * EZI)) * EZJ);
                EZR = EZK;
                EZS = EZL;
            } else {
                let EZN = VI * EZM;
                let EZP = (EZM * VG) - (EZJ * (EZB + (EZO * (EZE - AA))));
                let EZQ = Lanes([EZN[0], EZN[1], EZN[2], 0.0]) - ((EZC + (EZF * EZO)) * EZJ);
                EZR = EZP;
                EZS = EZQ;
            }
            let EZX;
            let EZY;
            if EUD != 0.0 {
                let EZT = VB + ADI;
                let EZU = Lanes([VC[0], 0.0, VC[1]]);
                EZX = EZT;
                EZY = EZU;
            } else {
                let EZV = UF + ADI;
                let EZW = Lanes([UG[0], UG[1], 0.0]);
                EZX = EZV;
                EZY = EZW;
            }
            let EZZ = EZY * EZX;
            let FAA = ((EZX * EZX) + 8e-2f64).sqrt();
            let FAB = YE * (EZX - FAA);
            let FAC = (EZY - ((EZZ + EZZ) * (EA / (DZ * FAA)))) * YE;
            let FAD = (AA - ((CJN * FAB) / EZD)).sqrt();
            let FAE = (((FAC * CJN) / EZD) * DW) * (EA / (DZ * FAD));
            let FAQ;
            let FAR;
            if EUD != 0.0 {
                let FAG = VC * FAF;
                let FAJ = (FAF * VB) - (FAI * (FAB + (FAH * (FAD - AA))));
                let FAK = Lanes([FAG[0], 0.0, FAG[1]]) - ((FAC + (FAE * FAH)) * FAI);
                FAQ = FAJ;
                FAR = FAK;
            } else {
                let FAM = UG * FAL;
                let FAO = (FAL * UF) - (FAI * (FAB + (FAN * (FAD - AA))));
                let FAP = Lanes([FAM[0], FAM[1], 0.0]) - ((FAC + (FAE * FAN)) * FAI);
                FAQ = FAO;
                FAR = FAP;
            }
            let FAW;
            let FAX;
            let FAY;
            let FAZ;
            if DIR != 0.0 {
                let FAS = EZR * DGP;
                let FAT = EZS * DGP;
                let FAU = FAQ * DGP;
                let FAV = FAR * DGP;
                FAW = FAU;
                FAX = FAS;
                FAY = FAV;
                FAZ = FAT;
            } else {
                FAW = FAQ;
                FAX = EZR;
                FAY = FAR;
                FAZ = EZS;
            }
            let FBA = Lanes([0.0, FAY[0], FAY[1], FAY[2]]) + FAZ;
            let FBB = DRH + (FAW + FAX);
            let FBC = Lanes([DRL[0], DRL[1], DRL[2], DRL[3], DRL[4], DRL[5], 0.0]) + Lanes([0.0, 0.0, 0.0, FBA[0], FBA[1], FBA[2], FBA[3]]);
            let FBF = if DKO != 0.0 {
                let FBD = ((((DJQ + DJR) - DJS) + DJT) + DJU).abs();
                FBD
            } else {
                let FBE = ((((DJQ - DJR) - DJV) + DJT) + DJU).abs();
                FBE
            };
            if FBH != 0.0 {
            } else {
                if FBI != 0.0 {
                    let FBJ = BEA / BNM;
                    let FBK = FBJ * FBJ;
                    let FBL = parameters[216] * (AA + ((FBK * parameters[214]) * AHB));
                    let FBM = parameters[217] * (AA + ((FBK * parameters[215]) * AHB));
                    let FBN = if FBM > EQE { 1.0 } else { 0.0 };
                    let FBO = if FBN != 0.0 {
                        EQE
                    } else {
                        FBM
                    };
                    let FBP = if FBO > (EQE * FBL) { 1.0 } else { 0.0 };
                } else {
                }
            }
            if FBQ != 0.0 {
                if FBR != 0.0 {
                    let FBT = if ((FBF / staged[427]) * parameters[244]) < FJ { 1.0 } else { 0.0 };
                } else {
                    let FBU = if FBF < FJ { 1.0 } else { 0.0 };
                }
            } else {
                let FBX;
                if FBS != 0.0 {
                    FBX = S;
                } else {
                    let FBV = ((BPU / BQH) + parameters[282]) / BNL;
                    let FBW = if FBV < FJ { 1.0 } else { 0.0 };
                    let FCJ = if FBW != 0.0 {
                        let FCH = BQH * FCG;
                        FCH
                    } else {
                        let FCI = BQH * (FBV.ln());
                        FCI
                    };
                    FBX = FCJ;
                }
                let FBY = ((3.544146987039303e-61f64 * FBF) * W) * BNG;
                let FBZ = (((1e10f64 * BJL) * RS) * AHB) * AHB;
                let FCA = RS * BEA;
                let FCB = FCA / ABR;
                let FCC = (FCA * (AA - (BNT * BPS))) / ABR;
                let FCD = FCC + AXS;
                let FCE = (FCB + AXS) / FCD;
                let FCF = if FCE < FJ { 1.0 } else { 0.0 };
                let FCN = if FCF != 0.0 {
                    let FCL = FCK * FCG;
                    FCL
                } else {
                    let FCM = FCK * (FCE.ln());
                    FCM
                };
                let FCP = ((FBY / FBZ) * ((FCN + (FCO * (FCB - FCC))) + (staged[428] * ((FCB * FCB) - (FCC * FCC))))) + (((((((1.3806503e-23f64 * W) * FBF) * FBF) / staged[429]) * FBX) * ((FCK + (FCO * FCC)) + ((parameters[211] * FCC) * FCC))) / (FCD * FCD));
                let FCQ = (((staged[430] * W) / ((staged[431] * AXS) * AXS)) * FBF) * FBF;
                let FCR = if (if (if (FCQ + FCP) > S { 1.0 } else { 0.0 }) != 0.0 && (if FCP > S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if FCQ > S { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            }
            let FDE;
            let FDF;
            let FDG;
            let FDH;
            if FBG != 0.0 {
                let FCT = Lanes([FCS, 0.0]) - Lanes([0.0, TV]);
                let FCU = (node_potentials[0] - TT) / DIN;
                let FCV = DIP * FCU;
                let FCW = (Lanes([FCT[0], 0.0, 0.0, FCT[1], 0.0, 0.0]) - Lanes([0.0, FCV[0], FCV[1], FCV[2], FCV[3], FCV[4]])) / DIN;
                let FCY = Lanes([FCX, 0.0]) - Lanes([0.0, TW]);
                let FCZ = (node_potentials[2] - TU) / DIO;
                let FDA = DIQ * FCZ;
                let FDB = (Lanes([FCY[0], 0.0, 0.0, FCY[1], 0.0]) - Lanes([0.0, FDA[0], FDA[1], FDA[2], FDA[3]])) / DIO;
                FDE = FCU;
                FDF = FCZ;
                FDG = FCW;
                FDH = FDB;
            } else {
                FDE = S;
                FDF = S;
                FDG = FDC;
                FDH = FDD;
            }
            let FEQ;
            let FER;
            let FES;
            let FET;
            let FEU;
            let FEV;
            let FEW;
            let FEX;
            let FEY;
            let FEZ;
            let FFA;
            let FFB;
            let FFC;
            let FFD;
            let FFE;
            let FFF;
            let FFG;
            let FFH;
            let FFI;
            let FFJ;
            if DKO != 0.0 {
                let FDI = SY * (DJQ + DJR);
                let FDJ = (Lanes([DKC[0], DKC[1], DKC[2], DKC[3], DKC[4], DKC[5], 0.0, 0.0]) + Lanes([0.0, 0.0, DKD[0], DKD[1], DKD[2], 0.0, DKD[3], DKD[4]])) * SY;
                let FDK = SY * DJT;
                let FDL = DKF * SY;
                let FDM = SY * DJU;
                let FDN = DKG * SY;
                let FDO = SY * DJW;
                let FDP = DKI * SY;
                let FDQ = SY * DJX;
                let FDR = DKJ * SY;
                let FDS = SY * DJY;
                let FDT = DKK * SY;
                let FDU = SY * DRI;
                let FDV = DRM * SY;
                let FDW = SY * DRJ;
                let FDX = DRN * SY;
                let FDY = Lanes([0.0, FDN[0], FDN[1], FDN[2], FDN[3], FDN[4]]);
                FEQ = FDM;
                FER = FDO;
                FES = FDQ;
                FET = FDS;
                FEU = FDU;
                FEV = FDW;
                FEW = FDI;
                FEX = FDK;
                FEY = S;
                FEZ = S;
                FFA = FDY;
                FFB = FDP;
                FFC = FDR;
                FFD = FDT;
                FFE = FDV;
                FFF = FDX;
                FFG = FDJ;
                FFH = FDL;
                FFI = DAX;
                FFJ = DAX;
            } else {
                let FDZ = SY * (DJQ - DJR);
                let FEA = (Lanes([DKC[0], DKC[1], DKC[2], DKC[3], DKC[4], DKC[5], 0.0, 0.0]) - Lanes([0.0, 0.0, DKD[0], DKD[1], DKD[2], 0.0, DKD[3], DKD[4]])) * SY;
                let FEB = SY * DJT;
                let FEC = DKF * SY;
                let FED = SY * DJU;
                let FEE = DKG * SY;
                let FEF = SY * DJW;
                let FEG = DKI * SY;
                let FEH = SY * DJX;
                let FEI = DKJ * SY;
                let FEJ = SY * DJY;
                let FEK = DKK * SY;
                let FEL = SY * DRI;
                let FEM = DRM * SY;
                let FEN = SY * DRJ;
                let FEO = DRN * SY;
                let FEP = Lanes([0.0, FEE[0], FEE[1], FEE[2], FEE[3], FEE[4]]);
                FEQ = FEF;
                FER = FED;
                FES = FEJ;
                FET = FEH;
                FEU = FEN;
                FEV = FEL;
                FEW = S;
                FEX = S;
                FEY = FDZ;
                FEZ = FEB;
                FFA = FEG;
                FFB = FEP;
                FFC = FEK;
                FFD = FEI;
                FFE = FEO;
                FFF = FEM;
                FFG = DAX;
                FFH = DAX;
                FFI = FEA;
                FFJ = FEC;
            }
            let FFK = DKL * SY;
            let FFL = DKM * SY;
            let FFM = SY * DJS;
            let FFN = DKE * SY;
            let FFO = SY * DJV;
            let FFP = DKH * SY;
            let FFQ = (SY * DJZ) + FES;
            let FFR = Lanes([0.0, 0.0, 0.0, FFK[0], FFK[1], FFK[2]]) + FFC;
            let FFS = (SY * DKA) + FET;
            let FFT = Lanes([0.0, 0.0, 0.0, 0.0, FFL[0], FFL[1]]) + FFD;
            let FFX;
            let FFY;
            if FFU != 0.0 {
                FFX = S;
                FFY = DAY;
            } else {
                let FFV = SY * DBA;
                let FFW = DBC * SY;
                FFX = FFV;
                FFY = FFW;
            }
            let FFZ = ddt(45451, FEU);
            let FGB = FFE * FGA;
            let FGC = ddt(45453, FEV);
            let FGD = FFF * FGA;
            let FGE = SY * ddt(45456, FBB);
            let FGF = (FBC * FGA) * SY;
            let FGG = SY * FBB;
            let FGH = FBC * SY;
            let FGI = SY * ddt(45460, DRK);
            let FGJ = (DRO * FGA) * SY;
            let FGK = SY * DRK;
            let FGL = DRO * SY;
            let FGM = SY * ddt(45464, EQI);
            let FGN = (EQK * FGA) * SY;
            let FGO = SY * EQI;
            let FGP = EQK * SY;
            let FGQ = SY * ddt(45468, EQJ);
            let FGR = (EQL * FGA) * SY;
            let FGS = SY * EQJ;
            let FGT = EQL * SY;
            let FHX;
            let FHY;
            let FHZ;
            let FIA;
            let FIB;
            let FIC;
            let FID;
            let FIE;
            let FIF;
            let FIG;
            let FIH;
            let FII;
            let FIJ;
            let FIK;
            let FIL;
            let FIM;
            let FIN;
            let FIO;
            let FIP;
            let FIQ;
            let FIR;
            let FIS;
            let FIT;
            let FIU;
            if EUD != 0.0 {
                let FGU = SY * ddt(45475, FAX);
                let FGV = (FAZ * FGA) * SY;
                let FGW = SY * FAX;
                let FGX = FAZ * SY;
                let FGY = SY * ddt(45479, FAW);
                let FGZ = (FAY * FGA) * SY;
                let FHA = SY * FAW;
                let FHB = FAY * SY;
                let FHD = (UZ - UH) * FHC;
                let FHE = (Lanes([0.0, VA]) - Lanes([UI, 0.0])) * FHC;
                let FHF = ddt(45484, FHD);
                let FHG = FHE * FGA;
                FHX = FGU;
                FHY = FGY;
                FHZ = FHF;
                FIA = S;
                FIB = S;
                FIC = S;
                FID = FGW;
                FIE = FHA;
                FIF = FHD;
                FIG = S;
                FIH = S;
                FII = S;
                FIJ = FGV;
                FIK = FGZ;
                FIL = FHG;
                FIM = FHH;
                FIN = FHI;
                FIO = FHJ;
                FIP = FGX;
                FIQ = FHB;
                FIR = FHE;
                FIS = FHH;
                FIT = FHI;
                FIU = FHJ;
            } else {
                let FHK = SY * ddt(45487, FAX);
                let FHL = (FAZ * FGA) * SY;
                let FHM = SY * FAX;
                let FHN = FAZ * SY;
                let FHO = SY * ddt(45491, FAW);
                let FHP = (FAY * FGA) * SY;
                let FHQ = SY * FAW;
                let FHR = FAY * SY;
                let FHS = (UD - UH) * FHC;
                let FHT = (Lanes([0.0, UE]) - Lanes([UI, 0.0])) * FHC;
                let FHU = ddt(45496, FHS);
                let FHV = FHT * FGA;
                FHX = S;
                FHY = S;
                FHZ = S;
                FIA = FHK;
                FIB = FHO;
                FIC = FHU;
                FID = S;
                FIE = S;
                FIF = S;
                FIG = FHM;
                FIH = FHQ;
                FII = FHS;
                FIJ = FHH;
                FIK = FHI;
                FIL = FHW;
                FIM = FHL;
                FIN = FHP;
                FIO = FHV;
                FIP = FHH;
                FIQ = FHI;
                FIR = FHW;
                FIS = FHN;
                FIT = FHR;
                FIU = FHT;
            }
            let FIV = ddt(45498, EUB);
            let FIW = EUC * FGA;
            let FIX = ddt(45500, ETY);
            let FIY = ETZ * FGA;
            let FJE;
            let FJF;
            if FIZ != 0.0 {
                FJE = S;
                FJF = FJA;
            } else {
                let FJC = (node_potentials[1] - UZ) * DGV;
                let FJD = (Lanes([FJB, 0.0]) - Lanes([0.0, VA])) * DGV;
                FJE = FJC;
                FJF = FJD;
            }
            let FJN;
            let FJO;
            if FJG != 0.0 {
                FJN = S;
                FJO = FJH;
            } else {
                let FJI = UZ - UD;
                let FJJ = FJI * DGN;
                let FJK = (Lanes([0.0, VA]) - Lanes([UE, 0.0])) * DGN;
                let FJL = DGO * FJI;
                let FJM = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, FJK[0], FJK[1]]) + Lanes([FJL[0], FJL[1], FJL[2], FJL[3], FJL[4], FJL[5], 0.0]);
                FJN = FJJ;
                FJO = FJM;
            }
            let FJX;
            let FJY;
            let FJZ;
            let FKA;
            if M != 0.0 {
                let FJQ = (TZ - UV) * FJP;
                let FJR = (Lanes([UA, 0.0]) - Lanes([0.0, UW])) * FJP;
                let FJT = (TZ - UR) * FJS;
                let FJU = (Lanes([UA, 0.0]) - Lanes([0.0, US])) * FJS;
                FJX = FJQ;
                FJY = FJT;
                FJZ = FJR;
                FKA = FJU;
            } else {
                FJX = S;
                FJY = S;
                FJZ = FJV;
                FKA = FJW;
            }
            let FKL;
            let FKM;
            let FKN;
            let FKO;
            let FKP;
            let FKQ;
            if P != 0.0 {
                let FKB = -DJQ;
                let FKC = XO * FKB;
                let FKE = (FKB * WR) + (U / FKD);
                let FKF = (((DKC * DW) * WR) + Lanes([0.0, 0.0, 0.0, FKC[0], FKC[1], 0.0])) + Lanes([0.0, 0.0, (V / FKD), 0.0, 0.0, 0.0]);
                let FKH = U * FKG;
                let FKI = V * FKG;
                let FKJ = ddt(45590, FKH);
                let FKK = FKI * FGA;
                FKL = FKE;
                FKM = FKJ;
                FKN = FKH;
                FKO = FKF;
                FKP = FKK;
                FKQ = FKI;
            } else {
                FKL = S;
                FKM = S;
                FKN = S;
                FKO = ATP;
                FKP = T;
                FKQ = T;
            }
            let FKR = FGH[4];
            let FKS = FGH[3];
            let FKT = FFE[5];
            let FKU = FFE[3];
            let FKV = FFE[4];
            let FKW = FGP[2];
            let FKX = FGT[2];
            let FKY = FDG[0];
            let FKZ = FDG[1];
            let FLA = FDG[2];
            let FLB = FDG[3];
            let FLC = FDG[4];
            let FLD = FDG[5];
            let FLE = FDH[0];
            let FLF = FDH[1];
            let FLG = FDH[2];
            let FLH = FDH[3];
            let FLI = FDH[4];
            let FLJ = FFG[0];
            let FLK = FFG[1];
            let FLL = FFG[2];
            let FLM = FFG[3];
            let FLN = FFG[4];
            let FLO = FFG[5];
            let FLP = FFG[6];
            let FLQ = FFG[7];
            let FLR = FFH[0];
            let FLS = FFH[1];
            let FLT = FFH[2];
            let FLU = FFH[3];
            let FLV = FFH[4];
            let FLW = FFH[5];
            let FLX = FFH[6];
            let FLY = FFH[7];
            let FLZ = FFI[0];
            let FMA = FFI[1];
            let FMB = FFI[2];
            let FMC = FFI[3];
            let FMD = FFI[4];
            let FME = FFI[5];
            let FMF = FFI[6];
            let FMG = FFI[7];
            let FMH = FFJ[0];
            let FMI = FFJ[1];
            let FMJ = FFJ[2];
            let FMK = FFJ[3];
            let FML = FFJ[4];
            let FMM = FFJ[5];
            let FMN = FFJ[6];
            let FMO = FFJ[7];
            let FMP = FFA[0];
            let FMQ = FFA[1];
            let FMR = FFA[2];
            let FMS = FFA[3];
            let FMT = FFA[4];
            let FMU = FFA[5];
            let FMV = FFB[0];
            let FMW = FFB[1];
            let FMX = FFB[2];
            let FMY = FFB[3];
            let FMZ = FFB[4];
            let FNA = FFB[5];
            let FNB = FFN[0];
            let FNC = FFN[1];
            let FND = FFN[2];
            let FNE = FFP[0];
            let FNF = FFP[1];
            let FNG = FFP[2];
            let FNH = FFR[0];
            let FNI = FFR[1];
            let FNJ = FFR[2];
            let FNK = FFR[3];
            let FNL = FFR[4];
            let FNM = FFR[5];
            let FNN = FFT[0];
            let FNO = FFT[1];
            let FNP = FFT[2];
            let FNQ = FFT[3];
            let FNR = FFT[4];
            let FNS = FFT[5];
            let FNT = DKN[0];
            let FNU = DKN[1];
            let FNV = DKN[2];
            let FNW = DKN[3];
            let FNX = DKN[4];
            let FNY = DKN[5];
            let FNZ = CZR[0];
            let FOA = CZR[1];
            let FOB = CZR[2];
            let FOC = FFY[0];
            let FOD = FFY[1];
            let FOE = FGB[0];
            let FOF = FGB[1];
            let FOG = FGB[2];
            let FOH = FGB[3];
            let FOI = FGB[4];
            let FOJ = FGB[5];
            let FOK = FGD[0];
            let FOL = FGD[1];
            let FOM = FGD[2];
            let FON = FGD[3];
            let FOO = FGD[4];
            let FOP = FGD[5];
            let FOQ = FGF[0];
            let FOR = FGF[1];
            let FOS = FGF[2];
            let FOT = FGF[3];
            let FOU = FGF[4];
            let FOV = FGF[5];
            let FOW = FGF[6];
            let FOX = FGJ[0];
            let FOY = FGJ[1];
            let FOZ = FGJ[2];
            let FPA = FGJ[3];
            let FPB = FGJ[4];
            let FPC = FGJ[5];
            let FPD = FGN[0];
            let FPE = FGN[1];
            let FPF = FGN[2];
            let FPG = FGR[0];
            let FPH = FGR[1];
            let FPI = FGR[2];
            let FPJ = FIJ[0];
            let FPK = FIJ[1];
            let FPL = FIJ[2];
            let FPM = FIJ[3];
            let FPN = FIK[0];
            let FPO = FIK[1];
            let FPP = FIK[2];
            let FPQ = FIL[0];
            let FPR = FIL[1];
            let FPS = FIM[0];
            let FPT = FIM[1];
            let FPU = FIM[2];
            let FPV = FIM[3];
            let FPW = FIN[0];
            let FPX = FIN[1];
            let FPY = FIN[2];
            let FPZ = FIO[0];
            let FQA = FIO[1];
            let FQB = FIW[0];
            let FQC = FIW[1];
            let FQD = FIW[2];
            let FQE = FIY[0];
            let FQF = FIY[1];
            let FQG = FJF[0];
            let FQH = FJF[1];
            let FQI = FJO[0];
            let FQJ = FJO[1];
            let FQK = FJO[2];
            let FQL = FJO[3];
            let FQM = FJO[4];
            let FQN = FJO[5];
            let FQO = FJO[6];
            let FQP = FJZ[0];
            let FQQ = FJZ[1];
            let FQR = FKA[0];
            let FQS = FKA[1];
            let FQT = FKO[0];
            let FQU = FKO[1];
            let FQV = FKO[2];
            let FQW = FKO[3];
            let FQX = FKO[4];
            let FQY = FKO[5];
            let FQZ = FKP;
            let FRA = FFE[0];
            let FRB = FFE[1];
            let FRC = FFE[2];
            let FRD = FFF[0];
            let FRE = FFF[1];
            let FRF = FFF[2];
            let FRG = FFF[3];
            let FRH = FFF[4];
            let FRI = FFF[5];
            let FRJ = FGH[0];
            let FRK = FGH[1];
            let FRL = FGH[2];
            let FRM = FGH[5];
            let FRN = FGH[6];
            let FRO = FGL[0];
            let FRP = FGL[1];
            let FRQ = FGL[2];
            let FRR = FGL[3];
            let FRS = FGL[4];
            let FRT = FGL[5];
            let FRU = FGP[0];
            let FRV = FGP[1];
            let FRW = FGT[0];
            let FRX = FGT[1];
            let FRY = FIP[0];
            let FRZ = FIP[1];
            let FSA = FIP[2];
            let FSB = FIP[3];
            let FSC = FIQ[0];
            let FSD = FIQ[1];
            let FSE = FIQ[2];
            let FSF = FIR[0];
            let FSG = FIR[1];
            let FSH = FIS[0];
            let FSI = FIS[1];
            let FSJ = FIS[2];
            let FSK = FIS[3];
            let FSL = FIT[0];
            let FSM = FIT[1];
            let FSN = FIT[2];
            let FSO = FIU[0];
            let FSP = FIU[1];
            let FSQ = EUC[0];
            let FSR = EUC[1];
            let FSS = EUC[2];
            let FST = ETZ[0];
            let FSU = ETZ[1];
            let FSV = FKQ;
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[862]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[863]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (staged[864]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (FSW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(0),
            Some(7),
            multiplicity * (FDE),
            [0, 5, 6, 7, 8, 9],
            [FKY, FKZ, FLA, FLB, FLC, FLD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(7),
            multiplicity * (staged[865]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(8),
            multiplicity * (FDF),
            [2, 5, 6, 8, 9],
            [FLE, FLF, FLG, FLH, FLI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(8),
            multiplicity * (staged[866]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(7), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            staged[867],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(2), Some(8), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            staged[868],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(8),
            multiplicity * (FEW),
            [3, 5, 6, 7, 8, 9, 11, 12],
            [FLJ, FLK, FLL, FLM, FLN, FLO, FLP, FLQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(5),
            multiplicity * (FEX),
            [3, 5, 6, 7, 8, 9, 11, 12],
            [FLR, FLS, FLT, FLU, FLV, FLW, FLX, FLY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(7),
            multiplicity * (FEY),
            [3, 5, 6, 7, 8, 9, 11, 12],
            [FLZ, FMA, FMB, FMC, FMD, FME, FMF, FMG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(5),
            multiplicity * (FEZ),
            [3, 5, 6, 7, 8, 9, 11, 12],
            [FMH, FMI, FMJ, FMK, FML, FMM, FMN, FMO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (FEQ),
            [3, 5, 6, 7, 8, 9],
            [FMP, FMQ, FMR, FMS, FMT, FMU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (FER),
            [3, 5, 6, 7, 8, 9],
            [FMV, FMW, FMX, FMY, FMZ, FNA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(7),
            multiplicity * (FFM),
            [6, 7, 12],
            [FNB, FNC, FND],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(8),
            multiplicity * (FFO),
            [6, 8, 11],
            [FNE, FNF, FNG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (FFQ),
            [3, 5, 6, 7, 8, 9],
            [FNH, FNI, FNJ, FNK, FNL, FNM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(8),
            multiplicity * (FFS),
            [3, 5, 6, 7, 8, 9],
            [FNN, FNO, FNP, FNQ, FNR, FNS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(5),
            multiplicity * (DKB),
            [3, 5, 6, 7, 8, 9],
            [FNT, FNU, FNV, FNW, FNX, FNY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(4),
            multiplicity * (CZQ),
            [4, 6, 9],
            [FNZ, FOA, FOB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(4), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            staged[869],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (FFX),
            [4, 5],
            [FOC, FOD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (staged[870]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            Some(7),
            multiplicity * (FSX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (FSY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (FSZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(8),
            multiplicity * (FTA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(5),
            multiplicity * (FTB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (FFZ),
            [3, 5, 6, 7, 8, 9],
            [FOE, FOF, FOG, FOH, FOI, FOJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (FGC),
            [3, 5, 6, 7, 8, 9],
            [FOK, FOL, FOM, FON, FOO, FOP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(9),
            Some(5),
            multiplicity * (FGE),
            [3, 5, 6, 7, 8, 9, 10],
            [FOQ, FOR, FOS, FOT, FOU, FOV, FOW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(5),
            multiplicity * (FGI),
            [3, 5, 6, 7, 8, 9],
            [FOX, FOY, FOZ, FPA, FPB, FPC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(12),
            Some(7),
            multiplicity * (FGM),
            [6, 7, 12],
            [FPD, FPE, FPF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(8),
            multiplicity * (FGQ),
            [6, 8, 11],
            [FPG, FPH, FPI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(7),
            multiplicity * (FHX),
            [7, 8, 9, 10],
            [FPJ, FPK, FPL, FPM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(8),
            multiplicity * (FHY),
            [8, 9, 10],
            [FPN, FPO, FPP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(10),
            Some(3),
            multiplicity * (FHZ),
            [3, 10],
            [FPQ, FPR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(7),
            multiplicity * (FIA),
            [7, 8, 9, 10],
            [FPS, FPT, FPU, FPV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(8),
            multiplicity * (FIB),
            [8, 9, 10],
            [FPW, FPX, FPY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(3),
            multiplicity * (FIC),
            [3, 9],
            [FPZ, FQA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(3),
            multiplicity * (FIV),
            [3, 7, 8],
            [FQB, FQC, FQD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(8),
            Some(3),
            multiplicity * (FIX),
            [3, 8],
            [FQE, FQF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(10), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            staged[871],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(10),
            multiplicity * (FJE),
            [1, 10],
            [FQG, FQH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(10),
            multiplicity * (staged[872]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(9), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            staged[873],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(9),
            multiplicity * (FJN),
            [3, 5, 6, 7, 8, 9, 10],
            [FQI, FQJ, FQK, FQL, FQM, FQN, FQO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(9),
            multiplicity * (staged[874]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(12),
            multiplicity * (FJX),
            [5, 12],
            [FQP, FQQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(11),
            multiplicity * (FJY),
            [5, 11],
            [FQR, FQS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(12),
            multiplicity * (staged[875]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(11),
            multiplicity * (staged[876]),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(5), Some(12), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            staged[877],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(11), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            staged[878],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), Some(8), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            staged[879],
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            None,
            multiplicity * (FKL),
            [3, 5, 6, 7, 8, 9],
            [FQT, FQU, FQV, FQW, FQX, FQY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            None,
            multiplicity * (FKM),
            [6],
            [FQZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(6), None, 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            8,
            staged[880],
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = staged[862];
        self.canonical_reactive[1] = staged[863];
        self.canonical_reactive[2] = staged[864];
        self.canonical_reactive[3] = FSW;
        self.canonical_reactive[4] = FDE;
        self.canonical_reactive[5] = staged[865];
        self.canonical_reactive[6] = FDF;
        self.canonical_reactive[7] = staged[866];
        self.canonical_reactive[8] = staged[867];
        self.canonical_reactive[9] = staged[868];
        self.canonical_reactive[10] = FEW;
        self.canonical_reactive[11] = FEX;
        self.canonical_reactive[12] = FEY;
        self.canonical_reactive[13] = FEZ;
        self.canonical_reactive[14] = FEQ;
        self.canonical_reactive[15] = FER;
        self.canonical_reactive[16] = FFM;
        self.canonical_reactive[17] = FFO;
        self.canonical_reactive[18] = FFQ;
        self.canonical_reactive[19] = FFS;
        self.canonical_reactive[20] = DKB;
        self.canonical_reactive[21] = CZQ;
        self.canonical_reactive[22] = staged[869];
        self.canonical_reactive[23] = FFX;
        self.canonical_reactive[24] = staged[870];
        self.canonical_reactive[25] = FSX;
        self.canonical_reactive[26] = FSY;
        self.canonical_reactive[27] = FSZ;
        self.canonical_reactive[28] = FTA;
        self.canonical_reactive[29] = FTB;
        self.canonical_reactive[30] = FEU;
        self.canonical_reactive[31] = FRA;
        self.canonical_reactive[32] = FRB;
        self.canonical_reactive[33] = FRC;
        self.canonical_reactive[34] = FKU;
        self.canonical_reactive[35] = FKV;
        self.canonical_reactive[36] = FKT;
        self.canonical_reactive[37] = FEV;
        self.canonical_reactive[38] = FRD;
        self.canonical_reactive[39] = FRE;
        self.canonical_reactive[40] = FRF;
        self.canonical_reactive[41] = FRG;
        self.canonical_reactive[42] = FRH;
        self.canonical_reactive[43] = FRI;
        self.canonical_reactive[44] = FGG;
        self.canonical_reactive[45] = FRJ;
        self.canonical_reactive[46] = FRK;
        self.canonical_reactive[47] = FRL;
        self.canonical_reactive[48] = FKS;
        self.canonical_reactive[49] = FKR;
        self.canonical_reactive[50] = FRM;
        self.canonical_reactive[51] = FRN;
        self.canonical_reactive[52] = FGK;
        self.canonical_reactive[53] = FRO;
        self.canonical_reactive[54] = FRP;
        self.canonical_reactive[55] = FRQ;
        self.canonical_reactive[56] = FRR;
        self.canonical_reactive[57] = FRS;
        self.canonical_reactive[58] = FRT;
        self.canonical_reactive[59] = FGO;
        self.canonical_reactive[60] = FRU;
        self.canonical_reactive[61] = FRV;
        self.canonical_reactive[62] = FKW;
        self.canonical_reactive[63] = FGS;
        self.canonical_reactive[64] = FRW;
        self.canonical_reactive[65] = FRX;
        self.canonical_reactive[66] = FKX;
        self.canonical_reactive[67] = FID;
        self.canonical_reactive[68] = FRY;
        self.canonical_reactive[69] = FRZ;
        self.canonical_reactive[70] = FSA;
        self.canonical_reactive[71] = FSB;
        self.canonical_reactive[72] = FIE;
        self.canonical_reactive[73] = FSC;
        self.canonical_reactive[74] = FSD;
        self.canonical_reactive[75] = FSE;
        self.canonical_reactive[76] = FIF;
        self.canonical_reactive[77] = FSF;
        self.canonical_reactive[78] = FSG;
        self.canonical_reactive[79] = FIG;
        self.canonical_reactive[80] = FSH;
        self.canonical_reactive[81] = FSI;
        self.canonical_reactive[82] = FSJ;
        self.canonical_reactive[83] = FSK;
        self.canonical_reactive[84] = FIH;
        self.canonical_reactive[85] = FSL;
        self.canonical_reactive[86] = FSM;
        self.canonical_reactive[87] = FSN;
        self.canonical_reactive[88] = FII;
        self.canonical_reactive[89] = FSO;
        self.canonical_reactive[90] = FSP;
        self.canonical_reactive[91] = EUB;
        self.canonical_reactive[92] = FSQ;
        self.canonical_reactive[93] = FSR;
        self.canonical_reactive[94] = FSS;
        self.canonical_reactive[95] = ETY;
        self.canonical_reactive[96] = FST;
        self.canonical_reactive[97] = FSU;
        self.canonical_reactive[98] = staged[871];
        self.canonical_reactive[99] = FJE;
        self.canonical_reactive[100] = staged[872];
        self.canonical_reactive[101] = staged[873];
        self.canonical_reactive[102] = FJN;
        self.canonical_reactive[103] = staged[874];
        self.canonical_reactive[104] = FJX;
        self.canonical_reactive[105] = FJY;
        self.canonical_reactive[106] = staged[875];
        self.canonical_reactive[107] = staged[876];
        self.canonical_reactive[108] = staged[877];
        self.canonical_reactive[109] = staged[878];
        self.canonical_reactive[110] = staged[879];
        self.canonical_reactive[111] = FKL;
        self.canonical_reactive[112] = FKN;
        self.canonical_reactive[113] = FSV;
        self.canonical_reactive[114] = staged[880];
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 5, 6, 7, 8, 9],
            &[cached[31], cached[32], cached[33], cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[3, 5, 6, 7, 8, 9],
            &[cached[38], cached[39], cached[40], cached[41], cached[42], cached[43]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(5),
            &[3, 5, 6, 7, 8, 9, 10],
            &[cached[45], cached[46], cached[47], cached[48], cached[49], cached[50], cached[51]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(5),
            &[3, 5, 6, 7, 8, 9],
            &[cached[53], cached[54], cached[55], cached[56], cached[57], cached[58]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            Some(7),
            &[6, 7, 12],
            &[cached[60], cached[61], cached[62]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(8),
            &[6, 8, 11],
            &[cached[64], cached[65], cached[66]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(7),
            &[7, 8, 9, 10],
            &[cached[68], cached[69], cached[70], cached[71]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(8),
            &[8, 9, 10],
            &[cached[73], cached[74], cached[75]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            Some(3),
            &[3, 10],
            &[cached[77], cached[78]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(7),
            &[7, 8, 9, 10],
            &[cached[80], cached[81], cached[82], cached[83]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(8),
            &[8, 9, 10],
            &[cached[85], cached[86], cached[87]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            Some(3),
            &[3, 9],
            &[cached[89], cached[90]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(3),
            &[3, 7, 8],
            &[cached[92], cached[93], cached[94]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(3),
            &[3, 8],
            &[cached[96], cached[97]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            None,
            &[6],
            &[cached[113]],
            &[],
            &[],
            multiplicity,
        );
    }

}
