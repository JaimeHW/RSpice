#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::rspice_limited_exp;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 11] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 23, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FG_GE_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "fg", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ge", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("Id"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("Igs"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("Igd"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("Igd"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("Igs"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGB", label: Some("Igb"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGB", label: Some("Igb"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8])];
            let A = 0e0f64;
            let B = 1e0f64;
            let C = 1.0f64;
            let D = parameters[18];
            let E = parameters[310];
            let G = parameters[12];
            let I = -1e0f64;
            let J = parameters[13];
            let L = -1e0f64;
            let M = parameters[59];
            let P = parameters[1];
            let Q = parameters[2];
            let AC = 2e0f64;
            let AF = 1e-9f64;
            let AR = 1e-6f64;
            let GD = parameters[317];
            let GG = 3.9e0f64;
            let GH = parameters[45];
            let GJ = parameters[47];
            let GL = parameters[46];
            let GN = parameters[49];
            let GR = parameters[60];
            let GT = parameters[138];
            let HE = parameters[188];
            let HO = parameters[14];
            let HU = 1e6f64;
            let IH = 3e-2f64;
            let IP = parameters[190];
            let IT = parameters[194];
            let IX = parameters[198];
            let JK = 1e-38f64;
            let JL = 5e-1f64;
            let JP = 3e0f64;
            let JR = 3.333333333333333e-1f64;
            let JT = 1e-8f64;
            let KA = parameters[296];
            let KF = 1e-3f64;
            let KU = parameters[297];
            let KW = 3.0015e2f64;
            let KX = 2.7315e2f64;
            let KZ = 4.97232e-7f64;
            let LA = 3.42537e-7f64;
            let LB = 7.45669e11f64;
            let LC = 1.16645e12f64;
            let LD = parameters[99];
            let LF = parameters[239];
            let LJ = parameters[315];
            let LN = 1e3f64;
            let LO = parameters[19];
            let LQ = temperature;
            let LR = parameters[9];
            let LY = 2.5e-1f64;
            let LZ = 1e-2f64;
            let MF = parameters[55];
            let MN = parameters[52];
            let MP = 4e0f64;
            let MQ = 1e-4f64;
            let NI = 9e-1f64;
            let PB = node_potentials[8];
            let PC = node_potentials[6];
            let PE = node_potentials[5];
            let PH = node_potentials[3];
            let PM = -1e0f64;
            let PP = 2e-2f64;
            let QA = 4e1f64;
            let QI = parameters[83];
            let RL = 1.60219e-19f64;
            let SN = parameters[10];
            let ST = 3.947841e1f64;
            let TQ = 6.534e-2f64;
            let TS = 8.57973e0f64;
            let TU = 7.895683e1f64;
            let TV = 5e1f64;
            let UP = 1e1f64;
            let UZ = 1.05e0f64;
            let ABY = parameters[154];
            let ACA = parameters[11];
            let ALR = parameters[162];
            let ALV = parameters[189];
            let AMT = parameters[109];
            let AMW = parameters[134];
            let ANK = parameters[213];
            let APU = 8e-2f64;
            let AQX = parameters[234];
            let ARD = parameters[235];
            let ARL = parameters[288];
            let ARM = parameters[289];
            let ARN = parameters[290];
            let ARS = parameters[287];
            let ARZ = parameters[292];
            let ASB = 1e10f64;
            let AUB = parameters[286];
            if C != 0.0 {
                let F = if (if D == A { 1.0 } else { 0.0 }) != 0.0 || (if E == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if F != 0.0 {
                } else {
                }
            } else {
            }
            let H = if G == B { 1.0 } else { 0.0 };
            let MZ = if H != 0.0 {
                B
            } else {
                I
            };
            let K = if J == B { 1.0 } else { 0.0 };
            let RO = if K != 0.0 {
                B
            } else {
                L
            };
            let N = M * 8.85418e-12f64;
            let O = if parameters[21] == A { 1.0 } else { 0.0 };
            let T = if O != 0.0 {
                let R = P / Q;
                R
            } else {
                P
            };
            let S = parameters[0] + parameters[23];
            let U = T + parameters[24];
            let V = S.powf((-parameters[29]));
            let W = U.powf((-parameters[30]));
            let X = V * W;
            let Y = S.powf((-parameters[35]));
            let Z = U.powf((-parameters[36]));
            let AA = Y * Z;
            let AB = ((parameters[31] + (parameters[32] * Y)) + (parameters[33] * Z)) + (parameters[34] * AA);
            let AD = S - (AC * (((parameters[25] + (parameters[26] * V)) + (parameters[27] * W)) + (parameters[28] * X)));
            let AE = if AD <= A { 1.0 } else { 0.0 };
            if AE != 0.0 {
            } else {
                let AG = if AD <= AF { 1.0 } else { 0.0 };
                if AG != 0.0 {
                } else {
                }
            }
            let AH = U - (AC * AB);
            let AI = if AH <= A { 1.0 } else { 0.0 };
            if AI != 0.0 {
            } else {
                let AJ = if AH <= AF { 1.0 } else { 0.0 };
                if AJ != 0.0 {
                } else {
                }
            }
            let AK = ((parameters[41] + (parameters[42] * Y)) + (parameters[43] * Z)) + (parameters[44] * AA);
            let AL = S - (AC * (((parameters[37] + (parameters[38] * V)) + (parameters[39] * W)) + (parameters[40] * X)));
            let AM = if AL <= A { 1.0 } else { 0.0 };
            if AM != 0.0 {
            } else {
                let AN = if AL <= AF { 1.0 } else { 0.0 };
                if AN != 0.0 {
                } else {
                }
            }
            let AO = U - (AC * AK);
            let AP = if AO <= A { 1.0 } else { 0.0 };
            if AP != 0.0 {
            } else {
                let AQ = if AO <= AF { 1.0 } else { 0.0 };
                if AQ != 0.0 {
                } else {
                }
            }
            let AS = AR / AD;
            let AT = AR / AH;
            let AU = AS * AT;
            let AV = ((parameters[191] + (parameters[319] * AS)) + (parameters[320] * AT)) + (parameters[321] * AU);
            let AW = ((parameters[199] + (parameters[325] * AS)) + (parameters[326] * AT)) + (parameters[327] * AU);
            let AX = ((parameters[195] + (parameters[322] * AS)) + (parameters[323] * AT)) + (parameters[324] * AU);
            let AY = ((parameters[202] + (parameters[328] * AS)) + (parameters[329] * AT)) + (parameters[330] * AU);
            let AZ = ((parameters[203] + (parameters[331] * AS)) + (parameters[332] * AT)) + (parameters[333] * AU);
            let BA = ((parameters[204] + (parameters[334] * AS)) + (parameters[335] * AT)) + (parameters[336] * AU);
            let BB = ((parameters[57] + (parameters[337] * AS)) + (parameters[338] * AT)) + (parameters[339] * AU);
            let BC = ((parameters[58] + (parameters[340] * AS)) + (parameters[341] * AT)) + (parameters[342] * AU);
            let BD = ((parameters[51] + (parameters[343] * AS)) + (parameters[344] * AT)) + (parameters[345] * AU);
            let BE = ((parameters[50] + (parameters[346] * AS)) + (parameters[347] * AT)) + (parameters[348] * AU);
            let BF = ((parameters[63] + (parameters[349] * AS)) + (parameters[350] * AT)) + (parameters[351] * AU);
            let BG = ((parameters[64] + (parameters[352] * AS)) + (parameters[353] * AT)) + (parameters[354] * AU);
            let BH = ((parameters[65] + (parameters[355] * AS)) + (parameters[356] * AT)) + (parameters[357] * AU);
            let BI = ((parameters[68] + (parameters[358] * AS)) + (parameters[359] * AT)) + (parameters[360] * AU);
            let BJ = ((parameters[276] + (parameters[361] * AS)) + (parameters[362] * AT)) + (parameters[363] * AU);
            let BK = ((parameters[291] + (parameters[751] * AS)) + (parameters[752] * AT)) + (parameters[753] * AU);
            let BL = ((parameters[294] + (parameters[757] * AS)) + (parameters[758] * AT)) + (parameters[759] * AU);
            let BM = ((parameters[293] + (parameters[754] * AS)) + (parameters[755] * AT)) + (parameters[756] * AU);
            let BN = if BJ < A { 1.0 } else { 0.0 };
            let QX;
            if BN != 0.0 {
                QX = A;
            } else {
                let BO = if BJ > B { 1.0 } else { 0.0 };
                let QY = if BO != 0.0 {
                    B
                } else {
                    BJ
                };
                QX = QY;
            }
            let BP = ((parameters[277] + (parameters[364] * AS)) + (parameters[365] * AT)) + (parameters[366] * AU);
            let BQ = ((parameters[278] + (parameters[367] * AS)) + (parameters[368] * AT)) + (parameters[369] * AU);
            let BR = ((parameters[275] + (parameters[370] * AS)) + (parameters[371] * AT)) + (parameters[372] * AU);
            let BS = ((parameters[272] + (parameters[373] * AS)) + (parameters[374] * AT)) + (parameters[375] * AU);
            let BT = ((parameters[273] + (parameters[376] * AS)) + (parameters[377] * AT)) + (parameters[378] * AU);
            let BU = ((parameters[274] + (parameters[379] * AS)) + (parameters[380] * AT)) + (parameters[381] * AU);
            let BV = ((parameters[283] + (parameters[382] * AS)) + (parameters[383] * AT)) + (parameters[384] * AU);
            let BW = if BV < A { 1.0 } else { 0.0 };
            let RF;
            if BW != 0.0 {
                RF = A;
            } else {
                let BX = if BV > B { 1.0 } else { 0.0 };
                let RG = if BX != 0.0 {
                    B
                } else {
                    BV
                };
                RF = RG;
            }
            let BY = ((parameters[284] + (parameters[385] * AS)) + (parameters[386] * AT)) + (parameters[387] * AU);
            let BZ = ((parameters[285] + (parameters[388] * AS)) + (parameters[389] * AT)) + (parameters[390] * AU);
            let CA = ((parameters[282] + (parameters[391] * AS)) + (parameters[392] * AT)) + (parameters[393] * AU);
            let CB = ((parameters[279] + (parameters[394] * AS)) + (parameters[395] * AT)) + (parameters[396] * AU);
            let CC = ((parameters[280] + (parameters[397] * AS)) + (parameters[398] * AT)) + (parameters[399] * AU);
            let CD = ((parameters[281] + (parameters[400] * AS)) + (parameters[401] * AT)) + (parameters[402] * AU);
            let CE = ((parameters[71] + (parameters[403] * AS)) + (parameters[404] * AT)) + (parameters[405] * AU);
            let CF = ((parameters[72] + (parameters[406] * AS)) + (parameters[407] * AT)) + (parameters[408] * AU);
            let CG = ((parameters[73] + (parameters[409] * AS)) + (parameters[410] * AT)) + (parameters[411] * AU);
            let CH = ((parameters[74] + (parameters[412] * AS)) + (parameters[413] * AT)) + (parameters[414] * AU);
            let CI = ((parameters[75] + (parameters[415] * AS)) + (parameters[416] * AT)) + (parameters[417] * AU);
            let CJ = ((parameters[84] + (parameters[418] * AS)) + (parameters[419] * AT)) + (parameters[420] * AU);
            let CK = ((parameters[76] + (parameters[421] * AS)) + (parameters[422] * AT)) + (parameters[423] * AU);
            let CL = ((parameters[87] + (parameters[430] * AS)) + (parameters[431] * AT)) + (parameters[432] * AU);
            let CM = ((parameters[88] + (parameters[433] * AS)) + (parameters[434] * AT)) + (parameters[435] * AU);
            let CN = ((parameters[61] + (parameters[436] * AS)) + (parameters[437] * AT)) + (parameters[438] * AU);
            let CO = ((parameters[62] + (parameters[439] * AS)) + (parameters[440] * AT)) + (parameters[441] * AU);
            let CP = ((parameters[85] + (parameters[424] * AS)) + (parameters[425] * AT)) + (parameters[426] * AU);
            let CQ = ((parameters[86] + (parameters[427] * AS)) + (parameters[428] * AT)) + (parameters[429] * AU);
            let CR = ((parameters[113] + (parameters[460] * AS)) + (parameters[461] * AT)) + (parameters[462] * AU);
            let CS = ((parameters[89] + (parameters[442] * AS)) + (parameters[443] * AT)) + (parameters[444] * AU);
            let CT = ((parameters[90] + (parameters[445] * AS)) + (parameters[446] * AT)) + (parameters[447] * AU);
            let CU = ((parameters[91] + (parameters[448] * AS)) + (parameters[449] * AT)) + (parameters[450] * AU);
            let CV = ((parameters[92] + (parameters[451] * AS)) + (parameters[452] * AT)) + (parameters[453] * AU);
            let CW = ((parameters[93] + (parameters[454] * AS)) + (parameters[455] * AT)) + (parameters[456] * AU);
            let CX = ((parameters[94] + (parameters[457] * AS)) + (parameters[458] * AT)) + (parameters[459] * AU);
            let CY = ((parameters[116] + (parameters[463] * AS)) + (parameters[464] * AT)) + (parameters[465] * AU);
            let CZ = ((parameters[123] + (parameters[466] * AS)) + (parameters[467] * AT)) + (parameters[468] * AU);
            let DA = ((parameters[124] + (parameters[469] * AS)) + (parameters[470] * AT)) + (parameters[471] * AU);
            let DB = ((parameters[122] + (parameters[472] * AS)) + (parameters[473] * AT)) + (parameters[474] * AU);
            let DC = ((parameters[135] + (parameters[475] * AS)) + (parameters[476] * AT)) + (parameters[477] * AU);
            let DD = ((parameters[139] + (parameters[478] * AS)) + (parameters[479] * AT)) + (parameters[480] * AU);
            let DE = ((parameters[145] + (parameters[481] * AS)) + (parameters[482] * AT)) + (parameters[483] * AU);
            let DF = ((parameters[148] + (parameters[484] * AS)) + (parameters[485] * AT)) + (parameters[486] * AU);
            let DG = ((parameters[155] + (parameters[487] * AS)) + (parameters[488] * AT)) + (parameters[489] * AU);
            let DH = ((parameters[142] + (parameters[490] * AS)) + (parameters[491] * AT)) + (parameters[492] * AU);
            let DI = ((parameters[163] + (parameters[493] * AS)) + (parameters[494] * AT)) + (parameters[495] * AU);
            let DJ = ((parameters[157] + (parameters[496] * AS)) + (parameters[497] * AT)) + (parameters[498] * AU);
            let DK = ((parameters[156] + (parameters[499] * AS)) + (parameters[500] * AT)) + (parameters[501] * AU);
            let DL = ((parameters[158] + (parameters[502] * AS)) + (parameters[503] * AT)) + (parameters[504] * AU);
            let DM = ((parameters[160] + (parameters[505] * AS)) + (parameters[506] * AT)) + (parameters[507] * AU);
            let DN = ((parameters[161] + (parameters[508] * AS)) + (parameters[509] * AT)) + (parameters[510] * AU);
            let DO = ((parameters[136] + (parameters[511] * AS)) + (parameters[512] * AT)) + (parameters[513] * AU);
            let DP = ((parameters[166] + (parameters[514] * AS)) + (parameters[515] * AT)) + (parameters[516] * AU);
            let DQ = ((parameters[167] + (parameters[517] * AS)) + (parameters[518] * AT)) + (parameters[519] * AU);
            let DR = ((parameters[173] + (parameters[520] * AS)) + (parameters[521] * AT)) + (parameters[522] * AU);
            let DS = ((parameters[176] + (parameters[523] * AS)) + (parameters[524] * AT)) + (parameters[525] * AU);
            let DT = ((parameters[182] + (parameters[526] * AS)) + (parameters[527] * AT)) + (parameters[528] * AU);
            let DU = ((parameters[170] + (parameters[529] * AS)) + (parameters[530] * AT)) + (parameters[531] * AU);
            let DV = ((parameters[183] + (parameters[532] * AS)) + (parameters[533] * AT)) + (parameters[534] * AU);
            let DW = ((parameters[186] + (parameters[535] * AS)) + (parameters[536] * AT)) + (parameters[537] * AU);
            let DX = ((parameters[119] + (parameters[538] * AS)) + (parameters[539] * AT)) + (parameters[540] * AU);
            let DY = ((parameters[130] + (parameters[541] * AS)) + (parameters[542] * AT)) + (parameters[543] * AU);
            let DZ = ((parameters[205] + (parameters[544] * AS)) + (parameters[545] * AT)) + (parameters[546] * AU);
            let EA = ((parameters[305] + (parameters[547] * AS)) + (parameters[548] * AT)) + (parameters[549] * AU);
            let EB = ((parameters[306] + (parameters[550] * AS)) + (parameters[551] * AT)) + (parameters[552] * AU);
            let EC = ((parameters[307] + (parameters[553] * AS)) + (parameters[554] * AT)) + (parameters[555] * AU);
            let ED = ((parameters[308] + (parameters[556] * AS)) + (parameters[557] * AT)) + (parameters[558] * AU);
            let EE = ((parameters[210] + (parameters[559] * AS)) + (parameters[560] * AT)) + (parameters[561] * AU);
            let EF = ((parameters[214] + (parameters[562] * AS)) + (parameters[563] * AT)) + (parameters[564] * AU);
            let EG = ((parameters[208] + (parameters[565] * AS)) + (parameters[566] * AT)) + (parameters[567] * AU);
            let EH = ((parameters[206] + (parameters[568] * AS)) + (parameters[569] * AT)) + (parameters[570] * AU);
            let EI = ((parameters[207] + (parameters[571] * AS)) + (parameters[572] * AT)) + (parameters[573] * AU);
            let EJ = ((parameters[209] + (parameters[574] * AS)) + (parameters[575] * AT)) + (parameters[576] * AU);
            let EK = ((parameters[256] + (parameters[577] * AS)) + (parameters[578] * AT)) + (parameters[579] * AU);
            let EL = ((parameters[257] + (parameters[580] * AS)) + (parameters[581] * AT)) + (parameters[582] * AU);
            let EM = ((parameters[258] + (parameters[583] * AS)) + (parameters[584] * AT)) + (parameters[585] * AU);
            let EN = ((parameters[217] + (AS * parameters[706])) + (AT * parameters[707])) + (AU * parameters[708]);
            let EO = ((parameters[218] + (AS * parameters[709])) + (AT * parameters[710])) + (AU * parameters[711]);
            let EP = ((parameters[219] + (AS * parameters[712])) + (AT * parameters[713])) + (AU * parameters[714]);
            let EQ = ((parameters[220] + (AS * parameters[715])) + (AT * parameters[716])) + (AU * parameters[717]);
            let ER = ((parameters[221] + (AS * parameters[718])) + (AT * parameters[719])) + (AU * parameters[720]);
            let ES = ((parameters[222] + (AS * parameters[721])) + (AT * parameters[722])) + (AU * parameters[723]);
            let ET = ((parameters[223] + (AS * parameters[724])) + (AT * parameters[725])) + (AU * parameters[726]);
            let EU = ((parameters[224] + (AS * parameters[727])) + (AT * parameters[728])) + (AU * parameters[729]);
            let EV = ((parameters[225] + (AS * parameters[730])) + (AT * parameters[731])) + (AU * parameters[732]);
            let EW = ((parameters[226] + (parameters[586] * AS)) + (parameters[587] * AT)) + (parameters[588] * AU);
            let EX = ((parameters[227] + (parameters[589] * AS)) + (parameters[590] * AT)) + (parameters[591] * AU);
            let EY = ((parameters[228] + (parameters[592] * AS)) + (parameters[593] * AT)) + (parameters[594] * AU);
            let EZ = ((parameters[230] + (parameters[595] * AS)) + (parameters[596] * AT)) + (parameters[597] * AU);
            let FA = ((parameters[229] + (parameters[598] * AS)) + (parameters[599] * AT)) + (parameters[600] * AU);
            let FB = ((parameters[250] + (parameters[619] * AS)) + (parameters[620] * AT)) + (parameters[621] * AU);
            let FC = ((parameters[251] + (parameters[622] * AS)) + (parameters[623] * AT)) + (parameters[624] * AU);
            let FD = ((parameters[244] + (parameters[601] * AS)) + (parameters[602] * AT)) + (parameters[603] * AU);
            let FE = ((parameters[245] + (parameters[604] * AS)) + (parameters[605] * AT)) + (parameters[606] * AU);
            let FF = ((parameters[231] + (parameters[637] * AS)) + (parameters[638] * AT)) + (parameters[639] * AU);
            let FG = ((parameters[232] + (parameters[643] * AS)) + (parameters[644] * AT)) + (parameters[645] * AU);
            let FH = ((parameters[233] + (parameters[649] * AS)) + (parameters[650] * AT)) + (parameters[651] * AU);
            let FI = ((parameters[242] + (parameters[655] * AS)) + (parameters[656] * AT)) + (parameters[657] * AU);
            let FJ = ((parameters[236] + (parameters[640] * AS)) + (parameters[641] * AT)) + (parameters[642] * AU);
            let FK = ((parameters[237] + (parameters[646] * AS)) + (parameters[647] * AT)) + (parameters[648] * AU);
            let FL = ((parameters[238] + (parameters[652] * AS)) + (parameters[653] * AT)) + (parameters[654] * AU);
            let FM = ((parameters[243] + (parameters[658] * AS)) + (parameters[659] * AT)) + (parameters[660] * AU);
            let FN = ((parameters[240] + (parameters[661] * AS)) + (parameters[662] * AT)) + (parameters[663] * AU);
            let FO = ((parameters[241] + (parameters[664] * AS)) + (parameters[665] * AT)) + (parameters[666] * AU);
            let FP = ((parameters[100] + (parameters[679] * AS)) + (parameters[680] * AT)) + (parameters[681] * AU);
            let FQ = ((parameters[129] + (parameters[682] * AS)) + (parameters[683] * AT)) + (parameters[684] * AU);
            let FR = ((parameters[103] + (parameters[685] * AS)) + (parameters[686] * AT)) + (parameters[687] * AU);
            let FS = ((parameters[106] + (parameters[688] * AS)) + (parameters[689] * AT)) + (parameters[690] * AU);
            let FT = ((parameters[110] + (parameters[691] * AS)) + (parameters[692] * AT)) + (parameters[693] * AU);
            let FU = ((parameters[111] + (parameters[694] * AS)) + (parameters[695] * AT)) + (parameters[696] * AU);
            let FV = ((parameters[112] + (parameters[697] * AS)) + (parameters[698] * AT)) + (parameters[699] * AU);
            let FW = ((parameters[137] + (parameters[700] * AS)) + (parameters[701] * AT)) + (parameters[702] * AU);
            let FX = ((parameters[187] + (parameters[703] * AS)) + (parameters[704] * AT)) + (parameters[705] * AU);
            let FY = ((parameters[95] + (parameters[739] * AS)) + (parameters[740] * AT)) + (parameters[741] * AU);
            let FZ = ((parameters[96] + (parameters[742] * AS)) + (parameters[743] * AT)) + (parameters[744] * AU);
            let GA = ((parameters[97] + (parameters[745] * AS)) + (parameters[746] * AT)) + (parameters[747] * AU);
            let GB = ((parameters[98] + (parameters[748] * AS)) + (parameters[749] * AT)) + (parameters[750] * AU);
            let GC = if parameters[20] == B { 1.0 } else { 0.0 };
            let GE = if GC != 0.0 && (if GD != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ATB = if GE != 0.0 {
                let GF = ((GD + (parameters[733] * AS)) + (parameters[734] * AT)) + (parameters[735] * AU);
                GF
            } else {
                A
            };
            let GI = 3.4531302e-11f64 / GH;
            let GK = 3.4531302e-11f64 / GJ;
            let GM = 3.4531302e-11f64 / GL;
            let GO = N / GN;
            let GP = M / GG;
            let GQ = if (if parameter_given[47] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            let APE = if GQ != 0.0 {
                let GS = ((GH * GR) / GG) - parameters[48];
                GS
            } else {
                GJ
            };
            let GU = if GT > A { 1.0 } else { 0.0 };
            let IF = if GU != 0.0 {
                let GV = DC * (B - (FW * (AD.powf((-GT)))));
                GV
            } else {
                let GW = DC * (B - FW);
                GW
            };
            let GX = -AD;
            let GY = DD + (parameters[140] * (rspice_limited_exp((GX / parameters[141]))));
            let GZ = DE + (parameters[146] * (rspice_limited_exp((GX / parameters[147]))));
            let HA = parameters[151] + (parameters[152] * (rspice_limited_exp((GX / parameters[153]))));
            let HB = DF + (parameters[149] * (rspice_limited_exp((GX / parameters[150]))));
            let HC = DH + (parameters[143] * (rspice_limited_exp((GX / parameters[144]))));
            let HD = DI + (parameters[164] * (rspice_limited_exp((GX / parameters[165]))));
            let HF = if HE > A { 1.0 } else { 0.0 };
            let ACD = if HF != 0.0 {
                let HG = DP * (B - (FX * (AD.powf((-HE)))));
                HG
            } else {
                let HH = DP * (B - FX);
                HH
            };
            let HI = DQ + (parameters[168] * (rspice_limited_exp((GX / parameters[169]))));
            let HJ = DR + (parameters[174] * (rspice_limited_exp((GX / parameters[175]))));
            let HK = parameters[179] + (parameters[180] * (rspice_limited_exp((GX / parameters[181]))));
            let HL = DS + (parameters[177] * (rspice_limited_exp((GX / parameters[178]))));
            let HM = DU + (parameters[171] * (rspice_limited_exp((GX / parameters[172]))));
            let HN = DV + (parameters[184] * (rspice_limited_exp((GX / parameters[185]))));
            let HP = if HO == B { 1.0 } else { 0.0 };
            let IR;
            let IV;
            let IZ;
            if HP != 0.0 {
                let HQ = AX + (parameters[196] * (rspice_limited_exp((GX / parameters[197]))));
                let HR = AW + (parameters[200] * (rspice_limited_exp((GX / parameters[201]))));
                IR = AV;
                IV = HQ;
                IZ = HR;
            } else {
                let HS = AV + (parameters[192] * (rspice_limited_exp((GX / parameters[193]))));
                IR = HS;
                IV = AX;
                IZ = AW;
            }
            let HT = EE + (parameters[211] * (rspice_limited_exp((GX / parameters[212]))));
            let HV = CR + (parameters[114] * ((AD * HU).powf((-parameters[115]))));
            let HW = CY + (parameters[117] * (rspice_limited_exp((GX / parameters[118]))));
            let HX = CZ + (parameters[125] * (rspice_limited_exp((GX / parameters[126]))));
            let HY = DA + (parameters[127] * (rspice_limited_exp((GX / parameters[128]))));
            let HZ = FP + (parameters[101] * (rspice_limited_exp((GX / parameters[102]))));
            let IA = FQ + (parameters[132] * (rspice_limited_exp((GX / parameters[133]))));
            let IB = FR + (parameters[104] * (rspice_limited_exp((GX / parameters[105]))));
            let IC = FS + (parameters[107] * (rspice_limited_exp((GX / parameters[108]))));
            let ID = parameters[77] + (parameters[79] * (rspice_limited_exp((GX / parameters[80]))));
            let IE = parameters[78] + (parameters[81] * (rspice_limited_exp((GX / parameters[82]))));
            let IG = if IF < A { 1.0 } else { 0.0 };
            let NH = if IG != 0.0 {
                IH
            } else {
                IF
            };
            let II = if GY < A { 1.0 } else { 0.0 };
            let NN = if II != 0.0 {
                A
            } else {
                GY
            };
            let IJ = if HC < A { 1.0 } else { 0.0 };
            let ABV = if IJ != 0.0 {
                A
            } else {
                HC
            };
            let IK = if HB < A { 1.0 } else { 0.0 };
            let NQ = if IK != 0.0 {
                A
            } else {
                HB
            };
            let IL = if DG < A { 1.0 } else { 0.0 };
            let NS = if IL != 0.0 {
                A
            } else {
                DG
            };
            let IM = if IA < A { 1.0 } else { 0.0 };
            let OG = if IM != 0.0 {
                A
            } else {
                IA
            };
            let IN = if CF <= A { 1.0 } else { 0.0 };
            if IN != 0.0 {
            } else {
            }
            let IO = if CK <= A { 1.0 } else { 0.0 };
            if IO != 0.0 {
            } else {
            }
            let IQ = if IP < A { 1.0 } else { 0.0 };
            let KQ = if IQ != 0.0 {
                A
            } else {
                IP
            };
            let IS = if IR < A { 1.0 } else { 0.0 };
            let KS = if IS != 0.0 {
                A
            } else {
                IR
            };
            let IU = if IT < A { 1.0 } else { 0.0 };
            let KI = if IU != 0.0 {
                A
            } else {
                IT
            };
            let IW = if IV < A { 1.0 } else { 0.0 };
            let KM = if IW != 0.0 {
                A
            } else {
                IV
            };
            let IY = if IX < A { 1.0 } else { 0.0 };
            let KK = if IY != 0.0 {
                A
            } else {
                IX
            };
            let JA = if IZ < A { 1.0 } else { 0.0 };
            let KO = if JA != 0.0 {
                A
            } else {
                IZ
            };
            let JB = if AY < A { 1.0 } else { 0.0 };
            let ACJ = if JB != 0.0 {
                A
            } else {
                AY
            };
            let JC = if EH < A { 1.0 } else { 0.0 };
            if JC != 0.0 {
            } else {
            }
            let JD = if EI < A { 1.0 } else { 0.0 };
            if JD != 0.0 {
            } else {
            }
            let JE = if EG <= A { 1.0 } else { 0.0 };
            if JE != 0.0 {
            } else {
            }
            let JF = if HV < AC { 1.0 } else { 0.0 };
            let JI = if JF != 0.0 {
                AC
            } else {
                HV
            };
            let JG = ((B + (CQ / AD)).sqrt()) - B;
            let JH = GN + (GP * (GH + GL));
            let JJ = B / JI;
            let JM = JL * DO;
            let JN = JL * DW;
            let JO = if G != B { 1.0 } else { 0.0 };
            let ABP;
            let ABR;
            let AMN;
            if JO != 0.0 {
                let JQ = 3.333333333333333e-1f64 * DO;
                let JS = 3.333333333333333e-1f64 * DW;
                ABP = JQ;
                ABR = JS;
                AMN = JR;
            } else {
                ABP = JM;
                ABR = JN;
                AMN = JL;
            }
            let JU = GP * GH;
            let JV = JT / JU;
            let JW = B / (((AH * HU).powf(BA)) * Q);
            let JX = (JU * GN).sqrt();
            let JY = GP * GL;
            let JZ = JT / JY;
            let KB = if KA >= (AD / AC) { 1.0 } else { 0.0 };
            let ARP = if KB != 0.0 {
                A
            } else {
                KA
            };
            let KC = if (if D != A { 1.0 } else { 0.0 }) != 0.0 && (if E > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if KC != 0.0 {
            } else {
            }
            let KD = parameters[215] * parameters[7];
            let KE = parameters[216] * parameters[8];
            let KG = if KD <= KF { 1.0 } else { 0.0 };
            let ACR = if KG != 0.0 {
                KF
            } else {
                KD
            };
            let KH = if KE <= KF { 1.0 } else { 0.0 };
            let ACS = if KH != 0.0 {
                KF
            } else {
                KE
            };
            let ACL;
            let ACN;
            let AOC;
            let AOE;
            let AOJ;
            let AOL;
            if HP != 0.0 {
                let KJ = if KI <= A { 1.0 } else { 0.0 };
                let AOD = if KJ != 0.0 {
                    A
                } else {
                    KI
                };
                let KL = if KK <= A { 1.0 } else { 0.0 };
                let AOK = if KL != 0.0 {
                    A
                } else {
                    KK
                };
                let KN = if KM <= A { 1.0 } else { 0.0 };
                let AOF = if KN != 0.0 {
                    A
                } else {
                    KM
                };
                let KP = if KO <= A { 1.0 } else { 0.0 };
                let AOM = if KP != 0.0 {
                    A
                } else {
                    KO
                };
                ACL = KQ;
                ACN = KS;
                AOC = AOD;
                AOE = AOF;
                AOJ = AOK;
                AOL = AOM;
            } else {
                let KR = if KQ <= A { 1.0 } else { 0.0 };
                let ACM = if KR != 0.0 {
                    A
                } else {
                    KQ
                };
                let KT = if KS <= A { 1.0 } else { 0.0 };
                let ACO = if KT != 0.0 {
                    A
                } else {
                    KS
                };
                ACL = ACM;
                ACN = ACO;
                AOC = KI;
                AOE = KM;
                AOJ = KK;
                AOL = KO;
            }
            let KV = if KU <= A { 1.0 } else { 0.0 };
            let MB = if KV != 0.0 {
                KW
            } else {
                let KY = KU + KX;
                KY
            };
            let LH = if H != 0.0 {
                KZ
            } else {
                LA
            };
            let AQI = if H != 0.0 {
                LB
            } else {
                LC
            };
            let LE = LD * FO;
            let LG = (rspice_limited_exp((FN * ((if (LF / LD) >= JK { (LF / LD) } else { JK }).ln())))) / (LD * LD);
            let LI = (AH * LH) * ((rspice_limited_exp((FN * ((if (LF / LE) >= JK { (LF / LE) } else { JK }).ln())))) / (LE * LE));
            let LK = (parameters[316] * (parameters[313] + ((AH / JP) / LJ))) / ((LJ * Q) * (S - parameters[314]));
            let LL = if LK > KF { 1.0 } else { 0.0 };
            let ATX;
            if LL != 0.0 {
                let LM = B / LK;
                ATX = LM;
            } else {
                let LP = if LO != A { 1.0 } else { 0.0 };
                if LP != 0.0 {
                } else {
                }
                ATX = LN;
            }
            let LV = if KC != 0.0 {
                let LS = (LQ + node_potentials[4]) + LR;
                LS
            } else {
                let LT = LQ + LR;
                LT
            };
            let LU = parameters[298] + KX;
            let LW = if LV > LU { 1.0 } else { 0.0 };
            if LW != 0.0 {
            } else {
            }
            let LX = LV - LU;
            let MA = JL * ((LV + LU) - (((LX * LX) + 2.5e-5f64).sqrt()));
            let MC = MA / MB;
            let MD = MA - MB;
            let ME = 8.61708e-5f64 * MA;
            let MG = MF - (((parameters[299] * MA) * MA) / (MA + parameters[300]));
            let MH = MA / KW;
            let MI = AC * ME;
            let MJ = (parameters[54] * (MH * (MH.sqrt()))) * (rspice_limited_exp(((MF / 5.1728331239999994e-2f64) - (MG / MI))));
            let MK = ME * ((if ((BD * BE) / (MJ * MJ)) >= JK { ((BD * BE) / (MJ * MJ)) } else { JK }).ln());
            let ML = ME * ((if (BE / MJ) >= JK { (BE / MJ) } else { JK }).ln());
            let MM = JL * MG;
            let MO = MM - (ME * ((if (MN / MJ) >= JK { (MN / MJ) } else { JK }).ln()));
            let MR = MM - (JL * (MO + (((MO * MO) + 4e-8f64).sqrt())));
            let MS = if MN != A { 1.0 } else { 0.0 };
            let MT = if MS != 0.0 && (if (if parameter_given[58] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let NB;
            if MT != 0.0 {
                let MU = if J == -1e0f64 { 1.0 } else { 0.0 };
                let NC = if MU != 0.0 {
                    let MV = (BC - (JL * MF)) + MR;
                    MV
                } else {
                    let MW = (BC + (JL * MF)) - MR;
                    MW
                };
                NB = NC;
            } else {
                NB = BC;
            }
            let MX = MG / AC;
            let MY = parameters[53] + MX;
            let NA = MZ * (BB - MY);
            let ND = MZ * (NB - MY);
            let NE = MY - (MZ * (if MX <= (ME * ((if (BD / MJ) >= JK { (BD / MJ) } else { JK }).ln())) { MX } else { (ME * ((if (BD / MJ) >= JK { (BD / MJ) } else { JK }).ln())) }));
            let NF = MZ * (BB - NE);
            let NG = MZ * (NB - NE);
            let NJ = NI + (DJ * MD);
            let NK = (NH * (MC.powf(DK))) * ((B + (JL * (NJ + (((NJ * NJ) + 4e-6f64).sqrt())))) - 9.000011111097395e-1f64);
            let NL = (B + (parameters[159] * MD)) - AR;
            let NM = GZ * (JL * (NL + (((NL * NL) + 4e-6f64).sqrt())));
            let NO = (B + (DL * MD)) - AR;
            let NP = NN * (JL * (NO + (((NO * NO) + 4e-6f64).sqrt())));
            let NR = NQ * (MC.powf(DM));
            let NT = NS * (MC.powf(DN));
            let NU = (B + (DZ * MD)) - AR;
            let NV = JL * (NU + (((NU * NU) + 4e-6f64).sqrt()));
            let NW = NI - ((DX * (B + (AS * parameters[120]))) * MD);
            let NX = NW * NW;
            let NY = HZ * ((B + (JL * (NW + ((NX + 4e-6f64).sqrt())))) - 9.000011111097395e-1f64);
            let NZ = if NY < LN { 1.0 } else { 0.0 };
            let ACU = if NZ != 0.0 {
                LN
            } else {
                NY
            };
            let OA = IB * ((B + (JL * (NW + ((NX + 4e-6f64).sqrt())))) - 9.000011111097395e-1f64);
            let OB = if OA < LN { 1.0 } else { 0.0 };
            let AMQ = if OB != 0.0 {
                LN
            } else {
                OA
            };
            let OC = IC * ((B + (JL * (NW + ((NX + 4e-6f64).sqrt())))) - 9.000011111097395e-1f64);
            let OD = if OC < LN { 1.0 } else { 0.0 };
            let AMY = if OD != 0.0 {
                LN
            } else {
                OC
            };
            let OE = parameters[309] * MD;
            let OF = CH * (B + (-9e-1f64 + (JL * (((OE - -9e-1f64) - MQ) + (((((OE - -9e-1f64) - MQ) * ((OE - -9e-1f64) - MQ)) - -3.6e-4f64).sqrt())))));
            let OH = NI - ((DY * (B + (AS * parameters[131]))) * MD);
            let OI = OG * ((B + (JL * (OH + (((OH * OH) + 4e-6f64).sqrt())))) - 9.000011111097395e-1f64);
            let OJ = (JI * (B + (parameters[121] * MD))) - AC;
            let OK = (JL * (OJ + (((OJ * OJ) + 4e-6f64).sqrt()))) + AC;
            let OL = CS + (CT * MD);
            let OM = -CU;
            let ON = ((CV * MD) - OM) - AR;
            let OO = CU + (OM + (JL * (ON + (((ON * ON) - ((MP * OM) * AR)).sqrt()))));
            let OP = CW + (CX * MD);
            let OQ = (B - (DB * MD)) - AR;
            let OR = HW * (JL * (OQ + (((OQ * OQ) + 4e-6f64).sqrt())));
            let OS = MC - B;
            let OT = (parameters[301] + (parameters[302] / AD)) * OS;
            let OU = EM * (MC.powf(EA));
            let OV = (B + (EB * MD)) - AR;
            let OW = FE * (JL * (OV + (((OV * OV) + 4e-6f64).sqrt())));
            let OX = (B + (EC * MD)) - AR;
            let OY = FC * (JL * (OX + (((OX * OX) + 4e-6f64).sqrt())));
            let OZ = rspice_limited_exp((ED * ((if MC >= JK { MC } else { JK }).ln())));
            let PA = LI * OZ;
            let PD = MZ * (PB - PC);
            let PF = MZ * (PE - PC);
            let PG = MZ * (PB - PE);
            let PI = MZ * (PH - PC);
            let PJ = MZ * (PH - PE);
            let PK = MZ * (PB - PH);
            let PL = if PF < A { 1.0 } else { 0.0 };
            let PO;
            let PS;
            let PU;
            let AOO;
            let AQV;
            if PL != 0.0 {
                let PN = -PF;
                PO = PN;
                PS = PJ;
                PU = PG;
                AOO = PI;
                AQV = PM;
            } else {
                PO = PF;
                PS = PI;
                PU = PD;
                AOO = PJ;
                AQV = B;
            }
            let PQ = (((PO * PO) + 4e-4f64).sqrt()) - PP;
            let PR = JL * (PQ - PO);
            let PT = PS + PR;
            let PV = PU - NA;
            let PW = PS - ND;
            let PX = (GN * (JU + (3.75e-1f64 * GN))).sqrt();
            let PY = PX + (((((CN + (CO * ((((PV * JY) + (PW * (JU + GN))) / JH) + PR))).atan()) / 3.141592653589793e0f64) + JL) * ((((GP * GN) * GH).sqrt()) - PX));
            let PZ = ((CF * AD) / PY) + AR;
            let QB = if PZ < QA { 1.0 } else { 0.0 };
            let SE = if QB != 0.0 {
                let QC = JL / ((PZ.cosh()) - B);
                QC
            } else {
                let QD = rspice_limited_exp((-PZ));
                QD
            };
            let QE = ((CK * AD) / PY) + AR;
            let QF = if QE < QA { 1.0 } else { 0.0 };
            let SF = if QF != 0.0 {
                let QG = JL / ((QE.cosh()) - B);
                QG
            } else {
                let QH = rspice_limited_exp((-QE));
                QH
            };
            let SH = if QF != 0.0 {
                let QJ = B / (if (B + (QI * ((QE.cosh()) - AC))) >= AR { (B + (QI * ((QE.cosh()) - AC))) } else { AR });
                QJ
            } else {
                let QK = rspice_limited_exp((-QE));
                let QL = QK / (if (QK + QI) >= AR { (QK + QI) } else { AR });
                QL
            };
            let QM = ((EG * AD) / PY) + AR;
            let QN = if QM < QA { 1.0 } else { 0.0 };
            let ANF = if QN != 0.0 {
                let QO = ((JL * EH) / ((QM.cosh()) - B)) + EI;
                QO
            } else {
                let QP = (EH * (rspice_limited_exp((-QM)))) + EI;
                QP
            };
            let QQ = if J == -1e0f64 { 1.0 } else { 0.0 };
            let RH;
            let RI;
            let RP;
            let RT;
            let RY;
            if QQ != 0.0 {
                let QR = (BR * AD) / PY;
                let QS = if QR > QA { 1.0 } else { 0.0 };
                let QV = if QS != 0.0 {
                    let QT = (rspice_limited_exp(QR)) / AC;
                    QT
                } else {
                    let QU = (QR.cosh()) - B;
                    QU
                };
                let QW = BS - ((JL * BT) / QV);
                RH = QW;
                RI = BU;
                RP = BP;
                RT = BQ;
                RY = QX;
            } else {
                let QZ = (CA * AD) / PY;
                let RA = if QZ > QA { 1.0 } else { 0.0 };
                let RD = if RA != 0.0 {
                    let RB = (rspice_limited_exp(QZ)) / AC;
                    RB
                } else {
                    let RC = (QZ.cosh()) - B;
                    RC
                };
                let RE = CB - ((JL * CC) / RD);
                RH = RE;
                RI = CD;
                RP = BY;
                RT = BZ;
                RY = RF;
            }
            let RJ = RH - RI;
            let RK = RI + (JL * (RJ + (((RJ * RJ) + MQ).sqrt())));
            let RM = AC * GM;
            let RN = ((RL * MN) * N) / (RM * GM);
            let RS = if MS != 0.0 {
                let RQ = RO * ((MZ * PT) - RP);
                let RR = ((B + ((JL * (RQ + (((RQ * RQ) + 4e-4f64).sqrt()))) / RN)).sqrt()) - B;
                RR
            } else {
                A
            };
            let RU = -RT;
            let RV = ((-((RN * RS) * RS)) - RU) - LZ;
            let RW = GM + GO;
            let RX = ((-GM) * GO) / (RW * GI);
            let RZ = (RX * RK) * ((PW - (((MZ * RO) * RY) * (-(RU + (JL * (RV + (((RV * RV) - ((MP * RU) * LZ)).sqrt()))))))) - (-1.2e0f64 - PR));
            let SA = JL * (PT + (((PT * PT) + 4e-6f64).sqrt()));
            let SB = (4e-1f64 + ML) + CG;
            let SC = if SB < A { 1.0 } else { 0.0 };
            let SL = if SC != 0.0 {
                A
            } else {
                let SD = (CP * JG) * (SB.sqrt());
                SD
            };
            let SG = PQ + LZ;
            let SI = GI + ((GO * GM) / RW);
            let SJ = (ME * ((SI + BF) + (((parameters[66] * PT) + ((parameters[67] * PT) * PT)) + (SE * (((BG + (BI * PT)) + ((parameters[69] * PT) * PT)) + ((BH + (parameters[70] * SA)) * PQ)))))) / SI;
            let SK = ((RL * BE) * GN) / GI;
            let SM = ((((((((-CE) * SE) * (MK - SB)) + ((((-(OF + (CJ * PT))) * SF) * (PQ + (CI * (SG.sqrt())))) + ((ID * SH) * (SG.powf(IE))))) + SL) + (((-CL) / (AD + CM)) * PQ)) + (SK * (B - ((JL * GN) / (GN + JY))))) + (OT + (((parameters[303] + (parameters[304] / AD)) * PT) * OS))) + RZ;
            let SO = (PV - SM) + SN;
            let SP = (((3.20438e-19f64 * MJ) * GN) * GN) / (N * ME);
            let SQ = GI / GO;
            let SR = GM / GO;
            let SS = SP.ln();
            let SU = 3.675753940198048e0f64 - SS;
            let SV = SQ * SQ;
            let SW = SQ / (((SR * SQ) + SR) + SQ);
            let SX = SO / SJ;
            let SY = PW - SM;
            let SZ = SY + SN;
            let TA = SZ / SJ;
            let TB = SX - SU;
            let TC = ((((SV * TB) * TB) + ST).ln()) - SS;
            let TD = B + SR;
            let TE = (TC + (SR * TA)) / TD;
            let TF = if (if (TA + (SW * (SX - TA))) <= TC { (TA + (SW * (SX - TA))) } else { TC }) <= SU { (if (TA + (SW * (SX - TA))) <= TC { (TA + (SW * (SX - TA))) } else { TC }) } else { SU };
            let TG = B + SQ;
            let TH = (TF + (SQ * SX)) / TG;
            let TI = TA - TE;
            let TJ = SR * SR;
            let TK = ((TJ * TI) * TI) - (SP * (TE.exp()));
            let TL = if TK < A { 1.0 } else { 0.0 };
            let UG = if TL != 0.0 {
                let TM = (TA - TF) * SR;
                let TN = QA * SQ;
                let TO = TN + TM;
                let TP = TN * TM;
                let TR = (TQ * TO) + B;
                let TT = ((TO * TS) + TP) + ST;
                let TW = if ((((-TT) + ((((-4e0f64 * TR) * ((TU * TO) + (ST * TP))) + (TT * TT)).sqrt())) / (AC * TR)) * (B - (((-((SX - (((SU * TG) - TF) / SQ)) + AC)) / 2.8985507246376816e0f64).exp()))) <= TV { ((((-TT) + ((((-4e0f64 * TR) * ((TU * TO) + (ST * TP))) + (TT * TT)).sqrt())) / (AC * TR)) * (B - (((-((SX - (((SU * TG) - TF) / SQ)) + AC)) / 2.8985507246376816e0f64).exp()))) } else { TV };
                TW
            } else {
                TK
            };
            let TX = if SX >= SU { SX } else { SU };
            let TY = TX - SU;
            let TZ = SU * TG;
            let UA = ((TZ - TF) / SQ) - SU;
            let UB = (((((SV * TY) * TY) + ST).ln()) - SS) - ((((((SV * UA) * UA) + ST).ln()) - SS) - SU);
            let UC = TX - UB;
            let UD = -SP;
            let UE = UD * (UB.exp());
            let UF = SV * UC;
            let UH = UB + ((-(((UF * UC) + UE) - UG)) / ((-2e0f64 * UF) + UE));
            let UI = TX - UH;
            let UJ = SV * UI;
            let UK = (UJ * UI) - UG;
            let UL = B / UK;
            let UM = B / (((-2e0f64 * UJ) * UL) - B);
            let UN = AC * SV;
            let UO = ((((UK.abs()).ln()) - SS) - UH) * UM;
            let UQ = UH + (if (if ((-UO) - ((((JL * UO) * UO) * (((((-4e0f64 * UJ) * UJ) * UL) * UL) + (UN * UL))) * UM)) >= -1e1f64 { ((-UO) - ((((JL * UO) * UO) * (((((-4e0f64 * UJ) * UJ) * UL) * UL) + (UN * UL))) * UM)) } else { -1e1f64 }) <= UP { (if ((-UO) - ((((JL * UO) * UO) * (((((-4e0f64 * UJ) * UJ) * UL) * UL) + (UN * UL))) * UM)) >= -1e1f64 { ((-UO) - ((((JL * UO) * UO) * (((((-4e0f64 * UJ) * UJ) * UL) * UL) + (UN * UL))) * UM)) } else { -1e1f64 }) } else { UP });
            let UR = TX - UQ;
            let US = SV * UR;
            let UT = (US * UR) - UG;
            let UU = B / UT;
            let UV = B / (((-2e0f64 * US) * UU) - B);
            let UW = ((((UT.abs()).ln()) - SS) - UQ) * UV;
            let UX = SU - MP;
            let UY = if (UQ + (if (if ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) >= -1e1f64 { ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) } else { -1e1f64 }) <= UP { (if ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) >= -1e1f64 { ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) } else { -1e1f64 }) } else { UP })) >= UX { (UQ + (if (if ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) >= -1e1f64 { ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) } else { -1e1f64 }) <= UP { (if ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) >= -1e1f64 { ((-UW) - ((((JL * UW) * UW) * (((((-4e0f64 * US) * US) * UU) * UU) + (UN * UU))) * UV)) } else { -1e1f64 }) } else { UP })) } else { UX };
            let VA = if (TH - ((B + ((TH - (UZ * UY)).exp())).ln())) <= UY { (TH - ((B + ((TH - (UZ * UY)).exp())).ln())) } else { UY };
            let VB = SX - VA;
            let VC = SQ * VB;
            let VD = UD * (VA.exp());
            let VE = (VC * VC) + VD;
            let VF = if VE < A { 1.0 } else { 0.0 };
            let VT;
            let VU;
            let VY;
            let WA;
            let WC;
            if VF != 0.0 {
                let VG = (-VE).sqrt();
                let VH = JL * VG;
                let VI = B / (VH.sin());
                let VJ = VI * VI;
                let VK = (VH.cos()) * VI;
                let VL = (-5e-1f64 * VK) / VG;
                let VM = (LY * VJ) + VL;
                VT = VG;
                VU = VK;
                VY = VJ;
                WA = VL;
                WC = VM;
            } else {
                let VN = VE.sqrt();
                let VO = B / ((JL * VN).sinh());
                let VP = VO * VO;
                let VQ = (B + VP).sqrt();
                let VR = (JL * VQ) / VN;
                let VS = (-2.5e-1f64 * VP) + VR;
                VT = VN;
                VU = VQ;
                VY = VP;
                WA = VR;
                WC = VS;
            }
            let VV = VC + (VT * VU);
            let VW = B / VV;
            let VX = TA - SX;
            let VZ = (VX + VB) - (((((VE * VY) * VW) * VW).abs()).ln());
            let WB = ((-2e0f64 * SQ) * VC) + VD;
            let WD = WC * WB;
            let WE = -SQ;
            let WF = VA + ((-(VD + (VV * ((SR * VZ) + VC)))) / (((VD - (SQ * (VC + VV))) + (VC * WD)) + (SR * ((((-1e0f64 + (AC * ((WE + WD) * VW))) - (((B / VE) - WA) * WB)) * VV) + (VZ * (WD - SQ))))));
            let WG = SX - WF;
            let WH = SQ * WG;
            let WI = UD * (WF.exp());
            let WJ = (WH * WH) + WI;
            let WK = if WJ < A { 1.0 } else { 0.0 };
            let WY;
            let WZ;
            let XC;
            let XE;
            let XG;
            if WK != 0.0 {
                let WL = (-WJ).sqrt();
                let WM = JL * WL;
                let WN = B / (WM.sin());
                let WO = WN * WN;
                let WP = (WM.cos()) * WN;
                let WQ = (-5e-1f64 * WP) / WL;
                let WR = (LY * WO) + WQ;
                WY = WL;
                WZ = WP;
                XC = WO;
                XE = WQ;
                XG = WR;
            } else {
                let WS = WJ.sqrt();
                let WT = B / ((JL * WS).sinh());
                let WU = WT * WT;
                let WV = (B + WU).sqrt();
                let WW = (JL * WV) / WS;
                let WX = (-2.5e-1f64 * WU) + WW;
                WY = WS;
                WZ = WV;
                XC = WU;
                XE = WW;
                XG = WX;
            }
            let XA = WH + (WY * WZ);
            let XB = B / XA;
            let XD = (VX + WG) - (((((WJ * XC) * XB) * XB).abs()).ln());
            let XF = ((-2e0f64 * SQ) * WH) + WI;
            let XH = XG * XF;
            let XI = WF + ((-(WI + (XA * ((SR * XD) + WH)))) / (((WI - (SQ * (WH + XA))) + (WH * XH)) + (SR * ((((-1e0f64 + (AC * ((WE + XH) * XB))) - (((B / WJ) - XE) * XF)) * XA) + (XD * (XH - SQ))))));
            let XJ = SX - XI;
            let XK = SQ * XJ;
            let XL = UD * (XI.exp());
            let XM = (XK * XK) + XL;
            let XN = if XM < A { 1.0 } else { 0.0 };
            let YB;
            let YC;
            let YF;
            let YH;
            let YJ;
            if XN != 0.0 {
                let XO = (-XM).sqrt();
                let XP = JL * XO;
                let XQ = B / (XP.sin());
                let XR = XQ * XQ;
                let XS = (XP.cos()) * XQ;
                let XT = (-5e-1f64 * XS) / XO;
                let XU = (LY * XR) + XT;
                YB = XO;
                YC = XS;
                YF = XR;
                YH = XT;
                YJ = XU;
            } else {
                let XV = XM.sqrt();
                let XW = B / ((JL * XV).sinh());
                let XX = XW * XW;
                let XY = (B + XX).sqrt();
                let XZ = (JL * XY) / XV;
                let YA = (-2.5e-1f64 * XX) + XZ;
                YB = XV;
                YC = XY;
                YF = XX;
                YH = XZ;
                YJ = YA;
            }
            let YD = XK + (YB * YC);
            let YE = B / YD;
            let YG = (VX + XJ) - (((((XM * YF) * YE) * YE).abs()).ln());
            let YI = ((-2e0f64 * SQ) * XK) + XL;
            let YK = YJ * YI;
            let YL = XI + ((-(XL + (YD * ((SR * YG) + XK)))) / (((XL - (SQ * (XK + YD))) + (XK * YK)) + (SR * ((((-1e0f64 + (AC * ((WE + YK) * YE))) - (((B / XM) - YH) * YI)) * YD) + (YG * (YK - SQ))))));
            let YM = SX - YL;
            let YN = SQ * YM;
            let YO = UD * (YL.exp());
            let YP = (YN * YN) + YO;
            let YQ = if YP < A { 1.0 } else { 0.0 };
            let ZE;
            let ZF;
            let ZI;
            let ZK;
            let ZM;
            if YQ != 0.0 {
                let YR = (-YP).sqrt();
                let YS = JL * YR;
                let YT = B / (YS.sin());
                let YU = YT * YT;
                let YV = (YS.cos()) * YT;
                let YW = (-5e-1f64 * YV) / YR;
                let YX = (LY * YU) + YW;
                ZE = YR;
                ZF = YV;
                ZI = YU;
                ZK = YW;
                ZM = YX;
            } else {
                let YY = YP.sqrt();
                let YZ = B / ((JL * YY).sinh());
                let ZA = YZ * YZ;
                let ZB = (B + ZA).sqrt();
                let ZC = (JL * ZB) / YY;
                let ZD = (-2.5e-1f64 * ZA) + ZC;
                ZE = YY;
                ZF = ZB;
                ZI = ZA;
                ZK = ZC;
                ZM = ZD;
            }
            let ZG = YN + (ZE * ZF);
            let ZH = B / ZG;
            let ZJ = (VX + YM) - (((((YP * ZI) * ZH) * ZH).abs()).ln());
            let ZL = ((-2e0f64 * SQ) * YN) + YO;
            let ZN = ZM * ZL;
            let ZO = YL + ((-(YO + (ZG * ((SR * ZJ) + YN)))) / (((YO - (SQ * (YN + ZG))) + (YN * ZN)) + (SR * ((((-1e0f64 + (AC * ((WE + ZN) * ZH))) - (((B / YP) - ZK) * ZL)) * ZG) + (ZJ * (ZN - SQ))))));
            let ZP = SX - ZO;
            let ZQ = SQ * ZP;
            let ZR = UD * (ZO.exp());
            let ZS = (ZQ * ZQ) + ZR;
            let ZT = if ZS < A { 1.0 } else { 0.0 };
            let AAH;
            let AAI;
            let AAL;
            let AAN;
            let AAP;
            if ZT != 0.0 {
                let ZU = (-ZS).sqrt();
                let ZV = JL * ZU;
                let ZW = B / (ZV.sin());
                let ZX = ZW * ZW;
                let ZY = (ZV.cos()) * ZW;
                let ZZ = (-5e-1f64 * ZY) / ZU;
                let AAA = (LY * ZX) + ZZ;
                AAH = ZU;
                AAI = ZY;
                AAL = ZX;
                AAN = ZZ;
                AAP = AAA;
            } else {
                let AAB = ZS.sqrt();
                let AAC = B / ((JL * AAB).sinh());
                let AAD = AAC * AAC;
                let AAE = (B + AAD).sqrt();
                let AAF = (JL * AAE) / AAB;
                let AAG = (-2.5e-1f64 * AAD) + AAF;
                AAH = AAB;
                AAI = AAE;
                AAL = AAD;
                AAN = AAF;
                AAP = AAG;
            }
            let AAJ = ZQ + (AAH * AAI);
            let AAK = B / AAJ;
            let AAM = (VX + ZP) - (((((ZS * AAL) * AAK) * AAK).abs()).ln());
            let AAO = ((-2e0f64 * SQ) * ZQ) + ZR;
            let AAQ = AAP * AAO;
            let AAR = ZO + ((-(ZR + (AAJ * ((SR * AAM) + ZQ)))) / (((ZR - (SQ * (ZQ + AAJ))) + (ZQ * AAQ)) + (SR * ((((-1e0f64 + (AC * ((WE + AAQ) * AAK))) - (((B / ZS) - AAN) * AAO)) * AAJ) + (AAM * (AAQ - SQ))))));
            let AAS = SX - AAR;
            let AAT = SP * (AAR.exp());
            let AAU = ((SV * AAS) * AAS) - AAT;
            let AAV = if AAU < A { 1.0 } else { 0.0 };
            let ABG;
            let ABH;
            if AAV != 0.0 {
                let AAW = (-AAU).sqrt();
                let AAX = JL * AAW;
                let AAY = AAW / (AAX.tan());
                let AAZ = AAX.sin();
                let ABA = (-AAZ) * AAZ;
                ABG = AAY;
                ABH = ABA;
            } else {
                let ABB = AAU.sqrt();
                let ABC = JL * ABB;
                let ABD = ABC.sinh();
                let ABE = ABD * ABD;
                let ABF = ABB / (ABC.tanh());
                ABG = ABF;
                ABH = ABE;
            }
            let ABI = ((SQ * AAS) - ABG) / (B - (AAU / (ABH * AAT)));
            let ABJ = (AAS * GI) * SJ;
            let ABK = (ABI * GO) * SJ;
            let ABL = ABK - ABJ;
            let ABM = TA - (ABL / (GM * SJ));
            let ABN = ((AAR + ABM) * SJ) / AC;
            let ABO = ABK / GI;
            let ABQ = ((ABP * ABJ) / GI) + SK;
            let ABS = ((ABR * ABL) / GM) + SK;
            let ABT = LZ / GI;
            let ABU = JL * (B + ((ABO / ABT).abs()));
            let ABW = B + (((NP + (PS * NM)) * (((JV * (JL * (ABQ + (((ABQ * ABQ) + KF).sqrt())))).abs()).powf((ABV + (HD * PS))))) + (NR / (ABU.powf(NT))));
            let ABX = ABW - B;
            let ABZ = (LY * ABY) * ABY;
            let ACB = B + (((HI + (PS * HJ)) * (((JZ * (JL * (ABS + (((ABS * ABS) + KF).sqrt())))).abs()).powf((HM + (HN * PS))))) + (HL / (ABU.powf(DT))));
            let ACC = ACB - B;
            let ACE = ((SO - (ABJ / GI)) / SJ).exp();
            let ACF = ((SY - (ABL / GM)) / SJ).exp();
            let ACG = ACE + ACF;
            let ACH = ((ACE / ACG) * (NK / ((JL * ((ABW + B) + (((ABX * ABX) + ABZ).sqrt()))) / ACA))) + ((ACF / ACG) * (ACD / ((JL * ((ACB + B) + (((ACC * ACC) + ABZ).sqrt()))) / ACA)));
            let ACY;
            if HP != 0.0 {
                ACY = A;
            } else {
                let ACI = if HO == A { 1.0 } else { 0.0 };
                let ACZ = if ACI != 0.0 {
                    let ACK = B / (B + (ACJ * ABO));
                    let ACP = (((ACL + (ACN * (JL * (ACK + (((ACK * ACK) + LZ).sqrt()))))) * JW) * Q) * NV;
                    ACP
                } else {
                    let ACQ = B / (B + (ACJ * ABO));
                    let ACT = (((((ACR + ACS) + ACL) + (ACN * (JL * (ACQ + (((ACQ * ACQ) + LZ).sqrt()))))) * JW) * Q) * NV;
                    ACT
                };
                ACY = ACZ;
            }
            let ACV = AC * ACU;
            let ACW = (ACV / ACH) * AD;
            let ACX = FT * ((ABO + (FV * SA)) + (MI * FU));
            let ADA = if ACY == A { 1.0 } else { 0.0 };
            let ADG = if ADA != 0.0 {
                let ADB = (ACW * ACX) / (ACW + ACX);
                ADB
            } else {
                let ADC = ((AH * ACU) * GI) * ACY;
                let ADD = AC * ADC;
                let ADE = (ACX + ACW) + ((JP * ACX) * ADC);
                let ADF = (ADE - (((ADE * ADE) - ((AC * ADD) * (ACX * (ACW + ((AC * ACX) * ADC))))).sqrt())) / ADD;
                ADF
            };
            let ADH = ADG - KF;
            let ADI = (JL * (ADH + (((ADH * ADH) + 4.0000000000000007e-10f64).sqrt()))) + KF;
            let ADJ = PO / ((B + ((PO / ADI).powf(OK))).powf(JJ));
            let ADK = if ADJ > PO { 1.0 } else { 0.0 };
            let ADL = if ADK != 0.0 {
                PO
            } else {
                ADJ
            };
            let ADM = (SO - ADL) / SJ;
            let ADN = (SZ - ADL) / SJ;
            let ADO = ADM - SU;
            let ADP = ((((SV * ADO) * ADO) + ST).ln()) - SS;
            let ADQ = ((TZ - ABM) / SQ) - SU;
            let ADR = ((ADP - ((((((SV * ADQ) * ADQ) + ST).ln()) - SS) - SU)) + (SR * ADN)) / TD;
            let ADS = if (if (ADN + (SW * (ADM - ADN))) <= ADP { (ADN + (SW * (ADM - ADN))) } else { ADP }) <= SU { (if (ADN + (SW * (ADM - ADN))) <= ADP { (ADN + (SW * (ADM - ADN))) } else { ADP }) } else { SU };
            let ADT = (ADS + (SQ * ADM)) / TG;
            let ADU = ADN - ADR;
            let ADV = ((TJ * ADU) * ADU) - (SP * (ADR.exp()));
            let ADW = if ADV < A { 1.0 } else { 0.0 };
            let AEL = if ADW != 0.0 {
                let ADX = (ADN - ADS) * SR;
                let ADY = QA * SQ;
                let ADZ = ADY + ADX;
                let AEA = ADY * ADX;
                let AEB = (TQ * ADZ) + B;
                let AEC = ((ADZ * TS) + AEA) + ST;
                let AED = if ((((-AEC) + ((((-4e0f64 * AEB) * ((TU * ADZ) + (ST * AEA))) + (AEC * AEC)).sqrt())) / (AC * AEB)) * (B - (((-((ADM - ((TZ - ADS) / SQ)) + AC)) / 2.8985507246376816e0f64).exp()))) <= TV { ((((-AEC) + ((((-4e0f64 * AEB) * ((TU * ADZ) + (ST * AEA))) + (AEC * AEC)).sqrt())) / (AC * AEB)) * (B - (((-((ADM - ((TZ - ADS) / SQ)) + AC)) / 2.8985507246376816e0f64).exp()))) } else { TV };
                AED
            } else {
                ADV
            };
            let AEE = if ADM >= SU { ADM } else { SU };
            let AEF = AEE - SU;
            let AEG = ((TZ - ADS) / SQ) - SU;
            let AEH = (((((SV * AEF) * AEF) + ST).ln()) - SS) - ((((((SV * AEG) * AEG) + ST).ln()) - SS) - SU);
            let AEI = AEE - AEH;
            let AEJ = UD * (AEH.exp());
            let AEK = SV * AEI;
            let AEM = AEH + ((-(((AEK * AEI) + AEJ) - AEL)) / ((-2e0f64 * AEK) + AEJ));
            let AEN = AEE - AEM;
            let AEO = SV * AEN;
            let AEP = (AEO * AEN) - AEL;
            let AEQ = B / AEP;
            let AER = B / (((-2e0f64 * AEO) * AEQ) - B);
            let AES = ((((AEP.abs()).ln()) - SS) - AEM) * AER;
            let AET = AEM + (if (if ((-AES) - ((((JL * AES) * AES) * (((((-4e0f64 * AEO) * AEO) * AEQ) * AEQ) + (UN * AEQ))) * AER)) >= -1e1f64 { ((-AES) - ((((JL * AES) * AES) * (((((-4e0f64 * AEO) * AEO) * AEQ) * AEQ) + (UN * AEQ))) * AER)) } else { -1e1f64 }) <= UP { (if ((-AES) - ((((JL * AES) * AES) * (((((-4e0f64 * AEO) * AEO) * AEQ) * AEQ) + (UN * AEQ))) * AER)) >= -1e1f64 { ((-AES) - ((((JL * AES) * AES) * (((((-4e0f64 * AEO) * AEO) * AEQ) * AEQ) + (UN * AEQ))) * AER)) } else { -1e1f64 }) } else { UP });
            let AEU = AEE - AET;
            let AEV = SV * AEU;
            let AEW = (AEV * AEU) - AEL;
            let AEX = B / AEW;
            let AEY = B / (((-2e0f64 * AEV) * AEX) - B);
            let AEZ = ((((AEW.abs()).ln()) - SS) - AET) * AEY;
            let AFA = if (AET + (if (if ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) >= -1e1f64 { ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) } else { -1e1f64 }) <= UP { (if ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) >= -1e1f64 { ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) } else { -1e1f64 }) } else { UP })) >= UX { (AET + (if (if ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) >= -1e1f64 { ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) } else { -1e1f64 }) <= UP { (if ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) >= -1e1f64 { ((-AEZ) - ((((JL * AEZ) * AEZ) * (((((-4e0f64 * AEV) * AEV) * AEX) * AEX) + (UN * AEX))) * AEY)) } else { -1e1f64 }) } else { UP })) } else { UX };
            let AFB = if (ADT - ((B + ((ADT - (UZ * AFA)).exp())).ln())) <= AFA { (ADT - ((B + ((ADT - (UZ * AFA)).exp())).ln())) } else { AFA };
            let AFC = ADM - AFB;
            let AFD = SQ * AFC;
            let AFE = UD * (AFB.exp());
            let AFF = (AFD * AFD) + AFE;
            let AFG = if AFF < A { 1.0 } else { 0.0 };
            let AFU;
            let AFV;
            let AFZ;
            let AGB;
            let AGD;
            if AFG != 0.0 {
                let AFH = (-AFF).sqrt();
                let AFI = JL * AFH;
                let AFJ = B / (AFI.sin());
                let AFK = AFJ * AFJ;
                let AFL = (AFI.cos()) * AFJ;
                let AFM = (-5e-1f64 * AFL) / AFH;
                let AFN = (LY * AFK) + AFM;
                AFU = AFH;
                AFV = AFL;
                AFZ = AFK;
                AGB = AFM;
                AGD = AFN;
            } else {
                let AFO = AFF.sqrt();
                let AFP = B / ((JL * AFO).sinh());
                let AFQ = AFP * AFP;
                let AFR = (B + AFQ).sqrt();
                let AFS = (JL * AFR) / AFO;
                let AFT = (-2.5e-1f64 * AFQ) + AFS;
                AFU = AFO;
                AFV = AFR;
                AFZ = AFQ;
                AGB = AFS;
                AGD = AFT;
            }
            let AFW = AFD + (AFU * AFV);
            let AFX = B / AFW;
            let AFY = ADN - ADM;
            let AGA = (AFY + AFC) - (((((AFF * AFZ) * AFX) * AFX).abs()).ln());
            let AGC = ((-2e0f64 * SQ) * AFD) + AFE;
            let AGE = AGD * AGC;
            let AGF = AFB + ((-(AFE + (AFW * ((SR * AGA) + AFD)))) / (((AFE - (SQ * (AFD + AFW))) + (AFD * AGE)) + (SR * ((((-1e0f64 + (AC * ((WE + AGE) * AFX))) - (((B / AFF) - AGB) * AGC)) * AFW) + (AGA * (AGE - SQ))))));
            let AGG = ADM - AGF;
            let AGH = SQ * AGG;
            let AGI = UD * (AGF.exp());
            let AGJ = (AGH * AGH) + AGI;
            let AGK = if AGJ < A { 1.0 } else { 0.0 };
            let AGY;
            let AGZ;
            let AHC;
            let AHE;
            let AHG;
            if AGK != 0.0 {
                let AGL = (-AGJ).sqrt();
                let AGM = JL * AGL;
                let AGN = B / (AGM.sin());
                let AGO = AGN * AGN;
                let AGP = (AGM.cos()) * AGN;
                let AGQ = (-5e-1f64 * AGP) / AGL;
                let AGR = (LY * AGO) + AGQ;
                AGY = AGL;
                AGZ = AGP;
                AHC = AGO;
                AHE = AGQ;
                AHG = AGR;
            } else {
                let AGS = AGJ.sqrt();
                let AGT = B / ((JL * AGS).sinh());
                let AGU = AGT * AGT;
                let AGV = (B + AGU).sqrt();
                let AGW = (JL * AGV) / AGS;
                let AGX = (-2.5e-1f64 * AGU) + AGW;
                AGY = AGS;
                AGZ = AGV;
                AHC = AGU;
                AHE = AGW;
                AHG = AGX;
            }
            let AHA = AGH + (AGY * AGZ);
            let AHB = B / AHA;
            let AHD = (AFY + AGG) - (((((AGJ * AHC) * AHB) * AHB).abs()).ln());
            let AHF = ((-2e0f64 * SQ) * AGH) + AGI;
            let AHH = AHG * AHF;
            let AHI = AGF + ((-(AGI + (AHA * ((SR * AHD) + AGH)))) / (((AGI - (SQ * (AGH + AHA))) + (AGH * AHH)) + (SR * ((((-1e0f64 + (AC * ((WE + AHH) * AHB))) - (((B / AGJ) - AHE) * AHF)) * AHA) + (AHD * (AHH - SQ))))));
            let AHJ = ADM - AHI;
            let AHK = SQ * AHJ;
            let AHL = UD * (AHI.exp());
            let AHM = (AHK * AHK) + AHL;
            let AHN = if AHM < A { 1.0 } else { 0.0 };
            let AIB;
            let AIC;
            let AIF;
            let AIH;
            let AIJ;
            if AHN != 0.0 {
                let AHO = (-AHM).sqrt();
                let AHP = JL * AHO;
                let AHQ = B / (AHP.sin());
                let AHR = AHQ * AHQ;
                let AHS = (AHP.cos()) * AHQ;
                let AHT = (-5e-1f64 * AHS) / AHO;
                let AHU = (LY * AHR) + AHT;
                AIB = AHO;
                AIC = AHS;
                AIF = AHR;
                AIH = AHT;
                AIJ = AHU;
            } else {
                let AHV = AHM.sqrt();
                let AHW = B / ((JL * AHV).sinh());
                let AHX = AHW * AHW;
                let AHY = (B + AHX).sqrt();
                let AHZ = (JL * AHY) / AHV;
                let AIA = (-2.5e-1f64 * AHX) + AHZ;
                AIB = AHV;
                AIC = AHY;
                AIF = AHX;
                AIH = AHZ;
                AIJ = AIA;
            }
            let AID = AHK + (AIB * AIC);
            let AIE = B / AID;
            let AIG = (AFY + AHJ) - (((((AHM * AIF) * AIE) * AIE).abs()).ln());
            let AII = ((-2e0f64 * SQ) * AHK) + AHL;
            let AIK = AIJ * AII;
            let AIL = AHI + ((-(AHL + (AID * ((SR * AIG) + AHK)))) / (((AHL - (SQ * (AHK + AID))) + (AHK * AIK)) + (SR * ((((-1e0f64 + (AC * ((WE + AIK) * AIE))) - (((B / AHM) - AIH) * AII)) * AID) + (AIG * (AIK - SQ))))));
            let AIM = ADM - AIL;
            let AIN = SQ * AIM;
            let AIO = UD * (AIL.exp());
            let AIP = (AIN * AIN) + AIO;
            let AIQ = if AIP < A { 1.0 } else { 0.0 };
            let AJE;
            let AJF;
            let AJI;
            let AJK;
            let AJM;
            if AIQ != 0.0 {
                let AIR = (-AIP).sqrt();
                let AIS = JL * AIR;
                let AIT = B / (AIS.sin());
                let AIU = AIT * AIT;
                let AIV = (AIS.cos()) * AIT;
                let AIW = (-5e-1f64 * AIV) / AIR;
                let AIX = (LY * AIU) + AIW;
                AJE = AIR;
                AJF = AIV;
                AJI = AIU;
                AJK = AIW;
                AJM = AIX;
            } else {
                let AIY = AIP.sqrt();
                let AIZ = B / ((JL * AIY).sinh());
                let AJA = AIZ * AIZ;
                let AJB = (B + AJA).sqrt();
                let AJC = (JL * AJB) / AIY;
                let AJD = (-2.5e-1f64 * AJA) + AJC;
                AJE = AIY;
                AJF = AJB;
                AJI = AJA;
                AJK = AJC;
                AJM = AJD;
            }
            let AJG = AIN + (AJE * AJF);
            let AJH = B / AJG;
            let AJJ = (AFY + AIM) - (((((AIP * AJI) * AJH) * AJH).abs()).ln());
            let AJL = ((-2e0f64 * SQ) * AIN) + AIO;
            let AJN = AJM * AJL;
            let AJO = AIL + ((-(AIO + (AJG * ((SR * AJJ) + AIN)))) / (((AIO - (SQ * (AIN + AJG))) + (AIN * AJN)) + (SR * ((((-1e0f64 + (AC * ((WE + AJN) * AJH))) - (((B / AIP) - AJK) * AJL)) * AJG) + (AJJ * (AJN - SQ))))));
            let AJP = ADM - AJO;
            let AJQ = SQ * AJP;
            let AJR = UD * (AJO.exp());
            let AJS = (AJQ * AJQ) + AJR;
            let AJT = if AJS < A { 1.0 } else { 0.0 };
            let AKH;
            let AKI;
            let AKL;
            let AKN;
            let AKP;
            if AJT != 0.0 {
                let AJU = (-AJS).sqrt();
                let AJV = JL * AJU;
                let AJW = B / (AJV.sin());
                let AJX = AJW * AJW;
                let AJY = (AJV.cos()) * AJW;
                let AJZ = (-5e-1f64 * AJY) / AJU;
                let AKA = (LY * AJX) + AJZ;
                AKH = AJU;
                AKI = AJY;
                AKL = AJX;
                AKN = AJZ;
                AKP = AKA;
            } else {
                let AKB = AJS.sqrt();
                let AKC = B / ((JL * AKB).sinh());
                let AKD = AKC * AKC;
                let AKE = (B + AKD).sqrt();
                let AKF = (JL * AKE) / AKB;
                let AKG = (-2.5e-1f64 * AKD) + AKF;
                AKH = AKB;
                AKI = AKE;
                AKL = AKD;
                AKN = AKF;
                AKP = AKG;
            }
            let AKJ = AJQ + (AKH * AKI);
            let AKK = B / AKJ;
            let AKM = (AFY + AJP) - (((((AJS * AKL) * AKK) * AKK).abs()).ln());
            let AKO = ((-2e0f64 * SQ) * AJQ) + AJR;
            let AKQ = AKP * AKO;
            let AKR = AJO + ((-(AJR + (AKJ * ((SR * AKM) + AJQ)))) / (((AJR - (SQ * (AJQ + AKJ))) + (AJQ * AKQ)) + (SR * ((((-1e0f64 + (AC * ((WE + AKQ) * AKK))) - (((B / AJS) - AKN) * AKO)) * AKJ) + (AKM * (AKQ - SQ))))));
            let AKS = ADM - AKR;
            let AKT = SP * (AKR.exp());
            let AKU = ((SV * AKS) * AKS) - AKT;
            let AKV = if AKU < A { 1.0 } else { 0.0 };
            let ALG;
            let ALH;
            if AKV != 0.0 {
                let AKW = (-AKU).sqrt();
                let AKX = JL * AKW;
                let AKY = AKW / (AKX.tan());
                let AKZ = AKX.sin();
                let ALA = (-AKZ) * AKZ;
                ALG = AKY;
                ALH = ALA;
            } else {
                let ALB = AKU.sqrt();
                let ALC = JL * ALB;
                let ALD = ALC.sinh();
                let ALE = ALD * ALD;
                let ALF = ALB / (ALC.tanh());
                ALG = ALF;
                ALH = ALE;
            }
            let ALI = ((SQ * AKS) - ALG) / (B - (AKU / (ALH * AKT)));
            let ALJ = (AKS * GI) * SJ;
            let ALK = (ALI * GO) * SJ;
            let ALL = ALK - ALJ;
            let ALM = ALK / GI;
            let ALN = JL * (ABO + ALM);
            let ALO = ABO - ALM;
            let ALP = ADL * ADL;
            let ALQ = ALP / 6.25e-4f64;
            let ALS = if ALR != A { 1.0 } else { 0.0 };
            let ALZ = if ALS != 0.0 {
                let ALT = ((ABJ + ALJ) / (AC * GI)) + ((((ALR * (B - (rspice_limited_exp((-ALQ))))) * JL) * (ABJ - ALJ)) / GI);
                ALT
            } else {
                let ALU = (ABJ + ALJ) / (AC * GI);
                ALU
            };
            let ALW = if ALV != A { 1.0 } else { 0.0 };
            let AMB = if ALW != 0.0 {
                let ALX = ((ABL + ALL) / RM) + ((((ALV * (B - (rspice_limited_exp((-ALQ))))) * JL) * (ABL - ALL)) / GM);
                ALX
            } else {
                let ALY = (ABL + ALL) / RM;
                ALY
            };
            let AMA = (ABP * ALZ) + SK;
            let AMC = (ABR * AMB) + SK;
            let AMD = JL * (B + ((ALN / ABT).abs()));
            let AME = B + (((NP + (PT * NM)) * (((JV * (JL * (AMA + (((AMA * AMA) + KF).sqrt())))).abs()).powf((ABV + (HD * PT))))) + ((NR + (PT * HA)) / (AMD.powf(NT))));
            let AMF = AME - B;
            let AMG = B + (((HI + (PT * HJ)) * (((JZ * (JL * (AMC + (((AMC * AMC) + KF).sqrt())))).abs()).powf((HM + (HN * PT))))) + ((HL + (PT * HK)) / (AMD.powf(DT))));
            let AMH = AMG - B;
            let AMI = ((SO - ((ABJ + ALJ) / (AC * GI))) / SJ).exp();
            let AMJ = ((SY - ((ABL + ALL) / RM)) / SJ).exp();
            let AMK = AMI + AMJ;
            let AML = ((AMI / AMK) * (NK / ((JL * ((AME + B) + (((AMF * AMF) + ABZ).sqrt()))) / ACA))) + ((AMJ / AMK) * (ACD / ((JL * ((AMG + B) + (((AMH * AMH) + ABZ).sqrt()))) / ACA)));
            let AMM = ((AML * GI) * AH) / AD;
            let AMO = B + (NP * (((JV * (SK + (AMN * ALN))).abs()).powf(ABV)));
            let AMP = AMO - B;
            let AMR = 8e-1f64 + (OI * PT);
            let AMS = (ALO / (((AC * AMQ) / AML) * AD)) * (2e-1f64 + (JL * (AMR + (((AMR * AMR) + LZ).sqrt()))));
            let AMU = ((B + ((AMT + (AMS * AMS)).sqrt())) / (B + (AMT.sqrt()))) + ((((JL * ((OR - (HX * SA)) - (HY * PT))) * ALN) * ALO) * ALO);
            let AMV = AMU - B;
            let AMX = JL * ((AMU + B) + (((AMV * AMV) + ((LY * AMW) * AMW)).sqrt()));
            let AMZ = (((AC * AMY) * ((JL * ((AMO + B) + (((AMP * AMP) + ABZ).sqrt()))) / ACA)) / NK) * AL;
            let ANA = if EJ > A { 1.0 } else { 0.0 };
            let ANH = if ANA != 0.0 {
                let ANB = B + ((EJ * ALN) / ACW);
                ANB
            } else {
                let ANC = B / (B - ((EJ * ALN) / ACW));
                ANC
            };
            let AND = PO - ADL;
            let ANE = ALN + MI;
            let ANG = if ANF > A { 1.0 } else { 0.0 };
            let ANQ = if ANG != 0.0 {
                let ANI = B + (AND / (((ANE / ANF) * (ANE / (ADI + ANE))) * ANH));
                ANI
            } else {
                B
            };
            let ANJ = if HT > A { 1.0 } else { 0.0 };
            let ANR;
            if ANJ != 0.0 {
                let ANL = if ANK < A { 1.0 } else { 0.0 };
                let ANO = if ANL != 0.0 {
                    let ANM = B / ((B / HT) - (ANK * ALN));
                    ANM
                } else {
                    let ANN = HT * (B + (ANK * ALN));
                    ANN
                };
                let ANP = B + (ANO * ((if (B + ((AND / ANO) / (ADI + ACW))) >= JK { (B + ((AND / ANO) / (ADI + ACW))) } else { JK }).ln()));
                ANR = ANP;
            } else {
                ANR = B;
            }
            let ANS = ANQ * ANR;
            let ANT = if EF > A { 1.0 } else { 0.0 };
            let APG = if ANT != 0.0 {
                let ANU = B + (EF * ((if (B + ((AND / EF) / (ADI + AMZ))) >= JK { (B + ((AND / EF) / (ADI + AMZ))) } else { JK }).ln()));
                ANU
            } else {
                B
            };
            let ANV = if OL != A { 1.0 } else { 0.0 };
            let AOX = if ANV != 0.0 {
                let ANW = rspice_limited_exp((-(OL / (((if A >= (OO + ((OP * ALO) * ALO)) { A } else { (OO + ((OP * ALO) * ALO)) }) * ALN) + (AC * SJ)))));
                ANW
            } else {
                B
            };
            let ANX = GO * SJ;
            let ANY = (((ANX * AC) * ME) * (ABI - ALI)) + (((((ANX * GO) * SJ) * JL) * ((ABI * ABI) - (ALI * ALI))) / GI);
            let ANZ = ALN + ME;
            let AOY;
            let ASX;
            let ATQ;
            let ATS;
            if HP != 0.0 {
                let AOA = PD - NF;
                let AOB = (B / (B + (ACJ * (JL * (AOA + (((AOA * AOA) + MQ).sqrt())))))) - ((JL * PI) * AZ);
                let AOG = NV * (ACR + ((AOC + (AOE * (JL * (AOB + (((AOB * AOB) + LZ).sqrt()))))) * JW));
                let AOH = PG - NF;
                let AOI = (B / (B + (ACJ * (JL * (AOH + (((AOH * AOH) + MQ).sqrt())))))) - ((JL * PJ) * AZ);
                let AON = NV * (ACS + ((AOJ + (AOL * (JL * (AOI + (((AOI * AOI) + LZ).sqrt()))))) * JW));
                AOY = B;
                ASX = A;
                ATQ = AON;
                ATS = AOG;
            } else {
                let AOP = (B / (B + (ACJ * ALN))) - ((JL * (AOO + PS)) * AZ);
                let AOQ = AOP + (((AOP * AOP) + LZ).sqrt());
                let AOR = NV * ((ACL + (ACN * (JL * AOQ))) * JW);
                let AOS = ((Q * AMM) * ANZ) / AMX;
                let AOT = B + (AOS * AOR);
                let AOU = if HO == AC { 1.0 } else { 0.0 };
                let AOZ;
                let ASY;
                let ATR;
                let ATT;
                if AOU != 0.0 {
                    let AOV = (NV * (((ACR + ACS) + ACL) + (ACN * (JL * AOQ)))) * JW;
                    let AOW = B + (AOS * AOV);
                    AOZ = AOW;
                    ASY = AOV;
                    ATR = A;
                    ATT = A;
                } else {
                    AOZ = AOT;
                    ASY = AOR;
                    ATR = ACS;
                    ATT = ACR;
                }
                AOY = AOZ;
                ASX = ASY;
                ATQ = ATR;
                ATS = ATT;
            }
            let APA = Q * (((((AMM / GI) * ANY) * ANS) * AOX) / (AMX * AOY));
            let APB = 1.6666666666666666e-1f64 * (ABK + (AC * ALK));
            let APC = 1.6666666666666666e-1f64 * ((AC * ABK) + ALK);
            let APD = if FY > A { 1.0 } else { 0.0 };
            let ASC = if APD != 0.0 {
                let APF = 3.4531302e-11f64 / (((APE * GG) / GR) + (((GN / (B + (((ALN + (FZ * SK)) / GA).powf(GB)))) * FY) / GP));
                APF
            } else {
                GK
            };
            let APH = (AO * AL) / APG;
            let API = (-APB) * APH;
            let APJ = (-APC) * APH;
            let APK = if (if ((EK + (EL * AD)) / AD) <= A { 1.0 } else { 0.0 }) != 0.0 || (if OU <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if APK != 0.0 {
            } else {
                let APL = if AND > (OU / 8e1f64) { 1.0 } else { 0.0 };
                if APL != 0.0 {
                } else {
                }
            }
            let APM = if parameters[17] != A { 1.0 } else { 0.0 };
            let AQB;
            let AQC;
            if APM != 0.0 {
                let APN = AH * AD;
                let APO = (((((APN * 3.75956e-7f64) * LG) * PK) * ((ER * ME) * ((if (B + (rspice_limited_exp((((ALN - EQ) / ER) / ME)))) >= JK { (B + (rspice_limited_exp((((ALN - EQ) / ER) / ME)))) } else { JK }).ln()))) * (rspice_limited_exp((((-9.82222e11f64 * LD) * (EN - (EO * ALN))) * (B + (EP * ALN)))))) * OZ;
                let APP = NA - ML;
                let APQ = APP - PK;
                let APR = (EV * ME) * ((if (B + (rspice_limited_exp(((APQ / EV) / ME)))) >= JK { (B + (rspice_limited_exp(((APQ / EV) / ME)))) } else { JK }).ln());
                let APS = if APP <= A { 1.0 } else { 0.0 };
                let APY = if APS != 0.0 {
                    let APT = APQ - PP;
                    let APV = JL * (APT + (((APT * APT) - (APU * APP)).sqrt()));
                    APV
                } else {
                    let APW = APQ - PP;
                    let APX = JL * (APW + (((APW * APW) + (APU * APP)).sqrt()));
                    APX
                };
                let APZ = (((((APN * KZ) * LG) * PK) * APR) * (rspice_limited_exp((((-7.45669e11f64 * LD) * (ES - (ET * APY))) * (B + (EU * APY)))))) * OZ;
                AQB = APO;
                AQC = APZ;
            } else {
                AQB = A;
                AQC = A;
            }
            let AQA = JL + (JL * (((6e-1f64 * PF) / ME).tanh()));
            let AQD = AQB + AQC;
            let AQE = AQA * AQD;
            let AQF = (B - AQA) * AQD;
            let AQG = if parameters[16] != A { 1.0 } else { 0.0 };
            let ATD;
            let ATF;
            let ATH;
            let ATL;
            if AQG != 0.0 {
                let AQH = PV - (EZ * ABN);
                let AQJ = (-AQI) * LD;
                let AQK = (((((AH * AD) * LH) * LG) * (ALN * (rspice_limited_exp(((AQJ * (EW - (EX * AQH))) * (B + (EY * AQH))))))) * ((PK + (JL * PQ)) + (JL * (PI + PJ)))) * OZ;
                let AQL = FA * (((ALP + LZ).sqrt()) - 1e-1f64);
                let AQM = rspice_limited_exp((-AQL));
                let AQN = (AQL * AQL) + 2e-4f64;
                let AQO = (AQK * ((B - ((AQL + B) * AQM)) + MQ)) / AQN;
                let AQP = (AQK * (((AQL + AQM) - B) + MQ)) / AQN;
                let AQQ = PS - NG;
                let AQR = (PD - NF) + ((FI * RX) * AQQ);
                let AQS = ((AQR * AQR) + MQ).sqrt();
                let AQT = AQJ * FO;
                let AQU = rspice_limited_exp(((AQT * (FF - (FG * AQS))) * (B + (FH * AQS))));
                let AQW = if AQV > A { 1.0 } else { 0.0 };
                let ATJ;
                let ATN;
                if AQW != 0.0 {
                    let AQY = (((PA * AQX) * PD) * AQS) * AQU;
                    ATJ = AQY;
                    ATN = A;
                } else {
                    let AQZ = (((PA * AQX) * PD) * AQS) * AQU;
                    ATJ = A;
                    ATN = AQZ;
                }
                let ARA = (PG - NF) + ((FM * RX) * AQQ);
                let ARB = ((ARA * ARA) + MQ).sqrt();
                let ARC = rspice_limited_exp(((AQT * (FJ - (FK * ARB))) * (B + (FL * ARB))));
                let ATI;
                let ATM;
                if AQW != 0.0 {
                    let ARE = (((PA * ARD) * PG) * ARB) * ARC;
                    ATI = ATJ;
                    ATM = ARE;
                } else {
                    let ARF = (((PA * ARD) * PG) * ARB) * ARC;
                    ATI = ARF;
                    ATM = ATN;
                }
                ATD = AQO;
                ATF = AQP;
                ATH = ATI;
                ATL = ATM;
            } else {
                ATD = A;
                ATF = A;
                ATH = A;
                ATL = A;
            }
            let ARG = if parameters[15] != A { 1.0 } else { 0.0 };
            if ARG != 0.0 {
                let ARH = if (if FD <= A { 1.0 } else { 0.0 }) != 0.0 || (if OW <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if ARH != 0.0 {
                } else {
                }
                let ARI = if AQV > A { 1.0 } else { 0.0 };
                if ARI != 0.0 {
                } else {
                }
                let ARJ = if (if FB <= A { 1.0 } else { 0.0 }) != 0.0 || (if OY <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if ARJ != 0.0 {
                } else {
                }
                if ARI != 0.0 {
                } else {
                }
            } else {
            }
            let ARK = ACV / AML;
            let ARO = if (if (if ARL > A { 1.0 } else { 0.0 }) != 0.0 || (if ARM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if ARN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ATZ;
            if ARO != 0.0 {
                let ARQ = AD - (AC * ARP);
                let ARR = ARQ * ARQ;
                let ART = if ARS <= A { 1.0 } else { 0.0 };
                let ASI;
                if ART != 0.0 {
                    ASI = A;
                } else {
                    let ARU = JX * ((if (((AND / JX) + ARS) / ARK) >= JK { (((AND / JX) + ARS) / ARK) } else { JK }).ln());
                    let ARV = if ARU < A { 1.0 } else { 0.0 };
                    let ASJ = if ARV != 0.0 {
                        A
                    } else {
                        ARU
                    };
                    ASI = ASJ;
                }
                let ARW = if parameters[22] == B { 1.0 } else { 0.0 };
                let ASG = if ARW != 0.0 {
                    let ARX = (BK / (B + ((ALZ / BL).powf(BM)))) / ARL;
                    let ARY = ARX - B;
                    let ASA = ARL * (JL * ((ARX + B) + (((ARY * ARY) + ((LY * ARZ) * ARZ)).sqrt())));
                    ASA
                } else {
                    ARL
                };
                let ASD = (ASC * ABO) / RL;
                let ASE = (ASC * ALM) / RL;
                let ASF = (ME / RL) * (ASC + BF);
                let ASH = ASE + ASF;
                let ASK = (((((4.112842231783458e-57f64 * ME) * (APA.abs())) * AML) / ((ASB * ASC) * ARR)) * (((ASG * ((if ((ASD + ASF) / ASH) >= JK { ((ASD + ASF) / ASH) } else { JK }).ln())) + (ARM * (ASD - ASE))) + ((JL * ARN) * ((ASD * ASD) - (ASE * ASE))))) + (((((((RL * ME) * APA) * APA) / (((ASB * ARR) * AH) * Q)) * ASI) * ((ASG + (ARM * ASE)) + ((ARN * ASE) * ASE))) / (ASH * ASH));
                let ASL = ((((ASG * RL) * ME) / (((((AH * Q) * ARQ) * ASB) * ASF) * ASF)) * APA) * APA;
                let ASM = ASL + ASK;
                let ASN = if ASM > A { 1.0 } else { 0.0 };
                let AUA = if ASN != 0.0 {
                    let ASO = (ASK * ASL) / ASM;
                    ASO
                } else {
                    A
                };
                ATZ = AUA;
            } else {
                ATZ = A;
            }
            let ASP = if AQV > A { 1.0 } else { 0.0 };
            let ASU;
            let ASV;
            if ASP != 0.0 {
                let ASQ = Q * APJ;
                let ASR = Q * API;
                ASU = ASQ;
                ASV = ASR;
            } else {
                let ASS = Q * API;
                let AST = Q * APJ;
                ASU = ASS;
                ASV = AST;
            }
            let ASW = AML * (-(ASU + ASV));
            let ASZ = (MP * ME) * RL;
            let ATA = ASZ * ((ASW / ((ASW * ASX) + (AD * AD))) * parameters[295]);
            let ATC = if GC != 0.0 && (if ATB != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if ATC != 0.0 {
            } else {
            }
            let ATE = Q * ATD;
            let ATG = Q * ATF;
            let ATK = Q * ATH;
            let ATO = Q * ATL;
            if ASP != 0.0 {
            } else {
            }
            let ATP = if HO == AC { 1.0 } else { 0.0 };
            let AUJ;
            let AUK;
            let AUL;
            let AUM;
            if ATP != 0.0 {
                AUJ = A;
                AUK = A;
                AUL = A;
                AUM = A;
            } else {
                let ATU = ASZ * (B / ATQ);
                let ATV = ASZ * (B / ATS);
                AUJ = B;
                AUK = ATU;
                AUL = B;
                AUM = ATV;
            }
            if ATC != 0.0 {
            } else {
            }
            let ATW = if LO == A { 1.0 } else { 0.0 };
            let AUN;
            let AUO;
            if ATW != 0.0 {
                AUN = A;
                AUO = A;
            } else {
                let ATY = ASZ * ATX;
                AUN = B;
                AUO = ATY;
            }
            let AUP;
            let AUR;
            let AUT;
            let AUV;
            let AUX;
            let AUZ;
            let AVB;
            let AVD;
            if AQG != 0.0 {
                let AUQ;
                let AUS;
                let AUU;
                let AUW;
                let AUY;
                let AVA;
                let AVC;
                let AVE;
                if ASP != 0.0 {
                    let AUC = 3.20438e-19f64 * ((ATG + ATK).abs());
                    let AUD = 3.20438e-19f64 * ((ATE + ATO).abs());
                    AUQ = B;
                    AUS = AUC;
                    AUU = B;
                    AUW = AUD;
                    AUY = A;
                    AVA = A;
                    AVC = A;
                    AVE = A;
                } else {
                    let AUE = 3.20438e-19f64 * ((ATG + ATK).abs());
                    let AUF = 3.20438e-19f64 * ((ATE + ATO).abs());
                    AUQ = A;
                    AUS = A;
                    AUU = A;
                    AUW = A;
                    AUY = B;
                    AVA = AUE;
                    AVC = B;
                    AVE = AUF;
                }
                AUP = AUQ;
                AUR = AUS;
                AUT = AUU;
                AUV = AUW;
                AUX = AUY;
                AUZ = AVA;
                AVB = AVC;
                AVD = AVE;
            } else {
                AUP = A;
                AUR = A;
                AUT = A;
                AUV = A;
                AUX = A;
                AUZ = A;
                AVB = A;
                AVD = A;
            }
            let AVF;
            let AVG;
            let AVH;
            let AVI;
            if APM != 0.0 {
                let AUG = 3.20438e-19f64 * (AQE.abs());
                let AUH = 3.20438e-19f64 * (AQF.abs());
                AVF = B;
                AVG = AUG;
                AVH = B;
                AVI = AUH;
            } else {
                AVF = A;
                AVG = A;
                AVH = A;
                AVI = A;
            }
            if KC != 0.0 {
                let AUI = if HO != AC { 1.0 } else { 0.0 };
                if AUI != 0.0 {
                } else {
                }
            } else {
            }
            if ASP != 0.0 {
            } else {
            }
            if ASP != 0.0 {
            } else {
            }
            if ASP != 0.0 {
            } else {
            }
        if AUJ == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AUK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AUL == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AUM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AUN == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AUO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ATZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(AUB);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ATA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AUP == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AUR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AUT == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AUV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AUX == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AUZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AVB == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AVD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AVF == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AVG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AVH == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AVI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
