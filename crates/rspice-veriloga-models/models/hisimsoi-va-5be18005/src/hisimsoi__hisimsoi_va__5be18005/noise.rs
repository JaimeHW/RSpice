#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 8] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DP_SP_IFLICK", label: Some("iflick"), kind: GeneratedNoiseKind::Flicker, equation: 14, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N_GND_INTERNAL", label: Some("internal"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "n", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DP_SP_IDS", label: Some("ids"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SP_S_ISOURCE", label: Some("isource"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DP_IDRAIN", label: Some("idrain"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_DP_IIGD", label: Some("iigd"), kind: GeneratedNoiseKind::White, equation: 23, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_SP_IIGS", label: Some("iigs"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_BP_IIGB", label: Some("iigb"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18])];
            let A = 0e0f64;
            let B = 1e0f64;
            let C = 1.0f64;
            let D = parameters[43];
            let F = 0.0f64;
            let I = 1e-12f64;
            let J = parameters[237];
            let K = 5e-1f64;
            let L = 1e1f64;
            let N = 2e2f64;
            let O = 1e-2f64;
            let Q = 1e-6f64;
            let V = 1e-4f64;
            let Y = parameters[240];
            let AB = parameters[242];
            let AI = parameters[83];
            let AK = parameters[84];
            let AM = parameters[85];
            let AO = parameters[80];
            let AQ = parameters[81];
            let AS = parameters[82];
            let AU = 1e6f64;
            let AW = 2.7315e2f64;
            let AY = parameters[58];
            let AZ = 1e2f64;
            let BB = parameters[46];
            let BC = parameters[34];
            let BD = if parameter_given[190] { 1.0 } else { 0.0 };
            let BE = parameters[190];
            let BH = 2e0f64;
            let BI = 1e-1f64;
            let BN = 4e0f64;
            let BO = 8e0f64;
            let BP = 1.0f64;
            let BQ = 0.0f64;
            let BR = 1.0f64;
            let BS = 0.0f64;
            let BT = 3e0f64;
            let BU = 0.0f64;
            let CH = 1e-7f64;
            let CJ = parameters[236];
            let CK = 1.034943e-10f64;
            let CN = 3.453133e-11f64;
            let CQ = parameters[239];
            let CU = parameters[0];
            let CV = parameters[56];
            let DB = parameters[9];
            let DD = parameters[60];
            let DF = parameters[295];
            let DH = parameters[61];
            let DL = parameters[18];
            let DY = parameters[72];
            let EF = 1.6021918e-19f64;
            let EG = 1.3806226e-23f64;
            let EJ = parameters[244];
            let EM = parameters[248];
            let EQ = parameters[89];
            let ES = parameters[68];
            let EX = parameters[6];
            let FA = parameters[130];
            let FB = parameters[131];
            let FD = parameters[124];
            let FE = parameters[125];
            let FF = parameters[126];
            let FH = parameters[123];
            let FJ = parameters[117];
            let FK = parameters[119];
            let FL = parameters[120];
            let FN = parameters[118];
            let FO = parameters[121];
            let FR = parameters[127];
            let FS = parameters[128];
            let FT = parameters[129];
            let FZ = parameters[65];
            let GE = parameters[114];
            let GF = 1e-50f64;
            let GI = parameters[50];
            let GK = if parameter_given[168] { 1.0 } else { 0.0 };
            let GL = if parameter_given[169] { 1.0 } else { 0.0 };
            let GM = if parameter_given[170] { 1.0 } else { 0.0 };
            let GN = if parameter_given[294] { 1.0 } else { 0.0 };
            let GO = if parameter_given[23] { 1.0 } else { 0.0 };
            let GP = if parameter_given[22] { 1.0 } else { 0.0 };
            let GQ = if parameter_given[16] { 1.0 } else { 0.0 };
            let GR = parameters[17];
            let GU = parameters[13];
            let GV = parameters[14];
            let GX = parameters[10];
            let GY = parameters[11];
            let GZ = parameters[12];
            let HL = parameters[161];
            let HM = parameters[163];
            let HW = parameters[164];
            let HX = parameters[166];
            let IO = 1e-3f64;
            let IP = 1e-10f64;
            let IS = parameters[35];
            let IV = parameters[261];
            let IY = parameters[262];
            let JB = 1e4f64;
            let JE = parameters[24];
            let JF = parameters[23];
            let JG = parameters[19];
            let JJ = parameters[22];
            let KD = node_potentials[6];
            let KE = node_potentials[7];
            let KH = node_potentials[12];
            let KJ = node_potentials[0];
            let KK = node_potentials[2];
            let KM = 1e-9f64;
            let KN = parameters[38];
            let KR = node_potentials[10];
            let KW = -1e0f64;
            let LA = 5e0f64;
            let LC = 6e0f64;
            let LE = temperature;
            let LR = parameters[160];
            let MA = 4e-1f64;
            let MP = 1.414213562373095e0f64;
            let NI = 8e-1f64;
            let NJ = 1.2e0f64;
            let NX = 1.0f64;
            let NY = 0.0f64;
            let NZ = 0.0f64;
            let OA = 1.0f64;
            let OB = 0.0f64;
            let OR = 2e1f64;
            let OY = -2e1f64;
            let PC = -2e1f64;
            let PG = parameters[226];
            let PJ = 5e-12f64;
            let QA = 5e-2f64;
            let QC = 2.0000000000000004e-2f64;
            let QD = 1.0f64;
            let QE = -2.0000000000000004e-2f64;
            let QK = parameters[204];
            let QL = parameters[206];
            let QM = parameters[205];
            let RT = 2e-3f64;
            let RU = 1.0f64;
            let RV = -2e-3f64;
            let TD = parameters[69];
            let TG = parameters[71];
            let TJ = parameters[86];
            let TY = 2.7e1f64;
            let UI = 2e-1f64;
            let UJ = 1.0f64;
            let UK = -2e-1f64;
            let UT = 7e0f64;
            let VE = 1e-5f64;
            let VG = parameters[39];
            let VT = 2.220446049250313e-15f64;
            let VZ = 8e-4f64;
            let YK = 1.0f64;
            let YL = 0.0f64;
            let YM = 1.0f64;
            let YN = 0.0f64;
            let YO = 0.0f64;
            let ZI = 1.0f64;
            let ZJ = 0.0f64;
            let ZK = 1.0f64;
            let ZL = 0.0f64;
            let ZM = 0.0f64;
            let AAD = 0.0f64;
            let AAI = 2.220446049250313e-15f64;
            let AAN = 8.1e1f64;
            let AAQ = 1.458e3f64;
            let AAR = 5.4e1f64;
            let AAT = 3.333333333333333e-1f64;
            let AAV = 1.259921049894873e0f64;
            let ABZ = 9.8e-1f64;
            let ACG = 1.0f64;
            let ACH = 0.0f64;
            let ACI = 1.0f64;
            let ACJ = 0.0f64;
            let ACK = 0.0f64;
            let ADG = 6e-1f64;
            let ADV = 2.220446049250313e-15f64;
            let AFV = parameters[25];
            let AFX = 2e-1f64;
            let AGA = parameters[137];
            let AGM = 3.0000000000000002e-2f64;
            let AGR = 2.220446049250313e-15f64;
            let AGY = 3e-2f64;
            let AHX = 2.5e-1f64;
            let AIT = 0e0f64;
            let AIU = parameters[122];
            let AIX = 0e0f64;
            let AJC = 0e0f64;
            let AJP = 1.0f64;
            let AJQ = 0.0f64;
            let AJR = 0.0f64;
            let AJS = 1.0f64;
            let AJT = 0.0f64;
            let AKU = parameters[26];
            let AKW = parameters[141];
            let AKZ = parameters[140];
            let ALC = parameters[37];
            let ALD = node_potentials[17];
            let AMJ = 5e2f64;
            let AML = 1.403592217853e217f64;
            let AMN = 6e1f64;
            let AMQ = 1.14200738981568e26f64;
            let ANR = 1.0f64;
            let ANS = 0.0f64;
            let ANT = 1.0f64;
            let ANU = 0.0f64;
            let ANV = 0.0f64;
            let APB = 1.0f64;
            let APC = 0.0f64;
            let APD = 1.0f64;
            let APE = 0.0f64;
            let APF = 0.0f64;
            let AQL = -1e0f64;
            let AQO = -1e0f64;
            let ARE = 8e1f64;
            let ARG = 1.25e2f64;
            let ARH = 4e1f64;
            let ARK = 2.5e1f64;
            let ATW = 1.0f64;
            let ATX = 0.0f64;
            let ATY = 0.0f64;
            let ATZ = 1.0f64;
            let AUA = 0.0f64;
            let AUX = 0.0f64;
            let AVV = 2.220446049250313e-15f64;
            let AWK = 2.220446049250313e-15f64;
            let BCW = 1.0f64;
            let BCX = 0.0f64;
            let BCY = 1.0f64;
            let BCZ = 0.0f64;
            let BDA = 0.0f64;
            let BEG = 1.0f64;
            let BEH = 0.0f64;
            let BEI = 1.0f64;
            let BEJ = 0.0f64;
            let BEK = 0.0f64;
            let BFQ = -1e0f64;
            let BFT = -1e0f64;
            let BIM = 1.0f64;
            let BIN = 0.0f64;
            let BIO = 1.0f64;
            let BIP = 0.0f64;
            let BIQ = 0.0f64;
            let BJF = 1.0f64;
            let BJG = 0.0f64;
            let BJH = 1.0f64;
            let BJI = 0.0f64;
            let BJJ = 0.0f64;
            let BKC = 1.0f64;
            let BKD = 0.0f64;
            let BKE = 1.0f64;
            let BKF = 0.0f64;
            let BKG = 0.0f64;
            let BKX = 2.220446049250313e-15f64;
            let BLM = -1e0f64;
            let BLR = 9e0f64;
            let BLV = 1e-8f64;
            let BMB = 1.2e1f64;
            let BMF = 0.0f64;
            let BMJ = 2.220446049250313e-15f64;
            let BOH = 1e-16f64;
            let BOQ = 5e-3f64;
            let BPG = -1e0f64;
            let BQO = 2.01e2f64;
            let BQW = -1e0f64;
            let BSN = 1.0f64;
            let BSO = 0.0f64;
            let BSP = 0.0f64;
            let BSQ = 1.0f64;
            let BSR = 0.0f64;
            let BTO = 0.0f64;
            let BTQ = 1.0f64;
            let BWT = 2.01e2f64;
            let BXB = -1e0f64;
            let BYV = 1.0f64;
            let BYW = 0.0f64;
            let BYX = 0.0f64;
            let BYY = 1.0f64;
            let BYZ = 0.0f64;
            let BZP = 2.220446049250313e-15f64;
            let CAP = parameters[191];
            let CBC = parameters[189];
            let CBR = 1e9f64;
            let CDB = parameters[227];
            let CDF = 2.220446049250313e-15f64;
            let CDI = 1.034943e-12f64;
            let CDY = parameters[113];
            let CEP = parameters[281];
            let CFQ = parameters[156];
            let CFY = -1e0f64;
            let CHD = 1.0f64;
            let CHE = 0.0f64;
            let CHF = 0.0f64;
            let CHG = 1.0f64;
            let CHH = 0.0f64;
            let CHU = parameters[30];
            let CHV = parameters[32];
            let CIL = parameters[285];
            let CIN = parameters[286];
            let CIX = 2.220446049250313e-15f64;
            let CJB = 1.0f64;
            let CJS = parameters[287];
            let CKP = 1.0f64;
            let CKQ = 0.0f64;
            let CKR = 1.0f64;
            let CKS = 0.0f64;
            let CKT = 0.0f64;
            let CPK = 2.01e2f64;
            let CPS = -1e0f64;
            let CQI = -1e0f64;
            let CRK = 1.0f64;
            let CRL = 1.0f64;
            let CRM = 0.0f64;
            let CRN = 0.0f64;
            let CRO = 0.0f64;
            let CSJ = parameters[49];
            let CTF = 1.0f64;
            let CTG = 0.0f64;
            let CTH = 0.0f64;
            let CTI = 1.0f64;
            let CTJ = 0.0f64;
            let CVA = parameters[47];
            let CWC = parameters[27];
            let CWO = parameters[219];
            let CWQ = parameters[218];
            let CWW = parameters[222];
            let CXO = parameters[209];
            let CXP = parameters[210];
            let CXQ = parameters[211];
            let CXV = parameters[208];
            let CYK = 1.0f64;
            let CYO = parameters[292];
            let CYP = 0.0f64;
            let CYV = 1e0f64;
            let CYW = 0e0f64;
            let DAS = 2.220446049250313e-15f64;
            let DAX = 2.220446049250313e-15f64;
            let DCE = parameters[42];
            let DCN = 2.9693154855771e-1f64;
            let DCO = 6.115288895133179e-3f64;
            let DCS = 7.07106781186548e-1f64;
            let DCT = 1.78800506338833e-2f64;
            let DCU = 6.36964918866352e-5f64;
            let DDP = 4.1e1f64;
            let DDX = -1e0f64;
            let DEX = 1.0f64;
            let DFC = 0.0f64;
            let DFF = 0e0f64;
            let DFG = 1e0f64;
            let DGP = 2.220446049250313e-15f64;
            let DGU = 2.220446049250313e-15f64;
            let DJG = 4.1e1f64;
            let DJO = -1e0f64;
            let DKU = 1.0f64;
            let DKX = 0.0f64;
            let DLD = parameters[64];
            let DLL = parameters[188];
            let DLZ = 1e0f64;
            let DMA = 0e0f64;
            let DNW = 2.220446049250313e-15f64;
            let DOB = 2.220446049250313e-15f64;
            let DOK = parameters[41];
            let DQS = 4.1e1f64;
            let DRA = -1e0f64;
            let DSJ = 0e0f64;
            let DSK = 1e0f64;
            let DUD = 2.220446049250313e-15f64;
            let DUI = 2.220446049250313e-15f64;
            let DWY = 4.1e1f64;
            let DXG = -1e0f64;
            let DYS = parameters[170];
            let DYT = parameters[169];
            let EAJ = parameters[174];
            let EAS = parameters[179];
            let EAT = parameters[2];
            let EAV = parameters[3];
            let EAZ = parameters[5];
            let EBB = parameters[180];
            let EBD = parameters[181];
            let EBI = parameters[182];
            let EBL = parameters[183];
            let EBO = parameters[184];
            let EBW = parameters[4];
            let ECU = parameters[233];
            let ECV = parameters[234];
            let EFN = parameters[168];
            let EFR = parameters[167];
            let ELG = parameters[259];
            let ELI = 1.0f64;
            let ELJ = parameters[264];
            let ELL = parameters[266];
            let ELM = parameters[268];
            let ELN = parameters[273];
            let ELO = parameters[263];
            let ELQ = parameters[255];
            let ELT = parameters[258];
            let ELV = parameters[265];
            let ELW = parameters[267];
            let ELX = parameters[272];
            let ELZ = parameters[256];
            let EMC = parameters[257];
            let EME = parameters[271];
            let EMI = parameters[269];
            let EMJ = parameters[270];
            let EML = parameters[274];
            let EMN = parameters[279];
            let EMO = parameters[280];
            let EMP = parameters[277];
            let EMQ = parameters[278];
            let EMR = parameters[275];
            let EMS = parameters[276];
            let ENZ = parameters[260];
            let EOB = 0.0f64;
            let EQL = parameters[231];
            if C != 0.0 {
                let E = if D == A { 1.0 } else { 0.0 };
                if E != 0.0 {
                } else {
                }
            } else {
                if F != 0.0 {
                    let G = if D == B { 1.0 } else { 0.0 };
                    if G != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let H = if D == A { 1.0 } else { 0.0 };
            if H != 0.0 {
            } else {
            }
            let M = (parameters[51] * L) % L;
            let P = parameters[52] * O;
            let R = parameters[73] / Q;
            let S = parameters[104] * O;
            let T = parameters[201] / Q;
            let U = parameters[229] * O;
            let W = parameters[228] / V;
            let X = parameters[230] / V;
            let Z = Y / Q;
            let AA = parameters[241] / Q;
            let AC = AB * O;
            let AD = parameters[59] / Q;
            let AE = parameters[284] / Q;
            let AF = parameters[148] / Q;
            let AG = parameters[198] / V;
            let AH = parameters[70] * O;
            let AJ = if AI == A { 1.0 } else { 0.0 };
            let AL = if AJ != 0.0 {
                A
            } else {
                AK
            };
            let AN = if AJ != 0.0 {
                A
            } else {
                AM
            };
            let AP = if AO == A { 1.0 } else { 0.0 };
            let AR = if AP != 0.0 {
                A
            } else {
                AQ
            };
            let AT = if AJ != 0.0 {
                A
            } else {
                AS
            };
            let AV = parameters[250] * AU;
            let AX = parameters[232] + AW;
            let BA = parameters[15] * AZ;
            let BG = if BD != 0.0 {
                BE
            } else {
                let BF = 5e9f64 / (J * Y);
                BF
            };
            let BJ = if (if BG < 2.1e0f64 { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
            let CBA;
            if BJ != 0.0 {
                let BK = 2.1e0f64 - BG;
                let BL = BK * BK;
                let BM = (BL * BL) + 1.0000000000000005e-4f64;
                let CF;
                if BP != 0.0 {
                    let CA;
                    if BQ != 0.0 {
                        CA = B;
                    } else {
                        let CB;
                        if BR != 0.0 {
                            CB = BH;
                        } else {
                            let CC;
                            if BS != 0.0 {
                                CC = BT;
                            } else {
                                let CD = if BU != 0.0 {
                                    BN
                                } else {
                                    A
                                };
                                CC = CD;
                            }
                            CB = CC;
                        }
                        CA = CB;
                    }
                    let mut BV = 0.0;
                    let mut BX = 0.0;
                    BV = A;
                    BX = BM;
                    loop {
                        let BW = if BV < CA { 1.0 } else { 0.0 };
                        if BW == 0.0 {
                            break;
                        }
                        let BY = BX.sqrt();
                        let BZ = BV + B;
                        BV = BZ;
                        BX = BY;
                    }
                    CF = BX;
                } else {
                    let CE = BM.powf(2.5e-1f64);
                    CF = CE;
                }
                let CG = 2.1e0f64 - ((BK * BI) * (B / CF));
                CBA = CG;
            } else {
                CBA = BG;
            }
            let CI = parameters[55] - (AX * (9.025e-5f64 + (AX * CH)));
            let CL = CK / J;
            let CM = B / CL;
            let CO = CN / CJ;
            let CP = CJ / CN;
            let CR = CN / CQ;
            let CS = CQ / CN;
            let CT = CS + CM;
            let CW = CU - (BH * CV);
            let CX = CU - (BH * parameters[57]);
            let CY = if parameters[40] == A { 1.0 } else { 0.0 };
            let CZ = if CY != 0.0 {
                CU
            } else {
                CW
            };
            let DA = CZ * AU;
            let DC = parameters[1] / DB;
            let DE = if M < B { 1.0 } else { 0.0 };
            let DG = if DE != 0.0 {
                A
            } else {
                DF
            };
            let DI = if DE != 0.0 {
                DD
            } else {
                DH
            };
            let DQ;
            let DS;
            if H != 0.0 {
                let DJ = DC - (BH * DD);
                let DK = DC - (BH * DI);
                DQ = DJ;
                DS = DK;
            } else {
                let DM = DC - (DL * DG);
                let DN = BH - DL;
                let DO = DM - (DN * DD);
                let DP = DM - (DN * DI);
                DQ = DO;
                DS = DP;
            }
            let DR = DQ * DB;
            let DT = DS * DB;
            let DU = DC * AU;
            let DV = DU * DA;
            let DW = (parameters[107] * (B + (parameters[108] / (DA.powf(parameters[111]))))) * (B + (parameters[109] / (DU.powf(parameters[110]))));
            let DX = if M > BT { 1.0 } else { 0.0 };
            let DZ = if DY > A { 1.0 } else { 0.0 };
            let EA = if (if DX != 0.0 && (if R < Z { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && DZ != 0.0 { 1.0 } else { 0.0 };
            let EB = if EA != 0.0 {
                Z
            } else {
                R
            };
            let EC = EB * (B + (parameters[74] / (DU.powf(parameters[75]))));
            let ED = K * CU;
            let EE = BH / ((B / (parameters[62] + ED)) + (B / (parameters[63] + ED)));
            let EH = EF / (EG * AX);
            let EI = (EF * AA) * CK;
            let EK = EJ * (DA.powf((-parameters[247])));
            let EL = parameters[251] * (DA.powf((-parameters[252])));
            let EN = EM * ((DA + AV).powf((-parameters[249])));
            let EO = ((3.2043836e-19f64 * AF) * CK).sqrt();
            let EP = B / (AF * AF);
            let ER = ((B + (B / DA)).powf(parameters[91])) * EQ;
            let ET = CZ + (parameters[76] / (DV.powf(parameters[77])));
            let EU = parameters[78] / (DV.powf(parameters[79]));
            let EV = (parameters[149] * (B + (parameters[150] / ((ET * AU).powf(parameters[151]))))) + (parameters[152] / (DU.powf(parameters[153])));
            let EW = B + ((DA.powf(parameters[192])) * parameters[193]);
            let EY = (parameters[67] * (parameters[7] + (DQ / (BT * EX)))) / ((EX * (CU - parameters[8])) * DB);
            let EZ = if parameters[44] <= A { 1.0 } else { 0.0 };
            let AIN;
            let AIV;
            let AIW;
            let AJB;
            let AKR;
            let AKS;
            if EZ != 0.0 {
                let FC = B + (FA / (DU.powf(FB)));
                let FG = FD * (B + (FE / (DA.powf(FF))));
                let FI = DA / (DA + FH);
                let FM = FJ * (B + (FK / (DA.powf(FL))));
                let FP = FN * (B + (FO / DA));
                AIN = FG;
                AIV = FI;
                AIW = FC;
                AJB = AJC;
                AKR = FP;
                AKS = FM;
            } else {
                let FQ = DU.powf(FB);
                let FU = (FR * (B + (FS / (DA.powf(FT))))) * (FQ / (FQ + FA));
                let FV = FD * (B + (FE / (DA.powf(FF))));
                let FW = FH * (B + (parameters[132] / (DA.powf(parameters[133]))));
                let FX = FJ * (B + (FK / (DA.powf(FL))));
                let FY = FN * (B + (FO / DA));
                AIN = FV;
                AIV = FW;
                AIW = AIX;
                AJB = FU;
                AKR = FY;
                AKS = FX;
            }
            let GA = ((AU * DT) * FZ) / (DA.powf(parameters[66]));
            let GB = parameters[134] * (B + (parameters[135] / (DA.powf(parameters[136]))));
            let AIS = if EZ != 0.0 {
                let GC = FR * (B + (FS / (DA.powf(FT))));
                GC
            } else {
                AIT
            };
            let GD = parameters[115] * DA;
            let GG = (((GD * GE) / (GD + GE)) + parameters[116]) + GF;
            let GH = if GG < BT { 1.0 } else { 0.0 };
            let AUT = if GH != 0.0 {
                BT
            } else {
                GG
            };
            let GJ = GI * parameters[253];
            let GS = if GR == A { 1.0 } else { 0.0 };
            let GT = if GS != 0.0 {
                A
            } else {
                B
            };
            let GW = parameters[16] + AW;
            let HA = if (if (if GX > A { 1.0 } else { 0.0 }) != 0.0 && (if GY > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if DB == B { 1.0 } else { 0.0 }) != 0.0 || (if (if DB > B { 1.0 } else { 0.0 }) != 0.0 && (if GZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HI;
            if HA != 0.0 {
                let mut HB = 0.0;
                let mut HD = 0.0;
                HB = A;
                HD = A;
                loop {
                    let HC = if HB < DB { 1.0 } else { 0.0 };
                    if HC == 0.0 {
                        break;
                    }
                    let HE = HB * (GZ + CU);
                    let HF = (HD + (B / ((GX + ED) + HE))) + (B / ((GY + ED) + HE));
                    let HG = HB + B;
                    HB = HG;
                    HD = HF;
                }
                let HH = (BH * DB) / HD;
                HI = HH;
            } else {
                HI = A;
            }
            let HJ = if HI > A { 1.0 } else { 0.0 };
            let IB = if HJ != 0.0 {
                let HK = B / (B + parameters[162]);
                let HN = (EC * (B + (HK * ((HL / HI).powf(HM))))) / (B + (HK * ((HL / EE).powf(HM))));
                HN
            } else {
                EC
            };
            let HO = T / Z;
            let HP = (HO - ((B + (parameters[199] / (DU.powf(parameters[200])))) * (B + (parameters[202] / (DA.powf(parameters[203])))))) - O;
            let HQ = (BN * HO) * O;
            let HR = if HQ > A { 1.0 } else { 0.0 };
            let HT = if HR != 0.0 {
                HQ
            } else {
                let HS = -HQ;
                HS
            };
            let HU = Z * (HO - (K * (HP + (((HP * HP) + HT).sqrt()))));
            let IA = if HJ != 0.0 {
                let HV = B / (B + parameters[165]);
                let HY = (HU * (B + (HV * ((HW / HI).powf(HX))))) / (B + (HV * ((HW / EE).powf(HX))));
                HY
            } else {
                HU
            };
            let HZ = if (if CZ > DY { 1.0 } else { 0.0 }) != 0.0 || (if DY <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IE = if HZ != 0.0 {
                let IC = ((IA * (CZ - DY)) + (IB * DY)) / CZ;
                IC
            } else {
                let ID = IB + (((IB - IA) * (DY - CZ)) / DY);
                ID
            };
            let IF = EF * IE;
            let IG = IF * CK;
            let IH = BH * IG;
            let II = if (if CZ <= (BH * DY) { 1.0 } else { 0.0 }) != 0.0 && DZ != 0.0 { 1.0 } else { 0.0 };
            let LX = if II != 0.0 {
                let IJ = ((((BH * IB) - (((IB - IA) * CZ) / DY)) - IA) / IA).ln();
                IJ
            } else {
                A
            };
            let IK = 5.1702525384001115e-2f64 * ((IE / 1.04e16f64).ln());
            let IL = 5.1702525384001115e-2f64 * ((IA / 1.04e16f64).ln());
            let IM = (1.2919089961638799e9f64 / IE).sqrt();
            let IN = (B + (parameters[194] / (DA.powf(parameters[195])))) * (B + (parameters[196] / (DV.powf(parameters[197]))));
            let IQ = (K * (IN + (((IN * IN) + 4e-6f64).sqrt()))) + 1e-13f64;
            let IR = if IQ < A { 1.0 } else { 0.0 };
            let LZ = if IR != 0.0 {
                A
            } else {
                IQ
            };
            let IT = if IS == B { 1.0 } else { 0.0 };
            if IT != 0.0 {
                let IU = if EY > IO { 1.0 } else { 0.0 };
                if IU != 0.0 {
                } else {
                }
            } else {
            }
            let IW = if IV == B { 1.0 } else { 0.0 };
            if IW != 0.0 {
                let IX = if ((parameters[289] * DR) + parameters[288]) < V { 1.0 } else { 0.0 };
                if IX != 0.0 {
                } else {
                }
            } else {
            }
            let IZ = if IY == B { 1.0 } else { 0.0 };
            if IZ != 0.0 {
                let JA = if parameters[290] < V { 1.0 } else { 0.0 };
                if JA != 0.0 {
                } else {
                }
                let JC = if parameters[291] < V { 1.0 } else { 0.0 };
                if JC != 0.0 {
                } else {
                }
            } else {
            }
            let JD = if D == B { 1.0 } else { 0.0 };
            let BRW;
            let CYL;
            let DLP;
            let DYV;
            let EAL;
            let EAM;
            let EFG;
            let EFJ;
            let EFU;
            let EFV;
            if JD != 0.0 {
                let BRX;
                let CYM;
                let EFH;
                let EFK;
                if JE != 0.0 {
                    let JI = if GO != 0.0 {
                        JF
                    } else {
                        let JH = (parameters[20] * DB) * JG;
                        JH
                    };
                    let JL = if GP != 0.0 {
                        JJ
                    } else {
                        let JK = (parameters[21] * DB) * JG;
                        JK
                    };
                    let JM = if (if JI > A { 1.0 } else { 0.0 }) != 0.0 && GN != 0.0 { 1.0 } else { 0.0 };
                    let EFI = if JM != 0.0 {
                        let JN = (-JI) * parameters[294];
                        JN
                    } else {
                        A
                    };
                    let JO = if (if JL > A { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[293] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BRY;
                    let EFL;
                    if JO != 0.0 {
                        let JP = (-JL) * parameters[293];
                        BRY = A;
                        EFL = JP;
                    } else {
                        BRY = JL;
                        EFL = A;
                    }
                    BRX = BRY;
                    CYM = JI;
                    EFH = EFI;
                    EFK = EFL;
                } else {
                    BRX = A;
                    CYM = A;
                    EFH = A;
                    EFK = A;
                }
                let JQ = if JG > CU { 1.0 } else { 0.0 };
                let JS = if JQ != 0.0 {
                    let JR = K * (JG - CU);
                    JR
                } else {
                    A
                };
                let JT = if (if parameter_given[13] { 1.0 } else { 0.0 }) == A { 1.0 } else { 0.0 };
                let JV = if JT != 0.0 {
                    JS
                } else {
                    GU
                };
                let JU = if (if parameter_given[14] { 1.0 } else { 0.0 }) == A { 1.0 } else { 0.0 };
                let JY = if JU != 0.0 {
                    JS
                } else {
                    GV
                };
                let JW = DB * JV;
                let JX = DR + JW;
                let JZ = DB * JY;
                let KA = DR + JZ;
                let KB = DT + JW;
                let KC = DT + JZ;
                BRW = BRX;
                CYL = CYM;
                DLP = KC;
                DYV = KB;
                EAL = JX;
                EAM = KA;
                EFG = EFH;
                EFJ = EFK;
                EFU = JV;
                EFV = JY;
            } else {
                BRW = A;
                CYL = A;
                DLP = A;
                DYV = A;
                EAL = A;
                EAM = A;
                EFG = A;
                EFJ = A;
                EFU = GU;
                EFV = GV;
            }
            let KF = GI * (KD - KE);
            let KG = GI * (node_potentials[11] - KE);
            let KI = GI * (KH - KE);
            let EAH;
            let EAI;
            if JD != 0.0 {
                let KL = GI * (KH - KD);
                if BC != 0.0 {
                } else {
                }
                EAH = KL;
                EAI = KI;
            } else {
                if BC != 0.0 {
                } else {
                }
                EAH = A;
                EAI = A;
            }
            let KO = if KN > A { 1.0 } else { 0.0 };
            let KP = if AC > A { 1.0 } else { 0.0 };
            let KQ = if KO != 0.0 && KP != 0.0 { 1.0 } else { 0.0 };
            let KU;
            if KQ != 0.0 {
                let KS = if KR > A { 1.0 } else { 0.0 };
                let KT = if KS != 0.0 {
                    KR
                } else {
                    A
                };
                KU = KT;
            } else {
                KU = A;
            }
            let KV = if KF >= A { 1.0 } else { 0.0 };
            let NO;
            let OQ;
            let OU;
            let CYX;
            let CYY;
            let DZI;
            if KV != 0.0 {
                NO = KI;
                OQ = KF;
                OU = KG;
                CYX = B;
                CYY = A;
                DZI = B;
            } else {
                let KX = -KF;
                let KY = KG - KF;
                let KZ = KI - KF;
                NO = KZ;
                OQ = KX;
                OU = KY;
                CYX = A;
                CYY = B;
                DZI = KW;
            }
            let LB = if BB >= LA { 1.0 } else { 0.0 };
            if LB != 0.0 {
            } else {
            }
            let LD = if BB >= LC { 1.0 } else { 0.0 };
            if LD != 0.0 {
            } else {
            }
            let LF = if GQ != 0.0 {
                GW
            } else {
                LE
            };
            let LH = if GT != 0.0 {
                let LG = LF + GR;
                LG
            } else {
                LF
            };
            let LI = LH + KU;
            let LJ = LI - AX;
            let LK = (CI - (parameters[53] * LJ)) - (parameters[54] * (LJ * (LI + AX)));
            let LL = EF / (EG * LI);
            let LM = LL * LL;
            let LN = B / LL;
            let LO = ((parameters[254] * (B + (parameters[98] / (DU.powf(parameters[99]))))) * (B + (parameters[100] / (DA.powf(parameters[101]))))) * (B + (parameters[102] / (DV.powf(parameters[103]))));
            let LP = B / (B + parameters[159]);
            let LQ = parameters[158] / BA;
            let LS = if (if LQ == A { 1.0 } else { 0.0 }) != 0.0 && (if LR == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let LU = if LS != 0.0 {
                B
            } else {
                let LT = LQ.powf(LR);
                LT
            };
            let LV = LI / AX;
            let LW = (LV.powf(parameters[112])) / (LO * (B + (LP * LU)));
            let LY = LX * LN;
            let MB = (1.8e0f64 + (MA * LV)) + ((BI * LV) * LV);
            let MC = B - LV;
            let MD = (LZ * P) / (MB - (S * MC));
            let ME = LK.sqrt();
            let MF = LK * ME;
            let MG = (1.04e16f64 * (LV * (LV.sqrt()))) * (((((-LK) / BH) * LL) + ((CI / BH) * EH)).exp());
            let MH = LN.sqrt();
            let MI = EO * MH;
            let MJ = MI * MI;
            let MK = MG * MG;
            let ML = MK * EP;
            let NA = if DX != 0.0 {
                let MM = (BH * LN) * ((IE / MG).ln());
                MM
            } else {
                let MN = (BH * LN) * ((IA / MG).ln());
                MN
            };
            let MO = CK / IF;
            let MQ = (IF * MP) * ((MO * LN).sqrt());
            let MW;
            let VK;
            let VW;
            if JD != 0.0 {
                let MR = MG / IE;
                MW = MR;
                VK = A;
                VW = A;
            } else {
                let MS = ((BH * EI) * LN).sqrt();
                let MT = MG / AA;
                let MU = MT * MT;
                let MV = MG / IA;
                MW = MV;
                VK = MS;
                VW = MU;
            }
            let MX = MW * MW;
            let MY = (BH * (MO / LL)).sqrt();
            let MZ = 1.2919089961638799e9f64 / IA;
            let NB = ((1.2919089961638799e9f64 * NA) / IA).sqrt();
            let NC = if DQ < KM { 1.0 } else { 0.0 };
            let NH = if NC != 0.0 {
                B
            } else {
                A
            };
            let ND = if DS < KM { 1.0 } else { 0.0 };
            let NG = if ND != 0.0 {
                B
            } else {
                NH
            };
            let NE = if CW < KM { 1.0 } else { 0.0 };
            let NF = if NE != 0.0 {
                B
            } else {
                NG
            };
            if NF != 0.0 {
            } else {
            }
            let NK;
            let NL;
            if JD != 0.0 {
                NK = MA;
                NL = NI;
            } else {
                NK = NI;
                NL = NJ;
            }
            let NM = NL * K;
            let NN = if NK > NM { 1.0 } else { 0.0 };
            let NP = if NN != 0.0 {
                NM
            } else {
                NK
            };
            let NQ = if NO > NP { 1.0 } else { 0.0 };
            let PA;
            let PE;
            if NQ != 0.0 {
                let NR = NO - NP;
                let NS = NL - NP;
                let NT = NR * NR;
                let NU = NS * NS;
                let NV = ((NU * NU) * NU) * NU;
                let NW = (((NT * NT) * NT) * NT) + NV;
                let OM;
                if NX != 0.0 {
                    let OH;
                    if NY != 0.0 {
                        OH = B;
                    } else {
                        let OI;
                        if NZ != 0.0 {
                            OI = BH;
                        } else {
                            let OJ;
                            if OA != 0.0 {
                                OJ = BT;
                            } else {
                                let OK = if OB != 0.0 {
                                    BN
                                } else {
                                    A
                                };
                                OJ = OK;
                            }
                            OI = OJ;
                        }
                        OH = OI;
                    }
                    let mut OC = 0.0;
                    let mut OE = 0.0;
                    OC = A;
                    OE = NW;
                    loop {
                        let OD = if OC < OH { 1.0 } else { 0.0 };
                        if OD == 0.0 {
                            break;
                        }
                        let OF = OE.sqrt();
                        let OG = OC + B;
                        OC = OG;
                        OE = OF;
                    }
                    OM = OE;
                } else {
                    let OL = NW.powf(1.25e-1f64);
                    OM = OL;
                }
                let ON = B / OM;
                let OO = ((NS * NV) * ON) / NW;
                let OP = NP + ((NR * NS) * ON);
                PA = OP;
                PE = OO;
            } else {
                PA = NO;
                PE = B;
            }
            let OS = if OQ > OR { 1.0 } else { 0.0 };
            let OT = if OS != 0.0 {
                OR
            } else {
                OQ
            };
            let OV = if OU > OR { 1.0 } else { 0.0 };
            let OW = if OV != 0.0 {
                OR
            } else {
                OU
            };
            let OX = if OU < -2e1f64 { 1.0 } else { 0.0 };
            let OZ = if OX != 0.0 {
                OY
            } else {
                OW
            };
            let PB = if PA < -2e1f64 { 1.0 } else { 0.0 };
            let PD = if PB != 0.0 {
                PC
            } else {
                PA
            };
            let PF = BH * ((PE * OT) / BH);
            let PH = PF / PG;
            let PI = PG / (B + (PH * (5e-1f64 + (PH * (1.6666666666666666e-1f64 + (PH * (4.1666666666666664e-2f64 + (PH * (8.333333333333333e-3f64 + (PH * (1.388888888888889e-3f64 + (PH * 1.984126984126984e-4f64))))))))))));
            let PK = if PI < PJ { 1.0 } else { 0.0 };
            let PL = if PK != 0.0 {
                PJ
            } else {
                PI
            };
            let PM = PD + PL;
            let PN = OT + (BH * PL);
            let PO = OZ + PL;
            let PV;
            let RR;
            if JD != 0.0 {
                PV = PD;
                RR = PM;
            } else {
                let PP = if M < BT { 1.0 } else { 0.0 };
                let PQ = if PP != 0.0 {
                    PD
                } else {
                    A
                };
                let PR = if PP != 0.0 {
                    PM
                } else {
                    A
                };
                PV = PQ;
                RR = PR;
            }
            let PS = (BH * IF) * CK;
            let PT = (PS * CP) * CP;
            let PU = OZ - ES;
            let PW = B + ((BH / PT) * ((PU - LN) - PV));
            let PX = (K * (PW + (((PW * PW) + 4e-6f64).sqrt()))) + 1e-13f64;
            let PY = if PX < A { 1.0 } else { 0.0 };
            let PZ = if PY != 0.0 {
                A
            } else {
                PX
            };
            let QB = (((PU + (PT * (B - ((PZ + GF).sqrt())))) - NA) - BI) - QA;
            let QF = if QD != 0.0 {
                QC
            } else {
                QE
            };
            let QG = OT / (BI + (K * (QB + (((QB * QB) + QF).sqrt()))));
            let QH = QG * QG;
            let QI = B - (B / ((((B + QG) + QH) + (QH * QG)) + (QH * QH)));
            let QJ = QI * QI;
            let QN = if (if (if QK == A { 1.0 } else { 0.0 }) != 0.0 && (if QL == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QM == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let QQ = if QN != 0.0 {
                A
            } else {
                B
            };
            let QO = IK + ES;
            let QP = QO + (((PS * IK).sqrt()) / CO);
            let QR = if QQ == A { 1.0 } else { 0.0 };
            let SR;
            let TM;
            let UV;
            if QR != 0.0 {
                let QS = ((MQ * CP) * CP) * MQ;
                SR = CP;
                TM = CO;
                UV = QS;
            } else {
                let QT = ((OZ - PV) - QP) + QM;
                let QU = (K * (QT + (((QT * QT) + 4e-8f64).sqrt()))) + 1.0000000000000002e-14f64;
                let QV = if QU < A { 1.0 } else { 0.0 };
                let QW = if QV != 0.0 {
                    A
                } else {
                    QU
                };
                let QX = B / QW;
                let QY = BH * (QP.abs());
                let QZ = (ES - QP) + QM;
                let RA = if QZ > QY { 1.0 } else { 0.0 };
                let RB = if RA != 0.0 {
                    QZ
                } else {
                    QY
                };
                let RC = B / RB;
                let RD = (RC - QX) - V;
                let RE = (BN * RC) * V;
                let RF = if RE > A { 1.0 } else { 0.0 };
                let RH = if RF != 0.0 {
                    RE
                } else {
                    let RG = -RE;
                    RG
                };
                let RI = (QK * (RC - (K * (RD + (((RD * RD) + RH).sqrt()))))) + QL;
                let RJ = if (RI * 1e12f64) < CJ { 1.0 } else { 0.0 };
                let RK = if RJ != 0.0 {
                    A
                } else {
                    RI
                };
                let RL = CJ + RK;
                let RM = CN / RL;
                let RN = RL / CN;
                let RO = ((MQ * MQ) * RN) * RN;
                SR = RN;
                TM = RM;
                UV = RO;
            }
            let RP = if M < BT { 1.0 } else { 0.0 };
            let RQ = if JD != 0.0 || RP != 0.0 { 1.0 } else { 0.0 };
            let SL;
            if RQ != 0.0 {
                let RS = (K - RR) - IO;
                let RW = if RU != 0.0 {
                    RT
                } else {
                    RV
                };
                let RX = (((((-J) * J) * IF) / 2.069886e-10f64) + NA) - LN;
                let RY = ((K - (K * (RS + (((RS * RS) + RW).sqrt())))) - RX) - IO;
                let RZ = (BN * RX) * IO;
                let SA = if RZ > A { 1.0 } else { 0.0 };
                let SC = if SA != 0.0 {
                    RZ
                } else {
                    let SB = -RZ;
                    SB
                };
                let SD = RX + (K * (RY + (((RY * RY) + SC).sqrt())));
                let SE = if M > BH { 1.0 } else { 0.0 };
                let SM;
                if SE != 0.0 {
                    let SF = (IK - SD) - IO;
                    let SG = (BN * IK) * IO;
                    let SH = if SG > A { 1.0 } else { 0.0 };
                    let SJ = if SH != 0.0 {
                        SG
                    } else {
                        let SI = -SG;
                        SI
                    };
                    let SK = IK - (K * (SF + (((SF * SF) + SJ).sqrt())));
                    SM = SK;
                } else {
                    SM = SD;
                }
                SL = SM;
            } else {
                SL = A;
            }
            let TC = if RP != 0.0 {
                J
            } else {
                let SN = ((2.069886e-10f64 / IF) * (IK - SL)).sqrt();
                SN
            };
            let SQ = if RP != 0.0 {
                let SO = (IH * IK).sqrt();
                SO
            } else {
                let SP = (IH * (IK - SL)).sqrt();
                SP
            };
            let SS = (QO + (SQ * SR)) + LY;
            let ST = 9.5e-1f64 * IK;
            let SU = (ST - SL) - IO;
            let SV = IK - (ST - (K * (SU + (((SU * SU) + ((3.8e0f64 * IK) * IO)).sqrt()))));
            let SW = SV.sqrt();
            let SX = if DY != A { 1.0 } else { 0.0 };
            let TN;
            if SX != 0.0 {
                let SY = (3.2043836e-19f64 * IA) * CK;
                let TB = if RP != 0.0 {
                    let SZ = (SY * IL).sqrt();
                    SZ
                } else {
                    let TA = (SY * (IL - SL)).sqrt();
                    TA
                };
                let TE = ((SS - ((IL + ES) + (TB * SR))) * (((CK * SR) * ((BH * TC) * (B / (DY * DY)))) * (TD - IK))) * ((AO + ((AT / DY) * SV)) + (AR * PN));
                TN = TE;
            } else {
                TN = A;
            }
            let TF = TD - IK;
            let TH = CZ - TG;
            let TI = (((SR * ((CK * TC) * BH)) * TF) * (B / (TH * TH))) * ((AI + ((AN / CZ) * SV)) + (AL * PN));
            let TK = if TJ > A { 1.0 } else { 0.0 };
            let TP = if TK != 0.0 {
                let TL = (((LK + NA) - (BH * parameters[88])) + (parameters[87] * PN)) * ((TJ * J) / ((CZ * K) + AH));
                TL
            } else {
                A
            };
            let TO = TI + TN;
            let TQ = ((TO + ((SQ * (SR - (B / (TM + (AG / DQ))))) + (parameters[105] / DU))) + TP) + EU;
            let TR = SS - TQ;
            let TS = if EQ == A { 1.0 } else { 0.0 };
            let TT = if TS != 0.0 {
                A
            } else {
                B
            };
            let TU = if TT == A { 1.0 } else { 0.0 };
            let UN;
            if TU != 0.0 {
                UN = A;
            } else {
                let TV = PO - parameters[90];
                let TW = if TV < -3e0f64 { 1.0 } else { 0.0 };
                let UB;
                if TW != 0.0 {
                    UB = A;
                } else {
                    let TX = if TV < A { 1.0 } else { 0.0 };
                    let UC = if TX != 0.0 {
                        let TZ = B + (TV * (B + (TV * (3.333333333333333e-1f64 + (TV * 3.7037037037037035e-2f64)))));
                        TZ
                    } else {
                        let UA = B + (TV * (B + (TV * (3.333333333333333e-1f64 + (TV * (4.02052934513951e-2f64 + (TV * 1.48148111111111e-1f64)))))));
                        UA
                    };
                    UB = UC;
                }
                let UD = UB - B;
                let UE = (K * (UD + (((UD * UD) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64;
                let UF = if UE < A { 1.0 } else { 0.0 };
                let UG = if UF != 0.0 {
                    A
                } else {
                    UE
                };
                let UH = (B - (UG * ER)) - QA;
                let UL = if UJ != 0.0 {
                    UI
                } else {
                    UK
                };
                let UM = B - (K * (UH + (((UH * UH) + UL).sqrt())));
                UN = UM;
            }
            let UO = (PU + TQ) - UN;
            let UP = LN * ((IA / AA).ln());
            let UQ = (ES - TQ) + UN;
            let UR = MQ * SR;
            let US = UR * UR;
            let BZX;
            let BZZ;
            let CAC;
            let CAF;
            let CAK;
            let CAR;
            let CAV;
            let CAZ;
            let CBL;
            let CCE;
            let CCL;
            let CCT;
            let CCU;
            let CCX;
            let CFA;
            let CGF;
            let CGT;
            let CHX;
            let CJJ;
            let CJN;
            let CJO;
            let CLO;
            let CSK;
            let CUP;
            let CVG;
            let CVR;
            let EDY;
            let EGK;
            let EGP;
            let EGT;
            let EGX;
            let EIL;
            let EIW;
            if H != 0.0 {
                let UU = NA + B;
                let UW = (B / MX) / UV;
                let UX = (MZ * ((((UW * UU) * UU).ln()) / (LL + (BH / UU)))).sqrt();
                let UY = if UX > J { 1.0 } else { 0.0 };
                let UZ = if UY != 0.0 {
                    J
                } else {
                    UX
                };
                let VA = (-1.6021918e-19f64 * IA) * UZ;
                let VB = (-1.6021918e-19f64 * IA) * J;
                let VC = -VB;
                let VD = VC * IO;
                let VF = VC * VE;
                let VM = if VG != 0.0 {
                    let VH = PM + UP;
                    VH
                } else {
                    let VI = PD + UP;
                    VI
                };
                let VJ = (BH / LL) * ((AA / MG).ln());
                let VL = ((VK * VK) * CT) * CT;
                let VN = -VM;
                let VO = VL * LL;
                let VP = (BH * VN) + VO;
                let VQ = VN * VN;
                let VR = (VP * VP) - (BN * (VQ + VL));
                let VS = if VR >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let VU = if VS != 0.0 {
                    VR
                } else {
                    VT
                };
                let VV = (VP - (VU.sqrt())) / BH;
                let VX = (((VQ / VL) / VW).ln()) / (LL + (BH / VN));
                let VY = if VV < VJ { 1.0 } else { 0.0 };
                let XQ;
                if VY != 0.0 {
                    XQ = VV;
                } else {
                    let WA = (VX - VV) - VZ;
                    let WB = (BN * VX) * VZ;
                    let WC = if WB > A { 1.0 } else { 0.0 };
                    let WE = if WC != 0.0 {
                        WB
                    } else {
                        let WD = -WB;
                        WD
                    };
                    let WF = VX - (K * (WA + (((WA * WA) + WE).sqrt())));
                    XQ = WF;
                }
                let mut WG = 0.0;
                let mut WI = 0.0;
                let mut XR = 0.0;
                let mut AAB = 0.0;
                WG = A;
                WI = XQ;
                XR = A;
                AAB = A;
                loop {
                    let WH = if WG < N { 1.0 } else { 0.0 };
                    if WH == 0.0 {
                        break;
                    }
                    let WJ = LL * WI;
                    let WK = (-WJ).exp();
                    let WL = if WI > KM { 1.0 } else { 0.0 };
                    let WU;
                    let XJ;
                    if WL != 0.0 {
                        let WM = WJ.exp();
                        let WN = (-VK) * ((((WK + WJ) - B) + (VW * (WM - B))).sqrt());
                        let WO = (EI / WN) * (((-WK) + B) + (VW * WM));
                        WU = WN;
                        XJ = WO;
                    } else {
                        let WP = if WI < -1e-9f64 { 1.0 } else { 0.0 };
                        let WV;
                        let XK;
                        if WP != 0.0 {
                            let WQ = VK * (((WK + WJ) - B).sqrt());
                            let WR = (EI / WQ) * ((-WK) + B);
                            WV = WQ;
                            XK = WR;
                        } else {
                            let WS = ((-((EI / LL).sqrt())) * LL) * WI;
                            let WT = -((EI * LL).sqrt());
                            WV = WS;
                            XK = WT;
                        }
                        WU = WV;
                        XJ = XK;
                    }
                    let WW = ((WU * WU) + ((BN * VD) * VD)).sqrt();
                    let WX = K * (B + (WU / WW));
                    let WY = (K * (WU + WW)) + (IP * VD);
                    let WZ = if WY < A { 1.0 } else { 0.0 };
                    let XA;
                    let XI;
                    if WZ != 0.0 {
                        XA = A;
                        XI = A;
                    } else {
                        XA = WY;
                        XI = WX;
                    }
                    let XB = (VC - XA) - VF;
                    let XC = (BN * VC) * VF;
                    let XD = if XC > A { 1.0 } else { 0.0 };
                    let XF = if XD != 0.0 {
                        XC
                    } else {
                        let XE = -XC;
                        XE
                    };
                    let XG = ((XB * XB) + XF).sqrt();
                    let XH = VC - (K * (XB + XG));
                    let XL = ((((XH * XH) / BH) / CK) / EF) / IA;
                    let XM = WI - (((((-WI) + (WU / CR)) - VM) + XL) / ((-1e0f64 + (XJ / CR)) + (((BH * XL) * (XI * (XJ * (K * (B + (XB / XG)))))) / XH)));
                    let XN = if ((XM - WI).abs()) < PJ { 1.0 } else { 0.0 };
                    let XO = if XN != 0.0 {
                        N
                    } else {
                        WG
                    };
                    let XP = XO + B;
                    WG = XP;
                    WI = XM;
                    XR = XL;
                    AAB = WU;
                }
                let XS = if (((1.2919089961638799e9f64 * XR) / IA).sqrt()) > (9.9e-1f64 * J) { 1.0 } else { 0.0 };
                let AAX;
                let AFW;
                if XS != 0.0 {
                    let XT = B / TM;
                    let XU = J / CK;
                    let XV = B / CR;
                    let XW = B / ((XT + XU) + XV);
                    let XX = (XT * (XW * (VN + ((XV + (K * XU)) * VC)))) / (B - (XW * XT));
                    let XY = UQ + XX;
                    AAX = XX;
                    AFW = XY;
                } else {
                    AAX = A;
                    AFW = UQ;
                }
                let XZ = PF / BI;
                let YA = BI / (B + (XZ * (5e-1f64 + (XZ * (1.6666666666666666e-1f64 + (XZ * (4.1666666666666664e-2f64 + (XZ * (8.333333333333333e-3f64 + (XZ * (1.388888888888889e-3f64 + (XZ * 1.984126984126984e-4f64))))))))))));
                let YB = if YA < PJ { 1.0 } else { 0.0 };
                let YC = if YB != 0.0 {
                    PJ
                } else {
                    YA
                };
                let YD = (UZ / (1.5e0f64 * NA)) * ((((OZ + YC) - ES) + TQ) - UN);
                let YE = J * UT;
                let YF = if (if YD < YE { 1.0 } else { 0.0 }) != 0.0 && (if YE >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ZB;
                if YF != 0.0 {
                    let YG = YE - YD;
                    let YH = YG * YG;
                    let YI = YE * YE;
                    let YJ = (YH * YH) + (YI * YI);
                    let YZ;
                    if YK != 0.0 {
                        let YU;
                        if YL != 0.0 {
                            YU = B;
                        } else {
                            let YV;
                            if YM != 0.0 {
                                YV = BH;
                            } else {
                                let YW;
                                if YN != 0.0 {
                                    YW = BT;
                                } else {
                                    let YX = if YO != 0.0 {
                                        BN
                                    } else {
                                        A
                                    };
                                    YW = YX;
                                }
                                YV = YW;
                            }
                            YU = YV;
                        }
                        let mut YP = 0.0;
                        let mut YR = 0.0;
                        YP = A;
                        YR = YJ;
                        loop {
                            let YQ = if YP < YU { 1.0 } else { 0.0 };
                            if YQ == 0.0 {
                                break;
                            }
                            let YS = YR.sqrt();
                            let YT = YP + B;
                            YP = YT;
                            YR = YS;
                        }
                        YZ = YR;
                    } else {
                        let YY = YJ.powf(2.5e-1f64);
                        YZ = YY;
                    }
                    let ZA = YE - ((YG * YE) * (B / YZ));
                    ZB = ZA;
                } else {
                    ZB = YD;
                }
                let ZC = UZ - J;
                let ZD = if (if ZB > ZC { 1.0 } else { 0.0 }) != 0.0 && (if J >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ZZ;
                if ZD != 0.0 {
                    let ZE = (ZB - UZ) + J;
                    let ZF = ZE * ZE;
                    let ZG = J * J;
                    let ZH = (ZF * ZF) + (ZG * ZG);
                    let ZX;
                    if ZI != 0.0 {
                        let ZS;
                        if ZJ != 0.0 {
                            ZS = B;
                        } else {
                            let ZT;
                            if ZK != 0.0 {
                                ZT = BH;
                            } else {
                                let ZU;
                                if ZL != 0.0 {
                                    ZU = BT;
                                } else {
                                    let ZV = if ZM != 0.0 {
                                        BN
                                    } else {
                                        A
                                    };
                                    ZU = ZV;
                                }
                                ZT = ZU;
                            }
                            ZS = ZT;
                        }
                        let mut ZN = 0.0;
                        let mut ZP = 0.0;
                        ZN = A;
                        ZP = ZH;
                        loop {
                            let ZO = if ZN < ZS { 1.0 } else { 0.0 };
                            if ZO == 0.0 {
                                break;
                            }
                            let ZQ = ZP.sqrt();
                            let ZR = ZN + B;
                            ZN = ZR;
                            ZP = ZQ;
                        }
                        ZX = ZP;
                    } else {
                        let ZW = ZH.powf(2.5e-1f64);
                        ZX = ZW;
                    }
                    let ZY = ZC + ((ZE * J) * (B / ZX));
                    ZZ = ZY;
                } else {
                    ZZ = ZB;
                }
                let AAA = (-ZZ) * IF;
                let AAC = ((((VC * J) / BH) / CK) + LN) - ((AAB * J) / CK);
                let ALF;
                let ALG;
                let ALH;
                let ATA;
                let ATJ;
                let AVC;
                let BHT;
                let CLP;
                if AAD != 0.0 {
                    let AAE = if A < AAC { 1.0 } else { 0.0 };
                    let AAF = if AAE != 0.0 {
                        B
                    } else {
                        BH
                    };
                    ALF = A;
                    ALG = A;
                    ALH = A;
                    ATA = AAF;
                    ATJ = A;
                    AVC = A;
                    BHT = A;
                    CLP = A;
                } else {
                    let AAG = B + ((BN * ((LL * UO) - B)) / (US * LM));
                    let AAH = if AAG >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let AAJ = if AAH != 0.0 {
                        AAG
                    } else {
                        AAI
                    };
                    let AAK = UO + (((US * LL) * K) * (B - (AAJ.sqrt())));
                    let AAL = if (LL * AAK) < BT { 1.0 } else { 0.0 };
                    let ABK;
                    if AAL != 0.0 {
                        let AAM = B / ((1.3094570021973102e-2f64 * LL) * UR);
                        let AAO = AAN + (BT * AAM);
                        let AAP = (TY * AAM) * (LL * (UO - PD));
                        let AAS = (AAQ - (AAN * (AAR + AAM))) + AAP;
                        let AAU = (((-2.916e3f64 - (AAN * AAM)) + AAP) + (((((BN * AAO) * AAO) * AAO) + (AAS * AAS)).sqrt())).powf(AAT);
                        let AAW = (((BT - ((AAV * AAO) / (BT * AAU))) + (2.6456684199469993e-1f64 * AAU)) * LN) + PD;
                        ABK = AAW;
                    } else {
                        let AAY = if (OZ - AAX) <= TR { 1.0 } else { 0.0 };
                        let ABL;
                        if AAY != 0.0 {
                            let AAZ = J / CK;
                            let ABA = B / CR;
                            let ABB = UO - (((B / (((B / TM) + AAZ) + ABA)) * ((UO - VM) + ((ABA + (K * AAZ)) * (-AAA)))) / TM);
                            ABL = ABB;
                        } else {
                            let ABC = UO - AAX;
                            let ABD = (((UW * ABC) * ABC).ln()) / (LL + (BH / ABC));
                            let ABE = (ABD - AAK) - VZ;
                            let ABF = (BN * ABD) * VZ;
                            let ABG = if ABF > A { 1.0 } else { 0.0 };
                            let ABI = if ABG != 0.0 {
                                ABF
                            } else {
                                let ABH = -ABF;
                                ABH
                            };
                            let ABJ = ABD - (K * (ABE + (((ABE * ABE) + ABI).sqrt())));
                            ABL = ABJ;
                        }
                        ABK = ABL;
                    }
                    let ABM = if ABK > A { 1.0 } else { 0.0 };
                    let ABO = if ABM != 0.0 {
                        let ABN = ((1.2919089961638799e9f64 * ABK) / IA).sqrt();
                        ABN
                    } else {
                        A
                    };
                    let ABP = if ABO < J { 1.0 } else { 0.0 };
                    let ATB = if ABP != 0.0 {
                        B
                    } else {
                        BH
                    };
                    let ABQ = if (OZ - AAX) <= TR { 1.0 } else { 0.0 };
                    let ACX;
                    let ADA;
                    if ABQ != 0.0 {
                        let ABR = J / CK;
                        let ABS = B / CR;
                        let ABT = UO - (((B / (((B / TM) + ABR) + ABS)) * ((UO - VM) + ((ABS + (K * ABR)) * (-AAA)))) / TM);
                        ACX = ABT;
                        ADA = ABT;
                    } else {
                        let ABU = J / CK;
                        let ABV = B / CR;
                        let ABW = UO - (((B / (((B / TM) + ABU) + ABV)) * ((UO - VM) + ((ABV + (K * ABU)) * (-AAA)))) / TM);
                        let ABX = UO - AAX;
                        let ABY = if ABX > A { 1.0 } else { 0.0 };
                        let ACY;
                        if ABY != 0.0 {
                            let ACA = ((((UW * ABX) * ABX).ln()) / (LL + (BH / ABX))) * ABZ;
                            let ACB = ACA - MA;
                            let ACC = if (if ABW > ACB { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                            let ACZ;
                            if ACC != 0.0 {
                                let ACD = (ABW - ACA) + MA;
                                let ACE = ACD * ACD;
                                let ACF = (ACE * ACE) + 2.560000000000001e-2f64;
                                let ACV;
                                if ACG != 0.0 {
                                    let ACQ;
                                    if ACH != 0.0 {
                                        ACQ = B;
                                    } else {
                                        let ACR;
                                        if ACI != 0.0 {
                                            ACR = BH;
                                        } else {
                                            let ACS;
                                            if ACJ != 0.0 {
                                                ACS = BT;
                                            } else {
                                                let ACT = if ACK != 0.0 {
                                                    BN
                                                } else {
                                                    A
                                                };
                                                ACS = ACT;
                                            }
                                            ACR = ACS;
                                        }
                                        ACQ = ACR;
                                    }
                                    let mut ACL = 0.0;
                                    let mut ACN = 0.0;
                                    ACL = A;
                                    ACN = ACF;
                                    loop {
                                        let ACM = if ACL < ACQ { 1.0 } else { 0.0 };
                                        if ACM == 0.0 {
                                            break;
                                        }
                                        let ACO = ACN.sqrt();
                                        let ACP = ACL + B;
                                        ACL = ACP;
                                        ACN = ACO;
                                    }
                                    ACV = ACN;
                                } else {
                                    let ACU = ACF.powf(2.5e-1f64);
                                    ACV = ACU;
                                }
                                let ACW = ACB + ((ACD * MA) * (B / ACV));
                                ACZ = ACW;
                            } else {
                                ACZ = ABW;
                            }
                            ACY = ACZ;
                        } else {
                            ACY = ABW;
                        }
                        ACX = ACY;
                        ADA = ABW;
                    }
                    let ADB = K * VB;
                    let ADC = (ACX + (ADB * CM)) - VM;
                    let ADD = if ADC < A { 1.0 } else { 0.0 };
                    let AFQ;
                    if ADD != 0.0 {
                        let ADE = VK * CT;
                        let ADF = ADE * ADE;
                        let ADH = (-1.6e0f64 * ADC) + ADG;
                        let ADI = ADH * IO;
                        let ADJ = (ADH - K) - ADI;
                        let ADK = (BN * ADH) * ADI;
                        let ADL = if ADK > A { 1.0 } else { 0.0 };
                        let ADN = if ADL != 0.0 {
                            ADK
                        } else {
                            let ADM = -ADK;
                            ADM
                        };
                        let ADO = (ADF * (ADH - (K * (ADJ + (((ADJ * ADJ) + ADN).sqrt()))))) * LM;
                        let ADP = (ADC * (B - (ADO.sqrt()))) / (B - ADO);
                        AFQ = ADP;
                    } else {
                        let ADQ = -((VM - ACX) - (((VB / BH) * J) / CK));
                        let ADR = (BH * ADQ) + VO;
                        let ADS = ADQ * ADQ;
                        let ADT = (ADR * ADR) - (BN * (ADS + VL));
                        let ADU = if ADT >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let ADW = if ADU != 0.0 {
                            ADT
                        } else {
                            ADV
                        };
                        let ADX = (ADR - (ADW.sqrt())) / BH;
                        let ADY = (((ADS / VL) / VW).ln()) / (LL + (BH / ADQ));
                        let ADZ = if ADX < VJ { 1.0 } else { 0.0 };
                        let AFR;
                        if ADZ != 0.0 {
                            AFR = ADX;
                        } else {
                            let AEA = (ADY - ADX) - VZ;
                            let AEB = (BN * ADY) * VZ;
                            let AEC = if AEB > A { 1.0 } else { 0.0 };
                            let AEE = if AEC != 0.0 {
                                AEB
                            } else {
                                let AED = -AEB;
                                AED
                            };
                            let AEF = ADY - (K * (AEA + (((AEA * AEA) + AEE).sqrt())));
                            AFR = AEF;
                        }
                        AFQ = AFR;
                    }
                    let mut AEG = 0.0;
                    let mut AEI = 0.0;
                    let mut AFT = 0.0;
                    AEG = A;
                    AEI = AFQ;
                    AFT = A;
                    loop {
                        let AEH = if AEG < N { 1.0 } else { 0.0 };
                        if AEH == 0.0 {
                            break;
                        }
                        let AEJ = LL * AEI;
                        let AEK = (-AEJ).exp();
                        let AEL = if AEI > KM { 1.0 } else { 0.0 };
                        let AEU;
                        let AFJ;
                        if AEL != 0.0 {
                            let AEM = AEJ.exp();
                            let AEN = (-VK) * ((((AEK + AEJ) - B) + (VW * (AEM - B))).sqrt());
                            let AEO = (EI / AEN) * (((-AEK) + B) + (VW * AEM));
                            AEU = AEN;
                            AFJ = AEO;
                        } else {
                            let AEP = if AEI < -1e-9f64 { 1.0 } else { 0.0 };
                            let AEV;
                            let AFK;
                            if AEP != 0.0 {
                                let AEQ = VK * (((AEK + AEJ) - B).sqrt());
                                let AER = (EI / AEQ) * ((-AEK) + B);
                                AEV = AEQ;
                                AFK = AER;
                            } else {
                                let AES = ((-((EI / LL).sqrt())) * LL) * AEI;
                                let AET = -((EI * LL).sqrt());
                                AEV = AES;
                                AFK = AET;
                            }
                            AEU = AEV;
                            AFJ = AFK;
                        }
                        let AEW = ((AEU * AEU) + ((BN * VD) * VD)).sqrt();
                        let AEX = K * (B + (AEU / AEW));
                        let AEY = (K * (AEU + AEW)) + (IP * VD);
                        let AEZ = if AEY < A { 1.0 } else { 0.0 };
                        let AFA;
                        let AFI;
                        if AEZ != 0.0 {
                            AFA = A;
                            AFI = A;
                        } else {
                            AFA = AEY;
                            AFI = AEX;
                        }
                        let AFB = (VC - AFA) - VF;
                        let AFC = (BN * VC) * VF;
                        let AFD = if AFC > A { 1.0 } else { 0.0 };
                        let AFF = if AFD != 0.0 {
                            AFC
                        } else {
                            let AFE = -AFC;
                            AFE
                        };
                        let AFG = ((AFB * AFB) + AFF).sqrt();
                        let AFH = VC - (K * (AFB + AFG));
                        let AFL = ((((AFH * AFH) / BH) / CK) / EF) / IA;
                        let AFM = AEI - ((((((ACX - AEI) + (AEU / CR)) + (((AEU + (VB / BH)) * J) / CK)) - VM) + AFL) / (((-1e0f64 + (AFJ / CR)) + ((AFJ * J) / CK)) + (((BH * AFL) * (AFI * (AFJ * (K * (B + (AFB / AFG)))))) / AFH)));
                        let AFN = if ((AFM - AEI).abs()) < IO { 1.0 } else { 0.0 };
                        let AFO = if AFN != 0.0 {
                            N
                        } else {
                            AEG
                        };
                        let AFP = AFO + B;
                        AEG = AFP;
                        AEI = AFM;
                        AFT = AEU;
                    }
                    let AFS = VM + AEI;
                    let AFU = ACX + (CM * (ADB + AFT));
                    ALF = ACX;
                    ALG = AFU;
                    ALH = AFS;
                    ATA = ATB;
                    ATJ = AFT;
                    AVC = ADA;
                    BHT = ABO;
                    CLP = ACX;
                }
                let AFY = if (if AFV == B { 1.0 } else { 0.0 }) != 0.0 && (if OZ > (AFW + AFX) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ASL;
                let AVA;
                let CGG;
                let CGU;
                let CUQ;
                let CVS;
                if AFY != 0.0 {
                    let AFZ = ((PO - GB) + TQ) - UN;
                    let AGB = (((3.2043836e-19f64 * IA) * CK) / LL).sqrt();
                    let AGC = (MK / IA) / IA;
                    let AGD = ((AGB * AGB) / TM) / TM;
                    let AGE = (AGD * LL) / BH;
                    let AGF = ((((B / AGC) / AGD) * (AFZ * AFZ)).ln()) / (LL + (BH / AFZ));
                    let AGG = (AGF - (AFZ + (AGE * (B - ((B + ((BN * ((LL * AFZ) - B)) / ((AGE * LL) * BH))).sqrt()))))) - AGA;
                    let AGH = AGF - (K * (AGG + (((AGG * AGG) + ((BN * AGA) * AGF)).sqrt())));
                    let AGI = LL * AGH;
                    let AGJ = AGI - B;
                    let AGK = AGJ + (AGC * (AGI.exp()));
                    let AGL = if (if AGK > A { 1.0 } else { 0.0 }) != 0.0 && (if AGJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ASM;
                    let AVB;
                    let CUR;
                    let CVT;
                    if AGL != 0.0 {
                        let AGN = -LL;
                        let AGO = (((((BH * DQ) / LL) * AGM) * (AGB * ((AGK.sqrt()) - (AGJ.sqrt())))) * (-(((AGN * PN).exp()) - B))) * (B / CW);
                        let AGP = B + ((BN * ((LL * UO) - B)) / (US * LM));
                        let AGQ = if AGP < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let AGS = if AGQ != 0.0 {
                            AGR
                        } else {
                            AGP
                        };
                        let AGT = UO + (((US * LL) * K) * (B - (AGS.sqrt())));
                        let AGU = AGT - AGH;
                        let AGV = if AGU < A { 1.0 } else { 0.0 };
                        let AGW = if AGV != 0.0 {
                            A
                        } else {
                            AGU
                        };
                        let AGX = 1.3e0f64 * AGW;
                        let AGZ = (AGX - PN) - AGY;
                        let AHA = AGX - (K * (AGZ + (((AGZ * AGZ) + ((BN * AGX) * AGY)).sqrt())));
                        let AHB = if AHA > AGW { 1.0 } else { 0.0 };
                        let AHC = if AHB != 0.0 {
                            AGW
                        } else {
                            AHA
                        };
                        let AHD = CJ * AZ;
                        let AHE = DR * AZ;
                        let AHF = CW * AZ;
                        let AHG = if parameters[36] == A { 1.0 } else { 0.0 };
                        let AKY;
                        if AHG != 0.0 {
                            AKY = A;
                        } else {
                            let AHH = ((parameters[142] * EF) * AHE) * AHF;
                            let AHI = AHH / ME;
                            let AHJ = (-(((((parameters[145] * RR) + TI) + TN) + LK) + parameters[144])) / AHD;
                            let mut AHK = 0.0;
                            let mut AID = 0.0;
                            AHK = A;
                            AID = A;
                            loop {
                                let AHL = if AHK <= 9.9e1f64 { 1.0 } else { 0.0 };
                                if AHL == 0.0 {
                                    break;
                                }
                                let AHM = (UO + PL) - ((AHC * (AHK / AZ)) + AGH);
                                let AHN = B - (AHM / 4.12e0f64);
                                let AHO = AHJ + (AHM / AHD);
                                let AHP = AHO * AHO;
                                let AHQ = (K * (AHN + (((AHN * AHN) + 4e-6f64).sqrt()))) + 1e-13f64;
                                let AHR = if AHQ < A { 1.0 } else { 0.0 };
                                let AHS = if AHR != 0.0 {
                                    A
                                } else {
                                    AHQ
                                };
                                let AHT = parameters[143] * (B - ((AHS.sqrt()) * AHS));
                                let AHU = (-AHT) / AHO;
                                let AHV = if AHU < -3.4e1f64 { 1.0 } else { 0.0 };
                                let AIA = if AHV != 0.0 {
                                    A
                                } else {
                                    let AHW = AHU.exp();
                                    AHW
                                };
                                let AHY = (((AHX * AHI) * AHT) * AHT) * 7.38905609893065e0f64;
                                let AHZ = if ((BH * AHO) + AHT) < A { 1.0 } else { 0.0 };
                                let AIE;
                                if AHZ != 0.0 {
                                    AIE = AHY;
                                } else {
                                    let AIB = (AHH * AHP) * AIA;
                                    let AIC = if (if AIB < AHY { 1.0 } else { 0.0 }) != 0.0 || (if AHO < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AIF = if AIC != 0.0 {
                                        AHY
                                    } else {
                                        AIB
                                    };
                                    AIE = AIF;
                                }
                                let AIG = AID + AIE;
                                let AIH = if AIE < KM { 1.0 } else { 0.0 };
                                let AII = if AIH != 0.0 {
                                    AZ
                                } else {
                                    AHK
                                };
                                let AIJ = AII + B;
                                AHK = AIJ;
                                AID = AIG;
                            }
                            AKY = AID;
                        }
                        let AIK = if (if FJ <= A { 1.0 } else { 0.0 }) != 0.0 || (if P <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let AKX;
                        if AIK != 0.0 {
                            AKX = A;
                        } else {
                            let AKN;
                            if EZ != 0.0 {
                                let AIL = TM * TM;
                                let AIM = IG / AIL;
                                let AIO = B + (((BH / IG) * AIL) * ((AFZ - LN) - (AIN * RR)));
                                let AIP = (K * (AIO + (((AIO * AIO) + 4e-6f64).sqrt()))) + 1e-13f64;
                                let AIQ = if AIP < A { 1.0 } else { 0.0 };
                                let AIR = if AIQ != 0.0 {
                                    A
                                } else {
                                    AIP
                                };
                                let AIY = ((AIU * PN) + AGH) - ((AIV * AIW) * ((AFZ * AIS) + (AIM * (B - ((AIR + GF).sqrt())))));
                                let AIZ = (K * (AIY + (((AIY * AIY) + 4e-4f64).sqrt()))) + 1e-12f64;
                                let AJA = if AIZ < A { 1.0 } else { 0.0 };
                                let AKO = if AJA != 0.0 {
                                    A
                                } else {
                                    AIZ
                                };
                                AKN = AKO;
                            } else {
                                let AJD = AJB * AFZ;
                                let AJE = TM * TM;
                                let AJF = IG / AJE;
                                let AJG = (BH / IG) * AJE;
                                let AJH = B + (AJG * ((AJD - LN) - (AIN * RR)));
                                let AJI = BH * (B + AJG);
                                let AJJ = GF + AJI;
                                let AJK = if (if AJH < AJJ { 1.0 } else { 0.0 }) != 0.0 && (if AJI >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AKG;
                                if AJK != 0.0 {
                                    let AJL = AJJ - AJH;
                                    let AJM = AJL * AJL;
                                    let AJN = AJI * AJI;
                                    let AJO = (((AJM * AJM) * AJM) * AJM) + (((AJN * AJN) * AJN) * AJN);
                                    let AKE;
                                    if AJP != 0.0 {
                                        let AJZ;
                                        if AJQ != 0.0 {
                                            AJZ = B;
                                        } else {
                                            let AKA;
                                            if AJR != 0.0 {
                                                AKA = BH;
                                            } else {
                                                let AKB;
                                                if AJS != 0.0 {
                                                    AKB = BT;
                                                } else {
                                                    let AKC = if AJT != 0.0 {
                                                        BN
                                                    } else {
                                                        A
                                                    };
                                                    AKB = AKC;
                                                }
                                                AKA = AKB;
                                            }
                                            AJZ = AKA;
                                        }
                                        let mut AJU = 0.0;
                                        let mut AJW = 0.0;
                                        AJU = A;
                                        AJW = AJO;
                                        loop {
                                            let AJV = if AJU < AJZ { 1.0 } else { 0.0 };
                                            if AJV == 0.0 {
                                                break;
                                            }
                                            let AJX = AJW.sqrt();
                                            let AJY = AJU + B;
                                            AJU = AJY;
                                            AJW = AJX;
                                        }
                                        AKE = AJW;
                                    } else {
                                        let AKD = AJO.powf(1.25e-1f64);
                                        AKE = AKD;
                                    }
                                    let AKF = AJJ - ((AJL * AJI) * (B / AKE));
                                    AKG = AKF;
                                } else {
                                    AKG = AJH;
                                }
                                let AKH = if AKG <= A { 1.0 } else { 0.0 };
                                let AKJ = if AKH != 0.0 {
                                    A
                                } else {
                                    let AKI = AKG.sqrt();
                                    AKI
                                };
                                let AKK = ((AIU * PN) + B) - ((DA / (AIV + DA)) * (AJD + (AJF * (B - AKJ))));
                                let AKL = (K * (AKK + (((AKK * AKK) + 4e-6f64).sqrt()))) + 1e-13f64;
                                let AKM = if AKL < A { 1.0 } else { 0.0 };
                                let AKP = if AKM != 0.0 {
                                    A
                                } else {
                                    AKL
                                };
                                AKN = AKP;
                            }
                            let AKQ = AKN + GF;
                            let AKT = ((AKS * AKQ) * AGO) * (((-AKR) / AKQ).exp());
                            AKX = AKT;
                        }
                        let AKV = if AKU == B { 1.0 } else { 0.0 };
                        let ASN;
                        if AKV != 0.0 {
                            let ALA = AGH - ((AKZ * LN) * ((B + ((AKX + AKY) * (2.1633307652783932e-2f64 / ((((EF * J) * DR) * ((AGN * AKW).exp())) * (4.1046315303568966e26f64 + (2.4665765749313358e0f64 * IA)))))).ln()));
                            let ALB = (-(((3.3163543761348e-29f64 * IA) * LN).sqrt())) * ((((((AGN * ALA).exp()) - B) + (LL * ALA)).sqrt()) - (((((AGN * AGH).exp()) - B) + AGI).sqrt()));
                            let ASO = if ALC != 0.0 {
                                let ALE = 1e-5f64 * ALD;
                                ALE
                            } else {
                                ALB
                            };
                            ASN = ASO;
                        } else {
                            ASN = A;
                        }
                        ASM = ASN;
                        AVB = AGT;
                        CUR = AKX;
                        CVT = AGM;
                    } else {
                        ASM = A;
                        AVB = AVC;
                        CUR = A;
                        CVT = A;
                    }
                    ASL = ASM;
                    AVA = AVB;
                    CGG = AGC;
                    CGU = AGB;
                    CUQ = CUR;
                    CVS = CVT;
                } else {
                    ASL = A;
                    AVA = AVC;
                    CGG = ML;
                    CGU = MI;
                    CUQ = A;
                    CVS = A;
                }
                let mut ALI = 0.0;
                let mut ALK = 0.0;
                let mut ALY = 0.0;
                let mut AMF = 0.0;
                let mut APZ = 0.0;
                let mut ASP = 0.0;
                let mut ASU = 0.0;
                let mut ATC = 0.0;
                let mut ATD = 0.0;
                let mut ATI = 0.0;
                ALI = B;
                ALK = ALH;
                ALY = ALF;
                AMF = ALG;
                APZ = A;
                ASP = A;
                ASU = A;
                ATC = A;
                ATD = A;
                ATI = ATJ;
                loop {
                    let ALJ = if ALI <= N { 1.0 } else { 0.0 };
                    if ALJ == 0.0 {
                        break;
                    }
                    let ALL = ALK - VM;
                    let ALM = LL * ALL;
                    let ALN = (-ALM).exp();
                    let ALO = if ALL < -1e-9f64 { 1.0 } else { 0.0 };
                    let AQB;
                    let AQH;
                    if ALO != 0.0 {
                        let ALP = VK * (((ALN + ALM) - B).sqrt());
                        let ALQ = (EI * ((-ALN) + B)) / ALP;
                        AQB = ALP;
                        AQH = ALQ;
                    } else {
                        let ALR = if ALL > KM { 1.0 } else { 0.0 };
                        let AQC;
                        let AQI;
                        if ALR != 0.0 {
                            let ALS = ALM.exp();
                            let ALT = (-VK) * ((((ALN + ALM) - B) + (VW * ((ALS + ALM) - B))).sqrt());
                            let ALU = (EI * (((-ALN) + B) + (VW * (ALS + B)))) / ALT;
                            AQC = ALT;
                            AQI = ALU;
                        } else {
                            let ALV = -VK;
                            let ALW = ALV * ALM;
                            let ALX = ALV * LL;
                            AQC = ALW;
                            AQI = ALX;
                        }
                        AQB = AQC;
                        AQH = AQI;
                    }
                    let ALZ = LL * ALY;
                    let AMA = ALZ.exp();
                    let AMB = (((AAA * AAA) / (MQ * MQ)) + ((BH * MX) * ((AMA + ALZ) - B))).sqrt();
                    let AMC = -MQ;
                    let AMD = (AMC * AMB) - AAA;
                    let AME = AMC * ((((BH * LL) * MX) * (AMA + B)) / (BH * AMB));
                    let AMG = (AMF - ALY) / UT;
                    let AMH = LL * AMG;
                    let AMI = -AMH;
                    let AMK = if AMI >= AMJ { 1.0 } else { 0.0 };
                    let AMX;
                    if AMK != 0.0 {
                        AMX = AML;
                    } else {
                        let mut AMM = 0.0;
                        let mut AMP = 0.0;
                        AMM = AMI;
                        AMP = B;
                        loop {
                            let AMO = if AMM >= AMN { 1.0 } else { 0.0 };
                            if AMO == 0.0 {
                                break;
                            }
                            let AMR = AMP * AMQ;
                            let AMS = AMM - AMN;
                            AMM = AMS;
                            AMP = AMR;
                        }
                        let AMT = AMP * (AMM.exp());
                        AMX = AMT;
                    }
                    let AMU = (((AMI.exp()) + AMH) - B).sqrt();
                    let AMV = if AMG < -1e-9f64 { 1.0 } else { 0.0 };
                    let ANH;
                    let AOK;
                    let AOO;
                    if AMV != 0.0 {
                        let AMW = MQ * AMU;
                        let AMY = (((MQ * LL) * ((-AMX) + B)) / (BH * AMU)) / UT;
                        let AMZ = -AMY;
                        ANH = AMW;
                        AOK = AMY;
                        AOO = AMZ;
                    } else {
                        let ANA = if AMG > KM { 1.0 } else { 0.0 };
                        let ANI;
                        let AOL;
                        let AOP;
                        if ANA != 0.0 {
                            let ANB = AMC * AMU;
                            let ANC = (((AMC * LL) * ((-AMX) + B)) / (BH * AMU)) / UT;
                            let AND = -ANC;
                            ANI = ANB;
                            AOL = ANC;
                            AOP = AND;
                        } else {
                            let ANE = (AMC * AMH) / MP;
                            let ANF = (AMC * LL) / MP;
                            let ANG = -ANF;
                            ANI = ANE;
                            AOL = ANF;
                            AOP = ANG;
                        }
                        ANH = ANI;
                        AOK = AOL;
                        AOO = AOP;
                    }
                    let ANJ = -VA;
                    let ANK = A - ANJ;
                    let ANL = if (if ANH > ANK { 1.0 } else { 0.0 }) != 0.0 && (if ANJ >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AOM;
                    let AOR;
                    if ANL != 0.0 {
                        let ANM = ANH + ANJ;
                        let ANN = ANM * ANM;
                        let ANO = ANJ * ANJ;
                        let ANP = ANO * ANO;
                        let ANQ = (ANN * ANN) + ANP;
                        let AOG;
                        if ANR != 0.0 {
                            let AOB;
                            if ANS != 0.0 {
                                AOB = B;
                            } else {
                                let AOC;
                                if ANT != 0.0 {
                                    AOC = BH;
                                } else {
                                    let AOD;
                                    if ANU != 0.0 {
                                        AOD = BT;
                                    } else {
                                        let AOE = if ANV != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        AOD = AOE;
                                    }
                                    AOC = AOD;
                                }
                                AOB = AOC;
                            }
                            let mut ANW = 0.0;
                            let mut ANY = 0.0;
                            ANW = A;
                            ANY = ANQ;
                            loop {
                                let ANX = if ANW < AOB { 1.0 } else { 0.0 };
                                if ANX == 0.0 {
                                    break;
                                }
                                let ANZ = ANY.sqrt();
                                let AOA = ANW + B;
                                ANW = AOA;
                                ANY = ANZ;
                            }
                            AOG = ANY;
                        } else {
                            let AOF = ANQ.powf(2.5e-1f64);
                            AOG = AOF;
                        }
                        let AOH = B / AOG;
                        let AOI = ((ANJ * ANP) * AOH) / ANQ;
                        let AOJ = ANK + ((ANM * ANJ) * AOH);
                        AOM = AOI;
                        AOR = AOJ;
                    } else {
                        AOM = B;
                        AOR = ANH;
                    }
                    let AON = AOK * AOM;
                    let AOQ = AOO * AOM;
                    let AOS = VB - AAA;
                    let AOT = -AOS;
                    let AOU = AOS + AOT;
                    let AOV = if (if AOR < AOU { 1.0 } else { 0.0 }) != 0.0 && (if AOT >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let APU;
                    let APX;
                    if AOV != 0.0 {
                        let AOW = AOU - AOR;
                        let AOX = AOW * AOW;
                        let AOY = AOT * AOT;
                        let AOZ = AOY * AOY;
                        let APA = (AOX * AOX) + AOZ;
                        let APQ;
                        if APB != 0.0 {
                            let APL;
                            if APC != 0.0 {
                                APL = B;
                            } else {
                                let APM;
                                if APD != 0.0 {
                                    APM = BH;
                                } else {
                                    let APN;
                                    if APE != 0.0 {
                                        APN = BT;
                                    } else {
                                        let APO = if APF != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        APN = APO;
                                    }
                                    APM = APN;
                                }
                                APL = APM;
                            }
                            let mut APG = 0.0;
                            let mut API = 0.0;
                            APG = A;
                            API = APA;
                            loop {
                                let APH = if APG < APL { 1.0 } else { 0.0 };
                                if APH == 0.0 {
                                    break;
                                }
                                let APJ = API.sqrt();
                                let APK = APG + B;
                                APG = APK;
                                API = APJ;
                            }
                            APQ = API;
                        } else {
                            let APP = APA.powf(2.5e-1f64);
                            APQ = APP;
                        }
                        let APR = B / APQ;
                        let APS = ((AOT * AOZ) * APR) / APA;
                        let APT = AOU - ((AOW * AOT) * APR);
                        APU = APS;
                        APX = APT;
                    } else {
                        APU = B;
                        APX = AOR;
                    }
                    let APV = AOQ * APU;
                    let APW = AON * APU;
                    let APY = AAA + APX;
                    let AQA = if APZ == B { 1.0 } else { 0.0 };
                    let ASE;
                    let ASG;
                    let ASH;
                    let ASI;
                    let ASJ;
                    let ASQ;
                    if AQA != 0.0 {
                        ASE = N;
                        ASG = ALK;
                        ASH = ALY;
                        ASI = AMF;
                        ASJ = APZ;
                        ASQ = ALI;
                    } else {
                        let AQD = (ALY - UO) - (SR * ((((AQB + AAA) + AMD) + APX) + ASL));
                        let AQE = B - (SR * (AME + APV));
                        let AQF = -SR;
                        let AQG = AQF * APW;
                        let AQJ = AQF * AQH;
                        let AQK = AMF - (ALY + (CM * ((K * VB) + AQB)));
                        let AQM = -(CM * AQH);
                        let AQN = (ALK - AMF) - (CS * AQB);
                        let AQP = B - (CS * AQH);
                        let AQQ = AQE * AQP;
                        let AQR = AQE * AQM;
                        let AQS = AQG * AQL;
                        let AQT = AQJ * AQL;
                        let AQU = -(B / ((((AQQ - (AQR * AQO)) - (AQS * AQP)) + (AQT * AQO)) + GF));
                        let AQV = AQU * ((((AQP - (AQM * AQO)) * AQD) + (((AQJ * AQO) - (AQG * AQP)) * AQK)) + (((AQG * AQM) - AQJ) * AQN));
                        let AQW = AQU * (((AQP * AQD) + (AQQ * AQK)) + ((AQT - AQR) * AQN));
                        let AQX = AQU * ((AQD + (((-AQE) * AQO) * AQK)) + ((AQE - AQS) * AQN));
                        let AQY = AQV.abs();
                        let AQZ = AQW.abs();
                        let ARA = if AQY < AQZ { 1.0 } else { 0.0 };
                        let ARB = if ARA != 0.0 {
                            AQZ
                        } else {
                            AQY
                        };
                        let ARC = AQX.abs();
                        let ARD = if ARB < ARC { 1.0 } else { 0.0 };
                        let ARM = if ARD != 0.0 {
                            ARC
                        } else {
                            ARB
                        };
                        let ARF = if ALI > ARE { 1.0 } else { 0.0 };
                        let ARN;
                        if ARF != 0.0 {
                            ARN = ARG;
                        } else {
                            let ARI = if ALI > ARH { 1.0 } else { 0.0 };
                            let ARO;
                            if ARI != 0.0 {
                                ARO = ARG;
                            } else {
                                let ARJ = if ALI > OR { 1.0 } else { 0.0 };
                                let ARP;
                                if ARJ != 0.0 {
                                    ARP = ARK;
                                } else {
                                    let ARL = if ALI > L { 1.0 } else { 0.0 };
                                    let ARQ = if ARL != 0.0 {
                                        LA
                                    } else {
                                        B
                                    };
                                    ARP = ARQ;
                                }
                                ARO = ARP;
                            }
                            ARN = ARO;
                        }
                        let ARR = BI / ARN;
                        let ARS = if ARM > ARR { 1.0 } else { 0.0 };
                        let ARX;
                        let ARZ;
                        let ASB;
                        if ARS != 0.0 {
                            let ART = ARR / ARM;
                            let ARU = AQV * ART;
                            let ARV = AQW * ART;
                            let ARW = AQX * ART;
                            ARX = ARU;
                            ARZ = ARV;
                            ASB = ARW;
                        } else {
                            ARX = AQV;
                            ARZ = AQW;
                            ASB = AQX;
                        }
                        let ARY = ALY + ARX;
                        let ASA = AMF + ARZ;
                        let ASC = ALK + ASB;
                        let ASD = if ARM < (PJ * ARN) { 1.0 } else { 0.0 };
                        let ASK = if ASD != 0.0 {
                            B
                        } else {
                            APZ
                        };
                        ASE = ALI;
                        ASG = ASC;
                        ASH = ARY;
                        ASI = ASA;
                        ASJ = ASK;
                        ASQ = ASP;
                    }
                    let ASF = ASE + B;
                    ALI = ASF;
                    ALK = ASG;
                    ALY = ASH;
                    AMF = ASI;
                    APZ = ASJ;
                    ASP = ASQ;
                    ASU = AMD;
                    ATC = APX;
                    ATD = APY;
                    ATI = AQB;
                }
                let ASR = if ASP > A { 1.0 } else { 0.0 };
                if ASR != 0.0 {
                } else {
                }
                let ASS = if APZ == A { 1.0 } else { 0.0 };
                let AST;
                let ATL;
                let ATM;
                if ASS != 0.0 {
                    AST = ALF;
                    ATL = ALG;
                    ATM = ALH;
                } else {
                    AST = ALY;
                    ATL = AMF;
                    ATM = ALK;
                }
                let ASV = -ASU;
                let ASW = if ASV <= GF { 1.0 } else { 0.0 };
                let ASX = if ASW != 0.0 {
                    GF
                } else {
                    ASV
                };
                let ASY = ASX * SR;
                let ASZ = if (if AST <= A { 1.0 } else { 0.0 }) != 0.0 && A != 0.0 { 1.0 } else { 0.0 };
                let BLB;
                let BLH;
                let CAA;
                let CAD;
                let CAG;
                let CAL;
                let CAS;
                let CBM;
                let CCF;
                let CCM;
                let CCV;
                let CCY;
                let CHY;
                let CVH;
                let EDZ;
                let EGL;
                let EGQ;
                let EGU;
                let EGY;
                if ASZ != 0.0 {
                    let ATE = -5e-1f64 * ((AAA + ATC) + ATD);
                    let ATF = ((-DT) * CX) * ATE;
                    let ATG = ATF * K;
                    let ATH = ATF * 5e-1f64;
                    let ATK = (ATI * CX) * DT;
                    BLB = ATA;
                    BLH = A;
                    CAA = A;
                    CAD = A;
                    CAG = A;
                    CAL = B;
                    CAS = AST;
                    CBM = A;
                    CCF = ATE;
                    CCM = A;
                    CCV = ATI;
                    CCY = A;
                    CHY = A;
                    CVH = ATL;
                    EDZ = AST;
                    EGL = ATF;
                    EGQ = ATK;
                    EGU = ATG;
                    EGY = ATH;
                } else {
                    let ATN = IG / (TM * TM);
                    let ATO = BH / ATN;
                    let ATP = B + (ATO * (UO - GF));
                    let ATQ = B + ATO;
                    let ATR = if (if ATP < ATQ { 1.0 } else { 0.0 }) != 0.0 && (if ATQ >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AUN;
                    if ATR != 0.0 {
                        let ATS = ATQ - ATP;
                        let ATT = ATS * ATS;
                        let ATU = ATQ * ATQ;
                        let ATV = (((ATT * ATT) * ATT) * ATT) + (((ATU * ATU) * ATU) * ATU);
                        let AUL;
                        if ATW != 0.0 {
                            let AUG;
                            if ATX != 0.0 {
                                AUG = B;
                            } else {
                                let AUH;
                                if ATY != 0.0 {
                                    AUH = BH;
                                } else {
                                    let AUI;
                                    if ATZ != 0.0 {
                                        AUI = BT;
                                    } else {
                                        let AUJ = if AUA != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        AUI = AUJ;
                                    }
                                    AUH = AUI;
                                }
                                AUG = AUH;
                            }
                            let mut AUB = 0.0;
                            let mut AUD = 0.0;
                            AUB = A;
                            AUD = ATV;
                            loop {
                                let AUC = if AUB < AUG { 1.0 } else { 0.0 };
                                if AUC == 0.0 {
                                    break;
                                }
                                let AUE = AUD.sqrt();
                                let AUF = AUB + B;
                                AUB = AUF;
                                AUD = AUE;
                            }
                            AUL = AUD;
                        } else {
                            let AUK = ATV.powf(1.25e-1f64);
                            AUL = AUK;
                        }
                        let AUM = ATQ - ((ATS * ATQ) * (B / AUL));
                        AUN = AUM;
                    } else {
                        AUN = ATP;
                    }
                    let AUO = UO + (ATN * (B - (AUN.sqrt())));
                    let AUP = (K * (AUO + (((AUO * AUO) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let AUQ = if AUP < A { 1.0 } else { 0.0 };
                    let AUR = if AUQ != 0.0 {
                        A
                    } else {
                        AUP
                    };
                    let AUS = OT / AUR;
                    let AUU = B + ((AUS.powf((AUT - B))) * AUS);
                    let AUV = OT / ((AUU.powf(((B / AUT) - B))) * AUU);
                    let AUW = if AUV < A { 1.0 } else { 0.0 };
                    let BAE;
                    let BAJ;
                    let BAN;
                    let BHS;
                    let BIG;
                    let BLC;
                    if AUW != 0.0 {
                        BAE = ATL;
                        BAJ = AST;
                        BAN = ATM;
                        BHS = BHT;
                        BIG = A;
                        BLC = ATA;
                    } else {
                        let BAF;
                        let BAK;
                        let BAO;
                        let BHU;
                        let BIH;
                        let BLD;
                        if AUX != 0.0 {
                            let AUY = if A < AAC { 1.0 } else { 0.0 };
                            let AUZ = if AUY != 0.0 {
                                B
                            } else {
                                BH
                            };
                            BAF = A;
                            BAK = A;
                            BAO = A;
                            BHU = BHT;
                            BIH = A;
                            BLD = AUZ;
                        } else {
                            let AVD = AVA - AST;
                            let AVE = if AVD >= A { 1.0 } else { 0.0 };
                            let AVF = if AVE != 0.0 {
                                AVD
                            } else {
                                A
                            };
                            let AVG = ((1.3e0f64 * AVF) - AUV) - AGY;
                            let AVH = (BN * (1.3e0f64 * AVF)) * AGY;
                            let AVI = if AVH > A { 1.0 } else { 0.0 };
                            let AVK = if AVI != 0.0 {
                                AVH
                            } else {
                                let AVJ = -AVH;
                                AVJ
                            };
                            let AVL = (1.3e0f64 * AVF) - (K * (AVG + (((AVG * AVG) + AVK).sqrt())));
                            let AVM = if AVL <= AVF { 1.0 } else { 0.0 };
                            let AVN = if AVM != 0.0 {
                                AVL
                            } else {
                                AVF
                            };
                            let AVO = if AVN < A { 1.0 } else { 0.0 };
                            let AVQ;
                            if AVO != 0.0 {
                                AVQ = A;
                            } else {
                                let AVP = if AVN > AUV { 1.0 } else { 0.0 };
                                let AVR = if AVP != 0.0 {
                                    AUV
                                } else {
                                    AVN
                                };
                                AVQ = AVR;
                            }
                            let AVS = AST + AVQ;
                            let AVT = if AVS < AAC { 1.0 } else { 0.0 };
                            let AYI;
                            if AVT != 0.0 {
                                let AVU = if VR >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                                let AVW = if AVU != 0.0 {
                                    VR
                                } else {
                                    AVV
                                };
                                let AVX = (VP - (AVW.sqrt())) / BH;
                                let AVY = if AVX < VJ { 1.0 } else { 0.0 };
                                let AYJ;
                                if AVY != 0.0 {
                                    AYJ = AVX;
                                } else {
                                    let AVZ = (VX - AVX) - VZ;
                                    let AWA = (BN * VX) * VZ;
                                    let AWB = if AWA > A { 1.0 } else { 0.0 };
                                    let AWD = if AWB != 0.0 {
                                        AWA
                                    } else {
                                        let AWC = -AWA;
                                        AWC
                                    };
                                    let AWE = VX - (K * (AVZ + (((AVZ * AVZ) + AWD).sqrt())));
                                    AYJ = AWE;
                                }
                                AYI = AYJ;
                            } else {
                                let AWF = -((VM - AVS) - (((VB / BH) * J) / CK));
                                let AWG = (BH * AWF) + VO;
                                let AWH = AWF * AWF;
                                let AWI = (AWG * AWG) - (BN * (AWH + VL));
                                let AWJ = if AWI >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                                let AWL = if AWJ != 0.0 {
                                    AWI
                                } else {
                                    AWK
                                };
                                let AWM = (AWG - (AWL.sqrt())) / BH;
                                let AWN = (((AWH / VL) / VW).ln()) / (LL + (BH / AWF));
                                let AWO = if AWM < VJ { 1.0 } else { 0.0 };
                                let AYK;
                                if AWO != 0.0 {
                                    AYK = AWM;
                                } else {
                                    let AWP = (AWN - AWM) - VZ;
                                    let AWQ = (BN * AWN) * VZ;
                                    let AWR = if AWQ > A { 1.0 } else { 0.0 };
                                    let AWT = if AWR != 0.0 {
                                        AWQ
                                    } else {
                                        let AWS = -AWQ;
                                        AWS
                                    };
                                    let AWU = AWN - (K * (AWP + (((AWP * AWP) + AWT).sqrt())));
                                    AYK = AWU;
                                }
                                AYI = AYK;
                            }
                            let AWV = if ((1.2919089961638799e9f64 * AVS) / IA) > A { 1.0 } else { 0.0 };
                            let BHV = if AWV != 0.0 {
                                let AWW = ((1.2919089961638799e9f64 * AVS) / IA).sqrt();
                                AWW
                            } else {
                                A
                            };
                            let AWX = if AVT != 0.0 && A != 0.0 { 1.0 } else { 0.0 };
                            let BAB;
                            let BAP;
                            let BII;
                            let BLE;
                            if AWX != 0.0 {
                                let mut AWY = 0.0;
                                let mut AXA = 0.0;
                                let mut AYM = 0.0;
                                AWY = A;
                                AXA = AYI;
                                AYM = A;
                                loop {
                                    let AWZ = if AWY < N { 1.0 } else { 0.0 };
                                    if AWZ == 0.0 {
                                        break;
                                    }
                                    let AXB = LL * AXA;
                                    let AXC = (-AXB).exp();
                                    let AXD = if AXA > KM { 1.0 } else { 0.0 };
                                    let AXM;
                                    let AYB;
                                    if AXD != 0.0 {
                                        let AXE = AXB.exp();
                                        let AXF = (-VK) * ((((AXC + AXB) - B) + (VW * (AXE - B))).sqrt());
                                        let AXG = (EI / AXF) * (((-AXC) + B) + (VW * AXE));
                                        AXM = AXF;
                                        AYB = AXG;
                                    } else {
                                        let AXH = if AXA < -1e-9f64 { 1.0 } else { 0.0 };
                                        let AXN;
                                        let AYC;
                                        if AXH != 0.0 {
                                            let AXI = VK * (((AXC + AXB) - B).sqrt());
                                            let AXJ = (EI / AXI) * ((-AXC) + B);
                                            AXN = AXI;
                                            AYC = AXJ;
                                        } else {
                                            let AXK = ((-((EI / LL).sqrt())) * LL) * AXA;
                                            let AXL = -((EI * LL).sqrt());
                                            AXN = AXK;
                                            AYC = AXL;
                                        }
                                        AXM = AXN;
                                        AYB = AYC;
                                    }
                                    let AXO = ((AXM * AXM) + ((BN * VD) * VD)).sqrt();
                                    let AXP = K * (B + (AXM / AXO));
                                    let AXQ = (K * (AXM + AXO)) + (IP * VD);
                                    let AXR = if AXQ < A { 1.0 } else { 0.0 };
                                    let AXS;
                                    let AYA;
                                    if AXR != 0.0 {
                                        AXS = A;
                                        AYA = A;
                                    } else {
                                        AXS = AXQ;
                                        AYA = AXP;
                                    }
                                    let AXT = (VC - AXS) - VF;
                                    let AXU = (BN * VC) * VF;
                                    let AXV = if AXU > A { 1.0 } else { 0.0 };
                                    let AXX = if AXV != 0.0 {
                                        AXU
                                    } else {
                                        let AXW = -AXU;
                                        AXW
                                    };
                                    let AXY = ((AXT * AXT) + AXX).sqrt();
                                    let AXZ = VC - (K * (AXT + AXY));
                                    let AYD = ((((AXZ * AXZ) / BH) / CK) / EF) / IA;
                                    let AYE = AXA - (((((-AXA) + (AXM / CR)) - VM) + AYD) / ((-1e0f64 + (AYB / CR)) + (((BH * AYD) * (AYA * (AYB * (K * (B + (AXT / AXY)))))) / AXZ)));
                                    let AYF = if ((AYE - AXA).abs()) < PJ { 1.0 } else { 0.0 };
                                    let AYG = if AYF != 0.0 {
                                        N
                                    } else {
                                        AWY
                                    };
                                    let AYH = AYG + B;
                                    AWY = AYH;
                                    AXA = AYE;
                                    AYM = AXM;
                                }
                                let AYL = VM + AXA;
                                let AYN = AYL - (AYM / CR);
                                BAB = AYN;
                                BAP = AYL;
                                BII = AYM;
                                BLE = B;
                            } else {
                                let mut AYO = 0.0;
                                let mut AYQ = 0.0;
                                let mut AZZ = 0.0;
                                AYO = A;
                                AYQ = AYI;
                                AZZ = A;
                                loop {
                                    let AYP = if AYO < N { 1.0 } else { 0.0 };
                                    if AYP == 0.0 {
                                        break;
                                    }
                                    let AYR = LL * AYQ;
                                    let AYS = (-AYR).exp();
                                    let AYT = if AYQ > KM { 1.0 } else { 0.0 };
                                    let AZC;
                                    let AZR;
                                    if AYT != 0.0 {
                                        let AYU = AYR.exp();
                                        let AYV = (-VK) * ((((AYS + AYR) - B) + (VW * (AYU - B))).sqrt());
                                        let AYW = (EI / AYV) * (((-AYS) + B) + (VW * AYU));
                                        AZC = AYV;
                                        AZR = AYW;
                                    } else {
                                        let AYX = if AYQ < -1e-9f64 { 1.0 } else { 0.0 };
                                        let AZD;
                                        let AZS;
                                        if AYX != 0.0 {
                                            let AYY = VK * (((AYS + AYR) - B).sqrt());
                                            let AYZ = (EI / AYY) * ((-AYS) + B);
                                            AZD = AYY;
                                            AZS = AYZ;
                                        } else {
                                            let AZA = ((-((EI / LL).sqrt())) * LL) * AYQ;
                                            let AZB = -((EI * LL).sqrt());
                                            AZD = AZA;
                                            AZS = AZB;
                                        }
                                        AZC = AZD;
                                        AZR = AZS;
                                    }
                                    let AZE = ((AZC * AZC) + ((BN * VD) * VD)).sqrt();
                                    let AZF = K * (B + (AZC / AZE));
                                    let AZG = (K * (AZC + AZE)) + (IP * VD);
                                    let AZH = if AZG < A { 1.0 } else { 0.0 };
                                    let AZI;
                                    let AZQ;
                                    if AZH != 0.0 {
                                        AZI = A;
                                        AZQ = A;
                                    } else {
                                        AZI = AZG;
                                        AZQ = AZF;
                                    }
                                    let AZJ = (VC - AZI) - VF;
                                    let AZK = (BN * VC) * VF;
                                    let AZL = if AZK > A { 1.0 } else { 0.0 };
                                    let AZN = if AZL != 0.0 {
                                        AZK
                                    } else {
                                        let AZM = -AZK;
                                        AZM
                                    };
                                    let AZO = ((AZJ * AZJ) + AZN).sqrt();
                                    let AZP = VC - (K * (AZJ + AZO));
                                    let AZT = ((((AZP * AZP) / BH) / CK) / EF) / IA;
                                    let AZU = AYQ - ((((((AVS - AYQ) + (AZC / CR)) + (((AZC + (VB / BH)) * J) / CK)) - VM) + AZT) / (((-1e0f64 + (AZR / CR)) + ((AZR * J) / CK)) + (((BH * AZT) * (AZQ * (AZR * (K * (B + (AZJ / AZO)))))) / AZP)));
                                    let AZV = if ((AZU - AYQ).abs()) < PJ { 1.0 } else { 0.0 };
                                    let AZW = if AZV != 0.0 {
                                        N
                                    } else {
                                        AYO
                                    };
                                    let AZX = AZW + B;
                                    AYO = AZX;
                                    AYQ = AZU;
                                    AZZ = AZC;
                                }
                                let AZY = VM + AYQ;
                                let BAA = AZY - (AZZ / CR);
                                BAB = BAA;
                                BAP = AZY;
                                BII = AZZ;
                                BLE = BH;
                            }
                            let BAC = if BAB < A { 1.0 } else { 0.0 };
                            let BAG = if BAC != 0.0 {
                                A
                            } else {
                                BAB
                            };
                            BAF = BAG;
                            BAK = AVS;
                            BAO = BAP;
                            BHU = BHV;
                            BIH = BII;
                            BLD = BLE;
                        }
                        BAE = BAF;
                        BAJ = BAK;
                        BAN = BAO;
                        BHS = BHU;
                        BIG = BIH;
                        BLC = BLD;
                    }
                    let BAD = if AST < A { 1.0 } else { 0.0 };
                    let BAI = if BAD != 0.0 {
                        AST
                    } else {
                        BAJ
                    };
                    let BAH = if BAE < O { 1.0 } else { 0.0 };
                    let BAM = if BAH != 0.0 {
                        let BAL = BAI + (CM * ((K * VB) + ATI));
                        BAL
                    } else {
                        BAE
                    };
                    let mut BAQ = 0.0;
                    let mut BAS = 0.0;
                    let mut BBG = 0.0;
                    let mut BBM = 0.0;
                    let mut BFE = 0.0;
                    let mut BHM = 0.0;
                    let mut BHX = 0.0;
                    let mut BIC = 0.0;
                    let mut BIF = 0.0;
                    BAQ = B;
                    BAS = BAN;
                    BBG = BAI;
                    BBM = BAM;
                    BFE = A;
                    BHM = A;
                    BHX = A;
                    BIC = A;
                    BIF = BIG;
                    loop {
                        let BAR = if BAQ <= N { 1.0 } else { 0.0 };
                        if BAR == 0.0 {
                            break;
                        }
                        let BAT = BAS - VM;
                        let BAU = LL * BAT;
                        let BAV = (-BAU).exp();
                        let BAW = if BAT < -1e-9f64 { 1.0 } else { 0.0 };
                        let BFG;
                        let BFM;
                        if BAW != 0.0 {
                            let BAX = VK * (((BAV + BAU) - B).sqrt());
                            let BAY = (EI * ((-BAV) + B)) / BAX;
                            BFG = BAX;
                            BFM = BAY;
                        } else {
                            let BAZ = if BAT > KM { 1.0 } else { 0.0 };
                            let BFH;
                            let BFN;
                            if BAZ != 0.0 {
                                let BBA = BAU.exp();
                                let BBB = (-VK) * ((((BAV + BAU) - B) + (VW * ((BBA + BAU) - B))).sqrt());
                                let BBC = (EI * (((-BAV) + B) + (VW * (BBA + B)))) / BBB;
                                BFH = BBB;
                                BFN = BBC;
                            } else {
                                let BBD = -VK;
                                let BBE = BBD * BAU;
                                let BBF = BBD * LL;
                                BFH = BBE;
                                BFN = BBF;
                            }
                            BFG = BFH;
                            BFM = BFN;
                        }
                        let BBH = (LL * (BBG - AUV)).exp();
                        let BBI = (((AAA * AAA) / (MQ * MQ)) + ((BH * MX) * ((BBH + BAU) - B))).sqrt();
                        let BBJ = -MQ;
                        let BBK = (BBJ * BBI) - AAA;
                        let BBL = BBJ * ((((BH * LL) * MX) * (BBH + B)) / (BH * BBI));
                        let BBN = (BBM - BBG) / UT;
                        let BBO = LL * BBN;
                        let BBP = -BBO;
                        let BBQ = if BBP >= AMJ { 1.0 } else { 0.0 };
                        let BBY;
                        let BCC;
                        if BBQ != 0.0 {
                            let BBR = AML * ((B + BBP) - AMJ);
                            BBY = BBR;
                            BCC = AML;
                        } else {
                            let mut BBS = 0.0;
                            let mut BBU = 0.0;
                            BBS = BBP;
                            BBU = B;
                            loop {
                                let BBT = if BBS >= AMN { 1.0 } else { 0.0 };
                                if BBT == 0.0 {
                                    break;
                                }
                                let BBV = BBU * AMQ;
                                let BBW = BBS - AMN;
                                BBS = BBW;
                                BBU = BBV;
                            }
                            let BBX = BBU * (BBS.exp());
                            BBY = BBX;
                            BCC = BBX;
                        }
                        let BBZ = ((BBY + BBO) - B).sqrt();
                        let BCA = if BBN < -1e-9f64 { 1.0 } else { 0.0 };
                        let BCM;
                        let BDP;
                        let BDT;
                        if BCA != 0.0 {
                            let BCB = MQ * BBZ;
                            let BCD = (((MQ * LL) * ((-BCC) + B)) / (BH * BBZ)) / UT;
                            let BCE = -BCD;
                            BCM = BCB;
                            BDP = BCD;
                            BDT = BCE;
                        } else {
                            let BCF = if BBN > KM { 1.0 } else { 0.0 };
                            let BCN;
                            let BDQ;
                            let BDU;
                            if BCF != 0.0 {
                                let BCG = BBJ * BBZ;
                                let BCH = (((BBJ * LL) * ((-BCC) + B)) / (BH * BBZ)) / UT;
                                let BCI = -BCH;
                                BCN = BCG;
                                BDQ = BCH;
                                BDU = BCI;
                            } else {
                                let BCJ = (BBJ * BBO) / MP;
                                let BCK = (BBJ * LL) / MP;
                                let BCL = -BCK;
                                BCN = BCJ;
                                BDQ = BCK;
                                BDU = BCL;
                            }
                            BCM = BCN;
                            BDP = BDQ;
                            BDT = BDU;
                        }
                        let BCO = -VA;
                        let BCP = A - BCO;
                        let BCQ = if (if BCM > BCP { 1.0 } else { 0.0 }) != 0.0 && (if BCO >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BDR;
                        let BDW;
                        if BCQ != 0.0 {
                            let BCR = BCM + BCO;
                            let BCS = BCR * BCR;
                            let BCT = BCO * BCO;
                            let BCU = BCT * BCT;
                            let BCV = (BCS * BCS) + BCU;
                            let BDL;
                            if BCW != 0.0 {
                                let BDG;
                                if BCX != 0.0 {
                                    BDG = B;
                                } else {
                                    let BDH;
                                    if BCY != 0.0 {
                                        BDH = BH;
                                    } else {
                                        let BDI;
                                        if BCZ != 0.0 {
                                            BDI = BT;
                                        } else {
                                            let BDJ = if BDA != 0.0 {
                                                BN
                                            } else {
                                                A
                                            };
                                            BDI = BDJ;
                                        }
                                        BDH = BDI;
                                    }
                                    BDG = BDH;
                                }
                                let mut BDB = 0.0;
                                let mut BDD = 0.0;
                                BDB = A;
                                BDD = BCV;
                                loop {
                                    let BDC = if BDB < BDG { 1.0 } else { 0.0 };
                                    if BDC == 0.0 {
                                        break;
                                    }
                                    let BDE = BDD.sqrt();
                                    let BDF = BDB + B;
                                    BDB = BDF;
                                    BDD = BDE;
                                }
                                BDL = BDD;
                            } else {
                                let BDK = BCV.powf(2.5e-1f64);
                                BDL = BDK;
                            }
                            let BDM = B / BDL;
                            let BDN = ((BCO * BCU) * BDM) / BCV;
                            let BDO = BCP + ((BCR * BCO) * BDM);
                            BDR = BDN;
                            BDW = BDO;
                        } else {
                            BDR = B;
                            BDW = BCM;
                        }
                        let BDS = BDP * BDR;
                        let BDV = BDT * BDR;
                        let BDX = VB - AAA;
                        let BDY = -BDX;
                        let BDZ = BDX + BDY;
                        let BEA = if (if BDW < BDZ { 1.0 } else { 0.0 }) != 0.0 && (if BDY >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BEZ;
                        let BFC;
                        if BEA != 0.0 {
                            let BEB = BDZ - BDW;
                            let BEC = BEB * BEB;
                            let BED = BDY * BDY;
                            let BEE = BED * BED;
                            let BEF = (BEC * BEC) + BEE;
                            let BEV;
                            if BEG != 0.0 {
                                let BEQ;
                                if BEH != 0.0 {
                                    BEQ = B;
                                } else {
                                    let BER;
                                    if BEI != 0.0 {
                                        BER = BH;
                                    } else {
                                        let BES;
                                        if BEJ != 0.0 {
                                            BES = BT;
                                        } else {
                                            let BET = if BEK != 0.0 {
                                                BN
                                            } else {
                                                A
                                            };
                                            BES = BET;
                                        }
                                        BER = BES;
                                    }
                                    BEQ = BER;
                                }
                                let mut BEL = 0.0;
                                let mut BEN = 0.0;
                                BEL = A;
                                BEN = BEF;
                                loop {
                                    let BEM = if BEL < BEQ { 1.0 } else { 0.0 };
                                    if BEM == 0.0 {
                                        break;
                                    }
                                    let BEO = BEN.sqrt();
                                    let BEP = BEL + B;
                                    BEL = BEP;
                                    BEN = BEO;
                                }
                                BEV = BEN;
                            } else {
                                let BEU = BEF.powf(2.5e-1f64);
                                BEV = BEU;
                            }
                            let BEW = B / BEV;
                            let BEX = ((BDY * BEE) * BEW) / BEF;
                            let BEY = BDZ - ((BEB * BDY) * BEW);
                            BEZ = BEX;
                            BFC = BEY;
                        } else {
                            BEZ = B;
                            BFC = BDW;
                        }
                        let BFA = BDV * BEZ;
                        let BFB = BDS * BEZ;
                        let BFD = AAA + BFC;
                        let BFF = if (if BFE == B { 1.0 } else { 0.0 }) != 0.0 && (if BAQ > BT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BHF;
                        let BHH;
                        let BHI;
                        let BHJ;
                        let BHK;
                        let BHN;
                        if BFF != 0.0 {
                            BHF = N;
                            BHH = BAS;
                            BHI = BBG;
                            BHJ = BBM;
                            BHK = BFE;
                            BHN = BAQ;
                        } else {
                            let BFI = (BBG - UO) - (SR * ((((BFG + AAA) + BBK) + BFC) + ASL));
                            let BFJ = B - (SR * (BBL + BFA));
                            let BFK = -SR;
                            let BFL = BFK * BFB;
                            let BFO = BFK * BFM;
                            let BFP = BBM - (BBG + (CM * ((K * VB) + BFG)));
                            let BFR = -(CM * BFM);
                            let BFS = (BAS - BBM) - (CS * BFG);
                            let BFU = B - (CS * BFM);
                            let BFV = BFJ * BFU;
                            let BFW = BFJ * BFR;
                            let BFX = BFL * BFQ;
                            let BFY = BFO * BFQ;
                            let BFZ = -(B / ((((BFV - (BFW * BFT)) - (BFX * BFU)) + (BFY * BFT)) + GF));
                            let BGA = BFZ * ((((BFU - (BFR * BFT)) * BFI) + (((BFO * BFT) - (BFL * BFU)) * BFP)) + (((BFL * BFR) - BFO) * BFS));
                            let BGB = BFZ * (((BFU * BFI) + (BFV * BFP)) + ((BFY - BFW) * BFS));
                            let BGC = BFZ * ((BFI + (((-BFJ) * BFT) * BFP)) + ((BFJ - BFX) * BFS));
                            let BGD = BGA.abs();
                            let BGE = BGB.abs();
                            let BGF = if BGD < BGE { 1.0 } else { 0.0 };
                            let BGG = if BGF != 0.0 {
                                BGE
                            } else {
                                BGD
                            };
                            let BGH = BGC.abs();
                            let BGI = if BGG < BGH { 1.0 } else { 0.0 };
                            let BGN = if BGI != 0.0 {
                                BGH
                            } else {
                                BGG
                            };
                            let BGJ = if BAQ > ARE { 1.0 } else { 0.0 };
                            let BGO;
                            if BGJ != 0.0 {
                                BGO = ARG;
                            } else {
                                let BGK = if BAQ > ARH { 1.0 } else { 0.0 };
                                let BGP;
                                if BGK != 0.0 {
                                    BGP = ARG;
                                } else {
                                    let BGL = if BAQ > OR { 1.0 } else { 0.0 };
                                    let BGQ;
                                    if BGL != 0.0 {
                                        BGQ = ARK;
                                    } else {
                                        let BGM = if BAQ > L { 1.0 } else { 0.0 };
                                        let BGR = if BGM != 0.0 {
                                            LA
                                        } else {
                                            B
                                        };
                                        BGQ = BGR;
                                    }
                                    BGP = BGQ;
                                }
                                BGO = BGP;
                            }
                            let BGS = BI / BGO;
                            let BGT = if BGN > BGS { 1.0 } else { 0.0 };
                            let BGY;
                            let BHA;
                            let BHC;
                            if BGT != 0.0 {
                                let BGU = BGS / BGN;
                                let BGV = BGA * BGU;
                                let BGW = BGB * BGU;
                                let BGX = BGC * BGU;
                                BGY = BGV;
                                BHA = BGW;
                                BHC = BGX;
                            } else {
                                BGY = BGA;
                                BHA = BGB;
                                BHC = BGC;
                            }
                            let BGZ = BBG + BGY;
                            let BHB = BBM + BHA;
                            let BHD = BAS + BHC;
                            let BHE = if BGN < (PJ * BGO) { 1.0 } else { 0.0 };
                            let BHL = if BHE != 0.0 {
                                B
                            } else {
                                BFE
                            };
                            BHF = BAQ;
                            BHH = BHD;
                            BHI = BGZ;
                            BHJ = BHB;
                            BHK = BHL;
                            BHN = BHM;
                        }
                        let BHG = BHF + B;
                        BAQ = BHG;
                        BAS = BHH;
                        BBG = BHI;
                        BBM = BHJ;
                        BFE = BHK;
                        BHM = BHN;
                        BHX = BBK;
                        BIC = BFD;
                        BIF = BFG;
                    }
                    let BHO = if BHM > A { 1.0 } else { 0.0 };
                    if BHO != 0.0 {
                    } else {
                    }
                    let BHP = if BFE == A { 1.0 } else { 0.0 };
                    let BHQ;
                    let CVI;
                    if BHP != 0.0 {
                        BHQ = BAI;
                        CVI = BAM;
                    } else {
                        BHQ = BBG;
                        CVI = BBM;
                    }
                    let CAM = if BAD != 0.0 {
                        B
                    } else {
                        A
                    };
                    let BHR = BHQ - AST;
                    let BHW = BHS / CK;
                    let BHY = BHX - ASU;
                    let BHZ = BHX + ASU;
                    let BIA = BHY - (((LL * BHZ) * BHR) * K);
                    let BIB = if (if BIA < A { 1.0 } else { 0.0 }) != 0.0 || (if OT == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CBN = if BIB != 0.0 {
                        A
                    } else {
                        BIA
                    };
                    let BID = -5e-1f64 * (BIC + ATD);
                    let BIE = BHR + PJ;
                    let BIJ = VB * VE;
                    let BIK = if BIJ >= A { 1.0 } else { 0.0 };
                    let BIL = if (if (-(((BIF * BIF) - (ATI * ATI)) / (CR / ((CR * BHW) + B)))) < BIJ { 1.0 } else { 0.0 }) != 0.0 && BIK != 0.0 { 1.0 } else { 0.0 };
                    if BIL != 0.0 {
                        if BIM != 0.0 {
                            let BIU;
                            if BIN != 0.0 {
                                BIU = B;
                            } else {
                                let BIV;
                                if BIO != 0.0 {
                                    BIV = BH;
                                } else {
                                    let BIW;
                                    if BIP != 0.0 {
                                        BIW = BT;
                                    } else {
                                        let BIX = if BIQ != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        BIW = BIX;
                                    }
                                    BIV = BIW;
                                }
                                BIU = BIV;
                            }
                            let mut BIR = 0.0;
                            BIR = A;
                            loop {
                                let BIS = if BIR < BIU { 1.0 } else { 0.0 };
                                if BIS == 0.0 {
                                    break;
                                }
                                let BIT = BIR + B;
                                BIR = BIT;
                            }
                        } else {
                        }
                    } else {
                    }
                    let BIY = if ((LL * ATM) - B) > A { 1.0 } else { 0.0 };
                    if BIY != 0.0 {
                    } else {
                    }
                    let BIZ = -BHY;
                    let BJA = if (if BIZ < BIJ { 1.0 } else { 0.0 }) != 0.0 && BIK != 0.0 { 1.0 } else { 0.0 };
                    let BJW;
                    if BJA != 0.0 {
                        let BJB = BIJ - BIZ;
                        let BJC = BJB * BJB;
                        let BJD = BIJ * BIJ;
                        let BJE = (BJC * BJC) + (BJD * BJD);
                        let BJU;
                        if BJF != 0.0 {
                            let BJP;
                            if BJG != 0.0 {
                                BJP = B;
                            } else {
                                let BJQ;
                                if BJH != 0.0 {
                                    BJQ = BH;
                                } else {
                                    let BJR;
                                    if BJI != 0.0 {
                                        BJR = BT;
                                    } else {
                                        let BJS = if BJJ != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        BJR = BJS;
                                    }
                                    BJQ = BJR;
                                }
                                BJP = BJQ;
                            }
                            let mut BJK = 0.0;
                            let mut BJM = 0.0;
                            BJK = A;
                            BJM = BJE;
                            loop {
                                let BJL = if BJK < BJP { 1.0 } else { 0.0 };
                                if BJL == 0.0 {
                                    break;
                                }
                                let BJN = BJM.sqrt();
                                let BJO = BJK + B;
                                BJK = BJO;
                                BJM = BJN;
                            }
                            BJU = BJM;
                        } else {
                            let BJT = BJE.powf(2.5e-1f64);
                            BJU = BJT;
                        }
                        let BJV = BIJ - ((BJB * BIJ) * (B / BJU));
                        BJW = BJV;
                    } else {
                        BJW = BIZ;
                    }
                    let BJX = B - (((B + ((BH * (-BJW)) / (((LL * TM) * BIE) * BIE))) * BIE) / ASY);
                    let BJY = if (if BJX < 1e-5f64 { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                    let BKT;
                    if BJY != 0.0 {
                        let BJZ = 1e-5f64 - BJX;
                        let BKA = BJZ * BJZ;
                        let BKB = (BKA * BKA) + 1.0000000000000004e-20f64;
                        let BKR;
                        if BKC != 0.0 {
                            let BKM;
                            if BKD != 0.0 {
                                BKM = B;
                            } else {
                                let BKN;
                                if BKE != 0.0 {
                                    BKN = BH;
                                } else {
                                    let BKO;
                                    if BKF != 0.0 {
                                        BKO = BT;
                                    } else {
                                        let BKP = if BKG != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        BKO = BKP;
                                    }
                                    BKN = BKO;
                                }
                                BKM = BKN;
                            }
                            let mut BKH = 0.0;
                            let mut BKJ = 0.0;
                            BKH = A;
                            BKJ = BKB;
                            loop {
                                let BKI = if BKH < BKM { 1.0 } else { 0.0 };
                                if BKI == 0.0 {
                                    break;
                                }
                                let BKK = BKJ.sqrt();
                                let BKL = BKH + B;
                                BKH = BKL;
                                BKJ = BKK;
                            }
                            BKR = BKJ;
                        } else {
                            let BKQ = BKB.powf(2.5e-1f64);
                            BKR = BKQ;
                        }
                        let BKS = 1e-5f64 - ((BJZ * VE) * (B / BKR));
                        BKT = BKS;
                    } else {
                        BKT = BJX;
                    }
                    let BKU = B + BKT;
                    let BKV = B + (BKT * BKU);
                    let BKW = if BKU >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let BKY = if BKW != 0.0 {
                        BKU
                    } else {
                        BKX
                    };
                    let BKZ = -5e-1f64 * BHZ;
                    BLB = BLC;
                    BLH = BFE;
                    CAA = BKT;
                    CAD = BKY;
                    CAG = BKV;
                    CAL = CAM;
                    CAS = BHQ;
                    CBM = CBN;
                    CCF = BID;
                    CCM = BKZ;
                    CCV = BIF;
                    CCY = BHR;
                    CHY = ASY;
                    CVH = CVI;
                    EDZ = A;
                    EGL = A;
                    EGQ = A;
                    EGU = A;
                    EGY = A;
                }
                let BLA = if BB >= B { 1.0 } else { 0.0 };
                if BLA != 0.0 {
                    let BLF = if (if ATA == B { 1.0 } else { 0.0 }) != 0.0 && (if BLB == BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if BLF != 0.0 {
                    } else {
                    }
                    let BLG = if (if ATA == BH { 1.0 } else { 0.0 }) != 0.0 && (if BLB == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if BLG != 0.0 {
                    } else {
                    }
                } else {
                }
                if ASS != 0.0 {
                } else {
                }
                let BLI = if BLH == A { 1.0 } else { 0.0 };
                if BLI != 0.0 {
                } else {
                }
                let BLJ = if (APZ + BLH) < B { 1.0 } else { 0.0 };
                if BLJ != 0.0 {
                } else {
                }
                BZX = A;
                BZZ = CAA;
                CAC = CAD;
                CAF = CAG;
                CAK = CAL;
                CAR = CAS;
                CAV = AST;
                CAZ = ASX;
                CBL = CBM;
                CCE = CCF;
                CCL = CCM;
                CCT = ATI;
                CCU = CCV;
                CCX = CCY;
                CFA = ATL;
                CGF = CGG;
                CGT = CGU;
                CHX = CHY;
                CJJ = AAX;
                CJN = VM;
                CJO = AAA;
                CLO = CLP;
                CSK = ASL;
                CUP = CUQ;
                CVG = CVH;
                CVR = CVS;
                EDY = EDZ;
                EGK = EGL;
                EGP = EGQ;
                EGT = EGU;
                EGX = EGY;
                EIL = A;
                EIW = A;
            } else {
                let BLK = if NB < J { 1.0 } else { 0.0 };
                let BYF = if BLK != 0.0 {
                    B
                } else {
                    BH
                };
                let BLL = if OZ < (UQ + PD) { 1.0 } else { 0.0 };
                let BNO;
                let BRH;
                let BTR;
                let CLQ;
                if BLL != 0.0 {
                    let BLN = (BH * LN) * (((-GJ) / UR).ln());
                    let BLO = (B / (LL * MQ)) * TM;
                    let BLP = BH + (4.242640687119285e0f64 * BLO);
                    let BLQ = ((BO * BLP) * BLP) * BLP;
                    let BLS = (BLR * BLO) * ((LL * (UO - PD)) - BH);
                    let BLT = 9.899494936611664e0f64 - BLS;
                    let BLU = BLT * BLT;
                    let BLW = if BLQ < (BLU * BLV) { 1.0 } else { 0.0 };
                    let BLZ = if BLW != 0.0 {
                        let BLX = ((-9.899494936611664e0f64 + BLT) + ((K * BLQ) / BLT)) + BLS;
                        BLX
                    } else {
                        let BLY = (-9.899494936611664e0f64 + ((BLQ + BLU).sqrt())) + BLS;
                        BLY
                    };
                    let BMA = BLZ.powf(AAT);
                    let BMC = ((((((-5.65685424949238e0f64 - (BMB * BLO)) + (BH * BMA)) + ((MP * BMA) * BMA)) * (B / BMA)) * LN) + PD) - PD;
                    let BMD = BMC / BLN;
                    let BME = (BMC / ((B + (BMD * BMD)).sqrt())) + PD;
                    BNO = BME;
                    BRH = BLM;
                    BTR = A;
                    CLQ = A;
                } else {
                    let BNF;
                    let BNH;
                    if BMF != 0.0 {
                        BNF = A;
                        BNH = A;
                    } else {
                        let BMG = LL * (UO - PD);
                        let BMH = B + ((BN * (BMG - B)) / (US * LM));
                        let BMI = if BMH >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let BMK = if BMI != 0.0 {
                            BMH
                        } else {
                            BMJ
                        };
                        let BML = UO + (((US * LL) * K) * (B - (BMK.sqrt())));
                        let BMM = if (LL * (BML - PD)) < BT { 1.0 } else { 0.0 };
                        let BNC;
                        let BNI;
                        if BMM != 0.0 {
                            let BMN = B / ((1.3094570021973102e-2f64 * LL) * UR);
                            let BMO = AAN + (BT * BMN);
                            let BMP = (TY * BMN) * BMG;
                            let BMQ = (AAQ - (AAN * (AAR + BMN))) + BMP;
                            let BMR = (((-2.916e3f64 - (AAN * BMN)) + BMP) + (((((BN * BMO) * BMO) * BMO) + (BMQ * BMQ)).sqrt())).powf(AAT);
                            let BMS = (((BT - ((AAV * BMO) / (BT * BMR))) + (2.6456684199469993e-1f64 * BMR)) * LN) + PD;
                            BNC = BMS;
                            BNI = BMS;
                        } else {
                            let BMT = if OZ <= TR { 1.0 } else { 0.0 };
                            let BND;
                            if BMT != 0.0 {
                                BND = BML;
                            } else {
                                let BMU = (((((B / MX) / UV) * UO) * UO).ln()) / (LL + (BH / UO));
                                let BMV = (BMU - BML) - VZ;
                                let BMW = (BN * BMU) * VZ;
                                let BMX = if BMW > A { 1.0 } else { 0.0 };
                                let BMZ = if BMX != 0.0 {
                                    BMW
                                } else {
                                    let BMY = -BMW;
                                    BMY
                                };
                                let BNA = BMU - (K * (BMV + (((BMV * BMV) + BMZ).sqrt())));
                                BND = BNA;
                            }
                            BNC = BND;
                            BNI = BML;
                        }
                        let BNB = PD + 2.5e-12f64;
                        let BNE = if BNC < BNB { 1.0 } else { 0.0 };
                        let BNG = if BNE != 0.0 {
                            BNB
                        } else {
                            BNC
                        };
                        BNF = BNG;
                        BNH = BNI;
                    }
                    BNO = BNF;
                    BRH = A;
                    BTR = BNH;
                    CLQ = BNF;
                }
                let BNJ = if (if AFV == B { 1.0 } else { 0.0 }) != 0.0 && (if AKU == BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BNL = if BNJ != 0.0 {
                    let BNK = 1e-5f64 * ALD;
                    BNK
                } else {
                    A
                };
                let BNM = (LL * PD).exp();
                let BNN = MX * BNM;
                let BNP = (((IF * J) * J) / BH) / CK;
                let BNQ = ((BH * LL) * BNP).sqrt();
                let BNR = ((((BNQ.exp()) + ((-BNQ).exp())) / BH).ln()) / BNP;
                let mut BNS = 0.0;
                let mut BNU = 0.0;
                let mut BPC = 0.0;
                let mut BPI = 0.0;
                let mut BRI = 0.0;
                let mut BRM = 0.0;
                let mut BRN = 0.0;
                let mut BYE = 0.0;
                BNS = B;
                BNU = BNO;
                BPC = A;
                BPI = BRH;
                BRI = A;
                BRM = A;
                BRN = A;
                BYE = BYF;
                loop {
                    let BNT = if BNS <= 2.01e2f64 { 1.0 } else { 0.0 };
                    if BNT == 0.0 {
                        break;
                    }
                    let BNV = BNU - PD;
                    let BNW = LL * BNV;
                    let BNX = BNV - BNP;
                    let BNY = BNR * BNX;
                    let BNZ = if BNY < ARE { 1.0 } else { 0.0 };
                    let BOE;
                    let BOJ;
                    if BNZ != 0.0 {
                        let BOA = BNY.exp();
                        let BOB = B + (BOA - (((-BNR) * BNP).exp()));
                        let BOC = (BOB.ln()) / BNR;
                        let BOD = BOA / BOB;
                        BOE = BOC;
                        BOJ = BOD;
                    } else {
                        BOE = BNX;
                        BOJ = B;
                    }
                    let BOF = LL * BOE;
                    let BOG = BNW.abs();
                    let BOI = if BOG < BOH { 1.0 } else { 0.0 };
                    let BPK;
                    let BPS;
                    if BOI != 0.0 {
                        let BOK = ((B - (BOJ * BOJ)) / BH).sqrt();
                        let BOL = BNW * BOK;
                        let BOM = LL * BOK;
                        let BON = if BNW < A { 1.0 } else { 0.0 };
                        let BPL;
                        let BPT;
                        if BON != 0.0 {
                            let BOO = -BOL;
                            let BOP = -BOM;
                            BPL = BOO;
                            BPT = BOP;
                        } else {
                            BPL = BOL;
                            BPT = BOM;
                        }
                        BPK = BPL;
                        BPS = BPT;
                    } else {
                        let BOR = if BOG < BOQ { 1.0 } else { 0.0 };
                        let BPM;
                        let BPU;
                        if BOR != 0.0 {
                            let BOS = BNW / BT;
                            let BOT = BNW / BN;
                            let BOU = BOF / BT;
                            let BOV = BOF / BN;
                            let BOW = ((((BNW * BNW) / BH) * (B - (BOS * (B - (BOT * (B - (BNW / LA))))))) - (((BOF * BOF) / BH) * (B - (BOU * (B - (BOV * (B - (BOF / LA)))))))).sqrt();
                            let BOX = ((LL * K) * ((BNW * (B - ((BNW / BH) * (B - (BOS * (B - BOT)))))) - (BOJ * (BOF * (B - ((BOF / BH) * (B - (BOU * (B - BOV))))))))) / BOW;
                            BPM = BOW;
                            BPU = BOX;
                        } else {
                            let BOY = (-BNW).exp();
                            let BOZ = (-BOF).exp();
                            let BPA = ((BNW - BOF) + (BOY - BOZ)).sqrt();
                            let BPB = ((LL * K) * ((B - BOY) - (BOJ * (B - BOZ)))) / BPA;
                            BPM = BPA;
                            BPU = BPB;
                        }
                        BPK = BPM;
                        BPS = BPU;
                    }
                    let BPD = if BPC == B { 1.0 } else { 0.0 };
                    let BPE = if BNW < A { 1.0 } else { 0.0 };
                    let BPF = if BPD != 0.0 && BPE != 0.0 { 1.0 } else { 0.0 };
                    let BPH = if BPF != 0.0 {
                        BPG
                    } else {
                        BPI
                    };
                    let BPJ = if BPH == -1e0f64 { 1.0 } else { 0.0 };
                    let BPO = if BPJ != 0.0 {
                        A
                    } else {
                        let BPN = MY * BPK;
                        BPN
                    };
                    let BPP = if BPO < (J * 1.01e0f64) { 1.0 } else { 0.0 };
                    let BYG = if BPP != 0.0 {
                        B
                    } else {
                        BH
                    };
                    let BPQ = IF * BPO;
                    let BQI;
                    let BQL;
                    let BRO;
                    if BPE != 0.0 {
                        let BPR = -BPK;
                        let BPV = -BPS;
                        BQI = BPR;
                        BQL = BPV;
                        BRO = BRN;
                    } else {
                        let BPW = if BNW < CH { 1.0 } else { 0.0 };
                        let BQJ;
                        let BQM;
                        let BRP;
                        if BPW != 0.0 {
                            BQJ = BPK;
                            BQM = BPS;
                            BRP = BRN;
                        } else {
                            let BPX = if BNW < ARE { 1.0 } else { 0.0 };
                            let BQE;
                            let BQG;
                            if BPX != 0.0 {
                                let BPY = BNW.exp();
                                let BPZ = BNN * (BPY - (BNW + B));
                                let BQA = (BNN * LL) * (BPY - B);
                                BQE = BPZ;
                                BQG = BQA;
                            } else {
                                let BQB = (LL * BNU).exp();
                                let BQC = MX * (BQB - (BNM * (BNW + B)));
                                let BQD = (MX * LL) * (BQB - BNM);
                                BQE = BQC;
                                BQG = BQD;
                            }
                            let BQF = ((BPK * BPK) + BQE).sqrt();
                            let BQH = (K * (((BH * BPS) * BPK) + BQG)) / BQF;
                            BQJ = BQF;
                            BQM = BQH;
                            BRP = BQE;
                        }
                        BQI = BQJ;
                        BQL = BQM;
                        BRO = BRP;
                    }
                    let BQK = (((-UO) + BNU) + (UR * BQI)) - (SR * BNL);
                    let BQN = B + (UR * BQL);
                    let BRC;
                    let BRE;
                    let BRF;
                    if BPD != 0.0 {
                        BRC = BQO;
                        BRE = BNU;
                        BRF = BPC;
                    } else {
                        let BQP = (-BQK) / BQN;
                        let BQQ = BNU.abs();
                        let BQR = if B >= BQQ { 1.0 } else { 0.0 };
                        let BQS = if BQR != 0.0 {
                            B
                        } else {
                            BQQ
                        };
                        let BQT = 5e-2f64 * (B + BQS);
                        let BQU = if (BQP.abs()) > BQT { 1.0 } else { 0.0 };
                        let BQZ;
                        if BQU != 0.0 {
                            let BQV = if BQP >= A { 1.0 } else { 0.0 };
                            let BQX = if BQV != 0.0 {
                                B
                            } else {
                                BQW
                            };
                            let BQY = BQT * BQX;
                            BQZ = BQY;
                        } else {
                            BQZ = BQP;
                        }
                        let BRA = BNU + BQZ;
                        let BRB = if (if (BQZ.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (BQK.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BRG = if BRB != 0.0 {
                            B
                        } else {
                            BPC
                        };
                        BRC = BNS;
                        BRE = BRA;
                        BRF = BRG;
                    }
                    let BRD = BRC + B;
                    BNS = BRD;
                    BNU = BRE;
                    BPC = BRF;
                    BPI = BPH;
                    BRI = BPQ;
                    BRM = BQI;
                    BRN = BRO;
                    BYE = BYG;
                }
                let BRJ = BRI / MQ;
                let BRK = (BRJ * BRJ) + 2.220446049250313e-15f64;
                let BRL = BRJ + 2.220446049250313e-15f64;
                let BRQ = (MQ * BRN) * (B / (BRM + BRL));
                let BRR = -BRQ;
                let BRS = BRQ * SR;
                let BRT = if (if BPI == -1e0f64 { 1.0 } else { 0.0 }) != 0.0 || (if BRS <= I { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BSC;
                let BXK;
                let BYR;
                let CAN;
                let CAU;
                let CCJ;
                let EEA;
                let EGM;
                let EIM;
                let EIX;
                if BRT != 0.0 {
                    let BRU = TM * (UO - BNU);
                    let BRV = ((-DT) * CX) * BRU;
                    let BRZ = (-BRW) * BRU;
                    let BSA = BRZ * K;
                    let BSB = BRZ - BSA;
                    BSC = B;
                    BXK = BN;
                    BYR = A;
                    CAN = B;
                    CAU = BNU;
                    CCJ = BRU;
                    EEA = BNU;
                    EGM = BRV;
                    EIM = BSB;
                    EIX = BSA;
                } else {
                    BSC = A;
                    BXK = BPI;
                    BYR = BRS;
                    CAN = A;
                    CAU = A;
                    CCJ = A;
                    EEA = A;
                    EGM = A;
                    EIM = A;
                    EIX = A;
                }
                let BSD = if BSC == A { 1.0 } else { 0.0 };
                let CAB;
                let CAE;
                let CAH;
                let CAT;
                let CBO;
                let CCG;
                let CCN;
                let CCZ;
                if BSD != 0.0 {
                    let BSE = IG / (TM * TM);
                    let BSF = BH / BSE;
                    let BSG = B + (BSF * (UO - GF));
                    let BSH = B + BSF;
                    let BSI = if (if BSG < BSH { 1.0 } else { 0.0 }) != 0.0 && (if BSH >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BTE;
                    if BSI != 0.0 {
                        let BSJ = BSH - BSG;
                        let BSK = BSJ * BSJ;
                        let BSL = BSH * BSH;
                        let BSM = (((BSK * BSK) * BSK) * BSK) + (((BSL * BSL) * BSL) * BSL);
                        let BTC;
                        if BSN != 0.0 {
                            let BSX;
                            if BSO != 0.0 {
                                BSX = B;
                            } else {
                                let BSY;
                                if BSP != 0.0 {
                                    BSY = BH;
                                } else {
                                    let BSZ;
                                    if BSQ != 0.0 {
                                        BSZ = BT;
                                    } else {
                                        let BTA = if BSR != 0.0 {
                                            BN
                                        } else {
                                            A
                                        };
                                        BSZ = BTA;
                                    }
                                    BSY = BSZ;
                                }
                                BSX = BSY;
                            }
                            let mut BSS = 0.0;
                            let mut BSU = 0.0;
                            BSS = A;
                            BSU = BSM;
                            loop {
                                let BST = if BSS < BSX { 1.0 } else { 0.0 };
                                if BST == 0.0 {
                                    break;
                                }
                                let BSV = BSU.sqrt();
                                let BSW = BSS + B;
                                BSS = BSW;
                                BSU = BSV;
                            }
                            BTC = BSU;
                        } else {
                            let BTB = BSM.powf(1.25e-1f64);
                            BTC = BTB;
                        }
                        let BTD = BSH - ((BSJ * BSH) * (B / BTC));
                        BTE = BTD;
                    } else {
                        BTE = BSG;
                    }
                    let BTF = UO + (BSE * (B - (BTE.sqrt())));
                    let BTG = (K * (BTF + (((BTF * BTF) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let BTH = if BTG < A { 1.0 } else { 0.0 };
                    let BTI = if BTH != 0.0 {
                        A
                    } else {
                        BTG
                    };
                    let BTJ = OT / BTI;
                    let BTK = B + ((BTJ.powf((AUT - B))) * BTJ);
                    let BTL = OT / ((BTK.powf(((B / AUT) - B))) * BTK);
                    let BTM = (LL * (PD - BTL)).exp();
                    let BTN = if BTL <= A { 1.0 } else { 0.0 };
                    let BUK;
                    if BTN != 0.0 {
                        BUK = BNU;
                    } else {
                        let BUE = if BTO != 0.0 {
                            let BTP = A - BNU;
                            BTP
                        } else {
                            A
                        };
                        let BUD;
                        if BTQ != 0.0 {
                            let BTS = BTR - BNU;
                            let BTT = if BTS >= A { 1.0 } else { 0.0 };
                            let BTU = if BTT != 0.0 {
                                BTS
                            } else {
                                A
                            };
                            let BTV = ((1.3e0f64 * BTU) - BTL) - AGY;
                            let BTW = (BN * (1.3e0f64 * BTU)) * AGY;
                            let BTX = if BTW > A { 1.0 } else { 0.0 };
                            let BTZ = if BTX != 0.0 {
                                BTW
                            } else {
                                let BTY = -BTW;
                                BTY
                            };
                            let BUA = (1.3e0f64 * BTU) - (K * (BTV + (((BTV * BTV) + BTZ).sqrt())));
                            let BUB = if BUA <= BTU { 1.0 } else { 0.0 };
                            let BUC = if BUB != 0.0 {
                                BUA
                            } else {
                                BTU
                            };
                            BUD = BUC;
                        } else {
                            BUD = BUE;
                        }
                        let BUF = if BUD < A { 1.0 } else { 0.0 };
                        let BUH;
                        if BUF != 0.0 {
                            BUH = A;
                        } else {
                            let BUG = if BUD > BTL { 1.0 } else { 0.0 };
                            let BUI = if BUG != 0.0 {
                                BTL
                            } else {
                                BUD
                            };
                            BUH = BUI;
                        }
                        let BUJ = BNU + BUH;
                        BUK = BUJ;
                    }
                    let mut BUL = 0.0;
                    let mut BUN = 0.0;
                    let mut BWR = 0.0;
                    let mut BXN = 0.0;
                    let mut BXP = 0.0;
                    let mut BXQ = 0.0;
                    BUL = B;
                    BUN = BUK;
                    BWR = A;
                    BXN = BRI;
                    BXP = A;
                    BXQ = A;
                    loop {
                        let BUM = if BUL <= 2.01e2f64 { 1.0 } else { 0.0 };
                        if BUM == 0.0 {
                            break;
                        }
                        let BUO = BUN - PD;
                        let BUP = LL * BUO;
                        let BUQ = BUO - BNP;
                        let BUR = BNR * BUQ;
                        let BUS = if BUR < ARE { 1.0 } else { 0.0 };
                        let BUX;
                        let BVB;
                        if BUS != 0.0 {
                            let BUT = BUR.exp();
                            let BUU = B + (BUT - (((-BNR) * BNP).exp()));
                            let BUV = (BUU.ln()) / BNR;
                            let BUW = BUT / BUU;
                            BUX = BUV;
                            BVB = BUW;
                        } else {
                            BUX = BUQ;
                            BVB = B;
                        }
                        let BUY = LL * BUX;
                        let BUZ = BUP.abs();
                        let BVA = if BUZ < BOH { 1.0 } else { 0.0 };
                        let BVU;
                        let BWC;
                        if BVA != 0.0 {
                            let BVC = ((B - (BVB * BVB)) / BH).sqrt();
                            let BVD = BUP * BVC;
                            let BVE = LL * BVC;
                            let BVF = if BUP < A { 1.0 } else { 0.0 };
                            let BVV;
                            let BWD;
                            if BVF != 0.0 {
                                let BVG = -BVD;
                                let BVH = -BVE;
                                BVV = BVG;
                                BWD = BVH;
                            } else {
                                BVV = BVD;
                                BWD = BVE;
                            }
                            BVU = BVV;
                            BWC = BWD;
                        } else {
                            let BVI = if BUZ < BOQ { 1.0 } else { 0.0 };
                            let BVW;
                            let BWE;
                            if BVI != 0.0 {
                                let BVJ = BUP / BT;
                                let BVK = BUP / BN;
                                let BVL = BUY / BT;
                                let BVM = BUY / BN;
                                let BVN = ((((BUP * BUP) / BH) * (B - (BVJ * (B - (BVK * (B - (BUP / LA))))))) - (((BUY * BUY) / BH) * (B - (BVL * (B - (BVM * (B - (BUY / LA)))))))).sqrt();
                                let BVO = ((LL * K) * ((BUP * (B - ((BUP / BH) * (B - (BVJ * (B - BVK)))))) - (BVB * (BUY * (B - ((BUY / BH) * (B - (BVL * (B - BVM))))))))) / BVN;
                                BVW = BVN;
                                BWE = BVO;
                            } else {
                                let BVP = (-BUP).exp();
                                let BVQ = (-BUY).exp();
                                let BVR = ((BUP - BUY) + (BVP - BVQ)).sqrt();
                                let BVS = ((LL * K) * ((B - BVP) - (BVB * (B - BVQ)))) / BVR;
                                BVW = BVR;
                                BWE = BVS;
                            }
                            BVU = BVW;
                            BWC = BWE;
                        }
                        let BVT = if BXK == -1e0f64 { 1.0 } else { 0.0 };
                        let BVY = if BVT != 0.0 {
                            A
                        } else {
                            let BVX = MY * BVU;
                            BVX
                        };
                        let BVZ = IF * BVY;
                        let BWA = if BUP < A { 1.0 } else { 0.0 };
                        let BWL;
                        let BWO;
                        let BXR;
                        if BWA != 0.0 {
                            let BWB = -BVU;
                            let BWF = -BWC;
                            BWL = BWB;
                            BWO = BWF;
                            BXR = BXQ;
                        } else {
                            let BWG = if BUP < CH { 1.0 } else { 0.0 };
                            let BWM;
                            let BWP;
                            let BXS;
                            if BWG != 0.0 {
                                BWM = BVU;
                                BWP = BWC;
                                BXS = BXQ;
                            } else {
                                let BWH = (LL * (BUN - BTL)).exp();
                                let BWI = MX * (BWH - (BTM * (BUP + B)));
                                let BWJ = ((BVU * BVU) + BWI).sqrt();
                                let BWK = (K * (((BH * BWC) * BVU) + ((MX * LL) * (BWH - BTM)))) / BWJ;
                                BWM = BWJ;
                                BWP = BWK;
                                BXS = BWI;
                            }
                            BWL = BWM;
                            BWO = BWP;
                            BXR = BXS;
                        }
                        let BWN = (((-UO) + BUN) + (UR * BWL)) - (SR * BNL);
                        let BWQ = B + (UR * BWO);
                        let BWS = if (if BWR == B { 1.0 } else { 0.0 }) != 0.0 && (if BUL > BT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BXH;
                        let BXJ;
                        let BXL;
                        if BWS != 0.0 {
                            BXH = BWT;
                            BXJ = BUN;
                            BXL = BWR;
                        } else {
                            let BWU = (-BWN) / BWQ;
                            let BWV = BUN.abs();
                            let BWW = if B >= BWV { 1.0 } else { 0.0 };
                            let BWX = if BWW != 0.0 {
                                B
                            } else {
                                BWV
                            };
                            let BWY = 5e-2f64 * (B + BWX);
                            let BWZ = if (BWU.abs()) > BWY { 1.0 } else { 0.0 };
                            let BXE;
                            if BWZ != 0.0 {
                                let BXA = if BWU >= A { 1.0 } else { 0.0 };
                                let BXC = if BXA != 0.0 {
                                    B
                                } else {
                                    BXB
                                };
                                let BXD = BWY * BXC;
                                BXE = BXD;
                            } else {
                                BXE = BWU;
                            }
                            let BXF = BUN + BXE;
                            let BXG = if (if (BXE.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (BWN.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let BXM = if BXG != 0.0 {
                                B
                            } else {
                                BWR
                            };
                            BXH = BUL;
                            BXJ = BXF;
                            BXL = BXM;
                        }
                        let BXI = BXH + B;
                        BUL = BXI;
                        BUN = BXJ;
                        BWR = BXL;
                        BXN = BVZ;
                        BXP = BWL;
                        BXQ = BXR;
                    }
                    let BXO = BXN / MQ;
                    let BXT = -((MQ * BXQ) * (B / (BXP + (BXO + 2.220446049250313e-15f64))));
                    let BXU = BUN - BNU;
                    let BXV = K * (BRJ + BXO);
                    let BXW = ((LL * TM) * ((UO + LN) - (K * ((BH * BNU) + BXU)))) + ((LL * MQ) * ((-BXV) + ((B / (((((LL / BRK) * BXU) + B).sqrt()) + B)) / BRL)));
                    let BXX = BXN + BRI;
                    let BXY = BXX / BH;
                    let BXZ = BXT + BRR;
                    let BYA = (-BXZ) / BH;
                    let BYB = BXN - BRI;
                    let BYC = -(BXT - BRR);
                    let BYD = MQ * MQ;
                    let BYH = if BYE <= B { 1.0 } else { 0.0 };
                    let BYK = if BYH != 0.0 {
                        let BYI = (((BYA * LL) * BXU) - BYC) - ((((BYB * BYB) * BYB) / BYD) / LC);
                        BYI
                    } else {
                        let BYJ = BXU * BXW;
                        BYJ
                    };
                    let BYL = if (if BB >= B { 1.0 } else { 0.0 }) != 0.0 && (if BYK < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BYO = if BYL != 0.0 {
                        A
                    } else {
                        BYK
                    };
                    let CCH;
                    if BYH != 0.0 {
                        let BYM = if (BXU.abs()) > Q { 1.0 } else { 0.0 };
                        let CCI = if BYM != 0.0 {
                            let BYN = BH * BXY;
                            let BYP = ((BXY * (((BYA * LL) * BXU) - BYC)) + (((((((BYA - BYN) + ((TM / LL) * ((B - ((BYN * BXY) / BYD)) + (((BYB * BYB) / BYD) / L)))) * BYB) * BYB) * BYB) / BYD) / LC)) / BYO;
                            BYP
                        } else {
                            BXY
                        };
                        CCH = CCI;
                    } else {
                        let BYQ = K * BXX;
                        CCH = BYQ;
                    }
                    let BYS = B - (B - ((BXU + ((BH * UR) * (BXV - BRL))) * (B / BYR)));
                    let BYT = BYS * BYS;
                    let BYU = (((BYT * BYT) * BYT) * BYT) + 1e0f64;
                    let BZK;
                    if BYV != 0.0 {
                        let BZF;
                        if BYW != 0.0 {
                            BZF = B;
                        } else {
                            let BZG;
                            if BYX != 0.0 {
                                BZG = BH;
                            } else {
                                let BZH;
                                if BYY != 0.0 {
                                    BZH = BT;
                                } else {
                                    let BZI = if BYZ != 0.0 {
                                        BN
                                    } else {
                                        A
                                    };
                                    BZH = BZI;
                                }
                                BZG = BZH;
                            }
                            BZF = BZG;
                        }
                        let mut BZA = 0.0;
                        let mut BZC = 0.0;
                        BZA = A;
                        BZC = BYU;
                        loop {
                            let BZB = if BZA < BZF { 1.0 } else { 0.0 };
                            if BZB == 0.0 {
                                break;
                            }
                            let BZD = BZC.sqrt();
                            let BZE = BZA + B;
                            BZA = BZE;
                            BZC = BZD;
                        }
                        BZK = BZC;
                    } else {
                        let BZJ = BYU.powf(1.25e-1f64);
                        BZK = BZJ;
                    }
                    let BZL = B - (BYS * (B / BZK));
                    let BZM = B + BZL;
                    let BZN = B + (BZL * BZM);
                    let BZO = if BZM >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let BZQ = if BZO != 0.0 {
                        BZM
                    } else {
                        BZP
                    };
                    let CCO;
                    if BYH != 0.0 {
                        let BZR = if (BXU.abs()) > Q { 1.0 } else { 0.0 };
                        let CCP = if BZR != 0.0 {
                            let BZS = ((((((BYA * BYA) + ((BYC * BYC) / BMB)) * LL) * BXU) - (BYA * BYC)) - (((((((BH * BYA) + (((((TM / LL) * BYB) * BYB) / BYD) / LA)) * BYB) * BYB) * BYB) / BYD) / LC)) / BYO;
                            BZS
                        } else {
                            BYA
                        };
                        CCO = CCP;
                    } else {
                        let BZT = -5e-1f64 * BXZ;
                        CCO = BZT;
                    }
                    let BZU = if BPC == A { 1.0 } else { 0.0 };
                    if BZU != 0.0 {
                    } else {
                    }
                    let BZV = if BWR == A { 1.0 } else { 0.0 };
                    if BZV != 0.0 {
                    } else {
                    }
                    let BZW = if (BPC + BWR) < B { 1.0 } else { 0.0 };
                    if BZW != 0.0 {
                    } else {
                    }
                    CAB = BZL;
                    CAE = BZQ;
                    CAH = BZN;
                    CAT = BUN;
                    CBO = BYO;
                    CCG = CCH;
                    CCN = CCO;
                    CCZ = BXU;
                } else {
                    CAB = A;
                    CAE = A;
                    CAH = A;
                    CAT = CAU;
                    CBO = A;
                    CCG = CCJ;
                    CCN = A;
                    CCZ = A;
                }
                BZX = BSC;
                BZZ = CAB;
                CAC = CAE;
                CAF = CAH;
                CAK = CAN;
                CAR = CAT;
                CAV = BNU;
                CAZ = BRQ;
                CBL = CBO;
                CCE = CCG;
                CCL = CCN;
                CCT = A;
                CCU = A;
                CCX = CCZ;
                CFA = A;
                CGF = ML;
                CGT = MI;
                CHX = BYR;
                CJJ = A;
                CJN = A;
                CJO = A;
                CLO = CLQ;
                CSK = BNL;
                CUP = A;
                CVG = A;
                CVR = A;
                EDY = EEA;
                EGK = EGM;
                EGP = A;
                EGT = A;
                EGX = A;
                EIL = EIM;
                EIW = EIX;
            }
            let BZY = if BZX == A { 1.0 } else { 0.0 };
            let CIJ;
            let CSP;
            let CVQ;
            let CVY;
            let ECW;
            let EDE;
            let EDF;
            let EDU;
            let EEB;
            let EEO;
            let EES;
            let EEW;
            let EFD;
            let EGJ;
            let EGN;
            let EGR;
            let EGV;
            if BZY != 0.0 {
                let CAI = if (ADG - ((MA * (K + BZZ)) / (CAC * CAF))) > 5.0000001e-1f64 { 1.0 } else { 0.0 };
                if CAI != 0.0 {
                    let CAJ = if BB >= B { 1.0 } else { 0.0 };
                    if CAJ != 0.0 {
                    } else {
                    }
                } else {
                }
                let CAO = if CAK == A { 1.0 } else { 0.0 };
                let CBZ;
                let EDV;
                if CAO != 0.0 {
                    let CAQ = if (if BE < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if CAP < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CBX;
                    let EDW;
                    if CAQ != 0.0 {
                        let CAW = CAV + PN;
                        let CAX = if CAR > (CAW - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                        let EDX = if CAX != 0.0 {
                            let CAY = CAW - 2.220446049250313e-15f64;
                            CAY
                        } else {
                            CAR
                        };
                        CBX = A;
                        EDW = EDX;
                    } else {
                        if JD != 0.0 {
                        } else {
                        }
                        let CBB = CK * (B / ((CBA * IF) + (CAP * (CAZ * (B / J)))));
                        let CBD = (CBC * (OT + CAV)) + ((B - CBC) * CAR);
                        let CBE = CAV + PN;
                        let CBF = if CBD > (CBE - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                        let CBH = if CBF != 0.0 {
                            let CBG = CBE - 2.220446049250313e-15f64;
                            CBG
                        } else {
                            CBD
                        };
                        let CBI = CBH - CAR;
                        let CBJ = (K * (CBI + (((CBI * CBI) + 4e-6f64).sqrt()))) + 1e-13f64;
                        let CBK = if CBJ < A { 1.0 } else { 0.0 };
                        let CBT = if CBK != 0.0 {
                            A
                        } else {
                            CBJ
                        };
                        let CBP = CBL * (B / (LL * CAZ));
                        let CBQ = if CBP < LN { 1.0 } else { 0.0 };
                        let CBS = if CBQ != 0.0 {
                            LN
                        } else {
                            CBP
                        };
                        let CBU = (BH * (IF / CK)) * CBT;
                        let CBV = ((((BH * CBS) + (CBU * CBB)) + (CBR * CBB)) * (B / CW)) * CBB;
                        let CBW = QJ * (K * ((-CBV) + (((CBV * CBV) + (((BN * (CBU + CBR)) * CBB) * CBB)).sqrt())));
                        CBX = CBW;
                        EDW = CBH;
                    }
                    let CBY = CBX * EW;
                    CBZ = CBY;
                    EDV = EDW;
                } else {
                    CBZ = A;
                    EDV = EDY;
                }
                let CCA = CW - CBZ;
                let CCB = CX - CBZ;
                let CCC = if CCA < KM { 1.0 } else { 0.0 };
                let CDP = if CCC != 0.0 {
                    KM
                } else {
                    CCA
                };
                let CCD = (-DT) * CX;
                let CCK = CCD * CCE;
                let CCQ = CCD * CCL;
                let EGO;
                let EGS;
                let EGW;
                if H != 0.0 {
                    let CCR = CCK * K;
                    let CCS = CCK * 5e-1f64;
                    let CCW = ((K * (CCT + CCU)) * CX) * DT;
                    EGO = CCW;
                    EGS = CCR;
                    EGW = CCS;
                } else {
                    EGO = EGP;
                    EGS = EGT;
                    EGW = EGX;
                }
                let CDA = OT - CCX;
                let CDC = (BH * (CDA / BH)) / CDB;
                let CDD = CDB / (B + (CDC * (5e-1f64 + (CDC * (1.6666666666666666e-1f64 + (CDC * (4.1666666666666664e-2f64 + (CDC * (8.333333333333333e-3f64 + (CDC * (1.388888888888889e-3f64 + (CDC * 1.984126984126984e-4f64))))))))))));
                let CDE = if CDD < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let CDG = if CDE != 0.0 {
                    CDF
                } else {
                    CDD
                };
                let CDH = CAV + CDG;
                let CDJ = CCL / JB;
                let CDK = (((parameters[92] / CDI) * (CCE / JB)) + ((parameters[93] / CDI) * CDJ)) / (B + ((CAR - CAV) * parameters[94]));
                let CDL = (K * (CDK + (((CDK * CDK) + 3.6e7f64).sqrt()))) + 3e-7f64;
                let CDM = if CDL < A { 1.0 } else { 0.0 };
                let CDN = if CDM != 0.0 {
                    A
                } else {
                    CDL
                };
                let CDO = (B / (((B / (parameters[95] + ((parameters[96] * (CDJ / EF)) / 1e11f64))) + (LW * ((CDN.powf((parameters[97] - B))) * CDN))) + (((CDN.powf((DW - B))) * CDN) / parameters[106]))) * V;
                let CDQ = (LL * CAZ) * CDP;
                let CDR = (K * (CDQ + (((CDQ * CDQ) + 4e-100f64).sqrt()))) + 1.0000000000000001e-60f64;
                let CDS = if CDR < A { 1.0 } else { 0.0 };
                let CDT = if CDS != 0.0 {
                    A
                } else {
                    CDR
                };
                let CDU = CBL * (B / CDT);
                let CDV = (AFX * MD) / CDO;
                let CDW = ((CDU * CDU) + (CDV * CDV)).sqrt();
                let CDX = (CDO * CDW) / MD;
                let CDZ = if (if 9.999999999999978e-1f64 <= CDY { 1.0 } else { 0.0 }) != 0.0 && (if CDY <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CEC;
                if CDZ != 0.0 {
                    CEC = B;
                } else {
                    let CEA = if (if 1.9999999999999978e0f64 <= CDY { 1.0 } else { 0.0 }) != 0.0 && (if CDY <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CED = if CEA != 0.0 {
                        CDX
                    } else {
                        let CEB = CDX.powf((CDY - B));
                        CEB
                    };
                    CEC = CED;
                }
                let CEE = B + (CDX * CEC);
                let CEF = if (if 9.999999999999978e-1f64 <= CDY { 1.0 } else { 0.0 }) != 0.0 && (if CDY <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CEK;
                if CEF != 0.0 {
                    let CEG = B / CEE;
                    CEK = CEG;
                } else {
                    let CEH = if (if 1.9999999999999978e0f64 <= CDY { 1.0 } else { 0.0 }) != 0.0 && (if CDY <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CEL = if CEH != 0.0 {
                        let CEI = B / (CEE.sqrt());
                        CEI
                    } else {
                        let CEJ = CEE * (CEE.powf(((-1e0f64 / CDY) - B)));
                        CEJ
                    };
                    CEK = CEL;
                }
                let CEM = CDO * CEK;
                let CEN = (DR * LN) / CCA;
                let CEO = (CEN * CBL) * CEM;
                let CEQ = if (if CEP > A { 1.0 } else { 0.0 }) != 0.0 && (if EJ != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CFG;
                if CEQ != 0.0 {
                    let CER = (BH * (K * CDA)) / O;
                    let CES = CAV + (O / (B + (CER * (5e-1f64 + (CER * (1.6666666666666666e-1f64 + (CER * (4.1666666666666664e-2f64 + (CER * (8.333333333333333e-3f64 + (CER * (1.388888888888889e-3f64 + (CER * 1.984126984126984e-4f64)))))))))))));
                    let CET = 1.1e0f64 - CES;
                    let CEU = (K * (CET + (((CET * CET) + 1.0000000000000002e-2f64).sqrt()))) + 5.0000000000000005e-12f64;
                    let CEV = if CEU < A { 1.0 } else { 0.0 };
                    let CEW = if CEV != 0.0 {
                        A
                    } else {
                        CEU
                    };
                    let CEX = (TM * (LL * EK)) * (CEW.powf(parameters[245]));
                    let CEY = B + (PN * parameters[246]);
                    let CFC = if RQ != 0.0 {
                        let CEZ = CES - PM;
                        CEZ
                    } else {
                        let CFB = CES - CFA;
                        CFB
                    };
                    let CFD = CEX * (CEY + ((PN * EL) * CFC));
                    CFG = CFD;
                } else {
                    CFG = A;
                }
                let CFE = if EM != A { 1.0 } else { 0.0 };
                let CFH = if CFE != 0.0 {
                    let CFF = (TM * (LL * EN)) * PN;
                    CFF
                } else {
                    A
                };
                let CFI = CFG + CFH;
                let CFJ = if CFI > A { 1.0 } else { 0.0 };
                let CFL = if CFJ != 0.0 {
                    let CFK = (CEN * (CCX * CFI)) * CEM;
                    CFK
                } else {
                    A
                };
                let CFM = CEO + CFL;
                let CFN = if parameters[33] != A { 1.0 } else { 0.0 };
                let CIK;
                if CFN != 0.0 {
                    let CFO = ET - TG;
                    let CFP = (((((BH * TF) * (CK * SR)) * IM) * (B / (CFO * CFO))) * SW) * (parameters[154] + (parameters[155] * PN));
                    let CFR = ((PO - ES) + (CFQ - (parameters[157] * OT))) + CFP;
                    let CFS = (MJ * SR) * SR;
                    let CFT = (CFS * LL) * K;
                    let CFU = (CFT * LL) * BH;
                    let CFV = ((((LN - (CFS * (LL * AHX))) + ES) - CFQ) - CFP) + GF;
                    let CFW = (PO - CFV) - BOQ;
                    let CFX = if CFV >= A { 1.0 } else { 0.0 };
                    let CFZ = if CFX != 0.0 {
                        B
                    } else {
                        CFY
                    };
                    let CGA = B + (((LL * (((((CFV + (K * (CFW + (((CFW * CFW) + (((CFZ * BN) * CFV) * BOQ)).sqrt())))) - ES) + CFQ) + CFP) - RR)) - B) * (BN / CFU));
                    let CGB = (K * (CGA + (((CGA * CGA) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CGC = if CGB < A { 1.0 } else { 0.0 };
                    let CGD = if CGC != 0.0 {
                        A
                    } else {
                        CGB
                    };
                    let CGE = CFR + (CFT * (B - ((CGD + GF).sqrt())));
                    let CGH = ((((B / CGF) / CFS) * (CFR * CFR)).ln()) * (B / (LL + (BH / (CFR + GF))));
                    let CGI = (CGH - CGE) - 2e-3f64;
                    let CGJ = CGH - (K * (CGI + (((CGI * CGI) + (8e-3f64 * CGH)).sqrt())));
                    let CGK = (LL * (CGJ - RR)) - B;
                    let CGL = CGK + (CGF * ((LL * CGJ).exp()));
                    let CGM = (K * (CGL + (((CGL * CGL) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CGN = if CGM < A { 1.0 } else { 0.0 };
                    let CGO = if CGN != 0.0 {
                        A
                    } else {
                        CGM
                    };
                    let CGP = (CGO + 2.220446049250313e-15f64).sqrt();
                    let CGQ = (K * (CGK + (((CGK * CGK) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CGR = if CGQ < A { 1.0 } else { 0.0 };
                    let CGS = if CGR != 0.0 {
                        A
                    } else {
                        CGQ
                    };
                    let CGV = CGT * (CGP - ((CGS + 2.220446049250313e-15f64).sqrt()));
                    let CGW = CGE - CGJ;
                    let CGX = (K * (CGW + (((CGW * CGW) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64;
                    let CGY = if CGX < A { 1.0 } else { 0.0 };
                    let CGZ = if CGY != 0.0 {
                        A
                    } else {
                        CGX
                    };
                    let CHA = OT / (CGZ + 2.220446049250313e-15f64);
                    let CHB = CHA * CHA;
                    let CHC = (((CHB * CHB) * CHB) * CHB) + 1e0f64;
                    let CHS;
                    if CHD != 0.0 {
                        let CHN;
                        if CHE != 0.0 {
                            CHN = B;
                        } else {
                            let CHO;
                            if CHF != 0.0 {
                                CHO = BH;
                            } else {
                                let CHP;
                                if CHG != 0.0 {
                                    CHP = BT;
                                } else {
                                    let CHQ = if CHH != 0.0 {
                                        BN
                                    } else {
                                        A
                                    };
                                    CHP = CHQ;
                                }
                                CHO = CHP;
                            }
                            CHN = CHO;
                        }
                        let mut CHI = 0.0;
                        let mut CHK = 0.0;
                        CHI = A;
                        CHK = CHC;
                        loop {
                            let CHJ = if CHI < CHN { 1.0 } else { 0.0 };
                            if CHJ == 0.0 {
                                break;
                            }
                            let CHL = CHK.sqrt();
                            let CHM = CHI + B;
                            CHI = CHM;
                            CHK = CHL;
                        }
                        CHS = CHK;
                    } else {
                        let CHR = CHC.powf(1.25e-1f64);
                        CHS = CHR;
                    }
                    let CHT = CFM + (((((((BH * EV) * DB) * LN) * CEM) * CGV) * (CHA * (B / CHS))) / CDP);
                    CIK = CHT;
                } else {
                    CIK = CFM;
                }
                let CHW = if (if CHU != A { 1.0 } else { 0.0 }) != 0.0 && (if CHV != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EEP;
                let EET;
                let EEX;
                let EFE;
                if CHW != 0.0 {
                    let CHZ = CHX * CHX;
                    let CIA = CHZ - (((BH * LN) * SR) * CBL);
                    let CIB = (K * (CHZ + (((CHZ * CHZ) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let CIC = if CIB < A { 1.0 } else { 0.0 };
                    let CIF = if CIC != 0.0 {
                        A
                    } else {
                        CIB
                    };
                    let CID = (K * (CIA + (((CIA * CIA) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let CIE = if CID < A { 1.0 } else { 0.0 };
                    let CIG = if CIE != 0.0 {
                        A
                    } else {
                        CID
                    };
                    let CIH = CIF - CIG;
                    let CII = if (if CAZ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 || (if CIH < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EEQ = if CII != 0.0 {
                        A
                    } else {
                        B
                    };
                    EEP = EEQ;
                    EET = CIG;
                    EEX = CIF;
                    EFE = CIH;
                } else {
                    EEP = A;
                    EET = A;
                    EEX = A;
                    EFE = A;
                }
                CIJ = CIK;
                CSP = CDH;
                CVQ = CEM;
                CVY = CDW;
                ECW = CDP;
                EDE = CCQ;
                EDF = CCB;
                EDU = EDV;
                EEB = CDO;
                EEO = EEP;
                EES = EET;
                EEW = EEX;
                EFD = EFE;
                EGJ = CCK;
                EGN = EGO;
                EGR = EGS;
                EGV = EGW;
            } else {
                CIJ = A;
                CSP = B;
                CVQ = CVR;
                CVY = A;
                ECW = CW;
                EDE = A;
                EDF = A;
                EDU = EDY;
                EEB = A;
                EEO = A;
                EES = A;
                EEW = A;
                EFD = A;
                EGJ = EGK;
                EGN = EGP;
                EGR = EGT;
                EGV = EGX;
            }
            let CIM = if (if CEP > A { 1.0 } else { 0.0 }) != 0.0 && (if CIL > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CUH;
            let CYG;
            if CIM != 0.0 {
                let CIO = UO - CIN;
                let CIP = TR + CIN;
                let CIQ = LN * ((((AE / MG) * IE) / MG).ln());
                let CIR = if JD != 0.0 {
                    SL
                } else {
                    CFA
                };
                let CIS = ((((((3.2043836e-19f64 * (CIQ - CIR)) / CK) * IE) * AE) / (IE + AE)).sqrt()) * CZ;
                let CIT = ((-2.5e-1f64 * CIS) * CIS) / (OT + CIS);
                let CIU = LL * (CIO - CIT);
                let CIV = B + ((BN * (CIU - B)) / (US * LM));
                let CIW = if CIV >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let CIY = if CIW != 0.0 {
                    CIV
                } else {
                    CIX
                };
                let CIZ = CIO + (((US * LL) * K) * (B - (CIY.sqrt())));
                let CJA = if OZ < ((ES + CIP) * K) { 1.0 } else { 0.0 };
                if CJA != 0.0 {
                } else {
                }
                let CLH;
                let CLT;
                if CJB != 0.0 {
                    let CJC = if (LL * (CIZ - CIT)) < BT { 1.0 } else { 0.0 };
                    let CLM;
                    let CLW;
                    if CJC != 0.0 {
                        let CJD = B / ((1.3094570021973102e-2f64 * LL) * UR);
                        let CJE = AAN + (BT * CJD);
                        let CJF = (TY * CJD) * CIU;
                        let CJG = (AAQ - (AAN * (AAR + CJD))) + CJF;
                        let CJH = (((-2.916e3f64 - (AAN * CJD)) + CJF) + (((((BN * CJE) * CJE) * CJE) + (CJG * CJG)).sqrt())).powf(AAT);
                        let CJI = (((BT - ((AAV * CJE) / (BT * CJH))) + (2.6456684199469993e-1f64 * CJH)) * LN) + CIT;
                        CLM = CJI;
                        CLW = CJI;
                    } else {
                        let CJK = if (OZ - CJJ) <= CIP { 1.0 } else { 0.0 };
                        let CLN;
                        let CLX;
                        if CJK != 0.0 {
                            let CJQ = if H != 0.0 {
                                let CJL = J / CK;
                                let CJM = B / CR;
                                let CJP = CIO - (((B / (((B / TM) + CJL) + CJM)) * ((CIO - CJN) + ((CJM + (K * CJL)) * (-CJO)))) / TM);
                                CJP
                            } else {
                                CIZ
                            };
                            CLN = CJQ;
                            CLX = CJQ;
                        } else {
                            let CJR = CIO - CJJ;
                            let CJT = ((((((B / MX) / UV) * CJR) * CJR).ln()) / (LL + (BH / CJR))) + CJS;
                            let CJU = (CJT - CIZ) - VZ;
                            let CJV = (BN * CJT) * VZ;
                            let CJW = if CJV > A { 1.0 } else { 0.0 };
                            let CJY = if CJW != 0.0 {
                                CJV
                            } else {
                                let CJX = -CJV;
                                CJX
                            };
                            let CJZ = CJT - (K * (CJU + (((CJU * CJU) + CJY).sqrt())));
                            CLN = CJZ;
                            CLX = CIZ;
                        }
                        CLM = CLN;
                        CLW = CLX;
                    }
                    let CLI;
                    let CLU;
                    if H != 0.0 {
                        let CKA = if (OZ - CJJ) <= CIP { 1.0 } else { 0.0 };
                        let CLJ;
                        let CLV;
                        if CKA != 0.0 {
                            let CKB = J / CK;
                            let CKC = B / CR;
                            let CKD = CIO - (((B / (((B / TM) + CKB) + CKC)) * ((CIO - CJN) + ((CKC + (K * CKB)) * (-CJO)))) / TM);
                            CLJ = CKD;
                            CLV = CKD;
                        } else {
                            let CKE = J / CK;
                            let CKF = B / CR;
                            let CKG = CIO - (((B / (((B / TM) + CKE) + CKF)) * ((CIO - CJN) + ((CKF + (K * CKE)) * (-CJO)))) / TM);
                            let CKH = CIO - CJJ;
                            let CKI = if CKH > A { 1.0 } else { 0.0 };
                            let CLK;
                            if CKI != 0.0 {
                                let CKJ = (((((((B / MX) / UV) * CKH) * CKH).ln()) / (LL + (BH / CKH))) + CJS) * ABZ;
                                let CKK = CKJ - MA;
                                let CKL = if (if CKG > CKK { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                                let CLL;
                                if CKL != 0.0 {
                                    let CKM = (CKG - CKJ) + MA;
                                    let CKN = CKM * CKM;
                                    let CKO = (CKN * CKN) + 2.560000000000001e-2f64;
                                    let CLE;
                                    if CKP != 0.0 {
                                        let CKZ;
                                        if CKQ != 0.0 {
                                            CKZ = B;
                                        } else {
                                            let CLA;
                                            if CKR != 0.0 {
                                                CLA = BH;
                                            } else {
                                                let CLB;
                                                if CKS != 0.0 {
                                                    CLB = BT;
                                                } else {
                                                    let CLC = if CKT != 0.0 {
                                                        BN
                                                    } else {
                                                        A
                                                    };
                                                    CLB = CLC;
                                                }
                                                CLA = CLB;
                                            }
                                            CKZ = CLA;
                                        }
                                        let mut CKU = 0.0;
                                        let mut CKW = 0.0;
                                        CKU = A;
                                        CKW = CKO;
                                        loop {
                                            let CKV = if CKU < CKZ { 1.0 } else { 0.0 };
                                            if CKV == 0.0 {
                                                break;
                                            }
                                            let CKX = CKW.sqrt();
                                            let CKY = CKU + B;
                                            CKU = CKY;
                                            CKW = CKX;
                                        }
                                        CLE = CKW;
                                    } else {
                                        let CLD = CKO.powf(2.5e-1f64);
                                        CLE = CLD;
                                    }
                                    let CLF = CKK + ((CKM * MA) * (B / CLE));
                                    CLL = CLF;
                                } else {
                                    CLL = CKG;
                                }
                                CLK = CLL;
                            } else {
                                CLK = CKG;
                            }
                            CLJ = CLK;
                            CLV = CKG;
                        }
                        CLI = CLJ;
                        CLU = CLV;
                    } else {
                        CLI = CLM;
                        CLU = CLW;
                    }
                    CLH = CLI;
                    CLT = CLU;
                } else {
                    CLH = CLO;
                    CLT = CIZ;
                }
                let CLG = CIT + 2.5e-12f64;
                let CLR = if CLH < CLG { 1.0 } else { 0.0 };
                let CLS = if CLR != 0.0 {
                    CLG
                } else {
                    CLH
                };
                if A != 0.0 {
                    let CLY = CLT - CLS;
                    let CLZ = if CLY >= A { 1.0 } else { 0.0 };
                    let CMA = if CLZ != 0.0 {
                        CLY
                    } else {
                        A
                    };
                    let CMB = ((1.3e0f64 * CMA) - CJS) - AGY;
                    let CMC = (BN * (1.3e0f64 * CMA)) * AGY;
                    let CMD = if CMC > A { 1.0 } else { 0.0 };
                    let CMF = if CMD != 0.0 {
                        CMC
                    } else {
                        let CME = -CMC;
                        CME
                    };
                    let CMG = (1.3e0f64 * CMA) - (K * (CMB + (((CMB * CMB) + CMF).sqrt())));
                    let CMH = if CMG <= CMA { 1.0 } else { 0.0 };
                    let CMI = if CMH != 0.0 {
                        CMG
                    } else {
                        CMA
                    };
                    let CMJ = if CMI < A { 1.0 } else { 0.0 };
                    if CMJ != 0.0 {
                    } else {
                        let CMK = if CMI > OT { 1.0 } else { 0.0 };
                        if CMK != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let CML = if parameters[282] == B { 1.0 } else { 0.0 };
                let CQD;
                if CML != 0.0 {
                    let CMM = if OZ < ((UQ + CIT) + CIN) { 1.0 } else { 0.0 };
                    let CQE;
                    if CMM != 0.0 {
                        let CMN = (BH * LN) * (((-GJ) / UR).ln());
                        let CMO = (B / (LL * MQ)) * TM;
                        let CMP = BH + (4.242640687119285e0f64 * CMO);
                        let CMQ = ((BO * CMP) * CMP) * CMP;
                        let CMR = (BLR * CMO) * (CIU - BH);
                        let CMS = 9.899494936611664e0f64 - CMR;
                        let CMT = CMS * CMS;
                        let CMU = if CMQ < (CMT * BLV) { 1.0 } else { 0.0 };
                        let CMX = if CMU != 0.0 {
                            let CMV = ((-9.899494936611664e0f64 + CMS) + ((K * CMQ) / CMS)) + CMR;
                            CMV
                        } else {
                            let CMW = (-9.899494936611664e0f64 + ((CMQ + CMT).sqrt())) + CMR;
                            CMW
                        };
                        let CMY = CMX.powf(AAT);
                        let CMZ = ((((((-5.65685424949238e0f64 - (BMB * CMO)) + (BH * CMY)) + ((MP * CMY) * CMY)) * (B / CMY)) * LN) + CIT) - CIT;
                        let CNA = CMZ / CMN;
                        let CNB = (CMZ / ((B + (CNA * CNA)).sqrt())) + CIT;
                        CQE = CNB;
                    } else {
                        let CNC = (LL * (CIT - CJS)).exp();
                        let CND = (((IF * J) * J) / BH) / CK;
                        let CNE = ((BH * LL) * CND).sqrt();
                        let CNF = ((((CNE.exp()) + ((-CNE).exp())) / BH).ln()) / CND;
                        let mut CNG = 0.0;
                        let mut CNI = 0.0;
                        let mut COO = 0.0;
                        CNG = B;
                        CNI = CLS;
                        COO = A;
                        loop {
                            let CNH = if CNG <= 2.01e2f64 { 1.0 } else { 0.0 };
                            if CNH == 0.0 {
                                break;
                            }
                            let CNJ = CNI - CIT;
                            let CNK = LL * CNJ;
                            let CNL = CNJ - CND;
                            let CNM = CNF * CNL;
                            let CNN = if CNM < ARE { 1.0 } else { 0.0 };
                            let CNS;
                            let CNW;
                            if CNN != 0.0 {
                                let CNO = CNM.exp();
                                let CNP = B + (CNO - (((-CNF) * CND).exp()));
                                let CNQ = (CNP.ln()) / CNF;
                                let CNR = CNO / CNP;
                                CNS = CNQ;
                                CNW = CNR;
                            } else {
                                CNS = CNL;
                                CNW = B;
                            }
                            let CNT = LL * CNS;
                            let CNU = CNK.abs();
                            let CNV = if CNU < BOH { 1.0 } else { 0.0 };
                            let COS;
                            let COW;
                            if CNV != 0.0 {
                                let CNX = ((B - (CNW * CNW)) / BH).sqrt();
                                let CNY = CNK * CNX;
                                let CNZ = LL * CNX;
                                let COA = if CNK < A { 1.0 } else { 0.0 };
                                let COT;
                                let COX;
                                if COA != 0.0 {
                                    let COB = -CNY;
                                    let COC = -CNZ;
                                    COT = COB;
                                    COX = COC;
                                } else {
                                    COT = CNY;
                                    COX = CNZ;
                                }
                                COS = COT;
                                COW = COX;
                            } else {
                                let COD = if CNU < BOQ { 1.0 } else { 0.0 };
                                let COU;
                                let COY;
                                if COD != 0.0 {
                                    let COE = CNK / BT;
                                    let COF = CNK / BN;
                                    let COG = CNT / BT;
                                    let COH = CNT / BN;
                                    let COI = ((((CNK * CNK) / BH) * (B - (COE * (B - (COF * (B - (CNK / LA))))))) - (((CNT * CNT) / BH) * (B - (COG * (B - (COH * (B - (CNT / LA)))))))).sqrt();
                                    let COJ = ((LL * K) * ((CNK * (B - ((CNK / BH) * (B - (COE * (B - COF)))))) - (CNW * (CNT * (B - ((CNT / BH) * (B - (COG * (B - COH))))))))) / COI;
                                    COU = COI;
                                    COY = COJ;
                                } else {
                                    let COK = (-CNK).exp();
                                    let COL = (-CNT).exp();
                                    let COM = ((CNK - CNT) + (COK - COL)).sqrt();
                                    let CON = ((LL * K) * ((B - COK) - (CNW * (B - COL)))) / COM;
                                    COU = COM;
                                    COY = CON;
                                }
                                COS = COU;
                                COW = COY;
                            }
                            let COP = if COO == B { 1.0 } else { 0.0 };
                            let COQ = if CNK < A { 1.0 } else { 0.0 };
                            let COR = if COP != 0.0 && COQ != 0.0 { 1.0 } else { 0.0 };
                            if COR != 0.0 {
                            } else {
                            }
                            let CPE;
                            let CPH;
                            if COQ != 0.0 {
                                let COV = -COS;
                                let COZ = -COW;
                                CPE = COV;
                                CPH = COZ;
                            } else {
                                let CPA = if CNK < CH { 1.0 } else { 0.0 };
                                let CPF;
                                let CPI;
                                if CPA != 0.0 {
                                    CPF = COS;
                                    CPI = COW;
                                } else {
                                    let CPB = (LL * (CNI - CJS)).exp();
                                    let CPC = ((COS * COS) + (MX * (CPB - (CNC * (CNK + B))))).sqrt();
                                    let CPD = (K * (((BH * COW) * COS) + ((MX * LL) * (CPB - CNC)))) / CPC;
                                    CPF = CPC;
                                    CPI = CPD;
                                }
                                CPE = CPF;
                                CPH = CPI;
                            }
                            let CPG = ((-CIO) + CNI) + (UR * CPE);
                            let CPJ = B + (UR * CPH);
                            let CPY;
                            let CQA;
                            let CQB;
                            if COP != 0.0 {
                                CPY = CPK;
                                CQA = CNI;
                                CQB = COO;
                            } else {
                                let CPL = (-CPG) / CPJ;
                                let CPM = CNI.abs();
                                let CPN = if B >= CPM { 1.0 } else { 0.0 };
                                let CPO = if CPN != 0.0 {
                                    B
                                } else {
                                    CPM
                                };
                                let CPP = 5e-2f64 * (B + CPO);
                                let CPQ = if (CPL.abs()) > CPP { 1.0 } else { 0.0 };
                                let CPV;
                                if CPQ != 0.0 {
                                    let CPR = if CPL >= A { 1.0 } else { 0.0 };
                                    let CPT = if CPR != 0.0 {
                                        B
                                    } else {
                                        CPS
                                    };
                                    let CPU = CPP * CPT;
                                    CPV = CPU;
                                } else {
                                    CPV = CPL;
                                }
                                let CPW = CNI + CPV;
                                let CPX = if (if (CPV.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (CPG.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let CQC = if CPX != 0.0 {
                                    B
                                } else {
                                    COO
                                };
                                CPY = CNG;
                                CQA = CPW;
                                CQB = CQC;
                            }
                            let CPZ = CPY + B;
                            CNG = CPZ;
                            CNI = CQA;
                            COO = CQB;
                        }
                        CQE = CNI;
                    }
                    CQD = CQE;
                } else {
                    CQD = CLS;
                }
                let CQF = CQD - CIT;
                let CQG = (-LL) * CQF;
                let CQH = if CQG >= A { 1.0 } else { 0.0 };
                let CQJ = if CQH != 0.0 {
                    B
                } else {
                    CQI
                };
                let CQK = CQJ * CQG;
                let CQL = ((CQG.exp()) - B) - CQG;
                let CQM = if CQG > CH { 1.0 } else { 0.0 };
                let CQR;
                if CQM != 0.0 {
                    let CQN = (-MQ) * (CQL.sqrt());
                    CQR = CQN;
                } else {
                    let CQO = if CQK > CH { 1.0 } else { 0.0 };
                    let CQS = if CQO != 0.0 {
                        let CQP = MQ * (CQL.sqrt());
                        CQP
                    } else {
                        let CQQ = (((-CQJ) * CQK) * 7.071067811865475e-1f64) * ((B + ((CQK * AAT) * (B + (AHX * CQK)))).sqrt());
                        CQQ
                    };
                    CQR = CQS;
                }
                let CQT = (K * (CQR + (((CQR * CQR) + 4e-12f64).sqrt()))) + 1e-16f64;
                let CQU = if CQT < A { 1.0 } else { 0.0 };
                let CQV = if CQU != 0.0 {
                    A
                } else {
                    CQT
                };
                let CQW = CQV / IF;
                let CQX = CQW - parameters[283];
                let CQY = CQW * O;
                let CQZ = (K * (CQX + (((CQX * CQX) + ((BN * CQY) * CQY)).sqrt()))) + (IP * CQY);
                let CRA = if CQZ < A { 1.0 } else { 0.0 };
                let CRB = if CRA != 0.0 {
                    A
                } else {
                    CQZ
                };
                let CRC = (CQF * (((CRB / CQW) * CRB) / CQW)) + CIT;
                let CRD = ((LL * CRC).exp()) - ((LL * (CRC - OT)).exp());
                let CRE = (((3.2043836e-19f64 * AE) * CK).sqrt()) * MH;
                let CRF = LL * (CRC - CIT);
                let CRG = AFX * LL;
                let CRH = if (if CRF < CRG { 1.0 } else { 0.0 }) != 0.0 && (if CRG >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CSB;
                if CRH != 0.0 {
                    let CRI = CRG - CRF;
                    let CRJ = (CRI * CRI) + (CRG * CRG);
                    let CRZ;
                    if CRK != 0.0 {
                        let CRU;
                        if CRL != 0.0 {
                            CRU = B;
                        } else {
                            let CRV;
                            if CRM != 0.0 {
                                CRV = BH;
                            } else {
                                let CRW;
                                if CRN != 0.0 {
                                    CRW = BT;
                                } else {
                                    let CRX = if CRO != 0.0 {
                                        BN
                                    } else {
                                        A
                                    };
                                    CRW = CRX;
                                }
                                CRV = CRW;
                            }
                            CRU = CRV;
                        }
                        let mut CRP = 0.0;
                        let mut CRR = 0.0;
                        CRP = A;
                        CRR = CRJ;
                        loop {
                            let CRQ = if CRP < CRU { 1.0 } else { 0.0 };
                            if CRQ == 0.0 {
                                break;
                            }
                            let CRS = CRR.sqrt();
                            let CRT = CRP + B;
                            CRP = CRT;
                            CRR = CRS;
                        }
                        CRZ = CRR;
                    } else {
                        let CRY = CRJ.sqrt();
                        CRZ = CRY;
                    }
                    let CSA = CRG - ((CRI * CRG) * (B / CRZ));
                    CSB = CSA;
                } else {
                    CSB = CRF;
                }
                let CSC = CIJ + ((((((BH * LN) / CZ) * (CRE * ((CSB + 2.220446049250313e-15f64).sqrt()))) * CIL) * DR) * CRD);
                CUH = CSC;
                CYG = CQR;
            } else {
                CUH = CIJ;
                CYG = CCE;
            }
            let CSD = if JD != 0.0 || (if parameters[45] == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CUM;
            if CSD != 0.0 {
                let CSE = if (if CAK == B { 1.0 } else { 0.0 }) != 0.0 || (if AFV == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CUN;
                if CSE != 0.0 {
                    CUN = A;
                } else {
                    let CSF = if (if FJ <= A { 1.0 } else { 0.0 }) != 0.0 || (if P <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CUO;
                    if CSF != 0.0 {
                        CUO = A;
                    } else {
                        let CSG = (((PO - GB) + TQ) - UN) + parameters[48];
                        let CUD;
                        if EZ != 0.0 {
                            let CSH = TM * TM;
                            let CSI = IG / CSH;
                            let CSL = B + (((BH / IG) * CSH) * (((CSG - LN) - (AIN * RR)) - (AIN * ((CSJ * CSK) / CL))));
                            let CSM = (K * (CSL + (((CSL * CSL) + 4e-6f64).sqrt()))) + 1e-13f64;
                            let CSN = if CSM < A { 1.0 } else { 0.0 };
                            let CSO = if CSN != 0.0 {
                                A
                            } else {
                                CSM
                            };
                            let CSQ = ((AIU * PN) + CSP) - ((AIV * AIW) * ((CSG * AIS) + (CSI * (B - ((CSO + GF).sqrt())))));
                            let CSR = (K * (CSQ + (((CSQ * CSQ) + 4e-4f64).sqrt()))) + 1e-12f64;
                            let CSS = if CSR < A { 1.0 } else { 0.0 };
                            let CUE = if CSS != 0.0 {
                                A
                            } else {
                                CSR
                            };
                            CUD = CUE;
                        } else {
                            let CST = AJB * CSG;
                            let CSU = TM * TM;
                            let CSV = IG / CSU;
                            let CSW = (BH / IG) * CSU;
                            let CSX = B + (CSW * (((CST - LN) - (AIN * RR)) - (AIN * ((CSJ * CSK) / CL))));
                            let CSY = BH * (B + CSW);
                            let CSZ = GF + CSY;
                            let CTA = if (if CSX < CSZ { 1.0 } else { 0.0 }) != 0.0 && (if CSY >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let CTW;
                            if CTA != 0.0 {
                                let CTB = CSZ - CSX;
                                let CTC = CTB * CTB;
                                let CTD = CSY * CSY;
                                let CTE = (((CTC * CTC) * CTC) * CTC) + (((CTD * CTD) * CTD) * CTD);
                                let CTU;
                                if CTF != 0.0 {
                                    let CTP;
                                    if CTG != 0.0 {
                                        CTP = B;
                                    } else {
                                        let CTQ;
                                        if CTH != 0.0 {
                                            CTQ = BH;
                                        } else {
                                            let CTR;
                                            if CTI != 0.0 {
                                                CTR = BT;
                                            } else {
                                                let CTS = if CTJ != 0.0 {
                                                    BN
                                                } else {
                                                    A
                                                };
                                                CTR = CTS;
                                            }
                                            CTQ = CTR;
                                        }
                                        CTP = CTQ;
                                    }
                                    let mut CTK = 0.0;
                                    let mut CTM = 0.0;
                                    CTK = A;
                                    CTM = CTE;
                                    loop {
                                        let CTL = if CTK < CTP { 1.0 } else { 0.0 };
                                        if CTL == 0.0 {
                                            break;
                                        }
                                        let CTN = CTM.sqrt();
                                        let CTO = CTK + B;
                                        CTK = CTO;
                                        CTM = CTN;
                                    }
                                    CTU = CTM;
                                } else {
                                    let CTT = CTE.powf(1.25e-1f64);
                                    CTU = CTT;
                                }
                                let CTV = CSZ - ((CTB * CSY) * (B / CTU));
                                CTW = CTV;
                            } else {
                                CTW = CSX;
                            }
                            let CTX = if CTW <= A { 1.0 } else { 0.0 };
                            let CTZ = if CTX != 0.0 {
                                A
                            } else {
                                let CTY = CTW.sqrt();
                                CTY
                            };
                            let CUA = ((AIU * PN) + CSP) - ((DA / (AIV + DA)) * (CST + (CSV * (B - CTZ))));
                            let CUB = (K * (CUA + (((CUA * CUA) + 4e-6f64).sqrt()))) + 1e-13f64;
                            let CUC = if CUB < A { 1.0 } else { 0.0 };
                            let CUF = if CUC != 0.0 {
                                A
                            } else {
                                CUB
                            };
                            CUD = CUF;
                        }
                        let CUG = CUD + GF;
                        let CUI = ((AKS * CUG) * CUH) * (((-AKR) / CUG).exp());
                        CUO = CUI;
                    }
                    CUN = CUO;
                }
                CUM = CUN;
            } else {
                CUM = CUP;
            }
            let CUJ = if (if AFV == B { 1.0 } else { 0.0 }) != 0.0 && (if AKU == BH { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CUK = if CUJ != 0.0 && JD != 0.0 { 1.0 } else { 0.0 };
            if CUK != 0.0 {
                let CUL = -LL;
                let CUS = NA * O;
                let CUT = (NA - ((AKZ * LN) * ((B + (CUM * (2.1633307652783932e-2f64 / ((((EF * J) * DR) * ((CUL * AKW).exp())) * (4.1046315303568966e26f64 + (2.4665765749313358e0f64 * IA)))))).ln()))) - CUS;
                let CUU = (BN * NA) * CUS;
                let CUV = if CUU > A { 1.0 } else { 0.0 };
                let CUX = if CUV != 0.0 {
                    CUU
                } else {
                    let CUW = -CUU;
                    CUW
                };
                let CUY = CSP - (NA - (K * (CUT + (((CUT * CUT) + CUX).sqrt()))));
                let CUZ = if ((((CUL * CUY).exp()) - B) + (LL * CUY)) > A { 1.0 } else { 0.0 };
                if CUZ != 0.0 {
                } else {
                }
                let CVB = if ((BN * CVA) * (CVA * O)) > A { 1.0 } else { 0.0 };
                if CVB != 0.0 {
                } else {
                }
                let CVC = if parameters[138] > A { 1.0 } else { 0.0 };
                if CVC != 0.0 {
                } else {
                }
            } else {
            }
            let CVD = if CAK == A { 1.0 } else { 0.0 };
            let CVE = if (if CVD != 0.0 && (if CUM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[146] != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if CVE != 0.0 {
                let CVK;
                let CVN;
                if RP != 0.0 {
                    CVK = A;
                    CVN = A;
                } else {
                    let CVF = if JD != 0.0 {
                        PD
                    } else {
                        CFA
                    };
                    let CVJ = if JD != 0.0 {
                        PD
                    } else {
                        CVG
                    };
                    CVK = CVF;
                    CVN = CVJ;
                }
                let CVL = (LL * (CAV - CVK)) - B;
                let CVM = if ((K * (CVL + (((CVL * CVL) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64) < A { 1.0 } else { 0.0 };
                if CVM != 0.0 {
                } else {
                }
                let CVO = (LL * (CAR - CVN)) - B;
                let CVP = if ((K * (CVO + (((CVO * CVO) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64) < A { 1.0 } else { 0.0 };
                if CVP != 0.0 {
                } else {
                }
            } else {
            }
            let CVU = CJ * AZ;
            let CVV = TM / JB;
            let CVW = CW * AZ;
            let CVX = DR * AZ;
            let CVZ = CVY / AZ;
            let CWA = CCL / JB;
            let CWB = MQ / JB;
            let CWD = if CWC == A { 1.0 } else { 0.0 };
            let EJS;
            let EJW;
            let EJX;
            let EKA;
            let EKD;
            if CWD != 0.0 {
                EJS = A;
                EJW = A;
                EJX = A;
                EKA = A;
                EKD = A;
            } else {
                let EJY;
                if CVD != 0.0 {
                    let CWE = ((((PO - ES) + ((parameters[216] * (TQ - UN)) * CVW)) - (((CSP + PN) - 2.220446049250313e-15f64) * parameters[215])) * (B / CVU)) * (B + (CVZ * (B / parameters[217])));
                    let CWF = (K * (CWE + (((CWE * CWE) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CWG = if CWF < A { 1.0 } else { 0.0 };
                    let CWL = if CWG != 0.0 {
                        A
                    } else {
                        CWF
                    };
                    let CWH = (K * (PO + (((PO * PO) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let CWI = if CWH < A { 1.0 } else { 0.0 };
                    let CWJ = if CWI != 0.0 {
                        A
                    } else {
                        CWH
                    };
                    let CWK = (CWJ - PG) / BI;
                    let CWM = CWL * (B - (B / (B + (CWK * CWK))));
                    let CWN = CVW * CVX;
                    let CWP = CWO / (CWO + CWN);
                    let CWR = CWQ / (CWQ + PN);
                    let CWS = ((-parameters[214]) * MF) * (B / (CWM + GF));
                    let CWT = if CWS < -3.4e1f64 { 1.0 } else { 0.0 };
                    let EJZ = if CWT != 0.0 {
                        A
                    } else {
                        let CWU = (CWP * CWR) * (((((CWS.exp()) * (((parameters[213] / ME) * EF) * CWN)) * (((CWA + (CVV * I)) * (B / CWB)).sqrt())) * CWM) * CWM);
                        CWU
                    };
                    EJY = EJZ;
                } else {
                    EJY = A;
                }
                let CWV = -parameters[221];
                let CWX = (parameters[220] / AU) * CVX;
                let CWY = (CWX * ((CVU * ((CWV * OZ) + CWW)).exp())) * (OZ * ((OZ / CVU) / CVU));
                let CWZ = if OZ >= A { 1.0 } else { 0.0 };
                let EKE = if CWZ != 0.0 {
                    let CXA = CWY * -1e0f64;
                    CXA
                } else {
                    CWY
                };
                let CXB = OZ - OT;
                let CXC = (CWX * ((CVU * ((CWV * CXB) + CWW)).exp())) * (CXB * ((CXB / CVU) / CVU));
                let CXD = if CXB >= A { 1.0 } else { 0.0 };
                let EKB = if CXD != 0.0 {
                    let CXE = CXC * -1e0f64;
                    CXE
                } else {
                    CXC
                };
                let CXF = ((((-OZ) + PV) + ES) + parameters[225]) / CVU;
                let CXG = (K * (CXF + (((CXF * CXF) + 4e-4f64).sqrt()))) + 1e-12f64;
                let CXH = if CXG < A { 1.0 } else { 0.0 };
                let CXI = if CXH != 0.0 {
                    A
                } else {
                    CXG
                };
                let CXJ = CXI + GF;
                let CXK = (-parameters[224]) / CXJ;
                let CXL = if CXK < -3.4e1f64 { 1.0 } else { 0.0 };
                let EJT = if CXL != 0.0 {
                    A
                } else {
                    let CXM = ((((parameters[223] * CVX) * CVW) * CXJ) * CXJ) * (CXK.exp());
                    CXM
                };
                EJS = EJT;
                EJW = K;
                EJX = EJY;
                EKA = EKB;
                EKD = EKE;
            }
            let CXN = if parameters[28] == A { 1.0 } else { 0.0 };
            if CXN != 0.0 {
            } else {
                let CXR = (((CXO * (OT + CXP)) - OZ) + (TO * CXQ)) * (B / CJ);
                let CXS = (K * (CXR + (((CXR * CXR) + 4e-4f64).sqrt()))) + 1e-12f64;
                let CXT = if CXS < A { 1.0 } else { 0.0 };
                let CXU = if CXT != 0.0 {
                    A
                } else {
                    CXS
                };
                let CXW = if (((-CXV) * MF) * (B / (CXU + GF))) < -3.4e1f64 { 1.0 } else { 0.0 };
                if CXW != 0.0 {
                } else {
                }
                let CXX = if (OT - PV) > A { 1.0 } else { 0.0 };
                if CXX != 0.0 {
                } else {
                }
            }
            if CXN != 0.0 {
            } else {
                let CXY = (((CXO * ((-OT) + CXP)) - (OZ - OT)) + (TO * CXQ)) * (B / CJ);
                let CXZ = (K * (CXY + (((CXY * CXY) + 4e-4f64).sqrt()))) + 1e-12f64;
                let CYA = if CXZ < A { 1.0 } else { 0.0 };
                let CYB = if CYA != 0.0 {
                    A
                } else {
                    CXZ
                };
                let CYC = if (((-CXV) * MF) * (B / (CYB + GF))) < -3.4e1f64 { 1.0 } else { 0.0 };
                if CYC != 0.0 {
                } else {
                }
                let CYD = if (-PV) > A { 1.0 } else { 0.0 };
                if CYD != 0.0 {
                } else {
                }
            }
            let EHO;
            let EHV;
            let EIC;
            let EIN;
            if JD != 0.0 {
                let CYE = B / CO;
                let CYF = -BRW;
                let CYH = (CYF * CCL) + (CYF * CYG);
                let CYI = CYH * K;
                let CYJ = CYH - CYI;
                let EHP;
                let EHW;
                let EID;
                let EIO;
                if JE != 0.0 {
                    let CYR;
                    let CZS;
                    let DEY;
                    if CYK != 0.0 {
                        let CYN = CYL * K;
                        CYR = GN;
                        CZS = CYO;
                        DEY = CYN;
                    } else {
                        let CYS;
                        let CZT;
                        let DEZ;
                        if CYP != 0.0 {
                            let CYQ = BRW * K;
                            CYS = B;
                            CZT = ES;
                            DEZ = CYQ;
                        } else {
                            CYS = A;
                            CZT = A;
                            DEZ = A;
                        }
                        CYR = CYS;
                        CZS = CZT;
                        DEY = DEZ;
                    }
                    let CYT = if CYR == A { 1.0 } else { 0.0 };
                    let EHQ;
                    let EHX;
                    let EIE;
                    let EIP;
                    if CYT != 0.0 {
                        let CYU = MQ * ((IE / IE).sqrt());
                        let CYZ = (CYX * PD) + (CYY * (PD - OT));
                        let CZA = OZ - OT;
                        let CZB = (CYX * OZ) + (CYY * CZA);
                        let CZC = (CYY * OZ) + (CYX * CZA);
                        let CZD = ((CYX * OT) + (CYY * (-OT))) - CYZ;
                        let CZE = -CYZ;
                        let CZF = CYX + (CYW * CYY);
                        let CZG = CYY + (CYW * CYX);
                        let CZH = (CZF * CZB) + (CZG * CZC);
                        let CZI = -(((CZF * CZE) + (CZG * CZD)) + 2.220446049250313e-15f64);
                        let CZJ = if CZI > NP { 1.0 } else { 0.0 };
                        let CZO = if CZJ != 0.0 {
                            let CZK = NL - NP;
                            let CZL = (CZI - NP) / CZK;
                            let CZM = CZL * CZL;
                            let CZN = NP + (CZK * (B - (B / ((((B + CZL) + CZM) + (CZM * CZL)) + (CZM * CZM)))));
                            CZN
                        } else {
                            CZI
                        };
                        let CZP = (-CZO) - I;
                        let CZQ = CYU * CYE;
                        let CZR = CZQ * CZQ;
                        let CZU = CZH - CZS;
                        let CZV = (BH / LL) * ((IE / MG).ln());
                        let CZW = -CZP;
                        let CZX = if CZU < CZW { 1.0 } else { 0.0 };
                        let DEV;
                        let DKE;
                        let DKL;
                        let DKO;
                        if CZX != 0.0 {
                            let CZY = (B / (LL * CYU)) * CO;
                            let CZZ = BH + (4.242640687119285e0f64 * CZY);
                            let DAA = ((BO * CZZ) * CZZ) * CZZ;
                            let DAB = LK - CZV;
                            let DAC = (BLR * CZY) * ((LL * (CZU + CZP)) - BH);
                            let DAD = 9.899494936611664e0f64 - DAC;
                            let DAE = DAD * DAD;
                            let DAF = if DAA < (DAE * BLV) { 1.0 } else { 0.0 };
                            let DAI = if DAF != 0.0 {
                                let DAG = ((-9.899494936611664e0f64 + DAD) + ((K * DAA) / DAD)) + DAC;
                                DAG
                            } else {
                                let DAH = (-9.899494936611664e0f64 + ((DAA + DAE).sqrt())) + DAC;
                                DAH
                            };
                            let DAJ = DAI.powf(AAT);
                            let DAK = ((((((-5.65685424949238e0f64 - (BMB * CZY)) + (BH * DAJ)) + ((MP * DAJ) * DAJ)) / DAJ) * LN) - CZP) + CZP;
                            let DAL = DAK / DAB;
                            let DAM = CO * (CZU - ((DAK / ((B + (DAL * DAL)).sqrt())) - CZP));
                            DEV = DAM;
                            DKE = A;
                            DKL = A;
                            DKO = A;
                        } else {
                            let DAN = CZU + CZP;
                            let DAO = (LL * DAN) - B;
                            let DAP = CZR * LM;
                            let DAQ = B + ((BN * (DAO + 4.9787068367863944e-2f64)) / DAP);
                            let DAR = if DAQ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DAU = if DAR != 0.0 {
                                DAS
                            } else {
                                DAQ
                            };
                            let DAT = (CZR * LL) / BH;
                            let DAV = B + ((BN * (DAO + ((-(LL * ((CZU + (DAT * (B - (DAU.sqrt())))) + CZP))).exp()))) / DAP);
                            let DAW = if DAV < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DAY = if DAW != 0.0 {
                                DAX
                            } else {
                                DAV
                            };
                            let DAZ = LL * ((CZU + (DAT * (B - (DAY.sqrt())))) + CZP);
                            let DBA = if DAZ < BT { 1.0 } else { 0.0 };
                            let DBU = if DBA != 0.0 {
                                let DBB = 7.071067811865476e-1f64 + (B / (LL * CZQ));
                                let DBC = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DBB) / 5.286687693921294e-4f64)) + (((-DAN) / CZQ) / 1.8773541122053122e-2f64);
                                let DBD = ((2.8160311683079683e-2f64 * DBB) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DBE = ((DBC * DBC) + ((DBD * DBD) * DBD)).sqrt();
                                let DBF = LL * ((((((((-DBC) + DBE).powf(AAT)) + (-((DBC + DBE).powf(AAT)))) - -3.7209791878387604e0f64) * LN) - CZP) + CZP);
                                DBF
                            } else {
                                DAZ
                            };
                            let DBG = (LL * CZW).exp();
                            let DBH = MG / IE;
                            let DBI = DBH * DBH;
                            let DBJ = LL * (DAN + BI);
                            let DBK = (DBI * (DBG + GF)) * DAP;
                            let DBL = (DBI * DAP).ln();
                            let DBM = LL * CZP;
                            let DBN = (DBJ - ((((DBK + (DBJ * DBJ)).ln()) - DBL) + DBM)) - B;
                            let DBO = BN * DBJ;
                            let DBP = if DBO > A { 1.0 } else { 0.0 };
                            let DBR = if DBP != 0.0 {
                                DBO
                            } else {
                                let DBQ = -DBO;
                                DBQ
                            };
                            let DBS = (DBJ - (DBJ - (K * (DBN + (((DBN * DBN) + DBR).sqrt()))))) + (LL * BI);
                            let DBT = (((DBK + (DBS * DBS)).ln()) - DBL) + DBM;
                            let DBV = (DBT - DBU) - 6.0000000000000005e-2f64;
                            let DBW = (BN * DBT) * 6.0000000000000005e-2f64;
                            let DBX = if DBW > A { 1.0 } else { 0.0 };
                            let DBZ = if DBX != 0.0 {
                                DBW
                            } else {
                                let DBY = -DBW;
                                DBY
                            };
                            let DCA = DBT - (K * (DBV + (((DBV * DBV) + DBZ).sqrt())));
                            let DCB = (DCA / LL) - CZP;
                            let DCC = if ((DCA - B) + ((-DCA).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DCC != 0.0 {
                            } else {
                            }
                            let DCD = CO * (CZU - DCB);
                            let DCF = if DCE == B { 1.0 } else { 0.0 };
                            let DEW;
                            let DKF;
                            let DKM;
                            let DKP;
                            if DCF != 0.0 {
                                let DCG = DBI * DBG;
                                let mut DCH = 0.0;
                                let mut DCJ = 0.0;
                                let mut DDN = 0.0;
                                let mut DEJ = 0.0;
                                let mut DEM = 0.0;
                                let mut DER = 0.0;
                                let mut DES = 0.0;
                                DCH = B;
                                DCJ = DCB;
                                DDN = A;
                                DEJ = DCA;
                                DEM = A;
                                DER = A;
                                DES = A;
                                loop {
                                    let DCI = if DCH <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DCI == 0.0 {
                                        break;
                                    }
                                    let DCK = LL * (DCJ + CZP);
                                    let DCL = if DCK < LA { 1.0 } else { 0.0 };
                                    let DDJ;
                                    let DDL;
                                    let DEN;
                                    let DET;
                                    if DCL != 0.0 {
                                        let DCM = DCK * DCK;
                                        let DCP = (DCM * DCK) * (DCN + (DCK * (-7.053654284009761e-2f64 + (DCK * DCO))));
                                        let DCQ = DCK * LA;
                                        let DCR = (DCG * DCP) * DCP;
                                        let DCV = DCK * (DCS + (DCK * (-1.17851130197758e-1f64 + (DCK * (DCT + (DCK * (-1.63730162779191e-3f64 + (DCK * DCU))))))));
                                        let DCW = (((DCV * DCV) + DCR) + GF).sqrt();
                                        let DCX = ((((LL * (DCS + (DCK * (-2.35702260395516e-1f64 + (DCK * (5.3640151901649905e-2f64 + (DCK * (-6.54920651116764e-3f64 + (DCQ * DCU))))))))) * BH) * DCV) + ((((DCG * LL) * BH) * DCP) * (DCM * (8.907946456731299e-1f64 + (DCK * (-2.8214617136039044e-1f64 + (DCQ * DCO))))))) / (DCW + DCW);
                                        DDJ = DCW;
                                        DDL = DCX;
                                        DEN = DCV;
                                        DET = DCR;
                                    } else {
                                        let DCY = if DCK < ARE { 1.0 } else { 0.0 };
                                        let DDF;
                                        let DDH;
                                        if DCY != 0.0 {
                                            let DCZ = DCK.exp();
                                            let DDA = DCG * (DCZ - B);
                                            let DDB = (DCG * LL) * DCZ;
                                            DDF = DDA;
                                            DDH = DDB;
                                        } else {
                                            let DDC = (LL * DCJ).exp();
                                            let DDD = DBI * (DDC - DBG);
                                            let DDE = (DBI * LL) * DDC;
                                            DDF = DDD;
                                            DDH = DDE;
                                        }
                                        let DDG = ((DCK - B) + DDF).sqrt();
                                        let DDI = ((LL + DDH) / DDG) * K;
                                        DDJ = DDG;
                                        DDL = DDI;
                                        DEN = A;
                                        DET = DDF;
                                    }
                                    let DDK = (CZU - DCJ) - (CZQ * DDJ);
                                    let DDM = -1e0f64 - (CZQ * DDL);
                                    let DDO = if DDN == B { 1.0 } else { 0.0 };
                                    let DED;
                                    let DEF;
                                    let DEG;
                                    if DDO != 0.0 {
                                        DED = DDP;
                                        DEF = DCJ;
                                        DEG = DDN;
                                    } else {
                                        let DDQ = (-DDK) / DDM;
                                        let DDR = DCJ.abs();
                                        let DDS = if B >= DDR { 1.0 } else { 0.0 };
                                        let DDT = if DDS != 0.0 {
                                            B
                                        } else {
                                            DDR
                                        };
                                        let DDU = 5e-2f64 * (B + DDT);
                                        let DDV = if (DDQ.abs()) > DDU { 1.0 } else { 0.0 };
                                        let DEA;
                                        if DDV != 0.0 {
                                            let DDW = if DDQ >= A { 1.0 } else { 0.0 };
                                            let DDY = if DDW != 0.0 {
                                                B
                                            } else {
                                                DDX
                                            };
                                            let DDZ = DDU * DDY;
                                            DEA = DDZ;
                                        } else {
                                            DEA = DDQ;
                                        }
                                        let DEB = DCJ + DEA;
                                        let DEC = if (if (DEA.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (DDK.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DEH = if DEC != 0.0 {
                                            B
                                        } else {
                                            DDN
                                        };
                                        DED = DCH;
                                        DEF = DEB;
                                        DEG = DEH;
                                    }
                                    let DEE = DED + B;
                                    DCH = DEE;
                                    DCJ = DEF;
                                    DDN = DEG;
                                    DEJ = DCK;
                                    DEM = DEN;
                                    DER = DDJ;
                                    DES = DET;
                                }
                                let DEI = if DDN == A { 1.0 } else { 0.0 };
                                if DEI != 0.0 {
                                } else {
                                }
                                let DEK = if DEJ < LA { 1.0 } else { 0.0 };
                                let DEQ;
                                if DEK != 0.0 {
                                    let DEL = if DEJ < BT { 1.0 } else { 0.0 };
                                    if DEL != 0.0 {
                                    } else {
                                    }
                                    let DEO = DEM + 2.220446049250313e-15f64;
                                    DEQ = DEO;
                                } else {
                                    let DEP = (DEJ - B).sqrt();
                                    DEQ = DEP;
                                }
                                let DEU = (CYU * DEQ) + ((CYU * DES) * (B / (DER + DEQ)));
                                DEW = DEU;
                                DKF = DEM;
                                DKM = DER;
                                DKP = DES;
                            } else {
                                DEW = DCD;
                                DKF = A;
                                DKM = A;
                                DKP = A;
                            }
                            DEV = DEW;
                            DKE = DKF;
                            DKL = DKM;
                            DKO = DKP;
                        }
                        let EHT;
                        let EIA;
                        let EIG;
                        let EIR;
                        if DEX != 0.0 {
                            let EHU = if CYV != 0.0 {
                                let DFA = (-DEY) * DEV;
                                DFA
                            } else {
                                A
                            };
                            let EIB = if CYW != 0.0 {
                                let DFB = (-DEY) * DEV;
                                DFB
                            } else {
                                A
                            };
                            EHT = EHU;
                            EIA = EIB;
                            EIG = CYJ;
                            EIR = CYI;
                        } else {
                            let EIH;
                            let EIS;
                            if DFC != 0.0 {
                                let EII = if CYV != 0.0 {
                                    let DFD = (-DEY) * DEV;
                                    DFD
                                } else {
                                    CYJ
                                };
                                let EIT = if CYW != 0.0 {
                                    let DFE = (-DEY) * DEV;
                                    DFE
                                } else {
                                    CYI
                                };
                                EIH = EII;
                                EIS = EIT;
                            } else {
                                EIH = CYJ;
                                EIS = CYI;
                            }
                            EHT = A;
                            EIA = A;
                            EIG = EIH;
                            EIR = EIS;
                        }
                        let DFH = (DFF * CYX) + CYY;
                        let DFI = (DFF * CYY) + CYX;
                        let DFJ = (DFH * CZB) + (DFI * CZC);
                        let DFK = -(((DFH * CZE) + (DFI * CZD)) + 2.220446049250313e-15f64);
                        let DFL = if DFK > NP { 1.0 } else { 0.0 };
                        let DFQ = if DFL != 0.0 {
                            let DFM = NL - NP;
                            let DFN = (DFK - NP) / DFM;
                            let DFO = DFN * DFN;
                            let DFP = NP + (DFM * (B - (B / ((((B + DFN) + DFO) + (DFO * DFN)) + (DFO * DFO)))));
                            DFP
                        } else {
                            DFK
                        };
                        let DFR = (-DFQ) - I;
                        let DFS = DFJ - CZS;
                        let DFT = -DFR;
                        let DFU = if DFS < DFT { 1.0 } else { 0.0 };
                        let DKS;
                        if DFU != 0.0 {
                            let DFV = (B / (LL * CYU)) * CO;
                            let DFW = BH + (4.242640687119285e0f64 * DFV);
                            let DFX = ((BO * DFW) * DFW) * DFW;
                            let DFY = LK - CZV;
                            let DFZ = (BLR * DFV) * ((LL * (DFS + DFR)) - BH);
                            let DGA = 9.899494936611664e0f64 - DFZ;
                            let DGB = DGA * DGA;
                            let DGC = if DFX < (DGB * BLV) { 1.0 } else { 0.0 };
                            let DGF = if DGC != 0.0 {
                                let DGD = ((-9.899494936611664e0f64 + DGA) + ((K * DFX) / DGA)) + DFZ;
                                DGD
                            } else {
                                let DGE = (-9.899494936611664e0f64 + ((DFX + DGB).sqrt())) + DFZ;
                                DGE
                            };
                            let DGG = DGF.powf(AAT);
                            let DGH = ((((((-5.65685424949238e0f64 - (BMB * DFV)) + (BH * DGG)) + ((MP * DGG) * DGG)) / DGG) * LN) - DFR) + DFR;
                            let DGI = DGH / DFY;
                            let DGJ = CO * (DFS - ((DGH / ((B + (DGI * DGI)).sqrt())) - DFR));
                            DKS = DGJ;
                        } else {
                            let DGK = DFS + DFR;
                            let DGL = (LL * DGK) - B;
                            let DGM = CZR * LM;
                            let DGN = B + ((BN * (DGL + 4.9787068367863944e-2f64)) / DGM);
                            let DGO = if DGN < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DGR = if DGO != 0.0 {
                                DGP
                            } else {
                                DGN
                            };
                            let DGQ = (CZR * LL) / BH;
                            let DGS = B + ((BN * (DGL + ((-(LL * ((DFS + (DGQ * (B - (DGR.sqrt())))) + DFR))).exp()))) / DGM);
                            let DGT = if DGS < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DGV = if DGT != 0.0 {
                                DGU
                            } else {
                                DGS
                            };
                            let DGW = LL * ((DFS + (DGQ * (B - (DGV.sqrt())))) + DFR);
                            let DGX = if DGW < BT { 1.0 } else { 0.0 };
                            let DHR = if DGX != 0.0 {
                                let DGY = 7.071067811865476e-1f64 + (B / (LL * CZQ));
                                let DGZ = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DGY) / 5.286687693921294e-4f64)) + (((-DGK) / CZQ) / 1.8773541122053122e-2f64);
                                let DHA = ((2.8160311683079683e-2f64 * DGY) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DHB = ((DGZ * DGZ) + ((DHA * DHA) * DHA)).sqrt();
                                let DHC = LL * ((((((((-DGZ) + DHB).powf(AAT)) + (-((DGZ + DHB).powf(AAT)))) - -3.7209791878387604e0f64) * LN) - DFR) + DFR);
                                DHC
                            } else {
                                DGW
                            };
                            let DHD = (LL * DFT).exp();
                            let DHE = MG / IE;
                            let DHF = DHE * DHE;
                            let DHG = LL * (DGK + BI);
                            let DHH = (DHF * (DHD + GF)) * DGM;
                            let DHI = (DHF * DGM).ln();
                            let DHJ = LL * DFR;
                            let DHK = (DHG - ((((DHH + (DHG * DHG)).ln()) - DHI) + DHJ)) - B;
                            let DHL = BN * DHG;
                            let DHM = if DHL > A { 1.0 } else { 0.0 };
                            let DHO = if DHM != 0.0 {
                                DHL
                            } else {
                                let DHN = -DHL;
                                DHN
                            };
                            let DHP = (DHG - (DHG - (K * (DHK + (((DHK * DHK) + DHO).sqrt()))))) + (LL * BI);
                            let DHQ = (((DHH + (DHP * DHP)).ln()) - DHI) + DHJ;
                            let DHS = (DHQ - DHR) - 6.0000000000000005e-2f64;
                            let DHT = (BN * DHQ) * 6.0000000000000005e-2f64;
                            let DHU = if DHT > A { 1.0 } else { 0.0 };
                            let DHW = if DHU != 0.0 {
                                DHT
                            } else {
                                let DHV = -DHT;
                                DHV
                            };
                            let DHX = DHQ - (K * (DHS + (((DHS * DHS) + DHW).sqrt())));
                            let DHY = (DHX / LL) - DFR;
                            let DHZ = if ((DHX - B) + ((-DHX).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DHZ != 0.0 {
                            } else {
                            }
                            let DIA = CO * (DFS - DHY);
                            let DIB = if DCE == B { 1.0 } else { 0.0 };
                            let DKT;
                            if DIB != 0.0 {
                                let DIC = DHF * DHD;
                                let mut DID = 0.0;
                                let mut DIF = 0.0;
                                let mut DJE = 0.0;
                                let mut DKA = 0.0;
                                let mut DKD = 0.0;
                                let mut DKK = 0.0;
                                let mut DKN = 0.0;
                                DID = B;
                                DIF = DHY;
                                DJE = A;
                                DKA = DHX;
                                DKD = DKE;
                                DKK = DKL;
                                DKN = DKO;
                                loop {
                                    let DIE = if DID <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DIE == 0.0 {
                                        break;
                                    }
                                    let DIG = LL * (DIF + DFR);
                                    let DIH = if DIG < LA { 1.0 } else { 0.0 };
                                    let DJA;
                                    let DJC;
                                    let DKG;
                                    let DKQ;
                                    if DIH != 0.0 {
                                        let DII = DIG * DIG;
                                        let DIJ = (DII * DIG) * (DCN + (DIG * (-7.053654284009761e-2f64 + (DIG * DCO))));
                                        let DIK = DIG * LA;
                                        let DIL = (DIC * DIJ) * DIJ;
                                        let DIM = DIG * (DCS + (DIG * (-1.17851130197758e-1f64 + (DIG * (DCT + (DIG * (-1.63730162779191e-3f64 + (DIG * DCU))))))));
                                        let DIN = (((DIM * DIM) + DIL) + GF).sqrt();
                                        let DIO = ((((LL * (DCS + (DIG * (-2.35702260395516e-1f64 + (DIG * (5.3640151901649905e-2f64 + (DIG * (-6.54920651116764e-3f64 + (DIK * DCU))))))))) * BH) * DIM) + ((((DIC * LL) * BH) * DIJ) * (DII * (8.907946456731299e-1f64 + (DIG * (-2.8214617136039044e-1f64 + (DIK * DCO))))))) / (DIN + DIN);
                                        DJA = DIN;
                                        DJC = DIO;
                                        DKG = DIM;
                                        DKQ = DIL;
                                    } else {
                                        let DIP = if DIG < ARE { 1.0 } else { 0.0 };
                                        let DIW;
                                        let DIY;
                                        if DIP != 0.0 {
                                            let DIQ = DIG.exp();
                                            let DIR = DIC * (DIQ - B);
                                            let DIS = (DIC * LL) * DIQ;
                                            DIW = DIR;
                                            DIY = DIS;
                                        } else {
                                            let DIT = (LL * DIF).exp();
                                            let DIU = DHF * (DIT - DHD);
                                            let DIV = (DHF * LL) * DIT;
                                            DIW = DIU;
                                            DIY = DIV;
                                        }
                                        let DIX = ((DIG - B) + DIW).sqrt();
                                        let DIZ = ((LL + DIY) / DIX) * K;
                                        DJA = DIX;
                                        DJC = DIZ;
                                        DKG = A;
                                        DKQ = DIW;
                                    }
                                    let DJB = (DFS - DIF) - (CZQ * DJA);
                                    let DJD = -1e0f64 - (CZQ * DJC);
                                    let DJF = if DJE == B { 1.0 } else { 0.0 };
                                    let DJU;
                                    let DJW;
                                    let DJX;
                                    if DJF != 0.0 {
                                        DJU = DJG;
                                        DJW = DIF;
                                        DJX = DJE;
                                    } else {
                                        let DJH = (-DJB) / DJD;
                                        let DJI = DIF.abs();
                                        let DJJ = if B >= DJI { 1.0 } else { 0.0 };
                                        let DJK = if DJJ != 0.0 {
                                            B
                                        } else {
                                            DJI
                                        };
                                        let DJL = 5e-2f64 * (B + DJK);
                                        let DJM = if (DJH.abs()) > DJL { 1.0 } else { 0.0 };
                                        let DJR;
                                        if DJM != 0.0 {
                                            let DJN = if DJH >= A { 1.0 } else { 0.0 };
                                            let DJP = if DJN != 0.0 {
                                                B
                                            } else {
                                                DJO
                                            };
                                            let DJQ = DJL * DJP;
                                            DJR = DJQ;
                                        } else {
                                            DJR = DJH;
                                        }
                                        let DJS = DIF + DJR;
                                        let DJT = if (if (DJR.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (DJB.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DJY = if DJT != 0.0 {
                                            B
                                        } else {
                                            DJE
                                        };
                                        DJU = DID;
                                        DJW = DJS;
                                        DJX = DJY;
                                    }
                                    let DJV = DJU + B;
                                    DID = DJV;
                                    DIF = DJW;
                                    DJE = DJX;
                                    DKA = DIG;
                                    DKD = DKG;
                                    DKK = DJA;
                                    DKN = DKQ;
                                }
                                let DJZ = if DJE == A { 1.0 } else { 0.0 };
                                if DJZ != 0.0 {
                                } else {
                                }
                                let DKB = if DKA < LA { 1.0 } else { 0.0 };
                                let DKJ;
                                if DKB != 0.0 {
                                    let DKC = if DKA < BT { 1.0 } else { 0.0 };
                                    if DKC != 0.0 {
                                    } else {
                                    }
                                    let DKH = DKD + 2.220446049250313e-15f64;
                                    DKJ = DKH;
                                } else {
                                    let DKI = (DKA - B).sqrt();
                                    DKJ = DKI;
                                }
                                let DKR = (CYU * DKJ) + ((CYU * DKN) * (B / (DKK + DKJ)));
                                DKT = DKR;
                            } else {
                                DKT = DIA;
                            }
                            DKS = DKT;
                        }
                        let EHR;
                        let EHY;
                        let EIF;
                        let EIQ;
                        if DKU != 0.0 {
                            let EHS = if DFF != 0.0 {
                                let DKV = (-DEY) * DKS;
                                DKV
                            } else {
                                EHT
                            };
                            let EHZ = if DFG != 0.0 {
                                let DKW = (-DEY) * DKS;
                                DKW
                            } else {
                                EIA
                            };
                            EHR = EHS;
                            EHY = EHZ;
                            EIF = EIG;
                            EIQ = EIR;
                        } else {
                            let EIJ;
                            let EIU;
                            if DKX != 0.0 {
                                let EIK = if DFF != 0.0 {
                                    let DKY = (-DEY) * DKS;
                                    DKY
                                } else {
                                    EIG
                                };
                                let EIV = if DFG != 0.0 {
                                    let DKZ = (-DEY) * DKS;
                                    DKZ
                                } else {
                                    EIR
                                };
                                EIJ = EIK;
                                EIU = EIV;
                            } else {
                                EIJ = EIG;
                                EIU = EIR;
                            }
                            EHR = EHT;
                            EHY = EIA;
                            EIF = EIJ;
                            EIQ = EIU;
                        }
                        EHQ = EHR;
                        EHX = EHY;
                        EIE = EIF;
                        EIP = EIQ;
                    } else {
                        EHQ = A;
                        EHX = A;
                        EIE = CYJ;
                        EIP = CYI;
                    }
                    EHP = EHQ;
                    EHW = EHX;
                    EID = EIE;
                    EIO = EIP;
                } else {
                    EHP = A;
                    EHW = A;
                    EID = CYJ;
                    EIO = CYI;
                }
                EHO = EHP;
                EHV = EHW;
                EIC = EID;
                EIN = EIO;
            } else {
                EHO = A;
                EHV = A;
                EIC = EIL;
                EIN = EIW;
            }
            let DLA = if CAK != A { 1.0 } else { 0.0 };
            let EDS;
            let EHC;
            if DLA != 0.0 {
                let DLB = OT + CAV;
                let DLC = (CBC * DLB) + ((B - CBC) * CAR);
                let DLE = if DLD != A { 1.0 } else { 0.0 };
                if DLE != 0.0 {
                } else {
                }
                let DLF = if DLC > (DLB - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                let EDT = if DLF != 0.0 {
                    let DLG = DLB - 2.220446049250313e-15f64;
                    DLG
                } else {
                    DLC
                };
                EDS = EDT;
                EHC = A;
            } else {
                let DLH = if DLD != A { 1.0 } else { 0.0 };
                let EHD;
                if DLH != 0.0 {
                    let DLI = if CBL < 1e-15f64 { 1.0 } else { 0.0 };
                    let EHE = if DLI != 0.0 {
                        A
                    } else {
                        let DLJ = (CBL * (LN / CW)) * (B / CAZ);
                        DLJ
                    };
                    EHD = EHE;
                } else {
                    EHD = A;
                }
                EDS = EDU;
                EHC = EHD;
            }
            let DLK = B / CO;
            let EFW;
            let EGA;
            let EJF;
            let EJK;
            if JE != 0.0 {
                let DLM = if DLL > A { 1.0 } else { 0.0 };
                let DLN = if (if parameters[29] >= B { 1.0 } else { 0.0 }) != 0.0 && DLM != 0.0 { 1.0 } else { 0.0 };
                let EFX;
                let EGB;
                let EJG;
                let EJL;
                if DLN != 0.0 {
                    let DLO = if (if AD == A { 1.0 } else { 0.0 }) != 0.0 && DLM != 0.0 { 1.0 } else { 0.0 };
                    let DYY;
                    let DZF;
                    let EJH;
                    let EJM;
                    if DLO != 0.0 {
                        let DLS = if JD != 0.0 {
                            let DLQ = DLP * CO;
                            DLQ
                        } else {
                            let DLR = DT * CO;
                            DLR
                        };
                        let DLT = parameters[171] * DLS;
                        let DLU = parameters[172] + OZ;
                        let DLV = DLL * DLS;
                        let DLW = (OZ * DLV) - ((NJ - CAV) * (DLT * DLU));
                        let DLX = ((OZ - OT) * DLV) - ((DLT * (DLU - OT)) * (NJ - (CAR - OT)));
                        DYY = DLX;
                        DZF = DLW;
                        EJH = A;
                        EJM = A;
                    } else {
                        let DLY = MQ * ((AD / IE).sqrt());
                        let DMK;
                        let DMV;
                        let DSD;
                        let DSG;
                        if JD != 0.0 {
                            let DMB = (CYX * PD) + (CYY * (PD - OT));
                            let DMC = ((CYX * OZ) + (CYY * (OZ - OT))) - DMB;
                            let DMD = CYX + (DMA * CYY);
                            let DME = CYY + (DMA * CYX);
                            let DMF = ((DMD * (-DMB)) + (DME * (((CYX * OT) + (CYY * (-OT))) - DMB))) + 2.220446049250313e-15f64;
                            DMK = DMF;
                            DMV = DMC;
                            DSD = DMD;
                            DSG = DME;
                        } else {
                            let DMG = CYX + (DMA * CYY);
                            let DMH = CYY + (DMA * CYX);
                            let DMX = if DLZ != 0.0 {
                                let DMI = (CYX * OZ) + (CYY * (OZ - OT));
                                DMI
                            } else {
                                A
                            };
                            let DMW = if DMA != 0.0 {
                                let DMJ = (CYY * OZ) + (CYX * (OZ - OT));
                                DMJ
                            } else {
                                DMX
                            };
                            DMK = A;
                            DMV = DMW;
                            DSD = DMG;
                            DSG = DMH;
                        }
                        let DML = -DMK;
                        let DMM = if DML > NP { 1.0 } else { 0.0 };
                        let DMR = if DMM != 0.0 {
                            let DMN = NL - NP;
                            let DMO = (DML - NP) / DMN;
                            let DMP = DMO * DMO;
                            let DMQ = NP + (DMN * (B - (B / ((((B + DMO) + DMP) + (DMP * DMO)) + (DMP * DMP)))));
                            DMQ
                        } else {
                            DML
                        };
                        let DMS = (-DMR) - I;
                        let DMT = DLY * DLK;
                        let DMU = DMT * DMT;
                        let DMY = (-DMV) + AY;
                        let DMZ = (BH / LL) * ((AD / MG).ln());
                        let DNA = -DMS;
                        let DNB = if DMY < DNA { 1.0 } else { 0.0 };
                        let DRY;
                        let DXW;
                        if DNB != 0.0 {
                            let DNC = (B / (LL * DLY)) * CO;
                            let DND = BH + (4.242640687119285e0f64 * DNC);
                            let DNE = ((BO * DND) * DND) * DND;
                            let DNF = LK - DMZ;
                            let DNG = (BLR * DNC) * ((LL * (DMY + DMS)) - BH);
                            let DNH = 9.899494936611664e0f64 - DNG;
                            let DNI = DNH * DNH;
                            let DNJ = if DNE < (DNI * BLV) { 1.0 } else { 0.0 };
                            let DNM = if DNJ != 0.0 {
                                let DNK = ((-9.899494936611664e0f64 + DNH) + ((K * DNE) / DNH)) + DNG;
                                DNK
                            } else {
                                let DNL = (-9.899494936611664e0f64 + ((DNE + DNI).sqrt())) + DNG;
                                DNL
                            };
                            let DNN = DNM.powf(AAT);
                            let DNO = ((((((-5.65685424949238e0f64 - (BMB * DNC)) + (BH * DNN)) + ((MP * DNN) * DNN)) / DNN) * LN) - DMS) + DMS;
                            let DNP = DNO / DNF;
                            let DNQ = CO * (DMY - ((DNO / ((B + (DNP * DNP)).sqrt())) - DMS));
                            DRY = DNQ;
                            DXW = A;
                        } else {
                            let DNR = DMY + DMS;
                            let DNS = (LL * DNR) - B;
                            let DNT = DMU * LM;
                            let DNU = B + ((BN * (DNS + 4.9787068367863944e-2f64)) / DNT);
                            let DNV = if DNU < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DNY = if DNV != 0.0 {
                                DNW
                            } else {
                                DNU
                            };
                            let DNX = (DMU * LL) / BH;
                            let DNZ = B + ((BN * (DNS + ((-(LL * ((DMY + (DNX * (B - (DNY.sqrt())))) + DMS))).exp()))) / DNT);
                            let DOA = if DNZ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DOC = if DOA != 0.0 {
                                DOB
                            } else {
                                DNZ
                            };
                            let DOD = LL * ((DMY + (DNX * (B - (DOC.sqrt())))) + DMS);
                            let DOE = if DOD < BT { 1.0 } else { 0.0 };
                            let DOZ = if DOE != 0.0 {
                                let DOF = 7.071067811865476e-1f64 + (B / (LL * DMT));
                                let DOG = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DOF) / 5.286687693921294e-4f64)) + (((-DNR) / DMT) / 1.8773541122053122e-2f64);
                                let DOH = ((2.8160311683079683e-2f64 * DOF) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DOI = ((DOG * DOG) + ((DOH * DOH) * DOH)).sqrt();
                                let DOJ = LL * ((((((((-DOG) + DOI).powf(AAT)) + (-((DOG + DOI).powf(AAT)))) - -3.7209791878387604e0f64) * LN) - DMS) + DMS);
                                DOJ
                            } else {
                                DOD
                            };
                            let DOL = if DOK > A { 1.0 } else { 0.0 };
                            let DPG;
                            if DOL != 0.0 {
                                let DOM = MG / AD;
                                let DON = DOM * DOM;
                                let DOO = LL * (DNR + BI);
                                let DOP = (DON * (((LL * DNA).exp()) + GF)) * DNT;
                                let DOQ = (DON * DNT).ln();
                                let DOR = LL * DMS;
                                let DOS = (DOO - ((((DOP + (DOO * DOO)).ln()) - DOQ) + DOR)) - B;
                                let DOT = BN * DOO;
                                let DOU = if DOT > A { 1.0 } else { 0.0 };
                                let DOW = if DOU != 0.0 {
                                    DOT
                                } else {
                                    let DOV = -DOT;
                                    DOV
                                };
                                let DOX = (DOO - (DOO - (K * (DOS + (((DOS * DOS) + DOW).sqrt()))))) + (LL * BI);
                                let DOY = (((DOP + (DOX * DOX)).ln()) - DOQ) + DOR;
                                let DPA = (DOY - DOZ) - 6.0000000000000005e-2f64;
                                let DPB = (BN * DOY) * 6.0000000000000005e-2f64;
                                let DPC = if DPB > A { 1.0 } else { 0.0 };
                                let DPE = if DPC != 0.0 {
                                    DPB
                                } else {
                                    let DPD = -DPB;
                                    DPD
                                };
                                let DPF = DOY - (K * (DPA + (((DPA * DPA) + DPE).sqrt())));
                                DPG = DPF;
                            } else {
                                DPG = DOZ;
                            }
                            let DPH = (DPG / LL) - DMS;
                            let DPI = if ((DPG - B) + ((-DPG).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DPI != 0.0 {
                            } else {
                            }
                            let DPJ = CO * (DMY - DPH);
                            let DPK = if DOK == B { 1.0 } else { 0.0 };
                            let DRZ;
                            let DXX;
                            if DPK != 0.0 {
                                let DPL = (LL * DNA).exp();
                                let DPM = MG / AD;
                                let DPN = DPM * DPM;
                                let DPO = DPN * DPL;
                                let mut DPP = 0.0;
                                let mut DPR = 0.0;
                                let mut DQQ = 0.0;
                                let mut DRM = 0.0;
                                let mut DRP = 0.0;
                                let mut DRU = 0.0;
                                let mut DRV = 0.0;
                                DPP = B;
                                DPR = DPH;
                                DQQ = A;
                                DRM = DPG;
                                DRP = A;
                                DRU = A;
                                DRV = A;
                                loop {
                                    let DPQ = if DPP <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DPQ == 0.0 {
                                        break;
                                    }
                                    let DPS = LL * (DPR + DMS);
                                    let DPT = if DPS < LA { 1.0 } else { 0.0 };
                                    let DQM;
                                    let DQO;
                                    let DRQ;
                                    let DRW;
                                    if DPT != 0.0 {
                                        let DPU = DPS * DPS;
                                        let DPV = (DPU * DPS) * (DCN + (DPS * (-7.053654284009761e-2f64 + (DPS * DCO))));
                                        let DPW = DPS * LA;
                                        let DPX = (DPO * DPV) * DPV;
                                        let DPY = DPS * (DCS + (DPS * (-1.17851130197758e-1f64 + (DPS * (DCT + (DPS * (-1.63730162779191e-3f64 + (DPS * DCU))))))));
                                        let DPZ = (((DPY * DPY) + DPX) + GF).sqrt();
                                        let DQA = ((((LL * (DCS + (DPS * (-2.35702260395516e-1f64 + (DPS * (5.3640151901649905e-2f64 + (DPS * (-6.54920651116764e-3f64 + (DPW * DCU))))))))) * BH) * DPY) + ((((DPO * LL) * BH) * DPV) * (DPU * (8.907946456731299e-1f64 + (DPS * (-2.8214617136039044e-1f64 + (DPW * DCO))))))) / (DPZ + DPZ);
                                        DQM = DPZ;
                                        DQO = DQA;
                                        DRQ = DPY;
                                        DRW = DPX;
                                    } else {
                                        let DQB = if DPS < ARE { 1.0 } else { 0.0 };
                                        let DQI;
                                        let DQK;
                                        if DQB != 0.0 {
                                            let DQC = DPS.exp();
                                            let DQD = DPO * (DQC - B);
                                            let DQE = (DPO * LL) * DQC;
                                            DQI = DQD;
                                            DQK = DQE;
                                        } else {
                                            let DQF = (LL * DPR).exp();
                                            let DQG = DPN * (DQF - DPL);
                                            let DQH = (DPN * LL) * DQF;
                                            DQI = DQG;
                                            DQK = DQH;
                                        }
                                        let DQJ = ((DPS - B) + DQI).sqrt();
                                        let DQL = ((LL + DQK) / DQJ) * K;
                                        DQM = DQJ;
                                        DQO = DQL;
                                        DRQ = A;
                                        DRW = DQI;
                                    }
                                    let DQN = (DMY - DPR) - (DMT * DQM);
                                    let DQP = -1e0f64 - (DMT * DQO);
                                    let DQR = if DQQ == B { 1.0 } else { 0.0 };
                                    let DRG;
                                    let DRI;
                                    let DRJ;
                                    if DQR != 0.0 {
                                        DRG = DQS;
                                        DRI = DPR;
                                        DRJ = DQQ;
                                    } else {
                                        let DQT = (-DQN) / DQP;
                                        let DQU = DPR.abs();
                                        let DQV = if B >= DQU { 1.0 } else { 0.0 };
                                        let DQW = if DQV != 0.0 {
                                            B
                                        } else {
                                            DQU
                                        };
                                        let DQX = 5e-2f64 * (B + DQW);
                                        let DQY = if (DQT.abs()) > DQX { 1.0 } else { 0.0 };
                                        let DRD;
                                        if DQY != 0.0 {
                                            let DQZ = if DQT >= A { 1.0 } else { 0.0 };
                                            let DRB = if DQZ != 0.0 {
                                                B
                                            } else {
                                                DRA
                                            };
                                            let DRC = DQX * DRB;
                                            DRD = DRC;
                                        } else {
                                            DRD = DQT;
                                        }
                                        let DRE = DPR + DRD;
                                        let DRF = if (if (DRD.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (DQN.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DRK = if DRF != 0.0 {
                                            B
                                        } else {
                                            DQQ
                                        };
                                        DRG = DPP;
                                        DRI = DRE;
                                        DRJ = DRK;
                                    }
                                    let DRH = DRG + B;
                                    DPP = DRH;
                                    DPR = DRI;
                                    DQQ = DRJ;
                                    DRM = DPS;
                                    DRP = DRQ;
                                    DRU = DQM;
                                    DRV = DRW;
                                }
                                let DRL = if DQQ == A { 1.0 } else { 0.0 };
                                if DRL != 0.0 {
                                } else {
                                }
                                let DRN = if DRM < LA { 1.0 } else { 0.0 };
                                let DRT;
                                if DRN != 0.0 {
                                    let DRO = if DRM < BT { 1.0 } else { 0.0 };
                                    if DRO != 0.0 {
                                    } else {
                                    }
                                    let DRR = DRP + 2.220446049250313e-15f64;
                                    DRT = DRR;
                                } else {
                                    let DRS = (DRM - B).sqrt();
                                    DRT = DRS;
                                }
                                let DRX = (DLY * DRT) + ((DLY * DRV) * (B / (DRU + DRT)));
                                DRZ = DRX;
                                DXX = DRP;
                            } else {
                                DRZ = DPJ;
                                DXX = A;
                            }
                            DRY = DRZ;
                            DXW = DXX;
                        }
                        let DSC = if JD != 0.0 {
                            let DSA = DLP * DLL;
                            DSA
                        } else {
                            let DSB = DT * DLL;
                            DSB
                        };
                        let DSE = if (if DSD != 0.0 && H != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DLZ != 0.0 && JD != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJJ = if DSE != 0.0 {
                            let DSF = DSC * DRY;
                            DSF
                        } else {
                            A
                        };
                        let DSH = if (if DSG != 0.0 && H != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DMA != 0.0 && JD != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJO = if DSH != 0.0 {
                            let DSI = DSC * DRY;
                            DSI
                        } else {
                            A
                        };
                        let DSU;
                        let DTD;
                        let DYL;
                        let DYO;
                        if JD != 0.0 {
                            let DSL = (CYX * PD) + (CYY * (PD - OT));
                            let DSM = ((CYX * OZ) + (CYY * (OZ - OT))) - DSL;
                            let DSN = (DSJ * CYX) + CYY;
                            let DSO = (DSJ * CYY) + CYX;
                            let DSP = ((DSN * (-DSL)) + (DSO * (((CYX * OT) + (CYY * (-OT))) - DSL))) + 2.220446049250313e-15f64;
                            DSU = DSP;
                            DTD = DSM;
                            DYL = DSN;
                            DYO = DSO;
                        } else {
                            let DSQ = (DSJ * CYX) + CYY;
                            let DSR = (DSJ * CYY) + CYX;
                            let DTF = if DSJ != 0.0 {
                                let DSS = (CYX * OZ) + (CYY * (OZ - OT));
                                DSS
                            } else {
                                DMV
                            };
                            let DTE = if DSK != 0.0 {
                                let DST = (CYY * OZ) + (CYX * (OZ - OT));
                                DST
                            } else {
                                DTF
                            };
                            DSU = A;
                            DTD = DTE;
                            DYL = DSQ;
                            DYO = DSR;
                        }
                        let DSV = -DSU;
                        let DSW = if DSV > NP { 1.0 } else { 0.0 };
                        let DTB = if DSW != 0.0 {
                            let DSX = NL - NP;
                            let DSY = (DSV - NP) / DSX;
                            let DSZ = DSY * DSY;
                            let DTA = NP + (DSX * (B - (B / ((((B + DSY) + DSZ) + (DSZ * DSY)) + (DSZ * DSZ)))));
                            DTA
                        } else {
                            DSV
                        };
                        let DTC = (-DTB) - I;
                        let DTG = (-DTD) + AY;
                        let DTH = -DTC;
                        let DTI = if DTG < DTH { 1.0 } else { 0.0 };
                        let DYG;
                        if DTI != 0.0 {
                            let DTJ = (B / (LL * DLY)) * CO;
                            let DTK = BH + (4.242640687119285e0f64 * DTJ);
                            let DTL = ((BO * DTK) * DTK) * DTK;
                            let DTM = LK - DMZ;
                            let DTN = (BLR * DTJ) * ((LL * (DTG + DTC)) - BH);
                            let DTO = 9.899494936611664e0f64 - DTN;
                            let DTP = DTO * DTO;
                            let DTQ = if DTL < (DTP * BLV) { 1.0 } else { 0.0 };
                            let DTT = if DTQ != 0.0 {
                                let DTR = ((-9.899494936611664e0f64 + DTO) + ((K * DTL) / DTO)) + DTN;
                                DTR
                            } else {
                                let DTS = (-9.899494936611664e0f64 + ((DTL + DTP).sqrt())) + DTN;
                                DTS
                            };
                            let DTU = DTT.powf(AAT);
                            let DTV = ((((((-5.65685424949238e0f64 - (BMB * DTJ)) + (BH * DTU)) + ((MP * DTU) * DTU)) / DTU) * LN) - DTC) + DTC;
                            let DTW = DTV / DTM;
                            let DTX = CO * (DTG - ((DTV / ((B + (DTW * DTW)).sqrt())) - DTC));
                            DYG = DTX;
                        } else {
                            let DTY = DTG + DTC;
                            let DTZ = (LL * DTY) - B;
                            let DUA = DMU * LM;
                            let DUB = B + ((BN * (DTZ + 4.9787068367863944e-2f64)) / DUA);
                            let DUC = if DUB < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DUF = if DUC != 0.0 {
                                DUD
                            } else {
                                DUB
                            };
                            let DUE = (DMU * LL) / BH;
                            let DUG = B + ((BN * (DTZ + ((-(LL * ((DTG + (DUE * (B - (DUF.sqrt())))) + DTC))).exp()))) / DUA);
                            let DUH = if DUG < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DUJ = if DUH != 0.0 {
                                DUI
                            } else {
                                DUG
                            };
                            let DUK = LL * ((DTG + (DUE * (B - (DUJ.sqrt())))) + DTC);
                            let DUL = if DUK < BT { 1.0 } else { 0.0 };
                            let DVF = if DUL != 0.0 {
                                let DUM = 7.071067811865476e-1f64 + (B / (LL * DMT));
                                let DUN = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DUM) / 5.286687693921294e-4f64)) + (((-DTY) / DMT) / 1.8773541122053122e-2f64);
                                let DUO = ((2.8160311683079683e-2f64 * DUM) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DUP = ((DUN * DUN) + ((DUO * DUO) * DUO)).sqrt();
                                let DUQ = LL * ((((((((-DUN) + DUP).powf(AAT)) + (-((DUN + DUP).powf(AAT)))) - -3.7209791878387604e0f64) * LN) - DTC) + DTC);
                                DUQ
                            } else {
                                DUK
                            };
                            let DUR = if DOK > A { 1.0 } else { 0.0 };
                            let DVM;
                            if DUR != 0.0 {
                                let DUS = MG / AD;
                                let DUT = DUS * DUS;
                                let DUU = LL * (DTY + BI);
                                let DUV = (DUT * (((LL * DTH).exp()) + GF)) * DUA;
                                let DUW = (DUT * DUA).ln();
                                let DUX = LL * DTC;
                                let DUY = (DUU - ((((DUV + (DUU * DUU)).ln()) - DUW) + DUX)) - B;
                                let DUZ = BN * DUU;
                                let DVA = if DUZ > A { 1.0 } else { 0.0 };
                                let DVC = if DVA != 0.0 {
                                    DUZ
                                } else {
                                    let DVB = -DUZ;
                                    DVB
                                };
                                let DVD = (DUU - (DUU - (K * (DUY + (((DUY * DUY) + DVC).sqrt()))))) + (LL * BI);
                                let DVE = (((DUV + (DVD * DVD)).ln()) - DUW) + DUX;
                                let DVG = (DVE - DVF) - 6.0000000000000005e-2f64;
                                let DVH = (BN * DVE) * 6.0000000000000005e-2f64;
                                let DVI = if DVH > A { 1.0 } else { 0.0 };
                                let DVK = if DVI != 0.0 {
                                    DVH
                                } else {
                                    let DVJ = -DVH;
                                    DVJ
                                };
                                let DVL = DVE - (K * (DVG + (((DVG * DVG) + DVK).sqrt())));
                                DVM = DVL;
                            } else {
                                DVM = DVF;
                            }
                            let DVN = (DVM / LL) - DTC;
                            let DVO = if ((DVM - B) + ((-DVM).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DVO != 0.0 {
                            } else {
                            }
                            let DVP = CO * (DTG - DVN);
                            let DVQ = if DOK == B { 1.0 } else { 0.0 };
                            let DYH;
                            if DVQ != 0.0 {
                                let DVR = (LL * DTH).exp();
                                let DVS = MG / AD;
                                let DVT = DVS * DVS;
                                let DVU = DVT * DVR;
                                let mut DVV = 0.0;
                                let mut DVX = 0.0;
                                let mut DWW = 0.0;
                                let mut DXS = 0.0;
                                let mut DXV = 0.0;
                                let mut DYC = 0.0;
                                let mut DYD = 0.0;
                                DVV = B;
                                DVX = DVN;
                                DWW = A;
                                DXS = DVM;
                                DXV = DXW;
                                DYC = A;
                                DYD = A;
                                loop {
                                    let DVW = if DVV <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DVW == 0.0 {
                                        break;
                                    }
                                    let DVY = LL * (DVX + DTC);
                                    let DVZ = if DVY < LA { 1.0 } else { 0.0 };
                                    let DWS;
                                    let DWU;
                                    let DXY;
                                    let DYE;
                                    if DVZ != 0.0 {
                                        let DWA = DVY * DVY;
                                        let DWB = (DWA * DVY) * (DCN + (DVY * (-7.053654284009761e-2f64 + (DVY * DCO))));
                                        let DWC = DVY * LA;
                                        let DWD = (DVU * DWB) * DWB;
                                        let DWE = DVY * (DCS + (DVY * (-1.17851130197758e-1f64 + (DVY * (DCT + (DVY * (-1.63730162779191e-3f64 + (DVY * DCU))))))));
                                        let DWF = (((DWE * DWE) + DWD) + GF).sqrt();
                                        let DWG = ((((LL * (DCS + (DVY * (-2.35702260395516e-1f64 + (DVY * (5.3640151901649905e-2f64 + (DVY * (-6.54920651116764e-3f64 + (DWC * DCU))))))))) * BH) * DWE) + ((((DVU * LL) * BH) * DWB) * (DWA * (8.907946456731299e-1f64 + (DVY * (-2.8214617136039044e-1f64 + (DWC * DCO))))))) / (DWF + DWF);
                                        DWS = DWF;
                                        DWU = DWG;
                                        DXY = DWE;
                                        DYE = DWD;
                                    } else {
                                        let DWH = if DVY < ARE { 1.0 } else { 0.0 };
                                        let DWO;
                                        let DWQ;
                                        if DWH != 0.0 {
                                            let DWI = DVY.exp();
                                            let DWJ = DVU * (DWI - B);
                                            let DWK = (DVU * LL) * DWI;
                                            DWO = DWJ;
                                            DWQ = DWK;
                                        } else {
                                            let DWL = (LL * DVX).exp();
                                            let DWM = DVT * (DWL - DVR);
                                            let DWN = (DVT * LL) * DWL;
                                            DWO = DWM;
                                            DWQ = DWN;
                                        }
                                        let DWP = ((DVY - B) + DWO).sqrt();
                                        let DWR = ((LL + DWQ) / DWP) * K;
                                        DWS = DWP;
                                        DWU = DWR;
                                        DXY = A;
                                        DYE = DWO;
                                    }
                                    let DWT = (DTG - DVX) - (DMT * DWS);
                                    let DWV = -1e0f64 - (DMT * DWU);
                                    let DWX = if DWW == B { 1.0 } else { 0.0 };
                                    let DXM;
                                    let DXO;
                                    let DXP;
                                    if DWX != 0.0 {
                                        DXM = DWY;
                                        DXO = DVX;
                                        DXP = DWW;
                                    } else {
                                        let DWZ = (-DWT) / DWV;
                                        let DXA = DVX.abs();
                                        let DXB = if B >= DXA { 1.0 } else { 0.0 };
                                        let DXC = if DXB != 0.0 {
                                            B
                                        } else {
                                            DXA
                                        };
                                        let DXD = 5e-2f64 * (B + DXC);
                                        let DXE = if (DWZ.abs()) > DXD { 1.0 } else { 0.0 };
                                        let DXJ;
                                        if DXE != 0.0 {
                                            let DXF = if DWZ >= A { 1.0 } else { 0.0 };
                                            let DXH = if DXF != 0.0 {
                                                B
                                            } else {
                                                DXG
                                            };
                                            let DXI = DXD * DXH;
                                            DXJ = DXI;
                                        } else {
                                            DXJ = DWZ;
                                        }
                                        let DXK = DVX + DXJ;
                                        let DXL = if (if (DXJ.abs()) <= PJ { 1.0 } else { 0.0 }) != 0.0 && (if (DWT.abs()) <= BLV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DXQ = if DXL != 0.0 {
                                            B
                                        } else {
                                            DWW
                                        };
                                        DXM = DVV;
                                        DXO = DXK;
                                        DXP = DXQ;
                                    }
                                    let DXN = DXM + B;
                                    DVV = DXN;
                                    DVX = DXO;
                                    DWW = DXP;
                                    DXS = DVY;
                                    DXV = DXY;
                                    DYC = DWS;
                                    DYD = DYE;
                                }
                                let DXR = if DWW == A { 1.0 } else { 0.0 };
                                if DXR != 0.0 {
                                } else {
                                }
                                let DXT = if DXS < LA { 1.0 } else { 0.0 };
                                let DYB;
                                if DXT != 0.0 {
                                    let DXU = if DXS < BT { 1.0 } else { 0.0 };
                                    if DXU != 0.0 {
                                    } else {
                                    }
                                    let DXZ = DXV + 2.220446049250313e-15f64;
                                    DYB = DXZ;
                                } else {
                                    let DYA = (DXS - B).sqrt();
                                    DYB = DYA;
                                }
                                let DYF = (DLY * DYB) + ((DLY * DYD) * (B / (DYC + DYB)));
                                DYH = DYF;
                            } else {
                                DYH = DVP;
                            }
                            DYG = DYH;
                        }
                        let DYK = if JD != 0.0 {
                            let DYI = DLP * DLL;
                            DYI
                        } else {
                            let DYJ = DT * DLL;
                            DYJ
                        };
                        let DYM = if (if DYL != 0.0 && H != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DSJ != 0.0 && JD != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJI = if DYM != 0.0 {
                            let DYN = DYK * DYG;
                            DYN
                        } else {
                            EJJ
                        };
                        let DYP = if (if DYO != 0.0 && H != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DSK != 0.0 && JD != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJN = if DYP != 0.0 {
                            let DYQ = DYK * DYG;
                            DYQ
                        } else {
                            EJO
                        };
                        DYY = A;
                        DZF = A;
                        EJH = EJI;
                        EJM = EJN;
                    }
                    let DYR = (CYY * GM) + (CYX * GL);
                    let EFY;
                    if DYR != 0.0 {
                        let DYU = (CYY * DYS) + (CYX * DYT);
                        let DYZ = if JD != 0.0 {
                            let DYW = DYU * (-((CYY * DLP) + (CYX * DYV)));
                            DYW
                        } else {
                            let DYX = DYU * (-DT);
                            DYX
                        };
                        let DZA = DYY + ((-DYZ) * (OZ - OT));
                        EFY = DZA;
                    } else {
                        EFY = DYY;
                    }
                    let DZB = (CYX * GM) + (CYY * GL);
                    let EGC;
                    if DZB != 0.0 {
                        let DZC = (CYX * DYS) + (CYY * DYT);
                        let DZG = if JD != 0.0 {
                            let DZD = DZC * (-((CYX * DLP) + (CYY * DYV)));
                            DZD
                        } else {
                            let DZE = DZC * (-DT);
                            DZE
                        };
                        let DZH = DZF + ((-DZG) * OZ);
                        EGC = DZH;
                    } else {
                        EGC = DZF;
                    }
                    EFX = EFY;
                    EGB = EGC;
                    EJG = EJH;
                    EJL = EJM;
                } else {
                    let DZJ = if DZI == B { 1.0 } else { 0.0 };
                    let DZK = if GL == 0.0 { 1.0 } else { 0.0 };
                    let DZL = if DZI != B { 1.0 } else { 0.0 };
                    let DZM = if GM == 0.0 { 1.0 } else { 0.0 };
                    let DZN = if (if DZJ != 0.0 && DZK != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DZL != 0.0 && DZM != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DZT;
                    if DZN != 0.0 {
                        let DZU = if JD != 0.0 {
                            let DZO = ((-CO) * DLL) * DYV;
                            DZO
                        } else {
                            let DZP = ((-CO) * DLL) * DT;
                            DZP
                        };
                        DZT = DZU;
                    } else {
                        let DZQ = (CYY * DYS) + (CYX * DYT);
                        let DZV = if JD != 0.0 {
                            let DZR = DZQ * (-((CYY * DLP) + (CYX * DYV)));
                            DZR
                        } else {
                            let DZS = DZQ * (-DT);
                            DZS
                        };
                        DZT = DZV;
                    }
                    let DZW = (-DZT) * (OZ - OT);
                    let DZX = if (if DZJ != 0.0 && DZM != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DZL != 0.0 && DZK != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EAD;
                    if DZX != 0.0 {
                        let EAE = if JD != 0.0 {
                            let DZY = ((-CO) * DLL) * DLP;
                            DZY
                        } else {
                            let DZZ = ((-CO) * DLL) * DT;
                            DZZ
                        };
                        EAD = EAE;
                    } else {
                        let EAA = (CYX * DYS) + (CYY * DYT);
                        let EAF = if JD != 0.0 {
                            let EAB = EAA * (-((CYX * DLP) + (CYY * DYV)));
                            EAB
                        } else {
                            let EAC = EAA * (-DT);
                            EAC
                        };
                        EAD = EAF;
                    }
                    let EAG = (-EAD) * OZ;
                    EFX = DZW;
                    EGB = EAG;
                    EJG = A;
                    EJL = A;
                }
                EFW = EFX;
                EGA = EGB;
                EJF = EJG;
                EJK = EJL;
            } else {
                EFW = A;
                EGA = A;
                EJF = A;
                EJK = A;
            }
            if JD != 0.0 {
                let EAK = parameters[173] * (((((CI * EH) - (LK * LL)) + (parameters[175] * (LV.ln()))) / EAJ).exp());
                let EAN = EAJ / LL;
                let EAO = parameters[177] * (LV * LV);
                let EAP = EAN * ((B + (EAO / (((EAM * J) * EAK) + GF))).ln());
                let EAQ = if EAH < (EAN * ((B + (EAO / (((EAL * J) * EAK) + GF))).ln())) { 1.0 } else { 0.0 };
                if EAQ != 0.0 {
                } else {
                }
                let EAR = if EAI < EAP { 1.0 } else { 0.0 };
                if EAR != 0.0 {
                } else {
                }
                let EAU = EAS * EAT;
                let EAW = EAS * EAV;
                let EAX = J - parameters[238];
                let EAY = if EAX <= A { 1.0 } else { 0.0 };
                let EBG;
                let ECB;
                if EAY != 0.0 {
                    EBG = A;
                    ECB = A;
                } else {
                    EBG = EAW;
                    ECB = EAU;
                }
                let EBA = if EAZ > DLP { 1.0 } else { 0.0 };
                if EBA != 0.0 {
                    let EBC = EBB * (EAZ - DLP);
                    let EBE = EBD * DLP;
                    let EBF = if EAI < A { 1.0 } else { 0.0 };
                    if EBF != 0.0 {
                        let EBH = if EBG > A { 1.0 } else { 0.0 };
                        if EBH != 0.0 {
                            let EBJ = if EBI == K { 1.0 } else { 0.0 };
                            if EBJ != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let EBK = if EBC > A { 1.0 } else { 0.0 };
                        if EBK != 0.0 {
                            let EBM = if EBL == K { 1.0 } else { 0.0 };
                            if EBM != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let EBN = if EBE > A { 1.0 } else { 0.0 };
                        if EBN != 0.0 {
                            let EBP = if EBO == K { 1.0 } else { 0.0 };
                            if EBP != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                    let EBQ = EBD * EAZ;
                    let EBR = if EAI < A { 1.0 } else { 0.0 };
                    if EBR != 0.0 {
                        let EBS = if EBG > A { 1.0 } else { 0.0 };
                        if EBS != 0.0 {
                            let EBT = if EBI == K { 1.0 } else { 0.0 };
                            if EBT != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let EBU = if EBQ > A { 1.0 } else { 0.0 };
                        if EBU != 0.0 {
                            let EBV = if EBO == K { 1.0 } else { 0.0 };
                            if EBV != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                }
                let EBX = if EBW > DYV { 1.0 } else { 0.0 };
                if EBX != 0.0 {
                    let EBY = EBB * (EBW - DYV);
                    let EBZ = EBD * DYV;
                    let ECA = if EAH < A { 1.0 } else { 0.0 };
                    if ECA != 0.0 {
                        let ECC = if ECB > A { 1.0 } else { 0.0 };
                        if ECC != 0.0 {
                            let ECD = if EBI == K { 1.0 } else { 0.0 };
                            if ECD != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let ECE = if EBY > A { 1.0 } else { 0.0 };
                        if ECE != 0.0 {
                            let ECF = if EBL == K { 1.0 } else { 0.0 };
                            if ECF != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let ECG = if EBZ > A { 1.0 } else { 0.0 };
                        if ECG != 0.0 {
                            let ECH = if EBO == K { 1.0 } else { 0.0 };
                            if ECH != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                    let ECI = EBD * EBW;
                    let ECJ = if EAH < A { 1.0 } else { 0.0 };
                    if ECJ != 0.0 {
                        let ECK = if ECB > A { 1.0 } else { 0.0 };
                        if ECK != 0.0 {
                            let ECL = if EBI == K { 1.0 } else { 0.0 };
                            if ECL != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let ECM = if ECI > A { 1.0 } else { 0.0 };
                        if ECM != 0.0 {
                            let ECN = if EBO == K { 1.0 } else { 0.0 };
                            if ECN != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                }
                let ECO = if EBG > A { 1.0 } else { 0.0 };
                if ECO != 0.0 {
                    let ECP = -(((-1.6021918e-19f64 * IA) * EAX) * EAV);
                    let ECQ = if ((BN * ECP) * (IO * ECP)) > A { 1.0 } else { 0.0 };
                    if ECQ != 0.0 {
                    } else {
                    }
                } else {
                }
                let ECR = if ECB > A { 1.0 } else { 0.0 };
                if ECR != 0.0 {
                    let ECS = -(((-1.6021918e-19f64 * IA) * EAX) * EAT);
                    let ECT = if ((BN * ECS) * (IO * ECS)) > A { 1.0 } else { 0.0 };
                    if ECT != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let EPW;
            let EPZ;
            if BC != 0.0 {
                let EPX = if CVD != 0.0 {
                    let ECX = (((ECU * ECV) * ECW) * ECW) / ((((CVQ * CHX) * ECU) + ((ECV * ECW) * ECW)) + GF);
                    ECX
                } else {
                    let ECY = ECU + GF;
                    ECY
                };
                let ECZ = parameters[235] * TM;
                EPW = EPX;
                EPZ = ECZ;
            } else {
                EPW = A;
                EPZ = A;
            }
            let EDA = if CAK == 0.0 { 1.0 } else { 0.0 };
            let EDB = if (if parameters[31] != A { 1.0 } else { 0.0 }) != 0.0 && EDA != 0.0 { 1.0 } else { 0.0 };
            let EKI;
            if EDB != 0.0 {
                let EDC = CAZ / EF;
                let EDD = (((TM + (CAZ / (CAV - SL))) + X) * LN) / EF;
                let EDG = ((((-2e0f64 * EDE) / EF) / EDF) / DT) - EDC;
                let EDH = EDG - EDC;
                let EDI = if (EDH.abs()) > 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let EDO = if EDI != 0.0 {
                    let EDJ = EDC + EDD;
                    let EDK = EDG + EDD;
                    let EDL = (((B / EDJ) / EDK) + (((((BH * U) * CVY) * CVQ) / EDH) * ((EDK / EDJ).ln()))) + (((((U * CVY) * CVQ) * U) * CVY) * CVQ);
                    EDL
                } else {
                    let EDM = EDC + EDD;
                    let EDN = (((B / EDM) / (EDG + EDD)) + ((((BH * U) * CVY) * CVQ) / EDM)) + (((((U * CVY) * CVQ) * U) * CVY) * CVQ);
                    EDN
                };
                let EDP = (((CUH * CUH) * W) / ((ECW * LL) * DR)) * EDO;
                EKI = EDP;
            } else {
                EKI = A;
            }
            let EDQ = if CHV != A { 1.0 } else { 0.0 };
            let EDR = if EDQ != 0.0 && EDA != 0.0 { 1.0 } else { 0.0 };
            let EFC;
            let EKU;
            if EDR != 0.0 {
                let EEC = (EEB * ((EDS - CAV) / ECW)) / 1e5f64;
                let EED = if (if 9.999999999999978e-1f64 <= CDY { 1.0 } else { 0.0 }) != 0.0 && (if CDY <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EEG;
                if EED != 0.0 {
                    EEG = B;
                } else {
                    let EEE = if (if 1.9999999999999978e0f64 <= CDY { 1.0 } else { 0.0 }) != 0.0 && (if CDY <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EEH = if EEE != 0.0 {
                        EEC
                    } else {
                        let EEF = EEC.powf((CDY - B));
                        EEF
                    };
                    EEG = EEH;
                }
                let EEI = B + (EEC * EEG);
                let EEJ = EEB * (EEI * (EEI.powf(((-1e0f64 / CDY) - B))));
                let EEK = (CVQ + EEJ) / BH;
                let EEL = BZZ * BZZ;
                let EEM = BT * BZZ;
                let EEN = ((((DR * TM) * CHX) * CVQ) * ((((((B + EEM) + (LC * EEL)) * EEJ) * EEJ) + ((((BT + (BN * BZZ)) + (BT * EEL)) * EEJ) * CVQ)) + ((((LC + EEM) + EEL) * CVQ) * CVQ))) / ((((1.5e1f64 * ECW) * (B + BZZ)) * EEK) * EEK);
                EFC = EEN;
                EKU = EEJ;
            } else {
                EFC = A;
                EKU = A;
            }
            let EER = if (if (if (if CHU != A { 1.0 } else { 0.0 }) != 0.0 && EDQ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if EEO == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EDA != 0.0 { 1.0 } else { 0.0 };
            let EKQ;
            let EKV;
            let EKY;
            let ELB;
            if EER != 0.0 {
                let EEU = EES.sqrt();
                let EEV = CHX + EEU;
                let EEY = (((4.2e1f64 * EEW) * EES) + (BN * ((EEW * EEW) + (EES * EES)))) + (((OR * EEU) * CHX) * (EEW + EES));
                let EEZ = EEV * EEV;
                let EFA = EEY / ((EEZ * EEZ) * EEV);
                let EFB = ((DR / ECW) * CVQ) * TM;
                let EFF = ((3.872983346207417e0f64 * EFD) * ((EEW + ((BN * CHX) * EEU)) + EES)) / ((LC * EEV) * (((((EFC / (EFB * CHX)) * EEV) * CHX) * EEY).sqrt()));
                EKQ = EFB;
                EKV = EEU;
                EKY = EFA;
                ELB = EFF;
            } else {
                EKQ = I;
                EKV = A;
                EKY = A;
                ELB = A;
            }
            let EJA;
            let EJB;
            let EJC;
            if JD != 0.0 {
                let EFM = EFG + EFJ;
                let EFP = if GK != 0.0 {
                    let EFO = EFM - (EFN * CZ);
                    EFO
                } else {
                    EFM
                };
                let EFQ = OZ - PV;
                let EFS = 2.1983327444149834e-11f64 * ((B + (EFR / CJ)).ln());
                let EFT = EFS * DB;
                let EFZ = EFW + ((EFT * (DC + EFU)) * (OZ - OT));
                let EGD = EGA + ((EFT * (DC + EFV)) * OZ);
                let EGE = ((-EFP) * EFQ) + (((EFS * JG) * DB) * EFQ);
                EJA = EFZ;
                EJB = EGD;
                EJC = EGE;
            } else {
                let EJD = if GK != 0.0 {
                    let EGF = (-((-EFN) * CZ)) * (OZ - PV);
                    EGF
                } else {
                    A
                };
                let EGG = ((2.1983327444149834e-11f64 * DC) * DB) * ((B + (EFR / CJ)).ln());
                let EGH = EFW + (EGG * (OZ - OT));
                let EGI = EGA + (EGG * OZ);
                EJA = EGH;
                EJB = EGI;
                EJC = EJD;
            }
            let EIY;
            if BC != 0.0 {
                if JD != 0.0 {
                } else {
                }
                EIY = A;
            } else {
                let EIZ = if JD != 0.0 {
                    let EGZ = (-EGJ) - EDE;
                    EGZ
                } else {
                    let EHA = (((-EGN) - EDE) - EGV) - EGR;
                    EHA
                };
                EIY = EIZ;
            }
            let EHB = if DLD == A { 1.0 } else { 0.0 };
            let EHL;
            if EHB != 0.0 {
                EHL = A;
            } else {
                let EHF = (EHC * CW) + CAV;
                let EHG = if EHF > EDS { 1.0 } else { 0.0 };
                let EHI = if EHG != 0.0 {
                    EDS
                } else {
                    EHF
                };
                let EHH = OT + CAV;
                let EHJ = (((EHH - ((CBC * EHH) + ((B - CBC) * EHI))) / DLD) - EHC) * ((CK * DT) * (((2.069886e-10f64 / IF).sqrt()) * 1.3e0f64));
                EHL = EHJ;
            }
            let EHK = if FZ != A { 1.0 } else { 0.0 };
            let EJE = if EHK != 0.0 {
                let EHM = EHL + (GA * PV);
                EHM
            } else {
                EHL
            };
            let EHN = if JE == B { 1.0 } else { 0.0 };
            let EKJ;
            if EHN != 0.0 {
                let EKK = if JD != 0.0 {
                    let EJP = EIY + ((((((EJA + EJB) + EJC) - EJE) - EJF) - EJK) + ((((-EHO) - EHV) - EIC) - EIN));
                    EJP
                } else {
                    let EJQ = EIY + (((((EJA + EJB) + EJC) - EJE) - EJF) - EJK);
                    EJQ
                };
                EKJ = EKK;
            } else {
                EKJ = EIY;
            }
            if JD != 0.0 {
            } else {
            }
            let EJR = if AFV != B { 1.0 } else { 0.0 };
            if EJR != 0.0 {
            } else {
            }
            let EJU = -EJS;
            let EJV = if DZI == B { 1.0 } else { 0.0 };
            let EQD = if EJV != 0.0 {
                let EKC = (EJW * EJX) - EKA;
                EKC
            } else {
                let EKF = ((B - EJW) * EJX) - EKD;
                EKF
            };
            let EQE = if EJV != 0.0 {
                let EKG = ((B - EJW) * EJX) - EKD;
                EKG
            } else {
                let EKH = (EJW * EJX) - EKA;
                EKH
            };
            if EJV != 0.0 {
            } else {
            }
            if EJV != 0.0 {
            } else {
            }
            let EKL = GI * 0e0f64;
            let EKM = GI * 0e0f64;
            let EKN = if DZI > A { 1.0 } else { 0.0 };
            let EKO = if EKN != 0.0 {
                EKM
            } else {
                EKL
            };
            let EQN;
            let EQO;
            if EER != 0.0 {
                let EKP = ((Q * TM) * DT) * CX;
                let EKR = (((1.898893985185185e-20f64 * LN) * EKO) * EKO) / EKQ;
                let EKS = if (if EFD > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if OT > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EKZ = if EKS != 0.0 {
                    let EKT = EEB / CVQ;
                    let EKW = EKT + (((6.666666666666667e-1f64 * (((EEB / EKU) - EKT) / OT)) * ((EEW + (CHX * EKV)) + EES)) / (CHX + EKV));
                    EKW
                } else {
                    let EKX = EEB / EKU;
                    EKX
                };
                let ELA = (EKR * EKY) * EKZ;
                let ELC = if (-EKO) > EKP { 1.0 } else { 0.0 };
                let ELD = if ELC != 0.0 && (if ELA > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ELE = if ELD != 0.0 {
                    ELA
                } else {
                    A
                };
                let ELF = if ELC != 0.0 {
                    ELB
                } else {
                    A
                };
                EQN = ELF;
                EQO = ELE;
            } else {
                EQN = A;
                EQO = A;
            }
            let ELH = if ELG == B { 1.0 } else { 0.0 };
            let EQR;
            if ELH != 0.0 {
                let EMG;
                let EMH;
                let EMK;
                let EMV;
                let EMW;
                let ENP;
                let ENT;
                if ELI != 0.0 {
                    let ELK = ELJ / Q;
                    let ELP = if ELO > A { 1.0 } else { 0.0 };
                    let ELS = if ELP != 0.0 {
                        let ELR = ELO * ELQ;
                        ELR
                    } else {
                        A
                    };
                    let ELU = GI * (KE - KK);
                    EMG = ELL;
                    EMH = ELM;
                    EMK = ELN;
                    EMV = ELU;
                    EMW = ELT;
                    ENP = ELK;
                    ENT = ELS;
                } else {
                    let ELY = if ELO > A { 1.0 } else { 0.0 };
                    let EMB = if ELY != 0.0 {
                        let EMA = ELO * ELZ;
                        EMA
                    } else {
                        A
                    };
                    let EMD = GI * (KJ - KD);
                    EMG = ELV;
                    EMH = ELW;
                    EMK = ELX;
                    EMV = EMD;
                    EMW = EMC;
                    ENP = AD;
                    ENT = EMB;
                }
                let EMF = ((EME * EME) + (CV * CV)).sqrt();
                let EMM = EMK + (EML * LJ);
                let EMT = ((EMG / JB) / (LV.powf(EMI))) * (B + (EMN / (DA.powf(EMO))));
                let EMU = ((((EMH / AZ) / (MB - (EMJ * MC))) * (B + (EMR / (DU.powf(EMS))))) * (B + (EMP / (DA.powf(EMQ))))) + GF;
                let EMX = EMT * (EMV / EMW);
                let EMY = if EMV >= A { 1.0 } else { 0.0 };
                let END = if EMY != 0.0 {
                    let EMZ = EMX / EMU;
                    EMZ
                } else {
                    let ENA = (-EMX) / EMU;
                    ENA
                };
                let ENB = if (if 9.999999999999978e-1f64 <= EMM { 1.0 } else { 0.0 }) != 0.0 && (if EMM <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ENF;
                if ENB != 0.0 {
                    ENF = B;
                } else {
                    let ENC = if (if 1.9999999999999978e0f64 <= EMM { 1.0 } else { 0.0 }) != 0.0 && (if EMM <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ENG = if ENC != 0.0 {
                        END
                    } else {
                        let ENE = END.powf((EMM - B));
                        ENE
                    };
                    ENF = ENG;
                }
                let ENH = B + (END * ENF);
                let ENI = if (if 9.999999999999978e-1f64 <= EMM { 1.0 } else { 0.0 }) != 0.0 && (if EMM <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ENN;
                if ENI != 0.0 {
                    let ENJ = B / ENH;
                    ENN = ENJ;
                } else {
                    let ENK = if (if 1.9999999999999978e0f64 <= EMM { 1.0 } else { 0.0 }) != 0.0 && (if EMM <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ENO = if ENK != 0.0 {
                        let ENL = B / (ENH.sqrt());
                        ENL
                    } else {
                        let ENM = ENH * (ENH.powf(((-1e0f64 / EMM) - B)));
                        ENM
                    };
                    ENN = ENO;
                }
                let ENQ = (((EF / EMW) * EMF) * (EMT * ENN)) * ENP;
                let ENR = if ENQ <= A { 1.0 } else { 0.0 };
                let ENS = if ENR != 0.0 {
                    GF
                } else {
                    ENQ
                };
                let ENU = ((B / ENS) / DR) + ENT;
                let ENV = if (if ENU > V { 1.0 } else { 0.0 }) != 0.0 && EDQ != 0.0 { 1.0 } else { 0.0 };
                let ENX = if ENV != 0.0 {
                    let ENW = B / ENU;
                    ENW
                } else {
                    A
                };
                let ENY = if ENU < V { 1.0 } else { 0.0 };
                if ENY != 0.0 {
                } else {
                }
                EQR = ENX;
            } else {
                EQR = A;
            }
            let EOA = if ENZ == B { 1.0 } else { 0.0 };
            let EQT;
            if EOA != 0.0 {
                let EOM;
                let EON;
                let EOO;
                let EOS;
                let EOT;
                let EPM;
                let EPQ;
                if EOB != 0.0 {
                    let EOC = ELJ / Q;
                    let EOD = if ELO > A { 1.0 } else { 0.0 };
                    let EOF = if EOD != 0.0 {
                        let EOE = ELO * ELQ;
                        EOE
                    } else {
                        A
                    };
                    let EOG = GI * (KE - KK);
                    EOM = ELL;
                    EON = ELM;
                    EOO = ELN;
                    EOS = EOG;
                    EOT = ELT;
                    EPM = EOC;
                    EPQ = EOF;
                } else {
                    let EOH = if ELO > A { 1.0 } else { 0.0 };
                    let EOJ = if EOH != 0.0 {
                        let EOI = ELO * ELZ;
                        EOI
                    } else {
                        A
                    };
                    let EOK = GI * (KJ - KD);
                    EOM = ELV;
                    EON = ELW;
                    EOO = ELX;
                    EOS = EOK;
                    EOT = EMC;
                    EPM = AD;
                    EPQ = EOJ;
                }
                let EOL = ((EME * EME) + (CV * CV)).sqrt();
                let EOP = EOO + (EML * LJ);
                let EOQ = ((EOM / JB) / (LV.powf(EMI))) * (B + (EMN / (DA.powf(EMO))));
                let EOR = ((((EON / AZ) / (MB - (EMJ * MC))) * (B + (EMR / (DU.powf(EMS))))) * (B + (EMP / (DA.powf(EMQ))))) + GF;
                let EOU = EOQ * (EOS / EOT);
                let EOV = if EOS >= A { 1.0 } else { 0.0 };
                let EPA = if EOV != 0.0 {
                    let EOW = EOU / EOR;
                    EOW
                } else {
                    let EOX = (-EOU) / EOR;
                    EOX
                };
                let EOY = if (if 9.999999999999978e-1f64 <= EOP { 1.0 } else { 0.0 }) != 0.0 && (if EOP <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EPC;
                if EOY != 0.0 {
                    EPC = B;
                } else {
                    let EOZ = if (if 1.9999999999999978e0f64 <= EOP { 1.0 } else { 0.0 }) != 0.0 && (if EOP <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EPD = if EOZ != 0.0 {
                        EPA
                    } else {
                        let EPB = EPA.powf((EOP - B));
                        EPB
                    };
                    EPC = EPD;
                }
                let EPE = B + (EPA * EPC);
                let EPF = if (if 9.999999999999978e-1f64 <= EOP { 1.0 } else { 0.0 }) != 0.0 && (if EOP <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EPK;
                if EPF != 0.0 {
                    let EPG = B / EPE;
                    EPK = EPG;
                } else {
                    let EPH = if (if 1.9999999999999978e0f64 <= EOP { 1.0 } else { 0.0 }) != 0.0 && (if EOP <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EPL = if EPH != 0.0 {
                        let EPI = B / (EPE.sqrt());
                        EPI
                    } else {
                        let EPJ = EPE * (EPE.powf(((-1e0f64 / EOP) - B)));
                        EPJ
                    };
                    EPK = EPL;
                }
                let EPN = (((EF / EOT) * EOL) * (EOQ * EPK)) * EPM;
                let EPO = if EPN <= A { 1.0 } else { 0.0 };
                let EPP = if EPO != 0.0 {
                    GF
                } else {
                    EPN
                };
                let EPR = ((B / EPP) / DR) + EPQ;
                let EPS = if (if EPR > V { 1.0 } else { 0.0 }) != 0.0 && EDQ != 0.0 { 1.0 } else { 0.0 };
                let EPU = if EPS != 0.0 {
                    let EPT = B / EPR;
                    EPT
                } else {
                    A
                };
                let EPV = if EPR < V { 1.0 } else { 0.0 };
                if EPV != 0.0 {
                } else {
                }
                EQT = EPU;
            } else {
                EQT = A;
            }
            if JD != 0.0 {
                if BC != 0.0 {
                    let EPY = if EPW < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EPY != 0.0 {
                    } else {
                    }
                    let EQA = if EPZ < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EQA != 0.0 {
                    } else {
                    }
                    if EJV != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                if BC != 0.0 {
                    let EQB = if EPW < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EQB != 0.0 {
                    } else {
                    }
                    let EQC = if EPZ < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EQC != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if EJV != 0.0 {
            } else {
            }
            if JD != 0.0 {
            } else {
            }
            let EQF = if (if KN == B { 1.0 } else { 0.0 }) != 0.0 && KP != 0.0 { 1.0 } else { 0.0 };
            if EQF != 0.0 {
            } else {
            }
            let EQG = if DZI != B { 1.0 } else { 0.0 };
            if EQG != 0.0 {
            } else {
            }
            if JD != 0.0 {
            } else {
            }
            let EQH = if BB >= BO { 1.0 } else { 0.0 };
            if EQH != 0.0 {
                if JD != 0.0 {
                } else {
                }
            } else {
            }
            let EQI = 5.5224904e-23f64 * LI;
            let EQJ = if CWC == B { 1.0 } else { 0.0 };
            if EQJ != 0.0 {
            } else {
            }
            if ELG != 0.0 {
            } else {
            }
            if ENZ != 0.0 {
            } else {
            }
            let EQK = DZI * EKI;
            let EQM = EQI * EFC;
            let EQP = if (if EQM > A { 1.0 } else { 0.0 }) != 0.0 && (if EQO > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EQP != 0.0 {
            } else {
            }
            let EQQ = (B - (EQN * EQN)) * EQM;
            if EKN != 0.0 {
            } else {
            }
            if EKN != 0.0 {
            } else {
            }
            let ERA;
            let ERB;
            if ELG != 0.0 {
                let EQS = EQI * EQR;
                ERA = B;
                ERB = EQS;
            } else {
                ERA = A;
                ERB = A;
            }
            let ERC;
            let ERD;
            if ENZ != 0.0 {
                let EQU = EQI * EQT;
                ERC = B;
                ERD = EQU;
            } else {
                ERC = A;
                ERD = A;
            }
            let ERE;
            let ERF;
            let ERG;
            let ERH;
            let ERI;
            let ERJ;
            if EQJ != 0.0 {
                let EQV = 3.2043836e-19f64 * EQD;
                let EQW = 3.2043836e-19f64 * EQE;
                let EQX = 3.2043836e-19f64 * EJU;
                ERE = B;
                ERF = EQV;
                ERG = B;
                ERH = EQW;
                ERI = B;
                ERJ = EQX;
            } else {
                ERE = A;
                ERF = A;
                ERG = A;
                ERH = A;
                ERI = A;
                ERJ = A;
            }
            if IS != 0.0 {
            } else {
            }
            let EQY = if KO != 0.0 && (if AB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EQY != 0.0 {
            } else {
            }
            if JD != 0.0 {
                if IV != 0.0 {
                } else {
                }
                if IY != 0.0 {
                } else {
                }
                if BC != 0.0 {
                } else {
                }
                let EQZ = if ALC != 0.0 || CUJ != 0.0 { 1.0 } else { 0.0 };
                if EQZ != 0.0 {
                } else {
                }
            } else {
                if ALC != 0.0 {
                } else {
                }
                if BC != 0.0 {
                } else {
                }
            }
            if H != 0.0 {
            } else {
            }
        {
            let psd = EQK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EQL);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = EQM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = EQQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERA == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERC == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERE == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERG == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERI == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
