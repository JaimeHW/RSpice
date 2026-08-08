#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::rspice_limited_exp;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 21] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 4, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "N2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "N1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 14, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "N2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "N1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF_EDGEFET", label: Some("1overf_edgefet"), kind: GeneratedNoiseKind::Flicker, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GM_RG", label: Some("rg"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "gm", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
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
            let B = 1e0f64;
            let C = 1.602176462e-19f64;
            let D = 8.617342301212761e-5f64;
            let E = parameters[30];
            let G = -1e0f64;
            let H = parameters[109];
            let I = 8.8541878128e-12f64;
            let K = parameters[110];
            let M = parameters[76];
            let Q = 3.9e0f64;
            let S = parameters[77];
            let T = parameters[49];
            let V = parameters[1];
            let W = parameters[50];
            let AA = parameters[2];
            let AI = parameters[54];
            let AJ = parameters[55];
            let AK = parameters[56];
            let AL = parameters[57];
            let AM = parameters[64];
            let AP = parameters[65];
            let AT = parameters[60];
            let AU = parameters[61];
            let AV = parameters[62];
            let AW = parameters[63];
            let AY = 2e0f64;
            let BB = 1e-9f64;
            let BD = parameters[1375];
            let BJ = parameters[71];
            let BK = parameters[72];
            let BL = parameters[73];
            let BX = 1e-6f64;
            let CF = parameters[1026];
            let CL = parameters[1027];
            let LM = parameters[81];
            let LN = parameters[83];
            let LO = parameters[85];
            let LQ = parameters[238];
            let LR = parameters[240];
            let LT = parameters[283];
            let MA = parameters[290];
            let ME = parameters[339];
            let MG = parameters[338];
            let MS = parameters[350];
            let MT = parameters[352];
            let MY = parameters[367];
            let MZ = parameters[369];
            let NB = parameters[374];
            let NG = parameters[392];
            let NH = parameters[394];
            let NM = parameters[202];
            let NR = parameters[204];
            let NT = parameters[532];
            let NY = parameters[314];
            let NZ = 5e-1f64;
            let OB = parameters[550];
            let OD = parameters[406];
            let OI = parameters[300];
            let OJ = parameters[302];
            let OO = parameters[488];
            let OQ = 2.5e-1f64;
            let OU = parameters[505];
            let OZ = parameters[603];
            let PH = parameters[93];
            let PI = parameters[95];
            let PJ = parameters[97];
            let PN = parameters[124];
            let PO = parameters[126];
            let PQ = parameters[134];
            let PR = parameters[136];
            let PT = parameters[320];
            let PU = parameters[322];
            let PW = parameters[417];
            let PY = parameters[210];
            let PZ = parameters[212];
            let QB = parameters[1198];
            let QC = parameters[1200];
            let QE = parameters[220];
            let QF = parameters[222];
            let QH = parameters[1267];
            let QI = parameters[1269];
            let QK = parameters[448];
            let QR = parameters[33];
            let QT = parameters[462];
            let QV = parameters[472];
            let QX = parameters[479];
            let RR = parameters[141];
            let SJ = 6.7e-2f64;
            let SS = 1e1f64;
            let SU = parameters[1396];
            let SW = parameters[898];
            let SY = parameters[896];
            let TA = if parameter_given[3] { 1.0 } else { 0.0 };
            let TB = parameters[438];
            let TD = parameters[9];
            let TF = parameters[8];
            let TG = 9e0f64;
            let TK = parameters[6];
            let TO = 1.0f64;
            let TY = 1.0f64;
            let TZ = 1.0f64;
            let UA = 5e0f64;
            let UH = 3e0f64;
            let UI = 4e0f64;
            let UJ = 6e0f64;
            let UP = 7e0f64;
            let UT = 8e0f64;
            let UZ = 0.0f64;
            let VU = 1.0f64;
            let VV = 1.0f64;
            let WM = 0.0f64;
            let XC = 1.0f64;
            let XD = 1.0f64;
            let XS = 0.0f64;
            let YK = 1.0f64;
            let YL = 1.0f64;
            let ZA = 0.0f64;
            let ZQ = 1.0f64;
            let ZR = 1.0f64;
            let AAK = 1.0f64;
            let AAL = 1.0f64;
            let ABD = 1.0f64;
            let ABF = 0.0f64;
            let ABX = 1.0f64;
            let ACA = 0.0f64;
            let ACS = 1.0f64;
            let ACY = 1.0f64;
            let AJD = if parameter_given[4] { 1.0 } else { 0.0 };
            let AJM = 0.0f64;
            let AJW = 0.0f64;
            let AJX = 1.0f64;
            let AKT = 0.0f64;
            let ALQ = 0.0f64;
            let ALR = 1.0f64;
            let AMI = 0.0f64;
            let AMY = 0.0f64;
            let AMZ = 1.0f64;
            let ANO = 0.0f64;
            let AOG = 0.0f64;
            let AOH = 1.0f64;
            let AOW = 0.0f64;
            let APM = 0.0f64;
            let APN = 1.0f64;
            let AQG = 0.0f64;
            let AQH = 1.0f64;
            let AQZ = 0.0f64;
            let ARB = 0.0f64;
            let ART = 0.0f64;
            let ARW = 0.0f64;
            let ASO = 0.0f64;
            let ASU = 0.0f64;
            let AZG = parameters[1347];
            let AZW = parameters[22];
            let BAA = 1e3f64;
            let BAB = parameters[7];
            let BAE = parameters[722];
            let BAF = 1e-38f64;
            let BAI = parameters[703];
            let BAJ = parameters[702];
            let BAL = parameters[705];
            let BAM = parameters[704];
            let BAO = parameters[1373];
            let BAY = parameters[40];
            let BBA = parameters[1028];
            let BBC = 3.0015e2f64;
            let BBG = node_potentials[4];
            let BBH = node_potentials[5];
            let BCD = 0.0f64;
            let BCE = parameters[43];
            let BCG = parameters[45];
            let BCR = 4e-1f64;
            let BDB = 1e-3f64;
            let BDI = 3.333333333333333e-1f64;
            let BDJ = parameters[347];
            let BEZ = 1e2f64;
            let BGT = 1e-2f64;
            let BLI = if parameter_given[17] { 1.0 } else { 0.0 };
            let BLX = if parameter_given[18] { 1.0 } else { 0.0 };
            let BMM = if parameter_given[19] { 1.0 } else { 0.0 };
            let BMN = parameters[926];
            let BMP = parameters[19];
            let BNE = if parameter_given[20] { 1.0 } else { 0.0 };
            let BNG = parameters[20];
            let BNV = parameters[10];
            let BNW = parameters[11];
            let BNX = parameters[12];
            let BOW = parameters[1106];
            let BPG = parameters[27];
            let BPR = parameters[13];
            let BPS = parameters[14];
            let BPT = parameters[15];
            let BPV = parameters[16];
            let BPY = parameters[1137];
            let BQB = 1e-1f64;
            let BQE = 5e-2f64;
            let BQG = 2e1f64;
            let BRA = node_potentials[8];
            let BRB = node_potentials[10];
            let BRD = node_potentials[11];
            let BRF = node_potentials[6];
            let BRI = node_potentials[7];
            let BRR = node_potentials[3];
            let BRZ = -1e0f64;
            let BSF = parameters[1146];
            let BSH = 8e1f64;
            let BTV = parameters[74];
            let BTY = parameters[75];
            let BUA = 4e1f64;
            let BVC = 1.804851387e-35f64;
            let BVH = parameters[25];
            let BWC = 7.071067811865475e-1f64;
            let BWE = 1e-7f64;
            let BWG = 1.25e0f64;
            let BWJ = 7.324648775608221e-1f64;
            let BWT = 6.4e1f64;
            let BYJ = parameters[294];
            let BYR = 1.6666666666666666e-1f64;
            let BYV = 1.25e0f64;
            let BZL = 1.2e1f64;
            let CAB = 1e-40f64;
            let CCR = 6.4e-7f64;
            let CCS = 8e-4f64;
            let CEP = 3.7e1f64;
            let CIR = 3.912023005e0f64;
            let CJH = 1e-5f64;
            let CJM = 1e-8f64;
            let CJW = 1e6f64;
            let CKV = parameters[1349];
            let CKW = parameters[1350];
            let CKZ = parameters[1351];
            let CLA = parameters[1352];
            let CVH = 1e-10f64;
            let CXP = 1.25e-1f64;
            let CXX = parameters[46];
            let CYQ = 1e-35f64;
            let CZT = parameters[414];
            let DAK = parameters[433];
            let DAM = 5.540622384e34f64;
            let DCT = 4e-3f64;
            let DDW = parameters[1009];
            let DDX = parameters[1008];
            let DEJ = 1.115e0f64;
            let DFF = parameters[595];
            let DFI = parameters[920];
            let DFO = parameters[554];
            let DFY = parameters[36];
            let DGC = 1e-4f64;
            let DGJ = parameters[44];
            let DGS = parameters[666];
            let DHG = parameters[913];
            let DHK = 3.8025850929940455e0f64;
            let DHM = parameters[915];
            let DHQ = 3.8025850929940455e0f64;
            let DHS = parameters[917];
            let DHW = 3.8025850929940455e0f64;
            let DIA = 9e-1f64;
            let DJP = parameters[919];
            let DKI = parameters[914];
            let DKM = 3.8025850929940455e0f64;
            let DKO = parameters[916];
            let DKS = 3.8025850929940455e0f64;
            let DKU = parameters[918];
            let DKY = 3.8025850929940455e0f64;
            let DNM = 2e-1f64;
            let DNV = parameters[1379];
            let DOJ = if parameter_given[867] { 1.0 } else { 0.0 };
            let DOM = parameters[32];
            let DOO = parameters[1394];
            let DOP = parameters[1393];
            let DOW = 3.453133e-11f64;
            let DOX = parameters[1388];
            let DOY = parameters[1382];
            let DPG = parameters[140];
            let DPS = 1.25e0f64;
            let EQJ = parameters[1380];
            let ERC = parameters[38];
            let ERN = parameters[671];
            let ERW = parameters[696];
            let ESC = parameters[700];
            let ESD = parameters[701];
            let ESO = parameters[697];
            let EST = parameters[698];
            let ESU = parameters[699];
            let ETG = 2e-4f64;
            let ETO = parameters[1295];
            let EVA = parameters[1011];
            let EVQ = parameters[1012];
            let EVR = parameters[1013];
            let EVS = parameters[1014];
            let EVX = parameters[1015];
            let EWD = 1e10f64;
            let EWO = parameters[1016];
            let EWP = parameters[1017];
            let EWU = parameters[1010];
            let EWX = parameters[1019];
            let EWY = parameters[1022];
            let EXA = parameters[1020];
            let EXB = parameters[1023];
            let EXD = parameters[1297];
            let EXE = parameters[1298];
            let EXG = parameters[1021];
            let EXH = parameters[1024];
            let EXJ = parameters[1296];
            let EXQ = parameters[39];
            let EXX = parameters[1018];
            let EYO = parameters[1299];
            let EYR = 6e1f64;
            let EYS = 1.44e2f64;
            let EYT = 1.5e1f64;
            let EYV = 3.95e-1f64;
            let FAJ = parameters[1264];
            let FAM = parameters[1263];
            let FAP = parameters[1262];
            let FBD = parameters[1151];
            let FBI = parameters[1148];
            let FBJ = parameters[1149];
            let FBK = parameters[1150];
            let FBS = 1.25e0f64;
            let GCA = parameters[1147];
            let GCC = 1.4142135623730951e0f64;
            let GCW = 2.01491e-1f64;
            let GCX = 4.02982e-1f64;
            let GCY = 2.446562e0f64;
            let GDB = -1e2f64;
            let GDD = 1.804851387e-35f64;
            let GIU = -1e2f64;
            let GIW = 1.804851387e-35f64;
            let GKE = 8e-1f64;
            let GKF = 1.2e0f64;
            let GTT = 3.8025850929940455e0f64;
            let GTY = 3.8025850929940455e0f64;
            let GUD = 3.8025850929940455e0f64;
            let GWM = 3.8025850929940455e0f64;
            let GWR = 3.8025850929940455e0f64;
            let GWW = 3.8025850929940455e0f64;
            let GZS = parameters[1320];
            let GZY = parameters[1322];
            let HAV = -1e2f64;
            let HAX = 1.804851387e-35f64;
            let HGQ = -1e2f64;
            let HGS = 1.804851387e-35f64;
            let HIF = parameters[1353];
            let HIG = parameters[1354];
            let HII = parameters[1348];
            let HIX = -1e2f64;
            let HIZ = 1.804851387e-35f64;
            let HNC = -1e2f64;
            let HNE = 1.804851387e-35f64;
            let HOW = -1e2f64;
            let HOY = 1.804851387e-35f64;
            let HSY = -1e2f64;
            let HTA = 1.804851387e-35f64;
            let HUG = -1e2f64;
            let HUI = 1.804851387e-35f64;
            let HVL = parameters[1316];
            let HYJ = parameters[1356];
            let HYK = parameters[1360];
            let F = if E == B { 1.0 } else { 0.0 };
            let BCB = if F != 0.0 {
                B
            } else {
                G
            };
            let J = H * I;
            let L = K * I;
            let N = L / M;
            let O = H / K;
            let P = if (if parameter_given[77] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            let DNU = if P != 0.0 {
                let R = ((M * K) / Q) - parameters[78];
                R
            } else {
                S
            };
            let U = parameters[0] * T;
            let X = V * W;
            let Y = U + parameters[51];
            let Z = if Y <= A { 1.0 } else { 0.0 };
            if Z != 0.0 {
            } else {
            }
            let AB = (X / AA) + parameters[53];
            let AC = if AB <= A { 1.0 } else { 0.0 };
            if AC != 0.0 {
            } else {
            }
            let AD = -parameters[58];
            let AE = Y.powf(AD);
            let AF = -parameters[59];
            let AG = AB.powf(AF);
            let AH = AE * AG;
            let AN = -AM;
            let AO = Y.powf(AN);
            let AQ = -AP;
            let AR = AB.powf(AQ);
            let AS = AO * AR;
            let AX = ((AT + (AU * AO)) + (AV * AR)) + (AW * AS);
            let AZ = Y - (AY * (((AI + (AJ * AE)) + (AK * AG)) + (AL * AH)));
            let BA = if AZ <= A { 1.0 } else { 0.0 };
            if BA != 0.0 {
            } else {
                let BC = if AZ <= BB { 1.0 } else { 0.0 };
                if BC != 0.0 {
                } else {
                }
            }
            let BE = AB - (BD * parameters[1376]);
            let BF = AY - BD;
            let BG = BE - (BF * AX);
            let BH = if BG <= A { 1.0 } else { 0.0 };
            if BH != 0.0 {
            } else {
                let BI = if BG <= BB { 1.0 } else { 0.0 };
                if BI != 0.0 {
                } else {
                }
            }
            let BM = ((parameters[70] + (BJ * AO)) + (BK * AR)) + (BL * AS);
            let BN = Y - (AY * (((parameters[66] + (parameters[67] * AE)) + (parameters[68] * AG)) + (parameters[69] * AH)));
            let BO = if BN <= A { 1.0 } else { 0.0 };
            if BO != 0.0 {
            } else {
                let BP = if BN <= BB { 1.0 } else { 0.0 };
                if BP != 0.0 {
                } else {
                }
            }
            let BQ = BE - (BF * BM);
            let BR = if BQ <= A { 1.0 } else { 0.0 };
            if BR != 0.0 {
            } else {
                let BS = if BQ <= BB { 1.0 } else { 0.0 };
                if BS != 0.0 {
                } else {
                }
            }
            let BT = Y.powf(AM);
            let BU = AB.powf(AP);
            let BV = AB - (AY * (((parameters[927] + (BJ / BT)) + (BK / BU)) + ((BL / BT) / BU)));
            let BW = if BV <= A { 1.0 } else { 0.0 };
            if BW != 0.0 {
            } else {
            }
            let BY = BX / AZ;
            let BZ = BX / BG;
            let CA = BX / BN;
            let CB = BX / BQ;
            let CC = BX / parameters[48];
            let CD = BX / parameters[52];
            let CE = BY * BZ;
            let CG = if CF != A { 1.0 } else { 0.0 };
            let CR;
            let CV;
            if CG != 0.0 {
                let CH = if CF <= (-Y) { 1.0 } else { 0.0 };
                let CS;
                let CW;
                if CH != 0.0 {
                    CS = AE;
                    CW = AO;
                } else {
                    let CI = Y + CF;
                    let CJ = CI.powf(AD);
                    let CK = CI.powf(AN);
                    CS = CJ;
                    CW = CK;
                }
                CR = CS;
                CV = CW;
            } else {
                CR = AE;
                CV = AO;
            }
            let CM = if CL != A { 1.0 } else { 0.0 };
            let CT;
            let CX;
            if CM != 0.0 {
                let CN = if CL <= (-AB) { 1.0 } else { 0.0 };
                let CU;
                let CY;
                if CN != 0.0 {
                    CU = AG;
                    CY = AR;
                } else {
                    let CO = AB + CL;
                    let CP = CO.powf(AF);
                    let CQ = CO.powf(AQ);
                    CU = CP;
                    CY = CQ;
                }
                CT = CU;
                CX = CY;
            } else {
                CT = AG;
                CX = AR;
            }
            let CZ = ((AT + (AU * CV)) + (AV * CX)) + (AW * (CV * CX));
            let DA = (Y - (AY * (((AI + (AJ * CR)) + (AK * CT)) + (AL * (CR * CT))))) + CF;
            let DB = if DA <= A { 1.0 } else { 0.0 };
            if DB != 0.0 {
            } else {
            }
            let DC = (AB - (AY * CZ)) + CL;
            let DD = if DC <= A { 1.0 } else { 0.0 };
            if DD != 0.0 {
            } else {
            }
            let DE = if parameters[1025] == B { 1.0 } else { 0.0 };
            let DJ;
            let DK;
            if DE != 0.0 {
                let DF = BX / DA;
                let DG = BX / DC;
                DJ = DF;
                DK = DG;
            } else {
                let DH = B / DA;
                let DI = B / DC;
                DJ = DH;
                DK = DI;
            }
            let DL = DJ * DK;
            let DM = ((parameters[115] + (DJ * parameters[116])) + (DK * parameters[117])) + (DL * parameters[118]);
            let DN = ((parameters[119] + (DJ * parameters[120])) + (DK * parameters[121])) + (DL * parameters[122]);
            let DO = ((parameters[129] + (DJ * parameters[130])) + (DK * parameters[131])) + (DL * parameters[132]);
            let DP = ((parameters[142] + (DJ * parameters[143])) + (DK * parameters[144])) + (DL * parameters[145]);
            let DQ = ((parameters[79] + (DJ * parameters[88])) + (DK * parameters[89])) + (DL * parameters[90]);
            let DR = ((parameters[91] + (DJ * parameters[100])) + (DK * parameters[101])) + (DL * parameters[102]);
            let DS = ((parameters[103] + (DJ * parameters[104])) + (DK * parameters[105])) + (DL * parameters[106]);
            let DT = ((parameters[232] + (DJ * parameters[233])) + (DK * parameters[234])) + (DL * parameters[235]);
            let DU = ((parameters[236] + (DJ * parameters[243])) + (DK * parameters[244])) + (DL * parameters[245]);
            let DV = ((parameters[246] + (parameters[247] * DJ)) + (parameters[248] * DK)) + (parameters[249] * DL);
            let DW = ((parameters[250] + (parameters[251] * DJ)) + (parameters[252] * DK)) + (parameters[253] * DL);
            let DX = ((parameters[170] + (parameters[171] * DJ)) + (parameters[172] * DK)) + (parameters[173] * DL);
            let DY = ((parameters[174] + (parameters[175] * DJ)) + (parameters[176] * DK)) + (parameters[177] * DL);
            let DZ = ((parameters[178] + (parameters[179] * DJ)) + (parameters[180] * DK)) + (parameters[181] * DL);
            let EA = ((parameters[186] + (parameters[187] * DJ)) + (parameters[188] * DK)) + (parameters[189] * DL);
            let EB = ((parameters[182] + (parameters[183] * DJ)) + (parameters[184] * DK)) + (parameters[185] * DL);
            let EC = ((parameters[254] + (parameters[255] * DJ)) + (parameters[256] * DK)) + (parameters[257] * DL);
            let ED = ((parameters[258] + (DJ * parameters[259])) + (DK * parameters[260])) + (DL * parameters[261]);
            let EE = ((parameters[262] + (DJ * parameters[263])) + (DK * parameters[264])) + (DL * parameters[265]);
            let EF = ((parameters[1164] + (DJ * parameters[1165])) + (DK * parameters[1166])) + (DL * parameters[1167]);
            let EG = ((parameters[1191] + (DJ * parameters[1192])) + (DK * parameters[1193])) + (DL * parameters[1194]);
            let EH = ((parameters[288] + (DJ * parameters[291])) + (DK * parameters[292])) + (DL * parameters[293]);
            let EI = ((parameters[270] + (DJ * parameters[271])) + (DK * parameters[272])) + (DL * parameters[273]);
            let EJ = ((parameters[1176] + (DJ * parameters[1177])) + (DK * parameters[1178])) + (DL * parameters[1179]);
            let EK = ((parameters[275] + (DJ * parameters[276])) + (DK * parameters[277])) + (DL * parameters[278]);
            let EL = ((parameters[146] + (DJ * parameters[147])) + (DK * parameters[148])) + (DL * parameters[149]);
            let EM = ((parameters[1238] + (DJ * parameters[1239])) + (DK * parameters[1240])) + (DL * parameters[1241]);
            let EN = ((parameters[150] + (DJ * parameters[151])) + (DK * parameters[152])) + (DL * parameters[153]);
            let EO = ((parameters[1242] + (DJ * parameters[1243])) + (DK * parameters[1244])) + (DL * parameters[1245]);
            let EP = ((parameters[154] + (DJ * parameters[155])) + (DK * parameters[156])) + (DL * parameters[157]);
            let EQ = ((parameters[158] + (DJ * parameters[159])) + (DK * parameters[160])) + (DL * parameters[161]);
            let ER = ((parameters[162] + (DJ * parameters[163])) + (DK * parameters[164])) + (DL * parameters[165]);
            let ES = ((parameters[166] + (DJ * parameters[167])) + (DK * parameters[168])) + (DL * parameters[169]);
            let ET = ((parameters[1246] + (DJ * parameters[1247])) + (DK * parameters[1248])) + (DL * parameters[1249]);
            let EU = ((parameters[1250] + (DJ * parameters[1251])) + (DK * parameters[1252])) + (DL * parameters[1253]);
            let EV = ((parameters[1254] + (DJ * parameters[1255])) + (DK * parameters[1256])) + (DL * parameters[1257]);
            let EW = ((parameters[1258] + (DJ * parameters[1259])) + (DK * parameters[1260])) + (DL * parameters[1261]);
            let EX = ((parameters[218] + (DJ * parameters[225])) + (DK * parameters[226])) + (DL * parameters[227]);
            let EY = ((parameters[208] + (DJ * parameters[215])) + (DK * parameters[216])) + (DL * parameters[217]);
            let EZ = ((parameters[1196] + (DJ * parameters[1203])) + (DK * parameters[1204])) + (DL * parameters[1205]);
            let FA = ((parameters[111] + (DJ * parameters[112])) + (DK * parameters[113])) + (DL * parameters[114]);
            let FB = ((parameters[190] + (DJ * parameters[191])) + (DK * parameters[192])) + (DL * parameters[193]);
            let FC = ((parameters[194] + (DJ * parameters[195])) + (DK * parameters[196])) + (DL * parameters[197]);
            let FD = ((parameters[203] + (DJ * parameters[205])) + (DK * parameters[206])) + (DL * parameters[207]);
            let FE = ((parameters[309] + (DJ * parameters[310])) + (DK * parameters[311])) + (DL * parameters[312]);
            let FF = ((parameters[337] + (DJ * parameters[340])) + (DK * parameters[341])) + (DL * parameters[342]);
            let FG = ((parameters[348] + (DJ * parameters[355])) + (DK * parameters[356])) + (DL * parameters[357]);
            let FH = ((parameters[372] + (DJ * parameters[375])) + (DK * parameters[376])) + (DL * parameters[377]);
            let FI = ((parameters[362] + (DJ * parameters[363])) + (DK * parameters[364])) + (DL * parameters[365]);
            let FJ = ((parameters[382] + (DJ * parameters[383])) + (DK * parameters[384])) + (DL * parameters[385]);
            let FK = ((parameters[390] + (DJ * parameters[397])) + (DK * parameters[398])) + (DL * parameters[399]);
            let FL = ((parameters[404] + (DJ * parameters[407])) + (DK * parameters[408])) + (DL * parameters[409]);
            let FM = ((parameters[415] + (DJ * parameters[418])) + (DK * parameters[419])) + (DL * parameters[420]);
            let FN = ((parameters[457] + (DJ * parameters[458])) + (DK * parameters[459])) + (DL * parameters[460]);
            let FO = ((parameters[467] + (DJ * parameters[468])) + (DK * parameters[469])) + (DL * parameters[470]);
            let FP = ((parameters[439] + (DJ * parameters[440])) + (DK * parameters[441])) + (DL * parameters[442]);
            let FQ = ((parameters[443] + (DJ * parameters[444])) + (DK * parameters[445])) + (DL * parameters[446]);
            let FR = ((parameters[449] + (DJ * parameters[450])) + (DK * parameters[451])) + (DL * parameters[452]);
            let FS = ((parameters[453] + (DJ * parameters[454])) + (DK * parameters[455])) + (DL * parameters[456]);
            let FT = ((parameters[463] + (DJ * parameters[464])) + (DK * parameters[465])) + (DL * parameters[466]);
            let FU = ((parameters[477] + (DJ * parameters[480])) + (DK * parameters[481])) + (DL * parameters[482]);
            let FV = ((parameters[473] + (DJ * parameters[474])) + (DK * parameters[475])) + (DL * parameters[476]);
            let FW = ((parameters[498] + (DJ * parameters[499])) + (DK * parameters[500])) + (DL * parameters[501]);
            let FX = ((parameters[530] + (DJ * parameters[533])) + (DK * parameters[534])) + (DL * parameters[535]);
            let FY = ((parameters[540] + (DJ * parameters[541])) + (DK * parameters[542])) + (DL * parameters[543]);
            let FZ = ((parameters[421] + (DJ * parameters[422])) + (DK * parameters[423])) + (DL * parameters[424]);
            let GA = ((parameters[425] + (DJ * parameters[426])) + (DK * parameters[427])) + (DL * parameters[428]);
            let GB = ((parameters[429] + (DJ * parameters[430])) + (DK * parameters[431])) + (DL * parameters[432]);
            let GC = ((parameters[434] + (DJ * parameters[435])) + (DK * parameters[436])) + (DL * parameters[437]);
            let GD = ((parameters[548] + (DJ * parameters[551])) + (DK * parameters[552])) + (DL * parameters[553]);
            let GE = ((parameters[544] + (DJ * parameters[545])) + (DK * parameters[546])) + (DL * parameters[547]);
            let GF = ((parameters[295] + (DJ * parameters[296])) + (DK * parameters[297])) + (DL * parameters[298]);
            let GG = ((parameters[510] + (DJ * parameters[511])) + (DK * parameters[512])) + (DL * parameters[513]);
            let GH = ((parameters[325] + (DJ * parameters[326])) + (DK * parameters[327])) + (DL * parameters[328]);
            let GI = ((parameters[329] + (parameters[330] * DJ)) + (parameters[331] * DK)) + (parameters[332] * DL);
            let GJ = ((parameters[483] + (DJ * parameters[484])) + (DK * parameters[485])) + (DL * parameters[486]);
            let GK = ((parameters[315] + (DJ * parameters[316])) + (DK * parameters[317])) + (DL * parameters[318]);
            let GL = ((parameters[883] + (DJ * parameters[884])) + (DK * parameters[885])) + (DL * parameters[886]);
            let GM = ((parameters[887] + (DJ * parameters[888])) + (DK * parameters[889])) + (DL * parameters[890]);
            let GN = ((parameters[601] + (DJ * parameters[604])) + (DK * parameters[605])) + (DL * parameters[606]);
            let GO = ((parameters[607] + (DJ * parameters[608])) + (DK * parameters[609])) + (DL * parameters[610]);
            let GP = ((parameters[611] + (DJ * parameters[612])) + (DK * parameters[613])) + (DL * parameters[614]);
            let GQ = ((parameters[615] + (DJ * parameters[616])) + (DK * parameters[617])) + (DL * parameters[618]);
            let GR = ((parameters[662] + (DJ * parameters[663])) + (DK * parameters[664])) + (DL * parameters[665]);
            let GS = ((parameters[1361] + (DJ * parameters[1362])) + (DK * parameters[1363])) + (DL * parameters[1364]);
            let GT = ((parameters[1365] + (DJ * parameters[1366])) + (DK * parameters[1367])) + (DL * parameters[1368]);
            let GU = ((parameters[1369] + (DJ * parameters[1370])) + (DK * parameters[1371])) + (DL * parameters[1372]);
            let GV = ((parameters[932] + (parameters[934] * DJ)) + (parameters[936] * DK)) + (parameters[938] * DL);
            let GW = ((parameters[933] + (parameters[935] * DJ)) + (parameters[937] * DK)) + (parameters[939] * DL);
            let GX = ((parameters[940] + (parameters[941] * DJ)) + (parameters[942] * DK)) + (parameters[943] * DL);
            let GY = ((parameters[944] + (parameters[945] * DJ)) + (parameters[946] * DK)) + (parameters[947] * DL);
            let GZ = ((parameters[952] + (parameters[954] * DJ)) + (parameters[956] * DK)) + (parameters[958] * DL);
            let HA = ((parameters[953] + (parameters[955] * DJ)) + (parameters[957] * DK)) + (parameters[959] * DL);
            let HB = ((parameters[968] + (parameters[970] * DJ)) + (parameters[972] * DK)) + (parameters[974] * DL);
            let HC = ((parameters[969] + (parameters[971] * DJ)) + (parameters[973] * DK)) + (parameters[975] * DL);
            let HD = ((parameters[992] + (parameters[994] * DJ)) + (parameters[996] * DK)) + (parameters[998] * DL);
            let HE = ((parameters[993] + (parameters[995] * DJ)) + (parameters[997] * DK)) + (parameters[999] * DL);
            let HF = ((parameters[1000] + (parameters[1002] * DJ)) + (parameters[1004] * DK)) + (parameters[1006] * DL);
            let HG = ((parameters[1001] + (parameters[1003] * DJ)) + (parameters[1005] * DK)) + (parameters[1007] * DL);
            let HH = ((parameters[555] + (parameters[556] * DJ)) + (parameters[557] * DK)) + (parameters[558] * DL);
            let HI = ((parameters[559] + (parameters[560] * DJ)) + (parameters[561] * DK)) + (parameters[562] * DL);
            let HJ = ((parameters[563] + (DJ * parameters[565])) + (DK * parameters[567])) + (parameters[569] * DL);
            let HK = ((parameters[564] + (DJ * parameters[566])) + (DK * parameters[568])) + (parameters[570] * DL);
            let HL = ((parameters[571] + (parameters[572] * DJ)) + (parameters[573] * DK)) + (parameters[574] * DL);
            let HM = ((parameters[575] + (parameters[576] * DJ)) + (parameters[577] * DK)) + (parameters[578] * DL);
            let HN = ((parameters[579] + (parameters[582] * DJ)) + (parameters[581] * DK)) + (parameters[580] * DL);
            let HO = ((parameters[583] + (parameters[584] * DJ)) + (parameters[585] * DK)) + (parameters[586] * DL);
            let HP = ((parameters[594] + (parameters[589] * DJ)) + (parameters[591] * DK)) + (parameters[593] * DL);
            let HQ = ((parameters[921] + (parameters[922] * DJ)) + (parameters[923] * DK)) + (parameters[924] * DL);
            let HR = ((parameters[1125] + (DJ * parameters[1126])) + (DK * parameters[1127])) + (DL * parameters[1128]);
            let HS = ((parameters[1129] + (DJ * parameters[1130])) + (DK * parameters[1131])) + (DL * parameters[1132]);
            let HT = ((parameters[1133] + (DJ * parameters[1134])) + (DK * parameters[1135])) + (DL * parameters[1136]);
            let HU = ((parameters[799] + (DJ * parameters[802])) + (DK * parameters[803])) + (DL * parameters[804]);
            let HV = ((parameters[805] + (DJ * parameters[807])) + (DK * parameters[808])) + (DL * parameters[809]);
            let HW = ((parameters[806] + (parameters[810] * DJ)) + (parameters[811] * DK)) + (parameters[812] * DL);
            let HX = ((parameters[813] + (DJ * parameters[814])) + (DK * parameters[815])) + (DL * parameters[816]);
            let HY = ((parameters[821] + (DJ * parameters[824])) + (DK * parameters[825])) + (DL * parameters[826]);
            let HZ = ((parameters[827] + (DJ * parameters[829])) + (DK * parameters[830])) + (DL * parameters[831]);
            let IA = ((parameters[828] + (parameters[832] * DJ)) + (parameters[833] * DK)) + (parameters[834] * DL);
            let IB = ((parameters[835] + (DJ * parameters[836])) + (DK * parameters[837])) + (DL * parameters[838]);
            let IC = ((parameters[859] + (DJ * parameters[860])) + (DK * parameters[861])) + (DL * parameters[862]);
            let ID = ((parameters[847] + (DJ * parameters[848])) + (DK * parameters[849])) + (DL * parameters[850]);
            let IE = ((parameters[1032] + (DJ * parameters[1033])) + (DK * parameters[1034])) + (DL * parameters[1035]);
            let IF = ((parameters[1037] + (DJ * parameters[1038])) + (DK * parameters[1039])) + (DL * parameters[1040]);
            let IG = ((parameters[1042] + (DJ * parameters[1043])) + (DK * parameters[1044])) + (DL * parameters[1045]);
            let IH = ((parameters[1046] + (DJ * parameters[1047])) + (DK * parameters[1048])) + (DL * parameters[1049]);
            let II = ((parameters[1051] + (DJ * parameters[1052])) + (DK * parameters[1053])) + (DL * parameters[1054]);
            let IJ = ((parameters[1055] + (DJ * parameters[1056])) + (DK * parameters[1057])) + (DL * parameters[1058]);
            let IK = ((parameters[1060] + (DJ * parameters[1061])) + (DK * parameters[1062])) + (DL * parameters[1063]);
            let IL = ((parameters[1064] + (DJ * parameters[1065])) + (DK * parameters[1066])) + (DL * parameters[1067]);
            let IM = ((parameters[1070] + (DJ * parameters[1071])) + (DK * parameters[1072])) + (DL * parameters[1073]);
            let IN = ((parameters[1085] + (DJ * parameters[1086])) + (DK * parameters[1087])) + (DL * parameters[1088]);
            let IO = ((parameters[1089] + (DJ * parameters[1090])) + (DK * parameters[1091])) + (DL * parameters[1092]);
            let IP = ((parameters[706] + (DJ * parameters[732])) + (DK * parameters[733])) + (DL * parameters[734]);
            let IQ = ((parameters[684] + (DJ * parameters[685])) + (DK * parameters[686])) + (DL * parameters[687]);
            let IR = ((parameters[688] + (parameters[689] * DJ)) + (parameters[690] * DK)) + (parameters[691] * DL);
            let IS = ((parameters[692] + (DJ * parameters[693])) + (DK * parameters[694])) + (DL * parameters[695]);
            let IT = ((parameters[672] + (DJ * parameters[673])) + (DK * parameters[674])) + (DL * parameters[675]);
            let IU = ((parameters[676] + (parameters[677] * DJ)) + (parameters[678] * DK)) + (parameters[679] * DL);
            let IV = ((parameters[680] + (DJ * parameters[681])) + (DK * parameters[682])) + (DL * parameters[683]);
            let IW = ((parameters[707] + (DJ * parameters[735])) + (DK * parameters[737])) + (DL * parameters[739]);
            let IX = ((parameters[726] + (parameters[736] * DJ)) + (parameters[738] * DK)) + (parameters[740] * DL);
            let IY = ((parameters[708] + (DJ * parameters[741])) + (DK * parameters[742])) + (DL * parameters[743]);
            let IZ = ((parameters[709] + (DJ * parameters[744])) + (DK * parameters[745])) + (DL * parameters[746]);
            let JA = ((parameters[710] + (DJ * parameters[747])) + (DK * parameters[749])) + (DL * parameters[751]);
            let JB = ((parameters[711] + (parameters[748] * DJ)) + (parameters[750] * DK)) + (parameters[752] * DL);
            let JC = ((parameters[712] + (DJ * parameters[753])) + (DK * parameters[754])) + (DL * parameters[755]);
            let JD = ((parameters[713] + (DJ * parameters[756])) + (DK * parameters[757])) + (DL * parameters[758]);
            let JE = ((parameters[714] + (DJ * parameters[759])) + (DK * parameters[761])) + (DL * parameters[763]);
            let JF = ((parameters[715] + (parameters[760] * DJ)) + (parameters[762] * DK)) + (parameters[764] * DL);
            let JG = ((parameters[716] + (DJ * parameters[765])) + (DK * parameters[766])) + (DL * parameters[767]);
            let JH = ((parameters[717] + (DJ * parameters[768])) + (DK * parameters[769])) + (DL * parameters[770]);
            let JI = ((parameters[720] + (DJ * parameters[771])) + (DK * parameters[772])) + (DL * parameters[773]);
            let JJ = ((parameters[718] + (DJ * parameters[774])) + (DK * parameters[775])) + (DL * parameters[776]);
            let JK = ((parameters[719] + (DJ * parameters[777])) + (DK * parameters[778])) + (DL * parameters[779]);
            let JL = ((parameters[721] + (DJ * parameters[780])) + (DK * parameters[781])) + (DL * parameters[782]);
            let JM = ((parameters[1075] + (DJ * parameters[1078])) + (DK * parameters[1079])) + (DL * parameters[1080]);
            let JN = ((parameters[1081] + (DJ * parameters[1082])) + (DK * parameters[1083])) + (DL * parameters[1084]);
            let JO = ((parameters[489] + (DJ * parameters[494])) + (DK * parameters[495])) + (DL * parameters[496]);
            let JP = ((parameters[514] + (DJ * parameters[515])) + (DK * parameters[516])) + (DL * parameters[517]);
            let JQ = ((parameters[518] + (DJ * parameters[519])) + (DK * parameters[520])) + (DL * parameters[521]);
            let JR = ((parameters[522] + (DJ * parameters[523])) + (DK * parameters[524])) + (DL * parameters[525]);
            let JS = ((parameters[526] + (DJ * parameters[527])) + (DK * parameters[528])) + (DL * parameters[529]);
            let JT = ((parameters[1300] + (DJ * parameters[1301])) + (DK * parameters[1302])) + (DL * parameters[1303]);
            let JU = ((parameters[1308] + (DJ * parameters[1309])) + (DK * parameters[1310])) + (DL * parameters[1311]);
            let JV = ((parameters[1304] + (DJ * parameters[1305])) + (DK * parameters[1306])) + (DL * parameters[1307]);
            let JW = ((parameters[1312] + (DJ * parameters[1313])) + (DK * parameters[1314])) + (DL * parameters[1315]);
            let JX = ((parameters[1156] + (DJ * parameters[1157])) + (DK * parameters[1158])) + (DL * parameters[1159]);
            let JY = ((parameters[1152] + (DJ * parameters[1153])) + (DK * parameters[1154])) + (DL * parameters[1155]);
            let JZ = ((parameters[1160] + (DJ * parameters[1161])) + (DK * parameters[1162])) + (DL * parameters[1163]);
            let KA = ((parameters[1168] + (DJ * parameters[1169])) + (DK * parameters[1170])) + (DL * parameters[1171]);
            let KB = ((parameters[1186] + (DJ * parameters[1187])) + (DK * parameters[1188])) + (DL * parameters[1189]);
            let KC = ((parameters[1206] + (DJ * parameters[1207])) + (DK * parameters[1208])) + (DL * parameters[1209]);
            let KD = ((parameters[1210] + (DJ * parameters[1211])) + (DK * parameters[1212])) + (DL * parameters[1213]);
            let KE = ((parameters[1214] + (DJ * parameters[1215])) + (DK * parameters[1216])) + (DL * parameters[1217]);
            let KF = ((parameters[1218] + (DJ * parameters[1219])) + (DK * parameters[1220])) + (DL * parameters[1221]);
            let KG = ((parameters[1222] + (DJ * parameters[1223])) + (DK * parameters[1224])) + (DL * parameters[1225]);
            let KH = ((parameters[1226] + (DJ * parameters[1227])) + (DK * parameters[1228])) + (DL * parameters[1229]);
            let KI = ((parameters[1230] + (DJ * parameters[1231])) + (DK * parameters[1232])) + (DL * parameters[1233]);
            let KJ = ((parameters[1234] + (DJ * parameters[1235])) + (DK * parameters[1236])) + (DL * parameters[1237]);
            let KK = ((parameters[1265] + (DJ * parameters[1272])) + (DK * parameters[1273])) + (DL * parameters[1274]);
            let KL = ((parameters[1275] + (DJ * parameters[1276])) + (DK * parameters[1277])) + (DL * parameters[1278]);
            let KM = ((parameters[1283] + (DJ * parameters[1284])) + (DK * parameters[1285])) + (DL * parameters[1286]);
            let KN = ((parameters[1279] + (DJ * parameters[1280])) + (DK * parameters[1281])) + (DL * parameters[1282]);
            let KO = ((parameters[1287] + (DJ * parameters[1288])) + (DK * parameters[1289])) + (DL * parameters[1290]);
            let KP = ((parameters[1291] + (DJ * parameters[1292])) + (DK * parameters[1293])) + (DL * parameters[1294]);
            let KQ = ((parameters[1323] + (DJ * parameters[1324])) + (DK * parameters[1325])) + (DL * parameters[1326]);
            let KR = ((parameters[1327] + (DJ * parameters[1328])) + (DK * parameters[1329])) + (DL * parameters[1330]);
            let KS = ((parameters[1331] + (DJ * parameters[1332])) + (DK * parameters[1333])) + (DL * parameters[1334]);
            let KT = ((parameters[1335] + (DJ * parameters[1336])) + (DK * parameters[1337])) + (DL * parameters[1338]);
            let KU = ((parameters[1339] + (DJ * parameters[1340])) + (DK * parameters[1341])) + (DL * parameters[1342]);
            let KV = ((parameters[1343] + (DJ * parameters[1344])) + (DK * parameters[1345])) + (DL * parameters[1346]);
            let KW = ((parameters[1384] + (DJ * parameters[1385])) + (DK * parameters[1386])) + (DL * parameters[1387]);
            let KX = ((parameters[1389] + (DJ * parameters[1390])) + (DK * parameters[1391])) + (DL * parameters[1392]);
            let KY = if parameters[35] != A { 1.0 } else { 0.0 };
            let LW;
            let LY;
            let MJ;
            let MW;
            let NE;
            let NK;
            let NP;
            let NW;
            let OG;
            let OM;
            let OS;
            let OX;
            let RB;
            if KY != 0.0 {
                let KZ = ((parameters[1172] + (DJ * parameters[1173])) + (DK * parameters[1174])) + (DL * parameters[1175]);
                let LA = ((parameters[284] + (DJ * parameters[285])) + (DK * parameters[286])) + (DL * parameters[287]);
                let LB = ((parameters[198] + (DJ * parameters[199])) + (DK * parameters[200])) + (DL * parameters[201]);
                let LC = ((parameters[343] + (DJ * parameters[344])) + (DK * parameters[345])) + (DL * parameters[346]);
                let LD = ((parameters[358] + (DJ * parameters[359])) + (DK * parameters[360])) + (DL * parameters[361]);
                let LE = ((parameters[378] + (DJ * parameters[379])) + (DK * parameters[380])) + (DL * parameters[381]);
                let LF = ((parameters[386] + (DJ * parameters[387])) + (DK * parameters[388])) + (DL * parameters[389]);
                let LG = ((parameters[400] + (DJ * parameters[401])) + (DK * parameters[402])) + (DL * parameters[403]);
                let LH = ((parameters[410] + (DJ * parameters[411])) + (DK * parameters[412])) + (DL * parameters[413]);
                let LI = ((parameters[536] + (DJ * parameters[537])) + (DK * parameters[538])) + (DL * parameters[539]);
                let LJ = ((parameters[305] + (DJ * parameters[306])) + (DK * parameters[307])) + (DL * parameters[308]);
                let LK = ((parameters[490] + (DJ * parameters[491])) + (DK * parameters[492])) + (DL * parameters[493]);
                let LL = ((parameters[506] + (DJ * parameters[507])) + (DK * parameters[508])) + (DL * parameters[509]);
                LW = KZ;
                LY = LA;
                MJ = LC;
                MW = LD;
                NE = LE;
                NK = LG;
                NP = LB;
                NW = LI;
                OG = LH;
                OM = LJ;
                OS = LK;
                OX = LL;
                RB = LF;
            } else {
                LW = A;
                LY = A;
                MJ = A;
                MW = A;
                NE = A;
                NK = A;
                NP = A;
                NW = A;
                OG = A;
                OM = A;
                OS = A;
                OX = A;
                RB = A;
            }
            let LP = DQ * ((B + ((parameters[80] * (if ((BY.powf(LM)) - (CC.powf(LM))) >= A { ((BY.powf(LM)) - (CC.powf(LM))) } else { A })) + (parameters[82] * (if ((BY.powf(LN)) - (CC.powf(LN))) >= A { ((BY.powf(LN)) - (CC.powf(LN))) } else { A })))) + ((parameters[84] * (if ((BZ.powf(LO)) - (CD.powf(LO))) >= A { ((BZ.powf(LO)) - (CD.powf(LO))) } else { A })) + (parameters[86] * (CE.powf(parameters[87])))));
            let LS = DU * ((B + (parameters[237] * (if ((BY.powf(LQ)) - (CC.powf(LQ))) >= A { ((BY.powf(LQ)) - (CC.powf(LQ))) } else { A }))) + ((parameters[239] * (if ((BZ.powf(LR)) - (CD.powf(LR))) >= A { ((BZ.powf(LR)) - (CD.powf(LR))) } else { A })) + (parameters[241] * (CE.powf(parameters[242])))));
            let LU = B + (parameters[282] * (if ((BY.powf(LT)) - (CC.powf(LT))) >= A { ((BY.powf(LT)) - (CC.powf(LT))) } else { A }));
            let LV = ED * LU;
            let RZ;
            let SB;
            if KY != 0.0 {
                let LX = LW * LU;
                let LZ = LY * LU;
                RZ = LZ;
                SB = LX;
            } else {
                RZ = LY;
                SB = LW;
            }
            let MB = EH * (B + (parameters[289] * (if ((BY.powf(MA)) - (CC.powf(MA))) >= A { ((BY.powf(MA)) - (CC.powf(MA))) } else { A })));
            let MC = parameters[24] * FF;
            let MD = if parameters[42] != B { 1.0 } else { 0.0 };
            let SG;
            let BEG;
            if MD != 0.0 {
                let MF = if ME > A { 1.0 } else { 0.0 };
                let SH;
                let BEH;
                if MF != 0.0 {
                    let MH = B - (MG * (if ((BY.powf(ME)) - (CC.powf(ME))) >= A { ((BY.powf(ME)) - (CC.powf(ME))) } else { A }));
                    let MI = MC * MH;
                    let BEI = if KY != 0.0 {
                        let MK = MJ * MH;
                        MK
                    } else {
                        MJ
                    };
                    SH = MI;
                    BEH = BEI;
                } else {
                    let ML = B - MG;
                    let MM = MC * ML;
                    let BEJ = if KY != 0.0 {
                        let MN = MJ * ML;
                        MN
                    } else {
                        MJ
                    };
                    SH = MM;
                    BEH = BEJ;
                }
                SG = SH;
                BEG = BEH;
            } else {
                let MO = -AZ;
                let MP = (B - (parameters[333] * (rspice_limited_exp((MO / parameters[334]))))) - (parameters[335] * (rspice_limited_exp((MO / parameters[336]))));
                let MQ = MC * MP;
                let BEK = if KY != 0.0 {
                    let MR = MJ * MP;
                    MR
                } else {
                    MJ
                };
                SG = MQ;
                BEG = BEK;
            }
            let MU = (B + (parameters[349] * (if ((BY.powf(MS)) - (CC.powf(MS))) >= A { ((BY.powf(MS)) - (CC.powf(MS))) } else { A }))) + ((parameters[351] * (if ((BZ.powf(MT)) - (CD.powf(MT))) >= A { ((BZ.powf(MT)) - (CD.powf(MT))) } else { A })) + (parameters[353] * (CE.powf(parameters[354]))));
            let MV = FG * MU;
            let BEM = if KY != 0.0 {
                let MX = MW * MU;
                MX
            } else {
                MW
            };
            let NA = FI * ((B + (parameters[366] * (if ((BY.powf(MY)) - (CC.powf(MY))) >= A { ((BY.powf(MY)) - (CC.powf(MY))) } else { A }))) + ((parameters[368] * (if ((BZ.powf(MZ)) - (CD.powf(MZ))) >= A { ((BZ.powf(MZ)) - (CD.powf(MZ))) } else { A })) + (parameters[370] * (CE.powf(parameters[371])))));
            let NC = B + (parameters[373] * (if ((BY.powf(NB)) - (CC.powf(NB))) >= A { ((BY.powf(NB)) - (CC.powf(NB))) } else { A }));
            let ND = FH * NC;
            let BEQ = if KY != 0.0 {
                let NF = NE * NC;
                NF
            } else {
                NE
            };
            let NI = (B + (parameters[391] * (if ((BY.powf(NG)) - (CC.powf(NG))) >= A { ((BY.powf(NG)) - (CC.powf(NG))) } else { A }))) + ((parameters[393] * (if ((BZ.powf(NH)) - (CD.powf(NH))) >= A { ((BZ.powf(NH)) - (CD.powf(NH))) } else { A })) + (parameters[395] * (CE.powf(parameters[396]))));
            let NJ = FK * NI;
            let BEO = if KY != 0.0 {
                let NL = NK * NI;
                NL
            } else {
                NK
            };
            let NN = if ((BY.powf(NM)) - (CC.powf(NM))) >= A { ((BY.powf(NM)) - (CC.powf(NM))) } else { A };
            let NO = FC * NN;
            let BDF = if KY != 0.0 {
                let NQ = NP * NN;
                NQ
            } else {
                NP
            };
            let NS = FD * (if ((BY.powf(NR)) - (CC.powf(NR))) >= A { ((BY.powf(NR)) - (CC.powf(NR))) } else { A });
            let NU = B + (parameters[531] * (if ((BY.powf(NT)) - (CC.powf(NT))) >= A { ((BY.powf(NT)) - (CC.powf(NT))) } else { A }));
            let NV = FX * NU;
            let BSZ = if KY != 0.0 {
                let NX = NW * NU;
                NX
            } else {
                NW
            };
            let OA = if (FE * (B + (parameters[313] * (if ((BY.powf(NY)) - (CC.powf(NY))) >= A { ((BY.powf(NY)) - (CC.powf(NY))) } else { A })))) <= NZ { (FE * (B + (parameters[313] * (if ((BY.powf(NY)) - (CC.powf(NY))) >= A { ((BY.powf(NY)) - (CC.powf(NY))) } else { A })))) } else { NZ };
            let OC = GD * (B + (parameters[549] * (if ((BY.powf(OB)) - (CC.powf(OB))) >= A { ((BY.powf(OB)) - (CC.powf(OB))) } else { A })));
            let OE = B + (parameters[405] * (if ((BY.powf(OD)) - (CC.powf(OD))) >= A { ((BY.powf(OD)) - (CC.powf(OD))) } else { A }));
            let OF = if (FL * OE) >= A { (FL * OE) } else { A };
            let BTB = if KY != 0.0 {
                let OH = if (OG * OE) >= A { (OG * OE) } else { A };
                OH
            } else {
                OG
            };
            let OK = (B + (parameters[299] * (if ((BY.powf(OI)) - (CC.powf(OI))) >= A { ((BY.powf(OI)) - (CC.powf(OI))) } else { A }))) + ((parameters[301] * (if ((BZ.powf(OJ)) - (CD.powf(OJ))) >= A { ((BZ.powf(OJ)) - (CD.powf(OJ))) } else { A })) + (parameters[303] * (CE.powf(parameters[304]))));
            let OL = GF * OK;
            let BFB = if KY != 0.0 {
                let ON = OM * OK;
                ON
            } else {
                OM
            };
            let OP = B + (parameters[487] * (if ((BY.powf(OO)) - (CC.powf(OO))) >= A { ((BY.powf(OO)) - (CC.powf(OO))) } else { A }));
            let OR = if (GJ * OP) >= OQ { (GJ * OP) } else { OQ };
            let BTD = if KY != 0.0 {
                let OT = if (OS * OP) >= OQ { (OS * OP) } else { OQ };
                OT
            } else {
                OS
            };
            let OV = B + (parameters[502] * (if ((BY.powf(OU)) - (CC.powf(OU))) >= A { ((BY.powf(OU)) - (CC.powf(OU))) } else { A }));
            let OW = FW * OV;
            let BFL = if KY != 0.0 {
                let OY = OX * OV;
                OY
            } else {
                OX
            };
            let PA = GN * (B + (parameters[602] * (if ((BY.powf(OZ)) - (CC.powf(OZ))) >= A { ((BY.powf(OZ)) - (CC.powf(OZ))) } else { A })));
            let PB = HU * ((B + (parameters[800] * BY)) + (parameters[801] * BZ));
            let PC = HY * ((B + (parameters[822] * BY)) + (parameters[823] * BZ));
            let PD = IW * ((B + (parameters[724] * BY)) + (parameters[725] * BZ));
            let PE = JA * ((B + (parameters[727] * BY)) + (parameters[728] * BZ));
            let PF = JE * ((B + (parameters[729] * BY)) + (parameters[730] * BZ));
            let PG = parameters[723] * (B + (parameters[731] * BY));
            let PK = CB * CA;
            let PL = DR * ((B + ((parameters[92] * (if ((CA.powf(PH)) - (CC.powf(PH))) >= A { ((CA.powf(PH)) - (CC.powf(PH))) } else { A })) + (parameters[94] * (if ((CA.powf(PI)) - (CC.powf(PI))) >= A { ((CA.powf(PI)) - (CC.powf(PI))) } else { A })))) + ((parameters[96] * (if ((CB.powf(PJ)) - (CD.powf(PJ))) >= A { ((CB.powf(PJ)) - (CD.powf(PJ))) } else { A })) + (parameters[98] * (PK.powf(parameters[99])))));
            let PM = if parameters[29] == B { 1.0 } else { 0.0 };
            let RP = if PM != 0.0 {
                LP
            } else {
                PL
            };
            let PP = DM * ((B + (parameters[123] * (if ((BY.powf(PN)) - (CC.powf(PN))) >= A { ((BY.powf(PN)) - (CC.powf(PN))) } else { A }))) + ((parameters[125] * (if ((BZ.powf(PO)) - (CD.powf(PO))) >= A { ((BZ.powf(PO)) - (CD.powf(PO))) } else { A })) + (parameters[127] * (CE.powf(parameters[128])))));
            let PS = DO * ((B + (parameters[133] * (if ((CA.powf(PQ)) - (CC.powf(PQ))) >= A { ((CA.powf(PQ)) - (CC.powf(PQ))) } else { A }))) + ((parameters[135] * (if ((CB.powf(PR)) - (CD.powf(PR))) >= A { ((CB.powf(PR)) - (CD.powf(PR))) } else { A })) + (parameters[137] * (PK.powf(parameters[138])))));
            let PV = GK * ((B + (parameters[319] * (if ((CA.powf(PT)) - (CC.powf(PT))) >= A { ((CA.powf(PT)) - (CC.powf(PT))) } else { A }))) + ((parameters[321] * (if ((CB.powf(PU)) - (CD.powf(PU))) >= A { ((CB.powf(PU)) - (CD.powf(PU))) } else { A })) + (parameters[323] * (PK.powf(parameters[324])))));
            let PX = if (FM * (B + (parameters[416] * (if ((CA.powf(PW)) - (CC.powf(PW))) >= A { ((CA.powf(PW)) - (CC.powf(PW))) } else { A })))) >= A { (FM * (B + (parameters[416] * (if ((CA.powf(PW)) - (CC.powf(PW))) >= A { ((CA.powf(PW)) - (CC.powf(PW))) } else { A })))) } else { A };
            let QA = EY * ((B + (parameters[209] * (if ((BY.powf(PY)) - (CC.powf(PY))) >= A { ((BY.powf(PY)) - (CC.powf(PY))) } else { A }))) + ((parameters[211] * (if ((BZ.powf(PZ)) - (CD.powf(PZ))) >= A { ((BZ.powf(PZ)) - (CD.powf(PZ))) } else { A })) + (parameters[213] * (CE.powf(parameters[214])))));
            let QD = EZ * ((B + (parameters[1197] * (if ((BY.powf(QB)) - (CC.powf(QB))) >= A { ((BY.powf(QB)) - (CC.powf(QB))) } else { A }))) + ((parameters[1199] * (if ((BZ.powf(QC)) - (CD.powf(QC))) >= A { ((BZ.powf(QC)) - (CD.powf(QC))) } else { A })) + (parameters[1201] * (CE.powf(parameters[1202])))));
            let QG = EX * ((B + (parameters[219] * (if ((BY.powf(QE)) - (CC.powf(QE))) >= A { ((BY.powf(QE)) - (CC.powf(QE))) } else { A }))) + ((parameters[221] * (if ((BZ.powf(QF)) - (CD.powf(QF))) >= A { ((BZ.powf(QF)) - (CD.powf(QF))) } else { A })) + (parameters[223] * (CE.powf(parameters[224])))));
            let QJ = KK * ((B + (parameters[1266] * (if ((BY.powf(QH)) - (CC.powf(QH))) >= A { ((BY.powf(QH)) - (CC.powf(QH))) } else { A }))) + ((parameters[1268] * (if ((BZ.powf(QI)) - (CD.powf(QI))) >= A { ((BZ.powf(QI)) - (CD.powf(QI))) } else { A })) + (parameters[1270] * (CE.powf(parameters[1271])))));
            let QL = FQ * (B + (parameters[447] * (if ((BY.powf(QK)) - (CC.powf(QK))) >= A { ((BY.powf(QK)) - (CC.powf(QK))) } else { A })));
            let QM = IE * (B + (BY * parameters[1036]));
            let QN = IF * (B + (BY * parameters[1041]));
            let QO = IH * (B + (BY * parameters[1050]));
            let QP = IL * (B + (BY * parameters[1068]));
            let QQ = IM * (B + (BY * parameters[1074]));
            let QS = if QR == B { 1.0 } else { 0.0 };
            let AZP;
            let AZR;
            let AZU;
            if QS != 0.0 {
                let QU = FN * (B + (parameters[461] * (if ((BY.powf(QT)) - (CC.powf(QT))) >= A { ((BY.powf(QT)) - (CC.powf(QT))) } else { A })));
                let QW = FO * (B + (parameters[471] * (if ((BY.powf(QV)) - (CC.powf(QV))) >= A { ((BY.powf(QV)) - (CC.powf(QV))) } else { A })));
                AZP = QU;
                AZR = QW;
                AZU = FU;
            } else {
                let QY = FU * (B + (parameters[478] * (if ((BY.powf(QX)) - (CC.powf(QX))) >= A { ((BY.powf(QX)) - (CC.powf(QX))) } else { A })));
                AZP = FN;
                AZR = FO;
                AZU = QY;
            }
            let QZ = if FJ < B { 1.0 } else { 0.0 };
            let SN;
            if QZ != 0.0 {
                SN = B;
            } else {
                let RA = if FJ > AY { 1.0 } else { 0.0 };
                let SO = if RA != 0.0 {
                    AY
                } else {
                    FJ
                };
                SN = SO;
            }
            let BES;
            if KY != 0.0 {
                let RC = if RB < B { 1.0 } else { 0.0 };
                let BET;
                if RC != 0.0 {
                    BET = B;
                } else {
                    let RD = if RB > AY { 1.0 } else { 0.0 };
                    let BEU = if RD != 0.0 {
                        AY
                    } else {
                        RB
                    };
                    BET = BEU;
                }
                BES = BET;
            } else {
                BES = RB;
            }
            let RE = if HX < A { 1.0 } else { 0.0 };
            if RE != 0.0 {
            } else {
            }
            let RF = if IB < A { 1.0 } else { 0.0 };
            if RF != 0.0 {
            } else {
            }
            let RG = if GM <= A { 1.0 } else { 0.0 };
            if RG != 0.0 {
            } else {
            }
            let RH = if GL <= A { 1.0 } else { 0.0 };
            if RH != 0.0 {
            } else {
            }
            let RI = if GB < A { 1.0 } else { 0.0 };
            if RI != 0.0 {
            } else {
            }
            let RJ = if DT < A { 1.0 } else { 0.0 };
            if RJ != 0.0 {
            } else {
            }
            let RK = if LS < A { 1.0 } else { 0.0 };
            if RK != 0.0 {
            } else {
            }
            let RL = if QA < A { 1.0 } else { 0.0 };
            if RL != 0.0 {
            } else {
            }
            let RM = if QD < A { 1.0 } else { 0.0 };
            if RM != 0.0 {
            } else {
            }
            let RN = if DP <= A { 1.0 } else { 0.0 };
            if RN != 0.0 {
            } else {
            }
            let RO = if LP <= A { 1.0 } else { 0.0 };
            if RO != 0.0 {
            } else {
            }
            let RQ = if RP <= A { 1.0 } else { 0.0 };
            if RQ != 0.0 {
            } else {
            }
            let RS = if RR <= A { 1.0 } else { 0.0 };
            if RS != 0.0 {
            } else {
            }
            let RT = if JY <= A { 1.0 } else { 0.0 };
            if RT != 0.0 {
            } else {
            }
            let RU = if DS <= A { 1.0 } else { 0.0 };
            if RU != 0.0 {
            } else {
            }
            let RV = if FA <= A { 1.0 } else { 0.0 };
            if RV != 0.0 {
            } else {
            }
            let RW = if parameters[37] != A { 1.0 } else { 0.0 };
            if RW != 0.0 {
                let RX = if JI <= A { 1.0 } else { 0.0 };
                if RX != 0.0 {
                } else {
                }
            } else {
            }
            let RY = if LV < A { 1.0 } else { 0.0 };
            if RY != 0.0 {
            } else {
            }
            if KY != 0.0 {
                let SA = if RZ < A { 1.0 } else { 0.0 };
                if SA != 0.0 {
                } else {
                }
                let SC = if SB < A { 1.0 } else { 0.0 };
                if SC != 0.0 {
                } else {
                }
            } else {
            }
            let SD = if JJ < A { 1.0 } else { 0.0 };
            let ETY = if SD != 0.0 {
                A
            } else {
                JJ
            };
            let SE = if JK < A { 1.0 } else { 0.0 };
            let EUJ = if SE != 0.0 {
                A
            } else {
                JK
            };
            let SF = if JU < A { 1.0 } else { 0.0 };
            let BFX = if SF != 0.0 {
                A
            } else {
                JU
            };
            let SI = if SG <= A { 1.0 } else { 0.0 };
            let BDN = if SI != 0.0 {
                SJ
            } else {
                SG
            };
            let SK = if MV < A { 1.0 } else { 0.0 };
            let BDQ = if SK != 0.0 {
                A
            } else {
                MV
            };
            let SL = if NA < A { 1.0 } else { 0.0 };
            let BED = if SL != 0.0 {
                A
            } else {
                NA
            };
            let SM = if ND < A { 1.0 } else { 0.0 };
            let BDX = if SM != 0.0 {
                A
            } else {
                ND
            };
            let SP = if SN < A { 1.0 } else { 0.0 };
            let BEA = if SP != 0.0 {
                A
            } else {
                SN
            };
            let SQ = if HM <= A { 1.0 } else { 0.0 };
            let DEF = if SQ != 0.0 {
                B
            } else {
                HM
            };
            let SR = if GY <= A { 1.0 } else { 0.0 };
            if SR != 0.0 {
            } else {
            }
            let ST = if GX <= A { 1.0 } else { 0.0 };
            if ST != 0.0 {
            } else {
            }
            let SV = if SU < A { 1.0 } else { 0.0 };
            if SV != 0.0 {
            } else {
            }
            let SX = parameters[895] - SW;
            let SZ = parameters[897] - SW;
            let AKC;
            let AKY;
            let ATE;
            let AYW;
            let AZE;
            let BIF;
            let BIU;
            if TA != 0.0 {
                let TC = TB * parameters[3];
                AKC = A;
                AKY = A;
                ATE = A;
                AYW = A;
                AZE = TC;
                BIF = A;
                BIU = A;
            } else {
                let TE = if (if TD > A { 1.0 } else { 0.0 }) != 0.0 && (if TB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AKD;
                let AKZ;
                let ATF;
                let AYX;
                let AZF;
                let BIG;
                let BIV;
                if TE != 0.0 {
                    let TH = if TF < TG { 1.0 } else { 0.0 };
                    let UC;
                    let VB;
                    let ADE;
                    let BIH;
                    let BIW;
                    if TH != 0.0 {
                        let TI = if (AA % AY) != A { 1.0 } else { 0.0 };
                        let TP;
                        let TT;
                        let UD;
                        let VC;
                        if TI != 0.0 {
                            let TJ = AY * (if ((AA - B) / AY) >= A { ((AA - B) / AY) } else { A });
                            TP = TJ;
                            TT = TJ;
                            UD = B;
                            VC = B;
                        } else {
                            let TL = if TK == B { 1.0 } else { 0.0 };
                            let TQ;
                            let TU;
                            let UE;
                            let VD;
                            if TL != 0.0 {
                                let TM = AY * (if ((AA / AY) - B) >= A { ((AA / AY) - B) } else { A });
                                TQ = AA;
                                TU = TM;
                                UE = A;
                                VD = AY;
                            } else {
                                let TN = AY * (if ((AA / AY) - B) >= A { ((AA / AY) - B) } else { A });
                                TQ = TN;
                                TU = AA;
                                UE = AY;
                                VD = A;
                            }
                            TP = TQ;
                            TT = TU;
                            UD = UE;
                            VC = VD;
                        }
                        let ADF;
                        if TO != 0.0 {
                            let TR = if TP == A { 1.0 } else { 0.0 };
                            let ADG = if TR != 0.0 {
                                A
                            } else {
                                let TS = (TB * SX) / (BG * TP);
                                TS
                            };
                            ADF = ADG;
                        } else {
                            let TV = if TT == A { 1.0 } else { 0.0 };
                            let ADH = if TV != 0.0 {
                                A
                            } else {
                                let TW = (TB * SX) / (BG * TT);
                                TW
                            };
                            ADF = ADH;
                        }
                        UC = UD;
                        VB = VC;
                        ADE = ADF;
                        BIH = TP;
                        BIW = TT;
                    } else {
                        UC = A;
                        VB = A;
                        ADE = A;
                        BIH = A;
                        BIW = A;
                    }
                    let TX = if TF == A { 1.0 } else { 0.0 };
                    let ADD;
                    let ADX;
                    if TX != 0.0 {
                        let ADY;
                        if TY != 0.0 {
                            let ADZ;
                            if TZ != 0.0 {
                                let UB = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AEA;
                                if UB != 0.0 {
                                    let UF = if UC == A { 1.0 } else { 0.0 };
                                    let AEB = if UF != 0.0 {
                                        A
                                    } else {
                                        let UG = (TB * SX) / (BG * UC);
                                        UG
                                    };
                                    AEA = AEB;
                                } else {
                                    let UK = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AEC;
                                    if UK != 0.0 {
                                        let UL = SX + SY;
                                        let UM = if UL == A { 1.0 } else { 0.0 };
                                        if UM != 0.0 {
                                        } else {
                                        }
                                        let UN = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || UM != 0.0 { 1.0 } else { 0.0 };
                                        let AED = if UN != 0.0 {
                                            A
                                        } else {
                                            let UO = (TB * BG) / ((UH * UC) * UL);
                                            UO
                                        };
                                        AEC = AED;
                                    } else {
                                        AEC = A;
                                    }
                                    AEA = AEC;
                                }
                                ADZ = AEA;
                            } else {
                                let UQ = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AEE;
                                if UQ != 0.0 {
                                    let UR = if UC == A { 1.0 } else { 0.0 };
                                    let AEF = if UR != 0.0 {
                                        A
                                    } else {
                                        let US = (TB * SX) / (BG * UC);
                                        US
                                    };
                                    AEE = AEF;
                                } else {
                                    let UU = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AEG;
                                    if UU != 0.0 {
                                        let UV = SX + SY;
                                        let UW = if UV == A { 1.0 } else { 0.0 };
                                        if UW != 0.0 {
                                        } else {
                                        }
                                        let UX = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || UW != 0.0 { 1.0 } else { 0.0 };
                                        let AEH = if UX != 0.0 {
                                            A
                                        } else {
                                            let UY = (TB * BG) / ((UH * UC) * UV);
                                            UY
                                        };
                                        AEG = AEH;
                                    } else {
                                        AEG = A;
                                    }
                                    AEE = AEG;
                                }
                                ADZ = AEE;
                            }
                            ADY = ADZ;
                        } else {
                            let AEI;
                            if UZ != 0.0 {
                                let VA = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AEJ;
                                if VA != 0.0 {
                                    let VE = if VB == A { 1.0 } else { 0.0 };
                                    let AEK = if VE != 0.0 {
                                        A
                                    } else {
                                        let VF = (TB * SX) / (BG * VB);
                                        VF
                                    };
                                    AEJ = AEK;
                                } else {
                                    let VG = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AEL;
                                    if VG != 0.0 {
                                        let VH = SX + SY;
                                        let VI = if VH == A { 1.0 } else { 0.0 };
                                        if VI != 0.0 {
                                        } else {
                                        }
                                        let VJ = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || VI != 0.0 { 1.0 } else { 0.0 };
                                        let AEM = if VJ != 0.0 {
                                            A
                                        } else {
                                            let VK = (TB * BG) / ((UH * VB) * VH);
                                            VK
                                        };
                                        AEL = AEM;
                                    } else {
                                        AEL = A;
                                    }
                                    AEJ = AEL;
                                }
                                AEI = AEJ;
                            } else {
                                let VL = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AEN;
                                if VL != 0.0 {
                                    let VM = if VB == A { 1.0 } else { 0.0 };
                                    let AEO = if VM != 0.0 {
                                        A
                                    } else {
                                        let VN = (TB * SX) / (BG * VB);
                                        VN
                                    };
                                    AEN = AEO;
                                } else {
                                    let VO = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AEP;
                                    if VO != 0.0 {
                                        let VP = SX + SY;
                                        let VQ = if VP == A { 1.0 } else { 0.0 };
                                        if VQ != 0.0 {
                                        } else {
                                        }
                                        let VR = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || VQ != 0.0 { 1.0 } else { 0.0 };
                                        let AEQ = if VR != 0.0 {
                                            A
                                        } else {
                                            let VS = (TB * BG) / ((UH * VB) * VP);
                                            VS
                                        };
                                        AEP = AEQ;
                                    } else {
                                        AEP = A;
                                    }
                                    AEN = AEP;
                                }
                                AEI = AEN;
                            }
                            ADY = AEI;
                        }
                        ADD = ADE;
                        ADX = ADY;
                    } else {
                        let VT = if TF == B { 1.0 } else { 0.0 };
                        let ADI;
                        let AER;
                        if VT != 0.0 {
                            let AES;
                            if VU != 0.0 {
                                let AET;
                                if VV != 0.0 {
                                    let VW = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AEU;
                                    if VW != 0.0 {
                                        let VX = if UC == A { 1.0 } else { 0.0 };
                                        let AEV = if VX != 0.0 {
                                            A
                                        } else {
                                            let VY = (TB * SX) / (BG * UC);
                                            VY
                                        };
                                        AEU = AEV;
                                    } else {
                                        let VZ = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AEW;
                                        if VZ != 0.0 {
                                            let WA = SX + SY;
                                            let WB = if WA == A { 1.0 } else { 0.0 };
                                            if WB != 0.0 {
                                            } else {
                                            }
                                            let WC = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || WB != 0.0 { 1.0 } else { 0.0 };
                                            let AEX = if WC != 0.0 {
                                                A
                                            } else {
                                                let WD = (TB * BG) / ((UH * UC) * WA);
                                                WD
                                            };
                                            AEW = AEX;
                                        } else {
                                            AEW = A;
                                        }
                                        AEU = AEW;
                                    }
                                    AET = AEU;
                                } else {
                                    let WE = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AEY;
                                    if WE != 0.0 {
                                        let WF = if UC == A { 1.0 } else { 0.0 };
                                        let AEZ = if WF != 0.0 {
                                            A
                                        } else {
                                            let WG = (TB * SX) / (BG * UC);
                                            WG
                                        };
                                        AEY = AEZ;
                                    } else {
                                        let WH = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AFA;
                                        if WH != 0.0 {
                                            let WI = SX + SY;
                                            let WJ = if WI == A { 1.0 } else { 0.0 };
                                            if WJ != 0.0 {
                                            } else {
                                            }
                                            let WK = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || WJ != 0.0 { 1.0 } else { 0.0 };
                                            let AFB = if WK != 0.0 {
                                                A
                                            } else {
                                                let WL = (TB * BG) / ((UH * UC) * WI);
                                                WL
                                            };
                                            AFA = AFB;
                                        } else {
                                            AFA = A;
                                        }
                                        AEY = AFA;
                                    }
                                    AET = AEY;
                                }
                                AES = AET;
                            } else {
                                let AFC;
                                if WM != 0.0 {
                                    let WN = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AFD;
                                    if WN != 0.0 {
                                        let WO = if VB == A { 1.0 } else { 0.0 };
                                        let AFE = if WO != 0.0 {
                                            A
                                        } else {
                                            let WP = (TB * SX) / (BG * VB);
                                            WP
                                        };
                                        AFD = AFE;
                                    } else {
                                        let WQ = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AFF;
                                        if WQ != 0.0 {
                                            let WR = if SX == A { 1.0 } else { 0.0 };
                                            if WR != 0.0 {
                                            } else {
                                            }
                                            let WS = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || WR != 0.0 { 1.0 } else { 0.0 };
                                            let AFG = if WS != 0.0 {
                                                A
                                            } else {
                                                let WT = (TB * BG) / ((UJ * VB) * SX);
                                                WT
                                            };
                                            AFF = AFG;
                                        } else {
                                            AFF = A;
                                        }
                                        AFD = AFF;
                                    }
                                    AFC = AFD;
                                } else {
                                    let WU = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AFH;
                                    if WU != 0.0 {
                                        let WV = if VB == A { 1.0 } else { 0.0 };
                                        let AFI = if WV != 0.0 {
                                            A
                                        } else {
                                            let WW = (TB * SX) / (BG * VB);
                                            WW
                                        };
                                        AFH = AFI;
                                    } else {
                                        let WX = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AFJ;
                                        if WX != 0.0 {
                                            let WY = if SX == A { 1.0 } else { 0.0 };
                                            if WY != 0.0 {
                                            } else {
                                            }
                                            let WZ = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || WY != 0.0 { 1.0 } else { 0.0 };
                                            let AFK = if WZ != 0.0 {
                                                A
                                            } else {
                                                let XA = (TB * BG) / ((UJ * VB) * SX);
                                                XA
                                            };
                                            AFJ = AFK;
                                        } else {
                                            AFJ = A;
                                        }
                                        AFH = AFJ;
                                    }
                                    AFC = AFH;
                                }
                                AES = AFC;
                            }
                            ADI = ADE;
                            AER = AES;
                        } else {
                            let XB = if TF == AY { 1.0 } else { 0.0 };
                            let ADJ;
                            let AFL;
                            if XB != 0.0 {
                                let AFM;
                                if XC != 0.0 {
                                    let AFN;
                                    if XD != 0.0 {
                                        let XE = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AFO;
                                        if XE != 0.0 {
                                            let XF = if UC == A { 1.0 } else { 0.0 };
                                            let AFP = if XF != 0.0 {
                                                A
                                            } else {
                                                let XG = (TB * SX) / (BG * UC);
                                                XG
                                            };
                                            AFO = AFP;
                                        } else {
                                            let XH = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AFQ;
                                            if XH != 0.0 {
                                                let XI = if SX == A { 1.0 } else { 0.0 };
                                                if XI != 0.0 {
                                                } else {
                                                }
                                                let XJ = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || XI != 0.0 { 1.0 } else { 0.0 };
                                                let AFR = if XJ != 0.0 {
                                                    A
                                                } else {
                                                    let XK = (TB * BG) / ((UJ * UC) * SX);
                                                    XK
                                                };
                                                AFQ = AFR;
                                            } else {
                                                AFQ = A;
                                            }
                                            AFO = AFQ;
                                        }
                                        AFN = AFO;
                                    } else {
                                        let XL = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AFS;
                                        if XL != 0.0 {
                                            let XM = if UC == A { 1.0 } else { 0.0 };
                                            let AFT = if XM != 0.0 {
                                                A
                                            } else {
                                                let XN = (TB * SX) / (BG * UC);
                                                XN
                                            };
                                            AFS = AFT;
                                        } else {
                                            let XO = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AFU;
                                            if XO != 0.0 {
                                                let XP = if SX == A { 1.0 } else { 0.0 };
                                                if XP != 0.0 {
                                                } else {
                                                }
                                                let XQ = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || XP != 0.0 { 1.0 } else { 0.0 };
                                                let AFV = if XQ != 0.0 {
                                                    A
                                                } else {
                                                    let XR = (TB * BG) / ((UJ * UC) * SX);
                                                    XR
                                                };
                                                AFU = AFV;
                                            } else {
                                                AFU = A;
                                            }
                                            AFS = AFU;
                                        }
                                        AFN = AFS;
                                    }
                                    AFM = AFN;
                                } else {
                                    let AFW;
                                    if XS != 0.0 {
                                        let XT = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AFX;
                                        if XT != 0.0 {
                                            let XU = if VB == A { 1.0 } else { 0.0 };
                                            let AFY = if XU != 0.0 {
                                                A
                                            } else {
                                                let XV = (TB * SX) / (BG * VB);
                                                XV
                                            };
                                            AFX = AFY;
                                        } else {
                                            let XW = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AFZ;
                                            if XW != 0.0 {
                                                let XX = SX + SY;
                                                let XY = if XX == A { 1.0 } else { 0.0 };
                                                if XY != 0.0 {
                                                } else {
                                                }
                                                let XZ = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || XY != 0.0 { 1.0 } else { 0.0 };
                                                let AGA = if XZ != 0.0 {
                                                    A
                                                } else {
                                                    let YA = (TB * BG) / ((UH * VB) * XX);
                                                    YA
                                                };
                                                AFZ = AGA;
                                            } else {
                                                AFZ = A;
                                            }
                                            AFX = AFZ;
                                        }
                                        AFW = AFX;
                                    } else {
                                        let YB = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AGB;
                                        if YB != 0.0 {
                                            let YC = if VB == A { 1.0 } else { 0.0 };
                                            let AGC = if YC != 0.0 {
                                                A
                                            } else {
                                                let YD = (TB * SX) / (BG * VB);
                                                YD
                                            };
                                            AGB = AGC;
                                        } else {
                                            let YE = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AGD;
                                            if YE != 0.0 {
                                                let YF = SX + SY;
                                                let YG = if YF == A { 1.0 } else { 0.0 };
                                                if YG != 0.0 {
                                                } else {
                                                }
                                                let YH = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || YG != 0.0 { 1.0 } else { 0.0 };
                                                let AGE = if YH != 0.0 {
                                                    A
                                                } else {
                                                    let YI = (TB * BG) / ((UH * VB) * YF);
                                                    YI
                                                };
                                                AGD = AGE;
                                            } else {
                                                AGD = A;
                                            }
                                            AGB = AGD;
                                        }
                                        AFW = AGB;
                                    }
                                    AFM = AFW;
                                }
                                ADJ = ADE;
                                AFL = AFM;
                            } else {
                                let YJ = if TF == UH { 1.0 } else { 0.0 };
                                let ADK;
                                let AGF;
                                if YJ != 0.0 {
                                    let AGG;
                                    if YK != 0.0 {
                                        let AGH;
                                        if YL != 0.0 {
                                            let YM = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AGI;
                                            if YM != 0.0 {
                                                let YN = if UC == A { 1.0 } else { 0.0 };
                                                let AGJ = if YN != 0.0 {
                                                    A
                                                } else {
                                                    let YO = (TB * SX) / (BG * UC);
                                                    YO
                                                };
                                                AGI = AGJ;
                                            } else {
                                                let YP = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AGK;
                                                if YP != 0.0 {
                                                    let YQ = if SX == A { 1.0 } else { 0.0 };
                                                    if YQ != 0.0 {
                                                    } else {
                                                    }
                                                    let YR = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || YQ != 0.0 { 1.0 } else { 0.0 };
                                                    let AGL = if YR != 0.0 {
                                                        A
                                                    } else {
                                                        let YS = (TB * BG) / ((UJ * UC) * SX);
                                                        YS
                                                    };
                                                    AGK = AGL;
                                                } else {
                                                    AGK = A;
                                                }
                                                AGI = AGK;
                                            }
                                            AGH = AGI;
                                        } else {
                                            let YT = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AGM;
                                            if YT != 0.0 {
                                                let YU = if UC == A { 1.0 } else { 0.0 };
                                                let AGN = if YU != 0.0 {
                                                    A
                                                } else {
                                                    let YV = (TB * SX) / (BG * UC);
                                                    YV
                                                };
                                                AGM = AGN;
                                            } else {
                                                let YW = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AGO;
                                                if YW != 0.0 {
                                                    let YX = if SX == A { 1.0 } else { 0.0 };
                                                    if YX != 0.0 {
                                                    } else {
                                                    }
                                                    let YY = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || YX != 0.0 { 1.0 } else { 0.0 };
                                                    let AGP = if YY != 0.0 {
                                                        A
                                                    } else {
                                                        let YZ = (TB * BG) / ((UJ * UC) * SX);
                                                        YZ
                                                    };
                                                    AGO = AGP;
                                                } else {
                                                    AGO = A;
                                                }
                                                AGM = AGO;
                                            }
                                            AGH = AGM;
                                        }
                                        AGG = AGH;
                                    } else {
                                        let AGQ;
                                        if ZA != 0.0 {
                                            let ZB = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AGR;
                                            if ZB != 0.0 {
                                                let ZC = if VB == A { 1.0 } else { 0.0 };
                                                let AGS = if ZC != 0.0 {
                                                    A
                                                } else {
                                                    let ZD = (TB * SX) / (BG * VB);
                                                    ZD
                                                };
                                                AGR = AGS;
                                            } else {
                                                let ZE = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AGT;
                                                if ZE != 0.0 {
                                                    let ZF = if SX == A { 1.0 } else { 0.0 };
                                                    if ZF != 0.0 {
                                                    } else {
                                                    }
                                                    let ZG = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || ZF != 0.0 { 1.0 } else { 0.0 };
                                                    let AGU = if ZG != 0.0 {
                                                        A
                                                    } else {
                                                        let ZH = (TB * BG) / ((UJ * VB) * SX);
                                                        ZH
                                                    };
                                                    AGT = AGU;
                                                } else {
                                                    AGT = A;
                                                }
                                                AGR = AGT;
                                            }
                                            AGQ = AGR;
                                        } else {
                                            let ZI = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AGV;
                                            if ZI != 0.0 {
                                                let ZJ = if VB == A { 1.0 } else { 0.0 };
                                                let AGW = if ZJ != 0.0 {
                                                    A
                                                } else {
                                                    let ZK = (TB * SX) / (BG * VB);
                                                    ZK
                                                };
                                                AGV = AGW;
                                            } else {
                                                let ZL = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AGX;
                                                if ZL != 0.0 {
                                                    let ZM = if SX == A { 1.0 } else { 0.0 };
                                                    if ZM != 0.0 {
                                                    } else {
                                                    }
                                                    let ZN = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || ZM != 0.0 { 1.0 } else { 0.0 };
                                                    let AGY = if ZN != 0.0 {
                                                        A
                                                    } else {
                                                        let ZO = (TB * BG) / ((UJ * VB) * SX);
                                                        ZO
                                                    };
                                                    AGX = AGY;
                                                } else {
                                                    AGX = A;
                                                }
                                                AGV = AGX;
                                            }
                                            AGQ = AGV;
                                        }
                                        AGG = AGQ;
                                    }
                                    ADK = ADE;
                                    AGF = AGG;
                                } else {
                                    let ZP = if TF == UI { 1.0 } else { 0.0 };
                                    let ADL;
                                    let AGZ;
                                    if ZP != 0.0 {
                                        let AHA;
                                        if ZQ != 0.0 {
                                            let AHB;
                                            if ZR != 0.0 {
                                                let ZS = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AHC;
                                                if ZS != 0.0 {
                                                    let ZT = if UC == A { 1.0 } else { 0.0 };
                                                    let AHD = if ZT != 0.0 {
                                                        A
                                                    } else {
                                                        let ZU = (TB * SX) / (BG * UC);
                                                        ZU
                                                    };
                                                    AHC = AHD;
                                                } else {
                                                    let ZV = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AHE;
                                                    if ZV != 0.0 {
                                                        let ZW = SX + SY;
                                                        let ZX = if ZW == A { 1.0 } else { 0.0 };
                                                        if ZX != 0.0 {
                                                        } else {
                                                        }
                                                        let ZY = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || ZX != 0.0 { 1.0 } else { 0.0 };
                                                        let AHF = if ZY != 0.0 {
                                                            A
                                                        } else {
                                                            let ZZ = (TB * BG) / ((UH * UC) * ZW);
                                                            ZZ
                                                        };
                                                        AHE = AHF;
                                                    } else {
                                                        AHE = A;
                                                    }
                                                    AHC = AHE;
                                                }
                                                AHB = AHC;
                                            } else {
                                                let AAA = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AHG;
                                                if AAA != 0.0 {
                                                    let AAB = if UC == A { 1.0 } else { 0.0 };
                                                    let AHH = if AAB != 0.0 {
                                                        A
                                                    } else {
                                                        let AAC = (TB * SX) / (BG * UC);
                                                        AAC
                                                    };
                                                    AHG = AHH;
                                                } else {
                                                    let AAD = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AHI;
                                                    if AAD != 0.0 {
                                                        let AAE = SX + SY;
                                                        let AAF = if AAE == A { 1.0 } else { 0.0 };
                                                        if AAF != 0.0 {
                                                        } else {
                                                        }
                                                        let AAG = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || AAF != 0.0 { 1.0 } else { 0.0 };
                                                        let AHJ = if AAG != 0.0 {
                                                            A
                                                        } else {
                                                            let AAH = (TB * BG) / ((UH * UC) * AAE);
                                                            AAH
                                                        };
                                                        AHI = AHJ;
                                                    } else {
                                                        AHI = A;
                                                    }
                                                    AHG = AHI;
                                                }
                                                AHB = AHG;
                                            }
                                            AHA = AHB;
                                        } else {
                                            let AAI = (TB * SZ) / BG;
                                            AHA = AAI;
                                        }
                                        ADL = ADE;
                                        AGZ = AHA;
                                    } else {
                                        let AAJ = if TF == UA { 1.0 } else { 0.0 };
                                        let ADM;
                                        let AHK;
                                        if AAJ != 0.0 {
                                            let AHL;
                                            if AAK != 0.0 {
                                                let AHM;
                                                if AAL != 0.0 {
                                                    let AAM = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AHN;
                                                    if AAM != 0.0 {
                                                        let AAN = if UC == A { 1.0 } else { 0.0 };
                                                        let AHO = if AAN != 0.0 {
                                                            A
                                                        } else {
                                                            let AAO = (TB * SX) / (BG * UC);
                                                            AAO
                                                        };
                                                        AHN = AHO;
                                                    } else {
                                                        let AAP = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AHP;
                                                        if AAP != 0.0 {
                                                            let AAQ = if SX == A { 1.0 } else { 0.0 };
                                                            if AAQ != 0.0 {
                                                            } else {
                                                            }
                                                            let AAR = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || AAQ != 0.0 { 1.0 } else { 0.0 };
                                                            let AHQ = if AAR != 0.0 {
                                                                A
                                                            } else {
                                                                let AAS = (TB * BG) / ((UJ * UC) * SX);
                                                                AAS
                                                            };
                                                            AHP = AHQ;
                                                        } else {
                                                            AHP = A;
                                                        }
                                                        AHN = AHP;
                                                    }
                                                    AHM = AHN;
                                                } else {
                                                    let AAT = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AHR;
                                                    if AAT != 0.0 {
                                                        let AAU = if UC == A { 1.0 } else { 0.0 };
                                                        let AHS = if AAU != 0.0 {
                                                            A
                                                        } else {
                                                            let AAV = (TB * SX) / (BG * UC);
                                                            AAV
                                                        };
                                                        AHR = AHS;
                                                    } else {
                                                        let AAW = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AHT;
                                                        if AAW != 0.0 {
                                                            let AAX = if SX == A { 1.0 } else { 0.0 };
                                                            if AAX != 0.0 {
                                                            } else {
                                                            }
                                                            let AAY = if (if UC == A { 1.0 } else { 0.0 }) != 0.0 || AAX != 0.0 { 1.0 } else { 0.0 };
                                                            let AHU = if AAY != 0.0 {
                                                                A
                                                            } else {
                                                                let AAZ = (TB * BG) / ((UJ * UC) * SX);
                                                                AAZ
                                                            };
                                                            AHT = AHU;
                                                        } else {
                                                            AHT = A;
                                                        }
                                                        AHR = AHT;
                                                    }
                                                    AHM = AHR;
                                                }
                                                AHL = AHM;
                                            } else {
                                                let ABA = if VB == A { 1.0 } else { 0.0 };
                                                let AHV = if ABA != 0.0 {
                                                    A
                                                } else {
                                                    let ABB = (TB * SZ) / (BG * VB);
                                                    ABB
                                                };
                                                AHL = AHV;
                                            }
                                            ADM = ADE;
                                            AHK = AHL;
                                        } else {
                                            let ABC = if TF == UJ { 1.0 } else { 0.0 };
                                            let ADN;
                                            let AHW;
                                            if ABC != 0.0 {
                                                let AHX;
                                                if ABD != 0.0 {
                                                    let ABE = (TB * SZ) / BG;
                                                    AHX = ABE;
                                                } else {
                                                    let AHY;
                                                    if ABF != 0.0 {
                                                        let ABG = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AHZ;
                                                        if ABG != 0.0 {
                                                            let ABH = if VB == A { 1.0 } else { 0.0 };
                                                            let AIA = if ABH != 0.0 {
                                                                A
                                                            } else {
                                                                let ABI = (TB * SX) / (BG * VB);
                                                                ABI
                                                            };
                                                            AHZ = AIA;
                                                        } else {
                                                            let ABJ = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AIB;
                                                            if ABJ != 0.0 {
                                                                let ABK = SX + SY;
                                                                let ABL = if ABK == A { 1.0 } else { 0.0 };
                                                                if ABL != 0.0 {
                                                                } else {
                                                                }
                                                                let ABM = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || ABL != 0.0 { 1.0 } else { 0.0 };
                                                                let AIC = if ABM != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ABN = (TB * BG) / ((UH * VB) * ABK);
                                                                    ABN
                                                                };
                                                                AIB = AIC;
                                                            } else {
                                                                AIB = A;
                                                            }
                                                            AHZ = AIB;
                                                        }
                                                        AHY = AHZ;
                                                    } else {
                                                        let ABO = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AID;
                                                        if ABO != 0.0 {
                                                            let ABP = if VB == A { 1.0 } else { 0.0 };
                                                            let AIE = if ABP != 0.0 {
                                                                A
                                                            } else {
                                                                let ABQ = (TB * SX) / (BG * VB);
                                                                ABQ
                                                            };
                                                            AID = AIE;
                                                        } else {
                                                            let ABR = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AIF;
                                                            if ABR != 0.0 {
                                                                let ABS = SX + SY;
                                                                let ABT = if ABS == A { 1.0 } else { 0.0 };
                                                                if ABT != 0.0 {
                                                                } else {
                                                                }
                                                                let ABU = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || ABT != 0.0 { 1.0 } else { 0.0 };
                                                                let AIG = if ABU != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ABV = (TB * BG) / ((UH * VB) * ABS);
                                                                    ABV
                                                                };
                                                                AIF = AIG;
                                                            } else {
                                                                AIF = A;
                                                            }
                                                            AID = AIF;
                                                        }
                                                        AHY = AID;
                                                    }
                                                    AHX = AHY;
                                                }
                                                ADN = ADE;
                                                AHW = AHX;
                                            } else {
                                                let ABW = if TF == UP { 1.0 } else { 0.0 };
                                                let ADO;
                                                let AIH;
                                                if ABW != 0.0 {
                                                    let AII;
                                                    if ABX != 0.0 {
                                                        let ABY = if UC == A { 1.0 } else { 0.0 };
                                                        let AIJ = if ABY != 0.0 {
                                                            A
                                                        } else {
                                                            let ABZ = (TB * SZ) / (BG * UC);
                                                            ABZ
                                                        };
                                                        AII = AIJ;
                                                    } else {
                                                        let AIK;
                                                        if ACA != 0.0 {
                                                            let ACB = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AIL;
                                                            if ACB != 0.0 {
                                                                let ACC = if VB == A { 1.0 } else { 0.0 };
                                                                let AIM = if ACC != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ACD = (TB * SX) / (BG * VB);
                                                                    ACD
                                                                };
                                                                AIL = AIM;
                                                            } else {
                                                                let ACE = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AIN;
                                                                if ACE != 0.0 {
                                                                    let ACF = if SX == A { 1.0 } else { 0.0 };
                                                                    if ACF != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let ACG = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || ACF != 0.0 { 1.0 } else { 0.0 };
                                                                    let AIO = if ACG != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ACH = (TB * BG) / ((UJ * VB) * SX);
                                                                        ACH
                                                                    };
                                                                    AIN = AIO;
                                                                } else {
                                                                    AIN = A;
                                                                }
                                                                AIL = AIN;
                                                            }
                                                            AIK = AIL;
                                                        } else {
                                                            let ACI = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AIP;
                                                            if ACI != 0.0 {
                                                                let ACJ = if VB == A { 1.0 } else { 0.0 };
                                                                let AIQ = if ACJ != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ACK = (TB * SX) / (BG * VB);
                                                                    ACK
                                                                };
                                                                AIP = AIQ;
                                                            } else {
                                                                let ACL = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AIR;
                                                                if ACL != 0.0 {
                                                                    let ACM = if SX == A { 1.0 } else { 0.0 };
                                                                    if ACM != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let ACN = if (if VB == A { 1.0 } else { 0.0 }) != 0.0 || ACM != 0.0 { 1.0 } else { 0.0 };
                                                                    let AIS = if ACN != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ACO = (TB * BG) / ((UJ * VB) * SX);
                                                                        ACO
                                                                    };
                                                                    AIR = AIS;
                                                                } else {
                                                                    AIR = A;
                                                                }
                                                                AIP = AIR;
                                                            }
                                                            AIK = AIP;
                                                        }
                                                        AII = AIK;
                                                    }
                                                    ADO = ADE;
                                                    AIH = AII;
                                                } else {
                                                    let ACP = if TF == UT { 1.0 } else { 0.0 };
                                                    let ADP;
                                                    let AIT;
                                                    if ACP != 0.0 {
                                                        let ACQ = (TB * SZ) / BG;
                                                        ADP = ADE;
                                                        AIT = ACQ;
                                                    } else {
                                                        let ACR = if TF == TG { 1.0 } else { 0.0 };
                                                        let ADQ;
                                                        let AIU;
                                                        if ACR != 0.0 {
                                                            let ADR;
                                                            let AIV;
                                                            if ACS != 0.0 {
                                                                let ACT = ((NZ * TB) * SX) / BG;
                                                                let ACU = if AA == AY { 1.0 } else { 0.0 };
                                                                let ADS = if ACU != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ACV = (TB * SX) / (BG * (AA - AY));
                                                                    ACV
                                                                };
                                                                ADR = ADS;
                                                                AIV = ACT;
                                                            } else {
                                                                let ACW = (TB * SX) / (BG * AA);
                                                                ADR = ACW;
                                                                AIV = A;
                                                            }
                                                            ADQ = ADR;
                                                            AIU = AIV;
                                                        } else {
                                                            let ACX = if TF == SS { 1.0 } else { 0.0 };
                                                            let ADT;
                                                            let AIW;
                                                            if ACX != 0.0 {
                                                                let ADU;
                                                                let AIX;
                                                                if ACY != 0.0 {
                                                                    let ACZ = (TB * SX) / (BG * AA);
                                                                    ADU = ACZ;
                                                                    AIX = A;
                                                                } else {
                                                                    let ADA = ((NZ * TB) * SX) / BG;
                                                                    let ADB = if AA == AY { 1.0 } else { 0.0 };
                                                                    let ADV = if ADB != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ADC = (TB * SX) / (BG * (AA - AY));
                                                                        ADC
                                                                    };
                                                                    ADU = ADV;
                                                                    AIX = ADA;
                                                                }
                                                                ADT = ADU;
                                                                AIW = AIX;
                                                            } else {
                                                                ADT = A;
                                                                AIW = A;
                                                            }
                                                            ADQ = ADT;
                                                            AIU = AIW;
                                                        }
                                                        ADP = ADQ;
                                                        AIT = AIU;
                                                    }
                                                    ADO = ADP;
                                                    AIH = AIT;
                                                }
                                                ADN = ADO;
                                                AHW = AIH;
                                            }
                                            ADM = ADN;
                                            AHK = AHW;
                                        }
                                        ADL = ADM;
                                        AGZ = AHK;
                                    }
                                    ADK = ADL;
                                    AGF = AGZ;
                                }
                                ADJ = ADK;
                                AFL = AGF;
                            }
                            ADI = ADJ;
                            AER = AFL;
                        }
                        ADD = ADI;
                        ADX = AER;
                    }
                    let ADW = if ADD <= A { 1.0 } else { 0.0 };
                    let AJA;
                    if ADW != 0.0 {
                        AJA = ADX;
                    } else {
                        let AIY = if ADX <= A { 1.0 } else { 0.0 };
                        let AJB = if AIY != 0.0 {
                            ADD
                        } else {
                            let AIZ = (ADD * ADX) / (ADD + ADX);
                            AIZ
                        };
                        AJA = AJB;
                    }
                    let AJC = if AJA == A { 1.0 } else { 0.0 };
                    if AJC != 0.0 {
                    } else {
                    }
                    AKD = UC;
                    AKZ = VB;
                    ATF = ADD;
                    AYX = ADX;
                    AZF = AJA;
                    BIG = BIH;
                    BIV = BIW;
                } else {
                    AKD = A;
                    AKZ = A;
                    ATF = A;
                    AYX = A;
                    AZF = A;
                    BIG = A;
                    BIV = A;
                }
                AKC = AKD;
                AKY = AKZ;
                ATE = ATF;
                AYW = AYX;
                AZE = AZF;
                BIF = BIG;
                BIU = BIV;
            }
            let AZI;
            let BHZ;
            let BIE;
            let BIO;
            let BIT;
            if AJD != 0.0 {
                let AJE = TB * parameters[4];
                AZI = AJE;
                BHZ = AKC;
                BIE = BIF;
                BIO = AKY;
                BIT = BIU;
            } else {
                let AJF = if (if TD > A { 1.0 } else { 0.0 }) != 0.0 && (if TB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AZJ;
                let BIA;
                let BII;
                let BIP;
                let BIX;
                if AJF != 0.0 {
                    let AJG = if TF < TG { 1.0 } else { 0.0 };
                    let AJZ;
                    let AKV;
                    let ATA;
                    let BIJ;
                    let BIY;
                    if AJG != 0.0 {
                        let AJH = if (AA % AY) != A { 1.0 } else { 0.0 };
                        let AJN;
                        let AJR;
                        let AKA;
                        let AKW;
                        if AJH != 0.0 {
                            let AJI = AY * (if ((AA - B) / AY) >= A { ((AA - B) / AY) } else { A });
                            AJN = AJI;
                            AJR = AJI;
                            AKA = B;
                            AKW = B;
                        } else {
                            let AJJ = if TK == B { 1.0 } else { 0.0 };
                            let AJO;
                            let AJS;
                            let AKB;
                            let AKX;
                            if AJJ != 0.0 {
                                let AJK = AY * (if ((AA / AY) - B) >= A { ((AA / AY) - B) } else { A });
                                AJO = AA;
                                AJS = AJK;
                                AKB = A;
                                AKX = AY;
                            } else {
                                let AJL = AY * (if ((AA / AY) - B) >= A { ((AA / AY) - B) } else { A });
                                AJO = AJL;
                                AJS = AA;
                                AKB = AY;
                                AKX = A;
                            }
                            AJN = AJO;
                            AJR = AJS;
                            AKA = AKB;
                            AKW = AKX;
                        }
                        let ATB;
                        if AJM != 0.0 {
                            let AJP = if AJN == A { 1.0 } else { 0.0 };
                            let ATC = if AJP != 0.0 {
                                A
                            } else {
                                let AJQ = (TB * SX) / (BG * AJN);
                                AJQ
                            };
                            ATB = ATC;
                        } else {
                            let AJT = if AJR == A { 1.0 } else { 0.0 };
                            let ATD = if AJT != 0.0 {
                                A
                            } else {
                                let AJU = (TB * SX) / (BG * AJR);
                                AJU
                            };
                            ATB = ATD;
                        }
                        AJZ = AKA;
                        AKV = AKW;
                        ATA = ATB;
                        BIJ = AJN;
                        BIY = AJR;
                    } else {
                        AJZ = AKC;
                        AKV = AKY;
                        ATA = ATE;
                        BIJ = BIF;
                        BIY = BIU;
                    }
                    let AJV = if TF == A { 1.0 } else { 0.0 };
                    let ASZ;
                    let ATV;
                    if AJV != 0.0 {
                        let ATW;
                        if AJW != 0.0 {
                            let ATX;
                            if AJX != 0.0 {
                                let AJY = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ATY;
                                if AJY != 0.0 {
                                    let AKE = if AJZ == A { 1.0 } else { 0.0 };
                                    let ATZ = if AKE != 0.0 {
                                        A
                                    } else {
                                        let AKF = (TB * SX) / (BG * AJZ);
                                        AKF
                                    };
                                    ATY = ATZ;
                                } else {
                                    let AKG = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AUA;
                                    if AKG != 0.0 {
                                        let AKH = SX + SY;
                                        let AKI = if AKH == A { 1.0 } else { 0.0 };
                                        if AKI != 0.0 {
                                        } else {
                                        }
                                        let AKJ = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AKI != 0.0 { 1.0 } else { 0.0 };
                                        let AUB = if AKJ != 0.0 {
                                            A
                                        } else {
                                            let AKK = (TB * BG) / ((UH * AJZ) * AKH);
                                            AKK
                                        };
                                        AUA = AUB;
                                    } else {
                                        AUA = A;
                                    }
                                    ATY = AUA;
                                }
                                ATX = ATY;
                            } else {
                                let AKL = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AUC;
                                if AKL != 0.0 {
                                    let AKM = if AJZ == A { 1.0 } else { 0.0 };
                                    let AUD = if AKM != 0.0 {
                                        A
                                    } else {
                                        let AKN = (TB * SX) / (BG * AJZ);
                                        AKN
                                    };
                                    AUC = AUD;
                                } else {
                                    let AKO = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AUE;
                                    if AKO != 0.0 {
                                        let AKP = SX + SY;
                                        let AKQ = if AKP == A { 1.0 } else { 0.0 };
                                        if AKQ != 0.0 {
                                        } else {
                                        }
                                        let AKR = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AKQ != 0.0 { 1.0 } else { 0.0 };
                                        let AUF = if AKR != 0.0 {
                                            A
                                        } else {
                                            let AKS = (TB * BG) / ((UH * AJZ) * AKP);
                                            AKS
                                        };
                                        AUE = AUF;
                                    } else {
                                        AUE = A;
                                    }
                                    AUC = AUE;
                                }
                                ATX = AUC;
                            }
                            ATW = ATX;
                        } else {
                            let AUG;
                            if AKT != 0.0 {
                                let AKU = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AUH;
                                if AKU != 0.0 {
                                    let ALA = if AKV == A { 1.0 } else { 0.0 };
                                    let AUI = if ALA != 0.0 {
                                        A
                                    } else {
                                        let ALB = (TB * SX) / (BG * AKV);
                                        ALB
                                    };
                                    AUH = AUI;
                                } else {
                                    let ALC = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AUJ;
                                    if ALC != 0.0 {
                                        let ALD = SX + SY;
                                        let ALE = if ALD == A { 1.0 } else { 0.0 };
                                        if ALE != 0.0 {
                                        } else {
                                        }
                                        let ALF = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ALE != 0.0 { 1.0 } else { 0.0 };
                                        let AUK = if ALF != 0.0 {
                                            A
                                        } else {
                                            let ALG = (TB * BG) / ((UH * AKV) * ALD);
                                            ALG
                                        };
                                        AUJ = AUK;
                                    } else {
                                        AUJ = A;
                                    }
                                    AUH = AUJ;
                                }
                                AUG = AUH;
                            } else {
                                let ALH = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AUL;
                                if ALH != 0.0 {
                                    let ALI = if AKV == A { 1.0 } else { 0.0 };
                                    let AUM = if ALI != 0.0 {
                                        A
                                    } else {
                                        let ALJ = (TB * SX) / (BG * AKV);
                                        ALJ
                                    };
                                    AUL = AUM;
                                } else {
                                    let ALK = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AUN;
                                    if ALK != 0.0 {
                                        let ALL = SX + SY;
                                        let ALM = if ALL == A { 1.0 } else { 0.0 };
                                        if ALM != 0.0 {
                                        } else {
                                        }
                                        let ALN = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ALM != 0.0 { 1.0 } else { 0.0 };
                                        let AUO = if ALN != 0.0 {
                                            A
                                        } else {
                                            let ALO = (TB * BG) / ((UH * AKV) * ALL);
                                            ALO
                                        };
                                        AUN = AUO;
                                    } else {
                                        AUN = A;
                                    }
                                    AUL = AUN;
                                }
                                AUG = AUL;
                            }
                            ATW = AUG;
                        }
                        ASZ = ATA;
                        ATV = ATW;
                    } else {
                        let ALP = if TF == B { 1.0 } else { 0.0 };
                        let ATG;
                        let AUP;
                        if ALP != 0.0 {
                            let AUQ;
                            if ALQ != 0.0 {
                                let AUR;
                                if ALR != 0.0 {
                                    let ALS = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AUS;
                                    if ALS != 0.0 {
                                        let ALT = if AJZ == A { 1.0 } else { 0.0 };
                                        let AUT = if ALT != 0.0 {
                                            A
                                        } else {
                                            let ALU = (TB * SX) / (BG * AJZ);
                                            ALU
                                        };
                                        AUS = AUT;
                                    } else {
                                        let ALV = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AUU;
                                        if ALV != 0.0 {
                                            let ALW = SX + SY;
                                            let ALX = if ALW == A { 1.0 } else { 0.0 };
                                            if ALX != 0.0 {
                                            } else {
                                            }
                                            let ALY = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || ALX != 0.0 { 1.0 } else { 0.0 };
                                            let AUV = if ALY != 0.0 {
                                                A
                                            } else {
                                                let ALZ = (TB * BG) / ((UH * AJZ) * ALW);
                                                ALZ
                                            };
                                            AUU = AUV;
                                        } else {
                                            AUU = A;
                                        }
                                        AUS = AUU;
                                    }
                                    AUR = AUS;
                                } else {
                                    let AMA = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AUW;
                                    if AMA != 0.0 {
                                        let AMB = if AJZ == A { 1.0 } else { 0.0 };
                                        let AUX = if AMB != 0.0 {
                                            A
                                        } else {
                                            let AMC = (TB * SX) / (BG * AJZ);
                                            AMC
                                        };
                                        AUW = AUX;
                                    } else {
                                        let AMD = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AUY;
                                        if AMD != 0.0 {
                                            let AME = SX + SY;
                                            let AMF = if AME == A { 1.0 } else { 0.0 };
                                            if AMF != 0.0 {
                                            } else {
                                            }
                                            let AMG = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AMF != 0.0 { 1.0 } else { 0.0 };
                                            let AUZ = if AMG != 0.0 {
                                                A
                                            } else {
                                                let AMH = (TB * BG) / ((UH * AJZ) * AME);
                                                AMH
                                            };
                                            AUY = AUZ;
                                        } else {
                                            AUY = A;
                                        }
                                        AUW = AUY;
                                    }
                                    AUR = AUW;
                                }
                                AUQ = AUR;
                            } else {
                                let AVA;
                                if AMI != 0.0 {
                                    let AMJ = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AVB;
                                    if AMJ != 0.0 {
                                        let AMK = if AKV == A { 1.0 } else { 0.0 };
                                        let AVC = if AMK != 0.0 {
                                            A
                                        } else {
                                            let AML = (TB * SX) / (BG * AKV);
                                            AML
                                        };
                                        AVB = AVC;
                                    } else {
                                        let AMM = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AVD;
                                        if AMM != 0.0 {
                                            let AMN = if SX == A { 1.0 } else { 0.0 };
                                            if AMN != 0.0 {
                                            } else {
                                            }
                                            let AMO = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || AMN != 0.0 { 1.0 } else { 0.0 };
                                            let AVE = if AMO != 0.0 {
                                                A
                                            } else {
                                                let AMP = (TB * BG) / ((UJ * AKV) * SX);
                                                AMP
                                            };
                                            AVD = AVE;
                                        } else {
                                            AVD = A;
                                        }
                                        AVB = AVD;
                                    }
                                    AVA = AVB;
                                } else {
                                    let AMQ = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AVF;
                                    if AMQ != 0.0 {
                                        let AMR = if AKV == A { 1.0 } else { 0.0 };
                                        let AVG = if AMR != 0.0 {
                                            A
                                        } else {
                                            let AMS = (TB * SX) / (BG * AKV);
                                            AMS
                                        };
                                        AVF = AVG;
                                    } else {
                                        let AMT = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AVH;
                                        if AMT != 0.0 {
                                            let AMU = if SX == A { 1.0 } else { 0.0 };
                                            if AMU != 0.0 {
                                            } else {
                                            }
                                            let AMV = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || AMU != 0.0 { 1.0 } else { 0.0 };
                                            let AVI = if AMV != 0.0 {
                                                A
                                            } else {
                                                let AMW = (TB * BG) / ((UJ * AKV) * SX);
                                                AMW
                                            };
                                            AVH = AVI;
                                        } else {
                                            AVH = A;
                                        }
                                        AVF = AVH;
                                    }
                                    AVA = AVF;
                                }
                                AUQ = AVA;
                            }
                            ATG = ATA;
                            AUP = AUQ;
                        } else {
                            let AMX = if TF == AY { 1.0 } else { 0.0 };
                            let ATH;
                            let AVJ;
                            if AMX != 0.0 {
                                let AVK;
                                if AMY != 0.0 {
                                    let AVL;
                                    if AMZ != 0.0 {
                                        let ANA = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AVM;
                                        if ANA != 0.0 {
                                            let ANB = if AJZ == A { 1.0 } else { 0.0 };
                                            let AVN = if ANB != 0.0 {
                                                A
                                            } else {
                                                let ANC = (TB * SX) / (BG * AJZ);
                                                ANC
                                            };
                                            AVM = AVN;
                                        } else {
                                            let AND = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AVO;
                                            if AND != 0.0 {
                                                let ANE = if SX == A { 1.0 } else { 0.0 };
                                                if ANE != 0.0 {
                                                } else {
                                                }
                                                let ANF = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || ANE != 0.0 { 1.0 } else { 0.0 };
                                                let AVP = if ANF != 0.0 {
                                                    A
                                                } else {
                                                    let ANG = (TB * BG) / ((UJ * AJZ) * SX);
                                                    ANG
                                                };
                                                AVO = AVP;
                                            } else {
                                                AVO = A;
                                            }
                                            AVM = AVO;
                                        }
                                        AVL = AVM;
                                    } else {
                                        let ANH = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AVQ;
                                        if ANH != 0.0 {
                                            let ANI = if AJZ == A { 1.0 } else { 0.0 };
                                            let AVR = if ANI != 0.0 {
                                                A
                                            } else {
                                                let ANJ = (TB * SX) / (BG * AJZ);
                                                ANJ
                                            };
                                            AVQ = AVR;
                                        } else {
                                            let ANK = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AVS;
                                            if ANK != 0.0 {
                                                let ANL = if SX == A { 1.0 } else { 0.0 };
                                                if ANL != 0.0 {
                                                } else {
                                                }
                                                let ANM = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || ANL != 0.0 { 1.0 } else { 0.0 };
                                                let AVT = if ANM != 0.0 {
                                                    A
                                                } else {
                                                    let ANN = (TB * BG) / ((UJ * AJZ) * SX);
                                                    ANN
                                                };
                                                AVS = AVT;
                                            } else {
                                                AVS = A;
                                            }
                                            AVQ = AVS;
                                        }
                                        AVL = AVQ;
                                    }
                                    AVK = AVL;
                                } else {
                                    let AVU;
                                    if ANO != 0.0 {
                                        let ANP = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AVV;
                                        if ANP != 0.0 {
                                            let ANQ = if AKV == A { 1.0 } else { 0.0 };
                                            let AVW = if ANQ != 0.0 {
                                                A
                                            } else {
                                                let ANR = (TB * SX) / (BG * AKV);
                                                ANR
                                            };
                                            AVV = AVW;
                                        } else {
                                            let ANS = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AVX;
                                            if ANS != 0.0 {
                                                let ANT = SX + SY;
                                                let ANU = if ANT == A { 1.0 } else { 0.0 };
                                                if ANU != 0.0 {
                                                } else {
                                                }
                                                let ANV = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ANU != 0.0 { 1.0 } else { 0.0 };
                                                let AVY = if ANV != 0.0 {
                                                    A
                                                } else {
                                                    let ANW = (TB * BG) / ((UH * AKV) * ANT);
                                                    ANW
                                                };
                                                AVX = AVY;
                                            } else {
                                                AVX = A;
                                            }
                                            AVV = AVX;
                                        }
                                        AVU = AVV;
                                    } else {
                                        let ANX = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let AVZ;
                                        if ANX != 0.0 {
                                            let ANY = if AKV == A { 1.0 } else { 0.0 };
                                            let AWA = if ANY != 0.0 {
                                                A
                                            } else {
                                                let ANZ = (TB * SX) / (BG * AKV);
                                                ANZ
                                            };
                                            AVZ = AWA;
                                        } else {
                                            let AOA = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AWB;
                                            if AOA != 0.0 {
                                                let AOB = SX + SY;
                                                let AOC = if AOB == A { 1.0 } else { 0.0 };
                                                if AOC != 0.0 {
                                                } else {
                                                }
                                                let AOD = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || AOC != 0.0 { 1.0 } else { 0.0 };
                                                let AWC = if AOD != 0.0 {
                                                    A
                                                } else {
                                                    let AOE = (TB * BG) / ((UH * AKV) * AOB);
                                                    AOE
                                                };
                                                AWB = AWC;
                                            } else {
                                                AWB = A;
                                            }
                                            AVZ = AWB;
                                        }
                                        AVU = AVZ;
                                    }
                                    AVK = AVU;
                                }
                                ATH = ATA;
                                AVJ = AVK;
                            } else {
                                let AOF = if TF == UH { 1.0 } else { 0.0 };
                                let ATI;
                                let AWD;
                                if AOF != 0.0 {
                                    let AWE;
                                    if AOG != 0.0 {
                                        let AWF;
                                        if AOH != 0.0 {
                                            let AOI = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AWG;
                                            if AOI != 0.0 {
                                                let AOJ = if AJZ == A { 1.0 } else { 0.0 };
                                                let AWH = if AOJ != 0.0 {
                                                    A
                                                } else {
                                                    let AOK = (TB * SX) / (BG * AJZ);
                                                    AOK
                                                };
                                                AWG = AWH;
                                            } else {
                                                let AOL = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AWI;
                                                if AOL != 0.0 {
                                                    let AOM = if SX == A { 1.0 } else { 0.0 };
                                                    if AOM != 0.0 {
                                                    } else {
                                                    }
                                                    let AON = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AOM != 0.0 { 1.0 } else { 0.0 };
                                                    let AWJ = if AON != 0.0 {
                                                        A
                                                    } else {
                                                        let AOO = (TB * BG) / ((UJ * AJZ) * SX);
                                                        AOO
                                                    };
                                                    AWI = AWJ;
                                                } else {
                                                    AWI = A;
                                                }
                                                AWG = AWI;
                                            }
                                            AWF = AWG;
                                        } else {
                                            let AOP = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AWK;
                                            if AOP != 0.0 {
                                                let AOQ = if AJZ == A { 1.0 } else { 0.0 };
                                                let AWL = if AOQ != 0.0 {
                                                    A
                                                } else {
                                                    let AOR = (TB * SX) / (BG * AJZ);
                                                    AOR
                                                };
                                                AWK = AWL;
                                            } else {
                                                let AOS = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AWM;
                                                if AOS != 0.0 {
                                                    let AOT = if SX == A { 1.0 } else { 0.0 };
                                                    if AOT != 0.0 {
                                                    } else {
                                                    }
                                                    let AOU = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AOT != 0.0 { 1.0 } else { 0.0 };
                                                    let AWN = if AOU != 0.0 {
                                                        A
                                                    } else {
                                                        let AOV = (TB * BG) / ((UJ * AJZ) * SX);
                                                        AOV
                                                    };
                                                    AWM = AWN;
                                                } else {
                                                    AWM = A;
                                                }
                                                AWK = AWM;
                                            }
                                            AWF = AWK;
                                        }
                                        AWE = AWF;
                                    } else {
                                        let AWO;
                                        if AOW != 0.0 {
                                            let AOX = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AWP;
                                            if AOX != 0.0 {
                                                let AOY = if AKV == A { 1.0 } else { 0.0 };
                                                let AWQ = if AOY != 0.0 {
                                                    A
                                                } else {
                                                    let AOZ = (TB * SX) / (BG * AKV);
                                                    AOZ
                                                };
                                                AWP = AWQ;
                                            } else {
                                                let APA = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AWR;
                                                if APA != 0.0 {
                                                    let APB = if SX == A { 1.0 } else { 0.0 };
                                                    if APB != 0.0 {
                                                    } else {
                                                    }
                                                    let APC = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || APB != 0.0 { 1.0 } else { 0.0 };
                                                    let AWS = if APC != 0.0 {
                                                        A
                                                    } else {
                                                        let APD = (TB * BG) / ((UJ * AKV) * SX);
                                                        APD
                                                    };
                                                    AWR = AWS;
                                                } else {
                                                    AWR = A;
                                                }
                                                AWP = AWR;
                                            }
                                            AWO = AWP;
                                        } else {
                                            let APE = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let AWT;
                                            if APE != 0.0 {
                                                let APF = if AKV == A { 1.0 } else { 0.0 };
                                                let AWU = if APF != 0.0 {
                                                    A
                                                } else {
                                                    let APG = (TB * SX) / (BG * AKV);
                                                    APG
                                                };
                                                AWT = AWU;
                                            } else {
                                                let APH = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AWV;
                                                if APH != 0.0 {
                                                    let API = if SX == A { 1.0 } else { 0.0 };
                                                    if API != 0.0 {
                                                    } else {
                                                    }
                                                    let APJ = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || API != 0.0 { 1.0 } else { 0.0 };
                                                    let AWW = if APJ != 0.0 {
                                                        A
                                                    } else {
                                                        let APK = (TB * BG) / ((UJ * AKV) * SX);
                                                        APK
                                                    };
                                                    AWV = AWW;
                                                } else {
                                                    AWV = A;
                                                }
                                                AWT = AWV;
                                            }
                                            AWO = AWT;
                                        }
                                        AWE = AWO;
                                    }
                                    ATI = ATA;
                                    AWD = AWE;
                                } else {
                                    let APL = if TF == UI { 1.0 } else { 0.0 };
                                    let ATJ;
                                    let AWX;
                                    if APL != 0.0 {
                                        let AWY;
                                        if APM != 0.0 {
                                            let AWZ;
                                            if APN != 0.0 {
                                                let APO = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AXA;
                                                if APO != 0.0 {
                                                    let APP = if AJZ == A { 1.0 } else { 0.0 };
                                                    let AXB = if APP != 0.0 {
                                                        A
                                                    } else {
                                                        let APQ = (TB * SX) / (BG * AJZ);
                                                        APQ
                                                    };
                                                    AXA = AXB;
                                                } else {
                                                    let APR = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AXC;
                                                    if APR != 0.0 {
                                                        let APS = SX + SY;
                                                        let APT = if APS == A { 1.0 } else { 0.0 };
                                                        if APT != 0.0 {
                                                        } else {
                                                        }
                                                        let APU = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || APT != 0.0 { 1.0 } else { 0.0 };
                                                        let AXD = if APU != 0.0 {
                                                            A
                                                        } else {
                                                            let APV = (TB * BG) / ((UH * AJZ) * APS);
                                                            APV
                                                        };
                                                        AXC = AXD;
                                                    } else {
                                                        AXC = A;
                                                    }
                                                    AXA = AXC;
                                                }
                                                AWZ = AXA;
                                            } else {
                                                let APW = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let AXE;
                                                if APW != 0.0 {
                                                    let APX = if AJZ == A { 1.0 } else { 0.0 };
                                                    let AXF = if APX != 0.0 {
                                                        A
                                                    } else {
                                                        let APY = (TB * SX) / (BG * AJZ);
                                                        APY
                                                    };
                                                    AXE = AXF;
                                                } else {
                                                    let APZ = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AXG;
                                                    if APZ != 0.0 {
                                                        let AQA = SX + SY;
                                                        let AQB = if AQA == A { 1.0 } else { 0.0 };
                                                        if AQB != 0.0 {
                                                        } else {
                                                        }
                                                        let AQC = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AQB != 0.0 { 1.0 } else { 0.0 };
                                                        let AXH = if AQC != 0.0 {
                                                            A
                                                        } else {
                                                            let AQD = (TB * BG) / ((UH * AJZ) * AQA);
                                                            AQD
                                                        };
                                                        AXG = AXH;
                                                    } else {
                                                        AXG = A;
                                                    }
                                                    AXE = AXG;
                                                }
                                                AWZ = AXE;
                                            }
                                            AWY = AWZ;
                                        } else {
                                            let AQE = (TB * SZ) / BG;
                                            AWY = AQE;
                                        }
                                        ATJ = ATA;
                                        AWX = AWY;
                                    } else {
                                        let AQF = if TF == UA { 1.0 } else { 0.0 };
                                        let ATK;
                                        let AXI;
                                        if AQF != 0.0 {
                                            let AXJ;
                                            if AQG != 0.0 {
                                                let AXK;
                                                if AQH != 0.0 {
                                                    let AQI = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AXL;
                                                    if AQI != 0.0 {
                                                        let AQJ = if AJZ == A { 1.0 } else { 0.0 };
                                                        let AXM = if AQJ != 0.0 {
                                                            A
                                                        } else {
                                                            let AQK = (TB * SX) / (BG * AJZ);
                                                            AQK
                                                        };
                                                        AXL = AXM;
                                                    } else {
                                                        let AQL = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AXN;
                                                        if AQL != 0.0 {
                                                            let AQM = if SX == A { 1.0 } else { 0.0 };
                                                            if AQM != 0.0 {
                                                            } else {
                                                            }
                                                            let AQN = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AQM != 0.0 { 1.0 } else { 0.0 };
                                                            let AXO = if AQN != 0.0 {
                                                                A
                                                            } else {
                                                                let AQO = (TB * BG) / ((UJ * AJZ) * SX);
                                                                AQO
                                                            };
                                                            AXN = AXO;
                                                        } else {
                                                            AXN = A;
                                                        }
                                                        AXL = AXN;
                                                    }
                                                    AXK = AXL;
                                                } else {
                                                    let AQP = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let AXP;
                                                    if AQP != 0.0 {
                                                        let AQQ = if AJZ == A { 1.0 } else { 0.0 };
                                                        let AXQ = if AQQ != 0.0 {
                                                            A
                                                        } else {
                                                            let AQR = (TB * SX) / (BG * AJZ);
                                                            AQR
                                                        };
                                                        AXP = AXQ;
                                                    } else {
                                                        let AQS = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AXR;
                                                        if AQS != 0.0 {
                                                            let AQT = if SX == A { 1.0 } else { 0.0 };
                                                            if AQT != 0.0 {
                                                            } else {
                                                            }
                                                            let AQU = if (if AJZ == A { 1.0 } else { 0.0 }) != 0.0 || AQT != 0.0 { 1.0 } else { 0.0 };
                                                            let AXS = if AQU != 0.0 {
                                                                A
                                                            } else {
                                                                let AQV = (TB * BG) / ((UJ * AJZ) * SX);
                                                                AQV
                                                            };
                                                            AXR = AXS;
                                                        } else {
                                                            AXR = A;
                                                        }
                                                        AXP = AXR;
                                                    }
                                                    AXK = AXP;
                                                }
                                                AXJ = AXK;
                                            } else {
                                                let AQW = if AKV == A { 1.0 } else { 0.0 };
                                                let AXT = if AQW != 0.0 {
                                                    A
                                                } else {
                                                    let AQX = (TB * SZ) / (BG * AKV);
                                                    AQX
                                                };
                                                AXJ = AXT;
                                            }
                                            ATK = ATA;
                                            AXI = AXJ;
                                        } else {
                                            let AQY = if TF == UJ { 1.0 } else { 0.0 };
                                            let ATL;
                                            let AXU;
                                            if AQY != 0.0 {
                                                let AXV;
                                                if AQZ != 0.0 {
                                                    let ARA = (TB * SZ) / BG;
                                                    AXV = ARA;
                                                } else {
                                                    let AXW;
                                                    if ARB != 0.0 {
                                                        let ARC = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AXX;
                                                        if ARC != 0.0 {
                                                            let ARD = if AKV == A { 1.0 } else { 0.0 };
                                                            let AXY = if ARD != 0.0 {
                                                                A
                                                            } else {
                                                                let ARE = (TB * SX) / (BG * AKV);
                                                                ARE
                                                            };
                                                            AXX = AXY;
                                                        } else {
                                                            let ARF = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AXZ;
                                                            if ARF != 0.0 {
                                                                let ARG = SX + SY;
                                                                let ARH = if ARG == A { 1.0 } else { 0.0 };
                                                                if ARH != 0.0 {
                                                                } else {
                                                                }
                                                                let ARI = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ARH != 0.0 { 1.0 } else { 0.0 };
                                                                let AYA = if ARI != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ARJ = (TB * BG) / ((UH * AKV) * ARG);
                                                                    ARJ
                                                                };
                                                                AXZ = AYA;
                                                            } else {
                                                                AXZ = A;
                                                            }
                                                            AXX = AXZ;
                                                        }
                                                        AXW = AXX;
                                                    } else {
                                                        let ARK = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let AYB;
                                                        if ARK != 0.0 {
                                                            let ARL = if AKV == A { 1.0 } else { 0.0 };
                                                            let AYC = if ARL != 0.0 {
                                                                A
                                                            } else {
                                                                let ARM = (TB * SX) / (BG * AKV);
                                                                ARM
                                                            };
                                                            AYB = AYC;
                                                        } else {
                                                            let ARN = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AYD;
                                                            if ARN != 0.0 {
                                                                let ARO = SX + SY;
                                                                let ARP = if ARO == A { 1.0 } else { 0.0 };
                                                                if ARP != 0.0 {
                                                                } else {
                                                                }
                                                                let ARQ = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ARP != 0.0 { 1.0 } else { 0.0 };
                                                                let AYE = if ARQ != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ARR = (TB * BG) / ((UH * AKV) * ARO);
                                                                    ARR
                                                                };
                                                                AYD = AYE;
                                                            } else {
                                                                AYD = A;
                                                            }
                                                            AYB = AYD;
                                                        }
                                                        AXW = AYB;
                                                    }
                                                    AXV = AXW;
                                                }
                                                ATL = ATA;
                                                AXU = AXV;
                                            } else {
                                                let ARS = if TF == UP { 1.0 } else { 0.0 };
                                                let ATM;
                                                let AYF;
                                                if ARS != 0.0 {
                                                    let AYG;
                                                    if ART != 0.0 {
                                                        let ARU = if AJZ == A { 1.0 } else { 0.0 };
                                                        let AYH = if ARU != 0.0 {
                                                            A
                                                        } else {
                                                            let ARV = (TB * SZ) / (BG * AJZ);
                                                            ARV
                                                        };
                                                        AYG = AYH;
                                                    } else {
                                                        let AYI;
                                                        if ARW != 0.0 {
                                                            let ARX = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == AY { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UA { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AYJ;
                                                            if ARX != 0.0 {
                                                                let ARY = if AKV == A { 1.0 } else { 0.0 };
                                                                let AYK = if ARY != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ARZ = (TB * SX) / (BG * AKV);
                                                                    ARZ
                                                                };
                                                                AYJ = AYK;
                                                            } else {
                                                                let ASA = if (if (if TD == UH { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AYL;
                                                                if ASA != 0.0 {
                                                                    let ASB = if SX == A { 1.0 } else { 0.0 };
                                                                    if ASB != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let ASC = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ASB != 0.0 { 1.0 } else { 0.0 };
                                                                    let AYM = if ASC != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ASD = (TB * BG) / ((UJ * AKV) * SX);
                                                                        ASD
                                                                    };
                                                                    AYL = AYM;
                                                                } else {
                                                                    AYL = A;
                                                                }
                                                                AYJ = AYL;
                                                            }
                                                            AYI = AYJ;
                                                        } else {
                                                            let ASE = if (if (if TD == B { 1.0 } else { 0.0 }) != 0.0 || (if TD == UH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let AYN;
                                                            if ASE != 0.0 {
                                                                let ASF = if AKV == A { 1.0 } else { 0.0 };
                                                                let AYO = if ASF != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ASG = (TB * SX) / (BG * AKV);
                                                                    ASG
                                                                };
                                                                AYN = AYO;
                                                            } else {
                                                                let ASH = if (if (if TD == AY { 1.0 } else { 0.0 }) != 0.0 || (if TD == UI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if TD == UT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let AYP;
                                                                if ASH != 0.0 {
                                                                    let ASI = if SX == A { 1.0 } else { 0.0 };
                                                                    if ASI != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let ASJ = if (if AKV == A { 1.0 } else { 0.0 }) != 0.0 || ASI != 0.0 { 1.0 } else { 0.0 };
                                                                    let AYQ = if ASJ != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ASK = (TB * BG) / ((UJ * AKV) * SX);
                                                                        ASK
                                                                    };
                                                                    AYP = AYQ;
                                                                } else {
                                                                    AYP = A;
                                                                }
                                                                AYN = AYP;
                                                            }
                                                            AYI = AYN;
                                                        }
                                                        AYG = AYI;
                                                    }
                                                    ATM = ATA;
                                                    AYF = AYG;
                                                } else {
                                                    let ASL = if TF == UT { 1.0 } else { 0.0 };
                                                    let ATN;
                                                    let AYR;
                                                    if ASL != 0.0 {
                                                        let ASM = (TB * SZ) / BG;
                                                        ATN = ATA;
                                                        AYR = ASM;
                                                    } else {
                                                        let ASN = if TF == TG { 1.0 } else { 0.0 };
                                                        let ATO;
                                                        let AYS;
                                                        if ASN != 0.0 {
                                                            let ATP;
                                                            let AYT;
                                                            if ASO != 0.0 {
                                                                let ASP = ((NZ * TB) * SX) / BG;
                                                                let ASQ = if AA == AY { 1.0 } else { 0.0 };
                                                                let ATQ = if ASQ != 0.0 {
                                                                    A
                                                                } else {
                                                                    let ASR = (TB * SX) / (BG * (AA - AY));
                                                                    ASR
                                                                };
                                                                ATP = ATQ;
                                                                AYT = ASP;
                                                            } else {
                                                                let ASS = (TB * SX) / (BG * AA);
                                                                ATP = ASS;
                                                                AYT = A;
                                                            }
                                                            ATO = ATP;
                                                            AYS = AYT;
                                                        } else {
                                                            let AST = if TF == SS { 1.0 } else { 0.0 };
                                                            let ATR;
                                                            let AYU;
                                                            if AST != 0.0 {
                                                                let ATS;
                                                                let AYV;
                                                                if ASU != 0.0 {
                                                                    let ASV = (TB * SX) / (BG * AA);
                                                                    ATS = ASV;
                                                                    AYV = A;
                                                                } else {
                                                                    let ASW = ((NZ * TB) * SX) / BG;
                                                                    let ASX = if AA == AY { 1.0 } else { 0.0 };
                                                                    let ATT = if ASX != 0.0 {
                                                                        A
                                                                    } else {
                                                                        let ASY = (TB * SX) / (BG * (AA - AY));
                                                                        ASY
                                                                    };
                                                                    ATS = ATT;
                                                                    AYV = ASW;
                                                                }
                                                                ATR = ATS;
                                                                AYU = AYV;
                                                            } else {
                                                                ATR = A;
                                                                AYU = AYW;
                                                            }
                                                            ATO = ATR;
                                                            AYS = AYU;
                                                        }
                                                        ATN = ATO;
                                                        AYR = AYS;
                                                    }
                                                    ATM = ATN;
                                                    AYF = AYR;
                                                }
                                                ATL = ATM;
                                                AXU = AYF;
                                            }
                                            ATK = ATL;
                                            AXI = AXU;
                                        }
                                        ATJ = ATK;
                                        AWX = AXI;
                                    }
                                    ATI = ATJ;
                                    AWD = AWX;
                                }
                                ATH = ATI;
                                AVJ = AWD;
                            }
                            ATG = ATH;
                            AUP = AVJ;
                        }
                        ASZ = ATG;
                        ATV = AUP;
                    }
                    let ATU = if ASZ <= A { 1.0 } else { 0.0 };
                    let AZA;
                    if ATU != 0.0 {
                        AZA = ATV;
                    } else {
                        let AYY = if ATV <= A { 1.0 } else { 0.0 };
                        let AZB = if AYY != 0.0 {
                            ASZ
                        } else {
                            let AYZ = (ASZ * ATV) / (ASZ + ATV);
                            AYZ
                        };
                        AZA = AZB;
                    }
                    let AZC = if AZA == A { 1.0 } else { 0.0 };
                    if AZC != 0.0 {
                    } else {
                    }
                    AZJ = AZA;
                    BIA = AJZ;
                    BII = BIJ;
                    BIP = AKV;
                    BIX = BIY;
                } else {
                    AZJ = A;
                    BIA = AKC;
                    BII = BIF;
                    BIP = AKY;
                    BIX = BIU;
                }
                AZI = AZJ;
                BHZ = BIA;
                BIE = BII;
                BIO = BIP;
                BIT = BIX;
            }
            let AZD = if QR == A { 1.0 } else { 0.0 };
            let CIW;
            let CIZ;
            if AZD != 0.0 {
                let AZH = if AZE < AZG { 1.0 } else { 0.0 };
                let CJA = if AZH != 0.0 {
                    A
                } else {
                    AZE
                };
                let AZK = if AZI < AZG { 1.0 } else { 0.0 };
                let CIX = if AZK != 0.0 {
                    A
                } else {
                    AZI
                };
                CIW = CIX;
                CIZ = CJA;
            } else {
                let AZL = if AZE <= AZG { 1.0 } else { 0.0 };
                let CJB = if AZL != 0.0 {
                    AZG
                } else {
                    AZE
                };
                let AZM = if AZI <= AZG { 1.0 } else { 0.0 };
                let CIY = if AZM != 0.0 {
                    AZG
                } else {
                    AZI
                };
                CIW = CIY;
                CIZ = CJB;
            }
            let CJZ;
            let CKB;
            let DBW;
            let DBY;
            let DCD;
            let DCF;
            if QS != 0.0 {
                let AZN = if FS <= A { 1.0 } else { 0.0 };
                let DBX = if AZN != 0.0 {
                    A
                } else {
                    FS
                };
                let AZO = if FT <= A { 1.0 } else { 0.0 };
                let DCE = if AZO != 0.0 {
                    A
                } else {
                    FT
                };
                let AZQ = if AZP <= A { 1.0 } else { 0.0 };
                let DBZ = if AZQ != 0.0 {
                    A
                } else {
                    AZP
                };
                let AZS = if AZR <= A { 1.0 } else { 0.0 };
                let DCG = if AZS != 0.0 {
                    A
                } else {
                    AZR
                };
                CJZ = FV;
                CKB = AZU;
                DBW = DBX;
                DBY = DBZ;
                DCD = DCE;
                DCF = DCG;
            } else {
                let AZT = if FV <= A { 1.0 } else { 0.0 };
                let CKA = if AZT != 0.0 {
                    A
                } else {
                    FV
                };
                let AZV = if AZU <= A { 1.0 } else { 0.0 };
                let CKC = if AZV != 0.0 {
                    A
                } else {
                    AZU
                };
                CJZ = CKA;
                CKB = CKC;
                DBW = FS;
                DBY = AZP;
                DCD = FT;
                DCF = AZR;
            }
            let AZX = (parameters[900] * (parameters[21] + ((BV / UH) / AZW))) / ((AZW * AA) * (Y - parameters[899]));
            let AZY = if AZX > A { 1.0 } else { 0.0 };
            let DEA;
            if AZY != 0.0 {
                let AZZ = B / AZX;
                DEA = AZZ;
            } else {
                let BAC = if BAB != A { 1.0 } else { 0.0 };
                if BAC != 0.0 {
                } else {
                }
                DEA = BAA;
            }
            let BAD = M * JI;
            let BAG = (rspice_limited_exp((JL * ((if (BAE / M) >= BAF { (BAE / M) } else { BAF }).ln())))) / (M * M);
            let BAH = (rspice_limited_exp((JL * ((if (BAE / BAD) >= BAF { (BAE / BAD) } else { BAF }).ln())))) / (BAD * BAD);
            let BAK = if F != 0.0 {
                BAI
            } else {
                BAJ
            };
            let BAN = if F != 0.0 {
                BAL
            } else {
                BAM
            };
            let BAP = BG / BAO;
            let BAQ = (BAK * (BAP + parameters[1378])) * BAH;
            let BAR = (BAK * (BAP + parameters[1377])) * BAH;
            let BAS = (-BAN) * M;
            let BAT = BAS * JI;
            let BAU = parameters[1381] / AA;
            let BAV = (BAK * ((BAP * AZ) + BAU)) * BAG;
            let BAW = if (if parameters[41] != A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[1099] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BAX = if BAW != 0.0 && (if (parameters[1101] + BG) > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if BAX != 0.0 {
                let BAZ = if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 };
                if BAZ != 0.0 {
                    if B != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            if BAX != 0.0 {
            } else {
            }
            let BBB = if BBA <= -2.7315e2f64 { 1.0 } else { 0.0 };
            let BBO = if BBB != 0.0 {
                BBC
            } else {
                let BBD = BBA + 2.7315e2f64;
                BBD
            };
            let BBE = temperature + parameters[23];
            let BBI;
            if BAW != 0.0 {
                let BBF = if (if BAY != A { 1.0 } else { 0.0 }) != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 };
                let BBJ;
                if BBF != 0.0 {
                    let BBK = if B != 0.0 {
                        BBG
                    } else {
                        BBH
                    };
                    BBJ = BBK;
                } else {
                    BBJ = BBH;
                }
                BBI = BBJ;
            } else {
                BBI = A;
            }
            let BBL = BBI + BBE;
            let BBM = D * BBL;
            let BBN = B / BBM;
            let BBP = BBL / BBO;
            let BBQ = BBL - BBO;
            let BBR = parameters[108] - (((parameters[1029] * BBL) * BBL) / (BBL + parameters[1030]));
            let BBS = AY * BBM;
            let BBT = (parameters[107] * (BBP * (BBP.sqrt()))) * (rspice_limited_exp(((BBR / (AY * (D * BBO))) - (BBR / BBS))));
            let BCS = if BAX != 0.0 {
                let BBU = (if (LP / BBT) >= BAF { (LP / BBT) } else { BAF }).ln();
                let BBV = ((BBU * BBU) + BX).sqrt();
                BBV
            } else {
                let BBW = (if (LP / BBT) >= BAF { (LP / BBT) } else { BAF }).ln();
                BBW
            };
            let FAU = if BAX != 0.0 {
                let BBX = (if ((JY * DP) / (BBT * BBT)) >= BAF { ((JY * DP) / (BBT * BBT)) } else { BAF }).ln();
                let BBY = ((BBX * BBX) + BX).sqrt();
                BBY
            } else {
                let BBZ = (if ((JY * DP) / (BBT * BBT)) >= BAF { ((JY * DP) / (BBT * BBT)) } else { BAF }).ln();
                BBZ
            };
            let BCA = if DS > A { 1.0 } else { 0.0 };
            let DBT = if BCA != 0.0 {
                let BCC = (((-BCB) * BBM) * ((if (DS / DP) >= BAF { (DS / DP) } else { BAF }).ln())) + parameters[5];
                BCC
            } else {
                A
            };
            if BCD != 0.0 {
                let BCF = if BCE != A { 1.0 } else { 0.0 };
                if BCF != 0.0 {
                } else {
                }
                let BCH = if BCG != A { 1.0 } else { 0.0 };
                if BCH != 0.0 {
                } else {
                }
            } else {
                let BCI = if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 };
                if BCI != 0.0 {
                    let BCJ = if BCE != A { 1.0 } else { 0.0 };
                    if BCJ != 0.0 {
                    } else {
                    }
                    let BCK = if BCG != A { 1.0 } else { 0.0 };
                    if BCK != 0.0 {
                    } else {
                    }
                } else {
                    let BCL = if BCE == A { 1.0 } else { 0.0 };
                    if BCL != 0.0 {
                        let BCM = if BCG != A { 1.0 } else { 0.0 };
                        if BCM != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
            }
            let BCN = if BCE != A { 1.0 } else { 0.0 };
            let BCO = if BAY == B { 1.0 } else { 0.0 };
            let BCP = if (if BCN != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if BCO != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if BCP != 0.0 {
                let BCQ = if BCG == A { 1.0 } else { 0.0 };
                if BCQ != 0.0 {
                } else {
                }
            } else {
            }
            let BCT = if ((BCR + (BBM * BCS)) + FB) >= BCR { ((BCR + (BBM * BCS)) + FB) } else { BCR };
            let BCU = BCT.sqrt();
            let BCV = AY * J;
            let BCW = C * LP;
            let BCX = (BCV / BCW).sqrt();
            let BCY = (((J / L) * M) * FA).sqrt();
            let BCZ = BBP - B;
            let BDA = B + (parameters[1031] * BCZ);
            let BDC = LS * (NZ * (BDA + (((BDA * BDA) + 4e-6f64).sqrt())));
            let BDD = B + (parameters[1059] * BCZ);
            let BDE = NO * BDD;
            let BSW = if KY != 0.0 {
                let BDG = BDF * BDD;
                BDG
            } else {
                A
            };
            let BDH = if E != B { 1.0 } else { 0.0 };
            let BDM = if BDH != 0.0 {
                let BDK = BDI * BDJ;
                BDK
            } else {
                let BDL = NZ * BDJ;
                BDL
            };
            let BDO = BBP.powf(QM);
            let BDP = BDN * BDO;
            let BDR = (B + (QN * BBQ)) - BX;
            let BDS = BDR * BDR;
            let BDT = BDQ * (NZ * (BDR + ((BDS + 4e-6f64).sqrt())));
            let BDU = (B + (IG * BBQ)) - BX;
            let BDV = BDU * BDU;
            let BDW = NJ * (NZ * (BDU + ((BDV + 4e-6f64).sqrt())));
            let BDY = BBP.powf(QO);
            let BDZ = BDX * BDY;
            let BEB = BBP.powf(IJ);
            let BEC = BEA * BEB;
            let BEE = B + (II * BCZ);
            let BEF = BED * (NZ * (BEE + (((BEE * BEE) + 4e-6f64).sqrt())));
            let BTL;
            let BTN;
            let BTP;
            let BTR;
            let BTT;
            if KY != 0.0 {
                let BEL = BEG * BDO;
                let BEN = BEM * (NZ * (BDR + ((BDS + 4e-6f64).sqrt())));
                let BEP = BEO * (NZ * (BDU + ((BDV + 4e-6f64).sqrt())));
                let BER = BEQ * BDY;
                let BEV = BES * BEB;
                BTL = BEL;
                BTN = BEN;
                BTP = BEP;
                BTR = BER;
                BTT = BEV;
            } else {
                BTL = A;
                BTN = A;
                BTP = A;
                BTR = A;
                BTT = A;
            }
            let BEW = BBP.powf(IK);
            let BEX = BBP.powf((-QP));
            let BEY = OL * BEX;
            let BFA = if BEY < BEZ { 1.0 } else { 0.0 };
            let BPC = if BFA != 0.0 {
                BEZ
            } else {
                BEY
            };
            let BTF;
            if KY != 0.0 {
                let BFC = BFB * BEX;
                let BFD = if BFC < BEZ { 1.0 } else { 0.0 };
                let BTG = if BFD != 0.0 {
                    BEZ
                } else {
                    BFC
                };
                BTF = BTG;
            } else {
                BTF = A;
            }
            let BFE = PV * BEX;
            let BFF = if BFE < BEZ { 1.0 } else { 0.0 };
            let HHY = if BFF != 0.0 {
                BEZ
            } else {
                BFE
            };
            let BFG = ((B / OA) * (B + (parameters[1069] * BBQ))) - AY;
            let BFH = B / ((NZ * (BFG + (((BFG * BFG) + 4e-6f64).sqrt()))) + AY);
            let BFI = (B - (QQ * BBQ)) - BX;
            let BFJ = BFI * BFI;
            let BFK = OW * (NZ * (BFI + ((BFJ + 4e-6f64).sqrt())));
            let BTJ = if KY != 0.0 {
                let BFM = BFL * (NZ * (BFI + ((BFJ + 4e-6f64).sqrt())));
                BFM
            } else {
                A
            };
            let BFN = (B + (JQ * BBQ)) - BX;
            let BFO = JP * (NZ * (BFN + (((BFN * BFN) + 4e-6f64).sqrt())));
            let BFP = (B + (JS * BBQ)) - BX;
            let BFQ = JR * (NZ * (BFP + (((BFP * BFP) + 4e-6f64).sqrt())));
            let BFR = GO * (BBP.powf(IN));
            let BFS = HV + (HW * BCZ);
            let BFT = HZ + (IA * BCZ);
            let BFU = rspice_limited_exp((IO * ((if BBP >= BAF { BBP } else { BAF }).ln())));
            let BFV = (B + (JV * BBQ)) - BX;
            let BFW = JT * (NZ * (BFV + (((BFV * BFV) + 4e-6f64).sqrt())));
            let BFY = (B + (JW * BBQ)) - BX;
            let BFZ = BFX * (NZ * (BFY + (((BFY * BFY) + 4e-6f64).sqrt())));
            let BGA = (B + (KR * BBQ)) - BX;
            let BGB = KQ * (NZ * (BGA + (((BGA * BGA) + 4e-6f64).sqrt())));
            let BGC = (B + (KT * BBQ)) - BX;
            let BGD = KS * (NZ * (BGC + (((BGC * BGC) + 4e-6f64).sqrt())));
            let BGE = (B + (KV * BBQ)) - BX;
            let BGF = KU * (NZ * (BGE + (((BGE * BGE) + 4e-6f64).sqrt())));
            let BGG = (B + (parameters[1093] * BBQ)) - BX;
            let BGH = BGG * BGG;
            let BGI = parameters[901] * (NZ * (BGG + ((BGH + 4e-6f64).sqrt())));
            let BGJ = parameters[902] * (NZ * (BGG + ((BGH + 4e-6f64).sqrt())));
            let BGK = (B + (parameters[1094] * BBQ)) - BX;
            let BGL = BGK * BGK;
            let BGM = parameters[903] * (NZ * (BGK + ((BGL + 4e-6f64).sqrt())));
            let BGN = parameters[904] * (NZ * (BGK + ((BGL + 4e-6f64).sqrt())));
            let BGO = (B + (parameters[1095] * BBQ)) - BX;
            let BGP = BGO * BGO;
            let BGQ = parameters[905] * (NZ * (BGO + ((BGP + 4e-6f64).sqrt())));
            let BGR = parameters[906] * (NZ * (BGO + ((BGP + 4e-6f64).sqrt())));
            let BGS = parameters[1096] * BBQ;
            let BGU = (parameters[907] - BGS) - BGT;
            let BGV = (NZ * (BGU + (((BGU * BGU) + 4e-6f64).sqrt()))) + BGT;
            let BGW = (parameters[908] - BGS) - BGT;
            let BGX = (NZ * (BGW + (((BGW * BGW) + 4e-6f64).sqrt()))) + BGT;
            let BGY = parameters[1097] * BBQ;
            let BGZ = (parameters[909] - BGY) - BGT;
            let BHA = (NZ * (BGZ + (((BGZ * BGZ) + 4e-6f64).sqrt()))) + BGT;
            let BHB = (parameters[910] - BGY) - BGT;
            let BHC = (NZ * (BHB + (((BHB * BHB) + 4e-6f64).sqrt()))) + BGT;
            let BHD = parameters[1098] * BBQ;
            let BHE = (parameters[911] - BHD) - BGT;
            let BHF = (NZ * (BHE + (((BHE * BHE) + 4e-6f64).sqrt()))) + BGT;
            let BHG = (parameters[912] - BHD) - BGT;
            let BHH = (NZ * (BHG + (((BHG * BHG) + 4e-6f64).sqrt()))) + BGT;
            let BHI = if TF < TG { 1.0 } else { 0.0 };
            let BHW;
            let BIB;
            let BIL;
            let BIQ;
            if BHI != 0.0 {
                let BHJ = if (AA % AY) != A { 1.0 } else { 0.0 };
                let BHX;
                let BIC;
                let BIM;
                let BIR;
                if BHJ != 0.0 {
                    let BHK = AY * (if ((AA - B) / AY) >= A { ((AA - B) / AY) } else { A });
                    BHX = B;
                    BIC = BHK;
                    BIM = B;
                    BIR = BHK;
                } else {
                    let BHL = if TK == B { 1.0 } else { 0.0 };
                    let BHY;
                    let BID;
                    let BIN;
                    let BIS;
                    if BHL != 0.0 {
                        let BHM = AY * (if ((AA / AY) - B) >= A { ((AA / AY) - B) } else { A });
                        BHY = A;
                        BID = AA;
                        BIN = AY;
                        BIS = BHM;
                    } else {
                        let BHN = AY * (if ((AA / AY) - B) >= A { ((AA / AY) - B) } else { A });
                        BHY = AY;
                        BID = BHN;
                        BIN = A;
                        BIS = AA;
                    }
                    BHX = BHY;
                    BIC = BID;
                    BIM = BIN;
                    BIR = BIS;
                }
                BHW = BHX;
                BIB = BIC;
                BIL = BIM;
                BIQ = BIR;
            } else {
                BHW = BHZ;
                BIB = BIE;
                BIL = BIO;
                BIQ = BIT;
            }
            let BHO = SX + SY;
            let BHP = SX + SX;
            let BHQ = SZ + SZ;
            let BHR = (BHO + BHO) + BV;
            let BHS = BHO * BV;
            let BHT = SX * BV;
            let BHU = SZ * BV;
            let BHV = if TF == A { 1.0 } else { 0.0 };
            let BLK;
            let BLZ;
            let BMS;
            let BNJ;
            if BHV != 0.0 {
                let BIK = (BHW * BHR) + (BIB * BHP);
                let BIZ = (BIL * BHR) + (BIQ * BHP);
                let BJA = (BHW * BHS) + (BIB * BHT);
                let BJB = (BIL * BHS) + (BIQ * BHT);
                BLK = BJA;
                BLZ = BJB;
                BMS = BIK;
                BNJ = BIZ;
            } else {
                let BJC = if TF == B { 1.0 } else { 0.0 };
                let BLL;
                let BMA;
                let BMT;
                let BNK;
                if BJC != 0.0 {
                    let BJD = (BHW * BHR) + (BIB * BHP);
                    let BJE = BIL + BIQ;
                    let BJF = BJE * BHP;
                    let BJG = (BHW * BHS) + (BIB * BHT);
                    let BJH = BJE * BHT;
                    BLL = BJG;
                    BMA = BJH;
                    BMT = BJD;
                    BNK = BJF;
                } else {
                    let BJI = if TF == AY { 1.0 } else { 0.0 };
                    let BLM;
                    let BMB;
                    let BMU;
                    let BNL;
                    if BJI != 0.0 {
                        let BJJ = BHW + BIB;
                        let BJK = BJJ * BHP;
                        let BJL = (BIL * BHR) + (BIQ * BHP);
                        let BJM = BJJ * BHT;
                        let BJN = (BIL * BHS) + (BIQ * BHT);
                        BLM = BJM;
                        BMB = BJN;
                        BMU = BJK;
                        BNL = BJL;
                    } else {
                        let BJO = if TF == UH { 1.0 } else { 0.0 };
                        let BLN;
                        let BMC;
                        let BMV;
                        let BNM;
                        if BJO != 0.0 {
                            let BJP = BHW + BIB;
                            let BJQ = BJP * BHP;
                            let BJR = BIL + BIQ;
                            let BJS = BJR * BHP;
                            let BJT = BJP * BHT;
                            let BJU = BJR * BHT;
                            BLN = BJT;
                            BMC = BJU;
                            BMV = BJQ;
                            BNM = BJS;
                        } else {
                            let BJV = if TF == UI { 1.0 } else { 0.0 };
                            let BLO;
                            let BMD;
                            let BMW;
                            let BNN;
                            if BJV != 0.0 {
                                let BJW = (BHW * BHR) + (BIB * BHP);
                                let BJX = (BIL * BHQ) + (BIQ * BHP);
                                let BJY = (BHW * BHS) + (BIB * BHT);
                                let BJZ = (BIL * BHU) + (BIQ * BHT);
                                BLO = BJY;
                                BMD = BJZ;
                                BMW = BJW;
                                BNN = BJX;
                            } else {
                                let BKA = if TF == UA { 1.0 } else { 0.0 };
                                let BLP;
                                let BME;
                                let BMX;
                                let BNO;
                                if BKA != 0.0 {
                                    let BKB = BHW + BIB;
                                    let BKC = BKB * BHP;
                                    let BKD = (BIL * BHQ) + (BIQ * BHP);
                                    let BKE = BKB * BHT;
                                    let BKF = (BIL * BHU) + (BIQ * BHT);
                                    BLP = BKE;
                                    BME = BKF;
                                    BMX = BKC;
                                    BNO = BKD;
                                } else {
                                    let BKG = if TF == UJ { 1.0 } else { 0.0 };
                                    let BLQ;
                                    let BMF;
                                    let BMY;
                                    let BNP;
                                    if BKG != 0.0 {
                                        let BKH = (BHW * BHQ) + (BIB * BHP);
                                        let BKI = (BIL * BHR) + (BIQ * BHP);
                                        let BKJ = (BHW * BHU) + (BIB * BHT);
                                        let BKK = (BIL * BHS) + (BIQ * BHT);
                                        BLQ = BKJ;
                                        BMF = BKK;
                                        BMY = BKH;
                                        BNP = BKI;
                                    } else {
                                        let BKL = if TF == UP { 1.0 } else { 0.0 };
                                        let BLR;
                                        let BMG;
                                        let BMZ;
                                        let BNQ;
                                        if BKL != 0.0 {
                                            let BKM = (BHW * BHQ) + (BIB * BHP);
                                            let BKN = BIL + BIQ;
                                            let BKO = BKN * BHP;
                                            let BKP = (BHW * BHU) + (BIB * BHT);
                                            let BKQ = BKN * BHT;
                                            BLR = BKP;
                                            BMG = BKQ;
                                            BMZ = BKM;
                                            BNQ = BKO;
                                        } else {
                                            let BKR = if TF == UT { 1.0 } else { 0.0 };
                                            let BLS;
                                            let BMH;
                                            let BNA;
                                            let BNR;
                                            if BKR != 0.0 {
                                                let BKS = (BHW * BHQ) + (BIB * BHP);
                                                let BKT = (BIL * BHQ) + (BIQ * BHP);
                                                let BKU = (BHW * BHU) + (BIB * BHT);
                                                let BKV = (BIL * BHU) + (BIQ * BHT);
                                                BLS = BKU;
                                                BMH = BKV;
                                                BNA = BKS;
                                                BNR = BKT;
                                            } else {
                                                let BKW = if TF == TG { 1.0 } else { 0.0 };
                                                let BLT;
                                                let BMI;
                                                let BNB;
                                                let BNS;
                                                if BKW != 0.0 {
                                                    let BKX = AA - B;
                                                    let BKY = BHR + (BKX * BHP);
                                                    let BKZ = AA * BHP;
                                                    let BLA = BHS + (BKX * BHT);
                                                    let BLB = AA * BHT;
                                                    BLT = BLA;
                                                    BMI = BLB;
                                                    BNB = BKY;
                                                    BNS = BKZ;
                                                } else {
                                                    let BLC = if TF == SS { 1.0 } else { 0.0 };
                                                    let BLU;
                                                    let BMJ;
                                                    let BNC;
                                                    let BNT;
                                                    if BLC != 0.0 {
                                                        let BLD = AA * BHP;
                                                        let BLE = AA - B;
                                                        let BLF = BHR + (BLE * BHP);
                                                        let BLG = AA * BHT;
                                                        let BLH = BHS + (BLE * BHT);
                                                        BLU = BLG;
                                                        BMJ = BLH;
                                                        BNC = BLD;
                                                        BNT = BLF;
                                                    } else {
                                                        BLU = A;
                                                        BMJ = A;
                                                        BNC = A;
                                                        BNT = A;
                                                    }
                                                    BLT = BLU;
                                                    BMI = BMJ;
                                                    BNB = BNC;
                                                    BNS = BNT;
                                                }
                                                BLS = BLT;
                                                BMH = BMI;
                                                BNA = BNB;
                                                BNR = BNS;
                                            }
                                            BLR = BLS;
                                            BMG = BMH;
                                            BMZ = BNA;
                                            BNQ = BNR;
                                        }
                                        BLQ = BLR;
                                        BMF = BMG;
                                        BMY = BMZ;
                                        BNP = BNQ;
                                    }
                                    BLP = BLQ;
                                    BME = BMF;
                                    BMX = BMY;
                                    BNO = BNP;
                                }
                                BLO = BLP;
                                BMD = BME;
                                BMW = BMX;
                                BNN = BNO;
                            }
                            BLN = BLO;
                            BMC = BMD;
                            BMV = BMW;
                            BNM = BNN;
                        }
                        BLM = BLN;
                        BMB = BMC;
                        BMU = BMV;
                        BNL = BNM;
                    }
                    BLL = BLM;
                    BMA = BMB;
                    BMT = BMU;
                    BNK = BNL;
                }
                BLK = BLL;
                BLZ = BMA;
                BMS = BMT;
                BNJ = BNK;
            }
            let BLV = if BLI != 0.0 {
                let BLJ = (parameters[17] * W) * T;
                BLJ
            } else {
                BLK
            };
            let BLW = if BLV < A { 1.0 } else { 0.0 };
            let DGZ = if BLW != 0.0 {
                A
            } else {
                BLV
            };
            let BMK = if BLX != 0.0 {
                let BLY = (parameters[18] * W) * T;
                BLY
            } else {
                BLZ
            };
            let BML = if BMK < A { 1.0 } else { 0.0 };
            let DKB = if BML != 0.0 {
                A
            } else {
                BMK
            };
            let DHB;
            if BMM != 0.0 {
                let BMO = if BMN == A { 1.0 } else { 0.0 };
                let DHC = if BMO != 0.0 {
                    let BMQ = BMP * W;
                    BMQ
                } else {
                    let BMR = if ((BMP * W) - (BV * AA)) >= A { ((BMP * W) - (BV * AA)) } else { A };
                    BMR
                };
                DHB = DHC;
            } else {
                let BND = if BMS < A { 1.0 } else { 0.0 };
                let DHD = if BND != 0.0 {
                    A
                } else {
                    BMS
                };
                DHB = DHD;
            }
            let DKD;
            if BNE != 0.0 {
                let BNF = if BMN == A { 1.0 } else { 0.0 };
                let DKE = if BNF != 0.0 {
                    let BNH = BNG * W;
                    BNH
                } else {
                    let BNI = if ((BNG * W) - (BV * AA)) >= A { ((BNG * W) - (BV * AA)) } else { A };
                    BNI
                };
                DKD = DKE;
            } else {
                let BNU = if BNJ < A { 1.0 } else { 0.0 };
                let DKF = if BNU != 0.0 {
                    A
                } else {
                    BNJ
                };
                DKD = DKF;
            }
            let BNY = if (if (if BNV > A { 1.0 } else { 0.0 }) != 0.0 && (if BNW > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if AA == B { 1.0 } else { 0.0 }) != 0.0 || (if (if AA > B { 1.0 } else { 0.0 }) != 0.0 && (if BNX > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BQU;
            let BQW;
            let BQY;
            let BSX;
            let BTH;
            let BVL;
            let EZO;
            let FBE;
            if BNY != 0.0 {
                let BNZ = Y.powf(parameters[1111]);
                let BOA = AB + parameters[1104];
                let BOB = BOA.powf(parameters[1112]);
                let BOC = Y.powf(parameters[1117]);
                let BOD = BOA.powf(parameters[1118]);
                let BOE = B + (((parameters[1114] / BOC) + (parameters[1115] / BOD)) + (parameters[1116] / (BOC * BOD)));
                let BOF = ((B + (((parameters[1108] / BNZ) + (parameters[1109] / BOB)) + (parameters[1110] / (BNZ * BOB)))) * (B + (parameters[1107] * BCZ))) + BB;
                let mut BOG = 0.0;
                let mut BOL = 0.0;
                let mut BON = 0.0;
                BOG = A;
                BOL = A;
                BON = A;
                loop {
                    let BOH = if BOG < AA { 1.0 } else { 0.0 };
                    if BOH == 0.0 {
                        break;
                    }
                    let BOI = B / AA;
                    let BOJ = NZ * U;
                    let BOK = BOG * (BNX + U);
                    let BOM = BOL + (BOI / ((BNV + BOJ) + BOK));
                    let BOO = BON + (BOI / ((BNW + BOJ) + BOK));
                    let BOP = BOG + B;
                    BOG = BOP;
                    BOL = BOM;
                    BON = BOO;
                }
                let BOQ = NZ * U;
                let BOR = (B / (parameters[1102] + BOQ)) + (B / (parameters[1103] + BOQ));
                let BOS = parameters[1105] / BOF;
                let BOT = BOS * BOR;
                let BOU = BOL + BON;
                let BOV = BOS * BOU;
                let BOX = BOU - BOR;
                let BOY = (parameters[1113] / BOE) * BOX;
                let BOZ = BOE.powf(parameters[1120]);
                let BPA = BOE.powf(parameters[1122]);
                let BPB = BDP * ((B + BOV) / (B + BOT));
                let BPD = BPC * ((B + (BOV * BOW)) / (B + (BOT * BOW)));
                let BPE = QG + ((parameters[1119] / BOZ) * BOX);
                let BPF = BDE + ((parameters[1121] / BPA) * BOX);
                let BPH = if BPG == B { 1.0 } else { 0.0 };
                let BPL;
                let BPN;
                let FBF;
                if BPH != 0.0 {
                    let BPI = (KL / BOE) * BOX;
                    let BPJ = (KO / BOZ) * BOX;
                    let BPK = (KP / BPA) * BOX;
                    BPL = BPJ;
                    BPN = BPK;
                    FBF = BPI;
                } else {
                    BPL = A;
                    BPN = A;
                    FBF = A;
                }
                let BPM = QJ + BPL;
                let BPO = KC + BPN;
                BQU = BPB;
                BQW = BPE;
                BQY = BPM;
                BSX = BPF;
                BTH = BPD;
                BVL = BOY;
                EZO = BPO;
                FBE = FBF;
            } else {
                BQU = BDP;
                BQW = QG;
                BQY = QJ;
                BSX = BDE;
                BTH = BPC;
                BVL = A;
                EZO = KC;
                FBE = A;
            }
            let BPP = if parameters[34] == B { 1.0 } else { 0.0 };
            let BQI;
            let BQL;
            let BQO;
            if BPP != 0.0 {
                let BPQ = V / AA;
                let BPU = if (if (if (if parameter_given[13] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[14] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[15] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BQJ;
                let BQM;
                let BQP;
                if BPU != 0.0 {
                    let BPW = if (if parameter_given[16] { 1.0 } else { 0.0 }) != 0.0 && (if BPV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BQK;
                    let BQN;
                    let BQQ;
                    if BPW != 0.0 {
                        let BPX = BPV + BPQ;
                        let BPZ = B / BPY;
                        let BQA = (BPY * BPY) / (BPV * BPX);
                        let BQC = BGT * BPY;
                        let BQD = ((((BQB * BPV) + BQC) * (rspice_limited_exp(((-1e1f64 * BPV) * BPZ)))) - (((BQB * BPX) + BQC) * (rspice_limited_exp(((-1e1f64 * BPX) * BPZ))))) / BPQ;
                        let BQF = 2.5e-3f64 * BPY;
                        let BQH = ((((BQE * BPV) + BQF) * (rspice_limited_exp(((-2e1f64 * BPV) * BPZ)))) - (((BQE * BPX) + BQF) * (rspice_limited_exp(((-2e1f64 * BPX) * BPZ))))) / BPQ;
                        BQK = BQA;
                        BQN = BQD;
                        BQQ = BQH;
                    } else {
                        BQK = BPR;
                        BQN = BPS;
                        BQQ = BPT;
                    }
                    BQJ = BQK;
                    BQM = BQN;
                    BQP = BQQ;
                } else {
                    BQJ = BPR;
                    BQM = BPS;
                    BQP = BPT;
                }
                BQI = BQJ;
                BQL = BQM;
                BQO = BQP;
            } else {
                BQI = A;
                BQL = A;
                BQO = A;
            }
            let BQR = (BQI + (parameters[1123] * BQL)) + (parameters[1124] * BQO);
            let BQS = HR * BQR;
            let BQT = KN * BQR;
            let BQV = BQU * (B + (HT * BQR));
            let BQX = BQW + (HS * BQR);
            let BQZ = BQY + (KM * BQR);
            let BRC = BCB * (BRA - BRB);
            let BRE = BCB * (BRA - BRD);
            let BRG = BRF - BRB;
            let BRH = BCB * BRG;
            let BRJ = BRI - BRB;
            let BRK = BCB * BRJ;
            let BRL = BCB * (BRI - BRD);
            let BRM = BRH - BRK;
            let BRN = BCB * (BRB - BRI);
            let BRO = BCB * (BRB - BRF);
            let BRP = BRC - BRH;
            let BRQ = BRC - BRK;
            let BRS = BCB * (BRR - BRB);
            let BRT = BCB * (BRR - BRD);
            let BRU = BRS - BRK;
            let BRV = BRS - BRH;
            let BRW = -BCB;
            let BRX = BRW * BRJ;
            let BRY = if BRM < A { 1.0 } else { 0.0 };
            let BSC;
            let BSD;
            let BSP;
            let DGQ;
            let DOU;
            let DOZ;
            if BRY != 0.0 {
                let BSA = BCB * (BRF - BRD);
                let BSB = BRW * BRG;
                BSC = BRK;
                BSD = BRH;
                BSP = BSA;
                DGQ = BRZ;
                DOU = BRV;
                DOZ = BSB;
            } else {
                BSC = BRH;
                BSD = BRK;
                BSP = BRL;
                DGQ = B;
                DOU = BRU;
                DOZ = BRX;
            }
            let BSE = BSC - BSD;
            let BSG = BSF * BSE;
            let BSI = if BSG > BSH { 1.0 } else { 0.0 };
            let BSL = if BSI != 0.0 {
                BSG
            } else {
                let BSJ = (B + (rspice_limited_exp(BSG))).ln();
                BSJ
            };
            let BSK = AY / BSF;
            let BSM = ((BSK * BSL) - BSE) - (BSK * 6.931471805599453e-1f64);
            let BSN = NZ * (BSE - BSM);
            let BSO = -(BSD + BSN);
            let BSQ = -(BSP + BSN);
            let BSR = BRS + (NZ * (BSM - BSE));
            let BSS = NZ + (NZ * (((6e-1f64 * BRM) / BBM).tanh()));
            let BST = B - BSS;
            let BUL;
            let BUV;
            let CJO;
            let CJP;
            let CJQ;
            let CJS;
            let CKH;
            let CKJ;
            let CZH;
            let CZR;
            let EZT;
            let GET;
            let GFB;
            if KY != 0.0 {
                let BSU = (SB * BST) + (KA * BSS);
                let BSV = (RZ * BST) + (LV * BSS);
                let BSY = (BSW * BST) + (BSX * BSS);
                let BTA = (BSZ * BST) + (NV * BSS);
                let BTC = (BTB * BST) + (OF * BSS);
                let BTE = (BTD * BST) + (OR * BSS);
                let BTI = (BTF * BST) + (BTH * BSS);
                let BTK = (BTJ * BST) + (BFK * BSS);
                let BTM = (BTL * BST) + (BQV * BSS);
                let BTO = (BTN * BST) + (BDT * BSS);
                let BTQ = (BTP * BST) + (BDW * BSS);
                let BTS = (BTR * BST) + (BDZ * BSS);
                let BTU = (BTT * BST) + (BEC * BSS);
                BUL = BSV;
                BUV = BSY;
                CJO = BTU;
                CJP = BTO;
                CJQ = BTQ;
                CJS = BTS;
                CKH = BTI;
                CKJ = BTM;
                CZH = BTA;
                CZR = BTC;
                EZT = BSU;
                GET = BTE;
                GFB = BTK;
            } else {
                BUL = LV;
                BUV = BSX;
                CJO = BEC;
                CJP = BDT;
                CJQ = BDW;
                CJS = BDZ;
                CKH = BTH;
                CKJ = BQV;
                CZH = NV;
                CZR = OF;
                EZT = KA;
                GET = OR;
                GFB = BFK;
            }
            let BTW = O * M;
            let BTX = (BTV * (BTW + (3.75e-1f64 * BTV))).sqrt();
            let BTZ = ((EC * AZ) / (BTX + (((((DV + (DW * ((((BRC - PP) * (BTY * O)) + ((BRS - DN) * (BTW + BTV))) / (BTV + (O * (M + BTY)))))).atan()) / 3.141592653589793e0f64) + NZ) * ((((O * BTV) * M).sqrt()) - BTX)))) + BX;
            let BUB = if BTZ < BUA { 1.0 } else { 0.0 };
            let BUN = if BUB != 0.0 {
                let BUC = NZ / ((BTZ.cosh()) - B);
                BUC
            } else {
                let BUD = rspice_limited_exp((-BTZ));
                BUD
            };
            let BUE = J / BTV;
            let BUF = L / BTY;
            let BUG = BCT - BSO;
            let BUH = BUG - BQE;
            let BUI = (NZ * ((BUG + BQE) + (((BUH * BUH) + 2.5000000000000005e-3f64).sqrt()))).sqrt();
            let BUJ = BCX * BUI;
            let BUK = J / BUJ;
            let BUM = B + ((((DT + BDC) + (BUL * BSM)) - (MB * BSO)) / N);
            let BUQ = if PM != 0.0 {
                let BUO = N + ((BUE * BUF) / (BUE + BUF));
                let BUP = (((BUO + DT) + BDC) + (((((parameters[268] * BSR) + ((parameters[269] * BSR) * BSR)) - (parameters[280] * BSO)) - ((parameters[281] * BSO) * BSO)) + (BUN * (((((EE + (EI * BSR)) + ((parameters[274] * BSR) * BSR)) + (EK * BSO)) + ((parameters[279] * BSO) * BSO)) + (((BUL + (parameters[266] * BSR)) - (parameters[267] * BSO)) * BSM))))) / BUO;
                BUP
            } else {
                BUM
            };
            let BUR = BUQ - B;
            let BUS = NZ * ((BUQ + B) + (((BUR * BUR) + 6.250000000000001e-4f64).sqrt()));
            let BUT = BUS * BBM;
            let BUU = B / BUT;
            let BUW = (-(BUV + (NS * BSO))) * BSM;
            let BUX = (NZ * (BUW - (((BUW * BUW) + 6.25e-10f64).sqrt()))) + 1.25e-5f64;
            let BUY = ((JM + (parameters[1077] / AZ)) + (JN * BSO)) * ((BBP.powf(parameters[1076])) - B);
            let BUZ = if EL > A { 1.0 } else { 0.0 };
            let BVG;
            if BUZ != 0.0 {
                let BVA = (-EN) * BSM;
                let BVB = if BVA < -8e1f64 { 1.0 } else { 0.0 };
                let BVE = if BVB != 0.0 {
                    BVC
                } else {
                    let BVD = rspice_limited_exp(BVA);
                    BVD
                };
                let BVF = (-BUT) * ((if (AZ / (AZ + (EL * (B + BVE)))) >= BAF { (AZ / (AZ + (EL * (B + BVE)))) } else { BAF }).ln());
                BVG = BVF;
            } else {
                BVG = A;
            }
            let BVI = PP + BVH;
            let BVJ = BSD * BUU;
            let BVK = BUI - BCU;
            let BVM = ((BRC * BUU) - (BVI * BUU)) - ((((((BUX + (BVG - ((ES + (EP / (AZ.powf(EQ)))) * ((ER * BSM).tanh())))) + ((QA * BVK) - (BQX * BSO))) - BUY) + BVL) + BQS) * BUU);
            let BVN = (BRS * BUU) - (DN * BUU);
            let BVO = ((((3.204352924e-19f64 * J) * LP) * BBN).sqrt()) / N;
            let BVP = AY * BCS;
            let BVQ = BSD * BBN;
            let BVR = BVP + BVQ;
            let BVS = B + (BVO / (AY * ((NZ * (BVR + (((BVR * BVR) + 4e-6f64).sqrt()))).sqrt())));
            let BVT = AY * BVS;
            let BVU = B / BVO;
            let HWE;
            let HWF;
            let HWH;
            let HWL;
            let HWQ;
            let HWW;
            let HXB;
            let HXP;
            let HYN;
            let HYU;
            let HYV;
            let HYW;
            let HYX;
            let HYZ;
            let HZB;
            let HZE;
            let HZH;
            let HZK;
            let HZN;
            let HZQ;
            let HZT;
            let HZV;
            let HZX;
            let HZZ;
            let IAB;
            let IAD;
            let IAF;
            let IAH;
            let IAJ;
            let IAL;
            let IAN;
            let IAP;
            let IAR;
            let IAT;
            let IAV;
            let IAY;
            let IBB;
            let IBE;
            let IBH;
            let IBK;
            let IBN;
            let IBP;
            let IBR;
            let IBT;
            let IBV;
            let IBX;
            let IBZ;
            let ICB;
            let ICD;
            if PM != 0.0 {
                let BVV = ((((3.204352924e-19f64 * J) * LP) * BUU).sqrt()) / N;
                let BVW = B / BVV;
                let BVX = BVV * BVV;
                let BVY = B / BVX;
                let BVZ = (BUF + EA) / BUE;
                let BWA = M / BTY;
                let BWB = BVV / BWA;
                let BWD = B + (BWB * BWC);
                let BWF = BWE * BWD;
                let BWH = B / BWB;
                let BWI = BWB * BWB;
                let BWK = B / (BWG + (BWB * BWJ));
                let BWL = BVN.abs();
                let BWM = if BWL <= BWF { 1.0 } else { 0.0 };
                let BXT;
                if BWM != 0.0 {
                    let BWN = -BVN;
                    let BWO = (BWN * BWH) * (B + (BWB * (BWN / ((8.485281374238571e0f64 * BWD) * BWD))));
                    BXT = BWO;
                } else {
                    let BWP = if BVN < (-BWF) { 1.0 } else { 0.0 };
                    let BXU = if BWP != 0.0 {
                        let BWQ = -BVN;
                        let BWR = (BWG * BWQ) * BWH;
                        let BWS = BWR - UJ;
                        let BWU = NZ * ((BWR + SS) - (((BWS * BWS) + BWT).sqrt()));
                        let BWV = BWQ - BWU;
                        let BWW = (BWV * BWV) + (BWI * (BWU + B));
                        let BWX = (AY * BWV) - BWI;
                        let BWY = ((if (BWW / BWI) >= BAF { (BWW / BWI) } else { BAF }).ln()) - BWU;
                        let BWZ = BWW + BWX;
                        let BXA = BWX * BWX;
                        let BXB = (BWZ * BWZ) + (BWY * ((NZ * BXA) - BWW));
                        let BXC = BWU + (((BWW * BWZ) * BWY) / (BXB + (((((BWZ / BXB) * BWY) * BWY) * BWX) * ((BXA * BDI) - BWW))));
                        let BXD = rspice_limited_exp(BXC);
                        let BXE = BWQ - BXC;
                        let BXF = (AY * BXE) + (BWI * (BXD - B));
                        let BXG = (BXE * BXE) + (BWI * ((BXC + B) - BXD));
                        let BXH = -(BXC + (AY * (BXG / (BXF + (((BXF * BXF) - (UI * ((B - ((BWI * NZ) * BXD)) * BXG))).sqrt())))));
                        BXH
                    } else {
                        let BXI = BWI * NZ;
                        let BXJ = (BVN + BXI) - (BWB * (((BVN + (BWI * OQ)) - (B - (rspice_limited_exp((-((BVN * BWH) * (B + (((((BWD * BWG) * BWK) - B) * BWK) * BVN)))))))).sqrt()));
                        let BXK = rspice_limited_exp((-BXJ));
                        let BXL = BVN - BXJ;
                        let BXM = (AY * BXL) + (BWI * (B - BXK));
                        let BXN = (BXL * BXL) - (BWI * ((BXJ - B) + BXK));
                        let BXO = BXJ + (AY * (BXN / (BXM + (((BXM * BXM) - (UI * ((B - (BXI * BXK)) * BXN))).sqrt()))));
                        BXO
                    };
                    BXT = BXU;
                }
                let BXP = if BWL < BWF { 1.0 } else { 0.0 };
                let BXY = if BXP != 0.0 {
                    let BXQ = -BVN;
                    let BXR = (BXQ * BWH) * (B + (BWB * (BXQ / ((8.485281374238571e0f64 * BWD) * BWD))));
                    BXR
                } else {
                    let BXS = BWA * BWA;
                    let BXV = BVN - BXT;
                    let BXW = rspice_limited_exp((-BXT));
                    let BXX = BXT - ((((((BXS * BXV) * BXV) * BVW) * BVW) - ((BXW + BXT) - B)) / ((BXW + ((BXS * ((AY * BXT) - (AY * BVN))) / BVX)) - B));
                    BXX
                };
                let BXZ = BXY * BUT;
                let BYA = B + (BVV * BWC);
                let BYB = B / BYA;
                let BYC = BVP / BUS;
                let BYD = BYC + BVJ;
                let BYE = rspice_limited_exp((-BYD));
                let BYF = BDB * BYA;
                let BYG = ((-DY) * AZ) / BCY;
                let BYH = DX * ((rspice_limited_exp((NZ * BYG))) + (AY * (rspice_limited_exp(BYG))));
                let BYI = B + (GI / AZ);
                let BYK = ((BYI * ((BCW * BTV) * BTV)) / (BCV * BUT)) + (BYJ / BUT);
                let BYL = BVZ * BVN;
                let BYM = BYK - BYL;
                let BYN = BYM + (BVV * ((((rspice_limited_exp((-BYM))) + BYM) - B).sqrt()));
                let BYO = if BYM < BYD { 1.0 } else { 0.0 };
                let CDW;
                if BYO != 0.0 {
                    let BYP = if BVM < BYN { 1.0 } else { 0.0 };
                    let CDX;
                    if BYP != 0.0 {
                        let BYQ = if (BVM.abs()) <= BYF { 1.0 } else { 0.0 };
                        let CDY;
                        if BYQ != 0.0 {
                            let BYS = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * (((BYB * BYB) * BYR) * BWC)));
                            CDY = BYS;
                        } else {
                            let BYT = if BVM < (-BYF) { 1.0 } else { 0.0 };
                            let CDZ = if BYT != 0.0 {
                                let BYU = -BVM;
                                let BYW = BYV * (BYU * BYB);
                                let BYX = BYW - UJ;
                                let BYY = NZ * ((BYW + SS) - (((BYX * BYX) + BWT).sqrt()));
                                let BYZ = BYU - BYY;
                                let BZA = (BYZ * BYZ) + (BVX * (BYY + B));
                                let BZB = (AY * BYZ) - BVX;
                                let BZC = (-BYY) + ((if (BZA * BVY) >= BAF { (BZA * BVY) } else { BAF }).ln());
                                let BZD = BZA + BZB;
                                let BZE = BZB * BZB;
                                let BZF = (BZD * BZD) + (BZC * ((NZ * BZE) - BZA));
                                let BZG = BYY + (((BZA * BZD) * BZC) / (BZF + (((((BZD / BZF) * BZC) * BZC) * BZB) * ((BZE * BDI) - BZA))));
                                let BZH = rspice_limited_exp(BZG);
                                let BZI = BZG * BZG;
                                let BZJ = B / (AY + BZI);
                                let BZK = BZI * BZJ;
                                let BZM = BYU - BZG;
                                let BZN = BYE * (B / BZH);
                                let BZO = (AY * BZM) + (BVX * (((BZH - B) - BZN) + (BYE * (B - (UI * ((BZG * BZJ) * BZJ))))));
                                let BZP = (BZM * BZM) - (BVX * ((((BZH - BZG) - B) + BZN) + (BYE * ((BZG - B) - BZK))));
                                let BZQ = (-BZG) - (AY * (BZP / (BZO + (((BZO * BZO) - (AY * (BZP * (AY - (BVX * ((BZH + BZN) - (BYE * ((((UT * BZJ) - (BZL * BZK)) * BZJ) * BZJ)))))))).sqrt()))));
                                BZQ
                            } else {
                                let BZR = B / (BYV + (BVV * BWJ));
                                let BZS = (BVM + (BVX * NZ)) - (BVV * (((BVM + (BVX * OQ)) - (B - (rspice_limited_exp((-((BVM * BYB) * (B + (((((BYA * BYV) * BZR) - B) * BZR) * BVM)))))))).sqrt()));
                                let BZT = BYD + UH;
                                let BZU = BZS - BZT;
                                let BZV = (NZ * ((BZS + BZT) - (((BZU * BZU) + UA).sqrt()))) - (NZ * (BZT - (((BZT * BZT) + UA).sqrt())));
                                let BZW = BVM - BZV;
                                let BZX = rspice_limited_exp((-BZV));
                                let BZY = BZV * BZV;
                                let BZZ = B / (AY + BZY);
                                let CAA = BZY * BZZ;
                                let CAC = if CAB >= ((BZW * BZW) - (BVX * (((BZX + BZV) - B) - (BYE * ((BZV + B) + CAA))))) { CAB } else { ((BZW * BZW) - (BVX * (((BZX + BZV) - B) - (BYE * ((BZV + B) + CAA))))) };
                                let CAD = (AY * BZW) + (BVX * ((B - BZX) - (BYE * (B + (UI * ((BZV * BZZ) * BZZ))))));
                                let CAE = (BYD - BZV) + ((if (CAC / BVX) >= BAF { (CAC / BVX) } else { BAF }).ln());
                                let CAF = CAC + CAD;
                                let CAG = CAD * CAD;
                                let CAH = CAC * (B - (NZ * (BVX * (BZX - (BYE * ((((UT * BZZ) - (BZL * CAA)) * BZZ) * BZZ))))));
                                let CAI = (CAF * CAF) + (CAE * ((NZ * CAG) - CAH));
                                let CAJ = BZV + (((CAC * CAF) * CAE) / (CAI + (((((CAF / CAI) * CAE) * CAE) * CAD) * ((CAG * BDI) - CAH))));
                                let CAK = B / (rspice_limited_exp(CAJ));
                                let CAL = rspice_limited_exp((CAJ - BYD));
                                let CAM = CAJ * CAJ;
                                let CAN = B / (AY + CAM);
                                let CAO = CAM * CAN;
                                let CAP = BVM - CAJ;
                                let CAQ = (AY * CAP) + (BVX * (((B - CAK) + CAL) - (BYE * (B + (UI * ((CAJ * CAN) * CAN))))));
                                let CAR = (CAP * CAP) - (BVX * ((((CAK + CAJ) - B) + CAL) - (BYE * ((CAJ + B) + CAO))));
                                let CAS = CAJ + (AY * (CAR / (CAQ + (((CAQ * CAQ) - (AY * (CAR * (AY - (BVX * ((CAK + CAL) - (BYE * ((((UT * CAN) - (BZL * CAO)) * CAN) * CAN)))))))).sqrt()))));
                                CAS
                            };
                            CDY = CDZ;
                        }
                        CDX = CDY;
                    } else {
                        let CAT = BWA * BWA;
                        let CAU = BYM - (BXZ * BUU);
                        let CAV = BVM - (BVV * ((((rspice_limited_exp((-CAU))) + CAU) - B).sqrt()));
                        let CAW = BYD + UH;
                        let CAX = CAV - CAW;
                        let CAY = NZ * ((CAV + CAW) - (((CAX * CAX) + BUA).sqrt()));
                        let CAZ = BVM - CAY;
                        let CBA = (BVN - CAY) + BYM;
                        let CBB = ((CAZ * CAZ) - ((CAT * CBA) * CBA)) - (BVX * BYM);
                        let CBC = AY * CAT;
                        let CBD = (AY * CAZ) - (CBC * CBA);
                        let CBE = CBD * CBD;
                        let CBF = B - CAT;
                        let CBG = if CBB < A { 1.0 } else { 0.0 };
                        let CBH = if CBG != 0.0 {
                            A
                        } else {
                            CBB
                        };
                        let CBI = CBH + CBD;
                        let CBJ = CBH * CBF;
                        let CBK = (((CBI * CBI) / ((BYD - CAY) + ((if (CBH * BVY) >= BAF { (CBH * BVY) } else { BAF }).ln()))) + (NZ * CBE)) - CBJ;
                        let CBL = CAY + ((CBI * CBH) / (CBK + (((CBD * CBI) / CBK) * ((BDI * CBE) - CBJ))));
                        let CBM = rspice_limited_exp((CBL - BYD));
                        let CBN = BVM - CBL;
                        let CBO = (BVN - CBL) + BYM;
                        let CBP = BVX * CBM;
                        let CBQ = ((AY * CBN) - (CBC * CBO)) + CBP;
                        let CBR = AY * (((CBN * CBN) - ((CAT * CBO) * CBO)) - (BVX * (BYM + CBM)));
                        let CBS = CBL + (CBR / (CBQ + (((CBQ * CBQ) - (CBR * ((AY - CBC) - CBP))).sqrt())));
                        CDX = CBS;
                    }
                    CDW = CDX;
                } else {
                    let CBT = if (BVM.abs()) <= BYF { 1.0 } else { 0.0 };
                    let CEA;
                    if CBT != 0.0 {
                        let CBU = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * (((BYB * BYB) * BYR) * BWC)));
                        CEA = CBU;
                    } else {
                        let CBV = if BVM < (-BYF) { 1.0 } else { 0.0 };
                        let CEB = if CBV != 0.0 {
                            let CBW = -BVM;
                            let CBX = BYV * (CBW * BYB);
                            let CBY = CBX - UJ;
                            let CBZ = NZ * ((CBX + SS) - (((CBY * CBY) + BWT).sqrt()));
                            let CCA = CBW - CBZ;
                            let CCB = (CCA * CCA) + (BVX * (CBZ + B));
                            let CCC = (AY * CCA) - BVX;
                            let CCD = (-CBZ) + ((if (CCB * BVY) >= BAF { (CCB * BVY) } else { BAF }).ln());
                            let CCE = CCB + CCC;
                            let CCF = CCC * CCC;
                            let CCG = (CCE * CCE) + (CCD * ((NZ * CCF) - CCB));
                            let CCH = CBZ + (((CCB * CCE) * CCD) / (CCG + (((((CCE / CCG) * CCD) * CCD) * CCC) * ((CCF * BDI) - CCB))));
                            let CCI = rspice_limited_exp(CCH);
                            let CCJ = CCH * CCH;
                            let CCK = B / (AY + CCJ);
                            let CCL = CCJ * CCK;
                            let CCM = CBW - CCH;
                            let CCN = BYE * (B / CCI);
                            let CCO = (AY * CCM) + (BVX * (((CCI - B) - CCN) + (BYE * (B - (UI * ((CCH * CCK) * CCK))))));
                            let CCP = (CCM * CCM) - (BVX * ((((CCI - CCH) - B) + CCN) + (BYE * ((CCH - B) - CCL))));
                            let CCQ = (CCO * CCO) - (AY * (CCP * (AY - (BVX * ((CCI + CCN) - (BYE * ((((UT * CCK) - (BZL * CCL)) * CCK) * CCK)))))));
                            let CCT = (-CCH) - (AY * (CCP / (CCO + (((((CCQ * CCQ) + CCR).sqrt()) - CCS).sqrt()))));
                            CCT
                        } else {
                            let CCU = B / (BYV + (BVV * BWJ));
                            let CCV = (BVM + (BVX * NZ)) - (BVV * (((BVM + (BVX * OQ)) - (B - (rspice_limited_exp((-((BVM * BYB) * (B + (((((BYA * BYV) * CCU) - B) * CCU) * BVM)))))))).sqrt()));
                            let CCW = BYD + UH;
                            let CCX = CCV - CCW;
                            let CCY = (NZ * ((CCV + CCW) - (((CCX * CCX) + UA).sqrt()))) - (NZ * (CCW - (((CCW * CCW) + UA).sqrt())));
                            let CCZ = BVM - CCY;
                            let CDA = rspice_limited_exp((-CCY));
                            let CDB = CCY * CCY;
                            let CDC = B / (AY + CDB);
                            let CDD = CDB * CDC;
                            let CDE = if CAB >= ((CCZ * CCZ) - (BVX * (((CDA + CCY) - B) - (BYE * ((CCY + B) + CDD))))) { CAB } else { ((CCZ * CCZ) - (BVX * (((CDA + CCY) - B) - (BYE * ((CCY + B) + CDD))))) };
                            let CDF = (AY * CCZ) + (BVX * ((B - CDA) - (BYE * (B + (UI * ((CCY * CDC) * CDC))))));
                            let CDG = (BYD - CCY) + ((if (CDE / BVX) >= BAF { (CDE / BVX) } else { BAF }).ln());
                            let CDH = CDE + CDF;
                            let CDI = CDF * CDF;
                            let CDJ = CDE * (B - (NZ * (BVX * (CDA - (BYE * ((((UT * CDC) - (BZL * CDD)) * CDC) * CDC))))));
                            let CDK = (CDH * CDH) + (CDG * ((NZ * CDI) - CDJ));
                            let CDL = CCY + (((CDE * CDH) * CDG) / (CDK + (((((CDH / CDK) * CDG) * CDG) * CDF) * ((CDI * BDI) - CDJ))));
                            let CDM = B / (rspice_limited_exp(CDL));
                            let CDN = rspice_limited_exp((CDL - BYD));
                            let CDO = CDL * CDL;
                            let CDP = B / (AY + CDO);
                            let CDQ = CDO * CDP;
                            let CDR = BVM - CDL;
                            let CDS = (AY * CDR) + (BVX * (((B - CDM) + CDN) - (BYE * (B + (UI * ((CDL * CDP) * CDP))))));
                            let CDT = (CDR * CDR) - (BVX * ((((CDM + CDL) - B) + CDN) - (BYE * ((CDL + B) + CDQ))));
                            let CDU = (CDS * CDS) - (AY * (CDT * (AY - (BVX * ((CDM + CDN) - (BYE * ((((UT * CDP) - (BZL * CDQ)) * CDP) * CDP)))))));
                            let CDV = CDL + (AY * (CDT / (CDS + (((((CDU * CDU) + CCR).sqrt()) - CCS).sqrt()))));
                            CDV
                        };
                        CEA = CEB;
                    }
                    CDW = CEA;
                }
                let CEC = ((BYB * BYB) * BYR) * BWC;
                let CED = BTV * BTV;
                let CEE = (((BYI * (BCW * CED)) / BCV) + BYJ) - ((BVZ * (BVN * BUT)) * EB);
                let CEF = B + BVZ;
                let CEG = CEF * BXZ;
                let CEH = CEE + CEG;
                let CEI = BVM.abs();
                let CEJ = if CEI <= BWE { 1.0 } else { 0.0 };
                let CFV;
                let CVR;
                if CEJ != 0.0 {
                    let CEK = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC));
                    CFV = CEK;
                    CVR = A;
                } else {
                    let CEL = ((BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC))) * (NZ * (((-5e0f64 * (BVM - AY)).tanh()) + ((UA * (BVM + AY)).tanh())));
                    let CEM = ((CDW * BUT) - CEH) / BUT;
                    let CEN = rspice_limited_exp(CEM);
                    let CEO = ((CEL * BUT) - CEH) / BUT;
                    let CEQ = if CEM > CEP { 1.0 } else { 0.0 };
                    let CEY;
                    if CEQ != 0.0 {
                        CEY = CEM;
                    } else {
                        let CER = if CEM < -3.7e1f64 { 1.0 } else { 0.0 };
                        let CEZ = if CER != 0.0 {
                            let CES = CEM.exp();
                            CES
                        } else {
                            let CET = (B + (CEM.exp())).ln();
                            CET
                        };
                        CEY = CEZ;
                    }
                    let CEU = if CEO > CEP { 1.0 } else { 0.0 };
                    let CFA;
                    if CEU != 0.0 {
                        CFA = CEO;
                    } else {
                        let CEV = if CEO < -3.7e1f64 { 1.0 } else { 0.0 };
                        let CFB = if CEV != 0.0 {
                            let CEW = CEO.exp();
                            CEW
                        } else {
                            let CEX = (B + (CEO.exp())).ln();
                            CEX
                        };
                        CFA = CFB;
                    }
                    let CFC = -((BXZ / BUT) + ((CEY - CFA) / CEF));
                    let CFD = rspice_limited_exp(CFC);
                    let CFE = rspice_limited_exp((-CDW));
                    let CFF = CDW * CDW;
                    let CFG = B / (CFF + AY);
                    let CFH = rspice_limited_exp((CDW - BYD));
                    let CFI = BVM - CDW;
                    let CFJ = BVN + CFC;
                    let CFK = CFG * CFF;
                    let CFL = ((CFI * CFI) - (((BWA * BWA) * CFJ) * CFJ)) - (BVX * (((((CFE - CFD) + CDW) + CFC) + CFH) - (BYE * ((CDW + B) + CFK))));
                    let CFM = B + CEN;
                    let CFN = CEF * CFM;
                    let CFO = AY * CDW;
                    let CFP = CEN / CFN;
                    let CFQ = CEN * CFD;
                    let CFR = (((((((AY * CEN) * CFJ) * BWA) * BWA) / CFN) - (AY * BVM)) + CFO) - (BVX * (((((CFH + (BYE * ((((-2e0f64 * CDW) * CFG) + ((((CFO * CDW) * CDW) * CFG) * CFG)) - B))) - CFE) - CFP) + (CFQ / CFN)) + B));
                    let CFS = ((AY * BWA) * BWA) * CEN;
                    let CFT = CFS * CEN;
                    let CFU = CDW - ((CFL / CFR) * (B + ((CFL * ((((((CFS * CFJ) / CFN) - (CFT / ((CFN * CEF) * CFM))) - (BVX * (((CFE + CFH) - (((AY * BYE) * CFG) * (B - (CFK * (UA - ((UI * CFF) * CFG)))))) - (CFP * (((B - (CEN / CFM)) - CFD) + ((CFQ / CFM) * (B + (B / CEF)))))))) - ((CFT * CFJ) / (CFN * CFM))) + AY)) / ((AY * CFR) * CFR))));
                    CFV = CFU;
                    CVR = CEL;
                }
                let CHG;
                let CVQ;
                if CEJ != 0.0 {
                    let CFW = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC));
                    CHG = CFW;
                    CVQ = CVR;
                } else {
                    let CFX = ((BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC))) * (NZ * (((-5e0f64 * (BVM - AY)).tanh()) + ((UA * (BVM + AY)).tanh())));
                    let CFY = ((CFV * BUT) - CEH) / BUT;
                    let CFZ = rspice_limited_exp(CFY);
                    let CGA = ((CFX * BUT) - CEH) / BUT;
                    let CGB = if CFY > CEP { 1.0 } else { 0.0 };
                    let CGJ;
                    if CGB != 0.0 {
                        CGJ = CFY;
                    } else {
                        let CGC = if CFY < -3.7e1f64 { 1.0 } else { 0.0 };
                        let CGK = if CGC != 0.0 {
                            let CGD = CFY.exp();
                            CGD
                        } else {
                            let CGE = (B + (CFY.exp())).ln();
                            CGE
                        };
                        CGJ = CGK;
                    }
                    let CGF = if CGA > CEP { 1.0 } else { 0.0 };
                    let CGL;
                    if CGF != 0.0 {
                        CGL = CGA;
                    } else {
                        let CGG = if CGA < -3.7e1f64 { 1.0 } else { 0.0 };
                        let CGM = if CGG != 0.0 {
                            let CGH = CGA.exp();
                            CGH
                        } else {
                            let CGI = (B + (CGA.exp())).ln();
                            CGI
                        };
                        CGL = CGM;
                    }
                    let CGN = -((BXZ / BUT) + ((CGJ - CGL) / CEF));
                    let CGO = rspice_limited_exp(CGN);
                    let CGP = rspice_limited_exp((-CFV));
                    let CGQ = CFV * CFV;
                    let CGR = B / (CGQ + AY);
                    let CGS = rspice_limited_exp((CFV - BYD));
                    let CGT = BVM - CFV;
                    let CGU = BVN + CGN;
                    let CGV = CGR * CGQ;
                    let CGW = ((CGT * CGT) - (((BWA * BWA) * CGU) * CGU)) - (BVX * (((((CGP - CGO) + CFV) + CGN) + CGS) - (BYE * ((CFV + B) + CGV))));
                    let CGX = B + CFZ;
                    let CGY = CEF * CGX;
                    let CGZ = AY * CFV;
                    let CHA = CFZ / CGY;
                    let CHB = CFZ * CGO;
                    let CHC = (((((((AY * CFZ) * CGU) * BWA) * BWA) / CGY) - (AY * BVM)) + CGZ) - (BVX * (((((CGS + (BYE * ((((-2e0f64 * CFV) * CGR) + ((((CGZ * CFV) * CFV) * CGR) * CGR)) - B))) - CGP) - CHA) + (CHB / CGY)) + B));
                    let CHD = ((AY * BWA) * BWA) * CFZ;
                    let CHE = CHD * CFZ;
                    let CHF = CFV - ((CGW / CHC) * (B + ((CGW * ((((((CHD * CGU) / CGY) - (CHE / ((CGY * CEF) * CGX))) - (BVX * (((CGP + CGS) - (((AY * BYE) * CGR) * (B - (CGV * (UA - ((UI * CGQ) * CGR)))))) - (CHA * (((B - (CFZ / CGX)) - CGO) + ((CHB / CGX) * (B + (B / CEF)))))))) - ((CHE * CGU) / (CGY * CGX))) + AY)) / ((AY * CHC) * CHC))));
                    CHG = CHF;
                    CVQ = CFX;
                }
                let CIT;
                let CVP;
                if CEJ != 0.0 {
                    let CHH = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC));
                    CIT = CHH;
                    CVP = CVQ;
                } else {
                    let CHI = ((BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC))) * (NZ * (((-5e0f64 * (BVM - AY)).tanh()) + ((UA * (BVM + AY)).tanh())));
                    let CHJ = ((CHG * BUT) - CEH) / BUT;
                    let CHK = rspice_limited_exp(CHJ);
                    let CHL = ((CHI * BUT) - CEH) / BUT;
                    let CHM = if CHJ > CEP { 1.0 } else { 0.0 };
                    let CHU;
                    if CHM != 0.0 {
                        CHU = CHJ;
                    } else {
                        let CHN = if CHJ < -3.7e1f64 { 1.0 } else { 0.0 };
                        let CHV = if CHN != 0.0 {
                            let CHO = CHJ.exp();
                            CHO
                        } else {
                            let CHP = (B + (CHJ.exp())).ln();
                            CHP
                        };
                        CHU = CHV;
                    }
                    let CHQ = if CHL > CEP { 1.0 } else { 0.0 };
                    let CHW;
                    if CHQ != 0.0 {
                        CHW = CHL;
                    } else {
                        let CHR = if CHL < -3.7e1f64 { 1.0 } else { 0.0 };
                        let CHX = if CHR != 0.0 {
                            let CHS = CHL.exp();
                            CHS
                        } else {
                            let CHT = (B + (CHL.exp())).ln();
                            CHT
                        };
                        CHW = CHX;
                    }
                    let CHY = -((BXZ / BUT) + ((CHU - CHW) / CEF));
                    let CHZ = rspice_limited_exp(CHY);
                    let CIA = rspice_limited_exp((-CHG));
                    let CIB = CHG * CHG;
                    let CIC = B / (CIB + AY);
                    let CID = rspice_limited_exp((CHG - BYD));
                    let CIE = BVM - CHG;
                    let CIF = BVN + CHY;
                    let CIG = CIC * CIB;
                    let CIH = ((CIE * CIE) - (((BWA * BWA) * CIF) * CIF)) - (BVX * (((((CIA - CHZ) + CHG) + CHY) + CID) - (BYE * ((CHG + B) + CIG))));
                    let CII = B + CHK;
                    let CIJ = CEF * CII;
                    let CIK = AY * CHG;
                    let CIL = CHK / CIJ;
                    let CIM = CHK * CHZ;
                    let CIN = (((((((AY * CHK) * CIF) * BWA) * BWA) / CIJ) - (AY * BVM)) + CIK) - (BVX * (((((CID + (BYE * ((((-2e0f64 * CHG) * CIC) + ((((CIK * CHG) * CHG) * CIC) * CIC)) - B))) - CIA) - CIL) + (CIM / CIJ)) + B));
                    let CIO = ((AY * BWA) * BWA) * CHK;
                    let CIP = CIO * CHK;
                    let CIQ = CHG - ((CIH / CIN) * (B + ((CIH * ((((((CIO * CIF) / CIJ) - (CIP / ((CIJ * CEF) * CII))) - (BVX * (((CIA + CID) - (((AY * BYE) * CIC) * (B - (CIG * (UA - ((UI * CIB) * CIC)))))) - (CIL * (((B - (CHK / CII)) - CHZ) + ((CIM / CII) * (B + (B / CEF)))))))) - ((CIP * CIF) / (CIJ * CII))) + AY)) / ((AY * CIN) * CIN))));
                    CIT = CIQ;
                    CVP = CHI;
                }
                let CIS = CIR * BUT;
                let CIU = if CIT <= A { 1.0 } else { 0.0 };
                let DDF;
                let DDG;
                let DDH;
                let DDI;
                let DDJ;
                let DDM;
                let DDN;
                let DDO;
                let DDP;
                let DDR;
                let DDS;
                let DDV;
                let DGM;
                let DNB;
                let DNC;
                let DNE;
                let DNF;
                let DNI;
                let ENU;
                let ERI;
                let ETB;
                let EVI;
                let EWG;
                let EXU;
                let EYN;
                let HXG;
                let HXU;
                if CIU != 0.0 {
                    let CIV = (BVM - CIT) * BUT;
                    DDF = A;
                    DDG = A;
                    DDH = A;
                    DDI = A;
                    DDJ = B;
                    DDM = B;
                    DDN = B;
                    DDO = B;
                    DDP = B;
                    DDR = B;
                    DDS = B;
                    DDV = A;
                    DGM = A;
                    DNB = CIV;
                    DNC = A;
                    DNE = A;
                    DNF = A;
                    DNI = B;
                    ENU = CVP;
                    ERI = A;
                    ETB = A;
                    EVI = A;
                    EWG = A;
                    EXU = A;
                    EYN = CIS;
                    HXG = CIW;
                    HXU = CIZ;
                } else {
                    let CJC = CIT * CIT;
                    let CJD = B / (rspice_limited_exp(CIT));
                    let CJE = (rspice_limited_exp((CIT - BYD))) - (BYE * ((CIT + B) + (CJC * (B / (AY + CJC)))));
                    let CJF = BVM - CIT;
                    let CJG = (((CJF * CJF) * BVY) - CJE) - BDB;
                    let CJI = (NZ * (CJG + (((CJG * CJG) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                    let CJJ = BVV * (CJI.sqrt());
                    let CJK = ((BVX * CJE) * BUT) / ((BVV * ((CJI + CJE).sqrt())) + CJJ);
                    let CJL = CJJ * BUT;
                    let CJN = CJM / BTW;
                    let CJR = CJP + (CJQ * BSO);
                    let CJT = B + ((CJR * ((CJN * (CJL + (BDM * CJK))).powf(BEF))) + (CJS / (rspice_limited_exp((CJO * ((if (NZ * (B + (CJK / CJL))) >= BAF { (NZ * (B + (CJK / CJL))) } else { BAF }).ln()))))));
                    let CJU = CJT - B;
                    let CJV = NZ * ((CJT + B) + (((CJU * CJU) + 5.625e-7f64).sqrt()));
                    let CJX = B / (((BG * CJW).powf(FR)) * AA);
                    let CKN;
                    if QS != 0.0 {
                        CKN = A;
                    } else {
                        let CJY = (B / (B + (FP * CJK))) + (QL * BVK);
                        let CKD = ((CJZ + (CKB * (CJY + (((CJY * CJY) + BGT).sqrt())))) * CJX) * AA;
                        let CKE = CKD * BEW;
                        let CKF = if QR == AY { 1.0 } else { 0.0 };
                        let CKO = if CKF != 0.0 {
                            let CKG = ((CIZ + CKD) + CIW) * BEW;
                            CKG
                        } else {
                            CKE
                        };
                        CKN = CKO;
                    }
                    let CKI = AY * CKH;
                    let CKK = ((CKI / CKJ) * CJV) * AZ;
                    let CKL = AY * BUT;
                    let CKM = GG * (CJK + CKL);
                    let CKP = if CKN > A { 1.0 } else { 0.0 };
                    let CLE = if CKP != 0.0 {
                        let CKQ = ((BG * CKH) * N) * CKN;
                        let CKR = AY * CKQ;
                        let CKS = (CKM + CKK) + ((UH * CKM) * CKQ);
                        let CKT = (CKS - (((CKS * CKS) - ((AY * CKR) * (CKM * (CKK + ((AY * CKM) * CKQ))))).sqrt())) / CKR;
                        CKT
                    } else {
                        let CKU = (CKK * CKM) / (CKK + CKM);
                        CKU
                    };
                    let CKX = if (if CKV == A { 1.0 } else { 0.0 }) != 0.0 && (if CKW == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CLG = if CKX != 0.0 {
                        B
                    } else {
                        let CKY = AZ / (AZ + ((FA * BUJ).sqrt()));
                        let CLB = B + (((CKV * CKY) - (((CKW * CKY) * (CJK.powf(CKZ))) * BUT)) / (B + (CLA * BSO)));
                        let CLC = CLB - BQB;
                        let CLD = NZ * ((CLB + BQB) + (((CLC * CLC) + 6.25e-8f64).sqrt()));
                        CLD
                    };
                    let CLF = CLE - BDB;
                    let CLH = ((NZ * (CLF + (((CLF * CLF) + 4.0000000000000007e-10f64).sqrt()))) + BDB) / CLG;
                    let CLI = BSE * ((B + (((BSE / CLH) + BX).powf((B / BFH)))).powf((-BFH)));
                    let CLJ = BYC + ((CLI + BSD) * BUU);
                    let CLK = -CLJ;
                    let CLL = rspice_limited_exp(CLK);
                    let CLM = BSE * BUU;
                    let CLN = BYH * (CLM + (DZ * BUU));
                    let CLO = (BYK - (BYL * EB)) + (CEF * CLN);
                    let CLP = CLO + (BVV * ((((rspice_limited_exp((-CLO))) + CLO) - B).sqrt()));
                    let CLQ = if CLO < CLJ { 1.0 } else { 0.0 };
                    let CQQ;
                    if CLQ != 0.0 {
                        let CLR = if BVM < CLP { 1.0 } else { 0.0 };
                        let CQR;
                        if CLR != 0.0 {
                            let CLS = if CEI <= BYF { 1.0 } else { 0.0 };
                            let CQS;
                            if CLS != 0.0 {
                                let CLT = (BVM * BYB) * (B + (((BVM * (B - CLL)) * BVV) * CEC));
                                CQS = CLT;
                            } else {
                                let CLU = if BVM < (-BYF) { 1.0 } else { 0.0 };
                                let CQT = if CLU != 0.0 {
                                    let CLV = -BVM;
                                    let CLW = BYV * (CLV * BYB);
                                    let CLX = CLW - UJ;
                                    let CLY = NZ * ((CLW + SS) - (((CLX * CLX) + BWT).sqrt()));
                                    let CLZ = CLV - CLY;
                                    let CMA = (CLZ * CLZ) + (BVX * (CLY + B));
                                    let CMB = (AY * CLZ) - BVX;
                                    let CMC = (-CLY) + ((if (CMA * BVY) >= BAF { (CMA * BVY) } else { BAF }).ln());
                                    let CMD = CMA + CMB;
                                    let CME = CMB * CMB;
                                    let CMF = (CMD * CMD) + (CMC * ((NZ * CME) - CMA));
                                    let CMG = CLY + (((CMA * CMD) * CMC) / (CMF + (((((CMD / CMF) * CMC) * CMC) * CMB) * ((CME * BDI) - CMA))));
                                    let CMH = rspice_limited_exp(CMG);
                                    let CMI = CMG * CMG;
                                    let CMJ = B / (AY + CMI);
                                    let CMK = CMI * CMJ;
                                    let CML = CLV - CMG;
                                    let CMM = CLL * (B / CMH);
                                    let CMN = (AY * CML) + (BVX * (((CMH - B) - CMM) + (CLL * (B - (UI * ((CMG * CMJ) * CMJ))))));
                                    let CMO = (CML * CML) - (BVX * ((((CMH - CMG) - B) + CMM) + (CLL * ((CMG - B) - CMK))));
                                    let CMP = (-CMG) - (AY * (CMO / (CMN + (((CMN * CMN) - (AY * (CMO * (AY - (BVX * ((CMH + CMM) - (CLL * ((((UT * CMJ) - (BZL * CMK)) * CMJ) * CMJ)))))))).sqrt()))));
                                    CMP
                                } else {
                                    let CMQ = B / (BYV + (BVV * BWJ));
                                    let CMR = (BVM + (BVX * NZ)) - (BVV * (((BVM + (BVX * OQ)) - (B - (rspice_limited_exp((-((BVM * BYB) * (B + (((((BYA * BYV) * CMQ) - B) * CMQ) * BVM)))))))).sqrt()));
                                    let CMS = CLJ + UH;
                                    let CMT = CMR - CMS;
                                    let CMU = (NZ * ((CMR + CMS) - (((CMT * CMT) + UA).sqrt()))) - (NZ * (CMS - (((CMS * CMS) + UA).sqrt())));
                                    let CMV = BVM - CMU;
                                    let CMW = rspice_limited_exp((-CMU));
                                    let CMX = CMU * CMU;
                                    let CMY = B / (AY + CMX);
                                    let CMZ = CMX * CMY;
                                    let CNA = if CAB >= ((CMV * CMV) - (BVX * (((CMW + CMU) - B) - (CLL * ((CMU + B) + CMZ))))) { CAB } else { ((CMV * CMV) - (BVX * (((CMW + CMU) - B) - (CLL * ((CMU + B) + CMZ))))) };
                                    let CNB = (AY * CMV) + (BVX * ((B - CMW) - (CLL * (B + (UI * ((CMU * CMY) * CMY))))));
                                    let CNC = (CLJ - CMU) + ((if (CNA / BVX) >= BAF { (CNA / BVX) } else { BAF }).ln());
                                    let CND = CNA + CNB;
                                    let CNE = CNB * CNB;
                                    let CNF = CNA * (B - (NZ * (BVX * (CMW - (CLL * ((((UT * CMY) - (BZL * CMZ)) * CMY) * CMY))))));
                                    let CNG = (CND * CND) + (CNC * ((NZ * CNE) - CNF));
                                    let CNH = CMU + (((CNA * CND) * CNC) / (CNG + (((((CND / CNG) * CNC) * CNC) * CNB) * ((CNE * BDI) - CNF))));
                                    let CNI = B / (rspice_limited_exp(CNH));
                                    let CNJ = rspice_limited_exp((CNH - CLJ));
                                    let CNK = CNH * CNH;
                                    let CNL = B / (AY + CNK);
                                    let CNM = CNK * CNL;
                                    let CNN = BVM - CNH;
                                    let CNO = (AY * CNN) + (BVX * (((B - CNI) + CNJ) - (CLL * (B + (UI * ((CNH * CNL) * CNL))))));
                                    let CNP = (CNN * CNN) - (BVX * ((((CNI + CNH) - B) + CNJ) - (CLL * ((CNH + B) + CNM))));
                                    let CNQ = CNH + (AY * (CNP / (CNO + (((CNO * CNO) - (AY * (CNP * (AY - (BVX * ((CNI + CNJ) - (CLL * ((((UT * CNL) - (BZL * CNM)) * CNL) * CNL)))))))).sqrt()))));
                                    CNQ
                                };
                                CQS = CQT;
                            }
                            CQR = CQS;
                        } else {
                            let CNR = BWA * BWA;
                            let CNS = CLO - (BXZ * BUU);
                            let CNT = BVM - (BVV * ((((rspice_limited_exp((-CNS))) + CNS) - B).sqrt()));
                            let CNU = CLJ + UH;
                            let CNV = CNT - CNU;
                            let CNW = NZ * ((CNT + CNU) - (((CNV * CNV) + BUA).sqrt()));
                            let CNX = BVM - CNW;
                            let CNY = (BVN - CNW) + CLO;
                            let CNZ = ((CNX * CNX) - ((CNR * CNY) * CNY)) - (BVX * CLO);
                            let COA = AY * CNR;
                            let COB = (AY * CNX) - (COA * CNY);
                            let COC = COB * COB;
                            let COD = B - CNR;
                            let COE = if CNZ < A { 1.0 } else { 0.0 };
                            let COF = if COE != 0.0 {
                                A
                            } else {
                                CNZ
                            };
                            let COG = COF + COB;
                            let COH = COF * COD;
                            let COI = (((COG * COG) / ((CLJ - CNW) + ((if (COF * BVY) >= BAF { (COF * BVY) } else { BAF }).ln()))) + (NZ * COC)) - COH;
                            let COJ = CNW + ((COG * COF) / (COI + (((COB * COG) / COI) * ((BDI * COC) - COH))));
                            let COK = rspice_limited_exp((COJ - CLJ));
                            let COL = BVM - COJ;
                            let COM = (BVN - COJ) + CLO;
                            let CON = BVX * COK;
                            let COO = ((AY * COL) - (COA * COM)) + CON;
                            let COP = AY * (((COL * COL) - ((CNR * COM) * COM)) - (BVX * (CLO + COK)));
                            let COQ = COJ + (COP / (COO + (((COO * COO) - (COP * ((AY - COA) - CON))).sqrt())));
                            CQR = COQ;
                        }
                        CQQ = CQR;
                    } else {
                        let COR = if CEI <= BYF { 1.0 } else { 0.0 };
                        let CQU;
                        if COR != 0.0 {
                            let COS = (BVM * BYB) * (B + (((BVM * (B - CLL)) * BVV) * CEC));
                            CQU = COS;
                        } else {
                            let COT = if BVM < (-BYF) { 1.0 } else { 0.0 };
                            let CQV = if COT != 0.0 {
                                let COU = -BVM;
                                let COV = BYV * (COU * BYB);
                                let COW = COV - UJ;
                                let COX = NZ * ((COV + SS) - (((COW * COW) + BWT).sqrt()));
                                let COY = COU - COX;
                                let COZ = (COY * COY) + (BVX * (COX + B));
                                let CPA = (AY * COY) - BVX;
                                let CPB = (-COX) + ((if (COZ * BVY) >= BAF { (COZ * BVY) } else { BAF }).ln());
                                let CPC = COZ + CPA;
                                let CPD = CPA * CPA;
                                let CPE = (CPC * CPC) + (CPB * ((NZ * CPD) - COZ));
                                let CPF = COX + (((COZ * CPC) * CPB) / (CPE + (((((CPC / CPE) * CPB) * CPB) * CPA) * ((CPD * BDI) - COZ))));
                                let CPG = rspice_limited_exp(CPF);
                                let CPH = CPF * CPF;
                                let CPI = B / (AY + CPH);
                                let CPJ = CPH * CPI;
                                let CPK = COU - CPF;
                                let CPL = CLL * (B / CPG);
                                let CPM = (AY * CPK) + (BVX * (((CPG - B) - CPL) + (CLL * (B - (UI * ((CPF * CPI) * CPI))))));
                                let CPN = (CPK * CPK) - (BVX * ((((CPG - CPF) - B) + CPL) + (CLL * ((CPF - B) - CPJ))));
                                let CPO = (-CPF) - (AY * (CPN / (CPM + (((CPM * CPM) - (AY * (CPN * (AY - (BVX * ((CPG + CPL) - (CLL * ((((UT * CPI) - (BZL * CPJ)) * CPI) * CPI)))))))).sqrt()))));
                                CPO
                            } else {
                                let CPP = B / (BYV + (BVV * BWJ));
                                let CPQ = (BVM + (BVX * NZ)) - (BVV * (((BVM + (BVX * OQ)) - (B - (rspice_limited_exp((-((BVM * BYB) * (B + (((((BYA * BYV) * CPP) - B) * CPP) * BVM)))))))).sqrt()));
                                let CPR = CLJ + UH;
                                let CPS = CPQ - CPR;
                                let CPT = (NZ * ((CPQ + CPR) - (((CPS * CPS) + UA).sqrt()))) - (NZ * (CPR - (((CPR * CPR) + UA).sqrt())));
                                let CPU = BVM - CPT;
                                let CPV = rspice_limited_exp((-CPT));
                                let CPW = CPT * CPT;
                                let CPX = B / (AY + CPW);
                                let CPY = CPW * CPX;
                                let CPZ = if CAB >= ((CPU * CPU) - (BVX * (((CPV + CPT) - B) - (CLL * ((CPT + B) + CPY))))) { CAB } else { ((CPU * CPU) - (BVX * (((CPV + CPT) - B) - (CLL * ((CPT + B) + CPY))))) };
                                let CQA = (AY * CPU) + (BVX * ((B - CPV) - (CLL * (B + (UI * ((CPT * CPX) * CPX))))));
                                let CQB = (CLJ - CPT) + ((if (CPZ / BVX) >= BAF { (CPZ / BVX) } else { BAF }).ln());
                                let CQC = CPZ + CQA;
                                let CQD = CQA * CQA;
                                let CQE = CPZ * (B - (NZ * (BVX * (CPV - (CLL * ((((UT * CPX) - (BZL * CPY)) * CPX) * CPX))))));
                                let CQF = (CQC * CQC) + (CQB * ((NZ * CQD) - CQE));
                                let CQG = CPT + (((CPZ * CQC) * CQB) / (CQF + (((((CQC / CQF) * CQB) * CQB) * CQA) * ((CQD * BDI) - CQE))));
                                let CQH = B / (rspice_limited_exp(CQG));
                                let CQI = rspice_limited_exp((CQG - CLJ));
                                let CQJ = CQG * CQG;
                                let CQK = B / (AY + CQJ);
                                let CQL = CQJ * CQK;
                                let CQM = BVM - CQG;
                                let CQN = (AY * CQM) + (BVX * (((B - CQH) + CQI) - (CLL * (B + (UI * ((CQG * CQK) * CQK))))));
                                let CQO = (CQM * CQM) - (BVX * ((((CQH + CQG) - B) + CQI) - (CLL * ((CQG + B) + CQL))));
                                let CQP = CQG + (AY * (CQO / (CQN + (((CQN * CQN) - (AY * (CQO * (AY - (BVX * ((CQH + CQI) - (CLL * ((((UT * CQK) - (BZL * CQL)) * CQK) * CQK)))))))).sqrt()))));
                                CQP
                            };
                            CQU = CQV;
                        }
                        CQQ = CQU;
                    }
                    let CQW = (CEE + ((CEF * CLN) * BUT)) + CEG;
                    let CSH;
                    let CVO;
                    if CEJ != 0.0 {
                        let CQX = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC));
                        CSH = CQX;
                        CVO = CVP;
                    } else {
                        let CQY = ((BVM * BYB) * (B + (((BVM * (B - CLL)) * BVV) * CEC))) * (NZ * (((-5e0f64 * (BVM - AY)).tanh()) + ((UA * (BVM + AY)).tanh())));
                        let CQZ = ((CQQ * BUT) - CQW) / BUT;
                        let CRA = rspice_limited_exp(CQZ);
                        let CRB = ((CQY * BUT) - CQW) / BUT;
                        let CRC = if CQZ > CEP { 1.0 } else { 0.0 };
                        let CRK;
                        if CRC != 0.0 {
                            CRK = CQZ;
                        } else {
                            let CRD = if CQZ < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CRL = if CRD != 0.0 {
                                let CRE = CQZ.exp();
                                CRE
                            } else {
                                let CRF = (B + (CQZ.exp())).ln();
                                CRF
                            };
                            CRK = CRL;
                        }
                        let CRG = if CRB > CEP { 1.0 } else { 0.0 };
                        let CRM;
                        if CRG != 0.0 {
                            CRM = CRB;
                        } else {
                            let CRH = if CRB < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CRN = if CRH != 0.0 {
                                let CRI = CRB.exp();
                                CRI
                            } else {
                                let CRJ = (B + (CRB.exp())).ln();
                                CRJ
                            };
                            CRM = CRN;
                        }
                        let CRO = -((BXZ / BUT) + ((CRK - CRM) / CEF));
                        let CRP = rspice_limited_exp(CRO);
                        let CRQ = rspice_limited_exp((-CQQ));
                        let CRR = CQQ * CQQ;
                        let CRS = B / (CRR + AY);
                        let CRT = rspice_limited_exp((CQQ - CLJ));
                        let CRU = BVM - CQQ;
                        let CRV = BVN + CRO;
                        let CRW = CRS * CRR;
                        let CRX = ((CRU * CRU) - (((BWA * BWA) * CRV) * CRV)) - (BVX * (((((CRQ - CRP) + CQQ) + CRO) + CRT) - (CLL * ((CQQ + B) + CRW))));
                        let CRY = B + CRA;
                        let CRZ = CEF * CRY;
                        let CSA = AY * CQQ;
                        let CSB = CRA / CRZ;
                        let CSC = CRA * CRP;
                        let CSD = (((((((AY * CRA) * CRV) * BWA) * BWA) / CRZ) - (AY * BVM)) + CSA) - (BVX * (((((CRT + (CLL * ((((-2e0f64 * CQQ) * CRS) + ((((CSA * CQQ) * CQQ) * CRS) * CRS)) - B))) - CRQ) - CSB) + (CSC / CRZ)) + B));
                        let CSE = ((AY * BWA) * BWA) * CRA;
                        let CSF = CSE * CRA;
                        let CSG = CQQ - ((CRX / CSD) * (B + ((CRX * ((((((CSE * CRV) / CRZ) - (CSF / ((CRZ * CEF) * CRY))) - (BVX * (((CRQ + CRT) - (((AY * CLL) * CRS) * (B - (CRW * (UA - ((UI * CRR) * CRS)))))) - (CSB * (((B - (CRA / CRY)) - CRP) + ((CSC / CRY) * (B + (B / CEF)))))))) - ((CSF * CRV) / (CRZ * CRY))) + AY)) / ((AY * CSD) * CSD))));
                        CSH = CSG;
                        CVO = CQY;
                    }
                    let CTS;
                    let CVN;
                    if CEJ != 0.0 {
                        let CSI = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC));
                        CTS = CSI;
                        CVN = CVO;
                    } else {
                        let CSJ = ((BVM * BYB) * (B + (((BVM * (B - CLL)) * BVV) * CEC))) * (NZ * (((-5e0f64 * (BVM - AY)).tanh()) + ((UA * (BVM + AY)).tanh())));
                        let CSK = ((CSH * BUT) - CQW) / BUT;
                        let CSL = rspice_limited_exp(CSK);
                        let CSM = ((CSJ * BUT) - CQW) / BUT;
                        let CSN = if CSK > CEP { 1.0 } else { 0.0 };
                        let CSV;
                        if CSN != 0.0 {
                            CSV = CSK;
                        } else {
                            let CSO = if CSK < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CSW = if CSO != 0.0 {
                                let CSP = CSK.exp();
                                CSP
                            } else {
                                let CSQ = (B + (CSK.exp())).ln();
                                CSQ
                            };
                            CSV = CSW;
                        }
                        let CSR = if CSM > CEP { 1.0 } else { 0.0 };
                        let CSX;
                        if CSR != 0.0 {
                            CSX = CSM;
                        } else {
                            let CSS = if CSM < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CSY = if CSS != 0.0 {
                                let CST = CSM.exp();
                                CST
                            } else {
                                let CSU = (B + (CSM.exp())).ln();
                                CSU
                            };
                            CSX = CSY;
                        }
                        let CSZ = -((BXZ / BUT) + ((CSV - CSX) / CEF));
                        let CTA = rspice_limited_exp(CSZ);
                        let CTB = rspice_limited_exp((-CSH));
                        let CTC = CSH * CSH;
                        let CTD = B / (CTC + AY);
                        let CTE = rspice_limited_exp((CSH - CLJ));
                        let CTF = BVM - CSH;
                        let CTG = BVN + CSZ;
                        let CTH = CTD * CTC;
                        let CTI = ((CTF * CTF) - (((BWA * BWA) * CTG) * CTG)) - (BVX * (((((CTB - CTA) + CSH) + CSZ) + CTE) - (CLL * ((CSH + B) + CTH))));
                        let CTJ = B + CSL;
                        let CTK = CEF * CTJ;
                        let CTL = AY * CSH;
                        let CTM = CSL / CTK;
                        let CTN = CSL * CTA;
                        let CTO = (((((((AY * CSL) * CTG) * BWA) * BWA) / CTK) - (AY * BVM)) + CTL) - (BVX * (((((CTE + (CLL * ((((-2e0f64 * CSH) * CTD) + ((((CTL * CSH) * CSH) * CTD) * CTD)) - B))) - CTB) - CTM) + (CTN / CTK)) + B));
                        let CTP = ((AY * BWA) * BWA) * CSL;
                        let CTQ = CTP * CSL;
                        let CTR = CSH - ((CTI / CTO) * (B + ((CTI * ((((((CTP * CTG) / CTK) - (CTQ / ((CTK * CEF) * CTJ))) - (BVX * (((CTB + CTE) - (((AY * CLL) * CTD) * (B - (CTH * (UA - ((UI * CTC) * CTD)))))) - (CTM * (((B - (CSL / CTJ)) - CTA) + ((CTN / CTJ) * (B + (B / CEF)))))))) - ((CTQ * CTG) / (CTK * CTJ))) + AY)) / ((AY * CTO) * CTO))));
                        CTS = CTR;
                        CVN = CSJ;
                    }
                    let CVD;
                    let CVM;
                    if CEJ != 0.0 {
                        let CTT = (BVM * BYB) * (B + (((BVM * (B - BYE)) * BVV) * CEC));
                        CVD = CTT;
                        CVM = CVN;
                    } else {
                        let CTU = ((BVM * BYB) * (B + (((BVM * (B - CLL)) * BVV) * CEC))) * (NZ * (((-5e0f64 * (BVM - AY)).tanh()) + ((UA * (BVM + AY)).tanh())));
                        let CTV = ((CTS * BUT) - CQW) / BUT;
                        let CTW = rspice_limited_exp(CTV);
                        let CTX = ((CTU * BUT) - CQW) / BUT;
                        let CTY = if CTV > CEP { 1.0 } else { 0.0 };
                        let CUG;
                        if CTY != 0.0 {
                            CUG = CTV;
                        } else {
                            let CTZ = if CTV < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CUH = if CTZ != 0.0 {
                                let CUA = CTV.exp();
                                CUA
                            } else {
                                let CUB = (B + (CTV.exp())).ln();
                                CUB
                            };
                            CUG = CUH;
                        }
                        let CUC = if CTX > CEP { 1.0 } else { 0.0 };
                        let CUI;
                        if CUC != 0.0 {
                            CUI = CTX;
                        } else {
                            let CUD = if CTX < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CUJ = if CUD != 0.0 {
                                let CUE = CTX.exp();
                                CUE
                            } else {
                                let CUF = (B + (CTX.exp())).ln();
                                CUF
                            };
                            CUI = CUJ;
                        }
                        let CUK = -((BXZ / BUT) + ((CUG - CUI) / CEF));
                        let CUL = rspice_limited_exp(CUK);
                        let CUM = rspice_limited_exp((-CTS));
                        let CUN = CTS * CTS;
                        let CUO = B / (CUN + AY);
                        let CUP = rspice_limited_exp((CTS - CLJ));
                        let CUQ = BVM - CTS;
                        let CUR = BVN + CUK;
                        let CUS = CUO * CUN;
                        let CUT = ((CUQ * CUQ) - (((BWA * BWA) * CUR) * CUR)) - (BVX * (((((CUM - CUL) + CTS) + CUK) + CUP) - (CLL * ((CTS + B) + CUS))));
                        let CUU = B + CTW;
                        let CUV = CEF * CUU;
                        let CUW = AY * CTS;
                        let CUX = CTW / CUV;
                        let CUY = CTW * CUL;
                        let CUZ = (((((((AY * CTW) * CUR) * BWA) * BWA) / CUV) - (AY * BVM)) + CUW) - (BVX * (((((CUP + (CLL * ((((-2e0f64 * CTS) * CUO) + ((((CUW * CTS) * CTS) * CUO) * CUO)) - B))) - CUM) - CUX) + (CUY / CUV)) + B));
                        let CVA = ((AY * BWA) * BWA) * CTW;
                        let CVB = CVA * CTW;
                        let CVC = CTS - ((CUT / CUZ) * (B + ((CUT * ((((((CVA * CUR) / CUV) - (CVB / ((CUV * CEF) * CUU))) - (BVX * (((CUM + CUP) - (((AY * CLL) * CUO) * (B - (CUS * (UA - ((UI * CUN) * CUO)))))) - (CUX * (((B - (CTW / CUU)) - CUL) + ((CUY / CUU) * (B + (B / CEF)))))))) - ((CVB * CUR) / (CUV * CUU))) + AY)) / ((AY * CUZ) * CUZ))));
                        CVD = CVC;
                        CVM = CTU;
                    }
                    let CVE = CVD - CIT;
                    let CVF = -CLM;
                    let CVG = rspice_limited_exp(CVF);
                    let CVI = if CVE < CVH { 1.0 } else { 0.0 };
                    let CXD;
                    let CXF;
                    if CVI != 0.0 {
                        let CVJ = (CTS * BUT) - CQW;
                        let CVK = CVJ / BUT;
                        let CVL = rspice_limited_exp(CVK);
                        let CVS = ((CVM * BUT) - CQW) / BUT;
                        let CVT = if CVK > CEP { 1.0 } else { 0.0 };
                        let CWB;
                        if CVT != 0.0 {
                            CWB = CVK;
                        } else {
                            let CVU = if CVK < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CWC = if CVU != 0.0 {
                                let CVV = CVK.exp();
                                CVV
                            } else {
                                let CVW = (B + (CVK.exp())).ln();
                                CVW
                            };
                            CWB = CWC;
                        }
                        let CVX = if CVS > CEP { 1.0 } else { 0.0 };
                        let CWD;
                        if CVX != 0.0 {
                            CWD = CVS;
                        } else {
                            let CVY = if CVS < -3.7e1f64 { 1.0 } else { 0.0 };
                            let CWE = if CVY != 0.0 {
                                let CVZ = CVS.exp();
                                CVZ
                            } else {
                                let CWA = (B + (CVS.exp())).ln();
                                CWA
                            };
                            CWD = CWE;
                        }
                        let CWF = -((BXZ / BUT) + ((CWB - CWD) / CEF));
                        let CWG = rspice_limited_exp((-CTS));
                        let CWH = B / ((CTS * CTS) + AY);
                        let CWI = (AY * CVJ) / BUT;
                        let CWJ = rspice_limited_exp(CWI);
                        let CWK = rspice_limited_exp((CWI + CWF));
                        let CWL = AY * CVL;
                        let CWM = BVN + CWF;
                        let CWN = CEF * (CVL + B);
                        let CWO = AY * CTS;
                        let CWP = CVL / CWN;
                        let CWQ = (rspice_limited_exp((CWF + CVK))) / CWN;
                        let CWR = -(((((((CWL * CWM) * BWA) * BWA) / CWN) - (AY * BVM)) + CWO) - (BVX * ((((((rspice_limited_exp(((CTS - CLM) - CLJ))) + ((rspice_limited_exp((CVF - CLJ))) * ((((-2e0f64 * CTS) * CWH) + ((((CWO * CTS) * CTS) * CWH) * CWH)) - B))) - CWG) - CWP) + CWQ) + B)));
                        let CWS = (BVX * (B - CVG)) * CJE;
                        let CWT = (AY * BWA) * BWA;
                        let CWU = CWT * CWJ;
                        let CWV = (B + CWL) + CWJ;
                        let CWW = (CEF * CEF) * CWV;
                        let CWX = CEF * CWV;
                        let CWY = (CWR * CWR) - (AY * ((((((((CWT * CVL) * CWM) / CWN) - (CWU / CWW)) - (BVX * (((((((CWG + (rspice_limited_exp(((CTS - CLJ) - CLM)))) + ((rspice_limited_exp((CLK - CLM))) * (((-2e0f64 * CWH) + ((((SS * CTS) * CTS) * CWH) * CWH)) - (((((((UT * CTS) * CTS) * CTS) * CTS) * CWH) * CWH) * CWH)))) - CWP) + (CWJ / CWX)) + CWQ) - (CWK / CWX)) - (CWK / CWW)))) - ((CWU * CWM) / CWX)) + AY) * CWS));
                        let CWZ = if CWY >= A { 1.0 } else { 0.0 };
                        let CXB = if CWZ != 0.0 {
                            let CXA = AY * (CWS / (CWR + (CWY.sqrt())));
                            CXA
                        } else {
                            CVE
                        };
                        let CXC = CIT + CXB;
                        CXD = CXB;
                        CXF = CXC;
                    } else {
                        CXD = CVE;
                        CXF = CVD;
                    }
                    let CXE = CXD * BUT;
                    let CXG = CXF * CXF;
                    let CXH = (rspice_limited_exp((CXF - CLJ))) - (CLL * ((CXF + B) + (CXG / (AY + CXG))));
                    let CXI = BVM - CXF;
                    let CXJ = (((CXI * CXI) * BVY) - CXH) - BDB;
                    let CXK = (NZ * (CXJ + (((CXJ * CXJ) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                    let CXL = ((BVX * CXH) * BUT) / ((BVV * ((CXK + CXH).sqrt())) + (BVV * (CXK.sqrt())));
                    let CXM = NZ * (CIT + CXF);
                    let CXN = (((rspice_limited_exp((-CXF))) * CJD).abs()).sqrt();
                    let CXO = NZ * (CJE + CXH);
                    let CXQ = CXO + (CXP * ((CXD * CXD) * (CXN - (AY * BVY))));
                    let CXR = BVM - CXM;
                    let CXS = ((CXR * CXR) * BVY) - CXQ;
                    let CXT = BVV * ((CXQ + CXS).sqrt());
                    let CXU = CXS - BDB;
                    let CXV = (NZ * (CXU + (((CXU * CXU) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                    let CXW = CXV.sqrt();
                    let CXY = if CXX == B { 1.0 } else { 0.0 };
                    let CYP;
                    let CYT;
                    let CYU;
                    let CYV;
                    let DNJ;
                    if CXY != 0.0 {
                        let CXZ = (((AY * N) * N) * BUT) / ((C * J) * DS);
                        let CYA = B - CXN;
                        let CYB = B / ((B + (CXZ * CXT)).sqrt());
                        let CYC = CYB / (CYB + B);
                        let CYD = (CXZ * (((CYC * CYC) * CXT) * CXT)) * (CXQ / (CXQ + CXV));
                        let CYE = (AY * (CXT - CYD)) + (BVX * (CYA + CXQ));
                        let CYF = CYD * (CYD - (AY * CXT));
                        let CYG = (CYF * CYE) / ((CYE * CYE) - ((B - (NZ * (BVX * (CXN + CXQ)))) * CYF));
                        let CYH = rspice_limited_exp(CYG);
                        let CYI = CXQ * CYH;
                        let CYJ = (BVM - (CXM + CYG)) + CYG;
                        let CYK = ((CYJ * CYJ) * BVY) - (CYI / CYH);
                        let CYL = BVV * ((CYI + CYK).sqrt());
                        let CYM = (((CXD * CYH) * ((CYA + (AY * (CXT * BVY))) + CXO)) / (((B - (CXN / CYH)) + (AY * ((CYL * CYB) * BVY))) + (CYH * CXO))) * BUT;
                        let CYN = CYK - BDB;
                        let CYO = ((NZ * (CYN + (((CYN * CYN) + 4.0000000000000007e-10f64).sqrt()))) + BDB).sqrt();
                        CYP = CYM;
                        CYT = CYI;
                        CYU = CYL;
                        CYV = CYO;
                        DNJ = CYB;
                    } else {
                        CYP = CXE;
                        CYT = CXQ;
                        CYU = CXT;
                        CYV = CXW;
                        DNJ = B;
                    }
                    let CYR = if (CYP.abs()) > CYQ { 1.0 } else { 0.0 };
                    let DDB = if CYR != 0.0 {
                        let CYS = (CJK - CXL) / CYP;
                        CYS
                    } else {
                        A
                    };
                    let CYW = BVV * CYV;
                    let CYX = BUT * ((BVX * CYT) / (CYU + CYW));
                    let CYY = CYW * BUT;
                    let CYZ = CYU * BUT;
                    let CZA = B + ((CJR * ((CJN * (CYY + (BDM * CYX))).powf(BEF))) + (CJS / (rspice_limited_exp((CJO * ((if (NZ * (B + (CYX / CYY))) >= BAF { (NZ * (B + (CYX / CYY))) } else { BAF }).ln()))))));
                    let CZB = CZA - B;
                    let CZC = NZ * ((CZA + B) + (((CZB * CZB) + 5.625e-7f64).sqrt()));
                    let CZD = (CKI / (CKJ / CZC)) * AZ;
                    let CZE = if GE > A { 1.0 } else { 0.0 };
                    let CZM = if CZE != 0.0 {
                        let CZF = B + ((GE * CYX) / CZD);
                        CZF
                    } else {
                        let CZG = B / (B - ((GE * CYX) / CZD));
                        CZG
                    };
                    let CZI = BSE - CLI;
                    let CZJ = CYX + CKL;
                    let CZK = if CZH > A { 1.0 } else { 0.0 };
                    let DAF = if CZK != 0.0 {
                        let CZL = B + (FY * BSO);
                        let CZN = B + (CZI / ((((CZJ / CZH) * (CZJ / (CLH + CZJ))) * CZM) * (B / (NZ * (CZL + (((CZL * CZL) + 4e-6f64).sqrt()))))));
                        CZN
                    } else {
                        B
                    };
                    let CZO = if OC <= A { 1.0 } else { 0.0 };
                    let CZV = if CZO != 0.0 {
                        B
                    } else {
                        let CZP = B / (B + ((OC * (AZ.sqrt())) / CZJ));
                        CZP
                    };
                    let CZQ = CLH + CZD;
                    let CZS = if CZR > A { 1.0 } else { 0.0 };
                    let DAG;
                    if CZS != 0.0 {
                        let CZU = if CZT < A { 1.0 } else { 0.0 };
                        let CZY = if CZU != 0.0 {
                            let CZW = (CZR / (B - ((CZT * CYX) / CZD))) / CZV;
                            CZW
                        } else {
                            let CZX = (CZR * (B + ((CZT * CYX) / CZD))) / CZV;
                            CZX
                        };
                        let CZZ = B + (CZY * ((if (B + ((CZI / CZY) / CZQ)) >= BAF { (B + ((CZI / CZY) / CZQ)) } else { BAF }).ln()));
                        DAG = CZZ;
                    } else {
                        let DAA = if CZT < A { 1.0 } else { 0.0 };
                        let DAD = if DAA != 0.0 {
                            let DAB = (CZR / (B - ((CZT * CYX) / CZD))) / CZV;
                            DAB
                        } else {
                            let DAC = (CZR * (B + ((CZT * CYX) / CZD))) / CZV;
                            DAC
                        };
                        let DAE = B + DAD;
                        DAG = DAE;
                    }
                    let DAH = DAF * DAG;
                    let DAI = rspice_limited_exp((GC * BSE));
                    let DAJ = if GB > A { 1.0 } else { 0.0 };
                    let DAN = if DAJ != 0.0 {
                        let DAL = ((B + ((B + (DAK * AZ)) * DAI)) / GB) * CZV;
                        DAL
                    } else {
                        DAM
                    };
                    let DAO = DAH * (B + (CZI / DAN));
                    let DAP = if GA > A { 1.0 } else { 0.0 };
                    let DAU;
                    if DAP != 0.0 {
                        let DAQ = FZ * BCY;
                        let DAR = if CZI > (DAQ / BSH) { 1.0 } else { 0.0 };
                        let DAV = if DAR != 0.0 {
                            let DAS = (AZ * (rspice_limited_exp((DAQ / CZI)))) / GA;
                            DAS
                        } else {
                            let DAT = (DAM * AZ) / GA;
                            DAT
                        };
                        DAU = DAV;
                    } else {
                        DAU = DAM;
                    }
                    let DAW = DAO * (B + (CZI / DAU));
                    let DAX = if JO < A { 1.0 } else { 0.0 };
                    let DBA = if DAX != 0.0 {
                        let DAY = B / (B - (JO * BSO));
                        DAY
                    } else {
                        let DAZ = B + (JO * BSO);
                        DAZ
                    };
                    let DBB = CYX * DBA;
                    let DBC = BEZ * (DBB / (BEZ + DBB));
                    let DBD = B / parameters[503];
                    let DBE = parameters[504] * (((B + ((BSE - CYP) * DBD)) / (B + ((CLI - CYP) * DBD))).ln());
                    let DBF = B / ((B + DBE) + (DBE * DBE));
                    let DBG = CZC * DBF;
                    let DBH = if OR < A { 1.0 } else { 0.0 };
                    let DBK = if DBH != 0.0 {
                        let DBI = B / (B - (OR * DBC));
                        DBI
                    } else {
                        let DBJ = B + (OR * DBC);
                        DBJ
                    };
                    let DBL = GH * (DBK / DBG);
                    let DBM = ((DBL * DBL) * CYP) * CYP;
                    let DBN = if E == -1e0f64 { 1.0 } else { 0.0 };
                    let DBP = if DBN != 0.0 {
                        let DBO = DBM / (B + (DBL * CYP));
                        DBO
                    } else {
                        DBM
                    };
                    let DBQ = NZ * (DBG * (B + ((B + (AY * DBP)).sqrt())));
                    let DBR = B / DBQ;
                    let DBS = CJK + CXL;
                    let DDK;
                    let EXV;
                    let HXH;
                    let HXV;
                    if QS != 0.0 {
                        let DBU = BRQ - DBT;
                        let DBV = (B / (B + (FP * (NZ * (DBU + (((DBU * DBU) + BGT).sqrt())))))) + (QL * BRK);
                        let DCA = BEW * (CIZ + ((DBW + (DBY * (NZ * (DBV + (((DBV * DBV) + BGT).sqrt()))))) * CJX));
                        let DCB = BRP - DBT;
                        let DCC = (B / (B + (FP * (NZ * (DCB + (((DCB * DCB) + BGT).sqrt())))))) + (QL * BRH);
                        let DCH = BEW * (CIW + ((DCD + (DCF * (NZ * (DCC + (((DCC * DCC) + BGT).sqrt()))))) * CJX));
                        DDK = B;
                        EXV = A;
                        HXH = DCH;
                        HXV = DCA;
                    } else {
                        let DCI = (B / (B + (FP * DBS))) + (QL * BVK);
                        let DCJ = CJZ + (CKB * (NZ * (DCI + (((DCI * DCI) + BGT).sqrt()))));
                        let DCK = ((BEW * DCJ) * CJX) * AA;
                        let DCL = ((((CKJ / DBQ) * N) * BG) / AZ) * DBS;
                        let DCM = B + (DCL * DCK);
                        let DCN = if QR == AY { 1.0 } else { 0.0 };
                        let DDL;
                        let EXW;
                        let HXI;
                        let HXW;
                        if DCN != 0.0 {
                            let DCO = BEW * ((CIZ + ((DCJ * CJX) * AA)) + CIW);
                            let DCP = B + (DCL * DCO);
                            DDL = DCP;
                            EXW = DCO;
                            HXI = A;
                            HXW = A;
                        } else {
                            DDL = DCM;
                            EXW = DCK;
                            HXI = CIW;
                            HXW = CIZ;
                        }
                        DDK = DDL;
                        EXV = EXW;
                        HXH = HXI;
                        HXV = HXW;
                    }
                    let DCQ = (AY * BUS) * BBM;
                    let DCR = CJK - CXL;
                    let DCS = ((((BFO + (BFQ / (DBS + DCQ))) * DCR) * DCR) + B) - BDB;
                    let DCU = NZ * (B + ((B + (-1e0f64 + (NZ * (DCS + (((DCS * DCS) + DCT).sqrt()))))).sqrt()));
                    let DCV = DCU - B;
                    let DCW = (NZ * ((DCU + B) - (((DCV * DCV) + 2.5e-5f64).sqrt()))) + 2.5e-3f64;
                    let DCX = DCR / (DBS + BFZ);
                    let DCY = B + ((BFW * DCX) * DCX);
                    let DCZ = rspice_limited_exp((-(BGB / (((if A >= (BGD + ((BGF * DCR) * DCR)) { A } else { (BGD + ((BGF * DCR) * DCR)) }) * DBS) + DCQ))));
                    let DDA = DBG * DBR;
                    let DDC = (DDB * (B + (NZ * ((DBP * DDA) * DDA)))) - BDB;
                    let DDD = CYX + (BUT * DDB);
                    let DDE = (DDD / ((NZ * (DDC + (((DDC * DDC) + 4.0000000000000007e-10f64).sqrt()))) + BDB)) * (DBG / DBQ);
                    DDF = DDD;
                    DDG = CYP;
                    DDH = DBF;
                    DDI = DBR;
                    DDJ = DDK;
                    DDM = DAW;
                    DDN = DCW;
                    DDO = DCY;
                    DDP = DCZ;
                    DDR = CZC;
                    DDS = DBQ;
                    DDV = DBS;
                    DGM = CZI;
                    DNB = CYZ;
                    DNC = DDE;
                    DNE = CYX;
                    DNF = DDB;
                    DNI = DNJ;
                    ENU = CVM;
                    ERI = CXF;
                    ETB = CLI;
                    EVI = CXL;
                    EWG = CJK;
                    EXU = EXV;
                    EYN = CLH;
                    HXG = HXH;
                    HXU = HXV;
                }
                let DDQ = (((((((((AA * CKJ) * (BG / AZ)) * N) * DDF) * DDG) * ((DDH * DDI) / DDJ)) * DDM) / DDN) * DDO) * DDP;
                let DDT = CKJ / ((DDR * DDS) * DDJ);
                let DDU = if BAB > B { 1.0 } else { 0.0 };
                let HWM;
                let HWR;
                if DDU != 0.0 {
                    let DDY = (DDX * AA) * ((((((DDW * BBM) * DDT) * BG) / AZ) * N) + ((((DDT * BG) / AZ) * N) * DDV));
                    let DDZ = if BAB == AY { 1.0 } else { 0.0 };
                    let HWN;
                    let HWS;
                    if DDZ != 0.0 {
                        let DEB = if (B / DEA) < AZG { 1.0 } else { 0.0 };
                        let DED = if DEB != 0.0 {
                            let DEC = B / AZG;
                            DEC
                        } else {
                            DEA
                        };
                        let DEE = (DED * DDY) / (DED + DDY);
                        HWN = DEE;
                        HWS = DED;
                    } else {
                        HWN = DDY;
                        HWS = DEA;
                    }
                    HWM = HWN;
                    HWR = HWS;
                } else {
                    HWM = A;
                    HWR = DEA;
                }
                let DEG = BBM * DEF;
                let DEH = rspice_limited_exp((BRN / DEG));
                let DEI = rspice_limited_exp((BRO / DEG));
                let DEK = (DEJ / BBM) * BCZ;
                let DEL = if GV == A { 1.0 } else { 0.0 };
                if DEL != 0.0 {
                } else {
                }
                let DEM = if GW == A { 1.0 } else { 0.0 };
                if DEM != 0.0 {
                } else {
                }
                let DEN = if GZ == A { 1.0 } else { 0.0 };
                if DEN != 0.0 {
                } else {
                    let DEO = if (HF - BRN) < BDB { 1.0 } else { 0.0 };
                    if DEO != 0.0 {
                    } else {
                    }
                }
                let DEP = if HA == A { 1.0 } else { 0.0 };
                if DEP != 0.0 {
                } else {
                    let DEQ = if (HG - BRO) < BDB { 1.0 } else { 0.0 };
                    if DEQ != 0.0 {
                    } else {
                    }
                }
                let DER = BAP * BTV;
                let DES = if (if HN == A { 1.0 } else { 0.0 }) != 0.0 && (if HO == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let DJQ;
                let DMQ;
                if DES != 0.0 {
                    DJQ = A;
                    DMQ = A;
                } else {
                    let DET = rspice_limited_exp(((HL * DEK) / DEF));
                    let DEU = HN * DET;
                    let DEV = HK * DET;
                    let DEW = HO * DET;
                    let DEX = DEH - B;
                    let DEY = (HJ * DET) * DEX;
                    let DEZ = if DEY < CJH { 1.0 } else { 0.0 };
                    let DFG;
                    let DFS;
                    if DEZ != 0.0 {
                        DFG = B;
                        DFS = A;
                    } else {
                        let DFA = B / ((B + DEY).sqrt());
                        DFG = DFA;
                        DFS = DEY;
                    }
                    let DFB = DEI - B;
                    let DFC = DEV * DFB;
                    let DFD = if DFC < CJH { 1.0 } else { 0.0 };
                    let DFH;
                    let DFT;
                    if DFD != 0.0 {
                        DFH = B;
                        DFT = A;
                    } else {
                        let DFE = B / ((B + DFC).sqrt());
                        DFH = DFE;
                        DFT = DFC;
                    }
                    let DFJ = B + (DFI * ((HP * ((B / AZ) + (B / DFF))).powf(HQ)));
                    let DFK = (((DER * DEU) * DFJ) * DEX) * DFG;
                    let DFL = (((DER * DEW) * DFJ) * DFB) * DFH;
                    let DFM = HH + (HI * AZ);
                    let DFN = if DFM < B { 1.0 } else { 0.0 };
                    let DFQ = if DFN != 0.0 {
                        B
                    } else {
                        DFM
                    };
                    let DFP = if DFO == B { 1.0 } else { 0.0 };
                    if DFP != 0.0 {
                    } else {
                        let DFR = B + ((BRN + BRO) / DFQ);
                        let DFU = if ((DFR + (((DFR * DFR) + (UI * (DFS + DFT))).sqrt())) / AY) < BQB { 1.0 } else { 0.0 };
                        if DFU != 0.0 {
                        } else {
                        }
                    }
                    DJQ = DFK;
                    DMQ = DFL;
                }
                let DFV = if (if HB == A { 1.0 } else { 0.0 }) != 0.0 && (if HC == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if DFV != 0.0 {
                } else {
                    let DFW = if (HD - BRN) < BDB { 1.0 } else { 0.0 };
                    if DFW != 0.0 {
                    } else {
                    }
                    let DFX = if (HE - BRO) < BDB { 1.0 } else { 0.0 };
                    if DFX != 0.0 {
                    } else {
                    }
                }
                let DFZ = if DFY == A { 1.0 } else { 0.0 };
                if DFZ != 0.0 {
                    let DGA = if (if (if PB <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFS <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || RE != 0.0 { 1.0 } else { 0.0 };
                    if DGA != 0.0 {
                    } else {
                        let DGB = if HX != A { 1.0 } else { 0.0 };
                        if DGB != 0.0 {
                        } else {
                        }
                    }
                    let DGD = if (if (if PC <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFT <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || RF != 0.0 { 1.0 } else { 0.0 };
                    if DGD != 0.0 {
                    } else {
                        let DGE = if IB != A { 1.0 } else { 0.0 };
                        if DGE != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let DGF = if (if PB <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFS <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if DGF != 0.0 {
                    } else {
                        let DGG = if ID != A { 1.0 } else { 0.0 };
                        if DGG != 0.0 {
                        } else {
                        }
                    }
                    let DGH = if (if PC <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFT <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if DGH != 0.0 {
                    } else {
                        let DGI = if IC != A { 1.0 } else { 0.0 };
                        if DGI != 0.0 {
                        } else {
                        }
                    }
                }
                let DGK = if DGJ == A { 1.0 } else { 0.0 };
                if DGK != 0.0 {
                    let DGL = if (if PA <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFR <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if DGL != 0.0 {
                    } else {
                        let DGN = if DGM > (BFR / BSH) { 1.0 } else { 0.0 };
                        if DGN != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let DGO = if DGJ == B { 1.0 } else { 0.0 };
                    if DGO != 0.0 {
                        let DGP = if (if PA <= A { 1.0 } else { 0.0 }) != 0.0 || (if (if (if GQ == A { 1.0 } else { 0.0 }) != 0.0 && (if GP == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BFR == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if DGP != 0.0 {
                        } else {
                        }
                    } else {
                        let DGR = if (if PA <= A { 1.0 } else { 0.0 }) != 0.0 || (if (if (if GQ == A { 1.0 } else { 0.0 }) != 0.0 && (if GP == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BFR == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if DGR != 0.0 {
                        } else {
                        }
                        let DGT = GR * (B + (DGS * BCZ));
                        let DGU = if DGQ > A { 1.0 } else { 0.0 };
                        let DGX = if DGU != 0.0 {
                            let DGV = DGT - BRO;
                            DGV
                        } else {
                            let DGW = DGT - BRN;
                            DGW
                        };
                        let DGY = if DGX > A { 1.0 } else { 0.0 };
                        if DGY != 0.0 {
                        } else {
                        }
                    }
                }
                let DHA = BGI * DGZ;
                let DHE = BGM * DHB;
                let DHF = (BGQ * BV) * AA;
                let DHH = -DHG;
                let DHI = BQB.powf(DHH);
                let DHJ = if DHG == B { 1.0 } else { 0.0 };
                let DIL = if DHJ != 0.0 {
                    DHK
                } else {
                    let DHL = (B / (B - DHG)) * (B - (((BQE * DHG) * (B + DHG)) * DHI));
                    DHL
                };
                let DHN = -DHM;
                let DHO = BQB.powf(DHN);
                let DHP = if DHM == B { 1.0 } else { 0.0 };
                let DIZ = if DHP != 0.0 {
                    DHQ
                } else {
                    let DHR = (B / (B - DHM)) * (B - (((BQE * DHM) * (B + DHM)) * DHO));
                    DHR
                };
                let DHT = -DHS;
                let DHU = BQB.powf(DHT);
                let DHV = if DHS == B { 1.0 } else { 0.0 };
                let DJN = if DHV != 0.0 {
                    DHW
                } else {
                    let DHX = (B / (B - DHS)) * (B - (((BQE * DHS) * (B + DHS)) * DHU));
                    DHX
                };
                let DHY = if DHA > A { 1.0 } else { 0.0 };
                let DJR;
                if DHY != 0.0 {
                    let DHZ = BRN / BGV;
                    let DIB = if DHZ < DIA { 1.0 } else { 0.0 };
                    let DJS;
                    if DIB != 0.0 {
                        let DIC = B - DHZ;
                        let DID = if DHG != B { 1.0 } else { 0.0 };
                        let DJT;
                        if DID != 0.0 {
                            let DIE = if DHG == NZ { 1.0 } else { 0.0 };
                            let DIH = if DIE != 0.0 {
                                let DIF = B / (DIC.sqrt());
                                DIF
                            } else {
                                let DIG = rspice_limited_exp((DHH * (DIC.ln())));
                                DIG
                            };
                            let DII = ((BGV * DHA) * (B - (DIC * DIH))) / (B - DHG);
                            DJT = DII;
                        } else {
                            let DIJ = (BGV * DHA) * (-(DIC.ln()));
                            DJT = DIJ;
                        }
                        DJS = DJT;
                    } else {
                        let DIK = DHZ - B;
                        let DIM = (BGV * DHA) * (((DHI * DIK) * (((UA * DHG) * DIK) + (B + DHG))) + DIL);
                        DJS = DIM;
                    }
                    DJR = DJS;
                } else {
                    DJR = A;
                }
                let DIN = if DHE > A { 1.0 } else { 0.0 };
                let DJU;
                if DIN != 0.0 {
                    let DIO = BRN / BHA;
                    let DIP = if DIO < DIA { 1.0 } else { 0.0 };
                    let DJV;
                    if DIP != 0.0 {
                        let DIQ = B - DIO;
                        let DIR = if DHM != B { 1.0 } else { 0.0 };
                        let DJW;
                        if DIR != 0.0 {
                            let DIS = if DHM == NZ { 1.0 } else { 0.0 };
                            let DIV = if DIS != 0.0 {
                                let DIT = B / (DIQ.sqrt());
                                DIT
                            } else {
                                let DIU = rspice_limited_exp((DHN * (DIQ.ln())));
                                DIU
                            };
                            let DIW = ((BHA * DHE) * (B - (DIQ * DIV))) / (B - DHM);
                            DJW = DIW;
                        } else {
                            let DIX = (BHA * DHE) * (-(DIQ.ln()));
                            DJW = DIX;
                        }
                        DJV = DJW;
                    } else {
                        let DIY = DIO - B;
                        let DJA = (BHA * DHE) * (((DHO * DIY) * (((UA * DHM) * DIY) + (B + DHM))) + DIZ);
                        DJV = DJA;
                    }
                    DJU = DJV;
                } else {
                    DJU = A;
                }
                let DJB = if DHF > A { 1.0 } else { 0.0 };
                let DJX;
                if DJB != 0.0 {
                    let DJC = BRN / BHF;
                    let DJD = if DJC < DIA { 1.0 } else { 0.0 };
                    let DJY;
                    if DJD != 0.0 {
                        let DJE = B - DJC;
                        let DJF = if DHS != B { 1.0 } else { 0.0 };
                        let DJZ;
                        if DJF != 0.0 {
                            let DJG = if DHS == NZ { 1.0 } else { 0.0 };
                            let DJJ = if DJG != 0.0 {
                                let DJH = B / (DJE.sqrt());
                                DJH
                            } else {
                                let DJI = rspice_limited_exp((DHT * (DJE.ln())));
                                DJI
                            };
                            let DJK = ((BHF * DHF) * (B - (DJE * DJJ))) / (B - DHS);
                            DJZ = DJK;
                        } else {
                            let DJL = (BHF * DHF) * (-(DJE.ln()));
                            DJZ = DJL;
                        }
                        DJY = DJZ;
                    } else {
                        let DJM = DJC - B;
                        let DJO = (BHF * DHF) * (((DHU * DJM) * (((UA * DHS) * DJM) + (B + DHS))) + DJN);
                        DJY = DJO;
                    }
                    DJX = DJY;
                } else {
                    DJX = A;
                }
                let DKA = ((DJR + DJU) + DJX) + ((DJP * DJQ) * AA);
                let DKC = BGJ * DKB;
                let DKG = BGN * DKD;
                let DKH = (BGR * BV) * AA;
                let DKJ = -DKI;
                let DKK = BQB.powf(DKJ);
                let DKL = if DKI == B { 1.0 } else { 0.0 };
                let DLM = if DKL != 0.0 {
                    DKM
                } else {
                    let DKN = (B / (B - DKI)) * (B - (((BQE * DKI) * (B + DKI)) * DKK));
                    DKN
                };
                let DKP = -DKO;
                let DKQ = BQB.powf(DKP);
                let DKR = if DKO == B { 1.0 } else { 0.0 };
                let DMA = if DKR != 0.0 {
                    DKS
                } else {
                    let DKT = (B / (B - DKO)) * (B - (((BQE * DKO) * (B + DKO)) * DKQ));
                    DKT
                };
                let DKV = -DKU;
                let DKW = BQB.powf(DKV);
                let DKX = if DKU == B { 1.0 } else { 0.0 };
                let DMO = if DKX != 0.0 {
                    DKY
                } else {
                    let DKZ = (B / (B - DKU)) * (B - (((BQE * DKU) * (B + DKU)) * DKW));
                    DKZ
                };
                let DLA = if DKC > A { 1.0 } else { 0.0 };
                let DMR;
                if DLA != 0.0 {
                    let DLB = BRO / BGX;
                    let DLC = if DLB < DIA { 1.0 } else { 0.0 };
                    let DMS;
                    if DLC != 0.0 {
                        let DLD = B - DLB;
                        let DLE = if DKI != B { 1.0 } else { 0.0 };
                        let DMT;
                        if DLE != 0.0 {
                            let DLF = if DKI == NZ { 1.0 } else { 0.0 };
                            let DLI = if DLF != 0.0 {
                                let DLG = B / (DLD.sqrt());
                                DLG
                            } else {
                                let DLH = rspice_limited_exp((DKJ * (DLD.ln())));
                                DLH
                            };
                            let DLJ = ((BGX * DKC) * (B - (DLD * DLI))) / (B - DKI);
                            DMT = DLJ;
                        } else {
                            let DLK = (BGX * DKC) * (-(DLD.ln()));
                            DMT = DLK;
                        }
                        DMS = DMT;
                    } else {
                        let DLL = DLB - B;
                        let DLN = (BGX * DKC) * (((DKK * DLL) * (((UA * DKI) * DLL) + (B + DKI))) + DLM);
                        DMS = DLN;
                    }
                    DMR = DMS;
                } else {
                    DMR = A;
                }
                let DLO = if DKG > A { 1.0 } else { 0.0 };
                let DMU;
                if DLO != 0.0 {
                    let DLP = BRO / BHC;
                    let DLQ = if DLP < DIA { 1.0 } else { 0.0 };
                    let DMV;
                    if DLQ != 0.0 {
                        let DLR = B - DLP;
                        let DLS = if DKO != B { 1.0 } else { 0.0 };
                        let DMW;
                        if DLS != 0.0 {
                            let DLT = if DKO == NZ { 1.0 } else { 0.0 };
                            let DLW = if DLT != 0.0 {
                                let DLU = B / (DLR.sqrt());
                                DLU
                            } else {
                                let DLV = rspice_limited_exp((DKP * (DLR.ln())));
                                DLV
                            };
                            let DLX = ((BHC * DKG) * (B - (DLR * DLW))) / (B - DKO);
                            DMW = DLX;
                        } else {
                            let DLY = (BHC * DKG) * (-(DLR.ln()));
                            DMW = DLY;
                        }
                        DMV = DMW;
                    } else {
                        let DLZ = DLP - B;
                        let DMB = (BHC * DKG) * (((DKQ * DLZ) * (((UA * DKO) * DLZ) + (B + DKO))) + DMA);
                        DMV = DMB;
                    }
                    DMU = DMV;
                } else {
                    DMU = A;
                }
                let DMC = if DKH > A { 1.0 } else { 0.0 };
                let DMX;
                if DMC != 0.0 {
                    let DMD = BRO / BHH;
                    let DME = if DMD < DIA { 1.0 } else { 0.0 };
                    let DMY;
                    if DME != 0.0 {
                        let DMF = B - DMD;
                        let DMG = if DKU != B { 1.0 } else { 0.0 };
                        let DMZ;
                        if DMG != 0.0 {
                            let DMH = if DKU == NZ { 1.0 } else { 0.0 };
                            let DMK = if DMH != 0.0 {
                                let DMI = B / (DMF.sqrt());
                                DMI
                            } else {
                                let DMJ = rspice_limited_exp((DKV * (DMF.ln())));
                                DMJ
                            };
                            let DML = ((BHH * DKH) * (B - (DMF * DMK))) / (B - DKU);
                            DMZ = DML;
                        } else {
                            let DMM = (BHH * DKH) * (-(DMF.ln()));
                            DMZ = DMM;
                        }
                        DMY = DMZ;
                    } else {
                        let DMN = DMD - B;
                        let DMP = (BHH * DKH) * (((DKW * DMN) * (((UA * DKU) * DMN) + (B + DKU))) + DMO);
                        DMY = DMP;
                    }
                    DMX = DMY;
                } else {
                    DMX = A;
                }
                let DNA = ((DMR + DMU) + DMX) + ((DJP * DMQ) * AA);
                let DNQ;
                let DNR;
                let DNS;
                if CIU != 0.0 {
                    DNQ = DNB;
                    DNR = A;
                    DNS = A;
                } else {
                    let DND = NZ * (DDG / DNC);
                    let DNG = DNF * DDG;
                    let DNH = (B - DDH) * (DNE - (NZ * DNG));
                    let DNK = DNB + (NZ * ((DNI * DDG) * ((((DND * DDH) * BDI) - B) + DDH)));
                    let DNL = DNG * BYR;
                    let DNN = NZ * (((DDH * DDH) * (DNE - (DNL * ((B - DND) - (DNM * (DND * DND)))))) + (DNH * (B + DDH)));
                    let DNO = DNK - ((DDH * (DNE + (DNL * DND))) + DNH);
                    let DNP = (DNK - DNO) - DNN;
                    DNQ = DNO;
                    DNR = DNP;
                    DNS = DNN;
                }
                let DNT = ((DNR + DNS) + (parameters[231] * (NZ * (DNQ + (((DNQ * DNQ) + 2.5000000000000005e-3f64).sqrt()))))) / parameters[230];
                let DNW = ((AA * BQ) * BN) + DNV;
                let DNX = L / DNU;
                let DNY = ((-DNW) * DNX) * DNQ;
                let DNZ = DNW * (3.453133246992e-11f64 / (((DNU * Q) / K) + (((parameters[228] * 1.9e-9f64) / (B + ((NZ * (DNT + (((DNT * DNT) + 4e-6f64).sqrt()))).powf((7e-1f64 * parameters[229]))))) / O)));
                let DOA = if DGQ > A { 1.0 } else { 0.0 };
                let DOH;
                let DOI;
                if DOA != 0.0 {
                    let DOB = -DNZ;
                    let DOC = DOB * DNR;
                    let DOD = DOB * DNS;
                    DOH = DOC;
                    DOI = DOD;
                } else {
                    let DOE = -DNZ;
                    let DOF = DOE * DNS;
                    let DOG = DOE * DNR;
                    DOH = DOF;
                    DOI = DOG;
                }
                let DOK = if DOJ == 0.0 { 1.0 } else { 0.0 };
                if DOK != 0.0 {
                } else {
                }
                let DOL = BQ / BAO;
                let DON = if DOM == A { 1.0 } else { 0.0 };
                if DON != 0.0 {
                } else {
                }
                let DOQ = (BN - DOO) + (AY * DOP);
                let DOR = if KW > A { 1.0 } else { 0.0 };
                let DOV = if DOR != 0.0 {
                    let DOS = (BRW * BBM) * ((if (LP / KW) >= BAF { (LP / KW) } else { BAF }).ln());
                    DOS
                } else {
                    let DOT = (BRW * BBM) * ((if ((((-LP) * KW) / BBT) / BBT) >= BAF { ((((-LP) * KW) / BBT) / BBT) } else { BAF }).ln());
                    DOT
                };
                let DPA = (((KX * DOX) * (DOW / BTY)) * (((DOL * AA) * DOQ) + DOY)) * ((DOU - DOV) - DOZ);
                let DPB = if parameters[47] != A { 1.0 } else { 0.0 };
                if DPB != 0.0 {
                    let DPC = if (BMP - V) > A { 1.0 } else { 0.0 };
                    if DPC != 0.0 {
                    } else {
                    }
                    let DPD = if (BNG - V) > A { 1.0 } else { 0.0 };
                    if DPD != 0.0 {
                    } else {
                    }
                    let DPE = if SU != A { 1.0 } else { 0.0 };
                    if DPE != 0.0 {
                    } else {
                    }
                } else {
                }
                let DPF = if BCG == B { 1.0 } else { 0.0 };
                let EQT;
                let EQV;
                let ERF;
                let ERG;
                let ESZ;
                let EVG;
                let EVJ;
                let EVL;
                let EWE;
                let EXO;
                let EYA;
                let EYL;
                let FDM;
                let FDR;
                let FZL;
                let HXE;
                let HXS;
                if DPF != 0.0 {
                    let DPH = BSP * BBN;
                    let DPI = (BRE * BBN) - ((DPG + BVH) * BBN);
                    let DPJ = (if (RR / BBT) >= BAF { (RR / BBT) } else { BAF }).ln();
                    let DPK = ((((3.204352924e-19f64 * J) * RR) * BBN).sqrt()) / N;
                    let DPL = (BRT * BBN) - (DN * BBN);
                    let DPM = B / DPK;
                    let DPN = DPK * DPK;
                    let DPO = B / DPN;
                    let DPP = DPK / BWA;
                    let DPQ = B + (DPP * BWC);
                    let DPR = BWE * DPQ;
                    let DPT = B / DPP;
                    let DPU = DPP * DPP;
                    let DPV = B / (DPS + (DPP * BWJ));
                    let DPW = DPL.abs();
                    let DPX = if DPW <= DPR { 1.0 } else { 0.0 };
                    let DRD;
                    if DPX != 0.0 {
                        let DPY = -DPL;
                        let DPZ = (DPY * DPT) * (B + (DPP * (DPY / ((8.485281374238571e0f64 * DPQ) * DPQ))));
                        DRD = DPZ;
                    } else {
                        let DQA = if DPL < (-DPR) { 1.0 } else { 0.0 };
                        let DRE = if DQA != 0.0 {
                            let DQB = -DPL;
                            let DQC = (DPS * DQB) * DPT;
                            let DQD = DQC - UJ;
                            let DQE = NZ * ((DQC + SS) - (((DQD * DQD) + BWT).sqrt()));
                            let DQF = DQB - DQE;
                            let DQG = (DQF * DQF) + (DPU * (DQE + B));
                            let DQH = (AY * DQF) - DPU;
                            let DQI = ((if (DQG / DPU) >= BAF { (DQG / DPU) } else { BAF }).ln()) - DQE;
                            let DQJ = DQG + DQH;
                            let DQK = DQH * DQH;
                            let DQL = (DQJ * DQJ) + (DQI * ((NZ * DQK) - DQG));
                            let DQM = DQE + (((DQG * DQJ) * DQI) / (DQL + (((((DQJ / DQL) * DQI) * DQI) * DQH) * ((DQK * BDI) - DQG))));
                            let DQN = rspice_limited_exp(DQM);
                            let DQO = DQB - DQM;
                            let DQP = (AY * DQO) + (DPU * (DQN - B));
                            let DQQ = (DQO * DQO) + (DPU * ((DQM + B) - DQN));
                            let DQR = -(DQM + (AY * (DQQ / (DQP + (((DQP * DQP) - (UI * ((B - ((DPU * NZ) * DQN)) * DQQ))).sqrt())))));
                            DQR
                        } else {
                            let DQS = DPU * NZ;
                            let DQT = (DPL + DQS) - (DPP * (((DPL + (DPU * OQ)) - (B - (rspice_limited_exp((-((DPL * DPT) * (B + (((((DPQ * DPS) * DPV) - B) * DPV) * DPL)))))))).sqrt()));
                            let DQU = rspice_limited_exp((-DQT));
                            let DQV = DPL - DQT;
                            let DQW = (AY * DQV) + (DPU * (B - DQU));
                            let DQX = (DQV * DQV) - (DPU * ((DQT - B) + DQU));
                            let DQY = DQT + (AY * (DQX / (DQW + (((DQW * DQW) - (UI * ((B - (DQS * DQU)) * DQX))).sqrt()))));
                            DQY
                        };
                        DRD = DRE;
                    }
                    let DQZ = if DPW < DPR { 1.0 } else { 0.0 };
                    let DRI = if DQZ != 0.0 {
                        let DRA = -DPL;
                        let DRB = (DRA * DPT) * (B + (DPP * (DRA / ((8.485281374238571e0f64 * DPQ) * DPQ))));
                        DRB
                    } else {
                        let DRC = BWA * BWA;
                        let DRF = DPL - DRD;
                        let DRG = rspice_limited_exp((-DRD));
                        let DRH = DRD - ((((((DRC * DRF) * DRF) * DPM) * DPM) - ((DRG + DRD) - B)) / ((DRG + ((DRC * ((AY * DRD) - (AY * DPL))) / DPN)) - B));
                        DRH
                    };
                    let DRJ = DRI * BBM;
                    let DRK = B + (DPK * BWC);
                    let DRL = B / DRK;
                    let DRM = (AY * DPJ) / BUS;
                    let DRN = DRM + DPH;
                    let DRO = rspice_limited_exp((-DRN));
                    let DRP = BDB * DRK;
                    let DRQ = C * RR;
                    let DRR = ((BYI * ((DRQ * BTV) * BTV)) / (BCV * BBM)) + (BYJ / BBM);
                    let DRS = BVZ * DPL;
                    let DRT = DRR - DRS;
                    let DRU = DRT + (DPK * ((((rspice_limited_exp((-DRT))) + DRT) - B).sqrt()));
                    let DRV = if DRT < DRN { 1.0 } else { 0.0 };
                    let DWX;
                    if DRV != 0.0 {
                        let DRW = if DPI < DRU { 1.0 } else { 0.0 };
                        let DWY;
                        if DRW != 0.0 {
                            let DRX = if (DPI.abs()) <= DRP { 1.0 } else { 0.0 };
                            let DWZ;
                            if DRX != 0.0 {
                                let DRY = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * (((DRL * DRL) * BYR) * BWC)));
                                DWZ = DRY;
                            } else {
                                let DRZ = if DPI < (-DRP) { 1.0 } else { 0.0 };
                                let DXA = if DRZ != 0.0 {
                                    let DSA = -DPI;
                                    let DSB = BYV * (DSA * DRL);
                                    let DSC = DSB - UJ;
                                    let DSD = NZ * ((DSB + SS) - (((DSC * DSC) + BWT).sqrt()));
                                    let DSE = DSA - DSD;
                                    let DSF = (DSE * DSE) + (DPN * (DSD + B));
                                    let DSG = (AY * DSE) - DPN;
                                    let DSH = (-DSD) + ((if (DSF * DPO) >= BAF { (DSF * DPO) } else { BAF }).ln());
                                    let DSI = DSF + DSG;
                                    let DSJ = DSG * DSG;
                                    let DSK = (DSI * DSI) + (DSH * ((NZ * DSJ) - DSF));
                                    let DSL = DSD + (((DSF * DSI) * DSH) / (DSK + (((((DSI / DSK) * DSH) * DSH) * DSG) * ((DSJ * BDI) - DSF))));
                                    let DSM = rspice_limited_exp(DSL);
                                    let DSN = DSL * DSL;
                                    let DSO = B / (AY + DSN);
                                    let DSP = DSN * DSO;
                                    let DSQ = DSA - DSL;
                                    let DSR = DRO * (B / DSM);
                                    let DSS = (AY * DSQ) + (DPN * (((DSM - B) - DSR) + (DRO * (B - (UI * ((DSL * DSO) * DSO))))));
                                    let DST = (DSQ * DSQ) - (DPN * ((((DSM - DSL) - B) + DSR) + (DRO * ((DSL - B) - DSP))));
                                    let DSU = (-DSL) - (AY * (DST / (DSS + (((DSS * DSS) - (AY * (DST * (AY - (DPN * ((DSM + DSR) - (DRO * ((((UT * DSO) - (BZL * DSP)) * DSO) * DSO)))))))).sqrt()))));
                                    DSU
                                } else {
                                    let DSV = B / (BYV + (DPK * BWJ));
                                    let DSW = (DPI + (DPN * NZ)) - (DPK * (((DPI + (DPN * OQ)) - (B - (rspice_limited_exp((-((DPI * DRL) * (B + (((((DRK * BYV) * DSV) - B) * DSV) * DPI)))))))).sqrt()));
                                    let DSX = DRN + UH;
                                    let DSY = DSW - DSX;
                                    let DSZ = (NZ * ((DSW + DSX) - (((DSY * DSY) + UA).sqrt()))) - (NZ * (DSX - (((DSX * DSX) + UA).sqrt())));
                                    let DTA = DPI - DSZ;
                                    let DTB = rspice_limited_exp((-DSZ));
                                    let DTC = DSZ * DSZ;
                                    let DTD = B / (AY + DTC);
                                    let DTE = DTC * DTD;
                                    let DTF = if CAB >= ((DTA * DTA) - (DPN * (((DTB + DSZ) - B) - (DRO * ((DSZ + B) + DTE))))) { CAB } else { ((DTA * DTA) - (DPN * (((DTB + DSZ) - B) - (DRO * ((DSZ + B) + DTE))))) };
                                    let DTG = (AY * DTA) + (DPN * ((B - DTB) - (DRO * (B + (UI * ((DSZ * DTD) * DTD))))));
                                    let DTH = (DRN - DSZ) + ((if (DTF / DPN) >= BAF { (DTF / DPN) } else { BAF }).ln());
                                    let DTI = DTF + DTG;
                                    let DTJ = DTG * DTG;
                                    let DTK = DTF * (B - (NZ * (DPN * (DTB - (DRO * ((((UT * DTD) - (BZL * DTE)) * DTD) * DTD))))));
                                    let DTL = (DTI * DTI) + (DTH * ((NZ * DTJ) - DTK));
                                    let DTM = DSZ + (((DTF * DTI) * DTH) / (DTL + (((((DTI / DTL) * DTH) * DTH) * DTG) * ((DTJ * BDI) - DTK))));
                                    let DTN = B / (rspice_limited_exp(DTM));
                                    let DTO = rspice_limited_exp((DTM - DRN));
                                    let DTP = DTM * DTM;
                                    let DTQ = B / (AY + DTP);
                                    let DTR = DTP * DTQ;
                                    let DTS = DPI - DTM;
                                    let DTT = (AY * DTS) + (DPN * (((B - DTN) + DTO) - (DRO * (B + (UI * ((DTM * DTQ) * DTQ))))));
                                    let DTU = (DTS * DTS) - (DPN * ((((DTN + DTM) - B) + DTO) - (DRO * ((DTM + B) + DTR))));
                                    let DTV = DTM + (AY * (DTU / (DTT + (((DTT * DTT) - (AY * (DTU * (AY - (DPN * ((DTN + DTO) - (DRO * ((((UT * DTQ) - (BZL * DTR)) * DTQ) * DTQ)))))))).sqrt()))));
                                    DTV
                                };
                                DWZ = DXA;
                            }
                            DWY = DWZ;
                        } else {
                            let DTW = BWA * BWA;
                            let DTX = DRT - (DRJ * BBN);
                            let DTY = DPI - (DPK * ((((rspice_limited_exp((-DTX))) + DTX) - B).sqrt()));
                            let DTZ = DRN + UH;
                            let DUA = DTY - DTZ;
                            let DUB = NZ * ((DTY + DTZ) - (((DUA * DUA) + BUA).sqrt()));
                            let DUC = DPI - DUB;
                            let DUD = (DPL - DUB) + DRT;
                            let DUE = ((DUC * DUC) - ((DTW * DUD) * DUD)) - (DPN * DRT);
                            let DUF = AY * DTW;
                            let DUG = (AY * DUC) - (DUF * DUD);
                            let DUH = DUG * DUG;
                            let DUI = B - DTW;
                            let DUJ = if DUE < A { 1.0 } else { 0.0 };
                            let DUK = if DUJ != 0.0 {
                                A
                            } else {
                                DUE
                            };
                            let DUL = DUK + DUG;
                            let DUM = DUK * DUI;
                            let DUN = (((DUL * DUL) / ((DRN - DUB) + ((if (DUK * DPO) >= BAF { (DUK * DPO) } else { BAF }).ln()))) + (NZ * DUH)) - DUM;
                            let DUO = DUB + ((DUL * DUK) / (DUN + (((DUG * DUL) / DUN) * ((BDI * DUH) - DUM))));
                            let DUP = rspice_limited_exp((DUO - DRN));
                            let DUQ = DPI - DUO;
                            let DUR = (DPL - DUO) + DRT;
                            let DUS = DPN * DUP;
                            let DUT = ((AY * DUQ) - (DUF * DUR)) + DUS;
                            let DUU = AY * (((DUQ * DUQ) - ((DTW * DUR) * DUR)) - (DPN * (DRT + DUP)));
                            let DUV = DUO + (DUU / (DUT + (((DUT * DUT) - (DUU * ((AY - DUF) - DUS))).sqrt())));
                            DWY = DUV;
                        }
                        DWX = DWY;
                    } else {
                        let DUW = if (DPI.abs()) <= DRP { 1.0 } else { 0.0 };
                        let DXB;
                        if DUW != 0.0 {
                            let DUX = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * (((DRL * DRL) * BYR) * BWC)));
                            DXB = DUX;
                        } else {
                            let DUY = if DPI < (-DRP) { 1.0 } else { 0.0 };
                            let DXC = if DUY != 0.0 {
                                let DUZ = -DPI;
                                let DVA = BYV * (DUZ * DRL);
                                let DVB = DVA - UJ;
                                let DVC = NZ * ((DVA + SS) - (((DVB * DVB) + BWT).sqrt()));
                                let DVD = DUZ - DVC;
                                let DVE = (DVD * DVD) + (DPN * (DVC + B));
                                let DVF = (AY * DVD) - DPN;
                                let DVG = (-DVC) + ((if (DVE * DPO) >= BAF { (DVE * DPO) } else { BAF }).ln());
                                let DVH = DVE + DVF;
                                let DVI = DVF * DVF;
                                let DVJ = (DVH * DVH) + (DVG * ((NZ * DVI) - DVE));
                                let DVK = DVC + (((DVE * DVH) * DVG) / (DVJ + (((((DVH / DVJ) * DVG) * DVG) * DVF) * ((DVI * BDI) - DVE))));
                                let DVL = rspice_limited_exp(DVK);
                                let DVM = DVK * DVK;
                                let DVN = B / (AY + DVM);
                                let DVO = DVM * DVN;
                                let DVP = DUZ - DVK;
                                let DVQ = DRO * (B / DVL);
                                let DVR = (AY * DVP) + (DPN * (((DVL - B) - DVQ) + (DRO * (B - (UI * ((DVK * DVN) * DVN))))));
                                let DVS = (DVP * DVP) - (DPN * ((((DVL - DVK) - B) + DVQ) + (DRO * ((DVK - B) - DVO))));
                                let DVT = (DVR * DVR) - (AY * (DVS * (AY - (DPN * ((DVL + DVQ) - (DRO * ((((UT * DVN) - (BZL * DVO)) * DVN) * DVN)))))));
                                let DVU = (-DVK) - (AY * (DVS / (DVR + (((((DVT * DVT) + CCR).sqrt()) - CCS).sqrt()))));
                                DVU
                            } else {
                                let DVV = B / (BYV + (DPK * BWJ));
                                let DVW = (DPI + (DPN * NZ)) - (DPK * (((DPI + (DPN * OQ)) - (B - (rspice_limited_exp((-((DPI * DRL) * (B + (((((DRK * BYV) * DVV) - B) * DVV) * DPI)))))))).sqrt()));
                                let DVX = DRN + UH;
                                let DVY = DVW - DVX;
                                let DVZ = (NZ * ((DVW + DVX) - (((DVY * DVY) + UA).sqrt()))) - (NZ * (DVX - (((DVX * DVX) + UA).sqrt())));
                                let DWA = DPI - DVZ;
                                let DWB = rspice_limited_exp((-DVZ));
                                let DWC = DVZ * DVZ;
                                let DWD = B / (AY + DWC);
                                let DWE = DWC * DWD;
                                let DWF = if CAB >= ((DWA * DWA) - (DPN * (((DWB + DVZ) - B) - (DRO * ((DVZ + B) + DWE))))) { CAB } else { ((DWA * DWA) - (DPN * (((DWB + DVZ) - B) - (DRO * ((DVZ + B) + DWE))))) };
                                let DWG = (AY * DWA) + (DPN * ((B - DWB) - (DRO * (B + (UI * ((DVZ * DWD) * DWD))))));
                                let DWH = (DRN - DVZ) + ((if (DWF / DPN) >= BAF { (DWF / DPN) } else { BAF }).ln());
                                let DWI = DWF + DWG;
                                let DWJ = DWG * DWG;
                                let DWK = DWF * (B - (NZ * (DPN * (DWB - (DRO * ((((UT * DWD) - (BZL * DWE)) * DWD) * DWD))))));
                                let DWL = (DWI * DWI) + (DWH * ((NZ * DWJ) - DWK));
                                let DWM = DVZ + (((DWF * DWI) * DWH) / (DWL + (((((DWI / DWL) * DWH) * DWH) * DWG) * ((DWJ * BDI) - DWK))));
                                let DWN = B / (rspice_limited_exp(DWM));
                                let DWO = rspice_limited_exp((DWM - DRN));
                                let DWP = DWM * DWM;
                                let DWQ = B / (AY + DWP);
                                let DWR = DWP * DWQ;
                                let DWS = DPI - DWM;
                                let DWT = (AY * DWS) + (DPN * (((B - DWN) + DWO) - (DRO * (B + (UI * ((DWM * DWQ) * DWQ))))));
                                let DWU = (DWS * DWS) - (DPN * ((((DWN + DWM) - B) + DWO) - (DRO * ((DWM + B) + DWR))));
                                let DWV = (DWT * DWT) - (AY * (DWU * (AY - (DPN * ((DWN + DWO) - (DRO * ((((UT * DWQ) - (BZL * DWR)) * DWQ) * DWQ)))))));
                                let DWW = DWM + (AY * (DWU / (DWT + (((((DWV * DWV) + CCR).sqrt()) - CCS).sqrt()))));
                                DWW
                            };
                            DXB = DXC;
                        }
                        DWX = DXB;
                    }
                    let DXD = ((DRL * DRL) * BYR) * BWC;
                    let DXE = (((BYI * (DRQ * CED)) / BCV) + BYJ) - ((BVZ * (DPL * BBM)) * EB);
                    let DXF = CEF * DRJ;
                    let DXG = DXE + DXF;
                    let DXH = DPI.abs();
                    let DXI = if DXH <= BWE { 1.0 } else { 0.0 };
                    let DYT;
                    let ENT;
                    if DXI != 0.0 {
                        let DXJ = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD));
                        DYT = DXJ;
                        ENT = ENU;
                    } else {
                        let DXK = ((DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD))) * (NZ * (((-5e0f64 * (DPI - AY)).tanh()) + ((UA * (DPI + AY)).tanh())));
                        let DXL = ((DWX * BBM) - DXG) / BBM;
                        let DXM = rspice_limited_exp(DXL);
                        let DXN = if DXL > CEP { 1.0 } else { 0.0 };
                        let DXW;
                        if DXN != 0.0 {
                            DXW = DXL;
                        } else {
                            let DXO = if DXL < -3.7e1f64 { 1.0 } else { 0.0 };
                            let DXX = if DXO != 0.0 {
                                let DXP = DXL.exp();
                                DXP
                            } else {
                                let DXQ = (B + (DXL.exp())).ln();
                                DXQ
                            };
                            DXW = DXX;
                        }
                        let DXR = ((DXK * BBM) - DXG) / BBM;
                        let DXS = if DXR > CEP { 1.0 } else { 0.0 };
                        let DXY;
                        if DXS != 0.0 {
                            DXY = DXR;
                        } else {
                            let DXT = if DXR < -3.7e1f64 { 1.0 } else { 0.0 };
                            let DXZ = if DXT != 0.0 {
                                let DXU = DXR.exp();
                                DXU
                            } else {
                                let DXV = (B + (DXR.exp())).ln();
                                DXV
                            };
                            DXY = DXZ;
                        }
                        let DYA = -((DRJ / BBM) + ((DXW - DXY) / CEF));
                        let DYB = rspice_limited_exp(DYA);
                        let DYC = rspice_limited_exp((-DWX));
                        let DYD = DWX * DWX;
                        let DYE = B / (DYD + AY);
                        let DYF = rspice_limited_exp((DWX - DRN));
                        let DYG = DPI - DWX;
                        let DYH = DPL + DYA;
                        let DYI = DYE * DYD;
                        let DYJ = ((DYG * DYG) - (((BWA * BWA) * DYH) * DYH)) - (DPN * (((((DYC - DYB) + DWX) + DYA) + DYF) - (DRO * ((DWX + B) + DYI))));
                        let DYK = B + DXM;
                        let DYL = CEF * DYK;
                        let DYM = AY * DWX;
                        let DYN = DXM / DYL;
                        let DYO = DXM * DYB;
                        let DYP = (((((((AY * DXM) * DYH) * BWA) * BWA) / DYL) - (AY * DPI)) + DYM) - (DPN * (((((DYF + (DRO * ((((-2e0f64 * DWX) * DYE) + ((((DYM * DWX) * DWX) * DYE) * DYE)) - B))) - DYC) - DYN) + (DYO / DYL)) + B));
                        let DYQ = ((AY * BWA) * BWA) * DXM;
                        let DYR = DYQ * DXM;
                        let DYS = DWX - ((DYJ / DYP) * (B + ((DYJ * ((((((DYQ * DYH) / DYL) - (DYR / ((DYL * CEF) * DYK))) - (DPN * (((DYC + DYF) - (((AY * DRO) * DYE) * (B - (DYI * (UA - ((UI * DYD) * DYE)))))) - (DYN * (((B - (DXM / DYK)) - DYB) + ((DYO / DYK) * (B + (B / CEF)))))))) - ((DYR * DYH) / (DYL * DYK))) + AY)) / ((AY * DYP) * DYP))));
                        DYT = DYS;
                        ENT = DXK;
                    }
                    let EAE;
                    let ENS;
                    if DXI != 0.0 {
                        let DYU = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD));
                        EAE = DYU;
                        ENS = ENT;
                    } else {
                        let DYV = ((DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD))) * (NZ * (((-5e0f64 * (DPI - AY)).tanh()) + ((UA * (DPI + AY)).tanh())));
                        let DYW = ((DYT * BBM) - DXG) / BBM;
                        let DYX = rspice_limited_exp(DYW);
                        let DYY = if DYW > CEP { 1.0 } else { 0.0 };
                        let DZH;
                        if DYY != 0.0 {
                            DZH = DYW;
                        } else {
                            let DYZ = if DYW < -3.7e1f64 { 1.0 } else { 0.0 };
                            let DZI = if DYZ != 0.0 {
                                let DZA = DYW.exp();
                                DZA
                            } else {
                                let DZB = (B + (DYW.exp())).ln();
                                DZB
                            };
                            DZH = DZI;
                        }
                        let DZC = ((DYV * BBM) - DXG) / BBM;
                        let DZD = if DZC > CEP { 1.0 } else { 0.0 };
                        let DZJ;
                        if DZD != 0.0 {
                            DZJ = DZC;
                        } else {
                            let DZE = if DZC < -3.7e1f64 { 1.0 } else { 0.0 };
                            let DZK = if DZE != 0.0 {
                                let DZF = DZC.exp();
                                DZF
                            } else {
                                let DZG = (B + (DZC.exp())).ln();
                                DZG
                            };
                            DZJ = DZK;
                        }
                        let DZL = -((DRJ / BBM) + ((DZH - DZJ) / CEF));
                        let DZM = rspice_limited_exp(DZL);
                        let DZN = rspice_limited_exp((-DYT));
                        let DZO = DYT * DYT;
                        let DZP = B / (DZO + AY);
                        let DZQ = rspice_limited_exp((DYT - DRN));
                        let DZR = DPI - DYT;
                        let DZS = DPL + DZL;
                        let DZT = DZP * DZO;
                        let DZU = ((DZR * DZR) - (((BWA * BWA) * DZS) * DZS)) - (DPN * (((((DZN - DZM) + DYT) + DZL) + DZQ) - (DRO * ((DYT + B) + DZT))));
                        let DZV = B + DYX;
                        let DZW = CEF * DZV;
                        let DZX = AY * DYT;
                        let DZY = DYX / DZW;
                        let DZZ = DYX * DZM;
                        let EAA = (((((((AY * DYX) * DZS) * BWA) * BWA) / DZW) - (AY * DPI)) + DZX) - (DPN * (((((DZQ + (DRO * ((((-2e0f64 * DYT) * DZP) + ((((DZX * DYT) * DYT) * DZP) * DZP)) - B))) - DZN) - DZY) + (DZZ / DZW)) + B));
                        let EAB = ((AY * BWA) * BWA) * DYX;
                        let EAC = EAB * DYX;
                        let EAD = DYT - ((DZU / EAA) * (B + ((DZU * ((((((EAB * DZS) / DZW) - (EAC / ((DZW * CEF) * DZV))) - (DPN * (((DZN + DZQ) - (((AY * DRO) * DZP) * (B - (DZT * (UA - ((UI * DZO) * DZP)))))) - (DZY * (((B - (DYX / DZV)) - DZM) + ((DZZ / DZV) * (B + (B / CEF)))))))) - ((EAC * DZS) / (DZW * DZV))) + AY)) / ((AY * EAA) * EAA))));
                        EAE = EAD;
                        ENS = DYV;
                    }
                    let EBQ;
                    let ENR;
                    if DXI != 0.0 {
                        let EAF = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD));
                        EBQ = EAF;
                        ENR = ENS;
                    } else {
                        let EAG = ((DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD))) * (NZ * (((-5e0f64 * (DPI - AY)).tanh()) + ((UA * (DPI + AY)).tanh())));
                        let EAH = ((EAE * BBM) - DXG) / BBM;
                        let EAI = rspice_limited_exp(EAH);
                        let EAJ = if EAH > CEP { 1.0 } else { 0.0 };
                        let EAS;
                        if EAJ != 0.0 {
                            EAS = EAH;
                        } else {
                            let EAK = if EAH < -3.7e1f64 { 1.0 } else { 0.0 };
                            let EAT = if EAK != 0.0 {
                                let EAL = EAH.exp();
                                EAL
                            } else {
                                let EAM = (B + (EAH.exp())).ln();
                                EAM
                            };
                            EAS = EAT;
                        }
                        let EAN = ((EAG * BBM) - DXG) / BBM;
                        let EAO = if EAN > CEP { 1.0 } else { 0.0 };
                        let EAU;
                        if EAO != 0.0 {
                            EAU = EAN;
                        } else {
                            let EAP = if EAN < -3.7e1f64 { 1.0 } else { 0.0 };
                            let EAV = if EAP != 0.0 {
                                let EAQ = EAN.exp();
                                EAQ
                            } else {
                                let EAR = (B + (EAN.exp())).ln();
                                EAR
                            };
                            EAU = EAV;
                        }
                        let EAW = -((DRJ / BBM) + ((EAS - EAU) / CEF));
                        let EAX = rspice_limited_exp(EAW);
                        let EAY = rspice_limited_exp((-EAE));
                        let EAZ = EAE * EAE;
                        let EBA = B / (EAZ + AY);
                        let EBB = rspice_limited_exp((EAE - DRN));
                        let EBC = DPI - EAE;
                        let EBD = DPL + EAW;
                        let EBE = EBA * EAZ;
                        let EBF = ((EBC * EBC) - (((BWA * BWA) * EBD) * EBD)) - (DPN * (((((EAY - EAX) + EAE) + EAW) + EBB) - (DRO * ((EAE + B) + EBE))));
                        let EBG = B + EAI;
                        let EBH = CEF * EBG;
                        let EBI = AY * EAE;
                        let EBJ = EAI / EBH;
                        let EBK = EAI * EAX;
                        let EBL = (((((((AY * EAI) * EBD) * BWA) * BWA) / EBH) - (AY * DPI)) + EBI) - (DPN * (((((EBB + (DRO * ((((-2e0f64 * EAE) * EBA) + ((((EBI * EAE) * EAE) * EBA) * EBA)) - B))) - EAY) - EBJ) + (EBK / EBH)) + B));
                        let EBM = ((AY * BWA) * BWA) * EAI;
                        let EBN = EBM * EAI;
                        let EBO = EAE - ((EBF / EBL) * (B + ((EBF * ((((((EBM * EBD) / EBH) - (EBN / ((EBH * CEF) * EBG))) - (DPN * (((EAY + EBB) - (((AY * DRO) * EBA) * (B - (EBE * (UA - ((UI * EAZ) * EBA)))))) - (EBJ * (((B - (EAI / EBG)) - EAX) + ((EBK / EBG) * (B + (B / CEF)))))))) - ((EBN * EBD) / (EBH * EBG))) + AY)) / ((AY * EBL) * EBL))));
                        EBQ = EBO;
                        ENR = EAG;
                    }
                    let EBP = CIR * BBM;
                    let EBR = if EBQ <= A { 1.0 } else { 0.0 };
                    let EPZ;
                    let EQA;
                    let EQB;
                    let EQE;
                    let EQG;
                    let ERH;
                    let ETA;
                    let EVH;
                    let EVK;
                    let EVM;
                    let EWF;
                    let EXP;
                    let EYB;
                    let EYM;
                    let FZM;
                    let HXF;
                    let HXT;
                    if EBR != 0.0 {
                        let EBS = (DPI - EBQ) * BBM;
                        EPZ = EBS;
                        EQA = DDG;
                        EQB = DNC;
                        EQE = DNF;
                        EQG = DNE;
                        ERH = ERI;
                        ETA = ETB;
                        EVH = EVI;
                        EVK = B;
                        EVM = B;
                        EWF = A;
                        EXP = B;
                        EYB = B;
                        EYM = EBP;
                        FZM = ENR;
                        HXF = CIW;
                        HXT = CIZ;
                    } else {
                        let EBT = EBQ * EBQ;
                        let EBU = B / (rspice_limited_exp(EBQ));
                        let EBV = (rspice_limited_exp((EBQ - DRN))) - (DRO * ((EBQ + B) + (EBT * (B / (AY + EBT)))));
                        let EBW = DPI - EBQ;
                        let EBX = (((EBW * EBW) * DPO) - EBV) - BDB;
                        let EBY = (NZ * (EBX + (((EBX * EBX) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                        let EBZ = DPK * (EBY.sqrt());
                        let ECA = ((DPN * EBV) * BBM) / ((DPK * ((EBY + EBV).sqrt())) + EBZ);
                        let ECB = EBZ * BBM;
                        let ECC = B + (((CJP + (CJQ * BSQ)) * (((CJM / BTW) * (ECB + (BDM * ECA))).powf(BEF))) + (CJS / (rspice_limited_exp((CJO * ((if (NZ * (B + (ECA / ECB))) >= BAF { (NZ * (B + (ECA / ECB))) } else { BAF }).ln()))))));
                        let ECD = ECC - B;
                        let ECE = NZ * ((ECC + B) + (((ECD * ECD) + 5.625e-7f64).sqrt()));
                        let ECF = B / (((BG * CJW).powf(FR)) * AA);
                        let ECG = BCT - BSQ;
                        let ECH = ECG - BQE;
                        let ECI = (NZ * ((ECG + BQE) + (((ECH * ECH) + 2.5000000000000005e-3f64).sqrt()))).sqrt();
                        let ECQ;
                        if QS != 0.0 {
                            ECQ = A;
                        } else {
                            let ECJ = (B / (B + (FP * ECA))) + (QL * (ECI - BCU));
                            let ECK = ((CJZ + (CKB * (ECJ + (((ECJ * ECJ) + BGT).sqrt())))) * ECF) * AA;
                            let ECL = ECK * BEW;
                            let ECM = if QR == AY { 1.0 } else { 0.0 };
                            let ECR = if ECM != 0.0 {
                                let ECN = ((CIZ + ECK) + CIW) * BEW;
                                ECN
                            } else {
                                ECL
                            };
                            ECQ = ECR;
                        }
                        let ECO = (((AY * CKH) / CKJ) * ECE) * AZ;
                        let ECP = GG * (ECA + BBS);
                        let ECS = if ECQ > A { 1.0 } else { 0.0 };
                        let EDD = if ECS != 0.0 {
                            let ECT = ((BG * CKH) * N) * ECQ;
                            let ECU = AY * ECT;
                            let ECV = (ECP + ECO) + ((UH * ECP) * ECT);
                            let ECW = (ECV - (((ECV * ECV) - ((AY * ECU) * (ECP * (ECO + ((AY * ECP) * ECT))))).sqrt())) / ECU;
                            ECW
                        } else {
                            let ECX = (ECO * ECP) / (ECO + ECP);
                            ECX
                        };
                        let ECY = if (if CKV == A { 1.0 } else { 0.0 }) != 0.0 && (if CKW == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EDF = if ECY != 0.0 {
                            B
                        } else {
                            let ECZ = AZ / (AZ + ((FA * BUJ).sqrt()));
                            let EDA = B + (((CKV * ECZ) - (((CKW * ECZ) * (ECA.powf(CKZ))) * BBM)) / (B + (CLA * BSQ)));
                            let EDB = EDA - BQB;
                            let EDC = NZ * ((EDA + BQB) + (((EDB * EDB) + 6.25e-8f64).sqrt()));
                            EDC
                        };
                        let EDE = EDD - BDB;
                        let EDG = ((NZ * (EDE + (((EDE * EDE) + 4.0000000000000007e-10f64).sqrt()))) + BDB) / EDF;
                        let EDH = BSE * ((B + (((BSE / EDG) + BX).powf((B / BFH)))).powf((-BFH)));
                        let EDI = DRM + ((EDH + BSP) * BBN);
                        let EDJ = -EDI;
                        let EDK = rspice_limited_exp(EDJ);
                        let EDL = BSE * BBN;
                        let EDM = BYH * (EDL + (DZ * BBN));
                        let EDN = (DRR - (DRS * EB)) + (CEF * EDM);
                        let EDO = EDN + (DPK * ((((rspice_limited_exp((-EDN))) + EDN) - B).sqrt()));
                        let EDP = if EDN < EDI { 1.0 } else { 0.0 };
                        let EIP;
                        if EDP != 0.0 {
                            let EDQ = if DPI < EDO { 1.0 } else { 0.0 };
                            let EIQ;
                            if EDQ != 0.0 {
                                let EDR = if DXH <= DRP { 1.0 } else { 0.0 };
                                let EIR;
                                if EDR != 0.0 {
                                    let EDS = (DPI * DRL) * (B + (((DPI * (B - EDK)) * DPK) * DXD));
                                    EIR = EDS;
                                } else {
                                    let EDT = if DPI < (-DRP) { 1.0 } else { 0.0 };
                                    let EIS = if EDT != 0.0 {
                                        let EDU = -DPI;
                                        let EDV = BYV * (EDU * DRL);
                                        let EDW = EDV - UJ;
                                        let EDX = NZ * ((EDV + SS) - (((EDW * EDW) + BWT).sqrt()));
                                        let EDY = EDU - EDX;
                                        let EDZ = (EDY * EDY) + (DPN * (EDX + B));
                                        let EEA = (AY * EDY) - DPN;
                                        let EEB = (-EDX) + ((if (EDZ * DPO) >= BAF { (EDZ * DPO) } else { BAF }).ln());
                                        let EEC = EDZ + EEA;
                                        let EED = EEA * EEA;
                                        let EEE = (EEC * EEC) + (EEB * ((NZ * EED) - EDZ));
                                        let EEF = EDX + (((EDZ * EEC) * EEB) / (EEE + (((((EEC / EEE) * EEB) * EEB) * EEA) * ((EED * BDI) - EDZ))));
                                        let EEG = rspice_limited_exp(EEF);
                                        let EEH = EEF * EEF;
                                        let EEI = B / (AY + EEH);
                                        let EEJ = EEH * EEI;
                                        let EEK = EDU - EEF;
                                        let EEL = EDK * (B / EEG);
                                        let EEM = (AY * EEK) + (DPN * (((EEG - B) - EEL) + (EDK * (B - (UI * ((EEF * EEI) * EEI))))));
                                        let EEN = (EEK * EEK) - (DPN * ((((EEG - EEF) - B) + EEL) + (EDK * ((EEF - B) - EEJ))));
                                        let EEO = (-EEF) - (AY * (EEN / (EEM + (((EEM * EEM) - (AY * (EEN * (AY - (DPN * ((EEG + EEL) - (EDK * ((((UT * EEI) - (BZL * EEJ)) * EEI) * EEI)))))))).sqrt()))));
                                        EEO
                                    } else {
                                        let EEP = B / (BYV + (DPK * BWJ));
                                        let EEQ = (DPI + (DPN * NZ)) - (DPK * (((DPI + (DPN * OQ)) - (B - (rspice_limited_exp((-((DPI * DRL) * (B + (((((DRK * BYV) * EEP) - B) * EEP) * DPI)))))))).sqrt()));
                                        let EER = EDI + UH;
                                        let EES = EEQ - EER;
                                        let EET = (NZ * ((EEQ + EER) - (((EES * EES) + UA).sqrt()))) - (NZ * (EER - (((EER * EER) + UA).sqrt())));
                                        let EEU = DPI - EET;
                                        let EEV = rspice_limited_exp((-EET));
                                        let EEW = EET * EET;
                                        let EEX = B / (AY + EEW);
                                        let EEY = EEW * EEX;
                                        let EEZ = if CAB >= ((EEU * EEU) - (DPN * (((EEV + EET) - B) - (EDK * ((EET + B) + EEY))))) { CAB } else { ((EEU * EEU) - (DPN * (((EEV + EET) - B) - (EDK * ((EET + B) + EEY))))) };
                                        let EFA = (AY * EEU) + (DPN * ((B - EEV) - (EDK * (B + (UI * ((EET * EEX) * EEX))))));
                                        let EFB = (EDI - EET) + ((if (EEZ / DPN) >= BAF { (EEZ / DPN) } else { BAF }).ln());
                                        let EFC = EEZ + EFA;
                                        let EFD = EFA * EFA;
                                        let EFE = EEZ * (B - (NZ * (DPN * (EEV - (EDK * ((((UT * EEX) - (BZL * EEY)) * EEX) * EEX))))));
                                        let EFF = (EFC * EFC) + (EFB * ((NZ * EFD) - EFE));
                                        let EFG = EET + (((EEZ * EFC) * EFB) / (EFF + (((((EFC / EFF) * EFB) * EFB) * EFA) * ((EFD * BDI) - EFE))));
                                        let EFH = B / (rspice_limited_exp(EFG));
                                        let EFI = rspice_limited_exp((EFG - EDI));
                                        let EFJ = EFG * EFG;
                                        let EFK = B / (AY + EFJ);
                                        let EFL = EFJ * EFK;
                                        let EFM = DPI - EFG;
                                        let EFN = (AY * EFM) + (DPN * (((B - EFH) + EFI) - (EDK * (B + (UI * ((EFG * EFK) * EFK))))));
                                        let EFO = (EFM * EFM) - (DPN * ((((EFH + EFG) - B) + EFI) - (EDK * ((EFG + B) + EFL))));
                                        let EFP = EFG + (AY * (EFO / (EFN + (((EFN * EFN) - (AY * (EFO * (AY - (DPN * ((EFH + EFI) - (EDK * ((((UT * EFK) - (BZL * EFL)) * EFK) * EFK)))))))).sqrt()))));
                                        EFP
                                    };
                                    EIR = EIS;
                                }
                                EIQ = EIR;
                            } else {
                                let EFQ = BWA * BWA;
                                let EFR = EDN - (DRJ * BBN);
                                let EFS = DPI - (DPK * ((((rspice_limited_exp((-EFR))) + EFR) - B).sqrt()));
                                let EFT = EDI + UH;
                                let EFU = EFS - EFT;
                                let EFV = NZ * ((EFS + EFT) - (((EFU * EFU) + BUA).sqrt()));
                                let EFW = DPI - EFV;
                                let EFX = (DPL - EFV) + EDN;
                                let EFY = ((EFW * EFW) - ((EFQ * EFX) * EFX)) - (DPN * EDN);
                                let EFZ = AY * EFQ;
                                let EGA = (AY * EFW) - (EFZ * EFX);
                                let EGB = EGA * EGA;
                                let EGC = B - EFQ;
                                let EGD = if EFY < A { 1.0 } else { 0.0 };
                                let EGE = if EGD != 0.0 {
                                    A
                                } else {
                                    EFY
                                };
                                let EGF = EGE + EGA;
                                let EGG = EGE * EGC;
                                let EGH = (((EGF * EGF) / ((EDI - EFV) + ((if (EGE * DPO) >= BAF { (EGE * DPO) } else { BAF }).ln()))) + (NZ * EGB)) - EGG;
                                let EGI = EFV + ((EGF * EGE) / (EGH + (((EGA * EGF) / EGH) * ((BDI * EGB) - EGG))));
                                let EGJ = rspice_limited_exp((EGI - EDI));
                                let EGK = DPI - EGI;
                                let EGL = (DPL - EGI) + EDN;
                                let EGM = DPN * EGJ;
                                let EGN = ((AY * EGK) - (EFZ * EGL)) + EGM;
                                let EGO = AY * (((EGK * EGK) - ((EFQ * EGL) * EGL)) - (DPN * (EDN + EGJ)));
                                let EGP = EGI + (EGO / (EGN + (((EGN * EGN) - (EGO * ((AY - EFZ) - EGM))).sqrt())));
                                EIQ = EGP;
                            }
                            EIP = EIQ;
                        } else {
                            let EGQ = if DXH <= DRP { 1.0 } else { 0.0 };
                            let EIT;
                            if EGQ != 0.0 {
                                let EGR = (DPI * DRL) * (B + (((DPI * (B - EDK)) * DPK) * DXD));
                                EIT = EGR;
                            } else {
                                let EGS = if DPI < (-DRP) { 1.0 } else { 0.0 };
                                let EIU = if EGS != 0.0 {
                                    let EGT = -DPI;
                                    let EGU = BYV * (EGT * DRL);
                                    let EGV = EGU - UJ;
                                    let EGW = NZ * ((EGU + SS) - (((EGV * EGV) + BWT).sqrt()));
                                    let EGX = EGT - EGW;
                                    let EGY = (EGX * EGX) + (DPN * (EGW + B));
                                    let EGZ = (AY * EGX) - DPN;
                                    let EHA = (-EGW) + ((if (EGY * DPO) >= BAF { (EGY * DPO) } else { BAF }).ln());
                                    let EHB = EGY + EGZ;
                                    let EHC = EGZ * EGZ;
                                    let EHD = (EHB * EHB) + (EHA * ((NZ * EHC) - EGY));
                                    let EHE = EGW + (((EGY * EHB) * EHA) / (EHD + (((((EHB / EHD) * EHA) * EHA) * EGZ) * ((EHC * BDI) - EGY))));
                                    let EHF = rspice_limited_exp(EHE);
                                    let EHG = EHE * EHE;
                                    let EHH = B / (AY + EHG);
                                    let EHI = EHG * EHH;
                                    let EHJ = EGT - EHE;
                                    let EHK = EDK * (B / EHF);
                                    let EHL = (AY * EHJ) + (DPN * (((EHF - B) - EHK) + (EDK * (B - (UI * ((EHE * EHH) * EHH))))));
                                    let EHM = (EHJ * EHJ) - (DPN * ((((EHF - EHE) - B) + EHK) + (EDK * ((EHE - B) - EHI))));
                                    let EHN = (-EHE) - (AY * (EHM / (EHL + (((EHL * EHL) - (AY * (EHM * (AY - (DPN * ((EHF + EHK) - (EDK * ((((UT * EHH) - (BZL * EHI)) * EHH) * EHH)))))))).sqrt()))));
                                    EHN
                                } else {
                                    let EHO = B / (BYV + (DPK * BWJ));
                                    let EHP = (DPI + (DPN * NZ)) - (DPK * (((DPI + (DPN * OQ)) - (B - (rspice_limited_exp((-((DPI * DRL) * (B + (((((DRK * BYV) * EHO) - B) * EHO) * DPI)))))))).sqrt()));
                                    let EHQ = EDI + UH;
                                    let EHR = EHP - EHQ;
                                    let EHS = (NZ * ((EHP + EHQ) - (((EHR * EHR) + UA).sqrt()))) - (NZ * (EHQ - (((EHQ * EHQ) + UA).sqrt())));
                                    let EHT = DPI - EHS;
                                    let EHU = rspice_limited_exp((-EHS));
                                    let EHV = EHS * EHS;
                                    let EHW = B / (AY + EHV);
                                    let EHX = EHV * EHW;
                                    let EHY = if CAB >= ((EHT * EHT) - (DPN * (((EHU + EHS) - B) - (EDK * ((EHS + B) + EHX))))) { CAB } else { ((EHT * EHT) - (DPN * (((EHU + EHS) - B) - (EDK * ((EHS + B) + EHX))))) };
                                    let EHZ = (AY * EHT) + (DPN * ((B - EHU) - (EDK * (B + (UI * ((EHS * EHW) * EHW))))));
                                    let EIA = (EDI - EHS) + ((if (EHY / DPN) >= BAF { (EHY / DPN) } else { BAF }).ln());
                                    let EIB = EHY + EHZ;
                                    let EIC = EHZ * EHZ;
                                    let EID = EHY * (B - (NZ * (DPN * (EHU - (EDK * ((((UT * EHW) - (BZL * EHX)) * EHW) * EHW))))));
                                    let EIE = (EIB * EIB) + (EIA * ((NZ * EIC) - EID));
                                    let EIF = EHS + (((EHY * EIB) * EIA) / (EIE + (((((EIB / EIE) * EIA) * EIA) * EHZ) * ((EIC * BDI) - EID))));
                                    let EIG = B / (rspice_limited_exp(EIF));
                                    let EIH = rspice_limited_exp((EIF - EDI));
                                    let EII = EIF * EIF;
                                    let EIJ = B / (AY + EII);
                                    let EIK = EII * EIJ;
                                    let EIL = DPI - EIF;
                                    let EIM = (AY * EIL) + (DPN * (((B - EIG) + EIH) - (EDK * (B + (UI * ((EIF * EIJ) * EIJ))))));
                                    let EIN = (EIL * EIL) - (DPN * ((((EIG + EIF) - B) + EIH) - (EDK * ((EIF + B) + EIK))));
                                    let EIO = EIF + (AY * (EIN / (EIM + (((EIM * EIM) - (AY * (EIN * (AY - (DPN * ((EIG + EIH) - (EDK * ((((UT * EIJ) - (BZL * EIK)) * EIJ) * EIJ)))))))).sqrt()))));
                                    EIO
                                };
                                EIT = EIU;
                            }
                            EIP = EIT;
                        }
                        let EIV = (DXE + ((CEF * EDM) * BBM)) + DXF;
                        let EKG;
                        let ENQ;
                        if DXI != 0.0 {
                            let EIW = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD));
                            EKG = EIW;
                            ENQ = ENR;
                        } else {
                            let EIX = ((DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD))) * (NZ * (((-5e0f64 * (DPI - AY)).tanh()) + ((UA * (DPI + AY)).tanh())));
                            let EIY = ((EIP * BBM) - EIV) / BBM;
                            let EIZ = rspice_limited_exp(EIY);
                            let EJA = if EIY > CEP { 1.0 } else { 0.0 };
                            let EJJ;
                            if EJA != 0.0 {
                                EJJ = EIY;
                            } else {
                                let EJB = if EIY < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EJK = if EJB != 0.0 {
                                    let EJC = EIY.exp();
                                    EJC
                                } else {
                                    let EJD = (B + (EIY.exp())).ln();
                                    EJD
                                };
                                EJJ = EJK;
                            }
                            let EJE = ((EIX * BBM) - EIV) / BBM;
                            let EJF = if EJE > CEP { 1.0 } else { 0.0 };
                            let EJL;
                            if EJF != 0.0 {
                                EJL = EJE;
                            } else {
                                let EJG = if EJE < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EJM = if EJG != 0.0 {
                                    let EJH = EJE.exp();
                                    EJH
                                } else {
                                    let EJI = (B + (EJE.exp())).ln();
                                    EJI
                                };
                                EJL = EJM;
                            }
                            let EJN = -((DRJ / BBM) + ((EJJ - EJL) / CEF));
                            let EJO = rspice_limited_exp(EJN);
                            let EJP = rspice_limited_exp((-EIP));
                            let EJQ = EIP * EIP;
                            let EJR = B / (EJQ + AY);
                            let EJS = rspice_limited_exp((EIP - EDI));
                            let EJT = DPI - EIP;
                            let EJU = DPL + EJN;
                            let EJV = EJR * EJQ;
                            let EJW = ((EJT * EJT) - (((BWA * BWA) * EJU) * EJU)) - (DPN * (((((EJP - EJO) + EIP) + EJN) + EJS) - (EDK * ((EIP + B) + EJV))));
                            let EJX = B + EIZ;
                            let EJY = CEF * EJX;
                            let EJZ = AY * EIP;
                            let EKA = EIZ / EJY;
                            let EKB = EIZ * EJO;
                            let EKC = (((((((AY * EIZ) * EJU) * BWA) * BWA) / EJY) - (AY * DPI)) + EJZ) - (DPN * (((((EJS + (EDK * ((((-2e0f64 * EIP) * EJR) + ((((EJZ * EIP) * EIP) * EJR) * EJR)) - B))) - EJP) - EKA) + (EKB / EJY)) + B));
                            let EKD = ((AY * BWA) * BWA) * EIZ;
                            let EKE = EKD * EIZ;
                            let EKF = EIP - ((EJW / EKC) * (B + ((EJW * ((((((EKD * EJU) / EJY) - (EKE / ((EJY * CEF) * EJX))) - (DPN * (((EJP + EJS) - (((AY * EDK) * EJR) * (B - (EJV * (UA - ((UI * EJQ) * EJR)))))) - (EKA * (((B - (EIZ / EJX)) - EJO) + ((EKB / EJX) * (B + (B / CEF)))))))) - ((EKE * EJU) / (EJY * EJX))) + AY)) / ((AY * EKC) * EKC))));
                            EKG = EKF;
                            ENQ = EIX;
                        }
                        let ELR;
                        let ENP;
                        if DXI != 0.0 {
                            let EKH = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD));
                            ELR = EKH;
                            ENP = ENQ;
                        } else {
                            let EKI = ((DPI * DRL) * (B + (((DPI * (B - EDK)) * DPK) * DXD))) * (NZ * (((-5e0f64 * (DPI - AY)).tanh()) + ((UA * (DPI + AY)).tanh())));
                            let EKJ = ((EKG * BBM) - EIV) / BBM;
                            let EKK = rspice_limited_exp(EKJ);
                            let EKL = if EKJ > CEP { 1.0 } else { 0.0 };
                            let EKU;
                            if EKL != 0.0 {
                                EKU = EKJ;
                            } else {
                                let EKM = if EKJ < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EKV = if EKM != 0.0 {
                                    let EKN = EKJ.exp();
                                    EKN
                                } else {
                                    let EKO = (B + (EKJ.exp())).ln();
                                    EKO
                                };
                                EKU = EKV;
                            }
                            let EKP = ((EKI * BBM) - EIV) / BBM;
                            let EKQ = if EKP > CEP { 1.0 } else { 0.0 };
                            let EKW;
                            if EKQ != 0.0 {
                                EKW = EKP;
                            } else {
                                let EKR = if EKP < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EKX = if EKR != 0.0 {
                                    let EKS = EKP.exp();
                                    EKS
                                } else {
                                    let EKT = (B + (EKP.exp())).ln();
                                    EKT
                                };
                                EKW = EKX;
                            }
                            let EKY = -((DRJ / BBM) + ((EKU - EKW) / CEF));
                            let EKZ = rspice_limited_exp(EKY);
                            let ELA = rspice_limited_exp((-EKG));
                            let ELB = EKG * EKG;
                            let ELC = B / (ELB + AY);
                            let ELD = rspice_limited_exp((EKG - EDI));
                            let ELE = DPI - EKG;
                            let ELF = DPL + EKY;
                            let ELG = ELC * ELB;
                            let ELH = ((ELE * ELE) - (((BWA * BWA) * ELF) * ELF)) - (DPN * (((((ELA - EKZ) + EKG) + EKY) + ELD) - (EDK * ((EKG + B) + ELG))));
                            let ELI = B + EKK;
                            let ELJ = CEF * ELI;
                            let ELK = AY * EKG;
                            let ELL = EKK / ELJ;
                            let ELM = EKK * EKZ;
                            let ELN = (((((((AY * EKK) * ELF) * BWA) * BWA) / ELJ) - (AY * DPI)) + ELK) - (DPN * (((((ELD + (EDK * ((((-2e0f64 * EKG) * ELC) + ((((ELK * EKG) * EKG) * ELC) * ELC)) - B))) - ELA) - ELL) + (ELM / ELJ)) + B));
                            let ELO = ((AY * BWA) * BWA) * EKK;
                            let ELP = ELO * EKK;
                            let ELQ = EKG - ((ELH / ELN) * (B + ((ELH * ((((((ELO * ELF) / ELJ) - (ELP / ((ELJ * CEF) * ELI))) - (DPN * (((ELA + ELD) - (((AY * EDK) * ELC) * (B - (ELG * (UA - ((UI * ELB) * ELC)))))) - (ELL * (((B - (EKK / ELI)) - EKZ) + ((ELM / ELI) * (B + (B / CEF)))))))) - ((ELP * ELF) / (ELJ * ELI))) + AY)) / ((AY * ELN) * ELN))));
                            ELR = ELQ;
                            ENP = EKI;
                        }
                        let ENC;
                        let ENO;
                        if DXI != 0.0 {
                            let ELS = (DPI * DRL) * (B + (((DPI * (B - DRO)) * DPK) * DXD));
                            ENC = ELS;
                            ENO = ENP;
                        } else {
                            let ELT = ((DPI * DRL) * (B + (((DPI * (B - EDK)) * DPK) * DXD))) * (NZ * (((-5e0f64 * (DPI - AY)).tanh()) + ((UA * (DPI + AY)).tanh())));
                            let ELU = ((ELR * BBM) - EIV) / BBM;
                            let ELV = rspice_limited_exp(ELU);
                            let ELW = if ELU > CEP { 1.0 } else { 0.0 };
                            let EMF;
                            if ELW != 0.0 {
                                EMF = ELU;
                            } else {
                                let ELX = if ELU < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EMG = if ELX != 0.0 {
                                    let ELY = ELU.exp();
                                    ELY
                                } else {
                                    let ELZ = (B + (ELU.exp())).ln();
                                    ELZ
                                };
                                EMF = EMG;
                            }
                            let EMA = ((ELT * BBM) - EIV) / BBM;
                            let EMB = if EMA > CEP { 1.0 } else { 0.0 };
                            let EMH;
                            if EMB != 0.0 {
                                EMH = EMA;
                            } else {
                                let EMC = if EMA < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EMI = if EMC != 0.0 {
                                    let EMD = EMA.exp();
                                    EMD
                                } else {
                                    let EME = (B + (EMA.exp())).ln();
                                    EME
                                };
                                EMH = EMI;
                            }
                            let EMJ = -((DRJ / BBM) + ((EMF - EMH) / CEF));
                            let EMK = rspice_limited_exp(EMJ);
                            let EML = rspice_limited_exp((-ELR));
                            let EMM = ELR * ELR;
                            let EMN = B / (EMM + AY);
                            let EMO = rspice_limited_exp((ELR - EDI));
                            let EMP = DPI - ELR;
                            let EMQ = DPL + EMJ;
                            let EMR = EMN * EMM;
                            let EMS = ((EMP * EMP) - (((BWA * BWA) * EMQ) * EMQ)) - (DPN * (((((EML - EMK) + ELR) + EMJ) + EMO) - (EDK * ((ELR + B) + EMR))));
                            let EMT = B + ELV;
                            let EMU = CEF * EMT;
                            let EMV = AY * ELR;
                            let EMW = ELV / EMU;
                            let EMX = ELV * EMK;
                            let EMY = (((((((AY * ELV) * EMQ) * BWA) * BWA) / EMU) - (AY * DPI)) + EMV) - (DPN * (((((EMO + (EDK * ((((-2e0f64 * ELR) * EMN) + ((((EMV * ELR) * ELR) * EMN) * EMN)) - B))) - EML) - EMW) + (EMX / EMU)) + B));
                            let EMZ = ((AY * BWA) * BWA) * ELV;
                            let ENA = EMZ * ELV;
                            let ENB = ELR - ((EMS / EMY) * (B + ((EMS * ((((((EMZ * EMQ) / EMU) - (ENA / ((EMU * CEF) * EMT))) - (DPN * (((EML + EMO) - (((AY * EDK) * EMN) * (B - (EMR * (UA - ((UI * EMM) * EMN)))))) - (EMW * (((B - (ELV / EMT)) - EMK) + ((EMX / EMT) * (B + (B / CEF)))))))) - ((ENA * EMQ) / (EMU * EMT))) + AY)) / ((AY * EMY) * EMY))));
                            ENC = ENB;
                            ENO = ELT;
                        }
                        let END = ENC - EBQ;
                        let ENE = -EDL;
                        let ENF = rspice_limited_exp(ENE);
                        let ENG = if END < CVH { 1.0 } else { 0.0 };
                        let EPC;
                        let EPE;
                        if ENG != 0.0 {
                            let ENH = (ELR * BBM) - EIV;
                            let ENI = ENH / BBM;
                            let ENJ = rspice_limited_exp(ENI);
                            let ENK = if ENI > CEP { 1.0 } else { 0.0 };
                            let EOA;
                            if ENK != 0.0 {
                                EOA = ENI;
                            } else {
                                let ENL = if ENI < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EOB = if ENL != 0.0 {
                                    let ENM = ENI.exp();
                                    ENM
                                } else {
                                    let ENN = (B + (ENI.exp())).ln();
                                    ENN
                                };
                                EOA = EOB;
                            }
                            let ENV = ((ENO * BBM) - EIV) / BBM;
                            let ENW = if ENV > CEP { 1.0 } else { 0.0 };
                            let EOC;
                            if ENW != 0.0 {
                                EOC = ENV;
                            } else {
                                let ENX = if ENV < -3.7e1f64 { 1.0 } else { 0.0 };
                                let EOD = if ENX != 0.0 {
                                    let ENY = ENV.exp();
                                    ENY
                                } else {
                                    let ENZ = (B + (ENV.exp())).ln();
                                    ENZ
                                };
                                EOC = EOD;
                            }
                            let EOE = -((DRJ / BBM) + ((EOA - EOC) / CEF));
                            let EOF = rspice_limited_exp((-ELR));
                            let EOG = B / ((ELR * ELR) + AY);
                            let EOH = (AY * ENH) / BBM;
                            let EOI = rspice_limited_exp(EOH);
                            let EOJ = rspice_limited_exp((EOH + EOE));
                            let EOK = AY * ENJ;
                            let EOL = DPL + EOE;
                            let EOM = CEF * (ENJ + B);
                            let EON = AY * ELR;
                            let EOO = ENJ / EOM;
                            let EOP = (rspice_limited_exp((EOE + ENI))) / EOM;
                            let EOQ = -(((((((EOK * EOL) * BWA) * BWA) / EOM) - (AY * DPI)) + EON) - (DPN * ((((((rspice_limited_exp(((ELR - EDL) - EDI))) + ((rspice_limited_exp((ENE - EDI))) * ((((-2e0f64 * ELR) * EOG) + ((((EON * ELR) * ELR) * EOG) * EOG)) - B))) - EOF) - EOO) + EOP) + B)));
                            let EOR = (DPN * (B - ENF)) * EBV;
                            let EOS = (AY * BWA) * BWA;
                            let EOT = EOS * EOI;
                            let EOU = (B + EOK) + EOI;
                            let EOV = (CEF * CEF) * EOU;
                            let EOW = CEF * EOU;
                            let EOX = (EOQ * EOQ) - (AY * ((((((((EOS * ENJ) * EOL) / EOM) - (EOT / EOV)) - (DPN * (((((((EOF + (rspice_limited_exp(((ELR - EDI) - EDL)))) + ((rspice_limited_exp((EDJ - EDL))) * (((-2e0f64 * EOG) + ((((SS * ELR) * ELR) * EOG) * EOG)) - (((((((UT * ELR) * ELR) * ELR) * ELR) * EOG) * EOG) * EOG)))) - EOO) + (EOI / EOW)) + EOP) - (EOJ / EOW)) - (EOJ / EOV)))) - ((EOT * EOL) / EOW)) + AY) * EOR));
                            let EOY = if EOX >= A { 1.0 } else { 0.0 };
                            let EPA = if EOY != 0.0 {
                                let EOZ = AY * (EOR / (EOQ + (EOX.sqrt())));
                                EOZ
                            } else {
                                END
                            };
                            let EPB = EBQ + EPA;
                            EPC = EPA;
                            EPE = EPB;
                        } else {
                            EPC = END;
                            EPE = ENC;
                        }
                        let EPD = EPC * BBM;
                        let EPF = EPE * EPE;
                        let EPG = (rspice_limited_exp((EPE - EDI))) - (EDK * ((EPE + B) + (EPF / (AY + EPF))));
                        let EPH = DPI - EPE;
                        let EPI = (((EPH * EPH) * DPO) - EPG) - BDB;
                        let EPJ = (NZ * (EPI + (((EPI * EPI) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                        let EPK = ((DPN * EPG) * BBM) / ((DPK * ((EPJ + EPG).sqrt())) + (DPK * (EPJ.sqrt())));
                        let EPL = (NZ * (EBV + EPG)) + (CXP * ((EPC * EPC) * (((((rspice_limited_exp((-EPE))) * EBU).abs()).sqrt()) - (AY * DPO))));
                        let EPM = DPI - (NZ * (EBQ + EPE));
                        let EPN = ((EPM * EPM) * DPO) - EPL;
                        let EPO = DPK * ((EPL + EPN).sqrt());
                        let EPP = EPN - BDB;
                        let EPQ = ((NZ * (EPP + (((EPP * EPP) + 4.0000000000000007e-10f64).sqrt()))) + BDB).sqrt();
                        let EPR = if (EPD.abs()) > CYQ { 1.0 } else { 0.0 };
                        let EPV = if EPR != 0.0 {
                            let EPS = (ECA - EPK) / EPD;
                            EPS
                        } else {
                            DNF
                        };
                        let EPT = BBM * ((DPN * EPL) / (EPO + (DPK * EPQ)));
                        let EPU = EPO * BBM;
                        let EPW = EPV - BDB;
                        let EPX = (NZ * (EPW + (((EPW * EPW) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                        let EPY = (EPT + (BBM * EPX)) / EPX;
                        EPZ = EPU;
                        EQA = EPD;
                        EQB = EPY;
                        EQE = EPV;
                        EQG = EPT;
                        ERH = EPE;
                        ETA = EDH;
                        EVH = EPK;
                        EVK = DDP;
                        EVM = DDO;
                        EWF = ECA;
                        EXP = DDS;
                        EYB = DDM;
                        EYM = EDG;
                        FZM = ENO;
                        HXF = HXG;
                        HXT = HXU;
                    }
                    let EQL;
                    let EQN;
                    if EBR != 0.0 {
                        EQL = A;
                        EQN = A;
                    } else {
                        let EQC = NZ * (EQA / EQB);
                        let EQD = EPZ + (NZ * (EQA * (EQC * BDI)));
                        let EQF = (EQE * EQA) * BYR;
                        let EQH = NZ * (EQG - (EQF * ((B - EQC) - (DNM * (EQC * EQC)))));
                        let EQI = (EQD - (EQD - (EQG + (EQF * EQC)))) - EQH;
                        EQL = EQI;
                        EQN = EQH;
                    }
                    let EQK = -(EQJ * DNX);
                    let EQR;
                    let EQS;
                    if DOA != 0.0 {
                        let EQM = EQK * EQL;
                        let EQO = EQK * EQN;
                        EQR = EQM;
                        EQS = EQO;
                    } else {
                        let EQP = EQK * EQN;
                        let EQQ = EQK * EQL;
                        EQR = EQP;
                        EQS = EQQ;
                    }
                    EQT = EQR;
                    EQV = EQS;
                    ERF = EBQ;
                    ERG = ERH;
                    ESZ = ETA;
                    EVG = EVH;
                    EVJ = EVK;
                    EVL = EVM;
                    EWE = EWF;
                    EXO = EXP;
                    EYA = EYB;
                    EYL = EYM;
                    FDM = DPJ;
                    FDR = RR;
                    FZL = FZM;
                    HXE = HXF;
                    HXS = HXT;
                } else {
                    EQT = A;
                    EQV = A;
                    ERF = CIT;
                    ERG = ERI;
                    ESZ = ETB;
                    EVG = EVI;
                    EVJ = DDP;
                    EVL = DDO;
                    EWE = EWG;
                    EXO = DDS;
                    EYA = DDM;
                    EYL = EYN;
                    FDM = BCS;
                    FDR = LP;
                    FZL = ENU;
                    HXE = HXG;
                    HXS = HXU;
                }
                let EQU = BCB * (DOH + EQT);
                let EQW = BCB * (DOI + EQV);
                let EQX = PD + (IX * BCZ);
                let EQY = PE + (JB * BCZ);
                let EQZ = PF + (JF * BCZ);
                let ERA = IT + (IU * BCZ);
                let ERB = IQ + (IR * BCZ);
                let ERD = if ERC != A { 1.0 } else { 0.0 };
                let ERE = if RW != 0.0 || ERD != 0.0 { 1.0 } else { 0.0 };
                let EUL;
                let EUN;
                let EUP;
                let EUR;
                let EUU;
                if ERE != 0.0 {
                    let ERJ = BUT * (BVM - (NZ * (ERF + ERG)));
                    let ERK = ((ERJ * ERJ) + DGC).sqrt();
                    let ERL = NZ * ((-ERJ) + ERK);
                    let ERM = NZ * (ERJ + ERK);
                    let EUQ;
                    if ERD != 0.0 {
                        let ERO = -(ERJ / ERN);
                        let ERP = if ERO > CEP { 1.0 } else { 0.0 };
                        let ERT;
                        if ERP != 0.0 {
                            ERT = ERO;
                        } else {
                            let ERQ = if ERO < -3.7e1f64 { 1.0 } else { 0.0 };
                            let ERU = if ERQ != 0.0 {
                                let ERR = ERO.exp();
                                ERR
                            } else {
                                let ERS = (B + (ERO.exp())).ln();
                                ERS
                            };
                            ERT = ERU;
                        }
                        let ERV = ERN * ERT;
                        let ERX = if ERW != A { 1.0 } else { 0.0 };
                        let ERZ = if ERX != 0.0 {
                            let ERY = B - (ERL / ERW);
                            ERY
                        } else {
                            B
                        };
                        let ESA = if ERZ < BGT { 1.0 } else { 0.0 };
                        let ESE = if ESA != 0.0 {
                            BGT
                        } else {
                            ERZ
                        };
                        let ESB = ((AZ * BG) / BAO) + BAU;
                        let ESF = (((((ESB * ESC) * BAG) * BRC) * ERV) * (rspice_limited_exp((((ESD * M) * (ERB - (IS * ERL))) / ESE)))) * BFU;
                        let ESG = (ERJ - IP) / ERN;
                        let ESH = if ESG > CEP { 1.0 } else { 0.0 };
                        let ESL;
                        if ESH != 0.0 {
                            ESL = ESG;
                        } else {
                            let ESI = if ESG < -3.7e1f64 { 1.0 } else { 0.0 };
                            let ESM = if ESI != 0.0 {
                                let ESJ = ESG.exp();
                                ESJ
                            } else {
                                let ESK = (B + (ESG.exp())).ln();
                                ESK
                            };
                            ESL = ESM;
                        }
                        let ESN = ERN * ESL;
                        let ESP = if ESO != A { 1.0 } else { 0.0 };
                        let ESR = if ESP != 0.0 {
                            let ESQ = B - (ERM / ESO);
                            ESQ
                        } else {
                            B
                        };
                        let ESS = if ESR < BGT { 1.0 } else { 0.0 };
                        let ESV = if ESS != 0.0 {
                            BGT
                        } else {
                            ESR
                        };
                        let ESW = AA * (ESF + ((((((ESB * EST) * BAG) * BRC) * ESN) * (rspice_limited_exp((((ESU * M) * (ERA - (IV * ERM))) / ESV)))) * BFU));
                        EUQ = ESW;
                    } else {
                        EUQ = A;
                    }
                    let ESX = if (if (if (if BCN != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && DPF != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if EQJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if ESX != 0.0 {
                        if F != 0.0 {
                        } else {
                        }
                        if F != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    let EUM;
                    let EUO;
                    let EUS;
                    let EUV;
                    if RW != 0.0 {
                        let ESY = (((AA * BAV) * (((BVS * BUT) * DDV) * (rspice_limited_exp(((BAS * (EQX - (IY * ERM))) * (B + (IZ * ERM))))))) * ((BRC + (NZ * BSM)) - (NZ * (BSD + BSC)))) * BFU;
                        let ETC = PG * ((((ESZ * ESZ) + BGT).sqrt()) - BQB);
                        let ETD = rspice_limited_exp((-ETC));
                        let ETE = ((ETC + ETD) - B) + DGC;
                        let ETF = (B - ((ETC + B) * ETD)) + DGC;
                        let ETH = (ETC * ETC) + ETG;
                        let EUT;
                        let EUW;
                        if DOA != 0.0 {
                            let ETI = (ESY * ETF) / ETH;
                            let ETJ = (ESY * ETE) / ETH;
                            EUT = ETJ;
                            EUW = ETI;
                        } else {
                            let ETK = (ESY * ETF) / ETH;
                            let ETL = (ESY * ETE) / ETH;
                            EUT = ETK;
                            EUW = ETL;
                        }
                        let ETM = BRQ - DBT;
                        let ETN = ((ETM * ETM) + DGC).sqrt();
                        let ETP = if ETO == B { 1.0 } else { 0.0 };
                        let ETU;
                        let ETW;
                        if ETP != 0.0 {
                            let ETQ = EQY - (JC * ETN);
                            let ETR = NZ * (ETQ + (((ETQ * ETQ) + 4e-12f64).sqrt()));
                            let ETS = if JD < BGT { 1.0 } else { 0.0 };
                            let ETV = if ETS != 0.0 {
                                BGT
                            } else {
                                JD
                            };
                            ETU = ETV;
                            ETW = ETR;
                        } else {
                            let ETT = EQY - (JC * ETN);
                            ETU = JD;
                            ETW = ETT;
                        }
                        let ETX = BFU * AA;
                        let ETZ = ((((ETX * BAQ) * ETY) * BRQ) * ETN) * (rspice_limited_exp(((BAT * ETW) * (B + (ETU * ETN)))));
                        let EUA = BRP - DBT;
                        let EUB = ((EUA * EUA) + DGC).sqrt();
                        let EUG;
                        let EUI;
                        if ETP != 0.0 {
                            let EUC = EQZ - (JG * EUB);
                            let EUD = NZ * (EUC + (((EUC * EUC) + 4e-12f64).sqrt()));
                            let EUE = if JH < BGT { 1.0 } else { 0.0 };
                            let EUH = if EUE != 0.0 {
                                BGT
                            } else {
                                JH
                            };
                            EUG = EUH;
                            EUI = EUD;
                        } else {
                            let EUF = EQZ - (JG * EUB);
                            EUG = JH;
                            EUI = EUF;
                        }
                        let EUK = ((((ETX * BAR) * EUJ) * BRP) * EUB) * (rspice_limited_exp(((BAT * EUI) * (B + (EUG * EUB)))));
                        EUM = ETZ;
                        EUO = EUK;
                        EUS = EUT;
                        EUV = EUW;
                    } else {
                        EUM = A;
                        EUO = A;
                        EUS = A;
                        EUV = A;
                    }
                    EUL = EUM;
                    EUN = EUO;
                    EUP = EUQ;
                    EUR = EUS;
                    EUU = EUV;
                } else {
                    EUL = A;
                    EUN = A;
                    EUP = A;
                    EUR = A;
                    EUU = A;
                }
                let EUX = (UI * BBM) * C;
                let EUY = AY * CKH;
                let EUZ = EUY / DDT;
                let EVB = if EVA <= A { 1.0 } else { 0.0 };
                let EWI;
                if EVB != 0.0 {
                    EWI = A;
                } else {
                    let EVC = BCY * ((if (((DGM / BCY) + EVA) / EUZ) >= BAF { (((DGM / BCY) + EVA) / EUZ) } else { BAF }).ln());
                    let EVD = if EVC < A { 1.0 } else { 0.0 };
                    let EWJ = if EVD != 0.0 {
                        A
                    } else {
                        EVC
                    };
                    EWI = EWJ;
                }
                let EVE = (BBM / C) * ((N + BUK) + DT);
                let EVF = (BVT * N) * BBM;
                let EVN = (((EVF * EVG) * EVJ) * EVL) / C;
                let EVO = ((4.112737976006692e-57f64 * BBM) * (DDQ.abs())) * DDT;
                let EVP = ((C * BBM) * DDQ) * DDQ;
                let EVT = (EVQ + (EVR * EVN)) + ((EVS * EVN) * EVN);
                let EVU = EVN + EVE;
                let EVV = EVU * EVU;
                let EVW = (EVQ * C) * BBM;
                let EVY = if EVX >= (AZ / AY) { 1.0 } else { 0.0 };
                let EWA = if EVY != 0.0 {
                    A
                } else {
                    EVX
                };
                let EVZ = if (if (if EVQ > A { 1.0 } else { 0.0 }) != 0.0 || (if EVR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if EVS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EWR;
                if EVZ != 0.0 {
                    let EWB = AZ - (AY * EWA);
                    let EWC = EWB * EWB;
                    let EWH = (((EVF * EWE) * EVJ) * EVL) / C;
                    let EWK = ((EVO / ((EWD * N) * EWC)) * (((EVQ * ((if ((EWH + EVE) / EVU) >= BAF { ((EWH + EVE) / EVU) } else { BAF }).ln())) + (EVR * (EWH - EVN))) + ((NZ * EVS) * ((EWH * EWH) - (EVN * EVN))))) + ((((EVP / (((EWD * EWC) * BG) * AA)) * EWI) * EVT) / EVV);
                    let EWL = ((EVW / (((((BG * AA) * EWB) * EWD) * EVE) * EVE)) * DDQ) * DDQ;
                    let EWM = EWL + EWK;
                    let EWN = if EWM > A { 1.0 } else { 0.0 };
                    let EWS = if EWN != 0.0 {
                        let EWQ = ((EWK * EWL) / EWM) / (B + (EWO * ((EWE - EVG).powf(EWP))));
                        EWQ
                    } else {
                        A
                    };
                    EWR = EWS;
                } else {
                    EWR = A;
                }
                let EWT = DGQ * EWR;
                let EWV = (DDV / EUZ) / AZ;
                let EWW = EWV * EWV;
                let EWZ = EWX * (B + ((EWY * AZ) * EWW));
                let EXC = EXA * (B + ((EXB * AZ) * EWW));
                let EXF = EXD * (B + ((EXE * AZ) * EWW));
                let EXI = EXG * (B + ((EXH * AZ) * EWW));
                let EXK = rspice_limited_exp(((-AZ) / EXJ));
                let EXL = ((((UH * EWZ) * EWZ) - B) * EXK) + B;
                let EXM = EXF * EXF;
                let EXN = EXC * EXC;
                let EXR = if EXQ == A { 1.0 } else { 0.0 };
                let HYY;
                let HZA;
                let HZC;
                let HZF;
                let HZI;
                let HZL;
                let HZO;
                let HZR;
                if EXR != 0.0 {
                    let EXS = ((((-AA) * BG) * AZ) * N) * BBM;
                    let EXT = DDT * (((EXS * EQU) + (EXS * EQW)).abs());
                    let EXY = EUX * ((EXT / ((EXT * EXU) + (AZ * AZ))) * EXX);
                    HYY = B;
                    HZA = EXY;
                    HZC = A;
                    HZF = A;
                    HZI = A;
                    HZL = A;
                    HZO = A;
                    HZR = A;
                } else {
                    let EXZ = if EXQ == B { 1.0 } else { 0.0 };
                    let HZD;
                    let HZG;
                    let HZJ;
                    let HZM;
                    let HZP;
                    let HZS;
                    if EXZ != 0.0 {
                        let EYC = (((DDT * EXO) * EYA) * N) * (BVT * BUT);
                        let EYD = NZ * (EWE + EVG);
                        let EYE = EYD + NZ;
                        let EYF = EYE * EYE;
                        let EYG = EYF * EYE;
                        let EYH = EWE - EVG;
                        let EYI = EYH * EYH;
                        let EYJ = AZ * EXO;
                        let EYK = EYJ / AZ;
                        let EYP = (((B + ((EXM * (ESZ / EYL)) / (EYO + DDV))) - B) * EXK) + B;
                        let EYQ = BZL * EYE;
                        let EYU = ((((((EYJ * EYK) * EYK) * (((EYD / EYF) - ((((UJ * EYD) + NZ) * EYI) / ((EYR * EYF) * EYF))) + ((EYI * EYI) / ((EYS * EYF) * EYG)))) * EYT) / UI) * EXN) / (((AA * BG) * BZL) * EYC);
                        let EYW = ((EYK * ((EYH / EYQ) - ((EYI * EYH) / (EYS * EYG)))) * EXI) / EYV;
                        let EYX = (EUX * ((((EYC * AA) * BG) / EYJ) * ((EYD * (NZ * (EYP + (((EYP * EYP) + 2.5000000000000005e-3f64).sqrt())))) + ((EYI * EXL) / EYQ)))).sqrt();
                        let EYY = if EYU > A { 1.0 } else { 0.0 };
                        let EZC;
                        let EZE;
                        if EYY != 0.0 {
                            let EYZ = (EUX / EYU).sqrt();
                            let EZA = if EYX > A { 1.0 } else { 0.0 };
                            let EZD = if EZA != 0.0 {
                                let EZB = (EYW * EYZ) / EYX;
                                EZB
                            } else {
                                A
                            };
                            EZC = EZD;
                            EZE = EYZ;
                        } else {
                            EZC = A;
                            EZE = A;
                        }
                        let EZF = B - EZC;
                        let EZG = (EZE * EZE) * EZF;
                        let EZH = (EYX * EYX) * EZF;
                        HZD = B;
                        HZG = EZC;
                        HZJ = B;
                        HZM = EZG;
                        HZP = B;
                        HZS = EZH;
                    } else {
                        HZD = A;
                        HZG = A;
                        HZJ = A;
                        HZM = A;
                        HZP = A;
                        HZS = A;
                    }
                    HYY = A;
                    HZA = A;
                    HZC = HZD;
                    HZF = HZG;
                    HZI = HZJ;
                    HZL = HZM;
                    HZO = HZP;
                    HZR = HZS;
                }
                let HZU;
                let HZW;
                let HZY;
                let IAA;
                if RW != 0.0 {
                    let EZI = 3.204352924e-19f64 * ((EUR + EUL).abs());
                    let EZJ = 3.204352924e-19f64 * ((EUU + EUN).abs());
                    HZU = B;
                    HZW = EZI;
                    HZY = B;
                    IAA = EZJ;
                } else {
                    HZU = A;
                    HZW = A;
                    HZY = A;
                    IAA = A;
                }
                let IAC;
                let IAE;
                if ERD != 0.0 {
                    let EZK = 3.204352924e-19f64 * (EUP.abs());
                    IAC = B;
                    IAE = EZK;
                } else {
                    IAC = A;
                    IAE = A;
                }
                let EZL = if BPG == B { 1.0 } else { 0.0 };
                let HXC;
                let HXQ;
                if EZL != 0.0 {
                    let EZM = if ((BCR + (BBM * ((if (JY / BBT) >= BAF { (JY / BBT) } else { BAF }).ln()))) + FB) >= BCR { ((BCR + (BBM * ((if (JY / BBT) >= BAF { (JY / BBT) } else { BAF }).ln()))) + FB) } else { BCR };
                    let EZN = B + (KI * BCZ);
                    let EZP = EZM - BSO;
                    let EZQ = EZP - BQE;
                    let EZR = (NZ * ((EZP + BQE) + (((EZQ * EZQ) + 2.5000000000000005e-3f64).sqrt()))).sqrt();
                    let EZS = ((BCV / (C * JY)).sqrt()) * EZR;
                    let EZU = N + ((BUE * BUF) / (BUE + BUF));
                    let EZV = (((EZU + JZ) + (JX * (NZ * (EZN + (((EZN * EZN) + 4e-6f64).sqrt()))))) + (((((parameters[1181] * BSR) + ((parameters[1182] * BSR) * BSR)) - (parameters[1184] * BSO)) - ((parameters[1185] * BSO) * BSO)) + (BUN * (((((EF + (EJ * BSR)) + ((parameters[1180] * BSR) * BSR)) + (EG * BSO)) + ((parameters[1190] * BSO) * BSO)) + (((EZT + (parameters[1183] * BSR)) - (parameters[1195] * BSO)) * BSM))))) / EZU;
                    let EZW = EZV - B;
                    let EZX = NZ * ((EZV + B) + (((EZW * EZW) + 6.250000000000001e-4f64).sqrt()));
                    let EZY = EZX * BBM;
                    let EZZ = B / EZY;
                    let FAA = BRC * EZZ;
                    let FAB = BSD * EZZ;
                    let FAC = BVI * EZZ;
                    let FAD = DN * EZZ;
                    let FAE = BRS * EZZ;
                    let FAF = EZR - (EZM.sqrt());
                    let FAG = (QD * FAF) - (BQZ * BSO);
                    let FAH = (-((EZO * (B + (KJ * BCZ))) + (KD * BSO))) * BSM;
                    let FAI = ((KE + (KF / AZ)) + (KG * BSO)) * ((BBP.powf(KH)) - B);
                    let FAK = BCY * (B + (FAJ * BSO));
                    let FAL = if FAK > A { 1.0 } else { 0.0 };
                    let FAS;
                    if FAL != 0.0 {
                        let FAN = (FAM * AZ) / FAK;
                        let FAO = if FAN < BUA { 1.0 } else { 0.0 };
                        let FAT = if FAO != 0.0 {
                            let FAQ = (NZ * FAP) / ((FAN.cosh()) - B);
                            FAQ
                        } else {
                            let FAR = FAP * (rspice_limited_exp((-FAN)));
                            FAR
                        };
                        FAS = FAT;
                    } else {
                        FAS = A;
                    }
                    let FAV = FAS * (FAU - EZM);
                    let FAW = if EM > A { 1.0 } else { 0.0 };
                    let FBC;
                    if FAW != 0.0 {
                        let FAX = (-EO) * BSM;
                        let FAY = if FAX < -8e1f64 { 1.0 } else { 0.0 };
                        let FBA = if FAY != 0.0 {
                            BVC
                        } else {
                            let FAZ = rspice_limited_exp(FAX);
                            FAZ
                        };
                        let FBB = (-EZY) * ((if (AZ / (AZ + (EM * (B + FBA)))) >= BAF { (AZ / (AZ + (EM * (B + FBA)))) } else { BAF }).ln());
                        FBC = FBB;
                    } else {
                        FBC = A;
                    }
                    let FBG = (FAA - FAC) - ((((((((FAG + (FBC - ((EW + (ET / (AZ.powf(EU)))) * ((EV * BSM).tanh())))) + FAH) - FAI) + FAV) + FBD) + FBE) + BQT) * EZZ);
                    let FBH = FAE - FAD;
                    let FBL = (((((3.204352924e-19f64 * J) * JY) * EZZ).sqrt()) / N) * (B + (FBI * (B + (FBJ * (AZ.powf((-FBK)))))));
                    let FBM = B / FBL;
                    let FBN = FBL * FBL;
                    let FBO = B / FBN;
                    let FBP = FBL / BWA;
                    let FBQ = B + (FBP * BWC);
                    let FBR = BWE * FBQ;
                    let FBT = B / FBP;
                    let FBU = FBP * FBP;
                    let FBV = B / (FBS + (FBP * BWJ));
                    let FBW = FBH.abs();
                    let FBX = if FBW <= FBR { 1.0 } else { 0.0 };
                    let FDD;
                    if FBX != 0.0 {
                        let FBY = -FBH;
                        let FBZ = (FBY * FBT) * (B + (FBP * (FBY / ((8.485281374238571e0f64 * FBQ) * FBQ))));
                        FDD = FBZ;
                    } else {
                        let FCA = if FBH < (-FBR) { 1.0 } else { 0.0 };
                        let FDE = if FCA != 0.0 {
                            let FCB = -FBH;
                            let FCC = (FBS * FCB) * FBT;
                            let FCD = FCC - UJ;
                            let FCE = NZ * ((FCC + SS) - (((FCD * FCD) + BWT).sqrt()));
                            let FCF = FCB - FCE;
                            let FCG = (FCF * FCF) + (FBU * (FCE + B));
                            let FCH = (AY * FCF) - FBU;
                            let FCI = ((if (FCG / FBU) >= BAF { (FCG / FBU) } else { BAF }).ln()) - FCE;
                            let FCJ = FCG + FCH;
                            let FCK = FCH * FCH;
                            let FCL = (FCJ * FCJ) + (FCI * ((NZ * FCK) - FCG));
                            let FCM = FCE + (((FCG * FCJ) * FCI) / (FCL + (((((FCJ / FCL) * FCI) * FCI) * FCH) * ((FCK * BDI) - FCG))));
                            let FCN = rspice_limited_exp(FCM);
                            let FCO = FCB - FCM;
                            let FCP = (AY * FCO) + (FBU * (FCN - B));
                            let FCQ = (FCO * FCO) + (FBU * ((FCM + B) - FCN));
                            let FCR = -(FCM + (AY * (FCQ / (FCP + (((FCP * FCP) - (UI * ((B - ((FBU * NZ) * FCN)) * FCQ))).sqrt())))));
                            FCR
                        } else {
                            let FCS = FBU * NZ;
                            let FCT = (FBH + FCS) - (FBP * (((FBH + (FBU * OQ)) - (B - (rspice_limited_exp((-((FBH * FBT) * (B + (((((FBQ * FBS) * FBV) - B) * FBV) * FBH)))))))).sqrt()));
                            let FCU = rspice_limited_exp((-FCT));
                            let FCV = FBH - FCT;
                            let FCW = (AY * FCV) + (FBU * (B - FCU));
                            let FCX = (FCV * FCV) - (FBU * ((FCT - B) + FCU));
                            let FCY = FCT + (AY * (FCX / (FCW + (((FCW * FCW) - (UI * ((B - (FCS * FCU)) * FCX))).sqrt()))));
                            FCY
                        };
                        FDD = FDE;
                    }
                    let FCZ = if FBW < FBR { 1.0 } else { 0.0 };
                    let FDI = if FCZ != 0.0 {
                        let FDA = -FBH;
                        let FDB = (FDA * FBT) * (B + (FBP * (FDA / ((8.485281374238571e0f64 * FBQ) * FBQ))));
                        FDB
                    } else {
                        let FDC = BWA * BWA;
                        let FDF = FBH - FDD;
                        let FDG = rspice_limited_exp((-FDD));
                        let FDH = FDD - ((((((FDC * FDF) * FDF) * FBM) * FBM) - ((FDG + FDD) - B)) / ((FDG + ((FDC * ((AY * FDD) - (AY * FBH))) / FBN)) - B));
                        FDH
                    };
                    let FDJ = FDI * EZY;
                    let FDK = B + (FBL * BWC);
                    let FDL = B / FDK;
                    let FDN = (AY * FDM) / EZX;
                    let FDO = FDN + FAB;
                    let FDP = rspice_limited_exp((-FDO));
                    let FDQ = BDB * FDK;
                    let FDS = C * FDR;
                    let FDT = ((BYI * ((FDS * BTV) * BTV)) / (BCV * EZY)) + (BYJ / EZY);
                    let FDU = BVZ * FBH;
                    let FDV = FDT - FDU;
                    let FDW = FDV + (FBL * ((((rspice_limited_exp((-FDV))) + FDV) - B).sqrt()));
                    let FDX = if FDV < FDO { 1.0 } else { 0.0 };
                    let FIZ;
                    if FDX != 0.0 {
                        let FDY = if FBG < FDW { 1.0 } else { 0.0 };
                        let FJA;
                        if FDY != 0.0 {
                            let FDZ = if (FBG.abs()) <= FDQ { 1.0 } else { 0.0 };
                            let FJB;
                            if FDZ != 0.0 {
                                let FEA = (FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * (((FDL * FDL) * BYR) * BWC)));
                                FJB = FEA;
                            } else {
                                let FEB = if FBG < (-FDQ) { 1.0 } else { 0.0 };
                                let FJC = if FEB != 0.0 {
                                    let FEC = -FBG;
                                    let FED = BYV * (FEC * FDL);
                                    let FEE = FED - UJ;
                                    let FEF = NZ * ((FED + SS) - (((FEE * FEE) + BWT).sqrt()));
                                    let FEG = FEC - FEF;
                                    let FEH = (FEG * FEG) + (FBN * (FEF + B));
                                    let FEI = (AY * FEG) - FBN;
                                    let FEJ = (-FEF) + ((if (FEH * FBO) >= BAF { (FEH * FBO) } else { BAF }).ln());
                                    let FEK = FEH + FEI;
                                    let FEL = FEI * FEI;
                                    let FEM = (FEK * FEK) + (FEJ * ((NZ * FEL) - FEH));
                                    let FEN = FEF + (((FEH * FEK) * FEJ) / (FEM + (((((FEK / FEM) * FEJ) * FEJ) * FEI) * ((FEL * BDI) - FEH))));
                                    let FEO = rspice_limited_exp(FEN);
                                    let FEP = FEN * FEN;
                                    let FEQ = B / (AY + FEP);
                                    let FER = FEP * FEQ;
                                    let FES = FEC - FEN;
                                    let FET = FDP * (B / FEO);
                                    let FEU = (AY * FES) + (FBN * (((FEO - B) - FET) + (FDP * (B - (UI * ((FEN * FEQ) * FEQ))))));
                                    let FEV = (FES * FES) - (FBN * ((((FEO - FEN) - B) + FET) + (FDP * ((FEN - B) - FER))));
                                    let FEW = (-FEN) - (AY * (FEV / (FEU + (((FEU * FEU) - (AY * (FEV * (AY - (FBN * ((FEO + FET) - (FDP * ((((UT * FEQ) - (BZL * FER)) * FEQ) * FEQ)))))))).sqrt()))));
                                    FEW
                                } else {
                                    let FEX = B / (BYV + (FBL * BWJ));
                                    let FEY = (FBG + (FBN * NZ)) - (FBL * (((FBG + (FBN * OQ)) - (B - (rspice_limited_exp((-((FBG * FDL) * (B + (((((FDK * BYV) * FEX) - B) * FEX) * FBG)))))))).sqrt()));
                                    let FEZ = FDO + UH;
                                    let FFA = FEY - FEZ;
                                    let FFB = (NZ * ((FEY + FEZ) - (((FFA * FFA) + UA).sqrt()))) - (NZ * (FEZ - (((FEZ * FEZ) + UA).sqrt())));
                                    let FFC = FBG - FFB;
                                    let FFD = rspice_limited_exp((-FFB));
                                    let FFE = FFB * FFB;
                                    let FFF = B / (AY + FFE);
                                    let FFG = FFE * FFF;
                                    let FFH = if CAB >= ((FFC * FFC) - (FBN * (((FFD + FFB) - B) - (FDP * ((FFB + B) + FFG))))) { CAB } else { ((FFC * FFC) - (FBN * (((FFD + FFB) - B) - (FDP * ((FFB + B) + FFG))))) };
                                    let FFI = (AY * FFC) + (FBN * ((B - FFD) - (FDP * (B + (UI * ((FFB * FFF) * FFF))))));
                                    let FFJ = (FDO - FFB) + ((if (FFH / FBN) >= BAF { (FFH / FBN) } else { BAF }).ln());
                                    let FFK = FFH + FFI;
                                    let FFL = FFI * FFI;
                                    let FFM = FFH * (B - (NZ * (FBN * (FFD - (FDP * ((((UT * FFF) - (BZL * FFG)) * FFF) * FFF))))));
                                    let FFN = (FFK * FFK) + (FFJ * ((NZ * FFL) - FFM));
                                    let FFO = FFB + (((FFH * FFK) * FFJ) / (FFN + (((((FFK / FFN) * FFJ) * FFJ) * FFI) * ((FFL * BDI) - FFM))));
                                    let FFP = B / (rspice_limited_exp(FFO));
                                    let FFQ = rspice_limited_exp((FFO - FDO));
                                    let FFR = FFO * FFO;
                                    let FFS = B / (AY + FFR);
                                    let FFT = FFR * FFS;
                                    let FFU = FBG - FFO;
                                    let FFV = (AY * FFU) + (FBN * (((B - FFP) + FFQ) - (FDP * (B + (UI * ((FFO * FFS) * FFS))))));
                                    let FFW = (FFU * FFU) - (FBN * ((((FFP + FFO) - B) + FFQ) - (FDP * ((FFO + B) + FFT))));
                                    let FFX = FFO + (AY * (FFW / (FFV + (((FFV * FFV) - (AY * (FFW * (AY - (FBN * ((FFP + FFQ) - (FDP * ((((UT * FFS) - (BZL * FFT)) * FFS) * FFS)))))))).sqrt()))));
                                    FFX
                                };
                                FJB = FJC;
                            }
                            FJA = FJB;
                        } else {
                            let FFY = BWA * BWA;
                            let FFZ = FDV - (FDJ * EZZ);
                            let FGA = FBG - (FBL * ((((rspice_limited_exp((-FFZ))) + FFZ) - B).sqrt()));
                            let FGB = FDO + UH;
                            let FGC = FGA - FGB;
                            let FGD = NZ * ((FGA + FGB) - (((FGC * FGC) + BUA).sqrt()));
                            let FGE = FBG - FGD;
                            let FGF = (FBH - FGD) + FDV;
                            let FGG = ((FGE * FGE) - ((FFY * FGF) * FGF)) - (FBN * FDV);
                            let FGH = AY * FFY;
                            let FGI = (AY * FGE) - (FGH * FGF);
                            let FGJ = FGI * FGI;
                            let FGK = B - FFY;
                            let FGL = if FGG < A { 1.0 } else { 0.0 };
                            let FGM = if FGL != 0.0 {
                                A
                            } else {
                                FGG
                            };
                            let FGN = FGM + FGI;
                            let FGO = FGM * FGK;
                            let FGP = (((FGN * FGN) / ((FDO - FGD) + ((if (FGM * FBO) >= BAF { (FGM * FBO) } else { BAF }).ln()))) + (NZ * FGJ)) - FGO;
                            let FGQ = FGD + ((FGN * FGM) / (FGP + (((FGI * FGN) / FGP) * ((BDI * FGJ) - FGO))));
                            let FGR = rspice_limited_exp((FGQ - FDO));
                            let FGS = FBG - FGQ;
                            let FGT = (FBH - FGQ) + FDV;
                            let FGU = FBN * FGR;
                            let FGV = ((AY * FGS) - (FGH * FGT)) + FGU;
                            let FGW = AY * (((FGS * FGS) - ((FFY * FGT) * FGT)) - (FBN * (FDV + FGR)));
                            let FGX = FGQ + (FGW / (FGV + (((FGV * FGV) - (FGW * ((AY - FGH) - FGU))).sqrt())));
                            FJA = FGX;
                        }
                        FIZ = FJA;
                    } else {
                        let FGY = if (FBG.abs()) <= FDQ { 1.0 } else { 0.0 };
                        let FJD;
                        if FGY != 0.0 {
                            let FGZ = (FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * (((FDL * FDL) * BYR) * BWC)));
                            FJD = FGZ;
                        } else {
                            let FHA = if FBG < (-FDQ) { 1.0 } else { 0.0 };
                            let FJE = if FHA != 0.0 {
                                let FHB = -FBG;
                                let FHC = BYV * (FHB * FDL);
                                let FHD = FHC - UJ;
                                let FHE = NZ * ((FHC + SS) - (((FHD * FHD) + BWT).sqrt()));
                                let FHF = FHB - FHE;
                                let FHG = (FHF * FHF) + (FBN * (FHE + B));
                                let FHH = (AY * FHF) - FBN;
                                let FHI = (-FHE) + ((if (FHG * FBO) >= BAF { (FHG * FBO) } else { BAF }).ln());
                                let FHJ = FHG + FHH;
                                let FHK = FHH * FHH;
                                let FHL = (FHJ * FHJ) + (FHI * ((NZ * FHK) - FHG));
                                let FHM = FHE + (((FHG * FHJ) * FHI) / (FHL + (((((FHJ / FHL) * FHI) * FHI) * FHH) * ((FHK * BDI) - FHG))));
                                let FHN = rspice_limited_exp(FHM);
                                let FHO = FHM * FHM;
                                let FHP = B / (AY + FHO);
                                let FHQ = FHO * FHP;
                                let FHR = FHB - FHM;
                                let FHS = FDP * (B / FHN);
                                let FHT = (AY * FHR) + (FBN * (((FHN - B) - FHS) + (FDP * (B - (UI * ((FHM * FHP) * FHP))))));
                                let FHU = (FHR * FHR) - (FBN * ((((FHN - FHM) - B) + FHS) + (FDP * ((FHM - B) - FHQ))));
                                let FHV = (FHT * FHT) - (AY * (FHU * (AY - (FBN * ((FHN + FHS) - (FDP * ((((UT * FHP) - (BZL * FHQ)) * FHP) * FHP)))))));
                                let FHW = (-FHM) - (AY * (FHU / (FHT + (((((FHV * FHV) + CCR).sqrt()) - CCS).sqrt()))));
                                FHW
                            } else {
                                let FHX = B / (BYV + (FBL * BWJ));
                                let FHY = (FBG + (FBN * NZ)) - (FBL * (((FBG + (FBN * OQ)) - (B - (rspice_limited_exp((-((FBG * FDL) * (B + (((((FDK * BYV) * FHX) - B) * FHX) * FBG)))))))).sqrt()));
                                let FHZ = FDO + UH;
                                let FIA = FHY - FHZ;
                                let FIB = (NZ * ((FHY + FHZ) - (((FIA * FIA) + UA).sqrt()))) - (NZ * (FHZ - (((FHZ * FHZ) + UA).sqrt())));
                                let FIC = FBG - FIB;
                                let FID = rspice_limited_exp((-FIB));
                                let FIE = FIB * FIB;
                                let FIF = B / (AY + FIE);
                                let FIG = FIE * FIF;
                                let FIH = if CAB >= ((FIC * FIC) - (FBN * (((FID + FIB) - B) - (FDP * ((FIB + B) + FIG))))) { CAB } else { ((FIC * FIC) - (FBN * (((FID + FIB) - B) - (FDP * ((FIB + B) + FIG))))) };
                                let FII = (AY * FIC) + (FBN * ((B - FID) - (FDP * (B + (UI * ((FIB * FIF) * FIF))))));
                                let FIJ = (FDO - FIB) + ((if (FIH / FBN) >= BAF { (FIH / FBN) } else { BAF }).ln());
                                let FIK = FIH + FII;
                                let FIL = FII * FII;
                                let FIM = FIH * (B - (NZ * (FBN * (FID - (FDP * ((((UT * FIF) - (BZL * FIG)) * FIF) * FIF))))));
                                let FIN = (FIK * FIK) + (FIJ * ((NZ * FIL) - FIM));
                                let FIO = FIB + (((FIH * FIK) * FIJ) / (FIN + (((((FIK / FIN) * FIJ) * FIJ) * FII) * ((FIL * BDI) - FIM))));
                                let FIP = B / (rspice_limited_exp(FIO));
                                let FIQ = rspice_limited_exp((FIO - FDO));
                                let FIR = FIO * FIO;
                                let FIS = B / (AY + FIR);
                                let FIT = FIR * FIS;
                                let FIU = FBG - FIO;
                                let FIV = (AY * FIU) + (FBN * (((B - FIP) + FIQ) - (FDP * (B + (UI * ((FIO * FIS) * FIS))))));
                                let FIW = (FIU * FIU) - (FBN * ((((FIP + FIO) - B) + FIQ) - (FDP * ((FIO + B) + FIT))));
                                let FIX = (FIV * FIV) - (AY * (FIW * (AY - (FBN * ((FIP + FIQ) - (FDP * ((((UT * FIS) - (BZL * FIT)) * FIS) * FIS)))))));
                                let FIY = FIO + (AY * (FIW / (FIV + (((((FIX * FIX) + CCR).sqrt()) - CCS).sqrt()))));
                                FIY
                            };
                            FJD = FJE;
                        }
                        FIZ = FJD;
                    }
                    let FJF = ((FDL * FDL) * BYR) * BWC;
                    let FJG = (((BYI * (FDS * CED)) / BCV) + BYJ) - ((BVZ * (FBH * EZY)) * EB);
                    let FJH = CEF * FDJ;
                    let FJI = FJG + FJH;
                    let FJJ = FBG.abs();
                    let FJK = if FJJ <= BWE { 1.0 } else { 0.0 };
                    let FKV;
                    let FZK;
                    if FJK != 0.0 {
                        let FJL = (FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * FJF));
                        FKV = FJL;
                        FZK = FZL;
                    } else {
                        let FJM = ((FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * FJF))) * (NZ * (((-5e0f64 * (FBG - AY)).tanh()) + ((UA * (FBG + AY)).tanh())));
                        let FJN = ((FIZ * EZY) - FJI) / EZY;
                        let FJO = rspice_limited_exp(FJN);
                        let FJP = ((FJM * EZY) - FJI) / EZY;
                        let FJQ = if FJN > CEP { 1.0 } else { 0.0 };
                        let FJY;
                        if FJQ != 0.0 {
                            FJY = FJN;
                        } else {
                            let FJR = if FJN < -3.7e1f64 { 1.0 } else { 0.0 };
                            let FJZ = if FJR != 0.0 {
                                let FJS = FJN.exp();
                                FJS
                            } else {
                                let FJT = (B + (FJN.exp())).ln();
                                FJT
                            };
                            FJY = FJZ;
                        }
                        let FJU = if FJP > CEP { 1.0 } else { 0.0 };
                        let FKA;
                        if FJU != 0.0 {
                            FKA = FJP;
                        } else {
                            let FJV = if FJP < -3.7e1f64 { 1.0 } else { 0.0 };
                            let FKB = if FJV != 0.0 {
                                let FJW = FJP.exp();
                                FJW
                            } else {
                                let FJX = (B + (FJP.exp())).ln();
                                FJX
                            };
                            FKA = FKB;
                        }
                        let FKC = -((FDJ / EZY) + ((FJY - FKA) / CEF));
                        let FKD = rspice_limited_exp(FKC);
                        let FKE = rspice_limited_exp((-FIZ));
                        let FKF = FIZ * FIZ;
                        let FKG = B / (FKF + AY);
                        let FKH = rspice_limited_exp((FIZ - FDO));
                        let FKI = FBG - FIZ;
                        let FKJ = FBH + FKC;
                        let FKK = FKG * FKF;
                        let FKL = ((FKI * FKI) - (((BWA * BWA) * FKJ) * FKJ)) - (FBN * (((((FKE - FKD) + FIZ) + FKC) + FKH) - (FDP * ((FIZ + B) + FKK))));
                        let FKM = B + FJO;
                        let FKN = CEF * FKM;
                        let FKO = AY * FIZ;
                        let FKP = FJO / FKN;
                        let FKQ = FJO * FKD;
                        let FKR = (((((((AY * FJO) * FKJ) * BWA) * BWA) / FKN) - (AY * FBG)) + FKO) - (FBN * (((((FKH + (FDP * ((((-2e0f64 * FIZ) * FKG) + ((((FKO * FIZ) * FIZ) * FKG) * FKG)) - B))) - FKE) - FKP) + (FKQ / FKN)) + B));
                        let FKS = ((AY * BWA) * BWA) * FJO;
                        let FKT = FKS * FJO;
                        let FKU = FIZ - ((FKL / FKR) * (B + ((FKL * ((((((FKS * FKJ) / FKN) - (FKT / ((FKN * CEF) * FKM))) - (FBN * (((FKE + FKH) - (((AY * FDP) * FKG) * (B - (FKK * (UA - ((UI * FKF) * FKG)))))) - (FKP * (((B - (FJO / FKM)) - FKD) + ((FKQ / FKM) * (B + (B / CEF)))))))) - ((FKT * FKJ) / (FKN * FKM))) + AY)) / ((AY * FKR) * FKR))));
                        FKV = FKU;
                        FZK = FJM;
                    }
                    let FMG;
                    let FZJ;
                    if FJK != 0.0 {
                        let FKW = (FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * FJF));
                        FMG = FKW;
                        FZJ = FZK;
                    } else {
                        let FKX = ((FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * FJF))) * (NZ * (((-5e0f64 * (FBG - AY)).tanh()) + ((UA * (FBG + AY)).tanh())));
                        let FKY = ((FKV * EZY) - FJI) / EZY;
                        let FKZ = rspice_limited_exp(FKY);
                        let FLA = ((FKX * EZY) - FJI) / EZY;
                        let FLB = if FKY > CEP { 1.0 } else { 0.0 };
                        let FLJ;
                        if FLB != 0.0 {
                            FLJ = FKY;
                        } else {
                            let FLC = if FKY < -3.7e1f64 { 1.0 } else { 0.0 };
                            let FLK = if FLC != 0.0 {
                                let FLD = FKY.exp();
                                FLD
                            } else {
                                let FLE = (B + (FKY.exp())).ln();
                                FLE
                            };
                            FLJ = FLK;
                        }
                        let FLF = if FLA > CEP { 1.0 } else { 0.0 };
                        let FLL;
                        if FLF != 0.0 {
                            FLL = FLA;
                        } else {
                            let FLG = if FLA < -3.7e1f64 { 1.0 } else { 0.0 };
                            let FLM = if FLG != 0.0 {
                                let FLH = FLA.exp();
                                FLH
                            } else {
                                let FLI = (B + (FLA.exp())).ln();
                                FLI
                            };
                            FLL = FLM;
                        }
                        let FLN = -((FDJ / EZY) + ((FLJ - FLL) / CEF));
                        let FLO = rspice_limited_exp(FLN);
                        let FLP = rspice_limited_exp((-FKV));
                        let FLQ = FKV * FKV;
                        let FLR = B / (FLQ + AY);
                        let FLS = rspice_limited_exp((FKV - FDO));
                        let FLT = FBG - FKV;
                        let FLU = FBH + FLN;
                        let FLV = FLR * FLQ;
                        let FLW = ((FLT * FLT) - (((BWA * BWA) * FLU) * FLU)) - (FBN * (((((FLP - FLO) + FKV) + FLN) + FLS) - (FDP * ((FKV + B) + FLV))));
                        let FLX = B + FKZ;
                        let FLY = CEF * FLX;
                        let FLZ = AY * FKV;
                        let FMA = FKZ / FLY;
                        let FMB = FKZ * FLO;
                        let FMC = (((((((AY * FKZ) * FLU) * BWA) * BWA) / FLY) - (AY * FBG)) + FLZ) - (FBN * (((((FLS + (FDP * ((((-2e0f64 * FKV) * FLR) + ((((FLZ * FKV) * FKV) * FLR) * FLR)) - B))) - FLP) - FMA) + (FMB / FLY)) + B));
                        let FMD = ((AY * BWA) * BWA) * FKZ;
                        let FME = FMD * FKZ;
                        let FMF = FKV - ((FLW / FMC) * (B + ((FLW * ((((((FMD * FLU) / FLY) - (FME / ((FLY * CEF) * FLX))) - (FBN * (((FLP + FLS) - (((AY * FDP) * FLR) * (B - (FLV * (UA - ((UI * FLQ) * FLR)))))) - (FMA * (((B - (FKZ / FLX)) - FLO) + ((FMB / FLX) * (B + (B / CEF)))))))) - ((FME * FLU) / (FLY * FLX))) + AY)) / ((AY * FMC) * FMC))));
                        FMG = FMF;
                        FZJ = FKX;
                    }
                    let FNR;
                    let FZI;
                    if FJK != 0.0 {
                        let FMH = (FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * FJF));
                        FNR = FMH;
                        FZI = FZJ;
                    } else {
                        let FMI = ((FBG * FDL) * (B + (((FBG * (B - FDP)) * FBL) * FJF))) * (NZ * (((-5e0f64 * (FBG - AY)).tanh()) + ((UA * (FBG + AY)).tanh())));
                        let FMJ = ((FMG * EZY) - FJI) / EZY;
                        let FMK = rspice_limited_exp(FMJ);
                        let FML = ((FMI * EZY) - FJI) / EZY;
                        let FMM = if FMJ > CEP { 1.0 } else { 0.0 };
                        let FMU;
                        if FMM != 0.0 {
                            FMU = FMJ;
                        } else {
                            let FMN = if FMJ < -3.7e1f64 { 1.0 } else { 0.0 };
                            let FMV = if FMN != 0.0 {
                                let FMO = FMJ.exp();
                                FMO
                            } else {
                                let FMP = (B + (FMJ.exp())).ln();
                                FMP
                            };
                            FMU = FMV;
                        }
                        let FMQ = if FML > CEP { 1.0 } else { 0.0 };
                        let FMW;
                        if FMQ != 0.0 {
                            FMW = FML;
                        } else {
                            let FMR = if FML < -3.7e1f64 { 1.0 } else { 0.0 };
                            let FMX = if FMR != 0.0 {
                                let FMS = FML.exp();
                                FMS
                            } else {
                                let FMT = (B + (FML.exp())).ln();
                                FMT
                            };
                            FMW = FMX;
                        }
                        let FMY = -((FDJ / EZY) + ((FMU - FMW) / CEF));
                        let FMZ = rspice_limited_exp(FMY);
                        let FNA = rspice_limited_exp((-FMG));
                        let FNB = FMG * FMG;
                        let FNC = B / (FNB + AY);
                        let FND = rspice_limited_exp((FMG - FDO));
                        let FNE = FBG - FMG;
                        let FNF = FBH + FMY;
                        let FNG = FNC * FNB;
                        let FNH = ((FNE * FNE) - (((BWA * BWA) * FNF) * FNF)) - (FBN * (((((FNA - FMZ) + FMG) + FMY) + FND) - (FDP * ((FMG + B) + FNG))));
                        let FNI = B + FMK;
                        let FNJ = CEF * FNI;
                        let FNK = AY * FMG;
                        let FNL = FMK / FNJ;
                        let FNM = FMK * FMZ;
                        let FNN = (((((((AY * FMK) * FNF) * BWA) * BWA) / FNJ) - (AY * FBG)) + FNK) - (FBN * (((((FND + (FDP * ((((-2e0f64 * FMG) * FNC) + ((((FNK * FMG) * FMG) * FNC) * FNC)) - B))) - FNA) - FNL) + (FNM / FNJ)) + B));
                        let FNO = ((AY * BWA) * BWA) * FMK;
                        let FNP = FNO * FMK;
                        let FNQ = FMG - ((FNH / FNN) * (B + ((FNH * ((((((FNO * FNF) / FNJ) - (FNP / ((FNJ * CEF) * FNI))) - (FBN * (((FNA + FND) - (((AY * FDP) * FNC) * (B - (FNG * (UA - ((UI * FNB) * FNC)))))) - (FNL * (((B - (FMK / FNI)) - FMZ) + ((FNM / FNI) * (B + (B / CEF)))))))) - ((FNP * FNF) / (FNJ * FNI))) + AY)) / ((AY * FNN) * FNN))));
                        FNR = FNQ;
                        FZI = FMI;
                    }
                    let FNS = if FNR <= A { 1.0 } else { 0.0 };
                    let HXD;
                    let HXR;
                    if FNS != 0.0 {
                        HXD = CIW;
                        HXR = CIZ;
                    } else {
                        let FNT = FNR * FNR;
                        let FNU = B / (rspice_limited_exp(FNR));
                        let FNV = (rspice_limited_exp((FNR - FDO))) - (FDP * ((FNR + B) + (FNT * (B / (AY + FNT)))));
                        let FNW = FBG - FNR;
                        let FNX = (((FNW * FNW) * FBO) - FNV) - BDB;
                        let FNY = (NZ * (FNX + (((FNX * FNX) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                        let FNZ = FBL * (FNY.sqrt());
                        let FOA = ((FBN * FNV) * EZY) / ((FBL * ((FNY + FNV).sqrt())) + FNZ);
                        let FOB = FNZ * EZY;
                        let FOC = B + (((CJP + (CJQ * BSO)) * (((CJM / BTW) * (FOB + (BDM * FOA))).powf(BEF))) + (CJS / (rspice_limited_exp((CJO * ((if (NZ * (B + (FOA / FOB))) >= BAF { (NZ * (B + (FOA / FOB))) } else { BAF }).ln()))))));
                        let FOD = FOC - B;
                        let FOE = NZ * ((FOC + B) + (((FOD * FOD) + 5.625e-7f64).sqrt()));
                        let FOF = B / (((BG * CJW).powf(FR)) * AA);
                        let FON;
                        if QS != 0.0 {
                            FON = A;
                        } else {
                            let FOG = (B / (B + (FP * FOA))) + (QL * FAF);
                            let FOH = ((CJZ + (CKB * (FOG + (((FOG * FOG) + BGT).sqrt())))) * FOF) * AA;
                            let FOI = FOH * BEW;
                            let FOJ = if QR == AY { 1.0 } else { 0.0 };
                            let FOO = if FOJ != 0.0 {
                                let FOK = ((CIZ + FOH) + CIW) * BEW;
                                FOK
                            } else {
                                FOI
                            };
                            FON = FOO;
                        }
                        let FOL = ((EUY / CKJ) * FOE) * AZ;
                        let FOM = GG * (FOA + (AY * EZY));
                        let FOP = if FON > A { 1.0 } else { 0.0 };
                        let FPA = if FOP != 0.0 {
                            let FOQ = ((BG * CKH) * N) * FON;
                            let FOR = AY * FOQ;
                            let FOS = (FOM + FOL) + ((UH * FOM) * FOQ);
                            let FOT = (FOS - (((FOS * FOS) - ((AY * FOR) * (FOM * (FOL + ((AY * FOM) * FOQ))))).sqrt())) / FOR;
                            FOT
                        } else {
                            let FOU = (FOL * FOM) / (FOL + FOM);
                            FOU
                        };
                        let FOV = if (if CKV == A { 1.0 } else { 0.0 }) != 0.0 && (if CKW == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let FPC = if FOV != 0.0 {
                            B
                        } else {
                            let FOW = AZ / (AZ + ((FA * EZS).sqrt()));
                            let FOX = B + (((CKV * FOW) - (((CKW * FOW) * (FOA.powf(CKZ))) * EZY)) / (B + (CLA * BSO)));
                            let FOY = FOX - BQB;
                            let FOZ = NZ * ((FOX + BQB) + (((FOY * FOY) + 6.25e-8f64).sqrt()));
                            FOZ
                        };
                        let FPB = FPA - BDB;
                        let FPD = FDN + (((BSE * ((B + (((BSE / (((NZ * (FPB + (((FPB * FPB) + 4.0000000000000007e-10f64).sqrt()))) + BDB) / FPC)) + BX).powf((B / BFH)))).powf((-BFH)))) + BSD) * EZZ);
                        let FPE = -FPD;
                        let FPF = rspice_limited_exp(FPE);
                        let FPG = BSE * EZZ;
                        let FPH = BYH * (FPG + (DZ * EZZ));
                        let FPI = (FDT - (FDU * EB)) + (CEF * FPH);
                        let FPJ = FPI + (FBL * ((((rspice_limited_exp((-FPI))) + FPI) - B).sqrt()));
                        let FPK = if FPI < FPD { 1.0 } else { 0.0 };
                        let FUK;
                        if FPK != 0.0 {
                            let FPL = if FBG < FPJ { 1.0 } else { 0.0 };
                            let FUL;
                            if FPL != 0.0 {
                                let FPM = if FJJ <= FDQ { 1.0 } else { 0.0 };
                                let FUM;
                                if FPM != 0.0 {
                                    let FPN = (FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF));
                                    FUM = FPN;
                                } else {
                                    let FPO = if FBG < (-FDQ) { 1.0 } else { 0.0 };
                                    let FUN = if FPO != 0.0 {
                                        let FPP = -FBG;
                                        let FPQ = BYV * (FPP * FDL);
                                        let FPR = FPQ - UJ;
                                        let FPS = NZ * ((FPQ + SS) - (((FPR * FPR) + BWT).sqrt()));
                                        let FPT = FPP - FPS;
                                        let FPU = (FPT * FPT) + (FBN * (FPS + B));
                                        let FPV = (AY * FPT) - FBN;
                                        let FPW = (-FPS) + ((if (FPU * FBO) >= BAF { (FPU * FBO) } else { BAF }).ln());
                                        let FPX = FPU + FPV;
                                        let FPY = FPV * FPV;
                                        let FPZ = (FPX * FPX) + (FPW * ((NZ * FPY) - FPU));
                                        let FQA = FPS + (((FPU * FPX) * FPW) / (FPZ + (((((FPX / FPZ) * FPW) * FPW) * FPV) * ((FPY * BDI) - FPU))));
                                        let FQB = rspice_limited_exp(FQA);
                                        let FQC = FQA * FQA;
                                        let FQD = B / (AY + FQC);
                                        let FQE = FQC * FQD;
                                        let FQF = FPP - FQA;
                                        let FQG = FPF * (B / FQB);
                                        let FQH = (AY * FQF) + (FBN * (((FQB - B) - FQG) + (FPF * (B - (UI * ((FQA * FQD) * FQD))))));
                                        let FQI = (FQF * FQF) - (FBN * ((((FQB - FQA) - B) + FQG) + (FPF * ((FQA - B) - FQE))));
                                        let FQJ = (-FQA) - (AY * (FQI / (FQH + (((FQH * FQH) - (AY * (FQI * (AY - (FBN * ((FQB + FQG) - (FPF * ((((UT * FQD) - (BZL * FQE)) * FQD) * FQD)))))))).sqrt()))));
                                        FQJ
                                    } else {
                                        let FQK = B / (BYV + (FBL * BWJ));
                                        let FQL = (FBG + (FBN * NZ)) - (FBL * (((FBG + (FBN * OQ)) - (B - (rspice_limited_exp((-((FBG * FDL) * (B + (((((FDK * BYV) * FQK) - B) * FQK) * FBG)))))))).sqrt()));
                                        let FQM = FPD + UH;
                                        let FQN = FQL - FQM;
                                        let FQO = (NZ * ((FQL + FQM) - (((FQN * FQN) + UA).sqrt()))) - (NZ * (FQM - (((FQM * FQM) + UA).sqrt())));
                                        let FQP = FBG - FQO;
                                        let FQQ = rspice_limited_exp((-FQO));
                                        let FQR = FQO * FQO;
                                        let FQS = B / (AY + FQR);
                                        let FQT = FQR * FQS;
                                        let FQU = if CAB >= ((FQP * FQP) - (FBN * (((FQQ + FQO) - B) - (FPF * ((FQO + B) + FQT))))) { CAB } else { ((FQP * FQP) - (FBN * (((FQQ + FQO) - B) - (FPF * ((FQO + B) + FQT))))) };
                                        let FQV = (AY * FQP) + (FBN * ((B - FQQ) - (FPF * (B + (UI * ((FQO * FQS) * FQS))))));
                                        let FQW = (FPD - FQO) + ((if (FQU / FBN) >= BAF { (FQU / FBN) } else { BAF }).ln());
                                        let FQX = FQU + FQV;
                                        let FQY = FQV * FQV;
                                        let FQZ = FQU * (B - (NZ * (FBN * (FQQ - (FPF * ((((UT * FQS) - (BZL * FQT)) * FQS) * FQS))))));
                                        let FRA = (FQX * FQX) + (FQW * ((NZ * FQY) - FQZ));
                                        let FRB = FQO + (((FQU * FQX) * FQW) / (FRA + (((((FQX / FRA) * FQW) * FQW) * FQV) * ((FQY * BDI) - FQZ))));
                                        let FRC = B / (rspice_limited_exp(FRB));
                                        let FRD = rspice_limited_exp((FRB - FPD));
                                        let FRE = FRB * FRB;
                                        let FRF = B / (AY + FRE);
                                        let FRG = FRE * FRF;
                                        let FRH = FBG - FRB;
                                        let FRI = (AY * FRH) + (FBN * (((B - FRC) + FRD) - (FPF * (B + (UI * ((FRB * FRF) * FRF))))));
                                        let FRJ = (FRH * FRH) - (FBN * ((((FRC + FRB) - B) + FRD) - (FPF * ((FRB + B) + FRG))));
                                        let FRK = FRB + (AY * (FRJ / (FRI + (((FRI * FRI) - (AY * (FRJ * (AY - (FBN * ((FRC + FRD) - (FPF * ((((UT * FRF) - (BZL * FRG)) * FRF) * FRF)))))))).sqrt()))));
                                        FRK
                                    };
                                    FUM = FUN;
                                }
                                FUL = FUM;
                            } else {
                                let FRL = BWA * BWA;
                                let FRM = FPI - (FDJ * EZZ);
                                let FRN = FBG - (FBL * ((((rspice_limited_exp((-FRM))) + FRM) - B).sqrt()));
                                let FRO = FPD + UH;
                                let FRP = FRN - FRO;
                                let FRQ = NZ * ((FRN + FRO) - (((FRP * FRP) + BUA).sqrt()));
                                let FRR = FBG - FRQ;
                                let FRS = (FBH - FRQ) + FPI;
                                let FRT = ((FRR * FRR) - ((FRL * FRS) * FRS)) - (FBN * FPI);
                                let FRU = AY * FRL;
                                let FRV = (AY * FRR) - (FRU * FRS);
                                let FRW = FRV * FRV;
                                let FRX = B - FRL;
                                let FRY = if FRT < A { 1.0 } else { 0.0 };
                                let FRZ = if FRY != 0.0 {
                                    A
                                } else {
                                    FRT
                                };
                                let FSA = FRZ + FRV;
                                let FSB = FRZ * FRX;
                                let FSC = (((FSA * FSA) / ((FPD - FRQ) + ((if (FRZ * FBO) >= BAF { (FRZ * FBO) } else { BAF }).ln()))) + (NZ * FRW)) - FSB;
                                let FSD = FRQ + ((FSA * FRZ) / (FSC + (((FRV * FSA) / FSC) * ((BDI * FRW) - FSB))));
                                let FSE = rspice_limited_exp((FSD - FPD));
                                let FSF = FBG - FSD;
                                let FSG = (FBH - FSD) + FPI;
                                let FSH = FBN * FSE;
                                let FSI = ((AY * FSF) - (FRU * FSG)) + FSH;
                                let FSJ = AY * (((FSF * FSF) - ((FRL * FSG) * FSG)) - (FBN * (FPI + FSE)));
                                let FSK = FSD + (FSJ / (FSI + (((FSI * FSI) - (FSJ * ((AY - FRU) - FSH))).sqrt())));
                                FUL = FSK;
                            }
                            FUK = FUL;
                        } else {
                            let FSL = if FJJ <= FDQ { 1.0 } else { 0.0 };
                            let FUO;
                            if FSL != 0.0 {
                                let FSM = (FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF));
                                FUO = FSM;
                            } else {
                                let FSN = if FBG < (-FDQ) { 1.0 } else { 0.0 };
                                let FUP = if FSN != 0.0 {
                                    let FSO = -FBG;
                                    let FSP = BYV * (FSO * FDL);
                                    let FSQ = FSP - UJ;
                                    let FSR = NZ * ((FSP + SS) - (((FSQ * FSQ) + BWT).sqrt()));
                                    let FSS = FSO - FSR;
                                    let FST = (FSS * FSS) + (FBN * (FSR + B));
                                    let FSU = (AY * FSS) - FBN;
                                    let FSV = (-FSR) + ((if (FST * FBO) >= BAF { (FST * FBO) } else { BAF }).ln());
                                    let FSW = FST + FSU;
                                    let FSX = FSU * FSU;
                                    let FSY = (FSW * FSW) + (FSV * ((NZ * FSX) - FST));
                                    let FSZ = FSR + (((FST * FSW) * FSV) / (FSY + (((((FSW / FSY) * FSV) * FSV) * FSU) * ((FSX * BDI) - FST))));
                                    let FTA = rspice_limited_exp(FSZ);
                                    let FTB = FSZ * FSZ;
                                    let FTC = B / (AY + FTB);
                                    let FTD = FTB * FTC;
                                    let FTE = FSO - FSZ;
                                    let FTF = FPF * (B / FTA);
                                    let FTG = (AY * FTE) + (FBN * (((FTA - B) - FTF) + (FPF * (B - (UI * ((FSZ * FTC) * FTC))))));
                                    let FTH = (FTE * FTE) - (FBN * ((((FTA - FSZ) - B) + FTF) + (FPF * ((FSZ - B) - FTD))));
                                    let FTI = (-FSZ) - (AY * (FTH / (FTG + (((FTG * FTG) - (AY * (FTH * (AY - (FBN * ((FTA + FTF) - (FPF * ((((UT * FTC) - (BZL * FTD)) * FTC) * FTC)))))))).sqrt()))));
                                    FTI
                                } else {
                                    let FTJ = B / (BYV + (FBL * BWJ));
                                    let FTK = (FBG + (FBN * NZ)) - (FBL * (((FBG + (FBN * OQ)) - (B - (rspice_limited_exp((-((FBG * FDL) * (B + (((((FDK * BYV) * FTJ) - B) * FTJ) * FBG)))))))).sqrt()));
                                    let FTL = FPD + UH;
                                    let FTM = FTK - FTL;
                                    let FTN = (NZ * ((FTK + FTL) - (((FTM * FTM) + UA).sqrt()))) - (NZ * (FTL - (((FTL * FTL) + UA).sqrt())));
                                    let FTO = FBG - FTN;
                                    let FTP = rspice_limited_exp((-FTN));
                                    let FTQ = FTN * FTN;
                                    let FTR = B / (AY + FTQ);
                                    let FTS = FTQ * FTR;
                                    let FTT = if CAB >= ((FTO * FTO) - (FBN * (((FTP + FTN) - B) - (FPF * ((FTN + B) + FTS))))) { CAB } else { ((FTO * FTO) - (FBN * (((FTP + FTN) - B) - (FPF * ((FTN + B) + FTS))))) };
                                    let FTU = (AY * FTO) + (FBN * ((B - FTP) - (FPF * (B + (UI * ((FTN * FTR) * FTR))))));
                                    let FTV = (FPD - FTN) + ((if (FTT / FBN) >= BAF { (FTT / FBN) } else { BAF }).ln());
                                    let FTW = FTT + FTU;
                                    let FTX = FTU * FTU;
                                    let FTY = FTT * (B - (NZ * (FBN * (FTP - (FPF * ((((UT * FTR) - (BZL * FTS)) * FTR) * FTR))))));
                                    let FTZ = (FTW * FTW) + (FTV * ((NZ * FTX) - FTY));
                                    let FUA = FTN + (((FTT * FTW) * FTV) / (FTZ + (((((FTW / FTZ) * FTV) * FTV) * FTU) * ((FTX * BDI) - FTY))));
                                    let FUB = B / (rspice_limited_exp(FUA));
                                    let FUC = rspice_limited_exp((FUA - FPD));
                                    let FUD = FUA * FUA;
                                    let FUE = B / (AY + FUD);
                                    let FUF = FUD * FUE;
                                    let FUG = FBG - FUA;
                                    let FUH = (AY * FUG) + (FBN * (((B - FUB) + FUC) - (FPF * (B + (UI * ((FUA * FUE) * FUE))))));
                                    let FUI = (FUG * FUG) - (FBN * ((((FUB + FUA) - B) + FUC) - (FPF * ((FUA + B) + FUF))));
                                    let FUJ = FUA + (AY * (FUI / (FUH + (((FUH * FUH) - (AY * (FUI * (AY - (FBN * ((FUB + FUC) - (FPF * ((((UT * FUE) - (BZL * FUF)) * FUE) * FUE)))))))).sqrt()))));
                                    FUJ
                                };
                                FUO = FUP;
                            }
                            FUK = FUO;
                        }
                        let FUQ = (FJG + ((CEF * FPH) * EZY)) + FJH;
                        let FWB;
                        let FZH;
                        if FJK != 0.0 {
                            let FUR = (FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF));
                            FWB = FUR;
                            FZH = FZI;
                        } else {
                            let FUS = ((FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF))) * (NZ * (((-5e0f64 * (FBG - AY)).tanh()) + ((UA * (FBG + AY)).tanh())));
                            let FUT = ((FUK * EZY) - FUQ) / EZY;
                            let FUU = rspice_limited_exp(FUT);
                            let FUV = ((FUS * EZY) - FUQ) / EZY;
                            let FUW = if FUT > CEP { 1.0 } else { 0.0 };
                            let FVE;
                            if FUW != 0.0 {
                                FVE = FUT;
                            } else {
                                let FUX = if FUT < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FVF = if FUX != 0.0 {
                                    let FUY = FUT.exp();
                                    FUY
                                } else {
                                    let FUZ = (B + (FUT.exp())).ln();
                                    FUZ
                                };
                                FVE = FVF;
                            }
                            let FVA = if FUV > CEP { 1.0 } else { 0.0 };
                            let FVG;
                            if FVA != 0.0 {
                                FVG = FUV;
                            } else {
                                let FVB = if FUV < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FVH = if FVB != 0.0 {
                                    let FVC = FUV.exp();
                                    FVC
                                } else {
                                    let FVD = (B + (FUV.exp())).ln();
                                    FVD
                                };
                                FVG = FVH;
                            }
                            let FVI = -((FDJ / EZY) + ((FVE - FVG) / CEF));
                            let FVJ = rspice_limited_exp(FVI);
                            let FVK = rspice_limited_exp((-FUK));
                            let FVL = FUK * FUK;
                            let FVM = B / (FVL + AY);
                            let FVN = rspice_limited_exp((FUK - FPD));
                            let FVO = FBG - FUK;
                            let FVP = FBH + FVI;
                            let FVQ = FVM * FVL;
                            let FVR = ((FVO * FVO) - (((BWA * BWA) * FVP) * FVP)) - (FBN * (((((FVK - FVJ) + FUK) + FVI) + FVN) - (FPF * ((FUK + B) + FVQ))));
                            let FVS = B + FUU;
                            let FVT = CEF * FVS;
                            let FVU = AY * FUK;
                            let FVV = FUU / FVT;
                            let FVW = FUU * FVJ;
                            let FVX = (((((((AY * FUU) * FVP) * BWA) * BWA) / FVT) - (AY * FBG)) + FVU) - (FBN * (((((FVN + (FPF * ((((-2e0f64 * FUK) * FVM) + ((((FVU * FUK) * FUK) * FVM) * FVM)) - B))) - FVK) - FVV) + (FVW / FVT)) + B));
                            let FVY = ((AY * BWA) * BWA) * FUU;
                            let FVZ = FVY * FUU;
                            let FWA = FUK - ((FVR / FVX) * (B + ((FVR * ((((((FVY * FVP) / FVT) - (FVZ / ((FVT * CEF) * FVS))) - (FBN * (((FVK + FVN) - (((AY * FPF) * FVM) * (B - (FVQ * (UA - ((UI * FVL) * FVM)))))) - (FVV * (((B - (FUU / FVS)) - FVJ) + ((FVW / FVS) * (B + (B / CEF)))))))) - ((FVZ * FVP) / (FVT * FVS))) + AY)) / ((AY * FVX) * FVX))));
                            FWB = FWA;
                            FZH = FUS;
                        }
                        let FXM;
                        let FZG;
                        if FJK != 0.0 {
                            let FWC = (FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF));
                            FXM = FWC;
                            FZG = FZH;
                        } else {
                            let FWD = ((FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF))) * (NZ * (((-5e0f64 * (FBG - AY)).tanh()) + ((UA * (FBG + AY)).tanh())));
                            let FWE = ((FWB * EZY) - FUQ) / EZY;
                            let FWF = rspice_limited_exp(FWE);
                            let FWG = ((FWD * EZY) - FUQ) / EZY;
                            let FWH = if FWE > CEP { 1.0 } else { 0.0 };
                            let FWP;
                            if FWH != 0.0 {
                                FWP = FWE;
                            } else {
                                let FWI = if FWE < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FWQ = if FWI != 0.0 {
                                    let FWJ = FWE.exp();
                                    FWJ
                                } else {
                                    let FWK = (B + (FWE.exp())).ln();
                                    FWK
                                };
                                FWP = FWQ;
                            }
                            let FWL = if FWG > CEP { 1.0 } else { 0.0 };
                            let FWR;
                            if FWL != 0.0 {
                                FWR = FWG;
                            } else {
                                let FWM = if FWG < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FWS = if FWM != 0.0 {
                                    let FWN = FWG.exp();
                                    FWN
                                } else {
                                    let FWO = (B + (FWG.exp())).ln();
                                    FWO
                                };
                                FWR = FWS;
                            }
                            let FWT = -((FDJ / EZY) + ((FWP - FWR) / CEF));
                            let FWU = rspice_limited_exp(FWT);
                            let FWV = rspice_limited_exp((-FWB));
                            let FWW = FWB * FWB;
                            let FWX = B / (FWW + AY);
                            let FWY = rspice_limited_exp((FWB - FPD));
                            let FWZ = FBG - FWB;
                            let FXA = FBH + FWT;
                            let FXB = FWX * FWW;
                            let FXC = ((FWZ * FWZ) - (((BWA * BWA) * FXA) * FXA)) - (FBN * (((((FWV - FWU) + FWB) + FWT) + FWY) - (FPF * ((FWB + B) + FXB))));
                            let FXD = B + FWF;
                            let FXE = CEF * FXD;
                            let FXF = AY * FWB;
                            let FXG = FWF / FXE;
                            let FXH = FWF * FWU;
                            let FXI = (((((((AY * FWF) * FXA) * BWA) * BWA) / FXE) - (AY * FBG)) + FXF) - (FBN * (((((FWY + (FPF * ((((-2e0f64 * FWB) * FWX) + ((((FXF * FWB) * FWB) * FWX) * FWX)) - B))) - FWV) - FXG) + (FXH / FXE)) + B));
                            let FXJ = ((AY * BWA) * BWA) * FWF;
                            let FXK = FXJ * FWF;
                            let FXL = FWB - ((FXC / FXI) * (B + ((FXC * ((((((FXJ * FXA) / FXE) - (FXK / ((FXE * CEF) * FXD))) - (FBN * (((FWV + FWY) - (((AY * FPF) * FWX) * (B - (FXB * (UA - ((UI * FWW) * FWX)))))) - (FXG * (((B - (FWF / FXD)) - FWU) + ((FXH / FXD) * (B + (B / CEF)))))))) - ((FXK * FXA) / (FXE * FXD))) + AY)) / ((AY * FXI) * FXI))));
                            FXM = FXL;
                            FZG = FWD;
                        }
                        let FYX;
                        let FZF;
                        if FJK != 0.0 {
                            let FXN = (FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF));
                            FYX = FXN;
                            FZF = FZG;
                        } else {
                            let FXO = ((FBG * FDL) * (B + (((FBG * (B - FPF)) * FBL) * FJF))) * (NZ * (((-5e0f64 * (FBG - AY)).tanh()) + ((UA * (FBG + AY)).tanh())));
                            let FXP = ((FXM * EZY) - FUQ) / EZY;
                            let FXQ = rspice_limited_exp(FXP);
                            let FXR = ((FXO * EZY) - FUQ) / EZY;
                            let FXS = if FXP > CEP { 1.0 } else { 0.0 };
                            let FYA;
                            if FXS != 0.0 {
                                FYA = FXP;
                            } else {
                                let FXT = if FXP < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FYB = if FXT != 0.0 {
                                    let FXU = FXP.exp();
                                    FXU
                                } else {
                                    let FXV = (B + (FXP.exp())).ln();
                                    FXV
                                };
                                FYA = FYB;
                            }
                            let FXW = if FXR > CEP { 1.0 } else { 0.0 };
                            let FYC;
                            if FXW != 0.0 {
                                FYC = FXR;
                            } else {
                                let FXX = if FXR < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FYD = if FXX != 0.0 {
                                    let FXY = FXR.exp();
                                    FXY
                                } else {
                                    let FXZ = (B + (FXR.exp())).ln();
                                    FXZ
                                };
                                FYC = FYD;
                            }
                            let FYE = -((FDJ / EZY) + ((FYA - FYC) / CEF));
                            let FYF = rspice_limited_exp(FYE);
                            let FYG = rspice_limited_exp((-FXM));
                            let FYH = FXM * FXM;
                            let FYI = B / (FYH + AY);
                            let FYJ = rspice_limited_exp((FXM - FPD));
                            let FYK = FBG - FXM;
                            let FYL = FBH + FYE;
                            let FYM = FYI * FYH;
                            let FYN = ((FYK * FYK) - (((BWA * BWA) * FYL) * FYL)) - (FBN * (((((FYG - FYF) + FXM) + FYE) + FYJ) - (FPF * ((FXM + B) + FYM))));
                            let FYO = B + FXQ;
                            let FYP = CEF * FYO;
                            let FYQ = AY * FXM;
                            let FYR = FXQ / FYP;
                            let FYS = FXQ * FYF;
                            let FYT = (((((((AY * FXQ) * FYL) * BWA) * BWA) / FYP) - (AY * FBG)) + FYQ) - (FBN * (((((FYJ + (FPF * ((((-2e0f64 * FXM) * FYI) + ((((FYQ * FXM) * FXM) * FYI) * FYI)) - B))) - FYG) - FYR) + (FYS / FYP)) + B));
                            let FYU = ((AY * BWA) * BWA) * FXQ;
                            let FYV = FYU * FXQ;
                            let FYW = FXM - ((FYN / FYT) * (B + ((FYN * ((((((FYU * FYL) / FYP) - (FYV / ((FYP * CEF) * FYO))) - (FBN * (((FYG + FYJ) - (((AY * FPF) * FYI) * (B - (FYM * (UA - ((UI * FYH) * FYI)))))) - (FYR * (((B - (FXQ / FYO)) - FYF) + ((FYS / FYO) * (B + (B / CEF)))))))) - ((FYV * FYL) / (FYP * FYO))) + AY)) / ((AY * FYT) * FYT))));
                            FYX = FYW;
                            FZF = FXO;
                        }
                        let FYY = FYX - FNR;
                        let FYZ = -FPG;
                        let FZA = rspice_limited_exp(FYZ);
                        let FZB = if FYY < CVH { 1.0 } else { 0.0 };
                        let GAY;
                        let GBA;
                        if FZB != 0.0 {
                            let FZC = (FXM * EZY) - FUQ;
                            let FZD = FZC / EZY;
                            let FZE = rspice_limited_exp(FZD);
                            let FZN = ((FZF * EZY) - FUQ) / EZY;
                            let FZO = if FZD > CEP { 1.0 } else { 0.0 };
                            let FZW;
                            if FZO != 0.0 {
                                FZW = FZD;
                            } else {
                                let FZP = if FZD < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FZX = if FZP != 0.0 {
                                    let FZQ = FZD.exp();
                                    FZQ
                                } else {
                                    let FZR = (B + (FZD.exp())).ln();
                                    FZR
                                };
                                FZW = FZX;
                            }
                            let FZS = if FZN > CEP { 1.0 } else { 0.0 };
                            let FZY;
                            if FZS != 0.0 {
                                FZY = FZN;
                            } else {
                                let FZT = if FZN < -3.7e1f64 { 1.0 } else { 0.0 };
                                let FZZ = if FZT != 0.0 {
                                    let FZU = FZN.exp();
                                    FZU
                                } else {
                                    let FZV = (B + (FZN.exp())).ln();
                                    FZV
                                };
                                FZY = FZZ;
                            }
                            let GAA = -((FDJ / EZY) + ((FZW - FZY) / CEF));
                            let GAB = rspice_limited_exp((-FXM));
                            let GAC = B / ((FXM * FXM) + AY);
                            let GAD = (AY * FZC) / EZY;
                            let GAE = rspice_limited_exp(GAD);
                            let GAF = rspice_limited_exp((GAD + GAA));
                            let GAG = AY * FZE;
                            let GAH = FBH + GAA;
                            let GAI = CEF * (FZE + B);
                            let GAJ = AY * FXM;
                            let GAK = FZE / GAI;
                            let GAL = (rspice_limited_exp((GAA + FZD))) / GAI;
                            let GAM = -(((((((GAG * GAH) * BWA) * BWA) / GAI) - (AY * FBG)) + GAJ) - (FBN * ((((((rspice_limited_exp(((FXM - FPG) - FPD))) + ((rspice_limited_exp((FYZ - FPD))) * ((((-2e0f64 * FXM) * GAC) + ((((GAJ * FXM) * FXM) * GAC) * GAC)) - B))) - GAB) - GAK) + GAL) + B)));
                            let GAN = (FBN * (B - FZA)) * FNV;
                            let GAO = (AY * BWA) * BWA;
                            let GAP = GAO * GAE;
                            let GAQ = (B + GAG) + GAE;
                            let GAR = (CEF * CEF) * GAQ;
                            let GAS = CEF * GAQ;
                            let GAT = (GAM * GAM) - (AY * ((((((((GAO * FZE) * GAH) / GAI) - (GAP / GAR)) - (FBN * (((((((GAB + (rspice_limited_exp(((FXM - FPD) - FPG)))) + ((rspice_limited_exp((FPE - FPG))) * (((-2e0f64 * GAC) + ((((SS * FXM) * FXM) * GAC) * GAC)) - (((((((UT * FXM) * FXM) * FXM) * FXM) * GAC) * GAC) * GAC)))) - GAK) + (GAE / GAS)) + GAL) - (GAF / GAS)) - (GAF / GAR)))) - ((GAP * GAH) / GAS)) + AY) * GAN));
                            let GAU = if GAT >= A { 1.0 } else { 0.0 };
                            let GAW = if GAU != 0.0 {
                                let GAV = AY * (GAN / (GAM + (GAT.sqrt())));
                                GAV
                            } else {
                                FYY
                            };
                            let GAX = FNR + GAW;
                            GAY = GAW;
                            GBA = GAX;
                        } else {
                            GAY = FYY;
                            GBA = FYX;
                        }
                        let GAZ = GAY * EZY;
                        let GBB = GBA * GBA;
                        let GBC = NZ * (FNR + GBA);
                        let GBD = (((rspice_limited_exp((-GBA))) * FNU).abs()).sqrt();
                        let GBE = NZ * (FNV + ((rspice_limited_exp((GBA - FPD))) - (FPF * ((GBA + B) + (GBB / (AY + GBB))))));
                        let GBF = GBE + (CXP * ((GAY * GAY) * (GBD - (AY * FBO))));
                        let GBG = FBG - GBC;
                        let GBH = ((GBG * GBG) * FBO) - GBF;
                        let GBI = FBL * ((GBF + GBH).sqrt());
                        let GBJ = GBH - BDB;
                        let GBK = (NZ * (GBJ + (((GBJ * GBJ) + 4.0000000000000007e-10f64).sqrt()))) + BDB;
                        let GBL = if CXX == B { 1.0 } else { 0.0 };
                        let GBY = if GBL != 0.0 {
                            let GBM = (((AY * N) * N) * EZY) / ((C * J) * DS);
                            let GBN = B - GBD;
                            let GBO = B / ((B + (GBM * GBI)).sqrt());
                            let GBP = GBO / (GBO + B);
                            let GBQ = (GBM * (((GBP * GBP) * GBI) * GBI)) * (GBF / (GBF + GBK));
                            let GBR = (AY * (GBI - GBQ)) + (FBN * (GBN + GBF));
                            let GBS = GBQ * (GBQ - (AY * GBI));
                            let GBT = (GBS * GBR) / ((GBR * GBR) - ((B - (NZ * (FBN * (GBD + GBF)))) * GBS));
                            let GBU = rspice_limited_exp(GBT);
                            let GBV = GBF * GBU;
                            let GBW = (FBG - (GBC + GBT)) + GBT;
                            let GBX = (((GAY * GBU) * ((GBN + (AY * (GBI * FBO))) + GBE)) / (((B - (GBD / GBU)) + (AY * (((FBL * ((GBV + (((GBW * GBW) * FBO) - (GBV / GBU))).sqrt())) * GBO) * FBO))) + (GBU * GBE))) * EZY;
                            GBX
                        } else {
                            GAZ
                        };
                        let GBZ = if (GBY.abs()) > CYQ { 1.0 } else { 0.0 };
                        if GBZ != 0.0 {
                        } else {
                        }
                        HXD = HXE;
                        HXR = HXS;
                    }
                    HXC = HXD;
                    HXQ = HXR;
                } else {
                    HXC = HXE;
                    HXQ = HXS;
                }
                HWE = DKA;
                HWF = DNA;
                HWH = DPA;
                HWL = HWM;
                HWQ = HWR;
                HWW = EUX;
                HXB = HXC;
                HXP = HXQ;
                HYN = DNY;
                HYU = B;
                HYV = EWT;
                HYW = EWU;
                HYX = HYY;
                HYZ = HZA;
                HZB = HZC;
                HZE = HZF;
                HZH = HZI;
                HZK = HZL;
                HZN = HZO;
                HZQ = HZR;
                HZT = HZU;
                HZV = HZW;
                HZX = HZY;
                HZZ = IAA;
                IAB = IAC;
                IAD = IAE;
                IAF = A;
                IAH = A;
                IAJ = A;
                IAL = A;
                IAN = A;
                IAP = A;
                IAR = A;
                IAT = A;
                IAV = A;
                IAY = A;
                IBB = A;
                IBE = A;
                IBH = A;
                IBK = A;
                IBN = A;
                IBP = A;
                IBR = A;
                IBT = A;
                IBV = A;
                IBX = A;
                IBZ = A;
                ICB = A;
                ICD = A;
            } else {
                let GCB = BCS / BUS;
                let GCD = (NZ * BVM) - (UH * (B + (BVO / GCC)));
                let GCE = GCD + (((GCD * GCD) + (UJ * BVM)).sqrt());
                let GCF = if BVM < A { 1.0 } else { 0.0 };
                let GCM = if GCF != 0.0 {
                    let GCG = (BVM - GCE) / BVO;
                    let GCH = -((if ((B - GCE) + (GCG * GCG)) >= BAF { ((B - GCE) + (GCG * GCG)) } else { BAF }).ln());
                    GCH
                } else {
                    let GCI = rspice_limited_exp((-GCE));
                    let GCJ = NZ * BVO;
                    let GCK = ((((BVM - B) + GCI) + (GCJ * GCJ)).sqrt()) - GCJ;
                    let GCL = ((GCK * GCK) + B) - GCI;
                    GCL
                };
                let GCN = GCM + B;
                let GCO = GCM - B;
                let GCP = GCO * GCO;
                let GCQ = (NZ * (GCN + ((GCP + 1e0f64).sqrt()))).sqrt();
                let GCR = AY * GCQ;
                let GCS = (B + (BVO / GCR)) / BVO;
                let GCT = GCM - (AY * GCB);
                let GCU = GCT - BVJ;
                let GCV = GCU - ((if ((UI * GCS) * GCQ) >= BAF { ((UI * GCS) * GCQ) } else { BAF }).ln());
                let GCZ = NZ * ((GCV - GCW) - (((GCV * (GCV + GCX)) + GCY).sqrt()));
                let GDA = if GCZ <= -6.8e1f64 { 1.0 } else { 0.0 };
                let GDZ;
                if GDA != 0.0 {
                    let GDC = if GCZ < -1.1e2f64 { 1.0 } else { 0.0 };
                    let GDJ;
                    if GDC != 0.0 {
                        GDJ = GDD;
                    } else {
                        let GDE = if GCZ > -9e1f64 { 1.0 } else { 0.0 };
                        let GDK = if GDE != 0.0 {
                            let GDF = rspice_limited_exp(GCZ);
                            GDF
                        } else {
                            let GDG = (GCZ - GDB) / BQG;
                            let GDH = GDG * GDG;
                            let GDI = rspice_limited_exp((GDB + (BQG * ((7.8125e-2f64 + (NZ * GDG)) + (GDH * (9.375e-1f64 - (GDH * (BYV - GDH))))))));
                            GDI
                        };
                        GDJ = GDK;
                    }
                    let GDL = GDJ * (((B + GCU) - GCZ) - ((if ((AY * GCS) * (((GDJ * AY) * GCS) + GCR)) >= BAF { ((AY * GCS) * (((GDJ * AY) * GCS) + GCR)) } else { BAF }).ln()));
                    GDZ = GDL;
                } else {
                    let GDM = rspice_limited_exp(GCZ);
                    let GDN = AY * GDM;
                    let GDO = GDN * GCS;
                    let GDP = GCS + (B / GCQ);
                    let GDQ = GDM - (((GDN + ((if (GDO * (GDO + GCR)) >= BAF { (GDO * (GDO + GCR)) } else { BAF }).ln())) - GCU) / ((AY + (B / GDM)) + (GDP / ((GCS * GDM) + GCQ))));
                    let GDR = AY * GDQ;
                    let GDS = GDR * GCS;
                    let GDT = (GDR + ((if (GDS * (GDS + GCR)) >= BAF { (GDS * (GDS + GCR)) } else { BAF }).ln())) - GCU;
                    let GDU = B / GDQ;
                    let GDV = (GCS * GDQ) + GCQ;
                    let GDW = GDP / GDV;
                    let GDX = (AY + GDU) + GDW;
                    let GDY = GDQ - ((GDT / GDX) * (B + ((GDT * (((-(GDU * GDU)) - (B / (((GCQ * GCQ) * GCQ) * GDV))) - (GDW * GDW))) / ((AY * GDX) * GDX))));
                    GDZ = GDY;
                }
                let GEA = AY * GDZ;
                let GEB = GCM - GEA;
                let GEC = GEB - B;
                let GED = B + (BVO / (((NZ * (GCN + ((GCP + 1e0f64).sqrt()))).sqrt()) + ((NZ * ((GEB + B) + (((GEC * GEC) + 1e0f64).sqrt()))).sqrt())));
                let GEE = CJM / BTW;
                let GEF = BVM - GCM;
                let GEG = GED - B;
                let GEH = BUT * (GEF - (GEA * GEG));
                let GEI = NZ * (GEH + (((GEH * GEH) + 2.5000000000000005e-3f64).sqrt()));
                let GEJ = ((AY * GED) * BUT) * GDZ;
                let GEK = CJP + (CJQ * BSO);
                let GEL = B + ((GEK * ((GEE * (GEI + (BDM * GEJ))).powf(BEF))) + (CJS / ((NZ * (B + (GEJ / GEI))).powf(CJO))));
                let GEM = GEL - B;
                let GEN = NZ * ((GEL + B) + (((GEM * GEM) + 5.625e-7f64).sqrt()));
                let GEO = B / (((BG * CJW).powf(FR)) * AA);
                let GFF;
                if QS != 0.0 {
                    GFF = A;
                } else {
                    let GEP = (B / (B + (FP * GEJ))) + (QL * BVK);
                    let GEQ = GEP + (((GEP * GEP) + BGT).sqrt());
                    let GFG = if AZD != 0.0 {
                        let GER = (((CJZ + (CKB * GEQ)) * GEO) * AA) * BEW;
                        GER
                    } else {
                        let GES = ((CIZ + (((CJZ + (CKB * GEQ)) * GEO) * AA)) + CIW) * BEW;
                        GES
                    };
                    GFF = GFG;
                }
                let GEU = B / GET;
                let GEV = GEN.powf(GEU);
                let GEW = JO * BSO;
                let GEX = B - GEW;
                let GEY = NZ * (GEX + (((GEX * GEX) + ((BQB + (GEW * GEW)).sqrt())).sqrt()));
                let GEZ = SS * parameters[497];
                let GFA = ((GEZ * GDZ) * GEY) / (GEZ + (GDZ * GEY));
                let GFC = if GFB < A { 1.0 } else { 0.0 };
                let GFI = if GFC != 0.0 {
                    let GFD = (AY * (((CKJ / GEV) * BUT) / (CKH * AZ))) * (B / (B - (GFB * GFA)));
                    GFD
                } else {
                    let GFE = (AY * (((CKJ / GEV) * BUT) / (CKH * AZ))) * (B + (GFB * GFA));
                    GFE
                };
                let GFH = if GFF > A { 1.0 } else { 0.0 };
                let GHY;
                if GFH != 0.0 {
                    let GFJ = (((((((BG * AY) * GED) * N) * BUT) * CKH) * GFI) * GFF) / (AY * BUT);
                    let GFK = NZ * GFI;
                    let GFL = (GDZ * GDZ) + GDZ;
                    let GFM = (GFK * GFL) / (B + (GFK * (B + GDZ)));
                    let GFN = AY * GFI;
                    let GFO = GFN * (GDZ - GFM);
                    let GFP = GFO * GFO;
                    let GFQ = (B + GFP).sqrt();
                    let GFR = if GFO != A { 1.0 } else { 0.0 };
                    let GFV;
                    let GFX;
                    if GFR != 0.0 {
                        let GFS = GFO.asinh();
                        let GFT = GFQ + ((B / GFO) * GFS);
                        GFV = GFT;
                        GFX = GFS;
                    } else {
                        let GFU = GFQ + (B / GFQ);
                        GFV = GFU;
                        GFX = A;
                    }
                    let GFW = ((GFM * GFV) + ((GFJ * GFM) * ((GDZ + GFM) + B))) - (GFI * (GFL - ((GFM * GFM) + GFM)));
                    let GGA = if GFR != 0.0 {
                        let GFY = ((-2e0f64 * GFI) * ((GFO * GFQ) - GFX)) / GFP;
                        GFY
                    } else {
                        let GFZ = (-2e0f64 * GFI) * (GFO / GFQ);
                        GFZ
                    };
                    let GGB = AY * GFM;
                    let GGC = GFM - (GFW / ((((GFM * GGA) + GFV) + (GFJ * ((GDZ + GGB) + B))) + (GFI * (GGB + B))));
                    let GGD = GFN * (GDZ - GGC);
                    let GGE = GGD * GGD;
                    let GGF = (B + GGE).sqrt();
                    let GGG = if GGD != A { 1.0 } else { 0.0 };
                    let GGK;
                    let GGM;
                    if GGG != 0.0 {
                        let GGH = GGD.asinh();
                        let GGI = GGF + ((B / GGD) * GGH);
                        GGK = GGI;
                        GGM = GGH;
                    } else {
                        let GGJ = GGF + (B / GGF);
                        GGK = GGJ;
                        GGM = GFX;
                    }
                    let GGL = ((GGC * GGK) + ((GFJ * GGC) * ((GDZ + GGC) + B))) - (GFI * (GFL - ((GGC * GGC) + GGC)));
                    let GGP = if GGG != 0.0 {
                        let GGN = ((-2e0f64 * GFI) * ((GGD * GGF) - GGM)) / GGE;
                        GGN
                    } else {
                        let GGO = (-2e0f64 * GFI) * (GGD / GGF);
                        GGO
                    };
                    let GGQ = AY * GGC;
                    let GGR = GGC - (GGL / ((((GGC * GGP) + GGK) + (GFJ * ((GDZ + GGQ) + B))) + (GFI * (GGQ + B))));
                    GHY = GGR;
                } else {
                    let GGS = NZ * GFI;
                    let GGT = (GDZ * GDZ) + GDZ;
                    let GGU = (GGS * GGT) / (B + (GGS * (B + GDZ)));
                    let GGV = AY * GFI;
                    let GGW = GGV * (GDZ - GGU);
                    let GGX = GGW * GGW;
                    let GGY = (B + GGX).sqrt();
                    let GGZ = if GGW != A { 1.0 } else { 0.0 };
                    let GHD;
                    let GHF;
                    if GGZ != 0.0 {
                        let GHA = GGW.asinh();
                        let GHB = GGY + ((B / GGW) * GHA);
                        GHD = GHB;
                        GHF = GHA;
                    } else {
                        let GHC = GGY + (B / GGY);
                        GHD = GHC;
                        GHF = A;
                    }
                    let GHE = (GGU * GHD) - (GFI * (GGT - ((GGU * GGU) + GGU)));
                    let GHI = if GGZ != 0.0 {
                        let GHG = ((-2e0f64 * GFI) * ((GGW * GGY) - GHF)) / GGX;
                        GHG
                    } else {
                        let GHH = (-2e0f64 * GFI) * (GGW / GGY);
                        GHH
                    };
                    let GHJ = GGU - (GHE / (((GGU * GHI) + GHD) + (GFI * ((AY * GGU) + B))));
                    let GHK = GGV * (GDZ - GHJ);
                    let GHL = GHK * GHK;
                    let GHM = (B + GHL).sqrt();
                    let GHN = if GHK != A { 1.0 } else { 0.0 };
                    let GHR;
                    let GHT;
                    if GHN != 0.0 {
                        let GHO = GHK.asinh();
                        let GHP = GHM + ((B / GHK) * GHO);
                        GHR = GHP;
                        GHT = GHO;
                    } else {
                        let GHQ = GHM + (B / GHM);
                        GHR = GHQ;
                        GHT = GHF;
                    }
                    let GHS = (GHJ * GHR) - (GFI * (GGT - ((GHJ * GHJ) + GHJ)));
                    let GHW = if GHN != 0.0 {
                        let GHU = ((-2e0f64 * GFI) * ((GHK * GHM) - GHT)) / GHL;
                        GHU
                    } else {
                        let GHV = (-2e0f64 * GFI) * (GHK / GHM);
                        GHV
                    };
                    let GHX = GHJ - (GHS / (((GHJ * GHW) + GHR) + (GFI * ((AY * GHJ) + B))));
                    GHY = GHX;
                }
                let GHZ = AY * GHY;
                let GIA = (GHZ * GED) * BVU;
                let GIB = (GCT - (GHZ + ((if (GIA * (GIA + (BVO / GEG))) >= BAF { (GIA * (GIA + (BVO / GEG))) } else { BAF }).ln()))) * BUT;
                let GIC = if (if CKV == A { 1.0 } else { 0.0 }) != 0.0 && (if CKW == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GII = if GIC != 0.0 {
                    B
                } else {
                    let GID = AZ / (AZ + ((FA * BUJ).sqrt()));
                    let GIE = B + (((CKV * GID) - (((CKW * GID) * (GDZ.powf(CKZ))) * BUT)) / (B + (CLA * BSO)));
                    let GIF = GIE - BQB;
                    let GIG = NZ * ((GIE + BQB) + (((GIF * GIF) + 6.25e-8f64).sqrt()));
                    GIG
                };
                let GIH = GIB - BSD;
                let GIJ = (NZ * (GIH + (((GIH * GIH) + 2.5e-7f64).sqrt()))) / GII;
                let GIK = B / BFH;
                let GIL = -BFH;
                let GIM = BSE * ((B + (((BSE / GIJ) + BX).powf(GIK))).powf(GIL));
                let GIN = (NZ * (GCN + ((GCP + 1e0f64).sqrt()))).sqrt();
                let GIO = AY * GIN;
                let GIP = (B + (BVO / GIO)) / BVO;
                let GIQ = GCT - ((GIM + BSD) * BUU);
                let GIR = GIQ - ((if ((UI * GIP) * GIN) >= BAF { ((UI * GIP) * GIN) } else { BAF }).ln());
                let GIS = NZ * ((GIR - GCW) - (((GIR * (GIR + GCX)) + GCY).sqrt()));
                let GIT = if GIS <= -6.8e1f64 { 1.0 } else { 0.0 };
                let GJS;
                if GIT != 0.0 {
                    let GIV = if GIS < -1.1e2f64 { 1.0 } else { 0.0 };
                    let GJC;
                    if GIV != 0.0 {
                        GJC = GIW;
                    } else {
                        let GIX = if GIS > -9e1f64 { 1.0 } else { 0.0 };
                        let GJD = if GIX != 0.0 {
                            let GIY = rspice_limited_exp(GIS);
                            GIY
                        } else {
                            let GIZ = (GIS - GIU) / BQG;
                            let GJA = GIZ * GIZ;
                            let GJB = rspice_limited_exp((GIU + (BQG * ((7.8125e-2f64 + (NZ * GIZ)) + (GJA * (9.375e-1f64 - (GJA * (BYV - GJA))))))));
                            GJB
                        };
                        GJC = GJD;
                    }
                    let GJE = GJC * (((B + GIQ) - GIS) - ((if ((AY * GIP) * (((GJC * AY) * GIP) + GIO)) >= BAF { ((AY * GIP) * (((GJC * AY) * GIP) + GIO)) } else { BAF }).ln()));
                    GJS = GJE;
                } else {
                    let GJF = rspice_limited_exp(GIS);
                    let GJG = AY * GJF;
                    let GJH = GJG * GIP;
                    let GJI = GIP + (B / GIN);
                    let GJJ = GJF - (((GJG + ((if (GJH * (GJH + GIO)) >= BAF { (GJH * (GJH + GIO)) } else { BAF }).ln())) - GIQ) / ((AY + (B / GJF)) + (GJI / ((GIP * GJF) + GIN))));
                    let GJK = AY * GJJ;
                    let GJL = GJK * GIP;
                    let GJM = (GJK + ((if (GJL * (GJL + GIO)) >= BAF { (GJL * (GJL + GIO)) } else { BAF }).ln())) - GIQ;
                    let GJN = B / GJJ;
                    let GJO = (GIP * GJJ) + GIN;
                    let GJP = GJI / GJO;
                    let GJQ = (AY + GJN) + GJP;
                    let GJR = GJJ - ((GJM / GJQ) * (B + ((GJM * (((-(GJN * GJN)) - (B / (((GIN * GIN) * GIN) * GJO))) - (GJP * GJP))) / ((AY * GJQ) * GJQ))));
                    GJS = GJR;
                }
                let GJT = ((GCM - GDZ) - GJS) - B;
                let GJU = GJT - B;
                let GJV = B + (BVO / (GIN + ((NZ * ((GJT + B) + (((GJU * GJU) + 1e0f64).sqrt()))).sqrt())));
                let GJW = GDZ - GJS;
                let GJX = GJW * GJW;
                let GJY = (B + GDZ) + GJS;
                let GJZ = B / GJY;
                let GKA = GJX * GJZ;
                let GKB = GDZ + GJS;
                let GKC = BDI * GJV;
                let GKD = GKA * GJZ;
                let GKG = GKC * ((GEA + GJS) + ((NZ * ((B + (GKE * GDZ)) + (GKF * GJS))) * GKD));
                let GKH = GKC * ((GDZ + (AY * GJS)) + ((NZ * ((B + (GKF * GDZ)) + (GKE * GJS))) * GKD));
                let GKI = BUT * (GEF - ((GJV - B) * (GKB + (BDI * GKA))));
                let GKJ = NZ * (GKI + (((GKI * GKI) + 2.5000000000000005e-3f64).sqrt()));
                let GKK = BUT * (GKG + GKH);
                let GKL = B + ((GEK * ((GEE * (GKJ + (BDM * GKK))).powf(BEF))) + (CJS / ((NZ * (B + (GKK / GKJ))).powf(CJO))));
                let GKM = GKL - B;
                let GKN = NZ * ((GKL + B) + (((GKM * GKM) + 5.625e-7f64).sqrt()));
                let GKO = AY * CKH;
                let GKP = (GKO / (CKJ / GKN)) * AZ;
                let GKQ = if GE > A { 1.0 } else { 0.0 };
                let GKX = if GKQ != 0.0 {
                    let GKR = B + ((GE * GKK) / GKP);
                    GKR
                } else {
                    let GKS = B / (B - ((GE * GKK) / GKP));
                    GKS
                };
                let GKT = BSE - GIM;
                let GKU = GKK + (AY * BUT);
                let GKV = if CZH > A { 1.0 } else { 0.0 };
                let GLO = if GKV != 0.0 {
                    let GKW = B + (FY * BSO);
                    let GKY = B + (GKT / ((((GKU / CZH) * (GKU / (GIJ + GKU))) * GKX) * (B / (NZ * (GKW + (((GKW * GKW) + 4e-6f64).sqrt()))))));
                    GKY
                } else {
                    B
                };
                let GKZ = if OC <= A { 1.0 } else { 0.0 };
                let GLE = if GKZ != 0.0 {
                    B
                } else {
                    let GLA = B / (B + ((OC * (AZ.sqrt())) / GKU));
                    GLA
                };
                let GLB = GIJ + GKP;
                let GLC = if CZR > A { 1.0 } else { 0.0 };
                let GLP;
                if GLC != 0.0 {
                    let GLD = if CZT < A { 1.0 } else { 0.0 };
                    let GLH = if GLD != 0.0 {
                        let GLF = (CZR / (B - ((CZT * GKK) / GKP))) / GLE;
                        GLF
                    } else {
                        let GLG = (CZR * (B + ((CZT * GKK) / GKP))) / GLE;
                        GLG
                    };
                    let GLI = B + (GLH * ((if (B + ((GKT / GLH) / GLB)) >= BAF { (B + ((GKT / GLH) / GLB)) } else { BAF }).ln()));
                    GLP = GLI;
                } else {
                    let GLJ = if CZT < A { 1.0 } else { 0.0 };
                    let GLM = if GLJ != 0.0 {
                        let GLK = (CZR / (B - ((CZT * GKK) / GKP))) / GLE;
                        GLK
                    } else {
                        let GLL = (CZR * (B + ((CZT * GKK) / GKP))) / GLE;
                        GLL
                    };
                    let GLN = B + GLM;
                    GLP = GLN;
                }
                let GLQ = GLO * GLP;
                let GLR = rspice_limited_exp((GC * BSE));
                let GLS = if GB > A { 1.0 } else { 0.0 };
                let GLU = if GLS != 0.0 {
                    let GLT = ((B + ((B + (DAK * AZ)) * GLR)) / GB) * GLE;
                    GLT
                } else {
                    DAM
                };
                let GLV = GLQ * (B + (GKT / GLU));
                let GLW = if GA > A { 1.0 } else { 0.0 };
                let GMB;
                if GLW != 0.0 {
                    let GLX = FZ * BCY;
                    let GLY = if GKT > (GLX / BSH) { 1.0 } else { 0.0 };
                    let GMC = if GLY != 0.0 {
                        let GLZ = (AZ * (rspice_limited_exp((GLX / GKT)))) / GA;
                        GLZ
                    } else {
                        let GMA = (DAM * AZ) / GA;
                        GMA
                    };
                    GMB = GMC;
                } else {
                    GMB = DAM;
                }
                let GMD = GLV * (B + (GKT / GMB));
                let GME = GKN.powf(GEU);
                let GMF = ((GEZ * GKK) * GEY) / (GEZ + (GKK * GEY));
                let GMI = if GFC != 0.0 {
                    let GMG = (AY * (((CKJ / GME) * BUT) / (CKH * AZ))) * (B / (B - (GFB * GMF)));
                    GMG
                } else {
                    let GMH = (AY * (((CKJ / GME) * BUT) / (CKH * AZ))) * (B + (GFB * GMF));
                    GMH
                };
                let GMJ = (AY * GMI) * GJW;
                let GMK = (B + (GMJ * GMJ)).sqrt();
                let GML = if GMJ != A { 1.0 } else { 0.0 };
                let GMO = if GML != 0.0 {
                    let GMM = NZ * (GMK + ((B / GMJ) * (GMJ.asinh())));
                    GMM
                } else {
                    let GMN = NZ * (GMK + (B / GMK));
                    GMN
                };
                let GNK;
                let HEE;
                let HXJ;
                let HXX;
                if QS != 0.0 {
                    let GMP = BRQ - DBT;
                    let GMQ = (B / (B + (FP * (NZ * (GMP + (((GMP * GMP) + BGT).sqrt())))))) + (QL * BRK);
                    let GMR = BEW * (CIZ + ((DBW + (DBY * (NZ * (GMQ + (((GMQ * GMQ) + BGT).sqrt()))))) * GEO));
                    let GMS = BRP - DBT;
                    let GMT = (B / (B + (FP * (NZ * (GMS + (((GMS * GMS) + BGT).sqrt())))))) + (QL * BRH);
                    let GMU = BEW * (CIW + ((DCD + (DCF * (NZ * (GMT + (((GMT * GMT) + BGT).sqrt()))))) * GEO));
                    GNK = B;
                    HEE = A;
                    HXJ = GMU;
                    HXX = GMR;
                } else {
                    let GMV = (B / (B + (FP * GKK))) + (QL * BVK);
                    let GMW = CJZ + (CKB * (NZ * (GMV + (((GMV * GMV) + BGT).sqrt()))));
                    let GMX = ((BEW * GMW) * GEO) * AA;
                    let GMY = ((((CKJ / (GMO * GKN)) * N) * BG) / AZ) * GKK;
                    let GMZ = B + (GMY * GMX);
                    let GNA = if QR == AY { 1.0 } else { 0.0 };
                    let GNL;
                    let HEF;
                    let HXK;
                    let HXY;
                    if GNA != 0.0 {
                        let GNB = BEW * ((CIZ + ((GMW * GEO) * AA)) + CIW);
                        let GNC = B + (GMY * GNB);
                        GNL = GNC;
                        HEF = GNB;
                        HXK = A;
                        HXY = A;
                    } else {
                        GNL = GMZ;
                        HEF = GMX;
                        HXK = CIW;
                        HXY = CIZ;
                    }
                    GNK = GNL;
                    HEE = HEF;
                    HXJ = HXK;
                    HXX = HXY;
                }
                let GND = (AY * BUS) * BBM;
                let GNE = ((((BFO + (BFQ / (GKK + GND))) * GJW) * GJW) + B) - BDB;
                let GNF = NZ * (B + ((B + (-1e0f64 + (NZ * (GNE + (((GNE * GNE) + DCT).sqrt()))))).sqrt()));
                let GNG = GNF - B;
                let GNH = GJW / (GKB + BFZ);
                let GNI = B + ((BFW * GNH) * GNH);
                let GNJ = rspice_limited_exp((-(BGB / (((if A >= (BGD + ((BGF * GJW) * GJW)) { A } else { (BGD + ((BGF * GJW) * GJW)) }) * GKB) + GND))));
                let GNM = CKJ / ((GKN * GMO) * GNK);
                let GNN = AY * AA;
                let GNO = ((((((((((((GNN * GJV) * GNM) * BG) / AZ) * N) * BUT) * BUT) * (GJW * GJY)) * GMD) / ((NZ * ((GNF + B) - (((GNG * GNG) + 2.5e-5f64).sqrt()))) + 2.5e-3f64)) * GNI) * GNJ) * parameters[26];
                let GNP = if BAB > B { 1.0 } else { 0.0 };
                let HWO;
                let HWT;
                if GNP != 0.0 {
                    let GNQ = (DDX * AA) * ((((((DDW * BBM) * GNM) * BG) / AZ) * N) + ((((GNM * BG) / AZ) * N) * GKK));
                    let GNR = if BAB == AY { 1.0 } else { 0.0 };
                    let HWP;
                    let HWU;
                    if GNR != 0.0 {
                        let GNS = if (B / DEA) < AZG { 1.0 } else { 0.0 };
                        let GNU = if GNS != 0.0 {
                            let GNT = B / AZG;
                            GNT
                        } else {
                            DEA
                        };
                        let GNV = (GNU * GNQ) / (GNU + GNQ);
                        HWP = GNV;
                        HWU = GNU;
                    } else {
                        HWP = GNQ;
                        HWU = DEA;
                    }
                    HWO = HWP;
                    HWT = HWU;
                } else {
                    HWO = A;
                    HWT = DEA;
                }
                let GNW = BBM * DEF;
                let GNX = rspice_limited_exp((BRN / GNW));
                let GNY = rspice_limited_exp((BRO / GNW));
                let GNZ = (DEJ / BBM) * BCZ;
                let GOA = if GV == A { 1.0 } else { 0.0 };
                if GOA != 0.0 {
                } else {
                }
                let GOB = if GW == A { 1.0 } else { 0.0 };
                if GOB != 0.0 {
                } else {
                }
                let GOC = if GZ == A { 1.0 } else { 0.0 };
                if GOC != 0.0 {
                } else {
                    let GOD = if (HF - BRN) < BDB { 1.0 } else { 0.0 };
                    if GOD != 0.0 {
                    } else {
                    }
                }
                let GOE = if HA == A { 1.0 } else { 0.0 };
                if GOE != 0.0 {
                } else {
                    let GOF = if (HG - BRO) < BDB { 1.0 } else { 0.0 };
                    if GOF != 0.0 {
                    } else {
                    }
                }
                let GOG = BAP * BTV;
                let GOH = if (if HN == A { 1.0 } else { 0.0 }) != 0.0 && (if HO == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GVV;
                let GYO;
                if GOH != 0.0 {
                    GVV = A;
                    GYO = A;
                } else {
                    let GOI = rspice_limited_exp(((HL * GNZ) / DEF));
                    let GOJ = HN * GOI;
                    let GOK = HK * GOI;
                    let GOL = HO * GOI;
                    let GOM = GNX - B;
                    let GON = (HJ * GOI) * GOM;
                    let GOO = if GON < CJH { 1.0 } else { 0.0 };
                    let GOU;
                    let GPE;
                    if GOO != 0.0 {
                        GOU = B;
                        GPE = A;
                    } else {
                        let GOP = B / ((B + GON).sqrt());
                        GOU = GOP;
                        GPE = GON;
                    }
                    let GOQ = GNY - B;
                    let GOR = GOK * GOQ;
                    let GOS = if GOR < CJH { 1.0 } else { 0.0 };
                    let GOV;
                    let GPF;
                    if GOS != 0.0 {
                        GOV = B;
                        GPF = A;
                    } else {
                        let GOT = B / ((B + GOR).sqrt());
                        GOV = GOT;
                        GPF = GOR;
                    }
                    let GOW = B + (DFI * ((HP * ((B / AZ) + (B / DFF))).powf(HQ)));
                    let GOX = (((GOG * GOJ) * GOW) * GOM) * GOU;
                    let GOY = (((GOG * GOL) * GOW) * GOQ) * GOV;
                    let GOZ = HH + (HI * AZ);
                    let GPA = if GOZ < B { 1.0 } else { 0.0 };
                    let GPC = if GPA != 0.0 {
                        B
                    } else {
                        GOZ
                    };
                    let GPB = if DFO == B { 1.0 } else { 0.0 };
                    if GPB != 0.0 {
                    } else {
                        let GPD = B + ((BRN + BRO) / GPC);
                        let GPG = if ((GPD + (((GPD * GPD) + (UI * (GPE + GPF))).sqrt())) / AY) < BQB { 1.0 } else { 0.0 };
                        if GPG != 0.0 {
                        } else {
                        }
                    }
                    GVV = GOX;
                    GYO = GOY;
                }
                let GPH = if (if HB == A { 1.0 } else { 0.0 }) != 0.0 && (if HC == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GPH != 0.0 {
                } else {
                    let GPI = if (HD - BRN) < BDB { 1.0 } else { 0.0 };
                    if GPI != 0.0 {
                    } else {
                    }
                    let GPJ = if (HE - BRO) < BDB { 1.0 } else { 0.0 };
                    if GPJ != 0.0 {
                    } else {
                    }
                }
                let GPK = if DFY == A { 1.0 } else { 0.0 };
                if GPK != 0.0 {
                    let GPL = if (if (if PB <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFS <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || RE != 0.0 { 1.0 } else { 0.0 };
                    if GPL != 0.0 {
                    } else {
                        let GPM = if HX != A { 1.0 } else { 0.0 };
                        if GPM != 0.0 {
                        } else {
                        }
                    }
                    let GPN = if (if (if PC <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFT <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || RF != 0.0 { 1.0 } else { 0.0 };
                    if GPN != 0.0 {
                    } else {
                        let GPO = if IB != A { 1.0 } else { 0.0 };
                        if GPO != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let GPP = if (if PB <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFS <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GPP != 0.0 {
                    } else {
                        let GPQ = if ID != A { 1.0 } else { 0.0 };
                        if GPQ != 0.0 {
                        } else {
                        }
                    }
                    let GPR = if (if PC <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFT <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GPR != 0.0 {
                    } else {
                        let GPS = if IC != A { 1.0 } else { 0.0 };
                        if GPS != 0.0 {
                        } else {
                        }
                    }
                }
                let GPT = if DGJ == A { 1.0 } else { 0.0 };
                if GPT != 0.0 {
                    let GPU = if (if PA <= A { 1.0 } else { 0.0 }) != 0.0 || (if BFR <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GPU != 0.0 {
                    } else {
                        let GPV = if GKT > (BFR / BSH) { 1.0 } else { 0.0 };
                        if GPV != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let GPW = if DGJ == B { 1.0 } else { 0.0 };
                    if GPW != 0.0 {
                        let GPX = if (if PA <= A { 1.0 } else { 0.0 }) != 0.0 || (if (if (if GQ == A { 1.0 } else { 0.0 }) != 0.0 && (if GP == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BFR == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if GPX != 0.0 {
                        } else {
                        }
                    } else {
                        let GPY = if (if PA <= A { 1.0 } else { 0.0 }) != 0.0 || (if (if (if GQ == A { 1.0 } else { 0.0 }) != 0.0 && (if GP == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BFR == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if GPY != 0.0 {
                        } else {
                        }
                        let GPZ = GR * (B + (DGS * BCZ));
                        let GQA = if DGQ > A { 1.0 } else { 0.0 };
                        let GQD = if GQA != 0.0 {
                            let GQB = GPZ - BRO;
                            GQB
                        } else {
                            let GQC = GPZ - BRN;
                            GQC
                        };
                        let GQE = if GQD > A { 1.0 } else { 0.0 };
                        if GQE != 0.0 {
                        } else {
                        }
                    }
                }
                let GQF = PD + (IX * BCZ);
                let GQG = PE + (JB * BCZ);
                let GQH = PF + (JF * BCZ);
                let GQI = IT + (IU * BCZ);
                let GQJ = IQ + (IR * BCZ);
                let GQK = if ERC != A { 1.0 } else { 0.0 };
                let GQL = if RW != 0.0 || GQK != 0.0 { 1.0 } else { 0.0 };
                let GTB;
                let GTD;
                let GTF;
                let GTH;
                let GTK;
                if GQL != 0.0 {
                    let GQM = BUT * ((GEF + GDZ) + GJS);
                    let GQN = ((GQM * GQM) + DGC).sqrt();
                    let GQO = NZ * ((-GQM) + GQN);
                    let GQP = NZ * (GQM + GQN);
                    let GTG;
                    if GQK != 0.0 {
                        let GQQ = -(GQM / ERN);
                        let GQR = if GQQ > CEP { 1.0 } else { 0.0 };
                        let GQV;
                        if GQR != 0.0 {
                            GQV = GQQ;
                        } else {
                            let GQS = if GQQ < -3.7e1f64 { 1.0 } else { 0.0 };
                            let GQW = if GQS != 0.0 {
                                let GQT = GQQ.exp();
                                GQT
                            } else {
                                let GQU = (B + (GQQ.exp())).ln();
                                GQU
                            };
                            GQV = GQW;
                        }
                        let GQX = ERN * GQV;
                        let GQY = if ERW != A { 1.0 } else { 0.0 };
                        let GRA = if GQY != 0.0 {
                            let GQZ = B - (GQO / ERW);
                            GQZ
                        } else {
                            B
                        };
                        let GRB = if GRA < BGT { 1.0 } else { 0.0 };
                        let GRD = if GRB != 0.0 {
                            BGT
                        } else {
                            GRA
                        };
                        let GRC = ((AZ * BG) / BAO) + BAU;
                        let GRE = (((((GRC * ESC) * BAG) * BRC) * GQX) * (rspice_limited_exp((((ESD * M) * (GQJ - (IS * GQO))) / GRD)))) * BFU;
                        let GRF = (GQM - IP) / ERN;
                        let GRG = if GRF > CEP { 1.0 } else { 0.0 };
                        let GRK;
                        if GRG != 0.0 {
                            GRK = GRF;
                        } else {
                            let GRH = if GRF < -3.7e1f64 { 1.0 } else { 0.0 };
                            let GRL = if GRH != 0.0 {
                                let GRI = GRF.exp();
                                GRI
                            } else {
                                let GRJ = (B + (GRF.exp())).ln();
                                GRJ
                            };
                            GRK = GRL;
                        }
                        let GRM = ERN * GRK;
                        let GRN = if ESO != A { 1.0 } else { 0.0 };
                        let GRP = if GRN != 0.0 {
                            let GRO = B - (GQP / ESO);
                            GRO
                        } else {
                            B
                        };
                        let GRQ = if GRP < BGT { 1.0 } else { 0.0 };
                        let GRR = if GRQ != 0.0 {
                            BGT
                        } else {
                            GRP
                        };
                        let GRS = AA * (GRE + ((((((GRC * EST) * BAG) * BRC) * GRM) * (rspice_limited_exp((((ESU * M) * (GQI - (IV * GQP))) / GRR)))) * BFU));
                        GTG = GRS;
                    } else {
                        GTG = A;
                    }
                    let GRT = if (if (if (if BCN != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BCG == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if EQJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GRT != 0.0 {
                        if F != 0.0 {
                        } else {
                        }
                        if F != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    let GTC;
                    let GTE;
                    let GTI;
                    let GTL;
                    if RW != 0.0 {
                        let GRU = (((AA * BAV) * (((GJV * BUT) * GKB) * (rspice_limited_exp(((BAS * (GQF - (IY * GQP))) * (B + (IZ * GQP))))))) * ((BRC + (NZ * BSM)) - (NZ * (BSD + BSC)))) * BFU;
                        let GRV = PG * ((((GIM * GIM) + BGT).sqrt()) - BQB);
                        let GRW = rspice_limited_exp((-GRV));
                        let GRX = ((GRV + GRW) - B) + DGC;
                        let GRY = (B - ((GRV + B) * GRW)) + DGC;
                        let GRZ = (GRV * GRV) + ETG;
                        let GSA = if DGQ > A { 1.0 } else { 0.0 };
                        let GTJ;
                        let GTM;
                        if GSA != 0.0 {
                            let GSB = (GRU * GRY) / GRZ;
                            let GSC = (GRU * GRX) / GRZ;
                            GTJ = GSC;
                            GTM = GSB;
                        } else {
                            let GSD = (GRU * GRY) / GRZ;
                            let GSE = (GRU * GRX) / GRZ;
                            GTJ = GSD;
                            GTM = GSE;
                        }
                        let GSF = BRQ - DBT;
                        let GSG = ((GSF * GSF) + DGC).sqrt();
                        let GSH = if ETO == B { 1.0 } else { 0.0 };
                        let GSM;
                        let GSO;
                        if GSH != 0.0 {
                            let GSI = GQG - (JC * GSG);
                            let GSJ = NZ * (GSI + (((GSI * GSI) + 4e-12f64).sqrt()));
                            let GSK = if JD < BGT { 1.0 } else { 0.0 };
                            let GSN = if GSK != 0.0 {
                                BGT
                            } else {
                                JD
                            };
                            GSM = GSN;
                            GSO = GSJ;
                        } else {
                            let GSL = GQG - (JC * GSG);
                            GSM = JD;
                            GSO = GSL;
                        }
                        let GSP = BFU * AA;
                        let GSQ = ((((GSP * BAQ) * ETY) * BRQ) * GSG) * (rspice_limited_exp(((BAT * GSO) * (B + (GSM * GSG)))));
                        let GSR = BRP - DBT;
                        let GSS = ((GSR * GSR) + DGC).sqrt();
                        let GSX;
                        let GSZ;
                        if GSH != 0.0 {
                            let GST = GQH - (JG * GSS);
                            let GSU = NZ * (GST + (((GST * GST) + 4e-12f64).sqrt()));
                            let GSV = if JH < BGT { 1.0 } else { 0.0 };
                            let GSY = if GSV != 0.0 {
                                BGT
                            } else {
                                JH
                            };
                            GSX = GSY;
                            GSZ = GSU;
                        } else {
                            let GSW = GQH - (JG * GSS);
                            GSX = JH;
                            GSZ = GSW;
                        }
                        let GTA = ((((GSP * BAR) * EUJ) * BRP) * GSS) * (rspice_limited_exp(((BAT * GSZ) * (B + (GSX * GSS)))));
                        GTC = GSQ;
                        GTE = GTA;
                        GTI = GTJ;
                        GTL = GTM;
                    } else {
                        GTC = A;
                        GTE = A;
                        GTI = A;
                        GTL = A;
                    }
                    GTB = GTC;
                    GTD = GTE;
                    GTF = GTG;
                    GTH = GTI;
                    GTK = GTL;
                } else {
                    GTB = A;
                    GTD = A;
                    GTF = A;
                    GTH = A;
                    GTK = A;
                }
                let GTN = BGI * DGZ;
                let GTO = BGM * DHB;
                let GTP = (BGQ * BV) * AA;
                let GTQ = -DHG;
                let GTR = BQB.powf(GTQ);
                let GTS = if DHG == B { 1.0 } else { 0.0 };
                let GUR = if GTS != 0.0 {
                    GTT
                } else {
                    let GTU = (B / (B - DHG)) * (B - (((BQE * DHG) * (B + DHG)) * GTR));
                    GTU
                };
                let GTV = -DHM;
                let GTW = BQB.powf(GTV);
                let GTX = if DHM == B { 1.0 } else { 0.0 };
                let GVF = if GTX != 0.0 {
                    GTY
                } else {
                    let GTZ = (B / (B - DHM)) * (B - (((BQE * DHM) * (B + DHM)) * GTW));
                    GTZ
                };
                let GUA = -DHS;
                let GUB = BQB.powf(GUA);
                let GUC = if DHS == B { 1.0 } else { 0.0 };
                let GVT = if GUC != 0.0 {
                    GUD
                } else {
                    let GUE = (B / (B - DHS)) * (B - (((BQE * DHS) * (B + DHS)) * GUB));
                    GUE
                };
                let GUF = if GTN > A { 1.0 } else { 0.0 };
                let GVW;
                if GUF != 0.0 {
                    let GUG = BRN / BGV;
                    let GUH = if GUG < DIA { 1.0 } else { 0.0 };
                    let GVX;
                    if GUH != 0.0 {
                        let GUI = B - GUG;
                        let GUJ = if DHG != B { 1.0 } else { 0.0 };
                        let GVY;
                        if GUJ != 0.0 {
                            let GUK = if DHG == NZ { 1.0 } else { 0.0 };
                            let GUN = if GUK != 0.0 {
                                let GUL = B / (GUI.sqrt());
                                GUL
                            } else {
                                let GUM = rspice_limited_exp((GTQ * (GUI.ln())));
                                GUM
                            };
                            let GUO = ((BGV * GTN) * (B - (GUI * GUN))) / (B - DHG);
                            GVY = GUO;
                        } else {
                            let GUP = (BGV * GTN) * (-(GUI.ln()));
                            GVY = GUP;
                        }
                        GVX = GVY;
                    } else {
                        let GUQ = GUG - B;
                        let GUS = (BGV * GTN) * (((GTR * GUQ) * (((UA * DHG) * GUQ) + (B + DHG))) + GUR);
                        GVX = GUS;
                    }
                    GVW = GVX;
                } else {
                    GVW = A;
                }
                let GUT = if GTO > A { 1.0 } else { 0.0 };
                let GVZ;
                if GUT != 0.0 {
                    let GUU = BRN / BHA;
                    let GUV = if GUU < DIA { 1.0 } else { 0.0 };
                    let GWA;
                    if GUV != 0.0 {
                        let GUW = B - GUU;
                        let GUX = if DHM != B { 1.0 } else { 0.0 };
                        let GWB;
                        if GUX != 0.0 {
                            let GUY = if DHM == NZ { 1.0 } else { 0.0 };
                            let GVB = if GUY != 0.0 {
                                let GUZ = B / (GUW.sqrt());
                                GUZ
                            } else {
                                let GVA = rspice_limited_exp((GTV * (GUW.ln())));
                                GVA
                            };
                            let GVC = ((BHA * GTO) * (B - (GUW * GVB))) / (B - DHM);
                            GWB = GVC;
                        } else {
                            let GVD = (BHA * GTO) * (-(GUW.ln()));
                            GWB = GVD;
                        }
                        GWA = GWB;
                    } else {
                        let GVE = GUU - B;
                        let GVG = (BHA * GTO) * (((GTW * GVE) * (((UA * DHM) * GVE) + (B + DHM))) + GVF);
                        GWA = GVG;
                    }
                    GVZ = GWA;
                } else {
                    GVZ = A;
                }
                let GVH = if GTP > A { 1.0 } else { 0.0 };
                let GWC;
                if GVH != 0.0 {
                    let GVI = BRN / BHF;
                    let GVJ = if GVI < DIA { 1.0 } else { 0.0 };
                    let GWD;
                    if GVJ != 0.0 {
                        let GVK = B - GVI;
                        let GVL = if DHS != B { 1.0 } else { 0.0 };
                        let GWE;
                        if GVL != 0.0 {
                            let GVM = if DHS == NZ { 1.0 } else { 0.0 };
                            let GVP = if GVM != 0.0 {
                                let GVN = B / (GVK.sqrt());
                                GVN
                            } else {
                                let GVO = rspice_limited_exp((GUA * (GVK.ln())));
                                GVO
                            };
                            let GVQ = ((BHF * GTP) * (B - (GVK * GVP))) / (B - DHS);
                            GWE = GVQ;
                        } else {
                            let GVR = (BHF * GTP) * (-(GVK.ln()));
                            GWE = GVR;
                        }
                        GWD = GWE;
                    } else {
                        let GVS = GVI - B;
                        let GVU = (BHF * GTP) * (((GUB * GVS) * (((UA * DHS) * GVS) + (B + DHS))) + GVT);
                        GWD = GVU;
                    }
                    GWC = GWD;
                } else {
                    GWC = A;
                }
                let GWF = ((GVW + GVZ) + GWC) + ((DJP * GVV) * AA);
                let GWG = BGJ * DKB;
                let GWH = BGN * DKD;
                let GWI = (BGR * BV) * AA;
                let GWJ = -DKI;
                let GWK = BQB.powf(GWJ);
                let GWL = if DKI == B { 1.0 } else { 0.0 };
                let GXK = if GWL != 0.0 {
                    GWM
                } else {
                    let GWN = (B / (B - DKI)) * (B - (((BQE * DKI) * (B + DKI)) * GWK));
                    GWN
                };
                let GWO = -DKO;
                let GWP = BQB.powf(GWO);
                let GWQ = if DKO == B { 1.0 } else { 0.0 };
                let GXY = if GWQ != 0.0 {
                    GWR
                } else {
                    let GWS = (B / (B - DKO)) * (B - (((BQE * DKO) * (B + DKO)) * GWP));
                    GWS
                };
                let GWT = -DKU;
                let GWU = BQB.powf(GWT);
                let GWV = if DKU == B { 1.0 } else { 0.0 };
                let GYM = if GWV != 0.0 {
                    GWW
                } else {
                    let GWX = (B / (B - DKU)) * (B - (((BQE * DKU) * (B + DKU)) * GWU));
                    GWX
                };
                let GWY = if GWG > A { 1.0 } else { 0.0 };
                let GYP;
                if GWY != 0.0 {
                    let GWZ = BRO / BGX;
                    let GXA = if GWZ < DIA { 1.0 } else { 0.0 };
                    let GYQ;
                    if GXA != 0.0 {
                        let GXB = B - GWZ;
                        let GXC = if DKI != B { 1.0 } else { 0.0 };
                        let GYR;
                        if GXC != 0.0 {
                            let GXD = if DKI == NZ { 1.0 } else { 0.0 };
                            let GXG = if GXD != 0.0 {
                                let GXE = B / (GXB.sqrt());
                                GXE
                            } else {
                                let GXF = rspice_limited_exp((GWJ * (GXB.ln())));
                                GXF
                            };
                            let GXH = ((BGX * GWG) * (B - (GXB * GXG))) / (B - DKI);
                            GYR = GXH;
                        } else {
                            let GXI = (BGX * GWG) * (-(GXB.ln()));
                            GYR = GXI;
                        }
                        GYQ = GYR;
                    } else {
                        let GXJ = GWZ - B;
                        let GXL = (BGX * GWG) * (((GWK * GXJ) * (((UA * DKI) * GXJ) + (B + DKI))) + GXK);
                        GYQ = GXL;
                    }
                    GYP = GYQ;
                } else {
                    GYP = A;
                }
                let GXM = if GWH > A { 1.0 } else { 0.0 };
                let GYS;
                if GXM != 0.0 {
                    let GXN = BRO / BHC;
                    let GXO = if GXN < DIA { 1.0 } else { 0.0 };
                    let GYT;
                    if GXO != 0.0 {
                        let GXP = B - GXN;
                        let GXQ = if DKO != B { 1.0 } else { 0.0 };
                        let GYU;
                        if GXQ != 0.0 {
                            let GXR = if DKO == NZ { 1.0 } else { 0.0 };
                            let GXU = if GXR != 0.0 {
                                let GXS = B / (GXP.sqrt());
                                GXS
                            } else {
                                let GXT = rspice_limited_exp((GWO * (GXP.ln())));
                                GXT
                            };
                            let GXV = ((BHC * GWH) * (B - (GXP * GXU))) / (B - DKO);
                            GYU = GXV;
                        } else {
                            let GXW = (BHC * GWH) * (-(GXP.ln()));
                            GYU = GXW;
                        }
                        GYT = GYU;
                    } else {
                        let GXX = GXN - B;
                        let GXZ = (BHC * GWH) * (((GWP * GXX) * (((UA * DKO) * GXX) + (B + DKO))) + GXY);
                        GYT = GXZ;
                    }
                    GYS = GYT;
                } else {
                    GYS = A;
                }
                let GYA = if GWI > A { 1.0 } else { 0.0 };
                let GYV;
                if GYA != 0.0 {
                    let GYB = BRO / BHH;
                    let GYC = if GYB < DIA { 1.0 } else { 0.0 };
                    let GYW;
                    if GYC != 0.0 {
                        let GYD = B - GYB;
                        let GYE = if DKU != B { 1.0 } else { 0.0 };
                        let GYX;
                        if GYE != 0.0 {
                            let GYF = if DKU == NZ { 1.0 } else { 0.0 };
                            let GYI = if GYF != 0.0 {
                                let GYG = B / (GYD.sqrt());
                                GYG
                            } else {
                                let GYH = rspice_limited_exp((GWT * (GYD.ln())));
                                GYH
                            };
                            let GYJ = ((BHH * GWI) * (B - (GYD * GYI))) / (B - DKU);
                            GYX = GYJ;
                        } else {
                            let GYK = (BHH * GWI) * (-(GYD.ln()));
                            GYX = GYK;
                        }
                        GYW = GYX;
                    } else {
                        let GYL = GYB - B;
                        let GYN = (BHH * GWI) * (((GWU * GYL) * (((UA * DKU) * GYL) + (B + DKU))) + GYM);
                        GYW = GYN;
                    }
                    GYV = GYW;
                } else {
                    GYV = A;
                }
                let GYY = ((GYP + GYS) + GYV) + ((DJP * GYO) * AA);
                let GYZ = if parameters[28] != A { 1.0 } else { 0.0 };
                if GYZ != 0.0 {
                } else {
                }
                let GZA = (UI * BBM) * C;
                let GZB = GKO / GNM;
                let GZC = if EVA <= A { 1.0 } else { 0.0 };
                let HCQ;
                if GZC != 0.0 {
                    HCQ = A;
                } else {
                    let GZD = BCY * ((if (((GKT / BCY) + EVA) / GZB) >= BAF { (((GKT / BCY) + EVA) / GZB) } else { BAF }).ln());
                    let GZE = if GZD < A { 1.0 } else { 0.0 };
                    let HCR = if GZE != 0.0 {
                        A
                    } else {
                        GZD
                    };
                    HCQ = HCR;
                }
                let GZF = BBM / C;
                let GZG = GZF * ((N + BUK) + DT);
                let GZH = AY * GJV;
                let GZI = (GZH * N) * BBM;
                let GZJ = (((GZI * GJS) * GNJ) * GNI) / C;
                let GZK = ((4.112737976006692e-57f64 * BBM) * (GNO.abs())) * GNM;
                let GZL = C * BBM;
                let GZM = (GZL * GNO) * GNO;
                let GZN = (EVQ + (EVR * GZJ)) + ((EVS * GZJ) * GZJ);
                let GZO = GZJ + GZG;
                let GZP = GZO * GZO;
                let GZQ = (EVQ * C) * BBM;
                let GZR = if parameters[1319] == B { 1.0 } else { 0.0 };
                let HVP;
                let IAG;
                let IAI;
                let IAK;
                let IAM;
                let IAO;
                let IAQ;
                if GZR != 0.0 {
                    let GZT = if AZ > GZS { 1.0 } else { 0.0 };
                    let GZV;
                    let HBU;
                    if GZT != 0.0 {
                        let GZU = AZ - GZS;
                        GZV = GZU;
                        HBU = GZS;
                    } else {
                        GZV = AZ;
                        HBU = AZ;
                    }
                    let GZW = if EVX >= (GZV / AY) { 1.0 } else { 0.0 };
                    let HCN = if GZW != 0.0 {
                        A
                    } else {
                        EVX
                    };
                    let GZX = (BRC - BVI) / BBM;
                    let GZZ = ((((3.204352924e-19f64 * J) * GZY) / BBM).sqrt()) / N;
                    let HAA = (GZY / BBT).ln();
                    let HAB = (NZ * GZX) - (UH * (B + (GZZ / GCC)));
                    let HAC = HAB + (((HAB * HAB) + (UJ * GZX)).sqrt());
                    let HAD = if GZX < A { 1.0 } else { 0.0 };
                    let HAK = if HAD != 0.0 {
                        let HAE = (GZX - HAC) / GZZ;
                        let HAF = -((if ((B - HAC) + (HAE * HAE)) >= BAF { ((B - HAC) + (HAE * HAE)) } else { BAF }).ln());
                        HAF
                    } else {
                        let HAG = rspice_limited_exp((-HAC));
                        let HAH = NZ * GZZ;
                        let HAI = ((((GZX - B) + HAG) + (HAH * HAH)).sqrt()) - HAH;
                        let HAJ = ((HAI * HAI) + B) - HAG;
                        HAJ
                    };
                    let HAL = HAK + B;
                    let HAM = HAK - B;
                    let HAN = HAM * HAM;
                    let HAO = (NZ * (HAL + ((HAN + 1e0f64).sqrt()))).sqrt();
                    let HAP = AY * HAO;
                    let HAQ = (B + (GZZ / HAP)) / GZZ;
                    let HAR = (HAK - (AY * HAA)) - BVJ;
                    let HAS = HAR - ((if ((UI * HAQ) * HAO) >= BAF { ((UI * HAQ) * HAO) } else { BAF }).ln());
                    let HAT = NZ * ((HAS - GCW) - (((HAS * (HAS + GCX)) + GCY).sqrt()));
                    let HAU = if HAT <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HBZ;
                    if HAU != 0.0 {
                        let HAW = if HAT < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HBD;
                        if HAW != 0.0 {
                            HBD = HAX;
                        } else {
                            let HAY = if HAT > -9e1f64 { 1.0 } else { 0.0 };
                            let HBE = if HAY != 0.0 {
                                let HAZ = rspice_limited_exp(HAT);
                                HAZ
                            } else {
                                let HBA = (HAT - HAV) / BQG;
                                let HBB = HBA * HBA;
                                let HBC = rspice_limited_exp((HAV + (BQG * ((7.8125e-2f64 + (NZ * HBA)) + (HBB * (9.375e-1f64 - (HBB * (BYV - HBB))))))));
                                HBC
                            };
                            HBD = HBE;
                        }
                        let HBF = HBD * (((B + HAR) - HAT) - ((if ((AY * HAQ) * (((HBD * AY) * HAQ) + HAP)) >= BAF { ((AY * HAQ) * (((HBD * AY) * HAQ) + HAP)) } else { BAF }).ln()));
                        HBZ = HBF;
                    } else {
                        let HBG = rspice_limited_exp(HAT);
                        let HBH = AY * HBG;
                        let HBI = HBH * HAQ;
                        let HBJ = HAQ + (B / HAO);
                        let HBK = HBG - (((HBH + ((if (HBI * (HBI + HAP)) >= BAF { (HBI * (HBI + HAP)) } else { BAF }).ln())) - HAR) / ((AY + (B / HBG)) + (HBJ / ((HAQ * HBG) + HAO))));
                        let HBL = AY * HBK;
                        let HBM = HBL * HAQ;
                        let HBN = (HBL + ((if (HBM * (HBM + HAP)) >= BAF { (HBM * (HBM + HAP)) } else { BAF }).ln())) - HAR;
                        let HBO = B / HBK;
                        let HBP = (HAQ * HBK) + HAO;
                        let HBQ = HBJ / HBP;
                        let HBR = (AY + HBO) + HBQ;
                        let HBS = HBK - ((HBN / HBR) * (B + ((HBN * (((-(HBO * HBO)) - (B / (((HAO * HAO) * HAO) * HBP))) - (HBQ * HBQ))) / ((AY * HBR) * HBR))));
                        HBZ = HBS;
                    }
                    let HBT = (GNM * N) * BG;
                    let HBV = ((AY * (B + (GZZ / (AY * ((NZ * (HAL + ((HAN + 1e0f64).sqrt()))).sqrt()))))) * HBT) * BBM;
                    let HBW = AZ - HBU;
                    let HBX = GZH * HBT;
                    let HBY = (GNO * HBW) / ((HBX * BUT) * BUT);
                    let HCA = B + (UI * (((HBZ * HBZ) + HBZ) - ((GNO * HBU) / (HBV * BBM))));
                    let HCB = if HCA < B { 1.0 } else { 0.0 };
                    let HCE = if HCB != 0.0 {
                        A
                    } else {
                        let HCC = -5e-1f64 + (NZ * (HCA.sqrt()));
                        HCC
                    };
                    let HCD = -5e-1f64 + (NZ * ((B + (UI * (((GJS * GJS) + GJS) + HBY))).sqrt()));
                    let HCF = (HBV * HCE) * HBW;
                    let HCG = ((((AY * HBT) * BBM) * (HCD - GJS)) * HBU) + (((HBX * BBM) * GJS) * HBU);
                    let HCH = HCF + HCG;
                    let HCI = (B / HCH) / HCH;
                    let HCJ = (HCF * HCF) * HCI;
                    let HCK = (HCG * HCG) * HCI;
                    let HCL = if AZ != HBU { 1.0 } else { 0.0 };
                    let HCZ;
                    if HCL != 0.0 {
                        let HCM = (GZI * HCD) / C;
                        let HCO = (AZ - (AY * HCN)) - HBU;
                        let HCP = HCO * HCO;
                        let HCS = ((GZK / ((EWD * N) * HCP)) * (((EVQ * ((if ((HCM + GZG) / GZO) >= BAF { ((HCM + GZG) / GZO) } else { BAF }).ln())) + (EVR * (HCM - GZJ))) + ((NZ * EVS) * ((HCM * HCM) - (GZJ * GZJ))))) + ((((GZM / (((EWD * HCP) * BG) * AA)) * HCQ) * GZN) / GZP);
                        let HCT = ((GZQ / (((((BG * AA) * HCO) * EWD) * GZG) * GZG)) * GNO) * GNO;
                        let HCU = HCT + HCS;
                        let HCV = if HCU > A { 1.0 } else { 0.0 };
                        let HDA = if HCV != 0.0 {
                            let HCW = (HCS * HCT) / HCU;
                            HCW
                        } else {
                            A
                        };
                        HCZ = HDA;
                    } else {
                        HCZ = A;
                    }
                    let HCX = ((((parameters[1321] * C) * BBM) / (((((BG * AA) * HBU) * EWD) * GZG) * GZG)) * GNO) * GNO;
                    let HCY = if HCX > A { 1.0 } else { 0.0 };
                    let HDB = if HCY != 0.0 {
                        HCX
                    } else {
                        A
                    };
                    let HDC = DGQ * ((HCZ * HCJ) + (HDB * HCK));
                    HVP = HCN;
                    IAG = B;
                    IAI = HDC;
                    IAK = EWU;
                    IAM = A;
                    IAO = A;
                    IAQ = A;
                } else {
                    let HDD = if EVX >= (AZ / AY) { 1.0 } else { 0.0 };
                    let HDF = if HDD != 0.0 {
                        A
                    } else {
                        EVX
                    };
                    let HDE = if (if (if EVQ > A { 1.0 } else { 0.0 }) != 0.0 || (if EVR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if EVS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let HDO;
                    if HDE != 0.0 {
                        let HDG = AZ - (AY * HDF);
                        let HDH = HDG * HDG;
                        let HDI = (((GZI * GDZ) * GNJ) * GNI) / C;
                        let HDJ = ((GZK / ((EWD * N) * HDH)) * (((EVQ * ((if ((HDI + GZG) / GZO) >= BAF { ((HDI + GZG) / GZO) } else { BAF }).ln())) + (EVR * (HDI - GZJ))) + ((NZ * EVS) * ((HDI * HDI) - (GZJ * GZJ))))) + ((((GZM / (((EWD * HDH) * BG) * AA)) * HCQ) * GZN) / GZP);
                        let HDK = ((GZQ / (((((BG * AA) * HDG) * EWD) * GZG) * GZG)) * GNO) * GNO;
                        let HDL = HDK + HDJ;
                        let HDM = if HDL > A { 1.0 } else { 0.0 };
                        let HDP = if HDM != 0.0 {
                            let HDN = ((HDJ * HDK) / HDL) / (B + (EWO * (GJW.powf(EWP))));
                            HDN
                        } else {
                            A
                        };
                        HDO = HDP;
                    } else {
                        HDO = A;
                    }
                    let HDQ = DGQ * HDO;
                    HVP = HDF;
                    IAG = A;
                    IAI = A;
                    IAK = A;
                    IAM = B;
                    IAO = HDQ;
                    IAQ = EWU;
                }
                let HDR = (GKK / GZB) / AZ;
                let HDS = HDR * HDR;
                let HDT = EWX * (B + ((EWY * AZ) * HDS));
                let HDU = EXA * (B + ((EXB * AZ) * HDS));
                let HDV = EXD * (B + ((EXE * AZ) * HDS));
                let HDW = EXG * (B + ((EXH * AZ) * HDS));
                let HDX = rspice_limited_exp(((-AZ) / EXJ));
                let HDY = ((((UH * HDT) * HDT) - B) * HDX) + B;
                let HDZ = HDV * HDV;
                let HEA = HDU * HDU;
                let HEB = if EXQ == A { 1.0 } else { 0.0 };
                let IAS;
                let IAU;
                let IAW;
                let IAZ;
                let IBC;
                let IBF;
                let IBI;
                let IBL;
                if HEB != 0.0 {
                    let HEC = ((((-AA) * BG) * AZ) * N) * BBM;
                    let HED = GNM * (((HEC * GKG) + (HEC * GKH)).abs());
                    let HEG = GZA * ((HED / ((HED * HEE) + (AZ * AZ))) * EXX);
                    IAS = B;
                    IAU = HEG;
                    IAW = A;
                    IAZ = A;
                    IBC = A;
                    IBF = A;
                    IBI = A;
                    IBL = A;
                } else {
                    let HEH = if EXQ == B { 1.0 } else { 0.0 };
                    let IAX;
                    let IBA;
                    let IBD;
                    let IBG;
                    let IBJ;
                    let IBM;
                    if HEH != 0.0 {
                        let HEI = (((GNM * GMO) * GMD) * N) * (GZH * BUT);
                        let HEJ = NZ * GKB;
                        let HEK = HEJ + NZ;
                        let HEL = HEK * HEK;
                        let HEM = HEL * HEK;
                        let HEN = AZ * GMO;
                        let HEO = HEN / AZ;
                        let HEP = (((B + ((HDZ * (GIM / GIJ)) / (EYO + GKK))) - B) * HDX) + B;
                        let HEQ = BZL * HEK;
                        let HER = ((((((HEN * HEO) * HEO) * (((HEJ / HEL) - ((((UJ * HEJ) + NZ) * GJX) / ((EYR * HEL) * HEL))) + ((GJX * GJX) / ((EYS * HEL) * HEM)))) * EYT) / UI) * HEA) / (((AA * BG) * BZL) * HEI);
                        let HES = ((HEO * ((GJW / HEQ) - ((GJX * GJW) / (EYS * HEM)))) * HDW) / EYV;
                        let HET = (GZA * ((((HEI * AA) * BG) / HEN) * ((HEJ * (NZ * (HEP + (((HEP * HEP) + 2.5000000000000005e-3f64).sqrt())))) + ((GJX * HDY) / HEQ)))).sqrt();
                        let HEU = if HER > A { 1.0 } else { 0.0 };
                        let HEY;
                        let HFA;
                        if HEU != 0.0 {
                            let HEV = (GZA / HER).sqrt();
                            let HEW = if HET > A { 1.0 } else { 0.0 };
                            let HEZ = if HEW != 0.0 {
                                let HEX = (HES * HEV) / HET;
                                HEX
                            } else {
                                A
                            };
                            HEY = HEZ;
                            HFA = HEV;
                        } else {
                            HEY = A;
                            HFA = A;
                        }
                        let HFB = B - HEY;
                        let HFC = (HFA * HFA) * HFB;
                        let HFD = (HET * HET) * HFB;
                        IAX = B;
                        IBA = HEY;
                        IBD = B;
                        IBG = HFC;
                        IBJ = B;
                        IBM = HFD;
                    } else {
                        IAX = A;
                        IBA = A;
                        IBD = A;
                        IBG = A;
                        IBJ = A;
                        IBM = A;
                    }
                    IAS = A;
                    IAU = A;
                    IAW = IAX;
                    IAZ = IBA;
                    IBC = IBD;
                    IBF = IBG;
                    IBI = IBJ;
                    IBL = IBM;
                }
                let IBO;
                let IBQ;
                let IBS;
                let IBU;
                if RW != 0.0 {
                    let HFE = 3.204352924e-19f64 * ((GTH + GTB).abs());
                    let HFF = 3.204352924e-19f64 * ((GTK + GTD).abs());
                    IBO = B;
                    IBQ = HFE;
                    IBS = B;
                    IBU = HFF;
                } else {
                    IBO = A;
                    IBQ = A;
                    IBS = A;
                    IBU = A;
                }
                let IBW;
                let IBY;
                if GQK != 0.0 {
                    let HFG = 3.204352924e-19f64 * (GTF.abs());
                    IBW = B;
                    IBY = HFG;
                } else {
                    IBW = A;
                    IBY = A;
                }
                let HFH = if parameters[31] == B { 1.0 } else { 0.0 };
                let HKZ;
                let HLA;
                let HLE;
                let HLF;
                let HLI;
                let HLJ;
                let HLO;
                let HLV;
                let HLW;
                let HLY;
                if HFH != 0.0 {
                    let HFI = (BRC * BBN) - ((PS + BVH) * BBN);
                    let HFJ = (if (RP / BBT) >= BAF { (RP / BBT) } else { BAF }).ln();
                    let HFK = ((((3.204352924e-19f64 * J) * RP) * BBN).sqrt()) / N;
                    let HFL = B / HFK;
                    let HFM = ((3.204352924e-19f64 * J) * DS) / ((N * N) * BBM);
                    let HFO = if BCA != 0.0 {
                        let HFN = B / HFM;
                        HFN
                    } else {
                        A
                    };
                    let HFQ = if BCA != 0.0 {
                        let HFP = RP / DS;
                        HFP
                    } else {
                        A
                    };
                    let HFR = B + HFQ;
                    let HFS = HFI / HFR;
                    let HFT = HFK / HFR;
                    let HFU = UH * (B + (HFT / GCC));
                    let HFV = (NZ * HFS) - HFU;
                    let HFW = HFV + (((HFV * HFV) + (UJ * HFS)).sqrt());
                    let HFX = if HFS < A { 1.0 } else { 0.0 };
                    let HGE = if HFX != 0.0 {
                        let HFY = (HFS - HFW) / HFT;
                        let HFZ = -((if ((B - HFW) + (HFY * HFY)) >= BAF { ((B - HFW) + (HFY * HFY)) } else { BAF }).ln());
                        HFZ
                    } else {
                        let HGA = rspice_limited_exp((-HFW));
                        let HGB = NZ * HFT;
                        let HGC = ((((HFS - B) + HGA) + (HGB * HGB)).sqrt()) - HGB;
                        let HGD = ((HGC * HGC) + B) - HGA;
                        HGD
                    };
                    let HGF = HGE + B;
                    let HGG = HGE - B;
                    let HGH = HGG * HGG;
                    let HGI = (NZ * (HGF + ((HGH + 1e0f64).sqrt()))).sqrt();
                    let HGJ = AY * HGI;
                    let HGK = (B + (HFK / HGJ)) / HFK;
                    let HGL = HGE - (AY * HFJ);
                    let HGM = HGL - BVQ;
                    let HGN = HGM - ((if ((UI * HGK) * HGI) >= BAF { ((UI * HGK) * HGI) } else { BAF }).ln());
                    let HGO = NZ * ((HGN - GCW) - (((HGN * (HGN + GCX)) + GCY).sqrt()));
                    let HGP = if HGO <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HHO;
                    if HGP != 0.0 {
                        let HGR = if HGO < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HGY;
                        if HGR != 0.0 {
                            HGY = HGS;
                        } else {
                            let HGT = if HGO > -9e1f64 { 1.0 } else { 0.0 };
                            let HGZ = if HGT != 0.0 {
                                let HGU = rspice_limited_exp(HGO);
                                HGU
                            } else {
                                let HGV = (HGO - HGQ) / BQG;
                                let HGW = HGV * HGV;
                                let HGX = rspice_limited_exp((HGQ + (BQG * ((7.8125e-2f64 + (NZ * HGV)) + (HGW * (9.375e-1f64 - (HGW * (BYV - HGW))))))));
                                HGX
                            };
                            HGY = HGZ;
                        }
                        let HHA = HGY * (((B + HGM) - HGO) - ((if ((AY * HGK) * (((HGY * AY) * HGK) + HGJ)) >= BAF { ((AY * HGK) * (((HGY * AY) * HGK) + HGJ)) } else { BAF }).ln()));
                        HHO = HHA;
                    } else {
                        let HHB = rspice_limited_exp(HGO);
                        let HHC = AY * HHB;
                        let HHD = HHC * HGK;
                        let HHE = HGK + (B / HGI);
                        let HHF = HHB - (((HHC + ((if (HHD * (HHD + HGJ)) >= BAF { (HHD * (HHD + HGJ)) } else { BAF }).ln())) - HGM) / ((AY + (B / HHB)) + (HHE / ((HGK * HHB) + HGI))));
                        let HHG = AY * HHF;
                        let HHH = HHG * HGK;
                        let HHI = (HHG + ((if (HHH * (HHH + HGJ)) >= BAF { (HHH * (HHH + HGJ)) } else { BAF }).ln())) - HGM;
                        let HHJ = B / HHF;
                        let HHK = (HGK * HHF) + HGI;
                        let HHL = HHE / HHK;
                        let HHM = (AY + HHJ) + HHL;
                        let HHN = HHF - ((HHI / HHM) * (B + ((HHI * (((-(HHJ * HHJ)) - (B / (((HGI * HGI) * HGI) * HHK))) - (HHL * HHL))) / ((AY * HHM) * HHM))));
                        HHO = HHN;
                    }
                    let HHP = AY * HHO;
                    let HHQ = HGE - HHP;
                    let HHR = HHQ - B;
                    let HHS = B + (HFK / (((NZ * (HGF + ((HGH + 1e0f64).sqrt()))).sqrt()) + ((NZ * ((HHQ + B) + (((HHR * HHR) + 1e0f64).sqrt()))).sqrt())));
                    let HHT = HFI - HGE;
                    let HHU = HHS - B;
                    let HHV = BBM * (HHT - (HHP * HHU));
                    let HHW = B + (GEK * ((GEE * ((NZ * (HHV + (((HHV * HHV) + 2.5000000000000005e-3f64).sqrt()))) + (BDM * (((AY * HHS) * BBM) * HHO)))).powf(BEF)));
                    let HHX = HHW - B;
                    let HHZ = HHY * BN;
                    let HIA = ((CKJ / (NZ * ((HHW + B) + (((HHX * HHX) + 5.625e-7f64).sqrt())))) * BBM) / HHZ;
                    let HIB = AY * ((HIA * ((HHO * HHO) + HHO)) / (B + (HIA * (B + HHO))));
                    let HIC = (HIB * HHS) * HFL;
                    let HID = ((HGL - (HIB + ((if (HIC * (HIC + (HFK / HHU))) >= BAF { (HIC * (HIC + (HFK / HHU))) } else { BAF }).ln()))) * BBM) - BSD;
                    let HIE = NZ * (HID + (((HID * HID) + 2.5e-7f64).sqrt()));
                    let HIH = if (if HIF == A { 1.0 } else { 0.0 }) != 0.0 && (if HIG == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let HIN = if HIH != 0.0 {
                        HII
                    } else {
                        let HIJ = AZ / (AZ + ((FA * BUJ).sqrt()));
                        let HIK = B + (((HIF * HIJ) - (((HIG * HIJ) * HHO) * BUT)) / (B + (parameters[1355] * BSO)));
                        let HIL = HIK - BQB;
                        let HIM = NZ * ((HIK + BQB) + (((HIL * HIL) + 6.25e-8f64).sqrt()));
                        HIM
                    };
                    let HIO = HIE / HIN;
                    let HIP = BSE * ((B + (((BSE / HIO) + BX).powf(GIK))).powf(GIL));
                    let HIQ = (NZ * (HGF + ((HGH + 1e0f64).sqrt()))).sqrt();
                    let HIR = AY * HIQ;
                    let HIS = (B + (HFK / HIR)) / HFK;
                    let HIT = HGL - ((HIP + BSD) * BBN);
                    let HIU = HIT - ((if ((UI * HIS) * HIQ) >= BAF { ((UI * HIS) * HIQ) } else { BAF }).ln());
                    let HIV = NZ * ((HIU - GCW) - (((HIU * (HIU + GCX)) + GCY).sqrt()));
                    let HIW = if HIV <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HJV;
                    if HIW != 0.0 {
                        let HIY = if HIV < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HJF;
                        if HIY != 0.0 {
                            HJF = HIZ;
                        } else {
                            let HJA = if HIV > -9e1f64 { 1.0 } else { 0.0 };
                            let HJG = if HJA != 0.0 {
                                let HJB = rspice_limited_exp(HIV);
                                HJB
                            } else {
                                let HJC = (HIV - HIX) / BQG;
                                let HJD = HJC * HJC;
                                let HJE = rspice_limited_exp((HIX + (BQG * ((7.8125e-2f64 + (NZ * HJC)) + (HJD * (9.375e-1f64 - (HJD * (BYV - HJD))))))));
                                HJE
                            };
                            HJF = HJG;
                        }
                        let HJH = HJF * (((B + HIT) - HIV) - ((if ((AY * HIS) * (((HJF * AY) * HIS) + HIR)) >= BAF { ((AY * HIS) * (((HJF * AY) * HIS) + HIR)) } else { BAF }).ln()));
                        HJV = HJH;
                    } else {
                        let HJI = rspice_limited_exp(HIV);
                        let HJJ = AY * HJI;
                        let HJK = HJJ * HIS;
                        let HJL = HIS + (B / HIQ);
                        let HJM = HJI - (((HJJ + ((if (HJK * (HJK + HIR)) >= BAF { (HJK * (HJK + HIR)) } else { BAF }).ln())) - HIT) / ((AY + (B / HJI)) + (HJL / ((HIS * HJI) + HIQ))));
                        let HJN = AY * HJM;
                        let HJO = HJN * HIS;
                        let HJP = (HJN + ((if (HJO * (HJO + HIR)) >= BAF { (HJO * (HJO + HIR)) } else { BAF }).ln())) - HIT;
                        let HJQ = B / HJM;
                        let HJR = (HIS * HJM) + HIQ;
                        let HJS = HJL / HJR;
                        let HJT = (AY + HJQ) + HJS;
                        let HJU = HJM - ((HJP / HJT) * (B + ((HJP * (((-(HJQ * HJQ)) - (B / (((HIQ * HIQ) * HIQ) * HJR))) - (HJS * HJS))) / ((AY * HJT) * HJT))));
                        HJV = HJU;
                    }
                    let HJW = ((HGE - HHO) - HJV) - B;
                    let HJX = HJW - B;
                    let HJY = (NZ * ((HJW + B) + (((HJX * HJX) + 1e0f64).sqrt()))).sqrt();
                    let HJZ = HFR + (HFK / (HIQ + HJY));
                    let HKA = NZ + ((HFQ * HJY) * HFL);
                    let HKB = HHO + HJV;
                    let HKC = HJZ / (HKA + (((HKA * HKA) + ((HJZ * HKB) * HFO)).sqrt()));
                    let HKD = HKC - B;
                    let HKE = BBM * (HHT - (HHP * HKD));
                    let HKF = BBM * (HHT - ((AY * HJV) * HKD));
                    let HKG = GEE * ((NZ * ((NZ * (HKE + (((HKE * HKE) + 2.5000000000000005e-3f64).sqrt()))) + (NZ * (HKF + (((HKF * HKF) + 2.5000000000000005e-3f64).sqrt()))))) + (BDM * ((HKC * BBM) * HKB)));
                    let HKH = (HFI + (parameters[139] * BBN)) / HFR;
                    let HKI = (NZ * HKH) - HFU;
                    let HKJ = HKI + (((HKI * HKI) + (UJ * HKH)).sqrt());
                    let HKK = if HKH < A { 1.0 } else { 0.0 };
                    let HLG = if HKK != 0.0 {
                        let HKL = (HKH - HKJ) / HFT;
                        let HKM = -((if ((B - HKJ) + (HKL * HKL)) >= BAF { ((B - HKJ) + (HKL * HKL)) } else { BAF }).ln());
                        HKM
                    } else {
                        let HKN = rspice_limited_exp((-HKJ));
                        let HKO = NZ * HFT;
                        let HKP = ((((HKH - B) + HKN) + (HKO * HKO)).sqrt()) - HKO;
                        let HKQ = ((HKP * HKP) + B) - HKN;
                        HKQ
                    };
                    let HKR = B + (GEK * (HKG.powf(BEF)));
                    let HKS = HKR - B;
                    let HKT = CKJ / (NZ * ((HKR + B) + (((HKS * HKS) + 5.625e-7f64).sqrt())));
                    let HKU = (((AY * HKT) * BBM) / HHZ) * (HHO - HJV);
                    let HKV = NZ * (B + ((B + ((AY * HKU) * HKU)).sqrt()));
                    let HKW = HIO + (((AY * HHY) / HKT) * BN);
                    let HKX = BSE - HIP;
                    HKZ = HKX;
                    HLA = HKW;
                    HLE = HFI;
                    HLF = HLG;
                    HLI = HHO;
                    HLJ = HJV;
                    HLO = HFO;
                    HLV = HIN;
                    HLW = HKV;
                    HLY = HKC;
                } else {
                    HKZ = GKT;
                    HLA = GLB;
                    HLE = BVM;
                    HLF = GCM;
                    HLI = GDZ;
                    HLJ = GJS;
                    HLO = A;
                    HLV = B;
                    HLW = GMO;
                    HLY = GJV;
                }
                let HKY = if PX != A { 1.0 } else { 0.0 };
                let HLC = if HKY != 0.0 {
                    let HLB = B + (PX * ((if (B + ((HKZ / PX) / HLA)) >= BAF { (B + ((HKZ / PX) / HLA)) } else { BAF }).ln()));
                    HLB
                } else {
                    B
                };
                let HLD = B / HLC;
                let HLH = HLE - HLF;
                let HLK = HLI - HLJ;
                let HLL = HLK * HLK;
                let HLM = HLH + (AY * HLI);
                let HLN = HLH + (AY * HLJ);
                let HLP = (OQ + ((NZ * (HLM + (((HLM * HLM) + 6.25e-2f64).sqrt()))) * HLO)).sqrt();
                let HLQ = (OQ + ((NZ * (HLN + (((HLN * HLN) + 6.25e-2f64).sqrt()))) * HLO)).sqrt();
                let HLR = AY * HLQ;
                let HLS = B + HLR;
                let HLT = HLP + HLQ;
                let HLU = HLT * HLT;
                let HLX = ((HLV * HLW) * HLD) / ((B + HLI) + HLJ);
                let HLZ = L / DNU;
                let HMA = (((-(((AA * BQ) * BN) + DNV)) * HLZ) * BBM) * ((HLD * (((HLM / (B + (AY * HLP))) + (HLN / HLS)) + (((BDI * (HLL / (HLU * HLT))) * (((GKE * (HLU + (HLP * HLQ))) * HLX) + (AY * HLO))) - (HLY * ((HLI + HLJ) + ((BDI * HLL) * HLX)))))) + ((HLC - B) * ((HLH - ((AY * (HLY - B)) * HLJ)) + ((HLN * (HLR - B)) / HLS))));
                let HMB = if BCG == B { 1.0 } else { 0.0 };
                let HQO;
                if HMB != 0.0 {
                    let HMC = BSP * BBN;
                    let HMD = (BRE * BBN) - ((DPG + BVH) * BBN);
                    let HME = (if (RR / BBT) >= BAF { (RR / BBT) } else { BAF }).ln();
                    let HMF = ((((3.204352924e-19f64 * J) * RR) * BBN).sqrt()) / N;
                    let HMG = B / HMF;
                    let HMH = (NZ * HMD) - (UH * (B + (HMF / GCC)));
                    let HMI = HMH + (((HMH * HMH) + (UJ * HMD)).sqrt());
                    let HMJ = if HMD < A { 1.0 } else { 0.0 };
                    let HMQ = if HMJ != 0.0 {
                        let HMK = (HMD - HMI) / HMF;
                        let HML = -((if ((B - HMI) + (HMK * HMK)) >= BAF { ((B - HMI) + (HMK * HMK)) } else { BAF }).ln());
                        HML
                    } else {
                        let HMM = rspice_limited_exp((-HMI));
                        let HMN = NZ * HMF;
                        let HMO = ((((HMD - B) + HMM) + (HMN * HMN)).sqrt()) - HMN;
                        let HMP = ((HMO * HMO) + B) - HMM;
                        HMP
                    };
                    let HMR = HMQ + B;
                    let HMS = HMQ - B;
                    let HMT = HMS * HMS;
                    let HMU = (NZ * (HMR + ((HMT + 1e0f64).sqrt()))).sqrt();
                    let HMV = AY * HMU;
                    let HMW = (B + (HMF / HMV)) / HMF;
                    let HMX = HMQ - (AY * HME);
                    let HMY = HMX - HMC;
                    let HMZ = HMY - ((if ((UI * HMW) * HMU) >= BAF { ((UI * HMW) * HMU) } else { BAF }).ln());
                    let HNA = NZ * ((HMZ - GCW) - (((HMZ * (HMZ + GCX)) + GCY).sqrt()));
                    let HNB = if HNA <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HOA;
                    if HNB != 0.0 {
                        let HND = if HNA < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HNK;
                        if HND != 0.0 {
                            HNK = HNE;
                        } else {
                            let HNF = if HNA > -9e1f64 { 1.0 } else { 0.0 };
                            let HNL = if HNF != 0.0 {
                                let HNG = rspice_limited_exp(HNA);
                                HNG
                            } else {
                                let HNH = (HNA - HNC) / BQG;
                                let HNI = HNH * HNH;
                                let HNJ = rspice_limited_exp((HNC + (BQG * ((7.8125e-2f64 + (NZ * HNH)) + (HNI * (9.375e-1f64 - (HNI * (BYV - HNI))))))));
                                HNJ
                            };
                            HNK = HNL;
                        }
                        let HNM = HNK * (((B + HMY) - HNA) - ((if ((AY * HMW) * (((HNK * AY) * HMW) + HMV)) >= BAF { ((AY * HMW) * (((HNK * AY) * HMW) + HMV)) } else { BAF }).ln()));
                        HOA = HNM;
                    } else {
                        let HNN = rspice_limited_exp(HNA);
                        let HNO = AY * HNN;
                        let HNP = HNO * HMW;
                        let HNQ = HMW + (B / HMU);
                        let HNR = HNN - (((HNO + ((if (HNP * (HNP + HMV)) >= BAF { (HNP * (HNP + HMV)) } else { BAF }).ln())) - HMY) / ((AY + (B / HNN)) + (HNQ / ((HMW * HNN) + HMU))));
                        let HNS = AY * HNR;
                        let HNT = HNS * HMW;
                        let HNU = (HNS + ((if (HNT * (HNT + HMV)) >= BAF { (HNT * (HNT + HMV)) } else { BAF }).ln())) - HMY;
                        let HNV = B / HNR;
                        let HNW = (HMW * HNR) + HMU;
                        let HNX = HNQ / HNW;
                        let HNY = (AY + HNV) + HNX;
                        let HNZ = HNR - ((HNU / HNY) * (B + ((HNU * (((-(HNV * HNV)) - (B / (((HMU * HMU) * HMU) * HNW))) - (HNX * HNX))) / ((AY * HNY) * HNY))));
                        HOA = HNZ;
                    }
                    let HOB = AY * HOA;
                    let HOC = HMQ - HOB;
                    let HOD = HOC - B;
                    let HOE = B + (HMF / (((NZ * (HMR + ((HMT + 1e0f64).sqrt()))).sqrt()) + ((NZ * ((HOC + B) + (((HOD * HOD) + 1e0f64).sqrt()))).sqrt())));
                    let HOF = HMD - HMQ;
                    let HOG = HOE - B;
                    let HOH = BBM * (HOF - (HOB * HOG));
                    let HOI = B + ((CJP + (CJQ * BSQ)) * ((GEE * ((NZ * (HOH + (((HOH * HOH) + 2.5000000000000005e-3f64).sqrt()))) + (BDM * (((AY * HOE) * BBM) * HOA)))).powf(BEF)));
                    let HOJ = HOI - B;
                    let HOK = ((CKJ / (NZ * ((HOI + B) + (((HOJ * HOJ) + 5.625e-7f64).sqrt())))) * BBM) / (HHY * BN);
                    let HOL = B + HOA;
                    let HOM = AY * ((HOK * ((HOA * HOA) + HOA)) / (B + (HOK * HOL)));
                    let HON = (HOM * HOE) * HMG;
                    let HOO = ((HMX - (HOM + ((if (HON * (HON + (HMF / HOG))) >= BAF { (HON * (HON + (HMF / HOG))) } else { BAF }).ln()))) * BBM) - BSP;
                    let HOP = (NZ * (HMR + ((HMT + 1e0f64).sqrt()))).sqrt();
                    let HOQ = AY * HOP;
                    let HOR = (B + (HMF / HOQ)) / HMF;
                    let HOS = HMX - (((BSE * ((B + (((BSE / (NZ * (HOO + (((HOO * HOO) + 2.5e-7f64).sqrt())))) + BX).powf(GIK))).powf(GIL))) + BSP) * BBN);
                    let HOT = HOS - ((if ((UI * HOR) * HOP) >= BAF { ((UI * HOR) * HOP) } else { BAF }).ln());
                    let HOU = NZ * ((HOT - GCW) - (((HOT * (HOT + GCX)) + GCY).sqrt()));
                    let HOV = if HOU <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HPU;
                    if HOV != 0.0 {
                        let HOX = if HOU < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HPE;
                        if HOX != 0.0 {
                            HPE = HOY;
                        } else {
                            let HOZ = if HOU > -9e1f64 { 1.0 } else { 0.0 };
                            let HPF = if HOZ != 0.0 {
                                let HPA = rspice_limited_exp(HOU);
                                HPA
                            } else {
                                let HPB = (HOU - HOW) / BQG;
                                let HPC = HPB * HPB;
                                let HPD = rspice_limited_exp((HOW + (BQG * ((7.8125e-2f64 + (NZ * HPB)) + (HPC * (9.375e-1f64 - (HPC * (BYV - HPC))))))));
                                HPD
                            };
                            HPE = HPF;
                        }
                        let HPG = HPE * (((B + HOS) - HOU) - ((if ((AY * HOR) * (((HPE * AY) * HOR) + HOQ)) >= BAF { ((AY * HOR) * (((HPE * AY) * HOR) + HOQ)) } else { BAF }).ln()));
                        HPU = HPG;
                    } else {
                        let HPH = rspice_limited_exp(HOU);
                        let HPI = AY * HPH;
                        let HPJ = HPI * HOR;
                        let HPK = HOR + (B / HOP);
                        let HPL = HPH - (((HPI + ((if (HPJ * (HPJ + HOQ)) >= BAF { (HPJ * (HPJ + HOQ)) } else { BAF }).ln())) - HOS) / ((AY + (B / HPH)) + (HPK / ((HOR * HPH) + HOP))));
                        let HPM = AY * HPL;
                        let HPN = HPM * HOR;
                        let HPO = (HPM + ((if (HPN * (HPN + HOQ)) >= BAF { (HPN * (HPN + HOQ)) } else { BAF }).ln())) - HOS;
                        let HPP = B / HPL;
                        let HPQ = (HOR * HPL) + HOP;
                        let HPR = HPK / HPQ;
                        let HPS = (AY + HPP) + HPR;
                        let HPT = HPL - ((HPO / HPS) * (B + ((HPO * (((-(HPP * HPP)) - (B / (((HOP * HOP) * HOP) * HPQ))) - (HPR * HPR))) / ((AY * HPS) * HPS))));
                        HPU = HPT;
                    }
                    let HPV = ((HMQ - HOA) - HPU) - B;
                    let HPW = HPV - B;
                    let HPX = (NZ * ((HPV + B) + (((HPW * HPW) + 1e0f64).sqrt()))).sqrt();
                    let HPY = 1e0f64 + (HMF / (HOP + HPX));
                    let HPZ = NZ + ((A * HPX) * HMG);
                    let HQA = HOA + HPU;
                    let HQB = HPY / (HPZ + (((HPZ * HPZ) + ((HPY * HQA) * A)).sqrt()));
                    let HQC = HOA - HPU;
                    let HQD = HQC * HQC;
                    let HQE = HOF + HOB;
                    let HQF = HOF + (AY * HPU);
                    let HQG = (OQ + ((NZ * (HQE + (((HQE * HQE) + 6.25e-2f64).sqrt()))) * A)).sqrt();
                    let HQH = (OQ + ((NZ * (HQF + (((HQF * HQF) + 6.25e-2f64).sqrt()))) * A)).sqrt();
                    let HQI = AY * HQH;
                    let HQJ = B + HQI;
                    let HQK = HQG + HQH;
                    let HQL = HQK * HQK;
                    let HQM = 0e0f64 / (HOL + HPU);
                    let HQN = ((EQJ * HLZ) * BBM) * ((((HQE / (B + (AY * HQG))) + (HQF / HQJ)) + (((BDI * (HQD / (HQL * HQK))) * ((GKE * (HQL + (HQG * HQH))) * HQM)) - (HQB * (HQA + ((BDI * HQD) * HQM))))) + (0e0f64 * ((HOF - ((AY * (HQB - B)) * HPU)) + ((HQF * (HQI - B)) / HQJ))));
                    HQO = HQN;
                } else {
                    HQO = A;
                }
                let HQP = -((-HMA) + (BCG * HQO));
                let HQQ = if DOJ == 0.0 { 1.0 } else { 0.0 };
                if HQQ != 0.0 {
                } else {
                }
                let HQR = if DOM == A { 1.0 } else { 0.0 };
                if HQR != 0.0 {
                } else {
                }
                let HQS = (BN - DOO) + (AY * DOP);
                let HQT = if KW > A { 1.0 } else { 0.0 };
                let HQW = if HQT != 0.0 {
                    let HQU = (BRW * BBM) * ((if (LP / KW) >= BAF { (LP / KW) } else { BAF }).ln());
                    HQU
                } else {
                    let HQV = (BRW * BBM) * ((if ((((-LP) * KW) / BBT) / BBT) >= BAF { ((((-LP) * KW) / BBT) / BBT) } else { BAF }).ln());
                    HQV
                };
                let HQX = (((KX * DOX) * (DOW / BTY)) * ((((BQ / BAO) * AA) * HQS) + DOY)) * ((DOU - HQW) - DOZ);
                let HQY = if (BMP - V) > A { 1.0 } else { 0.0 };
                if HQY != 0.0 {
                } else {
                }
                let HQZ = if (BNG - V) > A { 1.0 } else { 0.0 };
                if HQZ != 0.0 {
                } else {
                }
                let HRA = if SU != A { 1.0 } else { 0.0 };
                if HRA != 0.0 {
                } else {
                }
                let HRB = if BPG == B { 1.0 } else { 0.0 };
                let ICA;
                let ICC;
                let ICE;
                if HRB != 0.0 {
                    let HRC = (if (JY / BBT) >= BAF { (JY / BBT) } else { BAF }).ln();
                    let HRD = if ((BCR + (BBM * HRC)) + FB) >= BCR { ((BCR + (BBM * HRC)) + FB) } else { BCR };
                    let HRE = B + (KI * BCZ);
                    let HRF = HRD - BSO;
                    let HRG = HRF - BQE;
                    let HRH = J / (((BCV / (C * JY)).sqrt()) * ((NZ * ((HRF + BQE) + (((HRG * HRG) + 2.5000000000000005e-3f64).sqrt()))).sqrt()));
                    let HRI = B + ((((JZ + (JX * (NZ * (HRE + (((HRE * HRE) + 4e-6f64).sqrt()))))) + (EZT * BSM)) - (KB * BSO)) / N);
                    let HRJ = HRI - B;
                    let HRK = NZ * ((HRI + B) + (((HRJ * HRJ) + 6.250000000000001e-4f64).sqrt()));
                    let HRL = HRK * BBM;
                    let HRM = B / HRL;
                    let HRN = BRC * HRM;
                    let HRO = BSD * HRM;
                    let HRP = BVI * HRM;
                    let HRQ = (-((EZO * (B + (KJ * BCZ))) + (KD * BSO))) * BSM;
                    let HRR = ((KE + (KF / AZ)) + (KG * BSO)) * ((BBP.powf(KH)) - B);
                    let HRS = BCY * (B + (FAJ * BSO));
                    let HRT = if HRS > A { 1.0 } else { 0.0 };
                    let HRY;
                    if HRT != 0.0 {
                        let HRU = (FAM * AZ) / HRS;
                        let HRV = if HRU < BUA { 1.0 } else { 0.0 };
                        let HRZ = if HRV != 0.0 {
                            let HRW = (NZ * FAP) / ((HRU.cosh()) - B);
                            HRW
                        } else {
                            let HRX = FAP * (rspice_limited_exp((-HRU)));
                            HRX
                        };
                        HRY = HRZ;
                    } else {
                        HRY = A;
                    }
                    let HSA = (HRN - HRP) - (((((((HRQ - HRR) + (HRY * (FAU - HRD))) + FBD) + FBE) - (BQZ * BSO)) + BQT) * HRM);
                    let HSB = (((((3.204352924e-19f64 * J) * JY) * HRM).sqrt()) / N) * (B + (FBI * (B + (FBJ * (AZ.powf((-FBK)))))));
                    let HSC = HRC / HRK;
                    let HSD = (NZ * HSA) - (UH * (B + (HSB / GCC)));
                    let HSE = HSD + (((HSD * HSD) + (UJ * HSA)).sqrt());
                    let HSF = if HSA < A { 1.0 } else { 0.0 };
                    let HSM = if HSF != 0.0 {
                        let HSG = (HSA - HSE) / HSB;
                        let HSH = -((if ((B - HSE) + (HSG * HSG)) >= BAF { ((B - HSE) + (HSG * HSG)) } else { BAF }).ln());
                        HSH
                    } else {
                        let HSI = rspice_limited_exp((-HSE));
                        let HSJ = NZ * HSB;
                        let HSK = ((((HSA - B) + HSI) + (HSJ * HSJ)).sqrt()) - HSJ;
                        let HSL = ((HSK * HSK) + B) - HSI;
                        HSL
                    };
                    let HSN = HSM + B;
                    let HSO = HSM - B;
                    let HSP = HSO * HSO;
                    let HSQ = (NZ * (HSN + ((HSP + 1e0f64).sqrt()))).sqrt();
                    let HSR = AY * HSQ;
                    let HSS = (B + (HSB / HSR)) / HSB;
                    let HST = HSM - (AY * HSC);
                    let HSU = HST - HRO;
                    let HSV = HSU - ((if ((UI * HSS) * HSQ) >= BAF { ((UI * HSS) * HSQ) } else { BAF }).ln());
                    let HSW = NZ * ((HSV - GCW) - (((HSV * (HSV + GCX)) + GCY).sqrt()));
                    let HSX = if HSW <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HTX;
                    if HSX != 0.0 {
                        let HSZ = if HSW < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HTG;
                        if HSZ != 0.0 {
                            HTG = HTA;
                        } else {
                            let HTB = if HSW > -9e1f64 { 1.0 } else { 0.0 };
                            let HTH = if HTB != 0.0 {
                                let HTC = rspice_limited_exp(HSW);
                                HTC
                            } else {
                                let HTD = (HSW - HSY) / BQG;
                                let HTE = HTD * HTD;
                                let HTF = rspice_limited_exp((HSY + (BQG * ((7.8125e-2f64 + (NZ * HTD)) + (HTE * (9.375e-1f64 - (HTE * (BYV - HTE))))))));
                                HTF
                            };
                            HTG = HTH;
                        }
                        let HTI = HTG * (((B + HSU) - HSW) - ((if ((AY * HSS) * (((HTG * AY) * HSS) + HSR)) >= BAF { ((AY * HSS) * (((HTG * AY) * HSS) + HSR)) } else { BAF }).ln()));
                        HTX = HTI;
                    } else {
                        let HTJ = rspice_limited_exp(HSW);
                        let HTK = AY * HTJ;
                        let HTL = HTK * HSS;
                        let HTM = HSS + (B / HSQ);
                        let HTN = HTJ - (((HTK + ((if (HTL * (HTL + HSR)) >= BAF { (HTL * (HTL + HSR)) } else { BAF }).ln())) - HSU) / ((AY + (B / HTJ)) + (HTM / ((HSS * HTJ) + HSQ))));
                        let HTO = AY * HTN;
                        let HTP = HTO * HSS;
                        let HTQ = (HTO + ((if (HTP * (HTP + HSR)) >= BAF { (HTP * (HTP + HSR)) } else { BAF }).ln())) - HSU;
                        let HTR = B / HTN;
                        let HTS = (HSS * HTN) + HSQ;
                        let HTT = HTM / HTS;
                        let HTU = (AY + HTR) + HTT;
                        let HTV = HTN - ((HTQ / HTU) * (B + ((HTQ * (((-(HTR * HTR)) - (B / (((HSQ * HSQ) * HSQ) * HTS))) - (HTT * HTT))) / ((AY * HTU) * HTU))));
                        HTX = HTV;
                    }
                    let HTW = AY * HRL;
                    let HTY = (((HTW * HTX) + HTW) + BSD) - BSD;
                    let HTZ = (NZ * (HSN + ((HSP + 1e0f64).sqrt()))).sqrt();
                    let HUA = AY * HTZ;
                    let HUB = (B + (HSB / HUA)) / HSB;
                    let HUC = HST - (((BSE * ((B + (((BSE / (NZ * (HTY + (((HTY * HTY) + 2.5e-7f64).sqrt())))) + BX).powf(GIK))).powf(GIL))) + BSD) * HRM);
                    let HUD = HUC - ((if ((UI * HUB) * HTZ) >= BAF { ((UI * HUB) * HTZ) } else { BAF }).ln());
                    let HUE = NZ * ((HUD - GCW) - (((HUD * (HUD + GCX)) + GCY).sqrt()));
                    let HUF = if HUE <= -6.8e1f64 { 1.0 } else { 0.0 };
                    let HVE;
                    if HUF != 0.0 {
                        let HUH = if HUE < -1.1e2f64 { 1.0 } else { 0.0 };
                        let HUO;
                        if HUH != 0.0 {
                            HUO = HUI;
                        } else {
                            let HUJ = if HUE > -9e1f64 { 1.0 } else { 0.0 };
                            let HUP = if HUJ != 0.0 {
                                let HUK = rspice_limited_exp(HUE);
                                HUK
                            } else {
                                let HUL = (HUE - HUG) / BQG;
                                let HUM = HUL * HUL;
                                let HUN = rspice_limited_exp((HUG + (BQG * ((7.8125e-2f64 + (NZ * HUL)) + (HUM * (9.375e-1f64 - (HUM * (BYV - HUM))))))));
                                HUN
                            };
                            HUO = HUP;
                        }
                        let HUQ = HUO * (((B + HUC) - HUE) - ((if ((AY * HUB) * (((HUO * AY) * HUB) + HUA)) >= BAF { ((AY * HUB) * (((HUO * AY) * HUB) + HUA)) } else { BAF }).ln()));
                        HVE = HUQ;
                    } else {
                        let HUR = rspice_limited_exp(HUE);
                        let HUS = AY * HUR;
                        let HUT = HUS * HUB;
                        let HUU = HUB + (B / HTZ);
                        let HUV = HUR - (((HUS + ((if (HUT * (HUT + HUA)) >= BAF { (HUT * (HUT + HUA)) } else { BAF }).ln())) - HUC) / ((AY + (B / HUR)) + (HUU / ((HUB * HUR) + HTZ))));
                        let HUW = AY * HUV;
                        let HUX = HUW * HUB;
                        let HUY = (HUW + ((if (HUX * (HUX + HUA)) >= BAF { (HUX * (HUX + HUA)) } else { BAF }).ln())) - HUC;
                        let HUZ = B / HUV;
                        let HVA = (HUB * HUV) + HTZ;
                        let HVB = HUU / HVA;
                        let HVC = (AY + HUZ) + HVB;
                        let HVD = HUV - ((HUY / HVC) * (B + ((HUY * (((-(HUZ * HUZ)) - (B / (((HTZ * HTZ) * HTZ) * HVA))) - (HVB * HVB))) / ((AY * HVC) * HVC))));
                        HVE = HVD;
                    }
                    let HVF = ((HSM - HTX) - HVE) - B;
                    let HVG = HVF - B;
                    let HVH = NZ * ((HVF + B) + (((HVG * HVG) + 1e0f64).sqrt()));
                    let HVI = B + (HSB / (((NZ * (HSN + ((HSP + 1e0f64).sqrt()))).sqrt()) + (HVH.sqrt())));
                    let HVJ = HTX - HVE;
                    let HVK = ((((((((GNN * HVI) * GNM) * GCA) / AZ) * N) * HRL) * HRL) * (HVJ * ((B + HTX) + HVE))) * GMD;
                    let HVM = EVQ * HVL;
                    let HVN = EVR * HVL;
                    let HVO = EVS * HVL;
                    let HVQ = AZ - (AY * HVP);
                    let HVR = GZF * ((N + HRH) + JZ);
                    let HVS = ((AY * HVI) * N) * BBM;
                    let HVT = (HVS * HVE) / C;
                    let HVU = HVT + HVR;
                    let HVV = (HVS * HTX) / C;
                    let HVW = (((((4.112737976006692e-57f64 * BBM) * (HVK.abs())) * GNM) / HVH) * (((HVM * ((if ((HVV + HVR) / HVU) >= BAF { ((HVV + HVR) / HVU) } else { BAF }).ln())) + (HVN * (HVV - HVT))) + ((NZ * HVO) * ((HVV * HVV) - (HVT * HVT))))) + ((((((GZL * HVK) * HVK) / (((EWD * (HVQ * HVQ)) * GCA) * AA)) * HCQ) * ((HVM + (HVN * HVT)) + ((HVO * HVT) * HVT))) / (HVU * HVU));
                    let HVX = ((((HVM * C) * BBM) / (((((GCA * AA) * HVQ) * EWD) * HVR) * HVR)) * HVK) * HVK;
                    let HVY = HVX + HVW;
                    let HVZ = if HVY > A { 1.0 } else { 0.0 };
                    let HWB = if HVZ != 0.0 {
                        let HWA = ((HVW * HVX) / HVY) / (B + (parameters[1317] * (HVJ.powf(parameters[1318]))));
                        HWA
                    } else {
                        A
                    };
                    let HWC = DGQ * HWB;
                    ICA = B;
                    ICC = HWC;
                    ICE = EWU;
                } else {
                    ICA = A;
                    ICC = A;
                    ICE = A;
                }
                let HWD = if DGQ > A { 1.0 } else { 0.0 };
                if HWD != 0.0 {
                } else {
                }
                HWE = GWF;
                HWF = GYY;
                HWH = HQX;
                HWL = HWO;
                HWQ = HWT;
                HWW = GZA;
                HXB = HXJ;
                HXP = HXX;
                HYN = HQP;
                HYU = A;
                HYV = A;
                HYW = A;
                HYX = A;
                HYZ = A;
                HZB = A;
                HZE = A;
                HZH = A;
                HZK = A;
                HZN = A;
                HZQ = A;
                HZT = A;
                HZV = A;
                HZX = A;
                HZZ = A;
                IAB = A;
                IAD = A;
                IAF = IAG;
                IAH = IAI;
                IAJ = IAK;
                IAL = IAM;
                IAN = IAO;
                IAP = IAQ;
                IAR = IAS;
                IAT = IAU;
                IAV = IAW;
                IAY = IAZ;
                IBB = IBC;
                IBE = IBF;
                IBH = IBI;
                IBK = IBL;
                IBN = IBO;
                IBP = IBQ;
                IBR = IBS;
                IBT = IBU;
                IBV = IBW;
                IBX = IBY;
                IBZ = ICA;
                ICB = ICC;
                ICD = ICE;
            }
            let HWG = if DGQ > A { 1.0 } else { 0.0 };
            if HWG != 0.0 {
            } else {
            }
            if HWG != 0.0 {
            } else {
            }
            let HWI = if ERC != A { 1.0 } else { 0.0 };
            if HWI != 0.0 {
            } else {
            }
            if RW != 0.0 {
            } else {
            }
            if HWG != 0.0 {
            } else {
            }
            let HWJ = if BAB == A { 1.0 } else { 0.0 };
            let ICF;
            let ICG;
            if HWJ != 0.0 {
                ICF = A;
                ICG = A;
            } else {
                let HWK = if BAB == AY { 1.0 } else { 0.0 };
                let HWX = if HWK != 0.0 {
                    let HWV = (HWL * HWL) / HWQ;
                    HWV
                } else {
                    HWQ
                };
                let HWY = HWW * HWX;
                ICF = B;
                ICG = HWY;
            }
            let HWZ = if QR != AY { 1.0 } else { 0.0 };
            let HXA = if HWZ != 0.0 && (if CIW > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HXM = if HXA != 0.0 {
                let HXL = B / HXB;
                HXL
            } else {
                A
            };
            let ICH;
            let ICI;
            if HXA != 0.0 {
                let HXN = HWW * HXM;
                ICH = B;
                ICI = HXN;
            } else {
                ICH = A;
                ICI = A;
            }
            let HXO = if HWZ != 0.0 && (if CIZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HYA = if HXO != 0.0 {
                let HXZ = B / HXP;
                HXZ
            } else {
                A
            };
            let ICJ;
            let ICK;
            if HXO != 0.0 {
                let HYB = HWW * HYA;
                ICJ = B;
                ICK = HYB;
            } else {
                ICJ = A;
                ICK = A;
            }
            let HYC = if BAB == UH { 1.0 } else { 0.0 };
            if HYC != 0.0 {
            } else {
            }
            if BAW != 0.0 {
                if HXA != 0.0 {
                } else {
                }
                if HXO != 0.0 {
                } else {
                }
                let HYD = if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 };
                if HYD != 0.0 {
                    if B != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let HYE = if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 };
                if HYE != 0.0 {
                    if B != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let HYF = parameters[1359] * parameters[1358];
            let HYG = if (if BCE == A { 1.0 } else { 0.0 }) != 0.0 || 0.0f64 != 0.0 { 1.0 } else { 0.0 };
            if HYG != 0.0 {
            } else {
                let HYH = if BAY != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 };
                if HYH != 0.0 {
                } else {
                    let HYI = if BCE == B { 1.0 } else { 0.0 };
                    if HYI != 0.0 {
                        let HYL = if ((((((parameters[1357] * HYJ) * HYK) / ((AY * HYJ) + (HYK * AZ))) * BG) / BAO) / AA) < BDB { 1.0 } else { 0.0 };
                        if HYL != 0.0 {
                            let HYM = if HYF <= BDB { 1.0 } else { 0.0 };
                            if HYM != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                        let HYO = if (B / ((AA * ((GS * (BBP.powf(GT))) * (((((C * GU) * BTV) * BG) * AZ) - ((-((HYN + HWE) + HWF)) + HWH)))) / (BG * BG))) < BDB { 1.0 } else { 0.0 };
                        if HYO != 0.0 {
                            let HYP = if HYF <= BDB { 1.0 } else { 0.0 };
                            if HYP != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                }
            }
            let HYQ = if BD == AY { 1.0 } else { 0.0 };
            if HYQ != 0.0 {
            } else {
            }
            let HYR = if parameters[1374] < BDB { 1.0 } else { 0.0 };
            if HYR != 0.0 {
            } else {
            }
            if B != 0.0 {
                let HYS = if (if BAY == A { 1.0 } else { 0.0 }) != 0.0 || B != 0.0 { 1.0 } else { 0.0 };
                if HYS != 0.0 {
                } else {
                }
            } else {
            }
            let HYT = if (if (if BCN != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if BCO != 0.0 && 0.0f64 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BCG == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if HYT != 0.0 {
            } else {
            }
        if HYU == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HYV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(HYW);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if HYX == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HYZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if HZB == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HZE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if HZH == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HZK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if HZN == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HZQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if HZT == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HZV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if HZX == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = HZZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IAB == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IAD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IAF == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IAH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(IAJ);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IAL == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IAN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(IAP);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IAR == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IAT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IAV == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IAY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IBB == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IBE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IBH == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IBK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IBN == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IBP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IBR == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IBT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IBV == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IBX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IBZ == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ICB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(ICD);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ICF == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ICG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ICH == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ICI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ICJ == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ICK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
