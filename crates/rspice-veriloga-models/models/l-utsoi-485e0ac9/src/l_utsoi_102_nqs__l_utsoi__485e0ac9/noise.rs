#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 10] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GP_RGATE", label: Some("rgate"), kind: GeneratedNoiseKind::White, equation: 12, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "gp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RSOURCE", label: Some("rsource"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RDRAIN", label: Some("rdrain"), kind: GeneratedNoiseKind::White, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BP_RWELL", label: Some("rwell"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NSIG_GND_IGN_G", label: Some("ign_g"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "NSIG", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_THERMAL_IDS", label: Some("thermal_ids"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_SI_SHOT_IGS", label: Some("shot_igs"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_DI_SHOT_IGD", label: Some("shot_igd"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_SHOT_IDS", label: Some("shot_ids"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13])];
            let A = 0e0f64;
            let D = 1e0f64;
            let F = 5e-1f64;
            let J = 1e1f64;
            let K = 8.617332384961e-5f64;
            let M = 6e2f64;
            let O = 1e-2f64;
            let R = 1e-3f64;
            let T = parameters[0];
            let V = parameters[172];
            let W = parameters[443];
            let Y = parameters[5];
            let AG = parameters[30];
            let AH = parameters[41];
            let AI = parameters[42];
            let AJ = parameters[43];
            let AK = parameters[44];
            let AL = parameters[45];
            let AN = -1e0f64;
            let AO = 1e19f64;
            let AP = 1e6f64;
            let AR = parameters[46];
            let AT = -1e0f64;
            let AU = 1e16f64;
            let AV = 1e21f64;
            let AX = parameters[47];
            let AY = parameters[48];
            let BB = parameters[51];
            let BC = parameters[52];
            let BD = parameters[53];
            let BF = parameters[55];
            let BG = parameters[56];
            let BH = parameters[57];
            let BI = parameters[58];
            let BL = parameters[60];
            let BM = parameters[61];
            let BN = parameters[62];
            let BO = parameters[63];
            let BQ = parameters[64];
            let BR = parameters[65];
            let BS = parameters[66];
            let BT = parameters[67];
            let BU = parameters[68];
            let BW = parameters[70];
            let BX = parameters[71];
            let BY = parameters[72];
            let BZ = parameters[73];
            let CA = parameters[74];
            let CB = parameters[75];
            let CC = parameters[76];
            let CD = parameters[77];
            let CE = parameters[78];
            let CF = parameters[79];
            let CG = parameters[80];
            let CH = parameters[81];
            let CI = parameters[82];
            let CJ = parameters[83];
            let CK = parameters[84];
            let CL = parameters[85];
            let CM = parameters[86];
            let CN = parameters[87];
            let CO = parameters[88];
            let CP = parameters[89];
            let CQ = parameters[90];
            let CR = parameters[91];
            let CS = parameters[92];
            let CT = parameters[93];
            let CU = parameters[94];
            let CV = parameters[95];
            let CW = parameters[96];
            let CX = parameters[97];
            let CY = parameters[98];
            let CZ = parameters[99];
            let DA = parameters[100];
            let DB = parameters[101];
            let DC = parameters[102];
            let DD = parameters[103];
            let DE = parameters[104];
            let DF = parameters[105];
            let DG = parameters[106];
            let DH = parameters[120];
            let DI = parameters[121];
            let DJ = parameters[107];
            let DK = parameters[108];
            let DL = parameters[109];
            let DM = parameters[123];
            let DN = parameters[110];
            let DO = parameters[111];
            let DP = parameters[112];
            let DQ = parameters[122];
            let DR = parameters[113];
            let DS = parameters[114];
            let DT = parameters[115];
            let DU = parameters[116];
            let DV = parameters[117];
            let DW = parameters[118];
            let DX = parameters[119];
            let DY = parameters[124];
            let DZ = parameters[125];
            let EA = parameters[126];
            let EB = parameters[127];
            let EC = parameters[128];
            let ED = parameters[129];
            let EE = parameters[130];
            let EF = parameters[131];
            let EG = parameters[132];
            let EH = parameters[133];
            let EI = parameters[147];
            let EJ = parameters[148];
            let EK = parameters[149];
            let EL = parameters[150];
            let EM = parameters[134];
            let EN = parameters[135];
            let EO = parameters[136];
            let EP = parameters[137];
            let EQ = parameters[138];
            let ER = parameters[139];
            let ES = parameters[140];
            let EU = parameters[142];
            let EW = parameters[144];
            let EX = parameters[145];
            let EY = parameters[146];
            let EZ = parameters[151];
            let FA = parameters[152];
            let FC = parameters[154];
            let FD = parameters[155];
            let FE = parameters[11];
            let FH = parameters[156];
            let FJ = parameters[157];
            let FL = parameters[158];
            let FP = parameters[159];
            let FT = parameters[160];
            let FV = parameters[161];
            let FX = parameters[162];
            let FY = parameters[163];
            let FZ = parameters[164];
            let GA = parameters[165];
            let GB = parameters[166];
            let GC = parameters[167];
            let GD = parameters[168];
            let GE = parameters[169];
            let GF = parameters[173];
            let GG = parameters[175];
            let GH = parameters[176];
            let GI = parameters[177];
            let GJ = parameters[178];
            let GK = parameters[179];
            let GL = parameters[180];
            let GM = parameters[181];
            let GN = parameters[182];
            let GO = parameters[187];
            let GP = parameters[188];
            let GQ = parameters[189];
            let GR = parameters[190];
            let GS = parameters[29];
            let GT = 1e-9f64;
            let GW = 1e-6f64;
            let GX = parameters[20];
            let HC = 2e0f64;
            let HT = parameters[201];
            let HU = parameters[202];
            let HV = parameters[203];
            let HW = parameters[204];
            let HX = parameters[205];
            let HZ = -1e0f64;
            let IB = parameters[206];
            let ID = -1e0f64;
            let IF = parameters[207];
            let IG = parameters[208];
            let IJ = parameters[212];
            let IK = parameters[213];
            let IL = parameters[214];
            let IM = parameters[215];
            let IO = parameters[211];
            let IP = parameters[216];
            let IQ = parameters[217];
            let IS = parameters[218];
            let IT = parameters[219];
            let IX = parameters[226];
            let IY = parameters[227];
            let IZ = 1.04479e-10f64;
            let JA = 1.43438e-10f64;
            let JB = 3.45313e-11f64;
            let JC = 4e-10f64;
            let JE = parameters[228];
            let JF = parameters[229];
            let JG = parameters[230];
            let JH = 5e0f64;
            let JJ = parameters[231];
            let JM = parameters[233];
            let JO = parameters[236];
            let JQ = parameters[237];
            let JT = parameters[235];
            let JW = parameters[238];
            let JZ = parameters[240];
            let KB = parameters[243];
            let KE = 8e1f64;
            let KH = 1.80485e-35f64;
            let KJ = 3.333333333333e-1f64;
            let KS = parameters[244];
            let KV = 1e-10f64;
            let KX = parameters[254];
            let LB = parameters[264];
            let LC = parameters[265];
            let LE = parameters[270];
            let LF = parameters[271];
            let LG = parameters[272];
            let LH = parameters[273];
            let LI = parameters[274];
            let LJ = parameters[275];
            let LK = parameters[276];
            let LL = parameters[277];
            let LN = parameters[283];
            let LO = parameters[284];
            let LP = parameters[285];
            let LR = parameters[288];
            let LS = parameters[289];
            let LT = parameters[290];
            let LU = parameters[291];
            let LV = parameters[292];
            let LW = parameters[293];
            let LX = parameters[294];
            let LY = parameters[295];
            let LZ = parameters[296];
            let MA = parameters[297];
            let ME = parameters[302];
            let MF = parameters[303];
            let MG = parameters[304];
            let MH = parameters[305];
            let MI = parameters[306];
            let MJ = parameters[307];
            let MK = parameters[308];
            let ML = 1.6e1f64;
            let MN = parameters[309];
            let MO = parameters[310];
            let MP = parameters[313];
            let MQ = parameters[311];
            let MR = parameters[312];
            let MU = parameters[319];
            let MV = parameters[320];
            let MW = parameters[321];
            let MX = parameters[322];
            let NF = parameters[328];
            let NG = parameters[342];
            let NH = parameters[329];
            let NI = parameters[330];
            let NJ = parameters[331];
            let NK = parameters[341];
            let NL = parameters[332];
            let NM = parameters[333];
            let NN = parameters[334];
            let NP = parameters[336];
            let NQ = parameters[337];
            let NR = parameters[338];
            let NU = parameters[347];
            let NV = parameters[348];
            let NW = parameters[349];
            let NX = parameters[350];
            let NY = parameters[351];
            let NZ = parameters[352];
            let OD = parameters[391];
            let OE = parameters[392];
            let OG = parameters[359];
            let OI = parameters[365];
            let OK = parameters[370];
            let OL = parameters[371];
            let OQ = parameters[380];
            let OR = parameters[382];
            let OY = parameters[400];
            let PB = parameters[401];
            let PD = parameters[402];
            let PF = parameters[403];
            let PH = parameters[406];
            let PJ = parameters[407];
            let PL = parameters[404];
            let PN = parameters[405];
            let PY = parameters[408];
            let QA = parameters[409];
            let QF = parameters[410];
            let QH = parameters[411];
            let QJ = parameters[412];
            let QQ = parameters[413];
            let QS = parameters[414];
            let QU = parameters[415];
            let RC = parameters[416];
            let RE = parameters[417];
            let RG = parameters[418];
            let RI = parameters[419];
            let RK = parameters[420];
            let RT = parameters[421];
            let RV = parameters[422];
            let RX = parameters[423];
            let RZ = parameters[424];
            let SB = parameters[425];
            let SJ = parameters[426];
            let SL = parameters[427];
            let SN = parameters[428];
            let SP = parameters[429];
            let SR = parameters[430];
            let TC = parameters[435];
            let TD = parameters[436];
            let TH = parameters[28];
            let TP = 5.54062e34f64;
            let TY = parameters[447];
            let TZ = parameters[451];
            let UE = parameters[458];
            let UF = parameters[459];
            let UG = parameters[460];
            let UH = parameters[37];
            let UL = parameters[7];
            let UR = parameters[461];
            let US = parameters[26];
            let UT = parameters[27];
            let VF = parameters[462];
            let VI = parameters[463];
            let VK = parameters[464];
            let VP = parameters[465];
            let VV = 1e-20f64;
            let WC = parameters[466];
            let WV = parameters[482];
            let XB = parameters[481];
            let YQ = parameters[488];
            let ZU = 1.17e0f64;
            let ZV = 4.73e-4f64;
            let ZW = 6.36e2f64;
            let ZZ = 7.44e-1f64;
            let AAA = 4.774e-4f64;
            let AAB = 2.35e2f64;
            let AAD = 4e-1f64;
            let AAI = 5e-2f64;
            let AAM = 1.602176565e-19f64;
            let AAV = parameters[13];
            let ABE = 3.3333333333e-3f64;
            let ABG = 4.05e25f64;
            let ACD = 6.931471805599e-1f64;
            let ADK = 1.4142135623731e0f64;
            let ADN = 1e-5f64;
            let AEK = 1.5e1f64;
            let AEL = 2.97e3f64;
            let AER = parameters[14];
            let AEU = 1.27520989e0f64;
            let AEX = 1.5412087e0f64;
            let AFE = parameters[34];
            let AFZ = parameters[35];
            let AGX = 1e-8f64;
            let AHM = 3.75e-1f64;
            let AJF = 4e0f64;
            let ALG = 2.5e-1f64;
            let APL = node_potentials[9];
            let APM = node_potentials[6];
            let APO = node_potentials[7];
            let APQ = node_potentials[8];
            let AQC = -1e0f64;
            let ARL = 1.5e0f64;
            let ASH = 1.666666666667e-1f64;
            let ASN = 1.25e0f64;
            let ASP = 6e0f64;
            let ASR = 6.4e1f64;
            let ATI = 8e0f64;
            let ATJ = 1.2e1f64;
            let ATP = 7.32464877560822e-1f64;
            let ATY = 3e0f64;
            let AUH = 1e-40f64;
            let BAS = 5e-3f64;
            let BBM = 1.66666666667e-2f64;
            let BBN = 2.38095238095e-2f64;
            let BBP = 2.5e-2f64;
            let BBS = 3.33333333333e-2f64;
            let BBU = 3.57142857143e-2f64;
            let BBX = 5.5555555556e-3f64;
            let BBY = 7.14285714286e-2f64;
            let BBZ = 4.20875420875421e-2f64;
            let BCC = 7.5e-2f64;
            let BCQ = 3.96825396825397e-2f64;
            let BCT = 1.01e0f64;
            let BEL = 1e-200f64;
            let BEQ = 6.5345483024e-2f64;
            let BES = 3.9478417604e1f64;
            let BET = 8.5797362674e0f64;
            let BFB = 2.3025850929941e0f64;
            let BSF = 1e-80f64;
            let BSU = 9e-1f64;
            let BWH = 2e-1f64;
            let BWU = 1e-12f64;
            let BXK = 7e-3f64;
            let BYL = 1e2f64;
            let BZN = 1e-14f64;
            let BZT = 1.48148148148e-1f64;
            let BZZ = 9.4e-1f64;
            let CAH = 3.6e1f64;
            let CBD = 2.666666666667e0f64;
            let CZF = 6e-1f64;
            let CZH = 6e1f64;
            let DAS = 1e-30f64;
            let DCH = 7.324648775608221e-1f64;
            let DML = 1e-4f64;
            let GLC = 9e0f64;
            let GMP = parameters[31];
            let GQR = 3.8e1f64;
            let B = 2.7315e2f64 + parameters[15];
            let C = if (temperature + parameters[36]) <= 1e3f64 { (temperature + parameters[36]) } else { 1e3f64 };
            let E = if parameters[10] == D { 1.0 } else { 0.0 };
            let Z;
            let AXZ;
            if E != 0.0 {
                let G = parameters[17] + (parameters[18] * C);
                let H = C - G;
                let I = F * ((C + G) + (((H * H) + parameters[19]).sqrt()));
                let L = J / (I * K);
                let N = L - M;
                let P = F * ((L + M) + (((N * N) + O).sqrt()));
                Z = I;
                AXZ = P;
            } else {
                let Q = C - D;
                let S = F * ((C + D) + (((Q * Q) + R).sqrt()));
                Z = S;
                AXZ = M;
            }
            let U = if T == A { 1.0 } else { 0.0 };
            let X = if (if U != 0.0 && (if V > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if T > A { 1.0 } else { 0.0 }) != 0.0 && (if W > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AMI = if X != 0.0 {
                Y
            } else {
                A
            };
            if D != 0.0 {
            } else {
            }
            let AA = Z * Z;
            let AB = Z - B;
            let AC = Z / B;
            let AD = B / Z;
            let AE = Z * K;
            let AF = D / AE;
            let ZG;
            let ZH;
            let ZI;
            let ZJ;
            let ZK;
            let ZL;
            let ZM;
            let ZN;
            let ZO;
            let ZP;
            let ZQ;
            let ZR;
            let AAN;
            let AAO;
            let AAQ;
            let AAU;
            let AAY;
            let ABM;
            let ABR;
            let ACF;
            let ACI;
            let ACK;
            let ACO;
            let ACS;
            let ACY;
            let ADE;
            let ADG;
            let ADS;
            let ADV;
            let AEA;
            let AEH;
            let AEZ;
            let AFH;
            let AFP;
            let AFY;
            let AGB;
            let AGF;
            let AGJ;
            let AGK;
            let AGL;
            let AGM;
            let AGO;
            let AGP;
            let AGR;
            let AGS;
            let AGU;
            let AGV;
            let AGZ;
            let AHB;
            let AHE;
            let AHJ;
            let AHK;
            let AHN;
            let AHP;
            let AHU;
            let AHW;
            let AIA;
            let AIH;
            let AIJ;
            let AIM;
            let AIQ;
            let AIU;
            let AIW;
            let AJB;
            let AJD;
            let AJG;
            let AJI;
            let AJK;
            let AJM;
            let AJO;
            let AJQ;
            let AJS;
            let AJU;
            let AJX;
            let AKC;
            let AKI;
            let AKL;
            let AKN;
            let AKO;
            let AKR;
            let AKV;
            let AKX;
            let AKZ;
            let ALB;
            let ALC;
            let ALE;
            let ALH;
            let ALK;
            let ALO;
            let ALP;
            let ALS;
            let ALU;
            let ALW;
            let ALZ;
            let AMC;
            let AMF;
            let AQO;
            let AQP;
            let AQT;
            let AXH;
            let AXP;
            let AXQ;
            let AXV;
            let AYC;
            let BWE;
            let BWL;
            let BWM;
            let BWQ;
            let BWT;
            let BWY;
            let BXD;
            let BYN;
            let BYS;
            let CYE;
            let CYJ;
            let CYL;
            let DBY;
            let DGY;
            let DJO;
            let DNH;
            let DOW;
            let DOX;
            let DYN;
            let DYZ;
            let DZQ;
            let DZS;
            let DZU;
            let DZX;
            let DZY;
            let EAB;
            let EAF;
            let EBX;
            let ECJ;
            let EDD;
            let EDG;
            let EDM;
            let GKE;
            let GMD;
            let GMF;
            let GMH;
            let GMN;
            let GMQ;
            let GPH;
            let GPI;
            let GPJ;
            let GPL;
            let GPM;
            let GPW;
            if U != 0.0 {
                let AM = if AL < A { 1.0 } else { 0.0 };
                let AAR = if AM != 0.0 {
                    AN
                } else {
                    D
                };
                let AQ = (if (AL.abs()) <= AO { (AL.abs()) } else { AO }) * AP;
                let AS = if AR < A { 1.0 } else { 0.0 };
                let ADW = if AS != 0.0 {
                    AT
                } else {
                    D
                };
                let AW = (if (if (AR.abs()) >= AU { (AR.abs()) } else { AU }) <= AV { (if (AR.abs()) >= AU { (AR.abs()) } else { AU }) } else { AV }) * AP;
                let AZ = parameters[49] * AP;
                let BA = parameters[50] * AP;
                let BE = parameters[54] * AP;
                let BJ = ((BI * BH) * AK) / AH;
                let BK = parameters[59] * AP;
                let BP = ((BO * BN) * AK) / AH;
                let BV = parameters[69] * BU;
                let ET = ((parameters[141] * ES) * AK) / AH;
                let EV = ((parameters[143] * EU) * AK) / AH;
                let FB = parameters[153] * AP;
                let FF = if FE > A { 1.0 } else { 0.0 };
                let ACT;
                let ACZ;
                let AEB;
                let AFQ;
                let AHQ;
                let AIB;
                let EDE;
                let EDH;
                let EDN;
                if FF != 0.0 {
                    let FG = if (if parameter_given[156] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let AFR = if FG != 0.0 {
                        FH
                    } else {
                        BB
                    };
                    let FI = if (if parameter_given[157] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let AEC = if FI != 0.0 {
                        FJ
                    } else {
                        BC
                    };
                    let FK = if (if parameter_given[158] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let FM = if FK != 0.0 {
                        FL
                    } else {
                        BH
                    };
                    let FN = ((BI * FM) * AK) / AH;
                    let FO = if (if parameter_given[159] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let FQ = if FO != 0.0 {
                        FP
                    } else {
                        BN
                    };
                    let FR = ((BO * FQ) * AK) / AH;
                    let FS = if (if parameter_given[160] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let AIC = if FS != 0.0 {
                        FT
                    } else {
                        CT
                    };
                    let FU = if (if parameter_given[161] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let AHR = if FU != 0.0 {
                        FV
                    } else {
                        CX
                    };
                    let FW = if (if parameter_given[162] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let EDO = if FW != 0.0 {
                        FX
                    } else {
                        CY
                    };
                    ACT = FQ;
                    ACZ = FR;
                    AEB = AEC;
                    AFQ = AFR;
                    AHQ = AHR;
                    AIB = AIC;
                    EDE = FM;
                    EDH = FN;
                    EDN = EDO;
                } else {
                    ACT = BN;
                    ACZ = BP;
                    AEB = BC;
                    AFQ = BB;
                    AHQ = CX;
                    AIB = CT;
                    EDE = BH;
                    EDH = BJ;
                    EDN = CY;
                }
                ZG = AZ;
                ZH = DF;
                ZI = DH;
                ZJ = DJ;
                ZK = DY;
                ZL = EA;
                ZM = EC;
                ZN = EE;
                ZO = EG;
                ZP = FY;
                ZQ = GD;
                ZR = AJ;
                AAN = AQ;
                AAO = AI;
                AAQ = AAR;
                AAU = AH;
                AAY = AK;
                ABM = BM;
                ABR = AX;
                ACF = BK;
                ACI = BQ;
                ACK = BN;
                ACO = BP;
                ACS = ACT;
                ACY = ACZ;
                ADE = BR;
                ADG = AW;
                ADS = BC;
                ADV = ADW;
                AEA = AEB;
                AEH = BE;
                AEZ = BD;
                AFH = BB;
                AFP = AFQ;
                AFY = BW;
                AGB = BU;
                AGF = BV;
                AGJ = CG;
                AGK = CF;
                AGL = CI;
                AGM = CH;
                AGO = CA;
                AGP = BX;
                AGR = CC;
                AGS = CB;
                AGU = CL;
                AGV = CJ;
                AGZ = CD;
                AHB = CE;
                AHE = CM;
                AHJ = CP;
                AHK = CN;
                AHN = CX;
                AHP = AHQ;
                AHU = CU;
                AHW = CT;
                AIA = AIB;
                AIH = CZ;
                AIJ = DL;
                AIM = DE;
                AIQ = DG;
                AIU = DK;
                AIW = DM;
                AJB = DI;
                AJD = DW;
                AJG = AY;
                AJI = DO;
                AJK = DN;
                AJM = DR;
                AJO = DP;
                AJQ = DT;
                AJS = DS;
                AJU = DD;
                AJX = DX;
                AKC = DZ;
                AKI = ED;
                AKL = EB;
                AKN = EJ;
                AKO = EK;
                AKR = EM;
                AKV = EP;
                AKX = EN;
                AKZ = EO;
                ALB = EY;
                ALC = EX;
                ALE = EZ;
                ALH = FB;
                ALK = FC;
                ALO = GF;
                ALP = V;
                ALS = GG;
                ALU = GH;
                ALW = GO;
                ALZ = GP;
                AMC = GQ;
                AMF = GR;
                AQO = BH;
                AQP = BJ;
                AQT = CY;
                AXH = BL;
                AXP = BS;
                AXQ = BT;
                AXV = BF;
                AYC = BG;
                BWE = CK;
                BWL = BY;
                BWM = BZ;
                BWQ = CQ;
                BWT = CR;
                BWY = CS;
                BXD = CO;
                BYN = CV;
                BYS = CW;
                CYE = DA;
                CYJ = DB;
                CYL = DC;
                DBY = GC;
                DGY = BA;
                DJO = FZ;
                DNH = DQ;
                DOW = DU;
                DOX = DV;
                DYN = EF;
                DYZ = EH;
                DZQ = ES;
                DZS = ET;
                DZU = EW;
                DZX = EU;
                DZY = EV;
                EAB = EQ;
                EAF = ER;
                EBX = EL;
                ECJ = EI;
                EDD = EDE;
                EDG = EDH;
                EDM = EDN;
                GKE = FD;
                GMD = GE;
                GMF = GA;
                GMH = GB;
                GMN = FA;
                GMQ = AG;
                GPH = GI;
                GPI = GJ;
                GPJ = GK;
                GPL = GL;
                GPM = GM;
                GPW = GN;
            } else {
                let GU = if (parameters[21] * (D / GS)) >= GT { (parameters[21] * (D / GS)) } else { GT };
                let GV = AG * GS;
                let GY = GW / GX;
                let GZ = GW / GU;
                let HA = (parameters[195] * (D + (parameters[197] * GZ))) * (D + (parameters[196] * GY));
                let HB = GX + ((parameters[191] * (D + (parameters[192] * GY))) * (D + (parameters[193] * GZ)));
                let HD = HB - (HC * parameters[194]);
                let HE = if HD >= GT { HD } else { GT };
                let HF = GU + HA;
                let HG = HF - (HC * parameters[198]);
                let HH = if HG >= GT { HG } else { GT };
                let HI = if (HD + parameters[199]) >= GT { (HD + parameters[199]) } else { GT };
                let HJ = if (HG + parameters[200]) >= GT { (HG + parameters[200]) } else { GT };
                let HK = GW / HE;
                let HL = GW / HH;
                let HM = HK * HL;
                let HN = if HB >= GT { HB } else { GT };
                let HO = HN / GW;
                let HP = if HF >= GT { HF } else { GT };
                let HQ = HP / GW;
                let HR = if (HN + parameters[499]) >= GT { (HN + parameters[499]) } else { GT };
                let HS = if (parameters[38] - (F * HA)) >= GT { (parameters[38] - (F * HA)) } else { GT };
                let HY = if HX < A { 1.0 } else { 0.0 };
                let AAS = if HY != 0.0 {
                    HZ
                } else {
                    D
                };
                let IA = (if (HX.abs()) <= AO { (HX.abs()) } else { AO }) * AP;
                let IC = if IB < A { 1.0 } else { 0.0 };
                let ADX = if IC != 0.0 {
                    ID
                } else {
                    D
                };
                let IE = (if (if (IB.abs()) >= AU { (IB.abs()) } else { AU }) <= AV { (if (IB.abs()) >= AU { (IB.abs()) } else { AU }) } else { AV }) * AP;
                let IH = parameters[209] * AP;
                let II = parameters[210] * AP;
                let IN = (IJ * (HK.powf(IK))) / (D + (IL * (HK.powf(IM))));
                let IR = ((IO + IN) + (IP * HL)) + (IQ * HM);
                let IU = IS + (((IT * HW) / HT) * IN);
                let IV = ((parameters[220] * (D + (parameters[221] * HK))) * (D + (parameters[222] * HL))) * (D + (parameters[223] * HM));
                let IW = if (if ((parameters[224] * (D + (parameters[225] * HK))) * AP) >= 1e25f64 { ((parameters[224] * (D + (parameters[225] * HK))) * AP) } else { 1e25f64 }) <= 1e28f64 { (if ((parameters[224] * (D + (parameters[225] * HK))) * AP) >= 1e25f64 { ((parameters[224] * (D + (parameters[225] * HK))) * AP) } else { 1e25f64 }) } else { 1e28f64 };
                let JD = ((((((IZ * (D - HV)) + (JA * HV)) / JB) * HU) * (HT + JC)).sqrt()) / HE;
                let JI = if (if (((JE * HC) * (JD.powf(JF))) * (D + (JG * HL))) >= A { (((JE * HC) * (JD.powf(JF))) * (D + (JG * HL))) } else { A }) <= JH { (if (((JE * HC) * (JD.powf(JF))) * (D + (JG * HL))) >= A { (((JE * HC) * (JD.powf(JF))) * (D + (JG * HL))) } else { A }) } else { JH };
                let JK = ((JJ * JI) * HW) / HT;
                let JL = parameters[232] * AP;
                let JN = if (if (parameters[234] * HL) >= -1e0f64 { (parameters[234] * HL) } else { -1e0f64 }) <= D { (if (parameters[234] * HL) >= -1e0f64 { (parameters[234] * HL) } else { -1e0f64 }) } else { D };
                let JP = JD.powf(JO);
                let JR = D + (JQ * HL);
                let JS = JP * JR;
                let JU = JT * JS;
                let JV = if JU >= A { JU } else { A };
                let JX = ((JW * JV) * HW) / HT;
                let JY = parameters[239] * JS;
                let KA = (parameters[241] * HK) / (if (D + (parameters[242] * HL)) >= R { (D + (parameters[242] * HL)) } else { R });
                let KC = -HE;
                let KD = KC / (parameters[247] * (if (D + (parameters[248] * HL)) >= R { (D + (parameters[248] * HL)) } else { R }));
                let KF = if KD > -8e1f64 { 1.0 } else { 0.0 };
                let KQ = if KF != 0.0 {
                    let KG = KD.exp();
                    KG
                } else {
                    let KI = (-KD) - KE;
                    let KK = KH / (D + (KI * (D + ((F * KI) * (D + (KI * KJ))))));
                    KK
                };
                let KL = KC / parameters[250];
                let KM = if KL > -8e1f64 { 1.0 } else { 0.0 };
                let KR = if KM != 0.0 {
                    let KN = KL.exp();
                    KN
                } else {
                    let KO = (-KL) - KE;
                    let KP = KH / (D + (KO * (D + ((F * KO) * (D + (KO * KJ))))));
                    KP
                };
                let KT = (KS / (if ((D + (((parameters[245] * (D + (parameters[246] * HL))) * (KQ - D)) / KD)) + ((parameters[249] * (KR - D)) / KL)) >= GW { ((D + (((parameters[245] * (D + (parameters[246] * HL))) * (KQ - D)) / KD)) + ((parameters[249] * (KR - D)) / KL)) } else { GW })) * (if ((D + (parameters[251] * HL)) + ((parameters[252] * HL) * ((D + (HH / parameters[253])).ln()))) >= GW { ((D + (parameters[251] * HL)) + ((parameters[252] * HL) * ((D + (HH / parameters[253])).ln()))) } else { GW });
                let KU = (KT * HH) / HE;
                let KW = if KU >= KV { KU } else { KV };
                let KY = KX * KW;
                let KZ = ((parameters[255] * (D + (parameters[256] * HK))) * (D + (parameters[257] * HL))) * (D + (parameters[258] * HM));
                let LA = if (((parameters[259] + (parameters[260] * (HK.powf(parameters[261])))) * (D + (parameters[262] * HL))) * (D + (parameters[263] * HM))) >= A { (((parameters[259] + (parameters[260] * (HK.powf(parameters[261])))) * (D + (parameters[262] * HL))) * (D + (parameters[263] * HM))) } else { A };
                let LD = ((parameters[266] * (D + (parameters[267] * HK))) * (D + (parameters[268] * HL))) * (D + (parameters[269] * HM));
                let LM = ((parameters[278] + (parameters[279] * (HK.powf(parameters[280])))) * (D + (parameters[281] * HL))) * (D + (parameters[282] * HM));
                let LQ = if ((parameters[286] * HL) * (D + (parameters[287] * HL))) >= A { ((parameters[286] * HL) * (D + (parameters[287] * HL))) } else { A };
                let MB = ((KT * (LW + (LX * (HK.powf(LY))))) * (D + (LZ * HL))) * (D + (MA * HM));
                let MC = if MB >= A { MB } else { A };
                let MD = ((parameters[298] * (D + (parameters[299] * HK))) * (D + (parameters[300] * HL))) * (D + (parameters[301] * HM));
                let MM = if (if (MG / (D + ((MH * (HK.powf(MI))) / (D + (MJ * (HK.powf(MK))))))) >= D { (MG / (D + ((MH * (HK.powf(MI))) / (D + (MJ * (HK.powf(MK))))))) } else { D }) <= ML { (if (MG / (D + ((MH * (HK.powf(MI))) / (D + (MJ * (HK.powf(MK))))))) >= D { (MG / (D + ((MH * (HK.powf(MI))) / (D + (MJ * (HK.powf(MK))))))) } else { D }) } else { ML };
                let MS = if (((MN * (HK.powf(MO))) * (D + (MP * HL))) / (D + (MQ * (HK.powf(MR))))) >= A { (((MN * (HK.powf(MO))) * (D + (MP * HL))) / (D + (MQ * (HK.powf(MR))))) } else { A };
                let MT = if (((parameters[314] * (HK.powf(parameters[315]))) * (D + (parameters[318] * HL))) / (D + (parameters[316] * (HK.powf(parameters[317]))))) >= A { (((parameters[314] * (HK.powf(parameters[315]))) * (D + (parameters[318] * HL))) / (D + (parameters[316] * (HK.powf(parameters[317]))))) } else { A };
                let MY = parameters[323] / HM;
                let MZ = parameters[324] / HL;
                let NA = parameters[325] / HL;
                let NB = parameters[339] / HL;
                let NC = parameters[340] / HL;
                let ND = parameters[326] / HL;
                let NE = parameters[327] / HL;
                let NO = parameters[335] * HK;
                let NS = if (parameters[343] + (parameters[345] / HL)) >= A { (parameters[343] + (parameters[345] / HL)) } else { A };
                let NT = if (parameters[344] + (parameters[346] / HL)) >= A { (parameters[344] + (parameters[346] / HL)) } else { A };
                let OA = parameters[353] + (parameters[355] * HK);
                let OB = parameters[354] + (parameters[356] * HK);
                let OC = if ((parameters[388] * (D + (parameters[389] * HK))) * (D + (parameters[390] * HL))) >= A { ((parameters[388] * (D + (parameters[389] * HK))) * (D + (parameters[390] * HL))) } else { A };
                let OF = if ((parameters[393] * (D + (parameters[394] * HK))) * (D + (parameters[395] * HL))) >= A { ((parameters[393] * (D + (parameters[394] * HK))) * (D + (parameters[395] * HL))) } else { A };
                let OH = ((parameters[360] + (parameters[361] * (HK.powf(parameters[362])))) + (parameters[363] * HL)) + (parameters[364] * HM);
                let OJ = ((parameters[366] * (D + (parameters[367] * HK))) * (D + (parameters[368] * HL))) * (D + (parameters[369] * HM));
                let OM = if (if (((parameters[372] * HC) * (JD.powf(parameters[373]))) * (D + (parameters[374] * HL))) >= A { (((parameters[372] * HC) * (JD.powf(parameters[373]))) * (D + (parameters[374] * HL))) } else { A }) <= JH { (if (((parameters[372] * HC) * (JD.powf(parameters[373]))) * (D + (parameters[374] * HL))) >= A { (((parameters[372] * HC) * (JD.powf(parameters[373]))) * (D + (parameters[374] * HL))) } else { A }) } else { JH };
                let ON = ((parameters[375] * OM) * HW) / HT;
                let OO = if (parameters[376] * ((JD.powf(parameters[377])) * (D + (parameters[378] * HL)))) >= A { (parameters[376] * ((JD.powf(parameters[377])) * (D + (parameters[378] * HL)))) } else { A };
                let OP = ((parameters[379] * OO) * HW) / HT;
                let OS = ((KS * ((HC * parameters[357]) + (parameters[358] * HH))) / ((if (D + (((parameters[381] * OR) / HE) * (D - ((KC / OR).exp())))) >= 1e-15f64 { (D + (((parameters[381] * OR) / HE) * (D - ((KC / OR).exp())))) } else { 1e-15f64 }) * HE)) * (D + (parameters[383] * HL));
                let OT = ((parameters[384] + (parameters[385] * HK)) + (parameters[386] * HL)) + ((parameters[387] * HK) * HL);
                let OU = HJ * HI;
                let OV = if (parameters[396] + (parameters[397] * HO)) >= A { (parameters[396] + (parameters[397] * HO)) } else { A };
                let OW = parameters[398] * AP;
                let OX = (parameters[399] * HJ) / GW;
                let OZ = if FE > A { 1.0 } else { 0.0 };
                let WF;
                let WK;
                let WM;
                let WQ;
                let ACW;
                let ADC;
                let AHS;
                let AIF;
                let EDF;
                let EDI;
                let EDP;
                if OZ != 0.0 {
                    let PA = if (if parameter_given[401] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PT = if PA != 0.0 {
                        PB
                    } else {
                        IO
                    };
                    let PC = if (if parameter_given[402] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PO = if PC != 0.0 {
                        PD
                    } else {
                        IJ
                    };
                    let PE = if (if parameter_given[403] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PP = if PE != 0.0 {
                        PF
                    } else {
                        IK
                    };
                    let PG = if (if parameter_given[406] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PU = if PG != 0.0 {
                        PH
                    } else {
                        IP
                    };
                    let PI = if (if parameter_given[407] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PV = if PI != 0.0 {
                        PJ
                    } else {
                        IQ
                    };
                    let PK = if (if parameter_given[404] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PQ = if PK != 0.0 {
                        PL
                    } else {
                        IL
                    };
                    let PM = if (if parameter_given[405] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let PR = if PM != 0.0 {
                        PN
                    } else {
                        IM
                    };
                    let PS = (PO * (HK.powf(PP))) / (D + (PQ * (HK.powf(PR))));
                    let PW = ((PT + PS) + (PU * HL)) + (PV * HM);
                    let PX = if (if parameter_given[408] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QB = if PX != 0.0 {
                        PY
                    } else {
                        IS
                    };
                    let PZ = if (if parameter_given[409] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QC = if PZ != 0.0 {
                        QA
                    } else {
                        IT
                    };
                    let QD = QB + (((QC * HW) / HT) * PS);
                    let QE = if (if parameter_given[410] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QK = if QE != 0.0 {
                        QF
                    } else {
                        JE
                    };
                    let QG = if (if parameter_given[411] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QL = if QG != 0.0 {
                        QH
                    } else {
                        JF
                    };
                    let QI = if (if parameter_given[412] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QM = if QI != 0.0 {
                        QJ
                    } else {
                        JG
                    };
                    let QN = if (if (((QK * HC) * (JD.powf(QL))) * (D + (QM * HL))) >= A { (((QK * HC) * (JD.powf(QL))) * (D + (QM * HL))) } else { A }) <= JH { (if (((QK * HC) * (JD.powf(QL))) * (D + (QM * HL))) >= A { (((QK * HC) * (JD.powf(QL))) * (D + (QM * HL))) } else { A }) } else { JH };
                    let QO = ((JJ * QN) * HW) / HT;
                    let QP = if (if parameter_given[413] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QX = if QP != 0.0 {
                        QQ
                    } else {
                        JT
                    };
                    let QR = if (if parameter_given[414] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QV = if QR != 0.0 {
                        QS
                    } else {
                        JO
                    };
                    let QT = if (if parameter_given[415] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let QW = if QT != 0.0 {
                        QU
                    } else {
                        JQ
                    };
                    let QY = QX * ((JD.powf(QV)) * (D + (QW * HL)));
                    let QZ = if QY >= A { QY } else { A };
                    let RA = ((JW * QZ) * HW) / HT;
                    let RB = if (if parameter_given[416] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let RL = if RB != 0.0 {
                        RC
                    } else {
                        LW
                    };
                    let RD = if (if parameter_given[417] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let RM = if RD != 0.0 {
                        RE
                    } else {
                        LX
                    };
                    let RF = if (if parameter_given[418] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let RN = if RF != 0.0 {
                        RG
                    } else {
                        LY
                    };
                    let RH = if (if parameter_given[419] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let RO = if RH != 0.0 {
                        RI
                    } else {
                        LZ
                    };
                    let RJ = if (if parameter_given[420] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let RP = if RJ != 0.0 {
                        RK
                    } else {
                        MA
                    };
                    let RQ = ((KT * (RL + (RM * (HK.powf(RN))))) * (D + (RO * HL))) * (D + (RP * HM));
                    let RR = if RQ >= A { RQ } else { A };
                    let RS = if (if parameter_given[421] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SC = if RS != 0.0 {
                        RT
                    } else {
                        MG
                    };
                    let RU = if (if parameter_given[422] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SD = if RU != 0.0 {
                        RV
                    } else {
                        MH
                    };
                    let RW = if (if parameter_given[423] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SE = if RW != 0.0 {
                        RX
                    } else {
                        MI
                    };
                    let RY = if (if parameter_given[424] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SF = if RY != 0.0 {
                        RZ
                    } else {
                        MJ
                    };
                    let SA = if (if parameter_given[425] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SG = if SA != 0.0 {
                        SB
                    } else {
                        MK
                    };
                    let SH = if (if (SC / (D + ((SD * (HK.powf(SE))) / (D + (SF * (HK.powf(SG))))))) >= D { (SC / (D + ((SD * (HK.powf(SE))) / (D + (SF * (HK.powf(SG))))))) } else { D }) <= ML { (if (SC / (D + ((SD * (HK.powf(SE))) / (D + (SF * (HK.powf(SG))))))) >= D { (SC / (D + ((SD * (HK.powf(SE))) / (D + (SF * (HK.powf(SG))))))) } else { D }) } else { ML };
                    let SI = if (if parameter_given[426] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SS = if SI != 0.0 {
                        SJ
                    } else {
                        MN
                    };
                    let SK = if (if parameter_given[427] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let ST = if SK != 0.0 {
                        SL
                    } else {
                        MO
                    };
                    let SM = if (if parameter_given[428] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SV = if SM != 0.0 {
                        SN
                    } else {
                        MQ
                    };
                    let SO = if (if parameter_given[429] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SW = if SO != 0.0 {
                        SP
                    } else {
                        MR
                    };
                    let SQ = if (if parameter_given[430] { 1.0 } else { 0.0 }) == D { 1.0 } else { 0.0 };
                    let SU = if SQ != 0.0 {
                        SR
                    } else {
                        MP
                    };
                    let SX = if (((SS * (HK.powf(ST))) * (D + (SU * HL))) / (D + (SV * (HK.powf(SW))))) >= A { (((SS * (HK.powf(ST))) * (D + (SU * HL))) / (D + (SV * (HK.powf(SW))))) } else { A };
                    WF = RQ;
                    WK = PW;
                    WM = QD;
                    WQ = QY;
                    ACW = QZ;
                    ADC = RA;
                    AHS = SH;
                    AIF = RR;
                    EDF = QN;
                    EDI = QO;
                    EDP = SX;
                } else {
                    WF = MB;
                    WK = IR;
                    WM = IU;
                    WQ = JU;
                    ACW = JV;
                    ADC = JX;
                    AHS = MM;
                    AIF = MC;
                    EDF = JI;
                    EDI = JK;
                    EDP = MS;
                }
                let SY = (JB / HT) * HJ;
                let SZ = SY * parameters[431];
                let TA = SY * parameters[432];
                let TB = parameters[433] / (if (D + ((parameters[434] * GW) / HJ)) >= R { (D + ((parameters[434] * GW) / HJ)) } else { R });
                let TE = if (parameters[437] + (parameters[439] * HQ)) >= A { (parameters[437] + (parameters[439] * HQ)) } else { A };
                let TF = if (parameters[438] + (parameters[440] * HQ)) >= A { (parameters[438] + (parameters[440] * HQ)) } else { A };
                let TG = if (((D + (parameters[444] * HO)) + (parameters[445] * HQ)) + ((parameters[446] * HO) * HQ)) >= KV { (((D + (parameters[444] * HO)) + (parameters[445] * HQ)) + ((parameters[446] * HO) * HQ)) } else { KV };
                let TI = if (if GS > D { 1.0 } else { 0.0 }) != 0.0 && (if TH > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let TW;
                if TI != 0.0 {
                    let TJ = (-(TH + GX)) / parameters[449];
                    let TK = if (TJ.abs()) < KE { 1.0 } else { 0.0 };
                    let TS;
                    if TK != 0.0 {
                        let TL = TJ.exp();
                        TS = TL;
                    } else {
                        let TM = if TJ < -8e1f64 { 1.0 } else { 0.0 };
                        let TT = if TM != 0.0 {
                            let TN = (-TJ) - KE;
                            let TO = KH / (D + (TN * (D + ((F * TN) * (D + (TN * KJ))))));
                            TO
                        } else {
                            let TQ = TJ - KE;
                            let TR = TP * (D + (TQ * (D + ((F * TQ) * (D + (TQ * KJ))))));
                            TR
                        };
                        TS = TT;
                    }
                    let TU = D - TS;
                    let TV = (((HC * parameters[450]) * TS) * (TU - ((D - (TS.powf(GS))) / GS))) / (TU * TU);
                    TW = TV;
                } else {
                    TW = A;
                }
                let TX = if (W / (TG / (D + TW))) >= GW { (W / (TG / (D + TW))) } else { GW };
                let UA = ((((parameters[452] * KU) * KU) * HL) * HL) * (HK.powf((parameters[453] - HC)));
                let UB = if ((parameters[454] * HM) + (parameters[455] * HL)) >= A { ((parameters[454] * HM) + (parameters[455] * HL)) } else { A };
                let UC = parameters[456] * HM;
                let UD = parameters[457] * HM;
                let UI = if ((((parameters[498] * (((KJ * HP) / UH) + HS)) / (UH * HR)) + ((parameters[496] + parameters[497]) / (HP * HN))) + (GS * parameters[495])) >= A { ((((parameters[498] * (((KJ * HP) / UH) + HS)) / (UH * HR)) + ((parameters[496] + parameters[497]) / (HP * HN))) + (GS * parameters[495])) } else { A };
                let UJ = if parameters[500] >= A { parameters[500] } else { A };
                let UK = if parameters[501] >= A { parameters[501] } else { A };
                let UM = if UL == A { 1.0 } else { 0.0 };
                let UO = if UM != 0.0 {
                    UJ
                } else {
                    UK
                };
                let UN = (GS * parameters[39]) * UJ;
                let UP = (GS * parameters[40]) * UO;
                let UQ = GS * parameters[502];
                let UU = if (if (if (if UR > A { 1.0 } else { 0.0 }) != 0.0 && (if US > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if UT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if GS == D { 1.0 } else { 0.0 }) != 0.0 || TI != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ACL;
                let ACP;
                let ACU;
                let ADA;
                let ADT;
                let AED;
                let AFI;
                let AFS;
                let AGC;
                let AGG;
                let AHX;
                let AID;
                if UU != 0.0 {
                    let UV = if UR == D { 1.0 } else { 0.0 };
                    let ACM;
                    let ACQ;
                    let ACV;
                    let ADB;
                    let ADU;
                    let AEE;
                    let AFJ;
                    let AFT;
                    let AGD;
                    let AGH;
                    let AHY;
                    let AIE;
                    if UV != 0.0 {
                        let mut UW = 0.0;
                        let mut UY = 0.0;
                        let mut VC = 0.0;
                        UW = A;
                        UY = A;
                        VC = A;
                        loop {
                            let UX = if UW < (GS - F) { 1.0 } else { 0.0 };
                            if UX == 0.0 {
                                break;
                            }
                            let UZ = F * GX;
                            let VA = UW * (TH + GX);
                            let VB = UY + (D / ((US + UZ) + VA));
                            let VD = VC + (D / ((UT + UZ) + VA));
                            let VE = UW + D;
                            UW = VE;
                            UY = VB;
                            VC = VD;
                        }
                        let VG = F * GX;
                        let VH = D / (VF + VG);
                        let VJ = D / (VI + VG);
                        let VL = if (HF + VK) >= GT { (HF + VK) } else { GT };
                        let VM = D / (HN.powf(parameters[471]));
                        let VN = D / (VL.powf(parameters[472]));
                        let VO = (((D + (parameters[468] * VM)) + (parameters[469] * VN)) + ((parameters[470] * VM) * VN)) * (D + (parameters[467] * (AC - D)));
                        let VQ = (UY / GS) + (VC / GS);
                        let VR = (VP * VQ) / VO;
                        let VS = (VP * (VH + VJ)) / VO;
                        let VT = D / (HN.powf(parameters[477]));
                        let VU = D / (VL.powf(parameters[478]));
                        let VW = if (((D + (parameters[474] * VT)) + (parameters[475] * VU)) + ((parameters[476] * VT) * VU)) >= VV { (((D + (parameters[474] * VT)) + (parameters[475] * VU)) + ((parameters[476] * VT) * VU)) } else { VV };
                        let VX = (VQ - VH) - VJ;
                        let VY = D + VR;
                        let VZ = D + VS;
                        let WA = if ((KU * VY) / VZ) >= KV { ((KU * VY) / VZ) } else { KV };
                        let WB = KX * WA;
                        let WD = (VY * (D + (WC * VS))) / (VZ * (D + (WC * VR)));
                        let WE = if (MB * WD) >= A { (MB * WD) } else { A };
                        let WG = if (WF * WD) >= A { (WF * WD) } else { A };
                        let WH = (parameters[473] * VX) / VW;
                        let WI = IR + WH;
                        let WJ = IU + WH;
                        let WL = WK + WH;
                        let WN = WM + WH;
                        let WO = (parameters[479] * VX) / (VW.powf(parameters[480]));
                        let WP = if (JU + WO) >= A { (JU + WO) } else { A };
                        let WR = if (WQ + WO) >= A { (WQ + WO) } else { A };
                        let WS = (JW * HW) / HT;
                        let WT = WP * WS;
                        let WU = WR * WS;
                        ACM = WP;
                        ACQ = WT;
                        ACV = WR;
                        ADB = WU;
                        ADU = WJ;
                        AEE = WN;
                        AFJ = WI;
                        AFT = WL;
                        AGD = WA;
                        AGH = WB;
                        AHY = WE;
                        AIE = WG;
                    } else {
                        let WW = -1e0f64 / WV;
                        let mut WX = 0.0;
                        let mut XP = 0.0;
                        WX = A;
                        XP = A;
                        loop {
                            let WY = if WX < (GS - F) { 1.0 } else { 0.0 };
                            if WY == 0.0 {
                                break;
                            }
                            let WZ = F * GX;
                            let XA = TH + GX;
                            let XC = (-((US + WZ) + (WX * XA))) / XB;
                            let XD = if XC > -8e1f64 { 1.0 } else { 0.0 };
                            let XM = if XD != 0.0 {
                                let XE = XC.exp();
                                XE
                            } else {
                                let XF = (-XC) - KE;
                                let XG = KH / (D + (XF * (D + ((F * XF) * (D + (XF * KJ))))));
                                XG
                            };
                            let XH = (-((UT + WZ) + (((GS - D) - WX) * XA))) / XB;
                            let XI = if XH > -8e1f64 { 1.0 } else { 0.0 };
                            let XO = if XI != 0.0 {
                                let XJ = XH.exp();
                                XJ
                            } else {
                                let XK = (-XH) - KE;
                                let XL = KH / (D + (XK * (D + ((F * XK) * (D + (XK * KJ))))));
                                XL
                            };
                            let XN = -WV;
                            let XQ = XP + ((F * (((D - XM).powf(XN)) + ((D - XO).powf(XN)))).powf(WW));
                            let XR = WX + D;
                            WX = XR;
                            XP = XQ;
                        }
                        let XS = D - (XP / GS);
                        let XT = F * GX;
                        let XU = (-(VF + XT)) / XB;
                        let XV = if XU > -8e1f64 { 1.0 } else { 0.0 };
                        let YE = if XV != 0.0 {
                            let XW = XU.exp();
                            XW
                        } else {
                            let XX = (-XU) - KE;
                            let XY = KH / (D + (XX * (D + ((F * XX) * (D + (XX * KJ))))));
                            XY
                        };
                        let XZ = (-(VI + XT)) / XB;
                        let YA = if XZ > -8e1f64 { 1.0 } else { 0.0 };
                        let YG = if YA != 0.0 {
                            let YB = XZ.exp();
                            YB
                        } else {
                            let YC = (-XZ) - KE;
                            let YD = KH / (D + (YC * (D + ((F * YC) * (D + (YC * KJ))))));
                            YD
                        };
                        let YF = -WV;
                        let YH = D - ((F * (((D - YE).powf(YF)) + ((D - YG).powf(YF)))).powf(WW));
                        let YI = parameters[486] / (D + (parameters[487] * (AC - D)));
                        let YJ = YI * XS;
                        let YK = YI * YH;
                        let YL = XS - YH;
                        let YM = D + YJ;
                        let YN = D + YK;
                        let YO = if ((KU * YM) / YN) >= KV { ((KU * YM) / YN) } else { KV };
                        let YP = KX * YO;
                        let YR = (YM * (D + (YQ * YK))) / (YN * (D + (YQ * YJ)));
                        let YS = if (MB * YR) >= A { (MB * YR) } else { A };
                        let YT = if (WF * YR) >= A { (WF * YR) } else { A };
                        let YU = (parameters[483] * YL) / (if (D + ((parameters[484] * (if (HF + VK) >= GT { (HF + VK) } else { GT })) / GW)) >= VV { (D + ((parameters[484] * (if (HF + VK) >= GT { (HF + VK) } else { GT })) / GW)) } else { VV });
                        let YV = IR + YU;
                        let YW = IU + YU;
                        let YX = WK + YU;
                        let YY = WM + YU;
                        let YZ = ((parameters[485] * YL) * JP) * JR;
                        let ZA = if (JU + YZ) >= A { (JU + YZ) } else { A };
                        let ZB = if (WQ + YZ) >= A { (WQ + YZ) } else { A };
                        let ZC = (JW * HW) / HT;
                        let ZD = ZA * ZC;
                        let ZE = ZB * ZC;
                        ACM = ZA;
                        ACQ = ZD;
                        ACV = ZB;
                        ADB = ZE;
                        ADU = YW;
                        AEE = YY;
                        AFJ = YV;
                        AFT = YX;
                        AGD = YO;
                        AGH = YP;
                        AHY = YS;
                        AIE = YT;
                    }
                    ACL = ACM;
                    ACP = ACQ;
                    ACU = ACV;
                    ADA = ADB;
                    ADT = ADU;
                    AED = AEE;
                    AFI = AFJ;
                    AFS = AFT;
                    AGC = AGD;
                    AGG = AGH;
                    AHX = AHY;
                    AID = AIE;
                } else {
                    ACL = JV;
                    ACP = JX;
                    ACU = ACW;
                    ADA = ADC;
                    ADT = IU;
                    AED = WM;
                    AFI = IR;
                    AFS = WK;
                    AGC = KW;
                    AGG = KY;
                    AHX = MC;
                    AID = AIF;
                }
                ZG = IH;
                ZH = MZ;
                ZI = NB;
                ZJ = ND;
                ZK = NS;
                ZL = NU;
                ZM = NW;
                ZN = NY;
                ZO = OA;
                ZP = SZ;
                ZQ = TE;
                ZR = HV;
                AAN = IA;
                AAO = HU;
                AAQ = AAS;
                AAU = HT;
                AAY = HW;
                ABM = JN;
                ABR = IF;
                ACF = JL;
                ACI = JY;
                ACK = ACL;
                ACO = ACP;
                ACS = ACU;
                ACY = ADA;
                ADE = JZ;
                ADG = IE;
                ADS = ADT;
                ADV = ADX;
                AEA = AED;
                AEH = IW;
                AEZ = IV;
                AFH = AFI;
                AFP = AFS;
                AFY = KZ;
                AGB = AGC;
                AGF = AGG;
                AGJ = LJ;
                AGK = LI;
                AGL = LL;
                AGM = LK;
                AGO = LD;
                AGP = LA;
                AGR = LF;
                AGS = LE;
                AGU = LO;
                AGV = LM;
                AGZ = LG;
                AHB = LH;
                AHE = LP;
                AHJ = LS;
                AHK = LQ;
                AHN = MM;
                AHP = AHS;
                AHU = MD;
                AHW = AHX;
                AIA = AID;
                AIH = MT;
                AIJ = NF;
                AIM = MY;
                AIQ = NA;
                AIU = NE;
                AIW = NG;
                AJB = NC;
                AJD = NQ;
                AJG = IG;
                AJI = NI;
                AJK = NH;
                AJM = NL;
                AJO = NJ;
                AJQ = NN;
                AJS = NM;
                AJU = MX;
                AJX = NR;
                AKC = NT;
                AKI = NX;
                AKL = NV;
                AKN = OD;
                AKO = OE;
                AKR = OG;
                AKV = OJ;
                AKX = OH;
                AKZ = OI;
                ALB = OT;
                ALC = OS;
                ALE = OU;
                ALH = OW;
                ALK = OX;
                ALO = TY;
                ALP = TX;
                ALS = TZ;
                ALU = UA;
                ALW = UI;
                ALZ = UN;
                AMC = UP;
                AMF = UQ;
                AQO = JI;
                AQP = JK;
                AQT = MS;
                AXH = JM;
                AXP = KA;
                AXQ = KB;
                AXV = IX;
                AYC = IY;
                BWE = LN;
                BWL = LB;
                BWM = LC;
                BWQ = LT;
                BWT = LU;
                BWY = LV;
                BXD = LR;
                BYN = ME;
                BYS = MF;
                CYE = MU;
                CYJ = MV;
                CYL = MW;
                DBY = TD;
                DGY = II;
                DJO = TA;
                DNH = NK;
                DOW = NO;
                DOX = NP;
                DYN = NZ;
                DYZ = OB;
                DZQ = OM;
                DZS = ON;
                DZU = OQ;
                DZX = OO;
                DZY = OP;
                EAB = OK;
                EAF = OL;
                EBX = OF;
                ECJ = OC;
                EDD = EDF;
                EDG = EDI;
                EDM = EDP;
                GKE = OY;
                GMD = TF;
                GMF = TB;
                GMH = TC;
                GMN = OV;
                GMQ = GV;
                GPH = UB;
                GPI = UC;
                GPJ = UD;
                GPL = UE;
                GPM = UF;
                GPW = UG;
            }
            let ZF = if UL == A { 1.0 } else { 0.0 };
            let AIP;
            let AIT;
            let AJA;
            let AKB;
            let AKH;
            let AKK;
            let DGX;
            let DJN;
            let DYM;
            let DYY;
            let GMC;
            if ZF != 0.0 {
                AIP = ZH;
                AIT = ZJ;
                AJA = ZI;
                AKB = ZK;
                AKH = ZM;
                AKK = ZL;
                DGX = ZG;
                DJN = ZP;
                DYM = ZN;
                DYY = ZO;
                GMC = ZQ;
            } else {
                AIP = AIQ;
                AIT = AIU;
                AJA = AJB;
                AKB = AKC;
                AKH = AKI;
                AKK = AKL;
                DGX = DGY;
                DJN = DJO;
                DYM = DYN;
                DYY = DYZ;
                GMC = GMD;
            }
            let ZS = D - ZR;
            let ZT = (IZ * ZS) + (JA * ZR);
            let ZX = ZW + Z;
            let ZY = ZU - ((ZV * AA) / ZX);
            let AAC = AAB + Z;
            let AAE = (((ZZ - ((AAA * AA) / AAC)) - ZY) + (-4e-1f64 * ZS)) * ZR;
            let AAF = F * (ZY + AAE);
            let AAG = AAF * AF;
            let AAH = D / (D + ((J * ZR).sqrt()));
            let AAJ = AAI * ZR;
            let AAK = F * AAE;
            let AAL = AAJ - AAK;
            let AAP = (((AAM * AAN) * F) * AAO) / JB;
            let AAT = if AAQ > A { 1.0 } else { 0.0 };
            let AFK;
            let AFN;
            if AAT != 0.0 {
                let AAW = AAV * JC;
                let AAX = AAP * (AAU + AAW);
                let AAZ = AAP * (AAY + AAW);
                AFK = AAX;
                AFN = AAZ;
            } else {
                let ABA = -AAP;
                let ABB = AAV * JC;
                let ABC = ABA * (AAU + ABB);
                let ABD = ABA * (AAY + ABB);
                AFK = ABC;
                AFN = ABD;
            }
            let ABF = (Z * ABE).sqrt();
            let ABH = ((ABG * ABF) * ABF) * ABF;
            let ABI = ABH * AAH;
            let ABJ = ABH * ((AAK * AF).exp());
            let ABK = JB / AAU;
            let ABL = JB / AAY;
            let ABN = if ABM > A { 1.0 } else { 0.0 };
            let ABV;
            let ABX;
            if ABN != 0.0 {
                let ABO = ABK * (D + ABM);
                ABV = ABO;
                ABX = ABL;
            } else {
                let ABP = ABL * (D - ABM);
                ABV = ABK;
                ABX = ABP;
            }
            let ABQ = ZT / AAO;
            let ABS = AE * (D + (ABR * AD));
            let ABT = D / ABS;
            let ABU = AAF * ABT;
            let ABW = ABV / ABQ;
            let ABY = ABX / ABQ;
            let ABZ = D / ABW;
            let ACA = D / ((D + ABZ) + (D / ABY));
            let ACB = ((3.20435313e-19f64 * ABI) * ZT) * ABT;
            let ACC = ABQ * ABQ;
            let ACE = ((ACC / ACB).ln()) - ACD;
            let ACG = ABV + ABX;
            let ACH = (((8.010882825e-20f64 * ACF) * AAO) / ACG) * ABT;
            let ACJ = ACI * AB;
            let ACN = ACK + ACJ;
            let ACR = ACO + ACJ;
            let ACX = ACS + ACJ;
            let ADD = ACY + ACJ;
            let ADF = ADE * ABT;
            let ADH = ((((3.20435313e-19f64 * ADG) * IZ) * AF).sqrt()) / ABX;
            let ADI = ADH * ADH;
            let ADJ = D / ADI;
            let ADL = D + (ADH / ADK);
            let ADM = D / ADL;
            let ADO = ADN * ADL;
            let ADP = ((ADG / ABJ).ln()) + AAG;
            let ADQ = HC * ADP;
            let ADR = if parameters[2] > A { 1.0 } else { 0.0 };
            let AFM;
            let AFV;
            if ADR != 0.0 {
                let ADY = (ADV * AE) * ADP;
                let ADZ = ADS + ADY;
                let AEF = AEA + ADY;
                AFM = ADZ;
                AFV = AEF;
            } else {
                AFM = ADS;
                AFV = AEA;
            }
            let AEG = if parameters[9] > A { 1.0 } else { 0.0 };
            let AFF = if AEG != 0.0 {
                let AEI = AE * (((AEH / ABJ).ln()) + AAG);
                AEI
            } else {
                A
            };
            let AEJ = (((3.20435313e-19f64 * ZT) * AEH).sqrt()) / ABK;
            let AVT = if E != 0.0 {
                let AEM = AEL / Z;
                let AEN = AEK - AEM;
                let AEO = F * ((AEK + AEM) + (((AEN * AEN) + GW).sqrt()));
                AEO
            } else {
                AEK
            };
            let AEP = (1e18f64 * AAO) * AAO;
            let AEQ = if AAV > A { 1.0 } else { 0.0 };
            let AFB;
            let AWB;
            if AEQ != 0.0 {
                let AES = if AER == D { 1.0 } else { 0.0 };
                let AFC;
                let AWC;
                if AES != 0.0 {
                    let AET = 4.09618895e-1f64 / AEP;
                    let AEV = ((AAD * AAV) * AEU) * ((-3.333333333333e-1f64 * ((ABS * AEP).ln())).exp());
                    AFC = AET;
                    AWC = AEV;
                } else {
                    let AEW = 7.23134895e-1f64 / AEP;
                    let AEY = ((AAD * AAV) * AEX) * ((-3.333333333333e-1f64 * ((ABS * AEP).ln())).exp());
                    AFC = AEW;
                    AWC = AEY;
                }
                AFB = AFC;
                AWB = AWC;
            } else {
                AFB = A;
                AWB = A;
            }
            let AFA = AER * AEZ;
            let AFD = (AFA * AB) + AFB;
            let AFG = (AFD + AFE) - AFF;
            let AFL = (AER * ((AFH + AAL) + AFK)) + AFG;
            let AFO = (AER * ((AFM + AAL) + AFN)) + AFD;
            let AFU = (AER * ((AFP + AAL) + AFK)) + AFG;
            let AFW = (AER * ((AFV + AAL) + AFN)) + AFD;
            let AFX = AD.ln();
            let AGA = ((AFY * AFX).exp()) * AFZ;
            let AGE = AGB * AGA;
            let AGI = AGF * AGA;
            let AGN = AGM * ((AGL * AFX).exp());
            let AGQ = AGP * ((AGO * AFX).exp());
            let AGT = AGS * ((AGR * AFX).exp());
            let AGW = AGV * ((AGU * AFX).exp());
            let AGY = ((AGX * ABS) / AAO) * (AGK * ((AGJ * AFX).exp()));
            let AHA = D / (F * AGZ);
            let AHC = AHA / AHB;
            let AHD = if AER == D { 1.0 } else { 0.0 };
            let AHH = if AHD != 0.0 {
                let AHF = F * AHE;
                AHF
            } else {
                let AHG = KJ * AHE;
                AHG
            };
            let AHI = D - AHH;
            let AHL = (HC * (AHK * ((AHJ * AFX).exp()))) * ABS;
            let AHO = ((AHM * (((((ML / AHN) * ACD).exp()) - D).ln())).exp()) - D;
            let AHT = ((AHM * (((((ML / AHP) * ACD).exp()) - D).ln())).exp()) - D;
            let AHV = (AHU * AFX).exp();
            let AHZ = ((AHW * AHV) * AGA) * ABS;
            let AIG = ((AIA * AHV) * AGA) * ABS;
            let AII = AIH * ABT;
            let AIK = -AIJ;
            let AIL = (AIK * AFX).exp();
            let AIN = AIM * AIL;
            let AIO = ZH * AIL;
            let AIR = AIP * AIL;
            let AIS = ZJ * AIL;
            let AIV = AIT * AIL;
            let AIX = -AIW;
            let AIY = (AIX * AFX).exp();
            let AIZ = ZI * AIY;
            let AJC = AJA * AIY;
            let AJE = D / AJD;
            let AJH = ((1.3333333333332e0f64 * ((2.9189679640027008e-49f64 * AJD).sqrt())) / 1.054571726e-34f64) * AJG;
            let AJJ = if AJI < A { 1.0 } else { 0.0 };
            let DUH = if AJJ != 0.0 {
                let AJL = (-4.95e-1f64 * AJK) / AJI;
                AJL
            } else {
                A
            };
            let AJN = if AJM < A { 1.0 } else { 0.0 };
            let DNC = if AJN != 0.0 {
                let AJP = (-4.95e-1f64 * AJO) / AJM;
                AJP
            } else {
                A
            };
            let AJR = if AJQ < A { 1.0 } else { 0.0 };
            let DNB = if AJR != 0.0 {
                let AJT = (-4.95e-1f64 * AJS) / AJQ;
                AJT
            } else {
                A
            };
            let AJV = AJU * ABS;
            let AJW = AJU * AE;
            let AJY = D / (D + (AJX * ABU));
            let AJZ = 4e-18f64 / (AJG * AJG);
            let AKA = ZK * AJZ;
            let AKD = AKB * AJZ;
            let AKE = AJG * 5e8f64;
            let AKF = D + (ZM * AB);
            let AKG = (ZL * (F * (AKF + (((AKF * AKF) + O).sqrt())))) * AKE;
            let AKJ = D + (AKH * AB);
            let AKM = (AKK * (F * (AKJ + (((AKJ * AKJ) + O).sqrt())))) * AKE;
            let AKP = -AKO;
            let AKQ = AKN * ((AKP * AFX).exp());
            let AKS = AE * (D + (AKR * AD));
            let AKT = D / AKS;
            let AKU = ((3.20435313e-19f64 * ABI) * ZT) * AKT;
            let AKW = ((AER * AKV) * AB) + AFB;
            let AKY = (((AER * ((AKX + AAL) + AFK)) + AKW) + AFE) - AFF;
            let ALA = (AER * ((AKZ + AAL) + AFN)) + AKW;
            let ALD = ALC * (((ALB * AFX).exp()) * AFZ);
            let ALF = ALE * ABS;
            let ALI = (4.0054414125e-20f64 * ALH) / (ZT * ABS);
            let ALJ = (ALH / ABI).ln();
            let ALL = ALK * 1.25e-6f64;
            let ALM = ALL * ABS;
            let ALN = (((ZT / JB) * AAO) * (AAU + JC)).sqrt();
            let ALQ = ALP * ((ALO * AFX).exp());
            let ALR = 5.5225952e-23f64 * Z;
            let ALT = ALS * ALR;
            let ALV = 9.10938291e-19f64 * ALU;
            let ALX = if ALW > A { 1.0 } else { 0.0 };
            let ECS = if ALX != 0.0 {
                let ALY = D / ALW;
                ALY
            } else {
                A
            };
            let AMA = if ALZ > A { 1.0 } else { 0.0 };
            let ECU = if AMA != 0.0 {
                let AMB = D / ALZ;
                AMB
            } else {
                A
            };
            let AMD = if AMC > A { 1.0 } else { 0.0 };
            let ECW = if AMD != 0.0 {
                let AME = D / AMC;
                AME
            } else {
                A
            };
            let AMG = if AMF > A { 1.0 } else { 0.0 };
            let ECY = if AMG != 0.0 {
                let AMH = D / AMF;
                AMH
            } else {
                A
            };
            let AMJ = if AMI > A { 1.0 } else { 0.0 };
            let AQH;
            let AQM;
            let AQN;
            let AQQ;
            let AQR;
            let AQS;
            let AQU;
            let ARH;
            let AVR;
            let AVY;
            let AXB;
            let AXE;
            let AXN;
            let AXX;
            let BVZ;
            let BWD;
            let BWK;
            let BWN;
            let BWX;
            let BXF;
            let BXG;
            let BXH;
            let BXI;
            let DBM;
            let DBR;
            let DBV;
            let DCK;
            let DCL;
            let DHF;
            let DHG;
            let DLT;
            let DMH;
            let DNF;
            let DQU;
            let DTM;
            let DUC;
            let DUL;
            let DUM;
            let DXP;
            let DYP;
            let ECA;
            let ECP;
            let ECR;
            let EDB;
            let EDC;
            let EDJ;
            let EDK;
            let EDL;
            let GKT;
            let GLL;
            let GLN;
            let GLR;
            let GOO;
            if AMJ != 0.0 {
                let AMK = Z + node_potentials[4];
                let AML = AMK * AMK;
                let AMM = AMK - B;
                let AMN = B / AMK;
                let AMO = AMK * K;
                let AMP = D / AMO;
                let AXY = if E != 0.0 {
                    let AMQ = J / AE;
                    let AMR = AMQ - M;
                    let AMS = F * ((AMQ + M) + (((AMR * AMR) + O).sqrt()));
                    AMS
                } else {
                    M
                };
                let AMT = ZU - ((ZV * AML) / (ZW + AMK));
                let AMU = (((ZZ - ((AAA * AML) / (AAB + AMK))) - AMT) + (-4e-1f64 * ZS)) * ZR;
                let AMV = F * (AMT + AMU);
                let AMW = AMV * AMP;
                let AMX = AAJ - (F * AMU);
                let AMY = (AMK * ABE).sqrt();
                let AMZ = (((ABG * AMY) * AMY) * AMY) * AAH;
                let ANA = AMO * (D + (ABR * AMN));
                let ANB = D / ANA;
                let ANC = AMV * ANB;
                let AND = ((3.20435313e-19f64 * AMZ) * ZT) * ANB;
                let ANE = ((ACC / AND).ln()) - ACD;
                let ANF = (((8.010882825e-20f64 * ACF) * AAO) / ACG) * ANB;
                let ANG = ACI * AMM;
                let ANH = ACK + ANG;
                let ANI = ACO + ANG;
                let ANJ = ADE * ANB;
                let ANK = ACS + ANG;
                let ANL = ACY + ANG;
                let ANT = if AEG != 0.0 {
                    let ANM = AMO * (((AEH / ABJ).ln()) + AAG);
                    ANM
                } else {
                    AFF
                };
                let AVS = if E != 0.0 {
                    let ANN = AEL / Z;
                    let ANO = AEK - ANN;
                    let ANP = F * ((AEK + ANN) + (((ANO * ANO) + GW).sqrt()));
                    ANP
                } else {
                    AVT
                };
                let AVZ;
                if AEQ != 0.0 {
                    let AWA = if AHD != 0.0 {
                        let ANQ = ((AAD * AAV) * AEU) * ((-3.333333333333e-1f64 * ((ANA * AEP).ln())).exp());
                        ANQ
                    } else {
                        let ANR = ((AAD * AAV) * AEX) * ((-3.333333333333e-1f64 * ((ANA * AEP).ln())).exp());
                        ANR
                    };
                    AVZ = AWA;
                } else {
                    AVZ = A;
                }
                let ANS = (AFA * AMM) + AFB;
                let ANU = (ANS + AFE) - ANT;
                let ANV = (AER * ((AFH + AMX) + AFK)) + ANU;
                let ANW = (AER * ((AFM + AMX) + AFN)) + ANS;
                let ANX = (AER * ((AFP + AMX) + AFK)) + ANU;
                let ANY = (AER * ((AFV + AMX) + AFN)) + ANS;
                let ANZ = AMN.ln();
                let AOA = ((AFY * ANZ).exp()) * AFZ;
                let AOB = AGB * AOA;
                let AOC = AGF * AOA;
                let AOD = AGM * ((AGL * ANZ).exp());
                let AOE = AGP * ((AGO * ANZ).exp());
                let AOF = AGS * ((AGR * ANZ).exp());
                let AOG = AGV * ((AGU * ANZ).exp());
                let AOH = ((AGX * ANA) / AAO) * (AGK * ((AGJ * ANZ).exp()));
                let AOI = (HC * (AHK * ((AHJ * ANZ).exp()))) * ANA;
                let AOJ = (AHU * ANZ).exp();
                let AOK = ((AHW * AOJ) * AOA) * ANA;
                let AOL = ((AIA * AOJ) * AOA) * ANA;
                let AOM = AIH * ANB;
                let AON = (AIK * ANZ).exp();
                let AOO = AIM * AON;
                let AOP = ZH * AON;
                let AOQ = AIP * AON;
                let AOR = ZJ * AON;
                let AOS = AIT * AON;
                let AOT = (AIX * ANZ).exp();
                let AOU = ZI * AOT;
                let AOV = AJA * AOT;
                let AOW = AJU * ANA;
                let AOX = AJU * AMO;
                let AOY = D / (D + (AJX * ANC));
                let AOZ = D + (ZM * AMM);
                let APA = (ZL * (F * (AOZ + (((AOZ * AOZ) + O).sqrt())))) * AKE;
                let APB = D + (AKH * AMM);
                let APC = (AKK * (F * (APB + (((APB * APB) + O).sqrt())))) * AKE;
                let APD = AKN * ((AKP * ANZ).exp());
                let APE = ALE * ANA;
                let APF = (4.0054414125e-20f64 * ALH) / (ZT * ANA);
                let APG = (ALH / AMZ).ln();
                let APH = ALL * ANA;
                let API = ALP * ((ALO * ANZ).exp());
                let APJ = 5.5225952e-23f64 * AMK;
                let APK = ALS * APJ;
                AQH = ANB;
                AQM = ANV;
                AQN = ANW;
                AQQ = ANH;
                AQR = ANI;
                AQS = AOK;
                AQU = AMW;
                ARH = AND;
                AVR = AVS;
                AVY = AVZ;
                AXB = ANE;
                AXE = ANF;
                AXN = ANJ;
                AXX = AXY;
                BVZ = ANZ;
                BWD = AOG;
                BWK = AOE;
                BWN = AOF;
                BWX = AOI;
                BXF = AOD;
                BXG = AOH;
                BXH = AOB;
                BXI = AOC;
                DBM = AOM;
                DBR = ANA;
                DBV = AMP;
                DCK = AOP;
                DCL = AOR;
                DHF = AOQ;
                DHG = AOS;
                DLT = AMO;
                DMH = AOX;
                DNF = AOU;
                DQU = AOV;
                DTM = AOO;
                DUC = AOW;
                DUL = AMV;
                DUM = AOY;
                DXP = APA;
                DYP = APC;
                ECA = APD;
                ECP = API;
                ECR = APJ;
                EDB = ANX;
                EDC = ANY;
                EDJ = ANK;
                EDK = ANL;
                EDL = AOL;
                GKT = APE;
                GLL = APF;
                GLN = APG;
                GLR = APH;
                GOO = APK;
            } else {
                AQH = ABT;
                AQM = AFL;
                AQN = AFO;
                AQQ = ACN;
                AQR = ACR;
                AQS = AHZ;
                AQU = AAG;
                ARH = ACB;
                AVR = AVT;
                AVY = AWB;
                AXB = ACE;
                AXE = ACH;
                AXN = ADF;
                AXX = AXZ;
                BVZ = AFX;
                BWD = AGW;
                BWK = AGQ;
                BWN = AGT;
                BWX = AHL;
                BXF = AGN;
                BXG = AGY;
                BXH = AGE;
                BXI = AGI;
                DBM = AII;
                DBR = ABS;
                DBV = AF;
                DCK = AIO;
                DCL = AIS;
                DHF = AIR;
                DHG = AIV;
                DLT = AE;
                DMH = AJW;
                DNF = AIZ;
                DQU = AJC;
                DTM = AIN;
                DUC = AJV;
                DUL = AAF;
                DUM = AJY;
                DXP = AKG;
                DYP = AKM;
                ECA = AKQ;
                ECP = ALQ;
                ECR = ALR;
                EDB = AFU;
                EDC = AFW;
                EDJ = ACX;
                EDK = ADD;
                EDL = AIG;
                GKT = ALF;
                GLL = ALI;
                GLN = ALJ;
                GLR = ALM;
                GOO = ALT;
            }
            let APV;
            let APX;
            let APZ;
            if AHD != 0.0 {
                let APN = APL - APM;
                let APP = APO - APM;
                let APR = APM - APQ;
                APV = APP;
                APX = APN;
                APZ = APR;
            } else {
                let APS = -(APL - APM);
                let APT = -(APO - APM);
                let APU = -(APM - APQ);
                APV = APT;
                APX = APS;
                APZ = APU;
            }
            let APW = -APV;
            let APY = APX + APW;
            let AQA = APV + APZ;
            let AQB = if APV < A { 1.0 } else { 0.0 };
            let AQD;
            let AQE;
            let AQG;
            let DXD;
            if AQB != 0.0 {
                AQD = APY;
                AQE = AQA;
                AQG = APW;
                DXD = AQC;
            } else {
                AQD = APX;
                AQE = APZ;
                AQG = APV;
                DXD = D;
            }
            let AQF = AQD + AQE;
            let AQI = AQG * AQH;
            let AQJ = (((AQG * AQG) + O).sqrt()) - 1e-1f64;
            let AQK = AQJ * AQH;
            let AQL = F * (AQI - AQK);
            let AQV = (((AQD - AQM) * AQH) - AQL) - AQU;
            let AQW = -AQE;
            let AQX = ((AQW - AQN) * AQH) - AQL;
            let AQY = AQX - AQU;
            let AVO;
            if ADR != 0.0 {
                let AQZ = AER * ADV;
                let ARA = D + ABW;
                let ARB = D + ABY;
                let ARC = ARA / ARB;
                let ARD = ARC.ln();
                let ARE = if ARD > AGX { 1.0 } else { 0.0 };
                let ARK = if ARE != 0.0 {
                    let ARF = ((HC * ARD) * (ARC + D)) / (ARC - D);
                    ARF
                } else {
                    let ARG = HC * (HC + ARD);
                    ARG
                };
                let ARI = ARH / ACC;
                let ARJ = D / ARB;
                let ARM = ((((ABW + (ABY * ARJ)) * ARK) / ARI).ln()) + ARL;
                let ARN = ((((ABY + (ABW * (D / ARA))) * ARK) / ARI).ln()) + ARL;
                let ARO = (ARM - (AQV - ((ACA * (AQV - AQY)) * ABZ))) / ARL;
                let ARP = if ARO < KE { 1.0 } else { 0.0 };
                let ARR = if ARP != 0.0 {
                    let ARQ = (D + (ARO.exp())).ln();
                    ARQ
                } else {
                    ARO
                };
                let ARS = (ARN - (((ABY * AQY) + (ARM - (ARL * ARR))) * ARJ)) / ARL;
                let ART = if ARS < KE { 1.0 } else { 0.0 };
                let ARV = if ART != 0.0 {
                    let ARU = (D + (ARS.exp())).ln();
                    ARU
                } else {
                    ARS
                };
                let ARW = AQZ * AQY;
                let ARX = (AQZ * (ARN - (ARL * ARV))) - ARW;
                let ARY = -ADQ;
                let ARZ = if (ARY.abs()) < KE { 1.0 } else { 0.0 };
                let ASI;
                if ARZ != 0.0 {
                    let ASA = ARY.exp();
                    ASI = ASA;
                } else {
                    let ASB = if ARY < -8e1f64 { 1.0 } else { 0.0 };
                    let ASJ = if ASB != 0.0 {
                        let ASC = (-ARY) - KE;
                        let ASD = KH / (D + (ASC * (D + ((F * ASC) * (D + (ASC * KJ))))));
                        ASD
                    } else {
                        let ASE = ARY - KE;
                        let ASF = TP * (D + (ASE * (D + ((F * ASE) * (D + (ASE * KJ))))));
                        ASF
                    };
                    ASI = ASJ;
                }
                let ASG = if (ARX.abs()) <= ADO { 1.0 } else { 0.0 };
                let AVL;
                if ASG != 0.0 {
                    let ASK = (ARX * ADM) * (D + (((ARX * (D - ASI)) * ADH) * (((ADM * ADM) * ASH) / ADK)));
                    AVL = ASK;
                } else {
                    let ASL = if ARX < (-ADO) { 1.0 } else { 0.0 };
                    let AVM;
                    if ASL != 0.0 {
                        let ASM = -ARX;
                        let ASO = ASN * (ASM * ADM);
                        let ASQ = ASO - ASP;
                        let ASS = F * ((ASO + J) - (((ASQ * ASQ) + ASR).sqrt()));
                        let AST = ASM - ASS;
                        let ASU = (AST * AST) + (ADI * (ASS + D));
                        let ASV = (HC * AST) - ADI;
                        let ASW = (-ASS) + ((ASU * ADJ).ln());
                        let ASX = ASU + ASV;
                        let ASY = (ASX * ASX) + (ASW * (((F * ASV) * ASV) - ASU));
                        let ASZ = ASS + (((ASU * ASX) * ASW) / (ASY + (((((ASX / ASY) * ASW) * ASW) * ASV) * (((ASV * ASV) * KJ) - ASU))));
                        let ATA = if ASZ < KE { 1.0 } else { 0.0 };
                        let ATE = if ATA != 0.0 {
                            let ATB = ASZ.exp();
                            ATB
                        } else {
                            let ATC = ASZ - KE;
                            let ATD = TP * (D + (ATC * (D + ((F * ATC) * (D + (ATC * KJ))))));
                            ATD
                        };
                        let ATF = ASZ * ASZ;
                        let ATG = D / (HC + ATF);
                        let ATH = ATF * ATG;
                        let ATK = ASM - ASZ;
                        let ATL = ASI * (D / ATE);
                        let ATM = (HC * ATK) + (ADI * (((ATE - D) - ATL) + (ASI * (D - (AJF * ((ASZ * ATG) * ATG))))));
                        let ATN = (ATK * ATK) - (ADI * ((((ATE - ASZ) - D) + ATL) + (ASI * ((ASZ - D) - ATH))));
                        let ATO = (-ASZ) - (HC * (ATN / (ATM + (((ATM * ATM) - (HC * (ATN * (HC - (ADI * ((ATE + ATL) - (ASI * ((((ATI * ATG) - (ATJ * ATH)) * ATG) * ATG)))))))).sqrt()))));
                        AVM = ATO;
                    } else {
                        let ATQ = D / (ASN + (ADH * ATP));
                        let ATR = -((ARX * ADM) * (D + (((((ASN * ADL) * ATQ) - D) * ATQ) * ARX)));
                        let ATS = if ATR > -8e1f64 { 1.0 } else { 0.0 };
                        let ATW = if ATS != 0.0 {
                            let ATT = ATR.exp();
                            ATT
                        } else {
                            let ATU = (-ATR) - KE;
                            let ATV = KH / (D + (ATU * (D + ((F * ATU) * (D + (ATU * KJ))))));
                            ATV
                        };
                        let ATX = (ARX + (ADI * F)) - (ADH * (((ARX + (ADI * ALG)) - (D - ATW)).sqrt()));
                        let ATZ = ADQ + ATY;
                        let AUA = ATX - ATZ;
                        let AUB = (F * ((ATX + ATZ) - (((AUA * AUA) + JH).sqrt()))) - (F * (ATZ - (((ATZ * ATZ) + JH).sqrt())));
                        let AUC = ARX - AUB;
                        let AUD = (-AUB).exp();
                        let AUE = AUB * AUB;
                        let AUF = D / (HC + AUE);
                        let AUG = AUE * AUF;
                        let AUI = if AUH >= ((AUC * AUC) - (ADI * (((AUD + AUB) - D) - (ASI * ((AUB + D) + AUG))))) { AUH } else { ((AUC * AUC) - (ADI * (((AUD + AUB) - D) - (ASI * ((AUB + D) + AUG))))) };
                        let AUJ = (HC * AUC) + (ADI * ((D - AUD) - (ASI * (D + (AJF * ((AUB * AUF) * AUF))))));
                        let AUK = (ADQ - AUB) + ((AUI / ADI).ln());
                        let AUL = AUI + AUJ;
                        let AUM = AUI * (D - (F * (ADI * (AUD - (ASI * ((((ATI * AUF) - (ATJ * AUG)) * AUF) * AUF))))));
                        let AUN = (AUL * AUL) + (AUK * (((F * AUJ) * AUJ) - AUM));
                        let AUO = AUB + (((AUI * AUL) * AUK) / (AUN + (((((AUL / AUN) * AUK) * AUK) * AUJ) * (((AUJ * AUJ) * KJ) - AUM))));
                        let AUP = if AUO < KE { 1.0 } else { 0.0 };
                        let AVE;
                        let AVG;
                        if AUP != 0.0 {
                            let AUQ = AUO.exp();
                            let AUR = D / AUQ;
                            let AUS = ASI * AUQ;
                            AVE = AUR;
                            AVG = AUS;
                        } else {
                            let AUT = if AUO > (ADQ - KE) { 1.0 } else { 0.0 };
                            let AVF;
                            let AVH;
                            if AUT != 0.0 {
                                let AUU = (AUO - ADQ).exp();
                                let AUV = ASI / AUU;
                                AVF = AUV;
                                AVH = AUU;
                            } else {
                                let AUW = (ADQ - AUO) - KE;
                                let AUX = KH / (D + (AUW * (D + ((F * AUW) * (D + (AUW * KJ))))));
                                let AUY = AUO - KE;
                                let AUZ = KH / (D + (AUY * (D + ((F * AUY) * (D + (AUY * KJ))))));
                                AVF = AUZ;
                                AVH = AUX;
                            }
                            AVE = AVF;
                            AVG = AVH;
                        }
                        let AVA = AUO * AUO;
                        let AVB = D / (HC + AVA);
                        let AVC = AVA * AVB;
                        let AVD = ARX - AUO;
                        let AVI = (HC * AVD) + (ADI * (((D - AVE) + AVG) - (ASI * (D + (AJF * ((AUO * AVB) * AVB))))));
                        let AVJ = (AVD * AVD) - (ADI * ((((AVE + AUO) - D) + AVG) - (ASI * ((AUO + D) + AVC))));
                        let AVK = AUO + (HC * (AVJ / (AVI + (((AVI * AVI) - (HC * (AVJ * (HC - (ADI * ((AVE + AVG) - (ASI * ((((ATI * AVB) - (ATJ * AVC)) * AVB) * AVB)))))))).sqrt()))));
                        AVM = AVK;
                    }
                    AVL = AVM;
                }
                let AVN = AQZ * (AVL + ARW);
                AVO = AVN;
            } else {
                AVO = AQY;
            }
            let AVP = AQV - AVO;
            let AVQ = ACA * AVP;
            let AWM;
            let AWS;
            let AWX;
            let AYL;
            let CZI;
            let CZK;
            if AEQ != 0.0 {
                let AVU = AVQ - AVR;
                let AVV = AVR * AVR;
                let AVW = -AVQ;
                let AVX = AVW - AVR;
                let AWD = AVY * ((-3.333333333333e-1f64 * ((F * ((AVQ + AVR) + (((AVU * AVU) + AVV).sqrt()))).ln())).exp());
                let AWE = AVY * ((-3.333333333333e-1f64 * ((F * ((AVW + AVR) + (((AVX * AVX) + AVV).sqrt()))).ln())).exp());
                let AWF = (D - AWD) - AWE;
                let AWG = ABQ / AWF;
                let AWH = (ABW * AWF) / (D + (ABW * AWD));
                let AWI = (ABY * AWF) / (D + (ABY * AWE));
                let AWJ = D / ((D + (D / AWH)) + (D / AWI));
                let AWK = D + (AWH * AWD);
                let AWL = D + (AWI * AWE);
                AWM = AWJ;
                AWS = AWH;
                AWX = AWI;
                AYL = AWG;
                CZI = AWK;
                CZK = AWL;
            } else {
                AWM = ACA;
                AWS = ABW;
                AWX = ABY;
                AYL = ABQ;
                CZI = D;
                CZK = D;
            }
            let AWN = AWM * AVP;
            let AWO = if AWN > A { 1.0 } else { 0.0 };
            let AXA;
            if AWO != 0.0 {
                let AWP = -AWN;
                let AWQ = if AWP < KE { 1.0 } else { 0.0 };
                let AWT = if AWQ != 0.0 {
                    let AWR = (D + (AWP.exp())).ln();
                    AWR
                } else {
                    AWP
                };
                let AWU = ((AQV - (AWN / AWS)) + AWT) - ACD;
                AXA = AWU;
            } else {
                let AWV = if AWN < KE { 1.0 } else { 0.0 };
                let AWY = if AWV != 0.0 {
                    let AWW = (D + (AWN.exp())).ln();
                    AWW
                } else {
                    AWN
                };
                let AWZ = ((AVO + (AWN / AWX)) + AWY) - ACD;
                AXA = AWZ;
            }
            let AXC = AXA - AXB;
            let AXD = F * ((AXA + AXB) - (((AXC * AXC) + AJF).sqrt()));
            let AXF = ((D + ((HC * (AXB - AXD)) / AXE)).sqrt()) - D;
            let AXG = AXD + (AXE * AXF);
            let AXI = D + (AXH * AQX);
            let AXJ = AXI - F;
            let AXK = F * ((AXI + F) + (((AXJ * AXJ) + O).sqrt()));
            let AXL = D / (D + (AQO * AXK));
            let AXM = D / (D + (AQP * AXK));
            let AXO = (HC * AXN) * (((D + (AQK / AXN)).sqrt()) - D);
            let AXR = (AXO * (D + (AXP * AXF))) * (D + (AXQ * AQX));
            let AXS = AQQ * AXR;
            let AXT = ((((AQV - AXG) + AXS) * AXL) + AXG) + AQL;
            let AXU = ((((AVO - AXG) + (AQR * AXR)) * AXM) + AXG) + AQL;
            let AXW = AXU + (AXV * (AXT - AXU));
            let AYA = AXW - AXX;
            let AYB = F * ((AXW + AXX) - (((AYA * AYA) + O).sqrt()));
            let AYD = AXT + (AYC * (AXU - AXT));
            let AYE = AYD - AXX;
            let AYF = F * ((AYD + AXX) - (((AYE * AYE) + O).sqrt()));
            let AYG = AWS / AXL;
            let AYH = AWX / AXM;
            let AYI = D / AYG;
            let AYJ = D / AYH;
            let AYK = D / ((D + AYI) + AYJ);
            let AYM = AYL * AYL;
            let AYN = ARH / AYM;
            let AYO = D + AYG;
            let AYP = D + AYH;
            let AYQ = AYO / AYP;
            let AYR = AYQ.ln();
            let AYS = if AYR > AGX { 1.0 } else { 0.0 };
            let AZD = if AYS != 0.0 {
                let AYT = ((HC * AYR) * (AYQ + D)) / (AYQ - D);
                AYT
            } else {
                let AYU = HC * (HC + AYR);
                AYU
            };
            let AYV = AYK * (AYB - AYF);
            let AYW = AYV * AYV;
            let AYX = AYV * AYI;
            let AYY = AYB - AYX;
            let AYZ = AYV * AYJ;
            let AZA = AYF + AYZ;
            let AZB = D / AYO;
            let AZC = D / AYP;
            let AZE = (((AYG + (AYH * AZC)) * AZD) / AYN).ln();
            let AZF = AZE + ATY;
            let AZG = (((AYH + (AYG * AZB)) * AZD) / AYN).ln();
            let AZH = AZG + ATY;
            let AZI = (AZF - AYY) * KJ;
            let AZJ = if AZI < KE { 1.0 } else { 0.0 };
            let AZL = if AZJ != 0.0 {
                let AZK = (D + (AZI.exp())).ln();
                AZK
            } else {
                AZI
            };
            let AZM = AZF - (ATY * AZL);
            let AZN = (AZH - AZA) * KJ;
            let AZO = if AZN < KE { 1.0 } else { 0.0 };
            let AZQ = if AZO != 0.0 {
                let AZP = (D + (AZN.exp())).ln();
                AZP
            } else {
                AZN
            };
            let AZR = AYG * AYB;
            let AZS = AYH * AYF;
            let AZT = (AZS + AZM) * AZC;
            let AZU = (AZF - ((AZR + (AZH - (ATY * AZQ))) * AZB)) * KJ;
            let AZV = if AZU < KE { 1.0 } else { 0.0 };
            let AZX = if AZV != 0.0 {
                let AZW = (D + (AZU.exp())).ln();
                AZW
            } else {
                AZU
            };
            let AZY = AZF - (ATY * AZX);
            let AZZ = (AZH - AZT) * KJ;
            let BAA = if AZZ < KE { 1.0 } else { 0.0 };
            let BAC = if BAA != 0.0 {
                let BAB = (D + (AZZ.exp())).ln();
                BAB
            } else {
                AZZ
            };
            let BAD = AYB - AZY;
            let BAE = AYF - (AZH - (ATY * BAC));
            let BAF = AYG * BAD;
            let BAG = AYB - BAD;
            let BAH = if BAG < KE { 1.0 } else { 0.0 };
            let BAL = if BAH != 0.0 {
                let BAI = BAG.exp();
                BAI
            } else {
                let BAJ = BAG - KE;
                let BAK = TP * (D + (BAJ * (D + ((F * BAJ) * (D + (BAJ * KJ))))));
                BAK
            };
            let BAM = AYN * BAL;
            let BAN = (BAF * BAF) - BAM;
            let BAO = HC * AYG;
            let BAP = (BAO * BAF) + BAM;
            let BAQ = BAO * AYG;
            let BAR = BAQ - BAM;
            let BAT = if BAN < -5e-3f64 { 1.0 } else { 0.0 };
            let BCF;
            let BCJ;
            let BCU;
            let BCY;
            let BDB;
            let BDI;
            let BDL;
            if BAT != 0.0 {
                let BAU = (BAN.abs()).sqrt();
                let BAV = BAU / ((F * BAU).tan());
                let BAW = (ALG * BAP) / BAN;
                let BAX = (BAN + (BAV * (HC - BAV))) * BAW;
                let BAY = ((BAP - ((HC * BAX) * (D + BAV))) * BAW) + ((BAX * BAR) / BAP);
                let BAZ = D - (F * BAV);
                let BBA = (BAP / BAN) * BAZ;
                let BBB = ((BAR * BAZ) - (BAP * (BBA + (F * BAX)))) / BAN;
                BCF = A;
                BCJ = BAU;
                BCU = BAV;
                BCY = BAX;
                BDB = BAY;
                BDI = BBA;
                BDL = BBB;
            } else {
                let BBC = if BAN > BAS { 1.0 } else { 0.0 };
                let BCG;
                let BCK;
                let BCV;
                let BCZ;
                let BDC;
                let BDJ;
                let BDM;
                if BBC != 0.0 {
                    let BBD = (BAN.abs()).sqrt();
                    let BBE = (-BBD).exp();
                    let BBF = (BBD * (D + BBE)) / (D - BBE);
                    let BBG = (ALG * BAP) / BAN;
                    let BBH = (BAN + (BBF * (HC - BBF))) * BBG;
                    let BBI = ((BAP - ((HC * BBH) * (D + BBF))) * BBG) + ((BBH * BAR) / BAP);
                    let BBJ = D - (F * BBF);
                    let BBK = (BAP / BAN) * BBJ;
                    let BBL = ((BAR * BBJ) - (BAP * (BBK + (F * BBH)))) / BAN;
                    BCG = BBE;
                    BCK = BBD;
                    BCV = BBF;
                    BCZ = BBH;
                    BDC = BBI;
                    BDJ = BBK;
                    BDM = BBL;
                } else {
                    let BBO = BAN * BBN;
                    let BBQ = ASH * (D - ((BAN * BBM) * (D - (BBO * (D - (BAN * BBP))))));
                    let BBR = HC + (BAN * BBQ);
                    let BBT = BAN * BBS;
                    let BBV = ASH * (D - (BBT * (D - ((BAN * BBU) * (D - BBT)))));
                    let BBW = BAP * BBV;
                    let BCA = (BAR * BBV) - ((BAP * BAP) * (BBX * (D - ((BAN * BBY) * (D - ((AAI * BAN) * (D - (BBZ * BAN))))))));
                    let BCB = (-5e-1f64 * BAP) * BBQ;
                    let BCD = ((-5e-1f64 * BAR) * BBQ) + (((1.3888888889e-3f64 * BAP) * BAP) * (D - (BBO * (HC - (BCC * BAN)))));
                    BCG = A;
                    BCK = A;
                    BCV = BBR;
                    BCZ = BBW;
                    BDC = BCA;
                    BDJ = BCB;
                    BDM = BCD;
                }
                BCF = BCG;
                BCJ = BCK;
                BCU = BCV;
                BCY = BCZ;
                BDB = BDC;
                BDI = BDJ;
                BDL = BDM;
            }
            let BCE = if BAN > BAS { 1.0 } else { 0.0 };
            let BDF;
            let BEC;
            if BCE != 0.0 {
                let BCH = (AJF * BAN) / (D - (BCF * (HC - BCF)));
                let BCI = BCH * BCF;
                let BCL = (BCH.ln()) - BCJ;
                BDF = BCI;
                BEC = BCL;
            } else {
                let BCM = if BAN < -5e-3f64 { 1.0 } else { 0.0 };
                let BDG;
                let BED;
                if BCM != 0.0 {
                    let BCN = (F * BCJ).sin();
                    let BCO = (-BAN) / (BCN * BCN);
                    let BCP = BCO.ln();
                    BDG = BCO;
                    BED = BCP;
                } else {
                    let BCR = AJF - ((BAN * KJ) * (D - ((AAI * BAN) * (D - (BCQ * BAN)))));
                    let BCS = BCR.ln();
                    BDG = BCR;
                    BED = BCS;
                }
                BDF = BDG;
                BEC = BED;
            }
            let BCW = if ((BCT * BAF) + BCU) > A { 1.0 } else { 0.0 };
            let BDO;
            let BDS;
            let BDU;
            if BCW != 0.0 {
                let BCX = BAF + BCU;
                let BDA = AYG + BCY;
                BDO = BCX;
                BDS = BDA;
                BDU = BDB;
            } else {
                let BDD = D / (BAF - BCU);
                let BDE = BCY - AYG;
                let BDH = (BAM - BDF) * BDD;
                let BDK = (((BDE * BDH) - BAM) - (BDI * BDF)) * BDD;
                let BDN = ((((BDB * BDH) + ((HC * BDE) * BDK)) + BAM) - ((BDL + (BDI * BDI)) * BDF)) * BDD;
                BDO = BDH;
                BDS = BDK;
                BDU = BDN;
            }
            let BDP = if BDO > A { 1.0 } else { 0.0 };
            let BEB;
            let BEE;
            let BEF;
            if BDP != 0.0 {
                let BDQ = BDO.ln();
                let BDR = D / BDO;
                let BDT = BDS * BDR;
                let BDV = (BDU * BDR) - (BDT * BDT);
                BEB = BDQ;
                BEE = BDT;
                BEF = BDV;
            } else {
                let BDW = (BAF + ACD) + ((-BAF).ln());
                let BDX = D / BAD;
                let BDY = AYG + BDX;
                let BDZ = (-BDX) * BDX;
                BEB = BDW;
                BEE = BDY;
                BEF = BDZ;
            }
            let BEA = AYF - AYB;
            let BEG = BAF + (AYH * (((BEA + BAD) + (HC * BEB)) - BEC));
            let BEH = AYG + (AYH * ((D + (HC * BEE)) - BDI));
            let BEI = (BEG * BDO) - BAM;
            let BEJ = ((BEH * BDO) + (BEG * BDS)) + BAM;
            let BEK = (BEJ * BEJ) - ((F * BEI) * (((((AYH * ((HC * BEF) - BDL)) * BDO) + ((HC * BEH) * BDS)) + (BEG * BDU)) - BAM));
            let BEM = BAD + ((((-BEI) * BEJ) * BEK) / ((BEK * BEK) + BEL));
            let BEN = AYG * BEM;
            let BEO = AYH * BAE;
            let BEP = BEN + BEO;
            let BER = D + (BEQ * BEP);
            let BEU = BEN * BEO;
            let BEV = (BES + (BET * BEP)) + BEU;
            let BEW = (BEN * BEN) - (((((BEV * BEV) - ((AJF * BER) * (BES * ((HC * BEP) + BEU)))).sqrt()) - BEV) / (HC * BER));
            let BEX = if BEW > A { 1.0 } else { 0.0 };
            let BFE;
            if BEX != 0.0 {
                let BEY = BEW * ((((BEW / AYN).ln()) - AYB) + BEM);
                let BEZ = (BAO * BEN) + BEW;
                let BFA = (AYB - BEM) - AZF;
                let BFC = if (if (if (if BEY < A { 1.0 } else { 0.0 }) != 0.0 && (if BEZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((BFA + BFB) + (AYG.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BFA > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BFF = if BFC != 0.0 {
                    let BFD = BEM - (BEY / BEZ);
                    BFD
                } else {
                    BEM
                };
                BFE = BFF;
            } else {
                BFE = BEM;
            }
            let BFG = AYG * BFE;
            let BFH = BFG + BEO;
            let BFI = D + (BEQ * BFH);
            let BFJ = BFG * BEO;
            let BFK = (BES + (BET * BFH)) + BFJ;
            let BFL = ((((BFK * BFK) - ((AJF * BFI) * (BES * ((HC * BFH) + BFJ)))).sqrt()) - BFK) / (HC * BFI);
            let BFM = if BFL < -5e-3f64 { 1.0 } else { 0.0 };
            let BFY;
            let BGA;
            let BIA;
            let BIH;
            if BFM != 0.0 {
                let BFN = (BFL.abs()).sqrt();
                let BFO = BFN / ((F * BFN).tan());
                let BFP = (ALG * (BFL + (BFO * (HC - BFO)))) / BFL;
                BFY = BFO;
                BGA = BFP;
                BIA = BCF;
                BIH = BFN;
            } else {
                let BFQ = if BFL > BAS { 1.0 } else { 0.0 };
                let BFZ;
                let BGB;
                let BIB;
                let BII;
                if BFQ != 0.0 {
                    let BFR = (BFL.abs()).sqrt();
                    let BFS = (-BFR).exp();
                    let BFT = (BFR * (D + BFS)) / (D - BFS);
                    let BFU = (ALG * (BFL + (BFT * (HC - BFT)))) / BFL;
                    BFZ = BFT;
                    BGB = BFU;
                    BIB = BFS;
                    BII = BFR;
                } else {
                    let BFV = HC + ((BFL * ASH) * (D - ((BFL * BBM) * (D - (BFL * BBN)))));
                    let BFW = BFL * BBS;
                    let BFX = ASH * (D - (BFW * (D - ((BFL * BBU) * (D - BFW)))));
                    BFZ = BFV;
                    BGB = BFX;
                    BIB = BCF;
                    BII = BCJ;
                }
                BFY = BFZ;
                BGA = BGB;
                BIA = BIB;
                BIH = BII;
            }
            let BGC = (BFG * BFG) - (BFL - ((((BFH * BFY) + BFJ) + BFL) / ((BFH * BGA) + D)));
            let BGD = if BGC > A { 1.0 } else { 0.0 };
            let BGJ;
            if BGD != 0.0 {
                let BGE = BGC * ((((BGC / AYN).ln()) - AYB) + BFE);
                let BGF = (BAO * BFG) + BGC;
                let BGG = (AYB - BFE) - AZF;
                let BGH = if (if (if (if BGE < A { 1.0 } else { 0.0 }) != 0.0 && (if BGF > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((BGG + BFB) + (AYG.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BGG > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BGK = if BGH != 0.0 {
                    let BGI = BFE - (BGE / BGF);
                    BGI
                } else {
                    BFE
                };
                BGJ = BGK;
            } else {
                BGJ = BFE;
            }
            let BGL = AYG * BGJ;
            let BGM = AYB - BGJ;
            let BGN = if BGM < KE { 1.0 } else { 0.0 };
            let BGR = if BGN != 0.0 {
                let BGO = BGM.exp();
                BGO
            } else {
                let BGP = BGM - KE;
                let BGQ = TP * (D + (BGP * (D + ((F * BGP) * (D + (BGP * KJ))))));
                BGQ
            };
            let BGS = AYN * BGR;
            let BGT = (BGL * BGL) - BGS;
            let BGU = (BAO * BGL) + BGS;
            let BGV = BAQ - BGS;
            let BGW = if BGT < -5e-3f64 { 1.0 } else { 0.0 };
            let BHZ;
            let BIF;
            let BIQ;
            let BIU;
            let BIX;
            let BJE;
            let BJH;
            if BGW != 0.0 {
                let BGX = (BGT.abs()).sqrt();
                let BGY = BGX / ((F * BGX).tan());
                let BGZ = (ALG * BGU) / BGT;
                let BHA = (BGT + (BGY * (HC - BGY))) * BGZ;
                let BHB = ((BGU - ((HC * BHA) * (D + BGY))) * BGZ) + ((BHA * BGV) / BGU);
                let BHC = D - (F * BGY);
                let BHD = (BGU / BGT) * BHC;
                let BHE = ((BGV * BHC) - (BGU * (BHD + (F * BHA)))) / BGT;
                BHZ = BIA;
                BIF = BGX;
                BIQ = BGY;
                BIU = BHA;
                BIX = BHB;
                BJE = BHD;
                BJH = BHE;
            } else {
                let BHF = if BGT > BAS { 1.0 } else { 0.0 };
                let BIC;
                let BIG;
                let BIR;
                let BIV;
                let BIY;
                let BJF;
                let BJI;
                if BHF != 0.0 {
                    let BHG = (BGT.abs()).sqrt();
                    let BHH = (-BHG).exp();
                    let BHI = (BHG * (D + BHH)) / (D - BHH);
                    let BHJ = (ALG * BGU) / BGT;
                    let BHK = (BGT + (BHI * (HC - BHI))) * BHJ;
                    let BHL = ((BGU - ((HC * BHK) * (D + BHI))) * BHJ) + ((BHK * BGV) / BGU);
                    let BHM = D - (F * BHI);
                    let BHN = (BGU / BGT) * BHM;
                    let BHO = ((BGV * BHM) - (BGU * (BHN + (F * BHK)))) / BGT;
                    BIC = BHH;
                    BIG = BHG;
                    BIR = BHI;
                    BIV = BHK;
                    BIY = BHL;
                    BJF = BHN;
                    BJI = BHO;
                } else {
                    let BHP = BGT * BBN;
                    let BHQ = ASH * (D - ((BGT * BBM) * (D - (BHP * (D - (BGT * BBP))))));
                    let BHR = HC + (BGT * BHQ);
                    let BHS = BGT * BBS;
                    let BHT = ASH * (D - (BHS * (D - ((BGT * BBU) * (D - BHS)))));
                    let BHU = BGU * BHT;
                    let BHV = (BGV * BHT) - ((BGU * BGU) * (BBX * (D - ((BGT * BBY) * (D - ((AAI * BGT) * (D - (BBZ * BGT))))))));
                    let BHW = (-5e-1f64 * BGU) * BHQ;
                    let BHX = ((-5e-1f64 * BGV) * BHQ) + (((1.3888888889e-3f64 * BGU) * BGU) * (D - (BHP * (HC - (BCC * BGT)))));
                    BIC = BIA;
                    BIG = BIH;
                    BIR = BHR;
                    BIV = BHU;
                    BIY = BHV;
                    BJF = BHW;
                    BJI = BHX;
                }
                BHZ = BIC;
                BIF = BIG;
                BIQ = BIR;
                BIU = BIV;
                BIX = BIY;
                BJE = BJF;
                BJH = BJI;
            }
            let BHY = if BGT > BAS { 1.0 } else { 0.0 };
            let BJB;
            let BJX;
            if BHY != 0.0 {
                let BID = (AJF * BGT) / (D - (BHZ * (HC - BHZ)));
                let BIE = BID * BHZ;
                let BIJ = (BID.ln()) - BIF;
                BJB = BIE;
                BJX = BIJ;
            } else {
                let BIK = if BGT < -5e-3f64 { 1.0 } else { 0.0 };
                let BJC;
                let BJY;
                if BIK != 0.0 {
                    let BIL = (F * BIF).sin();
                    let BIM = (-BGT) / (BIL * BIL);
                    let BIN = BIM.ln();
                    BJC = BIM;
                    BJY = BIN;
                } else {
                    let BIO = AJF - ((BGT * KJ) * (D - ((AAI * BGT) * (D - (BCQ * BGT)))));
                    let BIP = BIO.ln();
                    BJC = BIO;
                    BJY = BIP;
                }
                BJB = BJC;
                BJX = BJY;
            }
            let BIS = if ((BCT * BGL) + BIQ) > A { 1.0 } else { 0.0 };
            let BJK;
            let BJO;
            let BJQ;
            if BIS != 0.0 {
                let BIT = BGL + BIQ;
                let BIW = AYG + BIU;
                BJK = BIT;
                BJO = BIW;
                BJQ = BIX;
            } else {
                let BIZ = D / (BGL - BIQ);
                let BJA = BIU - AYG;
                let BJD = (BGS - BJB) * BIZ;
                let BJG = (((BJA * BJD) - BGS) - (BJE * BJB)) * BIZ;
                let BJJ = ((((BIX * BJD) + ((HC * BJA) * BJG)) + BGS) - ((BJH + (BJE * BJE)) * BJB)) * BIZ;
                BJK = BJD;
                BJO = BJG;
                BJQ = BJJ;
            }
            let BJL = if BJK > A { 1.0 } else { 0.0 };
            let BJW;
            let BJZ;
            let BKA;
            if BJL != 0.0 {
                let BJM = BJK.ln();
                let BJN = D / BJK;
                let BJP = BJO * BJN;
                let BJR = (BJQ * BJN) - (BJP * BJP);
                BJW = BJM;
                BJZ = BJP;
                BKA = BJR;
            } else {
                let BJS = (BGL + ACD) + ((-BGL).ln());
                let BJT = D / BGJ;
                let BJU = AYG + BJT;
                let BJV = (-BJT) * BJT;
                BJW = BJS;
                BJZ = BJU;
                BKA = BJV;
            }
            let BKB = BGL + (AYH * (((BEA + BGJ) + (HC * BJW)) - BJX));
            let BKC = AYG + (AYH * ((D + (HC * BJZ)) - BJE));
            let BKD = (BKB * BJK) - BGS;
            let BKE = ((BKC * BJK) + (BKB * BJO)) + BGS;
            let BKF = (BKE * BKE) - ((F * BKD) * (((((AYH * ((HC * BKA) - BJH)) * BJK) + ((HC * BKC) * BJO)) + (BKB * BJQ)) - BGS));
            let BKG = BGJ + ((((-BKD) * BKE) * BKF) / ((BKF * BKF) + BEL));
            let BKH = AYG * BKG;
            let BKI = AYB - BKG;
            let BKJ = if BKI < KE { 1.0 } else { 0.0 };
            let BKN = if BKJ != 0.0 {
                let BKK = BKI.exp();
                BKK
            } else {
                let BKL = BKI - KE;
                let BKM = TP * (D + (BKL * (D + ((F * BKL) * (D + (BKL * KJ))))));
                BKM
            };
            let BKO = AYN * BKN;
            let BKP = (BKH * BKH) - BKO;
            let BKQ = (BAO * BKH) + BKO;
            let BKR = BAQ - BKO;
            let BKS = if BKP < -5e-3f64 { 1.0 } else { 0.0 };
            let BLV;
            let BLZ;
            let BMI;
            let BMM;
            let BMP;
            let BMW;
            let BMZ;
            if BKS != 0.0 {
                let BKT = (BKP.abs()).sqrt();
                let BKU = BKT / ((F * BKT).tan());
                let BKV = (ALG * BKQ) / BKP;
                let BKW = (BKP + (BKU * (HC - BKU))) * BKV;
                let BKX = ((BKQ - ((HC * BKW) * (D + BKU))) * BKV) + ((BKW * BKR) / BKQ);
                let BKY = D - (F * BKU);
                let BKZ = (BKQ / BKP) * BKY;
                let BLA = ((BKR * BKY) - (BKQ * (BKZ + (F * BKW)))) / BKP;
                BLV = BHZ;
                BLZ = BKT;
                BMI = BKU;
                BMM = BKW;
                BMP = BKX;
                BMW = BKZ;
                BMZ = BLA;
            } else {
                let BLB = if BKP > BAS { 1.0 } else { 0.0 };
                let BLW;
                let BMA;
                let BMJ;
                let BMN;
                let BMQ;
                let BMX;
                let BNA;
                if BLB != 0.0 {
                    let BLC = (BKP.abs()).sqrt();
                    let BLD = (-BLC).exp();
                    let BLE = (BLC * (D + BLD)) / (D - BLD);
                    let BLF = (ALG * BKQ) / BKP;
                    let BLG = (BKP + (BLE * (HC - BLE))) * BLF;
                    let BLH = ((BKQ - ((HC * BLG) * (D + BLE))) * BLF) + ((BLG * BKR) / BKQ);
                    let BLI = D - (F * BLE);
                    let BLJ = (BKQ / BKP) * BLI;
                    let BLK = ((BKR * BLI) - (BKQ * (BLJ + (F * BLG)))) / BKP;
                    BLW = BLD;
                    BMA = BLC;
                    BMJ = BLE;
                    BMN = BLG;
                    BMQ = BLH;
                    BMX = BLJ;
                    BNA = BLK;
                } else {
                    let BLL = BKP * BBN;
                    let BLM = ASH * (D - ((BKP * BBM) * (D - (BLL * (D - (BKP * BBP))))));
                    let BLN = HC + (BKP * BLM);
                    let BLO = BKP * BBS;
                    let BLP = ASH * (D - (BLO * (D - ((BKP * BBU) * (D - BLO)))));
                    let BLQ = BKQ * BLP;
                    let BLR = (BKR * BLP) - ((BKQ * BKQ) * (BBX * (D - ((BKP * BBY) * (D - ((AAI * BKP) * (D - (BBZ * BKP))))))));
                    let BLS = (-5e-1f64 * BKQ) * BLM;
                    let BLT = ((-5e-1f64 * BKR) * BLM) + (((1.3888888889e-3f64 * BKQ) * BKQ) * (D - (BLL * (HC - (BCC * BKP)))));
                    BLW = BHZ;
                    BMA = BIF;
                    BMJ = BLN;
                    BMN = BLQ;
                    BMQ = BLR;
                    BMX = BLS;
                    BNA = BLT;
                }
                BLV = BLW;
                BLZ = BMA;
                BMI = BMJ;
                BMM = BMN;
                BMP = BMQ;
                BMW = BMX;
                BMZ = BNA;
            }
            let BLU = if BKP > BAS { 1.0 } else { 0.0 };
            let BMT;
            let BNP;
            if BLU != 0.0 {
                let BLX = (AJF * BKP) / (D - (BLV * (HC - BLV)));
                let BLY = BLX * BLV;
                let BMB = (BLX.ln()) - BLZ;
                BMT = BLY;
                BNP = BMB;
            } else {
                let BMC = if BKP < -5e-3f64 { 1.0 } else { 0.0 };
                let BMU;
                let BNQ;
                if BMC != 0.0 {
                    let BMD = (F * BLZ).sin();
                    let BME = (-BKP) / (BMD * BMD);
                    let BMF = BME.ln();
                    BMU = BME;
                    BNQ = BMF;
                } else {
                    let BMG = AJF - ((BKP * KJ) * (D - ((AAI * BKP) * (D - (BCQ * BKP)))));
                    let BMH = BMG.ln();
                    BMU = BMG;
                    BNQ = BMH;
                }
                BMT = BMU;
                BNP = BNQ;
            }
            let BMK = if ((BCT * BKH) + BMI) > A { 1.0 } else { 0.0 };
            let BNC;
            let BNG;
            let BNI;
            if BMK != 0.0 {
                let BML = BKH + BMI;
                let BMO = AYG + BMM;
                BNC = BML;
                BNG = BMO;
                BNI = BMP;
            } else {
                let BMR = D / (BKH - BMI);
                let BMS = BMM - AYG;
                let BMV = (BKO - BMT) * BMR;
                let BMY = (((BMS * BMV) - BKO) - (BMW * BMT)) * BMR;
                let BNB = ((((BMP * BMV) + ((HC * BMS) * BMY)) + BKO) - ((BMZ + (BMW * BMW)) * BMT)) * BMR;
                BNC = BMV;
                BNG = BMY;
                BNI = BNB;
            }
            let BND = if BNC > A { 1.0 } else { 0.0 };
            let BNO;
            let BNR;
            let BNS;
            if BND != 0.0 {
                let BNE = BNC.ln();
                let BNF = D / BNC;
                let BNH = BNG * BNF;
                let BNJ = (BNI * BNF) - (BNH * BNH);
                BNO = BNE;
                BNR = BNH;
                BNS = BNJ;
            } else {
                let BNK = (BKH + ACD) + ((-BKH).ln());
                let BNL = D / BKG;
                let BNM = AYG + BNL;
                let BNN = (-BNL) * BNL;
                BNO = BNK;
                BNR = BNM;
                BNS = BNN;
            }
            let BNT = BKH + (AYH * (((BEA + BKG) + (HC * BNO)) - BNP));
            let BNU = AYG + (AYH * ((D + (HC * BNR)) - BMW));
            let BNV = (BNT * BNC) - BKO;
            let BNW = ((BNU * BNC) + (BNT * BNG)) + BKO;
            let BNX = (BNW * BNW) - ((F * BNV) * (((((AYH * ((HC * BNS) - BMZ)) * BNC) + ((HC * BNU) * BNG)) + (BNT * BNI)) - BKO));
            let BNY = (((-BNV) * BNW) * BNX) / ((BNX * BNX) + BEL);
            let BNZ = BKG + BNY;
            let BRT;
            let BTB;
            let BTG;
            if E != 0.0 {
                let BOA = if (BNY.abs()) > O { 1.0 } else { 0.0 };
                let BRU;
                let BTC;
                let BTH;
                if BOA != 0.0 {
                    let BOB = AYG * BNZ;
                    let BOC = AYB - BNZ;
                    let BOD = if BOC < KE { 1.0 } else { 0.0 };
                    let BOH = if BOD != 0.0 {
                        let BOE = BOC.exp();
                        BOE
                    } else {
                        let BOF = BOC - KE;
                        let BOG = TP * (D + (BOF * (D + ((F * BOF) * (D + (BOF * KJ))))));
                        BOG
                    };
                    let BOI = AYN * BOH;
                    let BOJ = (BOB * BOB) - BOI;
                    let BOK = (BAO * BOB) + BOI;
                    let BOL = BAQ - BOI;
                    let BOM = if BOJ < -5e-3f64 { 1.0 } else { 0.0 };
                    let BPP;
                    let BPT;
                    let BQC;
                    let BQG;
                    let BQJ;
                    let BQQ;
                    let BQT;
                    if BOM != 0.0 {
                        let BON = (BOJ.abs()).sqrt();
                        let BOO = BON / ((F * BON).tan());
                        let BOP = (ALG * BOK) / BOJ;
                        let BOQ = (BOJ + (BOO * (HC - BOO))) * BOP;
                        let BOR = ((BOK - ((HC * BOQ) * (D + BOO))) * BOP) + ((BOQ * BOL) / BOK);
                        let BOS = D - (F * BOO);
                        let BOT = (BOK / BOJ) * BOS;
                        let BOU = ((BOL * BOS) - (BOK * (BOT + (F * BOQ)))) / BOJ;
                        BPP = BLV;
                        BPT = BON;
                        BQC = BOO;
                        BQG = BOQ;
                        BQJ = BOR;
                        BQQ = BOT;
                        BQT = BOU;
                    } else {
                        let BOV = if BOJ > BAS { 1.0 } else { 0.0 };
                        let BPQ;
                        let BPU;
                        let BQD;
                        let BQH;
                        let BQK;
                        let BQR;
                        let BQU;
                        if BOV != 0.0 {
                            let BOW = (BOJ.abs()).sqrt();
                            let BOX = (-BOW).exp();
                            let BOY = (BOW * (D + BOX)) / (D - BOX);
                            let BOZ = (ALG * BOK) / BOJ;
                            let BPA = (BOJ + (BOY * (HC - BOY))) * BOZ;
                            let BPB = ((BOK - ((HC * BPA) * (D + BOY))) * BOZ) + ((BPA * BOL) / BOK);
                            let BPC = D - (F * BOY);
                            let BPD = (BOK / BOJ) * BPC;
                            let BPE = ((BOL * BPC) - (BOK * (BPD + (F * BPA)))) / BOJ;
                            BPQ = BOX;
                            BPU = BOW;
                            BQD = BOY;
                            BQH = BPA;
                            BQK = BPB;
                            BQR = BPD;
                            BQU = BPE;
                        } else {
                            let BPF = BOJ * BBN;
                            let BPG = ASH * (D - ((BOJ * BBM) * (D - (BPF * (D - (BOJ * BBP))))));
                            let BPH = HC + (BOJ * BPG);
                            let BPI = BOJ * BBS;
                            let BPJ = ASH * (D - (BPI * (D - ((BOJ * BBU) * (D - BPI)))));
                            let BPK = BOK * BPJ;
                            let BPL = (BOL * BPJ) - ((BOK * BOK) * (BBX * (D - ((BOJ * BBY) * (D - ((AAI * BOJ) * (D - (BBZ * BOJ))))))));
                            let BPM = (-5e-1f64 * BOK) * BPG;
                            let BPN = ((-5e-1f64 * BOL) * BPG) + (((1.3888888889e-3f64 * BOK) * BOK) * (D - (BPF * (HC - (BCC * BOJ)))));
                            BPQ = BLV;
                            BPU = BLZ;
                            BQD = BPH;
                            BQH = BPK;
                            BQK = BPL;
                            BQR = BPM;
                            BQU = BPN;
                        }
                        BPP = BPQ;
                        BPT = BPU;
                        BQC = BQD;
                        BQG = BQH;
                        BQJ = BQK;
                        BQQ = BQR;
                        BQT = BQU;
                    }
                    let BPO = if BOJ > BAS { 1.0 } else { 0.0 };
                    let BQN;
                    let BRJ;
                    if BPO != 0.0 {
                        let BPR = (AJF * BOJ) / (D - (BPP * (HC - BPP)));
                        let BPS = BPR * BPP;
                        let BPV = (BPR.ln()) - BPT;
                        BQN = BPS;
                        BRJ = BPV;
                    } else {
                        let BPW = if BOJ < -5e-3f64 { 1.0 } else { 0.0 };
                        let BQO;
                        let BRK;
                        if BPW != 0.0 {
                            let BPX = (F * BPT).sin();
                            let BPY = (-BOJ) / (BPX * BPX);
                            let BPZ = BPY.ln();
                            BQO = BPY;
                            BRK = BPZ;
                        } else {
                            let BQA = AJF - ((BOJ * KJ) * (D - ((AAI * BOJ) * (D - (BCQ * BOJ)))));
                            let BQB = BQA.ln();
                            BQO = BQA;
                            BRK = BQB;
                        }
                        BQN = BQO;
                        BRJ = BRK;
                    }
                    let BQE = if ((BCT * BOB) + BQC) > A { 1.0 } else { 0.0 };
                    let BQW;
                    let BRA;
                    let BRC;
                    if BQE != 0.0 {
                        let BQF = BOB + BQC;
                        let BQI = AYG + BQG;
                        BQW = BQF;
                        BRA = BQI;
                        BRC = BQJ;
                    } else {
                        let BQL = D / (BOB - BQC);
                        let BQM = BQG - AYG;
                        let BQP = (BOI - BQN) * BQL;
                        let BQS = (((BQM * BQP) - BOI) - (BQQ * BQN)) * BQL;
                        let BQV = ((((BQJ * BQP) + ((HC * BQM) * BQS)) + BOI) - ((BQT + (BQQ * BQQ)) * BQN)) * BQL;
                        BQW = BQP;
                        BRA = BQS;
                        BRC = BQV;
                    }
                    let BQX = if BQW > A { 1.0 } else { 0.0 };
                    let BRI;
                    let BRL;
                    let BRM;
                    if BQX != 0.0 {
                        let BQY = BQW.ln();
                        let BQZ = D / BQW;
                        let BRB = BRA * BQZ;
                        let BRD = (BRC * BQZ) - (BRB * BRB);
                        BRI = BQY;
                        BRL = BRB;
                        BRM = BRD;
                    } else {
                        let BRE = (BOB + ACD) + ((-BOB).ln());
                        let BRF = D / BNZ;
                        let BRG = AYG + BRF;
                        let BRH = (-BRF) * BRF;
                        BRI = BRE;
                        BRL = BRG;
                        BRM = BRH;
                    }
                    let BRN = BOB + (AYH * (((BEA + BNZ) + (HC * BRI)) - BRJ));
                    let BRO = AYG + (AYH * ((D + (HC * BRL)) - BQQ));
                    let BRP = (BRN * BQW) - BOI;
                    let BRQ = ((BRO * BQW) + (BRN * BRA)) + BOI;
                    let BRR = (BRQ * BRQ) - ((F * BRP) * (((((AYH * ((HC * BRM) - BQT)) * BQW) + ((HC * BRO) * BRA)) + (BRN * BRC)) - BOI));
                    let BRS = BNZ + ((((-BRP) * BRQ) * BRR) / ((BRR * BRR) + BEL));
                    BRU = BRS;
                    BTC = BPP;
                    BTH = BPT;
                } else {
                    BRU = BNZ;
                    BTC = BLV;
                    BTH = BLZ;
                }
                BRT = BRU;
                BTB = BTC;
                BTG = BTH;
            } else {
                BRT = BNZ;
                BTB = BLV;
                BTG = BLZ;
            }
            let BRV = AYG * BRT;
            let BRW = AYB - BRT;
            let BRX = if BRW < KE { 1.0 } else { 0.0 };
            let BSB = if BRX != 0.0 {
                let BRY = BRW.exp();
                BRY
            } else {
                let BRZ = BRW - KE;
                let BSA = TP * (D + (BRZ * (D + ((F * BRZ) * (D + (BRZ * KJ))))));
                BSA
            };
            let BSC = AYN * BSB;
            let BSD = (BRV * BRV) - BSC;
            let BSE = if BSC <= A { 1.0 } else { 0.0 };
            let BUJ;
            let BUT;
            let BVA;
            if BSE != 0.0 {
                let BSG = BSF - BRV;
                let BSH = BSG / AYH;
                BUJ = BSH;
                BUT = BSF;
                BVA = BSG;
            } else {
                let BSI = if BSD < -5e-3f64 { 1.0 } else { 0.0 };
                let BSQ;
                let BTA;
                let BTE;
                if BSI != 0.0 {
                    let BSJ = (BSD.abs()).sqrt();
                    let BSK = BSJ / ((F * BSJ).tan());
                    BSQ = BSK;
                    BTA = BTB;
                    BTE = BSJ;
                } else {
                    let BSL = if BSD > BAS { 1.0 } else { 0.0 };
                    let BSR;
                    let BTD;
                    let BTF;
                    if BSL != 0.0 {
                        let BSM = (BSD.abs()).sqrt();
                        let BSN = (-BSM).exp();
                        let BSO = (BSM * (D + BSN)) / (D - BSN);
                        BSR = BSO;
                        BTD = BSN;
                        BTF = BSM;
                    } else {
                        let BSP = HC + ((BSD * ASH) * (D - ((BSD * BBM) * (D - (BSD * BBN)))));
                        BSR = BSP;
                        BTD = BTB;
                        BTF = BTG;
                    }
                    BSQ = BSR;
                    BTA = BTD;
                    BTE = BTF;
                }
                let BSS = if ((BCT * BRV) + BSQ) > A { 1.0 } else { 0.0 };
                let BUK;
                let BUU;
                let BVB;
                if BSS != 0.0 {
                    let BST = BRV + BSQ;
                    let BSV = if (BSC * BRV) < (((BSU * BRV) * BRV) * BST) { 1.0 } else { 0.0 };
                    let BUL;
                    let BUV;
                    let BVC;
                    if BSV != 0.0 {
                        let BSW = (BSC / BST) + BSF;
                        let BSX = BSW - BRV;
                        let BSY = BSX / AYH;
                        BUL = BSY;
                        BUV = BSW;
                        BVC = BSX;
                    } else {
                        let BSZ = if BSD > BAS { 1.0 } else { 0.0 };
                        let BTN;
                        if BSZ != 0.0 {
                            let BTI = (((AJF * BSD) / (D - (BTA * (HC - BTA)))).ln()) - BTE;
                            BTN = BTI;
                        } else {
                            let BTJ = if BSD < -5e-3f64 { 1.0 } else { 0.0 };
                            let BTO = if BTJ != 0.0 {
                                let BTK = (F * BTE).sin();
                                let BTL = ((-BSD) / (BTK * BTK)).ln();
                                BTL
                            } else {
                                let BTM = (AJF - ((BSD * KJ) * (D - ((AAI * BSD) * (D - (BCQ * BSD)))))).ln();
                                BTM
                            };
                            BTN = BTO;
                        }
                        let BTP = ((BEA + BRT) + (HC * (BST.ln()))) - BTN;
                        let BTQ = AYH * BTP;
                        let BTR = BRV + BTQ;
                        BUL = BTP;
                        BUV = BTR;
                        BVC = BTQ;
                    }
                    BUK = BUL;
                    BUU = BUV;
                    BVB = BVC;
                } else {
                    let BTS = if BSD > BAS { 1.0 } else { 0.0 };
                    let BUE;
                    if BTS != 0.0 {
                        let BTT = (BRT - AYB) - BTE;
                        let BTU = if BTT < KE { 1.0 } else { 0.0 };
                        let BTY = if BTU != 0.0 {
                            let BTV = BTT.exp();
                            BTV
                        } else {
                            let BTW = BTT - KE;
                            let BTX = TP * (D + (BTW * (D + ((F * BTW) * (D + (BTW * KJ))))));
                            BTX
                        };
                        let BTZ = ((AJF * BSD) * (BTY / AYN)) / (D - (BTA * (HC - BTA)));
                        BUE = BTZ;
                    } else {
                        let BUA = if BSD < -5e-3f64 { 1.0 } else { 0.0 };
                        let BUF = if BUA != 0.0 {
                            let BUB = (F * BTE).sin();
                            let BUC = ((-BSD) / (BUB * BUB)) / BSC;
                            BUC
                        } else {
                            let BUD = (AJF - ((BSD * KJ) * (D - ((AAI * BSD) * (D - (BCQ * BSD)))))) / BSC;
                            BUD
                        };
                        BUE = BUF;
                    }
                    let BUG = ((BRV - BSQ) / (D - BUE)) + BSF;
                    let BUH = BUG - BRV;
                    let BUI = BUH / AYH;
                    BUK = BUI;
                    BUU = BUG;
                    BVB = BUH;
                }
                BUJ = BUK;
                BUT = BUU;
                BVA = BVB;
            }
            let BUM = AYF - BUJ;
            let BUN = if BUM < KE { 1.0 } else { 0.0 };
            let BUR = if BUN != 0.0 {
                let BUO = BUM.exp();
                BUO
            } else {
                let BUP = BUM - KE;
                let BUQ = TP * (D + (BUP * (D + ((F * BUP) * (D + (BUP * KJ))))));
                BUQ
            };
            let BUS = AYN * BUR;
            let BUW = if BUT > GW { 1.0 } else { 0.0 };
            let BYW;
            let BYY;
            let BYZ;
            let BZA;
            if BUW != 0.0 {
                let BUX = BSC * AYI;
                let BUY = BUS * AYJ;
                let BUZ = BUX + (HC * BRV);
                let BVD = BUY + (HC * BVA);
                let BVE = ((HC * BUT) + BUX) + BUY;
                let BVF = if (BSD.abs()) > BAS { 1.0 } else { 0.0 };
                let BYX = if BVF != 0.0 {
                    let BVG = ((-4e0f64 * BSD) * BVE) / (BUT * (((BUZ * BVD) + ((HC * (BRT + HC)) * BVD)) + ((HC * (BUJ + HC)) * BUZ)));
                    BVG
                } else {
                    let BVH = BSD * BBS;
                    let BVI = ((BSC * BUS) * BVE) / (BUT * (((BUZ * BSC) + (BVD * BUS)) + (((BUZ * BVD) * BUT) * (D + (BUT * (ASH * (D - (BVH * (D - ((BSD * BBU) * (D - BVH)))))))))));
                    BVI
                };
                BYW = BYX;
                BYY = BVE;
                BYZ = BUZ;
                BZA = BVD;
            } else {
                BYW = A;
                BYY = A;
                BYZ = A;
                BZA = A;
            }
            let BVJ = BUT.ln();
            let BVK = BRV / HC;
            let BVL = if BVK < KE { 1.0 } else { 0.0 };
            let BVN = if BVL != 0.0 {
                let BVM = (D + (BVK.exp())).ln();
                BVM
            } else {
                BVK
            };
            let BVO = HC * BVN;
            let BVP = BVA / HC;
            let BVQ = if BVP < KE { 1.0 } else { 0.0 };
            let BVS = if BVQ != 0.0 {
                let BVR = (D + (BVP.exp())).ln();
                BVR
            } else {
                BVP
            };
            let BVT = HC * BVS;
            let BVU = BVT - BVA;
            let BVV = BVO - BRV;
            let BVW = (AHH * BVO) + (AHI * BVU);
            let BVX = (AHH * BVT) + (AHI * BVV);
            let BVY = BUT / (BVO + BVT);
            let BWA = (AFY * BVZ).exp();
            let BWB = (BVO * AGB) * BWA;
            let BWC = (BVT * AGF) * BWA;
            let BWF = BWD * (BVU + (BWE * BVV));
            let BWG = D + BWF;
            let BWI = D + (BWH * BWF);
            let BWJ = (F * (BWG + (((BWG * BWG) + O).sqrt()))) / (F * (BWI + (((BWI * BWI) + O).sqrt())));
            let BWO = -BWN;
            let BWP = (BWK * ((D + (BWL * BVU)) + (BWM * BVV))) * ((BWO * (((D + ((BVO * BVY) * AHA)) + ((BVT * BVY) * AHC)).ln())).exp());
            let BWR = if BWQ == A { 1.0 } else { 0.0 };
            let BXB;
            if BWR != 0.0 {
                BXB = D;
            } else {
                let BWS = if BWQ < A { 1.0 } else { 0.0 };
                let BXC = if BWS != 0.0 {
                    let BWV = D - (BWQ * ((BWT * ((BUT + BWU).ln())).exp()));
                    BWV
                } else {
                    let BWW = D / (D + (BWQ * ((BWT * ((BUT + BWU).ln())).exp())));
                    BWW
                };
                BXB = BXC;
            }
            let BWZ = D - (BWY * AQX);
            let BXA = ((BWX * AYL) * F) * (BWZ + (((BWZ * BWZ) + O).sqrt()));
            let BXE = BXA * ((BUT * BXB) + BXD);
            let BXJ = (BWJ * (BWB + BWC)) / ((BWB / (((D + ((BXF * (((BXG * BVW) + GW).ln())).exp())) + BWP) + (BXH * BXE))) + (BWC / (((D + ((BXF * (((BXG * BVX) + GW).ln())).exp())) + BWP) + (BXI * BXE))));
            let BXL = if (AYV.abs()) > BXK { 1.0 } else { 0.0 };
            let BYJ;
            let BZE;
            let DAV;
            let DAW;
            let DAY;
            let DAZ;
            if BXL != 0.0 {
                let BXM = if AYV > A { 1.0 } else { 0.0 };
                let BXV;
                let BXX;
                let BZF;
                if BXM != 0.0 {
                    let BXN = (-AYV).exp();
                    let BXO = AYV / (D - BXN);
                    let BXP = BXN * BXO;
                    let BXQ = (((AYN / (BUT * BXO)).ln()) - ACD) + AYY;
                    BXV = BXO;
                    BXX = BXP;
                    BZF = BXQ;
                } else {
                    let BXR = AYV.exp();
                    let BXS = AYV / (BXR - D);
                    let BXT = BXR * BXS;
                    let BXU = (((AYN / (BUT * BXS)).ln()) - ACD) + AZA;
                    BXV = BXT;
                    BXX = BXS;
                    BZF = BXU;
                }
                let BXW = (-AYV) / (AYK * ((D - BXV) - AYZ));
                let BXY = AYV / (AYK * ((D - BXX) + AYX));
                let BXZ = AYV / ((((BXX * AYJ) + F) / BXY) - (((BXV * AYI) + F) / BXW));
                BYJ = BXZ;
                BZE = BZF;
                DAV = BXV;
                DAW = BXW;
                DAY = BXX;
                DAZ = BXY;
            } else {
                let BYA = 8.333333333335e-2f64 * AYW;
                let BYB = F * AYV;
                let BYC = (D + BYB) + BYA;
                let BYD = (D - BYB) + BYA;
                let BYE = ASH * BYB;
                let BYF = D / (AYK * ((F + AYJ) + BYE));
                let BYG = D / (AYK * ((F + AYI) - BYE));
                let BYH = (((AYN / (BUT * (D - (F * BYA)))).ln()) - ACD) + (F * (AYY + AZA));
                let BYI = -1.2e1f64 / ((((AJF - (ATY * AYK)) + ((ATJ * AYK) / (AYG * AYH))) + ((AYK * (AYI - AYJ)) * AYV)) + ((KJ * (BWH - (ALG * AYK))) * AYW));
                BYJ = BYI;
                BZE = BYH;
                DAV = BYC;
                DAW = BYF;
                DAY = BYD;
                DAZ = BYG;
            }
            let BYK = D / BYJ;
            let CAL;
            let CAQ;
            let DAD;
            if BUW != 0.0 {
                let BYM = (BYL * BVO) / (BYL + BVO);
                let BYO = if BYN < A { 1.0 } else { 0.0 };
                let BZH = if BYO != 0.0 {
                    let BYP = D / (D - (BYN * BYM));
                    BYP
                } else {
                    let BYQ = D + (BYN * BYM);
                    BYQ
                };
                let BYR = (BYL * BVT) / (BYL + BVT);
                let BYT = if BYS < A { 1.0 } else { 0.0 };
                let BZI = if BYT != 0.0 {
                    let BYU = D / (D - (BYS * BYR));
                    BYU
                } else {
                    let BYV = D + (BYS * BYR);
                    BYV
                };
                let BZB = ((BYW * BYY) / (BYZ * BZA)) - (((BSC / BYZ) + (BUS / BZA)) / BUT);
                let BZC = (BZB * BUT) / (BZB + D);
                let BZD = BYJ - BZC;
                let BZG = (BUT + (BYJ * BZE)) / BZD;
                let BZJ = ((AQS / BXJ) * F) * (BZH + BZI);
                let BZK = D - (BUT / BZC);
                let BZL = D + BZE;
                let BZM = (((((HC * BZC) - BUT) * BYK) - HC) - BZE) * (F * (BZG + (((BZG * BZG) + GW).sqrt())));
                let BZO = if BZJ > BZN { 1.0 } else { 0.0 };
                let CAA;
                let CAB;
                if BZO != 0.0 {
                    let BZP = HC / (BZJ * BZJ);
                    let BZQ = BZP * BZK;
                    let BZR = BZP + BZM;
                    let BZS = BZP * BZL;
                    let BZU = (((BZQ * BZQ) + (((BZT * BZP) * BZP) * BZP)) + VV).sqrt();
                    let BZV = (((BZS * BZS) + (((BZT * BZR) * BZR) * BZR)) + VV).sqrt();
                    let BZW = ((KJ * ((F * (BZU + BZQ)).ln())).exp()) - ((KJ * ((F * (BZU - BZQ)).ln())).exp());
                    let BZX = ((KJ * ((F * (BZV + BZS)).ln())).exp()) - ((KJ * ((F * (BZV - BZS)).ln())).exp());
                    CAA = BZW;
                    CAB = BZX;
                } else {
                    CAA = BZK;
                    CAB = BZL;
                }
                let BZY = BZD * BZD;
                let CAC = CAA - CAB;
                let CAD = 4.7e-1f64 * ((CAA + CAB) + (((CAC * CAC) + (J * BZY)).sqrt()));
                let CAE = BUT + (BZC * CAD);
                let CAF = BYJ * (CAD - BZE);
                let CAG = CAE - CAF;
                let CAI = F * ((CAE + CAF) + (((CAG * CAG) + (CAH * BZY)).sqrt()));
                CAL = CAI;
                CAQ = CAD;
                DAD = BZC;
            } else {
                let CAJ = BZZ * (D + BZE);
                let CAK = (F * BUT) + (BYJ * (CAJ - (F * BZE)));
                CAL = CAK;
                CAQ = CAJ;
                DAD = BYJ;
            }
            let CAM = CAL - F;
            let CAN = if CAM < KE { 1.0 } else { 0.0 };
            let CAP = if CAN != 0.0 {
                let CAO = (D + (CAM.exp())).ln();
                CAO
            } else {
                CAM
            };
            let CAR = (CAQ + ((BUT / (CAP + F)).ln())) - ASP;
            let CAS = if CAR < KE { 1.0 } else { 0.0 };
            let CAU = if CAS != 0.0 {
                let CAT = (D + (CAR.exp())).ln();
                CAT
            } else {
                CAR
            };
            let CAV = AXX - (CAU + ASP);
            let CAW = if CAV < KE { 1.0 } else { 0.0 };
            let CAY = if CAW != 0.0 {
                let CAX = (D + (CAV.exp())).ln();
                CAX
            } else {
                CAV
            };
            let CAZ = AQI / (AXX - CAY);
            let CBA = CAZ * CAZ;
            let CBB = CBA * CBA;
            let CBC = CBB * CBB;
            let CBE = AQI * ((-6.25e-2f64 * ((((CBD * ((D + (AHO * CBB)).ln())).exp()) + (CBC * CBC)).ln())).exp());
            let CBF = (AZE + CBE) + ATY;
            let CBG = (AZG + CBE) + ATY;
            let CBH = (CBF - AYY) * KJ;
            let CBI = if CBH < KE { 1.0 } else { 0.0 };
            let CBK = if CBI != 0.0 {
                let CBJ = (D + (CBH.exp())).ln();
                CBJ
            } else {
                CBH
            };
            let CBL = CBF - (ATY * CBK);
            let CBM = (CBG - AZA) * KJ;
            let CBN = if CBM < KE { 1.0 } else { 0.0 };
            let CBP = if CBN != 0.0 {
                let CBO = (D + (CBM.exp())).ln();
                CBO
            } else {
                CBM
            };
            let CBQ = (AZS + CBL) * AZC;
            let CBR = (CBF - ((AZR + (CBG - (ATY * CBP))) * AZB)) * KJ;
            let CBS = if CBR < KE { 1.0 } else { 0.0 };
            let CBU = if CBS != 0.0 {
                let CBT = (D + (CBR.exp())).ln();
                CBT
            } else {
                CBR
            };
            let CBV = CBF - (ATY * CBU);
            let CBW = (CBG - CBQ) * KJ;
            let CBX = if CBW < KE { 1.0 } else { 0.0 };
            let CBZ = if CBX != 0.0 {
                let CBY = (D + (CBW.exp())).ln();
                CBY
            } else {
                CBW
            };
            let CCA = AYB - CBV;
            let CCB = AYF - (CBG - (ATY * CBZ));
            let CCC = AYG * CCA;
            let CCD = (AYB - CCA) - CBE;
            let CCE = if CCD < KE { 1.0 } else { 0.0 };
            let CCI = if CCE != 0.0 {
                let CCF = CCD.exp();
                CCF
            } else {
                let CCG = CCD - KE;
                let CCH = TP * (D + (CCG * (D + ((F * CCG) * (D + (CCG * KJ))))));
                CCH
            };
            let CCJ = AYN * CCI;
            let CCK = (CCC * CCC) - CCJ;
            let CCL = (BAO * CCC) + CCJ;
            let CCM = BAQ - CCJ;
            let CCN = if CCK < -5e-3f64 { 1.0 } else { 0.0 };
            let CDQ;
            let CDU;
            let CED;
            let CEH;
            let CEK;
            let CER;
            let CEU;
            if CCN != 0.0 {
                let CCO = (CCK.abs()).sqrt();
                let CCP = CCO / ((F * CCO).tan());
                let CCQ = (ALG * CCL) / CCK;
                let CCR = (CCK + (CCP * (HC - CCP))) * CCQ;
                let CCS = ((CCL - ((HC * CCR) * (D + CCP))) * CCQ) + ((CCR * CCM) / CCL);
                let CCT = D - (F * CCP);
                let CCU = (CCL / CCK) * CCT;
                let CCV = ((CCM * CCT) - (CCL * (CCU + (F * CCR)))) / CCK;
                CDQ = A;
                CDU = CCO;
                CED = CCP;
                CEH = CCR;
                CEK = CCS;
                CER = CCU;
                CEU = CCV;
            } else {
                let CCW = if CCK > BAS { 1.0 } else { 0.0 };
                let CDR;
                let CDV;
                let CEE;
                let CEI;
                let CEL;
                let CES;
                let CEV;
                if CCW != 0.0 {
                    let CCX = (CCK.abs()).sqrt();
                    let CCY = (-CCX).exp();
                    let CCZ = (CCX * (D + CCY)) / (D - CCY);
                    let CDA = (ALG * CCL) / CCK;
                    let CDB = (CCK + (CCZ * (HC - CCZ))) * CDA;
                    let CDC = ((CCL - ((HC * CDB) * (D + CCZ))) * CDA) + ((CDB * CCM) / CCL);
                    let CDD = D - (F * CCZ);
                    let CDE = (CCL / CCK) * CDD;
                    let CDF = ((CCM * CDD) - (CCL * (CDE + (F * CDB)))) / CCK;
                    CDR = CCY;
                    CDV = CCX;
                    CEE = CCZ;
                    CEI = CDB;
                    CEL = CDC;
                    CES = CDE;
                    CEV = CDF;
                } else {
                    let CDG = CCK * BBN;
                    let CDH = ASH * (D - ((CCK * BBM) * (D - (CDG * (D - (CCK * BBP))))));
                    let CDI = HC + (CCK * CDH);
                    let CDJ = CCK * BBS;
                    let CDK = ASH * (D - (CDJ * (D - ((CCK * BBU) * (D - CDJ)))));
                    let CDL = CCL * CDK;
                    let CDM = (CCM * CDK) - ((CCL * CCL) * (BBX * (D - ((CCK * BBY) * (D - ((AAI * CCK) * (D - (BBZ * CCK))))))));
                    let CDN = (-5e-1f64 * CCL) * CDH;
                    let CDO = ((-5e-1f64 * CCM) * CDH) + (((1.3888888889e-3f64 * CCL) * CCL) * (D - (CDG * (HC - (BCC * CCK)))));
                    CDR = A;
                    CDV = A;
                    CEE = CDI;
                    CEI = CDL;
                    CEL = CDM;
                    CES = CDN;
                    CEV = CDO;
                }
                CDQ = CDR;
                CDU = CDV;
                CED = CEE;
                CEH = CEI;
                CEK = CEL;
                CER = CES;
                CEU = CEV;
            }
            let CDP = if CCK > BAS { 1.0 } else { 0.0 };
            let CEO;
            let CFK;
            if CDP != 0.0 {
                let CDS = (AJF * CCK) / (D - (CDQ * (HC - CDQ)));
                let CDT = CDS * CDQ;
                let CDW = (CDS.ln()) - CDU;
                CEO = CDT;
                CFK = CDW;
            } else {
                let CDX = if CCK < -5e-3f64 { 1.0 } else { 0.0 };
                let CEP;
                let CFL;
                if CDX != 0.0 {
                    let CDY = (F * CDU).sin();
                    let CDZ = (-CCK) / (CDY * CDY);
                    let CEA = CDZ.ln();
                    CEP = CDZ;
                    CFL = CEA;
                } else {
                    let CEB = AJF - ((CCK * KJ) * (D - ((AAI * CCK) * (D - (BCQ * CCK)))));
                    let CEC = CEB.ln();
                    CEP = CEB;
                    CFL = CEC;
                }
                CEO = CEP;
                CFK = CFL;
            }
            let CEF = if ((BCT * CCC) + CED) > A { 1.0 } else { 0.0 };
            let CEX;
            let CFB;
            let CFD;
            if CEF != 0.0 {
                let CEG = CCC + CED;
                let CEJ = AYG + CEH;
                CEX = CEG;
                CFB = CEJ;
                CFD = CEK;
            } else {
                let CEM = D / (CCC - CED);
                let CEN = CEH - AYG;
                let CEQ = (CCJ - CEO) * CEM;
                let CET = (((CEN * CEQ) - CCJ) - (CER * CEO)) * CEM;
                let CEW = ((((CEK * CEQ) + ((HC * CEN) * CET)) + CCJ) - ((CEU + (CER * CER)) * CEO)) * CEM;
                CEX = CEQ;
                CFB = CET;
                CFD = CEW;
            }
            let CEY = if CEX > A { 1.0 } else { 0.0 };
            let CFJ;
            let CFM;
            let CFN;
            if CEY != 0.0 {
                let CEZ = CEX.ln();
                let CFA = D / CEX;
                let CFC = CFB * CFA;
                let CFE = (CFD * CFA) - (CFC * CFC);
                CFJ = CEZ;
                CFM = CFC;
                CFN = CFE;
            } else {
                let CFF = (CCC + ACD) + ((-CCC).ln());
                let CFG = D / CCA;
                let CFH = AYG + CFG;
                let CFI = (-CFG) * CFG;
                CFJ = CFF;
                CFM = CFH;
                CFN = CFI;
            }
            let CFO = CCC + (AYH * (((BEA + CCA) + (HC * CFJ)) - CFK));
            let CFP = AYG + (AYH * ((D + (HC * CFM)) - CER));
            let CFQ = (CFO * CEX) - CCJ;
            let CFR = ((CFP * CEX) + (CFO * CFB)) + CCJ;
            let CFS = (CFR * CFR) - ((F * CFQ) * (((((AYH * ((HC * CFN) - CEU)) * CEX) + ((HC * CFP) * CFB)) + (CFO * CFD)) - CCJ));
            let CFT = CCA + ((((-CFQ) * CFR) * CFS) / ((CFS * CFS) + BEL));
            let CFU = AYG * CFT;
            let CFV = AYH * CCB;
            let CFW = CFU + CFV;
            let CFX = D + (BEQ * CFW);
            let CFY = CFU * CFV;
            let CFZ = (BES + (BET * CFW)) + CFY;
            let CGA = (CFU * CFU) - (((((CFZ * CFZ) - ((AJF * CFX) * (BES * ((HC * CFW) + CFY)))).sqrt()) - CFZ) / (HC * CFX));
            let CGB = if CGA > A { 1.0 } else { 0.0 };
            let CGH;
            if CGB != 0.0 {
                let CGC = CGA * (((((CGA / AYN).ln()) + CBE) - AYB) + CFT);
                let CGD = (BAO * CFU) + CGA;
                let CGE = (AYB - CFT) - CBF;
                let CGF = if (if (if (if CGC < A { 1.0 } else { 0.0 }) != 0.0 && (if CGD > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((CGE + BFB) + (AYG.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if CGE > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CGI = if CGF != 0.0 {
                    let CGG = CFT - (CGC / CGD);
                    CGG
                } else {
                    CFT
                };
                CGH = CGI;
            } else {
                CGH = CFT;
            }
            let CGJ = AYG * CGH;
            let CGK = CGJ + CFV;
            let CGL = D + (BEQ * CGK);
            let CGM = CGJ * CFV;
            let CGN = (BES + (BET * CGK)) + CGM;
            let CGO = ((((CGN * CGN) - ((AJF * CGL) * (BES * ((HC * CGK) + CGM)))).sqrt()) - CGN) / (HC * CGL);
            let CGP = if CGO < -5e-3f64 { 1.0 } else { 0.0 };
            let CHB;
            let CHD;
            let CJD;
            let CJK;
            if CGP != 0.0 {
                let CGQ = (CGO.abs()).sqrt();
                let CGR = CGQ / ((F * CGQ).tan());
                let CGS = (ALG * (CGO + (CGR * (HC - CGR)))) / CGO;
                CHB = CGR;
                CHD = CGS;
                CJD = CDQ;
                CJK = CGQ;
            } else {
                let CGT = if CGO > BAS { 1.0 } else { 0.0 };
                let CHC;
                let CHE;
                let CJE;
                let CJL;
                if CGT != 0.0 {
                    let CGU = (CGO.abs()).sqrt();
                    let CGV = (-CGU).exp();
                    let CGW = (CGU * (D + CGV)) / (D - CGV);
                    let CGX = (ALG * (CGO + (CGW * (HC - CGW)))) / CGO;
                    CHC = CGW;
                    CHE = CGX;
                    CJE = CGV;
                    CJL = CGU;
                } else {
                    let CGY = HC + ((CGO * ASH) * (D - ((CGO * BBM) * (D - (CGO * BBN)))));
                    let CGZ = CGO * BBS;
                    let CHA = ASH * (D - (CGZ * (D - ((CGO * BBU) * (D - CGZ)))));
                    CHC = CGY;
                    CHE = CHA;
                    CJE = CDQ;
                    CJL = CDU;
                }
                CHB = CHC;
                CHD = CHE;
                CJD = CJE;
                CJK = CJL;
            }
            let CHF = (CGJ * CGJ) - (CGO - ((((CGK * CHB) + CGM) + CGO) / ((CGK * CHD) + D)));
            let CHG = if CHF > A { 1.0 } else { 0.0 };
            let CHM;
            if CHG != 0.0 {
                let CHH = CHF * (((((CHF / AYN).ln()) + CBE) - AYB) + CGH);
                let CHI = (BAO * CGJ) + CHF;
                let CHJ = (AYB - CGH) - CBF;
                let CHK = if (if (if (if CHH < A { 1.0 } else { 0.0 }) != 0.0 && (if CHI > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((CHJ + BFB) + (AYG.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if CHJ > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CHN = if CHK != 0.0 {
                    let CHL = CGH - (CHH / CHI);
                    CHL
                } else {
                    CGH
                };
                CHM = CHN;
            } else {
                CHM = CGH;
            }
            let CHO = AYG * CHM;
            let CHP = (AYB - CHM) - CBE;
            let CHQ = if CHP < KE { 1.0 } else { 0.0 };
            let CHU = if CHQ != 0.0 {
                let CHR = CHP.exp();
                CHR
            } else {
                let CHS = CHP - KE;
                let CHT = TP * (D + (CHS * (D + ((F * CHS) * (D + (CHS * KJ))))));
                CHT
            };
            let CHV = AYN * CHU;
            let CHW = (CHO * CHO) - CHV;
            let CHX = (BAO * CHO) + CHV;
            let CHY = BAQ - CHV;
            let CHZ = if CHW < -5e-3f64 { 1.0 } else { 0.0 };
            let CJC;
            let CJI;
            let CJT;
            let CJX;
            let CKA;
            let CKH;
            let CKK;
            if CHZ != 0.0 {
                let CIA = (CHW.abs()).sqrt();
                let CIB = CIA / ((F * CIA).tan());
                let CIC = (ALG * CHX) / CHW;
                let CID = (CHW + (CIB * (HC - CIB))) * CIC;
                let CIE = ((CHX - ((HC * CID) * (D + CIB))) * CIC) + ((CID * CHY) / CHX);
                let CIF = D - (F * CIB);
                let CIG = (CHX / CHW) * CIF;
                let CIH = ((CHY * CIF) - (CHX * (CIG + (F * CID)))) / CHW;
                CJC = CJD;
                CJI = CIA;
                CJT = CIB;
                CJX = CID;
                CKA = CIE;
                CKH = CIG;
                CKK = CIH;
            } else {
                let CII = if CHW > BAS { 1.0 } else { 0.0 };
                let CJF;
                let CJJ;
                let CJU;
                let CJY;
                let CKB;
                let CKI;
                let CKL;
                if CII != 0.0 {
                    let CIJ = (CHW.abs()).sqrt();
                    let CIK = (-CIJ).exp();
                    let CIL = (CIJ * (D + CIK)) / (D - CIK);
                    let CIM = (ALG * CHX) / CHW;
                    let CIN = (CHW + (CIL * (HC - CIL))) * CIM;
                    let CIO = ((CHX - ((HC * CIN) * (D + CIL))) * CIM) + ((CIN * CHY) / CHX);
                    let CIP = D - (F * CIL);
                    let CIQ = (CHX / CHW) * CIP;
                    let CIR = ((CHY * CIP) - (CHX * (CIQ + (F * CIN)))) / CHW;
                    CJF = CIK;
                    CJJ = CIJ;
                    CJU = CIL;
                    CJY = CIN;
                    CKB = CIO;
                    CKI = CIQ;
                    CKL = CIR;
                } else {
                    let CIS = CHW * BBN;
                    let CIT = ASH * (D - ((CHW * BBM) * (D - (CIS * (D - (CHW * BBP))))));
                    let CIU = HC + (CHW * CIT);
                    let CIV = CHW * BBS;
                    let CIW = ASH * (D - (CIV * (D - ((CHW * BBU) * (D - CIV)))));
                    let CIX = CHX * CIW;
                    let CIY = (CHY * CIW) - ((CHX * CHX) * (BBX * (D - ((CHW * BBY) * (D - ((AAI * CHW) * (D - (BBZ * CHW))))))));
                    let CIZ = (-5e-1f64 * CHX) * CIT;
                    let CJA = ((-5e-1f64 * CHY) * CIT) + (((1.3888888889e-3f64 * CHX) * CHX) * (D - (CIS * (HC - (BCC * CHW)))));
                    CJF = CJD;
                    CJJ = CJK;
                    CJU = CIU;
                    CJY = CIX;
                    CKB = CIY;
                    CKI = CIZ;
                    CKL = CJA;
                }
                CJC = CJF;
                CJI = CJJ;
                CJT = CJU;
                CJX = CJY;
                CKA = CKB;
                CKH = CKI;
                CKK = CKL;
            }
            let CJB = if CHW > BAS { 1.0 } else { 0.0 };
            let CKE;
            let CLA;
            if CJB != 0.0 {
                let CJG = (AJF * CHW) / (D - (CJC * (HC - CJC)));
                let CJH = CJG * CJC;
                let CJM = (CJG.ln()) - CJI;
                CKE = CJH;
                CLA = CJM;
            } else {
                let CJN = if CHW < -5e-3f64 { 1.0 } else { 0.0 };
                let CKF;
                let CLB;
                if CJN != 0.0 {
                    let CJO = (F * CJI).sin();
                    let CJP = (-CHW) / (CJO * CJO);
                    let CJQ = CJP.ln();
                    CKF = CJP;
                    CLB = CJQ;
                } else {
                    let CJR = AJF - ((CHW * KJ) * (D - ((AAI * CHW) * (D - (BCQ * CHW)))));
                    let CJS = CJR.ln();
                    CKF = CJR;
                    CLB = CJS;
                }
                CKE = CKF;
                CLA = CLB;
            }
            let CJV = if ((BCT * CHO) + CJT) > A { 1.0 } else { 0.0 };
            let CKN;
            let CKR;
            let CKT;
            if CJV != 0.0 {
                let CJW = CHO + CJT;
                let CJZ = AYG + CJX;
                CKN = CJW;
                CKR = CJZ;
                CKT = CKA;
            } else {
                let CKC = D / (CHO - CJT);
                let CKD = CJX - AYG;
                let CKG = (CHV - CKE) * CKC;
                let CKJ = (((CKD * CKG) - CHV) - (CKH * CKE)) * CKC;
                let CKM = ((((CKA * CKG) + ((HC * CKD) * CKJ)) + CHV) - ((CKK + (CKH * CKH)) * CKE)) * CKC;
                CKN = CKG;
                CKR = CKJ;
                CKT = CKM;
            }
            let CKO = if CKN > A { 1.0 } else { 0.0 };
            let CKZ;
            let CLC;
            let CLD;
            if CKO != 0.0 {
                let CKP = CKN.ln();
                let CKQ = D / CKN;
                let CKS = CKR * CKQ;
                let CKU = (CKT * CKQ) - (CKS * CKS);
                CKZ = CKP;
                CLC = CKS;
                CLD = CKU;
            } else {
                let CKV = (CHO + ACD) + ((-CHO).ln());
                let CKW = D / CHM;
                let CKX = AYG + CKW;
                let CKY = (-CKW) * CKW;
                CKZ = CKV;
                CLC = CKX;
                CLD = CKY;
            }
            let CLE = CHO + (AYH * (((BEA + CHM) + (HC * CKZ)) - CLA));
            let CLF = AYG + (AYH * ((D + (HC * CLC)) - CKH));
            let CLG = (CLE * CKN) - CHV;
            let CLH = ((CLF * CKN) + (CLE * CKR)) + CHV;
            let CLI = (CLH * CLH) - ((F * CLG) * (((((AYH * ((HC * CLD) - CKK)) * CKN) + ((HC * CLF) * CKR)) + (CLE * CKT)) - CHV));
            let CLJ = CHM + ((((-CLG) * CLH) * CLI) / ((CLI * CLI) + BEL));
            let CLK = AYG * CLJ;
            let CLL = (AYB - CLJ) - CBE;
            let CLM = if CLL < KE { 1.0 } else { 0.0 };
            let CLQ = if CLM != 0.0 {
                let CLN = CLL.exp();
                CLN
            } else {
                let CLO = CLL - KE;
                let CLP = TP * (D + (CLO * (D + ((F * CLO) * (D + (CLO * KJ))))));
                CLP
            };
            let CLR = AYN * CLQ;
            let CLS = (CLK * CLK) - CLR;
            let CLT = (BAO * CLK) + CLR;
            let CLU = BAQ - CLR;
            let CLV = if CLS < -5e-3f64 { 1.0 } else { 0.0 };
            let CMY;
            let CNC;
            let CNL;
            let CNP;
            let CNS;
            let CNZ;
            let COC;
            if CLV != 0.0 {
                let CLW = (CLS.abs()).sqrt();
                let CLX = CLW / ((F * CLW).tan());
                let CLY = (ALG * CLT) / CLS;
                let CLZ = (CLS + (CLX * (HC - CLX))) * CLY;
                let CMA = ((CLT - ((HC * CLZ) * (D + CLX))) * CLY) + ((CLZ * CLU) / CLT);
                let CMB = D - (F * CLX);
                let CMC = (CLT / CLS) * CMB;
                let CMD = ((CLU * CMB) - (CLT * (CMC + (F * CLZ)))) / CLS;
                CMY = CJC;
                CNC = CLW;
                CNL = CLX;
                CNP = CLZ;
                CNS = CMA;
                CNZ = CMC;
                COC = CMD;
            } else {
                let CME = if CLS > BAS { 1.0 } else { 0.0 };
                let CMZ;
                let CND;
                let CNM;
                let CNQ;
                let CNT;
                let COA;
                let COD;
                if CME != 0.0 {
                    let CMF = (CLS.abs()).sqrt();
                    let CMG = (-CMF).exp();
                    let CMH = (CMF * (D + CMG)) / (D - CMG);
                    let CMI = (ALG * CLT) / CLS;
                    let CMJ = (CLS + (CMH * (HC - CMH))) * CMI;
                    let CMK = ((CLT - ((HC * CMJ) * (D + CMH))) * CMI) + ((CMJ * CLU) / CLT);
                    let CML = D - (F * CMH);
                    let CMM = (CLT / CLS) * CML;
                    let CMN = ((CLU * CML) - (CLT * (CMM + (F * CMJ)))) / CLS;
                    CMZ = CMG;
                    CND = CMF;
                    CNM = CMH;
                    CNQ = CMJ;
                    CNT = CMK;
                    COA = CMM;
                    COD = CMN;
                } else {
                    let CMO = CLS * BBN;
                    let CMP = ASH * (D - ((CLS * BBM) * (D - (CMO * (D - (CLS * BBP))))));
                    let CMQ = HC + (CLS * CMP);
                    let CMR = CLS * BBS;
                    let CMS = ASH * (D - (CMR * (D - ((CLS * BBU) * (D - CMR)))));
                    let CMT = CLT * CMS;
                    let CMU = (CLU * CMS) - ((CLT * CLT) * (BBX * (D - ((CLS * BBY) * (D - ((AAI * CLS) * (D - (BBZ * CLS))))))));
                    let CMV = (-5e-1f64 * CLT) * CMP;
                    let CMW = ((-5e-1f64 * CLU) * CMP) + (((1.3888888889e-3f64 * CLT) * CLT) * (D - (CMO * (HC - (BCC * CLS)))));
                    CMZ = CJC;
                    CND = CJI;
                    CNM = CMQ;
                    CNQ = CMT;
                    CNT = CMU;
                    COA = CMV;
                    COD = CMW;
                }
                CMY = CMZ;
                CNC = CND;
                CNL = CNM;
                CNP = CNQ;
                CNS = CNT;
                CNZ = COA;
                COC = COD;
            }
            let CMX = if CLS > BAS { 1.0 } else { 0.0 };
            let CNW;
            let COS;
            if CMX != 0.0 {
                let CNA = (AJF * CLS) / (D - (CMY * (HC - CMY)));
                let CNB = CNA * CMY;
                let CNE = (CNA.ln()) - CNC;
                CNW = CNB;
                COS = CNE;
            } else {
                let CNF = if CLS < -5e-3f64 { 1.0 } else { 0.0 };
                let CNX;
                let COT;
                if CNF != 0.0 {
                    let CNG = (F * CNC).sin();
                    let CNH = (-CLS) / (CNG * CNG);
                    let CNI = CNH.ln();
                    CNX = CNH;
                    COT = CNI;
                } else {
                    let CNJ = AJF - ((CLS * KJ) * (D - ((AAI * CLS) * (D - (BCQ * CLS)))));
                    let CNK = CNJ.ln();
                    CNX = CNJ;
                    COT = CNK;
                }
                CNW = CNX;
                COS = COT;
            }
            let CNN = if ((BCT * CLK) + CNL) > A { 1.0 } else { 0.0 };
            let COF;
            let COJ;
            let COL;
            if CNN != 0.0 {
                let CNO = CLK + CNL;
                let CNR = AYG + CNP;
                COF = CNO;
                COJ = CNR;
                COL = CNS;
            } else {
                let CNU = D / (CLK - CNL);
                let CNV = CNP - AYG;
                let CNY = (CLR - CNW) * CNU;
                let COB = (((CNV * CNY) - CLR) - (CNZ * CNW)) * CNU;
                let COE = ((((CNS * CNY) + ((HC * CNV) * COB)) + CLR) - ((COC + (CNZ * CNZ)) * CNW)) * CNU;
                COF = CNY;
                COJ = COB;
                COL = COE;
            }
            let COG = if COF > A { 1.0 } else { 0.0 };
            let COR;
            let COU;
            let COV;
            if COG != 0.0 {
                let COH = COF.ln();
                let COI = D / COF;
                let COK = COJ * COI;
                let COM = (COL * COI) - (COK * COK);
                COR = COH;
                COU = COK;
                COV = COM;
            } else {
                let CON = (CLK + ACD) + ((-CLK).ln());
                let COO = D / CLJ;
                let COP = AYG + COO;
                let COQ = (-COO) * COO;
                COR = CON;
                COU = COP;
                COV = COQ;
            }
            let COW = CLK + (AYH * (((BEA + CLJ) + (HC * COR)) - COS));
            let COX = AYG + (AYH * ((D + (HC * COU)) - CNZ));
            let COY = (COW * COF) - CLR;
            let COZ = ((COX * COF) + (COW * COJ)) + CLR;
            let CPA = (COZ * COZ) - ((F * COY) * (((((AYH * ((HC * COV) - COC)) * COF) + ((HC * COX) * COJ)) + (COW * COL)) - CLR));
            let CPB = (((-COY) * COZ) * CPA) / ((CPA * CPA) + BEL);
            let CPC = CLJ + CPB;
            let CSW;
            let CUC;
            let CUH;
            if E != 0.0 {
                let CPD = if (CPB.abs()) > O { 1.0 } else { 0.0 };
                let CSX;
                let CUD;
                let CUI;
                if CPD != 0.0 {
                    let CPE = AYG * CPC;
                    let CPF = (AYB - CPC) - CBE;
                    let CPG = if CPF < KE { 1.0 } else { 0.0 };
                    let CPK = if CPG != 0.0 {
                        let CPH = CPF.exp();
                        CPH
                    } else {
                        let CPI = CPF - KE;
                        let CPJ = TP * (D + (CPI * (D + ((F * CPI) * (D + (CPI * KJ))))));
                        CPJ
                    };
                    let CPL = AYN * CPK;
                    let CPM = (CPE * CPE) - CPL;
                    let CPN = (BAO * CPE) + CPL;
                    let CPO = BAQ - CPL;
                    let CPP = if CPM < -5e-3f64 { 1.0 } else { 0.0 };
                    let CQS;
                    let CQW;
                    let CRF;
                    let CRJ;
                    let CRM;
                    let CRT;
                    let CRW;
                    if CPP != 0.0 {
                        let CPQ = (CPM.abs()).sqrt();
                        let CPR = CPQ / ((F * CPQ).tan());
                        let CPS = (ALG * CPN) / CPM;
                        let CPT = (CPM + (CPR * (HC - CPR))) * CPS;
                        let CPU = ((CPN - ((HC * CPT) * (D + CPR))) * CPS) + ((CPT * CPO) / CPN);
                        let CPV = D - (F * CPR);
                        let CPW = (CPN / CPM) * CPV;
                        let CPX = ((CPO * CPV) - (CPN * (CPW + (F * CPT)))) / CPM;
                        CQS = CMY;
                        CQW = CPQ;
                        CRF = CPR;
                        CRJ = CPT;
                        CRM = CPU;
                        CRT = CPW;
                        CRW = CPX;
                    } else {
                        let CPY = if CPM > BAS { 1.0 } else { 0.0 };
                        let CQT;
                        let CQX;
                        let CRG;
                        let CRK;
                        let CRN;
                        let CRU;
                        let CRX;
                        if CPY != 0.0 {
                            let CPZ = (CPM.abs()).sqrt();
                            let CQA = (-CPZ).exp();
                            let CQB = (CPZ * (D + CQA)) / (D - CQA);
                            let CQC = (ALG * CPN) / CPM;
                            let CQD = (CPM + (CQB * (HC - CQB))) * CQC;
                            let CQE = ((CPN - ((HC * CQD) * (D + CQB))) * CQC) + ((CQD * CPO) / CPN);
                            let CQF = D - (F * CQB);
                            let CQG = (CPN / CPM) * CQF;
                            let CQH = ((CPO * CQF) - (CPN * (CQG + (F * CQD)))) / CPM;
                            CQT = CQA;
                            CQX = CPZ;
                            CRG = CQB;
                            CRK = CQD;
                            CRN = CQE;
                            CRU = CQG;
                            CRX = CQH;
                        } else {
                            let CQI = CPM * BBN;
                            let CQJ = ASH * (D - ((CPM * BBM) * (D - (CQI * (D - (CPM * BBP))))));
                            let CQK = HC + (CPM * CQJ);
                            let CQL = CPM * BBS;
                            let CQM = ASH * (D - (CQL * (D - ((CPM * BBU) * (D - CQL)))));
                            let CQN = CPN * CQM;
                            let CQO = (CPO * CQM) - ((CPN * CPN) * (BBX * (D - ((CPM * BBY) * (D - ((AAI * CPM) * (D - (BBZ * CPM))))))));
                            let CQP = (-5e-1f64 * CPN) * CQJ;
                            let CQQ = ((-5e-1f64 * CPO) * CQJ) + (((1.3888888889e-3f64 * CPN) * CPN) * (D - (CQI * (HC - (BCC * CPM)))));
                            CQT = CMY;
                            CQX = CNC;
                            CRG = CQK;
                            CRK = CQN;
                            CRN = CQO;
                            CRU = CQP;
                            CRX = CQQ;
                        }
                        CQS = CQT;
                        CQW = CQX;
                        CRF = CRG;
                        CRJ = CRK;
                        CRM = CRN;
                        CRT = CRU;
                        CRW = CRX;
                    }
                    let CQR = if CPM > BAS { 1.0 } else { 0.0 };
                    let CRQ;
                    let CSM;
                    if CQR != 0.0 {
                        let CQU = (AJF * CPM) / (D - (CQS * (HC - CQS)));
                        let CQV = CQU * CQS;
                        let CQY = (CQU.ln()) - CQW;
                        CRQ = CQV;
                        CSM = CQY;
                    } else {
                        let CQZ = if CPM < -5e-3f64 { 1.0 } else { 0.0 };
                        let CRR;
                        let CSN;
                        if CQZ != 0.0 {
                            let CRA = (F * CQW).sin();
                            let CRB = (-CPM) / (CRA * CRA);
                            let CRC = CRB.ln();
                            CRR = CRB;
                            CSN = CRC;
                        } else {
                            let CRD = AJF - ((CPM * KJ) * (D - ((AAI * CPM) * (D - (BCQ * CPM)))));
                            let CRE = CRD.ln();
                            CRR = CRD;
                            CSN = CRE;
                        }
                        CRQ = CRR;
                        CSM = CSN;
                    }
                    let CRH = if ((BCT * CPE) + CRF) > A { 1.0 } else { 0.0 };
                    let CRZ;
                    let CSD;
                    let CSF;
                    if CRH != 0.0 {
                        let CRI = CPE + CRF;
                        let CRL = AYG + CRJ;
                        CRZ = CRI;
                        CSD = CRL;
                        CSF = CRM;
                    } else {
                        let CRO = D / (CPE - CRF);
                        let CRP = CRJ - AYG;
                        let CRS = (CPL - CRQ) * CRO;
                        let CRV = (((CRP * CRS) - CPL) - (CRT * CRQ)) * CRO;
                        let CRY = ((((CRM * CRS) + ((HC * CRP) * CRV)) + CPL) - ((CRW + (CRT * CRT)) * CRQ)) * CRO;
                        CRZ = CRS;
                        CSD = CRV;
                        CSF = CRY;
                    }
                    let CSA = if CRZ > A { 1.0 } else { 0.0 };
                    let CSL;
                    let CSO;
                    let CSP;
                    if CSA != 0.0 {
                        let CSB = CRZ.ln();
                        let CSC = D / CRZ;
                        let CSE = CSD * CSC;
                        let CSG = (CSF * CSC) - (CSE * CSE);
                        CSL = CSB;
                        CSO = CSE;
                        CSP = CSG;
                    } else {
                        let CSH = (CPE + ACD) + ((-CPE).ln());
                        let CSI = D / CPC;
                        let CSJ = AYG + CSI;
                        let CSK = (-CSI) * CSI;
                        CSL = CSH;
                        CSO = CSJ;
                        CSP = CSK;
                    }
                    let CSQ = CPE + (AYH * (((BEA + CPC) + (HC * CSL)) - CSM));
                    let CSR = AYG + (AYH * ((D + (HC * CSO)) - CRT));
                    let CSS = (CSQ * CRZ) - CPL;
                    let CST = ((CSR * CRZ) + (CSQ * CSD)) + CPL;
                    let CSU = (CST * CST) - ((F * CSS) * (((((AYH * ((HC * CSP) - CRW)) * CRZ) + ((HC * CSR) * CSD)) + (CSQ * CSF)) - CPL));
                    let CSV = CPC + ((((-CSS) * CST) * CSU) / ((CSU * CSU) + BEL));
                    CSX = CSV;
                    CUD = CQS;
                    CUI = CQW;
                } else {
                    CSX = CPC;
                    CUD = CMY;
                    CUI = CNC;
                }
                CSW = CSX;
                CUC = CUD;
                CUH = CUI;
            } else {
                CSW = CPC;
                CUC = CMY;
                CUH = CNC;
            }
            let CSY = AYG * CSW;
            let CSZ = (AYB - CSW) - CBE;
            let CTA = if CSZ < KE { 1.0 } else { 0.0 };
            let CTE = if CTA != 0.0 {
                let CTB = CSZ.exp();
                CTB
            } else {
                let CTC = CSZ - KE;
                let CTD = TP * (D + (CTC * (D + ((F * CTC) * (D + (CTC * KJ))))));
                CTD
            };
            let CTF = AYN * CTE;
            let CTG = (CSY * CSY) - CTF;
            let CTH = if CTF <= A { 1.0 } else { 0.0 };
            let CVK;
            let CVX;
            let CWB;
            if CTH != 0.0 {
                let CTI = BSF - CSY;
                let CTJ = CTI / AYH;
                CVK = CTJ;
                CVX = CTI;
                CWB = BSF;
            } else {
                let CTK = if CTG < -5e-3f64 { 1.0 } else { 0.0 };
                let CTS;
                let CUB;
                let CUF;
                if CTK != 0.0 {
                    let CTL = (CTG.abs()).sqrt();
                    let CTM = CTL / ((F * CTL).tan());
                    CTS = CTM;
                    CUB = CUC;
                    CUF = CTL;
                } else {
                    let CTN = if CTG > BAS { 1.0 } else { 0.0 };
                    let CTT;
                    let CUE;
                    let CUG;
                    if CTN != 0.0 {
                        let CTO = (CTG.abs()).sqrt();
                        let CTP = (-CTO).exp();
                        let CTQ = (CTO * (D + CTP)) / (D - CTP);
                        CTT = CTQ;
                        CUE = CTP;
                        CUG = CTO;
                    } else {
                        let CTR = HC + ((CTG * ASH) * (D - ((CTG * BBM) * (D - (CTG * BBN)))));
                        CTT = CTR;
                        CUE = CUC;
                        CUG = CUH;
                    }
                    CTS = CTT;
                    CUB = CUE;
                    CUF = CUG;
                }
                let CTU = if ((BCT * CSY) + CTS) > A { 1.0 } else { 0.0 };
                let CVL;
                let CVY;
                let CWC;
                if CTU != 0.0 {
                    let CTV = CSY + CTS;
                    let CTW = if (CTF * CSY) < (((BSU * CSY) * CSY) * CTV) { 1.0 } else { 0.0 };
                    let CVM;
                    let CVZ;
                    let CWD;
                    if CTW != 0.0 {
                        let CTX = (CTF / CTV) + BSF;
                        let CTY = CTX - CSY;
                        let CTZ = CTY / AYH;
                        CVM = CTZ;
                        CVZ = CTY;
                        CWD = CTX;
                    } else {
                        let CUA = if CTG > BAS { 1.0 } else { 0.0 };
                        let CUO;
                        if CUA != 0.0 {
                            let CUJ = (((AJF * CTG) / (D - (CUB * (HC - CUB)))).ln()) - CUF;
                            CUO = CUJ;
                        } else {
                            let CUK = if CTG < -5e-3f64 { 1.0 } else { 0.0 };
                            let CUP = if CUK != 0.0 {
                                let CUL = (F * CUF).sin();
                                let CUM = ((-CTG) / (CUL * CUL)).ln();
                                CUM
                            } else {
                                let CUN = (AJF - ((CTG * KJ) * (D - ((AAI * CTG) * (D - (BCQ * CTG)))))).ln();
                                CUN
                            };
                            CUO = CUP;
                        }
                        let CUQ = ((BEA + CSW) + (HC * (CTV.ln()))) - CUO;
                        let CUR = AYH * CUQ;
                        let CUS = CSY + CUR;
                        CVM = CUQ;
                        CVZ = CUR;
                        CWD = CUS;
                    }
                    CVL = CVM;
                    CVY = CVZ;
                    CWC = CWD;
                } else {
                    let CUT = if CTG > BAS { 1.0 } else { 0.0 };
                    let CVF;
                    if CUT != 0.0 {
                        let CUU = ((CSW + CBE) - AYB) - CUF;
                        let CUV = if CUU < KE { 1.0 } else { 0.0 };
                        let CUZ = if CUV != 0.0 {
                            let CUW = CUU.exp();
                            CUW
                        } else {
                            let CUX = CUU - KE;
                            let CUY = TP * (D + (CUX * (D + ((F * CUX) * (D + (CUX * KJ))))));
                            CUY
                        };
                        let CVA = ((AJF * CTG) * (CUZ / AYN)) / (D - (CUB * (HC - CUB)));
                        CVF = CVA;
                    } else {
                        let CVB = if CTG < -5e-3f64 { 1.0 } else { 0.0 };
                        let CVG = if CVB != 0.0 {
                            let CVC = (F * CUF).sin();
                            let CVD = ((-CTG) / (CVC * CVC)) / CTF;
                            CVD
                        } else {
                            let CVE = (AJF - ((CTG * KJ) * (D - ((AAI * CTG) * (D - (BCQ * CTG)))))) / CTF;
                            CVE
                        };
                        CVF = CVG;
                    }
                    let CVH = ((CSY - CTS) / (D - CVF)) + BSF;
                    let CVI = CVH - CSY;
                    let CVJ = CVI / AYH;
                    CVL = CVJ;
                    CVY = CVI;
                    CWC = CVH;
                }
                CVK = CVL;
                CVX = CVY;
                CWB = CWC;
            }
            let CVN = (AYF - CVK) - CBE;
            let CVO = if CVN < KE { 1.0 } else { 0.0 };
            let CVS = if CVO != 0.0 {
                let CVP = CVN.exp();
                CVP
            } else {
                let CVQ = CVN - KE;
                let CVR = TP * (D + (CVQ * (D + ((F * CVQ) * (D + (CVQ * KJ))))));
                CVR
            };
            let CVT = AYN * CVS;
            let CZN;
            let CZP;
            let CZU;
            let CZY;
            if BUW != 0.0 {
                let CVU = CTF * AYI;
                let CVV = CVT * AYJ;
                let CVW = CVU + (HC * CSY);
                let CWA = CVV + (HC * CVX);
                let CWE = ((HC * CWB) + CVU) + CVV;
                let CWF = if (CTG.abs()) > BAS { 1.0 } else { 0.0 };
                let CZV = if CWF != 0.0 {
                    let CWG = ((-4e0f64 * CTG) * CWE) / (CWB * (((CVW * CWA) + ((HC * (CSW + HC)) * CWA)) + ((HC * (CVK + HC)) * CVW)));
                    CWG
                } else {
                    let CWH = CTG * BBS;
                    let CWI = ((CTF * CVT) * CWE) / (CWB * (((CVW * CTF) + (CWA * CVT)) + (((CVW * CWA) * CWB) * (D + (CWB * (ASH * (D - (CWH * (D - ((CTG * BBU) * (D - CWH)))))))))));
                    CWI
                };
                CZN = CWA;
                CZP = CVW;
                CZU = CZV;
                CZY = CWE;
            } else {
                CZN = A;
                CZP = A;
                CZU = A;
                CZY = A;
            }
            let CWJ = CBE + (CWB.ln());
            let CWK = F * (BUT + CWB);
            let CWL = CWJ - BVJ;
            let CXN = if AEG != 0.0 {
                let CWM = (F * (BRV + CSY)) / AYG;
                let CWN = CWM - ADN;
                let CWO = F * ((CWM + ADN) + (((CWN * CWN) + D).sqrt()));
                let CWP = (((CWO / AQH) + ((ALG * AEJ) * AEJ)).sqrt()) - (F * AEJ);
                let CWQ = D - (((CWP * CWP) * AQH) / CWO);
                CWQ
            } else {
                D
            };
            let CWR = CSY / HC;
            let CWS = if CWR < KE { 1.0 } else { 0.0 };
            let CWU = if CWS != 0.0 {
                let CWT = (D + (CWR.exp())).ln();
                CWT
            } else {
                CWR
            };
            let CWV = HC * CWU;
            let CWW = CVX / HC;
            let CWX = if CWW < KE { 1.0 } else { 0.0 };
            let CWZ = if CWX != 0.0 {
                let CWY = (D + (CWW.exp())).ln();
                CWY
            } else {
                CWW
            };
            let CXA = HC * CWZ;
            let CXB = CXA - CVX;
            let CXC = CWV - CSY;
            let CXD = F * (BVO + CWV);
            let CXE = F * (BVT + CXA);
            let CXF = CXD + CXE;
            let CXG = D / CXF;
            let CXH = (CWK * CXD) * CXG;
            let CXI = (CWK * CXE) * CXG;
            let CXJ = F * (BVU + CXB);
            let CXK = F * (BVV + CXC);
            let CXL = F * (BVW + ((AHH * CWV) + (AHI * CXB)));
            let CXM = F * (BVX + ((AHH * CXA) + (AHI * CXC)));
            let CXO = ((CXD * AGB) * BWA) * CXN;
            let CXP = (CXE * AGF) * BWA;
            let CXQ = CXO + CXP;
            let CXR = BWD * (CXJ + (BWE * CXK));
            let CXS = D + CXR;
            let CXT = D + (BWH * CXR);
            let CXU = (F * (CXS + (((CXS * CXS) + O).sqrt()))) / (F * (CXT + (((CXT * CXT) + O).sqrt())));
            let CXV = (BWK * ((D + (BWL * CXJ)) + (BWM * CXK))) * ((BWO * (((D + (CXH * AHA)) + (CXI * AHC)).ln())).exp());
            let CXZ;
            if BWR != 0.0 {
                CXZ = D;
            } else {
                let CXW = if BWQ < A { 1.0 } else { 0.0 };
                let CYA = if CXW != 0.0 {
                    let CXX = D - (BWQ * ((BWT * ((CWK + BWU).ln())).exp()));
                    CXX
                } else {
                    let CXY = D / (D + (BWQ * ((BWT * ((CWK + BWU).ln())).exp())));
                    CXY
                };
                CXZ = CYA;
            }
            let CYB = BXA * ((CWK * CXZ) + BXD);
            let CYC = (CXU * CXQ) / ((CXO / (((D + ((BXF * (((BXG * CXL) + GW).ln())).exp())) + CXV) + (BXH * CYB))) + (CXP / (((D + ((BXF * (((BXG * CXM) + GW).ln())).exp())) + CXV) + (BXI * CYB))));
            let CYD = D / (AJF + CWK);
            let CYF = if CYE > A { 1.0 } else { 0.0 };
            let CYI = if CYF != 0.0 {
                let CYG = D / (D + (CYE * CXI));
                CYG
            } else {
                let CYH = D - (CYE * CXI);
                CYH
            };
            let CYK = CYJ * AQH;
            let CYM = ((D + ((AQI - CBE) / (CYK + ((CYL * CWK) * CWK)))).ln()) * ((CWK * CYD) * CYI);
            let CYN = AQT * CYM;
            let CYO = D / (D + (CYN * (D + CYN)));
            let CYP = (BYL * CXD) / (BYL + CXD);
            let CYQ = if BYN < A { 1.0 } else { 0.0 };
            let CYX = if CYQ != 0.0 {
                let CYR = D / (D - (BYN * CYP));
                CYR
            } else {
                let CYS = D + (BYN * CYP);
                CYS
            };
            let CYT = (BYL * CXE) / (BYL + CXE);
            let CYU = if BYS < A { 1.0 } else { 0.0 };
            let CYY = if CYU != 0.0 {
                let CYV = D / (D - (BYS * CYT));
                CYV
            } else {
                let CYW = D + (BYS * CYT);
                CYW
            };
            let CYZ = ((AQS * CWL) * F) * (CYX + CYY);
            let CZA = CYC * CYO;
            let CZB = CYZ / CZA;
            let CZC = CZB * CZB;
            let CZD = (D + CZC).sqrt();
            let CZE = (D + (ARL * CZC)) / CZD;
            let DBI;
            let DBJ;
            if AEQ != 0.0 {
                let CZG = CZF * AVY;
                let CZJ = (D + (AYG * (CZG * ((-1.666666666667e-1f64 * (((CXD * CXD) + CZH).ln())).exp())))) / CZI;
                let CZL = (D + (AYH * (CZG * ((-1.666666666667e-1f64 * (((CXE * CXE) + CZH).ln())).exp())))) / CZK;
                DBI = CZJ;
                DBJ = CZL;
            } else {
                DBI = D;
                DBJ = D;
            }
            let DAP;
            let DBK;
            if BUW != 0.0 {
                let CZM = if CWB > GW { 1.0 } else { 0.0 };
                let DAB;
                if CZM != 0.0 {
                    let CZO = if (CZN.abs()) < O { 1.0 } else { 0.0 };
                    let DAC = if CZO != 0.0 {
                        let CZQ = HC + CVK;
                        let CZR = ((HC + CSW) + (F * CZP)) / (CZQ * CZP);
                        let CZS = CZR * CZN;
                        let CZT = CZS * CZS;
                        let CZW = ((((CZU * CWB) - CTF) / CZP) - ((CVX - (((HC * CTG) * (CZR - (D / CZP))) * (((D - CZS) + CZT) - (CZS * CZT)))) / CZQ)) / CWB;
                        let CZX = (CZW * CWB) / (CZW + D);
                        CZX
                    } else {
                        let CZZ = ((CZU * CZY) / (CZP * CZN)) - (((CTF / CZP) + (CVT / CZN)) / CWB);
                        let DAA = (CZZ * CWB) / (CZZ + D);
                        DAA
                    };
                    DAB = DAC;
                } else {
                    DAB = BYJ;
                }
                let DAE = DAB - DAD;
                let DAF = D + ((CAH * DAE) * DAE);
                let DAG = if (DAE.abs()) > R { 1.0 } else { 0.0 };
                let DAQ = if DAG != 0.0 {
                    let DAH = CWB - BUT;
                    let DAI = DAH - (DAB * CWL);
                    let DAJ = DAH - (DAD * CWL);
                    let DAK = ((DAI * DAI) + DAF).sqrt();
                    let DAL = ((DAJ * DAJ) + DAF).sqrt();
                    let DAM = (ALG / DAE) * (((DAL * DAI) - (DAK * DAJ)) + (DAF * (((DAJ + DAL) / (DAI + DAK)).ln())));
                    DAM
                } else {
                    let DAN = CWL * DAE;
                    let DAO = (((-4.1666666666675e-2f64 * CWL) * DAN) * DAN) / (DAF.sqrt());
                    DAO
                };
                DAP = DAQ;
                DBK = DAB;
            } else {
                DAP = A;
                DBK = BYJ;
            }
            let DAR = (((CWK * CWL) + DAP) + BUT) - CWB;
            let DBD;
            if BUW != 0.0 {
                let DAT = if DAR > DAS { 1.0 } else { 0.0 };
                let DBE = if DAT != 0.0 {
                    let DAU = ((BYZ / ((BSC / BUT) - BYW)) - (CZP / ((CTF / CWB) - CZU))) / DAR;
                    DAU
                } else {
                    A
                };
                DBD = DBE;
            } else {
                let DAX = (-2e0f64 * DAV) * ((AYI / DAW) + BYK);
                let DBA = (-2e0f64 * DAY) * ((AYJ / DAZ) + BYK);
                let DBB = DBA * AYJ;
                let DBC = (-DAW) * (((((DBB + ((DBA - DAX) * BYK)) - (((DAX * AYI) + DBB) / DAW)) / (ATY + (HC * ((DAV * AYI) + (DAY * AYJ))))) * DAW) + BYK);
                DBD = DBC;
            }
            let DBF = DBD * CZE;
            let DBG = F * (CSY - BRV);
            let DBH = DBG * DBF;
            let DBL = (CXQ * AFZ) / CXF;
            let DBN = (AQT + (DBM * CYD)) * CYM;
            let DBO = (D + (DBN * (D + DBN))) * CYO;
            let DBP = CZA * CZD;
            let DBT = if AEQ != 0.0 {
                let DBQ = CXF / ((CXD / DBI) + (CXE / DBJ));
                DBQ
            } else {
                D
            };
            let DBS = (DBR * DBR) * DBL;
            let DBU = ((((DBS * AYL) * DAR) * DBO) / DBP) / DBT;
            let DBW = (-APX) * DBV;
            let DBX = (-APY) * DBV;
            let DBZ = ((AER * DBY) * DBV) + AQU;
            let DCA = DBW + DBZ;
            let DCB = DBX + DBZ;
            let DCC = ((((3.20435313e-19f64 * ZG) * ZT) * DBV).sqrt()) / ABV;
            let DCD = DCC * DCC;
            let DCE = D + (DCC / ADK);
            let DCF = ADN * DCE;
            let DCG = D / DCE;
            let DCI = D / (ASN + (DCC * DCH));
            let DCJ = if parameters[3] > A { 1.0 } else { 0.0 };
            let DCM = if (if DCK > A { 1.0 } else { 0.0 }) != 0.0 || (if DCL > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DCN = if parameters[4] > A { 1.0 } else { 0.0 };
            let DCO = if DCN != 0.0 && (if AKA > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DCP = if (if DCJ != 0.0 && DCM != 0.0 { 1.0 } else { 0.0 }) != 0.0 || DCO != 0.0 { 1.0 } else { 0.0 };
            let DLV;
            if DCP != 0.0 {
                let DCQ = if (DBW.abs()) <= DCF { 1.0 } else { 0.0 };
                let DLW;
                if DCQ != 0.0 {
                    let DCR = (-DBW) * DCG;
                    DLW = DCR;
                } else {
                    let DCS = if DBW < (-DCF) { 1.0 } else { 0.0 };
                    let DER;
                    if DCS != 0.0 {
                        let DCT = -DBW;
                        let DCU = (ASN * DCT) * DCG;
                        let DCV = DCU - ASP;
                        let DCW = F * ((DCU + J) - (((DCV * DCV) + ASR).sqrt()));
                        let DCX = DCT - DCW;
                        let DCY = (DCX * DCX) + (DCD * (DCW + D));
                        let DCZ = (HC * DCX) - DCD;
                        let DDA = ((DCY / DCD).ln()) - DCW;
                        let DDB = DCY + DCZ;
                        let DDC = (DDB * DDB) + (DDA * (((F * DCZ) * DCZ) - DCY));
                        let DDD = DCW + (((DCY * DDB) * DDA) / (DDC + (((((DDB / DDC) * DDA) * DDA) * DCZ) * (((DCZ * DCZ) * KJ) - DCY))));
                        let DDE = if (DDD.abs()) < KE { 1.0 } else { 0.0 };
                        let DDM;
                        if DDE != 0.0 {
                            let DDF = DDD.exp();
                            DDM = DDF;
                        } else {
                            let DDG = if DDD < -8e1f64 { 1.0 } else { 0.0 };
                            let DDN = if DDG != 0.0 {
                                let DDH = (-DDD) - KE;
                                let DDI = KH / (D + (DDH * (D + ((F * DDH) * (D + (DDH * KJ))))));
                                DDI
                            } else {
                                let DDJ = DDD - KE;
                                let DDK = TP * (D + (DDJ * (D + ((F * DDJ) * (D + (DDJ * KJ))))));
                                DDK
                            };
                            DDM = DDN;
                        }
                        let DDL = DCT - DDD;
                        let DDO = (HC * DDL) + (DCD * (DDM - D));
                        let DDP = (DDL * DDL) + (DCD * ((DDD + D) - DDM));
                        let DDQ = -(DDD + ((HC * DDP) / (DDO + (((DDO * DDO) - (AJF * ((D - ((DCD * F) * DDM)) * DDP))).sqrt()))));
                        DER = DDQ;
                    } else {
                        let DDR = -((DBW * DCG) * (D + (((((DCE * ASN) * DCI) - D) * DCI) * DBW)));
                        let DDS = if (DDR.abs()) < KE { 1.0 } else { 0.0 };
                        let DDZ;
                        if DDS != 0.0 {
                            let DDT = DDR.exp();
                            DDZ = DDT;
                        } else {
                            let DDU = if DDR < -8e1f64 { 1.0 } else { 0.0 };
                            let DEA = if DDU != 0.0 {
                                let DDV = (-DDR) - KE;
                                let DDW = KH / (D + (DDV * (D + ((F * DDV) * (D + (DDV * KJ))))));
                                DDW
                            } else {
                                let DDX = DDR - KE;
                                let DDY = TP * (D + (DDX * (D + ((F * DDX) * (D + (DDX * KJ))))));
                                DDY
                            };
                            DDZ = DEA;
                        }
                        let DEB = DCD * F;
                        let DEC = (DBW + DEB) - (DCC * (((DBW + (DCD * ALG)) - (D - DDZ)).sqrt()));
                        let DED = -DEC;
                        let DEE = if (DED.abs()) < KE { 1.0 } else { 0.0 };
                        let DEM;
                        if DEE != 0.0 {
                            let DEF = DED.exp();
                            DEM = DEF;
                        } else {
                            let DEG = if DED < -8e1f64 { 1.0 } else { 0.0 };
                            let DEN = if DEG != 0.0 {
                                let DEH = (-DED) - KE;
                                let DEI = KH / (D + (DEH * (D + ((F * DEH) * (D + (DEH * KJ))))));
                                DEI
                            } else {
                                let DEJ = DED - KE;
                                let DEK = TP * (D + (DEJ * (D + ((F * DEJ) * (D + (DEJ * KJ))))));
                                DEK
                            };
                            DEM = DEN;
                        }
                        let DEL = DBW - DEC;
                        let DEO = (HC * DEL) + (DCD * (D - DEM));
                        let DEP = (DEL * DEL) - (DCD * ((DEC - D) + DEM));
                        let DEQ = DEC + ((HC * DEP) / (DEO + (((DEO * DEO) - (AJF * ((D - (DEB * DEM)) * DEP))).sqrt())));
                        DER = DEQ;
                    }
                    let DES = -DER;
                    DLW = DES;
                }
                DLV = DLW;
            } else {
                DLV = A;
            }
            let DET = if ZP > A { 1.0 } else { 0.0 };
            let DMB;
            if DET != 0.0 {
                let DEU = if (DCA.abs()) <= DCF { 1.0 } else { 0.0 };
                let DMC;
                if DEU != 0.0 {
                    let DEV = (-DCA) * DCG;
                    DMC = DEV;
                } else {
                    let DEW = if DCA < (-DCF) { 1.0 } else { 0.0 };
                    let DGV;
                    if DEW != 0.0 {
                        let DEX = -DCA;
                        let DEY = (ASN * DEX) * DCG;
                        let DEZ = DEY - ASP;
                        let DFA = F * ((DEY + J) - (((DEZ * DEZ) + ASR).sqrt()));
                        let DFB = DEX - DFA;
                        let DFC = (DFB * DFB) + (DCD * (DFA + D));
                        let DFD = (HC * DFB) - DCD;
                        let DFE = ((DFC / DCD).ln()) - DFA;
                        let DFF = DFC + DFD;
                        let DFG = (DFF * DFF) + (DFE * (((F * DFD) * DFD) - DFC));
                        let DFH = DFA + (((DFC * DFF) * DFE) / (DFG + (((((DFF / DFG) * DFE) * DFE) * DFD) * (((DFD * DFD) * KJ) - DFC))));
                        let DFI = if (DFH.abs()) < KE { 1.0 } else { 0.0 };
                        let DFQ;
                        if DFI != 0.0 {
                            let DFJ = DFH.exp();
                            DFQ = DFJ;
                        } else {
                            let DFK = if DFH < -8e1f64 { 1.0 } else { 0.0 };
                            let DFR = if DFK != 0.0 {
                                let DFL = (-DFH) - KE;
                                let DFM = KH / (D + (DFL * (D + ((F * DFL) * (D + (DFL * KJ))))));
                                DFM
                            } else {
                                let DFN = DFH - KE;
                                let DFO = TP * (D + (DFN * (D + ((F * DFN) * (D + (DFN * KJ))))));
                                DFO
                            };
                            DFQ = DFR;
                        }
                        let DFP = DEX - DFH;
                        let DFS = (HC * DFP) + (DCD * (DFQ - D));
                        let DFT = (DFP * DFP) + (DCD * ((DFH + D) - DFQ));
                        let DFU = -(DFH + ((HC * DFT) / (DFS + (((DFS * DFS) - (AJF * ((D - ((DCD * F) * DFQ)) * DFT))).sqrt()))));
                        DGV = DFU;
                    } else {
                        let DFV = -((DCA * DCG) * (D + (((((DCE * ASN) * DCI) - D) * DCI) * DCA)));
                        let DFW = if (DFV.abs()) < KE { 1.0 } else { 0.0 };
                        let DGD;
                        if DFW != 0.0 {
                            let DFX = DFV.exp();
                            DGD = DFX;
                        } else {
                            let DFY = if DFV < -8e1f64 { 1.0 } else { 0.0 };
                            let DGE = if DFY != 0.0 {
                                let DFZ = (-DFV) - KE;
                                let DGA = KH / (D + (DFZ * (D + ((F * DFZ) * (D + (DFZ * KJ))))));
                                DGA
                            } else {
                                let DGB = DFV - KE;
                                let DGC = TP * (D + (DGB * (D + ((F * DGB) * (D + (DGB * KJ))))));
                                DGC
                            };
                            DGD = DGE;
                        }
                        let DGF = DCD * F;
                        let DGG = (DCA + DGF) - (DCC * (((DCA + (DCD * ALG)) - (D - DGD)).sqrt()));
                        let DGH = -DGG;
                        let DGI = if (DGH.abs()) < KE { 1.0 } else { 0.0 };
                        let DGQ;
                        if DGI != 0.0 {
                            let DGJ = DGH.exp();
                            DGQ = DGJ;
                        } else {
                            let DGK = if DGH < -8e1f64 { 1.0 } else { 0.0 };
                            let DGR = if DGK != 0.0 {
                                let DGL = (-DGH) - KE;
                                let DGM = KH / (D + (DGL * (D + ((F * DGL) * (D + (DGL * KJ))))));
                                DGM
                            } else {
                                let DGN = DGH - KE;
                                let DGO = TP * (D + (DGN * (D + ((F * DGN) * (D + (DGN * KJ))))));
                                DGO
                            };
                            DGQ = DGR;
                        }
                        let DGP = DCA - DGG;
                        let DGS = (HC * DGP) + (DCD * (D - DGQ));
                        let DGT = (DGP * DGP) - (DCD * ((DGG - D) + DGQ));
                        let DGU = DGG + ((HC * DGT) / (DGS + (((DGS * DGS) - (AJF * ((D - (DGF * DGQ)) * DGT))).sqrt())));
                        DGV = DGU;
                    }
                    let DGW = -DGV;
                    DMC = DGW;
                }
                DMB = DMC;
            } else {
                DMB = A;
            }
            let DGZ = ((((3.20435313e-19f64 * DGX) * ZT) * DBV).sqrt()) / ABV;
            let DHA = DGZ * DGZ;
            let DHB = D + (DGZ / ADK);
            let DHC = ADN * DHB;
            let DHD = D / DHB;
            let DHE = D / (ASN + (DGZ * DCH));
            let DHH = if (if DHF > A { 1.0 } else { 0.0 }) != 0.0 || (if DHG > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DHI = if DCN != 0.0 && (if AKD > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DHJ = if (if DCJ != 0.0 && DHH != 0.0 { 1.0 } else { 0.0 }) != 0.0 || DHI != 0.0 { 1.0 } else { 0.0 };
            let DLY;
            if DHJ != 0.0 {
                let DHK = if (DBX.abs()) <= DHC { 1.0 } else { 0.0 };
                let DLZ;
                if DHK != 0.0 {
                    let DHL = (-DBX) * DHD;
                    DLZ = DHL;
                } else {
                    let DHM = if DBX < (-DHC) { 1.0 } else { 0.0 };
                    let DJL;
                    if DHM != 0.0 {
                        let DHN = -DBX;
                        let DHO = (ASN * DHN) * DHD;
                        let DHP = DHO - ASP;
                        let DHQ = F * ((DHO + J) - (((DHP * DHP) + ASR).sqrt()));
                        let DHR = DHN - DHQ;
                        let DHS = (DHR * DHR) + (DHA * (DHQ + D));
                        let DHT = (HC * DHR) - DHA;
                        let DHU = ((DHS / DHA).ln()) - DHQ;
                        let DHV = DHS + DHT;
                        let DHW = (DHV * DHV) + (DHU * (((F * DHT) * DHT) - DHS));
                        let DHX = DHQ + (((DHS * DHV) * DHU) / (DHW + (((((DHV / DHW) * DHU) * DHU) * DHT) * (((DHT * DHT) * KJ) - DHS))));
                        let DHY = if (DHX.abs()) < KE { 1.0 } else { 0.0 };
                        let DIG;
                        if DHY != 0.0 {
                            let DHZ = DHX.exp();
                            DIG = DHZ;
                        } else {
                            let DIA = if DHX < -8e1f64 { 1.0 } else { 0.0 };
                            let DIH = if DIA != 0.0 {
                                let DIB = (-DHX) - KE;
                                let DIC = KH / (D + (DIB * (D + ((F * DIB) * (D + (DIB * KJ))))));
                                DIC
                            } else {
                                let DID = DHX - KE;
                                let DIE = TP * (D + (DID * (D + ((F * DID) * (D + (DID * KJ))))));
                                DIE
                            };
                            DIG = DIH;
                        }
                        let DIF = DHN - DHX;
                        let DII = (HC * DIF) + (DHA * (DIG - D));
                        let DIJ = (DIF * DIF) + (DHA * ((DHX + D) - DIG));
                        let DIK = -(DHX + ((HC * DIJ) / (DII + (((DII * DII) - (AJF * ((D - ((DHA * F) * DIG)) * DIJ))).sqrt()))));
                        DJL = DIK;
                    } else {
                        let DIL = -((DBX * DHD) * (D + (((((DHB * ASN) * DHE) - D) * DHE) * DBX)));
                        let DIM = if (DIL.abs()) < KE { 1.0 } else { 0.0 };
                        let DIT;
                        if DIM != 0.0 {
                            let DIN = DIL.exp();
                            DIT = DIN;
                        } else {
                            let DIO = if DIL < -8e1f64 { 1.0 } else { 0.0 };
                            let DIU = if DIO != 0.0 {
                                let DIP = (-DIL) - KE;
                                let DIQ = KH / (D + (DIP * (D + ((F * DIP) * (D + (DIP * KJ))))));
                                DIQ
                            } else {
                                let DIR = DIL - KE;
                                let DIS = TP * (D + (DIR * (D + ((F * DIR) * (D + (DIR * KJ))))));
                                DIS
                            };
                            DIT = DIU;
                        }
                        let DIV = DHA * F;
                        let DIW = (DBX + DIV) - (DGZ * (((DBX + (DHA * ALG)) - (D - DIT)).sqrt()));
                        let DIX = -DIW;
                        let DIY = if (DIX.abs()) < KE { 1.0 } else { 0.0 };
                        let DJG;
                        if DIY != 0.0 {
                            let DIZ = DIX.exp();
                            DJG = DIZ;
                        } else {
                            let DJA = if DIX < -8e1f64 { 1.0 } else { 0.0 };
                            let DJH = if DJA != 0.0 {
                                let DJB = (-DIX) - KE;
                                let DJC = KH / (D + (DJB * (D + ((F * DJB) * (D + (DJB * KJ))))));
                                DJC
                            } else {
                                let DJD = DIX - KE;
                                let DJE = TP * (D + (DJD * (D + ((F * DJD) * (D + (DJD * KJ))))));
                                DJE
                            };
                            DJG = DJH;
                        }
                        let DJF = DBX - DIW;
                        let DJI = (HC * DJF) + (DHA * (D - DJG));
                        let DJJ = (DJF * DJF) - (DHA * ((DIW - D) + DJG));
                        let DJK = DIW + ((HC * DJJ) / (DJI + (((DJI * DJI) - (AJF * ((D - (DIV * DJG)) * DJJ))).sqrt())));
                        DJL = DJK;
                    }
                    let DJM = -DJL;
                    DLZ = DJM;
                }
                DLY = DLZ;
            } else {
                DLY = A;
            }
            let DJP = if DJN > A { 1.0 } else { 0.0 };
            let DME;
            if DJP != 0.0 {
                let DJQ = if (DCB.abs()) <= DHC { 1.0 } else { 0.0 };
                let DMF;
                if DJQ != 0.0 {
                    let DJR = (-DCB) * DHD;
                    DMF = DJR;
                } else {
                    let DJS = if DCB < (-DHC) { 1.0 } else { 0.0 };
                    let DLR;
                    if DJS != 0.0 {
                        let DJT = -DCB;
                        let DJU = (ASN * DJT) * DHD;
                        let DJV = DJU - ASP;
                        let DJW = F * ((DJU + J) - (((DJV * DJV) + ASR).sqrt()));
                        let DJX = DJT - DJW;
                        let DJY = (DJX * DJX) + (DHA * (DJW + D));
                        let DJZ = (HC * DJX) - DHA;
                        let DKA = ((DJY / DHA).ln()) - DJW;
                        let DKB = DJY + DJZ;
                        let DKC = (DKB * DKB) + (DKA * (((F * DJZ) * DJZ) - DJY));
                        let DKD = DJW + (((DJY * DKB) * DKA) / (DKC + (((((DKB / DKC) * DKA) * DKA) * DJZ) * (((DJZ * DJZ) * KJ) - DJY))));
                        let DKE = if (DKD.abs()) < KE { 1.0 } else { 0.0 };
                        let DKM;
                        if DKE != 0.0 {
                            let DKF = DKD.exp();
                            DKM = DKF;
                        } else {
                            let DKG = if DKD < -8e1f64 { 1.0 } else { 0.0 };
                            let DKN = if DKG != 0.0 {
                                let DKH = (-DKD) - KE;
                                let DKI = KH / (D + (DKH * (D + ((F * DKH) * (D + (DKH * KJ))))));
                                DKI
                            } else {
                                let DKJ = DKD - KE;
                                let DKK = TP * (D + (DKJ * (D + ((F * DKJ) * (D + (DKJ * KJ))))));
                                DKK
                            };
                            DKM = DKN;
                        }
                        let DKL = DJT - DKD;
                        let DKO = (HC * DKL) + (DHA * (DKM - D));
                        let DKP = (DKL * DKL) + (DHA * ((DKD + D) - DKM));
                        let DKQ = -(DKD + ((HC * DKP) / (DKO + (((DKO * DKO) - (AJF * ((D - ((DHA * F) * DKM)) * DKP))).sqrt()))));
                        DLR = DKQ;
                    } else {
                        let DKR = -((DCB * DHD) * (D + (((((DHB * ASN) * DHE) - D) * DHE) * DCB)));
                        let DKS = if (DKR.abs()) < KE { 1.0 } else { 0.0 };
                        let DKZ;
                        if DKS != 0.0 {
                            let DKT = DKR.exp();
                            DKZ = DKT;
                        } else {
                            let DKU = if DKR < -8e1f64 { 1.0 } else { 0.0 };
                            let DLA = if DKU != 0.0 {
                                let DKV = (-DKR) - KE;
                                let DKW = KH / (D + (DKV * (D + ((F * DKV) * (D + (DKV * KJ))))));
                                DKW
                            } else {
                                let DKX = DKR - KE;
                                let DKY = TP * (D + (DKX * (D + ((F * DKX) * (D + (DKX * KJ))))));
                                DKY
                            };
                            DKZ = DLA;
                        }
                        let DLB = DHA * F;
                        let DLC = (DCB + DLB) - (DGZ * (((DCB + (DHA * ALG)) - (D - DKZ)).sqrt()));
                        let DLD = -DLC;
                        let DLE = if (DLD.abs()) < KE { 1.0 } else { 0.0 };
                        let DLM;
                        if DLE != 0.0 {
                            let DLF = DLD.exp();
                            DLM = DLF;
                        } else {
                            let DLG = if DLD < -8e1f64 { 1.0 } else { 0.0 };
                            let DLN = if DLG != 0.0 {
                                let DLH = (-DLD) - KE;
                                let DLI = KH / (D + (DLH * (D + ((F * DLH) * (D + (DLH * KJ))))));
                                DLI
                            } else {
                                let DLJ = DLD - KE;
                                let DLK = TP * (D + (DLJ * (D + ((F * DLJ) * (D + (DLJ * KJ))))));
                                DLK
                            };
                            DLM = DLN;
                        }
                        let DLL = DCB - DLC;
                        let DLO = (HC * DLL) + (DHA * (D - DLM));
                        let DLP = (DLL * DLL) - (DHA * ((DLC - D) + DLM));
                        let DLQ = DLC + ((HC * DLP) / (DLO + (((DLO * DLO) - (AJF * ((D - (DLB * DLM)) * DLP))).sqrt())));
                        DLR = DLQ;
                    }
                    let DLS = -DLR;
                    DMF = DLS;
                }
                DME = DMF;
            } else {
                DME = A;
            }
            let DLU = -DLT;
            let DLX = DLU * (DBW + DLV);
            let DMA = DLU * (DBX + DLY);
            let DMD = DLU * (DCA + DMB);
            let DMG = DLU * (DCB + DME);
            let GMV;
            let GMY;
            if DCJ != 0.0 {
                let DXG;
                if DCM != 0.0 {
                    let DMI = DLX + DMH;
                    let DMJ = A - DMI;
                    let DMK = F * (DMI - (((DMJ * DMJ) + O).sqrt()));
                    let DMM = (((DLX * DLX) + DML).sqrt()) * AJE;
                    let DMN = F * DBW;
                    let DMO = if (DMN.abs()) < KE { 1.0 } else { 0.0 };
                    let DMV;
                    if DMO != 0.0 {
                        let DMP = DMN.exp();
                        DMV = DMP;
                    } else {
                        let DMQ = if DMN < -8e1f64 { 1.0 } else { 0.0 };
                        let DMW = if DMQ != 0.0 {
                            let DMR = (-DMN) - KE;
                            let DMS = KH / (D + (DMR * (D + ((F * DMR) * (D + (DMR * KJ))))));
                            DMS
                        } else {
                            let DMT = DMN - KE;
                            let DMU = TP * (D + (DMT * (D + ((F * DMT) * (D + (DMT * KJ))))));
                            DMU
                        };
                        DMV = DMW;
                    }
                    let DMX = D / (D + DMV);
                    let DMY = D - DMX;
                    let DMZ = (AJS * DMX) + (AJO * DMY);
                    let DNA = (AJQ * DMX) + (AJM * DMY);
                    let DND = (DNB * DMX) + (DNC * DMY);
                    let DNE = (DCL * DMX) + (DCK * DMY);
                    let DNG = (DNF * DMY) * GW;
                    let DNI = AJH * ((-1e0f64 * DNH) / DMM);
                    let DNJ = if DNA < A { 1.0 } else { 0.0 };
                    let DOC = if DNJ != 0.0 {
                        let DNK = DMM - DND;
                        let DNL = F * ((DMM + DND) - (((DNK * DNK) + GW).sqrt()));
                        DNL
                    } else {
                        DMM
                    };
                    let DNM = (ATY + DLV) + (DMK * DBV);
                    let DNN = if (DNM.abs()) < KE { 1.0 } else { 0.0 };
                    let DOQ;
                    if DNN != 0.0 {
                        let DNO = DNM.exp();
                        DOQ = DNO;
                    } else {
                        let DNP = if DNM < -8e1f64 { 1.0 } else { 0.0 };
                        let DOR = if DNP != 0.0 {
                            let DNQ = (-DNM) - KE;
                            let DNR = KH / (D + (DNQ * (D + ((F * DNQ) * (D + (DNQ * KJ))))));
                            DNR
                        } else {
                            let DNS = DNM - KE;
                            let DNT = TP * (D + (DNS * (D + ((F * DNS) * (D + (DNS * KJ))))));
                            DNT
                        };
                        DOQ = DOR;
                    }
                    let DNU = DNM + DBW;
                    let DNV = if (DNU.abs()) < KE { 1.0 } else { 0.0 };
                    let DOS;
                    if DNV != 0.0 {
                        let DNW = DNU.exp();
                        DOS = DNW;
                    } else {
                        let DNX = if DNU < -8e1f64 { 1.0 } else { 0.0 };
                        let DOT = if DNX != 0.0 {
                            let DNY = (-DNU) - KE;
                            let DNZ = KH / (D + (DNY * (D + ((F * DNY) * (D + (DNY * KJ))))));
                            DNZ
                        } else {
                            let DOA = DNU - KE;
                            let DOB = TP * (D + (DOA * (D + ((F * DOA) * (D + (DOA * KJ))))));
                            DOB
                        };
                        DOS = DOT;
                    }
                    let DOD = AJH * (-1.5e0f64 + (DOC * (DMZ + (DNA * DOC))));
                    let DOE = if DOD > A { 1.0 } else { 0.0 };
                    let DPO;
                    if DOE != 0.0 {
                        let DOF = D + (DOD * (D + ((F * DOD) * (D + (DOD * KJ)))));
                        DPO = DOF;
                    } else {
                        let DOG = if DOD > -8e1f64 { 1.0 } else { 0.0 };
                        let DPP = if DOG != 0.0 {
                            let DOH = DOD.exp();
                            DOH
                        } else {
                            let DOI = (-DOD) - KE;
                            let DOJ = KH / (D + (DOI * (D + ((F * DOI) * (D + (DOI * KJ))))));
                            DOJ
                        };
                        DPO = DPP;
                    }
                    let DOK = if DNI > A { 1.0 } else { 0.0 };
                    let DPX;
                    if DOK != 0.0 {
                        let DOL = D + (DNI * (D + ((F * DNI) * (D + (DNI * KJ)))));
                        DPX = DOL;
                    } else {
                        let DOM = if DNI > -8e1f64 { 1.0 } else { 0.0 };
                        let DPY = if DOM != 0.0 {
                            let DON = DNI.exp();
                            DON
                        } else {
                            let DOO = (-DNI) - KE;
                            let DOP = KH / (D + (DOO * (D + ((F * DOO) * (D + (DOO * KJ))))));
                            DOP
                        };
                        DPX = DPY;
                    }
                    let DOU = (D + DOQ) / (D + DOS);
                    let DOV = if DOU < BSF { 1.0 } else { 0.0 };
                    let DPQ = if DOV != 0.0 {
                        BSF
                    } else {
                        DOU
                    };
                    let DOY = DOW * (APY - DOX);
                    let DOZ = if (DOY.abs()) < KE { 1.0 } else { 0.0 };
                    let DPR;
                    if DOZ != 0.0 {
                        let DPA = DOY.exp();
                        DPR = DPA;
                    } else {
                        let DPB = if DOY < -8e1f64 { 1.0 } else { 0.0 };
                        let DPS = if DPB != 0.0 {
                            let DPC = (-DOY) - KE;
                            let DPD = KH / (D + (DPC * (D + ((F * DPC) * (D + (DPC * KJ))))));
                            DPD
                        } else {
                            let DPE = DOY - KE;
                            let DPF = TP * (D + (DPE * (D + ((F * DPE) * (D + (DPE * KJ))))));
                            DPF
                        };
                        DPR = DPS;
                    }
                    let DPG = (DOW * APW) + DOY;
                    let DPH = if (DPG.abs()) < KE { 1.0 } else { 0.0 };
                    let DPU;
                    if DPH != 0.0 {
                        let DPI = DPG.exp();
                        DPU = DPI;
                    } else {
                        let DPJ = if DPG < -8e1f64 { 1.0 } else { 0.0 };
                        let DPV = if DPJ != 0.0 {
                            let DPK = (-DPG) - KE;
                            let DPL = KH / (D + (DPK * (D + ((F * DPK) * (D + (DPK * KJ))))));
                            DPL
                        } else {
                            let DPM = DPG - KE;
                            let DPN = TP * (D + (DPM * (D + ((F * DPM) * (D + (DPM * KJ))))));
                            DPN
                        };
                        DPU = DPV;
                    }
                    let DPT = D + DPR;
                    let DPW = D + DPU;
                    let DPZ = ((((DNE * DPO) * (DPQ.ln())) * DPT) / DPW) - (((DNG * DPX) * DPT) / DPW);
                    DXG = DPZ;
                } else {
                    DXG = A;
                }
                let DXJ;
                if DHH != 0.0 {
                    let DQA = DMA + DMH;
                    let DQB = A - DQA;
                    let DQC = F * (DQA - (((DQB * DQB) + O).sqrt()));
                    let DQD = (((DMA * DMA) + DML).sqrt()) * AJE;
                    let DQE = F * DBX;
                    let DQF = if (DQE.abs()) < KE { 1.0 } else { 0.0 };
                    let DQM;
                    if DQF != 0.0 {
                        let DQG = DQE.exp();
                        DQM = DQG;
                    } else {
                        let DQH = if DQE < -8e1f64 { 1.0 } else { 0.0 };
                        let DQN = if DQH != 0.0 {
                            let DQI = (-DQE) - KE;
                            let DQJ = KH / (D + (DQI * (D + ((F * DQI) * (D + (DQI * KJ))))));
                            DQJ
                        } else {
                            let DQK = DQE - KE;
                            let DQL = TP * (D + (DQK * (D + ((F * DQK) * (D + (DQK * KJ))))));
                            DQL
                        };
                        DQM = DQN;
                    }
                    let DQO = D / (D + DQM);
                    let DQP = D - DQO;
                    let DQQ = (AJS * DQO) + (AJO * DQP);
                    let DQR = (AJQ * DQO) + (AJM * DQP);
                    let DQS = (DNB * DQO) + (DNC * DQP);
                    let DQT = (DHG * DQO) + (DHF * DQP);
                    let DQV = (DQU * DQP) * GW;
                    let DQW = AJH * ((-1e0f64 * DNH) / DQD);
                    let DQX = if DQR < A { 1.0 } else { 0.0 };
                    let DRQ = if DQX != 0.0 {
                        let DQY = DQD - DQS;
                        let DQZ = F * ((DQD + DQS) - (((DQY * DQY) + GW).sqrt()));
                        DQZ
                    } else {
                        DQD
                    };
                    let DRA = (ATY + DLY) + (DQC * DBV);
                    let DRB = if (DRA.abs()) < KE { 1.0 } else { 0.0 };
                    let DSE;
                    if DRB != 0.0 {
                        let DRC = DRA.exp();
                        DSE = DRC;
                    } else {
                        let DRD = if DRA < -8e1f64 { 1.0 } else { 0.0 };
                        let DSF = if DRD != 0.0 {
                            let DRE = (-DRA) - KE;
                            let DRF = KH / (D + (DRE * (D + ((F * DRE) * (D + (DRE * KJ))))));
                            DRF
                        } else {
                            let DRG = DRA - KE;
                            let DRH = TP * (D + (DRG * (D + ((F * DRG) * (D + (DRG * KJ))))));
                            DRH
                        };
                        DSE = DSF;
                    }
                    let DRI = DRA + DBX;
                    let DRJ = if (DRI.abs()) < KE { 1.0 } else { 0.0 };
                    let DSG;
                    if DRJ != 0.0 {
                        let DRK = DRI.exp();
                        DSG = DRK;
                    } else {
                        let DRL = if DRI < -8e1f64 { 1.0 } else { 0.0 };
                        let DSH = if DRL != 0.0 {
                            let DRM = (-DRI) - KE;
                            let DRN = KH / (D + (DRM * (D + ((F * DRM) * (D + (DRM * KJ))))));
                            DRN
                        } else {
                            let DRO = DRI - KE;
                            let DRP = TP * (D + (DRO * (D + ((F * DRO) * (D + (DRO * KJ))))));
                            DRP
                        };
                        DSG = DSH;
                    }
                    let DRR = AJH * (-1.5e0f64 + (DRQ * (DQQ + (DQR * DRQ))));
                    let DRS = if DRR > A { 1.0 } else { 0.0 };
                    let DTA;
                    if DRS != 0.0 {
                        let DRT = D + (DRR * (D + ((F * DRR) * (D + (DRR * KJ)))));
                        DTA = DRT;
                    } else {
                        let DRU = if DRR > -8e1f64 { 1.0 } else { 0.0 };
                        let DTB = if DRU != 0.0 {
                            let DRV = DRR.exp();
                            DRV
                        } else {
                            let DRW = (-DRR) - KE;
                            let DRX = KH / (D + (DRW * (D + ((F * DRW) * (D + (DRW * KJ))))));
                            DRX
                        };
                        DTA = DTB;
                    }
                    let DRY = if DQW > A { 1.0 } else { 0.0 };
                    let DTJ;
                    if DRY != 0.0 {
                        let DRZ = D + (DQW * (D + ((F * DQW) * (D + (DQW * KJ)))));
                        DTJ = DRZ;
                    } else {
                        let DSA = if DQW > -8e1f64 { 1.0 } else { 0.0 };
                        let DTK = if DSA != 0.0 {
                            let DSB = DQW.exp();
                            DSB
                        } else {
                            let DSC = (-DQW) - KE;
                            let DSD = KH / (D + (DSC * (D + ((F * DSC) * (D + (DSC * KJ))))));
                            DSD
                        };
                        DTJ = DTK;
                    }
                    let DSI = (D + DSE) / (D + DSG);
                    let DSJ = if DSI < BSF { 1.0 } else { 0.0 };
                    let DTC = if DSJ != 0.0 {
                        BSF
                    } else {
                        DSI
                    };
                    let DSK = DOW * (APX - DOX);
                    let DSL = if (DSK.abs()) < KE { 1.0 } else { 0.0 };
                    let DTD;
                    if DSL != 0.0 {
                        let DSM = DSK.exp();
                        DTD = DSM;
                    } else {
                        let DSN = if DSK < -8e1f64 { 1.0 } else { 0.0 };
                        let DTE = if DSN != 0.0 {
                            let DSO = (-DSK) - KE;
                            let DSP = KH / (D + (DSO * (D + ((F * DSO) * (D + (DSO * KJ))))));
                            DSP
                        } else {
                            let DSQ = DSK - KE;
                            let DSR = TP * (D + (DSQ * (D + ((F * DSQ) * (D + (DSQ * KJ))))));
                            DSR
                        };
                        DTD = DTE;
                    }
                    let DSS = (DOW * APV) + DSK;
                    let DST = if (DSS.abs()) < KE { 1.0 } else { 0.0 };
                    let DTG;
                    if DST != 0.0 {
                        let DSU = DSS.exp();
                        DTG = DSU;
                    } else {
                        let DSV = if DSS < -8e1f64 { 1.0 } else { 0.0 };
                        let DTH = if DSV != 0.0 {
                            let DSW = (-DSS) - KE;
                            let DSX = KH / (D + (DSW * (D + ((F * DSW) * (D + (DSW * KJ))))));
                            DSX
                        } else {
                            let DSY = DSS - KE;
                            let DSZ = TP * (D + (DSY * (D + ((F * DSY) * (D + (DSY * KJ))))));
                            DSZ
                        };
                        DTG = DTH;
                    }
                    let DTF = D + DTD;
                    let DTI = D + DTG;
                    let DTL = ((((DQT * DTA) * (DTC.ln())) * DTF) / DTI) - (((DQV * DTJ) * DTF) / DTI);
                    DXJ = DTL;
                } else {
                    DXJ = A;
                }
                let DTN = if DTM > A { 1.0 } else { 0.0 };
                let DXF;
                let DXI;
                if DTN != 0.0 {
                    let DTO = (-DBG) * AYI;
                    let DTP = (HC * DTO) - CBE;
                    let DTQ = if (DTP.abs()) < KE { 1.0 } else { 0.0 };
                    let DTX;
                    if DTQ != 0.0 {
                        let DTR = DTP.exp();
                        DTX = DTR;
                    } else {
                        let DTS = if DTP < -8e1f64 { 1.0 } else { 0.0 };
                        let DTY = if DTS != 0.0 {
                            let DTT = (-DTP) - KE;
                            let DTU = KH / (D + (DTT * (D + ((F * DTT) * (D + (DTT * KJ))))));
                            DTU
                        } else {
                            let DTV = DTP - KE;
                            let DTW = TP * (D + (DTV * (D + ((F * DTV) * (D + (DTV * KJ))))));
                            DTW
                        };
                        DTX = DTY;
                    }
                    let DTZ = DBR * ((DTO + ACD) - ((D + DTX).ln()));
                    let DUA = F * (BRT + CSW);
                    let DUB = DBR * DUA;
                    let DUD = DUB + DUC;
                    let DUE = A - DUD;
                    let DUF = F * (DUD - (((DUE * DUE) + O).sqrt()));
                    let DUG = (((DUB * DUB) + DML).sqrt()) * AJE;
                    let DVI = if AJJ != 0.0 {
                        let DUI = DUG - DUH;
                        let DUJ = F * ((DUG + DUH) - (((DUI * DUI) + GW).sqrt()));
                        DUJ
                    } else {
                        DUG
                    };
                    let DUK = AYB + AQU;
                    let DUN = ((DUK - DUA) + (((DUF - DUL) - DTZ) * AQH)) * DUM;
                    let DUO = if (DUN.abs()) < KE { 1.0 } else { 0.0 };
                    let DVD;
                    if DUO != 0.0 {
                        let DUP = DUN.exp();
                        DVD = DUP;
                    } else {
                        let DUQ = if DUN < -8e1f64 { 1.0 } else { 0.0 };
                        let DVE = if DUQ != 0.0 {
                            let DUR = (-DUN) - KE;
                            let DUS = KH / (D + (DUR * (D + ((F * DUR) * (D + (DUR * KJ))))));
                            DUS
                        } else {
                            let DUT = DUN - KE;
                            let DUU = TP * (D + (DUT * (D + ((F * DUT) * (D + (DUT * KJ))))));
                            DUU
                        };
                        DVD = DVE;
                    }
                    let DUV = ((-(AQD - DTZ)) * AQH) * DUM;
                    let DUW = if (DUV.abs()) < KE { 1.0 } else { 0.0 };
                    let DVF;
                    if DUW != 0.0 {
                        let DUX = DUV.exp();
                        DVF = DUX;
                    } else {
                        let DUY = if DUV < -8e1f64 { 1.0 } else { 0.0 };
                        let DVG = if DUY != 0.0 {
                            let DUZ = (-DUV) - KE;
                            let DVA = KH / (D + (DUZ * (D + ((F * DUZ) * (D + (DUZ * KJ))))));
                            DVA
                        } else {
                            let DVB = DUV - KE;
                            let DVC = TP * (D + (DVB * (D + ((F * DVB) * (D + (DVB * KJ))))));
                            DVC
                        };
                        DVF = DVG;
                    }
                    let DVH = DVD * DVF;
                    let DVJ = AJH * (-1.5e0f64 + (DVI * (AJK + (AJI * DVI))));
                    let DVK = if DVJ > A { 1.0 } else { 0.0 };
                    let DVT;
                    if DVK != 0.0 {
                        let DVL = D + (DVJ * (D + ((F * DVJ) * (D + (DVJ * KJ)))));
                        DVT = DVL;
                    } else {
                        let DVM = if (DVJ.abs()) < KE { 1.0 } else { 0.0 };
                        let DVU;
                        if DVM != 0.0 {
                            let DVN = DVJ.exp();
                            DVU = DVN;
                        } else {
                            let DVO = if DVJ < -8e1f64 { 1.0 } else { 0.0 };
                            let DVV = if DVO != 0.0 {
                                let DVP = (-DVJ) - KE;
                                let DVQ = KH / (D + (DVP * (D + ((F * DVP) * (D + (DVP * KJ))))));
                                DVQ
                            } else {
                                let DVR = DVJ - KE;
                                let DVS = TP * (D + (DVR * (D + ((F * DVR) * (D + (DVR * KJ))))));
                                DVS
                            };
                            DVU = DVV;
                        }
                        DVT = DVU;
                    }
                    let DVW = (DTM * DVT) * (((D + DVD) / (D + DVH)).ln());
                    let DVX = if (if DUK <= A { 1.0 } else { 0.0 }) != 0.0 || (if (if AJK == A { 1.0 } else { 0.0 }) != 0.0 && (if AJI == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DWX;
                    let DWZ;
                    if DVX != 0.0 {
                        DWX = D;
                        DWZ = F;
                    } else {
                        let DVY = (AJD / ((AJK + ((HC * AJI) * DVI)) * AJH)) * AQH;
                        let DVZ = DTO / DVY;
                        let DWA = (DVY * DBF) * AYG;
                        let DWB = D - DWA;
                        let DWC = (DWA * DWB) * F;
                        let DWD = F - (ATY * DWC);
                        let DWE = if DVZ < R { 1.0 } else { 0.0 };
                        let DWY;
                        let DXA;
                        if DWE != 0.0 {
                            let DWF = DVZ * DVZ;
                            let DWG = D + (DWF * ((ASH + (DWA * KJ)) + ((DWF * ASH) * (AAI + (BWH * DWA)))));
                            let DWH = (F * DWG) - ((DVZ * ASH) * (D + (DWF * ((AAD * (DWC + ALG)) + ((2.85714285714e-2f64 * DWF) * (1.25e-1f64 + DWC))))));
                            DWY = DWG;
                            DXA = DWH;
                        } else {
                            let DWI = D / DVZ;
                            let DWJ = if (DVZ.abs()) < KE { 1.0 } else { 0.0 };
                            let DWQ;
                            if DWJ != 0.0 {
                                let DWK = DVZ.exp();
                                DWQ = DWK;
                            } else {
                                let DWL = if DVZ < -8e1f64 { 1.0 } else { 0.0 };
                                let DWR = if DWL != 0.0 {
                                    let DWM = (-DVZ) - KE;
                                    let DWN = KH / (D + (DWM * (D + ((F * DWM) * (D + (DWM * KJ))))));
                                    DWN
                                } else {
                                    let DWO = DVZ - KE;
                                    let DWP = TP * (D + (DWO * (D + ((F * DWO) * (D + (DWO * KJ))))));
                                    DWP
                                };
                                DWQ = DWR;
                            }
                            let DWS = D / DWQ;
                            let DWT = DWQ - DWS;
                            let DWU = DWQ + DWS;
                            let DWV = F * (((DWB * DWT) * DWI) + (DWA * DWU));
                            let DWW = F * ((DWV - (DWT * (DWC - ((DWD * DWI) * DWI)))) - ((DWD * DWU) * DWI));
                            DWY = DWV;
                            DXA = DWW;
                        }
                        DWX = DWY;
                        DWZ = DXA;
                    }
                    let DXB = DVW * DWZ;
                    let DXC = (DVW * DWX) - DXB;
                    DXF = DXB;
                    DXI = DXC;
                } else {
                    DXF = A;
                    DXI = A;
                }
                let DXE = if DXD < A { 1.0 } else { 0.0 };
                let GMW;
                let GMZ;
                if DXE != 0.0 {
                    let DXH = DXF + DXG;
                    let DXK = DXI + DXJ;
                    GMW = DXH;
                    GMZ = DXK;
                } else {
                    let DXL = DXI + DXG;
                    let DXM = DXF + DXJ;
                    GMW = DXL;
                    GMZ = DXM;
                }
                GMV = GMW;
                GMY = GMZ;
            } else {
                GMV = A;
                GMY = A;
            }
            let DXN = if DCO != 0.0 && (if DLX < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GND;
            if DXN != 0.0 {
                let DXO = (((DLX * DLX) + (((ZN * ZN) * APZ) * APZ)) + GW).sqrt();
                let DXQ = (-DXP) / DXO;
                let DXR = if (DXQ.abs()) < KE { 1.0 } else { 0.0 };
                let DYG;
                if DXR != 0.0 {
                    let DXS = DXQ.exp();
                    DYG = DXS;
                } else {
                    let DXT = if DXQ < -8e1f64 { 1.0 } else { 0.0 };
                    let DYH = if DXT != 0.0 {
                        let DXU = (-DXQ) - KE;
                        let DXV = KH / (D + (DXU * (D + ((F * DXU) * (D + (DXU * KJ))))));
                        DXV
                    } else {
                        let DXW = DXQ - KE;
                        let DXX = TP * (D + (DXW * (D + ((F * DXW) * (D + (DXW * KJ))))));
                        DXX
                    };
                    DYG = DYH;
                }
                let DXY = ZO * APW;
                let DXZ = if (DXY.abs()) < KE { 1.0 } else { 0.0 };
                let DYI;
                if DXZ != 0.0 {
                    let DYA = DXY.exp();
                    DYI = DYA;
                } else {
                    let DYB = if DXY < -8e1f64 { 1.0 } else { 0.0 };
                    let DYJ = if DYB != 0.0 {
                        let DYC = (-DXY) - KE;
                        let DYD = KH / (D + (DYC * (D + ((F * DYC) * (D + (DYC * KJ))))));
                        DYD
                    } else {
                        let DYE = DXY - KE;
                        let DYF = TP * (D + (DYE * (D + ((F * DYE) * (D + (DYE * KJ))))));
                        DYF
                    };
                    DYI = DYJ;
                }
                let DYK = ((((((-AKA) * APW) * DLX) * DXO) * DYG) * F) * (D + DYI);
                GND = DYK;
            } else {
                GND = A;
            }
            let DYL = if DHI != 0.0 && (if DMA < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GNB;
            if DYL != 0.0 {
                let DYO = (((DMA * DMA) + (((DYM * DYM) * AQA) * AQA)) + GW).sqrt();
                let DYQ = (-DYP) / DYO;
                let DYR = if (DYQ.abs()) < KE { 1.0 } else { 0.0 };
                let DZI;
                if DYR != 0.0 {
                    let DYS = DYQ.exp();
                    DZI = DYS;
                } else {
                    let DYT = if DYQ < -8e1f64 { 1.0 } else { 0.0 };
                    let DZJ = if DYT != 0.0 {
                        let DYU = (-DYQ) - KE;
                        let DYV = KH / (D + (DYU * (D + ((F * DYU) * (D + (DYU * KJ))))));
                        DYV
                    } else {
                        let DYW = DYQ - KE;
                        let DYX = TP * (D + (DYW * (D + ((F * DYW) * (D + (DYW * KJ))))));
                        DYX
                    };
                    DZI = DZJ;
                }
                let DZA = DYY * APV;
                let DZB = if (DZA.abs()) < KE { 1.0 } else { 0.0 };
                let DZK;
                if DZB != 0.0 {
                    let DZC = DZA.exp();
                    DZK = DZC;
                } else {
                    let DZD = if DZA < -8e1f64 { 1.0 } else { 0.0 };
                    let DZL = if DZD != 0.0 {
                        let DZE = (-DZA) - KE;
                        let DZF = KH / (D + (DZE * (D + ((F * DZE) * (D + (DZE * KJ))))));
                        DZF
                    } else {
                        let DZG = DZA - KE;
                        let DZH = TP * (D + (DZG * (D + ((F * DZG) * (D + (DZG * KJ))))));
                        DZH
                    };
                    DZK = DZL;
                }
                let DZM = ((((((-AKD) * APV) * DMA) * DYO) * DZI) * F) * (D + DZK);
                GNB = DZM;
            } else {
                GNB = A;
            }
            let DZN = if parameters[12] > A { 1.0 } else { 0.0 };
            let ECN;
            if DZN != 0.0 {
                let DZO = AQJ * AKT;
                let DZP = F * ((AQG * AKT) - DZO);
                let DZR = D / (D + DZQ);
                let DZT = D / (D + DZS);
                let DZV = DZU * AKT;
                let DZW = (HC * DZV) * (((D + (DZO / DZV)).sqrt()) - D);
                let DZZ = ((((((AQD - AKY) * AKT) - DZP) - AQU) + (DZX * DZW)) * DZR) + DZP;
                let EAA = ((((((AQW - ALA) * AKT) - DZP) - AQU) + (DZY * DZW)) * DZT) + DZP;
                let EAC = EAA + (EAB * (DZZ - EAA));
                let EAD = EAC - AXX;
                let EAE = F * ((EAC + AXX) - (((EAD * EAD) + O).sqrt()));
                let EAG = DZZ + (EAF * (EAA - DZZ));
                let EAH = EAG - AXX;
                let EAI = F * ((EAG + AXX) - (((EAH * EAH) + O).sqrt()));
                let EAJ = D / (ABW / DZR);
                let EAK = D / (ABY / DZT);
                let EAL = D / ((D + EAJ) + EAK);
                let EAM = AKU / AYM;
                let EAN = EAL * (EAE - EAI);
                let EAO = if ((EAI - EAE).abs()) <= BWU { 1.0 } else { 0.0 };
                let EAR = if EAO != 0.0 {
                    let EAP = ((F * (((D - (EAL * EAJ)) - (EAL * EAK)) - ((((EAK + (((F * EAJ) * EAL) * EAJ)) - (((F * EAK) * EAL) * EAK)) - (F / EAL)) * EAN))) * EAM) / EAL;
                    EAP
                } else {
                    let EAQ = (EAM * ((((-EAJ) * EAN).exp()) - (((EAK - (D / EAL)) * EAN).exp()))) / (HC * EAN);
                    EAQ
                };
                let EAS = if EAE < KE { 1.0 } else { 0.0 };
                let EBE;
                if EAS != 0.0 {
                    let EAT = (D + (EAR * (EAE.exp()))).ln();
                    let EAU = EAT * (D - (((D + EAT).ln()) / (HC + EAT)));
                    EBE = EAU;
                } else {
                    let EAV = if EAE < A { 1.0 } else { 0.0 };
                    let EBF;
                    if EAV != 0.0 {
                        let EAW = if EAE > -8e1f64 { 1.0 } else { 0.0 };
                        let EBA = if EAW != 0.0 {
                            let EAX = EAE.exp();
                            EAX
                        } else {
                            let EAY = (-EAE) - KE;
                            let EAZ = KH / (D + (EAY * (D + ((F * EAY) * (D + (EAY * KJ))))));
                            EAZ
                        };
                        let EBB = EAR * EBA;
                        EBF = EBB;
                    } else {
                        let EBC = (EAR.ln()) + EAE;
                        let EBD = EBC * (D - (((D + EBC).ln()) / (HC + EBC)));
                        EBF = EBD;
                    }
                    EBE = EBF;
                }
                let EBG = EAE - CBE;
                let EBH = if EBG < KE { 1.0 } else { 0.0 };
                let EBT;
                if EBH != 0.0 {
                    let EBI = (D + (EAR * (EBG.exp()))).ln();
                    let EBJ = EBI * (D - (((D + EBI).ln()) / (HC + EBI)));
                    EBT = EBJ;
                } else {
                    let EBK = if EBG < A { 1.0 } else { 0.0 };
                    let EBU;
                    if EBK != 0.0 {
                        let EBL = if EBG > -8e1f64 { 1.0 } else { 0.0 };
                        let EBP = if EBL != 0.0 {
                            let EBM = EBG.exp();
                            EBM
                        } else {
                            let EBN = (-EBG) - KE;
                            let EBO = KH / (D + (EBN * (D + ((F * EBN) * (D + (EBN * KJ))))));
                            EBO
                        };
                        let EBQ = EAR * EBP;
                        EBU = EBQ;
                    } else {
                        let EBR = (EAR.ln()) + EBG;
                        let EBS = EBR * (D - (((D + EBR).ln()) / (HC + EBR)));
                        EBU = EBS;
                    }
                    EBT = EBU;
                }
                let EBV = ((((AKS * AKS) * ALD) * ABV) * (((F * (EBE + EBT)) + D) * (EBE - EBT))) / CYC;
                ECN = EBV;
            } else {
                ECN = A;
            }
            let EBW = if parameters[8] != A { 1.0 } else { 0.0 };
            let GMS;
            let GPR;
            if EBW != 0.0 {
                let EBY = (AQI - (EBX * CBE)) / AQH;
                let EBZ = if EBY > A { 1.0 } else { 0.0 };
                let GMT;
                let GPS;
                if EBZ != 0.0 {
                    let ECB = (-1e0f64 * ECA) / (EBY + DAS);
                    let ECC = if (ECB.abs()) < KE { 1.0 } else { 0.0 };
                    let ECK;
                    if ECC != 0.0 {
                        let ECD = ECB.exp();
                        ECK = ECD;
                    } else {
                        let ECE = if ECB < -8e1f64 { 1.0 } else { 0.0 };
                        let ECL = if ECE != 0.0 {
                            let ECF = (-ECB) - KE;
                            let ECG = KH / (D + (ECF * (D + ((F * ECF) * (D + (ECF * KJ))))));
                            ECG
                        } else {
                            let ECH = ECB - KE;
                            let ECI = TP * (D + (ECH * (D + ((F * ECH) * (D + (ECH * KJ))))));
                            ECI
                        };
                        ECK = ECL;
                    }
                    let ECM = (ECJ * EBY) * ECK;
                    let ECO = ECM * (DBU + ECN);
                    GMT = ECO;
                    GPS = ECM;
                } else {
                    GMT = A;
                    GPS = A;
                }
                GMS = GMT;
                GPR = GPS;
            } else {
                GMS = A;
                GPR = A;
            }
            if AMJ != 0.0 {
                let ECQ = if ((((DBU + ECN) * AQG).abs()) * ECP) > (1e8f64 * parameters[16]) { 1.0 } else { 0.0 };
                if ECQ != 0.0 {
                } else {
                }
            } else {
            }
            let ECT = ECR * ECS;
            let ECV = ECR * ECU;
            let ECX = ECR * ECW;
            let ECZ = ECR * ECY;
            let EDA = if FE > A { 1.0 } else { 0.0 };
            let GKF;
            let GKG;
            let GKH;
            let GKI;
            let GKJ;
            let GKL;
            let GKM;
            let GKO;
            let GKP;
            let GKR;
            let GKS;
            let GKX;
            let GKZ;
            let GLH;
            let GLI;
            let GLK;
            let GLS;
            let GLU;
            let GMG;
            let GMI;
            let GTR;
            let GTS;
            let GTT;
            let GTX;
            let GTY;
            let GTZ;
            if EDA != 0.0 {
                let EDQ = (((AQD - EDB) * AQH) - AQL) - AQU;
                let EDR = ((AQW - EDC) * AQH) - AQL;
                let EDS = EDR - AQU;
                let EHX;
                if ADR != 0.0 {
                    let EDT = AER * ADV;
                    let EDU = D + ABW;
                    let EDV = D + ABY;
                    let EDW = EDU / EDV;
                    let EDX = EDW.ln();
                    let EDY = if EDX > AGX { 1.0 } else { 0.0 };
                    let EED = if EDY != 0.0 {
                        let EDZ = ((HC * EDX) * (EDW + D)) / (EDW - D);
                        EDZ
                    } else {
                        let EEA = HC * (HC + EDX);
                        EEA
                    };
                    let EEB = ARH / ACC;
                    let EEC = D / EDV;
                    let EEE = ((((ABW + (ABY * EEC)) * EED) / EEB).ln()) + ARL;
                    let EEF = ((((ABY + (ABW * (D / EDU))) * EED) / EEB).ln()) + ARL;
                    let EEG = (EEE - (EDQ - ((ACA * (EDQ - EDS)) * ABZ))) / ARL;
                    let EEH = if EEG < KE { 1.0 } else { 0.0 };
                    let EEJ = if EEH != 0.0 {
                        let EEI = (D + (EEG.exp())).ln();
                        EEI
                    } else {
                        EEG
                    };
                    let EEK = (EEF - (((ABY * EDS) + (EEE - (ARL * EEJ))) * EEC)) / ARL;
                    let EEL = if EEK < KE { 1.0 } else { 0.0 };
                    let EEN = if EEL != 0.0 {
                        let EEM = (D + (EEK.exp())).ln();
                        EEM
                    } else {
                        EEK
                    };
                    let EEO = EDT * EDS;
                    let EEP = (EDT * (EEF - (ARL * EEN))) - EEO;
                    let EEQ = -ADQ;
                    let EER = if (EEQ.abs()) < KE { 1.0 } else { 0.0 };
                    let EEZ;
                    if EER != 0.0 {
                        let EES = EEQ.exp();
                        EEZ = EES;
                    } else {
                        let EET = if EEQ < -8e1f64 { 1.0 } else { 0.0 };
                        let EFA = if EET != 0.0 {
                            let EEU = (-EEQ) - KE;
                            let EEV = KH / (D + (EEU * (D + ((F * EEU) * (D + (EEU * KJ))))));
                            EEV
                        } else {
                            let EEW = EEQ - KE;
                            let EEX = TP * (D + (EEW * (D + ((F * EEW) * (D + (EEW * KJ))))));
                            EEX
                        };
                        EEZ = EFA;
                    }
                    let EEY = if (EEP.abs()) <= ADO { 1.0 } else { 0.0 };
                    let EHU;
                    if EEY != 0.0 {
                        let EFB = (EEP * ADM) * (D + (((EEP * (D - EEZ)) * ADH) * (((ADM * ADM) * ASH) / ADK)));
                        EHU = EFB;
                    } else {
                        let EFC = if EEP < (-ADO) { 1.0 } else { 0.0 };
                        let EHV;
                        if EFC != 0.0 {
                            let EFD = -EEP;
                            let EFE = ASN * (EFD * ADM);
                            let EFF = EFE - ASP;
                            let EFG = F * ((EFE + J) - (((EFF * EFF) + ASR).sqrt()));
                            let EFH = EFD - EFG;
                            let EFI = (EFH * EFH) + (ADI * (EFG + D));
                            let EFJ = (HC * EFH) - ADI;
                            let EFK = (-EFG) + ((EFI * ADJ).ln());
                            let EFL = EFI + EFJ;
                            let EFM = (EFL * EFL) + (EFK * (((F * EFJ) * EFJ) - EFI));
                            let EFN = EFG + (((EFI * EFL) * EFK) / (EFM + (((((EFL / EFM) * EFK) * EFK) * EFJ) * (((EFJ * EFJ) * KJ) - EFI))));
                            let EFO = if EFN < KE { 1.0 } else { 0.0 };
                            let EFS = if EFO != 0.0 {
                                let EFP = EFN.exp();
                                EFP
                            } else {
                                let EFQ = EFN - KE;
                                let EFR = TP * (D + (EFQ * (D + ((F * EFQ) * (D + (EFQ * KJ))))));
                                EFR
                            };
                            let EFT = EFN * EFN;
                            let EFU = D / (HC + EFT);
                            let EFV = EFT * EFU;
                            let EFW = EFD - EFN;
                            let EFX = EEZ * (D / EFS);
                            let EFY = (HC * EFW) + (ADI * (((EFS - D) - EFX) + (EEZ * (D - (AJF * ((EFN * EFU) * EFU))))));
                            let EFZ = (EFW * EFW) - (ADI * ((((EFS - EFN) - D) + EFX) + (EEZ * ((EFN - D) - EFV))));
                            let EGA = (-EFN) - (HC * (EFZ / (EFY + (((EFY * EFY) - (HC * (EFZ * (HC - (ADI * ((EFS + EFX) - (EEZ * ((((ATI * EFU) - (ATJ * EFV)) * EFU) * EFU)))))))).sqrt()))));
                            EHV = EGA;
                        } else {
                            let EGB = D / (ASN + (ADH * ATP));
                            let EGC = -((EEP * ADM) * (D + (((((ASN * ADL) * EGB) - D) * EGB) * EEP)));
                            let EGD = if EGC > -8e1f64 { 1.0 } else { 0.0 };
                            let EGH = if EGD != 0.0 {
                                let EGE = EGC.exp();
                                EGE
                            } else {
                                let EGF = (-EGC) - KE;
                                let EGG = KH / (D + (EGF * (D + ((F * EGF) * (D + (EGF * KJ))))));
                                EGG
                            };
                            let EGI = (EEP + (ADI * F)) - (ADH * (((EEP + (ADI * ALG)) - (D - EGH)).sqrt()));
                            let EGJ = ADQ + ATY;
                            let EGK = EGI - EGJ;
                            let EGL = (F * ((EGI + EGJ) - (((EGK * EGK) + JH).sqrt()))) - (F * (EGJ - (((EGJ * EGJ) + JH).sqrt())));
                            let EGM = EEP - EGL;
                            let EGN = (-EGL).exp();
                            let EGO = EGL * EGL;
                            let EGP = D / (HC + EGO);
                            let EGQ = EGO * EGP;
                            let EGR = if AUH >= ((EGM * EGM) - (ADI * (((EGN + EGL) - D) - (EEZ * ((EGL + D) + EGQ))))) { AUH } else { ((EGM * EGM) - (ADI * (((EGN + EGL) - D) - (EEZ * ((EGL + D) + EGQ))))) };
                            let EGS = (HC * EGM) + (ADI * ((D - EGN) - (EEZ * (D + (AJF * ((EGL * EGP) * EGP))))));
                            let EGT = (ADQ - EGL) + ((EGR / ADI).ln());
                            let EGU = EGR + EGS;
                            let EGV = EGR * (D - (F * (ADI * (EGN - (EEZ * ((((ATI * EGP) - (ATJ * EGQ)) * EGP) * EGP))))));
                            let EGW = (EGU * EGU) + (EGT * (((F * EGS) * EGS) - EGV));
                            let EGX = EGL + (((EGR * EGU) * EGT) / (EGW + (((((EGU / EGW) * EGT) * EGT) * EGS) * (((EGS * EGS) * KJ) - EGV))));
                            let EGY = if EGX < KE { 1.0 } else { 0.0 };
                            let EHN;
                            let EHP;
                            if EGY != 0.0 {
                                let EGZ = EGX.exp();
                                let EHA = D / EGZ;
                                let EHB = EEZ * EGZ;
                                EHN = EHA;
                                EHP = EHB;
                            } else {
                                let EHC = if EGX > (ADQ - KE) { 1.0 } else { 0.0 };
                                let EHO;
                                let EHQ;
                                if EHC != 0.0 {
                                    let EHD = (EGX - ADQ).exp();
                                    let EHE = EEZ / EHD;
                                    EHO = EHE;
                                    EHQ = EHD;
                                } else {
                                    let EHF = (ADQ - EGX) - KE;
                                    let EHG = KH / (D + (EHF * (D + ((F * EHF) * (D + (EHF * KJ))))));
                                    let EHH = EGX - KE;
                                    let EHI = KH / (D + (EHH * (D + ((F * EHH) * (D + (EHH * KJ))))));
                                    EHO = EHI;
                                    EHQ = EHG;
                                }
                                EHN = EHO;
                                EHP = EHQ;
                            }
                            let EHJ = EGX * EGX;
                            let EHK = D / (HC + EHJ);
                            let EHL = EHJ * EHK;
                            let EHM = EEP - EGX;
                            let EHR = (HC * EHM) + (ADI * (((D - EHN) + EHP) - (EEZ * (D + (AJF * ((EGX * EHK) * EHK))))));
                            let EHS = (EHM * EHM) - (ADI * ((((EHN + EGX) - D) + EHP) - (EEZ * ((EGX + D) + EHL))));
                            let EHT = EGX + (HC * (EHS / (EHR + (((EHR * EHR) - (HC * (EHS * (HC - (ADI * ((EHN + EHP) - (EEZ * ((((ATI * EHK) - (ATJ * EHL)) * EHK) * EHK)))))))).sqrt()))));
                            EHV = EHT;
                        }
                        EHU = EHV;
                    }
                    let EHW = EDT * (EHU + EEO);
                    EHX = EHW;
                } else {
                    EHX = EDS;
                }
                let EHY = EDQ - EHX;
                let EHZ = ACA * EHY;
                let EIM;
                let EIS;
                let EIX;
                let EJZ;
                let GIH;
                if AEQ != 0.0 {
                    let EIA = EHZ - AVR;
                    let EIB = AVR * AVR;
                    let EIC = -EHZ;
                    let EID = EIC - AVR;
                    let EIE = AVY * ((-3.333333333333e-1f64 * ((F * ((EHZ + AVR) + (((EIA * EIA) + EIB).sqrt()))).ln())).exp());
                    let EIF = AVY * ((-3.333333333333e-1f64 * ((F * ((EIC + AVR) + (((EID * EID) + EIB).sqrt()))).ln())).exp());
                    let EIG = (D - EIE) - EIF;
                    let EIH = ABQ / EIG;
                    let EII = (ABW * EIG) / (D + (ABW * EIE));
                    let EIJ = (ABY * EIG) / (D + (ABY * EIF));
                    let EIK = D / ((D + (D / EII)) + (D / EIJ));
                    let EIL = D + (EII * EIE);
                    EIM = EIK;
                    EIS = EII;
                    EIX = EIJ;
                    EJZ = EIH;
                    GIH = EIL;
                } else {
                    EIM = ACA;
                    EIS = ABW;
                    EIX = ABY;
                    EJZ = ABQ;
                    GIH = D;
                }
                let EIN = EIM * EHY;
                let EIO = if EIN > A { 1.0 } else { 0.0 };
                let EJA;
                if EIO != 0.0 {
                    let EIP = -EIN;
                    let EIQ = if EIP < KE { 1.0 } else { 0.0 };
                    let EIT = if EIQ != 0.0 {
                        let EIR = (D + (EIP.exp())).ln();
                        EIR
                    } else {
                        EIP
                    };
                    let EIU = ((EDQ - (EIN / EIS)) + EIT) - ACD;
                    EJA = EIU;
                } else {
                    let EIV = if EIN < KE { 1.0 } else { 0.0 };
                    let EIY = if EIV != 0.0 {
                        let EIW = (D + (EIN.exp())).ln();
                        EIW
                    } else {
                        EIN
                    };
                    let EIZ = ((EHX + (EIN / EIX)) + EIY) - ACD;
                    EJA = EIZ;
                }
                let EJB = EJA - AXB;
                let EJC = F * ((EJA + AXB) - (((EJB * EJB) + AJF).sqrt()));
                let EJD = ((D + ((HC * (AXB - EJC)) / AXE)).sqrt()) - D;
                let EJE = EJC + (AXE * EJD);
                let EJF = D + (AXH * EDR);
                let EJG = EJF - F;
                let EJH = F * ((EJF + F) + (((EJG * EJG) + O).sqrt()));
                let EJI = D / (D + (EDD * EJH));
                let EJJ = D / (D + (EDG * EJH));
                let EJK = (AXO * (D + (AXP * EJD))) * (D + (AXQ * EDR));
                let EJL = EDJ * EJK;
                let EJM = ((((EDQ - EJE) + EJL) * EJI) + EJE) + AQL;
                let EJN = ((((EHX - EJE) + (EDK * EJK)) * EJJ) + EJE) + AQL;
                let EJO = EJN + (AXV * (EJM - EJN));
                let EJP = EJO - AXX;
                let EJQ = F * ((EJO + AXX) - (((EJP * EJP) + O).sqrt()));
                let EJR = EJM + (AYC * (EJN - EJM));
                let EJS = EJR - AXX;
                let EJT = F * ((EJR + AXX) - (((EJS * EJS) + O).sqrt()));
                let EJU = EIS / EJI;
                let EJV = EIX / EJJ;
                let EJW = D / EJU;
                let EJX = D / EJV;
                let EJY = D / ((D + EJW) + EJX);
                let EKA = ARH / (EJZ * EJZ);
                let EKB = D + EJU;
                let EKC = D + EJV;
                let EKD = EKB / EKC;
                let EKE = EKD.ln();
                let EKF = if EKE > AGX { 1.0 } else { 0.0 };
                let EKQ = if EKF != 0.0 {
                    let EKG = ((HC * EKE) * (EKD + D)) / (EKD - D);
                    EKG
                } else {
                    let EKH = HC * (HC + EKE);
                    EKH
                };
                let EKI = EJY * (EJQ - EJT);
                let EKJ = EKI * EKI;
                let EKK = EKI * EJW;
                let EKL = EJQ - EKK;
                let EKM = EKI * EJX;
                let EKN = EJT + EKM;
                let EKO = D / EKB;
                let EKP = D / EKC;
                let EKR = (((EJU + (EJV * EKP)) * EKQ) / EKA).ln();
                let EKS = EKR + ATY;
                let EKT = (((EJV + (EJU * EKO)) * EKQ) / EKA).ln();
                let EKU = EKT + ATY;
                let EKV = (EKS - EKL) * KJ;
                let EKW = if EKV < KE { 1.0 } else { 0.0 };
                let EKY = if EKW != 0.0 {
                    let EKX = (D + (EKV.exp())).ln();
                    EKX
                } else {
                    EKV
                };
                let EKZ = EKS - (ATY * EKY);
                let ELA = (EKU - EKN) * KJ;
                let ELB = if ELA < KE { 1.0 } else { 0.0 };
                let ELD = if ELB != 0.0 {
                    let ELC = (D + (ELA.exp())).ln();
                    ELC
                } else {
                    ELA
                };
                let ELE = EJU * EJQ;
                let ELF = EJV * EJT;
                let ELG = (ELF + EKZ) * EKP;
                let ELH = (EKS - ((ELE + (EKU - (ATY * ELD))) * EKO)) * KJ;
                let ELI = if ELH < KE { 1.0 } else { 0.0 };
                let ELK = if ELI != 0.0 {
                    let ELJ = (D + (ELH.exp())).ln();
                    ELJ
                } else {
                    ELH
                };
                let ELL = EKS - (ATY * ELK);
                let ELM = (EKU - ELG) * KJ;
                let ELN = if ELM < KE { 1.0 } else { 0.0 };
                let ELP = if ELN != 0.0 {
                    let ELO = (D + (ELM.exp())).ln();
                    ELO
                } else {
                    ELM
                };
                let ELQ = EJQ - ELL;
                let ELR = EJT - (EKU - (ATY * ELP));
                let ELS = EJU * ELQ;
                let ELT = EJQ - ELQ;
                let ELU = if ELT < KE { 1.0 } else { 0.0 };
                let ELY = if ELU != 0.0 {
                    let ELV = ELT.exp();
                    ELV
                } else {
                    let ELW = ELT - KE;
                    let ELX = TP * (D + (ELW * (D + ((F * ELW) * (D + (ELW * KJ))))));
                    ELX
                };
                let ELZ = EKA * ELY;
                let EMA = (ELS * ELS) - ELZ;
                let EMB = HC * EJU;
                let EMC = (EMB * ELS) + ELZ;
                let EMD = EMB * EJU;
                let EME = EMD - ELZ;
                let EMF = if EMA < -5e-3f64 { 1.0 } else { 0.0 };
                let ENI;
                let ENM;
                let ENV;
                let ENZ;
                let EOC;
                let EOJ;
                let EOM;
                if EMF != 0.0 {
                    let EMG = (EMA.abs()).sqrt();
                    let EMH = EMG / ((F * EMG).tan());
                    let EMI = (ALG * EMC) / EMA;
                    let EMJ = (EMA + (EMH * (HC - EMH))) * EMI;
                    let EMK = ((EMC - ((HC * EMJ) * (D + EMH))) * EMI) + ((EMJ * EME) / EMC);
                    let EML = D - (F * EMH);
                    let EMM = (EMC / EMA) * EML;
                    let EMN = ((EME * EML) - (EMC * (EMM + (F * EMJ)))) / EMA;
                    ENI = A;
                    ENM = EMG;
                    ENV = EMH;
                    ENZ = EMJ;
                    EOC = EMK;
                    EOJ = EMM;
                    EOM = EMN;
                } else {
                    let EMO = if EMA > BAS { 1.0 } else { 0.0 };
                    let ENJ;
                    let ENN;
                    let ENW;
                    let EOA;
                    let EOD;
                    let EOK;
                    let EON;
                    if EMO != 0.0 {
                        let EMP = (EMA.abs()).sqrt();
                        let EMQ = (-EMP).exp();
                        let EMR = (EMP * (D + EMQ)) / (D - EMQ);
                        let EMS = (ALG * EMC) / EMA;
                        let EMT = (EMA + (EMR * (HC - EMR))) * EMS;
                        let EMU = ((EMC - ((HC * EMT) * (D + EMR))) * EMS) + ((EMT * EME) / EMC);
                        let EMV = D - (F * EMR);
                        let EMW = (EMC / EMA) * EMV;
                        let EMX = ((EME * EMV) - (EMC * (EMW + (F * EMT)))) / EMA;
                        ENJ = EMQ;
                        ENN = EMP;
                        ENW = EMR;
                        EOA = EMT;
                        EOD = EMU;
                        EOK = EMW;
                        EON = EMX;
                    } else {
                        let EMY = EMA * BBN;
                        let EMZ = ASH * (D - ((EMA * BBM) * (D - (EMY * (D - (EMA * BBP))))));
                        let ENA = HC + (EMA * EMZ);
                        let ENB = EMA * BBS;
                        let ENC = ASH * (D - (ENB * (D - ((EMA * BBU) * (D - ENB)))));
                        let END = EMC * ENC;
                        let ENE = (EME * ENC) - ((EMC * EMC) * (BBX * (D - ((EMA * BBY) * (D - ((AAI * EMA) * (D - (BBZ * EMA))))))));
                        let ENF = (-5e-1f64 * EMC) * EMZ;
                        let ENG = ((-5e-1f64 * EME) * EMZ) + (((1.3888888889e-3f64 * EMC) * EMC) * (D - (EMY * (HC - (BCC * EMA)))));
                        ENJ = A;
                        ENN = A;
                        ENW = ENA;
                        EOA = END;
                        EOD = ENE;
                        EOK = ENF;
                        EON = ENG;
                    }
                    ENI = ENJ;
                    ENM = ENN;
                    ENV = ENW;
                    ENZ = EOA;
                    EOC = EOD;
                    EOJ = EOK;
                    EOM = EON;
                }
                let ENH = if EMA > BAS { 1.0 } else { 0.0 };
                let EOG;
                let EPD;
                if ENH != 0.0 {
                    let ENK = (AJF * EMA) / (D - (ENI * (HC - ENI)));
                    let ENL = ENK * ENI;
                    let ENO = (ENK.ln()) - ENM;
                    EOG = ENL;
                    EPD = ENO;
                } else {
                    let ENP = if EMA < -5e-3f64 { 1.0 } else { 0.0 };
                    let EOH;
                    let EPE;
                    if ENP != 0.0 {
                        let ENQ = (F * ENM).sin();
                        let ENR = (-EMA) / (ENQ * ENQ);
                        let ENS = ENR.ln();
                        EOH = ENR;
                        EPE = ENS;
                    } else {
                        let ENT = AJF - ((EMA * KJ) * (D - ((AAI * EMA) * (D - (BCQ * EMA)))));
                        let ENU = ENT.ln();
                        EOH = ENT;
                        EPE = ENU;
                    }
                    EOG = EOH;
                    EPD = EPE;
                }
                let ENX = if ((BCT * ELS) + ENV) > A { 1.0 } else { 0.0 };
                let EOP;
                let EOT;
                let EOV;
                if ENX != 0.0 {
                    let ENY = ELS + ENV;
                    let EOB = EJU + ENZ;
                    EOP = ENY;
                    EOT = EOB;
                    EOV = EOC;
                } else {
                    let EOE = D / (ELS - ENV);
                    let EOF = ENZ - EJU;
                    let EOI = (ELZ - EOG) * EOE;
                    let EOL = (((EOF * EOI) - ELZ) - (EOJ * EOG)) * EOE;
                    let EOO = ((((EOC * EOI) + ((HC * EOF) * EOL)) + ELZ) - ((EOM + (EOJ * EOJ)) * EOG)) * EOE;
                    EOP = EOI;
                    EOT = EOL;
                    EOV = EOO;
                }
                let EOQ = if EOP > A { 1.0 } else { 0.0 };
                let EPC;
                let EPF;
                let EPG;
                if EOQ != 0.0 {
                    let EOR = EOP.ln();
                    let EOS = D / EOP;
                    let EOU = EOT * EOS;
                    let EOW = (EOV * EOS) - (EOU * EOU);
                    EPC = EOR;
                    EPF = EOU;
                    EPG = EOW;
                } else {
                    let EOX = (ELS + ACD) + ((-ELS).ln());
                    let EOY = D / ELQ;
                    let EOZ = EJU + EOY;
                    let EPA = (-EOY) * EOY;
                    EPC = EOX;
                    EPF = EOZ;
                    EPG = EPA;
                }
                let EPB = EJT - EJQ;
                let EPH = ELS + (EJV * (((EPB + ELQ) + (HC * EPC)) - EPD));
                let EPI = EJU + (EJV * ((D + (HC * EPF)) - EOJ));
                let EPJ = (EPH * EOP) - ELZ;
                let EPK = ((EPI * EOP) + (EPH * EOT)) + ELZ;
                let EPL = (EPK * EPK) - ((F * EPJ) * (((((EJV * ((HC * EPG) - EOM)) * EOP) + ((HC * EPI) * EOT)) + (EPH * EOV)) - ELZ));
                let EPM = ELQ + ((((-EPJ) * EPK) * EPL) / ((EPL * EPL) + BEL));
                let EPN = EJU * EPM;
                let EPO = EJV * ELR;
                let EPP = EPN + EPO;
                let EPQ = D + (BEQ * EPP);
                let EPR = EPN * EPO;
                let EPS = (BES + (BET * EPP)) + EPR;
                let EPT = (EPN * EPN) - (((((EPS * EPS) - ((AJF * EPQ) * (BES * ((HC * EPP) + EPR)))).sqrt()) - EPS) / (HC * EPQ));
                let EPU = if EPT > A { 1.0 } else { 0.0 };
                let EQA;
                if EPU != 0.0 {
                    let EPV = EPT * ((((EPT / EKA).ln()) - EJQ) + EPM);
                    let EPW = (EMB * EPN) + EPT;
                    let EPX = (EJQ - EPM) - EKS;
                    let EPY = if (if (if (if EPV < A { 1.0 } else { 0.0 }) != 0.0 && (if EPW > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((EPX + BFB) + (EJU.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if EPX > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EQB = if EPY != 0.0 {
                        let EPZ = EPM - (EPV / EPW);
                        EPZ
                    } else {
                        EPM
                    };
                    EQA = EQB;
                } else {
                    EQA = EPM;
                }
                let EQC = EJU * EQA;
                let EQD = EQC + EPO;
                let EQE = D + (BEQ * EQD);
                let EQF = EQC * EPO;
                let EQG = (BES + (BET * EQD)) + EQF;
                let EQH = ((((EQG * EQG) - ((AJF * EQE) * (BES * ((HC * EQD) + EQF)))).sqrt()) - EQG) / (HC * EQE);
                let EQI = if EQH < -5e-3f64 { 1.0 } else { 0.0 };
                let EQU;
                let EQW;
                let ESW;
                let ETD;
                if EQI != 0.0 {
                    let EQJ = (EQH.abs()).sqrt();
                    let EQK = EQJ / ((F * EQJ).tan());
                    let EQL = (ALG * (EQH + (EQK * (HC - EQK)))) / EQH;
                    EQU = EQK;
                    EQW = EQL;
                    ESW = ENI;
                    ETD = EQJ;
                } else {
                    let EQM = if EQH > BAS { 1.0 } else { 0.0 };
                    let EQV;
                    let EQX;
                    let ESX;
                    let ETE;
                    if EQM != 0.0 {
                        let EQN = (EQH.abs()).sqrt();
                        let EQO = (-EQN).exp();
                        let EQP = (EQN * (D + EQO)) / (D - EQO);
                        let EQQ = (ALG * (EQH + (EQP * (HC - EQP)))) / EQH;
                        EQV = EQP;
                        EQX = EQQ;
                        ESX = EQO;
                        ETE = EQN;
                    } else {
                        let EQR = HC + ((EQH * ASH) * (D - ((EQH * BBM) * (D - (EQH * BBN)))));
                        let EQS = EQH * BBS;
                        let EQT = ASH * (D - (EQS * (D - ((EQH * BBU) * (D - EQS)))));
                        EQV = EQR;
                        EQX = EQT;
                        ESX = ENI;
                        ETE = ENM;
                    }
                    EQU = EQV;
                    EQW = EQX;
                    ESW = ESX;
                    ETD = ETE;
                }
                let EQY = (EQC * EQC) - (EQH - ((((EQD * EQU) + EQF) + EQH) / ((EQD * EQW) + D)));
                let EQZ = if EQY > A { 1.0 } else { 0.0 };
                let ERF;
                if EQZ != 0.0 {
                    let ERA = EQY * ((((EQY / EKA).ln()) - EJQ) + EQA);
                    let ERB = (EMB * EQC) + EQY;
                    let ERC = (EJQ - EQA) - EKS;
                    let ERD = if (if (if (if ERA < A { 1.0 } else { 0.0 }) != 0.0 && (if ERB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((ERC + BFB) + (EJU.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if ERC > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ERG = if ERD != 0.0 {
                        let ERE = EQA - (ERA / ERB);
                        ERE
                    } else {
                        EQA
                    };
                    ERF = ERG;
                } else {
                    ERF = EQA;
                }
                let ERH = EJU * ERF;
                let ERI = EJQ - ERF;
                let ERJ = if ERI < KE { 1.0 } else { 0.0 };
                let ERN = if ERJ != 0.0 {
                    let ERK = ERI.exp();
                    ERK
                } else {
                    let ERL = ERI - KE;
                    let ERM = TP * (D + (ERL * (D + ((F * ERL) * (D + (ERL * KJ))))));
                    ERM
                };
                let ERO = EKA * ERN;
                let ERP = (ERH * ERH) - ERO;
                let ERQ = (EMB * ERH) + ERO;
                let ERR = EMD - ERO;
                let ERS = if ERP < -5e-3f64 { 1.0 } else { 0.0 };
                let ESV;
                let ETB;
                let ETM;
                let ETQ;
                let ETT;
                let EUA;
                let EUD;
                if ERS != 0.0 {
                    let ERT = (ERP.abs()).sqrt();
                    let ERU = ERT / ((F * ERT).tan());
                    let ERV = (ALG * ERQ) / ERP;
                    let ERW = (ERP + (ERU * (HC - ERU))) * ERV;
                    let ERX = ((ERQ - ((HC * ERW) * (D + ERU))) * ERV) + ((ERW * ERR) / ERQ);
                    let ERY = D - (F * ERU);
                    let ERZ = (ERQ / ERP) * ERY;
                    let ESA = ((ERR * ERY) - (ERQ * (ERZ + (F * ERW)))) / ERP;
                    ESV = ESW;
                    ETB = ERT;
                    ETM = ERU;
                    ETQ = ERW;
                    ETT = ERX;
                    EUA = ERZ;
                    EUD = ESA;
                } else {
                    let ESB = if ERP > BAS { 1.0 } else { 0.0 };
                    let ESY;
                    let ETC;
                    let ETN;
                    let ETR;
                    let ETU;
                    let EUB;
                    let EUE;
                    if ESB != 0.0 {
                        let ESC = (ERP.abs()).sqrt();
                        let ESD = (-ESC).exp();
                        let ESE = (ESC * (D + ESD)) / (D - ESD);
                        let ESF = (ALG * ERQ) / ERP;
                        let ESG = (ERP + (ESE * (HC - ESE))) * ESF;
                        let ESH = ((ERQ - ((HC * ESG) * (D + ESE))) * ESF) + ((ESG * ERR) / ERQ);
                        let ESI = D - (F * ESE);
                        let ESJ = (ERQ / ERP) * ESI;
                        let ESK = ((ERR * ESI) - (ERQ * (ESJ + (F * ESG)))) / ERP;
                        ESY = ESD;
                        ETC = ESC;
                        ETN = ESE;
                        ETR = ESG;
                        ETU = ESH;
                        EUB = ESJ;
                        EUE = ESK;
                    } else {
                        let ESL = ERP * BBN;
                        let ESM = ASH * (D - ((ERP * BBM) * (D - (ESL * (D - (ERP * BBP))))));
                        let ESN = HC + (ERP * ESM);
                        let ESO = ERP * BBS;
                        let ESP = ASH * (D - (ESO * (D - ((ERP * BBU) * (D - ESO)))));
                        let ESQ = ERQ * ESP;
                        let ESR = (ERR * ESP) - ((ERQ * ERQ) * (BBX * (D - ((ERP * BBY) * (D - ((AAI * ERP) * (D - (BBZ * ERP))))))));
                        let ESS = (-5e-1f64 * ERQ) * ESM;
                        let EST = ((-5e-1f64 * ERR) * ESM) + (((1.3888888889e-3f64 * ERQ) * ERQ) * (D - (ESL * (HC - (BCC * ERP)))));
                        ESY = ESW;
                        ETC = ETD;
                        ETN = ESN;
                        ETR = ESQ;
                        ETU = ESR;
                        EUB = ESS;
                        EUE = EST;
                    }
                    ESV = ESY;
                    ETB = ETC;
                    ETM = ETN;
                    ETQ = ETR;
                    ETT = ETU;
                    EUA = EUB;
                    EUD = EUE;
                }
                let ESU = if ERP > BAS { 1.0 } else { 0.0 };
                let ETX;
                let EUT;
                if ESU != 0.0 {
                    let ESZ = (AJF * ERP) / (D - (ESV * (HC - ESV)));
                    let ETA = ESZ * ESV;
                    let ETF = (ESZ.ln()) - ETB;
                    ETX = ETA;
                    EUT = ETF;
                } else {
                    let ETG = if ERP < -5e-3f64 { 1.0 } else { 0.0 };
                    let ETY;
                    let EUU;
                    if ETG != 0.0 {
                        let ETH = (F * ETB).sin();
                        let ETI = (-ERP) / (ETH * ETH);
                        let ETJ = ETI.ln();
                        ETY = ETI;
                        EUU = ETJ;
                    } else {
                        let ETK = AJF - ((ERP * KJ) * (D - ((AAI * ERP) * (D - (BCQ * ERP)))));
                        let ETL = ETK.ln();
                        ETY = ETK;
                        EUU = ETL;
                    }
                    ETX = ETY;
                    EUT = EUU;
                }
                let ETO = if ((BCT * ERH) + ETM) > A { 1.0 } else { 0.0 };
                let EUG;
                let EUK;
                let EUM;
                if ETO != 0.0 {
                    let ETP = ERH + ETM;
                    let ETS = EJU + ETQ;
                    EUG = ETP;
                    EUK = ETS;
                    EUM = ETT;
                } else {
                    let ETV = D / (ERH - ETM);
                    let ETW = ETQ - EJU;
                    let ETZ = (ERO - ETX) * ETV;
                    let EUC = (((ETW * ETZ) - ERO) - (EUA * ETX)) * ETV;
                    let EUF = ((((ETT * ETZ) + ((HC * ETW) * EUC)) + ERO) - ((EUD + (EUA * EUA)) * ETX)) * ETV;
                    EUG = ETZ;
                    EUK = EUC;
                    EUM = EUF;
                }
                let EUH = if EUG > A { 1.0 } else { 0.0 };
                let EUS;
                let EUV;
                let EUW;
                if EUH != 0.0 {
                    let EUI = EUG.ln();
                    let EUJ = D / EUG;
                    let EUL = EUK * EUJ;
                    let EUN = (EUM * EUJ) - (EUL * EUL);
                    EUS = EUI;
                    EUV = EUL;
                    EUW = EUN;
                } else {
                    let EUO = (ERH + ACD) + ((-ERH).ln());
                    let EUP = D / ERF;
                    let EUQ = EJU + EUP;
                    let EUR = (-EUP) * EUP;
                    EUS = EUO;
                    EUV = EUQ;
                    EUW = EUR;
                }
                let EUX = ERH + (EJV * (((EPB + ERF) + (HC * EUS)) - EUT));
                let EUY = EJU + (EJV * ((D + (HC * EUV)) - EUA));
                let EUZ = (EUX * EUG) - ERO;
                let EVA = ((EUY * EUG) + (EUX * EUK)) + ERO;
                let EVB = (EVA * EVA) - ((F * EUZ) * (((((EJV * ((HC * EUW) - EUD)) * EUG) + ((HC * EUY) * EUK)) + (EUX * EUM)) - ERO));
                let EVC = ERF + ((((-EUZ) * EVA) * EVB) / ((EVB * EVB) + BEL));
                let EVD = EJU * EVC;
                let EVE = EJQ - EVC;
                let EVF = if EVE < KE { 1.0 } else { 0.0 };
                let EVJ = if EVF != 0.0 {
                    let EVG = EVE.exp();
                    EVG
                } else {
                    let EVH = EVE - KE;
                    let EVI = TP * (D + (EVH * (D + ((F * EVH) * (D + (EVH * KJ))))));
                    EVI
                };
                let EVK = EKA * EVJ;
                let EVL = (EVD * EVD) - EVK;
                let EVM = (EMB * EVD) + EVK;
                let EVN = EMD - EVK;
                let EVO = if EVL < -5e-3f64 { 1.0 } else { 0.0 };
                let EWR;
                let EWV;
                let EXE;
                let EXI;
                let EXL;
                let EXS;
                let EXV;
                if EVO != 0.0 {
                    let EVP = (EVL.abs()).sqrt();
                    let EVQ = EVP / ((F * EVP).tan());
                    let EVR = (ALG * EVM) / EVL;
                    let EVS = (EVL + (EVQ * (HC - EVQ))) * EVR;
                    let EVT = ((EVM - ((HC * EVS) * (D + EVQ))) * EVR) + ((EVS * EVN) / EVM);
                    let EVU = D - (F * EVQ);
                    let EVV = (EVM / EVL) * EVU;
                    let EVW = ((EVN * EVU) - (EVM * (EVV + (F * EVS)))) / EVL;
                    EWR = ESV;
                    EWV = EVP;
                    EXE = EVQ;
                    EXI = EVS;
                    EXL = EVT;
                    EXS = EVV;
                    EXV = EVW;
                } else {
                    let EVX = if EVL > BAS { 1.0 } else { 0.0 };
                    let EWS;
                    let EWW;
                    let EXF;
                    let EXJ;
                    let EXM;
                    let EXT;
                    let EXW;
                    if EVX != 0.0 {
                        let EVY = (EVL.abs()).sqrt();
                        let EVZ = (-EVY).exp();
                        let EWA = (EVY * (D + EVZ)) / (D - EVZ);
                        let EWB = (ALG * EVM) / EVL;
                        let EWC = (EVL + (EWA * (HC - EWA))) * EWB;
                        let EWD = ((EVM - ((HC * EWC) * (D + EWA))) * EWB) + ((EWC * EVN) / EVM);
                        let EWE = D - (F * EWA);
                        let EWF = (EVM / EVL) * EWE;
                        let EWG = ((EVN * EWE) - (EVM * (EWF + (F * EWC)))) / EVL;
                        EWS = EVZ;
                        EWW = EVY;
                        EXF = EWA;
                        EXJ = EWC;
                        EXM = EWD;
                        EXT = EWF;
                        EXW = EWG;
                    } else {
                        let EWH = EVL * BBN;
                        let EWI = ASH * (D - ((EVL * BBM) * (D - (EWH * (D - (EVL * BBP))))));
                        let EWJ = HC + (EVL * EWI);
                        let EWK = EVL * BBS;
                        let EWL = ASH * (D - (EWK * (D - ((EVL * BBU) * (D - EWK)))));
                        let EWM = EVM * EWL;
                        let EWN = (EVN * EWL) - ((EVM * EVM) * (BBX * (D - ((EVL * BBY) * (D - ((AAI * EVL) * (D - (BBZ * EVL))))))));
                        let EWO = (-5e-1f64 * EVM) * EWI;
                        let EWP = ((-5e-1f64 * EVN) * EWI) + (((1.3888888889e-3f64 * EVM) * EVM) * (D - (EWH * (HC - (BCC * EVL)))));
                        EWS = ESV;
                        EWW = ETB;
                        EXF = EWJ;
                        EXJ = EWM;
                        EXM = EWN;
                        EXT = EWO;
                        EXW = EWP;
                    }
                    EWR = EWS;
                    EWV = EWW;
                    EXE = EXF;
                    EXI = EXJ;
                    EXL = EXM;
                    EXS = EXT;
                    EXV = EXW;
                }
                let EWQ = if EVL > BAS { 1.0 } else { 0.0 };
                let EXP;
                let EYL;
                if EWQ != 0.0 {
                    let EWT = (AJF * EVL) / (D - (EWR * (HC - EWR)));
                    let EWU = EWT * EWR;
                    let EWX = (EWT.ln()) - EWV;
                    EXP = EWU;
                    EYL = EWX;
                } else {
                    let EWY = if EVL < -5e-3f64 { 1.0 } else { 0.0 };
                    let EXQ;
                    let EYM;
                    if EWY != 0.0 {
                        let EWZ = (F * EWV).sin();
                        let EXA = (-EVL) / (EWZ * EWZ);
                        let EXB = EXA.ln();
                        EXQ = EXA;
                        EYM = EXB;
                    } else {
                        let EXC = AJF - ((EVL * KJ) * (D - ((AAI * EVL) * (D - (BCQ * EVL)))));
                        let EXD = EXC.ln();
                        EXQ = EXC;
                        EYM = EXD;
                    }
                    EXP = EXQ;
                    EYL = EYM;
                }
                let EXG = if ((BCT * EVD) + EXE) > A { 1.0 } else { 0.0 };
                let EXY;
                let EYC;
                let EYE;
                if EXG != 0.0 {
                    let EXH = EVD + EXE;
                    let EXK = EJU + EXI;
                    EXY = EXH;
                    EYC = EXK;
                    EYE = EXL;
                } else {
                    let EXN = D / (EVD - EXE);
                    let EXO = EXI - EJU;
                    let EXR = (EVK - EXP) * EXN;
                    let EXU = (((EXO * EXR) - EVK) - (EXS * EXP)) * EXN;
                    let EXX = ((((EXL * EXR) + ((HC * EXO) * EXU)) + EVK) - ((EXV + (EXS * EXS)) * EXP)) * EXN;
                    EXY = EXR;
                    EYC = EXU;
                    EYE = EXX;
                }
                let EXZ = if EXY > A { 1.0 } else { 0.0 };
                let EYK;
                let EYN;
                let EYO;
                if EXZ != 0.0 {
                    let EYA = EXY.ln();
                    let EYB = D / EXY;
                    let EYD = EYC * EYB;
                    let EYF = (EYE * EYB) - (EYD * EYD);
                    EYK = EYA;
                    EYN = EYD;
                    EYO = EYF;
                } else {
                    let EYG = (EVD + ACD) + ((-EVD).ln());
                    let EYH = D / EVC;
                    let EYI = EJU + EYH;
                    let EYJ = (-EYH) * EYH;
                    EYK = EYG;
                    EYN = EYI;
                    EYO = EYJ;
                }
                let EYP = EVD + (EJV * (((EPB + EVC) + (HC * EYK)) - EYL));
                let EYQ = EJU + (EJV * ((D + (HC * EYN)) - EXS));
                let EYR = (EYP * EXY) - EVK;
                let EYS = ((EYQ * EXY) + (EYP * EYC)) + EVK;
                let EYT = (EYS * EYS) - ((F * EYR) * (((((EJV * ((HC * EYO) - EXV)) * EXY) + ((HC * EYQ) * EYC)) + (EYP * EYE)) - EVK));
                let EYU = (((-EYR) * EYS) * EYT) / ((EYT * EYT) + BEL);
                let EYV = EVC + EYU;
                let FCP;
                let FDV;
                let FEA;
                if E != 0.0 {
                    let EYW = if (EYU.abs()) > O { 1.0 } else { 0.0 };
                    let FCQ;
                    let FDW;
                    let FEB;
                    if EYW != 0.0 {
                        let EYX = EJU * EYV;
                        let EYY = EJQ - EYV;
                        let EYZ = if EYY < KE { 1.0 } else { 0.0 };
                        let EZD = if EYZ != 0.0 {
                            let EZA = EYY.exp();
                            EZA
                        } else {
                            let EZB = EYY - KE;
                            let EZC = TP * (D + (EZB * (D + ((F * EZB) * (D + (EZB * KJ))))));
                            EZC
                        };
                        let EZE = EKA * EZD;
                        let EZF = (EYX * EYX) - EZE;
                        let EZG = (EMB * EYX) + EZE;
                        let EZH = EMD - EZE;
                        let EZI = if EZF < -5e-3f64 { 1.0 } else { 0.0 };
                        let FAL;
                        let FAP;
                        let FAY;
                        let FBC;
                        let FBF;
                        let FBM;
                        let FBP;
                        if EZI != 0.0 {
                            let EZJ = (EZF.abs()).sqrt();
                            let EZK = EZJ / ((F * EZJ).tan());
                            let EZL = (ALG * EZG) / EZF;
                            let EZM = (EZF + (EZK * (HC - EZK))) * EZL;
                            let EZN = ((EZG - ((HC * EZM) * (D + EZK))) * EZL) + ((EZM * EZH) / EZG);
                            let EZO = D - (F * EZK);
                            let EZP = (EZG / EZF) * EZO;
                            let EZQ = ((EZH * EZO) - (EZG * (EZP + (F * EZM)))) / EZF;
                            FAL = EWR;
                            FAP = EZJ;
                            FAY = EZK;
                            FBC = EZM;
                            FBF = EZN;
                            FBM = EZP;
                            FBP = EZQ;
                        } else {
                            let EZR = if EZF > BAS { 1.0 } else { 0.0 };
                            let FAM;
                            let FAQ;
                            let FAZ;
                            let FBD;
                            let FBG;
                            let FBN;
                            let FBQ;
                            if EZR != 0.0 {
                                let EZS = (EZF.abs()).sqrt();
                                let EZT = (-EZS).exp();
                                let EZU = (EZS * (D + EZT)) / (D - EZT);
                                let EZV = (ALG * EZG) / EZF;
                                let EZW = (EZF + (EZU * (HC - EZU))) * EZV;
                                let EZX = ((EZG - ((HC * EZW) * (D + EZU))) * EZV) + ((EZW * EZH) / EZG);
                                let EZY = D - (F * EZU);
                                let EZZ = (EZG / EZF) * EZY;
                                let FAA = ((EZH * EZY) - (EZG * (EZZ + (F * EZW)))) / EZF;
                                FAM = EZT;
                                FAQ = EZS;
                                FAZ = EZU;
                                FBD = EZW;
                                FBG = EZX;
                                FBN = EZZ;
                                FBQ = FAA;
                            } else {
                                let FAB = EZF * BBN;
                                let FAC = ASH * (D - ((EZF * BBM) * (D - (FAB * (D - (EZF * BBP))))));
                                let FAD = HC + (EZF * FAC);
                                let FAE = EZF * BBS;
                                let FAF = ASH * (D - (FAE * (D - ((EZF * BBU) * (D - FAE)))));
                                let FAG = EZG * FAF;
                                let FAH = (EZH * FAF) - ((EZG * EZG) * (BBX * (D - ((EZF * BBY) * (D - ((AAI * EZF) * (D - (BBZ * EZF))))))));
                                let FAI = (-5e-1f64 * EZG) * FAC;
                                let FAJ = ((-5e-1f64 * EZH) * FAC) + (((1.3888888889e-3f64 * EZG) * EZG) * (D - (FAB * (HC - (BCC * EZF)))));
                                FAM = EWR;
                                FAQ = EWV;
                                FAZ = FAD;
                                FBD = FAG;
                                FBG = FAH;
                                FBN = FAI;
                                FBQ = FAJ;
                            }
                            FAL = FAM;
                            FAP = FAQ;
                            FAY = FAZ;
                            FBC = FBD;
                            FBF = FBG;
                            FBM = FBN;
                            FBP = FBQ;
                        }
                        let FAK = if EZF > BAS { 1.0 } else { 0.0 };
                        let FBJ;
                        let FCF;
                        if FAK != 0.0 {
                            let FAN = (AJF * EZF) / (D - (FAL * (HC - FAL)));
                            let FAO = FAN * FAL;
                            let FAR = (FAN.ln()) - FAP;
                            FBJ = FAO;
                            FCF = FAR;
                        } else {
                            let FAS = if EZF < -5e-3f64 { 1.0 } else { 0.0 };
                            let FBK;
                            let FCG;
                            if FAS != 0.0 {
                                let FAT = (F * FAP).sin();
                                let FAU = (-EZF) / (FAT * FAT);
                                let FAV = FAU.ln();
                                FBK = FAU;
                                FCG = FAV;
                            } else {
                                let FAW = AJF - ((EZF * KJ) * (D - ((AAI * EZF) * (D - (BCQ * EZF)))));
                                let FAX = FAW.ln();
                                FBK = FAW;
                                FCG = FAX;
                            }
                            FBJ = FBK;
                            FCF = FCG;
                        }
                        let FBA = if ((BCT * EYX) + FAY) > A { 1.0 } else { 0.0 };
                        let FBS;
                        let FBW;
                        let FBY;
                        if FBA != 0.0 {
                            let FBB = EYX + FAY;
                            let FBE = EJU + FBC;
                            FBS = FBB;
                            FBW = FBE;
                            FBY = FBF;
                        } else {
                            let FBH = D / (EYX - FAY);
                            let FBI = FBC - EJU;
                            let FBL = (EZE - FBJ) * FBH;
                            let FBO = (((FBI * FBL) - EZE) - (FBM * FBJ)) * FBH;
                            let FBR = ((((FBF * FBL) + ((HC * FBI) * FBO)) + EZE) - ((FBP + (FBM * FBM)) * FBJ)) * FBH;
                            FBS = FBL;
                            FBW = FBO;
                            FBY = FBR;
                        }
                        let FBT = if FBS > A { 1.0 } else { 0.0 };
                        let FCE;
                        let FCH;
                        let FCI;
                        if FBT != 0.0 {
                            let FBU = FBS.ln();
                            let FBV = D / FBS;
                            let FBX = FBW * FBV;
                            let FBZ = (FBY * FBV) - (FBX * FBX);
                            FCE = FBU;
                            FCH = FBX;
                            FCI = FBZ;
                        } else {
                            let FCA = (EYX + ACD) + ((-EYX).ln());
                            let FCB = D / EYV;
                            let FCC = EJU + FCB;
                            let FCD = (-FCB) * FCB;
                            FCE = FCA;
                            FCH = FCC;
                            FCI = FCD;
                        }
                        let FCJ = EYX + (EJV * (((EPB + EYV) + (HC * FCE)) - FCF));
                        let FCK = EJU + (EJV * ((D + (HC * FCH)) - FBM));
                        let FCL = (FCJ * FBS) - EZE;
                        let FCM = ((FCK * FBS) + (FCJ * FBW)) + EZE;
                        let FCN = (FCM * FCM) - ((F * FCL) * (((((EJV * ((HC * FCI) - FBP)) * FBS) + ((HC * FCK) * FBW)) + (FCJ * FBY)) - EZE));
                        let FCO = EYV + ((((-FCL) * FCM) * FCN) / ((FCN * FCN) + BEL));
                        FCQ = FCO;
                        FDW = FAL;
                        FEB = FAP;
                    } else {
                        FCQ = EYV;
                        FDW = EWR;
                        FEB = EWV;
                    }
                    FCP = FCQ;
                    FDV = FDW;
                    FEA = FEB;
                } else {
                    FCP = EYV;
                    FDV = EWR;
                    FEA = EWV;
                }
                let FCR = EJU * FCP;
                let FCS = EJQ - FCP;
                let FCT = if FCS < KE { 1.0 } else { 0.0 };
                let FCX = if FCT != 0.0 {
                    let FCU = FCS.exp();
                    FCU
                } else {
                    let FCV = FCS - KE;
                    let FCW = TP * (D + (FCV * (D + ((F * FCV) * (D + (FCV * KJ))))));
                    FCW
                };
                let FCY = EKA * FCX;
                let FCZ = (FCR * FCR) - FCY;
                let FDA = if FCY <= A { 1.0 } else { 0.0 };
                let FFD;
                let FFN;
                let FFU;
                if FDA != 0.0 {
                    let FDB = BSF - FCR;
                    let FDC = FDB / EJV;
                    FFD = FDC;
                    FFN = BSF;
                    FFU = FDB;
                } else {
                    let FDD = if FCZ < -5e-3f64 { 1.0 } else { 0.0 };
                    let FDL;
                    let FDU;
                    let FDY;
                    if FDD != 0.0 {
                        let FDE = (FCZ.abs()).sqrt();
                        let FDF = FDE / ((F * FDE).tan());
                        FDL = FDF;
                        FDU = FDV;
                        FDY = FDE;
                    } else {
                        let FDG = if FCZ > BAS { 1.0 } else { 0.0 };
                        let FDM;
                        let FDX;
                        let FDZ;
                        if FDG != 0.0 {
                            let FDH = (FCZ.abs()).sqrt();
                            let FDI = (-FDH).exp();
                            let FDJ = (FDH * (D + FDI)) / (D - FDI);
                            FDM = FDJ;
                            FDX = FDI;
                            FDZ = FDH;
                        } else {
                            let FDK = HC + ((FCZ * ASH) * (D - ((FCZ * BBM) * (D - (FCZ * BBN)))));
                            FDM = FDK;
                            FDX = FDV;
                            FDZ = FEA;
                        }
                        FDL = FDM;
                        FDU = FDX;
                        FDY = FDZ;
                    }
                    let FDN = if ((BCT * FCR) + FDL) > A { 1.0 } else { 0.0 };
                    let FFE;
                    let FFO;
                    let FFV;
                    if FDN != 0.0 {
                        let FDO = FCR + FDL;
                        let FDP = if (FCY * FCR) < (((BSU * FCR) * FCR) * FDO) { 1.0 } else { 0.0 };
                        let FFF;
                        let FFP;
                        let FFW;
                        if FDP != 0.0 {
                            let FDQ = (FCY / FDO) + BSF;
                            let FDR = FDQ - FCR;
                            let FDS = FDR / EJV;
                            FFF = FDS;
                            FFP = FDQ;
                            FFW = FDR;
                        } else {
                            let FDT = if FCZ > BAS { 1.0 } else { 0.0 };
                            let FEH;
                            if FDT != 0.0 {
                                let FEC = (((AJF * FCZ) / (D - (FDU * (HC - FDU)))).ln()) - FDY;
                                FEH = FEC;
                            } else {
                                let FED = if FCZ < -5e-3f64 { 1.0 } else { 0.0 };
                                let FEI = if FED != 0.0 {
                                    let FEE = (F * FDY).sin();
                                    let FEF = ((-FCZ) / (FEE * FEE)).ln();
                                    FEF
                                } else {
                                    let FEG = (AJF - ((FCZ * KJ) * (D - ((AAI * FCZ) * (D - (BCQ * FCZ)))))).ln();
                                    FEG
                                };
                                FEH = FEI;
                            }
                            let FEJ = ((EPB + FCP) + (HC * (FDO.ln()))) - FEH;
                            let FEK = EJV * FEJ;
                            let FEL = FCR + FEK;
                            FFF = FEJ;
                            FFP = FEL;
                            FFW = FEK;
                        }
                        FFE = FFF;
                        FFO = FFP;
                        FFV = FFW;
                    } else {
                        let FEM = if FCZ > BAS { 1.0 } else { 0.0 };
                        let FEY;
                        if FEM != 0.0 {
                            let FEN = (FCP - EJQ) - FDY;
                            let FEO = if FEN < KE { 1.0 } else { 0.0 };
                            let FES = if FEO != 0.0 {
                                let FEP = FEN.exp();
                                FEP
                            } else {
                                let FEQ = FEN - KE;
                                let FER = TP * (D + (FEQ * (D + ((F * FEQ) * (D + (FEQ * KJ))))));
                                FER
                            };
                            let FET = ((AJF * FCZ) * (FES / EKA)) / (D - (FDU * (HC - FDU)));
                            FEY = FET;
                        } else {
                            let FEU = if FCZ < -5e-3f64 { 1.0 } else { 0.0 };
                            let FEZ = if FEU != 0.0 {
                                let FEV = (F * FDY).sin();
                                let FEW = ((-FCZ) / (FEV * FEV)) / FCY;
                                FEW
                            } else {
                                let FEX = (AJF - ((FCZ * KJ) * (D - ((AAI * FCZ) * (D - (BCQ * FCZ)))))) / FCY;
                                FEX
                            };
                            FEY = FEZ;
                        }
                        let FFA = ((FCR - FDL) / (D - FEY)) + BSF;
                        let FFB = FFA - FCR;
                        let FFC = FFB / EJV;
                        FFE = FFC;
                        FFO = FFA;
                        FFV = FFB;
                    }
                    FFD = FFE;
                    FFN = FFO;
                    FFU = FFV;
                }
                let FFG = EJT - FFD;
                let FFH = if FFG < KE { 1.0 } else { 0.0 };
                let FFL = if FFH != 0.0 {
                    let FFI = FFG.exp();
                    FFI
                } else {
                    let FFJ = FFG - KE;
                    let FFK = TP * (D + (FFJ * (D + ((F * FFJ) * (D + (FFJ * KJ))))));
                    FFK
                };
                let FFM = EKA * FFL;
                let FFQ = if FFN > GW { 1.0 } else { 0.0 };
                let FIP;
                let FIR;
                let FIS;
                let FIT;
                if FFQ != 0.0 {
                    let FFR = FCY * EJW;
                    let FFS = FFM * EJX;
                    let FFT = FFR + (HC * FCR);
                    let FFX = FFS + (HC * FFU);
                    let FFY = ((HC * FFN) + FFR) + FFS;
                    let FFZ = if (FCZ.abs()) > BAS { 1.0 } else { 0.0 };
                    let FIQ = if FFZ != 0.0 {
                        let FGA = ((-4e0f64 * FCZ) * FFY) / (FFN * (((FFT * FFX) + ((HC * (FCP + HC)) * FFX)) + ((HC * (FFD + HC)) * FFT)));
                        FGA
                    } else {
                        let FGB = FCZ * BBS;
                        let FGC = ((FCY * FFM) * FFY) / (FFN * (((FFT * FCY) + (FFX * FFM)) + (((FFT * FFX) * FFN) * (D + (FFN * (ASH * (D - (FGB * (D - ((FCZ * BBU) * (D - FGB)))))))))));
                        FGC
                    };
                    FIP = FIQ;
                    FIR = FFY;
                    FIS = FFT;
                    FIT = FFX;
                } else {
                    FIP = A;
                    FIR = A;
                    FIS = A;
                    FIT = A;
                }
                let FGD = FFN.ln();
                let FGE = FCR / HC;
                let FGF = if FGE < KE { 1.0 } else { 0.0 };
                let FGH = if FGF != 0.0 {
                    let FGG = (D + (FGE.exp())).ln();
                    FGG
                } else {
                    FGE
                };
                let FGI = HC * FGH;
                let FGJ = FFU / HC;
                let FGK = if FGJ < KE { 1.0 } else { 0.0 };
                let FGM = if FGK != 0.0 {
                    let FGL = (D + (FGJ.exp())).ln();
                    FGL
                } else {
                    FGJ
                };
                let FGN = HC * FGM;
                let FGO = FGN - FFU;
                let FGP = FGI - FCR;
                let FGQ = (AHH * FGI) + (AHI * FGO);
                let FGR = (AHH * FGN) + (AHI * FGP);
                let FGS = FFN / (FGI + FGN);
                let FGT = (FGI * AGB) * BWA;
                let FGU = (FGN * AGF) * BWA;
                let FGV = BWD * (FGO + (BWE * FGP));
                let FGW = D + FGV;
                let FGX = D + (BWH * FGV);
                let FGY = (F * (FGW + (((FGW * FGW) + O).sqrt()))) / (F * (FGX + (((FGX * FGX) + O).sqrt())));
                let FGZ = (BWK * ((D + (BWL * FGO)) + (BWM * FGP))) * ((BWO * (((D + ((FGI * FGS) * AHA)) + ((FGN * FGS) * AHC)).ln())).exp());
                let FHF;
                if BWR != 0.0 {
                    FHF = D;
                } else {
                    let FHA = if BWQ < A { 1.0 } else { 0.0 };
                    let FHG = if FHA != 0.0 {
                        let FHB = D - (BWQ * ((BWT * ((FFN + BWU).ln())).exp()));
                        FHB
                    } else {
                        let FHC = D / (D + (BWQ * ((BWT * ((FFN + BWU).ln())).exp())));
                        FHC
                    };
                    FHF = FHG;
                }
                let FHD = D - (BWY * EDR);
                let FHE = ((BWX * EJZ) * F) * (FHD + (((FHD * FHD) + O).sqrt()));
                let FHH = FHE * ((FFN * FHF) + BXD);
                let FHI = (FGY * (FGT + FGU)) / ((FGT / (((D + ((BXF * (((BXG * FGQ) + GW).ln())).exp())) + FGZ) + (BXH * FHH))) + (FGU / (((D + ((BXF * (((BXG * FGR) + GW).ln())).exp())) + FGZ) + (BXI * FHH))));
                let FHJ = if (EKI.abs()) > BXK { 1.0 } else { 0.0 };
                let FIH;
                let FIX;
                let GJR;
                let GJS;
                let GJU;
                let GJV;
                if FHJ != 0.0 {
                    let FHK = if EKI > A { 1.0 } else { 0.0 };
                    let FHT;
                    let FHV;
                    let FIY;
                    if FHK != 0.0 {
                        let FHL = (-EKI).exp();
                        let FHM = EKI / (D - FHL);
                        let FHN = FHL * FHM;
                        let FHO = (((EKA / (FFN * FHM)).ln()) - ACD) + EKL;
                        FHT = FHM;
                        FHV = FHN;
                        FIY = FHO;
                    } else {
                        let FHP = EKI.exp();
                        let FHQ = EKI / (FHP - D);
                        let FHR = FHP * FHQ;
                        let FHS = (((EKA / (FFN * FHQ)).ln()) - ACD) + EKN;
                        FHT = FHR;
                        FHV = FHQ;
                        FIY = FHS;
                    }
                    let FHU = (-EKI) / (EJY * ((D - FHT) - EKM));
                    let FHW = EKI / (EJY * ((D - FHV) + EKK));
                    let FHX = EKI / ((((FHV * EJX) + F) / FHW) - (((FHT * EJW) + F) / FHU));
                    FIH = FHX;
                    FIX = FIY;
                    GJR = FHT;
                    GJS = FHU;
                    GJU = FHV;
                    GJV = FHW;
                } else {
                    let FHY = 8.333333333335e-2f64 * EKJ;
                    let FHZ = F * EKI;
                    let FIA = (D + FHZ) + FHY;
                    let FIB = (D - FHZ) + FHY;
                    let FIC = ASH * FHZ;
                    let FID = D / (EJY * ((F + EJX) + FIC));
                    let FIE = D / (EJY * ((F + EJW) - FIC));
                    let FIF = (((EKA / (FFN * (D - (F * FHY)))).ln()) - ACD) + (F * (EKL + EKN));
                    let FIG = -1.2e1f64 / ((((AJF - (ATY * EJY)) + ((ATJ * EJY) / (EJU * EJV))) + ((EJY * (EJW - EJX)) * EKI)) + ((KJ * (BWH - (ALG * EJY))) * EKJ));
                    FIH = FIG;
                    FIX = FIF;
                    GJR = FIA;
                    GJS = FID;
                    GJU = FIB;
                    GJV = FIE;
                }
                let FII = D / FIH;
                let FKA;
                let FKF;
                let GJA;
                if FFQ != 0.0 {
                    let FIJ = (BYL * FGI) / (BYL + FGI);
                    let FJA = if CYQ != 0.0 {
                        let FIK = D / (D - (BYN * FIJ));
                        FIK
                    } else {
                        let FIL = D + (BYN * FIJ);
                        FIL
                    };
                    let FIM = (BYL * FGN) / (BYL + FGN);
                    let FJB = if CYU != 0.0 {
                        let FIN = D / (D - (BYS * FIM));
                        FIN
                    } else {
                        let FIO = D + (BYS * FIM);
                        FIO
                    };
                    let FIU = ((FIP * FIR) / (FIS * FIT)) - (((FCY / FIS) + (FFM / FIT)) / FFN);
                    let FIV = (FIU * FFN) / (FIU + D);
                    let FIW = FIH - FIV;
                    let FIZ = (FFN + (FIH * FIX)) / FIW;
                    let FJC = ((EDL / FHI) * F) * (FJA + FJB);
                    let FJD = D - (FFN / FIV);
                    let FJE = D + FIX;
                    let FJF = (((((HC * FIV) - FFN) * FII) - HC) - FIX) * (F * (FIZ + (((FIZ * FIZ) + GW).sqrt())));
                    let FJG = if FJC > BZN { 1.0 } else { 0.0 };
                    let FJQ;
                    let FJR;
                    if FJG != 0.0 {
                        let FJH = HC / (FJC * FJC);
                        let FJI = FJH * FJD;
                        let FJJ = FJH + FJF;
                        let FJK = FJH * FJE;
                        let FJL = (((FJI * FJI) + (((BZT * FJH) * FJH) * FJH)) + VV).sqrt();
                        let FJM = (((FJK * FJK) + (((BZT * FJJ) * FJJ) * FJJ)) + VV).sqrt();
                        let FJN = ((KJ * ((F * (FJL + FJI)).ln())).exp()) - ((KJ * ((F * (FJL - FJI)).ln())).exp());
                        let FJO = ((KJ * ((F * (FJM + FJK)).ln())).exp()) - ((KJ * ((F * (FJM - FJK)).ln())).exp());
                        FJQ = FJN;
                        FJR = FJO;
                    } else {
                        FJQ = FJD;
                        FJR = FJE;
                    }
                    let FJP = FIW * FIW;
                    let FJS = FJQ - FJR;
                    let FJT = 4.7e-1f64 * ((FJQ + FJR) + (((FJS * FJS) + (J * FJP)).sqrt()));
                    let FJU = FFN + (FIV * FJT);
                    let FJV = FIH * (FJT - FIX);
                    let FJW = FJU - FJV;
                    let FJX = F * ((FJU + FJV) + (((FJW * FJW) + (CAH * FJP)).sqrt()));
                    FKA = FJX;
                    FKF = FJT;
                    GJA = FIV;
                } else {
                    let FJY = BZZ * (D + FIX);
                    let FJZ = (F * FFN) + (FIH * (FJY - (F * FIX)));
                    FKA = FJZ;
                    FKF = FJY;
                    GJA = FIH;
                }
                let FKB = FKA - F;
                let FKC = if FKB < KE { 1.0 } else { 0.0 };
                let FKE = if FKC != 0.0 {
                    let FKD = (D + (FKB.exp())).ln();
                    FKD
                } else {
                    FKB
                };
                let FKG = (FKF + ((FFN / (FKE + F)).ln())) - ASP;
                let FKH = if FKG < KE { 1.0 } else { 0.0 };
                let FKJ = if FKH != 0.0 {
                    let FKI = (D + (FKG.exp())).ln();
                    FKI
                } else {
                    FKG
                };
                let FKK = AXX - (FKJ + ASP);
                let FKL = if FKK < KE { 1.0 } else { 0.0 };
                let FKN = if FKL != 0.0 {
                    let FKM = (D + (FKK.exp())).ln();
                    FKM
                } else {
                    FKK
                };
                let FKO = AQI / (AXX - FKN);
                let FKP = FKO * FKO;
                let FKQ = FKP * FKP;
                let FKR = FKQ * FKQ;
                let FKS = AQI * ((-6.25e-2f64 * ((((CBD * ((D + (AHT * FKQ)).ln())).exp()) + (FKR * FKR)).ln())).exp());
                let FKT = (EKR + FKS) + ATY;
                let FKU = (EKT + FKS) + ATY;
                let FKV = (FKT - EKL) * KJ;
                let FKW = if FKV < KE { 1.0 } else { 0.0 };
                let FKY = if FKW != 0.0 {
                    let FKX = (D + (FKV.exp())).ln();
                    FKX
                } else {
                    FKV
                };
                let FKZ = FKT - (ATY * FKY);
                let FLA = (FKU - EKN) * KJ;
                let FLB = if FLA < KE { 1.0 } else { 0.0 };
                let FLD = if FLB != 0.0 {
                    let FLC = (D + (FLA.exp())).ln();
                    FLC
                } else {
                    FLA
                };
                let FLE = (ELF + FKZ) * EKP;
                let FLF = (FKT - ((ELE + (FKU - (ATY * FLD))) * EKO)) * KJ;
                let FLG = if FLF < KE { 1.0 } else { 0.0 };
                let FLI = if FLG != 0.0 {
                    let FLH = (D + (FLF.exp())).ln();
                    FLH
                } else {
                    FLF
                };
                let FLJ = FKT - (ATY * FLI);
                let FLK = (FKU - FLE) * KJ;
                let FLL = if FLK < KE { 1.0 } else { 0.0 };
                let FLN = if FLL != 0.0 {
                    let FLM = (D + (FLK.exp())).ln();
                    FLM
                } else {
                    FLK
                };
                let FLO = EJQ - FLJ;
                let FLP = EJT - (FKU - (ATY * FLN));
                let FLQ = EJU * FLO;
                let FLR = (EJQ - FLO) - FKS;
                let FLS = if FLR < KE { 1.0 } else { 0.0 };
                let FLW = if FLS != 0.0 {
                    let FLT = FLR.exp();
                    FLT
                } else {
                    let FLU = FLR - KE;
                    let FLV = TP * (D + (FLU * (D + ((F * FLU) * (D + (FLU * KJ))))));
                    FLV
                };
                let FLX = EKA * FLW;
                let FLY = (FLQ * FLQ) - FLX;
                let FLZ = (EMB * FLQ) + FLX;
                let FMA = EMD - FLX;
                let FMB = if FLY < -5e-3f64 { 1.0 } else { 0.0 };
                let FNE;
                let FNI;
                let FNR;
                let FNV;
                let FNY;
                let FOF;
                let FOI;
                if FMB != 0.0 {
                    let FMC = (FLY.abs()).sqrt();
                    let FMD = FMC / ((F * FMC).tan());
                    let FME = (ALG * FLZ) / FLY;
                    let FMF = (FLY + (FMD * (HC - FMD))) * FME;
                    let FMG = ((FLZ - ((HC * FMF) * (D + FMD))) * FME) + ((FMF * FMA) / FLZ);
                    let FMH = D - (F * FMD);
                    let FMI = (FLZ / FLY) * FMH;
                    let FMJ = ((FMA * FMH) - (FLZ * (FMI + (F * FMF)))) / FLY;
                    FNE = A;
                    FNI = FMC;
                    FNR = FMD;
                    FNV = FMF;
                    FNY = FMG;
                    FOF = FMI;
                    FOI = FMJ;
                } else {
                    let FMK = if FLY > BAS { 1.0 } else { 0.0 };
                    let FNF;
                    let FNJ;
                    let FNS;
                    let FNW;
                    let FNZ;
                    let FOG;
                    let FOJ;
                    if FMK != 0.0 {
                        let FML = (FLY.abs()).sqrt();
                        let FMM = (-FML).exp();
                        let FMN = (FML * (D + FMM)) / (D - FMM);
                        let FMO = (ALG * FLZ) / FLY;
                        let FMP = (FLY + (FMN * (HC - FMN))) * FMO;
                        let FMQ = ((FLZ - ((HC * FMP) * (D + FMN))) * FMO) + ((FMP * FMA) / FLZ);
                        let FMR = D - (F * FMN);
                        let FMS = (FLZ / FLY) * FMR;
                        let FMT = ((FMA * FMR) - (FLZ * (FMS + (F * FMP)))) / FLY;
                        FNF = FMM;
                        FNJ = FML;
                        FNS = FMN;
                        FNW = FMP;
                        FNZ = FMQ;
                        FOG = FMS;
                        FOJ = FMT;
                    } else {
                        let FMU = FLY * BBN;
                        let FMV = ASH * (D - ((FLY * BBM) * (D - (FMU * (D - (FLY * BBP))))));
                        let FMW = HC + (FLY * FMV);
                        let FMX = FLY * BBS;
                        let FMY = ASH * (D - (FMX * (D - ((FLY * BBU) * (D - FMX)))));
                        let FMZ = FLZ * FMY;
                        let FNA = (FMA * FMY) - ((FLZ * FLZ) * (BBX * (D - ((FLY * BBY) * (D - ((AAI * FLY) * (D - (BBZ * FLY))))))));
                        let FNB = (-5e-1f64 * FLZ) * FMV;
                        let FNC = ((-5e-1f64 * FMA) * FMV) + (((1.3888888889e-3f64 * FLZ) * FLZ) * (D - (FMU * (HC - (BCC * FLY)))));
                        FNF = A;
                        FNJ = A;
                        FNS = FMW;
                        FNW = FMZ;
                        FNZ = FNA;
                        FOG = FNB;
                        FOJ = FNC;
                    }
                    FNE = FNF;
                    FNI = FNJ;
                    FNR = FNS;
                    FNV = FNW;
                    FNY = FNZ;
                    FOF = FOG;
                    FOI = FOJ;
                }
                let FND = if FLY > BAS { 1.0 } else { 0.0 };
                let FOC;
                let FOY;
                if FND != 0.0 {
                    let FNG = (AJF * FLY) / (D - (FNE * (HC - FNE)));
                    let FNH = FNG * FNE;
                    let FNK = (FNG.ln()) - FNI;
                    FOC = FNH;
                    FOY = FNK;
                } else {
                    let FNL = if FLY < -5e-3f64 { 1.0 } else { 0.0 };
                    let FOD;
                    let FOZ;
                    if FNL != 0.0 {
                        let FNM = (F * FNI).sin();
                        let FNN = (-FLY) / (FNM * FNM);
                        let FNO = FNN.ln();
                        FOD = FNN;
                        FOZ = FNO;
                    } else {
                        let FNP = AJF - ((FLY * KJ) * (D - ((AAI * FLY) * (D - (BCQ * FLY)))));
                        let FNQ = FNP.ln();
                        FOD = FNP;
                        FOZ = FNQ;
                    }
                    FOC = FOD;
                    FOY = FOZ;
                }
                let FNT = if ((BCT * FLQ) + FNR) > A { 1.0 } else { 0.0 };
                let FOL;
                let FOP;
                let FOR;
                if FNT != 0.0 {
                    let FNU = FLQ + FNR;
                    let FNX = EJU + FNV;
                    FOL = FNU;
                    FOP = FNX;
                    FOR = FNY;
                } else {
                    let FOA = D / (FLQ - FNR);
                    let FOB = FNV - EJU;
                    let FOE = (FLX - FOC) * FOA;
                    let FOH = (((FOB * FOE) - FLX) - (FOF * FOC)) * FOA;
                    let FOK = ((((FNY * FOE) + ((HC * FOB) * FOH)) + FLX) - ((FOI + (FOF * FOF)) * FOC)) * FOA;
                    FOL = FOE;
                    FOP = FOH;
                    FOR = FOK;
                }
                let FOM = if FOL > A { 1.0 } else { 0.0 };
                let FOX;
                let FPA;
                let FPB;
                if FOM != 0.0 {
                    let FON = FOL.ln();
                    let FOO = D / FOL;
                    let FOQ = FOP * FOO;
                    let FOS = (FOR * FOO) - (FOQ * FOQ);
                    FOX = FON;
                    FPA = FOQ;
                    FPB = FOS;
                } else {
                    let FOT = (FLQ + ACD) + ((-FLQ).ln());
                    let FOU = D / FLO;
                    let FOV = EJU + FOU;
                    let FOW = (-FOU) * FOU;
                    FOX = FOT;
                    FPA = FOV;
                    FPB = FOW;
                }
                let FPC = FLQ + (EJV * (((EPB + FLO) + (HC * FOX)) - FOY));
                let FPD = EJU + (EJV * ((D + (HC * FPA)) - FOF));
                let FPE = (FPC * FOL) - FLX;
                let FPF = ((FPD * FOL) + (FPC * FOP)) + FLX;
                let FPG = (FPF * FPF) - ((F * FPE) * (((((EJV * ((HC * FPB) - FOI)) * FOL) + ((HC * FPD) * FOP)) + (FPC * FOR)) - FLX));
                let FPH = FLO + ((((-FPE) * FPF) * FPG) / ((FPG * FPG) + BEL));
                let FPI = EJU * FPH;
                let FPJ = EJV * FLP;
                let FPK = FPI + FPJ;
                let FPL = D + (BEQ * FPK);
                let FPM = FPI * FPJ;
                let FPN = (BES + (BET * FPK)) + FPM;
                let FPO = (FPI * FPI) - (((((FPN * FPN) - ((AJF * FPL) * (BES * ((HC * FPK) + FPM)))).sqrt()) - FPN) / (HC * FPL));
                let FPP = if FPO > A { 1.0 } else { 0.0 };
                let FPV;
                if FPP != 0.0 {
                    let FPQ = FPO * (((((FPO / EKA).ln()) + FKS) - EJQ) + FPH);
                    let FPR = (EMB * FPI) + FPO;
                    let FPS = (EJQ - FPH) - FKT;
                    let FPT = if (if (if (if FPQ < A { 1.0 } else { 0.0 }) != 0.0 && (if FPR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((FPS + BFB) + (EJU.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if FPS > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let FPW = if FPT != 0.0 {
                        let FPU = FPH - (FPQ / FPR);
                        FPU
                    } else {
                        FPH
                    };
                    FPV = FPW;
                } else {
                    FPV = FPH;
                }
                let FPX = EJU * FPV;
                let FPY = FPX + FPJ;
                let FPZ = D + (BEQ * FPY);
                let FQA = FPX * FPJ;
                let FQB = (BES + (BET * FPY)) + FQA;
                let FQC = ((((FQB * FQB) - ((AJF * FPZ) * (BES * ((HC * FPY) + FQA)))).sqrt()) - FQB) / (HC * FPZ);
                let FQD = if FQC < -5e-3f64 { 1.0 } else { 0.0 };
                let FQP;
                let FQR;
                let FSR;
                let FSY;
                if FQD != 0.0 {
                    let FQE = (FQC.abs()).sqrt();
                    let FQF = FQE / ((F * FQE).tan());
                    let FQG = (ALG * (FQC + (FQF * (HC - FQF)))) / FQC;
                    FQP = FQF;
                    FQR = FQG;
                    FSR = FNE;
                    FSY = FQE;
                } else {
                    let FQH = if FQC > BAS { 1.0 } else { 0.0 };
                    let FQQ;
                    let FQS;
                    let FSS;
                    let FSZ;
                    if FQH != 0.0 {
                        let FQI = (FQC.abs()).sqrt();
                        let FQJ = (-FQI).exp();
                        let FQK = (FQI * (D + FQJ)) / (D - FQJ);
                        let FQL = (ALG * (FQC + (FQK * (HC - FQK)))) / FQC;
                        FQQ = FQK;
                        FQS = FQL;
                        FSS = FQJ;
                        FSZ = FQI;
                    } else {
                        let FQM = HC + ((FQC * ASH) * (D - ((FQC * BBM) * (D - (FQC * BBN)))));
                        let FQN = FQC * BBS;
                        let FQO = ASH * (D - (FQN * (D - ((FQC * BBU) * (D - FQN)))));
                        FQQ = FQM;
                        FQS = FQO;
                        FSS = FNE;
                        FSZ = FNI;
                    }
                    FQP = FQQ;
                    FQR = FQS;
                    FSR = FSS;
                    FSY = FSZ;
                }
                let FQT = (FPX * FPX) - (FQC - ((((FPY * FQP) + FQA) + FQC) / ((FPY * FQR) + D)));
                let FQU = if FQT > A { 1.0 } else { 0.0 };
                let FRA;
                if FQU != 0.0 {
                    let FQV = FQT * (((((FQT / EKA).ln()) + FKS) - EJQ) + FPV);
                    let FQW = (EMB * FPX) + FQT;
                    let FQX = (EJQ - FPV) - FKT;
                    let FQY = if (if (if (if FQV < A { 1.0 } else { 0.0 }) != 0.0 && (if FQW > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if ((FQX + BFB) + (EJU.ln())) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if FQX > D { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let FRB = if FQY != 0.0 {
                        let FQZ = FPV - (FQV / FQW);
                        FQZ
                    } else {
                        FPV
                    };
                    FRA = FRB;
                } else {
                    FRA = FPV;
                }
                let FRC = EJU * FRA;
                let FRD = (EJQ - FRA) - FKS;
                let FRE = if FRD < KE { 1.0 } else { 0.0 };
                let FRI = if FRE != 0.0 {
                    let FRF = FRD.exp();
                    FRF
                } else {
                    let FRG = FRD - KE;
                    let FRH = TP * (D + (FRG * (D + ((F * FRG) * (D + (FRG * KJ))))));
                    FRH
                };
                let FRJ = EKA * FRI;
                let FRK = (FRC * FRC) - FRJ;
                let FRL = (EMB * FRC) + FRJ;
                let FRM = EMD - FRJ;
                let FRN = if FRK < -5e-3f64 { 1.0 } else { 0.0 };
                let FSQ;
                let FSW;
                let FTH;
                let FTL;
                let FTO;
                let FTV;
                let FTY;
                if FRN != 0.0 {
                    let FRO = (FRK.abs()).sqrt();
                    let FRP = FRO / ((F * FRO).tan());
                    let FRQ = (ALG * FRL) / FRK;
                    let FRR = (FRK + (FRP * (HC - FRP))) * FRQ;
                    let FRS = ((FRL - ((HC * FRR) * (D + FRP))) * FRQ) + ((FRR * FRM) / FRL);
                    let FRT = D - (F * FRP);
                    let FRU = (FRL / FRK) * FRT;
                    let FRV = ((FRM * FRT) - (FRL * (FRU + (F * FRR)))) / FRK;
                    FSQ = FSR;
                    FSW = FRO;
                    FTH = FRP;
                    FTL = FRR;
                    FTO = FRS;
                    FTV = FRU;
                    FTY = FRV;
                } else {
                    let FRW = if FRK > BAS { 1.0 } else { 0.0 };
                    let FST;
                    let FSX;
                    let FTI;
                    let FTM;
                    let FTP;
                    let FTW;
                    let FTZ;
                    if FRW != 0.0 {
                        let FRX = (FRK.abs()).sqrt();
                        let FRY = (-FRX).exp();
                        let FRZ = (FRX * (D + FRY)) / (D - FRY);
                        let FSA = (ALG * FRL) / FRK;
                        let FSB = (FRK + (FRZ * (HC - FRZ))) * FSA;
                        let FSC = ((FRL - ((HC * FSB) * (D + FRZ))) * FSA) + ((FSB * FRM) / FRL);
                        let FSD = D - (F * FRZ);
                        let FSE = (FRL / FRK) * FSD;
                        let FSF = ((FRM * FSD) - (FRL * (FSE + (F * FSB)))) / FRK;
                        FST = FRY;
                        FSX = FRX;
                        FTI = FRZ;
                        FTM = FSB;
                        FTP = FSC;
                        FTW = FSE;
                        FTZ = FSF;
                    } else {
                        let FSG = FRK * BBN;
                        let FSH = ASH * (D - ((FRK * BBM) * (D - (FSG * (D - (FRK * BBP))))));
                        let FSI = HC + (FRK * FSH);
                        let FSJ = FRK * BBS;
                        let FSK = ASH * (D - (FSJ * (D - ((FRK * BBU) * (D - FSJ)))));
                        let FSL = FRL * FSK;
                        let FSM = (FRM * FSK) - ((FRL * FRL) * (BBX * (D - ((FRK * BBY) * (D - ((AAI * FRK) * (D - (BBZ * FRK))))))));
                        let FSN = (-5e-1f64 * FRL) * FSH;
                        let FSO = ((-5e-1f64 * FRM) * FSH) + (((1.3888888889e-3f64 * FRL) * FRL) * (D - (FSG * (HC - (BCC * FRK)))));
                        FST = FSR;
                        FSX = FSY;
                        FTI = FSI;
                        FTM = FSL;
                        FTP = FSM;
                        FTW = FSN;
                        FTZ = FSO;
                    }
                    FSQ = FST;
                    FSW = FSX;
                    FTH = FTI;
                    FTL = FTM;
                    FTO = FTP;
                    FTV = FTW;
                    FTY = FTZ;
                }
                let FSP = if FRK > BAS { 1.0 } else { 0.0 };
                let FTS;
                let FUO;
                if FSP != 0.0 {
                    let FSU = (AJF * FRK) / (D - (FSQ * (HC - FSQ)));
                    let FSV = FSU * FSQ;
                    let FTA = (FSU.ln()) - FSW;
                    FTS = FSV;
                    FUO = FTA;
                } else {
                    let FTB = if FRK < -5e-3f64 { 1.0 } else { 0.0 };
                    let FTT;
                    let FUP;
                    if FTB != 0.0 {
                        let FTC = (F * FSW).sin();
                        let FTD = (-FRK) / (FTC * FTC);
                        let FTE = FTD.ln();
                        FTT = FTD;
                        FUP = FTE;
                    } else {
                        let FTF = AJF - ((FRK * KJ) * (D - ((AAI * FRK) * (D - (BCQ * FRK)))));
                        let FTG = FTF.ln();
                        FTT = FTF;
                        FUP = FTG;
                    }
                    FTS = FTT;
                    FUO = FUP;
                }
                let FTJ = if ((BCT * FRC) + FTH) > A { 1.0 } else { 0.0 };
                let FUB;
                let FUF;
                let FUH;
                if FTJ != 0.0 {
                    let FTK = FRC + FTH;
                    let FTN = EJU + FTL;
                    FUB = FTK;
                    FUF = FTN;
                    FUH = FTO;
                } else {
                    let FTQ = D / (FRC - FTH);
                    let FTR = FTL - EJU;
                    let FTU = (FRJ - FTS) * FTQ;
                    let FTX = (((FTR * FTU) - FRJ) - (FTV * FTS)) * FTQ;
                    let FUA = ((((FTO * FTU) + ((HC * FTR) * FTX)) + FRJ) - ((FTY + (FTV * FTV)) * FTS)) * FTQ;
                    FUB = FTU;
                    FUF = FTX;
                    FUH = FUA;
                }
                let FUC = if FUB > A { 1.0 } else { 0.0 };
                let FUN;
                let FUQ;
                let FUR;
                if FUC != 0.0 {
                    let FUD = FUB.ln();
                    let FUE = D / FUB;
                    let FUG = FUF * FUE;
                    let FUI = (FUH * FUE) - (FUG * FUG);
                    FUN = FUD;
                    FUQ = FUG;
                    FUR = FUI;
                } else {
                    let FUJ = (FRC + ACD) + ((-FRC).ln());
                    let FUK = D / FRA;
                    let FUL = EJU + FUK;
                    let FUM = (-FUK) * FUK;
                    FUN = FUJ;
                    FUQ = FUL;
                    FUR = FUM;
                }
                let FUS = FRC + (EJV * (((EPB + FRA) + (HC * FUN)) - FUO));
                let FUT = EJU + (EJV * ((D + (HC * FUQ)) - FTV));
                let FUU = (FUS * FUB) - FRJ;
                let FUV = ((FUT * FUB) + (FUS * FUF)) + FRJ;
                let FUW = (FUV * FUV) - ((F * FUU) * (((((EJV * ((HC * FUR) - FTY)) * FUB) + ((HC * FUT) * FUF)) + (FUS * FUH)) - FRJ));
                let FUX = FRA + ((((-FUU) * FUV) * FUW) / ((FUW * FUW) + BEL));
                let FUY = EJU * FUX;
                let FUZ = (EJQ - FUX) - FKS;
                let FVA = if FUZ < KE { 1.0 } else { 0.0 };
                let FVE = if FVA != 0.0 {
                    let FVB = FUZ.exp();
                    FVB
                } else {
                    let FVC = FUZ - KE;
                    let FVD = TP * (D + (FVC * (D + ((F * FVC) * (D + (FVC * KJ))))));
                    FVD
                };
                let FVF = EKA * FVE;
                let FVG = (FUY * FUY) - FVF;
                let FVH = (EMB * FUY) + FVF;
                let FVI = EMD - FVF;
                let FVJ = if FVG < -5e-3f64 { 1.0 } else { 0.0 };
                let FWM;
                let FWQ;
                let FWZ;
                let FXD;
                let FXG;
                let FXN;
                let FXQ;
                if FVJ != 0.0 {
                    let FVK = (FVG.abs()).sqrt();
                    let FVL = FVK / ((F * FVK).tan());
                    let FVM = (ALG * FVH) / FVG;
                    let FVN = (FVG + (FVL * (HC - FVL))) * FVM;
                    let FVO = ((FVH - ((HC * FVN) * (D + FVL))) * FVM) + ((FVN * FVI) / FVH);
                    let FVP = D - (F * FVL);
                    let FVQ = (FVH / FVG) * FVP;
                    let FVR = ((FVI * FVP) - (FVH * (FVQ + (F * FVN)))) / FVG;
                    FWM = FSQ;
                    FWQ = FVK;
                    FWZ = FVL;
                    FXD = FVN;
                    FXG = FVO;
                    FXN = FVQ;
                    FXQ = FVR;
                } else {
                    let FVS = if FVG > BAS { 1.0 } else { 0.0 };
                    let FWN;
                    let FWR;
                    let FXA;
                    let FXE;
                    let FXH;
                    let FXO;
                    let FXR;
                    if FVS != 0.0 {
                        let FVT = (FVG.abs()).sqrt();
                        let FVU = (-FVT).exp();
                        let FVV = (FVT * (D + FVU)) / (D - FVU);
                        let FVW = (ALG * FVH) / FVG;
                        let FVX = (FVG + (FVV * (HC - FVV))) * FVW;
                        let FVY = ((FVH - ((HC * FVX) * (D + FVV))) * FVW) + ((FVX * FVI) / FVH);
                        let FVZ = D - (F * FVV);
                        let FWA = (FVH / FVG) * FVZ;
                        let FWB = ((FVI * FVZ) - (FVH * (FWA + (F * FVX)))) / FVG;
                        FWN = FVU;
                        FWR = FVT;
                        FXA = FVV;
                        FXE = FVX;
                        FXH = FVY;
                        FXO = FWA;
                        FXR = FWB;
                    } else {
                        let FWC = FVG * BBN;
                        let FWD = ASH * (D - ((FVG * BBM) * (D - (FWC * (D - (FVG * BBP))))));
                        let FWE = HC + (FVG * FWD);
                        let FWF = FVG * BBS;
                        let FWG = ASH * (D - (FWF * (D - ((FVG * BBU) * (D - FWF)))));
                        let FWH = FVH * FWG;
                        let FWI = (FVI * FWG) - ((FVH * FVH) * (BBX * (D - ((FVG * BBY) * (D - ((AAI * FVG) * (D - (BBZ * FVG))))))));
                        let FWJ = (-5e-1f64 * FVH) * FWD;
                        let FWK = ((-5e-1f64 * FVI) * FWD) + (((1.3888888889e-3f64 * FVH) * FVH) * (D - (FWC * (HC - (BCC * FVG)))));
                        FWN = FSQ;
                        FWR = FSW;
                        FXA = FWE;
                        FXE = FWH;
                        FXH = FWI;
                        FXO = FWJ;
                        FXR = FWK;
                    }
                    FWM = FWN;
                    FWQ = FWR;
                    FWZ = FXA;
                    FXD = FXE;
                    FXG = FXH;
                    FXN = FXO;
                    FXQ = FXR;
                }
                let FWL = if FVG > BAS { 1.0 } else { 0.0 };
                let FXK;
                let FYG;
                if FWL != 0.0 {
                    let FWO = (AJF * FVG) / (D - (FWM * (HC - FWM)));
                    let FWP = FWO * FWM;
                    let FWS = (FWO.ln()) - FWQ;
                    FXK = FWP;
                    FYG = FWS;
                } else {
                    let FWT = if FVG < -5e-3f64 { 1.0 } else { 0.0 };
                    let FXL;
                    let FYH;
                    if FWT != 0.0 {
                        let FWU = (F * FWQ).sin();
                        let FWV = (-FVG) / (FWU * FWU);
                        let FWW = FWV.ln();
                        FXL = FWV;
                        FYH = FWW;
                    } else {
                        let FWX = AJF - ((FVG * KJ) * (D - ((AAI * FVG) * (D - (BCQ * FVG)))));
                        let FWY = FWX.ln();
                        FXL = FWX;
                        FYH = FWY;
                    }
                    FXK = FXL;
                    FYG = FYH;
                }
                let FXB = if ((BCT * FUY) + FWZ) > A { 1.0 } else { 0.0 };
                let FXT;
                let FXX;
                let FXZ;
                if FXB != 0.0 {
                    let FXC = FUY + FWZ;
                    let FXF = EJU + FXD;
                    FXT = FXC;
                    FXX = FXF;
                    FXZ = FXG;
                } else {
                    let FXI = D / (FUY - FWZ);
                    let FXJ = FXD - EJU;
                    let FXM = (FVF - FXK) * FXI;
                    let FXP = (((FXJ * FXM) - FVF) - (FXN * FXK)) * FXI;
                    let FXS = ((((FXG * FXM) + ((HC * FXJ) * FXP)) + FVF) - ((FXQ + (FXN * FXN)) * FXK)) * FXI;
                    FXT = FXM;
                    FXX = FXP;
                    FXZ = FXS;
                }
                let FXU = if FXT > A { 1.0 } else { 0.0 };
                let FYF;
                let FYI;
                let FYJ;
                if FXU != 0.0 {
                    let FXV = FXT.ln();
                    let FXW = D / FXT;
                    let FXY = FXX * FXW;
                    let FYA = (FXZ * FXW) - (FXY * FXY);
                    FYF = FXV;
                    FYI = FXY;
                    FYJ = FYA;
                } else {
                    let FYB = (FUY + ACD) + ((-FUY).ln());
                    let FYC = D / FUX;
                    let FYD = EJU + FYC;
                    let FYE = (-FYC) * FYC;
                    FYF = FYB;
                    FYI = FYD;
                    FYJ = FYE;
                }
                let FYK = FUY + (EJV * (((EPB + FUX) + (HC * FYF)) - FYG));
                let FYL = EJU + (EJV * ((D + (HC * FYI)) - FXN));
                let FYM = (FYK * FXT) - FVF;
                let FYN = ((FYL * FXT) + (FYK * FXX)) + FVF;
                let FYO = (FYN * FYN) - ((F * FYM) * (((((EJV * ((HC * FYJ) - FXQ)) * FXT) + ((HC * FYL) * FXX)) + (FYK * FXZ)) - FVF));
                let FYP = (((-FYM) * FYN) * FYO) / ((FYO * FYO) + BEL);
                let FYQ = FUX + FYP;
                let GCK;
                let GDQ;
                let GDV;
                if E != 0.0 {
                    let FYR = if (FYP.abs()) > O { 1.0 } else { 0.0 };
                    let GCL;
                    let GDR;
                    let GDW;
                    if FYR != 0.0 {
                        let FYS = EJU * FYQ;
                        let FYT = (EJQ - FYQ) - FKS;
                        let FYU = if FYT < KE { 1.0 } else { 0.0 };
                        let FYY = if FYU != 0.0 {
                            let FYV = FYT.exp();
                            FYV
                        } else {
                            let FYW = FYT - KE;
                            let FYX = TP * (D + (FYW * (D + ((F * FYW) * (D + (FYW * KJ))))));
                            FYX
                        };
                        let FYZ = EKA * FYY;
                        let FZA = (FYS * FYS) - FYZ;
                        let FZB = (EMB * FYS) + FYZ;
                        let FZC = EMD - FYZ;
                        let FZD = if FZA < -5e-3f64 { 1.0 } else { 0.0 };
                        let GAG;
                        let GAK;
                        let GAT;
                        let GAX;
                        let GBA;
                        let GBH;
                        let GBK;
                        if FZD != 0.0 {
                            let FZE = (FZA.abs()).sqrt();
                            let FZF = FZE / ((F * FZE).tan());
                            let FZG = (ALG * FZB) / FZA;
                            let FZH = (FZA + (FZF * (HC - FZF))) * FZG;
                            let FZI = ((FZB - ((HC * FZH) * (D + FZF))) * FZG) + ((FZH * FZC) / FZB);
                            let FZJ = D - (F * FZF);
                            let FZK = (FZB / FZA) * FZJ;
                            let FZL = ((FZC * FZJ) - (FZB * (FZK + (F * FZH)))) / FZA;
                            GAG = FWM;
                            GAK = FZE;
                            GAT = FZF;
                            GAX = FZH;
                            GBA = FZI;
                            GBH = FZK;
                            GBK = FZL;
                        } else {
                            let FZM = if FZA > BAS { 1.0 } else { 0.0 };
                            let GAH;
                            let GAL;
                            let GAU;
                            let GAY;
                            let GBB;
                            let GBI;
                            let GBL;
                            if FZM != 0.0 {
                                let FZN = (FZA.abs()).sqrt();
                                let FZO = (-FZN).exp();
                                let FZP = (FZN * (D + FZO)) / (D - FZO);
                                let FZQ = (ALG * FZB) / FZA;
                                let FZR = (FZA + (FZP * (HC - FZP))) * FZQ;
                                let FZS = ((FZB - ((HC * FZR) * (D + FZP))) * FZQ) + ((FZR * FZC) / FZB);
                                let FZT = D - (F * FZP);
                                let FZU = (FZB / FZA) * FZT;
                                let FZV = ((FZC * FZT) - (FZB * (FZU + (F * FZR)))) / FZA;
                                GAH = FZO;
                                GAL = FZN;
                                GAU = FZP;
                                GAY = FZR;
                                GBB = FZS;
                                GBI = FZU;
                                GBL = FZV;
                            } else {
                                let FZW = FZA * BBN;
                                let FZX = ASH * (D - ((FZA * BBM) * (D - (FZW * (D - (FZA * BBP))))));
                                let FZY = HC + (FZA * FZX);
                                let FZZ = FZA * BBS;
                                let GAA = ASH * (D - (FZZ * (D - ((FZA * BBU) * (D - FZZ)))));
                                let GAB = FZB * GAA;
                                let GAC = (FZC * GAA) - ((FZB * FZB) * (BBX * (D - ((FZA * BBY) * (D - ((AAI * FZA) * (D - (BBZ * FZA))))))));
                                let GAD = (-5e-1f64 * FZB) * FZX;
                                let GAE = ((-5e-1f64 * FZC) * FZX) + (((1.3888888889e-3f64 * FZB) * FZB) * (D - (FZW * (HC - (BCC * FZA)))));
                                GAH = FWM;
                                GAL = FWQ;
                                GAU = FZY;
                                GAY = GAB;
                                GBB = GAC;
                                GBI = GAD;
                                GBL = GAE;
                            }
                            GAG = GAH;
                            GAK = GAL;
                            GAT = GAU;
                            GAX = GAY;
                            GBA = GBB;
                            GBH = GBI;
                            GBK = GBL;
                        }
                        let GAF = if FZA > BAS { 1.0 } else { 0.0 };
                        let GBE;
                        let GCA;
                        if GAF != 0.0 {
                            let GAI = (AJF * FZA) / (D - (GAG * (HC - GAG)));
                            let GAJ = GAI * GAG;
                            let GAM = (GAI.ln()) - GAK;
                            GBE = GAJ;
                            GCA = GAM;
                        } else {
                            let GAN = if FZA < -5e-3f64 { 1.0 } else { 0.0 };
                            let GBF;
                            let GCB;
                            if GAN != 0.0 {
                                let GAO = (F * GAK).sin();
                                let GAP = (-FZA) / (GAO * GAO);
                                let GAQ = GAP.ln();
                                GBF = GAP;
                                GCB = GAQ;
                            } else {
                                let GAR = AJF - ((FZA * KJ) * (D - ((AAI * FZA) * (D - (BCQ * FZA)))));
                                let GAS = GAR.ln();
                                GBF = GAR;
                                GCB = GAS;
                            }
                            GBE = GBF;
                            GCA = GCB;
                        }
                        let GAV = if ((BCT * FYS) + GAT) > A { 1.0 } else { 0.0 };
                        let GBN;
                        let GBR;
                        let GBT;
                        if GAV != 0.0 {
                            let GAW = FYS + GAT;
                            let GAZ = EJU + GAX;
                            GBN = GAW;
                            GBR = GAZ;
                            GBT = GBA;
                        } else {
                            let GBC = D / (FYS - GAT);
                            let GBD = GAX - EJU;
                            let GBG = (FYZ - GBE) * GBC;
                            let GBJ = (((GBD * GBG) - FYZ) - (GBH * GBE)) * GBC;
                            let GBM = ((((GBA * GBG) + ((HC * GBD) * GBJ)) + FYZ) - ((GBK + (GBH * GBH)) * GBE)) * GBC;
                            GBN = GBG;
                            GBR = GBJ;
                            GBT = GBM;
                        }
                        let GBO = if GBN > A { 1.0 } else { 0.0 };
                        let GBZ;
                        let GCC;
                        let GCD;
                        if GBO != 0.0 {
                            let GBP = GBN.ln();
                            let GBQ = D / GBN;
                            let GBS = GBR * GBQ;
                            let GBU = (GBT * GBQ) - (GBS * GBS);
                            GBZ = GBP;
                            GCC = GBS;
                            GCD = GBU;
                        } else {
                            let GBV = (FYS + ACD) + ((-FYS).ln());
                            let GBW = D / FYQ;
                            let GBX = EJU + GBW;
                            let GBY = (-GBW) * GBW;
                            GBZ = GBV;
                            GCC = GBX;
                            GCD = GBY;
                        }
                        let GCE = FYS + (EJV * (((EPB + FYQ) + (HC * GBZ)) - GCA));
                        let GCF = EJU + (EJV * ((D + (HC * GCC)) - GBH));
                        let GCG = (GCE * GBN) - FYZ;
                        let GCH = ((GCF * GBN) + (GCE * GBR)) + FYZ;
                        let GCI = (GCH * GCH) - ((F * GCG) * (((((EJV * ((HC * GCD) - GBK)) * GBN) + ((HC * GCF) * GBR)) + (GCE * GBT)) - FYZ));
                        let GCJ = FYQ + ((((-GCG) * GCH) * GCI) / ((GCI * GCI) + BEL));
                        GCL = GCJ;
                        GDR = GAG;
                        GDW = GAK;
                    } else {
                        GCL = FYQ;
                        GDR = FWM;
                        GDW = FWQ;
                    }
                    GCK = GCL;
                    GDQ = GDR;
                    GDV = GDW;
                } else {
                    GCK = FYQ;
                    GDQ = FWM;
                    GDV = FWQ;
                }
                let GCM = EJU * GCK;
                let GCN = (EJQ - GCK) - FKS;
                let GCO = if GCN < KE { 1.0 } else { 0.0 };
                let GCS = if GCO != 0.0 {
                    let GCP = GCN.exp();
                    GCP
                } else {
                    let GCQ = GCN - KE;
                    let GCR = TP * (D + (GCQ * (D + ((F * GCQ) * (D + (GCQ * KJ))))));
                    GCR
                };
                let GCT = EKA * GCS;
                let GCU = (GCM * GCM) - GCT;
                let GCV = if GCT <= A { 1.0 } else { 0.0 };
                let GEY;
                let GFL;
                let GFP;
                if GCV != 0.0 {
                    let GCW = BSF - GCM;
                    let GCX = GCW / EJV;
                    GEY = GCX;
                    GFL = GCW;
                    GFP = BSF;
                } else {
                    let GCY = if GCU < -5e-3f64 { 1.0 } else { 0.0 };
                    let GDG;
                    let GDP;
                    let GDT;
                    if GCY != 0.0 {
                        let GCZ = (GCU.abs()).sqrt();
                        let GDA = GCZ / ((F * GCZ).tan());
                        GDG = GDA;
                        GDP = GDQ;
                        GDT = GCZ;
                    } else {
                        let GDB = if GCU > BAS { 1.0 } else { 0.0 };
                        let GDH;
                        let GDS;
                        let GDU;
                        if GDB != 0.0 {
                            let GDC = (GCU.abs()).sqrt();
                            let GDD = (-GDC).exp();
                            let GDE = (GDC * (D + GDD)) / (D - GDD);
                            GDH = GDE;
                            GDS = GDD;
                            GDU = GDC;
                        } else {
                            let GDF = HC + ((GCU * ASH) * (D - ((GCU * BBM) * (D - (GCU * BBN)))));
                            GDH = GDF;
                            GDS = GDQ;
                            GDU = GDV;
                        }
                        GDG = GDH;
                        GDP = GDS;
                        GDT = GDU;
                    }
                    let GDI = if ((BCT * GCM) + GDG) > A { 1.0 } else { 0.0 };
                    let GEZ;
                    let GFM;
                    let GFQ;
                    if GDI != 0.0 {
                        let GDJ = GCM + GDG;
                        let GDK = if (GCT * GCM) < (((BSU * GCM) * GCM) * GDJ) { 1.0 } else { 0.0 };
                        let GFA;
                        let GFN;
                        let GFR;
                        if GDK != 0.0 {
                            let GDL = (GCT / GDJ) + BSF;
                            let GDM = GDL - GCM;
                            let GDN = GDM / EJV;
                            GFA = GDN;
                            GFN = GDM;
                            GFR = GDL;
                        } else {
                            let GDO = if GCU > BAS { 1.0 } else { 0.0 };
                            let GEC;
                            if GDO != 0.0 {
                                let GDX = (((AJF * GCU) / (D - (GDP * (HC - GDP)))).ln()) - GDT;
                                GEC = GDX;
                            } else {
                                let GDY = if GCU < -5e-3f64 { 1.0 } else { 0.0 };
                                let GED = if GDY != 0.0 {
                                    let GDZ = (F * GDT).sin();
                                    let GEA = ((-GCU) / (GDZ * GDZ)).ln();
                                    GEA
                                } else {
                                    let GEB = (AJF - ((GCU * KJ) * (D - ((AAI * GCU) * (D - (BCQ * GCU)))))).ln();
                                    GEB
                                };
                                GEC = GED;
                            }
                            let GEE = ((EPB + GCK) + (HC * (GDJ.ln()))) - GEC;
                            let GEF = EJV * GEE;
                            let GEG = GCM + GEF;
                            GFA = GEE;
                            GFN = GEF;
                            GFR = GEG;
                        }
                        GEZ = GFA;
                        GFM = GFN;
                        GFQ = GFR;
                    } else {
                        let GEH = if GCU > BAS { 1.0 } else { 0.0 };
                        let GET;
                        if GEH != 0.0 {
                            let GEI = ((GCK + FKS) - EJQ) - GDT;
                            let GEJ = if GEI < KE { 1.0 } else { 0.0 };
                            let GEN = if GEJ != 0.0 {
                                let GEK = GEI.exp();
                                GEK
                            } else {
                                let GEL = GEI - KE;
                                let GEM = TP * (D + (GEL * (D + ((F * GEL) * (D + (GEL * KJ))))));
                                GEM
                            };
                            let GEO = ((AJF * GCU) * (GEN / EKA)) / (D - (GDP * (HC - GDP)));
                            GET = GEO;
                        } else {
                            let GEP = if GCU < -5e-3f64 { 1.0 } else { 0.0 };
                            let GEU = if GEP != 0.0 {
                                let GEQ = (F * GDT).sin();
                                let GER = ((-GCU) / (GEQ * GEQ)) / GCT;
                                GER
                            } else {
                                let GES = (AJF - ((GCU * KJ) * (D - ((AAI * GCU) * (D - (BCQ * GCU)))))) / GCT;
                                GES
                            };
                            GET = GEU;
                        }
                        let GEV = ((GCM - GDG) / (D - GET)) + BSF;
                        let GEW = GEV - GCM;
                        let GEX = GEW / EJV;
                        GEZ = GEX;
                        GFM = GEW;
                        GFQ = GEV;
                    }
                    GEY = GEZ;
                    GFL = GFM;
                    GFP = GFQ;
                }
                let GFB = (EJT - GEY) - FKS;
                let GFC = if GFB < KE { 1.0 } else { 0.0 };
                let GFG = if GFC != 0.0 {
                    let GFD = GFB.exp();
                    GFD
                } else {
                    let GFE = GFB - KE;
                    let GFF = TP * (D + (GFE * (D + ((F * GFE) * (D + (GFE * KJ))))));
                    GFF
                };
                let GFH = EKA * GFG;
                let GIK;
                let GIM;
                let GIR;
                let GIV;
                if FFQ != 0.0 {
                    let GFI = GCT * EJW;
                    let GFJ = GFH * EJX;
                    let GFK = GFI + (HC * GCM);
                    let GFO = GFJ + (HC * GFL);
                    let GFS = ((HC * GFP) + GFI) + GFJ;
                    let GFT = if (GCU.abs()) > BAS { 1.0 } else { 0.0 };
                    let GIS = if GFT != 0.0 {
                        let GFU = ((-4e0f64 * GCU) * GFS) / (GFP * (((GFK * GFO) + ((HC * (GCK + HC)) * GFO)) + ((HC * (GEY + HC)) * GFK)));
                        GFU
                    } else {
                        let GFV = GCU * BBS;
                        let GFW = ((GCT * GFH) * GFS) / (GFP * (((GFK * GCT) + (GFO * GFH)) + (((GFK * GFO) * GFP) * (D + (GFP * (ASH * (D - (GFV * (D - ((GCU * BBU) * (D - GFV)))))))))));
                        GFW
                    };
                    GIK = GFO;
                    GIM = GFK;
                    GIR = GIS;
                    GIV = GFS;
                } else {
                    GIK = A;
                    GIM = A;
                    GIR = A;
                    GIV = A;
                }
                let GFX = FKS + (GFP.ln());
                let GFY = F * (FFN + GFP);
                let GFZ = GFX - FGD;
                let GHA = if AEG != 0.0 {
                    let GGA = (F * (FCR + GCM)) / EJU;
                    let GGB = GGA - ADN;
                    let GGC = F * ((GGA + ADN) + (((GGB * GGB) + D).sqrt()));
                    let GGD = (((GGC / AQH) + ((ALG * AEJ) * AEJ)).sqrt()) - (F * AEJ);
                    let GGE = D - (((GGD * GGD) * AQH) / GGC);
                    GGE
                } else {
                    D
                };
                let GGF = GCM / HC;
                let GGG = if GGF < KE { 1.0 } else { 0.0 };
                let GGI = if GGG != 0.0 {
                    let GGH = (D + (GGF.exp())).ln();
                    GGH
                } else {
                    GGF
                };
                let GGJ = HC * GGI;
                let GGK = GFL / HC;
                let GGL = if GGK < KE { 1.0 } else { 0.0 };
                let GGN = if GGL != 0.0 {
                    let GGM = (D + (GGK.exp())).ln();
                    GGM
                } else {
                    GGK
                };
                let GGO = HC * GGN;
                let GGP = GGO - GFL;
                let GGQ = GGJ - GCM;
                let GGR = F * (FGI + GGJ);
                let GGS = F * (FGN + GGO);
                let GGT = D / (GGR + GGS);
                let GGU = (GFY * GGR) * GGT;
                let GGV = (GFY * GGS) * GGT;
                let GGW = F * (FGO + GGP);
                let GGX = F * (FGP + GGQ);
                let GGY = F * (FGQ + ((AHH * GGJ) + (AHI * GGP)));
                let GGZ = F * (FGR + ((AHH * GGO) + (AHI * GGQ)));
                let GHB = ((GGR * AGB) * BWA) * GHA;
                let GHC = (GGS * AGF) * BWA;
                let GHD = GHB + GHC;
                let GHE = BWD * (GGW + (BWE * GGX));
                let GHF = D + GHE;
                let GHG = D + (BWH * GHE);
                let GHH = (F * (GHF + (((GHF * GHF) + O).sqrt()))) / (F * (GHG + (((GHG * GHG) + O).sqrt())));
                let GHI = (BWK * ((D + (BWL * GGW)) + (BWM * GGX))) * ((BWO * (((D + (GGU * AHA)) + (GGV * AHC)).ln())).exp());
                let GHM;
                if BWR != 0.0 {
                    GHM = D;
                } else {
                    let GHJ = if BWQ < A { 1.0 } else { 0.0 };
                    let GHN = if GHJ != 0.0 {
                        let GHK = D - (BWQ * ((BWT * ((GFY + BWU).ln())).exp()));
                        GHK
                    } else {
                        let GHL = D / (D + (BWQ * ((BWT * ((GFY + BWU).ln())).exp())));
                        GHL
                    };
                    GHM = GHN;
                }
                let GHO = FHE * ((GFY * GHM) + BXD);
                let GHP = (GHH * GHD) / ((GHB / (((D + ((BXF * (((BXG * GGY) + GW).ln())).exp())) + GHI) + (BXH * GHO))) + (GHC / (((D + ((BXF * (((BXG * GGZ) + GW).ln())).exp())) + GHI) + (BXI * GHO))));
                let GHQ = D / (AJF + GFY);
                let GHT = if CYF != 0.0 {
                    let GHR = D / (D + (CYE * GGV));
                    GHR
                } else {
                    let GHS = D - (CYE * GGV);
                    GHS
                };
                let GHU = EDM * (((D + ((AQI - FKS) / (CYK + ((CYL * GFY) * GFY)))).ln()) * ((GFY * GHQ) * GHT));
                let GHV = D / (D + (GHU * (D + GHU)));
                let GHW = (BYL * GGR) / (BYL + GGR);
                let GIC = if CYQ != 0.0 {
                    let GHX = D / (D - (BYN * GHW));
                    GHX
                } else {
                    let GHY = D + (BYN * GHW);
                    GHY
                };
                let GHZ = (BYL * GGS) / (BYL + GGS);
                let GID = if CYU != 0.0 {
                    let GIA = D / (D - (BYS * GHZ));
                    GIA
                } else {
                    let GIB = D + (BYS * GHZ);
                    GIB
                };
                let GIE = (((EDL * GFZ) * F) * (GIC + GID)) / (GHP * GHV);
                let GIF = GIE * GIE;
                let GIG = (D + (ARL * GIF)) / ((D + GIF).sqrt());
                let GKD = if AEQ != 0.0 {
                    let GII = (D + (EJU * ((CZF * AVY) * ((-1.666666666667e-1f64 * (((GGR * GGR) + CZH).ln())).exp())))) / GIH;
                    GII
                } else {
                    D
                };
                let GJM;
                if FFQ != 0.0 {
                    let GIJ = if GFP > GW { 1.0 } else { 0.0 };
                    let GIY;
                    if GIJ != 0.0 {
                        let GIL = if (GIK.abs()) < O { 1.0 } else { 0.0 };
                        let GIZ = if GIL != 0.0 {
                            let GIN = HC + GEY;
                            let GIO = ((HC + GCK) + (F * GIM)) / (GIN * GIM);
                            let GIP = GIO * GIK;
                            let GIQ = GIP * GIP;
                            let GIT = ((((GIR * GFP) - GCT) / GIM) - ((GFL - (((HC * GCU) * (GIO - (D / GIM))) * (((D - GIP) + GIQ) - (GIP * GIQ)))) / GIN)) / GFP;
                            let GIU = (GIT * GFP) / (GIT + D);
                            GIU
                        } else {
                            let GIW = ((GIR * GIV) / (GIM * GIK)) - (((GCT / GIM) + (GFH / GIK)) / GFP);
                            let GIX = (GIW * GFP) / (GIW + D);
                            GIX
                        };
                        GIY = GIZ;
                    } else {
                        GIY = FIH;
                    }
                    let GJB = GIY - GJA;
                    let GJC = D + ((CAH * GJB) * GJB);
                    let GJD = if (GJB.abs()) > R { 1.0 } else { 0.0 };
                    let GJN = if GJD != 0.0 {
                        let GJE = GFP - FFN;
                        let GJF = GJE - (GIY * GFZ);
                        let GJG = GJE - (GJA * GFZ);
                        let GJH = ((GJF * GJF) + GJC).sqrt();
                        let GJI = ((GJG * GJG) + GJC).sqrt();
                        let GJJ = (ALG / GJB) * (((GJI * GJF) - (GJH * GJG)) + (GJC * (((GJG + GJI) / (GJF + GJH)).ln())));
                        GJJ
                    } else {
                        let GJK = GFZ * GJB;
                        let GJL = (((-4.1666666666675e-2f64 * GFZ) * GJK) * GJK) / (GJC.sqrt());
                        GJL
                    };
                    GJM = GJN;
                } else {
                    GJM = A;
                }
                let GJO = (((GFY * GFZ) + GJM) + FFN) - GFP;
                let GJZ;
                if FFQ != 0.0 {
                    let GJP = if GJO > DAS { 1.0 } else { 0.0 };
                    let GKA = if GJP != 0.0 {
                        let GJQ = ((FIS / ((FCY / FFN) - FIP)) - (GIM / ((GCT / GFP) - GIR))) / GJO;
                        GJQ
                    } else {
                        A
                    };
                    GJZ = GKA;
                } else {
                    let GJT = (-2e0f64 * GJR) * ((EJW / GJS) + FII);
                    let GJW = (-2e0f64 * GJU) * ((EJX / GJV) + FII);
                    let GJX = GJW * EJX;
                    let GJY = (-GJS) * (((((GJX + ((GJW - GJT) * FII)) - (((GJT * EJW) + GJX) / GJS)) / (ATY + (HC * ((GJR * EJW) + (GJU * EJX))))) * GJS) + FII);
                    GJZ = GJY;
                }
                let GKB = F * (GCM - FCR);
                let GKC = GKB * (GJZ * GIG);
                GKF = EIN;
                GKG = EKI;
                GKH = GFY;
                GKI = FCR;
                GKJ = GCM;
                GKL = GGU;
                GKM = GKD;
                GKO = GKB;
                GKP = GKC;
                GKR = GHA;
                GKS = EJZ;
                GKX = FGD;
                GKZ = GFX;
                GLH = EJY;
                GLI = EJX;
                GLK = EJU;
                GLS = EJI;
                GLU = EJV;
                GMG = EJD;
                GMI = EDR;
                GTR = EKQ;
                GTS = EKA;
                GTT = EJT;
                GTX = EJN;
                GTY = EJE;
                GTZ = EJL;
            } else {
                GKF = AWN;
                GKG = AYV;
                GKH = CWK;
                GKI = BRV;
                GKJ = CSY;
                GKL = CXH;
                GKM = DBI;
                GKO = DBG;
                GKP = DBH;
                GKR = CXN;
                GKS = AYL;
                GKX = BVJ;
                GKZ = CWJ;
                GLH = AYK;
                GLI = AYJ;
                GLK = AYG;
                GLS = AXL;
                GLU = AYH;
                GMG = AXF;
                GMI = AQX;
                GTR = AZD;
                GTS = AYN;
                GTT = AYF;
                GTX = AXU;
                GTY = AXG;
                GTZ = AXS;
            }
            let GKK = (F * (GKI + GKJ)) + ((GKE * (GKF - GKG)) / (D + (ALG * GKH)));
            let GKQ = if AEQ != 0.0 {
                let GKN = (GKK + (GKL / GKM)) - GKL;
                GKN
            } else {
                GKK
            };
            let GKU = (GKS * GKT) * ((GKQ * GKR) + ((GKO * GKP) * KJ));
            let GKV = if ALK > A { 1.0 } else { 0.0 };
            let GNM;
            let GNO;
            if GKV != 0.0 {
                let GKW = AXB + 1.3862943611198e0f64;
                let GKY = GKX + GKW;
                let GLA = GKZ + GKW;
                let GLB = GKY - AXB;
                let GLD = F * ((GKY + AXB) - (((GLB * GLB) + GLC).sqrt()));
                let GLE = AXB + AQI;
                let GLF = GLA - GLE;
                let GLG = F * ((GLA + GLE) - (((GLF * GLF) + GLC).sqrt()));
                let GLJ = ALN * ((GLH * (F + GLI)).sqrt());
                let GLM = (GLJ * GLJ) * GLL;
                let GLO = HC * GLM;
                let GLP = GLD + (GLO * (((D + ((GLN - GLD) / GLM)).sqrt()) - D));
                let GLQ = GLG + (GLO * (((D + (((GLN + AQI) - GLG) / GLM)).sqrt()) - D));
                let GLT = (((-(GLR * GKS)) * GLJ) * GLK) * GLS;
                let GLV = GLP - GKY;
                let GLW = F * (GLV + (((GLV * GLV) + D).sqrt()));
                let GLX = ((GLT * GLW) * GLW) / (GLP - GLD);
                let GLY = GLQ - GLA;
                let GLZ = F * (GLY + (((GLY * GLY) + D).sqrt()));
                let GMA = ((GLT * GLZ) * GLZ) / (GLQ - GLG);
                GNM = GLX;
                GNO = GMA;
            } else {
                GNM = A;
                GNO = A;
            }
            let GMB = ZQ * APX;
            let GME = GMC * APY;
            let GMJ = D - ((GMF * GMG) * (D - (GMH * GMI)));
            let GMK = F * (GMJ + (((GMJ * GMJ) + BWH).sqrt()));
            let GML = (ZP * DMD) * GMK;
            let GMM = (DJN * DMG) * GMK;
            let GMO = GMN * AQF;
            if AMJ != 0.0 {
            } else {
            }
            let GMR = GMP * GMQ;
            let GMU = GMR * ((DBU + ECN) + GMS);
            let GMX = GMR * GMV;
            let GNA = GMR * GMY;
            let GNC = GMR * GNB;
            let GNE = GMR * GND;
            let GNF = if DXD < A { 1.0 } else { 0.0 };
            if GNF != 0.0 {
            } else {
            }
            let GVV;
            let GVW;
            if ALX != 0.0 {
                let GNG = GMR * ECT;
                GVV = D;
                GVW = GNG;
            } else {
                GVV = A;
                GVW = A;
            }
            let GVX;
            let GVY;
            if AMA != 0.0 {
                let GNH = GMR * ECV;
                GVX = D;
                GVY = GNH;
            } else {
                GVX = A;
                GVY = A;
            }
            let GVZ;
            let GWA;
            if AMD != 0.0 {
                let GNI = GMR * ECX;
                GVZ = D;
                GWA = GNI;
            } else {
                GVZ = A;
                GWA = A;
            }
            let GWB;
            let GWC;
            if AMG != 0.0 {
                let GNJ = GMR * ECZ;
                GWB = D;
                GWC = GNJ;
            } else {
                GWB = A;
                GWC = A;
            }
            if GNF != 0.0 {
            } else {
            }
            let GNK = parameters[32] * GMQ;
            let GNL = GNK * GKU;
            let GNN = GNK * GNM;
            let GNP = GNK * GNO;
            let GNQ = GNK * GMB;
            let GNR = GNK * GME;
            let GNS = GNK * GML;
            let GNT = GNK * GMM;
            let GNU = GNK * GMO;
            let GNW;
            let GNX;
            if GNF != 0.0 {
                GNW = GNN;
                GNX = GNP;
            } else {
                GNW = GNP;
                GNX = GNN;
            }
            let GNV = if GMQ > A { 1.0 } else { 0.0 };
            if GNV != 0.0 {
            } else {
            }
            let GNY = (AYL / AAM) * DBR;
            let GNZ = CWK + (-5e-1f64 * (DAD + DBK));
            let GOA = CWK / GNZ;
            let GOB = F * (GOA + (((GOA * GOA) + VV).sqrt()));
            let GOC = (-1.666666666667e-1f64 * DBG) * DBD;
            let GOD = GOC * GOC;
            let GOE = CZE - D;
            let GOF = if (D - ((ATJ * GOE) * GOD)) >= VV { (D - ((ATJ * GOE) * GOD)) } else { VV };
            let GOG = D / (GOF * GOF);
            let GOH = (((((DBL * AYL) * DBR) * GNZ) * DBO) / DBP) / DBT;
            let GOI = ATJ * GOD;
            let GOJ = D + GOB;
            let GOK = (GOH * GOG) * (if ((GOB + GOI) - (((HC * GOJ) * GOI) * GOE)) >= AUH { ((GOB + GOI) - (((HC * GOJ) * GOI) * GOE)) } else { AUH });
            let GOL = if ALU > A { 1.0 } else { 0.0 };
            let GOQ = if GOL != 0.0 {
                let GOM = CYZ / CYC;
                let GON = GOK + (((((ALV * DBU) * CBE) * DLT) / (((D + (GOM * GOM)) * GOF) * GOF)) / ALT);
                GON
            } else {
                GOK
            };
            let GOP = GMR * GOO;
            let GOR = GOP * GOQ;
            let GOS = if parameters[6] > A { 1.0 } else { 0.0 };
            let GPB;
            let GPU;
            if GOS != 0.0 {
                let GOT = ((GOH * GOF) * GOF) / (if (((GOB / ATJ) - (GOD * ((GOB + BWH) - GOI))) - (((1.6e0f64 * GOD) * (GOJ - GOI)) * GOE)) >= AUH { (((GOB / ATJ) - (GOD * ((GOB + BWH) - GOI))) - (((1.6e0f64 * GOD) * (GOJ - GOI)) * GOE)) } else { AUH });
                let GOU = GOP * GOT;
                let GOV = if GOQ > A { 1.0 } else { 0.0 };
                let GPC = if GOV != 0.0 {
                    let GOW = (GOG * GOC) * ((D - GOI) - (((GOB + (1.92e1f64 * GOD)) - (GOB * GOI)) * GOE));
                    let GOX = ((GOW * GOW) * GOT) / GOQ;
                    let GOY = F * (GOX + (((GOX * GOX) + AUH).sqrt()));
                    let GOZ = GOY - D;
                    let GPA = F * ((GOY + D) - (((GOZ * GOZ) + AUH).sqrt()));
                    GPA
                } else {
                    A
                };
                GPB = GPC;
                GPU = GOU;
            } else {
                GPB = A;
                GPU = A;
            }
            let GPD = GOR * (D - GPB);
            let GPE = CWK + D;
            let GPF = GNY * GPE;
            let GPG = GNY * (BUT - CWB);
            let GPK = F * GPG;
            let GPN = D + (((GPL * CXD) + (GPM * CXE)) / GPE);
            let GPO = GPN - O;
            let GPP = ((3.20435313e-19f64 * GMP) * GMQ) * (GMV.abs());
            let GPQ = ((3.20435313e-19f64 * GMP) * GMQ) * (GMY.abs());
            let GPT = (((3.20435313e-19f64 * GMP) * GMQ) * ((GNB - GND).abs())) + (GMR * (3.20435313e-19f64 * ((GPR + D) * (GMS.abs()))));
            let GPV = DXD * ((parameters[33] * GMQ) * (if ((((((AAM * DBS) * DBU) / DBP) * ((((GPH - (GPI * GNY)) + ((GPJ * GNY) * GNY)) * (((GPF + GPK) / (GPF - GPK)).ln())) + ((GPI + (GPJ * (GPF - (HC * GNY)))) * GPG))) / GNY) * (F * ((GPN + O) + (((GPO * GPO) + DML).sqrt())))) >= A { ((((((AAM * DBS) * DBU) / DBP) * ((((GPH - (GPI * GNY)) + ((GPJ * GNY) * GNY)) * (((GPF + GPK) / (GPF - GPK)).ln())) + ((GPI + (GPJ * (GPF - (HC * GNY)))) * GPG))) / GNY) * (F * ((GPN + O) + (((GPO * GPO) + DML).sqrt())))) } else { A }));
            let GPX = ZU - (((ZV * Z) * Z) / ZX);
            let GPY = (((ZZ - (((AAA * Z) * Z) / AAC)) - GPX) + (-4e-1f64 * ZS)) * ZR;
            let GPZ = (F * (GPX + GPY)) * AF;
            let GQA = AAJ - (F * GPY);
            let GQB = AF / (D + ((ABR * B) / Z));
            let GQC = ((3.20435313e-19f64 * ABI) * ZT) * GQB;
            let GQD = (((ACC / GQC).ln()) - ACD) + GPZ;
            let GQE = (((8.010882825e-20f64 * ACF) * AAO) / ACG) * GQB;
            let GQF = ADE * GQB;
            let GQS = if AEG != 0.0 {
                let GQG = (D / AF) * ((AEH / ABJ).ln());
                GQG
            } else {
                A
            };
            let GRK;
            if AEQ != 0.0 {
                let GRL = if AHD != 0.0 {
                    let GQH = ((AAD * AAV) * AEU) * ((-3.333333333333e-1f64 * ((AEP / GQB).ln())).exp());
                    GQH
                } else {
                    let GQI = ((AAD * AAV) * AEX) * ((-3.333333333333e-1f64 * ((AEP / GQB).ln())).exp());
                    GQI
                };
                GRK = GRL;
            } else {
                GRK = A;
            }
            let GQJ = AQJ * GQB;
            let GQK = F * ((AQG * GQB) - GQJ);
            let GQL = (AYH / AYG) / AYP;
            let GQM = (AYG / AYH) / AYO;
            let GQN = D + GQL;
            let GQO = (GQN * (((((AYG * GQN) * AZD) / AYN).ln()) + HC)) - (AYF * GQL);
            let GQP = ((D + (D / GQM)) * (((((AYH * (D + GQM)) * AZD) / AYN).ln()) + HC)) - (AYF / GQM);
            let GQQ = GQO - GQP;
            let GQT = (((AER * ((AFH + GQA) + AFK)) + AFD) + AFE) - GQS;
            let GQU = ((((DBR * ((((((((F * ((GQO + GQP) - (((GQQ * GQQ) + GQR).sqrt()))) - AXU) / AXV) + AXU) - AXG) / AXL) - AXS) + AXG)) + AQM) - GQT) * GQB) - GQK;
            let GQV = ((AQW - ((AER * ((AFM + GQA) + AFN)) + AFD)) * GQB) - GQK;
            let GRD;
            if ADR != 0.0 {
                let GQW = AER * ADV;
                let GQX = (GQW * (GQU - GQV)) / ADH;
                let GQY = if GQX < A { 1.0 } else { 0.0 };
                let GRB = if GQY != 0.0 {
                    let GQZ = -2e0f64 * ((D - GQX).ln());
                    GQZ
                } else {
                    let GRA = (GQX * GQX) / (D + ((HC * GQX) / ADH));
                    GRA
                };
                let GRC = GQV + (GQW * GRB);
                GRD = GRC;
            } else {
                GRD = GQV;
            }
            let GRE = GQU - GRD;
            let GRF = ACA * GRE;
            let GRT;
            let GRZ;
            let GSE;
            let GSX;
            if AEQ != 0.0 {
                let GRG = GRF - AVR;
                let GRH = AVR * AVR;
                let GRI = -GRF;
                let GRJ = GRI - AVR;
                let GRM = GRK * ((-3.333333333333e-1f64 * ((F * ((GRF + AVR) + (((GRG * GRG) + GRH).sqrt()))).ln())).exp());
                let GRN = GRK * ((-3.333333333333e-1f64 * ((F * ((GRI + AVR) + (((GRJ * GRJ) + GRH).sqrt()))).ln())).exp());
                let GRO = (D - GRM) - GRN;
                let GRP = ABQ / GRO;
                let GRQ = (ABW * GRO) / (D + (ABW * GRM));
                let GRR = (ABY * GRO) / (D + (ABY * GRN));
                let GRS = D / ((D + (D / GRQ)) + (D / GRR));
                GRT = GRS;
                GRZ = GRQ;
                GSE = GRR;
                GSX = GRP;
            } else {
                GRT = ACA;
                GRZ = ABW;
                GSE = ABY;
                GSX = ABQ;
            }
            let GRU = GRT * GRE;
            let GRV = if GRU > A { 1.0 } else { 0.0 };
            let GSH;
            if GRV != 0.0 {
                let GRW = -GRU;
                let GRX = if GRW < KE { 1.0 } else { 0.0 };
                let GSA = if GRX != 0.0 {
                    let GRY = (D + (GRW.exp())).ln();
                    GRY
                } else {
                    GRW
                };
                let GSB = ((GQU - (GRU / GRZ)) + GSA) - ACD;
                GSH = GSB;
            } else {
                let GSC = if GRU < KE { 1.0 } else { 0.0 };
                let GSF = if GSC != 0.0 {
                    let GSD = (D + (GRU.exp())).ln();
                    GSD
                } else {
                    GRU
                };
                let GSG = ((GRD + (GRU / GSE)) + GSF) - ACD;
                GSH = GSG;
            }
            let GSI = GSH - GQD;
            let GSJ = F * ((GSH + GQD) - (((GSI * GSI) + AJF).sqrt()));
            let GSK = ((D + ((HC * (GQD - GSJ)) / GQE)).sqrt()) - D;
            let GSL = GSJ + (GQE * GSK);
            let GSM = D + (AXH * GQV);
            let GSN = GSM - F;
            let GSO = F * ((GSM + F) + (((GSN * GSN) + O).sqrt()));
            let GSP = D / (D + (AQO * GSO));
            let GSQ = D / (D + (AQP * GSO));
            let GSR = (((HC * GQF) * (((D + (GQJ / GQF)).sqrt()) - D)) * (D + (AXP * GSK))) * (D + (AXQ * GQV));
            let GSS = ACN * GSR;
            let GST = ((((GQU - GSL) + GSS) * GSP) + GSL) + GQK;
            let GSU = GST + (AYC * ((((((GRD - GSL) + (ACR * GSR)) * GSQ) + GSL) + GQK) - GST));
            let GSV = GRZ / GSP;
            let GSW = GSE / GSQ;
            let GSY = GQC / (GSX * GSX);
            let GSZ = D + GSV;
            let GTA = D + GSW;
            let GTB = GSZ / GTA;
            let GTC = GTB.ln();
            let GTD = if GTC > AGX { 1.0 } else { 0.0 };
            let GTJ = if GTD != 0.0 {
                let GTE = ((HC * GTC) * (GTB + D)) / (GTB - D);
                GTE
            } else {
                let GTF = HC * (HC + GTC);
                GTF
            };
            let GTG = (GSW / GSV) / GTA;
            let GTH = (GSV / GSW) / GSZ;
            let GTI = D + GTG;
            let GTK = (GTI * ((((((GSV * GTI) * GTJ) / GSY).ln()) + HC) + GPZ)) - (GSU * GTG);
            let GTL = ((D + (D / GTH)) * ((((((GSW * (D + GTH)) * GTJ) / GSY).ln()) + HC) + GPZ)) - (GSU / GTH);
            let GTM = GTK - GTL;
            let GTN = (((((((((F * ((GTK + GTL) - (((GTM * GTM) + GQR).sqrt()))) - AXU) / AXV) + AXU) - GSL) / GSP) - GSS) + GSL) / GQB) + GQT;
            if EDA != 0.0 {
                let GTO = (GLU / GLK) / (D + GLU);
                let GTP = (GLK / GLU) / (D + GLK);
                let GTQ = D + GTO;
                let GTU = (GTQ * (((((GLK * GTQ) * GTR) / GTS).ln()) + HC)) - (GTT * GTO);
                let GTV = ((D + (D / GTP)) * (((((GLU * (D + GTP)) * GTR) / GTS).ln()) + HC)) - (GTT / GTP);
                let GTW = GTU - GTV;
                let GUA = ((((DBR * ((((((((F * ((GTU + GTV) - (((GTW * GTW) + GQR).sqrt()))) - GTX) / AXV) + GTX) - GTY) / GLS) - GTZ) + GTY)) + EDB) - ((((AER * ((AFP + GQA) + AFK)) + AFD) + AFE) - GQS)) * GQB) - GQK;
                let GUB = ((AQW - ((AER * ((AFV + GQA) + AFN)) + AFD)) * GQB) - GQK;
                let GUJ;
                if ADR != 0.0 {
                    let GUC = AER * ADV;
                    let GUD = (GUC * (GUA - GUB)) / ADH;
                    let GUE = if GUD < A { 1.0 } else { 0.0 };
                    let GUH = if GUE != 0.0 {
                        let GUF = -2e0f64 * ((D - GUD).ln());
                        GUF
                    } else {
                        let GUG = (GUD * GUD) / (D + ((HC * GUD) / ADH));
                        GUG
                    };
                    let GUI = GUB + (GUC * GUH);
                    GUJ = GUI;
                } else {
                    GUJ = GUB;
                }
                let GUK = GUA - GUJ;
                let GUL = ACA * GUK;
                let GUW;
                let GVA;
                let GVC;
                if AEQ != 0.0 {
                    let GUM = GUL - AVR;
                    let GUN = AVR * AVR;
                    let GUO = -GUL;
                    let GUP = GUO - AVR;
                    let GUQ = GRK * ((-3.333333333333e-1f64 * ((F * ((GUL + AVR) + (((GUM * GUM) + GUN).sqrt()))).ln())).exp());
                    let GUR = GRK * ((-3.333333333333e-1f64 * ((F * ((GUO + AVR) + (((GUP * GUP) + GUN).sqrt()))).ln())).exp());
                    let GUS = (D - GUQ) - GUR;
                    let GUT = (ABW * GUS) / (D + (ABW * GUQ));
                    let GUU = (ABY * GUS) / (D + (ABY * GUR));
                    let GUV = D / ((D + (D / GUT)) + (D / GUU));
                    GUW = GUV;
                    GVA = GUT;
                    GVC = GUU;
                } else {
                    GUW = ACA;
                    GVA = ABW;
                    GVC = ABY;
                }
                let GUX = GUW * GUK;
                let GUY = if GUX > A { 1.0 } else { 0.0 };
                if GUY != 0.0 {
                    let GUZ = if (-GUX) < KE { 1.0 } else { 0.0 };
                    if GUZ != 0.0 {
                    } else {
                    }
                } else {
                    let GVB = if GUX < KE { 1.0 } else { 0.0 };
                    if GVB != 0.0 {
                    } else {
                    }
                }
                let GVD = D + (AXH * GUB);
                let GVE = GVD - F;
                let GVF = F * ((GVD + F) + (((GVE * GVE) + O).sqrt()));
                let GVG = if (((D + (GVA / (D / (D + (EDD * GVF))))) / (D + (GVC / (D / (D + (EDG * GVF)))))).ln()) > AGX { 1.0 } else { 0.0 };
                if GVG != 0.0 {
                } else {
                }
            } else {
            }
            let GVH = AQD - GTN;
            let GVI;
            let GVJ;
            let GVK;
            if GNF != 0.0 {
                GVI = GNE;
                GVJ = GNC;
                GVK = GMX;
            } else {
                GVI = GNC;
                GVJ = GNE;
                GVK = GNA;
            }
            let GVL = ((GMU + GVI) - GVJ) - GVK;
            let GVQ = if GNF != 0.0 {
                let GVM = AER * 0e0f64;
                GVM
            } else {
                let GVN = AER * 0e0f64;
                GVN
            };
            if GNF != 0.0 {
            } else {
            }
            let GVO = ((((((GNL + GNQ) + GNR) + GNS) + GNT) + GNU) + GNX) + GNW;
            if GNF != 0.0 {
            } else {
            }
            let GVP = AER * 0e0f64;
            let GVR = if (GVQ.abs()) < AUH { 1.0 } else { 0.0 };
            if GVR != 0.0 {
            } else {
            }
            let GVS = if (GVH.abs()) < AUH { 1.0 } else { 0.0 };
            if GVS != 0.0 {
            } else {
            }
            let GVT = if (GVP.abs()) < AUH { 1.0 } else { 0.0 };
            if GVT != 0.0 {
            } else {
            }
            if GNF != 0.0 {
            } else {
            }
            let GVU = if (GVL.abs()) < AUH { 1.0 } else { 0.0 };
            if GVU != 0.0 {
            } else {
            }
        if GVV == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GVW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GVX == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GVY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GVZ == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GWA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GWB == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GWC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = GPU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = GPD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = GPV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(GPW);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = GPP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = GPQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = GPT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
