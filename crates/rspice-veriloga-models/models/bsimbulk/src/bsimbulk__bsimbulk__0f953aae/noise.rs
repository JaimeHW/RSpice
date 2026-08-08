#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::rspice_limited_exp;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 23] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 3, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 4, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "N2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 6, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "N1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 9, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF_EDGEFET", label: Some("1overf_edgefet"), kind: GeneratedNoiseKind::Flicker, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI1_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI1_DI_RDRIFT_D", label: Some("rdrift_d"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI1_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI1_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI1_SI_RDRIFT_S", label: Some("rdrift_s"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_SI1_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GM_RG", label: Some("rg"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "gm", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SBULK_BI_RBPS", label: Some("rbps"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "sbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SBULK_B_RBSB", label: Some("rbsb"), kind: GeneratedNoiseKind::White, equation: 63, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "sbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RBPB", label: Some("rbpb"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DBULK_BI_RBPD", label: Some("rbpd"), kind: GeneratedNoiseKind::White, equation: 65, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "dbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DBULK_B_RBDB", label: Some("rbdb"), kind: GeneratedNoiseKind::White, equation: 66, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "dbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DDBULK_D_RDB", label: Some("rdb"), kind: GeneratedNoiseKind::White, equation: 79, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "ddbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16])];
            let A = 0e0f64;
            let B = 1e0f64;
            let C = parameters[39];
            let E = -1e0f64;
            let F = parameters[110];
            let G = 8.85418e-12f64;
            let I = parameters[111];
            let K = parameters[77];
            let O = parameters[52];
            let Q = parameters[1];
            let R = parameters[53];
            let V = parameters[2];
            let AD = parameters[57];
            let AE = parameters[58];
            let AF = parameters[59];
            let AG = parameters[60];
            let AH = parameters[67];
            let AK = parameters[68];
            let AO = parameters[63];
            let AP = parameters[64];
            let AQ = parameters[65];
            let AR = parameters[66];
            let AT = 2e0f64;
            let AW = 1e-9f64;
            let BB = parameters[74];
            let BC = parameters[75];
            let BD = parameters[76];
            let BP = 1e-6f64;
            let BX = parameters[818];
            let CD = parameters[819];
            let IL = parameters[786];
            let JE = parameters[82];
            let JF = parameters[84];
            let JG = parameters[86];
            let JI = parameters[215];
            let JJ = parameters[217];
            let JL = parameters[225];
            let JQ = parameters[235];
            let JU = parameters[275];
            let JW = parameters[274];
            let KI = parameters[286];
            let KJ = parameters[288];
            let KO = parameters[303];
            let KP = parameters[305];
            let KR = parameters[310];
            let KW = parameters[328];
            let KX = parameters[330];
            let LC = parameters[179];
            let LH = parameters[181];
            let LJ = parameters[462];
            let LO = parameters[258];
            let LP = 5e-1f64;
            let LR = parameters[480];
            let LT = parameters[342];
            let LY = parameters[244];
            let LZ = parameters[246];
            let ME = parameters[424];
            let MG = 2.5e-1f64;
            let MK = parameters[439];
            let MP = parameters[486];
            let MQ = parameters[488];
            let MV = parameters[496];
            let MX = parameters[520];
            let MZ = parameters[523];
            let NH = parameters[94];
            let NI = parameters[96];
            let NJ = parameters[98];
            let NL = parameters[121];
            let NM = parameters[123];
            let NO = parameters[131];
            let NP = parameters[133];
            let NR = parameters[264];
            let NS = parameters[266];
            let NU = parameters[353];
            let NW = parameters[187];
            let NX = parameters[189];
            let NZ = parameters[197];
            let OA = parameters[199];
            let OC = parameters[384];
            let OJ = 0.0f64;
            let OK = parameters[49];
            let OL = parameters[909];
            let ON = parameters[42];
            let OP = parameters[398];
            let OR = parameters[408];
            let OT = parameters[415];
            let PZ = 6.7e-2f64;
            let QI = parameters[1066];
            let QL = parameters[801];
            let QO = parameters[698];
            let QQ = parameters[696];
            let QS = if parameter_given[3] { 1.0 } else { 0.0 };
            let QT = parameters[374];
            let QV = parameters[10];
            let QX = parameters[9];
            let QY = 9e0f64;
            let RC = parameters[6];
            let RG = 1.0f64;
            let RQ = 1.0f64;
            let RR = 1.0f64;
            let RS = 5e0f64;
            let RZ = 3e0f64;
            let SA = 4e0f64;
            let SB = 6e0f64;
            let SH = 7e0f64;
            let SL = 8e0f64;
            let SR = 0.0f64;
            let TM = 1.0f64;
            let TN = 1.0f64;
            let UE = 0.0f64;
            let UU = 1.0f64;
            let UV = 1.0f64;
            let VK = 0.0f64;
            let WC = 1.0f64;
            let WD = 1.0f64;
            let WS = 0.0f64;
            let XI = 1.0f64;
            let XJ = 1.0f64;
            let YC = 1.0f64;
            let YD = 1.0f64;
            let YV = 1.0f64;
            let YX = 0.0f64;
            let ZP = 1.0f64;
            let ZS = 0.0f64;
            let AAK = 1.0f64;
            let AAP = 1e1f64;
            let AAR = 1.0f64;
            let AGW = if parameter_given[4] { 1.0 } else { 0.0 };
            let AHF = 0.0f64;
            let AHP = 0.0f64;
            let AHQ = 1.0f64;
            let AIM = 0.0f64;
            let AJJ = 0.0f64;
            let AJK = 1.0f64;
            let AKB = 0.0f64;
            let AKR = 0.0f64;
            let AKS = 1.0f64;
            let ALH = 0.0f64;
            let ALZ = 0.0f64;
            let AMA = 1.0f64;
            let AMP = 0.0f64;
            let ANF = 0.0f64;
            let ANG = 1.0f64;
            let ANZ = 0.0f64;
            let AOA = 1.0f64;
            let AOS = 0.0f64;
            let AOU = 0.0f64;
            let APM = 0.0f64;
            let APP = 0.0f64;
            let AQH = 0.0f64;
            let AQN = 0.0f64;
            let AWZ = parameters[1093];
            let AXP = parameters[8];
            let AXR = 1e6f64;
            let AXS = 1e-38f64;
            let AXW = parameters[11];
            let AXX = parameters[12];
            let AXY = parameters[13];
            let AXZ = parameters[14];
            let AYA = parameters[15];
            let AYY = 1e-3f64;
            let AZA = 1e3f64;
            let AZB = parameters[756];
            let BAC = parameters[1097];
            let BAE = parameters[16];
            let BAH = parameters[1128];
            let BAJ = parameters[32];
            let BAN = parameters[7];
            let BAQ = parameters[555];
            let BAT = 4.97232e-7f64;
            let BAU = 3.42537e-7f64;
            let BAW = 7.45669e11f64;
            let BAX = 1.16645e12f64;
            let BBF = parameters[820];
            let BBH = 3.0015e2f64;
            let BBK = node_potentials[4];
            let BBN = 8.617087e-5f64;
            let BBU = parameters[109];
            let BBV = parameters[821];
            let BBW = parameters[822];
            let BCJ = 4e-1f64;
            let BCO = 1.60219e-19f64;
            let BDD = 3.333333333333333e-1f64;
            let BDE = parameters[283];
            let BFM = 1e2f64;
            let BFO = parameters[1094];
            let BKF = 1e-2f64;
            let BQO = if parameter_given[24] { 1.0 } else { 0.0 };
            let BRD = if parameter_given[25] { 1.0 } else { 0.0 };
            let BRS = if parameter_given[26] { 1.0 } else { 0.0 };
            let BRT = parameters[137];
            let BRV = parameters[26];
            let BSK = if parameter_given[27] { 1.0 } else { 0.0 };
            let BSM = parameters[27];
            let BTQ = parameters[17];
            let BTR = parameters[18];
            let BTS = parameters[19];
            let BUR = parameters[916];
            let BVB = parameters[37];
            let BVM = parameters[20];
            let BVN = parameters[21];
            let BVO = parameters[22];
            let BVQ = parameters[23];
            let BVT = parameters[947];
            let BVW = 1e-1f64;
            let BVZ = 5e-2f64;
            let BWB = 2e1f64;
            let BWU = node_potentials[11];
            let BWW = node_potentials[5];
            let BWY = node_potentials[7];
            let BXC = node_potentials[13];
            let BXH = node_potentials[10];
            let BXK = parameters[1110];
            let BXO = node_potentials[6];
            let BXU = -1e0f64;
            let BYB = parameters[956];
            let BYD = 3.7e1f64;
            let CAD = 1.6e1f64;
            let CBA = 8e1f64;
            let CBC = 1.804851387e-35f64;
            let CBH = parameters[35];
            let CBZ = 1.4142135623730951e0f64;
            let CCT = 2.01491e-1f64;
            let CCU = 4.02982e-1f64;
            let CCV = 2.446562e0f64;
            let CCY = -1e2f64;
            let CDA = 1.804851387e-35f64;
            let CDF = 1.25e0f64;
            let CJG = parameters[1130];
            let CJH = parameters[1131];
            let CKH = -1e2f64;
            let CKJ = 1.804851387e-35f64;
            let CLV = 8e-1f64;
            let CLW = 1.2e0f64;
            let CNH = parameters[350];
            let CNZ = 5.540622384e34f64;
            let COV = node_potentials[8];
            let CQI = parameters[1117];
            let CQZ = parameters[1127];
            let CRH = parameters[514];
            let CRJ = parameters[1098];
            let CRL = node_potentials[3];
            let CRN = parameters[515];
            let CRP = parameters[1099];
            let CRY = parameters[1124];
            let CSA = parameters[1125];
            let CSM = parameters[1107];
            let CSP = parameters[1122];
            let CSX = parameters[1112];
            let CTA = parameters[516];
            let CTE = parameters[517];
            let CTG = parameters[1109];
            let CUK = 0e0f64;
            let CUO = parameters[1108];
            let CVS = parameters[28];
            let CVV = parameters[1114];
            let CWT = -1e2f64;
            let CWV = 1.804851387e-35f64;
            let CXV = parameters[1096];
            let CYQ = -1e2f64;
            let CYS = 1.804851387e-35f64;
            let DBB = parameters[504];
            let DBI = node_potentials[0];
            let DBJ = node_potentials[2];
            let DBK = parameters[512];
            let DBN = parameters[503];
            let DCJ = 1e-4f64;
            let DFR = parameters[748];
            let DFU = parameters[750];
            let DFX = parameters[752];
            let DGD = parameters[749];
            let DGI = parameters[751];
            let DGL = parameters[753];
            let DGU = parameters[713];
            let DGW = parameters[715];
            let DGY = parameters[717];
            let DHB = 9e-1f64;
            let DHU = parameters[714];
            let DHW = parameters[716];
            let DHY = parameters[718];
            let DJF = parameters[784];
            let DJR = parameters[785];
            let DJS = parameters[799];
            let DJT = parameters[800];
            let DJZ = parameters[1068];
            let DKW = -1e2f64;
            let DKY = 1.804851387e-35f64;
            let DMB = 0e0f64;
            let DMW = 0e0f64;
            let DMZ = 1e10f64;
            let DNM = parameters[30];
            let DNO = parameters[783];
            let DNV = parameters[798];
            let DOU = parameters[48];
            let DPQ = 1.2e1f64;
            let DPS = 1.44e2f64;
            let DRR = parameters[1137];
            let DRV = -1e2f64;
            let DRX = 1.804851387e-35f64;
            let DUF = parameters[1134];
            let DUG = parameters[1135];
            let DUI = parameters[1129];
            let DVB = -1e2f64;
            let DVD = 1.804851387e-35f64;
            let EAQ = parameters[1014];
            let EBX = -1e2f64;
            let EBZ = 1.804851387e-35f64;
            let EDJ = -1e2f64;
            let EDL = 1.804851387e-35f64;
            let EEU = parameters[957];
            let EEY = parameters[1062];
            let EGF = parameters[805];
            let EGQ = parameters[804];
            let D = if C == B { 1.0 } else { 0.0 };
            let BCH = if D != 0.0 {
                B
            } else {
                E
            };
            let H = F * G;
            let J = I * G;
            let L = J / K;
            let M = F / I;
            let N = if (if parameter_given[78] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            if N != 0.0 {
            } else {
            }
            let P = parameters[0] * O;
            let S = Q * R;
            let T = P + parameters[54];
            let U = if T <= A { 1.0 } else { 0.0 };
            if U != 0.0 {
            } else {
            }
            let W = (S / V) + parameters[56];
            let X = if W <= A { 1.0 } else { 0.0 };
            if X != 0.0 {
            } else {
            }
            let Y = -parameters[61];
            let Z = T.powf(Y);
            let AA = -parameters[62];
            let AB = W.powf(AA);
            let AC = Z * AB;
            let AI = -AH;
            let AJ = T.powf(AI);
            let AL = -AK;
            let AM = W.powf(AL);
            let AN = AJ * AM;
            let AS = ((AO + (AP * AJ)) + (AQ * AM)) + (AR * AN);
            let AU = T - (AT * (((AD + (AE * Z)) + (AF * AB)) + (AG * AC)));
            let AV = if AU <= A { 1.0 } else { 0.0 };
            if AV != 0.0 {
            } else {
                let AX = if AU <= AW { 1.0 } else { 0.0 };
                if AX != 0.0 {
                } else {
                }
            }
            let AY = W - (AT * AS);
            let AZ = if AY <= A { 1.0 } else { 0.0 };
            if AZ != 0.0 {
            } else {
                let BA = if AY <= AW { 1.0 } else { 0.0 };
                if BA != 0.0 {
                } else {
                }
            }
            let BE = ((parameters[73] + (BB * AJ)) + (BC * AM)) + (BD * AN);
            let BF = T - (AT * (((parameters[69] + (parameters[70] * Z)) + (parameters[71] * AB)) + (parameters[72] * AC)));
            let BG = if BF <= A { 1.0 } else { 0.0 };
            if BG != 0.0 {
            } else {
                let BH = if BF <= AW { 1.0 } else { 0.0 };
                if BH != 0.0 {
                } else {
                }
            }
            let BI = W - (AT * BE);
            let BJ = if BI <= A { 1.0 } else { 0.0 };
            if BJ != 0.0 {
            } else {
                let BK = if BI <= AW { 1.0 } else { 0.0 };
                if BK != 0.0 {
                } else {
                }
            }
            let BL = T.powf(AH);
            let BM = W.powf(AK);
            let BN = W - (AT * (((parameters[138] + (BB / BL)) + (BC / BM)) + ((BD / BL) / BM)));
            let BO = if BN <= A { 1.0 } else { 0.0 };
            if BO != 0.0 {
            } else {
            }
            let BQ = BP / AU;
            let BR = BP / AY;
            let BS = BP / BF;
            let BT = BP / BI;
            let BU = BP / parameters[51];
            let BV = BP / parameters[55];
            let BW = BQ * BR;
            let BY = if BX != A { 1.0 } else { 0.0 };
            let CJ;
            let CN;
            if BY != 0.0 {
                let BZ = if BX <= (-T) { 1.0 } else { 0.0 };
                let CK;
                let CO;
                if BZ != 0.0 {
                    CK = Z;
                    CO = AJ;
                } else {
                    let CA = T + BX;
                    let CB = CA.powf(Y);
                    let CC = CA.powf(AI);
                    CK = CB;
                    CO = CC;
                }
                CJ = CK;
                CN = CO;
            } else {
                CJ = Z;
                CN = AJ;
            }
            let CE = if CD != A { 1.0 } else { 0.0 };
            let CL;
            let CP;
            if CE != 0.0 {
                let CF = if CD <= (-W) { 1.0 } else { 0.0 };
                let CM;
                let CQ;
                if CF != 0.0 {
                    CM = AB;
                    CQ = AM;
                } else {
                    let CG = W + CD;
                    let CH = CG.powf(AA);
                    let CI = CG.powf(AL);
                    CM = CH;
                    CQ = CI;
                }
                CL = CM;
                CP = CQ;
            } else {
                CL = AB;
                CP = AM;
            }
            let CR = ((AO + (AP * CN)) + (AQ * CP)) + (AR * (CN * CP));
            let CS = (T - (AT * (((AD + (AE * CJ)) + (AF * CL)) + (AG * (CJ * CL))))) + BX;
            let CT = if CS <= A { 1.0 } else { 0.0 };
            if CT != 0.0 {
            } else {
            }
            let CU = (W - (AT * CR)) + CD;
            let CV = if CU <= A { 1.0 } else { 0.0 };
            if CV != 0.0 {
            } else {
            }
            let CW = if parameters[817] == B { 1.0 } else { 0.0 };
            let DB;
            let DC;
            if CW != 0.0 {
                let CX = BP / CS;
                let CY = BP / CU;
                DB = CX;
                DC = CY;
            } else {
                let CZ = B / CS;
                let DA = B / CU;
                DB = CZ;
                DC = DA;
            }
            let DD = DB * DC;
            let DE = ((parameters[116] + (DB * parameters[117])) + (DC * parameters[118])) + (DD * parameters[119]);
            let DF = ((parameters[126] + (DB * parameters[127])) + (DC * parameters[128])) + (DD * parameters[129]);
            let DG = ((parameters[139] + (DB * parameters[140])) + (DC * parameters[141])) + (DD * parameters[142]);
            let DH = ((parameters[80] + (DB * parameters[89])) + (DC * parameters[90])) + (DD * parameters[91]);
            let DI = ((parameters[92] + (DB * parameters[101])) + (DC * parameters[102])) + (DD * parameters[103]);
            let DJ = ((parameters[104] + (DB * parameters[105])) + (DC * parameters[106])) + (DD * parameters[107]);
            let DK = ((parameters[209] + (DB * parameters[210])) + (DC * parameters[211])) + (DD * parameters[212]);
            let DL = ((parameters[213] + (DB * parameters[220])) + (DC * parameters[221])) + (DD * parameters[222]);
            let DM = ((parameters[223] + (DB * parameters[226])) + (DC * parameters[227])) + (DD * parameters[228]);
            let DN = ((parameters[233] + (DB * parameters[236])) + (DC * parameters[237])) + (DD * parameters[238]);
            let DO = ((parameters[143] + (DB * parameters[144])) + (DC * parameters[145])) + (DD * parameters[146]);
            let DP = ((parameters[147] + (DB * parameters[148])) + (DC * parameters[149])) + (DD * parameters[150]);
            let DQ = ((parameters[151] + (DB * parameters[152])) + (DC * parameters[153])) + (DD * parameters[154]);
            let DR = ((parameters[155] + (DB * parameters[156])) + (DC * parameters[157])) + (DD * parameters[158]);
            let DS = ((parameters[159] + (DB * parameters[160])) + (DC * parameters[161])) + (DD * parameters[162]);
            let DT = ((parameters[163] + (DB * parameters[164])) + (DC * parameters[165])) + (DD * parameters[166]);
            let DU = ((parameters[195] + (DB * parameters[202])) + (DC * parameters[203])) + (DD * parameters[204]);
            let DV = ((parameters[185] + (DB * parameters[192])) + (DC * parameters[193])) + (DD * parameters[194]);
            let DW = ((parameters[112] + (DB * parameters[113])) + (DC * parameters[114])) + (DD * parameters[115]);
            let DX = ((parameters[167] + (DB * parameters[168])) + (DC * parameters[169])) + (DD * parameters[170]);
            let DY = ((parameters[171] + (DB * parameters[172])) + (DC * parameters[173])) + (DD * parameters[174]);
            let DZ = ((parameters[180] + (DB * parameters[182])) + (DC * parameters[183])) + (DD * parameters[184]);
            let EA = ((parameters[253] + (DB * parameters[254])) + (DC * parameters[255])) + (DD * parameters[256]);
            let EB = ((parameters[273] + (DB * parameters[276])) + (DC * parameters[277])) + (DD * parameters[278]);
            let EC = ((parameters[284] + (DB * parameters[291])) + (DC * parameters[292])) + (DD * parameters[293]);
            let ED = ((parameters[308] + (DB * parameters[311])) + (DC * parameters[312])) + (DD * parameters[313]);
            let EE = ((parameters[298] + (DB * parameters[299])) + (DC * parameters[300])) + (DD * parameters[301]);
            let EF = ((parameters[318] + (DB * parameters[319])) + (DC * parameters[320])) + (DD * parameters[321]);
            let EG = ((parameters[326] + (DB * parameters[333])) + (DC * parameters[334])) + (DD * parameters[335]);
            let EH = ((parameters[340] + (DB * parameters[343])) + (DC * parameters[344])) + (DD * parameters[345]);
            let EI = ((parameters[351] + (DB * parameters[354])) + (DC * parameters[355])) + (DD * parameters[356]);
            let EJ = ((parameters[393] + (DB * parameters[394])) + (DC * parameters[395])) + (DD * parameters[396]);
            let EK = ((parameters[403] + (DB * parameters[404])) + (DC * parameters[405])) + (DD * parameters[406]);
            let EL = ((parameters[375] + (DB * parameters[376])) + (DC * parameters[377])) + (DD * parameters[378]);
            let EM = ((parameters[379] + (DB * parameters[380])) + (DC * parameters[381])) + (DD * parameters[382]);
            let EN = ((parameters[385] + (DB * parameters[386])) + (DC * parameters[387])) + (DD * parameters[388]);
            let EO = ((parameters[389] + (DB * parameters[390])) + (DC * parameters[391])) + (DD * parameters[392]);
            let EP = ((parameters[399] + (DB * parameters[400])) + (DC * parameters[401])) + (DD * parameters[402]);
            let EQ = ((parameters[413] + (DB * parameters[416])) + (DC * parameters[417])) + (DD * parameters[418]);
            let ER = ((parameters[409] + (DB * parameters[410])) + (DC * parameters[411])) + (DD * parameters[412]);
            let ES = ((parameters[434] + (DB * parameters[435])) + (DC * parameters[436])) + (DD * parameters[437]);
            let ET = ((parameters[460] + (DB * parameters[463])) + (DC * parameters[464])) + (DD * parameters[465]);
            let EU = ((parameters[470] + (DB * parameters[471])) + (DC * parameters[472])) + (DD * parameters[473]);
            let EV = ((parameters[357] + (DB * parameters[358])) + (DC * parameters[359])) + (DD * parameters[360]);
            let EW = ((parameters[361] + (DB * parameters[362])) + (DC * parameters[363])) + (DD * parameters[364]);
            let EX = ((parameters[365] + (DB * parameters[366])) + (DC * parameters[367])) + (DD * parameters[368]);
            let EY = ((parameters[370] + (DB * parameters[371])) + (DC * parameters[372])) + (DD * parameters[373]);
            let EZ = ((parameters[478] + (DB * parameters[481])) + (DC * parameters[482])) + (DD * parameters[483]);
            let FA = ((parameters[474] + (DB * parameters[475])) + (DC * parameters[476])) + (DD * parameters[477]);
            let FB = ((parameters[239] + (DB * parameters[240])) + (DC * parameters[241])) + (DD * parameters[242]);
            let FC = ((parameters[419] + (DB * parameters[420])) + (DC * parameters[421])) + (DD * parameters[422]);
            let FD = ((parameters[259] + (DB * parameters[260])) + (DC * parameters[261])) + (DD * parameters[262]);
            let FE = ((parameters[682] + (DB * parameters[683])) + (DC * parameters[684])) + (DD * parameters[685]);
            let FF = ((parameters[686] + (DB * parameters[687])) + (DC * parameters[688])) + (DD * parameters[689]);
            let FG = ((parameters[484] + (DB * parameters[489])) + (DC * parameters[490])) + (DD * parameters[491]);
            let FH = ((parameters[494] + (DB * parameters[497])) + (DC * parameters[498])) + (DD * parameters[499]);
            let FI = ((parameters[935] + (DB * parameters[936])) + (DC * parameters[937])) + (DD * parameters[938]);
            let FJ = ((parameters[939] + (DB * parameters[940])) + (DC * parameters[941])) + (DD * parameters[942]);
            let FK = ((parameters[943] + (DB * parameters[944])) + (DC * parameters[945])) + (DD * parameters[946]);
            let FL = ((parameters[630] + (DB * parameters[633])) + (DC * parameters[634])) + (DD * parameters[635]);
            let FM = ((parameters[636] + (DB * parameters[637])) + (DC * parameters[638])) + (DD * parameters[639]);
            let FN = ((parameters[640] + (DB * parameters[641])) + (DC * parameters[642])) + (DD * parameters[643]);
            let FO = ((parameters[644] + (DB * parameters[645])) + (DC * parameters[646])) + (DD * parameters[647]);
            let FP = ((parameters[648] + (DB * parameters[651])) + (DC * parameters[652])) + (DD * parameters[653]);
            let FQ = ((parameters[654] + (DB * parameters[655])) + (DC * parameters[656])) + (DD * parameters[657]);
            let FR = ((parameters[658] + (DB * parameters[659])) + (DC * parameters[660])) + (DD * parameters[661]);
            let FS = ((parameters[662] + (DB * parameters[663])) + (DC * parameters[664])) + (DD * parameters[665]);
            let FT = ((parameters[824] + (DB * parameters[825])) + (DC * parameters[826])) + (DD * parameters[827]);
            let FU = ((parameters[829] + (DB * parameters[830])) + (DC * parameters[831])) + (DD * parameters[832]);
            let FV = ((parameters[834] + (DB * parameters[835])) + (DC * parameters[836])) + (DD * parameters[837]);
            let FW = ((parameters[838] + (DB * parameters[839])) + (DC * parameters[840])) + (DD * parameters[841]);
            let FX = ((parameters[843] + (DB * parameters[844])) + (DC * parameters[845])) + (DD * parameters[846]);
            let FY = ((parameters[847] + (DB * parameters[848])) + (DC * parameters[849])) + (DD * parameters[850]);
            let FZ = ((parameters[852] + (DB * parameters[853])) + (DC * parameters[854])) + (DD * parameters[855]);
            let GA = ((parameters[856] + (DB * parameters[857])) + (DC * parameters[858])) + (DD * parameters[859]);
            let GB = ((parameters[862] + (DB * parameters[863])) + (DC * parameters[864])) + (DD * parameters[865]);
            let GC = ((parameters[877] + (DB * parameters[878])) + (DC * parameters[879])) + (DD * parameters[880]);
            let GD = ((parameters[885] + (DB * parameters[886])) + (DC * parameters[887])) + (DD * parameters[888]);
            let GE = ((parameters[881] + (DB * parameters[882])) + (DC * parameters[883])) + (DD * parameters[884]);
            let GF = ((parameters[537] + (DB * parameters[564])) + (DC * parameters[565])) + (DD * parameters[566]);
            let GG = ((parameters[538] + (DB * parameters[567])) + (DC * parameters[568])) + (DD * parameters[569]);
            let GH = ((parameters[539] + (DB * parameters[570])) + (DC * parameters[571])) + (DD * parameters[572]);
            let GI = ((parameters[540] + (DB * parameters[573])) + (DC * parameters[574])) + (DD * parameters[575]);
            let GJ = ((parameters[541] + (DB * parameters[576])) + (DC * parameters[577])) + (DD * parameters[578]);
            let GK = ((parameters[533] + (DB * parameters[579])) + (DC * parameters[580])) + (DD * parameters[581]);
            let GL = ((parameters[534] + (DB * parameters[582])) + (DC * parameters[583])) + (DD * parameters[584]);
            let GM = ((parameters[535] + (DB * parameters[585])) + (DC * parameters[586])) + (DD * parameters[587]);
            let GN = ((parameters[536] + (DB * parameters[588])) + (DC * parameters[589])) + (DD * parameters[590]);
            let GO = ((parameters[542] + (DB * parameters[591])) + (DC * parameters[592])) + (DD * parameters[593]);
            let GP = ((parameters[543] + (DB * parameters[594])) + (DC * parameters[595])) + (DD * parameters[596]);
            let GQ = ((parameters[544] + (DB * parameters[597])) + (DC * parameters[598])) + (DD * parameters[599]);
            let GR = ((parameters[545] + (DB * parameters[600])) + (DC * parameters[601])) + (DD * parameters[602]);
            let GS = ((parameters[546] + (DB * parameters[603])) + (DC * parameters[604])) + (DD * parameters[605]);
            let GT = ((parameters[547] + (DB * parameters[606])) + (DC * parameters[607])) + (DD * parameters[608]);
            let GU = ((parameters[548] + (DB * parameters[609])) + (DC * parameters[610])) + (DD * parameters[611]);
            let GV = ((parameters[549] + (DB * parameters[612])) + (DC * parameters[613])) + (DD * parameters[614]);
            let GW = ((parameters[550] + (DB * parameters[615])) + (DC * parameters[616])) + (DD * parameters[617]);
            let GX = ((parameters[553] + (DB * parameters[618])) + (DC * parameters[619])) + (DD * parameters[620]);
            let GY = ((parameters[551] + (DB * parameters[621])) + (DC * parameters[622])) + (DD * parameters[623]);
            let GZ = ((parameters[552] + (DB * parameters[624])) + (DC * parameters[625])) + (DD * parameters[626]);
            let HA = ((parameters[554] + (DB * parameters[627])) + (DC * parameters[628])) + (DD * parameters[629]);
            let HB = ((parameters[867] + (DB * parameters[870])) + (DC * parameters[871])) + (DD * parameters[872]);
            let HC = ((parameters[873] + (DB * parameters[874])) + (DC * parameters[875])) + (DD * parameters[876]);
            let HD = ((parameters[425] + (DB * parameters[430])) + (DC * parameters[431])) + (DD * parameters[432]);
            let HE = ((parameters[444] + (DB * parameters[445])) + (DC * parameters[446])) + (DD * parameters[447]);
            let HF = ((parameters[448] + (DB * parameters[449])) + (DC * parameters[450])) + (DD * parameters[451]);
            let HG = ((parameters[452] + (DB * parameters[453])) + (DC * parameters[454])) + (DD * parameters[455]);
            let HH = ((parameters[456] + (DB * parameters[457])) + (DC * parameters[458])) + (DD * parameters[459]);
            let HI = ((parameters[1046] + (DB * parameters[1047])) + (DC * parameters[1048])) + (DD * parameters[1049]);
            let HJ = ((parameters[1054] + (DB * parameters[1055])) + (DC * parameters[1056])) + (DD * parameters[1057]);
            let HK = ((parameters[1050] + (DB * parameters[1051])) + (DC * parameters[1052])) + (DD * parameters[1053]);
            let HL = ((parameters[1058] + (DB * parameters[1059])) + (DC * parameters[1060])) + (DD * parameters[1061]);
            let HM = ((parameters[966] + (DB * parameters[967])) + (DC * parameters[968])) + (DD * parameters[969]);
            let HN = ((parameters[962] + (DB * parameters[963])) + (DC * parameters[964])) + (DD * parameters[965]);
            let HO = ((parameters[970] + (DB * parameters[971])) + (DC * parameters[972])) + (DD * parameters[973]);
            let HP = ((parameters[974] + (DB * parameters[975])) + (DC * parameters[976])) + (DD * parameters[977]);
            let HQ = ((parameters[978] + (DB * parameters[979])) + (DC * parameters[980])) + (DD * parameters[981]);
            let HR = ((parameters[982] + (DB * parameters[983])) + (DC * parameters[984])) + (DD * parameters[985]);
            let HS = ((parameters[986] + (DB * parameters[987])) + (DC * parameters[988])) + (DD * parameters[989]);
            let HT = ((parameters[990] + (DB * parameters[991])) + (DC * parameters[992])) + (DD * parameters[993]);
            let HU = ((parameters[994] + (DB * parameters[995])) + (DC * parameters[996])) + (DD * parameters[997]);
            let HV = ((parameters[998] + (DB * parameters[999])) + (DC * parameters[1000])) + (DD * parameters[1001]);
            let HW = ((parameters[1002] + (DB * parameters[1003])) + (DC * parameters[1004])) + (DD * parameters[1005]);
            let HX = ((parameters[1006] + (DB * parameters[1007])) + (DC * parameters[1008])) + (DD * parameters[1009]);
            let HY = ((parameters[1010] + (DB * parameters[1011])) + (DC * parameters[1012])) + (DD * parameters[1013]);
            let HZ = ((parameters[1017] + (DB * parameters[1018])) + (DC * parameters[1019])) + (DD * parameters[1020]);
            let IA = ((parameters[1021] + (DB * parameters[1022])) + (DC * parameters[1023])) + (DD * parameters[1024]);
            let IB = ((parameters[1029] + (DB * parameters[1030])) + (DC * parameters[1031])) + (DD * parameters[1032]);
            let IC = ((parameters[1025] + (DB * parameters[1026])) + (DC * parameters[1027])) + (DD * parameters[1028]);
            let ID = ((parameters[1033] + (DB * parameters[1034])) + (DC * parameters[1035])) + (DD * parameters[1036]);
            let IE = ((parameters[1037] + (DB * parameters[1038])) + (DC * parameters[1039])) + (DD * parameters[1040]);
            let IF = ((parameters[1069] + (DB * parameters[1070])) + (DC * parameters[1071])) + (DD * parameters[1072]);
            let IG = ((parameters[1073] + (DB * parameters[1074])) + (DC * parameters[1075])) + (DD * parameters[1076]);
            let IH = ((parameters[1077] + (DB * parameters[1078])) + (DC * parameters[1079])) + (DD * parameters[1080]);
            let II = ((parameters[1081] + (DB * parameters[1082])) + (DC * parameters[1083])) + (DD * parameters[1084]);
            let IJ = ((parameters[1085] + (DB * parameters[1086])) + (DC * parameters[1087])) + (DD * parameters[1088]);
            let IK = ((parameters[1089] + (DB * parameters[1090])) + (DC * parameters[1091])) + (DD * parameters[1092]);
            let IM = ((IL + (DB * parameters[787])) + (DC * parameters[788])) + (DD * parameters[789]);
            let IN = ((parameters[794] + (DB * parameters[795])) + (DC * parameters[796])) + (DD * parameters[797]);
            let IO = ((parameters[790] + (DB * parameters[791])) + (DC * parameters[792])) + (DD * parameters[793]);
            let IP = if parameters[44] != A { 1.0 } else { 0.0 };
            let JO;
            let JZ;
            let KM;
            let KU;
            let LA;
            let LF;
            let LM;
            let LW;
            let MC;
            let MI;
            let MN;
            let MT;
            let OX;
            let BHD;
            if IP != 0.0 {
                let IQ = ((parameters[229] + (DB * parameters[230])) + (DC * parameters[231])) + (DD * parameters[232]);
                let IR = ((parameters[175] + (DB * parameters[176])) + (DC * parameters[177])) + (DD * parameters[178]);
                let IS = ((parameters[279] + (DB * parameters[280])) + (DC * parameters[281])) + (DD * parameters[282]);
                let IT = ((parameters[294] + (DB * parameters[295])) + (DC * parameters[296])) + (DD * parameters[297]);
                let IU = ((parameters[314] + (DB * parameters[315])) + (DC * parameters[316])) + (DD * parameters[317]);
                let IV = ((parameters[322] + (DB * parameters[323])) + (DC * parameters[324])) + (DD * parameters[325]);
                let IW = ((parameters[336] + (DB * parameters[337])) + (DC * parameters[338])) + (DD * parameters[339]);
                let IX = ((parameters[346] + (DB * parameters[347])) + (DC * parameters[348])) + (DD * parameters[349]);
                let IY = ((parameters[466] + (DB * parameters[467])) + (DC * parameters[468])) + (DD * parameters[469]);
                let IZ = ((parameters[249] + (DB * parameters[250])) + (DC * parameters[251])) + (DD * parameters[252]);
                let JA = ((parameters[426] + (DB * parameters[427])) + (DC * parameters[428])) + (DD * parameters[429]);
                let JB = ((parameters[440] + (DB * parameters[441])) + (DC * parameters[442])) + (DD * parameters[443]);
                let JC = ((parameters[525] + (DB * parameters[526])) + (DC * parameters[527])) + (DD * parameters[528]);
                let JD = ((parameters[529] + (DB * parameters[530])) + (DC * parameters[531])) + (DD * parameters[532]);
                JO = IQ;
                JZ = IS;
                KM = IT;
                KU = IU;
                LA = IW;
                LF = IR;
                LM = IY;
                LW = IX;
                MC = IZ;
                MI = JA;
                MN = JB;
                MT = JC;
                OX = IV;
                BHD = JD;
            } else {
                JO = A;
                JZ = A;
                KM = A;
                KU = A;
                LA = A;
                LF = A;
                LM = A;
                LW = A;
                MC = A;
                MI = A;
                MN = A;
                MT = A;
                OX = A;
                BHD = A;
            }
            let JH = DH * ((B + ((parameters[81] * (if ((BQ.powf(JE)) - (BU.powf(JE))) >= A { ((BQ.powf(JE)) - (BU.powf(JE))) } else { A })) + (parameters[83] * (if ((BQ.powf(JF)) - (BU.powf(JF))) >= A { ((BQ.powf(JF)) - (BU.powf(JF))) } else { A })))) + ((parameters[85] * (if ((BR.powf(JG)) - (BV.powf(JG))) >= A { ((BR.powf(JG)) - (BV.powf(JG))) } else { A })) + (parameters[87] * (BW.powf(parameters[88])))));
            let JK = DL * ((B + (parameters[214] * (if ((BQ.powf(JI)) - (BU.powf(JI))) >= A { ((BQ.powf(JI)) - (BU.powf(JI))) } else { A }))) + ((parameters[216] * (if ((BR.powf(JJ)) - (BV.powf(JJ))) >= A { ((BR.powf(JJ)) - (BV.powf(JJ))) } else { A })) + (parameters[218] * (BW.powf(parameters[219])))));
            let JM = B + (parameters[224] * (if ((BQ.powf(JL)) - (BU.powf(JL))) >= A { ((BQ.powf(JL)) - (BU.powf(JL))) } else { A }));
            let JN = DM * JM;
            let PR = if IP != 0.0 {
                let JP = JO * JM;
                JP
            } else {
                JO
            };
            let JR = DN * (B + (parameters[234] * (if ((BQ.powf(JQ)) - (BU.powf(JQ))) >= A { ((BQ.powf(JQ)) - (BU.powf(JQ))) } else { A })));
            let JS = parameters[34] * EB;
            let JT = if parameters[50] != B { 1.0 } else { 0.0 };
            let PW;
            let BEL;
            if JT != 0.0 {
                let JV = if JU > A { 1.0 } else { 0.0 };
                let PX;
                let BEM;
                if JV != 0.0 {
                    let JX = B - (JW * (if ((BQ.powf(JU)) - (BU.powf(JU))) >= A { ((BQ.powf(JU)) - (BU.powf(JU))) } else { A }));
                    let JY = JS * JX;
                    let BEN = if IP != 0.0 {
                        let KA = JZ * JX;
                        KA
                    } else {
                        JZ
                    };
                    PX = JY;
                    BEM = BEN;
                } else {
                    let KB = B - JW;
                    let KC = JS * KB;
                    let BEO = if IP != 0.0 {
                        let KD = JZ * KB;
                        KD
                    } else {
                        JZ
                    };
                    PX = KC;
                    BEM = BEO;
                }
                PW = PX;
                BEL = BEM;
            } else {
                let KE = -AU;
                let KF = (B - (parameters[269] * (rspice_limited_exp((KE / parameters[270]))))) - (parameters[271] * (rspice_limited_exp((KE / parameters[272]))));
                let KG = JS * KF;
                let BEP = if IP != 0.0 {
                    let KH = JZ * KF;
                    KH
                } else {
                    JZ
                };
                PW = KG;
                BEL = BEP;
            }
            let KK = (B + (parameters[285] * (if ((BQ.powf(KI)) - (BU.powf(KI))) >= A { ((BQ.powf(KI)) - (BU.powf(KI))) } else { A }))) + ((parameters[287] * (if ((BR.powf(KJ)) - (BV.powf(KJ))) >= A { ((BR.powf(KJ)) - (BV.powf(KJ))) } else { A })) + (parameters[289] * (BW.powf(parameters[290]))));
            let KL = EC * KK;
            let BEU = if IP != 0.0 {
                let KN = KM * KK;
                KN
            } else {
                KM
            };
            let KQ = EE * ((B + (parameters[302] * (if ((BQ.powf(KO)) - (BU.powf(KO))) >= A { ((BQ.powf(KO)) - (BU.powf(KO))) } else { A }))) + ((parameters[304] * (if ((BR.powf(KP)) - (BV.powf(KP))) >= A { ((BR.powf(KP)) - (BV.powf(KP))) } else { A })) + (parameters[306] * (BW.powf(parameters[307])))));
            let KS = B + (parameters[309] * (if ((BQ.powf(KR)) - (BU.powf(KR))) >= A { ((BQ.powf(KR)) - (BU.powf(KR))) } else { A }));
            let KT = ED * KS;
            let BFD = if IP != 0.0 {
                let KV = KU * KS;
                KV
            } else {
                KU
            };
            let KY = (B + (parameters[327] * (if ((BQ.powf(KW)) - (BU.powf(KW))) >= A { ((BQ.powf(KW)) - (BU.powf(KW))) } else { A }))) + ((parameters[329] * (if ((BR.powf(KX)) - (BV.powf(KX))) >= A { ((BR.powf(KX)) - (BV.powf(KX))) } else { A })) + (parameters[331] * (BW.powf(parameters[332]))));
            let KZ = EG * KY;
            let BFA = if IP != 0.0 {
                let LB = LA * KY;
                LB
            } else {
                LA
            };
            let LD = if ((BQ.powf(LC)) - (BU.powf(LC))) >= A { ((BQ.powf(LC)) - (BU.powf(LC))) } else { A };
            let LE = DY * LD;
            let BDA = if IP != 0.0 {
                let LG = LF * LD;
                LG
            } else {
                LF
            };
            let LI = DZ * (if ((BQ.powf(LH)) - (BU.powf(LH))) >= A { ((BQ.powf(LH)) - (BU.powf(LH))) } else { A });
            let LK = B + (parameters[461] * (if ((BQ.powf(LJ)) - (BU.powf(LJ))) >= A { ((BQ.powf(LJ)) - (BU.powf(LJ))) } else { A }));
            let LL = ET * LK;
            let BZB = if IP != 0.0 {
                let LN = LM * LK;
                LN
            } else {
                LM
            };
            let LQ = if (EA * (B + (parameters[257] * (if ((BQ.powf(LO)) - (BU.powf(LO))) >= A { ((BQ.powf(LO)) - (BU.powf(LO))) } else { A })))) <= LP { (EA * (B + (parameters[257] * (if ((BQ.powf(LO)) - (BU.powf(LO))) >= A { ((BQ.powf(LO)) - (BU.powf(LO))) } else { A })))) } else { LP };
            let LS = EZ * (B + (parameters[479] * (if ((BQ.powf(LR)) - (BU.powf(LR))) >= A { ((BQ.powf(LR)) - (BU.powf(LR))) } else { A })));
            let LU = B + (parameters[341] * (if ((BQ.powf(LT)) - (BU.powf(LT))) >= A { ((BQ.powf(LT)) - (BU.powf(LT))) } else { A }));
            let LV = if (EH * LU) >= A { (EH * LU) } else { A };
            let BZD = if IP != 0.0 {
                let LX = if (LW * LU) >= A { (LW * LU) } else { A };
                LX
            } else {
                LW
            };
            let MA = (B + (parameters[243] * (if ((BQ.powf(LY)) - (BU.powf(LY))) >= A { ((BQ.powf(LY)) - (BU.powf(LY))) } else { A }))) + ((parameters[245] * (if ((BR.powf(LZ)) - (BV.powf(LZ))) >= A { ((BR.powf(LZ)) - (BV.powf(LZ))) } else { A })) + (parameters[247] * (BW.powf(parameters[248]))));
            let MB = FB * MA;
            let BFS = if IP != 0.0 {
                let MD = MC * MA;
                MD
            } else {
                MC
            };
            let MF = B + (parameters[423] * (if ((BQ.powf(ME)) - (BU.powf(ME))) >= A { ((BQ.powf(ME)) - (BU.powf(ME))) } else { A }));
            let MH = if (FC * MF) >= MG { (FC * MF) } else { MG };
            let BZF = if IP != 0.0 {
                let MJ = if (MI * MF) >= MG { (MI * MF) } else { MG };
                MJ
            } else {
                MI
            };
            let ML = B + (parameters[438] * (if ((BQ.powf(MK)) - (BU.powf(MK))) >= A { ((BQ.powf(MK)) - (BU.powf(MK))) } else { A }));
            let MM = ES * ML;
            let BGM = if IP != 0.0 {
                let MO = MN * ML;
                MO
            } else {
                MN
            };
            let MR = (B + (parameters[485] * (if ((BQ.powf(MP)) - (BU.powf(MP))) >= A { ((BQ.powf(MP)) - (BU.powf(MP))) } else { A }))) + (parameters[487] * (if ((BR.powf(MQ)) - (BV.powf(MQ))) >= A { ((BR.powf(MQ)) - (BV.powf(MQ))) } else { A }));
            let MS = FG * MR;
            let BZX = if IP != 0.0 {
                let MU = MT * MR;
                MU
            } else {
                MT
            };
            let MW = FH * (B + (parameters[495] * (if ((BR.powf(MV)) - (BV.powf(MV))) >= A { ((BR.powf(MV)) - (BV.powf(MV))) } else { A })));
            let MY = parameters[518] * (B + (parameters[519] * (if ((BR.powf(MX)) - (BV.powf(MX))) >= A { ((BR.powf(MX)) - (BV.powf(MX))) } else { A })));
            let NA = parameters[521] * (B + (parameters[522] * (if ((BR.powf(MZ)) - (BV.powf(MZ))) >= A { ((BR.powf(MZ)) - (BV.powf(MZ))) } else { A })));
            let NB = FL * ((B + (parameters[631] * BQ)) + (parameters[632] * BR));
            let NC = FP * ((B + (parameters[649] * BQ)) + (parameters[650] * BR));
            let ND = GO * ((B + (parameters[557] * BQ)) + (parameters[558] * BR));
            let NE = GR * ((B + (parameters[559] * BQ)) + (parameters[560] * BR));
            let NF = GU * ((B + (parameters[561] * BQ)) + (parameters[562] * BR));
            let NG = parameters[556] * (B + (parameters[563] * BQ));
            let NK = DI * ((B + ((parameters[93] * (if ((BS.powf(NH)) - (BU.powf(NH))) >= A { ((BS.powf(NH)) - (BU.powf(NH))) } else { A })) + (parameters[95] * (if ((BS.powf(NI)) - (BU.powf(NI))) >= A { ((BS.powf(NI)) - (BU.powf(NI))) } else { A })))) + ((parameters[97] * (if ((BT.powf(NJ)) - (BV.powf(NJ))) >= A { ((BT.powf(NJ)) - (BV.powf(NJ))) } else { A })) + (parameters[99] * ((BT * BS).powf(parameters[100])))));
            let NN = DE * ((B + (parameters[120] * (if ((BS.powf(NL)) - (BU.powf(NL))) >= A { ((BS.powf(NL)) - (BU.powf(NL))) } else { A }))) + ((parameters[122] * (if ((BT.powf(NM)) - (BV.powf(NM))) >= A { ((BT.powf(NM)) - (BV.powf(NM))) } else { A })) + (parameters[124] * (BW.powf(parameters[125])))));
            let NQ = DF * ((B + (parameters[130] * (if ((BS.powf(NO)) - (BU.powf(NO))) >= A { ((BS.powf(NO)) - (BU.powf(NO))) } else { A }))) + ((parameters[132] * (if ((BT.powf(NP)) - (BV.powf(NP))) >= A { ((BT.powf(NP)) - (BV.powf(NP))) } else { A })) + (parameters[134] * (BW.powf(parameters[135])))));
            let NT = FD * ((B + (parameters[263] * (if ((BS.powf(NR)) - (BU.powf(NR))) >= A { ((BS.powf(NR)) - (BU.powf(NR))) } else { A }))) + ((parameters[265] * (if ((BR.powf(NS)) - (BV.powf(NS))) >= A { ((BR.powf(NS)) - (BV.powf(NS))) } else { A })) + (parameters[267] * (BW.powf(parameters[268])))));
            let NV = if (EI * (B + (parameters[352] * (if ((BS.powf(NU)) - (BU.powf(NU))) >= A { ((BS.powf(NU)) - (BU.powf(NU))) } else { A })))) >= A { (EI * (B + (parameters[352] * (if ((BS.powf(NU)) - (BU.powf(NU))) >= A { ((BS.powf(NU)) - (BU.powf(NU))) } else { A })))) } else { A };
            let NY = DV * ((B + (parameters[186] * (if ((BQ.powf(NW)) - (BU.powf(NW))) >= A { ((BQ.powf(NW)) - (BU.powf(NW))) } else { A }))) + ((parameters[188] * (if ((BR.powf(NX)) - (BV.powf(NX))) >= A { ((BR.powf(NX)) - (BV.powf(NX))) } else { A })) + (parameters[190] * (BW.powf(parameters[191])))));
            let OB = DU * ((B + (parameters[196] * (if ((BQ.powf(NZ)) - (BU.powf(NZ))) >= A { ((BQ.powf(NZ)) - (BU.powf(NZ))) } else { A }))) + ((parameters[198] * (if ((BR.powf(OA)) - (BV.powf(OA))) >= A { ((BR.powf(OA)) - (BV.powf(OA))) } else { A })) + (parameters[200] * (BW.powf(parameters[201])))));
            let OD = EM * (B + (parameters[383] * (if ((BQ.powf(OC)) - (BU.powf(OC))) >= A { ((BQ.powf(OC)) - (BU.powf(OC))) } else { A })));
            let OE = FT * (B + (BQ * parameters[828]));
            let OF = FU * (B + (BQ * parameters[833]));
            let OG = FW * (B + (BQ * parameters[842]));
            let OH = GA * (B + (BQ * parameters[860]));
            let OI = GB * (B + (BQ * parameters[866]));
            if OJ != 0.0 {
                let OM = if (if OK == A { 1.0 } else { 0.0 }) != 0.0 || (if OL == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if OM != 0.0 {
                } else {
                }
            } else {
            }
            let OO = if ON == B { 1.0 } else { 0.0 };
            let AXI;
            let AXK;
            let AXN;
            if OO != 0.0 {
                let OQ = EJ * (B + (parameters[397] * (if ((BQ.powf(OP)) - (BU.powf(OP))) >= A { ((BQ.powf(OP)) - (BU.powf(OP))) } else { A })));
                let OS = EK * (B + (parameters[407] * (if ((BQ.powf(OR)) - (BU.powf(OR))) >= A { ((BQ.powf(OR)) - (BU.powf(OR))) } else { A })));
                AXI = OQ;
                AXK = OS;
                AXN = EQ;
            } else {
                let OU = EQ * (B + (parameters[414] * (if ((BQ.powf(OT)) - (BU.powf(OT))) >= A { ((BQ.powf(OT)) - (BU.powf(OT))) } else { A })));
                AXI = EJ;
                AXK = EK;
                AXN = OU;
            }
            let OV = if EF < B { 1.0 } else { 0.0 };
            let QD;
            if OV != 0.0 {
                QD = B;
            } else {
                let OW = if EF > AT { 1.0 } else { 0.0 };
                let QE = if OW != 0.0 {
                    AT
                } else {
                    EF
                };
                QD = QE;
            }
            let BFF;
            if IP != 0.0 {
                let OY = if OX < B { 1.0 } else { 0.0 };
                let BFG;
                if OY != 0.0 {
                    BFG = B;
                } else {
                    let OZ = if OX > AT { 1.0 } else { 0.0 };
                    let BFH = if OZ != 0.0 {
                        AT
                    } else {
                        OX
                    };
                    BFG = BFH;
                }
                BFF = BFG;
            } else {
                BFF = OX;
            }
            let PA = if FN < A { 1.0 } else { 0.0 };
            if PA != 0.0 {
            } else {
            }
            let PB = if FR < A { 1.0 } else { 0.0 };
            if PB != 0.0 {
            } else {
            }
            let PC = if FF <= A { 1.0 } else { 0.0 };
            if PC != 0.0 {
            } else {
            }
            let PD = if FE <= A { 1.0 } else { 0.0 };
            if PD != 0.0 {
            } else {
            }
            let PE = if EX < A { 1.0 } else { 0.0 };
            if PE != 0.0 {
            } else {
            }
            let PF = if DK < A { 1.0 } else { 0.0 };
            if PF != 0.0 {
            } else {
            }
            let PG = if JK < A { 1.0 } else { 0.0 };
            if PG != 0.0 {
            } else {
            }
            let PH = if NY < A { 1.0 } else { 0.0 };
            if PH != 0.0 {
            } else {
            }
            let PI = if DG <= A { 1.0 } else { 0.0 };
            if PI != 0.0 {
            } else {
            }
            let PJ = if JH <= A { 1.0 } else { 0.0 };
            if PJ != 0.0 {
            } else {
            }
            let PK = if NK <= A { 1.0 } else { 0.0 };
            if PK != 0.0 {
            } else {
            }
            let PL = if parameters[47] != A { 1.0 } else { 0.0 };
            if PL != 0.0 {
                let PM = if GJ <= A { 1.0 } else { 0.0 };
                if PM != 0.0 {
                } else {
                }
                let PN = if GN <= A { 1.0 } else { 0.0 };
                if PN != 0.0 {
                } else {
                }
            } else {
            }
            let PO = if parameters[46] != A { 1.0 } else { 0.0 };
            if PO != 0.0 {
                let PP = if GX <= A { 1.0 } else { 0.0 };
                if PP != 0.0 {
                } else {
                }
            } else {
            }
            let PQ = if JN < A { 1.0 } else { 0.0 };
            if PQ != 0.0 {
            } else {
            }
            if IP != 0.0 {
                let PS = if PR < A { 1.0 } else { 0.0 };
                if PS != 0.0 {
                } else {
                }
            } else {
            }
            let PT = if GY < A { 1.0 } else { 0.0 };
            let DED = if PT != 0.0 {
                A
            } else {
                GY
            };
            let PU = if GZ < A { 1.0 } else { 0.0 };
            let DER = if PU != 0.0 {
                A
            } else {
                GZ
            };
            let PV = if HJ < A { 1.0 } else { 0.0 };
            let BIC = if PV != 0.0 {
                A
            } else {
                HJ
            };
            let PY = if PW <= A { 1.0 } else { 0.0 };
            let BDI = if PY != 0.0 {
                PZ
            } else {
                PW
            };
            let QA = if KL < A { 1.0 } else { 0.0 };
            let BDP = if QA != 0.0 {
                A
            } else {
                KL
            };
            let QB = if KQ < A { 1.0 } else { 0.0 };
            let BEI = if QB != 0.0 {
                A
            } else {
                KQ
            };
            let QC = if KT < A { 1.0 } else { 0.0 };
            let BDY = if QC != 0.0 {
                A
            } else {
                KT
            };
            let QF = if QD < A { 1.0 } else { 0.0 };
            let BEB = if QF != 0.0 {
                A
            } else {
                QD
            };
            let QG = if MY < A { 1.0 } else { 0.0 };
            let DAH = if QG != 0.0 {
                A
            } else {
                MY
            };
            let QH = if parameters[1065] == B { 1.0 } else { 0.0 };
            let DLZ;
            let DMU;
            if QH != 0.0 {
                let QJ = if AU > QI { 1.0 } else { 0.0 };
                let QM;
                let DMA;
                if QJ != 0.0 {
                    let QK = AU - QI;
                    QM = QK;
                    DMA = QI;
                } else {
                    QM = AU;
                    DMA = AU;
                }
                let QN = if QL >= (QM / AT) { 1.0 } else { 0.0 };
                let DMV = if QN != 0.0 {
                    A
                } else {
                    QL
                };
                DLZ = DMA;
                DMU = DMV;
            } else {
                DLZ = DMB;
                DMU = DMW;
            }
            let QP = parameters[695] - QO;
            let QR = parameters[697] - QO;
            let AHV;
            let AIR;
            let AQX;
            let AWP;
            let AWX;
            let BNL;
            let BOA;
            if QS != 0.0 {
                let QU = QT * parameters[3];
                AHV = A;
                AIR = A;
                AQX = A;
                AWP = A;
                AWX = QU;
                BNL = A;
                BOA = A;
            } else {
                let QW = if (if QV > A { 1.0 } else { 0.0 }) != 0.0 && (if QT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AHW;
                let AIS;
                let AQY;
                let AWQ;
                let AWY;
                let BNM;
                let BOB;
                if QW != 0.0 {
                    let QZ = if QX < QY { 1.0 } else { 0.0 };
                    let RU;
                    let ST;
                    let AAX;
                    let BNN;
                    let BOC;
                    if QZ != 0.0 {
                        let RA = if (V % AT) != A { 1.0 } else { 0.0 };
                        let RH;
                        let RL;
                        let RV;
                        let SU;
                        if RA != 0.0 {
                            let RB = AT * (if ((V - B) / AT) >= A { ((V - B) / AT) } else { A });
                            RH = RB;
                            RL = RB;
                            RV = B;
                            SU = B;
                        } else {
                            let RD = if RC == B { 1.0 } else { 0.0 };
                            let RI;
                            let RM;
                            let RW;
                            let SV;
                            if RD != 0.0 {
                                let RE = AT * (if ((V / AT) - B) >= A { ((V / AT) - B) } else { A });
                                RI = V;
                                RM = RE;
                                RW = A;
                                SV = AT;
                            } else {
                                let RF = AT * (if ((V / AT) - B) >= A { ((V / AT) - B) } else { A });
                                RI = RF;
                                RM = V;
                                RW = AT;
                                SV = A;
                            }
                            RH = RI;
                            RL = RM;
                            RV = RW;
                            SU = SV;
                        }
                        let AAY;
                        if RG != 0.0 {
                            let RJ = if RH == A { 1.0 } else { 0.0 };
                            let AAZ = if RJ != 0.0 {
                                A
                            } else {
                                let RK = (QT * QP) / (AY * RH);
                                RK
                            };
                            AAY = AAZ;
                        } else {
                            let RN = if RL == A { 1.0 } else { 0.0 };
                            let ABA = if RN != 0.0 {
                                A
                            } else {
                                let RO = (QT * QP) / (AY * RL);
                                RO
                            };
                            AAY = ABA;
                        }
                        RU = RV;
                        ST = SU;
                        AAX = AAY;
                        BNN = RH;
                        BOC = RL;
                    } else {
                        RU = A;
                        ST = A;
                        AAX = A;
                        BNN = A;
                        BOC = A;
                    }
                    let RP = if QX == A { 1.0 } else { 0.0 };
                    let AAW;
                    let ABQ;
                    if RP != 0.0 {
                        let ABR;
                        if RQ != 0.0 {
                            let ABS;
                            if RR != 0.0 {
                                let RT = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ABT;
                                if RT != 0.0 {
                                    let RX = if RU == A { 1.0 } else { 0.0 };
                                    let ABU = if RX != 0.0 {
                                        A
                                    } else {
                                        let RY = (QT * QP) / (AY * RU);
                                        RY
                                    };
                                    ABT = ABU;
                                } else {
                                    let SC = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ABV;
                                    if SC != 0.0 {
                                        let SD = QP + QQ;
                                        let SE = if SD == A { 1.0 } else { 0.0 };
                                        if SE != 0.0 {
                                        } else {
                                        }
                                        let SF = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || SE != 0.0 { 1.0 } else { 0.0 };
                                        let ABW = if SF != 0.0 {
                                            A
                                        } else {
                                            let SG = (QT * AY) / ((RZ * RU) * SD);
                                            SG
                                        };
                                        ABV = ABW;
                                    } else {
                                        ABV = A;
                                    }
                                    ABT = ABV;
                                }
                                ABS = ABT;
                            } else {
                                let SI = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ABX;
                                if SI != 0.0 {
                                    let SJ = if RU == A { 1.0 } else { 0.0 };
                                    let ABY = if SJ != 0.0 {
                                        A
                                    } else {
                                        let SK = (QT * QP) / (AY * RU);
                                        SK
                                    };
                                    ABX = ABY;
                                } else {
                                    let SM = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ABZ;
                                    if SM != 0.0 {
                                        let SN = QP + QQ;
                                        let SO = if SN == A { 1.0 } else { 0.0 };
                                        if SO != 0.0 {
                                        } else {
                                        }
                                        let SP = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || SO != 0.0 { 1.0 } else { 0.0 };
                                        let ACA = if SP != 0.0 {
                                            A
                                        } else {
                                            let SQ = (QT * AY) / ((RZ * RU) * SN);
                                            SQ
                                        };
                                        ABZ = ACA;
                                    } else {
                                        ABZ = A;
                                    }
                                    ABX = ABZ;
                                }
                                ABS = ABX;
                            }
                            ABR = ABS;
                        } else {
                            let ACB;
                            if SR != 0.0 {
                                let SS = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ACC;
                                if SS != 0.0 {
                                    let SW = if ST == A { 1.0 } else { 0.0 };
                                    let ACD = if SW != 0.0 {
                                        A
                                    } else {
                                        let SX = (QT * QP) / (AY * ST);
                                        SX
                                    };
                                    ACC = ACD;
                                } else {
                                    let SY = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ACE;
                                    if SY != 0.0 {
                                        let SZ = QP + QQ;
                                        let TA = if SZ == A { 1.0 } else { 0.0 };
                                        if TA != 0.0 {
                                        } else {
                                        }
                                        let TB = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || TA != 0.0 { 1.0 } else { 0.0 };
                                        let ACF = if TB != 0.0 {
                                            A
                                        } else {
                                            let TC = (QT * AY) / ((RZ * ST) * SZ);
                                            TC
                                        };
                                        ACE = ACF;
                                    } else {
                                        ACE = A;
                                    }
                                    ACC = ACE;
                                }
                                ACB = ACC;
                            } else {
                                let TD = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ACG;
                                if TD != 0.0 {
                                    let TE = if ST == A { 1.0 } else { 0.0 };
                                    let ACH = if TE != 0.0 {
                                        A
                                    } else {
                                        let TF = (QT * QP) / (AY * ST);
                                        TF
                                    };
                                    ACG = ACH;
                                } else {
                                    let TG = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ACI;
                                    if TG != 0.0 {
                                        let TH = QP + QQ;
                                        let TI = if TH == A { 1.0 } else { 0.0 };
                                        if TI != 0.0 {
                                        } else {
                                        }
                                        let TJ = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || TI != 0.0 { 1.0 } else { 0.0 };
                                        let ACJ = if TJ != 0.0 {
                                            A
                                        } else {
                                            let TK = (QT * AY) / ((RZ * ST) * TH);
                                            TK
                                        };
                                        ACI = ACJ;
                                    } else {
                                        ACI = A;
                                    }
                                    ACG = ACI;
                                }
                                ACB = ACG;
                            }
                            ABR = ACB;
                        }
                        AAW = AAX;
                        ABQ = ABR;
                    } else {
                        let TL = if QX == B { 1.0 } else { 0.0 };
                        let ABB;
                        let ACK;
                        if TL != 0.0 {
                            let ACL;
                            if TM != 0.0 {
                                let ACM;
                                if TN != 0.0 {
                                    let TO = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ACN;
                                    if TO != 0.0 {
                                        let TP = if RU == A { 1.0 } else { 0.0 };
                                        let ACO = if TP != 0.0 {
                                            A
                                        } else {
                                            let TQ = (QT * QP) / (AY * RU);
                                            TQ
                                        };
                                        ACN = ACO;
                                    } else {
                                        let TR = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ACP;
                                        if TR != 0.0 {
                                            let TS = QP + QQ;
                                            let TT = if TS == A { 1.0 } else { 0.0 };
                                            if TT != 0.0 {
                                            } else {
                                            }
                                            let TU = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || TT != 0.0 { 1.0 } else { 0.0 };
                                            let ACQ = if TU != 0.0 {
                                                A
                                            } else {
                                                let TV = (QT * AY) / ((RZ * RU) * TS);
                                                TV
                                            };
                                            ACP = ACQ;
                                        } else {
                                            ACP = A;
                                        }
                                        ACN = ACP;
                                    }
                                    ACM = ACN;
                                } else {
                                    let TW = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ACR;
                                    if TW != 0.0 {
                                        let TX = if RU == A { 1.0 } else { 0.0 };
                                        let ACS = if TX != 0.0 {
                                            A
                                        } else {
                                            let TY = (QT * QP) / (AY * RU);
                                            TY
                                        };
                                        ACR = ACS;
                                    } else {
                                        let TZ = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ACT;
                                        if TZ != 0.0 {
                                            let UA = QP + QQ;
                                            let UB = if UA == A { 1.0 } else { 0.0 };
                                            if UB != 0.0 {
                                            } else {
                                            }
                                            let UC = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || UB != 0.0 { 1.0 } else { 0.0 };
                                            let ACU = if UC != 0.0 {
                                                A
                                            } else {
                                                let UD = (QT * AY) / ((RZ * RU) * UA);
                                                UD
                                            };
                                            ACT = ACU;
                                        } else {
                                            ACT = A;
                                        }
                                        ACR = ACT;
                                    }
                                    ACM = ACR;
                                }
                                ACL = ACM;
                            } else {
                                let ACV;
                                if UE != 0.0 {
                                    let UF = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ACW;
                                    if UF != 0.0 {
                                        let UG = if ST == A { 1.0 } else { 0.0 };
                                        let ACX = if UG != 0.0 {
                                            A
                                        } else {
                                            let UH = (QT * QP) / (AY * ST);
                                            UH
                                        };
                                        ACW = ACX;
                                    } else {
                                        let UI = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ACY;
                                        if UI != 0.0 {
                                            let UJ = if QP == A { 1.0 } else { 0.0 };
                                            if UJ != 0.0 {
                                            } else {
                                            }
                                            let UK = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || UJ != 0.0 { 1.0 } else { 0.0 };
                                            let ACZ = if UK != 0.0 {
                                                A
                                            } else {
                                                let UL = (QT * AY) / ((SB * ST) * QP);
                                                UL
                                            };
                                            ACY = ACZ;
                                        } else {
                                            ACY = A;
                                        }
                                        ACW = ACY;
                                    }
                                    ACV = ACW;
                                } else {
                                    let UM = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ADA;
                                    if UM != 0.0 {
                                        let UN = if ST == A { 1.0 } else { 0.0 };
                                        let ADB = if UN != 0.0 {
                                            A
                                        } else {
                                            let UO = (QT * QP) / (AY * ST);
                                            UO
                                        };
                                        ADA = ADB;
                                    } else {
                                        let UP = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ADC;
                                        if UP != 0.0 {
                                            let UQ = if QP == A { 1.0 } else { 0.0 };
                                            if UQ != 0.0 {
                                            } else {
                                            }
                                            let UR = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || UQ != 0.0 { 1.0 } else { 0.0 };
                                            let ADD = if UR != 0.0 {
                                                A
                                            } else {
                                                let US = (QT * AY) / ((SB * ST) * QP);
                                                US
                                            };
                                            ADC = ADD;
                                        } else {
                                            ADC = A;
                                        }
                                        ADA = ADC;
                                    }
                                    ACV = ADA;
                                }
                                ACL = ACV;
                            }
                            ABB = AAX;
                            ACK = ACL;
                        } else {
                            let UT = if QX == AT { 1.0 } else { 0.0 };
                            let ABC;
                            let ADE;
                            if UT != 0.0 {
                                let ADF;
                                if UU != 0.0 {
                                    let ADG;
                                    if UV != 0.0 {
                                        let UW = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ADH;
                                        if UW != 0.0 {
                                            let UX = if RU == A { 1.0 } else { 0.0 };
                                            let ADI = if UX != 0.0 {
                                                A
                                            } else {
                                                let UY = (QT * QP) / (AY * RU);
                                                UY
                                            };
                                            ADH = ADI;
                                        } else {
                                            let UZ = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ADJ;
                                            if UZ != 0.0 {
                                                let VA = if QP == A { 1.0 } else { 0.0 };
                                                if VA != 0.0 {
                                                } else {
                                                }
                                                let VB = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || VA != 0.0 { 1.0 } else { 0.0 };
                                                let ADK = if VB != 0.0 {
                                                    A
                                                } else {
                                                    let VC = (QT * AY) / ((SB * RU) * QP);
                                                    VC
                                                };
                                                ADJ = ADK;
                                            } else {
                                                ADJ = A;
                                            }
                                            ADH = ADJ;
                                        }
                                        ADG = ADH;
                                    } else {
                                        let VD = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ADL;
                                        if VD != 0.0 {
                                            let VE = if RU == A { 1.0 } else { 0.0 };
                                            let ADM = if VE != 0.0 {
                                                A
                                            } else {
                                                let VF = (QT * QP) / (AY * RU);
                                                VF
                                            };
                                            ADL = ADM;
                                        } else {
                                            let VG = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ADN;
                                            if VG != 0.0 {
                                                let VH = if QP == A { 1.0 } else { 0.0 };
                                                if VH != 0.0 {
                                                } else {
                                                }
                                                let VI = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || VH != 0.0 { 1.0 } else { 0.0 };
                                                let ADO = if VI != 0.0 {
                                                    A
                                                } else {
                                                    let VJ = (QT * AY) / ((SB * RU) * QP);
                                                    VJ
                                                };
                                                ADN = ADO;
                                            } else {
                                                ADN = A;
                                            }
                                            ADL = ADN;
                                        }
                                        ADG = ADL;
                                    }
                                    ADF = ADG;
                                } else {
                                    let ADP;
                                    if VK != 0.0 {
                                        let VL = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ADQ;
                                        if VL != 0.0 {
                                            let VM = if ST == A { 1.0 } else { 0.0 };
                                            let ADR = if VM != 0.0 {
                                                A
                                            } else {
                                                let VN = (QT * QP) / (AY * ST);
                                                VN
                                            };
                                            ADQ = ADR;
                                        } else {
                                            let VO = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ADS;
                                            if VO != 0.0 {
                                                let VP = QP + QQ;
                                                let VQ = if VP == A { 1.0 } else { 0.0 };
                                                if VQ != 0.0 {
                                                } else {
                                                }
                                                let VR = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || VQ != 0.0 { 1.0 } else { 0.0 };
                                                let ADT = if VR != 0.0 {
                                                    A
                                                } else {
                                                    let VS = (QT * AY) / ((RZ * ST) * VP);
                                                    VS
                                                };
                                                ADS = ADT;
                                            } else {
                                                ADS = A;
                                            }
                                            ADQ = ADS;
                                        }
                                        ADP = ADQ;
                                    } else {
                                        let VT = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ADU;
                                        if VT != 0.0 {
                                            let VU = if ST == A { 1.0 } else { 0.0 };
                                            let ADV = if VU != 0.0 {
                                                A
                                            } else {
                                                let VV = (QT * QP) / (AY * ST);
                                                VV
                                            };
                                            ADU = ADV;
                                        } else {
                                            let VW = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ADW;
                                            if VW != 0.0 {
                                                let VX = QP + QQ;
                                                let VY = if VX == A { 1.0 } else { 0.0 };
                                                if VY != 0.0 {
                                                } else {
                                                }
                                                let VZ = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || VY != 0.0 { 1.0 } else { 0.0 };
                                                let ADX = if VZ != 0.0 {
                                                    A
                                                } else {
                                                    let WA = (QT * AY) / ((RZ * ST) * VX);
                                                    WA
                                                };
                                                ADW = ADX;
                                            } else {
                                                ADW = A;
                                            }
                                            ADU = ADW;
                                        }
                                        ADP = ADU;
                                    }
                                    ADF = ADP;
                                }
                                ABC = AAX;
                                ADE = ADF;
                            } else {
                                let WB = if QX == RZ { 1.0 } else { 0.0 };
                                let ABD;
                                let ADY;
                                if WB != 0.0 {
                                    let ADZ;
                                    if WC != 0.0 {
                                        let AEA;
                                        if WD != 0.0 {
                                            let WE = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AEB;
                                            if WE != 0.0 {
                                                let WF = if RU == A { 1.0 } else { 0.0 };
                                                let AEC = if WF != 0.0 {
                                                    A
                                                } else {
                                                    let WG = (QT * QP) / (AY * RU);
                                                    WG
                                                };
                                                AEB = AEC;
                                            } else {
                                                let WH = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AED;
                                                if WH != 0.0 {
                                                    let WI = if QP == A { 1.0 } else { 0.0 };
                                                    if WI != 0.0 {
                                                    } else {
                                                    }
                                                    let WJ = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || WI != 0.0 { 1.0 } else { 0.0 };
                                                    let AEE = if WJ != 0.0 {
                                                        A
                                                    } else {
                                                        let WK = (QT * AY) / ((SB * RU) * QP);
                                                        WK
                                                    };
                                                    AED = AEE;
                                                } else {
                                                    AED = A;
                                                }
                                                AEB = AED;
                                            }
                                            AEA = AEB;
                                        } else {
                                            let WL = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AEF;
                                            if WL != 0.0 {
                                                let WM = if RU == A { 1.0 } else { 0.0 };
                                                let AEG = if WM != 0.0 {
                                                    A
                                                } else {
                                                    let WN = (QT * QP) / (AY * RU);
                                                    WN
                                                };
                                                AEF = AEG;
                                            } else {
                                                let WO = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AEH;
                                                if WO != 0.0 {
                                                    let WP = if QP == A { 1.0 } else { 0.0 };
                                                    if WP != 0.0 {
                                                    } else {
                                                    }
                                                    let WQ = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || WP != 0.0 { 1.0 } else { 0.0 };
                                                    let AEI = if WQ != 0.0 {
                                                        A
                                                    } else {
                                                        let WR = (QT * AY) / ((SB * RU) * QP);
                                                        WR
                                                    };
                                                    AEH = AEI;
                                                } else {
                                                    AEH = A;
                                                }
                                                AEF = AEH;
                                            }
                                            AEA = AEF;
                                        }
                                        ADZ = AEA;
                                    } else {
                                        let AEJ;
                                        if WS != 0.0 {
                                            let WT = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AEK;
                                            if WT != 0.0 {
                                                let WU = if ST == A { 1.0 } else { 0.0 };
                                                let AEL = if WU != 0.0 {
                                                    A
                                                } else {
                                                    let WV = (QT * QP) / (AY * ST);
                                                    WV
                                                };
                                                AEK = AEL;
                                            } else {
                                                let WW = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AEM;
                                                if WW != 0.0 {
                                                    let WX = if QP == A { 1.0 } else { 0.0 };
                                                    if WX != 0.0 {
                                                    } else {
                                                    }
                                                    let WY = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || WX != 0.0 { 1.0 } else { 0.0 };
                                                    let AEN = if WY != 0.0 {
                                                        A
                                                    } else {
                                                        let WZ = (QT * AY) / ((SB * ST) * QP);
                                                        WZ
                                                    };
                                                    AEM = AEN;
                                                } else {
                                                    AEM = A;
                                                }
                                                AEK = AEM;
                                            }
                                            AEJ = AEK;
                                        } else {
                                            let XA = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AEO;
                                            if XA != 0.0 {
                                                let XB = if ST == A { 1.0 } else { 0.0 };
                                                let AEP = if XB != 0.0 {
                                                    A
                                                } else {
                                                    let XC = (QT * QP) / (AY * ST);
                                                    XC
                                                };
                                                AEO = AEP;
                                            } else {
                                                let XD = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AEQ;
                                                if XD != 0.0 {
                                                    let XE = if QP == A { 1.0 } else { 0.0 };
                                                    if XE != 0.0 {
                                                    } else {
                                                    }
                                                    let XF = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || XE != 0.0 { 1.0 } else { 0.0 };
                                                    let AER = if XF != 0.0 {
                                                        A
                                                    } else {
                                                        let XG = (QT * AY) / ((SB * ST) * QP);
                                                        XG
                                                    };
                                                    AEQ = AER;
                                                } else {
                                                    AEQ = A;
                                                }
                                                AEO = AEQ;
                                            }
                                            AEJ = AEO;
                                        }
                                        ADZ = AEJ;
                                    }
                                    ABD = AAX;
                                    ADY = ADZ;
                                } else {
                                    let XH = if QX == SA { 1.0 } else { 0.0 };
                                    let ABE;
                                    let AES;
                                    if XH != 0.0 {
                                        let AET;
                                        if XI != 0.0 {
                                            let AEU;
                                            if XJ != 0.0 {
                                                let XK = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AEV;
                                                if XK != 0.0 {
                                                    let XL = if RU == A { 1.0 } else { 0.0 };
                                                    let AEW = if XL != 0.0 {
                                                        A
                                                    } else {
                                                        let XM = (QT * QP) / (AY * RU);
                                                        XM
                                                    };
                                                    AEV = AEW;
                                                } else {
                                                    let XN = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AEX;
                                                    if XN != 0.0 {
                                                        let XO = QP + QQ;
                                                        let XP = if XO == A { 1.0 } else { 0.0 };
                                                        if XP != 0.0 {
                                                        } else {
                                                        }
                                                        let XQ = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || XP != 0.0 { 1.0 } else { 0.0 };
                                                        let AEY = if XQ != 0.0 {
                                                            A
                                                        } else {
                                                            let XR = (QT * AY) / ((RZ * RU) * XO);
                                                            XR
                                                        };
                                                        AEX = AEY;
                                                    } else {
                                                        AEX = A;
                                                    }
                                                    AEV = AEX;
                                                }
                                                AEU = AEV;
                                            } else {
                                                let XS = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AEZ;
                                                if XS != 0.0 {
                                                    let XT = if RU == A { 1.0 } else { 0.0 };
                                                    let AFA = if XT != 0.0 {
                                                        A
                                                    } else {
                                                        let XU = (QT * QP) / (AY * RU);
                                                        XU
                                                    };
                                                    AEZ = AFA;
                                                } else {
                                                    let XV = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AFB;
                                                    if XV != 0.0 {
                                                        let XW = QP + QQ;
                                                        let XX = if XW == A { 1.0 } else { 0.0 };
                                                        if XX != 0.0 {
                                                        } else {
                                                        }
                                                        let XY = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || XX != 0.0 { 1.0 } else { 0.0 };
                                                        let AFC = if XY != 0.0 {
                                                            A
                                                        } else {
                                                            let XZ = (QT * AY) / ((RZ * RU) * XW);
                                                            XZ
                                                        };
                                                        AFB = AFC;
                                                    } else {
                                                        AFB = A;
                                                    }
                                                    AEZ = AFB;
                                                }
                                                AEU = AEZ;
                                            }
                                            AET = AEU;
                                        } else {
                                            let YA = (QT * QR) / AY;
                                            AET = YA;
                                        }
                                        ABE = AAX;
                                        AES = AET;
                                    } else {
                                        let YB = if QX == RS { 1.0 } else { 0.0 };
                                        let ABF;
                                        let AFD;
                                        if YB != 0.0 {
                                            let AFE;
                                            if YC != 0.0 {
                                                let AFF;
                                                if YD != 0.0 {
                                                    let YE = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AFG;
                                                    if YE != 0.0 {
                                                        let YF = if RU == A { 1.0 } else { 0.0 };
                                                        let AFH = if YF != 0.0 {
                                                            A
                                                        } else {
                                                            let YG = (QT * QP) / (AY * RU);
                                                            YG
                                                        };
                                                        AFG = AFH;
                                                    } else {
                                                        let YH = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AFI;
                                                        if YH != 0.0 {
                                                            let YI = if QP == A { 1.0 } else { 0.0 };
                                                            if YI != 0.0 {
                                                            } else {
                                                            }
                                                            let YJ = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || YI != 0.0 { 1.0 } else { 0.0 };
                                                            let AFJ = if YJ != 0.0 {
                                                                A
                                                            } else {
                                                                let YK = (QT * AY) / ((SB * RU) * QP);
                                                                YK
                                                            };
                                                            AFI = AFJ;
                                                        } else {
                                                            AFI = A;
                                                        }
                                                        AFG = AFI;
                                                    }
                                                    AFF = AFG;
                                                } else {
                                                    let YL = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AFK;
                                                    if YL != 0.0 {
                                                        let YM = if RU == A { 1.0 } else { 0.0 };
                                                        let AFL = if YM != 0.0 {
                                                            A
                                                        } else {
                                                            let YN = (QT * QP) / (AY * RU);
                                                            YN
                                                        };
                                                        AFK = AFL;
                                                    } else {
                                                        let YO = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AFM;
                                                        if YO != 0.0 {
                                                            let YP = if QP == A { 1.0 } else { 0.0 };
                                                            if YP != 0.0 {
                                                            } else {
                                                            }
                                                            let YQ = if (if RU == A { 1.0 } else { 0.0 }) != 0.0 || YP != 0.0 { 1.0 } else { 0.0 };
                                                            let AFN = if YQ != 0.0 {
                                                                A
                                                            } else {
                                                                let YR = (QT * AY) / ((SB * RU) * QP);
                                                                YR
                                                            };
                                                            AFM = AFN;
                                                        } else {
                                                            AFM = A;
                                                        }
                                                        AFK = AFM;
                                                    }
                                                    AFF = AFK;
                                                }
                                                AFE = AFF;
                                            } else {
                                                let YS = if ST == A { 1.0 } else { 0.0 };
                                                let AFO = if YS != 0.0 {
                                                    A
                                                } else {
                                                    let YT = (QT * QR) / (AY * ST);
                                                    YT
                                                };
                                                AFE = AFO;
                                            }
                                            ABF = AAX;
                                            AFD = AFE;
                                        } else {
                                            let YU = if QX == SB { 1.0 } else { 0.0 };
                                            let ABG;
                                            let AFP;
                                            if YU != 0.0 {
                                                let AFQ;
                                                if YV != 0.0 {
                                                    let YW = (QT * QR) / AY;
                                                    AFQ = YW;
                                                } else {
                                                    let AFR;
                                                    if YX != 0.0 {
                                                        let YY = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AFS;
                                                        if YY != 0.0 {
                                                            let YZ = if ST == A { 1.0 } else { 0.0 };
                                                            let AFT = if YZ != 0.0 {
                                                                A
                                                            } else {
                                                                let ZA = (QT * QP) / (AY * ST);
                                                                ZA
                                                            };
                                                            AFS = AFT;
                                                        } else {
                                                            let ZB = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AFU;
                                                            if ZB != 0.0 {
                                                                let ZC = QP + QQ;
                                                                let ZD = if ZC == A { 1.0 } else { 0.0 };
                                                                if ZD != 0.0 {
                                                                } else {
                                                                }
                                                                let ZE = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || ZD != 0.0 { 1.0 } else { 0.0 };
                                                                let AFV = if ZE != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ZF = (QT * AY) / ((RZ * ST) * ZC);
                                                                    ZF
                                                                };
                                                                AFU = AFV;
                                                            } else {
                                                                AFU = A;
                                                            }
                                                            AFS = AFU;
                                                        }
                                                        AFR = AFS;
                                                    } else {
                                                        let ZG = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AFW;
                                                        if ZG != 0.0 {
                                                            let ZH = if ST == A { 1.0 } else { 0.0 };
                                                            let AFX = if ZH != 0.0 {
                                                                A
                                                            } else {
                                                                let ZI = (QT * QP) / (AY * ST);
                                                                ZI
                                                            };
                                                            AFW = AFX;
                                                        } else {
                                                            let ZJ = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AFY;
                                                            if ZJ != 0.0 {
                                                                let ZK = QP + QQ;
                                                                let ZL = if ZK == A { 1.0 } else { 0.0 };
                                                                if ZL != 0.0 {
                                                                } else {
                                                                }
                                                                let ZM = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || ZL != 0.0 { 1.0 } else { 0.0 };
                                                                let AFZ = if ZM != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ZN = (QT * AY) / ((RZ * ST) * ZK);
                                                                    ZN
                                                                };
                                                                AFY = AFZ;
                                                            } else {
                                                                AFY = A;
                                                            }
                                                            AFW = AFY;
                                                        }
                                                        AFR = AFW;
                                                    }
                                                    AFQ = AFR;
                                                }
                                                ABG = AAX;
                                                AFP = AFQ;
                                            } else {
                                                let ZO = if QX == SH { 1.0 } else { 0.0 };
                                                let ABH;
                                                let AGA;
                                                if ZO != 0.0 {
                                                    let AGB;
                                                    if ZP != 0.0 {
                                                        let ZQ = if RU == A { 1.0 } else { 0.0 };
                                                        let AGC = if ZQ != 0.0 {
                                                            A
                                                        } else {
                                                            let ZR = (QT * QR) / (AY * RU);
                                                            ZR
                                                        };
                                                        AGB = AGC;
                                                    } else {
                                                        let AGD;
                                                        if ZS != 0.0 {
                                                            let ZT = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AGE;
                                                            if ZT != 0.0 {
                                                                let ZU = if ST == A { 1.0 } else { 0.0 };
                                                                let AGF = if ZU != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ZV = (QT * QP) / (AY * ST);
                                                                    ZV
                                                                };
                                                                AGE = AGF;
                                                            } else {
                                                                let ZW = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AGG;
                                                                if ZW != 0.0 {
                                                                    let ZX = if QP == A { 1.0 } else { 0.0 };
                                                                    if ZX != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let ZY = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || ZX != 0.0 { 1.0 } else { 0.0 };
                                                                    let AGH = if ZY != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ZZ = (QT * AY) / ((SB * ST) * QP);
                                                                        ZZ
                                                                    };
                                                                    AGG = AGH;
                                                                } else {
                                                                    AGG = A;
                                                                }
                                                                AGE = AGG;
                                                            }
                                                            AGD = AGE;
                                                        } else {
                                                            let AAA = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AGI;
                                                            if AAA != 0.0 {
                                                                let AAB = if ST == A { 1.0 } else { 0.0 };
                                                                let AGJ = if AAB != 0.0 {
                                                                    A
                                                                } else {
                                                                    let AAC = (QT * QP) / (AY * ST);
                                                                    AAC
                                                                };
                                                                AGI = AGJ;
                                                            } else {
                                                                let AAD = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AGK;
                                                                if AAD != 0.0 {
                                                                    let AAE = if QP == A { 1.0 } else { 0.0 };
                                                                    if AAE != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let AAF = if (if ST == A { 1.0 } else { 0.0 }) != 0.0 || AAE != 0.0 { 1.0 } else { 0.0 };
                                                                    let AGL = if AAF != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let AAG = (QT * AY) / ((SB * ST) * QP);
                                                                        AAG
                                                                    };
                                                                    AGK = AGL;
                                                                } else {
                                                                    AGK = A;
                                                                }
                                                                AGI = AGK;
                                                            }
                                                            AGD = AGI;
                                                        }
                                                        AGB = AGD;
                                                    }
                                                    ABH = AAX;
                                                    AGA = AGB;
                                                } else {
                                                    let AAH = if QX == SL { 1.0 } else { 0.0 };
                                                    let ABI;
                                                    let AGM;
                                                    if AAH != 0.0 {
                                                        let AAI = (QT * QR) / AY;
                                                        ABI = AAX;
                                                        AGM = AAI;
                                                    } else {
                                                        let AAJ = if QX == QY { 1.0 } else { 0.0 };
                                                        let ABJ;
                                                        let AGN;
                                                        if AAJ != 0.0 {
                                                            let ABK;
                                                            let AGO;
                                                            if AAK != 0.0 {
                                                                let AAL = ((LP * QT) * QP) / AY;
                                                                let AAM = if V == AT { 1.0 } else { 0.0 };
                                                                let ABL = if AAM != 0.0 {
                                                                    A
                                                                } else {
                                                                    let AAN = (QT * QP) / (AY * (V - AT));
                                                                    AAN
                                                                };
                                                                ABK = ABL;
                                                                AGO = AAL;
                                                            } else {
                                                                let AAO = (QT * QP) / (AY * V);
                                                                ABK = AAO;
                                                                AGO = A;
                                                            }
                                                            ABJ = ABK;
                                                            AGN = AGO;
                                                        } else {
                                                            let AAQ = if QX == AAP { 1.0 } else { 0.0 };
                                                            let ABM;
                                                            let AGP;
                                                            if AAQ != 0.0 {
                                                                let ABN;
                                                                let AGQ;
                                                                if AAR != 0.0 {
                                                                    let AAS = (QT * QP) / (AY * V);
                                                                    ABN = AAS;
                                                                    AGQ = A;
                                                                } else {
                                                                    let AAT = ((LP * QT) * QP) / AY;
                                                                    let AAU = if V == AT { 1.0 } else { 0.0 };
                                                                    let ABO = if AAU != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let AAV = (QT * QP) / (AY * (V - AT));
                                                                        AAV
                                                                    };
                                                                    ABN = ABO;
                                                                    AGQ = AAT;
                                                                }
                                                                ABM = ABN;
                                                                AGP = AGQ;
                                                            } else {
                                                                ABM = A;
                                                                AGP = A;
                                                            }
                                                            ABJ = ABM;
                                                            AGN = AGP;
                                                        }
                                                        ABI = ABJ;
                                                        AGM = AGN;
                                                    }
                                                    ABH = ABI;
                                                    AGA = AGM;
                                                }
                                                ABG = ABH;
                                                AFP = AGA;
                                            }
                                            ABF = ABG;
                                            AFD = AFP;
                                        }
                                        ABE = ABF;
                                        AES = AFD;
                                    }
                                    ABD = ABE;
                                    ADY = AES;
                                }
                                ABC = ABD;
                                ADE = ADY;
                            }
                            ABB = ABC;
                            ACK = ADE;
                        }
                        AAW = ABB;
                        ABQ = ACK;
                    }
                    let ABP = if AAW <= A { 1.0 } else { 0.0 };
                    let AGT;
                    if ABP != 0.0 {
                        AGT = ABQ;
                    } else {
                        let AGR = if ABQ <= A { 1.0 } else { 0.0 };
                        let AGU = if AGR != 0.0 {
                            AAW
                        } else {
                            let AGS = (AAW * ABQ) / (AAW + ABQ);
                            AGS
                        };
                        AGT = AGU;
                    }
                    let AGV = if AGT == A { 1.0 } else { 0.0 };
                    if AGV != 0.0 {
                    } else {
                    }
                    AHW = RU;
                    AIS = ST;
                    AQY = AAW;
                    AWQ = ABQ;
                    AWY = AGT;
                    BNM = BNN;
                    BOB = BOC;
                } else {
                    AHW = A;
                    AIS = A;
                    AQY = A;
                    AWQ = A;
                    AWY = A;
                    BNM = A;
                    BOB = A;
                }
                AHV = AHW;
                AIR = AIS;
                AQX = AQY;
                AWP = AWQ;
                AWX = AWY;
                BNL = BNM;
                BOA = BOB;
            }
            let AXB;
            let BNF;
            let BNK;
            let BNU;
            let BNZ;
            if AGW != 0.0 {
                let AGX = QT * parameters[4];
                AXB = AGX;
                BNF = AHV;
                BNK = BNL;
                BNU = AIR;
                BNZ = BOA;
            } else {
                let AGY = if (if QV > A { 1.0 } else { 0.0 }) != 0.0 && (if QT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AXC;
                let BNG;
                let BNO;
                let BNV;
                let BOD;
                if AGY != 0.0 {
                    let AGZ = if QX < QY { 1.0 } else { 0.0 };
                    let AHS;
                    let AIO;
                    let AQT;
                    let BNP;
                    let BOE;
                    if AGZ != 0.0 {
                        let AHA = if (V % AT) != A { 1.0 } else { 0.0 };
                        let AHG;
                        let AHK;
                        let AHT;
                        let AIP;
                        if AHA != 0.0 {
                            let AHB = AT * (if ((V - B) / AT) >= A { ((V - B) / AT) } else { A });
                            AHG = AHB;
                            AHK = AHB;
                            AHT = B;
                            AIP = B;
                        } else {
                            let AHC = if RC == B { 1.0 } else { 0.0 };
                            let AHH;
                            let AHL;
                            let AHU;
                            let AIQ;
                            if AHC != 0.0 {
                                let AHD = AT * (if ((V / AT) - B) >= A { ((V / AT) - B) } else { A });
                                AHH = V;
                                AHL = AHD;
                                AHU = A;
                                AIQ = AT;
                            } else {
                                let AHE = AT * (if ((V / AT) - B) >= A { ((V / AT) - B) } else { A });
                                AHH = AHE;
                                AHL = V;
                                AHU = AT;
                                AIQ = A;
                            }
                            AHG = AHH;
                            AHK = AHL;
                            AHT = AHU;
                            AIP = AIQ;
                        }
                        let AQU;
                        if AHF != 0.0 {
                            let AHI = if AHG == A { 1.0 } else { 0.0 };
                            let AQV = if AHI != 0.0 {
                                A
                            } else {
                                let AHJ = (QT * QP) / (AY * AHG);
                                AHJ
                            };
                            AQU = AQV;
                        } else {
                            let AHM = if AHK == A { 1.0 } else { 0.0 };
                            let AQW = if AHM != 0.0 {
                                A
                            } else {
                                let AHN = (QT * QP) / (AY * AHK);
                                AHN
                            };
                            AQU = AQW;
                        }
                        AHS = AHT;
                        AIO = AIP;
                        AQT = AQU;
                        BNP = AHG;
                        BOE = AHK;
                    } else {
                        AHS = AHV;
                        AIO = AIR;
                        AQT = AQX;
                        BNP = BNL;
                        BOE = BOA;
                    }
                    let AHO = if QX == A { 1.0 } else { 0.0 };
                    let AQS;
                    let ARO;
                    if AHO != 0.0 {
                        let ARP;
                        if AHP != 0.0 {
                            let ARQ;
                            if AHQ != 0.0 {
                                let AHR = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ARR;
                                if AHR != 0.0 {
                                    let AHX = if AHS == A { 1.0 } else { 0.0 };
                                    let ARS = if AHX != 0.0 {
                                        A
                                    } else {
                                        let AHY = (QT * QP) / (AY * AHS);
                                        AHY
                                    };
                                    ARR = ARS;
                                } else {
                                    let AHZ = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ART;
                                    if AHZ != 0.0 {
                                        let AIA = QP + QQ;
                                        let AIB = if AIA == A { 1.0 } else { 0.0 };
                                        if AIB != 0.0 {
                                        } else {
                                        }
                                        let AIC = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AIB != 0.0 { 1.0 } else { 0.0 };
                                        let ARU = if AIC != 0.0 {
                                            A
                                        } else {
                                            let AID = (QT * AY) / ((RZ * AHS) * AIA);
                                            AID
                                        };
                                        ART = ARU;
                                    } else {
                                        ART = A;
                                    }
                                    ARR = ART;
                                }
                                ARQ = ARR;
                            } else {
                                let AIE = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ARV;
                                if AIE != 0.0 {
                                    let AIF = if AHS == A { 1.0 } else { 0.0 };
                                    let ARW = if AIF != 0.0 {
                                        A
                                    } else {
                                        let AIG = (QT * QP) / (AY * AHS);
                                        AIG
                                    };
                                    ARV = ARW;
                                } else {
                                    let AIH = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ARX;
                                    if AIH != 0.0 {
                                        let AII = QP + QQ;
                                        let AIJ = if AII == A { 1.0 } else { 0.0 };
                                        if AIJ != 0.0 {
                                        } else {
                                        }
                                        let AIK = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AIJ != 0.0 { 1.0 } else { 0.0 };
                                        let ARY = if AIK != 0.0 {
                                            A
                                        } else {
                                            let AIL = (QT * AY) / ((RZ * AHS) * AII);
                                            AIL
                                        };
                                        ARX = ARY;
                                    } else {
                                        ARX = A;
                                    }
                                    ARV = ARX;
                                }
                                ARQ = ARV;
                            }
                            ARP = ARQ;
                        } else {
                            let ARZ;
                            if AIM != 0.0 {
                                let AIN = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ASA;
                                if AIN != 0.0 {
                                    let AIT = if AIO == A { 1.0 } else { 0.0 };
                                    let ASB = if AIT != 0.0 {
                                        A
                                    } else {
                                        let AIU = (QT * QP) / (AY * AIO);
                                        AIU
                                    };
                                    ASA = ASB;
                                } else {
                                    let AIV = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ASC;
                                    if AIV != 0.0 {
                                        let AIW = QP + QQ;
                                        let AIX = if AIW == A { 1.0 } else { 0.0 };
                                        if AIX != 0.0 {
                                        } else {
                                        }
                                        let AIY = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || AIX != 0.0 { 1.0 } else { 0.0 };
                                        let ASD = if AIY != 0.0 {
                                            A
                                        } else {
                                            let AIZ = (QT * AY) / ((RZ * AIO) * AIW);
                                            AIZ
                                        };
                                        ASC = ASD;
                                    } else {
                                        ASC = A;
                                    }
                                    ASA = ASC;
                                }
                                ARZ = ASA;
                            } else {
                                let AJA = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ASE;
                                if AJA != 0.0 {
                                    let AJB = if AIO == A { 1.0 } else { 0.0 };
                                    let ASF = if AJB != 0.0 {
                                        A
                                    } else {
                                        let AJC = (QT * QP) / (AY * AIO);
                                        AJC
                                    };
                                    ASE = ASF;
                                } else {
                                    let AJD = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ASG;
                                    if AJD != 0.0 {
                                        let AJE = QP + QQ;
                                        let AJF = if AJE == A { 1.0 } else { 0.0 };
                                        if AJF != 0.0 {
                                        } else {
                                        }
                                        let AJG = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || AJF != 0.0 { 1.0 } else { 0.0 };
                                        let ASH = if AJG != 0.0 {
                                            A
                                        } else {
                                            let AJH = (QT * AY) / ((RZ * AIO) * AJE);
                                            AJH
                                        };
                                        ASG = ASH;
                                    } else {
                                        ASG = A;
                                    }
                                    ASE = ASG;
                                }
                                ARZ = ASE;
                            }
                            ARP = ARZ;
                        }
                        AQS = AQT;
                        ARO = ARP;
                    } else {
                        let AJI = if QX == B { 1.0 } else { 0.0 };
                        let AQZ;
                        let ASI;
                        if AJI != 0.0 {
                            let ASJ;
                            if AJJ != 0.0 {
                                let ASK;
                                if AJK != 0.0 {
                                    let AJL = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ASL;
                                    if AJL != 0.0 {
                                        let AJM = if AHS == A { 1.0 } else { 0.0 };
                                        let ASM = if AJM != 0.0 {
                                            A
                                        } else {
                                            let AJN = (QT * QP) / (AY * AHS);
                                            AJN
                                        };
                                        ASL = ASM;
                                    } else {
                                        let AJO = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ASN;
                                        if AJO != 0.0 {
                                            let AJP = QP + QQ;
                                            let AJQ = if AJP == A { 1.0 } else { 0.0 };
                                            if AJQ != 0.0 {
                                            } else {
                                            }
                                            let AJR = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AJQ != 0.0 { 1.0 } else { 0.0 };
                                            let ASO = if AJR != 0.0 {
                                                A
                                            } else {
                                                let AJS = (QT * AY) / ((RZ * AHS) * AJP);
                                                AJS
                                            };
                                            ASN = ASO;
                                        } else {
                                            ASN = A;
                                        }
                                        ASL = ASN;
                                    }
                                    ASK = ASL;
                                } else {
                                    let AJT = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ASP;
                                    if AJT != 0.0 {
                                        let AJU = if AHS == A { 1.0 } else { 0.0 };
                                        let ASQ = if AJU != 0.0 {
                                            A
                                        } else {
                                            let AJV = (QT * QP) / (AY * AHS);
                                            AJV
                                        };
                                        ASP = ASQ;
                                    } else {
                                        let AJW = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ASR;
                                        if AJW != 0.0 {
                                            let AJX = QP + QQ;
                                            let AJY = if AJX == A { 1.0 } else { 0.0 };
                                            if AJY != 0.0 {
                                            } else {
                                            }
                                            let AJZ = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AJY != 0.0 { 1.0 } else { 0.0 };
                                            let ASS = if AJZ != 0.0 {
                                                A
                                            } else {
                                                let AKA = (QT * AY) / ((RZ * AHS) * AJX);
                                                AKA
                                            };
                                            ASR = ASS;
                                        } else {
                                            ASR = A;
                                        }
                                        ASP = ASR;
                                    }
                                    ASK = ASP;
                                }
                                ASJ = ASK;
                            } else {
                                let AST;
                                if AKB != 0.0 {
                                    let AKC = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ASU;
                                    if AKC != 0.0 {
                                        let AKD = if AIO == A { 1.0 } else { 0.0 };
                                        let ASV = if AKD != 0.0 {
                                            A
                                        } else {
                                            let AKE = (QT * QP) / (AY * AIO);
                                            AKE
                                        };
                                        ASU = ASV;
                                    } else {
                                        let AKF = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ASW;
                                        if AKF != 0.0 {
                                            let AKG = if QP == A { 1.0 } else { 0.0 };
                                            if AKG != 0.0 {
                                            } else {
                                            }
                                            let AKH = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || AKG != 0.0 { 1.0 } else { 0.0 };
                                            let ASX = if AKH != 0.0 {
                                                A
                                            } else {
                                                let AKI = (QT * AY) / ((SB * AIO) * QP);
                                                AKI
                                            };
                                            ASW = ASX;
                                        } else {
                                            ASW = A;
                                        }
                                        ASU = ASW;
                                    }
                                    AST = ASU;
                                } else {
                                    let AKJ = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let ASY;
                                    if AKJ != 0.0 {
                                        let AKK = if AIO == A { 1.0 } else { 0.0 };
                                        let ASZ = if AKK != 0.0 {
                                            A
                                        } else {
                                            let AKL = (QT * QP) / (AY * AIO);
                                            AKL
                                        };
                                        ASY = ASZ;
                                    } else {
                                        let AKM = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ATA;
                                        if AKM != 0.0 {
                                            let AKN = if QP == A { 1.0 } else { 0.0 };
                                            if AKN != 0.0 {
                                            } else {
                                            }
                                            let AKO = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || AKN != 0.0 { 1.0 } else { 0.0 };
                                            let ATB = if AKO != 0.0 {
                                                A
                                            } else {
                                                let AKP = (QT * AY) / ((SB * AIO) * QP);
                                                AKP
                                            };
                                            ATA = ATB;
                                        } else {
                                            ATA = A;
                                        }
                                        ASY = ATA;
                                    }
                                    AST = ASY;
                                }
                                ASJ = AST;
                            }
                            AQZ = AQT;
                            ASI = ASJ;
                        } else {
                            let AKQ = if QX == AT { 1.0 } else { 0.0 };
                            let ARA;
                            let ATC;
                            if AKQ != 0.0 {
                                let ATD;
                                if AKR != 0.0 {
                                    let ATE;
                                    if AKS != 0.0 {
                                        let AKT = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ATF;
                                        if AKT != 0.0 {
                                            let AKU = if AHS == A { 1.0 } else { 0.0 };
                                            let ATG = if AKU != 0.0 {
                                                A
                                            } else {
                                                let AKV = (QT * QP) / (AY * AHS);
                                                AKV
                                            };
                                            ATF = ATG;
                                        } else {
                                            let AKW = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ATH;
                                            if AKW != 0.0 {
                                                let AKX = if QP == A { 1.0 } else { 0.0 };
                                                if AKX != 0.0 {
                                                } else {
                                                }
                                                let AKY = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AKX != 0.0 { 1.0 } else { 0.0 };
                                                let ATI = if AKY != 0.0 {
                                                    A
                                                } else {
                                                    let AKZ = (QT * AY) / ((SB * AHS) * QP);
                                                    AKZ
                                                };
                                                ATH = ATI;
                                            } else {
                                                ATH = A;
                                            }
                                            ATF = ATH;
                                        }
                                        ATE = ATF;
                                    } else {
                                        let ALA = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ATJ;
                                        if ALA != 0.0 {
                                            let ALB = if AHS == A { 1.0 } else { 0.0 };
                                            let ATK = if ALB != 0.0 {
                                                A
                                            } else {
                                                let ALC = (QT * QP) / (AY * AHS);
                                                ALC
                                            };
                                            ATJ = ATK;
                                        } else {
                                            let ALD = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ATL;
                                            if ALD != 0.0 {
                                                let ALE = if QP == A { 1.0 } else { 0.0 };
                                                if ALE != 0.0 {
                                                } else {
                                                }
                                                let ALF = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || ALE != 0.0 { 1.0 } else { 0.0 };
                                                let ATM = if ALF != 0.0 {
                                                    A
                                                } else {
                                                    let ALG = (QT * AY) / ((SB * AHS) * QP);
                                                    ALG
                                                };
                                                ATL = ATM;
                                            } else {
                                                ATL = A;
                                            }
                                            ATJ = ATL;
                                        }
                                        ATE = ATJ;
                                    }
                                    ATD = ATE;
                                } else {
                                    let ATN;
                                    if ALH != 0.0 {
                                        let ALI = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ATO;
                                        if ALI != 0.0 {
                                            let ALJ = if AIO == A { 1.0 } else { 0.0 };
                                            let ATP = if ALJ != 0.0 {
                                                A
                                            } else {
                                                let ALK = (QT * QP) / (AY * AIO);
                                                ALK
                                            };
                                            ATO = ATP;
                                        } else {
                                            let ALL = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ATQ;
                                            if ALL != 0.0 {
                                                let ALM = QP + QQ;
                                                let ALN = if ALM == A { 1.0 } else { 0.0 };
                                                if ALN != 0.0 {
                                                } else {
                                                }
                                                let ALO = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || ALN != 0.0 { 1.0 } else { 0.0 };
                                                let ATR = if ALO != 0.0 {
                                                    A
                                                } else {
                                                    let ALP = (QT * AY) / ((RZ * AIO) * ALM);
                                                    ALP
                                                };
                                                ATQ = ATR;
                                            } else {
                                                ATQ = A;
                                            }
                                            ATO = ATQ;
                                        }
                                        ATN = ATO;
                                    } else {
                                        let ALQ = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let ATS;
                                        if ALQ != 0.0 {
                                            let ALR = if AIO == A { 1.0 } else { 0.0 };
                                            let ATT = if ALR != 0.0 {
                                                A
                                            } else {
                                                let ALS = (QT * QP) / (AY * AIO);
                                                ALS
                                            };
                                            ATS = ATT;
                                        } else {
                                            let ALT = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ATU;
                                            if ALT != 0.0 {
                                                let ALU = QP + QQ;
                                                let ALV = if ALU == A { 1.0 } else { 0.0 };
                                                if ALV != 0.0 {
                                                } else {
                                                }
                                                let ALW = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || ALV != 0.0 { 1.0 } else { 0.0 };
                                                let ATV = if ALW != 0.0 {
                                                    A
                                                } else {
                                                    let ALX = (QT * AY) / ((RZ * AIO) * ALU);
                                                    ALX
                                                };
                                                ATU = ATV;
                                            } else {
                                                ATU = A;
                                            }
                                            ATS = ATU;
                                        }
                                        ATN = ATS;
                                    }
                                    ATD = ATN;
                                }
                                ARA = AQT;
                                ATC = ATD;
                            } else {
                                let ALY = if QX == RZ { 1.0 } else { 0.0 };
                                let ARB;
                                let ATW;
                                if ALY != 0.0 {
                                    let ATX;
                                    if ALZ != 0.0 {
                                        let ATY;
                                        if AMA != 0.0 {
                                            let AMB = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let ATZ;
                                            if AMB != 0.0 {
                                                let AMC = if AHS == A { 1.0 } else { 0.0 };
                                                let AUA = if AMC != 0.0 {
                                                    A
                                                } else {
                                                    let AMD = (QT * QP) / (AY * AHS);
                                                    AMD
                                                };
                                                ATZ = AUA;
                                            } else {
                                                let AME = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AUB;
                                                if AME != 0.0 {
                                                    let AMF = if QP == A { 1.0 } else { 0.0 };
                                                    if AMF != 0.0 {
                                                    } else {
                                                    }
                                                    let AMG = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AMF != 0.0 { 1.0 } else { 0.0 };
                                                    let AUC = if AMG != 0.0 {
                                                        A
                                                    } else {
                                                        let AMH = (QT * AY) / ((SB * AHS) * QP);
                                                        AMH
                                                    };
                                                    AUB = AUC;
                                                } else {
                                                    AUB = A;
                                                }
                                                ATZ = AUB;
                                            }
                                            ATY = ATZ;
                                        } else {
                                            let AMI = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AUD;
                                            if AMI != 0.0 {
                                                let AMJ = if AHS == A { 1.0 } else { 0.0 };
                                                let AUE = if AMJ != 0.0 {
                                                    A
                                                } else {
                                                    let AMK = (QT * QP) / (AY * AHS);
                                                    AMK
                                                };
                                                AUD = AUE;
                                            } else {
                                                let AML = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AUF;
                                                if AML != 0.0 {
                                                    let AMM = if QP == A { 1.0 } else { 0.0 };
                                                    if AMM != 0.0 {
                                                    } else {
                                                    }
                                                    let AMN = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AMM != 0.0 { 1.0 } else { 0.0 };
                                                    let AUG = if AMN != 0.0 {
                                                        A
                                                    } else {
                                                        let AMO = (QT * AY) / ((SB * AHS) * QP);
                                                        AMO
                                                    };
                                                    AUF = AUG;
                                                } else {
                                                    AUF = A;
                                                }
                                                AUD = AUF;
                                            }
                                            ATY = AUD;
                                        }
                                        ATX = ATY;
                                    } else {
                                        let AUH;
                                        if AMP != 0.0 {
                                            let AMQ = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AUI;
                                            if AMQ != 0.0 {
                                                let AMR = if AIO == A { 1.0 } else { 0.0 };
                                                let AUJ = if AMR != 0.0 {
                                                    A
                                                } else {
                                                    let AMS = (QT * QP) / (AY * AIO);
                                                    AMS
                                                };
                                                AUI = AUJ;
                                            } else {
                                                let AMT = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AUK;
                                                if AMT != 0.0 {
                                                    let AMU = if QP == A { 1.0 } else { 0.0 };
                                                    if AMU != 0.0 {
                                                    } else {
                                                    }
                                                    let AMV = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || AMU != 0.0 { 1.0 } else { 0.0 };
                                                    let AUL = if AMV != 0.0 {
                                                        A
                                                    } else {
                                                        let AMW = (QT * AY) / ((SB * AIO) * QP);
                                                        AMW
                                                    };
                                                    AUK = AUL;
                                                } else {
                                                    AUK = A;
                                                }
                                                AUI = AUK;
                                            }
                                            AUH = AUI;
                                        } else {
                                            let AMX = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AUM;
                                            if AMX != 0.0 {
                                                let AMY = if AIO == A { 1.0 } else { 0.0 };
                                                let AUN = if AMY != 0.0 {
                                                    A
                                                } else {
                                                    let AMZ = (QT * QP) / (AY * AIO);
                                                    AMZ
                                                };
                                                AUM = AUN;
                                            } else {
                                                let ANA = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AUO;
                                                if ANA != 0.0 {
                                                    let ANB = if QP == A { 1.0 } else { 0.0 };
                                                    if ANB != 0.0 {
                                                    } else {
                                                    }
                                                    let ANC = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || ANB != 0.0 { 1.0 } else { 0.0 };
                                                    let AUP = if ANC != 0.0 {
                                                        A
                                                    } else {
                                                        let AND = (QT * AY) / ((SB * AIO) * QP);
                                                        AND
                                                    };
                                                    AUO = AUP;
                                                } else {
                                                    AUO = A;
                                                }
                                                AUM = AUO;
                                            }
                                            AUH = AUM;
                                        }
                                        ATX = AUH;
                                    }
                                    ARB = AQT;
                                    ATW = ATX;
                                } else {
                                    let ANE = if QX == SA { 1.0 } else { 0.0 };
                                    let ARC;
                                    let AUQ;
                                    if ANE != 0.0 {
                                        let AUR;
                                        if ANF != 0.0 {
                                            let AUS;
                                            if ANG != 0.0 {
                                                let ANH = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AUT;
                                                if ANH != 0.0 {
                                                    let ANI = if AHS == A { 1.0 } else { 0.0 };
                                                    let AUU = if ANI != 0.0 {
                                                        A
                                                    } else {
                                                        let ANJ = (QT * QP) / (AY * AHS);
                                                        ANJ
                                                    };
                                                    AUT = AUU;
                                                } else {
                                                    let ANK = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AUV;
                                                    if ANK != 0.0 {
                                                        let ANL = QP + QQ;
                                                        let ANM = if ANL == A { 1.0 } else { 0.0 };
                                                        if ANM != 0.0 {
                                                        } else {
                                                        }
                                                        let ANN = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || ANM != 0.0 { 1.0 } else { 0.0 };
                                                        let AUW = if ANN != 0.0 {
                                                            A
                                                        } else {
                                                            let ANO = (QT * AY) / ((RZ * AHS) * ANL);
                                                            ANO
                                                        };
                                                        AUV = AUW;
                                                    } else {
                                                        AUV = A;
                                                    }
                                                    AUT = AUV;
                                                }
                                                AUS = AUT;
                                            } else {
                                                let ANP = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AUX;
                                                if ANP != 0.0 {
                                                    let ANQ = if AHS == A { 1.0 } else { 0.0 };
                                                    let AUY = if ANQ != 0.0 {
                                                        A
                                                    } else {
                                                        let ANR = (QT * QP) / (AY * AHS);
                                                        ANR
                                                    };
                                                    AUX = AUY;
                                                } else {
                                                    let ANS = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AUZ;
                                                    if ANS != 0.0 {
                                                        let ANT = QP + QQ;
                                                        let ANU = if ANT == A { 1.0 } else { 0.0 };
                                                        if ANU != 0.0 {
                                                        } else {
                                                        }
                                                        let ANV = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || ANU != 0.0 { 1.0 } else { 0.0 };
                                                        let AVA = if ANV != 0.0 {
                                                            A
                                                        } else {
                                                            let ANW = (QT * AY) / ((RZ * AHS) * ANT);
                                                            ANW
                                                        };
                                                        AUZ = AVA;
                                                    } else {
                                                        AUZ = A;
                                                    }
                                                    AUX = AUZ;
                                                }
                                                AUS = AUX;
                                            }
                                            AUR = AUS;
                                        } else {
                                            let ANX = (QT * QR) / AY;
                                            AUR = ANX;
                                        }
                                        ARC = AQT;
                                        AUQ = AUR;
                                    } else {
                                        let ANY = if QX == RS { 1.0 } else { 0.0 };
                                        let ARD;
                                        let AVB;
                                        if ANY != 0.0 {
                                            let AVC;
                                            if ANZ != 0.0 {
                                                let AVD;
                                                if AOA != 0.0 {
                                                    let AOB = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AVE;
                                                    if AOB != 0.0 {
                                                        let AOC = if AHS == A { 1.0 } else { 0.0 };
                                                        let AVF = if AOC != 0.0 {
                                                            A
                                                        } else {
                                                            let AOD = (QT * QP) / (AY * AHS);
                                                            AOD
                                                        };
                                                        AVE = AVF;
                                                    } else {
                                                        let AOE = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AVG;
                                                        if AOE != 0.0 {
                                                            let AOF = if QP == A { 1.0 } else { 0.0 };
                                                            if AOF != 0.0 {
                                                            } else {
                                                            }
                                                            let AOG = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AOF != 0.0 { 1.0 } else { 0.0 };
                                                            let AVH = if AOG != 0.0 {
                                                                A
                                                            } else {
                                                                let AOH = (QT * AY) / ((SB * AHS) * QP);
                                                                AOH
                                                            };
                                                            AVG = AVH;
                                                        } else {
                                                            AVG = A;
                                                        }
                                                        AVE = AVG;
                                                    }
                                                    AVD = AVE;
                                                } else {
                                                    let AOI = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AVI;
                                                    if AOI != 0.0 {
                                                        let AOJ = if AHS == A { 1.0 } else { 0.0 };
                                                        let AVJ = if AOJ != 0.0 {
                                                            A
                                                        } else {
                                                            let AOK = (QT * QP) / (AY * AHS);
                                                            AOK
                                                        };
                                                        AVI = AVJ;
                                                    } else {
                                                        let AOL = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AVK;
                                                        if AOL != 0.0 {
                                                            let AOM = if QP == A { 1.0 } else { 0.0 };
                                                            if AOM != 0.0 {
                                                            } else {
                                                            }
                                                            let AON = if (if AHS == A { 1.0 } else { 0.0 }) != 0.0 || AOM != 0.0 { 1.0 } else { 0.0 };
                                                            let AVL = if AON != 0.0 {
                                                                A
                                                            } else {
                                                                let AOO = (QT * AY) / ((SB * AHS) * QP);
                                                                AOO
                                                            };
                                                            AVK = AVL;
                                                        } else {
                                                            AVK = A;
                                                        }
                                                        AVI = AVK;
                                                    }
                                                    AVD = AVI;
                                                }
                                                AVC = AVD;
                                            } else {
                                                let AOP = if AIO == A { 1.0 } else { 0.0 };
                                                let AVM = if AOP != 0.0 {
                                                    A
                                                } else {
                                                    let AOQ = (QT * QR) / (AY * AIO);
                                                    AOQ
                                                };
                                                AVC = AVM;
                                            }
                                            ARD = AQT;
                                            AVB = AVC;
                                        } else {
                                            let AOR = if QX == SB { 1.0 } else { 0.0 };
                                            let ARE;
                                            let AVN;
                                            if AOR != 0.0 {
                                                let AVO;
                                                if AOS != 0.0 {
                                                    let AOT = (QT * QR) / AY;
                                                    AVO = AOT;
                                                } else {
                                                    let AVP;
                                                    if AOU != 0.0 {
                                                        let AOV = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AVQ;
                                                        if AOV != 0.0 {
                                                            let AOW = if AIO == A { 1.0 } else { 0.0 };
                                                            let AVR = if AOW != 0.0 {
                                                                A
                                                            } else {
                                                                let AOX = (QT * QP) / (AY * AIO);
                                                                AOX
                                                            };
                                                            AVQ = AVR;
                                                        } else {
                                                            let AOY = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AVS;
                                                            if AOY != 0.0 {
                                                                let AOZ = QP + QQ;
                                                                let APA = if AOZ == A { 1.0 } else { 0.0 };
                                                                if APA != 0.0 {
                                                                } else {
                                                                }
                                                                let APB = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || APA != 0.0 { 1.0 } else { 0.0 };
                                                                let AVT = if APB != 0.0 {
                                                                    A
                                                                } else {
                                                                    let APC = (QT * AY) / ((RZ * AIO) * AOZ);
                                                                    APC
                                                                };
                                                                AVS = AVT;
                                                            } else {
                                                                AVS = A;
                                                            }
                                                            AVQ = AVS;
                                                        }
                                                        AVP = AVQ;
                                                    } else {
                                                        let APD = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AVU;
                                                        if APD != 0.0 {
                                                            let APE = if AIO == A { 1.0 } else { 0.0 };
                                                            let AVV = if APE != 0.0 {
                                                                A
                                                            } else {
                                                                let APF = (QT * QP) / (AY * AIO);
                                                                APF
                                                            };
                                                            AVU = AVV;
                                                        } else {
                                                            let APG = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AVW;
                                                            if APG != 0.0 {
                                                                let APH = QP + QQ;
                                                                let API = if APH == A { 1.0 } else { 0.0 };
                                                                if API != 0.0 {
                                                                } else {
                                                                }
                                                                let APJ = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || API != 0.0 { 1.0 } else { 0.0 };
                                                                let AVX = if APJ != 0.0 {
                                                                    A
                                                                } else {
                                                                    let APK = (QT * AY) / ((RZ * AIO) * APH);
                                                                    APK
                                                                };
                                                                AVW = AVX;
                                                            } else {
                                                                AVW = A;
                                                            }
                                                            AVU = AVW;
                                                        }
                                                        AVP = AVU;
                                                    }
                                                    AVO = AVP;
                                                }
                                                ARE = AQT;
                                                AVN = AVO;
                                            } else {
                                                let APL = if QX == SH { 1.0 } else { 0.0 };
                                                let ARF;
                                                let AVY;
                                                if APL != 0.0 {
                                                    let AVZ;
                                                    if APM != 0.0 {
                                                        let APN = if AHS == A { 1.0 } else { 0.0 };
                                                        let AWA = if APN != 0.0 {
                                                            A
                                                        } else {
                                                            let APO = (QT * QR) / (AY * AHS);
                                                            APO
                                                        };
                                                        AVZ = AWA;
                                                    } else {
                                                        let AWB;
                                                        if APP != 0.0 {
                                                            let APQ = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AWC;
                                                            if APQ != 0.0 {
                                                                let APR = if AIO == A { 1.0 } else { 0.0 };
                                                                let AWD = if APR != 0.0 {
                                                                    A
                                                                } else {
                                                                    let APS = (QT * QP) / (AY * AIO);
                                                                    APS
                                                                };
                                                                AWC = AWD;
                                                            } else {
                                                                let APT = if (if (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AWE;
                                                                if APT != 0.0 {
                                                                    let APU = if QP == A { 1.0 } else { 0.0 };
                                                                    if APU != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let APV = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || APU != 0.0 { 1.0 } else { 0.0 };
                                                                    let AWF = if APV != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let APW = (QT * AY) / ((SB * AIO) * QP);
                                                                        APW
                                                                    };
                                                                    AWE = AWF;
                                                                } else {
                                                                    AWE = A;
                                                                }
                                                                AWC = AWE;
                                                            }
                                                            AWB = AWC;
                                                        } else {
                                                            let APX = if (if (if QV == B { 1.0 } else { 0.0 }) != 0.0 || (if QV == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AWG;
                                                            if APX != 0.0 {
                                                                let APY = if AIO == A { 1.0 } else { 0.0 };
                                                                let AWH = if APY != 0.0 {
                                                                    A
                                                                } else {
                                                                    let APZ = (QT * QP) / (AY * AIO);
                                                                    APZ
                                                                };
                                                                AWG = AWH;
                                                            } else {
                                                                let AQA = if (if (if QV == AT { 1.0 } else { 0.0 }) != 0.0 || (if QV == SA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QV == SL { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AWI;
                                                                if AQA != 0.0 {
                                                                    let AQB = if QP == A { 1.0 } else { 0.0 };
                                                                    if AQB != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let AQC = if (if AIO == A { 1.0 } else { 0.0 }) != 0.0 || AQB != 0.0 { 1.0 } else { 0.0 };
                                                                    let AWJ = if AQC != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let AQD = (QT * AY) / ((SB * AIO) * QP);
                                                                        AQD
                                                                    };
                                                                    AWI = AWJ;
                                                                } else {
                                                                    AWI = A;
                                                                }
                                                                AWG = AWI;
                                                            }
                                                            AWB = AWG;
                                                        }
                                                        AVZ = AWB;
                                                    }
                                                    ARF = AQT;
                                                    AVY = AVZ;
                                                } else {
                                                    let AQE = if QX == SL { 1.0 } else { 0.0 };
                                                    let ARG;
                                                    let AWK;
                                                    if AQE != 0.0 {
                                                        let AQF = (QT * QR) / AY;
                                                        ARG = AQT;
                                                        AWK = AQF;
                                                    } else {
                                                        let AQG = if QX == QY { 1.0 } else { 0.0 };
                                                        let ARH;
                                                        let AWL;
                                                        if AQG != 0.0 {
                                                            let ARI;
                                                            let AWM;
                                                            if AQH != 0.0 {
                                                                let AQI = ((LP * QT) * QP) / AY;
                                                                let AQJ = if V == AT { 1.0 } else { 0.0 };
                                                                let ARJ = if AQJ != 0.0 {
                                                                    A
                                                                } else {
                                                                    let AQK = (QT * QP) / (AY * (V - AT));
                                                                    AQK
                                                                };
                                                                ARI = ARJ;
                                                                AWM = AQI;
                                                            } else {
                                                                let AQL = (QT * QP) / (AY * V);
                                                                ARI = AQL;
                                                                AWM = A;
                                                            }
                                                            ARH = ARI;
                                                            AWL = AWM;
                                                        } else {
                                                            let AQM = if QX == AAP { 1.0 } else { 0.0 };
                                                            let ARK;
                                                            let AWN;
                                                            if AQM != 0.0 {
                                                                let ARL;
                                                                let AWO;
                                                                if AQN != 0.0 {
                                                                    let AQO = (QT * QP) / (AY * V);
                                                                    ARL = AQO;
                                                                    AWO = A;
                                                                } else {
                                                                    let AQP = ((LP * QT) * QP) / AY;
                                                                    let AQQ = if V == AT { 1.0 } else { 0.0 };
                                                                    let ARM = if AQQ != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let AQR = (QT * QP) / (AY * (V - AT));
                                                                        AQR
                                                                    };
                                                                    ARL = ARM;
                                                                    AWO = AQP;
                                                                }
                                                                ARK = ARL;
                                                                AWN = AWO;
                                                            } else {
                                                                ARK = A;
                                                                AWN = AWP;
                                                            }
                                                            ARH = ARK;
                                                            AWL = AWN;
                                                        }
                                                        ARG = ARH;
                                                        AWK = AWL;
                                                    }
                                                    ARF = ARG;
                                                    AVY = AWK;
                                                }
                                                ARE = ARF;
                                                AVN = AVY;
                                            }
                                            ARD = ARE;
                                            AVB = AVN;
                                        }
                                        ARC = ARD;
                                        AUQ = AVB;
                                    }
                                    ARB = ARC;
                                    ATW = AUQ;
                                }
                                ARA = ARB;
                                ATC = ATW;
                            }
                            AQZ = ARA;
                            ASI = ATC;
                        }
                        AQS = AQZ;
                        ARO = ASI;
                    }
                    let ARN = if AQS <= A { 1.0 } else { 0.0 };
                    let AWT;
                    if ARN != 0.0 {
                        AWT = ARO;
                    } else {
                        let AWR = if ARO <= A { 1.0 } else { 0.0 };
                        let AWU = if AWR != 0.0 {
                            AQS
                        } else {
                            let AWS = (AQS * ARO) / (AQS + ARO);
                            AWS
                        };
                        AWT = AWU;
                    }
                    let AWV = if AWT == A { 1.0 } else { 0.0 };
                    if AWV != 0.0 {
                    } else {
                    }
                    AXC = AWT;
                    BNG = AHS;
                    BNO = BNP;
                    BNV = AIO;
                    BOD = BOE;
                } else {
                    AXC = A;
                    BNG = AHV;
                    BNO = BNL;
                    BNV = AIR;
                    BOD = BOA;
                }
                AXB = AXC;
                BNF = BNG;
                BNK = BNO;
                BNU = BNV;
                BNZ = BOD;
            }
            let AWW = if ON == A { 1.0 } else { 0.0 };
            let CFN;
            let CFQ;
            if AWW != 0.0 {
                let AXA = if AWX < AWZ { 1.0 } else { 0.0 };
                let CFO = if AXA != 0.0 {
                    A
                } else {
                    AWX
                };
                let AXD = if AXB < AWZ { 1.0 } else { 0.0 };
                let CFR = if AXD != 0.0 {
                    A
                } else {
                    AXB
                };
                CFN = CFO;
                CFQ = CFR;
            } else {
                let AXE = if AWX <= AWZ { 1.0 } else { 0.0 };
                let CFP = if AXE != 0.0 {
                    AWZ
                } else {
                    AWX
                };
                let AXF = if AXB <= AWZ { 1.0 } else { 0.0 };
                let CFS = if AXF != 0.0 {
                    AWZ
                } else {
                    AXB
                };
                CFN = CFP;
                CFQ = CFS;
            }
            let CFI;
            let CFK;
            let CPA;
            let CPC;
            let CPI;
            let CPK;
            if OO != 0.0 {
                let AXG = if EO <= A { 1.0 } else { 0.0 };
                let CPB = if AXG != 0.0 {
                    A
                } else {
                    EO
                };
                let AXH = if EP <= A { 1.0 } else { 0.0 };
                let CPJ = if AXH != 0.0 {
                    A
                } else {
                    EP
                };
                let AXJ = if AXI <= A { 1.0 } else { 0.0 };
                let CPD = if AXJ != 0.0 {
                    A
                } else {
                    AXI
                };
                let AXL = if AXK <= A { 1.0 } else { 0.0 };
                let CPL = if AXL != 0.0 {
                    A
                } else {
                    AXK
                };
                CFI = ER;
                CFK = AXN;
                CPA = CPB;
                CPC = CPD;
                CPI = CPJ;
                CPK = CPL;
            } else {
                let AXM = if ER <= A { 1.0 } else { 0.0 };
                let CFJ = if AXM != 0.0 {
                    A
                } else {
                    ER
                };
                let AXO = if AXN <= A { 1.0 } else { 0.0 };
                let CFL = if AXO != 0.0 {
                    A
                } else {
                    AXN
                };
                CFI = CFJ;
                CFK = CFL;
                CPA = EO;
                CPC = AXI;
                CPI = EP;
                CPK = AXK;
            }
            let AXQ = if AXP != A { 1.0 } else { 0.0 };
            let EHD;
            let EHJ;
            let EHO;
            let EHV;
            let EIA;
            if AXQ != 0.0 {
                let AXT = (if (AU * AXR) >= AXS { (AU * AXR) } else { AXS }).ln();
                let AXU = (if (AY * AXR) >= AXS { (AY * AXR) } else { AXS }).ln();
                let AXV = (if V >= AXS { V } else { AXS }).ln();
                let AYB = if (if (if parameter_given[757] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if parameter_given[761] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AYE;
                if AYB != 0.0 {
                    AYE = B;
                } else {
                    let AYC = if (if (if (if parameter_given[773] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[774] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if parameter_given[775] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[776] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AYF = if AYC != 0.0 {
                        RZ
                    } else {
                        RS
                    };
                    AYE = AYF;
                }
                let AYD = if AXP == AT { 1.0 } else { 0.0 };
                let AYW;
                let AZD;
                let AZG;
                let AZK;
                let AZO;
                if AYD != 0.0 {
                    let AYG = if AYE == RS { 1.0 } else { 0.0 };
                    let AYX;
                    let AZL;
                    if AYG != 0.0 {
                        let AYH = rspice_limited_exp((((parameters[777] * AXT) + (parameters[778] * AXU)) + (parameters[779] * AXV)));
                        let AYI = parameters[773] * AYH;
                        let AYJ = rspice_limited_exp((((parameters[780] * AXT) + (parameters[781] * AXU)) + (parameters[782] * AXV)));
                        let AYK = parameters[774] * AYJ;
                        let AYL = (AYI * AYK) / (AYI + AYK);
                        let AYM = parameters[775] * AYH;
                        let AYN = parameters[776] * AYJ;
                        let AYO = (AYM * AYN) / (AYM + AYN);
                        AYX = AYO;
                        AZL = AYL;
                    } else {
                        AYX = AXZ;
                        AZL = AYA;
                    }
                    let AYP = if (if AYE == RZ { 1.0 } else { 0.0 }) != 0.0 || AYG != 0.0 { 1.0 } else { 0.0 };
                    let AZH;
                    let AZP;
                    if AYP != 0.0 {
                        let AYQ = parameters[757] * (rspice_limited_exp((((parameters[758] * AXT) + (parameters[759] * AXU)) + (parameters[760] * AXV))));
                        let AYR = parameters[761] * (rspice_limited_exp((((parameters[762] * AXT) + (parameters[763] * AXU)) + (parameters[764] * AXV))));
                        AZH = AYQ;
                        AZP = AYR;
                    } else {
                        AZH = AXY;
                        AZP = AXX;
                    }
                    let AYS = parameters[765] * (rspice_limited_exp((((parameters[766] * AXT) + (parameters[767] * AXU)) + (parameters[768] * AXV))));
                    let AYT = parameters[769] * (rspice_limited_exp((((parameters[770] * AXT) + (parameters[771] * AXU)) + (parameters[772] * AXV))));
                    let AYU = (AYS * AYT) / (AYS + AYT);
                    AYW = AYX;
                    AZD = AYU;
                    AZG = AZH;
                    AZK = AZL;
                    AZO = AZP;
                } else {
                    AYW = AXZ;
                    AZD = AXW;
                    AZG = AXY;
                    AZK = AYA;
                    AZO = AXX;
                }
                let AYV = if (if AXP == B { 1.0 } else { 0.0 }) != 0.0 || (if AYD != 0.0 && (if AYE == RS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EHE;
                let EHK;
                let EHP;
                let EHW;
                let EIB;
                if AYV != 0.0 {
                    let AYZ = if AYW < AYY { 1.0 } else { 0.0 };
                    let EHX = if AYZ != 0.0 {
                        AZA
                    } else {
                        let AZC = AZB + (B / AYW);
                        AZC
                    };
                    let AZE = if AZD < AYY { 1.0 } else { 0.0 };
                    let EHQ = if AZE != 0.0 {
                        AZA
                    } else {
                        let AZF = AZB + (B / AZD);
                        AZF
                    };
                    let AZI = if AZG < AYY { 1.0 } else { 0.0 };
                    let EHF = if AZI != 0.0 {
                        AZA
                    } else {
                        let AZJ = AZB + (B / AZG);
                        AZJ
                    };
                    let AZM = if AZK < AYY { 1.0 } else { 0.0 };
                    let EHL = if AZM != 0.0 {
                        AZA
                    } else {
                        let AZN = AZB + (B / AZK);
                        AZN
                    };
                    let AZQ = if AZO < AYY { 1.0 } else { 0.0 };
                    let EIC = if AZQ != 0.0 {
                        AZA
                    } else {
                        let AZR = AZB + (B / AZO);
                        AZR
                    };
                    EHE = EHF;
                    EHK = EHL;
                    EHP = EHQ;
                    EHW = EHX;
                    EIB = EIC;
                } else {
                    let AZS = if AYD != 0.0 && (if AYE == RZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EHG;
                    let EHM;
                    let EHR;
                    let EHY;
                    let EID;
                    if AZS != 0.0 {
                        let AZT = if AZD < AYY { 1.0 } else { 0.0 };
                        let EHS = if AZT != 0.0 {
                            AZA
                        } else {
                            let AZU = AZB + (B / AZD);
                            AZU
                        };
                        let AZV = if AZG < AYY { 1.0 } else { 0.0 };
                        let EHH = if AZV != 0.0 {
                            AZA
                        } else {
                            let AZW = AZB + (B / AZG);
                            AZW
                        };
                        let AZX = if AZO < AYY { 1.0 } else { 0.0 };
                        let EIE = if AZX != 0.0 {
                            AZA
                        } else {
                            let AZY = AZB + (B / AZO);
                            AZY
                        };
                        EHG = EHH;
                        EHM = AZB;
                        EHR = EHS;
                        EHY = AZB;
                        EID = EIE;
                    } else {
                        let AZZ = if AYD != 0.0 && (if AYE == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EHI;
                        let EHN;
                        let EHT;
                        let EHZ;
                        let EIF;
                        if AZZ != 0.0 {
                            let BAA = if AZD < AYY { 1.0 } else { 0.0 };
                            let EHU = if BAA != 0.0 {
                                AZA
                            } else {
                                let BAB = AZB + (B / AZD);
                                BAB
                            };
                            EHI = AZA;
                            EHN = AZB;
                            EHT = EHU;
                            EHZ = AZB;
                            EIF = AZA;
                        } else {
                            EHI = A;
                            EHN = A;
                            EHT = A;
                            EHZ = A;
                            EIF = A;
                        }
                        EHG = EHI;
                        EHM = EHN;
                        EHR = EHT;
                        EHY = EHZ;
                        EID = EIF;
                    }
                    EHE = EHG;
                    EHK = EHM;
                    EHP = EHR;
                    EHW = EHY;
                    EIB = EID;
                }
                EHD = EHE;
                EHJ = EHK;
                EHO = EHP;
                EHV = EHW;
                EIA = EIB;
            } else {
                EHD = A;
                EHJ = A;
                EHO = A;
                EHV = A;
                EIA = A;
            }
            let BAD = if BAC == B { 1.0 } else { 0.0 };
            let DFZ;
            let EIO;
            if BAD != 0.0 {
                let BAF = if BAE < AYY { 1.0 } else { 0.0 };
                let EIP = if BAF != 0.0 {
                    AZA
                } else {
                    let BAG = AZB + (B / BAE);
                    BAG
                };
                let BAI = B - BAH;
                DFZ = BAI;
                EIO = EIP;
            } else {
                DFZ = B;
                EIO = A;
            }
            let BAK = (parameters[700] * (parameters[31] + ((BN / RZ) / BAJ))) / ((BAJ * V) * (T - parameters[699]));
            let BAL = if BAK > A { 1.0 } else { 0.0 };
            let CZU;
            if BAL != 0.0 {
                let BAM = B / BAK;
                CZU = BAM;
            } else {
                let BAO = if BAN != A { 1.0 } else { 0.0 };
                if BAO != 0.0 {
                } else {
                }
                CZU = AZA;
            }
            let BAP = K * GX;
            let BAR = (rspice_limited_exp((HA * ((if (BAQ / K) >= AXS { (BAQ / K) } else { AXS }).ln())))) / (K * K);
            let BAS = (rspice_limited_exp((HA * ((if (BAQ / BAP) >= AXS { (BAQ / BAP) } else { AXS }).ln())))) / (BAP * BAP);
            let BAV = if D != 0.0 {
                BAT
            } else {
                BAU
            };
            let BAY = if D != 0.0 {
                BAW
            } else {
                BAX
            };
            let BAZ = (BAV * AY) * BAS;
            let BBA = (-BAY) * K;
            let BBB = BBA * GX;
            let BBC = BAV * ((AY * AU) * BAR);
            let BBD = if (if OK != A { 1.0 } else { 0.0 }) != 0.0 && (if OL > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BBE = if BBD != 0.0 && (if (parameters[911] + AY) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if BBE != 0.0 {
            } else {
            }
            let BBG = if BBF <= -2.7315e2f64 { 1.0 } else { 0.0 };
            let BBQ = if BBG != 0.0 {
                BBH
            } else {
                let BBI = BBF + 2.7315e2f64;
                BBI
            };
            let BBJ = temperature + parameters[33];
            let BBL = if BBE != 0.0 {
                BBK
            } else {
                A
            };
            let BBM = BBL + BBJ;
            let BBO = BBN * BBM;
            let BBP = B / BBO;
            let BBR = BBM / BBQ;
            let BBS = BBM - BBQ;
            let BBT = BBN * BBQ;
            let BBX = BBU - (((BBV * BBM) * BBM) / (BBM + BBW));
            let BBY = BBU - (((BBV * BBQ) * BBQ) / (BBQ + BBW));
            let BBZ = (parameters[108] * (BBR * (BBR.sqrt()))) * (rspice_limited_exp(((BBX / (AT * BBT)) - (BBX / (AT * BBO)))));
            let BCK = if BBE != 0.0 {
                let BCA = (if (JH / BBZ) >= AXS { (JH / BBZ) } else { AXS }).ln();
                let BCB = ((BCA * BCA) + BP).sqrt();
                BCB
            } else {
                let BCC = (if (JH / BBZ) >= AXS { (JH / BBZ) } else { AXS }).ln();
                BCC
            };
            let EAV = if BBE != 0.0 {
                let BCD = (if ((HN * DG) / (BBZ * BBZ)) >= AXS { ((HN * DG) / (BBZ * BBZ)) } else { AXS }).ln();
                let BCE = ((BCD * BCD) + BP).sqrt();
                BCE
            } else {
                let BCF = (if ((HN * DG) / (BBZ * BBZ)) >= AXS { ((HN * DG) / (BBZ * BBZ)) } else { AXS }).ln();
                BCF
            };
            let BCG = if DJ > A { 1.0 } else { 0.0 };
            let COX = if BCG != 0.0 {
                let BCI = (((-BCH) * BBO) * ((if (DJ / DG) >= AXS { (DJ / DG) } else { AXS }).ln())) + parameters[5];
                BCI
            } else {
                A
            };
            let BCL = if ((BCJ + (BBO * BCK)) + DX) >= BCJ { ((BCJ + (BBO * BCK)) + DX) } else { BCJ };
            let BCM = BCL.sqrt();
            let BCN = AT * H;
            let BCP = (BCN / (BCO * JH)).sqrt();
            let BCQ = (((H / J) * K) * DW).sqrt();
            let BCR = BBR - B;
            let BCS = B + (parameters[823] * BCR);
            let BCT = if BCS < -1e1f64 { 1.0 } else { 0.0 };
            let BCW = if BCT != 0.0 {
                let BCU = -1e-6f64 / BCS;
                BCU
            } else {
                let BCV = LP * (BCS + (((BCS * BCS) + 4e-6f64).sqrt()));
                BCV
            };
            let BCX = JK * BCW;
            let BCY = B + (parameters[851] * BCR);
            let BCZ = LE * BCY;
            let BYY = if IP != 0.0 {
                let BDB = BDA * BCY;
                BDB
            } else {
                A
            };
            let BDC = if C != B { 1.0 } else { 0.0 };
            let BDH = if BDC != 0.0 {
                let BDF = BDD * BDE;
                BDF
            } else {
                let BDG = LP * BDE;
                BDG
            };
            let BDJ = BBR.powf(OE);
            let BDK = BDI * BDJ;
            let BDL = (B + (OF * BBS)) - BP;
            let BDM = if BDL < -1e1f64 { 1.0 } else { 0.0 };
            let BDQ = if BDM != 0.0 {
                let BDN = -1e-6f64 / BDL;
                BDN
            } else {
                let BDO = LP * (BDL + (((BDL * BDL) + 4e-6f64).sqrt()));
                BDO
            };
            let BDR = BDP * BDQ;
            let BDS = (B + (FV * BBS)) - BP;
            let BDT = if BDS < -1e1f64 { 1.0 } else { 0.0 };
            let BDW = if BDT != 0.0 {
                let BDU = -1e-6f64 / BDS;
                BDU
            } else {
                let BDV = LP * (BDS + (((BDS * BDS) + 4e-6f64).sqrt()));
                BDV
            };
            let BDX = KZ * BDW;
            let BDZ = BBR.powf(OG);
            let BEA = BDY * BDZ;
            let BEC = BBR.powf(FY);
            let BED = BEB * BEC;
            let BEE = B + (FX * BCR);
            let BEF = if BEE < -1e1f64 { 1.0 } else { 0.0 };
            let BEJ = if BEF != 0.0 {
                let BEG = -1e-6f64 / BEE;
                BEG
            } else {
                let BEH = LP * (BEE + (((BEE * BEE) + 4e-6f64).sqrt()));
                BEH
            };
            let BEK = BEI * BEJ;
            let BZN;
            let BZP;
            let BZR;
            let BZT;
            let BZV;
            if IP != 0.0 {
                let BEQ = BEL * BDJ;
                let BER = if BDL < -1e1f64 { 1.0 } else { 0.0 };
                let BEV = if BER != 0.0 {
                    let BES = -1e-6f64 / BDL;
                    BES
                } else {
                    let BET = LP * (BDL + (((BDL * BDL) + 4e-6f64).sqrt()));
                    BET
                };
                let BEW = BEU * BEV;
                let BEX = if BDS < -1e1f64 { 1.0 } else { 0.0 };
                let BFB = if BEX != 0.0 {
                    let BEY = -1e-6f64 / BDS;
                    BEY
                } else {
                    let BEZ = LP * (BDS + (((BDS * BDS) + 4e-6f64).sqrt()));
                    BEZ
                };
                let BFC = BFA * BFB;
                let BFE = BFD * BDZ;
                let BFI = BFF * BEC;
                BZN = BEQ;
                BZP = BEW;
                BZR = BFC;
                BZT = BFE;
                BZV = BFI;
            } else {
                BZN = A;
                BZP = A;
                BZR = A;
                BZT = A;
                BZV = A;
            }
            let BFJ = BBR.powf(FZ);
            let BFK = BBR.powf((-OH));
            let BFL = MB * BFK;
            let BFN = if BFL < BFM { 1.0 } else { 0.0 };
            let BUX = if BFN != 0.0 {
                BFM
            } else {
                BFL
            };
            let BFP = if BFO == B { 1.0 } else { 0.0 };
            let CQW;
            let CSI;
            if BFP != 0.0 {
                let BFQ = BBR.powf(parameters[1120]);
                let BFR = parameters[1100] * (BBR.powf((-parameters[1121])));
                CQW = BFR;
                CSI = BFQ;
            } else {
                CQW = B;
                CSI = B;
            }
            let BZH;
            if IP != 0.0 {
                let BFT = BFS * BFK;
                let BFU = if BFT < BFM { 1.0 } else { 0.0 };
                let BZI = if BFU != 0.0 {
                    BFM
                } else {
                    BFT
                };
                BZH = BZI;
            } else {
                BZH = A;
            }
            let BFV = NT * BFK;
            let BFW = if BFV < BFM { 1.0 } else { 0.0 };
            let DTW = if BFW != 0.0 {
                BFM
            } else {
                BFV
            };
            let BFX = ((B / LQ) * (B + (parameters[861] * BBS))) - AT;
            let BFY = if BFX < -1e1f64 { 1.0 } else { 0.0 };
            let BGB = if BFY != 0.0 {
                let BFZ = -1e-6f64 / BFX;
                BFZ
            } else {
                let BGA = LP * (BFX + (((BFX * BFX) + 4e-6f64).sqrt()));
                BGA
            };
            let BGC = B / (BGB + AT);
            let BGD = (B - (OI * BBS)) - BP;
            let BGE = if BGD < -1e1f64 { 1.0 } else { 0.0 };
            let BGH = if BGE != 0.0 {
                let BGF = -1e-6f64 / BGD;
                BGF
            } else {
                let BGG = LP * (BGD + (((BGD * BGD) + 4e-6f64).sqrt()));
                BGG
            };
            let BGI = MM * BGH;
            let BZL;
            if IP != 0.0 {
                let BGJ = if BGD < -1e1f64 { 1.0 } else { 0.0 };
                let BGN = if BGJ != 0.0 {
                    let BGK = -1e-6f64 / BGD;
                    BGK
                } else {
                    let BGL = LP * (BGD + (((BGD * BGD) + 4e-6f64).sqrt()));
                    BGL
                };
                let BGO = BGM * BGN;
                BZL = BGO;
            } else {
                BZL = A;
            }
            let BGP = (B + (HF * BBS)) - BP;
            let BGQ = if BGP < -1e1f64 { 1.0 } else { 0.0 };
            let BGT = if BGQ != 0.0 {
                let BGR = -1e-6f64 / BGP;
                BGR
            } else {
                let BGS = LP * (BGP + (((BGP * BGP) + 4e-6f64).sqrt()));
                BGS
            };
            let BGU = HE * BGT;
            let BGV = (B + (HH * BBS)) - BP;
            let BGW = if BGV < -1e1f64 { 1.0 } else { 0.0 };
            let BGZ = if BGW != 0.0 {
                let BGX = -1e-6f64 / BGV;
                BGX
            } else {
                let BGY = LP * (BGV + (((BGV * BGV) + 4e-6f64).sqrt()));
                BGY
            };
            let BHA = HG * BGZ;
            let BHB = BBR.powf(GC);
            let BHC = MW * BHB;
            let BZZ = if IP != 0.0 {
                let BHE = BHD * BHB;
                BHE
            } else {
                A
            };
            let BHF = (B + (GD * BBS)) - BP;
            let BHG = if BHF < -1e1f64 { 1.0 } else { 0.0 };
            let BHJ = if BHG != 0.0 {
                let BHH = -1e-6f64 / BHF;
                BHH
            } else {
                let BHI = LP * (BHF + (((BHF * BHF) + 4e-6f64).sqrt()));
                BHI
            };
            let BHK = FM * BHJ;
            let BHL = if BHF < -1e1f64 { 1.0 } else { 0.0 };
            let BHO = if BHL != 0.0 {
                let BHM = -1e-6f64 / BHF;
                BHM
            } else {
                let BHN = LP * (BHF + (((BHF * BHF) + 4e-6f64).sqrt()));
                BHN
            };
            let BHP = FQ * BHO;
            let BHQ = (if BBR >= AXS { BBR } else { AXS }).ln();
            let BHR = rspice_limited_exp((GE * BHQ));
            let BHS = (B + (HK * BBS)) - BP;
            let BHT = if BHS < -1e1f64 { 1.0 } else { 0.0 };
            let BHW = if BHT != 0.0 {
                let BHU = -1e-6f64 / BHS;
                BHU
            } else {
                let BHV = LP * (BHS + (((BHS * BHS) + 4e-6f64).sqrt()));
                BHV
            };
            let BHX = HI * BHW;
            let BHY = (B + (HL * BBS)) - BP;
            let BHZ = if BHY < -1e1f64 { 1.0 } else { 0.0 };
            let BID = if BHZ != 0.0 {
                let BIA = -1e-6f64 / BHY;
                BIA
            } else {
                let BIB = LP * (BHY + (((BHY * BHY) + 4e-6f64).sqrt()));
                BIB
            };
            let BIE = BIC * BID;
            let BIF = (B + (IG * BBS)) - BP;
            let BIG = if BIF < -1e1f64 { 1.0 } else { 0.0 };
            let BIJ = if BIG != 0.0 {
                let BIH = -1e-6f64 / BIF;
                BIH
            } else {
                let BII = LP * (BIF + (((BIF * BIF) + 4e-6f64).sqrt()));
                BII
            };
            let BIK = IF * BIJ;
            let BIL = (B + (II * BBS)) - BP;
            let BIM = if BIL < -1e1f64 { 1.0 } else { 0.0 };
            let BIP = if BIM != 0.0 {
                let BIN = -1e-6f64 / BIL;
                BIN
            } else {
                let BIO = LP * (BIL + (((BIL * BIL) + 4e-6f64).sqrt()));
                BIO
            };
            let BIQ = IH * BIP;
            let BIR = (B + (IK * BBS)) - BP;
            let BIS = if BIR < -1e1f64 { 1.0 } else { 0.0 };
            let BIV = if BIS != 0.0 {
                let BIT = -1e-6f64 / BIR;
                BIT
            } else {
                let BIU = LP * (BIR + (((BIR * BIR) + 4e-6f64).sqrt()));
                BIU
            };
            let BIW = IJ * BIV;
            let BIX = (B + (parameters[889] * BBS)) - BP;
            let BIY = if BIX < -1e1f64 { 1.0 } else { 0.0 };
            let BJB = if BIY != 0.0 {
                let BIZ = -1e-6f64 / BIX;
                BIZ
            } else {
                let BJA = LP * (BIX + (((BIX * BIX) + 4e-6f64).sqrt()));
                BJA
            };
            let BJC = parameters[701] * BJB;
            let BJD = if BIX < -1e1f64 { 1.0 } else { 0.0 };
            let BJG = if BJD != 0.0 {
                let BJE = -1e-6f64 / BIX;
                BJE
            } else {
                let BJF = LP * (BIX + (((BIX * BIX) + 4e-6f64).sqrt()));
                BJF
            };
            let BJH = parameters[702] * BJG;
            let BJI = (B + (parameters[890] * BBS)) - BP;
            let BJJ = if BJI < -1e1f64 { 1.0 } else { 0.0 };
            let BJM = if BJJ != 0.0 {
                let BJK = -1e-6f64 / BJI;
                BJK
            } else {
                let BJL = LP * (BJI + (((BJI * BJI) + 4e-6f64).sqrt()));
                BJL
            };
            let BJN = parameters[703] * BJM;
            let BJO = if BJI < -1e1f64 { 1.0 } else { 0.0 };
            let BJR = if BJO != 0.0 {
                let BJP = -1e-6f64 / BJI;
                BJP
            } else {
                let BJQ = LP * (BJI + (((BJI * BJI) + 4e-6f64).sqrt()));
                BJQ
            };
            let BJS = parameters[704] * BJR;
            let BJT = (B + (parameters[891] * BBS)) - BP;
            let BJU = if BJT < -1e1f64 { 1.0 } else { 0.0 };
            let BJX = if BJU != 0.0 {
                let BJV = -1e-6f64 / BJT;
                BJV
            } else {
                let BJW = LP * (BJT + (((BJT * BJT) + 4e-6f64).sqrt()));
                BJW
            };
            let BJY = parameters[705] * BJX;
            let BJZ = if BJT < -1e1f64 { 1.0 } else { 0.0 };
            let BKC = if BJZ != 0.0 {
                let BKA = -1e-6f64 / BJT;
                BKA
            } else {
                let BKB = LP * (BJT + (((BJT * BJT) + 4e-6f64).sqrt()));
                BKB
            };
            let BKD = parameters[706] * BKC;
            let BKE = parameters[892] * BBS;
            let BKG = (parameters[707] - BKE) - BKF;
            let BKH = if BKG < -1e1f64 { 1.0 } else { 0.0 };
            let BKK = if BKH != 0.0 {
                let BKI = -1e-6f64 / BKG;
                BKI
            } else {
                let BKJ = LP * (BKG + (((BKG * BKG) + 4e-6f64).sqrt()));
                BKJ
            };
            let BKL = BKK + BKF;
            let BKM = (parameters[708] - BKE) - BKF;
            let BKN = if BKM < -1e1f64 { 1.0 } else { 0.0 };
            let BKQ = if BKN != 0.0 {
                let BKO = -1e-6f64 / BKM;
                BKO
            } else {
                let BKP = LP * (BKM + (((BKM * BKM) + 4e-6f64).sqrt()));
                BKP
            };
            let BKR = BKQ + BKF;
            let BKS = parameters[893] * BBS;
            let BKT = (parameters[709] - BKS) - BKF;
            let BKU = if BKT < -1e1f64 { 1.0 } else { 0.0 };
            let BKX = if BKU != 0.0 {
                let BKV = -1e-6f64 / BKT;
                BKV
            } else {
                let BKW = LP * (BKT + (((BKT * BKT) + 4e-6f64).sqrt()));
                BKW
            };
            let BKY = BKX + BKF;
            let BKZ = (parameters[710] - BKS) - BKF;
            let BLA = if BKZ < -1e1f64 { 1.0 } else { 0.0 };
            let BLD = if BLA != 0.0 {
                let BLB = -1e-6f64 / BKZ;
                BLB
            } else {
                let BLC = LP * (BKZ + (((BKZ * BKZ) + 4e-6f64).sqrt()));
                BLC
            };
            let BLE = BLD + BKF;
            let BLF = parameters[894] * BBS;
            let BLG = (parameters[711] - BLF) - BKF;
            let BLH = if BLG < -1e1f64 { 1.0 } else { 0.0 };
            let BLK = if BLH != 0.0 {
                let BLI = -1e-6f64 / BLG;
                BLI
            } else {
                let BLJ = LP * (BLG + (((BLG * BLG) + 4e-6f64).sqrt()));
                BLJ
            };
            let BLL = BLK + BKF;
            let BLM = (parameters[712] - BLF) - BKF;
            let BLN = if BLM < -1e1f64 { 1.0 } else { 0.0 };
            let BLQ = if BLN != 0.0 {
                let BLO = -1e-6f64 / BLM;
                BLO
            } else {
                let BLP = LP * (BLM + (((BLM * BLM) + 4e-6f64).sqrt()));
                BLP
            };
            let BLR = BLQ + BKF;
            let BLS = (BBY / BBT) - (BBX / BBO);
            let BLT = rspice_limited_exp(((BLS + (parameters[895] * BHQ)) / parameters[725]));
            let BLU = parameters[719] * BLT;
            let BLV = parameters[721] * BLT;
            let BLW = parameters[723] * BLT;
            let BLX = rspice_limited_exp(((BLS + (parameters[896] * BHQ)) / parameters[726]));
            let BLY = parameters[720] * BLX;
            let BLZ = parameters[722] * BLX;
            let BMA = parameters[724] * BLX;
            let BMB = parameters[735] * (rspice_limited_exp((((BBY * parameters[897]) * BCR) / BBO)));
            let BMC = parameters[737] * (rspice_limited_exp((((BBY * parameters[899]) * BCR) / BBO)));
            let BMD = ((parameters[741] / BN).sqrt()) + B;
            let BME = (parameters[739] * BMD) * (rspice_limited_exp((((BBY * parameters[901]) * BCR) / BBO)));
            let BMF = parameters[736] * (rspice_limited_exp((((BBY * parameters[898]) * BCR) / BBO)));
            let BMG = parameters[738] * (rspice_limited_exp((((BBY * parameters[900]) * BCR) / BBO)));
            let BMH = (parameters[740] * BMD) * (rspice_limited_exp((((BBY * parameters[902]) * BCR) / BBO)));
            let BMI = if ((parameters[742] * (B + (parameters[903] * BCR))) - BKF) < -1e1f64 { 1.0 } else { 0.0 };
            if BMI != 0.0 {
            } else {
            }
            let BMJ = if ((parameters[744] * (B + (parameters[905] * BCR))) - BKF) < -1e1f64 { 1.0 } else { 0.0 };
            if BMJ != 0.0 {
            } else {
            }
            let BMK = if ((parameters[746] * (B + (parameters[907] * BCR))) - BKF) < -1e1f64 { 1.0 } else { 0.0 };
            if BMK != 0.0 {
            } else {
            }
            let BML = if ((parameters[743] * (B + (parameters[904] * BCR))) - BKF) < -1e1f64 { 1.0 } else { 0.0 };
            if BML != 0.0 {
            } else {
            }
            let BMM = if ((parameters[745] * (B + (parameters[906] * BCR))) - BKF) < -1e1f64 { 1.0 } else { 0.0 };
            if BMM != 0.0 {
            } else {
            }
            let BMN = if ((parameters[747] * (B + (parameters[908] * BCR))) - BKF) < -1e1f64 { 1.0 } else { 0.0 };
            if BMN != 0.0 {
            } else {
            }
            let BMO = if QX < QY { 1.0 } else { 0.0 };
            let BNC;
            let BNH;
            let BNR;
            let BNW;
            if BMO != 0.0 {
                let BMP = if (V % AT) != A { 1.0 } else { 0.0 };
                let BND;
                let BNI;
                let BNS;
                let BNX;
                if BMP != 0.0 {
                    let BMQ = AT * (if ((V - B) / AT) >= A { ((V - B) / AT) } else { A });
                    BND = B;
                    BNI = BMQ;
                    BNS = B;
                    BNX = BMQ;
                } else {
                    let BMR = if RC == B { 1.0 } else { 0.0 };
                    let BNE;
                    let BNJ;
                    let BNT;
                    let BNY;
                    if BMR != 0.0 {
                        let BMS = AT * (if ((V / AT) - B) >= A { ((V / AT) - B) } else { A });
                        BNE = A;
                        BNJ = V;
                        BNT = AT;
                        BNY = BMS;
                    } else {
                        let BMT = AT * (if ((V / AT) - B) >= A { ((V / AT) - B) } else { A });
                        BNE = AT;
                        BNJ = BMT;
                        BNT = A;
                        BNY = V;
                    }
                    BND = BNE;
                    BNI = BNJ;
                    BNS = BNT;
                    BNX = BNY;
                }
                BNC = BND;
                BNH = BNI;
                BNR = BNS;
                BNW = BNX;
            } else {
                BNC = BNF;
                BNH = BNK;
                BNR = BNU;
                BNW = BNZ;
            }
            let BMU = QP + QQ;
            let BMV = QP + QP;
            let BMW = QR + QR;
            let BMX = (BMU + BMU) + BN;
            let BMY = BMU * BN;
            let BMZ = QP * BN;
            let BNA = QR * BN;
            let BNB = if QX == A { 1.0 } else { 0.0 };
            let BQQ;
            let BRF;
            let BRY;
            let BSP;
            if BNB != 0.0 {
                let BNQ = (BNC * BMX) + (BNH * BMV);
                let BOF = (BNR * BMX) + (BNW * BMV);
                let BOG = (BNC * BMY) + (BNH * BMZ);
                let BOH = (BNR * BMY) + (BNW * BMZ);
                BQQ = BOG;
                BRF = BOH;
                BRY = BNQ;
                BSP = BOF;
            } else {
                let BOI = if QX == B { 1.0 } else { 0.0 };
                let BQR;
                let BRG;
                let BRZ;
                let BSQ;
                if BOI != 0.0 {
                    let BOJ = (BNC * BMX) + (BNH * BMV);
                    let BOK = BNR + BNW;
                    let BOL = BOK * BMV;
                    let BOM = (BNC * BMY) + (BNH * BMZ);
                    let BON = BOK * BMZ;
                    BQR = BOM;
                    BRG = BON;
                    BRZ = BOJ;
                    BSQ = BOL;
                } else {
                    let BOO = if QX == AT { 1.0 } else { 0.0 };
                    let BQS;
                    let BRH;
                    let BSA;
                    let BSR;
                    if BOO != 0.0 {
                        let BOP = BNC + BNH;
                        let BOQ = BOP * BMV;
                        let BOR = (BNR * BMX) + (BNW * BMV);
                        let BOS = BOP * BMZ;
                        let BOT = (BNR * BMY) + (BNW * BMZ);
                        BQS = BOS;
                        BRH = BOT;
                        BSA = BOQ;
                        BSR = BOR;
                    } else {
                        let BOU = if QX == RZ { 1.0 } else { 0.0 };
                        let BQT;
                        let BRI;
                        let BSB;
                        let BSS;
                        if BOU != 0.0 {
                            let BOV = BNC + BNH;
                            let BOW = BOV * BMV;
                            let BOX = BNR + BNW;
                            let BOY = BOX * BMV;
                            let BOZ = BOV * BMZ;
                            let BPA = BOX * BMZ;
                            BQT = BOZ;
                            BRI = BPA;
                            BSB = BOW;
                            BSS = BOY;
                        } else {
                            let BPB = if QX == SA { 1.0 } else { 0.0 };
                            let BQU;
                            let BRJ;
                            let BSC;
                            let BST;
                            if BPB != 0.0 {
                                let BPC = (BNC * BMX) + (BNH * BMV);
                                let BPD = (BNR * BMW) + (BNW * BMV);
                                let BPE = (BNC * BMY) + (BNH * BMZ);
                                let BPF = (BNR * BNA) + (BNW * BMZ);
                                BQU = BPE;
                                BRJ = BPF;
                                BSC = BPC;
                                BST = BPD;
                            } else {
                                let BPG = if QX == RS { 1.0 } else { 0.0 };
                                let BQV;
                                let BRK;
                                let BSD;
                                let BSU;
                                if BPG != 0.0 {
                                    let BPH = BNC + BNH;
                                    let BPI = BPH * BMV;
                                    let BPJ = (BNR * BMW) + (BNW * BMV);
                                    let BPK = BPH * BMZ;
                                    let BPL = (BNR * BNA) + (BNW * BMZ);
                                    BQV = BPK;
                                    BRK = BPL;
                                    BSD = BPI;
                                    BSU = BPJ;
                                } else {
                                    let BPM = if QX == SB { 1.0 } else { 0.0 };
                                    let BQW;
                                    let BRL;
                                    let BSE;
                                    let BSV;
                                    if BPM != 0.0 {
                                        let BPN = (BNC * BMW) + (BNH * BMV);
                                        let BPO = (BNR * BMX) + (BNW * BMV);
                                        let BPP = (BNC * BNA) + (BNH * BMZ);
                                        let BPQ = (BNR * BMY) + (BNW * BMZ);
                                        BQW = BPP;
                                        BRL = BPQ;
                                        BSE = BPN;
                                        BSV = BPO;
                                    } else {
                                        let BPR = if QX == SH { 1.0 } else { 0.0 };
                                        let BQX;
                                        let BRM;
                                        let BSF;
                                        let BSW;
                                        if BPR != 0.0 {
                                            let BPS = (BNC * BMW) + (BNH * BMV);
                                            let BPT = BNR + BNW;
                                            let BPU = BPT * BMV;
                                            let BPV = (BNC * BNA) + (BNH * BMZ);
                                            let BPW = BPT * BMZ;
                                            BQX = BPV;
                                            BRM = BPW;
                                            BSF = BPS;
                                            BSW = BPU;
                                        } else {
                                            let BPX = if QX == SL { 1.0 } else { 0.0 };
                                            let BQY;
                                            let BRN;
                                            let BSG;
                                            let BSX;
                                            if BPX != 0.0 {
                                                let BPY = (BNC * BMW) + (BNH * BMV);
                                                let BPZ = (BNR * BMW) + (BNW * BMV);
                                                let BQA = (BNC * BNA) + (BNH * BMZ);
                                                let BQB = (BNR * BNA) + (BNW * BMZ);
                                                BQY = BQA;
                                                BRN = BQB;
                                                BSG = BPY;
                                                BSX = BPZ;
                                            } else {
                                                let BQC = if QX == QY { 1.0 } else { 0.0 };
                                                let BQZ;
                                                let BRO;
                                                let BSH;
                                                let BSY;
                                                if BQC != 0.0 {
                                                    let BQD = V - B;
                                                    let BQE = BMX + (BQD * BMV);
                                                    let BQF = V * BMV;
                                                    let BQG = BMY + (BQD * BMZ);
                                                    let BQH = V * BMZ;
                                                    BQZ = BQG;
                                                    BRO = BQH;
                                                    BSH = BQE;
                                                    BSY = BQF;
                                                } else {
                                                    let BQI = if QX == AAP { 1.0 } else { 0.0 };
                                                    let BRA;
                                                    let BRP;
                                                    let BSI;
                                                    let BSZ;
                                                    if BQI != 0.0 {
                                                        let BQJ = V * BMV;
                                                        let BQK = V - B;
                                                        let BQL = BMX + (BQK * BMV);
                                                        let BQM = V * BMZ;
                                                        let BQN = BMY + (BQK * BMZ);
                                                        BRA = BQM;
                                                        BRP = BQN;
                                                        BSI = BQJ;
                                                        BSZ = BQL;
                                                    } else {
                                                        BRA = A;
                                                        BRP = A;
                                                        BSI = A;
                                                        BSZ = A;
                                                    }
                                                    BQZ = BRA;
                                                    BRO = BRP;
                                                    BSH = BSI;
                                                    BSY = BSZ;
                                                }
                                                BQY = BQZ;
                                                BRN = BRO;
                                                BSG = BSH;
                                                BSX = BSY;
                                            }
                                            BQX = BQY;
                                            BRM = BRN;
                                            BSF = BSG;
                                            BSW = BSX;
                                        }
                                        BQW = BQX;
                                        BRL = BRM;
                                        BSE = BSF;
                                        BSV = BSW;
                                    }
                                    BQV = BQW;
                                    BRK = BRL;
                                    BSD = BSE;
                                    BSU = BSV;
                                }
                                BQU = BQV;
                                BRJ = BRK;
                                BSC = BSD;
                                BST = BSU;
                            }
                            BQT = BQU;
                            BRI = BRJ;
                            BSB = BSC;
                            BSS = BST;
                        }
                        BQS = BQT;
                        BRH = BRI;
                        BSA = BSB;
                        BSR = BSS;
                    }
                    BQR = BQS;
                    BRG = BRH;
                    BRZ = BSA;
                    BSQ = BSR;
                }
                BQQ = BQR;
                BRF = BRG;
                BRY = BRZ;
                BSP = BSQ;
            }
            let BRB = if BQO != 0.0 {
                let BQP = (parameters[24] * R) * O;
                BQP
            } else {
                BQQ
            };
            let BRC = if BRB < A { 1.0 } else { 0.0 };
            let BTB = if BRC != 0.0 {
                A
            } else {
                BRB
            };
            let BRQ = if BRD != 0.0 {
                let BRE = (parameters[25] * R) * O;
                BRE
            } else {
                BRF
            };
            let BRR = if BRQ < A { 1.0 } else { 0.0 };
            let BTJ = if BRR != 0.0 {
                A
            } else {
                BRQ
            };
            let BTC;
            if BRS != 0.0 {
                let BRU = if BRT == A { 1.0 } else { 0.0 };
                let BTD = if BRU != 0.0 {
                    let BRW = BRV * R;
                    BRW
                } else {
                    let BRX = if ((BRV * R) - (BN * V)) >= A { ((BRV * R) - (BN * V)) } else { A };
                    BRX
                };
                BTC = BTD;
            } else {
                let BSJ = if BRY < A { 1.0 } else { 0.0 };
                let BTE = if BSJ != 0.0 {
                    A
                } else {
                    BRY
                };
                BTC = BTE;
            }
            let BTK;
            if BSK != 0.0 {
                let BSL = if BRT == A { 1.0 } else { 0.0 };
                let BTL = if BSL != 0.0 {
                    let BSN = BSM * R;
                    BSN
                } else {
                    let BSO = if ((BSM * R) - (BN * V)) >= A { ((BSM * R) - (BN * V)) } else { A };
                    BSO
                };
                BTK = BTL;
            } else {
                let BTA = if BSP < A { 1.0 } else { 0.0 };
                let BTM = if BTA != 0.0 {
                    A
                } else {
                    BSP
                };
                BTK = BTM;
            }
            let BTF = BN * V;
            let BTG = ((BTB * BLU) + (BTC * BLV)) + (BTF * BLW);
            let BTH = if BTG > A { 1.0 } else { 0.0 };
            if BTH != 0.0 {
                let BTI = if ((parameters[729] / BTG) - AAP) < -1e1f64 { 1.0 } else { 0.0 };
                if BTI != 0.0 {
                } else {
                }
            } else {
            }
            let BTN = ((BTJ * BLY) + (BTK * BLZ)) + (BTF * BMA);
            let BTO = if BTN > A { 1.0 } else { 0.0 };
            if BTO != 0.0 {
                let BTP = if ((parameters[730] / BTN) - AAP) < -1e1f64 { 1.0 } else { 0.0 };
                if BTP != 0.0 {
                } else {
                }
            } else {
            }
            let BTT = if (if (if BTQ > A { 1.0 } else { 0.0 }) != 0.0 && (if BTR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if V == B { 1.0 } else { 0.0 }) != 0.0 || (if (if V > B { 1.0 } else { 0.0 }) != 0.0 && (if BTS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BWQ;
            let BWS;
            let BYZ;
            let BZJ;
            let CBL;
            let DZQ;
            let EAW;
            let EAY;
            if BTT != 0.0 {
                let BTU = T.powf(parameters[921]);
                let BTV = W + parameters[914];
                let BTW = BTV.powf(parameters[922]);
                let BTX = T.powf(parameters[927]);
                let BTY = BTV.powf(parameters[928]);
                let BTZ = B + (((parameters[924] / BTX) + (parameters[925] / BTY)) + (parameters[926] / (BTX * BTY)));
                let BUA = ((B + (((parameters[918] / BTU) + (parameters[919] / BTW)) + (parameters[920] / (BTU * BTW)))) * (B + (parameters[917] * BCR))) + AW;
                let mut BUB = 0.0;
                let mut BUG = 0.0;
                let mut BUI = 0.0;
                BUB = A;
                BUG = A;
                BUI = A;
                loop {
                    let BUC = if BUB < V { 1.0 } else { 0.0 };
                    if BUC == 0.0 {
                        break;
                    }
                    let BUD = B / V;
                    let BUE = LP * P;
                    let BUF = BUB * (BTS + P);
                    let BUH = BUG + (BUD / ((BTQ + BUE) + BUF));
                    let BUJ = BUI + (BUD / ((BTR + BUE) + BUF));
                    let BUK = BUB + B;
                    BUB = BUK;
                    BUG = BUH;
                    BUI = BUJ;
                }
                let BUL = LP * P;
                let BUM = (B / (parameters[912] + BUL)) + (B / (parameters[913] + BUL));
                let BUN = parameters[915] / BUA;
                let BUO = BUN * BUM;
                let BUP = BUG + BUI;
                let BUQ = BUN * BUP;
                let BUS = BUP - BUM;
                let BUT = (parameters[923] / BTZ) * BUS;
                let BUU = BTZ.powf(parameters[930]);
                let BUV = BTZ.powf(parameters[932]);
                let BUW = BDK * ((B + BUQ) / (B + BUO));
                let BUY = BUX * ((B + (BUQ * BUR)) / (B + (BUO * BUR)));
                let BUZ = OB + ((parameters[929] / BUU) * BUS);
                let BVA = BCZ + ((parameters[931] / BUV) * BUS);
                let BVC = if BVB == B { 1.0 } else { 0.0 };
                let BVG;
                let BVI;
                let EAX;
                if BVC != 0.0 {
                    let BVD = (IA / BTZ) * BUS;
                    let BVE = (ID / BUU) * BUS;
                    let BVF = (IE / BUV) * BUS;
                    BVG = BVE;
                    BVI = BVF;
                    EAX = BVD;
                } else {
                    BVG = A;
                    BVI = A;
                    EAX = A;
                }
                let BVH = HZ + BVG;
                let BVJ = HR + BVI;
                BWQ = BUW;
                BWS = BUZ;
                BYZ = BVA;
                BZJ = BUY;
                CBL = BUT;
                DZQ = BVJ;
                EAW = EAX;
                EAY = BVH;
            } else {
                BWQ = BDK;
                BWS = OB;
                BYZ = BCZ;
                BZJ = BUX;
                CBL = A;
                DZQ = HR;
                EAW = A;
                EAY = HZ;
            }
            let BVK = if parameters[43] == B { 1.0 } else { 0.0 };
            let BWD;
            let BWG;
            let BWJ;
            if BVK != 0.0 {
                let BVL = Q / V;
                let BVP = if (if (if (if parameter_given[20] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[21] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[22] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BWE;
                let BWH;
                let BWK;
                if BVP != 0.0 {
                    let BVR = if (if parameter_given[23] { 1.0 } else { 0.0 }) != 0.0 && (if BVQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BWF;
                    let BWI;
                    let BWL;
                    if BVR != 0.0 {
                        let BVS = BVQ + BVL;
                        let BVU = B / BVT;
                        let BVV = (BVT * BVT) / (BVQ * BVS);
                        let BVX = BKF * BVT;
                        let BVY = ((((BVW * BVQ) + BVX) * (rspice_limited_exp(((-1e1f64 * BVQ) * BVU)))) - (((BVW * BVS) + BVX) * (rspice_limited_exp(((-1e1f64 * BVS) * BVU))))) / BVL;
                        let BWA = 2.5e-3f64 * BVT;
                        let BWC = ((((BVZ * BVQ) + BWA) * (rspice_limited_exp(((-2e1f64 * BVQ) * BVU)))) - (((BVZ * BVS) + BWA) * (rspice_limited_exp(((-2e1f64 * BVS) * BVU))))) / BVL;
                        BWF = BVV;
                        BWI = BVY;
                        BWL = BWC;
                    } else {
                        BWF = BVM;
                        BWI = BVN;
                        BWL = BVO;
                    }
                    BWE = BWF;
                    BWH = BWI;
                    BWK = BWL;
                } else {
                    BWE = BVM;
                    BWH = BVN;
                    BWK = BVO;
                }
                BWD = BWE;
                BWG = BWH;
                BWJ = BWK;
            } else {
                BWD = A;
                BWG = A;
                BWJ = A;
            }
            let BWM = (BWD + (parameters[933] * BWG)) + (parameters[934] * BWJ);
            let BWN = FI * BWM;
            let BWO = IC * BWM;
            let BWP = IB * BWM;
            let BWR = BWQ * (B + (FK * BWM));
            let BWT = BWS + (FJ * BWM);
            let BWV = BCH * (node_potentials[9] - BWU);
            let BWX = BCH * (BWW - BWU);
            let BWZ = BCH * (BWY - BWU);
            let BXA = BWX - BWZ;
            let BXB = BCH * (node_potentials[12] - BWY);
            let BXD = BCH * (BXC - BWW);
            let BXE = BCH * (BXC - node_potentials[14]);
            let BXF = BWV - BWX;
            let BXG = BWV - BWZ;
            let BXI = BCH * (BXH - BWW);
            let BXJ = BXH - BWY;
            let BXL = if BXK != A { 1.0 } else { 0.0 };
            let BXM = if parameters[1095] == B { 1.0 } else { 0.0 };
            let BXN = if (if (if BXL != 0.0 && OO != 0.0 { 1.0 } else { 0.0 }) != 0.0 && BXM != 0.0 { 1.0 } else { 0.0 }) != 0.0 && BFP != 0.0 { 1.0 } else { 0.0 };
            let BXS;
            let CVU;
            let DIB;
            if BXN != 0.0 {
                let BXP = BWX + ((BCH * (B - (parameters[1111] / BXK))) * (BXO - BWW));
                let BXQ = (BXD + BWX) - BXP;
                let BXR = (BXI + BWX) - BXP;
                BXS = BXP;
                CVU = BXR;
                DIB = BXQ;
            } else {
                BXS = BWX;
                CVU = BXI;
                DIB = BXD;
            }
            let BXT = if BXA < A { 1.0 } else { 0.0 };
            let BXV;
            let BXW;
            let BXY;
            let BXZ;
            let CUH;
            if BXT != 0.0 {
                BXV = BWZ;
                BXW = BWX;
                BXY = BWZ;
                BXZ = BXS;
                CUH = BXU;
            } else {
                BXV = BWX;
                BXW = BWZ;
                BXY = BXS;
                BXZ = BWZ;
                CUH = B;
            }
            let BXX = BXV - BXW;
            let BYA = BXY - BXZ;
            let BYC = BYB * BYA;
            let BYE = if BYC > BYD { 1.0 } else { 0.0 };
            let BYI;
            if BYE != 0.0 {
                BYI = BYC;
            } else {
                let BYF = if BYC < -3.7e1f64 { 1.0 } else { 0.0 };
                let BYJ = if BYF != 0.0 {
                    let BYG = BYC.exp();
                    BYG
                } else {
                    let BYH = (B + (BYC.exp())).ln();
                    BYH
                };
                BYI = BYJ;
            }
            let BYK = AT / BYB;
            let BYL = -(BXZ + (LP * (BYA - (((BYK * BYI) - BYA) - (BYK * 6.931471805599453e-1f64)))));
            let BYM = BYB * BXX;
            let BYN = if BYM > BYD { 1.0 } else { 0.0 };
            let BYR;
            if BYN != 0.0 {
                BYR = BYM;
            } else {
                let BYO = if BYM < -3.7e1f64 { 1.0 } else { 0.0 };
                let BYS = if BYO != 0.0 {
                    let BYP = BYM.exp();
                    BYP
                } else {
                    let BYQ = (B + (BYM.exp())).ln();
                    BYQ
                };
                BYR = BYS;
            }
            let BYT = ((BYK * BYR) - BXX) - (BYK * 6.931471805599453e-1f64);
            let BYU = -(BXW + (LP * (BXX - BYT)));
            let BYV = LP + (LP * (((parameters[1123] * BXA) / BBO).tanh()));
            let BYW = B - BYV;
            let CAL;
            let CAU;
            let CEV;
            let CEW;
            let CEX;
            let CEZ;
            let CFV;
            let CGD;
            let CGF;
            let CGG;
            let CMQ;
            let CNF;
            let DAA;
            let DAB;
            if IP != 0.0 {
                let BYX = (PR * BYW) + (JN * BYV);
                let BZA = (BYY * BYW) + (BYZ * BYV);
                let BZC = (BZB * BYW) + (LL * BYV);
                let BZE = (BZD * BYW) + (LV * BYV);
                let BZG = (BZF * BYW) + (MH * BYV);
                let BZK = (BZH * BYW) + (BZJ * BYV);
                let BZM = (BZL * BYW) + (BGI * BYV);
                let BZO = (BZN * BYW) + (BWR * BYV);
                let BZQ = (BZP * BYW) + (BDR * BYV);
                let BZS = (BZR * BYW) + (BDX * BYV);
                let BZU = (BZT * BYW) + (BEA * BYV);
                let BZW = (BZV * BYW) + (BED * BYV);
                let BZY = (BZX * BYW) + (MS * BYV);
                let CAA = (BZZ * BYW) + (BHC * BYV);
                CAL = BYX;
                CAU = BZA;
                CEV = BZW;
                CEW = BZQ;
                CEX = BZS;
                CEZ = BZU;
                CFV = BZG;
                CGD = BZM;
                CGF = BZO;
                CGG = BZK;
                CMQ = BZC;
                CNF = BZE;
                DAA = BZY;
                DAB = CAA;
            } else {
                CAL = JN;
                CAU = BYZ;
                CEV = BED;
                CEW = BDR;
                CEX = BDX;
                CEZ = BEA;
                CFV = MH;
                CGD = BGI;
                CGF = BWR;
                CGG = BZJ;
                CMQ = LL;
                CNF = LV;
                DAA = MS;
                DAB = BHC;
            }
            let CAB = BCL - BYU;
            let CAC = if 0.0f64 != 0.0 && (if CAB < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CAH = if CAC != 0.0 {
                let CAE = -1.0000000000000002e-2f64 / (CAD * CAB);
                CAE
            } else {
                let CAF = CAB - BVZ;
                let CAG = LP * ((CAB + BVZ) + (((CAF * CAF) + 2.5000000000000005e-3f64).sqrt()));
                CAG
            };
            let CAI = CAH.sqrt();
            let CAJ = BCP * CAI;
            let CAK = H / CAJ;
            let CAM = B + ((((DK + BCX) + (CAL * BYT)) - (JR * BYU)) / L);
            let CAN = if 0.0f64 != 0.0 && (if CAM < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CAR = if CAN != 0.0 {
                let CAO = -2.5000000000000005e-3f64 / (CAD * CAM);
                CAO
            } else {
                let CAP = CAM - B;
                let CAQ = LP * ((CAM + B) + (((CAP * CAP) + 6.250000000000001e-4f64).sqrt()));
                CAQ
            };
            let CAS = CAR * BBO;
            let CAT = B / CAS;
            let CAV = (-(CAU + (LI * BYU))) * BYT;
            let CAW = (LP * (CAV - (((CAV * CAV) + 6.25e-6f64).sqrt()))) + 1.25e-3f64;
            let CAX = ((HB + (parameters[869] / AU)) + (HC * BYU)) * ((BBR.powf(parameters[868])) - B);
            let CAY = if DO > A { 1.0 } else { 0.0 };
            let CBG;
            if CAY != 0.0 {
                let CAZ = (-DP) * BYT;
                let CBB = if CAZ < -8e1f64 { 1.0 } else { 0.0 };
                let CBE = if CBB != 0.0 {
                    CBC
                } else {
                    let CBD = rspice_limited_exp(CAZ);
                    CBD
                };
                let CBF = (-CAS) * ((if (AU / (AU + (DO * (B + CBE)))) >= AXS { (AU / (AU + (DO * (B + CBE)))) } else { AXS }).ln());
                CBG = CBF;
            } else {
                CBG = A;
            }
            let CBI = NN + CBH;
            let CBJ = BXW * CAT;
            let CBK = CAI - BCM;
            let CBM = ((BWV * CAT) - (CBI * CAT)) - ((((((CAW + (CBG - ((DT + (DQ / (AU.powf(DR)))) * ((DS * BYT).tanh())))) + ((NY * CBK) - (BWT * BYU))) - CAX) + CBL) + BWN) * CAT);
            let CBN = ((((3.20438e-19f64 * H) * JH) * BBP).sqrt()) / L;
            let CBO = (AT * BCK) + (BXW * BBP);
            let CBP = if CBO < -1e1f64 { 1.0 } else { 0.0 };
            let CBS = if CBP != 0.0 {
                let CBQ = -1e-6f64 / CBO;
                CBQ
            } else {
                let CBR = LP * (CBO + (((CBO * CBO) + 4e-6f64).sqrt()));
                CBR
            };
            let CBT = AT * (CBS.sqrt());
            let CBU = B + (CBN / CBT);
            let CBV = if (((CBO + -6.931471805599453e-1f64) + 1e0f64) + ((if (((AT * CBU) / CBN) * ((CBU / CBN) + CBT)) >= AXS { (((AT * CBU) / CBN) * ((CBU / CBN) + CBT)) } else { AXS }).ln())) < -1e1f64 { 1.0 } else { 0.0 };
            if CBV != 0.0 {
            } else {
            }
            let CBW = ((((3.20438e-19f64 * H) * JH) * CAT).sqrt()) / L;
            let CBX = B / CBW;
            let CBY = BCK / CAR;
            let CCA = (LP * CBM) - (RZ * (B + (CBW / CBZ)));
            let CCB = CCA + (((CCA * CCA) + (SB * CBM)).sqrt());
            let CCC = if CBM < A { 1.0 } else { 0.0 };
            let CCJ = if CCC != 0.0 {
                let CCD = (CBM - CCB) / CBW;
                let CCE = -((if ((B - CCB) + (CCD * CCD)) >= AXS { ((B - CCB) + (CCD * CCD)) } else { AXS }).ln());
                CCE
            } else {
                let CCF = rspice_limited_exp((-CCB));
                let CCG = LP * CBW;
                let CCH = ((((CBM - B) + CCF) + (CCG * CCG)).sqrt()) - CCG;
                let CCI = ((CCH * CCH) + B) - CCF;
                CCI
            };
            let CCK = CCJ + B;
            let CCL = CCJ - B;
            let CCM = CCL * CCL;
            let CCN = (LP * (CCK + ((CCM + 1e0f64).sqrt()))).sqrt();
            let CCO = AT * CCN;
            let CCP = (B + (CBW / CCO)) / CBW;
            let CCQ = CCJ - (AT * CBY);
            let CCR = CCQ - CBJ;
            let CCS = CCR - ((if ((SA * CCP) * CCN) >= AXS { ((SA * CCP) * CCN) } else { AXS }).ln());
            let CCW = LP * ((CCS - CCT) - (((CCS * (CCS + CCU)) + CCV).sqrt()));
            let CCX = if CCW <= -6.8e1f64 { 1.0 } else { 0.0 };
            let CEC;
            if CCX != 0.0 {
                let CCZ = if CCW < -1.1e2f64 { 1.0 } else { 0.0 };
                let CDH;
                if CCZ != 0.0 {
                    CDH = CDA;
                } else {
                    let CDB = if CCW > -9e1f64 { 1.0 } else { 0.0 };
                    let CDI = if CDB != 0.0 {
                        let CDC = rspice_limited_exp(CCW);
                        CDC
                    } else {
                        let CDD = (CCW - CCY) / BWB;
                        let CDE = CDD * CDD;
                        let CDG = rspice_limited_exp((CCY + (BWB * ((7.8125e-2f64 + (LP * CDD)) + (CDE * (9.375e-1f64 - (CDE * (CDF - CDE))))))));
                        CDG
                    };
                    CDH = CDI;
                }
                let CDJ = CDH * (((B + CCR) - CCW) - ((if ((AT * CCP) * (((CDH * AT) * CCP) + CCO)) >= AXS { ((AT * CCP) * (((CDH * AT) * CCP) + CCO)) } else { AXS }).ln()));
                CEC = CDJ;
            } else {
                let CDK = rspice_limited_exp(CCW);
                let CDL = AT * CDK;
                let CDM = CDL * CCP;
                let CDN = CCP + (B / CCN);
                let CDO = CDK - (((CDL + ((if (CDM * (CDM + CCO)) >= AXS { (CDM * (CDM + CCO)) } else { AXS }).ln())) - CCR) / ((AT + (1e0f64 / CDK)) + (CDN / ((CCP * CDK) + CCN))));
                let CDP = AT * CDO;
                let CDQ = CDP * CCP;
                let CDR = (CDP + ((if (CDQ * (CDQ + CCO)) >= AXS { (CDQ * (CDQ + CCO)) } else { AXS }).ln())) - CCR;
                let CDS = (CCP * CDO) + CCN;
                let CDT = CDN / CDS;
                let CDU = (AT + (1e0f64 / CDO)) + CDT;
                let CDV = B / CDO;
                let CDW = CDO - ((CDR / CDU) * (B + ((CDR * (((-1e0f64 * (CDV * CDV)) - (1e0f64 / (((CCN * CCN) * CCN) * CDS))) - (CDT * CDT))) / ((AT * CDU) * CDU))));
                CEC = CDW;
            }
            let CDX = if 0.0f64 != 0.0 && (if CCJ < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CEA = if CDX != 0.0 {
                let CDY = -4e0f64 / (CAD * CCJ);
                CDY
            } else {
                let CDZ = LP * (CCK + ((CCM + 1e0f64).sqrt()));
                CDZ
            };
            let CEB = CEA.sqrt();
            let CED = AT * CEC;
            let CEE = CCJ - CED;
            let CEF = if 0.0f64 != 0.0 && (if CEE < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CEJ = if CEF != 0.0 {
                let CEG = -4e0f64 / (CAD * CEE);
                CEG
            } else {
                let CEH = CEE - B;
                let CEI = LP * ((CEE + B) + (((CEH * CEH) + 1e0f64).sqrt()));
                CEI
            };
            let CEK = B + (CBW / (CEB + (CEJ.sqrt())));
            let CEL = M * K;
            let CEM = 1e-8f64 / CEL;
            let CEN = CBM - CCJ;
            let CEO = CEK - B;
            let CEP = CAS * (CEN - (CED * CEO));
            let CEQ = if 1.0f64 != 0.0 && (if CEP < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CEU = if CEQ != 0.0 {
                let CER = -1.0000000000000002e-2f64 / (CAD * CEP);
                CER
            } else {
                let CES = LP * (CEP + (((CEP * CEP) + 2.5000000000000005e-3f64).sqrt()));
                CES
            };
            let CET = ((AT * CEK) * CAS) * CEC;
            let CEY = CEW + (CEX * BYU);
            let CFA = B + ((CEY * ((CEM * (CEU + (BDH * CET))).powf(BEK))) + (CEZ / ((LP * (B + (CET / CEU))).powf(CEV))));
            let CFB = if 0.0f64 != 0.0 && (if CFA < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CFU = if CFB != 0.0 {
                let CFC = -2.25e-6f64 / (CAD * CFA);
                CFC
            } else {
                let CFD = CFA - B;
                let CFE = LP * ((CFA + B) + (((CFD * CFD) + 5.625e-7f64).sqrt()));
                CFE
            };
            let CFF = B / (((AY * AXR).powf(EN)) * V);
            let CGJ;
            if OO != 0.0 {
                CGJ = A;
            } else {
                let CFG = (B / (B + (EL * CET))) + (OD * CBK);
                let CFH = CFG + (((CFG * CFG) + BKF).sqrt());
                let CGK = if AWW != 0.0 {
                    let CFM = (((CFI + (CFK * CFH)) * CFF) * V) * BFJ;
                    CFM
                } else {
                    let CFT = ((CFN + (((CFI + (CFK * CFH)) * CFF) * V)) + CFQ) * BFJ;
                    CFT
                };
                CGJ = CGK;
            }
            let CFW = B / CFV;
            let CFX = CFU.powf(CFW);
            let CFY = HD * BYU;
            let CFZ = B - CFY;
            let CGA = LP * (CFZ + (((CFZ * CFZ) + ((BVW + (CFY * CFY)).sqrt())).sqrt()));
            let CGB = AAP * parameters[433];
            let CGC = ((CGB * CEC) * CGA) / (CGB + (CEC * CGA));
            let CGE = if CGD < A { 1.0 } else { 0.0 };
            let CGM = if CGE != 0.0 {
                let CGH = (AT * (((CGF / CFX) * CAS) / (CGG * AU))) * (B / (B - (CGD * CGC)));
                CGH
            } else {
                let CGI = (AT * (((CGF / CFX) * CAS) / (CGG * AU))) * (B + (CGD * CGC));
                CGI
            };
            let CGL = if CGJ > A { 1.0 } else { 0.0 };
            let CJC;
            if CGL != 0.0 {
                let CGN = (((((((AY * AT) * CEK) * L) * CAS) * CGG) * CGM) * CGJ) / (AT * CAS);
                let CGO = LP * CGM;
                let CGP = (CEC * CEC) + CEC;
                let CGQ = (CGO * CGP) / (B + (CGO * (B + CEC)));
                let CGR = AT * CGM;
                let CGS = CGR * (CEC - CGQ);
                let CGT = CGS * CGS;
                let CGU = (B + CGT).sqrt();
                let CGV = if CGS != A { 1.0 } else { 0.0 };
                let CGZ;
                let CHB;
                if CGV != 0.0 {
                    let CGW = CGS.asinh();
                    let CGX = CGU + ((B / CGS) * CGW);
                    CGZ = CGX;
                    CHB = CGW;
                } else {
                    let CGY = CGU + (B / CGU);
                    CGZ = CGY;
                    CHB = A;
                }
                let CHA = ((CGQ * CGZ) + ((CGN * CGQ) * ((CEC + CGQ) + B))) - (CGM * (CGP - ((CGQ * CGQ) + CGQ)));
                let CHE = if CGV != 0.0 {
                    let CHC = ((-2e0f64 * CGM) * ((CGS * CGU) - CHB)) / CGT;
                    CHC
                } else {
                    let CHD = (-2e0f64 * CGM) * (CGS / CGU);
                    CHD
                };
                let CHF = AT * CGQ;
                let CHG = CGQ - (CHA / ((((CGQ * CHE) + CGZ) + (CGN * ((CEC + CHF) + B))) + (CGM * (CHF + B))));
                let CHH = CGR * (CEC - CHG);
                let CHI = CHH * CHH;
                let CHJ = (B + CHI).sqrt();
                let CHK = if CHH != A { 1.0 } else { 0.0 };
                let CHO;
                let CHQ;
                if CHK != 0.0 {
                    let CHL = CHH.asinh();
                    let CHM = CHJ + ((B / CHH) * CHL);
                    CHO = CHM;
                    CHQ = CHL;
                } else {
                    let CHN = CHJ + (B / CHJ);
                    CHO = CHN;
                    CHQ = CHB;
                }
                let CHP = ((CHG * CHO) + ((CGN * CHG) * ((CEC + CHG) + B))) - (CGM * (CGP - ((CHG * CHG) + CHG)));
                let CHT = if CHK != 0.0 {
                    let CHR = ((-2e0f64 * CGM) * ((CHH * CHJ) - CHQ)) / CHI;
                    CHR
                } else {
                    let CHS = (-2e0f64 * CGM) * (CHH / CHJ);
                    CHS
                };
                let CHU = AT * CHG;
                let CHV = CHG - (CHP / ((((CHG * CHT) + CHO) + (CGN * ((CEC + CHU) + B))) + (CGM * (CHU + B))));
                CJC = CHV;
            } else {
                let CHW = LP * CGM;
                let CHX = (CEC * CEC) + CEC;
                let CHY = (CHW * CHX) / (B + (CHW * (B + CEC)));
                let CHZ = AT * CGM;
                let CIA = CHZ * (CEC - CHY);
                let CIB = CIA * CIA;
                let CIC = (B + CIB).sqrt();
                let CID = if CIA != A { 1.0 } else { 0.0 };
                let CIH;
                let CIJ;
                if CID != 0.0 {
                    let CIE = CIA.asinh();
                    let CIF = CIC + ((B / CIA) * CIE);
                    CIH = CIF;
                    CIJ = CIE;
                } else {
                    let CIG = CIC + (B / CIC);
                    CIH = CIG;
                    CIJ = A;
                }
                let CII = (CHY * CIH) - (CGM * (CHX - ((CHY * CHY) + CHY)));
                let CIM = if CID != 0.0 {
                    let CIK = ((-2e0f64 * CGM) * ((CIA * CIC) - CIJ)) / CIB;
                    CIK
                } else {
                    let CIL = (-2e0f64 * CGM) * (CIA / CIC);
                    CIL
                };
                let CIN = CHY - (CII / (((CHY * CIM) + CIH) + (CGM * ((AT * CHY) + B))));
                let CIO = CHZ * (CEC - CIN);
                let CIP = CIO * CIO;
                let CIQ = (B + CIP).sqrt();
                let CIR = if CIO != A { 1.0 } else { 0.0 };
                let CIV;
                let CIX;
                if CIR != 0.0 {
                    let CIS = CIO.asinh();
                    let CIT = CIQ + ((B / CIO) * CIS);
                    CIV = CIT;
                    CIX = CIS;
                } else {
                    let CIU = CIQ + (B / CIQ);
                    CIV = CIU;
                    CIX = CIJ;
                }
                let CIW = (CIN * CIV) - (CGM * (CHX - ((CIN * CIN) + CIN)));
                let CJA = if CIR != 0.0 {
                    let CIY = ((-2e0f64 * CGM) * ((CIO * CIQ) - CIX)) / CIP;
                    CIY
                } else {
                    let CIZ = (-2e0f64 * CGM) * (CIO / CIQ);
                    CIZ
                };
                let CJB = CIN - (CIW / (((CIN * CJA) + CIV) + (CGM * ((AT * CIN) + B))));
                CJC = CJB;
            }
            let CJD = AT * CJC;
            let CJE = (CJD * CEK) * CBX;
            let CJF = (CCQ - (CJD + ((if (CJE * (CJE + (CBW / CEO))) >= AXS { (CJE * (CJE + (CBW / CEO))) } else { AXS }).ln()))) * CAS;
            let CJI = if (if CJG == A { 1.0 } else { 0.0 }) != 0.0 && (if CJH == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CJU;
            if CJI != 0.0 {
                CJU = B;
            } else {
                let CJJ = AU / (AU + ((DW * CAJ).sqrt()));
                let CJK = B + (((CJG * CJJ) - (((CJH * CJJ) * (CEC.powf(parameters[1132]))) * CAS)) / (B + (parameters[1133] * BYU)));
                let CJL = if 0.0f64 != 0.0 && (if CJK < -1.25e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CJV = if CJL != 0.0 {
                    let CJM = -2.5e-7f64 / (CAD * CJK);
                    CJM
                } else {
                    let CJN = CJK - BVW;
                    let CJO = LP * ((CJK + BVW) + (((CJN * CJN) + 6.25e-8f64).sqrt()));
                    CJO
                };
                CJU = CJV;
            }
            let CJP = CJF - BXW;
            let CJQ = if 1.0f64 != 0.0 && (if CJP < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CJT = if CJQ != 0.0 {
                let CJR = -1e-6f64 / (CAD * CJP);
                CJR
            } else {
                let CJS = LP * (CJP + (((CJP * CJP) + 2.5e-7f64).sqrt()));
                CJS
            };
            let CJW = CJT / CJU;
            let CJX = B / BGC;
            let CJY = -BGC;
            let CJZ = BXX * ((B + (((BXX / CJW) + BP).powf(CJX))).powf(CJY));
            let CKA = (LP * (CCK + ((CCM + 1e0f64).sqrt()))).sqrt();
            let CKB = AT * CKA;
            let CKC = (B + (CBW / CKB)) / CBW;
            let CKD = CCQ - ((CJZ + BXW) * CAT);
            let CKE = CKD - ((if ((SA * CKC) * CKA) >= AXS { ((SA * CKC) * CKA) } else { AXS }).ln());
            let CKF = LP * ((CKE - CCT) - (((CKE * (CKE + CCU)) + CCV).sqrt()));
            let CKG = if CKF <= -6.8e1f64 { 1.0 } else { 0.0 };
            let CLF;
            if CKG != 0.0 {
                let CKI = if CKF < -1.1e2f64 { 1.0 } else { 0.0 };
                let CKP;
                if CKI != 0.0 {
                    CKP = CKJ;
                } else {
                    let CKK = if CKF > -9e1f64 { 1.0 } else { 0.0 };
                    let CKQ = if CKK != 0.0 {
                        let CKL = rspice_limited_exp(CKF);
                        CKL
                    } else {
                        let CKM = (CKF - CKH) / BWB;
                        let CKN = CKM * CKM;
                        let CKO = rspice_limited_exp((CKH + (BWB * ((7.8125e-2f64 + (LP * CKM)) + (CKN * (9.375e-1f64 - (CKN * (CDF - CKN))))))));
                        CKO
                    };
                    CKP = CKQ;
                }
                let CKR = CKP * (((B + CKD) - CKF) - ((if ((AT * CKC) * (((CKP * AT) * CKC) + CKB)) >= AXS { ((AT * CKC) * (((CKP * AT) * CKC) + CKB)) } else { AXS }).ln()));
                CLF = CKR;
            } else {
                let CKS = rspice_limited_exp(CKF);
                let CKT = AT * CKS;
                let CKU = CKT * CKC;
                let CKV = CKC + (B / CKA);
                let CKW = CKS - (((CKT + ((if (CKU * (CKU + CKB)) >= AXS { (CKU * (CKU + CKB)) } else { AXS }).ln())) - CKD) / ((AT + (1e0f64 / CKS)) + (CKV / ((CKC * CKS) + CKA))));
                let CKX = AT * CKW;
                let CKY = CKX * CKC;
                let CKZ = (CKX + ((if (CKY * (CKY + CKB)) >= AXS { (CKY * (CKY + CKB)) } else { AXS }).ln())) - CKD;
                let CLA = (CKC * CKW) + CKA;
                let CLB = CKV / CLA;
                let CLC = (AT + (1e0f64 / CKW)) + CLB;
                let CLD = B / CKW;
                let CLE = CKW - ((CKZ / CLC) * (B + ((CKZ * (((-1e0f64 * (CLD * CLD)) - (1e0f64 / (((CKA * CKA) * CKA) * CLA))) - (CLB * CLB))) / ((AT * CLC) * CLC))));
                CLF = CLE;
            }
            let CLG = ((CCJ - CEC) - CLF) - B;
            let CLH = if 0.0f64 != 0.0 && (if CLG < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CLL = if CLH != 0.0 {
                let CLI = -4e0f64 / (CAD * CLG);
                CLI
            } else {
                let CLJ = CLG - B;
                let CLK = LP * ((CLG + B) + (((CLJ * CLJ) + 1e0f64).sqrt()));
                CLK
            };
            let CLM = B + (CBW / (CKA + (CLL.sqrt())));
            let CLN = CEC - CLF;
            let CLO = CLN * CLN;
            let CLP = (B + CEC) + CLF;
            let CLQ = B / CLP;
            let CLR = CLO * CLQ;
            let CLS = CEC + CLF;
            let CLT = BDD * CLM;
            let CLU = CLR * CLQ;
            let CLX = CLT * ((CED + CLF) + ((LP * ((B + (CLV * CEC)) + (CLW * CLF))) * CLU));
            let CLY = CLT * ((CEC + (AT * CLF)) + ((LP * ((B + (CLW * CEC)) + (CLV * CLF))) * CLU));
            let CLZ = CAS * (CEN - ((CLM - B) * (CLS + (BDD * CLR))));
            let CMA = if 1.0f64 != 0.0 && (if CLZ < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CME = if CMA != 0.0 {
                let CMB = -1.0000000000000002e-2f64 / (CAD * CLZ);
                CMB
            } else {
                let CMC = LP * (CLZ + (((CLZ * CLZ) + 2.5000000000000005e-3f64).sqrt()));
                CMC
            };
            let CMD = CAS * (CLX + CLY);
            let CMF = B + ((CEY * ((CEM * (CME + (BDH * CMD))).powf(BEK))) + (CEZ / ((LP * (B + (CMD / CME))).powf(CEV))));
            let CMG = if 0.0f64 != 0.0 && (if CMF < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CML = if CMG != 0.0 {
                let CMH = -2.25e-6f64 / (CAD * CMF);
                CMH
            } else {
                let CMI = CMF - B;
                let CMJ = LP * ((CMF + B) + (((CMI * CMI) + 5.625e-7f64).sqrt()));
                CMJ
            };
            let CMK = AT * CGG;
            let CMM = (CMK / (CGF / CML)) * AU;
            let CMN = if FA > A { 1.0 } else { 0.0 };
            let CNA = if CMN != 0.0 {
                let CMO = B + ((FA * CMD) / CMM);
                CMO
            } else {
                let CMP = B / (B - ((FA * CMD) / CMM));
                CMP
            };
            let CMR = BXX - CJZ;
            let CMS = CMD + (AT * CAS);
            let CMT = if CMQ > A { 1.0 } else { 0.0 };
            let CNT;
            if CMT != 0.0 {
                let CMU = CMS / (CJW + CMS);
                let CMV = B + (EU * BYU);
                let CMW = if CMV < -1e1f64 { 1.0 } else { 0.0 };
                let CMZ = if CMW != 0.0 {
                    let CMX = -1e-6f64 / CMV;
                    CMX
                } else {
                    let CMY = LP * (CMV + (((CMV * CMV) + 4e-6f64).sqrt()));
                    CMY
                };
                let CNB = B + (CMR / ((((CMS / CMQ) * CMU) * CNA) * (B / CMZ)));
                CNT = CNB;
            } else {
                CNT = B;
            }
            let CNC = if LS <= A { 1.0 } else { 0.0 };
            let CNJ = if CNC != 0.0 {
                B
            } else {
                let CND = B / (B + ((LS * (AU.sqrt())) / CMS));
                CND
            };
            let CNE = CJW + CMM;
            let CNG = if CNF > A { 1.0 } else { 0.0 };
            let CNU;
            if CNG != 0.0 {
                let CNI = if CNH < A { 1.0 } else { 0.0 };
                let CNM = if CNI != 0.0 {
                    let CNK = (CNF / (B - ((CNH * CMD) / CMM))) / CNJ;
                    CNK
                } else {
                    let CNL = (CNF * (B + ((CNH * CMD) / CMM))) / CNJ;
                    CNL
                };
                let CNN = B + (CNM * ((if (B + ((CMR / CNM) / CNE)) >= AXS { (B + ((CMR / CNM) / CNE)) } else { AXS }).ln()));
                CNU = CNN;
            } else {
                let CNO = if CNH < A { 1.0 } else { 0.0 };
                let CNR = if CNO != 0.0 {
                    let CNP = (CNF / (B - ((CNH * CMD) / CMM))) / CNJ;
                    CNP
                } else {
                    let CNQ = (CNF * (B + ((CNH * CMD) / CMM))) / CNJ;
                    CNQ
                };
                let CNS = B + CNR;
                CNU = CNS;
            }
            let CNV = CNT * CNU;
            let CNW = rspice_limited_exp((EY * BXX));
            let CNX = if EX > A { 1.0 } else { 0.0 };
            let COA = if CNX != 0.0 {
                let CNY = ((B + ((B + (parameters[369] * AU)) * CNW)) / EX) * CNJ;
                CNY
            } else {
                CNZ
            };
            let COB = CNV * (B + (CMR / COA));
            let COC = if EW > A { 1.0 } else { 0.0 };
            let COH;
            if COC != 0.0 {
                let COD = EV * BCQ;
                let COE = if CMR > (COD / CBA) { 1.0 } else { 0.0 };
                let COI = if COE != 0.0 {
                    let COF = (AU * (rspice_limited_exp((COD / CMR)))) / EW;
                    COF
                } else {
                    let COG = (CNZ * AU) / EW;
                    COG
                };
                COH = COI;
            } else {
                COH = CNZ;
            }
            let COJ = COB * (B + (CMR / COH));
            let COK = CML.powf(CFW);
            let COL = ((CGB * CMD) * CGA) / (CGB + (CMD * CGA));
            let COO = if CGE != 0.0 {
                let COM = (AT * (((CGF / COK) * CAS) / (CGG * AU))) * (B / (B - (CGD * COL)));
                COM
            } else {
                let CON = (AT * (((CGF / COK) * CAS) / (CGG * AU))) * (B + (CGD * COL));
                CON
            };
            let COP = (AT * COO) * CLN;
            let COQ = (B + (COP * COP)).sqrt();
            let COR = if COP != A { 1.0 } else { 0.0 };
            let COU = if COR != 0.0 {
                let COS = LP * (COQ + ((B / COP) * (COP.asinh())));
                COS
            } else {
                let COT = LP * (COQ + (B / COQ));
                COT
            };
            let CQC;
            let DOY;
            let EFW;
            let EGH;
            if OO != 0.0 {
                let COW = BCH * (COV - BWU);
                let COY = (BWV - COW) - COX;
                let COZ = (B / (B + (EL * (LP * (COY + (((COY * COY) + BKF).sqrt())))))) + (OD * COW);
                let CPE = BFJ * (CFN + ((CPA + (CPC * (LP * (COZ + (((COZ * COZ) + BKF).sqrt()))))) * CFF));
                let CPF = BCH * (BXO - BWU);
                let CPG = (BWV - CPF) - COX;
                let CPH = (B / (B + (EL * (LP * (CPG + (((CPG * CPG) + BKF).sqrt())))))) + (OD * CPF);
                let CPM = BFJ * (CFQ + ((CPI + (CPK * (LP * (CPH + (((CPH * CPH) + BKF).sqrt()))))) * CFF));
                CQC = B;
                DOY = A;
                EFW = CPM;
                EGH = CPE;
            } else {
                let CPN = (B / (B + (EL * CMD))) + (OD * CBK);
                let CPO = CFI + (CFK * (LP * (CPN + (((CPN * CPN) + BKF).sqrt()))));
                let CPP = ((BFJ * CPO) * CFF) * V;
                let CPQ = ((((CGF / (COU * CML)) * L) * AY) / AU) * CMD;
                let CPR = B + (CPQ * CPP);
                let CPS = if ON == AT { 1.0 } else { 0.0 };
                let CQD;
                let DOZ;
                let EFX;
                let EGI;
                if CPS != 0.0 {
                    let CPT = BFJ * ((CFN + ((CPO * CFF) * V)) + CFQ);
                    let CPU = B + (CPQ * CPT);
                    CQD = CPU;
                    DOZ = CPT;
                    EFX = A;
                    EGI = A;
                } else {
                    CQD = CPR;
                    DOZ = CPP;
                    EFX = CFQ;
                    EGI = CFN;
                }
                CQC = CQD;
                DOY = DOZ;
                EFW = EFX;
                EGH = EGI;
            }
            let CPV = (AT * CAR) * BBO;
            let CPW = ((((BGU + (BHA / (CMD + CPV))) * CLN) * CLN) + B) - AYY;
            let CPX = LP * (B + ((B + (-1e0f64 + (LP * (CPW + (((CPW * CPW) + 4e-3f64).sqrt()))))).sqrt()));
            let CPY = CPX - B;
            let CPZ = CLN / (CLS + BIE);
            let CQA = B + ((BHX * CPZ) * CPZ);
            let CQB = rspice_limited_exp((-(BIK / (((if A >= (BIQ + ((BIW * CLN) * CLN)) { A } else { (BIQ + ((BIW * CLN) * CLN)) }) * CLS) + CPV))));
            let CQE = CGF / ((CML * COU) * CQC);
            let CQF = AT * V;
            let CQG = ((((((((((((CQF * CLM) * CQE) * AY) / AU) * L) * CAS) * CAS) * (CLN * CLP)) * COJ) / ((LP * ((CPX + B) - (((CPY * CPY) + 2.5e-5f64).sqrt()))) + 2.5e-3f64)) * CQA) * CQB) * parameters[36];
            let CQH = if OO != 0.0 && BFP != 0.0 { 1.0 } else { 0.0 };
            let DAE;
            let DBO;
            let EGB;
            let EGM;
            if CQH != 0.0 {
                let CQJ = BBO * (((JH * CQI) / (BBZ * BBZ)).ln());
                let CRT = if BBE != 0.0 {
                    let CQK = BBO * (((CQJ * CQJ) + BP).sqrt());
                    CQK
                } else {
                    CQJ
                };
                let CQL = B - (parameters[1113] * BWZ);
                let CQM = if 1.0f64 != 0.0 && (if CQL < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CSJ = if CQM != 0.0 {
                    let CQN = -1e-6f64 / (CAD * CQL);
                    CQN
                } else {
                    let CQO = LP * (CQL + (((CQL * CQL) + 2.5e-7f64).sqrt()));
                    CQO
                };
                let CQP = CEC - parameters[1102];
                let CQQ = if 0.0f64 != 0.0 && (if CQP < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CQV = if CQQ != 0.0 {
                    let CQR = -4e0f64 / (CAD * CQP);
                    CQR
                } else {
                    let CQS = CQP - BVW;
                    let CQT = LP * ((CQP + BVW) + (((CQS * CQS) + 1e0f64).sqrt()));
                    CQT
                };
                let CQU = AAP * parameters[1103];
                let CQX = ((V * AY) * BCO) * (CQW * (B + (parameters[1101] * ((CQU * CQV) / (CQU + CQV)))));
                let CUI;
                let DBR;
                let EGC;
                if BXL != 0.0 {
                    let CQY = (BXO - BWW).abs();
                    let CRA = if CQZ == A { 1.0 } else { 0.0 };
                    let CRQ;
                    if CRA != 0.0 {
                        CRQ = B;
                    } else {
                        let CRB = CQY - parameters[1126];
                        let CRC = if 1.0f64 != 0.0 && (if CRB < -1.25e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CRF = if CRC != 0.0 {
                            let CRD = -2.5e-1f64 / (CAD * CRB);
                            CRD
                        } else {
                            let CRE = LP * (CRB + (((CRB * CRB) + 6.25e-2f64).sqrt()));
                            CRE
                        };
                        let CRG = B + (CRF * CQZ);
                        CRQ = CRG;
                    }
                    let CRI = if (if CRH != A { 1.0 } else { 0.0 }) != 0.0 && (if AXP != AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if CRI != 0.0 {
                    } else {
                    }
                    let CRK = if (if CRJ != A { 1.0 } else { 0.0 }) != 0.0 && (if CRH > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CSG;
                    let DBS;
                    if CRK != 0.0 {
                        let CRM = BWU - CRL;
                        let CRO = ((CRM * CRM) + (AAP.powf(((AT * (-3e0f64 - (CRH.ln()))) / CRN)))).sqrt();
                        let CRR = ((CQX * CRP) * CRQ) * (B + (CRH * (CRO.powf(CRN))));
                        CSG = CRR;
                        DBS = CRO;
                    } else {
                        let CRS = (CQX * CRP) * CRQ;
                        CSG = CRS;
                        DBS = A;
                    }
                    let CRU = B + (BWZ / CRT);
                    let CRV = if 1.0f64 != 0.0 && (if CRU < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CRZ = if CRV != 0.0 {
                        let CRW = -2.5000000000000005e-3f64 / (CAD * CRU);
                        CRW
                    } else {
                        let CRX = LP * (CRU + (((CRU * CRU) + 6.250000000000001e-4f64).sqrt()));
                        CRX
                    };
                    let CSB = (B - (CRY * ((CRZ.sqrt()) - B))) - (CSA * BWZ);
                    let CSC = if 1.0f64 != 0.0 && (if CSB < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CSF = if CSC != 0.0 {
                        let CSD = -2.5000000000000005e-3f64 / (CAD * CSB);
                        CSD
                    } else {
                        let CSE = LP * (CSB + (((CSB * CSB) + 6.250000000000001e-4f64).sqrt()));
                        CSE
                    };
                    let CSH = CSF * CSG;
                    let CSK = ((CSI * BXK) * CFF) * CSJ;
                    let CSL = CSH * CSK;
                    let CSN = SA - CSM;
                    let CSO = CQY.powf(CSN);
                    let CSQ = B / CSM;
                    let CSR = (((CSO / (CSO + (CSP * (CSL.powf(CSN))))).powf(CSQ)) * CQY) / CSL;
                    let CSS = if 1.0f64 != 0.0 && (if CSR < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CSV = if CSS != 0.0 {
                        let CST = -1e-6f64 / (CAD * CSR);
                        CST
                    } else {
                        let CSU = LP * (CSR + (((CSR * CSR) + 2.5e-7f64).sqrt()));
                        CSU
                    };
                    let CSW = CSK * ((B + (CSV.powf(CSM))).powf(CSQ));
                    CUI = CSH;
                    DBR = DBS;
                    EGC = CSW;
                } else {
                    CUI = A;
                    DBR = A;
                    EGC = A;
                }
                let CSY = if CSX != A { 1.0 } else { 0.0 };
                let CUJ;
                let DBP;
                let EGN;
                if CSY != 0.0 {
                    let CSZ = (BWY - COV).abs();
                    let CTB = if (if CTA != A { 1.0 } else { 0.0 }) != 0.0 && (if AXP != AT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if CTB != 0.0 {
                    } else {
                    }
                    let CTC = if (if CRJ != A { 1.0 } else { 0.0 }) != 0.0 && (if CTA > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTT;
                    let DBQ;
                    if CTC != 0.0 {
                        let CTD = BWU - CRL;
                        let CTF = ((CTD * CTD) + (AAP.powf(((AT * (-3e0f64 - (CTA.ln()))) / CTE)))).sqrt();
                        let CTH = (CQX * CTG) * (B + (CTA * (CTF.powf(CTE))));
                        CTT = CTH;
                        DBQ = CTF;
                    } else {
                        let CTI = CQX * CTG;
                        CTT = CTI;
                        DBQ = DBR;
                    }
                    let CTJ = B + (BWZ / CRT);
                    let CTK = if 1.0f64 != 0.0 && (if CTJ < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTN = if CTK != 0.0 {
                        let CTL = -2.5000000000000005e-3f64 / (CAD * CTJ);
                        CTL
                    } else {
                        let CTM = LP * (CTJ + (((CTJ * CTJ) + 6.250000000000001e-4f64).sqrt()));
                        CTM
                    };
                    let CTO = (B - (CRY * ((CTN.sqrt()) - B))) - (CSA * BWZ);
                    let CTP = if 1.0f64 != 0.0 && (if CTO < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTS = if CTP != 0.0 {
                        let CTQ = -2.5000000000000005e-3f64 / (CAD * CTO);
                        CTQ
                    } else {
                        let CTR = LP * (CTO + (((CTO * CTO) + 6.250000000000001e-4f64).sqrt()));
                        CTR
                    };
                    let CTU = CTS * CTT;
                    let CTV = ((CSI * CSX) * CFF) * CSJ;
                    let CTW = CTU * CTV;
                    let CTX = SA - CSM;
                    let CTY = CSZ.powf(CTX);
                    let CTZ = B / CSM;
                    let CUA = (((CTY / (CTY + (CSP * (CTW.powf(CTX))))).powf(CTZ)) * CSZ) / CTW;
                    let CUB = if 1.0f64 != 0.0 && (if CUA < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CUE = if CUB != 0.0 {
                        let CUC = -1e-6f64 / (CAD * CUA);
                        CUC
                    } else {
                        let CUD = LP * (CUA + (((CUA * CUA) + 2.5e-7f64).sqrt()));
                        CUD
                    };
                    let CUF = CTV * ((B + (CUE.powf(CSM))).powf(CTZ));
                    CUJ = CTU;
                    DBP = DBQ;
                    EGN = CUF;
                } else {
                    CUJ = CUK;
                    DBP = DBR;
                    EGN = A;
                }
                let CUG = if BXL != 0.0 && CSY != 0.0 { 1.0 } else { 0.0 };
                let DAF;
                if CUG != 0.0 {
                    let CUL = if CUI <= CUJ { CUI } else { CUJ };
                    let CUM = (CUH * CQG) / CUL;
                    let CUN = CUM - B;
                    let CUP = MG * CUO;
                    let CUQ = CUP * CUO;
                    let CUR = ((((LP * ((CUM + B) - (((CUN * CUN) + CUQ).sqrt()))) + CUP) + (LP * ((1e0f64 + CUQ).sqrt()))) - LP) - CUP;
                    let CUS = if 0.0f64 != 0.0 && (if CUR < (-2.5e3f64 * CUO) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CUV = if CUS != 0.0 {
                        let CUT = ((-CUO) * CUO) / (CAD * CUR);
                        CUT
                    } else {
                        let CUU = LP * ((CUR + -1e0f64) + ((((CUR - -1e0f64) * (CUR - -1e0f64)) + CUQ).sqrt()));
                        CUU
                    };
                    let CUW = (CUH * CUL) * ((CUV - (LP * ((1e0f64 + CUQ).sqrt()))) + LP);
                    DAF = CUW;
                } else {
                    let CVH;
                    if BXL != 0.0 {
                        let CUX = (CUH * CQG) / CUI;
                        let CUY = CUX - B;
                        let CUZ = MG * CUO;
                        let CVA = CUZ * CUO;
                        let CVB = ((((LP * ((CUX + B) - (((CUY * CUY) + CVA).sqrt()))) + CUZ) + (LP * ((1e0f64 + CVA).sqrt()))) - LP) - CUZ;
                        let CVC = if 0.0f64 != 0.0 && (if CVB < (-2.5e3f64 * CUO) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CVF = if CVC != 0.0 {
                            let CVD = ((-CUO) * CUO) / (CAD * CVB);
                            CVD
                        } else {
                            let CVE = LP * ((CVB + -1e0f64) + ((((CVB - -1e0f64) * (CVB - -1e0f64)) + CVA).sqrt()));
                            CVE
                        };
                        let CVG = (CUH * CUI) * ((CVF - (LP * ((1e0f64 + CVA).sqrt()))) + LP);
                        CVH = CVG;
                    } else {
                        CVH = CQG;
                    }
                    let DAG;
                    if CSY != 0.0 {
                        let CVI = (CUH * CVH) / CUJ;
                        let CVJ = CVI - B;
                        let CVK = MG * CUO;
                        let CVL = CVK * CUO;
                        let CVM = ((((LP * ((CVI + B) - (((CVJ * CVJ) + CVL).sqrt()))) + CVK) + (LP * ((1e0f64 + CVL).sqrt()))) - LP) - CVK;
                        let CVN = if 0.0f64 != 0.0 && (if CVM < (-2.5e3f64 * CUO) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let CVQ = if CVN != 0.0 {
                            let CVO = ((-CUO) * CUO) / (CAD * CVM);
                            CVO
                        } else {
                            let CVP = LP * ((CVM + -1e0f64) + ((((CVM - -1e0f64) * (CVM - -1e0f64)) + CVL).sqrt()));
                            CVP
                        };
                        let CVR = (CUH * CUJ) * ((CVQ - (LP * ((1e0f64 + CVL).sqrt()))) + LP);
                        DAG = CVR;
                    } else {
                        DAG = CVH;
                    }
                    DAF = DAG;
                }
                DAE = DAF;
                DBO = DBP;
                EGB = EGC;
                EGM = EGN;
            } else {
                DAE = CQG;
                DBO = A;
                EGB = A;
                EGM = A;
            }
            let CVT = if (if OO != 0.0 && BXM != 0.0 { 1.0 } else { 0.0 }) != 0.0 && BFP != 0.0 { 1.0 } else { 0.0 };
            if CVT != 0.0 {
                let CVW = ((-CVU) - CVV) / BBO;
                let CVX = ((((3.20438e-19f64 * H) * CQI) * BBP).sqrt()) / L;
                let CVY = (if (CQI / BBZ) >= AXS { (CQI / BBZ) } else { AXS }).ln();
                let CVZ = RZ * (B + (CVX / CBZ));
                let CWA = (LP * CVW) - CVZ;
                let CWB = CWA + (((CWA * CWA) + (SB * CVW)).sqrt());
                let CWC = if CVW < A { 1.0 } else { 0.0 };
                let CWJ = if CWC != 0.0 {
                    let CWD = (CVW - CWB) / CVX;
                    let CWE = -((if ((B - CWB) + (CWD * CWD)) >= AXS { ((B - CWB) + (CWD * CWD)) } else { AXS }).ln());
                    CWE
                } else {
                    let CWF = rspice_limited_exp((-CWB));
                    let CWG = LP * CVX;
                    let CWH = ((((CVW - B) + CWF) + (CWG * CWG)).sqrt()) - CWG;
                    let CWI = ((CWH * CWH) + B) - CWF;
                    CWI
                };
                let CWK = CWJ - B;
                let CWL = (LP * ((CWJ + B) + (((CWK * CWK) + 1e0f64).sqrt()))).sqrt();
                let CWM = AT * CWL;
                let CWN = (B + (CVX / CWM)) / CVX;
                let CWO = AT * CVY;
                let CWP = (CWJ - CWO) - (BXS / BBO);
                let CWQ = CWP - ((if ((SA * CWN) * CWL) >= AXS { ((SA * CWN) * CWL) } else { AXS }).ln());
                let CWR = LP * ((CWQ - CCT) - (((CWQ * (CWQ + CCU)) + CCV).sqrt()));
                let CWS = if CWR <= -6.8e1f64 { 1.0 } else { 0.0 };
                let CXS;
                if CWS != 0.0 {
                    let CWU = if CWR < -1.1e2f64 { 1.0 } else { 0.0 };
                    let CXB;
                    if CWU != 0.0 {
                        CXB = CWV;
                    } else {
                        let CWW = if CWR > -9e1f64 { 1.0 } else { 0.0 };
                        let CXC = if CWW != 0.0 {
                            let CWX = rspice_limited_exp(CWR);
                            CWX
                        } else {
                            let CWY = (CWR - CWT) / BWB;
                            let CWZ = CWY * CWY;
                            let CXA = rspice_limited_exp((CWT + (BWB * ((7.8125e-2f64 + (LP * CWY)) + (CWZ * (9.375e-1f64 - (CWZ * (CDF - CWZ))))))));
                            CXA
                        };
                        CXB = CXC;
                    }
                    let CXD = CXB * (((B + CWP) - CWR) - ((if ((AT * CWN) * (((CXB * AT) * CWN) + CWM)) >= AXS { ((AT * CWN) * (((CXB * AT) * CWN) + CWM)) } else { AXS }).ln()));
                    CXS = CXD;
                } else {
                    let CXE = rspice_limited_exp(CWR);
                    let CXF = AT * CXE;
                    let CXG = CXF * CWN;
                    let CXH = CWN + (B / CWL);
                    let CXI = CXE - (((CXF + ((if (CXG * (CXG + CWM)) >= AXS { (CXG * (CXG + CWM)) } else { AXS }).ln())) - CWP) / ((AT + (1e0f64 / CXE)) + (CXH / ((CWN * CXE) + CWL))));
                    let CXJ = AT * CXI;
                    let CXK = CXJ * CWN;
                    let CXL = (CXJ + ((if (CXK * (CXK + CWM)) >= AXS { (CXK * (CXK + CWM)) } else { AXS }).ln())) - CWP;
                    let CXM = (CWN * CXI) + CWL;
                    let CXN = CXH / CXM;
                    let CXO = (AT + (1e0f64 / CXI)) + CXN;
                    let CXP = B / CXI;
                    let CXQ = CXI - ((CXL / CXO) * (B + ((CXL * (((-1e0f64 * (CXP * CXP)) - (1e0f64 / (((CWL * CWL) * CWL) * CXM))) - (CXN * CXN))) / ((AT * CXO) * CXO))));
                    CXS = CXQ;
                }
                let CXR = if 0.0f64 != 0.0 && (if CWJ < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if CXR != 0.0 {
                } else {
                }
                let CXT = if 0.0f64 != 0.0 && (if (CWJ - (AT * CXS)) < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if CXT != 0.0 {
                } else {
                }
                let CXU = if parameters[1118] > A { 1.0 } else { 0.0 };
                if CXU != 0.0 {
                } else {
                }
                let CXW = if CXV == B { 1.0 } else { 0.0 };
                if CXW != 0.0 {
                    let CXX = (((-BCH) * BXJ) - CVV) / BBO;
                    let CXY = (LP * CXX) - CVZ;
                    let CXZ = CXY + (((CXY * CXY) + (SB * CXX)).sqrt());
                    let CYA = if CXX < A { 1.0 } else { 0.0 };
                    let CYH = if CYA != 0.0 {
                        let CYB = (CXX - CXZ) / CVX;
                        let CYC = -((if ((B - CXZ) + (CYB * CYB)) >= AXS { ((B - CXZ) + (CYB * CYB)) } else { AXS }).ln());
                        CYC
                    } else {
                        let CYD = rspice_limited_exp((-CXZ));
                        let CYE = LP * CVX;
                        let CYF = ((((CXX - B) + CYD) + (CYE * CYE)).sqrt()) - CYE;
                        let CYG = ((CYF * CYF) + B) - CYD;
                        CYG
                    };
                    let CYI = CYH - B;
                    let CYJ = (LP * ((CYH + B) + (((CYI * CYI) + 1e0f64).sqrt()))).sqrt();
                    let CYK = AT * CYJ;
                    let CYL = (B + (CVX / CYK)) / CVX;
                    let CYM = (CYH - CWO) - (BWZ / BBO);
                    let CYN = CYM - ((if ((SA * CYL) * CYJ) >= AXS { ((SA * CYL) * CYJ) } else { AXS }).ln());
                    let CYO = LP * ((CYN - CCT) - (((CYN * (CYN + CCU)) + CCV).sqrt()));
                    let CYP = if CYO <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let CZP;
                    if CYP != 0.0 {
                        let CYR = if CYO < -1.1e2f64 { 1.0 } else { 0.0 };
                        let CYY;
                        if CYR != 0.0 {
                            CYY = CYS;
                        } else {
                            let CYT = if CYO > -9e1f64 { 1.0 } else { 0.0 };
                            let CYZ = if CYT != 0.0 {
                                let CYU = rspice_limited_exp(CYO);
                                CYU
                            } else {
                                let CYV = (CYO - CYQ) / BWB;
                                let CYW = CYV * CYV;
                                let CYX = rspice_limited_exp((CYQ + (BWB * ((7.8125e-2f64 + (LP * CYV)) + (CYW * (9.375e-1f64 - (CYW * (CDF - CYW))))))));
                                CYX
                            };
                            CYY = CYZ;
                        }
                        let CZA = CYY * (((B + CYM) - CYO) - ((if ((AT * CYL) * (((CYY * AT) * CYL) + CYK)) >= AXS { ((AT * CYL) * (((CYY * AT) * CYL) + CYK)) } else { AXS }).ln()));
                        CZP = CZA;
                    } else {
                        let CZB = rspice_limited_exp(CYO);
                        let CZC = AT * CZB;
                        let CZD = CZC * CYL;
                        let CZE = CYL + (B / CYJ);
                        let CZF = CZB - (((CZC + ((if (CZD * (CZD + CYK)) >= AXS { (CZD * (CZD + CYK)) } else { AXS }).ln())) - CYM) / ((AT + (1e0f64 / CZB)) + (CZE / ((CYL * CZB) + CYJ))));
                        let CZG = AT * CZF;
                        let CZH = CZG * CYL;
                        let CZI = (CZG + ((if (CZH * (CZH + CYK)) >= AXS { (CZH * (CZH + CYK)) } else { AXS }).ln())) - CYM;
                        let CZJ = (CYL * CZF) + CYJ;
                        let CZK = CZE / CZJ;
                        let CZL = (AT + (1e0f64 / CZF)) + CZK;
                        let CZM = B / CZF;
                        let CZN = CZF - ((CZI / CZL) * (B + ((CZI * (((-1e0f64 * (CZM * CZM)) - (1e0f64 / (((CYJ * CYJ) * CYJ) * CZJ))) - (CZK * CZK))) / ((AT * CZL) * CZL))));
                        CZP = CZN;
                    }
                    let CZO = if 0.0f64 != 0.0 && (if CYH < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if CZO != 0.0 {
                    } else {
                    }
                    let CZQ = if 0.0f64 != 0.0 && (if (CYH - (AT * CZP)) < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if CZQ != 0.0 {
                    } else {
                    }
                    if CXU != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let CZR = if BAN > B { 1.0 } else { 0.0 };
            let EGT;
            let EGV;
            if CZR != 0.0 {
                let CZS = (parameters[754] * V) * ((((((parameters[755] * BBO) * CQE) * AY) / AU) * L) + ((((CQE * AY) / AU) * L) * CMD));
                let CZT = if BAN == AT { 1.0 } else { 0.0 };
                let EGU;
                let EGW;
                if CZT != 0.0 {
                    let CZV = if (B / CZU) < AWZ { 1.0 } else { 0.0 };
                    let CZX = if CZV != 0.0 {
                        let CZW = B / AWZ;
                        CZW
                    } else {
                        CZU
                    };
                    let CZY = (CZX * CZS) / (CZX + CZS);
                    EGU = CZY;
                    EGW = CZX;
                } else {
                    EGU = CZS;
                    EGW = CZU;
                }
                EGT = EGU;
                EGV = EGW;
            } else {
                EGT = A;
                EGV = CZU;
            }
            let CZZ = if BFO == A { 1.0 } else { 0.0 };
            let DBL;
            if CZZ != 0.0 {
                let DAC = if (if DAA <= A { 1.0 } else { 0.0 }) != 0.0 || (if DAB <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if DAC != 0.0 {
                } else {
                    let DAD = if CMR > (DAB / CBA) { 1.0 } else { 0.0 };
                    if DAD != 0.0 {
                    } else {
                    }
                }
                DBL = A;
            } else {
                let DBM;
                if BFP != 0.0 {
                    let DAI = BXX * ((B + (((BXX / ((B + (DAH * BXX)) * CJW)) + BP).powf(CJX))).powf(CJY));
                    let DAJ = BXX - DAI;
                    let DAK = if 1.0f64 != 0.0 && (if DAJ < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DAQ = if DAK != 0.0 {
                        let DAL = -1e-6f64 / (CAD * DAJ);
                        DAL
                    } else {
                        let DAM = LP * (DAJ + (((DAJ * DAJ) + 2.5e-7f64).sqrt()));
                        DAM
                    };
                    let DAN = (LP * DAB) * (B + (DAI.powf(NA)));
                    let DAO = if 1.0f64 != 0.0 && (if ((DAA / (B + (parameters[493] * (rspice_limited_exp((parameters[492] * BYT)))))) * ((B + (parameters[505] * BYU)) + ((parameters[506] * BYU) * BYU))) < -2.5e-9f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if DAO != 0.0 {
                    } else {
                    }
                    let DAP = if (if DAA <= A { 1.0 } else { 0.0 }) != 0.0 || (if DAB <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if DAP != 0.0 {
                    } else {
                        let DAR = if DAQ > (DAN / CBA) { 1.0 } else { 0.0 };
                        if DAR != 0.0 {
                        } else {
                        }
                    }
                    DBM = DAI;
                } else {
                    DBM = A;
                }
                DBL = DBM;
            }
            let DAS = if BFP != 0.0 && (if CRJ == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if DAS != 0.0 {
                let DAT = CEC - parameters[1105];
                let DAU = if 0.0f64 != 0.0 && (if DAT < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DAZ = if DAU != 0.0 {
                    let DAV = -4e0f64 / (CAD * DAT);
                    DAV
                } else {
                    let DAW = DAT - BVW;
                    let DAX = LP * ((DAT + BVW) + (((DAW * DAW) + 1e0f64).sqrt()));
                    DAX
                };
                let DAY = AAP * parameters[1106];
                let DBA = (((parameters[502] * DAE) / (((V * AY) * BCO) * (CQW * (B + (parameters[1104] * ((DAY * DAZ) / (DAY + DAZ))))))) / CRP) - B;
                let DBC = if 1.0f64 != 0.0 && (if DBA < (-2.5e3f64 * DBB) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DBF = if DBC != 0.0 {
                    let DBD = ((-DBB) * DBB) / (CAD * DBA);
                    DBD
                } else {
                    let DBE = LP * (DBA + (((DBA * DBA) + ((MG * DBB) * DBB)).sqrt()));
                    DBE
                };
                let DBG = CRP * DBF;
                let DBH = if CRH > A { 1.0 } else { 0.0 };
                let DCB;
                if DBH != 0.0 {
                    let DBT = (((BCH * (DBI - DBJ)) - (DBK * DBL)) - DBN) - (CRH * (DBO.powf(parameters[513])));
                    let DBU = if 1.0f64 != 0.0 && (if DBT < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DCC = if DBU != 0.0 {
                        let DBV = -2.5000000000000005e-3f64 / (CAD * DBT);
                        DBV
                    } else {
                        let DBW = LP * (DBT + (((DBT * DBT) + 6.250000000000001e-4f64).sqrt()));
                        DBW
                    };
                    DCB = DCC;
                } else {
                    let DBX = ((BCH * (DBI - DBJ)) - (DBK * DBL)) - DBN;
                    let DBY = if 1.0f64 != 0.0 && (if DBX < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DCD = if DBY != 0.0 {
                        let DBZ = -2.5000000000000005e-3f64 / (CAD * DBX);
                        DBZ
                    } else {
                        let DCA = LP * (DBX + (((DBX * DBX) + 6.250000000000001e-4f64).sqrt()));
                        DCA
                    };
                    DCB = DCD;
                }
                let DCE = (((3.20438e-19f64 / H) * DBG) * DCB).sqrt();
                let DCF = if 1.0f64 != 0.0 && (if (parameters[500] * ((B + ((parameters[507] * BYU) + ((parameters[508] * BYU) * BYU))) + ((parameters[509] * DCB) + (parameters[510] * (DCB.powf(parameters[511])))))) < -2.5e-9f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if DCF != 0.0 {
                } else {
                }
                let DCG = if DCE > (parameters[501] / CBA) { 1.0 } else { 0.0 };
                if DCG != 0.0 {
                } else {
                }
            } else {
            }
            let DCH = if PO != 0.0 || PL != 0.0 { 1.0 } else { 0.0 };
            let DET;
            let DEV;
            let DEX;
            let DEZ;
            let DFC;
            if DCH != 0.0 {
                let DCI = CAS * ((CEN + CEC) + CLF);
                let DCK = ((DCI * DCI) + DCJ).sqrt();
                let DCL = LP * ((-DCI) + DCK);
                let DCM = LP * (DCI + DCK);
                let DEY;
                if PL != 0.0 {
                    let DCN = -((DCI / GN) / BBO);
                    let DCO = if DCN > BYD { 1.0 } else { 0.0 };
                    let DCS;
                    if DCO != 0.0 {
                        DCS = DCN;
                    } else {
                        let DCP = if DCN < -3.7e1f64 { 1.0 } else { 0.0 };
                        let DCT = if DCP != 0.0 {
                            let DCQ = DCN.exp();
                            DCQ
                        } else {
                            let DCR = (B + (DCN.exp())).ln();
                            DCR
                        };
                        DCS = DCT;
                    }
                    let DCU = (V * AY) * AU;
                    let DCV = (((((DCU * BAT) * BAR) * BWV) * ((GN * BBO) * DCS)) * (rspice_limited_exp((((-7.45669e11f64 * K) * (GK - (GL * DCL))) * (B + (GM * DCL)))))) * BHR;
                    let DCW = ((DCI - GI) / GJ) / BBO;
                    let DCX = if DCW > BYD { 1.0 } else { 0.0 };
                    let DDB;
                    if DCX != 0.0 {
                        DDB = DCW;
                    } else {
                        let DCY = if DCW < -3.7e1f64 { 1.0 } else { 0.0 };
                        let DDC = if DCY != 0.0 {
                            let DCZ = DCW.exp();
                            DCZ
                        } else {
                            let DDA = (B + (DCW.exp())).ln();
                            DDA
                        };
                        DDB = DDC;
                    }
                    let DDD = DCV + ((((((DCU * 3.75956e-7f64) * BAR) * BWV) * ((GJ * BBO) * DDB)) * (rspice_limited_exp((((-9.82222e11f64 * K) * (GF - (GG * DCM))) * (B + (GH * DCM)))))) * BHR);
                    DEY = DDD;
                } else {
                    DEY = A;
                }
                let DEU;
                let DEW;
                let DFA;
                let DFD;
                if PO != 0.0 {
                    let DDE = (((V * BBC) * (((CLM * CAS) * CLS) * (rspice_limited_exp(((BBA * (ND - (GP * DCM))) * (B + (GQ * DCM))))))) * ((BWV + (LP * BYT)) - (LP * (BXW + BXV)))) * BHR;
                    let DDF = NG * ((((CJZ * CJZ) + BKF).sqrt()) - BVW);
                    let DDG = rspice_limited_exp((-DDF));
                    let DDH = ((DDF + DDG) - B) + DCJ;
                    let DDI = (B - ((DDF + B) * DDG)) + DCJ;
                    let DDJ = (DDF * DDF) + 2e-4f64;
                    let DDK = if CUH > A { 1.0 } else { 0.0 };
                    let DFB;
                    let DFE;
                    if DDK != 0.0 {
                        let DDL = (DDE * DDI) / DDJ;
                        let DDM = (DDE * DDH) / DDJ;
                        DFB = DDM;
                        DFE = DDL;
                    } else {
                        let DDN = (DDE * DDI) / DDJ;
                        let DDO = (DDE * DDH) / DDJ;
                        DFB = DDN;
                        DFE = DDO;
                    }
                    let DDP = BXG - COX;
                    let DDQ = ((DDP * DDP) + DCJ).sqrt();
                    let DDR = if parameters[1041] == B { 1.0 } else { 0.0 };
                    let DDZ;
                    let DEB;
                    if DDR != 0.0 {
                        let DDS = NE - (GS * DDQ);
                        let DDT = if DDS < -1e-2f64 { 1.0 } else { 0.0 };
                        let DDW = if DDT != 0.0 {
                            let DDU = -1e-12f64 / DDS;
                            DDU
                        } else {
                            let DDV = LP * (DDS + (((DDS * DDS) + 4e-12f64).sqrt()));
                            DDV
                        };
                        let DDX = if GT < BKF { 1.0 } else { 0.0 };
                        let DEA = if DDX != 0.0 {
                            BKF
                        } else {
                            GT
                        };
                        DDZ = DEA;
                        DEB = DDW;
                    } else {
                        let DDY = NE - (GS * DDQ);
                        DDZ = GT;
                        DEB = DDY;
                    }
                    let DEC = (BHR * V) * BAZ;
                    let DEE = (((DEC * DED) * BXG) * DDQ) * (rspice_limited_exp(((BBB * DEB) * (B + (DDZ * DDQ)))));
                    let DEF = BXF - COX;
                    let DEG = ((DEF * DEF) + DCJ).sqrt();
                    let DEO;
                    let DEQ;
                    if DDR != 0.0 {
                        let DEH = NF - (GV * DEG);
                        let DEI = if DEH < -1e-2f64 { 1.0 } else { 0.0 };
                        let DEL = if DEI != 0.0 {
                            let DEJ = -1e-12f64 / DEH;
                            DEJ
                        } else {
                            let DEK = LP * (DEH + (((DEH * DEH) + 4e-12f64).sqrt()));
                            DEK
                        };
                        let DEM = if GW < BKF { 1.0 } else { 0.0 };
                        let DEP = if DEM != 0.0 {
                            BKF
                        } else {
                            GW
                        };
                        DEO = DEP;
                        DEQ = DEL;
                    } else {
                        let DEN = NF - (GV * DEG);
                        DEO = GW;
                        DEQ = DEN;
                    }
                    let DES = (((DEC * DER) * BXF) * DEG) * (rspice_limited_exp(((BBB * DEQ) * (B + (DEO * DEG)))));
                    DEU = DEE;
                    DEW = DES;
                    DFA = DFB;
                    DFD = DFE;
                } else {
                    DEU = A;
                    DEW = A;
                    DFA = A;
                    DFD = A;
                }
                DET = DEU;
                DEV = DEW;
                DEX = DEY;
                DEZ = DFA;
                DFC = DFD;
            } else {
                DET = A;
                DEV = A;
                DEX = A;
                DEZ = A;
                DFC = A;
            }
            let DFF = if parameters[45] != A { 1.0 } else { 0.0 };
            if DFF != 0.0 {
                let DFG = if (if (if NB <= A { 1.0 } else { 0.0 }) != 0.0 || (if BHK <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || PA != 0.0 { 1.0 } else { 0.0 };
                if DFG != 0.0 {
                } else {
                    let DFH = if ((((-BXF) - FO) + COX) / CEL) < -1e2f64 { 1.0 } else { 0.0 };
                    if DFH != 0.0 {
                    } else {
                    }
                    let DFI = if FN != A { 1.0 } else { 0.0 };
                    if DFI != 0.0 {
                        let DFJ = (BWX * BWX) * BWX;
                        let DFK = if (DFJ / ((FN + (DFJ.abs())) + DCJ)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if DFK != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
                let DFL = if (if (if NC <= A { 1.0 } else { 0.0 }) != 0.0 || (if BHP <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || PB != 0.0 { 1.0 } else { 0.0 };
                if DFL != 0.0 {
                } else {
                    let DFM = if ((((-BXG) - FS) + COX) / CEL) < -1e2f64 { 1.0 } else { 0.0 };
                    if DFM != 0.0 {
                    } else {
                    }
                    let DFN = if FR != A { 1.0 } else { 0.0 };
                    if DFN != 0.0 {
                        let DFO = (BWZ * BWZ) * BWZ;
                        let DFP = if (DFO / ((FR + (DFO.abs())) + DCJ)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if DFP != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
            } else {
            }
            if BTH != 0.0 {
            } else {
            }
            let DFQ = if BMB > A { 1.0 } else { 0.0 };
            if DFQ != 0.0 {
                let DFS = if (DFR - BXB) < (DFR * AYY) { 1.0 } else { 0.0 };
                if DFS != 0.0 {
                } else {
                }
            } else {
            }
            let DFT = if BMC > A { 1.0 } else { 0.0 };
            if DFT != 0.0 {
                let DFV = if (DFU - BXB) < (DFU * AYY) { 1.0 } else { 0.0 };
                if DFV != 0.0 {
                } else {
                }
            } else {
            }
            let DFW = if BME > A { 1.0 } else { 0.0 };
            if DFW != 0.0 {
                let DFY = if (DFX - BXB) < (DFX * AYY) { 1.0 } else { 0.0 };
                if DFY != 0.0 {
                } else {
                }
            } else {
            }
            if BTO != 0.0 {
                let DGA = if DFZ > A { 1.0 } else { 0.0 };
                if DGA != 0.0 {
                } else {
                }
                let DGB = if (if BAH > A { 1.0 } else { 0.0 }) != 0.0 && BAD != 0.0 { 1.0 } else { 0.0 };
                if DGB != 0.0 {
                } else {
                }
            } else {
            }
            let DGC = if BMF > A { 1.0 } else { 0.0 };
            if DGC != 0.0 {
                let DGE = if (DGD - BXD) < (DGD * AYY) { 1.0 } else { 0.0 };
                if DGE != 0.0 {
                } else {
                }
            } else {
            }
            let DGF = if BMG > A { 1.0 } else { 0.0 };
            if DGF != 0.0 {
                let DGG = if (if BAH > A { 1.0 } else { 0.0 }) != 0.0 && BAD != 0.0 { 1.0 } else { 0.0 };
                if DGG != 0.0 {
                    let DGH = if BTK > BTF { 1.0 } else { 0.0 };
                    if DGH != 0.0 {
                    } else {
                    }
                } else {
                }
                let DGJ = if (DGI - BXD) < (DGI * AYY) { 1.0 } else { 0.0 };
                if DGJ != 0.0 {
                } else {
                }
            } else {
            }
            let DGK = if BMH > A { 1.0 } else { 0.0 };
            if DGK != 0.0 {
                let DGM = if (DGL - BXD) < (DGL * AYY) { 1.0 } else { 0.0 };
                if DGM != 0.0 {
                } else {
                }
            } else {
            }
            let DGN = if BAH > A { 1.0 } else { 0.0 };
            if DGN != 0.0 {
                if DGC != 0.0 {
                    let DGO = if (DGD - BXE) < (DGD * AYY) { 1.0 } else { 0.0 };
                    if DGO != 0.0 {
                    } else {
                    }
                } else {
                }
                if DGF != 0.0 {
                    let DGP = if BTK > BTF { 1.0 } else { 0.0 };
                    if DGP != 0.0 {
                    } else {
                    }
                    let DGQ = if (DGI - BXE) < (DGI * AYY) { 1.0 } else { 0.0 };
                    if DGQ != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DGR = BJC * BTB;
            let DGS = BJN * BTC;
            let DGT = (BJY * BN) * V;
            let DGV = if DGU == B { 1.0 } else { 0.0 };
            if DGV != 0.0 {
            } else {
            }
            let DGX = if DGW == B { 1.0 } else { 0.0 };
            if DGX != 0.0 {
            } else {
            }
            let DGZ = if DGY == B { 1.0 } else { 0.0 };
            if DGZ != 0.0 {
            } else {
            }
            let DHA = if DGR > A { 1.0 } else { 0.0 };
            if DHA != 0.0 {
                let DHC = if (BXB / BKL) < DHB { 1.0 } else { 0.0 };
                if DHC != 0.0 {
                    let DHD = if DGU != B { 1.0 } else { 0.0 };
                    if DHD != 0.0 {
                        let DHE = if DGU == LP { 1.0 } else { 0.0 };
                        if DHE != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DHF = if DGS > A { 1.0 } else { 0.0 };
            if DHF != 0.0 {
                let DHG = if (BXB / BKY) < DHB { 1.0 } else { 0.0 };
                if DHG != 0.0 {
                    let DHH = if DGW != B { 1.0 } else { 0.0 };
                    if DHH != 0.0 {
                        let DHI = if DGW == LP { 1.0 } else { 0.0 };
                        if DHI != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DHJ = if DGT > A { 1.0 } else { 0.0 };
            if DHJ != 0.0 {
                let DHK = if (BXB / BLL) < DHB { 1.0 } else { 0.0 };
                if DHK != 0.0 {
                    let DHL = if DGY != B { 1.0 } else { 0.0 };
                    if DHL != 0.0 {
                        let DHM = if DGY == LP { 1.0 } else { 0.0 };
                        if DHM != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DHN = (DFZ * BJH) * BTJ;
            let DHO = if BTK > BTF { 1.0 } else { 0.0 };
            let DIF;
            if DHO != 0.0 {
                let DHP = if DGN != 0.0 && BAD != 0.0 { 1.0 } else { 0.0 };
                let DIG = if DHP != 0.0 {
                    let DHQ = (DFZ * BJS) * (BTK - BTF);
                    DHQ
                } else {
                    let DHR = (DFZ * BJS) * BTK;
                    DHR
                };
                DIF = DIG;
            } else {
                let DHS = (DFZ * BJS) * BTK;
                DIF = DHS;
            }
            let DHT = (BKD * BN) * V;
            let DHV = if DHU == B { 1.0 } else { 0.0 };
            if DHV != 0.0 {
            } else {
            }
            let DHX = if DHW == B { 1.0 } else { 0.0 };
            if DHX != 0.0 {
            } else {
            }
            let DHZ = if DHY == B { 1.0 } else { 0.0 };
            if DHZ != 0.0 {
            } else {
            }
            let DIA = if DHN > A { 1.0 } else { 0.0 };
            if DIA != 0.0 {
                let DIC = if (DIB / BKR) < DHB { 1.0 } else { 0.0 };
                if DIC != 0.0 {
                    let DID = if DHU != B { 1.0 } else { 0.0 };
                    if DID != 0.0 {
                        let DIE = if DHU == LP { 1.0 } else { 0.0 };
                        if DIE != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DIH = if DIF > A { 1.0 } else { 0.0 };
            if DIH != 0.0 {
                let DII = if (DIB / BLE) < DHB { 1.0 } else { 0.0 };
                if DII != 0.0 {
                    let DIJ = if DHW != B { 1.0 } else { 0.0 };
                    if DIJ != 0.0 {
                        let DIK = if DHW == LP { 1.0 } else { 0.0 };
                        if DIK != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DIL = if DHT > A { 1.0 } else { 0.0 };
            if DIL != 0.0 {
                let DIM = if (DIB / BLR) < DHB { 1.0 } else { 0.0 };
                if DIM != 0.0 {
                    let DIN = if DHY != B { 1.0 } else { 0.0 };
                    if DIN != 0.0 {
                        let DIO = if DHY == LP { 1.0 } else { 0.0 };
                        if DIO != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DIP = if DGN != 0.0 && BAD != 0.0 { 1.0 } else { 0.0 };
            if DIP != 0.0 {
                let DIQ = (BAH * BJH) * BTJ;
                let DIX = if DHO != 0.0 {
                    let DIR = BJS * ((BAH * (BTK - BTF)) + BTF);
                    DIR
                } else {
                    let DIS = (BAH * BJS) * BTK;
                    DIS
                };
                let DIT = if DIQ > A { 1.0 } else { 0.0 };
                if DIT != 0.0 {
                    let DIU = if (BXE / BKR) < DHB { 1.0 } else { 0.0 };
                    if DIU != 0.0 {
                        let DIV = if DHU != B { 1.0 } else { 0.0 };
                        if DIV != 0.0 {
                            let DIW = if DHU == LP { 1.0 } else { 0.0 };
                            if DIW != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                }
                let DIY = if DIX > A { 1.0 } else { 0.0 };
                if DIY != 0.0 {
                    let DIZ = if (BXE / BLE) < DHB { 1.0 } else { 0.0 };
                    if DIZ != 0.0 {
                        let DJA = if DHW != B { 1.0 } else { 0.0 };
                        if DJA != 0.0 {
                            let DJB = if DHW == LP { 1.0 } else { 0.0 };
                            if DJB != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let DJC = if parameters[38] != A { 1.0 } else { 0.0 };
            if DJC != 0.0 {
            } else {
            }
            let DJD = (SA * BBO) * BCO;
            let DJE = CMK / CQE;
            let DJG = if DJF <= A { 1.0 } else { 0.0 };
            let DNA;
            if DJG != 0.0 {
                DNA = A;
            } else {
                let DJH = BCQ * ((if (((CMR / BCQ) + DJF) / DJE) >= AXS { (((CMR / BCQ) + DJF) / DJE) } else { AXS }).ln());
                let DJI = if DJH < A { 1.0 } else { 0.0 };
                let DNB = if DJI != 0.0 {
                    A
                } else {
                    DJH
                };
                DNA = DNB;
            }
            let DJJ = BBO / BCO;
            let DJK = DJJ * ((L + CAK) + DK);
            let DJL = AT * CLM;
            let DJM = (DJL * L) * BBO;
            let DJN = (((DJM * CLF) * CQB) * CQA) / BCO;
            let DJO = ((4.112842231783458e-57f64 * BBO) * (DAE.abs())) * CQE;
            let DJP = BCO * BBO;
            let DJQ = (DJP * DAE) * DAE;
            let DJU = (DJR + (DJS * DJN)) + ((DJT * DJN) * DJN);
            let DJV = DJN + DJK;
            let DJW = DJV * DJV;
            let DJX = (DJR * BCO) * BBO;
            let EFC;
            let EIR;
            let EIS;
            let EIT;
            let EIU;
            let EIV;
            let EIW;
            if QH != 0.0 {
                let DJY = (BWV - CBI) / BBO;
                let DKA = ((((3.20438e-19f64 * H) * DJZ) / BBO).sqrt()) / L;
                let DKB = (DJZ / BBZ).ln();
                let DKC = (LP * DJY) - (RZ * (B + (DKA / CBZ)));
                let DKD = DKC + (((DKC * DKC) + (SB * DJY)).sqrt());
                let DKE = if DJY < A { 1.0 } else { 0.0 };
                let DKL = if DKE != 0.0 {
                    let DKF = (DJY - DKD) / DKA;
                    let DKG = -((if ((B - DKD) + (DKF * DKF)) >= AXS { ((B - DKD) + (DKF * DKF)) } else { AXS }).ln());
                    DKG
                } else {
                    let DKH = rspice_limited_exp((-DKD));
                    let DKI = LP * DKA;
                    let DKJ = ((((DJY - B) + DKH) + (DKI * DKI)).sqrt()) - DKI;
                    let DKK = ((DKJ * DKJ) + B) - DKH;
                    DKK
                };
                let DKM = DKL + B;
                let DKN = DKL - B;
                let DKO = DKN * DKN;
                let DKP = (LP * (DKM + ((DKO + 1e0f64).sqrt()))).sqrt();
                let DKQ = AT * DKP;
                let DKR = (B + (DKA / DKQ)) / DKA;
                let DKS = (DKL - (AT * DKB)) - CBJ;
                let DKT = DKS - ((if ((SA * DKR) * DKP) >= AXS { ((SA * DKR) * DKP) } else { AXS }).ln());
                let DKU = LP * ((DKT - CCT) - (((DKT * (DKT + CCU)) + CCV).sqrt()));
                let DKV = if DKU <= -6.8e1f64 { 1.0 } else { 0.0 };
                let DMG;
                if DKV != 0.0 {
                    let DKX = if DKU < -1.1e2f64 { 1.0 } else { 0.0 };
                    let DLE;
                    if DKX != 0.0 {
                        DLE = DKY;
                    } else {
                        let DKZ = if DKU > -9e1f64 { 1.0 } else { 0.0 };
                        let DLF = if DKZ != 0.0 {
                            let DLA = rspice_limited_exp(DKU);
                            DLA
                        } else {
                            let DLB = (DKU - DKW) / BWB;
                            let DLC = DLB * DLB;
                            let DLD = rspice_limited_exp((DKW + (BWB * ((7.8125e-2f64 + (LP * DLB)) + (DLC * (9.375e-1f64 - (DLC * (CDF - DLC))))))));
                            DLD
                        };
                        DLE = DLF;
                    }
                    let DLG = DLE * (((B + DKS) - DKU) - ((if ((AT * DKR) * (((DLE * AT) * DKR) + DKQ)) >= AXS { ((AT * DKR) * (((DLE * AT) * DKR) + DKQ)) } else { AXS }).ln()));
                    DMG = DLG;
                } else {
                    let DLH = rspice_limited_exp(DKU);
                    let DLI = AT * DLH;
                    let DLJ = DLI * DKR;
                    let DLK = DKR + (B / DKP);
                    let DLL = DLH - (((DLI + ((if (DLJ * (DLJ + DKQ)) >= AXS { (DLJ * (DLJ + DKQ)) } else { AXS }).ln())) - DKS) / ((AT + (1e0f64 / DLH)) + (DLK / ((DKR * DLH) + DKP))));
                    let DLM = AT * DLL;
                    let DLN = DLM * DKR;
                    let DLO = (DLM + ((if (DLN * (DLN + DKQ)) >= AXS { (DLN * (DLN + DKQ)) } else { AXS }).ln())) - DKS;
                    let DLP = (DKR * DLL) + DKP;
                    let DLQ = DLK / DLP;
                    let DLR = (AT + (1e0f64 / DLL)) + DLQ;
                    let DLS = B / DLL;
                    let DLT = DLL - ((DLO / DLR) * (B + ((DLO * (((-1e0f64 * (DLS * DLS)) - (1e0f64 / (((DKP * DKP) * DKP) * DLP))) - (DLQ * DLQ))) / ((AT * DLR) * DLR))));
                    DMG = DLT;
                }
                let DLU = if 0.0f64 != 0.0 && (if DKL < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DLX = if DLU != 0.0 {
                    let DLV = -4e0f64 / (CAD * DKL);
                    DLV
                } else {
                    let DLW = LP * (DKM + ((DKO + 1e0f64).sqrt()));
                    DLW
                };
                let DLY = (CQE * L) * AY;
                let DMC = ((AT * (B + (DKA / (AT * (DLX.sqrt()))))) * DLY) * BBO;
                let DMD = AU - DLZ;
                let DME = DJL * DLY;
                let DMF = (DAE * DMD) / ((DME * CAS) * CAS);
                let DMH = B + (SA * (((DMG * DMG) + DMG) - ((DAE * DLZ) / (DMC * BBO))));
                let DMI = if DMH < B { 1.0 } else { 0.0 };
                let DML = if DMI != 0.0 {
                    A
                } else {
                    let DMJ = -5e-1f64 + (LP * (DMH.sqrt()));
                    DMJ
                };
                let DMK = -5e-1f64 + (LP * ((B + (SA * (((CLF * CLF) + CLF) + DMF))).sqrt()));
                let DMM = (DMC * DML) * DMD;
                let DMN = ((((AT * DLY) * BBO) * (DMK - CLF)) * DLZ) + (((DME * BBO) * CLF) * DLZ);
                let DMO = DMM + DMN;
                let DMP = (B / DMO) / DMO;
                let DMQ = (DMM * DMM) * DMP;
                let DMR = (DMN * DMN) * DMP;
                let DMS = if AU != DLZ { 1.0 } else { 0.0 };
                let DNJ;
                if DMS != 0.0 {
                    let DMT = (DJM * DMK) / BCO;
                    let DMX = (AU - (AT * DMU)) - DLZ;
                    let DMY = DMX * DMX;
                    let DNC = ((DJO / ((DMZ * L) * DMY)) * (((DJR * ((if ((DMT + DJK) / DJV) >= AXS { ((DMT + DJK) / DJV) } else { AXS }).ln())) + (DJS * (DMT - DJN))) + ((LP * DJT) * ((DMT * DMT) - (DJN * DJN))))) + ((((DJQ / (((DMZ * DMY) * AY) * V)) * DNA) * DJU) / DJW);
                    let DND = ((DJX / (((((AY * V) * DMX) * DMZ) * DJK) * DJK)) * DAE) * DAE;
                    let DNE = DND + DNC;
                    let DNF = if DNE > A { 1.0 } else { 0.0 };
                    let DNK = if DNF != 0.0 {
                        let DNG = (DNC * DND) / DNE;
                        DNG
                    } else {
                        A
                    };
                    DNJ = DNK;
                } else {
                    DNJ = A;
                }
                let DNH = ((((parameters[1067] * BCO) * BBO) / (((((AY * V) * DLZ) * DMZ) * DJK) * DJK)) * DAE) * DAE;
                let DNI = if DNH > A { 1.0 } else { 0.0 };
                let DNL = if DNI != 0.0 {
                    DNH
                } else {
                    A
                };
                let DNN = (CUH * DNM) * ((DNJ * DMQ) + (DNL * DMR));
                EFC = DMU;
                EIR = B;
                EIS = DNN;
                EIT = DNO;
                EIU = A;
                EIV = A;
                EIW = A;
            } else {
                let DNP = if QL >= (AU / AT) { 1.0 } else { 0.0 };
                let DNX = if DNP != 0.0 {
                    A
                } else {
                    QL
                };
                let DNQ = if DJR > A { 1.0 } else { 0.0 };
                let DNR = if (if DNQ != 0.0 || (if DJS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DJT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DOH;
                if DNR != 0.0 {
                    let DNS = if (if IL != A { 1.0 } else { 0.0 }) != 0.0 && DNQ != 0.0 { 1.0 } else { 0.0 };
                    let DOB = if DNS != 0.0 {
                        let DNT = (IM / (B + ((CMD / IN).powf(IO)))) / DJR;
                        let DNU = DNT - B;
                        let DNW = DJR * (LP * ((DNT + B) + (((DNU * DNU) + ((MG * DNV) * DNV)).sqrt())));
                        DNW
                    } else {
                        DJR
                    };
                    let DNY = AU - (AT * DNX);
                    let DNZ = DNY * DNY;
                    let DOA = (((DJM * CEC) * CQB) * CQA) / BCO;
                    let DOC = ((DJO / ((DMZ * L) * DNZ)) * (((DOB * ((if ((DOA + DJK) / DJV) >= AXS { ((DOA + DJK) / DJV) } else { AXS }).ln())) + (DJS * (DOA - DJN))) + ((LP * DJT) * ((DOA * DOA) - (DJN * DJN))))) + ((((DJQ / (((DMZ * DNZ) * AY) * V)) * DNA) * DJU) / DJW);
                    let DOD = ((((DOB * BCO) * BBO) / (((((AY * V) * DNY) * DMZ) * DJK) * DJK)) * DAE) * DAE;
                    let DOE = DOD + DOC;
                    let DOF = if DOE > A { 1.0 } else { 0.0 };
                    let DOI = if DOF != 0.0 {
                        let DOG = ((DOC * DOD) / DOE) / (B + (parameters[802] * (CLN.powf(parameters[803]))));
                        DOG
                    } else {
                        A
                    };
                    DOH = DOI;
                } else {
                    DOH = A;
                }
                let DOJ = (CUH * DNM) * DOH;
                EFC = DNX;
                EIR = A;
                EIS = A;
                EIT = A;
                EIU = B;
                EIV = DOJ;
                EIW = DNO;
            }
            let DOK = (CMD / DJE) / AU;
            let DOL = DOK * DOK;
            let DOM = parameters[811] * (B + ((parameters[814] * AU) * DOL));
            let DON = parameters[812] * (B + ((parameters[815] * AU) * DOL));
            let DOO = parameters[1043] * (B + ((parameters[1044] * AU) * DOL));
            let DOP = parameters[813] * (B + ((parameters[816] * AU) * DOL));
            let DOQ = ((-AU) / parameters[1042]).exp();
            let DOR = ((((RZ * DOM) * DOM) - B) * DOQ) + B;
            let DOS = DOO * DOO;
            let DOT = DON * DON;
            let DOV = if DOU == A { 1.0 } else { 0.0 };
            let EIX;
            let EIY;
            let EIZ;
            let EJB;
            let EJD;
            let EJF;
            let EJH;
            let EJJ;
            if DOV != 0.0 {
                let DOW = ((((-V) * AY) * AU) * L) * BBO;
                let DOX = CQE * (((DOW * CLX) + (DOW * CLY)).abs());
                let DPA = CVS * (DJD * ((DOX / ((DOX * DOY) + (AU * AU))) * parameters[810]));
                EIX = B;
                EIY = DPA;
                EIZ = A;
                EJB = A;
                EJD = A;
                EJF = A;
                EJH = A;
                EJJ = A;
            } else {
                let DPB = if DOU == B { 1.0 } else { 0.0 };
                let EJA;
                let EJC;
                let EJE;
                let EJG;
                let EJI;
                let EJK;
                if DPB != 0.0 {
                    let DPC = (((CQE * COU) * COJ) * L) * (DJL * CAS);
                    let DPD = LP * CLS;
                    let DPE = DPD + LP;
                    let DPF = DPE * DPE;
                    let DPG = DPF * DPE;
                    let DPH = CLO * CLN;
                    let DPI = ((SB * DPD) + LP) * CLO;
                    let DPJ = AU * COU;
                    let DPK = DPJ / AU;
                    let DPL = (((B + ((DOS * (CJZ / CJW)) / (parameters[1045] + CMD))) - B) * DOQ) + B;
                    let DPM = if 1.0f64 != 0.0 && (if DPL < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DPP = if DPM != 0.0 {
                        let DPN = -1.0000000000000002e-2f64 / (CAD * DPL);
                        DPN
                    } else {
                        let DPO = LP * (DPL + (((DPL * DPL) + 2.5000000000000005e-3f64).sqrt()));
                        DPO
                    };
                    let DPR = DPQ * DPE;
                    let DPT = ((((((DPJ * DPK) * DPK) * (((DPD / DPF) - (DPI / ((6e1f64 * DPF) * DPF))) + ((CLO * CLO) / ((DPS * DPF) * DPG)))) * 1.5e1f64) / SA) * DOT) / (((V * AY) * DPQ) * DPC);
                    let DPU = ((DPK * ((CLN / DPR) - (DPH / (DPS * DPG)))) * DOP) / 3.95e-1f64;
                    let DPV = (DJD * ((((DPC * V) * AY) / DPJ) * ((DPD * DPP) + ((CLO * DOR) / DPR)))).sqrt();
                    let DPW = if DPT > A { 1.0 } else { 0.0 };
                    let DQA;
                    let DQC;
                    if DPW != 0.0 {
                        let DPX = (DJD / DPT).sqrt();
                        let DPY = if DPV > A { 1.0 } else { 0.0 };
                        let DQB = if DPY != 0.0 {
                            let DPZ = (DPU * DPX) / DPV;
                            DPZ
                        } else {
                            A
                        };
                        DQA = DQB;
                        DQC = DPX;
                    } else {
                        DQA = A;
                        DQC = A;
                    }
                    let DQD = B - DQA;
                    let DQE = (DQC * DQC) * DQD;
                    let DQF = ((CVS * DPV) * DPV) * DQD;
                    EJA = B;
                    EJC = DQA;
                    EJE = B;
                    EJG = DQE;
                    EJI = B;
                    EJK = DQF;
                } else {
                    EJA = A;
                    EJC = A;
                    EJE = A;
                    EJG = A;
                    EJI = A;
                    EJK = A;
                }
                EIX = A;
                EIY = A;
                EIZ = EJA;
                EJB = EJC;
                EJD = EJE;
                EJF = EJG;
                EJH = EJI;
                EJJ = EJK;
            }
            let EJL;
            let EJM;
            let EJN;
            let EJO;
            if PO != 0.0 {
                let DQG = (CVS * AT) * BCO;
                let DQH = DQG * ((DEZ + DET).abs());
                let DQI = DQG * ((DFC + DEV).abs());
                EJL = B;
                EJM = DQH;
                EJN = B;
                EJO = DQI;
            } else {
                EJL = A;
                EJM = A;
                EJN = A;
                EJO = A;
            }
            let EJP;
            let EJQ;
            if PL != 0.0 {
                let DQJ = ((CVS * AT) * BCO) * (DEX.abs());
                EJP = B;
                EJQ = DQJ;
            } else {
                EJP = A;
                EJQ = A;
            }
            let DQK = if parameters[40] == B { 1.0 } else { 0.0 };
            let DXU;
            let DXV;
            let DYA;
            let DYB;
            let DYE;
            let DYF;
            let DYR;
            let DYZ;
            let DZA;
            let DZC;
            if DQK != 0.0 {
                let DQL = BXZ * BBP;
                let DQM = (BWV * BBP) - ((NQ + CBH) * BBP);
                let DQN = (if (NK / BBZ) >= AXS { (NK / BBZ) } else { AXS }).ln();
                let DQO = ((((3.20438e-19f64 * H) * NK) * BBP).sqrt()) / L;
                let DQP = B / DQO;
                let DQQ = ((3.20438e-19f64 * H) * DJ) / ((L * L) * BBO);
                let DQS = if BCG != 0.0 {
                    let DQR = B / DQQ;
                    DQR
                } else {
                    A
                };
                let DQU = if BCG != 0.0 {
                    let DQT = NK / DJ;
                    DQT
                } else {
                    A
                };
                let DQV = B + DQU;
                let DQW = DQM / DQV;
                let DQX = DQO / DQV;
                let DQY = RZ * (B + (DQX / CBZ));
                let DQZ = (LP * DQW) - DQY;
                let DRA = DQZ + (((DQZ * DQZ) + (SB * DQW)).sqrt());
                let DRB = if DQW < A { 1.0 } else { 0.0 };
                let DRI = if DRB != 0.0 {
                    let DRC = (DQW - DRA) / DQX;
                    let DRD = -((if ((B - DRA) + (DRC * DRC)) >= AXS { ((B - DRA) + (DRC * DRC)) } else { AXS }).ln());
                    DRD
                } else {
                    let DRE = rspice_limited_exp((-DRA));
                    let DRF = LP * DQX;
                    let DRG = ((((DQW - B) + DRE) + (DRF * DRF)).sqrt()) - DRF;
                    let DRH = ((DRG * DRG) + B) - DRE;
                    DRH
                };
                let DRJ = DRI + B;
                let DRK = DRI - B;
                let DRL = DRK * DRK;
                let DRM = (LP * (DRJ + ((DRL + 1e0f64).sqrt()))).sqrt();
                let DRN = AT * DRM;
                let DRO = (B + (DQO / DRN)) / DQO;
                let DRP = DRI - (AT * DQN);
                let DRQ = DRP - DQL;
                let DRS = (DRQ / DRR) - ((if ((SA * DRO) * DRM) >= AXS { ((SA * DRO) * DRM) } else { AXS }).ln());
                let DRT = LP * ((DRS - CCT) - (((DRS * (DRS + CCU)) + CCV).sqrt()));
                let DRU = if DRT <= -6.8e1f64 { 1.0 } else { 0.0 };
                let DSZ;
                if DRU != 0.0 {
                    let DRW = if DRT < -1.1e2f64 { 1.0 } else { 0.0 };
                    let DSD;
                    if DRW != 0.0 {
                        DSD = DRX;
                    } else {
                        let DRY = if DRT > -9e1f64 { 1.0 } else { 0.0 };
                        let DSE = if DRY != 0.0 {
                            let DRZ = rspice_limited_exp(DRT);
                            DRZ
                        } else {
                            let DSA = (DRT - DRV) / BWB;
                            let DSB = DSA * DSA;
                            let DSC = rspice_limited_exp((DRV + (BWB * ((7.8125e-2f64 + (LP * DSA)) + (DSB * (9.375e-1f64 - (DSB * (CDF - DSB))))))));
                            DSC
                        };
                        DSD = DSE;
                    }
                    let DSF = DSD * (((B + DRQ) - (DRR * DRT)) - (DRR * ((if ((AT * DRO) * (((DSD * AT) * DRO) + DRN)) >= AXS { ((AT * DRO) * (((DSD * AT) * DRO) + DRN)) } else { AXS }).ln())));
                    DSZ = DSF;
                } else {
                    let DSG = rspice_limited_exp(DRT);
                    let DSH = AT * DSG;
                    let DSI = DSH * DRO;
                    let DSJ = DRO + (B / DRM);
                    let DSK = DRR * DSJ;
                    let DSL = DSG - (((DSH + (DRR * ((if (DSI * (DSI + DRN)) >= AXS { (DSI * (DSI + DRN)) } else { AXS }).ln()))) - DRQ) / ((AT + (DRR / DSG)) + (DSK / ((DRO * DSG) + DRM))));
                    let DSM = AT * DSL;
                    let DSN = DSM * DRO;
                    let DSO = (DSM + (DRR * ((if (DSN * (DSN + DRN)) >= AXS { (DSN * (DSN + DRN)) } else { AXS }).ln()))) - DRQ;
                    let DSP = (DRO * DSL) + DRM;
                    let DSQ = (AT + (DRR / DSL)) + (DSK / DSP);
                    let DSR = DSJ / DSP;
                    let DSS = B / DSL;
                    let DST = DSL - ((DSO / DSQ) * (B + ((DSO * ((((-DRR) * (DSS * DSS)) - (DRR / (((DRM * DRM) * DRM) * DSP))) - ((DRR * DSR) * DSR))) / ((AT * DSQ) * DSQ))));
                    DSZ = DST;
                }
                let DSU = if 0.0f64 != 0.0 && (if DRI < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DSX = if DSU != 0.0 {
                    let DSV = -4e0f64 / (CAD * DRI);
                    DSV
                } else {
                    let DSW = LP * (DRJ + ((DRL + 1e0f64).sqrt()));
                    DSW
                };
                let DSY = DSX.sqrt();
                let DTA = AT * DSZ;
                let DTB = DRI - DTA;
                let DTC = if 0.0f64 != 0.0 && (if DTB < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DTG = if DTC != 0.0 {
                    let DTD = -4e0f64 / (CAD * DTB);
                    DTD
                } else {
                    let DTE = DTB - B;
                    let DTF = LP * ((DTB + B) + (((DTE * DTE) + 1e0f64).sqrt()));
                    DTF
                };
                let DTH = B + (DQO / (DSY + (DTG.sqrt())));
                let DTI = DQM - DRI;
                let DTJ = DTH - B;
                let DTK = BBO * (DTI - (DTA * DTJ));
                let DTL = if 1.0f64 != 0.0 && (if DTK < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DTO = if DTL != 0.0 {
                    let DTM = -1.0000000000000002e-2f64 / (CAD * DTK);
                    DTM
                } else {
                    let DTN = LP * (DTK + (((DTK * DTK) + 2.5000000000000005e-3f64).sqrt()));
                    DTN
                };
                let DTP = CEW + (CEX * BYL);
                let DTQ = B + (DTP * ((CEM * (DTO + (BDH * (((AT * DTH) * BBO) * DSZ)))).powf(BEK)));
                let DTR = if 0.0f64 != 0.0 && (if DTQ < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DTV = if DTR != 0.0 {
                    let DTS = -2.25e-6f64 / (CAD * DTQ);
                    DTS
                } else {
                    let DTT = DTQ - B;
                    let DTU = LP * ((DTQ + B) + (((DTT * DTT) + 5.625e-7f64).sqrt()));
                    DTU
                };
                let DTX = DTW * BF;
                let DTY = ((CGF / DTV) * BBO) / DTX;
                let DTZ = AT * ((DTY * ((DSZ * DSZ) + DSZ)) / (B + (DTY * (B + DSZ))));
                let DUA = (DTZ * DTH) * DQP;
                let DUB = ((DRP - (DTZ + ((if (DUA * (DUA + (DQO / DTJ))) >= AXS { (DUA * (DUA + (DQO / DTJ))) } else { AXS }).ln()))) * BBO) - BXZ;
                let DUC = if 1.0f64 != 0.0 && (if DUB < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DUP = if DUC != 0.0 {
                    let DUD = -1e-6f64 / (CAD * DUB);
                    DUD
                } else {
                    let DUE = LP * (DUB + (((DUB * DUB) + 2.5e-7f64).sqrt()));
                    DUE
                };
                let DUH = if (if DUF == A { 1.0 } else { 0.0 }) != 0.0 && (if DUG == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DUQ;
                if DUH != 0.0 {
                    DUQ = DUI;
                } else {
                    let DUJ = AU / (AU + ((DW * CAJ).sqrt()));
                    let DUK = B + (((DUF * DUJ) - (((DUG * DUJ) * DSZ) * CAS)) / (B + (parameters[1136] * BYU)));
                    let DUL = if 0.0f64 != 0.0 && (if DUK < -1.25e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DUR = if DUL != 0.0 {
                        let DUM = -2.5e-7f64 / (CAD * DUK);
                        DUM
                    } else {
                        let DUN = DUK - BVW;
                        let DUO = LP * ((DUK + BVW) + (((DUN * DUN) + 6.25e-8f64).sqrt()));
                        DUO
                    };
                    DUQ = DUR;
                }
                let DUS = DUP / DUQ;
                let DUT = BYA * ((B + (((BXX / DUS) + BP).powf(CJX))).powf(CJY));
                let DUU = (LP * (DRJ + ((DRL + 1e0f64).sqrt()))).sqrt();
                let DUV = AT * DUU;
                let DUW = (B + (DQO / DUV)) / DQO;
                let DUX = DRP - ((DUT + BXZ) * BBP);
                let DUY = (DUX / DRR) - ((if ((SA * DUW) * DUU) >= AXS { ((SA * DUW) * DUU) } else { AXS }).ln());
                let DUZ = LP * ((DUY - CCT) - (((DUY * (DUY + CCU)) + CCV).sqrt()));
                let DVA = if DUZ <= -6.8e1f64 { 1.0 } else { 0.0 };
                let DWA;
                if DVA != 0.0 {
                    let DVC = if DUZ < -1.1e2f64 { 1.0 } else { 0.0 };
                    let DVJ;
                    if DVC != 0.0 {
                        DVJ = DVD;
                    } else {
                        let DVE = if DUZ > -9e1f64 { 1.0 } else { 0.0 };
                        let DVK = if DVE != 0.0 {
                            let DVF = rspice_limited_exp(DUZ);
                            DVF
                        } else {
                            let DVG = (DUZ - DVB) / BWB;
                            let DVH = DVG * DVG;
                            let DVI = rspice_limited_exp((DVB + (BWB * ((7.8125e-2f64 + (LP * DVG)) + (DVH * (9.375e-1f64 - (DVH * (CDF - DVH))))))));
                            DVI
                        };
                        DVJ = DVK;
                    }
                    let DVL = DVJ * (((B + DUX) - (DRR * DUZ)) - (DRR * ((if ((AT * DUW) * (((DVJ * AT) * DUW) + DUV)) >= AXS { ((AT * DUW) * (((DVJ * AT) * DUW) + DUV)) } else { AXS }).ln())));
                    DWA = DVL;
                } else {
                    let DVM = rspice_limited_exp(DUZ);
                    let DVN = AT * DVM;
                    let DVO = DVN * DUW;
                    let DVP = DUW + (B / DUU);
                    let DVQ = DRR * DVP;
                    let DVR = DVM - (((DVN + (DRR * ((if (DVO * (DVO + DUV)) >= AXS { (DVO * (DVO + DUV)) } else { AXS }).ln()))) - DUX) / ((AT + (DRR / DVM)) + (DVQ / ((DUW * DVM) + DUU))));
                    let DVS = AT * DVR;
                    let DVT = DVS * DUW;
                    let DVU = (DVS + (DRR * ((if (DVT * (DVT + DUV)) >= AXS { (DVT * (DVT + DUV)) } else { AXS }).ln()))) - DUX;
                    let DVV = (DUW * DVR) + DUU;
                    let DVW = (AT + (DRR / DVR)) + (DVQ / DVV);
                    let DVX = DVP / DVV;
                    let DVY = B / DVR;
                    let DVZ = DVR - ((DVU / DVW) * (B + ((DVU * ((((-DRR) * (DVY * DVY)) - (DRR / (((DUU * DUU) * DUU) * DVV))) - ((DRR * DVX) * DVX))) / ((AT * DVW) * DVW))));
                    DWA = DVZ;
                }
                let DWB = ((DRI - DSZ) - DWA) - B;
                let DWC = if 0.0f64 != 0.0 && (if DWB < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DWG = if DWC != 0.0 {
                    let DWD = -4e0f64 / (CAD * DWB);
                    DWD
                } else {
                    let DWE = DWB - B;
                    let DWF = LP * ((DWB + B) + (((DWE * DWE) + 1e0f64).sqrt()));
                    DWF
                };
                let DWH = DWG.sqrt();
                let DWI = DQV + (DQO / (DUU + DWH));
                let DWJ = LP + ((DQU * DWH) * DQP);
                let DWK = DSZ + DWA;
                let DWL = DWI / (DWJ + (((DWJ * DWJ) + ((DWI * DWK) * DQS)).sqrt()));
                let DWM = DWL - B;
                let DWN = BBO * (DTI - (DTA * DWM));
                let DWO = if 1.0f64 != 0.0 && (if DWN < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DWV = if DWO != 0.0 {
                    let DWP = -1.0000000000000002e-2f64 / (CAD * DWN);
                    DWP
                } else {
                    let DWQ = LP * (DWN + (((DWN * DWN) + 2.5000000000000005e-3f64).sqrt()));
                    DWQ
                };
                let DWR = BBO * (DTI - ((AT * DWA) * DWM));
                let DWS = if 1.0f64 != 0.0 && (if DWR < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DWW = if DWS != 0.0 {
                    let DWT = -1.0000000000000002e-2f64 / (CAD * DWR);
                    DWT
                } else {
                    let DWU = LP * (DWR + (((DWR * DWR) + 2.5000000000000005e-3f64).sqrt()));
                    DWU
                };
                let DWX = CEM * ((LP * (DWV + DWW)) + (BDH * ((DWL * BBO) * DWK)));
                let DWY = (DQM + (parameters[136] * BBP)) / DQV;
                let DWZ = (LP * DWY) - DQY;
                let DXA = DWZ + (((DWZ * DWZ) + (SB * DWY)).sqrt());
                let DXB = if DWY < A { 1.0 } else { 0.0 };
                let DYC = if DXB != 0.0 {
                    let DXC = (DWY - DXA) / DQX;
                    let DXD = -((if ((B - DXA) + (DXC * DXC)) >= AXS { ((B - DXA) + (DXC * DXC)) } else { AXS }).ln());
                    DXD
                } else {
                    let DXE = rspice_limited_exp((-DXA));
                    let DXF = LP * DQX;
                    let DXG = ((((DWY - B) + DXE) + (DXF * DXF)).sqrt()) - DXF;
                    let DXH = ((DXG * DXG) + B) - DXE;
                    DXH
                };
                let DXI = B + (DTP * (DWX.powf(BEK)));
                let DXJ = if 0.0f64 != 0.0 && (if DXI < -3.75e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DXN = if DXJ != 0.0 {
                    let DXK = -2.25e-6f64 / (CAD * DXI);
                    DXK
                } else {
                    let DXL = DXI - B;
                    let DXM = LP * ((DXI + B) + (((DXL * DXL) + 5.625e-7f64).sqrt()));
                    DXM
                };
                let DXO = CGF / DXN;
                let DXP = (((AT * DXO) * BBO) / DTX) * (DSZ - DWA);
                let DXQ = LP * (B + ((B + ((AT * DXP) * DXP)).sqrt()));
                let DXR = DUS + (((AT * DTW) / DXO) * BF);
                let DXS = BYA - DUT;
                DXU = DXS;
                DXV = DXR;
                DYA = DQM;
                DYB = DYC;
                DYE = DSZ;
                DYF = DWA;
                DYR = DQS;
                DYZ = DUQ;
                DZA = DXQ;
                DZC = DWL;
            } else {
                DXU = CMR;
                DXV = CNE;
                DYA = CBM;
                DYB = CCJ;
                DYE = CEC;
                DYF = CLF;
                DYR = A;
                DYZ = B;
                DZA = COU;
                DZC = CLM;
            }
            let DXT = if NV != A { 1.0 } else { 0.0 };
            let DXX = if DXT != 0.0 {
                let DXW = B + (NV * ((if (B + ((DXU / NV) / DXV)) >= AXS { (B + ((DXU / NV) / DXV)) } else { AXS }).ln()));
                DXW
            } else {
                B
            };
            let DXY = B / DXX;
            let DXZ = DXX - B;
            let DYD = DYA - DYB;
            let DYG = DYE - DYF;
            let DYH = DYG * DYG;
            let DYI = DYD + (AT * DYE);
            let DYJ = DYD + (AT * DYF);
            let DYK = if 1.0f64 != 0.0 && (if DYI < -1.25e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DYQ = if DYK != 0.0 {
                let DYL = -2.5e-1f64 / (CAD * DYI);
                DYL
            } else {
                let DYM = LP * (DYI + (((DYI * DYI) + 6.25e-2f64).sqrt()));
                DYM
            };
            let DYN = if 1.0f64 != 0.0 && (if DYJ < -1.25e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DYT = if DYN != 0.0 {
                let DYO = -2.5e-1f64 / (CAD * DYJ);
                DYO
            } else {
                let DYP = LP * (DYJ + (((DYJ * DYJ) + 6.25e-2f64).sqrt()));
                DYP
            };
            let DYS = (MG + (DYQ * DYR)).sqrt();
            let DYU = (MG + (DYT * DYR)).sqrt();
            let DYV = AT * DYU;
            let DYW = B + DYV;
            let DYX = DYS + DYU;
            let DYY = DYX * DYX;
            let DZB = ((DYZ * DZA) * DXY) / ((B + DYE) + DYF);
            let DZD = if 1.0f64 != 0.0 && (if (BBO * ((DXY * (((DYI / (B + (AT * DYS))) + (DYJ / DYW)) + (((BDD * (DYH / (DYY * DYX))) * (((CLV * (DYY + (DYS * DYU))) * DZB) + (AT * DYR))) - (DZC * ((DYE + DYF) + ((BDD * DYH) * DZB)))))) + (DXZ * ((DYD - ((AT * (DZC - B)) * DYF)) + ((DYJ * (DYV - B)) / DYW))))) < (-2.5e3f64 * parameters[694]) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if DZD != 0.0 {
            } else {
            }
            let DZE = if (if parameter_given[666] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            if DZE != 0.0 {
            } else {
            }
            let DZF = if parameters[41] == A { 1.0 } else { 0.0 };
            if DZF != 0.0 {
            } else {
            }
            let DZG = if BVB == B { 1.0 } else { 0.0 };
            let EFT;
            let EJR;
            let EJS;
            let EJT;
            if DZG != 0.0 {
                let DZH = (if (HN / BBZ) >= AXS { (HN / BBZ) } else { AXS }).ln();
                let DZI = if ((BCJ + (BBO * DZH)) + DX) >= BCJ { ((BCJ + (BBO * DZH)) + DX) } else { BCJ };
                let DZJ = (BCN / (BCO * HN)).sqrt();
                let DZK = B + (HX * BCR);
                let DZL = if DZK < -1e1f64 { 1.0 } else { 0.0 };
                let DZO = if DZL != 0.0 {
                    let DZM = -1e-6f64 / DZK;
                    DZM
                } else {
                    let DZN = LP * (DZK + (((DZK * DZK) + 4e-6f64).sqrt()));
                    DZN
                };
                let DZP = HM * DZO;
                let DZR = DZQ * (B + (HY * BCR));
                let DZS = DZI - BYU;
                let DZT = if 0.0f64 != 0.0 && (if DZS < -2.5e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DZX = if DZT != 0.0 {
                    let DZU = -1.0000000000000002e-2f64 / (CAD * DZS);
                    DZU
                } else {
                    let DZV = DZS - BVZ;
                    let DZW = LP * ((DZS + BVZ) + (((DZV * DZV) + 2.5000000000000005e-3f64).sqrt()));
                    DZW
                };
                let DZY = H / (DZJ * (DZX.sqrt()));
                let DZZ = B + ((((HO + DZP) + (HP * BYT)) - (HQ * BYU)) / L);
                let EAA = if 0.0f64 != 0.0 && (if DZZ < -1.25e2f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EAE = if EAA != 0.0 {
                    let EAB = -2.5000000000000005e-3f64 / (CAD * DZZ);
                    EAB
                } else {
                    let EAC = DZZ - B;
                    let EAD = LP * ((DZZ + B) + (((EAC * EAC) + 6.250000000000001e-4f64).sqrt()));
                    EAD
                };
                let EAF = EAE * BBO;
                let EAG = B / EAF;
                let EAH = BWV * EAG;
                let EAI = BXW * EAG;
                let EAJ = CBI * EAG;
                let EAK = (-(DZR + (HS * BYU))) * BYT;
                let EAL = ((HT + (HU / AU)) + (HV * BYU)) * ((BBR.powf(HW)) - B);
                let EAM = BCQ * (B + (parameters[1016] * BYU));
                let EAN = if EAM > A { 1.0 } else { 0.0 };
                let EAT;
                if EAN != 0.0 {
                    let EAO = (parameters[1015] * AU) / EAM;
                    let EAP = if EAO < 4e1f64 { 1.0 } else { 0.0 };
                    let EAU = if EAP != 0.0 {
                        let EAR = (LP * EAQ) / ((EAO.cosh()) - B);
                        EAR
                    } else {
                        let EAS = EAQ * (rspice_limited_exp((-EAO)));
                        EAS
                    };
                    EAT = EAU;
                } else {
                    EAT = A;
                }
                let EAZ = (EAH - EAJ) - (((((((EAK - EAL) + (EAT * (EAV - DZI))) + parameters[961]) + EAW) - ((EAY + BWP) * BYU)) + BWO) * EAG);
                let EBA = (((((3.20438e-19f64 * H) * HN) * EAG).sqrt()) / L) * (B + (parameters[958] * (B + (parameters[959] * (AU.powf((-parameters[960])))))));
                let EBB = DZH / EAE;
                let EBC = (LP * EAZ) - (RZ * (B + (EBA / CBZ)));
                let EBD = EBC + (((EBC * EBC) + (SB * EAZ)).sqrt());
                let EBE = if EAZ < A { 1.0 } else { 0.0 };
                let EBL = if EBE != 0.0 {
                    let EBF = (EAZ - EBD) / EBA;
                    let EBG = -((if ((B - EBD) + (EBF * EBF)) >= AXS { ((B - EBD) + (EBF * EBF)) } else { AXS }).ln());
                    EBG
                } else {
                    let EBH = rspice_limited_exp((-EBD));
                    let EBI = LP * EBA;
                    let EBJ = ((((EAZ - B) + EBH) + (EBI * EBI)).sqrt()) - EBI;
                    let EBK = ((EBJ * EBJ) + B) - EBH;
                    EBK
                };
                let EBM = EBL + B;
                let EBN = EBL - B;
                let EBO = EBN * EBN;
                let EBP = (LP * (EBM + ((EBO + 1e0f64).sqrt()))).sqrt();
                let EBQ = AT * EBP;
                let EBR = (B + (EBA / EBQ)) / EBA;
                let EBS = EBL - (AT * EBB);
                let EBT = EBS - EAI;
                let EBU = EBT - ((if ((SA * EBR) * EBP) >= AXS { ((SA * EBR) * EBP) } else { AXS }).ln());
                let EBV = LP * ((EBU - CCT) - (((EBU * (EBU + CCU)) + CCV).sqrt()));
                let EBW = if EBV <= -6.8e1f64 { 1.0 } else { 0.0 };
                let ECW;
                if EBW != 0.0 {
                    let EBY = if EBV < -1.1e2f64 { 1.0 } else { 0.0 };
                    let ECF;
                    if EBY != 0.0 {
                        ECF = EBZ;
                    } else {
                        let ECA = if EBV > -9e1f64 { 1.0 } else { 0.0 };
                        let ECG = if ECA != 0.0 {
                            let ECB = rspice_limited_exp(EBV);
                            ECB
                        } else {
                            let ECC = (EBV - EBX) / BWB;
                            let ECD = ECC * ECC;
                            let ECE = rspice_limited_exp((EBX + (BWB * ((7.8125e-2f64 + (LP * ECC)) + (ECD * (9.375e-1f64 - (ECD * (CDF - ECD))))))));
                            ECE
                        };
                        ECF = ECG;
                    }
                    let ECH = ECF * (((B + EBT) - EBV) - ((if ((AT * EBR) * (((ECF * AT) * EBR) + EBQ)) >= AXS { ((AT * EBR) * (((ECF * AT) * EBR) + EBQ)) } else { AXS }).ln()));
                    ECW = ECH;
                } else {
                    let ECI = rspice_limited_exp(EBV);
                    let ECJ = AT * ECI;
                    let ECK = ECJ * EBR;
                    let ECL = EBR + (B / EBP);
                    let ECM = ECI - (((ECJ + ((if (ECK * (ECK + EBQ)) >= AXS { (ECK * (ECK + EBQ)) } else { AXS }).ln())) - EBT) / ((AT + (1e0f64 / ECI)) + (ECL / ((EBR * ECI) + EBP))));
                    let ECN = AT * ECM;
                    let ECO = ECN * EBR;
                    let ECP = (ECN + ((if (ECO * (ECO + EBQ)) >= AXS { (ECO * (ECO + EBQ)) } else { AXS }).ln())) - EBT;
                    let ECQ = (EBR * ECM) + EBP;
                    let ECR = ECL / ECQ;
                    let ECS = (AT + (1e0f64 / ECM)) + ECR;
                    let ECT = B / ECM;
                    let ECU = ECM - ((ECP / ECS) * (B + ((ECP * (((-1e0f64 * (ECT * ECT)) - (1e0f64 / (((EBP * EBP) * EBP) * ECQ))) - (ECR * ECR))) / ((AT * ECS) * ECS))));
                    ECW = ECU;
                }
                let ECV = AT * EAF;
                let ECX = (((ECV * ECW) + ECV) + BXW) - BXW;
                let ECY = if 1.0f64 != 0.0 && (if ECX < -2.5e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EDB = if ECY != 0.0 {
                    let ECZ = -1e-6f64 / (CAD * ECX);
                    ECZ
                } else {
                    let EDA = LP * (ECX + (((ECX * ECX) + 2.5e-7f64).sqrt()));
                    EDA
                };
                let EDC = (LP * (EBM + ((EBO + 1e0f64).sqrt()))).sqrt();
                let EDD = AT * EDC;
                let EDE = (B + (EBA / EDD)) / EBA;
                let EDF = EBS - (((BXX * ((B + ((BXX / EDB).powf(CJX))).powf(CJY))) + BXW) * EAG);
                let EDG = EDF - ((if ((SA * EDE) * EDC) >= AXS { ((SA * EDE) * EDC) } else { AXS }).ln());
                let EDH = LP * ((EDG - CCT) - (((EDG * (EDG + CCU)) + CCV).sqrt()));
                let EDI = if EDH <= -6.8e1f64 { 1.0 } else { 0.0 };
                let EEM;
                if EDI != 0.0 {
                    let EDK = if EDH < -1.1e2f64 { 1.0 } else { 0.0 };
                    let EDR;
                    if EDK != 0.0 {
                        EDR = EDL;
                    } else {
                        let EDM = if EDH > -9e1f64 { 1.0 } else { 0.0 };
                        let EDS = if EDM != 0.0 {
                            let EDN = rspice_limited_exp(EDH);
                            EDN
                        } else {
                            let EDO = (EDH - EDJ) / BWB;
                            let EDP = EDO * EDO;
                            let EDQ = rspice_limited_exp((EDJ + (BWB * ((7.8125e-2f64 + (LP * EDO)) + (EDP * (9.375e-1f64 - (EDP * (CDF - EDP))))))));
                            EDQ
                        };
                        EDR = EDS;
                    }
                    let EDT = EDR * (((B + EDF) - EDH) - ((if ((AT * EDE) * (((EDR * AT) * EDE) + EDD)) >= AXS { ((AT * EDE) * (((EDR * AT) * EDE) + EDD)) } else { AXS }).ln()));
                    EEM = EDT;
                } else {
                    let EDU = rspice_limited_exp(EDH);
                    let EDV = AT * EDU;
                    let EDW = EDV * EDE;
                    let EDX = EDE + (B / EDC);
                    let EDY = EDU - (((EDV + ((if (EDW * (EDW + EDD)) >= AXS { (EDW * (EDW + EDD)) } else { AXS }).ln())) - EDF) / ((AT + (1e0f64 / EDU)) + (EDX / ((EDE * EDU) + EDC))));
                    let EDZ = AT * EDY;
                    let EEA = EDZ * EDE;
                    let EEB = (EDZ + ((if (EEA * (EEA + EDD)) >= AXS { (EEA * (EEA + EDD)) } else { AXS }).ln())) - EDF;
                    let EEC = (EDE * EDY) + EDC;
                    let EED = EDX / EEC;
                    let EEE = (AT + (1e0f64 / EDY)) + EED;
                    let EEF = B / EDY;
                    let EEG = EDY - ((EEB / EEE) * (B + ((EEB * (((-1e0f64 * (EEF * EEF)) - (1e0f64 / (((EDC * EDC) * EDC) * EEC))) - (EED * EED))) / ((AT * EEE) * EEE))));
                    EEM = EEG;
                }
                let EEH = if 0.0f64 != 0.0 && (if EBL < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EEK = if EEH != 0.0 {
                    let EEI = -4e0f64 / (CAD * EBL);
                    EEI
                } else {
                    let EEJ = LP * (EBM + ((EBO + 1e0f64).sqrt()));
                    EEJ
                };
                let EEL = EEK.sqrt();
                let EEN = ((EBL - ECW) - EEM) - B;
                let EEO = if 0.0f64 != 0.0 && (if EEN < -5e3f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EES = if EEO != 0.0 {
                    let EEP = -4e0f64 / (CAD * EEN);
                    EEP
                } else {
                    let EEQ = EEN - B;
                    let EER = LP * ((EEN + B) + (((EEQ * EEQ) + 1e0f64).sqrt()));
                    EER
                };
                let EET = B + (EBA / (EEL + (EES.sqrt())));
                let EEV = ECW - EEM;
                let EEW = ((((((((CQF * EET) * CQE) * EEU) / AU) * L) * EAF) * EAF) * (EEV * ((B + ECW) + EEM))) * COJ;
                let EEX = EEW + DAE;
                let EEZ = DJR * EEY;
                let EFA = DJS * EEY;
                let EFB = DJT * EEY;
                let EFD = AU - (AT * EFC);
                let EFE = DJJ * ((L + DZY) + HO);
                let EFF = ((AT * EET) * L) * BBO;
                let EFG = (EFF * EEM) / BCO;
                let EFH = EFG + EFE;
                let EFI = (EFF * ECW) / BCO;
                let EFJ = (((((4.112842231783458e-57f64 * BBO) * (EEW.abs())) * CQE) / EES) * (((EEZ * ((if ((EFI + EFE) / EFH) >= AXS { ((EFI + EFE) / EFH) } else { AXS }).ln())) + (EFA * (EFI - EFG))) + ((LP * EFB) * ((EFI * EFI) - (EFG * EFG))))) + ((((((DJP * EEW) * EEW) / (((DMZ * (EFD * EFD)) * EEU) * V)) * DNA) * ((EEZ + (EFA * EFG)) + ((EFB * EFG) * EFG))) / (EFH * EFH));
                let EFK = ((((EEZ * BCO) * BBO) / (((((EEU * V) * EFD) * DMZ) * EFE) * EFE)) * EEW) * EEW;
                let EFL = EFK + EFJ;
                let EFM = if EFL > A { 1.0 } else { 0.0 };
                let EFO = if EFM != 0.0 {
                    let EFN = ((EFJ * EFK) / EFL) / (B + (parameters[1063] * (EEV.powf(parameters[1064]))));
                    EFN
                } else {
                    A
                };
                let EFP = (CUH * DNM) * EFO;
                EFT = EEX;
                EJR = B;
                EJS = EFP;
                EJT = DNO;
            } else {
                EFT = DAE;
                EJR = A;
                EJS = A;
                EJT = A;
            }
            let EFQ = if CUH > A { 1.0 } else { 0.0 };
            if EFQ != 0.0 {
            } else {
            }
            let EFR = if BFP != 0.0 && BXM != 0.0 { 1.0 } else { 0.0 };
            if EFR != 0.0 {
                let EFS = if CXV == B { 1.0 } else { 0.0 };
                if EFS != 0.0 {
                } else {
                }
            } else {
            }
            if EFQ != 0.0 {
            } else {
            }
            if EFQ != 0.0 {
            } else {
            }
            if PL != 0.0 {
            } else {
            }
            if PO != 0.0 {
            } else {
            }
            if EFQ != 0.0 {
            } else {
            }
            let EFU = if ON != AT { 1.0 } else { 0.0 };
            let EFV = if EFU != 0.0 && (if CFQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let EJU;
            let EJV;
            let EJW;
            let EJY;
            let EKA;
            let EKC;
            let EKE;
            if EFV != 0.0 {
                let EFY = CVS * DJD;
                let EFZ = EFY * (B / EFW);
                let EGA = if CQH != 0.0 && (if BXK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EJX;
                let EJZ;
                let EKB;
                let EKD;
                let EKF;
                if EGA != 0.0 {
                    let EGD = EFY * (B / EGB);
                    let EGE = ((DNM * parameters[807]) * Q) * ((EFT / Q).powf(parameters[809]));
                    EJX = B;
                    EJZ = EGD;
                    EKB = B;
                    EKD = EGE;
                    EKF = EGF;
                } else {
                    EJX = A;
                    EJZ = A;
                    EKB = A;
                    EKD = A;
                    EKF = A;
                }
                EJU = B;
                EJV = EFZ;
                EJW = EJX;
                EJY = EJZ;
                EKA = EKB;
                EKC = EKD;
                EKE = EKF;
            } else {
                EJU = A;
                EJV = A;
                EJW = A;
                EJY = A;
                EKA = A;
                EKC = A;
                EKE = A;
            }
            let EGG = if EFU != 0.0 && (if CFN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let EKG;
            let EKH;
            let EKI;
            let EKK;
            let EKM;
            let EKO;
            let EKQ;
            if EGG != 0.0 {
                let EGJ = CVS * DJD;
                let EGK = EGJ * (B / EGH);
                let EGL = if CQH != 0.0 && (if CSX > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EKJ;
                let EKL;
                let EKN;
                let EKP;
                let EKR;
                if EGL != 0.0 {
                    let EGO = EGJ * (B / EGM);
                    let EGP = ((DNM * parameters[806]) * Q) * ((EFT / Q).powf(parameters[808]));
                    EKJ = B;
                    EKL = EGO;
                    EKN = B;
                    EKP = EGP;
                    EKR = EGQ;
                } else {
                    EKJ = A;
                    EKL = A;
                    EKN = A;
                    EKP = A;
                    EKR = A;
                }
                EKG = B;
                EKH = EGK;
                EKI = EKJ;
                EKK = EKL;
                EKM = EKN;
                EKO = EKP;
                EKQ = EKR;
            } else {
                EKG = A;
                EKH = A;
                EKI = A;
                EKK = A;
                EKM = A;
                EKO = A;
                EKQ = A;
            }
            let EGR = if BAN == A { 1.0 } else { 0.0 };
            let EKS;
            let EKT;
            if EGR != 0.0 {
                EKS = A;
                EKT = A;
            } else {
                let EGS = if BAN == AT { 1.0 } else { 0.0 };
                let EGY = if EGS != 0.0 {
                    let EGX = (EGT * EGT) / EGV;
                    EGX
                } else {
                    EGV
                };
                let EGZ = (CVS * DJD) * EGY;
                EKS = B;
                EKT = EGZ;
            }
            let EHA = if BAN == RZ { 1.0 } else { 0.0 };
            if EHA != 0.0 {
            } else {
            }
            if BBD != 0.0 {
                if EFV != 0.0 {
                    let EHB = if CQH != 0.0 && (if BXK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if EHB != 0.0 {
                    } else {
                    }
                } else {
                }
                if EGG != 0.0 {
                    let EHC = if CQH != 0.0 && (if CSX > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if EHC != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let EKU;
            let EKV;
            let EKW;
            let EKX;
            let EKY;
            let EKZ;
            let ELA;
            let ELB;
            let ELC;
            let ELD;
            if AXQ != 0.0 {
                let EIG = DJD * CVS;
                let EIH = EIG * EHD;
                let EII = EIG * EHJ;
                let EIJ = EIG * EHO;
                let EIK = EIG * EIA;
                let EIL = EIG * EHV;
                EKU = B;
                EKV = EIH;
                EKW = B;
                EKX = EII;
                EKY = B;
                EKZ = EIJ;
                ELA = B;
                ELB = EIK;
                ELC = B;
                ELD = EIL;
            } else {
                EKU = A;
                EKV = A;
                EKW = A;
                EKX = A;
                EKY = A;
                EKZ = A;
                ELA = A;
                ELB = A;
                ELC = A;
                ELD = A;
            }
            if AXQ != 0.0 {
                let EIM = if BAC == A { 1.0 } else { 0.0 };
                if EIM != 0.0 {
                } else {
                }
            } else {
            }
            let EIN = if AXQ != 0.0 && BAD != 0.0 { 1.0 } else { 0.0 };
            let ELE;
            let ELF;
            if EIN != 0.0 {
                let EIQ = (DJD * CVS) * EIO;
                ELE = B;
                ELF = EIQ;
            } else {
                ELE = A;
                ELF = A;
            }
            if EIN != 0.0 {
            } else {
            }
        if EIR == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EIS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EIT);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EIU == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EIV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EIW);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EIX == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EIY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EIZ == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJD == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJH == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJL == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJN == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJP == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJR == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EJT);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJU == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EJW == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EJY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKA == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EKE);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKG == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKI == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKM == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EKQ);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKS == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKU == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKW == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EKY == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EKZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ELA == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ELB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ELC == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ELD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ELE == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ELF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
