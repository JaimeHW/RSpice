#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::{Lanes, rspice_limited_exp, rspice_limited_exp_derivative};
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 84, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI2_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 85, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "di2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI2_DI1_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 86, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "di2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI1_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 87, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI1_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 88, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GE_RG", label: Some("rg"), kind: GeneratedNoiseKind::White, equation: 89, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "ge", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 90, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 93, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "n", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 94, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS_V", label: Some("igs_v"), kind: GeneratedNoiseKind::White, equation: 98, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD_V", label: Some("igd_v"), kind: GeneratedNoiseKind::White, equation: 99, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD_V", label: Some("igd_v"), kind: GeneratedNoiseKind::White, equation: 100, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS_V", label: Some("igs_v"), kind: GeneratedNoiseKind::White, equation: 101, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_E_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 102, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGBS_V", label: Some("igbs_v"), kind: GeneratedNoiseKind::White, equation: 103, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGBD_V", label: Some("igbd_v"), kind: GeneratedNoiseKind::White, equation: 104, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
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
            let C = 1.0f64;
            let D = parameters[74];
            let E = parameters[1791];
            let G = parameters[60];
            let I = -1e0f64;
            let J = parameters[103];
            let K = 8.8542e-12f64;
            let M = parameters[1088];
            let O = parameters[102];
            let Q = 4e0f64;
            let R = 2e0f64;
            let S = 1e-6f64;
            let T = parameters[0];
            let U = parameters[5];
            let AD = parameters[83];
            let AF = parameters[85];
            let AL = 1e-9f64;
            let AR = parameters[61];
            let AV = parameters[62];
            let AW = 5e0f64;
            let AY = parameters[43];
            let AZ = 1e-12f64;
            let BJ = parameters[59];
            let BW = parameters[95];
            let BZ = 1e-38f64;
            let CC = -8.7498233534e1f64;
            let CH = 1e22f64;
            let CI = 1e18f64;
            let CL = parameters[1802];
            let CM = parameters[1803];
            let CO = parameters[92];
            let CQ = parameters[89];
            let CS = parameters[3];
            let DQ = 3e0f64;
            let DS = parameters[2];
            let DX = -8.7498233534e1f64;
            let EC = parameters[1801];
            let ED = parameters[1800];
            let EE = parameters[1799];
            let EF = parameters[40];
            let EK = parameters[56];
            let HD = 1.60219e-19f64;
            let HN = parameters[1085];
            let HO = parameters[1680];
            let HQ = parameters[145];
            let NF = parameters[881];
            let OJ = parameters[70];
            let OT = parameters[66];
            let PV = parameters[582];
            let QA = -8.7498233534e1f64;
            let QP = parameters[161];
            let QU = -8.7498233534e1f64;
            let QX = parameters[21];
            let RF = parameters[73];
            let RH = parameters[1668];
            let RW = parameters[100];
            let SB = -8.7498233534e1f64;
            let SE = parameters[158];
            let SJ = -8.7498233534e1f64;
            let SM = parameters[152];
            let SR = -8.7498233534e1f64;
            let SU = parameters[154];
            let SZ = -8.7498233534e1f64;
            let TC = parameters[156];
            let TH = -8.7498233534e1f64;
            let TL = parameters[428];
            let TQ = -8.7498233534e1f64;
            let TT = parameters[432];
            let TY = -8.7498233534e1f64;
            let UB = parameters[434];
            let UG = -8.7498233534e1f64;
            let UK = parameters[581];
            let UP = -8.7498233534e1f64;
            let US = parameters[583];
            let UX = -8.7498233534e1f64;
            let VN = parameters[589];
            let WC = parameters[590];
            let WI = parameters[64];
            let XS = 5e-2f64;
            let XY = 4.61e0f64;
            let YC = 1e-2f64;
            let YG = parameters[1682];
            let YI = 1.2e0f64;
            let YM = 8.5e4f64;
            let YR = 6e-1f64;
            let ZA = 1.06e0f64;
            let ZJ = 2e-1f64;
            let ZR = 3e-2f64;
            let ABJ = parameters[1108];
            let ABQ = 1e-3f64;
            let ABS = parameters[71];
            let ACJ = 2e-2f64;
            let ACN = parameters[4];
            let ACT = 2.6e0f64;
            let ACY = 1.4e1f64;
            let ADB = 2.4e1f64;
            let ADG = 1.39e-1f64;
            let ADL = 1.12e1f64;
            let ADO = 8.02e0f64;
            let ADR = 6.18e0f64;
            let ADW = parameters[76];
            let ADY = parameters[6];
            let AEC = 1.2e1f64;
            let AEJ = parameters[1080];
            let AEP = if parameter_given[1083] { 1.0 } else { 0.0 };
            let AEQ = parameters[1083];
            let AER = 1.417e3f64;
            let AES = 4.705e2f64;
            let AEU = parameters[97];
            let AEW = 5.22e1f64;
            let AEX = 1e-4f64;
            let AFA = 4.49e1f64;
            let AFJ = parameters[1082];
            let AFL = parameters[20];
            let AGG = parameters[151];
            let AGM = parameters[78];
            let AGO = if parameter_given[1542] { 1.0 } else { 0.0 };
            let AGP = if parameter_given[85] { 1.0 } else { 0.0 };
            let AGS = if parameter_given[1543] { 1.0 } else { 0.0 };
            let AGW = parameters[1089];
            let AGX = parameters[1090];
            let AGZ = 5e-1f64;
            let AHB = parameters[90];
            let AHD = parameters[1081];
            let AHG = 1e-7f64;
            let AHH = 3.9e0f64;
            let AHI = parameters[1087];
            let AHK = 2.3e0f64;
            let AHN = 1.05e0f64;
            let AHQ = 1.7e12f64;
            let AHS = 8e1f64;
            let AHU = 3.7e1f64;
            let ANY = 1e6f64;
            let AOF = 4e1f64;
            let AOJ = parameters[172];
            let AOQ = parameters[174];
            let AOX = parameters[173];
            let APC = parameters[171];
            let APH = 4.97232e-7f64;
            let API = 7.45669e11f64;
            let APJ = 3.42537e-7f64;
            let APK = 1.16645e12f64;
            let APL = parameters[1109];
            let APT = parameters[1717];
            let APV = 3.0015e2f64;
            let AQA = 1e9f64;
            let AQO = 2.5e-1f64;
            let AQY = 1.001e0f64;
            let ARB = 2.001e0f64;
            let ART = 1.5e0f64;
            let ARX = parameters[1893];
            let ARZ = 9.24e5f64;
            let ASA = 1.81e4f64;
            let ASD = 5.5e0f64;
            let ASE = parameters[1894];
            let ASG = 8e0f64;
            let ASI = parameters[1895];
            let ASK = parameters[1896];
            let ASL = parameters[1897];
            let ASN = 1e-1f64;
            let ASS = parameters[1899];
            let ASU = parameters[1900];
            let ASV = parameters[1901];
            let ASX = parameters[1902];
            let ATZ = 3.14e0f64;
            let AUA = 3.85e-2f64;
            let AUD = 7.5893e-7f64;
            let AUF = 6.9583e-5f64;
            let AUH = 6e0f64;
            let AUI = 6.583e-4f64;
            let AUJ = 6.5e-3f64;
            let AUK = 2.6e-2f64;
            let AUL = 1.371e-1f64;
            let AUM = 9.59e-1f64;
            let AVB = parameters[889];
            let AVG = parameters[892];
            let AVH = parameters[894];
            let AVM = parameters[897];
            let AVT = parameters[905];
            let AVU = parameters[906];
            let AVX = 1e-5f64;
            let AWF = temperature;
            let AWG = parameters[22];
            let AWO = 8.617087e-5f64;
            let AWR = parameters[1786];
            let AWS = parameters[80];
            let AWV = parameters[1788];
            let AXA = parameters[1787];
            let AXC = parameters[1789];
            let AXS = 2.1e2f64;
            let AZA = parameters[106];
            let AZB = parameters[1718];
            let AZD = parameters[1719];
            let AZL = parameters[105];
            let AZT = parameters[107];
            let BAC = -8.7498233534e1f64;
            let BAR = 4.389473684210526e0f64;
            let BAW = 3.493821377127659e-68f64;
            let BBE = -8.7498233534e1f64;
            let BBM = 9e-1f64;
            let BBN = -9e-1f64;
            let BBP = -9e-1f64;
            let BBQ = -9e-1f64;
            let BBS = -9e-1f64;
            let BBU = -9e-1f64;
            let BCD = -9e-1f64;
            let BCG = -9e-1f64;
            let BCH = -9e-1f64;
            let BCJ = -9e-1f64;
            let BCL = -9e-1f64;
            let BDS = parameters[75];
            let BEY = 1e3f64;
            let BFC = 0e0f64;
            let BFE = 0e0f64;
            let BFH = 0e0f64;
            let BFL = 0e0f64;
            let BFO = 0e0f64;
            let BFS = 0e0f64;
            let BFU = 0e0f64;
            let BHK = parameters[450];
            let BHU = parameters[452];
            let BIC = parameters[1720];
            let BIK = -9e-1f64;
            let BIM = -9e-1f64;
            let BIN = -9e-1f64;
            let BIP = -9e-1f64;
            let BIR = -9e-1f64;
            let BIY = -9e-1f64;
            let BJA = -9e-1f64;
            let BJB = -9e-1f64;
            let BJD = -9e-1f64;
            let BJF = -9e-1f64;
            let BKQ = 0e0f64;
            let BKV = 0e0f64;
            let BKY = 0e0f64;
            let BMQ = parameters[561];
            let BPC = parameters[574];
            let BPL = parameters[451];
            let BQD = parameters[498];
            let BQE = parameters[499];
            let BQS = parameters[1026];
            let BRE = parameters[1747];
            let BRF = parameters[1748];
            let BRG = parameters[1749];
            let BRR = -9e-1f64;
            let BRT = -9e-1f64;
            let BRU = -9e-1f64;
            let BRW = -9e-1f64;
            let BRY = -9e-1f64;
            let BSF = -9e-1f64;
            let BSH = -9e-1f64;
            let BSI = -9e-1f64;
            let BSK = -9e-1f64;
            let BSM = -9e-1f64;
            let BST = -8.7498233534e1f64;
            let BTF = -8.7498233534e1f64;
            let BUC = -8.7498233534e1f64;
            let BUO = -8.7498233534e1f64;
            let CAZ = -8.7498233534e1f64;
            let CBI = -8.7498233534e1f64;
            let CHY = 0e0f64;
            let CIC = 0e0f64;
            let CJG = parameters[164];
            let CJT = 0e0f64;
            let CJV = parameters[165];
            let CKG = parameters[166];
            let CLD = parameters[917];
            let CLF = parameters[923];
            let CLO = parameters[918];
            let CLY = parameters[919];
            let CMA = parameters[924];
            let CMJ = parameters[920];
            let CQN = 1e-25f64;
            let CQR = 1e-20f64;
            let CQW = parameters[1584];
            let CQY = parameters[1721];
            let CRH = parameters[1585];
            let CRR = parameters[1586];
            let CRT = parameters[1722];
            let CSC = parameters[1587];
            let CSM = parameters[1588];
            let CSO = parameters[1723];
            let CSX = parameters[1589];
            let CUV = parameters[1620];
            let CVA = parameters[1621];
            let CVX = -8.7498233534e1f64;
            let CWH = -8.7498233534e1f64;
            let CWT = -8.7498233534e1f64;
            let CXD = -8.7498233534e1f64;
            let CXP = -8.7498233534e1f64;
            let CXX = parameters[104];
            let CYC = -8.7498233534e1f64;
            let CYL = parameters[1106];
            let CYU = -8.7498233534e1f64;
            let CZD = -8.7498233534e1f64;
            let CZI = -8.7498233534e1f64;
            let CZR = -8.7498233534e1f64;
            let DAA = 3.33333333e-1f64;
            let DAC = parameters[11];
            let DAE = parameters[13];
            let DAL = parameters[1626];
            let DAN = parameters[1628];
            let DAP = 1e1f64;
            let DAU = -8.7498233534e1f64;
            let DBF = -8.7498233534e1f64;
            let DBL = parameters[12];
            let DBN = parameters[14];
            let DBT = parameters[1627];
            let DBV = parameters[1629];
            let DCB = -8.7498233534e1f64;
            let DCM = -8.7498233534e1f64;
            let DDK = parameters[1602];
            let DDN = parameters[1596];
            let DDP = parameters[1608];
            let DDR = parameters[1604];
            let DDU = parameters[1598];
            let DDW = parameters[1610];
            let DDY = parameters[1606];
            let DEB = parameters[1600];
            let DED = parameters[1612];
            let DEF = parameters[1603];
            let DEI = parameters[1597];
            let DEK = parameters[1609];
            let DEM = parameters[1605];
            let DEP = parameters[1599];
            let DER = parameters[1611];
            let DET = parameters[1607];
            let DEW = parameters[1601];
            let DEY = parameters[1613];
            let DFA = node_potentials[11];
            let DFB = node_potentials[6];
            let DFD = node_potentials[5];
            let DFG = node_potentials[3];
            let DFL = node_potentials[10];
            let DFO = node_potentials[14];
            let DFS = -1e0f64;
            let DFU = -1e0f64;
            let DGF = 4e-3f64;
            let DLA = parameters[175];
            let DLR = -8.7498233534e1f64;
            let DLY = -8.7498233534e1f64;
            let DMF = 6.66666667e-1f64;
            let DNG = parameters[108];
            let DNN = -8.7498233534e1f64;
            let DNQ = parameters[23];
            let DOD = -8.7498233534e1f64;
            let DPW = parameters[1805];
            let DQG = -8.7498233534e1f64;
            let DQJ = -8.7498233534e1f64;
            let DQR = -3.33333333e-1f64;
            let DQW = 2.222222222222222e-1f64;
            let DQY = -1.333333333e0f64;
            let DRQ = -8.7498233534e1f64;
            let DRT = -8.7498233534e1f64;
            let DSA = -3.33333333e-1f64;
            let DSF = 2.222222222222222e-1f64;
            let DSH = -1.333333333e0f64;
            let DVD = parameters[604];
            let DVG = parameters[24];
            let DVM = parameters[908];
            let DZI = -8.7498233534e1f64;
            let DZL = -8.7498233534e1f64;
            let DZT = -3.33333333e-1f64;
            let DZY = 2.222222222222222e-1f64;
            let EAA = -1.333333333e0f64;
            let EAS = -8.7498233534e1f64;
            let EAV = -8.7498233534e1f64;
            let EBC = -3.33333333e-1f64;
            let EBH = 2.222222222222222e-1f64;
            let EBJ = -1.333333333e0f64;
            let ECW = 0e0f64;
            let EDA = 0e0f64;
            let EDI = -8.7498233534e1f64;
            let EDU = -8.7498233534e1f64;
            let EFG = -8.7498233534e1f64;
            let EFJ = -8.7498233534e1f64;
            let EFX = -8.7498233534e1f64;
            let EGA = -8.7498233534e1f64;
            let EGW = 0e0f64;
            let EHH = 0e0f64;
            let EJS = -8.7498233534e1f64;
            let EJV = -8.7498233534e1f64;
            let EKJ = -8.7498233534e1f64;
            let EKM = -8.7498233534e1f64;
            let ELG = 6.25e-4f64;
            let ELI = parameters[603];
            let ELQ = -8.7498233534e1f64;
            let EMK = -8.7498233534e1f64;
            let ENA = 1e-15f64;
            let EPQ = parameters[887];
            let EPY = 0e0f64;
            let EQB = 0e0f64;
            let EQD = 0e0f64;
            let ESJ = -8.7498233534e1f64;
            let ESV = -8.7498233534e1f64;
            let ETD = -8.7498233534e1f64;
            let ETJ = -8.7498233534e1f64;
            let EUC = parameters[453];
            let EVJ = 0e0f64;
            let EVV = -8.7498233534e1f64;
            let EWM = node_potentials[8];
            let EWT = node_potentials[9];
            let EXW = parameters[25];
            let EXZ = 0e0f64;
            let EYV = parameters[63];
            let EZR = -8.7498233534e1f64;
            let FAJ = node_potentials[7];
            let FAM = parameters[530];
            let FBG = -8.7498233534e1f64;
            let FBU = parameters[1441];
            let FCA = parameters[1442];
            let FCR = parameters[27];
            let FDH = 8e-2f64;
            let FET = parameters[1104];
            let FFK = parameters[1105];
            let FJZ = parameters[1643];
            let FKD = parameters[1645];
            let FKH = parameters[1647];
            let FLB = parameters[1644];
            let FLF = parameters[1646];
            let FLJ = parameters[1648];
            let FSX = parameters[1683];
            let FSY = parameters[1684];
            let FTC = parameters[79];
            let FTH = parameters[1681];
            let FTM = -8.7498233534e1f64;
            let FTU = parameters[1688];
            let FTX = 1e10f64;
            let FUG = -8.7498233534e1f64;
            let FUZ = -8.7498233534e1f64;
            let FVC = parameters[72];
            let FXT = parameters[1910];
            let FXV = parameters[1912];
            let FYB = parameters[1904];
            let FYD = parameters[1913];
            let FYM = parameters[1906];
            let FYQ = parameters[1907];
            let FYT = parameters[1905];
            let FZB = parameters[1917];
            let FZI = parameters[1908];
            let FZN = parameters[1911];
            let GHB = 1e0f64;
            let GHC = 1e0f64;
            let GHD = 1e0f64;
            let GHE = 1e0f64;
            let GHF = 1e0f64;
            let GHG = 1e0f64;
            let GRF = 0e0f64;
            let GRJ = 2e0f64;
            let GRP = -1e0f64;
            let HIO = Lanes([0e0f64; 3]);
            let HIX = Lanes([0e0f64; 3]);
            let HMT = Lanes([0e0f64; 5]);
            if C != 0.0 {
                let F = if (if (if D == A { 1.0 } else { 0.0 }) != 0.0 || (if E == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[81] == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if F != 0.0 {
                } else {
                }
            } else {
            }
            let H = if G == B { 1.0 } else { 0.0 };
            let CWN = if H != 0.0 {
                B
            } else {
                I
            };
            let L = J * K;
            let N = M * K;
            let P = J / O;
            let V = T * U;
            let W = ((parameters[117] + ((S * parameters[118]) / T)) + (parameters[119] / U)) + ((parameters[120] * S) / V);
            let X = ((parameters[113] + ((S * parameters[114]) / T)) + (parameters[115] / U)) + ((parameters[116] * S) / V);
            let Y = T + (((parameters[109] + ((S * parameters[110]) / T)) + (parameters[111] / U)) + ((parameters[112] * S) / V));
            let Z = if Y <= A { 1.0 } else { 0.0 };
            let AA = if Z != 0.0 {
                T
            } else {
                Y
            };
            let AB = -parameters[84];
            let AC = AA.powf(AB);
            let AE = AA + W;
            let AG = AA - (R * (X + (AD * AC)));
            let AH = AE - (R * (X + (AD * (AE.powf(AB)))));
            let AI = AA - (R * (AF + (parameters[88] * AC)));
            let AJ = AI - parameters[86];
            let AK = if AG <= A { 1.0 } else { 0.0 };
            let RB;
            if AK != 0.0 {
                RB = AA;
            } else {
                let AM = if AG <= AL { 1.0 } else { 0.0 };
                if AM != 0.0 {
                } else {
                }
                RB = AG;
            }
            let AN = if AH <= A { 1.0 } else { 0.0 };
            let BL;
            if AN != 0.0 {
                BL = AA;
            } else {
                let AO = if AH <= AL { 1.0 } else { 0.0 };
                if AO != 0.0 {
                } else {
                }
                BL = AH;
            }
            let AP = if AI <= A { 1.0 } else { 0.0 };
            let XF;
            if AP != 0.0 {
                XF = AA;
            } else {
                let AQ = if AI <= AL { 1.0 } else { 0.0 };
                if AQ != 0.0 {
                } else {
                }
                XF = AI;
            }
            let AS = if AR != A { 1.0 } else { 0.0 };
            if AS != 0.0 {
                let AT = if AJ <= A { 1.0 } else { 0.0 };
                if AT != 0.0 {
                } else {
                    let AU = if AJ <= AL { 1.0 } else { 0.0 };
                    if AU != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let AX = if AV == AW { 1.0 } else { 0.0 };
            let BD;
            let BF;
            if AX != 0.0 {
                let BA = T * AY;
                let BB = ((((parameters[121] + ((S * parameters[122]) / T)) + (parameters[123] / U)) + ((parameters[124] * S) / V)) + ((S * parameters[125]) / AY)) + ((parameters[126] * AZ) / BA);
                let BC = ((((parameters[127] + ((S * parameters[128]) / T)) + (parameters[129] / U)) + ((parameters[130] * S) / V)) + ((S * parameters[131]) / AY)) + ((parameters[132] * AZ) / BA);
                BD = BB;
                BF = BC;
            } else {
                BD = A;
                BF = A;
            }
            let BE = AY + BD;
            let BG = BE + BF;
            let BP;
            if AX != 0.0 {
                let BH = if BG <= A { 1.0 } else { 0.0 };
                let BQ;
                if BH != 0.0 {
                    BQ = AY;
                } else {
                    let BI = if BG <= AL { 1.0 } else { 0.0 };
                    if BI != 0.0 {
                    } else {
                    }
                    BQ = BG;
                }
                BP = BQ;
            } else {
                BP = BG;
            }
            let BK = U * BJ;
            let BM = S / BL;
            let BN = B / U;
            let BO = S / (BL * U);
            let BT;
            let BU;
            if AX != 0.0 {
                let BR = S / BP;
                let BS = AZ / (BP * BL);
                BT = BR;
                BU = BS;
            } else {
                BT = A;
                BU = A;
            }
            let BV = ((((parameters[133] + (BM * parameters[134])) + (BN * parameters[135])) + (BO * parameters[136])) + (BT * A)) + (BU * A);
            let BX = if BW != A { 1.0 } else { 0.0 };
            let CF;
            if BX != 0.0 {
                let BY = B + (U / parameters[96]);
                let CA = if BY > BZ { 1.0 } else { 0.0 };
                let CD = if CA != 0.0 {
                    let CB = BY.ln();
                    CB
                } else {
                    CC
                };
                let CE = BV * (B + ((BW / U) * CD));
                CF = CE;
            } else {
                CF = BV;
            }
            let CG = if CF <= A { 1.0 } else { 0.0 };
            let HE;
            if CG != 0.0 {
                HE = CH;
            } else {
                let CJ = if CF <= CI { 1.0 } else { 0.0 };
                if CJ != 0.0 {
                } else {
                }
                HE = CF;
            }
            let CK = if AV == A { 1.0 } else { 0.0 };
            let FW;
            let GF;
            let GO;
            if CK != 0.0 {
                let CN = if (if CL == A { 1.0 } else { 0.0 }) != 0.0 || (if CM == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let FX;
                let GG;
                let GP;
                if CN != 0.0 {
                    let CP = R * CO;
                    let CR = ((CP * O) * K) / CQ;
                    let CT = CO * CS;
                    FX = CR;
                    GG = CP;
                    GP = CT;
                } else {
                    let CU = CL - CM;
                    let CV = R * (((CO * CO) + ((CU * CU) / Q)).sqrt());
                    let CW = ((CV * O) * K) / CQ;
                    let CX = (CO * (CL + CM)) / R;
                    FX = CW;
                    GG = CV;
                    GP = CX;
                }
                FW = FX;
                GF = GG;
                GO = GP;
            } else {
                let CY = if AV == B { 1.0 } else { 0.0 };
                let FY;
                let GH;
                let GQ;
                if CY != 0.0 {
                    let CZ = if (if CL == A { 1.0 } else { 0.0 }) != 0.0 || (if CM == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let FZ;
                    let GI;
                    let GR;
                    if CZ != 0.0 {
                        let DA = (R * CO) + CS;
                        let DB = ((DA * O) * K) / CQ;
                        let DC = CO * CS;
                        FZ = DB;
                        GI = DA;
                        GR = DC;
                    } else {
                        let DD = CL - CM;
                        let DE = (R * (((CO * CO) + ((DD * DD) / Q)).sqrt())) + CL;
                        let DF = ((DE * O) * K) / CQ;
                        let DG = (CO * (CL + CM)) / R;
                        FZ = DF;
                        GI = DE;
                        GR = DG;
                    }
                    FY = FZ;
                    GH = GI;
                    GQ = GR;
                } else {
                    let DH = if AV == R { 1.0 } else { 0.0 };
                    let GA;
                    let GJ;
                    let GS;
                    if DH != 0.0 {
                        let DI = if (if CL == A { 1.0 } else { 0.0 }) != 0.0 || (if CM == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let GB;
                        let GK;
                        let GT;
                        if DI != 0.0 {
                            let DJ = (R * CO) + (R * CS);
                            let DK = ((DJ * O) * K) / CQ;
                            let DL = CO * CS;
                            GB = DK;
                            GK = DJ;
                            GT = DL;
                        } else {
                            let DM = CL - CM;
                            let DN = ((R * (((CO * CO) + ((DM * DM) / Q)).sqrt())) + CL) + CM;
                            let DO = ((DN * O) * K) / CQ;
                            let DP = (CO * (CL + CM)) / R;
                            GB = DO;
                            GK = DN;
                            GT = DP;
                        }
                        GA = GB;
                        GJ = GK;
                        GS = GT;
                    } else {
                        let DR = if AV == DQ { 1.0 } else { 0.0 };
                        let GC;
                        let GL;
                        let GU;
                        if DR != 0.0 {
                            let DT = 3.141592653589793e0f64 * DS;
                            let DU = B + ((R * CQ) / DS);
                            let DV = if DU > BZ { 1.0 } else { 0.0 };
                            let DY = if DV != 0.0 {
                                let DW = DU.ln();
                                DW
                            } else {
                                DX
                            };
                            let DZ = ((6.283185307179586e0f64 * O) * K) / DY;
                            let EA = (DT * DS) / Q;
                            GC = DZ;
                            GL = DT;
                            GU = EA;
                        } else {
                            let EB = if AV == Q { 1.0 } else { 0.0 };
                            let GD;
                            let GM;
                            let GV;
                            if EB != 0.0 {
                                GD = ED;
                                GM = EC;
                                GV = EE;
                            } else {
                                let GE;
                                let GN;
                                let GW;
                                if AX != 0.0 {
                                    let EG = R * (BE + EF);
                                    let EH = EG + parameters[44];
                                    let EI = BE * EF;
                                    let EJ = EI + parameters[45];
                                    let EL = if EK > B { 1.0 } else { 0.0 };
                                    let ET;
                                    let EV;
                                    let FU;
                                    let HB;
                                    if EL != 0.0 {
                                        let EM = EG + parameters[46];
                                        let EN = EI + parameters[47];
                                        let EO = EH + EM;
                                        let EP = EJ + EN;
                                        ET = EM;
                                        EV = EN;
                                        FU = EO;
                                        HB = EP;
                                    } else {
                                        ET = A;
                                        EV = A;
                                        FU = EH;
                                        HB = EJ;
                                    }
                                    let EQ = if EK > R { 1.0 } else { 0.0 };
                                    let FA;
                                    let FC;
                                    let FT;
                                    let HA;
                                    if EQ != 0.0 {
                                        let ER = EG + parameters[48];
                                        let ES = EI + parameters[49];
                                        let EU = (EH + ET) + ER;
                                        let EW = (EJ + EV) + ES;
                                        FA = ER;
                                        FC = ES;
                                        FT = EU;
                                        HA = EW;
                                    } else {
                                        FA = A;
                                        FC = A;
                                        FT = FU;
                                        HA = HB;
                                    }
                                    let EX = if EK > DQ { 1.0 } else { 0.0 };
                                    let FH;
                                    let FJ;
                                    let FS;
                                    let GZ;
                                    if EX != 0.0 {
                                        let EY = EG + parameters[50];
                                        let EZ = EI + parameters[51];
                                        let FB = ((EH + ET) + FA) + EY;
                                        let FD = ((EJ + EV) + FC) + EZ;
                                        FH = EY;
                                        FJ = EZ;
                                        FS = FB;
                                        GZ = FD;
                                    } else {
                                        FH = A;
                                        FJ = A;
                                        FS = FT;
                                        GZ = HA;
                                    }
                                    let FE = if EK > Q { 1.0 } else { 0.0 };
                                    let FM;
                                    let FO;
                                    let FR;
                                    let GY;
                                    if FE != 0.0 {
                                        let FF = EG + parameters[52];
                                        let FG = EI + parameters[53];
                                        let FI = (((EH + ET) + FA) + FH) + FF;
                                        let FK = (((EJ + EV) + FC) + FJ) + FG;
                                        FM = FF;
                                        FO = FG;
                                        FR = FI;
                                        GY = FK;
                                    } else {
                                        FM = A;
                                        FO = A;
                                        FR = FS;
                                        GY = GZ;
                                    }
                                    let FL = if EK > AW { 1.0 } else { 0.0 };
                                    let FQ;
                                    let GX;
                                    if FL != 0.0 {
                                        let FN = ((((EH + ET) + FA) + FH) + FM) + (EG + parameters[54]);
                                        let FP = ((((EJ + EV) + FC) + FJ) + FO) + (EI + parameters[55]);
                                        FQ = FN;
                                        GX = FP;
                                    } else {
                                        FQ = FR;
                                        GX = GY;
                                    }
                                    let FV = ((FQ * O) * K) / CQ;
                                    GE = FV;
                                    GN = FQ;
                                    GW = GX;
                                } else {
                                    GE = A;
                                    GN = A;
                                    GW = A;
                                }
                                GD = GE;
                                GM = GN;
                                GV = GW;
                            }
                            GC = GD;
                            GL = GM;
                            GU = GV;
                        }
                        GA = GC;
                        GJ = GL;
                        GS = GU;
                    }
                    FY = GA;
                    GH = GJ;
                    GQ = GS;
                }
                FW = FY;
                GF = GH;
                GO = GQ;
            }
            let HC = (R * FW) / (((GF * GF) * L) / GO);
            let HF = ((-1.60219e-19f64 * HE) * GO) / FW;
            let HG = FW / GF;
            if AS != 0.0 {
            } else {
            }
            let HH = GF - parameters[93];
            let HI = GF - parameters[94];
            let HK = if AX != 0.0 {
                let HJ = HH - ((R * EK) * parameters[87]);
                HJ
            } else {
                HH
            };
            if AX != 0.0 {
                if AS != 0.0 {
                    let HL = if HK <= A { 1.0 } else { 0.0 };
                    if HL != 0.0 {
                    } else {
                        let HM = if BP <= AL { 1.0 } else { 0.0 };
                        if HM != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            let HP = ((((parameters[137] + (BM * parameters[138])) + (BN * parameters[139])) + (BO * parameters[140])) + (BT * parameters[141])) + (BU * parameters[142]);
            let HR = ((((HQ + (BM * parameters[146])) + (BN * parameters[147])) + (BO * parameters[148])) + (BT * parameters[149])) + (BU * parameters[150]);
            let HS = ((((parameters[188] + (BM * parameters[189])) + (BN * parameters[190])) + (BO * parameters[191])) + (BT * parameters[192])) + (BU * parameters[193]);
            let HT = ((((parameters[200] + (BM * parameters[201])) + (BN * parameters[202])) + (BO * parameters[203])) + (BT * parameters[204])) + (BU * parameters[205]);
            let HU = ((((parameters[206] + (BM * parameters[207])) + (BN * parameters[208])) + (BO * parameters[209])) + (BT * parameters[210])) + (BU * parameters[211]);
            let HV = ((((parameters[218] + (BM * parameters[219])) + (BN * parameters[220])) + (BO * parameters[221])) + (BT * parameters[222])) + (BU * parameters[223]);
            let HW = ((((parameters[224] + (BM * parameters[225])) + (BN * parameters[226])) + (BO * parameters[227])) + (BT * parameters[228])) + (BU * parameters[229]);
            let HX = ((((parameters[230] + (BM * parameters[231])) + (BN * parameters[232])) + (BO * parameters[233])) + (BT * parameters[234])) + (BU * parameters[235]);
            let HY = ((((parameters[236] + (BM * parameters[237])) + (BN * parameters[238])) + (BO * parameters[239])) + (BT * parameters[240])) + (BU * parameters[241]);
            let HZ = ((((parameters[242] + (BM * parameters[243])) + (BN * parameters[244])) + (BO * parameters[245])) + (BT * parameters[246])) + (BU * parameters[247]);
            let IA = ((((parameters[248] + (BM * parameters[249])) + (BN * parameters[250])) + (BO * parameters[251])) + (BT * parameters[252])) + (BU * parameters[253]);
            let IB = ((((parameters[266] + (BM * parameters[267])) + (BN * parameters[268])) + (BO * parameters[269])) + (BT * parameters[270])) + (BU * parameters[271]);
            let IC = ((((parameters[272] + (BM * parameters[273])) + (BN * parameters[274])) + (BO * parameters[275])) + (BT * parameters[276])) + (BU * parameters[277]);
            let ID = ((((parameters[278] + (BM * parameters[279])) + (BN * parameters[280])) + (BO * parameters[281])) + (BT * parameters[282])) + (BU * parameters[283]);
            let IE = ((((parameters[284] + (BM * parameters[285])) + (BN * parameters[286])) + (BO * parameters[287])) + (BT * parameters[288])) + (BU * parameters[289]);
            let IF = ((((parameters[296] + (BM * parameters[297])) + (BN * parameters[298])) + (BO * parameters[299])) + (BT * parameters[300])) + (BU * parameters[301]);
            let IG = ((((parameters[302] + (BM * parameters[303])) + (BN * parameters[304])) + (BO * parameters[305])) + (BT * parameters[306])) + (BU * parameters[307]);
            let IH = ((((parameters[308] + (BM * parameters[309])) + (BN * parameters[310])) + (BO * parameters[311])) + (BT * parameters[312])) + (BU * parameters[313]);
            let II = ((((parameters[314] + (BM * parameters[315])) + (BN * parameters[316])) + (BO * parameters[317])) + (BT * parameters[318])) + (BU * parameters[319]);
            let IJ = ((((parameters[320] + (BM * parameters[321])) + (BN * parameters[322])) + (BO * parameters[323])) + (BT * parameters[324])) + (BU * parameters[325]);
            let IK = ((((parameters[326] + (BM * parameters[327])) + (BN * parameters[328])) + (BO * parameters[329])) + (BT * parameters[330])) + (BU * parameters[331]);
            let IL = ((((parameters[332] + (BM * parameters[333])) + (BN * parameters[334])) + (BO * parameters[335])) + (BT * parameters[336])) + (BU * parameters[337]);
            let IM = ((((parameters[338] + (BM * parameters[339])) + (BN * parameters[340])) + (BO * parameters[341])) + (BT * parameters[342])) + (BU * parameters[343]);
            let IN = ((((parameters[344] + (BM * parameters[345])) + (BN * parameters[346])) + (BO * parameters[347])) + (BT * parameters[348])) + (BU * parameters[349]);
            let IO = ((((parameters[350] + (BM * parameters[351])) + (BN * parameters[352])) + (BO * parameters[353])) + (BT * parameters[354])) + (BU * parameters[355]);
            let IP = ((((parameters[403] + (BM * parameters[404])) + (BN * parameters[405])) + (BO * parameters[406])) + (BT * parameters[407])) + (BU * parameters[408]);
            let IQ = ((((parameters[409] + (BM * parameters[410])) + (BN * parameters[411])) + (BO * parameters[412])) + (BT * parameters[413])) + (BU * parameters[414]);
            let IR = ((((parameters[415] + (BM * parameters[416])) + (BN * parameters[417])) + (BO * parameters[418])) + (BT * parameters[419])) + (BU * parameters[420]);
            let IS = ((((parameters[421] + (BM * parameters[422])) + (BN * parameters[423])) + (BO * parameters[424])) + (BT * parameters[425])) + (BU * parameters[426]);
            let IT = ((((parameters[455] + (BM * parameters[456])) + (BN * parameters[457])) + (BO * parameters[458])) + (BT * parameters[459])) + (BU * parameters[460]);
            let IU = ((((parameters[467] + (BM * parameters[468])) + (BN * parameters[469])) + (BO * parameters[470])) + (BT * parameters[471])) + (BU * parameters[472]);
            let IV = ((((parameters[506] + (BM * parameters[507])) + (BN * parameters[508])) + (BO * parameters[509])) + (BT * parameters[510])) + (BU * parameters[511]);
            let IW = ((((parameters[512] + (BM * parameters[513])) + (BN * parameters[514])) + (BO * parameters[515])) + (BT * parameters[516])) + (BU * parameters[517]);
            let IX = ((((parameters[479] + (BM * parameters[480])) + (BN * parameters[481])) + (BO * parameters[482])) + (BT * parameters[483])) + (BU * parameters[484]);
            let IY = ((((parameters[485] + (BM * parameters[486])) + (BN * parameters[487])) + (BO * parameters[488])) + (BT * parameters[489])) + (BU * parameters[490]);
            let IZ = ((((parameters[518] + (BM * parameters[519])) + (BN * parameters[520])) + (BO * parameters[521])) + (BT * parameters[522])) + (BU * parameters[523]);
            let JA = ((((parameters[524] + (BM * parameters[525])) + (BN * parameters[526])) + (BO * parameters[527])) + (BT * parameters[528])) + (BU * parameters[529]);
            let JB = ((((parameters[492] + (BM * parameters[493])) + (BN * parameters[494])) + (BO * parameters[495])) + (BT * parameters[496])) + (BU * parameters[497]);
            let JC = ((((parameters[531] + (BM * parameters[532])) + (BN * parameters[533])) + (BO * parameters[534])) + (BT * parameters[535])) + (BU * parameters[536]);
            let JD = ((((parameters[543] + (BM * parameters[544])) + (BN * parameters[545])) + (BO * parameters[546])) + (BT * parameters[547])) + (BU * parameters[548]);
            let JE = ((((parameters[605] + (BM * parameters[606])) + (BN * parameters[607])) + (BO * parameters[608])) + (BT * parameters[609])) + (BU * parameters[610]);
            let JF = ((((parameters[623] + (BM * parameters[624])) + (BN * parameters[625])) + (BO * parameters[626])) + (BT * parameters[627])) + (BU * parameters[628]);
            let JG = ((((parameters[629] + (BM * parameters[630])) + (BN * parameters[631])) + (BO * parameters[632])) + (BT * parameters[633])) + (BU * parameters[634]);
            let JH = ((((parameters[641] + (BM * parameters[642])) + (BN * parameters[643])) + (BO * parameters[644])) + (BT * parameters[645])) + (BU * parameters[646]);
            let JI = ((((parameters[677] + (BM * parameters[678])) + (BN * parameters[679])) + (BO * parameters[680])) + (BT * parameters[681])) + (BU * parameters[682]);
            let JJ = ((((parameters[689] + (BM * parameters[690])) + (BN * parameters[691])) + (BO * parameters[692])) + (BT * parameters[693])) + (BU * parameters[694]);
            let JK = ((((parameters[707] + (BM * parameters[708])) + (BN * parameters[709])) + (BO * parameters[710])) + (BT * parameters[711])) + (BU * parameters[712]);
            let JL = ((((parameters[713] + (BM * parameters[714])) + (BN * parameters[715])) + (BO * parameters[716])) + (BT * parameters[717])) + (BU * parameters[718]);
            let JM = ((((parameters[719] + (BM * parameters[720])) + (BN * parameters[721])) + (BO * parameters[722])) + (BT * parameters[723])) + (BU * parameters[724]);
            let JN = ((((parameters[725] + (BM * parameters[726])) + (BN * parameters[727])) + (BO * parameters[728])) + (BT * parameters[729])) + (BU * parameters[730]);
            let JO = ((((parameters[731] + (BM * parameters[732])) + (BN * parameters[733])) + (BO * parameters[734])) + (BT * parameters[735])) + (BU * parameters[736]);
            let JP = ((((parameters[1025] + (BM * parameters[1027])) + (BN * parameters[1028])) + (BO * parameters[1029])) + (BT * parameters[1030])) + (BU * parameters[1031]);
            let JQ = ((((parameters[1038] + (BM * parameters[1039])) + (BN * parameters[1040])) + (BO * parameters[1041])) + (BT * parameters[1042])) + (BU * parameters[1043]);
            let JR = ((((parameters[1044] + (BM * parameters[1045])) + (BN * parameters[1046])) + (BO * parameters[1047])) + (BT * parameters[1048])) + (BU * parameters[1049]);
            let JS = ((((parameters[1050] + (BM * parameters[1051])) + (BN * parameters[1052])) + (BO * parameters[1053])) + (BT * parameters[1054])) + (BU * parameters[1055]);
            let JT = ((((parameters[1056] + (BM * parameters[1057])) + (BN * parameters[1058])) + (BO * parameters[1059])) + (BT * parameters[1060])) + (BU * parameters[1061]);
            let JU = ((((parameters[1062] + (BM * parameters[1063])) + (BN * parameters[1064])) + (BO * parameters[1065])) + (BT * parameters[1066])) + (BU * parameters[1067]);
            let JV = ((((parameters[1068] + (BM * parameters[1069])) + (BN * parameters[1070])) + (BO * parameters[1071])) + (BT * parameters[1072])) + (BU * parameters[1073]);
            let JW = ((((parameters[925] + (BM * parameters[926])) + (BN * parameters[927])) + (BO * parameters[928])) + (BT * parameters[929])) + (BU * parameters[930]);
            let JX = ((((parameters[931] + (BM * parameters[932])) + (BN * parameters[933])) + (BO * parameters[934])) + (BT * parameters[935])) + (BU * parameters[936]);
            let JY = ((((parameters[937] + (BM * parameters[938])) + (BN * parameters[939])) + (BO * parameters[940])) + (BT * parameters[941])) + (BU * parameters[942]);
            let JZ = ((((parameters[949] + (BM * parameters[950])) + (BN * parameters[951])) + (BO * parameters[952])) + (BT * parameters[953])) + (BU * parameters[954]);
            let KA = ((((parameters[943] + (BM * parameters[944])) + (BN * parameters[945])) + (BO * parameters[946])) + (BT * parameters[947])) + (BU * parameters[948]);
            let KB = ((((parameters[955] + (BM * parameters[956])) + (BN * parameters[957])) + (BO * parameters[958])) + (BT * parameters[959])) + (BU * parameters[960]);
            let KC = ((((parameters[985] + (BM * parameters[986])) + (BN * parameters[987])) + (BO * parameters[988])) + (BT * parameters[989])) + (BU * parameters[990]);
            let KD = ((((parameters[991] + (BM * parameters[992])) + (BN * parameters[993])) + (BO * parameters[994])) + (BT * parameters[995])) + (BU * parameters[996]);
            let KE = ((((parameters[1009] + (BM * parameters[1010])) + (BN * parameters[1011])) + (BO * parameters[1012])) + (BT * parameters[1013])) + (BU * parameters[1014]);
            let KF = ((((parameters[1015] + (BM * parameters[1016])) + (BN * parameters[1017])) + (BO * parameters[1018])) + (BT * parameters[1019])) + (BU * parameters[1020]);
            let KG = ((((parameters[1119] + (BM * parameters[1120])) + (BN * parameters[1121])) + (BO * parameters[1122])) + (BT * parameters[1123])) + (BU * parameters[1124]);
            let KH = ((((parameters[1125] + (BM * parameters[1126])) + (BN * parameters[1127])) + (BO * parameters[1128])) + (BT * parameters[1129])) + (BU * parameters[1130]);
            let KI = ((((parameters[1131] + (BM * parameters[1132])) + (BN * parameters[1133])) + (BO * parameters[1134])) + (BT * parameters[1135])) + (BU * parameters[1136]);
            let KJ = ((((parameters[1137] + (BM * parameters[1138])) + (BN * parameters[1139])) + (BO * parameters[1140])) + (BT * parameters[1141])) + (BU * parameters[1142]);
            let KK = ((((parameters[1143] + (BM * parameters[1144])) + (BN * parameters[1145])) + (BO * parameters[1146])) + (BT * parameters[1147])) + (BU * parameters[1148]);
            let KL = ((((parameters[1149] + (BM * parameters[1150])) + (BN * parameters[1151])) + (BO * parameters[1152])) + (BT * parameters[1153])) + (BU * parameters[1154]);
            let KM = ((((parameters[1155] + (BM * parameters[1156])) + (BN * parameters[1157])) + (BO * parameters[1158])) + (BT * parameters[1159])) + (BU * parameters[1160]);
            let KN = ((((parameters[1161] + (BM * parameters[1162])) + (BN * parameters[1163])) + (BO * parameters[1164])) + (BT * parameters[1165])) + (BU * parameters[1166]);
            let KO = ((((parameters[1167] + (BM * parameters[1168])) + (BN * parameters[1169])) + (BO * parameters[1170])) + (BT * parameters[1171])) + (BU * parameters[1172]);
            let KP = ((((parameters[1173] + (BM * parameters[1174])) + (BN * parameters[1175])) + (BO * parameters[1176])) + (BT * parameters[1177])) + (BU * parameters[1178]);
            let KQ = ((((parameters[1179] + (BM * parameters[1180])) + (BN * parameters[1181])) + (BO * parameters[1182])) + (BT * parameters[1183])) + (BU * parameters[1184]);
            let KR = ((((parameters[1185] + (BM * parameters[1186])) + (BN * parameters[1187])) + (BO * parameters[1188])) + (BT * parameters[1189])) + (BU * parameters[1190]);
            let KS = ((((parameters[1191] + (BM * parameters[1192])) + (BN * parameters[1193])) + (BO * parameters[1194])) + (BT * parameters[1195])) + (BU * parameters[1196]);
            let KT = ((((parameters[1197] + (BM * parameters[1198])) + (BN * parameters[1199])) + (BO * parameters[1200])) + (BT * parameters[1201])) + (BU * parameters[1202]);
            let KU = ((((parameters[1203] + (BM * parameters[1204])) + (BN * parameters[1205])) + (BO * parameters[1206])) + (BT * parameters[1207])) + (BU * parameters[1208]);
            let KV = ((((parameters[1209] + (BM * parameters[1210])) + (BN * parameters[1211])) + (BO * parameters[1212])) + (BT * parameters[1213])) + (BU * parameters[1214]);
            let KW = ((((parameters[1215] + (BM * parameters[1216])) + (BN * parameters[1217])) + (BO * parameters[1218])) + (BT * parameters[1219])) + (BU * parameters[1220]);
            let KX = ((((parameters[1221] + (BM * parameters[1222])) + (BN * parameters[1223])) + (BO * parameters[1224])) + (BT * parameters[1225])) + (BU * parameters[1226]);
            let KY = ((((parameters[1227] + (BM * parameters[1228])) + (BN * parameters[1229])) + (BO * parameters[1230])) + (BT * parameters[1231])) + (BU * parameters[1232]);
            let KZ = ((((parameters[1233] + (BM * parameters[1234])) + (BN * parameters[1235])) + (BO * parameters[1236])) + (BT * parameters[1237])) + (BU * parameters[1238]);
            let LA = ((((parameters[1239] + (BM * parameters[1240])) + (BN * parameters[1241])) + (BO * parameters[1242])) + (BT * parameters[1243])) + (BU * parameters[1244]);
            let LB = ((((parameters[1245] + (BM * parameters[1246])) + (BN * parameters[1247])) + (BO * parameters[1248])) + (BT * parameters[1249])) + (BU * parameters[1250]);
            let LC = ((((parameters[1251] + (BM * parameters[1252])) + (BN * parameters[1253])) + (BO * parameters[1254])) + (BT * parameters[1255])) + (BU * parameters[1256]);
            let LD = ((((parameters[1257] + (BM * parameters[1258])) + (BN * parameters[1259])) + (BO * parameters[1260])) + (BT * parameters[1261])) + (BU * parameters[1262]);
            let LE = ((((parameters[1113] + (BM * parameters[1114])) + (BN * parameters[1115])) + (BO * parameters[1116])) + (BT * parameters[1117])) + (BU * parameters[1118]);
            let LF = ((((parameters[1263] + (BM * parameters[1264])) + (BN * parameters[1265])) + (BO * parameters[1266])) + (BT * parameters[1267])) + (BU * parameters[1268]);
            let LG = ((((parameters[1269] + (BM * parameters[1270])) + (BN * parameters[1271])) + (BO * parameters[1272])) + (BT * parameters[1273])) + (BU * parameters[1274]);
            let LH = ((((parameters[1275] + (BM * parameters[1276])) + (BN * parameters[1277])) + (BO * parameters[1278])) + (BT * parameters[1279])) + (BU * parameters[1280]);
            let LI = ((((parameters[1281] + (BM * parameters[1282])) + (BN * parameters[1283])) + (BO * parameters[1284])) + (BT * parameters[1285])) + (BU * parameters[1286]);
            let LJ = ((((parameters[1287] + (BM * parameters[1288])) + (BN * parameters[1289])) + (BO * parameters[1290])) + (BT * parameters[1291])) + (BU * parameters[1292]);
            let LK = ((((parameters[1329] + (BM * parameters[1330])) + (BN * parameters[1331])) + (BO * parameters[1332])) + (BT * parameters[1333])) + (BU * parameters[1334]);
            let LL = ((((parameters[1335] + (BM * parameters[1336])) + (BN * parameters[1337])) + (BO * parameters[1338])) + (BT * parameters[1339])) + (BU * parameters[1340]);
            let LM = ((((parameters[1341] + (BM * parameters[1342])) + (BN * parameters[1343])) + (BO * parameters[1344])) + (BT * parameters[1345])) + (BU * parameters[1346]);
            let LN = ((((parameters[1347] + (BM * parameters[1348])) + (BN * parameters[1349])) + (BO * parameters[1350])) + (BT * parameters[1351])) + (BU * parameters[1352]);
            let LO = ((((parameters[1299] + (BM * parameters[1300])) + (BN * parameters[1301])) + (BO * parameters[1302])) + (BT * parameters[1303])) + (BU * parameters[1304]);
            let LP = ((((parameters[1305] + (BM * parameters[1306])) + (BN * parameters[1307])) + (BO * parameters[1308])) + (BT * parameters[1309])) + (BU * parameters[1310]);
            let LQ = ((((parameters[1311] + (BM * parameters[1312])) + (BN * parameters[1313])) + (BO * parameters[1314])) + (BT * parameters[1315])) + (BU * parameters[1316]);
            let LR = ((((parameters[1317] + (BM * parameters[1318])) + (BN * parameters[1319])) + (BO * parameters[1320])) + (BT * parameters[1321])) + (BU * parameters[1322]);
            let LS = ((((parameters[1353] + (BM * parameters[1354])) + (BN * parameters[1355])) + (BO * parameters[1356])) + (BT * parameters[1357])) + (BU * parameters[1358]);
            let LT = ((((parameters[1359] + (BM * parameters[1360])) + (BN * parameters[1361])) + (BO * parameters[1362])) + (BT * parameters[1363])) + (BU * parameters[1364]);
            let LU = ((((parameters[1365] + (BM * parameters[1366])) + (BN * parameters[1367])) + (BO * parameters[1368])) + (BT * parameters[1369])) + (BU * parameters[1370]);
            let LV = ((((parameters[1371] + (BM * parameters[1372])) + (BN * parameters[1373])) + (BO * parameters[1374])) + (BT * parameters[1375])) + (BU * parameters[1376]);
            let LW = ((((parameters[1444] + (BM * parameters[1445])) + (BN * parameters[1446])) + (BO * parameters[1447])) + (BT * parameters[1448])) + (BU * parameters[1449]);
            let LX = ((((parameters[1450] + (BM * parameters[1451])) + (BN * parameters[1452])) + (BO * parameters[1453])) + (BT * parameters[1454])) + (BU * parameters[1455]);
            let LY = ((((parameters[1462] + (BM * parameters[1463])) + (BN * parameters[1464])) + (BO * parameters[1465])) + (BT * parameters[1466])) + (BU * parameters[1467]);
            let LZ = ((((parameters[1468] + (BM * parameters[1469])) + (BN * parameters[1470])) + (BO * parameters[1471])) + (BT * parameters[1472])) + (BU * parameters[1473]);
            let MA = ((((parameters[1456] + (BM * parameters[1457])) + (BN * parameters[1458])) + (BO * parameters[1459])) + (BT * parameters[1460])) + (BU * parameters[1461]);
            let MB = ((((parameters[1474] + (BM * parameters[1475])) + (BN * parameters[1476])) + (BO * parameters[1477])) + (BT * parameters[1478])) + (BU * parameters[1479]);
            let MC = ((((parameters[1480] + (BM * parameters[1481])) + (BN * parameters[1482])) + (BO * parameters[1483])) + (BT * parameters[1484])) + (BU * parameters[1485]);
            let MD = ((((parameters[1486] + (BM * parameters[1487])) + (BN * parameters[1488])) + (BO * parameters[1489])) + (BT * parameters[1490])) + (BU * parameters[1491]);
            let ME = ((((parameters[1492] + (BM * parameters[1493])) + (BN * parameters[1494])) + (BO * parameters[1495])) + (BT * parameters[1496])) + (BU * parameters[1497]);
            let MF = ((((parameters[1498] + (BM * parameters[1499])) + (BN * parameters[1500])) + (BO * parameters[1501])) + (BT * parameters[1502])) + (BU * parameters[1503]);
            let MG = ((((parameters[1510] + (BM * parameters[1511])) + (BN * parameters[1512])) + (BO * parameters[1513])) + (BT * parameters[1514])) + (BU * parameters[1515]);
            let MH = ((((parameters[1516] + (BM * parameters[1517])) + (BN * parameters[1518])) + (BO * parameters[1519])) + (BT * parameters[1520])) + (BU * parameters[1521]);
            let MI = ((((parameters[1522] + (BM * parameters[1523])) + (BN * parameters[1524])) + (BO * parameters[1525])) + (BT * parameters[1526])) + (BU * parameters[1527]);
            let MJ = ((((parameters[1762] + (BM * parameters[1763])) + (BN * parameters[1764])) + (BO * parameters[1765])) + (BT * parameters[1766])) + (BU * parameters[1767]);
            let MK = ((((parameters[1530] + (BM * parameters[1531])) + (BN * parameters[1532])) + (BO * parameters[1533])) + (BT * parameters[1534])) + (BU * parameters[1535]);
            let ML = ((((parameters[1536] + (BM * parameters[1537])) + (BN * parameters[1538])) + (BO * parameters[1539])) + (BT * parameters[1540])) + (BU * parameters[1541]);
            let MM = ((((parameters[28] + (BM * parameters[29])) + (BN * parameters[30])) + (BO * parameters[31])) + (BT * parameters[32])) + (BU * parameters[33]);
            let MN = ((((parameters[34] + (BM * parameters[35])) + (BN * parameters[36])) + (BO * parameters[37])) + (BT * parameters[38])) + (BU * parameters[39]);
            let MO = ((((parameters[1547] + (BM * parameters[1548])) + (BN * parameters[1549])) + (BO * parameters[1550])) + (BT * parameters[1551])) + (BU * parameters[1552]);
            let MP = ((((parameters[1553] + (BM * parameters[1554])) + (BN * parameters[1555])) + (BO * parameters[1556])) + (BT * parameters[1557])) + (BU * parameters[1558]);
            let MQ = ((((parameters[1559] + (BM * parameters[1560])) + (BN * parameters[1561])) + (BO * parameters[1562])) + (BT * parameters[1563])) + (BU * parameters[1564]);
            let MR = ((((parameters[1565] + (BM * parameters[1566])) + (BN * parameters[1567])) + (BO * parameters[1568])) + (BT * parameters[1569])) + (BU * parameters[1570]);
            let MS = ((((parameters[1571] + (BM * parameters[1572])) + (BN * parameters[1573])) + (BO * parameters[1574])) + (BT * parameters[1575])) + (BU * parameters[1576]);
            let MT = ((((parameters[1577] + (BM * parameters[1578])) + (BN * parameters[1579])) + (BO * parameters[1580])) + (BT * parameters[1581])) + (BU * parameters[1582]);
            let MU = ((((parameters[1650] + (BM * parameters[1651])) + (BN * parameters[1652])) + (BO * parameters[1653])) + (BT * parameters[1654])) + (BU * parameters[1655]);
            let MV = ((((parameters[737] + (BM * parameters[738])) + (BN * parameters[739])) + (BO * parameters[740])) + (BT * parameters[741])) + (BU * parameters[742]);
            let MW = ((((parameters[755] + (BM * parameters[756])) + (BN * parameters[757])) + (BO * parameters[758])) + (BT * parameters[759])) + (BU * parameters[760]);
            let MX = ((((parameters[767] + (BM * parameters[768])) + (BN * parameters[769])) + (BO * parameters[770])) + (BT * parameters[771])) + (BU * parameters[772]);
            let MY = ((((parameters[785] + (BM * parameters[786])) + (BN * parameters[787])) + (BO * parameters[788])) + (BT * parameters[789])) + (BU * parameters[790]);
            let MZ = ((((parameters[791] + (BM * parameters[792])) + (BN * parameters[793])) + (BO * parameters[794])) + (BT * parameters[795])) + (BU * parameters[796]);
            let NA = ((((parameters[809] + (BM * parameters[810])) + (BN * parameters[811])) + (BO * parameters[812])) + (BT * parameters[813])) + (BU * parameters[814]);
            let NB = ((((parameters[821] + (BM * parameters[822])) + (BN * parameters[823])) + (BO * parameters[824])) + (BT * parameters[825])) + (BU * parameters[826]);
            let NC = ((((parameters[845] + (BM * parameters[846])) + (BN * parameters[847])) + (BO * parameters[848])) + (BT * parameters[849])) + (BU * parameters[850]);
            let ND = ((((parameters[863] + (BM * parameters[864])) + (BN * parameters[865])) + (BO * parameters[866])) + (BT * parameters[867])) + (BU * parameters[868]);
            let NE = ((((parameters[875] + (BM * parameters[876])) + (BN * parameters[877])) + (BO * parameters[878])) + (BT * parameters[879])) + (BU * parameters[880]);
            let NG = ((((NF + (BM * parameters[882])) + (BN * parameters[883])) + (BO * parameters[884])) + (BT * parameters[885])) + (BU * parameters[886]);
            let NH = ((((parameters[575] + (BM * parameters[576])) + (BN * parameters[577])) + (BO * parameters[578])) + (BT * parameters[579])) + (BU * parameters[580]);
            let NI = ((((parameters[555] + (BM * parameters[556])) + (BN * parameters[557])) + (BO * parameters[558])) + (BT * parameters[559])) + (BU * parameters[560]);
            let NJ = ((((parameters[568] + (BM * parameters[569])) + (BN * parameters[570])) + (BO * parameters[571])) + (BT * parameters[572])) + (BU * parameters[573]);
            let NK = ((((parameters[961] + (BM * parameters[962])) + (BN * parameters[963])) + (BO * parameters[964])) + (BT * parameters[965])) + (BU * parameters[966]);
            let NL = ((((parameters[967] + (BM * parameters[968])) + (BN * parameters[969])) + (BO * parameters[970])) + (BT * parameters[971])) + (BU * parameters[972]);
            let NM = ((((parameters[973] + (BM * parameters[974])) + (BN * parameters[975])) + (BO * parameters[976])) + (BT * parameters[977])) + (BU * parameters[978]);
            let NN = ((((parameters[979] + (BM * parameters[980])) + (BN * parameters[981])) + (BO * parameters[982])) + (BT * parameters[983])) + (BU * parameters[984]);
            let NO = ((((parameters[1741] + (BM * parameters[1742])) + (BN * parameters[1743])) + (BO * parameters[1744])) + (BT * parameters[1745])) + (BU * parameters[1746]);
            let NP = ((((parameters[1750] + (BM * parameters[1751])) + (BN * parameters[1752])) + (BO * parameters[1753])) + (BT * parameters[1754])) + (BU * parameters[1755]);
            let NQ = ((((parameters[1756] + (BM * parameters[1757])) + (BN * parameters[1758])) + (BO * parameters[1759])) + (BT * parameters[1760])) + (BU * parameters[1761]);
            let NR = ((((parameters[1768] + (BM * parameters[1769])) + (BN * parameters[1770])) + (BO * parameters[1771])) + (BT * parameters[1772])) + (BU * parameters[1773]);
            let NS = ((((parameters[1774] + (BM * parameters[1775])) + (BN * parameters[1776])) + (BO * parameters[1777])) + (BT * parameters[1778])) + (BU * parameters[1779]);
            let NT = ((((parameters[1780] + (BM * parameters[1781])) + (BN * parameters[1782])) + (BO * parameters[1783])) + (BT * parameters[1784])) + (BU * parameters[1785]);
            let NU = ((((parameters[176] + (BM * parameters[177])) + (BN * parameters[178])) + (BO * parameters[179])) + (BT * parameters[180])) + (BU * parameters[181]);
            let NV = ((((parameters[182] + (BM * parameters[183])) + (BN * parameters[184])) + (BO * parameters[185])) + (BT * parameters[186])) + (BU * parameters[187]);
            let NW = ((((parameters[1689] + (BM * parameters[1690])) + (BN * parameters[1691])) + (BO * parameters[1692])) + (BT * parameters[1693])) + (BU * parameters[1694]);
            let NX = ((((parameters[1701] + (BM * parameters[1702])) + (BN * parameters[1703])) + (BO * parameters[1704])) + (BT * parameters[1705])) + (BU * parameters[1706]);
            let NY = ((((parameters[1695] + (BM * parameters[1696])) + (BN * parameters[1697])) + (BO * parameters[1698])) + (BT * parameters[1699])) + (BU * parameters[1700]);
            let XZ;
            let ZI;
            let CHJ;
            let CHL;
            let COL;
            let COP;
            let COR;
            let COW;
            let COZ;
            let FGS;
            let FGW;
            let FGZ;
            let FHI;
            let FIO;
            let FIS;
            let FIV;
            let FJE;
            if AS != 0.0 {
                let NZ = ((((parameters[356] + (BM * parameters[357])) + (BN * parameters[358])) + (BO * parameters[359])) + (BT * parameters[360])) + (BU * parameters[361]);
                let OA = ((((parameters[362] + (BM * parameters[363])) + (BN * parameters[364])) + (BO * parameters[365])) + (BT * parameters[366])) + (BU * parameters[367]);
                let OB = ((((parameters[368] + (BM * parameters[369])) + (BN * parameters[370])) + (BO * parameters[371])) + (BT * parameters[372])) + (BU * parameters[373]);
                let OC = ((((parameters[659] + (BM * parameters[660])) + (BN * parameters[661])) + (BO * parameters[662])) + (BT * parameters[663])) + (BU * parameters[664]);
                let OD = ((((parameters[827] + (BM * parameters[828])) + (BN * parameters[829])) + (BO * parameters[830])) + (BT * parameters[831])) + (BU * parameters[832]);
                let OE = if AR == R { 1.0 } else { 0.0 };
                let COQ;
                let COS;
                let COX;
                let CPA;
                if OE != 0.0 {
                    let OF = ((((parameters[386] + (BM * parameters[387])) + (BN * parameters[388])) + (BO * parameters[389])) + (BT * parameters[390])) + (BU * parameters[391]);
                    let OG = ((((parameters[392] + (BM * parameters[393])) + (BN * parameters[394])) + (BO * parameters[395])) + (BT * parameters[396])) + (BU * parameters[397]);
                    let OH = ((((parameters[374] + (BM * parameters[375])) + (BN * parameters[376])) + (BO * parameters[377])) + (BT * parameters[378])) + (BU * parameters[379]);
                    let OI = ((((parameters[380] + (BM * parameters[381])) + (BN * parameters[382])) + (BO * parameters[383])) + (BT * parameters[384])) + (BU * parameters[385]);
                    COQ = OH;
                    COS = OI;
                    COX = OF;
                    CPA = OG;
                } else {
                    COQ = A;
                    COS = A;
                    COX = A;
                    CPA = A;
                }
                let OK = if (if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 || (if OJ == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let FGT;
                let FGX;
                let FHA;
                let FHJ;
                let FIP;
                let FIT;
                let FIW;
                let FJF;
                if OK != 0.0 {
                    let OL = ((((parameters[1377] + (BM * parameters[1378])) + (BN * parameters[1379])) + (BO * parameters[1380])) + (BT * parameters[1381])) + (BU * parameters[1382]);
                    let OM = ((((parameters[1383] + (BM * parameters[1384])) + (BN * parameters[1385])) + (BO * parameters[1386])) + (BT * parameters[1387])) + (BU * parameters[1388]);
                    let ON = ((((parameters[1389] + (BM * parameters[1390])) + (BN * parameters[1391])) + (BO * parameters[1392])) + (BT * parameters[1393])) + (BU * parameters[1394]);
                    let OO = ((((parameters[1395] + (BM * parameters[1396])) + (BN * parameters[1397])) + (BO * parameters[1398])) + (BT * parameters[1399])) + (BU * parameters[1400]);
                    let OP = ((((parameters[1407] + (BM * parameters[1408])) + (BN * parameters[1409])) + (BO * parameters[1410])) + (BT * parameters[1411])) + (BU * parameters[1412]);
                    let OQ = ((((parameters[1413] + (BM * parameters[1414])) + (BN * parameters[1415])) + (BO * parameters[1416])) + (BT * parameters[1417])) + (BU * parameters[1418]);
                    let OR = ((((parameters[1419] + (BM * parameters[1420])) + (BN * parameters[1421])) + (BO * parameters[1422])) + (BT * parameters[1423])) + (BU * parameters[1424]);
                    let OS = ((((parameters[1425] + (BM * parameters[1426])) + (BN * parameters[1427])) + (BO * parameters[1428])) + (BT * parameters[1429])) + (BU * parameters[1430]);
                    FGT = OM;
                    FGX = OL;
                    FHA = OO;
                    FHJ = ON;
                    FIP = OQ;
                    FIT = OP;
                    FIW = OS;
                    FJF = OR;
                } else {
                    FGT = A;
                    FGX = A;
                    FHA = A;
                    FHJ = A;
                    FIP = A;
                    FIT = A;
                    FIW = A;
                    FJF = A;
                }
                XZ = OA;
                ZI = NZ;
                CHJ = OC;
                CHL = OD;
                COL = OB;
                COP = COQ;
                COR = COS;
                COW = COX;
                COZ = CPA;
                FGS = FGT;
                FGW = FGX;
                FGZ = FHA;
                FHI = FHJ;
                FIO = FIP;
                FIS = FIT;
                FIV = FIW;
                FJE = FJF;
            } else {
                XZ = A;
                ZI = A;
                CHJ = A;
                CHL = A;
                COL = A;
                COP = A;
                COR = A;
                COW = A;
                COZ = A;
                FGS = A;
                FGW = A;
                FGZ = A;
                FHI = A;
                FIO = A;
                FIS = A;
                FIV = A;
                FJE = A;
            }
            let OU = if OT != A { 1.0 } else { 0.0 };
            let TI;
            let UH;
            let UY;
            let VW;
            let VY;
            let WA;
            let WF;
            let WO;
            let WR;
            let WV;
            let ZD;
            let AAH;
            let AAJ;
            let BCA;
            let BCE;
            let BCX;
            let BDG;
            let BEI;
            let BEK;
            let CIL;
            let CIO;
            let DIS;
            let DIY;
            let DJA;
            if OU != 0.0 {
                let OV = ((((parameters[212] + (BM * parameters[213])) + (BN * parameters[214])) + (BO * parameters[215])) + (BT * parameters[216])) + (BU * parameters[217]);
                let OW = ((((parameters[194] + (BM * parameters[195])) + (BN * parameters[196])) + (BO * parameters[197])) + (BT * parameters[198])) + (BU * parameters[199]);
                let OX = ((((parameters[254] + (BM * parameters[255])) + (BN * parameters[256])) + (BO * parameters[257])) + (BT * parameters[258])) + (BU * parameters[259]);
                let OY = ((((parameters[473] + (BM * parameters[474])) + (BN * parameters[475])) + (BO * parameters[476])) + (BT * parameters[477])) + (BU * parameters[478]);
                let OZ = ((((parameters[537] + (BM * parameters[538])) + (BN * parameters[539])) + (BO * parameters[540])) + (BT * parameters[541])) + (BU * parameters[542]);
                let PA = ((((parameters[549] + (BM * parameters[550])) + (BN * parameters[551])) + (BO * parameters[552])) + (BT * parameters[553])) + (BU * parameters[554]);
                let PB = ((((parameters[997] + (BM * parameters[998])) + (BN * parameters[999])) + (BO * parameters[1000])) + (BT * parameters[1001])) + (BU * parameters[1002]);
                let PC = ((((parameters[1003] + (BM * parameters[1004])) + (BN * parameters[1005])) + (BO * parameters[1006])) + (BT * parameters[1007])) + (BU * parameters[1008]);
                let PD = ((((parameters[1032] + (BM * parameters[1033])) + (BN * parameters[1034])) + (BO * parameters[1035])) + (BT * parameters[1036])) + (BU * parameters[1037]);
                let PE = ((((parameters[290] + (BM * parameters[291])) + (BN * parameters[292])) + (BO * parameters[293])) + (BT * parameters[294])) + (BU * parameters[295]);
                let PF = ((((parameters[461] + (BM * parameters[462])) + (BN * parameters[463])) + (BO * parameters[464])) + (BT * parameters[465])) + (BU * parameters[466]);
                let PG = ((((parameters[500] + (BM * parameters[501])) + (BN * parameters[502])) + (BO * parameters[503])) + (BT * parameters[504])) + (BU * parameters[505]);
                let PH = ((((parameters[611] + (BM * parameters[612])) + (BN * parameters[613])) + (BO * parameters[614])) + (BT * parameters[615])) + (BU * parameters[616]);
                let PI = ((((parameters[647] + (BM * parameters[648])) + (BN * parameters[649])) + (BO * parameters[650])) + (BT * parameters[651])) + (BU * parameters[652]);
                let PJ = ((((parameters[635] + (BM * parameters[636])) + (BN * parameters[637])) + (BO * parameters[638])) + (BT * parameters[639])) + (BU * parameters[640]);
                let PK = ((((parameters[683] + (BM * parameters[684])) + (BN * parameters[685])) + (BO * parameters[686])) + (BT * parameters[687])) + (BU * parameters[688]);
                let PL = ((((parameters[695] + (BM * parameters[696])) + (BN * parameters[697])) + (BO * parameters[698])) + (BT * parameters[699])) + (BU * parameters[700]);
                let PM = ((((parameters[743] + (BM * parameters[744])) + (BN * parameters[745])) + (BO * parameters[746])) + (BT * parameters[747])) + (BU * parameters[748]);
                let PN = ((((parameters[773] + (BM * parameters[774])) + (BN * parameters[775])) + (BO * parameters[776])) + (BT * parameters[777])) + (BU * parameters[778]);
                let PO = ((((parameters[797] + (BM * parameters[798])) + (BN * parameters[799])) + (BO * parameters[800])) + (BT * parameters[801])) + (BU * parameters[802]);
                let PP = ((((parameters[851] + (BM * parameters[852])) + (BN * parameters[853])) + (BO * parameters[854])) + (BT * parameters[855])) + (BU * parameters[856]);
                let PQ = ((((parameters[562] + (BM * parameters[563])) + (BN * parameters[564])) + (BO * parameters[565])) + (BT * parameters[566])) + (BU * parameters[567]);
                let CIM;
                let CIP;
                if AS != 0.0 {
                    let PR = ((((parameters[665] + (BM * parameters[666])) + (BN * parameters[667])) + (BO * parameters[668])) + (BT * parameters[669])) + (BU * parameters[670]);
                    let PS = ((((parameters[833] + (BM * parameters[834])) + (BN * parameters[835])) + (BO * parameters[836])) + (BT * parameters[837])) + (BU * parameters[838]);
                    CIM = PR;
                    CIP = PS;
                } else {
                    CIM = A;
                    CIP = A;
                }
                TI = OV;
                UH = OY;
                UY = PH;
                VW = PI;
                VY = PL;
                WA = PK;
                WF = PJ;
                WO = PD;
                WR = OZ;
                WV = PA;
                ZD = OX;
                AAH = PB;
                AAJ = PC;
                BCA = PM;
                BCE = PN;
                BCX = PO;
                BDG = PP;
                BEI = PF;
                BEK = PQ;
                CIL = CIM;
                CIO = CIP;
                DIS = PG;
                DIY = PE;
                DJA = OW;
            } else {
                TI = A;
                UH = A;
                UY = A;
                VW = A;
                VY = A;
                WA = A;
                WF = A;
                WO = A;
                WR = A;
                WV = A;
                ZD = A;
                AAH = A;
                AAJ = A;
                BCA = A;
                BCE = A;
                BCX = A;
                BDG = A;
                BEI = A;
                BEK = A;
                CIL = A;
                CIO = A;
                DIS = A;
                DIY = A;
                DJA = A;
            }
            let PT = if parameters[67] == B { 1.0 } else { 0.0 };
            let BFA;
            let BFD;
            let BFG;
            let BFK;
            let BFN;
            let BFR;
            let BFT;
            let BKP;
            let BKU;
            let BKX;
            let CHW;
            let CIA;
            let CJR;
            if PT != 0.0 {
                let PU = ((((parameters[617] + (BM * parameters[618])) + (BN * parameters[619])) + (BO * parameters[620])) + (BT * parameters[621])) + (BU * parameters[622]);
                let PW = if PV != A { 1.0 } else { 0.0 };
                let QZ;
                if PW != 0.0 {
                    let PX = B + (U / parameters[585]);
                    let PY = if PX > BZ { 1.0 } else { 0.0 };
                    let QB = if PY != 0.0 {
                        let PZ = PX.ln();
                        PZ
                    } else {
                        QA
                    };
                    let QC = PU * (B + ((PV / U) * QB));
                    QZ = QC;
                } else {
                    QZ = PU;
                }
                let QD = ((((parameters[653] + (BM * parameters[654])) + (BN * parameters[655])) + (BO * parameters[656])) + (BT * parameters[657])) + (BU * parameters[658]);
                let QE = ((((parameters[701] + (BM * parameters[702])) + (BN * parameters[703])) + (BO * parameters[704])) + (BT * parameters[705])) + (BU * parameters[706]);
                let QF = ((((parameters[749] + (BM * parameters[750])) + (BN * parameters[751])) + (BO * parameters[752])) + (BT * parameters[753])) + (BU * parameters[754]);
                let QG = ((((parameters[761] + (BM * parameters[762])) + (BN * parameters[763])) + (BO * parameters[764])) + (BT * parameters[765])) + (BU * parameters[766]);
                let QH = ((((parameters[779] + (BM * parameters[780])) + (BN * parameters[781])) + (BO * parameters[782])) + (BT * parameters[783])) + (BU * parameters[784]);
                let QI = ((((parameters[803] + (BM * parameters[804])) + (BN * parameters[805])) + (BO * parameters[806])) + (BT * parameters[807])) + (BU * parameters[808]);
                let QJ = ((((parameters[815] + (BM * parameters[816])) + (BN * parameters[817])) + (BO * parameters[818])) + (BT * parameters[819])) + (BU * parameters[820]);
                let QK = ((((parameters[857] + (BM * parameters[858])) + (BN * parameters[859])) + (BO * parameters[860])) + (BT * parameters[861])) + (BU * parameters[862]);
                let QL = ((((parameters[869] + (BM * parameters[870])) + (BN * parameters[871])) + (BO * parameters[872])) + (BT * parameters[873])) + (BU * parameters[874]);
                let CHX;
                let CIB;
                if AS != 0.0 {
                    let QM = ((((parameters[671] + (BM * parameters[672])) + (BN * parameters[673])) + (BO * parameters[674])) + (BT * parameters[675])) + (BU * parameters[676]);
                    let QN = ((((parameters[839] + (BM * parameters[840])) + (BN * parameters[841])) + (BO * parameters[842])) + (BT * parameters[843])) + (BU * parameters[844]);
                    CHX = QM;
                    CIB = QN;
                } else {
                    CHX = CHY;
                    CIB = CIC;
                }
                let QO = ((((parameters[260] + (BM * parameters[261])) + (BN * parameters[262])) + (BO * parameters[263])) + (BT * parameters[264])) + (BU * parameters[265]);
                let QQ = if QP != A { 1.0 } else { 0.0 };
                let RD;
                if QQ != 0.0 {
                    let QR = B + (U / parameters[162]);
                    let QS = if QR > BZ { 1.0 } else { 0.0 };
                    let QV = if QS != 0.0 {
                        let QT = QR.ln();
                        QT
                    } else {
                        QU
                    };
                    let QW = QO * (B + ((QP / U) * QV));
                    RD = QW;
                } else {
                    RD = QO;
                }
                let QY = if QX != A { 1.0 } else { 0.0 };
                let BFB;
                let CJS;
                if QY != 0.0 {
                    let RA = U - QX;
                    let RC = QZ * (B + ((RA * parameters[588]) * RB));
                    let RE = RD * (B + ((RA * parameters[163]) * RB));
                    BFB = RC;
                    CJS = RE;
                } else {
                    BFB = QZ;
                    CJS = RD;
                }
                BFA = BFB;
                BFD = QF;
                BFG = QH;
                BFK = QD;
                BFN = QI;
                BFR = QE;
                BFT = QK;
                BKP = QG;
                BKU = QJ;
                BKX = QL;
                CHW = CHX;
                CIA = CIB;
                CJR = CJS;
            } else {
                BFA = BFC;
                BFD = BFE;
                BFG = BFH;
                BFK = BFL;
                BFN = BFO;
                BFR = BFS;
                BFT = BFU;
                BKP = BKQ;
                BKU = BKV;
                BKX = BKY;
                CHW = CHY;
                CIA = CIC;
                CJR = CJT;
            }
            let RG = if RF != A { 1.0 } else { 0.0 };
            let RI = if RG != 0.0 && (if RH != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ABO = if RI != 0.0 {
                let RJ = ((((RH + (BM * parameters[1669])) + (BN * parameters[1670])) + (BO * parameters[1671])) + (BT * parameters[1672])) + (BU * parameters[1673]);
                RJ
            } else {
                A
            };
            let RK = if parameters[57] == B { 1.0 } else { 0.0 };
            let ACP;
            let ACR;
            let ACU;
            let ACW;
            let ACZ;
            let ADC;
            let ADE;
            let ADH;
            let ADJ;
            let ADM;
            let ADP;
            if RK != 0.0 {
                let RL = ((((parameters[1807] + (BM * parameters[1808])) + (BN * parameters[1809])) + (BO * parameters[1810])) + (BT * parameters[1811])) + (BU * parameters[1812]);
                let RM = ((((parameters[1814] + (BM * parameters[1815])) + (BN * parameters[1816])) + (BO * parameters[1817])) + (BT * parameters[1818])) + (BU * parameters[1819]);
                let RN = ((((parameters[1821] + (BM * parameters[1822])) + (BN * parameters[1823])) + (BO * parameters[1824])) + (BT * parameters[1825])) + (BU * parameters[1826]);
                let RO = ((((parameters[1829] + (BM * parameters[1830])) + (BN * parameters[1831])) + (BO * parameters[1832])) + (BT * parameters[1833])) + (BU * parameters[1834]);
                let RP = ((((parameters[1835] + (BM * parameters[1836])) + (BN * parameters[1837])) + (BO * parameters[1838])) + (BT * parameters[1839])) + (BU * parameters[1840]);
                let RQ = ((((parameters[1841] + (BM * parameters[1842])) + (BN * parameters[1843])) + (BO * parameters[1844])) + (BT * parameters[1845])) + (BU * parameters[1846]);
                let RR = ((((parameters[1853] + (BM * parameters[1854])) + (BN * parameters[1855])) + (BO * parameters[1856])) + (BT * parameters[1857])) + (BU * parameters[1858]);
                let RS = ((((parameters[1859] + (BM * parameters[1860])) + (BN * parameters[1861])) + (BO * parameters[1862])) + (BT * parameters[1863])) + (BU * parameters[1864]);
                let RT = ((((parameters[1869] + (BM * parameters[1870])) + (BN * parameters[1871])) + (BO * parameters[1872])) + (BT * parameters[1873])) + (BU * parameters[1874]);
                let RU = ((((parameters[1875] + (BM * parameters[1876])) + (BN * parameters[1877])) + (BO * parameters[1878])) + (BT * parameters[1879])) + (BU * parameters[1880]);
                let RV = ((((parameters[1881] + (BM * parameters[1882])) + (BN * parameters[1883])) + (BO * parameters[1884])) + (BT * parameters[1885])) + (BU * parameters[1886]);
                ACP = RL;
                ACR = RM;
                ACU = RN;
                ACW = RO;
                ACZ = RP;
                ADC = RQ;
                ADE = RR;
                ADH = RS;
                ADJ = RT;
                ADM = RU;
                ADP = RV;
            } else {
                ACP = A;
                ACR = A;
                ACU = A;
                ACW = A;
                ACZ = A;
                ADC = A;
                ADE = A;
                ADH = A;
                ADJ = A;
                ADM = A;
                ADP = A;
            }
            let RX = if RW != A { 1.0 } else { 0.0 };
            let VC;
            if RX != 0.0 {
                let RY = B + (U / parameters[101]);
                let RZ = if RY > BZ { 1.0 } else { 0.0 };
                let SC = if RZ != 0.0 {
                    let SA = RY.ln();
                    SA
                } else {
                    SB
                };
                let SD = HP * (B + ((RW / U) * SC));
                VC = SD;
            } else {
                VC = HP;
            }
            let SF = if SE != A { 1.0 } else { 0.0 };
            let VF;
            if SF != 0.0 {
                let SG = B + (U / parameters[159]);
                let SH = if SG > BZ { 1.0 } else { 0.0 };
                let SK = if SH != 0.0 {
                    let SI = SG.ln();
                    SI
                } else {
                    SJ
                };
                let SL = HZ * (B + ((SE / U) * SK));
                VF = SL;
            } else {
                VF = HZ;
            }
            let SN = if SM != A { 1.0 } else { 0.0 };
            let YT;
            if SN != 0.0 {
                let SO = B + (U / parameters[153]);
                let SP = if SO > BZ { 1.0 } else { 0.0 };
                let SS = if SP != 0.0 {
                    let SQ = SO.ln();
                    SQ
                } else {
                    SR
                };
                let ST = HT * (B + ((SM / U) * SS));
                YT = ST;
            } else {
                YT = HT;
            }
            let SV = if SU != A { 1.0 } else { 0.0 };
            let YV;
            if SV != 0.0 {
                let SW = B + (U / parameters[155]);
                let SX = if SW > BZ { 1.0 } else { 0.0 };
                let TA = if SX != 0.0 {
                    let SY = SW.ln();
                    SY
                } else {
                    SZ
                };
                let TB = HU * (B + ((SU / U) * TA));
                YV = TB;
            } else {
                YV = HU;
            }
            let TD = if TC != A { 1.0 } else { 0.0 };
            let YX;
            if TD != 0.0 {
                let TE = B + (U / parameters[157]);
                let TF = if TE > BZ { 1.0 } else { 0.0 };
                let TJ = if TF != 0.0 {
                    let TG = TE.ln();
                    TG
                } else {
                    TH
                };
                let TK = TI * (B + ((TC / U) * TJ));
                YX = TK;
            } else {
                YX = TI;
            }
            let TM = if TL != A { 1.0 } else { 0.0 };
            let WX;
            if TM != 0.0 {
                let TN = B + (U / parameters[429]);
                let TO = if TN > BZ { 1.0 } else { 0.0 };
                let TR = if TO != 0.0 {
                    let TP = TN.ln();
                    TP
                } else {
                    TQ
                };
                let TS = IT * (B + ((TL / U) * TR));
                WX = TS;
            } else {
                WX = IT;
            }
            let TU = if TT != A { 1.0 } else { 0.0 };
            let WZ;
            if TU != 0.0 {
                let TV = B + (U / parameters[433]);
                let TW = if TV > BZ { 1.0 } else { 0.0 };
                let TZ = if TW != 0.0 {
                    let TX = TV.ln();
                    TX
                } else {
                    TY
                };
                let UA = IU * (B + ((TT / U) * TZ));
                WZ = UA;
            } else {
                WZ = IU;
            }
            let UC = if UB != A { 1.0 } else { 0.0 };
            let XC;
            if UC != 0.0 {
                let UD = B + (U / parameters[435]);
                let UE = if UD > BZ { 1.0 } else { 0.0 };
                let UI = if UE != 0.0 {
                    let UF = UD.ln();
                    UF
                } else {
                    UG
                };
                let UJ = UH * (B + ((UB / U) * UI));
                XC = UJ;
            } else {
                XC = UH;
            }
            let UL = if UK != A { 1.0 } else { 0.0 };
            let VH;
            if UL != 0.0 {
                let UM = B + (U / parameters[584]);
                let UN = if UM > BZ { 1.0 } else { 0.0 };
                let UQ = if UN != 0.0 {
                    let UO = UM.ln();
                    UO
                } else {
                    UP
                };
                let UR = JE * (B + ((UK / U) * UQ));
                VH = UR;
            } else {
                VH = JE;
            }
            let UT = if US != A { 1.0 } else { 0.0 };
            let WE;
            if UT != 0.0 {
                let UU = B + (U / parameters[586]);
                let UV = if UU > BZ { 1.0 } else { 0.0 };
                let UZ = if UV != 0.0 {
                    let UW = UU.ln();
                    UW
                } else {
                    UX
                };
                let VA = UY * (B + ((US / U) * UZ));
                WE = VA;
            } else {
                WE = UY;
            }
            let VB = if QX != A { 1.0 } else { 0.0 };
            let VK;
            let VP;
            let ZB;
            if VB != 0.0 {
                let VD = U - QX;
                let VE = VC * (B + ((VD * parameters[99]) * RB));
                let VG = VF * (B + ((VD * parameters[160]) * RB));
                let VI = VH * (B + ((VD * parameters[587]) * RB));
                VK = VE;
                VP = VI;
                ZB = VG;
            } else {
                VK = VC;
                VP = VH;
                ZB = VF;
            }
            let VJ = RB.ln();
            let VL = VK + (parameters[98] * RB);
            let VM = IS + (parameters[427] * RB);
            let VO = if VN > A { 1.0 } else { 0.0 };
            let ZP = if VO != 0.0 {
                let VQ = VP * (B - (JG * (((-VN) * VJ).exp())));
                VQ
            } else {
                let VR = VP * (B - JG);
                VR
            };
            let VS = -RB;
            let VT = JH + (parameters[591] * (rspice_limited_exp((VS / parameters[593]))));
            let VU = JJ + (parameters[599] * (rspice_limited_exp((VS / parameters[601]))));
            let VV = JI + (parameters[595] * (rspice_limited_exp((VS / parameters[597]))));
            let AAL;
            let AAO;
            let AAQ;
            let AAS;
            if OU != 0.0 {
                let VX = VW + (parameters[592] * (rspice_limited_exp((VS / parameters[594]))));
                let VZ = VY + (parameters[600] * (rspice_limited_exp((VS / parameters[602]))));
                let WB = WA + (parameters[596] * (rspice_limited_exp((VS / parameters[598]))));
                let WD = if WC > A { 1.0 } else { 0.0 };
                let AAM = if WD != 0.0 {
                    let WG = WE * (B - (WF * (((-WC) * VJ).exp())));
                    WG
                } else {
                    let WH = WE * (B - WF);
                    WH
                };
                AAL = AAM;
                AAO = VX;
                AAQ = WB;
                AAS = VZ;
            } else {
                AAL = WE;
                AAO = VW;
                AAQ = WA;
                AAS = VY;
            }
            let WJ = if WI == B { 1.0 } else { 0.0 };
            let ZX;
            let ZZ;
            let AAB;
            if WJ != 0.0 {
                let WK = JX + (parameters[912] * (rspice_limited_exp((VS / parameters[913]))));
                let WL = JY + (parameters[915] * (rspice_limited_exp((VS / parameters[916]))));
                ZX = JW;
                ZZ = WK;
                AAB = WL;
            } else {
                let WM = JW + (parameters[909] * (rspice_limited_exp((VS / parameters[910]))));
                ZX = WM;
                ZZ = JX;
                AAB = JY;
            }
            let WN = JP + (parameters[1021] * (rspice_limited_exp((VS / parameters[1023]))));
            let DHZ = if OU != 0.0 {
                let WP = WO + (parameters[1022] * (((-parameters[1024]) * VJ).exp()));
                WP
            } else {
                WO
            };
            let WQ = JC + (parameters[444] * (((-parameters[445]) * VJ).exp()));
            let AAX = if OU != 0.0 {
                let WS = WR + (parameters[446] * (((-parameters[447]) * VJ).exp()));
                WS
            } else {
                WR
            };
            let WT = parameters[448] * (rspice_limited_exp((VS / parameters[449])));
            let WU = JD + WT;
            let CNH = if OU != 0.0 {
                let WW = WV + WT;
                WW
            } else {
                WV
            };
            let WY = WX + (parameters[430] * (rspice_limited_exp((VS / parameters[431]))));
            let XA = parameters[436] * (rspice_limited_exp((VS / parameters[437])));
            let XB = WZ + XA;
            let YO = if OU != 0.0 {
                let XD = XC + XA;
                XD
            } else {
                XC
            };
            let XE = IY + (parameters[438] * (rspice_limited_exp((VS / parameters[439]))));
            let XG = -XF;
            let XH = JA + (parameters[442] * (rspice_limited_exp((XG / parameters[443]))));
            let XI = IV + (parameters[440] * (rspice_limited_exp((XG / parameters[441]))));
            let XJ = NU + (parameters[167] * (rspice_limited_exp((VS / parameters[168]))));
            let XK = NV + (parameters[169] * (rspice_limited_exp((VS / parameters[170]))));
            let XL = if IQ > A { 1.0 } else { 0.0 };
            let XM = if XL != 0.0 || (if IR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let EOI = if XM != 0.0 {
                let XN = (R * GO) / GF;
                let XO = XN * (B + (parameters[398] * (rspice_limited_exp(((-XN) / parameters[399])))));
                XO
            } else {
                A
            };
            let XP = if HH <= AL { 1.0 } else { 0.0 };
            if XP != 0.0 {
            } else {
            }
            let XQ = if HI <= AL { 1.0 } else { 0.0 };
            if XQ != 0.0 {
            } else {
            }
            let XR = if NX <= A { 1.0 } else { 0.0 };
            let YD = if XR != 0.0 {
                XS
            } else {
                NX
            };
            let XT = if HR < A { 1.0 } else { 0.0 };
            let GEV;
            if XT != 0.0 {
                GEV = A;
            } else {
                let XU = if (if HR != A { 1.0 } else { 0.0 }) != 0.0 && (if HR <= 1e24f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GEW;
                if XU != 0.0 {
                    GEW = HR;
                } else {
                    let XV = if HR > 1e31f64 { 1.0 } else { 0.0 };
                    let GEX = if XV != 0.0 {
                        A
                    } else {
                        HR
                    };
                    GEW = GEX;
                }
                GEV = GEW;
            }
            let XW = if HV < A { 1.0 } else { 0.0 };
            if XW != 0.0 {
            } else {
            }
            let XX = if VL <= A { 1.0 } else { 0.0 };
            let CXW = if XX != 0.0 {
                XY
            } else {
                VL
            };
            let COI;
            if AS != 0.0 {
                let YA = if XZ < S { 1.0 } else { 0.0 };
                let COJ = if YA != 0.0 {
                    S
                } else {
                    XZ
                };
                COI = COJ;
            } else {
                COI = XZ;
            }
            let YB = if NN < A { 1.0 } else { 0.0 };
            let BLJ = if YB != 0.0 {
                YC
            } else {
                NN
            };
            let YE = if YD < A { 1.0 } else { 0.0 };
            let FTP = if YE != 0.0 {
                XS
            } else {
                YD
            };
            let YF = if NW < A { 1.0 } else { 0.0 };
            let FTR = if YF != 0.0 {
                YG
            } else {
                NW
            };
            let YH = if NY < A { 1.0 } else { 0.0 };
            let FTQ = if YH != 0.0 {
                YI
            } else {
                NY
            };
            let YJ = if MM < A { 1.0 } else { 0.0 };
            if YJ != 0.0 {
            } else {
            }
            let YK = if MN < A { 1.0 } else { 0.0 };
            if YK != 0.0 {
            } else {
            }
            let YL = if WY <= A { 1.0 } else { 0.0 };
            let BDU = if YL != 0.0 {
                YM
            } else {
                WY
            };
            let YN = if XB <= A { 1.0 } else { 0.0 };
            let BFW = if YN != 0.0 {
                YM
            } else {
                XB
            };
            let YP = if OU != 0.0 && (if YO <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BGK = if YP != 0.0 {
                YM
            } else {
                YO
            };
            let YQ = if HW <= A { 1.0 } else { 0.0 };
            let AOD = if YQ != 0.0 {
                YR
            } else {
                HW
            };
            let YS = if HX <= A { 1.0 } else { 0.0 };
            let AOL = if YS != 0.0 {
                YR
            } else {
                HX
            };
            let YU = if YT < A { 1.0 } else { 0.0 };
            if YU != 0.0 {
            } else {
            }
            let YW = if YV < A { 1.0 } else { 0.0 };
            if YW != 0.0 {
            } else {
            }
            let YY = if OU != 0.0 && (if YX < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if YY != 0.0 {
            } else {
            }
            let YZ = if IB <= A { 1.0 } else { 0.0 };
            let AOS = if YZ != 0.0 {
                ZA
            } else {
                IB
            };
            let ZC = if ZB < A { 1.0 } else { 0.0 };
            let CJE = if ZC != 0.0 {
                A
            } else {
                ZB
            };
            let ZE = if ZD < A { 1.0 } else { 0.0 };
            let CKE = if ZE != 0.0 {
                A
            } else {
                ZD
            };
            let ZF = if ID < VS { 1.0 } else { 0.0 };
            let AOY = if ZF != 0.0 {
                A
            } else {
                ID
            };
            let ZG = if IH < A { 1.0 } else { 0.0 };
            let CNY = if ZG != 0.0 {
                A
            } else {
                IH
            };
            let ZH = if IJ < A { 1.0 } else { 0.0 };
            let COD = if ZH != 0.0 {
                A
            } else {
                IJ
            };
            let ZK = if AS != 0.0 && (if ZI < ZJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ZL = if ZK != 0.0 {
                ZJ
            } else {
                ZI
            };
            let ZM = if AS != 0.0 && (if ZL > YI { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DGC = if ZM != 0.0 {
                YI
            } else {
                ZL
            };
            let ZN = if XE < R { 1.0 } else { 0.0 };
            let ESW = if ZN != 0.0 {
                R
            } else {
                XE
            };
            let ZO = if XH < R { 1.0 } else { 0.0 };
            let EVK = if ZO != 0.0 {
                R
            } else {
                XH
            };
            let ZQ = if ZP < A { 1.0 } else { 0.0 };
            let AWA = if ZQ != 0.0 {
                ZR
            } else {
                ZP
            };
            let ZS = if VT < A { 1.0 } else { 0.0 };
            let AVF = if ZS != 0.0 {
                A
            } else {
                VT
            };
            let ZT = if VV < A { 1.0 } else { 0.0 };
            let AVN = if ZT != 0.0 {
                A
            } else {
                VV
            };
            let ZU = if VU < A { 1.0 } else { 0.0 };
            let AWC = if ZU != 0.0 {
                A
            } else {
                VU
            };
            let ZV = if JK < A { 1.0 } else { 0.0 };
            let BDJ = if ZV != 0.0 {
                A
            } else {
                JK
            };
            let ZW = if JF < A { 1.0 } else { 0.0 };
            let AVC = if ZW != 0.0 {
                A
            } else {
                JF
            };
            let ZY = if ZX < A { 1.0 } else { 0.0 };
            let DVN = if ZY != 0.0 {
                A
            } else {
                ZX
            };
            let AAA = if ZZ < A { 1.0 } else { 0.0 };
            let EWP = if AAA != 0.0 {
                A
            } else {
                ZZ
            };
            let AAC = if AAB < A { 1.0 } else { 0.0 };
            let EWX = if AAC != 0.0 {
                A
            } else {
                AAB
            };
            let AAD = if JZ < A { 1.0 } else { 0.0 };
            let EWV = if AAD != 0.0 {
                A
            } else {
                JZ
            };
            let AAE = if KA < A { 1.0 } else { 0.0 };
            let DVI = if AAE != 0.0 {
                A
            } else {
                KA
            };
            let AAF = if WN < A { 1.0 } else { 0.0 };
            if AAF != 0.0 {
            } else {
            }
            let AAG = if KC < A { 1.0 } else { 0.0 };
            if AAG != 0.0 {
            } else {
            }
            let BBY;
            let BCU;
            let BDE;
            let DKJ;
            if OU != 0.0 {
                let AAI = if AAH < A { 1.0 } else { 0.0 };
                if AAI != 0.0 {
                } else {
                }
                let AAK = if AAJ < A { 1.0 } else { 0.0 };
                if AAK != 0.0 {
                } else {
                }
                let AAN = if AAL < A { 1.0 } else { 0.0 };
                let BBZ = if AAN != 0.0 {
                    A
                } else {
                    AAL
                };
                let AAP = if AAO < A { 1.0 } else { 0.0 };
                let BCV = if AAP != 0.0 {
                    A
                } else {
                    AAO
                };
                let AAR = if AAQ < A { 1.0 } else { 0.0 };
                let DKK = if AAR != 0.0 {
                    A
                } else {
                    AAQ
                };
                let AAT = if AAS < A { 1.0 } else { 0.0 };
                let BDF = if AAT != 0.0 {
                    A
                } else {
                    AAS
                };
                BBY = BBZ;
                BCU = BCV;
                BDE = BDF;
                DKJ = DKK;
            } else {
                BBY = AAL;
                BCU = AAO;
                BDE = AAS;
                DKJ = AAQ;
            }
            let AAU = if KD < A { 1.0 } else { 0.0 };
            if AAU != 0.0 {
            } else {
            }
            let AAV = if KE <= A { 1.0 } else { 0.0 };
            let EQS = if AAV != 0.0 {
                ZA
            } else {
                KE
            };
            let AAW = if WQ < R { 1.0 } else { 0.0 };
            let BHJ = if AAW != 0.0 {
                R
            } else {
                WQ
            };
            let BHS;
            if OU != 0.0 {
                let AAY = if AAX < R { 1.0 } else { 0.0 };
                let BHT = if AAY != 0.0 {
                    R
                } else {
                    AAX
                };
                BHS = BHT;
            } else {
                BHS = AAX;
            }
            let AAZ = if WU < A { 1.0 } else { 0.0 };
            let CMT = if AAZ != 0.0 {
                A
            } else {
                WU
            };
            let ABA = if LI < A { 1.0 } else { 0.0 };
            let FFW = if ABA != 0.0 {
                A
            } else {
                LI
            };
            let ABB = if LQ < A { 1.0 } else { 0.0 };
            let FHU = if ABB != 0.0 {
                A
            } else {
                LQ
            };
            let ABC = if parameters[69] != A { 1.0 } else { 0.0 };
            let FCF;
            let FCV;
            if ABC != 0.0 {
                let ABD = if KL <= A { 1.0 } else { 0.0 };
                let FCG = if ABD != 0.0 {
                    DQ
                } else {
                    KL
                };
                let ABE = if KQ <= A { 1.0 } else { 0.0 };
                let FCW = if ABE != 0.0 {
                    B
                } else {
                    KQ
                };
                FCF = FCG;
                FCV = FCW;
            } else {
                FCF = KL;
                FCV = KQ;
            }
            let ABF = if parameters[68] != A { 1.0 } else { 0.0 };
            let APN;
            let FDW;
            if ABF != 0.0 {
                let ABG = if LF <= A { 1.0 } else { 0.0 };
                let APO = if ABG != 0.0 {
                    B
                } else {
                    LF
                };
                let ABH = if KV <= A { 1.0 } else { 0.0 };
                let FDX = if ABH != 0.0 {
                    B
                } else {
                    KV
                };
                APN = APO;
                FDW = FDX;
            } else {
                APN = LF;
                FDW = KV;
            }
            let ABI = if ABF != 0.0 || ABC != 0.0 { 1.0 } else { 0.0 };
            if ABI != 0.0 {
                let ABK = if ABJ <= A { 1.0 } else { 0.0 };
                if ABK != 0.0 {
                } else {
                }
            } else {
            }
            let ABL = if parameters[1649] >= (RB / R) { 1.0 } else { 0.0 };
            if ABL != 0.0 {
            } else {
            }
            let ABM = if MU <= A { 1.0 } else { 0.0 };
            if ABM != 0.0 {
            } else {
            }
            let ABN = if RF == B { 1.0 } else { 0.0 };
            let ABP = if ABN != 0.0 && (if ABO != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let FRY;
            if ABP != 0.0 {
                let ABR = if ABO < ABQ { 1.0 } else { 0.0 };
                let FRZ = if ABR != 0.0 {
                    A
                } else {
                    ABO
                };
                FRY = FRZ;
            } else {
                FRY = ABO;
            }
            let ABT = if ABS == R { 1.0 } else { 0.0 };
            if ABT != 0.0 {
                let ABU = if MB < A { 1.0 } else { 0.0 };
                if ABU != 0.0 {
                } else {
                }
                let ABV = if MC < A { 1.0 } else { 0.0 };
                if ABV != 0.0 {
                } else {
                }
                let ABW = if MD < A { 1.0 } else { 0.0 };
                if ABW != 0.0 {
                } else {
                }
                let ABX = if ME < A { 1.0 } else { 0.0 };
                if ABX != 0.0 {
                } else {
                }
                let ABY = if MF < A { 1.0 } else { 0.0 };
                if ABY != 0.0 {
                } else {
                }
                let ABZ = if MG < A { 1.0 } else { 0.0 };
                if ABZ != 0.0 {
                } else {
                }
                let ACA = if MH < A { 1.0 } else { 0.0 };
                if ACA != 0.0 {
                } else {
                }
                let ACB = if MI < A { 1.0 } else { 0.0 };
                if ACB != 0.0 {
                } else {
                }
            } else {
            }
            let ACC = if HO <= A { 1.0 } else { 0.0 };
            let GDJ;
            if ACC != 0.0 {
                GDJ = B;
            } else {
                let ACD = if HO > R { 1.0 } else { 0.0 };
                let GDK = if ACD != 0.0 {
                    B
                } else {
                    HO
                };
                GDJ = GDK;
            }
            let ACE = if MO < A { 1.0 } else { 0.0 };
            if ACE != 0.0 {
            } else {
            }
            let ACF = if MP < A { 1.0 } else { 0.0 };
            if ACF != 0.0 {
            } else {
            }
            let ACG = if MK < A { 1.0 } else { 0.0 };
            if ACG != 0.0 {
            } else {
            }
            let ACH = if ML < A { 1.0 } else { 0.0 };
            if ACH != 0.0 {
            } else {
            }
            let ACI = if MQ < A { 1.0 } else { 0.0 };
            if ACI != 0.0 {
            } else {
            }
            let ACK = if MR <= ACJ { 1.0 } else { 0.0 };
            if ACK != 0.0 {
            } else {
            }
            let ACL = if MS <= ACJ { 1.0 } else { 0.0 };
            if ACL != 0.0 {
            } else {
            }
            let ACM = if MT <= ACJ { 1.0 } else { 0.0 };
            if ACM != 0.0 {
            } else {
            }
            let ACO = if HN < (-ACN) { 1.0 } else { 0.0 };
            let AEN = if ACO != 0.0 {
                A
            } else {
                HN
            };
            let APX;
            let AQE;
            let AQH;
            let AQL;
            let AQQ;
            let AQU;
            let ATB;
            let ATF;
            let ATM;
            let ATR;
            let ATW;
            if RK != 0.0 {
                let ACQ = if (if ACP < B { 1.0 } else { 0.0 }) != 0.0 || (if ACP > DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let APY = if ACQ != 0.0 {
                    R
                } else {
                    ACP
                };
                let ACS = if (if ACR < B { 1.0 } else { 0.0 }) != 0.0 || (if ACR > DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AQF = if ACS != 0.0 {
                    ACT
                } else {
                    ACR
                };
                let ACV = if (if ACU < B { 1.0 } else { 0.0 }) != 0.0 || (if ACU > DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AQI = if ACV != 0.0 {
                    ACT
                } else {
                    ACU
                };
                let ACX = if ACW < A { 1.0 } else { 0.0 };
                let AQM = if ACX != 0.0 {
                    ACY
                } else {
                    ACW
                };
                let ADA = if ACZ < A { 1.0 } else { 0.0 };
                let AQR = if ADA != 0.0 {
                    ADB
                } else {
                    ACZ
                };
                let ADD = if ADC < A { 1.0 } else { 0.0 };
                let AQV = if ADD != 0.0 {
                    ADB
                } else {
                    ADC
                };
                let ADF = if ADE < A { 1.0 } else { 0.0 };
                let ATC = if ADF != 0.0 {
                    ADG
                } else {
                    ADE
                };
                let ADI = if ADH < A { 1.0 } else { 0.0 };
                let ATG = if ADI != 0.0 {
                    R
                } else {
                    ADH
                };
                let ADK = if ADJ < A { 1.0 } else { 0.0 };
                let ATN = if ADK != 0.0 {
                    ADL
                } else {
                    ADJ
                };
                let ADN = if ADM < A { 1.0 } else { 0.0 };
                let ATS = if ADN != 0.0 {
                    ADO
                } else {
                    ADM
                };
                let ADQ = if ADP < A { 1.0 } else { 0.0 };
                let ATX = if ADQ != 0.0 {
                    ADR
                } else {
                    ADP
                };
                APX = APY;
                AQE = AQF;
                AQH = AQI;
                AQL = AQM;
                AQQ = AQR;
                AQU = AQV;
                ATB = ATC;
                ATF = ATG;
                ATM = ATN;
                ATR = ATS;
                ATW = ATX;
            } else {
                APX = ACP;
                AQE = ACR;
                AQH = ACU;
                AQL = ACW;
                AQQ = ACZ;
                AQU = ADC;
                ATB = ADE;
                ATF = ADH;
                ATM = ADJ;
                ATR = ADM;
                ATW = ADP;
            }
            let ADS = if (if D != A { 1.0 } else { 0.0 }) != 0.0 && (if E > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if ADS != 0.0 {
                let ADT = if parameters[1795] != A { 1.0 } else { 0.0 };
                if ADT != 0.0 {
                } else {
                }
                let ADU = if parameters[1794] != A { 1.0 } else { 0.0 };
                if ADU != 0.0 {
                } else {
                }
                if AX != 0.0 {
                    let ADV = if parameters[1796] != A { 1.0 } else { 0.0 };
                    if ADV != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let ADX = if ADW != A { 1.0 } else { 0.0 };
            let GDB;
            if ADX != 0.0 {
                let ADZ = parameters[1074] / ADY;
                let AEA = parameters[1075] * U;
                let AEB = if ADY == R { 1.0 } else { 0.0 };
                let AED = if AEB != 0.0 {
                    AEC
                } else {
                    DQ
                };
                let AEE = B / (if ABQ >= ((ADZ + (AEA / AED)) / BJ) { ABQ } else { ((ADZ + (AEA / AED)) / BJ) });
                let AEF = if ADW == R { 1.0 } else { 0.0 };
                if AEF != 0.0 {
                } else {
                }
                GDB = AEE;
            } else {
                GDB = A;
            }
            let AEG = if parameters[77] == A { 1.0 } else { 0.0 };
            let AGF;
            let AGI;
            if AEG != 0.0 {
                let AEH = parameters[1078] * parameters[18];
                let AEI = parameters[1079] * parameters[19];
                AGF = AEH;
                AGI = AEI;
            } else {
                let AEK = if AEJ > A { 1.0 } else { 0.0 };
                let AFE = if AEK != 0.0 {
                    let AEL = (ACN * CO) + ((CS + ((ACN - CS) * parameters[1084])) * AEJ);
                    AEL
                } else {
                    let AEM = ACN * (if AL >= (CO + AEJ) { AL } else { (CO + AEJ) });
                    AEM
                };
                let AEO = ACN + AEN;
                let AFG;
                if AEP != 0.0 {
                    AFG = AEQ;
                } else {
                    let AET = if H != 0.0 {
                        AER
                    } else {
                        AES
                    };
                    let AFC = if H != 0.0 {
                        let AEV = 3.43e26f64 / AEU;
                        let AEY = ((AEW + ((AET - AEW) / (B + ((AEU / 9.68e22f64).powf(6.8e-1f64))))) - (4.34e1f64 / (B + (AEV * AEV)))) * AEX;
                        AEY
                    } else {
                        let AEZ = 6.1e26f64 / AEU;
                        let AFB = ((AFA + ((AET - AFA) / (B + ((AEU / 2.23e22f64).powf(7.19e-1f64))))) - (2.9e1f64 / (B + (AEZ * AEZ)))) * AEX;
                        AFB
                    };
                    let AFD = B / ((HD * AEU) * AFC);
                    AFG = AFD;
                }
                let AFF = if AFE <= (if 1e-18f64 >= (CS * (CO + (if A <= AEJ { A } else { AEJ }))) { 1e-18f64 } else { (CS * (CO + (if A <= AEJ { A } else { AEJ }))) }) { AFE } else { (if 1e-18f64 >= (CS * (CO + (if A <= AEJ { A } else { AEJ }))) { 1e-18f64 } else { (CS * (CO + (if A <= AEJ { A } else { AEJ }))) }) };
                let AFH = ((AFG / 1.4281480067421144e0f64) / (1.7724538509055159e0f64 * U)) * (((B / (AFF.sqrt())) - (R / (AFE.sqrt()))) + ((AFF / (AFE * AFE)).sqrt()));
                let AFI = (AFE * U) + parameters[1092];
                let AFK = ((AFJ * AFI) / (AFG * ((AEO * U) + parameters[1093]))).sqrt();
                let AFM = rspice_limited_exp((R * (AFL / AFK)));
                let AFN = if parameters[1086] == B { 1.0 } else { 0.0 };
                let AFU;
                let AFV;
                if AFN != 0.0 {
                    let AFO = (AFG * AFK) / AFJ;
                    let AFP = AFM * (B + AFO);
                    let AFQ = (AFP + B) - AFO;
                    let AFR = (AFP - B) + AFO;
                    AFU = AFQ;
                    AFV = AFR;
                } else {
                    let AFS = AFM + B;
                    let AFT = AFM - B;
                    AFU = AFS;
                    AFV = AFT;
                }
                let AFW = ((AFG * AFK) * AFU) / (AFI * AFV);
                let AFX = if AEJ < -1e-10f64 { 1.0 } else { 0.0 };
                let AGC = if AFX != 0.0 {
                    let AFY = AFJ / (((-AEJ) * CS) * U);
                    let AFZ = AFW + AFH;
                    let AGA = (AFZ * AFY) / (AFZ + AFY);
                    AGA
                } else {
                    let AGB = AFW + AFH;
                    AGB
                };
                let AGD = (AGC / BJ) * (if A >= ((((parameters[1094] + (parameters[1095] * CS)) + (parameters[1096] * ACN)) + (parameters[1097] * AFL)) + (parameters[1098] * AEJ)) { A } else { ((((parameters[1094] + (parameters[1095] * CS)) + (parameters[1096] * ACN)) + (parameters[1097] * AFL)) + (parameters[1098] * AEJ)) });
                AGF = AGD;
                AGI = AGD;
            }
            let AGE = if WI == A { 1.0 } else { 0.0 };
            let DVV;
            let DVY;
            if AGE != 0.0 {
                let AGH = if AGF < AGG { 1.0 } else { 0.0 };
                let DVW = if AGH != 0.0 {
                    A
                } else {
                    AGF
                };
                let AGJ = if AGI < AGG { 1.0 } else { 0.0 };
                let DVZ = if AGJ != 0.0 {
                    A
                } else {
                    AGI
                };
                DVV = DVW;
                DVY = DVZ;
            } else {
                let AGK = if AGF <= AGG { 1.0 } else { 0.0 };
                let DVX = if AGK != 0.0 {
                    AGG
                } else {
                    AGF
                };
                let AGL = if AGI <= AGG { 1.0 } else { 0.0 };
                let DWA = if AGL != 0.0 {
                    AGG
                } else {
                    AGI
                };
                DVV = DVX;
                DVY = DWA;
            }
            let AGN = if AGM != B { 1.0 } else { 0.0 };
            if AGN != 0.0 {
                if AGO != 0.0 {
                } else {
                    let AGQ = if AGP != 0.0 && (if AF > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if AGQ != 0.0 {
                    } else {
                        let AGR = if AGM == DQ { 1.0 } else { 0.0 };
                        if AGR != 0.0 {
                        } else {
                        }
                    }
                }
                if AGS != 0.0 {
                } else {
                    let AGT = if AGP != 0.0 && (if AF > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if AGT != 0.0 {
                    } else {
                        let AGU = if AGM == DQ { 1.0 } else { 0.0 };
                        if AGU != 0.0 {
                        } else {
                        }
                    }
                }
            } else {
            }
            let AGV = if AGM == R { 1.0 } else { 0.0 };
            if AGV != 0.0 {
                let AGY = AGW + AGX;
                let AHA = AGZ * (ACN - CS);
                let AHC = if A >= (AHA - AHB) { A } else { (AHA - AHB) };
                let AHE = if A >= (AEJ + AHD) { A } else { (AEJ + AHD) };
                let AHF = if AGX > A { 1.0 } else { 0.0 };
                if AHF != 0.0 {
                    let AHJ = if ((AHG * M) / (AHH * AHI)) > BZ { 1.0 } else { 0.0 };
                    if AHJ != 0.0 {
                    } else {
                    }
                } else {
                    let AHL = AGY + AHB;
                    let AHM = AHK + ((ZJ * AHL) / AHE);
                    let AHO = (AHL - AHE).abs();
                    let AHP = AHI * AHN;
                    let AHR = AHQ * ((N * ((if AHE <= AHL { AHE } else { AHL }) - (AHI / (AHM + B)))) / AHI);
                    let AHT = if AHR > AHS { 1.0 } else { 0.0 };
                    if AHT != 0.0 {
                    } else {
                        let AHV = if AHR > AHU { 1.0 } else { 0.0 };
                        if AHV != 0.0 {
                        } else {
                            let AHW = if AHR < -3.7e1f64 { 1.0 } else { 0.0 };
                            if AHW != 0.0 {
                            } else {
                            }
                        }
                    }
                    let AHX = if ((AHI + (1.5707963267948966e0f64 * (AHO * (AGZ * (if (AHE / AHL) <= (AHL / AHE) { (AHE / AHL) } else { (AHL / AHE) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                    if AHX != 0.0 {
                    } else {
                    }
                    let AHY = AHP / AGY;
                    let AHZ = AHY + B;
                    let AIA = if (((((((((AHB * AHB) + ((R * AGY) * AHB)) + ((AGY * AGY) * AHZ)).sqrt()) * (AHZ.sqrt())) + AHB) + (AGY * AHY)) + AGY) / ((AHB * ((AHZ * (AHY + Q)).sqrt())) + (AHB * (AHY + R)))) > BZ { 1.0 } else { 0.0 };
                    if AIA != 0.0 {
                    } else {
                    }
                    let AIB = AHM * AHN;
                    let AIC = AIB * AIB;
                    let AID = AIC + B;
                    let AIE = AIB * AHB;
                    let AIF = if ((((((AID * (((AIE * AIE) + (((R * AIB) * AHP) * AHB)) + ((AID * AHP) * AHP))).sqrt()) + AIE) + (AIC * AHP)) + AHP) / (((AID.sqrt()) + B) * AIE)) > BZ { 1.0 } else { 0.0 };
                    if AIF != 0.0 {
                    } else {
                    }
                }
                if AHF != 0.0 {
                    let AIG = AHC + AHB;
                    let AIH = AHK + ((ZJ * AIG) / AHA);
                    let AII = (AIG - AHA).abs();
                    let AIJ = AHI * AHN;
                    let AIK = AHQ * ((N * ((if AHA <= AIG { AHA } else { AIG }) - (AHI / (AIH + B)))) / AHI);
                    let AIL = if AIK > AHS { 1.0 } else { 0.0 };
                    if AIL != 0.0 {
                    } else {
                        let AIM = if AIK > AHU { 1.0 } else { 0.0 };
                        if AIM != 0.0 {
                        } else {
                            let AIN = if AIK < -3.7e1f64 { 1.0 } else { 0.0 };
                            if AIN != 0.0 {
                            } else {
                            }
                        }
                    }
                    let AIO = if ((AHI + (1.5707963267948966e0f64 * (AII * (AGZ * (if (AHA / AIG) <= (AIG / AHA) { (AHA / AIG) } else { (AIG / AHA) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                    if AIO != 0.0 {
                    } else {
                    }
                    let AIP = AIJ / AHC;
                    let AIQ = AIP + B;
                    let AIR = if (((((((((AHB * AHB) + ((R * AHC) * AHB)) + ((AHC * AHC) * AIQ)).sqrt()) * (AIQ.sqrt())) + AHB) + (AHC * AIP)) + AHC) / ((AHB * ((AIQ * (AIP + Q)).sqrt())) + (AHB * (AIP + R)))) > BZ { 1.0 } else { 0.0 };
                    if AIR != 0.0 {
                    } else {
                    }
                    let AIS = AIH * AHN;
                    let AIT = AIS * AIS;
                    let AIU = AIT + B;
                    let AIV = AIS * AHB;
                    let AIW = if ((((((AIU * (((AIV * AIV) + (((R * AIS) * AIJ) * AHB)) + ((AIU * AIJ) * AIJ))).sqrt()) + AIV) + (AIT * AIJ)) + AIJ) / (((AIU.sqrt()) + B) * AIV)) > BZ { 1.0 } else { 0.0 };
                    if AIW != 0.0 {
                    } else {
                    }
                } else {
                    let AIX = AHC + AHB;
                    let AIY = AHK + ((ZJ * AIX) / AHA);
                    let AIZ = (AIX - AHA).abs();
                    let AJA = AHI * AHN;
                    let AJB = AHQ * ((N * ((if AHA <= AIX { AHA } else { AIX }) - (AHI / (AIY + B)))) / AHI);
                    let AJC = if AJB > AHS { 1.0 } else { 0.0 };
                    if AJC != 0.0 {
                    } else {
                        let AJD = if AJB > AHU { 1.0 } else { 0.0 };
                        if AJD != 0.0 {
                        } else {
                            let AJE = if AJB < -3.7e1f64 { 1.0 } else { 0.0 };
                            if AJE != 0.0 {
                            } else {
                            }
                        }
                    }
                    let AJF = if ((AHI + (1.5707963267948966e0f64 * (AIZ * (AGZ * (if (AHA / AIX) <= (AIX / AHA) { (AHA / AIX) } else { (AIX / AHA) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                    if AJF != 0.0 {
                    } else {
                    }
                    let AJG = AJA / AHC;
                    let AJH = AJG + B;
                    let AJI = if (((((((((AHB * AHB) + ((R * AHC) * AHB)) + ((AHC * AHC) * AJH)).sqrt()) * (AJH.sqrt())) + AHB) + (AHC * AJG)) + AHC) / ((AHB * ((AJH * (AJG + Q)).sqrt())) + (AHB * (AJG + R)))) > BZ { 1.0 } else { 0.0 };
                    if AJI != 0.0 {
                    } else {
                    }
                    let AJJ = AIY * AHN;
                    let AJK = AJJ * AJJ;
                    let AJL = AJK + B;
                    let AJM = AJJ * AHB;
                    let AJN = if ((((((AJL * (((AJM * AJM) + (((R * AJJ) * AJA) * AHB)) + ((AJL * AJA) * AJA))).sqrt()) + AJM) + (AJK * AJA)) + AJA) / (((AJL.sqrt()) + B) * AJM)) > BZ { 1.0 } else { 0.0 };
                    if AJN != 0.0 {
                    } else {
                    }
                }
                if AHF != 0.0 {
                } else {
                    let AJO = if AEJ > A { 1.0 } else { 0.0 };
                    if AJO != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let AJP = if AGM == DQ { 1.0 } else { 0.0 };
            if AJP != 0.0 {
                let AJQ = AGW + AGX;
                let AJR = AGZ * (ACN - AY);
                let AJS = if A >= (AJR - AHB) { A } else { (AJR - AHB) };
                let AJT = if A >= (AEJ + AHD) { A } else { (AEJ + AHD) };
                let AJU = AGZ * parameters[41];
                let AJV = if AGX > A { 1.0 } else { 0.0 };
                if AJV != 0.0 {
                    let AJW = if ((AHG * M) / (AHH * AHI)) > BZ { 1.0 } else { 0.0 };
                    if AJW != 0.0 {
                    } else {
                    }
                } else {
                    let AJX = AJQ + AHB;
                    let AJY = AHK + ((ZJ * AJX) / AJT);
                    let AJZ = (AJX - AJT).abs();
                    let AKA = AHI * AHN;
                    let AKB = AHQ * ((N * ((if AJT <= AJX { AJT } else { AJX }) - (AHI / (AJY + B)))) / AHI);
                    let AKC = if AKB > AHS { 1.0 } else { 0.0 };
                    if AKC != 0.0 {
                    } else {
                        let AKD = if AKB > AHU { 1.0 } else { 0.0 };
                        if AKD != 0.0 {
                        } else {
                            let AKE = if AKB < -3.7e1f64 { 1.0 } else { 0.0 };
                            if AKE != 0.0 {
                            } else {
                            }
                        }
                    }
                    let AKF = if ((AHI + (1.5707963267948966e0f64 * (AJZ * (AGZ * (if (AJT / AJX) <= (AJX / AJT) { (AJT / AJX) } else { (AJX / AJT) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                    if AKF != 0.0 {
                    } else {
                    }
                    let AKG = AKA / AJQ;
                    let AKH = AKG + B;
                    let AKI = if (((((((((AHB * AHB) + ((R * AJQ) * AHB)) + ((AJQ * AJQ) * AKH)).sqrt()) * (AKH.sqrt())) + AHB) + (AJQ * AKG)) + AJQ) / ((AHB * ((AKH * (AKG + Q)).sqrt())) + (AHB * (AKG + R)))) > BZ { 1.0 } else { 0.0 };
                    if AKI != 0.0 {
                    } else {
                    }
                    let AKJ = AJY * AHN;
                    let AKK = AKJ * AKJ;
                    let AKL = AKK + B;
                    let AKM = AKJ * AHB;
                    let AKN = if ((((((AKL * (((AKM * AKM) + (((R * AKJ) * AKA) * AHB)) + ((AKL * AKA) * AKA))).sqrt()) + AKM) + (AKK * AKA)) + AKA) / (((AKL.sqrt()) + B) * AKM)) > BZ { 1.0 } else { 0.0 };
                    if AKN != 0.0 {
                    } else {
                    }
                }
                let AKO = AGW + AHB;
                let AKP = AHK + ((ZJ * AKO) / AJU);
                let AKQ = (AKO - AJU).abs();
                let AKR = AHI * AHN;
                let AKS = AHQ * ((N * ((if AJU <= AKO { AJU } else { AKO }) - (AHI / (AKP + B)))) / AHI);
                let AKT = if AKS > AHS { 1.0 } else { 0.0 };
                if AKT != 0.0 {
                } else {
                    let AKU = if AKS > AHU { 1.0 } else { 0.0 };
                    if AKU != 0.0 {
                    } else {
                        let AKV = if AKS < -3.7e1f64 { 1.0 } else { 0.0 };
                        if AKV != 0.0 {
                        } else {
                        }
                    }
                }
                let AKW = if ((AHI + (1.5707963267948966e0f64 * (AKQ * (AGZ * (if (AJU / AKO) <= (AKO / AJU) { (AJU / AKO) } else { (AKO / AJU) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                if AKW != 0.0 {
                } else {
                }
                let AKX = AKR / AGW;
                let AKY = AKX + B;
                let AKZ = AHB * AHB;
                let ALA = if ((((((((AKZ + ((R * AGW) * AHB)) + ((AGW * AGW) * AKY)).sqrt()) * (AKY.sqrt())) + AHB) + (AGW * AKX)) + AGW) / ((AHB * ((AKY * (AKX + Q)).sqrt())) + (AHB * (AKX + R)))) > BZ { 1.0 } else { 0.0 };
                if ALA != 0.0 {
                } else {
                }
                let ALB = AKP * AHN;
                let ALC = ALB * ALB;
                let ALD = ALC + B;
                let ALE = ALB * AHB;
                let ALF = if ((((((ALD * (((ALE * ALE) + (((R * ALB) * AKR) * AHB)) + ((ALD * AKR) * AKR))).sqrt()) + ALE) + (ALC * AKR)) + AKR) / (((ALD.sqrt()) + B) * ALE)) > BZ { 1.0 } else { 0.0 };
                if ALF != 0.0 {
                } else {
                }
                if AJV != 0.0 {
                    let ALG = AJS + AHB;
                    let ALH = AHK + ((ZJ * ALG) / AJR);
                    let ALI = (ALG - AJR).abs();
                    let ALJ = AHQ * ((N * ((if AJR <= ALG { AJR } else { ALG }) - (AHI / (ALH + B)))) / AHI);
                    let ALK = if ALJ > AHS { 1.0 } else { 0.0 };
                    if ALK != 0.0 {
                    } else {
                        let ALL = if ALJ > AHU { 1.0 } else { 0.0 };
                        if ALL != 0.0 {
                        } else {
                            let ALM = if ALJ < -3.7e1f64 { 1.0 } else { 0.0 };
                            if ALM != 0.0 {
                            } else {
                            }
                        }
                    }
                    let ALN = if ((AHI + (1.5707963267948966e0f64 * (ALI * (AGZ * (if (AJR / ALG) <= (ALG / AJR) { (AJR / ALG) } else { (ALG / AJR) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                    if ALN != 0.0 {
                    } else {
                    }
                    let ALO = AKR / AJS;
                    let ALP = ALO + B;
                    let ALQ = if ((((((((AKZ + ((R * AJS) * AHB)) + ((AJS * AJS) * ALP)).sqrt()) * (ALP.sqrt())) + AHB) + (AJS * ALO)) + AJS) / ((AHB * ((ALP * (ALO + Q)).sqrt())) + (AHB * (ALO + R)))) > BZ { 1.0 } else { 0.0 };
                    if ALQ != 0.0 {
                    } else {
                    }
                    let ALR = ALH * AHN;
                    let ALS = ALR * ALR;
                    let ALT = ALS + B;
                    let ALU = ALR * AHB;
                    let ALV = if ((((((ALT * (((ALU * ALU) + (((R * ALR) * AKR) * AHB)) + ((ALT * AKR) * AKR))).sqrt()) + ALU) + (ALS * AKR)) + AKR) / (((ALT.sqrt()) + B) * ALU)) > BZ { 1.0 } else { 0.0 };
                    if ALV != 0.0 {
                    } else {
                    }
                } else {
                    let ALW = AJS + AHB;
                    let ALX = AHK + ((ZJ * ALW) / AJR);
                    let ALY = (ALW - AJR).abs();
                    let ALZ = AHQ * ((N * ((if AJR <= ALW { AJR } else { ALW }) - (AHI / (ALX + B)))) / AHI);
                    let AMA = if ALZ > AHS { 1.0 } else { 0.0 };
                    if AMA != 0.0 {
                    } else {
                        let AMB = if ALZ > AHU { 1.0 } else { 0.0 };
                        if AMB != 0.0 {
                        } else {
                            let AMC = if ALZ < -3.7e1f64 { 1.0 } else { 0.0 };
                            if AMC != 0.0 {
                            } else {
                            }
                        }
                    }
                    let AMD = if ((AHI + (1.5707963267948966e0f64 * (ALY * (AGZ * (if (AJR / ALW) <= (ALW / AJR) { (AJR / ALW) } else { (ALW / AJR) }))))) / AHI) > BZ { 1.0 } else { 0.0 };
                    if AMD != 0.0 {
                    } else {
                    }
                    let AME = AKR / AJS;
                    let AMF = AME + B;
                    let AMG = if ((((((((AKZ + ((R * AJS) * AHB)) + ((AJS * AJS) * AMF)).sqrt()) * (AMF.sqrt())) + AHB) + (AJS * AME)) + AJS) / ((AHB * ((AMF * (AME + Q)).sqrt())) + (AHB * (AME + R)))) > BZ { 1.0 } else { 0.0 };
                    if AMG != 0.0 {
                    } else {
                    }
                    let AMH = ALX * AHN;
                    let AMI = AMH * AMH;
                    let AMJ = AMI + B;
                    let AMK = AMH * AHB;
                    let AML = if ((((((AMJ * (((AMK * AMK) + (((R * AMH) * AKR) * AHB)) + ((AMJ * AKR) * AKR))).sqrt()) + AMK) + (AMI * AKR)) + AKR) / (((AMJ.sqrt()) + B) * AMK)) > BZ { 1.0 } else { 0.0 };
                    if AML != 0.0 {
                    } else {
                    }
                }
                let AMM = AJS + AHB;
                let AMN = AHK + ((ZJ * AMM) / AJR);
                let AMO = (AMM - AJR).abs();
                let AMP = N * ((if AJR <= AMM { AJR } else { AMM }) - (AHI / (AMN + B)));
                let AMQ = AHQ * (AMP / AHI);
                let AMR = if AMQ > AHS { 1.0 } else { 0.0 };
                if AMR != 0.0 {
                } else {
                    let AMS = if AMQ > AHU { 1.0 } else { 0.0 };
                    if AMS != 0.0 {
                    } else {
                        let AMT = if AMQ < -3.7e1f64 { 1.0 } else { 0.0 };
                        if AMT != 0.0 {
                        } else {
                        }
                    }
                }
                let AMU = AMO * (AGZ * (if (AJR / AMM) <= (AMM / AJR) { (AJR / AMM) } else { (AMM / AJR) }));
                let AMV = if ((AHI + (1.5707963267948966e0f64 * AMU)) / AHI) > BZ { 1.0 } else { 0.0 };
                if AMV != 0.0 {
                } else {
                }
                let AMW = AKR / AJS;
                let AMX = AMW + B;
                let AMY = ((((AKZ + ((R * AJS) * AHB)) + ((AJS * AJS) * AMX)).sqrt()) * (AMX.sqrt())) + AHB;
                let AMZ = AJS * AMW;
                let ANA = (AHB * ((AMX * (AMW + Q)).sqrt())) + (AHB * (AMW + R));
                let ANB = if (((AMY + AMZ) + AJS) / ANA) > BZ { 1.0 } else { 0.0 };
                if ANB != 0.0 {
                } else {
                }
                let ANC = AMN * AHN;
                let AND = ANC * ANC;
                let ANE = AND + B;
                let ANF = ANE.sqrt();
                let ANG = ANC * AHB;
                let ANH = ANG * ANG;
                let ANI = ((R * ANC) * AKR) * AHB;
                let ANJ = ANE * AKR;
                let ANK = AND * AKR;
                let ANL = if ((((((ANE * ((ANH + ANI) + (ANJ * AKR))).sqrt()) + ANG) + ANK) + AKR) / ((ANF + B) * ANG)) > BZ { 1.0 } else { 0.0 };
                if ANL != 0.0 {
                } else {
                }
                let ANM = AHQ * (AMP / AHI);
                let ANN = if ANM > AHS { 1.0 } else { 0.0 };
                if ANN != 0.0 {
                } else {
                    let ANO = if ANM > AHU { 1.0 } else { 0.0 };
                    if ANO != 0.0 {
                    } else {
                        let ANP = if ANM < -3.7e1f64 { 1.0 } else { 0.0 };
                        if ANP != 0.0 {
                        } else {
                        }
                    }
                }
                let ANQ = if ((AHI + (1.5707963267948966e0f64 * AMU)) / AHI) > BZ { 1.0 } else { 0.0 };
                if ANQ != 0.0 {
                } else {
                }
                let ANR = if (((AMY + AMZ) + AJS) / ANA) > BZ { 1.0 } else { 0.0 };
                if ANR != 0.0 {
                } else {
                }
                let ANS = if ((((((ANE * ((ANH + ANI) + (ANJ * AKR))).sqrt()) + ANG) + ANK) + AKR) / ((ANF + B) * ANG)) > BZ { 1.0 } else { 0.0 };
                if ANS != 0.0 {
                } else {
                }
                if AJV != 0.0 {
                } else {
                    let ANT = if AEJ > A { 1.0 } else { 0.0 };
                    if ANT != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let ANU = if (B + (CO / parameters[91])) > BZ { 1.0 } else { 0.0 };
            if ANU != 0.0 {
            } else {
            }
            let ANV = if AV != AW { 1.0 } else { 0.0 };
            if ANV != 0.0 {
            } else {
            }
            let ANW = P * CQ;
            let ANX = 1e-8f64 / ANW;
            let ANZ = B / (BK * ((HH * ANY).powf(KB)));
            let AOA = ((ANW * AGZ) * CS).sqrt();
            let AOB = (((L * GO) / FW) * (B + ((GO * FW) / (((R * L) * GF) * GF)))).sqrt();
            let AOC = if (if parameter_given[172] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            let DMI;
            if AOC != 0.0 {
                let AOE = ((AOD * RB) / AOB) + S;
                let AOG = if AOE < AOF { 1.0 } else { 0.0 };
                let DMJ = if AOG != 0.0 {
                    let AOH = AGZ / ((AOE.cosh()) - B);
                    AOH
                } else {
                    let AOI = rspice_limited_exp((-AOE));
                    AOI
                };
                DMI = DMJ;
            } else {
                DMI = AOJ;
            }
            let AOK = if (if parameter_given[174] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            let DKW;
            if AOK != 0.0 {
                let AOM = ((AOL * RB) / AOB) + S;
                let AON = if AOM < AOF { 1.0 } else { 0.0 };
                let DKX = if AON != 0.0 {
                    let AOO = AGZ / ((AOM.cosh()) - B);
                    AOO
                } else {
                    let AOP = rspice_limited_exp((-AOM));
                    AOP
                };
                DKW = DKX;
            } else {
                DKW = AOQ;
            }
            let AOR = if (if parameter_given[173] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            let DMO;
            if AOR != 0.0 {
                let AOT = ((AOS * RB) / AOB) + S;
                let AOU = if AOT < AOF { 1.0 } else { 0.0 };
                let DMP = if AOU != 0.0 {
                    let AOV = AGZ / ((AOT.cosh()) - B);
                    AOV
                } else {
                    let AOW = rspice_limited_exp((-AOT));
                    AOW
                };
                DMO = DMP;
            } else {
                DMO = AOX;
            }
            let AOZ = ((B + (AOY / RB)).sqrt()) - B;
            let APA = ((AOS * RB) / AOB) + S;
            let APB = if APA < AOF { 1.0 } else { 0.0 };
            let DMU = if APB != 0.0 {
                let APD = B / (if (B + (APC * ((APA.cosh()) - R))) >= S { (B + (APC * ((APA.cosh()) - R))) } else { S });
                APD
            } else {
                let APE = rspice_limited_exp((-APA));
                let APF = APE / (if (APC + APE) >= S { (APC + APE) } else { S });
                APF
            };
            let APG = ((HD * HE) * GO) / FW;
            let APR;
            let FDT;
            if H != 0.0 {
                APR = APH;
                FDT = API;
            } else {
                APR = APJ;
                FDT = APK;
            }
            let APM = APL * APL;
            let APP = APL * APN;
            let APQ = ((ABJ / APL).powf(LE)) / APM;
            let APS = (HH * APR) * (((ABJ / APP).powf(LE)) / (APP * APP));
            let APU = if APT < -2.7315e2f64 { 1.0 } else { 0.0 };
            let AWK = if APU != 0.0 {
                APV
            } else {
                let APW = APT + 2.7315e2f64;
                APW
            };
            let DSX;
            let DTA;
            let DTC;
            let DTG;
            let DTI;
            let DTL;
            let DTN;
            let DTR;
            let DTT;
            let DTW;
            let DTY;
            let DUC;
            let DUD;
            if RK != 0.0 {
                let APZ = parameters[1806] - APX;
                let AQB = AY * AQA;
                let AQC = B + (rspice_limited_exp((((parameters[1827] * AQA) - AQB) / parameters[1828])));
                let AQD = (APZ / AQC) + APX;
                let AQG = ((parameters[1813] - AQE) / AQC) + AQE;
                let AQJ = ((parameters[1820] - AQH) / AQC) + AQH;
                let AQK = B + (rspice_limited_exp((((parameters[1850] * AQA) - AQB) / parameters[1851])));
                let AQN = ((-parameters[1847]) / AQK) + AQL;
                let AQP = AGZ * (AQN + (((AQN * AQN) + 2.5e-7f64).sqrt()));
                let AQS = ((-parameters[1848]) / AQK) + AQQ;
                let AQT = AGZ * (AQS + (((AQS * AQS) + 2.5e-7f64).sqrt()));
                let AQW = ((-parameters[1849]) / AQK) + AQU;
                let AQX = AGZ * (AQW + (((AQW * AQW) + 2.5e-7f64).sqrt()));
                let AQZ = AQY * (AQD - AQY);
                let ARA = AQZ * AQZ;
                let ARC = AQY * (AQD - ARB);
                let ARD = ARC * ARC;
                let ARE = AQY * (AQG - AQY);
                let ARF = ARE * ARE;
                let ARG = AQY * (AQG - ARB);
                let ARH = ARG * ARG;
                let ARI = AQY * (AQJ - AQY);
                let ARJ = ARI * ARI;
                let ARK = AQY * (AQJ - ARB);
                let ARL = ARK * ARK;
                let ARM = GO / HH;
                let ARN = EF * AQA;
                let ARO = B / (B + (rspice_limited_exp(((2.75e0f64 - ARN) / 7.8e-1f64))));
                let ARP = ARO - AGZ;
                let ARQ = AGZ * ((ARO + AGZ) + (((ARP * ARP) + 2.25e-6f64).sqrt()));
                let ARR = (((B - ARQ) * (AQD - APX)) / APZ) + ARQ;
                let ARS = B / (B + (rspice_limited_exp(((ARR - 9.99e-1f64) / AEX))));
                let ARU = ((((AGZ * EF) * EF) * CI) - ((ART * EF) * AQA)) + R;
                let ARV = ARU - Q;
                let ARW = ARN - ((AGZ * ((ARU + Q) - (((ARV * ARV) + 2.5e-5f64).sqrt()))) + 2.5e-3f64);
                let ARY = ARW.powf(ARX);
                let ASB = R.powf(ARX);
                let ASC = (AGZ * (((AGZ * (((APM + ((ARY * 9.059e5f64) / ASB)) + ASA) + (((((APM + ((ARY * 9.059e5f64) / ASB)) - ASA) * ((APM + ((ARY * 9.059e5f64) / ASB)) - ASA)) + 2.5e-5f64).sqrt()))) + ARZ) - (((((AGZ * (((APM + ((ARY * 9.059e5f64) / ASB)) + ASA) + (((((APM + ((ARY * 9.059e5f64) / ASB)) - ASA) * ((APM + ((ARY * 9.059e5f64) / ASB)) - ASA)) + 2.5e-5f64).sqrt()))) - ARZ) * ((AGZ * (((APM + ((ARY * 9.059e5f64) / ASB)) + ASA) + (((((APM + ((ARY * 9.059e5f64) / ASB)) - ASA) * ((APM + ((ARY * 9.059e5f64) / ASB)) - ASA)) + 2.5e-5f64).sqrt()))) - ARZ)) + 2.13444e7f64).sqrt()))) + 2.31e3f64;
                let ASF = ARW.powf(ASE);
                let ASH = R.powf(ASE);
                let ASJ = (1.2066e2f64 * (Q.powf(ASI))) / (ARN.powf(ASI));
                let ASM = (1.07e2f64 * (Q.powf(ASL))) / (ARN.powf(ASL));
                let ASO = 7e-1f64 + ((ARW.powf(parameters[1898])) * ASN);
                let ASP = ASO + AGZ;
                let ASQ = ASO - AGZ;
                let ASR = ASQ * ASQ;
                let AST = (1.03e2f64 * (Q.powf(ASS))) / (ARN.powf(ASS));
                let ASW = (8.33e2f64 * (Q.powf(ASV))) / (ARN.powf(ASV));
                let ASY = parameters[1852] * AQA;
                let ASZ = parameters[1867] * ((AGZ * (((AGZ * ((ASD + ((ASF * 2.5e0f64) / ASH)) + ((((ASD + ((ASF * 2.5e0f64) / ASH)) * (ASD + ((ASF * 2.5e0f64) / ASH))) + 2.5e-5f64).sqrt()))) + ASG) - (((((AGZ * ((ASD + ((ASF * 2.5e0f64) / ASH)) + ((((ASD + ((ASF * 2.5e0f64) / ASH)) * (ASD + ((ASF * 2.5e0f64) / ASH))) + 2.5e-5f64).sqrt()))) - ASG) * ((AGZ * ((ASD + ((ASF * 2.5e0f64) / ASH)) + ((((ASD + ((ASF * 2.5e0f64) / ASH)) * (ASD + ((ASF * 2.5e0f64) / ASH))) + 2.5e-5f64).sqrt()))) - ASG)) + 2.5e-5f64).sqrt()))) + 2.5e-3f64);
                let ATA = parameters[1868] * ((R * (Q.powf(ASK))) / (ARN.powf(ASK)));
                let ATD = ATB + (parameters[1865] * ((ASC / (AQB.powf(ASZ))) - (ASC / (ASY.powf(ASZ)))));
                let ATE = AGZ * (ATD + (((ATD * ATD) + 2.5e-5f64).sqrt()));
                let ATH = ATF + (parameters[1866] * ((ASJ / (AQB.powf(ATA))) - (ASJ / (ASY.powf(ATA)))));
                let ATI = AGZ * (ATH + (((ATH * ATH) + 2.5e-5f64).sqrt()));
                let ATJ = parameters[1890] * ((AGZ * (((AGZ * (ASP + ((ASR + 2.5e-5f64).sqrt()))) + B) - (((((AGZ * (ASP + ((ASR + 2.5e-5f64).sqrt()))) - B) * ((AGZ * (ASP + ((ASR + 2.5e-5f64).sqrt()))) - B)) + 2.5e-5f64).sqrt()))) + 2.5e-3f64);
                let ATK = ASM / ((B + (AW * (AQB.powf(ATJ)))).sqrt());
                let ATL = ASM / ((B + (AW * (ASY.powf(ATJ)))).sqrt());
                let ATO = parameters[1891] * ((ART * (Q.powf(ASU))) / (ARN.powf(ASU)));
                let ATP = AST / ((B + (AW * (AQB.powf(ATO)))).sqrt());
                let ATQ = AST / ((B + (AW * (ASY.powf(ATO)))).sqrt());
                let ATT = parameters[1892] * ((3.4e0f64 * (Q.powf(ASX))) / (ARN.powf(ASX)));
                let ATU = ASW / ((B + (AW * (AQB.powf(ATT)))).sqrt());
                let ATV = ASW / ((B + (AW * (ASY.powf(ATT)))).sqrt());
                let ATY = AQD / R;
                let AUB = B + ATY;
                let AUC = AUB - B;
                let AUE = (R * AUB) - DQ;
                let AUG = R * AUC;
                let HVA = AUG * AUG;
                let HVB = HVA * AUG;
                let HVC = HVB * AUG;
                let HVD = HVC * AUG;
                let AUN = (((HD * ATY) * ((ATZ.powf(ATY)) / (((((((((AUA * ((-4.6e0f64 * AUC).exp())) + (AUD * (((((((AUE * AUE) * AUE) * AUE) * AUE) * AUE) * AUE) * AUE))) + (AUF * (HVD * AUG))) - (AUI * HVD)) + (AUJ * HVC)) - (AUK * HVB)) + (AUL * HVA)) - (3.88e-1f64 * AUC)) + AUM))) * (((ATM + (parameters[1887] * ((AGZ * (ATK + (((ATK * ATK) + 2.5000000000000005e-3f64).sqrt()))) - (AGZ * (ATL + (((ATL * ATL) + 2.5000000000000005e-3f64).sqrt())))))) * ANY).powf(AQD))) * (((HH.powf(((AGZ * (((AGZ * (AQZ + ((ARA + 2.5e-7f64).sqrt()))) + B) - (((((AGZ * (AQZ + ((ARA + 2.5e-7f64).sqrt()))) - B) * ((AGZ * (AQZ + ((ARA + 2.5e-7f64).sqrt()))) - B)) + 2.5e-7f64).sqrt()))) + 2.5e-4f64))) * (ARM.powf(((AGZ * (((AGZ * (ARC + ((ARD + 2.5e-7f64).sqrt()))) + B) - (((((AGZ * (ARC + ((ARD + 2.5e-7f64).sqrt()))) - B) * ((AGZ * (ARC + ((ARD + 2.5e-7f64).sqrt()))) - B)) + 2.5e-7f64).sqrt()))) + 2.5e-4f64)))) / FW);
                let AUO = AQG / R;
                let AUP = B + AUO;
                let AUQ = AUP - B;
                let AUR = (R * AUP) - DQ;
                let AUS = R * AUQ;
                let HVE = AUS * AUS;
                let HVF = HVE * AUS;
                let HVG = HVF * AUS;
                let HVH = HVG * AUS;
                let AUT = (((HD * AUO) * ((ATZ.powf(AUO)) / (((((((((AUA * ((-4.6e0f64 * AUQ).exp())) + (AUD * (((((((AUR * AUR) * AUR) * AUR) * AUR) * AUR) * AUR) * AUR))) + (AUF * (HVH * AUS))) - (AUI * HVH)) + (AUJ * HVG)) - (AUK * HVF)) + (AUL * HVE)) - (3.88e-1f64 * AUQ)) + AUM))) * (((ATR + (parameters[1888] * ((AGZ * (ATP + (((ATP * ATP) + 2.5000000000000005e-3f64).sqrt()))) - (AGZ * (ATQ + (((ATQ * ATQ) + 2.5000000000000005e-3f64).sqrt())))))) * ANY).powf(AQG))) * (((HH.powf(((AGZ * (((AGZ * (ARE + ((ARF + 2.5e-7f64).sqrt()))) + B) - (((((AGZ * (ARE + ((ARF + 2.5e-7f64).sqrt()))) - B) * ((AGZ * (ARE + ((ARF + 2.5e-7f64).sqrt()))) - B)) + 2.5e-7f64).sqrt()))) + 2.5e-4f64))) * (ARM.powf(((AGZ * (((AGZ * (ARG + ((ARH + 2.5e-7f64).sqrt()))) + B) - (((((AGZ * (ARG + ((ARH + 2.5e-7f64).sqrt()))) - B) * ((AGZ * (ARG + ((ARH + 2.5e-7f64).sqrt()))) - B)) + 2.5e-7f64).sqrt()))) + 2.5e-4f64)))) / FW);
                let AUU = AQJ / R;
                let AUV = B + AUU;
                let AUW = AUV - B;
                let AUX = (R * AUV) - DQ;
                let AUY = R * AUW;
                let HVI = AUY * AUY;
                let HVJ = HVI * AUY;
                let HVK = HVJ * AUY;
                let HVL = HVK * AUY;
                let AUZ = (((HD * AUU) * ((ATZ.powf(AUU)) / (((((((((AUA * ((-4.6e0f64 * AUW).exp())) + (AUD * (((((((AUX * AUX) * AUX) * AUX) * AUX) * AUX) * AUX) * AUX))) + (AUF * (HVL * AUY))) - (AUI * HVL)) + (AUJ * HVK)) - (AUK * HVJ)) + (AUL * HVI)) - (3.88e-1f64 * AUW)) + AUM))) * (((ATW + (parameters[1889] * ((AGZ * (ATU + (((ATU * ATU) + 2.5000000000000005e-3f64).sqrt()))) - (AGZ * (ATV + (((ATV * ATV) + 2.5000000000000005e-3f64).sqrt())))))) * ANY).powf(AQJ))) * (((HH.powf(((AGZ * (((AGZ * (ARI + ((ARJ + 2.5e-7f64).sqrt()))) + B) - (((((AGZ * (ARI + ((ARJ + 2.5e-7f64).sqrt()))) - B) * ((AGZ * (ARI + ((ARJ + 2.5e-7f64).sqrt()))) - B)) + 2.5e-7f64).sqrt()))) + 2.5e-4f64))) * (ARM.powf(((AGZ * (((AGZ * (ARK + ((ARL + 2.5e-7f64).sqrt()))) + B) - (((((AGZ * (ARK + ((ARL + 2.5e-7f64).sqrt()))) - B) * ((AGZ * (ARK + ((ARL + 2.5e-7f64).sqrt()))) - B)) + 2.5e-7f64).sqrt()))) + 2.5e-4f64)))) / FW);
                DSX = AQP;
                DTA = AQD;
                DTC = AUN;
                DTG = ATE;
                DTI = AQT;
                DTL = AQG;
                DTN = AUT;
                DTR = ATI;
                DTT = AQX;
                DTW = AQJ;
                DTY = AUZ;
                DUC = ARR;
                DUD = ARS;
            } else {
                DSX = A;
                DTA = A;
                DTC = A;
                DTG = A;
                DTI = A;
                DTL = A;
                DTN = A;
                DTR = A;
                DTT = A;
                DTW = A;
                DTY = A;
                DUC = B;
                DUD = A;
            }
            let AVA = if parameters[58] == B { 1.0 } else { 0.0 };
            let BBJ;
            let BCO;
            let BCT;
            let BDB;
            let CKR;
            if AVA != 0.0 {
                let AVD = EF * AQA;
                let AVE = AVB + ((AVC - AVB) / ((rspice_limited_exp((((parameters[890] * AQA) - AVD) / parameters[891]))) + B));
                let AVI = ((((AVF - AVG) - ((parameters[893] * AQA) * AVH)) + (AVD * AVH)) / (B + (rspice_limited_exp((((parameters[895] * AQA) - AVD) / parameters[896]))))) + AVG;
                let AVJ = AVF + ZJ;
                let AVK = AVI - AVJ;
                let AVL = AGZ * ((AVI + AVJ) - (((AVK * AVK) + 9e-2f64).sqrt()));
                let AVO = AVM - AVN;
                let AVP = (((AVO * 3.7e2f64) / (AVD.powf(parameters[898]))) + (AVO / (B + (rspice_limited_exp(((AVD - (parameters[899] * AQA)) / parameters[900])))))) + AVN;
                let AVQ = AVP - AVM;
                let AVR = AGZ * ((AVP + AVM) - (((AVQ * AVQ) + 1.0000000000000002e-2f64).sqrt()));
                let AVS = AY / (AY + EF);
                let AVV = (((AVT * EF) * EF) * CI) - (AVU * ABQ);
                let AVW = AVT + 2.4e-1f64;
                let AVY = AEX / ((((AVV + (((AVV * AVV) + ((((((Q * AVU) * ABQ) * AVW) * EF) * EF) * CI)).sqrt())) / ((((R * AVW) * EF) * EF) * CI)) - 8.208e-1f64) - (parameters[907] * AVX));
                let AVZ = AVY - B;
                let AWB = (AWA * (AVS + (parameters[904] * (B - AVS)))) * (AGZ * ((AVY + B) - (((AVZ * AVZ) + 9e-4f64).sqrt())));
                let AWD = (parameters[902] * AQA) - AVD;
                let AWE = ((parameters[901] - AWC) * ((AGZ * (AWD + (((AWD * AWD) + 2.5e-1f64).sqrt()))).powf(parameters[903]))) + AWC;
                BBJ = AWB;
                BCO = AVL;
                BCT = AVR;
                BDB = AWE;
                CKR = AVE;
            } else {
                BBJ = AWA;
                BCO = AVF;
                BCT = AVN;
                BDB = AWC;
                CKR = AVC;
            }
            let AWJ;
            let GHH;
            if ADS != 0.0 {
                let AWH = (AWF + node_potentials[4]) + AWG;
                AWJ = AWH;
                GHH = GHC;
            } else {
                let AWI = AWF + AWG;
                AWJ = AWI;
                GHH = GRF;
            }
            let AWL = AWJ / AWK;
            let GRG = GHH / AWK;
            let AWM = AWL - B;
            let AWN = AWJ - AWK;
            let AWP = AWO * AWJ;
            let GRH = GHH * AWO;
            let AWQ = AWO * AWK;
            let AWT = if AWS != A { 1.0 } else { 0.0 };
            let AZV;
            let BRL;
            let BTN;
            let BTQ;
            let BWJ;
            let CGJ;
            let DLG;
            let GHI;
            let GHJ;
            let GHK;
            let GHL;
            let GHM;
            let GHN;
            let GHO;
            if AWT != 0.0 {
                let AWU = AWJ - AWR;
                let GRI = GHH * AWU;
                let AWW = (AQO * AWV) * AWV;
                let AWX = ((AWU * AWU) + AWW).sqrt();
                let AWY = AGZ * ((AWJ + AWR) + AWX);
                let GRK = (GHH + ((GRI + GRI) * (GHB / (GRJ * AWX)))) * AGZ;
                let AWZ = -parameters[1790];
                let AXB = AWZ * (AWJ - AXA);
                let GRL = GHH * AWZ;
                let GRM = GRL * AXB;
                let AXD = (AQO * AXC) * AXC;
                let AXE = ((AXB * AXB) + AXD).sqrt();
                let AXF = AGZ * (AXB + AXE);
                let GRN = (GRL + ((GRM + GRM) * (GHB / (GRJ * AXE)))) * AGZ;
                let AXG = if AWS == B { 1.0 } else { 0.0 };
                let AYX;
                let BRM;
                let BTO;
                let BTR;
                let BWK;
                let CGK;
                let GHP;
                let GHQ;
                let GHR;
                let GHS;
                let GHT;
                let GHU;
                if AXG != 0.0 {
                    let AXH = AWK - AWR;
                    let AXI = AGZ * ((AWK + AWR) + (((AXH * AXH) + AWW).sqrt()));
                    let AXJ = AWZ * (AWK - AXA);
                    let AXK = AGZ * (AXJ + (((AXJ * AXJ) + AXD).sqrt()));
                    let AXL = if AWK > AWR { 1.0 } else { 0.0 };
                    let AXO;
                    let GHV;
                    if AXL != 0.0 {
                        let GSC = GRK + GRN;
                        let AXM = (((AWY + AXF) - AXI) - AXK) + AWK;
                        AXO = AXM;
                        GHV = GSC;
                    } else {
                        let GSB = GRK + GRN;
                        let AXN = (((AWY + AXF) - AXI) - AXK) + AWR;
                        AXO = AXN;
                        GHV = GSB;
                    }
                    let AXP = AWJ - AXO;
                    let GSD = (GHH - GHV) * AXP;
                    let AXQ = ((AXP * AXP) + 1.0000000000000002e-2f64).sqrt();
                    let AXR = AGZ * ((AWJ + AXO) + AXQ);
                    let GSE = ((GHH + GHV) + ((GSD + GSD) * (GHB / (GRJ * AXQ)))) * AGZ;
                    AYX = AXR;
                    BRM = A;
                    BTO = A;
                    BTR = A;
                    BWK = A;
                    CGK = A;
                    GHP = GSE;
                    GHQ = GRF;
                    GHR = GRF;
                    GHS = GRF;
                    GHT = GRF;
                    GHU = GRF;
                } else {
                    let AXT = if AWR > AXS { 1.0 } else { 0.0 };
                    let AXZ = if AXT != 0.0 {
                        AXS
                    } else {
                        AWR
                    };
                    let AXU = AWJ - AXS;
                    let AXV = (AGZ * AXU).tanh();
                    let GRO = ((GHH * AGZ) * (GHB - (AXV * AXV))) * AGZ;
                    let AXW = AGZ + (AGZ * AXV);
                    let AXX = B - AXW;
                    let GRQ = GRO * GRP;
                    let AXY = if AWK > AXS { 1.0 } else { 0.0 };
                    let AYY;
                    let GHW;
                    if AXY != 0.0 {
                        let AYA = AXS - AXZ;
                        let AYB = AWZ * (AXS - AXA);
                        let GRV = GRK + GRN;
                        let AYC = (((AWY + AXF) - (AGZ * ((AXS + AXZ) + (((AYA * AYA) + AWW).sqrt())))) - (AGZ * (AYB + (((AYB * AYB) + AXD).sqrt())))) + AXS;
                        let AYD = AWJ - AYC;
                        let GRW = (GHH - GRV) * AYD;
                        let AYE = ((AYD * AYD) + 1.0000000000000002e-2f64).sqrt();
                        let AYF = AGZ * ((AWJ + AYC) + AYE);
                        let GRX = ((GHH + GRV) + ((GRW + GRW) * (GHB / (GRJ * AYE)))) * AGZ;
                        AYY = AYF;
                        GHW = GRX;
                    } else {
                        let AYG = AWK - AXZ;
                        let AYH = AGZ * ((AWK + AXZ) + (((AYG * AYG) + AWW).sqrt()));
                        let AYI = AWZ * (AWK - AXA);
                        let AYJ = AGZ * (AYI + (((AYI * AYI) + AXD).sqrt()));
                        let AYK = if AWK > AXZ { 1.0 } else { 0.0 };
                        let AYN;
                        let GHX;
                        if AYK != 0.0 {
                            let GRS = GRK + GRN;
                            let AYL = (((AWY + AXF) - AYH) - AYJ) + AWK;
                            AYN = AYL;
                            GHX = GRS;
                        } else {
                            let GRR = GRK + GRN;
                            let AYM = (((AWY + AXF) - AYH) - AYJ) + AXZ;
                            AYN = AYM;
                            GHX = GRR;
                        }
                        let AYO = AWJ - AYN;
                        let GRT = (GHH - GHX) * AYO;
                        let AYP = ((AYO * AYO) + 1.0000000000000002e-2f64).sqrt();
                        let AYQ = AGZ * ((AWJ + AYN) + AYP);
                        let AYR = (AXX * AYQ) + (AXW * AWJ);
                        let GRU = ((GRQ * AYQ) + ((((GHH + GHX) + ((GRT + GRT) * (GHB / (GRJ * AYP)))) * AGZ) * AXX)) + ((GRO * AWJ) + (GHH * AXW));
                        AYY = AYR;
                        GHW = GRU;
                    }
                    let GRY = GHH * AXU;
                    let AYS = ((AXU * AXU) + 1.0000000000000002e-2f64).sqrt();
                    let AYT = AGZ * ((AWJ + AXS) - AYS);
                    let GRZ = (GHH - ((GRY + GRY) * (GHB / (GRJ * AYS)))) * AGZ;
                    let AYU = AWK - AXS;
                    let AYV = AYT - (AGZ * ((AWK + AXS) - (((AYU * AYU) + 1.0000000000000002e-2f64).sqrt())));
                    let AYW = (AYT - AXS) / AWK;
                    let GSA = GRZ / AWK;
                    AYX = AYY;
                    BRM = AYW;
                    BTO = AXX;
                    BTR = AXW;
                    BWK = AYV;
                    CGK = AYT;
                    GHP = GHW;
                    GHQ = GSA;
                    GHR = GRQ;
                    GHS = GRO;
                    GHT = GRZ;
                    GHU = GRZ;
                }
                let AYZ = AWO * AYX;
                let GSF = GHP * AWO;
                AZV = AYX;
                BRL = BRM;
                BTN = BTO;
                BTQ = BTR;
                BWJ = BWK;
                CGJ = CGK;
                DLG = AYZ;
                GHI = GHP;
                GHJ = GHQ;
                GHK = GHR;
                GHL = GHS;
                GHM = GHT;
                GHN = GHU;
                GHO = GSF;
            } else {
                AZV = A;
                BRL = A;
                BTN = A;
                BTQ = A;
                BWJ = A;
                CGJ = A;
                DLG = A;
                GHI = GRF;
                GHJ = GRF;
                GHK = GRF;
                GHL = GRF;
                GHM = GRF;
                GHN = GRF;
                GHO = GRF;
            }
            let AZC = AZB * AWJ;
            let AZE = AWJ + AZD;
            let AZF = (AZC * AWJ) / AZE;
            let AZG = AZA - AZF;
            let GSG = (((((GHH * AZB) * AWJ) + (GHH * AZC)) - (GHH * AZF)) / AZE) * GRP;
            let AZH = AZA - (((AZB * AWK) * AWK) / (AWK + AZD));
            let AZI = AWJ / APV;
            let GSH = GHH / APV;
            let AZJ = AZI.sqrt();
            let AZK = AZI * AZJ;
            let GSI = (GSH * AZJ) + ((GSH * (GHB / (GRJ * AZJ))) * AZI);
            let AZM = AZL * AZK;
            let GSJ = GSI * AZL;
            let AZN = R * AWP;
            let GSK = GRH * R;
            let AZO = AZG / AZN;
            let GSL = (GSG - (GSK * AZO)) / AZN;
            let AZP = (AZA / 5.1728373261e-2f64) - AZO;
            let AZQ = rspice_limited_exp(AZP);
            let AZR = AZM * AZQ;
            let GSM = (GSJ * AZQ) + (((GSL * GRP) * (rspice_limited_exp_derivative(AZP))) * AZM);
            let AZS = if AWS == A { 1.0 } else { 0.0 };
            let BAX;
            let CWW;
            let GHY;
            let GHZ;
            if AZS != 0.0 {
                let AZU = AZT * AZK;
                let GSR = GSI * AZT;
                BAX = AZU;
                CWW = A;
                GHY = GSR;
                GHZ = GRF;
            } else {
                let AZW = AZV / APV;
                let GSN = GHI / APV;
                let AZX = AZT * AZW;
                let AZY = AZW.sqrt();
                let AZZ = AZX * AZY;
                let GSO = ((GSN * AZT) * AZY) + ((GSN * (GHB / (GRJ * AZY))) * AZX);
                let BAA = if AZM > BZ { 1.0 } else { 0.0 };
                let BAD;
                let GIA;
                if BAA != 0.0 {
                    let BAB = AZM.ln();
                    let GSP = GSJ * (GHB / AZM);
                    BAD = BAB;
                    GIA = GSP;
                } else {
                    BAD = BAC;
                    GIA = GRF;
                }
                let BAE = (BAD + (AZA / 5.1728373261e-2f64)) - AZO;
                let GSQ = GIA - GSL;
                BAX = AZZ;
                CWW = BAE;
                GHY = GSO;
                GHZ = GSQ;
            }
            let GSS = GHH * NP;
            let BAF = (B + (NP * AWN)) - S;
            let BAG = if BAF < -1e1f64 { 1.0 } else { 0.0 };
            let BAK;
            let GIB;
            if BAG != 0.0 {
                let BAH = -1e-6f64 / BAF;
                let GSV = ((GSS * BAH) * GRP) / BAF;
                BAK = BAH;
                GIB = GSV;
            } else {
                let GST = GSS * BAF;
                let BAI = ((BAF * BAF) + 4e-6f64).sqrt();
                let BAJ = AGZ * (BAF + BAI);
                let GSU = (GSS + ((GST + GST) * (GHB / (GRJ * BAI)))) * AGZ;
                BAK = BAJ;
                GIB = GSU;
            }
            let BAL = AWP * HD;
            let GSW = GRH * HD;
            let BAM = (R * GO) / GF;
            let BAN = 3.313029364696188e-34f64 / BAM;
            let BAO = BAN * BAN;
            let BAP = BAO / 1.6689520000000002e-30f64;
            let BAQ = BAO / 3.4618e-31f64;
            let BAS = (BAP - BAQ) / BAL;
            let BAT = (BAP - (Q * BAP)) / BAL;
            let BAU = (BAP - (Q * BAQ)) / BAL;
            let BAV = ((B + (BAR * (rspice_limited_exp(BAS)))) + (rspice_limited_exp(BAT))) + (BAR * (rspice_limited_exp(BAU)));
            let BAY = BAW * BAX;
            let BAZ = 3.4618e-31f64 / BAY;
            let BBA = (BAZ * BAL) / BAM;
            let BBB = BBA * BAV;
            let GSX = ((((((((GHY * BAW) * BAZ) * GRP) / BAY) * BAL) + (GSW * BAZ)) / BAM) * BAV) + ((((((((GSW * BAS) * GRP) / BAL) * (rspice_limited_exp_derivative(BAS))) * BAR) + ((((GSW * BAT) * GRP) / BAL) * (rspice_limited_exp_derivative(BAT)))) + (((((GSW * BAU) * GRP) / BAL) * (rspice_limited_exp_derivative(BAU))) * BAR)) * BBA);
            let BBC = if BBB > BZ { 1.0 } else { 0.0 };
            let BBG;
            let GIC;
            if BBC != 0.0 {
                let BBD = BBB.ln();
                let GSY = GSX * (GHB / BBB);
                BBG = BBD;
                GIC = GSY;
            } else {
                BBG = BBE;
                GIC = GRF;
            }
            let BBF = -AWP;
            let BBH = IP * ((BAP / HD) + (BBF * BBG));
            let GSZ = (((GRH * GRP) * BBG) + (GIC * BBF)) * IP;
            let BBI = AWL.ln();
            let GTA = GRG * (GHB / AWL);
            let CGP;
            let CGV;
            let CHB;
            let DGS;
            let DGX;
            let DHE;
            let DIA;
            let DIF;
            let DIT;
            let DJC;
            let DJH;
            let DJK;
            let DJP;
            let DJS;
            let DJX;
            let DKL;
            let DNA;
            let DUM;
            let DVP;
            let ECY;
            let EGU;
            let EHF;
            let EOS;
            let EOU;
            let GID;
            let GIE;
            let GIF;
            let GIG;
            let GIH;
            let GII;
            let GIJ;
            let GIK;
            let GIL;
            let GIM;
            let GIN;
            let GIO;
            let GIP;
            let GIQ;
            let GIR;
            let GIS;
            let GIT;
            let GIU;
            let GIV;
            let GIW;
            if AZS != 0.0 {
                let BBK = (MV * BBI).exp();
                let BBL = BBJ * BBK;
                let HCG = ((GTA * MV) * BBK) * BBJ;
                let BBO = MX * AWN;
                let HCH = GHH * MX;
                let BBR = (BBO - (BBQ * BBL)) - AEX;
                let BBT = (BBO - (BBS * BBL)) - AEX;
                let BBV = ((BBR * BBT) - ((Q * (BBU * BBL)) * AEX)).sqrt();
                let BBW = BBL + ((BBN * BBL) + (AGZ * (((BBO - (BBP * BBL)) - AEX) + BBV)));
                let HCI = HCG + ((HCG * BBN) + (((HCH - (HCG * BBP)) + (((((HCH - (HCG * BBQ)) * BBT) + ((HCH - (HCG * BBS)) * BBR)) - (((HCG * BBU) * Q) * AEX)) * (GHB / (GRJ * BBV)))) * AGZ));
                let BBX = if OT == B { 1.0 } else { 0.0 };
                let DJD;
                let GIX;
                if BBX != 0.0 {
                    let BCB = (BCA * BBI).exp();
                    let BCC = BBY * BCB;
                    let HCJ = ((GTA * BCA) * BCB) * BBY;
                    let BCF = BCE * AWN;
                    let HCK = GHH * BCE;
                    let BCI = (BCF - (BCH * BCC)) - AEX;
                    let BCK = (BCF - (BCJ * BCC)) - AEX;
                    let BCM = ((BCI * BCK) - ((Q * (BCL * BCC)) * AEX)).sqrt();
                    let BCN = BCC + ((BCD * BCC) + (AGZ * (((BCF - (BCG * BCC)) - AEX) + BCM)));
                    let HCL = HCJ + ((HCJ * BCD) + (((HCK - (HCJ * BCG)) + (((((HCK - (HCJ * BCH)) * BCK) + ((HCK - (HCJ * BCJ)) * BCI)) - (((HCJ * BCL) * Q) * AEX)) * (GHB / (GRJ * BCM)))) * AGZ));
                    DJD = BCN;
                    GIX = HCL;
                } else {
                    DJD = A;
                    GIX = GRF;
                }
                let BCP = -BCO;
                let HCM = GHH * MZ;
                let BCQ = ((MZ * AWN) - BCP) - S;
                let HCN = HCM * BCQ;
                let BCR = ((BCQ * BCQ) - ((Q * BCP) * S)).sqrt();
                let HCO = (HCM + ((HCN + HCN) * (GHB / (GRJ * BCR)))) * AGZ;
                let BCS = BCO + (BCP + (AGZ * (BCQ + BCR)));
                let DJL;
                let GIY;
                if OU != 0.0 {
                    let BCW = -BCU;
                    let HCP = GHH * BCX;
                    let BCY = ((BCX * AWN) - BCW) - S;
                    let HCQ = HCP * BCY;
                    let BCZ = ((BCY * BCY) - ((Q * BCW) * S)).sqrt();
                    let HCR = (HCP + ((HCQ + HCQ) * (GHB / (GRJ * BCZ)))) * AGZ;
                    let BDA = BCU + (BCW + (AGZ * (BCY + BCZ)));
                    DJL = BDA;
                    GIY = HCR;
                } else {
                    DJL = A;
                    GIY = GRF;
                }
                let BDC = (NC * BBI).exp();
                let BDD = BDB * BDC;
                let HCS = ((GTA * NC) * BDC) * BDB;
                let DJT;
                let GIZ;
                if OU != 0.0 {
                    let BDH = (BDG * BBI).exp();
                    let BDI = BDE * BDH;
                    let HCT = ((GTA * BDG) * BDH) * BDE;
                    DJT = BDI;
                    GIZ = HCT;
                } else {
                    DJT = A;
                    GIZ = GRF;
                }
                let BDK = (NE * BBI).exp();
                let BDL = BDJ * BDK;
                let HCU = ((GTA * NE) * BDK) * BDJ;
                let HCV = GHH * NK;
                let BDM = (B + (NK * AWN)) - S;
                let BDN = if BDM < -1e1f64 { 1.0 } else { 0.0 };
                let BDR;
                let GJA;
                if BDN != 0.0 {
                    let BDO = -1e-6f64 / BDM;
                    let HCY = ((HCV * BDO) * GRP) / BDM;
                    BDR = BDO;
                    GJA = HCY;
                } else {
                    let HCW = HCV * BDM;
                    let BDP = ((BDM * BDM) + 4e-6f64).sqrt();
                    let BDQ = AGZ * (BDM + BDP);
                    let HCX = (HCV + ((HCW + HCW) * (GHB / (GRJ * BDP)))) * AGZ;
                    BDR = BDQ;
                    GJA = HCX;
                }
                let BDT = if BDS != A { 1.0 } else { 0.0 };
                let CGQ;
                let GJB;
                if BDT != 0.0 {
                    let BDV = -BDU;
                    let BDW = -NI;
                    let HDE = GHH * BDW;
                    let BDX = ((BDW * AWN) - BDV) - S;
                    let HDF = HDE * BDX;
                    let BDY = ((BDX * BDX) - ((Q * BDV) * S)).sqrt();
                    let HDG = (HDE + ((HDF + HDF) * (GHB / (GRJ * BDY)))) * AGZ;
                    let BDZ = BDU + (BDV + (AGZ * (BDX + BDY)));
                    CGQ = BDZ;
                    GJB = HDG;
                } else {
                    let BEA = -NI;
                    let HCZ = GHH * BEA;
                    let BEB = (B + (BEA * AWN)) - S;
                    let BEC = if BEB < -1e1f64 { 1.0 } else { 0.0 };
                    let BEG;
                    let GJC;
                    if BEC != 0.0 {
                        let BED = -1e-6f64 / BEB;
                        let HDC = ((HCZ * BED) * GRP) / BEB;
                        BEG = BED;
                        GJC = HDC;
                    } else {
                        let HDA = HCZ * BEB;
                        let BEE = ((BEB * BEB) + 4e-6f64).sqrt();
                        let BEF = AGZ * (BEB + BEE);
                        let HDB = (HCZ + ((HDA + HDA) * (GHB / (GRJ * BEE)))) * AGZ;
                        BEG = BEF;
                        GJC = HDB;
                    }
                    let BEH = BDU * BEG;
                    let HDD = GJC * BDU;
                    CGQ = BEH;
                    GJB = HDD;
                }
                let DIG;
                let GJD;
                if OU != 0.0 {
                    let BEX;
                    let GJE;
                    if BDT != 0.0 {
                        let BEJ = -BEI;
                        let BEL = -BEK;
                        let HDM = GHH * BEL;
                        let BEM = ((BEL * AWN) - BEJ) - S;
                        let HDN = HDM * BEM;
                        let BEN = ((BEM * BEM) - ((Q * BEJ) * S)).sqrt();
                        let HDO = (HDM + ((HDN + HDN) * (GHB / (GRJ * BEN)))) * AGZ;
                        let BEO = BEI + (BEJ + (AGZ * (BEM + BEN)));
                        BEX = BEO;
                        GJE = HDO;
                    } else {
                        let BEP = -BEK;
                        let HDH = GHH * BEP;
                        let BEQ = (B + (BEP * AWN)) - S;
                        let BER = if BEQ < -1e1f64 { 1.0 } else { 0.0 };
                        let BEV;
                        let GJF;
                        if BER != 0.0 {
                            let BES = -1e-6f64 / BEQ;
                            let HDK = ((HDH * BES) * GRP) / BEQ;
                            BEV = BES;
                            GJF = HDK;
                        } else {
                            let HDI = HDH * BEQ;
                            let BET = ((BEQ * BEQ) + 4e-6f64).sqrt();
                            let BEU = AGZ * (BEQ + BET);
                            let HDJ = (HDH + ((HDI + HDI) * (GHB / (GRJ * BET)))) * AGZ;
                            BEV = BEU;
                            GJF = HDJ;
                        }
                        let BEW = BEI * BEV;
                        let HDL = GJF * BEI;
                        BEX = BEW;
                        GJE = HDL;
                    }
                    let BEZ = if BEX < BEY { 1.0 } else { 0.0 };
                    let DIH;
                    let GJG;
                    if BEZ != 0.0 {
                        DIH = BEY;
                        GJG = GRF;
                    } else {
                        DIH = BEX;
                        GJG = GJE;
                    }
                    DIG = DIH;
                    GJD = GJG;
                } else {
                    DIG = A;
                    GJD = GRF;
                }
                let ECZ;
                let EGV;
                let EHG;
                if PT != 0.0 {
                    let BFF = BFA * ((BFD * BBI).exp());
                    let BFI = BFG * AWN;
                    let BFJ = BFF + ((-9e-1f64 * BFF) + (AGZ * (((BFI - (-9e-1f64 * BFF)) - AEX) + (((((BFI - (-9e-1f64 * BFF)) - AEX) * ((BFI - (-9e-1f64 * BFF)) - AEX)) - ((Q * (-9e-1f64 * BFF)) * AEX)).sqrt()))));
                    let BFM = -BFK;
                    let BFP = ((BFN * AWN) - BFM) - S;
                    let BFQ = BFK + (BFM + (AGZ * (BFP + (((BFP * BFP) - ((Q * BFM) * S)).sqrt()))));
                    let BFV = BFR * ((BFT * BBI).exp());
                    ECZ = BFJ;
                    EGV = BFQ;
                    EHG = BFV;
                } else {
                    ECZ = EDA;
                    EGV = EGW;
                    EHG = EHH;
                }
                let CGW;
                let GJH;
                if BDT != 0.0 {
                    let BFX = -BFW;
                    let BFY = -NI;
                    let HDU = GHH * BFY;
                    let BFZ = ((BFY * AWN) - BFX) - S;
                    let HDV = HDU * BFZ;
                    let BGA = ((BFZ * BFZ) - ((Q * BFX) * S)).sqrt();
                    let HDW = (HDU + ((HDV + HDV) * (GHB / (GRJ * BGA)))) * AGZ;
                    let BGB = BFW + (BFX + (AGZ * (BFZ + BGA)));
                    CGW = BGB;
                    GJH = HDW;
                } else {
                    let BGC = -NI;
                    let HDP = GHH * BGC;
                    let BGD = (B + (BGC * AWN)) - S;
                    let BGE = if BGD < -1e1f64 { 1.0 } else { 0.0 };
                    let BGI;
                    let GJI;
                    if BGE != 0.0 {
                        let BGF = -1e-6f64 / BGD;
                        let HDS = ((HDP * BGF) * GRP) / BGD;
                        BGI = BGF;
                        GJI = HDS;
                    } else {
                        let HDQ = HDP * BGD;
                        let BGG = ((BGD * BGD) + 4e-6f64).sqrt();
                        let BGH = AGZ * (BGD + BGG);
                        let HDR = (HDP + ((HDQ + HDQ) * (GHB / (GRJ * BGG)))) * AGZ;
                        BGI = BGH;
                        GJI = HDR;
                    }
                    let BGJ = BFW * BGI;
                    let HDT = GJI * BFW;
                    CGW = BGJ;
                    GJH = HDT;
                }
                let DHF;
                let GJJ;
                if OU != 0.0 {
                    let BGY;
                    let GJK;
                    if BDT != 0.0 {
                        let BGL = -BGK;
                        let BGM = -NI;
                        let HEC = GHH * BGM;
                        let BGN = ((BGM * AWN) - BGL) - S;
                        let HED = HEC * BGN;
                        let BGO = ((BGN * BGN) - ((Q * BGL) * S)).sqrt();
                        let HEE = (HEC + ((HED + HED) * (GHB / (GRJ * BGO)))) * AGZ;
                        let BGP = BGK + (BGL + (AGZ * (BGN + BGO)));
                        BGY = BGP;
                        GJK = HEE;
                    } else {
                        let BGQ = -NI;
                        let HDX = GHH * BGQ;
                        let BGR = (B + (BGQ * AWN)) - S;
                        let BGS = if BGR < -1e1f64 { 1.0 } else { 0.0 };
                        let BGW;
                        let GJL;
                        if BGS != 0.0 {
                            let BGT = -1e-6f64 / BGR;
                            let HEA = ((HDX * BGT) * GRP) / BGR;
                            BGW = BGT;
                            GJL = HEA;
                        } else {
                            let HDY = HDX * BGR;
                            let BGU = ((BGR * BGR) + 4e-6f64).sqrt();
                            let BGV = AGZ * (BGR + BGU);
                            let HDZ = (HDX + ((HDY + HDY) * (GHB / (GRJ * BGU)))) * AGZ;
                            BGW = BGV;
                            GJL = HDZ;
                        }
                        let BGX = BGK * BGW;
                        let HEB = GJL * BGK;
                        BGY = BGX;
                        GJK = HEB;
                    }
                    let BGZ = if BGY < BEY { 1.0 } else { 0.0 };
                    let DHG;
                    let GJM;
                    if BGZ != 0.0 {
                        DHG = BEY;
                        GJM = GRF;
                    } else {
                        DHG = BGY;
                        GJM = GJK;
                    }
                    DHF = DHG;
                    GJJ = GJM;
                } else {
                    DHF = A;
                    GJJ = GRF;
                }
                let CHC;
                if BDT != 0.0 {
                    let BHA = -XI;
                    let BHB = (((-NJ) * AWN) - BHA) - S;
                    let BHC = XI + (BHA + (AGZ * (BHB + (((BHB * BHB) - ((Q * BHA) * S)).sqrt()))));
                    CHC = BHC;
                } else {
                    let BHD = (B + ((-NJ) * AWN)) - S;
                    let BHE = if BHD < -1e1f64 { 1.0 } else { 0.0 };
                    let BHH = if BHE != 0.0 {
                        let BHF = -1e-6f64 / BHD;
                        BHF
                    } else {
                        let BHG = AGZ * (BHD + (((BHD * BHD) + 4e-6f64).sqrt()));
                        BHG
                    };
                    let BHI = XI * BHH;
                    CHC = BHI;
                }
                let HEF = (GHH * BHK) * BHJ;
                let BHL = (BHJ * (B + (BHK * AWN))) - R;
                let BHM = if BHL < -1e1f64 { 1.0 } else { 0.0 };
                let BHQ;
                let GJN;
                if BHM != 0.0 {
                    let BHN = -1e-6f64 / BHL;
                    let HEI = ((HEF * BHN) * GRP) / BHL;
                    BHQ = BHN;
                    GJN = HEI;
                } else {
                    let HEG = HEF * BHL;
                    let BHO = ((BHL * BHL) + 4e-6f64).sqrt();
                    let BHP = AGZ * (BHL + BHO);
                    let HEH = (HEF + ((HEG + HEG) * (GHB / (GRJ * BHO)))) * AGZ;
                    BHQ = BHP;
                    GJN = HEH;
                }
                let BHR = BHQ + R;
                let DGT;
                let GJO;
                if OU != 0.0 {
                    let HEJ = (GHH * BHU) * BHS;
                    let BHV = (BHS * (B + (BHU * AWN))) - R;
                    let BHW = if BHV < -1e1f64 { 1.0 } else { 0.0 };
                    let BIA;
                    let GJP;
                    if BHW != 0.0 {
                        let BHX = -1e-6f64 / BHV;
                        let HEM = ((HEJ * BHX) * GRP) / BHV;
                        BIA = BHX;
                        GJP = HEM;
                    } else {
                        let HEK = HEJ * BHV;
                        let BHY = ((BHV * BHV) + 4e-6f64).sqrt();
                        let BHZ = AGZ * (BHV + BHY);
                        let HEL = (HEJ + ((HEK + HEK) * (GHB / (GRJ * BHY)))) * AGZ;
                        BIA = BHZ;
                        GJP = HEL;
                    }
                    let BIB = BIA + R;
                    DGT = BIB;
                    GJO = GJP;
                } else {
                    DGT = A;
                    GJO = GRF;
                }
                let BID = NO + (BIC / RB);
                let BIE = BID * AWM;
                let HEN = GRG * BID;
                CGP = CGQ;
                CGV = CGW;
                CHB = CHC;
                DGS = DGT;
                DGX = BHR;
                DHE = DHF;
                DIA = WN;
                DIF = DIG;
                DIT = JB;
                DJC = DJD;
                DJH = BBW;
                DJK = DJL;
                DJP = BCS;
                DJS = DJT;
                DJX = BDD;
                DKL = BCT;
                DNA = BIE;
                DUM = BDL;
                DVP = BDR;
                ECY = ECZ;
                EGU = EGV;
                EHF = EHG;
                EOS = A;
                EOU = A;
                GID = GJB;
                GIE = GJH;
                GIF = GJO;
                GIG = GJN;
                GIH = GJJ;
                GII = GRF;
                GIJ = GJD;
                GIK = GRF;
                GIL = GIX;
                GIM = HCI;
                GIN = GIY;
                GIO = HCO;
                GIP = GIZ;
                GIQ = HCS;
                GIR = GRF;
                GIS = HEN;
                GIT = HCU;
                GIU = GJA;
                GIV = GRF;
                GIW = GRF;
            } else {
                let BIF = if AWS == B { 1.0 } else { 0.0 };
                let CGR;
                let CGX;
                let CHD;
                let DGU;
                let DGY;
                let DHH;
                let DIB;
                let DII;
                let DIU;
                let DJE;
                let DJI;
                let DJM;
                let DJQ;
                let DJU;
                let DJY;
                let DKM;
                let DNB;
                let DUN;
                let DVQ;
                let EDB;
                let EGX;
                let EHI;
                let EOT;
                let EOV;
                let GJQ;
                let GJR;
                let GJS;
                let GJT;
                let GJU;
                let GJV;
                let GJW;
                let GJX;
                let GJY;
                let GJZ;
                let GKA;
                let GKB;
                let GKC;
                let GKD;
                let GKE;
                let GKF;
                let GKG;
                let GKH;
                let GKI;
                let GKJ;
                if BIF != 0.0 {
                    let BIG = MW * AWL;
                    let BIH = MV + BIG;
                    let GYM = (GRG * MW) * BBI;
                    let BII = (BIH * BBI).exp();
                    let BIJ = BBJ * BII;
                    let GYN = ((GYM + (GTA * BIH)) * BII) * BBJ;
                    let BIL = MX * AWN;
                    let GYO = GHH * MX;
                    let BIO = (BIL - (BIN * BIJ)) - AEX;
                    let BIQ = (BIL - (BIP * BIJ)) - AEX;
                    let BIS = ((BIO * BIQ) - ((Q * (BIR * BIJ)) * AEX)).sqrt();
                    let BIT = BIJ + ((BIK * BIJ) + (AGZ * (((BIL - (BIM * BIJ)) - AEX) + BIS)));
                    let GYP = GYN + ((GYN * BIK) + (((GYO - (GYN * BIM)) + (((((GYO - (GYN * BIN)) * BIQ) + ((GYO - (GYN * BIP)) * BIO)) - (((GYN * BIR) * Q) * AEX)) * (GHB / (GRJ * BIS)))) * AGZ));
                    let BIU = if OT == B { 1.0 } else { 0.0 };
                    let DJF;
                    let GKK;
                    if BIU != 0.0 {
                        let BIV = BCA + BIG;
                        let BIW = (BIV * BBI).exp();
                        let BIX = BBY * BIW;
                        let GYQ = ((GYM + (GTA * BIV)) * BIW) * BBY;
                        let BIZ = BCE * AWN;
                        let GYR = GHH * BCE;
                        let BJC = (BIZ - (BJB * BIX)) - AEX;
                        let BJE = (BIZ - (BJD * BIX)) - AEX;
                        let BJG = ((BJC * BJE) - ((Q * (BJF * BIX)) * AEX)).sqrt();
                        let BJH = BIX + ((BIY * BIX) + (AGZ * (((BIZ - (BJA * BIX)) - AEX) + BJG)));
                        let GYS = GYQ + ((GYQ * BIY) + (((GYR - (GYQ * BJA)) + (((((GYR - (GYQ * BJB)) * BJE) + ((GYR - (GYQ * BJD)) * BJC)) - (((GYQ * BJF) * Q) * AEX)) * (GHB / (GRJ * BJG)))) * AGZ));
                        DJF = BJH;
                        GKK = GYS;
                    } else {
                        DJF = A;
                        GKK = GRF;
                    }
                    let BJI = NA * AWL;
                    let BJJ = MZ + BJI;
                    let GYT = (GRG * NA) * BBI;
                    let BJK = (BJJ * BBI).exp();
                    let BJL = BCO * BJK;
                    let GYU = ((GYT + (GTA * BJJ)) * BJK) * BCO;
                    let DJN;
                    let GKL;
                    if OU != 0.0 {
                        let BJM = BCX + BJI;
                        let BJN = (BJM * BBI).exp();
                        let BJO = BCU * BJN;
                        let GYV = ((GYT + (GTA * BJM)) * BJN) * BCU;
                        DJN = BJO;
                        GKL = GYV;
                    } else {
                        DJN = A;
                        GKL = GRF;
                    }
                    let BJP = ND * AWL;
                    let BJQ = NC + BJP;
                    let GYW = (GRG * ND) * BBI;
                    let BJR = (BJQ * BBI).exp();
                    let BJS = BDB * BJR;
                    let GYX = ((GYW + (GTA * BJQ)) * BJR) * BDB;
                    let DJV;
                    let GKM;
                    if OU != 0.0 {
                        let BJT = BDG + BJP;
                        let BJU = (BJT * BBI).exp();
                        let BJV = BDE * BJU;
                        let GYY = ((GYW + (GTA * BJT)) * BJU) * BDE;
                        DJV = BJV;
                        GKM = GYY;
                    } else {
                        DJV = A;
                        GKM = GRF;
                    }
                    let BJW = NE + (NF * AWL);
                    let BJX = (BJW * BBI).exp();
                    let BJY = BDJ * BJX;
                    let GYZ = ((((GRG * NF) * BBI) + (GTA * BJW)) * BJX) * BDJ;
                    let BJZ = JM * AWM;
                    let GZA = ((GRG * JM) * (rspice_limited_exp_derivative(BJZ))) * JL;
                    let BKA = JO * AWM;
                    let GZB = ((GRG * JO) * (rspice_limited_exp_derivative(BKA))) * JN;
                    let BKB = AGZ + (JL * ((rspice_limited_exp(BJZ)) - B));
                    let BKC = AGZ + (JN * ((rspice_limited_exp(BKA)) - B));
                    let BKD = if BDS != A { 1.0 } else { 0.0 };
                    let DKN;
                    let GKN;
                    if BKD != 0.0 {
                        let BKE = -BCT;
                        let GZH = GHH * NB;
                        let BKF = ((NB * AWN) - BKE) - S;
                        let GZI = GZH * BKF;
                        let BKG = ((BKF * BKF) - ((Q * BKE) * S)).sqrt();
                        let GZJ = (GZH + ((GZI + GZI) * (GHB / (GRJ * BKG)))) * AGZ;
                        let BKH = BCT + (BKE + (AGZ * (BKF + BKG)));
                        DKN = BKH;
                        GKN = GZJ;
                    } else {
                        let GZC = GHH * NB;
                        let BKI = (B + (NB * AWN)) - S;
                        let BKJ = if BKI < -1e1f64 { 1.0 } else { 0.0 };
                        let BKN;
                        let GKO;
                        if BKJ != 0.0 {
                            let BKK = -1e-6f64 / BKI;
                            let GZF = ((GZC * BKK) * GRP) / BKI;
                            BKN = BKK;
                            GKO = GZF;
                        } else {
                            let GZD = GZC * BKI;
                            let BKL = ((BKI * BKI) + 4e-6f64).sqrt();
                            let BKM = AGZ * (BKI + BKL);
                            let GZE = (GZC + ((GZD + GZD) * (GHB / (GRJ * BKL)))) * AGZ;
                            BKN = BKM;
                            GKO = GZE;
                        }
                        let BKO = BCT * BKN;
                        let GZG = GKO * BCT;
                        DKN = BKO;
                        GKN = GZG;
                    }
                    let EDC;
                    let EGY;
                    let EHJ;
                    if PT != 0.0 {
                        let BKR = BFA * (((BFD + (BKP * AWL)) * BBI).exp());
                        let BKS = BFG * AWN;
                        let BKT = BKR + ((-9e-1f64 * BKR) + (AGZ * (((BKS - (-9e-1f64 * BKR)) - AEX) + (((((BKS - (-9e-1f64 * BKR)) - AEX) * ((BKS - (-9e-1f64 * BKR)) - AEX)) - ((Q * (-9e-1f64 * BKR)) * AEX)).sqrt()))));
                        let BKW = BFK * (((BFN + (BKU * AWL)) * BBI).exp());
                        let BKZ = BFR * (((BFT + (BKX * AWL)) * BBI).exp());
                        EDC = BKT;
                        EGY = BKW;
                        EHJ = BKZ;
                    } else {
                        EDC = EDA;
                        EGY = EGW;
                        EHJ = EHH;
                    }
                    let BLA = if NK == NL { 1.0 } else { 0.0 };
                    let BME;
                    let GKP;
                    if BLA != 0.0 {
                        let GZW = GHH * NK;
                        let BLB = B + (NK * AWN);
                        BME = BLB;
                        GKP = GZW;
                    } else {
                        let BLC = if NM < AWK { 1.0 } else { 0.0 };
                        let BMF;
                        let GKQ;
                        if BLC != 0.0 {
                            let GZQ = GHH * NK;
                            let BLD = B + (NK * AWN);
                            let GZR = GHH * NL;
                            let BLE = NM - AWK;
                            let BLF = (B + (NL * (AWJ - NM))) + (NK * BLE);
                            let BLG = (NK - NL) * BLE;
                            let BLH = if NL < NK { 1.0 } else { 0.0 };
                            let BMG;
                            let GKR;
                            if BLH != 0.0 {
                                let BLI = BLD - BLF;
                                let GZU = (GZQ - GZR) * BLI;
                                let BLK = (AQO * BLJ) * BLJ;
                                let BLL = ((BLI * BLI) + BLK).sqrt();
                                let GZV = ((GZQ + GZR) + ((GZU + GZU) * (GHB / (GRJ * BLL)))) * AGZ;
                                let BLM = (AGZ * ((BLD + BLF) + BLL)) - (AGZ * (BLG + (((BLG * BLG) + BLK).sqrt())));
                                BMG = BLM;
                                GKR = GZV;
                            } else {
                                let BLN = BLD - BLF;
                                let GZS = (GZQ - GZR) * BLN;
                                let BLO = (AQO * BLJ) * BLJ;
                                let BLP = ((BLN * BLN) + BLO).sqrt();
                                let GZT = ((GZQ + GZR) - ((GZS + GZS) * (GHB / (GRJ * BLP)))) * AGZ;
                                let BLQ = (AGZ * ((BLD + BLF) - BLP)) - (AGZ * (BLG - (((BLG * BLG) + BLO).sqrt())));
                                BMG = BLQ;
                                GKR = GZT;
                            }
                            BMF = BMG;
                            GKQ = GKR;
                        } else {
                            let GZK = GHH * NL;
                            let BLR = B + (NL * AWN);
                            let GZL = GHH * NK;
                            let BLS = NM - AWK;
                            let BLT = (B + (NK * (AWJ - NM))) + (NL * BLS);
                            let BLU = (NL - NK) * BLS;
                            let BLV = if NL < NK { 1.0 } else { 0.0 };
                            let BMH;
                            let GKS;
                            if BLV != 0.0 {
                                let BLW = BLR - BLT;
                                let GZO = (GZK - GZL) * BLW;
                                let BLX = (AQO * BLJ) * BLJ;
                                let BLY = ((BLW * BLW) + BLX).sqrt();
                                let GZP = ((GZK + GZL) + ((GZO + GZO) * (GHB / (GRJ * BLY)))) * AGZ;
                                let BLZ = (AGZ * ((BLR + BLT) + BLY)) - (AGZ * (BLU + (((BLU * BLU) + BLX).sqrt())));
                                BMH = BLZ;
                                GKS = GZP;
                            } else {
                                let BMA = BLR - BLT;
                                let GZM = (GZK - GZL) * BMA;
                                let BMB = (AQO * BLJ) * BLJ;
                                let BMC = ((BMA * BMA) + BMB).sqrt();
                                let GZN = ((GZK + GZL) - ((GZM + GZM) * (GHB / (GRJ * BMC)))) * AGZ;
                                let BMD = (AGZ * ((BLR + BLT) - BMC)) - (AGZ * (BLU - (((BLU * BLU) + BMB).sqrt())));
                                BMH = BMD;
                                GKS = GZN;
                            }
                            BMF = BMH;
                            GKQ = GKS;
                        }
                        BME = BMF;
                        GKP = GKQ;
                    }
                    let BMI = BME - S;
                    let BMJ = if BMI < -1e1f64 { 1.0 } else { 0.0 };
                    let BMN;
                    let GKT;
                    if BMJ != 0.0 {
                        let BMK = -1e-6f64 / BMI;
                        let GZZ = ((GKP * BMK) * GRP) / BMI;
                        BMN = BMK;
                        GKT = GZZ;
                    } else {
                        let GZX = GKP * BMI;
                        let BML = ((BMI * BMI) + 4e-6f64).sqrt();
                        let BMM = AGZ * (BMI + BML);
                        let GZY = (GKP + ((GZX + GZX) * (GHB / (GRJ * BML)))) * AGZ;
                        BMN = BMM;
                        GKT = GZY;
                    }
                    let CGS;
                    let GKU;
                    if BKD != 0.0 {
                        let BMO = -BDU;
                        let BMP = -NI;
                        let BMR = BMQ * AWN;
                        let HAF = (GHH * BMP) + (((GHH * BMQ) * AWN) + (GHH * BMR));
                        let BMS = (((BMP * AWN) + (BMR * AWN)) - BMO) - S;
                        let HAG = HAF * BMS;
                        let BMT = ((BMS * BMS) - ((Q * BMO) * S)).sqrt();
                        let HAH = (HAF + ((HAG + HAG) * (GHB / (GRJ * BMT)))) * AGZ;
                        let BMU = BDU + (BMO + (AGZ * (BMS + BMT)));
                        CGS = BMU;
                        GKU = HAH;
                    } else {
                        let BMV = -NI;
                        let BMW = BMQ * AWN;
                        let HAA = (GHH * BMV) + (((GHH * BMQ) * AWN) + (GHH * BMW));
                        let BMX = ((B + (BMV * AWN)) + (BMW * AWN)) - S;
                        let BMY = if BMX < -1e1f64 { 1.0 } else { 0.0 };
                        let BNC;
                        let GKV;
                        if BMY != 0.0 {
                            let BMZ = -1e-6f64 / BMX;
                            let HAD = ((HAA * BMZ) * GRP) / BMX;
                            BNC = BMZ;
                            GKV = HAD;
                        } else {
                            let HAB = HAA * BMX;
                            let BNA = ((BMX * BMX) + 4e-6f64).sqrt();
                            let BNB = AGZ * (BMX + BNA);
                            let HAC = (HAA + ((HAB + HAB) * (GHB / (GRJ * BNA)))) * AGZ;
                            BNC = BNB;
                            GKV = HAC;
                        }
                        let BND = BDU * BNC;
                        let HAE = GKV * BDU;
                        CGS = BND;
                        GKU = HAE;
                    }
                    let DIJ;
                    let GKW;
                    if OU != 0.0 {
                        let BNT;
                        let GKX;
                        if BKD != 0.0 {
                            let BNE = -BEI;
                            let BNF = -BEK;
                            let BNG = BMQ * AWN;
                            let HAN = (GHH * BNF) + (((GHH * BMQ) * AWN) + (GHH * BNG));
                            let BNH = (((BNF * AWN) + (BNG * AWN)) - BNE) - S;
                            let HAO = HAN * BNH;
                            let BNI = ((BNH * BNH) - ((Q * BNE) * S)).sqrt();
                            let HAP = (HAN + ((HAO + HAO) * (GHB / (GRJ * BNI)))) * AGZ;
                            let BNJ = BEI + (BNE + (AGZ * (BNH + BNI)));
                            BNT = BNJ;
                            GKX = HAP;
                        } else {
                            let BNK = -BEK;
                            let BNL = BMQ * AWN;
                            let HAI = (GHH * BNK) + (((GHH * BMQ) * AWN) + (GHH * BNL));
                            let BNM = ((B + (BNK * AWN)) + (BNL * AWN)) - S;
                            let BNN = if BNM < -1e1f64 { 1.0 } else { 0.0 };
                            let BNR;
                            let GKY;
                            if BNN != 0.0 {
                                let BNO = -1e-6f64 / BNM;
                                let HAL = ((HAI * BNO) * GRP) / BNM;
                                BNR = BNO;
                                GKY = HAL;
                            } else {
                                let HAJ = HAI * BNM;
                                let BNP = ((BNM * BNM) + 4e-6f64).sqrt();
                                let BNQ = AGZ * (BNM + BNP);
                                let HAK = (HAI + ((HAJ + HAJ) * (GHB / (GRJ * BNP)))) * AGZ;
                                BNR = BNQ;
                                GKY = HAK;
                            }
                            let BNS = BEI * BNR;
                            let HAM = GKY * BEI;
                            BNT = BNS;
                            GKX = HAM;
                        }
                        let BNU = if BNT < BEY { 1.0 } else { 0.0 };
                        let DIK;
                        let GKZ;
                        if BNU != 0.0 {
                            DIK = BEY;
                            GKZ = GRF;
                        } else {
                            DIK = BNT;
                            GKZ = GKX;
                        }
                        DIJ = DIK;
                        GKW = GKZ;
                    } else {
                        DIJ = A;
                        GKW = GRF;
                    }
                    let CGY;
                    let GLA;
                    if BKD != 0.0 {
                        let BNV = -BFW;
                        let BNW = -NI;
                        let BNX = BMQ * AWN;
                        let HAV = (GHH * BNW) + (((GHH * BMQ) * AWN) + (GHH * BNX));
                        let BNY = (((BNW * AWN) + (BNX * AWN)) - BNV) - S;
                        let HAW = HAV * BNY;
                        let BNZ = ((BNY * BNY) - ((Q * BNV) * S)).sqrt();
                        let HAX = (HAV + ((HAW + HAW) * (GHB / (GRJ * BNZ)))) * AGZ;
                        let BOA = BFW + (BNV + (AGZ * (BNY + BNZ)));
                        CGY = BOA;
                        GLA = HAX;
                    } else {
                        let BOB = -NI;
                        let BOC = BMQ * AWN;
                        let HAQ = (GHH * BOB) + (((GHH * BMQ) * AWN) + (GHH * BOC));
                        let BOD = ((B + (BOB * AWN)) + (BOC * AWN)) - S;
                        let BOE = if BOD < -1e1f64 { 1.0 } else { 0.0 };
                        let BOI;
                        let GLB;
                        if BOE != 0.0 {
                            let BOF = -1e-6f64 / BOD;
                            let HAT = ((HAQ * BOF) * GRP) / BOD;
                            BOI = BOF;
                            GLB = HAT;
                        } else {
                            let HAR = HAQ * BOD;
                            let BOG = ((BOD * BOD) + 4e-6f64).sqrt();
                            let BOH = AGZ * (BOD + BOG);
                            let HAS = (HAQ + ((HAR + HAR) * (GHB / (GRJ * BOG)))) * AGZ;
                            BOI = BOH;
                            GLB = HAS;
                        }
                        let BOJ = BFW * BOI;
                        let HAU = GLB * BFW;
                        CGY = BOJ;
                        GLA = HAU;
                    }
                    let DHI;
                    let GLC;
                    if OU != 0.0 {
                        let BOZ;
                        let GLD;
                        if BKD != 0.0 {
                            let BOK = -BGK;
                            let BOL = -NI;
                            let BOM = BMQ * AWN;
                            let HBD = (GHH * BOL) + (((GHH * BMQ) * AWN) + (GHH * BOM));
                            let BON = (((BOL * AWN) + (BOM * AWN)) - BOK) - S;
                            let HBE = HBD * BON;
                            let BOO = ((BON * BON) - ((Q * BOK) * S)).sqrt();
                            let HBF = (HBD + ((HBE + HBE) * (GHB / (GRJ * BOO)))) * AGZ;
                            let BOP = BGK + (BOK + (AGZ * (BON + BOO)));
                            BOZ = BOP;
                            GLD = HBF;
                        } else {
                            let BOQ = -NI;
                            let BOR = BMQ * AWN;
                            let HAY = (GHH * BOQ) + (((GHH * BMQ) * AWN) + (GHH * BOR));
                            let BOS = ((B + (BOQ * AWN)) + (BOR * AWN)) - S;
                            let BOT = if BOS < -1e1f64 { 1.0 } else { 0.0 };
                            let BOX;
                            let GLE;
                            if BOT != 0.0 {
                                let BOU = -1e-6f64 / BOS;
                                let HBB = ((HAY * BOU) * GRP) / BOS;
                                BOX = BOU;
                                GLE = HBB;
                            } else {
                                let HAZ = HAY * BOS;
                                let BOV = ((BOS * BOS) + 4e-6f64).sqrt();
                                let BOW = AGZ * (BOS + BOV);
                                let HBA = (HAY + ((HAZ + HAZ) * (GHB / (GRJ * BOV)))) * AGZ;
                                BOX = BOW;
                                GLE = HBA;
                            }
                            let BOY = BGK * BOX;
                            let HBC = GLE * BGK;
                            BOZ = BOY;
                            GLD = HBC;
                        }
                        let BPA = if BOZ < BEY { 1.0 } else { 0.0 };
                        let DHJ;
                        let GLF;
                        if BPA != 0.0 {
                            DHJ = BEY;
                            GLF = GRF;
                        } else {
                            DHJ = BOZ;
                            GLF = GLD;
                        }
                        DHI = DHJ;
                        GLC = GLF;
                    } else {
                        DHI = A;
                        GLC = GRF;
                    }
                    let CHE;
                    if BKD != 0.0 {
                        let BPB = -XI;
                        let BPD = ((((-NJ) * AWN) + ((BPC * AWN) * AWN)) - BPB) - S;
                        let BPE = XI + (BPB + (AGZ * (BPD + (((BPD * BPD) - ((Q * BPB) * S)).sqrt()))));
                        CHE = BPE;
                    } else {
                        let BPF = ((B + ((-NJ) * AWN)) + ((BPC * AWN) * AWN)) - S;
                        let BPG = if BPF < -1e1f64 { 1.0 } else { 0.0 };
                        let BPJ = if BPG != 0.0 {
                            let BPH = -1e-6f64 / BPF;
                            BPH
                        } else {
                            let BPI = AGZ * (BPF + (((BPF * BPF) + 4e-6f64).sqrt()));
                            BPI
                        };
                        let BPK = XI * BPJ;
                        CHE = BPK;
                    }
                    let BPM = BPL * AWN;
                    let BPN = BPM * AWN;
                    let HBG = ((GHH * BPL) * AWN) + (GHH * BPM);
                    let HBH = ((GHH * BHK) + HBG) * BHJ;
                    let BPO = (BHJ * ((B + (BHK * AWN)) + BPN)) - R;
                    let BPP = if BPO < -1e1f64 { 1.0 } else { 0.0 };
                    let BPT;
                    let GLG;
                    if BPP != 0.0 {
                        let BPQ = -1e-6f64 / BPO;
                        let HBK = ((HBH * BPQ) * GRP) / BPO;
                        BPT = BPQ;
                        GLG = HBK;
                    } else {
                        let HBI = HBH * BPO;
                        let BPR = ((BPO * BPO) + 4e-6f64).sqrt();
                        let BPS = AGZ * (BPO + BPR);
                        let HBJ = (HBH + ((HBI + HBI) * (GHB / (GRJ * BPR)))) * AGZ;
                        BPT = BPS;
                        GLG = HBJ;
                    }
                    let BPU = BPT + R;
                    let DGV;
                    let GLH;
                    if OU != 0.0 {
                        let HBL = ((GHH * BHU) + HBG) * BHS;
                        let BPV = (BHS * ((B + (BHU * AWN)) + BPN)) - R;
                        let BPW = if BPV < -1e1f64 { 1.0 } else { 0.0 };
                        let BQA;
                        let GLI;
                        if BPW != 0.0 {
                            let BPX = -1e-6f64 / BPV;
                            let HBO = ((HBL * BPX) * GRP) / BPV;
                            BQA = BPX;
                            GLI = HBO;
                        } else {
                            let HBM = HBL * BPV;
                            let BPY = ((BPV * BPV) + 4e-6f64).sqrt();
                            let BPZ = AGZ * (BPV + BPY);
                            let HBN = (HBL + ((HBM + HBM) * (GHB / (GRJ * BPY)))) * AGZ;
                            BQA = BPZ;
                            GLI = HBN;
                        }
                        let BQB = BQA + R;
                        DGV = BQB;
                        GLH = GLI;
                    } else {
                        DGV = A;
                        GLH = GRF;
                    }
                    let DIV;
                    let GLJ;
                    if BKD != 0.0 {
                        let BQC = -JB;
                        let BQF = BQE * AWN;
                        let HBU = (GHH * BQD) + (((GHH * BQE) * AWN) + (GHH * BQF));
                        let BQG = (((BQD * AWN) + (BQF * AWN)) - BQC) - S;
                        let HBV = HBU * BQG;
                        let BQH = ((BQG * BQG) - ((Q * BQC) * S)).sqrt();
                        let HBW = (HBU + ((HBV + HBV) * (GHB / (GRJ * BQH)))) * AGZ;
                        let BQI = JB + (BQC + (AGZ * (BQG + BQH)));
                        DIV = BQI;
                        GLJ = HBW;
                    } else {
                        let BQJ = BQE * AWN;
                        let HBP = (GHH * BQD) + (((GHH * BQE) * AWN) + (GHH * BQJ));
                        let BQK = ((B + (BQD * AWN)) + (BQJ * AWN)) - S;
                        let BQL = if BQK < -1e1f64 { 1.0 } else { 0.0 };
                        let BQP;
                        let GLK;
                        if BQL != 0.0 {
                            let BQM = -1e-6f64 / BQK;
                            let HBS = ((HBP * BQM) * GRP) / BQK;
                            BQP = BQM;
                            GLK = HBS;
                        } else {
                            let HBQ = HBP * BQK;
                            let BQN = ((BQK * BQK) + 4e-6f64).sqrt();
                            let BQO = AGZ * (BQK + BQN);
                            let HBR = (HBP + ((HBQ + HBQ) * (GHB / (GRJ * BQN)))) * AGZ;
                            BQP = BQO;
                            GLK = HBR;
                        }
                        let BQQ = JB * BQP;
                        let HBT = GLK * JB;
                        DIV = BQQ;
                        GLJ = HBT;
                    }
                    let DIC;
                    let GLL;
                    if BKD != 0.0 {
                        let BQR = -WN;
                        let HCC = GHH * BQS;
                        let BQT = ((BQS * AWN) - BQR) - S;
                        let HCD = HCC * BQT;
                        let BQU = ((BQT * BQT) - ((Q * BQR) * S)).sqrt();
                        let HCE = (HCC + ((HCD + HCD) * (GHB / (GRJ * BQU)))) * AGZ;
                        let BQV = WN + (BQR + (AGZ * (BQT + BQU)));
                        DIC = BQV;
                        GLL = HCE;
                    } else {
                        let HBX = GHH * BQS;
                        let BQW = (B + (BQS * AWN)) - S;
                        let BQX = if BQW < -1e1f64 { 1.0 } else { 0.0 };
                        let BRB;
                        let GLM;
                        if BQX != 0.0 {
                            let BQY = -1e-6f64 / BQW;
                            let HCA = ((HBX * BQY) * GRP) / BQW;
                            BRB = BQY;
                            GLM = HCA;
                        } else {
                            let HBY = HBX * BQW;
                            let BQZ = ((BQW * BQW) + 4e-6f64).sqrt();
                            let BRA = AGZ * (BQW + BQZ);
                            let HBZ = (HBX + ((HBY + HBY) * (GHB / (GRJ * BQZ)))) * AGZ;
                            BRB = BRA;
                            GLM = HBZ;
                        }
                        let BRC = WN * BRB;
                        let HCB = GLM * WN;
                        DIC = BRC;
                        GLL = HCB;
                    }
                    let BRD = NO + (BIC / RB);
                    let BRH = BRF * (AWJ - BRG);
                    let BRI = B + (rspice_limited_exp(BRH));
                    let BRJ = BRE / BRI;
                    let HCF = (GRG * BRD) + (((((GHH * BRF) * (rspice_limited_exp_derivative(BRH))) * BRJ) * GRP) / BRI);
                    let BRK = ((BRD * AWM) + BRJ) - (BRE / (B + (rspice_limited_exp((BRF * (AWK - BRG))))));
                    CGR = CGS;
                    CGX = CGY;
                    CHD = CHE;
                    DGU = DGV;
                    DGY = BPU;
                    DHH = DHI;
                    DIB = DIC;
                    DII = DIJ;
                    DIU = DIV;
                    DJE = DJF;
                    DJI = BIT;
                    DJM = DJN;
                    DJQ = BJL;
                    DJU = DJV;
                    DJY = BJS;
                    DKM = DKN;
                    DNB = BRK;
                    DUN = BJY;
                    DVQ = BMN;
                    EDB = EDC;
                    EGX = EGY;
                    EHI = EHJ;
                    EOT = BKB;
                    EOV = BKC;
                    GJQ = GKU;
                    GJR = GLA;
                    GJS = GLH;
                    GJT = GLG;
                    GJU = GLC;
                    GJV = GLL;
                    GJW = GKW;
                    GJX = GLJ;
                    GJY = GKK;
                    GJZ = GYP;
                    GKA = GKL;
                    GKB = GYU;
                    GKC = GKM;
                    GKD = GYX;
                    GKE = GKN;
                    GKF = HCF;
                    GKG = GYZ;
                    GKH = GKT;
                    GKI = GZA;
                    GKJ = GZB;
                } else {
                    let BRN = MW * BRL;
                    let BRO = MV + BRN;
                    let GTB = (GHJ * MW) * BBI;
                    let BRP = (BRO * BBI).exp();
                    let BRQ = BBJ * BRP;
                    let GTC = ((GTB + (GTA * BRO)) * BRP) * BBJ;
                    let BRS = MX * AWN;
                    let GTD = GHH * MX;
                    let BRV = (BRS - (BRU * BRQ)) - AEX;
                    let BRX = (BRS - (BRW * BRQ)) - AEX;
                    let BRZ = ((BRV * BRX) - ((Q * (BRY * BRQ)) * AEX)).sqrt();
                    let BSA = BRQ + ((BRR * BRQ) + (AGZ * (((BRS - (BRT * BRQ)) - AEX) + BRZ)));
                    let GTE = GTC + ((GTC * BRR) + (((GTD - (GTC * BRT)) + (((((GTD - (GTC * BRU)) * BRX) + ((GTD - (GTC * BRW)) * BRV)) - (((GTC * BRY) * Q) * AEX)) * (GHB / (GRJ * BRZ)))) * AGZ));
                    let BSB = if OT == B { 1.0 } else { 0.0 };
                    let DJG;
                    let GLN;
                    if BSB != 0.0 {
                        let BSC = BCA + BRN;
                        let BSD = (BSC * BBI).exp();
                        let BSE = BBY * BSD;
                        let GTF = ((GTB + (GTA * BSC)) * BSD) * BBY;
                        let BSG = BCE * AWN;
                        let GTG = GHH * BCE;
                        let BSJ = (BSG - (BSI * BSE)) - AEX;
                        let BSL = (BSG - (BSK * BSE)) - AEX;
                        let BSN = ((BSJ * BSL) - ((Q * (BSM * BSE)) * AEX)).sqrt();
                        let BSO = BSE + ((BSF * BSE) + (AGZ * (((BSG - (BSH * BSE)) - AEX) + BSN)));
                        let GTH = GTF + ((GTF * BSF) + (((GTG - (GTF * BSH)) + (((((GTG - (GTF * BSI)) * BSL) + ((GTG - (GTF * BSK)) * BSJ)) - (((GTF * BSM) * Q) * AEX)) * (GHB / (GRJ * BSN)))) * AGZ));
                        DJG = BSO;
                        GLN = GTH;
                    } else {
                        DJG = A;
                        GLN = GRF;
                    }
                    let BSP = if AWK > AXS { 1.0 } else { 0.0 };
                    let BTP;
                    let BTS;
                    let GLO;
                    let GLP;
                    if BSP != 0.0 {
                        let BSQ = AXS / AWK;
                        let BSR = if BSQ > BZ { 1.0 } else { 0.0 };
                        let BSV = if BSR != 0.0 {
                            let BSS = BSQ.ln();
                            BSS
                        } else {
                            BST
                        };
                        let BSU = BCO + (MZ * (AXS - AWK));
                        let BSW = AXS * ((MZ / BSU) - ((NA * (BSV + B)) / AWK));
                        let BSX = BSU / (BSQ.powf((BSW + (NA * BSQ))));
                        let BSY = BSW + (NA * AWL);
                        let BSZ = AWL.powf(BSY);
                        let BTA = BSX * BSZ;
                        let GTK = ((GRG * (BSY * (AWL.powf((BSY - GHB))))) + ((GRG * NA) * (BSZ * BBI))) * BSX;
                        let GTL = GHH * MZ;
                        let BTB = BCO + (MZ * AWN);
                        BTP = BTA;
                        BTS = BTB;
                        GLO = GTK;
                        GLP = GTL;
                    } else {
                        let BTC = AXS / AWK;
                        let BTD = if BTC > BZ { 1.0 } else { 0.0 };
                        let BTH = if BTD != 0.0 {
                            let BTE = BTC.ln();
                            BTE
                        } else {
                            BTF
                        };
                        let BTG = BCO * (BTC.powf((MZ + (NA * BTC))));
                        let BTI = BTG * ((MZ / AXS) + ((NA * (BTH + B)) / AWK));
                        let BTJ = MZ + (NA * AWL);
                        let BTK = AWL.powf(BTJ);
                        let BTL = BCO * BTK;
                        let GTI = ((GRG * (BTJ * (AWL.powf((BTJ - GHB))))) + ((GRG * NA) * (BTK * BBI))) * BCO;
                        let GTJ = GHH * BTI;
                        let BTM = (BTG - (BTI * (AXS - AWK))) + (BTI * AWN);
                        BTP = BTL;
                        BTS = BTM;
                        GLO = GTI;
                        GLP = GTJ;
                    }
                    let BTT = (BTN * BTP) + (BTQ * BTS);
                    let GTM = ((GHK * BTP) + (GLO * BTN)) + ((GHL * BTS) + (GLP * BTQ));
                    let BTU = if BTT < -1e-2f64 { 1.0 } else { 0.0 };
                    let BTY;
                    let GLQ;
                    if BTU != 0.0 {
                        let BTV = -1e-12f64 / BTT;
                        let GTP = ((GTM * BTV) * GRP) / BTT;
                        BTY = BTV;
                        GLQ = GTP;
                    } else {
                        let GTN = GTM * BTT;
                        let BTW = ((BTT * BTT) + 4e-12f64).sqrt();
                        let BTX = AGZ * (BTT + BTW);
                        let GTO = (GTM + ((GTN + GTN) * (GHB / (GRJ * BTW)))) * AGZ;
                        BTY = BTX;
                        GLQ = GTO;
                    }
                    let DJO;
                    let GLR;
                    if OU != 0.0 {
                        let BUW;
                        let BUX;
                        let GLS;
                        let GLT;
                        if BSP != 0.0 {
                            let BTZ = AXS / AWK;
                            let BUA = if BTZ > BZ { 1.0 } else { 0.0 };
                            let BUE = if BUA != 0.0 {
                                let BUB = BTZ.ln();
                                BUB
                            } else {
                                BUC
                            };
                            let BUD = BCU + (BCX * (AXS - AWK));
                            let BUF = AXS * ((BCX / BUD) - ((NA * (BUE + B)) / AWK));
                            let BUG = BUD / (BTZ.powf((BUF + (NA * BTZ))));
                            let BUH = BUF + (NA * AWL);
                            let BUI = AWL.powf(BUH);
                            let BUJ = BUG * BUI;
                            let GTS = ((GRG * (BUH * (AWL.powf((BUH - GHB))))) + ((GRG * NA) * (BUI * BBI))) * BUG;
                            let GTT = GHH * BCX;
                            let BUK = BCU + (BCX * AWN);
                            BUW = BUJ;
                            BUX = BUK;
                            GLS = GTS;
                            GLT = GTT;
                        } else {
                            let BUL = AXS / AWK;
                            let BUM = if BUL > BZ { 1.0 } else { 0.0 };
                            let BUQ = if BUM != 0.0 {
                                let BUN = BUL.ln();
                                BUN
                            } else {
                                BUO
                            };
                            let BUP = BCU * (BUL.powf((BCX + (NA * BUL))));
                            let BUR = BUP * ((BCX / AXS) + ((NA * (BUQ + B)) / AWK));
                            let BUS = BCX + (NA * AWL);
                            let BUT = AWL.powf(BUS);
                            let BUU = BCU * BUT;
                            let GTQ = ((GRG * (BUS * (AWL.powf((BUS - GHB))))) + ((GRG * NA) * (BUT * BBI))) * BCU;
                            let GTR = GHH * BUR;
                            let BUV = (BUP - (BUR * (AXS - AWK))) + (BUR * AWN);
                            BUW = BUU;
                            BUX = BUV;
                            GLS = GTQ;
                            GLT = GTR;
                        }
                        let BUY = (BTN * BUW) + (BTQ * BUX);
                        let GTU = ((GHK * BUW) + (GLS * BTN)) + ((GHL * BUX) + (GLT * BTQ));
                        let BUZ = if BUY < -1e-2f64 { 1.0 } else { 0.0 };
                        let BVD;
                        let GLU;
                        if BUZ != 0.0 {
                            let BVA = -1e-12f64 / BUY;
                            let GTX = ((GTU * BVA) * GRP) / BUY;
                            BVD = BVA;
                            GLU = GTX;
                        } else {
                            let GTV = GTU * BUY;
                            let BVB = ((BUY * BUY) + 4e-12f64).sqrt();
                            let BVC = AGZ * (BUY + BVB);
                            let GTW = (GTU + ((GTV + GTV) * (GHB / (GRJ * BVB)))) * AGZ;
                            BVD = BVC;
                            GLU = GTW;
                        }
                        DJO = BVD;
                        GLR = GLU;
                    } else {
                        DJO = A;
                        GLR = GRF;
                    }
                    let BVE = ND * BRL;
                    let BVF = NC + BVE;
                    let GTY = (GHJ * ND) * BBI;
                    let BVG = (BVF * BBI).exp();
                    let BVH = BDB * BVG;
                    let GTZ = ((GTY + (GTA * BVF)) * BVG) * BDB;
                    let DJW;
                    let GLV;
                    if OU != 0.0 {
                        let BVI = BDG + BVE;
                        let BVJ = (BVI * BBI).exp();
                        let BVK = BDE * BVJ;
                        let GUA = ((GTY + (GTA * BVI)) * BVJ) * BDE;
                        DJW = BVK;
                        GLV = GUA;
                    } else {
                        DJW = A;
                        GLV = GRF;
                    }
                    let BVL = NE + (NG * BRL);
                    let BVM = (BVL * BBI).exp();
                    let BVN = BDJ * BVM;
                    let GUB = ((((GHJ * NG) * BBI) + (GTA * BVL)) * BVM) * BDJ;
                    let BVO = AWK - AXS;
                    let BVP = (JM * BVO) / AWK;
                    let BVQ = if (BVP.abs()) < S { 1.0 } else { 0.0 };
                    let BWD;
                    let GLW;
                    if BVQ != 0.0 {
                        let BVR = JM * BRL;
                        let BVS = JL * ((rspice_limited_exp(BVR)) - B);
                        let GUD = ((GHJ * JM) * (rspice_limited_exp_derivative(BVR))) * JL;
                        BWD = BVS;
                        GLW = GUD;
                    } else {
                        let BVT = JM * BRL;
                        let BVU = ((rspice_limited_exp(BVP)) - B).abs();
                        let BVV = (JL * ((rspice_limited_exp(BVT)) - B)) / BVU;
                        let GUC = (((GHJ * JM) * (rspice_limited_exp_derivative(BVT))) * JL) / BVU;
                        BWD = BVV;
                        GLW = GUC;
                    }
                    let BVW = (JO * BVO) / AWK;
                    let BVX = if (BVW.abs()) < S { 1.0 } else { 0.0 };
                    let BWF;
                    let GLX;
                    if BVX != 0.0 {
                        let BVY = JO * BRL;
                        let BVZ = JN * ((rspice_limited_exp(BVY)) - B);
                        let GUF = ((GHJ * JO) * (rspice_limited_exp_derivative(BVY))) * JN;
                        BWF = BVZ;
                        GLX = GUF;
                    } else {
                        let BWA = JO * BRL;
                        let BWB = ((rspice_limited_exp(BVW)) - B).abs();
                        let BWC = (JN * ((rspice_limited_exp(BWA)) - B)) / BWB;
                        let GUE = (((GHJ * JO) * (rspice_limited_exp_derivative(BWA))) * JN) / BWB;
                        BWF = BWC;
                        GLX = GUE;
                    }
                    let BWE = AGZ + BWD;
                    let BWG = AGZ + BWF;
                    let BWH = if BDS != A { 1.0 } else { 0.0 };
                    let CGT;
                    let CGZ;
                    let CHF;
                    let DHK;
                    let DID;
                    let DIL;
                    let DIW;
                    let DKO;
                    let GLY;
                    let GLZ;
                    let GMA;
                    let GMB;
                    let GMC;
                    let GMD;
                    let GME;
                    if BWH != 0.0 {
                        let BWI = -BCT;
                        let GVO = GHM * NB;
                        let BWL = ((NB * BWJ) - BWI) - S;
                        let GVP = GVO * BWL;
                        let BWM = ((BWL * BWL) - ((Q * BWI) * S)).sqrt();
                        let GVQ = (GVO + ((GVP + GVP) * (GHB / (GRJ * BWM)))) * AGZ;
                        let BWN = BCT + (BWI + (AGZ * (BWL + BWM)));
                        let BWO = -BDU;
                        let BWP = -NI;
                        let BWQ = BMQ * BWJ;
                        let BWR = BWQ * BWJ;
                        let GVR = ((GHM * BMQ) * BWJ) + (GHM * BWQ);
                        let BWS = (BWP * AWN) + BWR;
                        let GVS = (GHH * BWP) + GVR;
                        let BWT = (BWS - BWO) - S;
                        let GVT = GVS * BWT;
                        let BWU = ((BWT * BWT) - ((Q * BWO) * S)).sqrt();
                        let GVU = (GVS + ((GVT + GVT) * (GHB / (GRJ * BWU)))) * AGZ;
                        let BWV = BDU + (BWO + (AGZ * (BWT + BWU)));
                        let DIM;
                        let GMF;
                        if OU != 0.0 {
                            let BWW = -BEI;
                            let BWX = -BEK;
                            let GVV = (GHH * BWX) + GVR;
                            let BWY = (((BWX * AWN) + BWR) - BWW) - S;
                            let GVW = GVV * BWY;
                            let BWZ = ((BWY * BWY) - ((Q * BWW) * S)).sqrt();
                            let GVX = (GVV + ((GVW + GVW) * (GHB / (GRJ * BWZ)))) * AGZ;
                            let BXA = BEI + (BWW + (AGZ * (BWY + BWZ)));
                            let BXB = if BXA < BEY { 1.0 } else { 0.0 };
                            let DIN;
                            let GMG;
                            if BXB != 0.0 {
                                DIN = BEY;
                                GMG = GRF;
                            } else {
                                DIN = BXA;
                                GMG = GVX;
                            }
                            DIM = DIN;
                            GMF = GMG;
                        } else {
                            DIM = A;
                            GMF = GRF;
                        }
                        let BXC = -BFW;
                        let BXD = (BWS - BXC) - S;
                        let GVY = GVS * BXD;
                        let BXE = ((BXD * BXD) - ((Q * BXC) * S)).sqrt();
                        let GVZ = (GVS + ((GVY + GVY) * (GHB / (GRJ * BXE)))) * AGZ;
                        let BXF = BFW + (BXC + (AGZ * (BXD + BXE)));
                        let DHL;
                        let GMH;
                        if OU != 0.0 {
                            let BXG = -BGK;
                            let BXH = (BWS - BXG) - S;
                            let GWA = GVS * BXH;
                            let BXI = ((BXH * BXH) - ((Q * BXG) * S)).sqrt();
                            let GWB = (GVS + ((GWA + GWA) * (GHB / (GRJ * BXI)))) * AGZ;
                            let BXJ = BGK + (BXG + (AGZ * (BXH + BXI)));
                            let BXK = if BXJ < BEY { 1.0 } else { 0.0 };
                            let DHM;
                            let GMI;
                            if BXK != 0.0 {
                                DHM = BEY;
                                GMI = GRF;
                            } else {
                                DHM = BXJ;
                                GMI = GWB;
                            }
                            DHL = DHM;
                            GMH = GMI;
                        } else {
                            DHL = A;
                            GMH = GRF;
                        }
                        let BXL = -XI;
                        let BXM = ((((-NJ) * AWN) + ((BPC * BWJ) * BWJ)) - BXL) - S;
                        let BXN = XI + (BXL + (AGZ * (BXM + (((BXM * BXM) - ((Q * BXL) * S)).sqrt()))));
                        let BXO = -JB;
                        let BXP = BQE * BWJ;
                        let GWC = (GHM * BQD) + (((GHM * BQE) * BWJ) + (GHM * BXP));
                        let BXQ = (((BQD * BWJ) + (BXP * BWJ)) - BXO) - S;
                        let GWD = GWC * BXQ;
                        let BXR = ((BXQ * BXQ) - ((Q * BXO) * S)).sqrt();
                        let GWE = (GWC + ((GWD + GWD) * (GHB / (GRJ * BXR)))) * AGZ;
                        let BXS = JB + (BXO + (AGZ * (BXQ + BXR)));
                        let BXT = -WN;
                        let GWF = GHM * BQS;
                        let BXU = ((BQS * BWJ) - BXT) - S;
                        let GWG = GWF * BXU;
                        let BXV = ((BXU * BXU) - ((Q * BXT) * S)).sqrt();
                        let GWH = (GWF + ((GWG + GWG) * (GHB / (GRJ * BXV)))) * AGZ;
                        let BXW = WN + (BXT + (AGZ * (BXU + BXV)));
                        CGT = BWV;
                        CGZ = BXF;
                        CHF = BXN;
                        DHK = DHL;
                        DID = BXW;
                        DIL = DIM;
                        DIW = BXS;
                        DKO = BWN;
                        GLY = GVU;
                        GLZ = GVZ;
                        GMA = GMH;
                        GMB = GWH;
                        GMC = GMF;
                        GMD = GWE;
                        GME = GVQ;
                    } else {
                        let GUG = GHM * NB;
                        let BXX = (B + (NB * BWJ)) - S;
                        let BXY = if BXX < -1e1f64 { 1.0 } else { 0.0 };
                        let BYC;
                        let GMJ;
                        if BXY != 0.0 {
                            let BXZ = -1e-6f64 / BXX;
                            let GUJ = ((GUG * BXZ) * GRP) / BXX;
                            BYC = BXZ;
                            GMJ = GUJ;
                        } else {
                            let GUH = GUG * BXX;
                            let BYA = ((BXX * BXX) + 4e-6f64).sqrt();
                            let BYB = AGZ * (BXX + BYA);
                            let GUI = (GUG + ((GUH + GUH) * (GHB / (GRJ * BYA)))) * AGZ;
                            BYC = BYB;
                            GMJ = GUI;
                        }
                        let BYD = BCT * BYC;
                        let GUK = GMJ * BCT;
                        let BYE = BMQ * BWJ;
                        let BYF = BYE * BWJ;
                        let GUL = ((GHM * BMQ) * BWJ) + (GHM * BYE);
                        let GUM = ((GHH * NI) * GRP) + GUL;
                        let BYG = ((B - (NI * AWN)) + BYF) - S;
                        let BYH = if BYG < -1e1f64 { 1.0 } else { 0.0 };
                        let BYL;
                        let GMK;
                        if BYH != 0.0 {
                            let BYI = -1e-6f64 / BYG;
                            let GUP = ((GUM * BYI) * GRP) / BYG;
                            BYL = BYI;
                            GMK = GUP;
                        } else {
                            let GUN = GUM * BYG;
                            let BYJ = ((BYG * BYG) + 4e-6f64).sqrt();
                            let BYK = AGZ * (BYG + BYJ);
                            let GUO = (GUM + ((GUN + GUN) * (GHB / (GRJ * BYJ)))) * AGZ;
                            BYL = BYK;
                            GMK = GUO;
                        }
                        let BYM = BDU * BYL;
                        let GUQ = GMK * BDU;
                        let DIO;
                        let GML;
                        if OU != 0.0 {
                            let GUR = ((GHH * BEK) * GRP) + GUL;
                            let BYN = ((B - (BEK * AWN)) + BYF) - S;
                            let BYO = if BYN < -1e1f64 { 1.0 } else { 0.0 };
                            let BYS;
                            let GMM;
                            if BYO != 0.0 {
                                let BYP = -1e-6f64 / BYN;
                                let GUU = ((GUR * BYP) * GRP) / BYN;
                                BYS = BYP;
                                GMM = GUU;
                            } else {
                                let GUS = GUR * BYN;
                                let BYQ = ((BYN * BYN) + 4e-6f64).sqrt();
                                let BYR = AGZ * (BYN + BYQ);
                                let GUT = (GUR + ((GUS + GUS) * (GHB / (GRJ * BYQ)))) * AGZ;
                                BYS = BYR;
                                GMM = GUT;
                            }
                            let BYT = BEI * BYS;
                            let GUV = GMM * BEI;
                            let BYU = if BYT < BEY { 1.0 } else { 0.0 };
                            let DIP;
                            let GMN;
                            if BYU != 0.0 {
                                DIP = BEY;
                                GMN = GRF;
                            } else {
                                DIP = BYT;
                                GMN = GUV;
                            }
                            DIO = DIP;
                            GML = GMN;
                        } else {
                            DIO = A;
                            GML = GRF;
                        }
                        let BYV = if BYG < -1e1f64 { 1.0 } else { 0.0 };
                        let BYZ;
                        let GMO;
                        if BYV != 0.0 {
                            let BYW = -1e-6f64 / BYG;
                            let GUY = ((GUM * BYW) * GRP) / BYG;
                            BYZ = BYW;
                            GMO = GUY;
                        } else {
                            let GUW = GUM * BYG;
                            let BYX = ((BYG * BYG) + 4e-6f64).sqrt();
                            let BYY = AGZ * (BYG + BYX);
                            let GUX = (GUM + ((GUW + GUW) * (GHB / (GRJ * BYX)))) * AGZ;
                            BYZ = BYY;
                            GMO = GUX;
                        }
                        let BZA = BFW * BYZ;
                        let GUZ = GMO * BFW;
                        let DHN;
                        let GMP;
                        if OU != 0.0 {
                            let BZB = if BYG < -1e1f64 { 1.0 } else { 0.0 };
                            let BZF;
                            let GMQ;
                            if BZB != 0.0 {
                                let BZC = -1e-6f64 / BYG;
                                let GVC = ((GUM * BZC) * GRP) / BYG;
                                BZF = BZC;
                                GMQ = GVC;
                            } else {
                                let GVA = GUM * BYG;
                                let BZD = ((BYG * BYG) + 4e-6f64).sqrt();
                                let BZE = AGZ * (BYG + BZD);
                                let GVB = (GUM + ((GVA + GVA) * (GHB / (GRJ * BZD)))) * AGZ;
                                BZF = BZE;
                                GMQ = GVB;
                            }
                            let BZG = BGK * BZF;
                            let GVD = GMQ * BGK;
                            let BZH = if BZG < BEY { 1.0 } else { 0.0 };
                            let DHO;
                            let GMR;
                            if BZH != 0.0 {
                                DHO = BEY;
                                GMR = GRF;
                            } else {
                                DHO = BZG;
                                GMR = GVD;
                            }
                            DHN = DHO;
                            GMP = GMR;
                        } else {
                            DHN = A;
                            GMP = GRF;
                        }
                        let BZI = ((B - (NJ * AWN)) + ((BPC * BWJ) * BWJ)) - S;
                        let BZJ = if BZI < -1e1f64 { 1.0 } else { 0.0 };
                        let BZM = if BZJ != 0.0 {
                            let BZK = -1e-6f64 / BZI;
                            BZK
                        } else {
                            let BZL = AGZ * (BZI + (((BZI * BZI) + 4e-6f64).sqrt()));
                            BZL
                        };
                        let BZN = XI * BZM;
                        let BZO = BQE * BWJ;
                        let GVE = (GHM * BQD) + (((GHM * BQE) * BWJ) + (GHM * BZO));
                        let BZP = ((B + (BQD * BWJ)) + (BZO * BWJ)) - S;
                        let BZQ = if BZP < -1e1f64 { 1.0 } else { 0.0 };
                        let BZU;
                        let GMS;
                        if BZQ != 0.0 {
                            let BZR = -1e-6f64 / BZP;
                            let GVH = ((GVE * BZR) * GRP) / BZP;
                            BZU = BZR;
                            GMS = GVH;
                        } else {
                            let GVF = GVE * BZP;
                            let BZS = ((BZP * BZP) + 4e-6f64).sqrt();
                            let BZT = AGZ * (BZP + BZS);
                            let GVG = (GVE + ((GVF + GVF) * (GHB / (GRJ * BZS)))) * AGZ;
                            BZU = BZT;
                            GMS = GVG;
                        }
                        let BZV = JB * BZU;
                        let GVI = GMS * JB;
                        let GVJ = GHM * BQS;
                        let BZW = (B + (BQS * BWJ)) - S;
                        let BZX = if BZW < -1e1f64 { 1.0 } else { 0.0 };
                        let CAB;
                        let GMT;
                        if BZX != 0.0 {
                            let BZY = -1e-6f64 / BZW;
                            let GVM = ((GVJ * BZY) * GRP) / BZW;
                            CAB = BZY;
                            GMT = GVM;
                        } else {
                            let GVK = GVJ * BZW;
                            let BZZ = ((BZW * BZW) + 4e-6f64).sqrt();
                            let CAA = AGZ * (BZW + BZZ);
                            let GVL = (GVJ + ((GVK + GVK) * (GHB / (GRJ * BZZ)))) * AGZ;
                            CAB = CAA;
                            GMT = GVL;
                        }
                        let CAC = WN * CAB;
                        let GVN = GMT * WN;
                        CGT = BYM;
                        CGZ = BZA;
                        CHF = BZN;
                        DHK = DHN;
                        DID = CAC;
                        DIL = DIO;
                        DIW = BZV;
                        DKO = BYD;
                        GLY = GUQ;
                        GLZ = GUZ;
                        GMA = GMP;
                        GMB = GVN;
                        GMC = GML;
                        GMD = GVI;
                        GME = GUK;
                    }
                    let CAD = BPL * BWJ;
                    let CAE = CAD * BWJ;
                    let GWI = ((GHM * BPL) * BWJ) + (GHM * CAD);
                    let GWJ = ((GHH * BHK) + GWI) * BHJ;
                    let CAF = (BHJ * ((B + (BHK * AWN)) + CAE)) - R;
                    let CAG = if CAF < -1e1f64 { 1.0 } else { 0.0 };
                    let CAK;
                    let GMU;
                    if CAG != 0.0 {
                        let CAH = -1e-6f64 / CAF;
                        let GWM = ((GWJ * CAH) * GRP) / CAF;
                        CAK = CAH;
                        GMU = GWM;
                    } else {
                        let GWK = GWJ * CAF;
                        let CAI = ((CAF * CAF) + 4e-6f64).sqrt();
                        let CAJ = AGZ * (CAF + CAI);
                        let GWL = (GWJ + ((GWK + GWK) * (GHB / (GRJ * CAI)))) * AGZ;
                        CAK = CAJ;
                        GMU = GWL;
                    }
                    let CAL = CAK + R;
                    let DGW;
                    let GMV;
                    if OU != 0.0 {
                        let GWN = ((GHH * BHU) + GWI) * BHS;
                        let CAM = (BHS * ((B + (BHU * AWN)) + CAE)) - R;
                        let CAN = if CAM < -1e1f64 { 1.0 } else { 0.0 };
                        let CAR;
                        let GMW;
                        if CAN != 0.0 {
                            let CAO = -1e-6f64 / CAM;
                            let GWQ = ((GWN * CAO) * GRP) / CAM;
                            CAR = CAO;
                            GMW = GWQ;
                        } else {
                            let GWO = GWN * CAM;
                            let CAP = ((CAM * CAM) + 4e-6f64).sqrt();
                            let CAQ = AGZ * (CAM + CAP);
                            let GWP = (GWN + ((GWO + GWO) * (GHB / (GRJ * CAP)))) * AGZ;
                            CAR = CAQ;
                            GMW = GWP;
                        }
                        let CAS = CAR + R;
                        DGW = CAS;
                        GMV = GMW;
                    } else {
                        DGW = A;
                        GMV = GRF;
                    }
                    let EDD;
                    let EGZ;
                    let EHK;
                    if PT != 0.0 {
                        let CAT = BFA * (((BFD + (BKP * BRL)) * BBI).exp());
                        let CAU = BFG * AWN;
                        let CAV = CAT + ((-9e-1f64 * CAT) + (AGZ * (((CAU - (-9e-1f64 * CAT)) - AEX) + (((((CAU - (-9e-1f64 * CAT)) - AEX) * ((CAU - (-9e-1f64 * CAT)) - AEX)) - ((Q * (-9e-1f64 * CAT)) * AEX)).sqrt()))));
                        let CBO;
                        let CBP;
                        if BSP != 0.0 {
                            let CAW = AXS / AWK;
                            let CAX = if CAW > BZ { 1.0 } else { 0.0 };
                            let CBB = if CAX != 0.0 {
                                let CAY = CAW.ln();
                                CAY
                            } else {
                                CAZ
                            };
                            let CBA = BFK + (BFN * (AXS - AWK));
                            let CBC = AXS * ((BFN / CBA) - ((BKU * (CBB + B)) / AWK));
                            let CBD = (CBA / (CAW.powf((CBC + (BKU * CAW))))) * (AWL.powf((CBC + (BKU * AWL))));
                            let CBE = BFK + (BFN * AWN);
                            CBO = CBD;
                            CBP = CBE;
                        } else {
                            let CBF = AXS / AWK;
                            let CBG = if CBF > BZ { 1.0 } else { 0.0 };
                            let CBK = if CBG != 0.0 {
                                let CBH = CBF.ln();
                                CBH
                            } else {
                                CBI
                            };
                            let CBJ = BFK * (CBF.powf((BFN + (BKU * CBF))));
                            let CBL = CBJ * ((BFN / AXS) + ((BKU * (CBK + B)) / AWK));
                            let CBM = BFK * (AWL.powf((BFN + (BKU * AWL))));
                            let CBN = (CBJ - (CBL * (AXS - AWK))) + (CBL * AWN);
                            CBO = CBM;
                            CBP = CBN;
                        }
                        let CBQ = (BTN * CBO) + (BTQ * CBP);
                        let CBR = if CBQ < -1e-2f64 { 1.0 } else { 0.0 };
                        let CBU = if CBR != 0.0 {
                            let CBS = -1e-12f64 / CBQ;
                            CBS
                        } else {
                            let CBT = AGZ * (CBQ + (((CBQ * CBQ) + 4e-12f64).sqrt()));
                            CBT
                        };
                        let CBV = BFR * (((BFT + (BKX * BRL)) * BBI).exp());
                        EDD = CAV;
                        EGZ = CBU;
                        EHK = CBV;
                    } else {
                        EDD = EDA;
                        EGZ = EGW;
                        EHK = EHH;
                    }
                    let CBW = if NK == NL { 1.0 } else { 0.0 };
                    let CFS;
                    let GMX;
                    if CBW != 0.0 {
                        let GYH = GHH * NK;
                        let CBX = B + (NK * AWN);
                        CFS = CBX;
                        GMX = GYH;
                    } else {
                        let CBY = if NM < AXS { 1.0 } else { 0.0 };
                        let CFT;
                        let GMY;
                        if CBY != 0.0 {
                            let CFU;
                            let GMZ;
                            if BSP != 0.0 {
                                let GXX = GHH * NK;
                                let CBZ = B + (NK * AWN);
                                let GXY = GHH * NL;
                                let CCA = NK * (NM - AWK);
                                let CCB = (B + (NL * (AWJ - NM))) + CCA;
                                let CCC = B + (NK * (AXS - AWK));
                                let CCD = (B + (NL * (AXS - NM))) + CCA;
                                let CCE = if NL < NK { 1.0 } else { 0.0 };
                                let CFV;
                                let GNA;
                                if CCE != 0.0 {
                                    let CCF = CBZ - CCB;
                                    let GYD = (GXX - GXY) * CCF;
                                    let CCG = (AQO * BLJ) * BLJ;
                                    let CCH = ((CCF * CCF) + CCG).sqrt();
                                    let GYE = ((GXX + GXY) + ((GYD + GYD) * (GHB / (GRJ * CCH)))) * AGZ;
                                    let CCI = CCC - CCD;
                                    let CCJ = ((AGZ * ((CBZ + CCB) + CCH)) - (AGZ * ((CCC + CCD) + (((CCI * CCI) + CCG).sqrt())))) + CCC;
                                    let CCK = CCJ - CBZ;
                                    let GYF = (GYE - GXX) * CCK;
                                    let CCL = ((CCK * CCK) + 2.5e-7f64).sqrt();
                                    let CCM = AGZ * ((CCJ + CBZ) + CCL);
                                    let GYG = ((GYE + GXX) + ((GYF + GYF) * (GHB / (GRJ * CCL)))) * AGZ;
                                    CFV = CCM;
                                    GNA = GYG;
                                } else {
                                    let CCN = CBZ - CCB;
                                    let GXZ = (GXX - GXY) * CCN;
                                    let CCO = (AQO * BLJ) * BLJ;
                                    let CCP = ((CCN * CCN) + CCO).sqrt();
                                    let GYA = ((GXX + GXY) - ((GXZ + GXZ) * (GHB / (GRJ * CCP)))) * AGZ;
                                    let CCQ = CCC - CCD;
                                    let CCR = ((AGZ * ((CBZ + CCB) - CCP)) - (AGZ * ((CCC + CCD) - (((CCQ * CCQ) + CCO).sqrt())))) + CCC;
                                    let CCS = CCR - CBZ;
                                    let GYB = (GYA - GXX) * CCS;
                                    let CCT = ((CCS * CCS) + 2.5e-7f64).sqrt();
                                    let CCU = AGZ * ((CCR + CBZ) - CCT);
                                    let GYC = ((GYA + GXX) - ((GYB + GYB) * (GHB / (GRJ * CCT)))) * AGZ;
                                    CFV = CCU;
                                    GNA = GYC;
                                }
                                CFU = CFV;
                                GMZ = GNA;
                            } else {
                                let CCV = if AWK > NM { 1.0 } else { 0.0 };
                                let CFW;
                                let GNB;
                                if CCV != 0.0 {
                                    let GXN = GHH * NK;
                                    let CCW = B + (NK * AWN);
                                    let GXO = GHH * NL;
                                    let CCX = NM - AWK;
                                    let CCY = NK * CCX;
                                    let CCZ = (B + (NL * (AWJ - NM))) + CCY;
                                    let CDA = (NK - NL) * CCX;
                                    let CDB = B + (NK * (AXS - AWK));
                                    let CDC = (B + (NL * (AXS - NM))) + CCY;
                                    let CDD = if NL < NK { 1.0 } else { 0.0 };
                                    let CFX;
                                    let GNC;
                                    if CDD != 0.0 {
                                        let CDE = CCW - CCZ;
                                        let GXT = (GXN - GXO) * CDE;
                                        let CDF = (AQO * BLJ) * BLJ;
                                        let CDG = ((CDE * CDE) + CDF).sqrt();
                                        let GXU = ((GXN + GXO) + ((GXT + GXT) * (GHB / (GRJ * CDG)))) * AGZ;
                                        let CDH = AGZ * (CDA + (((CDA * CDA) + CDF).sqrt()));
                                        let CDI = (AGZ * ((CCW + CCZ) + CDG)) - CDH;
                                        let CDJ = CDB - CDC;
                                        let CDK = ((AGZ * ((CDB + CDC) + (((CDJ * CDJ) + CDF).sqrt()))) - CDH) + (NK * (AWJ - AXS));
                                        let CDL = CDI - CDK;
                                        let GXV = (GXU - GXN) * CDL;
                                        let CDM = ((CDL * CDL) + 2.5e-7f64).sqrt();
                                        let CDN = AGZ * ((CDI + CDK) + CDM);
                                        let GXW = ((GXU + GXN) + ((GXV + GXV) * (GHB / (GRJ * CDM)))) * AGZ;
                                        CFX = CDN;
                                        GNC = GXW;
                                    } else {
                                        let CDO = CCW - CCZ;
                                        let GXP = (GXN - GXO) * CDO;
                                        let CDP = (AQO * BLJ) * BLJ;
                                        let CDQ = ((CDO * CDO) + CDP).sqrt();
                                        let GXQ = ((GXN + GXO) - ((GXP + GXP) * (GHB / (GRJ * CDQ)))) * AGZ;
                                        let CDR = AGZ * (CDA - (((CDA * CDA) + CDP).sqrt()));
                                        let CDS = (AGZ * ((CCW + CCZ) - CDQ)) - CDR;
                                        let CDT = CDB - CDC;
                                        let CDU = ((AGZ * ((CDB + CDC) - (((CDT * CDT) + CDP).sqrt()))) - CDR) + (NK * (AWJ - AXS));
                                        let CDV = CDS - CDU;
                                        let GXR = (GXQ - GXN) * CDV;
                                        let CDW = ((CDV * CDV) + 2.5e-7f64).sqrt();
                                        let CDX = AGZ * ((CDS + CDU) - CDW);
                                        let GXS = ((GXQ + GXN) - ((GXR + GXR) * (GHB / (GRJ * CDW)))) * AGZ;
                                        CFX = CDX;
                                        GNC = GXS;
                                    }
                                    CFW = CFX;
                                    GNB = GNC;
                                } else {
                                    let GXD = GHH * NL;
                                    let CDY = B + (NL * AWN);
                                    let GXE = GHH * NK;
                                    let CDZ = NM - AWK;
                                    let CEA = NL * CDZ;
                                    let CEB = (B + (NK * (AWJ - NM))) + CEA;
                                    let CEC = (NL - NK) * CDZ;
                                    let CED = B + (NL * (AXS - AWK));
                                    let CEE = (B + (NK * (AXS - NM))) + CEA;
                                    let CEF = if NL < NK { 1.0 } else { 0.0 };
                                    let CFY;
                                    let GND;
                                    if CEF != 0.0 {
                                        let CEG = CEB - CDY;
                                        let GXJ = (GXE - GXD) * CEG;
                                        let CEH = (AQO * BLJ) * BLJ;
                                        let CEI = ((CEG * CEG) + CEH).sqrt();
                                        let GXK = ((GXE + GXD) + ((GXJ + GXJ) * (GHB / (GRJ * CEI)))) * AGZ;
                                        let CEJ = AGZ * (CEC + (((CEC * CEC) + CEH).sqrt()));
                                        let CEK = (AGZ * ((CEB + CDY) + CEI)) - CEJ;
                                        let CEL = CED - CEE;
                                        let CEM = ((AGZ * ((CED + CEE) + (((CEL * CEL) + CEH).sqrt()))) - CEJ) + (NK * (AWJ - AXS));
                                        let CEN = CEK - CEM;
                                        let GXL = (GXK - GXE) * CEN;
                                        let CEO = ((CEN * CEN) + 2.5e-7f64).sqrt();
                                        let CEP = AGZ * ((CEK + CEM) + CEO);
                                        let GXM = ((GXK + GXE) + ((GXL + GXL) * (GHB / (GRJ * CEO)))) * AGZ;
                                        CFY = CEP;
                                        GND = GXM;
                                    } else {
                                        let CEQ = CEB - CDY;
                                        let GXF = (GXE - GXD) * CEQ;
                                        let CER = (AQO * BLJ) * BLJ;
                                        let CES = ((CEQ * CEQ) + CER).sqrt();
                                        let GXG = ((GXE + GXD) - ((GXF + GXF) * (GHB / (GRJ * CES)))) * AGZ;
                                        let CET = AGZ * (CEC - (((CEC * CEC) + CER).sqrt()));
                                        let CEU = (AGZ * ((CEB + CDY) - CES)) - CET;
                                        let CEV = CED - CEE;
                                        let CEW = ((AGZ * ((CED + CEE) - (((CEV * CEV) + CER).sqrt()))) - CET) + (NK * (AWJ - AXS));
                                        let CEX = CEU - CEW;
                                        let GXH = (GXG - GXE) * CEX;
                                        let CEY = ((CEX * CEX) + 2.5e-7f64).sqrt();
                                        let CEZ = AGZ * ((CEU + CEW) - CEY);
                                        let GXI = ((GXG + GXE) - ((GXH + GXH) * (GHB / (GRJ * CEY)))) * AGZ;
                                        CFY = CEZ;
                                        GND = GXI;
                                    }
                                    CFW = CFY;
                                    GNB = GND;
                                }
                                CFU = CFW;
                                GMZ = GNB;
                            }
                            CFT = CFU;
                            GMY = GMZ;
                        } else {
                            let CFZ;
                            let GNE;
                            if BSP != 0.0 {
                                let GWX = GHH * NK;
                                let CFA = B + (NK * AWN);
                                let GWY = GHH * NL;
                                let CFB = (B + (NL * (AWJ - AXS))) + (NK * (AXS - AWK));
                                let CFC = if NL < NK { 1.0 } else { 0.0 };
                                let CGA;
                                let GNF;
                                if CFC != 0.0 {
                                    let CFD = CFA - CFB;
                                    let GXB = (GWX - GWY) * CFD;
                                    let CFE = ((CFD * CFD) + 2.5e-5f64).sqrt();
                                    let CFF = AGZ * ((CFA + CFB) + CFE);
                                    let GXC = ((GWX + GWY) + ((GXB + GXB) * (GHB / (GRJ * CFE)))) * AGZ;
                                    CGA = CFF;
                                    GNF = GXC;
                                } else {
                                    let CFG = CFA - CFB;
                                    let GWZ = (GWX - GWY) * CFG;
                                    let CFH = ((CFG * CFG) + 2.5e-5f64).sqrt();
                                    let CFI = AGZ * ((CFA + CFB) - CFH);
                                    let GXA = ((GWX + GWY) - ((GWZ + GWZ) * (GHB / (GRJ * CFH)))) * AGZ;
                                    CGA = CFI;
                                    GNF = GXA;
                                }
                                CFZ = CGA;
                                GNE = GNF;
                            } else {
                                let GWR = GHH * NL;
                                let CFJ = B + (NL * AWN);
                                let GWS = GHH * NK;
                                let CFK = (B + (NK * (AWJ - AXS))) + (NL * (AXS - AWK));
                                let CFL = if NL < NK { 1.0 } else { 0.0 };
                                let CGB;
                                let GNG;
                                if CFL != 0.0 {
                                    let CFM = CFK - CFJ;
                                    let GWV = (GWS - GWR) * CFM;
                                    let CFN = ((CFM * CFM) + 2.5e-5f64).sqrt();
                                    let CFO = AGZ * ((CFK + CFJ) + CFN);
                                    let GWW = ((GWS + GWR) + ((GWV + GWV) * (GHB / (GRJ * CFN)))) * AGZ;
                                    CGB = CFO;
                                    GNG = GWW;
                                } else {
                                    let CFP = CFK - CFJ;
                                    let GWT = (GWS - GWR) * CFP;
                                    let CFQ = ((CFP * CFP) + 2.5e-5f64).sqrt();
                                    let CFR = AGZ * ((CFK + CFJ) - CFQ);
                                    let GWU = ((GWS + GWR) - ((GWT + GWT) * (GHB / (GRJ * CFQ)))) * AGZ;
                                    CGB = CFR;
                                    GNG = GWU;
                                }
                                CFZ = CGB;
                                GNE = GNG;
                            }
                            CFT = CFZ;
                            GMY = GNE;
                        }
                        CFS = CFT;
                        GMX = GMY;
                    }
                    let CGC = CFS - S;
                    let CGD = if CGC < -1e1f64 { 1.0 } else { 0.0 };
                    let CGH;
                    let GNH;
                    if CGD != 0.0 {
                        let CGE = -1e-6f64 / CGC;
                        let GYK = ((GMX * CGE) * GRP) / CGC;
                        CGH = CGE;
                        GNH = GYK;
                    } else {
                        let GYI = GMX * CGC;
                        let CGF = ((CGC * CGC) + 4e-6f64).sqrt();
                        let CGG = AGZ * (CGC + CGF);
                        let GYJ = (GMX + ((GYI + GYI) * (GHB / (GRJ * CGF)))) * AGZ;
                        CGH = CGG;
                        GNH = GYJ;
                    }
                    let CGI = NO + (BIC / RB);
                    let CGL = BRF * (CGJ - BRG);
                    let CGM = B + (rspice_limited_exp(CGL));
                    let CGN = BRE / CGM;
                    let GYL = (GRG * CGI) + (((((GHN * BRF) * (rspice_limited_exp_derivative(CGL))) * CGN) * GRP) / CGM);
                    let CGO = ((CGI * AWM) + CGN) - (BRE / (B + (rspice_limited_exp((BRF * ((AGZ * ((AWK + AXS) - (((BVO * BVO) + 1.0000000000000002e-2f64).sqrt()))) - BRG))))));
                    CGR = CGT;
                    CGX = CGZ;
                    CHD = CHF;
                    DGU = DGW;
                    DGY = CAL;
                    DHH = DHK;
                    DIB = DID;
                    DII = DIL;
                    DIU = DIW;
                    DJE = DJG;
                    DJI = BSA;
                    DJM = DJO;
                    DJQ = BTY;
                    DJU = DJW;
                    DJY = BVH;
                    DKM = DKO;
                    DNB = CGO;
                    DUN = BVN;
                    DVQ = CGH;
                    EDB = EDD;
                    EGX = EGZ;
                    EHI = EHK;
                    EOT = BWE;
                    EOV = BWG;
                    GJQ = GLY;
                    GJR = GLZ;
                    GJS = GMV;
                    GJT = GMU;
                    GJU = GMA;
                    GJV = GMB;
                    GJW = GMC;
                    GJX = GMD;
                    GJY = GLN;
                    GJZ = GTE;
                    GKA = GLR;
                    GKB = GLQ;
                    GKC = GLV;
                    GKD = GTZ;
                    GKE = GME;
                    GKF = GYL;
                    GKG = GUB;
                    GKH = GNH;
                    GKI = GLW;
                    GKJ = GLX;
                }
                CGP = CGR;
                CGV = CGX;
                CHB = CHD;
                DGS = DGU;
                DGX = DGY;
                DHE = DHH;
                DIA = DIB;
                DIF = DII;
                DIT = DIU;
                DJC = DJE;
                DJH = DJI;
                DJK = DJM;
                DJP = DJQ;
                DJS = DJU;
                DJX = DJY;
                DKL = DKM;
                DNA = DNB;
                DUM = DUN;
                DVP = DVQ;
                ECY = EDB;
                EGU = EGX;
                EHF = EHI;
                EOS = EOT;
                EOU = EOV;
                GID = GJQ;
                GIE = GJR;
                GIF = GJS;
                GIG = GJT;
                GIH = GJU;
                GII = GJV;
                GIJ = GJW;
                GIK = GJX;
                GIL = GJY;
                GIM = GJZ;
                GIN = GKA;
                GIO = GKB;
                GIP = GKC;
                GIQ = GKD;
                GIR = GKE;
                GIS = GKF;
                GIT = GKG;
                GIU = GKH;
                GIV = GKI;
                GIW = GKJ;
            }
            let CGU = if CGP < BEY { 1.0 } else { 0.0 };
            let DIQ;
            let GNI;
            if CGU != 0.0 {
                DIQ = BEY;
                GNI = GRF;
            } else {
                DIQ = CGP;
                GNI = GID;
            }
            let CHA = if CGV < BEY { 1.0 } else { 0.0 };
            let DHP;
            let GNJ;
            if CHA != 0.0 {
                DHP = BEY;
                GNJ = GRF;
            } else {
                DHP = CGV;
                GNJ = GIE;
            }
            let CHG = if CHB < BEY { 1.0 } else { 0.0 };
            let EHV = if CHG != 0.0 {
                BEY
            } else {
                CHB
            };
            let DKA;
            let DKF;
            let EHA;
            let GNK;
            let GNL;
            if AS != 0.0 {
                let CHH = if BDS == A { 1.0 } else { 0.0 };
                let DKB;
                let DKG;
                let EHB;
                let GNM;
                let GNN;
                if CHH != 0.0 {
                    let CHI = if BDS != A { 1.0 } else { 0.0 };
                    let DKH;
                    let GNO;
                    if CHI != 0.0 {
                        let CHK = -CHJ;
                        let HEV = GHH * CHL;
                        let CHM = ((CHL * AWN) - CHK) - S;
                        let HEW = HEV * CHM;
                        let CHN = ((CHM * CHM) - ((Q * CHK) * S)).sqrt();
                        let HEX = (HEV + ((HEW + HEW) * (GHB / (GRJ * CHN)))) * AGZ;
                        let CHO = CHJ + (CHK + (AGZ * (CHM + CHN)));
                        DKH = CHO;
                        GNO = HEX;
                    } else {
                        let HEQ = GHH * CHL;
                        let CHP = (B + (CHL * AWN)) - S;
                        let CHQ = if CHP < -1e1f64 { 1.0 } else { 0.0 };
                        let CHU;
                        let GNP;
                        if CHQ != 0.0 {
                            let CHR = -1e-6f64 / CHP;
                            let HET = ((HEQ * CHR) * GRP) / CHP;
                            CHU = CHR;
                            GNP = HET;
                        } else {
                            let HER = HEQ * CHP;
                            let CHS = ((CHP * CHP) + 4e-6f64).sqrt();
                            let CHT = AGZ * (CHP + CHS);
                            let HES = (HEQ + ((HER + HER) * (GHB / (GRJ * CHS)))) * AGZ;
                            CHU = CHT;
                            GNP = HES;
                        }
                        let CHV = CHJ * CHU;
                        let HEU = GNP * CHJ;
                        DKH = CHV;
                        GNO = HEU;
                    }
                    let EHC;
                    if PT != 0.0 {
                        let EHD;
                        if CHI != 0.0 {
                            let CHZ = -CHW;
                            let CID = ((CIA * AWN) - CHZ) - S;
                            let CIE = CHW + (CHZ + (AGZ * (CID + (((CID * CID) - ((Q * CHZ) * S)).sqrt()))));
                            EHD = CIE;
                        } else {
                            let CIF = (B + (CIA * AWN)) - S;
                            let CIG = if CIF < -1e1f64 { 1.0 } else { 0.0 };
                            let CIJ = if CIG != 0.0 {
                                let CIH = -1e-6f64 / CIF;
                                CIH
                            } else {
                                let CII = AGZ * (CIF + (((CIF * CIF) + 4e-6f64).sqrt()));
                                CII
                            };
                            let CIK = CHW * CIJ;
                            EHD = CIK;
                        }
                        EHC = EHD;
                    } else {
                        EHC = A;
                    }
                    let DKC;
                    let GNQ;
                    if OU != 0.0 {
                        let DKD;
                        let GNR;
                        if CHI != 0.0 {
                            let CIN = -CIL;
                            let HFD = GHH * CIO;
                            let CIQ = ((CIO * AWN) - CIN) - S;
                            let HFE = HFD * CIQ;
                            let CIR = ((CIQ * CIQ) - ((Q * CIN) * S)).sqrt();
                            let HFF = (HFD + ((HFE + HFE) * (GHB / (GRJ * CIR)))) * AGZ;
                            let CIS = CIL + (CIN + (AGZ * (CIQ + CIR)));
                            DKD = CIS;
                            GNR = HFF;
                        } else {
                            let HEY = GHH * CIO;
                            let CIT = (B + (CIO * AWN)) - S;
                            let CIU = if CIT < -1e1f64 { 1.0 } else { 0.0 };
                            let CIY;
                            let GNS;
                            if CIU != 0.0 {
                                let CIV = -1e-6f64 / CIT;
                                let HFB = ((HEY * CIV) * GRP) / CIT;
                                CIY = CIV;
                                GNS = HFB;
                            } else {
                                let HEZ = HEY * CIT;
                                let CIW = ((CIT * CIT) + 4e-6f64).sqrt();
                                let CIX = AGZ * (CIT + CIW);
                                let HFA = (HEY + ((HEZ + HEZ) * (GHB / (GRJ * CIW)))) * AGZ;
                                CIY = CIX;
                                GNS = HFA;
                            }
                            let CIZ = CIL * CIY;
                            let HFC = GNS * CIL;
                            DKD = CIZ;
                            GNR = HFC;
                        }
                        DKC = DKD;
                        GNQ = GNR;
                    } else {
                        DKC = A;
                        GNQ = GRF;
                    }
                    DKB = DKC;
                    DKG = DKH;
                    EHB = EHC;
                    GNM = GNQ;
                    GNN = GNO;
                } else {
                    let HEO = GHH * CHL;
                    let CJA = CHJ + (CHL * AWN);
                    let EHE = if PT != 0.0 {
                        let CJB = CHW + (CIA * AWN);
                        CJB
                    } else {
                        A
                    };
                    let DKE;
                    let GNT;
                    if OU != 0.0 {
                        let HEP = GHH * CIO;
                        let CJC = CIL + (CIO * AWN);
                        DKE = CJC;
                        GNT = HEP;
                    } else {
                        DKE = A;
                        GNT = GRF;
                    }
                    DKB = DKE;
                    DKG = CJA;
                    EHB = EHE;
                    GNM = GNT;
                    GNN = HEO;
                }
                DKA = DKB;
                DKF = DKG;
                EHA = EHB;
                GNK = GNM;
                GNL = GNN;
            } else {
                DKA = A;
                DKF = A;
                EHA = A;
                GNK = GRF;
                GNL = GRF;
            }
            let CJD = if BDS != A { 1.0 } else { 0.0 };
            let DGO;
            let GNU;
            if CJD != 0.0 {
                let CJF = -CJE;
                let HFL = GHH * CJG;
                let CJH = ((CJG * AWN) - CJF) - S;
                let HFM = HFL * CJH;
                let CJI = ((CJH * CJH) - ((Q * CJF) * S)).sqrt();
                let HFN = (HFL + ((HFM + HFM) * (GHB / (GRJ * CJI)))) * AGZ;
                let CJJ = CJE + (CJF + (AGZ * (CJH + CJI)));
                DGO = CJJ;
                GNU = HFN;
            } else {
                let HFG = GHH * CJG;
                let CJK = (B + (CJG * AWN)) - S;
                let CJL = if CJK < -1e1f64 { 1.0 } else { 0.0 };
                let CJP;
                let GNV;
                if CJL != 0.0 {
                    let CJM = -1e-6f64 / CJK;
                    let HFJ = ((HFG * CJM) * GRP) / CJK;
                    CJP = CJM;
                    GNV = HFJ;
                } else {
                    let HFH = HFG * CJK;
                    let CJN = ((CJK * CJK) + 4e-6f64).sqrt();
                    let CJO = AGZ * (CJK + CJN);
                    let HFI = (HFG + ((HFH + HFH) * (GHB / (GRJ * CJN)))) * AGZ;
                    CJP = CJO;
                    GNV = HFI;
                }
                let CJQ = CJE * CJP;
                let HFK = GNV * CJE;
                DGO = CJQ;
                GNU = HFK;
            }
            let ECU;
            if PT != 0.0 {
                let ECV;
                if CJD != 0.0 {
                    let CJU = -CJR;
                    let CJW = ((CJV * AWN) - CJU) - S;
                    let CJX = CJR + (CJU + (AGZ * (CJW + (((CJW * CJW) - ((Q * CJU) * S)).sqrt()))));
                    ECV = CJX;
                } else {
                    let CJY = (B + (CJV * AWN)) - S;
                    let CJZ = if CJY < -1e1f64 { 1.0 } else { 0.0 };
                    let CKC = if CJZ != 0.0 {
                        let CKA = -1e-6f64 / CJY;
                        CKA
                    } else {
                        let CKB = AGZ * (CJY + (((CJY * CJY) + 4e-6f64).sqrt()));
                        CKB
                    };
                    let CKD = CJR * CKC;
                    ECV = CKD;
                }
                ECU = ECV;
            } else {
                ECU = ECW;
            }
            let DGN;
            let GNW;
            if CJD != 0.0 {
                let CKF = -CKE;
                let HFT = GHH * CKG;
                let CKH = ((CKG * AWN) - CKF) - S;
                let HFU = HFT * CKH;
                let CKI = ((CKH * CKH) - ((Q * CKF) * S)).sqrt();
                let HFV = (HFT + ((HFU + HFU) * (GHB / (GRJ * CKI)))) * AGZ;
                let CKJ = CKE + (CKF + (AGZ * (CKH + CKI)));
                DGN = CKJ;
                GNW = HFV;
            } else {
                let HFO = GHH * CKG;
                let CKK = (B + (CKG * AWN)) - S;
                let CKL = if CKK < -1e1f64 { 1.0 } else { 0.0 };
                let CKP;
                let GNX;
                if CKL != 0.0 {
                    let CKM = -1e-6f64 / CKK;
                    let HFR = ((HFO * CKM) * GRP) / CKK;
                    CKP = CKM;
                    GNX = HFR;
                } else {
                    let HFP = HFO * CKK;
                    let CKN = ((CKK * CKK) + 4e-6f64).sqrt();
                    let CKO = AGZ * (CKK + CKN);
                    let HFQ = (HFO + ((HFP + HFP) * (GHB / (GRJ * CKN)))) * AGZ;
                    CKP = CKO;
                    GNX = HFQ;
                }
                let CKQ = CKE * CKP;
                let HFS = GNX * CKE;
                DGN = CKQ;
                GNW = HFS;
            }
            let CZX;
            let GNY;
            if CJD != 0.0 {
                let CKS = -CKR;
                let HGB = GHH * MY;
                let CKT = ((MY * AWN) - CKS) - S;
                let HGC = HGB * CKT;
                let CKU = ((CKT * CKT) - ((Q * CKS) * S)).sqrt();
                let HGD = (HGB + ((HGC + HGC) * (GHB / (GRJ * CKU)))) * AGZ;
                let CKV = CKR + (CKS + (AGZ * (CKT + CKU)));
                CZX = CKV;
                GNY = HGD;
            } else {
                let HFW = GHH * MY;
                let CKW = (B + (MY * AWN)) - S;
                let CKX = if CKW < -1e1f64 { 1.0 } else { 0.0 };
                let CLB;
                let GNZ;
                if CKX != 0.0 {
                    let CKY = -1e-6f64 / CKW;
                    let HFZ = ((HFW * CKY) * GRP) / CKW;
                    CLB = CKY;
                    GNZ = HFZ;
                } else {
                    let HFX = HFW * CKW;
                    let CKZ = ((CKW * CKW) + 4e-6f64).sqrt();
                    let CLA = AGZ * (CKW + CKZ);
                    let HFY = (HFW + ((HFX + HFX) * (GHB / (GRJ * CKZ)))) * AGZ;
                    CLB = CLA;
                    GNZ = HFY;
                }
                let CLC = CKR * CLB;
                let HGA = GNZ * CKR;
                CZX = CLC;
                GNY = HGA;
            }
            let DHT;
            if CJD != 0.0 {
                let CLE = -CLD;
                let CLG = ((CLF * AWN) - CLE) - S;
                let CLH = CLD + (CLE + (AGZ * (CLG + (((CLG * CLG) - ((Q * CLE) * S)).sqrt()))));
                DHT = CLH;
            } else {
                let CLI = (B + (CLF * AWN)) - S;
                let CLJ = if CLI < -1e1f64 { 1.0 } else { 0.0 };
                let CLM = if CLJ != 0.0 {
                    let CLK = -1e-6f64 / CLI;
                    CLK
                } else {
                    let CLL = AGZ * (CLI + (((CLI * CLI) + 4e-6f64).sqrt()));
                    CLL
                };
                let CLN = CLD * CLM;
                DHT = CLN;
            }
            let DHR;
            if OU != 0.0 {
                let DHS;
                if CJD != 0.0 {
                    let CLP = -CLO;
                    let CLQ = ((CLF * AWN) - CLP) - S;
                    let CLR = CLO + (CLP + (AGZ * (CLQ + (((CLQ * CLQ) - ((Q * CLP) * S)).sqrt()))));
                    DHS = CLR;
                } else {
                    let CLS = (B + (CLF * AWN)) - S;
                    let CLT = if CLS < -1e1f64 { 1.0 } else { 0.0 };
                    let CLW = if CLT != 0.0 {
                        let CLU = -1e-6f64 / CLS;
                        CLU
                    } else {
                        let CLV = AGZ * (CLS + (((CLS * CLS) + 4e-6f64).sqrt()));
                        CLV
                    };
                    let CLX = CLO * CLW;
                    DHS = CLX;
                }
                DHR = DHS;
            } else {
                DHR = A;
            }
            let DHX;
            if CJD != 0.0 {
                let CLZ = -CLY;
                let CMB = ((CMA * AWN) - CLZ) - S;
                let CMC = CLY + (CLZ + (AGZ * (CMB + (((CMB * CMB) - ((Q * CLZ) * S)).sqrt()))));
                DHX = CMC;
            } else {
                let CMD = (B + (CMA * AWN)) - S;
                let CME = if CMD < -1e1f64 { 1.0 } else { 0.0 };
                let CMH = if CME != 0.0 {
                    let CMF = -1e-6f64 / CMD;
                    CMF
                } else {
                    let CMG = AGZ * (CMD + (((CMD * CMD) + 4e-6f64).sqrt()));
                    CMG
                };
                let CMI = CLY * CMH;
                DHX = CMI;
            }
            let DHV;
            if OU != 0.0 {
                let DHW;
                if CJD != 0.0 {
                    let CMK = -CMJ;
                    let CML = ((CMA * AWN) - CMK) - S;
                    let CMM = CMJ + (CMK + (AGZ * (CML + (((CML * CML) - ((Q * CMK) * S)).sqrt()))));
                    DHW = CMM;
                } else {
                    let CMN = (B + (CMA * AWN)) - S;
                    let CMO = if CMN < -1e1f64 { 1.0 } else { 0.0 };
                    let CMR = if CMO != 0.0 {
                        let CMP = -1e-6f64 / CMN;
                        CMP
                    } else {
                        let CMQ = AGZ * (CMN + (((CMN * CMN) + 4e-6f64).sqrt()));
                        CMQ
                    };
                    let CMS = CMJ * CMR;
                    DHW = CMS;
                }
                DHV = DHW;
            } else {
                DHV = A;
            }
            let DHC;
            let GOA;
            if CJD != 0.0 {
                let CMU = -CMT;
                let CMV = -NH;
                let HGJ = GHH * CMV;
                let CMW = ((CMV * AWN) - CMU) - S;
                let HGK = HGJ * CMW;
                let CMX = ((CMW * CMW) - ((Q * CMU) * S)).sqrt();
                let HGL = (HGJ + ((HGK + HGK) * (GHB / (GRJ * CMX)))) * AGZ;
                let CMY = CMT + (CMU + (AGZ * (CMW + CMX)));
                DHC = CMY;
                GOA = HGL;
            } else {
                let CMZ = -NH;
                let HGE = GHH * CMZ;
                let CNA = (B + (CMZ * AWN)) - S;
                let CNB = if CNA < -1e1f64 { 1.0 } else { 0.0 };
                let CNF;
                let GOB;
                if CNB != 0.0 {
                    let CNC = -1e-6f64 / CNA;
                    let HGH = ((HGE * CNC) * GRP) / CNA;
                    CNF = CNC;
                    GOB = HGH;
                } else {
                    let HGF = HGE * CNA;
                    let CND = ((CNA * CNA) + 4e-6f64).sqrt();
                    let CNE = AGZ * (CNA + CND);
                    let HGG = (HGE + ((HGF + HGF) * (GHB / (GRJ * CND)))) * AGZ;
                    CNF = CNE;
                    GOB = HGG;
                }
                let CNG = CMT * CNF;
                let HGI = GOB * CMT;
                DHC = CNG;
                GOA = HGI;
            }
            let DHA;
            let GOC;
            if OU != 0.0 {
                let DHB;
                let GOD;
                if CJD != 0.0 {
                    let CNI = -CNH;
                    let CNJ = -NH;
                    let HGR = GHH * CNJ;
                    let CNK = ((CNJ * AWN) - CNI) - S;
                    let HGS = HGR * CNK;
                    let CNL = ((CNK * CNK) - ((Q * CNI) * S)).sqrt();
                    let HGT = (HGR + ((HGS + HGS) * (GHB / (GRJ * CNL)))) * AGZ;
                    let CNM = CNH + (CNI + (AGZ * (CNK + CNL)));
                    DHB = CNM;
                    GOD = HGT;
                } else {
                    let CNN = -NH;
                    let HGM = GHH * CNN;
                    let CNO = (B + (CNN * AWN)) - S;
                    let CNP = if CNO < -1e1f64 { 1.0 } else { 0.0 };
                    let CNT;
                    let GOE;
                    if CNP != 0.0 {
                        let CNQ = -1e-6f64 / CNO;
                        let HGP = ((HGM * CNQ) * GRP) / CNO;
                        CNT = CNQ;
                        GOE = HGP;
                    } else {
                        let HGN = HGM * CNO;
                        let CNR = ((CNO * CNO) + 4e-6f64).sqrt();
                        let CNS = AGZ * (CNO + CNR);
                        let HGO = (HGM + ((HGN + HGN) * (GHB / (GRJ * CNR)))) * AGZ;
                        CNT = CNS;
                        GOE = HGO;
                    }
                    let CNU = CNH * CNT;
                    let HGQ = GOE * CNH;
                    DHB = CNU;
                    GOD = HGQ;
                }
                DHA = DHB;
                GOC = GOD;
            } else {
                DHA = A;
                GOC = GRF;
            }
            let CNV = MA * ((NQ * BBI).exp());
            let CNW = if ((B + (MJ * AWM)) - YC) < -1e1f64 { 1.0 } else { 0.0 };
            if CNW != 0.0 {
            } else {
            }
            let HGU = GHH * IG;
            let CNX = IF + (IG * AWN);
            let CNZ = -CNY;
            let HGV = GHH * II;
            let COA = ((II * AWN) - CNZ) - S;
            let HGW = HGV * COA;
            let COB = ((COA * COA) - ((Q * CNZ) * S)).sqrt();
            let HGX = (HGV + ((HGW + HGW) * (GHB / (GRJ * COB)))) * AGZ;
            let COC = CNY + (CNZ + (AGZ * (COA + COB)));
            let COE = -COD;
            let HGY = GHH * IK;
            let COF = ((IK * AWN) - COE) - S;
            let HGZ = HGY * COF;
            let COG = ((COF * COF) - ((Q * COE) * S)).sqrt();
            let HHA = (HGY + ((HGZ + HGZ) * (GHB / (GRJ * COG)))) * AGZ;
            let COH = COD + (COE + (AGZ * (COF + COG)));
            let COK = -COI;
            let HHB = GHH * COL;
            let COM = ((COL * AWN) - COK) - S;
            let HHC = HHB * COM;
            let CON = ((COM * COM) - ((Q * COK) * S)).sqrt();
            let HHD = (HHB + ((HHC + HHC) * (GHB / (GRJ * CON)))) * AGZ;
            let COO = COI + (COK + (AGZ * (COM + CON)));
            let HHE = GHH * COR;
            let COT = COP + (COR * AWN);
            let HHF = GHH * JT;
            let COU = JS + (JT * AWN);
            let HHG = GHH * JV;
            let COV = JU + (JV * AWN);
            let COY = -COW;
            let HHH = GHH * COZ;
            let CPB = ((COZ * AWN) - COY) - S;
            let HHI = HHH * CPB;
            let CPC = ((CPB * CPB) - ((Q * COY) * S)).sqrt();
            let HHJ = (HHH + ((HHI + HHI) * (GHB / (GRJ * CPC)))) * AGZ;
            let CPD = COW + (COY + (AGZ * (CPB + CPC)));
            let HHK = GHH * IM;
            let CPE = IL + (IM * AWN);
            let HHL = GHH * IO;
            let CPF = IN + (IO * AWN);
            let CPG = -KG;
            let CPH = ((KH * AWN) - CPG) - S;
            let CPI = KG + (CPG + (AGZ * (CPH + (((CPH * CPH) - ((Q * CPG) * S)).sqrt()))));
            let CPJ = -KM;
            let CPK = ((KN * AWN) - CPJ) - S;
            let CPL = KM + (CPJ + (AGZ * (CPK + (((CPK * CPK) - ((Q * CPJ) * S)).sqrt()))));
            let CPM = -KR;
            let CPN = ((KS * AWN) - CPM) - S;
            let CPO = KR + (CPM + (AGZ * (CPN + (((CPN * CPN) - ((Q * CPM) * S)).sqrt()))));
            let CPP = -KW;
            let CPQ = ((KX * AWN) - CPP) - S;
            let CPR = KW + (CPP + (AGZ * (CPQ + (((CPQ * CPQ) - ((Q * CPP) * S)).sqrt()))));
            let CPS = -LA;
            let CPT = ((LB * AWN) - CPS) - S;
            let CPU = LA + (CPS + (AGZ * (CPT + (((CPT * CPT) - ((Q * CPS) * S)).sqrt()))));
            let CPV = (B + (NR * AWN)) - S;
            let CPW = if CPV < -1e1f64 { 1.0 } else { 0.0 };
            let CPZ = if CPW != 0.0 {
                let CPX = -1e-6f64 / CPV;
                CPX
            } else {
                let CPY = AGZ * (CPV + (((CPV * CPV) + 4e-6f64).sqrt()));
                CPY
            };
            let CQA = LH * CPZ;
            let CQB = if CPV < -1e1f64 { 1.0 } else { 0.0 };
            let CQE = if CQB != 0.0 {
                let CQC = -1e-6f64 / CPV;
                CQC
            } else {
                let CQD = AGZ * (CPV + (((CPV * CPV) + 4e-6f64).sqrt()));
                CQD
            };
            let CQF = LP * CQE;
            let CQG = -LW;
            let CQH = ((parameters[1437] * AWN) - CQG) - S;
            let CQI = LW + (CQG + (AGZ * (CQH + (((CQH * CQH) - ((Q * CQG) * S)).sqrt()))));
            let CQJ = -LX;
            let CQK = ((parameters[1438] * AWN) - CQJ) - S;
            let CQL = LX + (CQJ + (AGZ * (CQK + (((CQK * CQK) - ((Q * CQJ) * S)).sqrt()))));
            let CQM = -LY;
            let CQO = ((parameters[1439] * AWN) - CQM) - CQN;
            let CQP = LY + (CQM + (AGZ * (CQO + (((CQO * CQO) - ((Q * CQM) * CQN)).sqrt()))));
            let CQQ = -LZ;
            let CQS = ((parameters[1440] * AWN) - CQQ) - CQR;
            let CQT = LZ + (CQQ + (AGZ * (CQS + (((CQS * CQS) - ((Q * CQQ) * CQR)).sqrt()))));
            let CQU = (NT * BBI).exp();
            let CQV = APS * CQU;
            let DAD;
            let DAF;
            let DAH;
            let DBM;
            let DBO;
            let DBP;
            let DCS;
            let DCV;
            let DCY;
            let DDB;
            let DDE;
            let DDH;
            let DDM;
            let DDT;
            let DEA;
            let DEH;
            let DEO;
            let DEV;
            let FJX;
            let FKB;
            let FKF;
            let FKZ;
            let FLD;
            let FLH;
            if AS != 0.0 {
                let DCT;
                if CJD != 0.0 {
                    let CQX = -CQW;
                    let CQZ = ((CQY * AWN) - CQX) - S;
                    let CRA = CQW + (CQX + (AGZ * (CQZ + (((CQZ * CQZ) - ((Q * CQX) * S)).sqrt()))));
                    DCT = CRA;
                } else {
                    let CRB = (B + (CQY * AWN)) - S;
                    let CRC = if CRB < -1e1f64 { 1.0 } else { 0.0 };
                    let CRF = if CRC != 0.0 {
                        let CRD = -1e-6f64 / CRB;
                        CRD
                    } else {
                        let CRE = AGZ * (CRB + (((CRB * CRB) + 4e-6f64).sqrt()));
                        CRE
                    };
                    let CRG = CQW * CRF;
                    DCT = CRG;
                }
                let DDC;
                if CJD != 0.0 {
                    let CRI = -CRH;
                    let CRJ = ((CQY * AWN) - CRI) - S;
                    let CRK = CRH + (CRI + (AGZ * (CRJ + (((CRJ * CRJ) - ((Q * CRI) * S)).sqrt()))));
                    DDC = CRK;
                } else {
                    let CRL = (B + (CQY * AWN)) - S;
                    let CRM = if CRL < -1e1f64 { 1.0 } else { 0.0 };
                    let CRP = if CRM != 0.0 {
                        let CRN = -1e-6f64 / CRL;
                        CRN
                    } else {
                        let CRO = AGZ * (CRL + (((CRL * CRL) + 4e-6f64).sqrt()));
                        CRO
                    };
                    let CRQ = CRH * CRP;
                    DDC = CRQ;
                }
                let DCW;
                if CJD != 0.0 {
                    let CRS = -CRR;
                    let CRU = ((CRT * AWN) - CRS) - S;
                    let CRV = CRR + (CRS + (AGZ * (CRU + (((CRU * CRU) - ((Q * CRS) * S)).sqrt()))));
                    DCW = CRV;
                } else {
                    let CRW = (B + (CRT * AWN)) - S;
                    let CRX = if CRW < -1e1f64 { 1.0 } else { 0.0 };
                    let CSA = if CRX != 0.0 {
                        let CRY = -1e-6f64 / CRW;
                        CRY
                    } else {
                        let CRZ = AGZ * (CRW + (((CRW * CRW) + 4e-6f64).sqrt()));
                        CRZ
                    };
                    let CSB = CRR * CSA;
                    DCW = CSB;
                }
                let DDF;
                if CJD != 0.0 {
                    let CSD = -CSC;
                    let CSE = ((CRT * AWN) - CSD) - S;
                    let CSF = CSC + (CSD + (AGZ * (CSE + (((CSE * CSE) - ((Q * CSD) * S)).sqrt()))));
                    DDF = CSF;
                } else {
                    let CSG = (B + (CRT * AWN)) - S;
                    let CSH = if CSG < -1e1f64 { 1.0 } else { 0.0 };
                    let CSK = if CSH != 0.0 {
                        let CSI = -1e-6f64 / CSG;
                        CSI
                    } else {
                        let CSJ = AGZ * (CSG + (((CSG * CSG) + 4e-6f64).sqrt()));
                        CSJ
                    };
                    let CSL = CSC * CSK;
                    DDF = CSL;
                }
                let DCZ;
                if CJD != 0.0 {
                    let CSN = -CSM;
                    let CSP = ((CSO * AWN) - CSN) - S;
                    let CSQ = CSM + (CSN + (AGZ * (CSP + (((CSP * CSP) - ((Q * CSN) * S)).sqrt()))));
                    DCZ = CSQ;
                } else {
                    let CSR = (B + (CSO * AWN)) - S;
                    let CSS = if CSR < -1e1f64 { 1.0 } else { 0.0 };
                    let CSV = if CSS != 0.0 {
                        let CST = -1e-6f64 / CSR;
                        CST
                    } else {
                        let CSU = AGZ * (CSR + (((CSR * CSR) + 4e-6f64).sqrt()));
                        CSU
                    };
                    let CSW = CSM * CSV;
                    DCZ = CSW;
                }
                let DDI;
                if CJD != 0.0 {
                    let CSY = -CSX;
                    let CSZ = ((CSO * AWN) - CSY) - S;
                    let CTA = CSX + (CSY + (AGZ * (CSZ + (((CSZ * CSZ) - ((Q * CSY) * S)).sqrt()))));
                    DDI = CTA;
                } else {
                    let CTB = (B + (CSO * AWN)) - S;
                    let CTC = if CTB < -1e1f64 { 1.0 } else { 0.0 };
                    let CTF = if CTC != 0.0 {
                        let CTD = -1e-6f64 / CTB;
                        CTD
                    } else {
                        let CTE = AGZ * (CTB + (((CTB * CTB) + 4e-6f64).sqrt()));
                        CTE
                    };
                    let CTG = CSX * CTF;
                    DDI = CTG;
                }
                let CTH = parameters[1724] * AWN;
                let CTI = (parameters[1590] - CTH) - YC;
                let CTJ = if CTI < -1e1f64 { 1.0 } else { 0.0 };
                let CTM = if CTJ != 0.0 {
                    let CTK = -1e-6f64 / CTI;
                    CTK
                } else {
                    let CTL = AGZ * (CTI + (((CTI * CTI) + 4e-6f64).sqrt()));
                    CTL
                };
                let CTN = CTM + YC;
                let CTO = (parameters[1591] - CTH) - YC;
                let CTP = if CTO < -1e1f64 { 1.0 } else { 0.0 };
                let CTS = if CTP != 0.0 {
                    let CTQ = -1e-6f64 / CTO;
                    CTQ
                } else {
                    let CTR = AGZ * (CTO + (((CTO * CTO) + 4e-6f64).sqrt()));
                    CTR
                };
                let CTT = CTS + YC;
                let CTU = parameters[1725] * AWN;
                let CTV = (parameters[1592] - CTU) - YC;
                let CTW = if CTV < -1e1f64 { 1.0 } else { 0.0 };
                let CTZ = if CTW != 0.0 {
                    let CTX = -1e-6f64 / CTV;
                    CTX
                } else {
                    let CTY = AGZ * (CTV + (((CTV * CTV) + 4e-6f64).sqrt()));
                    CTY
                };
                let CUA = CTZ + YC;
                let CUB = (parameters[1593] - CTU) - YC;
                let CUC = if CUB < -1e1f64 { 1.0 } else { 0.0 };
                let CUF = if CUC != 0.0 {
                    let CUD = -1e-6f64 / CUB;
                    CUD
                } else {
                    let CUE = AGZ * (CUB + (((CUB * CUB) + 4e-6f64).sqrt()));
                    CUE
                };
                let CUG = CUF + YC;
                let CUH = parameters[1726] * AWN;
                let CUI = (parameters[1594] - CUH) - YC;
                let CUJ = if CUI < -1e1f64 { 1.0 } else { 0.0 };
                let CUM = if CUJ != 0.0 {
                    let CUK = -1e-6f64 / CUI;
                    CUK
                } else {
                    let CUL = AGZ * (CUI + (((CUI * CUI) + 4e-6f64).sqrt()));
                    CUL
                };
                let CUN = CUM + YC;
                let CUO = (parameters[1595] - CUH) - YC;
                let CUP = if CUO < -1e1f64 { 1.0 } else { 0.0 };
                let CUS = if CUP != 0.0 {
                    let CUQ = -1e-6f64 / CUO;
                    CUQ
                } else {
                    let CUR = AGZ * (CUO + (((CUO * CUO) + 4e-6f64).sqrt()));
                    CUR
                };
                let CUT = CUS + YC;
                let CUU = (AZH / AWQ) - (AZG / AWP);
                let CUW = rspice_limited_exp(((CUU + (parameters[1727] * BBI)) / CUV));
                let CUX = parameters[1614] * CUW;
                let CUY = parameters[1616] * CUW;
                let CUZ = parameters[1618] * CUW;
                let CVB = rspice_limited_exp(((CUU + (parameters[1728] * BBI)) / CVA));
                let CVC = parameters[1615] * CVB;
                let CVD = parameters[1617] * CVB;
                let CVE = parameters[1619] * CVB;
                let CVF = parameters[1630] * (rspice_limited_exp((((AZH * parameters[1729]) * AWM) / AWP)));
                let CVG = parameters[1631] * (rspice_limited_exp((((AZH * parameters[1730]) * AWM) / AWP)));
                let CVH = parameters[1632] * (rspice_limited_exp((((AZH * parameters[1731]) * AWM) / AWP)));
                let CVI = parameters[1633] * (rspice_limited_exp((((AZH * parameters[1732]) * AWM) / AWP)));
                let CVJ = ((parameters[1636] / HH).sqrt()) + B;
                let CVK = (parameters[1634] * CVJ) * (rspice_limited_exp((((AZH * parameters[1733]) * AWM) / AWP)));
                let CVL = (parameters[1635] * CVJ) * (rspice_limited_exp((((AZH * parameters[1734]) * AWM) / AWP)));
                let CVM = if ((parameters[1637] * (B + (parameters[1735] * AWM))) - YC) < -1e1f64 { 1.0 } else { 0.0 };
                if CVM != 0.0 {
                } else {
                }
                let CVN = if ((parameters[1638] * (B + (parameters[1736] * AWM))) - YC) < -1e1f64 { 1.0 } else { 0.0 };
                if CVN != 0.0 {
                } else {
                }
                let CVO = if ((parameters[1639] * (B + (parameters[1737] * AWM))) - YC) < -1e1f64 { 1.0 } else { 0.0 };
                if CVO != 0.0 {
                } else {
                }
                let CVP = if ((parameters[1640] * (B + (parameters[1738] * AWM))) - YC) < -1e1f64 { 1.0 } else { 0.0 };
                if CVP != 0.0 {
                } else {
                }
                let CVQ = if ((parameters[1641] * (B + (parameters[1739] * AWM))) - YC) < -1e1f64 { 1.0 } else { 0.0 };
                if CVQ != 0.0 {
                } else {
                }
                let CVR = if ((parameters[1642] * (B + (parameters[1740] * AWM))) - YC) < -1e1f64 { 1.0 } else { 0.0 };
                if CVR != 0.0 {
                } else {
                }
                DAD = CUX;
                DAF = CUY;
                DAH = CUZ;
                DBM = CVC;
                DBO = CVD;
                DBP = CVE;
                DCS = DCT;
                DCV = DCW;
                DCY = DCZ;
                DDB = DDC;
                DDE = DDF;
                DDH = DDI;
                DDM = CTN;
                DDT = CUA;
                DEA = CUN;
                DEH = CTT;
                DEO = CUG;
                DEV = CUT;
                FJX = CVF;
                FKB = CVH;
                FKF = CVK;
                FKZ = CVG;
                FLD = CVI;
                FLH = CVL;
            } else {
                DAD = A;
                DAF = A;
                DAH = A;
                DBM = A;
                DBO = A;
                DBP = A;
                DCS = A;
                DCV = A;
                DCY = A;
                DDB = A;
                DDE = A;
                DDH = A;
                DDM = A;
                DDT = A;
                DEA = A;
                DEH = A;
                DEO = A;
                DEV = A;
                FJX = A;
                FKB = A;
                FKF = A;
                FKZ = A;
                FLD = A;
                FLH = A;
            }
            let CVS = if (if parameter_given[1106] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            let CYN;
            if CVS != 0.0 {
                let CVT = if HQ > A { 1.0 } else { 0.0 };
                let CYO;
                if CVT != 0.0 {
                    let CYP;
                    if AZS != 0.0 {
                        let CVU = HQ / AZR;
                        let CVV = if CVU > BZ { 1.0 } else { 0.0 };
                        let CVZ = if CVV != 0.0 {
                            let CVW = CVU.ln();
                            CVW
                        } else {
                            CVX
                        };
                        let CVY = AGZ * AZG;
                        let CWA = CVY - (AWP * CVZ);
                        let CWB = if CWA < -1e0f64 { 1.0 } else { 0.0 };
                        let CWO = if CWB != 0.0 {
                            let CWC = -1e-8f64 / CWA;
                            CWC
                        } else {
                            let CWD = AGZ * (CWA + (((CWA * CWA) + 4e-8f64).sqrt()));
                            CWD
                        };
                        let CWE = AEU / AZR;
                        let CWF = if CWE > BZ { 1.0 } else { 0.0 };
                        let CWI = if CWF != 0.0 {
                            let CWG = CWE.ln();
                            CWG
                        } else {
                            CWH
                        };
                        let CWJ = CVY - (AWP * CWI);
                        let CWK = if CWJ < -1e0f64 { 1.0 } else { 0.0 };
                        let CWP = if CWK != 0.0 {
                            let CWL = -1e-8f64 / CWJ;
                            CWL
                        } else {
                            let CWM = AGZ * (CWJ + (((CWJ * CWJ) + 4e-8f64).sqrt()));
                            CWM
                        };
                        let CWQ = CWN * (CWO - (CVY - (CWN * (CVY - CWP))));
                        CYP = CWQ;
                    } else {
                        let CWR = if HQ > BZ { 1.0 } else { 0.0 };
                        let CWV = if CWR != 0.0 {
                            let CWS = HQ.ln();
                            CWS
                        } else {
                            CWT
                        };
                        let CWU = AGZ * AZG;
                        let CWX = CWU - (AWP * (CWV - CWW));
                        let CWY = if CWX < -1e0f64 { 1.0 } else { 0.0 };
                        let CXJ = if CWY != 0.0 {
                            let CWZ = -1e-8f64 / CWX;
                            CWZ
                        } else {
                            let CXA = AGZ * (CWX + (((CWX * CWX) + 4e-8f64).sqrt()));
                            CXA
                        };
                        let CXB = if AEU > BZ { 1.0 } else { 0.0 };
                        let CXE = if CXB != 0.0 {
                            let CXC = AEU.ln();
                            CXC
                        } else {
                            CXD
                        };
                        let CXF = CWU - (AWP * (CXE - CWW));
                        let CXG = if CXF < -1e0f64 { 1.0 } else { 0.0 };
                        let CXK = if CXG != 0.0 {
                            let CXH = -1e-8f64 / CXF;
                            CXH
                        } else {
                            let CXI = AGZ * (CXF + (((CXF * CXF) + 4e-8f64).sqrt()));
                            CXI
                        };
                        let CXL = CWN * (CXJ - (CWU - (CWN * (CWU - CXK))));
                        CYP = CXL;
                    }
                    CYO = CYP;
                } else {
                    let CYQ;
                    if AZS != 0.0 {
                        let CXM = AEU / AZR;
                        let CXN = if CXM > BZ { 1.0 } else { 0.0 };
                        let CXR = if CXN != 0.0 {
                            let CXO = CXM.ln();
                            CXO
                        } else {
                            CXP
                        };
                        let CXQ = AGZ * AZG;
                        let CXS = CXQ - (AWP * CXR);
                        let CXT = if CXS < -1e0f64 { 1.0 } else { 0.0 };
                        let CXY = if CXT != 0.0 {
                            let CXU = -1e-8f64 / CXS;
                            CXU
                        } else {
                            let CXV = AGZ * (CXS + (((CXS * CXS) + 4e-8f64).sqrt()));
                            CXV
                        };
                        let CXZ = CWN * (CXW - ((CXX + CXQ) - (CWN * (CXQ - CXY))));
                        CYQ = CXZ;
                    } else {
                        let CYA = if AEU > BZ { 1.0 } else { 0.0 };
                        let CYE = if CYA != 0.0 {
                            let CYB = AEU.ln();
                            CYB
                        } else {
                            CYC
                        };
                        let CYD = AGZ * AZG;
                        let CYF = CYD - (AWP * (CYE - CWW));
                        let CYG = if CYF < -1e0f64 { 1.0 } else { 0.0 };
                        let CYJ = if CYG != 0.0 {
                            let CYH = -1e-8f64 / CYF;
                            CYH
                        } else {
                            let CYI = AGZ * (CYF + (((CYF * CYF) + 4e-8f64).sqrt()));
                            CYI
                        };
                        let CYK = CWN * (CXW - ((CXX + CYD) - (CWN * (CYD - CYJ))));
                        CYQ = CYK;
                    }
                    CYO = CYQ;
                }
                CYN = CYO;
            } else {
                CYN = CYL;
            }
            let CYM = if (if parameter_given[1107] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
            if CYM != 0.0 {
            } else {
            }
            let DKS;
            let DML;
            let GOF;
            let GOG;
            if AZS != 0.0 {
                let CYR = HE / AZR;
                let HHQ = ((GSM * CYR) * GRP) / AZR;
                let CYS = if CYR > BZ { 1.0 } else { 0.0 };
                let CYV;
                let GOH;
                if CYS != 0.0 {
                    let CYT = CYR.ln();
                    let HHR = HHQ * (GHB / CYR);
                    CYV = CYT;
                    GOH = HHR;
                } else {
                    CYV = CYU;
                    GOH = GRF;
                }
                let CYW = AWP * CYV;
                let HHS = (GRH * CYV) + (GOH * AWP);
                let HHT = HHS * CYW;
                let CYX = ((CYW * CYW) + 2.5000000000000002e-21f64).sqrt();
                let CYY = AGZ * (CYW + CYX);
                let HHU = (HHS + ((HHT + HHT) * (GHB / (GRJ * CYX)))) * AGZ;
                let CYZ = AZR * AZR;
                let HHV = GSM * AZR;
                let CZA = (HE * AEU) / CYZ;
                let HHW = (((HHV + HHV) * CZA) * GRP) / CYZ;
                let CZB = if CZA > BZ { 1.0 } else { 0.0 };
                let CZE;
                let GOI;
                if CZB != 0.0 {
                    let CZC = CZA.ln();
                    let HHX = HHW * (GHB / CZA);
                    CZE = CZC;
                    GOI = HHX;
                } else {
                    CZE = CZD;
                    GOI = GRF;
                }
                let CZF = AWP * CZE;
                let HHY = (GRH * CZE) + (GOI * AWP);
                DKS = CYY;
                DML = CZF;
                GOF = HHU;
                GOG = HHY;
            } else {
                let CZG = if HE > BZ { 1.0 } else { 0.0 };
                let CZJ = if CZG != 0.0 {
                    let CZH = HE.ln();
                    CZH
                } else {
                    CZI
                };
                let CZK = CZJ - CWW;
                let CZL = AWP * CZK;
                let HHM = (GRH * CZK) + ((GHZ * GRP) * AWP);
                let HHN = HHM * CZL;
                let CZM = ((CZL * CZL) + 2.5000000000000002e-21f64).sqrt();
                let CZN = AGZ * (CZL + CZM);
                let HHO = (HHM + ((HHN + HHN) * (GHB / (GRJ * CZM)))) * AGZ;
                let CZO = HE * AEU;
                let CZP = if CZO > BZ { 1.0 } else { 0.0 };
                let CZS = if CZP != 0.0 {
                    let CZQ = CZO.ln();
                    CZQ
                } else {
                    CZR
                };
                let CZT = CZS - (R * CWW);
                let CZU = AWP * CZT;
                let HHP = (GRH * CZT) + (((GHZ * R) * GRP) * AWP);
                DKS = CZN;
                DML = CZU;
                GOF = HHO;
                GOG = HHP;
            }
            let CZV;
            let GOJ;
            if H != 0.0 {
                CZV = A;
                GOJ = GRF;
            } else {
                CZV = AZG;
                GOJ = GSG;
            }
            let CZW = CWN * (CXW - (CXX + CZV));
            let HHZ = (GOJ * GRP) * CWN;
            let CZY = AGZ * CZX;
            let HIA = GNY * AGZ;
            let CZZ = if G != B { 1.0 } else { 0.0 };
            let DUG;
            let EQK;
            let GOK;
            if CZZ != 0.0 {
                let DAB = DAA * CZX;
                let HIB = GNY * DAA;
                DUG = DAB;
                EQK = DAA;
                GOK = HIB;
            } else {
                DUG = CZY;
                EQK = AGZ;
                GOK = HIA;
            }
            let FJH;
            let FJJ;
            let FJM;
            let FJO;
            let FJQ;
            let FJT;
            let FKJ;
            let FKL;
            let FKO;
            let FKQ;
            let FKS;
            let FKV;
            let FLL;
            let FLQ;
            let FMB;
            let FMN;
            let FMS;
            let FND;
            let FNP;
            let FNU;
            let FOF;
            let FOR;
            let FOW;
            let FPH;
            let FPT;
            let FPY;
            let FQJ;
            let FQV;
            let FRA;
            let FRL;
            if AS != 0.0 {
                let DAG = CS * BK;
                let DAI = ((DAC * DAD) + (DAE * DAF)) + (DAG * DAH);
                let DAJ = if DAI > A { 1.0 } else { 0.0 };
                let FJK;
                let FJN;
                let FJP;
                let FJR;
                let FJU;
                if DAJ != 0.0 {
                    let DAK = AWP * CUV;
                    let DAM = -DAL;
                    let DAO = (rspice_limited_exp((DAM / DAK))) * DAN;
                    let DAQ = (B + (if (parameters[1622] / DAI) >= DAP { (parameters[1622] / DAI) } else { DAP })) - DAO;
                    let DAR = AGZ * (DAQ + (((DAQ * DAQ) + (Q * DAO)).sqrt()));
                    let DAS = if DAR > BZ { 1.0 } else { 0.0 };
                    let DAV = if DAS != 0.0 {
                        let DAT = DAR.ln();
                        DAT
                    } else {
                        DAU
                    };
                    let DAW = DAK * DAV;
                    let DAX = (parameters[1624] / DAI) - DAP;
                    let DAY = if DAX < -1e1f64 { 1.0 } else { 0.0 };
                    let DBB = if DAY != 0.0 {
                        let DAZ = -1e-6f64 / DAX;
                        DAZ
                    } else {
                        let DBA = AGZ * (DAX + (((DAX * DAX) + 4e-6f64).sqrt()));
                        DBA
                    };
                    let DBC = ((DBB + DAP) - B) / DAN;
                    let DBD = if DBC > BZ { 1.0 } else { 0.0 };
                    let DBG = if DBD != 0.0 {
                        let DBE = DBC.ln();
                        DBE
                    } else {
                        DBF
                    };
                    let DBH = DAM - (DAK * DBG);
                    let DBI = DAN * (rspice_limited_exp(((-(DAL + DBH)) / DAK)));
                    let DBJ = DAI * (B + DBI);
                    let DBK = ((-DAI) * DBI) / DAK;
                    FJK = DBH;
                    FJN = DAK;
                    FJP = DBJ;
                    FJR = DBK;
                    FJU = DAW;
                } else {
                    FJK = A;
                    FJN = A;
                    FJP = A;
                    FJR = A;
                    FJU = A;
                }
                let DBQ = ((DBL * DBM) + (DBN * DBO)) + (DAG * DBP);
                let DBR = if DBQ > A { 1.0 } else { 0.0 };
                let FKM;
                let FKP;
                let FKR;
                let FKT;
                let FKW;
                if DBR != 0.0 {
                    let DBS = AWP * CVA;
                    let DBU = -DBT;
                    let DBW = (rspice_limited_exp((DBU / DBS))) * DBV;
                    let DBX = (B + (if (parameters[1623] / DBQ) >= DAP { (parameters[1623] / DBQ) } else { DAP })) - DBW;
                    let DBY = AGZ * (DBX + (((DBX * DBX) + (Q * DBW)).sqrt()));
                    let DBZ = if DBY > BZ { 1.0 } else { 0.0 };
                    let DCC = if DBZ != 0.0 {
                        let DCA = DBY.ln();
                        DCA
                    } else {
                        DCB
                    };
                    let DCD = DBS * DCC;
                    let DCE = (parameters[1625] / DBQ) - DAP;
                    let DCF = if DCE < -1e1f64 { 1.0 } else { 0.0 };
                    let DCI = if DCF != 0.0 {
                        let DCG = -1e-6f64 / DCE;
                        DCG
                    } else {
                        let DCH = AGZ * (DCE + (((DCE * DCE) + 4e-6f64).sqrt()));
                        DCH
                    };
                    let DCJ = ((DCI + DAP) - B) / DBV;
                    let DCK = if DCJ > BZ { 1.0 } else { 0.0 };
                    let DCN = if DCK != 0.0 {
                        let DCL = DCJ.ln();
                        DCL
                    } else {
                        DCM
                    };
                    let DCO = DBU - (DBS * DCN);
                    let DCP = DBV * (rspice_limited_exp(((-(DBT + DCO)) / DBS)));
                    let DCQ = DBQ * (B + DCP);
                    let DCR = ((-DBQ) * DCP) / DBS;
                    FKM = DCO;
                    FKP = DBS;
                    FKR = DCQ;
                    FKT = DCR;
                    FKW = DCD;
                } else {
                    FKM = A;
                    FKP = A;
                    FKR = A;
                    FKT = A;
                    FKW = A;
                }
                let DCU = DCS * DAC;
                let DCX = DCV * DAE;
                let DDA = (DCY * HH) * BK;
                let DDD = DDB * DBL;
                let DDG = DDE * DBN;
                let DDJ = (DDH * HH) * BK;
                let DDL = if DDK > A { 1.0 } else { 0.0 };
                let FLR;
                let FMC;
                if DDL != 0.0 {
                    let DDO = DDM * (B - ((B / DDK).powf((B / DDN))));
                    let DDQ = (((DDM * DDK) * DDP) / DDN) / ((B - (DDO / DDM)).powf((-(B + DDN))));
                    FLR = DDO;
                    FMC = DDQ;
                } else {
                    FLR = A;
                    FMC = A;
                }
                let DDS = if DDR > A { 1.0 } else { 0.0 };
                let FMT;
                let FNE;
                if DDS != 0.0 {
                    let DDV = DDT * (B - ((B / DDR).powf((B / DDU))));
                    let DDX = (((DDT * DDR) * DDW) / DDU) / ((B - (DDV / DDT)).powf((-(B + DDU))));
                    FMT = DDV;
                    FNE = DDX;
                } else {
                    FMT = A;
                    FNE = A;
                }
                let DDZ = if DDY > A { 1.0 } else { 0.0 };
                let FNV;
                let FOG;
                if DDZ != 0.0 {
                    let DEC = DEA * (B - ((B / DDY).powf((B / DEB))));
                    let DEE = (((DEA * DDY) * DED) / DEB) / ((B - (DEC / DEA)).powf((-(B + DEB))));
                    FNV = DEC;
                    FOG = DEE;
                } else {
                    FNV = A;
                    FOG = A;
                }
                let DEG = if DEF > A { 1.0 } else { 0.0 };
                let FOX;
                let FPI;
                if DEG != 0.0 {
                    let DEJ = DEH * (B - ((B / DEF).powf((B / DEI))));
                    let DEL = (((DEH * DEF) * DEK) / DEI) / ((B - (DEJ / DEH)).powf((-(B + DEI))));
                    FOX = DEJ;
                    FPI = DEL;
                } else {
                    FOX = A;
                    FPI = A;
                }
                let DEN = if DEM > A { 1.0 } else { 0.0 };
                let FPZ;
                let FQK;
                if DEN != 0.0 {
                    let DEQ = DEO * (B - ((B / DEM).powf((B / DEP))));
                    let DES = (((DEO * DEM) * DER) / DEP) / ((B - (DEQ / DEO)).powf((-(B + DEP))));
                    FPZ = DEQ;
                    FQK = DES;
                } else {
                    FPZ = A;
                    FQK = A;
                }
                let DEU = if DET > A { 1.0 } else { 0.0 };
                let FRB;
                let FRM;
                if DEU != 0.0 {
                    let DEX = DEV * (B - ((B / DET).powf((B / DEW))));
                    let DEZ = (((DEV * DET) * DEY) / DEW) / ((B - (DEX / DEV)).powf((-(B + DEW))));
                    FRB = DEX;
                    FRM = DEZ;
                } else {
                    FRB = A;
                    FRM = A;
                }
                FJH = DAI;
                FJJ = FJK;
                FJM = FJN;
                FJO = FJP;
                FJQ = FJR;
                FJT = FJU;
                FKJ = DBQ;
                FKL = FKM;
                FKO = FKP;
                FKQ = FKR;
                FKS = FKT;
                FKV = FKW;
                FLL = DCU;
                FLQ = FLR;
                FMB = FMC;
                FMN = DCX;
                FMS = FMT;
                FND = FNE;
                FNP = DDA;
                FNU = FNV;
                FOF = FOG;
                FOR = DDD;
                FOW = FOX;
                FPH = FPI;
                FPT = DDG;
                FPY = FPZ;
                FQJ = FQK;
                FQV = DDJ;
                FRA = FRB;
                FRL = FRM;
            } else {
                FJH = A;
                FJJ = A;
                FJM = A;
                FJO = A;
                FJQ = A;
                FJT = A;
                FKJ = A;
                FKL = A;
                FKO = A;
                FKQ = A;
                FKS = A;
                FKV = A;
                FLL = A;
                FLQ = A;
                FMB = A;
                FMN = A;
                FMS = A;
                FND = A;
                FNP = A;
                FNU = A;
                FOF = A;
                FOR = A;
                FOW = A;
                FPH = A;
                FPT = A;
                FPY = A;
                FQJ = A;
                FQV = A;
                FRA = A;
                FRL = A;
            }
            let DFC = CWN * (DFA - DFB);
            let HIC = (Lanes([0.0, GHD]) - Lanes([GHE, 0.0])) * CWN;
            let DFE = CWN * (DFD - DFB);
            let HID = (Lanes([GHF, 0.0]) - Lanes([0.0, GHE])) * CWN;
            let DFF = CWN * (DFA - DFD);
            let DFH = CWN * (DFG - DFB);
            let HIE = (Lanes([GHG, 0.0]) - Lanes([0.0, GHE])) * CWN;
            let DFI = CWN * (DFG - DFD);
            let HIF = (Lanes([GHG, 0.0]) - Lanes([0.0, GHF])) * CWN;
            let DFJ = CWN * (DFA - DFG);
            let DFK = if ADW != R { 1.0 } else { 0.0 };
            let EZB;
            let FAU;
            if DFK != 0.0 {
                let DFM = CWN * (DFL - DFD);
                let DFN = CWN * (DFL - DFB);
                EZB = DFM;
                FAU = DFN;
            } else {
                let DFP = CWN * (DFO - DFD);
                let DFQ = CWN * (node_potentials[13] - DFB);
                EZB = DFP;
                FAU = DFQ;
            }
            let DFR = if DFE < A { 1.0 } else { 0.0 };
            let DFW;
            let DFY;
            let DGB;
            let EYP;
            let GOL;
            let GOM;
            let GON;
            if DFR != 0.0 {
                let DFT = DFC - DFE;
                let HII = Lanes([0.0, HIC[0], HIC[1]]) - Lanes([HID[0], HID[1], 0.0]);
                let DFV = DFU * DFE;
                let HIJ = HID * DFU;
                let HIK = Lanes([HIF[0], HIF[1], 0.0]);
                DFW = DFT;
                DFY = DFV;
                DGB = DFI;
                EYP = DFS;
                GOL = HII;
                GOM = HIJ;
                GON = HIK;
            } else {
                let HIG = Lanes([0.0, HIC[0], HIC[1]]);
                let HIH = Lanes([HIE[0], 0.0, HIE[1]]);
                DFW = DFC;
                DFY = DFE;
                DGB = DFH;
                EYP = B;
                GOL = HIG;
                GOM = HID;
                GON = HIH;
            }
            let DFX = DFW - CZW;
            let HIL = Lanes([0.0, GOL[0], GOL[1], GOL[2]]) - Lanes([HHZ, 0.0, 0.0, 0.0]);
            let HIM = GOM * DFY;
            let DFZ = ((DFY * DFY) + YC).sqrt();
            let HIN = (HIM + HIM) * (GHB / (GRJ * DFZ));
            let DGA = DFZ - ASN;
            let DUT;
            let GOO;
            if AS != 0.0 {
                let HIP = (GOM - HIN) * AGZ;
                let DGD = 9.5e-1f64 * DGC;
                let HIQ = (GON - Lanes([0.0, HIP[0], HIP[1]])) * GRP;
                let DGE = (DGD - (DGB - (AGZ * (DFY - DGA)))) - ABQ;
                let HIR = HIQ * DGE;
                let DGG = ((DGE * DGE) + (DGF * DGD)).sqrt();
                let DGH = DGD - (AGZ * (DGE + DGG));
                let HIS = ((HIQ + ((HIR + HIR) * (GHB / (GRJ * DGG)))) * AGZ) * GRP;
                DUT = DGH;
                GOO = HIS;
            } else {
                DUT = A;
                GOO = HIO;
            }
            let HIT = HID * YR;
            let DGI = (YR * DFE) / AWP;
            let DGJ = DGI.tanh();
            let HIU = (((Lanes([0.0, HIT[0], HIT[1]]) - Lanes([(GRH * DGI), 0.0, 0.0])) / AWP) * (GHB - (DGJ * DGJ))) * AGZ;
            let DGK = AGZ + (AGZ * DGJ);
            let DGL = B - DGK;
            let HIV = HIU * GRP;
            let DKQ;
            let DKY;
            let DLD;
            let DMN;
            let DNC;
            let DNE;
            let DUP;
            let DUR;
            let DUS;
            let DUV;
            let DWD;
            let DWH;
            let EQV;
            let EQX;
            let ERV;
            let ESP;
            let ETM;
            let EWQ;
            let EWY;
            let GOP;
            let GOQ;
            let GOR;
            let GOS;
            let GOT;
            let GOU;
            let GOV;
            let GOW;
            let GOX;
            let GOY;
            let GOZ;
            let GPA;
            let GPB;
            let GPC;
            let GPD;
            let GPE;
            let GPF;
            if OU != 0.0 {
                let DGM = (YX * DGL) + (YV * DGK);
                let HJJ = (HIV * YX) + (HIU * YV);
                let DGP = (DGN * DGL) + (DGO * DGK);
                let HJK = (Lanes([(GNW * DGL), 0.0, 0.0]) + (HIV * DGN)) + (Lanes([(GNU * DGK), 0.0, 0.0]) + (HIU * DGO));
                let DGQ = (AAH * DGL) + (KC * DGK);
                let HJL = (HIV * AAH) + (HIU * KC);
                let DGR = (AAJ * DGL) + (KD * DGK);
                let HJM = (HIV * AAJ) + (HIU * KD);
                let DGZ = (DGS * DGL) + (DGX * DGK);
                let HJN = (Lanes([(GIF * DGL), 0.0, 0.0]) + (HIV * DGS)) + (Lanes([(GIG * DGK), 0.0, 0.0]) + (HIU * DGX));
                let DHD = (DHA * DGL) + (DHC * DGK);
                let HJO = (Lanes([(GOC * DGL), 0.0, 0.0]) + (HIV * DHA)) + (Lanes([(GOA * DGK), 0.0, 0.0]) + (HIU * DHC));
                let DHQ = (DHE * DGL) + (DHP * DGK);
                let HJP = (Lanes([(GIH * DGL), 0.0, 0.0]) + (HIV * DHE)) + (Lanes([(GNJ * DGK), 0.0, 0.0]) + (HIU * DHP));
                let DHU = (DHR * DGL) + (DHT * DGK);
                let DHY = (DHV * DGL) + (DHX * DGK);
                let DIE = (DHZ * DGL) + (DIA * DGK);
                let HJQ = (HIV * DHZ) + (Lanes([(GII * DGK), 0.0, 0.0]) + (HIU * DIA));
                let DIR = (DIF * DGL) + (DIQ * DGK);
                let HJR = (Lanes([(GIJ * DGL), 0.0, 0.0]) + (HIV * DIF)) + (Lanes([(GNI * DGK), 0.0, 0.0]) + (HIU * DIQ));
                let DIX = (DIS * DGL) + (DIT * DGK);
                let HJS = (HIV * DIS) + (Lanes([(GIK * DGK), 0.0, 0.0]) + (HIU * DIT));
                let DIZ = (DIY * DGL) + (IE * DGK);
                let HJT = (HIV * DIY) + (HIU * IE);
                let DJB = (DJA * DGL) + (HS * DGK);
                let HJU = (HIV * DJA) + (HIU * HS);
                let DJJ = (DJC * DGL) + (DJH * DGK);
                let HJV = (Lanes([(GIL * DGL), 0.0, 0.0]) + (HIV * DJC)) + (Lanes([(GIM * DGK), 0.0, 0.0]) + (HIU * DJH));
                let DJR = (DJK * DGL) + (DJP * DGK);
                let HJW = (Lanes([(GIN * DGL), 0.0, 0.0]) + (HIV * DJK)) + (Lanes([(GIO * DGK), 0.0, 0.0]) + (HIU * DJP));
                let DJZ = (DJS * DGL) + (DJX * DGK);
                let HJX = (Lanes([(GIP * DGL), 0.0, 0.0]) + (HIV * DJS)) + (Lanes([(GIQ * DGK), 0.0, 0.0]) + (HIU * DJX));
                let DKI = (DKA * DGL) + (DKF * DGK);
                let HJY = (Lanes([(GNK * DGL), 0.0, 0.0]) + (HIV * DKA)) + (Lanes([(GNL * DGK), 0.0, 0.0]) + (HIU * DKF));
                let DKP = (DKJ * DGL) + (DKL * DGK);
                let HJZ = (HIV * DKJ) + (Lanes([(GIR * DGK), 0.0, 0.0]) + (HIU * DKL));
                DKQ = DGZ;
                DKY = DGM;
                DLD = DJB;
                DMN = DGP;
                DNC = DIZ;
                DNE = DJJ;
                DUP = DKP;
                DUR = DJR;
                DUS = DKI;
                DUV = DJZ;
                DWD = DIR;
                DWH = DIX;
                EQV = DGQ;
                EQX = DGR;
                ERV = DIE;
                ESP = DHQ;
                ETM = DHD;
                EWQ = DHU;
                EWY = DHY;
                GOP = HJN;
                GOQ = HJJ;
                GOR = HJU;
                GOS = HJK;
                GOT = HJT;
                GOU = HJV;
                GOV = HJZ;
                GOW = HJW;
                GOX = HJY;
                GOY = HJX;
                GOZ = HJR;
                GPA = HJS;
                GPB = HJL;
                GPC = HJM;
                GPD = HJQ;
                GPE = HJP;
                GPF = HJO;
            } else {
                let HIW = Lanes([GIG, 0.0, 0.0]);
                let HIY = Lanes([GNU, 0.0, 0.0]);
                let HIZ = Lanes([GIM, 0.0, 0.0]);
                let HJA = Lanes([GIR, 0.0, 0.0]);
                let HJB = Lanes([GIO, 0.0, 0.0]);
                let HJC = Lanes([GNL, 0.0, 0.0]);
                let HJD = Lanes([GIQ, 0.0, 0.0]);
                let HJE = Lanes([GNI, 0.0, 0.0]);
                let HJF = Lanes([GIK, 0.0, 0.0]);
                let HJG = Lanes([GII, 0.0, 0.0]);
                let HJH = Lanes([GNJ, 0.0, 0.0]);
                let HJI = Lanes([GOA, 0.0, 0.0]);
                DKQ = DGX;
                DKY = YV;
                DLD = HS;
                DMN = DGO;
                DNC = IE;
                DNE = DJH;
                DUP = DKL;
                DUR = DJP;
                DUS = DKF;
                DUV = DJX;
                DWD = DIQ;
                DWH = DIT;
                EQV = KC;
                EQX = KD;
                ERV = DIA;
                ESP = DHP;
                ETM = DHC;
                EWQ = DHT;
                EWY = DHX;
                GOP = HIW;
                GOQ = HIX;
                GOR = HIX;
                GOS = HIY;
                GOT = HIX;
                GOU = HIZ;
                GOV = HJA;
                GOW = HJB;
                GOX = HJC;
                GOY = HJD;
                GOZ = HJE;
                GPA = HJF;
                GPB = HIX;
                GPC = HIX;
                GPD = HJG;
                GPE = HJH;
                GPF = HJI;
            }
            let DKR = B / DKQ;
            let HKA = ((GOP * DKR) * GRP) / DKQ;
            let DKT = (4e-1f64 + DKS) + HY;
            let DKU = R * HG;
            let DKV = DKU / (HC + R);
            let HKB = HIN * DKY;
            let DKZ = DKW * (YT + (DKY * DGA));
            let HKC = ((GOQ * DGA) + Lanes([0.0, HKB[0], HKB[1]])) * DKW;
            let DLB = if DLA == A { 1.0 } else { 0.0 };
            let DLK;
            let GPG;
            if DLB != 0.0 {
                let DLL;
                let GPH;
                if AZS != 0.0 {
                    let DLC = AWP * BAK;
                    let DLE = B + ((DLD + DKZ) / DKV);
                    let DLF = DLC * DLE;
                    let HKE = Lanes([(((GRH * BAK) + (GIB * AWP)) * DLE), 0.0, 0.0]) + (((GOR + HKC) / DKV) * DLC);
                    DLL = DLF;
                    GPH = HKE;
                } else {
                    let DLH = DLG * BAK;
                    let DLI = B + ((DLD + DKZ) / DKV);
                    let DLJ = DLH * DLI;
                    let HKD = Lanes([(((GHO * BAK) + (GIB * DLG)) * DLI), 0.0, 0.0]) + (((GOR + HKC) / DKV) * DLH);
                    DLL = DLJ;
                    GPH = HKD;
                }
                DLK = DLL;
                GPG = GPH;
            } else {
                DLK = DLA;
                GPG = HIX;
            }
            let DLM = HF / DLK;
            let HKF = ((GPG * DLM) * GRP) / DLK;
            let DLN = ((HD * BAX) * R) * GO;
            let DLO = (FW * DLK) / DLN;
            let HKG = ((GPG * FW) - Lanes([((((GHY * HD) * R) * GO) * DLO), 0.0, 0.0])) / DLN;
            let DLP = if DLO > BZ { 1.0 } else { 0.0 };
            let DLS;
            let GPI;
            if DLP != 0.0 {
                let DLQ = DLO.ln();
                let HKH = HKG * (GHB / DLO);
                DLS = DLQ;
                GPI = HKH;
            } else {
                DLS = DLR;
                GPI = HIX;
            }
            let DLT = DLM * HC;
            let HKI = HKF * HC;
            let HKJ = HKI * DLT;
            let DLU = ((rspice_limited_exp(DLT)) - DLT) - B;
            let DLV = (DLT * DLT) / DLU;
            let HKK = ((HKJ + HKJ) - (((HKI * (rspice_limited_exp_derivative(DLT))) - HKI) * DLV)) / DLU;
            let DLW = if DLV > BZ { 1.0 } else { 0.0 };
            let DLZ;
            let GPJ;
            if DLW != 0.0 {
                let DLX = DLV.ln();
                let HKL = HKK * (GHB / DLV);
                DLZ = DLX;
                GPJ = HKL;
            } else {
                DLZ = DLY;
                GPJ = HIX;
            }
            let DMA = DLZ + DLS;
            let HKM = GPJ + GPI;
            let HKN = (GPG * DAP) / HC;
            let DMB = R * APG;
            let DMC = ((DAP * DLK) / HC) + DMB;
            let DMD = GF * L;
            let DME = (AWP * FW) / DMD;
            let DMG = parameters[1804] * 1.2879922655862042e-25f64;
            let DMH = (DMG * (DME.powf(DMF))) / BAL;
            let HKO = (((((GRH * FW) / DMD) * (DMF * (DME.powf(-3.33333333e-1f64)))) * DMG) - (GSW * DMH)) / BAL;
            let DMK = (-HV) * DMI;
            let DMM = DMK * (DML - DKT);
            let DMQ = (-DMN) * DMO;
            let DMR = DGA + YC;
            let DMS = DMR.sqrt();
            let DMT = DGA + (IA * DMS);
            let HKP = (HIN + ((HIN * (GHB / (GRJ * DMS))) * IA)) * DMQ;
            let DMV = XJ * DMU;
            let DMW = DMV * (DMR.powf(XK));
            let HKQ = (HIN * (XK * (DMR.powf((XK - GHB))))) * DMV;
            let DMX = IC * AOZ;
            let DMY = DKT.sqrt();
            let DMZ = DMX * DMY;
            let HKR = (((Lanes([((GOG - GOF) * DMK), 0.0, 0.0]) + (((((GOS * GRP) * DMO) * DMT) + Lanes([0.0, HKP[0], HKP[1]])) + Lanes([0.0, HKQ[0], HKQ[1]]))) + Lanes([((GOF * (GHB / (GRJ * DMY))) * DMX), 0.0, 0.0])) + Lanes([GIS, 0.0, 0.0])) + GOT;
            let DND = DFX - ((((DMM + ((DMQ * DMT) + DMW)) + DMZ) + DNA) + DNC);
            let HKS = HIL - Lanes([HKR[0], HKR[1], HKR[2], 0.0]);
            let DNF = ((DNE * HG) * HH) / RB;
            let HKT = ((GOU * HG) * HH) / RB;
            let DPK;
            let GPK;
            if AZS != 0.0 {
                let DNH = (DNF * DLK) * HD;
                let DNI = (DNH * BAX) * CS;
                let DNJ = (DKU * DNG) / DNI;
                let DNK = DNJ.powf(DLK);
                let HLF = ((((((((((HKT * DLK) + (GPG * DNF)) * HD) * BAX) + Lanes([(GHY * DNH), 0.0, 0.0])) * CS) * DNJ) * GRP) / DNI) * (DLK * (DNJ.powf((DLK - GHB))))) + (GPG * (DNK * (DNJ.ln())));
                let DNL = if DNK > BZ { 1.0 } else { 0.0 };
                let DNO;
                let GPL;
                if DNL != 0.0 {
                    let DNM = DNK.ln();
                    let HLG = HLF * (GHB / DNK);
                    DNO = DNM;
                    GPL = HLG;
                } else {
                    DNO = DNN;
                    GPL = HIX;
                }
                let DNP = -(BBH + DNO);
                let HLH = (Lanes([GSZ, 0.0, 0.0]) + GPL) * GRP;
                let HLI = Lanes([HLH[0], HLH[1], HLH[2], 0.0]);
                let HLJ = HKS + HLI;
                let DNR = (DND + DNP) + DNQ;
                let DNS = if DNR < -1e0f64 { 1.0 } else { 0.0 };
                let DNW;
                let GPM;
                if DNS != 0.0 {
                    let DNT = -1e-8f64 / DNR;
                    let HLM = ((HLJ * DNT) * GRP) / DNR;
                    DNW = DNT;
                    GPM = HLM;
                } else {
                    let HLK = HLJ * DNR;
                    let DNU = ((DNR * DNR) + 4e-8f64).sqrt();
                    let DNV = AGZ * (DNR + DNU);
                    let HLL = (HLJ + ((HLK + HLK) * (GHB / (GRJ * DNU)))) * AGZ;
                    DNW = DNV;
                    GPM = HLL;
                }
                let DNX = DNW - DNP;
                let HLN = GPM - HLI;
                DPK = DNX;
                GPK = HLN;
            } else {
                let DNY = (DNF * DLK) * HD;
                let DNZ = (DNY * BAX) * CS;
                let DOA = (DKU * DNG) / DNZ;
                let HKU = ((((((((HKT * DLK) + (GPG * DNF)) * HD) * BAX) + Lanes([(GHY * DNY), 0.0, 0.0])) * CS) * DOA) * GRP) / DNZ;
                let DOB = if DOA > BZ { 1.0 } else { 0.0 };
                let DOF;
                let GPN;
                if DOB != 0.0 {
                    let DOC = DOA.ln();
                    let HKV = HKU * (GHB / DOA);
                    DOF = DOC;
                    GPN = HKV;
                } else {
                    DOF = DOD;
                    GPN = HIX;
                }
                let DOE = -DLK;
                let DOG = DOE * DOF;
                let HKW = ((GPG * GRP) * DOF) + (GPN * DOE);
                let DOH = DOG - YC;
                let HKX = HKW * DOH;
                let DOI = ((DOH * DOH) + 2.5e-9f64).sqrt();
                let DOJ = (-BBH) + (AGZ * ((DOG + YC) + DOI));
                let HKY = Lanes([(GSZ * GRP), 0.0, 0.0]) + ((HKW + ((HKX + HKX) * (GHB / (GRJ * DOI)))) * AGZ);
                let HKZ = Lanes([HKY[0], HKY[1], HKY[2], 0.0]);
                let HLA = HKS + HKZ;
                let DOK = (DND + DOJ) + DNQ;
                let DOL = if DOK < -1e0f64 { 1.0 } else { 0.0 };
                let DOP;
                let GPO;
                if DOL != 0.0 {
                    let DOM = -1e-8f64 / DOK;
                    let HLD = ((HLA * DOM) * GRP) / DOK;
                    DOP = DOM;
                    GPO = HLD;
                } else {
                    let HLB = HLA * DOK;
                    let DON = ((DOK * DOK) + 4e-8f64).sqrt();
                    let DOO = AGZ * (DOK + DON);
                    let HLC = (HLA + ((HLB + HLB) * (GHB / (GRJ * DON)))) * AGZ;
                    DOP = DOO;
                    GPO = HLC;
                }
                let DOQ = DOP - DOJ;
                let HLE = GPO - HKZ;
                DPK = DOQ;
                GPK = HLE;
            }
            let DOR = -DLM;
            let HLO = HKF * GRP;
            let DOS = DOR.powf(DMF);
            let HLP = HLO * (DMF * (DOR.powf(-3.33333333e-1f64)));
            let DPM;
            let DPO;
            let GPP;
            let GPQ;
            if AS != 0.0 {
                let DOT = R * DKS;
                let HLU = GOF * R;
                let DOU = (DOT + BBH) - DGB;
                let HLV = Lanes([0.0, (HLU + GSZ), 0.0, 0.0]) - Lanes([GON[0], 0.0, GON[1], GON[2]]);
                let DOV = if DOU < -1e3f64 { 1.0 } else { 0.0 };
                let DOZ;
                let GPR;
                if DOV != 0.0 {
                    let DOW = -1.0000000000000002e-2f64 / DOU;
                    let HLY = ((HLV * DOW) * GRP) / DOU;
                    DOZ = DOW;
                    GPR = HLY;
                } else {
                    let HLW = HLV * DOU;
                    let DOX = ((DOU * DOU) + 4.000000000000001e-2f64).sqrt();
                    let DOY = AGZ * (DOU + DOX);
                    let HLX = (HLV + ((HLW + HLW) * (GHB / (GRJ * DOX)))) * AGZ;
                    DOZ = DOY;
                    GPR = HLX;
                }
                let DPA = R * DLK;
                let DPB = (-COO) / DPA;
                let DPC = DOZ.sqrt();
                let DPD = DOT.sqrt();
                let DPE = DPC - DPD;
                let HLZ = ((Lanes([(HHD * GRP), 0.0, 0.0]) - ((GPG * R) * DPB)) / DPA) * DPE;
                let DPF = DOR - (DPB * DPE);
                let HMA = Lanes([0.0, HLO[0], HLO[1], HLO[2]]) - (Lanes([0.0, HLZ[0], HLZ[1], HLZ[2]]) + (((GPR * (GHB / (GRJ * DPC))) - Lanes([0.0, (HLU * (GHB / (GRJ * DPD))), 0.0, 0.0])) * DPB));
                let HMB = Lanes([(HKO * DOS), 0.0, 0.0]) + (HLP * DMH);
                let DPG = (DPF + DMA) + (DMH * DOS);
                let HMC = (HMA + Lanes([0.0, HKM[0], HKM[1], HKM[2]])) + Lanes([0.0, HMB[0], HMB[1], HMB[2]]);
                let DPH = DPF + DLS;
                let HMD = HMA + Lanes([0.0, GPI[0], GPI[1], GPI[2]]);
                DPM = DPH;
                DPO = DPG;
                GPP = HMD;
                GPQ = HMC;
            } else {
                let DPI = (DOR + DMA) + (DMH * DOS);
                let HLQ = (HLO + HKM) + (Lanes([(HKO * DOS), 0.0, 0.0]) + (HLP * DMH));
                let DPJ = DOR + DLS;
                let HLR = HLO + GPI;
                let HLS = Lanes([0.0, HLR[0], HLR[1], HLR[2]]);
                let HLT = Lanes([0.0, HLQ[0], HLQ[1], HLQ[2]]);
                DPM = DPJ;
                DPO = DPI;
                GPP = HLS;
                GPQ = HLT;
            }
            let HME = Lanes([GSZ, 0.0, 0.0, 0.0]);
            let DPL = (DPK - BBH) / DLK;
            let HMF = GPG * DPL;
            let HMG = ((GPK - HME) - Lanes([HMF[0], HMF[1], HMF[2], 0.0])) / DLK;
            let HMH = HMG * GRP;
            let DPN = (-DPL) + DPM;
            let HMI = Lanes([0.0, HMH[0], HMH[1], HMH[2], HMH[3]]) + Lanes([GPP[0], GPP[1], GPP[2], GPP[3], 0.0]);
            let DPP = AGZ * (DPL - DPO);
            let DPQ = rspice_limited_exp(DPP);
            let HMJ = ((Lanes([0.0, HMG[0], HMG[1], HMG[2], HMG[3]]) - Lanes([GPQ[0], GPQ[1], GPQ[2], GPQ[3], 0.0])) * AGZ) * (rspice_limited_exp_derivative(DPP));
            let DPR = if DPQ > AHG { 1.0 } else { 0.0 };
            let DSS;
            let GPS;
            if DPR != 0.0 {
                let DPS = B + DPQ;
                let DPT = DPS.ln();
                let HML = (HMJ * (GHB / DPS)) * DPT;
                let DPU = (B + (DPT * DPT)).sqrt();
                let DPV = R * (B - DPU);
                let HMM = (((HML + HML) * (GHB / (GRJ * DPU))) * GRP) * R;
                let HMN = Lanes([0.0, HKF[0], HKF[1], HKF[2], 0.0]);
                let DPX = ((DPV * DPW) + DLM) * HC;
                let HMO = ((HMM * DPW) + HMN) * HC;
                let DPY = ((rspice_limited_exp(DPX)) - DPX) - B;
                let DPZ = DPX / DPY;
                let HMP = (HMO - (((HMO * (rspice_limited_exp_derivative(DPX))) - HMO) * DPZ)) / DPY;
                let DQA = DPX * DPZ;
                let HMQ = (HMO * DPZ) + (HMP * DPX);
                let DQB = -(DPV + DLM);
                let DQC = DQB.ln();
                let HMR = ((HMM + HMN) * GRP) * (GHB / DQB);
                let DQD = -DPV;
                let HMS = HMM * GRP;
                let DQE = if DQD > BZ { 1.0 } else { 0.0 };
                let DQK;
                let GPT;
                if DQE != 0.0 {
                    let DQF = DQD.ln();
                    let HMU = HMS * (GHB / DQD);
                    DQK = DQF;
                    GPT = HMU;
                } else {
                    DQK = DQG;
                    GPT = HMT;
                }
                let DQH = if DQA > BZ { 1.0 } else { 0.0 };
                let DQL;
                let GPU;
                if DQH != 0.0 {
                    let DQI = DQA.ln();
                    let HMV = HMQ * (GHB / DQA);
                    DQL = DQI;
                    GPU = HMV;
                } else {
                    DQL = DQJ;
                    GPU = HMT;
                }
                let DQM = (DMF * DQC).exp();
                let DQN = (((DPN - DPV) + DQK) + DQL) + (DMH * DQM);
                let HMW = (((HMI - HMM) + GPT) + GPU) + (Lanes([0.0, (HKO * DQM), 0.0, 0.0, 0.0]) + (((HMR * DMF) * DQM) * DMH));
                let DQO = B / DPV;
                let DQP = R / DPX;
                let DQQ = DMF * DMH;
                let HMX = HKO * DMF;
                let DQS = (DQR * DQC).exp();
                let DQT = ((-1e0f64 + DQO) + (((DQP - DPZ) - B) * HC)) - (DQQ * DQS);
                let HMY = ((((HMM * DQO) * GRP) / DPV) + (((((HMO * DQP) * GRP) / DPX) - HMP) * HC)) - (Lanes([0.0, (HMX * DQS), 0.0, 0.0, 0.0]) + (((HMR * DQR) * DQS) * DQQ));
                let DQU = DPV * DPV;
                let HMZ = HMM * DPV;
                let DQV = -1e0f64 / DQU;
                let DQX = DQW * DMH;
                let DQZ = (DQY * DQC).exp();
                let DRA = DQV - (DQX * DQZ);
                let DRB = DQN / DQT;
                let DRC = R * DQT;
                let DRD = DRC * DQT;
                let DRE = (DQN * DRA) / DRD;
                let DRF = B + DRE;
                let DRG = DPV - (DRB * DRF);
                let HNA = HMM - ((((HMW - (HMY * DRB)) / DQT) * DRF) + (((((HMW * DRA) + ((((((HMZ + HMZ) * DQV) * GRP) / DQU) - (Lanes([0.0, ((HKO * DQW) * DQZ), 0.0, 0.0, 0.0]) + (((HMR * DQY) * DQZ) * DQX))) * DQN)) - ((((HMY * R) * DQT) + (HMY * DRC)) * DRE)) / DRD) * DRB));
                let DRH = ((DRG * DPW) + DLM) * HC;
                let HNB = ((HNA * DPW) + HMN) * HC;
                let DRI = ((rspice_limited_exp(DRH)) - DRH) - B;
                let DRJ = DRH / DRI;
                let HNC = (HNB - (((HNB * (rspice_limited_exp_derivative(DRH))) - HNB) * DRJ)) / DRI;
                let DRK = DRH * DRJ;
                let HND = (HNB * DRJ) + (HNC * DRH);
                let DRL = -(DRG + DLM);
                let DRM = DRL.ln();
                let HNE = ((HNA + HMN) * GRP) * (GHB / DRL);
                let DRN = -DRG;
                let HNF = HNA * GRP;
                let DRO = if DRN > BZ { 1.0 } else { 0.0 };
                let DRU;
                let GPV;
                if DRO != 0.0 {
                    let DRP = DRN.ln();
                    let HNG = HNF * (GHB / DRN);
                    DRU = DRP;
                    GPV = HNG;
                } else {
                    DRU = DRQ;
                    GPV = HMT;
                }
                let DRR = if DRK > BZ { 1.0 } else { 0.0 };
                let DRV;
                let GPW;
                if DRR != 0.0 {
                    let DRS = DRK.ln();
                    let HNH = HND * (GHB / DRK);
                    DRV = DRS;
                    GPW = HNH;
                } else {
                    DRV = DRT;
                    GPW = HMT;
                }
                let DRW = (DMF * DRM).exp();
                let DRX = (((DPN - DRG) + DRU) + DRV) + (DMH * DRW);
                let HNI = (((HMI - HNA) + GPV) + GPW) + (Lanes([0.0, (HKO * DRW), 0.0, 0.0, 0.0]) + (((HNE * DMF) * DRW) * DMH));
                let DRY = B / DRG;
                let DRZ = R / DRH;
                let DSB = (DSA * DRM).exp();
                let DSC = ((-1e0f64 + DRY) + (((DRZ - DRJ) - B) * HC)) - (DQQ * DSB);
                let HNJ = ((((HNA * DRY) * GRP) / DRG) + (((((HNB * DRZ) * GRP) / DRH) - HNC) * HC)) - (Lanes([0.0, (HMX * DSB), 0.0, 0.0, 0.0]) + (((HNE * DSA) * DSB) * DQQ));
                let DSD = DRG * DRG;
                let HNK = HNA * DRG;
                let DSE = -1e0f64 / DSD;
                let DSG = DSF * DMH;
                let DSI = (DSH * DRM).exp();
                let DSJ = DSE - (DSG * DSI);
                let DSK = DRX / DSC;
                let DSL = R * DSC;
                let DSM = DSL * DSC;
                let DSN = (DRX * DSJ) / DSM;
                let DSO = B + DSN;
                let DSP = DRG - (DSK * DSO);
                let HNL = HNA - ((((HNI - (HNJ * DSK)) / DSC) * DSO) + (((((HNI * DSJ) + ((((((HNK + HNK) * DSE) * GRP) / DSD) - (Lanes([0.0, ((HKO * DSF) * DSI), 0.0, 0.0, 0.0]) + (((HNE * DSH) * DSI) * DSG))) * DRX)) - ((((HNJ * R) * DSC) + (HNJ * DSL)) * DSN)) / DSM) * DSK));
                DSS = DSP;
                GPS = HNL;
            } else {
                let DSQ = -DPQ;
                let DSR = DSQ * DPQ;
                let HMK = ((HMJ * GRP) * DPQ) + (HMJ * DSQ);
                DSS = DSR;
                GPS = HMK;
            }
            let DST = -DSS;
            let DSU = DST * DLK;
            let HNM = GPG * DST;
            let HNN = ((GPS * GRP) * DLK) + Lanes([0.0, HNM[0], HNM[1], HNM[2], 0.0]);
            let DUH;
            let GPX;
            if RK != 0.0 {
                let DSV = DND - BBH;
                let HNO = HKS - HME;
                let DSW = DSV / DLK;
                let HNP = GPG * DSW;
                let HNQ = (HNO - Lanes([HNP[0], HNP[1], HNP[2], 0.0])) / DLK;
                let HNR = HNQ * DSW;
                let DSY = ((DSW * DSW) + ((AQO * DSX) * DSX)).sqrt();
                let DSZ = AGZ * (DSW + DSY);
                let HNS = (HNQ + ((HNR + HNR) * (GHB / (GRJ * DSY)))) * AGZ;
                let DTB = DTA / R;
                let DTD = DTC * (DSZ.powf(DTB));
                let DTE = DSW - DSZ;
                let DTF = rspice_limited_exp(DTE);
                let DTH = (DSV - DTG) / DLK;
                let HNT = GPG * DTH;
                let HNU = (HNO - Lanes([HNT[0], HNT[1], HNT[2], 0.0])) / DLK;
                let HNV = HNU * DTH;
                let DTJ = ((DTH * DTH) + ((AQO * DTI) * DTI)).sqrt();
                let DTK = AGZ * (DTH + DTJ);
                let HNW = (HNU + ((HNV + HNV) * (GHB / (GRJ * DTJ)))) * AGZ;
                let DTM = DTL / R;
                let DTO = DTN * (DTK.powf(DTM));
                let DTP = DTH - DTK;
                let DTQ = rspice_limited_exp(DTP);
                let DTS = (DSV - DTR) / DLK;
                let HNX = GPG * DTS;
                let HNY = (HNO - Lanes([HNX[0], HNX[1], HNX[2], 0.0])) / DLK;
                let HNZ = HNY * DTS;
                let DTU = ((DTS * DTS) + ((AQO * DTT) * DTT)).sqrt();
                let DTV = AGZ * (DTS + DTU);
                let HOA = (HNY + ((HNZ + HNZ) * (GHB / (GRJ * DTU)))) * AGZ;
                let DTX = DTW / R;
                let DTZ = DTY * (DTV.powf(DTX));
                let DUA = DTS - DTV;
                let DUB = rspice_limited_exp(DUA);
                let HOB = ((((((HNS * (DTB * (DSZ.powf((DTB - GHB))))) * DTC) * DTF) + (((HNQ - HNS) * (rspice_limited_exp_derivative(DTE))) * DTD)) + ((((HNW * (DTM * (DTK.powf((DTM - GHB))))) * DTN) * DTQ) + (((HNU - HNW) * (rspice_limited_exp_derivative(DTP))) * DTO))) + ((((HOA * (DTX * (DTV.powf((DTX - GHB))))) * DTY) * DUB) + (((HNY - HOA) * (rspice_limited_exp_derivative(DUA))) * DTZ))) * DUD;
                let DUE = (DUC * DSU) + (DUD * (((DTD * DTF) + (DTO * DTQ)) + (DTZ * DUB)));
                let HOC = (HNN * DUC) + Lanes([0.0, HOB[0], HOB[1], HOB[2], HOB[3]]);
                DUH = DUE;
                GPX = HOC;
            } else {
                DUH = DSU;
                GPX = HNN;
            }
            let DUF = YC / HG;
            let DUI = DUG * DUH;
            let DUJ = ANX * (APG + DUI);
            let DUK = DUH / DUF;
            let DUL = AGZ * (B + DUK);
            let DUO = DUL.powf(DUM);
            let HOD = DUM - GHB;
            let HOE = (((GPX / DUF) * AGZ) * (DUM * (DUL.powf(HOD)))) + Lanes([0.0, (GIT * (DUO * (DUL.ln()))), 0.0, 0.0, 0.0]);
            let DUQ = DUJ.powf(DUP);
            let HOF = DUP - GHB;
            let HOG = GOV * (DUQ * (DUJ.ln()));
            let HOH = (((Lanes([0.0, (GOK * DUH), 0.0, 0.0, 0.0]) + (GPX * DUG)) * ANX) * (DUP * (DUJ.powf(HOF)))) + Lanes([0.0, HOG[0], HOG[1], HOG[2], 0.0]);
            let DVA;
            let GPY;
            if AS != 0.0 {
                let HOK = GOX * DUT;
                let HOL = GOO * DUS;
                let DUU = DUR + (DUS * DUT);
                let HOM = (Lanes([0.0, GOW[0], GOW[1], GOW[2]]) + (Lanes([0.0, HOK[0], HOK[1], HOK[2]]) + Lanes([HOL[0], 0.0, HOL[1], HOL[2]]))) * DUQ;
                let DUW = DUV / DUO;
                let DUX = (DUU * DUQ) + DUW;
                let HON = (Lanes([HOM[0], HOM[1], HOM[2], HOM[3], 0.0]) + (HOH * DUU)) + ((Lanes([0.0, GOY[0], GOY[1], GOY[2], 0.0]) - (HOE * DUW)) / DUO);
                DVA = DUX;
                GPY = HON;
            } else {
                let HOI = GOW * DUQ;
                let DUY = DUV / DUO;
                let DUZ = (DUR * DUQ) + DUY;
                let HOJ = (Lanes([0.0, HOI[0], HOI[1], HOI[2], 0.0]) + (HOH * DUR)) + ((Lanes([0.0, GOY[0], GOY[1], GOY[2], 0.0]) - (HOE * DUY)) / DUO);
                DVA = DUZ;
                GPY = HOJ;
            }
            let DVB = B + DVA;
            let DVC = DVB - B;
            let HOO = GPY * DVC;
            let DVE = (AQO * DVD) * DVD;
            let DVF = ((DVC * DVC) + DVE).sqrt();
            let DVH = (AGZ * ((DVB + B) + DVF)) / DVG;
            let HOP = ((GPY + ((HOO + HOO) * (GHB / (GRJ * DVF)))) * AGZ) / DVG;
            let DWM;
            let GPZ;
            if WJ != 0.0 {
                DWM = A;
                GPZ = HMT;
            } else {
                let DWN;
                let GQA;
                if AGE != 0.0 {
                    let DVJ = B + (DVI * DUH);
                    let DVK = B / DVJ;
                    let HOT = (((GPX * DVI) * DVK) * GRP) / DVJ;
                    let HOU = HOT * DVK;
                    let DVL = ((DVK * DVK) + YC).sqrt();
                    let DVO = ((DVM + (DVN * (AGZ * (DVK + DVL)))) * ANZ) * BK;
                    let DVR = DVO * DVP;
                    let HOV = ((((((HOT + ((HOU + HOU) * (GHB / (GRJ * DVL)))) * AGZ) * DVN) * ANZ) * BK) * DVP) + Lanes([0.0, (GIU * DVO), 0.0, 0.0, 0.0]);
                    DWN = DVR;
                    GQA = HOV;
                } else {
                    let DVS = B + (DVI * DUH);
                    let DVT = B / DVS;
                    let HOQ = (((GPX * DVI) * DVT) * GRP) / DVS;
                    let HOR = HOQ * DVT;
                    let DVU = ((DVT * DVT) + YC).sqrt();
                    let DWB = (DVV + DVY) + (((DVM + (DVN * (AGZ * (DVT + DVU)))) * ANZ) * BK);
                    let DWC = DWB * DVP;
                    let HOS = ((((((HOQ + ((HOR + HOR) * (GHB / (GRJ * DVU)))) * AGZ) * DVN) * ANZ) * BK) * DVP) + Lanes([0.0, (GIU * DWB), 0.0, 0.0, 0.0]);
                    DWN = DWC;
                    GQA = HOS;
                }
                DWM = DWN;
                GPZ = GQA;
            }
            let DWE = R * DWD;
            let DWF = DWE / DNE;
            let HOW = (((GOZ * R) - (GOU * DWF)) / DNE) * DVH;
            let DWG = (DWF * DVH) * RB;
            let HOX = (Lanes([0.0, HOW[0], HOW[1], HOW[2], 0.0]) + (HOP * DWF)) * RB;
            let DWS;
            let GQB;
            if AZS != 0.0 {
                let DWI = DUH + AZN;
                let DWJ = DWH * DWI;
                let HPA = GPA * DWI;
                let HPB = Lanes([0.0, HPA[0], HPA[1], HPA[2], 0.0]) + ((GPX + Lanes([0.0, GSK, 0.0, 0.0, 0.0])) * DWH);
                DWS = DWJ;
                GQB = HPB;
            } else {
                let DWK = DUH + (R * DLG);
                let DWL = DWH * DWK;
                let HOY = GPA * DWK;
                let HOZ = Lanes([0.0, HOY[0], HOY[1], HOY[2], 0.0]) + ((GPX + Lanes([0.0, (GHO * R), 0.0, 0.0, 0.0])) * DWH);
                DWS = DWL;
                GQB = HOZ;
            }
            let DWO = if DWM > A { 1.0 } else { 0.0 };
            let DXH;
            let GQC;
            if DWO != 0.0 {
                let DWP = (HH * DWD) * HG;
                let DWQ = DWP * DWM;
                let HPD = ((GOZ * HH) * HG) * DWM;
                let HPE = Lanes([0.0, HPD[0], HPD[1], HPD[2], 0.0]) + (GPZ * DWP);
                let DWR = R * DWQ;
                let HPF = HPE * R;
                let DWT = DQ * DWS;
                let DWU = (DWS + DWG) + (DWT * DWQ);
                let HPG = (GQB + HOX) + (((GQB * DQ) * DWQ) + (HPE * DWT));
                let DWV = R * DWS;
                let DWW = DWG + (DWV * DWQ);
                let DWX = DWS * DWW;
                let DWY = DWU * DWU;
                let HPH = HPG * DWU;
                let HPI = HPH + HPH;
                let DWZ = R * DWR;
                let DXA = DWY - (DWZ * DWX);
                let HPJ = HPI - (((HPF * R) * DWX) + (((GQB * DWW) + ((HOX + (((GQB * R) * DWQ) + (HPE * DWV))) * DWS)) * DWZ));
                let DXB = DXA.sqrt();
                let DXC = DWU + DXB;
                let DXD = DXC * DWR;
                let DXE = (DWY - DXA) / DXD;
                let HPK = ((HPI - HPJ) - ((((HPG + (HPJ * (GHB / (GRJ * DXB)))) * DWR) + (HPF * DXC)) * DXE)) / DXD;
                DXH = DXE;
                GQC = HPK;
            } else {
                let DXF = DWG + DWS;
                let DXG = (DWG * DWS) / DXF;
                let HPC = (((HOX * DWS) + (GQB * DWG)) - ((HOX + GQB) * DXG)) / DXF;
                DXH = DXG;
                GQC = HPC;
            }
            let DXI = DXH - ABQ;
            let DXJ = if DXI < -1e-1f64 { 1.0 } else { 0.0 };
            let DXN;
            let GQD;
            if DXJ != 0.0 {
                let DXK = -1.0000000000000002e-10f64 / DXI;
                let HPN = ((GQC * DXK) * GRP) / DXI;
                DXN = DXK;
                GQD = HPN;
            } else {
                let HPL = GQC * DXI;
                let DXL = ((DXI * DXI) + 4.0000000000000007e-10f64).sqrt();
                let DXM = AGZ * (DXI + DXL);
                let HPM = (GQC + ((HPL + HPL) * (GHB / (GRJ * DXL)))) * AGZ;
                DXN = DXM;
                GQD = HPM;
            }
            let DXO = DXN + ABQ;
            let DXP = DFY / DXO;
            let HPO = Lanes([0.0, 0.0, GOM[0], GOM[1], 0.0]);
            let DXQ = DXP + S;
            let DXR = DXQ.powf(DKQ);
            let HPP = GOP * (DXR * (DXQ.ln()));
            let DXS = B + DXR;
            let DXT = DXS.powf(DKR);
            let HPQ = HKA * (DXT * (DXS.ln()));
            let DXU = DFY / DXT;
            let DXV = if DXU <= DFY { DXU } else { DFY };
            let HPR = HPO + ((((HPO - (((((((HPO - (GQD * DXP)) / DXO) * (DKQ * (DXQ.powf((DKQ - GHB))))) + Lanes([0.0, HPP[0], HPP[1], HPP[2], 0.0])) * (DKR * (DXS.powf((DKR - GHB))))) + Lanes([0.0, HPQ[0], HPQ[1], HPQ[2], 0.0])) * DXU)) / DXT) - HPO) * (if DXU <= DFY { 1.0 } else { 0.0 }));
            let DXW = DXV + BBH;
            let HPS = HPR + Lanes([0.0, GSZ, 0.0, 0.0, 0.0]);
            let HPT = HLO * (DMF * (DOR.powf(-3.33333333e-1f64)));
            let DYP;
            let DYR;
            let GQE;
            let GQF;
            if AS != 0.0 {
                let DXX = R * DKS;
                let HPY = GOF * R;
                let DXY = (DXX + DXW) - DGB;
                let HPZ = (Lanes([0.0, HPY, 0.0, 0.0, 0.0]) + HPS) - Lanes([GON[0], 0.0, GON[1], GON[2], 0.0]);
                let DXZ = if DXY < -1e3f64 { 1.0 } else { 0.0 };
                let DYD;
                let GQG;
                if DXZ != 0.0 {
                    let DYA = -1.0000000000000002e-2f64 / DXY;
                    let HQC = ((HPZ * DYA) * GRP) / DXY;
                    DYD = DYA;
                    GQG = HQC;
                } else {
                    let HQA = HPZ * DXY;
                    let DYB = ((DXY * DXY) + 4.000000000000001e-2f64).sqrt();
                    let DYC = AGZ * (DXY + DYB);
                    let HQB = (HPZ + ((HQA + HQA) * (GHB / (GRJ * DYB)))) * AGZ;
                    DYD = DYC;
                    GQG = HQB;
                }
                let DYE = R * DLK;
                let DYF = (-COO) / DYE;
                let DYG = DYD.sqrt();
                let DYH = DXX.sqrt();
                let DYI = DYG - DYH;
                let HQD = ((Lanes([(HHD * GRP), 0.0, 0.0]) - ((GPG * R) * DYF)) / DYE) * DYI;
                let DYJ = DOR - (DYF * DYI);
                let HQE = Lanes([0.0, HLO[0], HLO[1], HLO[2], 0.0]) - (Lanes([0.0, HQD[0], HQD[1], HQD[2], 0.0]) + (((GQG * (GHB / (GRJ * DYG))) - Lanes([0.0, (HPY * (GHB / (GRJ * DYH))), 0.0, 0.0, 0.0])) * DYF));
                let HQF = Lanes([(HKO * DOS), 0.0, 0.0]) + (HPT * DMH);
                let DYK = (DYJ + DMA) + (DMH * DOS);
                let HQG = (HQE + Lanes([0.0, HKM[0], HKM[1], HKM[2], 0.0])) + Lanes([0.0, HQF[0], HQF[1], HQF[2], 0.0]);
                let DYL = DYJ + DLS;
                let HQH = HQE + Lanes([0.0, GPI[0], GPI[1], GPI[2], 0.0]);
                DYP = DYL;
                DYR = DYK;
                GQE = HQH;
                GQF = HQG;
            } else {
                let DYM = (DOR + DMA) + (DMH * DOS);
                let HPU = (HLO + HKM) + (Lanes([(HKO * DOS), 0.0, 0.0]) + (HPT * DMH));
                let DYN = DOR + DLS;
                let HPV = HLO + GPI;
                let HPW = Lanes([0.0, HPV[0], HPV[1], HPV[2], 0.0]);
                let HPX = Lanes([0.0, HPU[0], HPU[1], HPU[2], 0.0]);
                DYP = DYN;
                DYR = DYM;
                GQE = HPW;
                GQF = HPX;
            }
            let DYO = (DPK - DXW) / DLK;
            let HQI = GPG * DYO;
            let HQJ = ((Lanes([0.0, GPK[0], GPK[1], GPK[2], GPK[3]]) - HPS) - Lanes([0.0, HQI[0], HQI[1], HQI[2], 0.0])) / DLK;
            let DYQ = (-DYO) + DYP;
            let HQK = (HQJ * GRP) + GQE;
            let DYS = (DYO - DYR) * AGZ;
            let DYT = rspice_limited_exp(DYS);
            let HQL = ((HQJ - GQF) * AGZ) * (rspice_limited_exp_derivative(DYS));
            let DYU = if DYT > AHG { 1.0 } else { 0.0 };
            let EBU;
            let GQH;
            if DYU != 0.0 {
                let DYV = B + DYT;
                let DYW = DYV.ln();
                let HQN = (HQL * (GHB / DYV)) * DYW;
                let DYX = (B + (DYW * DYW)).sqrt();
                let DYY = R * (B - DYX);
                let HQO = (((HQN + HQN) * (GHB / (GRJ * DYX))) * GRP) * R;
                let HQP = Lanes([0.0, HKF[0], HKF[1], HKF[2], 0.0]);
                let DYZ = ((DYY * DPW) + DLM) * HC;
                let HQQ = ((HQO * DPW) + HQP) * HC;
                let DZA = ((rspice_limited_exp(DYZ)) - DYZ) - B;
                let DZB = DYZ / DZA;
                let HQR = (HQQ - (((HQQ * (rspice_limited_exp_derivative(DYZ))) - HQQ) * DZB)) / DZA;
                let DZC = DYZ * DZB;
                let HQS = (HQQ * DZB) + (HQR * DYZ);
                let DZD = -(DYY + DLM);
                let DZE = DZD.ln();
                let HQT = ((HQO + HQP) * GRP) * (GHB / DZD);
                let DZF = -DYY;
                let HQU = HQO * GRP;
                let DZG = if DZF > BZ { 1.0 } else { 0.0 };
                let DZM;
                let GQI;
                if DZG != 0.0 {
                    let DZH = DZF.ln();
                    let HQV = HQU * (GHB / DZF);
                    DZM = DZH;
                    GQI = HQV;
                } else {
                    DZM = DZI;
                    GQI = HMT;
                }
                let DZJ = if DZC > BZ { 1.0 } else { 0.0 };
                let DZN;
                let GQJ;
                if DZJ != 0.0 {
                    let DZK = DZC.ln();
                    let HQW = HQS * (GHB / DZC);
                    DZN = DZK;
                    GQJ = HQW;
                } else {
                    DZN = DZL;
                    GQJ = HMT;
                }
                let DZO = (DMF * DZE).exp();
                let DZP = (((DYQ - DYY) + DZM) + DZN) + (DMH * DZO);
                let HQX = (((HQK - HQO) + GQI) + GQJ) + (Lanes([0.0, (HKO * DZO), 0.0, 0.0, 0.0]) + (((HQT * DMF) * DZO) * DMH));
                let DZQ = B / DYY;
                let DZR = R / DYZ;
                let DZS = DMF * DMH;
                let HQY = HKO * DMF;
                let DZU = (DZT * DZE).exp();
                let DZV = ((-1e0f64 + DZQ) + (((DZR - DZB) - B) * HC)) - (DZS * DZU);
                let HQZ = ((((HQO * DZQ) * GRP) / DYY) + (((((HQQ * DZR) * GRP) / DYZ) - HQR) * HC)) - (Lanes([0.0, (HQY * DZU), 0.0, 0.0, 0.0]) + (((HQT * DZT) * DZU) * DZS));
                let DZW = DYY * DYY;
                let HRA = HQO * DYY;
                let DZX = -1e0f64 / DZW;
                let DZZ = DZY * DMH;
                let EAB = (EAA * DZE).exp();
                let EAC = DZX - (DZZ * EAB);
                let EAD = DZP / DZV;
                let EAE = R * DZV;
                let EAF = EAE * DZV;
                let EAG = (DZP * EAC) / EAF;
                let EAH = B + EAG;
                let EAI = DYY - (EAD * EAH);
                let HRB = HQO - ((((HQX - (HQZ * EAD)) / DZV) * EAH) + (((((HQX * EAC) + ((((((HRA + HRA) * DZX) * GRP) / DZW) - (Lanes([0.0, ((HKO * DZY) * EAB), 0.0, 0.0, 0.0]) + (((HQT * EAA) * EAB) * DZZ))) * DZP)) - ((((HQZ * R) * DZV) + (HQZ * EAE)) * EAG)) / EAF) * EAD));
                let EAJ = ((EAI * DPW) + DLM) * HC;
                let HRC = ((HRB * DPW) + HQP) * HC;
                let EAK = ((rspice_limited_exp(EAJ)) - EAJ) - B;
                let EAL = EAJ / EAK;
                let HRD = (HRC - (((HRC * (rspice_limited_exp_derivative(EAJ))) - HRC) * EAL)) / EAK;
                let EAM = EAJ * EAL;
                let HRE = (HRC * EAL) + (HRD * EAJ);
                let EAN = -(EAI + DLM);
                let EAO = EAN.ln();
                let HRF = ((HRB + HQP) * GRP) * (GHB / EAN);
                let EAP = -EAI;
                let HRG = HRB * GRP;
                let EAQ = if EAP > BZ { 1.0 } else { 0.0 };
                let EAW;
                let GQK;
                if EAQ != 0.0 {
                    let EAR = EAP.ln();
                    let HRH = HRG * (GHB / EAP);
                    EAW = EAR;
                    GQK = HRH;
                } else {
                    EAW = EAS;
                    GQK = HMT;
                }
                let EAT = if EAM > BZ { 1.0 } else { 0.0 };
                let EAX;
                let GQL;
                if EAT != 0.0 {
                    let EAU = EAM.ln();
                    let HRI = HRE * (GHB / EAM);
                    EAX = EAU;
                    GQL = HRI;
                } else {
                    EAX = EAV;
                    GQL = HMT;
                }
                let EAY = (DMF * EAO).exp();
                let EAZ = (((DYQ - EAI) + EAW) + EAX) + (DMH * EAY);
                let HRJ = (((HQK - HRB) + GQK) + GQL) + (Lanes([0.0, (HKO * EAY), 0.0, 0.0, 0.0]) + (((HRF * DMF) * EAY) * DMH));
                let EBA = B / EAI;
                let EBB = R / EAJ;
                let EBD = (EBC * EAO).exp();
                let EBE = ((-1e0f64 + EBA) + (((EBB - EAL) - B) * HC)) - (DZS * EBD);
                let HRK = ((((HRB * EBA) * GRP) / EAI) + (((((HRC * EBB) * GRP) / EAJ) - HRD) * HC)) - (Lanes([0.0, (HQY * EBD), 0.0, 0.0, 0.0]) + (((HRF * EBC) * EBD) * DZS));
                let EBF = EAI * EAI;
                let HRL = HRB * EAI;
                let EBG = -1e0f64 / EBF;
                let EBI = EBH * DMH;
                let EBK = (EBJ * EAO).exp();
                let EBL = EBG - (EBI * EBK);
                let EBM = EAZ / EBE;
                let EBN = R * EBE;
                let EBO = EBN * EBE;
                let EBP = (EAZ * EBL) / EBO;
                let EBQ = B + EBP;
                let EBR = EAI - (EBM * EBQ);
                let HRM = HRB - ((((HRJ - (HRK * EBM)) / EBE) * EBQ) + (((((HRJ * EBL) + ((((((HRL + HRL) * EBG) * GRP) / EBF) - (Lanes([0.0, ((HKO * EBH) * EBK), 0.0, 0.0, 0.0]) + (((HRF * EBJ) * EBK) * EBI))) * EAZ)) - ((((HRK * R) * EBE) + (HRK * EBN)) * EBP)) / EBO) * EBM));
                EBU = EBR;
                GQH = HRM;
            } else {
                let EBS = -DYT;
                let EBT = EBS * DYT;
                let HQM = ((HQL * GRP) * DYT) + (HQL * EBS);
                EBU = EBT;
                GQH = HQM;
            }
            let EBV = -EBU;
            let EBW = EBV * DLK;
            let HRN = GPG * EBV;
            let HRO = ((GQH * GRP) * DLK) + Lanes([0.0, HRN[0], HRN[1], HRN[2], 0.0]);
            let ENY;
            let GQM;
            if RK != 0.0 {
                let EBX = DND - DXW;
                let HRP = Lanes([0.0, HKS[0], HKS[1], HKS[2], HKS[3]]) - HPS;
                let EBY = EBX / DLK;
                let HRQ = GPG * EBY;
                let HRR = (HRP - Lanes([0.0, HRQ[0], HRQ[1], HRQ[2], 0.0])) / DLK;
                let HRS = HRR * EBY;
                let EBZ = ((EBY * EBY) + ((AQO * DSX) * DSX)).sqrt();
                let ECA = AGZ * (EBY + EBZ);
                let HRT = (HRR + ((HRS + HRS) * (GHB / (GRJ * EBZ)))) * AGZ;
                let ECB = DTA / R;
                let ECC = DTC * (ECA.powf(ECB));
                let ECD = EBY - ECA;
                let ECE = rspice_limited_exp(ECD);
                let ECF = (EBX - DTG) / DLK;
                let HRU = GPG * ECF;
                let HRV = (HRP - Lanes([0.0, HRU[0], HRU[1], HRU[2], 0.0])) / DLK;
                let HRW = HRV * ECF;
                let ECG = ((ECF * ECF) + ((AQO * DTI) * DTI)).sqrt();
                let ECH = AGZ * (ECF + ECG);
                let HRX = (HRV + ((HRW + HRW) * (GHB / (GRJ * ECG)))) * AGZ;
                let ECI = DTL / R;
                let ECJ = DTN * (ECH.powf(ECI));
                let ECK = ECF - ECH;
                let ECL = rspice_limited_exp(ECK);
                let ECM = (EBX - DTR) / DLK;
                let HRY = GPG * ECM;
                let HRZ = (HRP - Lanes([0.0, HRY[0], HRY[1], HRY[2], 0.0])) / DLK;
                let HSA = HRZ * ECM;
                let ECN = ((ECM * ECM) + ((AQO * DTT) * DTT)).sqrt();
                let ECO = AGZ * (ECM + ECN);
                let HSB = (HRZ + ((HSA + HSA) * (GHB / (GRJ * ECN)))) * AGZ;
                let ECP = DTW / R;
                let ECQ = DTY * (ECO.powf(ECP));
                let ECR = ECM - ECO;
                let ECS = rspice_limited_exp(ECR);
                let ECT = (DUC * EBW) + (DUD * (((ECC * ECE) + (ECJ * ECL)) + (ECQ * ECS)));
                let HSC = (HRO * DUC) + (((((((HRT * (ECB * (ECA.powf((ECB - GHB))))) * DTC) * ECE) + (((HRR - HRT) * (rspice_limited_exp_derivative(ECD))) * ECC)) + ((((HRX * (ECI * (ECH.powf((ECI - GHB))))) * DTN) * ECL) + (((HRV - HRX) * (rspice_limited_exp_derivative(ECK))) * ECJ))) + ((((HSB * (ECP * (ECO.powf((ECP - GHB))))) * DTY) * ECS) + (((HRZ - HSB) * (rspice_limited_exp_derivative(ECR))) * ECQ))) * DUD);
                ENY = ECT;
                GQM = HSC;
            } else {
                ENY = EBW;
                GQM = HRO;
            }
            let EPW;
            let EQA;
            let EQC;
            let EVI;
            let EXY;
            if PT != 0.0 {
                let ECX = DFX - ((((DMM + ((((-ECU) * DMO) * DMT) + DMW)) + DMZ) + DNA) + DNC);
                let EDE = ((ECY * HG) * HH) / RB;
                let EEQ;
                if AZS != 0.0 {
                    let EDF = ((DKU * DNG) / ((((EDE * DLK) * HD) * BAX) * CS)).powf(DLK);
                    let EDG = if EDF > BZ { 1.0 } else { 0.0 };
                    let EDJ = if EDG != 0.0 {
                        let EDH = EDF.ln();
                        EDH
                    } else {
                        EDI
                    };
                    let EDK = -(BBH + EDJ);
                    let EDL = (ECX + EDK) + DNQ;
                    let EDM = if EDL < -1e0f64 { 1.0 } else { 0.0 };
                    let EDP = if EDM != 0.0 {
                        let EDN = -1e-8f64 / EDL;
                        EDN
                    } else {
                        let EDO = AGZ * (EDL + (((EDL * EDL) + 4e-8f64).sqrt()));
                        EDO
                    };
                    let EDQ = EDP - EDK;
                    EEQ = EDQ;
                } else {
                    let EDR = (DKU * DNG) / ((((EDE * DLK) * HD) * BAX) * CS);
                    let EDS = if EDR > BZ { 1.0 } else { 0.0 };
                    let EDV = if EDS != 0.0 {
                        let EDT = EDR.ln();
                        EDT
                    } else {
                        EDU
                    };
                    let EDW = (-DLK) * EDV;
                    let EDX = EDW - YC;
                    let EDY = (-BBH) + (AGZ * ((EDW + YC) + (((EDX * EDX) + 2.5e-9f64).sqrt())));
                    let EDZ = (ECX + EDY) + DNQ;
                    let EEA = if EDZ < -1e0f64 { 1.0 } else { 0.0 };
                    let EED = if EEA != 0.0 {
                        let EEB = -1e-8f64 / EDZ;
                        EEB
                    } else {
                        let EEC = AGZ * (EDZ + (((EDZ * EDZ) + 4e-8f64).sqrt()));
                        EEC
                    };
                    let EEE = EED - EDY;
                    EEQ = EEE;
                }
                let EES;
                let EEU;
                if AS != 0.0 {
                    let EEF = R * DKS;
                    let EEG = (EEF + BBH) - DGB;
                    let EEH = if EEG < -1e3f64 { 1.0 } else { 0.0 };
                    let EEK = if EEH != 0.0 {
                        let EEI = -1.0000000000000002e-2f64 / EEG;
                        EEI
                    } else {
                        let EEJ = AGZ * (EEG + (((EEG * EEG) + 4.000000000000001e-2f64).sqrt()));
                        EEJ
                    };
                    let EEL = DOR - (((-COO) / (R * DLK)) * ((EEK.sqrt()) - (EEF.sqrt())));
                    let EEM = (EEL + DMA) + (DMH * DOS);
                    let EEN = EEL + DLS;
                    EES = EEN;
                    EEU = EEM;
                } else {
                    let EEO = (DOR + DMA) + (DMH * DOS);
                    let EEP = DOR + DLS;
                    EES = EEP;
                    EEU = EEO;
                }
                let EER = (EEQ - BBH) / DLK;
                let EET = (-EER) + EES;
                let EEV = rspice_limited_exp((AGZ * (EER - EEU)));
                let EEW = if EEV > AHG { 1.0 } else { 0.0 };
                let EGH;
                if EEW != 0.0 {
                    let EEX = (B + EEV).ln();
                    let EEY = R * (B - ((B + (EEX * EEX)).sqrt()));
                    let EEZ = ((EEY * DPW) + DLM) * HC;
                    let EFA = EEZ / (((rspice_limited_exp(EEZ)) - EEZ) - B);
                    let EFB = EEZ * EFA;
                    let EFC = (-(EEY + DLM)).ln();
                    let EFD = -EEY;
                    let EFE = if EFD > BZ { 1.0 } else { 0.0 };
                    let EFK = if EFE != 0.0 {
                        let EFF = EFD.ln();
                        EFF
                    } else {
                        EFG
                    };
                    let EFH = if EFB > BZ { 1.0 } else { 0.0 };
                    let EFL = if EFH != 0.0 {
                        let EFI = EFB.ln();
                        EFI
                    } else {
                        EFJ
                    };
                    let EFM = (((EET - EEY) + EFK) + EFL) + (DMH * ((DMF * EFC).exp()));
                    let EFN = DMF * DMH;
                    let EFO = ((-1e0f64 + (B / EEY)) + ((((R / EEZ) - EFA) - B) * HC)) - (EFN * ((-3.33333333e-1f64 * EFC).exp()));
                    let EFP = EEY - ((EFM / EFO) * (B + ((EFM * ((-1e0f64 / (EEY * EEY)) - ((2.222222222222222e-1f64 * DMH) * ((-1.333333333e0f64 * EFC).exp())))) / ((R * EFO) * EFO))));
                    let EFQ = ((EFP * DPW) + DLM) * HC;
                    let EFR = EFQ / (((rspice_limited_exp(EFQ)) - EFQ) - B);
                    let EFS = EFQ * EFR;
                    let EFT = (-(EFP + DLM)).ln();
                    let EFU = -EFP;
                    let EFV = if EFU > BZ { 1.0 } else { 0.0 };
                    let EGB = if EFV != 0.0 {
                        let EFW = EFU.ln();
                        EFW
                    } else {
                        EFX
                    };
                    let EFY = if EFS > BZ { 1.0 } else { 0.0 };
                    let EGC = if EFY != 0.0 {
                        let EFZ = EFS.ln();
                        EFZ
                    } else {
                        EGA
                    };
                    let EGD = (((EET - EFP) + EGB) + EGC) + (DMH * ((DMF * EFT).exp()));
                    let EGE = ((-1e0f64 + (B / EFP)) + ((((R / EFQ) - EFR) - B) * HC)) - (EFN * ((-3.33333333e-1f64 * EFT).exp()));
                    let EGF = EFP - ((EGD / EGE) * (B + ((EGD * ((-1e0f64 / (EFP * EFP)) - ((2.222222222222222e-1f64 * DMH) * ((-1.333333333e0f64 * EFT).exp())))) / ((R * EGE) * EGE))));
                    EGH = EGF;
                } else {
                    let EGG = (-EEV) * EEV;
                    EGH = EGG;
                }
                let EGI = (-EGH) * DLK;
                let EGR = if RK != 0.0 {
                    let EGJ = ECX - BBH;
                    let EGK = EGJ / DLK;
                    let EGL = AGZ * (EGK + (((EGK * EGK) + ((AQO * DSX) * DSX)).sqrt()));
                    let EGM = (EGJ - DTG) / DLK;
                    let EGN = AGZ * (EGM + (((EGM * EGM) + ((AQO * DTI) * DTI)).sqrt()));
                    let EGO = (EGJ - DTR) / DLK;
                    let EGP = AGZ * (EGO + (((EGO * EGO) + ((AQO * DTT) * DTT)).sqrt()));
                    let EGQ = (DUC * EGI) + (DUD * ((((DTC * (EGL.powf((DTA / R)))) * (rspice_limited_exp((EGK - EGL)))) + ((DTN * (EGN.powf((DTL / R)))) * (rspice_limited_exp((EGM - EGN))))) + ((DTY * (EGP.powf((DTW / R)))) * (rspice_limited_exp((EGO - EGP))))));
                    EGQ
                } else {
                    EGI
                };
                let EGS = (AGZ * (B + (EGR / DUF))).powf(DUM);
                let EGT = (ANX * (APG + (DUG * EGR))).powf(DUP);
                let EHN = if AS != 0.0 {
                    let EHL = ((EGU + (EHA * DUT)) * EGT) + (EHF / EGS);
                    EHL
                } else {
                    let EHM = (EGU * EGT) + (EHF / EGS);
                    EHM
                };
                let EHO = B + EHN;
                let EHP = EHO - B;
                let EHQ = (AGZ * ((EHO + B) + (((EHP * EHP) + DVE).sqrt()))) / DVG;
                let EHZ;
                if WJ != 0.0 {
                    EHZ = A;
                } else {
                    let EIA = if AGE != 0.0 {
                        let EHR = B / (B + (DVI * EGR));
                        let EHS = (((DVM + (DVN * (AGZ * (EHR + (((EHR * EHR) + YC).sqrt()))))) * ANZ) * BK) * DVP;
                        EHS
                    } else {
                        let EHT = B / (B + (DVI * EGR));
                        let EHU = ((DVV + DVY) + (((DVM + (DVN * (AGZ * (EHT + (((EHT * EHT) + YC).sqrt()))))) * ANZ) * BK)) * DVP;
                        EHU
                    };
                    EHZ = EIA;
                }
                let EHW = (((R * EHV) / ECY) * EHQ) * RB;
                let EIE = if AZS != 0.0 {
                    let EHX = DWH * (EGR + AZN);
                    EHX
                } else {
                    let EHY = DWH * (EGR + (R * DLG));
                    EHY
                };
                let EIB = if EHZ > A { 1.0 } else { 0.0 };
                let EIK = if EIB != 0.0 {
                    let EIC = ((HH * EHV) * HG) * EHZ;
                    let EID = R * EIC;
                    let EIF = (EIE + EHW) + ((DQ * EIE) * EIC);
                    let EIG = EIF * EIF;
                    let EIH = EIG - ((R * EID) * (EIE * (EHW + ((R * EIE) * EIC))));
                    let EII = (EIG - EIH) / ((EIF + (EIH.sqrt())) * EID);
                    EII
                } else {
                    let EIJ = (EHW * EIE) / (EHW + EIE);
                    EIJ
                };
                let EIL = EIK - ABQ;
                let EIM = if EIL < -1e-1f64 { 1.0 } else { 0.0 };
                let EIP = if EIM != 0.0 {
                    let EIN = -1.0000000000000002e-10f64 / EIL;
                    EIN
                } else {
                    let EIO = AGZ * (EIL + (((EIL * EIL) + 4.0000000000000007e-10f64).sqrt()));
                    EIO
                };
                let EIQ = if (DFY / ((B + (((DFY / (EIP + ABQ)) + S).powf(DKQ))).powf(DKR))) <= DFY { (DFY / ((B + (((DFY / (EIP + ABQ)) + S).powf(DKQ))).powf(DKR))) } else { DFY };
                let EIR = EIQ + BBH;
                let EJE;
                let EJG;
                if AS != 0.0 {
                    let EIS = R * DKS;
                    let EIT = (EIS + EIR) - DGB;
                    let EIU = if EIT < -1e3f64 { 1.0 } else { 0.0 };
                    let EIX = if EIU != 0.0 {
                        let EIV = -1.0000000000000002e-2f64 / EIT;
                        EIV
                    } else {
                        let EIW = AGZ * (EIT + (((EIT * EIT) + 4.000000000000001e-2f64).sqrt()));
                        EIW
                    };
                    let EIY = DOR - (((-COO) / (R * DLK)) * ((EIX.sqrt()) - (EIS.sqrt())));
                    let EIZ = (EIY + DMA) + (DMH * DOS);
                    let EJA = EIY + DLS;
                    EJE = EJA;
                    EJG = EIZ;
                } else {
                    let EJB = (DOR + DMA) + (DMH * DOS);
                    let EJC = DOR + DLS;
                    EJE = EJC;
                    EJG = EJB;
                }
                let EJD = (EEQ - EIR) / DLK;
                let EJF = (-EJD) + EJE;
                let EJH = rspice_limited_exp(((EJD - EJG) * AGZ));
                let EJI = if EJH > AHG { 1.0 } else { 0.0 };
                let EKT;
                if EJI != 0.0 {
                    let EJJ = (B + EJH).ln();
                    let EJK = R * (B - ((B + (EJJ * EJJ)).sqrt()));
                    let EJL = ((EJK * DPW) + DLM) * HC;
                    let EJM = EJL / (((rspice_limited_exp(EJL)) - EJL) - B);
                    let EJN = EJL * EJM;
                    let EJO = (-(EJK + DLM)).ln();
                    let EJP = -EJK;
                    let EJQ = if EJP > BZ { 1.0 } else { 0.0 };
                    let EJW = if EJQ != 0.0 {
                        let EJR = EJP.ln();
                        EJR
                    } else {
                        EJS
                    };
                    let EJT = if EJN > BZ { 1.0 } else { 0.0 };
                    let EJX = if EJT != 0.0 {
                        let EJU = EJN.ln();
                        EJU
                    } else {
                        EJV
                    };
                    let EJY = (((EJF - EJK) + EJW) + EJX) + (DMH * ((DMF * EJO).exp()));
                    let EJZ = DMF * DMH;
                    let EKA = ((-1e0f64 + (B / EJK)) + ((((R / EJL) - EJM) - B) * HC)) - (EJZ * ((-3.33333333e-1f64 * EJO).exp()));
                    let EKB = EJK - ((EJY / EKA) * (B + ((EJY * ((-1e0f64 / (EJK * EJK)) - ((2.222222222222222e-1f64 * DMH) * ((-1.333333333e0f64 * EJO).exp())))) / ((R * EKA) * EKA))));
                    let EKC = ((EKB * DPW) + DLM) * HC;
                    let EKD = EKC / (((rspice_limited_exp(EKC)) - EKC) - B);
                    let EKE = EKC * EKD;
                    let EKF = (-(EKB + DLM)).ln();
                    let EKG = -EKB;
                    let EKH = if EKG > BZ { 1.0 } else { 0.0 };
                    let EKN = if EKH != 0.0 {
                        let EKI = EKG.ln();
                        EKI
                    } else {
                        EKJ
                    };
                    let EKK = if EKE > BZ { 1.0 } else { 0.0 };
                    let EKO = if EKK != 0.0 {
                        let EKL = EKE.ln();
                        EKL
                    } else {
                        EKM
                    };
                    let EKP = (((EJF - EKB) + EKN) + EKO) + (DMH * ((DMF * EKF).exp()));
                    let EKQ = ((-1e0f64 + (B / EKB)) + ((((R / EKC) - EKD) - B) * HC)) - (EJZ * ((-3.33333333e-1f64 * EKF).exp()));
                    let EKR = EKB - ((EKP / EKQ) * (B + ((EKP * ((-1e0f64 / (EKB * EKB)) - ((2.222222222222222e-1f64 * DMH) * ((-1.333333333e0f64 * EKF).exp())))) / ((R * EKQ) * EKQ))));
                    EKT = EKR;
                } else {
                    let EKS = (-EJH) * EJH;
                    EKT = EKS;
                }
                let EKU = (-EKT) * DLK;
                let ELD = if RK != 0.0 {
                    let EKV = ECX - EIR;
                    let EKW = EKV / DLK;
                    let EKX = AGZ * (EKW + (((EKW * EKW) + ((AQO * DSX) * DSX)).sqrt()));
                    let EKY = (EKV - DTG) / DLK;
                    let EKZ = AGZ * (EKY + (((EKY * EKY) + ((AQO * DTI) * DTI)).sqrt()));
                    let ELA = (EKV - DTR) / DLK;
                    let ELB = AGZ * (ELA + (((ELA * ELA) + ((AQO * DTT) * DTT)).sqrt()));
                    let ELC = (DUC * EKU) + (DUD * ((((DTC * (EKX.powf((DTA / R)))) * (rspice_limited_exp((EKW - EKX)))) + ((DTN * (EKZ.powf((DTL / R)))) * (rspice_limited_exp((EKY - EKZ))))) + ((DTY * (ELB.powf((DTW / R)))) * (rspice_limited_exp((ELA - ELB))))));
                    ELC
                } else {
                    EKU
                };
                let ELE = AGZ * (EGR + ELD);
                let ELF = EGR - ELD;
                let ELH = (EIQ * EIQ) / ELG;
                let ELJ = if ELI != A { 1.0 } else { 0.0 };
                let EPX = if ELJ != 0.0 {
                    let ELK = ELE + (((ELI * (B - (rspice_limited_exp((-ELH))))) * AGZ) * ELF);
                    ELK
                } else {
                    ELE
                };
                EPW = EPX;
                EQA = EGR;
                EQC = ELD;
                EVI = ELF;
                EXY = ELE;
            } else {
                EPW = EPY;
                EQA = EQB;
                EQC = EQD;
                EVI = EVJ;
                EXY = EXZ;
            }
            let EOL;
            let EYK;
            let EYM;
            if AS != 0.0 {
                let ELL = (COO / (R * DLK)) * (AWP.sqrt());
                let ELM = ELL / R;
                let ELN = HE / BAX;
                let ELO = if ELN > BZ { 1.0 } else { 0.0 };
                let ELR = if ELO != 0.0 {
                    let ELP = ELN.ln();
                    ELP
                } else {
                    ELQ
                };
                let ELS = (DFJ - (((CZW - AZG) - (AWP * ELR)) + parameters[1529])) / AWP;
                let ELT = if (ELS * AWP) > (DKS + (ELL * ((DKS * AWP).sqrt()))) { 1.0 } else { 0.0 };
                let EMV;
                let EMX;
                if ELT != 0.0 {
                    let ELU = (((ELS - B) + (ELM * ELM)).sqrt()) - ELM;
                    let ELV = B + (ELU * ELU);
                    let ELW = -ELV;
                    let ELX = if (ELW.abs()) < AHG { 1.0 } else { 0.0 };
                    let EMA = if ELX != 0.0 {
                        let ELY = ELW + ((AGZ * ELW) * ELW);
                        ELY
                    } else {
                        let ELZ = (rspice_limited_exp(ELW)) - B;
                        ELZ
                    };
                    EMV = EMA;
                    EMX = ELV;
                } else {
                    let EMB = (ELS * AGZ) - (DQ * (B + (ELL / 1.4142135623730951e0f64)));
                    let EMC = EMB + (((EMB * EMB) + (AUH * ELS)).sqrt());
                    let EMD = if ELS < A { 1.0 } else { 0.0 };
                    let EMW;
                    let EMY;
                    if EMD != 0.0 {
                        let EME = (ELS - EMC) / ELL;
                        let EMF = EME * EME;
                        let EMG = (-EMC) + EMF;
                        let EMH = (B - EMC) + EMF;
                        let EMI = if EMH > BZ { 1.0 } else { 0.0 };
                        let EML = if EMI != 0.0 {
                            let EMJ = EMH.ln();
                            EMJ
                        } else {
                            EMK
                        };
                        let EMM = -EML;
                        EMW = EMG;
                        EMY = EMM;
                    } else {
                        let EMN = rspice_limited_exp((-EMC));
                        let EMO = ((((ELS - B) + EMN) + (ELM * ELM)).sqrt()) - ELM;
                        let EMP = (B - EMN) + (EMO * EMO);
                        let EMQ = -EMP;
                        let EMR = if (EMQ.abs()) < AHG { 1.0 } else { 0.0 };
                        let EMU = if EMR != 0.0 {
                            let EMS = EMQ + ((AGZ * EMQ) * EMQ);
                            EMS
                        } else {
                            let EMT = (rspice_limited_exp(EMQ)) - B;
                            EMT
                        };
                        EMW = EMU;
                        EMY = EMP;
                    }
                    EMV = EMW;
                    EMX = EMY;
                }
                let EMZ = (EMV + EMX).sqrt();
                let ENB = if EMX > ENA { 1.0 } else { 0.0 };
                let ENT;
                let EYL;
                if ENB != 0.0 {
                    let ENC = EMX - (((-(ELS - EMX)) + (ELL * EMZ)) / (B - (((ELL * AGZ) * EMV) / EMZ)));
                    let END = -ENC;
                    let ENE = if (END.abs()) < AHG { 1.0 } else { 0.0 };
                    let ENH = if ENE != 0.0 {
                        let ENF = END + ((AGZ * END) * END);
                        ENF
                    } else {
                        let ENG = (rspice_limited_exp(END)) - B;
                        ENG
                    };
                    let ENI = ((-ELL) * ((ENH + ENC).sqrt())) * AWP;
                    ENT = ENC;
                    EYL = ENI;
                } else {
                    let ENJ = if EMX < -1e-15f64 { 1.0 } else { 0.0 };
                    let ENR;
                    let ENU;
                    if ENJ != 0.0 {
                        let ENK = EMX - (((-(ELS - EMX)) - (ELL * EMZ)) / (B + (((ELL * AGZ) * EMV) / EMZ)));
                        let ENL = -ENK;
                        let ENM = if (ENL.abs()) < AHG { 1.0 } else { 0.0 };
                        let ENP = if ENM != 0.0 {
                            let ENN = ENL + ((AGZ * ENL) * ENL);
                            ENN
                        } else {
                            let ENO = (rspice_limited_exp(ENL)) - B;
                            ENO
                        };
                        let ENQ = ELL * ((ENP + ENK).sqrt());
                        ENR = ENQ;
                        ENU = ENK;
                    } else {
                        ENR = A;
                        ENU = A;
                    }
                    let ENS = ENR * AWP;
                    ENT = ENU;
                    EYL = ENS;
                }
                let ENV = (ELL * (rspice_limited_exp(((-ENT) / R)))) * AWP;
                let ENW = ENT - B;
                let ENX = B + (ELL / ((AGZ * ((ENT + B) + (((ENW * ENW) + 1e0f64).sqrt()))).sqrt()));
                EOL = ENV;
                EYK = EYL;
                EYM = ENX;
            } else {
                EOL = A;
                EYK = A;
                EYM = A;
            }
            let ENZ = DUH + ENY;
            let EOA = AGZ * ENZ;
            let HSD = (GPX + GQM) * AGZ;
            let EOB = DUH - ENY;
            let HSE = GPX - GQM;
            let EOC = DXV * DXV;
            let HSF = HPR * DXV;
            let EOD = EOC / ELG;
            let HSG = (HSF + HSF) / ELG;
            let EOE = if ELI != A { 1.0 } else { 0.0 };
            let EOM;
            let GQN;
            if EOE != 0.0 {
                let EOF = -EOD;
                let EOG = (ELI * (B - (rspice_limited_exp(EOF)))) * AGZ;
                let EOH = EOA + (EOG * EOB);
                let HSH = HSD + (((((((HSG * GRP) * (rspice_limited_exp_derivative(EOF))) * GRP) * ELI) * AGZ) * EOB) + (HSE * EOG));
                EOM = EOH;
                GQN = HSH;
            } else {
                EOM = EOA;
                GQN = HSD;
            }
            let EYI = if XL != 0.0 {
                let EOJ = B / ((B / ((HG * CQ) / AHB)) + (((EOI / (B + ((EOA / parameters[400]).powf(VM)))) * IQ) / L));
                EOJ
            } else {
                HG
            };
            let EOK = if AS != 0.0 && (if IR != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EOK != 0.0 {
            } else {
            }
            let EON = ANX * (APG + (DUG * EOM));
            let HSI = (Lanes([0.0, (GOK * EOM), 0.0, 0.0, 0.0]) + (GQN * DUG)) * ANX;
            let EPH;
            let GQO;
            if AZS != 0.0 {
                let EOO = AGZ * (B + (EOM / DUF));
                let EOP = EOO.powf(DUM);
                let HSO = (((GQN / DUF) * AGZ) * (DUM * (EOO.powf(HOD)))) + Lanes([0.0, (GIT * (EOP * (EOO.ln()))), 0.0, 0.0, 0.0]);
                EPH = EOP;
                GQO = HSO;
            } else {
                let EOQ = -EOD;
                let EOR = B - (rspice_limited_exp(EOQ));
                let EOW = (EOS * DUH) + (EOU * ENY);
                let EOX = EOW * EOR;
                let HSJ = (((Lanes([0.0, (GIV * DUH), 0.0, 0.0, 0.0]) + (GPX * EOS)) + (Lanes([0.0, (GIW * ENY), 0.0, 0.0, 0.0]) + (GQM * EOU))) * EOR) + ((((HSG * GRP) * (rspice_limited_exp_derivative(EOQ))) * GRP) * EOW);
                let EOY = if EOX < -1e-8f64 { 1.0 } else { 0.0 };
                let EPC;
                let GQP;
                if EOY != 0.0 {
                    let EOZ = -1e-24f64 / EOX;
                    let HSM = ((HSJ * EOZ) * GRP) / EOX;
                    EPC = EOZ;
                    GQP = HSM;
                } else {
                    let HSK = HSJ * EOX;
                    let EPA = ((EOX * EOX) + 4e-24f64).sqrt();
                    let EPB = AGZ * (EOX + EPA);
                    let HSL = (HSJ + ((HSK + HSK) * (GHB / (GRJ * EPA)))) * AGZ;
                    EPC = EPB;
                    GQP = HSL;
                }
                let EPD = AGZ * (B + (EPC / DUF));
                let EPE = EPD.powf(DUM);
                let HSN = (((GQP / DUF) * AGZ) * (DUM * (EPD.powf(HOD)))) + Lanes([0.0, (GIT * (EPE * (EPD.ln()))), 0.0, 0.0, 0.0]);
                EPH = EPE;
                GQO = HSN;
            }
            let EPF = EON.powf(DUP);
            let HSP = GOV * (EPF * (EON.ln()));
            let HSQ = (HSI * (DUP * (EON.powf(HOF)))) + Lanes([0.0, HSP[0], HSP[1], HSP[2], 0.0]);
            let EPM;
            let GQQ;
            if AS != 0.0 {
                let HST = GOX * DUT;
                let HSU = GOO * DUS;
                let EPG = DUR + (DUS * DUT);
                let HSV = (Lanes([0.0, GOW[0], GOW[1], GOW[2]]) + (Lanes([0.0, HST[0], HST[1], HST[2]]) + Lanes([HSU[0], 0.0, HSU[1], HSU[2]]))) * EPF;
                let EPI = DUV / EPH;
                let EPJ = (EPG * EPF) + EPI;
                let HSW = (Lanes([HSV[0], HSV[1], HSV[2], HSV[3], 0.0]) + (HSQ * EPG)) + ((Lanes([0.0, GOY[0], GOY[1], GOY[2], 0.0]) - (GQO * EPI)) / EPH);
                EPM = EPJ;
                GQQ = HSW;
            } else {
                let HSR = GOW * EPF;
                let EPK = DUV / EPH;
                let EPL = (DUR * EPF) + EPK;
                let HSS = (Lanes([0.0, HSR[0], HSR[1], HSR[2], 0.0]) + (HSQ * DUR)) + ((Lanes([0.0, GOY[0], GOY[1], GOY[2], 0.0]) - (GQO * EPK)) / EPH);
                EPM = EPL;
                GQQ = HSS;
            }
            let EPN = B + EPM;
            let EPO = EPN - B;
            let HSX = GQQ * EPO;
            let EPP = ((EPO * EPO) + DVE).sqrt();
            let EPR = -parameters[888];
            let EPS = EPR * DXV;
            let EPT = DVG * (B - (EPQ * (rspice_limited_exp(EPS))));
            let EPU = (AGZ * ((EPN + B) + EPP)) / EPT;
            let HSY = (((GQQ + ((HSX + HSX) * (GHB / (GRJ * EPP)))) * AGZ) - ((((((HPR * EPR) * (rspice_limited_exp_derivative(EPS))) * EPQ) * GRP) * DVG) * EPU)) / EPT;
            let EPV = DNE / EPU;
            let HSZ = (Lanes([0.0, GOU[0], GOU[1], GOU[2], 0.0]) - (HSY * EPV)) / EPU;
            let EQO;
            if PT != 0.0 {
                let EQL;
                if AZS != 0.0 {
                    let EPZ = (AGZ * (B + (EPW / DUF))).powf(DUM);
                    EQL = EPZ;
                } else {
                    let EQE = (EOS * EQA) + (EOU * EQC);
                    let EQF = if EQE < -1e-8f64 { 1.0 } else { 0.0 };
                    let EQI = if EQF != 0.0 {
                        let EQG = -1e-24f64 / EQE;
                        EQG
                    } else {
                        let EQH = AGZ * (EQE + (((EQE * EQE) + 4e-24f64).sqrt()));
                        EQH
                    };
                    let EQJ = (AGZ * (B + (EQI / DUF))).powf(DUM);
                    EQL = EQJ;
                }
                let EQM = (EGU * ((ANX * (APG + (EQK * EPW))).powf(DUP))) + (EHF / EQL);
                EQO = EQM;
            } else {
                let EQN = (DUR * ((ANX * (APG + (EQK * EOM))).powf(DUP))) + (DUV / EPH);
                EQO = EQN;
            }
            let EQP = B + EQO;
            let EQQ = EQP - B;
            let EQR = (AGZ * ((EQP + B) + (((EQQ * EQQ) + DVE).sqrt()))) / EPT;
            let EQT = ((EQS * RB) / AOB) + S;
            let EQU = if EQT < AOF { 1.0 } else { 0.0 };
            let ERK;
            let GQR;
            if EQU != 0.0 {
                let EQW = (EQT.cosh()) - B;
                let EQY = ((AGZ * EQV) / EQW) + EQX;
                let HTB = ((GPB * AGZ) / EQW) + GPC;
                ERK = EQY;
                GQR = HTB;
            } else {
                let EQZ = rspice_limited_exp((-EQT));
                let ERA = (EQV * EQZ) + EQX;
                let HTA = (GPB * EQZ) + GPC;
                ERK = ERA;
                GQR = HTA;
            }
            let ERB = if KF > A { 1.0 } else { 0.0 };
            let ERR;
            let GQS;
            if ERB != 0.0 {
                let ERC = (KF * EOA) / DWG;
                let HTD = ((HSD * KF) - (HOX * ERC)) / DWG;
                let ERD = B + ERC;
                ERR = ERD;
                GQS = HTD;
            } else {
                let ERE = (KF * EOA) / DWG;
                let ERF = B - ERE;
                let ERG = B / ERF;
                let HTC = ((((((HSD * KF) - (HOX * ERE)) / DWG) * GRP) * ERG) * GRP) / ERF;
                ERR = ERG;
                GQS = HTC;
            }
            let ERH = DFY - DXV;
            let HTE = HPO - HPR;
            let ERM;
            let GQT;
            if AZS != 0.0 {
                let ERI = EOA + AZN;
                let HTG = HSD + Lanes([0.0, GSK, 0.0, 0.0, 0.0]);
                ERM = ERI;
                GQT = HTG;
            } else {
                let ERJ = EOA + (R * DLG);
                let HTF = HSD + Lanes([0.0, (GHO * R), 0.0, 0.0, 0.0]);
                ERM = ERJ;
                GQT = HTF;
            }
            let ERL = if ERK > A { 1.0 } else { 0.0 };
            let ESM;
            let GQU;
            if ERL != 0.0 {
                let ERN = DXO + ERM;
                let ERO = ERM / ERN;
                let ERP = ERM / ERK;
                let HTH = GQR * ERP;
                let ERQ = ERP * ERO;
                let ERS = ERQ * ERR;
                let ERT = ERH / ERS;
                let HTI = (HTE - (((((((GQT - Lanes([0.0, HTH[0], HTH[1], HTH[2], 0.0])) / ERK) * ERO) + (((GQT - ((GQD + GQT) * ERO)) / ERN) * ERP)) * ERR) + (GQS * ERQ)) * ERT)) / ERS;
                let ERU = B + ERT;
                ESM = ERU;
                GQU = HTI;
            } else {
                ESM = B;
                GQU = HMT;
            }
            let ERW = if ERV > A { 1.0 } else { 0.0 };
            let ESN;
            let GQV;
            if ERW != 0.0 {
                let ERX = if JQ < A { 1.0 } else { 0.0 };
                let ESC;
                let GQW;
                if ERX != 0.0 {
                    let ERY = B / ERV;
                    let HTK = ((GPD * ERY) * GRP) / ERV;
                    let ERZ = ERY - (JQ * EOA);
                    let ESA = B / ERZ;
                    let HTL = (((Lanes([0.0, HTK[0], HTK[1], HTK[2], 0.0]) - (HSD * JQ)) * ESA) * GRP) / ERZ;
                    ESC = ESA;
                    GQW = HTL;
                } else {
                    let ESB = ERV + (JQ * EOA);
                    let HTJ = Lanes([0.0, GPD[0], GPD[1], GPD[2], 0.0]) + (HSD * JQ);
                    ESC = ESB;
                    GQW = HTJ;
                }
                let ESD = ERH / ESC;
                let ESE = DXO + DWG;
                let ESF = ESD / ESE;
                let HTM = (((HTE - (GQW * ESD)) / ESC) - ((GQD + HOX) * ESF)) / ESE;
                let ESG = B + ESF;
                let ESH = if ESG > BZ { 1.0 } else { 0.0 };
                let ESK;
                let GQX;
                if ESH != 0.0 {
                    let ESI = ESG.ln();
                    let HTN = HTM * (GHB / ESG);
                    ESK = ESI;
                    GQX = HTN;
                } else {
                    ESK = ESJ;
                    GQX = HMT;
                }
                let HTO = (GQW * ESK) + (GQX * ESC);
                let ESL = B + (ESC * ESK);
                ESN = ESL;
                GQV = HTO;
            } else {
                ESN = B;
                GQV = HMT;
            }
            let ESO = ESM * ESN;
            let HTP = (GQU * ESN) + (GQV * ESM);
            let HTQ = GPE * R;
            let ESQ = (R * ESP) / EPV;
            let ESR = ESQ * RB;
            let ESS = EOB / ESR;
            let HTR = (HSE - ((((Lanes([0.0, HTQ[0], HTQ[1], HTQ[2], 0.0]) - (HSZ * ESQ)) / EPV) * RB) * ESS)) / ESR;
            let EST = if ESS > BZ { 1.0 } else { 0.0 };
            let ESX;
            let GQY;
            if EST != 0.0 {
                let ESU = ESS.ln();
                let HTS = HTR * (GHB / ESS);
                ESX = ESU;
                GQY = HTS;
            } else {
                ESX = ESV;
                GQY = HMT;
            }
            let ESY = ESW * ESX;
            let ESZ = rspice_limited_exp(ESY);
            let HTT = (GQY * ESW) * (rspice_limited_exp_derivative(ESY));
            let ETA = B / ESW;
            let ETB = if IX > BZ { 1.0 } else { 0.0 };
            let ETE = if ETB != 0.0 {
                let ETC = IX.ln();
                ETC
            } else {
                ETD
            };
            let ETF = B + (rspice_limited_exp((ETA * ETE)));
            let ETG = IX + ESZ;
            let ETH = if ETG > BZ { 1.0 } else { 0.0 };
            let ETK;
            let GQZ;
            if ETH != 0.0 {
                let ETI = ETG.ln();
                let HTU = HTT * (GHB / ETG);
                ETK = ETI;
                GQZ = HTU;
            } else {
                ETK = ETJ;
                GQZ = HMT;
            }
            let ETL = ETA * ETK;
            let ETN = AGZ * ETM;
            let ETO = ETN * EOA;
            let HTV = (GPF * AGZ) * EOA;
            let ETP = ETO * EOB;
            let ETQ = ((B + (rspice_limited_exp(ETL))) / ETF) + (ETP * EOB);
            let ETR = R * DLK;
            let HTW = GPG * R;
            let ETS = EOA + ETR;
            let HTX = Lanes([0.0, HTW[0], HTW[1], HTW[2], 0.0]);
            let ETT = COV / ETS;
            let ETU = COU + ETT;
            let ETV = ETU * EOB;
            let HTY = ((((Lanes([0.0, HHF, 0.0, 0.0, 0.0]) + ((Lanes([0.0, HHG, 0.0, 0.0, 0.0]) - ((HSD + HTX) * ETT)) / ETS)) * EOB) + (HSE * ETU)) * EOB) + (HSE * ETV);
            let ETW = ((ETV * EOB) + B) - ABQ;
            let HTZ = HTY * ETW;
            let ETX = ((ETW * ETW) + DGF).sqrt();
            let ETY = (B + (-1e0f64 + (AGZ * (ETW + ETX)))).sqrt();
            let ETZ = AGZ * (B + ETY);
            let EUA = ETQ * ETZ;
            let HUA = (((((GQZ * ETA) * (rspice_limited_exp_derivative(ETL))) / ETF) + (((((Lanes([0.0, HTV[0], HTV[1], HTV[2], 0.0]) + (HSD * ETN)) * EOB) + (HSE * ETO)) * EOB) + (HSE * ETP))) * ETZ) + (((((HTY + ((HTZ + HTZ) * (GHB / (GRJ * ETX)))) * AGZ) * (GHB / (GRJ * ETY))) * AGZ) * ETQ);
            let EUB = EUA - B;
            let HUB = HUA * EUB;
            let EUD = AQO * EUC;
            let EUE = ((EUB * EUB) + (EUD * EUC)).sqrt();
            let EUF = AGZ * ((EUA + B) + EUE);
            let HUC = (HUA + ((HUB + HUB) * (GHB / (GRJ * EUE)))) * AGZ;
            let EUG = CPE * EOB;
            let EUH = COC + (EUG * EOB);
            let EUI = if A >= EUH { A } else { EUH };
            let EUJ = (EUI * EOA) + ETR;
            let EUK = CNX / EUJ;
            let EUL = -EUK;
            let EUM = rspice_limited_exp(EUL);
            let HUD = (((Lanes([0.0, HGU, 0.0, 0.0, 0.0]) - ((((((Lanes([0.0, HGX, 0.0, 0.0, 0.0]) + (((Lanes([0.0, (HHK * EOB), 0.0, 0.0, 0.0]) + (HSE * CPE)) * EOB) + (HSE * EUG))) * (GHB - (if A >= EUH { 1.0 } else { 0.0 }))) * EOA) + (HSD * EUI)) + HTX) * EUK)) / EUJ) * GRP) * (rspice_limited_exp_derivative(EUL));
            let EUN = if AR == R { 1.0 } else { 0.0 };
            let EXP;
            let GRA;
            if EUN != 0.0 {
                let HUE = HIN * COT;
                let EUO = CPD + (COT * DGA);
                let HUF = Lanes([HHJ, 0.0, 0.0]) + (Lanes([(HHE * DGA), 0.0, 0.0]) + Lanes([0.0, HUE[0], HUE[1]]));
                let EUP = if EUO < -1e-2f64 { 1.0 } else { 0.0 };
                let EUT;
                let GRB;
                if EUP != 0.0 {
                    let EUQ = -1e-12f64 / EUO;
                    let HUI = ((HUF * EUQ) * GRP) / EUO;
                    EUT = EUQ;
                    GRB = HUI;
                } else {
                    let HUG = HUF * EUO;
                    let EUR = ((EUO * EUO) + 4e-12f64).sqrt();
                    let EUS = AGZ * (EUO + EUR);
                    let HUH = (HUF + ((HUG + HUG) * (GHB / (GRJ * EUR)))) * AGZ;
                    EUT = EUS;
                    GRB = HUH;
                }
                let EUU = CPF * EOB;
                let EUV = COH + (EUU * EOB);
                let EUW = if A >= EUV { A } else { EUV };
                let EUX = (EUW * EOA) + ETR;
                let EUY = EUT / EUX;
                let EUZ = (DGC - DUT).sqrt();
                let EVA = EUZ - (DGC.sqrt());
                let EVB = -EUY;
                let EVC = EVB * EVA;
                let HUJ = ((GOO * GRP) * (GHB / (GRJ * EUZ))) * EVB;
                let EVD = rspice_limited_exp(EVC);
                let HUK = (((((Lanes([0.0, GRB[0], GRB[1], GRB[2], 0.0]) - ((((((Lanes([0.0, HHA, 0.0, 0.0, 0.0]) + (((Lanes([0.0, (HHL * EOB), 0.0, 0.0, 0.0]) + (HSE * CPF)) * EOB) + (HSE * EUU))) * (GHB - (if A >= EUV { 1.0 } else { 0.0 }))) * EOA) + (HSD * EUW)) + HTX) * EUY)) / EUX) * GRP) * EVA) + Lanes([HUJ[0], 0.0, HUJ[1], HUJ[2], 0.0])) * (rspice_limited_exp_derivative(EVC));
                EXP = EVD;
                GRA = HUK;
            } else {
                EXP = B;
                GRA = HMT;
            }
            let EVG = if PT != 0.0 {
                let EVE = ((R * EHV) * EQR) / ECY;
                EVE
            } else {
                let EVF = ((R * EHV) * EQR) / DNE;
                EVF
            };
            let EVH = EVG * XF;
            let EVO = if PT != 0.0 {
                let EVL = (EVI / EVH).powf(EVK);
                EVL
            } else {
                let EVM = (EOB / EVH).powf(EVK);
                EVM
            };
            let EVN = B / EVK;
            let EVP = IW - ASN;
            let EVQ = ((B + ((IZ + EVO).powf(EVN))) / (B + (IZ.powf(EVN)))) * (AGZ * ((IW + ASN) + (((EVP * EVP) + 2.5e-7f64).sqrt())));
            let EVR = if JR != A { 1.0 } else { 0.0 };
            let EYE;
            if EVR != 0.0 {
                let EVS = B + ((ERH / JR) / (DXO + EVH));
                let EVT = if EVS > BZ { 1.0 } else { 0.0 };
                let EVW = if EVT != 0.0 {
                    let EVU = EVS.ln();
                    EVU
                } else {
                    EVV
                };
                let EVX = B + (JR * EVW);
                EYE = EVX;
            } else {
                EYE = B;
            }
            let EVY = DMC + EOA;
            let HUL = Lanes([0.0, HKN[0], HKN[1], HKN[2], 0.0]);
            let EVZ = DMC / EVY;
            let EWA = R - EVZ;
            let HUM = GPG * EWA;
            let EWB = EOA + (EWA * DLK);
            let HUN = HSD + (((((HUL - ((HUL + HSD) * EVZ)) / EVY) * GRP) * DLK) + Lanes([0.0, HUM[0], HUM[1], HUM[2], 0.0]));
            let EWC = EWB * EOB;
            let HUO = (HUN * EOB) + (HSE * EWB);
            let EXR;
            let FVF;
            let GBV;
            let GCL;
            let GCT;
            let GRC;
            if AGE != 0.0 {
                let EWD = B + (DVI * EOA);
                let EWE = B / EWD;
                let HUT = (((HSD * DVI) * EWE) * GRP) / EWD;
                let HUU = HUT * EWE;
                let EWF = ((EWE * EWE) + YC).sqrt();
                let EWG = DVM + (DVN * (AGZ * (EWE + EWF)));
                let EWH = (DVP * EWG) * ANZ;
                let EWI = BK * DNF;
                let HUV = (HKT * BK) * EWB;
                let EWJ = EPU * EUF;
                let EWK = (EWI * EWB) / EWJ;
                let HUW = ((((Lanes([0.0, HUV[0], HUV[1], HUV[2], 0.0]) + (HUN * EWI)) - (((HSY * EUF) + (HUC * EPU)) * EWK)) / EWJ) * EWH) + (((Lanes([0.0, (GIU * EWG), 0.0, 0.0, 0.0]) + ((((HUT + ((HUU + HUU) * (GHB / (GRJ * EWF)))) * AGZ) * DVN) * DVP)) * ANZ) * EWK);
                let EWL = B + (EWK * EWH);
                EXR = EWL;
                FVF = EWH;
                GBV = ETW;
                GCL = DVY;
                GCT = DVV;
                GRC = HUW;
            } else {
                let EXS;
                let FVG;
                let GBW;
                let GCM;
                let GCU;
                let GRD;
                if WJ != 0.0 {
                    let EWN = (CWN * (DFA - EWM)) - CYN;
                    let EWO = B / (B + (DVI * (AGZ * (EWN + (((EWN * EWN) + ASN).sqrt())))));
                    let EWR = node_potentials[2] - EWM;
                    let EWS = DVP * (DVV + ((parameters[911] + ((EWP * (B + (EWQ * (((EWR * EWR) + S).powf((AGZ * parameters[921])))))) * (AGZ * (EWO + (((EWO * EWO) + YC).sqrt()))))) * ANZ));
                    let EWU = (CWN * (DFA - EWT)) - CYN;
                    let EWW = B / (B + (EWV * (AGZ * (EWU + (((EWU * EWU) + ASN).sqrt())))));
                    let EWZ = node_potentials[0] - EWT;
                    let EXA = DVP * (DVY + ((parameters[914] + ((EWX * (B + (EWY * (((EWZ * EWZ) + S).powf((AGZ * parameters[922])))))) * (AGZ * (EWW + (((EWW * EWW) + YC).sqrt()))))) * ANZ));
                    EXS = B;
                    FVG = A;
                    GBW = EWU;
                    GCM = EXA;
                    GCU = EWS;
                    GRD = HMT;
                } else {
                    let EXB = if WI == R { 1.0 } else { 0.0 };
                    let EXT;
                    let FVH;
                    let GRE;
                    if EXB != 0.0 {
                        let EXC = B + (DVI * EOA);
                        let EXD = B / EXC;
                        let HUP = (((HSD * DVI) * EXD) * GRP) / EXC;
                        let HUQ = HUP * EXD;
                        let EXE = ((EXD * EXD) + YC).sqrt();
                        let EXF = (((DVM + (DVN * (AGZ * (EXD + EXE)))) * ANZ) + DVV) + DVY;
                        let EXG = DVP * EXF;
                        let EXH = BK * DNF;
                        let HUR = (HKT * BK) * EWB;
                        let EXI = EPU * EUF;
                        let EXJ = (EXH * EWB) / EXI;
                        let HUS = ((((Lanes([0.0, HUR[0], HUR[1], HUR[2], 0.0]) + (HUN * EXH)) - (((HSY * EUF) + (HUC * EPU)) * EXJ)) / EXI) * EXG) + ((Lanes([0.0, (GIU * EXF), 0.0, 0.0, 0.0]) + (((((HUP + ((HUQ + HUQ) * (GHB / (GRJ * EXE)))) * AGZ) * DVN) * ANZ) * DVP)) * EXJ);
                        let EXK = B + (EXJ * EXG);
                        EXT = EXK;
                        FVH = EXG;
                        GRE = HUS;
                    } else {
                        EXT = A;
                        FVH = A;
                        GRE = HMT;
                    }
                    EXS = EXT;
                    FVG = FVH;
                    GBW = ETW;
                    GCM = A;
                    GCU = A;
                    GRD = GRE;
                }
                EXR = EXS;
                FVF = FVG;
                GBV = GBW;
                GCL = GCM;
                GCT = GCU;
                GRC = GRD;
            }
            let EXL = BK * DNF;
            let EXM = EXL * EWC;
            let HUX = (HKT * BK) * EWC;
            let EXN = EXM * ESO;
            let EXO = EXN * EUM;
            let EXQ = EPU * EUF;
            let EXU = EXQ * EXR;
            let EXV = (EXO * EXP) / EXU;
            let EXX = EXV * EXW;
            let HUY = (((((((((Lanes([0.0, HUX[0], HUX[1], HUX[2], 0.0]) + (HUO * EXL)) * ESO) + (HTP * EXM)) * EUM) + (HUD * EXN)) * EXP) + (GRA * EXO)) - (((((HSY * EUF) + (HUC * EPU)) * EXR) + (GRC * EXQ)) * EXV)) / EXU) * EXW;
            let EYF;
            let EYN;
            if PT != 0.0 {
                let EYA = ((R * EXY) + DLK) / EVQ;
                let EYB = EXY + ((EVI * EVI) / (AUH * EYA));
                EYF = EYB;
                EYN = EYA;
            } else {
                let EYC = ((R * EOA) + DLK) / EVQ;
                let EYD = EOA + ((EOB * EOB) / (AUH * EYC));
                EYF = EYD;
                EYN = EYC;
            }
            let EYG = ((B / EYE) * EYF) + ((EYE - B) * ENY);
            let EYH = if RF == R { 1.0 } else { 0.0 };
            if EYH != 0.0 {
            } else {
            }
            let EYJ = (((BK * HI) * XF) * EYI) * EYG;
            let GBU;
            if AS != 0.0 {
                if AX != 0.0 {
                } else {
                }
                let EYO = ((EYM - B) * AGZ) * (EOA + ((EOB * EOB) / (AUH * EYN)));
                GBU = EYO;
            } else {
                GBU = GBV;
            }
            let EYQ = if EYP < A { 1.0 } else { 0.0 };
            if EYQ != 0.0 {
            } else {
            }
            let GBS;
            if AGN != 0.0 {
                let GBT = if DFK != 0.0 {
                    let EYR = CWN * (DFL - DFD);
                    EYR
                } else {
                    let EYS = CWN * (DFO - DFD);
                    EYS
                };
                GBS = GBT;
            } else {
                GBS = GBU;
            }
            let EYT = if AGM == A { 1.0 } else { 0.0 };
            if EYT != 0.0 {
                if DFK != 0.0 {
                } else {
                }
            } else {
                let EYU = if AGM == B { 1.0 } else { 0.0 };
                if EYU != 0.0 {
                    if DFK != 0.0 {
                        let EYW = if EYV == B { 1.0 } else { 0.0 };
                        if EYW != 0.0 {
                        } else {
                        }
                    } else {
                        let EYX = if EYV == B { 1.0 } else { 0.0 };
                        if EYX != 0.0 {
                        } else {
                        }
                    }
                } else {
                    if DFK != 0.0 {
                    } else {
                    }
                }
            }
            let EYY = if parameters[65] == B { 1.0 } else { 0.0 };
            let GBR;
            if EYY != 0.0 {
                let EYZ = (S / ETR) * (AWP.sqrt());
                let EZA = EYZ / R;
                let EZC = (-(EZB - parameters[144])) / AWP;
                let EZD = DKS + (EYZ * ((DKS * AWP).sqrt()));
                let EZE = if (EZC * AWP) > EZD { 1.0 } else { 0.0 };
                let EZY;
                let FAA;
                if EZE != 0.0 {
                    let EZF = (((EZC - B) + (EZA * EZA)).sqrt()) - EZA;
                    let EZG = B + (EZF * EZF);
                    let EZH = (rspice_limited_exp((-EZG))) - B;
                    EZY = EZH;
                    FAA = EZG;
                } else {
                    let EZI = (EZC * AGZ) - (DQ * (B + (EYZ / 1.4142135623730951e0f64)));
                    let EZJ = EZI + (((EZI * EZI) + (AUH * EZC)).sqrt());
                    let EZK = if EZC < A { 1.0 } else { 0.0 };
                    let EZZ;
                    let FAB;
                    if EZK != 0.0 {
                        let EZL = (EZC - EZJ) / EYZ;
                        let EZM = EZL * EZL;
                        let EZN = (-EZJ) + EZM;
                        let EZO = (B - EZJ) + EZM;
                        let EZP = if EZO > BZ { 1.0 } else { 0.0 };
                        let EZS = if EZP != 0.0 {
                            let EZQ = EZO.ln();
                            EZQ
                        } else {
                            EZR
                        };
                        let EZT = -EZS;
                        EZZ = EZN;
                        FAB = EZT;
                    } else {
                        let EZU = rspice_limited_exp((-1.2e0f64 * EZJ));
                        let EZV = ((((EZC - B) + EZU) + (EZA * EZA)).sqrt()) - EZA;
                        let EZW = (B - EZU) + (EZV * EZV);
                        let EZX = (rspice_limited_exp((-EZW))) - B;
                        EZZ = EZX;
                        FAB = EZW;
                    }
                    EZY = EZZ;
                    FAA = FAB;
                }
                let FAC = (EZY + FAA).sqrt();
                let FAD = if FAA > ENA { 1.0 } else { 0.0 };
                let FAH;
                if FAD != 0.0 {
                    let FAE = FAA - (((-(EZC - FAA)) + (EYZ * FAC)) / (B - (((EYZ * AGZ) * EZY) / FAC)));
                    FAH = FAE;
                } else {
                    let FAF = if FAA < -1e-15f64 { 1.0 } else { 0.0 };
                    let FAI = if FAF != 0.0 {
                        let FAG = FAA - (((-(EZC - FAA)) - (EYZ * FAC)) / (B + (((EYZ * AGZ) * EZY) / FAC)));
                        FAG
                    } else {
                        A
                    };
                    FAH = FAI;
                }
                let FAK = (FAJ - DFB).abs();
                let FAL = (((R * parameters[454]) / DNE) * DVH) * parameters[1];
                let FAN = B / FAM;
                let FAO = parameters[491] * (((EYZ * (rspice_limited_exp(((-FAH) / R)))) * AWP) + (R * DLG));
                let FAP = ((FAL * FAO) / (FAL + FAO)) - ABQ;
                let FAQ = if FAP < -1e-1f64 { 1.0 } else { 0.0 };
                let FAT = if FAQ != 0.0 {
                    let FAR = -1.0000000000000002e-10f64 / FAP;
                    FAR
                } else {
                    let FAS = AGZ * (FAP + (((FAP * FAP) + 4.0000000000000007e-10f64).sqrt()));
                    FAS
                };
                let FAV = (-((FAU + (if (FAK / ((B + (((FAK / (FAT + ABQ)) + S).powf(FAM))).powf(FAN))) <= FAK { (FAK / ((B + (((FAK / (FAT + ABQ)) + S).powf(FAM))).powf(FAN))) } else { FAK })) - parameters[143])) / AWP;
                let FAW = if (FAV * AWP) > EZD { 1.0 } else { 0.0 };
                let FBM;
                if FAW != 0.0 {
                    let FAX = (((FAV - B) + (EZA * EZA)).sqrt()) - EZA;
                    let FAY = B + (FAX * FAX);
                    FBM = FAY;
                } else {
                    let FAZ = (FAV * AGZ) - (DQ * (B + (EYZ / 1.4142135623730951e0f64)));
                    let FBA = FAZ + (((FAZ * FAZ) + (AUH * FAV)).sqrt());
                    let FBB = if FAV < A { 1.0 } else { 0.0 };
                    let FBN;
                    if FBB != 0.0 {
                        let FBC = (FAV - FBA) / EYZ;
                        let FBD = (B - FBA) + (FBC * FBC);
                        let FBE = if FBD > BZ { 1.0 } else { 0.0 };
                        let FBH = if FBE != 0.0 {
                            let FBF = FBD.ln();
                            FBF
                        } else {
                            FBG
                        };
                        let FBI = -FBH;
                        FBN = FBI;
                    } else {
                        let FBJ = rspice_limited_exp((-1.2e0f64 * FBA));
                        let FBK = ((((FAV - B) + FBJ) + (EZA * EZA)).sqrt()) - EZA;
                        let FBL = (B - FBJ) + (FBK * FBK);
                        FBN = FBL;
                    }
                    FBM = FBN;
                }
                let FBO = if FBM > ENA { 1.0 } else { 0.0 };
                if FBO != 0.0 {
                } else {
                    let FBP = if FBM < -1e-15f64 { 1.0 } else { 0.0 };
                    if FBP != 0.0 {
                    } else {
                    }
                }
                if AX != 0.0 {
                } else {
                }
                GBR = FAV;
            } else {
                GBR = GBS;
            }
            let FBQ = if ABS == B { 1.0 } else { 0.0 };
            let GBQ;
            if FBQ != 0.0 {
                let FBR = if (if ((CQI + (CQL * RB)) / RB) <= A { 1.0 } else { 0.0 }) != 0.0 || (if CNV <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if FBR != 0.0 {
                } else {
                }
                GBQ = GBR;
            } else {
                let GBX;
                if ABT != 0.0 {
                    let FBS = if ((CQP + (CQT * RB)) / RB) <= A { 1.0 } else { 0.0 };
                    let GBY;
                    if FBS != 0.0 {
                        GBY = GBR;
                    } else {
                        let FBT = MG * DPK;
                        let FBV = if FBT < (-1e4f64 * FBU) { 1.0 } else { 0.0 };
                        let FBY = if FBV != 0.0 {
                            let FBW = ((-FBU) * FBU) / FBT;
                            FBW
                        } else {
                            let FBX = AGZ * (FBT + (((FBT * FBT) + ((Q * FBU) * FBU)).sqrt()));
                            FBX
                        };
                        let FBZ = DPK * ((B / (B + FBY)) + MH);
                        let FCB = if FBZ < (-1e4f64 * FCA) { 1.0 } else { 0.0 };
                        let FCE = if FCB != 0.0 {
                            let FCC = ((-FCA) * FCA) / FBZ;
                            FCC
                        } else {
                            let FCD = AGZ * (FBZ + (((FBZ * FBZ) + ((Q * FCA) * FCA)).sqrt()));
                            FCD
                        };
                        GBY = FCE;
                    }
                    GBX = GBY;
                } else {
                    GBX = GBR;
                }
                GBQ = GBX;
            }
            let FSO;
            let FSQ;
            let GBP;
            if ABC != 0.0 {
                let FCH = ((EOA - KK) / FCF) / AWP;
                let FCI = if FCH > AHU { 1.0 } else { 0.0 };
                let FCM;
                if FCI != 0.0 {
                    FCM = FCH;
                } else {
                    let FCJ = if FCH < -3.7e1f64 { 1.0 } else { 0.0 };
                    let FCN = if FCJ != 0.0 {
                        let FCK = FCH.exp();
                        FCK
                    } else {
                        let FCL = (B + (FCH.exp())).ln();
                        FCL
                    };
                    FCM = FCN;
                }
                let FCO = -parameters[1110];
                let FCP = ((CPI - (KI * EOA)) - FCO) - S;
                let FCQ = HH * RB;
                let FCS = (FCR * (((((FCQ * 3.75956e-7f64) * APQ) * DFJ) * ((FCF * AWP) * FCM)) * (rspice_limited_exp((((-9.82222e11f64 * APL) * (FCO + (AGZ * (FCP + (((FCP * FCP) - ((Q * FCO) * S)).sqrt()))))) * (B + (KJ * EOA))))))) * CQU;
                let FCT = (CZW - (AZG / R)) - DKS;
                let FCU = FCT - DFJ;
                let FCX = (FCU / FCV) / AWP;
                let FCY = if FCX > AHU { 1.0 } else { 0.0 };
                let FDC;
                if FCY != 0.0 {
                    FDC = FCX;
                } else {
                    let FCZ = if FCX < -3.7e1f64 { 1.0 } else { 0.0 };
                    let FDD = if FCZ != 0.0 {
                        let FDA = FCX.exp();
                        FDA
                    } else {
                        let FDB = (B + (FCX.exp())).ln();
                        FDB
                    };
                    FDC = FDD;
                }
                let FDE = (FCV * AWP) * FDC;
                let FDM;
                if AS != 0.0 {
                    FDM = EOL;
                } else {
                    let FDF = if FCT <= A { 1.0 } else { 0.0 };
                    let FDN = if FDF != 0.0 {
                        let FDG = FCU - ACJ;
                        let FDI = AGZ * (FDG + (((FDG * FDG) - (FDH * FCT)).sqrt()));
                        FDI
                    } else {
                        let FDJ = FCU - ACJ;
                        let FDK = AGZ * (FDJ + (((FDJ * FDJ) + (FDH * FCT)).sqrt()));
                        FDK
                    };
                    FDM = FDN;
                }
                let FDL = -parameters[1111];
                let FDO = ((CPL - (KO * FDM)) - FDL) - S;
                let FDP = FDL + (AGZ * (FDO + (((FDO * FDO) - ((Q * FDL) * S)).sqrt())));
                let FDQ = (FCR * (((((FCQ * APH) * APQ) * DFJ) * FDE) * (rspice_limited_exp((((-7.45669e11f64 * APL) * FDP) * (B + (KP * FDM))))))) * CQU;
                FSO = FCS;
                FSQ = FDQ;
                GBP = FDP;
            } else {
                FSO = A;
                FSQ = A;
                GBP = GBQ;
            }
            let FSC;
            let FSE;
            let FSG;
            let FSK;
            let GBO;
            if ABF != 0.0 {
                let FDR = -parameters[1112];
                let FDS = ((CPO - (KT * EOA)) - FDR) - S;
                let FDU = (-FDT) * APL;
                let FDV = ((((((parameters[26] * HH) * RB) * APR) * APQ) * (EOA * (rspice_limited_exp(((FDU * (FDR + (AGZ * (FDS + (((FDS * FDS) - ((Q * FDR) * S)).sqrt()))))) * (B + (KU * EOA))))))) * ((DFJ + (AGZ * DGA)) + (AGZ * (DFH + DFI)))) * CQU;
                let FDY = FDW * (((EOC + YC).sqrt()) - ASN);
                let FDZ = rspice_limited_exp((-FDY));
                let FEA = (FDY * FDY) + 2e-4f64;
                let FEB = (FDV * ((B - ((FDY + B) * FDZ)) + AEX)) / FEA;
                let FEC = (FDV * (((FDY + FDZ) - B) + AEX)) / FEA;
                let FED = DFC - CYN;
                let FEE = ((FED * FED) + AEX).sqrt();
                let FEF = if parameters[82] == B { 1.0 } else { 0.0 };
                let FEN;
                let FEQ;
                if FEF != 0.0 {
                    let FEG = CPR - (KY * FEE);
                    let FEH = if FEG < -1e-2f64 { 1.0 } else { 0.0 };
                    let FEK = if FEH != 0.0 {
                        let FEI = -1e-12f64 / FEG;
                        FEI
                    } else {
                        let FEJ = AGZ * (FEG + (((FEG * FEG) + 4e-12f64).sqrt()));
                        FEJ
                    };
                    let FEL = if KZ < YC { 1.0 } else { 0.0 };
                    let FEO = if FEL != 0.0 {
                        YC
                    } else {
                        KZ
                    };
                    FEN = FEO;
                    FEQ = FEK;
                } else {
                    let FEM = CPR - (KY * FEE);
                    FEN = KZ;
                    FEQ = FEM;
                }
                let FEP = FDU * APN;
                let FER = rspice_limited_exp(((FEP * FEQ) * (B + (FEN * FEE))));
                let FES = if EYP > A { 1.0 } else { 0.0 };
                let FSI;
                let FSM;
                if FES != 0.0 {
                    let FEU = (((CQV * FET) * DFC) * FEE) * FER;
                    FSI = FEU;
                    FSM = A;
                } else {
                    let FEV = (((CQV * FET) * DFC) * FEE) * FER;
                    FSI = A;
                    FSM = FEV;
                }
                let FEW = DFF - CYN;
                let FEX = ((FEW * FEW) + AEX).sqrt();
                let FFF;
                let FFI;
                if FEF != 0.0 {
                    let FEY = CPU - (LC * FEX);
                    let FEZ = if FEY < -1e-2f64 { 1.0 } else { 0.0 };
                    let FFC = if FEZ != 0.0 {
                        let FFA = -1e-12f64 / FEY;
                        FFA
                    } else {
                        let FFB = AGZ * (FEY + (((FEY * FEY) + 4e-12f64).sqrt()));
                        FFB
                    };
                    let FFD = if LD < YC { 1.0 } else { 0.0 };
                    let FFG = if FFD != 0.0 {
                        YC
                    } else {
                        LD
                    };
                    FFF = FFG;
                    FFI = FFC;
                } else {
                    let FFE = CPU - (LC * FEX);
                    FFF = LD;
                    FFI = FFE;
                }
                let FFH = B + (FFF * FEX);
                let FFJ = rspice_limited_exp(((FEP * FFI) * FFH));
                let FSH;
                let FSL;
                if FES != 0.0 {
                    let FFL = (((CQV * FFK) * DFF) * FEX) * FFJ;
                    FSH = FSI;
                    FSL = FFL;
                } else {
                    let FFM = (((CQV * FFK) * DFF) * FEX) * FFJ;
                    FSH = FFM;
                    FSL = FSM;
                }
                FSC = FEB;
                FSE = FEC;
                FSG = FSH;
                FSK = FSL;
                GBO = FFH;
            } else {
                FSC = A;
                FSE = A;
                FSG = A;
                FSK = A;
                GBO = GBP;
            }
            let FFN = if OJ != A { 1.0 } else { 0.0 };
            let GBD;
            if FFN != 0.0 {
                let FFO = if (if LG <= A { 1.0 } else { 0.0 }) != 0.0 || (if CQA <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GBN;
                if FFO != 0.0 {
                    GBN = GBO;
                } else {
                    let FFP = (((-DFF) - LJ) + CYN) / ANW;
                    let FFQ = if FFP < -1e2f64 { 1.0 } else { 0.0 };
                    let FFT = if FFQ != 0.0 {
                        let FFR = -1e-4f64 / FFP;
                        FFR
                    } else {
                        let FFS = AGZ * (FFP + (((FFP * FFP) + 4e-4f64).sqrt()));
                        FFS
                    };
                    let FFU = CQA / (FFT + ABQ);
                    if AS != 0.0 {
                        let FFV = ((-DFI) * DFI) * DFI;
                        let FFX = if (FFV / ((FFW + (FFV.abs())) + AVX)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if FFX != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    GBN = FFU;
                }
                let FFY = if OJ == DQ { 1.0 } else { 0.0 };
                let FFZ = if FFY != 0.0 && (if LK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GBL;
                if FFZ != 0.0 {
                    let GBM;
                    if AS != 0.0 {
                        let FGA = (B + (NS * AWN)) - S;
                        let FGB = if FGA < -1e1f64 { 1.0 } else { 0.0 };
                        let FGE = if FGB != 0.0 {
                            let FGC = -1e-6f64 / FGA;
                            FGC
                        } else {
                            let FGD = AGZ * (FGA + (((FGA * FGA) + 4e-6f64).sqrt()));
                            FGD
                        };
                        let FGF = ((LK * HH) * AZR) * (rspice_limited_exp(((((((LL * DFF) * DFF) - ((LM * FGE) * DFF)) - LN) + CYN) / AWP)));
                        let FGG = ((-DFI) * DFI) * DFI;
                        let FGH = if (FGG / ((FFW + (FGG.abs())) + AVX)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if FGH != 0.0 {
                        } else {
                        }
                        GBM = FGF;
                    } else {
                        let FGI = (B + (NS * AWN)) - S;
                        let FGJ = if FGI < -1e1f64 { 1.0 } else { 0.0 };
                        let FGM = if FGJ != 0.0 {
                            let FGK = -1e-6f64 / FGI;
                            FGK
                        } else {
                            let FGL = AGZ * (FGI + (((FGI * FGI) + 4e-6f64).sqrt()));
                            FGL
                        };
                        let FGN = ((LK * HH) * AZR) * (rspice_limited_exp(((((((LL * DFF) * DFF) - ((LM * FGM) * DFF)) - LN) + CYN) / AWP)));
                        GBM = FGN;
                    }
                    GBL = GBM;
                } else {
                    GBL = GBN;
                }
                let FGO = if (if AS != 0.0 && (if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 || FFY != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GBJ;
                if FGO != 0.0 {
                    let FGP = if CPV < -1e1f64 { 1.0 } else { 0.0 };
                    let FGU = if FGP != 0.0 {
                        let FGQ = -1e-6f64 / CPV;
                        FGQ
                    } else {
                        let FGR = AGZ * (CPV + (((CPV * CPV) + 4e-6f64).sqrt()));
                        FGR
                    };
                    let FGV = FGS * FGU;
                    let FGY = if (if FGW <= A { 1.0 } else { 0.0 }) != 0.0 || (if FGV <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GBK;
                    if FGY != 0.0 {
                        GBK = GBL;
                    } else {
                        let FHB = (((-DFF) - FGZ) + CYN) / ANW;
                        let FHC = if FHB < -1e2f64 { 1.0 } else { 0.0 };
                        let FHF = if FHC != 0.0 {
                            let FHD = -1e-4f64 / FHB;
                            FHD
                        } else {
                            let FHE = AGZ * (FHB + (((FHB * FHB) + 4e-4f64).sqrt()));
                            FHE
                        };
                        let FHG = FGV / (FHF + ABQ);
                        let FHH = ((-DFI) * DFI) * DFI;
                        let FHK = if (FHH / ((FHI + (FHH.abs())) + AVX)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if FHK != 0.0 {
                        } else {
                        }
                        GBK = FHG;
                    }
                    GBJ = GBK;
                } else {
                    GBJ = GBL;
                }
                let FHL = if EYP > A { 1.0 } else { 0.0 };
                if FHL != 0.0 {
                } else {
                }
                let FHM = if (if LO <= A { 1.0 } else { 0.0 }) != 0.0 || (if CQF <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GBI;
                if FHM != 0.0 {
                    GBI = GBJ;
                } else {
                    let FHN = (((-DFC) - LR) + CYN) / ANW;
                    let FHO = if FHN < -1e2f64 { 1.0 } else { 0.0 };
                    let FHR = if FHO != 0.0 {
                        let FHP = -1e-4f64 / FHN;
                        FHP
                    } else {
                        let FHQ = AGZ * (FHN + (((FHN * FHN) + 4e-4f64).sqrt()));
                        FHQ
                    };
                    let FHS = CQF / (FHR + ABQ);
                    if AS != 0.0 {
                        let FHT = ((-DFH) * DFH) * DFH;
                        let FHV = if (FHT / ((FHU + (FHT.abs())) + AVX)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if FHV != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    GBI = FHS;
                }
                let FHW = if FFY != 0.0 && (if LS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GBG;
                if FHW != 0.0 {
                    let GBH;
                    if AS != 0.0 {
                        let FHX = (B + (NS * AWN)) - S;
                        let FHY = if FHX < -1e1f64 { 1.0 } else { 0.0 };
                        let FIB = if FHY != 0.0 {
                            let FHZ = -1e-6f64 / FHX;
                            FHZ
                        } else {
                            let FIA = AGZ * (FHX + (((FHX * FHX) + 4e-6f64).sqrt()));
                            FIA
                        };
                        let FIC = ((LS * HH) * AZR) * (rspice_limited_exp(((((((LT * DFC) * DFC) - ((LU * FIB) * DFC)) - LV) + CYN) / AWP)));
                        let FID = ((-DFH) * DFH) * DFH;
                        let FIE = if (FID / ((FHU + (FID.abs())) + AVX)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if FIE != 0.0 {
                        } else {
                        }
                        GBH = FIC;
                    } else {
                        let FIF = (B + (NS * AWN)) - S;
                        let FIG = if FIF < -1e1f64 { 1.0 } else { 0.0 };
                        let FIJ = if FIG != 0.0 {
                            let FIH = -1e-6f64 / FIF;
                            FIH
                        } else {
                            let FII = AGZ * (FIF + (((FIF * FIF) + 4e-6f64).sqrt()));
                            FII
                        };
                        let FIK = ((LS * HH) * AZR) * (rspice_limited_exp(((((((LT * DFC) * DFC) - ((LU * FIJ) * DFC)) - LV) + CYN) / AWP)));
                        GBH = FIK;
                    }
                    GBG = GBH;
                } else {
                    GBG = GBI;
                }
                let GBE;
                if FGO != 0.0 {
                    let FIL = if CPV < -1e1f64 { 1.0 } else { 0.0 };
                    let FIQ = if FIL != 0.0 {
                        let FIM = -1e-6f64 / CPV;
                        FIM
                    } else {
                        let FIN = AGZ * (CPV + (((CPV * CPV) + 4e-6f64).sqrt()));
                        FIN
                    };
                    let FIR = FIO * FIQ;
                    let FIU = if (if FIS <= A { 1.0 } else { 0.0 }) != 0.0 || (if FIR <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let GBF;
                    if FIU != 0.0 {
                        GBF = GBG;
                    } else {
                        let FIX = (((-DFC) - FIV) + CYN) / ANW;
                        let FIY = if FIX < -1e2f64 { 1.0 } else { 0.0 };
                        let FJB = if FIY != 0.0 {
                            let FIZ = -1e-4f64 / FIX;
                            FIZ
                        } else {
                            let FJA = AGZ * (FIX + (((FIX * FIX) + 4e-4f64).sqrt()));
                            FJA
                        };
                        let FJC = FIR / (FJB + ABQ);
                        let FJD = ((-DFH) * DFH) * DFH;
                        let FJG = if (FJD / ((FJE + (FJD.abs())) + AVX)) < -1e-2f64 { 1.0 } else { 0.0 };
                        if FJG != 0.0 {
                        } else {
                        }
                        GBF = FJC;
                    }
                    GBE = GBF;
                } else {
                    GBE = GBG;
                }
                if FHL != 0.0 {
                } else {
                }
                GBD = GBE;
            } else {
                GBD = GBO;
            }
            let GAW;
            if AS != 0.0 {
                let FJI = if FJH > A { 1.0 } else { 0.0 };
                let GBA;
                if FJI != 0.0 {
                    let FJL = if DFH < FJJ { 1.0 } else { 0.0 };
                    let GBB;
                    if FJL != 0.0 {
                        let FJS = FJO + (FJQ * (DFH - FJJ));
                        GBB = FJS;
                    } else {
                        let FJV = if DFH <= FJT { 1.0 } else { 0.0 };
                        let GBC = if FJV != 0.0 {
                            let FJW = rspice_limited_exp((-((DAL + DFH) / FJM)));
                            FJW
                        } else {
                            GBD
                        };
                        GBB = GBC;
                    }
                    GBA = GBB;
                } else {
                    GBA = GBD;
                }
                let FJY = if FJX > A { 1.0 } else { 0.0 };
                if FJY != 0.0 {
                    let FKA = if (FJZ - DFH) < (FJZ * ABQ) { 1.0 } else { 0.0 };
                    if FKA != 0.0 {
                    } else {
                    }
                } else {
                }
                let FKC = if FKB > A { 1.0 } else { 0.0 };
                if FKC != 0.0 {
                    let FKE = if (FKD - DFH) < (FKD * ABQ) { 1.0 } else { 0.0 };
                    if FKE != 0.0 {
                    } else {
                    }
                } else {
                }
                let FKG = if FKF > A { 1.0 } else { 0.0 };
                if FKG != 0.0 {
                    let FKI = if (FKH - DFH) < (FKH * ABQ) { 1.0 } else { 0.0 };
                    if FKI != 0.0 {
                    } else {
                    }
                } else {
                }
                let FKK = if FKJ > A { 1.0 } else { 0.0 };
                let GAX;
                if FKK != 0.0 {
                    let FKN = if DFI < FKL { 1.0 } else { 0.0 };
                    let GAY;
                    if FKN != 0.0 {
                        let FKU = FKQ + (FKS * (DFI - FKL));
                        GAY = FKU;
                    } else {
                        let FKX = if DFI <= FKV { 1.0 } else { 0.0 };
                        let GAZ = if FKX != 0.0 {
                            let FKY = rspice_limited_exp((-((DBT + DFI) / FKO)));
                            FKY
                        } else {
                            GBA
                        };
                        GAY = GAZ;
                    }
                    GAX = GAY;
                } else {
                    GAX = GBA;
                }
                let FLA = if FKZ > A { 1.0 } else { 0.0 };
                if FLA != 0.0 {
                    let FLC = if (FLB - DFI) < (FLB * ABQ) { 1.0 } else { 0.0 };
                    if FLC != 0.0 {
                    } else {
                    }
                } else {
                }
                let FLE = if FLD > A { 1.0 } else { 0.0 };
                if FLE != 0.0 {
                    let FLG = if (FLF - DFI) < (FLF * ABQ) { 1.0 } else { 0.0 };
                    if FLG != 0.0 {
                    } else {
                    }
                } else {
                }
                let FLI = if FLH > A { 1.0 } else { 0.0 };
                if FLI != 0.0 {
                    let FLK = if (FLJ - DFI) < (FLJ * ABQ) { 1.0 } else { 0.0 };
                    if FLK != 0.0 {
                    } else {
                    }
                } else {
                }
                let FLM = if FLL > A { 1.0 } else { 0.0 };
                if FLM != 0.0 {
                    let FLN = DFH / DDM;
                    let FLO = if FLN < BBM { 1.0 } else { 0.0 };
                    if FLO != 0.0 {
                        let FLP = if DDK > A { 1.0 } else { 0.0 };
                        if FLP != 0.0 {
                            let FLS = if DFH > FLQ { 1.0 } else { 0.0 };
                            if FLS != 0.0 {
                                let FLT = B - FLN;
                                let FLU = if DDN != B { 1.0 } else { 0.0 };
                                if FLU != 0.0 {
                                    let FLV = if DDN == AGZ { 1.0 } else { 0.0 };
                                    if FLV != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FLW = if FLT > BZ { 1.0 } else { 0.0 };
                                    if FLW != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let FLX = B - (FLQ / DDM);
                                let FLY = if DDN != B { 1.0 } else { 0.0 };
                                if FLY != 0.0 {
                                    let FLZ = if DDN == AGZ { 1.0 } else { 0.0 };
                                    if FLZ != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FMA = if FLX > BZ { 1.0 } else { 0.0 };
                                    if FMA != 0.0 {
                                    } else {
                                    }
                                }
                                let FMD = B - ((DFH - FLQ) / FMB);
                                let FME = if DDP != B { 1.0 } else { 0.0 };
                                if FME != 0.0 {
                                    let FMF = if DDP == AGZ { 1.0 } else { 0.0 };
                                    if FMF != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FMG = if FMD > BZ { 1.0 } else { 0.0 };
                                    if FMG != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let FMH = B - FLN;
                            let FMI = if DDN != B { 1.0 } else { 0.0 };
                            if FMI != 0.0 {
                                let FMJ = if DDN == AGZ { 1.0 } else { 0.0 };
                                if FMJ != 0.0 {
                                } else {
                                }
                            } else {
                                let FMK = if FMH > BZ { 1.0 } else { 0.0 };
                                if FMK != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let FML = if DDN != B { 1.0 } else { 0.0 };
                        if FML != 0.0 {
                            let FMM = if DDN == AGZ { 1.0 } else { 0.0 };
                            if FMM != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let FMO = if FMN > A { 1.0 } else { 0.0 };
                if FMO != 0.0 {
                    let FMP = DFH / DDT;
                    let FMQ = if FMP < BBM { 1.0 } else { 0.0 };
                    if FMQ != 0.0 {
                        let FMR = if DDR > A { 1.0 } else { 0.0 };
                        if FMR != 0.0 {
                            let FMU = if DFH > FMS { 1.0 } else { 0.0 };
                            if FMU != 0.0 {
                                let FMV = B - FMP;
                                let FMW = if DDU != B { 1.0 } else { 0.0 };
                                if FMW != 0.0 {
                                    let FMX = if DDU == AGZ { 1.0 } else { 0.0 };
                                    if FMX != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FMY = if FMV > BZ { 1.0 } else { 0.0 };
                                    if FMY != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let FMZ = B - (FMS / DDT);
                                let FNA = if DDU != B { 1.0 } else { 0.0 };
                                if FNA != 0.0 {
                                    let FNB = if DDU == AGZ { 1.0 } else { 0.0 };
                                    if FNB != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FNC = if FMZ > BZ { 1.0 } else { 0.0 };
                                    if FNC != 0.0 {
                                    } else {
                                    }
                                }
                                let FNF = B - ((DFH - FMS) / FND);
                                let FNG = if DDW != B { 1.0 } else { 0.0 };
                                if FNG != 0.0 {
                                    let FNH = if DDW == AGZ { 1.0 } else { 0.0 };
                                    if FNH != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FNI = if FNF > BZ { 1.0 } else { 0.0 };
                                    if FNI != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let FNJ = B - FMP;
                            let FNK = if DDU != B { 1.0 } else { 0.0 };
                            if FNK != 0.0 {
                                let FNL = if DDU == AGZ { 1.0 } else { 0.0 };
                                if FNL != 0.0 {
                                } else {
                                }
                            } else {
                                let FNM = if FNJ > BZ { 1.0 } else { 0.0 };
                                if FNM != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let FNN = if DDU != B { 1.0 } else { 0.0 };
                        if FNN != 0.0 {
                            let FNO = if DDU == AGZ { 1.0 } else { 0.0 };
                            if FNO != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let FNQ = if FNP > A { 1.0 } else { 0.0 };
                if FNQ != 0.0 {
                    let FNR = DFH / DEA;
                    let FNS = if FNR < BBM { 1.0 } else { 0.0 };
                    if FNS != 0.0 {
                        let FNT = if DDY > A { 1.0 } else { 0.0 };
                        if FNT != 0.0 {
                            let FNW = if DFH > FNU { 1.0 } else { 0.0 };
                            if FNW != 0.0 {
                                let FNX = B - FNR;
                                let FNY = if DEB != B { 1.0 } else { 0.0 };
                                if FNY != 0.0 {
                                    let FNZ = if DEB == AGZ { 1.0 } else { 0.0 };
                                    if FNZ != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FOA = if FNX > BZ { 1.0 } else { 0.0 };
                                    if FOA != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let FOB = B - (FNU / DEA);
                                let FOC = if DEB != B { 1.0 } else { 0.0 };
                                if FOC != 0.0 {
                                    let FOD = if DEB == AGZ { 1.0 } else { 0.0 };
                                    if FOD != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FOE = if FOB > BZ { 1.0 } else { 0.0 };
                                    if FOE != 0.0 {
                                    } else {
                                    }
                                }
                                let FOH = B - ((DFH - FNU) / FOF);
                                let FOI = if DED != B { 1.0 } else { 0.0 };
                                if FOI != 0.0 {
                                    let FOJ = if DED == AGZ { 1.0 } else { 0.0 };
                                    if FOJ != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FOK = if FOH > BZ { 1.0 } else { 0.0 };
                                    if FOK != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let FOL = B - FNR;
                            let FOM = if DEB != B { 1.0 } else { 0.0 };
                            if FOM != 0.0 {
                                let FON = if DEB == AGZ { 1.0 } else { 0.0 };
                                if FON != 0.0 {
                                } else {
                                }
                            } else {
                                let FOO = if FOL > BZ { 1.0 } else { 0.0 };
                                if FOO != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let FOP = if DEB != B { 1.0 } else { 0.0 };
                        if FOP != 0.0 {
                            let FOQ = if DEB == AGZ { 1.0 } else { 0.0 };
                            if FOQ != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let FOS = if FOR > A { 1.0 } else { 0.0 };
                if FOS != 0.0 {
                    let FOT = DFI / DEH;
                    let FOU = if FOT < BBM { 1.0 } else { 0.0 };
                    if FOU != 0.0 {
                        let FOV = if DEF > A { 1.0 } else { 0.0 };
                        if FOV != 0.0 {
                            let FOY = if DFI > FOW { 1.0 } else { 0.0 };
                            if FOY != 0.0 {
                                let FOZ = B - FOT;
                                let FPA = if DEI != B { 1.0 } else { 0.0 };
                                if FPA != 0.0 {
                                    let FPB = if DEI == AGZ { 1.0 } else { 0.0 };
                                    if FPB != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FPC = if FOZ > BZ { 1.0 } else { 0.0 };
                                    if FPC != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let FPD = B - (FOW / DEH);
                                let FPE = if DEI != B { 1.0 } else { 0.0 };
                                if FPE != 0.0 {
                                    let FPF = if DEI == AGZ { 1.0 } else { 0.0 };
                                    if FPF != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FPG = if FPD > BZ { 1.0 } else { 0.0 };
                                    if FPG != 0.0 {
                                    } else {
                                    }
                                }
                                let FPJ = B - ((DFI - FOW) / FPH);
                                let FPK = if DEK != B { 1.0 } else { 0.0 };
                                if FPK != 0.0 {
                                    let FPL = if DEK == AGZ { 1.0 } else { 0.0 };
                                    if FPL != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FPM = if FPJ > BZ { 1.0 } else { 0.0 };
                                    if FPM != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let FPN = B - FOT;
                            let FPO = if DEI != B { 1.0 } else { 0.0 };
                            if FPO != 0.0 {
                                let FPP = if DEI == AGZ { 1.0 } else { 0.0 };
                                if FPP != 0.0 {
                                } else {
                                }
                            } else {
                                let FPQ = if FPN > BZ { 1.0 } else { 0.0 };
                                if FPQ != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let FPR = if DEI != B { 1.0 } else { 0.0 };
                        if FPR != 0.0 {
                            let FPS = if DEI == AGZ { 1.0 } else { 0.0 };
                            if FPS != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let FPU = if FPT > A { 1.0 } else { 0.0 };
                if FPU != 0.0 {
                    let FPV = DFI / DEO;
                    let FPW = if FPV < BBM { 1.0 } else { 0.0 };
                    if FPW != 0.0 {
                        let FPX = if DEM > A { 1.0 } else { 0.0 };
                        if FPX != 0.0 {
                            let FQA = if DFI > FPY { 1.0 } else { 0.0 };
                            if FQA != 0.0 {
                                let FQB = B - FPV;
                                let FQC = if DEP != B { 1.0 } else { 0.0 };
                                if FQC != 0.0 {
                                    let FQD = if DEP == AGZ { 1.0 } else { 0.0 };
                                    if FQD != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FQE = if FQB > BZ { 1.0 } else { 0.0 };
                                    if FQE != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let FQF = B - (FPY / DEO);
                                let FQG = if DEP != B { 1.0 } else { 0.0 };
                                if FQG != 0.0 {
                                    let FQH = if DEP == AGZ { 1.0 } else { 0.0 };
                                    if FQH != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FQI = if FQF > BZ { 1.0 } else { 0.0 };
                                    if FQI != 0.0 {
                                    } else {
                                    }
                                }
                                let FQL = B - ((DFI - FPY) / FQJ);
                                let FQM = if DER != B { 1.0 } else { 0.0 };
                                if FQM != 0.0 {
                                    let FQN = if DER == AGZ { 1.0 } else { 0.0 };
                                    if FQN != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FQO = if FQL > BZ { 1.0 } else { 0.0 };
                                    if FQO != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let FQP = B - FPV;
                            let FQQ = if DEP != B { 1.0 } else { 0.0 };
                            if FQQ != 0.0 {
                                let FQR = if DEP == AGZ { 1.0 } else { 0.0 };
                                if FQR != 0.0 {
                                } else {
                                }
                            } else {
                                let FQS = if FQP > BZ { 1.0 } else { 0.0 };
                                if FQS != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let FQT = if DEP != B { 1.0 } else { 0.0 };
                        if FQT != 0.0 {
                            let FQU = if DEP == AGZ { 1.0 } else { 0.0 };
                            if FQU != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let FQW = if FQV > A { 1.0 } else { 0.0 };
                if FQW != 0.0 {
                    let FQX = DFI / DEV;
                    let FQY = if FQX < BBM { 1.0 } else { 0.0 };
                    if FQY != 0.0 {
                        let FQZ = if DET > A { 1.0 } else { 0.0 };
                        if FQZ != 0.0 {
                            let FRC = if DFI > FRA { 1.0 } else { 0.0 };
                            if FRC != 0.0 {
                                let FRD = B - FQX;
                                let FRE = if DEW != B { 1.0 } else { 0.0 };
                                if FRE != 0.0 {
                                    let FRF = if DEW == AGZ { 1.0 } else { 0.0 };
                                    if FRF != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FRG = if FRD > BZ { 1.0 } else { 0.0 };
                                    if FRG != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let FRH = B - (FRA / DEV);
                                let FRI = if DEW != B { 1.0 } else { 0.0 };
                                if FRI != 0.0 {
                                    let FRJ = if DEW == AGZ { 1.0 } else { 0.0 };
                                    if FRJ != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FRK = if FRH > BZ { 1.0 } else { 0.0 };
                                    if FRK != 0.0 {
                                    } else {
                                    }
                                }
                                let FRN = B - ((DFI - FRA) / FRL);
                                let FRO = if DEY != B { 1.0 } else { 0.0 };
                                if FRO != 0.0 {
                                    let FRP = if DEY == AGZ { 1.0 } else { 0.0 };
                                    if FRP != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let FRQ = if FRN > BZ { 1.0 } else { 0.0 };
                                    if FRQ != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let FRR = B - FQX;
                            let FRS = if DEW != B { 1.0 } else { 0.0 };
                            if FRS != 0.0 {
                                let FRT = if DEW == AGZ { 1.0 } else { 0.0 };
                                if FRT != 0.0 {
                                } else {
                                }
                            } else {
                                let FRU = if FRR > BZ { 1.0 } else { 0.0 };
                                if FRU != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let FRV = if DEW != B { 1.0 } else { 0.0 };
                        if FRV != 0.0 {
                            let FRW = if DEW == AGZ { 1.0 } else { 0.0 };
                            if FRW != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                GAW = GAX;
            } else {
                GAW = GBD;
            }
            let GAV = if AS != 0.0 {
                let FRX = CWN * (DFL - DFG);
                FRX
            } else {
                GAW
            };
            let FSA = if FRY != A { 1.0 } else { 0.0 };
            let FSB = if RG != 0.0 && FSA != 0.0 { 1.0 } else { 0.0 };
            if FSB != 0.0 {
            } else {
            }
            if EYH != 0.0 {
            } else {
            }
            let FSD = BK * FSC;
            let FSF = BK * FSE;
            let FSJ = BK * FSG;
            let FSN = BK * FSK;
            let FSP = BK * FSO;
            let FSR = BK * FSQ;
            let FSS = if AR == A { 1.0 } else { 0.0 };
            let FXP;
            let FXQ;
            if FSS != 0.0 {
                let FST = FSP + FSR;
                let FSU = FST * DGK;
                let FSV = FST * DGL;
                FXP = FSU;
                FXQ = FSV;
            } else {
                FXP = A;
                FXQ = A;
            }
            let HUZ = (HUY * CWN)[4];
            let FSW = DWE / EPV;
            let FSZ = if (if (if YG > A { 1.0 } else { 0.0 }) != 0.0 || (if FSX > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if FSY > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GAS;
            let GDD;
            if FSZ != 0.0 {
                let FTA = RB - (R * parameters[1687]);
                let FTB = if FTA <= A { 1.0 } else { 0.0 };
                let FTF = if FTB != 0.0 {
                    RB
                } else {
                    FTA
                };
                let FTD = if FTC == B { 1.0 } else { 0.0 };
                let FTE = if FTD != 0.0 || (if FTC == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GAT;
                let GDE;
                if FTE != 0.0 {
                    let FTG = FTF * FTF;
                    let FTI = if FTH > A { 1.0 } else { 0.0 };
                    let FUJ;
                    if FTI != 0.0 {
                        let FTJ = ((ERH / AOA) + FTH) / FSW;
                        let FTK = if FTJ > BZ { 1.0 } else { 0.0 };
                        let FTN = if FTK != 0.0 {
                            let FTL = FTJ.ln();
                            FTL
                        } else {
                            FTM
                        };
                        let FTO = AOA * FTN;
                        FUJ = FTO;
                    } else {
                        FUJ = A;
                    }
                    let FUH = if FTD != 0.0 {
                        let FTS = (FTR / (B + ((EOM / FTP).powf(FTQ)))) / YG;
                        let FTT = FTS - B;
                        let FTV = YG * (AGZ * ((FTS + B) + (((FTT * FTT) + ((AQO * FTU) * FTU)).sqrt())));
                        FTV
                    } else {
                        YG
                    };
                    let FTW = ((4.112842231783458e-57f64 * AWP) * (EXX.abs())) * EPV;
                    let FTY = (FTX * EYI) * FTG;
                    let FTZ = (EYI * DUH) / HD;
                    let FUA = (EYI * ENY) / HD;
                    let FUB = (AWP / HD) * (EYI + DLD);
                    let FUC = FUA + FUB;
                    let FUD = (FTZ + FUB) / FUC;
                    let FUE = if FUD > BZ { 1.0 } else { 0.0 };
                    let FUI = if FUE != 0.0 {
                        let FUF = FUD.ln();
                        FUF
                    } else {
                        FUG
                    };
                    let FUK = ((FTW / FTY) * (((FUH * FUI) + (FSX * (FTZ - FUA))) + ((AGZ * FSY) * ((FTZ * FTZ) - (FUA * FUA))))) + ((((((BAL * EXX) * EXX) / (((FTX * FTG) * HH) * BK)) * FUJ) * ((FUH + (FSX * FUA)) + ((FSY * FUA) * FUA))) / (FUC * FUC));
                    let FUL = ((((FUH * HD) * AWP) / (((((HH * BK) * FTF) * FTX) * FUB) * FUB)) * EXX) * EXX;
                    let FUM = FUL + FUK;
                    let FUN = if FUM > A { 1.0 } else { 0.0 };
                    let GDF = if FUN != 0.0 {
                        let FUO = (FUK * FUL) / FUM;
                        FUO
                    } else {
                        A
                    };
                    GAT = FTY;
                    GDE = GDF;
                } else {
                    let FUP = if FTC == R { 1.0 } else { 0.0 };
                    let GAU;
                    let GDG;
                    if FUP != 0.0 {
                        let FUQ = (FTR / (B + ((EOM / FTP).powf(FTQ)))) / YG;
                        let FUR = FUQ - B;
                        let FUS = B + (parameters[1685] * EOB);
                        let FUT = ((YG * (AGZ * ((FUQ + B) + (((FUR * FUR) + ((AQO * FTU) * FTU)).sqrt())))) + (FSX * EOA)) + ((FSY * EOA) * EOA);
                        let FUU = if (if (B + ((AZN / DWG) * EOB)) > A { 1.0 } else { 0.0 }) != 0.0 && (if FUS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let GDH;
                        if FUU != 0.0 {
                            let FUV = FUS.powf((-parameters[1686]));
                            let FUW = (DUH + AGZ) / (ENY + AGZ);
                            let FUX = if FUW > BZ { 1.0 } else { 0.0 };
                            let FVA = if FUX != 0.0 {
                                let FUY = FUW.ln();
                                FUY
                            } else {
                                FUZ
                            };
                            let FVB = (((((((FVA * (ENZ + B)) / (R * EOB)) * (4.112739300563051e-57f64 * AWP)) * HUZ) * HUZ) * FUT) * FUV) / ((((((1e14f64 * HG) * HG) * HH) * HH) * FTF) * BK);
                            GDH = FVB;
                        } else {
                            GDH = A;
                        }
                        GAU = FUS;
                        GDG = GDH;
                    } else {
                        GAU = GAV;
                        GDG = A;
                    }
                    GAT = GAU;
                    GDE = GDG;
                }
                GAS = GAT;
                GDD = GDE;
            } else {
                GAS = GAV;
                GDD = A;
            }
            let FVD = if FVC == A { 1.0 } else { 0.0 };
            let GAR;
            let GEA;
            let GEC;
            if FVD != 0.0 {
                let FVE = EPV * EYJ;
                let FVI = ((Q * AWP) * HD) * ((FVE / ((FVE * FVF) + (RB * RB))) * parameters[1707]);
                GAR = GAS;
                GEA = FVI;
                GEC = A;
            } else {
                let FVJ = if FVC == B { 1.0 } else { 0.0 };
                let GBZ;
                let GEB;
                let GED;
                if FVJ != 0.0 {
                    let FVK = EOA / DWG;
                    let FVL = FVK * FVK;
                    let FVM = parameters[1708] * (B + ((FVL * parameters[1709]) * RB));
                    let FVN = parameters[1710] * (B + ((FVL * parameters[1711]) * RB));
                    let FVO = parameters[1714] * (B + ((FVL * parameters[1715]) * RB));
                    let FVP = (DQ * FVM) * FVM;
                    let FVQ = (7.5e0f64 * FVN) * FVN;
                    let FVR = 2.5298e0f64 * (parameters[1712] * (B + ((FVL * parameters[1713]) * RB)));
                    let FVS = DXV / DXO;
                    let FVT = (ENY / DUH) * (B - FVS);
                    let FVU = (EUF * EUF) * EUF;
                    let FVV = rspice_limited_exp((-(CNX / (((if A >= COC { A } else { COC }) * DUH) + ETR))));
                    let FWQ;
                    if EUN != 0.0 {
                        let FVW = if CPD < -1e-2f64 { 1.0 } else { 0.0 };
                        let FVZ = if FVW != 0.0 {
                            let FVX = -1e-12f64 / CPD;
                            FVX
                        } else {
                            let FVY = AGZ * (CPD + (((CPD * CPD) + 4e-12f64).sqrt()));
                            FVY
                        };
                        let FWA = rspice_limited_exp(((-(FVZ / (((if A >= COH { A } else { COH }) * DUH) + ETR))) * (((DGC - DUT).sqrt()) - (DGC.sqrt()))));
                        FWQ = FWA;
                    } else {
                        FWQ = B;
                    }
                    let FWB = ANX * (EYK + DUI);
                    let FWC = (AGZ * (B + (DUK.abs()))).powf(DUM);
                    let FWF = if AS != 0.0 {
                        let FWD = ((DUR + (DUS * DUT)) * ((FWB.abs()).powf(DUP))) + (DUV / FWC);
                        FWD
                    } else {
                        let FWE = (DUR * ((FWB.abs()).powf(DUP))) + (DUV / FWC);
                        FWE
                    };
                    let FWG = B + FWF;
                    let FWH = FWG - B;
                    let FWI = (AGZ * ((FWG + B) + (((FWH * FWH) + DVE).sqrt()))) / DVG;
                    let FWJ = B + EUD;
                    let FWK = DUH + ((R - (DMC / (DMC + DUH))) * DLK);
                    let FWR;
                    if AGE != 0.0 {
                        let FWL = B / (B + (DVI * DUH));
                        let FWM = B + (((EXL * FWK) / (FWI * FWJ)) * ((DVP * (DVM + (DVN * (AGZ * (FWL + (((FWL * FWL) + YC).sqrt())))))) * ANZ));
                        FWR = FWM;
                    } else {
                        let FWS;
                        if WJ != 0.0 {
                            FWS = B;
                        } else {
                            let FWN = if WI == R { 1.0 } else { 0.0 };
                            let FWT = if FWN != 0.0 {
                                let FWO = B / (B + (DVI * DUH));
                                let FWP = B + (((EXL * FWK) / (FWI * FWJ)) * (DVP * ((DVV + DVY) + ((DVM + (DVN * (AGZ * (FWO + (((FWO * FWO) + YC).sqrt()))))) * ANZ))));
                                FWP
                            } else {
                                A
                            };
                            FWS = FWT;
                        }
                        FWR = FWS;
                    }
                    let FWU = (((EXL * DUH) * FVV) * FWQ) / ((FWI * FWJ) * FWR);
                    let FWV = B + FVT;
                    let FWW = B - FVT;
                    let FWX = ((R * EVZ) / DUH) * DLK;
                    let FWY = FWV + FWX;
                    let FWZ = FWW * FWW;
                    let FXA = FWZ * FWW;
                    let FXB = FWY * FWY;
                    let FXC = FXB * FWY;
                    let FXD = FXC * FWY;
                    let FXE = AGZ * FWV;
                    let FXF = FWZ / (AUH * FWY);
                    let FXG = ESO / EUF;
                    let FXH = ESO / AUH;
                    let FXI = (FVR * ((FXH * EUF) * ((FWW / FWY) - (FXA / (DQ * FXC))))) / (((FXG * (FXE + FXF)) * ((FXH * FVU) * (((FWV / FXB) - ((((AUH * FWV) + FWX) * FWZ) / (1.5e1f64 * FXD))) + ((FXA * FWW) / (9e0f64 * (FXD * FWY)))))).sqrt());
                    let FXJ = if FXI > B { 1.0 } else { 0.0 };
                    let GEE;
                    if FXJ != 0.0 {
                        GEE = B;
                    } else {
                        let FXK = if FXI < A { 1.0 } else { 0.0 };
                        let GEF = if FXK != 0.0 {
                            A
                        } else {
                            FXI
                        };
                        GEE = GEF;
                    }
                    let FXL = (((Q * AWP) * HD) * (FXG * (((B + (((FVO * FVO) / (parameters[1716] + EOA)) * FVS)) * FXE) + (FVP * FXF)))) * FWU;
                    GBZ = FVQ;
                    GEB = FXL;
                    GED = GEE;
                } else {
                    GBZ = GAS;
                    GEB = A;
                    GED = A;
                }
                GAR = GBZ;
                GEA = GEB;
                GEC = GED;
            }
            let FXM = if EYP > A { 1.0 } else { 0.0 };
            if FXM != 0.0 {
            } else {
            }
            if EYH != 0.0 {
            } else {
            }
            if EYY != 0.0 {
            } else {
            }
            if FXM != 0.0 {
                if AS != 0.0 {
                    let FXN = if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if FXN != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                if AS != 0.0 {
                    let FXO = if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if FXO != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if FSS != 0.0 {
            } else {
            }
            if AS != 0.0 {
            } else {
            }
            if DFK != 0.0 {
                if EYY != 0.0 {
                } else {
                }
                let FXR = if AGM == B { 1.0 } else { 0.0 };
                if FXR != 0.0 {
                } else {
                }
            } else {
                if EYY != 0.0 {
                } else {
                }
                let FXS = if AGM == B { 1.0 } else { 0.0 };
                if FXS != 0.0 {
                } else {
                }
            }
            if AS != 0.0 {
            } else {
            }
            let GCP;
            let GCX;
            if WJ != 0.0 {
                let FXU = if FXT > A { 1.0 } else { 0.0 };
                let GAQ;
                let GCB;
                let GCQ;
                if FXU != 0.0 {
                    let FXW = (B + (FXV * AWN)) - S;
                    let FXX = if FXW < -1e1f64 { 1.0 } else { 0.0 };
                    let FYA = if FXX != 0.0 {
                        let FXY = -1e-6f64 / FXW;
                        FXY
                    } else {
                        let FXZ = AGZ * (FXW + (((FXW * FXW) + 4e-6f64).sqrt()));
                        FXZ
                    };
                    let FYS;
                    if CJD != 0.0 {
                        let FYC = -FYB;
                        let FYE = (((-FYD) * AWN) - FYC) - S;
                        let FYF = FYB + (FYC + (AGZ * (FYE + (((FYE * FYE) - ((Q * FYC) * S)).sqrt()))));
                        FYS = FYF;
                    } else {
                        let FYG = (B + ((-FYD) * AWN)) - S;
                        let FYH = if FYG < -1e1f64 { 1.0 } else { 0.0 };
                        let FYK = if FYH != 0.0 {
                            let FYI = -1e-6f64 / FYG;
                            FYI
                        } else {
                            let FYJ = AGZ * (FYG + (((FYG * FYG) + 4e-6f64).sqrt()));
                            FYJ
                        };
                        let FYL = FYB * FYK;
                        FYS = FYL;
                    }
                    let FYN = DUH - FYM;
                    let FYO = FYN - ASN;
                    let FYP = AGZ * ((FYN + ASN) + (((FYO * FYO) + 1e0f64).sqrt()));
                    let FYR = DAP * FYQ;
                    let FYU = FYS * (B + (FYT * ((FYR * FYP) / (FYR + FYP))));
                    let FYV = if FYU < -1e5f64 { 1.0 } else { 0.0 };
                    let FYY = if FYV != 0.0 {
                        let FYW = -1e2f64 / FYU;
                        FYW
                    } else {
                        let FYX = AGZ * (FYU + (((FYU * FYU) + 4e2f64).sqrt()));
                        FYX
                    };
                    let FYZ = ((BK * HH) * HD) * FYY;
                    let FZA = (EWT - FAJ).abs();
                    let FZC = if FZB == A { 1.0 } else { 0.0 };
                    let FZF = if FZC != 0.0 {
                        B
                    } else {
                        let FZD = FZA - parameters[1916];
                        let FZE = B + ((AGZ * (FZD + (((FZD * FZD) + 6.25e-2f64).sqrt()))) * FZB);
                        FZE
                    };
                    let FZG = (FYA * FXT) * ANZ;
                    let FZH = ((FYZ * parameters[1903]) * FZF) * FZG;
                    let FZJ = Q - FZI;
                    let FZK = FZA.powf(FZJ);
                    let FZL = B / FZI;
                    let FZM = FZG * ((B + (((((FZK / (FZK + (parameters[1914] * (FZH.powf(FZJ))))).powf(FZL)) * FZA) / FZH).powf(FZI))).powf(FZL));
                    GAQ = FYZ;
                    GCB = FYA;
                    GCQ = FZM;
                } else {
                    GAQ = GAR;
                    GCB = A;
                    GCQ = A;
                }
                let FZO = if FZN > A { 1.0 } else { 0.0 };
                let GCY;
                if FZO != 0.0 {
                    let FZP = if FXT == A { 1.0 } else { 0.0 };
                    let GAP;
                    let GCA;
                    if FZP != 0.0 {
                        let FZQ = (B + (FXV * AWN)) - S;
                        let FZR = if FZQ < -1e1f64 { 1.0 } else { 0.0 };
                        let FZU = if FZR != 0.0 {
                            let FZS = -1e-6f64 / FZQ;
                            FZS
                        } else {
                            let FZT = AGZ * (FZQ + (((FZQ * FZQ) + 4e-6f64).sqrt()));
                            FZT
                        };
                        let GAI;
                        if CJD != 0.0 {
                            let FZV = -FYB;
                            let FZW = (((-FYD) * AWN) - FZV) - S;
                            let FZX = FYB + (FZV + (AGZ * (FZW + (((FZW * FZW) - ((Q * FZV) * S)).sqrt()))));
                            GAI = FZX;
                        } else {
                            let FZY = (B + ((-FYD) * AWN)) - S;
                            let FZZ = if FZY < -1e1f64 { 1.0 } else { 0.0 };
                            let GAC = if FZZ != 0.0 {
                                let GAA = -1e-6f64 / FZY;
                                GAA
                            } else {
                                let GAB = AGZ * (FZY + (((FZY * FZY) + 4e-6f64).sqrt()));
                                GAB
                            };
                            let GAD = FYB * GAC;
                            GAI = GAD;
                        }
                        let GAE = DUH - FYM;
                        let GAF = GAE - ASN;
                        let GAG = AGZ * ((GAE + ASN) + (((GAF * GAF) + 1e0f64).sqrt()));
                        let GAH = DAP * FYQ;
                        let GAJ = GAI * (B + (FYT * ((GAH * GAG) / (GAH + GAG))));
                        let GAK = if GAJ < -1e5f64 { 1.0 } else { 0.0 };
                        let GAN = if GAK != 0.0 {
                            let GAL = -1e2f64 / GAJ;
                            GAL
                        } else {
                            let GAM = AGZ * (GAJ + (((GAJ * GAJ) + 4e2f64).sqrt()));
                            GAM
                        };
                        let GAO = ((BK * HH) * HD) * GAN;
                        GAP = GAO;
                        GCA = FZU;
                    } else {
                        GAP = GAQ;
                        GCA = GCB;
                    }
                    let GCC = (GCA * FZN) * ANZ;
                    let GCD = (GAP * parameters[1909]) * GCC;
                    let GCE = (DFB - EWM).abs();
                    let GCF = Q - FZI;
                    let GCG = GCE.powf(GCF);
                    let GCH = B / FZI;
                    let GCI = GCC * ((B + (((((GCG / (GCG + (parameters[1915] * (GCD.powf(GCF))))).powf(GCH)) * GCE) / GCD).powf(FZI))).powf(GCH));
                    GCY = GCI;
                } else {
                    GCY = A;
                }
                GCP = GCQ;
                GCX = GCY;
            } else {
                GCP = A;
                GCX = A;
            }
            let GCJ = if WI != R { 1.0 } else { 0.0 };
            let GCK = if GCJ != 0.0 && (if DVY > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GDM;
            let GDP;
            if GCK != 0.0 {
                let GCN = B / GCL;
                let GCO = if WJ != 0.0 && (if FXT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GDQ = if GCO != 0.0 {
                    let GCR = B / GCP;
                    GCR
                } else {
                    A
                };
                GDM = GCN;
                GDP = GDQ;
            } else {
                GDM = A;
                GDP = A;
            }
            let GCS = if GCJ != 0.0 && (if DVV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GDT;
            let GDW;
            if GCS != 0.0 {
                let GCV = B / GCT;
                let GCW = if WJ != 0.0 && (if FZN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GDX = if GCW != 0.0 {
                    let GCZ = B / GCX;
                    GCZ
                } else {
                    A
                };
                GDT = GCV;
                GDW = GDX;
            } else {
                GDT = A;
                GDW = A;
            }
            let GDA = if ABN != 0.0 && FSA != 0.0 { 1.0 } else { 0.0 };
            if GDA != 0.0 {
            } else {
            }
            if EYH != 0.0 {
            } else {
            }
            if ADX != 0.0 {
                let GDC = if ADW == R { 1.0 } else { 0.0 };
                if GDC != 0.0 {
                } else {
                }
            } else {
            }
            let GDI = EYP * GDD;
            let GFF;
            let GFG;
            let GFH;
            let GFJ;
            if GCK != 0.0 {
                let GDL = (Q * AWP) * HD;
                let GDN = GDL * GDM;
                let GDO = if WJ != 0.0 && (if FXT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GFI;
                let GFK;
                if GDO != 0.0 {
                    let GDR = GDL * GDP;
                    GFI = B;
                    GFK = GDR;
                } else {
                    GFI = A;
                    GFK = A;
                }
                GFF = B;
                GFG = GDN;
                GFH = GFI;
                GFJ = GFK;
            } else {
                GFF = A;
                GFG = A;
                GFH = A;
                GFJ = A;
            }
            let GFL;
            let GFM;
            let GFN;
            let GFP;
            if GCS != 0.0 {
                let GDS = (Q * AWP) * HD;
                let GDU = GDS * GDT;
                let GDV = if WJ != 0.0 && (if FZN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let GFO;
                let GFQ;
                if GDV != 0.0 {
                    let GDY = GDS * GDW;
                    GFO = B;
                    GFQ = GDY;
                } else {
                    GFO = A;
                    GFQ = A;
                }
                GFL = B;
                GFM = GDU;
                GFN = GFO;
                GFP = GFQ;
            } else {
                GFL = A;
                GFM = A;
                GFN = A;
                GFP = A;
            }
            let GFR;
            let GFS;
            if ADX != 0.0 {
                let GDZ = ((Q * AWP) * HD) * GDB;
                GFR = B;
                GFS = GDZ;
            } else {
                GFR = A;
                GFS = A;
            }
            let GFT;
            let GFU;
            let GFV;
            let GFW;
            let GFX;
            let GFY;
            if FVD != 0.0 {
                GFT = B;
                GFU = GEA;
                GFV = A;
                GFW = A;
                GFX = A;
                GFY = A;
            } else {
                let GEG = GEA * (B - (GEC * GEC));
                GFT = A;
                GFU = A;
                GFV = B;
                GFW = GEA;
                GFX = B;
                GFY = GEG;
            }
            let GFZ;
            let GGB;
            let GGD;
            let GGF;
            let GGH;
            let GGJ;
            let GGL;
            let GGN;
            if ABF != 0.0 {
                let GGA;
                let GGC;
                let GGE;
                let GGG;
                let GGI;
                let GGK;
                let GGM;
                let GGO;
                if FXM != 0.0 {
                    let GEH = 3.20438e-19f64 * ((FSF + FSJ).abs());
                    let GEI = 3.20438e-19f64 * ((FSD + FSN).abs());
                    GGA = B;
                    GGC = GEH;
                    GGE = B;
                    GGG = GEI;
                    GGI = A;
                    GGK = A;
                    GGM = A;
                    GGO = A;
                } else {
                    let GEJ = 3.20438e-19f64 * ((FSF + FSJ).abs());
                    let GEK = 3.20438e-19f64 * ((FSD + FSN).abs());
                    GGA = A;
                    GGC = A;
                    GGE = A;
                    GGG = A;
                    GGI = B;
                    GGK = GEJ;
                    GGM = B;
                    GGO = GEK;
                }
                GFZ = GGA;
                GGB = GGC;
                GGD = GGE;
                GGF = GGG;
                GGH = GGI;
                GGJ = GGK;
                GGL = GGM;
                GGN = GGO;
            } else {
                GFZ = A;
                GGB = A;
                GGD = A;
                GGF = A;
                GGH = A;
                GGJ = A;
                GGL = A;
                GGN = A;
            }
            let GGP;
            let GGR;
            let GGT;
            let GGV;
            let GGX;
            let GGZ;
            if ABC != 0.0 {
                let GGQ;
                let GGS;
                let GGU;
                let GGW;
                let GGY;
                let GHA;
                if AS != 0.0 {
                    let GEL = 3.20438e-19f64 * ((FSP + FSR).abs());
                    GGQ = B;
                    GGS = GEL;
                    GGU = A;
                    GGW = A;
                    GGY = A;
                    GHA = A;
                } else {
                    let GEM = 3.20438e-19f64 * (FXP.abs());
                    let GEN = 3.20438e-19f64 * (FXQ.abs());
                    GGQ = A;
                    GGS = A;
                    GGU = B;
                    GGW = GEM;
                    GGY = B;
                    GHA = GEN;
                }
                GGP = GGQ;
                GGR = GGS;
                GGT = GGU;
                GGV = GGW;
                GGX = GGY;
                GGZ = GHA;
            } else {
                GGP = A;
                GGR = A;
                GGT = A;
                GGV = A;
                GGX = A;
                GGZ = A;
            }
            if ADS != 0.0 {
                if GCK != 0.0 {
                    let GEO = if WJ != 0.0 && (if FXT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GEO != 0.0 {
                    } else {
                    }
                } else {
                }
                if GCS != 0.0 {
                    let GEP = if WJ != 0.0 && (if FZN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GEP != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            if FXM != 0.0 {
                if AS != 0.0 {
                    let GEQ = if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GEQ != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                if AS != 0.0 {
                    let GER = if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if GER != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if FSS != 0.0 {
            } else {
            }
            if FXM != 0.0 {
                let GES = if (if AS != 0.0 && (if OJ == R { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GES != 0.0 {
                } else {
                }
            } else {
                let GET = if (if AS != 0.0 && (if OJ == R { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GET != 0.0 {
                } else {
                }
            }
            if AS != 0.0 {
            } else {
            }
            if AS != 0.0 {
                let GEU = if (if (if OJ == R { 1.0 } else { 0.0 }) != 0.0 || (if OJ == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if AV == R { 1.0 } else { 0.0 }) != 0.0 || (if AV == DQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || AX != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GEU != 0.0 {
                } else {
                }
            } else {
            }
            let GEY = if GEV > A { 1.0 } else { 0.0 };
            if GEY != 0.0 {
                if AZS != 0.0 {
                    let GEZ = if (GEV / AZR) > BZ { 1.0 } else { 0.0 };
                    if GEZ != 0.0 {
                    } else {
                    }
                } else {
                    let GFA = if GEV > BZ { 1.0 } else { 0.0 };
                    if GFA != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let GFB = if (((HG * HG) * (AWP * (AWP + (((DAP * AWP) / HC) + DMB)))) / (((3.20438e-19f64 * AZR) * L) * AWP)) > BZ { 1.0 } else { 0.0 };
            if GFB != 0.0 {
            } else {
            }
            if FXM != 0.0 {
            } else {
            }
            if AS != 0.0 {
            } else {
            }
            let GFC = if AGM == B { 1.0 } else { 0.0 };
            if GFC != 0.0 {
            } else {
            }
            if GFC != 0.0 {
            } else {
            }
            if GFC != 0.0 {
            } else {
            }
            let GFD = if ADW == R { 1.0 } else { 0.0 };
            if GFD != 0.0 {
                if GFC != 0.0 {
                } else {
                }
                if GFC != 0.0 {
                } else {
                }
            } else {
                if GFC != 0.0 {
                } else {
                }
                if GFC != 0.0 {
                } else {
                }
            }
            if GFD != 0.0 {
                if GFC != 0.0 {
                } else {
                }
            } else {
                if GFC != 0.0 {
                } else {
                }
            }
            if GFD != 0.0 {
                if GFC != 0.0 {
                } else {
                }
            } else {
                if GFC != 0.0 {
                } else {
                }
            }
            if GFC != 0.0 {
            } else {
            }
            if GFC != 0.0 {
            } else {
            }
            if FSS != 0.0 {
            } else {
            }
            if GCJ != 0.0 {
                let GFE = if WJ != 0.0 && (if (if FXT > A { 1.0 } else { 0.0 }) != 0.0 || (if FZN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GFE != 0.0 {
                } else {
                }
            } else {
            }
        {
            let psd = GDI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(GDJ);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFF == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFH == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFL == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFN == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFR == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFT == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFV == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFX == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GFY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GFZ == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GGD == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GGH == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GGL == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GGP == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GGT == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if GGX == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = GGZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
