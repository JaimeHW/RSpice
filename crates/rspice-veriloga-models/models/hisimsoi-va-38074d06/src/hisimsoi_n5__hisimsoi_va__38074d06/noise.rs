#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 8] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DP_SP_IFLICK", label: Some("iflick"), kind: GeneratedNoiseKind::Flicker, equation: 13, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N_GND_INTERNAL", label: Some("internal"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "n", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DP_SP_IDS", label: Some("ids"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SP_S_ISOURCE", label: Some("isource"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DP_IDRAIN", label: Some("idrain"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_DP_IIGD", label: Some("iigd"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_SP_IIGS", label: Some("iigs"), kind: GeneratedNoiseKind::White, equation: 23, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_BP_IIGB", label: Some("iigb"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
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
            let C = 0.0f64;
            let D = parameters[43];
            let G = 1e-12f64;
            let H = parameters[237];
            let I = 5e-1f64;
            let J = 1e1f64;
            let L = 2e2f64;
            let M = 1e-2f64;
            let O = 1e-6f64;
            let T = 1e-4f64;
            let W = parameters[240];
            let Z = parameters[242];
            let AG = parameters[83];
            let AI = parameters[84];
            let AK = parameters[85];
            let AM = parameters[80];
            let AO = parameters[81];
            let AQ = parameters[82];
            let AS = 1e6f64;
            let AU = 2.7315e2f64;
            let AW = parameters[58];
            let AX = 1e2f64;
            let AZ = parameters[46];
            let BA = parameters[34];
            let BB = if parameter_given[190] { 1.0 } else { 0.0 };
            let BC = parameters[190];
            let BF = 2e0f64;
            let BG = 1e-1f64;
            let BL = 4e0f64;
            let BM = 8e0f64;
            let BN = 1.0f64;
            let BO = 0.0f64;
            let BP = 1.0f64;
            let BQ = 0.0f64;
            let BR = 3e0f64;
            let BS = 0.0f64;
            let CF = 1e-7f64;
            let CH = parameters[236];
            let CI = 1.034943e-10f64;
            let CL = 3.453133e-11f64;
            let CO = parameters[239];
            let CS = parameters[0];
            let CT = parameters[56];
            let CZ = parameters[9];
            let DB = parameters[60];
            let DD = parameters[295];
            let DF = parameters[61];
            let DJ = parameters[18];
            let DW = parameters[72];
            let ED = 1.6021918e-19f64;
            let EE = 1.3806226e-23f64;
            let EH = parameters[244];
            let EK = parameters[248];
            let EO = parameters[89];
            let EQ = parameters[68];
            let EV = parameters[6];
            let EY = parameters[130];
            let EZ = parameters[131];
            let FB = parameters[124];
            let FC = parameters[125];
            let FD = parameters[126];
            let FF = parameters[123];
            let FH = parameters[117];
            let FI = parameters[119];
            let FJ = parameters[120];
            let FL = parameters[118];
            let FM = parameters[121];
            let FP = parameters[127];
            let FQ = parameters[128];
            let FR = parameters[129];
            let FX = parameters[65];
            let GC = parameters[114];
            let GD = 1e-50f64;
            let GG = parameters[50];
            let GI = if parameter_given[168] { 1.0 } else { 0.0 };
            let GJ = if parameter_given[169] { 1.0 } else { 0.0 };
            let GK = if parameter_given[170] { 1.0 } else { 0.0 };
            let GL = if parameter_given[294] { 1.0 } else { 0.0 };
            let GM = if parameter_given[23] { 1.0 } else { 0.0 };
            let GN = if parameter_given[22] { 1.0 } else { 0.0 };
            let GO = if parameter_given[16] { 1.0 } else { 0.0 };
            let GP = parameters[17];
            let GS = parameters[13];
            let GT = parameters[14];
            let GV = parameters[10];
            let GW = parameters[11];
            let GX = parameters[12];
            let HJ = parameters[161];
            let HK = parameters[163];
            let HU = parameters[164];
            let HV = parameters[166];
            let IM = 1e-3f64;
            let IN = 1e-10f64;
            let IQ = parameters[35];
            let IT = parameters[261];
            let IW = parameters[262];
            let IZ = 1e4f64;
            let JC = parameters[24];
            let JD = parameters[23];
            let JE = parameters[19];
            let JH = parameters[22];
            let KB = node_potentials[6];
            let KC = node_potentials[7];
            let KF = node_potentials[12];
            let KH = node_potentials[0];
            let KI = node_potentials[2];
            let KK = 1e-9f64;
            let KL = parameters[38];
            let KP = node_potentials[10];
            let KU = -1e0f64;
            let KY = 5e0f64;
            let LA = 6e0f64;
            let LC = temperature;
            let LP = parameters[160];
            let LY = 4e-1f64;
            let MN = 1.414213562373095e0f64;
            let NG = 8e-1f64;
            let NH = 1.2e0f64;
            let NV = 1.0f64;
            let NW = 0.0f64;
            let NX = 0.0f64;
            let NY = 1.0f64;
            let NZ = 0.0f64;
            let OP = 2e1f64;
            let OW = -2e1f64;
            let PA = -2e1f64;
            let PE = parameters[226];
            let PH = 5e-12f64;
            let PY = 5e-2f64;
            let QA = 2.0000000000000004e-2f64;
            let QB = 1.0f64;
            let QC = -2.0000000000000004e-2f64;
            let QI = parameters[204];
            let QJ = parameters[206];
            let QK = parameters[205];
            let RR = 2e-3f64;
            let RS = 1.0f64;
            let RT = -2e-3f64;
            let TB = parameters[69];
            let TE = parameters[71];
            let TH = parameters[86];
            let TW = 2.7e1f64;
            let UG = 2e-1f64;
            let UH = 1.0f64;
            let UI = -2e-1f64;
            let UR = 7e0f64;
            let VC = 1e-5f64;
            let VE = parameters[39];
            let VR = 2.220446049250313e-15f64;
            let VX = 8e-4f64;
            let YI = 1.0f64;
            let YJ = 0.0f64;
            let YK = 1.0f64;
            let YL = 0.0f64;
            let YM = 0.0f64;
            let ZG = 1.0f64;
            let ZH = 0.0f64;
            let ZI = 1.0f64;
            let ZJ = 0.0f64;
            let ZK = 0.0f64;
            let AAB = 0.0f64;
            let AAG = 2.220446049250313e-15f64;
            let AAL = 8.1e1f64;
            let AAO = 1.458e3f64;
            let AAP = 5.4e1f64;
            let AAR = 3.333333333333333e-1f64;
            let AAT = 1.259921049894873e0f64;
            let ABX = 9.8e-1f64;
            let ACE = 1.0f64;
            let ACF = 0.0f64;
            let ACG = 1.0f64;
            let ACH = 0.0f64;
            let ACI = 0.0f64;
            let ADE = 6e-1f64;
            let ADT = 2.220446049250313e-15f64;
            let AFT = parameters[25];
            let AFV = 2e-1f64;
            let AFY = parameters[137];
            let AGK = 3.0000000000000002e-2f64;
            let AGP = 2.220446049250313e-15f64;
            let AGW = 3e-2f64;
            let AHV = 2.5e-1f64;
            let AIR = 0e0f64;
            let AIS = parameters[122];
            let AIV = 0e0f64;
            let AJA = 0e0f64;
            let AJN = 1.0f64;
            let AJO = 0.0f64;
            let AJP = 0.0f64;
            let AJQ = 1.0f64;
            let AJR = 0.0f64;
            let AKS = parameters[26];
            let AKU = parameters[141];
            let AKX = parameters[140];
            let ALA = parameters[37];
            let ALB = node_potentials[17];
            let AMH = 5e2f64;
            let AMJ = 1.403592217853e217f64;
            let AML = 6e1f64;
            let AMO = 1.14200738981568e26f64;
            let ANP = 1.0f64;
            let ANQ = 0.0f64;
            let ANR = 1.0f64;
            let ANS = 0.0f64;
            let ANT = 0.0f64;
            let AOZ = 1.0f64;
            let APA = 0.0f64;
            let APB = 1.0f64;
            let APC = 0.0f64;
            let APD = 0.0f64;
            let AQJ = -1e0f64;
            let AQM = -1e0f64;
            let ARC = 8e1f64;
            let ARE = 1.25e2f64;
            let ARF = 4e1f64;
            let ARI = 2.5e1f64;
            let ATU = 1.0f64;
            let ATV = 0.0f64;
            let ATW = 0.0f64;
            let ATX = 1.0f64;
            let ATY = 0.0f64;
            let AUV = 0.0f64;
            let AVT = 2.220446049250313e-15f64;
            let AWI = 2.220446049250313e-15f64;
            let BCU = 1.0f64;
            let BCV = 0.0f64;
            let BCW = 1.0f64;
            let BCX = 0.0f64;
            let BCY = 0.0f64;
            let BEE = 1.0f64;
            let BEF = 0.0f64;
            let BEG = 1.0f64;
            let BEH = 0.0f64;
            let BEI = 0.0f64;
            let BFO = -1e0f64;
            let BFR = -1e0f64;
            let BIK = 1.0f64;
            let BIL = 0.0f64;
            let BIM = 1.0f64;
            let BIN = 0.0f64;
            let BIO = 0.0f64;
            let BJD = 1.0f64;
            let BJE = 0.0f64;
            let BJF = 1.0f64;
            let BJG = 0.0f64;
            let BJH = 0.0f64;
            let BKA = 1.0f64;
            let BKB = 0.0f64;
            let BKC = 1.0f64;
            let BKD = 0.0f64;
            let BKE = 0.0f64;
            let BKV = 2.220446049250313e-15f64;
            let BLK = -1e0f64;
            let BLP = 9e0f64;
            let BLT = 1e-8f64;
            let BLZ = 1.2e1f64;
            let BMD = 0.0f64;
            let BMH = 2.220446049250313e-15f64;
            let BOF = 1e-16f64;
            let BOO = 5e-3f64;
            let BPE = -1e0f64;
            let BQM = 2.01e2f64;
            let BQU = -1e0f64;
            let BSL = 1.0f64;
            let BSM = 0.0f64;
            let BSN = 0.0f64;
            let BSO = 1.0f64;
            let BSP = 0.0f64;
            let BTM = 0.0f64;
            let BTO = 1.0f64;
            let BWR = 2.01e2f64;
            let BWZ = -1e0f64;
            let BYT = 1.0f64;
            let BYU = 0.0f64;
            let BYV = 0.0f64;
            let BYW = 1.0f64;
            let BYX = 0.0f64;
            let BZN = 2.220446049250313e-15f64;
            let CAN = parameters[191];
            let CBA = parameters[189];
            let CBP = 1e9f64;
            let CCZ = parameters[227];
            let CDD = 2.220446049250313e-15f64;
            let CDG = 1.034943e-12f64;
            let CDW = parameters[113];
            let CEN = parameters[281];
            let CFO = parameters[156];
            let CFW = -1e0f64;
            let CHB = 1.0f64;
            let CHC = 0.0f64;
            let CHD = 0.0f64;
            let CHE = 1.0f64;
            let CHF = 0.0f64;
            let CHS = parameters[30];
            let CHT = parameters[32];
            let CIJ = parameters[285];
            let CIL = parameters[286];
            let CIV = 2.220446049250313e-15f64;
            let CIZ = 1.0f64;
            let CJQ = parameters[287];
            let CKN = 1.0f64;
            let CKO = 0.0f64;
            let CKP = 1.0f64;
            let CKQ = 0.0f64;
            let CKR = 0.0f64;
            let CPI = 2.01e2f64;
            let CPQ = -1e0f64;
            let CQG = -1e0f64;
            let CRI = 1.0f64;
            let CRJ = 1.0f64;
            let CRK = 0.0f64;
            let CRL = 0.0f64;
            let CRM = 0.0f64;
            let CSH = parameters[49];
            let CTD = 1.0f64;
            let CTE = 0.0f64;
            let CTF = 0.0f64;
            let CTG = 1.0f64;
            let CTH = 0.0f64;
            let CUY = parameters[47];
            let CWA = parameters[27];
            let CWM = parameters[219];
            let CWO = parameters[218];
            let CWU = parameters[222];
            let CXM = parameters[209];
            let CXN = parameters[210];
            let CXO = parameters[211];
            let CXT = parameters[208];
            let CYI = 1.0f64;
            let CYM = parameters[292];
            let CYN = 0.0f64;
            let CYT = 1e0f64;
            let CYU = 0e0f64;
            let DAQ = 2.220446049250313e-15f64;
            let DAV = 2.220446049250313e-15f64;
            let DCC = parameters[42];
            let DCL = 2.9693154855771e-1f64;
            let DCM = 6.115288895133179e-3f64;
            let DCQ = 7.07106781186548e-1f64;
            let DCR = 1.78800506338833e-2f64;
            let DCS = 6.36964918866352e-5f64;
            let DDN = 4.1e1f64;
            let DDV = -1e0f64;
            let DEV = 1.0f64;
            let DFA = 0.0f64;
            let DFD = 0e0f64;
            let DFE = 1e0f64;
            let DGN = 2.220446049250313e-15f64;
            let DGS = 2.220446049250313e-15f64;
            let DJE = 4.1e1f64;
            let DJM = -1e0f64;
            let DKS = 1.0f64;
            let DKV = 0.0f64;
            let DLB = parameters[64];
            let DLJ = parameters[188];
            let DLX = 1e0f64;
            let DLY = 0e0f64;
            let DNU = 2.220446049250313e-15f64;
            let DNZ = 2.220446049250313e-15f64;
            let DOI = parameters[41];
            let DQQ = 4.1e1f64;
            let DQY = -1e0f64;
            let DSH = 0e0f64;
            let DSI = 1e0f64;
            let DUB = 2.220446049250313e-15f64;
            let DUG = 2.220446049250313e-15f64;
            let DWW = 4.1e1f64;
            let DXE = -1e0f64;
            let DYQ = parameters[170];
            let DYR = parameters[169];
            let EAH = parameters[174];
            let EAQ = parameters[179];
            let EAR = parameters[2];
            let EAT = parameters[3];
            let EAX = parameters[5];
            let EAZ = parameters[180];
            let EBB = parameters[181];
            let EBG = parameters[182];
            let EBJ = parameters[183];
            let EBM = parameters[184];
            let EBU = parameters[4];
            let ECS = parameters[233];
            let ECT = parameters[234];
            let EFL = parameters[168];
            let EFP = parameters[167];
            let ELE = parameters[259];
            let ELG = 1.0f64;
            let ELH = parameters[264];
            let ELJ = parameters[266];
            let ELK = parameters[268];
            let ELL = parameters[273];
            let ELM = parameters[263];
            let ELO = parameters[255];
            let ELR = parameters[258];
            let ELT = parameters[265];
            let ELU = parameters[267];
            let ELV = parameters[272];
            let ELX = parameters[256];
            let EMA = parameters[257];
            let EMC = parameters[271];
            let EMG = parameters[269];
            let EMH = parameters[270];
            let EMJ = parameters[274];
            let EML = parameters[279];
            let EMM = parameters[280];
            let EMN = parameters[277];
            let EMO = parameters[278];
            let EMP = parameters[275];
            let EMQ = parameters[276];
            let ENX = parameters[260];
            let ENZ = 0.0f64;
            let EQJ = parameters[231];
            if C != 0.0 {
                let E = if D == B { 1.0 } else { 0.0 };
                if E != 0.0 {
                } else {
                }
            } else {
            }
            let F = if D == A { 1.0 } else { 0.0 };
            if F != 0.0 {
            } else {
            }
            let K = (parameters[51] * J) % J;
            let N = parameters[52] * M;
            let P = parameters[73] / O;
            let Q = parameters[104] * M;
            let R = parameters[201] / O;
            let S = parameters[229] * M;
            let U = parameters[228] / T;
            let V = parameters[230] / T;
            let X = W / O;
            let Y = parameters[241] / O;
            let AA = Z * M;
            let AB = parameters[59] / O;
            let AC = parameters[284] / O;
            let AD = parameters[148] / O;
            let AE = parameters[198] / T;
            let AF = parameters[70] * M;
            let AH = if AG == A { 1.0 } else { 0.0 };
            let AJ = if AH != 0.0 {
                A
            } else {
                AI
            };
            let AL = if AH != 0.0 {
                A
            } else {
                AK
            };
            let AN = if AM == A { 1.0 } else { 0.0 };
            let AP = if AN != 0.0 {
                A
            } else {
                AO
            };
            let AR = if AH != 0.0 {
                A
            } else {
                AQ
            };
            let AT = parameters[250] * AS;
            let AV = parameters[232] + AU;
            let AY = parameters[15] * AX;
            let BE = if BB != 0.0 {
                BC
            } else {
                let BD = 5e9f64 / (H * W);
                BD
            };
            let BH = if (if BE < 2.1e0f64 { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
            let CAY;
            if BH != 0.0 {
                let BI = 2.1e0f64 - BE;
                let BJ = BI * BI;
                let BK = (BJ * BJ) + 1.0000000000000005e-4f64;
                let CD;
                if BN != 0.0 {
                    let BY;
                    if BO != 0.0 {
                        BY = B;
                    } else {
                        let BZ;
                        if BP != 0.0 {
                            BZ = BF;
                        } else {
                            let CA;
                            if BQ != 0.0 {
                                CA = BR;
                            } else {
                                let CB = if BS != 0.0 {
                                    BL
                                } else {
                                    A
                                };
                                CA = CB;
                            }
                            BZ = CA;
                        }
                        BY = BZ;
                    }
                    let mut BT = 0.0;
                    let mut BV = 0.0;
                    BT = A;
                    BV = BK;
                    loop {
                        let BU = if BT < BY { 1.0 } else { 0.0 };
                        if BU == 0.0 {
                            break;
                        }
                        let BW = BV.sqrt();
                        let BX = BT + B;
                        BT = BX;
                        BV = BW;
                    }
                    CD = BV;
                } else {
                    let CC = BK.powf(2.5e-1f64);
                    CD = CC;
                }
                let CE = 2.1e0f64 - ((BI * BG) * (B / CD));
                CAY = CE;
            } else {
                CAY = BE;
            }
            let CG = parameters[55] - (AV * (9.025e-5f64 + (AV * CF)));
            let CJ = CI / H;
            let CK = B / CJ;
            let CM = CL / CH;
            let CN = CH / CL;
            let CP = CL / CO;
            let CQ = CO / CL;
            let CR = CQ + CK;
            let CU = CS - (BF * CT);
            let CV = CS - (BF * parameters[57]);
            let CW = if parameters[40] == A { 1.0 } else { 0.0 };
            let CX = if CW != 0.0 {
                CS
            } else {
                CU
            };
            let CY = CX * AS;
            let DA = parameters[1] / CZ;
            let DC = if K < B { 1.0 } else { 0.0 };
            let DE = if DC != 0.0 {
                A
            } else {
                DD
            };
            let DG = if DC != 0.0 {
                DB
            } else {
                DF
            };
            let DO;
            let DQ;
            if F != 0.0 {
                let DH = DA - (BF * DB);
                let DI = DA - (BF * DG);
                DO = DH;
                DQ = DI;
            } else {
                let DK = DA - (DJ * DE);
                let DL = BF - DJ;
                let DM = DK - (DL * DB);
                let DN = DK - (DL * DG);
                DO = DM;
                DQ = DN;
            }
            let DP = DO * CZ;
            let DR = DQ * CZ;
            let DS = DA * AS;
            let DT = DS * CY;
            let DU = (parameters[107] * (B + (parameters[108] / (CY.powf(parameters[111]))))) * (B + (parameters[109] / (DS.powf(parameters[110]))));
            let DV = if K > BR { 1.0 } else { 0.0 };
            let DX = if DW > A { 1.0 } else { 0.0 };
            let DY = if (if DV != 0.0 && (if P < X { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && DX != 0.0 { 1.0 } else { 0.0 };
            let DZ = if DY != 0.0 {
                X
            } else {
                P
            };
            let EA = DZ * (B + (parameters[74] / (DS.powf(parameters[75]))));
            let EB = I * CS;
            let EC = BF / ((B / (parameters[62] + EB)) + (B / (parameters[63] + EB)));
            let EF = ED / (EE * AV);
            let EG = (ED * Y) * CI;
            let EI = EH * (CY.powf((-parameters[247])));
            let EJ = parameters[251] * (CY.powf((-parameters[252])));
            let EL = EK * ((CY + AT).powf((-parameters[249])));
            let EM = ((3.2043836e-19f64 * AD) * CI).sqrt();
            let EN = B / (AD * AD);
            let EP = ((B + (B / CY)).powf(parameters[91])) * EO;
            let ER = CX + (parameters[76] / (DT.powf(parameters[77])));
            let ES = parameters[78] / (DT.powf(parameters[79]));
            let ET = (parameters[149] * (B + (parameters[150] / ((ER * AS).powf(parameters[151]))))) + (parameters[152] / (DS.powf(parameters[153])));
            let EU = B + ((CY.powf(parameters[192])) * parameters[193]);
            let EW = (parameters[67] * (parameters[7] + (DO / (BR * EV)))) / ((EV * (CS - parameters[8])) * CZ);
            let EX = if parameters[44] <= A { 1.0 } else { 0.0 };
            let AIL;
            let AIT;
            let AIU;
            let AIZ;
            let AKP;
            let AKQ;
            if EX != 0.0 {
                let FA = B + (EY / (DS.powf(EZ)));
                let FE = FB * (B + (FC / (CY.powf(FD))));
                let FG = CY / (CY + FF);
                let FK = FH * (B + (FI / (CY.powf(FJ))));
                let FN = FL * (B + (FM / CY));
                AIL = FE;
                AIT = FG;
                AIU = FA;
                AIZ = AJA;
                AKP = FN;
                AKQ = FK;
            } else {
                let FO = DS.powf(EZ);
                let FS = (FP * (B + (FQ / (CY.powf(FR))))) * (FO / (FO + EY));
                let FT = FB * (B + (FC / (CY.powf(FD))));
                let FU = FF * (B + (parameters[132] / (CY.powf(parameters[133]))));
                let FV = FH * (B + (FI / (CY.powf(FJ))));
                let FW = FL * (B + (FM / CY));
                AIL = FT;
                AIT = FU;
                AIU = AIV;
                AIZ = FS;
                AKP = FW;
                AKQ = FV;
            }
            let FY = ((AS * DR) * FX) / (CY.powf(parameters[66]));
            let FZ = parameters[134] * (B + (parameters[135] / (CY.powf(parameters[136]))));
            let AIQ = if EX != 0.0 {
                let GA = FP * (B + (FQ / (CY.powf(FR))));
                GA
            } else {
                AIR
            };
            let GB = parameters[115] * CY;
            let GE = (((GB * GC) / (GB + GC)) + parameters[116]) + GD;
            let GF = if GE < BR { 1.0 } else { 0.0 };
            let AUR = if GF != 0.0 {
                BR
            } else {
                GE
            };
            let GH = GG * parameters[253];
            let GQ = if GP == A { 1.0 } else { 0.0 };
            let GR = if GQ != 0.0 {
                A
            } else {
                B
            };
            let GU = parameters[16] + AU;
            let GY = if (if (if GV > A { 1.0 } else { 0.0 }) != 0.0 && (if GW > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if CZ == B { 1.0 } else { 0.0 }) != 0.0 || (if (if CZ > B { 1.0 } else { 0.0 }) != 0.0 && (if GX > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HG;
            if GY != 0.0 {
                let mut GZ = 0.0;
                let mut HB = 0.0;
                GZ = A;
                HB = A;
                loop {
                    let HA = if GZ < CZ { 1.0 } else { 0.0 };
                    if HA == 0.0 {
                        break;
                    }
                    let HC = GZ * (GX + CS);
                    let HD = (HB + (B / ((GV + EB) + HC))) + (B / ((GW + EB) + HC));
                    let HE = GZ + B;
                    GZ = HE;
                    HB = HD;
                }
                let HF = (BF * CZ) / HB;
                HG = HF;
            } else {
                HG = A;
            }
            let HH = if HG > A { 1.0 } else { 0.0 };
            let HZ = if HH != 0.0 {
                let HI = B / (B + parameters[162]);
                let HL = (EA * (B + (HI * ((HJ / HG).powf(HK))))) / (B + (HI * ((HJ / EC).powf(HK))));
                HL
            } else {
                EA
            };
            let HM = R / X;
            let HN = (HM - ((B + (parameters[199] / (DS.powf(parameters[200])))) * (B + (parameters[202] / (CY.powf(parameters[203])))))) - M;
            let HO = (BL * HM) * M;
            let HP = if HO > A { 1.0 } else { 0.0 };
            let HR = if HP != 0.0 {
                HO
            } else {
                let HQ = -HO;
                HQ
            };
            let HS = X * (HM - (I * (HN + (((HN * HN) + HR).sqrt()))));
            let HY = if HH != 0.0 {
                let HT = B / (B + parameters[165]);
                let HW = (HS * (B + (HT * ((HU / HG).powf(HV))))) / (B + (HT * ((HU / EC).powf(HV))));
                HW
            } else {
                HS
            };
            let HX = if (if CX > DW { 1.0 } else { 0.0 }) != 0.0 || (if DW <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IC = if HX != 0.0 {
                let IA = ((HY * (CX - DW)) + (HZ * DW)) / CX;
                IA
            } else {
                let IB = HZ + (((HZ - HY) * (DW - CX)) / DW);
                IB
            };
            let ID = ED * IC;
            let IE = ID * CI;
            let IF = BF * IE;
            let IG = if (if CX <= (BF * DW) { 1.0 } else { 0.0 }) != 0.0 && DX != 0.0 { 1.0 } else { 0.0 };
            let LV = if IG != 0.0 {
                let IH = ((((BF * HZ) - (((HZ - HY) * CX) / DW)) - HY) / HY).ln();
                IH
            } else {
                A
            };
            let II = 5.1702525384001115e-2f64 * ((IC / 1.04e16f64).ln());
            let IJ = 5.1702525384001115e-2f64 * ((HY / 1.04e16f64).ln());
            let IK = (1.2919089961638799e9f64 / IC).sqrt();
            let IL = (B + (parameters[194] / (CY.powf(parameters[195])))) * (B + (parameters[196] / (DT.powf(parameters[197]))));
            let IO = (I * (IL + (((IL * IL) + 4e-6f64).sqrt()))) + 1e-13f64;
            let IP = if IO < A { 1.0 } else { 0.0 };
            let LX = if IP != 0.0 {
                A
            } else {
                IO
            };
            let IR = if IQ == B { 1.0 } else { 0.0 };
            if IR != 0.0 {
                let IS = if EW > IM { 1.0 } else { 0.0 };
                if IS != 0.0 {
                } else {
                }
            } else {
            }
            let IU = if IT == B { 1.0 } else { 0.0 };
            if IU != 0.0 {
                let IV = if ((parameters[289] * DP) + parameters[288]) < T { 1.0 } else { 0.0 };
                if IV != 0.0 {
                } else {
                }
            } else {
            }
            let IX = if IW == B { 1.0 } else { 0.0 };
            if IX != 0.0 {
                let IY = if parameters[290] < T { 1.0 } else { 0.0 };
                if IY != 0.0 {
                } else {
                }
                let JA = if parameters[291] < T { 1.0 } else { 0.0 };
                if JA != 0.0 {
                } else {
                }
            } else {
            }
            let JB = if D == B { 1.0 } else { 0.0 };
            let BRU;
            let CYJ;
            let DLN;
            let DYT;
            let EAJ;
            let EAK;
            let EFE;
            let EFH;
            let EFS;
            let EFT;
            if JB != 0.0 {
                let BRV;
                let CYK;
                let EFF;
                let EFI;
                if JC != 0.0 {
                    let JG = if GM != 0.0 {
                        JD
                    } else {
                        let JF = (parameters[20] * CZ) * JE;
                        JF
                    };
                    let JJ = if GN != 0.0 {
                        JH
                    } else {
                        let JI = (parameters[21] * CZ) * JE;
                        JI
                    };
                    let JK = if (if JG > A { 1.0 } else { 0.0 }) != 0.0 && GL != 0.0 { 1.0 } else { 0.0 };
                    let EFG = if JK != 0.0 {
                        let JL = (-JG) * parameters[294];
                        JL
                    } else {
                        A
                    };
                    let JM = if (if JJ > A { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[293] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BRW;
                    let EFJ;
                    if JM != 0.0 {
                        let JN = (-JJ) * parameters[293];
                        BRW = A;
                        EFJ = JN;
                    } else {
                        BRW = JJ;
                        EFJ = A;
                    }
                    BRV = BRW;
                    CYK = JG;
                    EFF = EFG;
                    EFI = EFJ;
                } else {
                    BRV = A;
                    CYK = A;
                    EFF = A;
                    EFI = A;
                }
                let JO = if JE > CS { 1.0 } else { 0.0 };
                let JQ = if JO != 0.0 {
                    let JP = I * (JE - CS);
                    JP
                } else {
                    A
                };
                let JR = if (if parameter_given[13] { 1.0 } else { 0.0 }) == A { 1.0 } else { 0.0 };
                let JT = if JR != 0.0 {
                    JQ
                } else {
                    GS
                };
                let JS = if (if parameter_given[14] { 1.0 } else { 0.0 }) == A { 1.0 } else { 0.0 };
                let JW = if JS != 0.0 {
                    JQ
                } else {
                    GT
                };
                let JU = CZ * JT;
                let JV = DP + JU;
                let JX = CZ * JW;
                let JY = DP + JX;
                let JZ = DR + JU;
                let KA = DR + JX;
                BRU = BRV;
                CYJ = CYK;
                DLN = KA;
                DYT = JZ;
                EAJ = JV;
                EAK = JY;
                EFE = EFF;
                EFH = EFI;
                EFS = JT;
                EFT = JW;
            } else {
                BRU = A;
                CYJ = A;
                DLN = A;
                DYT = A;
                EAJ = A;
                EAK = A;
                EFE = A;
                EFH = A;
                EFS = GS;
                EFT = GT;
            }
            let KD = GG * (KB - KC);
            let KE = GG * (node_potentials[11] - KC);
            let KG = GG * (KF - KC);
            let EAF;
            let EAG;
            if JB != 0.0 {
                let KJ = GG * (KF - KB);
                if BA != 0.0 {
                } else {
                }
                EAF = KJ;
                EAG = KG;
            } else {
                if BA != 0.0 {
                } else {
                }
                EAF = A;
                EAG = A;
            }
            let KM = if KL > A { 1.0 } else { 0.0 };
            let KN = if AA > A { 1.0 } else { 0.0 };
            let KO = if KM != 0.0 && KN != 0.0 { 1.0 } else { 0.0 };
            let KS;
            if KO != 0.0 {
                let KQ = if KP > A { 1.0 } else { 0.0 };
                let KR = if KQ != 0.0 {
                    KP
                } else {
                    A
                };
                KS = KR;
            } else {
                KS = A;
            }
            let KT = if KD >= A { 1.0 } else { 0.0 };
            let NM;
            let OO;
            let OS;
            let CYV;
            let CYW;
            let DZG;
            if KT != 0.0 {
                NM = KG;
                OO = KD;
                OS = KE;
                CYV = B;
                CYW = A;
                DZG = B;
            } else {
                let KV = -KD;
                let KW = KE - KD;
                let KX = KG - KD;
                NM = KX;
                OO = KV;
                OS = KW;
                CYV = A;
                CYW = B;
                DZG = KU;
            }
            let KZ = if AZ >= KY { 1.0 } else { 0.0 };
            if KZ != 0.0 {
            } else {
            }
            let LB = if AZ >= LA { 1.0 } else { 0.0 };
            if LB != 0.0 {
            } else {
            }
            let LD = if GO != 0.0 {
                GU
            } else {
                LC
            };
            let LF = if GR != 0.0 {
                let LE = LD + GP;
                LE
            } else {
                LD
            };
            let LG = LF + KS;
            let LH = LG - AV;
            let LI = (CG - (parameters[53] * LH)) - (parameters[54] * (LH * (LG + AV)));
            let LJ = ED / (EE * LG);
            let LK = LJ * LJ;
            let LL = B / LJ;
            let LM = ((parameters[254] * (B + (parameters[98] / (DS.powf(parameters[99]))))) * (B + (parameters[100] / (CY.powf(parameters[101]))))) * (B + (parameters[102] / (DT.powf(parameters[103]))));
            let LN = B / (B + parameters[159]);
            let LO = parameters[158] / AY;
            let LQ = if (if LO == A { 1.0 } else { 0.0 }) != 0.0 && (if LP == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let LS = if LQ != 0.0 {
                B
            } else {
                let LR = LO.powf(LP);
                LR
            };
            let LT = LG / AV;
            let LU = (LT.powf(parameters[112])) / (LM * (B + (LN * LS)));
            let LW = LV * LL;
            let LZ = (1.8e0f64 + (LY * LT)) + ((BG * LT) * LT);
            let MA = B - LT;
            let MB = (LX * N) / (LZ - (Q * MA));
            let MC = LI.sqrt();
            let MD = LI * MC;
            let ME = (1.04e16f64 * (LT * (LT.sqrt()))) * (((((-LI) / BF) * LJ) + ((CG / BF) * EF)).exp());
            let MF = LL.sqrt();
            let MG = EM * MF;
            let MH = MG * MG;
            let MI = ME * ME;
            let MJ = MI * EN;
            let MY = if DV != 0.0 {
                let MK = (BF * LL) * ((IC / ME).ln());
                MK
            } else {
                let ML = (BF * LL) * ((HY / ME).ln());
                ML
            };
            let MM = CI / ID;
            let MO = (ID * MN) * ((MM * LL).sqrt());
            let MU;
            let VI;
            let VU;
            if JB != 0.0 {
                let MP = ME / IC;
                MU = MP;
                VI = A;
                VU = A;
            } else {
                let MQ = ((BF * EG) * LL).sqrt();
                let MR = ME / Y;
                let MS = MR * MR;
                let MT = ME / HY;
                MU = MT;
                VI = MQ;
                VU = MS;
            }
            let MV = MU * MU;
            let MW = (BF * (MM / LJ)).sqrt();
            let MX = 1.2919089961638799e9f64 / HY;
            let MZ = ((1.2919089961638799e9f64 * MY) / HY).sqrt();
            let NA = if DO < KK { 1.0 } else { 0.0 };
            let NF = if NA != 0.0 {
                B
            } else {
                A
            };
            let NB = if DQ < KK { 1.0 } else { 0.0 };
            let NE = if NB != 0.0 {
                B
            } else {
                NF
            };
            let NC = if CU < KK { 1.0 } else { 0.0 };
            let ND = if NC != 0.0 {
                B
            } else {
                NE
            };
            if ND != 0.0 {
            } else {
            }
            let NI;
            let NJ;
            if JB != 0.0 {
                NI = LY;
                NJ = NG;
            } else {
                NI = NG;
                NJ = NH;
            }
            let NK = NJ * I;
            let NL = if NI > NK { 1.0 } else { 0.0 };
            let NN = if NL != 0.0 {
                NK
            } else {
                NI
            };
            let NO = if NM > NN { 1.0 } else { 0.0 };
            let OY;
            let PC;
            if NO != 0.0 {
                let NP = NM - NN;
                let NQ = NJ - NN;
                let NR = NP * NP;
                let NS = NQ * NQ;
                let NT = ((NS * NS) * NS) * NS;
                let NU = (((NR * NR) * NR) * NR) + NT;
                let OK;
                if NV != 0.0 {
                    let OF;
                    if NW != 0.0 {
                        OF = B;
                    } else {
                        let OG;
                        if NX != 0.0 {
                            OG = BF;
                        } else {
                            let OH;
                            if NY != 0.0 {
                                OH = BR;
                            } else {
                                let OI = if NZ != 0.0 {
                                    BL
                                } else {
                                    A
                                };
                                OH = OI;
                            }
                            OG = OH;
                        }
                        OF = OG;
                    }
                    let mut OA = 0.0;
                    let mut OC = 0.0;
                    OA = A;
                    OC = NU;
                    loop {
                        let OB = if OA < OF { 1.0 } else { 0.0 };
                        if OB == 0.0 {
                            break;
                        }
                        let OD = OC.sqrt();
                        let OE = OA + B;
                        OA = OE;
                        OC = OD;
                    }
                    OK = OC;
                } else {
                    let OJ = NU.powf(1.25e-1f64);
                    OK = OJ;
                }
                let OL = B / OK;
                let OM = ((NQ * NT) * OL) / NU;
                let ON = NN + ((NP * NQ) * OL);
                OY = ON;
                PC = OM;
            } else {
                OY = NM;
                PC = B;
            }
            let OQ = if OO > OP { 1.0 } else { 0.0 };
            let OR = if OQ != 0.0 {
                OP
            } else {
                OO
            };
            let OT = if OS > OP { 1.0 } else { 0.0 };
            let OU = if OT != 0.0 {
                OP
            } else {
                OS
            };
            let OV = if OS < -2e1f64 { 1.0 } else { 0.0 };
            let OX = if OV != 0.0 {
                OW
            } else {
                OU
            };
            let OZ = if OY < -2e1f64 { 1.0 } else { 0.0 };
            let PB = if OZ != 0.0 {
                PA
            } else {
                OY
            };
            let PD = BF * ((PC * OR) / BF);
            let PF = PD / PE;
            let PG = PE / (B + (PF * (5e-1f64 + (PF * (1.6666666666666666e-1f64 + (PF * (4.1666666666666664e-2f64 + (PF * (8.333333333333333e-3f64 + (PF * (1.388888888888889e-3f64 + (PF * 1.984126984126984e-4f64))))))))))));
            let PI = if PG < PH { 1.0 } else { 0.0 };
            let PJ = if PI != 0.0 {
                PH
            } else {
                PG
            };
            let PK = PB + PJ;
            let PL = OR + (BF * PJ);
            let PM = OX + PJ;
            let PT;
            let RP;
            if JB != 0.0 {
                PT = PB;
                RP = PK;
            } else {
                let PN = if K < BR { 1.0 } else { 0.0 };
                let PO = if PN != 0.0 {
                    PB
                } else {
                    A
                };
                let PP = if PN != 0.0 {
                    PK
                } else {
                    A
                };
                PT = PO;
                RP = PP;
            }
            let PQ = (BF * ID) * CI;
            let PR = (PQ * CN) * CN;
            let PS = OX - EQ;
            let PU = B + ((BF / PR) * ((PS - LL) - PT));
            let PV = (I * (PU + (((PU * PU) + 4e-6f64).sqrt()))) + 1e-13f64;
            let PW = if PV < A { 1.0 } else { 0.0 };
            let PX = if PW != 0.0 {
                A
            } else {
                PV
            };
            let PZ = (((PS + (PR * (B - ((PX + GD).sqrt())))) - MY) - BG) - PY;
            let QD = if QB != 0.0 {
                QA
            } else {
                QC
            };
            let QE = OR / (BG + (I * (PZ + (((PZ * PZ) + QD).sqrt()))));
            let QF = QE * QE;
            let QG = B - (B / ((((B + QE) + QF) + (QF * QE)) + (QF * QF)));
            let QH = QG * QG;
            let QL = if (if (if QI == A { 1.0 } else { 0.0 }) != 0.0 && (if QJ == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if QK == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let QO = if QL != 0.0 {
                A
            } else {
                B
            };
            let QM = II + EQ;
            let QN = QM + (((PQ * II).sqrt()) / CM);
            let QP = if QO == A { 1.0 } else { 0.0 };
            let SP;
            let TK;
            let UT;
            if QP != 0.0 {
                let QQ = ((MO * CN) * CN) * MO;
                SP = CN;
                TK = CM;
                UT = QQ;
            } else {
                let QR = ((OX - PT) - QN) + QK;
                let QS = (I * (QR + (((QR * QR) + 4e-8f64).sqrt()))) + 1.0000000000000002e-14f64;
                let QT = if QS < A { 1.0 } else { 0.0 };
                let QU = if QT != 0.0 {
                    A
                } else {
                    QS
                };
                let QV = B / QU;
                let QW = BF * (QN.abs());
                let QX = (EQ - QN) + QK;
                let QY = if QX > QW { 1.0 } else { 0.0 };
                let QZ = if QY != 0.0 {
                    QX
                } else {
                    QW
                };
                let RA = B / QZ;
                let RB = (RA - QV) - T;
                let RC = (BL * RA) * T;
                let RD = if RC > A { 1.0 } else { 0.0 };
                let RF = if RD != 0.0 {
                    RC
                } else {
                    let RE = -RC;
                    RE
                };
                let RG = (QI * (RA - (I * (RB + (((RB * RB) + RF).sqrt()))))) + QJ;
                let RH = if (RG * 1e12f64) < CH { 1.0 } else { 0.0 };
                let RI = if RH != 0.0 {
                    A
                } else {
                    RG
                };
                let RJ = CH + RI;
                let RK = CL / RJ;
                let RL = RJ / CL;
                let RM = ((MO * MO) * RL) * RL;
                SP = RL;
                TK = RK;
                UT = RM;
            }
            let RN = if K < BR { 1.0 } else { 0.0 };
            let RO = if JB != 0.0 || RN != 0.0 { 1.0 } else { 0.0 };
            let SJ;
            if RO != 0.0 {
                let RQ = (I - RP) - IM;
                let RU = if RS != 0.0 {
                    RR
                } else {
                    RT
                };
                let RV = (((((-H) * H) * ID) / 2.069886e-10f64) + MY) - LL;
                let RW = ((I - (I * (RQ + (((RQ * RQ) + RU).sqrt())))) - RV) - IM;
                let RX = (BL * RV) * IM;
                let RY = if RX > A { 1.0 } else { 0.0 };
                let SA = if RY != 0.0 {
                    RX
                } else {
                    let RZ = -RX;
                    RZ
                };
                let SB = RV + (I * (RW + (((RW * RW) + SA).sqrt())));
                let SC = if K > BF { 1.0 } else { 0.0 };
                let SK;
                if SC != 0.0 {
                    let SD = (II - SB) - IM;
                    let SE = (BL * II) * IM;
                    let SF = if SE > A { 1.0 } else { 0.0 };
                    let SH = if SF != 0.0 {
                        SE
                    } else {
                        let SG = -SE;
                        SG
                    };
                    let SI = II - (I * (SD + (((SD * SD) + SH).sqrt())));
                    SK = SI;
                } else {
                    SK = SB;
                }
                SJ = SK;
            } else {
                SJ = A;
            }
            let TA = if RN != 0.0 {
                H
            } else {
                let SL = ((2.069886e-10f64 / ID) * (II - SJ)).sqrt();
                SL
            };
            let SO = if RN != 0.0 {
                let SM = (IF * II).sqrt();
                SM
            } else {
                let SN = (IF * (II - SJ)).sqrt();
                SN
            };
            let SQ = (QM + (SO * SP)) + LW;
            let SR = 9.5e-1f64 * II;
            let SS = (SR - SJ) - IM;
            let ST = II - (SR - (I * (SS + (((SS * SS) + ((3.8e0f64 * II) * IM)).sqrt()))));
            let SU = ST.sqrt();
            let SV = if DW != A { 1.0 } else { 0.0 };
            let TL;
            if SV != 0.0 {
                let SW = (3.2043836e-19f64 * HY) * CI;
                let SZ = if RN != 0.0 {
                    let SX = (SW * IJ).sqrt();
                    SX
                } else {
                    let SY = (SW * (IJ - SJ)).sqrt();
                    SY
                };
                let TC = ((SQ - ((IJ + EQ) + (SZ * SP))) * (((CI * SP) * ((BF * TA) * (B / (DW * DW)))) * (TB - II))) * ((AM + ((AR / DW) * ST)) + (AP * PL));
                TL = TC;
            } else {
                TL = A;
            }
            let TD = TB - II;
            let TF = CX - TE;
            let TG = (((SP * ((CI * TA) * BF)) * TD) * (B / (TF * TF))) * ((AG + ((AL / CX) * ST)) + (AJ * PL));
            let TI = if TH > A { 1.0 } else { 0.0 };
            let TN = if TI != 0.0 {
                let TJ = (((LI + MY) - (BF * parameters[88])) + (parameters[87] * PL)) * ((TH * H) / ((CX * I) + AF));
                TJ
            } else {
                A
            };
            let TM = TG + TL;
            let TO = ((TM + ((SO * (SP - (B / (TK + (AE / DO))))) + (parameters[105] / DS))) + TN) + ES;
            let TP = SQ - TO;
            let TQ = if EO == A { 1.0 } else { 0.0 };
            let TR = if TQ != 0.0 {
                A
            } else {
                B
            };
            let TS = if TR == A { 1.0 } else { 0.0 };
            let UL;
            if TS != 0.0 {
                UL = A;
            } else {
                let TT = PM - parameters[90];
                let TU = if TT < -3e0f64 { 1.0 } else { 0.0 };
                let TZ;
                if TU != 0.0 {
                    TZ = A;
                } else {
                    let TV = if TT < A { 1.0 } else { 0.0 };
                    let UA = if TV != 0.0 {
                        let TX = B + (TT * (B + (TT * (3.333333333333333e-1f64 + (TT * 3.7037037037037035e-2f64)))));
                        TX
                    } else {
                        let TY = B + (TT * (B + (TT * (3.333333333333333e-1f64 + (TT * (4.02052934513951e-2f64 + (TT * 1.48148111111111e-1f64)))))));
                        TY
                    };
                    TZ = UA;
                }
                let UB = TZ - B;
                let UC = (I * (UB + (((UB * UB) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64;
                let UD = if UC < A { 1.0 } else { 0.0 };
                let UE = if UD != 0.0 {
                    A
                } else {
                    UC
                };
                let UF = (B - (UE * EP)) - PY;
                let UJ = if UH != 0.0 {
                    UG
                } else {
                    UI
                };
                let UK = B - (I * (UF + (((UF * UF) + UJ).sqrt())));
                UL = UK;
            }
            let UM = (PS + TO) - UL;
            let UN = LL * ((HY / Y).ln());
            let UO = (EQ - TO) + UL;
            let UP = MO * SP;
            let UQ = UP * UP;
            let BZV;
            let BZX;
            let CAA;
            let CAD;
            let CAI;
            let CAP;
            let CAT;
            let CAX;
            let CBJ;
            let CCC;
            let CCJ;
            let CCR;
            let CCS;
            let CCV;
            let CEY;
            let CGD;
            let CGR;
            let CHV;
            let CJH;
            let CJL;
            let CJM;
            let CLM;
            let CSI;
            let CUN;
            let CVE;
            let CVP;
            let EDW;
            let EGI;
            let EGN;
            let EGR;
            let EGV;
            let EIJ;
            let EIU;
            if F != 0.0 {
                let US = MY + B;
                let UU = (B / MV) / UT;
                let UV = (MX * ((((UU * US) * US).ln()) / (LJ + (BF / US)))).sqrt();
                let UW = if UV > H { 1.0 } else { 0.0 };
                let UX = if UW != 0.0 {
                    H
                } else {
                    UV
                };
                let UY = (-1.6021918e-19f64 * HY) * UX;
                let UZ = (-1.6021918e-19f64 * HY) * H;
                let VA = -UZ;
                let VB = VA * IM;
                let VD = VA * VC;
                let VK = if VE != 0.0 {
                    let VF = PK + UN;
                    VF
                } else {
                    let VG = PB + UN;
                    VG
                };
                let VH = (BF / LJ) * ((Y / ME).ln());
                let VJ = ((VI * VI) * CR) * CR;
                let VL = -VK;
                let VM = VJ * LJ;
                let VN = (BF * VL) + VM;
                let VO = VL * VL;
                let VP = (VN * VN) - (BL * (VO + VJ));
                let VQ = if VP >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let VS = if VQ != 0.0 {
                    VP
                } else {
                    VR
                };
                let VT = (VN - (VS.sqrt())) / BF;
                let VV = (((VO / VJ) / VU).ln()) / (LJ + (BF / VL));
                let VW = if VT < VH { 1.0 } else { 0.0 };
                let XO;
                if VW != 0.0 {
                    XO = VT;
                } else {
                    let VY = (VV - VT) - VX;
                    let VZ = (BL * VV) * VX;
                    let WA = if VZ > A { 1.0 } else { 0.0 };
                    let WC = if WA != 0.0 {
                        VZ
                    } else {
                        let WB = -VZ;
                        WB
                    };
                    let WD = VV - (I * (VY + (((VY * VY) + WC).sqrt())));
                    XO = WD;
                }
                let mut WE = 0.0;
                let mut WG = 0.0;
                let mut XP = 0.0;
                let mut ZZ = 0.0;
                WE = A;
                WG = XO;
                XP = A;
                ZZ = A;
                loop {
                    let WF = if WE < L { 1.0 } else { 0.0 };
                    if WF == 0.0 {
                        break;
                    }
                    let WH = LJ * WG;
                    let WI = (-WH).exp();
                    let WJ = if WG > KK { 1.0 } else { 0.0 };
                    let WS;
                    let XH;
                    if WJ != 0.0 {
                        let WK = WH.exp();
                        let WL = (-VI) * ((((WI + WH) - B) + (VU * (WK - B))).sqrt());
                        let WM = (EG / WL) * (((-WI) + B) + (VU * WK));
                        WS = WL;
                        XH = WM;
                    } else {
                        let WN = if WG < -1e-9f64 { 1.0 } else { 0.0 };
                        let WT;
                        let XI;
                        if WN != 0.0 {
                            let WO = VI * (((WI + WH) - B).sqrt());
                            let WP = (EG / WO) * ((-WI) + B);
                            WT = WO;
                            XI = WP;
                        } else {
                            let WQ = ((-((EG / LJ).sqrt())) * LJ) * WG;
                            let WR = -((EG * LJ).sqrt());
                            WT = WQ;
                            XI = WR;
                        }
                        WS = WT;
                        XH = XI;
                    }
                    let WU = ((WS * WS) + ((BL * VB) * VB)).sqrt();
                    let WV = I * (B + (WS / WU));
                    let WW = (I * (WS + WU)) + (IN * VB);
                    let WX = if WW < A { 1.0 } else { 0.0 };
                    let WY;
                    let XG;
                    if WX != 0.0 {
                        WY = A;
                        XG = A;
                    } else {
                        WY = WW;
                        XG = WV;
                    }
                    let WZ = (VA - WY) - VD;
                    let XA = (BL * VA) * VD;
                    let XB = if XA > A { 1.0 } else { 0.0 };
                    let XD = if XB != 0.0 {
                        XA
                    } else {
                        let XC = -XA;
                        XC
                    };
                    let XE = ((WZ * WZ) + XD).sqrt();
                    let XF = VA - (I * (WZ + XE));
                    let XJ = ((((XF * XF) / BF) / CI) / ED) / HY;
                    let XK = WG - (((((-WG) + (WS / CP)) - VK) + XJ) / ((-1e0f64 + (XH / CP)) + (((BF * XJ) * (XG * (XH * (I * (B + (WZ / XE)))))) / XF)));
                    let XL = if ((XK - WG).abs()) < PH { 1.0 } else { 0.0 };
                    let XM = if XL != 0.0 {
                        L
                    } else {
                        WE
                    };
                    let XN = XM + B;
                    WE = XN;
                    WG = XK;
                    XP = XJ;
                    ZZ = WS;
                }
                let XQ = if (((1.2919089961638799e9f64 * XP) / HY).sqrt()) > (9.9e-1f64 * H) { 1.0 } else { 0.0 };
                let AAV;
                let AFU;
                if XQ != 0.0 {
                    let XR = B / TK;
                    let XS = H / CI;
                    let XT = B / CP;
                    let XU = B / ((XR + XS) + XT);
                    let XV = (XR * (XU * (VL + ((XT + (I * XS)) * VA)))) / (B - (XU * XR));
                    let XW = UO + XV;
                    AAV = XV;
                    AFU = XW;
                } else {
                    AAV = A;
                    AFU = UO;
                }
                let XX = PD / BG;
                let XY = BG / (B + (XX * (5e-1f64 + (XX * (1.6666666666666666e-1f64 + (XX * (4.1666666666666664e-2f64 + (XX * (8.333333333333333e-3f64 + (XX * (1.388888888888889e-3f64 + (XX * 1.984126984126984e-4f64))))))))))));
                let XZ = if XY < PH { 1.0 } else { 0.0 };
                let YA = if XZ != 0.0 {
                    PH
                } else {
                    XY
                };
                let YB = (UX / (1.5e0f64 * MY)) * ((((OX + YA) - EQ) + TO) - UL);
                let YC = H * UR;
                let YD = if (if YB < YC { 1.0 } else { 0.0 }) != 0.0 && (if YC >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let YZ;
                if YD != 0.0 {
                    let YE = YC - YB;
                    let YF = YE * YE;
                    let YG = YC * YC;
                    let YH = (YF * YF) + (YG * YG);
                    let YX;
                    if YI != 0.0 {
                        let YS;
                        if YJ != 0.0 {
                            YS = B;
                        } else {
                            let YT;
                            if YK != 0.0 {
                                YT = BF;
                            } else {
                                let YU;
                                if YL != 0.0 {
                                    YU = BR;
                                } else {
                                    let YV = if YM != 0.0 {
                                        BL
                                    } else {
                                        A
                                    };
                                    YU = YV;
                                }
                                YT = YU;
                            }
                            YS = YT;
                        }
                        let mut YN = 0.0;
                        let mut YP = 0.0;
                        YN = A;
                        YP = YH;
                        loop {
                            let YO = if YN < YS { 1.0 } else { 0.0 };
                            if YO == 0.0 {
                                break;
                            }
                            let YQ = YP.sqrt();
                            let YR = YN + B;
                            YN = YR;
                            YP = YQ;
                        }
                        YX = YP;
                    } else {
                        let YW = YH.powf(2.5e-1f64);
                        YX = YW;
                    }
                    let YY = YC - ((YE * YC) * (B / YX));
                    YZ = YY;
                } else {
                    YZ = YB;
                }
                let ZA = UX - H;
                let ZB = if (if YZ > ZA { 1.0 } else { 0.0 }) != 0.0 && (if H >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ZX;
                if ZB != 0.0 {
                    let ZC = (YZ - UX) + H;
                    let ZD = ZC * ZC;
                    let ZE = H * H;
                    let ZF = (ZD * ZD) + (ZE * ZE);
                    let ZV;
                    if ZG != 0.0 {
                        let ZQ;
                        if ZH != 0.0 {
                            ZQ = B;
                        } else {
                            let ZR;
                            if ZI != 0.0 {
                                ZR = BF;
                            } else {
                                let ZS;
                                if ZJ != 0.0 {
                                    ZS = BR;
                                } else {
                                    let ZT = if ZK != 0.0 {
                                        BL
                                    } else {
                                        A
                                    };
                                    ZS = ZT;
                                }
                                ZR = ZS;
                            }
                            ZQ = ZR;
                        }
                        let mut ZL = 0.0;
                        let mut ZN = 0.0;
                        ZL = A;
                        ZN = ZF;
                        loop {
                            let ZM = if ZL < ZQ { 1.0 } else { 0.0 };
                            if ZM == 0.0 {
                                break;
                            }
                            let ZO = ZN.sqrt();
                            let ZP = ZL + B;
                            ZL = ZP;
                            ZN = ZO;
                        }
                        ZV = ZN;
                    } else {
                        let ZU = ZF.powf(2.5e-1f64);
                        ZV = ZU;
                    }
                    let ZW = ZA + ((ZC * H) * (B / ZV));
                    ZX = ZW;
                } else {
                    ZX = YZ;
                }
                let ZY = (-ZX) * ID;
                let AAA = ((((VA * H) / BF) / CI) + LL) - ((ZZ * H) / CI);
                let ALD;
                let ALE;
                let ALF;
                let ASY;
                let ATH;
                let AVA;
                let BHR;
                let CLN;
                if AAB != 0.0 {
                    let AAC = if A < AAA { 1.0 } else { 0.0 };
                    let AAD = if AAC != 0.0 {
                        B
                    } else {
                        BF
                    };
                    ALD = A;
                    ALE = A;
                    ALF = A;
                    ASY = AAD;
                    ATH = A;
                    AVA = A;
                    BHR = A;
                    CLN = A;
                } else {
                    let AAE = B + ((BL * ((LJ * UM) - B)) / (UQ * LK));
                    let AAF = if AAE >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let AAH = if AAF != 0.0 {
                        AAE
                    } else {
                        AAG
                    };
                    let AAI = UM + (((UQ * LJ) * I) * (B - (AAH.sqrt())));
                    let AAJ = if (LJ * AAI) < BR { 1.0 } else { 0.0 };
                    let ABI;
                    if AAJ != 0.0 {
                        let AAK = B / ((1.3094570021973102e-2f64 * LJ) * UP);
                        let AAM = AAL + (BR * AAK);
                        let AAN = (TW * AAK) * (LJ * (UM - PB));
                        let AAQ = (AAO - (AAL * (AAP + AAK))) + AAN;
                        let AAS = (((-2.916e3f64 - (AAL * AAK)) + AAN) + (((((BL * AAM) * AAM) * AAM) + (AAQ * AAQ)).sqrt())).powf(AAR);
                        let AAU = (((BR - ((AAT * AAM) / (BR * AAS))) + (2.6456684199469993e-1f64 * AAS)) * LL) + PB;
                        ABI = AAU;
                    } else {
                        let AAW = if (OX - AAV) <= TP { 1.0 } else { 0.0 };
                        let ABJ;
                        if AAW != 0.0 {
                            let AAX = H / CI;
                            let AAY = B / CP;
                            let AAZ = UM - (((B / (((B / TK) + AAX) + AAY)) * ((UM - VK) + ((AAY + (I * AAX)) * (-ZY)))) / TK);
                            ABJ = AAZ;
                        } else {
                            let ABA = UM - AAV;
                            let ABB = (((UU * ABA) * ABA).ln()) / (LJ + (BF / ABA));
                            let ABC = (ABB - AAI) - VX;
                            let ABD = (BL * ABB) * VX;
                            let ABE = if ABD > A { 1.0 } else { 0.0 };
                            let ABG = if ABE != 0.0 {
                                ABD
                            } else {
                                let ABF = -ABD;
                                ABF
                            };
                            let ABH = ABB - (I * (ABC + (((ABC * ABC) + ABG).sqrt())));
                            ABJ = ABH;
                        }
                        ABI = ABJ;
                    }
                    let ABK = if ABI > A { 1.0 } else { 0.0 };
                    let ABM = if ABK != 0.0 {
                        let ABL = ((1.2919089961638799e9f64 * ABI) / HY).sqrt();
                        ABL
                    } else {
                        A
                    };
                    let ABN = if ABM < H { 1.0 } else { 0.0 };
                    let ASZ = if ABN != 0.0 {
                        B
                    } else {
                        BF
                    };
                    let ABO = if (OX - AAV) <= TP { 1.0 } else { 0.0 };
                    let ACV;
                    let ACY;
                    if ABO != 0.0 {
                        let ABP = H / CI;
                        let ABQ = B / CP;
                        let ABR = UM - (((B / (((B / TK) + ABP) + ABQ)) * ((UM - VK) + ((ABQ + (I * ABP)) * (-ZY)))) / TK);
                        ACV = ABR;
                        ACY = ABR;
                    } else {
                        let ABS = H / CI;
                        let ABT = B / CP;
                        let ABU = UM - (((B / (((B / TK) + ABS) + ABT)) * ((UM - VK) + ((ABT + (I * ABS)) * (-ZY)))) / TK);
                        let ABV = UM - AAV;
                        let ABW = if ABV > A { 1.0 } else { 0.0 };
                        let ACW;
                        if ABW != 0.0 {
                            let ABY = ((((UU * ABV) * ABV).ln()) / (LJ + (BF / ABV))) * ABX;
                            let ABZ = ABY - LY;
                            let ACA = if (if ABU > ABZ { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                            let ACX;
                            if ACA != 0.0 {
                                let ACB = (ABU - ABY) + LY;
                                let ACC = ACB * ACB;
                                let ACD = (ACC * ACC) + 2.560000000000001e-2f64;
                                let ACT;
                                if ACE != 0.0 {
                                    let ACO;
                                    if ACF != 0.0 {
                                        ACO = B;
                                    } else {
                                        let ACP;
                                        if ACG != 0.0 {
                                            ACP = BF;
                                        } else {
                                            let ACQ;
                                            if ACH != 0.0 {
                                                ACQ = BR;
                                            } else {
                                                let ACR = if ACI != 0.0 {
                                                    BL
                                                } else {
                                                    A
                                                };
                                                ACQ = ACR;
                                            }
                                            ACP = ACQ;
                                        }
                                        ACO = ACP;
                                    }
                                    let mut ACJ = 0.0;
                                    let mut ACL = 0.0;
                                    ACJ = A;
                                    ACL = ACD;
                                    loop {
                                        let ACK = if ACJ < ACO { 1.0 } else { 0.0 };
                                        if ACK == 0.0 {
                                            break;
                                        }
                                        let ACM = ACL.sqrt();
                                        let ACN = ACJ + B;
                                        ACJ = ACN;
                                        ACL = ACM;
                                    }
                                    ACT = ACL;
                                } else {
                                    let ACS = ACD.powf(2.5e-1f64);
                                    ACT = ACS;
                                }
                                let ACU = ABZ + ((ACB * LY) * (B / ACT));
                                ACX = ACU;
                            } else {
                                ACX = ABU;
                            }
                            ACW = ACX;
                        } else {
                            ACW = ABU;
                        }
                        ACV = ACW;
                        ACY = ABU;
                    }
                    let ACZ = I * UZ;
                    let ADA = (ACV + (ACZ * CK)) - VK;
                    let ADB = if ADA < A { 1.0 } else { 0.0 };
                    let AFO;
                    if ADB != 0.0 {
                        let ADC = VI * CR;
                        let ADD = ADC * ADC;
                        let ADF = (-1.6e0f64 * ADA) + ADE;
                        let ADG = ADF * IM;
                        let ADH = (ADF - I) - ADG;
                        let ADI = (BL * ADF) * ADG;
                        let ADJ = if ADI > A { 1.0 } else { 0.0 };
                        let ADL = if ADJ != 0.0 {
                            ADI
                        } else {
                            let ADK = -ADI;
                            ADK
                        };
                        let ADM = (ADD * (ADF - (I * (ADH + (((ADH * ADH) + ADL).sqrt()))))) * LK;
                        let ADN = (ADA * (B - (ADM.sqrt()))) / (B - ADM);
                        AFO = ADN;
                    } else {
                        let ADO = -((VK - ACV) - (((UZ / BF) * H) / CI));
                        let ADP = (BF * ADO) + VM;
                        let ADQ = ADO * ADO;
                        let ADR = (ADP * ADP) - (BL * (ADQ + VJ));
                        let ADS = if ADR >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let ADU = if ADS != 0.0 {
                            ADR
                        } else {
                            ADT
                        };
                        let ADV = (ADP - (ADU.sqrt())) / BF;
                        let ADW = (((ADQ / VJ) / VU).ln()) / (LJ + (BF / ADO));
                        let ADX = if ADV < VH { 1.0 } else { 0.0 };
                        let AFP;
                        if ADX != 0.0 {
                            AFP = ADV;
                        } else {
                            let ADY = (ADW - ADV) - VX;
                            let ADZ = (BL * ADW) * VX;
                            let AEA = if ADZ > A { 1.0 } else { 0.0 };
                            let AEC = if AEA != 0.0 {
                                ADZ
                            } else {
                                let AEB = -ADZ;
                                AEB
                            };
                            let AED = ADW - (I * (ADY + (((ADY * ADY) + AEC).sqrt())));
                            AFP = AED;
                        }
                        AFO = AFP;
                    }
                    let mut AEE = 0.0;
                    let mut AEG = 0.0;
                    let mut AFR = 0.0;
                    AEE = A;
                    AEG = AFO;
                    AFR = A;
                    loop {
                        let AEF = if AEE < L { 1.0 } else { 0.0 };
                        if AEF == 0.0 {
                            break;
                        }
                        let AEH = LJ * AEG;
                        let AEI = (-AEH).exp();
                        let AEJ = if AEG > KK { 1.0 } else { 0.0 };
                        let AES;
                        let AFH;
                        if AEJ != 0.0 {
                            let AEK = AEH.exp();
                            let AEL = (-VI) * ((((AEI + AEH) - B) + (VU * (AEK - B))).sqrt());
                            let AEM = (EG / AEL) * (((-AEI) + B) + (VU * AEK));
                            AES = AEL;
                            AFH = AEM;
                        } else {
                            let AEN = if AEG < -1e-9f64 { 1.0 } else { 0.0 };
                            let AET;
                            let AFI;
                            if AEN != 0.0 {
                                let AEO = VI * (((AEI + AEH) - B).sqrt());
                                let AEP = (EG / AEO) * ((-AEI) + B);
                                AET = AEO;
                                AFI = AEP;
                            } else {
                                let AEQ = ((-((EG / LJ).sqrt())) * LJ) * AEG;
                                let AER = -((EG * LJ).sqrt());
                                AET = AEQ;
                                AFI = AER;
                            }
                            AES = AET;
                            AFH = AFI;
                        }
                        let AEU = ((AES * AES) + ((BL * VB) * VB)).sqrt();
                        let AEV = I * (B + (AES / AEU));
                        let AEW = (I * (AES + AEU)) + (IN * VB);
                        let AEX = if AEW < A { 1.0 } else { 0.0 };
                        let AEY;
                        let AFG;
                        if AEX != 0.0 {
                            AEY = A;
                            AFG = A;
                        } else {
                            AEY = AEW;
                            AFG = AEV;
                        }
                        let AEZ = (VA - AEY) - VD;
                        let AFA = (BL * VA) * VD;
                        let AFB = if AFA > A { 1.0 } else { 0.0 };
                        let AFD = if AFB != 0.0 {
                            AFA
                        } else {
                            let AFC = -AFA;
                            AFC
                        };
                        let AFE = ((AEZ * AEZ) + AFD).sqrt();
                        let AFF = VA - (I * (AEZ + AFE));
                        let AFJ = ((((AFF * AFF) / BF) / CI) / ED) / HY;
                        let AFK = AEG - ((((((ACV - AEG) + (AES / CP)) + (((AES + (UZ / BF)) * H) / CI)) - VK) + AFJ) / (((-1e0f64 + (AFH / CP)) + ((AFH * H) / CI)) + (((BF * AFJ) * (AFG * (AFH * (I * (B + (AEZ / AFE)))))) / AFF)));
                        let AFL = if ((AFK - AEG).abs()) < IM { 1.0 } else { 0.0 };
                        let AFM = if AFL != 0.0 {
                            L
                        } else {
                            AEE
                        };
                        let AFN = AFM + B;
                        AEE = AFN;
                        AEG = AFK;
                        AFR = AES;
                    }
                    let AFQ = VK + AEG;
                    let AFS = ACV + (CK * (ACZ + AFR));
                    ALD = ACV;
                    ALE = AFS;
                    ALF = AFQ;
                    ASY = ASZ;
                    ATH = AFR;
                    AVA = ACY;
                    BHR = ABM;
                    CLN = ACV;
                }
                let AFW = if (if AFT == B { 1.0 } else { 0.0 }) != 0.0 && (if OX > (AFU + AFV) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ASJ;
                let AUY;
                let CGE;
                let CGS;
                let CUO;
                let CVQ;
                if AFW != 0.0 {
                    let AFX = ((PM - FZ) + TO) - UL;
                    let AFZ = (((3.2043836e-19f64 * HY) * CI) / LJ).sqrt();
                    let AGA = (MI / HY) / HY;
                    let AGB = ((AFZ * AFZ) / TK) / TK;
                    let AGC = (AGB * LJ) / BF;
                    let AGD = ((((B / AGA) / AGB) * (AFX * AFX)).ln()) / (LJ + (BF / AFX));
                    let AGE = (AGD - (AFX + (AGC * (B - ((B + ((BL * ((LJ * AFX) - B)) / ((AGC * LJ) * BF))).sqrt()))))) - AFY;
                    let AGF = AGD - (I * (AGE + (((AGE * AGE) + ((BL * AFY) * AGD)).sqrt())));
                    let AGG = LJ * AGF;
                    let AGH = AGG - B;
                    let AGI = AGH + (AGA * (AGG.exp()));
                    let AGJ = if (if AGI > A { 1.0 } else { 0.0 }) != 0.0 && (if AGH > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ASK;
                    let AUZ;
                    let CUP;
                    let CVR;
                    if AGJ != 0.0 {
                        let AGL = -LJ;
                        let AGM = (((((BF * DO) / LJ) * AGK) * (AFZ * ((AGI.sqrt()) - (AGH.sqrt())))) * (-(((AGL * PL).exp()) - B))) * (B / CU);
                        let AGN = B + ((BL * ((LJ * UM) - B)) / (UQ * LK));
                        let AGO = if AGN < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let AGQ = if AGO != 0.0 {
                            AGP
                        } else {
                            AGN
                        };
                        let AGR = UM + (((UQ * LJ) * I) * (B - (AGQ.sqrt())));
                        let AGS = AGR - AGF;
                        let AGT = if AGS < A { 1.0 } else { 0.0 };
                        let AGU = if AGT != 0.0 {
                            A
                        } else {
                            AGS
                        };
                        let AGV = 1.3e0f64 * AGU;
                        let AGX = (AGV - PL) - AGW;
                        let AGY = AGV - (I * (AGX + (((AGX * AGX) + ((BL * AGV) * AGW)).sqrt())));
                        let AGZ = if AGY > AGU { 1.0 } else { 0.0 };
                        let AHA = if AGZ != 0.0 {
                            AGU
                        } else {
                            AGY
                        };
                        let AHB = CH * AX;
                        let AHC = DP * AX;
                        let AHD = CU * AX;
                        let AHE = if parameters[36] == A { 1.0 } else { 0.0 };
                        let AKW;
                        if AHE != 0.0 {
                            AKW = A;
                        } else {
                            let AHF = ((parameters[142] * ED) * AHC) * AHD;
                            let AHG = AHF / MC;
                            let AHH = (-(((((parameters[145] * RP) + TG) + TL) + LI) + parameters[144])) / AHB;
                            let mut AHI = 0.0;
                            let mut AIB = 0.0;
                            AHI = A;
                            AIB = A;
                            loop {
                                let AHJ = if AHI <= 9.9e1f64 { 1.0 } else { 0.0 };
                                if AHJ == 0.0 {
                                    break;
                                }
                                let AHK = (UM + PJ) - ((AHA * (AHI / AX)) + AGF);
                                let AHL = B - (AHK / 4.12e0f64);
                                let AHM = AHH + (AHK / AHB);
                                let AHN = AHM * AHM;
                                let AHO = (I * (AHL + (((AHL * AHL) + 4e-6f64).sqrt()))) + 1e-13f64;
                                let AHP = if AHO < A { 1.0 } else { 0.0 };
                                let AHQ = if AHP != 0.0 {
                                    A
                                } else {
                                    AHO
                                };
                                let AHR = parameters[143] * (B - ((AHQ.sqrt()) * AHQ));
                                let AHS = (-AHR) / AHM;
                                let AHT = if AHS < -3.4e1f64 { 1.0 } else { 0.0 };
                                let AHY = if AHT != 0.0 {
                                    A
                                } else {
                                    let AHU = AHS.exp();
                                    AHU
                                };
                                let AHW = (((AHV * AHG) * AHR) * AHR) * 7.38905609893065e0f64;
                                let AHX = if ((BF * AHM) + AHR) < A { 1.0 } else { 0.0 };
                                let AIC;
                                if AHX != 0.0 {
                                    AIC = AHW;
                                } else {
                                    let AHZ = (AHF * AHN) * AHY;
                                    let AIA = if (if AHZ < AHW { 1.0 } else { 0.0 }) != 0.0 || (if AHM < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let AID = if AIA != 0.0 {
                                        AHW
                                    } else {
                                        AHZ
                                    };
                                    AIC = AID;
                                }
                                let AIE = AIB + AIC;
                                let AIF = if AIC < KK { 1.0 } else { 0.0 };
                                let AIG = if AIF != 0.0 {
                                    AX
                                } else {
                                    AHI
                                };
                                let AIH = AIG + B;
                                AHI = AIH;
                                AIB = AIE;
                            }
                            AKW = AIB;
                        }
                        let AII = if (if FH <= A { 1.0 } else { 0.0 }) != 0.0 || (if N <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let AKV;
                        if AII != 0.0 {
                            AKV = A;
                        } else {
                            let AKL;
                            if EX != 0.0 {
                                let AIJ = TK * TK;
                                let AIK = IE / AIJ;
                                let AIM = B + (((BF / IE) * AIJ) * ((AFX - LL) - (AIL * RP)));
                                let AIN = (I * (AIM + (((AIM * AIM) + 4e-6f64).sqrt()))) + 1e-13f64;
                                let AIO = if AIN < A { 1.0 } else { 0.0 };
                                let AIP = if AIO != 0.0 {
                                    A
                                } else {
                                    AIN
                                };
                                let AIW = ((AIS * PL) + AGF) - ((AIT * AIU) * ((AFX * AIQ) + (AIK * (B - ((AIP + GD).sqrt())))));
                                let AIX = (I * (AIW + (((AIW * AIW) + 4e-4f64).sqrt()))) + 1e-12f64;
                                let AIY = if AIX < A { 1.0 } else { 0.0 };
                                let AKM = if AIY != 0.0 {
                                    A
                                } else {
                                    AIX
                                };
                                AKL = AKM;
                            } else {
                                let AJB = AIZ * AFX;
                                let AJC = TK * TK;
                                let AJD = IE / AJC;
                                let AJE = (BF / IE) * AJC;
                                let AJF = B + (AJE * ((AJB - LL) - (AIL * RP)));
                                let AJG = BF * (B + AJE);
                                let AJH = GD + AJG;
                                let AJI = if (if AJF < AJH { 1.0 } else { 0.0 }) != 0.0 && (if AJG >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let AKE;
                                if AJI != 0.0 {
                                    let AJJ = AJH - AJF;
                                    let AJK = AJJ * AJJ;
                                    let AJL = AJG * AJG;
                                    let AJM = (((AJK * AJK) * AJK) * AJK) + (((AJL * AJL) * AJL) * AJL);
                                    let AKC;
                                    if AJN != 0.0 {
                                        let AJX;
                                        if AJO != 0.0 {
                                            AJX = B;
                                        } else {
                                            let AJY;
                                            if AJP != 0.0 {
                                                AJY = BF;
                                            } else {
                                                let AJZ;
                                                if AJQ != 0.0 {
                                                    AJZ = BR;
                                                } else {
                                                    let AKA = if AJR != 0.0 {
                                                        BL
                                                    } else {
                                                        A
                                                    };
                                                    AJZ = AKA;
                                                }
                                                AJY = AJZ;
                                            }
                                            AJX = AJY;
                                        }
                                        let mut AJS = 0.0;
                                        let mut AJU = 0.0;
                                        AJS = A;
                                        AJU = AJM;
                                        loop {
                                            let AJT = if AJS < AJX { 1.0 } else { 0.0 };
                                            if AJT == 0.0 {
                                                break;
                                            }
                                            let AJV = AJU.sqrt();
                                            let AJW = AJS + B;
                                            AJS = AJW;
                                            AJU = AJV;
                                        }
                                        AKC = AJU;
                                    } else {
                                        let AKB = AJM.powf(1.25e-1f64);
                                        AKC = AKB;
                                    }
                                    let AKD = AJH - ((AJJ * AJG) * (B / AKC));
                                    AKE = AKD;
                                } else {
                                    AKE = AJF;
                                }
                                let AKF = if AKE <= A { 1.0 } else { 0.0 };
                                let AKH = if AKF != 0.0 {
                                    A
                                } else {
                                    let AKG = AKE.sqrt();
                                    AKG
                                };
                                let AKI = ((AIS * PL) + B) - ((CY / (AIT + CY)) * (AJB + (AJD * (B - AKH))));
                                let AKJ = (I * (AKI + (((AKI * AKI) + 4e-6f64).sqrt()))) + 1e-13f64;
                                let AKK = if AKJ < A { 1.0 } else { 0.0 };
                                let AKN = if AKK != 0.0 {
                                    A
                                } else {
                                    AKJ
                                };
                                AKL = AKN;
                            }
                            let AKO = AKL + GD;
                            let AKR = ((AKQ * AKO) * AGM) * (((-AKP) / AKO).exp());
                            AKV = AKR;
                        }
                        let AKT = if AKS == B { 1.0 } else { 0.0 };
                        let ASL;
                        if AKT != 0.0 {
                            let AKY = AGF - ((AKX * LL) * ((B + ((AKV + AKW) * (2.1633307652783932e-2f64 / ((((ED * H) * DP) * ((AGL * AKU).exp())) * (4.1046315303568966e26f64 + (2.4665765749313358e0f64 * HY)))))).ln()));
                            let AKZ = (-(((3.3163543761348e-29f64 * HY) * LL).sqrt())) * ((((((AGL * AKY).exp()) - B) + (LJ * AKY)).sqrt()) - (((((AGL * AGF).exp()) - B) + AGG).sqrt()));
                            let ASM = if ALA != 0.0 {
                                let ALC = 1e-5f64 * ALB;
                                ALC
                            } else {
                                AKZ
                            };
                            ASL = ASM;
                        } else {
                            ASL = A;
                        }
                        ASK = ASL;
                        AUZ = AGR;
                        CUP = AKV;
                        CVR = AGK;
                    } else {
                        ASK = A;
                        AUZ = AVA;
                        CUP = A;
                        CVR = A;
                    }
                    ASJ = ASK;
                    AUY = AUZ;
                    CGE = AGA;
                    CGS = AFZ;
                    CUO = CUP;
                    CVQ = CVR;
                } else {
                    ASJ = A;
                    AUY = AVA;
                    CGE = MJ;
                    CGS = MG;
                    CUO = A;
                    CVQ = A;
                }
                let mut ALG = 0.0;
                let mut ALI = 0.0;
                let mut ALW = 0.0;
                let mut AMD = 0.0;
                let mut APX = 0.0;
                let mut ASN = 0.0;
                let mut ASS = 0.0;
                let mut ATA = 0.0;
                let mut ATB = 0.0;
                let mut ATG = 0.0;
                ALG = B;
                ALI = ALF;
                ALW = ALD;
                AMD = ALE;
                APX = A;
                ASN = A;
                ASS = A;
                ATA = A;
                ATB = A;
                ATG = ATH;
                loop {
                    let ALH = if ALG <= L { 1.0 } else { 0.0 };
                    if ALH == 0.0 {
                        break;
                    }
                    let ALJ = ALI - VK;
                    let ALK = LJ * ALJ;
                    let ALL = (-ALK).exp();
                    let ALM = if ALJ < -1e-9f64 { 1.0 } else { 0.0 };
                    let APZ;
                    let AQF;
                    if ALM != 0.0 {
                        let ALN = VI * (((ALL + ALK) - B).sqrt());
                        let ALO = (EG * ((-ALL) + B)) / ALN;
                        APZ = ALN;
                        AQF = ALO;
                    } else {
                        let ALP = if ALJ > KK { 1.0 } else { 0.0 };
                        let AQA;
                        let AQG;
                        if ALP != 0.0 {
                            let ALQ = ALK.exp();
                            let ALR = (-VI) * ((((ALL + ALK) - B) + (VU * ((ALQ + ALK) - B))).sqrt());
                            let ALS = (EG * (((-ALL) + B) + (VU * (ALQ + B)))) / ALR;
                            AQA = ALR;
                            AQG = ALS;
                        } else {
                            let ALT = -VI;
                            let ALU = ALT * ALK;
                            let ALV = ALT * LJ;
                            AQA = ALU;
                            AQG = ALV;
                        }
                        APZ = AQA;
                        AQF = AQG;
                    }
                    let ALX = LJ * ALW;
                    let ALY = ALX.exp();
                    let ALZ = (((ZY * ZY) / (MO * MO)) + ((BF * MV) * ((ALY + ALX) - B))).sqrt();
                    let AMA = -MO;
                    let AMB = (AMA * ALZ) - ZY;
                    let AMC = AMA * ((((BF * LJ) * MV) * (ALY + B)) / (BF * ALZ));
                    let AME = (AMD - ALW) / UR;
                    let AMF = LJ * AME;
                    let AMG = -AMF;
                    let AMI = if AMG >= AMH { 1.0 } else { 0.0 };
                    let AMV;
                    if AMI != 0.0 {
                        AMV = AMJ;
                    } else {
                        let mut AMK = 0.0;
                        let mut AMN = 0.0;
                        AMK = AMG;
                        AMN = B;
                        loop {
                            let AMM = if AMK >= AML { 1.0 } else { 0.0 };
                            if AMM == 0.0 {
                                break;
                            }
                            let AMP = AMN * AMO;
                            let AMQ = AMK - AML;
                            AMK = AMQ;
                            AMN = AMP;
                        }
                        let AMR = AMN * (AMK.exp());
                        AMV = AMR;
                    }
                    let AMS = (((AMG.exp()) + AMF) - B).sqrt();
                    let AMT = if AME < -1e-9f64 { 1.0 } else { 0.0 };
                    let ANF;
                    let AOI;
                    let AOM;
                    if AMT != 0.0 {
                        let AMU = MO * AMS;
                        let AMW = (((MO * LJ) * ((-AMV) + B)) / (BF * AMS)) / UR;
                        let AMX = -AMW;
                        ANF = AMU;
                        AOI = AMW;
                        AOM = AMX;
                    } else {
                        let AMY = if AME > KK { 1.0 } else { 0.0 };
                        let ANG;
                        let AOJ;
                        let AON;
                        if AMY != 0.0 {
                            let AMZ = AMA * AMS;
                            let ANA = (((AMA * LJ) * ((-AMV) + B)) / (BF * AMS)) / UR;
                            let ANB = -ANA;
                            ANG = AMZ;
                            AOJ = ANA;
                            AON = ANB;
                        } else {
                            let ANC = (AMA * AMF) / MN;
                            let AND = (AMA * LJ) / MN;
                            let ANE = -AND;
                            ANG = ANC;
                            AOJ = AND;
                            AON = ANE;
                        }
                        ANF = ANG;
                        AOI = AOJ;
                        AOM = AON;
                    }
                    let ANH = -UY;
                    let ANI = A - ANH;
                    let ANJ = if (if ANF > ANI { 1.0 } else { 0.0 }) != 0.0 && (if ANH >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AOK;
                    let AOP;
                    if ANJ != 0.0 {
                        let ANK = ANF + ANH;
                        let ANL = ANK * ANK;
                        let ANM = ANH * ANH;
                        let ANN = ANM * ANM;
                        let ANO = (ANL * ANL) + ANN;
                        let AOE;
                        if ANP != 0.0 {
                            let ANZ;
                            if ANQ != 0.0 {
                                ANZ = B;
                            } else {
                                let AOA;
                                if ANR != 0.0 {
                                    AOA = BF;
                                } else {
                                    let AOB;
                                    if ANS != 0.0 {
                                        AOB = BR;
                                    } else {
                                        let AOC = if ANT != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        AOB = AOC;
                                    }
                                    AOA = AOB;
                                }
                                ANZ = AOA;
                            }
                            let mut ANU = 0.0;
                            let mut ANW = 0.0;
                            ANU = A;
                            ANW = ANO;
                            loop {
                                let ANV = if ANU < ANZ { 1.0 } else { 0.0 };
                                if ANV == 0.0 {
                                    break;
                                }
                                let ANX = ANW.sqrt();
                                let ANY = ANU + B;
                                ANU = ANY;
                                ANW = ANX;
                            }
                            AOE = ANW;
                        } else {
                            let AOD = ANO.powf(2.5e-1f64);
                            AOE = AOD;
                        }
                        let AOF = B / AOE;
                        let AOG = ((ANH * ANN) * AOF) / ANO;
                        let AOH = ANI + ((ANK * ANH) * AOF);
                        AOK = AOG;
                        AOP = AOH;
                    } else {
                        AOK = B;
                        AOP = ANF;
                    }
                    let AOL = AOI * AOK;
                    let AOO = AOM * AOK;
                    let AOQ = UZ - ZY;
                    let AOR = -AOQ;
                    let AOS = AOQ + AOR;
                    let AOT = if (if AOP < AOS { 1.0 } else { 0.0 }) != 0.0 && (if AOR >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let APS;
                    let APV;
                    if AOT != 0.0 {
                        let AOU = AOS - AOP;
                        let AOV = AOU * AOU;
                        let AOW = AOR * AOR;
                        let AOX = AOW * AOW;
                        let AOY = (AOV * AOV) + AOX;
                        let APO;
                        if AOZ != 0.0 {
                            let APJ;
                            if APA != 0.0 {
                                APJ = B;
                            } else {
                                let APK;
                                if APB != 0.0 {
                                    APK = BF;
                                } else {
                                    let APL;
                                    if APC != 0.0 {
                                        APL = BR;
                                    } else {
                                        let APM = if APD != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        APL = APM;
                                    }
                                    APK = APL;
                                }
                                APJ = APK;
                            }
                            let mut APE = 0.0;
                            let mut APG = 0.0;
                            APE = A;
                            APG = AOY;
                            loop {
                                let APF = if APE < APJ { 1.0 } else { 0.0 };
                                if APF == 0.0 {
                                    break;
                                }
                                let APH = APG.sqrt();
                                let API = APE + B;
                                APE = API;
                                APG = APH;
                            }
                            APO = APG;
                        } else {
                            let APN = AOY.powf(2.5e-1f64);
                            APO = APN;
                        }
                        let APP = B / APO;
                        let APQ = ((AOR * AOX) * APP) / AOY;
                        let APR = AOS - ((AOU * AOR) * APP);
                        APS = APQ;
                        APV = APR;
                    } else {
                        APS = B;
                        APV = AOP;
                    }
                    let APT = AOO * APS;
                    let APU = AOL * APS;
                    let APW = ZY + APV;
                    let APY = if APX == B { 1.0 } else { 0.0 };
                    let ASC;
                    let ASE;
                    let ASF;
                    let ASG;
                    let ASH;
                    let ASO;
                    if APY != 0.0 {
                        ASC = L;
                        ASE = ALI;
                        ASF = ALW;
                        ASG = AMD;
                        ASH = APX;
                        ASO = ALG;
                    } else {
                        let AQB = (ALW - UM) - (SP * ((((APZ + ZY) + AMB) + APV) + ASJ));
                        let AQC = B - (SP * (AMC + APT));
                        let AQD = -SP;
                        let AQE = AQD * APU;
                        let AQH = AQD * AQF;
                        let AQI = AMD - (ALW + (CK * ((I * UZ) + APZ)));
                        let AQK = -(CK * AQF);
                        let AQL = (ALI - AMD) - (CQ * APZ);
                        let AQN = B - (CQ * AQF);
                        let AQO = AQC * AQN;
                        let AQP = AQC * AQK;
                        let AQQ = AQE * AQJ;
                        let AQR = AQH * AQJ;
                        let AQS = -(B / ((((AQO - (AQP * AQM)) - (AQQ * AQN)) + (AQR * AQM)) + GD));
                        let AQT = AQS * ((((AQN - (AQK * AQM)) * AQB) + (((AQH * AQM) - (AQE * AQN)) * AQI)) + (((AQE * AQK) - AQH) * AQL));
                        let AQU = AQS * (((AQN * AQB) + (AQO * AQI)) + ((AQR - AQP) * AQL));
                        let AQV = AQS * ((AQB + (((-AQC) * AQM) * AQI)) + ((AQC - AQQ) * AQL));
                        let AQW = AQT.abs();
                        let AQX = AQU.abs();
                        let AQY = if AQW < AQX { 1.0 } else { 0.0 };
                        let AQZ = if AQY != 0.0 {
                            AQX
                        } else {
                            AQW
                        };
                        let ARA = AQV.abs();
                        let ARB = if AQZ < ARA { 1.0 } else { 0.0 };
                        let ARK = if ARB != 0.0 {
                            ARA
                        } else {
                            AQZ
                        };
                        let ARD = if ALG > ARC { 1.0 } else { 0.0 };
                        let ARL;
                        if ARD != 0.0 {
                            ARL = ARE;
                        } else {
                            let ARG = if ALG > ARF { 1.0 } else { 0.0 };
                            let ARM;
                            if ARG != 0.0 {
                                ARM = ARE;
                            } else {
                                let ARH = if ALG > OP { 1.0 } else { 0.0 };
                                let ARN;
                                if ARH != 0.0 {
                                    ARN = ARI;
                                } else {
                                    let ARJ = if ALG > J { 1.0 } else { 0.0 };
                                    let ARO = if ARJ != 0.0 {
                                        KY
                                    } else {
                                        B
                                    };
                                    ARN = ARO;
                                }
                                ARM = ARN;
                            }
                            ARL = ARM;
                        }
                        let ARP = BG / ARL;
                        let ARQ = if ARK > ARP { 1.0 } else { 0.0 };
                        let ARV;
                        let ARX;
                        let ARZ;
                        if ARQ != 0.0 {
                            let ARR = ARP / ARK;
                            let ARS = AQT * ARR;
                            let ART = AQU * ARR;
                            let ARU = AQV * ARR;
                            ARV = ARS;
                            ARX = ART;
                            ARZ = ARU;
                        } else {
                            ARV = AQT;
                            ARX = AQU;
                            ARZ = AQV;
                        }
                        let ARW = ALW + ARV;
                        let ARY = AMD + ARX;
                        let ASA = ALI + ARZ;
                        let ASB = if ARK < (PH * ARL) { 1.0 } else { 0.0 };
                        let ASI = if ASB != 0.0 {
                            B
                        } else {
                            APX
                        };
                        ASC = ALG;
                        ASE = ASA;
                        ASF = ARW;
                        ASG = ARY;
                        ASH = ASI;
                        ASO = ASN;
                    }
                    let ASD = ASC + B;
                    ALG = ASD;
                    ALI = ASE;
                    ALW = ASF;
                    AMD = ASG;
                    APX = ASH;
                    ASN = ASO;
                    ASS = AMB;
                    ATA = APV;
                    ATB = APW;
                    ATG = APZ;
                }
                let ASP = if ASN > A { 1.0 } else { 0.0 };
                if ASP != 0.0 {
                } else {
                }
                let ASQ = if APX == A { 1.0 } else { 0.0 };
                let ASR;
                let ATJ;
                let ATK;
                if ASQ != 0.0 {
                    ASR = ALD;
                    ATJ = ALE;
                    ATK = ALF;
                } else {
                    ASR = ALW;
                    ATJ = AMD;
                    ATK = ALI;
                }
                let AST = -ASS;
                let ASU = if AST <= GD { 1.0 } else { 0.0 };
                let ASV = if ASU != 0.0 {
                    GD
                } else {
                    AST
                };
                let ASW = ASV * SP;
                let ASX = if (if ASR <= A { 1.0 } else { 0.0 }) != 0.0 && A != 0.0 { 1.0 } else { 0.0 };
                let BKZ;
                let BLF;
                let BZY;
                let CAB;
                let CAE;
                let CAJ;
                let CAQ;
                let CBK;
                let CCD;
                let CCK;
                let CCT;
                let CCW;
                let CHW;
                let CVF;
                let EDX;
                let EGJ;
                let EGO;
                let EGS;
                let EGW;
                if ASX != 0.0 {
                    let ATC = -5e-1f64 * ((ZY + ATA) + ATB);
                    let ATD = ((-DR) * CV) * ATC;
                    let ATE = ATD * I;
                    let ATF = ATD * 5e-1f64;
                    let ATI = (ATG * CV) * DR;
                    BKZ = ASY;
                    BLF = A;
                    BZY = A;
                    CAB = A;
                    CAE = A;
                    CAJ = B;
                    CAQ = ASR;
                    CBK = A;
                    CCD = ATC;
                    CCK = A;
                    CCT = ATG;
                    CCW = A;
                    CHW = A;
                    CVF = ATJ;
                    EDX = ASR;
                    EGJ = ATD;
                    EGO = ATI;
                    EGS = ATE;
                    EGW = ATF;
                } else {
                    let ATL = IE / (TK * TK);
                    let ATM = BF / ATL;
                    let ATN = B + (ATM * (UM - GD));
                    let ATO = B + ATM;
                    let ATP = if (if ATN < ATO { 1.0 } else { 0.0 }) != 0.0 && (if ATO >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AUL;
                    if ATP != 0.0 {
                        let ATQ = ATO - ATN;
                        let ATR = ATQ * ATQ;
                        let ATS = ATO * ATO;
                        let ATT = (((ATR * ATR) * ATR) * ATR) + (((ATS * ATS) * ATS) * ATS);
                        let AUJ;
                        if ATU != 0.0 {
                            let AUE;
                            if ATV != 0.0 {
                                AUE = B;
                            } else {
                                let AUF;
                                if ATW != 0.0 {
                                    AUF = BF;
                                } else {
                                    let AUG;
                                    if ATX != 0.0 {
                                        AUG = BR;
                                    } else {
                                        let AUH = if ATY != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        AUG = AUH;
                                    }
                                    AUF = AUG;
                                }
                                AUE = AUF;
                            }
                            let mut ATZ = 0.0;
                            let mut AUB = 0.0;
                            ATZ = A;
                            AUB = ATT;
                            loop {
                                let AUA = if ATZ < AUE { 1.0 } else { 0.0 };
                                if AUA == 0.0 {
                                    break;
                                }
                                let AUC = AUB.sqrt();
                                let AUD = ATZ + B;
                                ATZ = AUD;
                                AUB = AUC;
                            }
                            AUJ = AUB;
                        } else {
                            let AUI = ATT.powf(1.25e-1f64);
                            AUJ = AUI;
                        }
                        let AUK = ATO - ((ATQ * ATO) * (B / AUJ));
                        AUL = AUK;
                    } else {
                        AUL = ATN;
                    }
                    let AUM = UM + (ATL * (B - (AUL.sqrt())));
                    let AUN = (I * (AUM + (((AUM * AUM) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let AUO = if AUN < A { 1.0 } else { 0.0 };
                    let AUP = if AUO != 0.0 {
                        A
                    } else {
                        AUN
                    };
                    let AUQ = OR / AUP;
                    let AUS = B + ((AUQ.powf((AUR - B))) * AUQ);
                    let AUT = OR / ((AUS.powf(((B / AUR) - B))) * AUS);
                    let AUU = if AUT < A { 1.0 } else { 0.0 };
                    let BAC;
                    let BAH;
                    let BAL;
                    let BHQ;
                    let BIE;
                    let BLA;
                    if AUU != 0.0 {
                        BAC = ATJ;
                        BAH = ASR;
                        BAL = ATK;
                        BHQ = BHR;
                        BIE = A;
                        BLA = ASY;
                    } else {
                        let BAD;
                        let BAI;
                        let BAM;
                        let BHS;
                        let BIF;
                        let BLB;
                        if AUV != 0.0 {
                            let AUW = if A < AAA { 1.0 } else { 0.0 };
                            let AUX = if AUW != 0.0 {
                                B
                            } else {
                                BF
                            };
                            BAD = A;
                            BAI = A;
                            BAM = A;
                            BHS = BHR;
                            BIF = A;
                            BLB = AUX;
                        } else {
                            let AVB = AUY - ASR;
                            let AVC = if AVB >= A { 1.0 } else { 0.0 };
                            let AVD = if AVC != 0.0 {
                                AVB
                            } else {
                                A
                            };
                            let AVE = ((1.3e0f64 * AVD) - AUT) - AGW;
                            let AVF = (BL * (1.3e0f64 * AVD)) * AGW;
                            let AVG = if AVF > A { 1.0 } else { 0.0 };
                            let AVI = if AVG != 0.0 {
                                AVF
                            } else {
                                let AVH = -AVF;
                                AVH
                            };
                            let AVJ = (1.3e0f64 * AVD) - (I * (AVE + (((AVE * AVE) + AVI).sqrt())));
                            let AVK = if AVJ <= AVD { 1.0 } else { 0.0 };
                            let AVL = if AVK != 0.0 {
                                AVJ
                            } else {
                                AVD
                            };
                            let AVM = if AVL < A { 1.0 } else { 0.0 };
                            let AVO;
                            if AVM != 0.0 {
                                AVO = A;
                            } else {
                                let AVN = if AVL > AUT { 1.0 } else { 0.0 };
                                let AVP = if AVN != 0.0 {
                                    AUT
                                } else {
                                    AVL
                                };
                                AVO = AVP;
                            }
                            let AVQ = ASR + AVO;
                            let AVR = if AVQ < AAA { 1.0 } else { 0.0 };
                            let AYG;
                            if AVR != 0.0 {
                                let AVS = if VP >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                                let AVU = if AVS != 0.0 {
                                    VP
                                } else {
                                    AVT
                                };
                                let AVV = (VN - (AVU.sqrt())) / BF;
                                let AVW = if AVV < VH { 1.0 } else { 0.0 };
                                let AYH;
                                if AVW != 0.0 {
                                    AYH = AVV;
                                } else {
                                    let AVX = (VV - AVV) - VX;
                                    let AVY = (BL * VV) * VX;
                                    let AVZ = if AVY > A { 1.0 } else { 0.0 };
                                    let AWB = if AVZ != 0.0 {
                                        AVY
                                    } else {
                                        let AWA = -AVY;
                                        AWA
                                    };
                                    let AWC = VV - (I * (AVX + (((AVX * AVX) + AWB).sqrt())));
                                    AYH = AWC;
                                }
                                AYG = AYH;
                            } else {
                                let AWD = -((VK - AVQ) - (((UZ / BF) * H) / CI));
                                let AWE = (BF * AWD) + VM;
                                let AWF = AWD * AWD;
                                let AWG = (AWE * AWE) - (BL * (AWF + VJ));
                                let AWH = if AWG >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                                let AWJ = if AWH != 0.0 {
                                    AWG
                                } else {
                                    AWI
                                };
                                let AWK = (AWE - (AWJ.sqrt())) / BF;
                                let AWL = (((AWF / VJ) / VU).ln()) / (LJ + (BF / AWD));
                                let AWM = if AWK < VH { 1.0 } else { 0.0 };
                                let AYI;
                                if AWM != 0.0 {
                                    AYI = AWK;
                                } else {
                                    let AWN = (AWL - AWK) - VX;
                                    let AWO = (BL * AWL) * VX;
                                    let AWP = if AWO > A { 1.0 } else { 0.0 };
                                    let AWR = if AWP != 0.0 {
                                        AWO
                                    } else {
                                        let AWQ = -AWO;
                                        AWQ
                                    };
                                    let AWS = AWL - (I * (AWN + (((AWN * AWN) + AWR).sqrt())));
                                    AYI = AWS;
                                }
                                AYG = AYI;
                            }
                            let AWT = if ((1.2919089961638799e9f64 * AVQ) / HY) > A { 1.0 } else { 0.0 };
                            let BHT = if AWT != 0.0 {
                                let AWU = ((1.2919089961638799e9f64 * AVQ) / HY).sqrt();
                                AWU
                            } else {
                                A
                            };
                            let AWV = if AVR != 0.0 && A != 0.0 { 1.0 } else { 0.0 };
                            let AZZ;
                            let BAN;
                            let BIG;
                            let BLC;
                            if AWV != 0.0 {
                                let mut AWW = 0.0;
                                let mut AWY = 0.0;
                                let mut AYK = 0.0;
                                AWW = A;
                                AWY = AYG;
                                AYK = A;
                                loop {
                                    let AWX = if AWW < L { 1.0 } else { 0.0 };
                                    if AWX == 0.0 {
                                        break;
                                    }
                                    let AWZ = LJ * AWY;
                                    let AXA = (-AWZ).exp();
                                    let AXB = if AWY > KK { 1.0 } else { 0.0 };
                                    let AXK;
                                    let AXZ;
                                    if AXB != 0.0 {
                                        let AXC = AWZ.exp();
                                        let AXD = (-VI) * ((((AXA + AWZ) - B) + (VU * (AXC - B))).sqrt());
                                        let AXE = (EG / AXD) * (((-AXA) + B) + (VU * AXC));
                                        AXK = AXD;
                                        AXZ = AXE;
                                    } else {
                                        let AXF = if AWY < -1e-9f64 { 1.0 } else { 0.0 };
                                        let AXL;
                                        let AYA;
                                        if AXF != 0.0 {
                                            let AXG = VI * (((AXA + AWZ) - B).sqrt());
                                            let AXH = (EG / AXG) * ((-AXA) + B);
                                            AXL = AXG;
                                            AYA = AXH;
                                        } else {
                                            let AXI = ((-((EG / LJ).sqrt())) * LJ) * AWY;
                                            let AXJ = -((EG * LJ).sqrt());
                                            AXL = AXI;
                                            AYA = AXJ;
                                        }
                                        AXK = AXL;
                                        AXZ = AYA;
                                    }
                                    let AXM = ((AXK * AXK) + ((BL * VB) * VB)).sqrt();
                                    let AXN = I * (B + (AXK / AXM));
                                    let AXO = (I * (AXK + AXM)) + (IN * VB);
                                    let AXP = if AXO < A { 1.0 } else { 0.0 };
                                    let AXQ;
                                    let AXY;
                                    if AXP != 0.0 {
                                        AXQ = A;
                                        AXY = A;
                                    } else {
                                        AXQ = AXO;
                                        AXY = AXN;
                                    }
                                    let AXR = (VA - AXQ) - VD;
                                    let AXS = (BL * VA) * VD;
                                    let AXT = if AXS > A { 1.0 } else { 0.0 };
                                    let AXV = if AXT != 0.0 {
                                        AXS
                                    } else {
                                        let AXU = -AXS;
                                        AXU
                                    };
                                    let AXW = ((AXR * AXR) + AXV).sqrt();
                                    let AXX = VA - (I * (AXR + AXW));
                                    let AYB = ((((AXX * AXX) / BF) / CI) / ED) / HY;
                                    let AYC = AWY - (((((-AWY) + (AXK / CP)) - VK) + AYB) / ((-1e0f64 + (AXZ / CP)) + (((BF * AYB) * (AXY * (AXZ * (I * (B + (AXR / AXW)))))) / AXX)));
                                    let AYD = if ((AYC - AWY).abs()) < PH { 1.0 } else { 0.0 };
                                    let AYE = if AYD != 0.0 {
                                        L
                                    } else {
                                        AWW
                                    };
                                    let AYF = AYE + B;
                                    AWW = AYF;
                                    AWY = AYC;
                                    AYK = AXK;
                                }
                                let AYJ = VK + AWY;
                                let AYL = AYJ - (AYK / CP);
                                AZZ = AYL;
                                BAN = AYJ;
                                BIG = AYK;
                                BLC = B;
                            } else {
                                let mut AYM = 0.0;
                                let mut AYO = 0.0;
                                let mut AZX = 0.0;
                                AYM = A;
                                AYO = AYG;
                                AZX = A;
                                loop {
                                    let AYN = if AYM < L { 1.0 } else { 0.0 };
                                    if AYN == 0.0 {
                                        break;
                                    }
                                    let AYP = LJ * AYO;
                                    let AYQ = (-AYP).exp();
                                    let AYR = if AYO > KK { 1.0 } else { 0.0 };
                                    let AZA;
                                    let AZP;
                                    if AYR != 0.0 {
                                        let AYS = AYP.exp();
                                        let AYT = (-VI) * ((((AYQ + AYP) - B) + (VU * (AYS - B))).sqrt());
                                        let AYU = (EG / AYT) * (((-AYQ) + B) + (VU * AYS));
                                        AZA = AYT;
                                        AZP = AYU;
                                    } else {
                                        let AYV = if AYO < -1e-9f64 { 1.0 } else { 0.0 };
                                        let AZB;
                                        let AZQ;
                                        if AYV != 0.0 {
                                            let AYW = VI * (((AYQ + AYP) - B).sqrt());
                                            let AYX = (EG / AYW) * ((-AYQ) + B);
                                            AZB = AYW;
                                            AZQ = AYX;
                                        } else {
                                            let AYY = ((-((EG / LJ).sqrt())) * LJ) * AYO;
                                            let AYZ = -((EG * LJ).sqrt());
                                            AZB = AYY;
                                            AZQ = AYZ;
                                        }
                                        AZA = AZB;
                                        AZP = AZQ;
                                    }
                                    let AZC = ((AZA * AZA) + ((BL * VB) * VB)).sqrt();
                                    let AZD = I * (B + (AZA / AZC));
                                    let AZE = (I * (AZA + AZC)) + (IN * VB);
                                    let AZF = if AZE < A { 1.0 } else { 0.0 };
                                    let AZG;
                                    let AZO;
                                    if AZF != 0.0 {
                                        AZG = A;
                                        AZO = A;
                                    } else {
                                        AZG = AZE;
                                        AZO = AZD;
                                    }
                                    let AZH = (VA - AZG) - VD;
                                    let AZI = (BL * VA) * VD;
                                    let AZJ = if AZI > A { 1.0 } else { 0.0 };
                                    let AZL = if AZJ != 0.0 {
                                        AZI
                                    } else {
                                        let AZK = -AZI;
                                        AZK
                                    };
                                    let AZM = ((AZH * AZH) + AZL).sqrt();
                                    let AZN = VA - (I * (AZH + AZM));
                                    let AZR = ((((AZN * AZN) / BF) / CI) / ED) / HY;
                                    let AZS = AYO - ((((((AVQ - AYO) + (AZA / CP)) + (((AZA + (UZ / BF)) * H) / CI)) - VK) + AZR) / (((-1e0f64 + (AZP / CP)) + ((AZP * H) / CI)) + (((BF * AZR) * (AZO * (AZP * (I * (B + (AZH / AZM)))))) / AZN)));
                                    let AZT = if ((AZS - AYO).abs()) < PH { 1.0 } else { 0.0 };
                                    let AZU = if AZT != 0.0 {
                                        L
                                    } else {
                                        AYM
                                    };
                                    let AZV = AZU + B;
                                    AYM = AZV;
                                    AYO = AZS;
                                    AZX = AZA;
                                }
                                let AZW = VK + AYO;
                                let AZY = AZW - (AZX / CP);
                                AZZ = AZY;
                                BAN = AZW;
                                BIG = AZX;
                                BLC = BF;
                            }
                            let BAA = if AZZ < A { 1.0 } else { 0.0 };
                            let BAE = if BAA != 0.0 {
                                A
                            } else {
                                AZZ
                            };
                            BAD = BAE;
                            BAI = AVQ;
                            BAM = BAN;
                            BHS = BHT;
                            BIF = BIG;
                            BLB = BLC;
                        }
                        BAC = BAD;
                        BAH = BAI;
                        BAL = BAM;
                        BHQ = BHS;
                        BIE = BIF;
                        BLA = BLB;
                    }
                    let BAB = if ASR < A { 1.0 } else { 0.0 };
                    let BAG = if BAB != 0.0 {
                        ASR
                    } else {
                        BAH
                    };
                    let BAF = if BAC < M { 1.0 } else { 0.0 };
                    let BAK = if BAF != 0.0 {
                        let BAJ = BAG + (CK * ((I * UZ) + ATG));
                        BAJ
                    } else {
                        BAC
                    };
                    let mut BAO = 0.0;
                    let mut BAQ = 0.0;
                    let mut BBE = 0.0;
                    let mut BBK = 0.0;
                    let mut BFC = 0.0;
                    let mut BHK = 0.0;
                    let mut BHV = 0.0;
                    let mut BIA = 0.0;
                    let mut BID = 0.0;
                    BAO = B;
                    BAQ = BAL;
                    BBE = BAG;
                    BBK = BAK;
                    BFC = A;
                    BHK = A;
                    BHV = A;
                    BIA = A;
                    BID = BIE;
                    loop {
                        let BAP = if BAO <= L { 1.0 } else { 0.0 };
                        if BAP == 0.0 {
                            break;
                        }
                        let BAR = BAQ - VK;
                        let BAS = LJ * BAR;
                        let BAT = (-BAS).exp();
                        let BAU = if BAR < -1e-9f64 { 1.0 } else { 0.0 };
                        let BFE;
                        let BFK;
                        if BAU != 0.0 {
                            let BAV = VI * (((BAT + BAS) - B).sqrt());
                            let BAW = (EG * ((-BAT) + B)) / BAV;
                            BFE = BAV;
                            BFK = BAW;
                        } else {
                            let BAX = if BAR > KK { 1.0 } else { 0.0 };
                            let BFF;
                            let BFL;
                            if BAX != 0.0 {
                                let BAY = BAS.exp();
                                let BAZ = (-VI) * ((((BAT + BAS) - B) + (VU * ((BAY + BAS) - B))).sqrt());
                                let BBA = (EG * (((-BAT) + B) + (VU * (BAY + B)))) / BAZ;
                                BFF = BAZ;
                                BFL = BBA;
                            } else {
                                let BBB = -VI;
                                let BBC = BBB * BAS;
                                let BBD = BBB * LJ;
                                BFF = BBC;
                                BFL = BBD;
                            }
                            BFE = BFF;
                            BFK = BFL;
                        }
                        let BBF = (LJ * (BBE - AUT)).exp();
                        let BBG = (((ZY * ZY) / (MO * MO)) + ((BF * MV) * ((BBF + BAS) - B))).sqrt();
                        let BBH = -MO;
                        let BBI = (BBH * BBG) - ZY;
                        let BBJ = BBH * ((((BF * LJ) * MV) * (BBF + B)) / (BF * BBG));
                        let BBL = (BBK - BBE) / UR;
                        let BBM = LJ * BBL;
                        let BBN = -BBM;
                        let BBO = if BBN >= AMH { 1.0 } else { 0.0 };
                        let BBW;
                        let BCA;
                        if BBO != 0.0 {
                            let BBP = AMJ * ((B + BBN) - AMH);
                            BBW = BBP;
                            BCA = AMJ;
                        } else {
                            let mut BBQ = 0.0;
                            let mut BBS = 0.0;
                            BBQ = BBN;
                            BBS = B;
                            loop {
                                let BBR = if BBQ >= AML { 1.0 } else { 0.0 };
                                if BBR == 0.0 {
                                    break;
                                }
                                let BBT = BBS * AMO;
                                let BBU = BBQ - AML;
                                BBQ = BBU;
                                BBS = BBT;
                            }
                            let BBV = BBS * (BBQ.exp());
                            BBW = BBV;
                            BCA = BBV;
                        }
                        let BBX = ((BBW + BBM) - B).sqrt();
                        let BBY = if BBL < -1e-9f64 { 1.0 } else { 0.0 };
                        let BCK;
                        let BDN;
                        let BDR;
                        if BBY != 0.0 {
                            let BBZ = MO * BBX;
                            let BCB = (((MO * LJ) * ((-BCA) + B)) / (BF * BBX)) / UR;
                            let BCC = -BCB;
                            BCK = BBZ;
                            BDN = BCB;
                            BDR = BCC;
                        } else {
                            let BCD = if BBL > KK { 1.0 } else { 0.0 };
                            let BCL;
                            let BDO;
                            let BDS;
                            if BCD != 0.0 {
                                let BCE = BBH * BBX;
                                let BCF = (((BBH * LJ) * ((-BCA) + B)) / (BF * BBX)) / UR;
                                let BCG = -BCF;
                                BCL = BCE;
                                BDO = BCF;
                                BDS = BCG;
                            } else {
                                let BCH = (BBH * BBM) / MN;
                                let BCI = (BBH * LJ) / MN;
                                let BCJ = -BCI;
                                BCL = BCH;
                                BDO = BCI;
                                BDS = BCJ;
                            }
                            BCK = BCL;
                            BDN = BDO;
                            BDR = BDS;
                        }
                        let BCM = -UY;
                        let BCN = A - BCM;
                        let BCO = if (if BCK > BCN { 1.0 } else { 0.0 }) != 0.0 && (if BCM >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BDP;
                        let BDU;
                        if BCO != 0.0 {
                            let BCP = BCK + BCM;
                            let BCQ = BCP * BCP;
                            let BCR = BCM * BCM;
                            let BCS = BCR * BCR;
                            let BCT = (BCQ * BCQ) + BCS;
                            let BDJ;
                            if BCU != 0.0 {
                                let BDE;
                                if BCV != 0.0 {
                                    BDE = B;
                                } else {
                                    let BDF;
                                    if BCW != 0.0 {
                                        BDF = BF;
                                    } else {
                                        let BDG;
                                        if BCX != 0.0 {
                                            BDG = BR;
                                        } else {
                                            let BDH = if BCY != 0.0 {
                                                BL
                                            } else {
                                                A
                                            };
                                            BDG = BDH;
                                        }
                                        BDF = BDG;
                                    }
                                    BDE = BDF;
                                }
                                let mut BCZ = 0.0;
                                let mut BDB = 0.0;
                                BCZ = A;
                                BDB = BCT;
                                loop {
                                    let BDA = if BCZ < BDE { 1.0 } else { 0.0 };
                                    if BDA == 0.0 {
                                        break;
                                    }
                                    let BDC = BDB.sqrt();
                                    let BDD = BCZ + B;
                                    BCZ = BDD;
                                    BDB = BDC;
                                }
                                BDJ = BDB;
                            } else {
                                let BDI = BCT.powf(2.5e-1f64);
                                BDJ = BDI;
                            }
                            let BDK = B / BDJ;
                            let BDL = ((BCM * BCS) * BDK) / BCT;
                            let BDM = BCN + ((BCP * BCM) * BDK);
                            BDP = BDL;
                            BDU = BDM;
                        } else {
                            BDP = B;
                            BDU = BCK;
                        }
                        let BDQ = BDN * BDP;
                        let BDT = BDR * BDP;
                        let BDV = UZ - ZY;
                        let BDW = -BDV;
                        let BDX = BDV + BDW;
                        let BDY = if (if BDU < BDX { 1.0 } else { 0.0 }) != 0.0 && (if BDW >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BEX;
                        let BFA;
                        if BDY != 0.0 {
                            let BDZ = BDX - BDU;
                            let BEA = BDZ * BDZ;
                            let BEB = BDW * BDW;
                            let BEC = BEB * BEB;
                            let BED = (BEA * BEA) + BEC;
                            let BET;
                            if BEE != 0.0 {
                                let BEO;
                                if BEF != 0.0 {
                                    BEO = B;
                                } else {
                                    let BEP;
                                    if BEG != 0.0 {
                                        BEP = BF;
                                    } else {
                                        let BEQ;
                                        if BEH != 0.0 {
                                            BEQ = BR;
                                        } else {
                                            let BER = if BEI != 0.0 {
                                                BL
                                            } else {
                                                A
                                            };
                                            BEQ = BER;
                                        }
                                        BEP = BEQ;
                                    }
                                    BEO = BEP;
                                }
                                let mut BEJ = 0.0;
                                let mut BEL = 0.0;
                                BEJ = A;
                                BEL = BED;
                                loop {
                                    let BEK = if BEJ < BEO { 1.0 } else { 0.0 };
                                    if BEK == 0.0 {
                                        break;
                                    }
                                    let BEM = BEL.sqrt();
                                    let BEN = BEJ + B;
                                    BEJ = BEN;
                                    BEL = BEM;
                                }
                                BET = BEL;
                            } else {
                                let BES = BED.powf(2.5e-1f64);
                                BET = BES;
                            }
                            let BEU = B / BET;
                            let BEV = ((BDW * BEC) * BEU) / BED;
                            let BEW = BDX - ((BDZ * BDW) * BEU);
                            BEX = BEV;
                            BFA = BEW;
                        } else {
                            BEX = B;
                            BFA = BDU;
                        }
                        let BEY = BDT * BEX;
                        let BEZ = BDQ * BEX;
                        let BFB = ZY + BFA;
                        let BFD = if (if BFC == B { 1.0 } else { 0.0 }) != 0.0 && (if BAO > BR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BHD;
                        let BHF;
                        let BHG;
                        let BHH;
                        let BHI;
                        let BHL;
                        if BFD != 0.0 {
                            BHD = L;
                            BHF = BAQ;
                            BHG = BBE;
                            BHH = BBK;
                            BHI = BFC;
                            BHL = BAO;
                        } else {
                            let BFG = (BBE - UM) - (SP * ((((BFE + ZY) + BBI) + BFA) + ASJ));
                            let BFH = B - (SP * (BBJ + BEY));
                            let BFI = -SP;
                            let BFJ = BFI * BEZ;
                            let BFM = BFI * BFK;
                            let BFN = BBK - (BBE + (CK * ((I * UZ) + BFE)));
                            let BFP = -(CK * BFK);
                            let BFQ = (BAQ - BBK) - (CQ * BFE);
                            let BFS = B - (CQ * BFK);
                            let BFT = BFH * BFS;
                            let BFU = BFH * BFP;
                            let BFV = BFJ * BFO;
                            let BFW = BFM * BFO;
                            let BFX = -(B / ((((BFT - (BFU * BFR)) - (BFV * BFS)) + (BFW * BFR)) + GD));
                            let BFY = BFX * ((((BFS - (BFP * BFR)) * BFG) + (((BFM * BFR) - (BFJ * BFS)) * BFN)) + (((BFJ * BFP) - BFM) * BFQ));
                            let BFZ = BFX * (((BFS * BFG) + (BFT * BFN)) + ((BFW - BFU) * BFQ));
                            let BGA = BFX * ((BFG + (((-BFH) * BFR) * BFN)) + ((BFH - BFV) * BFQ));
                            let BGB = BFY.abs();
                            let BGC = BFZ.abs();
                            let BGD = if BGB < BGC { 1.0 } else { 0.0 };
                            let BGE = if BGD != 0.0 {
                                BGC
                            } else {
                                BGB
                            };
                            let BGF = BGA.abs();
                            let BGG = if BGE < BGF { 1.0 } else { 0.0 };
                            let BGL = if BGG != 0.0 {
                                BGF
                            } else {
                                BGE
                            };
                            let BGH = if BAO > ARC { 1.0 } else { 0.0 };
                            let BGM;
                            if BGH != 0.0 {
                                BGM = ARE;
                            } else {
                                let BGI = if BAO > ARF { 1.0 } else { 0.0 };
                                let BGN;
                                if BGI != 0.0 {
                                    BGN = ARE;
                                } else {
                                    let BGJ = if BAO > OP { 1.0 } else { 0.0 };
                                    let BGO;
                                    if BGJ != 0.0 {
                                        BGO = ARI;
                                    } else {
                                        let BGK = if BAO > J { 1.0 } else { 0.0 };
                                        let BGP = if BGK != 0.0 {
                                            KY
                                        } else {
                                            B
                                        };
                                        BGO = BGP;
                                    }
                                    BGN = BGO;
                                }
                                BGM = BGN;
                            }
                            let BGQ = BG / BGM;
                            let BGR = if BGL > BGQ { 1.0 } else { 0.0 };
                            let BGW;
                            let BGY;
                            let BHA;
                            if BGR != 0.0 {
                                let BGS = BGQ / BGL;
                                let BGT = BFY * BGS;
                                let BGU = BFZ * BGS;
                                let BGV = BGA * BGS;
                                BGW = BGT;
                                BGY = BGU;
                                BHA = BGV;
                            } else {
                                BGW = BFY;
                                BGY = BFZ;
                                BHA = BGA;
                            }
                            let BGX = BBE + BGW;
                            let BGZ = BBK + BGY;
                            let BHB = BAQ + BHA;
                            let BHC = if BGL < (PH * BGM) { 1.0 } else { 0.0 };
                            let BHJ = if BHC != 0.0 {
                                B
                            } else {
                                BFC
                            };
                            BHD = BAO;
                            BHF = BHB;
                            BHG = BGX;
                            BHH = BGZ;
                            BHI = BHJ;
                            BHL = BHK;
                        }
                        let BHE = BHD + B;
                        BAO = BHE;
                        BAQ = BHF;
                        BBE = BHG;
                        BBK = BHH;
                        BFC = BHI;
                        BHK = BHL;
                        BHV = BBI;
                        BIA = BFB;
                        BID = BFE;
                    }
                    let BHM = if BHK > A { 1.0 } else { 0.0 };
                    if BHM != 0.0 {
                    } else {
                    }
                    let BHN = if BFC == A { 1.0 } else { 0.0 };
                    let BHO;
                    let CVG;
                    if BHN != 0.0 {
                        BHO = BAG;
                        CVG = BAK;
                    } else {
                        BHO = BBE;
                        CVG = BBK;
                    }
                    let CAK = if BAB != 0.0 {
                        B
                    } else {
                        A
                    };
                    let BHP = BHO - ASR;
                    let BHU = BHQ / CI;
                    let BHW = BHV - ASS;
                    let BHX = BHV + ASS;
                    let BHY = BHW - (((LJ * BHX) * BHP) * I);
                    let BHZ = if (if BHY < A { 1.0 } else { 0.0 }) != 0.0 || (if OR == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CBL = if BHZ != 0.0 {
                        A
                    } else {
                        BHY
                    };
                    let BIB = -5e-1f64 * (BIA + ATB);
                    let BIC = BHP + PH;
                    let BIH = UZ * VC;
                    let BII = if BIH >= A { 1.0 } else { 0.0 };
                    let BIJ = if (if (-(((BID * BID) - (ATG * ATG)) / (CP / ((CP * BHU) + B)))) < BIH { 1.0 } else { 0.0 }) != 0.0 && BII != 0.0 { 1.0 } else { 0.0 };
                    if BIJ != 0.0 {
                        if BIK != 0.0 {
                            let BIS;
                            if BIL != 0.0 {
                                BIS = B;
                            } else {
                                let BIT;
                                if BIM != 0.0 {
                                    BIT = BF;
                                } else {
                                    let BIU;
                                    if BIN != 0.0 {
                                        BIU = BR;
                                    } else {
                                        let BIV = if BIO != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        BIU = BIV;
                                    }
                                    BIT = BIU;
                                }
                                BIS = BIT;
                            }
                            let mut BIP = 0.0;
                            BIP = A;
                            loop {
                                let BIQ = if BIP < BIS { 1.0 } else { 0.0 };
                                if BIQ == 0.0 {
                                    break;
                                }
                                let BIR = BIP + B;
                                BIP = BIR;
                            }
                        } else {
                        }
                    } else {
                    }
                    let BIW = if ((LJ * ATK) - B) > A { 1.0 } else { 0.0 };
                    if BIW != 0.0 {
                    } else {
                    }
                    let BIX = -BHW;
                    let BIY = if (if BIX < BIH { 1.0 } else { 0.0 }) != 0.0 && BII != 0.0 { 1.0 } else { 0.0 };
                    let BJU;
                    if BIY != 0.0 {
                        let BIZ = BIH - BIX;
                        let BJA = BIZ * BIZ;
                        let BJB = BIH * BIH;
                        let BJC = (BJA * BJA) + (BJB * BJB);
                        let BJS;
                        if BJD != 0.0 {
                            let BJN;
                            if BJE != 0.0 {
                                BJN = B;
                            } else {
                                let BJO;
                                if BJF != 0.0 {
                                    BJO = BF;
                                } else {
                                    let BJP;
                                    if BJG != 0.0 {
                                        BJP = BR;
                                    } else {
                                        let BJQ = if BJH != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        BJP = BJQ;
                                    }
                                    BJO = BJP;
                                }
                                BJN = BJO;
                            }
                            let mut BJI = 0.0;
                            let mut BJK = 0.0;
                            BJI = A;
                            BJK = BJC;
                            loop {
                                let BJJ = if BJI < BJN { 1.0 } else { 0.0 };
                                if BJJ == 0.0 {
                                    break;
                                }
                                let BJL = BJK.sqrt();
                                let BJM = BJI + B;
                                BJI = BJM;
                                BJK = BJL;
                            }
                            BJS = BJK;
                        } else {
                            let BJR = BJC.powf(2.5e-1f64);
                            BJS = BJR;
                        }
                        let BJT = BIH - ((BIZ * BIH) * (B / BJS));
                        BJU = BJT;
                    } else {
                        BJU = BIX;
                    }
                    let BJV = B - (((B + ((BF * (-BJU)) / (((LJ * TK) * BIC) * BIC))) * BIC) / ASW);
                    let BJW = if (if BJV < 1e-5f64 { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                    let BKR;
                    if BJW != 0.0 {
                        let BJX = 1e-5f64 - BJV;
                        let BJY = BJX * BJX;
                        let BJZ = (BJY * BJY) + 1.0000000000000004e-20f64;
                        let BKP;
                        if BKA != 0.0 {
                            let BKK;
                            if BKB != 0.0 {
                                BKK = B;
                            } else {
                                let BKL;
                                if BKC != 0.0 {
                                    BKL = BF;
                                } else {
                                    let BKM;
                                    if BKD != 0.0 {
                                        BKM = BR;
                                    } else {
                                        let BKN = if BKE != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        BKM = BKN;
                                    }
                                    BKL = BKM;
                                }
                                BKK = BKL;
                            }
                            let mut BKF = 0.0;
                            let mut BKH = 0.0;
                            BKF = A;
                            BKH = BJZ;
                            loop {
                                let BKG = if BKF < BKK { 1.0 } else { 0.0 };
                                if BKG == 0.0 {
                                    break;
                                }
                                let BKI = BKH.sqrt();
                                let BKJ = BKF + B;
                                BKF = BKJ;
                                BKH = BKI;
                            }
                            BKP = BKH;
                        } else {
                            let BKO = BJZ.powf(2.5e-1f64);
                            BKP = BKO;
                        }
                        let BKQ = 1e-5f64 - ((BJX * VC) * (B / BKP));
                        BKR = BKQ;
                    } else {
                        BKR = BJV;
                    }
                    let BKS = B + BKR;
                    let BKT = B + (BKR * BKS);
                    let BKU = if BKS >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let BKW = if BKU != 0.0 {
                        BKS
                    } else {
                        BKV
                    };
                    let BKX = -5e-1f64 * BHX;
                    BKZ = BLA;
                    BLF = BFC;
                    BZY = BKR;
                    CAB = BKW;
                    CAE = BKT;
                    CAJ = CAK;
                    CAQ = BHO;
                    CBK = CBL;
                    CCD = BIB;
                    CCK = BKX;
                    CCT = BID;
                    CCW = BHP;
                    CHW = ASW;
                    CVF = CVG;
                    EDX = A;
                    EGJ = A;
                    EGO = A;
                    EGS = A;
                    EGW = A;
                }
                let BKY = if AZ >= B { 1.0 } else { 0.0 };
                if BKY != 0.0 {
                    let BLD = if (if ASY == B { 1.0 } else { 0.0 }) != 0.0 && (if BKZ == BF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if BLD != 0.0 {
                    } else {
                    }
                    let BLE = if (if ASY == BF { 1.0 } else { 0.0 }) != 0.0 && (if BKZ == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if BLE != 0.0 {
                    } else {
                    }
                } else {
                }
                if ASQ != 0.0 {
                } else {
                }
                let BLG = if BLF == A { 1.0 } else { 0.0 };
                if BLG != 0.0 {
                } else {
                }
                let BLH = if (APX + BLF) < B { 1.0 } else { 0.0 };
                if BLH != 0.0 {
                } else {
                }
                BZV = A;
                BZX = BZY;
                CAA = CAB;
                CAD = CAE;
                CAI = CAJ;
                CAP = CAQ;
                CAT = ASR;
                CAX = ASV;
                CBJ = CBK;
                CCC = CCD;
                CCJ = CCK;
                CCR = ATG;
                CCS = CCT;
                CCV = CCW;
                CEY = ATJ;
                CGD = CGE;
                CGR = CGS;
                CHV = CHW;
                CJH = AAV;
                CJL = VK;
                CJM = ZY;
                CLM = CLN;
                CSI = ASJ;
                CUN = CUO;
                CVE = CVF;
                CVP = CVQ;
                EDW = EDX;
                EGI = EGJ;
                EGN = EGO;
                EGR = EGS;
                EGV = EGW;
                EIJ = A;
                EIU = A;
            } else {
                let BLI = if MZ < H { 1.0 } else { 0.0 };
                let BYD = if BLI != 0.0 {
                    B
                } else {
                    BF
                };
                let BLJ = if OX < (UO + PB) { 1.0 } else { 0.0 };
                let BNM;
                let BRF;
                let BTP;
                let CLO;
                if BLJ != 0.0 {
                    let BLL = (BF * LL) * (((-GH) / UP).ln());
                    let BLM = (B / (LJ * MO)) * TK;
                    let BLN = BF + (4.242640687119285e0f64 * BLM);
                    let BLO = ((BM * BLN) * BLN) * BLN;
                    let BLQ = (BLP * BLM) * ((LJ * (UM - PB)) - BF);
                    let BLR = 9.899494936611664e0f64 - BLQ;
                    let BLS = BLR * BLR;
                    let BLU = if BLO < (BLS * BLT) { 1.0 } else { 0.0 };
                    let BLX = if BLU != 0.0 {
                        let BLV = ((-9.899494936611664e0f64 + BLR) + ((I * BLO) / BLR)) + BLQ;
                        BLV
                    } else {
                        let BLW = (-9.899494936611664e0f64 + ((BLO + BLS).sqrt())) + BLQ;
                        BLW
                    };
                    let BLY = BLX.powf(AAR);
                    let BMA = ((((((-5.65685424949238e0f64 - (BLZ * BLM)) + (BF * BLY)) + ((MN * BLY) * BLY)) * (B / BLY)) * LL) + PB) - PB;
                    let BMB = BMA / BLL;
                    let BMC = (BMA / ((B + (BMB * BMB)).sqrt())) + PB;
                    BNM = BMC;
                    BRF = BLK;
                    BTP = A;
                    CLO = A;
                } else {
                    let BND;
                    let BNF;
                    if BMD != 0.0 {
                        BND = A;
                        BNF = A;
                    } else {
                        let BME = LJ * (UM - PB);
                        let BMF = B + ((BL * (BME - B)) / (UQ * LK));
                        let BMG = if BMF >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let BMI = if BMG != 0.0 {
                            BMF
                        } else {
                            BMH
                        };
                        let BMJ = UM + (((UQ * LJ) * I) * (B - (BMI.sqrt())));
                        let BMK = if (LJ * (BMJ - PB)) < BR { 1.0 } else { 0.0 };
                        let BNA;
                        let BNG;
                        if BMK != 0.0 {
                            let BML = B / ((1.3094570021973102e-2f64 * LJ) * UP);
                            let BMM = AAL + (BR * BML);
                            let BMN = (TW * BML) * BME;
                            let BMO = (AAO - (AAL * (AAP + BML))) + BMN;
                            let BMP = (((-2.916e3f64 - (AAL * BML)) + BMN) + (((((BL * BMM) * BMM) * BMM) + (BMO * BMO)).sqrt())).powf(AAR);
                            let BMQ = (((BR - ((AAT * BMM) / (BR * BMP))) + (2.6456684199469993e-1f64 * BMP)) * LL) + PB;
                            BNA = BMQ;
                            BNG = BMQ;
                        } else {
                            let BMR = if OX <= TP { 1.0 } else { 0.0 };
                            let BNB;
                            if BMR != 0.0 {
                                BNB = BMJ;
                            } else {
                                let BMS = (((((B / MV) / UT) * UM) * UM).ln()) / (LJ + (BF / UM));
                                let BMT = (BMS - BMJ) - VX;
                                let BMU = (BL * BMS) * VX;
                                let BMV = if BMU > A { 1.0 } else { 0.0 };
                                let BMX = if BMV != 0.0 {
                                    BMU
                                } else {
                                    let BMW = -BMU;
                                    BMW
                                };
                                let BMY = BMS - (I * (BMT + (((BMT * BMT) + BMX).sqrt())));
                                BNB = BMY;
                            }
                            BNA = BNB;
                            BNG = BMJ;
                        }
                        let BMZ = PB + 2.5e-12f64;
                        let BNC = if BNA < BMZ { 1.0 } else { 0.0 };
                        let BNE = if BNC != 0.0 {
                            BMZ
                        } else {
                            BNA
                        };
                        BND = BNE;
                        BNF = BNG;
                    }
                    BNM = BND;
                    BRF = A;
                    BTP = BNF;
                    CLO = BND;
                }
                let BNH = if (if AFT == B { 1.0 } else { 0.0 }) != 0.0 && (if AKS == BF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BNJ = if BNH != 0.0 {
                    let BNI = 1e-5f64 * ALB;
                    BNI
                } else {
                    A
                };
                let BNK = (LJ * PB).exp();
                let BNL = MV * BNK;
                let BNN = (((ID * H) * H) / BF) / CI;
                let BNO = ((BF * LJ) * BNN).sqrt();
                let BNP = ((((BNO.exp()) + ((-BNO).exp())) / BF).ln()) / BNN;
                let mut BNQ = 0.0;
                let mut BNS = 0.0;
                let mut BPA = 0.0;
                let mut BPG = 0.0;
                let mut BRG = 0.0;
                let mut BRK = 0.0;
                let mut BRL = 0.0;
                let mut BYC = 0.0;
                BNQ = B;
                BNS = BNM;
                BPA = A;
                BPG = BRF;
                BRG = A;
                BRK = A;
                BRL = A;
                BYC = BYD;
                loop {
                    let BNR = if BNQ <= 2.01e2f64 { 1.0 } else { 0.0 };
                    if BNR == 0.0 {
                        break;
                    }
                    let BNT = BNS - PB;
                    let BNU = LJ * BNT;
                    let BNV = BNT - BNN;
                    let BNW = BNP * BNV;
                    let BNX = if BNW < ARC { 1.0 } else { 0.0 };
                    let BOC;
                    let BOH;
                    if BNX != 0.0 {
                        let BNY = BNW.exp();
                        let BNZ = B + (BNY - (((-BNP) * BNN).exp()));
                        let BOA = (BNZ.ln()) / BNP;
                        let BOB = BNY / BNZ;
                        BOC = BOA;
                        BOH = BOB;
                    } else {
                        BOC = BNV;
                        BOH = B;
                    }
                    let BOD = LJ * BOC;
                    let BOE = BNU.abs();
                    let BOG = if BOE < BOF { 1.0 } else { 0.0 };
                    let BPI;
                    let BPQ;
                    if BOG != 0.0 {
                        let BOI = ((B - (BOH * BOH)) / BF).sqrt();
                        let BOJ = BNU * BOI;
                        let BOK = LJ * BOI;
                        let BOL = if BNU < A { 1.0 } else { 0.0 };
                        let BPJ;
                        let BPR;
                        if BOL != 0.0 {
                            let BOM = -BOJ;
                            let BON = -BOK;
                            BPJ = BOM;
                            BPR = BON;
                        } else {
                            BPJ = BOJ;
                            BPR = BOK;
                        }
                        BPI = BPJ;
                        BPQ = BPR;
                    } else {
                        let BOP = if BOE < BOO { 1.0 } else { 0.0 };
                        let BPK;
                        let BPS;
                        if BOP != 0.0 {
                            let BOQ = BNU / BR;
                            let BOR = BNU / BL;
                            let BOS = BOD / BR;
                            let BOT = BOD / BL;
                            let BOU = ((((BNU * BNU) / BF) * (B - (BOQ * (B - (BOR * (B - (BNU / KY))))))) - (((BOD * BOD) / BF) * (B - (BOS * (B - (BOT * (B - (BOD / KY)))))))).sqrt();
                            let BOV = ((LJ * I) * ((BNU * (B - ((BNU / BF) * (B - (BOQ * (B - BOR)))))) - (BOH * (BOD * (B - ((BOD / BF) * (B - (BOS * (B - BOT))))))))) / BOU;
                            BPK = BOU;
                            BPS = BOV;
                        } else {
                            let BOW = (-BNU).exp();
                            let BOX = (-BOD).exp();
                            let BOY = ((BNU - BOD) + (BOW - BOX)).sqrt();
                            let BOZ = ((LJ * I) * ((B - BOW) - (BOH * (B - BOX)))) / BOY;
                            BPK = BOY;
                            BPS = BOZ;
                        }
                        BPI = BPK;
                        BPQ = BPS;
                    }
                    let BPB = if BPA == B { 1.0 } else { 0.0 };
                    let BPC = if BNU < A { 1.0 } else { 0.0 };
                    let BPD = if BPB != 0.0 && BPC != 0.0 { 1.0 } else { 0.0 };
                    let BPF = if BPD != 0.0 {
                        BPE
                    } else {
                        BPG
                    };
                    let BPH = if BPF == -1e0f64 { 1.0 } else { 0.0 };
                    let BPM = if BPH != 0.0 {
                        A
                    } else {
                        let BPL = MW * BPI;
                        BPL
                    };
                    let BPN = if BPM < (H * 1.01e0f64) { 1.0 } else { 0.0 };
                    let BYE = if BPN != 0.0 {
                        B
                    } else {
                        BF
                    };
                    let BPO = ID * BPM;
                    let BQG;
                    let BQJ;
                    let BRM;
                    if BPC != 0.0 {
                        let BPP = -BPI;
                        let BPT = -BPQ;
                        BQG = BPP;
                        BQJ = BPT;
                        BRM = BRL;
                    } else {
                        let BPU = if BNU < CF { 1.0 } else { 0.0 };
                        let BQH;
                        let BQK;
                        let BRN;
                        if BPU != 0.0 {
                            BQH = BPI;
                            BQK = BPQ;
                            BRN = BRL;
                        } else {
                            let BPV = if BNU < ARC { 1.0 } else { 0.0 };
                            let BQC;
                            let BQE;
                            if BPV != 0.0 {
                                let BPW = BNU.exp();
                                let BPX = BNL * (BPW - (BNU + B));
                                let BPY = (BNL * LJ) * (BPW - B);
                                BQC = BPX;
                                BQE = BPY;
                            } else {
                                let BPZ = (LJ * BNS).exp();
                                let BQA = MV * (BPZ - (BNK * (BNU + B)));
                                let BQB = (MV * LJ) * (BPZ - BNK);
                                BQC = BQA;
                                BQE = BQB;
                            }
                            let BQD = ((BPI * BPI) + BQC).sqrt();
                            let BQF = (I * (((BF * BPQ) * BPI) + BQE)) / BQD;
                            BQH = BQD;
                            BQK = BQF;
                            BRN = BQC;
                        }
                        BQG = BQH;
                        BQJ = BQK;
                        BRM = BRN;
                    }
                    let BQI = (((-UM) + BNS) + (UP * BQG)) - (SP * BNJ);
                    let BQL = B + (UP * BQJ);
                    let BRA;
                    let BRC;
                    let BRD;
                    if BPB != 0.0 {
                        BRA = BQM;
                        BRC = BNS;
                        BRD = BPA;
                    } else {
                        let BQN = (-BQI) / BQL;
                        let BQO = BNS.abs();
                        let BQP = if B >= BQO { 1.0 } else { 0.0 };
                        let BQQ = if BQP != 0.0 {
                            B
                        } else {
                            BQO
                        };
                        let BQR = 5e-2f64 * (B + BQQ);
                        let BQS = if (BQN.abs()) > BQR { 1.0 } else { 0.0 };
                        let BQX;
                        if BQS != 0.0 {
                            let BQT = if BQN >= A { 1.0 } else { 0.0 };
                            let BQV = if BQT != 0.0 {
                                B
                            } else {
                                BQU
                            };
                            let BQW = BQR * BQV;
                            BQX = BQW;
                        } else {
                            BQX = BQN;
                        }
                        let BQY = BNS + BQX;
                        let BQZ = if (if (BQX.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (BQI.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BRE = if BQZ != 0.0 {
                            B
                        } else {
                            BPA
                        };
                        BRA = BNQ;
                        BRC = BQY;
                        BRD = BRE;
                    }
                    let BRB = BRA + B;
                    BNQ = BRB;
                    BNS = BRC;
                    BPA = BRD;
                    BPG = BPF;
                    BRG = BPO;
                    BRK = BQG;
                    BRL = BRM;
                    BYC = BYE;
                }
                let BRH = BRG / MO;
                let BRI = (BRH * BRH) + 2.220446049250313e-15f64;
                let BRJ = BRH + 2.220446049250313e-15f64;
                let BRO = (MO * BRL) * (B / (BRK + BRJ));
                let BRP = -BRO;
                let BRQ = BRO * SP;
                let BRR = if (if BPG == -1e0f64 { 1.0 } else { 0.0 }) != 0.0 || (if BRQ <= G { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BSA;
                let BXI;
                let BYP;
                let CAL;
                let CAS;
                let CCH;
                let EDY;
                let EGK;
                let EIK;
                let EIV;
                if BRR != 0.0 {
                    let BRS = TK * (UM - BNS);
                    let BRT = ((-DR) * CV) * BRS;
                    let BRX = (-BRU) * BRS;
                    let BRY = BRX * I;
                    let BRZ = BRX - BRY;
                    BSA = B;
                    BXI = BL;
                    BYP = A;
                    CAL = B;
                    CAS = BNS;
                    CCH = BRS;
                    EDY = BNS;
                    EGK = BRT;
                    EIK = BRZ;
                    EIV = BRY;
                } else {
                    BSA = A;
                    BXI = BPG;
                    BYP = BRQ;
                    CAL = A;
                    CAS = A;
                    CCH = A;
                    EDY = A;
                    EGK = A;
                    EIK = A;
                    EIV = A;
                }
                let BSB = if BSA == A { 1.0 } else { 0.0 };
                let BZZ;
                let CAC;
                let CAF;
                let CAR;
                let CBM;
                let CCE;
                let CCL;
                let CCX;
                if BSB != 0.0 {
                    let BSC = IE / (TK * TK);
                    let BSD = BF / BSC;
                    let BSE = B + (BSD * (UM - GD));
                    let BSF = B + BSD;
                    let BSG = if (if BSE < BSF { 1.0 } else { 0.0 }) != 0.0 && (if BSF >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BTC;
                    if BSG != 0.0 {
                        let BSH = BSF - BSE;
                        let BSI = BSH * BSH;
                        let BSJ = BSF * BSF;
                        let BSK = (((BSI * BSI) * BSI) * BSI) + (((BSJ * BSJ) * BSJ) * BSJ);
                        let BTA;
                        if BSL != 0.0 {
                            let BSV;
                            if BSM != 0.0 {
                                BSV = B;
                            } else {
                                let BSW;
                                if BSN != 0.0 {
                                    BSW = BF;
                                } else {
                                    let BSX;
                                    if BSO != 0.0 {
                                        BSX = BR;
                                    } else {
                                        let BSY = if BSP != 0.0 {
                                            BL
                                        } else {
                                            A
                                        };
                                        BSX = BSY;
                                    }
                                    BSW = BSX;
                                }
                                BSV = BSW;
                            }
                            let mut BSQ = 0.0;
                            let mut BSS = 0.0;
                            BSQ = A;
                            BSS = BSK;
                            loop {
                                let BSR = if BSQ < BSV { 1.0 } else { 0.0 };
                                if BSR == 0.0 {
                                    break;
                                }
                                let BST = BSS.sqrt();
                                let BSU = BSQ + B;
                                BSQ = BSU;
                                BSS = BST;
                            }
                            BTA = BSS;
                        } else {
                            let BSZ = BSK.powf(1.25e-1f64);
                            BTA = BSZ;
                        }
                        let BTB = BSF - ((BSH * BSF) * (B / BTA));
                        BTC = BTB;
                    } else {
                        BTC = BSE;
                    }
                    let BTD = UM + (BSC * (B - (BTC.sqrt())));
                    let BTE = (I * (BTD + (((BTD * BTD) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let BTF = if BTE < A { 1.0 } else { 0.0 };
                    let BTG = if BTF != 0.0 {
                        A
                    } else {
                        BTE
                    };
                    let BTH = OR / BTG;
                    let BTI = B + ((BTH.powf((AUR - B))) * BTH);
                    let BTJ = OR / ((BTI.powf(((B / AUR) - B))) * BTI);
                    let BTK = (LJ * (PB - BTJ)).exp();
                    let BTL = if BTJ <= A { 1.0 } else { 0.0 };
                    let BUI;
                    if BTL != 0.0 {
                        BUI = BNS;
                    } else {
                        let BUC = if BTM != 0.0 {
                            let BTN = A - BNS;
                            BTN
                        } else {
                            A
                        };
                        let BUB;
                        if BTO != 0.0 {
                            let BTQ = BTP - BNS;
                            let BTR = if BTQ >= A { 1.0 } else { 0.0 };
                            let BTS = if BTR != 0.0 {
                                BTQ
                            } else {
                                A
                            };
                            let BTT = ((1.3e0f64 * BTS) - BTJ) - AGW;
                            let BTU = (BL * (1.3e0f64 * BTS)) * AGW;
                            let BTV = if BTU > A { 1.0 } else { 0.0 };
                            let BTX = if BTV != 0.0 {
                                BTU
                            } else {
                                let BTW = -BTU;
                                BTW
                            };
                            let BTY = (1.3e0f64 * BTS) - (I * (BTT + (((BTT * BTT) + BTX).sqrt())));
                            let BTZ = if BTY <= BTS { 1.0 } else { 0.0 };
                            let BUA = if BTZ != 0.0 {
                                BTY
                            } else {
                                BTS
                            };
                            BUB = BUA;
                        } else {
                            BUB = BUC;
                        }
                        let BUD = if BUB < A { 1.0 } else { 0.0 };
                        let BUF;
                        if BUD != 0.0 {
                            BUF = A;
                        } else {
                            let BUE = if BUB > BTJ { 1.0 } else { 0.0 };
                            let BUG = if BUE != 0.0 {
                                BTJ
                            } else {
                                BUB
                            };
                            BUF = BUG;
                        }
                        let BUH = BNS + BUF;
                        BUI = BUH;
                    }
                    let mut BUJ = 0.0;
                    let mut BUL = 0.0;
                    let mut BWP = 0.0;
                    let mut BXL = 0.0;
                    let mut BXN = 0.0;
                    let mut BXO = 0.0;
                    BUJ = B;
                    BUL = BUI;
                    BWP = A;
                    BXL = BRG;
                    BXN = A;
                    BXO = A;
                    loop {
                        let BUK = if BUJ <= 2.01e2f64 { 1.0 } else { 0.0 };
                        if BUK == 0.0 {
                            break;
                        }
                        let BUM = BUL - PB;
                        let BUN = LJ * BUM;
                        let BUO = BUM - BNN;
                        let BUP = BNP * BUO;
                        let BUQ = if BUP < ARC { 1.0 } else { 0.0 };
                        let BUV;
                        let BUZ;
                        if BUQ != 0.0 {
                            let BUR = BUP.exp();
                            let BUS = B + (BUR - (((-BNP) * BNN).exp()));
                            let BUT = (BUS.ln()) / BNP;
                            let BUU = BUR / BUS;
                            BUV = BUT;
                            BUZ = BUU;
                        } else {
                            BUV = BUO;
                            BUZ = B;
                        }
                        let BUW = LJ * BUV;
                        let BUX = BUN.abs();
                        let BUY = if BUX < BOF { 1.0 } else { 0.0 };
                        let BVS;
                        let BWA;
                        if BUY != 0.0 {
                            let BVA = ((B - (BUZ * BUZ)) / BF).sqrt();
                            let BVB = BUN * BVA;
                            let BVC = LJ * BVA;
                            let BVD = if BUN < A { 1.0 } else { 0.0 };
                            let BVT;
                            let BWB;
                            if BVD != 0.0 {
                                let BVE = -BVB;
                                let BVF = -BVC;
                                BVT = BVE;
                                BWB = BVF;
                            } else {
                                BVT = BVB;
                                BWB = BVC;
                            }
                            BVS = BVT;
                            BWA = BWB;
                        } else {
                            let BVG = if BUX < BOO { 1.0 } else { 0.0 };
                            let BVU;
                            let BWC;
                            if BVG != 0.0 {
                                let BVH = BUN / BR;
                                let BVI = BUN / BL;
                                let BVJ = BUW / BR;
                                let BVK = BUW / BL;
                                let BVL = ((((BUN * BUN) / BF) * (B - (BVH * (B - (BVI * (B - (BUN / KY))))))) - (((BUW * BUW) / BF) * (B - (BVJ * (B - (BVK * (B - (BUW / KY)))))))).sqrt();
                                let BVM = ((LJ * I) * ((BUN * (B - ((BUN / BF) * (B - (BVH * (B - BVI)))))) - (BUZ * (BUW * (B - ((BUW / BF) * (B - (BVJ * (B - BVK))))))))) / BVL;
                                BVU = BVL;
                                BWC = BVM;
                            } else {
                                let BVN = (-BUN).exp();
                                let BVO = (-BUW).exp();
                                let BVP = ((BUN - BUW) + (BVN - BVO)).sqrt();
                                let BVQ = ((LJ * I) * ((B - BVN) - (BUZ * (B - BVO)))) / BVP;
                                BVU = BVP;
                                BWC = BVQ;
                            }
                            BVS = BVU;
                            BWA = BWC;
                        }
                        let BVR = if BXI == -1e0f64 { 1.0 } else { 0.0 };
                        let BVW = if BVR != 0.0 {
                            A
                        } else {
                            let BVV = MW * BVS;
                            BVV
                        };
                        let BVX = ID * BVW;
                        let BVY = if BUN < A { 1.0 } else { 0.0 };
                        let BWJ;
                        let BWM;
                        let BXP;
                        if BVY != 0.0 {
                            let BVZ = -BVS;
                            let BWD = -BWA;
                            BWJ = BVZ;
                            BWM = BWD;
                            BXP = BXO;
                        } else {
                            let BWE = if BUN < CF { 1.0 } else { 0.0 };
                            let BWK;
                            let BWN;
                            let BXQ;
                            if BWE != 0.0 {
                                BWK = BVS;
                                BWN = BWA;
                                BXQ = BXO;
                            } else {
                                let BWF = (LJ * (BUL - BTJ)).exp();
                                let BWG = MV * (BWF - (BTK * (BUN + B)));
                                let BWH = ((BVS * BVS) + BWG).sqrt();
                                let BWI = (I * (((BF * BWA) * BVS) + ((MV * LJ) * (BWF - BTK)))) / BWH;
                                BWK = BWH;
                                BWN = BWI;
                                BXQ = BWG;
                            }
                            BWJ = BWK;
                            BWM = BWN;
                            BXP = BXQ;
                        }
                        let BWL = (((-UM) + BUL) + (UP * BWJ)) - (SP * BNJ);
                        let BWO = B + (UP * BWM);
                        let BWQ = if (if BWP == B { 1.0 } else { 0.0 }) != 0.0 && (if BUJ > BR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BXF;
                        let BXH;
                        let BXJ;
                        if BWQ != 0.0 {
                            BXF = BWR;
                            BXH = BUL;
                            BXJ = BWP;
                        } else {
                            let BWS = (-BWL) / BWO;
                            let BWT = BUL.abs();
                            let BWU = if B >= BWT { 1.0 } else { 0.0 };
                            let BWV = if BWU != 0.0 {
                                B
                            } else {
                                BWT
                            };
                            let BWW = 5e-2f64 * (B + BWV);
                            let BWX = if (BWS.abs()) > BWW { 1.0 } else { 0.0 };
                            let BXC;
                            if BWX != 0.0 {
                                let BWY = if BWS >= A { 1.0 } else { 0.0 };
                                let BXA = if BWY != 0.0 {
                                    B
                                } else {
                                    BWZ
                                };
                                let BXB = BWW * BXA;
                                BXC = BXB;
                            } else {
                                BXC = BWS;
                            }
                            let BXD = BUL + BXC;
                            let BXE = if (if (BXC.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (BWL.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let BXK = if BXE != 0.0 {
                                B
                            } else {
                                BWP
                            };
                            BXF = BUJ;
                            BXH = BXD;
                            BXJ = BXK;
                        }
                        let BXG = BXF + B;
                        BUJ = BXG;
                        BUL = BXH;
                        BWP = BXJ;
                        BXL = BVX;
                        BXN = BWJ;
                        BXO = BXP;
                    }
                    let BXM = BXL / MO;
                    let BXR = -((MO * BXO) * (B / (BXN + (BXM + 2.220446049250313e-15f64))));
                    let BXS = BUL - BNS;
                    let BXT = I * (BRH + BXM);
                    let BXU = ((LJ * TK) * ((UM + LL) - (I * ((BF * BNS) + BXS)))) + ((LJ * MO) * ((-BXT) + ((B / (((((LJ / BRI) * BXS) + B).sqrt()) + B)) / BRJ)));
                    let BXV = BXL + BRG;
                    let BXW = BXV / BF;
                    let BXX = BXR + BRP;
                    let BXY = (-BXX) / BF;
                    let BXZ = BXL - BRG;
                    let BYA = -(BXR - BRP);
                    let BYB = MO * MO;
                    let BYF = if BYC <= B { 1.0 } else { 0.0 };
                    let BYI = if BYF != 0.0 {
                        let BYG = (((BXY * LJ) * BXS) - BYA) - ((((BXZ * BXZ) * BXZ) / BYB) / LA);
                        BYG
                    } else {
                        let BYH = BXS * BXU;
                        BYH
                    };
                    let BYJ = if (if AZ >= B { 1.0 } else { 0.0 }) != 0.0 && (if BYI < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BYM = if BYJ != 0.0 {
                        A
                    } else {
                        BYI
                    };
                    let CCF;
                    if BYF != 0.0 {
                        let BYK = if (BXS.abs()) > O { 1.0 } else { 0.0 };
                        let CCG = if BYK != 0.0 {
                            let BYL = BF * BXW;
                            let BYN = ((BXW * (((BXY * LJ) * BXS) - BYA)) + (((((((BXY - BYL) + ((TK / LJ) * ((B - ((BYL * BXW) / BYB)) + (((BXZ * BXZ) / BYB) / J)))) * BXZ) * BXZ) * BXZ) / BYB) / LA)) / BYM;
                            BYN
                        } else {
                            BXW
                        };
                        CCF = CCG;
                    } else {
                        let BYO = I * BXV;
                        CCF = BYO;
                    }
                    let BYQ = B - (B - ((BXS + ((BF * UP) * (BXT - BRJ))) * (B / BYP)));
                    let BYR = BYQ * BYQ;
                    let BYS = (((BYR * BYR) * BYR) * BYR) + 1e0f64;
                    let BZI;
                    if BYT != 0.0 {
                        let BZD;
                        if BYU != 0.0 {
                            BZD = B;
                        } else {
                            let BZE;
                            if BYV != 0.0 {
                                BZE = BF;
                            } else {
                                let BZF;
                                if BYW != 0.0 {
                                    BZF = BR;
                                } else {
                                    let BZG = if BYX != 0.0 {
                                        BL
                                    } else {
                                        A
                                    };
                                    BZF = BZG;
                                }
                                BZE = BZF;
                            }
                            BZD = BZE;
                        }
                        let mut BYY = 0.0;
                        let mut BZA = 0.0;
                        BYY = A;
                        BZA = BYS;
                        loop {
                            let BYZ = if BYY < BZD { 1.0 } else { 0.0 };
                            if BYZ == 0.0 {
                                break;
                            }
                            let BZB = BZA.sqrt();
                            let BZC = BYY + B;
                            BYY = BZC;
                            BZA = BZB;
                        }
                        BZI = BZA;
                    } else {
                        let BZH = BYS.powf(1.25e-1f64);
                        BZI = BZH;
                    }
                    let BZJ = B - (BYQ * (B / BZI));
                    let BZK = B + BZJ;
                    let BZL = B + (BZJ * BZK);
                    let BZM = if BZK >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let BZO = if BZM != 0.0 {
                        BZK
                    } else {
                        BZN
                    };
                    let CCM;
                    if BYF != 0.0 {
                        let BZP = if (BXS.abs()) > O { 1.0 } else { 0.0 };
                        let CCN = if BZP != 0.0 {
                            let BZQ = ((((((BXY * BXY) + ((BYA * BYA) / BLZ)) * LJ) * BXS) - (BXY * BYA)) - (((((((BF * BXY) + (((((TK / LJ) * BXZ) * BXZ) / BYB) / KY)) * BXZ) * BXZ) * BXZ) / BYB) / LA)) / BYM;
                            BZQ
                        } else {
                            BXY
                        };
                        CCM = CCN;
                    } else {
                        let BZR = -5e-1f64 * BXX;
                        CCM = BZR;
                    }
                    let BZS = if BPA == A { 1.0 } else { 0.0 };
                    if BZS != 0.0 {
                    } else {
                    }
                    let BZT = if BWP == A { 1.0 } else { 0.0 };
                    if BZT != 0.0 {
                    } else {
                    }
                    let BZU = if (BPA + BWP) < B { 1.0 } else { 0.0 };
                    if BZU != 0.0 {
                    } else {
                    }
                    BZZ = BZJ;
                    CAC = BZO;
                    CAF = BZL;
                    CAR = BUL;
                    CBM = BYM;
                    CCE = CCF;
                    CCL = CCM;
                    CCX = BXS;
                } else {
                    BZZ = A;
                    CAC = A;
                    CAF = A;
                    CAR = CAS;
                    CBM = A;
                    CCE = CCH;
                    CCL = A;
                    CCX = A;
                }
                BZV = BSA;
                BZX = BZZ;
                CAA = CAC;
                CAD = CAF;
                CAI = CAL;
                CAP = CAR;
                CAT = BNS;
                CAX = BRO;
                CBJ = CBM;
                CCC = CCE;
                CCJ = CCL;
                CCR = A;
                CCS = A;
                CCV = CCX;
                CEY = A;
                CGD = MJ;
                CGR = MG;
                CHV = BYP;
                CJH = A;
                CJL = A;
                CJM = A;
                CLM = CLO;
                CSI = BNJ;
                CUN = A;
                CVE = A;
                CVP = A;
                EDW = EDY;
                EGI = EGK;
                EGN = A;
                EGR = A;
                EGV = A;
                EIJ = EIK;
                EIU = EIV;
            }
            let BZW = if BZV == A { 1.0 } else { 0.0 };
            let CIH;
            let CSN;
            let CVO;
            let CVW;
            let ECU;
            let EDC;
            let EDD;
            let EDS;
            let EDZ;
            let EEM;
            let EEQ;
            let EEU;
            let EFB;
            let EGH;
            let EGL;
            let EGP;
            let EGT;
            if BZW != 0.0 {
                let CAG = if (ADE - ((LY * (I + BZX)) / (CAA * CAD))) > 5.0000001e-1f64 { 1.0 } else { 0.0 };
                if CAG != 0.0 {
                    let CAH = if AZ >= B { 1.0 } else { 0.0 };
                    if CAH != 0.0 {
                    } else {
                    }
                } else {
                }
                let CAM = if CAI == A { 1.0 } else { 0.0 };
                let CBX;
                let EDT;
                if CAM != 0.0 {
                    let CAO = if (if BC < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if CAN < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CBV;
                    let EDU;
                    if CAO != 0.0 {
                        let CAU = CAT + PL;
                        let CAV = if CAP > (CAU - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                        let EDV = if CAV != 0.0 {
                            let CAW = CAU - 2.220446049250313e-15f64;
                            CAW
                        } else {
                            CAP
                        };
                        CBV = A;
                        EDU = EDV;
                    } else {
                        if JB != 0.0 {
                        } else {
                        }
                        let CAZ = CI * (B / ((CAY * ID) + (CAN * (CAX * (B / H)))));
                        let CBB = (CBA * (OR + CAT)) + ((B - CBA) * CAP);
                        let CBC = CAT + PL;
                        let CBD = if CBB > (CBC - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                        let CBF = if CBD != 0.0 {
                            let CBE = CBC - 2.220446049250313e-15f64;
                            CBE
                        } else {
                            CBB
                        };
                        let CBG = CBF - CAP;
                        let CBH = (I * (CBG + (((CBG * CBG) + 4e-6f64).sqrt()))) + 1e-13f64;
                        let CBI = if CBH < A { 1.0 } else { 0.0 };
                        let CBR = if CBI != 0.0 {
                            A
                        } else {
                            CBH
                        };
                        let CBN = CBJ * (B / (LJ * CAX));
                        let CBO = if CBN < LL { 1.0 } else { 0.0 };
                        let CBQ = if CBO != 0.0 {
                            LL
                        } else {
                            CBN
                        };
                        let CBS = (BF * (ID / CI)) * CBR;
                        let CBT = ((((BF * CBQ) + (CBS * CAZ)) + (CBP * CAZ)) * (B / CU)) * CAZ;
                        let CBU = QH * (I * ((-CBT) + (((CBT * CBT) + (((BL * (CBS + CBP)) * CAZ) * CAZ)).sqrt())));
                        CBV = CBU;
                        EDU = CBF;
                    }
                    let CBW = CBV * EU;
                    CBX = CBW;
                    EDT = EDU;
                } else {
                    CBX = A;
                    EDT = EDW;
                }
                let CBY = CU - CBX;
                let CBZ = CV - CBX;
                let CCA = if CBY < KK { 1.0 } else { 0.0 };
                let CDN = if CCA != 0.0 {
                    KK
                } else {
                    CBY
                };
                let CCB = (-DR) * CV;
                let CCI = CCB * CCC;
                let CCO = CCB * CCJ;
                let EGM;
                let EGQ;
                let EGU;
                if F != 0.0 {
                    let CCP = CCI * I;
                    let CCQ = CCI * 5e-1f64;
                    let CCU = ((I * (CCR + CCS)) * CV) * DR;
                    EGM = CCU;
                    EGQ = CCP;
                    EGU = CCQ;
                } else {
                    EGM = EGN;
                    EGQ = EGR;
                    EGU = EGV;
                }
                let CCY = OR - CCV;
                let CDA = (BF * (CCY / BF)) / CCZ;
                let CDB = CCZ / (B + (CDA * (5e-1f64 + (CDA * (1.6666666666666666e-1f64 + (CDA * (4.1666666666666664e-2f64 + (CDA * (8.333333333333333e-3f64 + (CDA * (1.388888888888889e-3f64 + (CDA * 1.984126984126984e-4f64))))))))))));
                let CDC = if CDB < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let CDE = if CDC != 0.0 {
                    CDD
                } else {
                    CDB
                };
                let CDF = CAT + CDE;
                let CDH = CCJ / IZ;
                let CDI = (((parameters[92] / CDG) * (CCC / IZ)) + ((parameters[93] / CDG) * CDH)) / (B + ((CAP - CAT) * parameters[94]));
                let CDJ = (I * (CDI + (((CDI * CDI) + 3.6e7f64).sqrt()))) + 3e-7f64;
                let CDK = if CDJ < A { 1.0 } else { 0.0 };
                let CDL = if CDK != 0.0 {
                    A
                } else {
                    CDJ
                };
                let CDM = (B / (((B / (parameters[95] + ((parameters[96] * (CDH / ED)) / 1e11f64))) + (LU * ((CDL.powf((parameters[97] - B))) * CDL))) + (((CDL.powf((DU - B))) * CDL) / parameters[106]))) * T;
                let CDO = (LJ * CAX) * CDN;
                let CDP = (I * (CDO + (((CDO * CDO) + 4e-100f64).sqrt()))) + 1.0000000000000001e-60f64;
                let CDQ = if CDP < A { 1.0 } else { 0.0 };
                let CDR = if CDQ != 0.0 {
                    A
                } else {
                    CDP
                };
                let CDS = CBJ * (B / CDR);
                let CDT = (AFV * MB) / CDM;
                let CDU = ((CDS * CDS) + (CDT * CDT)).sqrt();
                let CDV = (CDM * CDU) / MB;
                let CDX = if (if 9.999999999999978e-1f64 <= CDW { 1.0 } else { 0.0 }) != 0.0 && (if CDW <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CEA;
                if CDX != 0.0 {
                    CEA = B;
                } else {
                    let CDY = if (if 1.9999999999999978e0f64 <= CDW { 1.0 } else { 0.0 }) != 0.0 && (if CDW <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CEB = if CDY != 0.0 {
                        CDV
                    } else {
                        let CDZ = CDV.powf((CDW - B));
                        CDZ
                    };
                    CEA = CEB;
                }
                let CEC = B + (CDV * CEA);
                let CED = if (if 9.999999999999978e-1f64 <= CDW { 1.0 } else { 0.0 }) != 0.0 && (if CDW <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CEI;
                if CED != 0.0 {
                    let CEE = B / CEC;
                    CEI = CEE;
                } else {
                    let CEF = if (if 1.9999999999999978e0f64 <= CDW { 1.0 } else { 0.0 }) != 0.0 && (if CDW <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CEJ = if CEF != 0.0 {
                        let CEG = B / (CEC.sqrt());
                        CEG
                    } else {
                        let CEH = CEC * (CEC.powf(((-1e0f64 / CDW) - B)));
                        CEH
                    };
                    CEI = CEJ;
                }
                let CEK = CDM * CEI;
                let CEL = (DP * LL) / CBY;
                let CEM = (CEL * CBJ) * CEK;
                let CEO = if (if CEN > A { 1.0 } else { 0.0 }) != 0.0 && (if EH != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CFE;
                if CEO != 0.0 {
                    let CEP = (BF * (I * CCY)) / M;
                    let CEQ = CAT + (M / (B + (CEP * (5e-1f64 + (CEP * (1.6666666666666666e-1f64 + (CEP * (4.1666666666666664e-2f64 + (CEP * (8.333333333333333e-3f64 + (CEP * (1.388888888888889e-3f64 + (CEP * 1.984126984126984e-4f64)))))))))))));
                    let CER = 1.1e0f64 - CEQ;
                    let CES = (I * (CER + (((CER * CER) + 1.0000000000000002e-2f64).sqrt()))) + 5.0000000000000005e-12f64;
                    let CET = if CES < A { 1.0 } else { 0.0 };
                    let CEU = if CET != 0.0 {
                        A
                    } else {
                        CES
                    };
                    let CEV = (TK * (LJ * EI)) * (CEU.powf(parameters[245]));
                    let CEW = B + (PL * parameters[246]);
                    let CFA = if RO != 0.0 {
                        let CEX = CEQ - PK;
                        CEX
                    } else {
                        let CEZ = CEQ - CEY;
                        CEZ
                    };
                    let CFB = CEV * (CEW + ((PL * EJ) * CFA));
                    CFE = CFB;
                } else {
                    CFE = A;
                }
                let CFC = if EK != A { 1.0 } else { 0.0 };
                let CFF = if CFC != 0.0 {
                    let CFD = (TK * (LJ * EL)) * PL;
                    CFD
                } else {
                    A
                };
                let CFG = CFE + CFF;
                let CFH = if CFG > A { 1.0 } else { 0.0 };
                let CFJ = if CFH != 0.0 {
                    let CFI = (CEL * (CCV * CFG)) * CEK;
                    CFI
                } else {
                    A
                };
                let CFK = CEM + CFJ;
                let CFL = if parameters[33] != A { 1.0 } else { 0.0 };
                let CII;
                if CFL != 0.0 {
                    let CFM = ER - TE;
                    let CFN = (((((BF * TD) * (CI * SP)) * IK) * (B / (CFM * CFM))) * SU) * (parameters[154] + (parameters[155] * PL));
                    let CFP = ((PM - EQ) + (CFO - (parameters[157] * OR))) + CFN;
                    let CFQ = (MH * SP) * SP;
                    let CFR = (CFQ * LJ) * I;
                    let CFS = (CFR * LJ) * BF;
                    let CFT = ((((LL - (CFQ * (LJ * AHV))) + EQ) - CFO) - CFN) + GD;
                    let CFU = (PM - CFT) - BOO;
                    let CFV = if CFT >= A { 1.0 } else { 0.0 };
                    let CFX = if CFV != 0.0 {
                        B
                    } else {
                        CFW
                    };
                    let CFY = B + (((LJ * (((((CFT + (I * (CFU + (((CFU * CFU) + (((CFX * BL) * CFT) * BOO)).sqrt())))) - EQ) + CFO) + CFN) - RP)) - B) * (BL / CFS));
                    let CFZ = (I * (CFY + (((CFY * CFY) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CGA = if CFZ < A { 1.0 } else { 0.0 };
                    let CGB = if CGA != 0.0 {
                        A
                    } else {
                        CFZ
                    };
                    let CGC = CFP + (CFR * (B - ((CGB + GD).sqrt())));
                    let CGF = ((((B / CGD) / CFQ) * (CFP * CFP)).ln()) * (B / (LJ + (BF / (CFP + GD))));
                    let CGG = (CGF - CGC) - 2e-3f64;
                    let CGH = CGF - (I * (CGG + (((CGG * CGG) + (8e-3f64 * CGF)).sqrt())));
                    let CGI = (LJ * (CGH - RP)) - B;
                    let CGJ = CGI + (CGD * ((LJ * CGH).exp()));
                    let CGK = (I * (CGJ + (((CGJ * CGJ) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CGL = if CGK < A { 1.0 } else { 0.0 };
                    let CGM = if CGL != 0.0 {
                        A
                    } else {
                        CGK
                    };
                    let CGN = (CGM + 2.220446049250313e-15f64).sqrt();
                    let CGO = (I * (CGI + (((CGI * CGI) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CGP = if CGO < A { 1.0 } else { 0.0 };
                    let CGQ = if CGP != 0.0 {
                        A
                    } else {
                        CGO
                    };
                    let CGT = CGR * (CGN - ((CGQ + 2.220446049250313e-15f64).sqrt()));
                    let CGU = CGC - CGH;
                    let CGV = (I * (CGU + (((CGU * CGU) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64;
                    let CGW = if CGV < A { 1.0 } else { 0.0 };
                    let CGX = if CGW != 0.0 {
                        A
                    } else {
                        CGV
                    };
                    let CGY = OR / (CGX + 2.220446049250313e-15f64);
                    let CGZ = CGY * CGY;
                    let CHA = (((CGZ * CGZ) * CGZ) * CGZ) + 1e0f64;
                    let CHQ;
                    if CHB != 0.0 {
                        let CHL;
                        if CHC != 0.0 {
                            CHL = B;
                        } else {
                            let CHM;
                            if CHD != 0.0 {
                                CHM = BF;
                            } else {
                                let CHN;
                                if CHE != 0.0 {
                                    CHN = BR;
                                } else {
                                    let CHO = if CHF != 0.0 {
                                        BL
                                    } else {
                                        A
                                    };
                                    CHN = CHO;
                                }
                                CHM = CHN;
                            }
                            CHL = CHM;
                        }
                        let mut CHG = 0.0;
                        let mut CHI = 0.0;
                        CHG = A;
                        CHI = CHA;
                        loop {
                            let CHH = if CHG < CHL { 1.0 } else { 0.0 };
                            if CHH == 0.0 {
                                break;
                            }
                            let CHJ = CHI.sqrt();
                            let CHK = CHG + B;
                            CHG = CHK;
                            CHI = CHJ;
                        }
                        CHQ = CHI;
                    } else {
                        let CHP = CHA.powf(1.25e-1f64);
                        CHQ = CHP;
                    }
                    let CHR = CFK + (((((((BF * ET) * CZ) * LL) * CEK) * CGT) * (CGY * (B / CHQ))) / CDN);
                    CII = CHR;
                } else {
                    CII = CFK;
                }
                let CHU = if (if CHS != A { 1.0 } else { 0.0 }) != 0.0 && (if CHT != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EEN;
                let EER;
                let EEV;
                let EFC;
                if CHU != 0.0 {
                    let CHX = CHV * CHV;
                    let CHY = CHX - (((BF * LL) * SP) * CBJ);
                    let CHZ = (I * (CHX + (((CHX * CHX) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let CIA = if CHZ < A { 1.0 } else { 0.0 };
                    let CID = if CIA != 0.0 {
                        A
                    } else {
                        CHZ
                    };
                    let CIB = (I * (CHY + (((CHY * CHY) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let CIC = if CIB < A { 1.0 } else { 0.0 };
                    let CIE = if CIC != 0.0 {
                        A
                    } else {
                        CIB
                    };
                    let CIF = CID - CIE;
                    let CIG = if (if CAX < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 || (if CIF < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EEO = if CIG != 0.0 {
                        A
                    } else {
                        B
                    };
                    EEN = EEO;
                    EER = CIE;
                    EEV = CID;
                    EFC = CIF;
                } else {
                    EEN = A;
                    EER = A;
                    EEV = A;
                    EFC = A;
                }
                CIH = CII;
                CSN = CDF;
                CVO = CEK;
                CVW = CDU;
                ECU = CDN;
                EDC = CCO;
                EDD = CBZ;
                EDS = EDT;
                EDZ = CDM;
                EEM = EEN;
                EEQ = EER;
                EEU = EEV;
                EFB = EFC;
                EGH = CCI;
                EGL = EGM;
                EGP = EGQ;
                EGT = EGU;
            } else {
                CIH = A;
                CSN = B;
                CVO = CVP;
                CVW = A;
                ECU = CU;
                EDC = A;
                EDD = A;
                EDS = EDW;
                EDZ = A;
                EEM = A;
                EEQ = A;
                EEU = A;
                EFB = A;
                EGH = EGI;
                EGL = EGN;
                EGP = EGR;
                EGT = EGV;
            }
            let CIK = if (if CEN > A { 1.0 } else { 0.0 }) != 0.0 && (if CIJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CUF;
            let CYE;
            if CIK != 0.0 {
                let CIM = UM - CIL;
                let CIN = TP + CIL;
                let CIO = LL * ((((AC / ME) * IC) / ME).ln());
                let CIP = if JB != 0.0 {
                    SJ
                } else {
                    CEY
                };
                let CIQ = ((((((3.2043836e-19f64 * (CIO - CIP)) / CI) * IC) * AC) / (IC + AC)).sqrt()) * CX;
                let CIR = ((-2.5e-1f64 * CIQ) * CIQ) / (OR + CIQ);
                let CIS = LJ * (CIM - CIR);
                let CIT = B + ((BL * (CIS - B)) / (UQ * LK));
                let CIU = if CIT >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let CIW = if CIU != 0.0 {
                    CIT
                } else {
                    CIV
                };
                let CIX = CIM + (((UQ * LJ) * I) * (B - (CIW.sqrt())));
                let CIY = if OX < ((EQ + CIN) * I) { 1.0 } else { 0.0 };
                if CIY != 0.0 {
                } else {
                }
                let CLF;
                let CLR;
                if CIZ != 0.0 {
                    let CJA = if (LJ * (CIX - CIR)) < BR { 1.0 } else { 0.0 };
                    let CLK;
                    let CLU;
                    if CJA != 0.0 {
                        let CJB = B / ((1.3094570021973102e-2f64 * LJ) * UP);
                        let CJC = AAL + (BR * CJB);
                        let CJD = (TW * CJB) * CIS;
                        let CJE = (AAO - (AAL * (AAP + CJB))) + CJD;
                        let CJF = (((-2.916e3f64 - (AAL * CJB)) + CJD) + (((((BL * CJC) * CJC) * CJC) + (CJE * CJE)).sqrt())).powf(AAR);
                        let CJG = (((BR - ((AAT * CJC) / (BR * CJF))) + (2.6456684199469993e-1f64 * CJF)) * LL) + CIR;
                        CLK = CJG;
                        CLU = CJG;
                    } else {
                        let CJI = if (OX - CJH) <= CIN { 1.0 } else { 0.0 };
                        let CLL;
                        let CLV;
                        if CJI != 0.0 {
                            let CJO = if F != 0.0 {
                                let CJJ = H / CI;
                                let CJK = B / CP;
                                let CJN = CIM - (((B / (((B / TK) + CJJ) + CJK)) * ((CIM - CJL) + ((CJK + (I * CJJ)) * (-CJM)))) / TK);
                                CJN
                            } else {
                                CIX
                            };
                            CLL = CJO;
                            CLV = CJO;
                        } else {
                            let CJP = CIM - CJH;
                            let CJR = ((((((B / MV) / UT) * CJP) * CJP).ln()) / (LJ + (BF / CJP))) + CJQ;
                            let CJS = (CJR - CIX) - VX;
                            let CJT = (BL * CJR) * VX;
                            let CJU = if CJT > A { 1.0 } else { 0.0 };
                            let CJW = if CJU != 0.0 {
                                CJT
                            } else {
                                let CJV = -CJT;
                                CJV
                            };
                            let CJX = CJR - (I * (CJS + (((CJS * CJS) + CJW).sqrt())));
                            CLL = CJX;
                            CLV = CIX;
                        }
                        CLK = CLL;
                        CLU = CLV;
                    }
                    let CLG;
                    let CLS;
                    if F != 0.0 {
                        let CJY = if (OX - CJH) <= CIN { 1.0 } else { 0.0 };
                        let CLH;
                        let CLT;
                        if CJY != 0.0 {
                            let CJZ = H / CI;
                            let CKA = B / CP;
                            let CKB = CIM - (((B / (((B / TK) + CJZ) + CKA)) * ((CIM - CJL) + ((CKA + (I * CJZ)) * (-CJM)))) / TK);
                            CLH = CKB;
                            CLT = CKB;
                        } else {
                            let CKC = H / CI;
                            let CKD = B / CP;
                            let CKE = CIM - (((B / (((B / TK) + CKC) + CKD)) * ((CIM - CJL) + ((CKD + (I * CKC)) * (-CJM)))) / TK);
                            let CKF = CIM - CJH;
                            let CKG = if CKF > A { 1.0 } else { 0.0 };
                            let CLI;
                            if CKG != 0.0 {
                                let CKH = (((((((B / MV) / UT) * CKF) * CKF).ln()) / (LJ + (BF / CKF))) + CJQ) * ABX;
                                let CKI = CKH - LY;
                                let CKJ = if (if CKE > CKI { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                                let CLJ;
                                if CKJ != 0.0 {
                                    let CKK = (CKE - CKH) + LY;
                                    let CKL = CKK * CKK;
                                    let CKM = (CKL * CKL) + 2.560000000000001e-2f64;
                                    let CLC;
                                    if CKN != 0.0 {
                                        let CKX;
                                        if CKO != 0.0 {
                                            CKX = B;
                                        } else {
                                            let CKY;
                                            if CKP != 0.0 {
                                                CKY = BF;
                                            } else {
                                                let CKZ;
                                                if CKQ != 0.0 {
                                                    CKZ = BR;
                                                } else {
                                                    let CLA = if CKR != 0.0 {
                                                        BL
                                                    } else {
                                                        A
                                                    };
                                                    CKZ = CLA;
                                                }
                                                CKY = CKZ;
                                            }
                                            CKX = CKY;
                                        }
                                        let mut CKS = 0.0;
                                        let mut CKU = 0.0;
                                        CKS = A;
                                        CKU = CKM;
                                        loop {
                                            let CKT = if CKS < CKX { 1.0 } else { 0.0 };
                                            if CKT == 0.0 {
                                                break;
                                            }
                                            let CKV = CKU.sqrt();
                                            let CKW = CKS + B;
                                            CKS = CKW;
                                            CKU = CKV;
                                        }
                                        CLC = CKU;
                                    } else {
                                        let CLB = CKM.powf(2.5e-1f64);
                                        CLC = CLB;
                                    }
                                    let CLD = CKI + ((CKK * LY) * (B / CLC));
                                    CLJ = CLD;
                                } else {
                                    CLJ = CKE;
                                }
                                CLI = CLJ;
                            } else {
                                CLI = CKE;
                            }
                            CLH = CLI;
                            CLT = CKE;
                        }
                        CLG = CLH;
                        CLS = CLT;
                    } else {
                        CLG = CLK;
                        CLS = CLU;
                    }
                    CLF = CLG;
                    CLR = CLS;
                } else {
                    CLF = CLM;
                    CLR = CIX;
                }
                let CLE = CIR + 2.5e-12f64;
                let CLP = if CLF < CLE { 1.0 } else { 0.0 };
                let CLQ = if CLP != 0.0 {
                    CLE
                } else {
                    CLF
                };
                if A != 0.0 {
                    let CLW = CLR - CLQ;
                    let CLX = if CLW >= A { 1.0 } else { 0.0 };
                    let CLY = if CLX != 0.0 {
                        CLW
                    } else {
                        A
                    };
                    let CLZ = ((1.3e0f64 * CLY) - CJQ) - AGW;
                    let CMA = (BL * (1.3e0f64 * CLY)) * AGW;
                    let CMB = if CMA > A { 1.0 } else { 0.0 };
                    let CMD = if CMB != 0.0 {
                        CMA
                    } else {
                        let CMC = -CMA;
                        CMC
                    };
                    let CME = (1.3e0f64 * CLY) - (I * (CLZ + (((CLZ * CLZ) + CMD).sqrt())));
                    let CMF = if CME <= CLY { 1.0 } else { 0.0 };
                    let CMG = if CMF != 0.0 {
                        CME
                    } else {
                        CLY
                    };
                    let CMH = if CMG < A { 1.0 } else { 0.0 };
                    if CMH != 0.0 {
                    } else {
                        let CMI = if CMG > OR { 1.0 } else { 0.0 };
                        if CMI != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let CMJ = if parameters[282] == B { 1.0 } else { 0.0 };
                let CQB;
                if CMJ != 0.0 {
                    let CMK = if OX < ((UO + CIR) + CIL) { 1.0 } else { 0.0 };
                    let CQC;
                    if CMK != 0.0 {
                        let CML = (BF * LL) * (((-GH) / UP).ln());
                        let CMM = (B / (LJ * MO)) * TK;
                        let CMN = BF + (4.242640687119285e0f64 * CMM);
                        let CMO = ((BM * CMN) * CMN) * CMN;
                        let CMP = (BLP * CMM) * (CIS - BF);
                        let CMQ = 9.899494936611664e0f64 - CMP;
                        let CMR = CMQ * CMQ;
                        let CMS = if CMO < (CMR * BLT) { 1.0 } else { 0.0 };
                        let CMV = if CMS != 0.0 {
                            let CMT = ((-9.899494936611664e0f64 + CMQ) + ((I * CMO) / CMQ)) + CMP;
                            CMT
                        } else {
                            let CMU = (-9.899494936611664e0f64 + ((CMO + CMR).sqrt())) + CMP;
                            CMU
                        };
                        let CMW = CMV.powf(AAR);
                        let CMX = ((((((-5.65685424949238e0f64 - (BLZ * CMM)) + (BF * CMW)) + ((MN * CMW) * CMW)) * (B / CMW)) * LL) + CIR) - CIR;
                        let CMY = CMX / CML;
                        let CMZ = (CMX / ((B + (CMY * CMY)).sqrt())) + CIR;
                        CQC = CMZ;
                    } else {
                        let CNA = (LJ * (CIR - CJQ)).exp();
                        let CNB = (((ID * H) * H) / BF) / CI;
                        let CNC = ((BF * LJ) * CNB).sqrt();
                        let CND = ((((CNC.exp()) + ((-CNC).exp())) / BF).ln()) / CNB;
                        let mut CNE = 0.0;
                        let mut CNG = 0.0;
                        let mut COM = 0.0;
                        CNE = B;
                        CNG = CLQ;
                        COM = A;
                        loop {
                            let CNF = if CNE <= 2.01e2f64 { 1.0 } else { 0.0 };
                            if CNF == 0.0 {
                                break;
                            }
                            let CNH = CNG - CIR;
                            let CNI = LJ * CNH;
                            let CNJ = CNH - CNB;
                            let CNK = CND * CNJ;
                            let CNL = if CNK < ARC { 1.0 } else { 0.0 };
                            let CNQ;
                            let CNU;
                            if CNL != 0.0 {
                                let CNM = CNK.exp();
                                let CNN = B + (CNM - (((-CND) * CNB).exp()));
                                let CNO = (CNN.ln()) / CND;
                                let CNP = CNM / CNN;
                                CNQ = CNO;
                                CNU = CNP;
                            } else {
                                CNQ = CNJ;
                                CNU = B;
                            }
                            let CNR = LJ * CNQ;
                            let CNS = CNI.abs();
                            let CNT = if CNS < BOF { 1.0 } else { 0.0 };
                            let COQ;
                            let COU;
                            if CNT != 0.0 {
                                let CNV = ((B - (CNU * CNU)) / BF).sqrt();
                                let CNW = CNI * CNV;
                                let CNX = LJ * CNV;
                                let CNY = if CNI < A { 1.0 } else { 0.0 };
                                let COR;
                                let COV;
                                if CNY != 0.0 {
                                    let CNZ = -CNW;
                                    let COA = -CNX;
                                    COR = CNZ;
                                    COV = COA;
                                } else {
                                    COR = CNW;
                                    COV = CNX;
                                }
                                COQ = COR;
                                COU = COV;
                            } else {
                                let COB = if CNS < BOO { 1.0 } else { 0.0 };
                                let COS;
                                let COW;
                                if COB != 0.0 {
                                    let COC = CNI / BR;
                                    let COD = CNI / BL;
                                    let COE = CNR / BR;
                                    let COF = CNR / BL;
                                    let COG = ((((CNI * CNI) / BF) * (B - (COC * (B - (COD * (B - (CNI / KY))))))) - (((CNR * CNR) / BF) * (B - (COE * (B - (COF * (B - (CNR / KY)))))))).sqrt();
                                    let COH = ((LJ * I) * ((CNI * (B - ((CNI / BF) * (B - (COC * (B - COD)))))) - (CNU * (CNR * (B - ((CNR / BF) * (B - (COE * (B - COF))))))))) / COG;
                                    COS = COG;
                                    COW = COH;
                                } else {
                                    let COI = (-CNI).exp();
                                    let COJ = (-CNR).exp();
                                    let COK = ((CNI - CNR) + (COI - COJ)).sqrt();
                                    let COL = ((LJ * I) * ((B - COI) - (CNU * (B - COJ)))) / COK;
                                    COS = COK;
                                    COW = COL;
                                }
                                COQ = COS;
                                COU = COW;
                            }
                            let CON = if COM == B { 1.0 } else { 0.0 };
                            let COO = if CNI < A { 1.0 } else { 0.0 };
                            let COP = if CON != 0.0 && COO != 0.0 { 1.0 } else { 0.0 };
                            if COP != 0.0 {
                            } else {
                            }
                            let CPC;
                            let CPF;
                            if COO != 0.0 {
                                let COT = -COQ;
                                let COX = -COU;
                                CPC = COT;
                                CPF = COX;
                            } else {
                                let COY = if CNI < CF { 1.0 } else { 0.0 };
                                let CPD;
                                let CPG;
                                if COY != 0.0 {
                                    CPD = COQ;
                                    CPG = COU;
                                } else {
                                    let COZ = (LJ * (CNG - CJQ)).exp();
                                    let CPA = ((COQ * COQ) + (MV * (COZ - (CNA * (CNI + B))))).sqrt();
                                    let CPB = (I * (((BF * COU) * COQ) + ((MV * LJ) * (COZ - CNA)))) / CPA;
                                    CPD = CPA;
                                    CPG = CPB;
                                }
                                CPC = CPD;
                                CPF = CPG;
                            }
                            let CPE = ((-CIM) + CNG) + (UP * CPC);
                            let CPH = B + (UP * CPF);
                            let CPW;
                            let CPY;
                            let CPZ;
                            if CON != 0.0 {
                                CPW = CPI;
                                CPY = CNG;
                                CPZ = COM;
                            } else {
                                let CPJ = (-CPE) / CPH;
                                let CPK = CNG.abs();
                                let CPL = if B >= CPK { 1.0 } else { 0.0 };
                                let CPM = if CPL != 0.0 {
                                    B
                                } else {
                                    CPK
                                };
                                let CPN = 5e-2f64 * (B + CPM);
                                let CPO = if (CPJ.abs()) > CPN { 1.0 } else { 0.0 };
                                let CPT;
                                if CPO != 0.0 {
                                    let CPP = if CPJ >= A { 1.0 } else { 0.0 };
                                    let CPR = if CPP != 0.0 {
                                        B
                                    } else {
                                        CPQ
                                    };
                                    let CPS = CPN * CPR;
                                    CPT = CPS;
                                } else {
                                    CPT = CPJ;
                                }
                                let CPU = CNG + CPT;
                                let CPV = if (if (CPT.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (CPE.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let CQA = if CPV != 0.0 {
                                    B
                                } else {
                                    COM
                                };
                                CPW = CNE;
                                CPY = CPU;
                                CPZ = CQA;
                            }
                            let CPX = CPW + B;
                            CNE = CPX;
                            CNG = CPY;
                            COM = CPZ;
                        }
                        CQC = CNG;
                    }
                    CQB = CQC;
                } else {
                    CQB = CLQ;
                }
                let CQD = CQB - CIR;
                let CQE = (-LJ) * CQD;
                let CQF = if CQE >= A { 1.0 } else { 0.0 };
                let CQH = if CQF != 0.0 {
                    B
                } else {
                    CQG
                };
                let CQI = CQH * CQE;
                let CQJ = ((CQE.exp()) - B) - CQE;
                let CQK = if CQE > CF { 1.0 } else { 0.0 };
                let CQP;
                if CQK != 0.0 {
                    let CQL = (-MO) * (CQJ.sqrt());
                    CQP = CQL;
                } else {
                    let CQM = if CQI > CF { 1.0 } else { 0.0 };
                    let CQQ = if CQM != 0.0 {
                        let CQN = MO * (CQJ.sqrt());
                        CQN
                    } else {
                        let CQO = (((-CQH) * CQI) * 7.071067811865475e-1f64) * ((B + ((CQI * AAR) * (B + (AHV * CQI)))).sqrt());
                        CQO
                    };
                    CQP = CQQ;
                }
                let CQR = (I * (CQP + (((CQP * CQP) + 4e-12f64).sqrt()))) + 1e-16f64;
                let CQS = if CQR < A { 1.0 } else { 0.0 };
                let CQT = if CQS != 0.0 {
                    A
                } else {
                    CQR
                };
                let CQU = CQT / ID;
                let CQV = CQU - parameters[283];
                let CQW = CQU * M;
                let CQX = (I * (CQV + (((CQV * CQV) + ((BL * CQW) * CQW)).sqrt()))) + (IN * CQW);
                let CQY = if CQX < A { 1.0 } else { 0.0 };
                let CQZ = if CQY != 0.0 {
                    A
                } else {
                    CQX
                };
                let CRA = (CQD * (((CQZ / CQU) * CQZ) / CQU)) + CIR;
                let CRB = ((LJ * CRA).exp()) - ((LJ * (CRA - OR)).exp());
                let CRC = (((3.2043836e-19f64 * AC) * CI).sqrt()) * MF;
                let CRD = LJ * (CRA - CIR);
                let CRE = AFV * LJ;
                let CRF = if (if CRD < CRE { 1.0 } else { 0.0 }) != 0.0 && (if CRE >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CRZ;
                if CRF != 0.0 {
                    let CRG = CRE - CRD;
                    let CRH = (CRG * CRG) + (CRE * CRE);
                    let CRX;
                    if CRI != 0.0 {
                        let CRS;
                        if CRJ != 0.0 {
                            CRS = B;
                        } else {
                            let CRT;
                            if CRK != 0.0 {
                                CRT = BF;
                            } else {
                                let CRU;
                                if CRL != 0.0 {
                                    CRU = BR;
                                } else {
                                    let CRV = if CRM != 0.0 {
                                        BL
                                    } else {
                                        A
                                    };
                                    CRU = CRV;
                                }
                                CRT = CRU;
                            }
                            CRS = CRT;
                        }
                        let mut CRN = 0.0;
                        let mut CRP = 0.0;
                        CRN = A;
                        CRP = CRH;
                        loop {
                            let CRO = if CRN < CRS { 1.0 } else { 0.0 };
                            if CRO == 0.0 {
                                break;
                            }
                            let CRQ = CRP.sqrt();
                            let CRR = CRN + B;
                            CRN = CRR;
                            CRP = CRQ;
                        }
                        CRX = CRP;
                    } else {
                        let CRW = CRH.sqrt();
                        CRX = CRW;
                    }
                    let CRY = CRE - ((CRG * CRE) * (B / CRX));
                    CRZ = CRY;
                } else {
                    CRZ = CRD;
                }
                let CSA = CIH + ((((((BF * LL) / CX) * (CRC * ((CRZ + 2.220446049250313e-15f64).sqrt()))) * CIJ) * DP) * CRB);
                CUF = CSA;
                CYE = CQP;
            } else {
                CUF = CIH;
                CYE = CCC;
            }
            let CSB = if JB != 0.0 || (if parameters[45] == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CUK;
            if CSB != 0.0 {
                let CSC = if (if CAI == B { 1.0 } else { 0.0 }) != 0.0 || (if AFT == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CUL;
                if CSC != 0.0 {
                    CUL = A;
                } else {
                    let CSD = if (if FH <= A { 1.0 } else { 0.0 }) != 0.0 || (if N <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CUM;
                    if CSD != 0.0 {
                        CUM = A;
                    } else {
                        let CSE = (((PM - FZ) + TO) - UL) + parameters[48];
                        let CUB;
                        if EX != 0.0 {
                            let CSF = TK * TK;
                            let CSG = IE / CSF;
                            let CSJ = B + (((BF / IE) * CSF) * (((CSE - LL) - (AIL * RP)) - (AIL * ((CSH * CSI) / CJ))));
                            let CSK = (I * (CSJ + (((CSJ * CSJ) + 4e-6f64).sqrt()))) + 1e-13f64;
                            let CSL = if CSK < A { 1.0 } else { 0.0 };
                            let CSM = if CSL != 0.0 {
                                A
                            } else {
                                CSK
                            };
                            let CSO = ((AIS * PL) + CSN) - ((AIT * AIU) * ((CSE * AIQ) + (CSG * (B - ((CSM + GD).sqrt())))));
                            let CSP = (I * (CSO + (((CSO * CSO) + 4e-4f64).sqrt()))) + 1e-12f64;
                            let CSQ = if CSP < A { 1.0 } else { 0.0 };
                            let CUC = if CSQ != 0.0 {
                                A
                            } else {
                                CSP
                            };
                            CUB = CUC;
                        } else {
                            let CSR = AIZ * CSE;
                            let CSS = TK * TK;
                            let CST = IE / CSS;
                            let CSU = (BF / IE) * CSS;
                            let CSV = B + (CSU * (((CSR - LL) - (AIL * RP)) - (AIL * ((CSH * CSI) / CJ))));
                            let CSW = BF * (B + CSU);
                            let CSX = GD + CSW;
                            let CSY = if (if CSV < CSX { 1.0 } else { 0.0 }) != 0.0 && (if CSW >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let CTU;
                            if CSY != 0.0 {
                                let CSZ = CSX - CSV;
                                let CTA = CSZ * CSZ;
                                let CTB = CSW * CSW;
                                let CTC = (((CTA * CTA) * CTA) * CTA) + (((CTB * CTB) * CTB) * CTB);
                                let CTS;
                                if CTD != 0.0 {
                                    let CTN;
                                    if CTE != 0.0 {
                                        CTN = B;
                                    } else {
                                        let CTO;
                                        if CTF != 0.0 {
                                            CTO = BF;
                                        } else {
                                            let CTP;
                                            if CTG != 0.0 {
                                                CTP = BR;
                                            } else {
                                                let CTQ = if CTH != 0.0 {
                                                    BL
                                                } else {
                                                    A
                                                };
                                                CTP = CTQ;
                                            }
                                            CTO = CTP;
                                        }
                                        CTN = CTO;
                                    }
                                    let mut CTI = 0.0;
                                    let mut CTK = 0.0;
                                    CTI = A;
                                    CTK = CTC;
                                    loop {
                                        let CTJ = if CTI < CTN { 1.0 } else { 0.0 };
                                        if CTJ == 0.0 {
                                            break;
                                        }
                                        let CTL = CTK.sqrt();
                                        let CTM = CTI + B;
                                        CTI = CTM;
                                        CTK = CTL;
                                    }
                                    CTS = CTK;
                                } else {
                                    let CTR = CTC.powf(1.25e-1f64);
                                    CTS = CTR;
                                }
                                let CTT = CSX - ((CSZ * CSW) * (B / CTS));
                                CTU = CTT;
                            } else {
                                CTU = CSV;
                            }
                            let CTV = if CTU <= A { 1.0 } else { 0.0 };
                            let CTX = if CTV != 0.0 {
                                A
                            } else {
                                let CTW = CTU.sqrt();
                                CTW
                            };
                            let CTY = ((AIS * PL) + CSN) - ((CY / (AIT + CY)) * (CSR + (CST * (B - CTX))));
                            let CTZ = (I * (CTY + (((CTY * CTY) + 4e-6f64).sqrt()))) + 1e-13f64;
                            let CUA = if CTZ < A { 1.0 } else { 0.0 };
                            let CUD = if CUA != 0.0 {
                                A
                            } else {
                                CTZ
                            };
                            CUB = CUD;
                        }
                        let CUE = CUB + GD;
                        let CUG = ((AKQ * CUE) * CUF) * (((-AKP) / CUE).exp());
                        CUM = CUG;
                    }
                    CUL = CUM;
                }
                CUK = CUL;
            } else {
                CUK = CUN;
            }
            let CUH = if (if AFT == B { 1.0 } else { 0.0 }) != 0.0 && (if AKS == BF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CUI = if CUH != 0.0 && JB != 0.0 { 1.0 } else { 0.0 };
            if CUI != 0.0 {
                let CUJ = -LJ;
                let CUQ = MY * M;
                let CUR = (MY - ((AKX * LL) * ((B + (CUK * (2.1633307652783932e-2f64 / ((((ED * H) * DP) * ((CUJ * AKU).exp())) * (4.1046315303568966e26f64 + (2.4665765749313358e0f64 * HY)))))).ln()))) - CUQ;
                let CUS = (BL * MY) * CUQ;
                let CUT = if CUS > A { 1.0 } else { 0.0 };
                let CUV = if CUT != 0.0 {
                    CUS
                } else {
                    let CUU = -CUS;
                    CUU
                };
                let CUW = CSN - (MY - (I * (CUR + (((CUR * CUR) + CUV).sqrt()))));
                let CUX = if ((((CUJ * CUW).exp()) - B) + (LJ * CUW)) > A { 1.0 } else { 0.0 };
                if CUX != 0.0 {
                } else {
                }
                let CUZ = if ((BL * CUY) * (CUY * M)) > A { 1.0 } else { 0.0 };
                if CUZ != 0.0 {
                } else {
                }
                let CVA = if parameters[138] > A { 1.0 } else { 0.0 };
                if CVA != 0.0 {
                } else {
                }
            } else {
            }
            let CVB = if CAI == A { 1.0 } else { 0.0 };
            let CVC = if (if CVB != 0.0 && (if CUK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[146] != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if CVC != 0.0 {
                let CVI;
                let CVL;
                if RN != 0.0 {
                    CVI = A;
                    CVL = A;
                } else {
                    let CVD = if JB != 0.0 {
                        PB
                    } else {
                        CEY
                    };
                    let CVH = if JB != 0.0 {
                        PB
                    } else {
                        CVE
                    };
                    CVI = CVD;
                    CVL = CVH;
                }
                let CVJ = (LJ * (CAT - CVI)) - B;
                let CVK = if ((I * (CVJ + (((CVJ * CVJ) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64) < A { 1.0 } else { 0.0 };
                if CVK != 0.0 {
                } else {
                }
                let CVM = (LJ * (CAP - CVL)) - B;
                let CVN = if ((I * (CVM + (((CVM * CVM) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64) < A { 1.0 } else { 0.0 };
                if CVN != 0.0 {
                } else {
                }
            } else {
            }
            let CVS = CH * AX;
            let CVT = TK / IZ;
            let CVU = CU * AX;
            let CVV = DP * AX;
            let CVX = CVW / AX;
            let CVY = CCJ / IZ;
            let CVZ = MO / IZ;
            let CWB = if CWA == A { 1.0 } else { 0.0 };
            let EJQ;
            let EJU;
            let EJV;
            let EJY;
            let EKB;
            if CWB != 0.0 {
                EJQ = A;
                EJU = A;
                EJV = A;
                EJY = A;
                EKB = A;
            } else {
                let EJW;
                if CVB != 0.0 {
                    let CWC = ((((PM - EQ) + ((parameters[216] * (TO - UL)) * CVU)) - (((CSN + PL) - 2.220446049250313e-15f64) * parameters[215])) * (B / CVS)) * (B + (CVX * (B / parameters[217])));
                    let CWD = (I * (CWC + (((CWC * CWC) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let CWE = if CWD < A { 1.0 } else { 0.0 };
                    let CWJ = if CWE != 0.0 {
                        A
                    } else {
                        CWD
                    };
                    let CWF = (I * (PM + (((PM * PM) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let CWG = if CWF < A { 1.0 } else { 0.0 };
                    let CWH = if CWG != 0.0 {
                        A
                    } else {
                        CWF
                    };
                    let CWI = (CWH - PE) / BG;
                    let CWK = CWJ * (B - (B / (B + (CWI * CWI))));
                    let CWL = CVU * CVV;
                    let CWN = CWM / (CWM + CWL);
                    let CWP = CWO / (CWO + PL);
                    let CWQ = ((-parameters[214]) * MD) * (B / (CWK + GD));
                    let CWR = if CWQ < -3.4e1f64 { 1.0 } else { 0.0 };
                    let EJX = if CWR != 0.0 {
                        A
                    } else {
                        let CWS = (CWN * CWP) * (((((CWQ.exp()) * (((parameters[213] / MC) * ED) * CWL)) * (((CVY + (CVT * G)) * (B / CVZ)).sqrt())) * CWK) * CWK);
                        CWS
                    };
                    EJW = EJX;
                } else {
                    EJW = A;
                }
                let CWT = -parameters[221];
                let CWV = (parameters[220] / AS) * CVV;
                let CWW = (CWV * ((CVS * ((CWT * OX) + CWU)).exp())) * (OX * ((OX / CVS) / CVS));
                let CWX = if OX >= A { 1.0 } else { 0.0 };
                let EKC = if CWX != 0.0 {
                    let CWY = CWW * -1e0f64;
                    CWY
                } else {
                    CWW
                };
                let CWZ = OX - OR;
                let CXA = (CWV * ((CVS * ((CWT * CWZ) + CWU)).exp())) * (CWZ * ((CWZ / CVS) / CVS));
                let CXB = if CWZ >= A { 1.0 } else { 0.0 };
                let EJZ = if CXB != 0.0 {
                    let CXC = CXA * -1e0f64;
                    CXC
                } else {
                    CXA
                };
                let CXD = ((((-OX) + PT) + EQ) + parameters[225]) / CVS;
                let CXE = (I * (CXD + (((CXD * CXD) + 4e-4f64).sqrt()))) + 1e-12f64;
                let CXF = if CXE < A { 1.0 } else { 0.0 };
                let CXG = if CXF != 0.0 {
                    A
                } else {
                    CXE
                };
                let CXH = CXG + GD;
                let CXI = (-parameters[224]) / CXH;
                let CXJ = if CXI < -3.4e1f64 { 1.0 } else { 0.0 };
                let EJR = if CXJ != 0.0 {
                    A
                } else {
                    let CXK = ((((parameters[223] * CVV) * CVU) * CXH) * CXH) * (CXI.exp());
                    CXK
                };
                EJQ = EJR;
                EJU = I;
                EJV = EJW;
                EJY = EJZ;
                EKB = EKC;
            }
            let CXL = if parameters[28] == A { 1.0 } else { 0.0 };
            if CXL != 0.0 {
            } else {
                let CXP = (((CXM * (OR + CXN)) - OX) + (TM * CXO)) * (B / CH);
                let CXQ = (I * (CXP + (((CXP * CXP) + 4e-4f64).sqrt()))) + 1e-12f64;
                let CXR = if CXQ < A { 1.0 } else { 0.0 };
                let CXS = if CXR != 0.0 {
                    A
                } else {
                    CXQ
                };
                let CXU = if (((-CXT) * MD) * (B / (CXS + GD))) < -3.4e1f64 { 1.0 } else { 0.0 };
                if CXU != 0.0 {
                } else {
                }
                let CXV = if (OR - PT) > A { 1.0 } else { 0.0 };
                if CXV != 0.0 {
                } else {
                }
            }
            if CXL != 0.0 {
            } else {
                let CXW = (((CXM * ((-OR) + CXN)) - (OX - OR)) + (TM * CXO)) * (B / CH);
                let CXX = (I * (CXW + (((CXW * CXW) + 4e-4f64).sqrt()))) + 1e-12f64;
                let CXY = if CXX < A { 1.0 } else { 0.0 };
                let CXZ = if CXY != 0.0 {
                    A
                } else {
                    CXX
                };
                let CYA = if (((-CXT) * MD) * (B / (CXZ + GD))) < -3.4e1f64 { 1.0 } else { 0.0 };
                if CYA != 0.0 {
                } else {
                }
                let CYB = if (-PT) > A { 1.0 } else { 0.0 };
                if CYB != 0.0 {
                } else {
                }
            }
            let EHM;
            let EHT;
            let EIA;
            let EIL;
            if JB != 0.0 {
                let CYC = B / CM;
                let CYD = -BRU;
                let CYF = (CYD * CCJ) + (CYD * CYE);
                let CYG = CYF * I;
                let CYH = CYF - CYG;
                let EHN;
                let EHU;
                let EIB;
                let EIM;
                if JC != 0.0 {
                    let CYP;
                    let CZQ;
                    let DEW;
                    if CYI != 0.0 {
                        let CYL = CYJ * I;
                        CYP = GL;
                        CZQ = CYM;
                        DEW = CYL;
                    } else {
                        let CYQ;
                        let CZR;
                        let DEX;
                        if CYN != 0.0 {
                            let CYO = BRU * I;
                            CYQ = B;
                            CZR = EQ;
                            DEX = CYO;
                        } else {
                            CYQ = A;
                            CZR = A;
                            DEX = A;
                        }
                        CYP = CYQ;
                        CZQ = CZR;
                        DEW = DEX;
                    }
                    let CYR = if CYP == A { 1.0 } else { 0.0 };
                    let EHO;
                    let EHV;
                    let EIC;
                    let EIN;
                    if CYR != 0.0 {
                        let CYS = MO * ((IC / IC).sqrt());
                        let CYX = (CYV * PB) + (CYW * (PB - OR));
                        let CYY = OX - OR;
                        let CYZ = (CYV * OX) + (CYW * CYY);
                        let CZA = (CYW * OX) + (CYV * CYY);
                        let CZB = ((CYV * OR) + (CYW * (-OR))) - CYX;
                        let CZC = -CYX;
                        let CZD = CYV + (CYU * CYW);
                        let CZE = CYW + (CYU * CYV);
                        let CZF = (CZD * CYZ) + (CZE * CZA);
                        let CZG = -(((CZD * CZC) + (CZE * CZB)) + 2.220446049250313e-15f64);
                        let CZH = if CZG > NN { 1.0 } else { 0.0 };
                        let CZM = if CZH != 0.0 {
                            let CZI = NJ - NN;
                            let CZJ = (CZG - NN) / CZI;
                            let CZK = CZJ * CZJ;
                            let CZL = NN + (CZI * (B - (B / ((((B + CZJ) + CZK) + (CZK * CZJ)) + (CZK * CZK)))));
                            CZL
                        } else {
                            CZG
                        };
                        let CZN = (-CZM) - G;
                        let CZO = CYS * CYC;
                        let CZP = CZO * CZO;
                        let CZS = CZF - CZQ;
                        let CZT = (BF / LJ) * ((IC / ME).ln());
                        let CZU = -CZN;
                        let CZV = if CZS < CZU { 1.0 } else { 0.0 };
                        let DET;
                        let DKC;
                        let DKJ;
                        let DKM;
                        if CZV != 0.0 {
                            let CZW = (B / (LJ * CYS)) * CM;
                            let CZX = BF + (4.242640687119285e0f64 * CZW);
                            let CZY = ((BM * CZX) * CZX) * CZX;
                            let CZZ = LI - CZT;
                            let DAA = (BLP * CZW) * ((LJ * (CZS + CZN)) - BF);
                            let DAB = 9.899494936611664e0f64 - DAA;
                            let DAC = DAB * DAB;
                            let DAD = if CZY < (DAC * BLT) { 1.0 } else { 0.0 };
                            let DAG = if DAD != 0.0 {
                                let DAE = ((-9.899494936611664e0f64 + DAB) + ((I * CZY) / DAB)) + DAA;
                                DAE
                            } else {
                                let DAF = (-9.899494936611664e0f64 + ((CZY + DAC).sqrt())) + DAA;
                                DAF
                            };
                            let DAH = DAG.powf(AAR);
                            let DAI = ((((((-5.65685424949238e0f64 - (BLZ * CZW)) + (BF * DAH)) + ((MN * DAH) * DAH)) / DAH) * LL) - CZN) + CZN;
                            let DAJ = DAI / CZZ;
                            let DAK = CM * (CZS - ((DAI / ((B + (DAJ * DAJ)).sqrt())) - CZN));
                            DET = DAK;
                            DKC = A;
                            DKJ = A;
                            DKM = A;
                        } else {
                            let DAL = CZS + CZN;
                            let DAM = (LJ * DAL) - B;
                            let DAN = CZP * LK;
                            let DAO = B + ((BL * (DAM + 4.9787068367863944e-2f64)) / DAN);
                            let DAP = if DAO < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DAS = if DAP != 0.0 {
                                DAQ
                            } else {
                                DAO
                            };
                            let DAR = (CZP * LJ) / BF;
                            let DAT = B + ((BL * (DAM + ((-(LJ * ((CZS + (DAR * (B - (DAS.sqrt())))) + CZN))).exp()))) / DAN);
                            let DAU = if DAT < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DAW = if DAU != 0.0 {
                                DAV
                            } else {
                                DAT
                            };
                            let DAX = LJ * ((CZS + (DAR * (B - (DAW.sqrt())))) + CZN);
                            let DAY = if DAX < BR { 1.0 } else { 0.0 };
                            let DBS = if DAY != 0.0 {
                                let DAZ = 7.071067811865476e-1f64 + (B / (LJ * CZO));
                                let DBA = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DAZ) / 5.286687693921294e-4f64)) + (((-DAL) / CZO) / 1.8773541122053122e-2f64);
                                let DBB = ((2.8160311683079683e-2f64 * DAZ) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DBC = ((DBA * DBA) + ((DBB * DBB) * DBB)).sqrt();
                                let DBD = LJ * ((((((((-DBA) + DBC).powf(AAR)) + (-((DBA + DBC).powf(AAR)))) - -3.7209791878387604e0f64) * LL) - CZN) + CZN);
                                DBD
                            } else {
                                DAX
                            };
                            let DBE = (LJ * CZU).exp();
                            let DBF = ME / IC;
                            let DBG = DBF * DBF;
                            let DBH = LJ * (DAL + BG);
                            let DBI = (DBG * (DBE + GD)) * DAN;
                            let DBJ = (DBG * DAN).ln();
                            let DBK = LJ * CZN;
                            let DBL = (DBH - ((((DBI + (DBH * DBH)).ln()) - DBJ) + DBK)) - B;
                            let DBM = BL * DBH;
                            let DBN = if DBM > A { 1.0 } else { 0.0 };
                            let DBP = if DBN != 0.0 {
                                DBM
                            } else {
                                let DBO = -DBM;
                                DBO
                            };
                            let DBQ = (DBH - (DBH - (I * (DBL + (((DBL * DBL) + DBP).sqrt()))))) + (LJ * BG);
                            let DBR = (((DBI + (DBQ * DBQ)).ln()) - DBJ) + DBK;
                            let DBT = (DBR - DBS) - 6.0000000000000005e-2f64;
                            let DBU = (BL * DBR) * 6.0000000000000005e-2f64;
                            let DBV = if DBU > A { 1.0 } else { 0.0 };
                            let DBX = if DBV != 0.0 {
                                DBU
                            } else {
                                let DBW = -DBU;
                                DBW
                            };
                            let DBY = DBR - (I * (DBT + (((DBT * DBT) + DBX).sqrt())));
                            let DBZ = (DBY / LJ) - CZN;
                            let DCA = if ((DBY - B) + ((-DBY).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DCA != 0.0 {
                            } else {
                            }
                            let DCB = CM * (CZS - DBZ);
                            let DCD = if DCC == B { 1.0 } else { 0.0 };
                            let DEU;
                            let DKD;
                            let DKK;
                            let DKN;
                            if DCD != 0.0 {
                                let DCE = DBG * DBE;
                                let mut DCF = 0.0;
                                let mut DCH = 0.0;
                                let mut DDL = 0.0;
                                let mut DEH = 0.0;
                                let mut DEK = 0.0;
                                let mut DEP = 0.0;
                                let mut DEQ = 0.0;
                                DCF = B;
                                DCH = DBZ;
                                DDL = A;
                                DEH = DBY;
                                DEK = A;
                                DEP = A;
                                DEQ = A;
                                loop {
                                    let DCG = if DCF <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DCG == 0.0 {
                                        break;
                                    }
                                    let DCI = LJ * (DCH + CZN);
                                    let DCJ = if DCI < KY { 1.0 } else { 0.0 };
                                    let DDH;
                                    let DDJ;
                                    let DEL;
                                    let DER;
                                    if DCJ != 0.0 {
                                        let DCK = DCI * DCI;
                                        let DCN = (DCK * DCI) * (DCL + (DCI * (-7.053654284009761e-2f64 + (DCI * DCM))));
                                        let DCO = DCI * KY;
                                        let DCP = (DCE * DCN) * DCN;
                                        let DCT = DCI * (DCQ + (DCI * (-1.17851130197758e-1f64 + (DCI * (DCR + (DCI * (-1.63730162779191e-3f64 + (DCI * DCS))))))));
                                        let DCU = (((DCT * DCT) + DCP) + GD).sqrt();
                                        let DCV = ((((LJ * (DCQ + (DCI * (-2.35702260395516e-1f64 + (DCI * (5.3640151901649905e-2f64 + (DCI * (-6.54920651116764e-3f64 + (DCO * DCS))))))))) * BF) * DCT) + ((((DCE * LJ) * BF) * DCN) * (DCK * (8.907946456731299e-1f64 + (DCI * (-2.8214617136039044e-1f64 + (DCO * DCM))))))) / (DCU + DCU);
                                        DDH = DCU;
                                        DDJ = DCV;
                                        DEL = DCT;
                                        DER = DCP;
                                    } else {
                                        let DCW = if DCI < ARC { 1.0 } else { 0.0 };
                                        let DDD;
                                        let DDF;
                                        if DCW != 0.0 {
                                            let DCX = DCI.exp();
                                            let DCY = DCE * (DCX - B);
                                            let DCZ = (DCE * LJ) * DCX;
                                            DDD = DCY;
                                            DDF = DCZ;
                                        } else {
                                            let DDA = (LJ * DCH).exp();
                                            let DDB = DBG * (DDA - DBE);
                                            let DDC = (DBG * LJ) * DDA;
                                            DDD = DDB;
                                            DDF = DDC;
                                        }
                                        let DDE = ((DCI - B) + DDD).sqrt();
                                        let DDG = ((LJ + DDF) / DDE) * I;
                                        DDH = DDE;
                                        DDJ = DDG;
                                        DEL = A;
                                        DER = DDD;
                                    }
                                    let DDI = (CZS - DCH) - (CZO * DDH);
                                    let DDK = -1e0f64 - (CZO * DDJ);
                                    let DDM = if DDL == B { 1.0 } else { 0.0 };
                                    let DEB;
                                    let DED;
                                    let DEE;
                                    if DDM != 0.0 {
                                        DEB = DDN;
                                        DED = DCH;
                                        DEE = DDL;
                                    } else {
                                        let DDO = (-DDI) / DDK;
                                        let DDP = DCH.abs();
                                        let DDQ = if B >= DDP { 1.0 } else { 0.0 };
                                        let DDR = if DDQ != 0.0 {
                                            B
                                        } else {
                                            DDP
                                        };
                                        let DDS = 5e-2f64 * (B + DDR);
                                        let DDT = if (DDO.abs()) > DDS { 1.0 } else { 0.0 };
                                        let DDY;
                                        if DDT != 0.0 {
                                            let DDU = if DDO >= A { 1.0 } else { 0.0 };
                                            let DDW = if DDU != 0.0 {
                                                B
                                            } else {
                                                DDV
                                            };
                                            let DDX = DDS * DDW;
                                            DDY = DDX;
                                        } else {
                                            DDY = DDO;
                                        }
                                        let DDZ = DCH + DDY;
                                        let DEA = if (if (DDY.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (DDI.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DEF = if DEA != 0.0 {
                                            B
                                        } else {
                                            DDL
                                        };
                                        DEB = DCF;
                                        DED = DDZ;
                                        DEE = DEF;
                                    }
                                    let DEC = DEB + B;
                                    DCF = DEC;
                                    DCH = DED;
                                    DDL = DEE;
                                    DEH = DCI;
                                    DEK = DEL;
                                    DEP = DDH;
                                    DEQ = DER;
                                }
                                let DEG = if DDL == A { 1.0 } else { 0.0 };
                                if DEG != 0.0 {
                                } else {
                                }
                                let DEI = if DEH < KY { 1.0 } else { 0.0 };
                                let DEO;
                                if DEI != 0.0 {
                                    let DEJ = if DEH < BR { 1.0 } else { 0.0 };
                                    if DEJ != 0.0 {
                                    } else {
                                    }
                                    let DEM = DEK + 2.220446049250313e-15f64;
                                    DEO = DEM;
                                } else {
                                    let DEN = (DEH - B).sqrt();
                                    DEO = DEN;
                                }
                                let DES = (CYS * DEO) + ((CYS * DEQ) * (B / (DEP + DEO)));
                                DEU = DES;
                                DKD = DEK;
                                DKK = DEP;
                                DKN = DEQ;
                            } else {
                                DEU = DCB;
                                DKD = A;
                                DKK = A;
                                DKN = A;
                            }
                            DET = DEU;
                            DKC = DKD;
                            DKJ = DKK;
                            DKM = DKN;
                        }
                        let EHR;
                        let EHY;
                        let EIE;
                        let EIP;
                        if DEV != 0.0 {
                            let EHS = if CYT != 0.0 {
                                let DEY = (-DEW) * DET;
                                DEY
                            } else {
                                A
                            };
                            let EHZ = if CYU != 0.0 {
                                let DEZ = (-DEW) * DET;
                                DEZ
                            } else {
                                A
                            };
                            EHR = EHS;
                            EHY = EHZ;
                            EIE = CYH;
                            EIP = CYG;
                        } else {
                            let EIF;
                            let EIQ;
                            if DFA != 0.0 {
                                let EIG = if CYT != 0.0 {
                                    let DFB = (-DEW) * DET;
                                    DFB
                                } else {
                                    CYH
                                };
                                let EIR = if CYU != 0.0 {
                                    let DFC = (-DEW) * DET;
                                    DFC
                                } else {
                                    CYG
                                };
                                EIF = EIG;
                                EIQ = EIR;
                            } else {
                                EIF = CYH;
                                EIQ = CYG;
                            }
                            EHR = A;
                            EHY = A;
                            EIE = EIF;
                            EIP = EIQ;
                        }
                        let DFF = (DFD * CYV) + CYW;
                        let DFG = (DFD * CYW) + CYV;
                        let DFH = (DFF * CYZ) + (DFG * CZA);
                        let DFI = -(((DFF * CZC) + (DFG * CZB)) + 2.220446049250313e-15f64);
                        let DFJ = if DFI > NN { 1.0 } else { 0.0 };
                        let DFO = if DFJ != 0.0 {
                            let DFK = NJ - NN;
                            let DFL = (DFI - NN) / DFK;
                            let DFM = DFL * DFL;
                            let DFN = NN + (DFK * (B - (B / ((((B + DFL) + DFM) + (DFM * DFL)) + (DFM * DFM)))));
                            DFN
                        } else {
                            DFI
                        };
                        let DFP = (-DFO) - G;
                        let DFQ = DFH - CZQ;
                        let DFR = -DFP;
                        let DFS = if DFQ < DFR { 1.0 } else { 0.0 };
                        let DKQ;
                        if DFS != 0.0 {
                            let DFT = (B / (LJ * CYS)) * CM;
                            let DFU = BF + (4.242640687119285e0f64 * DFT);
                            let DFV = ((BM * DFU) * DFU) * DFU;
                            let DFW = LI - CZT;
                            let DFX = (BLP * DFT) * ((LJ * (DFQ + DFP)) - BF);
                            let DFY = 9.899494936611664e0f64 - DFX;
                            let DFZ = DFY * DFY;
                            let DGA = if DFV < (DFZ * BLT) { 1.0 } else { 0.0 };
                            let DGD = if DGA != 0.0 {
                                let DGB = ((-9.899494936611664e0f64 + DFY) + ((I * DFV) / DFY)) + DFX;
                                DGB
                            } else {
                                let DGC = (-9.899494936611664e0f64 + ((DFV + DFZ).sqrt())) + DFX;
                                DGC
                            };
                            let DGE = DGD.powf(AAR);
                            let DGF = ((((((-5.65685424949238e0f64 - (BLZ * DFT)) + (BF * DGE)) + ((MN * DGE) * DGE)) / DGE) * LL) - DFP) + DFP;
                            let DGG = DGF / DFW;
                            let DGH = CM * (DFQ - ((DGF / ((B + (DGG * DGG)).sqrt())) - DFP));
                            DKQ = DGH;
                        } else {
                            let DGI = DFQ + DFP;
                            let DGJ = (LJ * DGI) - B;
                            let DGK = CZP * LK;
                            let DGL = B + ((BL * (DGJ + 4.9787068367863944e-2f64)) / DGK);
                            let DGM = if DGL < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DGP = if DGM != 0.0 {
                                DGN
                            } else {
                                DGL
                            };
                            let DGO = (CZP * LJ) / BF;
                            let DGQ = B + ((BL * (DGJ + ((-(LJ * ((DFQ + (DGO * (B - (DGP.sqrt())))) + DFP))).exp()))) / DGK);
                            let DGR = if DGQ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DGT = if DGR != 0.0 {
                                DGS
                            } else {
                                DGQ
                            };
                            let DGU = LJ * ((DFQ + (DGO * (B - (DGT.sqrt())))) + DFP);
                            let DGV = if DGU < BR { 1.0 } else { 0.0 };
                            let DHP = if DGV != 0.0 {
                                let DGW = 7.071067811865476e-1f64 + (B / (LJ * CZO));
                                let DGX = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DGW) / 5.286687693921294e-4f64)) + (((-DGI) / CZO) / 1.8773541122053122e-2f64);
                                let DGY = ((2.8160311683079683e-2f64 * DGW) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DGZ = ((DGX * DGX) + ((DGY * DGY) * DGY)).sqrt();
                                let DHA = LJ * ((((((((-DGX) + DGZ).powf(AAR)) + (-((DGX + DGZ).powf(AAR)))) - -3.7209791878387604e0f64) * LL) - DFP) + DFP);
                                DHA
                            } else {
                                DGU
                            };
                            let DHB = (LJ * DFR).exp();
                            let DHC = ME / IC;
                            let DHD = DHC * DHC;
                            let DHE = LJ * (DGI + BG);
                            let DHF = (DHD * (DHB + GD)) * DGK;
                            let DHG = (DHD * DGK).ln();
                            let DHH = LJ * DFP;
                            let DHI = (DHE - ((((DHF + (DHE * DHE)).ln()) - DHG) + DHH)) - B;
                            let DHJ = BL * DHE;
                            let DHK = if DHJ > A { 1.0 } else { 0.0 };
                            let DHM = if DHK != 0.0 {
                                DHJ
                            } else {
                                let DHL = -DHJ;
                                DHL
                            };
                            let DHN = (DHE - (DHE - (I * (DHI + (((DHI * DHI) + DHM).sqrt()))))) + (LJ * BG);
                            let DHO = (((DHF + (DHN * DHN)).ln()) - DHG) + DHH;
                            let DHQ = (DHO - DHP) - 6.0000000000000005e-2f64;
                            let DHR = (BL * DHO) * 6.0000000000000005e-2f64;
                            let DHS = if DHR > A { 1.0 } else { 0.0 };
                            let DHU = if DHS != 0.0 {
                                DHR
                            } else {
                                let DHT = -DHR;
                                DHT
                            };
                            let DHV = DHO - (I * (DHQ + (((DHQ * DHQ) + DHU).sqrt())));
                            let DHW = (DHV / LJ) - DFP;
                            let DHX = if ((DHV - B) + ((-DHV).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DHX != 0.0 {
                            } else {
                            }
                            let DHY = CM * (DFQ - DHW);
                            let DHZ = if DCC == B { 1.0 } else { 0.0 };
                            let DKR;
                            if DHZ != 0.0 {
                                let DIA = DHD * DHB;
                                let mut DIB = 0.0;
                                let mut DID = 0.0;
                                let mut DJC = 0.0;
                                let mut DJY = 0.0;
                                let mut DKB = 0.0;
                                let mut DKI = 0.0;
                                let mut DKL = 0.0;
                                DIB = B;
                                DID = DHW;
                                DJC = A;
                                DJY = DHV;
                                DKB = DKC;
                                DKI = DKJ;
                                DKL = DKM;
                                loop {
                                    let DIC = if DIB <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DIC == 0.0 {
                                        break;
                                    }
                                    let DIE = LJ * (DID + DFP);
                                    let DIF = if DIE < KY { 1.0 } else { 0.0 };
                                    let DIY;
                                    let DJA;
                                    let DKE;
                                    let DKO;
                                    if DIF != 0.0 {
                                        let DIG = DIE * DIE;
                                        let DIH = (DIG * DIE) * (DCL + (DIE * (-7.053654284009761e-2f64 + (DIE * DCM))));
                                        let DII = DIE * KY;
                                        let DIJ = (DIA * DIH) * DIH;
                                        let DIK = DIE * (DCQ + (DIE * (-1.17851130197758e-1f64 + (DIE * (DCR + (DIE * (-1.63730162779191e-3f64 + (DIE * DCS))))))));
                                        let DIL = (((DIK * DIK) + DIJ) + GD).sqrt();
                                        let DIM = ((((LJ * (DCQ + (DIE * (-2.35702260395516e-1f64 + (DIE * (5.3640151901649905e-2f64 + (DIE * (-6.54920651116764e-3f64 + (DII * DCS))))))))) * BF) * DIK) + ((((DIA * LJ) * BF) * DIH) * (DIG * (8.907946456731299e-1f64 + (DIE * (-2.8214617136039044e-1f64 + (DII * DCM))))))) / (DIL + DIL);
                                        DIY = DIL;
                                        DJA = DIM;
                                        DKE = DIK;
                                        DKO = DIJ;
                                    } else {
                                        let DIN = if DIE < ARC { 1.0 } else { 0.0 };
                                        let DIU;
                                        let DIW;
                                        if DIN != 0.0 {
                                            let DIO = DIE.exp();
                                            let DIP = DIA * (DIO - B);
                                            let DIQ = (DIA * LJ) * DIO;
                                            DIU = DIP;
                                            DIW = DIQ;
                                        } else {
                                            let DIR = (LJ * DID).exp();
                                            let DIS = DHD * (DIR - DHB);
                                            let DIT = (DHD * LJ) * DIR;
                                            DIU = DIS;
                                            DIW = DIT;
                                        }
                                        let DIV = ((DIE - B) + DIU).sqrt();
                                        let DIX = ((LJ + DIW) / DIV) * I;
                                        DIY = DIV;
                                        DJA = DIX;
                                        DKE = A;
                                        DKO = DIU;
                                    }
                                    let DIZ = (DFQ - DID) - (CZO * DIY);
                                    let DJB = -1e0f64 - (CZO * DJA);
                                    let DJD = if DJC == B { 1.0 } else { 0.0 };
                                    let DJS;
                                    let DJU;
                                    let DJV;
                                    if DJD != 0.0 {
                                        DJS = DJE;
                                        DJU = DID;
                                        DJV = DJC;
                                    } else {
                                        let DJF = (-DIZ) / DJB;
                                        let DJG = DID.abs();
                                        let DJH = if B >= DJG { 1.0 } else { 0.0 };
                                        let DJI = if DJH != 0.0 {
                                            B
                                        } else {
                                            DJG
                                        };
                                        let DJJ = 5e-2f64 * (B + DJI);
                                        let DJK = if (DJF.abs()) > DJJ { 1.0 } else { 0.0 };
                                        let DJP;
                                        if DJK != 0.0 {
                                            let DJL = if DJF >= A { 1.0 } else { 0.0 };
                                            let DJN = if DJL != 0.0 {
                                                B
                                            } else {
                                                DJM
                                            };
                                            let DJO = DJJ * DJN;
                                            DJP = DJO;
                                        } else {
                                            DJP = DJF;
                                        }
                                        let DJQ = DID + DJP;
                                        let DJR = if (if (DJP.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (DIZ.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DJW = if DJR != 0.0 {
                                            B
                                        } else {
                                            DJC
                                        };
                                        DJS = DIB;
                                        DJU = DJQ;
                                        DJV = DJW;
                                    }
                                    let DJT = DJS + B;
                                    DIB = DJT;
                                    DID = DJU;
                                    DJC = DJV;
                                    DJY = DIE;
                                    DKB = DKE;
                                    DKI = DIY;
                                    DKL = DKO;
                                }
                                let DJX = if DJC == A { 1.0 } else { 0.0 };
                                if DJX != 0.0 {
                                } else {
                                }
                                let DJZ = if DJY < KY { 1.0 } else { 0.0 };
                                let DKH;
                                if DJZ != 0.0 {
                                    let DKA = if DJY < BR { 1.0 } else { 0.0 };
                                    if DKA != 0.0 {
                                    } else {
                                    }
                                    let DKF = DKB + 2.220446049250313e-15f64;
                                    DKH = DKF;
                                } else {
                                    let DKG = (DJY - B).sqrt();
                                    DKH = DKG;
                                }
                                let DKP = (CYS * DKH) + ((CYS * DKL) * (B / (DKI + DKH)));
                                DKR = DKP;
                            } else {
                                DKR = DHY;
                            }
                            DKQ = DKR;
                        }
                        let EHP;
                        let EHW;
                        let EID;
                        let EIO;
                        if DKS != 0.0 {
                            let EHQ = if DFD != 0.0 {
                                let DKT = (-DEW) * DKQ;
                                DKT
                            } else {
                                EHR
                            };
                            let EHX = if DFE != 0.0 {
                                let DKU = (-DEW) * DKQ;
                                DKU
                            } else {
                                EHY
                            };
                            EHP = EHQ;
                            EHW = EHX;
                            EID = EIE;
                            EIO = EIP;
                        } else {
                            let EIH;
                            let EIS;
                            if DKV != 0.0 {
                                let EII = if DFD != 0.0 {
                                    let DKW = (-DEW) * DKQ;
                                    DKW
                                } else {
                                    EIE
                                };
                                let EIT = if DFE != 0.0 {
                                    let DKX = (-DEW) * DKQ;
                                    DKX
                                } else {
                                    EIP
                                };
                                EIH = EII;
                                EIS = EIT;
                            } else {
                                EIH = EIE;
                                EIS = EIP;
                            }
                            EHP = EHR;
                            EHW = EHY;
                            EID = EIH;
                            EIO = EIS;
                        }
                        EHO = EHP;
                        EHV = EHW;
                        EIC = EID;
                        EIN = EIO;
                    } else {
                        EHO = A;
                        EHV = A;
                        EIC = CYH;
                        EIN = CYG;
                    }
                    EHN = EHO;
                    EHU = EHV;
                    EIB = EIC;
                    EIM = EIN;
                } else {
                    EHN = A;
                    EHU = A;
                    EIB = CYH;
                    EIM = CYG;
                }
                EHM = EHN;
                EHT = EHU;
                EIA = EIB;
                EIL = EIM;
            } else {
                EHM = A;
                EHT = A;
                EIA = EIJ;
                EIL = EIU;
            }
            let DKY = if CAI != A { 1.0 } else { 0.0 };
            let EDQ;
            let EHA;
            if DKY != 0.0 {
                let DKZ = OR + CAT;
                let DLA = (CBA * DKZ) + ((B - CBA) * CAP);
                let DLC = if DLB != A { 1.0 } else { 0.0 };
                if DLC != 0.0 {
                } else {
                }
                let DLD = if DLA > (DKZ - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                let EDR = if DLD != 0.0 {
                    let DLE = DKZ - 2.220446049250313e-15f64;
                    DLE
                } else {
                    DLA
                };
                EDQ = EDR;
                EHA = A;
            } else {
                let DLF = if DLB != A { 1.0 } else { 0.0 };
                let EHB;
                if DLF != 0.0 {
                    let DLG = if CBJ < 1e-15f64 { 1.0 } else { 0.0 };
                    let EHC = if DLG != 0.0 {
                        A
                    } else {
                        let DLH = (CBJ * (LL / CU)) * (B / CAX);
                        DLH
                    };
                    EHB = EHC;
                } else {
                    EHB = A;
                }
                EDQ = EDS;
                EHA = EHB;
            }
            let DLI = B / CM;
            let EFU;
            let EFY;
            let EJD;
            let EJI;
            if JC != 0.0 {
                let DLK = if DLJ > A { 1.0 } else { 0.0 };
                let DLL = if (if parameters[29] >= B { 1.0 } else { 0.0 }) != 0.0 && DLK != 0.0 { 1.0 } else { 0.0 };
                let EFV;
                let EFZ;
                let EJE;
                let EJJ;
                if DLL != 0.0 {
                    let DLM = if (if AB == A { 1.0 } else { 0.0 }) != 0.0 && DLK != 0.0 { 1.0 } else { 0.0 };
                    let DYW;
                    let DZD;
                    let EJF;
                    let EJK;
                    if DLM != 0.0 {
                        let DLQ = if JB != 0.0 {
                            let DLO = DLN * CM;
                            DLO
                        } else {
                            let DLP = DR * CM;
                            DLP
                        };
                        let DLR = parameters[171] * DLQ;
                        let DLS = parameters[172] + OX;
                        let DLT = DLJ * DLQ;
                        let DLU = (OX * DLT) - ((NH - CAT) * (DLR * DLS));
                        let DLV = ((OX - OR) * DLT) - ((DLR * (DLS - OR)) * (NH - (CAP - OR)));
                        DYW = DLV;
                        DZD = DLU;
                        EJF = A;
                        EJK = A;
                    } else {
                        let DLW = MO * ((AB / IC).sqrt());
                        let DMI;
                        let DMT;
                        let DSB;
                        let DSE;
                        if JB != 0.0 {
                            let DLZ = (CYV * PB) + (CYW * (PB - OR));
                            let DMA = ((CYV * OX) + (CYW * (OX - OR))) - DLZ;
                            let DMB = CYV + (DLY * CYW);
                            let DMC = CYW + (DLY * CYV);
                            let DMD = ((DMB * (-DLZ)) + (DMC * (((CYV * OR) + (CYW * (-OR))) - DLZ))) + 2.220446049250313e-15f64;
                            DMI = DMD;
                            DMT = DMA;
                            DSB = DMB;
                            DSE = DMC;
                        } else {
                            let DME = CYV + (DLY * CYW);
                            let DMF = CYW + (DLY * CYV);
                            let DMV = if DLX != 0.0 {
                                let DMG = (CYV * OX) + (CYW * (OX - OR));
                                DMG
                            } else {
                                A
                            };
                            let DMU = if DLY != 0.0 {
                                let DMH = (CYW * OX) + (CYV * (OX - OR));
                                DMH
                            } else {
                                DMV
                            };
                            DMI = A;
                            DMT = DMU;
                            DSB = DME;
                            DSE = DMF;
                        }
                        let DMJ = -DMI;
                        let DMK = if DMJ > NN { 1.0 } else { 0.0 };
                        let DMP = if DMK != 0.0 {
                            let DML = NJ - NN;
                            let DMM = (DMJ - NN) / DML;
                            let DMN = DMM * DMM;
                            let DMO = NN + (DML * (B - (B / ((((B + DMM) + DMN) + (DMN * DMM)) + (DMN * DMN)))));
                            DMO
                        } else {
                            DMJ
                        };
                        let DMQ = (-DMP) - G;
                        let DMR = DLW * DLI;
                        let DMS = DMR * DMR;
                        let DMW = (-DMT) + AW;
                        let DMX = (BF / LJ) * ((AB / ME).ln());
                        let DMY = -DMQ;
                        let DMZ = if DMW < DMY { 1.0 } else { 0.0 };
                        let DRW;
                        let DXU;
                        if DMZ != 0.0 {
                            let DNA = (B / (LJ * DLW)) * CM;
                            let DNB = BF + (4.242640687119285e0f64 * DNA);
                            let DNC = ((BM * DNB) * DNB) * DNB;
                            let DND = LI - DMX;
                            let DNE = (BLP * DNA) * ((LJ * (DMW + DMQ)) - BF);
                            let DNF = 9.899494936611664e0f64 - DNE;
                            let DNG = DNF * DNF;
                            let DNH = if DNC < (DNG * BLT) { 1.0 } else { 0.0 };
                            let DNK = if DNH != 0.0 {
                                let DNI = ((-9.899494936611664e0f64 + DNF) + ((I * DNC) / DNF)) + DNE;
                                DNI
                            } else {
                                let DNJ = (-9.899494936611664e0f64 + ((DNC + DNG).sqrt())) + DNE;
                                DNJ
                            };
                            let DNL = DNK.powf(AAR);
                            let DNM = ((((((-5.65685424949238e0f64 - (BLZ * DNA)) + (BF * DNL)) + ((MN * DNL) * DNL)) / DNL) * LL) - DMQ) + DMQ;
                            let DNN = DNM / DND;
                            let DNO = CM * (DMW - ((DNM / ((B + (DNN * DNN)).sqrt())) - DMQ));
                            DRW = DNO;
                            DXU = A;
                        } else {
                            let DNP = DMW + DMQ;
                            let DNQ = (LJ * DNP) - B;
                            let DNR = DMS * LK;
                            let DNS = B + ((BL * (DNQ + 4.9787068367863944e-2f64)) / DNR);
                            let DNT = if DNS < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DNW = if DNT != 0.0 {
                                DNU
                            } else {
                                DNS
                            };
                            let DNV = (DMS * LJ) / BF;
                            let DNX = B + ((BL * (DNQ + ((-(LJ * ((DMW + (DNV * (B - (DNW.sqrt())))) + DMQ))).exp()))) / DNR);
                            let DNY = if DNX < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DOA = if DNY != 0.0 {
                                DNZ
                            } else {
                                DNX
                            };
                            let DOB = LJ * ((DMW + (DNV * (B - (DOA.sqrt())))) + DMQ);
                            let DOC = if DOB < BR { 1.0 } else { 0.0 };
                            let DOX = if DOC != 0.0 {
                                let DOD = 7.071067811865476e-1f64 + (B / (LJ * DMR));
                                let DOE = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DOD) / 5.286687693921294e-4f64)) + (((-DNP) / DMR) / 1.8773541122053122e-2f64);
                                let DOF = ((2.8160311683079683e-2f64 * DOD) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DOG = ((DOE * DOE) + ((DOF * DOF) * DOF)).sqrt();
                                let DOH = LJ * ((((((((-DOE) + DOG).powf(AAR)) + (-((DOE + DOG).powf(AAR)))) - -3.7209791878387604e0f64) * LL) - DMQ) + DMQ);
                                DOH
                            } else {
                                DOB
                            };
                            let DOJ = if DOI > A { 1.0 } else { 0.0 };
                            let DPE;
                            if DOJ != 0.0 {
                                let DOK = ME / AB;
                                let DOL = DOK * DOK;
                                let DOM = LJ * (DNP + BG);
                                let DON = (DOL * (((LJ * DMY).exp()) + GD)) * DNR;
                                let DOO = (DOL * DNR).ln();
                                let DOP = LJ * DMQ;
                                let DOQ = (DOM - ((((DON + (DOM * DOM)).ln()) - DOO) + DOP)) - B;
                                let DOR = BL * DOM;
                                let DOS = if DOR > A { 1.0 } else { 0.0 };
                                let DOU = if DOS != 0.0 {
                                    DOR
                                } else {
                                    let DOT = -DOR;
                                    DOT
                                };
                                let DOV = (DOM - (DOM - (I * (DOQ + (((DOQ * DOQ) + DOU).sqrt()))))) + (LJ * BG);
                                let DOW = (((DON + (DOV * DOV)).ln()) - DOO) + DOP;
                                let DOY = (DOW - DOX) - 6.0000000000000005e-2f64;
                                let DOZ = (BL * DOW) * 6.0000000000000005e-2f64;
                                let DPA = if DOZ > A { 1.0 } else { 0.0 };
                                let DPC = if DPA != 0.0 {
                                    DOZ
                                } else {
                                    let DPB = -DOZ;
                                    DPB
                                };
                                let DPD = DOW - (I * (DOY + (((DOY * DOY) + DPC).sqrt())));
                                DPE = DPD;
                            } else {
                                DPE = DOX;
                            }
                            let DPF = (DPE / LJ) - DMQ;
                            let DPG = if ((DPE - B) + ((-DPE).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DPG != 0.0 {
                            } else {
                            }
                            let DPH = CM * (DMW - DPF);
                            let DPI = if DOI == B { 1.0 } else { 0.0 };
                            let DRX;
                            let DXV;
                            if DPI != 0.0 {
                                let DPJ = (LJ * DMY).exp();
                                let DPK = ME / AB;
                                let DPL = DPK * DPK;
                                let DPM = DPL * DPJ;
                                let mut DPN = 0.0;
                                let mut DPP = 0.0;
                                let mut DQO = 0.0;
                                let mut DRK = 0.0;
                                let mut DRN = 0.0;
                                let mut DRS = 0.0;
                                let mut DRT = 0.0;
                                DPN = B;
                                DPP = DPF;
                                DQO = A;
                                DRK = DPE;
                                DRN = A;
                                DRS = A;
                                DRT = A;
                                loop {
                                    let DPO = if DPN <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DPO == 0.0 {
                                        break;
                                    }
                                    let DPQ = LJ * (DPP + DMQ);
                                    let DPR = if DPQ < KY { 1.0 } else { 0.0 };
                                    let DQK;
                                    let DQM;
                                    let DRO;
                                    let DRU;
                                    if DPR != 0.0 {
                                        let DPS = DPQ * DPQ;
                                        let DPT = (DPS * DPQ) * (DCL + (DPQ * (-7.053654284009761e-2f64 + (DPQ * DCM))));
                                        let DPU = DPQ * KY;
                                        let DPV = (DPM * DPT) * DPT;
                                        let DPW = DPQ * (DCQ + (DPQ * (-1.17851130197758e-1f64 + (DPQ * (DCR + (DPQ * (-1.63730162779191e-3f64 + (DPQ * DCS))))))));
                                        let DPX = (((DPW * DPW) + DPV) + GD).sqrt();
                                        let DPY = ((((LJ * (DCQ + (DPQ * (-2.35702260395516e-1f64 + (DPQ * (5.3640151901649905e-2f64 + (DPQ * (-6.54920651116764e-3f64 + (DPU * DCS))))))))) * BF) * DPW) + ((((DPM * LJ) * BF) * DPT) * (DPS * (8.907946456731299e-1f64 + (DPQ * (-2.8214617136039044e-1f64 + (DPU * DCM))))))) / (DPX + DPX);
                                        DQK = DPX;
                                        DQM = DPY;
                                        DRO = DPW;
                                        DRU = DPV;
                                    } else {
                                        let DPZ = if DPQ < ARC { 1.0 } else { 0.0 };
                                        let DQG;
                                        let DQI;
                                        if DPZ != 0.0 {
                                            let DQA = DPQ.exp();
                                            let DQB = DPM * (DQA - B);
                                            let DQC = (DPM * LJ) * DQA;
                                            DQG = DQB;
                                            DQI = DQC;
                                        } else {
                                            let DQD = (LJ * DPP).exp();
                                            let DQE = DPL * (DQD - DPJ);
                                            let DQF = (DPL * LJ) * DQD;
                                            DQG = DQE;
                                            DQI = DQF;
                                        }
                                        let DQH = ((DPQ - B) + DQG).sqrt();
                                        let DQJ = ((LJ + DQI) / DQH) * I;
                                        DQK = DQH;
                                        DQM = DQJ;
                                        DRO = A;
                                        DRU = DQG;
                                    }
                                    let DQL = (DMW - DPP) - (DMR * DQK);
                                    let DQN = -1e0f64 - (DMR * DQM);
                                    let DQP = if DQO == B { 1.0 } else { 0.0 };
                                    let DRE;
                                    let DRG;
                                    let DRH;
                                    if DQP != 0.0 {
                                        DRE = DQQ;
                                        DRG = DPP;
                                        DRH = DQO;
                                    } else {
                                        let DQR = (-DQL) / DQN;
                                        let DQS = DPP.abs();
                                        let DQT = if B >= DQS { 1.0 } else { 0.0 };
                                        let DQU = if DQT != 0.0 {
                                            B
                                        } else {
                                            DQS
                                        };
                                        let DQV = 5e-2f64 * (B + DQU);
                                        let DQW = if (DQR.abs()) > DQV { 1.0 } else { 0.0 };
                                        let DRB;
                                        if DQW != 0.0 {
                                            let DQX = if DQR >= A { 1.0 } else { 0.0 };
                                            let DQZ = if DQX != 0.0 {
                                                B
                                            } else {
                                                DQY
                                            };
                                            let DRA = DQV * DQZ;
                                            DRB = DRA;
                                        } else {
                                            DRB = DQR;
                                        }
                                        let DRC = DPP + DRB;
                                        let DRD = if (if (DRB.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (DQL.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DRI = if DRD != 0.0 {
                                            B
                                        } else {
                                            DQO
                                        };
                                        DRE = DPN;
                                        DRG = DRC;
                                        DRH = DRI;
                                    }
                                    let DRF = DRE + B;
                                    DPN = DRF;
                                    DPP = DRG;
                                    DQO = DRH;
                                    DRK = DPQ;
                                    DRN = DRO;
                                    DRS = DQK;
                                    DRT = DRU;
                                }
                                let DRJ = if DQO == A { 1.0 } else { 0.0 };
                                if DRJ != 0.0 {
                                } else {
                                }
                                let DRL = if DRK < KY { 1.0 } else { 0.0 };
                                let DRR;
                                if DRL != 0.0 {
                                    let DRM = if DRK < BR { 1.0 } else { 0.0 };
                                    if DRM != 0.0 {
                                    } else {
                                    }
                                    let DRP = DRN + 2.220446049250313e-15f64;
                                    DRR = DRP;
                                } else {
                                    let DRQ = (DRK - B).sqrt();
                                    DRR = DRQ;
                                }
                                let DRV = (DLW * DRR) + ((DLW * DRT) * (B / (DRS + DRR)));
                                DRX = DRV;
                                DXV = DRN;
                            } else {
                                DRX = DPH;
                                DXV = A;
                            }
                            DRW = DRX;
                            DXU = DXV;
                        }
                        let DSA = if JB != 0.0 {
                            let DRY = DLN * DLJ;
                            DRY
                        } else {
                            let DRZ = DR * DLJ;
                            DRZ
                        };
                        let DSC = if (if DSB != 0.0 && F != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DLX != 0.0 && JB != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJH = if DSC != 0.0 {
                            let DSD = DSA * DRW;
                            DSD
                        } else {
                            A
                        };
                        let DSF = if (if DSE != 0.0 && F != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DLY != 0.0 && JB != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJM = if DSF != 0.0 {
                            let DSG = DSA * DRW;
                            DSG
                        } else {
                            A
                        };
                        let DSS;
                        let DTB;
                        let DYJ;
                        let DYM;
                        if JB != 0.0 {
                            let DSJ = (CYV * PB) + (CYW * (PB - OR));
                            let DSK = ((CYV * OX) + (CYW * (OX - OR))) - DSJ;
                            let DSL = (DSH * CYV) + CYW;
                            let DSM = (DSH * CYW) + CYV;
                            let DSN = ((DSL * (-DSJ)) + (DSM * (((CYV * OR) + (CYW * (-OR))) - DSJ))) + 2.220446049250313e-15f64;
                            DSS = DSN;
                            DTB = DSK;
                            DYJ = DSL;
                            DYM = DSM;
                        } else {
                            let DSO = (DSH * CYV) + CYW;
                            let DSP = (DSH * CYW) + CYV;
                            let DTD = if DSH != 0.0 {
                                let DSQ = (CYV * OX) + (CYW * (OX - OR));
                                DSQ
                            } else {
                                DMT
                            };
                            let DTC = if DSI != 0.0 {
                                let DSR = (CYW * OX) + (CYV * (OX - OR));
                                DSR
                            } else {
                                DTD
                            };
                            DSS = A;
                            DTB = DTC;
                            DYJ = DSO;
                            DYM = DSP;
                        }
                        let DST = -DSS;
                        let DSU = if DST > NN { 1.0 } else { 0.0 };
                        let DSZ = if DSU != 0.0 {
                            let DSV = NJ - NN;
                            let DSW = (DST - NN) / DSV;
                            let DSX = DSW * DSW;
                            let DSY = NN + (DSV * (B - (B / ((((B + DSW) + DSX) + (DSX * DSW)) + (DSX * DSX)))));
                            DSY
                        } else {
                            DST
                        };
                        let DTA = (-DSZ) - G;
                        let DTE = (-DTB) + AW;
                        let DTF = -DTA;
                        let DTG = if DTE < DTF { 1.0 } else { 0.0 };
                        let DYE;
                        if DTG != 0.0 {
                            let DTH = (B / (LJ * DLW)) * CM;
                            let DTI = BF + (4.242640687119285e0f64 * DTH);
                            let DTJ = ((BM * DTI) * DTI) * DTI;
                            let DTK = LI - DMX;
                            let DTL = (BLP * DTH) * ((LJ * (DTE + DTA)) - BF);
                            let DTM = 9.899494936611664e0f64 - DTL;
                            let DTN = DTM * DTM;
                            let DTO = if DTJ < (DTN * BLT) { 1.0 } else { 0.0 };
                            let DTR = if DTO != 0.0 {
                                let DTP = ((-9.899494936611664e0f64 + DTM) + ((I * DTJ) / DTM)) + DTL;
                                DTP
                            } else {
                                let DTQ = (-9.899494936611664e0f64 + ((DTJ + DTN).sqrt())) + DTL;
                                DTQ
                            };
                            let DTS = DTR.powf(AAR);
                            let DTT = ((((((-5.65685424949238e0f64 - (BLZ * DTH)) + (BF * DTS)) + ((MN * DTS) * DTS)) / DTS) * LL) - DTA) + DTA;
                            let DTU = DTT / DTK;
                            let DTV = CM * (DTE - ((DTT / ((B + (DTU * DTU)).sqrt())) - DTA));
                            DYE = DTV;
                        } else {
                            let DTW = DTE + DTA;
                            let DTX = (LJ * DTW) - B;
                            let DTY = DMS * LK;
                            let DTZ = B + ((BL * (DTX + 4.9787068367863944e-2f64)) / DTY);
                            let DUA = if DTZ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DUD = if DUA != 0.0 {
                                DUB
                            } else {
                                DTZ
                            };
                            let DUC = (DMS * LJ) / BF;
                            let DUE = B + ((BL * (DTX + ((-(LJ * ((DTE + (DUC * (B - (DUD.sqrt())))) + DTA))).exp()))) / DTY);
                            let DUF = if DUE < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            let DUH = if DUF != 0.0 {
                                DUG
                            } else {
                                DUE
                            };
                            let DUI = LJ * ((DTE + (DUC * (B - (DUH.sqrt())))) + DTA);
                            let DUJ = if DUI < BR { 1.0 } else { 0.0 };
                            let DVD = if DUJ != 0.0 {
                                let DUK = 7.071067811865476e-1f64 + (B / (LJ * DMR));
                                let DUL = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * DUK) / 5.286687693921294e-4f64)) + (((-DTW) / DMR) / 1.8773541122053122e-2f64);
                                let DUM = ((2.8160311683079683e-2f64 * DUK) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                                let DUN = ((DUL * DUL) + ((DUM * DUM) * DUM)).sqrt();
                                let DUO = LJ * ((((((((-DUL) + DUN).powf(AAR)) + (-((DUL + DUN).powf(AAR)))) - -3.7209791878387604e0f64) * LL) - DTA) + DTA);
                                DUO
                            } else {
                                DUI
                            };
                            let DUP = if DOI > A { 1.0 } else { 0.0 };
                            let DVK;
                            if DUP != 0.0 {
                                let DUQ = ME / AB;
                                let DUR = DUQ * DUQ;
                                let DUS = LJ * (DTW + BG);
                                let DUT = (DUR * (((LJ * DTF).exp()) + GD)) * DTY;
                                let DUU = (DUR * DTY).ln();
                                let DUV = LJ * DTA;
                                let DUW = (DUS - ((((DUT + (DUS * DUS)).ln()) - DUU) + DUV)) - B;
                                let DUX = BL * DUS;
                                let DUY = if DUX > A { 1.0 } else { 0.0 };
                                let DVA = if DUY != 0.0 {
                                    DUX
                                } else {
                                    let DUZ = -DUX;
                                    DUZ
                                };
                                let DVB = (DUS - (DUS - (I * (DUW + (((DUW * DUW) + DVA).sqrt()))))) + (LJ * BG);
                                let DVC = (((DUT + (DVB * DVB)).ln()) - DUU) + DUV;
                                let DVE = (DVC - DVD) - 6.0000000000000005e-2f64;
                                let DVF = (BL * DVC) * 6.0000000000000005e-2f64;
                                let DVG = if DVF > A { 1.0 } else { 0.0 };
                                let DVI = if DVG != 0.0 {
                                    DVF
                                } else {
                                    let DVH = -DVF;
                                    DVH
                                };
                                let DVJ = DVC - (I * (DVE + (((DVE * DVE) + DVI).sqrt())));
                                DVK = DVJ;
                            } else {
                                DVK = DVD;
                            }
                            let DVL = (DVK / LJ) - DTA;
                            let DVM = if ((DVK - B) + ((-DVK).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                            if DVM != 0.0 {
                            } else {
                            }
                            let DVN = CM * (DTE - DVL);
                            let DVO = if DOI == B { 1.0 } else { 0.0 };
                            let DYF;
                            if DVO != 0.0 {
                                let DVP = (LJ * DTF).exp();
                                let DVQ = ME / AB;
                                let DVR = DVQ * DVQ;
                                let DVS = DVR * DVP;
                                let mut DVT = 0.0;
                                let mut DVV = 0.0;
                                let mut DWU = 0.0;
                                let mut DXQ = 0.0;
                                let mut DXT = 0.0;
                                let mut DYA = 0.0;
                                let mut DYB = 0.0;
                                DVT = B;
                                DVV = DVL;
                                DWU = A;
                                DXQ = DVK;
                                DXT = DXU;
                                DYA = A;
                                DYB = A;
                                loop {
                                    let DVU = if DVT <= 4.1e1f64 { 1.0 } else { 0.0 };
                                    if DVU == 0.0 {
                                        break;
                                    }
                                    let DVW = LJ * (DVV + DTA);
                                    let DVX = if DVW < KY { 1.0 } else { 0.0 };
                                    let DWQ;
                                    let DWS;
                                    let DXW;
                                    let DYC;
                                    if DVX != 0.0 {
                                        let DVY = DVW * DVW;
                                        let DVZ = (DVY * DVW) * (DCL + (DVW * (-7.053654284009761e-2f64 + (DVW * DCM))));
                                        let DWA = DVW * KY;
                                        let DWB = (DVS * DVZ) * DVZ;
                                        let DWC = DVW * (DCQ + (DVW * (-1.17851130197758e-1f64 + (DVW * (DCR + (DVW * (-1.63730162779191e-3f64 + (DVW * DCS))))))));
                                        let DWD = (((DWC * DWC) + DWB) + GD).sqrt();
                                        let DWE = ((((LJ * (DCQ + (DVW * (-2.35702260395516e-1f64 + (DVW * (5.3640151901649905e-2f64 + (DVW * (-6.54920651116764e-3f64 + (DWA * DCS))))))))) * BF) * DWC) + ((((DVS * LJ) * BF) * DVZ) * (DVY * (8.907946456731299e-1f64 + (DVW * (-2.8214617136039044e-1f64 + (DWA * DCM))))))) / (DWD + DWD);
                                        DWQ = DWD;
                                        DWS = DWE;
                                        DXW = DWC;
                                        DYC = DWB;
                                    } else {
                                        let DWF = if DVW < ARC { 1.0 } else { 0.0 };
                                        let DWM;
                                        let DWO;
                                        if DWF != 0.0 {
                                            let DWG = DVW.exp();
                                            let DWH = DVS * (DWG - B);
                                            let DWI = (DVS * LJ) * DWG;
                                            DWM = DWH;
                                            DWO = DWI;
                                        } else {
                                            let DWJ = (LJ * DVV).exp();
                                            let DWK = DVR * (DWJ - DVP);
                                            let DWL = (DVR * LJ) * DWJ;
                                            DWM = DWK;
                                            DWO = DWL;
                                        }
                                        let DWN = ((DVW - B) + DWM).sqrt();
                                        let DWP = ((LJ + DWO) / DWN) * I;
                                        DWQ = DWN;
                                        DWS = DWP;
                                        DXW = A;
                                        DYC = DWM;
                                    }
                                    let DWR = (DTE - DVV) - (DMR * DWQ);
                                    let DWT = -1e0f64 - (DMR * DWS);
                                    let DWV = if DWU == B { 1.0 } else { 0.0 };
                                    let DXK;
                                    let DXM;
                                    let DXN;
                                    if DWV != 0.0 {
                                        DXK = DWW;
                                        DXM = DVV;
                                        DXN = DWU;
                                    } else {
                                        let DWX = (-DWR) / DWT;
                                        let DWY = DVV.abs();
                                        let DWZ = if B >= DWY { 1.0 } else { 0.0 };
                                        let DXA = if DWZ != 0.0 {
                                            B
                                        } else {
                                            DWY
                                        };
                                        let DXB = 5e-2f64 * (B + DXA);
                                        let DXC = if (DWX.abs()) > DXB { 1.0 } else { 0.0 };
                                        let DXH;
                                        if DXC != 0.0 {
                                            let DXD = if DWX >= A { 1.0 } else { 0.0 };
                                            let DXF = if DXD != 0.0 {
                                                B
                                            } else {
                                                DXE
                                            };
                                            let DXG = DXB * DXF;
                                            DXH = DXG;
                                        } else {
                                            DXH = DWX;
                                        }
                                        let DXI = DVV + DXH;
                                        let DXJ = if (if (DXH.abs()) <= PH { 1.0 } else { 0.0 }) != 0.0 && (if (DWR.abs()) <= BLT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let DXO = if DXJ != 0.0 {
                                            B
                                        } else {
                                            DWU
                                        };
                                        DXK = DVT;
                                        DXM = DXI;
                                        DXN = DXO;
                                    }
                                    let DXL = DXK + B;
                                    DVT = DXL;
                                    DVV = DXM;
                                    DWU = DXN;
                                    DXQ = DVW;
                                    DXT = DXW;
                                    DYA = DWQ;
                                    DYB = DYC;
                                }
                                let DXP = if DWU == A { 1.0 } else { 0.0 };
                                if DXP != 0.0 {
                                } else {
                                }
                                let DXR = if DXQ < KY { 1.0 } else { 0.0 };
                                let DXZ;
                                if DXR != 0.0 {
                                    let DXS = if DXQ < BR { 1.0 } else { 0.0 };
                                    if DXS != 0.0 {
                                    } else {
                                    }
                                    let DXX = DXT + 2.220446049250313e-15f64;
                                    DXZ = DXX;
                                } else {
                                    let DXY = (DXQ - B).sqrt();
                                    DXZ = DXY;
                                }
                                let DYD = (DLW * DXZ) + ((DLW * DYB) * (B / (DYA + DXZ)));
                                DYF = DYD;
                            } else {
                                DYF = DVN;
                            }
                            DYE = DYF;
                        }
                        let DYI = if JB != 0.0 {
                            let DYG = DLN * DLJ;
                            DYG
                        } else {
                            let DYH = DR * DLJ;
                            DYH
                        };
                        let DYK = if (if DYJ != 0.0 && F != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DSH != 0.0 && JB != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJG = if DYK != 0.0 {
                            let DYL = DYI * DYE;
                            DYL
                        } else {
                            EJH
                        };
                        let DYN = if (if DYM != 0.0 && F != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DSI != 0.0 && JB != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let EJL = if DYN != 0.0 {
                            let DYO = DYI * DYE;
                            DYO
                        } else {
                            EJM
                        };
                        DYW = A;
                        DZD = A;
                        EJF = EJG;
                        EJK = EJL;
                    }
                    let DYP = (CYW * GK) + (CYV * GJ);
                    let EFW;
                    if DYP != 0.0 {
                        let DYS = (CYW * DYQ) + (CYV * DYR);
                        let DYX = if JB != 0.0 {
                            let DYU = DYS * (-((CYW * DLN) + (CYV * DYT)));
                            DYU
                        } else {
                            let DYV = DYS * (-DR);
                            DYV
                        };
                        let DYY = DYW + ((-DYX) * (OX - OR));
                        EFW = DYY;
                    } else {
                        EFW = DYW;
                    }
                    let DYZ = (CYV * GK) + (CYW * GJ);
                    let EGA;
                    if DYZ != 0.0 {
                        let DZA = (CYV * DYQ) + (CYW * DYR);
                        let DZE = if JB != 0.0 {
                            let DZB = DZA * (-((CYV * DLN) + (CYW * DYT)));
                            DZB
                        } else {
                            let DZC = DZA * (-DR);
                            DZC
                        };
                        let DZF = DZD + ((-DZE) * OX);
                        EGA = DZF;
                    } else {
                        EGA = DZD;
                    }
                    EFV = EFW;
                    EFZ = EGA;
                    EJE = EJF;
                    EJJ = EJK;
                } else {
                    let DZH = if DZG == B { 1.0 } else { 0.0 };
                    let DZI = if GJ == 0.0 { 1.0 } else { 0.0 };
                    let DZJ = if DZG != B { 1.0 } else { 0.0 };
                    let DZK = if GK == 0.0 { 1.0 } else { 0.0 };
                    let DZL = if (if DZH != 0.0 && DZI != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DZJ != 0.0 && DZK != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DZR;
                    if DZL != 0.0 {
                        let DZS = if JB != 0.0 {
                            let DZM = ((-CM) * DLJ) * DYT;
                            DZM
                        } else {
                            let DZN = ((-CM) * DLJ) * DR;
                            DZN
                        };
                        DZR = DZS;
                    } else {
                        let DZO = (CYW * DYQ) + (CYV * DYR);
                        let DZT = if JB != 0.0 {
                            let DZP = DZO * (-((CYW * DLN) + (CYV * DYT)));
                            DZP
                        } else {
                            let DZQ = DZO * (-DR);
                            DZQ
                        };
                        DZR = DZT;
                    }
                    let DZU = (-DZR) * (OX - OR);
                    let DZV = if (if DZH != 0.0 && DZK != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if DZJ != 0.0 && DZI != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EAB;
                    if DZV != 0.0 {
                        let EAC = if JB != 0.0 {
                            let DZW = ((-CM) * DLJ) * DLN;
                            DZW
                        } else {
                            let DZX = ((-CM) * DLJ) * DR;
                            DZX
                        };
                        EAB = EAC;
                    } else {
                        let DZY = (CYV * DYQ) + (CYW * DYR);
                        let EAD = if JB != 0.0 {
                            let DZZ = DZY * (-((CYV * DLN) + (CYW * DYT)));
                            DZZ
                        } else {
                            let EAA = DZY * (-DR);
                            EAA
                        };
                        EAB = EAD;
                    }
                    let EAE = (-EAB) * OX;
                    EFV = DZU;
                    EFZ = EAE;
                    EJE = A;
                    EJJ = A;
                }
                EFU = EFV;
                EFY = EFZ;
                EJD = EJE;
                EJI = EJJ;
            } else {
                EFU = A;
                EFY = A;
                EJD = A;
                EJI = A;
            }
            if JB != 0.0 {
                let EAI = parameters[173] * (((((CG * EF) - (LI * LJ)) + (parameters[175] * (LT.ln()))) / EAH).exp());
                let EAL = EAH / LJ;
                let EAM = parameters[177] * (LT * LT);
                let EAN = EAL * ((B + (EAM / (((EAK * H) * EAI) + GD))).ln());
                let EAO = if EAF < (EAL * ((B + (EAM / (((EAJ * H) * EAI) + GD))).ln())) { 1.0 } else { 0.0 };
                if EAO != 0.0 {
                } else {
                }
                let EAP = if EAG < EAN { 1.0 } else { 0.0 };
                if EAP != 0.0 {
                } else {
                }
                let EAS = EAQ * EAR;
                let EAU = EAQ * EAT;
                let EAV = H - parameters[238];
                let EAW = if EAV <= A { 1.0 } else { 0.0 };
                let EBE;
                let EBZ;
                if EAW != 0.0 {
                    EBE = A;
                    EBZ = A;
                } else {
                    EBE = EAU;
                    EBZ = EAS;
                }
                let EAY = if EAX > DLN { 1.0 } else { 0.0 };
                if EAY != 0.0 {
                    let EBA = EAZ * (EAX - DLN);
                    let EBC = EBB * DLN;
                    let EBD = if EAG < A { 1.0 } else { 0.0 };
                    if EBD != 0.0 {
                        let EBF = if EBE > A { 1.0 } else { 0.0 };
                        if EBF != 0.0 {
                            let EBH = if EBG == I { 1.0 } else { 0.0 };
                            if EBH != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let EBI = if EBA > A { 1.0 } else { 0.0 };
                        if EBI != 0.0 {
                            let EBK = if EBJ == I { 1.0 } else { 0.0 };
                            if EBK != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let EBL = if EBC > A { 1.0 } else { 0.0 };
                        if EBL != 0.0 {
                            let EBN = if EBM == I { 1.0 } else { 0.0 };
                            if EBN != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                    let EBO = EBB * EAX;
                    let EBP = if EAG < A { 1.0 } else { 0.0 };
                    if EBP != 0.0 {
                        let EBQ = if EBE > A { 1.0 } else { 0.0 };
                        if EBQ != 0.0 {
                            let EBR = if EBG == I { 1.0 } else { 0.0 };
                            if EBR != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let EBS = if EBO > A { 1.0 } else { 0.0 };
                        if EBS != 0.0 {
                            let EBT = if EBM == I { 1.0 } else { 0.0 };
                            if EBT != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                }
                let EBV = if EBU > DYT { 1.0 } else { 0.0 };
                if EBV != 0.0 {
                    let EBW = EAZ * (EBU - DYT);
                    let EBX = EBB * DYT;
                    let EBY = if EAF < A { 1.0 } else { 0.0 };
                    if EBY != 0.0 {
                        let ECA = if EBZ > A { 1.0 } else { 0.0 };
                        if ECA != 0.0 {
                            let ECB = if EBG == I { 1.0 } else { 0.0 };
                            if ECB != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let ECC = if EBW > A { 1.0 } else { 0.0 };
                        if ECC != 0.0 {
                            let ECD = if EBJ == I { 1.0 } else { 0.0 };
                            if ECD != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let ECE = if EBX > A { 1.0 } else { 0.0 };
                        if ECE != 0.0 {
                            let ECF = if EBM == I { 1.0 } else { 0.0 };
                            if ECF != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                    let ECG = EBB * EBU;
                    let ECH = if EAF < A { 1.0 } else { 0.0 };
                    if ECH != 0.0 {
                        let ECI = if EBZ > A { 1.0 } else { 0.0 };
                        if ECI != 0.0 {
                            let ECJ = if EBG == I { 1.0 } else { 0.0 };
                            if ECJ != 0.0 {
                            } else {
                            }
                        } else {
                        }
                        let ECK = if ECG > A { 1.0 } else { 0.0 };
                        if ECK != 0.0 {
                            let ECL = if EBM == I { 1.0 } else { 0.0 };
                            if ECL != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                }
                let ECM = if EBE > A { 1.0 } else { 0.0 };
                if ECM != 0.0 {
                    let ECN = -(((-1.6021918e-19f64 * HY) * EAV) * EAT);
                    let ECO = if ((BL * ECN) * (IM * ECN)) > A { 1.0 } else { 0.0 };
                    if ECO != 0.0 {
                    } else {
                    }
                } else {
                }
                let ECP = if EBZ > A { 1.0 } else { 0.0 };
                if ECP != 0.0 {
                    let ECQ = -(((-1.6021918e-19f64 * HY) * EAV) * EAR);
                    let ECR = if ((BL * ECQ) * (IM * ECQ)) > A { 1.0 } else { 0.0 };
                    if ECR != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let EPU;
            let EPX;
            if BA != 0.0 {
                let EPV = if CVB != 0.0 {
                    let ECV = (((ECS * ECT) * ECU) * ECU) / ((((CVO * CHV) * ECS) + ((ECT * ECU) * ECU)) + GD);
                    ECV
                } else {
                    let ECW = ECS + GD;
                    ECW
                };
                let ECX = parameters[235] * TK;
                EPU = EPV;
                EPX = ECX;
            } else {
                EPU = A;
                EPX = A;
            }
            let ECY = if CAI == 0.0 { 1.0 } else { 0.0 };
            let ECZ = if (if parameters[31] != A { 1.0 } else { 0.0 }) != 0.0 && ECY != 0.0 { 1.0 } else { 0.0 };
            let EKG;
            if ECZ != 0.0 {
                let EDA = CAX / ED;
                let EDB = (((TK + (CAX / (CAT - SJ))) + V) * LL) / ED;
                let EDE = ((((-2e0f64 * EDC) / ED) / EDD) / DR) - EDA;
                let EDF = EDE - EDA;
                let EDG = if (EDF.abs()) > 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let EDM = if EDG != 0.0 {
                    let EDH = EDA + EDB;
                    let EDI = EDE + EDB;
                    let EDJ = (((B / EDH) / EDI) + (((((BF * S) * CVW) * CVO) / EDF) * ((EDI / EDH).ln()))) + (((((S * CVW) * CVO) * S) * CVW) * CVO);
                    EDJ
                } else {
                    let EDK = EDA + EDB;
                    let EDL = (((B / EDK) / (EDE + EDB)) + ((((BF * S) * CVW) * CVO) / EDK)) + (((((S * CVW) * CVO) * S) * CVW) * CVO);
                    EDL
                };
                let EDN = (((CUF * CUF) * U) / ((ECU * LJ) * DP)) * EDM;
                EKG = EDN;
            } else {
                EKG = A;
            }
            let EDO = if CHT != A { 1.0 } else { 0.0 };
            let EDP = if EDO != 0.0 && ECY != 0.0 { 1.0 } else { 0.0 };
            let EFA;
            let EKS;
            if EDP != 0.0 {
                let EEA = (EDZ * ((EDQ - CAT) / ECU)) / 1e5f64;
                let EEB = if (if 9.999999999999978e-1f64 <= CDW { 1.0 } else { 0.0 }) != 0.0 && (if CDW <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EEE;
                if EEB != 0.0 {
                    EEE = B;
                } else {
                    let EEC = if (if 1.9999999999999978e0f64 <= CDW { 1.0 } else { 0.0 }) != 0.0 && (if CDW <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EEF = if EEC != 0.0 {
                        EEA
                    } else {
                        let EED = EEA.powf((CDW - B));
                        EED
                    };
                    EEE = EEF;
                }
                let EEG = B + (EEA * EEE);
                let EEH = EDZ * (EEG * (EEG.powf(((-1e0f64 / CDW) - B))));
                let EEI = (CVO + EEH) / BF;
                let EEJ = BZX * BZX;
                let EEK = BR * BZX;
                let EEL = ((((DP * TK) * CHV) * CVO) * ((((((B + EEK) + (LA * EEJ)) * EEH) * EEH) + ((((BR + (BL * BZX)) + (BR * EEJ)) * EEH) * CVO)) + ((((LA + EEK) + EEJ) * CVO) * CVO))) / ((((1.5e1f64 * ECU) * (B + BZX)) * EEI) * EEI);
                EFA = EEL;
                EKS = EEH;
            } else {
                EFA = A;
                EKS = A;
            }
            let EEP = if (if (if (if CHS != A { 1.0 } else { 0.0 }) != 0.0 && EDO != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if EEM == B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ECY != 0.0 { 1.0 } else { 0.0 };
            let EKO;
            let EKT;
            let EKW;
            let EKZ;
            if EEP != 0.0 {
                let EES = EEQ.sqrt();
                let EET = CHV + EES;
                let EEW = (((4.2e1f64 * EEU) * EEQ) + (BL * ((EEU * EEU) + (EEQ * EEQ)))) + (((OP * EES) * CHV) * (EEU + EEQ));
                let EEX = EET * EET;
                let EEY = EEW / ((EEX * EEX) * EET);
                let EEZ = ((DP / ECU) * CVO) * TK;
                let EFD = ((3.872983346207417e0f64 * EFB) * ((EEU + ((BL * CHV) * EES)) + EEQ)) / ((LA * EET) * (((((EFA / (EEZ * CHV)) * EET) * CHV) * EEW).sqrt()));
                EKO = EEZ;
                EKT = EES;
                EKW = EEY;
                EKZ = EFD;
            } else {
                EKO = G;
                EKT = A;
                EKW = A;
                EKZ = A;
            }
            let EIY;
            let EIZ;
            let EJA;
            if JB != 0.0 {
                let EFK = EFE + EFH;
                let EFN = if GI != 0.0 {
                    let EFM = EFK - (EFL * CX);
                    EFM
                } else {
                    EFK
                };
                let EFO = OX - PT;
                let EFQ = 2.1983327444149834e-11f64 * ((B + (EFP / CH)).ln());
                let EFR = EFQ * CZ;
                let EFX = EFU + ((EFR * (DA + EFS)) * (OX - OR));
                let EGB = EFY + ((EFR * (DA + EFT)) * OX);
                let EGC = ((-EFN) * EFO) + (((EFQ * JE) * CZ) * EFO);
                EIY = EFX;
                EIZ = EGB;
                EJA = EGC;
            } else {
                let EJB = if GI != 0.0 {
                    let EGD = (-((-EFL) * CX)) * (OX - PT);
                    EGD
                } else {
                    A
                };
                let EGE = ((2.1983327444149834e-11f64 * DA) * CZ) * ((B + (EFP / CH)).ln());
                let EGF = EFU + (EGE * (OX - OR));
                let EGG = EFY + (EGE * OX);
                EIY = EGF;
                EIZ = EGG;
                EJA = EJB;
            }
            let EIW;
            if BA != 0.0 {
                if JB != 0.0 {
                } else {
                }
                EIW = A;
            } else {
                let EIX = if JB != 0.0 {
                    let EGX = (-EGH) - EDC;
                    EGX
                } else {
                    let EGY = (((-EGL) - EDC) - EGT) - EGP;
                    EGY
                };
                EIW = EIX;
            }
            let EGZ = if DLB == A { 1.0 } else { 0.0 };
            let EHJ;
            if EGZ != 0.0 {
                EHJ = A;
            } else {
                let EHD = (EHA * CU) + CAT;
                let EHE = if EHD > EDQ { 1.0 } else { 0.0 };
                let EHG = if EHE != 0.0 {
                    EDQ
                } else {
                    EHD
                };
                let EHF = OR + CAT;
                let EHH = (((EHF - ((CBA * EHF) + ((B - CBA) * EHG))) / DLB) - EHA) * ((CI * DR) * (((2.069886e-10f64 / ID).sqrt()) * 1.3e0f64));
                EHJ = EHH;
            }
            let EHI = if FX != A { 1.0 } else { 0.0 };
            let EJC = if EHI != 0.0 {
                let EHK = EHJ + (FY * PT);
                EHK
            } else {
                EHJ
            };
            let EHL = if JC == B { 1.0 } else { 0.0 };
            let EKH;
            if EHL != 0.0 {
                let EKI = if JB != 0.0 {
                    let EJN = EIW + ((((((EIY + EIZ) + EJA) - EJC) - EJD) - EJI) + ((((-EHM) - EHT) - EIA) - EIL));
                    EJN
                } else {
                    let EJO = EIW + (((((EIY + EIZ) + EJA) - EJC) - EJD) - EJI);
                    EJO
                };
                EKH = EKI;
            } else {
                EKH = EIW;
            }
            if JB != 0.0 {
            } else {
            }
            let EJP = if AFT != B { 1.0 } else { 0.0 };
            if EJP != 0.0 {
            } else {
            }
            let EJS = -EJQ;
            let EJT = if DZG == B { 1.0 } else { 0.0 };
            let EQB = if EJT != 0.0 {
                let EKA = (EJU * EJV) - EJY;
                EKA
            } else {
                let EKD = ((B - EJU) * EJV) - EKB;
                EKD
            };
            let EQC = if EJT != 0.0 {
                let EKE = ((B - EJU) * EJV) - EKB;
                EKE
            } else {
                let EKF = (EJU * EJV) - EJY;
                EKF
            };
            if EJT != 0.0 {
            } else {
            }
            if EJT != 0.0 {
            } else {
            }
            let EKJ = GG * 0e0f64;
            let EKK = GG * 0e0f64;
            let EKL = if DZG > A { 1.0 } else { 0.0 };
            let EKM = if EKL != 0.0 {
                EKK
            } else {
                EKJ
            };
            let EQL;
            let EQM;
            if EEP != 0.0 {
                let EKN = ((O * TK) * DR) * CV;
                let EKP = (((1.898893985185185e-20f64 * LL) * EKM) * EKM) / EKO;
                let EKQ = if (if EFB > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if OR > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EKX = if EKQ != 0.0 {
                    let EKR = EDZ / CVO;
                    let EKU = EKR + (((6.666666666666667e-1f64 * (((EDZ / EKS) - EKR) / OR)) * ((EEU + (CHV * EKT)) + EEQ)) / (CHV + EKT));
                    EKU
                } else {
                    let EKV = EDZ / EKS;
                    EKV
                };
                let EKY = (EKP * EKW) * EKX;
                let ELA = if (-EKM) > EKN { 1.0 } else { 0.0 };
                let ELB = if ELA != 0.0 && (if EKY > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ELC = if ELB != 0.0 {
                    EKY
                } else {
                    A
                };
                let ELD = if ELA != 0.0 {
                    EKZ
                } else {
                    A
                };
                EQL = ELD;
                EQM = ELC;
            } else {
                EQL = A;
                EQM = A;
            }
            let ELF = if ELE == B { 1.0 } else { 0.0 };
            let EQP;
            if ELF != 0.0 {
                let EME;
                let EMF;
                let EMI;
                let EMT;
                let EMU;
                let ENN;
                let ENR;
                if ELG != 0.0 {
                    let ELI = ELH / O;
                    let ELN = if ELM > A { 1.0 } else { 0.0 };
                    let ELQ = if ELN != 0.0 {
                        let ELP = ELM * ELO;
                        ELP
                    } else {
                        A
                    };
                    let ELS = GG * (KC - KI);
                    EME = ELJ;
                    EMF = ELK;
                    EMI = ELL;
                    EMT = ELS;
                    EMU = ELR;
                    ENN = ELI;
                    ENR = ELQ;
                } else {
                    let ELW = if ELM > A { 1.0 } else { 0.0 };
                    let ELZ = if ELW != 0.0 {
                        let ELY = ELM * ELX;
                        ELY
                    } else {
                        A
                    };
                    let EMB = GG * (KH - KB);
                    EME = ELT;
                    EMF = ELU;
                    EMI = ELV;
                    EMT = EMB;
                    EMU = EMA;
                    ENN = AB;
                    ENR = ELZ;
                }
                let EMD = ((EMC * EMC) + (CT * CT)).sqrt();
                let EMK = EMI + (EMJ * LH);
                let EMR = ((EME / IZ) / (LT.powf(EMG))) * (B + (EML / (CY.powf(EMM))));
                let EMS = ((((EMF / AX) / (LZ - (EMH * MA))) * (B + (EMP / (DS.powf(EMQ))))) * (B + (EMN / (CY.powf(EMO))))) + GD;
                let EMV = EMR * (EMT / EMU);
                let EMW = if EMT >= A { 1.0 } else { 0.0 };
                let ENB = if EMW != 0.0 {
                    let EMX = EMV / EMS;
                    EMX
                } else {
                    let EMY = (-EMV) / EMS;
                    EMY
                };
                let EMZ = if (if 9.999999999999978e-1f64 <= EMK { 1.0 } else { 0.0 }) != 0.0 && (if EMK <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let END;
                if EMZ != 0.0 {
                    END = B;
                } else {
                    let ENA = if (if 1.9999999999999978e0f64 <= EMK { 1.0 } else { 0.0 }) != 0.0 && (if EMK <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ENE = if ENA != 0.0 {
                        ENB
                    } else {
                        let ENC = ENB.powf((EMK - B));
                        ENC
                    };
                    END = ENE;
                }
                let ENF = B + (ENB * END);
                let ENG = if (if 9.999999999999978e-1f64 <= EMK { 1.0 } else { 0.0 }) != 0.0 && (if EMK <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ENL;
                if ENG != 0.0 {
                    let ENH = B / ENF;
                    ENL = ENH;
                } else {
                    let ENI = if (if 1.9999999999999978e0f64 <= EMK { 1.0 } else { 0.0 }) != 0.0 && (if EMK <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ENM = if ENI != 0.0 {
                        let ENJ = B / (ENF.sqrt());
                        ENJ
                    } else {
                        let ENK = ENF * (ENF.powf(((-1e0f64 / EMK) - B)));
                        ENK
                    };
                    ENL = ENM;
                }
                let ENO = (((ED / EMU) * EMD) * (EMR * ENL)) * ENN;
                let ENP = if ENO <= A { 1.0 } else { 0.0 };
                let ENQ = if ENP != 0.0 {
                    GD
                } else {
                    ENO
                };
                let ENS = ((B / ENQ) / DP) + ENR;
                let ENT = if (if ENS > T { 1.0 } else { 0.0 }) != 0.0 && EDO != 0.0 { 1.0 } else { 0.0 };
                let ENV = if ENT != 0.0 {
                    let ENU = B / ENS;
                    ENU
                } else {
                    A
                };
                let ENW = if ENS < T { 1.0 } else { 0.0 };
                if ENW != 0.0 {
                } else {
                }
                EQP = ENV;
            } else {
                EQP = A;
            }
            let ENY = if ENX == B { 1.0 } else { 0.0 };
            let EQR;
            if ENY != 0.0 {
                let EOK;
                let EOL;
                let EOM;
                let EOQ;
                let EOR;
                let EPK;
                let EPO;
                if ENZ != 0.0 {
                    let EOA = ELH / O;
                    let EOB = if ELM > A { 1.0 } else { 0.0 };
                    let EOD = if EOB != 0.0 {
                        let EOC = ELM * ELO;
                        EOC
                    } else {
                        A
                    };
                    let EOE = GG * (KC - KI);
                    EOK = ELJ;
                    EOL = ELK;
                    EOM = ELL;
                    EOQ = EOE;
                    EOR = ELR;
                    EPK = EOA;
                    EPO = EOD;
                } else {
                    let EOF = if ELM > A { 1.0 } else { 0.0 };
                    let EOH = if EOF != 0.0 {
                        let EOG = ELM * ELX;
                        EOG
                    } else {
                        A
                    };
                    let EOI = GG * (KH - KB);
                    EOK = ELT;
                    EOL = ELU;
                    EOM = ELV;
                    EOQ = EOI;
                    EOR = EMA;
                    EPK = AB;
                    EPO = EOH;
                }
                let EOJ = ((EMC * EMC) + (CT * CT)).sqrt();
                let EON = EOM + (EMJ * LH);
                let EOO = ((EOK / IZ) / (LT.powf(EMG))) * (B + (EML / (CY.powf(EMM))));
                let EOP = ((((EOL / AX) / (LZ - (EMH * MA))) * (B + (EMP / (DS.powf(EMQ))))) * (B + (EMN / (CY.powf(EMO))))) + GD;
                let EOS = EOO * (EOQ / EOR);
                let EOT = if EOQ >= A { 1.0 } else { 0.0 };
                let EOY = if EOT != 0.0 {
                    let EOU = EOS / EOP;
                    EOU
                } else {
                    let EOV = (-EOS) / EOP;
                    EOV
                };
                let EOW = if (if 9.999999999999978e-1f64 <= EON { 1.0 } else { 0.0 }) != 0.0 && (if EON <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EPA;
                if EOW != 0.0 {
                    EPA = B;
                } else {
                    let EOX = if (if 1.9999999999999978e0f64 <= EON { 1.0 } else { 0.0 }) != 0.0 && (if EON <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EPB = if EOX != 0.0 {
                        EOY
                    } else {
                        let EOZ = EOY.powf((EON - B));
                        EOZ
                    };
                    EPA = EPB;
                }
                let EPC = B + (EOY * EPA);
                let EPD = if (if 9.999999999999978e-1f64 <= EON { 1.0 } else { 0.0 }) != 0.0 && (if EON <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EPI;
                if EPD != 0.0 {
                    let EPE = B / EPC;
                    EPI = EPE;
                } else {
                    let EPF = if (if 1.9999999999999978e0f64 <= EON { 1.0 } else { 0.0 }) != 0.0 && (if EON <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EPJ = if EPF != 0.0 {
                        let EPG = B / (EPC.sqrt());
                        EPG
                    } else {
                        let EPH = EPC * (EPC.powf(((-1e0f64 / EON) - B)));
                        EPH
                    };
                    EPI = EPJ;
                }
                let EPL = (((ED / EOR) * EOJ) * (EOO * EPI)) * EPK;
                let EPM = if EPL <= A { 1.0 } else { 0.0 };
                let EPN = if EPM != 0.0 {
                    GD
                } else {
                    EPL
                };
                let EPP = ((B / EPN) / DP) + EPO;
                let EPQ = if (if EPP > T { 1.0 } else { 0.0 }) != 0.0 && EDO != 0.0 { 1.0 } else { 0.0 };
                let EPS = if EPQ != 0.0 {
                    let EPR = B / EPP;
                    EPR
                } else {
                    A
                };
                let EPT = if EPP < T { 1.0 } else { 0.0 };
                if EPT != 0.0 {
                } else {
                }
                EQR = EPS;
            } else {
                EQR = A;
            }
            if JB != 0.0 {
                if BA != 0.0 {
                    let EPW = if EPU < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EPW != 0.0 {
                    } else {
                    }
                    let EPY = if EPX < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EPY != 0.0 {
                    } else {
                    }
                    if EJT != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                if BA != 0.0 {
                    let EPZ = if EPU < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EPZ != 0.0 {
                    } else {
                    }
                    let EQA = if EPX < 1.0000000000000001e-11f64 { 1.0 } else { 0.0 };
                    if EQA != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if EJT != 0.0 {
            } else {
            }
            if JB != 0.0 {
            } else {
            }
            let EQD = if (if KL == B { 1.0 } else { 0.0 }) != 0.0 && KN != 0.0 { 1.0 } else { 0.0 };
            if EQD != 0.0 {
            } else {
            }
            let EQE = if DZG != B { 1.0 } else { 0.0 };
            if EQE != 0.0 {
            } else {
            }
            if JB != 0.0 {
            } else {
            }
            let EQF = if AZ >= BM { 1.0 } else { 0.0 };
            if EQF != 0.0 {
                if JB != 0.0 {
                } else {
                }
            } else {
            }
            let EQG = 5.5224904e-23f64 * LG;
            let EQH = if CWA == B { 1.0 } else { 0.0 };
            if EQH != 0.0 {
            } else {
            }
            if ELE != 0.0 {
            } else {
            }
            if ENX != 0.0 {
            } else {
            }
            let EQI = DZG * EKG;
            let EQK = EQG * EFA;
            let EQN = if (if EQK > A { 1.0 } else { 0.0 }) != 0.0 && (if EQM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EQN != 0.0 {
            } else {
            }
            let EQO = (B - (EQL * EQL)) * EQK;
            if EKL != 0.0 {
            } else {
            }
            if EKL != 0.0 {
            } else {
            }
            let EQY;
            let EQZ;
            if ELE != 0.0 {
                let EQQ = EQG * EQP;
                EQY = B;
                EQZ = EQQ;
            } else {
                EQY = A;
                EQZ = A;
            }
            let ERA;
            let ERB;
            if ENX != 0.0 {
                let EQS = EQG * EQR;
                ERA = B;
                ERB = EQS;
            } else {
                ERA = A;
                ERB = A;
            }
            let ERC;
            let ERD;
            let ERE;
            let ERF;
            let ERG;
            let ERH;
            if EQH != 0.0 {
                let EQT = 3.2043836e-19f64 * EQB;
                let EQU = 3.2043836e-19f64 * EQC;
                let EQV = 3.2043836e-19f64 * EJS;
                ERC = B;
                ERD = EQT;
                ERE = B;
                ERF = EQU;
                ERG = B;
                ERH = EQV;
            } else {
                ERC = A;
                ERD = A;
                ERE = A;
                ERF = A;
                ERG = A;
                ERH = A;
            }
            if IQ != 0.0 {
            } else {
            }
            let EQW = if KM != 0.0 && (if Z > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EQW != 0.0 {
            } else {
            }
            if JB != 0.0 {
                if IT != 0.0 {
                } else {
                }
                if IW != 0.0 {
                } else {
                }
                if BA != 0.0 {
                } else {
                }
                let EQX = if ALA != 0.0 || CUH != 0.0 { 1.0 } else { 0.0 };
                if EQX != 0.0 {
                } else {
                }
            } else {
                if ALA != 0.0 {
                } else {
                }
                if BA != 0.0 {
                } else {
                }
            }
            if F != 0.0 {
            } else {
            }
        {
            let psd = EQI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(EQJ);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = EQK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = EQO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if EQY == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = EQZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERA == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERC == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERE == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ERG == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ERH;
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
